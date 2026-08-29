//! [`Body<T>`]: the arena store a B-rep *is* — scalar-free topology
//! arenas plus `T`-valued geometry arenas (Q1's genericity boundary).
//!
//! A body is a plain value (D1): cheaply cloneable, serializable later,
//! validatable now. It owns ten slotmap arenas — seven topology kinds,
//! each with its own key type, and three geometry kinds — plus one D5
//! provenance `SecondaryMap` per topology kind (**uniform across all
//! seven** — half-edges carry provenance like everything else).
//!
//! # Determinism (D9)
//!
//! Slotmap iteration runs in slot-index order, which is deterministic
//! given identical construction history — and D9's pure-replay model
//! guarantees identical construction history for identical inputs. The
//! crate-level invariant this rests on: **no iteration over these arenas
//! may feed a geometry decision unless the construction sequence that
//! filled them is itself deterministic** — true under the replay model,
//! and the reason this crate contains no `HashMap`/`HashSet` (whose
//! iteration order is seeded per-process and would break replay).
//!
//! # Bounded traversal (D9: the kernel never hangs)
//!
//! The half-edge structure is walked through *derived* accessors —
//! [`Body::mate`], [`Body::half_edge_end`], [`Body::loop_cycle`],
//! [`Body::vertex_orbit`] — all total: `Option`-returning on stale keys,
//! never panicking. The two walks are **bounded**: they cap iteration at
//! the half-edge arena length and surface non-closure as `None` instead
//! of spinning. This matters because the validator consumes them on
//! possibly-malformed bodies — an infinite loop on corrupt input would
//! be a D9-charter violation just as surely as a panic.
//!
//! # Key validity: stale vs. foreign
//!
//! Arena lookups go through [`Body::get_solid`] and friends, which return
//! `Option`. A **stale** key — one whose slot was removed, or whose
//! generation was bumped when its slot was reused — yields `None`, never a
//! panic (D9): generational indices make staleness detectable. Slotmap's
//! `Index` impl (`map[key]`) panics on stale keys and is not exposed: the
//! arenas are private, and kernel code uses `.get()`-shaped accessors
//! exclusively.
//!
//! **Foreignness is different, and not protected against.** A key is
//! meaningful only for the body — or its clone lineage — that minted it. A
//! key minted by an *unrelated* body with a compatible slot history may
//! silently resolve to `Some(arbitrary entity)` rather than `None`:
//! generational indices guard against staleness, not foreignness (this is
//! empirically demonstrable, not merely theoretical). Passing a key across
//! body lineages is the documented hazard here, and the accessors cannot
//! catch it. The flip side of this same coin is load-bearing — see the
//! [`Body`] docs on lineage-scoped keys.

use geom::Surface;
use geom_brep::{EdgeCurve, EdgeDescription, PcurveCache};
use geom_core::{Point3, Real};
use slotmap::{SecondaryMap, SlotMap};

use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, HalfEdge, HalfEdgeKey, Loop, LoopKey, Shell, ShellKey,
    Solid, SolidKey, Vertex, VertexKey,
};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::null::{CurveGeom, NullEdge, NullFacePair};
use crate::provenance::Provenance;
use crate::source::{GeomSource, SourceAttachError};

/// Outcome of a bounded half-edge traversal (crate-internal; the public
/// wrappers collapse the failure cases to `None`).
///
/// The validator needs the three-way distinction: `Broken` means the walk
/// hit a link that does not resolve or a mate that does not exist —
/// always accompanied by a reference/bijection error from an earlier
/// pass, so the walk itself stays silent about it — while `Overrun`
/// means every link resolved but the walk failed to return to its start
/// within the arena bound (a corruption with no more-local witness, so it
/// gets its own error variant).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Walk {
    /// The walk returned to its starting half-edge; the members are in
    /// walk order, starting with the start itself.
    Closed(Vec<HalfEdgeKey>),
    /// A link failed to resolve mid-walk (stale key, or a mate that
    /// does not exist).
    Broken,
    /// Every link resolved but the walk did not return to its start
    /// within the arena-length bound.
    Overrun,
}

/// A manifold B-rep body: topology arenas (scalar-free) plus geometry
/// arenas over the scalar type `T` (Q1's genericity boundary, D1's
/// arena store).
///
/// # No `PartialEq`, deliberately
///
/// Body equality is a topology-plus-certification question, not a
/// derivable one: structural key-for-key equality is too strict (two
/// replays of the same recipe can allocate different key histories yet
/// denote the same shape) and geometric coincidence is a tolerance
/// decision (D4), never a bit pattern — and the scalar `T` carries no
/// comparison surface anyway. Deriving `PartialEq` would answer the wrong
/// question silently, so no equality is offered at all.
///
/// # Lineage-scoped keys (one coin, two faces)
///
/// A key's identity is meaningful only within the lineage of the body that
/// minted it (see the [module docs](self) on stale-vs-foreign keys): a key
/// crossing into an *unrelated* body is the documented hazard, silently
/// resolvable to an arbitrary entity. The very same property is
/// load-bearing in the other direction. Two bodies built from an identical
/// construction history mint identical key sequences (D9 determinism), so
/// an interval-arithmetic replay materializes a `Body<Interval>` whose
/// topology keys match the `f64` build it mirrors **key for key** (Q1's
/// pure-replay model). Lineage-scoped key identity is precisely what lets
/// that replay cross-reference the two builds — a feature there, a hazard
/// everywhere else.
///
/// # Typed lookups
///
/// Every accessor takes the key type of its own arena, so cross-arena
/// lookups fail to typecheck:
///
/// ```compile_fail,E0308
/// use geom_core::Point3;
/// use topo::Body;
///
/// let mut body = Body::<f64>::new();
/// let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0)).unwrap();
/// // A `VertexKey` is not a `ShellKey`: this example must NOT compile.
/// let _ = body.get_shell(seed.vertex);
/// ```
#[derive(Clone, Debug)]
pub struct Body<T: Real> {
    // Topology arenas (scalar-free).
    pub(crate) solids: SlotMap<SolidKey, Solid>,
    pub(crate) shells: SlotMap<ShellKey, Shell>,
    pub(crate) faces: SlotMap<FaceKey, Face>,
    pub(crate) loops: SlotMap<LoopKey, Loop>,
    pub(crate) half_edges: SlotMap<HalfEdgeKey, HalfEdge>,
    pub(crate) edges: SlotMap<EdgeKey, Edge>,
    pub(crate) vertices: SlotMap<VertexKey, Vertex>,
    // Geometry arenas — the only `T`-carrying storage (real element
    // types since M2 PR 3; see `crate::geometry`).
    pub(crate) points: SlotMap<PointKey, Point3<T>>,
    pub(crate) curves: SlotMap<CurveKey, CurveGeom<T>>,
    pub(crate) surfaces: SlotMap<SurfaceKey, Surface<T>>,
    // M5 PR 6 pcurve caches (C4): the per-HALF-EDGE certified chart
    // image of an edge's carrier. Parallel to the half-edge arena
    // exactly as provenance is — and per half-edge, not per edge and
    // not per (edge, face), because a SEAM edge's two half-edges lie
    // on the SAME surface with DIFFERENT pcurves (the u = 0 vs u = 2π
    // branches), which any coarser key cannot hold (C4, spec §1). A
    // row is present only where a cache was minted and certified
    // (`crate::pcurves`); planar faces store nothing (M2's
    // derive-on-demand status, C4 verbatim), so an all-planar body
    // carries an empty map. Absence is never a claim about geometry.
    pub(crate) pcurves: SecondaryMap<HalfEdgeKey, PcurveCache<T>>,
    // M3 PR 1 null-face annotations (F9): typed loop-role attributes on
    // null (section-polygon) faces, parallel to the face arena like the
    // provenance maps — a record never outlives its face (kill-op
    // hygiene; the validator makes leaks loud). See `crate::null`.
    pub(crate) null_faces: SecondaryMap<FaceKey, NullFacePair>,
    // D5 provenance, parallel to the topology arenas (see
    // `crate::provenance` for the SecondaryMap-vs-inline rationale).
    // Uniform across all seven topology kinds — half-edges included.
    pub(crate) solid_provenance: SecondaryMap<SolidKey, Provenance>,
    pub(crate) shell_provenance: SecondaryMap<ShellKey, Provenance>,
    pub(crate) face_provenance: SecondaryMap<FaceKey, Provenance>,
    pub(crate) loop_provenance: SecondaryMap<LoopKey, Provenance>,
    pub(crate) half_edge_provenance: SecondaryMap<HalfEdgeKey, Provenance>,
    pub(crate) edge_provenance: SecondaryMap<EdgeKey, Provenance>,
    pub(crate) vertex_provenance: SecondaryMap<VertexKey, Provenance>,
    // N6 GeomSource records (M4 PR 5), parallel to the geometry arenas
    // exactly as provenance parallels the topology arenas: the recipe
    // source of each description, stamped by the recipe layer
    // (`editor-core`) after each op and carried by clone and graft.
    // An absent row = no recipe source (raw/kernel-level
    // construction); absence never certifies coincidence (the
    // ladder's conservative direction).
    pub(crate) point_sources: SecondaryMap<PointKey, GeomSource>,
    pub(crate) curve_sources: SecondaryMap<CurveKey, GeomSource>,
    pub(crate) surface_sources: SecondaryMap<SurfaceKey, GeomSource>,
}

impl<T: Real> Body<T> {
    /// An empty body: every arena empty.
    pub fn new() -> Self {
        Self {
            solids: SlotMap::with_key(),
            shells: SlotMap::with_key(),
            faces: SlotMap::with_key(),
            loops: SlotMap::with_key(),
            half_edges: SlotMap::with_key(),
            edges: SlotMap::with_key(),
            vertices: SlotMap::with_key(),
            points: SlotMap::with_key(),
            curves: SlotMap::with_key(),
            surfaces: SlotMap::with_key(),
            pcurves: SecondaryMap::new(),
            null_faces: SecondaryMap::new(),
            solid_provenance: SecondaryMap::new(),
            shell_provenance: SecondaryMap::new(),
            face_provenance: SecondaryMap::new(),
            loop_provenance: SecondaryMap::new(),
            half_edge_provenance: SecondaryMap::new(),
            edge_provenance: SecondaryMap::new(),
            vertex_provenance: SecondaryMap::new(),
            point_sources: SecondaryMap::new(),
            curve_sources: SecondaryMap::new(),
            surface_sources: SecondaryMap::new(),
        }
    }

    // ------------------------------------------------------------------
    // Raw insertion — crate-internal since M1 PR 5.
    //
    // The Euler operators are the ONLY public construction path (D1:
    // topology is built exclusively through validity-preserving Euler
    // ops). The raw builder survives as `pub(crate)` test scaffolding:
    // the validator's malformed fixtures and the review artifacts need
    // to build states no operator can reach. Raw insertion makes NO
    // validity promises: it can build arbitrarily malformed bodies by
    // design — judging the result is `topo::validate`'s job.
    //
    // The half-edge structure is cyclic (next/prev cycles, the edge ↔
    // half-edge bijection, spine back-pointers), so pure insertion in
    // any order cannot close a coherent body — not without forging
    // slotmap keys (`KeyData::from_ffi` plus deterministic minting makes
    // the forge reliable, but it is a hack, not an API) or a batch-graph
    // constructor. The raw builder therefore pairs each `add_*` with a
    // `get_*_mut` patching accessor: insert with provisional keys (e.g.
    // `Default::default()` null keys, which never resolve), then patch
    // the cycles closed. The Euler operators never need external
    // patching because each operator is itself a complete surgery.
    // ------------------------------------------------------------------

    /// Inserts a point (vertex geometry), returning its key.
    ///
    /// Crate-internal raw insertion (see the builder-API note in the
    /// source): no validity promises.
    pub(crate) fn add_point(&mut self, point: Point3<T>) -> PointKey {
        self.points.insert(point)
    }

    /// Inserts curve geometry (a certified [`EdgeCurve`] — certification
    /// is the only way to build one), returning its key.
    ///
    /// Crate-internal raw insertion: no validity promises beyond what
    /// the `EdgeCurve` itself carries.
    pub(crate) fn add_curve(&mut self, curve: EdgeCurve<T>) -> CurveKey {
        self.curves.insert(CurveGeom::Certified(curve))
    }

    /// Inserts a null-edge scaffolding entry
    /// ([`CurveGeom::NullScaffold`] — see [`crate::null`]), returning
    /// its key.
    ///
    /// Crate-internal raw insertion: no validity promises (the attribute
    /// vertices may dangle — scaffolding coherence is the minting op's
    /// contract).
    pub(crate) fn add_null_curve(&mut self, attr: NullEdge) -> CurveKey {
        self.curves.insert(CurveGeom::NullScaffold(attr))
    }

    /// Inserts surface geometry, returning its key.
    ///
    /// Crate-internal raw insertion: no validity promises.
    pub(crate) fn add_surface(&mut self, surface: Surface<T>) -> SurfaceKey {
        self.surfaces.insert(surface)
    }

    /// Inserts a vertex with its birth provenance (D5), returning its key.
    ///
    /// Crate-internal raw insertion: no validity promises (the referenced
    /// point may dangle — the validator reports it).
    pub(crate) fn add_vertex(&mut self, vertex: Vertex, provenance: Provenance) -> VertexKey {
        let key = self.vertices.insert(vertex);
        self.vertex_provenance.insert(key, provenance);
        key
    }

    /// Inserts a half-edge with its birth provenance (D5), returning its
    /// key.
    ///
    /// Crate-internal raw insertion: no validity promises. Because
    /// `next`/`prev` form cycles, callers typically insert with
    /// provisional keys and patch via [`Body::get_half_edge_mut`].
    pub(crate) fn add_half_edge(
        &mut self,
        half_edge: HalfEdge,
        provenance: Provenance,
    ) -> HalfEdgeKey {
        let key = self.half_edges.insert(half_edge);
        self.half_edge_provenance.insert(key, provenance);
        key
    }

    /// Inserts an edge with its birth provenance (D5), returning its key.
    ///
    /// Crate-internal raw insertion: no validity promises.
    pub(crate) fn add_edge(&mut self, edge: Edge, provenance: Provenance) -> EdgeKey {
        let key = self.edges.insert(edge);
        self.edge_provenance.insert(key, provenance);
        key
    }

    /// Inserts a loop with its birth provenance (D5), returning its key.
    ///
    /// Crate-internal raw insertion: no validity promises.
    pub(crate) fn add_loop(&mut self, loop_: Loop, provenance: Provenance) -> LoopKey {
        let key = self.loops.insert(loop_);
        self.loop_provenance.insert(key, provenance);
        key
    }

    /// Inserts a face with its birth provenance (D5), returning its key.
    ///
    /// Crate-internal raw insertion: no validity promises.
    pub(crate) fn add_face(&mut self, face: Face, provenance: Provenance) -> FaceKey {
        let key = self.faces.insert(face);
        self.face_provenance.insert(key, provenance);
        key
    }

    /// Inserts a shell with its birth provenance (D5), returning its key.
    ///
    /// Crate-internal raw insertion: no validity promises.
    pub(crate) fn add_shell(&mut self, shell: Shell, provenance: Provenance) -> ShellKey {
        let key = self.shells.insert(shell);
        self.shell_provenance.insert(key, provenance);
        key
    }

    /// Inserts a solid with its birth provenance (D5), returning its key.
    ///
    /// Crate-internal raw insertion: no validity promises.
    pub(crate) fn add_solid(&mut self, solid: Solid, provenance: Provenance) -> SolidKey {
        let key = self.solids.insert(solid);
        self.solid_provenance.insert(key, provenance);
        key
    }

    // ------------------------------------------------------------------
    // Raw mutation — the patching half of the raw builder (see the
    // raw-insertion note above for why cyclic references force it).
    // Total like the lookups: stale keys yield `None`. No validity
    // promises; `pub(crate)` with the rest since M1 PR 5.
    // ------------------------------------------------------------------

    /// Mutable access to the solid at `key` (raw-builder patching; see
    /// the source note on cyclic references). `None` if the key is stale.
    pub(crate) fn get_solid_mut(&mut self, key: SolidKey) -> Option<&mut Solid> {
        self.solids.get_mut(key)
    }

    /// Mutable access to the shell at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub(crate) fn get_shell_mut(&mut self, key: ShellKey) -> Option<&mut Shell> {
        self.shells.get_mut(key)
    }

    /// Mutable access to the face at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub(crate) fn get_face_mut(&mut self, key: FaceKey) -> Option<&mut Face> {
        self.faces.get_mut(key)
    }

    /// Mutable access to the loop at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub(crate) fn get_loop_mut(&mut self, key: LoopKey) -> Option<&mut Loop> {
        self.loops.get_mut(key)
    }

    /// Mutable access to the half-edge at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub(crate) fn get_half_edge_mut(&mut self, key: HalfEdgeKey) -> Option<&mut HalfEdge> {
        self.half_edges.get_mut(key)
    }

    /// Mutable access to the edge at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub(crate) fn get_edge_mut(&mut self, key: EdgeKey) -> Option<&mut Edge> {
        self.edges.get_mut(key)
    }

    /// Mutable access to the vertex at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub(crate) fn get_vertex_mut(&mut self, key: VertexKey) -> Option<&mut Vertex> {
        self.vertices.get_mut(key)
    }

    // ------------------------------------------------------------------
    // Kill-side geometry hygiene (crate-internal; M1 PR 3). The Euler
    // operators own topology-arena removal directly (the arenas are
    // pub(crate)); geometry removal goes through these guarded paths so
    // that a kill never strands geometry (tier 1's OrphanGeometry) and
    // never removes geometry something still references.
    // ------------------------------------------------------------------

    /// Removes `curve` from the curve arena iff no edge references it,
    /// returning whether it was removed. Used by edge-killing operators
    /// (`kemr`; PR 4's `kev`/`kef`), `split_edge`, and the edge-curve
    /// setter: in Euler-built bodies every edge owns its own
    /// placeholder curve, but the scan keeps the op sound standalone.
    /// Deterministic (D9): a full sweep of the edge arena — the outcome
    /// is a pure function of the arena contents. A stale `curve` key is
    /// a no-op returning `false`.
    ///
    /// A removed curve's description was a live surface reference
    /// ([`Body::description_surfaces`] — the same references that keep
    /// [`Body::remove_surface_if_orphaned`] from dangling it), so
    /// dropping the curve can orphan a surface whose faces are already
    /// gone: sweep its description surfaces through the same guarded
    /// rule (issue #86 — the second boolean of a boolean-of-boolean
    /// merges away a face while first-boolean `Intersection`
    /// descriptions still name its surface; re-describing the last
    /// such edge must take the surface with it, not strand it).
    pub(crate) fn remove_curve_if_orphaned(&mut self, curve: CurveKey) -> bool {
        if self.edges.values().any(|edge| edge.curve == curve) {
            return false;
        }
        let Some(removed) = self.curves.remove(curve) else {
            return false;
        };
        self.curve_sources.remove(curve);
        for surface in Self::description_surfaces(&removed) {
            self.remove_surface_if_orphaned(surface);
        }
        true
    }

    /// Removes `surface` from the surface arena iff nothing references
    /// it, returning whether it was removed. Used by face-killing
    /// operators (`kfmrh`, `kef`) and the surface-attachment setter.
    /// References counted (M2 PR 3): faces' `surface` keys AND edge
    /// descriptions' surface keys ([`Body::description_surfaces`]) — an
    /// `Intersection`/`Seam` description keeps its surfaces alive
    /// exactly like a face does, so removal can never dangle a
    /// description. Deterministic (D9), same shape as
    /// [`Body::remove_curve_if_orphaned`].
    pub(crate) fn remove_surface_if_orphaned(&mut self, surface: SurfaceKey) -> bool {
        if self.faces.values().any(|face| face.surface == surface) {
            return false;
        }
        if self
            .curves
            .values()
            .any(|curve| Self::description_surfaces(curve).contains(&surface))
        {
            return false;
        }
        let removed = self.surfaces.remove(surface).is_some();
        if removed {
            self.surface_sources.remove(surface);
        }
        removed
    }

    /// The surface keys an edge description references: the two
    /// intrinsic arms' pair, a chart image's chart, none for the
    /// scaffolding door (whose pushforward carries its own defining
    /// data and names no surface). Consulted by orphan hygiene and by
    /// the validator's referential-integrity pass.
    pub(crate) fn description_surfaces(curve: &CurveGeom<T>) -> Vec<SurfaceKey> {
        match curve {
            CurveGeom::Certified(curve) => match curve.description() {
                EdgeDescription::Intersection { s1, s2, .. }
                | EdgeDescription::TangentIntersection { s1, s2, .. } => vec![*s1, *s2],
                EdgeDescription::Chart(c) => vec![c.surface],
                EdgeDescription::Scaffold(_) => Vec::new(),
            },
            // Null scaffolding has no description and keeps no surface
            // alive.
            CurveGeom::NullScaffold(_) => Vec::new(),
        }
    }

    /// Removes `point` from the point arena iff no vertex references it,
    /// returning whether it was removed. Used by vertex-killing operators
    /// (PR 4's `kev`/`kvfs`): with M1's per-vertex point minting the
    /// killed vertex's point is always orphaned in practice, but the scan
    /// is the rule — it keeps the op sound standalone if points are ever
    /// shared. Deterministic (D9), same shape as
    /// [`Body::remove_curve_if_orphaned`].
    pub(crate) fn remove_point_if_orphaned(&mut self, point: PointKey) -> bool {
        if self.vertices.values().any(|vertex| vertex.point == point) {
            return false;
        }
        let removed = self.points.remove(point).is_some();
        if removed {
            self.point_sources.remove(point);
        }
        removed
    }

    // ------------------------------------------------------------------
    // GeomSource records (N6, M4 PR 5) — see `crate::source`.
    // ------------------------------------------------------------------

    /// The recipe source of a surface description, if the recipe layer
    /// stamped one (`None` for raw/kernel-level constructions — never
    /// coincidence-certifying).
    pub fn surface_source(&self, key: SurfaceKey) -> Option<&GeomSource> {
        self.surface_sources.get(key)
    }

    /// The recipe source of a curve description ([`Body::surface_source`]).
    pub fn curve_source(&self, key: CurveKey) -> Option<&GeomSource> {
        self.curve_sources.get(key)
    }

    /// The recipe source of a point ([`Body::surface_source`]).
    pub fn point_source(&self, key: PointKey) -> Option<&GeomSource> {
        self.point_sources.get(key)
    }

    /// Stamps (or replaces) the recipe source of a surface description
    /// — the recipe layer's post-op attachment door.
    ///
    /// # Errors
    ///
    /// [`SourceAttachError::StaleKey`] if the key does not resolve
    /// (attaching identity to nothing is a caller bug, refused loudly).
    pub fn set_surface_source(
        &mut self,
        key: SurfaceKey,
        source: GeomSource,
    ) -> Result<(), SourceAttachError> {
        if self.surfaces.get(key).is_none() {
            return Err(SourceAttachError::StaleKey);
        }
        self.surface_sources.insert(key, source);
        Ok(())
    }

    /// Stamps the recipe source of a curve description
    /// ([`Body::set_surface_source`]).
    ///
    /// # Errors
    ///
    /// [`SourceAttachError::StaleKey`] on a non-resolving key.
    pub fn set_curve_source(
        &mut self,
        key: CurveKey,
        source: GeomSource,
    ) -> Result<(), SourceAttachError> {
        if self.curves.get(key).is_none() {
            return Err(SourceAttachError::StaleKey);
        }
        self.curve_sources.insert(key, source);
        Ok(())
    }

    /// Stamps the recipe source of a point ([`Body::set_surface_source`]).
    ///
    /// # Errors
    ///
    /// [`SourceAttachError::StaleKey`] on a non-resolving key.
    pub fn set_point_source(
        &mut self,
        key: PointKey,
        source: GeomSource,
    ) -> Result<(), SourceAttachError> {
        if self.points.get(key).is_none() {
            return Err(SourceAttachError::StaleKey);
        }
        self.point_sources.insert(key, source);
        Ok(())
    }

    /// Clears every GeomSource record — the honest posture after a
    /// kernel-level geometric rewrite without recipe context
    /// (`transform_rigid`): the old sources' bit-identity claim no
    /// longer holds, so keeping them would let same-source certify
    /// coincidence between bit-DIFFERENT descriptions. The recipe
    /// layer re-stamps composed sources after the op (N6: the
    /// transform node composes into `expr`).
    pub fn clear_geom_sources(&mut self) {
        self.point_sources.clear();
        self.curve_sources.clear();
        self.surface_sources.clear();
    }

    // ------------------------------------------------------------------
    // Lookup. Total: a stale key yields `None`, never a panic. A foreign
    // key is NOT caught — it may resolve to an arbitrary entity (see the
    // module docs on stale-vs-foreign keys).
    // ------------------------------------------------------------------

    /// The solid at `key`, or `None` if the key is stale (a foreign key is
    /// not caught — see the [module docs](self)).
    pub fn get_solid(&self, key: SolidKey) -> Option<&Solid> {
        self.solids.get(key)
    }

    /// The shell at `key`, or `None` if the key is stale (a foreign key is
    /// not caught — see the [module docs](self)).
    pub fn get_shell(&self, key: ShellKey) -> Option<&Shell> {
        self.shells.get(key)
    }

    /// The face at `key`, or `None` if the key is stale (a foreign key is
    /// not caught — see the [module docs](self)).
    pub fn get_face(&self, key: FaceKey) -> Option<&Face> {
        self.faces.get(key)
    }

    /// **Test-only door**: a clone of this body with `face`'s
    /// [`Face::sense`] bit inverted — the hand-flipped face that S10's
    /// acceptance rows use to prove the outward-normal consumers
    /// actually honor the bit.
    ///
    /// Deliberately NOT a construction operator. Legitimate writers
    /// keep the two orientation encodings coherent: constructors mint
    /// the honest bit for the wall they are building (M5 S11,
    /// [`Body::set_face_sense`] — the loop winding is already the
    /// material-true one, so a concave wall's `false` agrees with it),
    /// and curved `revert` (the follow-on unit) will flip *every* face
    /// of a body at once. Flipping a single face makes the body
    /// **inside-out at that
    /// face** — geometrically incoherent by construction, which is
    /// exactly the point: it is the discriminating input for "does this
    /// consumer read the sense, or did it silently keep reading the
    /// chart normal?". Tier-3 validation is *expected to refuse* such a
    /// body; that refusal is one of the acceptance rows.
    ///
    /// Returns `None` iff `face` is stale.
    #[doc(hidden)]
    #[must_use]
    pub fn flipped_face_sense_for_tests(&self, face: FaceKey) -> Option<Self> {
        let mut out = self.clone();
        let f = out.faces.get_mut(face)?;
        f.sense = !f.sense;
        Some(out)
    }

    /// The loop at `key`, or `None` if the key is stale (a foreign key is
    /// not caught — see the [module docs](self)).
    /// (`get_loop`, like all lookups here, keeps the `get_` prefix partly
    /// for uniformity and partly because `loop` is a Rust keyword.)
    pub fn get_loop(&self, key: LoopKey) -> Option<&Loop> {
        self.loops.get(key)
    }

    /// The half-edge at `key`, or `None` if the key is stale (a foreign
    /// key is not caught — see the [module docs](self)).
    pub fn get_half_edge(&self, key: HalfEdgeKey) -> Option<&HalfEdge> {
        self.half_edges.get(key)
    }

    /// The edge at `key`, or `None` if the key is stale (a foreign key is
    /// not caught — see the [module docs](self)).
    pub fn get_edge(&self, key: EdgeKey) -> Option<&Edge> {
        self.edges.get(key)
    }

    /// The D5 birth record of a live edge (M4 PR 3: the naming layer
    /// reads `SplitEdge` parentage from here — birth data, never
    /// inspection). `None` iff the key is stale.
    pub fn edge_provenance_of(&self, key: EdgeKey) -> Option<&Provenance> {
        self.edge_provenance.get(key)
    }

    /// The D5 birth record of a live vertex (see
    /// [`Body::edge_provenance_of`]).
    pub fn vertex_provenance_of(&self, key: VertexKey) -> Option<&Provenance> {
        self.vertex_provenance.get(key)
    }

    /// The D5 birth record of a live face (see
    /// [`Body::edge_provenance_of`]).
    pub fn face_provenance_of(&self, key: FaceKey) -> Option<&Provenance> {
        self.face_provenance.get(key)
    }

    /// The vertex at `key`, or `None` if the key is stale (a foreign key is
    /// not caught — see the [module docs](self)).
    pub fn get_vertex(&self, key: VertexKey) -> Option<&Vertex> {
        self.vertices.get(key)
    }

    /// The point at `key`, or `None` if the key is stale (a foreign key is
    /// not caught — see the [module docs](self)).
    pub fn get_point(&self, key: PointKey) -> Option<&Point3<T>> {
        self.points.get(key)
    }

    /// The curve-arena entry at `key` — a certified carrier or M3
    /// null-edge scaffolding ([`CurveGeom`], the arena's element type
    /// since M3 PR 1) — or `None` if the key is stale (a foreign key is
    /// not caught — see the [module docs](self)).
    ///
    /// There is deliberately **no** accessor that silently narrows to a
    /// certified [`EdgeCurve`]: consumers that need a real carrier
    /// match on the sum (usually via [`CurveGeom::certified`]) and
    /// decide loudly what a scaffolding entry means for them.
    pub fn get_curve_geom(&self, key: CurveKey) -> Option<&CurveGeom<T>> {
        self.curves.get(key)
    }

    /// The surface geometry at `key`, or `None` if the key is stale (a
    /// foreign key is not caught — see the [module docs](self)).
    pub fn get_surface(&self, key: SurfaceKey) -> Option<&Surface<T>> {
        self.surfaces.get(key)
    }

    /// The D5 provenance of a topology entity, or `None` if the key is
    /// stale (a foreign key is not caught — see the [module docs](self)).
    /// Always `Some` for a live entity: the builder API
    /// records provenance at every insertion, so an entity without
    /// provenance is unrepresentable.
    pub fn provenance(&self, entity: EntityId) -> Option<&Provenance> {
        match entity {
            EntityId::Solid(k) => self.solid_provenance.get(k),
            EntityId::Shell(k) => self.shell_provenance.get(k),
            EntityId::Face(k) => self.face_provenance.get(k),
            EntityId::Loop(k) => self.loop_provenance.get(k),
            EntityId::HalfEdge(k) => self.half_edge_provenance.get(k),
            EntityId::Edge(k) => self.edge_provenance.get(k),
            EntityId::Vertex(k) => self.vertex_provenance.get(k),
        }
    }

    // ------------------------------------------------------------------
    // Derived half-edge accessors. All total (D9): stale keys and
    // structural corruption yield `None`, never a panic; the walks are
    // bounded (see the module docs on bounded traversal).
    // ------------------------------------------------------------------

    /// The mate of `he`: the other half of `he`'s edge. **Computed, never
    /// stored** — the edge node is the single source of the pairing.
    ///
    /// Returns `None` if `he` is stale, if `he.edge` does not resolve, or
    /// if the edge does not claim `he` in either slot (a corrupt
    /// bijection — the validator reports it as its own error). On a
    /// *corrupt* body the returned key is whatever the edge's other slot
    /// holds and may itself be stale; callers resolve it like any key.
    pub fn mate(&self, he: HalfEdgeKey) -> Option<HalfEdgeKey> {
        let half_edge = self.half_edges.get(he)?;
        let edge = self.edges.get(half_edge.edge)?;
        if edge.he_plus == he {
            Some(edge.he_minus)
        } else if edge.he_minus == he {
            Some(edge.he_plus)
        } else {
            None
        }
    }

    /// The end vertex of `he`, derived as `start(next(he))` — end
    /// vertices are never stored (see [`HalfEdge`]). `None` if `he` or
    /// its `next` is stale.
    pub fn half_edge_end(&self, he: HalfEdgeKey) -> Option<VertexKey> {
        let half_edge = self.half_edges.get(he)?;
        let next = self.half_edges.get(half_edge.next)?;
        Some(next.start)
    }

    /// The full cycle of `he`'s loop in `next` order, starting at `he`.
    ///
    /// **Bounded** (D9): the walk caps at the half-edge arena length and
    /// returns `None` if the cycle fails to close within the bound, if a
    /// link is stale, or if `he` itself is stale — it never spins on a
    /// corrupted body.
    pub fn loop_cycle(&self, he: HalfEdgeKey) -> Option<Vec<HalfEdgeKey>> {
        match self.loop_walk(he) {
            Walk::Closed(members) => Some(members),
            Walk::Broken | Walk::Overrun => None,
        }
    }

    /// The orbit of half-edges starting at `he`'s start vertex, walked
    /// **clockwise** viewed from outside (outward normal toward the
    /// viewer), starting at `he`.
    ///
    /// The step is `next(mate(he))` — under OUR counterclockwise
    /// interior-left convention this advances one face *clockwise* around
    /// the vertex; the inverse step `mate(prev(he))` walks the orbit
    /// counterclockwise. Derivation and the GWB-is-mirrored warning:
    /// [`crate::entity`] module docs (orientation conventions).
    ///
    /// **Bounded** (D9): caps at the half-edge arena length; `None` on
    /// stale keys, a broken mate (corrupt edge ↔ half-edge bijection), or
    /// non-closure within the bound. On a *valid* body the orbit always
    /// closes and visits exactly the half-edges starting at the vertex
    /// (the validator's manifoldness check).
    pub fn vertex_orbit(&self, he: HalfEdgeKey) -> Option<Vec<HalfEdgeKey>> {
        match self.orbit_walk(he) {
            Walk::Closed(members) => Some(members),
            Walk::Broken | Walk::Overrun => None,
        }
    }

    /// Bounded loop-cycle walk with the three-way outcome the validator
    /// needs (see [`Walk`]). Step: `next`.
    pub(crate) fn loop_walk(&self, first: HalfEdgeKey) -> Walk {
        self.bounded_walk(first, |body, he| {
            body.half_edges.get(he).map(|half_edge| half_edge.next)
        })
    }

    /// Bounded vertex-orbit walk with the three-way outcome the validator
    /// needs (see [`Walk`]). Step: `next(mate(he))` — the clockwise
    /// orbit; see [`Body::vertex_orbit`].
    pub(crate) fn orbit_walk(&self, first: HalfEdgeKey) -> Walk {
        self.bounded_walk(first, |body, he| {
            let mate = body.mate(he)?;
            body.half_edges.get(mate).map(|half_edge| half_edge.next)
        })
    }

    /// The shared bounded-walk engine: iterates `step` from `first` until
    /// it returns to `first` (`Closed`), a link fails to resolve
    /// (`Broken`), or the member count exceeds the half-edge arena length
    /// (`Overrun` — a valid cycle can never be longer than the arena).
    fn bounded_walk(
        &self,
        first: HalfEdgeKey,
        step: impl Fn(&Self, HalfEdgeKey) -> Option<HalfEdgeKey>,
    ) -> Walk {
        if !self.half_edges.contains_key(first) {
            return Walk::Broken;
        }
        let cap = self.half_edges.len();
        let mut members = vec![first];
        let mut current = first;
        loop {
            let Some(next) = step(self, current) else {
                return Walk::Broken;
            };
            if !self.half_edges.contains_key(next) {
                return Walk::Broken;
            }
            if next == first {
                return Walk::Closed(members);
            }
            if members.len() == cap {
                return Walk::Overrun;
            }
            members.push(next);
            current = next;
        }
    }

    // ------------------------------------------------------------------
    // Iteration. Slot-index order: deterministic given identical
    // construction history (see the module docs' D9 note).
    // ------------------------------------------------------------------

    /// All solids, in slot-index order (deterministic per D9 — see the
    /// [module docs](self)).
    pub fn solids(&self) -> impl Iterator<Item = (SolidKey, &Solid)> {
        self.solids.iter()
    }

    /// All shells, in slot-index order (deterministic per D9).
    pub fn shells(&self) -> impl Iterator<Item = (ShellKey, &Shell)> {
        self.shells.iter()
    }

    /// All faces, in slot-index order (deterministic per D9).
    pub fn faces(&self) -> impl Iterator<Item = (FaceKey, &Face)> {
        self.faces.iter()
    }

    /// All loops, in slot-index order (deterministic per D9).
    pub fn loops(&self) -> impl Iterator<Item = (LoopKey, &Loop)> {
        self.loops.iter()
    }

    /// All half-edges, in slot-index order (deterministic per D9).
    pub fn half_edges(&self) -> impl Iterator<Item = (HalfEdgeKey, &HalfEdge)> {
        self.half_edges.iter()
    }

    /// All edges, in slot-index order (deterministic per D9).
    pub fn edges(&self) -> impl Iterator<Item = (EdgeKey, &Edge)> {
        self.edges.iter()
    }

    /// All vertices, in slot-index order (deterministic per D9).
    pub fn vertices(&self) -> impl Iterator<Item = (VertexKey, &Vertex)> {
        self.vertices.iter()
    }

    /// All points, in slot-index order (deterministic per D9).
    pub fn points(&self) -> impl Iterator<Item = (PointKey, &Point3<T>)> {
        self.points.iter()
    }

    /// All curve-arena entries (certified carriers and null-edge
    /// scaffolding alike), in slot-index order (deterministic per D9).
    pub fn curves(&self) -> impl Iterator<Item = (CurveKey, &CurveGeom<T>)> {
        self.curves.iter()
    }

    /// All stored pcurve caches (C4 — see [`crate::pcurves`]), in
    /// half-edge-slot order (deterministic per D9).
    ///
    /// Emptiness is the normal state: planar faces keep M2's
    /// derive-on-demand status and store nothing, and only the charts
    /// with a certified closed-form image mint caches at M5, so a
    /// prism, a box, or any all-planar body yields zero rows here.
    pub fn pcurves(&self) -> impl Iterator<Item = (HalfEdgeKey, &PcurveCache<T>)> {
        self.pcurves.iter()
    }

    /// The stored pcurve cache of `half_edge`, or `None` when the
    /// half-edge stores none (derive it on demand through
    /// [`crate::pcurves::pcurve_of`]).
    pub fn pcurve(&self, half_edge: HalfEdgeKey) -> Option<&PcurveCache<T>> {
        self.pcurves.get(half_edge)
    }

    /// Attaches a **certified** pcurve cache to `half_edge`, returning
    /// the row it replaced.
    ///
    /// The argument's type is the guarantee: a [`PcurveCache`] cannot
    /// be built except through certification, so no uncertified pcurve
    /// can enter a body through this door (or any other). What this
    /// door cannot check is *coherence with this body* — that the cache
    /// was certified against THIS half-edge's carrier, THIS face's
    /// surface, and a branch consistent with its loop. That is the
    /// tier-3 pcurve pass's job ([`crate::pcurves::validate_pcurves`]),
    /// which re-derives all three and never consults the stored
    /// certificate.
    ///
    /// The ordinary producer is [`crate::pcurves::mint_pcurves`], which
    /// the splitting lane runs on every side it mints.
    pub fn attach_pcurve(
        &mut self,
        half_edge: HalfEdgeKey,
        cache: PcurveCache<T>,
    ) -> Option<PcurveCache<T>> {
        self.pcurves.insert(half_edge, cache)
    }

    /// Removes and returns `half_edge`'s stored pcurve cache —
    /// [`Body::attach_pcurve`]'s inverse (same trust posture: the
    /// tier-3 pcurve pass owns coherence, and a face left HALF-minted
    /// fails it loudly as `MissingCache`). Consumers of caches refuse
    /// typed on absence; nothing re-derives a branch silently.
    pub fn detach_pcurve(&mut self, half_edge: HalfEdgeKey) -> Option<PcurveCache<T>> {
        self.pcurves.remove(half_edge)
    }

    /// All null-face annotations (F9 — see [`crate::null`]), in
    /// face-slot order (deterministic per D9).
    pub fn null_faces(&self) -> impl Iterator<Item = (FaceKey, &NullFacePair)> {
        self.null_faces.iter()
    }

    /// The F9 null-face annotation of `face`, or `None` if the face is
    /// not marked (or the key is stale).
    pub fn null_face_pair(&self, face: FaceKey) -> Option<&NullFacePair> {
        self.null_faces.get(face)
    }

    /// All surface geometry, in slot-index order (deterministic per D9).
    pub fn surfaces(&self) -> impl Iterator<Item = (SurfaceKey, &Surface<T>)> {
        self.surfaces.iter()
    }
}

impl<T: Real> Default for Body<T> {
    /// The empty body ([`Body::new`]).
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::fixtures::{mvfs_state, pillow, prov};
    use geom_core::Tol;

    fn origin() -> Point3<f64> {
        Point3::origin()
    }

    #[test]
    fn empty_body_has_empty_arenas() {
        let body = Body::<f64>::new();
        assert_eq!(body.solids().count(), 0);
        assert_eq!(body.shells().count(), 0);
        assert_eq!(body.faces().count(), 0);
        assert_eq!(body.loops().count(), 0);
        assert_eq!(body.half_edges().count(), 0);
        assert_eq!(body.edges().count(), 0);
        assert_eq!(body.vertices().count(), 0);
        assert_eq!(body.points().count(), 0);
        assert_eq!(body.curves().count(), 0);
        assert_eq!(body.surfaces().count(), 0);
    }

    #[test]
    fn insertion_roundtrips_through_lookup() {
        // The pillow fixture exercises every adder and patcher; read the
        // whole containment structure back through the lookups.
        let t = pillow(Tol::witness());
        let body = &t.body;

        let v0 = body.get_vertex(t.vertices[0]).unwrap();
        assert_eq!(v0.point, t.points[0]);
        assert_eq!(v0.emanating, Some(t.hes_a[0]));

        let a0 = body.get_half_edge(t.hes_a[0]).unwrap();
        assert_eq!(a0.edge, t.edges[0]);
        assert_eq!(a0.start, t.vertices[0]);
        assert_eq!(a0.parent_loop, t.loop_a);
        assert_eq!(a0.next, t.hes_a[1]);
        assert_eq!(a0.prev, t.hes_a[1]);

        let e0 = body.get_edge(t.edges[0]).unwrap();
        assert_eq!(e0.he_plus, t.hes_a[0]);
        assert_eq!(e0.he_minus, t.hes_b[0]);
        assert_eq!(e0.curve, t.curves[0]);

        let loop_a = body.get_loop(t.loop_a).unwrap();
        assert_eq!(
            loop_a.boundary,
            crate::entity::LoopBoundary::Cycle { first: t.hes_a[0] }
        );
        assert_eq!(loop_a.face, t.face_a);

        let face_a = body.get_face(t.face_a).unwrap();
        assert_eq!(face_a.outer, t.loop_a);
        assert!(face_a.rings.is_empty());
        assert_eq!(face_a.shell, t.shell);

        let shell = body.get_shell(t.shell).unwrap();
        assert_eq!(shell.faces, vec![t.face_a, t.face_b]);
        assert_eq!(shell.solid, t.solid);
        assert_eq!(body.get_solid(t.solid).unwrap().shells, vec![t.shell]);
    }

    #[test]
    fn stale_key_lookup_is_none_not_panic() {
        let mut body = Body::<f64>::new();
        let sh = body.add_shell(
            Shell {
                faces: vec![],
                solid: SolidKey::default(),
            },
            prov(),
        );
        // Unit tests may reach into the pub(crate) arenas; sanctioned
        // removal is the kill-side Euler ops (bodies are values — raw
        // removal here is deliberate test corruption).
        body.shells.remove(sh);
        assert!(body.get_shell(sh).is_none());
        assert!(body.get_shell_mut(sh).is_none());
        // Reusing the freed slot bumps the version: the stale key stays
        // stale even though the slot is occupied again (generational keys,
        // D1).
        let sh2 = body.add_shell(
            Shell {
                faces: vec![],
                solid: SolidKey::default(),
            },
            prov(),
        );
        assert!(body.get_shell(sh).is_none());
        assert!(body.get_shell(sh2).is_some());
    }

    #[test]
    fn provenance_is_recorded_at_birth_for_every_kind() {
        // The pillow fixture inserts every topology kind (half-edges
        // included — D5 provenance is uniform across all seven arenas).
        let t = pillow(Tol::witness());
        let cases = [
            EntityId::Vertex(t.vertices[0]),
            EntityId::HalfEdge(t.hes_a[0]),
            EntityId::Edge(t.edges[0]),
            EntityId::Loop(t.loop_a),
            EntityId::Face(t.face_a),
            EntityId::Shell(t.shell),
            EntityId::Solid(t.solid),
        ];
        for id in cases {
            assert_eq!(t.body.provenance(id), Some(&prov()), "for {id}");
        }
    }

    #[test]
    fn clone_is_independent() {
        let mut body = Body::<f64>::new();
        let p = body.add_point(origin());
        body.add_vertex(
            Vertex {
                point: p,
                emanating: None,
            },
            prov(),
        );

        let mut cloned = body.clone();
        let p2 = cloned.add_point(Point3::new(1.0, 0.0, 0.0));
        let v2 = cloned.add_vertex(
            Vertex {
                point: p2,
                emanating: None,
            },
            prov(),
        );

        // The clone grew; the original did not (deep copy, no aliasing).
        assert_eq!(cloned.vertices().count(), 2);
        assert_eq!(cloned.points().count(), 2);
        assert_eq!(body.vertices().count(), 1);
        assert_eq!(body.points().count(), 1);
        // The clone's new key resolves in the clone only.
        assert!(cloned.get_vertex(v2).is_some());
        assert!(body.get_vertex(v2).is_none());
        // Provenance maps are independent too.
        assert_eq!(cloned.provenance(EntityId::Vertex(v2)), Some(&prov()));
        assert_eq!(body.provenance(EntityId::Vertex(v2)), None);
    }

    // ------------------------------------------------------------------
    // Derived accessors: mate / end / loop cycle / vertex orbit.
    // ------------------------------------------------------------------

    #[test]
    fn mate_is_a_computed_involution() {
        let t = pillow(Tol::witness());
        // e0's halves are a0 (v0 → v1 in face A) and b0 (v1 → v0 in
        // face B).
        assert_eq!(t.body.mate(t.hes_a[0]), Some(t.hes_b[0]));
        assert_eq!(t.body.mate(t.hes_b[0]), Some(t.hes_a[0]));
        // Involution over all four half-edges.
        for he in t.hes_a.iter().chain(&t.hes_b) {
            let mate = t.body.mate(*he).unwrap();
            assert_eq!(t.body.mate(mate), Some(*he));
            assert_ne!(mate, *he);
        }
    }

    #[test]
    fn mate_is_none_on_stale_or_unclaiming_edge() {
        let mut t = pillow(Tol::witness());
        assert_eq!(t.body.mate(HalfEdgeKey::default()), None);
        // Repoint e0's plus slot away from a0: e0 no longer claims a0, so
        // a0 has no mate (a corrupt bijection the validator reports).
        t.body.get_edge_mut(t.edges[0]).unwrap().he_plus = t.hes_a[1];
        assert_eq!(t.body.mate(t.hes_a[0]), None);
    }

    #[test]
    fn half_edge_end_is_derived_from_next() {
        let t = pillow(Tol::witness());
        // a0 runs v0 → v1; its end is the start of its next (a1).
        assert_eq!(t.body.half_edge_end(t.hes_a[0]), Some(t.vertices[1]));
        assert_eq!(t.body.half_edge_end(t.hes_a[1]), Some(t.vertices[0]));
        assert_eq!(t.body.half_edge_end(HalfEdgeKey::default()), None);
    }

    #[test]
    fn loop_cycle_walks_in_next_order() {
        let t = pillow(Tol::witness());
        assert_eq!(
            t.body.loop_cycle(t.hes_a[0]),
            Some(vec![t.hes_a[0], t.hes_a[1]])
        );
        // Starting elsewhere in the same cycle rotates the result.
        assert_eq!(
            t.body.loop_cycle(t.hes_a[1]),
            Some(vec![t.hes_a[1], t.hes_a[0]])
        );
        assert_eq!(t.body.loop_cycle(HalfEdgeKey::default()), None);
    }

    /// **A `Closed` walk proves `next(first)` resolves.** This is the
    /// property [`Body::kef`] consumes: it takes `b = next(he)` as
    /// proven by `loop_cycle(he)` succeeding rather than checking it,
    /// then passes `b` to a helper that announces on a dead key.
    ///
    /// [`Body::bounded_walk`] holds that property twice over — by the
    /// explicit `contains_key(next)`, and by `step` itself, which is a
    /// `half_edges.get(..)` and so returns `None` when asked to
    /// advance from a dead key one iteration later. Either alone
    /// suffices, which is why reordering the explicit check against
    /// the `next == first` return does NOT break `kef`: a `next` equal
    /// to `first` is live by the walk's entry check. What would break
    /// it is a rewrite that drops both — and dropping both is hard to
    /// do by accident, because reading `he.next` at all requires
    /// resolving `he`, so a step proves its own source and the only
    /// unchecked candidate is a `next` equal to the entry-checked
    /// `first`. **So this row is a cheap statement of a consumed
    /// property, not a guard**: no small mutation of `bounded_walk`
    /// was found that makes it red. Its value is that a future rewrite
    /// of the walk has to keep the sentence true.
    #[test]
    fn a_closed_walk_proves_the_first_step_resolves() {
        let mut t = pillow(Tol::witness());
        // Dangle the very link kef consumes: next(first).
        let first = t.hes_a[0];
        t.body.get_half_edge_mut(first).unwrap().next = HalfEdgeKey::default();
        assert_eq!(
            t.body.loop_cycle(first),
            None,
            "a walk whose first step dangles must not report Closed; kef \
             reads next(he) as proven by this walk succeeding"
        );
    }

    #[test]
    fn vertex_orbit_visits_the_half_edges_starting_at_the_vertex() {
        let t = pillow(Tol::witness());
        // Half-edges starting at v0: a0 (face A) and b1 (face B). Orbit
        // step next(mate(a0)) = next(b0) = b1.
        assert_eq!(
            t.body.vertex_orbit(t.hes_a[0]),
            Some(vec![t.hes_a[0], t.hes_b[1]])
        );
        // Both members start at v0.
        for he in t.body.vertex_orbit(t.hes_a[0]).unwrap() {
            assert_eq!(t.body.get_half_edge(he).unwrap().start, t.vertices[0]);
        }
        assert_eq!(t.body.vertex_orbit(HalfEdgeKey::default()), None);
    }

    #[test]
    fn walks_are_bounded_on_corrupt_bodies() {
        // Overrun: point a0's next into loop B's cycle — the walk from a0
        // can never return to a0, and must terminate as None rather than
        // spin (D9: the kernel never hangs on any input).
        let mut t = pillow(Tol::witness());
        let b0 = t.hes_b[0];
        t.body.get_half_edge_mut(t.hes_a[0]).unwrap().next = b0;
        assert_eq!(t.body.loop_cycle(t.hes_a[0]), None);
        assert_eq!(t.body.vertex_orbit(t.hes_a[1]), None);

        // Broken: a stale link mid-walk is also None, not a panic.
        let mut t = pillow(Tol::witness());
        let dead = t.body.add_half_edge(
            HalfEdge {
                edge: t.edges[0],
                start: t.vertices[0],
                parent_loop: t.loop_a,
                next: t.hes_a[0],
                prev: t.hes_a[0],
            },
            prov(),
        );
        t.body.half_edges.remove(dead);
        t.body.get_half_edge_mut(t.hes_a[0]).unwrap().next = dead;
        assert_eq!(t.body.loop_cycle(t.hes_a[0]), None);
    }

    #[test]
    fn mvfs_state_reads_back() {
        let t = mvfs_state();
        let vertex = t.body.get_vertex(t.vertex).unwrap();
        assert_eq!(vertex.emanating, None);
        let loop_ = t.body.get_loop(t.lone_loop).unwrap();
        assert_eq!(
            loop_.boundary,
            crate::entity::LoopBoundary::Empty { vertex: t.vertex }
        );
        assert_eq!(loop_.face, t.face);
    }
}

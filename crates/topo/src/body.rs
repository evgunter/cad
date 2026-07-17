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

use geom_core::{Point3, Real};
use slotmap::{SecondaryMap, SlotMap};

use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, HalfEdge, HalfEdgeKey, Loop, LoopKey, Shell, ShellKey,
    Solid, SolidKey, Vertex, VertexKey,
};
use crate::geometry::{CurveGeom, CurveKey, PointKey, SurfaceGeom, SurfaceKey};
use crate::provenance::Provenance;

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
/// use topo::{Body, Provenance, Vertex};
///
/// let mut body = Body::<f64>::new();
/// let p = body.add_point(Point3::new(0.0, 0.0, 0.0));
/// let v = body.add_vertex(
///     Vertex { point: p, emanating: None },
///     Provenance::Primordial { op: "doc" },
/// );
/// // A `VertexKey` is not a `ShellKey`: this example must NOT compile.
/// let _ = body.get_shell(v);
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
    // Geometry arenas — the only `T`-carrying storage.
    pub(crate) points: SlotMap<PointKey, Point3<T>>,
    pub(crate) curves: SlotMap<CurveKey, CurveGeom<T>>,
    pub(crate) surfaces: SlotMap<SurfaceKey, SurfaceGeom<T>>,
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
            solid_provenance: SecondaryMap::new(),
            shell_provenance: SecondaryMap::new(),
            face_provenance: SecondaryMap::new(),
            loop_provenance: SecondaryMap::new(),
            half_edge_provenance: SecondaryMap::new(),
            edge_provenance: SecondaryMap::new(),
            vertex_provenance: SecondaryMap::new(),
        }
    }

    // ------------------------------------------------------------------
    // Raw insertion — placeholder builder API until PR 5.
    //
    // M1's Euler operators become the ONLY sanctioned construction path
    // (D1: topology is built exclusively through validity-preserving
    // Euler ops); these raw insertions then retreat behind them (the
    // `pub(crate)` demotion is PR 5). They are `pub` for now — not
    // `pub(crate)` — because construction has to be exercisable from
    // integration tests, doctests, and e2e review demos, all of which sit
    // outside the crate. Raw insertion makes NO validity promises: it can
    // build arbitrarily malformed bodies by design — judging the result
    // is `topo::validate`'s job.
    //
    // The half-edge structure is cyclic (next/prev cycles, the edge ↔
    // half-edge bijection, spine back-pointers), so pure insertion in
    // any order cannot close a coherent body — not without forging
    // slotmap keys (`KeyData::from_ffi` plus deterministic minting makes
    // the forge reliable, but it is a hack, not an API) or a batch-graph
    // constructor. The raw builder therefore pairs each `add_*` with a
    // `get_*_mut` patching accessor: insert with provisional keys (e.g.
    // `Default::default()` null keys, which never resolve), then patch
    // the cycles closed.
    // Both halves of the builder retreat to `pub(crate)` together at
    // PR 5 — the Euler operators never need external patching because
    // each operator is itself a complete surgery.
    // ------------------------------------------------------------------

    /// Inserts a point (vertex geometry), returning its key.
    ///
    /// Placeholder raw insertion (see the builder-API note in the
    /// source): no validity promises.
    pub fn add_point(&mut self, point: Point3<T>) -> PointKey {
        self.points.insert(point)
    }

    /// Inserts curve geometry, returning its key.
    ///
    /// Placeholder raw insertion: no validity promises.
    pub fn add_curve(&mut self, curve: CurveGeom<T>) -> CurveKey {
        self.curves.insert(curve)
    }

    /// Inserts surface geometry, returning its key.
    ///
    /// Placeholder raw insertion: no validity promises.
    pub fn add_surface(&mut self, surface: SurfaceGeom<T>) -> SurfaceKey {
        self.surfaces.insert(surface)
    }

    /// Inserts a vertex with its birth provenance (D5), returning its key.
    ///
    /// Placeholder raw insertion: no validity promises (the referenced
    /// point may dangle — the validator reports it).
    pub fn add_vertex(&mut self, vertex: Vertex, provenance: Provenance) -> VertexKey {
        let key = self.vertices.insert(vertex);
        self.vertex_provenance.insert(key, provenance);
        key
    }

    /// Inserts a half-edge with its birth provenance (D5), returning its
    /// key.
    ///
    /// Placeholder raw insertion: no validity promises. Because
    /// `next`/`prev` form cycles, callers typically insert with
    /// provisional keys and patch via [`Body::get_half_edge_mut`].
    pub fn add_half_edge(&mut self, half_edge: HalfEdge, provenance: Provenance) -> HalfEdgeKey {
        let key = self.half_edges.insert(half_edge);
        self.half_edge_provenance.insert(key, provenance);
        key
    }

    /// Inserts an edge with its birth provenance (D5), returning its key.
    ///
    /// Placeholder raw insertion: no validity promises.
    pub fn add_edge(&mut self, edge: Edge, provenance: Provenance) -> EdgeKey {
        let key = self.edges.insert(edge);
        self.edge_provenance.insert(key, provenance);
        key
    }

    /// Inserts a loop with its birth provenance (D5), returning its key.
    ///
    /// Placeholder raw insertion: no validity promises.
    pub fn add_loop(&mut self, loop_: Loop, provenance: Provenance) -> LoopKey {
        let key = self.loops.insert(loop_);
        self.loop_provenance.insert(key, provenance);
        key
    }

    /// Inserts a face with its birth provenance (D5), returning its key.
    ///
    /// Placeholder raw insertion: no validity promises.
    pub fn add_face(&mut self, face: Face, provenance: Provenance) -> FaceKey {
        let key = self.faces.insert(face);
        self.face_provenance.insert(key, provenance);
        key
    }

    /// Inserts a shell with its birth provenance (D5), returning its key.
    ///
    /// Placeholder raw insertion: no validity promises.
    pub fn add_shell(&mut self, shell: Shell, provenance: Provenance) -> ShellKey {
        let key = self.shells.insert(shell);
        self.shell_provenance.insert(key, provenance);
        key
    }

    /// Inserts a solid with its birth provenance (D5), returning its key.
    ///
    /// Placeholder raw insertion: no validity promises.
    pub fn add_solid(&mut self, solid: Solid, provenance: Provenance) -> SolidKey {
        let key = self.solids.insert(solid);
        self.solid_provenance.insert(key, provenance);
        key
    }

    // ------------------------------------------------------------------
    // Raw mutation — the patching half of the placeholder builder (see
    // the raw-insertion note above for why cyclic references force it).
    // Total like the lookups: stale keys yield `None`. No validity
    // promises; retreats to `pub(crate)` with the rest at PR 5.
    // ------------------------------------------------------------------

    /// Mutable access to the solid at `key` (raw-builder patching; see
    /// the source note on cyclic references). `None` if the key is stale.
    pub fn get_solid_mut(&mut self, key: SolidKey) -> Option<&mut Solid> {
        self.solids.get_mut(key)
    }

    /// Mutable access to the shell at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub fn get_shell_mut(&mut self, key: ShellKey) -> Option<&mut Shell> {
        self.shells.get_mut(key)
    }

    /// Mutable access to the face at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub fn get_face_mut(&mut self, key: FaceKey) -> Option<&mut Face> {
        self.faces.get_mut(key)
    }

    /// Mutable access to the loop at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub fn get_loop_mut(&mut self, key: LoopKey) -> Option<&mut Loop> {
        self.loops.get_mut(key)
    }

    /// Mutable access to the half-edge at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub fn get_half_edge_mut(&mut self, key: HalfEdgeKey) -> Option<&mut HalfEdge> {
        self.half_edges.get_mut(key)
    }

    /// Mutable access to the edge at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub fn get_edge_mut(&mut self, key: EdgeKey) -> Option<&mut Edge> {
        self.edges.get_mut(key)
    }

    /// Mutable access to the vertex at `key` (raw-builder patching).
    /// `None` if the key is stale.
    pub fn get_vertex_mut(&mut self, key: VertexKey) -> Option<&mut Vertex> {
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
    /// (`kemr`; PR 4's `kev`/`kef`): in Euler-built bodies every edge
    /// owns its own placeholder curve, but the scan keeps the op sound
    /// standalone. Deterministic (D9): a full sweep of the edge arena —
    /// the outcome is a pure function of the arena contents. A stale
    /// `curve` key is a no-op returning `false`.
    pub(crate) fn remove_curve_if_orphaned(&mut self, curve: CurveKey) -> bool {
        if self.edges.values().any(|edge| edge.curve == curve) {
            return false;
        }
        self.curves.remove(curve).is_some()
    }

    /// Removes `surface` from the surface arena iff no face references
    /// it, returning whether it was removed. Used by face-killing
    /// operators (`kfmrh`; PR 4's `kef`): in M1 constructions all faces
    /// share `mvfs`'s surface (so nothing is removed), but the scan
    /// keeps the op sound standalone. Deterministic (D9), same shape as
    /// [`Body::remove_curve_if_orphaned`].
    pub(crate) fn remove_surface_if_orphaned(&mut self, surface: SurfaceKey) -> bool {
        if self.faces.values().any(|face| face.surface == surface) {
            return false;
        }
        self.surfaces.remove(surface).is_some()
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

    /// The curve geometry at `key`, or `None` if the key is stale (a
    /// foreign key is not caught — see the [module docs](self)).
    pub fn get_curve(&self, key: CurveKey) -> Option<&CurveGeom<T>> {
        self.curves.get(key)
    }

    /// The surface geometry at `key`, or `None` if the key is stale (a
    /// foreign key is not caught — see the [module docs](self)).
    pub fn get_surface(&self, key: SurfaceKey) -> Option<&SurfaceGeom<T>> {
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

    /// All curve geometry, in slot-index order (deterministic per D9).
    pub fn curves(&self) -> impl Iterator<Item = (CurveKey, &CurveGeom<T>)> {
        self.curves.iter()
    }

    /// All surface geometry, in slot-index order (deterministic per D9).
    pub fn surfaces(&self) -> impl Iterator<Item = (SurfaceKey, &SurfaceGeom<T>)> {
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
        let t = pillow();
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
        // Unit tests may reach into the pub(crate) arenas; removal has no
        // public API yet (bodies are values, and only M1 PR 4's kill-side
        // Euler ops introduce sanctioned removal).
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
        let t = pillow();
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
        let t = pillow();
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
        let mut t = pillow();
        assert_eq!(t.body.mate(HalfEdgeKey::default()), None);
        // Repoint e0's plus slot away from a0: e0 no longer claims a0, so
        // a0 has no mate (a corrupt bijection the validator reports).
        t.body.get_edge_mut(t.edges[0]).unwrap().he_plus = t.hes_a[1];
        assert_eq!(t.body.mate(t.hes_a[0]), None);
    }

    #[test]
    fn half_edge_end_is_derived_from_next() {
        let t = pillow();
        // a0 runs v0 → v1; its end is the start of its next (a1).
        assert_eq!(t.body.half_edge_end(t.hes_a[0]), Some(t.vertices[1]));
        assert_eq!(t.body.half_edge_end(t.hes_a[1]), Some(t.vertices[0]));
        assert_eq!(t.body.half_edge_end(HalfEdgeKey::default()), None);
    }

    #[test]
    fn loop_cycle_walks_in_next_order() {
        let t = pillow();
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

    #[test]
    fn vertex_orbit_visits_the_half_edges_starting_at_the_vertex() {
        let t = pillow();
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
        let mut t = pillow();
        let b0 = t.hes_b[0];
        t.body.get_half_edge_mut(t.hes_a[0]).unwrap().next = b0;
        assert_eq!(t.body.loop_cycle(t.hes_a[0]), None);
        assert_eq!(t.body.vertex_orbit(t.hes_a[1]), None);

        // Broken: a stale link mid-walk is also None, not a panic.
        let mut t = pillow();
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

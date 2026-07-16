//! [`Body<T>`]: the arena store a B-rep *is* — scalar-free topology
//! arenas plus `T`-valued geometry arenas (Q1's genericity boundary).
//!
//! A body is a plain value (D1): cheaply cloneable, serializable later,
//! validatable now. It owns nine slotmap arenas — six topology kinds, each
//! with its own key type, and three geometry kinds — plus one D5
//! provenance `SecondaryMap` per topology kind.
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
    Edge, EdgeKey, EntityId, Face, FaceKey, Loop, LoopKey, Shell, ShellKey, Solid, SolidKey,
    Vertex, VertexKey,
};
use crate::geometry::{CurveGeom, CurveKey, PointKey, SurfaceGeom, SurfaceKey};
use crate::provenance::Provenance;

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
/// let v = body.add_vertex(Vertex { point: p }, Provenance::Primordial { op: "doc" });
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
    pub(crate) edges: SlotMap<EdgeKey, Edge>,
    pub(crate) vertices: SlotMap<VertexKey, Vertex>,
    // Geometry arenas — the only `T`-carrying storage.
    pub(crate) points: SlotMap<PointKey, Point3<T>>,
    pub(crate) curves: SlotMap<CurveKey, CurveGeom<T>>,
    pub(crate) surfaces: SlotMap<SurfaceKey, SurfaceGeom<T>>,
    // D5 provenance, parallel to the topology arenas (see
    // `crate::provenance` for the SecondaryMap-vs-inline rationale).
    pub(crate) solid_provenance: SecondaryMap<SolidKey, Provenance>,
    pub(crate) shell_provenance: SecondaryMap<ShellKey, Provenance>,
    pub(crate) face_provenance: SecondaryMap<FaceKey, Provenance>,
    pub(crate) loop_provenance: SecondaryMap<LoopKey, Provenance>,
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
            edges: SlotMap::with_key(),
            vertices: SlotMap::with_key(),
            points: SlotMap::with_key(),
            curves: SlotMap::with_key(),
            surfaces: SlotMap::with_key(),
            solid_provenance: SecondaryMap::new(),
            shell_provenance: SecondaryMap::new(),
            face_provenance: SecondaryMap::new(),
            loop_provenance: SecondaryMap::new(),
            edge_provenance: SecondaryMap::new(),
            vertex_provenance: SecondaryMap::new(),
        }
    }

    // ------------------------------------------------------------------
    // Raw insertion — M1-placeholder builder API.
    //
    // M1's Euler operators become the ONLY sanctioned construction path
    // (D1: topology is built exclusively through validity-preserving
    // Euler ops); these raw insertions then retreat behind them. They are
    // `pub` at M0 — not `pub(crate)` — because construction has to be
    // exercisable from integration tests, doctests, and e2e review demos,
    // all of which sit outside the crate. Raw insertion makes NO validity
    // promises: it can build arbitrarily malformed bodies by design —
    // judging the result is `topo::validate`'s job.
    // ------------------------------------------------------------------

    /// Inserts a point (vertex geometry), returning its key.
    ///
    /// M1-placeholder raw insertion (see the builder-API note in the
    /// source): no validity promises.
    pub fn add_point(&mut self, point: Point3<T>) -> PointKey {
        self.points.insert(point)
    }

    /// Inserts curve geometry, returning its key.
    ///
    /// M1-placeholder raw insertion: no validity promises.
    pub fn add_curve(&mut self, curve: CurveGeom<T>) -> CurveKey {
        self.curves.insert(curve)
    }

    /// Inserts surface geometry, returning its key.
    ///
    /// M1-placeholder raw insertion: no validity promises.
    pub fn add_surface(&mut self, surface: SurfaceGeom<T>) -> SurfaceKey {
        self.surfaces.insert(surface)
    }

    /// Inserts a vertex with its birth provenance (D5), returning its key.
    ///
    /// M1-placeholder raw insertion: no validity promises (the referenced
    /// point may dangle — the validator reports it).
    pub fn add_vertex(&mut self, vertex: Vertex, provenance: Provenance) -> VertexKey {
        let key = self.vertices.insert(vertex);
        self.vertex_provenance.insert(key, provenance);
        key
    }

    /// Inserts an edge with its birth provenance (D5), returning its key.
    ///
    /// M1-placeholder raw insertion: no validity promises.
    pub fn add_edge(&mut self, edge: Edge, provenance: Provenance) -> EdgeKey {
        let key = self.edges.insert(edge);
        self.edge_provenance.insert(key, provenance);
        key
    }

    /// Inserts a loop with its birth provenance (D5), returning its key.
    ///
    /// M1-placeholder raw insertion: no validity promises.
    pub fn add_loop(&mut self, loop_: Loop, provenance: Provenance) -> LoopKey {
        let key = self.loops.insert(loop_);
        self.loop_provenance.insert(key, provenance);
        key
    }

    /// Inserts a face with its birth provenance (D5), returning its key.
    ///
    /// M1-placeholder raw insertion: no validity promises.
    pub fn add_face(&mut self, face: Face, provenance: Provenance) -> FaceKey {
        let key = self.faces.insert(face);
        self.face_provenance.insert(key, provenance);
        key
    }

    /// Inserts a shell with its birth provenance (D5), returning its key.
    ///
    /// M1-placeholder raw insertion: no validity promises.
    pub fn add_shell(&mut self, shell: Shell, provenance: Provenance) -> ShellKey {
        let key = self.shells.insert(shell);
        self.shell_provenance.insert(key, provenance);
        key
    }

    /// Inserts a solid with its birth provenance (D5), returning its key.
    ///
    /// M1-placeholder raw insertion: no validity promises.
    pub fn add_solid(&mut self, solid: Solid, provenance: Provenance) -> SolidKey {
        let key = self.solids.insert(solid);
        self.solid_provenance.insert(key, provenance);
        key
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
            EntityId::Edge(k) => self.edge_provenance.get(k),
            EntityId::Vertex(k) => self.vertex_provenance.get(k),
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

    fn prov(op: &'static str) -> Provenance {
        Provenance::Primordial { op }
    }

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
        assert_eq!(body.edges().count(), 0);
        assert_eq!(body.vertices().count(), 0);
        assert_eq!(body.points().count(), 0);
        assert_eq!(body.curves().count(), 0);
        assert_eq!(body.surfaces().count(), 0);
    }

    #[test]
    fn insertion_roundtrips_through_lookup() {
        let mut body = Body::<f64>::new();
        let p = body.add_point(Point3::new(1.0, 2.0, 3.0));
        let v = body.add_vertex(Vertex { point: p }, prov("t"));
        let c = body.add_curve(CurveGeom::Placeholder { anchor: origin() });
        let e = body.add_edge(
            Edge {
                start: v,
                end: v,
                curve: c,
            },
            prov("t"),
        );
        let lp = body.add_loop(Loop { edges: vec![e] }, prov("t"));
        let s = body.add_surface(SurfaceGeom::Placeholder { anchor: origin() });
        let f = body.add_face(
            Face {
                surface: s,
                loops: vec![lp],
            },
            prov("t"),
        );
        let sh = body.add_shell(Shell { faces: vec![f] }, prov("t"));
        let so = body.add_solid(Solid { shells: vec![sh] }, prov("t"));

        // Every lookup resolves and the containment spine reads back.
        assert_eq!(body.get_vertex(v).unwrap().point, p);
        let edge = body.get_edge(e).unwrap();
        assert_eq!((edge.start, edge.end, edge.curve), (v, v, c));
        assert_eq!(body.get_loop(lp).unwrap().edges, vec![e]);
        let face = body.get_face(f).unwrap();
        assert_eq!(face.surface, s);
        assert_eq!(face.loops, vec![lp]);
        assert_eq!(body.get_shell(sh).unwrap().faces, vec![f]);
        assert_eq!(body.get_solid(so).unwrap().shells, vec![sh]);
        assert!(body.get_point(p).is_some());
        assert!(body.get_curve(c).is_some());
        assert!(body.get_surface(s).is_some());
    }

    #[test]
    fn stale_key_lookup_is_none_not_panic() {
        let mut body = Body::<f64>::new();
        let sh = body.add_shell(Shell { faces: vec![] }, prov("t"));
        // Unit tests may reach into the pub(crate) arenas; removal has no
        // public API at M0 (bodies are values, operators never shrink them
        // — M1's kill-side Euler ops introduce sanctioned removal).
        body.shells.remove(sh);
        assert!(body.get_shell(sh).is_none());
        // Reusing the freed slot bumps the version: the stale key stays
        // stale even though the slot is occupied again (generational keys,
        // D1).
        let sh2 = body.add_shell(Shell { faces: vec![] }, prov("t"));
        assert!(body.get_shell(sh).is_none());
        assert!(body.get_shell(sh2).is_some());
    }

    #[test]
    fn provenance_is_recorded_at_birth_for_every_kind() {
        let mut body = Body::<f64>::new();
        let p = body.add_point(origin());
        let v = body.add_vertex(Vertex { point: p }, prov("v"));
        let c = body.add_curve(CurveGeom::Placeholder { anchor: origin() });
        let e = body.add_edge(
            Edge {
                start: v,
                end: v,
                curve: c,
            },
            prov("e"),
        );
        let lp = body.add_loop(Loop { edges: vec![e] }, prov("l"));
        let s = body.add_surface(SurfaceGeom::Placeholder { anchor: origin() });
        let f = body.add_face(
            Face {
                surface: s,
                loops: vec![lp],
            },
            prov("f"),
        );
        let sh = body.add_shell(Shell { faces: vec![f] }, prov("sh"));
        let so = body.add_solid(Solid { shells: vec![sh] }, prov("so"));

        let cases = [
            (EntityId::Vertex(v), "v"),
            (EntityId::Edge(e), "e"),
            (EntityId::Loop(lp), "l"),
            (EntityId::Face(f), "f"),
            (EntityId::Shell(sh), "sh"),
            (EntityId::Solid(so), "so"),
        ];
        for (id, op) in cases {
            assert_eq!(body.provenance(id), Some(&prov(op)), "for {id}");
        }
    }

    #[test]
    fn clone_is_independent() {
        let mut body = Body::<f64>::new();
        let p = body.add_point(origin());
        body.add_vertex(Vertex { point: p }, prov("t"));

        let mut cloned = body.clone();
        let p2 = cloned.add_point(Point3::new(1.0, 0.0, 0.0));
        let v2 = cloned.add_vertex(Vertex { point: p2 }, prov("t2"));

        // The clone grew; the original did not (deep copy, no aliasing).
        assert_eq!(cloned.vertices().count(), 2);
        assert_eq!(cloned.points().count(), 2);
        assert_eq!(body.vertices().count(), 1);
        assert_eq!(body.points().count(), 1);
        // The clone's new key resolves in the clone only.
        assert!(cloned.get_vertex(v2).is_some());
        assert!(body.get_vertex(v2).is_none());
        // Provenance maps are independent too.
        assert_eq!(cloned.provenance(EntityId::Vertex(v2)), Some(&prov("t2")));
        assert_eq!(body.provenance(EntityId::Vertex(v2)), None);
    }
}

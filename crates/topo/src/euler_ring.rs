//! Ring/genus Euler operators — [`Body::kemr`], [`Body::mekr`],
//! [`Body::kfmrh`] — plus the [`Body::ring_move`] helper (M1 PR 3).
//!
//! These are the operators that create and consume **rings** (interior
//! loops of faces) and, through `kfmrh`, **genus** — the connected sum
//! that turns a blind pocket into a through-hole (Mäntylä §9.2.3–§9.2.4;
//! ch. 11 surgeries re-derived under our conventions). They share the
//! operator contracts of [`crate::euler`] — atomic typed-error
//! preconditions, deterministic minting, typed D5 provenance, debug
//! postconditions — and extend them with the **kill-side duties** that
//! the make-only operators never had:
//!
//! - **Kill hygiene.** A killed entity's arena slot AND its D5
//!   provenance `SecondaryMap` entry are removed together (a provenance
//!   record outliving its entity is a leak — PR 5's bidirectional check
//!   makes leaks loud). Killed keys returned in the result structs are
//!   **dead**: they no longer resolve; they are returned for the
//!   caller's records only.
//! - **Geometry hygiene.** Killing an edge/face may orphan its
//!   curve/surface; the operators remove the geometry iff nothing else
//!   references it ([`Body::remove_curve_if_orphaned`] /
//!   [`Body::remove_surface_if_orphaned`] — deterministic reference
//!   scans), else tier 1's `OrphanGeometry` would fire.
//! - **Survivor re-anchoring** (GWB's `delhe` hazards, §11.3.4): a
//!   surviving loop's `Cycle::first` and a surviving vertex's
//!   `emanating` may point at killed half-edges. Both are overwritten
//!   **unconditionally** by documented deterministic rules (branch-free,
//!   replay-deterministic — PR 2's `emanating` style), never patched
//!   only-when-dangling.
//!
//! # Kills and lineage (D9)
//!
//! Deterministic minting extends to kills: identical operator sequences
//! (including the kills) free and reuse identical slots, so replay
//! remains bit-identical. But a kill is *part of the construction
//! history*: a history that includes a kemr∘mekr roundtrip re-mints the
//! killed entities in **recycled slots with bumped generations**, so its
//! keys differ from a history without the pair — that is expected, not
//! a leak (pinned by test). Once the freed slots are consumed, minting
//! converges again: entities created after a balanced kill/make pair get
//! the same keys they would have gotten without it.
//!
//! # Which side becomes the ring (`kemr`'s association, ratified)
//!
//! `kemr(he1, he2)` kills an edge whose two halves lie in one loop; the
//! cycle then falls apart into two components. **`he1`'s side — the
//! half-edges strictly between `he1` and `he2` in `next` order — becomes
//! the NEW loop, the ring**; `he2`'s side keeps the old loop's key (and
//! its outer/ring designation — an outer loop stays outer). This is
//! uniform with `mef`'s "he1's side is the new thing". GWB's `lkemr`
//! states the same convention from the other argument's perspective
//! ("the component containing h2 becomes the new loop": after its
//! `nxt`-exchange trick, h2 sits in the successor-side of h1 — exactly
//! our he1's side), so the *content* agrees; only the description is
//! keyed to the other argument. Callers wanting the other association
//! swap the arguments (as GWB §11.5.1 advises). Like `mef`'s, the
//! association is orientation-neutral — nothing here needed mirroring
//! for our CCW convention.
//!
//! # Empty-loop anchors (`kemr`'s degenerate cases, derived)
//!
//! By antiparallelism the killed edge runs between `u = start(he1)
//! = end(he2)` and `w = start(he2) = end(he1)`. If a side of the split
//! is empty (the two halves adjacent in the cycle), the vertex where the
//! cycle was cut on that side is stranded, and on tier-1-valid input it
//! is stranded *completely* (`next(he1) == he2` forces `w`'s orbit to be
//! exactly `{he2}` — valence 1 — and symmetrically for `u`):
//!
//! - **ring side empty** (`next(he1) == he2`): the ring is
//!   [`LoopBoundary::Empty`] **at `w`** — THE hole-planting state of
//!   Mäntylä §9.3 step (g) (strut kill → lone vertex ring);
//! - **old side empty** (`next(he2) == he1`): the old loop becomes
//!   `Empty` **at `u`**;
//! - **both sides empty** (the loop was the 2-cycle `[he1, he2]`, a
//!   segment loop): both rules apply — old loop `Empty` at `u`, ring
//!   `Empty` at `w`. If `u == w` (the segment loop's edge is a
//!   self-loop) the two empty loops would share one lone vertex, which
//!   tier 1 forbids: typed error
//!   [`EulerOpError::EmptyAnchorsCollide`], believed unreachable through
//!   valid operator sequences (such a body is already tier-1-invalid —
//!   its vertex orbit is split), checked defensively.
//!
//! # Deterministic re-anchoring rules (`kemr`)
//!
//! - **`Cycle::first`**, unconditionally: the old loop re-anchors at
//!   `next(he2)` (the first survivor of its side in `next` order), the
//!   ring at `next(he1)` — whether or not the previous `first` was
//!   killed or migrated.
//! - **`emanating`**, unconditionally: `u` gets `Some(next(he2))`
//!   (which starts at `u`) or `None` if its side is empty; `w` gets
//!   `Some(next(he1))` or `None`. When `u == w` (self-loop edge with
//!   non-empty sides) the `w`-write is last and wins — deterministic.
//!
//! # `mekr` — the inverse (site-typed, like `mev`/`mef`)
//!
//! `mekr` joins a **ring** of a face back into another loop (the
//! **target**) of the same face with a new edge; the ring's loop key is
//! KILLED (its provenance entry removed), its half-edges (if any)
//! splice into the target loop. The typed-`Empty` loop state means the
//! degenerate configurations are addressed explicitly ([`MekrSite`], one
//! variant per boundary-shape combination — covering the inverse of
//! every `kemr` output). Edge orientation, uniform with PR 2:
//! **`he_plus` runs from the target-loop anchor vertex `u` toward the
//! ring anchor vertex `w`** (`he_plus` plays the killed `he1`'s role in
//! the reconstructed cycle, `he_minus` plays `he2`'s). Both new halves
//! land in the target loop. Re-anchoring, unconditionally: the target
//! loop's `Cycle::first` becomes `he_plus`; `u.emanating = he_plus`,
//! `w.emanating = he_minus` (`mev`'s rule).
//!
//! # `kfmrh` — the connected sum
//!
//! `kfmrh(f1, f2)` makes `f2`'s outer loop a ring of `f1` and kills
//! `f2`. Same shell ⇒ +1 genus (the through-hole finisher, §9.3 step
//! (l)); **cross-shell is a typed error until M3**
//! ([`EulerOpError::CrossShell`] — applied across shells the operator
//! would merge them, and multi-shell solids arrive with M3's
//! splitting/booleans, per the ratified plan). `f2` must carry **no
//! rings** (move them off first with [`Body::ring_move`]); its outer
//! loop may be `Empty` or `Cycle` — an `Empty` outer becomes an `Empty`
//! ring of `f1`. No half-edge, vertex, or edge is touched: the
//! manipulation is purely at the loop/face/shell level (a "truly global"
//! operation, Mäntylä §9.2.4).
//!
//! # `ring_move` — NOT an Euler operator
//!
//! [`Body::ring_move`] reparents a ring from its face to another face of
//! the same shell. It is **not an Euler operator**: the Euler vector is
//! unchanged, nothing is created or killed, and no provenance changes —
//! D5 records are *birth* records, and reparenting is not a re-birth. It
//! exists because `mef` deliberately does **not** reclassify the split
//! face's rings (Mäntylä p. 192: `lmef` leaves all rings in the old
//! face; the non-Euler `lringmv` fixes them up afterwards when the
//! geometry demands it).
//!
//! # Surgery diagrams
//!
//! `kemr` (general case; `∗` marks killed halves):
//!
//! ```text
//!            before                              after
//!     ┌── he1* ─────────┐                ┌────────────────┐
//!     u                 w                │      RING      │ he1's side
//!     │   (one loop)    │                │ next(he1) ….   │
//!     w                 u                └────────────────┘
//!     └───────── he2* ──┘                ┌────────────────┐
//!                                        │    OLD LOOP    │ he2's side
//!       cycle: he1 → [ring side]         │ next(he2) ….   │
//!              → he2 → [old side] →      └────────────────┘
//! ```
//!
//! `mekr` (`Cycles` site) rebuilds exactly that shape: with target
//! anchor `t` (starting at `u`) and ring anchor `r` (starting at `w`),
//! the merged cycle is
//!
//! ```text
//!   he_plus(u→w) → r → … → prev(r) → he_minus(w→u) → t → … → prev(t) ┐
//!   └──────────────────────────────────────────────────────────────┘
//! ```
//!
//! so `kemr(he1, he2)` followed by
//! `mekr(Cycles { target: next(he2), ring: next(he1) })` restores the
//! original cycle order (with fresh keys for the edge, halves — and the
//! surviving loop is the original; the ring key `kemr` minted dies).
//!
//! # Example: plant a hole anchor, then undo it
//!
//! The §9.3 idiom — strut, kill it, leaving an empty ring — and `mekr`
//! as its exact inverse:
//!
//! ```
//! use geom_core::Point3;
//! use topo::{Body, LoopBoundary, MekrSite, MevSite};
//!
//! # fn run() -> Result<(), topo::EulerOpError> {
//! let mut body = Body::<f64>::new();
//! let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0))?;
//! let seg = body.mev(
//!     MevSite::Lone { r#loop: seed.r#loop },
//!     Point3::new(1.0, 0.0, 0.0),
//! )?;
//! // A strut, then kill it: the far vertex is stranded as an EMPTY RING
//! // of the face — Mäntylä §9.3 step (g), the hole-planting state.
//! let strut = body.mev(
//!     MevSite::Fan { he1: seg.he_minus, he2: seg.he_minus },
//!     Point3::new(2.0, 0.0, 0.0),
//! )?;
//! let kill = body.kemr(strut.he_plus, strut.he_minus)?;
//! assert_eq!(
//!     body.get_loop(kill.ring).unwrap().boundary,
//!     LoopBoundary::Empty { vertex: strut.vertex },
//! );
//! // mekr absorbs the lone-vertex ring back with a new edge.
//! let restore = body.mekr(MekrSite::EmptyRing {
//!     target: seg.he_minus,
//!     ring: kill.ring,
//! })?;
//! assert_eq!(topo::validate(&body), Ok(()));
//! assert!(body.get_loop(kill.ring).is_none()); // the ring key died
//! assert_eq!(body.half_edge_end(restore.he_plus), Some(strut.vertex));
//! # Ok(()) }
//! # run().unwrap();
//! ```

use geom_core::{Point3, Real};

use crate::body::Body;
use crate::entity::{
    EdgeKey, EntityId, FaceKey, HalfEdgeKey, Loop, LoopBoundary, LoopKey, VertexKey,
};
use crate::euler::EulerOpError;
use crate::geometry::{CurveGeom, CurveKey, SurfaceKey};
use crate::provenance::Provenance;

/// Where [`Body::mekr`] acts: the site addressing for "make edge, kill
/// ring".
///
/// Same design as [`crate::MevSite`]/[`crate::MefSite`]: the ratified
/// typed-`Empty` loop state makes every degenerate configuration its own
/// variant instead of a placeholder half-edge. One variant per
/// boundary-shape combination of the two loops, covering the inverse of
/// every [`Body::kemr`] output configuration (general split, ring-side
/// empty, old-side empty, both empty). In every variant the **target**
/// loop survives (and receives the new halves) and the **ring** loop is
/// killed; cycle loops are addressed by a member half-edge, empty loops
/// by their loop key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MekrSite {
    /// Both loops are cycles (the inverse of `kemr`'s general case).
    /// The new edge runs `start(target) → start(ring)`; the merged cycle
    /// is `he_plus → ring-cycle-from-ring → he_minus →
    /// target-cycle-from-target` (module docs diagram).
    Cycles {
        /// A half-edge of the surviving loop; the new edge starts at
        /// `start(target)`, and `he_minus` is spliced immediately before
        /// `target`.
        target: HalfEdgeKey,
        /// A half-edge of the ring to kill; the new edge ends at
        /// `start(ring)`, and `he_plus` is spliced immediately before
        /// `ring` (in the merged cycle).
        ring: HalfEdgeKey,
    },
    /// The ring is an empty loop (lone vertex `w`) and the target a
    /// cycle: absorb the lone vertex via an edge from `start(target)` to
    /// `w` (the inverse of `kemr`'s ring-side-empty case — un-planting
    /// §9.3's hole anchor). The merged cycle inserts
    /// `he_plus → he_minus` immediately before `target`.
    EmptyRing {
        /// A half-edge of the surviving cycle loop.
        target: HalfEdgeKey,
        /// The empty loop to kill; its lone vertex becomes the new
        /// edge's far end.
        ring: LoopKey,
    },
    /// The target is an empty loop (lone vertex `u`) and the ring a
    /// cycle (the inverse of `kemr`'s old-side-empty case): the target
    /// loop's key survives but its boundary grows from `Empty` to the
    /// merged cycle `he_plus → ring-cycle-from-ring → he_minus`.
    EmptyTarget {
        /// The empty loop that survives and receives everything.
        target: LoopKey,
        /// A half-edge of the cycle ring to kill.
        ring: HalfEdgeKey,
    },
    /// Both loops are empty (the inverse of `kemr`'s both-empty case):
    /// two lone vertices of one face joined into a 2-cycle segment loop
    /// (`he_plus → he_minus`) under the target's key. The two lone
    /// vertices must be distinct ([`EulerOpError::EmptyAnchorsCollide`]
    /// otherwise — tier-1-invalid input, checked defensively).
    BothEmpty {
        /// The empty loop that survives (lone vertex `u`, the new
        /// edge's start).
        target: LoopKey,
        /// The empty loop to kill (lone vertex `w`, the new edge's
        /// end).
        ring: LoopKey,
    },
}

/// The outcome of one [`Body::kemr`] call: one created key, three (or
/// four) dead ones.
///
/// The `killed_*` keys **no longer resolve** — they are returned for the
/// caller's records (e.g. cross-referencing an operation log), not for
/// lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KemrResult {
    /// The new ring: `he1`'s side of the split, pushed onto the face's
    /// `rings` (a [`LoopBoundary::Empty`] loop at `start(he2)` when that
    /// side was empty).
    pub ring: LoopKey,
    /// The killed edge (dead key).
    pub killed_edge: EdgeKey,
    /// The killed edge's plus half (dead key). Slot association is the
    /// edge's, not the argument order's: this is `he1` or `he2`
    /// according to which the edge claimed as [`crate::Edge::he_plus`].
    pub killed_he_plus: HalfEdgeKey,
    /// The killed edge's minus half (dead key).
    pub killed_he_minus: HalfEdgeKey,
    /// The killed edge's curve (dead key), if killing the edge orphaned
    /// it and it was removed (geometry hygiene, module docs); `None` if
    /// another edge still references the curve.
    pub killed_curve: Option<CurveKey>,
}

/// The outcome of one [`Body::mekr`] call: the created edge complex and
/// the dead ring key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MekrResult {
    /// The new edge joining the target anchor vertex to the ring anchor
    /// vertex.
    pub edge: EdgeKey,
    /// The plus half: target anchor vertex → ring anchor vertex, in the
    /// target loop (which it re-anchors as `Cycle::first`).
    pub he_plus: HalfEdgeKey,
    /// The minus half: ring anchor vertex → target anchor vertex, also
    /// in the target loop.
    pub he_minus: HalfEdgeKey,
    /// The edge's placeholder curve (M1 geometry policy; anchored at the
    /// target anchor vertex's coordinates).
    pub curve: CurveKey,
    /// The killed ring loop (dead key — no longer resolves).
    pub killed_ring: LoopKey,
}

/// The outcome of one [`Body::kfmrh`] call: nothing created (the ring is
/// the surviving demoted loop), one face — and possibly its orphaned
/// surface — dead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KfmrhResult {
    /// `f2`'s former outer loop, now a ring of `f1`. A **surviving**
    /// key: the loop keeps its identity and its D5 birth record.
    pub ring: LoopKey,
    /// The killed face `f2` (dead key).
    pub killed_face: FaceKey,
    /// `f2`'s surface (dead key), if killing the face orphaned it and it
    /// was removed (geometry hygiene, module docs); `None` if another
    /// face still references the surface (as in every M1 construction,
    /// where all faces share `mvfs`'s surface).
    pub killed_surface: Option<SurfaceKey>,
}

impl<T: Real> Body<T> {
    /// KEMR — *kill edge, make ring*: remove an edge occurring twice in
    /// one loop, splitting the loop's cycle into two components; `he1`'s
    /// side becomes a new **ring** of the same face.
    ///
    /// The ratified side association, the empty-anchor rules, and the
    /// deterministic re-anchoring rules: [module docs](self). `he1` and
    /// `he2` must be the two halves of one edge and lie in the same
    /// loop.
    ///
    /// Euler vector: `(v 0, e −1, f 0, h 0, r +1, s 0)` — arena deltas
    /// +1 loop, −2 half-edges, −1 edge.
    ///
    /// **Minting order** (D9, exact): the ring loop — the only entity
    /// minted. **Kill order** (D9, exact): `he1`, `he2`, the edge (each
    /// with its provenance entry), then the edge's curve iff orphaned
    /// ([`Body::remove_curve_if_orphaned`]).
    ///
    /// # Precondition check order
    ///
    /// `he1` resolves, `he2` resolves ([`EulerOpError::StaleKey`]); they
    /// are distinct halves of one edge which claims them both
    /// ([`EulerOpError::NotSameEdge`]); the edge resolves (`StaleKey`);
    /// same parent loop ([`EulerOpError::NotSameLoop`]); the loop
    /// resolves (`StaleKey`) and is a cycle
    /// ([`EulerOpError::LoopNotCycle`]); the loop's face resolves
    /// (`StaleKey`); the cycle walk from `he1` reaches `he2`
    /// ([`EulerOpError::LoopCycleBroken`]); both start vertices resolve
    /// (`StaleKey`); the two empty components (if both sides are empty)
    /// anchor at distinct vertices
    /// ([`EulerOpError::EmptyAnchorsCollide`]).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kemr(
        &mut self,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
    ) -> Result<KemrResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let he1_data = self.resolve_half_edge(he1)?;
        let he2_data = self.resolve_half_edge(he2)?;
        let edge = he1_data.edge;
        if he1 == he2 || he2_data.edge != edge {
            return Err(EulerOpError::NotSameEdge { he1, he2 });
        }
        let edge_data = self
            .get_edge(edge)
            .cloned()
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Edge(edge),
            })?;
        let claims_both = (edge_data.he_plus == he1 && edge_data.he_minus == he2)
            || (edge_data.he_plus == he2 && edge_data.he_minus == he1);
        if !claims_both {
            // Same `edge` field but the edge does not claim them: a
            // corrupt bijection — tier-1-invalid input.
            return Err(EulerOpError::NotSameEdge { he1, he2 });
        }
        let loop_key = he1_data.parent_loop;
        if he2_data.parent_loop != loop_key {
            return Err(EulerOpError::NotSameLoop { he1, he2 });
        }
        let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(loop_key),
        })?;
        if matches!(loop_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle { r#loop: loop_key });
        }
        let face_key = loop_data.face;
        if !self.faces.contains_key(face_key) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Face(face_key),
            });
        }
        // The full cycle from he1 (bounded, D9); its split at he2 yields
        // the two survivor sides.
        let cycle = self
            .loop_cycle(he1)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
        let position = cycle
            .iter()
            .position(|&he| he == he2)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
        // he1's side (strictly between he1 and he2): becomes the ring.
        let ring_side: Vec<HalfEdgeKey> = cycle[1..position].to_vec();
        // he2's side (strictly between he2 and he1): keeps the old loop.
        let old_side: Vec<HalfEdgeKey> = cycle[position + 1..].to_vec();
        let u = he1_data.start;
        let w = he2_data.start;
        for vertex in [u, w] {
            if !self.vertices.contains_key(vertex) {
                // The op rewrites both emanating anchors; a dangling
                // start vertex is tier-1-invalid input caught here.
                return Err(EulerOpError::StaleKey {
                    key: EntityId::Vertex(vertex),
                });
            }
        }
        if ring_side.is_empty() && old_side.is_empty() && u == w {
            return Err(EulerOpError::EmptyAnchorsCollide { vertex: u });
        }

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented above): the ring loop only.
        let provenance = Provenance::Kemr { he1, he2 };
        let ring_boundary = match ring_side.first() {
            Some(&first) => LoopBoundary::Cycle { first },
            None => LoopBoundary::Empty { vertex: w },
        };
        let ring = self.add_loop(
            Loop {
                boundary: ring_boundary,
                face: face_key,
            },
            provenance,
        );
        // Move he1's side into the ring and close its cycle.
        for &moved in &ring_side {
            if let Some(he) = self.get_half_edge_mut(moved) {
                he.parent_loop = ring;
            }
        }
        if let (Some(&first), Some(&last)) = (ring_side.first(), ring_side.last()) {
            // last = prev(he2), first = next(he1): closing the ring cycle
            // (self-link when the side has one member).
            self.link_half_edges(last, first);
        }
        // Close the old loop's cycle and re-anchor it (unconditional
        // rule: first := next(he2); Empty at u when its side is empty).
        if let (Some(&first), Some(&last)) = (old_side.first(), old_side.last()) {
            // last = prev(he1), first = next(he2).
            self.link_half_edges(last, first);
        }
        let old_boundary = match old_side.first() {
            Some(&first) => LoopBoundary::Cycle { first },
            None => LoopBoundary::Empty { vertex: u },
        };
        if let Some(l) = self.get_loop_mut(loop_key) {
            l.boundary = old_boundary;
        }
        if let Some(face) = self.get_face_mut(face_key) {
            face.rings.push(ring);
        }
        // Kills, with their provenance entries (kill order documented
        // above).
        self.half_edges.remove(he1);
        self.half_edge_provenance.remove(he1);
        self.half_edges.remove(he2);
        self.half_edge_provenance.remove(he2);
        self.edges.remove(edge);
        self.edge_provenance.remove(edge);
        let killed_curve = self
            .remove_curve_if_orphaned(edge_data.curve)
            .then_some(edge_data.curve);
        // Emanating (unconditional rule, module docs). When u == w the
        // second write wins — deterministic.
        let u_anchor = old_side.first().copied();
        let w_anchor = ring_side.first().copied();
        if let Some(vertex) = self.get_vertex_mut(u) {
            vertex.emanating = u_anchor;
        }
        if let Some(vertex) = self.get_vertex_mut(w) {
            vertex.emanating = w_anchor;
        }

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, 0, 1, -2, -1, 0), "kemr");
        Ok(KemrResult {
            ring,
            killed_edge: edge,
            killed_he_plus: edge_data.he_plus,
            killed_he_minus: edge_data.he_minus,
            killed_curve,
        })
    }

    /// MEKR — *make edge, kill ring*: join a ring of a face back into
    /// another loop of the same face with a new edge; the ring's loop
    /// key dies. The exact inverse of [`Body::kemr`].
    ///
    /// Site semantics (one variant per boundary-shape combination):
    /// [`MekrSite`]. Direction convention (module docs): `he_plus` runs
    /// **target anchor vertex → ring anchor vertex**; both halves land
    /// in the target loop, which is re-anchored at `he_plus`
    /// (unconditional). Emanating: `u → he_plus`, `w → he_minus`
    /// (unconditional, `mev`'s rule).
    ///
    /// Euler vector: `(v 0, e +1, f 0, h 0, r −1, s 0)` — arena deltas
    /// −1 loop, +2 half-edges, +1 edge.
    ///
    /// **Minting order** (D9, exact): curve (placeholder, anchored at
    /// the target anchor vertex's coordinates), edge, `he_plus`,
    /// `he_minus`. **Kill order**: the ring loop (with its provenance
    /// entry), after the splice.
    ///
    /// # Precondition check order
    ///
    /// Per site, in the documented order: anchors resolve
    /// ([`EulerOpError::StaleKey`]); the two loops are distinct
    /// ([`EulerOpError::SameLoop`]); each loop resolves (`StaleKey`) and
    /// has the site's required boundary shape
    /// ([`EulerOpError::LoopNotCycle`] /
    /// [`EulerOpError::LoopNotEmpty`]); both loops belong to one face
    /// ([`EulerOpError::NotSameFace`]) which resolves (`StaleKey`); the
    /// ring is not that face's outer loop
    /// ([`EulerOpError::RingIsOuter`]); cycle walks close
    /// ([`EulerOpError::LoopCycleBroken`]); splice/anchor keys resolve
    /// (`StaleKey` / [`EulerOpError::StaleGeometry`]); `BothEmpty`'s
    /// lone vertices are distinct
    /// ([`EulerOpError::EmptyAnchorsCollide`]).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mekr(&mut self, site: MekrSite) -> Result<MekrResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let created = match site {
            MekrSite::Cycles { target, ring } => self.mekr_cycles(site, target, ring),
            MekrSite::EmptyRing { target, ring } => self.mekr_empty_ring(site, target, ring),
            MekrSite::EmptyTarget { target, ring } => self.mekr_empty_target(site, target, ring),
            MekrSite::BothEmpty { target, ring } => self.mekr_both_empty(site, target, ring),
        }?;

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, 0, -1, 2, 1, 0), "mekr");
        Ok(created)
    }

    /// KFMRH — *kill face, make ring–hole*: the connected sum. `f2`'s
    /// outer loop becomes a ring of `f1`; `f2` dies.
    ///
    /// Semantics, the same-shell requirement (cross-shell deferred to
    /// M3), and the no-rings-on-`f2` rule: [module docs](self). `f2`'s
    /// outer may be `Empty` or `Cycle` — both legal. No half-edge,
    /// vertex, or edge is touched.
    ///
    /// Euler vector: `(v 0, e 0, f −1, h +1, r +1, s 0)` — arena delta
    /// −1 face (the "+1 ring" is the surviving loop's reclassification,
    /// not a mint; genus is derived, not stored).
    ///
    /// **Minting order**: nothing is minted (the loop survives with its
    /// D5 birth record — no provenance changes for survivors). **Kill
    /// order** (D9, exact): the face `f2` (with its provenance entry),
    /// then its surface iff orphaned
    /// ([`Body::remove_surface_if_orphaned`]).
    ///
    /// # Precondition check order
    ///
    /// `f1` resolves, `f2` resolves ([`EulerOpError::StaleKey`]); they
    /// are distinct ([`EulerOpError::SameFace`]); same shell
    /// ([`EulerOpError::CrossShell`]); the shell resolves (`StaleKey`);
    /// `f2` has no rings ([`EulerOpError::FaceHasRings`]); `f2`'s outer
    /// loop resolves (`StaleKey`).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kfmrh(&mut self, f1: FaceKey, f2: FaceKey) -> Result<KfmrhResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions. ----
        let f1_data = self.get_face(f1).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(f1),
        })?;
        let f1_shell = f1_data.shell;
        let f2_data = self
            .get_face(f2)
            .cloned()
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Face(f2),
            })?;
        if f1 == f2 {
            return Err(EulerOpError::SameFace { face: f1 });
        }
        if f2_data.shell != f1_shell {
            return Err(EulerOpError::CrossShell { f1, f2 });
        }
        if !self.shells.contains_key(f1_shell) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Shell(f1_shell),
            });
        }
        if !f2_data.rings.is_empty() {
            return Err(EulerOpError::FaceHasRings { face: f2 });
        }
        let ring = f2_data.outer;
        if !self.loops.contains_key(ring) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Loop(ring),
            });
        }

        // ---- Mutation (infallible from here on). ----
        // The surviving loop is repointed and demoted; nothing else at
        // the half-edge/vertex/edge level is touched.
        if let Some(l) = self.get_loop_mut(ring) {
            l.face = f1;
        }
        if let Some(face) = self.get_face_mut(f1) {
            face.rings.push(ring);
        }
        if let Some(shell) = self.get_shell_mut(f1_shell) {
            shell.faces.retain(|&face| face != f2);
        }
        self.faces.remove(f2);
        self.face_provenance.remove(f2);
        let killed_surface = self
            .remove_surface_if_orphaned(f2_data.surface)
            .then_some(f2_data.surface);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, -1, 0, 0, 0, 0), "kfmrh");
        Ok(KfmrhResult {
            ring,
            killed_face: f2,
            killed_surface,
        })
    }

    /// Reparents a ring from its face to `to_face` (same shell).
    /// **NOT an Euler operator** — see the [module docs](self): the
    /// Euler vector is unchanged, nothing is created or killed, and no
    /// provenance changes (D5 records are birth records; reparenting is
    /// not a re-birth). It is the non-Euler addendum to `mef`/`kfmrh`
    /// (GWB's `lringmv`), needed because `mef` deliberately does not
    /// reclassify the split face's rings and `kfmrh` requires `f2`
    /// ring-free.
    ///
    /// Pure reparenting: the ring leaves its face's `rings`, joins
    /// `to_face.rings` (appended — deterministic order), and its `face`
    /// back-pointer is repointed. Moving a ring to its own face is a
    /// documented no-op (`Ok(())`, body untouched — the rings order is
    /// NOT perturbed, keeping replay byte-stable).
    ///
    /// # Precondition check order
    ///
    /// The ring resolves ([`EulerOpError::StaleKey`]); its face
    /// resolves (`StaleKey`); it is not that face's outer loop
    /// ([`EulerOpError::RingIsOuter`]); `to_face` resolves (`StaleKey`);
    /// both faces lie in one shell ([`EulerOpError::CrossShell`]).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn ring_move(&mut self, ring: LoopKey, to_face: FaceKey) -> Result<(), EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions. ----
        let ring_data = self.get_loop(ring).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(ring),
        })?;
        let from_face = ring_data.face;
        let from_data = self.get_face(from_face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(from_face),
        })?;
        if from_data.outer == ring {
            return Err(EulerOpError::RingIsOuter { r#loop: ring });
        }
        let to_data = self.get_face(to_face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(to_face),
        })?;
        if to_data.shell != from_data.shell {
            return Err(EulerOpError::CrossShell {
                f1: from_face,
                f2: to_face,
            });
        }

        // ---- Mutation (infallible; no-op when the faces coincide). ----
        if from_face != to_face {
            if let Some(face) = self.get_face_mut(from_face) {
                face.rings.retain(|&l| l != ring);
            }
            if let Some(face) = self.get_face_mut(to_face) {
                face.rings.push(ring);
            }
            if let Some(l) = self.get_loop_mut(ring) {
                l.face = to_face;
            }
        }

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, (0, 0, 0, 0, 0, 0, 0), "ring_move");
        Ok(())
    }

    // ------------------------------------------------------------------
    // mekr implementation (one precondition block + surgery per site,
    // sharing the mint and finish helpers below)
    // ------------------------------------------------------------------

    /// [`MekrSite::Cycles`] — precondition block, then the merge
    /// surgery.
    fn mekr_cycles(
        &mut self,
        site: MekrSite,
        target: HalfEdgeKey,
        ring: HalfEdgeKey,
    ) -> Result<MekrResult, EulerOpError> {
        // ---- Preconditions. ----
        let target_data = self.resolve_half_edge(target)?;
        let ring_data = self.resolve_half_edge(ring)?;
        let target_loop = target_data.parent_loop;
        let ring_loop = ring_data.parent_loop;
        if target_loop == ring_loop {
            return Err(EulerOpError::SameLoop {
                r#loop: target_loop,
            });
        }
        let target_loop_data = self.get_loop(target_loop).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(target_loop),
        })?;
        if matches!(target_loop_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle {
                r#loop: target_loop,
            });
        }
        let face_key = target_loop_data.face;
        let ring_loop_data = self.get_loop(ring_loop).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(ring_loop),
        })?;
        if matches!(ring_loop_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle { r#loop: ring_loop });
        }
        if ring_loop_data.face != face_key {
            return Err(EulerOpError::NotSameFace {
                target: target_loop,
                ring: ring_loop,
            });
        }
        self.check_ring_not_outer(face_key, ring_loop)?;
        // The ring's full cycle (bounded, D9): reparented wholesale, and
        // its last member (= prev(ring)) is a splice point.
        let ring_members = self
            .loop_cycle(ring)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring_loop })?;
        let ring_last = ring_members
            .last()
            .copied()
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring_loop })?;
        let target_prev = target_data.prev;
        if !self.half_edges.contains_key(target_prev) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::HalfEdge(target_prev),
            });
        }
        let u = target_data.start;
        let w = ring_data.start;
        let anchor = self.check_anchors(u, w)?;

        // ---- Mutation (infallible from here on). ----
        let (curve, edge, he_plus, he_minus) = self.mekr_mint(site, u, w, target_loop, anchor);
        // Reparent the whole ring cycle into the target loop.
        for &moved in &ring_members {
            if let Some(he) = self.get_half_edge_mut(moved) {
                he.parent_loop = target_loop;
            }
        }
        // Splice (module docs diagram): he_plus → ring … prev(ring) →
        // he_minus → target … prev(target) → he_plus.
        self.link_half_edges(target_prev, he_plus);
        self.link_half_edges(he_plus, ring);
        self.link_half_edges(ring_last, he_minus);
        self.link_half_edges(he_minus, target);
        self.mekr_finish(target_loop, ring_loop, face_key, (u, w), (he_plus, he_minus));

        Ok(MekrResult {
            edge,
            he_plus,
            he_minus,
            curve,
            killed_ring: ring_loop,
        })
    }

    /// [`MekrSite::EmptyRing`] — precondition block, then the
    /// lone-vertex absorption surgery.
    fn mekr_empty_ring(
        &mut self,
        site: MekrSite,
        target: HalfEdgeKey,
        ring: LoopKey,
    ) -> Result<MekrResult, EulerOpError> {
        // ---- Preconditions. ----
        let target_data = self.resolve_half_edge(target)?;
        let target_loop = target_data.parent_loop;
        if target_loop == ring {
            return Err(EulerOpError::SameLoop { r#loop: ring });
        }
        let target_loop_data = self.get_loop(target_loop).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(target_loop),
        })?;
        if matches!(target_loop_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle {
                r#loop: target_loop,
            });
        }
        let face_key = target_loop_data.face;
        let ring_data = self.get_loop(ring).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(ring),
        })?;
        let LoopBoundary::Empty { vertex: w } = ring_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: ring });
        };
        if ring_data.face != face_key {
            return Err(EulerOpError::NotSameFace {
                target: target_loop,
                ring,
            });
        }
        self.check_ring_not_outer(face_key, ring)?;
        let target_prev = target_data.prev;
        if !self.half_edges.contains_key(target_prev) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::HalfEdge(target_prev),
            });
        }
        let u = target_data.start;
        let anchor = self.check_anchors(u, w)?;

        // ---- Mutation (infallible from here on). ----
        let (curve, edge, he_plus, he_minus) = self.mekr_mint(site, u, w, target_loop, anchor);
        // Splice: … prev(target) → he_plus → he_minus → target … (the
        // strut shape, re-created; inverse of kemr's ring-side-empty
        // case).
        self.link_half_edges(target_prev, he_plus);
        self.link_half_edges(he_plus, he_minus);
        self.link_half_edges(he_minus, target);
        self.mekr_finish(target_loop, ring, face_key, (u, w), (he_plus, he_minus));

        Ok(MekrResult {
            edge,
            he_plus,
            he_minus,
            curve,
            killed_ring: ring,
        })
    }

    /// [`MekrSite::EmptyTarget`] — precondition block, then the
    /// grow-the-lone-target surgery.
    fn mekr_empty_target(
        &mut self,
        site: MekrSite,
        target: LoopKey,
        ring: HalfEdgeKey,
    ) -> Result<MekrResult, EulerOpError> {
        // ---- Preconditions. ----
        let target_data = self.get_loop(target).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(target),
        })?;
        let LoopBoundary::Empty { vertex: u } = target_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: target });
        };
        let face_key = target_data.face;
        let ring_data = self.resolve_half_edge(ring)?;
        let ring_loop = ring_data.parent_loop;
        if ring_loop == target {
            return Err(EulerOpError::SameLoop { r#loop: target });
        }
        let ring_loop_data = self.get_loop(ring_loop).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(ring_loop),
        })?;
        if matches!(ring_loop_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle { r#loop: ring_loop });
        }
        if ring_loop_data.face != face_key {
            return Err(EulerOpError::NotSameFace {
                target,
                ring: ring_loop,
            });
        }
        self.check_ring_not_outer(face_key, ring_loop)?;
        let ring_members = self
            .loop_cycle(ring)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring_loop })?;
        let ring_last = ring_members
            .last()
            .copied()
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring_loop })?;
        let w = ring_data.start;
        let anchor = self.check_anchors(u, w)?;

        // ---- Mutation (infallible from here on). ----
        let (curve, edge, he_plus, he_minus) = self.mekr_mint(site, u, w, target, anchor);
        for &moved in &ring_members {
            if let Some(he) = self.get_half_edge_mut(moved) {
                he.parent_loop = target;
            }
        }
        // Splice: he_plus → ring … prev(ring) → he_minus → he_plus (the
        // target contributes no half-edges; its Empty boundary grows to
        // this cycle — inverse of kemr's old-side-empty case).
        self.link_half_edges(he_plus, ring);
        self.link_half_edges(ring_last, he_minus);
        self.link_half_edges(he_minus, he_plus);
        self.mekr_finish(target, ring_loop, face_key, (u, w), (he_plus, he_minus));

        Ok(MekrResult {
            edge,
            he_plus,
            he_minus,
            curve,
            killed_ring: ring_loop,
        })
    }

    /// [`MekrSite::BothEmpty`] — precondition block, then the
    /// two-lone-vertices-to-segment surgery.
    fn mekr_both_empty(
        &mut self,
        site: MekrSite,
        target: LoopKey,
        ring: LoopKey,
    ) -> Result<MekrResult, EulerOpError> {
        // ---- Preconditions. ----
        let target_data = self.get_loop(target).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(target),
        })?;
        let LoopBoundary::Empty { vertex: u } = target_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: target });
        };
        let face_key = target_data.face;
        if target == ring {
            return Err(EulerOpError::SameLoop { r#loop: target });
        }
        let ring_data = self.get_loop(ring).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(ring),
        })?;
        let LoopBoundary::Empty { vertex: w } = ring_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: ring });
        };
        if ring_data.face != face_key {
            return Err(EulerOpError::NotSameFace { target, ring });
        }
        self.check_ring_not_outer(face_key, ring)?;
        if u == w {
            // Two empty loops holding one lone vertex — tier-1-invalid
            // input (see the module docs), checked defensively.
            return Err(EulerOpError::EmptyAnchorsCollide { vertex: u });
        }
        let anchor = self.check_anchors(u, w)?;

        // ---- Mutation (infallible from here on). ----
        let (curve, edge, he_plus, he_minus) = self.mekr_mint(site, u, w, target, anchor);
        // The two halves form the whole cycle: u → w → u (the segment
        // loop — inverse of kemr's both-empty case).
        self.link_half_edges(he_plus, he_minus);
        self.link_half_edges(he_minus, he_plus);
        self.mekr_finish(target, ring, face_key, (u, w), (he_plus, he_minus));

        Ok(MekrResult {
            edge,
            he_plus,
            he_minus,
            curve,
            killed_ring: ring,
        })
    }

    // ------------------------------------------------------------------
    // Shared mekr internals
    // ------------------------------------------------------------------

    /// The shared face-side precondition: the loop named as the ring
    /// must not be `face`'s outer loop. (`face` was resolved by the
    /// caller; a stale face key here is unreachable.)
    fn check_ring_not_outer(&self, face: FaceKey, ring: LoopKey) -> Result<(), EulerOpError> {
        let face_data = self.get_face(face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        if face_data.outer == ring {
            return Err(EulerOpError::RingIsOuter { r#loop: ring });
        }
        Ok(())
    }

    /// The shared vertex-side preconditions: both anchor vertices
    /// resolve (their `emanating` is rewritten) and `u`'s point resolves
    /// (the placeholder-curve anchor). Returns the anchor coordinates.
    fn check_anchors(&self, u: VertexKey, w: VertexKey) -> Result<Point3<T>, EulerOpError> {
        let anchor = self.resolve_vertex_point(u)?;
        if !self.vertices.contains_key(w) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Vertex(w),
            });
        }
        Ok(anchor)
    }

    /// `mekr`'s mint phase (documented minting order: curve, edge,
    /// `he_plus`, `he_minus`). Both halves land in the target loop;
    /// `next`/`prev` are provisional for the caller's splice.
    fn mekr_mint(
        &mut self,
        site: MekrSite,
        u: VertexKey,
        w: VertexKey,
        target_loop: LoopKey,
        anchor: Point3<T>,
    ) -> (CurveKey, EdgeKey, HalfEdgeKey, HalfEdgeKey) {
        let provenance = Provenance::Mekr { site };
        let curve = self.add_curve(CurveGeom::Placeholder { anchor });
        let edge = self.mint_edge(curve, &provenance);
        let (he_plus, he_minus) =
            self.mint_halves(edge, (u, target_loop), (w, target_loop), &provenance);
        (curve, edge, he_plus, he_minus)
    }

    /// `mekr`'s common tail: re-anchor the target loop at `he_plus`
    /// (unconditional), drop the ring from the face's ring list, kill
    /// the ring loop with its provenance entry, and apply the emanating
    /// rule (`u → he_plus`, `w → he_minus`, unconditional). `anchors` is
    /// `(u, w)`, `halves` is `(he_plus, he_minus)`.
    fn mekr_finish(
        &mut self,
        target_loop: LoopKey,
        ring_loop: LoopKey,
        face: FaceKey,
        anchors: (VertexKey, VertexKey),
        halves: (HalfEdgeKey, HalfEdgeKey),
    ) {
        let (u, w) = anchors;
        let (he_plus, he_minus) = halves;
        if let Some(l) = self.get_loop_mut(target_loop) {
            l.boundary = LoopBoundary::Cycle { first: he_plus };
        }
        if let Some(face_data) = self.get_face_mut(face) {
            face_data.rings.retain(|&l| l != ring_loop);
        }
        self.loops.remove(ring_loop);
        self.loop_provenance.remove(ring_loop);
        if let Some(vertex) = self.get_vertex_mut(u) {
            vertex.emanating = Some(he_plus);
        }
        if let Some(vertex) = self.get_vertex_mut(w) {
            vertex.emanating = Some(he_minus);
        }
    }
}

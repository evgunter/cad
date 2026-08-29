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
//!   record outliving its entity is a leak — the validator's
//!   bidirectional provenance pass makes leaks loud). Killed keys
//!   returned in the result structs are
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
//! a leak (pinned by test). Convergence with the pair-free history is
//! **per-arena**, because a balanced kemr∘mekr pair is slot-balanced
//! only per-arena: `mekr` immediately re-consumes the half-edge, edge,
//! and curve slots `kemr` freed, but the pair's net effect on the LOOP
//! arena is one freed slot (`kemr`'s ring, freed again by `mekr` and
//! re-consumed by nothing in the pair). So half-edge/edge/curve minting
//! converges right after the pair, while the loop arena converges one
//! loop-mint later — the first loop-minting op (e.g. `mef`) lands its
//! loop in the recycled slot with a bumped generation, and everything
//! from the second loop-mint on agrees (pinned by test). An UNBALANCED
//! kill history (a `kemr` with no inverse) offsets the killed arenas'
//! allocation cursors permanently: arenas the kill never touched stay
//! aligned with the kill-free history forever, killed arenas never
//! re-align.
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
//! (l)); cross-shell within one solid ⇒ **shell fusion** (M3 PR 1),
//! re-homing `f2`'s shell's surviving faces into `f1`'s shell. Across
//! **solids** it stays a typed error ([`EulerOpError::CrossSolid`]):
//! combining bodies is the boolean pipeline's combine step, not an
//! Euler surgery. `f2` must carry **no
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
//! use geom_core::{Point3, Tol};
//! use topo::{Body, LoopBoundary, MekrSite, MevSite};
//!
//! # fn run() -> Result<(), topo::EulerOpError> {
//! let tol = Tol::witness();
//! let mut body = Body::<f64>::new();
//! let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0))?;
//! let seg = body.mev_line(
//!     MevSite::Lone { r#loop: seed.r#loop },
//!     Point3::new(1.0, 0.0, 0.0),
//!     tol,
//! )?;
//! // A strut, then kill it: the far vertex is stranded as an EMPTY RING
//! // of the face — Mäntylä §9.3 step (g), the hole-planting state.
//! let strut = body.mev_line(
//!     MevSite::Fan { he1: seg.he_minus, he2: seg.he_minus },
//!     Point3::new(2.0, 0.0, 0.0),
//!     tol,
//! )?;
//! let kill = body.kemr(strut.he_plus, strut.he_minus)?;
//! assert_eq!(
//!     body.get_loop(kill.ring).unwrap().boundary,
//!     LoopBoundary::Empty { vertex: strut.vertex },
//! );
//! // mekr absorbs the lone-vertex ring back with a new edge.
//! let restore = body.mekr_chord(MekrSite::EmptyRing {
//!     target: seg.he_minus,
//!     ring: kill.ring,
//! }, tol)?;
//! assert_eq!(topo::validate(&body), Ok(()));
//! assert!(body.get_loop(kill.ring).is_none()); // the ring key died
//! assert_eq!(body.half_edge_end(restore.he_plus), Some(strut.vertex));
//! # Ok(()) }
//! # run().unwrap();
//! ```

use geom_brep::EdgeCurveSpec;
use geom_core::{Decide, Point3};

use crate::body::Body;
use crate::entity::{
    EdgeKey, EntityId, FaceKey, HalfEdgeKey, Loop, LoopBoundary, LoopKey, ShellKey, VertexKey,
};
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::EulerOpError;
use crate::geometry::{CurveKey, SurfaceKey};
use crate::live::{Live, require_key};
use crate::provenance::Provenance;
use geom_core::Tol;

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
    /// The edge's certified curve (the attachment-gated `EdgeCurve`
    /// built from the given spec — M2 geometry policy,
    /// `crate::euler` module docs).
    pub curve: CurveKey,
    /// The killed ring loop (dead key — no longer resolves).
    pub killed_ring: LoopKey,
}

/// The outcome of one [`Body::kfmrh`] call: nothing created (the ring is
/// the surviving demoted loop), one face — and, in the cross-shell form,
/// one shell — dead, plus possibly the face's orphaned surface.
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
    /// The killed shell (dead key) in the **cross-shell** form (M3
    /// PR 1): `f2`'s shell, whose surviving faces were re-homed into
    /// `f1`'s shell before it died. `None` in the same-shell form.
    pub killed_shell: Option<ShellKey>,
}

impl<T: Decide> Body<T> {
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
    pub fn kemr(&mut self, he1: HalfEdgeKey, he2: HalfEdgeKey) -> Result<KemrResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let he1_data = self.resolve_half_edge(he1)?;
        let he2_data = self.resolve_half_edge(he2)?;
        let edge = he1_data.edge;
        if he1 == he2 || he2_data.edge != edge {
            return Err(EulerOpError::NotSameEdge { he1, he2 });
        }
        let edge_data = self.get_edge(edge).cloned().ok_or(EulerOpError::StaleKey {
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
        require_key(&self.faces, face_key, EntityId::Face)?;
        // The full cycle from he1 (bounded, D9); its split at he2 yields
        // the two survivor sides.
        let cycle = self
            .loop_cycle_live(he1)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
        let position = cycle
            .iter()
            .position(|member| member.key() == he2)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
        // he1's side (strictly between he1 and he2): becomes the ring.
        // The walk resolved every member, so each side's ends arrive at
        // the splice below already proven.
        let ring_side: Vec<Live> = cycle[1..position].to_vec();
        // he2's side (strictly between he2 and he1): keeps the old loop.
        let old_side: Vec<Live> = cycle[position + 1..].to_vec();
        let u = he1_data.start;
        let w = he2_data.start;
        // The op rewrites both emanating anchors; a dangling start
        // vertex is tier-1-invalid input caught here.
        for vertex in [u, w] {
            require_key(&self.vertices, vertex, EntityId::Vertex)?;
        }
        if ring_side.is_empty() && old_side.is_empty() && u == w {
            return Err(EulerOpError::EmptyAnchorsCollide { vertex: u });
        }

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented above): the ring loop only.
        let provenance = Provenance::Kemr { he1, he2 };
        let ring_boundary = match ring_side.first() {
            Some(&first) => LoopBoundary::Cycle { first: first.key() },
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
            let Some(he) = self.get_half_edge_mut(moved.key()) else {
                unreachable!(
                    "kemr: the ring side's members were resolved by the plan phase's bounded walk"
                )
            };
            he.parent_loop = ring;
        }
        if let (Some(&first), Some(&last)) = (ring_side.first(), ring_side.last()) {
            // last = prev(he2), first = next(he1): closing the ring cycle.
            // When the side has ONE member this is a self-link (a
            // one-half-edge loop). That configuration needs a self-loop
            // half flanked by the killed halves on both sides — believed
            // unreachable through M1 operator sequences and verified by
            // derivation only (same for the old side below).
            self.link_half_edges(last, first);
        }
        // Close the old loop's cycle and re-anchor it (unconditional
        // rule: first := next(he2); Empty at u when its side is empty).
        if let (Some(&first), Some(&last)) = (old_side.first(), old_side.last()) {
            // last = prev(he1), first = next(he2).
            self.link_half_edges(last, first);
        }
        let old_boundary = match old_side.first() {
            Some(&first) => LoopBoundary::Cycle { first: first.key() },
            None => LoopBoundary::Empty { vertex: u },
        };
        let Some(l) = self.get_loop_mut(loop_key) else {
            unreachable!("kemr: the loop resolved in the plan phase")
        };
        l.boundary = old_boundary;
        let Some(face) = self.get_face_mut(face_key) else {
            unreachable!("kemr: the face resolved in the plan phase")
        };
        face.rings.push(ring);
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
        let u_anchor = old_side.first().map(|&member| member.key());
        let w_anchor = ring_side.first().map(|&member| member.key());
        let Some(vertex) = self.get_vertex_mut(u) else {
            unreachable!("kemr: `u` resolved in the plan phase")
        };
        vertex.emanating = u_anchor;
        let Some(vertex) = self.get_vertex_mut(w) else {
            unreachable!("kemr: `w` resolved in the plan phase")
        };
        vertex.emanating = w_anchor;

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                loops: 1,
                half_edges: -2,
                edges: -1,
                ..ArenaDelta::ZERO
            },
            "kemr",
        );
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
    /// The listing below is **`Cycles`-canonical**; the loop-key-
    /// addressed sites (`EmptyTarget`, `BothEmpty`) necessarily resolve
    /// the target loop (and check its boundary shape) *before* the
    /// distinctness check. Each site's own order is fixed and
    /// deterministic (the first failure wins).
    ///
    /// Anchors resolve ([`EulerOpError::StaleKey`]); the two loops are
    /// distinct ([`EulerOpError::SameLoop`]); each loop resolves
    /// (`StaleKey`) and has the site's required boundary shape
    /// ([`EulerOpError::LoopNotCycle`] /
    /// [`EulerOpError::LoopNotEmpty`]); both loops belong to one face
    /// ([`EulerOpError::NotSameFace`]) which resolves (`StaleKey`); the
    /// ring is not that face's outer loop
    /// ([`EulerOpError::RingIsOuter`]); cycle walks close
    /// ([`EulerOpError::LoopCycleBroken`]); splice/anchor keys resolve
    /// (`StaleKey` / [`EulerOpError::StaleGeometry`]); `BothEmpty`'s
    /// lone vertices are distinct
    /// ([`EulerOpError::EmptyAnchorsCollide`]). Then the geometry
    /// gate: `curve` certifies against the anchors' points, u → w in
    /// the `he_plus` forward order
    /// ([`EulerOpError::Certification`]; `crate::euler` module docs,
    /// M2 geometry policy). Chord sugar: [`Body::mekr_chord`].
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mekr(
        &mut self,
        site: MekrSite,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MekrResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let created = match site {
            MekrSite::Cycles { target, ring } => self.mekr_cycles(site, target, ring, curve, tol),
            MekrSite::EmptyRing { target, ring } => {
                self.mekr_empty_ring(site, target, ring, curve, tol)
            }
            MekrSite::EmptyTarget { target, ring } => {
                self.mekr_empty_target(site, target, ring, curve, tol)
            }
            MekrSite::BothEmpty { target, ring } => {
                self.mekr_both_empty(site, target, ring, curve, tol)
            }
        }?;

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                loops: -1,
                half_edges: 2,
                edges: 1,
                ..ArenaDelta::ZERO
            },
            "mekr",
        );
        Ok(created)
    }

    /// [`Body::mekr`] with derived scaffolding geometry — the
    /// polyhedral/migration sugar (see `crate::euler`'s geometry
    /// policy): the chord line between the two anchor vertices' points
    /// (`EdgeCurveSpec::line_between`), or the canonical scaffolding
    /// circle when both anchors are one vertex (structural key
    /// equality — `EdgeCurveSpec::self_loop_circle_at`).
    ///
    /// # Errors
    ///
    /// As [`Body::mekr`].
    pub fn mekr_chord(&mut self, site: MekrSite, tol: Tol) -> Result<MekrResult, EulerOpError> {
        let anchor_of = |body: &Self, r#loop: LoopKey| -> Result<VertexKey, EulerOpError> {
            let loop_data = body.get_loop(r#loop).ok_or(EulerOpError::StaleKey {
                key: EntityId::Loop(r#loop),
            })?;
            match loop_data.boundary {
                LoopBoundary::Empty { vertex } => Ok(vertex),
                LoopBoundary::Cycle { .. } => Err(EulerOpError::LoopNotEmpty { r#loop }),
            }
        };
        let (u, w) = match site {
            MekrSite::Cycles { target, ring } => (
                self.resolve_half_edge(target)?.start,
                self.resolve_half_edge(ring)?.start,
            ),
            MekrSite::EmptyRing { target, ring } => (
                self.resolve_half_edge(target)?.start,
                anchor_of(self, ring)?,
            ),
            MekrSite::EmptyTarget { target, ring } => (
                anchor_of(self, target)?,
                self.resolve_half_edge(ring)?.start,
            ),
            MekrSite::BothEmpty { target, ring } => {
                (anchor_of(self, target)?, anchor_of(self, ring)?)
            }
        };
        let p_u = self.resolve_vertex_point(u)?;
        let spec = if u == w {
            EdgeCurveSpec::self_loop_circle_at(p_u)
        } else {
            let p_w = self.resolve_vertex_point(w)?;
            EdgeCurveSpec::line_between(p_u, p_w)
        };
        self.mekr(site, spec, tol)
    }

    /// KFMRH — *kill face, make ring–hole*: the connected sum. `f2`'s
    /// outer loop becomes a ring of `f1`; `f2` dies.
    ///
    /// Two forms, selected by where the faces live (M3 PR 1 lifted the
    /// M1 cross-shell deferral — the ch. 12 glue / ch. 15 seam-zip call
    /// site now exists):
    ///
    /// - **Same shell** (the M1 form): genus surgery — a handle (or a
    ///   component merge within the shell, the PR 4 transient).
    /// - **Cross-shell, same solid** (M3): shell **fusion** — `f2`'s
    ///   shell's surviving faces are re-homed into `f1`'s shell and
    ///   `f2`'s shell dies. Per-component E–P reading: the two
    ///   incidence components join through the demoted ring (connected
    ///   sum — genera add, χ₁+χ₂−2). Serves ch. 15 `setopfinish`'s
    ///   seam zip and ch. 12 glue (M3 PR 5). Fusing across **solids**
    ///   stays a typed error ([`EulerOpError::CrossSolid`]): combining
    ///   bodies is the boolean pipeline's combine step, not an Euler
    ///   surgery.
    ///
    /// The no-rings-on-`f2` rule: [module docs](self). `f2`'s outer may
    /// be `Empty` or `Cycle` — both legal. No half-edge, vertex, or
    /// edge is touched.
    ///
    /// Euler vector: `(v 0, e 0, f −1, h +1, r +1, s 0)` — arena delta
    /// −1 face (the "+1 ring" is the surviving loop's reclassification,
    /// not a mint; genus is derived, not stored); the cross-shell form
    /// is additionally `s −1` at the *shell* arena (the solid count is
    /// unchanged — GWB's §9.2.4 "KFSMR" reading).
    ///
    /// **Minting order**: nothing is minted (the loop survives with its
    /// D5 birth record — no provenance changes for survivors; re-homed
    /// faces keep their birth records — re-homing is not a re-birth).
    /// **Kill order** (D9, exact): cross-shell form first re-homes
    /// `f2`'s shell's surviving faces (appended to `f1`'s shell's list
    /// in their surviving order) and kills the shell (with its
    /// provenance); then, both forms, the face `f2` (with its
    /// provenance and any F9 null-face record), then `f2`'s surface
    /// iff orphaned ([`Body::remove_surface_if_orphaned`]).
    ///
    /// # Precondition check order
    ///
    /// `f1` resolves, `f2` resolves ([`EulerOpError::StaleKey`]); they
    /// are distinct ([`EulerOpError::SameFace`]); both shells resolve
    /// (`StaleKey`); cross-shell only: one solid
    /// ([`EulerOpError::CrossSolid`]); `f2` has no rings
    /// ([`EulerOpError::FaceHasRings`]); `f2`'s outer loop resolves
    /// (`StaleKey`); cross-shell only: every surviving face of `f2`'s
    /// shell and the shared solid resolve (`StaleKey`) — the fusion
    /// writes through both.
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
        let f2_data = self.get_face(f2).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(f2),
        })?;
        if f1 == f2 {
            return Err(EulerOpError::SameFace { face: f1 });
        }
        let f2_shell = f2_data.shell;
        let cross_shell = f2_shell != f1_shell;
        let s1_solid = self
            .get_shell(f1_shell)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Shell(f1_shell),
            })?
            .solid;
        let s2_data = self
            .get_shell(f2_shell)
            .cloned()
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Shell(f2_shell),
            })?;
        if cross_shell && s2_data.solid != s1_solid {
            return Err(EulerOpError::CrossSolid { f1, f2 });
        }
        if !f2_data.rings.is_empty() {
            return Err(EulerOpError::FaceHasRings { face: f2 });
        }
        let ring = f2_data.outer;
        require_key(&self.loops, ring, EntityId::Loop)?;
        if cross_shell {
            // The fusion re-homes f2's shell's surviving faces and
            // rewrites the shared solid's shell list; both are keys read
            // out of the body rather than arguments, so prove them live
            // here — the mutation below cannot fail midway (atomicity).
            for &face in &s2_data.faces {
                if face != f2 {
                    require_key(&self.faces, face, EntityId::Face)?;
                }
            }
            require_key(&self.solids, s2_data.solid, EntityId::Solid)?;
        }

        // ---- Mutation (infallible from here on). ----
        // The surviving loop is repointed and demoted; nothing else at
        // the half-edge/vertex/edge level is touched.
        let Some(l) = self.get_loop_mut(ring) else {
            unreachable!("kfmrh: the ring resolved in the plan phase")
        };
        l.face = f1;
        let Some(face) = self.get_face_mut(f1) else {
            unreachable!("kfmrh: `f1` resolved in the plan phase")
        };
        face.rings.push(ring);
        let killed_shell = if cross_shell {
            // Shell fusion: f2's surviving faces re-home into f1's
            // shell — appended in their surviving f2-shell list order
            // (deterministic, D9) — then f2's shell dies.
            let moved: Vec<FaceKey> = s2_data.faces.iter().copied().filter(|&f| f != f2).collect();
            for &face in &moved {
                let Some(face_data) = self.get_face_mut(face) else {
                    unreachable!("kfmrh: f2's shell's faces were resolved in the plan phase")
                };
                face_data.shell = f1_shell;
            }
            let Some(shell) = self.get_shell_mut(f1_shell) else {
                unreachable!("kfmrh: f1's shell resolved in the plan phase")
            };
            shell.faces.extend(moved);
            let Some(solid) = self.get_solid_mut(s2_data.solid) else {
                unreachable!("kfmrh: the shared solid resolved in the plan phase")
            };
            solid.shells.retain(|&s| s != f2_shell);
            self.shells.remove(f2_shell);
            self.shell_provenance.remove(f2_shell);
            Some(f2_shell)
        } else {
            let Some(shell) = self.get_shell_mut(f1_shell) else {
                unreachable!("kfmrh: f1's shell resolved in the plan phase")
            };
            shell.faces.retain(|&face| face != f2);
            None
        };
        self.faces.remove(f2);
        self.face_provenance.remove(f2);
        // Null-face record hygiene (M3 PR 1): a record never outlives
        // its face (crate::null).
        self.null_faces.remove(f2);
        let killed_surface = self
            .remove_surface_if_orphaned(f2_data.surface)
            .then_some(f2_data.surface);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            if killed_shell.is_some() {
                ArenaDelta {
                    shells: -1,
                    faces: -1,
                    ..ArenaDelta::ZERO
                }
            } else {
                ArenaDelta {
                    faces: -1,
                    ..ArenaDelta::ZERO
                }
            },
            "kfmrh",
        );
        Ok(KfmrhResult {
            ring,
            killed_face: f2,
            killed_surface,
            killed_shell,
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
    /// **This op is also ch. 14's `laringmv`** (M3 PR 1 — ring
    /// re-homing after splits change containment; serves ch. 14
    /// `splitconnect` joining, M3 PR 3). Division of labor, ratified
    /// with the M3 plan: the *containment decision* — which face a
    /// ring now geometrically belongs to — is the **caller's**, made
    /// in the splitting/boolean pipeline through PR 2's trilean
    /// point-in-loop classification machinery. `ring_move` takes the
    /// target face explicitly and validates only structural legality
    /// (the ring's current owner, target in the same shell); it never
    /// runs a geometric containment check itself, so a wrong target is
    /// the caller's bug, surfaced by the designation-sensitive
    /// consumers (tier-3 region checks, mass properties), not here.
    ///
    /// Pure reparenting: the ring leaves its face's `rings`, joins
    /// `to_face.rings` (appended — deterministic order), and its `face`
    /// back-pointer is repointed. Moving a ring to its own face is a
    /// documented no-op (`Ok(())`, body untouched — the rings order is
    /// NOT perturbed, keeping replay byte-stable).
    ///
    /// # Tier-1 preservation (the demotion claim's least obvious case)
    ///
    /// `ring_move` re-glues the per-shell component partition (a face
    /// glues all its loops — validator pass 11), so unlike the Euler
    /// operators its tier-1 preservation is not a per-surgery
    /// Euler-vector fact. It holds anyway, by the separating-curve
    /// argument (independently derived by the PR 5 review): validator
    /// passes 3/4/6 force every component to be a closed oriented
    /// surface, and on a genus-0 component every ring cycle is a
    /// separating (Jordan) curve — so a cross-component move
    /// re-partitions the complex into pieces that are again closed
    /// surfaces with `χ = 2(1 − g)`, `g ≥ 0`; a NON-separating ring
    /// forces its component's genus ≥ 1 before the move, and a move can
    /// merge at most two components, so no move manufactures odd χ or
    /// negative genus. Exercised continuously by the seqgen fuzz lane
    /// (`ring_move` is a candidate mutator) and by the promoted review
    /// sweeps (`tests/review_m1_pr5.rs`: every `ring_move` to depth 2
    /// over adversarial fixtures, including the non-separating-ring
    /// pillow-torus).
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
            let Some(face) = self.get_face_mut(from_face) else {
                unreachable!("ring_move: the source face resolved in the plan phase")
            };
            face.rings.retain(|&l| l != ring);
            let Some(face) = self.get_face_mut(to_face) else {
                unreachable!("ring_move: the destination face resolved in the plan phase")
            };
            face.rings.push(ring);
            let Some(l) = self.get_loop_mut(ring) else {
                unreachable!("ring_move: the ring resolved in the plan phase")
            };
            l.face = to_face;
        }

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, ArenaDelta::ZERO, "ring_move");
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
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MekrResult, EulerOpError> {
        // ---- Preconditions. ----
        let (target_live, target_data) = self.resolve_half_edge_live(target)?;
        let (ring_live, ring_data) = self.resolve_half_edge_live(ring)?;
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
            .loop_cycle_live(ring)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring_loop })?;
        let ring_last = ring_members
            .last()
            .copied()
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring_loop })?;
        let target_prev = self.require_live(target_data.prev)?;
        let u = target_data.start;
        let w = ring_data.start;
        let (p_u, p_w) = self.check_anchors(u, w)?;
        // ---- Geometry gate (still no mutation): certify u → w (the
        // he_plus forward order).
        let certified = self.certify_edge_spec(curve, p_u, p_w, tol)?;

        // ---- Mutation (infallible from here on). ----
        let (curve, edge, he_plus, he_minus) = self.mekr_mint(site, u, w, target_loop, certified);
        // Reparent the whole ring cycle into the target loop.
        for &moved in &ring_members {
            let Some(he) = self.get_half_edge_mut(moved.key()) else {
                unreachable!(
                    "mekr chords: the ring's members were resolved by the plan phase's bounded walk"
                )
            };
            he.parent_loop = target_loop;
        }
        // Splice (module docs diagram): he_plus → ring … prev(ring) →
        // he_minus → target … prev(target) → he_plus.
        self.link_half_edges(target_prev, he_plus);
        self.link_half_edges(he_plus, ring_live);
        self.link_half_edges(ring_last, he_minus);
        self.link_half_edges(he_minus, target_live);
        // The splice is done; past it the halves are ordinary keys.
        let (he_plus, he_minus) = (he_plus.key(), he_minus.key());
        self.mekr_finish(
            target_loop,
            ring_loop,
            face_key,
            (u, w),
            (he_plus, he_minus),
        );

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
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MekrResult, EulerOpError> {
        // ---- Preconditions. ----
        let (target_live, target_data) = self.resolve_half_edge_live(target)?;
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
        let target_prev = self.require_live(target_data.prev)?;
        let u = target_data.start;
        let (p_u, p_w) = self.check_anchors(u, w)?;
        // ---- Geometry gate (still no mutation): certify u → w (the
        // he_plus forward order).
        let certified = self.certify_edge_spec(curve, p_u, p_w, tol)?;

        // ---- Mutation (infallible from here on). ----
        let (curve, edge, he_plus, he_minus) = self.mekr_mint(site, u, w, target_loop, certified);
        // Splice: … prev(target) → he_plus → he_minus → target … (the
        // strut shape, re-created; inverse of kemr's ring-side-empty
        // case).
        self.link_half_edges(target_prev, he_plus);
        self.link_half_edges(he_plus, he_minus);
        self.link_half_edges(he_minus, target_live);
        // The splice is done; past it the halves are ordinary keys.
        let (he_plus, he_minus) = (he_plus.key(), he_minus.key());
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
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MekrResult, EulerOpError> {
        // ---- Preconditions. ----
        let target_data = self.get_loop(target).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(target),
        })?;
        let LoopBoundary::Empty { vertex: u } = target_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: target });
        };
        let face_key = target_data.face;
        let (ring_live, ring_data) = self.resolve_half_edge_live(ring)?;
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
            .loop_cycle_live(ring)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring_loop })?;
        let ring_last = ring_members
            .last()
            .copied()
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: ring_loop })?;
        let w = ring_data.start;
        let (p_u, p_w) = self.check_anchors(u, w)?;
        // ---- Geometry gate (still no mutation): certify u → w (the
        // he_plus forward order).
        let certified = self.certify_edge_spec(curve, p_u, p_w, tol)?;

        // ---- Mutation (infallible from here on). ----
        let (curve, edge, he_plus, he_minus) = self.mekr_mint(site, u, w, target, certified);
        for &moved in &ring_members {
            let Some(he) = self.get_half_edge_mut(moved.key()) else {
                unreachable!(
                    "mekr empty-target: the ring's members were resolved by the plan phase's bounded walk"
                )
            };
            he.parent_loop = target;
        }
        // Splice: he_plus → ring … prev(ring) → he_minus → he_plus (the
        // target contributes no half-edges; its Empty boundary grows to
        // this cycle — inverse of kemr's old-side-empty case).
        self.link_half_edges(he_plus, ring_live);
        self.link_half_edges(ring_last, he_minus);
        self.link_half_edges(he_minus, he_plus);
        // The splice is done; past it the halves are ordinary keys.
        let (he_plus, he_minus) = (he_plus.key(), he_minus.key());
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
        curve: EdgeCurveSpec<T>,
        tol: Tol,
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
        let (p_u, p_w) = self.check_anchors(u, w)?;
        // ---- Geometry gate (still no mutation): certify u → w (the
        // he_plus forward order).
        let certified = self.certify_edge_spec(curve, p_u, p_w, tol)?;

        // ---- Mutation (infallible from here on). ----
        let (curve, edge, he_plus, he_minus) = self.mekr_mint(site, u, w, target, certified);
        // The two halves form the whole cycle: u → w → u (the segment
        // loop — inverse of kemr's both-empty case).
        self.link_half_edges(he_plus, he_minus);
        self.link_half_edges(he_minus, he_plus);
        // The splice is done; past it the halves are ordinary keys.
        let (he_plus, he_minus) = (he_plus.key(), he_minus.key());
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

    /// The shared vertex-side preconditions: both anchor vertices and
    /// their points resolve (`emanating` is rewritten; the points are
    /// the certification gate's endpoints). Returns `(u's point, w's
    /// point)`.
    ///
    /// The [`EulerOpError::StaleGeometry`] arm is dead-but-defensive:
    /// point removal exists (PR 4's `kev`/`kvfs` reap orphaned points),
    /// but every reaped point's vertex dies with it, so a live vertex
    /// with a stale point is still unreachable through the public API
    /// without in-crate corruption (tier 1 would report it as
    /// `DanglingGeometry`). Kept because the op must stay sound
    /// standalone.
    fn check_anchors(
        &self,
        u: VertexKey,
        w: VertexKey,
    ) -> Result<(Point3<T>, Point3<T>), EulerOpError> {
        let p_u = self.resolve_vertex_point(u)?;
        let p_w = self.resolve_vertex_point(w)?;
        Ok((p_u, p_w))
    }

    /// `mekr`'s mint phase (documented minting order: curve — the
    /// certified `EdgeCurve` from the attachment gate — edge,
    /// `he_plus`, `he_minus`). Both halves land in the target loop;
    /// `next`/`prev` are provisional for the caller's splice.
    fn mekr_mint(
        &mut self,
        site: MekrSite,
        u: VertexKey,
        w: VertexKey,
        target_loop: LoopKey,
        certified: geom_brep::EdgeCurve<T>,
    ) -> (CurveKey, EdgeKey, Live, Live) {
        let provenance = Provenance::Mekr { site };
        let curve = self.add_curve(certified);
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
        let Some(l) = self.get_loop_mut(target_loop) else {
            unreachable!(
                "mekr: `target_loop` proven live by the caller's plan phase (mekr_cycles / mekr_empty_ring / mekr_empty_target / mekr_both_empty)"
            )
        };
        l.boundary = LoopBoundary::Cycle { first: he_plus };
        let Some(face_data) = self.get_face_mut(face) else {
            unreachable!("mekr: the face resolved in check_ring_not_outer")
        };
        face_data.rings.retain(|&l| l != ring_loop);
        self.loops.remove(ring_loop);
        self.loop_provenance.remove(ring_loop);
        let Some(vertex) = self.get_vertex_mut(u) else {
            unreachable!("mekr: `u` resolved in check_anchors")
        };
        vertex.emanating = Some(he_plus);
        let Some(vertex) = self.get_vertex_mut(w) else {
            unreachable!("mekr: `w` resolved in check_anchors")
        };
        vertex.emanating = Some(he_minus);
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::Tol;

    use super::*;
    use crate::entity::{Edge, Face, HalfEdge, Shell, Solid, SolidKey, Vertex};
    use crate::euler::{MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
    use crate::fixtures::{deep_snapshot, mvfs_state, pillow, prov};
    use crate::validate::validate;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    /// The start vertices along a loop cycle from `he` — the structural
    /// fingerprint the roundtrip tests compare (loop membership and
    /// keys are asserted separately; this pins the *cycle order*).
    fn starts(body: &Body<f64>, he: HalfEdgeKey) -> Vec<VertexKey> {
        body.loop_cycle(he)
            .unwrap()
            .into_iter()
            .map(|member| body.get_half_edge(member).unwrap().start)
            .collect()
    }

    /// Runs `op` on `body`, asserts it fails with exactly `expected`,
    /// and asserts the body is DEEPLY untouched (every arena entry,
    /// payload, and provenance record — counts alone are too weak for
    /// kill-direction atomicity).
    fn assert_err_deep_unchanged(
        body: &mut Body<f64>,
        expected: &EulerOpError,
        op: impl FnOnce(&mut Body<f64>) -> EulerOpError,
    ) {
        let before = deep_snapshot(body);
        let err = op(body);
        assert_eq!(&err, expected);
        assert_eq!(deep_snapshot(body), before, "body changed on Err");
    }

    /// mvfs + mev(Lone): the segment body (v0 —e0— v1, one loop
    /// `[he_plus, he_minus]`, tol).
    fn segment() -> (Body<f64>, MvfsCreated, MevCreated) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
                Tol::witness(),
            )
            .unwrap();
        (body, seed, seg)
    }

    /// Segment + one strut at v1: cycle
    /// `[e0+, strut+, strut−, e0−]` (v0→v1→v2→v1→v0).
    fn strutted() -> (Body<f64>, MvfsCreated, MevCreated, MevCreated) {
        let (mut body, seed, seg) = segment();
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                p(2.0),
                Tol::witness(),
            )
            .unwrap();
        (body, seed, seg, strut)
    }

    /// The 3-edge open chain v0–v1–v2–v3 in one loop: mvfs + segment +
    /// two struts. Cycle pinned below; e1 is the middle edge whose two
    /// halves have NON-empty sides on both flanks — kemr's general case.
    fn chain3() -> (Body<f64>, MvfsCreated, [MevCreated; 3]) {
        let (mut body, seed, e0) = segment();
        let e1 = body
            .mev_line(
                MevSite::Fan {
                    he1: e0.he_minus,
                    he2: e0.he_minus,
                },
                p(2.0),
                Tol::witness(),
            )
            .unwrap();
        let e2 = body
            .mev_line(
                MevSite::Fan {
                    he1: e1.he_minus,
                    he2: e1.he_minus,
                },
                p(3.0),
                Tol::witness(),
            )
            .unwrap();
        // Pin the cycle the tests below stand on: out along the plus
        // halves, back along the minus halves.
        assert_eq!(
            body.loop_cycle(e0.he_plus),
            Some(vec![
                e0.he_plus,
                e1.he_plus,
                e2.he_plus,
                e2.he_minus,
                e1.he_minus,
                e0.he_minus
            ])
        );
        (body, seed, [e0, e1, e2])
    }

    // ------------------------------------------------------------------
    // kemr: general case, argument order, re-anchoring rules
    // ------------------------------------------------------------------

    #[test]
    fn kemr_general_case_splits_off_a_cycle_ring() {
        let (mut body, seed, [e0, e1, e2]) = chain3();
        let result = body.kemr(e1.he_plus, e1.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // he1's side (strictly between the halves in next order): e2's
        // halves → the ring, anchored at next(he1).
        assert_eq!(
            body.get_loop(result.ring).unwrap().boundary,
            LoopBoundary::Cycle { first: e2.he_plus }
        );
        assert_eq!(
            body.loop_cycle(e2.he_plus),
            Some(vec![e2.he_plus, e2.he_minus])
        );
        for he in [e2.he_plus, e2.he_minus] {
            assert_eq!(body.get_half_edge(he).unwrap().parent_loop, result.ring);
        }
        assert_eq!(body.get_loop(result.ring).unwrap().face, seed.face);
        // he2's side keeps the old loop (still the outer), re-anchored
        // at next(he2).
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle { first: e0.he_minus }
        );
        assert_eq!(
            body.loop_cycle(e0.he_minus),
            Some(vec![e0.he_minus, e0.he_plus])
        );
        let face = body.get_face(seed.face).unwrap();
        assert_eq!(face.outer, seed.r#loop);
        assert_eq!(face.rings, vec![result.ring]);
        // Kill hygiene: dead keys and dead provenance, together.
        assert!(body.get_half_edge(e1.he_plus).is_none());
        assert!(body.get_half_edge(e1.he_minus).is_none());
        assert!(body.get_edge(e1.edge).is_none());
        assert_eq!(body.provenance(EntityId::HalfEdge(e1.he_plus)), None);
        assert_eq!(body.provenance(EntityId::HalfEdge(e1.he_minus)), None);
        assert_eq!(body.provenance(EntityId::Edge(e1.edge)), None);
        // Geometry hygiene: the killed edge's curve was orphaned and
        // removed with it.
        assert_eq!(result.killed_curve, Some(e1.curve));
        assert!(body.get_curve_geom(e1.curve).is_none());
        assert_eq!(body.curves().count(), 2);
        // Result struct: dead keys with the EDGE's slot association
        // (e1.he_plus was minted as the plus half).
        assert_eq!(result.killed_edge, e1.edge);
        assert_eq!(result.killed_he_plus, e1.he_plus);
        assert_eq!(result.killed_he_minus, e1.he_minus);
        // Emanating rule: u = v1 → next(he2), w = v2 → next(he1).
        assert_eq!(
            body.get_vertex(e0.vertex).unwrap().emanating,
            Some(e0.he_minus)
        );
        assert_eq!(
            body.get_vertex(e1.vertex).unwrap().emanating,
            Some(e2.he_plus)
        );
        // D5: the ring records its birth; the argument keys in the
        // record are historical (they are dead now) but compare fine.
        assert_eq!(
            body.provenance(EntityId::Loop(result.ring)),
            Some(&Provenance::Kemr {
                he1: e1.he_plus,
                he2: e1.he_minus
            })
        );
        // Counts: v4 e2, 2 loops (outer + ring), 4 halves, 1 face.
        assert_eq!(body.vertices().count(), 4);
        assert_eq!(body.edges().count(), 2);
        assert_eq!(body.half_edges().count(), 4);
        assert_eq!(body.loops().count(), 2);
        assert_eq!(body.faces().count(), 1);
    }

    #[test]
    fn kemr_argument_order_selects_the_ring_side() {
        // Swapped arguments: the complementary side becomes the ring
        // (GWB §11.5.1's "swap the arguments" advice, same association).
        let (mut body, seed, [e0, e1, e2]) = chain3();
        let result = body.kemr(e1.he_minus, e1.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        // he1 = e1.he_minus: its side is e0's halves → the ring,
        // anchored at next(he1) = e0.he_minus.
        assert_eq!(
            body.get_loop(result.ring).unwrap().boundary,
            LoopBoundary::Cycle { first: e0.he_minus }
        );
        // Old loop keeps e2's halves, re-anchored at next(he2).
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle { first: e2.he_plus }
        );
        // The loop's original Cycle::first (e0.he_plus) migrated into
        // the ring — the unconditional re-anchor kept the survivor
        // coherent.
        assert_eq!(
            body.get_half_edge(e0.he_plus).unwrap().parent_loop,
            result.ring
        );
    }

    #[test]
    fn kemr_reanchors_a_cycle_first_that_pointed_at_a_killed_half() {
        let (mut body, seed, [e0, e1, _e2]) = chain3();
        // Point the loop's representative at a half kemr will kill (any
        // cycle member is a legal `first`, so this is a tier-1-valid
        // rewording of the same loop).
        body.get_loop_mut(seed.r#loop).unwrap().boundary =
            LoopBoundary::Cycle { first: e1.he_plus };
        assert_eq!(validate(&body), Ok(()));
        body.kemr(e1.he_plus, e1.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        // The survivor loop's first is next(he2) — live, never the dead
        // key.
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle { first: e0.he_minus }
        );
    }

    #[test]
    fn kemr_repoints_emanating_that_named_a_killed_half() {
        let (mut body, _seed, [e0, e1, e2]) = chain3();
        // Anchor both endpoint vertices at the doomed halves (tier-1
        // legal: each starts at its vertex).
        body.get_vertex_mut(e0.vertex).unwrap().emanating = Some(e1.he_plus);
        body.get_vertex_mut(e1.vertex).unwrap().emanating = Some(e1.he_minus);
        assert_eq!(validate(&body), Ok(()));
        body.kemr(e1.he_plus, e1.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            body.get_vertex(e0.vertex).unwrap().emanating,
            Some(e0.he_minus)
        );
        assert_eq!(
            body.get_vertex(e1.vertex).unwrap().emanating,
            Some(e2.he_plus)
        );
    }

    // ------------------------------------------------------------------
    // kemr: the degenerate (empty-side) cases
    // ------------------------------------------------------------------

    #[test]
    fn kemr_strut_kill_plants_an_empty_ring() {
        // Mäntylä §9.3 step (g): kill a strut edge; its dangling far
        // vertex is stranded as an EMPTY RING — the hole anchor.
        let (mut body, seed, seg, strut) = strutted();
        let result = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            body.get_loop(result.ring).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: strut.vertex
            }
        );
        assert_eq!(body.get_vertex(strut.vertex).unwrap().emanating, None);
        assert_eq!(body.get_face(seed.face).unwrap().rings, vec![result.ring]);
        // The old loop survives as the segment cycle, first := next(he2).
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: seg.he_minus
            }
        );
        assert_eq!(
            body.loop_cycle(seg.he_minus),
            Some(vec![seg.he_minus, seg.he_plus])
        );
        assert_eq!(
            body.get_vertex(seg.vertex).unwrap().emanating,
            Some(seg.he_minus)
        );
    }

    #[test]
    fn kemr_strut_kill_swapped_strands_the_outer_loop() {
        // Same strut, swapped arguments: the OLD loop (here the outer)
        // becomes the empty loop at the strut tip, and the ring takes
        // the whole surviving cycle. Legal — outer is a maintained
        // designation, not a derivable fact.
        let (mut body, seed, seg, strut) = strutted();
        let result = body.kemr(strut.he_minus, strut.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: strut.vertex
            }
        );
        assert_eq!(
            body.get_loop(result.ring).unwrap().boundary,
            LoopBoundary::Cycle {
                first: seg.he_minus
            }
        );
        assert_eq!(body.get_vertex(strut.vertex).unwrap().emanating, None);
        assert_eq!(body.get_face(seed.face).unwrap().outer, seed.r#loop);
    }

    #[test]
    fn kemr_segment_kill_makes_two_empty_loops() {
        // §9.9(c): killing a segment loop's edge strands BOTH endpoints
        // as lone vertices — old loop at u = start(he1), ring at
        // w = start(he2).
        let (mut body, seed, seg) = segment();
        let result = body.kemr(seg.he_plus, seg.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: seed.vertex
            }
        );
        assert_eq!(
            body.get_loop(result.ring).unwrap().boundary,
            LoopBoundary::Empty { vertex: seg.vertex }
        );
        assert_eq!(body.get_vertex(seed.vertex).unwrap().emanating, None);
        assert_eq!(body.get_vertex(seg.vertex).unwrap().emanating, None);
        assert_eq!(body.half_edges().count(), 0);
        assert_eq!(body.edges().count(), 0);
        assert_eq!(body.curves().count(), 0);

        // Swapped arguments anchor the other way — argument-order
        // determinism of the anchor rule.
        let (mut body2, seed2, seg2) = segment();
        let result2 = body2.kemr(seg2.he_minus, seg2.he_plus).unwrap();
        assert_eq!(validate(&body2), Ok(()));
        assert_eq!(
            body2.get_loop(seed2.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: seg2.vertex
            }
        );
        assert_eq!(
            body2.get_loop(result2.ring).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: seed2.vertex
            }
        );
    }

    // ------------------------------------------------------------------
    // kemr: preconditions (typed error + body deeply unchanged)
    // ------------------------------------------------------------------

    /// One face whose single loop is the 2-cycle of a self-loop edge at
    /// one vertex — the state whose kemr would strand two empty loops on
    /// one vertex. Deliberately tier-1-INVALID (the vertex orbit is
    /// split over the two halves), which is exactly why the check is
    /// defensive rather than load-bearing.
    fn self_loop_segment() -> (Body<f64>, HalfEdgeKey, HalfEdgeKey, VertexKey) {
        let mut body = Body::<f64>::new();
        let null_he = HalfEdgeKey::default();
        let pt = body.add_point(p(0.0));
        let v = body.add_vertex(
            Vertex {
                point: pt,
                emanating: None,
            },
            prov(),
        );
        let curve = body.add_curve(crate::fixtures::test_curve(p(0.0), Tol::witness()));
        let edge = body.add_edge(
            Edge {
                he_plus: null_he,
                he_minus: null_he,
                curve,
            },
            prov(),
        );
        let half = |body: &mut Body<f64>| {
            body.add_half_edge(
                HalfEdge {
                    edge,
                    start: v,
                    parent_loop: LoopKey::default(),
                    next: null_he,
                    prev: null_he,
                },
                prov(),
            )
        };
        let h1 = half(&mut body);
        let h2 = half(&mut body);
        let solid = body.add_solid(Solid { shells: vec![] }, prov());
        let shell = body.add_shell(
            Shell {
                faces: vec![],
                solid,
            },
            prov(),
        );
        body.get_solid_mut(solid).unwrap().shells.push(shell);
        let surface = body.add_surface(crate::fixtures::test_surface(p(0.0)));
        let lp = body.add_loop(
            Loop {
                boundary: LoopBoundary::Cycle { first: h1 },
                face: FaceKey::default(),
            },
            prov(),
        );
        let face = body.add_face(
            Face {
                sense: true,
                surface,
                outer: lp,
                rings: vec![],
                shell,
            },
            prov(),
        );
        body.get_loop_mut(lp).unwrap().face = face;
        body.get_shell_mut(shell).unwrap().faces.push(face);
        body.get_edge_mut(edge).unwrap().he_plus = h1;
        body.get_edge_mut(edge).unwrap().he_minus = h2;
        for (a, b) in [(h1, h2), (h2, h1)] {
            let he = body.get_half_edge_mut(a).unwrap();
            he.next = b;
            he.prev = b;
            he.parent_loop = lp;
        }
        body.get_vertex_mut(v).unwrap().emanating = Some(h1);
        (body, h1, h2, v)
    }

    #[test]
    fn kemr_rejects_colliding_empty_anchors() {
        let (mut body, h1, h2, v) = self_loop_segment();
        let expected = EulerOpError::EmptyAnchorsCollide { vertex: v };
        assert_err_deep_unchanged(&mut body, &expected, |b| b.kemr(h1, h2).unwrap_err());
    }

    #[test]
    fn kemr_rejects_stale_half_edges() {
        let (mut body, _seed, [e0, e1, _e2]) = chain3();
        let dead = body.add_half_edge(
            HalfEdge {
                edge: e0.edge,
                start: e0.vertex,
                parent_loop: LoopKey::default(),
                next: e0.he_plus,
                prev: e0.he_plus,
            },
            prov(),
        );
        body.half_edges.remove(dead);
        let expected = EulerOpError::StaleKey {
            key: EntityId::HalfEdge(dead),
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kemr(dead, e1.he_minus).unwrap_err()
        });
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kemr(e1.he_plus, dead).unwrap_err()
        });
    }

    #[test]
    fn kemr_rejects_non_mate_half_edges() {
        let (mut body, _seed, [e0, e1, _e2]) = chain3();
        // The same key twice.
        let expected = EulerOpError::NotSameEdge {
            he1: e0.he_plus,
            he2: e0.he_plus,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kemr(e0.he_plus, e0.he_plus).unwrap_err()
        });
        // Halves of different edges.
        let expected = EulerOpError::NotSameEdge {
            he1: e0.he_plus,
            he2: e1.he_minus,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kemr(e0.he_plus, e1.he_minus).unwrap_err()
        });
    }

    #[test]
    fn kemr_rejects_an_edge_that_does_not_claim_its_halves() {
        // Corrupt bijection: both halves agree on the edge key, but the
        // edge claims something else in one slot — tier-1-invalid input
        // surfaced as the same typed error.
        let (mut body, _seed, [e0, e1, _e2]) = chain3();
        body.get_edge_mut(e1.edge).unwrap().he_plus = e0.he_plus;
        let expected = EulerOpError::NotSameEdge {
            he1: e1.he_plus,
            he2: e1.he_minus,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kemr(e1.he_plus, e1.he_minus).unwrap_err()
        });
    }

    #[test]
    fn kemr_rejects_halves_in_different_loops() {
        // The pillow's rim edges have their halves in the two cap
        // loops; kemr needs an edge occurring twice in ONE loop.
        let mut t = pillow(Tol::witness());
        let (he1, he2) = (t.hes_a[0], t.hes_b[0]);
        let expected = EulerOpError::NotSameLoop { he1, he2 };
        assert_err_deep_unchanged(&mut t.body, &expected, |b| b.kemr(he1, he2).unwrap_err());
    }

    // ------------------------------------------------------------------
    // kemr ∘ mekr roundtrips, one per kemr output configuration.
    // Restoration is asserted STRUCTURALLY (cycle-order fingerprints,
    // memberships, counts, E–P) — keys differ by design (fresh
    // edge/halves; kemr's ring dies again). The general isomorphism
    // oracle is PR 4's proptest business.
    // ------------------------------------------------------------------

    #[test]
    fn kemr_mekr_roundtrip_general_case() {
        let (mut body, seed, [e0, e1, e2]) = chain3();
        let original = starts(&body, e1.he_plus);
        let kill = body.kemr(e1.he_plus, e1.he_minus).unwrap();
        // Inverse anchors (module docs): target = next(he2), ring =
        // next(he1) restores the original cycle order exactly.
        let site = MekrSite::Cycles {
            target: e0.he_minus,
            ring: e2.he_plus,
        };
        let restore = body.mekr_chord(site, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));

        assert_eq!(starts(&body, restore.he_plus), original);
        assert!(body.get_loop(kill.ring).is_none());
        assert_eq!(body.provenance(EntityId::Loop(kill.ring)), None);
        assert_eq!(restore.killed_ring, kill.ring);
        // One loop again — the seed loop — owning every half.
        assert_eq!(
            body.get_face(seed.face).unwrap().rings,
            Vec::<LoopKey>::new()
        );
        for he in body.loop_cycle(restore.he_plus).unwrap() {
            assert_eq!(body.get_half_edge(he).unwrap().parent_loop, seed.r#loop);
        }
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: restore.he_plus
            }
        );
        // Direction: he_plus runs target anchor vertex → ring anchor
        // vertex (v1 → v2, the killed e1's direction).
        assert_eq!(
            body.get_half_edge(restore.he_plus).unwrap().start,
            e0.vertex
        );
        assert_eq!(body.half_edge_end(restore.he_plus), Some(e1.vertex));
        // Counts + E–P ledger restored: v − e + f − r = 4 − 3 + 1 − 0
        // = 2 = 2(1 − 0), genus 0, one shell.
        assert_eq!(body.vertices().count(), 4);
        assert_eq!(body.edges().count(), 3);
        assert_eq!(body.half_edges().count(), 6);
        assert_eq!(body.loops().count(), 1);
        assert_eq!(body.faces().count(), 1);
        assert_eq!(body.curves().count(), 3);
        // Emanating rule after mekr: u → he_plus, w → he_minus.
        assert_eq!(
            body.get_vertex(e0.vertex).unwrap().emanating,
            Some(restore.he_plus)
        );
        assert_eq!(
            body.get_vertex(e1.vertex).unwrap().emanating,
            Some(restore.he_minus)
        );
        // D5 on the fresh entities.
        for id in [
            EntityId::Edge(restore.edge),
            EntityId::HalfEdge(restore.he_plus),
            EntityId::HalfEdge(restore.he_minus),
        ] {
            assert_eq!(
                body.provenance(id),
                Some(&Provenance::Mekr { site }),
                "for {id}"
            );
        }
    }

    #[test]
    fn kemr_mekr_roundtrip_empty_ring_case() {
        let (mut body, seed, seg, strut) = strutted();
        let original = starts(&body, strut.he_plus);
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        let restore = body
            .mekr_chord(
                MekrSite::EmptyRing {
                    target: seg.he_minus,
                    ring: kill.ring,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(starts(&body, restore.he_plus), original);
        assert!(body.get_loop(kill.ring).is_none());
        assert_eq!(
            body.get_face(seed.face).unwrap().rings,
            Vec::<LoopKey>::new()
        );
        assert_eq!(body.half_edge_end(restore.he_plus), Some(strut.vertex));
        assert_eq!(
            body.get_vertex(strut.vertex).unwrap().emanating,
            Some(restore.he_minus)
        );
        assert_eq!(body.vertices().count(), 3);
        assert_eq!(body.edges().count(), 2);
        assert_eq!(body.loops().count(), 1);
    }

    #[test]
    fn kemr_mekr_roundtrip_empty_target_case() {
        let (mut body, seed, seg, strut) = strutted();
        let original = starts(&body, strut.he_minus);
        let kill = body.kemr(strut.he_minus, strut.he_plus).unwrap();
        // The outer loop is now Empty (the target); the ring holds the
        // whole surviving cycle.
        let restore = body
            .mekr_chord(
                MekrSite::EmptyTarget {
                    target: seed.r#loop,
                    ring: seg.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(starts(&body, restore.he_plus), original);
        assert!(body.get_loop(kill.ring).is_none());
        // The surviving loop is the outer again, and it is a cycle.
        assert_eq!(body.get_face(seed.face).unwrap().outer, seed.r#loop);
        assert_eq!(
            body.get_face(seed.face).unwrap().rings,
            Vec::<LoopKey>::new()
        );
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: restore.he_plus
            }
        );
        // he_plus runs lone target vertex → ring anchor vertex.
        assert_eq!(
            body.get_half_edge(restore.he_plus).unwrap().start,
            strut.vertex
        );
        assert_eq!(body.half_edge_end(restore.he_plus), Some(seg.vertex));
    }

    #[test]
    fn kemr_mekr_roundtrip_both_empty_case() {
        let (mut body, seed, seg) = segment();
        let original = starts(&body, seg.he_plus);
        let kill = body.kemr(seg.he_plus, seg.he_minus).unwrap();
        let restore = body
            .mekr_chord(
                MekrSite::BothEmpty {
                    target: seed.r#loop,
                    ring: kill.ring,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(starts(&body, restore.he_plus), original);
        assert!(body.get_loop(kill.ring).is_none());
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: restore.he_plus
            }
        );
        assert_eq!(
            body.get_vertex(seed.vertex).unwrap().emanating,
            Some(restore.he_plus)
        );
        assert_eq!(
            body.get_vertex(seg.vertex).unwrap().emanating,
            Some(restore.he_minus)
        );
        assert_eq!(body.loops().count(), 1);
        assert_eq!(body.half_edges().count(), 2);
        assert_eq!(body.edges().count(), 1);
    }

    // ------------------------------------------------------------------
    // mekr: preconditions
    // ------------------------------------------------------------------

    #[test]
    fn mekr_rejects_anchors_in_one_loop() {
        let (mut body, seed, [e0, e1, _e2]) = chain3();
        let expected = EulerOpError::SameLoop {
            r#loop: seed.r#loop,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.mekr_chord(
                MekrSite::Cycles {
                    target: e0.he_plus,
                    ring: e1.he_plus,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn mekr_rejects_loops_of_different_faces() {
        let mut t = pillow(Tol::witness());
        let (target, ring) = (t.hes_a[0], t.hes_b[0]);
        let expected = EulerOpError::NotSameFace {
            target: t.loop_a,
            ring: t.loop_b,
        };
        assert_err_deep_unchanged(&mut t.body, &expected, |b| {
            b.mekr_chord(MekrSite::Cycles { target, ring }, Tol::witness())
                .unwrap_err()
        });
    }

    #[test]
    fn mekr_rejects_a_ring_argument_naming_the_outer_loop() {
        // Ring state (outer segment + cycle ring in one face), then the
        // ROLES swapped: the outer loop as the loop to kill.
        let (mut body, seed, [e0, e1, e2]) = chain3();
        body.kemr(e1.he_plus, e1.he_minus).unwrap();
        let expected = EulerOpError::RingIsOuter {
            r#loop: seed.r#loop,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.mekr_chord(
                MekrSite::Cycles {
                    target: e2.he_plus,
                    ring: e0.he_minus,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn mekr_sites_require_their_boundary_shapes() {
        let (mut body, seed, [e0, e1, _e2]) = chain3();
        let kill = body.kemr(e1.he_plus, e1.he_minus).unwrap(); // cycle ring
        // EmptyRing with a cycle ring loop.
        let expected = EulerOpError::LoopNotEmpty { r#loop: kill.ring };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.mekr_chord(
                MekrSite::EmptyRing {
                    target: e0.he_minus,
                    ring: kill.ring,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
        // EmptyTarget with a cycle target loop.
        let expected = EulerOpError::LoopNotEmpty {
            r#loop: seed.r#loop,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.mekr_chord(
                MekrSite::EmptyTarget {
                    target: seed.r#loop,
                    ring: e0.he_minus,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
        // BothEmpty with cycle loops.
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.mekr_chord(
                MekrSite::BothEmpty {
                    target: seed.r#loop,
                    ring: kill.ring,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn mekr_rejects_stale_keys() {
        let (mut body, _seed, [e0, e1, _e2]) = chain3();
        body.kemr(e1.he_plus, e1.he_minus).unwrap();
        // The killed plus half as an anchor: stale.
        let expected = EulerOpError::StaleKey {
            key: EntityId::HalfEdge(e1.he_plus),
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.mekr_chord(
                MekrSite::Cycles {
                    target: e1.he_plus,
                    ring: e0.he_minus,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
        // A null loop key as the ring.
        let dead_loop = LoopKey::default();
        let expected = EulerOpError::StaleKey {
            key: EntityId::Loop(dead_loop),
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.mekr_chord(
                MekrSite::EmptyRing {
                    target: e0.he_minus,
                    ring: dead_loop,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn mekr_rejects_colliding_lone_vertices() {
        // Two empty loops of one face holding the SAME lone vertex —
        // tier-1-invalid (MultiplyOwned), raw-built to pin the
        // defensive check.
        let mut t = mvfs_state();
        let extra = t.body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex: t.vertex },
                face: t.face,
            },
            prov(),
        );
        t.body.get_face_mut(t.face).unwrap().rings.push(extra);
        let expected = EulerOpError::EmptyAnchorsCollide { vertex: t.vertex };
        let target = t.lone_loop;
        assert_err_deep_unchanged(&mut t.body, &expected, |b| {
            b.mekr_chord(
                MekrSite::BothEmpty {
                    target,
                    ring: extra,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    // ------------------------------------------------------------------
    // Kill/lineage semantics under replay (D9)
    // ------------------------------------------------------------------

    #[test]
    fn kill_lineage_semantics_are_pinned() {
        // Build A: chain + balanced kemr∘mekr pair + one more mev.
        let (mut body_a, _seed_a, [a0, a1, a2]) = chain3();
        let kill_a = body_a.kemr(a1.he_plus, a1.he_minus).unwrap();
        let restore_a = body_a
            .mekr_chord(
                MekrSite::Cycles {
                    target: a0.he_minus,
                    ring: a2.he_plus,
                },
                Tol::witness(),
            )
            .unwrap();
        let extra_a = body_a
            .mev_line(
                MevSite::Fan {
                    he1: a0.he_minus,
                    he2: a0.he_minus,
                },
                p(9.0),
                Tol::witness(),
            )
            .unwrap();

        // Build B: the identical history — deeply identical body and
        // identical key sequence. D9's replay contract INCLUDES kills.
        let (mut body_b, _seed_b, [b0, b1, b2]) = chain3();
        let kill_b = body_b.kemr(b1.he_plus, b1.he_minus).unwrap();
        let restore_b = body_b
            .mekr_chord(
                MekrSite::Cycles {
                    target: b0.he_minus,
                    ring: b2.he_plus,
                },
                Tol::witness(),
            )
            .unwrap();
        let extra_b = body_b
            .mev_line(
                MevSite::Fan {
                    he1: b0.he_minus,
                    he2: b0.he_minus,
                },
                p(9.0),
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(deep_snapshot(&body_a), deep_snapshot(&body_b));
        assert_eq!((kill_a, restore_a, extra_a), (kill_b, restore_b, extra_b));

        // Build C: the same history WITHOUT the pair.
        let (mut body_c, _seed_c, [c0, _c1, _c2]) = chain3();
        let extra_c = body_c
            .mev_line(
                MevSite::Fan {
                    he1: c0.he_minus,
                    he2: c0.he_minus,
                },
                p(9.0),
                Tol::witness(),
            )
            .unwrap();

        // Kills consume and release slots: the roundtrip re-mints its
        // entities in the recycled slots with BUMPED GENERATIONS, so the
        // with-pair body is not deeply identical to the pair-free one.
        // That is expected — a kill is part of the construction history,
        // and D9 promises identical keys only for identical histories.
        assert_ne!(deep_snapshot(&body_a), deep_snapshot(&body_c));
        assert_ne!(restore_a.edge, a1.edge);
        assert_ne!(restore_a.he_plus, a1.he_plus);
        assert_ne!(restore_a.he_minus, a1.he_minus);
        // The pair re-consumed every half-edge/edge/curve slot it freed,
        // so a follow-up mev — which mints NO loop — gets the same keys
        // in both histories. (This deliberately does not witness the
        // loop arena: the pair leaves one freed LOOP slot behind, and
        // that boundary is pinned by pair_convergence_is_per_arena
        // below.)
        assert_eq!(extra_a, extra_c);
    }

    #[test]
    fn pair_convergence_is_per_arena() {
        // A balanced kemr∘mekr pair is slot-balanced only PER-ARENA:
        // mekr re-consumes the half-edge/edge/curve slots kemr freed,
        // but kemr's ring-LOOP slot (freed again by mekr) is re-consumed
        // by nothing in the pair. The previous test pins convergence via
        // mev — the one make-op that mints no loop — so the loop-arena
        // boundary needs this dedicated pin (review SHOULD, 2026-07-16).
        let (mut body_a, _seed_a, [a0, a1, a2]) = chain3();
        body_a.kemr(a1.he_plus, a1.he_minus).unwrap();
        body_a
            .mekr_chord(
                MekrSite::Cycles {
                    target: a0.he_minus,
                    ring: a2.he_plus,
                },
                Tol::witness(),
            )
            .unwrap();
        let (mut body_c, _seed_c, [c0, c1, c2]) = chain3();
        // chain3 is deterministic, so the two histories share their
        // pre-pair keys — the same mef sites address both bodies.
        assert_eq!((a0, a1, a2), (c0, c1, c2));

        // The FIRST loop-minting op after the pair: its loop lands in
        // the recycled slot with a bumped generation (keys differ);
        // every other minted key has already converged (face slots were
        // never touched, curve/edge/half-edge slots were re-consumed by
        // the pair).
        let mef1_a = body_a
            .mef_chord(
                MefSite::Chords {
                    he1: a0.he_plus,
                    he2: a2.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        let mef1_c = body_c
            .mef_chord(
                MefSite::Chords {
                    he1: c0.he_plus,
                    he2: c2.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_ne!(mef1_a.r#loop, mef1_c.r#loop);
        assert_eq!(mef1_a.face, mef1_c.face);
        assert_eq!(mef1_a.edge, mef1_c.edge);
        assert_eq!(mef1_a.he_plus, mef1_c.he_plus);
        assert_eq!(mef1_a.he_minus, mef1_c.he_minus);
        assert_eq!(mef1_a.curve, mef1_c.curve);

        // One loop-mint later the loop arena has converged too: the
        // second mef agrees on every key.
        let mef2_a = body_a
            .mef_chord(
                MefSite::Chords {
                    he1: a2.he_minus,
                    he2: a0.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        let mef2_c = body_c
            .mef_chord(
                MefSite::Chords {
                    he1: c2.he_minus,
                    he2: c0.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(mef2_a, mef2_c);
        assert_eq!(validate(&body_a), Ok(()));
        assert_eq!(validate(&body_c), Ok(()));
    }

    // ------------------------------------------------------------------
    // kfmrh
    // ------------------------------------------------------------------

    /// The digon pillow built through the operators (the euler module's
    /// doctest construction): two faces sharing two edges.
    fn ops_pillow() -> (Body<f64>, MvfsCreated, MevCreated, MefCreated) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
                Tol::witness(),
            )
            .unwrap();
        let split = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        (body, seed, seg, split)
    }

    /// A `format!` fingerprint of one arena's full contents (keys +
    /// payloads), for the "kfmrh touches no half-edge/vertex/edge"
    /// assertions.
    fn arena_lines<K: core::fmt::Debug, E: core::fmt::Debug>(
        iter: impl Iterator<Item = (K, E)>,
    ) -> Vec<String> {
        iter.map(|(k, e)| format!("{k:?}: {e:?}")).collect()
    }

    #[test]
    fn kfmrh_demotes_f2s_outer_to_a_ring_of_f1() {
        let (mut body, seed, _seg, split) = ops_pillow();
        let hes_before = arena_lines(body.half_edges());
        let edges_before = arena_lines(body.edges());
        let vertices_before = arena_lines(body.vertices());

        let result = body.kfmrh(seed.face, split.face).unwrap();
        assert_eq!(validate(&body), Ok(()));

        assert_eq!(result.ring, split.r#loop);
        assert_eq!(result.killed_face, split.face);
        // Shared surface (every mef face shares mvfs's): kept, and the
        // op reports it kept it.
        assert_eq!(result.killed_surface, None);
        assert_eq!(body.surfaces().count(), 1);
        // The loop SURVIVES — repointed and demoted, with its birth
        // record intact (kfmrh mints nothing and rewrites no records).
        let ring = body.get_loop(split.r#loop).unwrap();
        assert_eq!(ring.face, seed.face);
        assert!(matches!(
            body.provenance(EntityId::Loop(split.r#loop)),
            Some(Provenance::Mef { .. })
        ));
        let face = body.get_face(seed.face).unwrap();
        assert_eq!(face.outer, seed.r#loop);
        assert_eq!(face.rings, vec![split.r#loop]);
        // The face died with its provenance; the shell dropped it.
        assert!(body.get_face(split.face).is_none());
        assert_eq!(body.provenance(EntityId::Face(split.face)), None);
        assert_eq!(body.get_shell(seed.shell).unwrap().faces, vec![seed.face]);
        // No half-edge, edge, or vertex was touched — a truly global
        // manipulation (Mäntylä §9.2.4).
        assert_eq!(arena_lines(body.half_edges()), hes_before);
        assert_eq!(arena_lines(body.edges()), edges_before);
        assert_eq!(arena_lines(body.vertices()), vertices_before);
        // E–P at genus 1: v − e + f − r = 2 − 2 + 1 − 1 = 0 = 2(1 − 1).
        // The digon pillow's caps glued into one face is torus-like
        // incidence — geometrically degenerate, tier-1-legal.
        assert_eq!(body.vertices().count(), 2);
        assert_eq!(body.edges().count(), 2);
        assert_eq!(body.faces().count(), 1);
        assert_eq!(body.loops().count(), 2);
    }

    #[test]
    fn kfmrh_accepts_an_empty_f2_outer_and_reaps_its_surface() {
        // A same-shell face with an Empty outer is unreachable through
        // the PR 1–3 operator set (mvfs seeds a fresh solid; PR 4's
        // mfkrh will mint such faces), so the fixture grafts one in raw:
        // pillow + face C with an Empty outer at a lone vertex and its
        // OWN surface, same shell. Tier-1-valid.
        let mut t = pillow(Tol::witness());
        let pt = t.body.add_point(p(9.0));
        let v = t.body.add_vertex(
            Vertex {
                point: pt,
                emanating: None,
            },
            prov(),
        );
        let surface = t.body.add_surface(crate::fixtures::test_surface(p(9.0)));
        let lp = t.body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex: v },
                face: FaceKey::default(),
            },
            prov(),
        );
        let face_c = t.body.add_face(
            Face {
                sense: true,
                surface,
                outer: lp,
                rings: vec![],
                shell: t.shell,
            },
            prov(),
        );
        t.body.get_loop_mut(lp).unwrap().face = face_c;
        t.body.get_shell_mut(t.shell).unwrap().faces.push(face_c);
        assert_eq!(validate(&t.body), Ok(()), "fixture must be tier-1 valid");

        let result = t.body.kfmrh(t.face_a, face_c).unwrap();
        assert_eq!(validate(&t.body), Ok(()));
        // The Empty outer became an Empty RING of face A — §9.3 (g)'s
        // hole-planting state, reached through kfmrh instead of kemr.
        assert_eq!(
            t.body.get_loop(lp).unwrap().boundary,
            LoopBoundary::Empty { vertex: v }
        );
        assert_eq!(t.body.get_loop(lp).unwrap().face, t.face_a);
        assert_eq!(t.body.get_face(t.face_a).unwrap().rings, vec![lp]);
        // Face C's private surface was orphaned by the kill and removed
        // (geometry hygiene).
        assert_eq!(result.killed_surface, Some(surface));
        assert!(t.body.get_surface(surface).is_none());
    }

    #[test]
    fn kfmrh_rejects_same_face_and_cross_shell() {
        let (mut body, seed, _seg, _split) = ops_pillow();
        let expected = EulerOpError::SameFace { face: seed.face };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kfmrh(seed.face, seed.face).unwrap_err()
        });
        // A second mvfs is a second solid+shell in the same body:
        // cross-SOLID kfmrh stays a typed error (M3 PR 1 lifted only
        // the same-solid cross-shell case, as shell fusion).
        let other = body.mvfs(p(9.0)).unwrap();
        let expected = EulerOpError::CrossSolid {
            f1: seed.face,
            f2: other.face,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kfmrh(seed.face, other.face).unwrap_err()
        });
    }

    /// A same-solid two-shell body: the shape `kfmrh`'s fusion form
    /// exists for. It is not constructible through the public
    /// operators (`mvfs` mints one solid per shell), so the second
    /// shell is re-homed by raw in-crate write — the same adversarial
    /// posture as the rest of this module's corruption rows.
    fn fused_two_shell_body() -> (Body<f64>, MvfsCreated, MvfsCreated) {
        let (mut body, seed, _seg, _split) = ops_pillow();
        let other = body.mvfs(p(9.0)).unwrap();
        let f1_shell = body.get_face(seed.face).unwrap().shell;
        let first_solid = body.get_shell(f1_shell).unwrap().solid;
        body.get_shell_mut(other.shell).unwrap().solid = first_solid;
        body.get_solid_mut(first_solid)
            .unwrap()
            .shells
            .push(other.shell);
        body.get_solid_mut(other.solid).unwrap().shells.clear();
        (body, seed, other)
    }

    /// The fusion's first plan-phase guard, on the exact corruption it
    /// exists to refuse. The re-homing walks `f2`'s shell's face list —
    /// a key read out of the body, not an argument — so
    /// `kfmrh_rejects_stale_faces`, which corrupts only the two
    /// ARGUMENTS, cannot reach it. Without the guard the op half-wrote
    /// the fusion and returned `Ok`.
    #[test]
    fn kfmrh_refuses_a_dangling_face_in_f2s_shell() {
        let (mut body, seed, other) = fused_two_shell_body();
        let dead = FaceKey::default();
        body.get_shell_mut(other.shell).unwrap().faces.push(dead);
        let expected = EulerOpError::StaleKey {
            key: EntityId::Face(dead),
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kfmrh(seed.face, other.face).unwrap_err()
        });
    }

    /// The fusion's second guard. Both shells name the same DEAD solid,
    /// so the `CrossSolid` gate — which only compares the two — passes,
    /// and the solid guard is the only thing left to catch it. The
    /// fusion rewrites that solid's shell list.
    #[test]
    fn kfmrh_refuses_a_dead_shared_solid() {
        let (mut body, seed, other) = fused_two_shell_body();
        let dead = SolidKey::default();
        let f1_shell = body.get_face(seed.face).unwrap().shell;
        body.get_shell_mut(f1_shell).unwrap().solid = dead;
        body.get_shell_mut(other.shell).unwrap().solid = dead;
        let expected = EulerOpError::StaleKey {
            key: EntityId::Solid(dead),
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kfmrh(seed.face, other.face).unwrap_err()
        });
    }

    #[test]
    fn kfmrh_rejects_f2_with_rings() {
        // Give face B (the mef face) a ring — strut + kemr inside its
        // loop — then try to use it as f2.
        let (mut body, seed, seg, split) = ops_pillow();
        // seg.he_plus moved to the new face's loop at the mef.
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                p(2.0),
                Tol::witness(),
            )
            .unwrap();
        body.kemr(strut.he_plus, strut.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        let expected = EulerOpError::FaceHasRings { face: split.face };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kfmrh(seed.face, split.face).unwrap_err()
        });
    }

    #[test]
    fn kfmrh_rejects_stale_faces() {
        let (mut body, seed, _seg, split) = ops_pillow();
        let dead = FaceKey::default();
        let expected = EulerOpError::StaleKey {
            key: EntityId::Face(dead),
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kfmrh(dead, split.face).unwrap_err()
        });
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.kfmrh(seed.face, dead).unwrap_err()
        });
    }

    // ------------------------------------------------------------------
    // ring_move
    // ------------------------------------------------------------------

    /// Ops-built two-face body with an empty ring on face A (the seed
    /// face): digon pillow + strut + kemr. Everything through public
    /// operators.
    fn pillow_with_ring() -> (Body<f64>, MvfsCreated, MefCreated, KemrResult) {
        let (mut body, seed, seg, split) = ops_pillow();
        // The OLD loop (face A) kept he2's side = {split.he_plus,
        // seg.he_minus}; a strut before seg.he_minus lands in face A.
        let strut = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                p(2.0),
                Tol::witness(),
            )
            .unwrap();
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(body.get_face(seed.face).unwrap().rings, vec![kill.ring]);
        (body, seed, split, kill)
    }

    #[test]
    fn ring_move_reparents_within_a_shell() {
        let (mut body, seed, split, kill) = pillow_with_ring();
        let birth = body.provenance(EntityId::Loop(kill.ring)).cloned();
        let counts_before = (
            body.loops().count(),
            body.faces().count(),
            body.half_edges().count(),
        );

        body.ring_move(kill.ring, split.face).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            body.get_face(seed.face).unwrap().rings,
            Vec::<LoopKey>::new()
        );
        assert_eq!(body.get_face(split.face).unwrap().rings, vec![kill.ring]);
        assert_eq!(body.get_loop(kill.ring).unwrap().face, split.face);
        // NOT an Euler op: nothing created or killed…
        assert_eq!(
            (
                body.loops().count(),
                body.faces().count(),
                body.half_edges().count(),
            ),
            counts_before
        );
        // …and no provenance changes — D5 records are BIRTH records;
        // reparenting is not a re-birth.
        assert_eq!(body.provenance(EntityId::Loop(kill.ring)).cloned(), birth);

        // And back.
        body.ring_move(kill.ring, seed.face).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(body.get_face(seed.face).unwrap().rings, vec![kill.ring]);
    }

    #[test]
    fn ring_move_to_its_own_face_is_a_noop() {
        let (mut body, seed, _split, kill) = pillow_with_ring();
        let before = deep_snapshot(&body);
        assert_eq!(body.ring_move(kill.ring, seed.face), Ok(()));
        // Deeply untouched — in particular the rings order was not
        // perturbed (no retain+push cycle), keeping replay byte-stable.
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn ring_move_rejects_outer_loops_stale_keys_and_cross_shell() {
        let (mut body, seed, split, kill) = pillow_with_ring();
        // The outer loop is not a ring.
        let expected = EulerOpError::RingIsOuter {
            r#loop: seed.r#loop,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.ring_move(seed.r#loop, split.face).unwrap_err()
        });
        // Stale ring key.
        let dead_loop = LoopKey::default();
        let expected = EulerOpError::StaleKey {
            key: EntityId::Loop(dead_loop),
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.ring_move(dead_loop, split.face).unwrap_err()
        });
        // Stale destination face.
        let dead_face = FaceKey::default();
        let expected = EulerOpError::StaleKey {
            key: EntityId::Face(dead_face),
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.ring_move(kill.ring, dead_face).unwrap_err()
        });
        // Cross-shell destination: a second solid's face.
        let other = body.mvfs(p(9.0)).unwrap();
        let expected = EulerOpError::CrossShell {
            f1: seed.face,
            f2: other.face,
        };
        assert_err_deep_unchanged(&mut body, &expected, |b| {
            b.ring_move(kill.ring, other.face).unwrap_err()
        });
    }
}

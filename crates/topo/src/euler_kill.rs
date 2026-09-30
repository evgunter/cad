//! Kill-direction Euler duals — [`Body::kvfs`], [`Body::kev`] (with
//! its describing door [`Body::kev_describing`]), [`Body::kef`] — and
//! the ring-promotion inverse [`Body::mfkrh`] (M1 PR 4).
//!
//! These complete the ten-operator catalog (Mäntylä ch. 9): every
//! make-direction operator now has its exact inverse in-tree, which is
//! what makes the make/kill roundtrip property tests possible (this PR's
//! second half) and what M3's booleans and splitting will consume. The
//! semantics are **forced**: each operator is the exact inverse of its
//! dual, derived from the duals' documented surgeries rather than
//! designed fresh. They share the operator contracts of [`crate::euler`]
//! (atomic typed-error preconditions, deterministic minting, debug
//! postconditions) and the kill-side duties of [`crate::euler_ring`]:
//!
//! - **Kill hygiene.** A killed entity's arena slot AND its D5
//!   provenance entry are removed together; killed keys in the result
//!   structs are **dead** — returned for the caller's records only.
//! - **Geometry hygiene.** Orphaned curves/surfaces/points are reaped
//!   ([`Body::remove_curve_if_orphaned`] /
//!   [`Body::remove_surface_if_orphaned`] /
//!   [`Body::remove_point_if_orphaned`] — deterministic full-arena
//!   scans). With M1's per-entity minting the killed vertex's point and
//!   the killed edge's curve are always orphaned in practice, but the
//!   scan is the rule; the killed face's surface is usually *shared*
//!   (every `mef` face shares its parent's), so `kef`/`kvfs` usually
//!   reap no surface.
//! - **Survivor re-anchoring**, unconditional (the PR 3 pattern): a
//!   surviving loop's `Cycle::first` and a surviving vertex's
//!   `emanating` are overwritten by documented deterministic rules,
//!   never patched only-when-dangling. Which member they name is
//!   documented as arbitrary ([`crate::LoopBoundary::Cycle`],
//!   [`crate::Vertex::emanating`]), so a rule that does not restore the
//!   pre-make anchor is still an exact inverse *of the body*; the
//!   isomorphism oracle used by the roundtrip tests normalizes these
//!   anchors accordingly.
//!
//! Kills and lineage follow PR 3's per-arena replay semantics
//! ([`crate::euler_ring`] module docs): identical histories replay
//! deep-identically; a balanced make∘kill pair leaves survivor keys
//! untouched; a kill∘make pair re-mints in recycled slots with bumped
//! generations, per-arena.
//!
//! # `kvfs` — inverse of `mvfs`
//!
//! [`Body::kvfs`] destroys a solid that is EXACTLY the skeletal `mvfs`
//! state: one shell, one face, no rings, an [`Empty`] outer loop holding
//! the lone vertex. Anything else is a typed error naming the failing
//! arity precisely. Euler vector `(v −1, e 0, f −1, h 0, r 0, s −1)`.
//!
//! # `kev` — inverse of `mev` (the fan merge)
//!
//! [`Body::kev`] kills `he`'s edge and **`end(he)`** — the far vertex —
//! merging that vertex's remaining fan back onto `start(he)`. "The given
//! half-edge's side is the affected thing", uniform with `mef`/`kemr`:
//! the vertex `he` *points at* dies. Euler vector
//! `(v −1, e −1, f 0, h 0, r 0, s 0)`.
//!
//! Derivation (exact inverse of the `mev` fan surgery, [`crate::MevSite`]):
//! `mev(Fan { he1, he2 })` at `v` minted `he_plus`/`he_minus`, spliced
//! them immediately before `he1`/`he2`, and reassigned the clockwise
//! orbit run `[he1 .. he2)` to the new vertex `w`. With `he = he_plus`
//! and `m = mate(he) = he_minus`, the post-state satisfies
//! `next(he) = he1` and `next(m) = he2`, and `w`'s clockwise orbit is
//! `[m, run…]` (the first orbit step from `m` is
//! `next(mate(m)) = next(he) = he1`). So the inverse is forced:
//!
//! - **Fan merge**: every half-edge of `w`'s orbit except `m` — walked
//!   clockwise from `m`, i.e. `vertex_orbit(m)[1..]` — is reassigned to
//!   start at `v` (this is the run move, reversed; the whole remaining
//!   fan moves, there is no direction choice left to make).
//! - **Unsplice**: `link(prev(he), next(he))` and
//!   `link(prev(m), next(m))` — exactly undoing `mev`'s four link
//!   writes. The two half-edges may lie in one loop or two (mev splices
//!   across two loops when `he1`/`he2` do); the unsplice is loop-count
//!   preserving either way.
//!
//! ```text
//!         before                          after
//!      \  |  /                        \  |  /
//!       \ | /  ← fan (orbit of m       \ | /
//!         w       minus m itself)        v   ← merged fan
//!    m ↓  ↑ he ← killed edge            / \
//!         v                          he2   (rest of v's fan)
//!        / \
//!     he2   (rest of v's fan)
//! ```
//!
//! Degenerate cases, each the inverse of a `mev` site case:
//!
//! - **Strut kill** (`next(he) = m`): `w` has no other edges (its orbit
//!   is exactly `[m]` — `next(mate(m)) = next(he) = m` closes it), so
//!   the fan is empty and the kill is pure removal:
//!   `… → prev(he) → he → m → x → …` becomes `… → prev(he) → x → …`.
//!   Inverse of `MevSite::Fan { he1 == he2 }`.
//! - **Segment kill** (the loop is the 2-cycle `[he, m]`): removing both
//!   halves leaves the loop with no members — it becomes
//!   [`Empty`] at `v` (the survivor, matching `MevSite::Lone`'s
//!   old vertex; `Lone` grew exactly this loop). Inverse of
//!   `MevSite::Lone`.
//! - The mirror adjacency (`next(m) = he`) is `v`-valence-1: `v`'s only
//!   edge was the killed one and the whole fan migrates to it. Handled
//!   by the same unsplice, not a separate surgery case — and, like
//!   every other `kev` that merges a fan, it undoes no single `mev`
//!   (see the re-make taxonomy below).
//!
//! Re-anchoring rules (unconditional): the loop of `he` re-anchors at
//! the first survivor after `he` in `next` order (`next(he)`, or
//! `next(m)` when `next(he) = m`); the loop of `m` — when distinct — at
//! `next(m)`; when both halves share one loop the `next(m)`-side write
//! is last and wins (deterministic). `v.emanating` becomes the first
//! merged-fan member (= `next(he)`), else the strut case's `next(m)`,
//! else `None` (segment kill — `v` is lone again). The plan proves it
//! before mutating: the fan's member by the orbit walk, which shows it
//! starts at `w` for the merge to re-base, and the other two through
//! the crate-internal `Body::require_kill_anchors` (`next(m)` starts at
//! `v`; `None` leaves `v` lone: no half-edge but the killed two starts
//! there, and the segment kill's loop is `Empty` at it). The loop writes
//! go through the same proof: each `first` is not killed and lies in its
//! loop, and the segment kill's loop keeps no member but the killed two
//! and empties at a `v` written `None` that no other loop holds, so a
//! kill that merges a fan onto `v` never empties it. Where the killed
//! halves are adjacent, the one loop re-anchored is both halves' loop.
//!
//! **The merged fan's geometry.** The merge moves an end of every
//! merged member from `w`'s point to `v`'s, and each keeps the carrier
//! it was certified with. [`Body::kev`] is keys-only and ε-free: it
//! carries a merge that moves nothing — an empty fan, or a killed null
//! edge, whose two vertices hold one point — and otherwise refuses
//! every merge with a certified member, one at a single point included
//! ([`EulerOpError::MergeRebasesCarriers`], naming every one), or one
//! that moves one end of a null edge
//! ([`EulerOpError::RebasedNullEdge`]). [`Body::kev_describing`] is the
//! same kill with a band and the members' re-descriptions: a listed
//! member is certified with its spec at the merged endpoints, an
//! unlisted one passes the re-basing gate `mev`'s fan site passes, so
//! `kev_describing(he, &[], tol)` is the merge that moves nothing or
//! moves within band. [`Body::kev_merged_members`] reads the members
//! and their merged endpoints.
//!
//! # `kef` — inverse of `mef` (the loop splice)
//!
//! [`Body::kef`] kills `he`'s edge and **the face of `he`'s loop** —
//! `he`'s side dies, the exact inverse of `mef`'s "`he1`'s side becomes
//! the new face" (a `mef` is undone by `kef(created.he_minus)`, the half
//! it placed in the new loop, tol). The dying loop's remnant merges into the
//! mate's loop. The edge must border two DISTINCT faces and the dying
//! face must be RING-FREE (move rings off with [`Body::ring_move`]
//! first, mirroring `kfmrh` — or kill the mate's side if that one is
//! bare). Euler vector `(v 0, e −1, f −1, h 0, r 0, s 0)`.
//!
//! Derivation (exact inverse of the `mef` tail swap,
//! [`crate::MefSite`]): `mef(Chords { he1, he2 })` spliced
//! `he_minus` before `he1` and `he_plus` before `he2`, moving
//! `[he1 .. he2)` into the new loop. With `he = he_minus` and
//! `m = he_plus`, the post-state satisfies `next(he) = he1`,
//! `prev(he) = ` old `prev(he2)`, `next(m) = he2`, `prev(m) = ` old
//! `prev(he1)`. Undoing the four link writes gives the general splice:
//! `link(prev(m), next(he))` and `link(prev(he), next(m))` — the dying
//! loop's remnant is stitched into the mate's loop across the gap the
//! killed halves leave:
//!
//! ```text
//!          before                            after
//!    ┌── next(he) ──┐  DYING LOOP
//!    │              ↓  (he's side)     ┌── next(he) ──┐
//!    he ← killed    │                  │              ↓
//!    ↑              │                  │  (one loop)  │
//!    └── prev(he) ──┘                  ↑              │
//!    ┌── next(m) ───┐  MATE'S LOOP     └─ prev(he) →  │
//!    │              ↓  (survives)         next(m) ····┘
//!    m ← killed     │
//!    ↑              │
//!    └── prev(m) ───┘
//! ```
//!
//! Degenerate cases, each the inverse of a `mef` site case (`he` is
//! always the half in the dying loop):
//!
//! - **Dying loop is `[he]` alone** (`next(he) = he`): the remnant is
//!   empty; the mate is simply unspliced from its loop
//!   (`link(prev(m), next(m))`). Inverse of
//!   `MefSite::Chords { he1 == he2 }` applied at `created.he_minus` —
//!   the one-edge circular face dies.
//! - **Mate's loop is `[m]` alone** (`next(m) = m`, which forces the
//!   edge to be a self-loop): the remnant closes into itself
//!   (`link(prev(he), next(he))`) and becomes the surviving loop's
//!   whole cycle. This is `kef` on the OTHER half of the same
//!   configuration (killing the big side of a circular-edge split);
//!   the one-op re-make is a `Chords { he1 == he2 }` from the surviving
//!   side — exact up to isomorphism iff the surviving singleton is the
//!   outer of a ring-free face (the only shape `mef` itself creates —
//!   face identities swap, which the oracle does not track). When the
//!   survivor is a ring, or its face carries rings, the re-split lands
//!   the big loop on the wrong side of the ring distribution and no
//!   single op restores the original (see the taxonomy below).
//! - **Both alone** (`[he]` and `[m]`, a self-loop edge): the surviving
//!   loop becomes [`Empty`] at the one vertex, `emanating`
//!   `None`. Inverse of `MefSite::Lone`.
//!
//! `kef` allows `start(he) == end(he)`: self-loop edges are exactly its
//! `Lone`/circular-inverse territory (the *kev* precondition is the one
//! that demands distinct endpoints).
//!
//! Re-anchoring rules (unconditional): the surviving loop re-anchors at
//! `next(m)` when it survives the kill, else `next(he)` (both dead only
//! in the `Lone` inverse, which anchors the [`Empty`] state
//! instead). Emanating: `start(he)` gets `next(m)`, falling back to
//! `next(he)`, else `None`; `start(m)` symmetrically gets `next(he)`
//! falling back to `next(m)`, else `None`; when the endpoints coincide
//! the `start(m)`-side write is last and wins (kemr's precedent). The
//! plan proves every anchor write before mutating (the crate-internal
//! `Body::require_kill_anchors`): a `Some` starts at its endpoint, a
//! `None` leaves it lone (no half-edge but the killed two starts there,
//! and the `Empty` surviving loop holds it), the remnant claims the
//! dying loop and is all of it but `he`, the surviving loop's `first`
//! is not killed and lies in it once the remnant has moved in, and an
//! `Empty` surviving loop keeps no member but the killed two and holds
//! a vertex no other loop holds.
//!
//! # `mfkrh` — inverse of `kfmrh`
//!
//! [`Body::mfkrh`] promotes a ring — [`Cycle`](crate::LoopBoundary::Cycle)
//! OR [`Empty`] — to the outer loop of a NEW face in the same
//! shell. Empty-outer faces hereby become **operator-reachable** (as
//! PR 3 predicted): promoting an empty ring yields a face whose outer
//! loop is an empty loop, the `mvfs`-face shape inside a larger body.
//! Euler vector `(v 0, e 0, f +1, h −1, r −1, s 0)`.
//!
//! The new face's surface is the caller's [`FaceSurface`] choice —
//! `Inherit` (share the demoting face's), `New` (mint), or `Shared` (an
//! existing key); see [`Body::mfkrh`]. **Exact key or surface
//! restoration is NOT promised** for a `kfmrh ∘ mfkrh` pair: `kfmrh`
//! may have reaped the original surface, or — more commonly — the
//! killed face *shared* a surviving surface, so there is nothing to
//! restore a reference to. The isomorphism oracle ignores surface
//! payloads for exactly this reason (`crate::iso`).
//!
//! # The make ↔ kill inverse map (all five pairs, for the record)
//!
//! | make | exact inverse | degenerate cases |
//! |---|---|---|
//! | `mvfs` | `kvfs(created.solid)` | — |
//! | `mev(site)` | `kev(created.he_plus)` — `kev_describing(created.he_plus, &[], tol)` where the run is non-empty and the new edge certified | `Fan{he1==he2}` ↔ strut kill; `Lone` ↔ segment kill |
//! | `mef(site)` | `kef(created.he_minus)` | `Chords{he1==he2}` ↔ dying-loop-alone; `Lone` ↔ both-alone |
//! | `mekr(site)` | `kemr(created.he_plus, created.he_minus)` | per-site, see [`crate::euler_ring`] |
//! | `kfmrh(f1, f2)` | `mfkrh(result.ring)` | empty-outer `f2` ↔ empty-ring promotion |
//!
//! and in the kill∘make direction the re-make sites are derived from the
//! pre-kill neighborhood (`kev` ↔ `mev(Fan{next(he), next(mate(he))})`
//! etc., tol). The make∘kill direction is exact for every site; the
//! kill∘make direction is exact for every site EXCEPT the subcases
//! below, which have no single-op re-make (the roundtrip property
//! tests skip exactly these — precise statement and proof sketch in
//! the seqgen test-support module):
//!
//! - `kev` wherever the far vertex carries a fan, the mirror adjacency
//!   and the general merge alike. The mirror's full-fan `mev` run is
//!   inexpressible under the ratified empty-run convention and its
//!   strut re-make puts the fan at the wrong coordinates; the general
//!   merge's re-make is a fan `mev` that would have to start the
//!   merged members at a vertex their chords do not run to, which
//!   `mev`'s re-basing gate refuses. (Coordinate-coincident endpoints
//!   would collapse the distinction, inside the oracle's documented
//!   twin blind spot.)
//! - `kef` mate-alone where the surviving singleton loop is a ring or
//!   its face carries rings (see the degenerate-case list above); the
//!   bare-outer subcase IS one-op re-makeable and is exercised, not
//!   skipped.
//!
//! # Example: grow and ungrow
//!
//! ```
//! use geom_core::Point3;
//! use geom_core::Tol;
//! use topo::{Body, MevSite};
//!
//! # fn run() -> Result<(), topo::EulerOpError> {
//! let tol = Tol::witness();
//! let mut body = Body::<f64>::new();
//! let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true)?;
//! let seg = body.mev_line(
//!     MevSite::Lone { r#loop: seed.r#loop },
//!     Point3::new(1.0, 0.0, 0.0),
//!     tol,
//! )?;
//! // kev(he_plus) kills end(he_plus) — the far vertex — and the edge:
//! // the loop is empty again, holding the seed vertex.
//! let kill = body.kev(seg.he_plus)?;
//! assert_eq!(kill.killed_vertex, seg.vertex);
//! assert_eq!(
//!     body.get_loop(seed.r#loop).unwrap().boundary,
//!     topo::LoopBoundary::Empty { vertex: seed.vertex },
//! );
//! // And kvfs takes the skeletal remnant back to nothing.
//! let end = body.kvfs(seed.solid)?;
//! assert_eq!(end.killed_vertex, seed.vertex);
//! assert_eq!(body.solids().count(), 0);
//! assert_eq!(body.vertices().count(), 0);
//! assert_eq!(body.points().count(), 0);
//! # Ok(()) }
//! # run().unwrap();
//! ```
//!
//! [`Empty`]: crate::LoopBoundary::Empty

use geom_brep::{EdgeCurve, EdgeCurveSpec};
use geom_core::{Decide, Point3, Real, Tol};

use crate::body::Body;
use crate::entity::{
    EdgeKey, EntityId, Face, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey,
    VertexKey,
};
#[cfg(debug_assertions)]
use crate::euler::ArenaDelta;
use crate::euler::{
    EulerOpError, FaceSurface, KillAnchor, KillInto, KillRun, ParentSide, RunExtent, shared_loop,
};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::live::{Live, require_key};
use crate::pcurves::SiteHalf;
use crate::provenance::Provenance;

/// The outcome of one [`Body::kvfs`] call: five dead topology keys plus
/// the reaped geometry.
///
/// Every `killed_*` key is **dead** — it no longer resolves; the keys
/// are returned for the caller's records (e.g. an operation log), not
/// for lookup.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KvfsResult {
    /// The killed solid (dead key).
    pub killed_solid: SolidKey,
    /// The killed shell (dead key).
    pub killed_shell: ShellKey,
    /// The killed face (dead key).
    pub killed_face: FaceKey,
    /// The killed (empty) outer loop (dead key).
    pub killed_loop: LoopKey,
    /// The killed lone vertex (dead key).
    pub killed_vertex: VertexKey,
    /// The face's surface (dead key), if killing the face orphaned it
    /// and it was removed; `None` if another face still references it
    /// (possible only through surface sharing, which no M1 construction
    /// of a *skeletal* solid produces — the scan is the rule).
    pub killed_surface: Option<SurfaceKey>,
    /// The vertex's point (dead key), if killing the vertex orphaned it
    /// and it was removed; `None` if another vertex still references it
    /// (never, with M1's per-vertex minting — the scan is the rule).
    pub killed_point: Option<PointKey>,
}

/// The outcome of one [`Body::kev`] call: the killed edge complex, the
/// killed far vertex, and the reaped geometry.
///
/// Every `killed_*` key is **dead** (see [`KvfsResult`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KevResult {
    /// The killed edge (dead key).
    pub killed_edge: EdgeKey,
    /// The killed edge's plus half (dead key). Slot association is the
    /// edge's, not the argument's: this is the argument or its mate
    /// according to which the edge claimed as [`crate::Edge::he_plus`].
    pub killed_he_plus: HalfEdgeKey,
    /// The killed edge's minus half (dead key).
    pub killed_he_minus: HalfEdgeKey,
    /// The killed far vertex (dead key). Recorded as the mate's start
    /// vertex — equal to `end(he)` on tier-1-valid input (antiparallel
    /// mates); on corrupt input it reports what was actually killed.
    pub killed_vertex: VertexKey,
    /// The killed edge's curve (dead key), iff orphaned and removed.
    pub killed_curve: Option<CurveKey>,
    /// The killed vertex's point (dead key), iff orphaned and removed.
    pub killed_point: Option<PointKey>,
}

/// The outcome of one [`Body::kef`] call: the killed edge complex, the
/// killed face and its outer loop, and the reaped geometry.
///
/// Every `killed_*` key is **dead** (see [`KvfsResult`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KefResult {
    /// The killed edge (dead key).
    pub killed_edge: EdgeKey,
    /// The killed edge's plus half (dead key). Slot association is the
    /// edge's, as in [`KevResult`].
    pub killed_he_plus: HalfEdgeKey,
    /// The killed edge's minus half (dead key).
    pub killed_he_minus: HalfEdgeKey,
    /// The killed face — the face of the argument half-edge's loop
    /// (dead key).
    pub killed_face: FaceKey,
    /// The killed face's outer loop — the argument half-edge's loop,
    /// whose remnant merged into the mate's loop (dead key).
    pub killed_loop: LoopKey,
    /// The killed edge's curve (dead key), iff orphaned and removed.
    pub killed_curve: Option<CurveKey>,
    /// The killed face's surface (dead key), iff orphaned and removed
    /// by this call — through the explicit check or through the
    /// curve-removal cascade (the killed curve's `Intersection`/`Seam`
    /// description can hold the last reference; issue #86). `None`
    /// whenever something else still references it (the common case —
    /// every `mef` face shares its parent's surface).
    pub killed_surface: Option<SurfaceKey>,
}

/// Every key minted by one [`Body::mfkrh`] call. Nothing is killed: the
/// promoted ring survives as the new face's outer loop, keeping its key
/// and its D5 birth record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MfkrhCreated {
    /// The new face (outer loop = the promoted ring; same shell as the
    /// ring's former face).
    pub face: FaceKey,
    /// The new face's fresh placeholder surface (see the
    /// [module docs](self) on why sharing cannot be restored).
    pub surface: SurfaceKey,
}

/// One merged member of a [`Body::kev`] fan merge, as
/// [`Body::kev_merged_members`] reads it: the edge, and its two
/// endpoints once the merge has re-based it, in `he_plus` forward
/// order (`start` is `start(he_plus)`'s point, `end` is
/// `start(he_minus)`'s).
#[derive(Clone, Copy, Debug)]
pub struct MergedMember<T: Real> {
    /// The member edge.
    pub edge: EdgeKey,
    /// Where the edge will start: the point its `he_plus` starts at
    /// after the merge.
    pub start: Point3<T>,
    /// Where the edge will end: the point its `he_minus` starts at
    /// after the merge.
    pub end: Point3<T>,
}

/// Everything [`Body::kev_plan`] proves, handed to
/// [`Body::kev_execute`]: the keys the kill reads and writes, the
/// merged fan and its members, and the four splice links as proof
/// tokens.
struct KevPlan {
    he: HalfEdgeKey,
    m: HalfEdgeKey,
    edge: EdgeKey,
    he_plus: HalfEdgeKey,
    he_minus: HalfEdgeKey,
    curve: CurveKey,
    /// The surviving vertex, `start(he)`.
    v: VertexKey,
    /// The dying vertex, `end(he)`.
    w: VertexKey,
    w_point: PointKey,
    /// `he`'s loop and the mate's.
    loops: [LoopKey; 2],
    /// How the unsplice closes the killed halves' gap, and so which loop
    /// anchors it writes ([`KevUnsplice::loop_writes`], proved).
    unsplice: KevUnsplice,
    /// `w`'s surviving fan, clockwise from the mate: the run the merge
    /// re-bases onto `v`. Every half in it starts at `w`.
    fan: Vec<HalfEdgeKey>,
    /// The merged members: every edge with a half in `fan`, once, in
    /// orbit order. None is `edge`.
    members: Vec<EdgeKey>,
    /// `prev(he)`, `next(he)`, `prev(m)`, `next(m)`.
    links: [Live; 4],
    /// `v`'s new `emanating` (the emanating rule, module docs), proved:
    /// it starts at `v` once the merge has re-based the fan, or leaves
    /// `v` lone.
    anchor: KillAnchor,
}

/// How [`Body::kev`]'s unsplice closes the gap the killed halves leave
/// (module docs), read from their two `next` steps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KevUnsplice {
    /// The loop was the 2-cycle `[he, m]`, and empties.
    Segment,
    /// `next(he) = m`: one link, `prev(he) → next(m)`.
    Strut,
    /// `next(m) = he`: one link, `prev(m) → next(he)`.
    Mirror,
    /// Each half unspliced from its own loop, one loop or two.
    General,
}

impl KevUnsplice {
    /// The loop anchors this arm writes (the loop rule, module docs), in
    /// order, where both name one loop the second winning: each loop
    /// re-anchors at the first survivor after its killed half in `next`
    /// order, and the segment's loop empties at the survivor `v`.
    fn loop_writes(
        self,
        [l1, l2]: [LoopKey; 2],
        [next_he, next_m]: [HalfEdgeKey; 2],
        v: VertexKey,
    ) -> Vec<(LoopKey, LoopBoundary)> {
        match self {
            Self::Segment => vec![(l1, LoopBoundary::Empty { vertex: v })],
            Self::Strut => vec![(l1, LoopBoundary::Cycle { first: next_m })],
            Self::Mirror => vec![(l1, LoopBoundary::Cycle { first: next_he })],
            Self::General => vec![
                (l1, LoopBoundary::Cycle { first: next_he }),
                (l2, LoopBoundary::Cycle { first: next_m }),
            ],
        }
    }
}

/// How [`Body::kef`]'s splice closes the gap the killed halves leave
/// (module docs), read from the dying loop's remnant and the mate's
/// `next`.
#[derive(Clone, Copy)]
enum KefSplice {
    /// The `Lone` inverse: a self-loop edge whose halves were both
    /// one-half-edge loops. The surviving loop empties.
    Lone,
    /// An empty remnant (the dying loop was `[he]`): the mate unspliced
    /// from its loop.
    Unsplice,
    /// The mate was alone: the remnant, from its first member `b`,
    /// closes into itself and becomes the surviving loop's whole cycle.
    MateAlone(Live),
    /// The remnant, from its first member `b`, stitched across the
    /// mate's gap.
    General(Live),
}

/// [`Body::kev`]'s arena delta, shared by both kill doors.
#[cfg(debug_assertions)]
const KEV_DELTA: ArenaDelta = ArenaDelta {
    half_edges: -2,
    edges: -1,
    vertices: -1,
    ..ArenaDelta::ZERO
};

impl<T: Decide> Body<T> {
    /// KVFS — *kill vertex, face, solid*: the inverse of [`Body::mvfs`].
    /// Destroys a solid in EXACTLY the skeletal state `mvfs` creates:
    /// one shell, one face with no rings, an
    /// [`LoopBoundary::Empty`] outer loop holding one lone vertex.
    ///
    /// Euler vector: `(v −1, e 0, f −1, h 0, r 0, s −1)` — arena deltas
    /// −1 solid, −1 shell, −1 face, −1 loop, −1 vertex.
    ///
    /// **Minting order**: nothing is minted. **Kill order** (D9, exact,
    /// the reverse of `mvfs`'s spine minting): face, loop, shell, solid,
    /// vertex (each with its provenance entry), then the face's surface
    /// iff orphaned and the vertex's point iff orphaned (geometry
    /// hygiene, module docs).
    ///
    /// # Precondition check order
    ///
    /// The solid resolves ([`EulerOpError::StaleKey`]); it has exactly
    /// one shell ([`EulerOpError::SolidNotSingleShell`]); the shell
    /// resolves (`StaleKey`); it has exactly one face
    /// ([`EulerOpError::ShellNotSingleFace`]); the face resolves
    /// (`StaleKey`); it has no rings ([`EulerOpError::FaceHasRings`]);
    /// its outer loop resolves (`StaleKey`); the loop is empty
    /// ([`EulerOpError::LoopNotEmpty`]); the lone vertex resolves
    /// (`StaleKey`); no half-edge claims the loop
    /// ([`EulerOpError::LoopCycleBroken`] naming the loop); no half-edge
    /// starts at the vertex ([`EulerOpError::OrbitBroken`] naming the
    /// first that does, the `Lone` proof every kill that leaves a vertex
    /// lone makes). Both are tier-1-invalid input: a torn `parent_loop`
    /// or start makes a half-edge of another shell name the loop or the
    /// vertex, and the kill would leave it naming a dead one. A second
    /// `Empty` loop holding the vertex is tier-1-invalid input this plan
    /// does not check; the kill would leave that loop holding a dead
    /// vertex.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kvfs(&mut self, solid: SolidKey) -> Result<KvfsResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let listed = self.shells_of_solid(solid).ok_or(EulerOpError::StaleKey {
            key: EntityId::Solid(solid),
        })?;
        let [shell] = listed[..] else {
            return Err(EulerOpError::SolidNotSingleShell {
                solid,
                shells: listed.len(),
            });
        };
        let shell_data = self.get_shell(shell).ok_or(EulerOpError::StaleKey {
            key: EntityId::Shell(shell),
        })?;
        let [face] = shell_data.faces[..] else {
            return Err(EulerOpError::ShellNotSingleFace {
                shell,
                faces: shell_data.faces.len(),
            });
        };
        let face_data = self.get_face(face).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        if !face_data.rings.is_empty() {
            return Err(EulerOpError::FaceHasRings { face });
        }
        let loop_key = face_data.outer;
        let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(loop_key),
        })?;
        let LoopBoundary::Empty { vertex } = loop_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: loop_key });
        };
        let vertex_data = self.get_vertex(vertex).ok_or(EulerOpError::StaleKey {
            key: EntityId::Vertex(vertex),
        })?;
        let point = vertex_data.point;
        self.require_run_of([], loop_key, RunExtent::Whole, &[])?;
        if let Some(he) = self.starts_at_besides(vertex, &[]) {
            return Err(EulerOpError::OrbitBroken { he });
        }

        // ---- Mutation (infallible from here on). ----
        // Kill order (documented above): face, loop, shell, solid,
        // vertex, then orphaned geometry.
        self.faces.remove(face);
        self.face_provenance.remove(face);
        // Null-face record hygiene (M3 PR 1): a record never outlives
        // its face (crate::null).
        self.null_faces.remove(face);
        self.loops.remove(loop_key);
        self.loop_provenance.remove(loop_key);
        self.shells.remove(shell);
        self.shell_provenance.remove(shell);
        self.solids.remove(solid);
        self.solid_provenance.remove(solid);
        self.vertices.remove(vertex);
        self.vertex_provenance.remove(vertex);
        let killed_surface = self
            .remove_surface_if_orphaned(face_data.surface)
            .then_some(face_data.surface);
        let killed_point = self.remove_point_if_orphaned(point).then_some(point);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                solids: -1,
                shells: -1,
                faces: -1,
                loops: -1,
                vertices: -1,
                ..ArenaDelta::ZERO
            },
            "kvfs",
        );
        Ok(KvfsResult {
            killed_solid: solid,
            killed_shell: shell,
            killed_face: face,
            killed_loop: loop_key,
            killed_vertex: vertex,
            killed_surface,
            killed_point,
        })
    }

    /// KEV — *kill edge, vertex*: the inverse of [`Body::mev`]. Kills
    /// `he`'s edge and **`end(he)`** (the far vertex), merging that
    /// vertex's remaining edge fan back onto `start(he)`.
    ///
    /// Which vertex dies is the argument's choice: `kev(he)` kills the
    /// vertex `he` points at; to kill the other endpoint, pass the mate.
    /// Full derivation, surgery diagram, and the degenerate strut/
    /// segment cases (each the inverse of a [`crate::MevSite`] case):
    /// [module docs](self).
    ///
    /// Euler vector: `(v −1, e −1, f 0, h 0, r 0, s 0)` — arena deltas
    /// −2 half-edges, −1 edge, −1 vertex.
    ///
    /// **Minting order**: nothing is minted. **Kill order** (D9, exact):
    /// `he`, its mate, the edge, the far vertex (each with its
    /// provenance entry), then the edge's curve iff orphaned and the
    /// vertex's point iff orphaned.
    ///
    /// **Re-anchoring** (unconditional, module docs): affected loops
    /// re-anchor at the first survivor after the killed half in `next`
    /// order; the survivor vertex's `emanating` becomes the first
    /// merged-fan member, else the strut case's `next(mate)`, else
    /// `None` (segment kill — the loop is [`LoopBoundary::Empty`] at the
    /// survivor again).
    ///
    /// # The merged fan
    ///
    /// The far vertex's surviving edges — the **merged members** — are
    /// re-based onto `start(he)`, each keeping the curve it was
    /// certified with, which pins one end of its carrier to the dying
    /// vertex's point. This door takes no band, so it certifies none of
    /// them: it refuses **every** merge with a certified member
    /// ([`EulerOpError::MergeRebasesCarriers`], naming each in orbit
    /// order), a merge of two vertices at one point included, since
    /// whether the survivor's point is the dying vertex's is the question
    /// the re-basing gate does not ask (the crate-internal
    /// `Body::certify_rebased_run`'s docs). It carries what structure
    /// answers: an empty merged fan, a killed null edge, and a null
    /// member with both halves in the fan; a null member with one half
    /// in it refuses [`EulerOpError::RebasedNullEdge`].
    ///
    /// A killed null edge moves nothing because its two vertices hold
    /// one point: [`Body::mev_null`] mints them so, and both re-basing
    /// gates refuse to move one end of one. Nothing else enforces it,
    /// and the doors that write vertex points (`replace_face`,
    /// `offset_*`) take tier-2-valid bodies, which hold no null edge.
    ///
    /// A merge that moves nothing, or moves its members within band,
    /// goes through `kev_describing(he, &[], tol)`
    /// ([`Body::kev_describing`]), and a merge that re-describes its
    /// members hands their specs to that door;
    /// [`Body::kev_merged_members`] reads the members and the endpoints
    /// the merge gives each.
    ///
    /// # Precondition check order
    ///
    /// `he` resolves ([`EulerOpError::StaleKey`]); its edge resolves
    /// (`StaleKey`) and claims it
    /// ([`EulerOpError::UnclaimedHalfEdge`]); the mate resolves
    /// (`StaleKey`); the endpoints are distinct
    /// ([`EulerOpError::SelfLoopEdge`]); both endpoint vertices resolve
    /// (`StaleKey`); both parent loops resolve (`StaleKey`) and are
    /// cycles ([`EulerOpError::LoopNotCycle`]); the far vertex's orbit
    /// closes and every half-edge on it starts at the far vertex
    /// ([`EulerOpError::OrbitBroken`] — tier-1-invalid input: a torn
    /// `next` can walk it through the killed half itself); the four
    /// splice links (`prev`/`next` of both halves) resolve
    /// (`StaleKey`); where the merged fan is empty, the survivor's new
    /// `emanating` holds: `next(mate)` starts at the survivor, or, where
    /// `next(mate)` is `he` and the anchor is `None`, the survivor is
    /// left lone: no half-edge but the killed two starts there, and the
    /// segment kill's loop empties at it (`OrbitBroken` naming `he` — a
    /// torn `next` can put the step on another vertex, or on `he` at a
    /// survivor that keeps edges or whose loop does not empty; a fan's
    /// anchor is its first member, which the orbit proof covers); each
    /// loop's new anchor holds: its `first` is not killed and lies in the
    /// loop, and the segment kill's loop keeps no member but the killed
    /// two and empties at a survivor written `None` that no other loop
    /// holds ([`EulerOpError::LoopCycleBroken`] naming the loop —
    /// tier-1-invalid input: a torn `next` can land the step on the mate
    /// or in another loop, or read the loop as `[he, mate]` while it
    /// keeps other members or while the merge moves a fan onto the
    /// survivor, and a torn start can put the survivor on another loop's
    /// lone vertex);
    /// where the halves are adjacent in `next` order, both lie in one
    /// loop (`LoopCycleBroken` naming the mate's loop, which that arm
    /// does not re-anchor). Then, where the merged fan is not
    /// empty: the killed
    /// edge's curve entry resolves ([`EulerOpError::StaleGeometry`]),
    /// and unless it is a null edge, per merged member in orbit order,
    /// the member and its curve entry resolve (`StaleKey` /
    /// `StaleGeometry`) and it is not a null edge the merge moves one
    /// end of ([`EulerOpError::RebasedNullEdge`]); last, no member is
    /// certified ([`EulerOpError::MergeRebasesCarriers`], naming all
    /// of them).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kev(&mut self, he: HalfEdgeKey) -> Result<KevResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();
        let plan = self.kev_plan(he)?;
        self.kev_keys_only_gate(&plan)?;
        let result = self.kev_execute(plan);
        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, KEV_DELTA, "kev");
        Ok(result)
    }

    /// KEV with the merged fan's re-descriptions: [`Body::kev`]'s kill,
    /// taking a band and, for any merged member whose stored carrier
    /// does not describe it at the merged endpoints, the spec that
    /// does.
    ///
    /// The shape of [`Body::mev`] on the kill side: the site (`he`),
    /// the geometry the kill writes (`redescriptions`) and the band
    /// (`tol`). Topology and descriptions are written together, so no
    /// state between them is ever observable, and nothing is written
    /// until everything has certified.
    ///
    /// - **A listed member** is certified with its spec against the
    ///   endpoints the merge WILL give it — the surviving vertex's point
    ///   at every half in the merged fan, its current point elsewhere —
    ///   through the attachment gate [`Body::set_edge_curve`] certifies
    ///   through, adjacency coherence included, and its curve is
    ///   replaced by fresh insertion (the old one reaped iff orphaned).
    ///   Pcurve rows stand, as they do under `set_edge_curve` (its docs
    ///   say why, and what tier 3 does with them).
    /// - **An unlisted member** keeps its carrier and passes the
    ///   re-basing gate [`Body::mev`]'s fan site passes, under this
    ///   band: its stored description re-certified against the merged
    ///   endpoints, a pre-existing staleness carried and a null edge
    ///   the merge moves one end of refused. Where the killed edge is a
    ///   null edge nothing moves and the gate is not asked ([`Body::kev`]
    ///   says why).
    ///
    /// An empty list is the kill for a caller whose members all pass
    /// where they land — a merge of two vertices at one point, or a band
    /// apart — and a non-empty one is the kill for a caller that
    /// re-describes what it merges: `split_edge`'s inverse re-attaching
    /// the parent's spec, a blend handing a member the chord it spans.
    /// [`Body::kev_merged_members`] reads the members and the endpoints
    /// the merge gives each, which is what a caller states its specs
    /// against.
    ///
    /// Euler vector, kill order and re-anchoring: [`Body::kev`]'s. Then,
    /// per listed member in list order: its certified curve is inserted
    /// and its old curve reaped iff orphaned, through the write
    /// [`Body::set_edge_curve`] makes. [`KevResult`] reports what the
    /// kill kills and not these: a re-described member survives, its
    /// new curve is its edge's `curve`, and the replaced curve is reaped
    /// exactly as `set_edge_curve` reaps one, which reports no reaped
    /// key either.
    ///
    /// # Precondition check order
    ///
    /// [`Body::kev`]'s structural list (through the adjacent arms' one
    /// loop).
    /// Then per entry of `redescriptions`, in list order: the edge is a
    /// merged member ([`EulerOpError::NotMergedMember`]); it was not
    /// listed before ([`EulerOpError::DuplicateRedescription`]); its
    /// spec is adjacency-coherent
    /// ([`EulerOpError::DescriptionNotAdjacent`], with `StaleKey` for a
    /// half, loop or face that does not resolve); the merged endpoints
    /// resolve (`StaleKey` / [`EulerOpError::StaleGeometry`], the
    /// surviving vertex's point among them); the spec certifies against
    /// them ([`EulerOpError::RebasedCarrier`] naming the edge). Then,
    /// where the merged fan is not empty, the killed edge's curve entry
    /// resolves (`StaleGeometry`); and unless it is a null edge, where a
    /// member is left unlisted, the surviving vertex's point resolves
    /// (`StaleGeometry`) and the unlisted members pass the re-basing
    /// gate in orbit order ([`EulerOpError::StaleKey`] /
    /// `StaleGeometry` / [`EulerOpError::RebasedNullEdge`] /
    /// [`EulerOpError::RebasedCarrier`] naming the first that fails).
    /// So an empty merged fan with an empty list asks nothing past the
    /// structural list, and the two doors agree there.
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kev_describing(
        &mut self,
        he: HalfEdgeKey,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<KevResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();
        let plan = self.kev_plan(he)?;
        let described = self.kev_describing_gate(&plan, redescriptions, tol)?;
        let result = self.kev_execute(plan);
        // Every listed edge is a merged member, and no merged member is
        // the killed edge (`kev_plan` proves it), so each one survives
        // the kill to be written.
        for (edge, curve) in described {
            self.replace_edge_curve(edge, curve);
        }
        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, KEV_DELTA, "kev_describing");
        Ok(result)
    }

    /// The merged members of `kev(he)` — every edge the kill re-bases
    /// onto `start(he)`, once each, in the dying vertex's clockwise
    /// orbit order from the killed edge — each with the endpoints the
    /// merge gives it. Read from the plan both kill doors run and the
    /// endpoint reading their gates certify against, so a spec stated
    /// between these endpoints is one [`Body::kev_describing`]
    /// certifies at them. Pure.
    ///
    /// Empty where the merged fan is: a strut or segment kill merges
    /// nothing.
    ///
    /// # Errors
    ///
    /// [`Body::kev`]'s structural list; then, where the fan is not
    /// empty, `StaleKey` / [`EulerOpError::StaleGeometry`] where the
    /// surviving vertex's point or a member's other endpoint does not
    /// resolve. Nothing the merge's geometry decides is asked.
    pub fn kev_merged_members(
        &self,
        he: HalfEdgeKey,
    ) -> Result<Vec<MergedMember<T>>, EulerOpError> {
        let plan = self.kev_plan(he)?;
        if plan.members.is_empty() {
            return Ok(Vec::new());
        }
        let p_v = self.resolve_vertex_point(plan.v)?;
        plan.members
            .iter()
            .map(|&edge| {
                let (start, end) = self.rebased_endpoints(edge, &plan.fan, p_v)?;
                Ok(MergedMember { edge, start, end })
            })
            .collect()
    }

    /// The kill with neither door's gate: plan, then execute. For
    /// fixtures that plant a state no door produces (a carrier its
    /// endpoint left, a null self-loop), which the gates' own arms are
    /// rowed against.
    #[cfg(test)]
    pub(crate) fn kev_ungated(&mut self, he: HalfEdgeKey) -> Result<KevResult, EulerOpError> {
        let plan = self.kev_plan(he)?;
        Ok(self.kev_execute(plan))
    }

    /// [`Body::kev`]'s structural plan phase, shared by both kill doors
    /// (the precondition list through the adjacent arms' one loop, before
    /// the fan-merge geometry). Pure.
    ///
    /// Beyond resolving every key the kill writes, it proves that every
    /// half-edge of the merged fan starts at the dying vertex, and the
    /// survivor's new `emanating` and the loops' new anchors
    /// ([`Body::require_kill_anchors`]). The first is
    /// what keeps the killed edge out of its own merged members: its
    /// halves are `he`, which starts at the survivor, and the mate, which
    /// heads the orbit walk and so is not in the fan. The walk steps
    /// through `next(mate(·))` and reads no start vertex, so a torn
    /// `next` can put a foreign half-edge — the killed half among
    /// them — on it, and only this check sees one.
    fn kev_plan(&self, he: HalfEdgeKey) -> Result<KevPlan, EulerOpError> {
        let he_data = self.resolve_half_edge(he)?;
        let edge = he_data.edge;
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let (he_plus, he_minus, curve) = (edge_data.he_plus, edge_data.he_minus, edge_data.curve);
        let m = if he_plus == he {
            he_minus
        } else if he_minus == he {
            he_plus
        } else {
            return Err(EulerOpError::UnclaimedHalfEdge { he, edge });
        };
        let m_data = self.resolve_half_edge(m)?;
        let v = he_data.start; // survives
        let w = m_data.start; // dies (= end(he))
        if v == w {
            return Err(EulerOpError::SelfLoopEdge { edge, vertex: v });
        }
        require_key(&self.vertices, v, EntityId::Vertex)?;
        let w_point = self
            .get_vertex(w)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Vertex(w),
            })?
            .point;
        let l1 = he_data.parent_loop;
        let l2 = m_data.parent_loop;
        for loop_key in [l1, l2] {
            let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
                key: EntityId::Loop(loop_key),
            })?;
            if matches!(loop_data.boundary, LoopBoundary::Empty { .. }) {
                return Err(EulerOpError::LoopNotCycle { r#loop: loop_key });
            }
        }
        // w's whole fan, walked clockwise from the doomed mate: the
        // members after m are the survivors to merge onto v (bounded
        // walk, D9).
        let orbit_w = self
            .vertex_orbit(m)
            .ok_or(EulerOpError::OrbitBroken { he: m })?;
        self.require_orbit_starts_at(&orbit_w, w, m)?;
        let fan: Vec<HalfEdgeKey> = orbit_w[1..].to_vec();
        let members = self.run_edges(&fan)?;
        // The unsplice writes through all four neighbor links; prove
        // them now so the mutation below cannot fail midway (atomicity).
        let (a, b) = (
            self.require_live(he_data.prev)?,
            self.require_live(he_data.next)?,
        );
        let (c, d) = (
            self.require_live(m_data.prev)?,
            self.require_live(m_data.next)?,
        );
        // Emanating rule (unconditional, module docs): the fan's first
        // member, which the orbit proof above shows starts at `w` and the
        // merge re-bases onto `v`. With no fan, `v` keeps only what
        // already starts there: `next(m)`, `v`'s orbit step from `he`,
        // or nothing where that step is `he` itself.
        let anchor = match fan.first() {
            Some(&first) => KillAnchor::Merged(first),
            None => KillAnchor::step((d.key() != he).then_some(d.key())),
        };
        // The unsplice arm, read from the two `next` steps. The segment's
        // loop empties at `v`, so it needs `v` left lone: the proof below
        // refuses it where `v`'s anchor is not `Lone`, which is where a
        // fan merges onto `v`. An adjacent pair's arm re-anchors one loop,
        // so it needs both halves in that loop.
        let unsplice = match (b.key() == m, d.key() == he) {
            (true, true) => KevUnsplice::Segment,
            (true, false) => KevUnsplice::Strut,
            (false, true) => KevUnsplice::Mirror,
            (false, false) => KevUnsplice::General,
        };
        let loops = [l1, l2];
        self.require_kill_anchors(
            &[(v, anchor, he)],
            &unsplice.loop_writes(loops, [b.key(), d.key()], v),
            &[he, m],
            None,
        )?;
        if unsplice != KevUnsplice::General && shared_loop(&he_data, &m_data).is_none() {
            return Err(EulerOpError::LoopCycleBroken { r#loop: l2 });
        }
        Ok(KevPlan {
            he,
            m,
            edge,
            he_plus,
            he_minus,
            curve,
            v,
            w,
            w_point,
            loops,
            unsplice,
            fan,
            members,
            links: [a, b, c, d],
            anchor,
        })
    }

    /// Whether the merge moves anything: `false` for an empty merged
    /// fan and for a killed null edge (whose two vertices hold one
    /// point — [`Body::kev`]'s docs), `true` otherwise.
    fn kev_merge_moves(&self, plan: &KevPlan) -> Result<bool, EulerOpError> {
        if plan.fan.is_empty() {
            return Ok(false);
        }
        match self.get_curve_geom(plan.curve) {
            Some(crate::null::CurveGeom::NullScaffold(_)) => Ok(false),
            Some(crate::null::CurveGeom::Certified(_)) => Ok(true),
            None => Err(EulerOpError::StaleGeometry {
                key: crate::GeomRef::Curve(plan.curve),
            }),
        }
    }

    /// [`Body::kev`]'s ε-free gate over the merged fan (its docs): the
    /// re-basing gate's null arm, and every certified member named.
    fn kev_keys_only_gate(&self, plan: &KevPlan) -> Result<(), EulerOpError> {
        if !self.kev_merge_moves(plan)? {
            return Ok(());
        }
        let mut stranded: Vec<EdgeKey> = Vec::new();
        for &member in &plan.members {
            if self.rebased_carrier(member, &plan.fan)?.is_some() {
                stranded.push(member);
            }
        }
        if stranded.is_empty() {
            Ok(())
        } else {
            Err(EulerOpError::MergeRebasesCarriers { edges: stranded })
        }
    }

    /// [`Body::kev_describing`]'s gate: every listed spec certified
    /// against the merged endpoints, every unlisted member through the
    /// re-basing gate. Pure; returns the certified curves in list
    /// order for the mutation to write.
    fn kev_describing_gate(
        &self,
        plan: &KevPlan,
        redescriptions: &[(EdgeKey, EdgeCurveSpec<T>)],
        tol: Tol,
    ) -> Result<Vec<(EdgeKey, EdgeCurve<T>)>, EulerOpError> {
        // The survivor's point, resolved at the first question that
        // needs it and not before.
        let mut p_v: Option<Point3<T>> = None;
        let mut survivor_point = || -> Result<Point3<T>, EulerOpError> {
            if let Some(p) = p_v {
                return Ok(p);
            }
            let p = self.resolve_vertex_point(plan.v)?;
            p_v = Some(p);
            Ok(p)
        };
        let mut listed: Vec<EdgeKey> = Vec::with_capacity(redescriptions.len());
        let mut certified = Vec::with_capacity(redescriptions.len());
        for (edge, spec) in redescriptions {
            let edge = *edge;
            if !plan.members.contains(&edge) {
                return Err(EulerOpError::NotMergedMember { edge });
            }
            if listed.contains(&edge) {
                return Err(EulerOpError::DuplicateRedescription { edge });
            }
            listed.push(edge);
            self.check_description_adjacent(edge, &spec.description)?;
            let (p_start, p_end) = self.rebased_endpoints(edge, &plan.fan, survivor_point()?)?;
            let curve = self
                .certify_edge_spec(spec.clone(), p_start, p_end, tol)
                .map_err(|e| match e {
                    EulerOpError::Certification { error } => {
                        EulerOpError::RebasedCarrier { edge, error }
                    }
                    other => other,
                })?;
            certified.push((edge, curve));
        }
        if self.kev_merge_moves(plan)? && listed.len() < plan.members.len() {
            let mut run: Vec<HalfEdgeKey> = Vec::with_capacity(plan.fan.len());
            for &moved in &plan.fan {
                if !listed.contains(&self.resolve_half_edge(moved)?.edge) {
                    run.push(moved);
                }
            }
            self.certify_rebased_run(&run, survivor_point()?, tol)?;
        }
        Ok(certified)
    }

    /// The kill itself, from a proven plan: the fan merge, the
    /// unsplice, the re-anchoring and the kills. Infallible — every
    /// key it touches was resolved by [`Body::kev_plan`] — and it
    /// checks no geometry, so it is the gated doors' tail and nothing
    /// else's: it is private to this module, whose callers are those
    /// two doors and the test-only `Body::kev_ungated`.
    fn kev_execute(&mut self, plan: KevPlan) -> KevResult {
        let KevPlan {
            he,
            m,
            edge,
            he_plus,
            he_minus,
            curve,
            v,
            w,
            w_point,
            loops,
            unsplice,
            fan,
            members: _,
            links: [a, b, c, d],
            anchor,
        } = plan;
        // Fan merge: everything starting at w except the doomed mate now
        // starts at v (the run move, reversed — module docs).
        for &moved in &fan {
            let Some(half_edge) = self.get_half_edge_mut(moved) else {
                unreachable!(
                    "kev: the fan's members were resolved by the plan phase's bounded walk"
                )
            };
            half_edge.start = v;
        }
        // Unsplice (derived as mev's exact inverse — module docs), then
        // the loop anchors the plan proved.
        match unsplice {
            // The 2-cycle loop empties: the inverse of MevSite::Lone.
            KevUnsplice::Segment => {}
            // … a → he → m → d …: one write bridges both.
            KevUnsplice::Strut => self.link_half_edges(a, d),
            // … c → m → he → b ….
            KevUnsplice::Mirror => self.link_half_edges(c, b),
            KevUnsplice::General => {
                self.link_half_edges(a, b);
                self.link_half_edges(c, d);
            }
        }
        for (r#loop, boundary) in unsplice.loop_writes(loops, [b.key(), d.key()], v) {
            let Some(loop_data) = self.get_loop_mut(r#loop) else {
                unreachable!("kev: both loops resolved in the plan phase")
            };
            loop_data.boundary = boundary;
        }
        let Some(vertex) = self.get_vertex_mut(v) else {
            unreachable!("kev: `v` resolved in the plan phase, and only `w` (!= v) is reaped")
        };
        vertex.emanating = anchor.key();
        // Kills, each with its provenance entry (kill order documented
        // on `kev`).
        self.half_edges.remove(he);
        self.half_edge_provenance.remove(he);
        self.half_edges.remove(m);
        self.half_edge_provenance.remove(m);
        self.edges.remove(edge);
        self.edge_provenance.remove(edge);
        self.vertices.remove(w);
        self.vertex_provenance.remove(w);
        let killed_curve = self.remove_curve_if_orphaned(curve).then_some(curve);
        let killed_point = self.remove_point_if_orphaned(w_point).then_some(w_point);
        KevResult {
            killed_edge: edge,
            killed_he_plus: he_plus,
            killed_he_minus: he_minus,
            killed_vertex: w,
            killed_curve,
            killed_point,
        }
    }

    /// KEF — *kill edge, face*: the inverse of [`Body::mef`]. Kills
    /// `he`'s edge and **the face of `he`'s loop** (`he`'s side dies);
    /// the dying loop's remnant merges into the mate's loop.
    ///
    /// Which face dies is the argument's choice: a `mef` is undone by
    /// `kef(created.he_minus)`; passing the mate kills the other side.
    /// Full derivation, surgery diagram, and the degenerate circular/
    /// lone cases (each the inverse of a [`crate::MefSite`] case):
    /// [module docs](self). Self-loop edges (`start == end`) are legal
    /// here — they are the `Lone`-inverse territory.
    ///
    /// Euler vector: `(v 0, e −1, f −1, h 0, r 0, s 0)` — arena deltas
    /// −2 half-edges, −1 edge, −1 face, −1 loop.
    ///
    /// **Minting order**: nothing is minted. **Kill order** (D9, exact):
    /// `he`, its mate, the edge, the dying loop, the dying face (each
    /// with its provenance entry), then the edge's curve iff orphaned
    /// and the face's surface iff orphaned (usually shared, hence kept —
    /// module docs).
    ///
    /// **Re-anchoring** (unconditional, module docs): the surviving loop
    /// re-anchors at `next(mate)` when that survives, else `next(he)`;
    /// both endpoint vertices' `emanating` are rewritten to the derived
    /// survivors (or `None` in the `Lone` inverse, where the surviving
    /// loop becomes [`LoopBoundary::Empty`]).
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): the remnant's stored rows
    /// are curves stated in the DYING face's chart. Where the surviving
    /// face is on the same chart — one key, or two sharing one
    /// payload ([`Body::same_chart`]) — they stand; on any
    /// other chart the remnant's rows are DROPPED, for the reasons and
    /// with the consequences [`Body::drop_rows`] states. The surviving
    /// loop's own rows are untouched either way. Which is why the
    /// surviving face RESOLVES in the plan phase below, and the chart
    /// is decided there: it is what the mutation phase acts on, and a
    /// mutation phase reads nothing it has not proven.
    ///
    /// # Precondition check order
    ///
    /// `he` resolves ([`EulerOpError::StaleKey`]); its edge resolves
    /// (`StaleKey`) and claims it
    /// ([`EulerOpError::UnclaimedHalfEdge`]); the mate resolves
    /// (`StaleKey`); the two halves lie in distinct loops
    /// ([`EulerOpError::SameLoop`] — the same-loop configuration is
    /// [`Body::kemr`]'s); the dying loop resolves (`StaleKey`) and is
    /// a cycle ([`EulerOpError::LoopNotCycle`]), then the surviving
    /// loop likewise; the two loops lie on distinct faces
    /// ([`EulerOpError::SameFace`] — two loops of one face is what
    /// [`Body::kfmrh`] on adjacent faces leaves behind; kill such an
    /// edge with [`Body::kev`], or via [`Body::mfkrh`]-then-`kef` for
    /// the self-loop variant); the dying face resolves (`StaleKey`),
    /// then the surviving one; the dying face is ring-free
    /// ([`EulerOpError::FaceHasRings`]); its shell resolves
    /// (`StaleKey`); the dying loop's cycle closes
    /// ([`EulerOpError::LoopCycleBroken`]); `prev(he)` and the mate's
    /// `prev`/`next` resolve (`StaleKey` — `next(he)` is proven by the
    /// cycle walk); both endpoint vertices resolve (`StaleKey`); last,
    /// each endpoint's new `emanating` (the re-anchoring rule above)
    /// holds, `start(he)`'s and then `start(m)`'s: a `Some` starts at
    /// the endpoint, and `None` leaves it lone, with no half-edge but the
    /// killed two and the surviving loop `Empty` at it
    /// ([`EulerOpError::OrbitBroken`] naming `he`, then the mate —
    /// tier-1-invalid input: a torn `next` can put either anchor on
    /// another vertex, or land both steps on the killed halves at an
    /// endpoint that keeps edges, and a torn start can empty the loop at
    /// another vertex than the one it strands); then every remnant member claims the
    /// dying loop, and no half-edge but the remnant and `he` does
    /// ([`EulerOpError::LoopCycleBroken`] naming the dying loop —
    /// tier-1-invalid input: a torn `next` can divert its walk through
    /// another loop, whose members the move would take, or close it past
    /// a member, which the kill would leave naming a dead loop); then
    /// the surviving loop's new anchor holds: its `first` is not killed
    /// and lies in it once the remnant has moved in, and in the `Lone`
    /// inverse it keeps no member but the killed two and no other loop
    /// is `Empty` at its vertex (`LoopCycleBroken` naming the surviving
    /// loop — a torn `next(m)` can land on a killed half or in another
    /// loop, or read the mate as alone in a loop that keeps other
    /// members).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn kef(&mut self, he: HalfEdgeKey, tol: Tol) -> Result<KefResult, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let he_data = self.resolve_half_edge(he)?;
        let edge = he_data.edge;
        let edge_data = self.get_edge(edge).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let m = if edge_data.he_plus == he {
            edge_data.he_minus
        } else if edge_data.he_minus == he {
            edge_data.he_plus
        } else {
            return Err(EulerOpError::UnclaimedHalfEdge { he, edge });
        };
        let m_data = self.resolve_half_edge(m)?;
        let l1 = he_data.parent_loop; // dies with its face
        let l2 = m_data.parent_loop; // survives, absorbs the remnant
        if l1 == l2 {
            return Err(EulerOpError::SameLoop { r#loop: l1 });
        }
        let l1_data = self.get_loop(l1).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(l1),
        })?;
        if matches!(l1_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle { r#loop: l1 });
        }
        let f1 = l1_data.face; // dies
        let l2_data = self.get_loop(l2).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(l2),
        })?;
        if matches!(l2_data.boundary, LoopBoundary::Empty { .. }) {
            return Err(EulerOpError::LoopNotCycle { r#loop: l2 });
        }
        if l2_data.face == f1 {
            return Err(EulerOpError::SameFace { face: f1 });
        }
        let f1_data = self.get_face(f1).cloned().ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(f1),
        })?;
        // The surviving face is where the remnant lands, and its chart
        // decides the remnant's rows; a loop whose face does not
        // resolve is the tier-1 corruption the dying side is refused
        // for, in the same words.
        let f2 = l2_data.face;
        let f2_surface =
            self.get_face(f2)
                .map(|face| face.surface)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Face(f2),
                })?;
        // Decided here, where both keys resolve: the kills below may
        // reap the dying face's surface, and a decision carried out
        // of the plan phase has no order to keep against them.
        let remnant_changes_chart = !self.same_chart(f1_data.surface, f2_surface);
        if !f1_data.rings.is_empty() {
            return Err(EulerOpError::FaceHasRings { face: f1 });
        }
        let shell = f1_data.shell;
        require_key(&self.shells, shell, EntityId::Shell)?;
        // The dying loop's full cycle (bounded, D9): everything after he
        // is the remnant that moves to the mate's loop. The walk steps
        // `next` and resolves every member it returns, so it proves
        // them and nothing else — `prev/next` being mutual inverses is
        // a tier-1 fact, not one this call establishes.
        let cycle = self
            .loop_cycle_live(he)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop: l1 })?;
        let remnant: Vec<Live> = cycle.into_iter().skip(1).collect();
        // `b = next(he)` is the cycle's second member, so the walk
        // proved it and it wants no check of its own — and it is
        // `Option` rather than a key beside a `he_alone` flag because
        // the two are one fact: an empty remnant IS `next(he) == he`,
        // the one-half-edge dying loop, whose splice writes through no
        // `b` at all. `None` therefore means *the dying loop was [he]
        // alone*, and the arms that need `b` are exactly the arms that
        // have it.
        let b = remnant.first().copied();
        // The unsplice writes through the other three neighbor links,
        // each read straight out of the arena; prove them now so the
        // mutation below cannot fail midway (atomicity).
        let a = self.require_live(he_data.prev)?;
        let (c, d) = (
            self.require_live(m_data.prev)?,
            self.require_live(m_data.next)?,
        );
        let u = he_data.start;
        let w = m_data.start; // may equal u (self-loop edge)
        for vertex in [u, w] {
            require_key(&self.vertices, vertex, EntityId::Vertex)?;
        }
        // Emanating rule (unconditional, module docs): `u` takes
        // `next(m)`, its orbit step from `he`, and `w` takes `next(he)`,
        // its step from `m`; each falls back across, which on a valid
        // body happens only where `u == w`, else None. When `u == w` the
        // w-side write is last and wins (kemr's precedent). The plan
        // proves both writes (`Body::require_kill_anchors`).
        let survivor = |candidate: HalfEdgeKey, fallback: HalfEdgeKey| {
            if candidate != he && candidate != m {
                Some(candidate)
            } else if fallback != he && fallback != m {
                Some(fallback)
            } else {
                None
            }
        };
        // `next(he)` as a key: `he` itself in the absent case, which
        // `survivor` then rejects because `he` is reaped.
        let next_he = b.map_or(he, Live::key);
        let u_anchor = KillAnchor::step(survivor(d.key(), next_he));
        let w_anchor = KillAnchor::step(survivor(next_he, d.key()));
        // The splice arm, read from `b` and whether the mate's loop was
        // `[m]`, and the surviving loop's anchor it writes (unconditional
        // rule, module docs): `next(m)` where that survives, else the
        // remnant's first member, else `Empty` at `w`. The plan proves it
        // with the `emanating` writes, and the remnant it moves into `l2`.
        let splice = match (b, d.key() == m) {
            (None, true) => KefSplice::Lone,
            (None, false) => KefSplice::Unsplice,
            (Some(b), true) => KefSplice::MateAlone(b),
            (Some(b), false) => KefSplice::General(b),
        };
        let l2_boundary = match splice {
            KefSplice::Lone => LoopBoundary::Empty { vertex: w },
            KefSplice::MateAlone(b) => LoopBoundary::Cycle { first: b.key() },
            KefSplice::Unsplice | KefSplice::General(_) => LoopBoundary::Cycle { first: d.key() },
        };
        self.require_kill_anchors(
            &[(u, u_anchor, he), (w, w_anchor, m)],
            &[(l2, l2_boundary)],
            &[he, m],
            Some(KillRun {
                members: &remnant,
                from: l1,
                extent: RunExtent::Whole,
                into: KillInto::Kept(l2),
            }),
        )?;
        // The surviving loop as the splice leaves it, from its new
        // anchor: its own members from `next(m)` up to `m`, then the
        // remnant.
        let remnant_keys: Vec<HalfEdgeKey> = remnant.iter().map(|moved| moved.key()).collect();
        let rows = self.plan_moved_rows(
            &remnant_keys,
            !remnant_changes_chart,
            f2,
            |body| {
                let own: Vec<HalfEdgeKey> = if d.key() == m {
                    Vec::new()
                } else {
                    body.site_cycle_from(d.key(), l2)?
                        .into_iter()
                        .take_while(|&h| h != m)
                        .collect()
                };
                let mut site = body.site_face(
                    f2,
                    &[(
                        l2,
                        own.into_iter()
                            .chain(remnant_keys.iter().copied())
                            .map(SiteHalf::Existing)
                            .collect(),
                    )],
                    None,
                )?;
                site.moved = true;
                Ok(site)
            },
            tol,
        )?;

        // ---- Mutation (infallible from here on). ----
        // The remnant joins the mate's loop.
        for &moved in &remnant {
            let Some(half_edge) = self.get_half_edge_mut(moved.key()) else {
                unreachable!(
                    "kef: the remnant's members were resolved by the plan phase's bounded walk"
                )
            };
            half_edge.parent_loop = l2;
        }
        // The remnant's rows are stated in the dying face's chart, and
        // stand on the surviving face only where that is the same
        // chart (decided in the plan phase). The remnant is exactly
        // the set of half-edges whose `parent_loop` moved above, so it
        // is exactly what the surviving loop's walk
        // (`pcurves::loop_rows`) attributes to the moved half-edges
        // once spliced: that loop is its own survivors plus the
        // remnant, and its own rows are not this op's to touch.
        if remnant_changes_chart {
            self.drop_rows(remnant.iter().map(|moved| moved.key()));
        }
        crate::pcurves::apply_site_rows(self, rows, None);
        // Splice (derived as mef's exact inverse — module docs diagram).
        match splice {
            KefSplice::Lone => {}
            KefSplice::Unsplice => self.link_half_edges(c, d),
            KefSplice::MateAlone(b) => self.link_half_edges(a, b),
            KefSplice::General(b) => {
                self.link_half_edges(c, b);
                self.link_half_edges(a, d);
            }
        }
        let Some(loop_data) = self.get_loop_mut(l2) else {
            unreachable!("kef: `l2` resolved in the plan phase")
        };
        loop_data.boundary = l2_boundary;
        // Emanating: the anchors the plan phase derived and proved.
        let Some(vertex) = self.get_vertex_mut(u) else {
            unreachable!("kef: `u` resolved in the plan phase")
        };
        vertex.emanating = u_anchor.key();
        let Some(vertex) = self.get_vertex_mut(w) else {
            unreachable!("kef: `w` resolved in the plan phase")
        };
        vertex.emanating = w_anchor.key();
        // Kills, each with its provenance entry (kill order documented
        // above), then the shell's face list and orphaned geometry.
        self.half_edges.remove(he);
        self.half_edge_provenance.remove(he);
        self.half_edges.remove(m);
        self.half_edge_provenance.remove(m);
        self.edges.remove(edge);
        self.edge_provenance.remove(edge);
        self.loops.remove(l1);
        self.loop_provenance.remove(l1);
        self.faces.remove(f1);
        self.face_provenance.remove(f1);
        // Null-face record hygiene (M3 PR 1): a record never outlives
        // its face (crate::null).
        self.null_faces.remove(f1);
        let Some(shell_data) = self.get_shell_mut(shell) else {
            unreachable!("kef: the shell resolved in the plan phase; only `f1` is reaped above")
        };
        shell_data.faces.retain(|&face| face != f1);
        let killed_curve = self
            .remove_curve_if_orphaned(edge_data.curve)
            .then_some(edge_data.curve);
        // The curve hygiene above can itself reap f1's surface (a
        // killed curve's `Intersection`/`Seam` description can hold
        // the last reference — the issue #86 cascade); `f1_data`
        // resolved at entry, so a now-missing key means THIS call
        // removed it — report the kill through either door.
        let cascade_took_surface = self.get_surface(f1_data.surface).is_none();
        let killed_surface = (self.remove_surface_if_orphaned(f1_data.surface)
            || cascade_took_surface)
            .then_some(f1_data.surface);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                faces: -1,
                loops: -1,
                half_edges: -2,
                edges: -1,
                ..ArenaDelta::ZERO
            },
            "kef",
        );
        Ok(KefResult {
            killed_edge: edge,
            killed_he_plus: edge_data.he_plus,
            killed_he_minus: edge_data.he_minus,
            killed_face: f1,
            killed_loop: l1,
            killed_curve,
            killed_surface,
        })
    }

    /// MFKRH — *make face, kill ring–hole*: the inverse of
    /// [`Body::kfmrh`]. Promotes a ring (cycle or empty) to the outer
    /// loop of a NEW face in the same shell.
    ///
    /// **This op is also the book's cross-shell `lmfkrh`** (M3 PR 1 —
    /// the inverse *motion* of cross-shell [`Body::kfmrh`], serving
    /// ch. 14 `splitfinish` / ch. 15 `setopfinish` section-face
    /// promotion, M3 PRs 3 and 5): promoting a ring whose cycle is
    /// detached from its face's component **splits the shell's surface
    /// into two connected components** while the single shell entity
    /// remains — the M2-deferred multi-shell transient, tier-1 legal
    /// (component-aware E–P) and tier-2 refused until
    /// [`Body::movefac`] distributes the components into real shells.
    /// The shell-level split is deliberately movefac's job, not this
    /// op's: `mfkrh` cannot re-home the component without walking it,
    /// and pass 10 (edge-adjacency shell coherence) requires whole
    /// components to move together. For the split/boolean pipeline's
    /// section faces — where the promoted ring geometrically coincides
    /// with the remaining loop — pass [`FaceSurface::Inherit`] (same
    /// surface key as the demoting face; the pipeline re-plates the
    /// section plane immediately after) or `Shared` with the section
    /// plane's key.
    ///
    /// The promoted loop survives with its key and D5 birth record; the
    /// new face's surface comes from the [`FaceSurface`] spec (M2
    /// geometry policy, `crate::euler` module docs): `Inherit` shares
    /// the demoting face's surface, `New` mints (pass
    /// `Surface::Nurbs` for the honest "no description yet" state —
    /// exact restoration of `kfmrh`'s killed surface is impossible and
    /// NOT promised, [module docs](self)), `Shared` reuses an existing
    /// key.
    ///
    /// **Sense** ([`crate::Face::sense`]): `mfkrh` passes
    /// [`ParentSide::Against`], derived on the demoting face's chart
    /// and stated on any other ([`Body::resolve_face_surface`]).
    /// Promoting an [`LoopBoundary::Empty`] ring yields an
    /// **empty-outer face** — the `mvfs`-face shape, now
    /// operator-reachable inside a larger body.
    ///
    /// **Pcurve rows** ([`crate::pcurves`]): the promoted ring's stored
    /// rows are a curve stated in the DEMOTING face's chart. A spec on
    /// that chart ([`Body::same_chart`]) keeps them; any other surface
    /// DROPS them ([`Body::drop_loop_rows`]; [`Body::drop_rows`] states
    /// why). A new face's rows are the caller's to mint either way
    /// ([`crate::pcurves::mint_pcurves`]).
    ///
    /// Euler vector: `(v 0, e 0, f +1, h −1, r −1, s 0)` — arena delta
    /// +1 face (the "−1 ring" is the surviving loop's promotion, not a
    /// kill; genus is derived, not stored).
    ///
    /// **Minting order** (D9, exact): surface (only for
    /// [`FaceSurface::New`]), face. Nothing is killed.
    /// The new face is appended to the shell's face list; the ring
    /// leaves its former face's ring list (`retain`, order-preserving
    /// for the others).
    ///
    /// # Precondition check order
    ///
    /// The ring resolves ([`EulerOpError::StaleKey`]); its face resolves
    /// (`StaleKey`); it is not that face's outer loop
    /// ([`EulerOpError::RingIsOuter`]); the face's shell resolves
    /// (`StaleKey`); a [`FaceSurface::Shared`] key resolves
    /// ([`EulerOpError::StaleGeometry`]); a stated sense agrees with
    /// the derived one on the demoting face's chart
    /// ([`EulerOpError::SenseContradictsChart`]).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mfkrh(
        &mut self,
        ring: LoopKey,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MfkrhCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        // ---- Preconditions: no mutation until every check passes. ----
        let ring_data = self.get_loop(ring).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(ring),
        })?;
        let old_face = ring_data.face;
        let old_face_data = self.get_face(old_face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(old_face),
        })?;
        if old_face_data.outer == ring {
            return Err(EulerOpError::RingIsOuter { r#loop: ring });
        }
        let shell = old_face_data.shell;
        let (inherit_surface, inherit_sense) = (old_face_data.surface, old_face_data.sense);
        require_key(&self.shells, shell, EntityId::Shell)?;
        // The ring was wound clockwise about the parent's outward
        // normal (interior-left), and as an outer loop it winds
        // counter-clockwise about the new face's, so on the parent's
        // chart the new face takes the parent's bit negated.
        let resolved = self.resolve_face_surface(
            &surface,
            old_face,
            (inherit_surface, inherit_sense),
            ParentSide::Against,
        )?;
        let ring_halves = self.site_cycle(ring)?;
        let rows = self.plan_moved_rows(
            &ring_halves,
            resolved.on_parent_chart,
            old_face,
            |body| {
                body.new_site_face(
                    old_face,
                    &surface,
                    true,
                    ring_halves
                        .iter()
                        .copied()
                        .map(SiteHalf::Existing)
                        .collect(),
                )
            },
            tol,
        )?;

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented above): surface (for New), face.
        let surface = self.mint_face_surface(surface, inherit_surface);
        let face = self.add_face(
            Face {
                sense: resolved.sense,
                surface,
                outer: ring,
                rings: vec![],
                shell,
            },
            Provenance::Mfkrh { ring },
        );
        let Some(old) = self.get_face_mut(old_face) else {
            unreachable!("mfkrh: the old face resolved in the plan phase")
        };
        old.rings.retain(|&l| l != ring);
        let Some(loop_data) = self.get_loop_mut(ring) else {
            unreachable!("mfkrh: the ring resolved in the plan phase")
        };
        loop_data.face = face;
        if !resolved.on_parent_chart {
            self.drop_loop_rows(ring);
        }
        crate::pcurves::apply_site_rows(self, rows, None);
        let Some(shell_data) = self.get_shell_mut(shell) else {
            unreachable!("mfkrh: the shell resolved in the plan phase")
        };
        shell_data.faces.push(face);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                faces: 1,
                ..ArenaDelta::ZERO
            },
            "mfkrh",
        );
        Ok(MfkrhCreated { face, surface })
    }

    /// [`Body::mfkrh`] with [`FaceSurface::New`] holding the
    /// `Surface::Nurbs` placeholder — the new face is born with the
    /// honest "no description yet" surface (`crate::euler`'s geometry
    /// policy; replace it via [`Body::set_face_surface`] before rest).
    /// Mirrors the M1 fresh-surface semantics for the migrated suites
    /// and for promotions whose real surface is not yet known.
    ///
    /// The placeholder is a fresh payload, so it is never the demoting
    /// face's chart: `mfkrh` writes `sense`, the caller's provisional
    /// bit, as stated, and drops the promoted ring's rows. The caller
    /// states the honest bit, and mints the rows, when it gives the
    /// face a real surface.
    ///
    /// # Errors
    ///
    /// As [`Body::mfkrh`].
    pub fn mfkrh_plug(
        &mut self,
        ring: LoopKey,
        sense: bool,
        tol: Tol,
    ) -> Result<MfkrhCreated, EulerOpError> {
        self.mfkrh(
            ring,
            FaceSurface::New {
                surface: geom::Surface::nurbs_placeholder(),
                sense,
            },
            tol,
        )
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::Tol;

    use super::*;
    use crate::entity::{Edge, HalfEdge, Loop, Shell, Vertex};
    use crate::euler::{MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
    use crate::fixtures::{
        ArenaSnapshot, arena_snapshot, assert_err_deep_unchanged, assert_kill_refuses,
        deep_snapshot, ops_holed_box, prov,
    };
    use crate::iso::{canonical_form, isomorphic};
    use crate::readback::euler_counts;
    use crate::test_support_fixtures::declined_cube;
    use crate::test_support_impl::ArenaCounts;
    use crate::validate::validate;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    /// [`crate::fixtures::ops_segment`] at the witness tol.
    fn segment() -> (Body<f64>, MvfsCreated, MevCreated) {
        crate::fixtures::ops_segment(Tol::witness())
    }

    /// [`crate::fixtures::ops_strutted`] at the witness tol.
    fn strutted() -> (Body<f64>, MvfsCreated, MevCreated, MevCreated) {
        crate::fixtures::ops_strutted(Tol::witness())
    }

    // ------------------------------------------------------------------
    // kvfs
    // ------------------------------------------------------------------

    #[test]
    fn kvfs_inverts_mvfs_to_the_empty_body() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(1.0, 2.0, 3.0), true).unwrap();
        let result = body.kvfs(seed.solid).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // E–P vector (−1, 0, −1, 0, 0, −1): everything is gone.
        assert_eq!(arena_snapshot(&body), ArenaSnapshot::default());
        // Killed keys are exactly mvfs's mints, and they are dead.
        assert_eq!(result.killed_solid, seed.solid);
        assert_eq!(result.killed_shell, seed.shell);
        assert_eq!(result.killed_face, seed.face);
        assert_eq!(result.killed_loop, seed.r#loop);
        assert_eq!(result.killed_vertex, seed.vertex);
        assert_eq!(result.killed_surface, Some(seed.surface));
        assert_eq!(result.killed_point, Some(seed.point));
        assert!(body.get_solid(seed.solid).is_none());
        assert!(body.get_vertex(seed.vertex).is_none());
        // Kill hygiene: provenance entries died with their entities.
        for id in [
            EntityId::Solid(seed.solid),
            EntityId::Shell(seed.shell),
            EntityId::Face(seed.face),
            EntityId::Loop(seed.r#loop),
            EntityId::Vertex(seed.vertex),
        ] {
            assert_eq!(body.provenance(id), None, "for {id}");
        }
    }

    #[test]
    fn kvfs_roundtrips_with_mvfs_beside_another_solid() {
        // mvfs ∘ kvfs in a body that also holds an unrelated solid: the
        // bystander is untouched (deep) and the pair restores the
        // canonical form.
        let (mut body, _, _) = segment();
        let before = canonical_form(&body);
        let seed = body.mvfs(p(9.0), true).unwrap();
        body.kvfs(seed.solid).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kvfs_precondition_errors_are_atomic() {
        // Stale solid.
        let (mut body, seed, seg) = segment();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::Solid(SolidKey::default()),
            },
            |b| b.kvfs(SolidKey::default()).unwrap_err(),
        );
        // Grown solid: the outer loop is a cycle, not empty.
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::LoopNotEmpty {
                r#loop: seed.r#loop,
            },
            |b| b.kvfs(seed.solid).unwrap_err(),
        );
        // Two faces: kill the edge back and split instead.
        body.kev(seg.he_plus).unwrap();
        body.mef_chord(
            MefSite::Lone {
                r#loop: seed.r#loop,
            },
            Tol::witness(),
        )
        .unwrap();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::ShellNotSingleFace {
                shell: seed.shell,
                faces: 2,
            },
            |b| b.kvfs(seed.solid).unwrap_err(),
        );
    }

    #[test]
    fn kvfs_rejects_extra_shells_and_rings() {
        // Extra shell, raw-built: a legal multi-shell solid, which
        // `kvfs` still refuses because it unmakes the seed shell only.
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let extra = body.add_shell(
            Shell {
                faces: vec![],
                solid: seed.solid,
            },
            prov(),
        );
        body.get_solid_mut(seed.solid).unwrap().shells.push(extra);
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::SolidNotSingleShell {
                solid: seed.solid,
                shells: 2,
            },
            |b| b.kvfs(seed.solid).unwrap_err(),
        );
        // A ring on the single face (raw-built empty ring).
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let point = body.add_point(p(1.0));
        let vertex = body.add_vertex(
            Vertex {
                point,
                emanating: None,
            },
            prov(),
        );
        let ring = body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex },
                face: seed.face,
            },
            prov(),
        );
        body.get_face_mut(seed.face).unwrap().rings.push(ring);
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::FaceHasRings { face: seed.face },
            |b| b.kvfs(seed.solid).unwrap_err(),
        );
    }

    // ------------------------------------------------------------------
    // kev: segment / strut / general fan merge / two loops / mirror
    // ------------------------------------------------------------------

    #[test]
    fn kev_segment_kill_inverts_lone_mev() {
        let (mut body, seed, seg) = segment();
        let skeletal = {
            let mut fresh = Body::<f64>::new();
            fresh.mvfs(p(0.0), true).unwrap();
            canonical_form(&fresh)
        };
        let before = arena_snapshot(&body);
        let result = body.kev(seg.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // E–P vector (−1, −1, 0, 0, 0, 0): −1 vertex, −1 edge, −2 halves
        // (plus the reaped point and curve).
        let after = arena_snapshot(&body);
        assert_eq!(
            after,
            ArenaSnapshot {
                counts: ArenaCounts {
                    half_edges: before.counts.half_edges - 2,
                    edges: before.counts.edges - 1,
                    vertices: before.counts.vertices - 1,
                    ..before.counts
                },
                points: before.points - 1,
                curves: before.curves - 1,
                ..before
            }
        );
        // The far vertex died; the loop is empty at the survivor; the
        // survivor is lone again (emanating None).
        assert_eq!(result.killed_vertex, seg.vertex);
        assert_eq!(result.killed_edge, seg.edge);
        assert_eq!(result.killed_he_plus, seg.he_plus);
        assert_eq!(result.killed_he_minus, seg.he_minus);
        assert_eq!(result.killed_curve, Some(seg.curve));
        assert_eq!(result.killed_point, Some(seg.point));
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: seed.vertex
            }
        );
        assert_eq!(body.get_vertex(seed.vertex).unwrap().emanating, None);
        // Kill hygiene: dead keys, dead provenance.
        for id in [
            EntityId::Vertex(seg.vertex),
            EntityId::Edge(seg.edge),
            EntityId::HalfEdge(seg.he_plus),
            EntityId::HalfEdge(seg.he_minus),
        ] {
            assert_eq!(body.provenance(id), None, "for {id}");
        }
        assert!(body.get_edge(seg.edge).is_none());
        // The remnant is exactly the skeletal mvfs body.
        assert_eq!(canonical_form(&body), skeletal);
        // Hand kill∘make roundtrip: mev(Lone) at the same coordinates
        // restores the segment body up to isomorphism.
        let with_segment = {
            let (fresh, _, _) = segment();
            canonical_form(&fresh)
        };
        body.mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            p(1.0),
            Tol::witness(),
        )
        .unwrap();
        assert_eq!(canonical_form(&body), with_segment);
    }

    #[test]
    fn kev_kills_the_vertex_the_argument_points_at() {
        // The direction pin: kev(he_minus) kills end(he_minus) — the OLD
        // vertex — leaving the new one as the lone survivor.
        let (mut body, seed, seg) = segment();
        let result = body.kev(seg.he_minus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_vertex, seed.vertex);
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty { vertex: seg.vertex }
        );
        assert!(body.get_vertex(seed.vertex).is_none());
        assert!(body.get_vertex(seg.vertex).is_some());
    }

    #[test]
    fn kev_strut_kill_is_pure_removal() {
        let (mut body, seed, seg, strut) = strutted();
        let before_strut = {
            let (fresh, _, _) = segment();
            canonical_form(&fresh)
        };
        let result = body.kev(strut.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));

        assert_eq!(result.killed_vertex, strut.vertex);
        // The loop is the plain segment cycle again, re-anchored at the
        // first survivor after the killed pair (next(mate) =
        // seg.he_minus).
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: seg.he_minus
            }
        );
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![seg.he_plus, seg.he_minus])
        );
        // Emanating rule, strut case: the survivor anchors at next(mate)
        // (which starts at it).
        assert_eq!(
            body.get_vertex(seg.vertex).unwrap().emanating,
            Some(seg.he_minus)
        );
        assert_eq!(body.vertex_orbit(seg.he_minus), Some(vec![seg.he_minus]));
        // make ∘ kill: the strut mev is exactly undone.
        assert_eq!(canonical_form(&body), before_strut);
    }

    /// PR 2's valence-4 star: central vertex `v` with clockwise orbit
    /// `[a+, b+, c+, d+]`.
    fn four_spoke_star() -> (Body<f64>, MvfsCreated, [MevCreated; 4]) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let a = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
                Tol::witness(),
            )
            .unwrap();
        let strut_at = |body: &mut Body<f64>, x: f64| {
            body.mev_line(
                MevSite::Fan {
                    he1: a.he_plus,
                    he2: a.he_plus,
                },
                p(x),
                Tol::witness(),
            )
            .unwrap()
        };
        let b = strut_at(&mut body, 2.0);
        let c = strut_at(&mut body, 3.0);
        let d = strut_at(&mut body, 4.0);
        assert_eq!(
            body.vertex_orbit(a.he_plus),
            Some(vec![a.he_plus, b.he_plus, c.he_plus, d.he_plus])
        );
        (body, seed, [a, b, c, d])
    }

    #[test]
    fn kev_general_case_merges_the_fan_back() {
        // Inverse of THE direction-pinning mev test: split the valence-4
        // fan, then kev the new edge — the moved run {b+, c+} must come
        // back to v, and the orbit must be the original four spokes.
        let (mut body, seed, [a, b, c, d]) = four_spoke_star();
        let before = canonical_form(&body);
        let site = MevSite::Fan {
            he1: b.he_plus,
            he2: d.he_plus,
        };
        // The certified door first, as the `mev` pins do: at p(5.0) it
        // refuses to move a spoke onto a vertex its chord does not run
        // to (`euler`'s re-basing gate). So the split runs through the
        // coincident door, and the body this row kills on carries a
        // NULL edge — tier-1 valid, and tier 2 refuses it at rest
        // ([`crate::ValidationError::NullEdgeAtRest`]). The merge back
        // is this test's subject, not the split.
        assert!(matches!(
            body.clone().mev_line(site, p(5.0), Tol::witness()),
            Err(EulerOpError::RebasedCarrier { edge, .. }) if edge == b.edge
        ));
        let split = body.mev_null(site, crate::NewVertexSide::Above).unwrap();
        let result = body.kev(split.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));

        let v = seed.vertex;
        assert_eq!(result.killed_vertex, split.vertex);
        let start = |body: &Body<f64>, he| body.get_half_edge(he).unwrap().start;
        for spoke in [a.he_plus, b.he_plus, c.he_plus, d.he_plus] {
            assert_eq!(start(&body, spoke), v);
        }
        // Emanating rule, fan case: the first merged-fan member — the
        // clockwise-first half the killed plus half pointed at (b+).
        assert_eq!(body.get_vertex(v).unwrap().emanating, Some(b.he_plus));
        // Orbit closes over exactly the four spokes.
        let orbit = body.vertex_orbit(a.he_plus).unwrap();
        assert_eq!(orbit.len(), 4);
        // make ∘ kill: exact canonical restoration.
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kev_unsplices_across_two_loops() {
        // Inverse of PR 2's two-loop fan mev (the digon pillow's seed
        // vertex): he_plus and he_minus land in different loops; kev
        // must unsplice each from its own loop.
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
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
        let before = canonical_form(&body);
        let site = MevSite::Fan {
            he1: seg.he_plus,
            he2: split.he_plus,
        };
        // The certified door first, as in the valence-4 row: p(2.0) is
        // a vertex `seg`'s chord does not run to, so the splice is
        // pinned through the coincident door and the killed-on body
        // carries a null edge (tier-1 valid, tier-2 refused at rest).
        assert!(matches!(
            body.clone().mev_line(site, p(2.0), Tol::witness()),
            Err(EulerOpError::RebasedCarrier { edge, .. }) if edge == seg.edge
        ));
        let fan = body.mev_null(site, crate::NewVertexSide::Above).unwrap();
        // The new halves landed in different loops (pinned by PR 2's
        // test); now undo.
        let result = body.kev(fan.he_plus).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_vertex, fan.vertex);
        assert_eq!(
            body.get_half_edge(seg.he_plus).unwrap().start,
            seed.vertex,
            "the moved run returned to the old vertex"
        );
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kev_mirror_case_merges_fan_onto_the_valence_one_survivor() {
        // kev from the tip side: he = strut.he_minus starts at the
        // valence-1 tip and points at the fan-carrying vertex. The fan
        // migrates to the tip. (Documented asymmetry: THIS kill has no
        // single-op re-make — the re-making mev would have to start
        // the migrated fan at a vertex its chords do not run to, which
        // `mev`'s re-basing gate refuses — but the result must still be
        // tier-1 valid.) The migrated chord would miss its new end, so
        // the merge is the describing kill's, handed the chord it runs
        // along now; the row's subject is the merge's topology.
        let (mut body, seed, seg, strut) = strutted();
        let chord = EdgeCurveSpec::line_between(p(0.0), p(2.0));
        let result = body
            .kev_describing(strut.he_minus, &[(seg.edge, chord)], Tol::witness())
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_vertex, seg.vertex);
        // The whole old fan of seg.vertex (seg.he_minus) now starts at
        // the strut tip.
        assert_eq!(
            body.get_half_edge(seg.he_minus).unwrap().start,
            strut.vertex
        );
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![seg.he_plus, seg.he_minus])
        );
        // The segment now runs seed.vertex → strut.vertex.
        assert_eq!(body.get_half_edge(seg.he_plus).unwrap().start, seed.vertex);
        assert_eq!(body.half_edge_end(seg.he_plus), Some(strut.vertex));
    }

    #[test]
    fn a_merge_that_would_strand_a_carrier_refuses_keys_only_and_the_describing_kill_leaves_it_splittable()
     {
        // The mirror merge moves `seg`'s far end from `(1,0,0)` to the
        // strut tip `(2,0,0)` while its chord still runs to `(1,0,0)`:
        // the state `split_edge` refuses on, since it certifies both
        // children against the CURRENT endpoints. The keys-only kill
        // refuses it typed, naming the member, body untouched; so does
        // the describing kill that is handed a band and nothing to
        // write, through the re-basing gate. Handed the chord `seg` runs
        // along after the merge, it merges, certifies, and `split_edge`
        // runs on the merged edge.
        let tol = Tol::witness();
        let (mut body, _seed, seg, strut) = strutted();
        let before = deep_snapshot(&body);
        assert_eq!(
            body.kev(strut.he_minus).map(|_| ()),
            Err(EulerOpError::MergeRebasesCarriers {
                edges: vec![seg.edge]
            })
        );
        assert_eq!(deep_snapshot(&body), before);
        let err = body
            .kev_describing(strut.he_minus, &[], tol)
            .map(|_| ())
            .unwrap_err();
        assert!(
            matches!(err, EulerOpError::RebasedCarrier { edge, .. } if edge == seg.edge),
            "{err:?}"
        );
        assert_eq!(deep_snapshot(&body), before);

        let chord = EdgeCurveSpec::line_between(p(0.0), p(2.0));
        body.kev_describing(strut.he_minus, &[(seg.edge, chord)], tol)
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        let curve = body.get_edge(seg.edge).unwrap().curve;
        let certified = body.get_curve_geom(curve).unwrap().certified().unwrap();
        let (t0, t1) = certified.params();
        assert_eq!(
            certified.carrier().eval(t1).x,
            2.0,
            "the chord runs to the tip"
        );
        let split = body
            .split_edge(seg.edge, 0.5f64.mul_add(t1 - t0, t0), tol)
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            body.get_point(body.get_vertex(split.vertex).unwrap().point)
                .map(|q| q.x),
            Some(1.0),
            "the split lands on the merged chord's midpoint"
        );
    }

    /// A merged member the caller re-describes, certified against the
    /// merged endpoints; an unlisted member through the re-basing gate.
    /// The strutted segment with a second strut at the far vertex:
    /// killing the first strut from its tip merges `seg` and `other`
    /// onto the tip.
    fn two_member_merge() -> (Body<f64>, MevCreated, MevCreated, MevCreated) {
        let (mut body, _seed, seg, strut) = strutted();
        let other = body
            .mev_line(
                MevSite::Fan {
                    he1: strut.he_plus,
                    he2: strut.he_plus,
                },
                Point3::new(1.0, 1.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        (body, seg, strut, other)
    }

    #[test]
    fn kev_names_every_certified_member_in_orbit_order() {
        let (mut body, seg, strut, other) = two_member_merge();
        let before = deep_snapshot(&body);
        let err = body.kev(strut.he_minus).map(|_| ()).unwrap_err();
        let EulerOpError::MergeRebasesCarriers { edges } = err else {
            panic!("{err:?}")
        };
        let mut sorted = edges.clone();
        sorted.sort();
        let mut expected = vec![seg.edge, other.edge];
        expected.sort();
        assert_eq!(sorted, expected, "both members are named");
        let orbit = body.vertex_orbit(strut.he_plus).unwrap();
        let in_orbit: Vec<EdgeKey> = orbit[1..]
            .iter()
            .map(|&h| body.get_half_edge(h).unwrap().edge)
            .collect();
        assert_eq!(edges, in_orbit, "named in the dying vertex's orbit order");
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn kev_describing_refuses_a_listed_edge_that_is_not_a_merged_member() {
        // The killed edge itself, and an edge at the surviving end of
        // `seg`, which the merge does not touch.
        let (mut body, seg, strut, _other) = two_member_merge();
        let far = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: seg.he_plus,
                },
                Point3::new(0.0, -1.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        let before = deep_snapshot(&body);
        for stranger in [strut.edge, far.edge] {
            assert_eq!(
                body.kev_describing(
                    strut.he_minus,
                    &[(stranger, EdgeCurveSpec::line_between(p(0.0), p(9.0)))],
                    Tol::witness(),
                )
                .map(|_| ()),
                Err(EulerOpError::NotMergedMember { edge: stranger })
            );
            assert_eq!(deep_snapshot(&body), before);
        }
    }

    #[test]
    fn kev_describing_refuses_a_member_listed_twice() {
        let (mut body, seg, strut, _other) = two_member_merge();
        let before = deep_snapshot(&body);
        let chord = EdgeCurveSpec::line_between(p(0.0), p(2.0));
        assert_eq!(
            body.kev_describing(
                strut.he_minus,
                &[(seg.edge, chord.clone()), (seg.edge, chord)],
                Tol::witness(),
            )
            .map(|_| ()),
            Err(EulerOpError::DuplicateRedescription { edge: seg.edge })
        );
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn kev_describing_refuses_a_spec_that_does_not_certify_at_the_merged_endpoints() {
        // `seg`'s spec is the chord it had — to the dying vertex's
        // point, which is not where the merge ends it. Named at the
        // listed edge, body untouched.
        let (mut body, seg, strut, other) = two_member_merge();
        let before = deep_snapshot(&body);
        let tip_to_other = EdgeCurveSpec::line_between(p(2.0), Point3::new(1.0, 1.0, 0.0));
        let err = body
            .kev_describing(
                strut.he_minus,
                &[
                    (other.edge, tip_to_other),
                    (seg.edge, EdgeCurveSpec::line_between(p(0.0), p(1.0))),
                ],
                Tol::witness(),
            )
            .map(|_| ())
            .unwrap_err();
        assert!(
            matches!(
                err,
                EulerOpError::RebasedCarrier {
                    edge,
                    error: geom_brep::CertifyError::ResidualExceeded { .. },
                } if edge == seg.edge
            ),
            "{err:?}"
        );
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn kev_describing_refuses_a_spec_whose_description_is_not_the_members() {
        // A chart image must name one of the member's own faces'
        // surfaces: the attachment gate's adjacency rule, asked in the
        // plan phase against the faces the merge leaves the member on
        // (the kill moves no half-edge between loops). The chart named
        // is a live surface — another solid's face's — so what refuses
        // is whose it is, not that it does not resolve.
        let (mut body, seg, strut, _other) = two_member_merge();
        let elsewhere = body.mvfs(Point3::new(9.0, 9.0, 9.0), true).unwrap();
        let own = body
            .get_face(body.face_of_half_edge(seg.he_plus).unwrap())
            .unwrap()
            .surface;
        assert_ne!(elsewhere.surface, own);
        assert!(body.get_surface(elsewhere.surface).is_some());
        let before = deep_snapshot(&body);
        let mut spec = EdgeCurveSpec::line_between(p(0.0), p(2.0));
        spec.description = geom_brep::EdgeDescriptionSpec::chart(elsewhere.surface);
        assert_eq!(
            body.kev_describing(strut.he_minus, &[(seg.edge, spec)], Tol::witness())
                .map(|_| ()),
            Err(EulerOpError::DescriptionNotAdjacent { edge: seg.edge })
        );
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn kev_describing_refuses_an_unlisted_member_the_gate_refuses() {
        // Only `seg` is re-described; `other` keeps a chord to the dying
        // vertex's point and the re-basing gate names it.
        let (mut body, seg, strut, other) = two_member_merge();
        let before = deep_snapshot(&body);
        let err = body
            .kev_describing(
                strut.he_minus,
                &[(seg.edge, EdgeCurveSpec::line_between(p(0.0), p(2.0)))],
                Tol::witness(),
            )
            .map(|_| ())
            .unwrap_err();
        assert!(
            matches!(err, EulerOpError::RebasedCarrier { edge, .. } if edge == other.edge),
            "{err:?}"
        );
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn kev_describing_writes_every_listed_member_and_reaps_what_it_replaced() {
        let tol = Tol::witness();
        let (mut body, seg, strut, other) = two_member_merge();
        let old = [
            body.get_edge(seg.edge).unwrap().curve,
            body.get_edge(other.edge).unwrap().curve,
        ];
        let curves_before = body.curves().count();
        body.kev_describing(
            strut.he_minus,
            &[
                (
                    other.edge,
                    EdgeCurveSpec::line_between(p(2.0), Point3::new(1.0, 1.0, 0.0)),
                ),
                (seg.edge, EdgeCurveSpec::line_between(p(0.0), p(2.0))),
            ],
            tol,
        )
        .unwrap();
        assert_eq!(validate(&body), Ok(()));
        // The killed edge's curve and both replaced curves are reaped;
        // two are minted.
        assert_eq!(body.curves().count(), curves_before - 1);
        for (edge, was) in [(seg.edge, old[0]), (other.edge, old[1])] {
            let now = body.get_edge(edge).unwrap().curve;
            assert_ne!(now, was);
            assert!(
                body.get_curve_geom(was).is_none(),
                "the old curve is reaped"
            );
            assert!(carrier_is_coherent_here(&body, edge, tol));
        }
    }

    #[test]
    fn kev_merged_members_reads_what_the_describing_door_certifies_against() {
        // The members in orbit order, each with the endpoints the merge
        // gives it; handed back as chords, the describing kill takes
        // them. A strut kill merges nothing.
        let tol = Tol::witness();
        let (mut body, seg, strut, other) = two_member_merge();
        let members = body.kev_merged_members(strut.he_minus).unwrap();
        let orbit = body.vertex_orbit(strut.he_plus).unwrap();
        let in_orbit: Vec<EdgeKey> = orbit[1..]
            .iter()
            .map(|&h| body.get_half_edge(h).unwrap().edge)
            .collect();
        assert_eq!(members.iter().map(|m| m.edge).collect::<Vec<_>>(), in_orbit);
        let tip = [2.0, 0.0, 0.0];
        let xyz = |q: Point3<f64>| [q.x, q.y, q.z];
        for m in &members {
            let (start, end) = (xyz(m.start), xyz(m.end));
            if m.edge == seg.edge {
                assert_eq!((start, end), ([0.0; 3], tip), "seg now ends at the tip");
            } else {
                assert_eq!(m.edge, other.edge);
                assert_eq!(
                    (start, end),
                    (tip, [1.0, 1.0, 0.0]),
                    "other now starts there"
                );
            }
        }
        assert!(
            body.kev_merged_members(strut.he_plus).unwrap().is_empty(),
            "the strut kill from the far end merges nothing"
        );
        let chords: Vec<_> = members
            .iter()
            .map(|m| (m.edge, EdgeCurveSpec::line_between(m.start, m.end)))
            .collect();
        body.kev_describing(strut.he_minus, &chords, tol).unwrap();
        assert_eq!(validate(&body), Ok(()));
    }

    #[test]
    fn a_torn_orbit_through_the_killed_half_refuses_typed_in_both_doors() {
        // Two `next` tears walk the dying vertex's orbit through the
        // killed half itself: [seg−, strut+, seg+]. Unchecked, the
        // killed edge is its own merged member, the keys-only door
        // names it, and the describing door certifies it, kills it and
        // then has it to write. The plan phase sees that `seg+` does not
        // start at the dying vertex and both doors, and the read door,
        // refuse typed, body untouched.
        let tol = Tol::witness();
        let (mut body, _seed, seg, strut) = strutted();
        body.get_half_edge_mut(strut.he_minus).unwrap().next = seg.he_plus;
        body.get_half_edge_mut(seg.he_minus).unwrap().next = seg.he_minus;
        assert_eq!(
            body.vertex_orbit(seg.he_minus),
            Some(vec![seg.he_minus, strut.he_plus, seg.he_plus])
        );
        let torn = EulerOpError::OrbitBroken { he: seg.he_minus };
        let he = seg.he_plus;
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kev(he).unwrap_err());
        assert_err_deep_unchanged(&mut body, &torn, |b| {
            b.kev_describing(he, &[], tol).unwrap_err()
        });
        let chord = EdgeCurveSpec::line_between(p(0.0), p(2.0));
        assert_err_deep_unchanged(&mut body, &torn, |b| {
            b.kev_describing(he, &[(seg.edge, chord)], tol).unwrap_err()
        });
        assert_eq!(body.kev_merged_members(he).map(|_| ()), Err(torn));
    }

    #[test]
    fn a_twice_torn_declined_cube_refuses_typed_in_both_doors() {
        // The first counterexample a seeded search of two random
        // `next` tears on the declined cube found (22 of 96,000 kill
        // calls reached the describing door's write with the killed
        // edge listed): the tears and the kill, by position in the
        // fixture's half-edge arena. The orbit leaves the dying vertex,
        // and both doors refuse it in the plan phase.
        let tol = Tol::witness();
        let mut body = declined_cube::<f64>(tol).body;
        let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
        body.get_half_edge_mut(halves[8]).unwrap().next = halves[18];
        body.get_half_edge_mut(halves[16]).unwrap().next = halves[23];
        let he = halves[16];
        let m = body.mate(he).unwrap();
        let torn = EulerOpError::OrbitBroken { he: m };
        let killed = body.get_half_edge(he).unwrap().edge;
        let orbit = body.vertex_orbit(m).unwrap();
        assert!(
            orbit[1..]
                .iter()
                .any(|&h| body.get_half_edge(h).unwrap().edge == killed),
            "the torn orbit walks the killed edge back into its own fan"
        );
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kev(he).unwrap_err());
        let chords: Vec<_> = orbit[1..]
            .iter()
            .map(|&h| body.get_half_edge(h).unwrap().edge)
            .fold(Vec::new(), |mut edges, e| {
                if !edges.contains(&e) {
                    edges.push(e);
                }
                edges
            })
            .into_iter()
            .map(|e| (e, EdgeCurveSpec::line_between(p(0.0), p(1.0))))
            .collect();
        assert_err_deep_unchanged(&mut body, &torn, |b| {
            b.kev_describing(he, &chords, tol).unwrap_err()
        });
    }

    #[test]
    fn kev_describing_asks_the_survivors_point_only_where_a_question_needs_it() {
        // A strut kill from the tip's far end merges nothing; with the
        // surviving vertex's point gone, the describing door asks what
        // its docs list in their order: a listed non-member refuses as
        // one, and an empty list asks nothing past the structural list,
        // as the keys-only door asks nothing, so both kill. (The body is
        // tier-1-invalid, so the kills run inside a surgery scope,
        // whose close is dropped unswept.)
        let tol = Tol::witness();
        let (mut body, _seed, seg, strut) = strutted();
        let v = body.get_half_edge(strut.he_plus).unwrap().start;
        let point = body.get_vertex(v).unwrap().point;
        body.points.remove(point);
        assert!(body.kev_merged_members(strut.he_plus).unwrap().is_empty());
        let stranger = EdgeCurveSpec::line_between(p(0.0), p(9.0));
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::NotMergedMember { edge: seg.edge },
            |b| {
                b.kev_describing(strut.he_plus, &[(seg.edge, stranger)], tol)
                    .unwrap_err()
            },
        );
        for describing in [false, true] {
            let mut copy = body.clone();
            let mut scope = copy.begin_surgery();
            let got = if describing {
                scope.kev_describing(strut.he_plus, &[], tol)
            } else {
                scope.kev(strut.he_plus)
            };
            drop(scope);
            assert!(got.is_ok(), "describing: {describing}: {got:?}");
        }
    }

    #[test]
    fn both_doors_carry_a_killed_null_edge_whose_fan_holds_one_half_of_another() {
        // `n1` joins `seg`'s far vertex to `w` at one point, and `n2`
        // leaves `w` at that point too. Killing `n1` toward `w` merges
        // one half of `n2` onto the survivor: nothing moves, so neither
        // door asks the re-basing gate, whose null arm would refuse the
        // one half.
        let tol = Tol::witness();
        let (mut body, _seed, seg) = segment();
        let n1 = body
            .mev_null(
                MevSite::Fan {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                crate::NewVertexSide::Above,
            )
            .unwrap();
        let at_w = if body.get_half_edge(n1.he_plus).unwrap().start == n1.vertex {
            n1.he_plus
        } else {
            n1.he_minus
        };
        let n2 = body
            .mev_null(
                MevSite::Fan {
                    he1: at_w,
                    he2: at_w,
                },
                crate::NewVertexSide::Above,
            )
            .unwrap();
        let toward_w = body.mate(at_w).unwrap();
        assert_eq!(body.half_edge_end(toward_w), Some(n1.vertex));
        let members = body.kev_merged_members(toward_w).unwrap();
        assert_eq!(
            members.iter().map(|m| m.edge).collect::<Vec<_>>(),
            [n2.edge]
        );
        assert_eq!(body.clone().kev(toward_w).map(|_| ()), Ok(()));
        body.kev_describing(toward_w, &[], tol).unwrap();
        assert_eq!(validate(&body), Ok(()));
    }

    #[test]
    fn a_certified_self_loop_member_is_named_once() {
        // A self-loop circle at the dying vertex has both halves in the
        // merged fan; it is one member, named once by the refusal and
        // read once by the read door.
        let tol = Tol::witness();
        let (mut body, _seed, seg) = segment();
        let circle = body
            .mef(
                MefSite::Chords {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                EdgeCurveSpec::self_loop_circle_at(p(1.0)),
                crate::euler::FaceSurface::Inherit,
                tol,
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            body.kev_merged_members(seg.he_plus)
                .unwrap()
                .iter()
                .map(|m| m.edge)
                .collect::<Vec<_>>(),
            [circle.edge]
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::MergeRebasesCarriers {
                edges: vec![circle.edge],
            },
            |b| b.kev(seg.he_plus).unwrap_err(),
        );
    }

    #[test]
    fn the_keys_only_kill_refuses_a_merge_that_moves_no_point_and_kev_describing_takes_it() {
        // The exact inverse of a certified `mev` at the old point: the
        // kill merges the star's two middle spokes back onto a vertex at
        // their own point, so every carrier stays true bitwise. The
        // keys-only door refuses it anyway — it asks no band question,
        // and whether the two points are one is the question the
        // re-basing gate does not ask — and the describing door with
        // nothing listed takes it, carriers untouched.
        let tol = Tol::witness();
        let (mut body, _seed, [_a, b, c, d]) = four_spoke_star();
        let created = body
            .mev(
                MevSite::Fan {
                    he1: b.he_plus,
                    he2: d.he_plus,
                },
                p(0.0),
                EdgeCurveSpec::self_loop_circle_at(p(0.0)),
                tol,
            )
            .unwrap();
        let curves = [
            body.get_edge(b.edge).unwrap().curve,
            body.get_edge(c.edge).unwrap().curve,
        ];
        let carriers = curves.map(|k| format!("{:?}", body.get_curve_geom(k).unwrap()));
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::MergeRebasesCarriers {
                edges: vec![b.edge, c.edge],
            },
            |body| body.kev(created.he_plus).unwrap_err(),
        );
        body.kev_describing(created.he_plus, &[], tol).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            [b.edge, c.edge].map(|e| body.get_edge(e).unwrap().curve),
            curves,
            "no curve is re-minted"
        );
        assert_eq!(
            curves.map(|k| format!("{:?}", body.get_curve_geom(k).unwrap())),
            carriers,
            "every carrier is bitwise what it was"
        );
    }

    /// Whether `edge`'s stored carrier re-certifies against its own
    /// endpoints now.
    fn carrier_is_coherent_here(body: &Body<f64>, edge: EdgeKey, tol: Tol) -> bool {
        let e = body.get_edge(edge).unwrap();
        let point = |he| {
            let v = body.get_half_edge(he).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        };
        body.get_curve_geom(e.curve)
            .unwrap()
            .certified()
            .unwrap()
            .recertify(
                point(e.he_plus),
                point(e.he_minus),
                |k| body.get_surface(k).cloned(),
                geom_core::Band::linear(tol).unwrap(),
            )
            .is_ok()
    }

    #[test]
    fn kev_rejects_self_loops_and_stale_keys() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let circ = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::SelfLoopEdge {
                edge: circ.edge,
                vertex: seed.vertex,
            },
            |b| b.kev(circ.he_plus).unwrap_err(),
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::HalfEdge(HalfEdgeKey::default()),
            },
            |b| b.kev(HalfEdgeKey::default()).unwrap_err(),
        );
    }

    #[test]
    fn kev_rejects_a_corrupt_edge_bijection() {
        // Raw corruption: the edge no longer claims the argument half.
        let (mut body, _, seg, strut) = strutted();
        body.get_edge_mut(seg.edge).unwrap().he_plus = strut.he_plus;
        body.get_edge_mut(seg.edge).unwrap().he_minus = strut.he_minus;
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::UnclaimedHalfEdge {
                he: seg.he_plus,
                edge: seg.edge,
            },
            |b| b.kev(seg.he_plus).unwrap_err(),
        );
    }

    #[test]
    fn kev_rejects_corrupt_loops_and_orbits() {
        // LoopNotCycle: a half-edge whose parent loop claims to be
        // empty (tier-1-invalid raw corruption).
        let (mut body, seed, seg, _strut) = strutted();
        body.get_loop_mut(seed.r#loop).unwrap().boundary = LoopBoundary::Empty {
            vertex: seed.vertex,
        };
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::LoopNotCycle {
                r#loop: seed.r#loop,
            },
            |b| b.kev(seg.he_plus).unwrap_err(),
        );
        // OrbitBroken: the far vertex's orbit walk hits a corrupt mate
        // bijection mid-fan (raw corruption two steps away from the
        // argument).
        let (mut body, _seed, seg, strut) = strutted();
        body.get_edge_mut(strut.edge).unwrap().he_plus = strut.he_minus;
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::OrbitBroken { he: seg.he_minus },
            |b| b.kev(seg.he_plus).unwrap_err(),
        );
    }

    // ------------------------------------------------------------------
    // kef: general / argument side / circular / mate-alone / lone
    // ------------------------------------------------------------------

    /// The digon pillow built through the operators, plus its keys.
    fn ops_pillow() -> (Body<f64>, MvfsCreated, MevCreated, MefCreated) {
        let (mut body, seed, seg) = segment();
        let split = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        (body, seed, seg, split)
    }

    #[test]
    fn kef_general_case_inverts_mef() {
        // Split one pillow face with a second chord edge, then kef the
        // half in the NEW loop: exact undo.
        let (mut body, _seed, seg, split) = ops_pillow();
        let before = canonical_form(&body);
        let before_counts = arena_snapshot(&body);
        // Split the new face (loop [he_minus, seg.he_plus]) between its
        // two vertices.
        let cut = body
            .mef_chord(
                MefSite::Chords {
                    he1: split.he_minus,
                    he2: seg.he_plus,
                },
                Tol::witness(),
            )
            .unwrap();
        let result = body.kef(cut.he_minus, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // E–P vector (0, −1, −1, 0, 0, 0) relative to the pre-mef state.
        assert_eq!(arena_snapshot(&body), before_counts);
        assert_eq!(result.killed_face, cut.face);
        assert_eq!(result.killed_loop, cut.r#loop);
        assert_eq!(result.killed_edge, cut.edge);
        assert_eq!(result.killed_curve, Some(cut.curve));
        // The new face shared the split face's surface, so nothing is
        // reaped.
        assert_eq!(result.killed_surface, None);
        for id in [
            EntityId::Face(cut.face),
            EntityId::Loop(cut.r#loop),
            EntityId::Edge(cut.edge),
            EntityId::HalfEdge(cut.he_plus),
            EntityId::HalfEdge(cut.he_minus),
        ] {
            assert_eq!(body.provenance(id), None, "for {id}");
        }
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kef_kills_the_argument_side() {
        // Same construction, but kef(he_plus): the OLD face (he_plus's
        // side) dies instead, and the new face survives. Pins "he's side
        // dies".
        let (mut body, _seed, seg, split) = ops_pillow();
        let old_face = body.face_of_half_edge(split.he_minus).unwrap();
        let cut = body
            .mef_chord(
                MefSite::Chords {
                    he1: split.he_minus,
                    he2: seg.he_plus,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(old_face, split.face, "the mef split the new pillow face");
        let result = body.kef(cut.he_plus, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_face, split.face);
        assert!(body.get_face(split.face).is_none());
        assert!(body.get_face(cut.face).is_some());
    }

    #[test]
    fn kef_circular_face_inverse() {
        // mef(Chords{x, x}) makes the one-edge circular face; kef on the
        // self-cycled minus half is its exact inverse.
        let (mut body, _seed, seg) = segment();
        let before = canonical_form(&body);
        let circ = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        let result = body.kef(circ.he_minus, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_face, circ.face);
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kef_mate_alone_kills_the_big_side() {
        // Same circular-face configuration, but kef(he_plus): the BIG
        // side (the old face) dies and its remnant becomes the surviving
        // circular face's loop. (Documented asymmetry: no single-mef
        // re-make exists for this kill; validity is the contract.)
        let (mut body, seed, seg) = segment();
        let circ = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        let old_face = body.get_loop(seed.r#loop).unwrap().face;
        let result = body.kef(circ.he_plus, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(result.killed_face, old_face);
        assert_eq!(result.killed_loop, seed.r#loop);
        // The segment remnant survives under the (former) circular
        // face's loop.
        assert_eq!(
            body.get_half_edge(seg.he_plus).unwrap().parent_loop,
            circ.r#loop
        );
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![seg.he_plus, seg.he_minus])
        );
    }

    #[test]
    fn kef_lone_inverse_restores_the_empty_loop() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let before = canonical_form(&body);
        let before_counts = arena_snapshot(&body);
        let circ = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                Tol::witness(),
            )
            .unwrap();
        let result = body.kef(circ.he_minus, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(arena_snapshot(&body), before_counts);
        assert_eq!(result.killed_face, circ.face);
        assert_eq!(result.killed_loop, circ.r#loop);
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: seed.vertex
            }
        );
        assert_eq!(body.get_vertex(seed.vertex).unwrap().emanating, None);
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn kef_requires_two_loops_and_two_faces() {
        // Both halves in one loop: kemr's configuration.
        let (mut body, seed, seg) = segment();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::SameLoop {
                r#loop: seed.r#loop,
            },
            |b| b.kef(seg.he_plus, Tol::witness()).unwrap_err(),
        );
        // Two loops of ONE face (raw-built): SameFace.
        let mut body = Body::<f64>::new();
        let point = body.add_point(p(0.0));
        let vertex = body.add_vertex(
            Vertex {
                point,
                emanating: None,
            },
            prov(),
        );
        let solid = body.add_solid(crate::entity::Solid { shells: vec![] }, prov());
        let shell = body.add_shell(
            Shell {
                faces: vec![],
                solid,
            },
            prov(),
        );
        body.get_solid_mut(solid).unwrap().shells.push(shell);
        let curve = body.add_curve(crate::fixtures::test_curve(p(0.0), Tol::witness()));
        let edge = body.add_edge(
            Edge {
                he_plus: HalfEdgeKey::default(),
                he_minus: HalfEdgeKey::default(),
                curve,
            },
            prov(),
        );
        let surface = body.add_surface(crate::fixtures::test_surface(p(0.0)));
        let one_half_loop = |body: &mut Body<f64>| {
            let l = body.add_loop(
                Loop {
                    boundary: LoopBoundary::Cycle {
                        first: HalfEdgeKey::default(),
                    },
                    face: FaceKey::default(),
                },
                prov(),
            );
            let he = body.add_half_edge(
                HalfEdge {
                    edge,
                    start: vertex,
                    parent_loop: l,
                    next: HalfEdgeKey::default(),
                    prev: HalfEdgeKey::default(),
                },
                prov(),
            );
            body.get_half_edge_mut(he).unwrap().next = he;
            body.get_half_edge_mut(he).unwrap().prev = he;
            body.get_loop_mut(l).unwrap().boundary = LoopBoundary::Cycle { first: he };
            (l, he)
        };
        let (outer, he1) = one_half_loop(&mut body);
        let (ring, he2) = one_half_loop(&mut body);
        let edge_data = body.get_edge_mut(edge).unwrap();
        edge_data.he_plus = he1;
        edge_data.he_minus = he2;
        let face = body.add_face(
            Face {
                sense: true,
                surface,
                outer,
                rings: vec![ring],
                shell,
            },
            prov(),
        );
        body.get_loop_mut(outer).unwrap().face = face;
        body.get_loop_mut(ring).unwrap().face = face;
        body.get_shell_mut(shell).unwrap().faces.push(face);
        body.get_vertex_mut(vertex).unwrap().emanating = Some(he1);
        assert_err_deep_unchanged(&mut body, &EulerOpError::SameFace { face }, |b| {
            b.kef(he1, Tol::witness()).unwrap_err()
        });
    }

    /// The surviving face is where the remnant lands and whose chart
    /// decides the remnant's rows, so a surviving loop whose face does
    /// not resolve is refused in the plan phase, typed as the dying
    /// side's same corruption is, and the body is untouched.
    #[test]
    fn kef_refuses_a_surviving_loop_whose_face_does_not_resolve() {
        let (mut body, _seed, _seg, split) = ops_pillow();
        let m = body.get_edge(split.edge).unwrap().he_plus;
        assert_eq!(m, split.he_plus);
        let l2 = body.get_half_edge(m).unwrap().parent_loop;
        body.get_loop_mut(l2).unwrap().face = FaceKey::default();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::Face(FaceKey::default()),
            },
            |b| b.kef(split.he_minus, Tol::witness()).unwrap_err(),
        );
    }

    /// The documented order puts the surviving face's resolution
    /// before the dying face's ring check, so a body faulted both ways
    /// — the dying face carries a ring AND the surviving loop's face
    /// does not resolve — is refused as the stale face, not the ring.
    /// Two tier-1 corruptions on one call; which one the caller is
    /// told about is the order, and this pins it.
    #[test]
    fn kef_names_the_stale_surviving_face_before_the_dying_faces_ring() {
        let (mut body, _seed, seg, _split) = ops_pillow();
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
        let l2 = body.get_half_edge(seg.he_minus).unwrap().parent_loop;
        body.get_loop_mut(l2).unwrap().face = FaceKey::default();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::Face(FaceKey::default()),
            },
            |b| b.kef(seg.he_plus, Tol::witness()).unwrap_err(),
        );
    }

    #[test]
    fn kef_rejects_a_ringed_dying_face_but_accepts_the_mate() {
        // Plant an empty ring on one pillow face: kef from that side is
        // FaceHasRings; kef on the mate kills the bare other side.
        let (mut body, _seed, seg, split) = ops_pillow();
        // seg.he_plus lives in the new face's loop after the mef; strut
        // + kemr plants a ring there.
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
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::FaceHasRings { face: split.face },
            |b| b.kef(seg.he_plus, Tol::witness()).unwrap_err(),
        );
        // The mate's side (the old face) is ring-free: killable.
        let result = body.kef(seg.he_minus, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_ne!(result.killed_face, split.face);
    }

    // ------------------------------------------------------------------
    // mfkrh
    // ------------------------------------------------------------------

    /// Pillow with one empty ring planted on the new face (strut +
    /// kemr): the §9.3 hole-anchor idiom.
    fn pillow_with_empty_ring() -> (Body<f64>, MefCreated, crate::euler_ring::KemrResult) {
        let (mut body, _seed, seg, split) = ops_pillow();
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
        let kill = body.kemr(strut.he_plus, strut.he_minus).unwrap();
        (body, split, kill)
    }

    #[test]
    fn mfkrh_promotes_an_empty_ring_to_an_empty_outer_face() {
        // THE operator-reachable Empty-outer face (predicted by PR 3's
        // log): promoting an empty ring yields a face whose outer loop
        // is an empty loop — and the body validates.
        let (mut body, split, kill) = pillow_with_empty_ring();
        let before_counts = arena_snapshot(&body);
        let created = body.mfkrh_plug(kill.ring, true, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // E–P vector (0, 0, +1, −1, −1, 0): +1 face, +1 surface, all
        // else unchanged.
        let after = arena_snapshot(&body);
        assert_eq!(
            after,
            ArenaSnapshot {
                counts: ArenaCounts {
                    faces: before_counts.counts.faces + 1,
                    ..before_counts.counts
                },
                surfaces: before_counts.surfaces + 1,
                ..before_counts
            }
        );
        // The promoted loop survives with its key, now the outer loop of
        // the new face.
        let new_face = body.get_face(created.face).unwrap();
        assert_eq!(new_face.outer, kill.ring);
        assert!(new_face.rings.is_empty());
        assert_eq!(new_face.surface, created.surface);
        assert_eq!(body.get_loop(kill.ring).unwrap().face, created.face);
        assert!(matches!(
            body.get_loop(kill.ring).unwrap().boundary,
            LoopBoundary::Empty { .. }
        ));
        // It left the old face's ring list.
        assert!(body.get_face(split.face).unwrap().rings.is_empty());
        // D5: the new face records the promotion; the loop keeps its
        // kemr birth record (promotion is not a re-birth).
        assert_eq!(
            body.provenance(EntityId::Face(created.face)),
            Some(&Provenance::Mfkrh { ring: kill.ring })
        );
        assert!(matches!(
            body.provenance(EntityId::Loop(kill.ring)),
            Some(&Provenance::Kemr { .. })
        ));
    }

    #[test]
    fn mfkrh_then_kfmrh_roundtrips() {
        let (mut body, split, kill) = pillow_with_empty_ring();
        let before = canonical_form(&body);
        let created = body.mfkrh_plug(kill.ring, true, Tol::witness()).unwrap();
        let plug = body
            .kfmrh(split.face, created.face, Tol::witness())
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(plug.ring, kill.ring);
        // The fresh surface died with the face it was minted for.
        assert_eq!(plug.killed_surface, Some(created.surface));
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn mfkrh_unplugs_the_holed_box() {
        // kfmrh ∘ mfkrh on the genus-1 acceptance body: promoting the
        // demoted membrane loop back to a face takes genus 1 → 0, and
        // re-killing it restores the canonical form.
        let t = ops_holed_box(Tol::witness());
        let mut body = t.body;
        let before = canonical_form(&body);
        let bottom_face = t.box_mefs[0].face;
        let created = body.mfkrh_plug(t.plug.ring, true, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));
        // Euler–Poincaré at genus 0 with one ring left (the hole's top
        // rim): v − e + f − r = 16 − 24 + 11 − 1 = 2 = 2(1 − 0).
        let counts = euler_counts(&body);
        assert_eq!((counts.r, counts.f, counts.genus()), (1, 11, Ok(0)));
        let plug = body
            .kfmrh(bottom_face, created.face, Tol::witness())
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(plug.ring, t.plug.ring);
        assert_eq!(canonical_form(&body), before);
    }

    #[test]
    fn mfkrh_rejects_outer_loops_and_stale_keys() {
        let (mut body, _split, kill) = pillow_with_empty_ring();
        let outer = {
            let ring_face = body.get_loop(kill.ring).unwrap().face;
            body.get_face(ring_face).unwrap().outer
        };
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::RingIsOuter { r#loop: outer },
            |b| b.mfkrh_plug(outer, true, Tol::witness()).unwrap_err(),
        );
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleKey {
                key: EntityId::Loop(LoopKey::default()),
            },
            |b| {
                b.mfkrh_plug(LoopKey::default(), true, Tol::witness())
                    .unwrap_err()
            },
        );
    }

    #[test]
    fn mfkrh_rejects_a_stale_shared_surface() {
        // The M1 surface-anchor chain retired with the placeholder
        // surfaces (M2 PR 3); the geometry precondition is now the
        // FaceSurface::Shared key. A stale key must fail loudly and
        // atomically.
        let (mut body, _split, kill) = pillow_with_empty_ring();
        let stale = crate::geometry::SurfaceKey::default();
        assert_err_deep_unchanged(
            &mut body,
            &EulerOpError::StaleGeometry {
                key: crate::entity::GeomRef::Surface(stale),
            },
            |b| {
                b.mfkrh(
                    kill.ring,
                    crate::FaceSurface::Shared {
                        key: stale,
                        sense: true,
                    },
                    Tol::witness(),
                )
                .unwrap_err()
            },
        );
    }

    // ------------------------------------------------------------------
    // The cube teardown: kill all 12 edges via kef/kev in a derived
    // order back to the skeletal state, then kvfs — empty body.
    // ------------------------------------------------------------------

    #[test]
    fn cube_tears_down_to_the_empty_body() {
        let t = declined_cube::<f64>(Tol::witness());
        let mut body = t.body;
        // Undo the five mefs in reverse: each kef(created.he_minus)
        // kills the face that mef made.
        for mef in t.mefs.iter().rev() {
            let result = body.kef(mef.he_minus, Tol::witness()).unwrap();
            assert_eq!(validate(&body), Ok(()));
            assert_eq!(result.killed_face, mef.face);
        }
        // Undo the seven mevs in reverse: each kev(created.he_plus)
        // kills the vertex that mev made.
        for mev in t.mevs.iter().rev() {
            let result = body.kev(mev.he_plus).unwrap();
            assert_eq!(validate(&body), Ok(()));
            assert_eq!(result.killed_vertex, mev.vertex);
        }
        // Skeletal again: kvfs finishes the job.
        assert_eq!(
            body.get_loop(t.seed.r#loop).unwrap().boundary,
            LoopBoundary::Empty {
                vertex: t.seed.vertex
            }
        );
        body.kvfs(t.seed.solid).unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(arena_snapshot(&body), ArenaSnapshot::default());
    }

    // ------------------------------------------------------------------
    // Display coverage for the new error variants
    // ------------------------------------------------------------------

    #[test]
    fn new_error_variants_display() {
        let errors = [
            EulerOpError::UnclaimedHalfEdge {
                he: HalfEdgeKey::default(),
                edge: EdgeKey::default(),
            },
            EulerOpError::SelfLoopEdge {
                edge: EdgeKey::default(),
                vertex: VertexKey::default(),
            },
            EulerOpError::OrbitBroken {
                he: HalfEdgeKey::default(),
            },
            EulerOpError::SolidNotSingleShell {
                solid: SolidKey::default(),
                shells: 2,
            },
            EulerOpError::ShellNotSingleFace {
                shell: ShellKey::default(),
                faces: 3,
            },
        ];
        for err in &errors {
            assert!(!err.to_string().is_empty());
            let _: &dyn std::error::Error = err;
        }
    }

    #[test]
    fn isomorphic_smoke_check_for_the_module() {
        // Two identical grow-shrink histories agree deeply, not just
        // canonically (replay determinism extends to the kill ops).
        let run = || {
            let (mut body, _seed, seg, split) = ops_pillow();
            let cut = body
                .mef_chord(
                    MefSite::Chords {
                        he1: split.he_minus,
                        he2: seg.he_plus,
                    },
                    Tol::witness(),
                )
                .unwrap();
            body.kef(cut.he_minus, Tol::witness()).unwrap();
            // pillow → circular-edge body: the surviving edge closes
            // onto the seed vertex, so the kill re-describes it as the
            // circle there.
            let circle = EdgeCurveSpec::self_loop_circle_at(p(0.0));
            body.kev_describing(seg.he_plus, &[(split.edge, circle)], Tol::witness())
                .unwrap();
            body
        };
        let a = run();
        let b = run();
        assert_eq!(deep_snapshot(&a), deep_snapshot(&b));
        assert!(isomorphic(&a, &b));
    }

    /// The declined cube with `next` tears planted, each `(from, to)`
    /// by position in its half-edge arena, and the arena's keys.
    fn torn_cube(tears: &[(usize, usize)]) -> (Body<f64>, Vec<HalfEdgeKey>) {
        let mut body = declined_cube::<f64>(Tol::witness()).body;
        let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
        for &(from, to) in tears {
            body.get_half_edge_mut(halves[from]).unwrap().next = halves[to];
        }
        (body, halves)
    }

    #[test]
    fn kef_refuses_an_anchor_step_that_leaves_its_vertex() {
        // The first counterexample of a seeded search of one `next`
        // tear on the declined cube: the mate's `next` torn back onto
        // the killed half, so the anchor for `start(he)` falls back to
        // `next(he)`, which starts at the other end. Unchecked, the
        // kill writes that anchor and returns `Ok`.
        let (mut body, halves) = torn_cube(&[(9, 8)]);
        let he = halves[8];
        assert_eq!(body.mate(he), Some(halves[9]));
        let next_he = body.get_half_edge(he).unwrap().next;
        assert_ne!(
            body.get_half_edge(next_he).unwrap().start,
            body.get_half_edge(he).unwrap().start,
            "the fallback anchor starts off the vertex it would anchor"
        );
        let torn = EulerOpError::OrbitBroken { he };
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kef(he, Tol::witness()).unwrap_err());
    }

    #[test]
    fn kev_refuses_a_strut_anchor_that_leaves_the_survivor() {
        // The first counterexample of a seeded search of two `next`
        // tears on the declined cube: `next(he)` torn onto the mate
        // empties the merged fan, so the survivor is anchored at
        // `next(mate)`, which the other tear points at a half-edge of
        // another vertex. Every door that runs the plan refuses it.
        let tol = Tol::witness();
        let (mut body, halves) = torn_cube(&[(19, 5), (18, 19)]);
        let he = halves[18];
        let m = body.mate(he).unwrap();
        assert_eq!(m, halves[19]);
        assert_eq!(
            body.vertex_orbit(m),
            Some(vec![m]),
            "the merged fan is empty"
        );
        assert_ne!(
            body.get_half_edge(halves[5]).unwrap().start,
            body.get_half_edge(he).unwrap().start,
            "the strut anchor starts off the survivor"
        );
        let torn = EulerOpError::OrbitBroken { he };
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kev(he).unwrap_err());
        assert_err_deep_unchanged(&mut body, &torn, |b| {
            b.kev_describing(he, &[], tol).unwrap_err()
        });
        assert_eq!(body.kev_merged_members(he).map(|_| ()), Err(torn));
    }

    /// Whether a half-edge other than `killed` starts at `v`: the vertex
    /// keeps an edge through the kill.
    fn keeps_incidence(body: &Body<f64>, v: VertexKey, killed: &[HalfEdgeKey]) -> bool {
        body.half_edges()
            .any(|(x, data)| data.start == v && !killed.contains(&x))
    }

    #[test]
    fn kev_refuses_a_none_anchor_on_a_survivor_that_keeps_its_edges() {
        // The kill-anchor review's `None`-arm construction: the strut's
        // mate's `next` torn back onto the killed half, so `next(m) ==
        // he` reads as a segment kill and the orbit walk from `he`
        // closes on `[he]`, while the survivor keeps the cube's three
        // edges. Unchecked, the kill anchors it at `None` and returns
        // `Ok`.
        let tol = Tol::witness();
        let fixture = crate::fixtures::ops_strut_cube(tol);
        let mut body = fixture.body;
        let (he, m) = (fixture.strut.he_plus, fixture.strut.he_minus);
        body.get_half_edge_mut(m).unwrap().next = he;
        let v = body.get_half_edge(he).unwrap().start;
        assert_eq!(
            body.vertex_orbit(m),
            Some(vec![m]),
            "the merged fan is empty"
        );
        assert_eq!(
            body.vertex_orbit(he),
            Some(vec![he]),
            "the walk from `he` closes at once"
        );
        assert!(
            keeps_incidence(&body, v, &[he, m]),
            "the survivor keeps edges"
        );
        let torn = EulerOpError::OrbitBroken { he };
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kev(he).unwrap_err());
        assert_err_deep_unchanged(&mut body, &torn, |b| {
            b.kev_describing(he, &[], tol).unwrap_err()
        });
        assert_eq!(body.kev_merged_members(he).map(|_| ()), Err(torn));
    }

    #[test]
    fn kev_refuses_a_none_anchor_where_the_mate_is_not_on_the_killed_edge() {
        // The kill-anchor review's `EdgeBijection` case (`ops_strut_cube`,
        // seed 30, one tear): `halves[16]`'s edge claims `halves[25]` as
        // its other half, whose own edge is another. The orbit walk from
        // that mate steps through its own edge's mate, so the merged fan
        // reads empty although `next(he)` is not the mate, and `next(m)
        // == he` picks the `None` arm at a survivor that keeps edges.
        let tol = Tol::witness();
        let mut body = crate::fixtures::ops_strut_cube(tol).body;
        let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
        let (he, m) = (halves[16], halves[25]);
        let edge = body.get_half_edge(he).unwrap().edge;
        let edge_data = body.get_edge_mut(edge).unwrap();
        (edge_data.he_plus, edge_data.he_minus) = (he, m);
        let he_data = body.get_half_edge(he).unwrap();
        let m_data = body.get_half_edge(m).unwrap();
        assert_ne!(m_data.edge, edge, "the mate's own edge is another");
        assert_ne!(he_data.next, m, "the kill is not a strut's");
        assert_eq!(m_data.next, he, "`next(m)` is the killed half");
        assert_eq!(
            body.vertex_orbit(m),
            Some(vec![m]),
            "the merged fan reads empty"
        );
        assert!(
            keeps_incidence(&body, he_data.start, &[he, m]),
            "the survivor keeps edges"
        );
        let torn = EulerOpError::OrbitBroken { he };
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kev(he).unwrap_err());
        assert_err_deep_unchanged(&mut body, &torn, |b| {
            b.kev_describing(he, &[], tol).unwrap_err()
        });
        assert_eq!(body.kev_merged_members(he).map(|_| ()), Err(torn));
    }

    #[test]
    fn kef_refuses_a_none_anchor_on_a_vertex_that_keeps_its_edges() {
        // The kill-anchor review's `None`-arm construction: both killed
        // halves' `next` torn onto each other, so neither end finds a
        // surviving anchor, while both keep the cube's edges. Unchecked,
        // the kill anchors both at `None` and returns `Ok`.
        let (mut body, halves) = torn_cube(&[(9, 8), (8, 9)]);
        let (he, m) = (halves[8], halves[9]);
        assert_eq!(body.mate(he), Some(m));
        for end in [he, m] {
            let start = body.get_half_edge(end).unwrap().start;
            assert!(
                keeps_incidence(&body, start, &[he, m]),
                "{end:?}'s start keeps edges"
            );
        }
        let torn = EulerOpError::OrbitBroken { he };
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kef(he, Tol::witness()).unwrap_err());
    }

    #[test]
    fn kef_refuses_an_anchor_step_that_leaves_the_mates_start() {
        // `next(he)` shortcut past its successor inside the dying loop:
        // the cycle still closes and `start(he)`'s anchor `next(m)`
        // stands, but `start(m)`'s, `next(he)`, now starts elsewhere.
        let (mut body, halves) = torn_cube(&[]);
        let he = halves[8];
        let m = body.mate(he).unwrap();
        let successor = body.get_half_edge(he).unwrap().next;
        let skip = body.get_half_edge(successor).unwrap().next;
        body.get_half_edge_mut(he).unwrap().next = skip;
        let start = |b: &Body<f64>, x: HalfEdgeKey| b.get_half_edge(x).unwrap().start;
        let next_m = body.get_half_edge(m).unwrap().next;
        assert_eq!(
            start(&body, next_m),
            start(&body, he),
            "`start(he)`'s anchor stands"
        );
        assert_ne!(
            start(&body, skip),
            start(&body, m),
            "`start(m)`'s anchor starts off it"
        );
        let torn = EulerOpError::OrbitBroken { he: m };
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kef(he, Tol::witness()).unwrap_err());
    }

    #[test]
    fn kef_proves_the_anchor_that_wins_where_both_ends_are_one_vertex() {
        // The kill-anchor review's `u == w` construction: a circular edge
        // at one vertex `x`, the segment on one side and a strut on the
        // other, so `next(m)` and `next(he)` are distinct live anchors of
        // `x`. The `start(m)`-side write, `next(he)`, is last and wins.
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        let lone = MevSite::Lone {
            r#loop: seed.r#loop,
        };
        let segment = body
            .mev_line(lone, Point3::new(1.0, 0.0, 0.0), tol)
            .unwrap();
        let chords = MefSite::Chords {
            he1: segment.he_minus,
            he2: segment.he_minus,
        };
        let circle = body.mef_chord(chords, tol).unwrap();
        let fan = MevSite::Fan {
            he1: circle.he_minus,
            he2: circle.he_minus,
        };
        body.mev_line(fan, Point3::new(1.2, 0.1, 0.0), tol).unwrap();
        assert_eq!(validate(&body), Ok(()));
        let (he, m) = (circle.he_plus, circle.he_minus);
        let x = body.get_half_edge(he).unwrap().start;
        assert_eq!(body.get_half_edge(m).unwrap().start, x, "both ends are `x`");
        let next_he = body.get_half_edge(he).unwrap().next;
        let next_m = body.get_half_edge(m).unwrap().next;
        assert!(
            next_he != next_m && ![he, m].contains(&next_he) && ![he, m].contains(&next_m),
            "both anchors are live and distinct"
        );

        let mut killed = body.clone();
        killed.kef(he, tol).unwrap();
        assert_eq!(validate(&killed), Ok(()));
        assert_eq!(killed.get_vertex(x).unwrap().emanating, Some(next_he));

        // `next(he)` shortcut to the segment's other half, which starts
        // at the origin: the winning anchor leaves `x`.
        let skip = body.get_half_edge(next_he).unwrap().next;
        assert_ne!(body.get_half_edge(skip).unwrap().start, x);
        body.get_half_edge_mut(he).unwrap().next = skip;
        let torn = EulerOpError::OrbitBroken { he: m };
        assert_err_deep_unchanged(&mut body, &torn, |b| b.kef(he, tol).unwrap_err());
    }

    /// The loop `x` claims.
    fn loop_of(body: &Body<f64>, x: HalfEdgeKey) -> LoopKey {
        body.get_half_edge(x).unwrap().parent_loop
    }

    #[test]
    fn kef_refuses_a_loop_anchor_step_into_another_loop() {
        // The first one-tear counterexample of the loop-anchor probe
        // (`review_d18::kill_anchors_on_torn_bodies`) on the declined
        // cube: the mate's `next` torn onto a half-edge of a third loop
        // that starts where the mate ends. `start(he)`'s anchor stands,
        // but the surviving loop would anchor in the third loop.
        // Unchecked, the kill writes that anchor and returns `Ok`.
        let (mut body, halves) = torn_cube(&[(7, 14)]);
        let (he, m, step) = (halves[6], halves[7], halves[14]);
        assert_eq!(body.mate(he), Some(m));
        let (l1, l2) = (loop_of(&body, he), loop_of(&body, m));
        assert!(
            ![l1, l2].contains(&loop_of(&body, step)),
            "`next(m)` lies in a third loop"
        );
        let start = |x: HalfEdgeKey| body.get_half_edge(x).unwrap().start;
        assert_eq!(start(step), start(he), "the vertex anchor stands");
        let torn = EulerOpError::LoopCycleBroken { r#loop: l2 };
        assert_kill_refuses(&mut body, &torn, |b| b.kef(he, Tol::witness()));
    }

    /// Every door that runs `kev`'s plan refuses `torn` at `he`, with
    /// the body deep-unchanged: `kev_describing` with `chords` first,
    /// then `kev`, `kev_describing` with no list, the ungated kill and
    /// `kev_merged_members`. `chords` re-describes every merged member,
    /// so the describing door's gates pass and the plan is all that
    /// stands between a public door and the kill's writes; with no fan,
    /// it is empty and every door reaches the writes.
    fn assert_kev_doors_refuse(
        body: &mut Body<f64>,
        he: HalfEdgeKey,
        chords: &[(EdgeKey, EdgeCurveSpec<f64>)],
        torn: &EulerOpError,
    ) {
        let tol = Tol::witness();
        assert_kill_refuses(body, torn, |b| b.kev_describing(he, chords, tol));
        assert_kill_refuses(body, torn, |b| b.kev(he));
        assert_kill_refuses(body, torn, |b| b.kev_describing(he, &[], tol));
        assert_kill_refuses(body, torn, |b| b.kev_ungated(he));
        assert_eq!(body.kev_merged_members(he).map(|_| ()), Err(torn.clone()));
    }

    /// The chord re-descriptions of `kev(halves[at])` on the untorn
    /// declined cube, whose fan every tear below leaves standing: the
    /// list that takes the describing door past its gates.
    fn cube_chords(at: usize) -> Vec<(EdgeKey, EdgeCurveSpec<f64>)> {
        let (untorn, halves) = torn_cube(&[]);
        let chords = crate::seqgen::try_chord_redescriptions(&untorn, halves[at]).unwrap();
        assert!(!chords.is_empty(), "the kill merges a fan");
        chords
    }

    #[test]
    fn kev_refuses_a_loop_anchor_step_into_another_loop() {
        // The loop-anchor probe's first counterexample for `kev`: the
        // mate's `next` torn onto a half-edge of a third loop that starts
        // where the mate ends, so the fan walk and `he`'s loop stand and
        // the mate's loop would anchor in the third loop.
        let (mut body, halves) = torn_cube(&[(21, 5)]);
        let (he, m, step) = (halves[20], halves[21], halves[5]);
        assert_eq!(body.mate(he), Some(m));
        let (l1, l2) = (loop_of(&body, he), loop_of(&body, m));
        assert!(
            ![l1, l2].contains(&loop_of(&body, step)),
            "`next(m)` lies in a third loop"
        );
        let torn = EulerOpError::LoopCycleBroken { r#loop: l2 };
        assert_kev_doors_refuse(&mut body, he, &cube_chords(20), &torn);
    }

    #[test]
    fn kev_refuses_a_loop_anchor_at_the_killed_mate() {
        // The general arm with the mate's `next` torn onto itself: the
        // mate's loop would re-anchor at `next(m)`, which is the mate,
        // and it claims that loop, so only "the anchor is not killed"
        // refuses it.
        let (mut body, halves) = torn_cube(&[(7, 7)]);
        let (he, m) = (halves[6], halves[7]);
        assert_eq!(body.mate(he), Some(m));
        let (l1, l2) = (loop_of(&body, he), loop_of(&body, m));
        assert_ne!(l1, l2, "the general arm, across two loops");
        let torn = EulerOpError::LoopCycleBroken { r#loop: l2 };
        assert_kev_doors_refuse(&mut body, he, &cube_chords(6), &torn);
    }

    #[test]
    fn kev_refuses_an_adjacent_pair_whose_halves_lie_in_two_loops() {
        // The probe's one-tear mirror case on the declined cube: the
        // mate's `next` torn back onto the killed half, so `next(m) ==
        // he` reads the two halves as adjacent in one loop. The mirror
        // arm re-anchors `he`'s loop alone, and the mate's loop, which
        // is another and anchors at the mate itself, keeps a dead
        // anchor.
        let (mut body, halves) = torn_cube(&[(7, 6)]);
        let (he, m) = (halves[6], halves[7]);
        assert_eq!(body.mate(he), Some(m));
        let (l1, l2) = (loop_of(&body, he), loop_of(&body, m));
        assert_ne!(l1, l2, "the halves claim two loops");
        assert_eq!(
            body.get_loop(l2).unwrap().boundary,
            LoopBoundary::Cycle { first: m },
            "the mate's loop anchors at the mate"
        );
        let torn = EulerOpError::LoopCycleBroken { r#loop: l2 };
        assert_kev_doors_refuse(&mut body, he, &cube_chords(6), &torn);
    }

    #[test]
    fn kev_refuses_a_strut_whose_halves_lie_in_two_loops() {
        // The strut arm across two loops (the review's construction): on
        // the strut cube, the top loop's half `he` from the strut's base
        // has its mate in a side face; `next(he)` is torn onto the mate
        // and `next(m)` onto the strut, so the kill reads a strut in the
        // top loop. The fan walk closes at once on the mate, so no fan
        // merges and no gate is asked; the strut arm re-anchors the top
        // loop alone, and the side face's loop keeps the dead mate.
        let fixture = crate::fixtures::ops_strut_cube(Tol::witness());
        let mut body = fixture.body;
        let base = body.get_half_edge(fixture.strut.he_plus).unwrap().start;
        let (he, _) = body
            .half_edges()
            .find(|&(x, data)| {
                data.parent_loop == fixture.outer
                    && data.start == base
                    && x != fixture.strut.he_plus
            })
            .unwrap();
        let m = body.mate(he).unwrap();
        let l2 = loop_of(&body, m);
        assert_ne!(l2, fixture.outer, "the mate lies in a side face");
        body.get_half_edge_mut(he).unwrap().next = m;
        body.get_half_edge_mut(m).unwrap().next = fixture.strut.he_plus;
        assert_eq!(body.vertex_orbit(m), Some(vec![m]), "no fan merges");
        let torn = EulerOpError::LoopCycleBroken { r#loop: l2 };
        assert_kev_doors_refuse(&mut body, he, &[], &torn);
    }

    #[test]
    fn kev_refuses_a_segment_whose_halves_lie_in_two_loops() {
        // The segment arm across two loops: a segment beside a circle,
        // with the segment's minus half torn to claim the circle's plus
        // loop and that loop torn to anchor at it. The cycle `[he, m]`
        // reads a segment, whose arm empties `he`'s loop alone; the
        // circle's loop keeps the dead mate as its anchor.
        let tol = Tol::witness();
        let mut body = Body::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
                tol,
            )
            .unwrap();
        let other = body.mvfs(p(5.0), true).unwrap();
        let circle = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: other.r#loop,
                },
                tol,
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        let (he, m) = (seg.he_plus, seg.he_minus);
        let l2 = loop_of(&body, circle.he_plus);
        body.get_half_edge_mut(m).unwrap().parent_loop = l2;
        body.get_loop_mut(l2).unwrap().boundary = LoopBoundary::Cycle { first: m };
        let torn = EulerOpError::LoopCycleBroken { r#loop: l2 };
        assert_kev_doors_refuse(&mut body, he, &[], &torn);
    }

    #[test]
    fn kev_refuses_a_segment_kill_that_merges_a_fan() {
        // The segment arm where a fan merges onto the survivor, which
        // the mate's own edge being another makes possible. A digon
        // pillow with a strut at `v1` inside face B; the edge of `a0`
        // (`v0 → v1`) is torn to claim `a1` (`v1 → v0`, face A's other
        // half) as its mate, and the strut's return half is torn onto
        // `a1`. Face A's loop is `[a0, a1]`, so the kill reads a segment;
        // but the orbit walk from `a1` steps through `a1`'s own edge and
        // closes on `[a1, strut]`, a fan the merge moves onto `v0`. The
        // loop would empty at `v0` while the strut starts there.
        let tol = Tol::witness();
        let pillow = crate::fixtures::pillow(tol);
        let mut body = pillow.body;
        let (a0, a1, b0) = (pillow.hes_a[0], pillow.hes_a[1], pillow.hes_b[0]);
        let strut = body
            .mev_line(MevSite::Fan { he1: b0, he2: b0 }, p(1.5), tol)
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        let edge = body.get_edge_mut(pillow.edges[0]).unwrap();
        assert_eq!(edge.he_plus, a0);
        edge.he_minus = a1;
        body.get_half_edge_mut(strut.he_minus).unwrap().next = a1;
        let next = |b: &Body<f64>, x: HalfEdgeKey| b.get_half_edge(x).unwrap().next;
        assert_eq!((next(&body, a0), next(&body, a1)), (a1, a0), "a segment");
        assert_eq!(
            body.vertex_orbit(a1),
            Some(vec![a1, strut.he_plus]),
            "a merged fan"
        );
        // The strut, re-described where the merge lands it: from `v0`
        // to its tip.
        let point = |x: HalfEdgeKey| {
            let v = body.get_half_edge(x).unwrap().start;
            body.resolve_vertex_point(v).unwrap()
        };
        let chords = [(
            strut.edge,
            EdgeCurveSpec::line_between(point(a0), point(strut.he_minus)),
        )];
        let torn = EulerOpError::LoopCycleBroken {
            r#loop: pillow.loop_a,
        };
        assert_kev_doors_refuse(&mut body, a0, &chords, &torn);
    }

    #[test]
    fn kev_refuses_to_empty_a_loop_at_another_loops_lone_vertex() {
        // The review's S8: a segment beside a lone vertex `x` (a second
        // `mvfs`), with the segment's plus half torn to start at `x`.
        // The kill reads `x` as the survivor and the segment's loop as
        // emptying there, but `x` is already another loop's lone vertex,
        // and the segment's own start would keep a dead anchor.
        let tol = Tol::witness();
        let mut body = Body::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
                tol,
            )
            .unwrap();
        let other = body.mvfs(p(5.0), true).unwrap();
        body.get_half_edge_mut(seg.he_plus).unwrap().start = other.vertex;
        let torn = EulerOpError::LoopCycleBroken {
            r#loop: seed.r#loop,
        };
        assert_kev_doors_refuse(&mut body, seg.he_plus, &[], &torn);
    }

    #[test]
    fn kef_refuses_to_empty_a_loop_that_strands_its_vertex() {
        // The review's S9: a circle (a self-loop edge at `v`, each half
        // its own one-half-edge loop) beside a lone vertex `x`, with the
        // mate torn to start at `x`. The kill reads the `Lone` inverse
        // and anchors both `v` and `x` at `None`, but empties the
        // surviving loop at `x`, which leaves `v` held by no loop.
        let tol = Tol::witness();
        let mut body = Body::new();
        let seed = body.mvfs(p(0.0), true).unwrap();
        let circle = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                tol,
            )
            .unwrap();
        let other = body.mvfs(p(5.0), true).unwrap();
        let (he, m) = (circle.he_plus, circle.he_minus);
        assert_eq!(body.mate(he), Some(m));
        body.get_half_edge_mut(m).unwrap().start = other.vertex;
        let torn = EulerOpError::OrbitBroken { he };
        assert_kill_refuses(&mut body, &torn, |b| b.kef(he, tol));
    }

    #[test]
    fn kef_refuses_a_remnant_walked_through_another_loop() {
        // The loop-anchor probe's `kef` counterexample once the written
        // anchor is proven (declined cube, seed 57, two tears): the
        // dying loop's walk is diverted through a third loop and back,
        // so the remnant carries that loop's anchor into the surviving
        // loop. Unchecked, the third loop keeps an anchor that the kill
        // moved into another loop, and the kill returns `Ok`.
        let (mut body, halves) = torn_cube(&[(5, 14), (13, 1)]);
        let he = halves[1];
        let l1 = loop_of(&body, he);
        let walk = body.loop_cycle(he).unwrap();
        let taken: Vec<LoopKey> = walk
            .iter()
            .map(|&x| loop_of(&body, x))
            .filter(|&l| l != l1)
            .collect();
        assert!(
            taken.iter().any(|&l| {
                let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary else {
                    return false;
                };
                walk.contains(&first)
            }),
            "the walk takes a third loop's anchor"
        );
        let torn = EulerOpError::LoopCycleBroken { r#loop: l1 };
        assert_kill_refuses(&mut body, &torn, |b| b.kef(he, Tol::witness()));
    }

    #[test]
    fn kef_refuses_to_kill_a_loop_whose_walk_skips_a_member() {
        // The dying loop's walk torn past its third member
        // (`next(next(he)) := next(next(next(he)))`, one `next` tear on
        // the declined cube): the walk closes without that member, which
        // still claims the dying loop. Unchecked, the kill removes the
        // loop, leaves the member naming it, and returns `Ok`.
        let mut body = declined_cube::<f64>(Tol::witness()).body;
        let (he, _) = body.half_edges().next().unwrap();
        let second = body.get_half_edge(he).unwrap().next;
        let skipped = body.get_half_edge(second).unwrap().next;
        let fourth = body.get_half_edge(skipped).unwrap().next;
        body.get_half_edge_mut(second).unwrap().next = fourth;
        let l1 = loop_of(&body, he);
        assert!(
            !body.loop_cycle(he).unwrap().contains(&skipped) && loop_of(&body, skipped) == l1,
            "the walk skips a member of the dying loop"
        );
        let torn = EulerOpError::LoopCycleBroken { r#loop: l1 };
        assert_kill_refuses(&mut body, &torn, |b| b.kef(he, Tol::witness()));
    }

    /// A segment's solid beside a lone vertex's (`mvfs`, then `mev_line`
    /// at `MevSite::Lone`, then `mvfs`): the segment's plus half and the
    /// lone solid.
    fn segment_beside_a_lone_solid() -> (Body<f64>, HalfEdgeKey, MvfsCreated) {
        let (mut body, _, seg) = segment();
        let lone = body.mvfs(p(5.0), true).unwrap();
        (body, seg.he_plus, lone)
    }

    #[test]
    fn kvfs_refuses_a_loop_a_torn_half_edge_claims() {
        // The segment's plus half torn to claim the lone loop. Unchecked,
        // the kill removes the loop the half-edge names, and returns `Ok`.
        let (mut body, he, lone) = segment_beside_a_lone_solid();
        body.get_half_edge_mut(he).unwrap().parent_loop = lone.r#loop;
        let torn = EulerOpError::LoopCycleBroken {
            r#loop: lone.r#loop,
        };
        assert_kill_refuses(&mut body, &torn, |b| b.kvfs(lone.solid));
    }

    #[test]
    fn kvfs_refuses_a_vertex_a_torn_half_edge_starts_at() {
        // The segment's plus half torn to start at the lone vertex.
        // Unchecked, the kill removes the vertex the half-edge starts at,
        // and returns `Ok`.
        let (mut body, he, lone) = segment_beside_a_lone_solid();
        body.get_half_edge_mut(he).unwrap().start = lone.vertex;
        let torn = EulerOpError::OrbitBroken { he };
        assert_kill_refuses(&mut body, &torn, |b| b.kvfs(lone.solid));
    }
}

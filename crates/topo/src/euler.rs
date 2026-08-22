//! Euler operators — the sanctioned construction path for topology (D1).
//!
//! This module implements the make-direction operators [`Body::mvfs`],
//! [`Body::mev`], and [`Body::mef`] (Mäntylä ch. 9 semantics, ch. 11
//! surgeries re-derived under our orientation convention), plus the
//! addressing helper [`Body::find_half_edge`]. The ring/genus operators
//! ([`Body::kemr`], [`Body::mekr`], [`Body::kfmrh`]) and the `ring_move`
//! helper live in the sibling module [`crate::euler_ring`] (M1 PR 3),
//! the kill-direction duals ([`Body::kvfs`], [`Body::kev`],
//! [`Body::kef`], [`Body::mfkrh`]) in [`crate::euler_kill`] (M1 PR 4);
//! all share this module's contracts and [`EulerOpError`]. Since M1
//! PR 5 the raw-insertion builder is `pub(crate)`: these operators are
//! the **only** public construction path (D1).
//!
//! # Operator contracts (uniform across all ops)
//!
//! - **Atomic.** Every precondition is validated *before the first
//!   mutation*; on `Err` the body is untouched. Failures are typed
//!   ([`EulerOpError`], closed enum) — never panics (D9). Checks run in
//!   the documented order per op, and the first failure is returned.
//! - **Tier-1-valid input assumed.** The operators are specified on
//!   euler-valid bodies (what [`fn@crate::validate`] accepts). What D9
//!   guarantees on corrupt input, after the **D2 addendum** amended its
//!   footnote, is the bounded-traversal half: no panic, no hang, every
//!   traversal bounded — plus a typed error where corruption is
//!   detectable. **A mutation phase announces a failed lookup rather
//!   than discarding it, at every write in these modules** — and, as
//!   of D21, at every write in `split_edge`, the attach setters,
//!   `movefac`, `revert`, `merge_coplanar_faces`' role pass, the
//!   boolean graft and the splitting carve. That enumeration is the
//!   claim; it is not "the whole crate", and it has **one named
//!   exception**: `merge_coplanar_faces`' ring re-homing still
//!   defaults a failed face lookup to an empty ring list, because its
//!   key arrives from a loop's back-pointer and no check in the call
//!   proves it — so its disposition is a typed error rather than a
//!   panic, and it is open as `SMELL-SCAN-2026-08.md`'s **D88**. Every
//!   key a
//!   mutation writes through is either minted in that phase or proven
//!   live by a check in the same call — here the plan phase, which
//!   returns [`EulerOpError::StaleKey`] otherwise — and never by the
//!   body's tier-1 validity, which is a whole-body property no single
//!   call establishes; the writes themselves state
//!   that impossibility as `unreachable!` (the addendum's row 4), and
//!   the one write helper these modules share
//!   ([`Body::link_half_edges`]) states it as a precondition its
//!   callers discharge. The
//!   *output* still carries no validity promise on corruption the plan
//!   phase cannot see — a consistently wrong `parent_loop` makes every
//!   lookup succeed and write the wrong topology — but that residue is
//!   wrong data, not a swallowed failure.
//! - **Deterministic minting order** (D9 + lineage replay): each op's
//!   doc comment states the exact arena-insertion order of everything it
//!   mints. Two bodies built by identical operator sequences mint
//!   identical key sequences.
//! - **D5 provenance:** every topology entity minted by a call records
//!   that call — the operator and its argument keys — as a typed
//!   [`Provenance`] variant ([`Provenance::Mvfs`] / [`Provenance::Mev`] /
//!   [`Provenance::Mef`]).
//! - **Debug postconditions** (D1's ratified clause): under
//!   `cfg(debug_assertions)`, each successful op asserts that the arena
//!   count deltas match the `ArenaDelta` it declares and that the whole
//!   body still passes tier-1 [`crate::validate::validate`]. On
//!   tier-1-valid input
//!   a firing postcondition is a kernel bug by definition (the per-call
//!   instance of the ch. 9 soundness theorem failing against our
//!   transcription). Raw insertion is crate-internal since PR 5's
//!   builder demotion, so a body is reachable only through the public
//!   mutation paths, and the property those paths owe is that each
//!   **preserves tier 1**: the Euler operators with their chord/line
//!   sugar, and the non-operator structural mutators
//!   ([`Body::ring_move`], [`Body::split_edge`], [`Body::movefac`],
//!   [`Body::merge_coplanar_faces`]) declare the same debug
//!   postcondition or are composed of operators that do; the
//!   attach/metadata setters re-certify under their own tier-1
//!   assertion ([`Body::set_face_surface`], [`Body::set_edge_curve`])
//!   or write fields tier 1 does not constrain. **The closure property
//!   is the claim; a count of the doors is not** — an enumeration
//!   frozen into this sentence is what rots as doors are added, and
//!   `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
//!   checks the property against the real surface rather than against
//!   this list. `ring_move`'s case is the least obvious of the
//!   asserting doors: it re-glues the per-shell component partition,
//!   and the separating-curve argument lives in its docs.
//!
//!   **The exception, and it is a real one.**
//!   [`crate::instance`]'s grafts are a **raw transplant**, not an
//!   operator run: `graft_disjoint_all_keyed` mints an empty
//!   destination solid per source solid before transplanting, and a
//!   refusal raised mid-transplant leaves `dst` partially written —
//!   its own docs say the destination is then *spent, never
//!   resumable*, and the destination's own docs, `DESIGN.md`'s D9
//!   footnote and the 37-door allowlist entry in
//!   `review_m1_pr5_internal` all name that state as the tier-1 error
//!   [`crate::ValidationError::SolidWithoutShells`] — which is the
//!   *late* failure, raised after the transplant's second pass with
//!   every key patched. **All three understate it.** A refusal raised
//!   between the transplant's two passes leaves entities holding
//!   source-internal keys, which in `dst` either dangle or resolve to
//!   an unrelated live entity. Whoever takes S14 fixes one of three
//!   copies of the same sentence. So a caller that
//!   ignores a graft's `Err` and keeps using `dst` can hand the next
//!   operator a tier-1-invalid body and fire its postcondition from
//!   **API misuse rather than a kernel bug**. That is the state class
//!   D9's footnote asserts cannot occur and the D2 addendum's five
//!   classes do not cover; it is open as **S14** in
//!   `docs/SMELL-SCAN-2026-08.md` and is not settled here.
//!
//!   The D9 taxonomy consequence therefore holds **for every door but
//!   that one**: these debug panics are
//!   **unreachable by input** through the public API — reaching one
//!   requires in-crate raw corruption (which is what the validator's
//!   own tests do deliberately) or a discarded graft refusal. Release
//!   builds carry no postcondition either way: on corruption the plan
//!   phase cannot detect they return `Ok` with a garbage body. That is
//!   wrong data written by lookups that all succeeded — the silent
//!   discards the D2 addendum superseded are gone from these three
//!   modules, the shared write helper
//!   ([`Body::link_half_edges`]) included.
//!
//! # Geometry policy at M2 (PR 3 — the M0 placeholders retired)
//!
//! Edge-minting operators take the new edge's geometry as an
//! **uncertified spec** ([`geom_brep::EdgeCurveSpec`]: D2 intensional
//! description + carrier cache + parameter interval) and run the D4 ¶2
//! certification gate *before mutating*: the spec is certified against
//! the edge's endpoint points and the body's surfaces
//! (`EdgeCurve::certify`), and a failure is a typed
//! [`EulerOpError::Certification`] with the body untouched (atomicity
//! extends over the geometry gate). Face-minting operators take the new
//! face's surface as a [`FaceSurface`] spec (inherit the split face's
//! key / mint a new [`Surface`] / share an existing key).
//!
//! - `mvfs`/`mev` insert the given [`Point3`] as a new point (only
//!   vertex-creating operators carry coordinates — Mäntylä ch. 11).
//!   `mvfs`'s seed face gets the [`Surface::Nurbs`]
//!   representable-unimplemented placeholder — the honest "no
//!   description yet" state (a sweep's seed face becomes a cap whose
//!   plane exists only later; attach it via
//!   [`Body::set_face_surface`]). Legal mid-construction; the tier-3
//!   validator rejects it at rest.
//! - `mev`/`mef`/`mekr` certify their curve spec with `he_plus`'s
//!   endpoints in the `he_plus` forward order (increasing carrier
//!   parameter runs `start(he_plus) → end(he_plus)` — the ratified
//!   contract).
//! - Intrinsic (`Intersection`) descriptions typically attach **after**
//!   the adjacent faces' surfaces exist (a swept edge is minted before
//!   its side faces): mint with a conventional spec, then upgrade via
//!   [`Body::set_edge_curve`], which also enforces
//!   description-adjacency coherence. The operators themselves accept
//!   any spec that certifies.
//! - Chord-line sugar for polyhedral construction and the migrated M1
//!   suites: [`Body::mev_line`], [`Body::mef_chord`],
//!   [`Body::mekr_chord`] derive the spec from the site's endpoint
//!   points ([`geom_brep::EdgeCurveSpec::line_between`]; self-loop
//!   sites use the canonical scaffolding circle,
//!   [`geom_brep::EdgeCurveSpec::self_loop_circle_at`]).
//! - The new face joins the old face's shell (membership plus
//!   back-pointer).
//!
//! # Orientation: how the book's surgeries were adapted
//!
//! Mäntylä's GWB orients face loops **clockwise** viewed from outside;
//! we ratified counterclockwise (interior-left — see [`crate::entity`]).
//! Every orbit-order-sensitive detail below was re-derived from our
//! convention rather than transcribed:
//!
//! - The fan run of [`MevSite::Fan`] is defined along **our** orbit step
//!   `next(mate(·))`, which walks **clockwise** viewed from outside
//!   (derived and pinned in PR 1). The pointer surgery is combinatorially
//!   the same as `lmev`'s; the *geometric reading* of "from `he1` to
//!   `he2`" is mirrored relative to the book's figures.
//! - **Edge direction for `mev` deviates from Mäntylä**: our `he_plus`
//!   (the intrinsic direction, [`crate::Edge`]) runs **old vertex → new
//!   vertex**; `lmev` orients the new edge new → old. Chosen for
//!   readability ("the edge grows away from where you applied it");
//!   everything downstream reads direction off `he_plus`, so only this
//!   one site had to pick.
//! - Edge direction for `mef` keeps Mäntylä's association:
//!   `start(he1) → start(he2)` is `he_plus`, and **`he1`'s side becomes
//!   the new face's outer loop** — both orientation-neutral, so there
//!   was nothing to mirror.
//!
//! # Example: skeletal body → segment → digon pillow
//!
//! One `mvfs`, one `mev`, one `mef` build the minimal closed solid (two
//! vertices, two edges, two faces — the digon pillow):
//!
//! ```
//! use geom_core::Point3;
//! use topo::{Body, MefSite, MevSite};
//! use geom_core::Tol;
//!
//! # fn run() -> Result<(), topo::EulerOpError> {
//! let tol = Tol::witness();
//! let mut body = Body::<f64>::new();
//! // The skeletal body: one face whose outer loop is a lone vertex.
//! let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0))?;
//! // Grow the lone vertex into a segment edge v → w (chord-line sugar;
//! // a sweep would pass its own EdgeCurveSpec).
//! let seg = body.mev_line(
//!     MevSite::Lone { r#loop: seed.r#loop },
//!     Point3::new(1.0, 0.0, 0.0),
//!     tol,
//! )?;
//! // Split the loop with a second v–w edge: the segment closes into a
//! // two-edge, two-face pillow — the smallest closed manifold body.
//! let split = body.mef_chord(MefSite::Chords {
//!     he1: seg.he_plus,
//!     he2: seg.he_minus,
//! }, tol)?;
//! assert_eq!(topo::validate(&body), Ok(()));
//! assert_eq!(body.vertices().count(), 2);
//! assert_eq!(body.edges().count(), 2);
//! assert_eq!(body.faces().count(), 2);
//! // The new face's outer loop is he1's side, per the documented
//! // association.
//! assert_eq!(
//!     body.get_half_edge(seg.he_plus).unwrap().parent_loop,
//!     split.r#loop,
//! );
//! # Ok(()) }
//! # run().unwrap();
//! ```

use core::fmt;

use geom::Surface;
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec};
use geom_core::{Band, Decide, Point3, Real, Tol};

use crate::body::Body;
use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::live::Live;
use crate::provenance::Provenance;
#[cfg(debug_assertions)]
use crate::test_support_impl::ArenaCounts;

/// How a face-minting operator obtains the new face's surface (M2 PR 3
/// — the sweep supplies each face's surface explicitly; op parameters,
/// not post-hoc patching).
#[derive(Clone, Debug)]
pub enum FaceSurface<T: Real> {
    /// Share the *affected* face's surface key: `mef`'s split face (a
    /// face split is two regions of one surface — the M1 semantics,
    /// still right for coplanar/cosurface splits) or `mfkrh`'s
    /// demoting face.
    Inherit,
    /// Mint a new surface for the new face (the sweep's usual case —
    /// e.g. a Newell-certified plane from `geom_brep::newell_plane`).
    New(Surface<T>),
    /// Share an existing surface key (identical-by-construction
    /// surfaces keep one key — the ratified no-face-merging story's
    /// sharing half). Must resolve, checked as a precondition.
    Shared(SurfaceKey),
}

/// Where [`Body::mev`] acts: the site addressing for "make edge,
/// vertex".
///
/// The ratified typed-`Empty` loop state ([`LoopBoundary::Empty`]) means
/// the degenerate configurations are addressed **explicitly**, not
/// through placeholder half-edges: GWB's uniform "two half-edge
/// pointers" API silently accepts an empty loop's placeholder half-edge;
/// here the lone-vertex case is its own variant and a half-edge argument
/// always names a real edge segment.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MevSite {
    /// Split the edge fan of the vertex both half-edges start at.
    ///
    /// `he1` and `he2` must start at the same vertex `v`. The
    /// **contiguous run of `v`'s orbit from `he1` (inclusive) to `he2`
    /// (exclusive)** is reassigned to start at the new vertex; the new
    /// edge then joins `v` to the new vertex.
    ///
    /// **Run direction (ratified here, once):** the run is walked with
    /// the orbit step `next(mate(·))` — **clockwise around `v` viewed
    /// from outside** under our counterclockwise-loops convention
    /// (see [`crate::entity`]; the step itself is
    /// [`Body::vertex_orbit`]'s). Choosing the other direction would
    /// move the complementary set whenever `v` has valence ≥ 3 with
    /// `he1`/`he2` non-adjacent in the orbit; the valence-4 fan-split
    /// test pins this choice.
    ///
    /// `he1 == he2` means an **empty run**: nothing is reassigned and
    /// the new edge becomes a *strut* — a dangling edge from `v` to the
    /// new vertex, traversed twice by `he1`'s loop, spliced in
    /// immediately before `he1`.
    Fan {
        /// First half-edge of the run (inclusive); must start at the
        /// same vertex as `he2`.
        he1: HalfEdgeKey,
        /// End of the run (exclusive); must start at the same vertex as
        /// `he1`. The new edge's far end attaches here: the cycle
        /// position immediately before `he2` receives the new minus
        /// half.
        he2: HalfEdgeKey,
    },
    /// Grow a lone vertex: the loop must be [`LoopBoundary::Empty`],
    /// holding vertex `v`. The result is a *segment*: a single edge from
    /// `v` to the new vertex, and the loop becomes a two-half-edge
    /// cycle (`v`'s `emanating` goes from `None` to the new plus half).
    ///
    /// This is the state `mvfs` seeds; `mvfs` followed by `mev(Lone)`
    /// is the canonical opening of every construction.
    Lone {
        /// The empty loop to grow.
        r#loop: LoopKey,
    },
}

/// Where [`Body::mef`] acts: the site addressing for "make edge, face".
///
/// Same design as [`MevSite`]: the degenerate lone-vertex case is a
/// typed variant, not a placeholder half-edge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MefSite {
    /// Split a loop by joining `start(he1)` to `start(he2)` with a new
    /// edge. Both half-edges must belong to the **same loop**.
    ///
    /// The cycle is divided so that **`he1`'s side — the half-edges from
    /// `he1` (inclusive) to `he2` (exclusive) in `next` order — becomes
    /// the NEW face's outer loop** (Mäntylä's association; it is
    /// orientation-neutral, so it survives our mirrored convention
    /// unchanged). `he2`'s side stays in the old loop, which keeps its
    /// outer/ring designation on the old face.
    ///
    /// `he1 == he2` means an empty run: the new face is a one-edge
    /// **circular (self-loop) face** at `start(he1)` — its outer loop is
    /// the single new minus half, and the new plus half is spliced into
    /// the old loop immediately before `he1`.
    Chords {
        /// First half-edge of the side that becomes the new face's
        /// outer loop (inclusive). The new edge starts at `start(he1)`.
        he1: HalfEdgeKey,
        /// End of the moved side (exclusive); stays in the old loop.
        /// The new edge ends at `start(he2)`.
        he2: HalfEdgeKey,
    },
    /// Split an empty loop: the loop must be [`LoopBoundary::Empty`],
    /// holding lone vertex `v`. The result is ch. 9's "circular edge
    /// from a lone vertex" (Fig. 9.8b): one self-loop edge at `v` whose
    /// two halves are two one-half-edge loops — the old loop keeps the
    /// plus half, the new face's outer loop gets the minus half — and a
    /// new face.
    Lone {
        /// The empty loop to split.
        r#loop: LoopKey,
    },
}

/// Every key minted by one [`Body::mvfs`] call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MvfsCreated {
    /// The new solid.
    pub solid: SolidKey,
    /// The new (only) shell of the solid.
    pub shell: ShellKey,
    /// The new (only) face of the shell.
    pub face: FaceKey,
    /// The face's outer loop: [`LoopBoundary::Empty`], holding `vertex`.
    pub r#loop: LoopKey,
    /// The new lone vertex (`emanating: None`).
    pub vertex: VertexKey,
    /// The new point carrying the given coordinates.
    pub point: PointKey,
    /// The face's surface: the `Surface::Nurbs` "no description yet"
    /// placeholder (module docs, geometry policy) — attach the real
    /// surface via [`Body::set_face_surface`] before rest.
    pub surface: SurfaceKey,
}

/// Every key minted by one [`Body::mev`] call.
///
/// Direction convention (deviation from Mäntylä's `lmev`, documented in
/// the [module docs](self)): `he_plus` runs **old vertex → new vertex**.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MevCreated {
    /// The new vertex (`emanating` = `he_minus`).
    pub vertex: VertexKey,
    /// The new edge joining the old vertex to `vertex`.
    pub edge: EdgeKey,
    /// The plus half: starts at the **old** vertex, ends at `vertex`
    /// (the edge's intrinsic direction — old → new). Lands in `he1`'s
    /// loop ([`MevSite::Fan`]) or the grown loop ([`MevSite::Lone`]),
    /// spliced immediately before `he1` (before `he_minus` for a strut;
    /// the sole predecessor position for `Lone`).
    pub he_plus: HalfEdgeKey,
    /// The minus half: starts at `vertex`, ends at the old vertex.
    /// Lands in `he2`'s loop, spliced immediately before `he2`
    /// (`Fan`) or as the plus half's cycle partner (`Lone`).
    pub he_minus: HalfEdgeKey,
    /// The new point carrying the given coordinates.
    pub point: PointKey,
    /// The edge's certified curve (the attachment-gated `EdgeCurve`
    /// built from the given spec — M2 geometry policy, module docs).
    pub curve: CurveKey,
}

/// Every key minted by one [`Body::mef`] call.
///
/// Direction convention (Mäntylä's, kept): `he_plus` runs
/// `start(he1) → start(he2)` and lands in the **old** loop; `he_minus`
/// lands in the new face's outer loop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MefCreated {
    /// The new face (outer loop = `r#loop`; surface per the given
    /// [`FaceSurface`]; joins the old face's shell).
    pub face: FaceKey,
    /// The new face's outer loop: `he1`'s side of the split plus
    /// `he_minus` (its `Cycle::first`).
    pub r#loop: LoopKey,
    /// The new edge joining `start(he1)` to `start(he2)`.
    pub edge: EdgeKey,
    /// The plus half: `start(he1) → start(he2)`, in the **old** loop
    /// (which it re-anchors as `Cycle::first`).
    pub he_plus: HalfEdgeKey,
    /// The minus half: `start(he2) → start(he1)`, in the new loop.
    pub he_minus: HalfEdgeKey,
    /// The edge's certified curve (the attachment-gated `EdgeCurve`
    /// built from the given spec — M2 geometry policy, module docs).
    pub curve: CurveKey,
}

/// The validated site data of a [`MevSite::Fan`] application — the
/// output of the shared precondition block
/// ([`Body::mev_fan_plan`]), consumed by the shared surgery
/// ([`Body::mev_fan_execute`]). Crate-internal plumbing between
/// [`Body::mev`] and [`Body::mev_null`].
pub(crate) struct MevFanPlan<T: Real> {
    /// The shared start vertex of `he1`/`he2`.
    pub(crate) v: VertexKey,
    /// `v`'s point (the certification gate's start endpoint; the
    /// null lane's coincident-copy source).
    pub(crate) p_old: Point3<T>,
    /// The clockwise orbit run `[he1 .. he2)` to reassign.
    pub(crate) run: Vec<HalfEdgeKey>,
    /// The two fan half-edges, proven live.
    pub(crate) he1: Live,
    /// See [`MevFanPlan::he1`].
    pub(crate) he2: Live,
    /// `prev(he1)`, proven live.
    pub(crate) he1_prev: Live,
    /// `prev(he2)`, proven live.
    pub(crate) he2_prev: Live,
    /// `he1`'s parent loop.
    pub(crate) he1_loop: LoopKey,
    /// `he2`'s parent loop.
    pub(crate) he2_loop: LoopKey,
}

/// What the shared mev surgery mints into the curve arena — a
/// certified carrier ([`Body::mev`]) or the F9 null-scaffold entry
/// ([`Body::mev_null`]). Selects the documented minting order (see
/// [`Body::mev_fan_execute`]).
pub(crate) enum MevCurveMint<T: Real> {
    /// A certified carrier (the gate already ran).
    Certified(EdgeCurve<T>),
    /// A null-edge scaffolding entry; the payload declares which side
    /// the new vertex faces.
    Null(crate::null::NewVertexSide),
}

/// A failed Euler-operator precondition. Closed enum (D3 style); the
/// body is untouched whenever one of these is returned (the operators
/// are atomic).
///
/// The `Fan`/`Loop` "broken" variants are reachable only on
/// tier-1-invalid input: the operators assume euler-valid bodies and
/// surface the corruption a plan phase can observe as typed errors
/// instead of producing garbage (never a panic or a hang, per D9).
/// Detectability, not cost, is the line — the D2 addendum's rows 4/5
/// split on re-derivation.
///
/// (`Eq` was dropped at M2 PR 3: [`EulerOpError::Certification`]
/// carries margin diagnostics with `f64` payloads.)
#[derive(Clone, Debug, PartialEq)]
pub enum EulerOpError {
    /// The curve-geometry spec failed its D4 ¶2 certification at the
    /// attachment gate (residual exceeded, sliver escalation,
    /// unresolved/unimplemented described surface, …) — the typed
    /// operation-time failure of D4 ¶3. The body is untouched.
    Certification {
        /// The certification failure.
        error: CertifyError,
    },
    /// [`Body::set_edge_curve`]: an intrinsic (`Intersection`) or
    /// `Seam` description's surface keys do not match the edge's two
    /// adjacent faces' surfaces — the description does not describe
    /// *this* edge's locus (D2: an intersection edge's surfaces are its
    /// adjacent faces'; a seam's surface is on both sides).
    DescriptionNotAdjacent {
        /// The edge whose description is incoherent with its faces.
        edge: EdgeKey,
    },
    /// An argument key, or a key the operator must follow to do its
    /// work (a `prev` link, a spine parent, a start vertex), does not
    /// resolve.
    StaleKey {
        /// The unresolvable reference, wrapped with its kind.
        key: EntityId,
    },
    /// A geometry key the operator must read (an endpoint vertex's
    /// point for the certification gate, a `FaceSurface::Shared` key)
    /// does not resolve.
    StaleGeometry {
        /// The unresolvable geometry reference.
        key: GeomRef,
    },
    /// [`MevSite::Fan`]'s half-edges start at different vertices — there
    /// is no shared fan to split.
    FanStartMismatch {
        /// The first half-edge.
        he1: HalfEdgeKey,
        /// The second half-edge, starting elsewhere.
        he2: HalfEdgeKey,
    },
    /// The clockwise orbit walk from `he1` failed to close, or closed
    /// without visiting `he2` (despite the matching start vertex) —
    /// tier-1-invalid input.
    FanOrbitBroken {
        /// The half-edge the orbit was walked from.
        he1: HalfEdgeKey,
        /// The half-edge the walk never reached.
        he2: HalfEdgeKey,
    },
    /// The two half-edge arguments belong to different loops where one
    /// loop is required: [`MefSite::Chords`] splits one loop, and
    /// [`Body::kemr`] kills an edge occurring twice in one loop (joining
    /// two loops of a face is [`Body::mekr`]; joining two faces' loops
    /// is not an Euler op at all).
    NotSameLoop {
        /// The first half-edge.
        he1: HalfEdgeKey,
        /// The second half-edge, in a different loop.
        he2: HalfEdgeKey,
    },
    /// A cycle walk failed to close, or closed without visiting the
    /// half-edge it had to reach (despite matching parent-loop keys) —
    /// tier-1-invalid input.
    LoopCycleBroken {
        /// The loop whose cycle is broken.
        r#loop: LoopKey,
    },
    /// A site named a loop that must be [`LoopBoundary::Empty`] but is
    /// not: the `Lone` sites of `mev`/`mef` and the `Empty*` sites of
    /// [`Body::mekr`] apply to empty loops only, and [`Body::kvfs`]'s
    /// skeletal face must have an empty outer loop.
    LoopNotEmpty {
        /// The non-empty loop.
        r#loop: LoopKey,
    },
    /// A loop that a half-edge argument claims as parent is
    /// [`LoopBoundary::Empty`] — tier-1-invalid input (an empty loop
    /// reaches no half-edges).
    LoopNotCycle {
        /// The empty loop claimed as parent.
        r#loop: LoopKey,
    },
    /// [`Body::kemr`]'s half-edges are not the two halves of one edge:
    /// the keys are equal, their `edge` fields differ, or the edge does
    /// not claim exactly them in its two slots (the last is a corrupt
    /// bijection — tier-1-invalid input).
    NotSameEdge {
        /// The first half-edge.
        he1: HalfEdgeKey,
        /// The second half-edge.
        he2: HalfEdgeKey,
    },
    /// A half-edge argument's own edge does not claim it in either slot,
    /// so its mate cannot be resolved — a corrupt edge ↔ half-edge
    /// bijection, tier-1-invalid input. Fired by the single-half-edge
    /// kill operators ([`Body::kev`], [`Body::kef`]), whose mate is
    /// computed rather than passed.
    UnclaimedHalfEdge {
        /// The half-edge its own edge does not claim.
        he: HalfEdgeKey,
        /// The edge that fails to claim it.
        edge: EdgeKey,
    },
    /// [`Body::kev`]'s edge is a self-loop — both endpoints are one
    /// vertex, so there is no far vertex to kill and no fan to merge.
    /// `kev` requires distinct end vertices (Mäntylä §9.2.3); a
    /// self-loop edge is killed by [`Body::kef`] (its two sides border
    /// distinct faces) or [`Body::kemr`] (it occurs twice in one loop).
    SelfLoopEdge {
        /// The self-loop edge.
        edge: EdgeKey,
        /// The single vertex both its endpoints name.
        vertex: VertexKey,
    },
    /// The clockwise vertex orbit walked from `he` failed to close —
    /// tier-1-invalid input (fired by [`Body::kev`], which walks the far
    /// vertex's whole fan; the mev-specific target-missing form is
    /// [`EulerOpError::FanOrbitBroken`]).
    OrbitBroken {
        /// The half-edge whose start vertex's orbit is broken.
        he: HalfEdgeKey,
    },
    /// The operation would leave two [`LoopBoundary::Empty`] loops
    /// holding the same lone vertex, which tier 1 forbids (a vertex is
    /// the lone vertex of *exactly one* empty loop). Fired by
    /// [`Body::kemr`] when both split components are empty and would
    /// anchor at one vertex (a segment loop whose edge is a self-loop)
    /// and by [`Body::mekr`]'s `BothEmpty` site when the two lone
    /// vertices coincide. Believed unreachable through valid operator
    /// sequences (the offending inputs are already tier-1-invalid);
    /// checked defensively.
    EmptyAnchorsCollide {
        /// The vertex both empty loops would anchor at.
        vertex: VertexKey,
    },
    /// Two distinct loops are required but one loop was found:
    /// [`Body::mekr`]'s target and ring anchors name the same loop
    /// (`mekr` joins two *distinct* loops of a face), or
    /// [`Body::kef`]'s edge occurs twice in one loop — killing such an
    /// edge splits the loop instead of merging two faces, which is
    /// [`Body::kemr`]'s job.
    SameLoop {
        /// The loop named twice.
        r#loop: LoopKey,
    },
    /// [`Body::mekr`]'s two loops belong to different faces — `mekr`
    /// merges loops of a single face.
    NotSameFace {
        /// The target-side loop.
        target: LoopKey,
        /// The ring-side loop, in a different face.
        ring: LoopKey,
    },
    /// A loop named as a ring is its face's outer loop:
    /// [`Body::mekr`]'s ring argument, [`Body::ring_move`]'s ring, and
    /// [`Body::mfkrh`]'s ring must be interior loops (members of
    /// [`crate::Face::rings`]).
    RingIsOuter {
        /// The loop that is an outer loop, not a ring.
        r#loop: LoopKey,
    },
    /// Two distinct faces are required but one face was found:
    /// [`Body::kfmrh`]'s two face arguments are the same face (the
    /// connected sum needs two distinct faces), or [`Body::kef`]'s
    /// edge's two halves lie in different loops of ONE face — there is
    /// no second face to kill. (That configuration is what
    /// [`Body::kfmrh`] on two ADJACENT faces leaves behind: the shared
    /// edge's other half ends up in the demoted ring. Kill such an edge
    /// with [`Body::kev`] when its endpoints are distinct; the
    /// self-loop variant has no direct one-op killer — promote the ring
    /// back out with [`Body::mfkrh`], then [`Body::kef`].)
    SameFace {
        /// The face named twice.
        face: FaceKey,
    },
    /// The two faces lie in different shells where one shell is
    /// required. Since M3 PR 1 [`Body::kfmrh`] **accepts** cross-shell
    /// faces (same solid) as its shell-fusion form and no longer fires
    /// this; [`Body::ring_move`] still only reparents within one shell
    /// (cross-shell ring re-homing has no ch. 14/15 consumer — a ring
    /// moves between faces of one shell after splits change
    /// containment).
    CrossShell {
        /// The first face (ring_move's source face).
        f1: FaceKey,
        /// The second face, in a different shell.
        f2: FaceKey,
    },
    /// A face that must be ring-free still has rings: [`Body::kfmrh`]'s
    /// `f2` and [`Body::kef`]'s dying face are demoted/killed whole, and
    /// [`Body::kvfs`]'s single face must be the bare skeletal face — in
    /// every case the caller must move rings off first (via
    /// [`Body::ring_move`]; for `kef`, killing the mate half kills the
    /// other side instead, which may already be ring-free).
    FaceHasRings {
        /// The face with rings.
        face: FaceKey,
    },
    /// [`Body::kvfs`]'s solid does not have exactly one shell — the
    /// skeletal `mvfs` state it inverts has one.
    SolidNotSingleShell {
        /// The non-skeletal solid.
        solid: SolidKey,
        /// How many shells it has (≠ 1).
        shells: usize,
    },
    /// [`Body::kvfs`]'s solid's shell does not have exactly one face —
    /// the skeletal `mvfs` state it inverts has one.
    ShellNotSingleFace {
        /// The non-skeletal shell.
        shell: ShellKey,
        /// How many faces it has (≠ 1).
        faces: usize,
    },
    /// An operation requiring a certified carrier met M3 null-edge
    /// scaffolding ([`crate::null`]): the referenced curve entry is
    /// [`crate::CurveGeom::NullScaffold`], which has no carrier by
    /// type. Fired by [`Body::split_edge`] (nothing to split) and by
    /// the sweep upgrade paths (nothing to upgrade).
    NullScaffoldCurve {
        /// The scaffolding curve entry.
        curve: CurveKey,
    },
    /// [`Body::split_edge`]'s parameter is **definitely not interior**
    /// to the edge's certified interval: one of the two sub-spans
    /// `t − t₀` / `t₁ − t` classified non-positive (metered in meters,
    /// like the certification span gate). Splitting at an endpoint (or
    /// outside the interval) is refused — the split point must be a
    /// genuinely interior locus point.
    SplitParamNotInterior {
        /// The edge whose interval excludes the parameter.
        edge: EdgeKey,
    },
    /// [`Body::split_edge`]'s interiority test escalated: a sub-span
    /// margin fell in the tolerance band (the split point is
    /// indistinguishable from an endpoint at this ε) or was poisoned.
    /// Q1 trilean discipline — in-band never silently rounds to either
    /// verdict.
    SplitParamEscalated {
        /// The edge being split.
        edge: EdgeKey,
        /// The in-band/poisoned margin diagnostics.
        diag: geom_core::Indeterminate,
    },
    /// [`Body::kfmrh`]'s two faces lie in different **solids**. The
    /// cross-shell form (M3 PR 1) fuses two shells of one solid; fusing
    /// across solids is the boolean pipeline's combine step (M3 PRs
    /// 4–5), not an Euler surgery.
    CrossSolid {
        /// The first face.
        f1: FaceKey,
        /// The second face, in a different solid.
        f2: FaceKey,
    },
}

impl fmt::Display for EulerOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Certification { error } => {
                write!(f, "geometry attachment gate: {error}")
            }
            Self::DescriptionNotAdjacent { edge } => write!(
                f,
                "edge {edge:?}'s intrinsic/seam description names surfaces that are not \
                 its adjacent faces' surfaces (D2 adjacency coherence)"
            ),
            Self::StaleKey { key } => {
                write!(f, "euler op requires {key}, which does not resolve")
            }
            Self::StaleGeometry { key } => {
                write!(f, "euler op requires {key}, which does not resolve")
            }
            Self::FanStartMismatch { he1, he2 } => write!(
                f,
                "mev fan: half-edges {he1:?} and {he2:?} start at different \
                 vertices"
            ),
            Self::FanOrbitBroken { he1, he2 } => write!(
                f,
                "mev fan: the clockwise vertex orbit from {he1:?} never \
                 reaches {he2:?} (malformed body)"
            ),
            Self::NotSameLoop { he1, he2 } => write!(
                f,
                "half-edges {he1:?} and {he2:?} belong to different loops \
                 (one loop required)"
            ),
            Self::LoopCycleBroken { r#loop } => write!(
                f,
                "loop {loop:?}'s cycle walk never reaches the second \
                 half-edge (malformed body)",
                loop = r#loop
            ),
            Self::LoopNotEmpty { r#loop } => write!(
                f,
                "empty-loop site: loop {loop:?} is not an empty loop",
                loop = r#loop
            ),
            Self::LoopNotCycle { r#loop } => write!(
                f,
                "a half-edge argument claims parent loop {loop:?}, which is \
                 an empty loop (malformed body)",
                loop = r#loop
            ),
            Self::NotSameEdge { he1, he2 } => write!(
                f,
                "kemr: half-edges {he1:?} and {he2:?} are not the two halves \
                 of one edge"
            ),
            Self::UnclaimedHalfEdge { he, edge } => write!(
                f,
                "half-edge {he:?}'s edge {edge:?} does not claim it in either \
                 slot, so its mate cannot be resolved (malformed body)"
            ),
            Self::SelfLoopEdge { edge, vertex } => write!(
                f,
                "kev: edge {edge:?} is a self-loop at vertex {vertex:?} — kev \
                 needs distinct end vertices (kill a self-loop edge with kef \
                 or kemr)"
            ),
            Self::OrbitBroken { he } => write!(
                f,
                "the clockwise vertex orbit from {he:?} fails to close \
                 (malformed body)"
            ),
            Self::EmptyAnchorsCollide { vertex } => write!(
                f,
                "the operation would leave two empty loops holding the same \
                 lone vertex {vertex:?} (tier 1 allows exactly one)"
            ),
            Self::SameLoop { r#loop } => write!(
                f,
                "two distinct loops required, but both sides name loop \
                 {loop:?} (mekr joins two loops of a face; kef on an edge \
                 occurring twice in one loop is kemr's job)",
                loop = r#loop
            ),
            Self::NotSameFace { target, ring } => write!(
                f,
                "mekr: loops {target:?} and {ring:?} belong to different \
                 faces"
            ),
            Self::RingIsOuter { r#loop } => write!(
                f,
                "loop {loop:?} is its face's outer loop, not a ring",
                loop = r#loop
            ),
            Self::SameFace { face } => write!(
                f,
                "two distinct faces required, but both sides name face \
                 {face:?} (kfmrh sums two faces; kef on an edge interior to \
                 one face has no face to kill — see kev)"
            ),
            Self::CrossShell { f1, f2 } => write!(
                f,
                "faces {f1:?} and {f2:?} lie in different shells \
                 (ring_move reparents a ring within one shell only; \
                 cross-shell face merging is kfmrh's shell-fusion form)"
            ),
            Self::FaceHasRings { face } => write!(
                f,
                "face {face:?} still has rings and must be ring-free here \
                 (move them off with ring_move first)"
            ),
            Self::SolidNotSingleShell { solid, shells } => write!(
                f,
                "kvfs: solid {solid:?} has {shells} shells, not the skeletal \
                 single shell"
            ),
            Self::ShellNotSingleFace { shell, faces } => write!(
                f,
                "kvfs: shell {shell:?} has {faces} faces, not the skeletal \
                 single face"
            ),
            Self::NullScaffoldCurve { curve } => write!(
                f,
                "curve {curve:?} is null-edge scaffolding (no carrier by \
                 type); the operation requires a certified carrier"
            ),
            // Definite at ANY magnitude (a parameter far outside the
            // interval fires this same arm), so the coincidence levers
            // are offered conditionally — the unconditional fix is a
            // strictly interior parameter (S6 review, MINOR-2).
            Self::SplitParamNotInterior { edge } => write!(
                f,
                "split_edge: the parameter is definitely not interior to \
                 edge {edge:?}'s certified interval (it coincides with an \
                 endpoint, or lies outside) — pick a parameter strictly \
                 inside the interval; if it was meant to land exactly on \
                 an endpoint, {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::SplitParamEscalated { edge, diag } => write!(
                f,
                "split_edge: interiority test on edge {edge:?} escalated \
                 ({diag})"
            ),
            Self::CrossSolid { f1, f2 } => write!(
                f,
                "kfmrh: faces {f1:?} and {f2:?} lie in different solids \
                 (cross-solid fusion is the boolean combine step, not an \
                 Euler surgery)"
            ),
        }
    }
}

impl std::error::Error for EulerOpError {}

/// One operator's signed shift of the seven topology-arena lengths.
///
/// A different quantity from the six-component Euler vector
/// `(v, e, f, h, r, s)`: Δh is a genus change, not an arena length,
/// and cannot be derived from these seven.
///
/// Call sites name only the nonzero components and take the rest from
/// [`ArenaDelta::ZERO`], so a site reads as the op's actual shift.
#[cfg(debug_assertions)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ArenaDelta {
    pub(crate) solids: isize,
    pub(crate) shells: isize,
    pub(crate) faces: isize,
    pub(crate) loops: isize,
    pub(crate) half_edges: isize,
    pub(crate) edges: isize,
    pub(crate) vertices: isize,
}

#[cfg(debug_assertions)]
impl ArenaDelta {
    /// The shift of an operator that mints and kills nothing.
    pub(crate) const ZERO: Self = Self {
        solids: 0,
        shells: 0,
        faces: 0,
        loops: 0,
        half_edges: 0,
        edges: 0,
        vertices: 0,
    };
}

#[cfg(debug_assertions)]
impl ArenaCounts {
    /// The counts shifted by an op's arena delta. Components are
    /// signed since PR 3's kill-direction ops; an (impossible)
    /// underflow saturates to `usize::MAX`, which the postcondition
    /// assert then reports loudly.
    fn plus(self, delta: ArenaDelta) -> Self {
        let shift = |count: usize, d: isize| count.checked_add_signed(d).unwrap_or(usize::MAX);
        Self {
            solids: shift(self.solids, delta.solids),
            shells: shift(self.shells, delta.shells),
            faces: shift(self.faces, delta.faces),
            loops: shift(self.loops, delta.loops),
            half_edges: shift(self.half_edges, delta.half_edges),
            edges: shift(self.edges, delta.edges),
            vertices: shift(self.vertices, delta.vertices),
        }
    }
}

impl<T: Decide> Body<T> {
    /// MVFS — *make vertex, face, solid*: the initialization operator.
    ///
    /// Creates the skeletal body from scratch: a new solid with one
    /// shell, one face whose outer loop is an **empty loop**
    /// ([`LoopBoundary::Empty`]) holding one lone vertex at `point`.
    /// This is the boundary-model form of the skeletal plane model
    /// (Mäntylä §9.2.2) — the start state of every Euler construction.
    ///
    /// Euler vector: `(v +1, e 0, f +1, h 0, r 0, s +1)` — arena deltas
    /// +1 solid, +1 shell, +1 face, +1 loop, +1 vertex. (The shell is
    /// our entity, absent in GWB, where a "solid" is one connected
    /// boundary; a solid here may hold several, and does whenever a
    /// boolean leaves a void shell — [`crate::Solid`].)
    ///
    /// **Minting order** (D9, exact): point, surface, vertex, solid,
    /// shell, loop, face. The seed face's surface is the
    /// [`Surface::Nurbs`] representable-unimplemented placeholder —
    /// the honest "no description yet" state (module docs, geometry
    /// policy): a construction attaches the real surface via
    /// [`Body::set_face_surface`] once it exists; a body reaching rest
    /// with it fails tier 3.
    ///
    /// # Errors
    ///
    /// None today — `mvfs` has no preconditions (it consumes nothing).
    /// The `Result` keeps the operator signatures uniform.
    pub fn mvfs(&mut self, point: Point3<T>) -> Result<MvfsCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let point_key = self.add_point(point);
        let surface = self.add_surface(Surface::nurbs_placeholder());
        let vertex = self.add_vertex(
            Vertex {
                point: point_key,
                emanating: None,
            },
            Provenance::Mvfs,
        );
        let solid = self.add_solid(Solid { shells: vec![] }, Provenance::Mvfs);
        let shell = self.add_shell(
            Shell {
                faces: vec![],
                solid,
            },
            Provenance::Mvfs,
        );
        let r#loop = self.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex },
                // Provisional: the face does not exist yet (the loop ↔
                // face references are mutually cyclic); patched below.
                face: FaceKey::default(),
            },
            Provenance::Mvfs,
        );
        let face = self.add_face(
            Face {
                sense: true,
                surface,
                outer: r#loop,
                rings: vec![],
                shell,
            },
            Provenance::Mvfs,
        );
        // Close the cyclic references. Every patched key was minted five
        // lines up; the lookups cannot fail.
        let Some(l) = self.get_loop_mut(r#loop) else {
            unreachable!("mvfs: `r#loop` is minted by this function, above")
        };
        l.face = face;
        let Some(s) = self.get_shell_mut(shell) else {
            unreachable!("mvfs: `shell` is minted by this function, above")
        };
        s.faces.push(face);
        let Some(s) = self.get_solid_mut(solid) else {
            unreachable!("mvfs: `solid` is minted by this function, above")
        };
        s.shells.push(shell);

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                solids: 1,
                shells: 1,
                faces: 1,
                loops: 1,
                vertices: 1,
                ..ArenaDelta::ZERO
            },
            "mvfs",
        );
        Ok(MvfsCreated {
            solid,
            shell,
            face,
            r#loop,
            vertex,
            point: point_key,
            surface,
        })
    }

    /// MEV — *make edge, vertex*: split a vertex's edge fan (or grow a
    /// lone vertex) with a new vertex at `point` joined to the old
    /// vertex by a new edge.
    ///
    /// Site semantics, run direction (clockwise), and the degenerate
    /// strut/segment cases: [`MevSite`]. Direction convention:
    /// `he_plus` runs **old → new** (deviation from Mäntylä's `lmev`,
    /// see the [module docs](self)).
    ///
    /// Euler vector: `(v +1, e +1, f 0, h 0, r 0, s 0)` — arena deltas
    /// +1 vertex, +1 edge, +2 half-edges.
    ///
    /// **Geometry** (module docs, M2 policy): `curve` is certified
    /// against the endpoints **old vertex's point → `point`** (the
    /// `he_plus` forward order — `he_plus` runs old → new) before any
    /// mutation; failure is [`EulerOpError::Certification`], body
    /// untouched. Chord-line sugar: [`Body::mev_line`].
    ///
    /// **The moved run's carriers are NOT re-described.** At a fan site
    /// the run `[he1 .. he2)` is re-based onto the new vertex `w`, and
    /// each of those edges keeps the curve it was certified with
    /// against its OLD endpoint. If `point` differs from the old
    /// vertex's, every re-based edge is left describing a locus that no
    /// longer ends where the edge does: tier 1 does not constrain it
    /// and no operator re-checks it, tier 3 reports it at rest, and the
    /// next `split_edge` or `set_edge_curve` on such an edge refuses
    /// typed. **Re-describe the moved run** (via
    /// [`Body::set_edge_curve`]) whenever the two points differ — the
    /// same posture as [`Body::set_face_surface`]'s note about
    /// invalidating an adjacent edge's certification.
    ///
    /// **Minting order** (D9, exact): point, curve (the certified
    /// [`EdgeCurve`]), vertex, edge, `he_plus`, `he_minus`.
    ///
    /// **Emanating rule** (deterministic, unconditional): after `mev`,
    /// the old vertex's `emanating` is `he_plus` and the new vertex's is
    /// `he_minus` — whether or not the fan move stripped the old
    /// vertex's previous anchor. (Which half-edge `emanating` names is
    /// documented as arbitrary; the unconditional overwrite keeps the
    /// rule branch-free and replay-deterministic.)
    ///
    /// # Surgery (Fan, `he1 != he2`)
    ///
    /// Everything in the clockwise orbit run `[he1 .. he2)` is
    /// reassigned to start at the new vertex `w`; `he_plus` is spliced
    /// immediately before `he1` (in `he1`'s loop), `he_minus`
    /// immediately before `he2` (in `he2`'s loop):
    ///
    /// ```text
    ///        before                          after
    ///     \  |  /                        \  |  /
    ///      \ | /  ← run [he1..he2)        \ | /
    ///        v                              w  ← new vertex
    ///       / \                        minus↓↑plus  ← new edge
    ///    he2   (rest of fan)                v
    ///                                      / \
    ///                                   he2   (rest of fan)
    /// ```
    ///
    /// For a strut (`he1 == he2`, empty run) both new halves land
    /// before `he1`, plus first: `… → he_plus → he_minus → he1 → …`
    /// (`v → w → v`, the edge traversed twice by one loop).
    ///
    /// # Precondition check order
    ///
    /// `Fan`: `he1` resolves, `he2` resolves ([`EulerOpError::StaleKey`]);
    /// equal start vertices ([`EulerOpError::FanStartMismatch`]); the
    /// start vertex and its point resolve (`StaleKey` /
    /// [`EulerOpError::StaleGeometry`]); the orbit from `he1` reaches
    /// `he2` ([`EulerOpError::FanOrbitBroken`]); both `prev` links
    /// resolve (`StaleKey`). `Lone`: the loop resolves (`StaleKey`); it
    /// is empty ([`EulerOpError::LoopNotEmpty`]); its vertex and point
    /// resolve (`StaleKey` / `StaleGeometry`). Then, for both sites,
    /// the geometry gate: `curve` certifies
    /// ([`EulerOpError::Certification`]).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mev(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let created = match site {
            MevSite::Fan { he1, he2 } => self.mev_fan(site, he1, he2, point, curve, tol),
            MevSite::Lone { r#loop } => self.mev_lone(site, r#loop, point, curve, tol),
        }?;

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                half_edges: 2,
                edges: 1,
                vertices: 1,
                ..ArenaDelta::ZERO
            },
            "mev",
        );
        Ok(created)
    }

    /// [`Body::mev`] with the chord-line spec derived from the site:
    /// the new edge's carrier is the straight chord from the old
    /// vertex's point to `point`
    /// ([`EdgeCurveSpec::line_between`] — the caller asserts the locus
    /// *is* that chord; module docs, geometry policy). Coincident
    /// endpoints fail certification loudly.
    ///
    /// # Errors
    ///
    /// As [`Body::mev`].
    pub fn mev_line(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError> {
        let old = match site {
            MevSite::Fan { he1, .. } => self.resolve_half_edge(he1)?.start,
            MevSite::Lone { r#loop } => {
                let loop_data = self.get_loop(r#loop).ok_or(EulerOpError::StaleKey {
                    key: EntityId::Loop(r#loop),
                })?;
                match loop_data.boundary {
                    LoopBoundary::Empty { vertex } => vertex,
                    LoopBoundary::Cycle { .. } => {
                        return Err(EulerOpError::LoopNotEmpty { r#loop });
                    }
                }
            }
        };
        let p_old = self.resolve_vertex_point(old)?;
        self.mev(site, point, EdgeCurveSpec::line_between(p_old, point), tol)
    }

    /// MEF — *make edge, face*: split a loop (or an empty loop) with a
    /// new edge, creating a new face.
    ///
    /// Site semantics, the `he1`-side-becomes-new-face association, and
    /// the degenerate self-loop/circular cases: [`MefSite`]. Direction
    /// convention: `he_plus` runs `start(he1) → start(he2)` and stays in
    /// the old loop (Mäntylä's association, kept).
    ///
    /// Euler vector: `(v 0, e +1, f +1, h 0, r 0, s 0)` — arena deltas
    /// +1 edge, +1 face, +1 loop, +2 half-edges.
    ///
    /// **Geometry** (module docs, M2 policy): `curve` is certified
    /// against the endpoints `start(he1)`'s point → `start(he2)`'s
    /// point (the `he_plus` forward order) before any mutation;
    /// `surface` supplies the new face's surface per [`FaceSurface`]
    /// (`Inherit` keeps the M1 face-split semantics — two regions of
    /// one surface). Chord-line sugar: [`Body::mef_chord`].
    ///
    /// **Minting order** (D9, exact): surface (only for
    /// [`FaceSurface::New`]), curve (the certified [`EdgeCurve`]),
    /// edge, loop, face, `he_plus`, `he_minus`.
    ///
    /// The new face joins the old face's shell
    /// (membership plus back-pointer). `mef` does **not** reclassify
    /// the old face's rings (they all stay on the old face — Mäntylä
    /// p. 192; the `ring_move` helper is PR 3), and it touches
    /// `emanating` only in the `Lone` case (the lone vertex gains
    /// `he_plus`); in the `Chords` case every vertex keeps its anchor
    /// (no half-edge changes its start vertex).
    ///
    /// Both loops are re-anchored deterministically: the old loop's
    /// `Cycle::first` becomes `he_plus` (its previous first may have
    /// moved to the new loop), the new loop's is `he_minus`.
    ///
    /// # Surgery (Chords, `he1 != he2`)
    ///
    /// The run `[he1 .. he2)` in `next` order moves to the new loop;
    /// `he_minus` is spliced before `he1`, `he_plus` before `he2`, and
    /// the cycle splits in two:
    ///
    /// ```text
    ///          before                            after
    ///    ┌── he1 ──────┐                  ┌── he1 ──────┐
    ///    │             ↓                  │             ↓
    ///    │  (one loop) │            he_minus  NEW LOOP  │
    ///    ↑             │                  ↑             │
    ///    └────── he2 ──┘                  └─────────────┘
    ///                                     ┌── he2 ──────┐
    ///                                     │             ↓
    ///                                he_plus   OLD LOOP │
    ///                                     ↑             │
    ///                                     └─────────────┘
    /// ```
    ///
    /// For `he1 == he2` (empty run) the new loop is the single
    /// self-cycled `he_minus`, and `he_plus` is spliced before `he1` in
    /// the old loop — the one-edge circular face. For [`MefSite::Lone`]
    /// both halves are self-cycled one-half-edge loops at the lone
    /// vertex: the old loop keeps `he_plus`, the new loop `he_minus`.
    ///
    /// # Precondition check order
    ///
    /// `Chords`: `he1` resolves, `he2` resolves
    /// ([`EulerOpError::StaleKey`]); same parent loop
    /// ([`EulerOpError::NotSameLoop`]); the loop resolves (`StaleKey`);
    /// it is a cycle ([`EulerOpError::LoopNotCycle`]); the cycle walk
    /// from `he1` reaches `he2` ([`EulerOpError::LoopCycleBroken`]);
    /// both `prev` links resolve (`StaleKey`); the loop's face and the
    /// face's shell resolve (`StaleKey`); `start(he1)` and its point
    /// resolve (`StaleKey` / [`EulerOpError::StaleGeometry`]);
    /// `start(he2)` and its point resolve (`StaleKey` /
    /// `StaleGeometry`). `Lone`:
    /// the loop resolves; it is empty ([`EulerOpError::LoopNotEmpty`]);
    /// its vertex and point resolve; its face and shell resolve.
    /// Then, for both sites, the geometry gates: a
    /// [`FaceSurface::Shared`] key resolves (`StaleGeometry`) and
    /// `curve` certifies ([`EulerOpError::Certification`]).
    ///
    /// # Errors
    ///
    /// The first failing precondition above; the body is untouched on
    /// `Err`.
    pub fn mef(
        &mut self,
        site: MefSite,
        curve: EdgeCurveSpec<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError> {
        #[cfg(debug_assertions)]
        let before = self.arena_counts();

        let created = match site {
            MefSite::Chords { he1, he2 } => self.mef_chords(site, he1, he2, curve, surface, tol),
            MefSite::Lone { r#loop } => self.mef_lone(site, r#loop, curve, surface, tol),
        }?;

        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(
            before,
            ArenaDelta {
                faces: 1,
                loops: 1,
                half_edges: 2,
                edges: 1,
                ..ArenaDelta::ZERO
            },
            "mef",
        );
        Ok(created)
    }

    /// [`Body::mef`] with derived scaffolding geometry and
    /// [`FaceSurface::Inherit`] — the polyhedral/migration sugar
    /// (module docs, geometry policy):
    ///
    /// - distinct end vertices ⇒ the chord line between their points
    ///   ([`EdgeCurveSpec::line_between`]);
    /// - a self-loop site (`Lone`, `Chords` with `he1 == he2`, or both
    ///   halves starting at one vertex) ⇒ the canonical scaffolding
    ///   circle at the shared point
    ///   ([`EdgeCurveSpec::self_loop_circle_at`]).
    ///
    /// The dispatch is **structural** (key equality, never a scalar
    /// comparison); two *distinct* vertices at coincident coordinates
    /// still take the chord path and fail certification loudly.
    ///
    /// # Errors
    ///
    /// As [`Body::mef`].
    pub fn mef_chord(&mut self, site: MefSite, tol: Tol) -> Result<MefCreated, EulerOpError> {
        let (u1, u2) = match site {
            MefSite::Chords { he1, he2 } => (
                self.resolve_half_edge(he1)?.start,
                self.resolve_half_edge(he2)?.start,
            ),
            MefSite::Lone { r#loop } => {
                let loop_data = self.get_loop(r#loop).ok_or(EulerOpError::StaleKey {
                    key: EntityId::Loop(r#loop),
                })?;
                match loop_data.boundary {
                    LoopBoundary::Empty { vertex } => (vertex, vertex),
                    LoopBoundary::Cycle { .. } => {
                        return Err(EulerOpError::LoopNotEmpty { r#loop });
                    }
                }
            }
        };
        let p1 = self.resolve_vertex_point(u1)?;
        let spec = if u1 == u2 {
            EdgeCurveSpec::self_loop_circle_at(p1)
        } else {
            let p2 = self.resolve_vertex_point(u2)?;
            EdgeCurveSpec::line_between(p1, p2)
        };
        self.mef(site, spec, FaceSurface::Inherit, tol)
    }

    /// Finds the half-edge running `from → to` in `face`, or `None`.
    ///
    /// The searching counterpart of GWB's `fhe` (Program 11.9): tests
    /// and hand construction address half-edges by
    /// `(face, start vertex, end vertex)`; operators take half-edge keys
    /// directly (arenas make keys stable handles, so GWB's id-scan layer
    /// is dropped).
    ///
    /// **Deterministic scan order** (D9): the face's outer loop first,
    /// then its rings in list order; within each loop, cycle (`next`)
    /// order starting at the loop's `Cycle::first`. The first match
    /// wins. Empty loops hold no half-edges and are skipped; on a
    /// malformed body, loops whose keys or cycles do not resolve are
    /// skipped too (total, never panics), so `None` means "not found or
    /// not reachable".
    pub fn find_half_edge(
        &self,
        face: FaceKey,
        from: VertexKey,
        to: VertexKey,
    ) -> Option<HalfEdgeKey> {
        let face_data = self.get_face(face)?;
        for loop_key in core::iter::once(face_data.outer).chain(face_data.rings.iter().copied()) {
            let Some(loop_data) = self.get_loop(loop_key) else {
                continue;
            };
            let LoopBoundary::Cycle { first } = loop_data.boundary else {
                continue;
            };
            let Some(cycle) = self.loop_cycle(first) else {
                continue;
            };
            for he in cycle {
                let Some(he_data) = self.get_half_edge(he) else {
                    continue;
                };
                if he_data.start == from && self.half_edge_end(he) == Some(to) {
                    return Some(he);
                }
            }
        }
        None
    }

    // ------------------------------------------------------------------
    // mev implementation
    // ------------------------------------------------------------------

    /// [`MevSite::Fan`] — precondition block, then the fan surgery.
    fn mev_fan(
        &mut self,
        site: MevSite,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
        point: Point3<T>,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError> {
        // ---- Preconditions: no mutation until every check passes. ----
        let plan = self.mev_fan_plan(he1, he2)?;
        // ---- Geometry gate (still no mutation): certify the spec
        // against old point → new point (D4 ¶2 at attachment).
        let certified = self.certify_edge_spec(curve, plan.p_old, point, tol)?;
        // ---- Mutation (infallible from here on). ----
        Ok(self.mev_fan_execute(
            plan,
            point,
            MevCurveMint::Certified(certified),
            Provenance::Mev { site },
        ))
    }

    /// [`MevSite::Fan`]'s precondition block, shared by [`Body::mev`]
    /// and [`Body::mev_null`] (which replaces the geometry gate with a
    /// null-scaffold mint). Pure — no mutation.
    pub(crate) fn mev_fan_plan(
        &self,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
    ) -> Result<MevFanPlan<T>, EulerOpError> {
        let (he1_live, he1_data) = self.resolve_half_edge_live(he1)?;
        let (v, he1_prev, he1_loop) = (he1_data.start, he1_data.prev, he1_data.parent_loop);
        let (he2_live, he2_data) = self.resolve_half_edge_live(he2)?;
        let (he2_start, he2_prev, he2_loop) = (he2_data.start, he2_data.prev, he2_data.parent_loop);
        if he2_start != v {
            return Err(EulerOpError::FanStartMismatch { he1, he2 });
        }
        // The op rewrites v's emanating; a dangling start vertex (or
        // point) is tier-1-invalid input caught here. The point is the
        // certification's start endpoint (he_plus runs old → new).
        let p_old = self.resolve_vertex_point(v)?;
        // The clockwise run [he1 .. he2): members of the next(mate(·))
        // orbit walk. The walk is bounded (D9) and resolves every member
        // it returns.
        let run: Vec<HalfEdgeKey> = if he1 == he2 {
            Vec::new() // strut: empty run, no walk needed
        } else {
            let orbit = self
                .vertex_orbit(he1)
                .ok_or(EulerOpError::FanOrbitBroken { he1, he2 })?;
            let position = orbit
                .iter()
                .position(|&he| he == he2)
                .ok_or(EulerOpError::FanOrbitBroken { he1, he2 })?;
            orbit[..position].to_vec()
        };
        // The splice writes through both prev links; prove them now so
        // the mutation below cannot fail midway (atomicity).
        let he1_prev = self.require_live(he1_prev)?;
        let he2_prev = self.require_live(he2_prev)?;
        Ok(MevFanPlan {
            v,
            p_old,
            run,
            he1: he1_live,
            he2: he2_live,
            he1_prev,
            he2_prev,
            he1_loop,
            he2_loop,
        })
    }

    /// The fan surgery (infallible mutation phase), shared by
    /// [`Body::mev`] and [`Body::mev_null`]. The minting order follows
    /// the payload: `Certified` mints point, curve, vertex (mev's
    /// documented order); `Null` mints point, vertex, curve — the
    /// scaffolding entry's F9 attribute names the new vertex, so the
    /// vertex must exist first (mev_null's documented order).
    pub(crate) fn mev_fan_execute(
        &mut self,
        plan: MevFanPlan<T>,
        point: Point3<T>,
        mint: MevCurveMint<T>,
        provenance: Provenance,
    ) -> MevCreated {
        let MevFanPlan {
            v,
            p_old: _,
            run,
            he1,
            he2,
            he1_prev,
            he2_prev,
            he1_loop,
            he2_loop,
        } = plan;
        let point_key = self.add_point(point);
        let (curve, w) = self.mint_mev_vertex_and_curve(point_key, v, mint, &provenance);
        let edge = self.mint_edge(curve, &provenance);
        let (he_plus, he_minus) = self.mint_halves(
            edge,
            // he_plus: old vertex → new vertex (our deviation from
            // lmev's new → old), spliced into he1's loop.
            (v, he1_loop),
            // he_minus: new vertex → old vertex, spliced into he2's loop.
            (w, he2_loop),
            &provenance,
        );

        // Splice. Derived (module docs) rather than transcribed; the two
        // cases are the sequential "insert before he1, then before he2"
        // with the strut's second insertion landing between the first
        // and he1.
        if he1 == he2 {
            // Strut: … → prev → he_plus → he_minus → he1 → …
            self.link_half_edges(he1_prev, he_plus);
            self.link_half_edges(he_plus, he_minus);
            self.link_half_edges(he_minus, he1);
        } else {
            // … → prev(he1) → he_plus → he1 → …  (in he1's loop)
            // … → prev(he2) → he_minus → he2 → … (in he2's loop)
            self.link_half_edges(he1_prev, he_plus);
            self.link_half_edges(he_plus, he1);
            self.link_half_edges(he2_prev, he_minus);
            self.link_half_edges(he_minus, he2);
        }
        // The splice is done; past it the halves are ordinary keys.
        let (he_plus, he_minus) = (he_plus.key(), he_minus.key());
        // Reassign the clockwise run to the new vertex.
        for &moved in &run {
            let Some(he) = self.get_half_edge_mut(moved) else {
                unreachable!(
                    "mev fan: run members proven live by mev_fan_plan's bounded orbit walk"
                )
            };
            he.start = w;
        }
        // Emanating rule (documented on `mev`): unconditional.
        let Some(vertex) = self.get_vertex_mut(v) else {
            unreachable!("mev fan: `v` proven live by mev_fan_plan (resolve_vertex_point)")
        };
        vertex.emanating = Some(he_plus);
        let Some(vertex) = self.get_vertex_mut(w) else {
            unreachable!("mev fan: `w` was minted by mint_mev_vertex_and_curve")
        };
        vertex.emanating = Some(he_minus);

        MevCreated {
            vertex: w,
            edge,
            he_plus,
            he_minus,
            point: point_key,
            curve,
        }
    }

    /// [`MevSite::Lone`] — precondition block, then the segment surgery.
    fn mev_lone(
        &mut self,
        site: MevSite,
        loop_key: LoopKey,
        point: Point3<T>,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError> {
        // ---- Preconditions. ----
        let (v, p_old) = self.mev_lone_plan(loop_key)?;
        // ---- Geometry gate (still no mutation). ----
        let certified = self.certify_edge_spec(curve, p_old, point, tol)?;
        // ---- Mutation (infallible from here on). ----
        Ok(self.mev_lone_execute(
            loop_key,
            v,
            point,
            MevCurveMint::Certified(certified),
            Provenance::Mev { site },
        ))
    }

    /// [`MevSite::Lone`]'s precondition block, shared by [`Body::mev`]
    /// and [`Body::mev_null`]: the loop resolves and is empty; its
    /// vertex and point resolve. Pure — no mutation. Returns the lone
    /// vertex and its point.
    pub(crate) fn mev_lone_plan(
        &self,
        loop_key: LoopKey,
    ) -> Result<(VertexKey, Point3<T>), EulerOpError> {
        let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(loop_key),
        })?;
        let LoopBoundary::Empty { vertex: v } = loop_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: loop_key });
        };
        let p_old = self.resolve_vertex_point(v)?;
        Ok((v, p_old))
    }

    /// The lone-site surgery (infallible mutation phase), shared by
    /// [`Body::mev`] and [`Body::mev_null`]. Minting order per the
    /// payload as on [`Body::mev_fan_execute`].
    pub(crate) fn mev_lone_execute(
        &mut self,
        loop_key: LoopKey,
        v: VertexKey,
        point: Point3<T>,
        mint: MevCurveMint<T>,
        provenance: Provenance,
    ) -> MevCreated {
        let point_key = self.add_point(point);
        let (curve, w) = self.mint_mev_vertex_and_curve(point_key, v, mint, &provenance);
        let edge = self.mint_edge(curve, &provenance);
        let (he_plus, he_minus) = self.mint_halves(edge, (v, loop_key), (w, loop_key), &provenance);
        // The two halves form the whole cycle: v → w → v.
        self.link_half_edges(he_plus, he_minus);
        self.link_half_edges(he_minus, he_plus);
        // The splice is done; past it the halves are ordinary keys.
        let (he_plus, he_minus) = (he_plus.key(), he_minus.key());
        let Some(l) = self.get_loop_mut(loop_key) else {
            unreachable!("mev lone: `loop_key` proven live by mev_lone_plan")
        };
        l.boundary = LoopBoundary::Cycle { first: he_plus };
        let Some(vertex) = self.get_vertex_mut(v) else {
            unreachable!("mev lone: `v` proven live by mev_lone_plan (resolve_vertex_point)")
        };
        vertex.emanating = Some(he_plus);
        let Some(vertex) = self.get_vertex_mut(w) else {
            unreachable!("mev lone: `w` was minted by mint_mev_vertex_and_curve")
        };
        vertex.emanating = Some(he_minus);

        MevCreated {
            vertex: w,
            edge,
            he_plus,
            he_minus,
            point: point_key,
            curve,
        }
    }

    /// Mints the new vertex and the curve entry per the payload's
    /// documented order ([`Body::mev_fan_execute`]): `Certified` mints
    /// curve, then vertex; `Null` mints vertex, then the scaffolding
    /// entry whose F9 attribute names old (`v`) and new vertices per
    /// the declared side.
    fn mint_mev_vertex_and_curve(
        &mut self,
        point_key: crate::geometry::PointKey,
        v: VertexKey,
        mint: MevCurveMint<T>,
        provenance: &Provenance,
    ) -> (crate::geometry::CurveKey, VertexKey) {
        let vertex = |point| Vertex {
            point,
            emanating: None, // patched by the caller's surgery
        };
        match mint {
            MevCurveMint::Certified(certified) => {
                let curve = self.add_curve(certified);
                let w = self.add_vertex(vertex(point_key), provenance.clone());
                (curve, w)
            }
            MevCurveMint::Null(side) => {
                let w = self.add_vertex(vertex(point_key), provenance.clone());
                let attr = match side {
                    crate::null::NewVertexSide::Above => crate::null::NullEdge {
                        below_end: v,
                        above_end: w,
                    },
                    crate::null::NewVertexSide::Below => crate::null::NullEdge {
                        below_end: w,
                        above_end: v,
                    },
                };
                (self.add_null_curve(attr), w)
            }
        }
    }

    // ------------------------------------------------------------------
    // mef implementation
    // ------------------------------------------------------------------

    /// [`MefSite::Chords`] — precondition block, then the loop-split
    /// surgery.
    fn mef_chords(
        &mut self,
        site: MefSite,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
        curve: EdgeCurveSpec<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError> {
        // ---- Preconditions. ----
        let (he1_live, he1_data) = self.resolve_half_edge_live(he1)?;
        let (u1, he1_prev, loop_key) = (he1_data.start, he1_data.prev, he1_data.parent_loop);
        let (he2_live, he2_data) = self.resolve_half_edge_live(he2)?;
        let (u2, he2_prev) = (he2_data.start, he2_data.prev);
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
        // The run [he1 .. he2) in next order — he1's side of the split.
        // Walked from he1 itself (not the loop's first), bounded (D9).
        let run: Vec<HalfEdgeKey> = if he1 == he2 {
            Vec::new() // circular one-edge face: empty run
        } else {
            let cycle = self
                .loop_cycle(he1)
                .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
            let position = cycle
                .iter()
                .position(|&he| he == he2)
                .ok_or(EulerOpError::LoopCycleBroken { r#loop: loop_key })?;
            cycle[..position].to_vec()
        };
        // The splice writes through both prev links; prove them now so
        // the mutation below cannot fail midway (atomicity).
        let he1_prev = self.require_live(he1_prev)?;
        let he2_prev = self.require_live(he2_prev)?;
        let face_data = self.get_face(face_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face_key),
        })?;
        let (inherit_surface, inherit_sense, shell_key) =
            (face_data.surface, face_data.sense, face_data.shell);
        if !self.shells.contains_key(shell_key) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Shell(shell_key),
            });
        }
        let p1 = self.resolve_vertex_point(u1)?;
        // he_minus is minted with start = u2; its point is the
        // certification's end endpoint (he_plus runs u1 → u2).
        let p2 = self.resolve_vertex_point(u2)?;
        // ---- Geometry gates (still no mutation). ----
        self.check_face_surface(&surface)?;
        let certified = self.certify_edge_spec(curve, p1, p2, tol)?;

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented on `mef`): surface (for New),
        // curve, edge, loop, face, he_plus, he_minus.
        let provenance = Provenance::Mef { site };
        let (surface, sense) =
            self.mint_face_surface_and_sense(surface, inherit_surface, inherit_sense);
        let curve = self.add_curve(certified);
        let edge = self.mint_edge(curve, &provenance);
        let (new_loop, new_face) = self.mint_loop_and_face(surface, sense, shell_key, &provenance);
        let (he_plus, he_minus) = self.mint_halves(
            edge,
            // he_plus: start(he1) → start(he2), in the OLD loop.
            (u1, loop_key),
            // he_minus: start(he2) → start(he1), in the NEW loop.
            (u2, new_loop),
            &provenance,
        );

        // Splice (derivation in the module docs — Mäntylä's tail swap,
        // re-derived): he_minus closes he1's side into the new loop,
        // he_plus closes he2's side into the old loop.
        if he1 == he2 {
            // Circular one-edge face: the new loop is he_minus alone.
            self.link_half_edges(he_minus, he_minus);
            self.link_half_edges(he1_prev, he_plus);
            self.link_half_edges(he_plus, he1_live);
        } else {
            // New loop: … → prev(he2) → he_minus → he1 → … (he1's side)
            // Old loop: … → prev(he1) → he_plus → he2 → … (he2's side)
            self.link_half_edges(he2_prev, he_minus);
            self.link_half_edges(he_minus, he1_live);
            self.link_half_edges(he1_prev, he_plus);
            self.link_half_edges(he_plus, he2_live);
        }
        // The splice is done; past it the halves are ordinary keys.
        let (he_plus, he_minus) = (he_plus.key(), he_minus.key());
        // Move he1's side into the new loop.
        for &moved in &run {
            let Some(he) = self.get_half_edge_mut(moved) else {
                unreachable!(
                    "mef chords: the run's members were resolved by the plan phase's bounded walk"
                )
            };
            he.parent_loop = new_loop;
        }
        // Re-anchor both loops deterministically (the old loop's first
        // may have migrated to the new loop).
        let Some(l) = self.get_loop_mut(loop_key) else {
            unreachable!("mef chords: `loop_key` proven live by this function's plan phase")
        };
        l.boundary = LoopBoundary::Cycle { first: he_plus };
        let Some(l) = self.get_loop_mut(new_loop) else {
            unreachable!("mef chords: the loop was minted by mint_loop_and_face")
        };
        l.boundary = LoopBoundary::Cycle { first: he_minus };

        Ok(MefCreated {
            face: new_face,
            r#loop: new_loop,
            edge,
            he_plus,
            he_minus,
            curve,
        })
    }

    /// [`MefSite::Lone`] — precondition block, then the circular-edge
    /// surgery ("circular edge from a lone vertex", Mäntylä Fig. 9.8b).
    fn mef_lone(
        &mut self,
        site: MefSite,
        loop_key: LoopKey,
        curve: EdgeCurveSpec<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError> {
        // ---- Preconditions. ----
        let loop_data = self.get_loop(loop_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Loop(loop_key),
        })?;
        let LoopBoundary::Empty { vertex: v } = loop_data.boundary else {
            return Err(EulerOpError::LoopNotEmpty { r#loop: loop_key });
        };
        let face_key = loop_data.face;
        let anchor = self.resolve_vertex_point(v)?;
        let face_data = self.get_face(face_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face_key),
        })?;
        let (inherit_surface, inherit_sense, shell_key) =
            (face_data.surface, face_data.sense, face_data.shell);
        if !self.shells.contains_key(shell_key) {
            return Err(EulerOpError::StaleKey {
                key: EntityId::Shell(shell_key),
            });
        }
        // ---- Geometry gates (still no mutation): the self-loop edge
        // closes at the lone vertex — both endpoints are its point.
        self.check_face_surface(&surface)?;
        let certified = self.certify_edge_spec(curve, anchor, anchor, tol)?;

        // ---- Mutation (infallible from here on). ----
        // Same minting order as Chords: surface (for New), curve,
        // edge, loop, face, he_plus, he_minus.
        let provenance = Provenance::Mef { site };
        let (surface, sense) =
            self.mint_face_surface_and_sense(surface, inherit_surface, inherit_sense);
        let curve = self.add_curve(certified);
        let edge = self.mint_edge(curve, &provenance);
        let (new_loop, new_face) = self.mint_loop_and_face(surface, sense, shell_key, &provenance);
        let (he_plus, he_minus) = self.mint_halves(edge, (v, loop_key), (v, new_loop), &provenance);
        // Both halves are one-half-edge loops at v: the old loop keeps
        // he_plus, the new face's outer loop gets he_minus (the same
        // association as Chords — he1's "side" is the new loop).
        self.link_half_edges(he_plus, he_plus);
        self.link_half_edges(he_minus, he_minus);
        // The splice is done; past it the halves are ordinary keys.
        let (he_plus, he_minus) = (he_plus.key(), he_minus.key());
        let Some(l) = self.get_loop_mut(loop_key) else {
            unreachable!("mef lone: `loop_key` proven live by this function's plan phase")
        };
        l.boundary = LoopBoundary::Cycle { first: he_plus };
        let Some(l) = self.get_loop_mut(new_loop) else {
            unreachable!("mef lone: the loop was minted by mint_loop_and_face")
        };
        l.boundary = LoopBoundary::Cycle { first: he_minus };
        // The lone vertex gains its first half-edge (the only case where
        // mef touches emanating).
        let Some(vertex) = self.get_vertex_mut(v) else {
            unreachable!(
                "mef lone: `v` proven live by this function's plan phase (resolve_vertex_point)"
            )
        };
        vertex.emanating = Some(he_plus);

        Ok(MefCreated {
            face: new_face,
            r#loop: new_loop,
            edge,
            he_plus,
            he_minus,
            curve,
        })
    }

    // ------------------------------------------------------------------
    // Shared internals
    // ------------------------------------------------------------------

    /// Resolves a half-edge argument, copying out its fields
    /// ([`EulerOpError::StaleKey`] if it does not resolve).
    ///
    /// An operator that also splices through the key wants
    /// [`Body::resolve_half_edge_live`], which is this lookup keeping
    /// the proof it earns rather than re-earning it.
    pub(crate) fn resolve_half_edge(&self, he: HalfEdgeKey) -> Result<HalfEdge, EulerOpError> {
        self.resolve_half_edge_live(he).map(|(_, data)| data)
    }

    /// Resolves a vertex's point coordinates (the certification gate's
    /// endpoints), the read-back door's walk with its unresolved
    /// reference renamed: [`EulerOpError::StaleKey`] on the vertex,
    /// [`EulerOpError::StaleGeometry`] on the point.
    pub(crate) fn resolve_vertex_point(
        &self,
        vertex: VertexKey,
    ) -> Result<Point3<T>, EulerOpError> {
        crate::readback::vertex_point_ref(self, vertex).map_err(Into::into)
    }

    /// The attachment gate (D4 ¶2 at operation time): certifies an
    /// [`EdgeCurveSpec`] against the new edge's endpoint points, with
    /// surface keys resolved from this body's arena. Pure (no
    /// mutation) — ops call it inside their precondition phase.
    pub(crate) fn certify_edge_spec(
        &self,
        spec: EdgeCurveSpec<T>,
        p_start: Point3<T>,
        p_end: Point3<T>,
        tol: Tol,
    ) -> Result<EdgeCurve<T>, EulerOpError> {
        let band = Band::linear(tol).map_err(|e| EulerOpError::Certification {
            error: CertifyError::Band(e),
        })?;
        EdgeCurve::certify(
            spec,
            p_start,
            p_end,
            |k| self.surfaces.get(k).cloned(),
            band,
        )
        .map_err(|error| EulerOpError::Certification { error })
    }

    /// Precondition half of [`FaceSurface`] resolution: a `Shared` key
    /// must resolve now (so the mutation phase stays infallible);
    /// `Inherit`/`New` have nothing to check.
    pub(crate) fn check_face_surface(&self, spec: &FaceSurface<T>) -> Result<(), EulerOpError> {
        match spec {
            FaceSurface::Inherit | FaceSurface::New(_) => Ok(()),
            FaceSurface::Shared(key) => {
                if self.surfaces.contains_key(*key) {
                    Ok(())
                } else {
                    Err(EulerOpError::StaleGeometry {
                        key: GeomRef::Surface(*key),
                    })
                }
            }
        }
    }

    /// Mutation half of [`FaceSurface`] resolution: the new face's
    /// surface key — `inherit` for `Inherit`, a fresh insertion for
    /// `New` (part of the op's documented minting order), the given key
    /// for `Shared` (pre-validated by
    /// [`Body::check_face_surface`]).
    pub(crate) fn mint_face_surface(
        &mut self,
        spec: FaceSurface<T>,
        inherit: SurfaceKey,
    ) -> SurfaceKey {
        match spec {
            FaceSurface::Inherit => inherit,
            FaceSurface::New(surface) => self.add_surface(surface),
            FaceSurface::Shared(key) => key,
        }
    }

    /// The new face's surface key and material side when an operator
    /// carves a region off a parent face.
    ///
    /// A fragment that lands on the parent's OWN surface is a piece of
    /// the parent's region — the same surface with the same material
    /// side — so it takes the parent's [`crate::entity::Face::sense`].
    /// A `New` (or foreign `Shared`) surface is not this face's region
    /// at all, and the mint's `true` stands; the caller then attaches
    /// the honest bit through [`crate::Body::set_face_sense`], as the
    /// sweep constructors do. Key equality, never a numeric compare.
    ///
    /// The bit has teeth: a re-mint that stamped `true` unconditionally
    /// would silently reset the material side on every fragment of a
    /// `sense: false` wall, so a boolean split of such a wall would
    /// hand back correctly shaped faces facing the wrong way.
    pub(crate) fn mint_face_surface_and_sense(
        &mut self,
        spec: FaceSurface<T>,
        inherit_surface: SurfaceKey,
        inherit_sense: bool,
    ) -> (SurfaceKey, bool) {
        let surface = self.mint_face_surface(spec, inherit_surface);
        let sense = if surface == inherit_surface {
            inherit_sense
        } else {
            true
        };
        (surface, sense)
    }

    /// Mints an edge with provisional half-edge slots (the halves are
    /// minted next by [`Body::mint_halves`], which patches the slots).
    pub(crate) fn mint_edge(&mut self, curve: CurveKey, provenance: &Provenance) -> EdgeKey {
        self.add_edge(
            Edge {
                // Provisional: the halves do not exist yet; patched by
                // mint_halves.
                he_plus: HalfEdgeKey::default(),
                he_minus: HalfEdgeKey::default(),
                curve,
            },
            provenance.clone(),
        )
    }

    /// Mints an edge's two halves (`he_plus` first, then `he_minus` —
    /// part of every op's documented minting order) and wires the
    /// edge ↔ half-edge bijection. Each half is described by its
    /// `(start vertex, parent loop)` pair; `next`/`prev` are left
    /// provisional (null keys) for the caller's splice — which is why
    /// the halves come back [`Live`]: they were just inserted, and the
    /// caller's next act is to splice them.
    pub(crate) fn mint_halves(
        &mut self,
        edge: EdgeKey,
        plus: (VertexKey, LoopKey),
        minus: (VertexKey, LoopKey),
        provenance: &Provenance,
    ) -> (Live, Live) {
        let half = |(start, parent_loop): (VertexKey, LoopKey)| HalfEdge {
            edge,
            start,
            parent_loop,
            next: HalfEdgeKey::default(), // provisional; caller splices
            prev: HalfEdgeKey::default(), // provisional; caller splices
        };
        let he_plus = self.add_half_edge(half(plus), provenance.clone());
        let he_minus = self.add_half_edge(half(minus), provenance.clone());
        let Some(e) = self.get_edge_mut(edge) else {
            unreachable!(
                "mint_halves: `edge` comes from `mint_edge` in the caller's same mutation phase"
            )
        };
        e.he_plus = he_plus;
        e.he_minus = he_minus;
        let (Some(he_plus), Some(he_minus)) = (Live::of(self, he_plus), Live::of(self, he_minus))
        else {
            unreachable!("mint_halves: both halves were inserted four statements above")
        };
        (he_plus, he_minus)
    }

    /// Mints `mef`'s new loop and face (in that order — part of `mef`'s
    /// documented minting order) and joins the new face to the old
    /// face's shell. The loop's boundary anchor is provisional; the
    /// caller re-anchors it after the splice.
    ///
    /// **`sense` is the caller's inheritance decision**, taken by
    /// [`Body::mint_face_surface_and_sense`], which owns the rule.
    /// `mef` has no *material-side* knowledge of its own — it sees two
    /// chords, not a profile.
    fn mint_loop_and_face(
        &mut self,
        surface: SurfaceKey,
        sense: bool,
        shell: ShellKey,
        provenance: &Provenance,
    ) -> (LoopKey, FaceKey) {
        let new_loop = self.add_loop(
            Loop {
                // Provisional in both fields: the face is minted next
                // (cyclic reference), and the boundary's first half-edge
                // is spliced by the caller.
                boundary: LoopBoundary::Cycle {
                    first: HalfEdgeKey::default(),
                },
                face: FaceKey::default(),
            },
            provenance.clone(),
        );
        let new_face = self.add_face(
            Face {
                sense,   // inherited iff the surface is (fn docs)
                surface, // shared with the old face (M1 geometry policy)
                outer: new_loop,
                rings: vec![],
                shell,
            },
            provenance.clone(),
        );
        let Some(l) = self.get_loop_mut(new_loop) else {
            unreachable!("mint_loop_and_face: `new_loop` is minted by this function, above")
        };
        l.face = new_face;
        let Some(s) = self.get_shell_mut(shell) else {
            unreachable!(
                "mint_loop_and_face: `shell` proven live by the caller's plan phase (mef_chords / mef_lone)"
            )
        };
        s.faces.push(new_face);
        (new_loop, new_face)
    }

    /// Writes the mutual `next`/`prev` link `a → b`.
    ///
    /// **The precondition is the argument type.** Every door that hands
    /// out a [`Live`] performs the lookup — [`Live::of`],
    /// [`Body::require_live`], [`Body::resolve_half_edge_live`],
    /// [`Body::loop_cycle_live`] — so a key nothing has resolved cannot
    /// arrive here. What the token does and does not claim — in
    /// particular that it is a statement about the moment it was made,
    /// and that half-edge removal is therefore the last thing a
    /// mutation phase may do — is the [`live`](crate::live) module
    /// docs.
    ///
    /// **A bounded walk proves its members, not their `prev` fields.**
    /// [`Body::loop_cycle_live`] hands out a token per member and
    /// nothing else. The walk steps `next`, so having walked from `he`
    /// says nothing about `prev(he)`: that key wants
    /// [`Body::require_live`] in the plan phase, like any other
    /// value read out of the arena.
    ///
    /// A failed lookup here is the D2 addendum's row 4 — a token that
    /// outlived the removal of its key. The two arms cannot name their
    /// call site the way a per-site `unreachable!` does, a shared
    /// helper knowing none of its callers, so this is `#[track_caller]`
    /// and the panic reports the caller's location instead.
    #[track_caller]
    pub(crate) fn link_half_edges(&mut self, a: Live, b: Live) {
        let Some(he) = self.get_half_edge_mut(a.key()) else {
            unreachable!("link_half_edges: `a`'s proof outlived its key")
        };
        he.next = b.key();
        let Some(he) = self.get_half_edge_mut(b.key()) else {
            unreachable!("link_half_edges: `b`'s proof outlived its key")
        };
        he.prev = a.key();
    }

    /// D1's ratified postcondition-assert clause: after a successful
    /// operator, the arena deltas must match the [`ArenaDelta`] the op
    /// declares — a different quantity from its Euler vector, which is
    /// prose here and a `seqgen` ledger entry there — and the body must
    /// be tier-1 valid. On tier-1-valid input a failure
    /// here is a kernel bug (a per-call violation of the ch. 9
    /// soundness theorem by our transcription) — and with the raw
    /// builder `pub(crate)` since PR 5, every publicly-constructible
    /// input IS tier-1-valid, so this debug-only panic is unreachable
    /// by input through the public API. It remains reachable from
    /// in-crate raw corruption that slips past an op's preconditions
    /// (e.g. consistently swapped `parent_loop`s); release builds
    /// return a garbage body instead. That residue is corruption the
    /// plan phase cannot see, so every lookup succeeds and writes the
    /// wrong topology — not a discarded lookup: under the D2 addendum
    /// the mutation phases announce an impossible lookup rather than
    /// swallowing it (module docs, operator contracts).
    #[cfg(debug_assertions)]
    pub(crate) fn assert_euler_postcondition(
        &self,
        before: ArenaCounts,
        delta: ArenaDelta,
        op: &str,
    ) {
        debug_assert_eq!(
            self.arena_counts(),
            before.plus(delta),
            "{op} postcondition: arena deltas do not match the op's declared \
             arena delta (kernel bug)",
        );
        debug_assert_eq!(
            crate::validate::validate(self),
            Ok(()),
            "{op} postcondition: result is not tier-1 valid (kernel bug)",
        );
    }
}

/// The **plane × NURBS attach door** (M7-8).
///
/// A described NURBS operand in an `Intersection` certifies only
/// through `geom_brep`'s injected lane, whose derivation needs a
/// CERTIFYING scalar (`geom_brep::EdgeNurbsLane`'s static split; the
/// lane fn's own bound is `Decide + Bounds + CertifiedEnclosure`, and
/// since D1, 2026-08-19, it is that last term rather than `Bounds` that
/// a dual fails). Raising the whole Euler surface to that bound would push it
/// through hundreds of `T: Decide` signatures for a capability three
/// of the four sealed scalars have unconditionally, so the lane is a
/// SEPARATE DOOR onto the same shared machinery: identical
/// preconditions, identical adjacency rules, identical mutation. The
/// default door keeps refusing the class exactly as before — there is
/// no door that accepts it uncertified.
impl<T: geom_brep::EdgeNurbsLane> Body<T> {
    /// [`Body::set_edge_curve`] with the plane × NURBS lane wired in.
    ///
    /// # Errors
    ///
    /// As [`Body::set_edge_curve`].
    pub fn set_edge_curve_nurbs_lane(
        &mut self,
        edge: crate::entity::EdgeKey,
        curve: EdgeCurveSpec<T>,
        tol: Tol,
    ) -> Result<CurveKey, EulerOpError> {
        self.set_edge_curve_via(edge, curve, Self::certify_edge_spec_nurbs_lane, tol)
    }

    /// [`Body::certify_edge_spec`] with the plane × NURBS lane wired in.
    pub(crate) fn certify_edge_spec_nurbs_lane(
        &self,
        spec: EdgeCurveSpec<T>,
        p_start: Point3<T>,
        p_end: Point3<T>,
        tol: Tol,
    ) -> Result<EdgeCurve<T>, EulerOpError> {
        let band = Band::linear(tol).map_err(|e| EulerOpError::Certification {
            error: CertifyError::Band(e),
        })?;
        EdgeCurve::certify_nurbs_lane(
            spec,
            p_start,
            p_end,
            |k| self.surfaces.get(k).cloned(),
            band,
        )
        .map_err(|error| EulerOpError::Certification { error })
    }
}

// Deviation from the PR 2 spec's optional clause, recorded in-tree:
// random-op-sequence property tests are deliberately deferred to PR 4,
// whose make/kill roundtrip properties own the sequence generator.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Point3;
    use geom_core::Tol;

    use super::*;
    use crate::fixtures::{NgonPillow, arena_snapshot, deep_snapshot, pillow, prov};
    use crate::validate::validate;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    /// Runs `op` on `body`, asserts it fails with exactly `expected`,
    /// and asserts the body is untouched: identical arena counts and an
    /// identical spot-checked half-edge (when one exists).
    fn assert_err_and_unchanged(
        body: &mut Body<f64>,
        expected: &EulerOpError,
        op: impl FnOnce(&mut Body<f64>) -> EulerOpError,
    ) {
        let counts_before = arena_snapshot(body);
        let probe = body.half_edges().next().map(|(k, he)| (k, he.clone()));
        let err = op(body);
        assert_eq!(&err, expected);
        assert_eq!(
            arena_snapshot(body),
            counts_before,
            "arena counts changed on Err"
        );
        if let Some((key, before)) = probe {
            let after = body
                .get_half_edge(key)
                .expect("probe key must still resolve");
            assert_eq!(after.edge, before.edge);
            assert_eq!(after.start, before.start);
            assert_eq!(after.parent_loop, before.parent_loop);
            assert_eq!(after.next, before.next);
            assert_eq!(after.prev, before.prev);
        }
    }

    // ------------------------------------------------------------------
    // mvfs
    // ------------------------------------------------------------------

    #[test]
    fn mvfs_creates_the_skeletal_body() {
        let mut body = Body::<f64>::new();
        let c = body.mvfs(Point3::new(1.0, 2.0, 3.0)).unwrap();
        assert_eq!(validate(&body), Ok(()));

        let vertex = body.get_vertex(c.vertex).unwrap();
        assert_eq!(vertex.emanating, None);
        assert_eq!(vertex.point, c.point);
        let point = body.get_point(c.point).unwrap();
        assert_eq!((point.x, point.y, point.z), (1.0, 2.0, 3.0));
        let l = body.get_loop(c.r#loop).unwrap();
        assert_eq!(l.boundary, LoopBoundary::Empty { vertex: c.vertex });
        assert_eq!(l.face, c.face);
        let face = body.get_face(c.face).unwrap();
        assert_eq!(face.outer, c.r#loop);
        assert!(face.rings.is_empty());
        assert_eq!(face.shell, c.shell);
        assert_eq!(face.surface, c.surface);
        let shell = body.get_shell(c.shell).unwrap();
        assert_eq!(shell.faces, vec![c.face]);
        assert_eq!(shell.solid, c.solid);
        assert_eq!(body.get_solid(c.solid).unwrap().shells, vec![c.shell]);

        for id in [
            EntityId::Solid(c.solid),
            EntityId::Shell(c.shell),
            EntityId::Face(c.face),
            EntityId::Loop(c.r#loop),
            EntityId::Vertex(c.vertex),
        ] {
            assert_eq!(body.provenance(id), Some(&Provenance::Mvfs), "for {id}");
        }
    }

    // ------------------------------------------------------------------
    // mev: Lone (segment), Fan strut, Fan split (direction pin)
    // ------------------------------------------------------------------

    #[test]
    fn lone_mev_grows_a_segment() {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let site = MevSite::Lone {
            r#loop: seed.r#loop,
        };
        let seg = body.mev_line(site, p(1.0), Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // he_plus runs OLD vertex → NEW vertex (the documented deviation
        // from Mäntylä's new → old).
        assert_eq!(body.get_half_edge(seg.he_plus).unwrap().start, seed.vertex);
        assert_eq!(body.half_edge_end(seg.he_plus), Some(seg.vertex));
        assert_eq!(body.get_half_edge(seg.he_minus).unwrap().start, seg.vertex);
        assert_eq!(body.half_edge_end(seg.he_minus), Some(seed.vertex));
        // The empty loop became the two-half-edge cycle, anchored at
        // he_plus.
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle { first: seg.he_plus }
        );
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![seg.he_plus, seg.he_minus])
        );
        // Emanating rule: old vertex → he_plus, new vertex → he_minus.
        assert_eq!(
            body.get_vertex(seed.vertex).unwrap().emanating,
            Some(seg.he_plus)
        );
        assert_eq!(
            body.get_vertex(seg.vertex).unwrap().emanating,
            Some(seg.he_minus)
        );
        // The created-keys struct names the real edge slots.
        let edge = body.get_edge(seg.edge).unwrap();
        assert_eq!(edge.he_plus, seg.he_plus);
        assert_eq!(edge.he_minus, seg.he_minus);
        assert_eq!(edge.curve, seg.curve);
        assert_eq!(body.mate(seg.he_plus), Some(seg.he_minus));

        // Typed provenance with the exact site.
        for id in [
            EntityId::Vertex(seg.vertex),
            EntityId::Edge(seg.edge),
            EntityId::HalfEdge(seg.he_plus),
            EntityId::HalfEdge(seg.he_minus),
        ] {
            assert_eq!(
                body.provenance(id),
                Some(&Provenance::Mev { site }),
                "for {id}"
            );
        }
    }

    #[test]
    fn strut_mev_splices_plus_then_minus_before_he1() {
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
        // he1 == he2 at the old vertex: empty run, dangling strut.
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
        assert_eq!(validate(&body), Ok(()));

        // Documented splice: … → he_plus → he_minus → he1 → …, i.e. the
        // loop walks v → w → v → (old segment).
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![
                seg.he_plus,
                seg.he_minus,
                strut.he_plus,
                strut.he_minus
            ])
        );
        // The new vertex dangles: valence 1.
        assert_eq!(
            body.vertex_orbit(strut.he_minus),
            Some(vec![strut.he_minus])
        );
        // The old vertex's orbit gained the strut's plus half.
        assert_eq!(
            body.vertex_orbit(strut.he_plus),
            Some(vec![strut.he_plus, seg.he_plus])
        );
        assert_eq!(body.half_edge_end(strut.he_plus), Some(strut.vertex));
        assert_eq!(body.half_edge_end(strut.he_minus), Some(seed.vertex));
    }

    /// Builds `mvfs + segment + three struts`: a central vertex `v` of
    /// valence 4 whose clockwise orbit is `[pa, pb, pc, pd]` (each `p*`
    /// the plus half of one spoke, minted in that order).
    fn four_spoke_star() -> (Body<f64>, MvfsCreated, [MevCreated; 4]) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
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
        // The construction's clockwise orbit at v (hand-derived from the
        // strut splice, pinned here so the split test below stands on
        // verified ground): pa, pb, pc, pd.
        assert_eq!(
            body.vertex_orbit(a.he_plus),
            Some(vec![a.he_plus, b.he_plus, c.he_plus, d.he_plus])
        );
        (body, seed, [a, b, c, d])
    }

    #[test]
    fn fan_mev_moves_the_clockwise_run_exclusive_of_he2() {
        // THE direction-pinning test. At a valence-4 vertex with
        // clockwise orbit [pa, pb, pc, pd], splitting Fan { he1: pb,
        // he2: pd } must move exactly {pb, pc} — the run walked
        // CLOCKWISE (next ∘ mate) from pb to pd. A counterclockwise walk
        // would move the complementary asymmetric set {pb, pa}; the
        // assertions below distinguish the two.
        let (mut body, seed, [a, b, c, d]) = four_spoke_star();
        let site = MevSite::Fan {
            he1: b.he_plus,
            he2: d.he_plus,
        };
        let split = body.mev_line(site, p(5.0), Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));

        let v = seed.vertex;
        let w = split.vertex;
        let start = |body: &Body<f64>, he| body.get_half_edge(he).unwrap().start;
        // Moved: the clockwise run [pb, pc).. i.e. {pb, pc}.
        assert_eq!(start(&body, b.he_plus), w);
        assert_eq!(start(&body, c.he_plus), w);
        // NOT moved: pa (which the CCW walk would have moved) and pd
        // (exclusive end).
        assert_eq!(start(&body, a.he_plus), v);
        assert_eq!(start(&body, d.he_plus), v);
        // The new edge runs old → new.
        assert_eq!(start(&body, split.he_plus), v);
        assert_eq!(body.half_edge_end(split.he_plus), Some(w));
        // Full orbits after the split, hand-derived: at v the plus half
        // replaces the moved run; at w the minus half precedes it.
        assert_eq!(
            body.vertex_orbit(split.he_plus),
            Some(vec![split.he_plus, d.he_plus, a.he_plus])
        );
        assert_eq!(
            body.vertex_orbit(split.he_minus),
            Some(vec![split.he_minus, b.he_plus, c.he_plus])
        );
        // Emanating rule held on both vertices.
        assert_eq!(body.get_vertex(v).unwrap().emanating, Some(split.he_plus));
        assert_eq!(body.get_vertex(w).unwrap().emanating, Some(split.he_minus));
    }

    #[test]
    fn fan_mev_splices_across_two_loops() {
        // GWB's two-face mev form: he1 and he2 start at the same vertex
        // but sit in DIFFERENT loops (the new halves land in different
        // loops too — he_plus in he1's, he_minus in he2's).
        // Build the digon pillow through the ops (as in the module
        // doctest): at the seed vertex v the orbit is [p, hp2] with p in
        // the new face's loop and hp2 in the old loop.
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
        assert_eq!(
            body.vertex_orbit(seg.he_plus),
            Some(vec![seg.he_plus, split.he_plus])
        );

        let fan = body
            .mev_line(
                MevSite::Fan {
                    he1: seg.he_plus,
                    he2: split.he_plus,
                },
                p(2.0),
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));

        // The run [seg.he_plus] moved to the new vertex.
        assert_eq!(body.get_half_edge(seg.he_plus).unwrap().start, fan.vertex);
        // Each new half landed in its addressing half-edge's loop.
        assert_eq!(
            body.get_half_edge(fan.he_plus).unwrap().parent_loop,
            split.r#loop, // seg.he_plus's loop (the mef moved it there)
        );
        assert_eq!(
            body.get_half_edge(fan.he_minus).unwrap().parent_loop,
            seed.r#loop, // split.he_plus's loop
        );
        // v's orbit swapped the moved half for the new plus half.
        assert_eq!(
            body.vertex_orbit(fan.he_plus),
            Some(vec![fan.he_plus, split.he_plus])
        );
    }

    // ------------------------------------------------------------------
    // mef: self-loop, Lone (circular edge), ring split
    // ------------------------------------------------------------------

    #[test]
    fn self_loop_mef_makes_a_one_edge_circular_face() {
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
        // he1 == he2 at the new vertex: empty run, self-loop face.
        let site = MefSite::Chords {
            he1: seg.he_minus,
            he2: seg.he_minus,
        };
        let circ = body.mef_chord(site, Tol::witness()).unwrap();
        assert_eq!(validate(&body), Ok(()));

        // New face's outer loop: the self-cycled minus half alone.
        assert_eq!(body.loop_cycle(circ.he_minus), Some(vec![circ.he_minus]));
        assert_eq!(
            body.get_loop(circ.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: circ.he_minus
            }
        );
        assert_eq!(body.get_face(circ.face).unwrap().outer, circ.r#loop);
        // Old loop: plus half spliced before he1, both ends at w.
        assert_eq!(
            body.loop_cycle(seg.he_plus),
            Some(vec![seg.he_plus, circ.he_plus, seg.he_minus])
        );
        let w = seg.vertex;
        assert_eq!(body.get_half_edge(circ.he_plus).unwrap().start, w);
        assert_eq!(body.half_edge_end(circ.he_plus), Some(w));
        // New face shares the old face's surface (M1 geometry policy).
        assert_eq!(body.get_face(circ.face).unwrap().surface, seed.surface);
        // Chords never touches emanating.
        assert_eq!(body.get_vertex(w).unwrap().emanating, Some(seg.he_minus));
        // Typed provenance with the exact site.
        assert_eq!(
            body.provenance(EntityId::Face(circ.face)),
            Some(&Provenance::Mef { site })
        );
    }

    #[test]
    fn lone_mef_makes_a_circular_edge_from_a_lone_vertex() {
        // Mäntylä Fig. 9.8(b): lone dot ⇒ circle through the dot — one
        // self-loop edge, two one-half-edge loops, two faces.
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0)).unwrap();
        let circ = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));

        assert_eq!(body.vertices().count(), 1);
        assert_eq!(body.edges().count(), 1);
        assert_eq!(body.faces().count(), 2);
        assert_eq!(body.loops().count(), 2);
        assert_eq!(body.half_edges().count(), 2);
        // Old loop keeps the plus half; the new face's outer loop gets
        // the minus half (he1's "side", degenerately).
        assert_eq!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: circ.he_plus
            }
        );
        assert_eq!(
            body.get_loop(circ.r#loop).unwrap().boundary,
            LoopBoundary::Cycle {
                first: circ.he_minus
            }
        );
        assert_eq!(body.loop_cycle(circ.he_plus), Some(vec![circ.he_plus]));
        assert_eq!(body.loop_cycle(circ.he_minus), Some(vec![circ.he_minus]));
        // The lone vertex gained its first half-edge (the one case where
        // mef touches emanating); its orbit covers both halves.
        assert_eq!(
            body.get_vertex(seed.vertex).unwrap().emanating,
            Some(circ.he_plus)
        );
        assert_eq!(
            body.vertex_orbit(circ.he_plus),
            Some(vec![circ.he_plus, circ.he_minus])
        );
        // Surface shared, shell joined.
        let new_face = body.get_face(circ.face).unwrap();
        assert_eq!(new_face.surface, seed.surface);
        assert_eq!(new_face.shell, seed.shell);
        assert_eq!(
            body.get_shell(seed.shell).unwrap().faces,
            vec![seed.face, circ.face]
        );
    }

    /// The island keys added by [`pillow_with_island`].
    struct Island {
        v2: VertexKey,
        v3: VertexKey,
        /// Island face C's cycle: `c0: v2 → v3`, `c1: v3 → v2`.
        c0: HalfEdgeKey,
        c1: HalfEdgeKey,
        /// Ring R's cycle (their mates): `r0: v3 → v2`, `r1: v2 → v3`.
        r0: HalfEdgeKey,
        r1: HalfEdgeKey,
        /// The ring loop, an interior loop of the pillow's face A.
        ring: LoopKey,
        face_c: FaceKey,
    }

    /// Grafts a digon *island* into the pillow's face A: a floating
    /// two-vertex face C inside A, whose boundary's mates form a RING of
    /// A. The minimal tier-1-valid body with an interior loop (rings
    /// cannot be built through Euler ops until PR 3's kemr, so the raw
    /// builder supplies the fixture).
    fn pillow_with_island() -> (NgonPillow, Island) {
        let mut t = pillow(Tol::witness());
        let body = &mut t.body;
        let null_he = HalfEdgeKey::default();

        let p2 = body.add_point(p(10.0));
        let p3 = body.add_point(p(11.0));
        let v2 = body.add_vertex(
            Vertex {
                point: p2,
                emanating: None,
            },
            prov(),
        );
        let v3 = body.add_vertex(
            Vertex {
                point: p3,
                emanating: None,
            },
            prov(),
        );
        let cu2 = body.add_curve(crate::fixtures::test_curve(p(10.0), Tol::witness()));
        let cu3 = body.add_curve(crate::fixtures::test_curve(p(11.0), Tol::witness()));
        let e2 = body.add_edge(
            Edge {
                he_plus: null_he,
                he_minus: null_he,
                curve: cu2,
            },
            prov(),
        );
        let e3 = body.add_edge(
            Edge {
                he_plus: null_he,
                he_minus: null_he,
                curve: cu3,
            },
            prov(),
        );
        let half = |body: &mut Body<f64>, edge, start| {
            body.add_half_edge(
                HalfEdge {
                    edge,
                    start,
                    parent_loop: LoopKey::default(),
                    next: null_he,
                    prev: null_he,
                },
                prov(),
            )
        };
        let c0 = half(body, e2, v2);
        let c1 = half(body, e3, v3);
        let r0 = half(body, e2, v3);
        let r1 = half(body, e3, v2);
        let ring = body.add_loop(
            Loop {
                boundary: LoopBoundary::Cycle { first: r0 },
                face: t.face_a,
            },
            prov(),
        );
        let loop_c = body.add_loop(
            Loop {
                boundary: LoopBoundary::Cycle { first: c0 },
                face: FaceKey::default(),
            },
            prov(),
        );
        let surface_c = body.add_surface(crate::fixtures::test_surface(p(10.0)));
        let face_c = body.add_face(
            Face {
                sense: true,
                surface: surface_c,
                outer: loop_c,
                rings: vec![],
                shell: t.shell,
            },
            prov(),
        );
        body.get_loop_mut(loop_c).unwrap().face = face_c;
        body.get_shell_mut(t.shell).unwrap().faces.push(face_c);
        body.get_face_mut(t.face_a).unwrap().rings.push(ring);
        // Close the two digon cycles and the bijection.
        for (a, b, parent) in [(c0, c1, loop_c), (r0, r1, ring)] {
            for (x, y) in [(a, b), (b, a)] {
                let he = body.get_half_edge_mut(x).unwrap();
                he.next = y;
                he.prev = y;
                he.parent_loop = parent;
            }
        }
        body.get_edge_mut(e2).unwrap().he_plus = c0;
        body.get_edge_mut(e2).unwrap().he_minus = r0;
        body.get_edge_mut(e3).unwrap().he_plus = c1;
        body.get_edge_mut(e3).unwrap().he_minus = r1;
        body.get_vertex_mut(v2).unwrap().emanating = Some(c0);
        body.get_vertex_mut(v3).unwrap().emanating = Some(c1);

        assert_eq!(validate(&t.body), Ok(()), "fixture must be tier-1 valid");
        (
            t,
            Island {
                v2,
                v3,
                c0,
                c1,
                r0,
                r1,
                ring,
                face_c,
            },
        )
    }

    #[test]
    fn mef_splits_a_ring_and_the_ring_stays_a_ring() {
        let (mut t, island) = pillow_with_island();
        let site = MefSite::Chords {
            he1: island.r0,
            he2: island.r1,
        };
        let split = t.body.mef_chord(site, Tol::witness()).unwrap();
        assert_eq!(validate(&t.body), Ok(()));

        // he1's side (r0) became the NEW face's outer loop...
        assert_eq!(
            t.body.get_half_edge(island.r0).unwrap().parent_loop,
            split.r#loop
        );
        assert_eq!(t.body.get_face(split.face).unwrap().outer, split.r#loop);
        assert_eq!(
            t.body.loop_cycle(split.he_minus),
            Some(vec![split.he_minus, island.r0])
        );
        // ...while the old loop is STILL a ring of face A (mef does not
        // reclassify rings; ring_move is PR 3), re-anchored at he_plus.
        assert_eq!(t.body.get_face(t.face_a).unwrap().rings, vec![island.ring]);
        assert_eq!(t.body.get_face(t.face_a).unwrap().outer, t.loop_a);
        assert_eq!(
            t.body.get_loop(island.ring).unwrap().boundary,
            LoopBoundary::Cycle {
                first: split.he_plus
            }
        );
        assert_eq!(
            t.body.loop_cycle(split.he_plus),
            Some(vec![split.he_plus, island.r1])
        );
        // The new edge joins start(he1) = v3 to start(he2) = v2, plus
        // half in the old loop.
        assert_eq!(
            t.body.get_half_edge(split.he_plus).unwrap().start,
            island.v3
        );
        assert_eq!(t.body.half_edge_end(split.he_plus), Some(island.v2));
        // Shared surface with face A (the split face), same shell.
        assert_eq!(
            t.body.get_face(split.face).unwrap().surface,
            t.body.get_face(t.face_a).unwrap().surface
        );
        assert_eq!(t.body.get_face(split.face).unwrap().shell, t.shell);
    }

    // ------------------------------------------------------------------
    // find_half_edge
    // ------------------------------------------------------------------

    #[test]
    fn find_half_edge_scans_outer_then_rings_in_cycle_order() {
        let (t, island) = pillow_with_island();
        // Outer loop of face A: a0 runs v0 → v1, a1 runs v1 → v0.
        assert_eq!(
            t.body
                .find_half_edge(t.face_a, t.vertices[0], t.vertices[1]),
            Some(t.hes_a[0])
        );
        assert_eq!(
            t.body
                .find_half_edge(t.face_a, t.vertices[1], t.vertices[0]),
            Some(t.hes_a[1])
        );
        // Ring members are found through face A (scanned after the
        // outer loop): r1 runs v2 → v3, r0 runs v3 → v2.
        assert_eq!(
            t.body.find_half_edge(t.face_a, island.v2, island.v3),
            Some(island.r1)
        );
        assert_eq!(
            t.body.find_half_edge(t.face_a, island.v3, island.v2),
            Some(island.r0)
        );
        // The island's own face finds its halves (c0: v2 → v3,
        // c1: v3 → v2).
        assert_eq!(
            t.body.find_half_edge(island.face_c, island.v2, island.v3),
            Some(island.c0)
        );
        assert_eq!(
            t.body.find_half_edge(island.face_c, island.v3, island.v2),
            Some(island.c1)
        );
        // No such half-edge in this face: the pillow's rim pair does not
        // appear in the island face.
        assert_eq!(
            t.body
                .find_half_edge(island.face_c, t.vertices[0], t.vertices[1]),
            None
        );
        // Totality: a stale face key is None, not a panic.
        assert_eq!(
            t.body
                .find_half_edge(FaceKey::default(), island.v2, island.v3),
            None
        );
    }

    // ------------------------------------------------------------------
    // Preconditions: every EulerOpError variant reachable and exact,
    // with the body untouched on Err.
    // ------------------------------------------------------------------

    #[test]
    fn stale_half_edge_key_is_rejected() {
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
        let expected = EulerOpError::StaleKey {
            key: EntityId::HalfEdge(dead),
        };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mev_line(
                MevSite::Fan {
                    he1: dead,
                    he2: dead,
                },
                p(9.0),
                Tol::witness(),
            )
            .unwrap_err()
        });
        // Same rejection through mef's addressing.
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mef_chord(
                MefSite::Chords {
                    he1: dead,
                    he2: dead,
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn stale_loop_key_is_rejected() {
        let mut t = pillow(Tol::witness());
        let dead = t.body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty {
                    vertex: t.vertices[0],
                },
                face: t.face_a,
            },
            prov(),
        );
        t.body.loops.remove(dead);
        let expected = EulerOpError::StaleKey {
            key: EntityId::Loop(dead),
        };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mev_line(MevSite::Lone { r#loop: dead }, p(9.0), Tol::witness())
                .unwrap_err()
        });
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mef_chord(MefSite::Lone { r#loop: dead }, Tol::witness())
                .unwrap_err()
        });
    }

    #[test]
    fn stale_anchor_point_is_rejected_as_stale_geometry() {
        let mut t = pillow(Tol::witness());
        // Corrupt: v0's point is removed; mef needs its coordinates for
        // the placeholder curve anchor.
        let dead_point = t.body.points.remove(t.points[0]);
        assert!(dead_point.is_some());
        let expected = EulerOpError::StaleGeometry {
            key: GeomRef::Point(t.points[0]),
        };
        // a0 starts at v0 (whose point is now gone); every earlier
        // precondition (same loop, cycle walk, prevs, face, shell)
        // passes, so the anchor resolution is what fires.
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mef_chord(
                MefSite::Chords {
                    he1: t.hes_a[0],
                    he2: t.hes_a[1],
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn fan_start_mismatch_is_rejected() {
        let mut t = pillow(Tol::witness());
        // a0 starts at v0, a1 at v1.
        let expected = EulerOpError::FanStartMismatch {
            he1: t.hes_a[0],
            he2: t.hes_a[1],
        };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mev_line(
                MevSite::Fan {
                    he1: t.hes_a[0],
                    he2: t.hes_a[1],
                },
                p(9.0),
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn broken_fan_orbit_is_rejected() {
        let mut t = pillow(Tol::witness());
        // Corrupt the edge ↔ half-edge bijection so mate(a0) fails: the
        // orbit walk from a0 breaks. a0 and b1 both start at v0.
        t.body.get_edge_mut(t.edges[0]).unwrap().he_plus = t.hes_a[1];
        let expected = EulerOpError::FanOrbitBroken {
            he1: t.hes_a[0],
            he2: t.hes_b[1],
        };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mev_line(
                MevSite::Fan {
                    he1: t.hes_a[0],
                    he2: t.hes_b[1],
                },
                p(9.0),
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn chords_in_different_loops_are_rejected() {
        let mut t = pillow(Tol::witness());
        let expected = EulerOpError::NotSameLoop {
            he1: t.hes_a[0],
            he2: t.hes_b[0],
        };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mef_chord(
                MefSite::Chords {
                    he1: t.hes_a[0],
                    he2: t.hes_b[0],
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn broken_loop_cycle_is_rejected() {
        let mut t = pillow(Tol::witness());
        // Tear a0's next into loop B: the cycle walk from a0 can never
        // reach a1 (nor return to a0).
        t.body.get_half_edge_mut(t.hes_a[0]).unwrap().next = t.hes_b[0];
        let expected = EulerOpError::LoopCycleBroken { r#loop: t.loop_a };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mef_chord(
                MefSite::Chords {
                    he1: t.hes_a[0],
                    he2: t.hes_a[1],
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn non_empty_loop_is_rejected_by_lone_sites() {
        let mut t = pillow(Tol::witness());
        let expected = EulerOpError::LoopNotEmpty { r#loop: t.loop_a };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mev_line(MevSite::Lone { r#loop: t.loop_a }, p(9.0), Tol::witness())
                .unwrap_err()
        });
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mef_chord(MefSite::Lone { r#loop: t.loop_a }, Tol::witness())
                .unwrap_err()
        });
    }

    #[test]
    fn chords_claiming_an_empty_parent_loop_are_rejected() {
        let mut t = pillow(Tol::witness());
        // Corrupt: a fresh empty loop, and a0/a1 claim it as parent.
        let p2 = t.body.add_point(p(9.0));
        let v2 = t.body.add_vertex(
            Vertex {
                point: p2,
                emanating: None,
            },
            prov(),
        );
        let empty = t.body.add_loop(
            Loop {
                boundary: LoopBoundary::Empty { vertex: v2 },
                face: t.face_a,
            },
            prov(),
        );
        t.body.get_half_edge_mut(t.hes_a[0]).unwrap().parent_loop = empty;
        t.body.get_half_edge_mut(t.hes_a[1]).unwrap().parent_loop = empty;
        let expected = EulerOpError::LoopNotCycle { r#loop: empty };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mef_chord(
                MefSite::Chords {
                    he1: t.hes_a[0],
                    he2: t.hes_a[1],
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    #[test]
    fn chords_with_dangling_second_start_vertex_are_rejected() {
        let mut t = pillow(Tol::witness());
        // Corrupt: a1's start vertex (v1) is removed; every earlier
        // precondition (resolution, same loop, cycle walk, prevs, face,
        // shell, anchor at v0) passes, so the start(he2) liveness check
        // is what fires.
        t.body.vertices.remove(t.vertices[1]);
        let expected = EulerOpError::StaleKey {
            key: EntityId::Vertex(t.vertices[1]),
        };
        assert_err_and_unchanged(&mut t.body, &expected, |body| {
            body.mef_chord(
                MefSite::Chords {
                    he1: t.hes_a[0],
                    he2: t.hes_a[1],
                },
                Tol::witness(),
            )
            .unwrap_err()
        });
    }

    /// One construction: digon pillow + strut + self-loop face. With
    /// `with_failures`, four failing calls (each a different
    /// precondition) are interleaved between the successful ones.
    fn build_with_optional_failures(
        body: &mut Body<f64>,
        with_failures: bool,
    ) -> (MvfsCreated, MevCreated, MefCreated, MevCreated, MefCreated) {
        let seed = body.mvfs(p(0.0)).unwrap();
        if with_failures {
            // Stale loop key.
            let err = body
                .mev_line(
                    MevSite::Lone {
                        r#loop: LoopKey::default(),
                    },
                    p(9.0),
                    Tol::witness(),
                )
                .unwrap_err();
            assert!(matches!(err, EulerOpError::StaleKey { .. }));
        }
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0),
                Tol::witness(),
            )
            .unwrap();
        if with_failures {
            // Fan halves starting at different vertices.
            let err = body
                .mev_line(
                    MevSite::Fan {
                        he1: seg.he_plus,
                        he2: seg.he_minus,
                    },
                    p(9.0),
                    Tol::witness(),
                )
                .unwrap_err();
            assert!(matches!(err, EulerOpError::FanStartMismatch { .. }));
        }
        let split = body
            .mef_chord(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        if with_failures {
            // Lone site on a loop that is a cycle now.
            let err = body
                .mef_chord(
                    MefSite::Lone {
                        r#loop: seed.r#loop,
                    },
                    Tol::witness(),
                )
                .unwrap_err();
            assert!(matches!(err, EulerOpError::LoopNotEmpty { .. }));
        }
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
        if with_failures {
            // Chords across the two digon loops.
            let err = body
                .mef_chord(
                    MefSite::Chords {
                        he1: seg.he_plus,
                        he2: split.he_plus,
                    },
                    Tol::witness(),
                )
                .unwrap_err();
            assert!(matches!(err, EulerOpError::NotSameLoop { .. }));
        }
        let circ = body
            .mef_chord(
                MefSite::Chords {
                    he1: strut.he_minus,
                    he2: strut.he_minus,
                },
                Tol::witness(),
            )
            .unwrap();
        assert_eq!(validate(body), Ok(()));
        (seed, seg, split, strut, circ)
    }

    #[test]
    fn failed_ops_leave_the_key_sequence_pure() {
        // The error half of D9's lineage-replay contract: a failing
        // operator consumes NO key slots, so a construction interleaved
        // with failing calls mints the exact key sequence of the same
        // construction without them — and the final bodies are deeply
        // identical (every arena entry, every payload, every provenance
        // record), not merely equal in counts.
        let mut with_errs = Body::<f64>::new();
        let mut without_errs = Body::<f64>::new();
        let created_a = build_with_optional_failures(&mut with_errs, true);
        let created_b = build_with_optional_failures(&mut without_errs, false);
        assert_eq!(created_a, created_b);
        assert_eq!(deep_snapshot(&with_errs), deep_snapshot(&without_errs));
    }

    /// Display smoke test, one sample per [`EulerOpError`] variant.
    ///
    /// What the compiler enforces: `variant_index` matches the enum with
    /// NO wildcard arm, so a new variant fails to build until an arm
    /// exists for it, and the coverage assertion then names the variant
    /// whose sample is missing.
    ///
    /// What it does NOT enforce: `VARIANTS` is hand-written, so a new
    /// variant given an arm but no sample still passes. Closing that
    /// needs the variant count from the compiler — `strum`'s `EnumCount`
    /// derive or the workspace's first proc-macro crate — and neither is
    /// bought here. When you add an arm, its index is the new
    /// `VARIANTS - 1`.
    #[test]
    fn every_error_displays() {
        const VARIANTS: usize = 27;
        fn variant_index(e: &EulerOpError) -> usize {
            match e {
                EulerOpError::Certification { .. } => 0,
                EulerOpError::DescriptionNotAdjacent { .. } => 1,
                EulerOpError::StaleKey { .. } => 2,
                EulerOpError::StaleGeometry { .. } => 3,
                EulerOpError::FanStartMismatch { .. } => 4,
                EulerOpError::FanOrbitBroken { .. } => 5,
                EulerOpError::NotSameLoop { .. } => 6,
                EulerOpError::LoopCycleBroken { .. } => 7,
                EulerOpError::LoopNotEmpty { .. } => 8,
                EulerOpError::LoopNotCycle { .. } => 9,
                EulerOpError::NotSameEdge { .. } => 10,
                EulerOpError::UnclaimedHalfEdge { .. } => 11,
                EulerOpError::SelfLoopEdge { .. } => 12,
                EulerOpError::OrbitBroken { .. } => 13,
                EulerOpError::EmptyAnchorsCollide { .. } => 14,
                EulerOpError::SameLoop { .. } => 15,
                EulerOpError::NotSameFace { .. } => 16,
                EulerOpError::RingIsOuter { .. } => 17,
                EulerOpError::SameFace { .. } => 18,
                EulerOpError::CrossShell { .. } => 19,
                EulerOpError::FaceHasRings { .. } => 20,
                EulerOpError::SolidNotSingleShell { .. } => 21,
                EulerOpError::ShellNotSingleFace { .. } => 22,
                EulerOpError::NullScaffoldCurve { .. } => 23,
                EulerOpError::SplitParamNotInterior { .. } => 24,
                EulerOpError::SplitParamEscalated { .. } => 25,
                EulerOpError::CrossSolid { .. } => 26,
            }
        }
        let he = HalfEdgeKey::default();
        let lp = LoopKey::default();
        let fc = FaceKey::default();
        let ek = EdgeKey::default();
        let vk = VertexKey::default();
        let errors = [
            EulerOpError::Certification {
                error: CertifyError::Unimplemented,
            },
            EulerOpError::DescriptionNotAdjacent { edge: ek },
            EulerOpError::StaleKey {
                key: EntityId::HalfEdge(he),
            },
            EulerOpError::StaleGeometry {
                key: GeomRef::Point(PointKey::default()),
            },
            EulerOpError::FanStartMismatch { he1: he, he2: he },
            EulerOpError::FanOrbitBroken { he1: he, he2: he },
            EulerOpError::NotSameLoop { he1: he, he2: he },
            EulerOpError::LoopCycleBroken { r#loop: lp },
            EulerOpError::LoopNotEmpty { r#loop: lp },
            EulerOpError::LoopNotCycle { r#loop: lp },
            EulerOpError::NotSameEdge { he1: he, he2: he },
            EulerOpError::UnclaimedHalfEdge { he, edge: ek },
            EulerOpError::SelfLoopEdge {
                edge: ek,
                vertex: vk,
            },
            EulerOpError::OrbitBroken { he },
            EulerOpError::EmptyAnchorsCollide { vertex: vk },
            EulerOpError::SameLoop { r#loop: lp },
            EulerOpError::NotSameFace {
                target: lp,
                ring: lp,
            },
            EulerOpError::RingIsOuter { r#loop: lp },
            EulerOpError::SameFace { face: fc },
            EulerOpError::CrossShell { f1: fc, f2: fc },
            EulerOpError::FaceHasRings { face: fc },
            EulerOpError::SolidNotSingleShell {
                solid: SolidKey::default(),
                shells: 2,
            },
            EulerOpError::ShellNotSingleFace {
                shell: ShellKey::default(),
                faces: 2,
            },
            EulerOpError::NullScaffoldCurve {
                curve: CurveKey::default(),
            },
            EulerOpError::SplitParamNotInterior { edge: ek },
            EulerOpError::SplitParamEscalated {
                edge: ek,
                diag: geom_core::Indeterminate {
                    margin: geom_core::MarginDiag::Value(5e-9),
                    band: Band::new(1e-9, 1e-8).unwrap(),
                    predicate: Some("split_edge_param_interior"),
                },
            },
            EulerOpError::CrossSolid { f1: fc, f2: fc },
        ];
        let mut covered = [false; VARIANTS];
        for error in &errors {
            assert!(!error.to_string().is_empty(), "{error:?}");
            covered[variant_index(error)] = true;
        }
        assert!(
            covered.iter().all(|&c| c),
            "every EulerOpError variant needs a Display sample; missing index {:?}",
            covered.iter().position(|&c| !c),
        );
    }

    /// S6 (two-tolerance, D4 ¶1 addendum): both `split_edge`
    /// interiority refusal arms describe one user situation — the
    /// definite arm composes the shared recourse directly, the
    /// escalated arm carries it through the `Indeterminate` Display.
    #[test]
    fn split_param_pair_carries_the_shared_recourse() {
        let edge = EdgeKey::default();
        let not_interior = EulerOpError::SplitParamNotInterior { edge };
        let msg = not_interior.to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );

        let escalated = EulerOpError::SplitParamEscalated {
            edge,
            diag: geom_core::Indeterminate {
                margin: geom_core::MarginDiag::Value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("split_edge_param_interior"),
            },
        };
        let msg = escalated.to_string();
        assert_eq!(
            msg.matches(geom_core::COINCIDENCE_RECOURSE).count(),
            1,
            "{msg}"
        );
    }
}

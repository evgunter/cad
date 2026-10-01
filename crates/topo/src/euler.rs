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
//!   than discarding it, at every write in these modules** — and at
//!   every write in `split_edge`, the attach setters, `movefac`,
//!   `revert`, `merge_coplanar_faces`, the boolean graft and the
//!   splitting carve. That enumeration is the claim; it is not "the
//!   whole crate".
//!
//!   **Announcing is the rule; which mechanism announces follows the
//!   key's provenance.** A key a mutation writes through is either
//!   minted in that phase or proven live by a check in the same call
//!   — here the plan phase, which returns
//!   [`EulerOpError::StaleKey`] otherwise — and never by the
//!   body's tier-1 validity, which is a whole-body property no single
//!   call establishes; those writes state
//!   that impossibility as `unreachable!` (the addendum's row 4), and
//!   the one write helper these modules share
//!   ([`Body::link_half_edges`]) states it as a precondition its
//!   callers discharge. A key that arrives instead from an arena
//!   BACK-POINTER has no such proof available to the call, so its
//!   plan step announces a typed refusal rather than asserting an
//!   impossibility it cannot establish (the addendum's row 1) —
//!   `merge_coplanar_faces`' ring re-homing, whose absorbed-face key
//!   is a loop's `face`, is the site of that shape inside this
//!   enumeration. The
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
//!   count deltas match the `ArenaDelta` it declares, and the whole
//!   body is re-derived against tier-1
//!   [`crate::validate::validate`] **once per public door** — at the
//!   end of the door, over the state the caller will see (Ev's ruling
//!   on `work/perf/d1-per-op-tier1-sweep-price`, PR 2305). The delta
//!   check is O(1) and is the op's own declared contract, so it runs
//!   at every call; the sweep is O(body), and a door running n
//!   operators pays it once rather than n times. Which of the two an
//!   op is depends on where it is called: **an operator a consumer
//!   calls directly is itself a door and sweeps at its end**, and one
//!   called inside a composing door's surgery scope
//!   ([`crate::surgery`]) does not, because that door has undertaken
//!   to. On tier-1-valid input
//!   a firing postcondition is a kernel bug by definition (the per-call
//!   instance of the ch. 9 soundness theorem failing against our
//!   transcription). Raw insertion is crate-internal since PR 5's
//!   builder demotion, so a body is reachable only through the public
//!   mutation paths, and the property those paths owe is that each
//!   **preserves tier 1, checked at every observable boundary**: the
//!   Euler operators with their chord/line sugar, and the non-operator
//!   structural mutators
//!   ([`Body::ring_move`], [`Body::split_edge`], [`Body::movefac`],
//!   [`Body::merge_coplanar_faces`]) declare the same debug
//!   postcondition, or open a surgery scope and close it with the
//!   sweep, or are composed of doors that do; the
//!   attach/metadata setters re-certify under the same tier-1
//!   assertion ([`Body::set_face_surface`], [`Body::set_edge_curve`])
//!   or write fields tier 1 does not constrain. **The closure property
//!   is the claim; a count of the doors is not** — an enumeration
//!   frozen into this sentence is what rots as doors are added, and
//!   `review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`
//!   checks the property against the real surface rather than against
//!   this list, both spellings included, and a scope opened and never
//!   closed fails there by name. `ring_move`'s case is the least
//!   obvious of the asserting doors: it re-glues the per-shell
//!   component partition, and the separating-curve argument lives in
//!   its docs.
//!
//!   **Localizing a door-level failure.** A door-level panic names the
//!   door, not the operator inside it that broke tier 1. Rebuild with
//!   `--features topo/per-op-postcondition` and the sweep runs after
//!   every operator again — surgery scopes ignored — so the message
//!   names the operator. Opt-in, never default-on.
//!
//!   **One class is the scalpel's alone.** A corruption an operator
//!   introduces and a later operator in the SAME door repairs never
//!   reaches the door's close, because the state the door hands back
//!   is sound. The door-level check is a claim about that state and
//!   not about every state the door passed through; the per-operator
//!   sweep is a claim about both, and it is the only thing that sees
//!   this one.
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
//!   an unrelated live entity. The same sentence is written in three
//!   places, so a correction has to reach all three. So a caller that
//!   ignores a graft's `Err` and keeps using `dst` can hand the next
//!   operator a tier-1-invalid body and fire its postcondition from
//!   **API misuse rather than a kernel bug**. That is the state class
//!   D9's footnote asserts cannot occur and the D2 addendum's five
//!   classes do not cover. **It is an open question in front of Ev,
//!   not a thing this module settles**: whether the graft can be
//!   restructured so a partially-written destination is not
//!   representable — staging into a fresh body and committing on
//!   success, the shape [`Body::merge_coplanar_faces`] already uses —
//!   or whether the class gets a name of its own.
//!
//!   The D9 taxonomy consequence therefore holds **for every door but
//!   that one**: these debug panics are
//!   **unreachable by input** through the public API as it stands —
//!   no public path builds a `Body` from bytes, so reaching one
//!   requires in-crate raw corruption (which is what the validator's
//!   own tests do deliberately) or a discarded graft refusal. Release
//!   builds carry no postcondition either way: on corruption the plan
//!   phase cannot detect they return `Ok` with a garbage body. That is
//!   wrong data written by lookups that all succeeded — the silent
//!   discards the D2 addendum superseded are gone from these three
//!   modules, the shared write helper
//!   ([`Body::link_half_edges`]) included.
//!
//!   **The plan phases' typed refusals of a torn arena do not rest on
//!   that reachability claim.** They rest on the D2 addendum's row-4
//!   rule: the body's tier-1 validity is a whole-body property no
//!   single call establishes, so it never stands in for a check. What
//!   a plan reads and cannot prove from its own reads is refused
//!   typed, however the body came to be torn.
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
//! key / mint a new [`Surface`] / share an existing key); the new
//! face's `sense` is derived on the parent's chart and stated by the
//! spec on any other ([`Body::resolve_face_surface`]).
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
//!   any spec that certifies, save `mef` minting a face on another
//!   key: the chord's two faces then wear the old face's key and the
//!   one it mints, both known in its plan phase, so it asks the same
//!   adjacency question of the chord.
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
//! let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true)?;
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
use geom_brep::recourse::Reading;
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec};
use geom_core::{Band, Decide, Point3, Real, Tol};
use slotmap::SecondaryMap;

use crate::attach::{Slot, require_description_adjacent};
use crate::body::Body;
use crate::entity::{
    Edge, EdgeKey, EntityId, Face, FaceKey, GeomRef, HalfEdge, HalfEdgeKey, Loop, LoopBoundary,
    LoopKey, Shell, ShellKey, Solid, SolidKey, Vertex, VertexKey,
};
use crate::geometry::{CurveKey, PointKey, SurfaceKey};
use crate::live::{Live, require_key};
use crate::pcurves::{SiteFace, SiteHalf, SiteLoop, SiteRows};
use crate::provenance::Provenance;
#[cfg(debug_assertions)]
use crate::test_support_impl::ArenaCounts;

/// How a face-minting operator obtains the new face's surface, and
/// the material side ([`crate::entity::Face::sense`]) the face carries
/// on it: the caller states the bit beside the chart, and `Inherit`
/// states none. Which bit a door writes is
/// [`Body::resolve_face_surface`]'s rule.
#[derive(Clone, Debug)]
pub enum FaceSurface<T: Real> {
    /// Share the *affected* face's surface key: `mef`'s split face (a
    /// face split is two regions of one surface — the M1 semantics,
    /// still right for coplanar/cosurface splits) or `mfkrh`'s
    /// demoting face.
    Inherit,
    /// Mint a new surface for the new face (the sweep's usual case —
    /// e.g. a Newell-certified plane from `geom_brep::newell_plane`).
    New {
        /// The surface to mint.
        surface: Surface<T>,
        /// The material side against `surface`'s chart normal.
        sense: bool,
    },
    /// Share an existing surface key (identical-by-construction
    /// surfaces keep one key — the ratified no-face-merging story's
    /// sharing half). Must resolve, checked as a precondition.
    Shared {
        /// The existing surface key.
        key: SurfaceKey,
        /// The material side against `key`'s chart normal.
        sense: bool,
    },
}

/// The keys-only door a [`EulerOpError::RechartStrandsDescriptions`] or
/// [`EulerOpError::RechartUnvouched`] refusal is raised by. Each puts
/// existing half-edges, or a chord it mints, on a face wearing another
/// key than the one they lay on, and the lever its refusal names is
/// its own: a minting door picks the chart it mints the face on, a
/// moving door the face it moves the loop onto.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RechartDoor {
    /// [`Body::set_face_surface`]: re-charts a face in place.
    SetFaceSurface,
    /// [`Body::mef`] and [`Body::mef_chord`]: mints a face from part of
    /// a loop (the run `[he1 .. he2)` and the chord's `he_minus`).
    Mef,
    /// [`Body::mfkrh`] and [`Body::mfkrh_minting`]: mints a face from a
    /// ring.
    Mfkrh,
    /// [`Body::mfkrh_plug`]: mints a face from a ring on a fresh
    /// placeholder chart. Its caller picks no chart, so its lever is
    /// the door: [`Body::mfkrh`], which takes one.
    MfkrhPlug,
    /// [`Body::ring_move`] and [`Body::ring_move_minting`]: moves a
    /// ring onto another face.
    RingMove,
}

impl RechartDoor {
    /// The door's name, as its refusals lead with it.
    pub fn name(self) -> &'static str {
        match self {
            Self::SetFaceSurface => "set_face_surface",
            Self::Mef => "mef",
            Self::Mfkrh => "mfkrh",
            Self::MfkrhPlug => "mfkrh_plug",
            Self::RingMove => "ring_move",
        }
    }

    /// [`EulerOpError::RechartStrandsDescriptions`]' text at this door,
    /// ending in the door's own lever (D4 ¶1 (i)).
    fn strands(self, edges: &[EdgeKey]) -> String {
        let name = self.name();
        let (what, lever) = match self {
            Self::SetFaceSurface => (
                "the swap",
                "leave the face on the chart those edges name, or re-describe them on the \
                 chart it moves onto (set_face_surfaces_describing takes their \
                 re-descriptions under a band, and carried_redescriptions states the stored \
                 ones there)",
            ),
            Self::Mef | Self::Mfkrh => (
                "the face it mints",
                "mint the face on the chart those edges name, then move it onto its own \
                 (set_face_surfaces_describing takes their re-descriptions under a band, and \
                 carried_redescriptions states the stored ones there)",
            ),
            Self::MfkrhPlug => (
                "the face it mints",
                "promote the ring with mfkrh onto the chart those edges name",
            ),
            Self::RingMove => (
                "the move",
                "move the loop onto a face on the chart its edges name",
            ),
        };
        format!(
            "{name}: {what} would leave edges {edges:?} described against a surface their \
             faces no longer wear, and the keys-only door takes no band to re-describe them. \
             Recourse: {lever}"
        )
    }

    /// [`EulerOpError::RechartUnvouched`]' text at this door, ending in
    /// the door's own lever (D4 ¶1 (i)).
    fn unvouched(self, face: FaceKey, edges: &[EdgeKey], chord: bool) -> String {
        let name = self.name();
        let named = match (edges.is_empty(), chord) {
            (_, false) => format!("its certified edges {edges:?} do"),
            (true, true) => "the certified chord it mints does".to_string(),
            (false, true) => format!("its certified edges {edges:?} and the chord it mints do"),
        };
        match self {
            Self::SetFaceSurface => format!(
                "{name}: face {face:?} would move onto a chart that {named} not name, so \
                 nothing vouches that its boundary lies on that chart. Recourse: move the face \
                 onto a chart its certified edges name, or re-describe them on the new chart \
                 (set_face_surfaces_describing certifies each re-description it is handed \
                 against that chart)"
            ),
            Self::Mef | Self::Mfkrh => format!(
                "{name}: the face it would mint from face {face:?} lies on a chart that \
                 {named} not name, so nothing vouches that its boundary lies on that \
                 chart. Recourse: mint the face on a chart its certified edges name, or mint \
                 it on its parent's chart and move it with set_face_surfaces_describing, which \
                 certifies each re-description it is handed against the new chart"
            ),
            Self::MfkrhPlug => format!(
                "{name}: the face it would mint from face {face:?} lies on a fresh placeholder \
                 chart that {named} not name, so nothing vouches that its boundary lies on \
                 that chart; this door promotes scaffold rings. Recourse: promote the ring \
                 with mfkrh onto a chart its certified edges name"
            ),
            Self::RingMove => format!(
                "{name}: the loop would move onto face {face:?}, on a chart that {named} not \
                 name, so nothing vouches that they lie on that chart. Recourse: move the \
                 loop onto a face on a chart its edges name"
            ),
        }
    }
}

/// Which way a face on its parent's chart faces relative to the
/// parent, as the operator's own topology decides it
/// ([`Body::resolve_face_surface`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ParentSide {
    /// The parent's material side: a `mef` fragment, or a face
    /// re-charted in place.
    With,
    /// The other side: the ring `mfkrh` promotes.
    Against,
}

/// A [`FaceSurface`] resolved in a door's plan phase
/// ([`Body::resolve_face_surface`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct ResolvedFace {
    /// Whether the spec lands on the parent's chart
    /// ([`Body::same_chart`]) — the answer the pcurve rows take too.
    pub(crate) on_parent_chart: bool,
    /// The bit the face carries.
    pub(crate) sense: bool,
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

/// The curve a make operator certifies for its new edge: the caller's
/// spec, or the scaffolding its chord sugar derives from the endpoints
/// the operator's own plan resolved, so the sugar
/// ([`Body::mev_line`], [`Body::mef_chord`], `Body::mekr_chord`) keeps
/// the operator's precondition order.
pub(crate) enum NewCurve<T: Real> {
    /// The caller's spec.
    Given(EdgeCurveSpec<T>),
    /// The chord line between the endpoints' points, or the canonical
    /// scaffolding circle when both endpoints are one vertex.
    Chord,
}

impl<T: Real> NewCurve<T> {
    /// The spec to certify from `p_from` to `p_to`; `one_vertex` is
    /// structural key equality of the endpoints, never a scalar
    /// comparison.
    pub(crate) fn spec(
        self,
        one_vertex: bool,
        p_from: Point3<T>,
        p_to: Point3<T>,
    ) -> EdgeCurveSpec<T> {
        match self {
            Self::Given(spec) => spec,
            Self::Chord if one_vertex => EdgeCurveSpec::self_loop_circle_at(p_from),
            Self::Chord => EdgeCurveSpec::line_between(p_from, p_to),
        }
    }
}

/// [`require_description_adjacent`] for the chord [`Body::mef`] mints,
/// whose faces wear `old`, the parent's key, and `after`, the new
/// face's: asked where `after` is not `old`. Where it is, nothing is
/// asked — the boolean pipeline's chord joins mint a chord with the
/// description of the face that will be glued along it
/// (`work/topo/minting-doors-take-a-callers-description-unasked-where-the-new-face-keeps-the-key`).
fn require_chord_adjacent<T: Real>(
    spec: &EdgeCurveSpec<T>,
    (old, after): (SurfaceKey, Slot),
) -> Result<(), EulerOpError> {
    if after == Slot::Kept(old) {
        return Ok(());
    }
    require_description_adjacent(None, &spec.description, [Slot::Kept(old), after])
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
// The companion fieldless enum is the compiler's own statement of this
// enum's variants and their order: the Display-coverage row indexes by
// it and sizes its array from its `COUNT`, so neither the count nor the
// order is written down twice. Test builds only — nothing in the
// production surface names it.
#[cfg_attr(test, derive(strum::EnumDiscriminants))]
#[cfg_attr(
    test,
    strum_discriminants(
        name(EulerOpErrorKind),
        vis(pub(crate)),
        derive(strum::EnumCount, strum::EnumIter)
    )
)]
pub enum EulerOpError {
    /// The curve-geometry spec failed its D4 ¶2 certification at the
    /// attachment gate (residual exceeded, sliver escalation,
    /// unresolved/unimplemented described surface, …) — the typed
    /// operation-time failure of D4 ¶3. The body is untouched.
    Certification {
        /// The certification failure.
        error: CertifyError,
    },
    /// A re-based edge would carry a carrier that does not describe it
    /// at the endpoints the move gives it: [`Body::mev`]'s fan site
    /// re-certifying the moved run's stored description, or
    /// [`Body::kev_describing`] re-certifying a merged member it was
    /// not handed a spec for, or certifying the spec it was handed.
    /// Raised in the plan phase, so the body is untouched.
    ///
    /// The description is authoritative and the carrier is its
    /// certified cache (D2/D4 ¶2), so an operator that cannot certify
    /// the moved edge has nothing honest to write: re-describing it is
    /// the caller's decision, made with a spec of its own
    /// ([`Body::set_edge_curve`], or the list [`Body::kev_describing`]
    /// takes).
    RebasedCarrier {
        /// The re-based edge whose carrier no longer describes it.
        edge: EdgeKey,
        /// The typed re-certification failure.
        error: CertifyError,
    },
    /// [`Body::mev`]'s fan site, or either kill door's fan merge
    /// ([`Body::kev`], [`Body::kev_describing`]), would move one end of
    /// a **null edge**
    /// ([`crate::CurveGeom::NullScaffold`]) onto another vertex and not
    /// the other: the moved run holds exactly one of its half-edges.
    /// The re-basing gate refuses it because it cannot ask whether the
    /// moved end lands on the other end's point (the reason, and the
    /// door that would ask, are stated once, in the crate-internal
    /// `Body::certify_rebased_run`'s docs). Raised in the plan phase,
    /// so the body is untouched. A run holding both halves moves both
    /// ends onto the one vertex and is not refused; a fan split that
    /// moves nothing is [`Body::mev_null`], and a merge that moves
    /// nothing is a kill of a null edge.
    ///
    /// Not [`EulerOpError::NullScaffoldCurve`], which is an operation
    /// that needs a carrier meeting an edge that has none: this arm is
    /// about the run's shape, one half of a null edge in it and the
    /// other not, and no operation here asks the null edge for a
    /// carrier.
    RebasedNullEdge {
        /// The null edge the run would re-base.
        edge: EdgeKey,
    },
    /// [`Body::kev`]'s fan merge would re-base these certified edges —
    /// the members of the dying vertex's surviving fan — onto the
    /// surviving vertex, and the keys-only kill cannot certify a
    /// carrier there: it takes no band, so it asks no question a band
    /// would answer, and it refuses every merge with a certified
    /// member, one whose two vertices hold one point included. Raised
    /// in the plan phase, so the body is untouched.
    ///
    /// The kill that can is [`Body::kev_describing`], which takes a
    /// band, and a re-description for any member whose stored carrier
    /// does not describe it at the merged endpoints. Every certified
    /// member is named, in the dying vertex's clockwise orbit order
    /// from the killed edge, because that door's caller needs the
    /// whole list and not its first entry.
    MergeRebasesCarriers {
        /// The certified merged members, in orbit order.
        edges: Vec<EdgeKey>,
    },
    /// [`Body::kev_describing`] was handed a re-description for an edge
    /// that is not a member of the merged fan: no half of it starts at
    /// the dying vertex once the killed edge is gone, so the merge does
    /// not re-base it (the killed edge itself included).
    NotMergedMember {
        /// The listed edge.
        edge: EdgeKey,
    },
    /// [`Body::kev_describing`] or [`Body::set_face_surfaces_describing`]
    /// was handed two re-descriptions for one edge. Named at the second
    /// entry.
    DuplicateRedescription {
        /// The edge listed twice.
        edge: EdgeKey,
    },
    /// A door writing a description ([`Body::set_edge_curve`], the
    /// describing doors' listed specs, and the chord [`Body::mef`]
    /// mints): an intrinsic (`Intersection`) or `Seam` description's
    /// surface keys do not match the edge's two adjacent faces'
    /// surfaces, or a chart image names neither — the description does
    /// not describe *this* edge's locus (D2: an intersection edge's
    /// surfaces are its adjacent faces'; a seam's surface is on both
    /// sides).
    DescriptionNotAdjacent {
        /// The edge whose description is incoherent with its faces;
        /// `None` for the edge a minting door would mint, which has no
        /// key before the mutation phase.
        edge: Option<EdgeKey>,
    },
    /// A keys-only re-chart door ([`RechartDoor`]) would leave these
    /// edges described against a surface their faces no longer wear:
    /// each description is adjacency-coherent now and would not be once
    /// the face, loop or run moves, which tier 3 reports at rest as
    /// `DescriptionNotAdjacent`. Every one is named — in edge-arena
    /// order at [`Body::set_face_surface`], in the order the door moves
    /// them at the Euler doors — because the caller re-describes the
    /// whole list. The lever is the door's own ([`RechartDoor`]): the
    /// chart a face is re-charted or minted onto, or the face a loop
    /// moves onto; [`Body::set_face_surfaces_describing`] takes a band
    /// and the re-descriptions. Raised in the plan phase, so the body is
    /// untouched.
    RechartStrandsDescriptions {
        /// The door that refuses.
        door: RechartDoor,
        /// The stranded edges.
        edges: Vec<EdgeKey>,
    },
    /// A keys-only re-chart door ([`RechartDoor`]) would put these
    /// certified edges — or the certified chord a minting door mints —
    /// on a face whose key their descriptions do not name, so nothing
    /// vouches that they, and the vertices they end at, lie on its
    /// chart: on a plane, tier 3's `PlanarBoundaryResidual` /
    /// `PlanarFaceResidual` at rest. Every one is named, in the order
    /// [`EulerOpError::RechartStrandsDescriptions`] names them. The
    /// lever is the door's own ([`RechartDoor`]); the describing door,
    /// [`Body::set_face_surfaces_describing`], certifies every
    /// re-description it is handed, and asks the boundary's own
    /// residuals only on a plane: onto a curved chart, handed no
    /// re-descriptions, it asks nothing
    /// (`work/restfront/validate-tier3-curved-boundary-containment`,
    /// #638). Raised in the plan phase, so the body is untouched.
    RechartUnvouched {
        /// The door that refuses.
        door: RechartDoor,
        /// The face the move is asked about: the re-charted face, the
        /// face a minting door mints from, or the face a ring moves
        /// onto.
        face: FaceKey,
        /// The certified edges that name no key their face wears after
        /// the move.
        edges: Vec<EdgeKey>,
        /// Whether the chord a minting door mints is among them: it has
        /// no key before the mutation phase.
        chord: bool,
    },
    /// [`Body::set_face_surfaces_describing`] was handed no
    /// re-description for these edges, and the move would strand them
    /// as [`EulerOpError::RechartStrandsDescriptions`] names: the door
    /// re-describes nothing by default. Every one is named, in
    /// edge-arena order. Raised in the plan phase, so the body is
    /// untouched.
    RechartUndescribed {
        /// The stranded edges no re-description was listed for, in
        /// edge-arena order.
        edges: Vec<EdgeKey>,
    },
    /// [`Body::set_face_surfaces_describing`]: a listed re-description
    /// does not certify against the charts the move gives its edge.
    /// Raised in the plan phase, so the body is untouched.
    RechartFalsifies {
        /// The edge that does not certify.
        edge: EdgeKey,
        /// The typed certification failure.
        error: CertifyError,
    },
    /// [`Body::set_face_surfaces_describing`]: a face moved onto a plane
    /// has a vertex, or an interior certification sample of an edge,
    /// definitely off that plane — tier 3's `PlanarFaceResidual` /
    /// `PlanarBoundaryResidual`, asked before the move. Raised in the
    /// plan phase, so the body is untouched.
    RechartOffBoundary {
        /// The moved face.
        face: FaceKey,
        /// The vertex, or the edge whose sample, lies off the plane.
        on: EntityId,
    },
    /// [`Body::set_face_surfaces_describing`]: a moved face's residual
    /// against its new plane escalated (in the sliver band, or
    /// poisoned) — the escalation counterpart of
    /// [`EulerOpError::RechartOffBoundary`].
    RechartBoundaryEscalated {
        /// The moved face.
        face: FaceKey,
        /// The vertex, or the edge whose sample, escalated.
        on: EntityId,
        /// The in-band/poisoned margin diagnostics.
        diag: geom_core::Indeterminate,
    },
    /// [`Body::set_face_surfaces_describing`] was handed one face
    /// twice. Named at the second entry.
    FaceMovedTwice {
        /// The face listed twice.
        face: FaceKey,
    },
    /// An argument key, or a key the operator must follow to do its
    /// work (a `prev` link, a spine parent, a start vertex), does not
    /// resolve. A caller reaches the first and only a torn body the
    /// second, and the variant does not say which.
    StaleKey {
        /// The unresolvable reference, wrapped with its kind.
        key: EntityId,
    },
    /// A geometry key the operator must read (an endpoint vertex's
    /// point for the certification gate, a `FaceSurface::Shared` key)
    /// does not resolve. A caller reaches it through a key it passed (the
    /// `Shared` key), and only a torn body through one a record holds.
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
    /// The loop's `next` cycle disagrees with the half-edges that claim
    /// it — tier-1-invalid input. A cycle walk failed to close, or
    /// closed without visiting the half-edge it had to reach (despite
    /// matching parent-loop keys); a run a walk takes and the plan
    /// moves has a member of another loop, or the walk of a loop the
    /// plan removes misses a member (the crate-internal
    /// `Body::require_run_of`); a kill's
    /// `next` step disagrees with the loop's members: the member it
    /// would anchor the loop at is killed or lies in another loop, or
    /// the loop it would empty keeps a member, or empties at a vertex
    /// the kill does not leave lone or another loop's lone vertex
    /// ([`Body::kef`], [`Body::kev`],
    /// [`Body::kemr`]); or an `Empty` loop that [`Body::kvfs`] or
    /// [`Body::mekr`]'s `Empty` ring sites remove, or that
    /// [`Body::movefac`]'s labelling reaches, is claimed by a half-edge.
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
    /// Two half-edges a kill takes as the halves of the edge it removes
    /// are not the two halves of one edge: [`Body::kemr`]'s arguments
    /// are equal or name two edges, or, a corrupt bijection —
    /// tier-1-invalid input — the edge does not claim exactly them in
    /// its two slots, or the mate [`Body::kef`] or [`Body::kev`] reads
    /// from the edge's slots is the argument itself or names another
    /// edge (the crate-internal `require_halves`). [`Body::movefac`]
    /// refuses it for a cycle member whose mate, read from the member's
    /// edge, names another edge. A caller reaches the first, through
    /// `kemr`'s arguments, and only a torn body the rest.
    NotSameEdge {
        /// The first half-edge.
        he1: HalfEdgeKey,
        /// The second half-edge.
        he2: HalfEdgeKey,
    },
    /// A half-edge's own edge does not claim it in either slot, so its
    /// mate cannot be resolved — a corrupt edge ↔ half-edge bijection,
    /// tier-1-invalid input. Fired by the single-half-edge kill
    /// operators ([`Body::kev`], [`Body::kef`]) for their argument, whose
    /// mate is computed rather than passed, by [`Body::movefac`] for a
    /// cycle member whose mate its labelling reads the same way, and by
    /// the two kills and [`Body::kemr`] for a half-edge outside the
    /// removed pair that names the edge the kill removes (the
    /// crate-internal `Body::require_edge_unnamed`).
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
    /// A plan read `he`'s start vertex's orbit and found it broken —
    /// tier-1-invalid input. Either the clockwise orbit walk from `he`
    /// failed to close or reached a half-edge that does not start at that
    /// vertex (fired by [`Body::kev`], [`Body::kev_describing`] and
    /// [`Body::kev_merged_members`], which walk the far vertex's whole fan
    /// from the mate, and by a fan [`Body::mev`] or [`Body::mev_null`] for
    /// a walk from `he1` that leaves the split vertex; the mev-specific
    /// form for a walk that fails to close or misses `he2` is
    /// [`EulerOpError::FanOrbitBroken`]); or a kill's new `emanating` for
    /// that vertex, read one `next` step from either killed half, starts
    /// elsewhere, or is `None` while another half-edge still starts there
    /// or while no loop the kill empties holds the vertex (fired by
    /// [`Body::kef`], [`Body::kemr`] and the same three `kev`
    /// calls, with `he` the killed half that starts at the vertex); or
    /// `he` starts at a vertex a kill removes and the kill neither
    /// removes `he` nor re-bases it off the vertex: [`Body::kvfs`]'s lone
    /// vertex, or the far vertex of the three `kev` calls, whose orbit
    /// walk from the mate did not reach `he` (the crate-internal
    /// `Body::require_vertex_unnamed`); or the orbit
    /// walk from `he` that [`Body::merge_coplanar_faces`] reads a strut
    /// tip from fails to close (carried in its `MergeCoplanarError::Op`).
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
    /// A kill would remove `to` while `from`, a record it keeps, still
    /// names it — tier-1-invalid input: a torn `face`, `rings`, `shell`,
    /// `faces`, `solid` or `Empty` boundary names the record from outside
    /// the ownership the kill reads it by, and the kill would leave `from`
    /// naming a dead record. Fired by [`Body::kef`] (a face listing its dying loop, a
    /// loop or shell naming its dying face), [`Body::kvfs`] (a face
    /// listing its loop, a loop or shell naming its face, a face or solid
    /// naming its shell, a shell naming its solid), [`Body::mekr`] (a
    /// face other than the ring's listing the ring), [`Body::kfmrh`] and
    /// [`Body::kfmrh_minting`] (a loop or shell naming `f2`, a face or
    /// solid naming the shell the fusion form removes), and
    /// [`Body::kef`], [`Body::kev`] and [`Body::kemr`] for a half-edge's
    /// `next` or `prev`, a loop's `first`, a vertex's `emanating` or
    /// another edge's slot naming a half-edge they remove (the
    /// crate-internal `Body::require_killed_halves_unnamed`), and
    /// [`Body::kvfs`] and [`Body::kev`] for a loop they keep that is
    /// `Empty` at the vertex they remove (the crate-internal
    /// `Body::require_vertex_unnamed`).
    ///
    /// A half-edge naming a removed loop, vertex or edge has the variant
    /// that already decides that field: a half-edge claiming a removed
    /// loop is [`EulerOpError::LoopCycleBroken`], one starting at a
    /// removed vertex [`EulerOpError::OrbitBroken`], one naming a removed
    /// edge [`EulerOpError::UnclaimedHalfEdge`].
    KillLeavesDangling {
        /// The record the kill keeps, which names `to`.
        from: EntityId,
        /// The record the kill removes.
        to: EntityId,
    },
    /// [`Body::movefac`]'s labelling took `child` as `owner`'s, and the
    /// ownership does not hold in both directions — tier-1-invalid
    /// input: `owner` does not list `child`, or `child` does not name
    /// `owner`, or neither. Raised for a face the labelling labels,
    /// whether a seed from the shell's list or a neighbour reached
    /// across an edge, that the shell does not list or whose `shell` is
    /// another (`owner` is the shell), for a loop a face lists whose
    /// `face` is another, and for the loop a mate lies in whose `face`
    /// does not list it (`owner` is the face).
    NotOwned {
        /// The record the labelling took as `owner`'s.
        child: EntityId,
        /// The record that does not own it in both directions.
        owner: EntityId,
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
    /// [`Body::set_null_face_pair`]'s record names a loop that is not
    /// the marked face's own — neither its outer loop nor one of its
    /// rings. A null face is one face's two coincident loops
    /// ([`crate::null`]), so such a record describes none.
    NullPairForeignLoop {
        /// The face the record would mark.
        face: FaceKey,
        /// The first role loop, in declaration order, that `face` does
        /// not hold.
        r#loop: LoopKey,
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
    /// with [`Body::kev`] when its endpoints are distinct — or
    /// [`Body::kev_describing`], where the far vertex's merged fan
    /// carries certified edges; the
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
    /// type. Fired by [`Body::split_edge`] (nothing to split), by
    /// the sweep upgrade paths (nothing to upgrade), and by
    /// [`Body::set_face_surfaces_describing`] on a listed null edge (a
    /// null edge's first description mints its rows, which is
    /// [`Body::set_edge_curve`]'s).
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
        /// The sub-span's verdict: within the zero band of an end, with
        /// the margin it classified, or definitely outside the edge.
        verdict: geom_brep::recourse::Refused,
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
    /// [`Body::split_edge`] could not carry a parent half-edge's stored
    /// **pcurve row** across the split: the parent's chart image,
    /// restricted to a child's sub-interval, failed the certification
    /// the whole image passed. Raised before any mutation, so the body
    /// is untouched — a covered chart lane that refuses here is a
    /// defect, never a licence to leave the face half-minted
    /// (`crate::pcurves::split_cache`).
    PcurveSplit {
        /// The edge being split.
        edge: EdgeKey,
        /// The parent half-edge whose row was being restricted.
        half_edge: HalfEdgeKey,
        /// The typed certification failure, nested whole.
        error: geom_brep::PcurveCertifyError,
    },
    /// [`Body::mev`], [`Body::mef`] or [`Body::mekr`] would add a
    /// half-edge to a face the site mint re-mints — one whose **pcurve
    /// rows are complete**, or complete but for the loops a null edge
    /// holds open — and a row it needs cannot be minted under the
    /// operators' `Decide` bound ([`crate::pcurves::SiteRowRefusal`]: a
    /// complete face on a spline chart, the fitted frontier, or a
    /// half-edge of a loop the op re-mints does not resolve). Raised
    /// before any mutation, so the body is untouched — these three
    /// operators leave no complete face half-minted. Also raised by [`Body::set_edge_curve`] on a
    /// null edge's first description, which re-mints the faces the
    /// edge's halves are on through the same site mint, where a
    /// half-edge of such a face does not resolve.
    PcurveMint {
        /// The face whose rows were being re-minted: the face the new
        /// half-edge would join (for `mef`'s new face, the face it is
        /// carved from), or a face a described null edge's half is on.
        face: FaceKey,
        /// Why the row cannot be minted.
        refusal: crate::pcurves::SiteRowRefusal,
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
    /// [`Body::move_shells_to_new_solid`]'s list is empty: a solid
    /// with no shells is not a solid (tier 1's arity floor), so there
    /// is nothing to mint.
    NoShellsNamed,
    /// [`Body::move_shells_to_new_solid`]'s list names one shell
    /// twice — a caller desync, refused rather than resolved by list
    /// order.
    ShellRepeated {
        /// The shell named more than once.
        shell: ShellKey,
    },
    /// [`Body::move_shells_to_new_solid`]'s shells do not all belong
    /// to one solid: the op re-partitions ONE solid's shells, and a
    /// list spanning two has no single source solid to split from.
    ShellsAcrossSolids {
        /// The first shell, in the solid the op would split.
        shell: ShellKey,
        /// A later shell, in a different solid.
        other: ShellKey,
    },
    /// [`Body::move_shells_to_new_solid`] would move EVERY shell of
    /// its source solid, leaving it with none — tier 1's arity floor
    /// again, on the solid that stays behind.
    SolidWouldEmpty {
        /// The solid that would be left without shells.
        solid: SolidKey,
    },
    /// A [`FaceSurface`] spec on `face`'s own chart states a
    /// [`crate::entity::Face::sense`] other than the one
    /// [`Body::resolve_face_surface`] derives there. Raised in the plan
    /// phase of [`Body::mef`], [`Body::mfkrh`] and
    /// [`Body::set_face_surface`], so the body is untouched.
    SenseContradictsChart {
        /// The face whose chart the spec lands on: the parent of a
        /// minted face, or the face re-charted in place.
        face: FaceKey,
        /// The bit the spec stated.
        stated: bool,
        /// The bit the operator derives on that chart.
        derived: bool,
    },
}

impl EulerOpError {
    /// This refusal's text, a certification refusal's ending read at
    /// `reading` ([`CertifyError::ending`]): the door that reports the
    /// refusal decides where it is read. `Display` reads it at
    /// [`Reading::Build`], the operation that built the edge.
    #[must_use]
    pub fn render(&self, reading: Reading) -> String {
        match self {
            Self::Certification { error } => {
                format!("geometry attachment gate: {}", error.render(reading))
            }
            Self::RebasedCarrier { edge, error } => format!(
                "re-based edge {edge:?} would keep a carrier its endpoint left: {}",
                error.render(reading)
            ),
            Self::RebasedNullEdge { edge } => format!(
                "the moved run re-bases one end of null edge {edge:?} and not the other, \
                 and the re-basing gate cannot ask whether the moved end lands on the \
                 other's point (the split that moves nothing is mev_null; the merge that \
                 moves nothing is a kill of the null edge itself)"
            ),
            Self::MergeRebasesCarriers { edges } => format!(
                "kev: the fan merge re-bases certified edges {edges:?} onto the surviving \
                 vertex, and the keys-only kill takes no band to certify them there. \
                 Recourse: re-describe those edges at the surviving vertex, or kill the edge's \
                 other end where it meets no other edge (kev_describing takes their \
                 re-descriptions under a band, and kev on the other half kills the other end)"
            ),
            Self::NotMergedMember { edge } => format!(
                "kev_describing: edge {edge:?} is not a member of the merged fan, so the \
                 merge gives it nothing to re-describe"
            ),
            Self::DuplicateRedescription { edge } => {
                format!("edge {edge:?} is re-described twice")
            }
            Self::DescriptionNotAdjacent { edge } => {
                let edge = edge.map_or_else(
                    || "the edge the door mints".to_string(),
                    |edge| format!("edge {edge:?}"),
                );
                format!(
                    "{edge}: its description names surfaces that are not its two faces' \
                     surfaces (D2 adjacency coherence). Recourse: describe it against the \
                     surfaces its faces wear"
                )
            }
            Self::RechartStrandsDescriptions { door, edges } => door.strands(edges),
            Self::RechartUnvouched {
                door,
                face,
                edges,
                chord,
            } => door.unvouched(*face, edges, *chord),
            Self::RechartUndescribed { edges } => format!(
                "set_face_surfaces_describing: the move would leave edges {edges:?} described \
                 against a surface their faces no longer wear, and no re-description is listed \
                 for them (carried_redescriptions states their stored descriptions on the \
                 moved charts)"
            ),
            Self::RechartFalsifies { edge, error } => format!(
                "set_face_surfaces_describing: edge {edge:?}'s re-description does not certify \
                 on the charts the move gives it: {}",
                error.render(reading)
            ),
            Self::RechartOffBoundary { face, on } => format!(
                "set_face_surfaces_describing: face {face:?}'s boundary does not lie on the \
                 plane it moves onto ({on} is off it)"
            ),
            Self::RechartBoundaryEscalated { face, on, diag } => format!(
                "set_face_surfaces_describing: whether face {face:?}'s boundary lies on the \
                 plane it moves onto is undecided at {on}: {}",
                diag.payload()
            ),
            Self::FaceMovedTwice { face } => {
                format!("set_face_surfaces_describing: face {face:?} is moved twice")
            }
            Self::StaleKey { key } => {
                format!("euler op requires {key}, which does not resolve")
            }
            Self::StaleGeometry { key } => {
                format!("euler op requires {key}, which does not resolve")
            }
            Self::FanStartMismatch { he1, he2 } => format!(
                "mev fan: half-edges {he1:?} and {he2:?} start at different \
                 vertices"
            ),
            Self::FanOrbitBroken { he1, he2 } => format!(
                "mev fan: the clockwise vertex orbit from {he1:?} never \
                 reaches {he2:?}. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::NotSameLoop { he1, he2 } => format!(
                "half-edges {he1:?} and {he2:?} belong to different loops \
                 (one loop required)"
            ),
            Self::LoopCycleBroken { r#loop } => format!(
                "loop {loop:?}'s next cycle disagrees with the half-edges that \
                 claim it: a walk of it fails to close, strays into another \
                 loop or misses one of its members, or it is empty at a vertex \
                 another loop also holds or a half-edge starts at. {}",
                geom_core::KERNEL_DEFECT_ENDING,
                loop = r#loop
            ),
            Self::LoopNotEmpty { r#loop } => format!(
                "empty-loop site: loop {loop:?} is not an empty loop",
                loop = r#loop
            ),
            Self::LoopNotCycle { r#loop } => format!(
                "a half-edge argument claims parent loop {loop:?}, which is \
                 an empty loop. {}",
                geom_core::KERNEL_DEFECT_ENDING,
                loop = r#loop
            ),
            Self::NotSameEdge { he1, he2 } => format!(
                "half-edges {he1:?} and {he2:?} are not the two halves of one \
                 edge"
            ),
            Self::UnclaimedHalfEdge { he, edge } => format!(
                "half-edge {he:?}'s edge {edge:?} does not claim it in either \
                 slot, so its mate cannot be resolved. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::SelfLoopEdge { edge, vertex } => format!(
                "kev: edge {edge:?} is a self-loop at vertex {vertex:?} — kev \
                 needs distinct end vertices (kill a self-loop edge with kef \
                 or kemr)"
            ),
            Self::OrbitBroken { he } => format!(
                "the vertex half-edge {he:?} starts at has a broken orbit or \
                 anchor: a walk of its orbit fails to close or leaves the \
                 vertex, an anchor a kill writes for it starts elsewhere, a kill \
                 takes it for lone while a half-edge still starts at it or no \
                 empty loop holds it, or a kill removes it while {he:?}, which \
                 the kill keeps, still starts at it. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::EmptyAnchorsCollide { vertex } => format!(
                "the operation would leave two empty loops holding the same \
                 lone vertex {vertex:?}, which only a torn body reaches. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::KillLeavesDangling { from, to } => format!(
                "the kill removes {to}, which {from} still names outside the ownership \
                 the kill reads, so it would be left naming a dead record. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::NotOwned { child, owner } => format!(
                "movefac took {child} as {owner}'s, but the two do not own each \
                 other both ways: {owner} does not list {child}, or {child} does \
                 not name {owner}. {}",
                geom_core::KERNEL_DEFECT_ENDING
            ),
            Self::SameLoop { r#loop } => format!(
                "two distinct loops required, but both sides name loop \
                 {loop:?} (mekr joins two loops of a face; kef on an edge \
                 occurring twice in one loop is kemr's job)",
                loop = r#loop
            ),
            Self::NotSameFace { target, ring } => format!(
                "mekr: loops {target:?} and {ring:?} belong to different \
                 faces"
            ),
            Self::NullPairForeignLoop { face, r#loop } => format!(
                "a null-face record on face {face:?} names loop {loop:?}, which is not \
                 that face's outer loop or one of its rings (a null face is one face's \
                 two coincident loops)",
                loop = r#loop
            ),
            Self::RingIsOuter { r#loop } => format!(
                "loop {loop:?} is its face's outer loop, not a ring",
                loop = r#loop
            ),
            Self::SameFace { face } => format!(
                "two distinct faces required, but both sides name face \
                 {face:?} (kfmrh sums two faces; kef on an edge interior to \
                 one face has no face to kill — see kev and kev_describing)"
            ),
            Self::CrossShell { f1, f2 } => format!(
                "faces {f1:?} and {f2:?} lie in different shells \
                 (ring_move reparents a ring within one shell only; \
                 cross-shell face merging is kfmrh's shell-fusion form)"
            ),
            Self::FaceHasRings { face } => format!(
                "face {face:?} still has rings and must be ring-free here \
                 (move them off with ring_move first)"
            ),
            Self::SolidNotSingleShell { solid, shells } => format!(
                "kvfs: solid {solid:?} has {shells} shells, not the skeletal \
                 single shell"
            ),
            Self::ShellNotSingleFace { shell, faces } => format!(
                "kvfs: shell {shell:?} has {faces} faces, not the skeletal \
                 single face"
            ),
            Self::NullScaffoldCurve { curve } => format!(
                "curve {curve:?} is null-edge scaffolding (no carrier by \
                 type); the operation requires a certified carrier"
            ),
            // The split's two interiority arms are one decision, so both
            // end in its one ending; every door that splits an edge at a
            // crossing (the split, the blend, the Boolean) forwards them
            // whole.
            Self::SplitParamNotInterior { verdict, .. } => format!(
                "{}. {}",
                match verdict {
                    geom_brep::recourse::Refused::Zero(_) => {
                        "a crossing lands on an end of its edge at this tolerance, not \
                         strictly inside it"
                    }
                    geom_brep::recourse::Refused::Negative { .. } => {
                        "a crossing lands outside its edge"
                    }
                },
                crate::split::split_param_ending(verdict.arm())
            ),
            Self::SplitParamEscalated { diag, .. } => format!(
                "{} is undecided: {}. {}",
                crate::split::CROSSING_INTERIOR,
                diag.payload(),
                crate::split::split_param_ending(geom_brep::recourse::RefusedArm::Undecided(diag))
            ),
            Self::PcurveSplit {
                edge,
                half_edge,
                error,
            } => format!(
                "split_edge: on edge {edge:?}, half-edge {half_edge:?}'s stored pcurve \
                 row does not re-certify over a child's sub-interval: {error}"
            ),
            Self::PcurveMint { face, refusal } => format!(
                "the operator would add a half-edge to face {face:?}, whose pcurve rows are \
                 complete, and cannot mint its row: {refusal}"
            ),
            Self::CrossSolid { f1, f2 } => format!(
                "kfmrh: faces {f1:?} and {f2:?} lie in different solids \
                 (cross-solid fusion is the boolean combine step, not an \
                 Euler surgery)"
            ),
            Self::NoShellsNamed => "move_shells_to_new_solid: no shells named, and a solid with \
                                    no shells is not a solid"
                .to_owned(),
            Self::ShellRepeated { shell } => format!(
                "move_shells_to_new_solid: shell {shell:?} is named more than once \
                 (caller desync)"
            ),
            Self::ShellsAcrossSolids { shell, other } => format!(
                "move_shells_to_new_solid: shells {shell:?} and {other:?} lie in \
                 different solids (the op re-partitions one solid's shells)"
            ),
            Self::SolidWouldEmpty { solid } => format!(
                "move_shells_to_new_solid: moving every shell of solid {solid:?} \
                 would leave it with none"
            ),
            Self::SenseContradictsChart {
                face,
                stated,
                derived,
            } => format!(
                "the face-surface spec lands on face {face:?}'s chart and states sense \
                 {stated}, where the operator derives {derived} on that chart"
            ),
        }
    }
}

impl fmt::Display for EulerOpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.render(Reading::Build))
    }
}

impl std::error::Error for EulerOpError {}

/// One sample of every [`EulerOpError`] variant, in declaration
/// order — the crate's single such array.
///
/// The index and the count are the compiler's: `EulerOpErrorKind` is
/// derived from the enum, so a variant added without a sample fails
/// by name here and nothing restates the enum. The two derives'
/// agreement on order — `from(err) as usize` is the declaration index
/// and `iter()` walks the same sequence — is asserted here rather
/// than by each caller, so a row that consumes this array inherits
/// the guarantee instead of quietly relying on another row for it.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(crate) fn every_euler_op_error_once()
-> [EulerOpError; <EulerOpErrorKind as strum::EnumCount>::COUNT] {
    use strum::{EnumCount as _, IntoEnumIterator as _};
    let he = HalfEdgeKey::default();
    let lp = LoopKey::default();
    let fc = FaceKey::default();
    let ek = EdgeKey::default();
    let vk = VertexKey::default();
    let errors = [
        EulerOpError::Certification {
            error: CertifyError::Unimplemented,
        },
        EulerOpError::RebasedCarrier {
            edge: ek,
            error: CertifyError::Unimplemented,
        },
        EulerOpError::RebasedNullEdge { edge: ek },
        EulerOpError::MergeRebasesCarriers { edges: vec![ek] },
        EulerOpError::NotMergedMember { edge: ek },
        EulerOpError::DuplicateRedescription { edge: ek },
        EulerOpError::DescriptionNotAdjacent { edge: Some(ek) },
        EulerOpError::RechartStrandsDescriptions {
            door: RechartDoor::Mef,
            edges: vec![ek],
        },
        EulerOpError::RechartFalsifies {
            edge: ek,
            error: CertifyError::Unimplemented,
        },
        EulerOpError::RechartUnvouched {
            door: RechartDoor::RingMove,
            face: fc,
            edges: vec![ek],
            chord: false,
        },
        EulerOpError::RechartUndescribed { edges: vec![ek] },
        EulerOpError::RechartOffBoundary {
            face: fc,
            on: EntityId::Edge(ek),
        },
        EulerOpError::RechartBoundaryEscalated {
            face: fc,
            on: EntityId::Edge(ek),
            diag: geom_core::Indeterminate {
                margin: geom_core::MarginDiag::value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("rechart_boundary_residual"),
                terminal_sliver: false,
            },
        },
        EulerOpError::FaceMovedTwice { face: fc },
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
        EulerOpError::KillLeavesDangling {
            from: EntityId::Face(fc),
            to: EntityId::Loop(lp),
        },
        EulerOpError::NotOwned {
            child: EntityId::Loop(lp),
            owner: EntityId::Face(fc),
        },
        EulerOpError::SameLoop { r#loop: lp },
        EulerOpError::NotSameFace {
            target: lp,
            ring: lp,
        },
        EulerOpError::NullPairForeignLoop {
            face: fc,
            r#loop: lp,
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
        EulerOpError::SplitParamNotInterior {
            edge: ek,
            verdict: geom_brep::recourse::Refused::Negative {
                margin: geom_core::MarginDiag::value(-0.25),
            },
        },
        EulerOpError::SplitParamEscalated {
            edge: ek,
            diag: geom_core::Indeterminate {
                margin: geom_core::MarginDiag::value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("split_edge_param_interior"),
                terminal_sliver: false,
            },
        },
        EulerOpError::PcurveSplit {
            edge: ek,
            half_edge: he,
            error: geom_brep::PcurveCertifyError::UnsupportedCarrier,
        },
        EulerOpError::PcurveMint {
            face: fc,
            refusal: crate::pcurves::SiteRowRefusal::SplineChart,
        },
        EulerOpError::CrossSolid { f1: fc, f2: fc },
        EulerOpError::NoShellsNamed,
        EulerOpError::ShellRepeated {
            shell: ShellKey::default(),
        },
        EulerOpError::ShellsAcrossSolids {
            shell: ShellKey::default(),
            other: ShellKey::default(),
        },
        EulerOpError::SolidWouldEmpty {
            solid: SolidKey::default(),
        },
        EulerOpError::SenseContradictsChart {
            face: fc,
            stated: true,
            derived: false,
        },
    ];
    for (i, kind) in EulerOpErrorKind::iter().enumerate() {
        assert_eq!(kind as usize, i, "EnumIter order is the discriminant order");
    }
    let mut covered = [false; EulerOpErrorKind::COUNT];
    for error in &errors {
        covered[EulerOpErrorKind::from(error) as usize] = true;
    }
    let missing: Vec<EulerOpErrorKind> = EulerOpErrorKind::iter()
        .zip(covered)
        .filter_map(|(kind, seen)| (!seen).then_some(kind))
        .collect();
    assert!(
        missing.is_empty(),
        "every EulerOpError variant needs a sample; missing {missing:?}",
    );
    errors
}

impl EulerOpError {
    /// Whether this refusal reports a **torn arena** — a body that is
    /// already tier-1-invalid — rather than a fact about the
    /// operation that was asked for.
    ///
    /// The membership is this enum's own documentation: a variant
    /// answers `true` exactly when its doc comment says the state is
    /// tier-1-invalid input, plus [`EulerOpError::StaleKey`] and
    /// [`EulerOpError::StaleGeometry`], whose whole subject is a key
    /// that did not resolve. Callers
    /// that place a refusal — a driver deciding whether to record it
    /// and carry on, or to refuse — ask here instead of keeping a
    /// second copy of the list.
    ///
    /// **For three variants `true` assumes the call's keys are
    /// right.** A caller also reaches [`EulerOpError::StaleKey`] and
    /// [`EulerOpError::StaleGeometry`] by passing a key the body does
    /// not hold, and [`EulerOpError::NotSameEdge`] by passing `kemr` two
    /// half-edges that are not mates, and the variant does not say
    /// which reached it. The answer holds for them only where every
    /// key the call passed resolves in this body and `kemr`'s two are
    /// mates.
    ///
    /// The match is exhaustive on purpose: a new variant does not
    /// compile until someone says which side of this line it is on.
    #[must_use]
    pub fn reports_tier1_corruption(&self) -> bool {
        match self {
            // A key that did not resolve, whichever arena it names.
            Self::StaleKey { .. } | Self::StaleGeometry { .. } => true,
            // The walks that cannot fail on a tier-1-valid body.
            Self::FanOrbitBroken { .. }
            | Self::LoopCycleBroken { .. }
            | Self::OrbitBroken { .. } => true,
            // A half-edge whose parent loop is empty, and the two
            // corrupt edge <-> half-edge bijections.
            Self::LoopNotCycle { .. }
            | Self::NotSameEdge { .. }
            | Self::UnclaimedHalfEdge { .. } => true,
            // "Believed unreachable through valid operator sequences
            // (the offending inputs are already tier-1-invalid)".
            Self::EmptyAnchorsCollide { .. } => true,
            // A record a kill keeps naming one it removes, and a record
            // `movefac` takes as another's that does not own it both ways.
            Self::KillLeavesDangling { .. } | Self::NotOwned { .. } => true,
            // A row the operator could not mint: a fact about the
            // operation, except where the derivation met a key that
            // did not resolve.
            Self::PcurveMint { refusal, .. } => {
                matches!(refusal, crate::pcurves::SiteRowRefusal::Corrupt)
            }
            // Facts about the operation that was asked for: a
            // certification verdict, a site or argument that does not
            // meet the operator's precondition, a shape the operator
            // does not cover. Every one of these is legal to meet on
            // a tier-1-valid body.
            Self::Certification { .. }
            | Self::RebasedCarrier { .. }
            | Self::RebasedNullEdge { .. }
            | Self::MergeRebasesCarriers { .. }
            | Self::NotMergedMember { .. }
            | Self::DuplicateRedescription { .. }
            | Self::DescriptionNotAdjacent { .. }
            | Self::RechartStrandsDescriptions { .. }
            | Self::RechartUnvouched { .. }
            | Self::RechartUndescribed { .. }
            | Self::RechartFalsifies { .. }
            | Self::RechartOffBoundary { .. }
            | Self::RechartBoundaryEscalated { .. }
            | Self::FaceMovedTwice { .. }
            | Self::FanStartMismatch { .. }
            | Self::NotSameLoop { .. }
            | Self::LoopNotEmpty { .. }
            | Self::SelfLoopEdge { .. }
            | Self::SameLoop { .. }
            | Self::NotSameFace { .. }
            | Self::NullPairForeignLoop { .. }
            | Self::RingIsOuter { .. }
            | Self::SameFace { .. }
            | Self::CrossShell { .. }
            | Self::FaceHasRings { .. }
            | Self::SolidNotSingleShell { .. }
            | Self::ShellNotSingleFace { .. }
            | Self::NullScaffoldCurve { .. }
            | Self::SplitParamNotInterior { .. }
            | Self::SplitParamEscalated { .. }
            | Self::PcurveSplit { .. }
            | Self::CrossSolid { .. }
            | Self::NoShellsNamed
            | Self::ShellRepeated { .. }
            | Self::ShellsAcrossSolids { .. }
            | Self::SolidWouldEmpty { .. }
            | Self::SenseContradictsChart { .. } => false,
        }
    }
}

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

    /// [`Body::mev`]'s shift, and its chord sugar's.
    pub(crate) const MEV: Self = Self {
        half_edges: 2,
        edges: 1,
        vertices: 1,
        ..Self::ZERO
    };

    /// [`Body::mef`]'s shift, and its chord sugar's.
    pub(crate) const MEF: Self = Self {
        faces: 1,
        loops: 1,
        half_edges: 2,
        edges: 1,
        ..Self::ZERO
    };

    /// [`Body::mekr`]'s shift, and its chord sugar's.
    pub(crate) const MEKR: Self = Self {
        loops: -1,
        half_edges: 2,
        edges: 1,
        ..Self::ZERO
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

/// Does this re-certification failure name an ENDPOINT residual — the
/// one failure class re-basing a half-edge run can introduce?
///
/// Every other variant is a statement about the description or the
/// surfaces it names, which a move does not touch, so
/// [`Body::certify_rebased_run`]'s pre-existing-staleness arm is scoped
/// to these two checks and nothing else.
fn is_endpoint_residual(error: CertifyError) -> bool {
    matches!(
        error,
        CertifyError::ResidualExceeded {
            check: geom_brep::CertCheck::EndpointStart | geom_brep::CertCheck::EndpointEnd,
            ..
        }
    )
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
    /// `sense` is the seed face's [`crate::Face::sense`], stated by the
    /// caller and provisional until it charts the face
    /// ([`Body::resolve_face_surface`]).
    ///
    /// # Errors
    ///
    /// None today — `mvfs` has no preconditions (it consumes nothing).
    /// The `Result` keeps the operator signatures uniform.
    pub fn mvfs(&mut self, point: Point3<T>, sense: bool) -> Result<MvfsCreated, EulerOpError> {
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
                sense,
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
    /// **Pcurve rows** ([`crate::pcurves`]): a face the new halves join
    /// whose rows are COMPLETE is not left half-minted. The loops the
    /// halves join are re-minted with them, before any mutation — the
    /// rows the minting pass would store — and its other loops keep
    /// theirs; or, where the closed-form lane cannot mint the face as
    /// the surgery leaves it, it stores nothing; on a spline chart the
    /// op refuses [`EulerOpError::PcurveMint`]. A face whose only gaps
    /// are on loops a null edge holds open is taken the same way, and a
    /// loop the null edge still holds keeps what it had, its new halves
    /// rowless; on a spline chart that face is left as found. A face
    /// storing no row stays rowless, and one half-minted any other way
    /// is left as found (`crate::pcurves::site_rows` carries the rule). The cost is one
    /// walk and one certification per half-edge of the rewired loops,
    /// one presence read per half-edge of the face's other loops, and on
    /// a face missing a row three lookups per half-edge for which loops
    /// a null edge holds open.
    ///
    /// **The moved run's carriers are re-certified, never
    /// re-described.** At a fan site the run `[he1 .. he2)` is
    /// re-based onto the new vertex `w`, and each of those edges keeps
    /// the curve it was certified with. So the gate asks whether that
    /// curve still describes the edge `w` gives it, against the
    /// endpoints of the re-based edge, and **refuses**
    /// [`EulerOpError::RebasedCarrier`] naming the edge where THIS
    /// MOVE is what makes the answer no — body untouched, like every
    /// other precondition. A carrier that already missed its own
    /// endpoint before the call is carried rather than refused: it
    /// names a defect this operation did not create. Re-describing a
    /// run is [`Body::set_edge_curve`]'s decision, with the caller's own
    /// spec; the gate's own docs carry the argument for why an
    /// operator re-certifies exactly rather than re-fitting. A **null
    /// edge** the run moves one end of (one of its halves in the run,
    /// the other not) is refused [`EulerOpError::RebasedNullEdge`]; one
    /// whose two halves are both in the run moves whole and is carried.
    /// The one-half refusal, and the plane × NURBS class's
    /// `RebasedCarrier { NurbsLaneNotSupplied }`, stand even where `point` is
    /// the old vertex's own: the gate does not ask whether `point` is
    /// that point, and its docs (the crate-internal
    /// `Body::certify_rebased_run`) say why. A fan split that moves
    /// nothing is [`Body::mev_null`], which copies the old point; it
    /// leaves a null edge, and describing that edge is a second call,
    /// [`Body::set_edge_curve`], which can fail on its own and leave
    /// the null edge in place. The two calls are the no-move split, not
    /// one atomic door.
    ///
    /// **The variant family.** `mev` takes the new edge's spec;
    /// [`Body::mev_line`] derives the chord; [`Body::mev_null`] takes no
    /// geometry and moves nothing. The kill side mirrors it: [`Body::kev`]
    /// is keys-only and refuses every merge with a certified member,
    /// one that moves no point included, and [`Body::kev_describing`]
    /// takes a band and the merged fan's re-descriptions, as this door
    /// takes its spec and band — `kev_describing(he, &[], tol)` is the
    /// merge that moves nothing, or moves within band.
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
    /// [`EulerOpError::StaleGeometry`]); the orbit walk from `he1`
    /// closes and reaches `he2` ([`EulerOpError::FanOrbitBroken`]);
    /// every half-edge on it starts at the start vertex
    /// ([`EulerOpError::OrbitBroken`] — tier-1-invalid input: a torn
    /// `next` can walk it through another vertex's half-edge), a
    /// strut's walk included, since the strut splices into that orbit;
    /// both `prev` links resolve (`StaleKey`). `Lone`: the loop resolves (`StaleKey`); it
    /// is empty ([`EulerOpError::LoopNotEmpty`]); its vertex and point
    /// resolve (`StaleKey` / `StaleGeometry`). Then, for both sites,
    /// the geometry gate: `curve` certifies
    /// ([`EulerOpError::Certification`]); and at a `Fan` site, per
    /// edge of the moved run in run order: the run member and its edge
    /// resolve (`StaleKey`); the edge's curve entry resolves
    /// (`StaleGeometry` naming the curve) and is not a null edge the
    /// run moves one end of ([`EulerOpError::RebasedNullEdge`]); the
    /// edge's two end vertices and points resolve (`StaleKey` /
    /// `StaleGeometry`); and its carrier re-certifies against the
    /// endpoints the move gives it ([`EulerOpError::RebasedCarrier`]).
    /// The first edge of the run to fail names the refusal. Last, the
    /// pcurve rows, for both sites: the loops the new halves join, their
    /// faces and those faces' surfaces resolve (`StaleKey` /
    /// `StaleGeometry`); then, only where the site mint selects one of
    /// those faces, the loops the surgery rewires walk
    /// ([`EulerOpError::LoopCycleBroken`] / `StaleKey` /
    /// `StaleGeometry`), and each face's row plan is minted
    /// ([`EulerOpError::PcurveMint`]).
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
        let created = self.mev_with(site, point, NewCurve::Given(curve), tol)?;
        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, ArenaDelta::MEV, "mev");
        Ok(created)
    }

    /// [`Body::mev`]'s plan and surgery, with its curve as a
    /// [`NewCurve`], which the chord sugar derives inside the plan. The
    /// door that calls it declares the postcondition.
    fn mev_with(
        &mut self,
        site: MevSite,
        point: Point3<T>,
        curve: NewCurve<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError> {
        match site {
            MevSite::Fan { he1, he2 } => self.mev_fan(site, he1, he2, point, curve, tol),
            MevSite::Lone { r#loop } => self.mev_lone(site, r#loop, point, curve, tol),
        }
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
        #[cfg(debug_assertions)]
        let before = self.arena_counts();
        let created = self.mev_with(site, point, NewCurve::Chord, tol)?;
        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, ArenaDelta::MEV, "mev");
        Ok(created)
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
    /// **Sense** ([`crate::Face::sense`]): `mef` passes
    /// [`ParentSide::With`], derived on the old face's chart and stated
    /// on any other ([`Body::resolve_face_surface`]).
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
    /// **Pcurve rows** ([`crate::pcurves`]): the moved run's stored
    /// rows are curves stated in the OLD face's chart. Where the new
    /// face is on the same chart — [`FaceSurface::Inherit`], a
    /// [`FaceSurface::Shared`] naming the old key or one sharing its
    /// payload ([`Body::same_chart`]) — they stand; under any other
    /// surface the run's rows are DROPPED ([`Body::drop_rows`]). The
    /// old face's remaining rows are untouched either way. The two
    /// halves this op mints get their rows at the site, as
    /// [`Body::mev`]'s do: when the site mint selects the old face, it
    /// is re-minted with `he_plus` in it, and the new face is minted
    /// with `he_minus` and the run walked in the new face's chart —
    /// whichever chart that is, so a run whose rows were dropped is
    /// derived there again — on the terms [`Body::mev`] states. On a
    /// spline chart other than the old face's, the new face keeps the
    /// drop and is left unminted. A face that takes the old face's
    /// last null edge off it is re-minted whatever the old face missed:
    /// its loops the cut rewires leave complete, and a ring it keeps
    /// keeps what it had.
    ///
    /// **A chord bounding a face minted on another key describes the
    /// edge it is, or is refused.** Where `surface` is not the old
    /// face's key, the chord's two faces wear the old face's key and the
    /// new face's, and `curve`'s description is asked the adjacency
    /// question [`Body::set_edge_curve`] asks
    /// ([`EulerOpError::DescriptionNotAdjacent`], naming no edge: the
    /// chord has no key yet).
    ///
    /// **What it does not ask:** where the new face keeps the old
    /// face's key, whether the chord's description names it. The
    /// boolean pipeline's chord joins rely on that: they mint the chord
    /// with the description of the face glued along it afterwards
    /// (`work/topo/minting-doors-take-a-callers-description-unasked-where-the-new-face-keeps-the-key`).
    ///
    /// **A face minted on another chart is vouched for by the edges it
    /// takes, or refused** ([`RechartDoor::Mef`]). Where `surface` is
    /// not the old face's key, the run's edges and the chord change the
    /// key one of their faces wears, and this keys-only door asks
    /// [`Body::vouch_move`]'s questions of them. A `New` key is one no
    /// description names, so onto a `New` chart off the old face's
    /// payload only scaffold and null edges pass. The lever is the chart the face is minted on: mint it
    /// on a key its certified edges name, or on the old face's and move
    /// it with [`Body::set_face_surfaces_describing`].
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
    /// from `he1` reaches `he2`, and every member of the run it moves,
    /// `[he1 .. he2)`, claims the loop ([`EulerOpError::LoopCycleBroken`]
    /// — a torn `next` can divert the walk through another loop, whose
    /// members the run would take); both `prev` links resolve
    /// (`StaleKey`); the loop's face and the
    /// face's shell resolve (`StaleKey`); `start(he1)` and its point
    /// resolve (`StaleKey` / [`EulerOpError::StaleGeometry`]);
    /// `start(he2)` and its point resolve (`StaleKey` /
    /// `StaleGeometry`). `Lone`:
    /// the loop resolves; it is empty ([`EulerOpError::LoopNotEmpty`]);
    /// its vertex and point resolve; its face and shell resolve.
    /// Then, for both sites, the geometry gates: a
    /// [`FaceSurface::Shared`] key resolves (`StaleGeometry`), a stated
    /// sense agrees with the derived one on the old face's chart
    /// ([`EulerOpError::SenseContradictsChart`]), where the new face's
    /// key is not the old one's `curve`'s description is
    /// adjacency-coherent with the two
    /// ([`EulerOpError::DescriptionNotAdjacent`], `edge: None`), and
    /// `curve` certifies ([`EulerOpError::Certification`]) — the order
    /// [`Body::set_edge_curve`] asks them in. Then, where the new face's key
    /// is not the old one's, no edge of the run is stranded
    /// ([`EulerOpError::RechartStrandsDescriptions`], every one named,
    /// in run order), then every certified edge of the run and a
    /// certified chord name it ([`EulerOpError::RechartUnvouched`], the
    /// same; `StaleKey` / `StaleGeometry` where a key the walk over the
    /// run follows does not resolve). Last, the
    /// pcurve rows, as [`Body::mev`] states them: the loop, its face and
    /// the face's surface resolve; then, only where the site mint
    /// selects that face, the old loop's cycle from `he1` walks
    /// ([`EulerOpError::LoopCycleBroken`]), the new face's chart
    /// resolves (`StaleGeometry`), and the two faces' row plans are
    /// minted ([`EulerOpError::PcurveMint`]).
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
        let created = self.mef_with(site, NewCurve::Given(curve), surface, tol)?;
        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, ArenaDelta::MEF, "mef");
        Ok(created)
    }

    /// [`Body::mef`]'s plan and surgery, with its curve as a
    /// [`NewCurve`], which the chord sugar derives inside the plan. The
    /// door that calls it declares the postcondition.
    fn mef_with(
        &mut self,
        site: MefSite,
        curve: NewCurve<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError> {
        match site {
            MefSite::Chords { he1, he2 } => self.mef_chords(site, he1, he2, curve, surface, tol),
            MefSite::Lone { r#loop } => self.mef_lone(site, r#loop, curve, surface, tol),
        }
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
        #[cfg(debug_assertions)]
        let before = self.arena_counts();
        let created = self.mef_with(site, NewCurve::Chord, FaceSurface::Inherit, tol)?;
        #[cfg(debug_assertions)]
        self.assert_euler_postcondition(before, ArenaDelta::MEF, "mef");
        Ok(created)
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
        curve: NewCurve<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError> {
        // ---- Preconditions: no mutation until every check passes. ----
        let plan = self.mev_fan_plan(he1, he2)?;
        // ---- Geometry gate (still no mutation): certify the spec
        // against old point → new point (D4 ¶2 at attachment), then
        // the moved run's own carriers against the endpoints the move
        // gives them.
        let certified =
            self.certify_edge_spec(curve.spec(false, plan.p_old, point), plan.p_old, point, tol)?;
        self.certify_rebased_run(&plan.run, point, tol)?;
        // ---- The pcurve rows the new halves need (still no mutation). ----
        let rows = self.plan_site_rows(
            &[plan.he1_loop, plan.he2_loop],
            |body| body.mev_fan_site(&plan),
            &certified,
            tol,
        )?;
        // ---- Mutation (infallible from here on). ----
        Ok(self.mev_fan_execute(
            plan,
            point,
            MevCurveMint::Certified(certified),
            rows,
            Provenance::Mev { site },
        ))
    }

    /// The faces a fan `mev` splices into, as the surgery leaves them:
    /// `he_plus` lands before `he1`, `he_minus` before `he2` (both
    /// before `he1`, plus first, for a strut), and every loop keeps its
    /// `first`.
    fn mev_fan_site(&self, plan: &MevFanPlan<T>) -> Result<Vec<SiteFace<T>>, EulerOpError> {
        let (he1, he2) = (plan.he1.key(), plan.he2.key());
        let strut: [SiteHalf; 2] = [SiteHalf::NewPlus, SiteHalf::NewMinus];
        let mut rewired: Vec<(LoopKey, Vec<SiteHalf>)> = Vec::new();
        for lk in [plan.he1_loop, plan.he2_loop] {
            if rewired.iter().any(|(k, _)| *k == lk) {
                continue;
            }
            let cycle = self.site_cycle(lk)?;
            let mut inserts: Vec<(HalfEdgeKey, &[SiteHalf])> = Vec::new();
            if he1 == he2 {
                inserts.push((he1, &strut[..]));
            } else {
                if lk == plan.he1_loop {
                    inserts.push((he1, &strut[..1]));
                }
                if lk == plan.he2_loop {
                    inserts.push((he2, &strut[1..]));
                }
            }
            rewired.push((lk, spliced_before(&cycle, &inserts)));
        }
        let mut faces: Vec<SiteFace<T>> = Vec::new();
        for &(lk, _) in &rewired {
            let face = self
                .get_loop(lk)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Loop(lk),
                })?
                .face;
            if faces.iter().all(|f| f.rows_from != face) {
                faces.push(self.site_face(face, &rewired, None)?);
            }
        }
        Ok(faces)
    }

    /// [`MevSite::Fan`]'s precondition block, shared by [`Body::mev`]
    /// and [`Body::mev_null`] (which replaces the geometry gate with a
    /// null-scaffold mint). Pure — no mutation.
    ///
    /// Beyond resolving every key the split writes, it proves that
    /// every half-edge the orbit walk from `he1` visits starts at the
    /// split vertex ([`Body::require_orbit_starts_at`]), so the run
    /// `[he1 .. he2)` the surgery and the re-basing gate re-base is a
    /// slice of that vertex's orbit and the new halves splice into
    /// that orbit. A torn half-edge in the run would be re-based; one
    /// past `he2`, or anywhere on a strut's walk (whose run is empty),
    /// would have the split splice into a torn orbit.
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
        // orbit walk (bounded, D9), empty for a strut.
        let orbit = self
            .vertex_orbit(he1)
            .ok_or(EulerOpError::FanOrbitBroken { he1, he2 })?;
        let position = orbit
            .iter()
            .position(|&he| he == he2)
            .ok_or(EulerOpError::FanOrbitBroken { he1, he2 })?;
        self.require_orbit_starts_at(&orbit, v, he1)?;
        let run = orbit[..position].to_vec();
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
        rows: Vec<SiteRows<T>>,
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
        crate::pcurves::apply_site_rows(self, rows, Some((he_plus.key(), he_minus.key())));

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
        curve: NewCurve<T>,
        tol: Tol,
    ) -> Result<MevCreated, EulerOpError> {
        // ---- Preconditions. ----
        let (v, p_old) = self.mev_lone_plan(loop_key)?;
        // ---- Geometry gate (still no mutation). ----
        let certified =
            self.certify_edge_spec(curve.spec(false, p_old, point), p_old, point, tol)?;
        // ---- The pcurve rows the new halves need (still no mutation):
        // the empty loop becomes `he_plus → he_minus`, first `he_plus`.
        let rows = self.plan_site_rows(
            &[loop_key],
            |body| {
                let face = body
                    .get_loop(loop_key)
                    .ok_or(EulerOpError::StaleKey {
                        key: EntityId::Loop(loop_key),
                    })?
                    .face;
                let halves = vec![SiteHalf::NewPlus, SiteHalf::NewMinus];
                Ok(vec![body.site_face(face, &[(loop_key, halves)], None)?])
            },
            &certified,
            tol,
        )?;
        // ---- Mutation (infallible from here on). ----
        Ok(self.mev_lone_execute(
            loop_key,
            v,
            point,
            MevCurveMint::Certified(certified),
            rows,
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
        rows: Vec<SiteRows<T>>,
        provenance: Provenance,
    ) -> MevCreated {
        let point_key = self.add_point(point);
        let (curve, w) = self.mint_mev_vertex_and_curve(point_key, v, mint, &provenance);
        let edge = self.mint_edge(curve, &provenance);
        let (he_plus, he_minus) = self.mint_halves(edge, (v, loop_key), (w, loop_key), &provenance);
        crate::pcurves::apply_site_rows(self, rows, Some((he_plus.key(), he_minus.key())));
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
        curve: NewCurve<T>,
        surface: FaceSurface<T>,
        tol: Tol,
    ) -> Result<MefCreated, EulerOpError> {
        // ---- Preconditions. ----
        let (he1_live, he1_data) = self.resolve_half_edge_live(he1)?;
        let (u1, he1_prev) = (he1_data.start, he1_data.prev);
        let (he2_live, he2_data) = self.resolve_half_edge_live(he2)?;
        let (u2, he2_prev) = (he2_data.start, he2_data.prev);
        let loop_key =
            shared_loop(&he1_data, &he2_data).ok_or(EulerOpError::NotSameLoop { he1, he2 })?;
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
        self.require_run_of(run.iter().copied(), loop_key, RunExtent::Part, &[])?;
        // The splice writes through both prev links; prove them now so
        // the mutation below cannot fail midway (atomicity).
        let he1_prev = self.require_live(he1_prev)?;
        let he2_prev = self.require_live(he2_prev)?;
        let face_data = self.get_face(face_key).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face_key),
        })?;
        let (inherit_surface, inherit_sense, shell_key) =
            (face_data.surface, face_data.sense, face_data.shell);
        require_key(&self.shells, shell_key, EntityId::Shell)?;
        let p1 = self.resolve_vertex_point(u1)?;
        // he_minus is minted with start = u2; its point is the
        // certification's end endpoint (he_plus runs u1 → u2).
        let p2 = self.resolve_vertex_point(u2)?;
        // ---- Geometry gates (still no mutation). ----
        // A fragment is a piece of the parent's region, so on the
        // parent's chart it takes the parent's bit. The run's rows are
        // stated in the old face's chart and stand on the new face only
        // where that is the same chart — decided once, here, for the
        // bit, the rows the surgery carries and the rows it mints.
        let resolved = self.resolve_face_surface(
            &surface,
            face_key,
            (inherit_surface, inherit_sense),
            ParentSide::With,
        )?;
        let carried = resolved.on_parent_chart;
        let after = Slot::of_spec(&surface, inherit_surface);
        let spec = curve.spec(u1 == u2, p1, p2);
        require_chord_adjacent(&spec, (inherit_surface, after))?;
        let certified = self.certify_edge_spec(spec, p1, p2, tol)?;
        self.vouch_move(
            RechartDoor::Mef,
            face_key,
            (inherit_surface, after),
            self.run_edges(&run)?,
            |he, _, _| run.contains(&he),
            carried,
            Some(&certified),
        )?;
        // ---- The pcurve rows the new halves need (still no mutation).
        // The old loop becomes `he_plus` then he2's side, the new loop
        // `he_minus` then the run; both are re-anchored at the new half.
        let rows = self.plan_site_rows(
            &[loop_key],
            |body| {
                let (old_side, new_side) = if he1 == he2 {
                    (body.site_cycle_from(he1, loop_key)?, Vec::new())
                } else {
                    let whole = body.site_cycle_from(he1, loop_key)?;
                    let (run, rest) = whole.split_at(run.len());
                    (rest.to_vec(), run.to_vec())
                };
                let with = |new: SiteHalf, side: Vec<HalfEdgeKey>| {
                    core::iter::once(new)
                        .chain(side.into_iter().map(SiteHalf::Existing))
                        .collect::<Vec<_>>()
                };
                let old = body.site_face(
                    face_key,
                    &[(loop_key, with(SiteHalf::NewPlus, old_side))],
                    None,
                )?;
                let new = body.new_site_face(
                    face_key,
                    &surface,
                    !carried,
                    with(SiteHalf::NewMinus, new_side),
                )?;
                Ok(vec![old, new])
            },
            &certified,
            tol,
        )?;

        // ---- Mutation (infallible from here on). ----
        // Minting order (documented on `mef`): surface (for New),
        // curve, edge, loop, face, he_plus, he_minus.
        let provenance = Provenance::Mef { site };
        let surface = self.mint_face_surface(surface, inherit_surface);
        let curve = self.add_curve(certified);
        let edge = self.mint_edge(curve, &provenance);
        let (new_loop, new_face) =
            self.mint_loop_and_face(surface, resolved.sense, shell_key, &provenance);
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
        // The run's rows are stated in the old face's chart, and stand
        // on the new face only where that is the same chart. The run
        // is exactly the set of half-edges whose `parent_loop` moved
        // above, so it is exactly what the new loop's walk
        // (`pcurves::loop_rows`) attributes to the moved half-edges
        // once re-anchored. The drop comes first, so the site mint's
        // rows for the new loop — `he_minus` and the run, walked on the
        // new face's chart — are what the map keeps.
        if !carried {
            self.drop_rows(run.iter().copied());
        }
        crate::pcurves::apply_site_rows(self, rows, Some((he_plus, he_minus)));
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
        curve: NewCurve<T>,
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
        require_key(&self.shells, shell_key, EntityId::Shell)?;
        // ---- Geometry gates (still no mutation): the new face's bit
        // and chart as the Chords site decides them, then the self-loop
        // edge, which closes at the lone vertex — both endpoints are
        // its point.
        let resolved = self.resolve_face_surface(
            &surface,
            face_key,
            (inherit_surface, inherit_sense),
            ParentSide::With,
        )?;
        let after = Slot::of_spec(&surface, inherit_surface);
        let spec = curve.spec(true, anchor, anchor);
        require_chord_adjacent(&spec, (inherit_surface, after))?;
        let certified = self.certify_edge_spec(spec, anchor, anchor, tol)?;
        let carried = resolved.on_parent_chart;
        self.vouch_move(
            RechartDoor::Mef,
            face_key,
            (inherit_surface, after),
            [],
            |_, _, _| false,
            carried,
            Some(&certified),
        )?;
        // ---- The pcurve rows the new halves need (still no mutation):
        // each half is a one-half-edge loop of its own face.
        let rows = self.plan_site_rows(
            &[loop_key],
            |body| {
                let old = body.site_face(face_key, &[(loop_key, vec![SiteHalf::NewPlus])], None)?;
                let new =
                    body.new_site_face(face_key, &surface, !carried, vec![SiteHalf::NewMinus])?;
                Ok(vec![old, new])
            },
            &certified,
            tol,
        )?;

        // ---- Mutation (infallible from here on). ----
        // Same minting order as Chords: surface (for New), curve,
        // edge, loop, face, he_plus, he_minus.
        let provenance = Provenance::Mef { site };
        let surface = self.mint_face_surface(surface, inherit_surface);
        let curve = self.add_curve(certified);
        let edge = self.mint_edge(curve, &provenance);
        let (new_loop, new_face) =
            self.mint_loop_and_face(surface, resolved.sense, shell_key, &provenance);
        let (he_plus, he_minus) = self.mint_halves(edge, (v, loop_key), (v, new_loop), &provenance);
        crate::pcurves::apply_site_rows(self, rows, Some((he_plus.key(), he_minus.key())));
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

    /// Proves that every member of a closed orbit walk
    /// ([`Body::vertex_orbit`]), or of any list of half-edges a plan
    /// takes as `v`'s, starts at `v`, refusing
    /// [`EulerOpError::OrbitBroken`] naming the walk's origin otherwise.
    ///
    /// The walk steps `next(mate(·))` and reads no start vertex, so a
    /// torn `next` can close it through another vertex's half-edges; a
    /// plan that moves or splices into a vertex's orbit proves its walk
    /// here, and a kill proves the anchors it writes through
    /// [`Body::require_kill_anchors`], which calls this. The validator
    /// ([`crate::validate::validate`]) reports the same fault in pass 6.
    /// A member that does not resolve fails the proof.
    pub(crate) fn require_orbit_starts_at(
        &self,
        members: &[HalfEdgeKey],
        v: VertexKey,
        origin: HalfEdgeKey,
    ) -> Result<(), EulerOpError> {
        if members
            .iter()
            .all(|&member| self.half_edges.get(member).map(|he| he.start) == Some(v))
        {
            Ok(())
        } else {
            Err(EulerOpError::OrbitBroken { he: origin })
        }
    }

    /// The first half-edge in arena order, other than `besides`, that
    /// starts at `v`: the incidence scan behind a kill's proof that it
    /// leaves `v` lone ([`Body::require_kill_anchors`]'s `Lone` proof) or
    /// that nothing it keeps starts at a vertex it removes
    /// ([`Body::require_vertex_unnamed`]). It reads the whole arena,
    /// since no walk from a killed half reaches a stranger a torn start
    /// put at `v`; a plan refuses [`EulerOpError::OrbitBroken`] on a hit.
    pub(crate) fn starts_at_besides(
        &self,
        v: VertexKey,
        besides: &[HalfEdgeKey],
    ) -> Option<HalfEdgeKey> {
        let mut skip: SecondaryMap<HalfEdgeKey, ()> = SecondaryMap::new();
        for &he in besides {
            skip.insert(he, ());
        }
        self.half_edges
            .iter()
            .find(|&(he, data)| data.start == v && !skip.contains_key(he))
            .map(|(he, _)| he)
    }

    /// The first loop in arena order, other than `besides`, that is
    /// `Empty` at `v`: the scan behind a kill's proof that the loop it
    /// empties at `v` is the only one there
    /// ([`Body::require_kill_anchors`]) and that no loop it keeps holds a
    /// vertex it removes ([`Body::require_vertex_unnamed`]). It reads the
    /// whole loop arena.
    pub(crate) fn empty_at_besides(&self, v: VertexKey, besides: &[LoopKey]) -> Option<LoopKey> {
        self.loops
            .iter()
            .find(|&(l, data)| {
                data.boundary == LoopBoundary::Empty { vertex: v } && !besides.contains(&l)
            })
            .map(|(l, _)| l)
    }

    /// Proves the anchor writes of a kill that removes the half-edges
    /// `killed`, before it mutates: every `emanating` write, then the
    /// run it re-parents, then every loop write.
    ///
    /// One `(vertex, anchor, origin)` per `emanating` write, refusing
    /// [`EulerOpError::OrbitBroken`] naming `origin`, the killed half
    /// that starts at `vertex`, at the first that fails. A
    /// [`KillAnchor::Step`] starts at `vertex`
    /// ([`Body::require_orbit_starts_at`]); a [`KillAnchor::Merged`] is
    /// proven by the caller's orbit walk and asks nothing here; a
    /// [`KillAnchor::Lone`] leaves `vertex` lone: no half-edge but
    /// `killed` starts at it, and a loop this kill writes is `Empty` at
    /// it. A kill that writes one vertex twice proves both writes.
    ///
    /// The run is proven by [`Body::require_run_of`], with `killed`.
    ///
    /// One `(loop, boundary)` per loop the kill keeps and re-anchors,
    /// and the loop the run mints if it mints one
    /// ([`KillInto::Minted`]), refusing `LoopCycleBroken` naming the
    /// loop (for a minted loop, the run's) at the first that fails. A
    /// `Cycle`'s `first` is not killed and lies in the loop once the
    /// kill has run: its `parent_loop`, or the run's destination for a
    /// member of the run. An `Empty` loop holds a vertex that `writes`
    /// anchors [`KillAnchor::Lone`], keeps no member once the kill has
    /// run, and is the only loop `Empty` at that vertex.
    ///
    /// Each kill reads an anchor one `next` step from a killed half, and
    /// reads "no anchor" where that step lands on a killed half. A torn
    /// `next` can put the step on another vertex or into another loop,
    /// or land it on a killed half where the vertex or the loop keeps
    /// other members; a cycle walk it diverts through another loop
    /// hands the kill that loop's members as its run; and a torn start
    /// can put the vertex a kill empties a loop at on another lone
    /// vertex, leaving its own vertex with neither. The orbit and cycle
    /// walks from the killed half take the torn step first, so they
    /// close on the killed halves either way; the `Lone` and `Empty`
    /// proofs read the whole arena instead, bounded as the kill's orphan
    /// sweeps are. The validator reports the faults this refuses in pass
    /// 5 (`EmanatingStartMismatch`, `LoneVertexWithIncidence`,
    /// `EmptyLoopVertexWithEmanating`), in its cycle pass
    /// (`ParentLoopMismatch`, `UnreachableHalfEdge`), and as
    /// `MultiplyOwned` and `DanglingTopology`.
    pub(crate) fn require_kill_anchors(
        &self,
        writes: &[(VertexKey, KillAnchor, HalfEdgeKey)],
        loops: &[(LoopKey, LoopBoundary)],
        killed: &[HalfEdgeKey],
        run: Option<KillRun<'_>>,
    ) -> Result<(), EulerOpError> {
        // Every loop write: the loop it lands on (`None`: the one the
        // run mints), the loop a refusal names, and the boundary.
        let minted = run.and_then(|run| match run.into {
            KillInto::Minted(boundary) => Some((None, run.from, boundary)),
            KillInto::Kept(_) => None,
        });
        let written: Vec<(Option<LoopKey>, LoopKey, LoopBoundary)> = loops
            .iter()
            .map(|&(r#loop, boundary)| (Some(r#loop), r#loop, boundary))
            .chain(minted)
            .collect();
        for &(vertex, anchor, origin) in writes {
            match anchor {
                KillAnchor::Step(step) => {
                    self.require_orbit_starts_at(core::slice::from_ref(&step), vertex, origin)?;
                }
                KillAnchor::Merged(_) => {}
                KillAnchor::Lone => {
                    let held = written
                        .iter()
                        .any(|&(_, _, boundary)| boundary == LoopBoundary::Empty { vertex });
                    if self.starts_at_besides(vertex, killed).is_some() || !held {
                        return Err(EulerOpError::OrbitBroken { he: origin });
                    }
                }
            }
        }
        let mut moved: SecondaryMap<HalfEdgeKey, ()> = SecondaryMap::new();
        let mut joins = None;
        if let Some(KillRun {
            members,
            from,
            extent,
            into,
        }) = run
        {
            moved = self.require_run_of(members.iter().map(|m| m.key()), from, extent, killed)?;
            joins = match into {
                KillInto::Kept(r#loop) => Some(r#loop),
                KillInto::Minted(_) => None,
            };
        }
        let stays_in = |he: HalfEdgeKey, data: &HalfEdge, target: Option<LoopKey>| {
            !killed.contains(&he)
                && if moved.contains_key(he) {
                    joins == target
                } else {
                    Some(data.parent_loop) == target
                }
        };
        for (target, name, boundary) in written {
            let holds = match boundary {
                LoopBoundary::Cycle { first } => self
                    .half_edges
                    .get(first)
                    .is_some_and(|data| stays_in(first, data, target)),
                LoopBoundary::Empty { vertex } => {
                    writes
                        .iter()
                        .any(|&(lone, anchor, _)| lone == vertex && anchor == KillAnchor::Lone)
                        && !self
                            .half_edges
                            .iter()
                            .any(|(he, data)| stays_in(he, data, target))
                        && self.empty_at_besides(vertex, target.as_slice()).is_none()
                }
            };
            if !holds {
                return Err(EulerOpError::LoopCycleBroken { r#loop: name });
            }
        }
        Ok(())
    }

    /// Proves the run a plan's cycle walk took from `from` and moves
    /// out of it, before it mutates: every member claims `from`, so the
    /// move takes nothing out of a third loop; and for a
    /// [`RunExtent::Whole`] run, whose loop the plan removes, no
    /// half-edge but the run and `killed` claims `from`, so none is left
    /// naming a dead loop. Refuses [`EulerOpError::LoopCycleBroken`]
    /// naming `from` otherwise. Returns the run as a set.
    ///
    /// The walk steps `next` and reads no `parent_loop`, so a torn
    /// `next` can divert it through another loop and back, or close it
    /// past a member. The second proof reads the whole arena, bounded
    /// as [`Body::require_kill_anchors`]'s `Lone` proof is. The
    /// validator reports these faults in its cycle pass
    /// (`ParentLoopMismatch`, `UnreachableHalfEdge`) and as
    /// `DanglingTopology`.
    pub(crate) fn require_run_of(
        &self,
        members: impl IntoIterator<Item = HalfEdgeKey>,
        from: LoopKey,
        extent: RunExtent,
        killed: &[HalfEdgeKey],
    ) -> Result<SecondaryMap<HalfEdgeKey, ()>, EulerOpError> {
        let broken = EulerOpError::LoopCycleBroken { r#loop: from };
        let mut run: SecondaryMap<HalfEdgeKey, ()> = SecondaryMap::new();
        for member in members {
            if self.half_edges.get(member).map(|data| data.parent_loop) != Some(from) {
                return Err(broken);
            }
            run.insert(member, ());
        }
        if extent == RunExtent::Whole
            && self.half_edges.iter().any(|(he, data)| {
                data.parent_loop == from && !run.contains_key(he) && !killed.contains(&he)
            })
        {
            return Err(broken);
        }
        Ok(run)
    }

    /// Proves that no record a kill keeps names the vertex `v` it
    /// removes: no half-edge but `moved` starts at `v`, refusing
    /// [`EulerOpError::OrbitBroken`] naming the first that does in arena
    /// order; then no loop but `killed` is `Empty` at `v`, refusing
    /// [`EulerOpError::KillLeavesDangling`] from the first that is to
    /// `v`. `moved` is every half-edge the kill removes or re-bases off `v`,
    /// and `killed` every loop it removes.
    ///
    /// A plan reads what starts at `v` from an orbit walk, or finds `v`
    /// lone, and a torn `next`, start or boundary puts on `v` a record
    /// that neither reaches. Both proofs read the whole arena, one pass
    /// over the half-edges and one over the loops, bounded as
    /// [`Body::require_run_of`]'s `Whole` proof is. The validator reports
    /// what they refuse as `DanglingTopology` once `v` is gone.
    pub(crate) fn require_vertex_unnamed(
        &self,
        v: VertexKey,
        moved: &[HalfEdgeKey],
        killed: &[LoopKey],
    ) -> Result<(), EulerOpError> {
        if let Some(he) = self.starts_at_besides(v, moved) {
            return Err(EulerOpError::OrbitBroken { he });
        }
        match self.empty_at_besides(v, killed) {
            Some(r#loop) => Err(EulerOpError::KillLeavesDangling {
                from: EntityId::Loop(r#loop),
                to: EntityId::Vertex(v),
            }),
            None => Ok(()),
        }
    }

    /// `he`'s mate, read from `he`'s own edge and proven its pair: `he`
    /// resolves, its edge resolves ([`EulerOpError::StaleKey`]), the
    /// edge claims `he` ([`EulerOpError::UnclaimedHalfEdge`]), the mate
    /// the claim gives resolves (`StaleKey`), and the two are the edge's
    /// halves ([`require_halves`]: [`EulerOpError::NotSameEdge`]), in
    /// that order. The one hop from a half-edge to its mate a plan may
    /// read without proving more; [`Body::mate`] answers `None` for
    /// every one of these faults alike and proves no pair.
    pub(crate) fn proven_mate(&self, he: HalfEdgeKey) -> Result<ProvenMate<'_>, EulerOpError> {
        let he_data = self.resolve_half_edge(he)?;
        let edge = he_data.edge;
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let mate = edge_data
            .claim(he)
            .ok_or(EulerOpError::UnclaimedHalfEdge { he, edge })?
            .mate;
        let mate_data = self.resolve_half_edge(mate)?;
        require_halves(edge, edge_data, he, (mate, mate_data.edge))?;
        Ok(ProvenMate {
            he_data,
            edge,
            edge_data,
            mate,
            mate_data,
        })
    }

    /// Proves that no half-edge but `halves`, the two a kill removes with
    /// it, names the edge `edge`, refusing
    /// [`EulerOpError::UnclaimedHalfEdge`] naming the first that does in
    /// arena order: a torn `edge` field the edge's slots do not reach.
    /// [`require_halves`] proves the two are the edge's; this reads the
    /// whole half-edge arena, bounded as [`Body::require_run_of`]'s
    /// `Whole` proof is, so a kill runs it after every cheaper check.
    pub(crate) fn require_edge_unnamed(
        &self,
        edge: EdgeKey,
        halves: [HalfEdgeKey; 2],
    ) -> Result<(), EulerOpError> {
        match self
            .half_edges
            .iter()
            .find(|&(he, data)| data.edge == edge && !halves.contains(&he))
        {
            Some((he, _)) => Err(EulerOpError::UnclaimedHalfEdge { he, edge }),
            None => Ok(()),
        }
    }

    /// Proves that no record a kill keeps names either of the two
    /// half-edges `halves` it removes, once it has written what
    /// `clearing` says: no half-edge's `next` or `prev`, as the kill's
    /// links leave it, then no loop's `first`, no vertex's `emanating`
    /// and no slot of an edge but those `clearing` clears. Refuses
    /// [`EulerOpError::KillLeavesDangling`] naming the first record that
    /// does, in that order and in arena order within each arena.
    ///
    /// A plan reads a killed half's neighbours from its own `next` and
    /// `prev`, the loops it re-anchors from the halves' `parent_loop`,
    /// and the endpoints from their `start`, and a torn link, `first`,
    /// `emanating` or slot names a killed half from a record none of
    /// those reach. The loops and vertices `clearing` clears are the
    /// ones whose anchor the kill writes, which
    /// [`Body::require_kill_anchors`] proves. The proof reads every
    /// half-edge, loop, vertex and edge once, bounded as
    /// [`Body::require_run_of`]'s `Whole` proof is.
    pub(crate) fn require_killed_halves_unnamed(
        &self,
        halves: [HalfEdgeKey; 2],
        clearing: Clearing<'_>,
    ) -> Result<(), EulerOpError> {
        let killed = |he: HalfEdgeKey| halves.contains(&he);
        let by_link = self
            .half_edges
            .iter()
            .filter(|&(h, _)| !killed(h))
            .find_map(|(h, data)| {
                clearing
                    .links_of(h, data)
                    .into_iter()
                    .find(|&he| killed(he))
                    .map(|he| (EntityId::HalfEdge(h), he))
            });
        let by_loop = || {
            self.loops.iter().find_map(|(l, data)| match data.boundary {
                LoopBoundary::Cycle { first } if killed(first) && !clearing.clears_loop(l) => {
                    Some((EntityId::Loop(l), first))
                }
                _ => None,
            })
        };
        let by_vertex = || {
            self.vertices.iter().find_map(|(v, data)| {
                data.emanating
                    .filter(|&he| killed(he) && !clearing.clears_vertex(v))
                    .map(|he| (EntityId::Vertex(v), he))
            })
        };
        let by_edge = || {
            self.edges.iter().find_map(|(e, data)| {
                [data.he_plus, data.he_minus]
                    .into_iter()
                    .find(|&he| killed(he) && !clearing.clears_edge(e))
                    .map(|he| (EntityId::Edge(e), he))
            })
        };
        match by_link.or_else(by_loop).or_else(by_vertex).or_else(by_edge) {
            Some((from, he)) => Err(EulerOpError::KillLeavesDangling {
                from,
                to: EntityId::HalfEdge(he),
            }),
            None => Ok(()),
        }
    }

    /// Proves that no face but those in `clearing` lists the loop `l` a
    /// kill removes, as its outer loop or a ring, refusing
    /// [`EulerOpError::KillLeavesDangling`] naming the first that does in
    /// arena order. The half-edges that claim `l` are
    /// [`Body::require_run_of`]'s to prove.
    ///
    /// A plan reads the face `l` leaves from `l`'s own `face`, and a torn
    /// `rings` or `outer` lists it on a face that field does not name. The
    /// proof reads every face's list once, bounded as
    /// [`Body::require_run_of`]'s `Whole` proof is.
    pub(crate) fn require_loop_unlisted(
        &self,
        l: LoopKey,
        clearing: Clearing<'_>,
    ) -> Result<(), EulerOpError> {
        let stray = self.faces.iter().find(|&(face, data)| {
            (data.outer == l || data.rings.contains(&l)) && !clearing.clears_face(face)
        });
        match stray {
            Some((face, _)) => Err(EulerOpError::KillLeavesDangling {
                from: EntityId::Face(face),
                to: EntityId::Loop(l),
            }),
            None => Ok(()),
        }
    }

    /// Proves that no loop or shell but those in `clearing` names the face
    /// `f` a kill removes: no loop's `face` is `f`, then no shell lists
    /// it, refusing [`EulerOpError::KillLeavesDangling`] naming the first
    /// that does in arena order.
    ///
    /// A plan reads a face's loops from its `outer` and `rings`, and its
    /// shell from its `shell`, and a torn `face` or `faces` names `f` from
    /// a record neither reaches. The proof reads every loop and every
    /// shell's list once, bounded as [`Body::require_run_of`]'s `Whole`
    /// proof is.
    pub(crate) fn require_face_unnamed(
        &self,
        f: FaceKey,
        clearing: Clearing<'_>,
    ) -> Result<(), EulerOpError> {
        let by_loop = self
            .loops
            .iter()
            .find(|&(l, data)| data.face == f && !clearing.clears_loop(l))
            .map(|(l, _)| EntityId::Loop(l));
        let by_shell = || {
            self.shells
                .iter()
                .find(|&(s, data)| data.faces.contains(&f) && !clearing.clears_shell(s))
                .map(|(s, _)| EntityId::Shell(s))
        };
        match by_loop.or_else(by_shell) {
            Some(from) => Err(EulerOpError::KillLeavesDangling {
                from,
                to: EntityId::Face(f),
            }),
            None => Ok(()),
        }
    }

    /// Proves that no face or solid but those in `clearing` names the
    /// shell `s` a kill removes: no face's `shell` is `s`, then no solid
    /// lists it, refusing [`EulerOpError::KillLeavesDangling`] naming the
    /// first that does in arena order. Read as
    /// [`Body::require_face_unnamed`] reads, and bounded as it is.
    pub(crate) fn require_shell_unnamed(
        &self,
        s: ShellKey,
        clearing: Clearing<'_>,
    ) -> Result<(), EulerOpError> {
        let by_face = self
            .faces
            .iter()
            .find(|&(f, data)| data.shell == s && !clearing.clears_face(f))
            .map(|(f, _)| EntityId::Face(f));
        let by_solid = || {
            self.solids
                .iter()
                .find(|&(so, data)| data.shells.contains(&s) && !clearing.clears_solid(so))
                .map(|(so, _)| EntityId::Solid(so))
        };
        match by_face.or_else(by_solid) {
            Some(from) => Err(EulerOpError::KillLeavesDangling {
                from,
                to: EntityId::Shell(s),
            }),
            None => Ok(()),
        }
    }

    /// Proves that no shell but those in `clearing` names the solid a
    /// kill removes as its `solid`, refusing
    /// [`EulerOpError::KillLeavesDangling`] naming the first that does in
    /// arena order. Read as [`Body::require_face_unnamed`] reads, and
    /// bounded as it is.
    pub(crate) fn require_solid_unnamed(
        &self,
        solid: SolidKey,
        clearing: Clearing<'_>,
    ) -> Result<(), EulerOpError> {
        match self
            .shells
            .iter()
            .find(|&(s, data)| data.solid == solid && !clearing.clears_shell(s))
        {
            Some((s, _)) => Err(EulerOpError::KillLeavesDangling {
                from: EntityId::Shell(s),
                to: EntityId::Solid(solid),
            }),
            None => Ok(()),
        }
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

    /// `edge`'s two endpoint points, `he_plus` forward order (the
    /// interval's `t₀` end is `start(he_plus)`, its `t₁` end
    /// `start(he_minus)`), once `run` has moved onto a vertex at
    /// `p_new`: `p_new` at a half in the run, the half's current start
    /// point elsewhere. The one reading of "the endpoints the move
    /// gives it" that both re-basing doors certify against. Pure.
    pub(crate) fn rebased_endpoints(
        &self,
        edge: EdgeKey,
        run: &[HalfEdgeKey],
        p_new: Point3<T>,
    ) -> Result<(Point3<T>, Point3<T>), EulerOpError> {
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        let endpoint = |he: HalfEdgeKey| -> Result<Point3<T>, EulerOpError> {
            if run.contains(&he) {
                return Ok(p_new);
            }
            self.resolve_vertex_point(self.resolve_half_edge(he)?.start)
        };
        Ok((endpoint(edge_data.he_plus)?, endpoint(edge_data.he_minus)?))
    }

    /// Every edge with a half-edge in `run`, once, in run order: the
    /// unit both re-basing gates give one verdict per, since a
    /// self-loop at the moved vertex has both halves in the run. Pure.
    pub(crate) fn run_edges(&self, run: &[HalfEdgeKey]) -> Result<Vec<EdgeKey>, EulerOpError> {
        let mut edges: Vec<EdgeKey> = Vec::with_capacity(run.len());
        for &moved in run {
            let edge = self.resolve_half_edge(moved)?.edge;
            if !edges.contains(&edge) {
                edges.push(edge);
            }
        }
        Ok(edges)
    }

    /// What moving `run` makes of `edge` before any band is asked: its
    /// certified carrier, which the move re-bases; `None` for a null
    /// edge with BOTH halves in the run, which moves whole and stays
    /// one vertex by structure; and [`EulerOpError::RebasedNullEdge`]
    /// for a null edge with one half in it
    /// ([`Body::certify_rebased_run`] says why). The null arm of both
    /// re-basing gates and of [`Body::kev`]'s keys-only one. Pure.
    pub(crate) fn rebased_carrier(
        &self,
        edge: EdgeKey,
        run: &[HalfEdgeKey],
    ) -> Result<Option<&EdgeCurve<T>>, EulerOpError> {
        let edge_data = self.get_edge(edge).ok_or(EulerOpError::StaleKey {
            key: EntityId::Edge(edge),
        })?;
        match self.get_curve_geom(edge_data.curve) {
            Some(crate::null::CurveGeom::Certified(curve)) => Ok(Some(curve)),
            Some(crate::null::CurveGeom::NullScaffold(_))
                if run.contains(&edge_data.he_plus) && run.contains(&edge_data.he_minus) =>
            {
                Ok(None)
            }
            Some(crate::null::CurveGeom::NullScaffold(_)) => {
                Err(EulerOpError::RebasedNullEdge { edge })
            }
            None => Err(EulerOpError::StaleGeometry {
                key: GeomRef::Curve(edge_data.curve),
            }),
        }
    }

    /// The **re-basing gate**: every edge of a run of half-edges about
    /// to start at a vertex whose point is `p_new`, re-certified
    /// against the endpoints it will have once the run has moved.
    ///
    /// A fan site re-parents half-edges, not carriers: an
    /// edge whose start moves keeps the [`EdgeCurve`] it was certified
    /// with, which pins `carrier(t₀)` to the point the edge USED to
    /// run from. So the operator asks, before it mutates, whether that
    /// certificate is still true of the edge it is about to make —
    /// through [`EdgeCurve::recertify`], the same door `split_edge`
    /// certifies its children through, which re-derives rather than
    /// trusting the stored certificate. Where it holds the certificate
    /// is carried untouched (the coincident-point case: no endpoint
    /// moved, so nothing is re-minted and no byte changes); where it
    /// fails because of the move the operator refuses
    /// [`EulerOpError::RebasedCarrier`] naming the edge.
    ///
    /// **Exact re-certification, never a re-fit.** The description is
    /// authoritative (D2/U2) and the carrier is its certified cache,
    /// so re-fitting a moved edge would mean minting a description no
    /// modeler stated: an `Intersection`'s locus is its two surfaces'
    /// and moving an endpoint off it is a contradiction, a `Chart`
    /// image's pcurve IS the authority record, a `Scaffold` names a
    /// mapped source, and even a conventional line or circle would
    /// have its direction and interval recomputed — the silent
    /// geometry move [`Body::describe_at_rest`] refuses to make. A
    /// re-description is [`Body::set_edge_curve`]'s, with the caller's
    /// own spec.
    ///
    /// **A pre-existing staleness is carried, not refused.** Where the
    /// re-certification fails on an ENDPOINT residual, the gate asks
    /// the same question of the endpoints the edge has NOW; where that
    /// fails the same way, the carrier already missed its own endpoint
    /// and this move is not what made it false. Refusing there would
    /// name a defect this operation did not create, on an edge it may
    /// not even touch; tier 3 reports such a carrier at rest, and the
    /// gate's claim is the narrow one: no edge's carrier is made false
    /// BY THIS MOVE. (No re-basing door leaves one — this gate refuses,
    /// and both kill doors refuse or re-describe what a merge would
    /// strand — so the arm is fed only by a body that arrives with
    /// one.) Every other refusal is unconditional.
    ///
    /// **A null edge is refused where the run moves one of its ends and
    /// not the other** ([`EulerOpError::RebasedNullEdge`]). It carries
    /// no certificate ([`crate::CurveGeom::NullScaffold`]); what it has
    /// instead is the F9 shape [`Body::mev_null`] minted, a zero-length
    /// edge at ONE point. Where the run holds BOTH its halves (a null
    /// self-loop at the moved vertex), both ends move onto the one new
    /// vertex, so the edge stays one vertex and one point by structure
    /// and the gate carries it without asking anything. Where the run
    /// holds exactly one half, the moved end lands on `p_new` and the
    /// other stays on `p_old`, and whether those are one point is the
    /// exact question below, which the gate does not ask: it refuses,
    /// at the old point as at any other.
    ///
    /// **The question this gate does not ask: is `p_new` the point
    /// `p_old`, bit for bit.** Bitwise, not within band, because a point
    /// within band of the old one is still a move, and carrying a
    /// certificate (or a null edge's one-point shape) across it is the
    /// staleness this gate exists to close; a band decision
    /// (`Decide::sign_within` over a distance) is the wrong question and
    /// has an `Indeterminate` arm besides. At this gate's bound,
    /// `T: Real`, there is no door for the exact question: `Point3<T>`
    /// derives no `PartialEq`, `Real` offers no bit accessor, and
    /// `Real::register_equal` is a site-allowlisted identity axiom, not
    /// an equality. A bit comparison of points does exist in the tree
    /// one bound up, at `T: Bounds`: `crate::query`'s `same_point_bits`,
    /// which compares `lo()`/`hi()` bits and which `rim_of`'s circle
    /// identity uses in production. The absence of a door is therefore
    /// not what decides; `docs/DESIGN.md`'s standing outcome that
    /// production bit-identity coincidence checking is RETIRED is, and
    /// whether a kernel gate may ask this question, and through which
    /// door, is on
    /// `work/topo/the-re-basing-gate-refuses-m7-8-where-nothing-moves.md`.
    /// Every other doc that meets this question points here.
    ///
    /// **What that costs.** The plane × NURBS class (M7-8) needs an
    /// injected lane this bound cannot supply, so `recertify` answers
    /// `NurbsLaneNotSupplied` exactly as `split_edge` does on the same class —
    /// an operator makes no claim it cannot derive, and a claim it
    /// cannot derive is not a licence to move the edge. `recertify`
    /// answers that before any endpoint check, so the gate refuses where
    /// the new point is the old vertex's own too; and a run moving one
    /// half of a null edge is refused at the old point too. Both are
    /// over-refusals of a move that moves nothing. A fan split that
    /// keeps the old point is [`Body::mev_null`]'s, which copies the old
    /// point and so skips this gate structurally; every kernel run site
    /// (the splitting and boolean pipelines' null-edge insertion) goes
    /// through it. That is the no-move SPLIT, not an atomic or an
    /// at-rest-certified door: its new edge is a null edge, which tier 2
    /// refuses at rest; describing it afterwards through
    /// [`Body::set_edge_curve`] is a second call, which can fail on its
    /// own and leave that null edge behind; and on the M7-8 pillow the
    /// only spec that certifies there (a closed circle at the point)
    /// passes tiers 1 and 2 and not tier 3, since the edge lies where
    /// its two faces meet in a line.
    ///
    /// Pure (no mutation) — one verdict per EDGE in run order (D9),
    /// since a self-loop at the moved vertex has both halves in the
    /// run and both endpoints moving.
    pub(crate) fn certify_rebased_run(
        &self,
        run: &[HalfEdgeKey],
        p_new: Point3<T>,
        tol: Tol,
    ) -> Result<(), EulerOpError> {
        let band = Band::linear(tol).map_err(|e| EulerOpError::Certification {
            error: CertifyError::Band(e),
        })?;
        for edge_key in self.run_edges(run)? {
            let Some(curve) = self.rebased_carrier(edge_key, run)? else {
                continue;
            };
            let (p_start, p_end) = self.rebased_endpoints(edge_key, run, p_new)?;
            let surfaces = |k| self.surfaces.get(k).cloned();
            let Err(error) = curve.recertify(p_start, p_end, surfaces, band) else {
                continue;
            };
            if is_endpoint_residual(error) {
                // The carrier misses an endpoint AFTER the move. Ask
                // the same question of the endpoints it has NOW: an
                // identical answer means it already missed one, so
                // this move is not what made it false.
                let (now_start, now_end) = self.rebased_endpoints(edge_key, &[], p_new)?;
                if curve.recertify(now_start, now_end, surfaces, band).err() == Some(error) {
                    continue;
                }
            }
            return Err(EulerOpError::RebasedCarrier {
                edge: edge_key,
                error,
            });
        }
        Ok(())
    }

    /// Resolves `spec` against `parent` — the face a new face is
    /// minted from, or the face re-charted in place — in a door's plan
    /// phase: whether the spec lands on the parent's chart, and the
    /// [`crate::entity::Face::sense`] the face carries. D1's
    /// orientation bullet, implemented once:
    ///
    /// - **On the parent's chart** ([`Body::same_chart`]: one key, or
    ///   keys sharing one payload; `Inherit` always), the bit is
    ///   derived from the operator's topology: the parent's for
    ///   [`ParentSide::With`], its negation for
    ///   [`ParentSide::Against`]. A spec that states the other bit is
    ///   refused.
    /// - **On any other chart**, the stated bit is written as given.
    ///   Nothing is defaulted: the chart normal is the caller's, and so
    ///   is the side the material lies on against it.
    ///
    /// Where the parent bounds no region yet (an `mvfs` seed), its bit
    /// and so the derived one are provisional; the constructor states
    /// the honest bit when it charts the face.
    ///
    /// The chart question is asked here once, before any key is
    /// minted, and the door takes [`ResolvedFace::on_parent_chart`]
    /// for its pcurve rows as well as its bit.
    ///
    /// # Errors
    ///
    /// [`EulerOpError::StaleGeometry`] if a `Shared` key does not
    /// resolve; then [`EulerOpError::SenseContradictsChart`].
    pub(crate) fn resolve_face_surface(
        &self,
        spec: &FaceSurface<T>,
        parent: FaceKey,
        (parent_surface, parent_sense): (SurfaceKey, bool),
        side: ParentSide,
    ) -> Result<ResolvedFace, EulerOpError> {
        if let FaceSurface::Shared { key, .. } = spec
            && !self.surfaces.contains_key(*key)
        {
            return Err(EulerOpError::StaleGeometry {
                key: GeomRef::Surface(*key),
            });
        }
        let on_parent_chart = self.same_chart_spec(parent_surface, spec);
        let derived = match side {
            ParentSide::With => parent_sense,
            ParentSide::Against => !parent_sense,
        };
        let sense = match *spec {
            FaceSurface::Inherit => derived,
            FaceSurface::New { sense, .. } | FaceSurface::Shared { sense, .. } => {
                if on_parent_chart && sense != derived {
                    return Err(EulerOpError::SenseContradictsChart {
                        face: parent,
                        stated: sense,
                        derived,
                    });
                }
                sense
            }
        };
        Ok(ResolvedFace {
            on_parent_chart,
            sense,
        })
    }

    /// Mutation half of [`FaceSurface`] resolution: the new face's
    /// surface key — `inherit` for `Inherit`, a fresh insertion for
    /// `New` (part of the op's documented minting order), the given key
    /// for `Shared` (pre-validated by
    /// [`Body::resolve_face_surface`]).
    pub(crate) fn mint_face_surface(
        &mut self,
        spec: FaceSurface<T>,
        inherit: SurfaceKey,
    ) -> SurfaceKey {
        match spec {
            FaceSurface::Inherit => inherit,
            FaceSurface::New { surface, .. } => self.add_surface(surface),
            FaceSurface::Shared { key, .. } => key,
        }
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

    /// **The pcurve rows the surgery's new halves need**, one plan per
    /// face they land on, decided before the surgery mutates: the site
    /// mint as an Euler operator runs it, over [`Body::plan_site_mint`].
    /// `faces` describes the faces as the surgery will leave them.
    pub(crate) fn plan_site_rows(
        &self,
        touched: &[LoopKey],
        faces: impl FnOnce(&Self) -> Result<Vec<SiteFace<T>>, EulerOpError>,
        edge: &EdgeCurve<T>,
        tol: Tol,
    ) -> Result<Vec<SiteRows<T>>, EulerOpError> {
        self.plan_site_mint(touched, |body, _| faces(body), Some(edge), tol)
    }

    /// **A site mint's plan**: which faces it re-mints, and the rows it
    /// writes onto each, decided before its door mutates.
    ///
    /// `touched` names the loops the door's halves are in, as the body
    /// holds them now; their faces are what [`Body::plan_site_mint_of`]
    /// reads, each loop resolved as its face is reached.
    ///
    /// # Errors
    ///
    /// [`Body::plan_site_mint_of`]'s, a touched loop that does not
    /// resolve ([`EulerOpError::StaleKey`]) among them.
    pub(crate) fn plan_site_mint(
        &self,
        touched: &[LoopKey],
        faces: impl FnOnce(
            &Self,
            &[(FaceKey, crate::pcurves::SiteFrom<T>)],
        ) -> Result<Vec<SiteFace<T>>, EulerOpError>,
        edge: Option<&EdgeCurve<T>>,
        tol: Tol,
    ) -> Result<Vec<SiteRows<T>>, EulerOpError> {
        let read = touched.iter().map(|&lk| {
            self.get_loop(lk)
                .map(|l| l.face)
                .ok_or(EulerOpError::StaleKey {
                    key: EntityId::Loop(lk),
                })
        });
        self.plan_site_mint_of(read, faces, edge, Some(tol))
    }

    /// **A site mint's plan over the faces whose rows decide it**
    /// (`read`, as the body holds them now, a face named twice read
    /// once), decided before its door mutates.
    ///
    /// Each face is read first, once — one walk of each face on a
    /// chart that mints — and only a face
    /// [`crate::pcurves::site_rows_from`] reads further can be
    /// re-minted; every other face is left as found. So whether the
    /// door reads more than those faces, and whether it can refuse
    /// here, depends on them alone — never on rows held elsewhere in
    /// the body — and when none is read further, `faces` does not run.
    /// `faces` is handed those faces as found and describes the faces
    /// as the door leaves them; [`crate::pcurves::site_rows`] decides
    /// each one. `edge` carries the door's new or described halves, and
    /// is `None` for a door that names existing halves alone. `tol` is
    /// the band the rows are derived at; `None` for a keys-only door,
    /// which derives nothing and refuses
    /// [`crate::pcurves::SiteRowRefusal::KeysOnly`] where a face would
    /// be written ([`crate::pcurves::site_rows_owed`]).
    ///
    /// # Errors
    ///
    /// In this order, per face of `read` as it is reached: what `read`
    /// raises naming it, then the face does not resolve
    /// ([`EulerOpError::StaleKey`]), or its surface does not
    /// ([`EulerOpError::StaleGeometry`]), or a half of it does not
    /// ([`EulerOpError::PcurveMint`] naming the face); then, only when a
    /// face is read further, what `faces` raises; then
    /// [`EulerOpError::PcurveMint`] naming the face.
    pub(crate) fn plan_site_mint_of(
        &self,
        read: impl IntoIterator<Item = Result<FaceKey, EulerOpError>>,
        faces: impl FnOnce(
            &Self,
            &[(FaceKey, crate::pcurves::SiteFrom<T>)],
        ) -> Result<Vec<SiteFace<T>>, EulerOpError>,
        edge: Option<&EdgeCurve<T>>,
        tol: Option<Tol>,
    ) -> Result<Vec<SiteRows<T>>, EulerOpError> {
        let mut minted: Vec<(FaceKey, crate::pcurves::SiteFrom<T>)> = Vec::new();
        let mut seen: Vec<FaceKey> = Vec::new();
        for face in read {
            let face = face?;
            if seen.contains(&face) {
                continue;
            }
            seen.push(face);
            let face_data = self.get_face(face).ok_or(EulerOpError::StaleKey {
                key: EntityId::Face(face),
            })?;
            let surface =
                self.get_surface(face_data.surface)
                    .ok_or(EulerOpError::StaleGeometry {
                        key: GeomRef::Surface(face_data.surface),
                    })?;
            if let Some(from) = crate::pcurves::site_rows_from(self, face_data, surface)
                .map_err(|refusal| EulerOpError::PcurveMint { face, refusal })?
            {
                minted.push((face, from));
            }
        }
        if minted.is_empty() {
            return Ok(Vec::new());
        }
        let described = faces(self, &minted)?;
        let from_of = |face: &SiteFace<T>| {
            minted
                .iter()
                .find(|(f, _)| *f == face.rows_from)
                .map(|(_, from)| from)
        };
        let Some(tol) = tol else {
            for face in &described {
                let Some(from) = from_of(face) else { continue };
                let refused = |refusal| EulerOpError::PcurveMint {
                    face: face.rows_from,
                    refusal,
                };
                if crate::pcurves::site_rows_owed(self, face, from).map_err(refused)? {
                    return Err(refused(crate::pcurves::SiteRowRefusal::KeysOnly));
                }
            }
            return Ok(Vec::new());
        };
        let band = Band::linear(tol).map_err(|e| EulerOpError::Certification {
            error: CertifyError::Band(e),
        })?;
        described
            .iter()
            .map(|face| {
                let Some(from) = from_of(face) else {
                    return Ok(SiteRows::Leave);
                };
                crate::pcurves::site_rows(self, face, from, edge, band).map_err(|refusal| {
                    EulerOpError::PcurveMint {
                        face: face.rows_from,
                        refusal,
                    }
                })
            })
            .collect()
    }

    /// `face` as a surgery leaves it, for [`Body::plan_site_mint`]: its
    /// chart, and its loops outer first — each loop named in `rewired`
    /// replaced by the half-edge sequence given there, `killed` gone,
    /// every other loop kept.
    pub(crate) fn site_face(
        &self,
        face: FaceKey,
        rewired: &[(LoopKey, Vec<SiteHalf>)],
        killed: Option<LoopKey>,
    ) -> Result<SiteFace<T>, EulerOpError> {
        let face_data = self.get_face(face).ok_or(EulerOpError::StaleKey {
            key: EntityId::Face(face),
        })?;
        let surface =
            self.get_surface(face_data.surface)
                .cloned()
                .ok_or(EulerOpError::StaleGeometry {
                    key: GeomRef::Surface(face_data.surface),
                })?;
        let loops = core::iter::once(face_data.outer)
            .chain(face_data.rings.iter().copied())
            .filter(|&lk| Some(lk) != killed)
            .map(|lk| match rewired.iter().find(|(k, _)| *k == lk) {
                Some((_, halves)) => SiteLoop::Rewired(halves.clone()),
                None => SiteLoop::Kept(lk),
            })
            .collect();
        Ok(SiteFace {
            rows_from: face,
            surface,
            moved: false,
            loops,
        })
    }

    /// The half-edges of `he`'s loop in `next` order from `he` itself.
    pub(crate) fn site_cycle_from(
        &self,
        he: HalfEdgeKey,
        r#loop: LoopKey,
    ) -> Result<Vec<HalfEdgeKey>, EulerOpError> {
        self.loop_cycle(he)
            .ok_or(EulerOpError::LoopCycleBroken { r#loop })
    }

    /// A face a door makes out of `from` as the surgery leaves it —
    /// `mef`'s new face, `mfkrh`'s promoted one: one loop, `halves`, on
    /// the chart `surface` names, whose rows `from`'s decide. `moved`
    /// is whether that loop arrives without its rows standing
    /// ([`SiteFace::moved`]): stated in `from`'s chart where the new
    /// face's is another, or missing.
    pub(crate) fn new_site_face(
        &self,
        from: FaceKey,
        surface: &FaceSurface<T>,
        moved: bool,
        halves: Vec<SiteHalf>,
    ) -> Result<SiteFace<T>, EulerOpError> {
        let from_surface = self
            .get_face(from)
            .ok_or(EulerOpError::StaleKey {
                key: EntityId::Face(from),
            })?
            .surface;
        let key_surface = |key: SurfaceKey| {
            self.get_surface(key)
                .cloned()
                .ok_or(EulerOpError::StaleGeometry {
                    key: GeomRef::Surface(key),
                })
        };
        let chart = match surface {
            FaceSurface::Inherit => key_surface(from_surface)?,
            FaceSurface::Shared { key, .. } => key_surface(*key)?,
            FaceSurface::New { surface, .. } => surface.clone(),
        };
        Ok(SiteFace {
            rows_from: from,
            surface: chart,
            moved,
            loops: vec![SiteLoop::Rewired(halves)],
        })
    }

    /// The half-edges of `r#loop` in `next` order from its `first`, as
    /// the surgery finds them (empty for an empty loop).
    pub(crate) fn site_cycle(&self, r#loop: LoopKey) -> Result<Vec<HalfEdgeKey>, EulerOpError> {
        match crate::pcurves::loop_rows(self, r#loop) {
            crate::pcurves::LoopRows::Cycle(cycle) => Ok(cycle),
            crate::pcurves::LoopRows::NoCycle => Ok(Vec::new()),
            crate::pcurves::LoopRows::Corrupt => Err(EulerOpError::LoopCycleBroken { r#loop }),
        }
    }

    /// Mints `mef`'s new loop and face (in that order — part of `mef`'s
    /// documented minting order) and joins the new face to the old
    /// face's shell. The loop's boundary anchor is provisional; the
    /// caller re-anchors it after the splice. `surface` and `sense`
    /// are the plan phase's ([`Body::resolve_face_surface`]).
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
                sense,
                surface,
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
    /// out a [`Live`] performs the lookup, so a key nothing has
    /// resolved cannot arrive here. **Which doors those are is not
    /// restated here**: the [`live`](crate::live) module owns the list
    /// and a source-level row there enumerates it, so a fifth door reds
    /// against that row — while a copy of the names in this file would
    /// go stale against it silently, which is the direction a
    /// cross-reference fails in. What the token does and does not claim
    /// — in particular that it is a statement about the moment it was
    /// made, and that half-edge removal is therefore the last thing a
    /// mutation phase may do — is those same module docs.
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
    /// be tier-1 valid.
    ///
    /// **The delta check is unconditional; the tier-1 sweep is the
    /// door's.** The delta is O(1) and is this operator's own declared
    /// contract, so it runs at every call. The sweep re-derives the
    /// whole body, and inside an open surgery scope
    /// ([`crate::surgery`]) it is the composing door that runs it, once,
    /// over the state the caller will see. An operator a consumer calls
    /// directly is itself a door and sweeps here.
    ///
    /// On tier-1-valid input a failure
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
        self.assert_tier1_postcondition(op);
    }
}

/// A run of half-edges a kill re-parents
/// ([`Body::require_kill_anchors`]): the members its cycle walk took
/// from the loop `from`, which join `into`.
#[derive(Clone, Copy)]
pub(crate) struct KillRun<'a> {
    /// The run, as the walk returned it.
    pub(crate) members: &'a [Live],
    /// The loop the walk was of.
    pub(crate) from: LoopKey,
    /// How much of `from` the run is.
    pub(crate) extent: RunExtent,
    /// The loop the run joins.
    pub(crate) into: KillInto,
}

/// The records a kill's removal proofs
/// ([`Body::require_loop_unlisted`], [`Body::require_face_unnamed`],
/// [`Body::require_shell_unnamed`], [`Body::require_solid_unnamed`],
/// [`Body::require_killed_halves_unnamed`]) pass over, since the kill leaves
/// none of them naming the record it removes: those it removes beside
/// it, those it keeps and rewrites the proof's field of, and the
/// `next` links it writes. Each proof reads one field per arena, so a
/// clearing is built for the fields that proof reads.
#[derive(Clone, Copy, Default)]
pub(crate) struct Clearing<'a> {
    /// The records the kill removes.
    pub(crate) removed: Records<'a>,
    /// The records the kill keeps and rewrites the field of: `kef`'s
    /// shell and `kfmrh`'s same-shell form's, which drop the dying
    /// face, `mekr`'s face, which drops
    /// the ring, `kfmrh`'s demoted ring, which re-homes onto `f1`, and
    /// in its fusion form the faces that move to `f1`'s shell and the
    /// solid that drops the dying shell, and a kill's re-anchored loops
    /// and endpoints.
    pub(crate) edited: Records<'a>,
    /// The links the kill writes, in write order: each `(a, b)` sets
    /// `a.next = b` and `b.prev = a` ([`Body::link_half_edges`]).
    pub(crate) links: &'a [(Live, Live)],
}

/// One list per arena: a [`Clearing`]'s half.
#[derive(Clone, Copy, Default)]
pub(crate) struct Records<'a> {
    pub(crate) loops: &'a [LoopKey],
    pub(crate) faces: &'a [FaceKey],
    pub(crate) shells: &'a [ShellKey],
    pub(crate) solids: &'a [SolidKey],
    pub(crate) vertices: &'a [VertexKey],
    pub(crate) edges: &'a [EdgeKey],
}

impl Clearing<'_> {
    fn clears_loop(&self, l: LoopKey) -> bool {
        self.removed.loops.contains(&l) || self.edited.loops.contains(&l)
    }

    fn clears_vertex(&self, v: VertexKey) -> bool {
        self.removed.vertices.contains(&v) || self.edited.vertices.contains(&v)
    }

    fn clears_edge(&self, e: EdgeKey) -> bool {
        self.removed.edges.contains(&e) || self.edited.edges.contains(&e)
    }

    /// `he`'s `next` and `prev` once the kill has written its links.
    fn links_of(&self, he: HalfEdgeKey, data: &HalfEdge) -> [HalfEdgeKey; 2] {
        let next = self.links.iter().rev().find(|(a, _)| a.key() == he);
        let prev = self.links.iter().rev().find(|(_, b)| b.key() == he);
        [
            next.map_or(data.next, |(_, b)| b.key()),
            prev.map_or(data.prev, |(a, _)| a.key()),
        ]
    }

    fn clears_face(&self, f: FaceKey) -> bool {
        self.removed.faces.contains(&f) || self.edited.faces.contains(&f)
    }

    fn clears_shell(&self, s: ShellKey) -> bool {
        self.removed.shells.contains(&s) || self.edited.shells.contains(&s)
    }

    fn clears_solid(&self, s: SolidKey) -> bool {
        self.removed.solids.contains(&s) || self.edited.solids.contains(&s)
    }
}

/// How much of the loop it was walked from a moved run is
/// ([`Body::require_run_of`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RunExtent {
    /// Part of the loop, which keeps the rest.
    Part,
    /// All of the loop but the halves the plan kills: the loop dies.
    Whole,
}

/// The loop a [`KillRun`] joins.
#[derive(Clone, Copy)]
pub(crate) enum KillInto {
    /// A loop the kill keeps.
    Kept(LoopKey),
    /// The loop the kill mints, at this boundary, which
    /// [`Body::require_kill_anchors`] proves as it proves a kept loop's.
    Minted(LoopBoundary),
}

/// A vertex anchor a kill writes ([`Body::require_kill_anchors`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum KillAnchor {
    /// A half-edge read one `next` step from a killed half.
    Step(HalfEdgeKey),
    /// The first member of a fan the kill merges onto the vertex, which
    /// the kill's own orbit walk proves.
    Merged(HalfEdgeKey),
    /// No half-edge: the kill leaves the vertex lone, held by an `Empty`
    /// loop it writes.
    Lone,
}

impl KillAnchor {
    /// The anchor a kill reads one `next` step away, `Lone` where that
    /// step lands on a killed half.
    pub(crate) fn step(anchor: Option<HalfEdgeKey>) -> Self {
        anchor.map_or(Self::Lone, Self::Step)
    }

    /// The `emanating` the write stores.
    pub(crate) fn key(self) -> Option<HalfEdgeKey> {
        match self {
            Self::Step(he) | Self::Merged(he) => Some(he),
            Self::Lone => None,
        }
    }
}

/// The loop both half-edges claim, `None` where they claim two: the one
/// comparison behind [`EulerOpError::NotSameLoop`] (a caller's pair that
/// [`MefSite::Chords`] or [`Body::kemr`] needs in one loop) and
/// [`Body::kev`]'s adjacent-arm [`EulerOpError::LoopCycleBroken`] (a
/// torn `next` that reads two loops' halves as adjacent).
pub(crate) fn shared_loop(a: &HalfEdge, b: &HalfEdge) -> Option<LoopKey> {
    (a.parent_loop == b.parent_loop).then_some(a.parent_loop)
}

/// A half-edge, its edge and its mate, as [`Body::proven_mate`] proves
/// them.
pub(crate) struct ProvenMate<'a> {
    /// The half-edge the hop starts from.
    pub(crate) he_data: HalfEdge,
    /// Its edge.
    pub(crate) edge: EdgeKey,
    /// The edge's record, which claims both halves.
    pub(crate) edge_data: &'a Edge,
    /// The other half the edge claims.
    pub(crate) mate: HalfEdgeKey,
    /// The mate's record, which names `edge`.
    pub(crate) mate_data: HalfEdge,
}

/// Proves that `he1` and `he2` are the two halves of `edge`, the edge
/// `he1` names, refusing [`EulerOpError::NotSameEdge`] otherwise: they
/// are distinct, `edge` claims `he1` with `he2` as its mate
/// ([`Edge::claim`]), and `he2` names `edge` (`he2_edge`). The one
/// decision a kill takes on its pair, before any other check that reads
/// the mate: a torn bijection can claim one half in both slots, or hand
/// the kill a mate whose own edge is another, which the kill would leave
/// naming a dead edge. Reads the edge's slots and one field, O(1);
/// [`Body::require_edge_unnamed`] proves the rest of the arena.
pub(crate) fn require_halves(
    edge: EdgeKey,
    edge_data: &Edge,
    he1: HalfEdgeKey,
    (he2, he2_edge): (HalfEdgeKey, EdgeKey),
) -> Result<(), EulerOpError> {
    if he1 == he2 || he2_edge != edge || edge_data.claim(he1).map(|c| c.mate) != Some(he2) {
        return Err(EulerOpError::NotSameEdge { he1, he2 });
    }
    Ok(())
}

/// The **plane × NURBS attach door** (M7-8).
///
/// A described NURBS operand in an `Intersection` certifies only
/// through `geom_brep`'s injected lane, whose derivation needs a
/// CERTIFYING scalar (`geom_brep::plane_nurbs_limbs`'s own bound is
/// `Decide + Bounds + CertifiedEnclosure`, and since D1, 2026-08-19, it
/// is that last term rather than `Bounds` that a dual fails, so a dual
/// cannot write this door's call at all rather than being refused
/// inside it). Raising the whole Euler surface to that bound would push it
/// through hundreds of `T: Decide` signatures for a capability three
/// of the four sealed scalars have unconditionally, so the lane is a
/// SEPARATE DOOR onto the same shared machinery: identical
/// preconditions, identical adjacency rules, identical mutation. The
/// default door keeps refusing the class exactly as before — there is
/// no door that accepts it uncertified.
impl<T: Decide + geom_core::CertifiedBounds> Body<T> {
    /// [`Body::set_edge_curve`] with the plane × NURBS lane wired in.
    ///
    /// **A scalar without certification rights cannot write this
    /// call**, and that is the door's guarantee rather than a side
    /// effect: nothing runs, nothing refuses, the call cannot be
    /// formed.
    ///
    /// The code is **`E0599`, not `E0277`**, and the difference is
    /// where the bound sits: this is an INHERENT METHOD on an `impl`
    /// block whose bound `Dual` fails, so the method is not in scope
    /// at all and the compiler says *method exists … but its trait
    /// bounds were not satisfied: `Dual<f64>: CertifiedEnclosure`*
    /// rather than reporting an unsatisfied bound on a call it
    /// resolved. A free function with the same bound gives `E0277`
    /// (`geom_brep::plane_nurbs_limbs`' own row does).
    ///
    /// ```compile_fail,E0599
    /// use geom_core::{Dual64, Tol};
    /// use topo::{Body, EdgeCurveSpec, entity::EdgeKey};
    /// fn lane_door(b: &mut Body<Dual64>, e: EdgeKey, c: EdgeCurveSpec<Dual64>, tol: Tol) {
    ///     let _ = b.set_edge_curve_nurbs_lane(e, c, tol);
    /// }
    /// ```
    ///
    /// **What that annotation is worth, said out loud** (`S216`): on
    /// stable, rustdoc does NOT compare the emitted code to the one
    /// written here — the row passes green whichever code is named, so
    /// the annotation documents the expectation and checks nothing.
    /// The live check is the twin below: it differs in exactly one
    /// identifier and every path either row names resolves, so the
    /// failing row can only be failing on the bound. Read the pair,
    /// never the annotation alone.
    ///
    /// The DEFAULT door is open to it, which is the capability this
    /// separation exists to keep:
    ///
    /// ```
    /// use geom_core::{Dual64, Tol};
    /// use topo::{Body, EdgeCurveSpec, entity::EdgeKey};
    /// fn default_door(b: &mut Body<Dual64>, e: EdgeKey, c: EdgeCurveSpec<Dual64>, tol: Tol) {
    ///     let _ = b.set_edge_curve(e, c, tol);
    /// }
    /// ```
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

/// A loop's half-edges after a splice that inserts new halves, in walk
/// order from the loop's `first` — which the splice keeps. `cycle` is
/// the loop from its `first` before the splice; each `(x, halves)` puts
/// `halves` immediately before `x`. Splicing before `first` itself lands
/// the halves between `prev(first)` and `first`, which a walk from
/// `first` reaches last.
pub(crate) fn spliced_before(
    cycle: &[HalfEdgeKey],
    inserts: &[(HalfEdgeKey, &[SiteHalf])],
) -> Vec<SiteHalf> {
    let before = |he: HalfEdgeKey| {
        inserts
            .iter()
            .filter(move |(x, _)| *x == he)
            .flat_map(|(_, halves)| halves.iter().copied())
    };
    let mut out: Vec<SiteHalf> = Vec::with_capacity(cycle.len() + 2);
    for (i, &he) in cycle.iter().enumerate() {
        if i > 0 {
            out.extend(before(he));
        }
        out.push(SiteHalf::Existing(he));
    }
    if let Some(&first) = cycle.first() {
        out.extend(before(first));
    }
    out
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
    use crate::fixtures::{NgonPillow, assert_err_deep_unchanged, deep_snapshot, pillow, prov};
    use crate::validate::validate;

    fn p(x: f64) -> Point3<f64> {
        Point3::new(x, 0.0, 0.0)
    }

    // ------------------------------------------------------------------
    // mvfs
    // ------------------------------------------------------------------

    #[test]
    fn mvfs_creates_the_skeletal_body() {
        let mut body = Body::<f64>::new();
        let c = body.mvfs(Point3::new(1.0, 2.0, 3.0), true).unwrap();
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
        let seed = body.mvfs(p(0.0), true).unwrap();
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
        // The construction's clockwise orbit at v (hand-derived from the
        // strut splice, pinned here so the split test below stands on
        // verified ground): pa, pb, pc, pd.
        assert_eq!(
            body.vertex_orbit(a.he_plus),
            Some(vec![a.he_plus, b.he_plus, c.he_plus, d.he_plus])
        );
        (body, seed, [a, b, c, d])
    }

    /// The carrier of `edge`, printed — the byte-for-byte comparison
    /// this module makes about a certificate that must not move.
    fn carrier_bits(body: &Body<f64>, edge: EdgeKey) -> String {
        let curve = body.get_edge(edge).unwrap().curve;
        format!("{:?}", body.get_curve_geom(curve).unwrap())
    }

    #[test]
    fn a_fan_mev_that_would_move_a_run_off_its_carriers_refuses_untouched() {
        // The red-first row. On the merge base this returned `Ok` and
        // left every moved spoke describing a locus that no longer ran
        // to it — the state `split_edge` and `set_edge_curve` then
        // refused on, and the one `seqgen`'s split filter routes
        // around. The refusal names the first re-based edge in run
        // order, and the body is untouched to the byte.
        let (mut body, _seed, [_a, b, _c, d]) = four_spoke_star();
        let before = deep_snapshot(&body);
        let err = body
            .mev_line(
                MevSite::Fan {
                    he1: b.he_plus,
                    he2: d.he_plus,
                },
                p(5.0),
                Tol::witness(),
            )
            .unwrap_err();
        assert!(
            matches!(
                err,
                EulerOpError::RebasedCarrier {
                    edge,
                    error: geom_brep::CertifyError::ResidualExceeded {
                        check: geom_brep::CertCheck::EndpointStart,
                        sample: 0,
                    },
                } if edge == b.edge
            ),
            "{err:?}"
        );
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn a_coincident_fan_split_carries_every_certificate_byte_for_byte() {
        // The control. `mev_null`'s new vertex takes the old vertex's
        // point as a bitwise copy, so no re-based edge's endpoint
        // moves: the gate has nothing to refuse and nothing is
        // re-minted — each moved spoke carries the certificate it
        // already had.
        let (mut body, _seed, [_a, b, c, d]) = four_spoke_star();
        let before = [carrier_bits(&body, b.edge), carrier_bits(&body, c.edge)];
        body.mev_null(
            MevSite::Fan {
                he1: b.he_plus,
                he2: d.he_plus,
            },
            crate::NewVertexSide::Above,
        )
        .unwrap();
        let after = [carrier_bits(&body, b.edge), carrier_bits(&body, c.edge)];
        assert_eq!(
            before, after,
            "the moved run's certificates are the ones it had"
        );
    }

    // ------------------------------------------------------------------
    // The re-basing gate, per carrier kind and per arm
    //
    // `certify_rebased_run`'s `Ok` arm over a NON-EMPTY run is what the
    // operator's whole claim rests on, and until these rows it was
    // pinned by nothing: the only control was a `mev_null` surgery,
    // which never calls the gate at all. Each row below drives the gate
    // over a one-member run at the member's OWN start point (which must
    // carry, with the re-certification actually executed) and at a
    // moved point (which must refuse, naming the edge and the endpoint
    // check).
    // ------------------------------------------------------------------

    /// A digon pillow whose second face carries a plane: the smallest
    /// body that can hold a `Chart` or an `Intersection` description.
    fn described_pillow(tol: Tol) -> (Body<f64>, MevCreated, crate::MefCreated) {
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(1.0, 0.0, 0.0),
                tol,
            )
            .unwrap();
        let plane = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: geom_core::Vec3::unit_z(),
            u_ref: geom_core::Vec3::unit_x(),
        };
        let split = body
            .mef(
                MefSite::Chords {
                    he1: seg.he_plus,
                    he2: seg.he_minus,
                },
                geom_brep::EdgeCurveSpec::line_between(
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(1.0, 0.0, 0.0),
                ),
                FaceSurface::New {
                    surface: plane,
                    sense: true,
                },
                tol,
            )
            .unwrap();
        (body, seg, split)
    }

    /// Both gate arms over the one-member run `[he]`: its own start
    /// point carries, a point one unit off refuses on the start
    /// endpoint.
    fn assert_both_gate_arms(body: &Body<f64>, he: HalfEdgeKey, kind: &str) {
        let tol = Tol::witness();
        let v = body.get_half_edge(he).unwrap().start;
        let here = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        assert_eq!(
            body.certify_rebased_run(&[he], here, tol),
            Ok(()),
            "{kind}: a run that does not move must carry"
        );
        let moved = Point3::new(here.x, here.y, here.z + 1.0);
        let err = body.certify_rebased_run(&[he], moved, tol).unwrap_err();
        assert!(
            matches!(
                err,
                EulerOpError::RebasedCarrier {
                    error: geom_brep::CertifyError::ResidualExceeded {
                        check: geom_brep::CertCheck::EndpointStart,
                        ..
                    },
                    ..
                }
            ),
            "{kind}: a moved run must refuse, got {err:?}"
        );
    }

    #[test]
    fn the_gate_carries_and_refuses_a_line_under_a_scaffold_description() {
        let (body, seg, _split) = described_pillow(Tol::witness());
        assert_both_gate_arms(&body, seg.he_plus, "line + scaffold");
    }

    #[test]
    fn the_gate_carries_and_refuses_a_line_under_an_intersection_description() {
        let tol = Tol::witness();
        let (mut body, _seg, split) = described_pillow(tol);
        let s_plus = body.get_face(split.face).unwrap().surface;
        let seed_face = body.face_of_half_edge(split.he_plus).unwrap();
        let s_seed = body.get_face(seed_face).unwrap().surface;
        *body.surfaces.get_mut(s_seed).unwrap() = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: geom_core::Vec3::unit_z(),
            u_ref: geom_core::Vec3::unit_x(),
        };
        *body.surfaces.get_mut(s_plus).unwrap() = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: geom_core::Vec3::unit_y(),
            u_ref: geom_core::Vec3::unit_x(),
        };
        let mut spec = geom_brep::EdgeCurveSpec::line_between(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
            s1: s_seed,
            s2: s_plus,
            witness: Point3::new(0.5, 0.0, 0.0),
        };
        body.set_edge_curve(split.edge, spec, tol).unwrap();
        let hp = body.get_edge(split.edge).unwrap().he_plus;
        assert_both_gate_arms(&body, hp, "line + intersection");
    }

    #[test]
    fn the_gate_carries_and_refuses_a_line_under_a_chart_description() {
        let tol = Tol::witness();
        let (mut body, _seg, split) = described_pillow(tol);
        let s_plus = body.get_face(split.face).unwrap().surface;
        let mut spec = geom_brep::EdgeCurveSpec::line_between(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        );
        spec.description = geom_brep::EdgeDescriptionSpec::chart(s_plus);
        body.set_edge_curve(split.edge, spec, tol).unwrap();
        let hp = body.get_edge(split.edge).unwrap().he_plus;
        assert_both_gate_arms(&body, hp, "line + chart");
    }

    #[test]
    fn the_gate_carries_and_refuses_a_circle_under_a_scaffold_description() {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        let circ = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: seed.r#loop,
                },
                tol,
            )
            .unwrap();
        assert_both_gate_arms(&body, circ.he_plus, "circle + scaffold");
    }

    /// A segment `v0 → v1` with a null strut minted at `v1` by
    /// `mev_null`: `v1`'s fan is `[nul.he_plus, seg.he_minus]`, so
    /// `Fan { nul.he_plus, seg.he_minus }` is a run site whose run is
    /// exactly one half of the null edge.
    fn null_strut_on_a_segment() -> (Body<f64>, MevCreated, MevCreated) {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        let seg = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(1.0, 0.0, 0.0),
                tol,
            )
            .unwrap();
        let nul = body
            .mev_null(
                MevSite::Fan {
                    he1: seg.he_minus,
                    he2: seg.he_minus,
                },
                crate::NewVertexSide::Above,
            )
            .unwrap();
        (body, seg, nul)
    }

    /// The bits of the points at `edge`'s two ends, `he_plus` start
    /// first — equal exactly when the edge is still one point. Points
    /// compare by `to_bits` here and whole curve entries by `Debug`
    /// text in [`carrier_bits`]: a point has three scalars to read
    /// bits from, a curve entry has no bits accessor, and `f64`'s
    /// `Debug` is round-trip exact, so both are bit-faithful short of a
    /// NaN payload.
    fn end_point_bits(body: &Body<f64>, edge: EdgeKey) -> [[u64; 3]; 2] {
        let e = body.get_edge(edge).unwrap();
        let bits = |he: HalfEdgeKey| {
            let v = body.get_half_edge(he).unwrap().start;
            let q = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            [q.x.to_bits(), q.y.to_bits(), q.z.to_bits()]
        };
        [bits(e.he_plus), bits(e.he_minus)]
    }

    #[test]
    fn the_gate_refuses_a_run_that_moves_one_end_of_a_null_edge_wherever_it_moves() {
        // The run holds one half of the null edge, so the move splits
        // its two ends between the new vertex and the old one. The gate
        // refuses without comparing points (why: `certify_rebased_run`'s
        // docs), at a far point and at the null edge's own point alike.
        let tol = Tol::witness();
        let (body, _seg, nul) = null_strut_on_a_segment();
        for p_new in [Point3::new(99.0, 99.0, 99.0), Point3::new(1.0, 0.0, 0.0)] {
            assert_eq!(
                body.certify_rebased_run(&[nul.he_plus], p_new, tol),
                Err(EulerOpError::RebasedNullEdge { edge: nul.edge }),
                "one half of a null edge in the run at {p_new:?}"
            );
        }
    }

    #[test]
    fn a_fan_mev_refuses_to_move_one_end_of_a_null_edge_and_leaves_the_body_untouched() {
        // The public door, where the hole was: a certified `mev` at a
        // run site holding one half of a null edge. At a far point
        // (a chord spec) and at the old vertex's own point (the closed
        // spec, the only one that certifies a zero-span edge) alike,
        // the refusal is typed and the body is untouched, the null edge
        // still one point.
        let tol = Tol::witness();
        let (mut body, seg, nul) = null_strut_on_a_segment();
        let here = Point3::new(1.0, 0.0, 0.0);
        let far = Point3::new(99.0, 99.0, 99.0);
        let ends = end_point_bits(&body, nul.edge);
        assert_eq!(ends[0], ends[1], "mev_null minted one point");
        let before = deep_snapshot(&body);
        for (point, spec) in [
            (far, geom_brep::EdgeCurveSpec::line_between(here, far)),
            (here, geom_brep::EdgeCurveSpec::self_loop_circle_at(here)),
        ] {
            let site = MevSite::Fan {
                he1: nul.he_plus,
                he2: seg.he_minus,
            };
            assert_eq!(
                body.mev(site, point, spec, tol).map(|_| ()),
                Err(EulerOpError::RebasedNullEdge { edge: nul.edge }),
                "mev to {point:?}"
            );
            assert_eq!(deep_snapshot(&body), before, "mev to {point:?}");
        }
    }

    /// A null edge made a self-loop at one vertex `v` at `(1, 2, 3)`,
    /// with a real strut at `v` beside it: `mev_null` at a lone vertex,
    /// a closed circle edge between its two coincident ends, a kill of
    /// that circle edge merging the new vertex's fan (the null edge's
    /// other half) onto `v`, then the strut. Tier-1 green. Returns the
    /// body, the null edge and the strut's half at `v`.
    ///
    /// **No door builds this.** The kill moves ONE end of the null
    /// edge, which both kill doors refuse (`RebasedNullEdge`, the
    /// re-basing gate's null arm) — and a null edge only ever joins its
    /// minting vertex to a fresh one, so there is no other way to close
    /// one onto a single vertex. The fixture takes the kill's ungated
    /// execution, `Body::kev_ungated`, because the rows it serves pin
    /// the both-halves arm, which only this state exercises.
    fn null_self_loop_beside_a_strut() -> (Body<f64>, EdgeKey, HalfEdgeKey) {
        let tol = Tol::witness();
        let here = Point3::new(1.0, 2.0, 3.0);
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(here, true).unwrap();
        let nul = body
            .mev_null(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                crate::NewVertexSide::Above,
            )
            .unwrap();
        let circle = body
            .mef(
                MefSite::Chords {
                    he1: nul.he_minus,
                    he2: nul.he_plus,
                },
                geom_brep::EdgeCurveSpec::self_loop_circle_at(here),
                FaceSurface::Inherit,
                tol,
            )
            .unwrap();
        // `kev(he)` kills `end(he)`: take the half that starts at the
        // seed vertex, so the vertex `mev_null` minted dies.
        let kill = if body.get_half_edge(circle.he_plus).unwrap().start == seed.vertex {
            circle.he_plus
        } else {
            circle.he_minus
        };
        body.kev_ungated(kill).unwrap();
        let e = body.get_edge(nul.edge).unwrap();
        let (hp, hm) = (e.he_plus, e.he_minus);
        assert_eq!(
            body.get_half_edge(hp).unwrap().start,
            body.get_half_edge(hm).unwrap().start,
            "the null edge is a self-loop"
        );
        let strut = body
            .mev_line(
                MevSite::Fan { he1: hp, he2: hp },
                Point3::new(5.0, 2.0, 3.0),
                tol,
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        (body, nul.edge, strut.he_plus)
    }

    #[test]
    fn a_fan_mev_moving_both_halves_of_a_null_edge_keeps_it_one_vertex() {
        // Both halves in the run: both ends move onto the one new
        // vertex, so the null edge stays one vertex and one point by
        // structure, with nothing compared. The gate carries it at the
        // old point and at a far one, and the certified door runs.
        let tol = Tol::witness();
        let (mut body, nul_edge, strut) = null_self_loop_beside_a_strut();
        let here = Point3::new(1.0, 2.0, 3.0);
        let far = Point3::new(40.0, 2.0, 3.0);
        let e = body.get_edge(nul_edge).unwrap();
        let (hp, hm) = (e.he_plus, e.he_minus);
        // From the strut the orbit is [strut, x, y]; the run
        // `[x .. strut)` is exactly the null edge's two halves.
        let orbit = body.vertex_orbit(strut).unwrap();
        assert_eq!(orbit.len(), 3);
        let run = [orbit[1], orbit[2]];
        assert!(run.contains(&hp) && run.contains(&hm), "{orbit:?}");
        for p_new in [here, far] {
            assert_eq!(
                body.certify_rebased_run(&run, p_new, tol),
                Ok(()),
                "both halves in the run at {p_new:?}"
            );
        }
        let created = body
            .mev(
                MevSite::Fan {
                    he1: orbit[1],
                    he2: strut,
                },
                far,
                geom_brep::EdgeCurveSpec::line_between(here, far),
                tol,
            )
            .unwrap();
        for he in [hp, hm] {
            assert_eq!(body.get_half_edge(he).unwrap().start, created.vertex);
        }
        let ends = end_point_bits(&body, nul_edge);
        assert_eq!(ends[0], ends[1], "the null edge is still one point");
        assert_eq!(validate(&body), Ok(()));
    }

    #[test]
    fn a_fan_mev_refuses_in_run_order_between_a_null_edge_and_a_moved_carrier() {
        // `mev`'s precondition list is per edge of the moved run IN RUN
        // ORDER: a run whose first member is one half of a null edge
        // refuses `RebasedNullEdge`, and a run whose first member is a
        // chord the move breaks refuses `RebasedCarrier`, though each
        // run holds both. `v1` carries the segment's far end, the null
        // strut and a second chord strut.
        let tol = Tol::witness();
        let (mut body, _seg, nul) = null_strut_on_a_segment();
        body.mev_line(
            MevSite::Fan {
                he1: nul.he_plus,
                he2: nul.he_plus,
            },
            Point3::new(1.0, 5.0, 0.0),
            tol,
        )
        .unwrap();
        let o = body.vertex_orbit(nul.he_plus).unwrap();
        assert_eq!(o.len(), 3);
        let chord_after_null = body.get_half_edge(o[2]).unwrap().edge;
        let far = Point3::new(7.0, 7.0, 7.0);
        let spec = geom_brep::EdgeCurveSpec::line_between(Point3::new(1.0, 0.0, 0.0), far);
        let before = deep_snapshot(&body);
        // Run [o0, o1]: the null half first.
        assert_eq!(
            body.mev(
                MevSite::Fan {
                    he1: o[0],
                    he2: o[2],
                },
                far,
                spec.clone(),
                tol,
            )
            .map(|_| ()),
            Err(EulerOpError::RebasedNullEdge { edge: nul.edge })
        );
        assert_eq!(deep_snapshot(&body), before);
        // Run [o2, o0]: a chord first, the null half second.
        let err = body
            .mev(
                MevSite::Fan {
                    he1: o[2],
                    he2: o[1],
                },
                far,
                spec,
                tol,
            )
            .map(|_| ())
            .unwrap_err();
        assert!(
            matches!(err, EulerOpError::RebasedCarrier { edge, .. } if edge == chord_after_null),
            "{err:?}"
        );
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn the_gate_refuses_a_moved_edge_whose_curve_key_dangles_with_the_body_untouched() {
        // A tier-1-corrupt body: the first moved spoke's curve entry is
        // gone. The gate names the dangling key rather than skipping the
        // edge, and the rest of the run is never asked about it.
        let (mut body, _seed, [_a, b, _c, d]) = four_spoke_star();
        body.curves.remove(b.curve).unwrap();
        assert!(validate(&body).is_err(), "the plant is tier-1-corrupt");
        let before = deep_snapshot(&body);
        assert_eq!(
            body.mev_line(
                MevSite::Fan {
                    he1: b.he_plus,
                    he2: d.he_plus,
                },
                p(5.0),
                Tol::witness(),
            )
            .map(|_| ()),
            Err(EulerOpError::StaleGeometry {
                key: GeomRef::Curve(b.curve),
            })
        );
        assert_eq!(deep_snapshot(&body), before);
    }

    #[test]
    fn mev_null_splits_a_fan_across_null_scaffolding_and_keeps_it_one_point() {
        // The door a no-move fan split takes: `mev_null` re-bases the
        // same run the certified door refuses, and the null edge's two
        // ends stay one point, bit for bit, because the new vertex's
        // point is a copy.
        let (mut body, seg, nul) = null_strut_on_a_segment();
        let before = end_point_bits(&body, nul.edge);
        let created = body
            .mev_null(
                MevSite::Fan {
                    he1: nul.he_plus,
                    he2: seg.he_minus,
                },
                crate::NewVertexSide::Above,
            )
            .unwrap();
        assert_eq!(
            body.get_half_edge(nul.he_plus).unwrap().start,
            created.vertex,
            "the run moved onto the new vertex"
        );
        assert_eq!(end_point_bits(&body, nul.edge), before);
        assert_eq!(validate(&body), Ok(()));
    }

    #[test]
    fn kevs_fan_merge_refuses_to_move_one_end_of_a_null_edge_and_leaves_the_body_untouched() {
        // Killing the segment's far end `v1` would merge its fan, which
        // holds one half of the null edge, onto `v0`, so the null edge
        // would span `(0,0,0)` and `(1,0,0)`. Both kill doors refuse it
        // through the re-basing gate's null arm, body untouched and the
        // null edge still one point: the keys-only kill structurally,
        // the describing kill with a band and nothing listed.
        let tol = Tol::witness();
        let (mut body, seg, nul) = null_strut_on_a_segment();
        let ends = end_point_bits(&body, nul.edge);
        assert_eq!(ends[0], ends[1]);
        let before = deep_snapshot(&body);
        assert_eq!(
            body.kev(seg.he_plus).map(|_| ()),
            Err(EulerOpError::RebasedNullEdge { edge: nul.edge })
        );
        assert_eq!(deep_snapshot(&body), before);
        assert_eq!(
            body.kev_describing(seg.he_plus, &[], tol).map(|_| ()),
            Err(EulerOpError::RebasedNullEdge { edge: nul.edge })
        );
        assert_eq!(deep_snapshot(&body), before);
        assert_eq!(end_point_bits(&body, nul.edge), ends);
    }

    #[test]
    fn kevs_fan_merge_moving_both_halves_of_a_null_edge_keeps_it_one_vertex() {
        // The both-halves arm on the kill side. `w` carries a null
        // self-loop and a strut from `v`; killing the strut toward `w`
        // merges both halves of the null edge onto `v`, so it stays one
        // vertex and one point by structure. The keys-only kill carries
        // it with nothing compared, and so does the describing kill.
        let (body, nul_edge, strut) = null_self_loop_beside_a_strut();
        let tol = Tol::witness();
        let toward_w = body.mate(strut).unwrap();
        let v = body.get_half_edge(toward_w).unwrap().start;
        for describing in [false, true] {
            let mut body = body.clone();
            let killed = if describing {
                body.kev_describing(toward_w, &[], tol).unwrap()
            } else {
                body.kev(toward_w).unwrap()
            };
            assert_ne!(killed.killed_vertex, v);
            let e = body.get_edge(nul_edge).unwrap();
            for he in [e.he_plus, e.he_minus] {
                assert_eq!(body.get_half_edge(he).unwrap().start, v);
            }
            let ends = end_point_bits(&body, nul_edge);
            assert_eq!(ends[0], ends[1], "the null edge is still one point");
            assert_eq!(validate(&body), Ok(()));
        }
    }

    /// [`described_pillow`] with its seed face on a described NURBS
    /// patch in the `z = 0` plane and its second face on the plane
    /// `y = 0`, so the chord edge `(0,0,0) → (1,0,0)` is re-described as
    /// a plane × NURBS `Intersection` through the lane door (M7-8).
    /// Returns the body and that edge.
    fn m7_8_pillow(tol: Tol) -> (Body<f64>, EdgeKey) {
        use geom_core::spline::KnotVector;
        let (mut body, _seg, split) = described_pillow(tol);
        let s_plus = body.get_face(split.face).unwrap().surface;
        let seed_face = body.face_of_half_edge(split.he_plus).unwrap();
        let s_seed = body.get_face(seed_face).unwrap().surface;
        let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let ticks = [-1.0, 0.5, 2.0];
        let control: Vec<_> = ticks
            .iter()
            .flat_map(|&x| ticks.iter().map(move |&y| Point3::new(x, y, 0.0)))
            .collect();
        let weights = vec![1.0; control.len()];
        let patch = geom::NurbsSurface::new(k.clone(), k, control, weights).unwrap();
        assert!(!patch.is_placeholder());
        *body.surfaces.get_mut(s_seed).unwrap() = geom::Surface::Nurbs(std::sync::Arc::new(patch));
        *body.surfaces.get_mut(s_plus).unwrap() = geom::Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: geom_core::Vec3::unit_y(),
            u_ref: geom_core::Vec3::unit_x(),
        };
        // The lane's certificate is a hull statement about a control
        // net, so the declared carrier is the chord as a degree-1 spline.
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let chord = geom::NurbsCurve3::new(
            kv,
            vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
            vec![1.0, 1.0],
        )
        .unwrap();
        let spec = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::Intersection {
                s1: s_seed,
                s2: s_plus,
                witness: Point3::new(0.5, 0.0, 0.0),
            },
            carrier: geom::Curve3::Nurbs(std::sync::Arc::new(chord)),
            param_start: 0.0,
            param_end: 1.0,
        };
        body.set_edge_curve_nurbs_lane(split.edge, spec, tol)
            .unwrap();
        (body, split.edge)
    }

    /// The M7-8 pillow's fan site at the old vertex whose run is the
    /// plane × NURBS edge, with that vertex's point.
    fn m7_8_site(body: &Body<f64>, edge: EdgeKey) -> (MevSite, HalfEdgeKey, Point3<f64>) {
        let hp = body.get_edge(edge).unwrap().he_plus;
        let v = body.get_half_edge(hp).unwrap().start;
        let here = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let orbit = body.vertex_orbit(hp).unwrap();
        assert_eq!(orbit.len(), 2, "the pillow's vertex has valence two");
        let site = MevSite::Fan {
            he1: hp,
            he2: orbit[1],
        };
        (site, hp, here)
    }

    #[test]
    fn a_fan_mev_refuses_the_plane_x_nurbs_class_where_nothing_moves_and_mev_null_splits_it() {
        // The over-refusal the gate's docs state, through the public
        // door: `recertify` answers `NurbsLaneNotSupplied` for the M7-8 class
        // before any endpoint check, so `mev` refuses at the old
        // vertex's own point (the closed spec) as at a moved one (a
        // chord), body untouched. The no-move split is `mev_null`, which
        // carries the run's certificate untouched; its new edge is then
        // described by a second call.
        let tol = Tol::witness();
        let (mut body, edge) = m7_8_pillow(tol);
        let (site, hp, here) = m7_8_site(&body, edge);
        let moved = Point3::new(here.x, here.y, here.z + 1.0);
        let before = deep_snapshot(&body);
        for (point, spec) in [
            (here, geom_brep::EdgeCurveSpec::self_loop_circle_at(here)),
            (moved, geom_brep::EdgeCurveSpec::line_between(here, moved)),
        ] {
            assert_eq!(
                body.mev(site, point, spec, tol).map(|_| ()),
                Err(EulerOpError::RebasedCarrier {
                    edge,
                    error: geom_brep::CertifyError::NurbsLaneNotSupplied,
                }),
                "the M7-8 edge, mev to {point:?}"
            );
            assert_eq!(deep_snapshot(&body), before, "mev to {point:?}");
        }
        let carrier_before = carrier_bits(&body, edge);
        let created = body.mev_null(site, crate::NewVertexSide::Above).unwrap();
        assert_eq!(
            body.get_half_edge(hp).unwrap().start,
            created.vertex,
            "the M7-8 edge moved onto the new vertex"
        );
        assert_eq!(carrier_bits(&body, edge), carrier_before);
        body.set_edge_curve(
            created.edge,
            geom_brep::EdgeCurveSpec::self_loop_circle_at(here),
            tol,
        )
        .unwrap();
        // What the two calls leave: tiers 1 and 2 accept it.
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(crate::validate::validate_closed(&body), Ok(()));
        // Tier 3 does not. Over the pillow's own at-rest reading it
        // adds two, both on the new edge: its circle's description is
        // the scaffolding door's, which is transient and not at rest
        // (`ScaffoldAtRest`), and the circle leaves the plane face it
        // bounds (`PlanarBoundaryResidual`). No
        // description could do better here: the new edge lies on the
        // plane `y = 0` and the `z = 0` patch, which meet in a line, and
        // a closed edge of positive length cannot lie in a line. The
        // no-move split is therefore not an at-rest-certified door.
        let (pillow, _) = m7_8_pillow(tol);
        let mut expected = crate::validate::validate_geometric(&pillow, tol).unwrap_err();
        let plane_face = [created.he_plus, created.he_minus]
            .into_iter()
            .map(|he| body.face_of_half_edge(he).unwrap())
            .find(|&f| {
                matches!(
                    body.get_surface(body.get_face(f).unwrap().surface),
                    Some(geom::Surface::Plane { .. })
                )
            })
            .unwrap();
        expected.extend([
            crate::ValidationError::ScaffoldAtRest { edge: created.edge },
            crate::ValidationError::PlanarBoundaryResidual {
                face: plane_face,
                edge: created.edge,
            },
        ]);
        assert_eq!(
            crate::validate::validate_geometric(&body, tol),
            Err(expected)
        );
    }

    #[test]
    fn the_no_move_split_leaves_a_null_edge_at_rest_when_its_second_call_fails() {
        // The two calls are not one door: `mev_null` has already run
        // when `set_edge_curve` refuses, and what is left is a null edge
        // at rest, which tier 2 refuses. (A single certified door would
        // leave the body untouched on `Err`.)
        let tol = Tol::witness();
        let (mut body, edge) = m7_8_pillow(tol);
        let (site, _hp, here) = m7_8_site(&body, edge);
        let created = body.mev_null(site, crate::NewVertexSide::Above).unwrap();
        let off = Point3::new(here.x + 1.0, here.y, here.z);
        let err = body.set_edge_curve(
            created.edge,
            geom_brep::EdgeCurveSpec::line_between(here, off),
            tol,
        );
        assert!(
            matches!(err, Err(EulerOpError::Certification { .. })),
            "{err:?}"
        );
        assert_eq!(
            crate::validate::validate_closed(&body),
            Err(vec![crate::ValidationError::NullEdgeAtRest {
                edge: created.edge
            }])
        );
    }

    #[test]
    fn a_certified_mev_reaches_a_run_moving_fan_site_through_a_closed_carrier() {
        // The certified door is NOT confined to strut sites. A closed
        // carrier's two endpoints are the same point, so a self-loop
        // circle minted AT the old vertex's own point gives the new
        // edge a forward interval — and the run then moves onto a
        // vertex at its own coordinates, which every spoke's chord
        // still reaches. The gate carries, and the surgery runs.
        let tol = Tol::witness();
        let (mut body, _seed, [_a, b, c, d]) = four_spoke_star();
        let p_old = p(0.0);
        let before = [carrier_bits(&body, b.edge), carrier_bits(&body, c.edge)];
        let created = body
            .mev(
                MevSite::Fan {
                    he1: b.he_plus,
                    he2: d.he_plus,
                },
                p_old,
                geom_brep::EdgeCurveSpec::self_loop_circle_at(p_old),
                tol,
            )
            .unwrap();
        assert_eq!(validate(&body), Ok(()));
        assert_eq!(
            body.get_half_edge(b.he_plus).unwrap().start,
            created.vertex,
            "the run moved onto the new vertex"
        );
        let after = [carrier_bits(&body, b.edge), carrier_bits(&body, c.edge)];
        assert_eq!(
            before, after,
            "the moved run's certificates are the ones it had"
        );
    }

    #[test]
    fn the_gate_carries_a_run_an_earlier_kev_had_already_made_stale() {
        // The gate's claim is the narrow one: no edge's carrier is made
        // false BY THIS MOVE. A carrier already missing its own
        // endpoint is planted here with the kill's ungated execution
        // (both kill doors refuse to leave one), and a later `mev` that
        // moves that edge nowhere must not refuse in its name — the
        // edge is no worse for this op, and tier 3 is what reports it.
        let tol = Tol::witness();
        let (mut body, _seed, [a, _b, _c, _d]) = four_spoke_star();
        let s = body
            .mev_line(
                MevSite::Fan {
                    he1: a.he_minus,
                    he2: a.he_minus,
                },
                p(9.0),
                tol,
            )
            .unwrap();
        // Kills the far tip; `s` merges onto `a`'s start vertex while
        // its chord still runs to the dead vertex's point.
        body.kev_ungated(a.he_plus).unwrap();
        let stale = body.get_edge(s.edge).unwrap().he_plus;
        let orbit = body.vertex_orbit(stale).unwrap();
        let i = orbit.iter().position(|&h| h == stale).unwrap();
        let he2 = orbit[(i + 1) % orbit.len()];
        let p_old = p(0.0);
        assert_eq!(
            body.certify_rebased_run(&[stale], p_old, tol),
            Ok(()),
            "a carrier this move does not touch is not this operator's refusal"
        );
        body.mev(
            MevSite::Fan { he1: stale, he2 },
            p_old,
            geom_brep::EdgeCurveSpec::self_loop_circle_at(p_old),
            tol,
        )
        .unwrap();
        assert_eq!(validate(&body), Ok(()));
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
        // A certified mev cannot reach this surgery AT A MOVED POINT:
        // the run's spokes would start at a vertex their chords do not
        // run to, and the re-basing gate refuses. At the old vertex's
        // own point it reaches it through a closed carrier
        // (`a_certified_mev_reaches_a_run_moving_fan_site_through_a_closed_carrier`);
        // this row is about WHICH halves move, so it drives the
        // surgery through `mev_null`, whose new vertex is the old
        // one's point — nothing moves, so every spoke's certificate is
        // still its own.
        assert!(matches!(
            body.clone().mev_line(site, p(5.0), Tol::witness()),
            Err(EulerOpError::RebasedCarrier { edge, .. }) if edge == b.edge
        ));
        let split = body.mev_null(site, crate::NewVertexSide::Above).unwrap();
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
        assert_eq!(
            body.vertex_orbit(seg.he_plus),
            Some(vec![seg.he_plus, split.he_plus])
        );

        let site = MevSite::Fan {
            he1: seg.he_plus,
            he2: split.he_plus,
        };
        // As in the valence-4 star: the certified door refuses to move
        // `seg`'s chord to p(2.0), a vertex it does not run to, so the
        // cross-loop splice is pinned through the coincident door.
        assert!(matches!(
            body.clone().mev_line(site, p(2.0), Tol::witness()),
            Err(EulerOpError::RebasedCarrier { edge, .. }) if edge == seg.edge
        ));
        let fan = body.mev_null(site, crate::NewVertexSide::Above).unwrap();
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
        let seed = body.mvfs(p(0.0), true).unwrap();
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
            body.mev_line(MevSite::Lone { r#loop: dead }, p(9.0), Tol::witness())
                .unwrap_err()
        });
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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

    /// A segment with a strut at its far end, torn by two `next`
    /// writes so that the far vertex's clockwise walk from the strut is
    /// `[strut+, seg+, seg−]`: it closes and reaches both halves at the
    /// vertex, through `seg+`, which starts at the segment's other end.
    fn torn_strutted_segment() -> (Body<f64>, MevCreated, MevCreated) {
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
        body.get_half_edge_mut(strut.he_minus).unwrap().next = seg.he_plus;
        body.get_half_edge_mut(seg.he_minus).unwrap().next = seg.he_minus;
        assert_eq!(
            body.vertex_orbit(strut.he_plus),
            Some(vec![strut.he_plus, seg.he_plus, seg.he_minus])
        );
        (body, seg, strut)
    }

    /// Every fan door at `(he1, he2)` on `body`, each refusing
    /// `OrbitBroken` naming `he1` with the body untouched: `mev_null`,
    /// `mev_line` to a moved point, and a certified `mev` that moves
    /// nothing (a closed carrier at the old point, which the re-basing
    /// gate passes wherever the run starts at the split vertex).
    fn every_fan_door_refuses_the_torn_walk(
        body: &mut Body<f64>,
        he1: HalfEdgeKey,
        he2: HalfEdgeKey,
    ) {
        let tol = Tol::witness();
        let site = MevSite::Fan { he1, he2 };
        let v = body.get_half_edge(he1).unwrap().start;
        let at = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let torn = EulerOpError::OrbitBroken { he: he1 };
        assert_err_deep_unchanged(body, &torn, |b| {
            b.mev_null(site, crate::NewVertexSide::Above).unwrap_err()
        });
        assert_err_deep_unchanged(body, &torn, |b| {
            b.mev_line(site, at + geom_core::Vec3::new(0.5, 0.0, 0.0), tol)
                .unwrap_err()
        });
        assert_err_deep_unchanged(body, &torn, |b| {
            b.mev(site, at, EdgeCurveSpec::self_loop_circle_at(at), tol)
                .unwrap_err()
        });
    }

    #[test]
    fn a_torn_orbit_at_a_fan_site_refuses_typed_in_every_fan_door() {
        // From the strut the torn half is in the moved run, and a fan
        // split would re-base `seg+` off the segment's other end; from
        // the segment the run is `[seg−]` and the torn half follows
        // `he2`, and a split would carry the torn walk into its
        // result. Both are the split vertex's orbit only by the walk's
        // say-so, and every door refuses in the plan phase.
        let (mut body, seg, strut) = torn_strutted_segment();
        every_fan_door_refuses_the_torn_walk(&mut body, strut.he_plus, seg.he_minus);
        every_fan_door_refuses_the_torn_walk(&mut body, seg.he_minus, strut.he_plus);
    }

    /// The declined cube torn by two `next` writes, by position in its
    /// half-edge arena, with the arena's keys. A static witness: the
    /// walk from `halves[5]` closes through `halves[6]` and leaves its
    /// start vertex on the way, and so does the walk from `halves[6]`.
    fn twice_torn_cube() -> (Body<f64>, Vec<HalfEdgeKey>) {
        let tol = Tol::witness();
        let mut body = crate::test_support_fixtures::declined_cube::<f64>(tol).body;
        let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
        body.get_half_edge_mut(halves[19]).unwrap().next = halves[6];
        body.get_half_edge_mut(halves[4]).unwrap().next = halves[11];
        for (from, to) in [(halves[5], halves[6]), (halves[6], halves[5])] {
            let v = body.get_half_edge(from).unwrap().start;
            let orbit = body.vertex_orbit(from).unwrap();
            assert!(orbit.contains(&to), "the walk from {from:?} reaches {to:?}");
            assert!(
                orbit
                    .iter()
                    .any(|&h| body.get_half_edge(h).unwrap().start != v),
                "the walk from {from:?} leaves its start vertex"
            );
        }
        (body, halves)
    }

    #[test]
    fn a_twice_torn_declined_cube_refuses_a_fan_split_typed() {
        let (mut body, halves) = twice_torn_cube();
        let (a, b) = (halves[5], halves[6]);
        every_fan_door_refuses_the_torn_walk(&mut body, a, b);
        every_fan_door_refuses_the_torn_walk(&mut body, b, a);
    }

    #[test]
    fn a_strut_on_a_torn_orbit_refuses_typed_in_every_fan_door() {
        // A strut moves no half-edge but splices its new plus half into
        // the walk from `he1`; on a walk that leaves the vertex that is
        // a torn orbit, and every door refuses in the plan phase.
        let (mut body, seg, strut) = torn_strutted_segment();
        for he in [strut.he_plus, seg.he_minus] {
            every_fan_door_refuses_the_torn_walk(&mut body, he, he);
        }
        let (mut cube, halves) = twice_torn_cube();
        for he in [halves[5], halves[6]] {
            every_fan_door_refuses_the_torn_walk(&mut cube, he, he);
        }
    }

    #[test]
    fn a_torn_walk_refuses_ahead_of_a_dangling_prev_link() {
        // The orbit proof precedes the `prev` links in the documented
        // order, so a torn walk with a dangling `prev(he1)` or
        // `prev(he2)` names the torn walk, not the stale link.
        let (body, seg, strut) = torn_strutted_segment();
        for (he1, he2) in [(strut.he_plus, seg.he_minus), (seg.he_minus, strut.he_plus)] {
            for dangling in [he1, he2] {
                let mut body = body.clone();
                body.get_half_edge_mut(dangling).unwrap().prev = HalfEdgeKey::default();
                every_fan_door_refuses_the_torn_walk(&mut body, he1, he2);
            }
        }
    }

    #[test]
    fn a_fan_split_on_the_untorn_cube_moves_exactly_its_orbit_slice() {
        // The control: the counterexample's sites on the cube before
        // the tears. Every door that moves nothing splits, the run
        // `[he1 .. he2)` of the vertex's orbit moves to the new vertex
        // and nothing else changes its start.
        let tol = Tol::witness();
        let cube = crate::test_support_fixtures::declined_cube::<f64>(tol).body;
        let halves: Vec<HalfEdgeKey> = cube.half_edges().map(|(k, _)| k).collect();
        let (a, b) = (halves[5], halves[6]);
        for (he1, he2) in [(a, b), (b, a)] {
            let site = MevSite::Fan { he1, he2 };
            let v = cube.get_half_edge(he1).unwrap().start;
            let at = *cube.get_point(cube.get_vertex(v).unwrap().point).unwrap();
            let orbit = cube.vertex_orbit(he1).unwrap();
            let run = &orbit[..orbit.iter().position(|&h| h == he2).unwrap()];
            assert!(!run.is_empty());
            for door in ["mev_null", "mev"] {
                let mut body = cube.clone();
                let created = if door == "mev_null" {
                    body.mev_null(site, crate::NewVertexSide::Above).unwrap()
                } else {
                    body.mev(site, at, EdgeCurveSpec::self_loop_circle_at(at), tol)
                        .unwrap()
                };
                assert_eq!(validate(&body), Ok(()), "{door}");
                for &h in &halves {
                    let start = body.get_half_edge(h).unwrap().start;
                    let expected = if run.contains(&h) {
                        created.vertex
                    } else {
                        cube.get_half_edge(h).unwrap().start
                    };
                    assert_eq!(start, expected, "{door}: {h:?}");
                }
            }
        }
    }

    #[test]
    fn chords_in_different_loops_are_rejected() {
        let mut t = pillow(Tol::witness());
        let expected = EulerOpError::NotSameLoop {
            he1: t.hes_a[0],
            he2: t.hes_b[0],
        };
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
            body.mev_line(MevSite::Lone { r#loop: t.loop_a }, p(9.0), Tol::witness())
                .unwrap_err()
        });
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        assert_err_deep_unchanged(&mut t.body, &expected, |body| {
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
        let seed = body.mvfs(p(0.0), true).unwrap();
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

    /// Display smoke test, one sample per [`EulerOpError`] variant,
    /// over the crate's shared sample array
    /// ([`every_euler_op_error_once`], which carries the coverage and
    /// discriminant-order assertions this row used to keep).
    #[test]
    fn every_error_displays() {
        for error in every_euler_op_error_once() {
            assert!(!error.to_string().is_empty(), "{error:?}");
        }
    }

    /// **The corruption refusals end one way** (D4 ¶1 (i)): every
    /// variant [`EulerOpError::reports_tier1_corruption`] answers `true`
    /// for ends in [`geom_core::KERNEL_DEFECT_ENDING`], its one recourse,
    /// but for the three a caller reaches too, which state the fact and
    /// claim neither a recourse nor a defect; no other variant names a
    /// defect. `PcurveMint` answers by its payload, so both of its sides
    /// are sampled beside the shared array.
    #[test]
    fn corruption_refusals_end_in_the_kernel_defect_ending() {
        use crate::pcurves::SiteRowRefusal;
        use EulerOpErrorKind as K;
        const CALLERS_TOO: [K; 3] = [K::StaleKey, K::StaleGeometry, K::NotSameEdge];
        let pcurve_mint = |refusal| EulerOpError::PcurveMint {
            face: FaceKey::default(),
            refusal,
        };
        let samples = every_euler_op_error_once().into_iter().chain([
            pcurve_mint(SiteRowRefusal::Corrupt),
            pcurve_mint(SiteRowRefusal::KeysOnly),
        ]);
        let mut corrupt = 0;
        for error in samples {
            let text = error.to_string();
            if !error.reports_tier1_corruption() {
                assert!(
                    !text.contains("kernel defect") && !text.contains("malformed"),
                    "{text}"
                );
                continue;
            }
            corrupt += 1;
            if CALLERS_TOO.contains(&K::from(&error)) {
                assert!(
                    !text.contains("kernel defect")
                        && !text.contains("malformed")
                        && !text.contains("torn"),
                    "{text}"
                );
                assert_eq!(test_utils::refusal::recourse_markers(&text), 0, "{text}");
            } else {
                assert!(
                    text.ends_with(&format!(". {}", geom_core::KERNEL_DEFECT_ENDING)),
                    "{text}"
                );
                assert_eq!(test_utils::refusal::recourse_markers(&text), 1, "{text}");
            }
        }
        assert_eq!(corrupt, 12, "the corruption samples this row reads");
    }

    /// **`split_edge`'s interiority arms tell one story** (D4 ¶1 (iv)),
    /// each on a real raise: the crossing within the zero band of an
    /// end, the one outside the edge, and the one in the band. All three
    /// end in the one lever every splitting door has, and none offers a
    /// declaration. The band-decided arms (the zero one and the
    /// undecided one) of this decision, which passes on a positive
    /// margin, offer the tolerance their margin gives; the sign-certain
    /// arm offers none.
    #[test]
    fn split_param_arms_tell_one_story() {
        use geom_brep::recourse::{Classified, Refused};
        const LEVER: &str =
            "Recourse: move the geometry so the crossing lands clearly away from the edge's ends";
        let tol = geom_core::Tol::witness();
        let band = Band::linear(tol).unwrap();
        let k = band.escalate() / band.zero();
        let raise = |t: f64| {
            let cube = crate::test_support_fixtures::declined_cube::<f64>(tol);
            let mut body = cube.body;
            body.split_edge(cube.mevs[0].edge, t, tol).unwrap_err()
        };
        let offered = |text: &str| {
            text.split_once(
                ", or, if this distance from the edge's end is intended, tighten \
                             the tolerance below ",
            )
            .map(|(_, v)| v.strip_suffix(" m").unwrap().parse::<f64>().unwrap())
        };
        let value = |m: geom_core::MarginDiag| {
            m.diagnostic_f64_for_error_text()
                .value()
                .expect("a point margin")
        };
        let zero = raise(0.5 * band.zero());
        let outside = raise(1.5);
        let undecided = raise((band.zero() + band.escalate()) * 0.5);
        let want = match (&zero, &outside, &undecided) {
            (
                EulerOpError::SplitParamNotInterior {
                    verdict: Refused::Zero(Classified { margin: z, .. }),
                    ..
                },
                EulerOpError::SplitParamNotInterior {
                    verdict: Refused::Negative { .. },
                    ..
                },
                EulerOpError::SplitParamEscalated { diag, .. },
            ) => [Some(value(*z) / k), None, Some(value(diag.margin) / k)],
            other => panic!("a zero-band, an outside and an in-band split: {other:?}"),
        };
        for (err, want) in [zero, outside, undecided].iter().zip(want) {
            let text = err.to_string();
            assert!(
                text.contains(LEVER) && !text.contains("declare"),
                "the one lever, and no declaration: {text}"
            );
            assert_eq!(offered(&text), want, "the tolerance its arm gives: {text}");
        }
    }

    #[test]
    fn mef_refuses_a_run_walked_through_another_loop() {
        // The anchor probe's first `mef` counterexample
        // (`review_d18::kill_anchors_on_torn_bodies`: the strut cube,
        // seed 26, two `next` tears): the walk from `he1` is diverted
        // through another loop and back, so the run `[he1 .. he2)`
        // carries that loop's anchor into the new loop. Unchecked, that
        // loop is left anchored in another, and the split returns `Ok`.
        let mut body = crate::fixtures::ops_strut_cube(Tol::witness()).body;
        let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
        body.get_half_edge_mut(halves[14]).unwrap().next = halves[25];
        body.get_half_edge_mut(halves[24]).unwrap().next = halves[13];
        let (he1, he2) = (halves[4], halves[13]);
        let loop_key = body.get_half_edge(he1).unwrap().parent_loop;
        let walk = body.loop_cycle(he1).unwrap();
        let run = &walk[..walk.iter().position(|&x| x == he2).unwrap()];
        assert!(
            body.loops().any(|(l, data)| {
                l != loop_key
                    && matches!(data.boundary, LoopBoundary::Cycle { first } if run.contains(&first))
            }),
            "the run takes another loop's anchor"
        );
        let torn = EulerOpError::LoopCycleBroken { r#loop: loop_key };
        crate::fixtures::assert_make_refuses(&mut body, &torn, |b| {
            b.mef_chord(MefSite::Chords { he1, he2 }, Tol::witness())
        });
    }

    #[test]
    fn mef_chord_refuses_in_mefs_own_order() {
        // Two faults at once, after the review's two-fault probe: the
        // diverted run above, and `start(he1)` removed, which `mef`
        // checks after the walk. The sugar derives its chord inside
        // `mef`'s plan, so it refuses what `mef` does.
        let mut body = crate::fixtures::ops_strut_cube(Tol::witness()).body;
        let halves: Vec<HalfEdgeKey> = body.half_edges().map(|(k, _)| k).collect();
        body.get_half_edge_mut(halves[14]).unwrap().next = halves[25];
        body.get_half_edge_mut(halves[24]).unwrap().next = halves[13];
        let (he1, he2) = (halves[4], halves[13]);
        let loop_key = body.get_half_edge(he1).unwrap().parent_loop;
        let start = body.get_half_edge(he1).unwrap().start;
        let mut body = body
            .with_entity_removed_for_tests(EntityId::Vertex(start))
            .unwrap();
        let site = MefSite::Chords { he1, he2 };
        let torn = EulerOpError::LoopCycleBroken { r#loop: loop_key };
        let spec = EdgeCurveSpec::line_between(p(0.0), p(1.0));
        crate::fixtures::assert_make_refuses(&mut body.clone(), &torn, |b| {
            b.mef(site, spec, FaceSurface::Inherit, Tol::witness())
        });
        crate::fixtures::assert_make_refuses(&mut body, &torn, |b| {
            b.mef_chord(site, Tol::witness())
        });
    }
}

/// **Every naming relation the validator's tier-1 reference passes
/// check, and what a kill that removes the named record does about it.**
///
/// The relations are DERIVED from `validate.rs`: every
/// `ValidationError::Dangling*` or `ValidationError::Stale*` that
/// `tier1` constructs is one relation, keyed by the variant and the
/// kinds its fields wrap (`EntityId::Face(..)` reads `Face`), or by its
/// field names where it wraps none. `RELATIONS` gives each one a
/// disposition: the helper whose scan reads the naming field when a
/// kill removes the record it names, checked against that helper's
/// body — a proof that refuses before the kill, or, for the null-face
/// records a kill, or a move of a loop off its face, maintains rather
/// than refuses over, the drop in its mutation phase. A relation
/// `tier1` gains, or a variant of either
/// prefix the enum gains that `tier1` does not construct and
/// `NOT_BODY_RELATIONS` does not place, reds here until it is given
/// one.
///
/// What this cannot see: a relation the validator checks without a
/// `Dangling*` or `Stale*` variant, and a second field of one kind pair
/// checked at a site the key already names (a face's `outer` and
/// `rings` are one site, so one key; its reads name both fields).
/// Whether each kill calls the helper is the operators' rows' to show,
/// not this one's.
#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod removal_census {
    use std::collections::BTreeSet;
    use test_utils::source::{
        ItemBody, balanced_end, code_only, ident, item_body, top_level_split,
    };

    const VALIDATE: &str = include_str!("validate.rs");
    const HELPER_SOURCES: [&str; 2] = [include_str!("euler.rs"), include_str!("body.rs")];

    /// What a kill does about a relation's naming field.
    enum Disposition {
        /// The helper whose scan reads the field, and the code
        /// fragments its body reads it through.
        Read(&'static str, &'static [&'static str]),
    }
    use Disposition::Read;

    /// One disposition per relation `tier1` checks, in its pass order.
    const RELATIONS: [(&str, Disposition); 20] = [
        (
            "DanglingTopology: Solid -> Shell",
            Read("require_shell_unnamed", &["data.shells.contains"]),
        ),
        (
            "DanglingTopology: Shell -> Face",
            Read("require_face_unnamed", &["data.faces.contains"]),
        ),
        (
            "DanglingTopology: Shell -> Solid",
            Read("require_solid_unnamed", &["data.solid =="]),
        ),
        (
            "DanglingGeometry: Face -> Surface",
            Read("remove_surface_if_orphaned", &["face.surface =="]),
        ),
        (
            "DanglingTopology: Face -> Loop",
            Read(
                "require_loop_unlisted",
                &["data.outer ==", "data.rings.contains"],
            ),
        ),
        (
            "DanglingTopology: Face -> Shell",
            Read("require_shell_unnamed", &["data.shell =="]),
        ),
        (
            "DanglingTopology: Loop -> Vertex",
            Read("empty_at_besides", &["data.boundary =="]),
        ),
        (
            "DanglingTopology: Loop -> HalfEdge",
            Read(
                "require_killed_halves_unnamed",
                &["LoopBoundary::Cycle { first } if killed(first)"],
            ),
        ),
        (
            "DanglingTopology: Loop -> Face",
            Read("require_face_unnamed", &["data.face =="]),
        ),
        (
            "DanglingTopology: HalfEdge -> Edge",
            Read("require_edge_unnamed", &["data.edge =="]),
        ),
        (
            "DanglingTopology: HalfEdge -> Vertex",
            Read("starts_at_besides", &["data.start =="]),
        ),
        (
            "DanglingTopology: HalfEdge -> Loop",
            Read("require_run_of", &["data.parent_loop =="]),
        ),
        (
            "DanglingTopology: HalfEdge -> HalfEdge",
            Read("require_killed_halves_unnamed", &[".links_of(h, data)"]),
        ),
        (
            "DanglingTopology: Edge -> HalfEdge",
            Read(
                "require_killed_halves_unnamed",
                &["data.he_plus, data.he_minus"],
            ),
        ),
        (
            "DanglingGeometry: Edge -> Curve",
            Read("remove_curve_if_orphaned", &["edge.curve =="]),
        ),
        (
            "DanglingDescription: Curve -> Surface",
            Read("remove_surface_if_orphaned", &["description_surfaces("]),
        ),
        (
            "DanglingGeometry: Vertex -> Point",
            Read("remove_point_if_orphaned", &["vertex.point =="]),
        ),
        (
            "DanglingTopology: Vertex -> HalfEdge",
            Read("require_killed_halves_unnamed", &["data.emanating"]),
        ),
        (
            "StaleNullFaceLoop: face, named_loop",
            Read("drop_null_face_records_naming", &["pair.loops().contains"]),
        ),
        (
            "StaleNullFaceOwnership: face, named_loop",
            Read("drop_null_face_records_naming", &["pair.loops().contains"]),
        ),
    ];

    /// Variants of either prefix that name no record of a `Body`, so no
    /// kill can leave one naming a record it removes.
    const NOT_BODY_RELATIONS: [(&str, &str); 1] = [(
        "StaleContactDeclaration",
        "tier 3': a contact declaration is the caller's, not the body's",
    )];

    fn is_reference_variant(name: &str) -> bool {
        name.starts_with("Dangling") || name.starts_with("Stale")
    }

    /// The body of the one item whose head `head` names, in `code`.
    fn body_of<'a>(code: &'a str, head: &str) -> &'a str {
        let mut found = code
            .match_indices(head)
            .map(|(at, _)| match item_body(code, at) {
                ItemBody::Body(range) => &code[range],
                other => panic!("`{head}` has a body, not {other:?}"),
            });
        let body = found
            .next()
            .unwrap_or_else(|| panic!("`{head}` is in the source"));
        assert!(found.next().is_none(), "`{head}` is one item");
        body
    }

    /// Every `Dangling*` / `Stale*` construction in `tier1`: its
    /// variant, and its key.
    fn tier1_relations() -> Vec<(String, String)> {
        let code = code_only(VALIDATE);
        let tier1 = body_of(&code, "fn tier1<");
        let mut out = Vec::new();
        for (at, needle) in tier1.match_indices("ValidationError::") {
            let name = ident(tier1, at + needle.len());
            if !is_reference_variant(name) {
                continue;
            }
            let after = at + needle.len() + name.len();
            let open = after + tier1[after..].find('{').expect("a struct variant");
            assert!(
                tier1[after..open].trim().is_empty(),
                "`{name}` is constructed with its fields"
            );
            let close = balanced_end(tier1, open).expect("the fields close");
            let fields = &tier1[open + 1..close];
            let mut kinds: Vec<(usize, &str)> = ["EntityId::", "GeomRef::"]
                .iter()
                .flat_map(|wrap| {
                    fields
                        .match_indices(wrap)
                        .map(move |(k, _)| (k, ident(fields, k + wrap.len())))
                })
                .collect();
            kinds.sort_unstable();
            let key = if kinds.is_empty() {
                top_level_split(fields, ',')
                    .into_iter()
                    .map(|range| fields[range].trim())
                    .filter(|field| !field.is_empty())
                    .map(|field| ident(field, 0))
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                kinds
                    .iter()
                    .map(|&(_, kind)| kind)
                    .collect::<Vec<_>>()
                    .join(" -> ")
            };
            out.push((name.to_string(), format!("{name}: {key}")));
        }
        out
    }

    /// The enum's `Dangling*` / `Stale*` variants.
    fn reference_variants() -> BTreeSet<String> {
        let code = code_only(VALIDATE);
        let body = body_of(&code, "pub enum ValidationError ");
        let inner = &body[1..body.len() - 1];
        top_level_split(inner, ',')
            .into_iter()
            .map(|range| {
                let mut item = inner[range].trim_start();
                while item.starts_with("#[") {
                    let end = balanced_end(item, 1).expect("an attribute closes");
                    item = item[end + 1..].trim_start();
                }
                ident(item, 0).to_string()
            })
            .filter(|name| is_reference_variant(name))
            .collect()
    }

    #[test]
    fn every_relation_validate_checks_is_read_by_a_kill_helper() {
        let found = tier1_relations();
        let keys: Vec<&str> = found.iter().map(|(_, key)| key.as_str()).collect();
        let listed: Vec<&str> = RELATIONS.iter().map(|&(key, _)| key).collect();
        assert_eq!(
            keys, listed,
            "the relations `tier1` checks, in pass order, against `RELATIONS`: a relation \
             it gained is owed a disposition here"
        );
        let constructed: BTreeSet<String> = found.into_iter().map(|(name, _)| name).collect();
        let placed: BTreeSet<String> = NOT_BODY_RELATIONS
            .iter()
            .map(|&(name, _)| name.to_string())
            .collect();
        assert!(
            constructed.is_disjoint(&placed),
            "a variant `tier1` constructs is a body relation: {constructed:?} / {placed:?}"
        );
        assert_eq!(
            reference_variants(),
            &constructed | &placed,
            "every `Dangling*` / `Stale*` variant is a relation `tier1` checks or is placed \
             in `NOT_BODY_RELATIONS`"
        );
        let helpers: Vec<String> = HELPER_SOURCES.iter().map(|s| code_only(s)).collect();
        for (key, Read(helper, reads)) in &RELATIONS {
            let head = format!("fn {helper}(");
            let sources: Vec<&String> =
                helpers.iter().filter(|code| code.contains(&head)).collect();
            let [source] = sources[..] else {
                panic!("`{key}`: `{helper}` is defined once among the helper sources");
            };
            let body = body_of(source, &head);
            for read in *reads {
                assert!(body.contains(read), "`{key}`: `{helper}` reads `{read}`");
            }
        }
    }
}

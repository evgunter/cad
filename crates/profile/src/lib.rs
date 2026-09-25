//! 2-D sketch profiles as data: the [`Profile`] of closed loops and its
//! trilean validation into [`ValidatedProfile`] (M2 PR 2).
//!
//! A profile is the *input* to sweeps (M2 PR 4/5): closed 2-D loops on a
//! [`SketchPlane`], stored as plain data. The one consistency
//! condition the stored form has — an arc's carrier and sweep agree
//! with its two vertices — holds by construction: every loop is lowered
//! from its chords and bulges, and validation reads the carrier without
//! re-checking it. A predicate that verifies a stored carrier against
//! its vertices is owed once carriers are stored rather than derived.
//! Sweeps accept only a
//! [`ValidatedProfile`], the canonicalized output of
//! [`Profile::validate`]; arcs lower to `geom` circle carriers at
//! sweep time — this crate stays 2-D and depends on `geom-core` only.
//!
//! # Profile conventions (normative, stated once)
//!
//! - **Units (D6):** coordinates in meters in the sketch plane's (x, y)
//!   chart; angles in radians.
//! - **The stored form.** A [`ProfileLoop`] is its vertices, stored
//!   verbatim, plus one canonical [`Segment`] per edge: segment k runs
//!   from vertex k to vertex k+1, and the **last segment is the
//!   closing one back to the first vertex** — there is no open/closed
//!   flag and no way to write an open chain. A segment is a carrier
//!   plus a signed interval on it: a [`Segment::Line`] (the chord), or
//!   a [`Segment::Arc`] (centre, radius, and the signed sweep Δθ,
//!   positive counterclockwise). The vertices are authoritative;
//!   an arc's carrier and sweep agree with them, and validation reads
//!   the carrier rather than re-deriving it.
//! - **The bulge input form.** Loops are written as a vertex chain
//!   `[{pos, bulge}, …]` ([`ProfileVertex`]) — vertex k's `bulge`
//!   describes the segment leaving it — and LOWERED to the stored form
//!   once: a bulge of exactly zero (either sign) is a line, and any
//!   other an arc whose
//!   carrier is the closed form below and whose sweep is
//!   Δθ = 4·atan(b). The bulge each segment was lowered from is kept
//!   beside it ([`ProfileLoop::bulges`]).
//! - **Bulge semantics (DXF-compatible, ratified).** For the segment
//!   from vertex A to vertex B, `bulge` b = tan(θ/4) where θ is the
//!   arc's signed included angle; b = 0 is a straight line segment.
//!   **Positive b sweeps counterclockwise** about the arc's center
//!   (θ > 0): the path *turns left* in chain order, and the arc's apex
//!   bows to the **right** of the chord A→B — true for every θ (the
//!   sagitta formula below is exact for all b). The center's side
//!   depends on the arc class: for a **minor** arc (|b| < 1, |θ| < π)
//!   the center lies to the **left** of the chord (a minor arc bows
//!   away from its center); at |b| = 1 (a semicircle) the center is
//!   *on* the chord; for a **major** arc (|b| > 1, |θ| > π) the
//!   apothem L·(1 − b²)/(4b) changes sign and the center crosses to
//!   the **same side as the apex** (the arc wraps more than half the
//!   circle around it). Negative b is the mirror image of all of the
//!   above (clockwise sweep, apex left). The sign is *geometry, not
//!   winding*: it says which way this one segment curves, independent
//!   of the loop's traversal role.
//!   Closed forms used throughout, with L = |B − A|, û = (B − A)/L, and
//!   n̂ = û rotated +90° (the left normal):
//!   - signed sagitta (apex offset): apex = midpoint − n̂·(L·b/2);
//!   - signed apothem: center = midpoint + n̂·(L·(1 − b²)/(4b));
//!   - radius r = L·(1 + b²)/(4|b|); included angle θ = 4·atan(b).
//! - **Reversal is an involution.** Reversing a chain maps each
//!   segment's b ↦ −b (same locus, opposite traversal), so an arc
//!   keeps its carrier and its sweep changes sign;
//!   [`ProfileLoop::reversed`] implements the reindexing and
//!   `reversed ∘ reversed` is the identity, bit-exactly (negation is
//!   exact). Under test.
//! - **|Δθ| < 2π**: b = tan(θ/4) is finite, so no segment the bulge
//!   input form writes closes a full period, and validation refuses a
//!   loop of fewer than **2 vertices**: the minimal circle is two arcs.
//! - **Winding is invisible.** There is no direction concept in the
//!   API: users write loops in either traversal; [`Profile::validate`]
//!   derives nesting from containment and canonicalizes traversal
//!   internally (outers counterclockwise, holes clockwise, in sketch
//!   coordinates). No winding errors exist.
//! - **Tangency is declared intent, verified — never inferred (the
//!   #101 discipline).** A *joint* (the junction of two adjacent
//!   segments at their shared vertex) whose two **distinct** carriers
//!   meet tangentially must be declared in
//!   [`ProfileLoop::tangent_joints`]; validation refuses undeclared
//!   definite tangency ([`ProfileError::UndeclaredTangency`]) and
//!   contradicted declarations ([`ProfileError::TangencyContradicted`])
//!   alike. Same-carrier continuation (collinear lines, cocircular
//!   arcs — e.g. the minimal two-arc circle's joints) is carrier
//!   identity: legal undeclared, and **legal declared too** — every
//!   zero-turn joint is a declared tangent joint (Ev, in-chat,
//!   2026-09-02), because identity is a fact about the carriers and
//!   tangency a fact about the directions, which agree there. Free arcs
//!   whose joints are definitely transversal (secant carriers) remain
//!   legal undeclared — declaration marks *tangency*, not arc-ness. The
//!   authoring path is the PATHS lattice ([`path`]): `.fillet(r)`
//!   computes tangent geometry exactly and the continuation verbs
//!   declare the zero-turn joints they mint, both by construction.
//!   [`ProfileLoop::tangent_joints`] is the field that carries the
//!   result, and a fixture's way of writing one by hand.
//! - **The sketch plane is a placement, and validation never reads
//!   it.** [`SketchPlane`] is profile (x, y) ↦ plane origin + x·u +
//!   y·v, with u/v/normal the columns of the placement's linear part,
//!   and validation is purely 2-D (the plane is passed through
//!   untouched). Rigidity — u, v, normal orthonormal and right-handed
//!   — is the frame witness's: [`SketchPlane::from_frame`] takes an
//!   [`geom_core::OrthoFrame`], which was decided at its mint.
//!   [`SketchPlane::new`] is the read-back door and holds whatever
//!   [`geom_core::Affine3`] it is handed, so a placement that came
//!   from somewhere other than a frame carries only what its own
//!   source decided.
//!
//! # Validation and canonical form
//!
//! [`Profile::validate`] is the single gate: arity, degeneracy,
//! simplicity, the containment forest, and canonicalization, every
//! decision through named trilean predicates (geom-core's
//! [`geom_core::Decide`] door — see `crate::validate` docs for the
//! predicate inventory and `geom_core::k_stats` for the margin-statistics
//! hook). Its canonical-form rules (the authored starting vertex, loop
//! order, traversal senses) are documented on [`ValidatedProfile`]; the
//! canonical form is invariant under input traversal order of every
//! loop (under test — D9-load-bearing, since recipes replay this) and
//! keeps each loop's authored start. It is **not** invariant under
//! reordering the input's loop list: the outer is hoisted first, but
//! holes keep their discovery (input) order — a D9 recipe replays the
//! loop order it recorded, so reordering loops is a *different*
//! recipe, deliberately.
//!
//! **Deferred (named): session-box enforcement (D4 ¶4).** Nothing in
//! this crate rejects geometry outside the documented model size range
//! — construction sugar can mint absurd carriers from near-degenerate
//! input (e.g. [`bulge_from_via`] with a collinear-ish through-point
//! yields a ~1e15 m radius arc that validates if simple). D4 ¶4 wants
//! out-of-range geometry rejected at construction; that check is a
//! kernel-wide boundary (shared with `topo`/`geom`), not a
//! per-crate ad-hoc test, and lands as its own design item — this is
//! the first site where innocent input reaches the gap.
//!
//! # Totality
//!
//! Construction and sugar are evaluation code: total, comparison-free,
//! no panics; garbage in (NaN coordinates, a through-point coincident
//! with a chord endpoint) produces well-defined poison or degenerate
//! data *values*, which validation catches with typed errors — never a
//! guess (the poison policy of `geom_core::real`). The lowering adds no
//! poison of its own: a bulge of exactly zero lowers to a line, so no
//! stored arc has its centre at infinity. A stored arc's carrier is
//! poison only when its input was — a bulge or coordinate that is not
//! finite, which validation refuses typed — or when a tiny finite bulge
//! overflows its radius, which is a sub-tolerance arc that validation
//! classifies a line before any carrier is read.

mod fillet_select;
pub mod lift;
pub mod path;
mod seg;
pub mod structure;
mod sugar;
mod validate;

use geom_core::{Affine3, Mat3, OrthoFrame, Point2, Point3, Real, Vec3};

pub use lift::{Fidelity, LiftOutcome, LiftRefusal, lift, lift_checked};
pub use path::program::{
    ArcData, ArcMode, ClosedLoop, ReplayError, ReplayErrorKind, SpecForms, Step, Target,
    TargetKind, TipState, Verb, arc_specs_at, replay, replay_guided, replay_recording,
};
pub use path::{
    ArcCarrierScalar, ArcLen, ArcSide, ArrivesTangent, Bulge, Center, ContinueTarget, CornerReason,
    CornerRefusal, CornerWindow, LineTarget, Open, PartialPath, PathError, PathErrorKind,
    PathNoCornerReason, PointLeg, Radius, Start, Sweep, TangentArcTarget, Via, circle,
    circle_split,
};
pub use structure::{
    CanonicalStructure, CornerGate, Decision, DecisionValue, FilletDecision, LoopCanonical,
    ProfileStructure, RadiusEmission, RadiusRole, ReplayStructure, SegmentShape, StepSpan,
    StructureRefusal, StructureRefusalKind,
};
pub use sugar::{ArcSweep, FilletLegShape, bulge_from_center, bulge_from_via};
pub use validate::{
    BlendArc, ContactKind, EscalationSite, FilletLeg, FilletLegCarrier, LoopRole, NoCornerReason,
    ProfileError, SegmentKind, SegmentRef, ValidatedLoop, ValidatedProfile, ValidatedSegment,
};
/// The fillet recourse sentences and the map that selects one, under
/// `test-support` only.
///
/// They are prose a caller reads, so a suite that pins what a caller
/// reads has to spell them — and spelling them by restating the string
/// makes the assertion agree with itself instead of with the code.
/// This export is what lets `tests/fillet_recourse_followability.rs`
/// name them; it is off in every build that is not this crate's own
/// tests, so these are not part of the production surface, and
/// pncad's facade completeness guard does not see them.
///
/// [`validate::fillet_recourse_for`] rides the same export for the same
/// reason: the census row asserts that every `fillet_*` predicate name
/// the construction sugar decides has a sentence, and a census that
/// restated the mapping would be checking its own copy. So do
/// [`validate::SHARED_CLAUSE_ONLY`] and [`validate::shared_clause_only`]:
/// the roster row holds that list against the names the crate's `src`
/// decides, in both directions, and a restated copy would hold against
/// itself.
#[cfg(any(test, feature = "test-support"))]
pub use validate::{
    FILLET_ENCLOSING_RECOURSE, FILLET_FIT_RECOURSE, FILLET_FLATTENED_RECOURSE,
    FILLET_LEG_EXTENT_RECOURSE, FILLET_NO_CORNER_RECOURSE, FILLET_OFFSET_LEVER_RECOURSE,
    FILLET_SCENE_RESOLUTION_RECOURSE, FILLET_STORED_FORM_INBAND_RECOURSE,
    FILLET_TURN_INBAND_RECOURSE, SHARED_CLAUSE_ONLY, fillet_recourse_for, shared_clause_only,
};

/// One segment of a loop in its canonical form: a carrier plus a signed
/// interval on it (see the crate docs' segment semantics).
///
/// Segment `k` leaves vertex `k` and ends at vertex `k + 1 (mod n)`; the
/// vertices themselves are stored verbatim beside it and are
/// authoritative, so a segment carries no endpoint.
///
/// **The stored kind is exact, not a tolerance decision.** A segment
/// is stored as a `Line` exactly when the bulge it was lowered from is
/// exactly zero, of either sign, so a stored `Arc` is never built from
/// b = 0 and its carrier is always the finite-bulge closed form. A
/// stored `Arc` may still be sub-tolerance: whether it is too shallow
/// to be an arc is validation's ε-decision, and a
/// [`ValidatedSegment`]'s [`SegmentKind`] may classify it `Line`.
#[derive(Clone, Copy, Debug)]
pub enum Segment<T: Real> {
    /// A straight segment: its carrier is the chord between its two
    /// vertices, and the interval is the chord itself.
    Line,
    /// A circular arc.
    Arc {
        /// The carrier circle's centre (sketch coordinates).
        centre: Point2<T>,
        /// The carrier circle's radius (positive).
        radius: T,
        /// The signed sweep Δθ from the segment's start vertex to its
        /// end vertex about `centre`: positive counterclockwise.
        sweep: T,
    },
}

/// One vertex of the **bulge input form**: a position plus the bulge of
/// the segment *leaving* it toward the next vertex (see the crate docs'
/// bulge semantics).
///
/// This is an input record, not the stored form: the emission layer
/// builds its chain in it, and the fixture door takes a chain of them.
/// Both lower it to a [`ProfileLoop`]'s vertices and canonical
/// [`Segment`]s at once.
#[derive(Clone, Copy, Debug)]
pub struct ProfileVertex<T: Real> {
    pos: Point2<T>,
    bulge: T,
}

impl<T: Real> ProfileVertex<T> {
    /// A vertex: a position plus the bulge of the segment leaving it.
    /// The segment lowers to a [`Segment::Line`] when the bulge is
    /// exactly zero (either sign) and to a [`Segment::Arc`] otherwise.
    ///
    /// **What the privacy on this type does and does not claim.**
    /// Vertex *values* stay mintable wherever the type is nameable —
    /// this door is unconditional and every field reads back, exactly
    /// as for [`Point2`]. Privacy here buys representation freedom, not
    /// mint-prevention. The funnel claim is about LOOPS: outside this
    /// crate a [`ProfileLoop`] cannot be spelled from a vertex table:
    /// the lattice's emission layer and [`ProfileLoop::map_scalar`] are
    /// the only doors a shipped build has, and neither takes one. A
    /// caller holding a bag of vertices has nothing to put them in.
    pub fn new(pos: Point2<T>, bulge: T) -> Self {
        Self { pos, bulge }
    }

    /// **The leaf rung of the profile scalar lift**: the same vertex
    /// read at another scalar — the position through [`Point2::map`],
    /// the bulge through `f`.
    ///
    /// `map`, not `map_scalar`, because a vertex is a fixed pair of
    /// scalars with no structure to carry: `geom`'s `scalar_lift`
    /// convention is `map` on every leaf and `map_scalar` on every type
    /// whose lift has counts or indices to carry
    /// ([`ProfileLoop::map_scalar`], [`Profile::map_scalar`]).
    ///
    /// Structural, not arithmetic: every scalar goes through `f` and
    /// nothing is computed, so the lift is exact whenever `f` is.
    #[must_use]
    pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> ProfileVertex<U> {
        ProfileVertex::new(self.pos.map(&f), f(self.bulge))
    }

    /// The vertex position in sketch-plane coordinates (meters).
    pub fn pos(&self) -> Point2<T> {
        self.pos
    }

    /// The bulge b = tan(θ/4) of the segment from this vertex to the
    /// next (0 ⇒ line; sign per the crate docs — positive sweeps
    /// counterclockwise). The last vertex's bulge belongs to the
    /// implicit closing segment.
    pub fn bulge(&self) -> T {
        self.bulge
    }

    /// The canonical segment this vertex's leaving segment lowers to,
    /// given the vertex it ends at, by the one rule every lowering
    /// shares: a [`Segment::Line`] exactly when the bulge is exactly
    /// zero (either sign), and otherwise the arc [`lower_arc`] builds.
    pub(crate) fn lower_to(self, end: Point2<T>) -> Segment<T> {
        if is_exact_zero(self.bulge) {
            return Segment::Line;
        }
        let LoweredArc {
            centre,
            radius,
            sweep,
        } = lower_arc(self.pos, end, self.bulge);
        Segment::Arc {
            centre,
            radius,
            sweep,
        }
    }
}

/// An arc's carrier and sweep, as [`lower_arc`] derives them.
pub(crate) struct LoweredArc<T: Real> {
    /// The carrier circle's centre.
    pub centre: Point2<T>,
    /// The carrier circle's radius.
    pub radius: T,
    /// The signed sweep Δθ.
    pub sweep: T,
}

/// **The arc lowering**: the carrier [`seg::arc_carrier`] puts on the
/// chord `start → end` for `bulge`, and the sweep Δθ = 4·atan(b).
///
/// Pure arithmetic over its inputs, so it is the same expression at
/// every scalar: [`ProfileVertex::lower_to`] mints a stored arc
/// through it, and the validated form's lift rebuilds a validated
/// arc's carrier and sweep through it at the target scalar. A bulge of
/// exactly zero has no carrier (its centre is at infinity), and the
/// lowering rule sends it to a line before it reaches here.
pub(crate) fn lower_arc<T: Real>(start: Point2<T>, end: Point2<T>, bulge: T) -> LoweredArc<T> {
    let carrier = seg::arc_carrier(&seg::ChordFrame::of(start, end), bulge);
    LoweredArc {
        centre: carrier.center,
        radius: carrier.radius,
        sweep: T::from_f64(4.0) * bulge.atan(),
    }
}

/// Whether a bulge is exactly zero, of either sign — the line of the
/// bulge input form.
///
/// [`Real`] compares nothing, so the read is arithmetic: `b · 0` is
/// poison exactly when `b` is not finite, and for a finite `b`,
/// `b · (1/b)` is poison exactly when `b` is zero (0 · ∞; at `Interval`
/// the reciprocal of `[0, 0]` is empty). A poisoned bulge is not a
/// line; it lowers to an arc whose carrier is poison, which validation
/// refuses typed. This is an exact read, not a tolerance decision:
/// whether a nonzero bulge is too shallow to be an arc is validation's
/// `segment_straightness` question, asked of the bulge itself.
///
/// At `f64` the read is exactly `b == 0.0`, so a stored segment's kind
/// is the reading every consumer of the bulge form makes.
fn is_exact_zero<T: Real>(b: T) -> bool {
    !(b * T::zero()).is_poison() && (b * (T::one() / b)).is_poison()
}

/// A closed loop: a vertex chain, closed by construction (the last
/// vertex's segment returns to the first — see the crate docs).
///
/// Plain data — conventions are carried by data (D2), and nothing is
/// checked at construction: [`Profile::validate`] is the gate.
///
/// **A cache, not an authoring form.** The vertex table is what an
/// intensional recipe evaluates INTO — the same recipe→geometry seam
/// the kernel draws everywhere else — so nothing authors one by
/// writing coordinates down. **This is the one home for what mints a
/// loop; everywhere else points here.**
///
/// One PRIVATE constructor exists, and two public doors reach it:
///
/// - **the authoring door** — the [`path`] lattice's emission layer.
///   It classifies every junction and declares every tangency as the
///   chain is written, then calls the crate's private constructor. The
///   only door on the presented surface.
/// - **the materialization door** — [`ProfileLoop::map_scalar`]: a
///   table that already exists, read at another scalar. It authors
///   nothing; there is no table it can make that did not exist a moment
///   earlier. [`Profile::map_scalar`] is that same door run over a
///   profile's loops, not a second one.
/// - **fixtures** — `RawLoop` (unlinked deliberately: in a shipped
///   build it is a crate-private item, so a link from this public page
///   would name something the page's reader does not have), which IS
///   that private constructor,
///   additionally exported under `test`/`test-support`. In a shipped
///   build the trait item itself is `pub(crate)`, so no re-export of it
///   compiles and no downstream build can name it.
///
/// Two further materialization doors were anticipated by the Q1 ruling
/// and **do not exist**: a STEP-import face loop (`crates/step-import`
/// never names this crate) and a persisted-document read
/// (deserialization can never mint a `ProfileLoop` —
/// `editor-core/src/persist/wire.rs`'s header says so at the site).
///
/// **Sealed at the crate boundary.** The fields are private and read
/// back through [`vertices`](Self::vertices) /
/// [`tangent_joints`](Self::tangent_joints), so outside this crate
/// there is no route around those three. Naming the type, reading it,
/// and matching on error payloads all still work; spelling one from a
/// vertex table does not. The seal is a CRATE boundary, not a module
/// one — `crates/profile`'s own internals construct loops directly and
/// stay on the sealed-verbs discipline instead.
///
/// A doctest is a separate crate, so these two blocks are the seal
/// executed at the boundary it claims. A struct literal is a PRIVACY
/// error, not a missing-import one:
///
/// ```compile_fail,E0451
/// use geom_core::Point2;
/// use profile::{ProfileLoop, Segment};
/// let _: ProfileLoop<f64> = ProfileLoop {
///     vertices: Vec::<Point2<f64>>::new(),
///     segments: Vec::<Segment<f64>>::new(),
///     bulges: Vec::new(),
///     tangent_joints: Vec::new(),
/// };
/// ```
///
/// The authoring door compiles, in the same position — and it is the
/// lattice, which classifies each junction as it is authored rather
/// than accepting the table and waiting for [`Profile::validate`]:
///
/// ```
/// use geom_core::{Point2, Tol};
/// use profile::{Open, ProfileLoop, Start};
///
/// let tol = Tol::witness();
/// let square: ProfileLoop<f64> = Open
///     .at(Point2::new(0.0, 0.0))
///     .line_to(Point2::new(1.0, 0.0), tol)?
///     .line_to(Point2::new(1.0, 1.0), tol)?
///     .line_to(Point2::new(0.0, 1.0), tol)?
///     .line_to(Start, tol)?
///     .into();
/// assert_eq!(square.vertices().len(), 4);
/// assert!(square.tangent_joints().is_empty());
/// # Ok::<(), profile::PathError<f64>>(())
/// ```
#[derive(Clone, Debug)]
pub struct ProfileLoop<T: Real> {
    /// The vertices, verbatim, in traversal order (either winding —
    /// winding is invisible, see the crate docs).
    vertices: Vec<Point2<T>>,
    /// The canonical segments, segment `k` leaving vertex `k`.
    segments: Vec<Segment<T>>,
    /// The bulge each segment was lowered from — see
    /// [`ProfileLoop::bulges`].
    bulges: Vec<T>,
    /// Declared-tangent joints, as vertex indices — see
    /// [`ProfileLoop::tangent_joints`] for the normative semantics.
    tangent_joints: Vec<usize>,
}

/// Spells the raw door at a given VISIBILITY, so that the trait ITEM's
/// visibility follows the gate rather than a re-export's.
///
/// **Why a macro and not two `mod` arms.** The seal has to be
/// structural: while the trait was `pub` inside a private module and
/// only its RE-EXPORT was gated, one added line anywhere in the crate —
/// `pub use crate::raw_loop::RawLoop as LoopMint;` inside the carried
/// `path` module — put the minting tier back on every shipped build's
/// surface, and no census row saw it, because a census reads the
/// spellings it was taught. With the item itself `pub(crate)` in the
/// shut arm, that line is a compile error (E0365: a private item cannot
/// be re-exported), and the compiler is checking the invariant instead
/// of a test checking a spelling. Two cfg'd `mod` arms would say the
/// same thing by writing the trait and its impl out twice, and two
/// copies of a body drift; one macro body, instantiated once per arm,
/// cannot.
macro_rules! raw_door {
    ($vis:vis) => {
        /// The raw loop-minting door — **a dev-only fixture door**,
        /// absent from every shipped build (Ev's Q1 ruling half (ii),
        /// in-chat 2026-09-01).
        ///
        /// A [`ProfileLoop`] is a CACHE: the materialized form an
        /// intensional recipe evaluates into, like the kernel's other
        /// recipe→geometry seams. Nothing AUTHORS one from a vertex
        /// table; the ways one comes to exist are listed at
        /// [`ProfileLoop`] itself, and this trait is the fixture one.
        ///
        /// **What "absent" means here, exactly.** In a build that
        /// satisfies neither `test` nor `test-support` this trait is
        /// declared `pub(crate)`. Not "declared public and not
        /// re-exported" — declared crate-private, so no re-export of it
        /// can compile anywhere in this crate and a downstream build
        /// has no route to it at all. `crate::RawLoop` still resolves in
        /// both arms, so the emission layer needs no second spelling; it
        /// is calling the crate's own private constructor.
        ///
        /// Why a trait rather than inherent methods, still: inherent
        /// methods travel with their type, and the type must stay
        /// nameable (read-back, error payloads and
        /// [`ValidatedLoop`](crate::ValidatedLoop) all hand one back).
        /// Trait methods travel with the TRAIT, so gating the trait
        /// gates the authoring tier without touching the type.
        ///
        /// The seal is what makes the door total: [`ProfileLoop`]'s
        /// fields are private, so outside this crate there is no
        /// struct-literal route around it — a downstream
        /// `ProfileLoop { .. }` does not compile (E0451, under test).
        $vis trait RawLoop<T: Real>: Sized {
            /// Builds a loop from a chain in the bulge input form,
            /// lowered to verbatim vertices and canonical segments,
            /// with no declared-tangent joints.
            ///
            /// The one method of this trait that is NOT gated: the
            /// lattice's emission layer calls it as the crate's private
            /// constructor, so it exists in both arms.
            fn new(vertices: Vec<ProfileVertex<T>>) -> Self;

            /// Builds a loop of straight segments through the given
            /// points (all bulges zero) — polygon sugar.
            #[cfg(any(test, feature = "test-support"))]
            fn polygon(points: impl IntoIterator<Item = Point2<T>>) -> Self;

            /// The same loop with its **declared-tangent joints** set
            /// to the given vertex indices (see
            /// [`ProfileLoop::tangent_joints`] for what a declaration
            /// means and how validation verifies it).
            ///
            /// A fixture declares by hand; the [`path`](crate::path)
            /// lattice declares by construction, which is what makes it
            /// the authoring door.
            #[cfg(any(test, feature = "test-support"))]
            fn with_tangent_joints(self, tangent_joints: Vec<usize>) -> Self;
        }

        impl<T: Real> RawLoop<T> for ProfileLoop<T> {
            fn new(vertices: Vec<ProfileVertex<T>>) -> Self {
                Self::lower(&vertices, Vec::new())
            }

            #[cfg(any(test, feature = "test-support"))]
            fn polygon(points: impl IntoIterator<Item = Point2<T>>) -> Self {
                <Self as RawLoop<T>>::new(
                    points
                        .into_iter()
                        .map(|pos| ProfileVertex::new(pos, T::zero()))
                        .collect(),
                )
            }

            #[cfg(any(test, feature = "test-support"))]
            fn with_tangent_joints(mut self, tangent_joints: Vec<usize>) -> Self {
                self.tangent_joints = tangent_joints;
                self
            }
        }
    };
}

// The door's two arms: ONE body, spelled at the visibility its arm
// grants. The shut arm's `pub(crate)` is the seal — see the macro's
// own docs for what it buys over a gated re-export.
#[cfg(any(test, feature = "test-support"))]
raw_door!(pub);
#[cfg(not(any(test, feature = "test-support")))]
raw_door!(pub(crate));

impl<T: Real> ProfileLoop<T> {
    /// **A materialization door**: the same loop read at another
    /// scalar.
    ///
    /// The middle rung of this crate's scalar lift, between
    /// [`ProfileVertex::map`] and [`Profile::map_scalar`], and named by
    /// `geom`'s `scalar_lift` convention: `map` on every leaf
    /// ([`Point2::map`](geom_core::Point2::map),
    /// [`Vec2::map`](geom_core::Vec2::map),
    /// [`Affine3::map`](geom_core::Affine3::map),
    /// [`SketchPlane::map`], [`ProfileVertex::map`] — a fixed tuple of
    /// scalars), `map_scalar` wherever the lift has structure to carry,
    /// which here is the vertex count and the joint index set. One name per operation, on the type it lifts. It
    /// takes `&self` where a leaf takes `self`, because a loop owns its
    /// `Vec`s and its caller holds a borrow.
    ///
    /// This is re-materialization, not authoring. The table already
    /// exists — it was emitted by the lattice, or read back from a
    /// validated profile — and an evaluation at another scalar needs the
    /// same table in that scalar's arithmetic. The positions, the bulges
    /// the segments were lowered from and the declared tangent joints
    /// travel; each segment is DERIVED data, so it is lowered again at
    /// `U` from the mapped endpoints and bulge — its kind by the one
    /// lowering rule (a line exactly at a zero bulge, which `from_f64`
    /// preserves), an arc's carrier and sweep by the arc lowering (at a
    /// certified scalar, that derivation is what mints their
    /// enclosure). The declarations are
    /// re-verified in the evaluation scalar by [`Profile::validate`], so
    /// nothing is taken on trust by crossing.
    ///
    /// With `U::from_f64` — the widening direction, which never refuses
    /// — the crossing is total, and for any `U` whose `from_f64` is
    /// exact on `f64` (`f64` itself included) bit-identical.
    ///
    /// This door and the [`path`] lattice's emission layer are the whole
    /// production population; see [`ProfileLoop`]'s own docs for the two
    /// anticipated doors that do not exist.
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> ProfileLoop<U> {
        let chain: Vec<ProfileVertex<U>> = self.input_chain().map(|v| v.map(&f)).collect();
        ProfileLoop::lower(&chain, self.tangent_joints.clone())
    }

    /// **The lowering** — the one constructor every `ProfileLoop` comes
    /// out of: a chain in the bulge input form becomes verbatim vertices
    /// and one canonical segment per edge ([`ProfileVertex::lower_to`]),
    /// the bulges kept beside them. The emission layer, the fixture
    /// door, [`Self::map_scalar`], [`Self::reversed`] and the lift's
    /// re-seaming all build through it, so every stored segment is the
    /// lowering of its own chord and bulge. The fixture door's
    /// `with_tangent_joints` replaces the joint set of a loop it already
    /// lowered and touches nothing else.
    pub(crate) fn lower(chain: &[ProfileVertex<T>], tangent_joints: Vec<usize>) -> Self {
        let n = chain.len();
        Self {
            vertices: chain.iter().map(|v| v.pos).collect(),
            segments: (0..n)
                .map(|k| chain[k].lower_to(chain[(k + 1) % n].pos))
                .collect(),
            bulges: chain.iter().map(|v| v.bulge).collect(),
            tangent_joints,
        }
    }

    /// The input chain this loop was lowered from: each vertex with the
    /// bulge of its leaving segment.
    pub(crate) fn input_chain(&self) -> impl Iterator<Item = ProfileVertex<T>> + '_ {
        self.vertices
            .iter()
            .zip(&self.bulges)
            .map(|(&pos, &bulge)| ProfileVertex::new(pos, bulge))
    }
}

impl<T: Real> ProfileLoop<T> {
    /// The vertices, verbatim, in traversal order (either winding —
    /// winding is invisible, see the crate docs).
    pub fn vertices(&self) -> &[Point2<T>] {
        &self.vertices
    }

    /// The canonical segments, segment `k` running from vertex `k` to
    /// vertex `k + 1 (mod n)`: a line, or an arc's carrier and signed
    /// sweep.
    ///
    /// A stored `Line` is exactly a segment lowered from a zero bulge,
    /// so a stored `Arc` never comes from b = 0; a stored `Arc` may
    /// still be sub-tolerance, and validation may classify it a line
    /// (see [`Segment`]).
    pub fn segments(&self) -> &[Segment<T>] {
        &self.segments
    }

    /// The bulge each segment was lowered from, segment `k`'s at `k`
    /// (a line's is zero, of either sign).
    ///
    /// **Kept beside the canonical form, not a second description of
    /// it.** Every arc's carrier and sweep are the lowering of its
    /// endpoints and this bulge, so the two cannot disagree. The value
    /// is kept because a derived bulge does not reproduce it —
    /// `tan(Δθ/4)` rounds away from `b` (the literal `1` of a circle
    /// comes back `0.9999999999999999`) — and three kinds of reader
    /// need the value itself: arithmetic written in the bulge (the
    /// sagitta `L·b/2` and the apex it places, which are margins the K
    /// stream records), the lift of a stored segment to another scalar
    /// (which re-derives the carrier from it), and the `geom-brep`
    /// sketch-segment boundary, whose form is still the bulge.
    pub fn bulges(&self) -> &[T] {
        &self.bulges
    }

    /// **Declared-tangent joints** (the #101 discipline): vertex
    /// indices whose *joint* — the junction between the segment
    /// arriving at that vertex and the segment leaving it — is declared
    /// tangent (first-order carrier contact between two *distinct*
    /// carriers).
    ///
    /// Semantics, normative:
    /// - A declaration is **intent, verified — never trusted**:
    ///   validation checks the joint's carrier-tangency margin is
    ///   definite Zero and refuses
    ///   [`ProfileError::TangencyContradicted`] otherwise.
    /// - Conversely, a joint whose carriers *are* definitely tangent
    ///   **must** be declared: undeclared exact tangency is refused as
    ///   [`ProfileError::UndeclaredTangency`] (relying on tangency that
    ///   numerically happens-to-hold is the pattern the boolean door's
    ///   UndeclaredCoincidence retired; lifted here to the profile
    ///   door).
    /// - **Carrier identity is not a reason for anything** (Ev,
    ///   in-chat, 2026-09-02): a zero-turn joint is a tangent joint
    ///   whatever the carriers do, so a declaration on a collinear
    ///   line/line or cocircular arc/arc joint is honoured, not
    ///   contradicted. Identity is a fact about carriers, tangency a
    ///   fact about directions, and this rule reads the directions.
    /// - Duplicate indices are harmless (set semantics); an
    ///   out-of-range index is a typed validation error. Order is not
    ///   significant.
    ///
    /// **This list is `validate`'s question, and it is not the
    /// lattice's.** The lattice checks AUTHORING — declarations
    /// against authored data, at the moment a verb is written — and
    /// `validate` checks the MATERIALIZED TABLE, where
    /// `tangent_joints` is data like any other field. Two questions,
    /// never one rule with two answers; `validate`'s module header
    /// carries the full statement.
    ///
    /// The [`path`] lattice declares by construction (`.fillet(r)`
    /// computes the tangent geometry exactly; the continuation verbs
    /// declare the zero-turn joint they mint), which is what makes it
    /// the authoring door. The fixture door declares by hand instead;
    /// see [`ProfileLoop`]'s own docs for why the two are not
    /// alternatives.
    ///
    /// The fixture door is not linked here on purpose: in a shipped
    /// build it is a crate-private item, and a doc link on the
    /// PRESENTED surface may only name what that surface has.
    ///
    /// [`ProfileError::TangencyContradicted`]: validate::ProfileError::TangencyContradicted
    /// [`ProfileError::UndeclaredTangency`]: validate::ProfileError::UndeclaredTangency
    pub fn tangent_joints(&self) -> &[usize] {
        &self.tangent_joints
    }

    /// The reversed chain: the same locus traversed the other way.
    ///
    /// Reindexing: the reversed chain visits `v0, v(n−1), v(n−2), …,
    /// v1`, and each segment retraces the original segment
    /// `(n−k−1) mod n` backwards with its bulge **negated** (b ↦ −b,
    /// the reversal involution of the crate docs). Negation keeps a zero
    /// bulge zero, so a line stays a line; an arc keeps its carrier
    /// circle and radius, and its sweep changes sign
    /// (4·atan(−b) = −4·atan(b), atan being odd).
    ///
    /// The reversed chain is LOWERED like any other, so an arc's centre
    /// is derived on the reversed chord — whose midpoint `b + (a − b)/2`
    /// can differ from `a + (b − a)/2` in its last bit — rather than
    /// copied from the forward one. That keeps every stored carrier the
    /// lowering of its own chord and bulge, which is what validation
    /// and the scalar lift re-derive.
    ///
    /// `reversed ∘ reversed` is the identity bit-exactly (the
    /// reindexing round-trips, IEEE negation is exact, and the lowering
    /// is a function of the chain) — under test.
    ///
    /// Declared-tangent joints travel with their vertex: joint j maps
    /// to (n − j) mod n, the reversed chain's index of the same
    /// geometric junction (an involution, so the round-trip is exact
    /// elementwise).
    #[must_use]
    pub fn reversed(&self) -> Self {
        let n = self.vertices.len();
        if n == 0 {
            return self.clone();
        }
        let input: Vec<ProfileVertex<T>> = self.input_chain().collect();
        let chain: Vec<ProfileVertex<T>> = (0..n)
            .map(|k| ProfileVertex::new(input[(n - k) % n].pos, -input[(n - k - 1) % n].bulge))
            .collect();
        // Out-of-range indices (garbage data) pass through untouched —
        // total code; validation refuses them typed.
        let tangent_joints = self
            .tangent_joints
            .iter()
            .map(|&j| if j < n { (n - j) % n } else { j })
            .collect();
        Self::lower(&chain, tangent_joints)
    }
}

/// The rigid placement of a sketch plane in 3-space: profile (x, y) ↦
/// `placement`·(x, y, 0) = plane origin + x·u + y·v, where u, v, and the
/// plane normal are the columns of the placement's linear part
/// (`linear.c0`, `linear.c1`, `linear.c2`).
///
/// Rigidity — u, v, normal orthonormal and right-handed — comes from
/// the frame witness rather than from a caller's diligence WHEN the
/// plane came through [`Self::from_frame`], which is the only door
/// that mints one: the placement is then [`OrthoFrame::to_affine`],
/// and those axes were decided where the frame was minted. Tier-3
/// geometric validation certifies the placement at rest.
///
/// [`Self::new`] and the public `placement` field are the other half
/// of the truth and the docs say so plainly: both take and hand back
/// an arbitrary [`Affine3`], so a plane built or overwritten that way
/// is exactly as rigid as whatever produced that map. What the type
/// guarantees is that the MINTING road decides; it is not a proof
/// about every value of the type.
#[derive(Clone, Copy, Debug)]
pub struct SketchPlane<T: Real> {
    /// The placement map. Rigid when [`SketchPlane::from_frame`] built
    /// it; whatever it was assigned otherwise.
    pub placement: Affine3<T>,
}

impl<T: Real> SketchPlane<T> {
    /// **Wraps a placement map already built** — a read-back door, not
    /// a mint: it decides nothing and a skewed [`Affine3`] handed in
    /// comes back out as a skewed plane. A caller holding two authored
    /// directions wants [`Self::from_frame`], which decides them.
    pub fn new(placement: Affine3<T>) -> Self {
        Self { placement }
    }

    /// The world xy-plane: u = x̂, v = ŷ, normal = ẑ, origin at the
    /// world origin — the identity placement, spelled as the frame it
    /// is so all three world planes come from one mint.
    pub fn xy() -> Self {
        Self::from_frame(OrthoFrame::axes_xy(Point3::origin()))
    }

    /// The world yz-plane: u = ŷ, v = ẑ, normal = ŷ × ẑ = x̂, origin at
    /// the world origin.
    ///
    /// The cyclic convention (x→y→z→x) is what the tour's letterforms
    /// captions mean by "a yz sketch extruded +x": sketch (x, y) maps
    /// to world (0, x, y), and the extrusion normal — the third
    /// placement column — is +x̂.
    pub fn yz() -> Self {
        Self::from_frame(OrthoFrame::axes_yz(Point3::origin()))
    }

    /// The world zx-plane: u = ẑ, v = x̂, normal = ẑ × x̂ = ŷ, origin at
    /// the world origin.
    ///
    /// Same cyclic convention as [`Self::yz`] one step further round,
    /// so the captions' "a zx sketch extruded +y" is literal: sketch
    /// (x, y) maps to world (y, 0, x), and the extrusion normal is +ŷ.
    pub fn zx() -> Self {
        Self::from_frame(OrthoFrame::axes_zx(Point3::origin()))
    }

    /// The plane a frame witness places: the placement is
    /// [`OrthoFrame::to_affine`], so `u`, `v` and the normal are the
    /// frame's three axes, in that column order.
    ///
    /// There is no spelling of this door that takes three bare
    /// vectors. A caller holding an authored pair mints the frame
    /// first — [`OrthoFrame::gram_schmidt`] under its own band, or one
    /// of the exact world frames — which is where "these axes are
    /// orthonormal" stops being the caller's obligation. A SKEWED pair
    /// is not refused there: the mint orthonormalizes it, keeping the
    /// first axis and yielding the second. Only a pair that spans no
    /// plane refuses.
    pub fn from_frame(frame: OrthoFrame<T>) -> Self {
        Self::new(frame.to_affine())
    }

    /// The same plane read at another scalar: the stored placement
    /// through [`Affine3::map`] — twelve components, no arithmetic, so
    /// exact whenever `f` is (`Real::from_f64` carries an `f64` frame
    /// to any evaluation scalar).
    ///
    /// **What the lift means.** The stored normal was computed at the
    /// SOURCE scalar — `u × v` at the scalar [`Self::from_frame`] ran
    /// at — and is lifted here as a value. That is the same plane as
    /// the frame constructed at the target scalar exactly where the
    /// cross product's products and differences are exact (the
    /// canonical axes; any frame whose components multiply and
    /// subtract without rounding), and a different one wherever they
    /// round. The two spellings, and a caller chooses:
    ///
    /// - `plane.map(S::from_f64)` — the `f64` frame lifted whole. At
    ///   `Interval` every component is a point interval, the normal
    ///   included: the `f64` rounding of the cross product is carried
    ///   as if exact.
    /// - `SketchPlane::from_frame(frame)` for a frame minted at `S`.
    ///   Bit-identical to the lift on exact axes; at `Interval` on a
    ///   general frame the cross product of point intervals rounds
    ///   outward, so the stored normal carries the width of that
    ///   arithmetic.
    #[must_use]
    pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> SketchPlane<U> {
        SketchPlane::new(self.placement.map(f))
    }

    /// The same plane read at another scalar where the read may
    /// REFUSE: the stored placement through [`Affine3::try_map`] —
    /// twelve components, no arithmetic, the first refusal returned —
    /// and the plane rebuilt around whatever comes back.
    ///
    /// The fallible direction of [`Self::map`], and everything that
    /// method says about WHAT THE LIFT MEANS holds here unchanged: the
    /// stored normal is carried as a value, not recomputed at the
    /// target scalar.
    ///
    /// # Errors
    ///
    /// Whatever `f` refuses with, at the first component it refuses
    /// on.
    pub fn try_map<U: Real, E>(self, f: impl Fn(T) -> Result<U, E>) -> Result<SketchPlane<U>, E> {
        Ok(SketchPlane::new(self.placement.try_map(f)?))
    }

    /// Maps a sketch point to world space: `placement`·(x, y, 0),
    /// exactly [`Affine3::transform_point`] on the embedded point (fixed
    /// order, D9).
    pub fn to_world(&self, p: Point2<T>) -> Point3<T> {
        self.placement
            .transform_point(Point3::new(p.x, p.y, T::zero()))
    }

    /// The plane's origin — sketch (0, 0) in world space.
    ///
    /// The four accessors below READ the frame [`from_frame`] wrote:
    /// they are projections of `placement`, never a recomputation, so
    /// a frame's origin and three axes round-trip through them
    /// BITWISE. That is
    /// why the translation is transcribed component by component
    /// rather than added to the coordinate origin — `0 + (-0) = 0`
    /// would quietly launder a signed zero the stored frame kept.
    ///
    /// [`from_frame`]: Self::from_frame
    pub fn origin(&self) -> Point3<T> {
        let t = self.placement.translation;
        Point3::new(t.x, t.y, t.z)
    }

    /// The plane's u direction — the placement's first linear column,
    /// the world direction sketch +x runs.
    pub fn u(&self) -> Vec3<T> {
        self.placement.linear.c0
    }

    /// The plane's v direction — the placement's second linear column,
    /// the world direction sketch +y runs.
    pub fn v(&self) -> Vec3<T> {
        self.placement.linear.c1
    }

    /// The plane's normal — the placement's third linear column, which
    /// [`from_frame`] filled with u × v. This is the direction an
    /// extrusion of a profile on this plane runs.
    ///
    /// [`from_frame`]: Self::from_frame
    pub fn normal(&self) -> Vec3<T> {
        self.placement.linear.c2
    }
}

impl SketchPlane<f64> {
    /// Bit-exact frame equality (the `Doc::bit_eq` precedent, spec
    /// D7): every one of the twelve stored components compared by
    /// BITS, so `0.0` and `-0.0` are different planes here.
    ///
    /// Bit comparison, not tolerance comparison, is the only equality
    /// this type can honestly offer: a sketch plane carries no ε, and
    /// "the same plane" up to tolerance is a geometric question the
    /// kernel answers about BODIES, at tier 3. Two planes equal here
    /// place every sketch point identically, by construction.
    pub fn bit_eq(&self, other: &Self) -> bool {
        let bits = |p: &Self| {
            // **What holds the twelve complete is the four patterns,
            // not this function's own arithmetic.** A field added to
            // `SketchPlane`, to the `Affine3` it stores, to that map's
            // `Mat3` or to a `Vec3` column is an E0027 here, so a new
            // stored component cannot land outside the comparison
            // quietly. Read straight off `translation` rather than
            // through `Self::origin`, which transcribes exactly those
            // three components and nothing else: same bits, and a
            // pattern where there was a call.
            let Self { placement } = p;
            let Affine3 {
                linear,
                translation,
            } = placement;
            let Mat3 { c0, c1, c2 } = linear;
            [translation, c0, c1, c2].map(|v| {
                let Vec3 { x, y, z } = v;
                [x.to_bits(), y.to_bits(), z.to_bits()]
            })
        };
        bits(self) == bits(other)
    }
}

/// `==` IS [`SketchPlane::bit_eq`] — the comparison this type already
/// means, spelled as the trait so every caller reaches the same
/// answer.
///
/// Only `f64` carries it, because only `f64` has the bit reading
/// [`SketchPlane::bit_eq`] compares; a plane over another [`Real`] has
/// no equality here.
///
/// PARTIAL and no [`Eq`], deliberately: the type carries no hash on
/// either side of the binding boundary, and a plane is a placement to
/// compare, not a key to tally by.
///
/// **The tie to the declaration is inside [`SketchPlane::bit_eq`] and
/// no census can see it from here.** The arrival census over hand-written
/// `PartialEq` and `Debug` impls
/// (`crates/test-utils/tests/hand_written_impl_census.rs`) reads impl
/// bodies as text: this one calls a method, and no text reader can tell
/// a getter or a delegation from any other call without resolving it,
/// so the census records "reads no field" and would record the same
/// whether or not the twelve coordinates one level down were bound by
/// name. Putting a pattern HERE would buy nothing and cost the truth —
/// the reading is in `bit_eq`, so the tie belongs there, beside it.
impl PartialEq for SketchPlane<f64> {
    fn eq(&self, other: &Self) -> bool {
        self.bit_eq(other)
    }
}

/// A sketch profile: closed loops on a sketch plane — the raw input
/// data. [`Profile::validate`] is the only way to make it consumable by
/// sweeps.
#[derive(Clone, Debug)]
pub struct Profile<T: Real> {
    /// The plane the profile lives on (passed through validation
    /// untouched — validation is 2-D).
    pub plane: SketchPlane<T>,
    /// The loops, in any order and either winding (winding is
    /// invisible; nesting is derived by validation).
    pub loops: Vec<ProfileLoop<T>>,
}

impl<T: Real> Profile<T> {
    /// Builds a profile from a plane and loops.
    pub fn new(plane: SketchPlane<T>, loops: Vec<ProfileLoop<T>>) -> Self {
        Self { plane, loops }
    }

    /// **The top rung of this crate's scalar lift**: the same RAW
    /// profile read at another scalar — the plane through
    /// [`SketchPlane::map`], every loop through
    /// [`ProfileLoop::map_scalar`], the loop order carried.
    ///
    /// Structural, not arithmetic, at every rung: each stored scalar
    /// goes through `f` and nothing is computed, so the lift is exact
    /// whenever `f` is and bit-identical for any `U` whose `from_f64`
    /// is exact on `f64`. Nothing here is decided: a raw profile
    /// carries no verdict, and the loops that come out still owe
    /// [`Profile::validate`] at `U`.
    ///
    /// **This is the raw door, and it is rarely the one a build wants.**
    /// Lifting a raw profile and validating the result at `U` decides
    /// the profile's structure a second time, in `U`'s arithmetic, over
    /// data that is an exact embedding of the `f64` data a validation
    /// already ran on — where `U` is certified that second opinion can
    /// only agree or escalate, never disagree. A build that already
    /// holds the `f64` verdict lifts THAT, through
    /// [`ValidatedProfile::lift_onto`], which carries the decisions
    /// instead of remaking them.
    #[must_use]
    pub fn map_scalar<U: Real>(&self, f: impl Fn(T) -> U) -> Profile<U> {
        Profile::new(
            self.plane.map(&f),
            self.loops.iter().map(|lp| lp.map_scalar(&f)).collect(),
        )
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod lowering_tests {
    use super::*;
    use geom_core::{Dual, Dual64, DualInterval, Interval};

    /// The kind `b` lowers to on a unit chord.
    fn kind_of<T: Real>(b: T) -> &'static str {
        let v = ProfileVertex::new(Point2::new(T::zero(), T::zero()), b);
        match v.lower_to(Point2::new(T::one(), T::zero())) {
            Segment::Line => "line",
            Segment::Arc { .. } => "arc",
        }
    }

    /// **The lowering rule, at every lane scalar**: a line exactly at a
    /// bulge of exactly zero — `±0`, and at `Interval` only the
    /// degenerate hull `[−0, 0]` — and an arc for anything else, poison
    /// and every enclosure that merely contains zero included. A dual
    /// is read on its value: its derivative, seeded or not, does not
    /// make a zero bulge an arc.
    #[test]
    fn exact_zero_read_across_scalars() {
        for (b, want) in [
            (0.0, "line"),
            (-0.0, "line"),
            (f64::MIN_POSITIVE / 4.0, "arc"),
            (-1e-300, "arc"),
            (1.0, "arc"),
            (f64::NAN, "arc"),
            (f64::INFINITY, "arc"),
        ] {
            assert_eq!(kind_of(b), want, "f64 {b:e}");
            assert_eq!(kind_of(Dual64::from_f64(b)), want, "Dual {b:e}");
            assert_eq!(kind_of(Dual::variable(b)), want, "seeded Dual {b:e}");
            assert_eq!(kind_of(Dual::new(b, -3.5)), want, "Dual {b:e}, d = −3.5");
            if b.is_finite() {
                let i = Interval::from_f64(b);
                assert_eq!(kind_of(i), want, "Interval {b:e}");
                assert_eq!(
                    kind_of(DualInterval::from_f64(b)),
                    want,
                    "Dual<Interval> {b:e}"
                );
                assert_eq!(
                    kind_of(Dual::variable(i)),
                    want,
                    "seeded Dual<Interval> {b:e}"
                );
            }
        }
        let hull =
            |lo: f64, hi: f64| Interval::hull(Interval::from_f64(lo), Interval::from_f64(hi));
        for (lo, hi, want) in [
            (-0.0, 0.0, "line"),
            (0.0, -0.0, "line"),
            (-0.0, -0.0, "line"),
            (0.0, 1e-300, "arc"),
            (-1e-300, 0.0, "arc"),
            (-1e-300, 1e-300, "arc"),
            (-1.0, 1.0, "arc"),
            (0.5, 2.0, "arc"),
        ] {
            assert_eq!(kind_of(hull(lo, hi)), want, "Interval [{lo:e}, {hi:e}]");
            assert_eq!(
                kind_of(Dual::constant(hull(lo, hi))),
                want,
                "Dual<Interval> [{lo:e}, {hi:e}]"
            );
        }
    }

    /// A loop whose second segment's centre, derived on the reversed
    /// chord, differs from the forward centre in its last bits: the
    /// chord's two endpoints are three orders of magnitude apart in x,
    /// so `a + (b − a)/2` and `b + (a − b)/2` round differently.
    fn discriminating() -> Vec<ProfileVertex<f64>> {
        vec![
            ProfileVertex::new(Point2::new(0.1, 0.3), 0.37),
            ProfileVertex::new(Point2::new(0.000_524_560_164_915_884, 0.7), 0.37),
            ProfileVertex::new(Point2::new(0.039_166_573_353_688_7, 1.9), 0.0),
            ProfileVertex::new(Point2::new(0.9, 1.1), -1.3),
        ]
    }

    /// Asserts that `back` is `lp` reversed segment by segment: each
    /// reversed segment is BIT-identical to a fresh lowering of the
    /// retraced chord with the negated bulge, its sweep is the forward
    /// sweep negated and its radius the forward radius. Returns how many
    /// arcs' centres differ from the forward centre, which a copied
    /// centre would have matched.
    fn check_reversal<T: Real>(lp: &ProfileLoop<T>, back: &ProfileLoop<T>) -> usize {
        let bits = |x: &dyn core::fmt::Debug| format!("{x:?}");
        let n = lp.segments().len();
        let mut moved = 0;
        for k in 0..n {
            let j = (n - k - 1) % n;
            let (a, b) = (back.vertices()[k], back.vertices()[(k + 1) % n]);
            let relowered = ProfileVertex::new(a, -lp.bulges()[j]).lower_to(b);
            assert_eq!(bits(&back.segments()[k]), bits(&relowered), "segment {k}");
            match (lp.segments()[j], back.segments()[k]) {
                (Segment::Line, Segment::Line) => {}
                (
                    Segment::Arc {
                        centre: c,
                        radius: r,
                        sweep: s,
                    },
                    Segment::Arc {
                        centre: cb,
                        radius: rb,
                        sweep: sb,
                    },
                ) => {
                    assert_eq!(bits(&sb), bits(&-s), "segment {k} sweep");
                    assert_eq!(bits(&rb), bits(&r), "segment {k} radius");
                    if bits(&cb) != bits(&c) {
                        moved += 1;
                    }
                }
                (f, r) => panic!("segment {k}: {f:?} reversed to {r:?}"),
            }
        }
        moved
    }

    /// **Reversal lowers the retraced chords again**, at `f64` and at
    /// `Interval`: every reversed segment is the lowering of its own
    /// chord and negated bulge, bit for bit, so a forward centre copied
    /// across would fail on the fixture's discriminating chord.
    #[test]
    fn reversal_negates_the_sweep_and_keeps_the_carrier() {
        let lp = ProfileLoop::lower(&discriminating(), Vec::new());
        let moved = check_reversal(&lp, &lp.reversed());
        assert!(
            moved > 0,
            "the fixture no longer tells a copied centre apart"
        );
        let lp = lp.map_scalar(Interval::from_f64);
        check_reversal(&lp, &lp.reversed());
    }
}

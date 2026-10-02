//! The Hoffmann §6.2 stepper — **untrusted** (M5 PR 7 spec §1, C3).
//!
//! # Nothing here is trusted, and that is the design
//!
//! This module is a *candidate generator*. Its product is a polyline of
//! states (plus frames) handed to the PR 4 fitting stack; the **C2
//! certificate is the only gate** ([`super::certify`]). A branch jump
//! is not something a step predicate catches — no local datum can prove
//! the global property "no other branch within reach" — it is a
//! certificate refusal: the uniqueness tube fails to be a graph, or
//! transversality dies, and the refusal is typed. C3 inverts the
//! planning question on purpose.
//!
//! # The stepper (realized)
//!
//! At the current state `x`, with `A` the [`LocalSystem`] Jacobian:
//!
//! 1. `Svd<M, N>` of `A`. The null direction is the unit tangent `d₁`,
//!    oriented against the previous step; `σ_min` is Hoffmann's
//!    transversality signal (p. 217), reported as diagnostic.
//! 2. **`ssi_transversality`**: the *decision* uses the dimensionally
//!    honest form `sin θ · arm` in meters — θ the angle between the two
//!    surface normals, `arm` the folded curvature/extent lever arm —
//!    exactly `dihedral_wedge`'s shape (D4 ¶1). `Sign::Zero` is the
//!    **sliver band**: refuse toward C7 (`TangentIntersection`, PR 9).
//!    We never desingularize; Hoffmann §6.5's quadratic-transformation
//!    tracing through singular points is deliberately not adopted.
//! 3. `A·d₂ = b₂` solved **minimum-norm** — which *is* Frenet's γ₂ = 0
//!    (see [`geom_core::linalg::svd`]'s module docs for the one-line
//!    derivation from unit speed).
//! 4. `A·d₃ = b₃` minimum-norm, then `d₃ ← d₃ − κ²·d₁` with
//!    `κ = ‖d₂‖` — Frenet's γ₃ = −κ².
//! 5. **Step size by the small-contribution heuristic** (p. 215): the
//!    quadratic and cubic terms of the approximant may each deviate by
//!    at most [`SSI_STEP_DEVIATION`]·ε in meters, giving
//!    `h ≤ √(2δ/κ)` and `h ≤ ∛(6δ/‖d₃‖)`; clamped into
//!    `[h_min, h_max]`. **`ssi_step_progress`** refuses if the step
//!    collapses into the band — a stepper that cannot move at this
//!    tolerance says so.
//! 6. Advance by the cubic approximant, then **Newton refinement** to
//!    the surface pair: a fixed [`SSI_NEWTON_ITERS`] cap of
//!    minimum-norm corrections, early-exiting on
//!    [`SSI_NEWTON_TOL`]·ε, or the coordinates' noise where that is larger
//!    (`Readout`; an f64 structure branch, C6 lane).
//!
//! # The idealized stepper (the differential spec, T4/PERF-PLAN §4.4)
//!
//! [`StepperMode::Idealized`] replaces steps 3–5 with a **tangent-line
//! step of fixed tiny length** [`SSI_IDEALIZED_STEP`] — ten readable
//! lines that *define* the traced locus. Everything else (the
//! transversality band, Newton refinement, the closure trio, the
//! domain clipping) is shared, so the differential suite compares the
//! two steppers and nothing else.
//!
//! **What the differential pin can and cannot be.** PERF-PLAN §4.2's
//! usual pin is byte-equality; it is not available here and pretending
//! otherwise would be the dishonest option. The two steppers place
//! *different numbers of samples at different arc lengths* by
//! construction — that is the entire difference between them — so no
//! output object is common to both. The honest executable pin, which
//! the suite uses, is: identical **branch topology** (count, closure
//! verdict, end kind per branch) and every idealized sample lying
//! within the realized branch's own **certified** residual band of the
//! realized carrier. A divergence in either is a definite bug in the
//! realized stepper, which is what §4.3's correlated-error argument
//! asks the pin to catch.
//!
//! # Closure and loop topology are margined decisions
//!
//! Three named Q1 trileans, never raw comparisons (C3):
//!
//! - **`ssi_closure_return`** — "the trace returned to its start": the
//!   margin is `h − ‖x − x₀‖` in meters, so closure means *the next
//!   step would step over the seed*. Armed only after the trace has
//!   left a three-step neighborhood of its seed (a structural latch,
//!   not a decision).
//! - **`ssi_closure_tangent`** — "this branch closed, it did not merely
//!   arrive somewhere": the margin is `cos φ · arc`, `φ` the angle
//!   between the returning tangent and the seed tangent, `arc` the
//!   branch's own arc length as the lever arm. `Positive` is the
//!   affirmative (the trace came back running the same way it left);
//!   `Zero` means the two directions are perpendicular to within the
//!   band and `Negative` means it came back *reversed* — a cusp or a
//!   retrace, either way not a loop, and both refuse.
//!
//!   The margin is deliberately **not** `sin φ · arc`. A discretely
//!   marched loop closes at a point a step away from its seed, so its
//!   returning tangent differs from the seed tangent by `O(hκ)` —
//!   metering *that* against ε would demand nanoradian agreement from a
//!   stepper the design explicitly does not trust, and every honest
//!   loop would refuse. What this predicate is for is telling a closure
//!   apart from a *crossing*, and that distinction lives at `O(1)` in
//!   the angle, which is exactly where `cos φ` reads it. The
//!   fine-grained question — "is there really only one arc here?" — is
//!   not a stepper question at all: it is limb 3's, where a genuine
//!   self-crossing makes the uniqueness tube's enclosure straddle.
//!
//! # How a branch ends, by lane
//!
//! The two lanes end an open branch differently, and the type says
//! which: [`march`] takes an [`Exit`], and each lane has its own.
//!
//! - **The plane × NURBS lane ([`RectExit`])** decides its domain
//!   boundary before any march (`super::boundary`): the crossings of the
//!   locus with the wall's knot rectangle are known, certified, and are
//!   the only ends an open branch has. The march leaving the rectangle
//!   is a structure test on the chart coordinates, not a decision: the
//!   trace stops at its first state outside and hands back that step,
//!   and the caller matches it to the crossing on the side it left.
//! - **The ℝ³ lane ([`SlabExit`])** ends an open branch at the caller's
//!   slab by its boundary search. **`ssi_branch_open_end`** — "the
//!   branch ends on the slab": the margin is the signed distance to the
//!   slab in meters. `Negative` ends the branch open; `Zero` also ends
//!   it and labels the end in band ([`BranchEnd::SlabInBand`]). The
//!   label is a report, not a mechanism: a region no tube covers is
//!   refined by the accounting pass and refuses typed at the floor.
//!   The slab is the caller's box, not geometry
//!   (`work/ssi/ssi-r3-slab-is-not-geometry.md`).

use geom_core::linalg::svd::Svd;
use geom_core::{Band, Margin, Point3, Real, Sign, SupSpeed, Vec3};

use crate::dihedral::{decide, decide_positive, decide_reported};
use crate::recourse::Refused;

use super::enclose::Box3;
use super::exhaust::{self, ExhaustLane, UvRect};
use super::system::LocalSystem;
use super::{SSI_FIT_DEGREE, SsiError, TraceDecision};

/// The **candidate generator's** step tolerance, in meters.
///
/// The marcher is deliberately untrusted: it runs in `f64`, it makes no
/// trilean decision, and every number it produces is re-derived by the
/// certificate through the run's [`Band`] before anything is claimed.
/// [`SSI_NEWTON_TOL`] and [`SSI_STEP_DEVIATION`] scale this value into
/// that generator's convergence tolerance and between-sample deviation
/// budget.
///
/// It is a distinct type — not a bare `f64` beside the `Band` — because
/// the two are the *same quantity* on every door that certifies, and a
/// second `f64` copy of the run tolerance is exactly the shape that
/// lets a caller march at one tolerance and certify at another.
///
/// **The type does not leave this crate**: no public door takes or
/// returns one, and the uncertified trace — the only door that names
/// the generator's tolerance at all — takes a bare `f64`. A tolerance
/// that is not the run band's is mintable only inside `ssi`
/// ([`MarchTol::decoupled`]), and only that one door reaches for it;
/// every certifying door derives its own from the run band.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MarchTol {
    /// The tolerance, in metres.
    meters: f64,
    /// The residual Newton settles every state to, in metres.
    settle: f64,
}

/// **How finely a march's residual can be read**, where its states
/// reach furthest: the spacing of adjacent floats there, and the noise
/// the lane's evaluation carries on top of it, in ulps of that spacing.
/// A Newton target below that noise is one no state reliably reaches,
/// so [`MarchTol`] settles to whichever is larger.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Readout {
    /// Whose coordinates bind: the ℝ³ box the residual is evaluated in,
    /// or a NURBS operand's chart Newton moves through.
    lane: ExhaustLane,
    /// Their largest magnitude, in the lane's units.
    reach: f64,
    /// The spacing of adjacent floats there, in the lane's units.
    gap: f64,
    /// What bounds the reach.
    bound: ReachBound,
    /// The residual noise, in metres.
    noise: f64,
}

/// What bounds how far a march's states reach from zero
/// ([`SettlingRefusal::bound`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReachBound {
    /// The operands: the states lie on them, nearer zero than the
    /// domain's far side.
    Geometry,
    /// The caller's domain: no operand box bounds the states nearer.
    Domain,
}

impl Readout {
    /// The residual evaluated at coordinates in the caller's `domain`,
    /// cut down to the box the `operands` lie in where one bounds them,
    /// with `ulps` of their spacing as noise.
    pub(crate) fn spatial(domain: Box3, operands: Option<Box3>, ulps: f64) -> Self {
        let (whole, _) = exhaust::coordinate_gap(domain);
        let (reach, gap) = exhaust::coordinate_gap(operands.map_or(domain, |o| domain.meet(o)));
        let bound = if reach < whole {
            ReachBound::Geometry
        } else {
            ReachBound::Domain
        };
        Self {
            lane: ExhaustLane::R3,
            reach,
            gap,
            bound,
            noise: ulps * gap,
        }
    }

    /// The louder of this readout and a NURBS operand's chart `root`,
    /// whose parameter spacing crosses into metres at `speed`, with
    /// `ulps` of it as noise.
    pub(crate) fn or_chart(self, root: UvRect, speed: SupSpeed<f64>, ulps: f64) -> Self {
        let (reach, gap) = exhaust::coordinate_gap(root);
        let noise = ulps * speed.to_meters(gap);
        if noise <= self.noise {
            return self;
        }
        Self {
            lane: ExhaustLane::Chart { speed },
            reach,
            gap,
            bound: ReachBound::Geometry,
            noise,
        }
    }
}

/// Why the march cannot settle its states finely enough for the run's
/// tolerance ([`SsiError::SettlingUnresolvable`]): the coordinates the
/// residual is read in are too coarse, where they reach furthest, for a
/// settled state to sit well inside ε.
#[derive(Clone, Copy, Debug)]
pub struct SettlingRefusal {
    /// Whose coordinates bind, and on the chart lane the rate their
    /// spacing crossed into metres by.
    pub lane: ExhaustLane,
    /// Their largest magnitude, in the lane's units.
    pub reach: f64,
    /// The spacing of adjacent floats there, in the lane's units.
    pub gap: f64,
    /// What bounds the reach: on the chart lane, always the geometry.
    pub bound: ReachBound,
    /// The residual the march can settle to there, in metres.
    pub settle: f64,
    /// The run's tolerance, in metres.
    pub tolerance: f64,
}

impl MarchTol {
    /// The generator's tolerance derived from the run's band — the run
    /// tolerance ε itself, since a linear [`Band`]'s `zero()` **is** ε,
    /// stored unmodified by `Band::new`.
    ///
    /// A tolerance built any other way cannot cross the certifying
    /// seam — the finishers refuse with
    /// [`SsiError::MarchTolMismatch`] — so the marcher's spacing, the
    /// accounting floor and the certificate's floors are one number by
    /// enforcement rather than by intent.
    ///
    /// # Errors
    ///
    /// [`SsiError::SettlingUnresolvable`] as [`MarchTol::settling`].
    pub(crate) fn from_band(band: Band, readout: Readout) -> Result<Self, SsiError> {
        Self::mint(band.zero(), readout)
    }

    /// Whether this is the run band's own tolerance.
    #[must_use]
    pub(crate) fn is_of(self, band: Band) -> bool {
        self.meters == band.zero()
    }

    /// The tolerance, settling to the larger of [`SSI_NEWTON_TOL`] of it
    /// and the `readout`'s noise, refused where that is not within
    /// [`SSI_SETTLE_MAX`] of it.
    fn mint(meters: f64, readout: Readout) -> Result<Self, SsiError> {
        let settle = Real::max(SSI_NEWTON_TOL * meters, readout.noise);
        if settle.is_nan() || settle > SSI_SETTLE_MAX * meters {
            return Err(SsiError::SettlingUnresolvable(SettlingRefusal {
                lane: readout.lane,
                reach: readout.reach,
                gap: readout.gap,
                bound: readout.bound,
                settle,
                tolerance: meters,
            }));
        }
        Ok(Self { meters, settle })
    }

    /// The residual, in metres, Newton refinement settles every state
    /// to.
    #[must_use]
    pub(crate) fn settling(self) -> f64 {
        self.settle
    }

    /// A generator tolerance **deliberately decoupled** from the run
    /// band, for the door that returns no certificate.
    ///
    /// Private to `ssi` on purpose: a `pub` version of this is the
    /// second ε entering under a nicer name, and a maintainer editing a
    /// certifying door is the caller who would reach for it. The only
    /// legitimate use is measuring the marcher against itself — how the
    /// fitted pair's deviation scales as the generator is tightened,
    /// independently of the ambient tolerance the run is banded at.
    ///
    /// A decoupled march is **not** a hole in the certificate. Every
    /// floor and every trilean downstream is stated in the `Band`, so a
    /// generator run at some other tolerance can only produce a *worse
    /// carrier*, which then certifies honestly at the band or refuses
    /// on one of the three limbs.
    ///
    /// # Errors
    ///
    /// [`SsiError::InvalidMarchTol`] when `meters` is not finite and
    /// strictly positive — a typed refusal, never a silent clamp — and
    /// [`MarchTol::from_band`]'s refusal.
    pub(super) fn decoupled(meters: f64, readout: Readout) -> Result<Self, SsiError> {
        if !(meters.is_finite() && meters > 0.0) {
            return Err(SsiError::InvalidMarchTol { value: meters });
        }
        Self::mint(meters, readout)
    }

    /// The tolerance in meters — the `f64` the untrusted lane consumes.
    #[must_use]
    pub(crate) fn meters(self) -> f64 {
        self.meters
    }
}

/// Fixed Newton-refinement cap per step (D9 — never data-dependent).
pub const SSI_NEWTON_ITERS: usize = 8;

/// Newton's early-exit residual, as a fraction of ε: refinement stops
/// once the state satisfies both surfaces two orders inside tolerance,
/// where the coordinates resolve that (`Readout`).
/// A structure branch on `f64` (C6's lane), not a predicate.
pub const SSI_NEWTON_TOL: f64 = 1.0e-2;

/// The coarsest residual, as a fraction of ε, a march may settle its
/// states to. The certificate bounds the fitted carrier against ε, so
/// samples settled much coarser than this leave it no room: measured on
/// the threaded cylinder × sphere at ε = 1e-9, states settled to 0.47ε
/// certify and states settled to 0.93ε escalate limb 2; the plane ×
/// wall at ε = 1e-12 reads the same (0.45ε certifies, 0.91ε escalates).
/// The headroom below it is the fixture's, not a bound: a rational wall
/// (weights 1, 1.05, 0.97, 1) at 500 m and ε = 1e-12, settled to 0.45ε,
/// reads limb 2 at 0.92ε, 8% inside the band.
pub const SSI_SETTLE_MAX: f64 = 0.5;

/// A quadric pair's residual noise, in ulps of its coordinates: twice
/// the measured need. The threaded cylinder × sphere settles at every
/// reach to 1e6 m at ε = 1e-9 with one ulp.
pub const SSI_QUADRIC_NOISE_ULPS: f64 = 2.0;

/// A spline pair's residual noise, in ulps of its coordinates. The
/// plane × certifiable wall of `tests/m5_pr7_ssi.rs` at 20–1000 m and
/// ε = 1e-12 loses its branch settling to two ulps and settles at four;
/// across further walls the need measured 4 to 6 ulps, so this is 1.3
/// to 2 times the measured need, not a bound.
pub const SSI_SPLINE_NOISE_ULPS: f64 = 8.0;

/// Hoffmann's "keep the higher contributions small" (p. 215) as a named
/// constant: the quadratic and cubic terms of the approximant may each
/// reach this fraction of the **linear** term. Dimensionless, because
/// the heuristic is about the local model staying a good model, not
/// about tolerance.
pub const SSI_STEP_RELATIVE: f64 = 0.1;

/// The share of ε the *between-sample* error of the eventual cubic fit
/// may consume — the second, separately-named step cap.
///
/// Hoffmann's relative heuristic alone is not enough here, and saying
/// why matters: it keeps the *approximant* honest, but our samples are
/// then handed to a cubic fitting stack whose product must be within ε
/// of the locus **between** them (C2.2). The standard interpolation
/// bound `‖C − fit‖ ≲ h⁴·‖C⁗‖/384` with `‖C⁗‖ ≈ κ³` for a curve of
/// slowly-varying curvature turns that into a cap on `h`, which is the
/// one that actually binds on a small tight loop. Both caps are
/// applied; the step is the smaller.
///
/// Two caveats keep this a *design target* rather than a bound, and
/// the certificate — never this constant — is what refuses when the
/// target is missed. First, `‖C⁗‖ ≈ κ³` holds for slowly-varying
/// curvature and understates a curve whose curvature swings. Second,
/// limb 2's control-hull enclosure is conservative over the true
/// deviation by a factor measured at roughly 4–20× on the M5 fixtures.
/// The value is therefore set well below ε rather than at it.
pub const SSI_STEP_DEVIATION: f64 = 0.02;

/// The idealized stepper's fixed step, as a fraction of the caller's
/// named extent, and never longer than the march's step cap
/// (`march_both`). Tiny by construction: the tangent-line step's own
/// truncation is `O(h²κ)`, so at a thousandth of the feature extent it
/// is far below ε on anything with sane curvature, and Newton removes
/// what remains.
pub const SSI_IDEALIZED_STEP: f64 = 1.0e-3;

/// The realized stepper's largest step, as a fraction of the caller's
/// named extent — a cap so a nearly-straight branch still gets sampled
/// densely enough for the fit to have data. A branch too short for the
/// cubic fit at that cap is marched once more with its own length cut
/// into an odd number of steps (`march_both`).
pub const SSI_STEP_MAX: f64 = 1.0 / 32.0;

/// Which stepper — the realized third-order approximant or the
/// idealized tangent-line spec (module docs, PERF-PLAN §4.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepperMode {
    /// The Hoffmann third-order Frenet approximant with adaptive step.
    Realized,
    /// Tangent-line steps of fixed tiny length — the differential spec.
    Idealized,
}

impl StepperMode {
    /// The mode's display name (for typed refusals).
    pub fn name(self) -> &'static str {
        match self {
            Self::Realized => "realized (third-order Frenet approximant)",
            Self::Idealized => "idealized (tangent-line, fixed tiny step)",
        }
    }
}

/// What is wrong with a step the stepper minted
/// ([`SsiError::StepUnusable`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepFault {
    /// The march speed is not positive and finite, so it converts no
    /// step into metres.
    SpeedUnusable,
    /// The step is not finite in some coordinate of the state.
    NotFinite,
    /// The step is below the resolution of the state's coordinates:
    /// adding it leaves the state unchanged.
    DoesNotMove,
}

/// Which rungs held a march's steps short ([`SsiError::StepBudget`]):
/// the cap a caller's knobs set, the curvature, or both. A rung holding
/// fewer than [`STEP_BOUND_MINORITY`] of the steps is not named.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepBound {
    /// The curvature rungs. The fit's between-sample rung shrinks with
    /// ε, so loosening the tolerance lengthens it exactly. The relative
    /// heuristic's two rungs do not read ε at all; they bind only where
    /// the radius of curvature is within a few hundred tolerances or
    /// the torsion is extreme, and there the tolerance is an
    /// approximate lever. No extent lengthens any of them.
    Curvature,
    /// The cap: a fraction of the feature extent, or the domain's
    /// diagonal.
    Cap,
    /// Each held at least [`STEP_BOUND_MINORITY`] of the steps.
    Both,
}

/// The share of a march's steps a rung must hold to be named in
/// [`StepBound`]: a quarter.
pub const STEP_BOUND_MINORITY: (usize, usize) = (1, 4);

impl StepBound {
    /// The rungs `curvature` of `steps` steps name.
    fn of(curvature: usize, steps: usize) -> Self {
        let (num, den) = STEP_BOUND_MINORITY;
        let named = |n: usize| n * den >= steps * num;
        match (named(curvature), named(steps - curvature)) {
            (true, true) => Self::Both,
            (true, false) => Self::Curvature,
            (false, _) => Self::Cap,
        }
    }
}

/// How a certified branch ends.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BranchEnd {
    /// The trace returned to its seed on a matching tangent — a loop.
    Closed,
    /// The plane × NURBS lane: the branch runs between two crossings of
    /// the locus with the wall's knot rectangle, both certified by the
    /// boundary pass before any march.
    Crossings {
        /// The crossing it starts at.
        from: BoundaryPoint,
        /// The crossing it ends at.
        to: BoundaryPoint,
    },
    /// The ℝ³ lane: the trace left the caller's slab (definitely).
    Slab,
    /// The ℝ³ lane: the trace reached the slab **in band**: the end is
    /// within ε of the slab and the honest report is that we do not
    /// know which side it fell on. A label, not a mechanism: what
    /// happens to the region past such an end is what happens to any
    /// region no tube covers.
    SlabInBand,
}

pub use super::boundary::BoundaryPoint;

/// One traced branch: the state polyline plus how its march ended.
#[derive(Clone, Debug)]
pub struct Trace<const N: usize, E> {
    /// The states, in march order, seed first.
    pub states: Vec<[f64; N]>,
    /// How the march ended, in its lane's terms.
    pub end: E,
    /// The smallest `sin θ · arm` (meters) seen along the trace — the
    /// transversality headroom, reported so a consumer can see how
    /// close to the C7 regime this branch ran.
    pub min_transversality: f64,
    /// The smallest σ_min seen — Hoffmann's own signal, diagnostic.
    pub min_sigma: f64,
    /// Steps consumed.
    pub steps: usize,
}

/// How a march ends an open branch: one implementation per lane, so a
/// lane reaches only its own ending (module docs).
pub(crate) trait Exit<const M: usize, const N: usize, S: LocalSystem<M, N>> {
    /// The lane's end of a march.
    type End: Copy;
    /// The end of a trace that closed on its seed.
    const CLOSED: Self::End;
    /// Whether the refined seed may be marched. `Err` refuses it.
    ///
    /// # Errors
    ///
    /// The lane's refusal of a seed outside its domain.
    fn seed(
        &self,
        sys: &S,
        x: &[f64; N],
        ctx: &MarchContext<N>,
        band: Band,
    ) -> Result<(), SsiError>;
    /// The end of a march whose step from `inside` to `next` (refined)
    /// leaves the domain, or `None` to march on. May push the branch's
    /// last state.
    ///
    /// # Errors
    ///
    /// The lane's escalation of an undecided end.
    fn after_step(
        &self,
        sys: &S,
        states: &mut Vec<[f64; N]>,
        inside: [f64; N],
        next: [f64; N],
        ctx: &MarchContext<N>,
        band: Band,
    ) -> Result<Option<Self::End>, SsiError>;
    /// The end of a march whose predicted step to `predicted` would not
    /// settle back onto the locus, or `None` when that is the march
    /// losing its branch.
    fn unsettled(
        &self,
        inside: [f64; N],
        predicted: [f64; N],
        ctx: &MarchContext<N>,
    ) -> Option<Self::End>;
}

/// The plane × NURBS lane's exit: the wall's knot rectangle, the
/// state's coordinates 2 and 3, left by a structure test (module docs).
pub(crate) struct RectExit;

/// How a plane × NURBS march ended.
#[derive(Clone, Copy, Debug)]
pub(crate) enum RectEnd {
    /// It returned to its seed.
    Closed,
    /// Its step from `inside` to `outside` left the wall's rectangle.
    Left {
        /// The last state inside.
        inside: [f64; 4],
        /// The state the step reached, refined onto the locus where it
        /// settled, outside the rectangle.
        outside: [f64; 4],
    },
}

impl RectExit {
    /// Whether a state's chart coordinates lie in the wall's rectangle
    /// (a structure test on the raw coordinates, C6's lane).
    pub(crate) fn inside(x: &[f64; 4], ctx: &MarchContext<4>) -> bool {
        (2..4).all(|i| x[i] >= ctx.domain[i][0] && x[i] <= ctx.domain[i][1])
    }
}

impl<S: LocalSystem<3, 4>> Exit<3, 4, S> for RectExit {
    type End = RectEnd;
    const CLOSED: RectEnd = RectEnd::Closed;

    /// A seed is marched from inside the rectangle only. One that
    /// settles outside is no state to march from, and the branch it
    /// lies on reaches the wall's boundary, where the boundary pass has
    /// already traced it from its crossings.
    fn seed(
        &self,
        sys: &S,
        x: &[f64; 4],
        ctx: &MarchContext<4>,
        _band: Band,
    ) -> Result<(), SsiError> {
        if Self::inside(x, ctx) {
            Ok(())
        } else {
            Err(SsiError::SeedOffDomain {
                mode: StepperMode::Realized.name(),
                margin: domain_margin(x, ctx, sys, x),
            })
        }
    }

    fn after_step(
        &self,
        _sys: &S,
        _states: &mut Vec<[f64; 4]>,
        inside: [f64; 4],
        next: [f64; 4],
        ctx: &MarchContext<4>,
        _band: Band,
    ) -> Result<Option<RectEnd>, SsiError> {
        Ok((!Self::inside(&next, ctx)).then_some(RectEnd::Left {
            inside,
            outside: next,
        }))
    }

    fn unsettled(
        &self,
        inside: [f64; 4],
        predicted: [f64; 4],
        ctx: &MarchContext<4>,
    ) -> Option<RectEnd> {
        (!Self::inside(&predicted, ctx)).then_some(RectEnd::Left {
            inside,
            outside: predicted,
        })
    }
}

/// The ℝ³ lane's exit: the caller's slab, by the open-end decision and
/// the boundary search (module docs).
pub(crate) struct SlabExit;

/// How an ℝ³ march ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SlabEnd {
    /// It returned to its seed.
    Closed,
    /// It left the slab, definitely.
    Slab,
    /// It reached the slab in band.
    SlabInBand,
}

impl SlabEnd {
    /// The branch end this march end is.
    pub(crate) fn branch_end(self) -> BranchEnd {
        match self {
            Self::Closed => BranchEnd::Closed,
            Self::Slab => BranchEnd::Slab,
            Self::SlabInBand => BranchEnd::SlabInBand,
        }
    }
}

impl<S: LocalSystem<2, 3>> Exit<2, 3, S> for SlabExit {
    type End = SlabEnd;
    const CLOSED: SlabEnd = SlabEnd::Closed;

    fn seed(
        &self,
        _sys: &S,
        _x: &[f64; 3],
        _ctx: &MarchContext<3>,
        _band: Band,
    ) -> Result<(), SsiError> {
        Ok(())
    }

    fn after_step(
        &self,
        sys: &S,
        states: &mut Vec<[f64; 3]>,
        inside: [f64; 3],
        next: [f64; 3],
        ctx: &MarchContext<3>,
        band: Band,
    ) -> Result<Option<SlabEnd>, SsiError> {
        let margin = domain_margin(&next, ctx, sys, &inside);
        match decide("ssi_branch_open_end", Margin::of(margin), band) {
            Ok(Sign::Positive) => Ok(None),
            Ok(Sign::Zero) => {
                push_boundary(sys, states, inside, next, ctx);
                Ok(Some(SlabEnd::SlabInBand))
            }
            Ok(Sign::Negative) => {
                push_boundary(sys, states, inside, next, ctx);
                Ok(Some(SlabEnd::Slab))
            }
            Err(diag) => Err(TraceDecision::BranchOpenEnd.escalated(diag)),
        }
    }

    fn unsettled(
        &self,
        _inside: [f64; 3],
        _predicted: [f64; 3],
        _ctx: &MarchContext<3>,
    ) -> Option<SlabEnd> {
        None
    }
}

/// Everything the stepper needs that is not the system: the named
/// domain, the lever arms, and the budgets.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MarchContext<const N: usize> {
    /// The domain box in state coordinates, `[lo, hi]` per coordinate.
    pub domain: [[f64; 2]; N],
    /// The caller's named feature extent, in meters — the lever arm of
    /// last resort and the scale of the idealized step. The realized
    /// step's cap is [`march`]'s `step_cap`.
    pub extent: f64,
    /// The candidate generator's step tolerance. Derived from the run
    /// band on every certifying door — see [`MarchTol`].
    pub tol: MarchTol,
    /// Step budget.
    pub max_steps: usize,
}

impl<const N: usize> MarchContext<N> {
    /// The domain box's diagonal, in state units: the longest step the
    /// stepper takes. The trace ends at its first exit, so a longer step
    /// buys nothing, and it costs the trace twice. Landed far outside the
    /// domain, the step may not settle back onto the locus at all
    /// (Newton fails, and the march loses its branch), or it puts the
    /// crossing past what the boundary search's fixed bisections
    /// resolve, so the trace ends at the state it left.
    fn diagonal(&self) -> f64 {
        self.domain
            .iter()
            .map(|[lo, hi]| (hi - lo) * (hi - lo))
            .sum::<f64>()
            .sqrt()
    }
}

/// The unit normal of the locus's first surface at a state, and of the
/// second — the pair whose angle is the transversality margin.
pub(crate) type NormalPair = (Vec3<f64>, Vec3<f64>);

/// A system that can also report the two surface normals and the
/// curvature lever arm at a state — everything the *decisions* need,
/// separated from the algebra the stepper needs so the trilean's
/// margin stays dimensionally honest (meters) at both trace shapes.
pub(crate) trait TransversalityData<const N: usize> {
    /// Unit normals of the two operands at the state.
    fn normals(&self, x: &[f64; N]) -> NormalPair;
    /// The folded curvature lever arm at the state, in meters
    /// (`f64::MAX` where no curvature bounds it — the plane identity;
    /// the march clamps it to the run's extent). A poisoned operand
    /// makes the arm poison, so the arm guard escalates rather than
    /// levering against the sibling's.
    fn lever_arm(&self, x: &[f64; N]) -> f64;
}

/// March one branch from `seed` (module docs), no step longer than
/// `step_cap` meters.
///
/// # Errors
///
/// [`SsiError::TransversalityBand`] in the σ₂ sliver band (toward C7),
/// [`SsiError::StepCollapsed`], [`SsiError::StepBudget`],
/// [`SsiError::SelfCrossingLocus`], [`SsiError::Escalated`] for any
/// other in-band trilean, [`SsiError::StepUnusable`] when the step
/// minted from the march speed cannot be taken,
/// [`SsiError::SeedRefinementFailed`] when the seed will not settle
/// onto the locus, and [`SsiError::StepRefinementFailed`] when a step
/// from the locus will not settle back onto it.
#[allow(clippy::too_many_arguments)]
pub(crate) fn march<const M: usize, const N: usize, S, E>(
    sys: &S,
    exit: &E,
    seed: [f64; N],
    ctx: MarchContext<N>,
    mode: StepperMode,
    direction: f64,
    band: Band,
    step_cap: f64,
) -> Result<Trace<N, E::End>, SsiError>
where
    S: LocalSystem<M, N> + TransversalityData<N>,
    E: Exit<M, N, S>,
{
    let mut x = newton_refine(sys, seed, ctx.tol)
        .ok_or(SsiError::SeedRefinementFailed { mode: mode.name() })?;
    exit.seed(sys, &x, &ctx, band)?;
    let seed_state = x;
    let mut states = vec![x];
    let mut prev_tangent: Option<[f64; N]> = None;
    let mut seed_tangent: Option<[f64; N]> = None;
    let mut min_transversality = f64::INFINITY;
    let mut min_sigma = f64::INFINITY;
    let mut left_start = false;
    let mut steps = 0usize;
    let mut curvature_bound = 0usize;

    while steps < ctx.max_steps {
        // ---- 1. the local decomposition ----
        let a = sys.jacobian(&x);
        let svd = Svd::<M, N>::new(a);
        let sigma = svd.sigma_min();
        if sigma < min_sigma {
            min_sigma = sigma;
        }

        // ---- 2. ssi_transversality (the σ₂ sliver band ⇒ C7) ----
        let (n1, n2) = sys.normals(&x);
        let sin_theta = n1.cross(n2).norm() / (n1.norm() * n2.norm());
        let arm = Real::min(sys.lever_arm(&x), ctx.extent);
        decide_positive("ssi_transversality_arm", Margin::of(arm), band)
            .map_err(|cause| TraceDecision::TransversalityArm.escalated(cause))?;
        let transversality = Margin::levered(sin_theta, arm);
        if transversality.value() < min_transversality {
            min_transversality = transversality.value();
        }
        // Zero is the sliver band: a tangential (or in-band tangential)
        // contact along the candidate locus, C7's regime. `sin θ · arm`
        // is a magnitude, so a definite negative cannot arise.
        match decide_reported("ssi_transversality", transversality, band) {
            Ok(decided) => {
                if let Some(verdict) = Refused::of(decided, band) {
                    return Err(SsiError::TransversalityBand {
                        sin_theta,
                        arm,
                        sigma_min: sigma,
                        verdict,
                    });
                }
            }
            Err(diag) => return Err(TraceDecision::Transversality.escalated(diag)),
        }

        // ---- 3. the tangent, oriented along the march ----
        let mut d1 = svd.null_direction();
        if let Some(prev) = prev_tangent {
            if dot(&d1, &prev) < 0.0 {
                d1 = neg(&d1);
            }
        } else {
            if direction < 0.0 {
                d1 = neg(&d1);
            }
            seed_tangent = Some(d1);
        }
        prev_tangent = Some(d1);

        // Meters per unit of the march parameter (the state-space
        // tangent is a unit vector; the 3-D speed converts it).
        let speed = sys.tangent_speed(&x, &d1);
        let unusable = |fault| SsiError::StepUnusable {
            mode: mode.name(),
            speed,
            fault,
        };
        if !speed.is_finite() || speed <= 0.0 {
            // At zero the state moves and the point does not; at `±∞`
            // or poison the step divides to `0` and the meters it
            // claims are indeterminate, which `ssi_step_progress` would
            // escalate in the speed's place.
            return Err(unusable(StepFault::SpeedUnusable));
        }

        // ---- 4./5. the step ----
        let (dx, h_meters, bound) = match mode {
            StepperMode::Idealized => {
                // The spec: a tangent line of fixed tiny length.
                let h = [step_cap / speed, ctx.diagonal()]
                    .into_iter()
                    .fold((SSI_IDEALIZED_STEP * ctx.extent) / speed, Real::min);
                (scale(&d1, h), h * speed, StepBound::Cap)
            }
            StepperMode::Realized => {
                let b2 = sys.rhs2(&x, &d1);
                let d2 = svd.solve_min_norm(&b2);
                let kappa = norm(&d2);
                let b3 = sys.rhs3(&x, &d1, &d2);
                let mut d3 = svd.solve_min_norm(&b3);
                // Frenet: γ₃ = −κ².
                let kappa_sq = kappa * kappa;
                for (i, v) in d3.iter_mut().enumerate() {
                    *v -= kappa_sq * d1[i];
                }
                let n3 = norm(&d3);
                // (a) Hoffmann's relative heuristic: |h²κ/2| ≤ ρ·h and
                //     |h³‖d₃‖/6| ≤ ρ·h.
                // κ and ‖d₃‖ are the system's own answers, so each
                // bound is unbounded only at an exact zero and a
                // poisoned one carries through `h` to the step guard.
                // An overflowed κ² is not poison but makes ‖d₃‖ NaN
                // through ∞·0 in the correction above; `h_quad` already
                // binds there, so the cubic rung stands aside.
                let h_quad = if kappa != 0.0 {
                    2.0 * SSI_STEP_RELATIVE / kappa
                } else {
                    f64::INFINITY
                };
                let h_cub = if n3 != 0.0 && kappa_sq != f64::INFINITY {
                    (6.0 * SSI_STEP_RELATIVE / n3).sqrt()
                } else {
                    f64::INFINITY
                };
                // (b) the fit's between-sample budget (module docs).
                // κ and ‖d₃‖ are in state units; the curvature that
                // governs the 3-D fit is κ/speed², so convert once.
                // `> 0.0`, not `!= 0.0`: the quotient is NaN at an
                // underflowing speed (0/0) or an overflowing one (∞/∞),
                // which is the speed's fault, not a poisoned κ — and a
                // poisoned κ already reaches `h` through `h_quad`.
                let kappa3d = kappa / (speed * speed);
                let h_fit = if kappa3d > 0.0 {
                    // ¼ power as TWO square roots, not `powf(0.25)`:
                    // `f64::sqrt` is IEEE-correctly-rounded and so is
                    // its composition, while `powf` is a libm routine
                    // with no such guarantee and no libm-only contract
                    // behind it. Same value to within an ulp, but the
                    // D9 promise ("libm-only, bit-replayable") is only
                    // true of the sqrt route.
                    ((24.0 * SSI_STEP_DEVIATION * ctx.tol.meters()) / (kappa3d * kappa3d * kappa3d))
                        .sqrt()
                        .sqrt()
                        / speed
                } else {
                    f64::INFINITY
                };
                let h_curve = [h_cub, h_fit].into_iter().fold(h_quad, Real::min);
                let h_cap = Real::min(step_cap / speed, ctx.diagonal());
                let h = Real::min(h_curve, h_cap);
                // Bookkeeping for the budget's refusal only: a poisoned
                // `h` reaches the step guard below whichever rung is named.
                let bound = if h_curve < h_cap {
                    StepBound::Curvature
                } else {
                    StepBound::Cap
                };
                let mut step = [0.0f64; N];
                for (i, s) in step.iter_mut().enumerate() {
                    *s = h * d1[i] + 0.5 * h * h * d2[i] + (h * h * h / 6.0) * d3[i];
                }
                (step, h * speed, bound)
            }
        };

        // The stepper must be able to move at this tolerance.
        match decide("ssi_step_progress", Margin::of(h_meters), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => {
                return Err(SsiError::StepCollapsed {
                    mode: mode.name(),
                    step_meters: h_meters,
                    speed,
                });
            }
            Err(diag) => return Err(TraceDecision::StepProgress.escalated(diag)),
        }

        // The step itself, where it is minted: a positive finite speed
        // still overflows `h` (or `h²` in the realized approximant) when
        // it is small enough, and a large one shrinks the step below
        // the state's resolution. Either way no ε would help, so the
        // refusal names the speed. Asked after `ssi_step_progress`, so
        // a poisoned κ still escalates there.
        let mut next = x;
        for (i, v) in next.iter_mut().enumerate() {
            *v += dx[i];
        }
        if !dx.iter().all(|d| d.is_finite()) {
            return Err(unusable(StepFault::NotFinite));
        }
        if next == x {
            return Err(unusable(StepFault::DoesNotMove));
        }

        // ---- 6. Newton refinement to the surface pair ----
        let Some(refined) = newton_refine(sys, next, ctx.tol) else {
            // A step that will not settle is a step into nothing:
            // refuse rather than record a bad sample, unless it left
            // the lane's domain, which is where the branch ends.
            if let Some(end) = exit.unsettled(x, next, &ctx) {
                return Ok(Trace {
                    states,
                    end,
                    min_transversality,
                    min_sigma,
                    steps: steps + 1,
                });
            }
            return Err(SsiError::StepRefinementFailed {
                mode: mode.name(),
                step_meters: h_meters,
            });
        };
        next = refined;
        steps += 1;
        if bound == StepBound::Curvature {
            curvature_bound += 1;
        }

        // ---- the lane's exit ----
        if let Some(end) = exit.after_step(sys, &mut states, x, next, &ctx, band)? {
            return Ok(Trace {
                states,
                end,
                min_transversality,
                min_sigma,
                steps,
            });
        }

        // ---- the closure pair ----
        let back = distance_meters(sys, &next, &seed_state);
        if !left_start && back > 3.0 * h_meters {
            left_start = true;
        }
        if left_start {
            match decide("ssi_closure_return", Margin::of(h_meters - back), band) {
                Ok(Sign::Positive) | Ok(Sign::Zero) => {
                    // Returned. Is it a closure or a crossing?
                    let t0 = seed_tangent.unwrap_or(d1);
                    let cos_phi = dot(&d1, &t0).clamp(-1.0, 1.0);
                    let arc = arc_length(sys, &states);
                    match decide("ssi_closure_tangent", Margin::levered(cos_phi, arc), band) {
                        Ok(Sign::Positive) => {
                            // Close exactly onto the seed. If the last
                            // marched state is already essentially the
                            // seed, replace it rather than append: a
                            // zero-length chord is what the fitting
                            // stack refuses, and a loop that closed
                            // well is not an error.
                            if let Some(last) = states.last().copied() {
                                let mut d = 0.0f64;
                                for (i, v) in last.iter().enumerate() {
                                    let g = v - seed_state[i];
                                    d += g * g;
                                }
                                if d.sqrt() <= 1.0e-12 * h_meters.max(1.0) {
                                    states.pop();
                                }
                            }
                            states.push(seed_state);
                            return Ok(Trace {
                                states,
                                end: E::CLOSED,
                                min_transversality,
                                min_sigma,
                                steps,
                            });
                        }
                        Ok(Sign::Zero | Sign::Negative) => {
                            return Err(SsiError::SelfCrossingLocus {
                                cos_phi,
                                arc_length: arc,
                            });
                        }
                        Err(diag) => return Err(TraceDecision::ClosureTangent.escalated(diag)),
                    }
                }
                Ok(Sign::Negative) => {}
                Err(diag) => return Err(TraceDecision::ClosureReturn.escalated(diag)),
            }
        }

        states.push(next);
        x = next;
    }
    Err(SsiError::StepBudget {
        mode: mode.name(),
        budget: ctx.max_steps,
        bound: StepBound::of(curvature_bound, steps),
    })
}

/// Minimum-norm Newton onto `F = 0`, fixed cap, early exit at
/// [`MarchTol::settling`]. `None` when the iteration poisons or fails to
/// settle — never a best-effort state.
pub(crate) fn newton_refine<const M: usize, const N: usize, S>(
    sys: &S,
    mut x: [f64; N],
    step_tol: MarchTol,
) -> Option<[f64; N]>
where
    S: LocalSystem<M, N>,
{
    let tol = step_tol.settling();
    for _ in 0..SSI_NEWTON_ITERS {
        let f = sys.residual(&x);
        let mut worst = 0.0f64;
        for v in f.iter() {
            let a = v.abs();
            if a.is_nan() {
                return None;
            }
            if a > worst {
                worst = a;
            }
        }
        if worst <= tol {
            return Some(x);
        }
        let svd = Svd::<M, N>::new(sys.jacobian(&x));
        let mut rhs = [0.0f64; M];
        for (i, r) in rhs.iter_mut().enumerate() {
            *r = -f[i];
        }
        let dx = svd.solve_min_norm(&rhs);
        for (i, v) in x.iter_mut().enumerate() {
            *v += dx[i];
            if !v.is_finite() {
                return None;
            }
        }
    }
    // One last look: the cap is fixed, so a state that arrived inside
    // tolerance on the final correction still counts.
    let f = sys.residual(&x);
    if f.iter().all(|v| v.abs() <= tol) {
        Some(x)
    } else {
        None
    }
}

fn dot<const N: usize>(a: &[f64; N], b: &[f64; N]) -> f64 {
    let mut acc = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        acc += x * y;
    }
    acc
}

fn norm<const N: usize>(a: &[f64; N]) -> f64 {
    dot(a, a).sqrt()
}

fn neg<const N: usize>(a: &[f64; N]) -> [f64; N] {
    let mut o = *a;
    for v in o.iter_mut() {
        *v = -*v;
    }
    o
}

fn scale<const N: usize>(a: &[f64; N], k: f64) -> [f64; N] {
    let mut o = *a;
    for v in o.iter_mut() {
        *v *= k;
    }
    o
}

/// Distance between two states, in meters, through the coordinate
/// scales.
fn distance_meters<const M: usize, const N: usize, S>(sys: &S, a: &[f64; N], b: &[f64; N]) -> f64
where
    S: LocalSystem<M, N>,
{
    let scales = sys.coordinate_scale(a);
    let mut acc = 0.0f64;
    for (i, s) in scales.iter().enumerate() {
        let d = (a[i] - b[i]) * s;
        acc += d * d;
    }
    acc.sqrt()
}

/// The polyline's 3-D arc length so far, in meters — the closure
/// trilean's lever arm.
fn arc_length<const M: usize, const N: usize, S>(sys: &S, states: &[[f64; N]]) -> f64
where
    S: LocalSystem<M, N>,
{
    let mut acc = 0.0f64;
    for w in states.windows(2) {
        acc += (sys.point(&w[1]) - sys.point(&w[0])).norm();
    }
    acc
}

/// The signed distance from the state to the domain box, in meters
/// (positive inside), through the coordinate scales.
fn domain_margin<const M: usize, const N: usize, S>(
    x: &[f64; N],
    ctx: &MarchContext<N>,
    sys: &S,
    at: &[f64; N],
) -> f64
where
    S: LocalSystem<M, N>,
{
    let scales = sys.coordinate_scale(at);
    let mut worst = f64::INFINITY;
    for (i, s) in scales.iter().enumerate() {
        let lo = (x[i] - ctx.domain[i][0]) * s;
        let hi = (ctx.domain[i][1] - x[i]) * s;
        worst = [lo, hi].into_iter().fold(worst, Real::min);
    }
    worst
}

/// Fixed bisection count for the ℝ³ lane's slab-crossing search (D9).
pub const SSI_SLAB_BISECTIONS: usize = 32;

/// Whether a state is inside the named domain box (a structure test on
/// the raw coordinates — C6's lane, not a predicate).
fn within<const N: usize>(x: &[f64; N], domain: &[[f64; 2]; N]) -> bool {
    x.iter()
        .enumerate()
        .all(|(i, v)| *v >= domain[i][0] && *v <= domain[i][1])
}

/// Push an ℝ³ branch's endpoint on the caller's slab ([`SlabExit`]).
///
/// **Clipping, not clamping**, and the difference is a two-millimetre
/// residual: coordinate-wise clamping of the overshooting state moves
/// it *off the locus*, and that fabricated point then goes into the fit
/// and shows up as an on-locus certificate failure at the branch's own
/// end.
///
/// So the step is bisected (fixed count, D9) for the largest fraction
/// whose **Newton-refined** state is still inside the box. Testing
/// insideness *after* refinement rather than before is the whole
/// subtlety: Newton moves the crossing point along the locus, and on a
/// branch that runs to a surface's own knot-domain edge it moves it
/// *out* — by a micron on the M5 wall fixture, which is a thousand ε
/// and duly failed limb 1. The endpoint this produces is on the locus
/// **and** inside the domain, within one bisection of the boundary.
///
/// A crossing that will not refine, or one that lands on top of the
/// previous state, is dropped: the branch then ends at its last
/// certified state, which is honest.
fn push_boundary<S>(
    sys: &S,
    states: &mut Vec<[f64; 3]>,
    inside: [f64; 3],
    outside: [f64; 3],
    ctx: &MarchContext<3>,
) where
    S: LocalSystem<2, 3>,
{
    let mut lo = 0.0f64;
    let mut hi = 1.0f64;
    let mut best: Option<[f64; 3]> = None;
    for _ in 0..SSI_SLAB_BISECTIONS {
        let m = 0.5 * (lo + hi);
        let mut t = inside;
        for (i, v) in t.iter_mut().enumerate() {
            *v = inside[i] + (outside[i] - inside[i]) * m;
        }
        match newton_refine(sys, t, ctx.tol) {
            Some(r) if within(&r, &ctx.domain) => {
                lo = m;
                best = Some(r);
            }
            _ => hi = m,
        }
    }
    let Some(end) = best else {
        return;
    };
    // Reject a duplicate of the previous state (zero-length chord).
    let mut d = 0.0f64;
    for (i, v) in end.iter().enumerate() {
        let g = v - inside[i];
        d += g * g;
    }
    if d.sqrt() > 0.0 {
        states.push(end);
    }
}

/// The 3-D points of a trace — the fitting stack's input.
pub(crate) fn trace_points<const M: usize, const N: usize, S, E>(
    sys: &S,
    trace: &Trace<N, E>,
) -> Vec<Point3<f64>>
where
    S: LocalSystem<M, N>,
{
    trace.states.iter().map(|s| sys.point(s)).collect()
}

/// **The ℝ³ lane's march from a seed**: both ways, spliced, unless the
/// forward march already closed the loop.
///
/// A seed lands in the middle of whatever branch it is on. Marching one
/// direction covers half of an open branch, which is not a branch — it
/// is half a lie, and it would arrive at the fitting stack looking like
/// a complete carrier. So an open forward march is followed by a
/// backward one from the same seed, and the two are spliced with the
/// seed as the join. A closed forward march needs no second pass: it
/// already covered the component.
///
/// The first march caps its steps at [`SSI_STEP_MAX`] of the caller's
/// extent. A trace with fewer samples than the cubic fit needs, and a
/// positive length, is marched once more with its steps capped at that
/// length over [`SHORT_BRANCH_STEPS`]. The rule is fixed and taken at
/// most once (D9). Written for the ℝ³ state alone: the plane × NURBS
/// lane knows its branches' ends before it marches and never reaches
/// this.
///
/// # Errors
///
/// As [`march`]; a refusal in either direction, on either march, is the
/// operation's. [`SsiError::TraceUnresolved`] when the first trace has
/// no length to cut, or the re-march is still too short to fit: a limit
/// of the march against the caller's slab, not a branch. A trace with a
/// non-finite sample (a `NaN` length) goes to the fit, which refuses
/// the sample by name.
pub(crate) fn march_both<S>(
    sys: &S,
    seed: [f64; 3],
    ctx: MarchContext<3>,
    mode: StepperMode,
    band: Band,
) -> Result<Trace<3, SlabEnd>, SsiError>
where
    S: LocalSystem<2, 3> + TransversalityData<3>,
{
    let cap = SSI_STEP_MAX * ctx.extent;
    let first = march_both_at(sys, seed, ctx, mode, band, cap)?;
    if first.states.len() > SSI_FIT_DEGREE {
        return Ok(first);
    }
    let length = arc_length(sys, &first.states);
    if length.is_nan() {
        // A non-finite sample: the fit refuses it by name. Finite
        // samples whose chords overflow read `+∞`, which re-marches at
        // the extent's cap and refuses below if still short.
        return Ok(first);
    }
    if length <= 0.0 {
        return Err(SsiError::TraceUnresolved {
            samples: first.states.len(),
            step: cap,
        });
    }
    let step = Real::min(cap, length / SHORT_BRANCH_STEPS as f64);
    let again = march_both_at(sys, seed, ctx, mode, band, step)?;
    if again.states.len() > SSI_FIT_DEGREE {
        return Ok(again);
    }
    Err(SsiError::TraceUnresolved {
        samples: again.states.len(),
        step,
    })
}

/// How many steps a short branch's re-march cuts its length into: the
/// fewest odd count that gives the cubic fit its samples without the
/// two boundary ends, which [`push_boundary`] may drop. Odd, because a
/// seed lands near the middle of a branch as often as not: from there
/// an odd count leaves `n` states strictly inside the branch and each
/// end half a step from the nearest, where an even one walks a state
/// onto each end.
pub(crate) const SHORT_BRANCH_STEPS: usize = (SSI_FIT_DEGREE + 1) | 1;

/// [`march_both`]'s two marches and their splice, at one `step_cap`.
fn march_both_at<S>(
    sys: &S,
    seed: [f64; 3],
    ctx: MarchContext<3>,
    mode: StepperMode,
    band: Band,
    step_cap: f64,
) -> Result<Trace<3, SlabEnd>, SsiError>
where
    S: LocalSystem<2, 3> + TransversalityData<3>,
{
    let fwd = march(sys, &SlabExit, seed, ctx, mode, 1.0, band, step_cap)?;
    if fwd.end == SlabEnd::Closed {
        return Ok(fwd);
    }
    let bwd = march(sys, &SlabExit, seed, ctx, mode, -1.0, band, step_cap)?;
    let mut states = bwd.states;
    states.reverse();
    // `states` now runs backward-end → seed; append the forward half
    // without repeating the seed.
    states.extend_from_slice(&fwd.states[1..]);
    Ok(Trace {
        states,
        // If either end is in band of the slab, the branch's end is in
        // band: the honest verdict is the weaker of the two.
        end: if fwd.end == SlabEnd::SlabInBand || bwd.end == SlabEnd::SlabInBand {
            SlabEnd::SlabInBand
        } else {
            SlabEnd::Slab
        },
        min_transversality: fwd.min_transversality.min(bwd.min_transversality),
        min_sigma: fwd.min_sigma.min(bwd.min_sigma),
        steps: fwd.steps + bwd.steps,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{
        Box3, LocalSystem, MarchContext, MarchTol, NormalPair, ReachBound, Readout, SSI_NEWTON_TOL,
        SSI_QUADRIC_NOISE_ULPS, SSI_SETTLE_MAX, SSI_STEP_MAX, SsiError, StepFault, StepperMode,
        TraceDecision, TransversalityData, march,
    };
    use crate::ssi::ExhaustLane;
    use geom_core::{Band, Point3, Vec3};

    /// A quadric pair's residual read in the box of half-width `r`
    /// around `(x, 0, 0)`.
    fn read_at(x: f64, r: f64) -> Readout {
        Readout::spatial(
            Box3::around(Point3::new(x, 0.0, 0.0), r),
            None,
            SSI_QUADRIC_NOISE_ULPS,
        )
    }

    /// The same, about the origin.
    fn reaching(r: f64) -> Readout {
        read_at(0.0, r)
    }

    /// A two-plane system in ℝ³ whose locus is the `x` axis, with the
    /// chart speed, the order-2 and order-3 right-hand sides, the `x`
    /// coordinate scale and the lever arm dictated by the row. The
    /// residual is exact at the seed, the Jacobian is constant, and the
    /// two normals meet at a right angle — so the only thing a march
    /// over this system can refuse on is the one value the row spoils,
    /// and the refusal it produces names that value's guard.
    struct FixedSpeedR3 {
        /// Meters per unit of the march parameter.
        speed: f64,
        /// Both components of the order-2 right-hand side.
        rhs2: f64,
        /// Both components of the order-3 right-hand side.
        rhs3: f64,
        /// Meters per unit of the `x` coordinate.
        x_scale: f64,
        /// The lever arm the system reports, before the march's clamp.
        arm: f64,
    }

    impl FixedSpeedR3 {
        fn at_speed(speed: f64) -> Self {
            Self {
                speed,
                rhs2: 0.0,
                rhs3: 0.0,
                x_scale: 1.0,
                arm: f64::MAX,
            }
        }
    }

    impl LocalSystem<2, 3> for FixedSpeedR3 {
        fn residual(&self, x: &[f64; 3]) -> [f64; 2] {
            [x[1], x[2]]
        }

        fn jacobian(&self, _x: &[f64; 3]) -> [[f64; 3]; 2] {
            [[0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]
        }

        fn rhs2(&self, _x: &[f64; 3], _d1: &[f64; 3]) -> [f64; 2] {
            [self.rhs2, self.rhs2]
        }

        fn rhs3(&self, _x: &[f64; 3], _d1: &[f64; 3], _d2: &[f64; 3]) -> [f64; 2] {
            [self.rhs3, self.rhs3]
        }

        fn point(&self, x: &[f64; 3]) -> Point3<f64> {
            Point3::from_array(*x)
        }

        fn coordinate_scale(&self, _x: &[f64; 3]) -> [f64; 3] {
            [self.x_scale, 1.0, 1.0]
        }

        fn tangent_speed(&self, _x: &[f64; 3], _d: &[f64; 3]) -> f64 {
            self.speed
        }
    }

    impl TransversalityData<3> for FixedSpeedR3 {
        fn normals(&self, _x: &[f64; 3]) -> NormalPair {
            (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0))
        }

        fn lever_arm(&self, _x: &[f64; 3]) -> f64 {
            self.arm
        }
    }

    /// **The stepper's speed guard**: only a POSITIVE FINITE speed
    /// converts a state step into meters, and every value that is not
    /// one is refused by the speed's own name.
    ///
    /// `+∞` is the case a `is_nan() || <= 0.0` test admits, and what
    /// follows it is not a silent wrong answer but a wrong DIAGNOSIS:
    /// the idealized step `h = (step · extent)/∞` is exactly `0`, the
    /// meters it claims are `0 · ∞ = NaN`, and `ssi_step_progress`
    /// escalates on an indeterminate margin — telling the caller the
    /// stepper cannot progress at this ε when the truth is that this
    /// system has no usable speed and no ε would have helped.
    ///
    /// All five non-positive-finite values are pinned together because
    /// the guard's obligation is the class, not the one member of it a
    /// predicate happened to miss.
    #[test]
    fn a_non_finite_march_speed_refuses_by_the_speeds_own_name() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        for speed in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN, 0.0, -1.0] {
            let sys = FixedSpeedR3::at_speed(speed);
            let r = march(
                &sys,
                &super::SlabExit,
                [0.0; 3],
                unit_ctx(band),
                StepperMode::Idealized,
                1.0,
                band,
                SSI_STEP_MAX * unit_ctx(band).extent,
            );
            match r {
                Err(SsiError::StepUnusable {
                    mode,
                    speed: named,
                    fault: StepFault::SpeedUnusable,
                }) => {
                    assert_eq!(mode, StepperMode::Idealized.name());
                    assert_eq!(named.to_bits(), speed.to_bits(), "the speed it names");
                    let ending = SsiError::StepUnusable {
                        mode,
                        speed,
                        fault: StepFault::SpeedUnusable,
                    }
                    .ending(crate::recourse::Reading::Build);
                    assert_eq!(ending, geom_core::KERNEL_DEFECT_ENDING);
                }
                Err(SsiError::Escalated { decision, .. }) => panic!(
                    "WRONG DIAGNOSIS: a march speed of {speed:e} escalated on \
                     {decision:?} instead of being refused as the speed it is"
                ),
                other => panic!("expected the speed guard for {speed:e}, got {other:?}"),
            }
        }
    }

    /// **The step is guarded where it is minted**, for the positive
    /// finite speeds the class guard admits. Each row reached a wrong
    /// diagnosis or none before the guard, measured at this unit's base:
    ///
    /// - a tiny speed in a unit box: the idealized step `1e-3/speed`
    ///   overshot the box by up to `1e297` widths, the boundary search's
    ///   32 bisections found no crossing, and the march answered `Ok`
    ///   with a one-state trace (at `1e-300`), or the step overflowed
    ///   and Newton answered `SeedRefinementFailed` (at `1e-320` and
    ///   `5e-324`). The step is capped at the box's diagonal, so what
    ///   the speed leaves is a step of `~3.5·speed` metres, which
    ///   collapses into the band, naming the speed.
    /// - a tiny speed in a box too wide for that cap: the realized
    ///   step is `0.031` m, but its `h²` overflows in state units and
    ///   `∞·0` made the step NaN, which Newton answered
    ///   `SeedRefinementFailed`.
    /// - a huge speed away from the origin: the step `1e-303` does not
    ///   move a state at `0.5`, and the march spun to `StepBudget`.
    #[test]
    fn an_unusable_step_refuses_naming_the_speed() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let wide = MarchContext::<3> {
            domain: [[-1.0e200, 1.0e200]; 3],
            ..unit_ctx(band)
        };
        let rows = [
            (1.0e-300, StepperMode::Idealized, unit_ctx(band), 0.0, None),
            (1.0e-320, StepperMode::Idealized, unit_ctx(band), 0.0, None),
            (
                f64::from_bits(1),
                StepperMode::Realized,
                unit_ctx(band),
                0.0,
                None,
            ),
            (
                1.0e-180,
                StepperMode::Realized,
                wide,
                0.0,
                Some(StepFault::NotFinite),
            ),
            (
                1.0e300,
                StepperMode::Idealized,
                unit_ctx(band),
                0.5,
                Some(StepFault::DoesNotMove),
            ),
        ];
        for (speed, mode, ctx, x0, fault) in rows {
            let sys = FixedSpeedR3::at_speed(speed);
            let r = march(
                &sys,
                &super::SlabExit,
                [x0, 0.0, 0.0],
                ctx,
                mode,
                1.0,
                band,
                SSI_STEP_MAX * ctx.extent,
            );
            let named = match (fault, &r) {
                (None, Err(SsiError::StepCollapsed { speed, .. })) => *speed,
                (Some(want), Err(SsiError::StepUnusable { speed, fault, .. }))
                    if *fault == want =>
                {
                    *speed
                }
                _ => panic!("speed {speed:e} ({mode:?}): expected {fault:?}, got {r:?}"),
            };
            assert_eq!(named.to_bits(), speed.to_bits(), "the speed it names");
            let ending = r.unwrap_err().ending(crate::recourse::Reading::Build);
            assert!(
                ending.starts_with("Recourse: bring the operands"),
                "speed {speed:e}: {ending:?}"
            );
        }
    }

    /// The context every row below marches in.
    fn unit_ctx(band: Band) -> MarchContext<3> {
        MarchContext::<3> {
            domain: [[-1.0, 1.0]; 3],
            extent: 1.0,
            tol: MarchTol::from_band(band, reaching(1.0)).unwrap(),
            max_steps: 64,
        }
    }

    /// **A poisoned value reaches the guard that decides it**, rather
    /// than being folded away by a `min` that keeps the other operand.
    /// Each row poisons one input of one guarded fold and pins the
    /// escalation by that guard's name: the lever arm through the
    /// extent clamp to `ssi_transversality_arm`; the order-3 right-hand
    /// side (with a finite, zero κ) through the cubic rung of the step
    /// fold to `ssi_step_progress`; the order-2 right-hand side (κ
    /// itself) to the same guard, through both `h_quad` and the κ²
    /// correction in `‖d₃‖`, so this row goes red only when both paths
    /// drop it; and a coordinate scale through the domain-margin fold
    /// to `ssi_branch_open_end`.
    #[test]
    fn a_poisoned_fold_operand_escalates_at_its_own_guard() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let rows = [
            (
                FixedSpeedR3 {
                    arm: f64::NAN,
                    ..FixedSpeedR3::at_speed(1.0)
                },
                StepperMode::Idealized,
                TraceDecision::TransversalityArm,
            ),
            (
                FixedSpeedR3 {
                    rhs3: f64::NAN,
                    ..FixedSpeedR3::at_speed(1.0)
                },
                StepperMode::Realized,
                TraceDecision::StepProgress,
            ),
            (
                FixedSpeedR3 {
                    rhs2: f64::NAN,
                    ..FixedSpeedR3::at_speed(1.0)
                },
                StepperMode::Realized,
                TraceDecision::StepProgress,
            ),
            (
                FixedSpeedR3 {
                    x_scale: f64::NAN,
                    ..FixedSpeedR3::at_speed(1.0)
                },
                StepperMode::Idealized,
                TraceDecision::BranchOpenEnd,
            ),
        ];
        for (sys, mode, guard) in rows {
            match march(
                &sys,
                &super::SlabExit,
                [0.0, 0.0, 0.0],
                unit_ctx(band),
                mode,
                1.0,
                band,
                SSI_STEP_MAX * unit_ctx(band).extent,
            ) {
                Err(ref e @ SsiError::Escalated { decision, .. }) => {
                    assert_eq!(decision, guard, "the poisoned operand's guard");
                    // A poisoned margin names no lever of the decision's:
                    // the arm gate ends as transversality's unreadable
                    // margin, the rest as the kernel's defect.
                    let ending = e.ending(crate::recourse::Reading::Build);
                    if guard == TraceDecision::TransversalityArm {
                        assert!(
                            ending.ends_with(geom_core::UNREADABLE_MARGIN_NOTE),
                            "{guard:?}: {ending}"
                        );
                    } else {
                        assert_eq!(ending, geom_core::KERNEL_DEFECT_ENDING, "{guard:?}");
                    }
                }
                other => panic!("expected {guard:?} to escalate, got {other:?}"),
            }
        }
    }

    /// **A NaN the arithmetic manufactures from finite inputs is not a
    /// poisoned answer**, and must not escalate the step guard. A flat
    /// locus at an underflowing speed makes the fit rung's `κ/speed²`
    /// a `0/0`; a curvature whose square overflows makes the Frenet
    /// correction's `κ²·0` an `∞·0`. Neither is the system's fault at
    /// the step, so neither may surface as `ssi_step_progress`.
    #[test]
    fn a_nan_manufactured_from_finite_inputs_is_not_a_step_escalation() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let mut rows: Vec<FixedSpeedR3> = [1.0e-170, 1.0e-200, 1.0e-300]
            .into_iter()
            .map(FixedSpeedR3::at_speed)
            .collect();
        rows.push(FixedSpeedR3 {
            rhs2: 1.0e200,
            ..FixedSpeedR3::at_speed(1.0)
        });
        for sys in rows {
            let r = march(
                &sys,
                &super::SlabExit,
                [0.0; 3],
                unit_ctx(band),
                StepperMode::Realized,
                1.0,
                band,
                SSI_STEP_MAX * unit_ctx(band).extent,
            );
            assert!(
                !matches!(
                    &r,
                    Err(SsiError::Escalated {
                        decision: TraceDecision::StepProgress,
                        ..
                    })
                ),
                "speed {:e}, rhs2 {:e}: {r:?}",
                sys.speed,
                sys.rhs2
            );
        }
    }

    /// The bridge, stated as a row: a marcher tolerance derived from a
    /// band **is** that band's coincidence threshold. Nothing scales it,
    /// pads it, or rounds it — the marcher's step rule and the
    /// certificate's floors are the same number, so a run cannot march
    /// at one tolerance and certify at another.
    #[test]
    fn a_derived_march_tolerance_is_the_bands_own_zero() {
        for zero in [1.0e-3_f64, 1.0e-6, 1.0e-9, 1.0e-12] {
            // The escalate edge is arbitrary here: `MarchTol::from_band`
            // reads `band.zero()` and nothing else, which is the claim
            // this row makes. Any value above `zero` satisfies
            // `Band::new`'s `zero < escalate`; it is deliberately NOT
            // the run's K·zero, which would read as a quantity the
            // bridge consults.
            let band = Band::new(zero, 2.0 * zero).unwrap();
            let tol = MarchTol::from_band(band, reaching(1.0)).unwrap();
            assert_eq!(tol.meters(), band.zero());
            assert!(tol.is_of(band));
        }
    }

    /// The decoupled constructor is the only other door, and it refuses
    /// typed rather than clamping: a generator tolerance that is not a
    /// length is a caller error, not a value to repair silently.
    #[test]
    fn a_decoupled_march_tolerance_refuses_typed_on_a_non_length() {
        for bad in [0.0_f64, -1.0e-9, f64::NAN, f64::INFINITY] {
            match MarchTol::decoupled(bad, reaching(1.0)) {
                Err(SsiError::InvalidMarchTol { value }) => {
                    assert!(value.is_nan() || value == bad, "{value:e} vs {bad:e}");
                    let msg =
                        SsiError::InvalidMarchTol { value }.render(crate::recourse::Reading::Build);
                    assert!(
                        msg.contains("not a usable length")
                            && msg.ends_with(
                                "Recourse: name a march tolerance that is a positive finite \
                                 length"
                            )
                            && !msg.contains("MarchTol"),
                        "the caller's knob: {msg}"
                    );
                }
                other => panic!("expected a typed refusal for {bad:e}, got {other:?}"),
            }
        }
        assert_eq!(
            MarchTol::decoupled(1.0e-9, reaching(1.0)).unwrap().meters(),
            1.0e-9
        );
    }

    /// **The march settles to what its coordinates resolve, and refuses
    /// by name where that is not well inside ε.** Near the origin it
    /// settles to `SSI_NEWTON_TOL`·ε; where the coordinates' noise is
    /// larger it settles to the noise; where the noise passes
    /// `SSI_SETTLE_MAX`·ε, at `1e8` m against ε = `1e-9`, both doors
    /// refuse by the scale.
    #[test]
    fn the_march_settles_to_what_its_coordinates_resolve() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let near = MarchTol::from_band(band, reaching(1.0)).unwrap();
        assert_eq!(
            near.settling(),
            SSI_NEWTON_TOL * band.zero(),
            "near the origin"
        );
        let mid = MarchTol::from_band(band, read_at(1.0e5, 1.0)).unwrap();
        let gap = 1.0e5f64 - 1.0e5f64.next_down();
        assert!(
            mid.settling() > SSI_NEWTON_TOL * band.zero()
                && mid.settling() >= SSI_QUADRIC_NOISE_ULPS * gap,
            "at 1e5 m it settles to the noise: {:e}",
            mid.settling()
        );
        for minted in [
            MarchTol::from_band(band, read_at(1.0e8, 1.0)),
            MarchTol::decoupled(band.zero(), read_at(1.0e8, 1.0)),
        ] {
            let Err(SsiError::SettlingUnresolvable(r)) = minted else {
                panic!("expected the settling door, got {minted:?}");
            };
            assert!(matches!(r.lane, ExhaustLane::R3), "{r:?}");
            assert_eq!(r.bound, ReachBound::Domain, "no operand box: {r:?}");
            assert!(
                r.reach > 1.0e8 && r.gap > 1.0e-8 && r.settle > SSI_SETTLE_MAX * band.zero(),
                "{r:?}"
            );
        }
    }

    /// **The reach is where the states can be**: a domain reaching
    /// `1e8` m around an operand box at the origin reads at the box, and
    /// names the geometry as what bounds it.
    #[test]
    fn the_readout_reads_where_the_operands_bound_the_states() {
        let band = Band::new(1.0e-9, 1.0e-8).unwrap();
        let wide = Box3::around(Point3::new(0.0, 0.0, 0.0), 1.0e8);
        let unit = Box3::around(Point3::new(0.0, 0.0, 0.0), 1.0);
        let r = Readout::spatial(wide, Some(unit), SSI_QUADRIC_NOISE_ULPS);
        assert_eq!(r.bound, ReachBound::Geometry, "{r:?}");
        assert!(r.reach <= 1.0 + 1.0e-12, "{r:?}");
        let tol = MarchTol::from_band(band, r).unwrap();
        assert_eq!(tol.settling(), SSI_NEWTON_TOL * band.zero());
    }

    /// **A rung is named from a quarter of the steps**, on either side
    /// of the line, for each rung.
    #[test]
    fn a_rung_is_named_from_a_quarter_of_the_steps() {
        use super::StepBound;
        for (curvature, want) in [
            (0, StepBound::Cap),
            (4, StepBound::Cap),
            (5, StepBound::Both),
            (15, StepBound::Both),
            (16, StepBound::Curvature),
            (20, StepBound::Curvature),
        ] {
            assert_eq!(StepBound::of(curvature, 20), want, "{curvature} of 20");
        }
    }
}

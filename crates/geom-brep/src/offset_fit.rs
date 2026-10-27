//! **The offset fit and its certificate** — the Book's §9.4 stack for
//! surfaces, and the two-limb residual bound (`crates/geom-brep/README.md`
//! O2/O3, `crates/geom-brep/README.md` C8).
//!
//! The offset of a NURBS surface is **not** a NURBS — normalizing the
//! chart normal introduces a square root that breaks rationality — so
//! the kernel fits one and certifies the fit:
//!
//! ```text
//! sup_(u,v) ‖ S_fit(u,v) − (S(u,v) + d·n(u,v)) ‖ ≤ ε_precision
//! ```
//!
//! D4's two-tolerance split puts that residual on the ε_precision
//! side. The claim is **pointwise in the chart parameters**, the same
//! `(u,v)` on both sides — which is what makes a hull-side bound
//! possible at all, and it is the reason for this module's one
//! deliberate departure from the Book (below).
//!
//! # The fit engine, and where it leaves the Book
//!
//! **A9.4 (`GlobalSurfInterp`, p. 380), taken whole except for its
//! parameters.** The Book computes `(ū_k, v̄_ℓ)` by `SurfMeshParams` —
//! chord-length parameters of the data, averaged across rows and
//! columns — because its data is a bare point grid with no
//! parameterization of its own. Ours is not: the data is sampled
//! *from* a base surface, and the certificate's claim is stated at the
//! base's own `(u,v)`. So the fit interpolates **at the base's chart
//! parameters**, and everything downstream of that in A9.4 is
//! verbatim: Eq. 9.8's averaged knot vector over those parameters,
//! then the two passes of curve interpolation (Eqs. 9.26–9.27) —
//! `m+1` in `u` to get the `R_{i,ℓ}`, then `n+1` in `v` to get the
//! `P_{i,j}` — each pass a single collocation solve with every column
//! as a simultaneous right-hand side
//! (`geom::curves::fit::interpolate_columns`, the loft/skinning door),
//! so the columns cannot drift apart even in float.
//!
//! **A9.10's shape, not A9.10.** The Book's Type-2 method
//! (`GlobalCurveApproxErrBnd`, p. 431, and the surface knot-removal
//! bounds Eqs. 9.86–9.89 it would be built from) works *downward*: fit
//! a fixed point set at degree 1, remove knots while a removal bound
//! holds, degree-elevate, refit. That is the right algorithm when the
//! data is all you have. Here the target is a *function* — the exact
//! offset — that can be resampled at any parameter, so the loop runs
//! the other way: interpolate, certify, and **insert** sample
//! parameters (hence knots, through Eq. 9.8) inside the cells that
//! carry the sup, until every cell certifies or the crate's stop rule
//! ([`crate::descent`]) ends it: the bound stops falling, or a work
//! bound is reached.
//! What the two share is the thing that matters: a bound decides every
//! step, and exhaustion is a **typed refusal naming what ran out** —
//! a work bound ([`OffsetFitError::WallReached`]) or the
//! refinement itself ([`OffsetFitError::RefinementStalled`]),
//! each carrying the bound its last grid achieved and the smallest any
//! round reached, or [`OffsetFitError::BoundNotFinite`] when the last
//! round's bound is not one, carrying the smallest finite one (a bound
//! decides, expiry is typed and carries what was reached), never an
//! uncertified return.
//!
//! The compression half of A9.10 (knot removal under Eqs. 9.86–9.89,
//! shrinking the fitted structure once the tolerance is met) is NOT
//! built here: it changes no claim, only the size of the answer, and
//! it needs the surface analogues of A9.8/A9.9. It is banked for the
//! lane that first measures the fitted structure as too large.
//!
//! # The certificate's two limbs
//!
//! The SSI certificate's shape ([`crate::ssi::certify`]), lifted one
//! dimension, over a `(u,v)` span schedule — the cells of the merged
//! Bézier decomposition of the base and the fit.
//!
//! **Limb 1 — on-locus residual.** At a fixed
//! [`OFFSET_CERT_SAMPLES`]² schedule inside every cell, the exact
//! residual `‖S_fit − (S + d·n)‖` in metres. It steers; it does not
//! certify (a sampled max is not a bound).
//!
//! **Limb 2 — the hull-side sup bound.** This is where the square root
//! has to be faced. `S_fit − (S + d·n)` is not a polynomial and never
//! will be, so it cannot be given a coefficient net directly. What
//! *can* be is its two **rationalized parts**. Writing `E = S_fit − S`
//! and `m = S_u × S_v`, split the residual along and across the
//! normal:
//!
//! ```text
//! R = E − d·n = (E·n − d)·n + E_tangential
//! ‖R‖ ≤ | ‖E‖ − |d| |  +  τ  +  τ²/‖E‖        with τ = ‖E × m‖/‖m‖
//! ```
//!
//! (the middle step is `|E·n − d| ≤ | ‖E‖ − |d| | + (‖E‖ − |E·n|)` and
//! `‖E‖ − |E·n| = ‖E‖ − √(‖E‖² − τ²) ≤ τ²/‖E‖`, valid once `E·n` is
//! certified to carry `d`'s sign — which the composite checks, because
//! `sign(E·n) = sign(E·m)` and `E·m` is one of the polynomials below).
//!
//! Both ingredients are quotients of **polynomials whose coefficients
//! cancel**. With the base written homogeneously as `S = A/w` and the
//! fit as `S_fit = F̃/w_fit`, everything is homogeneous in the PRODUCT
//! `w̃ = w·w_fit` — so a rational fit is bounded as the surface it is,
//! not as its control net read flat:
//!
//! ```text
//! Ẽ = F̃·w − A·w_fit         (Ẽ = w̃·E)
//! M̃ = w·(A_u × A_v) − w_v·(A_u × A) − w_u·(A × A_v)      (M̃ = w³·m)
//! X = Ẽ·Ẽ − d²·w̃²           ( = w̃²·(‖E‖² − d²) )
//! Y = Ẽ × M̃                 ( = w̃·w³·(E × m) )
//! D = Ẽ · M̃                 ( = w̃·w³·(E · m) )
//! ```
//!
//! `M̃` carries the base's weight alone: `m = S_u × S_v` is the base's
//! own, and the fit's weights do not enter it.
//!
//! `X` and `Y` are the cancellation: `‖E‖ ≈ |d|` and `E ∥ m` are what
//! a good fit MEANS, so both polynomials are small — and a Bernstein
//! coefficient net of a small smooth function is small (its
//! derivatives are small too, so the Bernstein overshoot is), which is
//! what makes the hull bound track the residual's own scale instead of
//! the cell's geometric variation. The vector polynomials `Y` and `M̃`
//! are bounded from above by the largest norm of one coefficient
//! vector (D4 ¶2), never by a box of three channel hulls, so a rigid
//! map of base and fit moves those bounds only by rounding. Enclosing
//! `S_fit`, `S` and `d·n` separately and subtracting the enclosures
//! cannot see the cancellation: it
//! reports the sum of the two surfaces' motions across the cell, which
//! on a unit cylinder at `d = 0.2 m` would need millions of cells to
//! reach a micron. That failure mode is the one
//! [`geom_core::spline::compose::tensor`]'s docs record at one
//! parameter; here it is answered by
//! [`geom_core::spline::compose::patch`].
//!
//! **The small-`|d|` denominator.** The normal component divides
//! `|X|` by `w̃²·(‖E‖ + |d|)`. Bounding that below by `2|d|` alone is
//! both loose and brittle: once `dist` reaches `|d|` the cell
//! collapses to `+∞`, so a micron-scale offset on a metre-scale patch
//! certifies as `inf`. The composite therefore carries `Ẽ` and bounds
//! `‖E‖` below DIRECTLY, which makes the small-`|d|` case finite and
//! tightens every other row.
//!
//! It bounds it two ways and takes the larger. The first is the
//! componentwise mignitude assembly on `Ẽ`'s cell hulls — the same
//! inf-side shape meter 1 uses on the cross product. The second reads
//! the three components TOGETHER, through the sign witness the
//! composite already carries: `|E·n| ≤ ‖E‖` for any `E`, and with
//! `n = m/‖m‖`,
//!
//! ```text
//! ‖E‖ ≥ |E·n| = |E·m| / ‖m‖ = |D| / (w̃ · ‖M̃‖)
//! ```
//!
//! — `mig(D)` from below over the cell (positive on every cell that
//! passes the witness, since the witness is `D` definite) against the
//! sup of `‖M̃‖` from above, read from its coefficient vectors' norms.
//! The two disagree by orders of magnitude exactly where a good fit
//! lives: `E ≈ d·n`, so
//! every component of `E` straddles zero as the normal rotates across
//! the cell and the componentwise assembly collapses, while the
//! projection reads `‖E‖ ≈ |d|`. On the quarter cylinder at
//! `d = 1e-6` the sup cell's two readings are `1.58e-8` and
//! `5.64e-7`, and the cell's bound is `1.51e-5` rather than
//! `2.63e-4`.
//!
//! The `τ²/‖E‖` term takes the same floor, or `|d| − dist` when that
//! is larger — three lower bounds on one norm, whose max is one too.
//!
//! **Recentring, and what it did and did not buy.** Every net above
//! is built against one origin — the base control net's bbox midpoint
//! ([`recentre_origin`]) — so the composite's intermediates are the
//! size of the PATCH rather than of its coordinates. The identity is
//! exact in ℝ (`Ẽ` and `M̃` are both invariant under shifting base and
//! fit together), so nothing about the claim moves; only the rounding
//! does. What that bought is translation invariance, which the
//! residual always had and the bound did not: a micron offset on a
//! metre patch a kilometre from the origin certified as `inf` and now
//! certifies at the same `1.5e-5` the patch gives at the origin.
//!
//! What it does not reach is the floor on `‖E‖`: that one is not a
//! rounding problem at any origin, and it is answered above by the
//! projection through the sign witness rather than by recentring.
//!
//! **Where the regularity floor enters.** `τ` and `D` both divide by
//! `‖m‖`, and `X`'s reading divides by `w̃²`. Both weight hulls are
//! positive by the rational licence; `‖m‖` is positive only because
//! [`crate::offset_meters`]' floor says so — `‖M̃‖ ≥ floor·w³`. That
//! is the sense in which meter 1 "makes `1/‖S_u × S_v‖` boundable",
//! and it is why the fit door refuses on the floor before it fits
//! anything.
//!
//! # The production doors and the numeric-target instrument
//!
//! Every door here comes in two forms, and the pair is the shape D4 ¶1's
//! witness rule asks for.
//!
//! The **production** door takes [`Tol`] — [`fit_offset`],
//! [`certify_offset`], [`certify_offset_over`],
//! [`approx_offset_surface`], [`recertify_approx`]. That is the whole
//! surface the kernel reaches: [`crate::OffsetFitLane`] calls the last
//! three, and the mint calls down to the first three. A caller has no
//! number to pass, so two callers cannot fit against two epsilons, and
//! the value is read once (`precision_target`, private — the doors
//! are the surface).
//!
//! The **`_at`** form takes a chosen target and is `#[doc(hidden)]`,
//! because it is an INSTRUMENT rather than a door: this engine is a
//! general approximation routine, and the only way to measure its
//! refinement, its walls, its stall rule and each limb of its
//! classification is to run it at targets chosen for the measurement —
//! 1e-2 through 1e-18, and bounds derived from a measured residual.
//! One committed ε cannot express any of that, and rewriting those rows
//! to make the GEOMETRY worse instead would delete the evidence rather
//! than move it. The suites in `crates/geom-brep/tests/` and
//! `crates/sweep/tests/` are the whole population.
//!
//! **No production file other than this one reaches an `_at`
//! routine.** Here, each `Tol` door delegates to its `_at` twin at
//! `precision_target(tol)`; everywhere else, an approximating surface
//! stores no tolerance, so every production classification — the mint,
//! the validator's re-derivation and the transform door's remap — goes
//! through a `Tol` door. `crates/topo/tests/shell_tolerance_chain.rs`
//! holds that as a census, exempting this file by name.
//!
//! # Discipline
//!
//! The whole stack is **f64 substrate**: fitting is C6 structure
//! selection (same inputs ⇒ same knots and control bits, D9), and the
//! certification's hull bounds are `f64` upper bounds by construction — the
//! `SsiCertificate::hull_sup` posture. Predicates decide through the
//! kernel's one classification funnel.

use geom::curves::fit::{FitError, interpolate_columns};
use geom::surfaces::{NurbsSurface, Surface};
use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::max_bound;
use geom_core::spline::algebra::equal_split_points;
use geom_core::spline::compose::patch::PatchSpans;
use geom_core::spline::{KnotVector, SplineError};
use geom_core::{
    Band, BandError, Interval, KERNEL_LIMIT_LAST_RESORT, KERNEL_LIMIT_RECOURSE, Point3, Tol,
};

use crate::descent::{self, Descent, Work};
use crate::offset_meters::{MeterError, MeterResult, meter_patch, mig};
use crate::patch_bound::{PatchBoundError, check_direction, unit_scaled_weights};
use crate::recourse::{NARROW_SPAN, NARROW_SPAN_RECOURSE, Reading};
use geom_core::spline::{Derived, TensorCoeffs};

/// The fitted surface's degree in both directions. A CONSTANT (D9:
/// structure, never data-dependent tuning). Bicubic is the kernel's
/// fitting degree everywhere else (`SSI_FIT_DEGREE`), and it is the
/// lowest degree that reproduces an offset's curvature variation
/// without the fit's own wiggle dominating the residual.
pub const OFFSET_FIT_DEGREE: usize = 3;

/// How many equal pieces each nonempty span of the base is cut into
/// to seed the sample parameters, per direction, before the first
/// fit. A CONSTANT (D9).
pub const OFFSET_FIT_SEED_PER_SPAN: usize = 3;

/// The cap on every sample grid the fit interpolates: the product of
/// its per-direction sample counts, `us.len() · vs.len()`, the seed's
/// included.
///
/// It bounds the size of each fit, and with it the cost of each round:
/// the interpolation solves and the certificate's cells both scale with
/// the sample count. The total work is [`OFFSET_FIT_WORK`]'s.
///
/// A refinement round bisects, in each worst-carrying cell, the sample
/// intervals of the ONE direction that cell's model-space extent names,
/// so the grid can grow in one direction while its partner stands
/// still. The product admits that anisotropy up to the same total a
/// square grid has. A direction never loses samples, so neither
/// direction exceeds the cap divided by its partner's seed count — 576
/// against the `OFFSET_FIT_DEGREE + 1` samples an interpolation needs,
/// since a seed with fewer refuses at round 0's interpolation.
///
/// This constant is the lever of [`OffsetFitError::SeedOverCap`], raised
/// before round 0 when the seed alone exceeds it, and of
/// [`OffsetFitError::WallReached`] at [`OffsetFitWall::SampleCap`],
/// raised when the NEXT round's schedule would; a wall stop with a
/// non-finite bound on the grid it stopped on is
/// [`OffsetFitError::BoundNotFinite`] instead. Never an uncertified
/// return.
pub const OFFSET_FIT_SAMPLE_CAP: usize = 48 * 48;

/// The fit's cumulative work budget: the sample counts of every grid
/// its rounds fit, summed, the seed's included. A round runs only if
/// its grid fits in what is left. Where [`OFFSET_FIT_SAMPLE_CAP`] bounds
/// one fit, this bounds the wait for all of them: it is what stops a
/// loop whose grid grows by a sample a round, which the cap alone lets
/// run for 760 rounds (`scripted_rounds`).
///
/// No measured run reaches it. The most any fitted (2026-10-12, over
/// the offset-fit rows of geom-brep and sweep at the default ε and
/// 1e-12) is 8851 samples, the quarter cylinder at `d = -2e-8` and
/// 1e-15, which stops on the cap after 10 rounds; a budget under a
/// run's fitted samples stops it on an earlier round, so that row's
/// pinned rounds and grid (`a_bound_found_then_lost_carries_the_bound_it_lost`)
/// are the guard.
///
/// This constant is the lever of [`OffsetFitError::WallReached`] at
/// [`OffsetFitWall::Work`].
pub const OFFSET_FIT_WORK: usize = 16_384;

/// The per-direction on-locus sample count inside each certificate
/// cell (limb 1's fixed schedule, D9).
pub const OFFSET_CERT_SAMPLES: usize = 3;

/// Which limb of the certificate refused — so a consumer (and the
/// acceptance suite) can tell them apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetLimb {
    /// Limb 1 — the sampled on-locus residual.
    OnLocus,
    /// Limb 2 — the certified control-hull sup bound.
    HullSup,
}

impl OffsetLimb {
    /// The limb's display name.
    pub fn name(self) -> &'static str {
        match self {
            Self::OnLocus => "limb 1 (on-locus residual)",
            Self::HullSup => "limb 2 (control-hull sup bound)",
        }
    }

    /// What the limb measures, as a refusal names it.
    fn measured(self) -> &'static str {
        match self {
            Self::OnLocus => "sampled error",
            Self::HullSup => "certified error bound",
        }
    }
}

/// The at-rest limb's one recourse ([`OffsetFitError::Limb`]).
pub const LIMB_REFIT_RECOURSE: &str = "Recourse: re-fit the offset at this tolerance";

/// A limb's measurement against its tolerance, as a refusal states it:
/// a NaN bound is no number of metres.
struct Against {
    bound: f64,
    tolerance: f64,
}

impl core::fmt::Display for Against {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { bound, tolerance } = self;
        if bound.is_nan() {
            write!(f, "is not a number, against a tolerance of {tolerance} m")
        } else {
            write!(f, "is {bound} m against a tolerance of {tolerance} m")
        }
    }
}

/// **The smallest bound the refinement loop reached**, and the grid it
/// was first reached on: what every refinement refusal reports
/// ([`OffsetFitError::WallReached`],
/// [`OffsetFitError::RefinementStalled`], and
/// [`OffsetFitError::BoundNotFinite`] when a round reached one), and
/// the size a last-resort loosening names where no other lever is left.
///
/// # The recourse claim
///
/// **At the fit's band, a request at `bound` certifies on the round
/// that reached it, on `grid`, and a looser request certifies no
/// later.** The loop's schedule never reads the tolerance: `measure`,
/// the stop rule and `refine_schedule` take none, and that signature
/// is the guard. Only the certify test reads it, so the run follows the
/// same rounds until that test passes.
///
/// The condition is the band, and it is not idle. The door meters read
/// it before any round, and their winning rung sets the regularity
/// floor `measure` divides by and the chart speeds the marking
/// compares. The [`Tol`] doors derive their band from the same witness
/// as their target (`Band::linear(tol)`), so a caller that loosens ε
/// to `bound` moves the band too, the rung can change, and `bound` is
/// then the size to ask for rather than a guarantee.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BestBound {
    /// The smallest finite certified sup bound any round reached, in
    /// metres; no larger than the refusal's `achieved`, where it has
    /// one, and above its tolerance.
    pub bound: f64,
    /// The sample grid `bound` was first reached on, per direction.
    pub grid: (usize, usize),
}

/// The work bound that stopped the offset fit's refinement loop
/// ([`OffsetFitError::WallReached`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetFitWall {
    /// The next round's grid would have carried more samples than
    /// [`OFFSET_FIT_SAMPLE_CAP`].
    SampleCap {
        /// The cap, `us.len() · vs.len()`.
        cap: usize,
    },
    /// The next round's grid would have passed [`OFFSET_FIT_WORK`],
    /// the samples fitted over all the rounds.
    Work {
        /// The budget.
        budget: usize,
        /// The samples the rounds that ran fitted.
        spent: usize,
    },
}

/// A typed refusal of the offset fit door (fail-loud; the kernel never
/// panics and never returns an uncertified surface).
///
/// # How the refinement loop ends without a certificate
///
/// The certify-and-insert loop of [`fit_offset_at`] (and so of every
/// production door over it) leaves without a certificate in six ways.
/// Two are propagations from a round's own work and say nothing about
/// the loop: the interpolation's refusals ([`Self::Fit`],
/// [`Self::Structure`], [`Self::NonFiniteSample`]) and the certificate
/// assembly's ([`Self::PatchBound`]). The other four are the loop's
/// own terminations, each a face whose doc below names the kernel
/// lever that would have changed it. They are the crate's stop rule
/// ([`crate::descent`]), with a stall speaking over a wall on the
/// round both fire.
///
/// 0. **The seed grid is already over the sample cap** —
///    [`Self::SeedOverCap`], before any round; the lever is
///    [`OFFSET_FIT_SAMPLE_CAP`], and the caller's is a face split.
/// 1. **A work bound stopped the next round** —
///    [`Self::WallReached`], naming which: the sample cap on the next
///    grid ([`OFFSET_FIT_SAMPLE_CAP`]) or the cumulative budget over
///    all the rounds ([`OFFSET_FIT_WORK`]). Each is the lever of its
///    own wall. A stall on the same round speaks instead, so this face
///    never speaks for a round the stall rule refused.
/// 2. **Two rounds in a row did not lower the lowest bound, or the
///    both-directions step could not grow the schedule at all** —
///    [`Self::RefinementStalled`]; no lever, more rounds cannot help.
/// 3. **The last round's bound is not finite** —
///    [`Self::BoundNotFinite`], when a wall stopped the loop or on a
///    stall with no finite round behind it; it
///    carries the smallest finite bound any round
///    reached in place of an `inf`, and its lever is the schedule
///    when there was one and no constant of this loop when there was
///    not.
///
/// The levers above are the kernel's. The walls are not a caller's to
/// change, so each face's message names the caller's repair
/// instead: a face split into pieces that need less refinement where
/// more refinement was still paying (either wall), or, when no bound was
/// ever finite, an
/// offset distance of larger magnitude. A face whose last round did not
/// fall has no such lever: nothing in the loop's state says a smaller
/// piece would fall further. It names loosening the tolerance to no
/// tighter than the smallest bound any round reached ([`BestBound`],
/// whose doc states when a request there certifies) as the last resort
/// a kernel approximation limit leaves, with the note that the refusal
/// may be a kernel bug ([`geom_core::KERNEL_LIMIT_LAST_RESORT`]).
///
/// **D2 classification: row 1**, stated here once for the four faces
/// (their own docs point back here rather than restating it). Every
/// input that any face refuses is refused — the admission set of the
/// door is unchanged by which face speaks — and the split exists so
/// the caller learns which knob the refusal is about.
///
/// **A face row that moves owes no addendum, and the reason is
/// measured rather than structural.** The CELL bound only tightens —
/// it is a max over the reading it replaces, so on a fixed grid no
/// cell can rise, which is proved by construction and measured cell
/// by cell. The DOOR's bound is not monotone in it: the refinement
/// marking reads the round's sup, so tightening cells by different
/// factors reorders which of them clear the cut, and a different
/// schedule interpolates a different fit (`measure`, where the cut is
/// taken).
/// A request whose tolerance sits between the old door bound and the
/// new one would therefore cross INTO a refusal face, and what rules
/// that out here is a corpus measurement: over 70 requests (the
/// quarter cylinder and a bumpy patch, 7 deltas x 5 tolerances each),
/// one bound rose, by 1.8%, three orders below the tolerance it was
/// asked for, and no request crossed in. Every request that crossed OUT is
/// certified by the same decomposition that refused it.
#[derive(Clone, Debug, PartialEq)]
// The variant roster `topo`'s sample-coverage row reads (this
// crate's `test-support` feature, test builds only).
#[cfg_attr(
    feature = "test-support",
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(OffsetFitErrorKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum OffsetFitError {
    /// A door meter refused: the patch's normal is not certifiably
    /// non-degenerate, or `|d|` reaches its curvature reach.
    Meter(MeterError),
    /// The patch-bound assembly refused (a C⁰ crease, a degree-0
    /// direction, an illegal rational description).
    PatchBound(PatchBoundError),
    /// The interpolation stack refused. Its input is the kernel's own:
    /// the samples and parameters the refinement schedule chose, taken
    /// from a face whose non-finite points refuse earlier
    /// ([`Self::NonFiniteSample`]). So the carrier's repairs, addressed
    /// to a caller supplying data, name nothing the user supplied, and
    /// the carrier rides in `Debug`.
    ///
    /// A seed with too few distinct parameters
    /// ([`FitError::TooFewPoints`]) or a repeated one
    /// ([`FitError::ParamCountMismatch`], whose counts the fit always
    /// pairs, so only its ordering half is left) is [`NARROW_SPAN`]. No
    /// other arm is shown reachable or excluded — a collocation pivot
    /// that does not certify, a solve that overflows — so each ends in
    /// the last resort
    /// (`encl/offset-fit-unproven-refusals-end-in-the-last-resort`).
    Fit(FitError),
    /// Spline structure construction refused, on knots and control
    /// points the fit computed over the face's validated domain. Not
    /// shown unreachable: [`KnotVector::on_domain`] refuses averaged
    /// knots that rounding collapses onto one value, which sample
    /// parameters bisected down to adjacent floats could reach, so it
    /// ends in the last resort, as [`Self::Fit`]'s unproven arms do
    /// (`encl/offset-fit-unproven-refusals-end-in-the-last-resort`).
    Structure(SplineError),
    /// `d` or the tolerance is not a finite, non-zero (resp.
    /// positive) number. Both are the call's own arguments, so a
    /// non-finite one is usually a derived quantity that went
    /// non-finite upstream of the request.
    InvalidRequest {
        /// The offset distance as supplied.
        d: f64,
        /// The tolerance as supplied.
        tolerance: f64,
    },
    /// A sampled offset point is non-finite: the base evaluated to
    /// poison at a sample the whole-patch meters admitted in bound, so
    /// the base's own description is what to repair, not the request.
    NonFiniteSample {
        /// The offending parameters.
        uv: (f64, f64),
    },
    /// A work bound stopped the loop, [`OffsetFitWall`] naming which:
    /// the next round's schedule would have carried more samples than
    /// the cap, or would have passed the cumulative budget, while the
    /// stall rule had not refused. Carries the FINITE bound achieved on
    /// the grid the loop stopped on (a stop whose bound is not finite
    /// is [`Self::BoundNotFinite`]). Classification: the enum's, above.
    WallReached {
        /// Which work bound stopped the next round.
        wall: OffsetFitWall,
        /// How many refinement rounds ran before the wall stopped the
        /// next one — the same count a certificate's `rounds` carries.
        rounds: u32,
        /// The sample grid the loop stopped on, per direction.
        grid: (usize, usize),
        /// The certified sup bound on that grid, in metres. Finite.
        achieved: f64,
        /// The tolerance it had to reach.
        tolerance: f64,
        /// The smallest bound any round reached ([`BestBound`]).
        best: BestBound,
    },
    /// The seed grid alone carries more samples than
    /// [`OFFSET_FIT_SAMPLE_CAP`]: the base has so many knot spans that
    /// seeding [`OFFSET_FIT_SEED_PER_SPAN`] pieces in each exceeds the
    /// cap before any fit is made. Refused before round 0, so there is
    /// no bound to carry. Classification: the enum's, above.
    SeedOverCap {
        /// The sample cap, `us.len() · vs.len()`.
        cap: usize,
        /// The seed grid, per direction.
        grid: (usize, usize),
    },
    /// The loop stopped with the bound on its last grid not finite —
    /// on a wall, or on a stall with no finite round behind it:
    /// the certifying limb answered `+∞` there, so there is no achieved
    /// bound on that grid, and the type carries `best` in place of an
    /// `inf` where the caller needs a number. Two cases, told apart by
    /// that field, because they send the caller to different places:
    ///
    /// - **`best: None` — no round ever produced a finite
    ///   bound.** A cell's sign witness or its lower bound on `‖E‖`
    ///   could not be proved at the sampled cells on any grid, and a
    ///   finer schedule did not change that; the module docs'
    ///   small-`|d|` limit is the shape of it. The lever is NOT a
    ///   constant of this loop, so the walls are not worth raising. What
    ///   decides it is the limb's floors against the request's own
    ///   `|d|`: the regularity floor the door meters certify and the
    ///   floor on `‖E‖` — the larger of the componentwise mignitude
    ///   assembly and the projection through the sign witness. The
    ///   caller's move is there, not at a wall. Every instance the
    ///   shipped corpus reaches stops on the cap. The stall route, a
    ///   stall with no finite round behind it, has no row and none was
    ///   constructed: no round is idle until one is finite, so on a
    ///   never-finite run the only stall is the
    ///   schedule's exhaustion, a both-directions marking that grows no
    ///   direction. That needs every marked sample interval narrowed to
    ///   adjacent floats, where `bisect` drops the midpoint, and each
    ///   halving is a round of its own; or a composite with no cells,
    ///   to which a clamped knot pair never decomposes.
    /// - **`best: Some(b)` — a coarser grid reached the finite
    ///   bound `b.bound` and a finer one lost it.** The bound was
    ///   there and the schedule moved off it, so the schedule is the
    ///   lever and `b.bound` is the number the caller can size
    ///   against ([`BestBound`]). An infinite round after a finite one
    ///   is idle, so the stall rule answers the first with the
    ///   both-directions step and a second with the stall's face; this
    ///   case is a wall stop on the first. The quarter cylinder at
    ///   `d = -2e-8` and `1e-15` reaches it (`tests/offset_fit.rs`).
    ///
    /// Classification: the enum's, above.
    BoundNotFinite {
        /// How many refinement rounds ran before the loop stopped.
        rounds: u32,
        /// The sample grid the loop stopped on, per direction.
        grid: (usize, usize),
        /// The offset distance the request was for, in metres.
        d: f64,
        /// The tolerance it had to reach.
        tolerance: f64,
        /// The smallest bound any round reached ([`BestBound`]);
        /// `None` when no round's bound was finite.
        best: Option<BestBound>,
    },
    /// The refinement loop stopped IMPROVING: two rounds in a row did
    /// not lower the lowest bound any round reached, the second on a
    /// grid that bisected every failing cell in both directions (the
    /// crate's stop rule, [`crate::descent`]), or that step could not
    /// grow the schedule at all.
    ///
    /// A different finding from [`Self::WallReached`], whose last
    /// round was still falling or had not yet taken this step. This says the
    /// strongest step the loop has gained nothing, or could not be
    /// taken, so more rounds are not the lever: the tolerance is below
    /// what this fit's structure reached on this patch, and `best` is
    /// what it did reach.
    ///
    /// Classification: the enum's (row 1, stated once above) — and
    /// this face is where row 0 was answered first and answered no:
    /// "the bound stopped falling" is a measured numeric outcome on
    /// admissible input, so no type change can exclude it without
    /// making convergence a type-level property of caller-supplied
    /// geometry.
    ///
    /// *The minority reading, recorded because it becomes correct.*
    /// Row 2 (`Unsupported*`, valid-but-unbuilt) was argued on the
    /// strength of this variant's own wording — the tolerance is
    /// below what "this fit's structure" can reach, which sounds like
    /// a capability the kernel has not built. It is not one TODAY:
    /// the structure is fixed at [`OFFSET_FIT_DEGREE`] with schedule
    /// refinement as the only lever, so there is nothing unbuilt to
    /// reach for and the refusal is about the request. The day the
    /// banked compression half of A9.10 lands (knot removal under
    /// Eqs. 9.86–9.89, module docs), the door gains a second lever
    /// over the fitted structure, this refusal starts meaning "the
    /// structure this door is willing to build cannot reach it", and
    /// it should be RECLASSIFIED to row 2 then.
    RefinementStalled {
        /// How many refinement rounds ran before the stall.
        rounds: u32,
        /// The sample grid the loop stalled on, per direction.
        grid: (usize, usize),
        /// The certified sup bound on that grid, in metres. Not
        /// necessarily finite: a round that lost its bound did not
        /// lower it either.
        achieved: f64,
        /// The tolerance it had to reach.
        tolerance: f64,
        /// The smallest bound any round reached ([`BestBound`]).
        best: BestBound,
    },
    /// A certification was asked for over a window that is not the
    /// base's own chart rectangle. This module's derivation covers that
    /// rectangle and nothing narrower, so a sub-window claim would be a
    /// bound it never proved. Raised by [`certify_offset_over`], and so
    /// by every door that goes through it — the storage mint, the
    /// validator's re-derivation and the transform door's lane.
    WindowUnsupported {
        /// The window asked for.
        window: geom::ApproxWindow,
    },
    /// A certificate limb refused on a fit handed in from outside —
    /// the re-derivation door ([`certify_offset`]). An on-locus max
    /// above the tolerance is a fit wrong where the samples looked; a
    /// hull sup above it is a bound too weak between them. Either way
    /// the stored certificate is not accepted, and re-fitting through
    /// [`fit_offset`] at this tolerance is the repair.
    Limb {
        /// Which limb.
        limb: OffsetLimb,
        /// The bound that limb measured, in metres.
        bound: f64,
        /// The tolerance it was classified against.
        tolerance: f64,
    },
    /// A certificate limb refused on the fit the mint itself just
    /// made, at [`OffsetLimb::OnLocus`]: a round's sampled residual
    /// came out NaN inside the fit loop, or the mint's certification
    /// sampled the accepted fit above the tolerance its hull bound
    /// already held (the loop accepts on the bound, and limb 1 only
    /// steers it). [`OffsetLimb::HullSup`] is not reached: the mint's
    /// certification is the loop's measure on the same fit, so it
    /// re-derives the bound the loop accepted. Nothing is stored yet, so
    /// re-fitting is the call that refused; it is told apart from
    /// [`Self::Limb`] by its door. Neither arm is shown unreachable: a
    /// NaN residual needs an evaluation of a fit solved from finite
    /// samples to overflow, and the sampled residual is `f64`
    /// arithmetic on the model's coordinates while the bound the loop
    /// accepted is certified, so a sample's rounding above a bound just
    /// under the tolerance is not excluded. It ends in the last resort
    /// (`encl/offset-fit-unproven-refusals-end-in-the-last-resort`).
    MintLimb {
        /// Which limb.
        limb: OffsetLimb,
        /// The bound that limb measured, in metres.
        bound: f64,
        /// The tolerance it was classified against.
        tolerance: f64,
    },
    /// The run's tolerance could not form the linear band the door
    /// meters classify against. Both of `Band::linear`'s arms reach it
    /// from a tolerance the run's validator admits: an ε within a
    /// factor K of `f64::MAX`, and a subnormal ε with K near 1 (see
    /// `Band::linear`'s error docs for the exact region). Only the
    /// [`Tol`] doors derive a band; the `_at` instruments take theirs
    /// as an argument. `tests/offset_fit_band_probes.rs` drives
    /// [`fit_offset`] and [`certify_offset`] here at both arms.
    Band(BandError),
}

impl From<MeterError> for OffsetFitError {
    fn from(e: MeterError) -> Self {
        Self::Meter(e)
    }
}

impl From<MeterResult> for OffsetFitError {
    fn from(e: MeterResult) -> Self {
        match e {
            MeterResult::Meter(m) => Self::Meter(m),
            MeterResult::PatchBound(p) => Self::PatchBound(p),
        }
    }
}

impl From<PatchBoundError> for OffsetFitError {
    fn from(e: PatchBoundError) -> Self {
        Self::PatchBound(e)
    }
}

impl From<FitError> for OffsetFitError {
    fn from(e: FitError) -> Self {
        Self::Fit(e)
    }
}

/// What [`OffsetFitError::Meter`] says before its meter's refusal.
const METER_UNFITTED: &str = "the offset surface cannot be fitted";

impl OffsetFitError {
    /// This refusal as `Display` renders it, except that a meter's
    /// refusal ends in [`MeterError::render_with_lever`]'s words: for a
    /// door whose user sets the offset under another name.
    #[must_use]
    pub fn render_with_lever(&self, lever: &'static str) -> String {
        match self {
            Self::Meter(e) => format!(
                "{METER_UNFITTED}: {}",
                e.render_with_lever(lever, Reading::Build)
            ),
            other => other.to_string(),
        }
    }
}

impl core::fmt::Display for OffsetFitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Meter(e) => write!(f, "{METER_UNFITTED}: {}", e.render(Reading::Build)),
            Self::PatchBound(e) => write!(f, "{e}"),
            // The carriers' own prose is not rendered: their repairs
            // are addressed to a caller supplying samples or a spline,
            // and here the kernel supplied both (variant docs).
            Self::Fit(FitError::TooFewPoints { .. } | FitError::ParamCountMismatch { .. }) => {
                write!(
                    f,
                    "the offset surface's fit could not place distinct samples on the face: \
                     its surface has {NARROW_SPAN}. {NARROW_SPAN_RECOURSE}"
                )
            }
            Self::Fit(_) => write!(
                f,
                "the offset surface's fit could not interpolate the samples it chose on the \
                 face. {KERNEL_LIMIT_RECOURSE}"
            ),
            Self::Structure(_) => write!(
                f,
                "the offset surface's fit could not assemble a spline from the knots and \
                 points it computed. {KERNEL_LIMIT_RECOURSE}"
            ),
            Self::InvalidRequest { d, tolerance } => write!(
                f,
                "the offset cannot be fitted with an offset distance of {d} m and a \
                 tolerance of {tolerance} m. Recourse: supply a finite, non-zero offset \
                 distance and a finite, positive tolerance"
            ),
            Self::NonFiniteSample { uv } => write!(
                f,
                "the face's surface has no finite offset point at surface parameters \
                 ({}, {}), so its offset cannot be fitted. Recourse: repair the face's \
                 surface description there",
                uv.0, uv.1
            ),
            Self::WallReached {
                wall,
                tolerance,
                best: BestBound { bound: best, .. },
                ..
            } => {
                match wall {
                    OffsetFitWall::SampleCap { cap } => write!(
                        f,
                        "the offset surface's fit reached its limit of {cap} samples"
                    )?,
                    OffsetFitWall::Work { budget, .. } => write!(
                        f,
                        "the offset surface's fit reached its limit of {budget} samples \
                         fitted in total"
                    )?,
                }
                write!(
                    f,
                    ", and its best certified error was {best} m against a tolerance of \
                     {tolerance} m. Recourse: split the face so each piece needs fewer samples"
                )
            }
            Self::SeedOverCap { cap, grid } => write!(
                f,
                "the offset surface's fit would start from {} x {} samples, more than its \
                 limit of {cap} samples, because the face's surface has that many knot spans. \
                 Recourse: split the face",
                grid.0, grid.1
            ),
            Self::BoundNotFinite {
                d,
                tolerance,
                best: None,
                ..
            } => write!(
                f,
                "no refinement of the offset surface's fit could bound its error at an \
                 offset distance of {d} m, so it cannot be certified to {tolerance} m. \
                 Recourse: use an offset distance of larger magnitude"
            ),
            Self::BoundNotFinite {
                tolerance,
                best: Some(BestBound { bound: b, .. }),
                ..
            } => write!(
                f,
                "the offset surface's fit bounded its error at {b} m on coarser sampling \
                 and lost that bound on finer sampling, so it cannot be certified to \
                 {tolerance} m. Recourse: loosen the tolerance to {b} m or more, \
                 {KERNEL_LIMIT_LAST_RESORT}"
            ),
            Self::RefinementStalled {
                tolerance,
                best: BestBound { bound: best, .. },
                ..
            } => write!(
                f,
                "the offset surface's fit stopped improving: refining it in both directions \
                 either did not lower its best certified error or added no samples, and that \
                 error was {best} m against a tolerance of {tolerance} m. Recourse: \
                 loosen the tolerance to {best} m or more, {KERNEL_LIMIT_LAST_RESORT}"
            ),
            Self::WindowUnsupported { window } => write!(
                f,
                "the offset certificate was asked for over part of the face's surface \
                 (u {:?}, v {:?}), and it covers only the whole surface. Recourse: ask \
                 for it over the whole surface",
                window.u, window.v
            ),
            Self::Limb {
                limb,
                bound,
                tolerance,
            } => write!(
                f,
                "a stored offset surface does not hold to the tolerance: its {} {}. \
                 {LIMB_REFIT_RECOURSE}",
                limb.measured(),
                Against {
                    bound: *bound,
                    tolerance: *tolerance
                }
            ),
            Self::MintLimb {
                limb,
                bound,
                tolerance,
            } => write!(
                f,
                "the offset surface the door just fitted does not certify: its {} {}. \
                 {KERNEL_LIMIT_RECOURSE}",
                limb.measured(),
                Against {
                    bound: *bound,
                    tolerance: *tolerance
                }
            ),
            // The carrier's own prose is not rendered: its repairs
            // (set a positive ε; raise ε or K) are addressed to a
            // caller choosing a band's thresholds, and a `Tol` caller
            // already holds a valid tolerance whose derived band
            // failed anyway. What is true for every arm `Band::linear`
            // returns is that the run's ε sits at an extreme of the
            // range, and a less extreme one forms a band at any
            // admitted K.
            Self::Band(_) => write!(
                f,
                "the offset surface's meters cannot classify at the run's tolerance: it is \
                 too extreme for the ambiguity band above it to form. Recourse: run at a less \
                 extreme tolerance"
            ),
        }
    }
}

impl std::error::Error for OffsetFitError {}

// The certificate RECORD lives in `geom` (`geom::OffsetCertificate`):
// `ApproxSurface` stores it and `Surface` stores that, so the type has
// to sit below the surface enum. Its derivation is this module's, and
// every limb below writes into it.
//
// The DERIVATION is `f64`-only — every door here is monomorphic at
// `f64`, taking `NurbsSurface<f64>` or `ApproxSurface<f64>` and never a
// `T: Real` — but the RECORD is not confined to that scalar.
// It carries no scalar parameter of its own, and
// `ApproxSurface::map_scalar` carries it verbatim onto a lifted
// surface, so a certificate DOES reach consumers at scalars this
// module never runs at. It arrives there as provenance and never as
// authority: at `f64` the validator re-derives against the
// description and never consults the stored copy, and at a scalar
// with no re-derivation lane — `topo::AtRestPolicy::offset_fit_lane`
// answering `None` — tier 3 REFUSES the face with
// `ValidationError::ApproxLaneUnsupported` rather than accepting the
// carried record. That refusal is about the derivation missing at
// that scalar, never about a value that could not arrive.
pub use geom::OffsetCertificate;

// SHELL-TOLERANCE-CHAIN BEGIN — the sentinel
// `topo/tests/shell_tolerance_chain.rs` reads. Between here and the END
// sentinel are this module's five PRODUCTION doors, the one site that
// turns the run's witness into a number, and the numeric-target `_at`
// routines each door delegates to. No PRODUCTION signature here may
// take an `f64` epsilon, and the region may hold exactly one `.eps()`
// read. The `_at` routines are exempt from the first rule and only
// from it: taking a chosen target is what they are for, they are
// `#[doc(hidden)]`, and a separate census holds that no production file
// outside this one reaches one. Do not move a production door out of
// this region.
/// **The fit target: the run's ε, read through the [`Tol`] witness**
/// (D4 ¶1's witness rule; D4 ¶2's ε_precision; the residual O3
/// ratifies).
///
/// The invariant is ONE ε on the chain that reaches this module, the
/// one the run committed, read only through the witness whichever
/// function reads it: this one for the fit target, and `Band::linear`
/// for the meters' band ([`run_band`]). From the shell door down to
/// here the tolerance travels as the witness and nothing else —
/// `topo::shell`, the face-replacement doors, `topo::props`'s lane
/// doors and this module's five production doors all name it in their
/// signatures — so no caller on the way can name a second epsilon, and
/// none can do arithmetic on the one the run committed.
/// `crates/topo/tests/shell_tolerance_chain.rs` pins that this is the
/// only `.eps()` read in the guarded region.
fn precision_target(tol: Tol) -> f64 {
    tol.eps()
}

/// **The band the door meters classify against**, derived from the same
/// witness as [`precision_target`]: the run's linear band. A door that
/// took the band as a second argument would let a caller classify the
/// limbs at one ε and meter the regularity floor and the collapse reach
/// at another; deriving it here makes the pair one value.
fn run_band(tol: Tol) -> Result<Band, OffsetFitError> {
    Band::linear(tol).map_err(OffsetFitError::Band)
}

/// **The fit door**, at the run's ε_precision: fit the offset of `base`
/// at signed distance `d` until the measured residual meets the target,
/// or refuse typed. The docs of the routine it delegates to
/// ([`fit_offset_at`]) carry the method, the doors, the bound's
/// tightness and the refusal ladder; what is stated HERE is only which
/// target this form uses, because that is the whole difference between
/// the two.
///
/// # Errors
///
/// As [`fit_offset_at`], plus [`OffsetFitError::Band`] when the run's
/// tolerance forms no linear band.
pub fn fit_offset(
    base: &NurbsSurface<f64>,
    d: f64,
    tol: Tol,
) -> Result<(NurbsSurface<f64>, OffsetCertificate), OffsetFitError> {
    fit_offset_at(base, d, precision_target(tol), run_band(tol)?)
}

/// [`fit_offset`] against a CHOSEN target rather than the run's ε — the
/// engine as an instrument (module docs, "the numeric-target
/// instrument"). The method both forms run is documented here.
///
/// The method: fit a NURBS approximation of `S + d·n` over the base's
/// own chart rectangle and certify it.
///
/// Refuses — never degrades — on a patch whose chart normal is not
/// certifiably non-degenerate, on an offset distance that reaches the
/// patch's curvature reach, on reaching the sample cap, and on a refinement
/// loop whose strongest step stops lowering the bound.
///
/// Geometry only: no `Surface` variant, no storage, no topology. The
/// base and `d` travel as arguments; the intensional
/// `Offset { base, d }` description is the integration unit's.
///
/// # How much slack the certificate carries
///
/// `hull_sup` is an upper bound, and a consumer sizing a budget from
/// it will want to know by how much it exceeds the residual actually
/// achieved. Measured against a dense sample on the shipped fixtures:
/// **2.1x to 7.3x** (non-analytic bicubic 2.1x, quarter cylinder 2.8x
/// at both signs, sphere band 4.4x outward and 7.3x inward), and 3.3x
/// on coarse single-cell schedules. Chart aspect does NOT drive it —
/// 5:1 and 1:5 quarter cylinders both measure 3.3x, the same as 1:1.
///
/// The one regime that departs is small `|d|`: at `d = 1e-6` on a
/// metre patch the ratio is ~1.6e3, for the reason the module docs
/// give under recentring — the bound is then dominated by its
/// `τ²/‖E‖` term through a componentwise lower bound on `‖E‖`, and
/// that is a property of the bound rather than of the fit, which is
/// accurate to ~2e-7 there.
///
/// No row pins these ratios and none is owed: they are a measurement
/// of the enclosure's tightness, not a claim the door makes. They are
/// recorded because a consumer reading `hull_sup` as "the error"
/// would otherwise over-provision by roughly half an order, and by
/// three orders at micron offsets.
///
/// # Errors
///
/// [`OffsetFitError`] — the two door meters and their escalations,
/// the patch-bound refusals, the interpolation stack's refusals,
/// non-finite samples, and the refinement loop's four terminations —
/// [`OffsetFitError::SeedOverCap`] before any round,
/// [`OffsetFitError::WallReached`] and
/// [`OffsetFitError::RefinementStalled`] each carrying the last grid's
/// bound and the smallest any round reached, and
/// [`OffsetFitError::BoundNotFinite`] carrying that smallest finite
/// bound, or none; and [`OffsetFitError::MintLimb`] at
/// [`OffsetLimb::OnLocus`] when a round's sampled residual is NaN.
#[doc(hidden)]
pub fn fit_offset_at(
    base: &NurbsSurface<f64>,
    d: f64,
    tolerance: f64,
    band: Band,
) -> Result<(NurbsSurface<f64>, OffsetCertificate), OffsetFitError> {
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !d.is_finite() || d == 0.0 || !(tolerance > 0.0) || !tolerance.is_finite() {
        return Err(OffsetFitError::InvalidRequest { d, tolerance });
    }
    // The doors, decided BEFORE any fit (DESIGN.md's pre-construction
    // stance): the offset locus must exist, and it must not fold.
    // The ladder is the meters' own (`OFFSET_METER_LADDER`).
    let (reg, coll) = meter_patch(base, d, band)?;

    let (fit, report, round) = refine_rounds_at(
        seed_params(base),
        (reg.speed_u.get(), reg.speed_v.get()),
        d,
        tolerance,
        |us, vs| {
            let fit = interpolate_offset_grid(base, d, us, vs)?;
            let report = measure(base, &fit, d, reg.floor)?;
            Ok((fit, report))
        },
    )?;
    #[allow(clippy::cast_possible_truncation)]
    let cert = OffsetCertificate {
        distance: d,
        cells: report.cells,
        samples: OFFSET_CERT_SAMPLES as u32,
        on_locus_max: report.on_locus_max,
        hull_sup: report.hull_sup,
        normal_floor: reg.floor,
        curvature_reach: coll.reach,
        rounds: round as u32,
    };
    Ok((fit, cert))
}

/// The refinement loop `fit_offset_at` runs from the seed schedule
/// `(us, vs)`, and so part of that numeric-target instrument (its
/// chosen `tolerance` is the point): `round_at` fits and measures one
/// schedule, and every
/// decision about the next one — the certificate, the stop rule, the
/// marking and its fallback — is taken here. Returns the certifying
/// round's fit, report and round count. Generic over the fit so the
/// loop's ordering can be driven by a scripted bound sequence (this
/// module's tests), which no fixture grounded in the enclosure's width
/// holds for long.
///
/// **The stop rule is the crate's** ([`crate::descent`]): a round that
/// does not lower the lowest bound is idle, the second idle round in a
/// row is [`OffsetFitError::RefinementStalled`], the cap and the
/// cumulative budget are [`OffsetFitError::WallReached`], and a stall
/// speaks over a wall on the round both fire. This loop's action on
/// the first idle round is the both-directions marking
/// ([`Refine::BothDirections`]).
fn refine_rounds_at<S>(
    (mut us, mut vs): (Vec<f64>, Vec<f64>),
    (speed_u, speed_v): (f64, f64),
    d: f64,
    tolerance: f64,
    mut round_at: impl FnMut(&[f64], &[f64]) -> Result<(S, Report), OffsetFitError>,
) -> Result<(S, Report, usize), OffsetFitError> {
    // The lowest bound and the grid it was first reached on, which is
    // the grid `BestBound`'s recourse claim names.
    let mut descent: Option<Descent<(usize, usize)>> = None;
    let mut work = Work::new(OFFSET_FIT_WORK);
    // `round` counts the refinement rounds that produced the current
    // grid — the number a certificate and every refusal below carry.
    // The loop has no fall-through: every exit is a certificate or a
    // named refusal.
    let mut round = 0usize;
    // The cap reads every grid the loop fits, the seed's included.
    if us.len() * vs.len() > OFFSET_FIT_SAMPLE_CAP {
        return Err(OffsetFitError::SeedOverCap {
            cap: OFFSET_FIT_SAMPLE_CAP,
            grid: (us.len(), vs.len()),
        });
    }
    loop {
        let (fit, report) = round_at(&us, &vs)?;
        let achieved = report.hull_sup;
        let grid = (us.len(), vs.len());
        work.spend(grid.0 * grid.1);
        let rounds = Descent::after(descent.take(), achieved, grid);
        let best = Some(rounds.lowest())
            .filter(|(bound, _)| bound.is_finite())
            .map(|(bound, &grid)| BestBound { bound, grid });
        nan_residual_at(report.on_locus_max, tolerance)?;
        if achieved <= tolerance {
            return Ok((fit, report, round));
        }
        // Insert a sample parameter at the midpoint of every sample
        // interval a worst-carrying cell touches — the "knot
        // insertion on the worst spans" step (module docs), which
        // reaches the fitted knot vector through Eq. 9.8.
        //
        // **The direction is chosen, not both taken.** A failing cell
        // bisects the direction whose model-space extent
        // `h_d · sup‖S_d‖` is larger — the tessellation split
        // selection's own rule. A patch whose error is anisotropic
        // then pays a linear grid for a linear need instead of a
        // quadratic one. The speed ratio is a prediction, so an idle
        // round marks BOTH directions, and a second idle round after
        // that is the stall.
        let mode = if rounds.idle() == 0 {
            Refine::Directional
        } else {
            Refine::BothDirections
        };
        let mut next = refine_schedule(&us, &vs, &report, speed_u, speed_v, mode);
        // A directional marking can fail to grow the schedule even
        // though it marked intervals: `bisect` drops a midpoint that
        // is not strictly between its endpoints, which is what an
        // interval narrowed to consecutive floats gives. That takes the
        // idle round's fallback rather than escaping to a wall, but it
        // is not an idle round: idle is read on the bound alone, so the
        // round after it stalls only if it is the second idle one.
        if !next.grew(&us, &vs) && mode == Refine::Directional {
            next = refine_schedule(&us, &vs, &report, speed_u, speed_v, Refine::BothDirections);
        }
        let size = next.us.len() * next.vs.len();
        let wall = if size > OFFSET_FIT_SAMPLE_CAP {
            Some(OffsetFitWall::SampleCap {
                cap: OFFSET_FIT_SAMPLE_CAP,
            })
        } else if !work.admits(size) {
            Some(OffsetFitWall::Work {
                budget: OFFSET_FIT_WORK,
                spent: work.spent(),
            })
        } else {
            None
        };
        // Every stop goes through the stop rule, a stall over a wall.
        // Bisecting every failing cell in both directions that moves
        // nothing is the loop's exhaustion: no later round can move it
        // either — the intervals only narrow — so it stops as the stall
        // does, on the schedule rather than on the bound. A round that
        // marks nothing ends there too.
        if let Some(stop) = rounds.stop(!next.grew(&us, &vs), wall) {
            return Err(expiry(stop, round, grid, achieved, best, d, tolerance));
        }
        descent = Some(rounds);
        us = next.us;
        vs = next.vs;
        round += 1;
    }
}

/// Re-derives the certificate of an ALREADY fitted surface against a
/// base and `d`, and classifies both limbs against `tolerance` — the
/// validator posture (O5: never trust a stored certificate), and the
/// door a degraded fit is driven through.
///
/// # Errors
///
/// [`OffsetFitError::Limb`] naming the limb that measured above
/// tolerance, plus the door meters' and the patch-bound refusals, and
/// [`OffsetFitError::Band`] when the run's tolerance forms no linear
/// band.
pub fn certify_offset(
    base: &NurbsSurface<f64>,
    fit: &NurbsSurface<f64>,
    d: f64,
    tol: Tol,
) -> Result<OffsetCertificate, OffsetFitError> {
    certify_offset_at(base, fit, d, precision_target(tol), run_band(tol)?)
}

/// [`certify_offset`] against a CHOSEN target rather than the run's ε —
/// the engine as an instrument (module docs).
///
/// # Errors
///
/// As [`certify_offset`], less [`OffsetFitError::Band`]: the band is an
/// argument here.
#[doc(hidden)]
pub fn certify_offset_at(
    base: &NurbsSurface<f64>,
    fit: &NurbsSurface<f64>,
    d: f64,
    tolerance: f64,
    band: Band,
) -> Result<OffsetCertificate, OffsetFitError> {
    // The doors are re-derived too, on the same ladder: a stored
    // certificate is never trusted, and neither is the fact that the
    // patch was fittable at all (O5's posture).
    let (reg, coll) = meter_patch(base, d, band)?;
    let report = measure(base, fit, d, reg.floor)?;
    // Limb 1 first: a sampled max above tolerance is a fit that is
    // wrong where we looked, which is a different finding from a
    // bound that is merely too weak between the samples.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(report.on_locus_max <= tolerance) {
        return Err(OffsetFitError::Limb {
            limb: OffsetLimb::OnLocus,
            bound: report.on_locus_max,
            tolerance,
        });
    }
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(report.hull_sup <= tolerance) {
        return Err(OffsetFitError::Limb {
            limb: OffsetLimb::HullSup,
            bound: report.hull_sup,
            tolerance,
        });
    }
    #[allow(clippy::cast_possible_truncation)]
    Ok(OffsetCertificate {
        distance: d,
        cells: report.cells,
        samples: OFFSET_CERT_SAMPLES as u32,
        on_locus_max: report.on_locus_max,
        hull_sup: report.hull_sup,
        normal_floor: reg.floor,
        curvature_reach: coll.reach,
        rounds: 0,
    })
}

/// **The certifier of an approximating surface's `(description, fit,
/// window)` triple, and the window rule behind it, one home.** Every
/// door that certifies an offset fit against a described base goes
/// through here: the storage mint ([`approx_offset_surface`], whose
/// certifier closure calls it with the run's witness), the validator's
/// re-derivation ([`recertify_approx`], the same call on a surface's
/// own triple) and the transform door's remap
/// ([`crate::OffsetFitLane::remap`], the same call on a mapped triple).
///
/// The rule: [`certify_offset`] derives over the base's WHOLE chart
/// rectangle, so a `window` is honoured exactly when it IS that
/// rectangle. Checked here rather than attested at each caller —
/// three copies of one predicate is three chances for two doors to
/// disagree about the same surface, which is what a narrowed window
/// planted behind an honest certificate would exploit.
///
/// # Errors
///
/// [`OffsetFitError::WindowUnsupported`] for a window this derivation
/// does not cover, then whatever [`certify_offset`] refuses,
/// [`OffsetFitError::Band`] included.
pub fn certify_offset_over(
    description: &geom::SurfaceDescription<f64>,
    fit: &NurbsSurface<f64>,
    window: geom::ApproxWindow,
    tol: Tol,
) -> Result<OffsetCertificate, OffsetFitError> {
    certify_offset_over_at(
        description,
        fit,
        window,
        precision_target(tol),
        run_band(tol)?,
    )
}

/// [`certify_offset_over`] against a CHOSEN target rather than the run's
/// ε — the engine as an instrument (module docs).
///
/// # Errors
///
/// As [`certify_offset_over`], less [`OffsetFitError::Band`]: the band
/// is an argument here.
#[doc(hidden)]
pub fn certify_offset_over_at(
    description: &geom::SurfaceDescription<f64>,
    fit: &NurbsSurface<f64>,
    window: geom::ApproxWindow,
    tolerance: f64,
    band: Band,
) -> Result<OffsetCertificate, OffsetFitError> {
    let geom::SurfaceDescription::Offset { base, d } = description;
    if window != geom::ApproxWindow::of(&**base) {
        return Err(OffsetFitError::WindowUnsupported { window });
    }
    certify_offset_at(base, fit, *d, tolerance, band)
}

// ---------------------------------------------------------------------
// The exact offset point (the fit's data and limb 1's target)
// ---------------------------------------------------------------------

/// `S(u,v) + d·n(u,v)` with `n` the normalized chart normal — the
/// exact offset locus, evaluated. `None` for a non-finite result
/// (including a degenerate normal at the sample: poison in, refusal
/// out).
pub fn offset_point(base: &NurbsSurface<f64>, d: f64, u: f64, v: f64) -> Option<Point3<f64>> {
    let jet = base.ders(u, v);
    let m = jet.du.cross(jet.dv);
    let len = m.norm();
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if !(len > 0.0) || !len.is_finite() {
        return None;
    }
    let p = jet.point + m * (d / len);
    (p.x.is_finite() && p.y.is_finite() && p.z.is_finite()).then_some(p)
}

// ---------------------------------------------------------------------
// The storage door: fit, certify, and hand back a `Surface` variant
// ---------------------------------------------------------------------

/// **The approximating-surface door**: fit the offset of `base` at
/// signed distance `d`, certify the fit against the description, and
/// hand back the [`geom::ApproxSurface`] that stores both — the payload
/// of [`geom::Surface::Approx`], so a caller that stores it wraps it and
/// a caller that reads it needs no variant match.
///
/// The certificate is derived from the STORED pair — the description
/// that goes into the surface and the fit that goes into the surface —
/// by [`certify_offset`], not carried out of [`fit_offset`]'s
/// refinement loop. That costs one extra measure pass and buys the
/// property the private-field invariant is for: the certificate an
/// `ApproxSurface` holds is a certificate OF the `ApproxSurface`.
///
/// The one field that does not come from that re-derivation is
/// `rounds`, which is provenance of the FIT rather than a limb and
/// which no re-measurement can recompute: the loop's honest count
/// travels with the certificate rather than being flattened to `0`.
/// The certifier closes over the run's witness and checks the window
/// rather than attesting it ([`certify_offset_over`]).
///
/// # Errors
///
/// [`OffsetFitError`]: everything [`fit_offset`] refuses
/// ([`OffsetFitError::Band`] included), plus [`certify_offset`]'s
/// refusals on the fresh fit, a limb's as [`OffsetFitError::MintLimb`].
/// A rational fit takes
/// the same path as a polynomial one — the composite is weighted, so
/// rationality is not a refusal cause.
pub fn approx_offset_surface(
    base: std::sync::Arc<NurbsSurface<f64>>,
    d: f64,
    tol: Tol,
) -> Result<std::sync::Arc<geom::ApproxSurface<f64>>, OffsetFitError> {
    let fitted = fit_offset(&base, d, tol)?;
    mint(base, d, fitted, |description, fit, window| {
        certify_offset_over(description, fit, window, tol)
    })
}

/// [`approx_offset_surface`] against a CHOSEN target rather than the
/// run's ε — the engine as an instrument (module docs). Its certifier
/// closes over that target; the surface stores no tolerance either way.
///
/// # Errors
///
/// As [`approx_offset_surface`], less [`OffsetFitError::Band`]: the
/// band is an argument here.
#[doc(hidden)]
pub fn approx_offset_surface_at(
    base: std::sync::Arc<NurbsSurface<f64>>,
    d: f64,
    tolerance: f64,
    band: Band,
) -> Result<Surface<f64>, OffsetFitError> {
    let fitted = fit_offset_at(&base, d, tolerance, band)?;
    mint(base, d, fitted, |description, fit, window| {
        certify_offset_over_at(description, fit, window, tolerance, band)
    })
    .map(Surface::Approx)
}

/// [`fit_offset_at`]'s per-round guard, against its chosen target.
/// Limb 1 steers and does not gate the mint, but a certificate never
/// carries a poisoned field: a round whose sampled residual is NaN
/// refuses as the mint's. No fixture reaches it: the fit is interpolated
/// from finite data ([`OffsetFitError::NonFiniteSample`] refuses the
/// rest), and only an overflow in its solve or evaluation is left.
fn nan_residual_at(on_locus_max: f64, tolerance: f64) -> Result<(), OffsetFitError> {
    if on_locus_max.is_nan() {
        return Err(OffsetFitError::MintLimb {
            limb: OffsetLimb::OnLocus,
            bound: on_locus_max,
            tolerance,
        });
    }
    Ok(())
}

/// The storage step both mint forms share: the spec from the base, `d`
/// and the loop's fit, certified by `certifier` on the STORED triple,
/// with the loop's `rounds` carried
/// ([`geom::OffsetCertificate::carrying_rounds`]). A limb refusing
/// the fit the loop just accepted is the mint's own
/// ([`OffsetFitError::MintLimb`]).
fn mint(
    base: std::sync::Arc<NurbsSurface<f64>>,
    d: f64,
    (fit, loop_cert): (NurbsSurface<f64>, OffsetCertificate),
    certifier: impl FnOnce(
        &geom::SurfaceDescription<f64>,
        &NurbsSurface<f64>,
        geom::ApproxWindow,
    ) -> Result<OffsetCertificate, OffsetFitError>,
) -> Result<std::sync::Arc<geom::ApproxSurface<f64>>, OffsetFitError> {
    let spec = geom::SurfaceSpec {
        window: geom::ApproxWindow::of(&*base),
        description: geom::SurfaceDescription::Offset { base, d },
        fit,
    };
    let approx = geom::ApproxSurface::certify(spec, |description, fit, window| {
        certifier(description, fit, window)
            .map(|cert| cert.carrying_rounds(loop_cert.rounds))
            .map_err(|e| match e {
                OffsetFitError::Limb {
                    limb,
                    bound,
                    tolerance,
                } => OffsetFitError::MintLimb {
                    limb,
                    bound,
                    tolerance,
                },
                other => other,
            })
    })?;
    Ok(std::sync::Arc::new(approx))
}

/// **The re-derivation door** (O5's never-trust posture): re-runs
/// [`certify_offset`] against an approximating surface's own stored
/// description and fit, classified against the run's ε (`tol`).
///
/// The stored certificate is not read: **the classification tolerance
/// is the run's**, which is what lets tier 3 verify the ratified claim
/// (O3: the residual is `≤ ε_precision`) at the ε the validation call
/// runs at. A fit that has been degraded since it was minted —
/// coarsened, edited, transplanted — fails here with the limb that
/// caught it; so does one minted loose and re-derived at a tighter ε,
/// which is D4's blessed consequence of ε-tightening and the edge
/// machinery's exact behaviour.
///
/// The stored WINDOW is read, and it is the one stored datum that is:
/// it is not a claim to be re-measured but the statement of WHERE the
/// claim is made, so a surface asserting a rectangle this derivation
/// does not cover refuses here exactly as it does at the mint and at
/// the transform door ([`certify_offset_over`], the one home).
///
/// # Errors
///
/// As [`certify_offset_over`], [`OffsetFitError::Band`] included.
pub fn recertify_approx(
    approx: &geom::ApproxSurface<f64>,
    tol: Tol,
) -> Result<OffsetCertificate, OffsetFitError> {
    certify_offset_over(approx.description(), approx.fit(), approx.window(), tol)
}

/// [`recertify_approx`] against a CHOSEN target rather than the run's ε
/// — the engine as an instrument (module docs).
///
/// # Errors
///
/// As [`recertify_approx`], less [`OffsetFitError::Band`]: the band is
/// an argument here.
#[doc(hidden)]
pub fn recertify_approx_at(
    approx: &geom::ApproxSurface<f64>,
    tolerance: f64,
    band: Band,
) -> Result<OffsetCertificate, OffsetFitError> {
    certify_offset_over_at(
        approx.description(),
        approx.fit(),
        approx.window(),
        tolerance,
        band,
    )
}

// SHELL-TOLERANCE-CHAIN END.

// ---------------------------------------------------------------------
// The refinement loop's refusal constructors
// ---------------------------------------------------------------------
//
// Below the sentinel on purpose. The guarded region's charter is the
// five production doors, the one `.eps()` read and the `_at` routines,
// and the chain guard reads every `tolerance: f64` inside it as a
// signature on the shell chain. A private constructor that takes the
// tolerance it reports is not a door, so it lives here.

/// Which stopping condition ended the refinement loop without a
/// certificate.
type Stop = descent::Stop<OffsetFitWall>;

/// The refusal for a loop that `stop` ended after `rounds` refinement
/// rounds, with `achieved` measured on `grid` and `best` the smallest
/// finite bound any round reached.
///
/// Every face that carries a finite number carries `best`
/// ([`BestBound`]). [`OffsetFitError::BoundNotFinite`] speaks for a
/// wall stop whose last bound is not finite, and for a stall
/// with no finite round behind it, whose "loosen the tolerance" would
/// have no number to name. A stall with a finite round behind it keeps
/// the stall's face even when its own bound is `+∞`: the stall's
/// evidence is about the rounds, and `best` is still a bound reached.
///
/// The routing is one match over `(stop, best, achieved is finite)`
/// whose arms are pairwise disjoint, so their order decides nothing.
fn expiry(
    stop: Stop,
    rounds: usize,
    grid: (usize, usize),
    achieved: f64,
    best: Option<BestBound>,
    d: f64,
    tolerance: f64,
) -> OffsetFitError {
    // Every round adds a sample to a grid the cap bounds, so `rounds`
    // is below `OFFSET_FIT_SAMPLE_CAP` and the narrowing is exact.
    #[allow(clippy::cast_possible_truncation)]
    let rounds = rounds as u32;
    let not_finite = || OffsetFitError::BoundNotFinite {
        rounds,
        grid,
        d,
        tolerance,
        best,
    };
    match (stop, best, achieved.is_finite()) {
        (_, None, _) | (Stop::Wall(_), Some(_), false) => not_finite(),
        (Stop::Stall, Some(best), _) => OffsetFitError::RefinementStalled {
            rounds,
            grid,
            achieved,
            tolerance,
            best,
        },
        (Stop::Wall(wall), Some(best), true) => OffsetFitError::WallReached {
            wall,
            rounds,
            grid,
            achieved,
            tolerance,
            best,
        },
    }
}

// ---------------------------------------------------------------------
// A9.4 — global surface interpolation at the base's own parameters
// ---------------------------------------------------------------------

/// The seed sample parameters, per direction (module docs).
fn seed_params(base: &NurbsSurface<f64>) -> (Vec<f64>, Vec<f64>) {
    (
        seed_direction(base.knots_u_f64_interim()),
        seed_direction(base.knots_v_f64_interim()),
    )
}

/// One direction's seed: every distinct knot value of `kv`, ascending,
/// with the equal-split interior points
/// ([`equal_split_points`] at [`OFFSET_FIT_SEED_PER_SPAN`]) between
/// each pair of neighbours, then bisected up to the
/// `OFFSET_FIT_DEGREE + 1` parameters an interpolation needs.
///
/// Every interior point lies strictly inside its own span, so taking
/// the points below each knot value in turn places each one between
/// the two ends of the span it came from.
///
/// **This rests on `kv` being clamped.** Only then are the distinct
/// knot values exactly the span ends: the first and last are the
/// domain's ends, so no span lies outside them. [`KnotVector`] admits
/// clamped vectors only, and the debug assertion below restates that
/// premise where it is used.
fn seed_direction(kv: &KnotVector) -> Vec<f64> {
    debug_assert_eq!(
        (
            kv.knot_runs().next().map(|(t, _)| t),
            kv.knot_runs().next_back().map(|(t, _)| t)
        ),
        (Some(kv.domain().0), Some(kv.domain().1)),
        "the distinct knot values do not run from the domain's start to its end"
    );
    let mut interior = equal_split_points(kv, OFFSET_FIT_SEED_PER_SPAN)
        .into_iter()
        .peekable();
    let mut out = Vec::new();
    for (end, _) in kv.knot_runs() {
        while let Some(t) = interior.next_if(|t| *t < end) {
            out.push(t);
        }
        out.push(end);
    }
    // A degree-`p` interpolation needs `p + 1` parameters; a
    // single-span low-degree direction seeds too few without this.
    while out.len() < OFFSET_FIT_DEGREE + 1 {
        let next = bisect(&out, &vec![true; out.len().saturating_sub(1)]);
        if next.len() == out.len() {
            break;
        }
        out = next;
    }
    out
}

/// The sample list with a midpoint inserted into every interval whose
/// index is marked — the refinement step.
fn bisect(params: &[f64], marked: &[bool]) -> Vec<f64> {
    let mut out = Vec::with_capacity(params.len() * 2);
    for (i, w) in params.windows(2).enumerate() {
        out.push(w[0]);
        if marked.get(i).copied().unwrap_or(false) {
            let mid = 0.5 * (w[0] + w[1]);
            if mid > w[0] && mid < w[1] {
                out.push(mid);
            }
        }
    }
    if let Some(last) = params.last() {
        out.push(*last);
    }
    out
}

/// Affinely maps `params` (ascending, inside `[lo, hi]`) onto the
/// clamped `0 → 1` parameterization the interpolation door requires,
/// with the ends pinned exactly.
fn normalized(params: &[f64], lo: f64, hi: f64) -> Vec<f64> {
    let span = hi - lo;
    let mut out: Vec<f64> = params.iter().map(|t| (*t - lo) / span).collect();
    if let Some(first) = out.first_mut() {
        *first = 0.0;
    }
    if let Some(last) = out.last_mut() {
        *last = 1.0;
    }
    out
}

/// **A9.4**, at the base's chart parameters (module docs): sample the
/// exact offset on the `(us, vs)` grid, then two passes of curve
/// interpolation on Eq. 9.8's averaged knot vectors.
fn interpolate_offset_grid(
    base: &NurbsSurface<f64>,
    d: f64,
    us: &[f64],
    vs: &[f64],
) -> Result<NurbsSurface<f64>, OffsetFitError> {
    let (nu, nv) = (us.len(), vs.len());
    let (ulo, uhi) = base.knots_u_f64_interim().domain();
    let (vlo, vhi) = base.knots_v_f64_interim().domain();
    let ubar = normalized(us, ulo, uhi);
    let vbar = normalized(vs, vlo, vhi);
    // `Q_{k,ℓ}`, flattened per u-row as `3·nv` scalars so the u pass
    // is ONE collocation solve with every column simultaneous.
    let mut rows_u: Vec<Vec<f64>> = Vec::with_capacity(nu);
    for u in us {
        let mut row = Vec::with_capacity(nv * 3);
        for v in vs {
            let p = offset_point(base, d, *u, *v)
                .ok_or(OffsetFitError::NonFiniteSample { uv: (*u, *v) })?;
            row.extend_from_slice(&p.to_array());
        }
        rows_u.push(row);
    }
    // Pass 1 (Eq. 9.26): interpolate through the `Q_{k,ℓ}` in u,
    // yielding the `R_{i,ℓ}`.
    let (ku, r_rows) = interpolate_columns(&ubar, OFFSET_FIT_DEGREE, &rows_u)?;
    // Transpose to v-major rows: `rows_v[ℓ]` is `3·cu` scalars.
    let cu = ku.control_count();
    let mut rows_v: Vec<Vec<f64>> = Vec::with_capacity(nv);
    for l in 0..nv {
        let mut row = Vec::with_capacity(cu * 3);
        for r in &r_rows {
            row.extend_from_slice(&r[l * 3..l * 3 + 3]);
        }
        rows_v.push(row);
    }
    // Pass 2 (Eq. 9.27): interpolate through the `R_{i,ℓ}` in v,
    // yielding the `P_{i,j}`.
    let (kv, p_rows) = interpolate_columns(&vbar, OFFSET_FIT_DEGREE, &rows_v)?;
    let cv = kv.control_count();
    // `p_rows[j]` holds control row `j`'s `3·cu` scalars, u-major
    // inside; the surface net is row-major `iu·cv + iv`.
    let mut control = Vec::with_capacity(cu * cv);
    for i in 0..cu {
        for row in p_rows.iter().take(cv) {
            control.push(Point3::new(row[i * 3], row[i * 3 + 1], row[i * 3 + 2]));
        }
    }
    // The interpolation's `0 → 1` knots onto the base's own chart
    // rectangle, ends exact — so the fitted surface lives on the same
    // parameters and the certificate's pointwise claim is about the
    // same `(u, v)` on both sides.
    let ku = ku.on_domain(ulo, uhi).map_err(OffsetFitError::Structure)?;
    let kv = kv.on_domain(vlo, vhi).map_err(OffsetFitError::Structure)?;
    NurbsSurface::new(ku, kv, control, vec![1.0; cu * cv]).map_err(OffsetFitError::Structure)
}

// ---------------------------------------------------------------------
// The certificate's two limbs
// ---------------------------------------------------------------------

/// One cell's `(u, v)` rectangle, as `((u_lo, u_hi), (v_lo, v_hi))`.
///
/// Homed here, in the one module that reads it. `patch_bound` names
/// the same shape for its own cells and the two are not unified: the
/// consolidation that would give the patch-cell vocabulary a shared
/// home is #1006's, sequenced after this unit precisely so its seam
/// stays clean.
type CellBox = ((f64, f64), (f64, f64));

/// What one measurement pass proved, plus which sample intervals the
/// next refinement round must bisect.
struct Report {
    cells: u32,
    on_locus_max: f64,
    hull_sup: f64,
    /// The `(u, v)` rectangles of the cells that carry the sup — what
    /// the next refinement round attacks.
    failing: Vec<CellBox>,
}

/// Both limbs, over the merged Bézier cell schedule.
fn measure(
    base: &NurbsSurface<f64>,
    fit: &NurbsSurface<f64>,
    d: f64,
    floor: f64,
) -> Result<Report, OffsetFitError> {
    let comp = Composite::build(base, fit, d)?;
    let (nu, nv) = comp.x.cell_counts();
    if nu == 0 || nv == 0 {
        // A misaligned or refused composite has no cells to bound,
        // and "no cells" must never read as "nothing exceeded the
        // tolerance" (D4 ¶2). The unbounded report refuses.
        return Ok(Report {
            cells: 0,
            on_locus_max: f64::INFINITY,
            hull_sup: f64::INFINITY,
            failing: Vec::new(),
        });
    }
    let mut bounds = Vec::with_capacity(nu * nv);
    let mut on_locus_max = 0.0f64;
    let mut hull_sup = 0.0f64;
    for su in 0..nu {
        for sv in 0..nv {
            let (ub, vb) = comp.cell_box(su, sv);
            let cell = comp.cell_bound(su, sv, floor, d);
            hull_sup = max_bound(hull_sup, cell);
            on_locus_max = max_bound(on_locus_max, on_locus_cell(base, fit, d, ub, vb));
            bounds.push((ub, vb, cell));
        }
    }
    // Refinement attacks the cells that carry the sup: every cell
    // within a factor of two of it. A fixed factor, on structure —
    // deterministic (D9), and it cannot mark nothing (the sup's own
    // cell always qualifies).
    //
    // The cut is RELATIVE to the round's sup. A uniform tightening of
    // every cell bound marks the same cells; one that tightens cells by
    // different factors can mark different ones, and a different
    // schedule interpolates a different fit. So the door's bound is not
    // monotone in the cell bound: a strictly tighter cell bound can end
    // on a higher door bound. Monotonicity holds per cell on a fixed
    // grid, and not at the door. No marking rule can restore it there:
    // the schedule chooses the parameters the fit interpolates at, so
    // a different schedule is a different fitted surface, whose bound
    // is not comparable with this one's in either direction. The door
    // bound would be monotone only with the fit held fixed, and moving
    // the fit is what refinement is for.
    let cut = hull_sup * 0.5;
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    let failing: Vec<CellBox> = bounds
        .iter()
        .filter(|(_, _, b)| !(*b < cut))
        .map(|(u, v, _)| (*u, *v))
        .collect();
    #[allow(clippy::cast_possible_truncation)]
    Ok(Report {
        cells: (nu * nv) as u32,
        on_locus_max,
        hull_sup,
        failing,
    })
}

/// How a round that did not certify marks the next schedule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refine {
    /// Bisect each failing cell in the one direction its model-space
    /// extent names.
    Directional,
    /// Bisect every failing cell in BOTH directions: the round was
    /// idle (it did not lower the lowest bound), so the direction
    /// rule's prediction is not to be trusted on this patch.
    BothDirections,
}

/// One refinement round's next sample schedule.
struct Marking {
    us: Vec<f64>,
    vs: Vec<f64>,
}

impl Marking {
    /// Whether this schedule is strictly larger than the one it came
    /// from. A marking that marked intervals can still fail to grow —
    /// [`bisect`] drops a midpoint that is not strictly inside its
    /// interval — and "grew" is the property the loop actually needs.
    fn grew(&self, us: &[f64], vs: &[f64]) -> bool {
        self.us.len() != us.len() || self.vs.len() != vs.len()
    }
}

/// Marks and bisects one refinement round under `mode`.
fn refine_schedule(
    us: &[f64],
    vs: &[f64],
    report: &Report,
    speed_u: f64,
    speed_v: f64,
    mode: Refine,
) -> Marking {
    let (mu, mv) = if mode == Refine::Directional {
        directional_mark(us, vs, &report.failing, speed_u, speed_v)
    } else {
        (
            mark(us, report.failing.iter().map(|c| c.0)),
            mark(vs, report.failing.iter().map(|c| c.1)),
        )
    };
    Marking {
        us: bisect(us, &mu),
        vs: bisect(vs, &mv),
    }
}

/// Marks each failing cell in ONE direction: the one whose
/// model-space extent `h_d · sup‖S_d‖` is larger.
///
/// The rule is the tessellation split selection's. `speed_u` and
/// `speed_v` are the whole-patch chart speeds
/// ([`crate::offset_meters::PatchRegularity`]), so the comparison is
/// between cell extents measured in metres rather than in chart
/// parameters — which is what makes it invariant to how the two
/// directions happen to be parameterized. Ties go to `u`, on
/// structure (D9: deterministic, never data-dependent tuning).
///
/// The speeds arrive as bare `f64`: this is a **structure selection**,
/// not a classification, so no margin crosses the decide seam here and
/// the rate pair's tag comes off at the caller rather than riding into
/// a comparison the band never sees.
fn directional_mark(
    us: &[f64],
    vs: &[f64],
    failing: &[CellBox],
    speed_u: f64,
    speed_v: f64,
) -> (Vec<bool>, Vec<bool>) {
    let mut fu: Vec<(f64, f64)> = Vec::new();
    let mut fv: Vec<(f64, f64)> = Vec::new();
    for (ub, vb) in failing {
        if (ub.1 - ub.0) * speed_u >= (vb.1 - vb.0) * speed_v {
            fu.push(*ub);
        } else {
            fv.push(*vb);
        }
    }
    (mark(us, fu.into_iter()), mark(vs, fv.into_iter()))
}

/// Marks every interval of `params` that overlaps one of the ranges.
fn mark(params: &[f64], ranges: impl Iterator<Item = (f64, f64)>) -> Vec<bool> {
    let mut out = vec![false; params.len().saturating_sub(1)];
    for (lo, hi) in ranges {
        for (i, w) in params.windows(2).enumerate() {
            if w[0] < hi && w[1] > lo {
                out[i] = true;
            }
        }
    }
    out
}

/// Limb 1 inside one cell: the fixed [`OFFSET_CERT_SAMPLES`]²
/// schedule, exact residual in metres. A target `offset_point` cannot
/// answer gives `f64::INFINITY` and a NaN residual folds to NaN; limb
/// 1's `!(on_locus_max <= tolerance)` guard in [`certify_offset_at`]
/// refuses both.
fn on_locus_cell(
    base: &NurbsSurface<f64>,
    fit: &NurbsSurface<f64>,
    d: f64,
    ub: (f64, f64),
    vb: (f64, f64),
) -> f64 {
    let mut m = 0.0f64;
    for a in 0..OFFSET_CERT_SAMPLES {
        #[allow(clippy::cast_precision_loss)]
        let u = ub.0 + (ub.1 - ub.0) * ((a as f64 + 0.5) / OFFSET_CERT_SAMPLES as f64);
        for b in 0..OFFSET_CERT_SAMPLES {
            #[allow(clippy::cast_precision_loss)]
            let v = vb.0 + (vb.1 - vb.0) * ((b as f64 + 0.5) / OFFSET_CERT_SAMPLES as f64);
            let Some(target) = offset_point(base, d, u, v) else {
                return f64::INFINITY;
            };
            m = max_bound(m, (fit.eval(u, v) - target).norm());
        }
    }
    m
}

// ---------------------------------------------------------------------
// Limb 2 — the rationalized composite
// ---------------------------------------------------------------------

/// The polynomial parts of the residual, in per-cell Bernstein form
/// on one merged break structure (module docs).
struct Composite {
    /// `X = Ẽ·Ẽ − d²·w̃²`.
    x: PatchSpans,
    /// `Y = Ẽ × M̃`.
    y: [PatchSpans; 3],
    /// `D = Ẽ · M̃`, the sign witness of `E·n`.
    dd: PatchSpans,
    /// The BASE's weight channel `w` (the constant `1` patch when the
    /// base's weights are one constant, which the spatial form reads as
    /// `1`). `M̃ = w³·m` is scaled by this
    /// one alone, because `m` is the base's own.
    w: PatchSpans,
    /// `w̃ = w·w_fit`, the weight `Ẽ` is homogeneous in. Equal to `w`
    /// exactly when the fit's weights are one constant.
    wt: PatchSpans,
    /// `Ẽ = w̃·E` itself, kept so the residual's normal component can
    /// divide by a DIRECT lower bound on `‖E‖` (module docs, "the
    /// small-`|d|` denominator") instead of by `2|d|`.
    e: [PatchSpans; 3],
    /// `M̃ = w³·m`, the base's own homogeneous normal, channel by
    /// channel — NOT `m = S_u × S_v` itself, which is why the field
    /// wears the tilde. `Y` and `D` are formed from it and it is kept
    /// past them, because the lower bound on `‖E‖` reads `D` against
    /// a certified upper bound on `‖M̃‖` (module docs, "the
    /// small-`|d|` denominator").
    m_tilde: [PatchSpans; 3],
    breaks_u: Vec<f64>,
    breaks_v: Vec<f64>,
}

/// An enclosure net of one spatial channel of a control net, paired
/// with the surface's two knot vectors and optionally weighted (the
/// homogeneous `A^c = w·P^c`).
///
/// `patch_bound::comp_nets` is the same extraction, and the two
/// share one storage shape: `geom_core::spline::TensorCoeffs`, which
/// `PatchSpans::decompose` consumes and whose net `window_hull`
/// reads. What is shared is also the arithmetic — `weight ·
/// coordinate`, in that order — and a change to it is a change to
/// both. Where they still differ is WHEN the recentring happens: this
/// one folds the centre into the net, because the net feeds
/// polynomial products formed once for the whole patch; `patch_bound`
/// applies it at the hull read, because it reads a cell-local centre
/// off the cell's own control window.
///
/// **Recentred**, on the shared `centre` all of the composite's nets
/// are built against: the entry is `w·(P − centre)`, so the net
/// describes `S − centre` rather than `S`. The subtraction is the
/// RING's, not `f64`'s, which is what makes this sound — an `f64`
/// difference would round to a control point the base does not have,
/// and the certificate would then be about a surface nobody supplied.
/// Interval arithmetic's outward rounding of `P − centre` is one ulp of the
/// DIFFERENCE, i.e. of the patch's extent, where the unrecentred net
/// carried one ulp of the coordinate.
fn channel<'n>(
    n: &'n NurbsSurface<f64>,
    c: usize,
    form: NetForm,
    origin: &Origin,
) -> TensorCoeffs<'n> {
    let nv = n.knots_v_f64_interim().control_count();
    TensorCoeffs::from_fn(n.knots_u_f64_interim(), n.knots_v_f64_interim(), |i, j| {
        // Row-major layout: control[iu·nv + iv] — the net's own.
        let (p, w) = (n.control()[i * nv + j], n.weights()[i * nv + j]);
        let x = Interval::point(match c {
            0 => p.x,
            1 => p.y,
            _ => p.z,
        }) - Interval::point(origin.0[c]);
        match form {
            NetForm::Homogeneous => Interval::point(w) * x,
            NetForm::Spatial => x,
        }
    })
}

/// The weight net `w` of a control net, as point enclosures.
fn weight_net(n: &NurbsSurface<f64>) -> TensorCoeffs<'_> {
    let nv = n.knots_v_f64_interim().control_count();
    TensorCoeffs::from_fn(n.knots_u_f64_interim(), n.knots_v_f64_interim(), |i, j| {
        Interval::point(n.weights()[i * nv + j])
    })
}

/// Which net a [`channel`] extraction produces.
///
/// A named form rather than a `bool`: the two differ by whether the
/// weight multiplies, which is the difference between `A` and `P`, and
/// a bare `true` at a call site does not say which was meant.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NetForm {
    /// `P − c`, every weight read as `1`. Correct only where the
    /// weights are one constant (`NurbsSurface::constant_weight`).
    Spatial,
    /// `w·(P − c)`, the homogeneous net.
    Homogeneous,
}

impl NetForm {
    /// The form a surface's own weights call for.
    fn of(n: &NurbsSurface<f64>) -> Self {
        if n.constant_weight().is_none() {
            Self::Homogeneous
        } else {
            Self::Spatial
        }
    }
}

/// The lower bound on `‖E‖` [`Composite::cell_bound`] runs on: the
/// larger of [`Composite::e_floors`]' two, each a sound lower bound on
/// the same norm.
fn e_low_witness(mig: Interval, proj: Interval) -> f64 {
    max_bound(mig.lo(), proj.lo())
}

/// One cell's bound, term by term (module docs): `dist + tau + t3`
/// with `t3 = tau²/‖E‖`, and the lower bound `e_lo` on `‖E‖` it ran on.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "production reads `bound`; the terms are the measurement row's"
    )
)]
struct CellTerms {
    dist: f64,
    tau: f64,
    t3: f64,
    e_lo: f64,
    bound: f64,
}

/// The composite's recentring origin, carried whole.
///
/// It travels as one value and [`channel`] selects the coordinate with
/// the SAME index it reads the control point by, so a centre cannot be
/// paired with the wrong channel. Passed as three loose `f64`s that
/// pairing is a caller obligation, and getting it wrong certifies a
/// different surface silently: the net still looks like a net, just of
/// a sheared patch.
#[derive(Clone, Copy)]
struct Origin([f64; 3]);

/// The composite's recentring origin: the midpoint of the BASE's
/// control-net bounding box, per coordinate.
///
/// Every net in the composite is built against this one point, which
/// is what keeps the recentring exact in ℝ: `Ẽ = F̃·w − A·w_fit` and
/// `M̃` are both invariant under `P ↦ P − c` applied to base and fit
/// together (`Ã = A − c·w`, and knot differencing is linear, so
/// `Ã_u = A_u − c·w_u`). Only the rounding moves.
///
/// A whole-patch centre, not a per-cell one: the composite's products
/// are formed once over the merged break structure and read per cell,
/// so a per-cell centre would mean rebuilding the cost centre per
/// cell. What that would buy over this is the patch extent against
/// the cell extent, and the measurement that would justify it has not
/// been taken.
fn recentre_origin(base: &NurbsSurface<f64>) -> Origin {
    let mut lo = [f64::INFINITY; 3];
    let mut hi = [f64::NEG_INFINITY; 3];
    for p in base.control() {
        for (c, v) in p.to_array().into_iter().enumerate() {
            lo[c] = lo[c].min(v);
            hi[c] = hi[c].max(v);
        }
    }
    let mut out = [0.0; 3];
    for c in 0..3 {
        // A non-finite or empty net recentres on the origin: the
        // composite's own refusal handling is what reports it, and a
        // NaN centre would silently refuse every cell instead.
        let m = (lo[c] + hi[c]) * 0.5;
        out[c] = if m.is_finite() { m } else { 0.0 };
    }
    Origin(out)
}

/// Componentwise cross product of two triples of channels.
fn cross_spans(a: &[PatchSpans; 3], b: &[PatchSpans; 3]) -> [PatchSpans; 3] {
    [
        a[1].mul(&b[2]).sub(&a[2].mul(&b[1])),
        a[2].mul(&b[0]).sub(&a[0].mul(&b[2])),
        a[0].mul(&b[1]).sub(&a[1].mul(&b[0])),
    ]
}

/// Dot product of two triples of channels.
fn dot_spans(a: &[PatchSpans; 3], b: &[PatchSpans; 3]) -> PatchSpans {
    a[0].mul(&b[0]).add(&a[1].mul(&b[1])).add(&a[2].mul(&b[2]))
}

impl Composite {
    #[allow(clippy::too_many_lines)]
    fn build(
        base: &NurbsSurface<f64>,
        fit: &NurbsSurface<f64>,
        d: f64,
    ) -> Result<Self, OffsetFitError> {
        // The arms are the DESCRIBED nets' (`constant_weight`); the
        // arithmetic reads them at unit weight scale.
        let (base_form, fit_form) = (NetForm::of(base), NetForm::of(fit));
        let rational = base_form == NetForm::Homogeneous;
        let (base, fit) = (unit_scaled_weights(base), unit_scaled_weights(fit));
        let (base, fit) = (&*base, &*fit);
        let (ku, kv) = (base.knots_u_f64_interim(), base.knots_v_f64_interim());
        // One break list per direction, carrying every operand's
        // interior knots — the alignment substrate (patch docs).
        let mut extra_u: Vec<f64> = ku.interior_knots().map(|(t, _)| t).collect();
        extra_u.extend(fit.knots_u_f64_interim().interior_knots().map(|(t, _)| t));
        let mut extra_v: Vec<f64> = kv.interior_knots().map(|(t, _)| t).collect();
        extra_v.extend(fit.knots_v_f64_interim().interior_knots().map(|(t, _)| t));
        let dec = |grid: &TensorCoeffs<'_>| PatchSpans::decompose(grid, &extra_u, &extra_v);
        // The FIT's homogeneous net `F̃ = w_fit·P_fit` and its weight
        // channel. On a constant-weight fit the spatial net IS the
        // homogeneous one at `w_fit ≡ 1`, and `wf` is not formed: the
        // identity product would widen the enclosure for nothing.
        // The one recentring origin every net below is built against.
        let ctr = recentre_origin(base);
        let fc = |c: usize| channel(fit, c, fit_form, &ctr);
        let f: [PatchSpans; 3] = [dec(&fc(0)), dec(&fc(1)), dec(&fc(2))];
        let wf = (fit_form == NetForm::Homogeneous).then(|| dec(&weight_net(fit)));
        // The base's homogeneous nets and their first derivatives.
        let a_grid: [TensorCoeffs<'_>; 3] =
            core::array::from_fn(|c| channel(base, c, base_form, &ctr));
        // `check_direction` admits a degree-1 direction only as one
        // span, so its partial is one constant: span 0's difference,
        // decomposed on the direction's own vector as its degree-1 form.
        // Every base channel is then raised one degree in each degree-1
        // direction, in the ring: exact in ℝ, so the composite is still
        // about the base's own nets. The raise buys hull tightness,
        // measured once on `tests/offset_fit.rs`' degree-1 rows
        // (certified sup, raised against not): bilinear 4.26e-5 against
        // 4.58e-5, rational bilinear 7.58e-5 against 9.46e-5, ruled
        // 1.228e-5 against 1.229e-5, with the same cells and rounds.
        check_direction(ku)?;
        check_direction(kv)?;
        let lift = |k: &KnotVector| usize::from(matches!(k.derived(), Derived::SpanConstant(_)));
        let (lift_u, lift_v) = (lift(ku), lift(kv));
        let raise = |p: PatchSpans, lu: usize, lv: usize| {
            let (pu, pv) = p.degree();
            p.elevated(pu + lu, pv + lv)
        };
        let val = |g: &TensorCoeffs<'_>| raise(dec(g), lift_u, lift_v);
        let du = |g: &TensorCoeffs<'_>| {
            let p = match g.derived_u() {
                Derived::Vector(t) => dec(&t),
                Derived::SpanConstant(_) => {
                    let diff = g.diff_u();
                    dec(&TensorCoeffs::from_fn(ku, kv, |_, j| diff.get(0, j)))
                }
                Derived::Discontinuous(_) => unreachable!(
                    "the base's u vector passed `check_direction` above, which refuses a \
                     discontinuous first derivative as a crease: {:?}",
                    ku.knots()
                ),
            };
            raise(p, 0, lift_v)
        };
        let dv = |g: &TensorCoeffs<'_>| {
            let p = match g.derived_v() {
                Derived::Vector(t) => dec(&t),
                Derived::SpanConstant(_) => {
                    let diff = g.diff_v();
                    dec(&TensorCoeffs::from_fn(ku, kv, |i, _| diff.get(i, 0)))
                }
                Derived::Discontinuous(_) => unreachable!(
                    "the base's v vector passed `check_direction` above, which refuses a \
                     discontinuous first derivative as a crease: {:?}",
                    kv.knots()
                ),
            };
            raise(p, lift_u, 0)
        };
        let a: [PatchSpans; 3] = [val(&a_grid[0]), val(&a_grid[1]), val(&a_grid[2])];
        let a_u: [PatchSpans; 3] = [du(&a_grid[0]), du(&a_grid[1]), du(&a_grid[2])];
        let a_v: [PatchSpans; 3] = [dv(&a_grid[0]), dv(&a_grid[1]), dv(&a_grid[2])];
        let w_grid = weight_net(base);
        let w = if rational {
            val(&w_grid)
        } else {
            a[0].constant(Interval::one())
        };
        // Ẽ = F̃·w − A·w_fit = w·w_fit·(S_fit − S). The composite is
        // homogeneous in the PRODUCT of the two weights, which is
        // what `wt` carries: reading a rational fit's net as a
        // polynomial would bound a different surface than the one
        // supplied.
        let scaled_a = |c: usize| match &wf {
            Some(wf) => a[c].mul(wf),
            None => a[c].clone(),
        };
        let e: [PatchSpans; 3] = [
            f[0].mul(&w).sub(&scaled_a(0)),
            f[1].mul(&w).sub(&scaled_a(1)),
            f[2].mul(&w).sub(&scaled_a(2)),
        ];
        let wt = match &wf {
            Some(wf) => w.mul(wf),
            None => w.clone(),
        };
        // M̃ = w·(A_u × A_v) − w_v·(A_u × A) − w_u·(A × A_v). The last
        // two terms vanish identically for a non-rational base
        // (`w ≡ 1`), and are not formed there.
        let auav = cross_spans(&a_u, &a_v);
        let m_tilde: [PatchSpans; 3] = if rational {
            let w_u = du(&w_grid);
            let w_v = dv(&w_grid);
            let aua = cross_spans(&a_u, &a);
            let aav = cross_spans(&a, &a_v);
            [
                w.mul(&auav[0])
                    .sub(&w_v.mul(&aua[0]))
                    .sub(&w_u.mul(&aav[0])),
                w.mul(&auav[1])
                    .sub(&w_v.mul(&aua[1]))
                    .sub(&w_u.mul(&aav[1])),
                w.mul(&auav[2])
                    .sub(&w_v.mul(&aua[2]))
                    .sub(&w_u.mul(&aav[2])),
            ]
        } else {
            auav
        };
        let x = dot_spans(&e, &e).sub(&wt.mul(&wt).scale(Interval::point(d).sqr()));
        let y = cross_spans(&e, &m_tilde);
        let dd = dot_spans(&e, &m_tilde);
        let (bu, bv) = x.breaks();
        let (breaks_u, breaks_v) = (bu.to_vec(), bv.to_vec());
        Ok(Self {
            x,
            y,
            dd,
            w,
            wt,
            e,
            m_tilde,
            breaks_u,
            breaks_v,
        })
    }

    /// The `(u, v)` rectangle cell `(su, sv)` covers.
    fn cell_box(&self, su: usize, sv: usize) -> ((f64, f64), (f64, f64)) {
        (
            (self.breaks_u[su], self.breaks_u[su + 1]),
            (self.breaks_v[sv], self.breaks_v[sv + 1]),
        )
    }

    /// A certified UPPER bound on `‖M̃‖` over one cell, read from the
    /// norms of its vector coefficients (D4 ¶2).
    ///
    /// It is the DIVISOR of a lower bound on `‖E‖`
    /// ([`Composite::e_floors`]), so an upper bound short by one ulp
    /// makes that quotient unsound; [`PatchSpans::cell_norm_sup`]
    /// rounds every step outward.
    fn m_tilde_sup(&self, su: usize, sv: usize) -> f64 {
        PatchSpans::cell_norm_sup(
            [&self.m_tilde[0], &self.m_tilde[1], &self.m_tilde[2]],
            su,
            sv,
        )
    }

    /// A certified UPPER bound on `‖Y‖` over one cell, read from the
    /// norms of its vector coefficients (D4 ¶2) — the `τ` limb's
    /// numerator.
    fn y_sup(&self, su: usize, sv: usize) -> f64 {
        PatchSpans::cell_norm_sup([&self.y[0], &self.y[1], &self.y[2]], su, sv)
    }

    /// The two lower bounds on `‖E‖` one cell carries: the
    /// componentwise mignitude assembly on `Ẽ`'s own hulls, and the
    /// same norm read through the sign witness (module docs, "the
    /// small-`|d|` denominator"). Both are sound on their own.
    ///
    /// The first is what keeps the normal component's denominator
    /// honest when `|d|` is small: `‖E‖ + |d|` is the true divisor of
    /// `|‖E‖² − d²|`, and falling back on `2|d|` for it both loses
    /// accuracy and, once `dist` reaches `|d|`, collapses the cell to
    /// `+∞` for no geometric reason. It reads the components ONE AT A
    /// TIME, so it collapses wherever every component of `E`
    /// straddles zero — which is what a rotating normal does to
    /// `E ≈ d·n` across a cell.
    ///
    /// The second sees the three together: `|E·n| ≤ ‖E‖` for any `E`,
    /// and with `n = m/‖m‖`, `M̃ = w³·m` and `D = w̃·w³·(E·m)`,
    ///
    /// ```text
    /// ‖E‖ ≥ |E·n| = |E·m|/‖m‖ = |D| / (w̃·‖M̃‖).
    /// ```
    ///
    /// `mig(D)` is positive on exactly the cells whose `D` hull is
    /// sign definite, which is the condition [`Composite::cell_bound`]
    /// proves before it reads anything here; where the hull straddles
    /// this reads zero, which is a lower bound on any norm. Where
    /// `E ∥ n` it reads `‖E‖ ≈ |d|` and the componentwise assembly
    /// reads nothing.
    ///
    /// One home for both, because the row that measures the pair
    /// selects between these two intervals rather than re-spelling
    /// either: the reading under test is the only thing that differs
    /// between the row and production.
    fn e_floors(&self, su: usize, sv: usize) -> (Interval, Interval) {
        let wt = self.wt.cell_hull(su, sv);
        let e_mig_sq = Interval::point(mig(self.e[0].cell_hull(su, sv))).sqr()
            + Interval::point(mig(self.e[1].cell_hull(su, sv))).sqr()
            + Interval::point(mig(self.e[2].cell_hull(su, sv))).sqr();
        let e_mig_iv = e_mig_sq.sqrt() / wt;
        let m_sup = self.m_tilde_sup(su, sv);
        let e_proj_iv = if m_sup > 0.0 && m_sup.is_finite() {
            Interval::point(mig(self.dd.cell_hull(su, sv))) / (Interval::point(m_sup) * wt)
        } else {
            Interval::zero()
        };
        (e_mig_iv, e_proj_iv)
    }

    /// One cell's certified sup bound on `‖S_fit − (S + d·n)‖`
    /// (module docs). `f64::INFINITY` whenever a side condition is
    /// not proved — never a finite wrong answer.
    fn cell_bound(&self, su: usize, sv: usize, floor: f64, d: f64) -> f64 {
        self.cell_terms(su, sv, floor, d, e_low_witness)
            .map_or(f64::INFINITY, |t| t.bound)
    }

    /// [`Composite::cell_bound`]'s assembly, term by term, with the
    /// lower bound on `‖E‖` read from [`Composite::e_floors`]' pair by
    /// `e_low`. `None` whenever a side condition is not proved. The
    /// one home of the bound's guards: production and the row that
    /// measures the `e_low` reading both run these.
    ///
    /// **The whole assembly stays in certification arithmetic**, with
    /// `.hi()` read once per term at the end: every intermediate is an
    /// [`Interval`], so the outward rounding of each quotient,
    /// product and sum is interval arithmetic's. An `f64` fold of
    /// enclosure endpoints would round to nearest at each step and
    /// under-cover the real bound by ulps, which "certified" does not
    /// permit.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn cell_terms(
        &self,
        su: usize,
        sv: usize,
        floor: f64,
        d: f64,
        e_low: impl FnOnce(Interval, Interval) -> f64,
    ) -> Option<CellTerms> {
        // **Every refusal below is asked by name before its
        // endpoint is read.** A refusal is its decoration, so
        // a refused hull carries ordinary endpoints: `w_lo > 0.0` and
        // `w_lo.is_finite()` are both TRUE of one, and so is the
        // `hi.is_finite()` at the end. `f64::INFINITY` is the whole
        // function's answer for anything not proved.
        let w = self.w.cell_hull(su, sv);
        let w_lo = w.lo();
        if !w.is_certified() || !(w_lo > 0.0) || !w_lo.is_finite() {
            return None;
        }
        // `w̃ = w·w_fit`, the weight `Ẽ`, `X` and the sign witness are
        // homogeneous in. Its positivity is the rational licence's on
        // both factors, and it is proved here rather than assumed.
        let wt = self.wt.cell_hull(su, sv);
        let wt_lo = wt.lo();
        if !wt.is_certified() || !(wt_lo > 0.0) || !wt_lo.is_finite() {
            return None;
        }
        // The sign witness: `sign(E·n) = sign(D)` (the denominator
        // `w·‖M̃‖` is positive), and the normal-component bound below
        // needs `E·n` to carry `d`'s sign.
        let dh = self.dd.cell_hull(su, sv);
        if !dh.is_certified()
            || !(if d > 0.0 {
                dh.lo() > 0.0
            } else {
                dh.hi() < 0.0
            })
        {
            return None;
        }
        let abs_d = Interval::point(d.abs());
        let (e_mig_iv, e_proj_iv) = self.e_floors(su, sv);
        if !e_mig_iv.is_certified() || !e_proj_iv.is_certified() {
            return None;
        }
        let e_hull_lo = e_low(e_mig_iv, e_proj_iv);
        // | ‖E‖ − |d| | = |X| / (w̃²·(‖E‖ + |d|)).
        let x_mag = Interval::from_bounds(0.0, self.x.cell_hull(su, sv).mag());
        let dist_iv = x_mag / (wt.sqr() * (Interval::point(e_hull_lo) + abs_d));
        // τ = ‖Y‖ / (w̃·‖M̃‖) ≤ sup‖Y‖ / (floor·w̃·w³), using
        // ‖M̃‖ = w³·‖m‖ ≥ w³·floor.
        let y_mag = Interval::from_bounds(0.0, self.y_sup(su, sv));
        let tau_iv = y_mag / (Interval::point(floor) * wt * w.powi(3));
        // `‖E‖` from below once more, for the `τ²/‖E‖` term: the
        // better of the two hull readings above, or `|d| − dist` when
        // that is larger. The three are lower bounds on the same
        // norm, so their max is one too — the same `max`, spelled the
        // same way.
        if !dist_iv.is_certified() {
            return None;
        }
        let e_floor = max_bound(e_hull_lo, d.abs() - dist_iv.hi());
        if !(e_floor > 0.0) {
            return None;
        }
        let t3 = tau_iv.sqr() / Interval::point(e_floor);
        let bound = dist_iv + tau_iv + t3;
        if !bound.is_certified() {
            return None;
        }
        let hi = bound.hi();
        hi.is_finite().then(|| CellTerms {
            dist: dist_iv.hi(),
            tau: tau_iv.hi(),
            t3: t3.hi(),
            e_lo: e_hull_lo,
            bound: hi,
        })
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Composite, directional_mark, e_low_witness};
    use geom_core::Bounds;
    use geom_core::interval::{max_bound, min_bound, norm_sup};
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point3, Tol};

    /// One cell's certificate, split into the terms the module doc
    /// names, with the lower bound on `‖E‖` selectable: `Witness`
    /// takes the one the shipped [`Composite::cell_bound`] takes,
    /// `Componentwise` takes the mignitude assembly alone.
    ///
    /// Both run [`Composite::cell_terms`], the assembly and guards
    /// production runs, so the mode switches ONE expression — which
    /// is what makes the pair a measurement of that expression rather
    /// than of a second copy of the bound.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum ELow {
        Componentwise,
        Witness,
    }

    /// `(dist, tau, tau²/‖E‖, e_lo, bound)` at one cell; infinite
    /// terms and a zero `e_lo` where a side condition is not proved.
    fn decompose(
        comp: &Composite,
        su: usize,
        sv: usize,
        floor: f64,
        d: f64,
        mode: ELow,
    ) -> (f64, f64, f64, f64, f64) {
        let terms = match mode {
            ELow::Componentwise => comp.cell_terms(su, sv, floor, d, |mig, _| mig.lo()),
            ELow::Witness => comp.cell_terms(su, sv, floor, d, e_low_witness),
        };
        terms.map_or(
            (
                f64::INFINITY,
                f64::INFINITY,
                f64::INFINITY,
                0.0,
                f64::INFINITY,
            ),
            |t| (t.dist, t.tau, t.t3, t.e_lo, t.bound),
        )
    }

    /// The sup over a composite's cells under one `‖E‖` mode, with
    /// the cell that carries it; a NaN cell bound is taken, not passed.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn sup_cell(comp: &Composite, floor: f64, d: f64, mode: ELow) -> (usize, usize, f64) {
        let (nu, nv) = comp.x.cell_counts();
        let mut sup = (0usize, 0usize, 0.0f64);
        for su in 0..nu {
            for sv in 0..nv {
                let b = decompose(comp, su, sv, floor, d, mode).4;
                if !(b <= sup.2) {
                    sup = (su, sv, b);
                }
            }
        }
        sup
    }

    /// The cells whose bound under `mode` agrees with `sup` to nine
    /// significant digits — the tie a sup cell is picked out of.
    fn tied_with(
        comp: &Composite,
        floor: f64,
        d: f64,
        mode: ELow,
        sup: f64,
    ) -> Vec<(usize, usize)> {
        let (nu, nv) = comp.x.cell_counts();
        (0..nu)
            .flat_map(|su| (0..nv).map(move |sv| (su, sv)))
            .filter(|&(su, sv)| {
                (decompose(comp, su, sv, floor, d, mode).4 - sup).abs() <= sup * 1e-9
            })
            .collect()
    }

    /// Every cell of a composite, under both readings: the witness
    /// reading is a MAX over the componentwise one, so no cell's
    /// bound may rise. Returns the worst (largest) ratio old/new.
    fn no_cell_loosens(comp: &Composite, floor: f64, d: f64) -> f64 {
        let (nu, nv) = comp.x.cell_counts();
        let mut worst = 0.0f64;
        for su in 0..nu {
            for sv in 0..nv {
                let old = decompose(comp, su, sv, floor, d, ELow::Componentwise).4;
                let new = decompose(comp, su, sv, floor, d, ELow::Witness).4;
                assert!(
                    new <= old,
                    "cell ({su},{sv}) loosened: {new:e} against {old:e}"
                );
                if old.is_finite() && new > 0.0 {
                    worst = worst.max(old / new);
                }
            }
        }
        worst
    }

    /// The rational quarter cylinder the micron row fits.
    fn quarter_cylinder() -> geom::NurbsSurface<f64> {
        let s = (core::f64::consts::FRAC_PI_2 * 0.5).cos();
        let kv2 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
        ];
        geom::NurbsSurface::new(kv2, kv1, control, vec![1.0, 1.0, s, s, 1.0, 1.0]).unwrap()
    }

    /// Four significant digits, which is what these rows pin.
    fn near(x: f64, want: f64) -> bool {
        (x - want).abs() <= want.abs() * 5e-4
    }

    /// **The floor on `‖E‖` reads the components together.**
    ///
    /// At `d = 1e-6` on the quarter cylinder `E ≈ d·n` and the normal
    /// rotates across every cell, so each component of `Ẽ` straddles
    /// zero and the componentwise mignitude assembly reads `1.58e-8`
    /// where `‖E‖ ≈ 1e-6`. The projection through the sign witness,
    /// `|D|/(w̃·‖M̃‖)`, reads the same cell at `5.64e-7` — within a
    /// factor of two of `|d|` — and the `τ²/‖E‖` term that carried
    /// 95% of the cell's bound falls with it.
    ///
    /// The row measures the two readings on ONE composite, so the
    /// only thing that differs between them is the expression under
    /// test.
    ///
    /// **The sup cell is one of a tie.** Cells across the grid agree
    /// to nine significant digits (on the `1e-3` grid, `(0, 3)` and
    /// `(21, 6)` among them), and which one carries the sup is decided
    /// below them by rounding. So the row pins the tie and the witness's
    /// magnitudes, which every tied cell shares, and not the winner's
    /// indices.
    #[test]
    fn the_sign_witness_floors_norm_e_where_the_components_straddle_zero() {
        let base = quarter_cylinder();
        let d = 1e-6;
        let band = Band::linear(Tol::witness()).unwrap();
        let (reg, _) = crate::offset_meters::meter_patch(&base, d, band).unwrap();

        // The seed grid carries no bound at all under either reading:
        // its fit is too coarse for the sign witness. The floor is
        // therefore measured from the first grid that carries one.
        let (us, vs) = super::seed_params(&base);
        let seed = super::interpolate_offset_grid(&base, d, &us, &vs).unwrap();
        let comp0 = Composite::build(&base, &seed, d).unwrap();
        for mode in [ELow::Componentwise, ELow::Witness] {
            assert!(
                sup_cell(&comp0, reg.floor, d, mode).2.is_infinite(),
                "the seed grid's sup is finite — the sign witness now passes there"
            );
        }

        // The first grid that carries a finite bound, which is also
        // the grid a `1e-3` request certifies at.
        let (fit, cert) = super::fit_offset_at(&base, d, 1e-3, band).unwrap();
        assert_eq!((cert.rounds, cert.cells), (4, 308));
        let comp = Composite::build(&base, &fit, d).unwrap();
        let (su, sv, sup) = sup_cell(&comp, reg.floor, d, ELow::Witness);
        let tie = tied_with(&comp, reg.floor, d, ELow::Witness, sup);
        assert!(
            tie.len() > 1 && tie.contains(&(su, sv)),
            "the sup cell {:?} is one of a tie: {tie:?}",
            (su, sv)
        );
        assert!(near(sup, 1.5141e-5), "sup cell bound is {sup:e}");
        assert!(
            near(cert.hull_sup, sup),
            "the certificate carries another cell's bound"
        );
        let (dist, tau, t3, e_lo, _) = decompose(&comp, su, sv, reg.floor, d, ELow::Witness);
        assert!(
            e_lo > d * 0.5 && e_lo < d * 2.0,
            "the floor on ‖E‖ reads {e_lo:e} where ‖E‖ ≈ {d:e}"
        );
        assert!(near(e_lo, 5.6395e-7), "floor is {e_lo:e}");
        assert!(near(dist, 6.0900e-6) && near(tau, 1.9948e-6) && near(t3, 7.0563e-6));

        // The same cell read componentwise: the collapsed floor, and
        // the `τ²/‖E‖` term it inflates. `τ` is untouched — it divides
        // by the regularity floor, not by `‖E‖`.
        let (dist_c, tau_c, t3_c, e_mig, sup_c) =
            decompose(&comp, su, sv, reg.floor, d, ELow::Componentwise);
        assert!(near(e_mig, 1.5814e-8), "componentwise floor is {e_mig:e}");
        assert!(near(sup_c, 2.6302e-4) && near(dist_c, 9.3762e-6) && near(t3_c, 2.5165e-4));
        assert!(near(tau_c, tau), "τ moved: {tau_c:e} against {tau:e}");
        assert!(
            t3_c > 0.95 * sup_c,
            "the componentwise reading's sup is not its τ²/‖E‖ term"
        );
        assert!(
            near(sup_c / sup, 17.371),
            "the bound moved by {}",
            sup_c / sup
        );

        // No cell anywhere on this grid loosens: the reading is a max
        // over the one it replaces, and the row measures that rather
        // than resting on the argument.
        let worst = no_cell_loosens(&comp, reg.floor, d);
        assert!(near(worst, 17.371), "the widest cell gain is {worst}");

        // One round finer — the grid the `1e-9` request stops on at
        // the sample cap. The cells are small enough that the
        // componentwise assembly no longer collapses (`7.80e-7`), so
        // the witness reading adds 1%, and the sup is carried by `τ`,
        // which this bound does not reach.
        let (fit5, cert5) = super::fit_offset_at(&base, d, sup.next_down(), band).unwrap();
        assert_eq!((cert5.rounds, cert5.cells), (5, 364));
        let comp5 = Composite::build(&base, &fit5, d).unwrap();
        let (su5, sv5, sup5) = sup_cell(&comp5, reg.floor, d, ELow::Witness);
        let (dist5, tau5, t35, e_lo5, _) = decompose(&comp5, su5, sv5, reg.floor, d, ELow::Witness);
        // The SAME quantity `tests/offset_fit.rs`'s
        // `a_micron_scale_offset_certifies_and_names_its_limit` pins as
        // the cap stop's `achieved`, and it is pinned there to 5e-11
        // absolute. Re-measured here rather than left to ride `near`'s
        // 5e-4: one number must not have two precisions. It moved
        // under the convex insertion form (3.7544e-7 before) and again
        // when the regularity floor became the area quotient's inf
        // (3.74718e-7 before): `τ` divides by that floor.
        assert!(near(sup5, 3.45243e-7), "cap-grid sup is {sup5:e}");
        assert!(near(e_lo5, 8.39898e-7) && near(dist5, 1.21528e-7) && near(t35, 3.88872e-8));
        assert!(
            tau5 > dist5 && tau5 > t35,
            "τ = {tau5:e} no longer carries the cap grid's sup"
        );
        let (.., e_mig5, sup5_c) = decompose(&comp5, su5, sv5, reg.floor, d, ELow::Componentwise);
        assert!(
            near(e_mig5, 7.79946e-7),
            "componentwise floor is {e_mig5:e}"
        );
        assert!(near(sup5_c, 3.49518e-7) && near(sup5_c / sup5, 1.01238));
        let worst5 = no_cell_loosens(&comp5, reg.floor, d);
        assert!(worst5 >= 1.0, "the cap grid's widest cell gain is {worst5}");
    }

    /// **A limb refusing at the mint is the mint's.** The certifier
    /// `mint` runs on the loop's accepted fit is planted to refuse at
    /// limb 1, which the shipped certifier does only on a sample above
    /// the bound the loop accepted; the refusal leaves as `MintLimb`
    /// with its payload, and every other refusal passes through as it
    /// came.
    #[test]
    fn a_limb_refusing_the_fresh_fit_is_the_mints() {
        use super::{OffsetFitError, OffsetLimb, fit_offset_at, mint};
        let base = std::sync::Arc::new(bowed_patch());
        let band = Band::linear(Tol::witness()).unwrap();
        let fitted = fit_offset_at(&base, 0.05, 1e-3, band).unwrap();
        let limb = OffsetFitError::Limb {
            limb: OffsetLimb::OnLocus,
            bound: 2e-3,
            tolerance: 1e-3,
        };
        let refused = mint(
            std::sync::Arc::clone(&base),
            0.05,
            fitted.clone(),
            |_, _, _| Err(limb.clone()),
        )
        .unwrap_err();
        assert_eq!(
            refused,
            OffsetFitError::MintLimb {
                limb: OffsetLimb::OnLocus,
                bound: 2e-3,
                tolerance: 1e-3,
            },
            "the mint's limb left as the at-rest one"
        );
        let other = OffsetFitError::WindowUnsupported {
            window: geom::ApproxWindow {
                u: (0.25, 0.75),
                v: (0.25, 0.75),
            },
        };
        let passed = mint(base, 0.05, fitted, |_, _, _| Err(other.clone())).unwrap_err();
        assert_eq!(passed, other, "a non-limb refusal was rerouted");
    }

    /// **The loop's NaN residual is the mint's.** The guard the fit loop
    /// runs on every round's sampled residual refuses a NaN as
    /// `MintLimb` at limb 1 carrying the poison, and passes every
    /// number, an infinite one included (the hull bound decides those).
    #[test]
    fn a_nan_residual_in_the_fit_loop_is_the_mints() {
        use super::{OffsetFitError, OffsetLimb, nan_residual_at};
        match nan_residual_at(f64::NAN, 1e-3) {
            Err(OffsetFitError::MintLimb {
                limb: OffsetLimb::OnLocus,
                bound,
                tolerance,
            }) => {
                assert!(bound.is_nan(), "the poison was not carried: {bound}");
                assert_eq!(tolerance, 1e-3, "the tolerance was not carried");
            }
            other => panic!("a NaN residual did not refuse as the mint's: {other:?}"),
        }
        for r in [0.0, 2e-3, f64::INFINITY] {
            assert_eq!(nan_residual_at(r, 1e-3), Ok(()), "{r} refused");
        }
    }

    /// The bowed integral patch `topo`'s `Approx` fixture offsets.
    fn bowed_patch() -> geom::NurbsSurface<f64> {
        const BOW: f64 = 1.5e-2;
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let mut control = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                let (u, v) = (f64::from(i) * 0.5, f64::from(j) * 0.5);
                control.push(Point3::new(
                    u,
                    v,
                    BOW * u * (1.0 - u) + (BOW * 2.0 / 3.0) * v * v,
                ));
            }
        }
        geom::NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 9]).unwrap()
    }

    /// The rotation by `t` radians about `axis` (Rodrigues).
    fn rotation(axis: [f64; 3], t: f64) -> impl Fn(Point3<f64>) -> Point3<f64> {
        let n = axis.iter().map(|a| a * a).sum::<f64>().sqrt();
        let [x, y, z] = axis.map(|a| a / n);
        let (c, s, k) = (t.cos(), t.sin(), 1.0 - t.cos());
        let m = [
            [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
            [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
            [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
        ];
        move |p: Point3<f64>| {
            Point3::new(
                m[0][0] * p.x + m[0][1] * p.y + m[0][2] * p.z,
                m[1][0] * p.x + m[1][1] * p.y + m[1][2] * p.z,
                m[2][0] * p.x + m[2][1] * p.y + m[2][2] * p.z,
            )
        }
    }

    /// **The certificate's and the meters' vector upper bounds are
    /// frame-invariant to rounding (D4 ¶2).** A bowed patch, an offset
    /// "fit" of it, and a rational twin of the patch are rotated
    /// rigidly, and every cell's `sup‖Y‖` and `sup‖M̃‖`, every cell's
    /// [`Composite::cell_bound`] (at the unrotated frame's regularity
    /// floor, so the bound reads `Y` and `M̃` the way the certificate
    /// does), and both patches' chart speeds and per-cell
    /// `sup‖S_u × S_v‖` (the meters' cells) are compared with the
    /// unrotated frame's, each against its own largest value over the
    /// patch.
    /// Read from the coefficient vectors' norms they move by rounding
    /// alone; a box of three channel hulls folded into a norm reads up
    /// to `√3`× the vector's reach depending on how it sits against the
    /// axes, and moves `‖Y‖` by `8.7e-2` here.
    ///
    /// The fit is deliberately crude — the base lifted by `d` along
    /// `z` — so that `Y` is a real vector, the normal's tilt times `d`,
    /// rather than the rounding dust a good fit leaves in it.
    #[test]
    fn the_vector_upper_bounds_do_not_move_under_a_rotation() {
        // `Y`'s coefficients cancel products of size `|d|·‖M̃‖ ≈ 0.05` down
        // to `≈ 8e-4`, so each carries ~1e-17 of rounding per term, ~1e-14
        // of its scale, compounded over the products' terms to the measured
        // 5.6e-13 (7.5e-13 through `cell_bound`). The other columns are
        // norms of uncancelled vectors, a few products deep, and measure
        // 2e-15 to 4e-15: `sup‖M̃‖` 3.6e-15, both patches' speeds 2.0e-15
        // and 1.1e-15, the meters' `sup‖S_u × S_v‖` 2.2e-15 (integral) and
        // 4.0e-15 (rational). 1e-10 is ~130× the worst and nine decades
        // under a box fold.
        const DRIFT: f64 = 1e-10;
        let splits = [0.25, 0.5, 0.75];
        let base = bowed_patch()
            .refine_knots_u(&splits)
            .unwrap()
            .refine_knots_v(&splits)
            .unwrap();
        let d = 0.05;
        let fit = base.map_points(|p| Point3::new(p.x, p.y, p.z + d));
        // Weights that vary along both directions, so every quotient
        // term of the rational forms is live.
        let weights: Vec<f64> = (0..base.weights().len())
            .map(|k| 1.0 + 0.15 * f64::from(u32::try_from(k % 5).unwrap()) / 4.0)
            .collect();
        let rational = geom::NurbsSurface::new(
            base.knots_u_f64_interim().clone(),
            base.knots_v_f64_interim().clone(),
            base.control().to_vec(),
            weights,
        )
        .unwrap();
        // The regularity floor `τ` divides by is box-assembled, so it is
        // held at the unrotated frame's value: the `cell_bound` column
        // then moves only with the readings it takes from `Y` and `M̃`.
        let floor = crate::offset_meters::patch_regularity(
            &crate::patch_bound::patch_cells_refined(&base, 1).unwrap(),
        )
        .floor;
        let meters = |n: &geom::NurbsSurface<f64>| {
            let cells = crate::patch_bound::patch_cells_refined(n, 1).unwrap();
            let reg = crate::offset_meters::patch_regularity(&cells);
            let normal = cells.iter().map(|c| c.m_sup).collect::<Vec<_>>();
            (vec![reg.speed_u.get(), reg.speed_v.get()], normal)
        };
        let reading = |base: &geom::NurbsSurface<f64>,
                       fit: &geom::NurbsSurface<f64>,
                       rational: &geom::NurbsSurface<f64>| {
            let comp = Composite::build(base, fit, d).unwrap();
            let (nu, nv) = comp.x.cell_counts();
            let cells: Vec<(usize, usize)> = (0..nu)
                .flat_map(|su| (0..nv).map(move |sv| (su, sv)))
                .collect();
            let (speeds, normal) = meters(base);
            let (r_speeds, r_normal) = meters(rational);
            [
                cells
                    .iter()
                    .map(|&(su, sv)| comp.y_sup(su, sv))
                    .collect::<Vec<_>>(),
                cells
                    .iter()
                    .map(|&(su, sv)| comp.m_tilde_sup(su, sv))
                    .collect(),
                speeds,
                cells
                    .iter()
                    .map(|&(su, sv)| comp.cell_bound(su, sv, floor, d))
                    .collect(),
                normal,
                r_speeds,
                r_normal,
            ]
        };
        let names = [
            "sup‖Y‖",
            "sup‖M̃‖",
            "the chart speeds",
            "cell_bound",
            "the meters' sup‖S_u × S_v‖",
            "the rational chart speeds",
            "the rational meters' sup‖S_u × S_v‖",
        ];
        let at_rest = reading(&base, &fit, &rational);
        assert_eq!(
            at_rest[0].len(),
            16,
            "the refined bowed patch has 4×4 cells"
        );
        let mut worst = [0.0f64; 7];
        for axis in [[1.0, 1.0, 1.0], [0.3, -0.4, 0.8]] {
            for k in 1..=8 {
                let t = 0.405 * f64::from(k);
                let r = rotation(axis, t);
                let moved = reading(
                    &base.map_points(&r),
                    &fit.map_points(&r),
                    &rational.map_points(&r),
                );
                for (q, (now, then)) in moved.iter().zip(&at_rest).enumerate() {
                    assert_eq!(now.len(), then.len());
                    let scale = then.iter().copied().fold(0.0f64, f64::max);
                    for (a, b) in now.iter().zip(then) {
                        let drift = (a - b).abs() / scale;
                        assert!(
                            drift.is_finite() && scale > 0.0,
                            "{} at {axis:?}, {t}: {a:e} against {b:e}",
                            names[q]
                        );
                        worst[q] = worst[q].max(drift);
                    }
                }
            }
        }
        eprintln!(
            "worst drift: {:?}",
            names.iter().zip(worst).collect::<Vec<_>>()
        );
        for (q, w) in worst.iter().enumerate() {
            assert!(
                *w <= DRIFT,
                "{} moved by {w:e} of its scale under a rotation",
                names[q]
            );
        }
    }

    /// The weight function of a NURBS patch at `(u, v)`, read as the
    /// `x` coordinate of the integral patch on the same knots whose
    /// control points are `(w_ij, 0, 0)`.
    fn weight_at(n: &geom::NurbsSurface<f64>, u: f64, v: f64) -> f64 {
        let control = n
            .weights()
            .iter()
            .map(|w| Point3::new(*w, 0.0, 0.0))
            .collect();
        let ones = vec![1.0; n.weights().len()];
        geom::NurbsSurface::new(
            n.knots_u_f64_interim().clone(),
            n.knots_v_f64_interim().clone(),
            control,
            ones,
        )
        .unwrap()
        .eval(u, v)
        .x
    }

    /// **`sup‖M̃‖` and `sup‖Y‖` are certified from above, and never
    /// looser than the box.**
    ///
    /// [`Composite::m_tilde_sup`] divides a LOWER bound on `‖E‖`
    /// ([`Composite::e_floors`]) and [`Composite::y_sup`] is the `τ`
    /// limb's numerator, so either reading short of the true sup makes
    /// the certificate unsound. The row samples `M̃ = w³·(S_u × S_v)`
    /// and `Y = w·w_fit·(S_fit − S) × M̃` on a grid inside every cell
    /// and reds if either reading is below a sample (one part in 1e9
    /// of slack for the sample's own `f64` evaluation). It also reds
    /// if either reading exceeds the norm of the three channels' cell
    /// hulls: every coefficient lies in that box, so the coefficient
    /// norm can only be at or below it.
    #[test]
    fn the_normal_and_y_sups_bound_every_sample_and_never_exceed_the_box() {
        const N: usize = 5;
        let band = Band::linear(Tol::witness()).unwrap();
        let (name, base, d, tol) = ("qc", quarter_cylinder(), 1e-6, 1e-3);
        let (fit, _) = super::fit_offset_at(&base, d, tol, band).unwrap();
        let comp = Composite::build(&base, &fit, d).unwrap();
        let (nu, nv) = comp.x.cell_counts();
        let box_sup = |ch: &[geom_core::spline::compose::patch::PatchSpans; 3], su, sv| {
            norm_sup(&[
                ch[0].cell_hull(su, sv),
                ch[1].cell_hull(su, sv),
                ch[2].cell_hull(su, sv),
            ])
        };
        let (mut cells, mut tighter, mut worst_m, mut worst_y) = (0usize, 0usize, 0.0f64, 0.0f64);
        for su in 0..nu {
            for sv in 0..nv {
                let (m_sup, y_sup) = (comp.m_tilde_sup(su, sv), comp.y_sup(su, sv));
                assert!(
                    m_sup.is_finite() && y_sup.is_finite(),
                    "{name} cell ({su},{sv}): sup‖M̃‖ = {m_sup:e}, sup‖Y‖ = {y_sup:e}"
                );
                let (m_box, y_box) = (box_sup(&comp.m_tilde, su, sv), box_sup(&comp.y, su, sv));
                assert!(
                    m_sup <= m_box && y_sup <= y_box,
                    "{name} cell ({su},{sv}): a coefficient-norm reading exceeds its box \
                     (M̃ {m_sup:e} > {m_box:e} or Y {y_sup:e} > {y_box:e})"
                );
                if m_sup < m_box || y_sup < y_box {
                    tighter += 1;
                }
                cells += 1;
                let (ub, vb) = comp.cell_box(su, sv);
                for a in 0..N {
                    #[allow(clippy::cast_precision_loss)]
                    let u = ub.0 + (ub.1 - ub.0) * (a as f64 + 0.5) / N as f64;
                    for b in 0..N {
                        #[allow(clippy::cast_precision_loss)]
                        let v = vb.0 + (vb.1 - vb.0) * (b as f64 + 0.5) / N as f64;
                        let jet = base.ders(u, v);
                        let w = weight_at(&base, u, v);
                        let m = jet.du.cross(jet.dv) * w.powi(3);
                        let e = (fit.eval(u, v) - base.eval(u, v)) * (w * weight_at(&fit, u, v));
                        let y = e.cross(m);
                        worst_m = max_bound(worst_m, m.norm() / m_sup);
                        worst_y = max_bound(worst_y, y.norm() / y_sup);
                    }
                }
            }
        }
        assert!(cells > 0, "{name}: no cell");
        assert!(
            worst_m <= 1.0 + 1e-9 && worst_y <= 1.0 + 1e-9,
            "{name}: a sample exceeds its certified sup — ‖M̃‖ by {worst_m}, ‖Y‖ by {worst_y}"
        );
        eprintln!(
            "{name} d={d:e}: {cells} cells, {tighter} tighter than the box; \
             worst sample/sup ‖M̃‖ {worst_m:.4}, ‖Y‖ {worst_y:.4}"
        );
    }

    /// **The soundness claim, attacked by sampling.** `e_floors`
    /// claims `‖E(u,v)‖ ≥ e_lo` everywhere in the cell, with
    /// `E = S_fit − S`. Nothing else in the suite tests that claim
    /// against the surfaces themselves: the no-rise rows compare two
    /// readings with each other, and the decomposition rows pin
    /// digits. This one samples `E` on a grid inside every cell and
    /// reds if the floor ever exceeds the sampled minimum.
    ///
    /// A sampled minimum is an UPPER bound on the cell's true
    /// infimum, so the test is one-sided by construction: it can
    /// catch an unsound floor and can never certify a sound one.
    /// Measured worst ratio 0.9434 over this grid's cells, 0.9866 on
    /// the quarter cylinder at `d = 0.01` and 0.9984 on the bumpy
    /// patch at `d = 1e-5` — the floor is below every sample it was
    /// checked against, on every base tried, with headroom.
    #[test]
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn the_floor_on_norm_e_never_exceeds_a_sampled_norm_e() {
        let band = Band::linear(Tol::witness()).unwrap();
        const N: usize = 11;
        let (name, base, d, tol) = ("qc", quarter_cylinder(), 1e-6, 1e-3);
        let (fit, _) = super::fit_offset_at(&base, d, tol, band).unwrap();
        let comp = Composite::build(&base, &fit, d).unwrap();
        let (nu, nv) = comp.x.cell_counts();
        let (mut worst, mut worst_at, mut checked) = (0.0f64, (0usize, 0usize), 0usize);
        for su in 0..nu {
            for sv in 0..nv {
                let (mig_iv, proj_iv) = comp.e_floors(su, sv);
                let dh = comp.dd.cell_hull(su, sv);
                let definite = if d > 0.0 {
                    dh.lo() > 0.0
                } else {
                    dh.hi() < 0.0
                };
                if !definite {
                    continue;
                }
                let e_lo = e_low_witness(mig_iv, proj_iv);
                assert!(
                    !e_lo.is_nan(),
                    "{name} d={d:e}: the floor on ‖E‖ is NaN at ({su},{sv})"
                );
                if e_lo <= 0.0 {
                    continue;
                }
                let (ub, vb) = comp.cell_box(su, sv);
                let mut min_norm = f64::INFINITY;
                for a in 0..N {
                    #[allow(clippy::cast_precision_loss)]
                    let u = ub.0 + (ub.1 - ub.0) * (a as f64) / ((N - 1) as f64);
                    for b in 0..N {
                        #[allow(clippy::cast_precision_loss)]
                        let v = vb.0 + (vb.1 - vb.0) * (b as f64) / ((N - 1) as f64);
                        min_norm = min_bound(min_norm, (fit.eval(u, v) - base.eval(u, v)).norm());
                    }
                }
                checked += 1;
                let ratio = e_lo / min_norm;
                if !(ratio <= worst) {
                    worst = ratio;
                    worst_at = (su, sv);
                }
            }
        }
        assert!(checked > 0, "{name} d={d:e}: no cell carried a floor");
        assert!(
            worst <= 1.0,
            "{name} d={d:e}: the floor exceeds a sampled ‖E‖ by {worst} at {worst_at:?}"
        );
        eprintln!(
            "{name} d={d:e}: {checked} cells, worst e_lo/min‖E‖ = {worst:.4} at {worst_at:?}"
        );
    }

    /// The direction rule reads model-space extent, not chart extent:
    /// the same cell box marks `u` or `v` depending only on which
    /// chart speed makes its side longer in metres.
    #[test]
    fn the_direction_rule_compares_metres_not_parameters() {
        let us = vec![0.0, 0.5, 1.0];
        let vs = vec![0.0, 0.5, 1.0];
        let failing = [((0.0, 0.5), (0.0, 0.5))];
        // Equal chart extents, u ten times faster: u is marked.
        let (mu, mv) = directional_mark(&us, &vs, &failing, 10.0, 1.0);
        assert_eq!(mu, vec![true, false]);
        assert_eq!(mv, vec![false, false]);
        // The same cell, v ten times faster: v is marked instead.
        let (mu, mv) = directional_mark(&us, &vs, &failing, 1.0, 10.0);
        assert_eq!(mu, vec![false, false]);
        assert_eq!(mv, vec![true, false]);
        // A tie goes to u, on structure (D9).
        let (mu, mv) = directional_mark(&us, &vs, &failing, 3.0, 3.0);
        assert_eq!(mu, vec![true, false]);
        assert_eq!(mv, vec![false, false]);
    }
    /// A degree-1 direction's partial is its per-span constant, scaled
    /// by the span: on the bilinear `S = (u, v, u·v/2)` over
    /// `[0, 2] × [0, 1]`, `M̃ = S_u × S_v = (−v/2, −u/2, 1)`, so its
    /// three channels range over exactly `[−1/2, 0]`, `[−1, 0]` and
    /// `{1}`. A partial read on the wrong direction, or unscaled by the
    /// span width, moves a range.
    #[test]
    fn a_degree_1_partial_is_the_spans_constant() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 2.0, 2.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let base = geom::NurbsSurface::new(
            ku,
            kv,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(2.0, 0.0, 0.0),
                Point3::new(2.0, 1.0, 1.0),
            ],
            vec![1.0; 4],
        )
        .unwrap();
        let comp = Composite::build(&base, &base, 0.05).unwrap();
        assert_eq!(comp.m_tilde[0].cell_counts(), (1, 1), "cell structure");
        for (c, (lo, hi)) in [(-0.5, 0.0), (-1.0, 0.0), (1.0, 1.0)]
            .into_iter()
            .enumerate()
        {
            let h = comp.m_tilde[c].cell_hull(0, 0);
            assert!(
                h.lo() <= lo && hi <= h.hi() && lo - h.lo() < 1e-12 && h.hi() - hi < 1e-12,
                "M̃ channel {c} reads {h:?}, not [{lo}, {hi}]"
            );
        }
    }

    /// **The composite is about the supplied net, not a rounded
    /// elevation of it.** A rational bilinear with weight `1` on the
    /// `u = 0` ruling and `0.6` on `u = 1`, fitted by itself: every
    /// coefficient of `Ẽ` encloses `0`, and the base's weight channel,
    /// raised to degree 2 in `u`, encloses the exact middle weight
    /// `(1 + 0.6)/2` of the stored `0.6`. An `f64` elevation stores
    /// that weight as the point `0.8`, `5.6e-17` above it.
    #[test]
    fn a_rational_degree_1_composite_encloses_the_supplied_net() {
        let k1 = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let base = geom::NurbsSurface::new(
            k1(),
            k1(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.1),
                Point3::new(1.0, 0.0, 0.2),
                Point3::new(1.0, 1.0, 1.0),
            ],
            vec![1.0, 1.0, 0.6, 0.6],
        )
        .unwrap();
        let comp = Composite::build(&base, &base, 0.05).unwrap();
        for c in 0..3 {
            for (k, z) in comp.e[c].cell_coeffs(0, 0).unwrap().iter().enumerate() {
                assert!(
                    z.lo() <= 0.0 && 0.0 <= z.hi(),
                    "Ẽ{c} coefficient {k} reads {z:?}"
                );
            }
        }
        assert_eq!(comp.w.degree(), (2, 2), "the weight channel's bidegree");
        let w = comp.w.cell_coeffs(0, 0).unwrap();
        // Row-major in `u`: the middle `u` row is entries 3..6. `2·x − 1`
        // is exact for `2·x` in `[1, 2]` (Sterbenz), so the comparison
        // with the stored `0.6` is too.
        for (k, h) in w.iter().enumerate().take(6).skip(3) {
            assert!(
                2.0 * h.lo() - 1.0 <= 0.6 && 0.6 <= 2.0 * h.hi() - 1.0,
                "w coefficient {k} reads {h:?}, which misses (1 + 0.6)/2"
            );
        }
    }

    /// The bits of every cell bound of the `(base, fit)` composite at
    /// `d`, on the meters' floor for `base`.
    fn cell_bound_bits(
        base: &geom::NurbsSurface<f64>,
        fit: &geom::NurbsSurface<f64>,
        d: f64,
    ) -> Vec<u64> {
        let band = Band::linear(Tol::witness()).unwrap();
        let (reg, _) = crate::offset_meters::meter_patch(base, d, band).unwrap();
        let comp = Composite::build(base, fit, d).unwrap();
        let (nu, nv) = comp.x.cell_counts();
        (0..nu)
            .flat_map(|su| (0..nv).map(move |sv| (su, sv)))
            .map(|(su, sv)| comp.cell_bound(su, sv, reg.floor, d).to_bits())
            .collect()
    }

    /// `n` with its control net scaled by `k` about the `z` axis.
    fn radially_scaled(n: &geom::NurbsSurface<f64>, k: f64) -> geom::NurbsSurface<f64> {
        n.map_points(|p| Point3::new(k * p.x, k * p.y, p.z))
    }

    /// **One power of two on every weight moves no cell bound.** The
    /// quarter cylinder against its exact offset (the cylinder of
    /// radius `1 + d`, same weights): at `2⁻³⁰⁰` the base's `w³` and
    /// `w⁴` underflow to zero, so `cell_terms`' weight and sign-witness
    /// guards would refuse every cell; at `2³⁰⁰` they overflow;
    /// `2⁻¹⁰⁰⁰` sits at the bottom of the normal range. Each scale, on
    /// the base or on the fit, describes the same surface, so each cell
    /// bound is the unit pair's to the bit.
    ///
    /// **A constant weight is the polynomial arm's.** The same control
    /// net with every weight one constant is the polynomial surface on
    /// that net, so it takes the spatial form, and every cell bound is
    /// the unit-weight net's to the bit: at `3` and `0.1`, and at every
    /// power-of-two scale — `2⁻¹`, `2⁻³⁰⁰` and `2³⁰⁰` as at `2¹`. Red where a constant weight is read as
    /// rational: the homogeneous arm's sup on this net is
    /// `1.018_060_029_513_912e-1`.
    #[test]
    fn a_uniform_weight_scale_moves_no_cell_bound() {
        use crate::patch_bound::tests::weights_times_pow2;
        let d = 0.3;
        let cylinder = quarter_cylinder();
        let offset = radially_scaled(&cylinder, 1.0 + d);
        let want = cell_bound_bits(&cylinder, &offset, d);
        assert!(
            want.iter().all(|b| f64::from_bits(*b).is_finite()),
            "the unit pair left a cell unbounded"
        );
        for (kb, kf) in [(-300, 0), (300, 0), (-1000, 0), (0, -300), (-300, 600)] {
            let got = cell_bound_bits(
                &weights_times_pow2(&cylinder, kb),
                &weights_times_pow2(&offset, kf),
                d,
            );
            assert_eq!(got, want, "base 2^{kb}, fit 2^{kf}: a cell bound moved");
        }

        let constant = |w: f64| {
            geom::NurbsSurface::new(
                cylinder.knots_u_f64_interim().clone(),
                cylinder.knots_v_f64_interim().clone(),
                cylinder.control().to_vec(),
                vec![w; 6],
            )
            .unwrap()
        };
        let (ones, twos) = (constant(1.0), constant(2.0));
        let want = cell_bound_bits(&ones, &radially_scaled(&ones, 1.0 + d), d);
        let fit = radially_scaled(&twos, 1.0 + d);
        assert_eq!(
            cell_bound_bits(&twos, &fit, d),
            want,
            "the weights-2 net's cell bounds are not the unit-weight net's"
        );
        for k in [-2, -301, 299] {
            let got = cell_bound_bits(&weights_times_pow2(&twos, k), &fit, d);
            assert_eq!(got, want, "weights 2^{}: a cell bound moved", k + 1);
        }
        for c in [3.0, 0.1] {
            let net = constant(c);
            let got = cell_bound_bits(&net, &radially_scaled(&net, 1.0 + d), d);
            assert_eq!(got, want, "weights {c}: a cell bound moved");
        }
    }

    /// **The fit certifies a uniformly scaled face as it does the unit
    /// face.** At `2⁻³⁰⁰` the loop's every round would otherwise end
    /// unbounded, and the fit `BoundNotFinite` with no bound behind it.
    #[test]
    fn a_uniformly_scaled_face_certifies_as_the_unit_face() {
        use crate::patch_bound::tests::weights_times_pow2;
        let (d, tol) = (0.3, 1e-3);
        let band = Band::linear(Tol::witness()).unwrap();
        let unit = quarter_cylinder();
        let (_, cert) = super::fit_offset_at(&unit, d, tol, band).unwrap();
        let (_, scaled) = super::fit_offset_at(&weights_times_pow2(&unit, -300), d, tol, band)
            .unwrap_or_else(|e| panic!("the 2^-300 face refused: {e}"));
        assert_eq!(scaled.rounds, 3, "the 2^-300 face's rounds");
        assert_eq!(
            scaled.hull_sup.to_bits(),
            cert.hull_sup.to_bits(),
            "the 2^-300 face certified {:e}, the unit face {:e}",
            scaled.hull_sup,
            cert.hull_sup
        );
    }

    /// A degree-1 direction with an interior knot has a partial that
    /// jumps there, and the composite refuses it as the crease it is
    /// rather than reading the first span's constant across both.
    #[test]
    fn a_two_span_degree_1_direction_refuses_at_the_composite() {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let base = geom::NurbsSurface::new(
            ku,
            kv,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(0.5, 0.0, 0.2),
                Point3::new(0.5, 1.0, 0.1),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.3),
            ],
            vec![1.0; 6],
        )
        .unwrap();
        let err = Composite::build(&base, &base, 0.05).err();
        assert!(
            matches!(
                err,
                Some(super::OffsetFitError::PatchBound(
                    crate::patch_bound::PatchBoundError::Degree1Crease
                ))
            ),
            "a two-span degree-1 direction reached the composite as {err:?}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod recourse_tests {
    use super::{BestBound, OffsetFitError, OffsetFitWall, OffsetLimb};
    use crate::offset_meters::MeterError;
    use crate::patch_bound::PatchBoundError;
    use crate::recourse::{Classified, NARROW_SPAN_RECOURSE, Reading, Refused};
    use geom::curves::fit::FitError;
    use geom_core::spline::SplineError;
    use geom_core::{BandError, BandField};

    /// **The recourse claim for the carrier tier 3 renders whole.**
    /// `ValidationError::ApproxCertification { error }` contributes a
    /// face key and nothing else, so whatever this enum fails to say is
    /// absent from the message a user reads there.
    ///
    /// **The delegating arms are asserted differently.** `Meter`,
    /// `PatchBound`, `Fit` and `Structure` forward a carrier that
    /// holds an enforcement row of its own, so each is asserted
    /// TRANSITIVELY: the carrier is rendered whole, and its recourse
    /// survives into the message.
    ///
    /// `Band` does NOT render its carrier. The `BandError`'s repairs
    /// are addressed to a caller choosing thresholds, and a `Tol`
    /// caller holds a valid tolerance already. So its rows assert the
    /// opposite: the carrier's prose is absent, and the one recourse is
    /// the arm's own.
    ///
    /// **A floor, not a proof**, on the terms `topo`'s
    /// `every_chart_region_arm_names_a_recourse` states: a vocabulary
    /// check cannot tell a recourse from a sentence containing one of
    /// its words, and an arm whose recourse uses a word not listed
    /// fails it honestly — extend the list in the same change.
    #[test]
    fn every_offset_fit_error_arm_names_a_recourse() {
        // A vocabulary, not a part-of-speech test: an arm that names
        // what the caller changes satisfies the claim the same way an
        // imperative does.
        const RECOURSE_WORDS: &[&str] = &[
            "supply",
            "repair",
            "loosen",
            "ask",
            "re-fit",
            "split",
            "report",
            "describe",
            "drop",
            "larger magnitude",
            "less extreme",
            "widen",
        ];
        let meter = MeterError::NormalFloor {
            floor: 0.0,
            speed_lever: 1.0,
            verdict: Refused::Zero(Classified {
                margin: geom_core::MarginDiag::value(0.0),
                band: geom_core::Band::new(1e-9, 1e-8).unwrap(),
            }),
        };
        let patch_bound = PatchBoundError::DegreeZero;
        let fit = FitError::TooFewPoints { have: 1, need: 2 };
        let crowded = FitError::ParamCountMismatch {
            params: 5,
            points: 5,
        };
        let unproven = FitError::NonFinitePoint { index: 3 };
        let structure = SplineError::NonPositiveWeight {
            index: 0,
            weight: 0.0,
        };
        // The two `BandError` arms `Band::linear` can return.
        let bands = [
            BandError::InvalidValue {
                field: BandField::Escalate,
                value: f64::INFINITY,
            },
            BandError::Empty {
                zero: 5e-324,
                escalate: 5e-324,
            },
        ];
        let arms = [
            OffsetFitError::Meter(meter),
            OffsetFitError::PatchBound(patch_bound),
            OffsetFitError::Fit(fit.clone()),
            OffsetFitError::Fit(crowded.clone()),
            OffsetFitError::Fit(unproven.clone()),
            OffsetFitError::Structure(structure.clone()),
            OffsetFitError::InvalidRequest {
                d: 0.0,
                tolerance: 0.0,
            },
            OffsetFitError::NonFiniteSample { uv: (0.5, 0.5) },
            OffsetFitError::SeedOverCap {
                cap: 64,
                grid: (9, 8),
            },
            OffsetFitError::WallReached {
                wall: OffsetFitWall::SampleCap { cap: 64 },
                rounds: 3,
                grid: (64, 5),
                achieved: 2e-9,
                tolerance: 1e-9,
                best: BestBound {
                    bound: 1.5e-9,
                    grid: (32, 5),
                },
            },
            OffsetFitError::WallReached {
                wall: OffsetFitWall::Work {
                    budget: 512,
                    spent: 480,
                },
                rounds: 3,
                grid: (32, 5),
                achieved: 2e-9,
                tolerance: 1e-9,
                best: BestBound {
                    bound: 1.5e-9,
                    grid: (16, 5),
                },
            },
            OffsetFitError::BoundNotFinite {
                rounds: 3,
                grid: (5, 5),
                d: 1e-3,
                tolerance: 1e-9,
                best: None,
            },
            OffsetFitError::BoundNotFinite {
                rounds: 3,
                grid: (5, 5),
                d: 1e-3,
                tolerance: 1e-9,
                best: Some(BestBound {
                    bound: 2e-9,
                    grid: (4, 5),
                }),
            },
            OffsetFitError::RefinementStalled {
                rounds: 3,
                grid: (5, 5),
                achieved: 2e-9,
                tolerance: 1e-9,
                best: BestBound {
                    bound: 1.5e-9,
                    grid: (4, 5),
                },
            },
            OffsetFitError::WindowUnsupported {
                window: geom::ApproxWindow {
                    u: (0.25, 0.75),
                    v: (0.25, 0.75),
                },
            },
            OffsetFitError::Limb {
                limb: OffsetLimb::OnLocus,
                bound: 2e-9,
                tolerance: 1e-9,
            },
            OffsetFitError::Limb {
                limb: OffsetLimb::HullSup,
                bound: 2e-9,
                tolerance: 1e-9,
            },
            OffsetFitError::MintLimb {
                limb: OffsetLimb::OnLocus,
                bound: f64::NAN,
                tolerance: 1e-9,
            },
            OffsetFitError::MintLimb {
                limb: OffsetLimb::OnLocus,
                bound: 2e-9,
                tolerance: 1e-9,
            },
            OffsetFitError::Limb {
                limb: OffsetLimb::OnLocus,
                bound: f64::NAN,
                tolerance: 1e-9,
            },
            OffsetFitError::Band(bands[0]),
            OffsetFitError::Band(bands[1]),
        ];
        // Fourteen variants; `Fit` at its two narrow-span arms and an
        // unproven one, `WallReached` at both walls, which the
        // message names in two different words, `BoundNotFinite` at
        // both of its `best` cases, which are two different messages sending the caller
        // to two different repairs, `Limb` at both limbs, which the
        // message names in two different words, and at a NaN bound,
        // which it names as no number, `MintLimb` at its limb's NaN and
        // finite bounds, and `Band` at both arms `Band::linear` can
        // return.
        assert_eq!(arms.len(), 22, "an arm was added without a row here");
        for arm in &arms {
            let msg = arm.to_string();
            let delegated = match arm {
                OffsetFitError::Meter(_) => Some(meter.render(Reading::Build)),
                OffsetFitError::PatchBound(_) => Some(patch_bound.to_string()),
                _ => None,
            };
            // Two of the carriers hold an enforcement row of their own
            // (`each_meter_arm_ends_in_its_decisions_recourse`,
            // `every_patch_bound_error_arm_names_a_recourse`), so those
            // arms are asserted TRANSITIVELY: the carrier is rendered
            // whole AND its clause survives into the message a caller
            // reads. The carrier's row is what makes that a statement
            // about every payload rather than about the one built here.
            if let Some(carrier) = delegated {
                assert!(msg.contains(&carrier), "carrier not rendered whole: {msg}");
            }
            // The fit's and the structure's carriers address a caller
            // supplying the data; here the kernel supplied it, so the
            // arm renders its own ending instead of their repairs: the
            // narrow span's lever where a seed ran short, the last
            // resort on every arm not shown reachable or excluded.
            let kernel_input = match arm {
                OffsetFitError::Fit(FitError::TooFewPoints { .. }) => {
                    Some((fit.to_string(), NARROW_SPAN_RECOURSE))
                }
                OffsetFitError::Fit(FitError::ParamCountMismatch { .. }) => {
                    Some((crowded.to_string(), NARROW_SPAN_RECOURSE))
                }
                OffsetFitError::Fit(_) => {
                    Some((unproven.to_string(), geom_core::KERNEL_LIMIT_RECOURSE))
                }
                OffsetFitError::Structure(_) => {
                    Some((structure.to_string(), geom_core::KERNEL_LIMIT_RECOURSE))
                }
                _ => None,
            };
            if let Some((carrier, ending)) = kernel_input {
                assert!(
                    !msg.contains(&carrier) && msg.ends_with(ending),
                    "not the arm's own ending in place of the carrier's repair: {msg}"
                );
            }
            // The at-rest limb's repair is the re-fit; at the mint the
            // re-fit is the call that refused, so the last resort instead.
            match arm {
                OffsetFitError::Limb { .. } => assert!(
                    msg.starts_with("a stored offset surface does not hold to the tolerance")
                        && msg.ends_with(super::LIMB_REFIT_RECOURSE),
                    "the at-rest limb lost its re-fit: {msg}"
                ),
                OffsetFitError::MintLimb { .. } => assert!(
                    !msg.contains("stored") && msg.ends_with(geom_core::KERNEL_LIMIT_RECOURSE),
                    "the mint's limb is not the last resort: {msg}"
                ),
                _ => {}
            }
            // A NaN bound is no number of metres, at rest or at the mint.
            assert!(!msg.contains("NaN"), "a NaN rendered as metres: {msg}");
            if let OffsetFitError::Band(band) = arm {
                assert!(
                    !msg.contains(&band.to_string()),
                    "the carrier's own repairs are rendered: {msg}"
                );
                assert!(
                    msg.contains("Recourse: run at a less extreme tolerance"),
                    "not exactly the one recourse: {msg}"
                );
            }
            // One ending per refusal, a forwarded carrier's included:
            // its repair is labelled, so a `Recourse:` added here on top
            // of it would count two.
            assert_eq!(
                test_utils::refusal::recourse_markers(&msg),
                1,
                "not exactly one ending: {msg}"
            );
            let lower = msg.to_lowercase();
            assert!(
                RECOURSE_WORDS.iter().any(|w| lower.contains(w)),
                "no recourse in: {msg}"
            );
        }
    }
}

/// The refinement loop's ORDERING, driven by scripted bound sequences
/// through [`refine_rounds_at`] itself: idle rounds counted against the
/// lowest bound, the stall before the walls, and both walls, the cap
/// read as a total. A fixture
/// grounded in the enclosure's width stops stalling whenever the
/// assembly narrows, so the ordering is pinned here, on bounds that
/// cannot move.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod scripted_rounds {
    use super::{
        OFFSET_FIT_SAMPLE_CAP, OFFSET_FIT_WORK, OffsetFitError, OffsetFitWall, Report,
        refine_rounds_at,
    };

    /// Runs the loop from a 3 × 3 seed at chart speeds `speeds`, round
    /// `k` reporting `bound(k)` against a tolerance of 1, with one
    /// failing cell: the schedule's first, so each round grows the grid
    /// by a sample or two, or where `whole` the whole chart, so each
    /// round bisects every interval it marks.
    fn run_over(
        speeds: (f64, f64),
        whole: bool,
        bound: impl Fn(usize) -> f64,
    ) -> Result<usize, OffsetFitError> {
        run_from(
            (vec![0.0, 0.5, 1.0], vec![0.0, 0.5, 1.0]),
            speeds,
            whole,
            bound,
        )
    }

    /// [`run_over`] from the seed `seed`.
    fn run_from(
        seed: (Vec<f64>, Vec<f64>),
        speeds: (f64, f64),
        whole: bool,
        bound: impl Fn(usize) -> f64,
    ) -> Result<usize, OffsetFitError> {
        let mut k = 0;
        refine_rounds_at(seed, speeds, 0.1, 1.0, |us, vs| {
            let hull_sup = bound(k);
            k += 1;
            let (u1, v1) = if whole {
                (us[us.len() - 1], vs[vs.len() - 1])
            } else {
                (us[1], vs[1])
            };
            Ok((
                (),
                Report {
                    cells: 1,
                    on_locus_max: 0.0,
                    hull_sup,
                    failing: vec![((us[0], u1), (vs[0], v1))],
                },
            ))
        })
        .map(|(_, _, round)| round)
    }

    fn run_at(speeds: (f64, f64), bound: impl Fn(usize) -> f64) -> Result<usize, OffsetFitError> {
        run_over(speeds, false, bound)
    }

    fn run(bound: impl Fn(usize) -> f64) -> Result<usize, OffsetFitError> {
        run_at((1.0, 1.0), bound)
    }

    fn script(bounds: &[f64]) -> impl Fn(usize) -> f64 + '_ {
        |k| bounds[k]
    }

    #[test]
    fn a_bound_under_the_tolerance_certifies_on_its_round() {
        assert_eq!(run(script(&[f64::INFINITY, 4.0, 0.5])).unwrap(), 2);
    }

    /// Round 3 does not lower the lowest bound, so its successor is
    /// marked both ways; round 4 does not either, and the loop refuses
    /// there.
    #[test]
    fn a_both_directions_round_that_gains_nothing_stalls() {
        match run(script(&[f64::INFINITY, 5.0, 4.0, 4.0, 4.0])) {
            Err(
                ref e @ OffsetFitError::RefinementStalled {
                    rounds,
                    achieved,
                    best,
                    ..
                },
            ) => {
                assert_eq!((rounds, achieved, best.bound), (4, 4.0, 4.0));
                let msg = e.to_string();
                assert!(
                    msg.contains("stopped improving") && msg.contains("both directions"),
                    "the message says what was tried: {msg}"
                );
            }
            other => panic!("the stall's face: {other:?}"),
        }
    }

    /// One idle round is answered by the both-directions step, not a
    /// refusal, and a loop that lowers its bound again after it goes on
    /// to certify.
    #[test]
    fn a_single_idle_round_takes_the_both_directions_step() {
        assert_eq!(
            run(script(&[f64::INFINITY, 5.0, 6.0, 4.0, 0.5])).unwrap(),
            4
        );
    }

    /// **Idle is counted against the LOWEST bound.** Round 2 rises
    /// above round 1's 5, and round 3 recovers part of the way, to 5.5:
    /// below its predecessor, so a previous-round reading would call it
    /// progress and refine on, but still above the lowest, so it is the
    /// second idle round and the loop stalls there, naming round 1's
    /// bound and grid.
    #[test]
    fn a_rise_then_a_partial_recovery_stalls() {
        match run(script(&[f64::INFINITY, 5.0, 6.0, 5.5, 0.5])) {
            Err(OffsetFitError::RefinementStalled {
                rounds,
                grid,
                achieved,
                best,
                ..
            }) => {
                assert_eq!((rounds, achieved, best.bound), (3, 5.5, 5.0));
                assert_eq!(best.grid, (4, 3), "the grid round 1 reached it on");
                assert_eq!(grid, (5, 5), "round 3's grid, marked both ways");
            }
            other => panic!("the stall's face: {other:?}"),
        }
    }

    /// **A fallback marking is not an idle round.** `v` is the faster
    /// direction, so the directional marking bisects the failing cell's
    /// `v` interval, `[0, 5e-324]`, which has no midpoint strictly
    /// inside it: round 0's marking grows nothing and falls back to both
    /// directions. Round 1's bound rises, its first idle round, and is
    /// answered by the both-directions step again rather than refused;
    /// round 2 certifies. Idle is read on the bound alone, so the
    /// fallback does not stand in for the first idle round.
    #[test]
    fn a_fallback_marking_is_not_an_idle_round() {
        let seed = (vec![0.0, 0.5, 1.0], vec![0.0, 5e-324, 1.0]);
        assert_eq!(
            run_from(seed, (0.0, 1.0), false, script(&[5.0, 6.0, 0.5])).unwrap(),
            2
        );
    }

    /// An unbounded round after a finite one is idle: two of them
    /// stall, carrying the finite bound behind them.
    #[test]
    fn a_bound_lost_for_two_rounds_stalls_on_the_bound_it_lost() {
        let inf = f64::INFINITY;
        match run(script(&[inf, 5.0, inf, inf, 0.5])) {
            Err(OffsetFitError::RefinementStalled {
                rounds,
                achieved,
                best,
                ..
            }) => assert_eq!((rounds, achieved, best.bound), (3, inf, 5.0)),
            other => panic!("the stall's face: {other:?}"),
        }
    }

    /// No round count stops a loop whose bound keeps falling: twenty
    /// improving rounds, then a certifying one.
    #[test]
    fn a_loop_still_improving_is_not_stopped_by_a_round_count() {
        let rounds = run(|k| if k < 20 { 100.0 - k as f64 } else { 0.5 }).unwrap();
        assert_eq!(rounds, 20);
    }

    /// A seed over the cap refuses before any round is fitted: the cap
    /// reads the seed's grid as it reads every later one.
    #[test]
    fn a_seed_over_the_cap_refuses_before_round_0() {
        let side: Vec<f64> = (0..49).map(|i| f64::from(i) / 48.0).collect();
        assert!(side.len() * side.len() > OFFSET_FIT_SAMPLE_CAP);
        let mut fitted = 0;
        let r = refine_rounds_at((side.clone(), side), (1.0, 1.0), 0.1, 1.0, |_, _| {
            fitted += 1;
            Ok((
                (),
                Report {
                    cells: 1,
                    on_locus_max: 0.0,
                    hull_sup: 0.5,
                    failing: Vec::new(),
                },
            ))
        });
        assert!(
            matches!(
                r,
                Err(OffsetFitError::SeedOverCap {
                    cap: OFFSET_FIT_SAMPLE_CAP,
                    grid: (49, 49)
                })
            ),
            "a 49 x 49 seed: {:?}",
            r.as_ref().err()
        );
        assert_eq!(fitted, 0, "a round was fitted on a seed over the cap");
    }

    /// A loop that never certifies and never stalls, growing its grid
    /// by one sample a round, ends on the cumulative budget: the cap
    /// would let it run for over seven hundred rounds. A `v` that does
    /// not move in model space keeps every directional marking in `u`.
    #[test]
    fn a_slowly_growing_loop_ends_on_the_work_budget() {
        match run_at((1.0, 0.0), |k| 1e3 / (1.0 + k as f64)) {
            Err(
                ref e @ OffsetFitError::WallReached {
                    wall: OffsetFitWall::Work { budget, spent },
                    grid,
                    rounds,
                    ..
                },
            ) => {
                assert_eq!(budget, OFFSET_FIT_WORK);
                assert_eq!(grid.1, 3, "a still v is never refined");
                assert_eq!(rounds as usize, grid.0 - 3, "one sample a round");
                let fitted: usize = (3..=grid.0).map(|n| n * 3).sum();
                assert_eq!(
                    spent, fitted,
                    "every grid fitted is spent, the seed's included"
                );
                assert!(
                    spent <= budget && spent + (grid.0 + 1) * 3 > budget,
                    "the budget stops the first round that would pass it: {spent}"
                );
                assert!(
                    grid.0 * grid.1 < OFFSET_FIT_SAMPLE_CAP,
                    "under the cap: {grid:?}"
                );
                assert!(
                    e.to_string()
                        .contains(&format!("limit of {budget} samples fitted in total,")),
                    "the message names the budget: {e}"
                );
            }
            other => panic!("the work budget's face: {other:?}"),
        }
    }

    /// A loop that never certifies and never stalls, doubling its grid
    /// in `u` each round, ends on the cap, and the cap is a TOTAL: the
    /// grid stops with one direction far past `sqrt(cap)` and its
    /// product within the cap, which a per-direction cap at `sqrt(cap)`
    /// would have refused long before.
    #[test]
    fn a_fast_growing_loop_ends_on_the_total_sample_cap() {
        match run_over((1.0, 0.0), true, |k| 1e3 / (1.0 + k as f64)) {
            Err(OffsetFitError::WallReached {
                wall: OffsetFitWall::SampleCap { cap },
                grid,
                rounds,
                ..
            }) => {
                assert_eq!(cap, OFFSET_FIT_SAMPLE_CAP);
                assert_eq!(grid, (513, 3), "u doubles its intervals each round");
                assert_eq!(rounds, 8);
                assert!(
                    (2 * grid.0 - 1) * grid.1 > cap,
                    "the cap stops the first round whose grid would exceed it: {grid:?}"
                );
                assert!(
                    grid.0 * grid.0 > cap,
                    "the u direction alone outgrows a square of the cap: {grid:?}"
                );
            }
            other => panic!("the cap's face: {other:?}"),
        }
    }
}

//! **The offset fit and its certificate** — the Book's §9.4 stack for
//! surfaces, and the two-limb residual bound (`docs/OFFSET-DESIGN.md`
//! O2/O3, `docs/CURVED-DESIGN.md` C8).
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
//! carry the sup, until every cell certifies or the budget expires.
//! What the two share is the thing that matters: a bound decides every
//! step, and exhaustion is a **typed refusal carrying the achieved
//! bound** ([`OffsetFitError::BudgetExhausted`] — the
//! `QuadratureBudget` shape), never an uncertified return.
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
//! the cell's geometric variation. Enclosing `S_fit`, `S` and `d·n`
//! separately and subtracting the enclosures cannot see that: it
//! reports the sum of the two surfaces' motions across the cell, which
//! on a unit cylinder at `d = 0.2 m` would need millions of cells to
//! reach a micron. That failure mode is the one
//! [`geom_core::spline::compose::tensor`]'s docs record at one
//! parameter; here it is answered by
//! [`geom_core::spline::compose::patch`].
//!
//! **The small-`|d|` denominator, and the limit that remains.** The
//! normal component divides `|X|` by `w̃²·(‖E‖ + |d|)`. Bounding that
//! below by `2|d|` alone is both loose and brittle: once `dist`
//! reaches `|d|` the cell collapses to `+∞`, so a micron-scale offset
//! on a metre-scale patch certified as `inf`. The composite therefore
//! carries `Ẽ` and takes a DIRECT mignitude lower bound on `‖E‖` —
//! the same inf-side shape meter 1 uses on the cross product — which
//! makes the small-`|d|` case finite and tightens every other row.
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
//! certifies at the same `3.2e-4` the patch gives at the origin.
//!
//! It did NOT make the bound scale with `|d|`, and the reason is not
//! the one that motivated the recentring. At the origin the small-`d`
//! sup is 96% its `τ²/‖E‖` term, and that term is large because the
//! lower bound on `‖E‖` is assembled from the componentwise
//! mignitudes of `Ẽ`'s cell hulls: on a patch whose normal rotates
//! across the cell each component straddles zero, so the assembly
//! reads `1.6e-8` where `‖E‖ ≈ |d| = 1e-6`. A lower bound that saw
//! the components together rather than one at a time is what would
//! move this row; it is not a rounding problem and recentring cannot
//! reach it.
//!
//! **Where the regularity floor enters.** `τ` and `D` both divide by
//! `‖m‖`, and `X`'s reading divides by `w̃²`. Both weight hulls are
//! positive by the rational licence; `‖m‖` is positive only because
//! [`crate::offset_meters`]' floor says so — `‖M̃‖ ≥ floor·w³`. That
//! is the sense in which meter 1 "makes `1/‖S_u × S_v‖` boundable",
//! and it is why the fit door refuses on the floor before it fits
//! anything.
//!
//! # Discipline
//!
//! The whole stack is **f64 substrate**: fitting is C6 structure
//! selection (same inputs ⇒ same knots and control bits, D9), and the
//! C9 ring's hull bounds are `f64` upper bounds by construction — the
//! `SsiCertificate::hull_sup` posture. Predicates decide through the
//! kernel's one classification funnel.

use geom::curves::fit::{FitError, interpolate_columns};
use geom::surfaces::{NurbsSurface, Surface};
use geom_core::spline::compose::patch::PatchSpans;
use geom_core::spline::{KnotVector, SplineError};
use geom_core::{Band, Point3, ring_interval::RingInterval};

use crate::offset_meters::{MeterError, MeterResult, meter_patch, mig, sqrt_down, sqrt_up};
use crate::patch_bound::{Net, PatchBoundError, derived_knots, is_rational, net_d_u, net_d_v};

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

/// The refinement-round budget of the fit loop. Expiry is the typed
/// [`OffsetFitError::BudgetExhausted`] carrying the achieved bound —
/// the Book's own "both can fail to converge and this eventuality
/// must be dealt with" honesty, as a type.
pub const OFFSET_FIT_BUDGET: usize = 6;

/// The per-direction cap on sample parameters.
///
/// A refinement round bisects, in each worst-carrying cell, the
/// sample intervals of the ONE direction that cell's model-space
/// extent names — so a direction grows only while the residual keeps
/// asking for it, and the growth is per direction rather than
/// uniform. That is why the cap is per direction too: the binding
/// case is a patch whose error lives wholly in one direction, where
/// an unreachable tolerance would drive that direction alone toward
/// an unbounded grid while its partner stands still. The
/// both-directions fallback the stall guard falls back to can double
/// both at once, which the same cap bounds.
///
/// It is the second stopping condition, and it produces the same
/// typed [`OffsetFitError::BudgetExhausted`] refusal as the round
/// budget: never an uncertified return, and never an unbounded amount
/// of work.
pub const OFFSET_FIT_SAMPLE_CAP: usize = 48;

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
}

/// A typed refusal of the offset fit door (fail-loud; the kernel never
/// panics and never returns an uncertified surface).
#[derive(Clone, Debug, PartialEq)]
pub enum OffsetFitError {
    /// A door meter refused: the patch's normal is not certifiably
    /// non-degenerate, or `|d|` reaches its curvature reach.
    Meter(MeterError),
    /// The patch-bound assembly refused (a C⁰ crease, a degree-0
    /// direction, an illegal rational description).
    PatchBound(PatchBoundError),
    /// The interpolation stack refused.
    Fit(FitError),
    /// Spline structure construction refused.
    Structure(SplineError),
    /// `d` or the tolerance is not a finite, non-zero (resp.
    /// positive) number.
    InvalidRequest {
        /// The offset distance as supplied.
        d: f64,
        /// The tolerance as supplied.
        tolerance: f64,
    },
    /// A sampled offset point is non-finite: the base evaluated to
    /// poison at a sample the whole-patch meters admitted in bound.
    NonFiniteSample {
        /// The offending parameters.
        uv: (f64, f64),
    },
    /// The refinement loop stopped without certifying — either
    /// [`OFFSET_FIT_BUDGET`] rounds spent or the per-direction
    /// [`OFFSET_FIT_SAMPLE_CAP`] reached; carries the bound achieved
    /// so far and the grid it was achieved on.
    BudgetExhausted {
        /// The round budget.
        budget: usize,
        /// The sample grid the loop stopped on, per direction.
        grid: (usize, usize),
        /// The certified sup bound at expiry, in metres.
        achieved: f64,
        /// The tolerance it had to reach.
        tolerance: f64,
    },
    /// The refinement loop stopped IMPROVING before the budget ran
    /// out: a round that bisected every failing cell in both
    /// directions did not lower the bound its predecessor reached.
    ///
    /// A different finding from [`Self::BudgetExhausted`], and the
    /// distinction is what the caller does next. Exhaustion says the
    /// loop was still converging and ran out of rounds — more budget
    /// is the answer. This says the loop stopped converging while it
    /// still had rounds in hand, so more of them will not help: the
    /// tolerance is below what this fit's structure can reach on this
    /// patch.
    ///
    /// **D2 classification: row 1** — reachable by input, and invalid
    /// as a request to this door. Row 0 was answered first and
    /// answered no: "the bound stopped falling" is a measured numeric
    /// outcome on admissible input, so no type change can exclude it
    /// without making convergence a type-level property of
    /// caller-supplied geometry.
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
        /// The certified sup bound at the stall, in metres.
        achieved: f64,
        /// The tolerance it had to reach.
        tolerance: f64,
    },
    /// The storage door was asked to certify over a window that is not
    /// the base's own chart rectangle. The certificate this module
    /// derives covers that rectangle and nothing narrower, so a
    /// sub-window claim would be a bound it never proved.
    WindowUnsupported {
        /// The window asked for.
        window: geom::ApproxWindow,
    },
    /// A certificate limb refused on a fit handed in from outside —
    /// the re-derivation door ([`certify_offset`]).
    Limb {
        /// Which limb.
        limb: OffsetLimb,
        /// The bound that limb measured, in metres.
        bound: f64,
        /// The tolerance it was classified against.
        tolerance: f64,
    },
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

impl core::fmt::Display for OffsetFitError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Meter(e) => write!(f, "fit_offset refused at a door meter: {e}"),
            Self::PatchBound(e) => write!(f, "fit_offset: {e}"),
            Self::Fit(e) => write!(f, "fit_offset: the interpolation stack refused: {e}"),
            Self::Structure(e) => write!(f, "fit_offset: spline structure refused: {e:?}"),
            Self::InvalidRequest { d, tolerance } => write!(
                f,
                "fit_offset: the request is not fittable — offset distance {d} m must be \
                 finite and non-zero, tolerance {tolerance} m finite and positive"
            ),
            Self::NonFiniteSample { uv } => write!(
                f,
                "fit_offset: the base surface evaluated to a non-finite offset point at \
                 (u, v) = ({}, {}) — poison in, refusal out",
                uv.0, uv.1
            ),
            Self::BudgetExhausted {
                budget,
                grid,
                achieved,
                tolerance,
            } => write!(
                f,
                "fit_offset: the refinement loop stopped on a {}x{} sample grid without \
                 certifying (round budget {budget}, per-direction cap {}) — the achieved \
                 sup bound is {achieved} m against a tolerance of {tolerance} m; nothing \
                 uncertified is returned",
                grid.0, grid.1, OFFSET_FIT_SAMPLE_CAP
            ),
            Self::RefinementStalled {
                rounds,
                grid,
                achieved,
                tolerance,
            } => write!(
                f,
                "fit_offset: the refinement loop STALLED on a {}x{} sample grid after \
                 {rounds} rounds — bisecting every failing cell in both directions did \
                 not lower the achieved sup bound of {achieved} m, against a tolerance \
                 of {tolerance} m, so the remaining round budget cannot reach it; \
                 nothing uncertified is returned",
                grid.0, grid.1
            ),
            Self::WindowUnsupported { window } => write!(
                f,
                "approx_offset_surface: the window asked for (u {:?}, v {:?}) is not the base's \
                 own chart rectangle, and the certificate covers that rectangle only",
                window.u, window.v
            ),
            Self::Limb {
                limb,
                bound,
                tolerance,
            } => write!(
                f,
                "fit_offset: {} measured {bound} m against a tolerance of {tolerance} m",
                limb.name()
            ),
        }
    }
}

impl std::error::Error for OffsetFitError {}

// The certificate RECORD lives in `geom` (`geom::OffsetCertificate`):
// `ApproxSurface` stores it and `Surface` stores that, so the type has
// to sit below the surface enum. Its derivation is this module's, and
// every limb below writes into it.
pub use geom::OffsetCertificate;

/// **The offset fit door**: fit a NURBS approximation of `S + d·n`
/// over the base's own chart rectangle and certify it.
///
/// Refuses — never degrades — on a patch whose chart normal is not
/// certifiably non-degenerate, on an offset distance that reaches the
/// patch's curvature reach, on budget exhaustion, and on a refinement
/// loop that stops converging while it still has rounds in hand.
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
/// non-finite samples, [`OffsetFitError::BudgetExhausted`] carrying
/// the achieved bound, and [`OffsetFitError::RefinementStalled`]
/// carrying the bound the loop stopped improving on.
pub fn fit_offset(
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

    let (mut us, mut vs) = seed_params(base);
    let mut achieved = f64::INFINITY;
    // The stall guard's state: the previous round's bound, and
    // whether the marking that produced this grid was the
    // both-directions fallback.
    let mut prev_sup = f64::INFINITY;
    let mut marked_both = false;
    for round in 0..=OFFSET_FIT_BUDGET {
        let fit = interpolate_offset_grid(base, d, &us, &vs)?;
        let report = measure(base, &fit, d, reg.floor)?;
        achieved = report.hull_sup;
        if report.hull_sup <= tolerance {
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
            return Ok((fit, cert));
        }
        if round == OFFSET_FIT_BUDGET {
            break;
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
        // quadratic one.
        //
        // **The stall guard is what makes that safe.** The speed
        // ratio is a prediction, and a residual it mispredicts would
        // otherwise refine the useless direction until the budget
        // ran out. A round that did not improve on its predecessor
        // falls back to marking BOTH directions; a both-directions
        // round that still does not improve is not a budget problem
        // and does not become one — it refuses, named.
        #[allow(clippy::cast_possible_truncation)]
        let stalled = |grid: (usize, usize)| OffsetFitError::RefinementStalled {
            rounds: round as u32,
            grid,
            achieved,
            tolerance,
        };
        let verdict = stall_verdict(prev_sup, report.hull_sup, marked_both);
        if verdict == Refine::Refuse {
            return Err(stalled((us.len(), vs.len())));
        }
        prev_sup = report.hull_sup;
        // The mode is CARRIED out of the step that used it, not
        // inferred from a local set beside it: the refusal's admission
        // set is "the round whose schedule came from a both-directions
        // marking", and that is a fact about `next`, not about the
        // order of two statements.
        let mut next = refine_schedule(&us, &vs, &report, reg.speed_u, reg.speed_v, verdict);
        // A directional marking can fail to grow the schedule even
        // though it marked intervals: `bisect` drops a midpoint that
        // is not strictly between its endpoints, which is what an
        // interval narrowed to consecutive floats gives. That is the
        // same evidence as a round that gained nothing, so it takes
        // the same fallback rather than escaping to the budget.
        if !next.grew(&us, &vs) && next.mode == Refine::Directional {
            next = refine_schedule(
                &us,
                &vs,
                &report,
                reg.speed_u,
                reg.speed_v,
                Refine::BothDirections,
            );
        }
        if next.us.len() > OFFSET_FIT_SAMPLE_CAP || next.vs.len() > OFFSET_FIT_SAMPLE_CAP {
            // The per-direction cap: a REFINEMENT limit, not a
            // convergence one, and it keeps its own refusal.
            break;
        }
        if !next.grew(&us, &vs) {
            // Bisecting every failing cell in both directions moved
            // nothing. No later round can move it either — the
            // intervals only narrow — so this is the stall, reached
            // by exhaustion of the schedule rather than of the bound.
            return Err(stalled((us.len(), vs.len())));
        }
        marked_both = next.mode == Refine::BothDirections;
        us = next.us;
        vs = next.vs;
    }
    Err(OffsetFitError::BudgetExhausted {
        budget: OFFSET_FIT_BUDGET,
        grid: (us.len(), vs.len()),
        achieved,
        tolerance,
    })
}

/// Re-derives the certificate of an ALREADY fitted surface against a
/// base and `d`, and classifies both limbs against `tolerance` — the
/// validator posture (O5: never trust a stored certificate), and the
/// door a degraded fit is driven through.
///
/// # Errors
///
/// [`OffsetFitError::Limb`] naming the limb that measured above
/// tolerance, plus the door meters' and the patch-bound refusals.
pub fn certify_offset(
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
/// hand back the [`geom::Surface::Approx`] variant that stores both.
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
/// The window is checked rather than attested — see the closure.
///
/// # Errors
///
/// [`OffsetFitError`]: everything [`fit_offset`] refuses, plus
/// [`certify_offset`]'s limb classifications. A rational fit takes
/// the same path as a polynomial one — the composite is weighted, so
/// rationality is not a refusal cause.
pub fn approx_offset_surface(
    base: std::sync::Arc<NurbsSurface<f64>>,
    d: f64,
    tolerance: f64,
    band: Band,
) -> Result<Surface<f64>, OffsetFitError> {
    let (fit, loop_cert) = fit_offset(&base, d, tolerance, band)?;
    let spec = geom::SurfaceSpec {
        window: geom::ApproxWindow::of(&*base),
        description: geom::SurfaceDescription::Offset { base, d },
        fit,
        tolerance,
    };
    let approx = geom::ApproxSurface::certify(spec, |description, fit, window, tolerance| {
        let geom::SurfaceDescription::Offset { base, d } = description;
        // The certificate covers the base's whole chart rectangle, so
        // the window asked for is honoured exactly when it IS that
        // rectangle. Checked, not attested.
        if window != geom::ApproxWindow::of(base) {
            return Err(OffsetFitError::WindowUnsupported { window });
        }
        certify_offset(base, fit, *d, tolerance, band).map(|cert| OffsetCertificate {
            // Every measured field is the re-derivation's; `rounds` is
            // the FIT's provenance, which a re-measurement cannot
            // recompute, so the loop's honest count travels with it.
            rounds: loop_cert.rounds,
            ..cert
        })
    })?;
    Ok(Surface::Approx(std::sync::Arc::new(approx)))
}

/// **The re-derivation door** (O5's never-trust posture): re-runs
/// [`certify_offset`] against an approximating surface's own stored
/// description and fit, classified against `tolerance`.
///
/// The stored certificate is not read, and neither is the stored
/// tolerance: **the classification tolerance is the CALLER's**, which
/// is what lets tier 3 verify the ratified claim (O3: the residual is
/// `≤ ε_precision`) rather than whatever bound the mint happened to
/// ask for. A fit that has been degraded since it was minted —
/// coarsened, edited, transplanted — fails here with the limb that
/// caught it; so does one minted loose and re-derived at a tighter ε,
/// which is D4's blessed consequence of ε-tightening and the edge
/// machinery's exact behaviour.
///
/// # Errors
///
/// As [`certify_offset`].
pub fn recertify_approx(
    approx: &geom::ApproxSurface<f64>,
    tolerance: f64,
    band: Band,
) -> Result<OffsetCertificate, OffsetFitError> {
    let geom::SurfaceDescription::Offset { base, d } = approx.description();
    certify_offset(base, approx.fit(), *d, tolerance, band)
}

// ---------------------------------------------------------------------
// A9.4 — global surface interpolation at the base's own parameters
// ---------------------------------------------------------------------

/// The seed sample parameters, per direction (module docs).
fn seed_params(base: &NurbsSurface<f64>) -> (Vec<f64>, Vec<f64>) {
    (
        seed_direction(base.knots_u()),
        seed_direction(base.knots_v()),
    )
}

fn seed_direction(kv: &KnotVector) -> Vec<f64> {
    let mut out = Vec::new();
    let knots = kv.knots();
    for span in kv.first_span()..=kv.last_span() {
        let (Some(&lo), Some(&hi)) = (knots.get(span), knots.get(span + 1)) else {
            continue;
        };
        #[allow(clippy::neg_cmp_op_on_partial_ord)]
        if !(hi > lo) {
            continue;
        }
        if out.is_empty() {
            out.push(lo);
        }
        for k in 1..OFFSET_FIT_SEED_PER_SPAN {
            #[allow(clippy::cast_precision_loss)]
            let t = lo + (hi - lo) * (k as f64 / OFFSET_FIT_SEED_PER_SPAN as f64);
            if t > lo && t < hi {
                out.push(t);
            }
        }
        out.push(hi);
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

/// A clamped knot vector affinely rescaled from `0 → 1` onto
/// `[lo, hi]`, with the clamp runs pinned exactly — so the fitted
/// surface lives on the base's own chart rectangle and the
/// certificate's pointwise claim is about the same parameters on both
/// sides.
fn rescaled_knots(kv: &KnotVector, lo: f64, hi: f64) -> Result<KnotVector, SplineError> {
    let span = hi - lo;
    let n = kv.knots().len();
    let p = kv.degree();
    let scaled: Vec<f64> = kv
        .knots()
        .iter()
        .enumerate()
        .map(|(i, t)| {
            if i <= p {
                lo
            } else if i + p + 1 >= n {
                hi
            } else {
                lo + span * *t
            }
        })
        .collect();
    KnotVector::clamped(scaled, p)
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
    let (ulo, uhi) = base.knots_u().domain();
    let (vlo, vhi) = base.knots_v().domain();
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
            row.extend_from_slice(&[p.x, p.y, p.z]);
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
    let ku = rescaled_knots(&ku, ulo, uhi).map_err(OffsetFitError::Structure)?;
    let kv = rescaled_knots(&kv, vlo, vhi).map_err(OffsetFitError::Structure)?;
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
        // A misaligned or poisoned composite has no cells to bound,
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
            hull_sup = hull_sup.max(cell);
            on_locus_max = on_locus_max.max(on_locus_cell(base, fit, d, ub, vb));
            bounds.push((ub, vb, cell));
        }
    }
    // Refinement attacks the cells that carry the sup: every cell
    // within a factor of two of it. A fixed factor, on structure —
    // deterministic (D9), and it cannot mark nothing (the sup's own
    // cell always qualifies).
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

/// What the stall guard says about a round that did not certify.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refine {
    /// Bisect each failing cell in the one direction its model-space
    /// extent names.
    Directional,
    /// Bisect every failing cell in BOTH directions: the last round
    /// did not lower the bound, so the direction rule's prediction is
    /// not to be trusted on this patch.
    BothDirections,
    /// Refuse: a both-directions round did not lower the bound
    /// either.
    Refuse,
}

/// **The stall guard.** `prev_sup` is the bound the previous round
/// reached, `hull_sup` this round's, and `marked_both` whether the
/// marking that produced this round's grid was the both-directions
/// fallback.
///
/// The admission set of [`Refine::Refuse`] — what
/// [`OffsetFitError::RefinementStalled`] refuses — is exactly this: a
/// round whose grid came from bisecting every failing cell in both
/// directions, reporting a bound that did not fall below a finite
/// predecessor. Bisecting everything is the strongest step the loop
/// has, so a round that takes it and gains nothing is telling the
/// caller that the remaining rounds cannot reach the tolerance
/// either. That is a different finding from running out of rounds
/// while still converging, and the two are not merged.
///
/// **`+∞` is not a failure to improve.** A cell whose sign witness or
/// weight hull is not yet proved bounds at `+∞`, which early rounds
/// routinely report; a loop on its way from `+∞` to a finite bound is
/// converging. So the comparison is made only against a FINITE
/// predecessor, and the guard stays silent until there is one.
///
/// # Reachability — a recorded verdict, not an open question
///
/// [`Refine::Refuse`] has not been reached through [`fit_offset`]'s
/// door by any fixture tried: roughly a hundred adversarial requests
/// across two independent review lanes plus this unit's own seven
/// fixtures (bumpy, cylinder both signs, sphere both signs, near-reach
/// sphere, a 1000:1 thin patch, extreme weights at 0.05 and 8.0), over
/// tolerances from 1e-6 to 1e-15. None stalled.
///
/// **That is a property of the predicate's shape, not of the fixtures.**
/// The test is `hull_sup < prev_sup` with no epsilon, so ANY decrease
/// counts as improvement — including one in the last bit. Reaching
/// the refusal therefore needs a bound that fails to fall AT ALL
/// twice running, on a loop that re-interpolates at a strictly finer
/// schedule each round. Every round changes the fit, and a changed
/// fit on a finer schedule essentially always moves the bound
/// somewhere. So the refusal is close to unreachable BY CONSTRUCTION,
/// and the guard's practical work is done by its other two arms: the
/// both-directions fallback (which fires on the shipped cylinder
/// oracle every run) and the schedule-cannot-grow arm in
/// [`fit_offset`].
///
/// The verdict is recorded rather than resolved because the honest
/// options are both worse. Widening the test with a relative epsilon
/// ("improved by less than 1%") would make the refusal reachable, but
/// it would also refuse loops that are converging slowly and would
/// certify at a coarser schedule than the caller asked for — a
/// tolerance the fit CAN reach, refused. Deleting the arm would leave
/// the non-converging case spending budget silently, which is what
/// issue 1007 asked to end. So it stays: cheap, total, and pinned by
/// [`OffsetFitError::RefinementStalled`]'s own row rather than by a
/// fixture that does not exist.
fn stall_verdict(prev_sup: f64, hull_sup: f64, marked_both: bool) -> Refine {
    if !prev_sup.is_finite() || hull_sup < prev_sup {
        Refine::Directional
    } else if marked_both {
        Refine::Refuse
    } else {
        Refine::BothDirections
    }
}

/// One refinement round's next sample schedule, together with the
/// marking mode that produced it.
///
/// The mode travels WITH the schedule because it is what the stall
/// guard's admission set is stated in terms of. Held instead as a
/// local beside the marking, it records the mode that was *intended*;
/// held here, it records the mode that actually ran.
struct Marking {
    us: Vec<f64>,
    vs: Vec<f64>,
    mode: Refine,
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
///
/// [`Refine::Refuse`] never reaches here: it is the loop's exit, not a
/// marking. It is treated as the both-directions marking so that this
/// function is total, and the loop's own `verdict == Refuse` test is
/// what makes that arm unreachable.
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
        mode,
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
/// schedule, exact residual in metres. A non-finite sample answers
/// `f64::INFINITY`, which fails every classification.
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
            m = m.max((fit.eval(u, v) - target).norm());
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
    /// The BASE's weight channel `w` (a positive constant `1` patch
    /// when the base is non-rational). `M̃ = w³·m` is scaled by this
    /// one alone, because `m` is the base's own.
    w: PatchSpans,
    /// `w̃ = w·w_fit`, the weight `Ẽ` is homogeneous in. Equal to `w`
    /// exactly when the fit carries unit weights.
    wt: PatchSpans,
    /// `Ẽ = w̃·E` itself, kept so the residual's normal component can
    /// divide by a DIRECT lower bound on `‖E‖` (module docs, "the
    /// small-`|d|` denominator") instead of by `2|d|`.
    e: [PatchSpans; 3],
    breaks_u: Vec<f64>,
    breaks_v: Vec<f64>,
}

/// A row-major ring net of one spatial channel of a control net,
/// optionally weighted (the homogeneous `A^c = w·P^c`).
///
/// `patch_bound::comp_nets` is the same extraction in the nested
/// `Net` shape that module's windowed hulls index, and [`flat`] /
/// [`nest`] exist only to bridge the two. They are NOT unified,
/// deliberately and narrowly: the row-major slice is what
/// `PatchSpans::decompose` consumes and the nested form is what
/// `window_hull` indexes, so a single home would still hand one
/// caller the wrong shape. What is shared is the arithmetic —
/// `weight · coordinate`, in that order — and the two sites are
/// cross-referenced so a change to it is a change to both. Where they
/// differ is WHEN the recentring happens: this one folds the centre
/// into the net, because the net feeds polynomial products formed
/// once for the whole patch; `patch_bound` applies it at the hull
/// read, because it reads a cell-local centre off the cell's own
/// control window.
///
/// **Recentred**, on the shared `centre` all of the composite's nets
/// are built against: the entry is `w·(P − centre)`, so the net
/// describes `S − centre` rather than `S`. The subtraction is the
/// RING's, not `f64`'s, which is what makes this sound — an `f64`
/// difference would round to a control point the base does not have,
/// and the certificate would then be about a surface nobody supplied.
/// The ring's outward rounding of `P − centre` is one ulp of the
/// DIFFERENCE, i.e. of the patch's extent, where the unrecentred net
/// carried one ulp of the coordinate.
fn channel(n: &NurbsSurface<f64>, c: usize, form: NetForm, origin: &Origin) -> Vec<RingInterval> {
    n.control()
        .iter()
        .zip(n.weights().iter())
        .map(|(p, w)| {
            let x = RingInterval::point(match c {
                0 => p.x,
                1 => p.y,
                _ => p.z,
            }) - RingInterval::point(origin.0[c]);
            match form {
                NetForm::Homogeneous => RingInterval::point(*w) * x,
                NetForm::Spatial => x,
            }
        })
        .collect()
}

/// Which net a [`channel`] extraction produces.
///
/// A named form rather than a `bool`: the two differ by whether the
/// weight multiplies, which is the difference between `A` and `P`, and
/// a bare `true` at a call site does not say which was meant.
#[derive(Clone, Copy, PartialEq, Eq)]
enum NetForm {
    /// `P − c`. Correct only where every weight is `1`, so that the
    /// spatial net IS the homogeneous one.
    Spatial,
    /// `w·(P − c)`, the homogeneous net.
    Homogeneous,
}

impl NetForm {
    /// The form a surface's own weights call for.
    fn of(n: &NurbsSurface<f64>) -> Self {
        if is_rational(n) {
            Self::Homogeneous
        } else {
            Self::Spatial
        }
    }
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
        for (c, v) in [p.x, p.y, p.z].into_iter().enumerate() {
            lo[c] = lo[c].min(v);
            hi[c] = hi[c].max(v);
        }
    }
    let mut out = [0.0; 3];
    for c in 0..3 {
        // A non-finite or empty net recentres on the origin: the
        // composite's own poison handling is what reports it, and a
        // NaN centre would silently poison every cell instead.
        let m = (lo[c] + hi[c]) * 0.5;
        out[c] = if m.is_finite() { m } else { 0.0 };
    }
    Origin(out)
}

/// A row-major flat net from the `Net` (u-major nested) shape.
fn flat(net: &Net) -> Vec<RingInterval> {
    net.iter().flat_map(|row| row.iter().copied()).collect()
}

/// The `Net` (u-major nested) shape from a row-major flat net.
fn nest(grid: &[RingInterval], nu: usize, nv: usize) -> Net {
    (0..nu)
        .map(|i| grid[i * nv..(i + 1) * nv].to_vec())
        .collect()
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
        // A degree-1 direction has no derived KNOT VECTOR (degree 0
        // is not a clamped vector), and the composite needs one to
        // decompose the derivative nets. Degree elevation is exact in
        // ℝ and represents the same surface, so the composite is
        // built on the elevated form; the meters' floor, taken on the
        // original, is a fact about the same surface either way.
        let raised;
        let base = if base.knots_u().degree() < 2 || base.knots_v().degree() < 2 {
            let mut b = base.clone();
            if b.knots_u().degree() < 2 {
                b = b
                    .elevate_degree_u(2 - b.knots_u().degree())
                    .map_err(|_| OffsetFitError::PatchBound(PatchBoundError::DerivedKnots))?;
            }
            if b.knots_v().degree() < 2 {
                b = b
                    .elevate_degree_v(2 - b.knots_v().degree())
                    .map_err(|_| OffsetFitError::PatchBound(PatchBoundError::DerivedKnots))?;
            }
            raised = b;
            &raised
        } else {
            base
        };
        let (ku, kv) = (base.knots_u(), base.knots_v());
        let (nu, nv) = base.control_counts();
        let rational = is_rational(base);
        // One break list per direction, carrying every operand's
        // interior knots — the alignment substrate (patch docs).
        let mut extra_u: Vec<f64> = ku.interior_knots().map(|(t, _)| t).collect();
        extra_u.extend(fit.knots_u().interior_knots().map(|(t, _)| t));
        let mut extra_v: Vec<f64> = kv.interior_knots().map(|(t, _)| t).collect();
        extra_v.extend(fit.knots_v().interior_knots().map(|(t, _)| t));
        let dec = |kku: &KnotVector, kkv: &KnotVector, grid: &[RingInterval]| {
            PatchSpans::decompose(kku, kkv, grid, &extra_u, &extra_v)
        };
        // The FIT's homogeneous net `F̃ = w_fit·P_fit` and its weight
        // channel. On a unit-weight fit `w_fit ≡ 1`, the spatial net
        // IS the homogeneous one and `wf` is not formed: the identity
        // product would widen the ring for nothing.
        // The one recentring origin every net below is built against.
        let ctr = recentre_origin(base);
        let fit_form = NetForm::of(fit);
        let fc = |c: usize| channel(fit, c, fit_form, &ctr);
        let f: [PatchSpans; 3] = [
            dec(fit.knots_u(), fit.knots_v(), &fc(0)),
            dec(fit.knots_u(), fit.knots_v(), &fc(1)),
            dec(fit.knots_u(), fit.knots_v(), &fc(2)),
        ];
        let wf = (fit_form == NetForm::Homogeneous).then(|| {
            let g: Vec<RingInterval> = fit
                .weights()
                .iter()
                .map(|x| RingInterval::point(*x))
                .collect();
            dec(fit.knots_u(), fit.knots_v(), &g)
        });
        // The base's homogeneous nets and their first derivatives.
        let a_grid: Vec<Vec<RingInterval>> = (0..3)
            .map(|c| channel(base, c, NetForm::of(base), &ctr))
            .collect();
        let ku1 = derived_knots(ku)?;
        let kv1 = derived_knots(kv)?;
        let du = |g: &[RingInterval]| flat(&net_d_u(ku, &nest(g, nu, nv)));
        let dv = |g: &[RingInterval]| flat(&net_d_v(kv, &nest(g, nu, nv)));
        let a: [PatchSpans; 3] = [
            dec(ku, kv, &a_grid[0]),
            dec(ku, kv, &a_grid[1]),
            dec(ku, kv, &a_grid[2]),
        ];
        let a_u: [PatchSpans; 3] = [
            dec(&ku1, kv, &du(&a_grid[0])),
            dec(&ku1, kv, &du(&a_grid[1])),
            dec(&ku1, kv, &du(&a_grid[2])),
        ];
        let a_v: [PatchSpans; 3] = [
            dec(ku, &kv1, &dv(&a_grid[0])),
            dec(ku, &kv1, &dv(&a_grid[1])),
            dec(ku, &kv1, &dv(&a_grid[2])),
        ];
        let w_grid: Vec<RingInterval> = base
            .weights()
            .iter()
            .map(|x| RingInterval::point(*x))
            .collect();
        let w = if rational {
            dec(ku, kv, &w_grid)
        } else {
            a[0].constant(RingInterval::one())
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
            let w_u = dec(&ku1, kv, &du(&w_grid));
            let w_v = dec(ku, &kv1, &dv(&w_grid));
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
        let x = dot_spans(&e, &e).sub(&wt.mul(&wt).scale(RingInterval::point(d).sqr()));
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

    /// One cell's certified sup bound on `‖S_fit − (S + d·n)‖`
    /// (module docs). `f64::INFINITY` whenever a side condition is
    /// not proved — never a finite wrong answer.
    ///
    /// **The whole assembly stays in the ring**, with `.hi()` read
    /// exactly once at the end: every intermediate is a
    /// [`RingInterval`], so the outward rounding of each quotient,
    /// product and sum is the ring's. An `f64` fold of ring endpoints
    /// would round to nearest at each step and under-cover the real
    /// bound by ulps, which "certified" does not permit.
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn cell_bound(&self, su: usize, sv: usize, floor: f64, d: f64) -> f64 {
        let w = self.w.cell_hull(su, sv);
        let w_lo = w.lo();
        if !(w_lo > 0.0) || !w_lo.is_finite() {
            return f64::INFINITY;
        }
        // `w̃ = w·w_fit`, the weight `Ẽ`, `X` and the sign witness are
        // homogeneous in. Its positivity is the rational licence's on
        // both factors, and it is proved here rather than assumed.
        let wt = self.wt.cell_hull(su, sv);
        let wt_lo = wt.lo();
        if !(wt_lo > 0.0) || !wt_lo.is_finite() {
            return f64::INFINITY;
        }
        // The sign witness: `sign(E·n) = sign(D)` (the denominator
        // `w·‖M̃‖` is positive), and the normal-component bound below
        // needs `E·n` to carry `d`'s sign.
        let dh = self.dd.cell_hull(su, sv);
        if !(if d > 0.0 {
            dh.lo() > 0.0
        } else {
            dh.hi() < 0.0
        }) {
            return f64::INFINITY;
        }
        let abs_d = RingInterval::point(d.abs());
        // A DIRECT lower bound on `‖E‖`, from `E = Ẽ/w` and the
        // mignitude assembly on `Ẽ`'s own cell hulls — the same
        // inf-side shape meter 1 uses on the cross product. This is
        // what keeps the normal component's denominator honest when
        // `|d|` is small: `‖E‖ + |d|` is the true divisor of
        // `|‖E‖² − d²|`, and falling back on `2|d|` for it both
        // loses accuracy and, once `dist` reaches `|d|`, collapses
        // the cell to `+∞` for no geometric reason.
        let e_mig_sq = RingInterval::point(mig(self.e[0].cell_hull(su, sv))).sqr()
            + RingInterval::point(mig(self.e[1].cell_hull(su, sv))).sqr()
            + RingInterval::point(mig(self.e[2].cell_hull(su, sv))).sqr();
        let e_lo_iv = RingInterval::point(sqrt_down(e_mig_sq.lo())) / wt;
        // | ‖E‖ − |d| | = |X| / (w̃²·(‖E‖ + |d|)).
        let x_mag = RingInterval::from_bounds(0.0, self.x.cell_hull(su, sv).mag());
        let dist_iv = x_mag / (wt.sqr() * (e_lo_iv + abs_d));
        // τ = ‖Y‖ / (w̃·‖M̃‖) ≤ sup‖Y‖ / (floor·w̃·w³), using
        // ‖M̃‖ = w³·‖m‖ ≥ w³·floor.
        let y_sq = self.y[0].cell_hull(su, sv).mag().powi(2)
            + self.y[1].cell_hull(su, sv).mag().powi(2)
            + self.y[2].cell_hull(su, sv).mag().powi(2);
        let y_mag = RingInterval::from_bounds(0.0, sqrt_up(y_sq));
        let tau_iv = y_mag / (RingInterval::point(floor) * wt * w.powi(3));
        // `‖E‖` from below once more, for the `τ²/‖E‖` term: the
        // direct bound, or `|d| − dist` when that is larger.
        let e_floor = e_lo_iv.lo().max(d.abs() - dist_iv.hi());
        if !(e_floor > 0.0) {
            return f64::INFINITY;
        }
        let bound = dist_iv + tau_iv + tau_iv.sqr() / RingInterval::point(e_floor);
        let hi = bound.hi();
        if hi.is_finite() { hi } else { f64::INFINITY }
    }
}

#[cfg(test)]
mod tests {
    use super::{Refine, directional_mark, stall_verdict};

    /// The guard is silent while the bound is still `+∞`: an
    /// unbounded round is unproved, not unimproved, and a loop on its
    /// way from `+∞` to a finite bound is converging.
    #[test]
    fn an_infinite_predecessor_is_never_a_stall() {
        for marked_both in [false, true] {
            assert_eq!(
                stall_verdict(f64::INFINITY, f64::INFINITY, marked_both),
                Refine::Directional
            );
            assert_eq!(
                stall_verdict(f64::INFINITY, 1e-3, marked_both),
                Refine::Directional
            );
        }
    }

    /// A round that lowered the bound keeps refining directionally,
    /// whichever marking produced it.
    #[test]
    fn an_improving_round_stays_directional() {
        assert_eq!(stall_verdict(1e-3, 1e-4, false), Refine::Directional);
        assert_eq!(stall_verdict(1e-3, 1e-4, true), Refine::Directional);
    }

    /// A directional round that gained nothing does not refuse — it
    /// falls back to bisecting both directions, which is the answer
    /// when the speed ratio mispredicted where the error lives.
    /// Equality counts as no gain: a bound that held still is a bound
    /// that did not fall.
    #[test]
    fn a_directional_round_that_gains_nothing_falls_back_to_both() {
        assert_eq!(stall_verdict(1e-3, 1e-3, false), Refine::BothDirections);
        assert_eq!(stall_verdict(1e-3, 2e-3, false), Refine::BothDirections);
        assert_eq!(
            stall_verdict(1e-3, f64::INFINITY, false),
            Refine::BothDirections
        );
    }

    /// **The refusal's admission set.** Only a BOTH-directions round
    /// that still gains nothing refuses: the loop took the strongest
    /// step it has and got nothing, so the rounds it has left cannot
    /// reach the tolerance either.
    #[test]
    fn only_a_both_directions_round_that_gains_nothing_refuses() {
        assert_eq!(stall_verdict(1e-3, 1e-3, true), Refine::Refuse);
        assert_eq!(stall_verdict(1e-3, 2e-3, true), Refine::Refuse);
        assert_eq!(stall_verdict(1e-3, f64::INFINITY, true), Refine::Refuse);
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
}

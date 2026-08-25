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
//! cancel**. With the base written homogeneously as `S = A/w`:
//!
//! ```text
//! Ẽ = F·w − A                (F the fitted net; Ẽ = w·E)
//! M̃ = w·(A_u × A_v) − w_v·(A_u × A) − w_u·(A × A_v)      (M̃ = w³·m)
//! X = Ẽ·Ẽ − d²·w²           ( = w²·(‖E‖² − d²) )
//! Y = Ẽ × M̃                 ( = w⁴·(E × m) )
//! D = Ẽ · M̃                 ( = w⁴·(E · m) )
//! ```
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
//! **Where the regularity floor enters.** `τ` and `D` both divide by
//! `‖m‖`, and `X`'s reading divides by `w²`. The weight hull is
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
use geom::surfaces::NurbsSurface;
use geom_core::spline::compose::patch::PatchSpans;
use geom_core::spline::{KnotVector, SplineError};
use geom_core::{Band, Point3, ring_interval::RingInterval};

use crate::offset_meters::{MeterError, MeterResult, meter_patch};
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

/// The per-direction cap on sample parameters. A refinement round
/// bisects every sample interval a worst-carrying cell touches, so an
/// unreachable tolerance would otherwise double the grid every round
/// — the cap is the second stopping condition, and it produces the
/// same typed [`OffsetFitError::BudgetExhausted`] refusal as the
/// round budget: never an uncertified return, and never an unbounded
/// amount of work.
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

/// The two-limb certificate of a fitted offset surface. Every field
/// is a bound, in metres, that the corresponding limb proved — or the
/// structure it proved it over.
#[derive(Clone, Copy, Debug)]
pub struct OffsetCertificate {
    /// The signed offset distance the claim is about, in metres.
    pub distance: f64,
    /// How many `(u,v)` cells the span schedule has.
    pub cells: u32,
    /// The per-direction on-locus sample count inside each cell
    /// ([`OFFSET_CERT_SAMPLES`]).
    pub samples: u32,
    /// Limb 1: the largest on-locus residual over the schedule, in
    /// metres (the sampled max — it steers, it does not certify).
    pub on_locus_max: f64,
    /// Limb 2: the certified **sup-norm** bound over the whole chart
    /// rectangle, in metres. This is the number that certifies.
    pub hull_sup: f64,
    /// The regularity floor the normal enclosure rests on — a
    /// certified lower bound on `‖S_u × S_v‖` (m² per unit parameter
    /// area).
    pub normal_floor: f64,
    /// The collapse meter's certified fold radius on the folding
    /// side, in metres.
    pub curvature_reach: f64,
    /// How many refinement rounds the fit loop spent.
    pub rounds: u32,
}

/// **The offset fit door**: fit a NURBS approximation of `S + d·n`
/// over the base's own chart rectangle and certify it.
///
/// Refuses — never degrades — on a patch whose chart normal is not
/// certifiably non-degenerate, on an offset distance that reaches the
/// patch's curvature reach, and on budget exhaustion.
///
/// Geometry only: no `Surface` variant, no storage, no topology. The
/// base and `d` travel as arguments; the intensional
/// `Offset { base, d }` description is the integration unit's.
///
/// # Errors
///
/// [`OffsetFitError`] — the two door meters and their escalations,
/// the patch-bound refusals, the interpolation stack's refusals,
/// non-finite samples, and [`OffsetFitError::BudgetExhausted`]
/// carrying the achieved bound.
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
        let next_u = bisect(&us, &mark(&us, report.failing.iter().map(|c| c.0)));
        let next_v = bisect(&vs, &mark(&vs, report.failing.iter().map(|c| c.1)));
        if (next_u.len() == us.len() && next_v.len() == vs.len())
            || next_u.len() > OFFSET_FIT_SAMPLE_CAP
            || next_v.len() > OFFSET_FIT_SAMPLE_CAP
        {
            break;
        }
        us = next_u;
        vs = next_v;
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

/// What one measurement pass proved, plus which sample intervals the
/// next refinement round must bisect.
struct Report {
    cells: u32,
    on_locus_max: f64,
    hull_sup: f64,
    /// The `(u, v)` rectangles of the cells that carry the sup — what
    /// the next refinement round attacks.
    failing: Vec<((f64, f64), (f64, f64))>,
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
    let failing: Vec<((f64, f64), (f64, f64))> = bounds
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
    /// `X = Ẽ·Ẽ − d²·w²`.
    x: PatchSpans,
    /// `Y = Ẽ × M̃`.
    y: [PatchSpans; 3],
    /// `D = Ẽ · M̃`, the sign witness of `E·n`.
    dd: PatchSpans,
    /// The weight channel `w` (a positive constant `1` patch when the
    /// base is non-rational).
    w: PatchSpans,
    breaks_u: Vec<f64>,
    breaks_v: Vec<f64>,
}

/// A row-major ring net of one spatial channel of a control net,
/// optionally weighted (the homogeneous `A^c = w·P^c`).
fn channel(n: &NurbsSurface<f64>, c: usize, weighted: bool) -> Vec<RingInterval> {
    n.control()
        .iter()
        .zip(n.weights().iter())
        .map(|(p, w)| {
            let x = RingInterval::point(match c {
                0 => p.x,
                1 => p.y,
                _ => p.z,
            });
            if weighted {
                RingInterval::point(*w) * x
            } else {
                x
            }
        })
        .collect()
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
        // The fitted net (unit weights, so its spatial net IS its
        // homogeneous net).
        let f: [PatchSpans; 3] = [
            dec(fit.knots_u(), fit.knots_v(), &channel(fit, 0, false)),
            dec(fit.knots_u(), fit.knots_v(), &channel(fit, 1, false)),
            dec(fit.knots_u(), fit.knots_v(), &channel(fit, 2, false)),
        ];
        // The base's homogeneous nets and their first derivatives.
        let a_grid: Vec<Vec<RingInterval>> = (0..3).map(|c| channel(base, c, rational)).collect();
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
        // Ẽ = F·w − A.
        let e: [PatchSpans; 3] = [
            f[0].mul(&w).sub(&a[0]),
            f[1].mul(&w).sub(&a[1]),
            f[2].mul(&w).sub(&a[2]),
        ];
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
        let x = dot_spans(&e, &e).sub(&w.mul(&w).scale(RingInterval::point(d).sqr()));
        let y = cross_spans(&e, &m_tilde);
        let dd = dot_spans(&e, &m_tilde);
        let (bu, bv) = x.breaks();
        let (breaks_u, breaks_v) = (bu.to_vec(), bv.to_vec());
        Ok(Self {
            x,
            y,
            dd,
            w,
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
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    fn cell_bound(&self, su: usize, sv: usize, floor: f64, d: f64) -> f64 {
        let w_lo = self.w.cell_hull(su, sv).lo();
        if !(w_lo > 0.0) || !w_lo.is_finite() {
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
        // | ‖E‖ − |d| | = |‖E‖² − d²| / (‖E‖ + |d|) ≤ sup|X| / (w_lo²·|d|).
        let dist = (self.x.cell_hull(su, sv).mag() / w_lo.powi(2) / d.abs()).next_up();
        // τ = ‖Y‖ / (w·‖M̃‖) ≤ sup‖Y‖ / (floor·w_lo⁴), using
        // ‖M̃‖ = w³·‖m‖ ≥ w_lo³·floor.
        let y2 = self.y[0].cell_hull(su, sv).mag().powi(2)
            + self.y[1].cell_hull(su, sv).mag().powi(2)
            + self.y[2].cell_hull(su, sv).mag().powi(2);
        let denom = floor * w_lo.powi(4);
        if !(denom > 0.0) || !denom.is_finite() {
            return f64::INFINITY;
        }
        let tau = (y2.sqrt().next_up() / denom).next_up();
        // ‖E‖ ≥ |d| − dist; below that the residual already exceeds
        // any tolerance worth certifying.
        let e_lo = d.abs() - dist;
        if !(e_lo > 0.0) {
            return f64::INFINITY;
        }
        let bound = dist + tau + (tau.powi(2) / e_lo).next_up();
        if bound.is_finite() {
            bound
        } else {
            f64::INFINITY
        }
    }
}

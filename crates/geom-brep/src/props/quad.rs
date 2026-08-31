//! **Certified quadrature for curved-cut faces** (M5 PR 11, C12.7) —
//! the kernel's first quadrature: divergence-theorem contributions for
//! analytic charts whose trim loops are *curved* (conic cuts), computed
//! as certified enclosures with an interval/hull-bounded remainder.
//! Certified bounds or typed refusal; never a silent Gaussian trust
//! (D4).
//!
//! # Formulation (cylinder chart)
//!
//! With chart `S(u, v) = o + û·r·cos u + v̂·r·sin u + â·v` the chart
//! normal is radial and `dA = r du dv`, so for a face whose trim region
//! is Ω (traversed by the loop with interior-left, outward normals):
//!
//! ```text
//! A_s   = ∮ u dv           (signed UV area; A_s = s_f·|Ω|)
//! area  = r·|A_s|
//! flux  = ∮ p·n dA = r²·A_s + o·A⃗    (A⃗ the exact vector area)
//! ```
//!
//! — the loop's own traversal sign carries `s_f`, so no separate
//! orientation classification is needed. The boundary is the face's
//! **pcurves** (PR 6 caches; C4's first hot consumer): per edge the
//! contribution is `σ·∫ u(t)·v'(t) dt` over the stored carrier
//! interval.
//!
//! # The integrand substrate is the C9 ring
//!
//! Every enclosure here is [`RingInterval`] arithmetic — no
//! transcendental is ever *evaluated* on the certified path. The two
//! integrand families:
//!
//! - **Harmonic** (every minted cylinder-chart pcurve):
//!   `u, v(t) = c₀ + c_a·cos t + c_b·sin t + c_l·t`. Trigonometric
//!   values enter only as **enclosures propagated from the edge
//!   endpoints**: the caller supplies `(cos t₀, sin t₀)` brackets
//!   recovered *algebraically* from the carrier frame and the endpoint
//!   vertex point (within the run's ε of the carrier — D4 ¶2), and the
//!   engine advances them by exact rotation with polynomial
//!   alternating-series bounds on the step's `(cos, sin)` (valid for
//!   |step| ≤ 1.5; every step here is ≤ 1). Sound and transcendental-
//!   free.
//! - **B-spline** ([`bspline_green_integral`], the general machinery):
//!   channel hulls and derivative hulls from the control-coefficient
//!   convexity facts (`geom_core::spline::hull`). This family has **no
//!   at-rest consumer**: the NURBS-patch flux lanes are separate
//!   engines in this file — `patch_flux_exact` with its composite
//!   rounds for a polynomial patch, and `rational_patch_face`'s
//!   quotient composite (with the `w`-uniform-in-v exact arm) for a
//!   patch with non-unit weights — not this Green-form boundary
//!   integral. The consumer that would use it is
//!   the fitted-boundary Green lane, blocked on a construction that
//!   mints a fitted pcurve on an analytic chart at rest — the blocker
//!   `topo::props`' `QuadratureUnsupported` refusal names. That is a
//!   CONDITION, not a date: the lane is a deliberate frontier whose
//!   consumer lands with whichever banked construction (the marched
//!   join windows, the edge×NURBS-face boolean layer) first produces
//!   one, and its callers until then are tests.
//!
//! # The rule and its remainder
//!
//! Per piece `[p, q]`, `h = q − p`, `m` the midpoint (Taylor with
//! integral remainder; `∫(t−m)² dt = h³/12`):
//!
//! ```text
//! ∫_p^q f  ∈  h·f(m) + F₂·h³/24,   F₂ ⊇ f'' over [p, q]
//! ```
//!
//! `f(m)` is a *thin* enclosure (rotation to the midpoint), `F₂` a hull
//! enclosure — the remainder is the enclosure width itself, so the
//! bracket is sound at every refinement level and only *tightens* as
//! pieces shrink. Refinement doubles the piece count; **acceptance and
//! refinement go through named predicates** (`props_quad_*`, K-funnel
//! telemetry from birth): the convergence margin is metered as a
//! LENGTH, `width(flux)/(3·area)` — the mean boundary displacement the
//! enclosure width corresponds to, exactly check 7's V/A metering —
//! against `QUAD_TARGET_LEN_FACTOR·ε`. The factor is documented
//! honestly: the width floor is set by the ε-scale endpoint/boundary
//! slacks times the boundary length, so an ε-tight target would refuse
//! every real face; three decades above the floor and far below
//! display relevance is the useful band. Budget exhaustion is a typed
//! refusal ([`PropsError::QuadratureBudget`]), never a silent wide
//! answer.
//!
//! # Honesty pads (both directions accounted)
//!
//! - **Map residual**: each pcurve tracks its carrier within its
//!   certificate envelope `e` (metres through the map). The cylinder
//!   metric is `ds² = r²du² + dv²`, so the true trim boundary lies
//!   within metric distance `e` of the pcurve and the UV-area defect is
//!   ≤ Σ (metric edge length)·e / r — added to `A_s` symmetrically.
//! - **Endpoint parameter brackets**: a non-thin `t₀/t₁` bracket
//!   contributes `mag(f)·width` at each end.
//!
//! The closed-form `o·A⃗` term rides at the caller's scalar exactly as
//! the closed-form lanes do (f64 rounding of closed forms is the
//! documented slack there, unchanged).

use geom_core::ring_interval::RingInterval;
use geom_core::spline::hull::{derivative_coeffs, span_hull};
use geom_core::spline::{KnotVector, Span};
use geom_core::{Band, Decide, Margin, Sign};

use super::PropsError;

/// The initial piece count of the composite rule (round 0).
const QUAD_INIT_PIECES: usize = 16;
/// Refinement rounds before the typed budget refusal (pieces double
/// per round: ≤ `16·2¹²` = 65536 pieces).
const QUAD_MAX_ROUNDS: usize = 12;
/// Convergence target as a multiple of ε, metered as the mean boundary
/// displacement `width(flux)/(3·area)` (module docs: why not 1·ε).
const QUAD_TARGET_LEN_FACTOR: f64 = 1024.0;

/// Certified enclosures of one curved-cut face's contributions.
#[derive(Clone, Copy, Debug)]
pub struct FaceCutBounds {
    /// Enclosure of `∮ p·n dA` (n outward; volume = Σ flux/3).
    pub flux: RingInterval,
    /// Enclosure of the unsigned face area.
    pub area: RingInterval,
}

/// One harmonic chart channel `c₀ + c_a·cos t + c_b·sin t + c_l·t`
/// with ring-bracketed coefficients.
#[derive(Clone, Copy, Debug)]
pub struct HarmChan {
    /// The constant coefficient.
    pub c0: RingInterval,
    /// The `cos t` coefficient.
    pub ca: RingInterval,
    /// The `sin t` coefficient.
    pub cb: RingInterval,
    /// The linear-in-`t` coefficient.
    pub cl: RingInterval,
}

/// One trim-loop edge of the quadrature lane, fully bracketed by the
/// caller (key-free, like [`super::LoopEdge`]).
#[derive(Clone, Copy, Debug)]
pub struct TrimEdgeQ {
    /// The azimuth channel `u(t)`.
    pub u: HarmChan,
    /// The height channel `v(t)`.
    pub v: HarmChan,
    /// Bracket of the certified interval start.
    pub t0: RingInterval,
    /// Bracket of the certified interval end.
    pub t1: RingInterval,
    /// Loop traversal direction (`he_plus`-forward or reversed).
    pub forward: bool,
    /// Enclosure of `(cos t₀, sin t₀)` (module docs: algebraic, from
    /// the carrier frame and the endpoint vertex point, ε-padded).
    pub trig0: (RingInterval, RingInterval),
    /// The pcurve certificate envelope (metres through the map) — the
    /// map-residual honesty pad's per-edge input.
    pub env: RingInterval,
}

// ---------------------------------------------------------------------
// Transcendental-free trig enclosures (module docs)
// ---------------------------------------------------------------------

/// Shorthand: a point bracket.
fn pt(x: f64) -> RingInterval {
    RingInterval::point(x)
}

/// Enclosure of `cos s` for an EXACT step `s`, |s| ≤ 1.5, by the
/// degree-8 alternating-series pair
/// `1 − s²/2 + s⁴/24 − s⁶/720 ≤ cos s ≤ … + s⁸/40320` (truncations of
/// an alternating series with decreasing terms — decreasing needs
/// s² ≤ 56, ample here; both ends formed in the ring so their own
/// rounding is outward). Poison outside the domain.
fn cos_step(s: f64) -> RingInterval {
    if s.is_nan() || s.abs() > 1.5 {
        return RingInterval::poison();
    }
    let s2 = pt(s).sqr();
    let s4 = s2.sqr();
    let lo = pt(1.0) - s2 / pt(2.0) + s4 / pt(24.0) - s4 * s2 / pt(720.0);
    let hi = lo + s4.sqr() / pt(40_320.0);
    RingInterval::from_bounds(lo.lo(), hi.hi()).clamped_to(-1.0, 1.0)
}

/// Enclosure of `sin s` for an EXACT step `s`, |s| ≤ 1.5: the degree-9
/// alternating-series pair `s − s³/6 + s⁵/120 − s⁷/5040 ≤ sin s ≤
/// … + s⁹/362880` for s ≥ 0 (decreasing terms need s² ≤ 72), mirrored
/// by oddness for s < 0.
fn sin_step(s: f64) -> RingInterval {
    if s.is_nan() || s.abs() > 1.5 {
        return RingInterval::poison();
    }
    let a = s.abs();
    let a1 = pt(a);
    let a2 = a1.sqr();
    let a3 = a1 * a2;
    let a5 = a3 * a2;
    let a7 = a5 * a2;
    let lo = a1 - a3 / pt(6.0) + a5 / pt(120.0) - a7 / pt(5040.0);
    let hi = lo + a7 * a2 / pt(362_880.0);
    let unit = RingInterval::from_bounds(lo.lo(), hi.hi()).clamped_to(-1.0, 1.0);
    if s >= 0.0 {
        unit
    } else {
        RingInterval::from_bounds(-unit.hi(), -unit.lo())
    }
}

/// One rotation: `(c, s)` advanced by a step whose `(cos, sin)`
/// enclosures are given. Intersecting with [−1, 1] after every
/// compose keeps the propagated brackets from drifting past the
/// circle (sound: the true values lie in both).
fn rotate(
    c: RingInterval,
    s: RingInterval,
    ch: RingInterval,
    sh: RingInterval,
) -> (RingInterval, RingInterval) {
    let clamp = |x: RingInterval| x.clamped_to(-1.0, 1.0);
    (clamp(c * ch - s * sh), clamp(s * ch + c * sh))
}

/// `(cos, sin)` at exact offset `off` from a base enclosure:
/// argument-halved series + double-angle squaring. `off` is halved
/// (exactly — powers of two) until ≤ 0.05, the series pair bounds that
/// seed (truncation below the ring's own ulp there), and the rotation
/// is rebuilt by `k` double-angle steps `(c, s) → (c² − s², 2cs)` —
/// the width amplification is 2^k on a ~1e-16 seed (≈ 1e-14 for a
/// full-period offset), NOT the exponential-in-|off| compounding a
/// step-march would suffer. The composed rotation is applied to the
/// base exactly once.
fn trig_at(base: (RingInterval, RingInterval), off: f64) -> (RingInterval, RingInterval) {
    if off == 0.0 {
        return base;
    }
    if !off.is_finite() {
        return (RingInterval::poison(), RingInterval::poison());
    }
    let mut seed = off;
    let mut k = 0u32;
    while seed.abs() > 0.05 {
        seed *= 0.5;
        k += 1;
    }
    let (mut c, mut s) = (cos_step(seed), sin_step(seed));
    let clamp = |x: RingInterval| x.clamped_to(-1.0, 1.0);
    for _ in 0..k {
        // Double angle; `sqr` keeps the squares tight (the
        // interval-square rule).
        let c2 = clamp(c.sqr() - s.sqr());
        let s2 = clamp(pt(2.0) * c * s);
        c = c2;
        s = s2;
    }
    rotate(base.0, base.1, c, s)
}

/// `(cos, sin)` RANGE enclosures over offsets `[off, off + d]` from a
/// base enclosure, `d ≥ 0`: rotate to `off`, then widen by the
/// interval rotation `cos ∈ [1 − d²/2, 1]`, `sin ∈ [0, d]` (sound for
/// d ≤ π; larger spans fall back to the whole circle).
fn trig_over(base: (RingInterval, RingInterval), off: f64, d: f64) -> (RingInterval, RingInterval) {
    if d.is_nan() || d < 0.0 || !off.is_finite() {
        return (RingInterval::poison(), RingInterval::poison());
    }
    if d > 3.0 {
        let full = RingInterval::from_bounds(-1.0, 1.0);
        return (full, full);
    }
    let at = trig_at(base, off);
    // The `.max`/`.min` below are NOT the poison-swallowing shape
    // `RingInterval::clamped_to` exists for: `d` is a finite nonnegative
    // f64 by the guard above, so this ring arithmetic cannot produce
    // poison and there is no NaN for `f64::max` to absorb. `base` is the
    // operand that can be poison, and it reaches only `trig_at`/`rotate`,
    // which clamp through `clamped_to`.
    let ch = {
        let lo = (pt(1.0) - pt(d).sqr() / pt(2.0)).lo().max(-1.0);
        RingInterval::from_bounds(lo, 1.0)
    };
    let sh = RingInterval::from_bounds(0.0, d.min(1.0));
    rotate(at.0, at.1, ch, sh)
}

// ---------------------------------------------------------------------
// Harmonic channels
// ---------------------------------------------------------------------

impl HarmChan {
    /// The channel's derivative as a channel:
    /// `d/dt (c₀ + c_a c + c_b s + c_l t) = c_l + c_b c − c_a s`.
    fn deriv(self) -> Self {
        Self {
            c0: self.cl,
            ca: self.cb,
            cb: -self.ca,
            cl: RingInterval::zero(),
        }
    }

    /// Enclosure of the channel over trig enclosures `(c, s)` and a
    /// `t` bracket (fixed association order, D9).
    fn eval(self, c: RingInterval, s: RingInterval, t: RingInterval) -> RingInterval {
        self.c0 + self.ca * c + self.cb * s + self.cl * t
    }
}

// ---------------------------------------------------------------------
// The composite rule (module docs: h·f(m) + F₂·h³/24 per piece)
// ---------------------------------------------------------------------

/// `σ·∫_{a}^{b} u(t)·v'(t) dt` for one harmonic edge at `pieces`
/// resolution, plus the edge's two honesty pads (endpoint-bracket and
/// map-residual — the latter needs `radius` for the metric length).
fn harmonic_edge_integral(e: &TrimEdgeQ, pieces: usize, radius: RingInterval) -> RingInterval {
    let (a, b) = (mid(e.t0), mid(e.t1));
    let span = b - a;
    if !(span.is_finite() && span >= 0.0) || pieces == 0 {
        return RingInterval::poison();
    }
    let du = e.u.deriv();
    let dv = e.v.deriv();
    // f = u·v'; f'' = u''·v' + 2·u'·v'' + u·v'''.
    let ddu = du.deriv();
    let ddv = dv.deriv();
    let dddv = ddv.deriv();
    let mut total = RingInterval::zero();
    #[allow(clippy::cast_precision_loss)]
    let h = span / pieces as f64;
    let h3_24 = pt(h) * pt(h).sqr() / pt(24.0);
    // Per-piece midpoint rotation via `trig_at` (double-angle
    // composition): the base bracket passes through exactly ONE
    // rotation per piece, so its width never compounds across pieces
    // (a cursor march would amplify it by ~e^span).
    // Per-piece spread: midpoint ± h/2 (the interval rotation of
    // `trig_over`, midpoint-anchored).
    // `h` is finite by the `span`/`pieces` guard above, so — as in
    // `trig_over` — the `.max`/`.min` here operate on poison-free ring
    // arithmetic and are not the `clamped_to` hazard.
    let h2 = h * 0.5;
    let spread_c = if h2 <= 1.5 {
        RingInterval::from_bounds((pt(1.0) - pt(h2).sqr() / pt(2.0)).lo().max(-1.0), 1.0)
    } else {
        RingInterval::from_bounds(-1.0, 1.0)
    };
    let spread_s = if h2 <= 1.5 {
        RingInterval::from_bounds(-h2.min(1.0), h2.min(1.0))
    } else {
        RingInterval::from_bounds(-1.0, 1.0)
    };
    for i in 0..pieces {
        #[allow(clippy::cast_precision_loss)]
        let p = a + span * (i as f64 / pieces as f64);
        let m = p + h * 0.5;
        // Thin midpoint value.
        let (cm, sm) = trig_at(e.trig0, m - a);
        let fm = e.u.eval(cm, sm, pt(m)) * dv.eval(cm, sm, pt(m));
        // f'' hull over the piece (midpoint spread by ± h/2).
        let (cr, sr) = rotate(cm, sm, spread_c, spread_s);
        let tr = RingInterval::from_bounds(p, p + h);
        let f2 = ddu.eval(cr, sr, tr) * dv.eval(cr, sr, tr)
            + pt(2.0) * du.eval(cr, sr, tr) * ddv.eval(cr, sr, tr)
            + e.u.eval(cr, sr, tr) * dddv.eval(cr, sr, tr);
        total = total + pt(h) * fm + f2 * h3_24;
    }
    // Endpoint-bracket pad: |f| near the ends times the t-bracket
    // widths (module docs).
    let (c_all, s_all) = trig_over(e.trig0, 0.0, span);
    let t_all = RingInterval::from_bounds(a, b);
    let f_mag = (e.u.eval(c_all, s_all, t_all) * dv.eval(c_all, s_all, t_all)).mag();
    let wt = e.t0.width() + e.t1.width();
    let pad = f_mag * wt;
    let total = total + RingInterval::from_bounds(-pad, pad);
    // Map-residual pad: metric length ≤ (r·mag(u') + mag(v'))·span,
    // UV-area defect ≤ length·env/r (module docs).
    let uv_pad = (pt(edge_metric_length(e, radius)) * e.env / radius).mag();
    let total = total + RingInterval::from_bounds(-uv_pad, uv_pad);
    if e.forward { total } else { -total }
}

/// A certified upper bound on the edge's METRIC length (metres):
/// `∫ √(r²u'² + v'²) dt ≤ (r·sup|u'| + sup|v'|)·span`.
fn edge_metric_length(e: &TrimEdgeQ, radius: RingInterval) -> f64 {
    let (a, b) = (mid(e.t0), mid(e.t1));
    let span = b - a;
    let (c_all, s_all) = trig_over(e.trig0, 0.0, span);
    let t_all = RingInterval::from_bounds(a, b);
    let du = e.u.deriv();
    let dv = e.v.deriv();
    ((radius * du.eval(c_all, s_all, t_all).abs_enclosure()
        + dv.eval(c_all, s_all, t_all).abs_enclosure())
        * pt(span))
    .mag()
}

/// Midpoint of a bracket (structure selection for integration limits;
/// the bracket's width is repaid by the endpoint pad).
fn mid(x: RingInterval) -> f64 {
    (x.lo() + x.hi()) * 0.5
}

/// Interval absolute value: `|x|` as an enclosure.
trait AbsEnclosure {
    fn abs_enclosure(self) -> RingInterval;
}

impl AbsEnclosure for RingInterval {
    fn abs_enclosure(self) -> RingInterval {
        if self.is_poison() {
            return self;
        }
        if self.lo() >= 0.0 {
            self
        } else if self.hi() <= 0.0 {
            -self
        } else {
            RingInterval::from_bounds(0.0, self.mag())
        }
    }
}

// ---------------------------------------------------------------------
// Face assembly: refinement loop + K funnel
// ---------------------------------------------------------------------

/// Funnel wrapper (the `props_*` idiom of [`super::curved`]): margins
/// are certification-substrate `f64` lengths lifted to the caller's
/// scalar so every lane (f64 / Probe / Interval) records identically.
fn classify_len<T: Decide>(
    name: &'static str,
    margin: Margin<f64>,
    band: Band,
) -> Result<Sign, PropsError> {
    geom_core::k_stats::decide(name, margin.lift::<T>(), band)
        .map_err(|cause| PropsError::Escalated { cause })
}

/// The convergence meter: the flux enclosure's width expressed as the
/// **mean boundary displacement** it corresponds to,
/// `width(flux)/(3·area)` (module docs), with the area enclosure's
/// midpoint as the lever arm. Every convergence decision and every
/// [`PropsError::QuadratureBudget`] payload in this file comes from
/// here.
///
/// INVARIANT: the returned length is FINITE and non-negative — so a
/// budget refusal carries a number the caller can act on *by
/// construction*, never the `+inf` a cancelled lever used to produce
/// and never a `NaN`.
///
/// **Why the lever needs a guard.** `area` is accumulated as
/// `widen(g_mid, pad)` per cell — a SYMMETRIC pad — so once the
/// Lipschitz pad dwarfs the true area (measured: pad ≈ 1.7e20 against
/// a true area of 1 m² on the extreme-weight bilinear square),
/// `g_mid ± pad` rounds to exactly `∓pad` and `area.lo() + area.hi()`
/// cancels to EXACTLY 0.0. The unguarded division then yielded `+inf`,
/// and the refusal reported the enclosure as "stalled" at that width
/// — while the flux enclosure it was measuring went on narrowing round
/// by round, as an O(h²) rule must. The payload lied, and the
/// convergence predicate was decided by a division artifact instead of
/// by the enclosure. When the flux width is also
/// zero — [`cylinder_cut_face`]'s `area = [0, 0]` for a zero-UV-area
/// trim loop — the same division is `0/0 = NaN`, which the funnel
/// turns into `Indeterminate{margin: Invalid}`: a MIS-TYPED refusal,
/// since [`PropsError::Escalated`] means an *in-band* margin, not a
/// degenerate face.
///
/// **Why [`PropsError::DegenerateFace`] is the honest variant** for a
/// non-positive lever rather than a budget stall: `lever ≤ 0` forces
/// `area.lo() ≤ 0`, i.e. the enclosure does not certify that the face
/// has positive extent — exactly the verdict the
/// `props_quad_face_extent` gate reaches from the same `area.lo()` one
/// step later, reading the same number: definitely so when the
/// cancellation is large (an `area.lo()` of −1e20 is nowhere near any
/// band), and where `area` is exactly `[0, 0]` — the zero-UV-area trim
/// loop — the extent is not a tolerance question at all, it is zero.
/// The meter invents no refusal of its own; it reaches the extent
/// verdict one step earlier, on the same evidence.
///
/// **Why the midpoint stays the lever** — not `area.hi()`, not a
/// certified-positive lower lever like `area.lo()`: every face that
/// certifies today must keep certifying with BIT-IDENTICAL numbers
/// (D9), and both alternatives move the meter. `area.hi()` shrinks the
/// metered displacement, so faces the reviewed inventory refuses would
/// start certifying; `area.lo()` inflates it, de-certifying real
/// bodies. Which point of the area enclosure the meter divides by is a
/// metering decision (module docs) and would have to be re-measured
/// and re-ratified as one — the defect fixed here is the *unguarded
/// division*, and only that.
fn mean_boundary_displacement(flux: RingInterval, area: RingInterval) -> Result<f64, PropsError> {
    let width = flux.width();
    // Bit-for-bit the pre-guard expression: `(lo + hi)·0.5`, times 3.
    let denom = 3.0 * ((area.lo() + area.hi()) * 0.5);
    // A poisoned enclosure reads as NaN at both endpoints, so it lands
    // here rather than in the degeneracy branch: it is not a statement
    // about the face's extent at all.
    if !width.is_finite() || !denom.is_finite() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a quadrature enclosure with a non-finite width or area (a poisoned \
                   bracket) — the convergence meter has no honest length to report, \
                   and a refusal carrying a non-finite width would misstate the \
                   enclosure",
        });
    }
    if denom <= 0.0 {
        return Err(PropsError::DegenerateFace);
    }
    let len = width / denom;
    if !len.is_finite() {
        // Finite/finite still overflows on a subnormal lever against a
        // wide flux: the area is zero AT THE FLUX'S SCALE — the same
        // degeneracy the branch above names, reached by rounding.
        return Err(PropsError::DegenerateFace);
    }
    Ok(len)
}

/// The flux and area enclosures of a **cylinder** face with a curved
/// trim loop (module docs: the Green form, the composite rule, the
/// refinement funnel, the honesty pads).
///
/// `o_dot_va` is the caller's closed-form `o·A⃗` term bracketed;
/// `eps` is the run's ε (the convergence target's scale).
///
/// # Errors
///
/// [`PropsError`] — an escalated funnel decision, a degenerate face,
/// or the typed [`PropsError::QuadratureBudget`] refusal when the
/// enclosure will not tighten to target within the round budget.
pub fn cylinder_cut_face<T: Decide>(
    radius: RingInterval,
    o_dot_va: RingInterval,
    edges: &[TrimEdgeQ],
    eps: f64,
    band: Band,
) -> Result<FaceCutBounds, PropsError> {
    let target_len = QUAD_TARGET_LEN_FACTOR * eps;
    let mut pieces = QUAD_INIT_PIECES;
    // INVARIANT: every round assigns this from
    // `mean_boundary_displacement`, which returns only finite
    // lengths — so the budget refusal below carries a finite width
    // by construction (the `eps_posture` contract the suites pin).
    let mut last_width_len = f64::NAN;
    for round in 0..=QUAD_MAX_ROUNDS {
        // Signed UV area at this resolution: A_s = ∮ u dv.
        let mut a_s = RingInterval::zero();
        for e in edges {
            a_s = a_s + harmonic_edge_integral(e, pieces, radius);
        }
        let area = (radius * a_s).abs_enclosure();
        let flux = radius.sqr() * a_s + o_dot_va;
        // Convergence: mean-boundary-displacement metering (module
        // docs); the area midpoint is the lever arm, guarded — a
        // zero-UV-area trim loop makes `area` exactly `[0, 0]` here,
        // which is a degenerate face and not a tolerance question.
        let width_len = mean_boundary_displacement(flux, area)?;
        last_width_len = width_len;
        if classify_len::<T>(
            "props_quad_converged",
            Margin::of(target_len - width_len),
            band,
        )? == Sign::Positive
        {
            // Face-extent gate on the CONVERGED enclosure: the area
            // must be definitely positive, metered as the face's mean
            // width area/perimeter — a length (the closed-form lanes'
            // `props_face_extent` lever, quadrature-shaped).
            let perim: f64 = edges.iter().map(|e| edge_metric_length(e, radius)).sum();
            match classify_len::<T>(
                "props_quad_face_extent",
                Margin::over_lever(area.lo(), perim),
                band,
            )? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(PropsError::DegenerateFace),
            }
            return Ok(FaceCutBounds { flux, area });
        }
        if round < QUAD_MAX_ROUNDS {
            pieces *= 2;
        }
    }
    Err(PropsError::QuadratureBudget {
        width_len: last_width_len,
        target_len,
    })
}

// ---------------------------------------------------------------------
// The general hull-bounded lane: B-spline pcurve channels
// ---------------------------------------------------------------------

/// Interval de Boor: a nonrational scalar B-spline evaluated at an
/// exact parameter with ring-bracketed coefficients — the thin `f(m)`
/// the composite rule needs on spline channels. Pure ring arithmetic
/// (knots are `f64` structure; every knot difference is formed in the
/// ring so its rounding is outward, matching `deriv_coeff`).
fn bspline_eval_ring(kv: &KnotVector, coeffs: &[RingInterval], t: f64) -> RingInterval {
    if coeffs.len() != kv.control_count() || !t.is_finite() {
        return RingInterval::poison();
    }
    let p = kv.degree();
    let u = kv.knots();
    // The window is validated once, by `span_at`, and carries its own
    // first control point: the `span − p` this loop used to redo twice
    // (once per pass) has no use site left and cannot underflow. Its
    // in-range-ness is the `Span` invariant, so indexing `coeffs`
    // needs only the length check above.
    let first = kv.span_at(t).first_control();
    let mut d: Vec<RingInterval> = (0..=p).map(|j| coeffs[first + j]).collect();
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = first + j;
            let denom = pt(u[i + p + 1 - r]) - pt(u[i]);
            let alpha = (pt(t) - pt(u[i])) / denom;
            d[j] = (pt(1.0) - alpha) * d[j - 1] + alpha * d[j];
        }
    }
    d[p]
}

/// Hull of a scalar B-spline over `[lo, hi]`: the hull of the active
/// spans' coefficient hulls (conservative to span granularity).
fn bspline_range_hull(kv: &KnotVector, coeffs: &[RingInterval], lo: f64, hi: f64) -> RingInterval {
    let (s0, s1) = kv.span_range(lo, hi);
    let mut acc = RingInterval::poison();
    let mut seeded = false;
    for index in s0.index()..=s1.index() {
        // Emptiness check and span validation are one step.
        let Some(span) = kv.span(index) else { continue };
        let h = span_hull(kv, coeffs, span);
        acc = if seeded {
            RingInterval::hull(acc, h)
        } else {
            h
        };
        seeded = true;
    }
    acc
}

/// The derivative ladder of one nonrational channel, up to third
/// order. Level `k` holds the k-th derivative's coefficient brackets
/// and, where the degree allows (`p − k ≥ 1`), its materialised knot
/// vector. Missing HIGHER levels are **in-span polynomial zeros** — a
/// degree-p span polynomial has vanishing derivatives beyond order p —
/// which is sound only on knot-free pieces, and each composite rule
/// over this ladder is responsible for ensuring that. The 1-D Green
/// lane below does it by giving a piece that straddles an interior
/// knot the first-order hull rule, where no smoothness is assumed.
/// The two PATCH lanes do it differently and should not be read
/// through this sentence: their cells are cut ON the interior knots
/// ([`knot_aligned_cuts`]), so a straddling cell does not arise.
struct DerivLadder {
    /// (kv if materialisable, coefficient brackets) per order 1..=3;
    /// `None` when the level is an in-span zero.
    levels: [Option<(Option<KnotVector>, Vec<RingInterval>)>; 3],
}

/// Materialise the derivative knot vector (degree ≥ 2 parents only —
/// [`KnotVector`] deliberately refuses degree 0).
fn deriv_kv(kv: &KnotVector) -> Option<KnotVector> {
    if kv.degree() < 2 {
        return None;
    }
    let inner = kv.knots()[1..kv.knots().len() - 1].to_vec();
    KnotVector::clamped(inner, kv.degree() - 1).ok()
}

impl DerivLadder {
    fn build(kv: &KnotVector, coeffs: &[RingInterval]) -> Self {
        let mut levels: [Option<(Option<KnotVector>, Vec<RingInterval>)>; 3] = [None, None, None];
        let mut cur_kv = Some(kv.clone());
        let mut cur_coeffs = coeffs.to_vec();
        for level in levels.iter_mut() {
            let Some(k) = &cur_kv else { break };
            if cur_coeffs.len() < 2 {
                break;
            }
            let q = derivative_coeffs(k, &cur_coeffs);
            let next_kv = deriv_kv(k);
            *level = Some((next_kv.clone(), q.clone()));
            cur_kv = next_kv;
            cur_coeffs = q;
        }
        Self { levels }
    }

    /// Hull of the `order`-th derivative over `[lo, hi]`, assuming the
    /// piece is knot-free (module docs). `order` ∈ 1..=3.
    fn hull(&self, order: usize, lo: f64, hi: f64) -> RingInterval {
        match &self.levels[order - 1] {
            // In-span polynomial zero (degree exhausted).
            None => RingInterval::zero(),
            Some((Some(kv), q)) => bspline_range_hull(kv, q, lo, hi),
            // Coefficients exist but their kv does not (piecewise
            // constants): the whole-domain coefficient hull is a sound
            // range bound for any sub-interval.
            Some((None, q)) => {
                let mut acc = RingInterval::poison();
                for (n, c) in q.iter().enumerate() {
                    acc = if n == 0 {
                        *c
                    } else {
                        RingInterval::hull(acc, *c)
                    };
                }
                acc
            }
        }
    }
}

/// `∫_a^b u(t)·v\'(t) dt` for a nonrational B-spline pcurve `(u, v)` on
/// a shared knot vector — the general hull-bounded integrand lane
/// (module docs). Pieces free of interior knots use the composite rule
/// `h·f(m) + F₂·h³/24` (channel values via interval de Boor, `F₂` from
/// the derivative ladder); pieces straddling an interior knot use the
/// smoothness-free first-order hull rule `h·hull(f)` — both sound, so
/// the total is an enclosure at every resolution.
///
/// **No at-rest construction mints a stored B-spline pcurve**, so this
/// lane's callers today are tests (module docs). What M6-3 added on
/// loft walls is `Pcurve::IsoLine` — an exact straight line in UV, not
/// a spline — which this integrand does not take; a spline chart image
/// is the SSI trace's `Pcurve::Fitted`, and the construction that first
/// stores one at rest brings this lane's consumer with it. The
/// weights-not-1 refusal below says the same thing. Rational pcurves
/// refuse typed — a rational derivative is not a control-coefficient
/// convexity fact.
///
/// # Errors
///
/// [`PropsError::QuadratureUnsupported`] on rational weights or a
/// degenerate degree/knot structure.
pub fn bspline_green_integral(
    kv: &KnotVector,
    u_coeffs: &[RingInterval],
    v_coeffs: &[RingInterval],
    weights: &[f64],
    a: f64,
    b: f64,
    pieces: usize,
) -> Result<RingInterval, PropsError> {
    if weights.iter().any(|w| *w != 1.0) {
        return Err(PropsError::QuadratureUnsupported {
            what: "rational pcurve channels (weights != 1) — a rational derivative is not \
                   a hull convexity fact; no at-rest construction mints one (the loft \
                   assembly unit brings stored B-spline pcurves)",
        });
    }
    let span = b - a;
    if !(span.is_finite() && span >= 0.0) || pieces == 0 {
        return Err(PropsError::QuadratureUnsupported {
            what: "empty or non-finite parameter interval",
        });
    }
    let u_ladder = DerivLadder::build(kv, u_coeffs);
    let v_ladder = DerivLadder::build(kv, v_coeffs);
    let Some((v1_kv, v1)) = &v_ladder.levels[0] else {
        return Err(PropsError::QuadratureUnsupported {
            what: "height channel too degenerate to differentiate",
        });
    };
    let interior: Vec<f64> = {
        let (d0, d1) = kv.domain();
        kv.knots()
            .iter()
            .copied()
            .filter(|k| *k > d0 && *k < d1)
            .collect()
    };
    let mut total = RingInterval::zero();
    #[allow(clippy::cast_precision_loss)]
    let h = span / pieces as f64;
    let h3_24 = pt(h) * pt(h).sqr() / pt(24.0);
    for i in 0..pieces {
        #[allow(clippy::cast_precision_loss)]
        let p_lo = a + span * (i as f64 / pieces as f64);
        let p_hi = p_lo + h;
        let straddles = interior.iter().any(|k| *k > p_lo && *k < p_hi);
        let uh = bspline_range_hull(kv, u_coeffs, p_lo, p_hi);
        let v1h = match v1_kv {
            Some(k) => bspline_range_hull(k, v1, p_lo, p_hi),
            None => v_ladder.hull(1, p_lo, p_hi),
        };
        if straddles {
            // Smoothness-free rule across the knot.
            total = total + pt(h) * (uh * v1h);
            continue;
        }
        let m = p_lo + h * 0.5;
        let fm = bspline_eval_ring(kv, u_coeffs, m)
            * match v1_kv {
                Some(k) => bspline_eval_ring(k, v1, m),
                None => v1h,
            };
        // f\'\' = u\'\'·v\' + 2·u\'·v\'\' + u·v\'\'\' over the knot-free piece.
        let f2 = u_ladder.hull(2, p_lo, p_hi) * v1h
            + pt(2.0) * u_ladder.hull(1, p_lo, p_hi) * v_ladder.hull(2, p_lo, p_hi)
            + uh * v_ladder.hull(3, p_lo, p_hi);
        total = total + pt(h) * fm + f2 * h3_24;
    }
    Ok(total)
}

// ---------------------------------------------------------------------
// The NURBS-patch lane (M6-3 Leg C): certified volume flux (+ the
// area the +V meter consumes) over a full UV-rectangle patch
// ---------------------------------------------------------------------

/// Initial pieces PER AXIS of the 2-D composite rule (cells = p²).
const QUAD2_INIT_PIECES: usize = 8;
/// Refinement rounds before the typed budget refusal (pieces double
/// per axis per round: ≤ `8·2⁶ = 512` per axis). Fewer rounds than the
/// 1-D lane because cells grow quadratically; the rule's error is
/// O(h²), so real patches converge in the first rounds or not at all.
const QUAD2_MAX_ROUNDS: usize = 6;
/// Blocks per axis for the RATIONAL lane's Taylor-remainder hulls
/// (fixed, D9 — never data-dependent). `QUAD2_INIT_PIECES` is a
/// multiple of it, so every block owns whole cells at every
/// refinement level ([`rational_patch_face`]).
const QUAD2_HULL_BLOCKS: usize = 8;
/// Refinement rounds for the RATIONAL lane (pieces double per axis
/// per round: ≤ `8·2⁷ = 1024` per axis). More rounds than the
/// integral composite because the rational lane has no exact per-span
/// shortcut to fall back on — the midpoint rule's O(h²) IS the whole
/// convergence story there ([`rational_patch_face`]).
const QUAD2_RATIONAL_MAX_ROUNDS: usize = 7;
/// Cells per axis of BOTH patch lanes' area pass (fixed, D9). The
/// shared [`area_midpoint_taylor`] rule is O(h), so the resolution sets
/// the area's honest width directly.
const QUAD2_AREA_PIECES: usize = 64;
/// Spans per axis the RATIONAL lane refines its bracketed net to
/// before hulling anything (fixed, D9) — see [`refine_dir`] for why
/// span-granular hulls make this the only lever that works.
const QUAD2_REFINE_SPANS: usize = 16;

/// A ring-bracketed 3-vector (a control point, an enclosure).
pub type RVec3 = [RingInterval; 3];

fn rv_cross(a: RVec3, b: RVec3) -> RVec3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn rv_dot(a: RVec3, b: RVec3) -> RingInterval {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Sound enclosure of `√x` for a nonnegative-by-construction `x`
/// (component squares summed): correctly rounded `f64` sqrt widened
/// one ulp outward; a spurious negative low (impossible here — inputs
/// go through [`RingInterval::sqr`]) clamps to zero, the safe
/// direction for a magnitude.
fn sqrt_enclosure(x: RingInterval) -> RingInterval {
    // The early-out is what makes the `.max(0.0)` clamps below safe: past
    // it the endpoints are non-NaN, so no `f64::max` can absorb poison
    // into a plausible magnitude. Callers do pass poisonable sums.
    if x.is_poison() {
        return x;
    }
    let lo = x.lo().max(0.0).sqrt();
    let hi = x.hi().max(0.0).sqrt();
    RingInterval::from_bounds(lo.next_down().max(0.0), hi.next_up())
}

/// Widens both ends by a nonnegative pad (the honesty-pad fold).
fn widen(x: RingInterval, pad: f64) -> RingInterval {
    if x.is_poison() {
        return x;
    }
    RingInterval::from_bounds(x.lo() - pad, x.hi() + pad)
}

/// How a tensor direction collapses: a thin evaluation, a thin
/// evaluation of THIS SPAN'S polynomial at an enclosed parameter (the
/// exact Newton–Cotes lane — the span is located by a strictly
/// interior point, and the span polynomial is evaluated even at the
/// span's closed endpoints, which is exactly what integrating that
/// polynomial over the span wants), or a range hull.
#[derive(Clone, Copy)]
enum Collapse<'a> {
    At(f64),
    AtSpan {
        /// A point strictly inside the target span (locates it in
        /// every derivative knot vector at once — they share
        /// breakpoints).
        mid: f64,
        /// The enclosed evaluation parameter.
        t: &'a RingInterval,
    },
    Over(f64, f64),
}

/// One direction's knot structure: a real knot vector, or the
/// degree-0 remnant a derivative ladder bottoms out at (coefficients
/// are per-span constants over `knots[i]..knots[i+1]` — the parent's
/// interior knot list, kept so span-LOCAL collapse stays exact; the
/// 1-D ladder's whole-domain hull would lose the per-span constancy
/// the Newton–Cotes lane depends on).
#[derive(Clone)]
enum Dir {
    Kv(KnotVector),
    /// A derivative direction whose knot vector is **not representable**
    /// as a [`KnotVector`]: an interior knot of multiplicity equal to
    /// the parent's degree (the standard rational-arc structure) makes
    /// the derivative genuinely DISCONTINUOUS there, and the clamped
    /// invariant refuses interior multiplicity above the degree.
    ///
    /// Before M8-3 this case fell through to [`Dir::Const`], which
    /// silently read the FIRST derivative of a degree-2 spline as a
    /// per-span constant — wrong by O(1), and reachable from the
    /// integral lane's composite fallback too. Evaluated span-locally
    /// on the raw knots instead; the discontinuity is honest, and no
    /// patch cell spans it — the composite cells are cut on the
    /// interior knots ([`knot_aligned_cuts`]).
    Raw {
        knots: Vec<f64>,
        degree: usize,
    },
    Const {
        knots: Vec<f64>,
    },
}

/// The span index of `t` in a raw knot slice: the last nonempty span
/// whose lower knot does not exceed `t`, clamped into the valid range.
fn raw_span(knots: &[f64], degree: usize, count: usize, t: f64) -> usize {
    let last = count.saturating_sub(1).max(degree);
    let mut span = degree;
    let mut i = degree;
    while i <= last && i + 1 < knots.len() {
        if knots[i] <= t && knots[i] < knots[i + 1] {
            span = i;
        }
        i += 1;
    }
    span.min(last)
}

/// In-span de Boor on a raw knot slice (the [`Dir::Raw`] evaluator).
fn raw_eval(knots: &[f64], degree: usize, coeffs: &[RingInterval], t: f64) -> RingInterval {
    if coeffs.len() < degree + 1 || knots.len() < coeffs.len() + degree + 1 || !t.is_finite() {
        return RingInterval::poison();
    }
    let span = raw_span(knots, degree, coeffs.len(), t);
    let mut d: Vec<RingInterval> = (0..=degree).map(|j| coeffs[span - degree + j]).collect();
    for r in 1..=degree {
        for j in (r..=degree).rev() {
            let i = span - degree + j;
            let denom = pt(knots[i + degree + 1 - r]) - pt(knots[i]);
            let alpha = (pt(t) - pt(knots[i])) / denom;
            d[j] = (pt(1.0) - alpha) * d[j - 1] + alpha * d[j];
        }
    }
    d[degree]
}

/// Hull of a [`Dir::Raw`] spline over `[lo, hi]`: the local control
/// blocks of every touched span (the same convexity fact
/// [`bspline_range_hull`] uses).
fn raw_range_hull(
    knots: &[f64],
    degree: usize,
    coeffs: &[RingInterval],
    lo: f64,
    hi: f64,
) -> RingInterval {
    if coeffs.len() < degree + 1 {
        return RingInterval::poison();
    }
    let (s0, s1) = (
        raw_span(knots, degree, coeffs.len(), lo),
        raw_span(knots, degree, coeffs.len(), hi),
    );
    let mut acc = RingInterval::poison();
    let mut seeded = false;
    for span in s0..=s1 {
        for j in 0..=degree {
            let Some(c) = coeffs.get(span - degree + j) else {
                continue;
            };
            acc = if seeded {
                RingInterval::hull(acc, *c)
            } else {
                *c
            };
            seeded = true;
        }
    }
    acc
}

/// Derivative coefficients on a raw knot slice.
fn raw_deriv(knots: &[f64], degree: usize, coeffs: &[RingInterval]) -> Vec<RingInterval> {
    if degree == 0 || coeffs.len() < 2 {
        return Vec::new();
    }
    #[allow(clippy::cast_precision_loss)]
    let p = pt(degree as f64);
    (0..coeffs.len() - 1)
        .map(|i| {
            let (Some(&a), Some(&b)) = (knots.get(i + degree + 1), knots.get(i + 1)) else {
                return RingInterval::poison();
            };
            // `knots[i+1] == knots[i+degree+1]` marks a DEGENERATE
            // (empty) span — the derivative has no coefficient there
            // because the function has no value there. `raw_span`
            // never selects an empty span, so this slot is only ever
            // hulled; zero is the safe filler (enlarging a hull can
            // never make a containment claim false).
            if a == b {
                return RingInterval::zero();
            }
            p * (coeffs[i + 1] - coeffs[i]) / (pt(a) - pt(b))
        })
        .collect()
}

/// One direction's coefficient-differentiation step: the map from a
/// line of the grid to the same line of its derivative grid.
type DerivTake = Box<dyn Fn(&[RingInterval]) -> Vec<RingInterval>>;

impl Dir {
    /// The per-span-constant coefficient index for a point.
    fn const_index(knots: &[f64], t: f64, count: usize) -> usize {
        let mut i = 0usize;
        while i + 1 < count && i + 1 < knots.len() && knots[i + 1] <= t {
            i += 1;
        }
        i
    }
}

/// In-span de Boor at an ENCLOSED parameter: evaluates the span's
/// polynomial (the [`bspline_eval_ring`] recurrence with the span
/// fixed by the caller and `t` carried as a bracket) — sound for any
/// `t`, exact-in-kind for the Newton–Cotes nodes, which lie in the
/// span's closure.
fn bspline_eval_ring_in_span(
    kv: &KnotVector,
    coeffs: &[RingInterval],
    span: Span,
    t: RingInterval,
) -> RingInterval {
    if coeffs.len() != kv.control_count() {
        return RingInterval::poison();
    }
    let p = kv.degree();
    let u = kv.knots();
    // The window's base, off the `Span` — as in [`bspline_eval_ring`],
    // whose recurrence this is. The length check above is the only
    // structure left to verify: in-range-ness came with the `Span`.
    let first = span.first_control();
    let mut d: Vec<RingInterval> = (0..=p).map(|j| coeffs[first + j]).collect();
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = first + j;
            let denom = pt(u[i + p + 1 - r]) - pt(u[i]);
            let alpha = (t - pt(u[i])) / denom;
            d[j] = (pt(1.0) - alpha) * d[j - 1] + alpha * d[j];
        }
    }
    d[p]
}

/// One (possibly derivative-exhausted) tensor grid of a vector
/// channel triple, row-major `iu·nv + iv` — the 2-D counterpart of
/// [`DerivLadder`]'s levels: a [`Dir::Const`] direction holds
/// per-span constants, and a grid that differentiates to nothing at
/// all is `Option<PatchGrid> = None`, read as identically zero — sound
/// on a KNOT-FREE cell, where the piece really is one polynomial of
/// exhausted degree, and that is what [`knot_aligned_cuts`] makes
/// every cell of both patch composites.
struct PatchGrid {
    du: Dir,
    dv: Dir,
    nu: usize,
    nv: usize,
    ch: [Vec<RingInterval>; 3],
}
impl PatchGrid {
    /// The base grid from a bracketed control net.
    fn base(kv_u: &KnotVector, kv_v: &KnotVector, control: &[RVec3]) -> Self {
        let mut ch = [Vec::new(), Vec::new(), Vec::new()];
        for c in control {
            for (k, chan) in ch.iter_mut().enumerate() {
                chan.push(c[k]);
            }
        }
        Self {
            du: Dir::Kv(kv_u.clone()),
            dv: Dir::Kv(kv_v.clone()),
            nu: kv_u.control_count(),
            nv: kv_v.control_count(),
            ch,
        }
    }

    /// The derivative direction structure of a knot vector: a real
    /// vector while the degree allows, the per-span-constant remnant
    /// at degree 0.
    fn deriv_dir(kv: &KnotVector) -> Dir {
        let inner = kv.knots()[1..kv.knots().len() - 1].to_vec();
        match deriv_kv(kv) {
            Some(k) => Dir::Kv(k),
            // Degree ≥ 2 with an unrepresentable derivative structure
            // is the DISCONTINUOUS-derivative case, not the
            // per-span-constant one (see `Dir::Raw`).
            None if kv.degree() >= 2 => Dir::Raw {
                knots: inner,
                degree: kv.degree() - 1,
            },
            None => Dir::Const { knots: inner },
        }
    }

    /// The u-partial-derivative grid (`None` = identically zero away
    /// from knots — the ladder's outer-None).
    fn deriv_u(&self) -> Option<Self> {
        if self.nu < 2 {
            return None;
        }
        let (next_dir, take): (Dir, DerivTake) = match &self.du {
            Dir::Kv(kv) => {
                let kv = kv.clone();
                (
                    Self::deriv_dir(&kv),
                    Box::new(move |c: &[RingInterval]| derivative_coeffs(&kv, c)),
                )
            }
            // A `Raw` direction differentiates too — its own
            // derivative is `Raw` one degree down, or the honest
            // per-span constant at degree 1.
            Dir::Raw { knots, degree } => {
                let (knots, degree) = (knots.clone(), *degree);
                let inner = knots[1..knots.len() - 1].to_vec();
                let next = if degree >= 2 {
                    Dir::Raw {
                        knots: inner,
                        degree: degree - 1,
                    }
                } else {
                    Dir::Const { knots: inner }
                };
                (
                    next,
                    Box::new(move |c: &[RingInterval]| raw_deriv(&knots, degree, c)),
                )
            }
            Dir::Const { .. } => return None,
        };
        let mut ch = [Vec::new(), Vec::new(), Vec::new()];
        for (k, chan) in ch.iter_mut().enumerate() {
            let mut grid = vec![RingInterval::zero(); (self.nu - 1) * self.nv];
            for j in 0..self.nv {
                let col: Vec<RingInterval> =
                    (0..self.nu).map(|i| self.ch[k][i * self.nv + j]).collect();
                let d = take(&col);
                for (i, q) in d.iter().enumerate() {
                    if let Some(slot) = grid.get_mut(i * self.nv + j) {
                        *slot = *q;
                    }
                }
            }
            *chan = grid;
        }
        Some(Self {
            du: next_dir,
            dv: self.dv.clone(),
            nu: self.nu - 1,
            nv: self.nv,
            ch,
        })
    }

    /// The v-partial-derivative grid.
    fn deriv_v(&self) -> Option<Self> {
        if self.nv < 2 {
            return None;
        }
        let (next_dir, take): (Dir, DerivTake) = match &self.dv {
            Dir::Kv(kv) => {
                let kv = kv.clone();
                (
                    Self::deriv_dir(&kv),
                    Box::new(move |c: &[RingInterval]| derivative_coeffs(&kv, c)),
                )
            }
            Dir::Raw { knots, degree } => {
                let (knots, degree) = (knots.clone(), *degree);
                let inner = knots[1..knots.len() - 1].to_vec();
                let next = if degree >= 2 {
                    Dir::Raw {
                        knots: inner,
                        degree: degree - 1,
                    }
                } else {
                    Dir::Const { knots: inner }
                };
                (
                    next,
                    Box::new(move |c: &[RingInterval]| raw_deriv(&knots, degree, c)),
                )
            }
            Dir::Const { .. } => return None,
        };
        let mut ch = [Vec::new(), Vec::new(), Vec::new()];
        for (k, chan) in ch.iter_mut().enumerate() {
            let mut grid = Vec::with_capacity(self.nu * (self.nv - 1));
            for i in 0..self.nu {
                let row = &self.ch[k][i * self.nv..(i + 1) * self.nv];
                grid.extend(take(row));
            }
            *chan = grid;
        }
        Some(Self {
            du: self.du.clone(),
            dv: next_dir,
            nu: self.nu,
            nv: self.nv - 1,
            ch,
        })
    }

    /// One direction's collapse of a coefficient slice.
    fn collapse_1d(dir: &Dir, coeffs: &[RingInterval], op: Collapse<'_>) -> RingInterval {
        match (dir, op) {
            (Dir::Kv(kv), Collapse::At(t)) => bspline_eval_ring(kv, coeffs, t),
            (Dir::Kv(kv), Collapse::AtSpan { mid, t }) => {
                bspline_eval_ring_in_span(kv, coeffs, kv.span_at(mid), *t)
            }
            (Dir::Kv(kv), Collapse::Over(lo, hi)) => bspline_range_hull(kv, coeffs, lo, hi),
            (Dir::Raw { knots, degree }, Collapse::At(t)) => raw_eval(knots, *degree, coeffs, t),
            (Dir::Raw { knots, degree }, Collapse::AtSpan { mid, t }) => {
                // The node lies in the closure of `mid`'s span; the
                // span polynomial is what the rule integrates.
                let span = raw_span(knots, *degree, coeffs.len(), mid);
                let mut d: Vec<RingInterval> = (0..=*degree)
                    .map(|j| {
                        coeffs
                            .get(span.saturating_sub(*degree) + j)
                            .copied()
                            .unwrap_or_else(RingInterval::poison)
                    })
                    .collect();
                for r in 1..=*degree {
                    for j in (r..=*degree).rev() {
                        let i = span - *degree + j;
                        let (Some(&ka), Some(&kb)) = (knots.get(i + *degree + 1 - r), knots.get(i))
                        else {
                            return RingInterval::poison();
                        };
                        let alpha = (*t - pt(kb)) / (pt(ka) - pt(kb));
                        d[j] = (pt(1.0) - alpha) * d[j - 1] + alpha * d[j];
                    }
                }
                d[*degree]
            }
            (Dir::Raw { knots, degree }, Collapse::Over(lo, hi)) => {
                raw_range_hull(knots, *degree, coeffs, lo, hi)
            }
            (Dir::Const { knots }, Collapse::At(t)) => {
                coeffs[Dir::const_index(knots, t, coeffs.len())]
            }
            (Dir::Const { knots }, Collapse::AtSpan { mid, .. }) => {
                coeffs[Dir::const_index(knots, mid, coeffs.len())]
            }
            (Dir::Const { knots }, Collapse::Over(lo, hi)) => {
                let a = Dir::const_index(knots, lo, coeffs.len());
                let b = Dir::const_index(knots, hi, coeffs.len());
                let mut acc = coeffs[a];
                for c in &coeffs[a..=b.max(a)] {
                    acc = RingInterval::hull(acc, *c);
                }
                acc
            }
        }
    }

    /// Collapses one channel: v first (per u-row), then u.
    fn channel(&self, k: usize, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        let rows: Vec<RingInterval> = (0..self.nu)
            .map(|i| Self::collapse_1d(&self.dv, &self.ch[k][i * self.nv..(i + 1) * self.nv], v))
            .collect();
        Self::collapse_1d(&self.du, &rows, u)
    }

    /// **The u-first collapse**: this grid's three channels reduced in
    /// the u direction only, leaving one v-coefficient vector each.
    ///
    /// Mathematically identical to [`PatchGrid::channel`]'s v-then-u
    /// order (a tensor collapse commutes); the association differs, so
    /// the ring rounding does — which is why only the rational lane,
    /// whose numbers are its own, uses it. Its point is that every
    /// cell in one column of the composite grid shares the SAME u
    /// collapse, so hoisting it out of the inner loop turns a
    /// per-cell `nu`-fold de Boor into a per-column one.
    fn slice_u(&self, u: Collapse<'_>) -> [Vec<RingInterval>; 3] {
        core::array::from_fn(|k| {
            let mut col = vec![RingInterval::zero(); self.nu];
            (0..self.nv)
                .map(|j| {
                    for (i, c) in col.iter_mut().enumerate() {
                        *c = self.ch[k][i * self.nv + j];
                    }
                    Self::collapse_1d(&self.du, &col, u)
                })
                .collect()
        })
    }

    /// The v collapse of a [`PatchGrid::slice_u`] result.
    fn at_v(&self, sl: &[Vec<RingInterval>; 3], v: Collapse<'_>) -> RVec3 {
        [
            Self::collapse_1d(&self.dv, &sl[0], v),
            Self::collapse_1d(&self.dv, &sl[1], v),
            Self::collapse_1d(&self.dv, &sl[2], v),
        ]
    }

    /// The vector value/hull of the whole triple.
    fn vec(&self, u: Collapse<'_>, v: Collapse<'_>) -> RVec3 {
        [
            self.channel(0, u, v),
            self.channel(1, u, v),
            self.channel(2, u, v),
        ]
    }
}

/// The zero-or-collapse read of an optional (derivative) grid.
fn grid_vec(g: Option<&PatchGrid>, u: Collapse<'_>, v: Collapse<'_>) -> RVec3 {
    match g {
        Some(g) => g.vec(u, v),
        None => [
            RingInterval::zero(),
            RingInterval::zero(),
            RingInterval::zero(),
        ],
    }
}

/// Closed Newton–Cotes weights on `[0, 1]` with `m + 1` equally
/// spaced nodes `j/m`, as EXACT `i128` fractions reduced and then
/// bracketed outward — no Gaussian trust anywhere: the rule's
/// algebraic exactness for polynomials of degree ≤ m (m odd; m + 1
/// for even m) is a theorem, the nodes are rational, and the
/// enclosure carries only the ring divisions' rounding. `None` when
/// `m` is outside the supported window (the fraction arithmetic's
/// `i128` headroom, m ≤ 12 — callers fall back to the composite
/// rule).
fn newton_cotes_weights(m: usize) -> Option<Vec<RingInterval>> {
    if m == 0 || m > 12 {
        return None;
    }
    fn gcd(a: i128, b: i128) -> i128 {
        let (mut a, mut b) = (a.abs(), b.abs());
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a.max(1)
    }
    let mi = m as i128;
    let mut out = Vec::with_capacity(m + 1);
    for j in 0..=m {
        // L_j(x) = Π_{k≠j} (m·x − k) / (j − k): numerator coefficients
        // by convolution, denominator the exact product.
        let mut num: Vec<i128> = vec![1];
        let mut den: i128 = 1;
        for k in 0..=m {
            if k == j {
                continue;
            }
            let ki = k as i128;
            let ji = j as i128;
            den = den.checked_mul(ji - ki)?;
            let mut next = vec![0i128; num.len() + 1];
            for (i, c) in num.iter().enumerate() {
                next[i + 1] = next[i + 1].checked_add(c.checked_mul(mi)?)?;
                next[i] = next[i].checked_add(c.checked_mul(-ki)?)?;
            }
            num = next;
        }
        // ∫₀¹ Σ cᵢ xⁱ dx = Σ cᵢ/(i+1), accumulated as one fraction.
        let (mut acc_n, mut acc_d): (i128, i128) = (0, 1);
        for (i, c) in num.iter().enumerate() {
            let d = (i as i128) + 1;
            acc_n = acc_n.checked_mul(d)?.checked_add(c.checked_mul(acc_d)?)?;
            acc_d = acc_d.checked_mul(d)?;
            let g = gcd(acc_n, acc_d);
            acc_n /= g;
            acc_d /= g;
        }
        // w_j = acc / den, sign-normalised, bracketed via outward ring
        // division of exactly-representable integers.
        let mut w_n = acc_n;
        let mut w_d = acc_d.checked_mul(den)?;
        if w_d < 0 {
            w_n = -w_n;
            w_d = -w_d;
        }
        let g = gcd(w_n, w_d);
        w_n /= g;
        w_d /= g;
        let exact53 = |x: i128| x.abs() < (1i128 << 53);
        if !exact53(w_n) || !exact53(w_d) {
            return None;
        }
        #[allow(clippy::cast_precision_loss)]
        out.push(pt(w_n as f64) / pt(w_d as f64));
    }
    Some(out)
}

/// The knot-span intervals of a vector clipped to `[lo, hi]`, each
/// with a strictly interior locator point.
fn clipped_spans(kv: &KnotVector, lo: f64, hi: f64) -> Vec<(f64, f64, f64)> {
    let mut out = Vec::new();
    let mut edges: Vec<f64> = vec![lo];
    for k in kv.knots() {
        if *k > lo && *k < hi && edges.last().is_none_or(|e| *e != *k) {
            edges.push(*k);
        }
    }
    edges.push(hi);
    for w in edges.windows(2) {
        if w[1] > w[0] {
            out.push((w[0], w[1], w[0].midpoint(w[1])));
        }
    }
    out
}

/// **The exact per-span lane** (the primary flux path): on each
/// knot-span rectangle the integrand `f = S·(S_u×S_v)` is one
/// polynomial of degree ≤ 3p−1 per direction, so the tensor closed
/// Newton–Cotes rule of order `3p` integrates it EXACTLY — the whole
/// enclosure width is the nodes' and weights' ring rounding. `None`
/// when the rule order leaves the supported window (degree > 4 per
/// direction), in which case the caller runs the composite rounds.
fn patch_flux_exact(
    s: &PatchGrid,
    su: Option<&PatchGrid>,
    sv: Option<&PatchGrid>,
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    rect: (f64, f64, f64, f64),
) -> Option<RingInterval> {
    let (u0, u1, v0, v1) = rect;
    let (mu, mv) = (3 * kv_u.degree(), 3 * kv_v.degree());
    let (wu, wv) = (newton_cotes_weights(mu)?, newton_cotes_weights(mv)?);
    let mut flux = RingInterval::zero();
    for &(au, bu, mid_u) in &clipped_spans(kv_u, u0, u1) {
        let su_scale = pt(bu) - pt(au);
        // Node enclosures for this span, u direction.
        #[allow(clippy::cast_precision_loss)]
        let nodes_u: Vec<RingInterval> = (0..=mu)
            .map(|j| pt(au) + su_scale * (pt(j as f64) / pt(mu as f64)))
            .collect();
        for &(av, bv, mid_v) in &clipped_spans(kv_v, v0, v1) {
            let sv_scale = pt(bv) - pt(av);
            #[allow(clippy::cast_precision_loss)]
            let nodes_v: Vec<RingInterval> = (0..=mv)
                .map(|j| pt(av) + sv_scale * (pt(j as f64) / pt(mv as f64)))
                .collect();
            let scale = su_scale * sv_scale;
            let mut acc = RingInterval::zero();
            for (nu_i, wu_i) in nodes_u.iter().zip(&wu) {
                let cu = Collapse::AtSpan {
                    mid: mid_u,
                    t: nu_i,
                };
                let mut row = RingInterval::zero();
                for (nv_j, wv_j) in nodes_v.iter().zip(&wv) {
                    let cv = Collapse::AtSpan {
                        mid: mid_v,
                        t: nv_j,
                    };
                    let fm = rv_dot(
                        s.vec(cu, cv),
                        rv_cross(grid_vec(su, cu, cv), grid_vec(sv, cu, cv)),
                    );
                    row = row + *wv_j * fm;
                }
                acc = acc + *wu_i * row;
            }
            flux = flux + scale * acc;
        }
    }
    Some(flux)
}

// ---------------------------------------------------------------------
// The RATIONAL patch lane (M8-3): `S = A/w` through the quotient rule
// ---------------------------------------------------------------------

/// The derivative ladder of one polynomial (B-spline) net, built once.
///
/// The ladder bottoms out in `None` exactly where the polynomial
/// degree runs out, which [`grid_vec`] reads as the identically-zero
/// derivative. Two instantiations: the homogeneous position net
/// `A = w·P` (all three channels) and the weight net `w` (channel 0,
/// the other two structural zeros).
struct Ladder {
    a: PatchGrid,
    au: Option<PatchGrid>,
    av: Option<PatchGrid>,
    auu: Option<PatchGrid>,
    auv: Option<PatchGrid>,
    avv: Option<PatchGrid>,
    auuu: Option<PatchGrid>,
    auuv: Option<PatchGrid>,
    auvv: Option<PatchGrid>,
    avvv: Option<PatchGrid>,
}

impl Ladder {
    fn build(kv_u: &KnotVector, kv_v: &KnotVector, net: &[RVec3]) -> Self {
        let a = PatchGrid::base(kv_u, kv_v, net);
        let au = a.deriv_u();
        let av = a.deriv_v();
        let auu = au.as_ref().and_then(PatchGrid::deriv_u);
        let auv = au.as_ref().and_then(PatchGrid::deriv_v);
        let avv = av.as_ref().and_then(PatchGrid::deriv_v);
        let auuu = auu.as_ref().and_then(PatchGrid::deriv_u);
        let auuv = auu.as_ref().and_then(PatchGrid::deriv_v);
        let auvv = auv.as_ref().and_then(PatchGrid::deriv_v);
        let avvv = avv.as_ref().and_then(PatchGrid::deriv_v);
        Self {
            a,
            au,
            av,
            auu,
            auv,
            avv,
            auuu,
            auuv,
            auvv,
            avvv,
        }
    }

    /// `N = A·(A_u×A_v)` — the flux integrand's NUMERATOR
    /// ([`rational_patch_face`] docs derive why the quotient's other
    /// triple products vanish).
    fn num(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        rv_dot(
            self.a.vec(u, v),
            rv_cross(
                grid_vec(self.au.as_ref(), u, v),
                grid_vec(self.av.as_ref(), u, v),
            ),
        )
    }

    /// `N_uu = A_u·(A_uu×A_v) + A·(A_uuu×A_v) + 2·A·(A_uu×A_uv) +
    /// A·(A_u×A_uuv)` — the integral lane's `f_uu`, verbatim, on the
    /// homogeneous net.
    fn num_uu(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        let a = self.a.vec(u, v);
        let au = grid_vec(self.au.as_ref(), u, v);
        let av = grid_vec(self.av.as_ref(), u, v);
        let auu = grid_vec(self.auu.as_ref(), u, v);
        let auv = grid_vec(self.auv.as_ref(), u, v);
        rv_dot(au, rv_cross(auu, av))
            + rv_dot(a, rv_cross(grid_vec(self.auuu.as_ref(), u, v), av))
            + pt(2.0) * rv_dot(a, rv_cross(auu, auv))
            + rv_dot(a, rv_cross(au, grid_vec(self.auuv.as_ref(), u, v)))
    }

    /// `N_vv = A_v·(A_u×A_vv) + A·(A_uvv×A_v) + 2·A·(A_uv×A_vv) +
    /// A·(A_u×A_vvv)`.
    fn num_vv(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        let a = self.a.vec(u, v);
        let au = grid_vec(self.au.as_ref(), u, v);
        let av = grid_vec(self.av.as_ref(), u, v);
        let auv = grid_vec(self.auv.as_ref(), u, v);
        let avv = grid_vec(self.avv.as_ref(), u, v);
        rv_dot(av, rv_cross(au, avv))
            + rv_dot(a, rv_cross(grid_vec(self.auvv.as_ref(), u, v), av))
            + pt(2.0) * rv_dot(a, rv_cross(auv, avv))
            + rv_dot(a, rv_cross(au, grid_vec(self.avvv.as_ref(), u, v)))
    }

    /// `N_u = A·(A_uu×A_v) + A·(A_u×A_uv)` (the `A_u·(A_u×A_v)` term
    /// vanishes identically).
    fn num_u(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        let a = self.a.vec(u, v);
        rv_dot(
            a,
            rv_cross(
                grid_vec(self.auu.as_ref(), u, v),
                grid_vec(self.av.as_ref(), u, v),
            ),
        ) + rv_dot(
            a,
            rv_cross(
                grid_vec(self.au.as_ref(), u, v),
                grid_vec(self.auv.as_ref(), u, v),
            ),
        )
    }

    /// `N_v = A·(A_uv×A_v) + A·(A_u×A_vv)`.
    fn num_v(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        let a = self.a.vec(u, v);
        rv_dot(
            a,
            rv_cross(
                grid_vec(self.auv.as_ref(), u, v),
                grid_vec(self.av.as_ref(), u, v),
            ),
        ) + rv_dot(
            a,
            rv_cross(
                grid_vec(self.au.as_ref(), u, v),
                grid_vec(self.avv.as_ref(), u, v),
            ),
        )
    }

    /// The numerator of `S_u×S_v`:
    /// `w·(A_u×A_v) − w_v·(A_u×A) − w_u·(A×A_v)`, which sits over
    /// `w³` ([`rational_patch_face`] docs).
    fn cross_num(&self, w: &Self, u: Collapse<'_>, v: Collapse<'_>) -> RVec3 {
        let a = self.a.vec(u, v);
        let au = grid_vec(self.au.as_ref(), u, v);
        let av = grid_vec(self.av.as_ref(), u, v);
        let (w0, wu, wv) = (w.chan(u, v), w.chan_u(u, v), w.chan_v(u, v));
        let base = rv_cross(au, av);
        let ta = rv_cross(au, a);
        let tb = rv_cross(a, av);
        [
            w0 * base[0] - wv * ta[0] - wu * tb[0],
            w0 * base[1] - wv * ta[1] - wu * tb[1],
            w0 * base[2] - wv * ta[2] - wu * tb[2],
        ]
    }

    /// The scalar (weight-net) channel reads.
    fn chan(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        self.a.channel(0, u, v)
    }
    fn chan_u(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        grid_vec(self.au.as_ref(), u, v)[0]
    }
    fn chan_v(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        grid_vec(self.av.as_ref(), u, v)[0]
    }
    fn chan_uu(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        grid_vec(self.auu.as_ref(), u, v)[0]
    }
    fn chan_vv(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        grid_vec(self.avv.as_ref(), u, v)[0]
    }
    fn chan_uv(&self, u: Collapse<'_>, v: Collapse<'_>) -> RingInterval {
        grid_vec(self.auv.as_ref(), u, v)[0]
    }

    /// `∂_u` of [`Ladder::cross_num`] — the `w_u·(A_u×A_v)` terms
    /// cancel identically:
    /// `w(A_uu×A_v) + w(A_u×A_uv) − w_uv(A_u×A) − w_v(A_uu×A) −
    /// w_uu(A×A_v) − w_u(A×A_uv)`.
    fn cross_num_u(&self, w: &Self, u: Collapse<'_>, v: Collapse<'_>) -> RVec3 {
        let a = self.a.vec(u, v);
        let au = grid_vec(self.au.as_ref(), u, v);
        let av = grid_vec(self.av.as_ref(), u, v);
        let auu = grid_vec(self.auu.as_ref(), u, v);
        let auv = grid_vec(self.auv.as_ref(), u, v);
        let terms = [
            (w.chan(u, v), rv_cross(auu, av)),
            (w.chan(u, v), rv_cross(au, auv)),
            (-w.chan_uv(u, v), rv_cross(au, a)),
            (-w.chan_v(u, v), rv_cross(auu, a)),
            (-w.chan_uu(u, v), rv_cross(a, av)),
            (-w.chan_u(u, v), rv_cross(a, auv)),
        ];
        fold_terms(&terms)
    }

    /// `∂_v` of [`Ladder::cross_num`], the mirror:
    /// `w(A_uv×A_v) + w(A_u×A_vv) − w_vv(A_u×A) − w_v(A_uv×A) −
    /// w_uv(A×A_v) − w_u(A×A_vv)`.
    fn cross_num_v(&self, w: &Self, u: Collapse<'_>, v: Collapse<'_>) -> RVec3 {
        let a = self.a.vec(u, v);
        let au = grid_vec(self.au.as_ref(), u, v);
        let av = grid_vec(self.av.as_ref(), u, v);
        let auv = grid_vec(self.auv.as_ref(), u, v);
        let avv = grid_vec(self.avv.as_ref(), u, v);
        let terms = [
            (w.chan(u, v), rv_cross(auv, av)),
            (w.chan(u, v), rv_cross(au, avv)),
            (-w.chan_vv(u, v), rv_cross(au, a)),
            (-w.chan_v(u, v), rv_cross(auv, a)),
            (-w.chan_uv(u, v), rv_cross(a, av)),
            (-w.chan_u(u, v), rv_cross(a, avv)),
        ];
        fold_terms(&terms)
    }
}

/// The u-collapsed slice of the three grids the flux midpoint needs
/// (`A`, `A_u`, `A_v`) plus the weight channel — [`PatchGrid::slice_u`]
/// for why this exists.
struct FluxSlice {
    a: [Vec<RingInterval>; 3],
    au: Option<[Vec<RingInterval>; 3]>,
    av: Option<[Vec<RingInterval>; 3]>,
    w: [Vec<RingInterval>; 3],
}

impl Ladder {
    /// This ladder's flux slice at one u collapse, with the weight
    /// ladder's base grid alongside.
    fn flux_slice(&self, w: &Self, u: Collapse<'_>) -> FluxSlice {
        FluxSlice {
            a: self.a.slice_u(u),
            au: self.au.as_ref().map(|g| g.slice_u(u)),
            av: self.av.as_ref().map(|g| g.slice_u(u)),
            w: w.a.slice_u(u),
        }
    }

    /// `N/w³` from a slice, at one v collapse — the composite rule's
    /// per-cell integrand.
    fn integrand_at(&self, w: &Self, sl: &FluxSlice, v: Collapse<'_>) -> RingInterval {
        let zero = [
            RingInterval::zero(),
            RingInterval::zero(),
            RingInterval::zero(),
        ];
        let a = self.a.at_v(&sl.a, v);
        let au = match (self.au.as_ref(), sl.au.as_ref()) {
            (Some(g), Some(s)) => g.at_v(s, v),
            _ => zero,
        };
        let av = match (self.av.as_ref(), sl.av.as_ref()) {
            (Some(g), Some(s)) => g.at_v(s, v),
            _ => zero,
        };
        rv_dot(a, rv_cross(au, av)) / w.a.at_v(&sl.w, v)[0].powi(3)
    }

    /// `A·(A_u×A_v)` from a slice, at one v collapse — the flux
    /// numerator alone, for the arm that divides by `w³` once per
    /// column instead of once per node.
    fn numerator_at(&self, sl: &FluxSlice, v: Collapse<'_>) -> RingInterval {
        let zero = [
            RingInterval::zero(),
            RingInterval::zero(),
            RingInterval::zero(),
        ];
        let a = self.a.at_v(&sl.a, v);
        let au = match (self.au.as_ref(), sl.au.as_ref()) {
            (Some(g), Some(s)) => g.at_v(s, v),
            _ => zero,
        };
        let av = match (self.av.as_ref(), sl.av.as_ref()) {
            (Some(g), Some(s)) => g.at_v(s, v),
            _ => zero,
        };
        rv_dot(a, rv_cross(au, av))
    }

    /// **The EXACT v integral** `∫ N dv` over one v-cell, from a
    /// u-slice: closed Newton–Cotes per KNOT SPAN of the cell, which
    /// is exact because `N` is one polynomial of v-degree ≤ 3q−1 on
    /// each.
    ///
    /// `N = A·(A_u×A_v)` has v-degree `q + q + (q−1)`, so the
    /// order-`3q` rule integrates it with no truncation error at all;
    /// what the returned enclosure carries is the nodes' and weights'
    /// ring rounding and the de Boor recurrence's own widening.
    ///
    /// **The subdivision is per span and not per cell**, and that is
    /// load-bearing rather than tidy. One `Collapse::AtSpan` read
    /// evaluates a span's polynomial, and it stays exact in ℝ when the
    /// node sits outside that span — but only in ℝ. The interval de
    /// Boor widens with the distance extrapolated, and it widens
    /// without bound when the refinement has left a hairline span
    /// nearby, because the recurrence divides by knot differences.
    /// Integrating span by span never extrapolates: every node lies in
    /// the span it is read through.
    fn num_v_exact(
        &self,
        sl: &FluxSlice,
        kv: &KnotVector,
        vlo: f64,
        vhi: f64,
        nc: &[RingInterval],
    ) -> RingInterval {
        let m = nc.len() - 1;
        let mut total = RingInterval::zero();
        for (a, b, mid) in clipped_spans(kv, vlo, vhi) {
            let scale = pt(b) - pt(a);
            let mut acc = RingInterval::zero();
            for (j, wj) in nc.iter().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                let t = pt(a) + scale * (pt(j as f64) / pt(m as f64));
                acc = acc + *wj * self.numerator_at(sl, Collapse::AtSpan { mid, t: &t });
            }
            total = total + scale * acc;
        }
        total
    }
}

/// Ascending-index fold of scaled 3-vector terms (D9).
fn fold_terms(terms: &[(RingInterval, RVec3)]) -> RVec3 {
    let mut acc = [
        RingInterval::zero(),
        RingInterval::zero(),
        RingInterval::zero(),
    ];
    for (c, v) in terms {
        for k in 0..3 {
            acc[k] = acc[k] + *c * v[k];
        }
    }
    acc
}

/// An upper bound on `|v|` (2-norm) of a bracketed 3-vector.
fn norm_hi(v: RVec3) -> f64 {
    sqrt_enclosure(v[0].sqr() + v[1].sqr() + v[2].sqr()).hi()
}

/// Componentwise sum of two bracketed 3-vectors.
fn rv_add(a: RVec3, b: RVec3) -> RVec3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}

/// One cell of the shared area grid: its closed extent per axis and the
/// midpoint the rule evaluates at (carried rather than re-derived, so
/// every lane's midpoint is the SAME float).
#[derive(Clone, Copy)]
struct AreaBox {
    ulo: f64,
    uhi: f64,
    umid: f64,
    vlo: f64,
    vhi: f64,
    vmid: f64,
}

/// What a lane reports for one area cell: the THIN midpoint value of
/// the area integrand `g = |S_u×S_v|`, one-sided Lipschitz bounds on
/// its two partials over the cell, and the hull of `g` over the whole
/// cell.
struct AreaCell {
    /// `g(midpoint)` — thin, and the reason this rule beats the hull
    /// rule by orders.
    g_mid: RingInterval,
    /// An upper bound on `sup_cell |∂_u g|`.
    g_u: f64,
    /// An upper bound on `sup_cell |∂_v g|`.
    g_v: f64,
    /// An enclosure of `g` over the whole cell. Both this and the
    /// padded midpoint enclose the cell's MEAN of `g`, so the rule
    /// takes their intersection: the hull needs no smoothness and is
    /// a magnitude (hence never negative), which is what keeps a cell
    /// whose derivative hulls have blown up from dragging the area
    /// bracket across zero; the padded midpoint is orders tighter
    /// wherever the derivative hulls are sane. Neither is a fallback
    /// for the other.
    g_hull: RingInterval,
}

/// **The area rule both patch lanes use**: a fixed-resolution composite
/// midpoint rule with a first-order Taylor (Lipschitz) pad,
///
/// ```text
/// ∫∫_cell g ∈ A_cell·( g(m) ± [ (h_u/2)·G_u + (h_v/2)·G_v ] ),
///     G_d ⊇ sup_cell |∂_d g|
/// ```
///
/// which needs only that `g` be LIPSCHITZ on the cell — so the kink of
/// `|·|` at a vanishing cross product is covered, and the lane's
/// `|∂_d |c|| ≤ |∂_d c|` bound is exactly the Lipschitz constant.
///
/// The one thing it does NOT cover is a genuine JUMP: at an interior
/// knot of multiplicity ≥ degree the surface is only C⁰ and `S_u`
/// jumps, so `g` jumps and no derivative hull bounds the step. The
/// cells are therefore cut ON the interior knots
/// ([`knot_aligned_cuts`]), which removes the case rather than
/// bounding it: a knot on a cell boundary is in no cell's interior,
/// every cell is a smoothness island, and the Lipschitz pad is valid
/// throughout. The alternative — the smoothness-free hull rule
/// `A_cell·hull(g)` on the straddling cells — is what this rule used
/// to fall back to, and it does not converge: the hull is a
/// control-net fact and those are span-granular, so on a FIXED
/// resolution it is a permanent addend proportional to the off-grid
/// knot count. The flux path takes the same cuts for the same
/// reason.
///
/// Why not the plain hull rule everywhere: every hull in this file is a
/// control-net convexity fact and those are SPAN-GRANULAR, so on a
/// single-span patch the per-cell hull IS the whole-patch hull and the
/// rule never tightens (the unit square came out `[≈0, 10.4]`). Here
/// only the pad carries hulls, so the width is genuinely O(h).
///
/// The area is a certified DENOMINATOR (the +V meter and the extent
/// gate) — `boundary_defect` pads it directly.
fn area_midpoint_taylor<E>(
    rect: (f64, f64, f64, f64),
    n: usize,
    boundary_defect: f64,
    knots: (&[f64], &[f64]),
    mut cell: impl FnMut(AreaBox) -> Result<AreaCell, E>,
) -> Result<RingInterval, E> {
    let (u0, u1, v0, v1) = rect;
    let cuts_u = knot_aligned_cuts(u0, u1, n, knots.0);
    let cuts_v = knot_aligned_cuts(v0, v1, n, knots.1);
    let mut acc = RingInterval::zero();
    for iu in 0..cuts_u.len() - 1 {
        let (c_ulo, c_uhi) = (cuts_u[iu], cuts_u[iu + 1]);
        let hu = c_uhi - c_ulo;
        for iv in 0..cuts_v.len() - 1 {
            let (c_vlo, c_vhi) = (cuts_v[iv], cuts_v[iv + 1]);
            let hv = c_vhi - c_vlo;
            let c = cell(AreaBox {
                ulo: c_ulo,
                uhi: c_uhi,
                umid: c_ulo.midpoint(c_uhi),
                vlo: c_vlo,
                vhi: c_vhi,
                vmid: c_vlo.midpoint(c_vhi),
            })?;
            // The cell width as an ENCLOSURE, not a rounded float:
            // the cells share their cut points, so they tile the
            // rectangle exactly, and the outward subtraction is what
            // keeps the arithmetic over that tiling enclosing too.
            let cell_area = (pt(c_uhi) - pt(c_ulo)) * (pt(c_vhi) - pt(c_vlo));
            let mean = widen(c.g_mid, 0.5 * hu * c.g_u + 0.5 * hv * c.g_v)
                .clamped_to(c.g_hull.lo(), c.g_hull.hi());
            acc = acc + cell_area * mean;
        }
    }
    Ok(widen(acc, boundary_defect))
}

/// Ring lerp `x + (y − x)·λ` (the plan applier's fixed association),
/// componentwise — the rounding lands OUTWARD in the brackets, which
/// is what makes a refined net a certified enclosure of the same
/// patch rather than a re-approximation of it.
fn ring_lerp(x: RVec3, y: RVec3, lambda: f64) -> RVec3 {
    let l = pt(lambda);
    [
        x[0] + (y[0] - x[0]) * l,
        x[1] + (y[1] - x[1]) * l,
        x[2] + (y[2] - x[2]) * l,
    ]
}

/// **Certified knot refinement of a bracketed tensor net**, one
/// direction, to a FIXED span target (D9: the schedule is a constant,
/// never data-dependent).
///
/// Why the rational lane needs it and the integral lane does not:
/// every hull this file computes is a control-net convexity fact, and
/// those are **span-granular** — on a single-span patch the hull over
/// a sub-cell IS the hull over the whole patch, so no amount of
/// quadrature refinement tightens a remainder or an area pad. The
/// integral lane escapes through its exact per-span Newton–Cotes rule;
/// a rational integrand has none, so the enclosure has to be tightened
/// where it is actually loose: in the control net. Knot insertion is
/// exact in ℝ (Book §5.2/§5.3 — it changes the representation, never
/// the locus) and the control polygon converges to the patch at
/// O(h²), so `QUAD2_REFINE_SPANS` spans per axis buys roughly two
/// orders on every hull here.
///
/// `None` if the knot algebra refuses (a malformed net) — the caller
/// then refuses typed rather than proceeding on an unrefined net.
fn refine_dir(
    kv: &KnotVector,
    net: &[RVec3],
    nv: usize,
    along_u: bool,
) -> Option<(KnotVector, Vec<RVec3>, usize)> {
    let poison = [
        RingInterval::poison(),
        RingInterval::poison(),
        RingInterval::poison(),
    ];
    let count = kv.control_count();
    if count == 0 || nv == 0 {
        return None;
    }
    // The net is row-major with stride `nv`. On the u pass the refined
    // direction has `count` lines of `nv`; on the v pass it IS the
    // stride, so `count` must BE `nv` and the line count has to divide
    // out EXACTLY. (R1's m2: `net.len() != count * nv && along_u`
    // parses as `(…) && along_u`, so the v pass had no shape check at
    // all and `net.len() / nv` truncated silently.)
    let malformed = if along_u {
        net.len() != count * nv
    } else {
        count != nv || !net.len().is_multiple_of(nv)
    };
    if malformed {
        return None;
    }
    let other = if along_u { nv } else { net.len() / nv };
    let (d0, d1) = kv.domain();
    let mut add: Vec<f64> = Vec::new();
    for k in 1..QUAD2_REFINE_SPANS {
        #[allow(clippy::cast_precision_loss)]
        let t = d0 + (d1 - d0) * (k as f64 / QUAD2_REFINE_SPANS as f64);
        if t > d0 && t < d1 && !kv.knots().contains(&t) {
            add.push(t);
        }
    }
    let plans = geom_core::spline::algebra::refine_plan(kv, &vec![1.0; count], &add).ok()?;
    // Ascending-index fold over the plan chain, then over the lines of
    // this direction (D9).
    let mut cur_kv = kv.clone();
    let mut cur: Vec<Vec<RVec3>> = (0..other)
        .map(|j| {
            (0..count)
                .map(|i| {
                    let idx = if along_u { i * nv + j } else { j * nv + i };
                    net[idx]
                })
                .collect()
        })
        .collect();
    for plan in &plans {
        for line in &mut cur {
            *line = plan.apply_points(line, poison, ring_lerp);
        }
        cur_kv = plan.knots().clone();
    }
    let new_count = cur_kv.control_count();
    let (rows, cols) = if along_u {
        (new_count, other)
    } else {
        (other, new_count)
    };
    let mut out = vec![poison; rows * cols];
    for (j, line) in cur.iter().enumerate() {
        for (i, p) in line.iter().enumerate() {
            let idx = if along_u { i * cols + j } else { j * cols + i };
            out[idx] = *p;
        }
    }
    Some((cur_kv, out, new_count))
}

/// Both directions of [`refine_dir`], u then v.
fn refine_net(
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    net: &[RVec3],
) -> Option<(KnotVector, KnotVector, Vec<RVec3>)> {
    let nv = kv_v.control_count();
    let (ru, net_u, _) = refine_dir(kv_u, net, nv, true)?;
    let (rv, net_uv, _) = refine_dir(kv_v, &net_u, nv, false)?;
    Some((ru, rv, net_uv))
}

/// `f_uu` (or `f_vv`) of the quotient `f = N/w³`, from the six hulls
/// the two ladders supply:
///
/// ```text
/// f_dd = N_dd/w³ − 6·N_d·w_d/w⁴ − 3·N·w_dd/w⁴ + 12·N·w_d²/w⁵
/// ```
///
/// (differentiate `N·w⁻³` twice; the `w_d²` terms combine as
/// `−6 + 18 = 12`). Fixed evaluation order (D9); `w_d²` goes through
/// [`RingInterval::sqr`], never `x*x`.
fn quotient_second(
    n: RingInterval,
    n_d: RingInterval,
    n_dd: RingInterval,
    w: RingInterval,
    w_d: RingInterval,
    w_dd: RingInterval,
) -> RingInterval {
    n_dd / w.powi(3) - pt(6.0) * n_d * w_d / w.powi(4) - pt(3.0) * n * w_dd / w.powi(4)
        + pt(12.0) * n * w_d.sqr() / w.powi(5)
}

/// How close a grid cut may come to a knot before it is dropped
/// instead of minting a hairline cell, in ulps of the trim
/// rectangle's own span.
///
/// A few ulps, because that is the whole width of the defect: the
/// grid point and the knot are describing the same place, and the
/// cell between them is arithmetic noise rather than geometry. It is
/// deliberately NOT a tolerance in the ε sense — no input's meaning
/// depends on it, only whether one redundant subdivision is taken.
const SLIVER_CUT_ULPS: f64 = 8.0;

/// The `QUAD2_HULL_BLOCKS + 1` block boundaries of one direction, as
/// the block loop computes them — shared so the cut list and the
/// block index cannot drift apart.
fn block_edges(lo: f64, hi: f64) -> Vec<f64> {
    (0..=QUAD2_HULL_BLOCKS)
        .map(|b| {
            #[allow(clippy::cast_precision_loss)]
            let f = b as f64 / QUAD2_HULL_BLOCKS as f64;
            lo + (hi - lo) * f
        })
        .collect()
}

/// **The composite's cut list in one direction**: the uniform
/// `pieces` grid, the coarse block boundaries, and the interior knots,
/// merged.
///
/// The knots are the point. An integrand that is only C⁰ at a knot may
/// genuinely JUMP there, and no derivative hull bounds a jump — so a
/// cell holding a knot in its OPEN INTERIOR has no rule better than
/// the smoothness-free `A_cell·hull(f)`, whose width is a control-net
/// fact and therefore span-granular: it stops shrinking once cells are
/// finer than a span, and the enclosure inherits a Θ(1/pieces) floor
/// that halves per round while the cell count quadruples. Cutting ON
/// the knots removes the case rather than bounding it: a knot on a
/// cell BOUNDARY is not in any cell's interior, every cell is a
/// smoothness island, and the midpoint-plus-Taylor rule applies
/// throughout at its full O(h²).
///
/// The block boundaries are in the list for a soundness reason, not a
/// tightness one: each cell reads its remainder hulls from ONE coarse
/// block, which is only valid if the block contains it. Cutting there
/// makes that containment structural rather than an arithmetic
/// coincidence of `pieces` being a multiple of [`QUAD2_HULL_BLOCKS`].
///
/// The result is sorted, deduplicated, and spans exactly `[lo, hi]`,
/// so consecutive cells share a cut point and the cells tile the
/// rectangle with no gap.
///
/// **Near-twin, recorded and deliberately not unified**:
/// `geom_brep::patch_bound::split_points` builds the same concept for
/// the patch-hull lane — a knot-aligned subdivision of a parameter
/// range, with its own sliver guard. The two differ in what else they
/// must carry (this one owes the coarse hull blocks their containment;
/// that one does not) and unifying them is Track R's consolidation
/// ground (C-m/D30, gated behind #723), not this lane's. Whoever takes
/// it should also fold the `inner`-knot-slice expression, which is
/// copied verbatim five times across three crates.
fn knot_aligned_cuts(lo: f64, hi: f64, pieces: usize, knots: &[f64]) -> Vec<f64> {
    // MANDATORY cuts: the rectangle's own ends and every interior
    // knot. These carry the whole smoothness invariant — a knot that
    // is a cut is in no cell's open interior — so nothing may drop
    // one, and `hi` is pushed as ITSELF because `lo + (hi − lo)·1` is
    // a rounded expression that may miss it by an ulp and leave the
    // last sliver of the rectangle outside every cell.
    let mut cuts: Vec<f64> = Vec::with_capacity(pieces + QUAD2_HULL_BLOCKS + knots.len() + 2);
    cuts.push(lo);
    cuts.push(hi);
    cuts.extend(knots.iter().copied().filter(|k| *k > lo && *k < hi));
    cuts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
    cuts.dedup();

    // GRID cuts — the uniform `pieces` grid and the coarse block
    // edges — are conveniences, not invariants: they only subdivide,
    // and a cell that is wider because one was dropped is still
    // inside one smooth piece and still inside its block. So a grid
    // point is taken only when it stands clear of every mandatory cut
    // by `SLIVER`, which is what stops the cut rule minting hairline
    // cells (and hairline coarse blocks) when a knot happens to land
    // an ulp from a grid point.
    //
    // The test is against the MANDATORY set alone, never against
    // other grid points. That is what makes the block-edge list and
    // the cell list agree by construction rather than by an argument
    // about spacing: a block edge is also a `pieces` grid point (both
    // grids are `lo + (hi − lo)·k/n` and `pieces` is a multiple of
    // [`QUAD2_HULL_BLOCKS`]), and it faces the identical predicate in
    // both calls, so it is accepted in both or dropped in both — and
    // every cell therefore still lies inside exactly one block.
    let span = (hi - lo).abs();
    let sliver = span * SLIVER_CUT_ULPS * f64::EPSILON;
    let clear = |t: f64, mandatory: &[f64]| -> bool {
        t > lo && t < hi && mandatory.iter().all(|m| (t - *m).abs() > sliver)
    };
    let mandatory = cuts.clone();
    let mut grid: Vec<f64> = Vec::new();
    for i in 1..pieces {
        #[allow(clippy::cast_precision_loss)]
        let f = i as f64 / pieces as f64;
        let t = lo + (hi - lo) * f;
        if clear(t, &mandatory) {
            grid.push(t);
        }
    }
    for e in block_edges(lo, hi) {
        if clear(e, &mandatory) {
            grid.push(e);
        }
    }
    cuts.extend(grid);
    cuts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(core::cmp::Ordering::Equal));
    cuts.dedup();
    cuts
}

/// **Certified RATIONAL patch contributions** (M8-3) — the same two
/// numbers [`nurbs_patch_face`] certifies, for a patch whose weights
/// are not all 1.
///
/// # The enclosure, derived
///
/// Write the patch as the quotient of its homogeneous nets,
/// `S = A/w` with `A = Σ N_i N_j w_ij P_ij` and `w = Σ N_i N_j w_ij`
/// — **both polynomial**, so both carry the control-hull convexity
/// facts the integral lane already runs on. The quotient rule gives
///
/// ```text
/// S_u×S_v = [w·(A_u×A_v) − w_v·(A_u×A) − w_u·(A×A_v)] / w³
/// ```
///
/// (the `A×A` term vanishes), and dotting with `S = A/w` kills the two
/// triple products that carry `A` twice, leaving the **flux integrand
/// as one polynomial over one polynomial cube**:
///
/// ```text
/// f = S·(S_u×S_v) = A·(A_u×A_v) / w³ = N / w³
/// ```
///
/// `N` is exactly the integral lane's integrand on the homogeneous
/// net, so its derivative hulls are the SAME expressions
/// ([`Ladder::num_uu`]) — the rational extension is a division, not a
/// new geometry. Weights are `f64` structure and strictly positive
/// (checked exactly, C6), so `w`'s control hull excludes zero on every
/// cell and the ring division is defined; a hull that does not is
/// poison, and poison refuses typed rather than answering wide.
///
/// # The rule, the remainder, and why there is no exact lane
///
/// A rational integrand has **no finite exact quadrature rule** — the
/// integral-lane's per-span Newton–Cotes shortcut has no rational
/// counterpart and none is claimed. The honest deliverable is the
/// composite midpoint enclosure over a FIXED schedule (D9: pieces
/// double per round from [`QUAD2_INIT_PIECES`], at most
/// [`QUAD2_MAX_ROUNDS`] rounds — never a data-dependent iteration),
/// with the two 1-D Taylor remainders per cell:
///
/// ```text
/// ∫∫_cell f ∈ A_cell·f(m) + hull(f_uu)·h_u³h_v/24 + hull(f_vv)·h_u h_v³/24
/// ```
///
/// The `f_dd` hulls come from [`quotient_second`].
///
/// **The cells are cut ON the interior knots**
/// ([`knot_aligned_cuts`]), which is what makes that ONE rule the
/// whole rule: the Taylor remainder needs `f` to be twice
/// differentiable across the cell, and at an interior knot it need not
/// even be continuous. A uniform cut has to fall back to the
/// smoothness-free `A_cell·hull(f)` wherever a knot lands inside a
/// cell, and that hull is a control-net fact — span-granular, so it
/// stops shrinking once cells are finer than a span. Each straddling
/// knot line then contributes Θ(1/pieces): it halves per round while
/// the cell count quadruples, so the enclosure has a FLOOR and the
/// fixed schedule cannot reach a target below it. Aligning the cuts
/// removes the case instead of bounding it — a knot on a cell boundary
/// is in no cell's interior — and the enclosure is O(h²) throughout.
/// (The refinement to [`QUAD2_REFINE_SPANS`] does not do this on its
/// own: inserted knots do not change smoothness, so the ORIGINAL knot
/// vectors are what the cuts are taken from.) Acceptance goes through the EXISTING named
/// predicates (`props_quad_converged`, `props_quad_face_extent`) with
/// the existing meter — the enclosure width as a LENGTH,
/// `width(flux)/(3·area_mid)`, against `QUAD_TARGET_LEN_FACTOR·ε` —
/// so this lane adds no k-census row of its own. Budget exhaustion is
/// [`PropsError::QuadratureBudget`] carrying the measured width; a
/// wide answer is never returned silently.
///
/// # The practical envelope (measured; every row re-measured)
///
/// The rule is O(h²) and the meter is a **LENGTH**, so the achievable
/// width scales roughly linearly with part size while the target does
/// not: the frontier is a part-SIZE frontier, and metre-scale parts sit
/// ON it rather than comfortably inside. The refinement schedule is
/// fixed (D9), so each carrier bottoms out at the same displacement on
/// every ε row and only the `1024·ε` target moves. Measured floors
/// (`crates/geom-brep/tests/review_r1_rational_probes.rs`, which pins
/// each one), against the DEFAULT target of 1.024e-6 m:
///
/// ```text
/// carrier                                      floor (m)   @ ε = 1e-9
/// 1 µm quarter cylinder                          <1e-9     certifies*
/// 1 m × 2 m quarter cylinder, single span       1.533e-7   certifies (6.7×)
/// unit sphere octant (degenerate pole row)      9.683e-7   certifies (1.06×)
/// quarter torus, R = 2 m, r = 0.5 m             1.146e-6   REFUSES (1.12× over)
/// 1 m × 2 m half cylinder, TWO spans            1.304e-6   REFUSES (1.27× over)
/// 5 m C0-kink extruded wall                     4.785e-4   REFUSES
/// 1 km quarter cylinder                         1.533e-4   REFUSES
/// warped bilinear, weights 1e-1/1e1             2.363e-3   REFUSES
/// Möbius quarter cylinder, weight ratio 100     2.878e-3   REFUSES
/// bilinear square, weights 1e-3/1e3             3.415e+6   REFUSES
/// ```
///
/// \* Convergence is never the small end's problem — its floor scales
/// with the part. The FACE-EXTENT gate is: at ε = 1e-6 the 1 µm
/// cylinder's 0.22 µm mean width is under the run's own tolerance and
/// the same face is refused [`PropsError::DegenerateFace`]. That gate
/// is older than the meter guard and behaves identically with and
/// without it.
///
/// Read that honestly: a plain 1 m × 2 m half cylinder written as the
/// standard TWO-span quadratic — an ordinary part, not an adversarial
/// one — misses the default ε by 27%, and the sphere octant clears it
/// by 6%. The single-span twin of the same cylinder passes with 6.7×,
/// so "metre-scale parts converge" holds only for the simplest
/// spanning; splitting the same locus into two spans is enough to
/// cross the line. Callers on large, multi-span, or extreme-weight
/// parts should expect refusals rather than answers.
///
/// The last three rows are the AREA's failure, not the flux's, and the
/// last one is the extreme of it: with a 1e-3/1e3 weight spread the
/// area rule's symmetric Lipschitz pad runs to ~1.7e20 against a true
/// area of 1 m². What keeps that from straddling zero — and so from
/// refusing a plain unit square as having no extent at all — is that
/// the rule intersects the pad with the cell's own hull of `g`, whose
/// lower end is a magnitude and cannot be negative. The face therefore
/// certifies a positive extent and refuses with a measured width. The
/// width is useless; saying so is the point.
///
/// Every other refusal is a typed [`PropsError::QuadratureBudget`]
/// carrying its measured width. The next levers, in order, are a
/// higher-order rule (Simpson needs `A` to fifth derivatives), a
/// tighter area pad (the symmetric Lipschitz pad is what puts the
/// extreme-weight rows out of reach — it is the AREA, not the flux,
/// that fails there), a finer hull-block grid, and the round budget
/// itself: with the cells knot-aligned the enclosure quarters cleanly
/// per round, so a carrier under a factor of two over target is
/// exactly one round short.
///
/// The `w`-uniform-in-v fast path is no longer a lever — it is taken,
/// below — but what it buys is worth stating, because it is not what
/// the note proposing it expected. It makes the v integral EXACT and
/// the v cell count O(1), so it is a large COST win and it removes
/// the v direction from the error entirely; on the carriers that
/// satisfy it (cylinders, extruded and skinned walls) the curvature
/// is all in u and the v remainder was already near zero, so the
/// WIDTH barely moves. Its hypothesis is also narrower than "loft
/// walls satisfy it": geometrically they do, but a skin fit of degree
/// ≥ 2 SOLVES for its weights and the solve returns them equal only
/// to rounding. The test is exact `f64` structure (C6) — the arm's
/// exactness is a soundness hypothesis, not a tolerance — so those
/// walls take the composite arm.
///
/// # Errors
///
/// [`PropsError`] — non-positive or non-finite weights, a weight hull
/// that does not exclude zero, an escalated funnel decision, a
/// degenerate face, or the typed budget refusal.
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
#[allow(clippy::too_many_lines)] // one engine, kept whole like the integral lane
fn rational_patch_face<T: Decide>(
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    control: &[RVec3],
    weights: &[f64],
    rect: (f64, f64, f64, f64),
    perimeter: f64,
    boundary_defect: f64,
    eps: f64,
    band: Band,
) -> Result<FaceCutBounds, PropsError> {
    let (u0, u1, v0, v1) = rect;
    if weights.len() != control.len() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a rational patch whose weight count does not match its control net",
        });
    }
    // Exact `f64` structure (C6): the rational hull property IS the
    // positive-weight hypothesis, so a non-positive weight is not a
    // tolerance question but a refusal.
    if weights.iter().any(|w| *w <= 0.0 || !w.is_finite()) {
        return Err(PropsError::QuadratureUnsupported {
            what: "a rational patch with a non-positive or non-finite weight: the \
                   convex-hull property (and with it every enclosure here) needs \
                   strictly positive weights",
        });
    }
    // The homogeneous position net `A = w·P` and the weight net.
    let mut a_net: Vec<RVec3> = Vec::with_capacity(control.len());
    let mut w_net: Vec<RVec3> = Vec::with_capacity(control.len());
    for (c, w) in control.iter().zip(weights) {
        let rw = pt(*w);
        a_net.push([rw * c[0], rw * c[1], rw * c[2]]);
        w_net.push([rw, RingInterval::zero(), RingInterval::zero()]);
    }
    // Certified refinement FIRST (fn docs): every hull below is a
    // control-net fact, and only the net can tighten them.
    let refuse_refine = PropsError::QuadratureUnsupported {
        what: "a rational patch whose bracketed net would not refine (malformed knot \
               structure) — the enclosure's hulls have no other lever",
    };
    let (r_u, r_v, a_net) = refine_net(kv_u, kv_v, &a_net).ok_or(refuse_refine.clone())?;
    let (_, _, w_net) = refine_net(kv_u, kv_v, &w_net).ok_or(refuse_refine)?;
    let a = Ladder::build(&r_u, &r_v, &a_net);
    let w = Ladder::build(&r_u, &r_v, &w_net);

    // The CUT lists come from the ORIGINAL knot vectors: refinement's
    // inserted knots are artificial (the locus and its smoothness are
    // unchanged), so cutting on them would only cost cells. What the
    // cells must land on is where the smoothness actually breaks.
    let interior = |kv: &KnotVector, lo: f64, hi: f64| -> Vec<f64> {
        kv.knots()
            .iter()
            .copied()
            .filter(|k| *k > lo && *k < hi)
            .collect()
    };
    let knots_u = interior(kv_u, u0, u1);
    let knots_v = interior(kv_v, v0, v1);

    let over_all = (Collapse::Over(u0, u1), Collapse::Over(v0, v1));
    let g_w = w.chan(over_all.0, over_all.1);
    if g_w.lo() <= 0.0 || !g_w.lo().is_finite() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a rational patch whose weight-function hull does not exclude zero \
                   over the trim rectangle — the quotient's enclosures are undefined",
        });
    }
    // Position magnitude bound over the rectangle: `S = A/w`, so the
    // homogeneous hull over the weight hull bounds it (1-norm
    // over-bound of the 2-norm — only an upper bound is consumed).
    let a_hull = a.a.vec(over_all.0, over_all.1);
    let p_bound = (a_hull[0] / g_w).mag() + (a_hull[1] / g_w).mag() + (a_hull[2] / g_w).mag();

    // The Taylor remainders' `f_dd` hulls, on a FIXED coarse block
    // grid (D9) rather than one whole-rectangle hull. Sound because a
    // hull over a superset contains every sub-cell's own; MUCH tighter
    // because the quotient's dependency widening (five `w`-power
    // divisions per term) shrinks with the region.
    //
    // The block edges are KNOT-ALIGNED for the same reason the cells
    // are ([`knot_aligned_cuts`]), and it is the same defect: `f_dd`
    // is where the integrand's smoothness actually lives, so a block
    // spanning an interior knot hulls a function that jumps inside it
    // and hands every cell in that block the jump's width. Aligning
    // the blocks keeps each hull inside one smooth piece. Built once,
    // before the rounds; the cut list at every refinement level
    // contains these edges, so each block still owns whole cells.
    let edges_u = knot_aligned_cuts(u0, u1, QUAD2_HULL_BLOCKS, &knots_u);
    let edges_v = knot_aligned_cuts(v0, v1, QUAD2_HULL_BLOCKS, &knots_v);
    let (nbu, nbv) = (edges_u.len() - 1, edges_v.len() - 1);
    let mut blocks: Vec<(RingInterval, RingInterval)> = Vec::with_capacity(nbu * nbv);
    for bu in 0..nbu {
        let (b_ulo, b_uhi) = (edges_u[bu], edges_u[bu + 1]);
        for bv in 0..nbv {
            let (b_vlo, b_vhi) = (edges_v[bv], edges_v[bv + 1]);
            let o = (Collapse::Over(b_ulo, b_uhi), Collapse::Over(b_vlo, b_vhi));
            let (n, bw) = (a.num(o.0, o.1), w.chan(o.0, o.1));
            blocks.push((
                quotient_second(
                    n,
                    a.num_u(o.0, o.1),
                    a.num_uu(o.0, o.1),
                    bw,
                    w.chan_u(o.0, o.1),
                    w.chan_uu(o.0, o.1),
                ),
                quotient_second(
                    n,
                    a.num_v(o.0, o.1),
                    a.num_vv(o.0, o.1),
                    bw,
                    w.chan_v(o.0, o.1),
                    w.chan_vv(o.0, o.1),
                ),
            ));
        }
    }

    // AREA: the shared [`area_midpoint_taylor`] rule with this lane's
    // quotient integrand `g = |cross_num|/w³` and its pad
    //
    //     G_d ⊇ sup |∂_d g| ≤ |∂_d cross_num|/w³ + 3·|cross_num|·|w_d|/w⁴
    //
    // (`| ∂_d |c| | ≤ |∂_d c|` — the magnitude is 1-Lipschitz).
    let area = area_midpoint_taylor(
        rect,
        QUAD2_AREA_PIECES,
        boundary_defect,
        (&knots_u, &knots_v),
        |b| {
            let over = (Collapse::Over(b.ulo, b.uhi), Collapse::Over(b.vlo, b.vhi));
            let m = (Collapse::At(b.umid), Collapse::At(b.vmid));
            let cm = a.cross_num(&w, m.0, m.1);
            let wm = w.chan(m.0, m.1);
            let g_mid = sqrt_enclosure(cm[0].sqr() + cm[1].sqr() + cm[2].sqr()) / wm.powi(3);
            let wh = w.chan(over.0, over.1);
            if wh.lo() <= 0.0 || !wh.lo().is_finite() {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a rational patch cell whose weight hull does not exclude \
                           zero — the quotient's enclosures are undefined there",
                });
            }
            let (w3, w4) = (wh.lo().powi(3), wh.lo().powi(4));
            let ch = a.cross_num(&w, over.0, over.1);
            let c_hi = norm_hi(ch);
            let pad_d = |dc: RVec3, wd: RingInterval| -> f64 {
                norm_hi(dc) / w3 + 3.0 * c_hi * wd.mag() / w4
            };
            Ok(AreaCell {
                g_mid,
                g_u: pad_d(a.cross_num_u(&w, over.0, over.1), w.chan_u(over.0, over.1)),
                g_v: pad_d(a.cross_num_v(&w, over.0, over.1), w.chan_v(over.0, over.1)),
                g_hull: sqrt_enclosure(ch[0].sqr() + ch[1].sqr() + ch[2].sqr()) / wh.powi(3),
            })
        },
    )?;

    // **The `w`-uniform-in-v arm.** With the weights constant along v
    // the quotient's denominator leaves the v integral entirely —
    // `f = N(u,v)/w(u)³` — and `N` is a polynomial there, so the v
    // integral is EXACT per knot span: the true analogue of
    // [`patch_flux_exact`], available to the rational lane on the
    // patches that satisfy the hypothesis (loft and sweep walls, whose
    // weights come from the profile direction only, and the rational
    // cylinder walls a STEP file states the same way).
    //
    // Two things follow, and the second is the surprise. The v Taylor
    // remainder is gone, which is the tightening. And subdividing in v
    // no longer buys anything: the surviving remainder is
    // `hull(f_uu)·h_u³·h_v/24` with `hull(f_uu)` a per-BLOCK quantity,
    // so splitting a v-cell inside its block leaves the sum over that
    // block unchanged, and the exact v integrals over the pieces sum
    // to the exact integral over the whole. The v cuts are therefore
    // the block edges and the interior knots ONLY, and the cost drops
    // from `pieces²` cells to `pieces × (blocks + knots)`.
    //
    // The hypothesis is exact `f64` structure (C6), read off the
    // caller's weight net: nothing here is a tolerance question.
    let nv_in = kv_v.control_count();
    let w_uniform_in_v = nv_in > 0
        && weights.len().is_multiple_of(nv_in)
        && weights
            .chunks_exact(nv_in)
            .all(|row| row.iter().all(|x| *x == row[0]));
    let nc_v = if w_uniform_in_v {
        newton_cotes_weights(3 * r_v.degree())
    } else {
        None
    };

    let target_len = QUAD_TARGET_LEN_FACTOR * eps;
    let mut pieces = QUAD2_INIT_PIECES;
    // INVARIANT: every round assigns this from
    // `mean_boundary_displacement`, which returns only finite
    // lengths — so the budget refusal below carries a finite width
    // by construction (the `eps_posture` contract the suites pin).
    let mut last_width_len = f64::NAN;
    for round in 0..=QUAD2_RATIONAL_MAX_ROUNDS {
        let mut flux = RingInterval::zero();
        let cuts_u = knot_aligned_cuts(u0, u1, pieces, &knots_u);
        let cuts_v = knot_aligned_cuts(v0, v1, if nc_v.is_some() { 1 } else { pieces }, &knots_v);
        let mut bu = 0usize;
        for iu in 0..cuts_u.len() - 1 {
            let (c_ulo, c_uhi) = (cuts_u[iu], cuts_u[iu + 1]);
            // The block a cell reads its `f_dd` hulls from must
            // CONTAIN it (a hull over a superset contains every
            // sub-cell's own). The block edges are cut points, so
            // advancing while the cell starts past the current block's
            // top lands on the block that does.
            while bu + 1 < nbu && c_ulo >= edges_u[bu + 1] {
                bu += 1;
            }
            // The width as an ENCLOSURE of the true cell width, not a
            // rounded float: the cells tile the rectangle exactly
            // (consecutive cells share a cut point), and the outward
            // subtraction is what keeps the tiling's arithmetic
            // enclosing too.
            let hu = pt(c_uhi) - pt(c_ulo);
            let slice = a.flux_slice(&w, Collapse::At(c_ulo.midpoint(c_uhi)));
            // `w` does not depend on v under the exact arm, so its
            // cube is a per-COLUMN quantity there.
            let w3 = nc_v
                .as_ref()
                .map(|_| w.a.at_v(&slice.w, Collapse::At(v0.midpoint(v1)))[0].powi(3));
            let mut bv = 0usize;
            for iv in 0..cuts_v.len() - 1 {
                let (c_vlo, c_vhi) = (cuts_v[iv], cuts_v[iv + 1]);
                while bv + 1 < nbv && c_vlo >= edges_v[bv + 1] {
                    bv += 1;
                }
                let hv = pt(c_vhi) - pt(c_vlo);
                let (b_uu, b_vv) = blocks[bu * nbv + bv];
                let r_uu = b_uu * (hu * hu.sqr() * hv / pt(24.0));
                match (nc_v.as_ref(), w3) {
                    // The exact arm: `h_u·g(u_m) + hull(g'')·h_u³/24`
                    // with `g(u) = ∫_cell f(u,·)` taken exactly and
                    // `g'' = ∫_cell f_uu ⊆ h_v·hull(f_uu)`, which is
                    // the SAME `r_uu` the midpoint arm carries.
                    (Some(nc), Some(cube)) => {
                        flux = flux
                            + hu * (a.num_v_exact(&slice, &r_v, c_vlo, c_vhi, nc) / cube)
                            + r_uu;
                    }
                    _ => {
                        let fm = a.integrand_at(&w, &slice, Collapse::At(c_vlo.midpoint(c_vhi)));
                        flux = flux + hu * hv * fm + r_uu + b_vv * (hu * hv * hv.sqr() / pt(24.0));
                    }
                }
            }
        }
        let flux = widen(flux, boundary_defect * p_bound);
        if flux.is_poison() || area.is_poison() {
            return Err(PropsError::QuadratureUnsupported {
                what: "a rational patch enclosure poisoned (a weight hull straddling \
                       zero, or a non-finite net) — refusing rather than answering wide",
            });
        }
        let width_len = mean_boundary_displacement(flux, area)?;
        last_width_len = width_len;
        if classify_len::<T>(
            "props_quad_converged",
            Margin::of(target_len - width_len),
            band,
        )? == Sign::Positive
        {
            match classify_len::<T>(
                "props_quad_face_extent",
                Margin::over_lever(area.lo(), perimeter),
                band,
            )? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(PropsError::DegenerateFace),
            }
            return Ok(FaceCutBounds { flux, area });
        }
        if round < QUAD2_RATIONAL_MAX_ROUNDS {
            pieces *= 2;
        }
    }
    Err(PropsError::QuadratureBudget {
        width_len: last_width_len,
        target_len,
    })
}

/// **Certified NURBS-patch contributions** (M6-3 Leg C): the
/// chart-normal volume flux `∫∫ S·(S_u×S_v) du dv` and the patch area
/// `∫∫ |S_u×S_v| du dv` of a patch over the exact UV
/// rectangle `rect = (u0, u1, v0, v1)` — the loft/sweep wall case,
/// where the face's stored iso-line pcurves pin the trim region to a
/// rectangle exactly.
///
/// The flux rides the 2-D midpoint composite with hull-bounded
/// second-derivative remainders per axis (module docs' rule, iterated:
/// `∫∫_cell f ∈ A·f(m) + hull(f_uu)·h_u³h_v/24 + hull(f_vv)·h_u h_v³/24`,
/// no cross term — the two 1-D remainders compose), with
///
/// ```text
/// f_uu = S_u·(S_uu×S_v) + S·(S_uuu×S_v) + 2·S·(S_uu×S_uv) + S·(S_u×S_uuv)
/// f_vv = S_v·(S_u×S_vv) + S·(S_uvv×S_v) + 2·S·(S_uv×S_vv) + S·(S_u×S_vvv)
/// ```
///
/// (the `S_u·(S_u×S_uv)`-shaped triples vanish identically). Cells
/// are cut ON the interior knots ([`knot_aligned_cuts`]), so each one
/// lies inside a single smooth piece and the Taylor remainder is the
/// whole error there. The area rides the
/// shared [`area_midpoint_taylor`] rule (midpoint + Lipschitz pad on
/// the same knot-aligned cells) — sound at every
/// resolution and O(h), because the +V gate meters `V/A` and needs an
/// honest denominator. A patch with non-unit weights
/// routes to [`rational_patch_face`] (M8-3), which certifies the same
/// two numbers through the quotient rule.
///
/// Honesty pads (both directions accounted): `boundary_defect` is the
/// caller's `Σ (metric edge length)·(pcurve envelope)` bound on the
/// metric area between the true trim boundary and the rectangle; it
/// widens the area directly and the flux through the patch's position
/// magnitude bound. Exactly zero for minted (exact-in-family) iso
/// pcurves.
///
/// The flux is signed by the CHART normal `S_u×S_v`; the caller owns
/// the loop-winding sign (S10: winding-derived end to end).
///
/// # Errors
///
/// [`PropsError`] — an escalated funnel decision, a degenerate face,
/// the rational lane's own refusals, or the typed
/// [`PropsError::QuadratureBudget`] refusal.
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
#[allow(clippy::too_many_lines)] // one engine, kept whole like the cylinder lane
pub fn nurbs_patch_face<T: Decide>(
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    control: &[RVec3],
    weights: &[f64],
    rect: (f64, f64, f64, f64),
    perimeter: f64,
    boundary_defect: f64,
    eps: f64,
    band: Band,
) -> Result<FaceCutBounds, PropsError> {
    let (u0, u1, v0, v1) = rect;
    if !(u1 - u0).is_finite() || u1 <= u0 || !(v1 - v0).is_finite() || v1 <= v0 {
        return Err(PropsError::QuadratureUnsupported {
            what: "empty or non-finite UV rectangle",
        });
    }
    // M8-3: the rational patch is the SAME integrand over a polynomial
    // cube (`f = N/w³`), so it takes its own lane rather than refusing.
    if weights.iter().any(|w| *w != 1.0) {
        return rational_patch_face::<T>(
            kv_u,
            kv_v,
            control,
            weights,
            rect,
            perimeter,
            boundary_defect,
            eps,
            band,
        );
    }
    // The derivative grids, built once (mixed partials commute on
    // spline nets exactly).
    let s = PatchGrid::base(kv_u, kv_v, control);
    let su = s.deriv_u();
    let sv = s.deriv_v();
    let suu = su.as_ref().and_then(PatchGrid::deriv_u);
    let suv = su.as_ref().and_then(PatchGrid::deriv_v);
    let svv = sv.as_ref().and_then(PatchGrid::deriv_v);
    let suuu = suu.as_ref().and_then(PatchGrid::deriv_u);
    let suuv = suu.as_ref().and_then(PatchGrid::deriv_v);
    let suvv = suv.as_ref().and_then(PatchGrid::deriv_v);
    let svvv = svv.as_ref().and_then(PatchGrid::deriv_v);
    let interior = |kv: &KnotVector, lo: f64, hi: f64| -> Vec<f64> {
        kv.knots()
            .iter()
            .copied()
            .filter(|k| *k > lo && *k < hi)
            .collect()
    };
    let knots_u = interior(kv_u, u0, u1);
    let knots_v = interior(kv_v, v0, v1);
    // Position magnitude bound over the rectangle (the flux pad's
    // lever): a 1-norm over-bound of the 2-norm suffices — only an
    // upper bound is consumed.
    let s_hull = s.vec(Collapse::Over(u0, u1), Collapse::Over(v0, v1));
    let p_bound = s_hull[0].mag() + s_hull[1].mag() + s_hull[2].mag();

    let target_len = QUAD_TARGET_LEN_FACTOR * eps;
    let mut pieces = QUAD2_INIT_PIECES;
    // INVARIANT: every round assigns this from
    // `mean_boundary_displacement`, which returns only finite
    // lengths — so the budget refusal below carries a finite width
    // by construction (the `eps_posture` contract the suites pin).
    let mut last_width_len = f64::NAN;
    // GLOBAL second-derivative hulls, computed ONCE (the whole-rect
    // hull contains every cell's, so the per-cell remainder
    // `hull(f_uu)·h³/24` may use it soundly — looser by a constant,
    // still O(h²), and it turns the per-cell cost from ten grid hulls
    // into three thin evals; the 866-second debug wall this replaced
    // is the reason). It bounds the remainder on every cell because
    // the cells are cut on the interior knots, so each one lies inside
    // a single smooth piece and the Taylor remainder is the whole
    // error there.
    let over_all = (Collapse::Over(u0, u1), Collapse::Over(v0, v1));
    let g_s = s_hull;
    let g_su = grid_vec(su.as_ref(), over_all.0, over_all.1);
    let g_sv = grid_vec(sv.as_ref(), over_all.0, over_all.1);
    let g_suu = grid_vec(suu.as_ref(), over_all.0, over_all.1);
    let g_suv = grid_vec(suv.as_ref(), over_all.0, over_all.1);
    let g_svv = grid_vec(svv.as_ref(), over_all.0, over_all.1);
    let g_f_uu = rv_dot(g_su, rv_cross(g_suu, g_sv))
        + rv_dot(
            g_s,
            rv_cross(grid_vec(suuu.as_ref(), over_all.0, over_all.1), g_sv),
        )
        + pt(2.0) * rv_dot(g_s, rv_cross(g_suu, g_suv))
        + rv_dot(
            g_s,
            rv_cross(g_su, grid_vec(suuv.as_ref(), over_all.0, over_all.1)),
        );
    let g_f_vv = rv_dot(g_sv, rv_cross(g_su, g_svv))
        + rv_dot(
            g_s,
            rv_cross(grid_vec(suvv.as_ref(), over_all.0, over_all.1), g_sv),
        )
        + pt(2.0) * rv_dot(g_s, rv_cross(g_suv, g_svv))
        + rv_dot(
            g_s,
            rv_cross(g_su, grid_vec(svvv.as_ref(), over_all.0, over_all.1)),
        );

    // AREA: the shared [`area_midpoint_taylor`] rule (#313's integral
    // half) with this lane's polynomial integrand `g = |S_u×S_v|` —
    // the rational lane's rule at `w ≡ 1`, where the quotient pad
    // collapses to `|∂_d (S_u×S_v)|` exactly. It replaces the plain
    // per-cell hull rule, which was span-granular and therefore did
    // not tighten at all: on the standard multi-arc form it returned
    // `area.lo() == 0` and the face was refused DegenerateFace.
    let area = area_midpoint_taylor(
        rect,
        QUAD2_AREA_PIECES,
        boundary_defect,
        (&knots_u, &knots_v),
        |b| -> Result<AreaCell, PropsError> {
            let over = (Collapse::Over(b.ulo, b.uhi), Collapse::Over(b.vlo, b.vhi));
            let m = (Collapse::At(b.umid), Collapse::At(b.vmid));
            let cm = rv_cross(
                grid_vec(su.as_ref(), m.0, m.1),
                grid_vec(sv.as_ref(), m.0, m.1),
            );
            let (h_su, h_sv) = (
                grid_vec(su.as_ref(), over.0, over.1),
                grid_vec(sv.as_ref(), over.0, over.1),
            );
            // ∂_u (S_u×S_v) = S_uu×S_v + S_u×S_uv, and likewise in v.
            let d_u = rv_add(
                rv_cross(grid_vec(suu.as_ref(), over.0, over.1), h_sv),
                rv_cross(h_su, grid_vec(suv.as_ref(), over.0, over.1)),
            );
            let d_v = rv_add(
                rv_cross(grid_vec(suv.as_ref(), over.0, over.1), h_sv),
                rv_cross(h_su, grid_vec(svv.as_ref(), over.0, over.1)),
            );
            Ok(AreaCell {
                g_mid: sqrt_enclosure(cm[0].sqr() + cm[1].sqr() + cm[2].sqr()),
                g_u: norm_hi(d_u),
                g_v: norm_hi(d_v),
                g_hull: {
                    let c = rv_cross(h_su, h_sv);
                    sqrt_enclosure(c[0].sqr() + c[1].sqr() + c[2].sqr())
                },
            })
        },
    )?;

    // ---- The exact per-span lane first (fn docs): one tensor
    // Newton–Cotes pass whose enclosure width is ring rounding only.
    // The composite rounds below are the fallback for degrees outside
    // the rule window (> 4 per direction). ----
    if let Some(exact) =
        patch_flux_exact(&s, su.as_ref(), sv.as_ref(), kv_u, kv_v, (u0, u1, v0, v1))
    {
        let flux = widen(exact, boundary_defect * p_bound);
        let width_len = mean_boundary_displacement(flux, area)?;
        if classify_len::<T>(
            "props_quad_converged",
            Margin::of(target_len - width_len),
            band,
        )? == Sign::Positive
        {
            match classify_len::<T>(
                "props_quad_face_extent",
                Margin::over_lever(area.lo(), perimeter),
                band,
            )? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(PropsError::DegenerateFace),
            }
            return Ok(FaceCutBounds { flux, area });
        }
        // An exact-lane enclosure missing the target can only be pad-
        // or degeneracy-dominated; refinement cannot do better —
        // refuse at the budget with the honest width.
        return Err(PropsError::QuadratureBudget {
            width_len,
            target_len,
        });
    }

    for round in 0..=QUAD2_MAX_ROUNDS {
        let mut flux = RingInterval::zero();
        // Knot-aligned cells, for the reason [`knot_aligned_cuts`]
        // gives: this lane reaches the composite only where the exact
        // rule cannot run, and a cell holding an interior knot has no
        // rule but the smoothness-free hull — span-granular, and
        // therefore a floor rather than a remainder.
        let cuts_u = knot_aligned_cuts(u0, u1, pieces, &knots_u);
        let cuts_v = knot_aligned_cuts(v0, v1, pieces, &knots_v);
        for iu in 0..cuts_u.len() - 1 {
            let (c_ulo, c_uhi) = (cuts_u[iu], cuts_u[iu + 1]);
            let hu = pt(c_uhi) - pt(c_ulo);
            for iv in 0..cuts_v.len() - 1 {
                let (c_vlo, c_vhi) = (cuts_v[iv], cuts_v[iv + 1]);
                let hv = pt(c_vhi) - pt(c_vlo);
                let m = (
                    Collapse::At(c_ulo.midpoint(c_uhi)),
                    Collapse::At(c_vlo.midpoint(c_vhi)),
                );
                let fm = rv_dot(
                    s.vec(m.0, m.1),
                    rv_cross(
                        grid_vec(su.as_ref(), m.0, m.1),
                        grid_vec(sv.as_ref(), m.0, m.1),
                    ),
                );
                flux = flux
                    + hu * hv * fm
                    + g_f_uu * (hu * hu.sqr() * hv / pt(24.0))
                    + g_f_vv * (hu * hv * hv.sqr() / pt(24.0));
            }
        }
        let flux = widen(flux, boundary_defect * p_bound);
        // Convergence: the shared mean-boundary-displacement meter.
        let width_len = mean_boundary_displacement(flux, area)?;
        last_width_len = width_len;
        if classify_len::<T>(
            "props_quad_converged",
            Margin::of(target_len - width_len),
            band,
        )? == Sign::Positive
        {
            match classify_len::<T>(
                "props_quad_face_extent",
                Margin::over_lever(area.lo(), perimeter),
                band,
            )? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(PropsError::DegenerateFace),
            }
            return Ok(FaceCutBounds { flux, area });
        }
        if round < QUAD2_MAX_ROUNDS {
            pieces *= 2;
        }
    }
    Err(PropsError::QuadratureBudget {
        width_len: last_width_len,
        target_len,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// The corpus's declared ambient uncertainty — the ε the fixed
    /// quadrature schedule (D9) is dimensioned for, and the boundary
    /// the ε-row postures below are pinned against.
    const CORPUS_EPS: f64 = 1e-9;

    /// The three HONEST outcomes of an ε-coupled convergence row.
    ///
    /// The target is `QUAD_TARGET_LEN_FACTOR·ε` while the refinement
    /// schedule is FIXED (D9), so the same patch that certifies at
    /// ε = 1e-6 genuinely cannot at ε = 1e-12: the target is a million
    /// times tighter and the rounds do not grow. A row asserting `Ok`
    /// unconditionally is ε-BLIND, not strict — it fails on the hosted
    /// ε matrix for a reason that is the lane working correctly.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum EpsPosture {
        /// The enclosure certified — it must CONTAIN the truth.
        Certified,
        /// [`PropsError::QuadratureBudget`]: the fixed schedule's floor
        /// is DEFINITELY above the run's target.
        Budget,
        /// The same shortfall, in-band for the run's `Band{ε, Kε}` — so
        /// `props_quad_converged` escalates through the funnel before
        /// the budget can run out. The two-tolerance twin the
        /// [`PropsError::QuadratureBudget`] docs name.
        Escalated,
    }

    /// Assert the ε-row posture of one patch outcome and return it.
    ///
    /// Every arm PINS its live signature so a row can never be green
    /// for the wrong reason:
    ///
    /// - certified ⇒ both enclosures contain their truths (this is the
    ///   arm that catches a tight-but-WRONG bracket, the M8-3 MAJOR);
    /// - budget ⇒ the carried `width_len` really did exceed
    ///   `target_len`, AND that target IS `1024·ε` for this run;
    /// - escalated ⇒ the predicate is `props_quad_converged` and
    ///   nothing else, with an in-band margin.
    ///
    /// Any other error is an unsound outcome and panics. Nothing here
    /// widens a target, loosens a schedule, or special-cases ε into
    /// certifying.
    fn eps_posture(
        row: &str,
        out: &Result<FaceCutBounds, PropsError>,
        truth_flux: f64,
        truth_area: f64,
        slack: f64,
    ) -> EpsPosture {
        let target = QUAD_TARGET_LEN_FACTOR * Tol::witness().get().eps;
        match out {
            Ok(b) => {
                for (what, encl, truth) in
                    [("flux", b.flux, truth_flux), ("area", b.area, truth_area)]
                {
                    assert!(
                        encl.lo() - slack <= truth && truth <= encl.hi() + slack,
                        "{row}: {what} {encl:?} EXCLUDES the truth {truth}"
                    );
                }
                EpsPosture::Certified
            }
            Err(PropsError::QuadratureBudget {
                width_len,
                target_len,
            }) => {
                assert!(
                    width_len.is_finite() && width_len > target_len,
                    "{row}: a budget refusal must carry a width that really missed: \
                     {width_len:e} vs {target_len:e}"
                );
                assert!(
                    (target_len - target).abs() <= target * 1e-12,
                    "{row}: the refused target must BE 1024·ε for this run: \
                     {target_len:e} vs {target:e}"
                );
                EpsPosture::Budget
            }
            Err(PropsError::Escalated { cause }) => {
                assert_eq!(
                    cause.predicate,
                    Some("props_quad_converged"),
                    "{row}: only the convergence predicate may escalate here: {cause:?}"
                );
                assert!(
                    matches!(cause.margin, geom_core::MarginDiag::Value(m) if m.is_finite()),
                    "{row}: the escalation must carry a finite in-band margin: {cause:?}"
                );
                EpsPosture::Escalated
            }
            Err(e) => panic!("{row}: unsound outcome {e:?}"),
        }
    }

    fn chan(c0: f64, ca: f64, cb: f64, cl: f64) -> HarmChan {
        HarmChan {
            c0: pt(c0),
            ca: pt(ca),
            cb: pt(cb),
            cl: pt(cl),
        }
    }

    /// Trig propagation soundness: dense grid of bases and offsets —
    /// the true (std) cos/sin at base+offset must lie inside the
    /// propagated brackets, and the brackets stay tight (< 1e-9 after
    /// a 6-rad walk).
    #[test]
    fn trig_propagation_contains_truth_and_stays_tight() {
        for i in 0..40 {
            let t0 = -6.0 + 0.31 * f64::from(i);
            let base = (pt(t0.cos()), pt(t0.sin()));
            for j in 0..30 {
                let off = -6.0 + 0.41 * f64::from(j);
                let (c, s) = trig_at(base, off);
                let (tc, ts) = ((t0 + off).cos(), (t0 + off).sin());
                assert!(
                    c.lo() <= tc && tc <= c.hi() && s.lo() <= ts && ts <= s.hi(),
                    "trig_at unsound at t0={t0}, off={off}: cos {tc} vs {c:?}, sin {ts} vs {s:?}"
                );
                assert!(
                    c.width() < 1e-9 && s.width() < 1e-9,
                    "width blowup at {off}"
                );
                // The RANGE form contains a sample fan of the interval.
                let d = 0.7;
                let (cr, sr) = trig_over(base, off, d);
                for k in 0..=8 {
                    let t = t0 + off + d * f64::from(k) / 8.0;
                    assert!(
                        cr.lo() <= t.cos() && t.cos() <= cr.hi(),
                        "trig_over cos unsound at {t}"
                    );
                    assert!(
                        sr.lo() <= t.sin() && t.sin() <= sr.hi(),
                        "trig_over sin unsound at {t}"
                    );
                }
            }
        }
    }

    /// A full sinusoid band on a unit cylinder: bottom rim v = 0
    /// (u = t), top boundary v(u) = 2 + 0.5·sin u traversed backwards,
    /// closed by the two branches of the seam meridian. The exact
    /// signed UV area is 4π; the engine's converged enclosure must
    /// bracket it tightly, and a dense midpoint sum must sit inside
    /// the enclosure too (the reviewer's independent-oracle shape).
    #[test]
    fn sinusoid_band_area_brackets_the_closed_form() {
        let tau = core::f64::consts::TAU;
        let rim = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(0.0, 0.0, 0.0, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward: true,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let seam_up = TrimEdgeQ {
            u: chan(tau, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            t0: pt(0.0),
            t1: pt(2.0),
            forward: true,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        // Top: parameterized by t = u, v = 2 + 0.5 sin t, traversed
        // REVERSED (decreasing u).
        let top = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(2.0, 0.0, 0.5, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward: false,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let seam_down = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            t0: pt(0.0),
            t1: pt(2.0),
            forward: false,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let edges = [rim, seam_up, top, seam_down];
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let out = cylinder_cut_face::<f64>(pt(1.0), RingInterval::zero(), &edges, eps, band)
            .expect("the band face converges");
        let exact = 2.0 * tau; // ∮ u dv = 4π
        assert!(
            out.flux.lo() <= exact && exact <= out.flux.hi(),
            "flux {:?} must bracket {exact}",
            out.flux
        );
        assert!(
            out.area.lo() <= exact && exact <= out.area.hi(),
            "area {:?} must bracket {exact}",
            out.area
        );
        // Independent dense oracle: midpoint sum of Σ σ·∫u·v' dt.
        let mut acc = 0.0;
        let n = 200_000;
        for e in &edges {
            let (a, b) = (e.t0.lo(), e.t1.hi());
            let mut s = 0.0;
            for i in 0..n {
                let t = a + (b - a) * ((f64::from(i) + 0.5) / f64::from(n));
                let u =
                    e.u.c0.lo() + e.u.ca.lo() * t.cos() + e.u.cb.lo() * t.sin() + e.u.cl.lo() * t;
                let vp = e.v.cl.lo() + e.v.cb.lo() * t.cos() - e.v.ca.lo() * t.sin();
                s += u * vp;
            }
            s *= (b - a) / f64::from(n);
            acc += if e.forward { s } else { -s };
        }
        assert!(
            out.flux.lo() <= acc && acc <= out.flux.hi(),
            "dense oracle {acc} escapes the enclosure {:?}",
            out.flux
        );
        // The width cap SCALES FROM THE RESOLVED BAND (the multi-ε
        // lesson): the lane converges to
        // width(flux) ≤ QUAD_TARGET_LEN_FACTOR·ε · 3·lever — assert
        // against the target the lane actually used, not a fixed
        // number (at ε = 1e-6 a ~3e-2 width is a legitimately wider
        // certified enclosure, not a defect).
        let lever = (out.area.lo() + out.area.hi()) * 0.5;
        let width_cap = QUAD_TARGET_LEN_FACTOR * eps * 3.0 * lever * 1.000001;
        assert!(
            out.flux.width() <= width_cap,
            "width {} exceeds the lane's own converged target {width_cap} (eps {eps})",
            out.flux.width()
        );
    }

    /// Whole-circle endpoint ignorance cannot converge: the engine
    /// refuses typed at the budget, never returns a silently wide
    /// bracket.
    #[test]
    fn hopeless_endpoint_brackets_refuse_at_the_budget() {
        let wide = RingInterval::from_bounds(-1.0, 1.0);
        let e = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(2.0, 1.0, 0.0, 0.0),
            t0: pt(0.0),
            t1: pt(6.0),
            forward: true,
            trig0: (wide, wide),
            env: pt(0.0),
        };
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        match cylinder_cut_face::<f64>(pt(1.0), RingInterval::zero(), &[e], eps, band) {
            Err(PropsError::QuadratureBudget { .. }) => {}
            other => panic!("expected the typed budget refusal, got {other:?}"),
        }
    }

    /// The patch lane on the flat unit square at z = 1: `S = (u, v, 1)`
    /// has `f = S·(S_u×S_v) = 1` and `|S_u×S_v| = 1`, so flux = area
    /// = 1 exactly; then a genuinely warped bilinear patch is checked
    /// against a dense midpoint oracle (containment + tightness).
    #[test]
    fn nurbs_patch_flux_matches_flat_and_warped_oracles() {
        let band = Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let kv = KnotVector::unit_segment(1);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        // Row-major iu·nv+iv, nu = nv = 2: [(u0v0), (u0v1), (u1v0), (u1v1)].
        let flat = [
            p(0.0, 0.0, 1.0),
            p(0.0, 1.0, 1.0),
            p(1.0, 0.0, 1.0),
            p(1.0, 1.0, 1.0),
        ];
        let out = nurbs_patch_face::<f64>(
            &kv,
            &kv,
            &flat,
            &[1.0; 4],
            (0.0, 1.0, 0.0, 1.0),
            4.0,
            0.0,
            eps,
            band,
        )
        .unwrap();
        assert!(out.flux.contains(1.0), "flat flux {:?}", out.flux);
        assert!(out.area.contains(1.0), "flat area {:?}", out.area);
        assert!(out.flux.width() < 1e-9, "flux width {}", out.flux.width());

        // Warped: corners lifted unevenly — a ruled quadric patch.
        let warped = [
            p(0.0, 0.0, 0.5),
            p(-0.2, 1.1, 1.0),
            p(1.3, 0.0, 0.8),
            p(1.0, 1.0, 2.0),
        ];
        let out = nurbs_patch_face::<f64>(
            &kv,
            &kv,
            &warped,
            &[1.0; 4],
            (0.0, 1.0, 0.0, 1.0),
            6.0,
            0.0,
            eps,
            band,
        )
        .unwrap();
        // Dense midpoint oracle on the bilinear closed form.
        let sv = |u: f64, v: f64| -> [f64; 3] {
            core::array::from_fn(|k| {
                let c = [
                    warped[0][k].lo(),
                    warped[1][k].lo(),
                    warped[2][k].lo(),
                    warped[3][k].lo(),
                ];
                (1.0 - u) * ((1.0 - v) * c[0] + v * c[1]) + u * ((1.0 - v) * c[2] + v * c[3])
            })
        };
        let n = 400usize;
        let (mut oracle_flux, mut oracle_area) = (0.0f64, 0.0f64);
        let h = 1.0 / n as f64;
        for i in 0..n {
            for j in 0..n {
                let (u, v) = ((i as f64 + 0.5) * h, (j as f64 + 0.5) * h);
                let s = sv(u, v);
                let du = core::array::from_fn::<f64, 3, _>(|k| {
                    (sv(u + 1e-6, v)[k] - sv(u - 1e-6, v)[k]) / 2e-6
                });
                let dv = core::array::from_fn::<f64, 3, _>(|k| {
                    (sv(u, v + 1e-6)[k] - sv(u, v - 1e-6)[k]) / 2e-6
                });
                let cr = [
                    du[1] * dv[2] - du[2] * dv[1],
                    du[2] * dv[0] - du[0] * dv[2],
                    du[0] * dv[1] - du[1] * dv[0],
                ];
                oracle_flux += (s[0] * cr[0] + s[1] * cr[1] + s[2] * cr[2]) * h.powi(2);
                oracle_area += (cr[0].powi(2) + cr[1].powi(2) + cr[2].powi(2)).sqrt() * h.powi(2);
            }
        }
        assert!(
            out.flux.lo() - 1e-4 <= oracle_flux && oracle_flux <= out.flux.hi() + 1e-4,
            "flux {:?} vs oracle {oracle_flux}",
            out.flux
        );
        assert!(
            out.area.lo() - 1e-2 <= oracle_area && oracle_area <= out.area.hi() + 1e-2,
            "area {:?} vs oracle {oracle_area}",
            out.area
        );
    }

    /// **The M8-3 flip** of `rational_patch_refuses_typed`: the same
    /// patch — the unit square in the `z = 1` plane, reparameterized
    /// by a non-unit corner weight — now certifies an ENCLOSURE, and
    /// the re-derivation is that the rational bilinear map is a
    /// diffeomorphism of the square onto itself (corners fixed, edges
    /// to edges, positive Jacobian), so the reparameterization cannot
    /// move either number: flux `= ∫∫ 1·J = |Ω| = 1` and area `= 1`,
    /// exactly what the weight-1 patch gives.
    #[test]
    fn rational_patch_encloses_the_reparameterized_square() {
        let band = Band::linear(Tol::witness()).unwrap();
        let kv = KnotVector::unit_segment(1);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        let flat = [
            p(0.0, 0.0, 1.0),
            p(0.0, 1.0, 1.0),
            p(1.0, 0.0, 1.0),
            p(1.0, 1.0, 1.0),
        ];
        let out = nurbs_patch_face::<f64>(
            &kv,
            &kv,
            &flat,
            &[1.0, 2.0, 1.0, 1.0],
            (0.0, 1.0, 0.0, 1.0),
            4.0,
            0.0,
            Tol::witness().get().eps,
            band,
        );
        let posture = eps_posture("reparameterized square", &out, 1.0, 1.0, 0.0);
        eprintln!(
            "EPS-ROW reparameterized square @ eps={:e}: {posture:?}",
            Tol::witness().get().eps
        );
    }

    /// The SAME quarter cylinder built from TWO 45° sub-arcs (an
    /// interior double knot): the multi-span structure every arc wider
    /// than `MAX_SUB_ARC` actually has.
    #[test]
    fn rational_two_span_quarter_cylinder() {
        let band = Band::linear(Tol::witness()).unwrap();
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        let h = 2.0;
        let d = core::f64::consts::FRAC_PI_4;
        let hw = (d / 2.0).cos();
        let on = |a: f64| (a.cos(), a.sin());
        let tg = |a: f64| (a.cos() / hw, a.sin() / hw);
        let mut net = Vec::new();
        let mut weights = Vec::new();
        let pts: Vec<((f64, f64), f64)> = vec![
            (on(0.0), 1.0),
            (tg(d / 2.0), hw),
            (on(d), 1.0),
            (tg(1.5 * d), hw),
            (on(2.0 * d), 1.0),
        ];
        for ((x, y), w) in pts {
            net.push(p(x, y, 0.0));
            net.push(p(x, y, h));
            weights.push(w);
            weights.push(w);
        }
        let out = nurbs_patch_face::<f64>(
            &kv_u,
            &kv_v,
            &net,
            &weights,
            (0.0, 1.0, 0.0, 1.0),
            2.0f64.mul_add(2.0, core::f64::consts::PI),
            0.0,
            Tol::witness().get().eps,
            band,
        );
        let truth = core::f64::consts::PI;
        let posture = eps_posture("two-span quarter cylinder", &out, truth, truth, 0.0);
        eprintln!(
            "EPS-ROW two-span quarter cylinder @ eps={:e}: {posture:?}",
            Tol::witness().get().eps
        );
    }

    /// **The multiplicity ladder** (#313, the MAJOR's regression row).
    /// One degree-3 wall, one interior knot at `u = 0.5`, walked over
    /// every multiplicity the clamped invariant admits — `1..degree+1`,
    /// i.e. C² down to the C⁰ kink — through BOTH lanes (non-unit
    /// weights → rational, the unit-weight twin → integral).
    ///
    /// `deriv_kv` cannot represent the derivative knot vector from
    /// multiplicity `degree` upward; before the fix that fell through
    /// to a per-span CONSTANT first derivative, and the lane returned a
    /// thin enclosure that EXCLUDED the truth. The oracle here is
    /// independent — `geom_core`'s Book-A2.3 basis ladder and the
    /// quotient rule, not this file's de Boor — and the standing claim
    /// is the one the MAJOR broke: **an answer always contains the
    /// truth; the alternative is a typed refusal, never a wrong
    /// bracket.**
    #[test]
    #[allow(clippy::too_many_lines)] // the ladder plus its own oracle
    fn interior_multiplicity_ladder_never_certifies_a_wrong_enclosure() {
        use geom_core::spline::basis::ders_basis_funs;
        let band = Band::linear(Tol::witness()).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        // `pv` is gone: the oracle's v-window base is the `Span`'s own
        // `first_control()`, not a re-derived `span − degree`.
        let (pu, nv, height) = (3usize, 2usize, 2.0f64);
        let mut postures: Vec<(String, EpsPosture)> = Vec::new();
        for mult in 1..=pu {
            let mut knots = vec![0.0; pu + 1];
            knots.extend(core::iter::repeat_n(0.5, mult));
            knots.extend(core::iter::repeat_n(1.0, pu + 1));
            let kv_u = KnotVector::clamped(knots, pu).unwrap();
            let count = kv_u.control_count();
            // A curved wall: a flaring quarter-turn profile extruded.
            #[allow(clippy::cast_precision_loss)]
            let profile: Vec<(f64, f64)> = (0..count)
                .map(|i| {
                    let s = i as f64 / (count - 1) as f64;
                    let (a, r) = (s * core::f64::consts::FRAC_PI_2, 0.2f64.mul_add(s, 1.0));
                    (r * a.cos(), r * a.sin())
                })
                .collect();
            #[allow(clippy::cast_precision_loss)]
            let profile_w: Vec<f64> = (0..count)
                .map(|i| 0.06f64.mul_add((i % 3) as f64, 1.0))
                .collect();
            for rational in [false, true] {
                let mut net_f: Vec<[f64; 3]> = Vec::with_capacity(count * nv);
                let mut ws: Vec<f64> = Vec::with_capacity(count * nv);
                for (i, (x, y)) in profile.iter().enumerate() {
                    for z in [0.0, height] {
                        net_f.push([*x, *y, z]);
                        ws.push(if rational { profile_w[i] } else { 1.0 });
                    }
                }
                let net: Vec<RVec3> = net_f
                    .iter()
                    .map(|q| [pt(q[0]), pt(q[1]), pt(q[2])])
                    .collect();
                // The INDEPENDENT oracle: S = A/W through geom-core's
                // basis ladder, S_d by the quotient rule.
                let at = |u: f64, v: f64| -> ([f64; 3], [f64; 3], [f64; 3]) {
                    let (su, sv) = (kv_u.span_at(u), kv_v.span_at(v));
                    let bu = ders_basis_funs::<f64>(&kv_u, su, u, 1);
                    let bv = ders_basis_funs::<f64>(&kv_v, sv, v, 1);
                    // The `iu * nv + iv` stride stays written out on
                    // purpose: this oracle shares NO derivation with
                    // the code under test, so it does not borrow the
                    // surface window type.
                    let (mut a, mut w) = ([[0.0f64; 3]; 3], [0.0f64; 3]);
                    for (r, (nu0, nu1)) in bu[0].iter().zip(&bu[1]).enumerate() {
                        for (s, (nv0, nv1)) in bv[0].iter().zip(&bv[1]).enumerate() {
                            let idx = (su.first_control() + r) * nv + (sv.first_control() + s);
                            let (ww, q) = (ws[idx], net_f[idx]);
                            let b = [nu0 * nv0, nu1 * nv0, nu0 * nv1];
                            for k in 0..3 {
                                w[k] += b[k] * ww;
                                for (c, aq) in a[k].iter_mut().enumerate() {
                                    *aq += b[k] * ww * q[c];
                                }
                            }
                        }
                    }
                    let quot = |k: usize| -> [f64; 3] {
                        core::array::from_fn(|c| (a[k][c] * w[0] - a[0][c] * w[k]) / (w[0] * w[0]))
                    };
                    (core::array::from_fn(|c| a[0][c] / w[0]), quot(1), quot(2))
                };
                // WHY THE V-GRID IS 4 AND THE U-GRID IS 2048. Not a
                // budget judgement: on THIS net both integrands below
                // are EXACTLY independent of v, so the v-sum adds N
                // copies of one value and divides by N. Two premises,
                // both visible in the construction above — check them
                // before touching the net:
                //   (i)  `kv_v` is `unit_segment(1)`: degree 1, two
                //        control points, no interior knot;
                //   (ii) the weight pushed for BOTH v-rows of profile
                //        point `i` is the same `ws[i]` — the weight is
                //        indexed by the PROFILE index only, so the two
                //        v-rows carry EQUAL weights.
                // Through `at()` those give
                //   W = Σ_r N_r(u)·w_r·Σ_s N_s(v) = Σ_r N_r(u)·w_r
                //       (the v basis is a partition of unity), so `w[0]`
                //       is v-free and `w[2] = Σ_r N_r w_r · Σ_s N'_s`
                //       = 0 — the denominator has NO v-derivative;
                //   S   = (X(u), Y(u), height·v) — x,y repeat down each
                //       v-row, and in z the shared W cancels against
                //       Σ_s N_s(v)·z_s = height·v;
                //   S_u = (X_u, Y_u, 0) — the z component is
                //       (w[1]·h·v·w[0] − w[0]·h·v·w[1])/w[0]² ≡ 0;
                //   S_v = (0, 0, height) — w[2] = 0 leaves a[2]/w[0],
                //       and a[2] vanishes in x,y.
                // So cross = S_u × S_v = (Y_u·h, −X_u·h, 0), and
                //   S·cross = h·(X·Y_u − Y·X_u)  — S's v-dependent z
                //             multiplies cross_z = 0 and drops out;
                //   |cross| = h·√(X_u² + Y_u²).
                // Both are functions of u alone. MEASURED: 64 → 4 moves
                // the oracle by at most 2.3e-13 absolute on these O(3.5)
                // values (pure summation-order rounding) against the
                // 1e-5 slack below — 4e7x of headroom — while cutting
                // this row's runtime from 78 s to 53 s.
                //
                // The 2048 u-samples are the real content and STAY: u is
                // the O(h²) direction the slack is actually sized for.
                let (nu_s, nv_s) = (2048usize, 4usize);
                #[allow(clippy::cast_precision_loss)]
                let (hu, hv) = (1.0 / nu_s as f64, 1.0 / nv_s as f64);
                let (mut o_flux, mut o_area) = (0.0f64, 0.0f64);
                for i in 0..nu_s {
                    for j in 0..nv_s {
                        #[allow(clippy::cast_precision_loss)]
                        let (u, v) = ((i as f64 + 0.5) * hu, (j as f64 + 0.5) * hv);
                        let (s, s_u, s_v) = at(u, v);
                        let cr = [
                            s_u[1] * s_v[2] - s_u[2] * s_v[1],
                            s_u[2] * s_v[0] - s_u[0] * s_v[2],
                            s_u[0] * s_v[1] - s_u[1] * s_v[0],
                        ];
                        o_flux += (s[0] * cr[0] + s[1] * cr[1] + s[2] * cr[2]) * hu * hv;
                        o_area += (cr[0].powi(2) + cr[1].powi(2) + cr[2].powi(2)).sqrt() * hu * hv;
                    }
                }
                let lane = if rational { "rational" } else { "integral" };
                let row = format!("mult {mult} / {lane}");
                let out = nurbs_patch_face::<f64>(
                    &kv_u,
                    &kv_v,
                    &net,
                    &ws,
                    (0.0, 1.0, 0.0, 1.0),
                    10.0,
                    0.0,
                    Tol::witness().get().eps,
                    band,
                );
                // The slack is the ORACLE's discretization error, not
                // the lane's: on the polynomial rows the exact
                // Newton–Cotes lane returns the truth to rounding and
                // the dense midpoint sum lands 2.6e-7 off it (O(h²) at
                // 2048 u-samples, measured). 1e-5 is 40x that headroom
                // and still ~900x tighter than the exclusion the MAJOR
                // produced.
                postures.push((row.clone(), eps_posture(&row, &out, o_flux, o_area, 1e-5)));
            }
        }
        eprintln!(
            "EPS-ROW multiplicity ladder @ eps={:e}: {postures:?}",
            Tol::witness().get().eps
        );
        // Anti-vacuity, ε-KEYED rather than waived. The INTEGRAL rows
        // ride the exact per-span Newton–Cotes lane, whose width is
        // ring rounding only — ε-independent, so they must certify on
        // EVERY row of the matrix. The RATIONAL rows ride the O(h²)
        // composite against an ε-coupled target, so they certify at the
        // corpus ε and coarser and refuse typed below it; that boundary
        // is PINNED, not relaxed, and a posture that moves in either
        // direction is a finding.
        let certified: Vec<&str> = postures
            .iter()
            .filter(|(_, p)| *p == EpsPosture::Certified)
            .map(|(r, _)| r.as_str())
            .collect();
        let expected = if Tol::witness().get().eps >= CORPUS_EPS {
            2 * pu
        } else {
            pu
        };
        assert_eq!(
            certified.len(),
            expected,
            "the ladder's ε posture MOVED at eps={:e}: certified {certified:?}, all {postures:?}",
            Tol::witness().get().eps
        );
        assert!(
            certified.iter().all(|r| r.ends_with("integral")) || certified.len() == 2 * pu,
            "below the corpus ε only the exact-lane (integral) rows may certify: {certified:?}"
        );
    }

    /// A **genuinely curved** rational patch against a closed-form
    /// oracle: the quarter cylinder `r = 1`, height 2, as the standard
    /// rational-quadratic arc (weights `1, √2/2, 1`) crossed with a
    /// linear height. On it `S·(S_u×S_v) = (dθ/du)(dz/dv)`, so the
    /// flux is `∫dθ∫dz = (π/2)·2 = π` and the area is
    /// `r·(π/2)·2 = π` — both independent of the parameterization.
    #[test]
    fn rational_quarter_cylinder_brackets_the_closed_form() {
        let band = Band::linear(Tol::witness()).unwrap();
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        let h = 2.0;
        let net = [
            p(1.0, 0.0, 0.0),
            p(1.0, 0.0, h),
            p(1.0, 1.0, 0.0),
            p(1.0, 1.0, h),
            p(0.0, 1.0, 0.0),
            p(0.0, 1.0, h),
        ];
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let weights = [1.0, 1.0, w, w, 1.0, 1.0];
        let out = nurbs_patch_face::<f64>(
            &kv_u,
            &kv_v,
            &net,
            &weights,
            (0.0, 1.0, 0.0, 1.0),
            2.0f64.mul_add(2.0, core::f64::consts::PI),
            0.0,
            Tol::witness().get().eps,
            band,
        );
        let truth = core::f64::consts::PI;
        let posture = eps_posture("quarter cylinder", &out, truth, truth, 0.0);
        eprintln!(
            "EPS-ROW quarter cylinder @ eps={:e}: {posture:?}",
            Tol::witness().get().eps
        );
    }

    /// The general B-spline lane on a known integral: u = t (deg 2),
    /// v = t² (deg 2) on [0, 1]; ∫ u·v' dt = ∫ 2t² dt = 2/3.
    #[test]
    fn bspline_green_contains_the_polynomial_truth() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let u = [pt(0.0), pt(0.5), pt(1.0)];
        let v = [pt(0.0), pt(0.0), pt(1.0)];
        let w = [1.0, 1.0, 1.0];
        let out = bspline_green_integral(&kv, &u, &v, &w, 0.0, 1.0, 64).unwrap();
        let exact = 2.0 / 3.0;
        assert!(
            out.lo() <= exact && exact <= out.hi(),
            "{out:?} must bracket {exact}"
        );
        assert!(out.width() < 1e-6, "width {}", out.width());
    }

    /// Review MIN-2 pin (adopted): the endpoint-bracket pad's factor-2
    /// absorbs the trig0 PHASE-SHIFT term, not just the limit
    /// mismatch. The scan places the TRUE endpoints at the corners and
    /// edges of width-4e-3 brackets (trig0 anchored at the true t₀,
    /// integration limits at the bracket midpoints) and asserts the
    /// engine's enclosure contains a dense Kahan-summed oracle of the
    /// true integral at every placement.
    #[test]
    fn endpoint_bracket_phase_shift_corner_scan() {
        let w = 4e-3;
        // (fraction of w for true t0 off mid, same for t1): 4 corners,
        // 2 single-edge, center — 7 placements.
        let placements = [
            (-0.5, -0.5),
            (-0.5, 0.5),
            (0.5, -0.5),
            (0.5, 0.5),
            (-0.5, 0.0),
            (0.0, 0.5),
            (0.0, 0.0),
        ];
        // A strong linear channel against trig channels (the
        // phase-shift term is c_l-shaped).
        let u = chan(0.3, 0.0, 0.0, 1.0);
        let v = chan(2.0, 0.7, 0.4, 0.3);
        let (mid0, mid1) = (0.2, 5.9);
        for (d0, d1) in placements {
            let t0_true = mid0 + d0 * w;
            let t1_true = mid1 + d1 * w;
            let e = TrimEdgeQ {
                u,
                v,
                t0: RingInterval::from_bounds(mid0 - w * 0.5, mid0 + w * 0.5),
                t1: RingInterval::from_bounds(mid1 - w * 0.5, mid1 + w * 0.5),
                forward: true,
                trig0: (pt(t0_true.cos()), pt(t0_true.sin())),
                env: pt(0.0),
            };
            let enclosure = harmonic_edge_integral(&e, 65_536, pt(1.0));
            // Dense Kahan-summed midpoint oracle of the TRUE integral.
            let truth = kahan_dense_oracle(&u, &v, t0_true, t1_true, 400_000);
            assert!(
                enclosure.lo() <= truth && truth <= enclosure.hi(),
                "placement ({d0}, {d1}): truth {truth} escapes {enclosure:?} — the \
                 endpoint pad's two-term accounting failed"
            );
        }
    }

    /// Dense midpoint oracle with Kahan (compensated) summation — the
    /// reviewer's independent-integration shape.
    fn kahan_dense_oracle(u: &HarmChan, v: &HarmChan, a: f64, b: f64, n: u32) -> f64 {
        let (mut sum, mut c) = (0.0f64, 0.0f64);
        let h = (b - a) / f64::from(n);
        for i in 0..n {
            let t = a + h * (f64::from(i) + 0.5);
            let uu = u.c0.lo() + u.ca.lo() * t.cos() + u.cb.lo() * t.sin() + u.cl.lo() * t;
            let vp = v.cl.lo() + v.cb.lo() * t.cos() - v.ca.lo() * t.sin();
            let term = uu * vp * h - c;
            let s2 = sum + term;
            c = (s2 - sum) - term;
            sum = s2;
        }
        sum
    }

    /// Review probe family (adopted): three adversarial faces against
    /// dense Kahan-summed oracles — a near-pinching sinusoid band, a
    /// thin sliver band, and a mixed forward/reversed loop with large
    /// linear channels. Bound below truth anywhere = automatic MAJOR.
    #[test]
    fn adversarial_faces_contain_their_dense_oracles() {
        let tau = core::f64::consts::TAU;
        let band = geom_core::Band::linear(Tol::witness()).unwrap();
        let eps = Tol::witness().get().eps;
        let seam = |u_at: f64, v_hi: f64, forward: bool| TrimEdgeQ {
            u: chan(u_at, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            t0: pt(0.0),
            t1: pt(v_hi),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let rim = |forward: bool| TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(0.0, 0.0, 0.0, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        // (a) near-pinching band: v(u) = 1 + 0.999·sin u (dips to 1e-3).
        let pinch_top = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(1.0, 0.0, 0.999, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward: false,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let face_a = vec![
            rim(true),
            seam(tau, 1.0, true),
            pinch_top,
            seam(0.0, 1.0, false),
        ];
        // (b) thin sliver: bottom v = 1 + 0.5·cos u, top 1e-3 above it.
        let sliver_bot = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(1.0, 0.5, 0.0, 0.0),
            t0: pt(0.0),
            t1: pt(tau),
            forward: true,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let sliver_top = TrimEdgeQ {
            v: chan(1.0 + 1e-3, 0.5, 0.0, 0.0),
            forward: false,
            ..sliver_bot
        };
        // Both seams run v = 1.5 + 1e-3·t over t ∈ [0, 1] (the sliver
        // gap at u = 0 ≡ 2π); traversal direction alone distinguishes.
        let sliver_seam = |u_at: f64, forward: bool| TrimEdgeQ {
            u: chan(u_at, 0.0, 0.0, 0.0),
            v: chan(1.5, 0.0, 0.0, 1e-3),
            t0: pt(0.0),
            t1: pt(1.0),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let face_b = vec![
            sliver_bot,
            sliver_seam(tau, true),
            sliver_top,
            sliver_seam(0.0, false),
        ];
        // (c) mixed traversal with large linear channels: a skewed
        // band whose top runs v = 3 + 0.8·t (t = u) reversed.
        let skew_top = TrimEdgeQ {
            u: chan(0.0, 0.0, 0.0, 1.0),
            v: chan(3.0, 0.4, 0.0, 0.8),
            t0: pt(0.0),
            t1: pt(tau),
            forward: false,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let seam_c = |u_at: f64, v_hi: f64, forward: bool| TrimEdgeQ {
            u: chan(u_at, 0.0, 0.0, 0.0),
            v: chan(0.0, 0.0, 0.0, 1.0),
            t0: pt(0.0),
            t1: pt(v_hi),
            forward,
            trig0: (pt(1.0), pt(0.0)),
            env: pt(0.0),
        };
        let face_c = vec![
            rim(true),
            seam_c(tau, 3.0 + 0.8 * tau, true),
            skew_top,
            seam_c(0.0, 3.0, false),
        ];
        for (label, edges) in [("pinch", face_a), ("sliver", face_b), ("skew", face_c)] {
            // Independent truth: Σ σ·∫ u·v' dt, Kahan-summed.
            let mut truth = 0.0;
            for e in &edges {
                let s = kahan_dense_oracle(&e.u, &e.v, e.t0.lo(), e.t1.hi(), 400_000);
                truth += if e.forward { s } else { -s };
            }
            // Two honest outcomes, ε-dependent (the FitSampleBudget
            // precedent): a converged enclosure MUST contain the
            // oracle; at tight ε (1e-12 drives the target to ~1e-9 m
            // while the sliver's ring-arithmetic floor sits above it)
            // the TYPED budget refusal is the correct answer — never a
            // silently wide bracket, never a silent skip.
            match cylinder_cut_face::<f64>(pt(1.0), RingInterval::zero(), &edges, eps, band) {
                Ok(out) => {
                    assert!(
                        out.flux.lo() <= truth && truth <= out.flux.hi(),
                        "{label}: dense oracle {truth} escapes the certified flux {:?}",
                        out.flux
                    );
                }
                Err(PropsError::QuadratureBudget {
                    width_len,
                    target_len,
                }) => {
                    assert!(
                        width_len > target_len,
                        "{label}: a budget refusal must report an ACHIEVED width above \
                         its target, got {width_len} vs {target_len}"
                    );
                }
                Err(other) => panic!("{label}: outside the two honest arms: {other:?}"),
            }
        }
    }

    /// Rational channels refuse typed, naming the blocker.
    #[test]
    fn rational_pcurve_channels_refuse_typed() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let u = [pt(0.0), pt(0.5), pt(1.0)];
        let v = [pt(0.0), pt(0.0), pt(1.0)];
        let w = [1.0, 0.9, 1.0];
        match bspline_green_integral(&kv, &u, &v, &w, 0.0, 1.0, 8) {
            Err(PropsError::QuadratureUnsupported { what }) => {
                assert!(what.contains("rational"), "{what}");
                assert!(what.contains("loft"), "{what}");
            }
            other => panic!("expected the rational refusal, got {other:?}"),
        }
    }
}

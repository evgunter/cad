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
//!   convexity facts (`geom_core::spline::hull`). Since M6-3 the loft
//!   assembly mints stored iso-line pcurves on described NURBS walls
//!   and the patch flux engine consumes this machinery at rest.
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
use geom_core::spline::KnotVector;
use geom_core::spline::hull::{derivative_coeffs, span_hull};
use geom_core::{Band, Decide, Length, Sign};

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
    RingInterval::from_bounds(lo.lo().max(-1.0), hi.hi().min(1.0))
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
    let (l, h) = (lo.lo().max(-1.0), hi.hi().min(1.0));
    if s >= 0.0 {
        RingInterval::from_bounds(l, h)
    } else {
        RingInterval::from_bounds(-h, -l)
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
    let clamp = |x: RingInterval| RingInterval::from_bounds(x.lo().max(-1.0), x.hi().min(1.0));
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
    let clamp = |x: RingInterval| RingInterval::from_bounds(x.lo().max(-1.0), x.hi().min(1.0));
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
    margin: Length<f64>,
    band: Band,
) -> Result<Sign, PropsError> {
    geom_core::k_stats::decide(name, margin.lift::<T>(), band)
        .map_err(|cause| PropsError::Escalated { cause })
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
        // docs); the area midpoint is the lever arm.
        let lever = (area.lo() + area.hi()) * 0.5;
        let width_len = flux.width() / (3.0 * lever);
        last_width_len = width_len;
        if classify_len::<T>(
            "props_quad_converged",
            Length::of(target_len - width_len),
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
                Length::over_lever(area.lo(), perim),
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
    let span = kv.find_span(t);
    let mut d: Vec<RingInterval> = (0..=p).map(|j| coeffs[span - p + j]).collect();
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = span - p + j;
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
    for span in s0..=s1 {
        if !kv.span_is_nonempty(span) {
            continue;
        }
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
/// which is sound only on knot-free pieces; the composite rule
/// enforces exactly that (pieces straddling an interior knot take the
/// first-order hull rule, where no smoothness is assumed).
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
/// At-rest B-spline pcurves exist since M6-3 (the loft walls' exact
/// iso lines; general spline images remain the SSI trace's);
/// rational pcurves refuse typed — a rational derivative is not a
/// control-coefficient convexity fact.
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
    Const { knots: Vec<f64> },
}

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
    span: usize,
    t: RingInterval,
) -> RingInterval {
    if coeffs.len() != kv.control_count() {
        return RingInterval::poison();
    }
    let p = kv.degree();
    let u = kv.knots();
    let mut d: Vec<RingInterval> = (0..=p).map(|j| coeffs[span - p + j]).collect();
    for r in 1..=p {
        for j in (r..=p).rev() {
            let i = span - p + j;
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
/// all is `Option<PatchGrid> = None` (identically zero on knot-free
/// cells; cells straddling a knot take the smoothness-free
/// first-order rule, exactly the 1-D lane's argument).
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
        match deriv_kv(kv) {
            Some(k) => Dir::Kv(k),
            None => Dir::Const {
                knots: kv.knots()[1..kv.knots().len() - 1].to_vec(),
            },
        }
    }

    /// The u-partial-derivative grid (`None` = identically zero away
    /// from knots — the ladder's outer-None).
    fn deriv_u(&self) -> Option<Self> {
        let Dir::Kv(kv) = &self.du else {
            return None;
        };
        if self.nu < 2 {
            return None;
        }
        let mut ch = [Vec::new(), Vec::new(), Vec::new()];
        for (k, chan) in ch.iter_mut().enumerate() {
            let mut grid = vec![RingInterval::zero(); (self.nu - 1) * self.nv];
            for j in 0..self.nv {
                let col: Vec<RingInterval> =
                    (0..self.nu).map(|i| self.ch[k][i * self.nv + j]).collect();
                let d = derivative_coeffs(kv, &col);
                for (i, q) in d.iter().enumerate() {
                    grid[i * self.nv + j] = *q;
                }
            }
            *chan = grid;
        }
        Some(Self {
            du: Self::deriv_dir(kv),
            dv: self.dv.clone(),
            nu: self.nu - 1,
            nv: self.nv,
            ch,
        })
    }

    /// The v-partial-derivative grid.
    fn deriv_v(&self) -> Option<Self> {
        let Dir::Kv(kv) = &self.dv else {
            return None;
        };
        if self.nv < 2 {
            return None;
        }
        let mut ch = [Vec::new(), Vec::new(), Vec::new()];
        for (k, chan) in ch.iter_mut().enumerate() {
            let mut grid = Vec::with_capacity(self.nu * (self.nv - 1));
            for i in 0..self.nu {
                let row = &self.ch[k][i * self.nv..(i + 1) * self.nv];
                grid.extend(derivative_coeffs(kv, row));
            }
            *chan = grid;
        }
        Some(Self {
            du: self.du.clone(),
            dv: Self::deriv_dir(kv),
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
                bspline_eval_ring_in_span(kv, coeffs, kv.find_span(mid), *t)
            }
            (Dir::Kv(kv), Collapse::Over(lo, hi)) => bspline_range_hull(kv, coeffs, lo, hi),
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

/// **Certified NURBS-patch contributions** (M6-3 Leg C): the
/// chart-normal volume flux `∫∫ S·(S_u×S_v) du dv` and the patch area
/// `∫∫ |S_u×S_v| du dv` of a **non-rational** patch over the exact UV
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
/// straddling an interior knot in either direction take the
/// smoothness-free first-order rule `A·hull(f)`. The area rides the
/// first-order hull rule per cell — sound at every resolution,
/// tightening as O(h) — because the +V gate meters `V/A` and needs an
/// honest denominator, not a tight one (the RATIONAL area, which has
/// no such bound at all, refuses upstream with the rational walls).
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
/// [`PropsError`] — rational weights (typed, naming the banked
/// extension), an escalated funnel decision, a degenerate face, or
/// the typed [`PropsError::QuadratureBudget`] refusal.
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
    if weights.iter().any(|w| *w != 1.0) {
        return Err(PropsError::QuadratureUnsupported {
            what: "RATIONAL patch flux (weights != 1): the derivative-grid hulls are \
                   polynomial convexity facts, and a rational quotient's are not — the \
                   rational extension (any arc-bearing profile's walls) is BANKED; loft \
                   with a polyline profile, or wait for the rational-wall unit",
        });
    }
    let (u0, u1, v0, v1) = rect;
    if !(u1 - u0).is_finite() || u1 <= u0 || !(v1 - v0).is_finite() || v1 <= v0 {
        return Err(PropsError::QuadratureUnsupported {
            what: "empty or non-finite UV rectangle",
        });
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
    let mut last_width_len = f64::NAN;
    // GLOBAL second-derivative hulls, computed ONCE (the whole-rect
    // hull contains every knot-free cell's, so the per-cell remainder
    // `hull(f_uu)·h³/24` may use it soundly — looser by a constant,
    // still O(h²), and it turns the per-cell cost from ten grid hulls
    // into three thin evals; the 866-second debug wall this replaced
    // is the reason). Cells straddling an interior knot still take a
    // per-cell first-order hull (the smoothness-free rule needs local
    // tightness to converge, and such cells vanish under refinement).
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

    // AREA: one fixed-resolution pass of the first-order hull rule
    // (per-cell Su/Sv hulls). The area is a certified DENOMINATOR
    // (the +V meter and the extent gate), not a convergence-gated
    // quantity — its honest O(h) width lands in `area_pad`.
    let area = {
        let n = 2 * QUAD2_INIT_PIECES;
        #[allow(clippy::cast_precision_loss)]
        let (hu, hv) = ((u1 - u0) / n as f64, (v1 - v0) / n as f64);
        let cell_area = pt(hu) * pt(hv);
        let mut acc = RingInterval::zero();
        for iu in 0..n {
            #[allow(clippy::cast_precision_loss)]
            let c_ulo = u0 + (u1 - u0) * (iu as f64 / n as f64);
            for iv in 0..n {
                #[allow(clippy::cast_precision_loss)]
                let c_vlo = v0 + (v1 - v0) * (iv as f64 / n as f64);
                let over = (
                    Collapse::Over(c_ulo, c_ulo + hu),
                    Collapse::Over(c_vlo, c_vlo + hv),
                );
                let cross_h = rv_cross(
                    grid_vec(su.as_ref(), over.0, over.1),
                    grid_vec(sv.as_ref(), over.0, over.1),
                );
                let norm_sq = cross_h[0].sqr() + cross_h[1].sqr() + cross_h[2].sqr();
                acc = acc + cell_area * sqrt_enclosure(norm_sq);
            }
        }
        widen(acc, boundary_defect)
    };

    // ---- The exact per-span lane first (fn docs): one tensor
    // Newton–Cotes pass whose enclosure width is ring rounding only.
    // The composite rounds below are the fallback for degrees outside
    // the rule window (> 4 per direction). ----
    if let Some(exact) =
        patch_flux_exact(&s, su.as_ref(), sv.as_ref(), kv_u, kv_v, (u0, u1, v0, v1))
    {
        let flux = widen(exact, boundary_defect * p_bound);
        let lever = (area.lo() + area.hi()) * 0.5;
        let width_len = flux.width() / (3.0 * lever);
        if classify_len::<T>(
            "props_quad_converged",
            Length::of(target_len - width_len),
            band,
        )? == Sign::Positive
        {
            match classify_len::<T>(
                "props_quad_face_extent",
                Length::over_lever(area.lo(), perimeter),
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
        #[allow(clippy::cast_precision_loss)]
        let (hu, hv) = ((u1 - u0) / pieces as f64, (v1 - v0) / pieces as f64);
        let cell_area = pt(hu) * pt(hv);
        let r_uu = g_f_uu * (pt(hu) * pt(hu).sqr() * pt(hv) / pt(24.0));
        let r_vv = g_f_vv * (pt(hu) * pt(hv) * pt(hv).sqr() / pt(24.0));
        for iu in 0..pieces {
            #[allow(clippy::cast_precision_loss)]
            let c_ulo = u0 + (u1 - u0) * (iu as f64 / pieces as f64);
            let c_uhi = c_ulo + hu;
            let straddle_u = knots_u.iter().any(|k| *k > c_ulo && *k < c_uhi);
            for iv in 0..pieces {
                #[allow(clippy::cast_precision_loss)]
                let c_vlo = v0 + (v1 - v0) * (iv as f64 / pieces as f64);
                let c_vhi = c_vlo + hv;
                let straddle = straddle_u || knots_v.iter().any(|k| *k > c_vlo && *k < c_vhi);
                if straddle {
                    // Smoothness-free rule across a knot, per-cell
                    // hulls (local tightness needed here).
                    let over = (Collapse::Over(c_ulo, c_uhi), Collapse::Over(c_vlo, c_vhi));
                    let h_s = s.vec(over.0, over.1);
                    let cross_h = rv_cross(
                        grid_vec(su.as_ref(), over.0, over.1),
                        grid_vec(sv.as_ref(), over.0, over.1),
                    );
                    flux = flux + cell_area * rv_dot(h_s, cross_h);
                    continue;
                }
                let m = (
                    Collapse::At(c_ulo + hu * 0.5),
                    Collapse::At(c_vlo + hv * 0.5),
                );
                let fm = rv_dot(
                    s.vec(m.0, m.1),
                    rv_cross(
                        grid_vec(su.as_ref(), m.0, m.1),
                        grid_vec(sv.as_ref(), m.0, m.1),
                    ),
                );
                flux = flux + cell_area * fm + r_uu + r_vv;
            }
        }
        let flux = widen(flux, boundary_defect * p_bound);
        // Convergence: the shared mean-boundary-displacement meter.
        let lever = (area.lo() + area.hi()) * 0.5;
        let width_len = flux.width() / (3.0 * lever);
        last_width_len = width_len;
        if classify_len::<T>(
            "props_quad_converged",
            Length::of(target_len - width_len),
            band,
        )? == Sign::Positive
        {
            match classify_len::<T>(
                "props_quad_face_extent",
                Length::over_lever(area.lo(), perimeter),
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
    use geom_core::Tolerance;

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
        let band = geom_core::Band::linear().unwrap();
        let eps = Tolerance::get().eps;
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
        let band = geom_core::Band::linear().unwrap();
        let eps = Tolerance::get().eps;
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
        let band = Band::linear().unwrap();
        let eps = Tolerance::get().eps;
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

    /// The rational gate refuses typed, naming the banked extension.
    #[test]
    fn rational_patch_refuses_typed() {
        let band = Band::linear().unwrap();
        let kv = KnotVector::unit_segment(1);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        let flat = [
            p(0.0, 0.0, 1.0),
            p(0.0, 1.0, 1.0),
            p(1.0, 0.0, 1.0),
            p(1.0, 1.0, 1.0),
        ];
        let err = nurbs_patch_face::<f64>(
            &kv,
            &kv,
            &flat,
            &[1.0, 2.0, 1.0, 1.0],
            (0.0, 1.0, 0.0, 1.0),
            4.0,
            0.0,
            Tolerance::get().eps,
            band,
        )
        .unwrap_err();
        let text = format!("{err}");
        assert!(text.contains("RATIONAL patch flux"), "{text}");
        assert!(text.contains("BANKED"), "{text}");
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
        let band = geom_core::Band::linear().unwrap();
        let eps = Tolerance::get().eps;
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

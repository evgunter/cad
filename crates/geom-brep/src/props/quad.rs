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
//!   convexity facts (`geom_core::spline::hull`). Written generally;
//!   no at-rest construction mints a stored B-spline pcurve yet (the
//!   loft assembly unit does), so its consumers today are tests.
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
use geom_core::{Band, Decide, Sign};

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
    margin: f64,
    band: Band,
) -> Result<Sign, PropsError> {
    geom_core::k_stats::decide(name, T::from_f64(margin), band)
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
        if classify_len::<T>("props_quad_converged", target_len - width_len, band)?
            == Sign::Positive
        {
            // Face-extent gate on the CONVERGED enclosure: the area
            // must be definitely positive, metered as the face's mean
            // width area/perimeter — a length (the closed-form lanes'
            // `props_face_extent` lever, quadrature-shaped).
            let perim: f64 = edges.iter().map(|e| edge_metric_length(e, radius)).sum();
            match classify_len::<T>("props_quad_face_extent", area.lo() / perim, band)? {
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
/// No at-rest construction mints a stored B-spline pcurve yet (the
/// loft assembly unit brings them); rational pcurves refuse typed —
/// a rational derivative is not a control-coefficient convexity fact.
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
        assert!(out.flux.width() < 1e-5, "width {:?}", out.flux.width());
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
            let out = cylinder_cut_face::<f64>(pt(1.0), RingInterval::zero(), &edges, eps, band)
                .unwrap_or_else(|e| panic!("{label}: converges: {e:?}"));
            // Independent truth: Σ σ·∫ u·v' dt, Kahan-summed.
            let mut truth = 0.0;
            for e in &edges {
                let s = kahan_dense_oracle(&e.u, &e.v, e.t0.lo(), e.t1.hi(), 400_000);
                truth += if e.forward { s } else { -s };
            }
            assert!(
                out.flux.lo() <= truth && truth <= out.flux.hi(),
                "{label}: dense oracle {truth} escapes the certified flux {:?}",
                out.flux
            );
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

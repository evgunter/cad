//! **Certified quadrature for spiric-bounded faces** — the plane cap
//! and the torus wall a `Curve3::Spiric` bounds
//! (`docs/CURVED-SPIRIC-DESIGN.md` §4.7, Q5).
//!
//! With `P(t) = c + n·d + m·f(t) + a·r·sin t`, `m = a × n`,
//! `ρ = R + r·cos t`, `f = √(ρ² − d²)` (the carrier's own closed form,
//! `t` the torus's minor angle), both faces reduce to one-dimensional
//! integrals in `t` of algebraic functions of `(cos t, sin t)` — elliptic
//! integrals, with no elementary closed form. Everything else stays
//! closed.
//!
//! # The cap: the vector area
//!
//! A plane's flux is `anchor·A⃗` and its area `‖A⃗‖`, so the cap needs
//! only the spiric arc's vector-area term `(1/2)∫(P − ref) × P′ dt`.
//! Splitting `P − ref = (c + n·d − ref) + q`, `q = m·f + a·r·sin t`,
//! and using `m × a = n`:
//!
//! ```text
//! (1/2)·[(c + n·d − ref) × (P(t₁) − P(t₀)) + n·r·(2·J − [f·sin t]_{t₀}^{t₁})],
//! J = ∫ f(t)·cos t dt
//! ```
//!
//! (`q × q′ = n·r·(f cos t − f′ sin t)`, and the `f′` half integrates
//! by parts). `J` is the only quadrature.
//!
//! # The wall: the torus chart's Green form
//!
//! The torus face's flux about its centre and its chart area are
//! `−∮ F(v) du` and `−∮ G(v) du` over its lifted boundary, with
//! `F′ = r·ρ(v)·(R cos v + r)` and `G′ = r·ρ(v)` (the closed-form
//! lane's `torus_chart`, same antiderivatives). A rim at lifted level
//! `V` turning `Δu` gives `−F(V)·Δu` exactly; a meridian gives
//! nothing. A spiric's lifted image is exact
//! (`SpiricImage::Wall` on the carrier's own torus:
//! `u = u₀ + σ·atan2(f, d)`, `v = v₀ + σ·t`, `v₀ = k·τ`), so `sin v = σ sin t`, `cos v = cos t` and
//! `u′ = −σ·d·r·sin t/(ρ·f)`, and its terms are
//!
//! ```text
//! −∫ F(v₀ + σt)·u′ dt,   −∫ G(v₀ + σt)·u′ dt,   ∫ u′ dt  (its Δu, for the closure check)
//! ```
//!
//! # The rule and its remainder
//!
//! Per piece `[p, q]` with its midpoint `x` as the node (Taylor with
//! the Lagrange remainder; `gₖ = g⁽ᵏ⁾/k!`):
//!
//! ```text
//! ∫_p^q g  ∈  Σ_{k=0..3} gₖ(x)·∫_p^q (t − x)ᵏ dt  +  g₄([p, q])·∫_p^q (t − x)⁴ dt
//! ```
//!
//! — the weight `(t − x)⁴` is nonnegative, so the remainder's mean
//! value lies in the hull of `g₄` over the piece. The coefficients at
//! the node and over the piece come from one Taylor jet ([`Jet`]) in
//! certification arithmetic, whose only non-ring operation is `√`
//! (admitted by C9); a piece's width is `g₄`'s width over it times
//! `h⁵/80`, so the composite width falls as `h⁵`. `cos` and `sin`
//! enter as enclosures by [`super::quad`]'s transcendental-free
//! rotation (`trig_at` from the exact `(1, 0)` at `t = 0`, widened
//! over the piece by the interval rotation). The bracket is sound at
//! every resolution, so the enclosure only tightens as the schedule
//! doubles the pieces. Every piece boundary is an exact `f64`, the
//! last one the stored end itself, so the pieces tile the carrier
//! interval exactly.
//!
//! # Honesty pads
//!
//! - **Endpoint brackets**: a non-thin `t₀`/`t₁` bracket adds `sup|g|`
//!   (over the whole edge) times its half-width, and its closed-form
//!   endpoint terms are evaluated over the bracket.
//! - **Map residual** (the wall only): each lifted image tracks its
//!   carrier within its certificate envelope `e` — identically `0` for a
//!   spiric's own `SpiricImage::Wall`, an algebraic identity with its
//!   carrier, so on every body today the pad is the rims' and
//!   meridians' envelopes alone. The chart metric is
//!   `ds² = ρ²du² + r²dv²` with `ρ ≥ R − r`, so the chart region moves
//!   by at most `(chart length)·e/min(R − r, r)` of area, which costs
//!   at most `r(R + r)²` of flux and `r(R + r)` of area per unit.

use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi, cross, dot};
use geom_core::{Band, Bounds, Decide, Margin, Sign};

use super::closure::{IntervalJunction, interval_loop_turns, require_boundary_windings};
use super::quad::{
    AbsEnclosure, FaceCutBounds, QUAD_INIT_PIECES, QUAD_MAX_ROUNDS, QUAD_TARGET_LEN_FACTOR, RVec3,
    RoundOutcome, RoundWindow, classify_len, lo_or_refuse, mean_boundary_displacement, mid, pt,
    rotate, round_exit, spread_cos, spread_sin, trig_at,
};
use super::{PropsCheck, PropsError};

/// The Taylor coefficients `g⁽ᵏ⁾/k!`, `k = 0..=4`, of a function of `t`,
/// each an enclosure: at a node they are thin; over a piece (inputs
/// enclosing the piece) each encloses its coefficient over the whole
/// piece, which is what the remainder reads.
#[derive(Clone, Copy)]
struct Jet([Interval; ORDER]);

/// Coefficients carried: the rule is exact through degree 3 and its
/// remainder reads the degree-4 coefficient.
const ORDER: usize = 5;

impl Jet {
    fn cst(v: Interval) -> Self {
        let mut c = [Interval::zero(); ORDER];
        c[0] = v;
        Self(c)
    }

    fn add(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| self.0[k] + o.0[k]))
    }

    fn sub(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| self.0[k] - o.0[k]))
    }

    fn scale(self, x: Interval) -> Self {
        Self(self.0.map(|c| c * x))
    }

    /// The Cauchy product.
    fn mul(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| {
            (0..=k).fold(Interval::zero(), |acc, i| acc + self.0[i] * o.0[k - i])
        }))
    }

    /// `1/x`: `b₀ = 1/a₀`, `bₖ = −(Σ_{i=1..k} aᵢ·bₖ₋ᵢ)/a₀`.
    fn recip(self) -> Self {
        let inv = Interval::one() / self.0[0];
        let mut b = [Interval::zero(); ORDER];
        b[0] = inv;
        for k in 1..ORDER {
            let sum = (1..=k).fold(Interval::zero(), |acc, i| acc + self.0[i] * b[k - i]);
            b[k] = -(sum * inv);
        }
        Self(b)
    }

    /// `√x`: `s₀ = √a₀`, `sₖ = (aₖ − Σ_{i=1..k−1} sᵢ·sₖ₋ᵢ)/(2·s₀)`.
    fn sqrt(self) -> Self {
        let mut r = [Interval::zero(); ORDER];
        r[0] = self.0[0].sqrt();
        let half_inv = Interval::one() / (pt(2.0) * r[0]);
        for k in 1..ORDER {
            let sum = (1..k).fold(Interval::zero(), |acc, i| acc + r[i] * r[k - i]);
            r[k] = (self.0[k] - sum) * half_inv;
        }
        Self(r)
    }
}

/// The jets of `t`, `cos t` and `sin t` from enclosures of their values.
fn trig_jets(t: Interval, c: Interval, s: Interval) -> (Jet, Jet, Jet) {
    let z = Interval::zero();
    (
        Jet([t, Interval::one(), z, z, z]),
        Jet([c, -s, -c / pt(2.0), s / pt(6.0), c / pt(24.0)]),
        Jet([s, c, -s / pt(2.0), -c / pt(6.0), s / pt(24.0)]),
    )
}

/// `(cos t, sin t)` over every `t ∈ [lo, hi]`, exact `f64` ends: rotated
/// from the exact `(1, 0)` at `t = 0` to the midpoint, then widened over
/// the half-width by the interval rotation — the one widening every
/// enclosure here reads (a parameter bracket, a piece, an edge's span).
/// Exact for `lo = hi`; the whole circle past a half-width of `1.5`.
fn trig_range(lo: f64, hi: f64) -> (Interval, Interval) {
    let m = lo + (hi - lo) * 0.5;
    if !(m.is_finite() && lo <= hi) {
        return (Interval::refused(), Interval::refused());
    }
    let at = trig_at((Interval::one(), Interval::zero()), m);
    let hw = (pt(hi) - pt(m)).hi().max((pt(m) - pt(lo)).hi());
    rotate(at.0, at.1, spread_cos(hw), spread_sin(hw))
}

/// [`trig_range`] over a parameter bracket.
fn trig_of(x: Interval) -> (Interval, Interval) {
    if !x.is_certified() {
        return (Interval::refused(), Interval::refused());
    }
    trig_range(x.lo(), x.hi())
}

/// **The one home of a spiric edge's rate bounds over its span**: the
/// speed `|P′| ≤ S` and the wall image's azimuth rate
/// `|u′| = |d|·r·|sin t|/(ρ·f) ≤ √(S² − r²)/ρ_lo` (since
/// `S² = r²(1 + d²sin²/f²)` at the window's worst point), both from
/// [`geom::spiric_rate_bounds`] over the window the span actually
/// covers — `ρ` and `|sin t|` read off [`trig_range`] over
/// `[t₀.lo, t₁.hi]`. A cut whose plane passes the tube's hole
/// (`|d| > R − r`) has `f` real on its own arc though not on the whole
/// circle, and is bounded here by that arc. `None` when the window's
/// `f` is not certified positive.
pub fn span_rates(e: &SpiricEdgeQ) -> Option<(Interval, Interval)> {
    if !(e.t0.is_certified() && e.t1.is_certified() && e.minor.is_certified()) {
        return None;
    }
    let (c, s) = trig_range(e.t0.lo(), e.t1.hi());
    let rho = e.major + e.minor * c;
    if !(rho.is_certified() && s.is_certified()) {
        return None;
    }
    let rho_lo = pt(rho.lo());
    let (speed, _) =
        geom::spiric_rate_bounds(e.minor, e.offset, (rho_lo, pt(rho.hi())), pt(s.mag()));
    let reach = rho_lo.lo() - e.offset.mag();
    if !speed.is_certified() || reach.is_nan() || reach <= 0.0 {
        return None;
    }
    let du = (speed.sqr() - e.minor.sqr())
        .clamped_to(0.0, f64::INFINITY)
        .sqrt()
        / rho_lo;
    du.is_certified().then_some((speed, du))
}

/// An upper bound on a spiric edge's length, `S·(t₁ − t₀)`
/// ([`span_rates`]).
#[must_use]
pub fn spiric_length_bound(e: &SpiricEdgeQ) -> Option<f64> {
    let (speed, _) = span_rates(e)?;
    Some((speed * (e.t1 - e.t0)).mag())
}

/// `∫_a^b g` for `N` integrands at once, `pieces` equal pieces (the rule
/// in the module docs), plus the endpoint-bracket pad: `a`, `b` are the
/// brackets' midpoints, and `widths` their summed half-widths, paid at
/// `sup|g|` over the span's own window.
fn composite<const N: usize>(
    t0: Interval,
    t1: Interval,
    pieces: usize,
    g: impl Fn(Jet, Jet, Jet) -> [Jet; N],
) -> [Interval; N] {
    let (a, b) = (mid(t0), mid(t1));
    let span = b - a;
    if !(t0.is_certified() && t1.is_certified() && span.is_finite() && span >= 0.0) || pieces == 0 {
        return [Interval::refused(); N];
    }
    let mut total = [Interval::zero(); N];
    #[allow(clippy::cast_precision_loss)]
    let node = |i: usize| {
        if i == pieces {
            b
        } else {
            a + span * (i as f64 / pieces as f64)
        }
    };
    for i in 0..pieces {
        let (p, q) = (node(i), node(i + 1));
        let x = p + (q - p) * 0.5;
        let (ri, le) = (pt(q) - pt(x), pt(p) - pt(x));
        // `∫_p^q (t − x)ᵏ dt`, `k = 0..=4`.
        let weight: [Interval; ORDER] = core::array::from_fn(|k| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let e = k as i32 + 1;
            (ri.powi(e) - le.powi(e)) / pt(f64::from(e))
        });
        let (cx, sx) = trig_range(x, x);
        let (tj, cj, sj) = trig_jets(pt(x), cx, sx);
        let at = g(tj, cj, sj);
        let (cb, sb) = trig_range(p, q);
        let (tb, cbj, sbj) = trig_jets(Interval::from_bounds(p, q), cb, sb);
        let over = g(tb, cbj, sbj);
        for k in 0..N {
            let exact =
                (0..ORDER - 1).fold(Interval::zero(), |acc, j| acc + at[k].0[j] * weight[j]);
            total[k] = total[k] + exact + over[k].0[ORDER - 1] * weight[ORDER - 1];
        }
    }
    let widths = (pt(t0.hi()) - pt(a)).hi().max((pt(a) - pt(t0.lo())).hi())
        + (pt(t1.hi()) - pt(b)).hi().max((pt(b) - pt(t1.lo())).hi());
    if widths > 0.0 {
        let (c, s) = trig_range(t0.lo(), t1.hi());
        let (tj, cj, sj) = trig_jets(Interval::hull(t0, t1), c, s);
        let sup = g(tj, cj, sj);
        for k in 0..N {
            let pad = (sup[k].0[0] * pt(widths)).mag();
            total[k] = total[k] + Interval::from_bounds(-pad, pad);
        }
    }
    total
}

/// `ρ = R + r·cos t` and `f = √(ρ² − d²)` as jets.
fn rho_f(major: Interval, minor: Interval, offset: Interval, c: Jet) -> (Jet, Jet) {
    let rho = Jet::cst(major).add(c.scale(minor));
    let f = rho.mul(rho).sub(Jet::cst(offset.sqr())).sqrt();
    (rho, f)
}

/// One spiric boundary edge of a cap or a wall, bracketed by the
/// caller (key-free, like [`super::LoopEdge`]).
#[derive(Clone, Copy, Debug)]
pub struct SpiricEdgeQ {
    /// The carrier's centre less the face's reference point.
    pub centre: RVec3,
    /// The carrier's unit axis `a`.
    pub axis: RVec3,
    /// The cutting plane's unit normal `n` (the carrier's `u_ref`).
    pub normal: RVec3,
    /// `R`.
    pub major: Interval,
    /// `r`.
    pub minor: Interval,
    /// `d`.
    pub offset: Interval,
    /// The carrier interval's start.
    pub t0: Interval,
    /// The carrier interval's end.
    pub t1: Interval,
    /// Loop traversal direction.
    pub forward: bool,
    /// On a torus wall, the edge's lifted chart image.
    pub wall: Option<WallImageQ>,
}

/// A spiric's lifted image on its OWN torus wall
/// (`SpiricImage::Wall` tagged `WallTorus::Own`):
/// `u = u₀ + σ·atan2(f, d)`, `v = v₀ + σ·t`. A wall on the mirror torus
/// (`WallTorus::Mirror`, azimuth sign `−σ`) has no terms here yet, and
/// `topo`'s quad lane refuses it typed before building one
/// (FLUX's `the-spiric-wall-quadrature-lane-reads-only-the-own-torus`).
#[derive(Clone, Copy, Debug)]
pub struct WallImageQ {
    /// `σ = ±1`.
    pub sense: f64,
    /// The lifted minor-angle constant `v₀`.
    pub v0: Interval,
    /// The image's certificate envelope (metres).
    pub env: Interval,
}

/// A rim of a torus wall on its lift: the level `V` and its signed `Δu`
/// (traversal folded in).
#[derive(Clone, Copy, Debug)]
pub struct RimQ {
    /// The lifted level.
    pub v: Interval,
    /// The signed azimuth turn.
    pub du: Interval,
    /// The image's certificate envelope (metres).
    pub env: Interval,
}

/// A meridian of a torus wall on its lift: its signed `Δv`.
#[derive(Clone, Copy, Debug)]
pub struct MeridianQ {
    /// The signed minor-angle turn.
    pub dv: Interval,
    /// The image's certificate envelope (metres).
    pub env: Interval,
}

/// The torus chart a wall's Green form runs on.
#[derive(Clone, Debug)]
pub struct WallChartQ {
    /// The torus's `R`.
    pub major: Interval,
    /// The torus's `r`.
    pub minor: Interval,
    /// Its rims.
    pub rims: Vec<RimQ>,
    /// Its meridians.
    pub meridians: Vec<MeridianQ>,
    /// Every edge of its loop on the lift, in traversal order: what the
    /// closure decision reads ([`wall_windings`]).
    pub ends: Vec<ChartEndsQ>,
}

/// One edge of a torus wall's loop on its lift: where its traversal
/// starts and ends, `(u, v)`, and its image's certificate envelope
/// (metres).
#[derive(Clone, Copy, Debug)]
pub struct ChartEndsQ {
    /// The traversal's start.
    pub start: (Interval, Interval),
    /// The traversal's end.
    pub end: (Interval, Interval),
    /// The image's certificate envelope (metres).
    pub env: Interval,
}

/// One spiric-bounded face, bracketed by the caller.
///
/// The face's vector area about a reference point is `closed_va` (the
/// closed-form edges, already summed) plus each spiric's term; its flux
/// is `lever·A⃗` (a plane: `lever` is a point of the face less the
/// centre; a torus: the torus centre less the centre) plus, on a wall,
/// the chart Green form. The area is `‖A⃗‖` on a cap and the chart
/// area's magnitude on a wall.
#[derive(Clone, Debug)]
pub struct SpiricFaceQ {
    /// The closed-form edges' vector-area terms, about the reference.
    pub closed_va: RVec3,
    /// The spiric edges, centres relative to the reference.
    pub spirics: Vec<SpiricEdgeQ>,
    /// The flux lever.
    pub lever: RVec3,
    /// An upper bound on the boundary's length (metres) — the extent
    /// gate's lever.
    pub perimeter: f64,
    /// The torus chart, for a wall; `None` for a cap.
    pub wall: Option<WallChartQ>,
}

/// `J = ∫ f(t)·cos t dt` over the edge's carrier interval.
fn cos_moment(e: &SpiricEdgeQ, pieces: usize) -> Interval {
    let (big, r, d) = (e.major, e.minor, e.offset);
    let [j] = composite(e.t0, e.t1, pieces, |_, c, _| {
        let (_, f) = rho_f(big, r, d, c);
        [f.mul(c)]
    });
    j
}

/// One spiric edge's vector-area term about the reference (module
/// docs), at `pieces` resolution.
#[must_use]
pub fn spiric_vector_area(e: &SpiricEdgeQ, pieces: usize) -> RVec3 {
    let (big, r, d) = (e.major, e.minor, e.offset);
    let j = cos_moment(e, pieces);
    let end = |t: Interval| {
        let (c, s) = trig_of(t);
        let f = ((big + r * c).sqr() - d.sqr()).sqrt();
        (f, s)
    };
    let ((f0, s0), (f1, s1)) = (end(e.t0), end(e.t1));
    let m = cross(&e.axis, &e.normal);
    let base = [0, 1, 2].map(|k| e.centre[k] + e.normal[k] * d);
    let chord = [0, 1, 2].map(|k| m[k] * (f1 - f0) + e.axis[k] * (r * (s1 - s0)));
    let sector = r * (pt(2.0) * j - (f1 * s1 - f0 * s0));
    let cross = cross(&base, &chord);
    let half = pt(0.5);
    let va = [0, 1, 2].map(|k| (cross[k] + e.normal[k] * sector) * half);
    if e.forward { va } else { va.map(|x| -x) }
}

/// One spiric wall edge's Green-form terms `(−∫F du, −∫G du, Δu, Δv)`
/// on its lift (module docs), traversal folded in.
fn spiric_wall_terms(
    e: &SpiricEdgeQ,
    w: &WallImageQ,
    chart: &WallChartQ,
    pieces: usize,
) -> [Interval; 4] {
    let (big, r) = (chart.major, chart.minor);
    let d = e.offset;
    let sigma = pt(w.sense);
    let k = sigma * d * r;
    let rr = big.sqr() + r.sqr();
    let three_half = pt(1.5) * big * r;
    let half_br = pt(0.5) * big * r;
    let v0 = w.v0;
    let [flux, area, du] = composite(e.t0, e.t1, pieces, |t, c, s| {
        let (rho, f) = rho_f(big, r, d, c);
        let wgt = s.mul(rho.mul(f).recip()).scale(k);
        let v = Jet::cst(v0).add(t.scale(sigma));
        let ff = s
            .scale(sigma * rr)
            .add(v.scale(three_half))
            .add(s.mul(c).scale(sigma * half_br))
            .scale(r);
        let gg = v.scale(big).add(s.scale(sigma * r)).scale(r);
        [ff.mul(wgt), gg.mul(wgt), wgt.scale(-Interval::one())]
    });
    let dv = sigma * (e.t1 - e.t0);
    let terms = [flux, area, du, dv];
    if e.forward { terms } else { terms.map(|x| -x) }
}

/// `F(V)` and `G(V)` at a lifted level (the closed-form lane's
/// antiderivatives).
fn rim_fg(big: Interval, r: Interval, v: Interval) -> (Interval, Interval) {
    let (c, s) = trig_of(v);
    let f = r * ((big.sqr() + r.sqr()) * s + pt(1.5) * big * r * v + pt(0.5) * big * r * s * c);
    let g = r * (big * v + r * s);
    (f, g)
}

/// The wall's chart terms at `pieces`: `(flux, signed area, Σ Δu, Σ Δv)`
/// with the map-residual pad folded into the first two.
fn wall_chart(
    face: &SpiricFaceQ,
    chart: &WallChartQ,
    pieces: usize,
) -> Result<[Interval; 4], PropsError> {
    let (big, r) = (chart.major, chart.minor);
    let mut acc = [Interval::zero(); 4];
    // Σ (chart length · envelope), for the map-residual pad.
    let mut defect = Interval::zero();
    for e in &face.spirics {
        let Some(w) = e.wall.as_ref() else {
            return Err(PropsError::QuadratureUnsupported {
                what: "a spiric edge of a torus wall carries no lifted wall image",
            });
        };
        let t = spiric_wall_terms(e, w, chart, pieces);
        for k in 0..4 {
            acc[k] = acc[k] + t[k];
        }
        // The chart length: `|u′|` over the span, and `|v′| = 1`.
        let Some((_, du_max)) = span_rates(e) else {
            return Err(PropsError::QuadratureUnsupported {
                what: "a spiric wall edge whose span's rate window has no certified \
                       positive f",
            });
        };
        let len = (du_max + Interval::one()) * (e.t1 - e.t0);
        defect = defect + len * w.env;
    }
    for rim in &chart.rims {
        let (f, g) = rim_fg(big, r, rim.v);
        acc[0] = acc[0] - f * rim.du;
        acc[1] = acc[1] - g * rim.du;
        acc[2] = acc[2] + rim.du;
        defect = defect + pt(rim.du.mag()) * rim.env;
    }
    for mer in &chart.meridians {
        acc[3] = acc[3] + mer.dv;
        defect = defect + pt(mer.dv.mag()) * mer.env;
    }
    // The chart's smallest metric scale, `min(R − r, r)`, from below — a
    // refused radius has ordinary endpoints, so it is asked by name.
    let floor = (big - r).lo().min(r.lo());
    if !(big.is_certified() && r.is_certified()) || floor.is_nan() || floor <= 0.0 {
        return Err(PropsError::QuadratureUnsupported {
            what: "a spiric wall on a torus whose metric floor min(R − r, r) is not \
                   certified positive, or whose radii are refused brackets",
        });
    }
    let uv = (defect / pt(floor)).mag();
    let outer = big + r;
    let pad_flux = (pt(uv) * r * outer.sqr()).mag();
    let pad_area = (pt(uv) * r * outer).mag();
    acc[0] = acc[0] + Interval::from_bounds(-pad_flux, pad_flux);
    acc[1] = acc[1] + Interval::from_bounds(-pad_area, pad_area);
    Ok(acc)
}

/// The face's enclosures at `pieces` resolution.
fn face_bounds(
    face: &SpiricFaceQ,
    pieces: usize,
) -> Result<(FaceCutBounds, [Interval; 2]), PropsError> {
    let mut va = face.closed_va;
    for e in &face.spirics {
        let t = spiric_vector_area(e, pieces);
        va = [0, 1, 2].map(|k| va[k] + t[k]);
    }
    let position = dot(&face.lever, &va);
    Ok(match &face.wall {
        None => (
            FaceCutBounds {
                flux: position,
                area: (va[0].sqr() + va[1].sqr() + va[2].sqr()).sqrt(),
            },
            [Interval::zero(); 2],
        ),
        Some(chart) => {
            let [flux, area, du, dv] = wall_chart(face, chart, pieces)?;
            (
                FaceCutBounds {
                    flux: flux + position,
                    area: area.abs_enclosure(),
                },
                [du, dv],
            )
        }
    })
}

/// **A torus wall's loop closes on its chart and winds it not at all**
/// — the periodic-chart closure decision ([`super::closure`]) on the
/// loop's lifted images ([`WallChartQ::ends`]).
///
/// At each junction the azimuth's gap is metered at the junction's own
/// distance from the axis, `ρ = R + r·cos v`, enclosed over the after
/// edge's start, and the tube angle's at `r`; the slack is the two
/// images' envelopes. A wall carries one loop, so its turns must be
/// zero.
fn wall_windings<T: Decide>(chart: &WallChartQ, band: Band) -> Result<(), PropsError> {
    if chart.ends.is_empty() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a spiric torus wall handed no loop to decide closed",
        });
    }
    let (big, r) = (chart.major, chart.minor);
    let junctions = chart
        .ends
        .iter()
        .zip(chart.ends.iter().cycle().skip(1))
        .map(|(e, next)| {
            let (cos_v, _) = trig_of(next.start.1);
            IntervalJunction {
                delta: [e.end.0 - next.start.0, e.end.1 - next.start.1],
                lever: [big + r * cos_v, r],
                slack: certified_hi(e.env + next.env),
            }
        });
    let turns = interval_loop_turns::<T, 2>(junctions, [true, true], band)?;
    require_boundary_windings(&[turns])
}

/// **The certified flux and area of a spiric-bounded face** over a
/// [`RoundWindow`] — [`super::quad`]'s schedule, meter and exits
/// (pieces doubling from `QUAD_INIT_PIECES`, convergence at
/// `QUAD_TARGET_LEN_FACTOR·ε` of mean boundary displacement, the
/// face-extent gate, the budget refusal).
///
/// On a wall, the loop must close and wind the torus not at all
/// ([`wall_windings`]): the Green form bounds a region of the lifted
/// chart only then.
///
/// # Errors
///
/// [`PropsError`] — an escalated funnel decision, a degenerate face,
/// a wall loop that does not close ([`PropsError::ChartLoopOpen`]) or
/// winds the torus ([`PropsError::LoopsWindChart`]), or the typed
/// [`PropsError::QuadratureBudget`] refusal.
pub fn spiric_face_rounds<T: Decide>(
    face: &SpiricFaceQ,
    eps: f64,
    band: Band,
    window: RoundWindow,
) -> Result<RoundOutcome, PropsError> {
    if let Some(chart) = &face.wall {
        wall_windings::<T>(chart, band)?;
    }
    let target_len = QUAD_TARGET_LEN_FACTOR * eps;
    let schedule = super::quad::Schedule::to(QUAD_MAX_ROUNDS);
    let first = schedule.first(window);
    let mut pieces = QUAD_INIT_PIECES << first;
    for round in first..=schedule.last() {
        let (bounds, _) = face_bounds(face, pieces)?;
        let width_len = mean_boundary_displacement(bounds.flux, bounds.area)?;
        let converged = schedule.converged::<T>(
            round,
            classify_len::<T>(
                "props_quad_converged",
                Margin::of(target_len - width_len),
                band,
                PropsCheck::Converged,
            ),
            (target_len, width_len),
            band,
        )?;
        let out = if converged {
            match classify_len::<T>(
                "props_quad_face_extent",
                Margin::over_lever(lo_or_refuse(bounds.area), face.perimeter),
                band,
                PropsCheck::Extent,
            )? {
                Sign::Positive => {}
                Sign::Zero | Sign::Negative => return Err(PropsError::DegenerateFace),
            }
            Some(RoundOutcome::Converged(bounds))
        } else {
            round_exit(
                round,
                schedule.last(),
                window,
                bounds,
                (width_len, target_len),
            )
        };
        if let Some(out) = out {
            return Ok(out);
        }
        pieces *= 2;
    }
    unreachable!("the round window is non-empty: `first` is clamped to the last round")
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;

    /// One reference case: `(R, r, d, t₀, t₁, σ, v₀)` and the four
    /// integrals' exact values as f64 round-down/round-up pairs
    /// (`J`, `Δu`, `−∫F du`, `−∫G du`), from a 50-digit adaptive
    /// quadrature of the DEFINITIONS (`u` differentiated numerically
    /// from `σ·atan2(f, d)`, not from this file's `u′`).
    type Case = ((f64, f64, f64, f64, f64, f64, f64), [(f64, f64); 4]);

    const PI: f64 = core::f64::consts::PI;

    fn cases() -> Vec<(&'static str, Case)> {
        vec![
            (
                "the vessel's arc, d < 0",
                (
                    (
                        0.09375,
                        0.0703125,
                        -0.0078125,
                        -0.8911225078866529,
                        0.8911225078866528,
                        -1.0,
                        0.0,
                    ),
                    [
                        (0.24255864761540002, 0.24255864761540005),
                        (2.4967740135765447e-18, 2.496774013576545e-18),
                        (-1.9156748891069987e-05, -1.9156748891069984e-05),
                        (-0.0001220663324153128, -0.00012206633241531278),
                    ],
                ),
            ),
            (
                "the vessel's arc, d > 0",
                (
                    (
                        0.09375,
                        0.0703125,
                        0.0078125,
                        -0.8911225078866529,
                        0.8911225078866528,
                        -1.0,
                        0.0,
                    ),
                    [
                        (0.24255864761540002, 0.24255864761540005),
                        (-2.496774013576545e-18, -2.4967740135765447e-18),
                        (1.9156748891069984e-05, 1.9156748891069987e-05),
                        (0.00012206633241531278, 0.0001220663324153128),
                    ],
                ),
            ),
            (
                "a span across π on the lift's second turn",
                (
                    (1.2, 0.225, 0.05, PI / 2.0, 3.0 * PI / 2.0, 1.0, 2.0 * PI),
                    [
                        (-2.0441206394570477, -2.0441206394570473),
                        (-1.915173823133119e-18, -1.9151738231331187e-18),
                        (0.0030300413224422515, 0.003030041322442252),
                        (-0.0042231023902593095, -0.004223102390259309),
                    ],
                ),
            ),
            (
                "a whole period, d < 0, σ = −1, the lift one turn down",
                (
                    (1.0, 0.3, -0.25, 0.0, 2.0 * PI, -1.0, -2.0 * PI),
                    [
                        (0.9759549009659082, 0.9759549009659083),
                        (-1.356466621510341e-33, -1.3564666215103408e-33),
                        (-0.022829979179362032, -0.02282997917936203),
                        (0.11236141214982714, 0.11236141214982716),
                    ],
                ),
            ),
            (
                "an asymmetric span, where Δu is not zero",
                (
                    (1.2, 0.225, 0.05, 0.3, 2.1, -1.0, 0.0),
                    [
                        (0.802442501578793, 0.8024425015787932),
                        (0.010695140649616026, 0.010695140649616028),
                        (0.004534513409502406, 0.004534513409502407),
                        (0.004469279612030439, 0.00446927961203044),
                    ],
                ),
            ),
            (
                "a near-tangent cut, f_min ≈ 0.01",
                (
                    (1.0, 0.5, 0.4999, -PI, PI, 1.0, 0.0),
                    [
                        (1.9990851916383723, 1.9990851916383725),
                        (0.0, 0.0),
                        (2.8963604554843574, 2.896360455484358),
                        (3.126972087751443, 3.1269720877514433),
                    ],
                ),
            ),
        ]
    }

    fn edge(c: (f64, f64, f64, f64, f64, f64, f64)) -> (SpiricEdgeQ, WallImageQ, WallChartQ) {
        let (big, r, d, t0, t1, sense, v0) = c;
        let wall = WallImageQ {
            sense,
            v0: pt(v0),
            env: Interval::zero(),
        };
        let e = SpiricEdgeQ {
            centre: [pt(0.25), pt(-0.5), pt(1.0)],
            axis: [pt(0.0), pt(0.0), pt(1.0)],
            normal: [pt(1.0), pt(0.0), pt(0.0)],
            major: pt(big),
            minor: pt(r),
            offset: pt(d),
            t0: pt(t0),
            t1: pt(t1),
            forward: true,
            wall: Some(wall),
        };
        let chart = WallChartQ {
            ends: Vec::new(),
            major: pt(big),
            minor: pt(r),
            rims: Vec::new(),
            meridians: Vec::new(),
        };
        (e, wall, chart)
    }

    /// The width each case's enclosures reach at 1024 pieces, as a
    /// fraction of its largest integral: about 100× the measured width
    /// (`3e-13` to `2e-12` on the regular cases, `4e-6` near tangency).
    fn tightness(name: &str) -> f64 {
        match name {
            "a near-tangent cut, f_min ≈ 0.01" => 5e-4,
            "a whole period, d < 0, σ = −1, the lift one turn down" => 3e-10,
            _ => 1e-10,
        }
    }

    /// **Containment against exact values**, at every resolution the
    /// schedule visits (the bracket is sound at each, not only once
    /// converged), and the enclosure is useful by 1024 pieces: within
    /// about 100× the width it measures there ([`tightness`]), which a
    /// lower-order or wider rule would not be. The references are
    /// re-taken by `scripts/spiric-quadrature-references.py`.
    #[test]
    fn every_integral_encloses_its_exact_value_at_every_resolution() {
        for (name, (c, refs)) in cases() {
            let (e, w, chart) = edge(c);
            let scale = refs.iter().fold(0.0_f64, |m, &(lo, _)| m.max(lo.abs()));
            let tight = tightness(name);
            for pieces in [16, 128, 1024] {
                let [flux, area, du, _] = spiric_wall_terms(&e, &w, &chart, pieces);
                let got = [cos_moment(&e, pieces), du, flux, area];
                for (k, (g, (lo, hi))) in got.iter().zip(refs).enumerate() {
                    assert!(
                        g.is_certified() && g.lo() <= lo && hi <= g.hi(),
                        "{name}: integral {k} at {pieces} pieces is [{}, {}], \
                         the exact value is in [{lo}, {hi}]",
                        g.lo(),
                        g.hi()
                    );
                    if pieces == 1024 {
                        assert!(
                            g.hi() - g.lo() <= tight * scale,
                            "{name}: integral {k} at 1024 pieces is [{}, {}], wider than \
                             {tight:e} of the case's scale {scale}",
                            g.lo(),
                            g.hi()
                        );
                    }
                }
            }
        }
    }

    /// A reversed edge negates every term, and `Δv` is `σ·(t₁ − t₀)`.
    #[test]
    fn reversal_negates_and_dv_is_the_span() {
        let (_, (c, _)) = cases().swap_remove(4);
        let (e, w, chart) = edge(c);
        let f = spiric_wall_terms(&e, &w, &chart, 64);
        let r = spiric_wall_terms(
            &SpiricEdgeQ {
                forward: false,
                ..e
            },
            &w,
            &chart,
            64,
        );
        for (a, b) in f.iter().zip(r) {
            assert_eq!((a.lo(), a.hi()), (-b.hi(), -b.lo()));
        }
        assert!(f[3].contains(-(2.1 - 0.3)), "dv = σ·span, got {:?}", f[3]);
    }

    /// Over a whole period the chord and endpoint terms vanish and the
    /// vector area is `n·r·J` — the oval's area along its plane normal.
    #[test]
    fn a_whole_oval_has_vector_area_n_r_j() {
        let (_, (c, refs)) = cases().swap_remove(5);
        let (e, ..) = edge(c);
        let va = spiric_vector_area(&e, 1024);
        let (lo, hi) = refs[0];
        let area = va[0];
        assert!(
            area.lo() <= 0.5 * lo && 0.5 * hi <= area.hi(),
            "r·J with r = 0.5, got {area:?}"
        );
        assert!(va[1].contains(0.0) && va[2].contains(0.0), "{va:?}");
    }

    // ---- Independent f64 references and the rows that hold the pads ----

    /// Composite 5-point Gauss–Legendre on `n` equal pieces.
    #[allow(clippy::cast_precision_loss)]
    fn gl(a: f64, b: f64, n: usize, g: impl Fn(f64) -> f64) -> f64 {
        let x = [
            0.0,
            0.538_469_310_105_683_1,
            -0.538_469_310_105_683_1,
            0.906_179_845_938_664,
            -0.906_179_845_938_664,
        ];
        let w = [
            0.568_888_888_888_888_9,
            0.478_628_670_499_366_5,
            0.478_628_670_499_366_5,
            0.236_926_885_056_189_1,
            0.236_926_885_056_189_1,
        ];
        let h = (b - a) / n as f64;
        let mut acc = 0.0;
        for i in 0..n {
            let m = a + h * (i as f64 + 0.5);
            for k in 0..5 {
                acc += w[k] * g(m + 0.5 * h * x[k]);
            }
        }
        acc * 0.5 * h
    }

    fn f3cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
        [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ]
    }

    fn f3unit(a: [f64; 3]) -> [f64; 3] {
        let l = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
        a.map(|x| x / l)
    }

    /// A deterministic generator for the frame sweep (shape 3: a fixed
    /// population of frames, not a search).
    struct Lcg(u64);

    impl Lcg {
        #[allow(clippy::cast_precision_loss)]
        fn next(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((self.0 >> 11) as f64) / ((1_u64 << 53) as f64)
        }

        fn range(&mut self, a: f64, b: f64) -> f64 {
            a + (b - a) * self.next()
        }
    }

    /// A spiric edge in an arbitrary frame: `(edge, P(t) − ref)`.
    #[allow(clippy::too_many_arguments)]
    fn framed(
        (big, r, d): (f64, f64, f64),
        axis: [f64; 3],
        normal: [f64; 3],
        centre: [f64; 3],
        reference: [f64; 3],
        (t0, t1): (f64, f64),
        forward: bool,
    ) -> (SpiricEdgeQ, impl Fn(f64) -> [f64; 3]) {
        let m = f3cross(axis, normal);
        let point = move |t: f64| {
            let f = ((big + r * t.cos()).powi(2) - d * d).sqrt();
            [0, 1, 2].map(|k| {
                centre[k] + normal[k] * d + m[k] * f + axis[k] * r * t.sin() - reference[k]
            })
        };
        let e = SpiricEdgeQ {
            centre: [0, 1, 2].map(|k| pt(centre[k] - reference[k])),
            axis: axis.map(pt),
            normal: normal.map(pt),
            major: pt(big),
            minor: pt(r),
            offset: pt(d),
            t0: pt(t0),
            t1: pt(t1),
            forward,
            wall: None,
        };
        (e, point)
    }

    /// `½∫(P − ref) × P′ dt` of the carrier's own point formula, `P′` by
    /// central difference — nothing from this module's algebra.
    fn vector_area_reference(point: &impl Fn(f64) -> [f64; 3], t0: f64, t1: f64) -> [f64; 3] {
        let dp = |t: f64| {
            let (u, v) = (point(t + 1e-6), point(t - 1e-6));
            [0, 1, 2].map(|k| (u[k] - v[k]) / 2e-6)
        };
        [0, 1, 2].map(|k| 0.5 * gl(t0, t1, 2000, |t| f3cross(point(t), dp(t))[k]))
    }

    /// **The cap's vector-area term against the carrier's own point
    /// formula**, over a fixed population of frames: random axes,
    /// normals, centres and reference points, both signs of `d`,
    /// partial and multi-turn spans, both traversals. The reference is
    /// an f64 quadrature with a finite-difference `P′`, so a sound
    /// enclosure meets it within that reference's own error (`1e-8`).
    #[test]
    fn the_cap_term_encloses_the_carriers_own_vector_area_in_any_frame() {
        let mut g = Lcg(0x4554);
        for case in 0..60 {
            let big = g.range(0.5, 3.0);
            let r = big * g.range(0.1, 0.8);
            let d = (big - r) * g.range(0.02, 0.98) * if g.next() < 0.5 { -1.0 } else { 1.0 };
            let axis = f3unit([g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)]);
            let w = [g.range(-1.0, 1.0), g.range(-1.0, 1.0), g.range(-1.0, 1.0)];
            let along = w[0] * axis[0] + w[1] * axis[1] + w[2] * axis[2];
            let normal = f3unit([0, 1, 2].map(|k| w[k] - axis[k] * along));
            let centre = [g.range(-2.0, 2.0), g.range(-2.0, 2.0), g.range(-2.0, 2.0)];
            let reference = [g.range(-2.0, 2.0), g.range(-2.0, 2.0), g.range(-2.0, 2.0)];
            let t0 = g.range(-4.0, 4.0);
            let t1 = t0 + g.range(0.05, 6.2);
            for forward in [true, false] {
                let (e, point) = framed(
                    (big, r, d),
                    axis,
                    normal,
                    centre,
                    reference,
                    (t0, t1),
                    forward,
                );
                let want = vector_area_reference(&point, t0, t1);
                let got = spiric_vector_area(&e, 64);
                for k in 0..3 {
                    let w = if forward { want[k] } else { -want[k] };
                    assert!(
                        got[k].is_certified() && got[k].lo() - 1e-8 <= w && w <= got[k].hi() + 1e-8,
                        "case {case} forward {forward} channel {k}: [{}, {}] misses {w}",
                        got[k].lo(),
                        got[k].hi()
                    );
                }
            }
        }
    }

    /// **A cut through the tube's hole** (`|d| > R − r`): `f` is real on
    /// the arc though not on the whole circle, and the lane bounds it by
    /// the arc's own window ([`span_rates`]) — the vector area, the wall
    /// terms and the length bound all certify, against the definitions.
    /// The whole-circle bound (`f_min = √((R − r)² − d²)`) is NaN here.
    #[test]
    fn a_cut_through_the_tubes_hole_is_bounded_by_its_own_arc() {
        let (big, r, d, t0, t1) = (1.0, 0.4, 0.8, -1.2, 1.1);
        let (e, point) = framed(
            (big, r, d),
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.3, -0.2, 0.1],
            [0.0; 3],
            (t0, t1),
            true,
        );
        assert!(
            spiric_length_bound(&e).is_some_and(f64::is_finite),
            "the arc's length"
        );
        let want = vector_area_reference(&point, t0, t1);
        let got = spiric_vector_area(&e, 256);
        for k in 0..3 {
            assert!(got[k].lo() - 1e-8 <= want[k] && want[k] <= got[k].hi() + 1e-8);
        }
        let w = WallImageQ {
            sense: 1.0,
            v0: pt(0.0),
            env: pt(1e-6),
        };
        let chart = WallChartQ {
            ends: Vec::new(),
            major: pt(big),
            minor: pt(r),
            rims: vec![],
            meridians: vec![],
        };
        let face = SpiricFaceQ {
            closed_va: [Interval::zero(); 3],
            spirics: vec![SpiricEdgeQ { wall: Some(w), ..e }],
            lever: [Interval::zero(); 3],
            perimeter: 1.0,
            wall: Some(chart.clone()),
        };
        assert!(
            wall_chart(&face, &chart, 64).is_ok(),
            "the wall's pad has a rate bound"
        );
    }

    /// **The wall terms against the definitions** — `u` differentiated
    /// numerically from `σ·atan2(f, d)`, `F` and `G` the chart's
    /// antiderivatives at the lifted `v₀ + σt` — at both `σ`, lifts a
    /// turn either way, and a cut through the hole.
    #[test]
    fn the_wall_terms_match_their_definitions_at_every_lift() {
        let tau = 2.0 * PI;
        for &(big, r, d, t0, t1) in &[
            (1.2, 0.225, 0.05, 0.3, 2.1),
            (1.0, 0.4, 0.8, -1.2, 1.1),
            (1.0, 0.4, -0.8, -0.7, 1.2),
            (2.0, 0.5, -1.3, 2.0, 4.0),
        ] {
            for sigma in [1.0, -1.0] {
                for v0 in [0.0, tau, -tau] {
                    let f = |t: f64| ((big + r * t.cos()).powi(2) - d * d).sqrt();
                    let u = |t: f64| sigma * f(t).atan2(d);
                    let du = |t: f64| (u(t + 1e-6) - u(t - 1e-6)) / 2e-6;
                    let lifted = |t: f64| v0 + sigma * t;
                    let ff = |v: f64| {
                        r * ((big * big + r * r) * v.sin()
                            + 1.5 * big * r * v
                            + 0.5 * big * r * v.sin() * v.cos())
                    };
                    let gg = |v: f64| r * (big * v + r * v.sin());
                    let want = [
                        -gl(t0, t1, 2000, |t| ff(lifted(t)) * du(t)),
                        -gl(t0, t1, 2000, |t| gg(lifted(t)) * du(t)),
                        u(t1) - u(t0),
                    ];
                    let (e, _) = framed(
                        (big, r, d),
                        [0.0, 0.0, 1.0],
                        [1.0, 0.0, 0.0],
                        [0.0; 3],
                        [0.0; 3],
                        (t0, t1),
                        true,
                    );
                    let w = WallImageQ {
                        sense: sigma,
                        v0: pt(v0),
                        env: Interval::zero(),
                    };
                    let chart = WallChartQ {
                        ends: Vec::new(),
                        major: pt(big),
                        minor: pt(r),
                        rims: vec![],
                        meridians: vec![],
                    };
                    let got = spiric_wall_terms(&e, &w, &chart, 256);
                    for k in 0..3 {
                        assert!(
                            got[k].lo() - 1e-8 <= want[k] && want[k] <= got[k].hi() + 1e-8,
                            "R={big} r={r} d={d} σ={sigma} v0={v0} term {k}: {:?} vs {}",
                            got[k],
                            want[k]
                        );
                    }
                }
            }
        }
    }

    /// **A closed wall loop's area and flux against a direct 2-D
    /// integral** of the chart densities over the region it bounds —
    /// spiric, rim, meridian, rim through [`wall_chart`], both `σ` — so
    /// the `G` channel, the rims' `−F·Δu`/`−G·Δu` signs and the
    /// assembly are measured, not only each spiric term alone.
    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn a_closed_wall_loop_measures_the_region_it_bounds() {
        let (big, r, d, t0, t1, um) = (1.2_f64, 0.225_f64, 0.05_f64, 0.3_f64, 2.1_f64, -0.5_f64);
        for sigma in [1.0_f64, -1.0] {
            let f = |t: f64| ((big + r * t.cos()).powi(2) - d * d).sqrt();
            let u = |t: f64| sigma * f(t).atan2(d);
            let n = 100_000;
            let h = (t1 - t0) / n as f64;
            let (mut area, mut flux) = (0.0, 0.0);
            for i in 0..n {
                let t = t0 + h * (i as f64 + 0.5);
                let v = sigma * t;
                let rho = big + r * v.cos();
                let width = u(t) - um;
                area += r * rho * width * h;
                flux += r * rho * (big * v.cos() + r) * width * h;
            }
            let (e, _) = framed(
                (big, r, d),
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                [0.0; 3],
                [0.0; 3],
                (t0, t1),
                true,
            );
            let chart = WallChartQ {
                ends: Vec::new(),
                major: pt(big),
                minor: pt(r),
                rims: vec![
                    RimQ {
                        v: pt(sigma * t1),
                        du: pt(um) - pt(u(t1)),
                        env: Interval::zero(),
                    },
                    RimQ {
                        v: pt(sigma * t0),
                        du: pt(u(t0)) - pt(um),
                        env: Interval::zero(),
                    },
                ],
                meridians: vec![MeridianQ {
                    dv: pt(sigma * (t0 - t1)),
                    env: Interval::zero(),
                }],
            };
            let face = SpiricFaceQ {
                closed_va: [Interval::zero(); 3],
                spirics: vec![SpiricEdgeQ {
                    wall: Some(WallImageQ {
                        sense: sigma,
                        v0: pt(0.0),
                        env: Interval::zero(),
                    }),
                    ..e
                }],
                lever: [Interval::zero(); 3],
                perimeter: 1.0,
                wall: Some(chart.clone()),
            };
            let got = wall_chart(&face, &chart, 512).unwrap();
            // `σ = −1` mirrors `v`, so the loop winds the other way.
            let (area, flux) = (sigma * area, sigma * flux);
            let near = |x: Interval, w: f64| x.lo() - 1e-8 <= w && w <= x.hi() + 1e-8;
            assert!(near(got[1], area), "σ {sigma}: area {:?} vs {area}", got[1]);
            assert!(near(got[0], flux), "σ {sigma}: flux {:?} vs {flux}", got[0]);
            assert!(
                got[2].contains(0.0) && got[3].contains(0.0),
                "the loop closes"
            );
        }
    }

    /// A spiric wall edge's [`ChartEndsQ`] on its image
    /// `u = u₀ + σ·atan2(f, d)`, `v = v₀ + σ·t` ([`WallImageQ`]), `u0` the
    /// image's azimuth constant. `None` for an edge with no wall image.
    fn spiric_wall_ends(e: &SpiricEdgeQ, u0: Interval) -> Option<ChartEndsQ> {
        let w = e.wall.as_ref()?;
        let sigma = pt(w.sense);
        let at = |t: Interval| {
            let (c, _) = trig_of(t);
            let f = ((e.major + e.minor * c).sqr() - e.offset.sqr()).sqrt();
            (
                u0 + sigma * geom_core::Real::atan2(f, e.offset),
                w.v0 + sigma * t,
            )
        };
        let (a, b) = (at(e.t0), at(e.t1));
        let (start, end) = if e.forward { (a, b) } else { (b, a) };
        Some(ChartEndsQ {
            start,
            end,
            env: w.env,
        })
    }

    /// **A junction's azimuth gap is metered at the junction's own
    /// distance from the axis** ([`wall_windings`]). On fat tori (`R = 1`,
    /// `r = 0.5` and `0.6`), a whole oval from `t = π` closed by a meridian
    /// back meets it at the inner equator, `ρ = R − r`. The meridian set
    /// off in `u` by `δ` with `ρ·δ = 2ε` is within the vertex slack and
    /// measures, at `f64` and at the interval scalar; read at `R + r` the
    /// same gap is `6ε` and `8ε`, past it. Set off by `ρ·δ = 20ε` it is
    /// open ([`PropsError::ChartLoopOpen`]).
    #[test]
    fn an_inner_equator_junction_is_metered_at_its_own_radius() {
        let tol = geom_core::Tol::witness();
        let band = Band::linear(tol).unwrap();
        let eps = tol.eps();
        for r in [0.5, 0.6] {
            let rho = 1.0 - r;
            let face = |gap: f64| {
                let (e, _) = framed(
                    (1.0, r, 0.05),
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0, 0.0],
                    [0.0; 3],
                    [0.0; 3],
                    (PI, 3.0 * PI),
                    true,
                );
                let arc = SpiricEdgeQ {
                    wall: Some(WallImageQ {
                        sense: 1.0,
                        v0: Interval::zero(),
                        env: Interval::zero(),
                    }),
                    ..e
                };
                let oval = spiric_wall_ends(&arc, Interval::zero()).unwrap();
                let du = pt(gap / rho);
                let back = ChartEndsQ {
                    start: (oval.end.0 + du, oval.end.1),
                    end: (oval.end.0 + du, oval.start.1),
                    env: Interval::zero(),
                };
                SpiricFaceQ {
                    closed_va: [Interval::zero(); 3],
                    spirics: vec![arc],
                    lever: [Interval::zero(); 3],
                    perimeter: 20.0,
                    wall: Some(WallChartQ {
                        major: pt(1.0),
                        minor: pt(r),
                        rims: vec![],
                        meridians: vec![MeridianQ {
                            dv: oval.start.1 - oval.end.1,
                            env: Interval::zero(),
                        }],
                        ends: vec![oval, back],
                    }),
                }
            };
            let near = face(2.0 * eps);
            let at_f64 = spiric_face_rounds::<f64>(&near, eps, band, RoundWindow::SCHEDULE);
            assert!(at_f64.is_ok(), "r = {r}, f64: {at_f64:?}");
            let at_iv = spiric_face_rounds::<Interval>(&near, eps, band, RoundWindow::SCHEDULE);
            assert!(at_iv.is_ok(), "r = {r}, interval: {at_iv:?}");
            let far =
                spiric_face_rounds::<f64>(&face(20.0 * eps), eps, band, RoundWindow::SCHEDULE);
            assert!(
                matches!(far, Err(PropsError::ChartLoopOpen)),
                "r = {r}, 20ε: {far:?}"
            );
        }
    }

    /// Reviewer probe (PR 4938 verify): a whole oval from `t0` closed by a
    /// meridian back, the back set off by a true gap `g` (metres) in `u`
    /// (at the junction's own `ρ = R + r·cos t0`) or in `v` (at `r`).
    /// Expected: `g < S + ε = 4ε` measures, `4ε < g < 13ε` escalates,
    /// `g > 13ε` refuses `ChartLoopOpen`.
    #[test]
    fn review_4938_junction_gap_sweep() {
        let tol = geom_core::Tol::witness();
        let band = Band::linear(tol).unwrap();
        let eps = tol.eps();
        let k = tol.k();
        let mut bad = Vec::new();
        let mut rows = 0;
        let small = std::env::var_os("REVIEW4938_SMALL").is_some();
        let rs: &[f64] = if small {
            &[0.5, 0.6]
        } else {
            &[0.1, 0.3, 0.5, 0.6, 0.9]
        };
        let ts: &[f64] = if small {
            &[PI, 0.0]
        } else {
            &[PI, 0.0, PI / 2.0, 2.0 * PI / 3.0, 0.9 * PI]
        };
        for &r in rs {
            for &t0 in ts {
                let rho = 1.0 + r * t0.cos();
                for in_v in [false, true] {
                    for g in [
                        0.0,
                        1.0,
                        2.0,
                        3.0,
                        3.8,
                        4.3,
                        8.0,
                        0.5 + 3.0 + k - 0.8,
                        3.0 + k + 0.4,
                        20.0,
                    ] {
                        let (e, _) = framed(
                            (1.0, r, 0.05),
                            [0.0, 0.0, 1.0],
                            [1.0, 0.0, 0.0],
                            [0.0; 3],
                            [0.0; 3],
                            (t0, t0 + 2.0 * PI),
                            true,
                        );
                        let arc = SpiricEdgeQ {
                            wall: Some(WallImageQ {
                                sense: 1.0,
                                v0: Interval::zero(),
                                env: Interval::zero(),
                            }),
                            ..e
                        };
                        let oval = spiric_wall_ends(&arc, Interval::zero()).unwrap();
                        let (du, dv) = if in_v {
                            (Interval::zero(), pt(g * eps / r))
                        } else {
                            (pt(g * eps / rho), Interval::zero())
                        };
                        let back = ChartEndsQ {
                            start: (oval.end.0 + du, oval.end.1 + dv),
                            end: (oval.end.0 + du, oval.start.1 + dv),
                            env: Interval::zero(),
                        };
                        let face = SpiricFaceQ {
                            closed_va: [Interval::zero(); 3],
                            spirics: vec![arc],
                            lever: [Interval::zero(); 3],
                            perimeter: 20.0,
                            wall: Some(WallChartQ {
                                major: pt(1.0),
                                minor: pt(r),
                                rims: vec![],
                                meridians: vec![MeridianQ {
                                    dv: oval.start.1 - oval.end.1,
                                    env: Interval::zero(),
                                }],
                                ends: vec![oval, back],
                            }),
                        };
                        let want = if g < 4.0 {
                            "ok"
                        } else if g < 3.0 + k {
                            "esc"
                        } else {
                            "open"
                        };
                        let class = |x: &Result<RoundOutcome, PropsError>| match x {
                            Ok(_) => "ok",
                            Err(PropsError::Escalated { .. }) => "esc",
                            Err(PropsError::ChartLoopOpen) => "open",
                            Err(_) => "other",
                        };
                        let a = spiric_face_rounds::<f64>(&face, eps, band, RoundWindow::SCHEDULE);
                        let b =
                            spiric_face_rounds::<Interval>(&face, eps, band, RoundWindow::SCHEDULE);
                        rows += 1;
                        if class(&a) != want || class(&b) != want {
                            bad.push(format!(
                                "r={r} t0={t0:.3} in_v={in_v} g={g}ε want {want}: f64 {a:?} iv {b:?}"
                            ));
                        }
                    }
                }
            }
        }
        eprintln!("REVIEW4938 sweep eps={eps} rows={rows} bad={}", bad.len());
        for b in &bad {
            eprintln!("REVIEW4938 {b}");
        }
        assert!(bad.is_empty());
    }

    /// **A junction its gap's bounds cannot decide is never measured**:
    /// the meridian back's start held as a bracket `20ε/ρ` wide in `u`
    /// about the oval's end, so the gap's lower bound is within the slack
    /// and its upper bound past it. The closed decision reads the upper
    /// bound and the open one the lower, so neither holds: it escalates,
    /// at `f64` and at the interval scalar.
    #[test]
    fn a_junction_its_bounds_cannot_decide_is_never_measured() {
        let tol = geom_core::Tol::witness();
        let band = Band::linear(tol).unwrap();
        let eps = tol.eps();
        let (e, _) = framed(
            (1.2, 0.275, 0.05),
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0; 3],
            [0.0; 3],
            (0.0, 2.0 * PI),
            true,
        );
        let arc = SpiricEdgeQ {
            wall: Some(WallImageQ {
                sense: 1.0,
                v0: Interval::zero(),
                env: Interval::zero(),
            }),
            ..e
        };
        let oval = spiric_wall_ends(&arc, Interval::zero()).unwrap();
        // At `v = 0` the junction sits at `ρ = R + r`.
        let half = 10.0 * eps / 1.475;
        let wide = oval.end.0 + Interval::from_bounds(-half, half);
        let face = SpiricFaceQ {
            closed_va: [Interval::zero(); 3],
            spirics: vec![arc],
            lever: [Interval::zero(); 3],
            perimeter: 20.0,
            wall: Some(WallChartQ {
                major: pt(1.2),
                minor: pt(0.275),
                rims: vec![],
                meridians: vec![MeridianQ {
                    dv: oval.start.1 - oval.end.1,
                    env: Interval::zero(),
                }],
                ends: vec![
                    oval,
                    ChartEndsQ {
                        start: (wide, oval.end.1),
                        end: (oval.start.0, oval.start.1),
                        env: Interval::zero(),
                    },
                ],
            }),
        };
        for got in [
            spiric_face_rounds::<f64>(&face, eps, band, RoundWindow::SCHEDULE),
            spiric_face_rounds::<Interval>(&face, eps, band, RoundWindow::SCHEDULE),
        ] {
            assert!(
                matches!(got, Err(PropsError::Escalated { .. })),
                "an undecidable junction escalates: {got:?}"
            );
        }
    }

    /// **The closure decision refuses an open loop and a winding one,
    /// and admits a closed one** ([`wall_windings`]): a whole oval closed
    /// by a meridian back passes; the oval alone winds the tube once
    /// ([`PropsError::LoopsWindChart`]); an asymmetric arc closed by a
    /// meridian at its end's azimuth is open in `u` by `≈ 0.0107`
    /// ([`PropsError::ChartLoopOpen`]); the oval twice round winds twice
    /// ([`PropsError::LoopsWindChart`]); and the closed loop with a
    /// junction open by `2ε`, inside the vertex slack, measures.
    #[test]
    fn the_closure_decision_refuses_open_and_winding_and_admits_closed() {
        let tol = geom_core::Tol::witness();
        let band = Band::linear(tol).unwrap();
        let tau = 2.0 * PI;
        let wall = |d: f64, t0: f64, t1: f64| {
            let (e, _) = framed(
                (1.2, 0.275, d),
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                [0.0; 3],
                [0.0; 3],
                (t0, t1),
                true,
            );
            SpiricEdgeQ {
                wall: Some(WallImageQ {
                    sense: 1.0,
                    v0: Interval::zero(),
                    env: Interval::zero(),
                }),
                ..e
            }
        };
        // The meridian back from the arc's end to its start's level, at
        // the end's azimuth.
        let back = |arc: &SpiricEdgeQ| {
            let ends = spiric_wall_ends(arc, Interval::zero()).unwrap();
            let dv = ends.start.1 - ends.end.1;
            (
                MeridianQ {
                    dv,
                    env: Interval::zero(),
                },
                ChartEndsQ {
                    start: ends.end,
                    end: (ends.end.0, ends.start.1),
                    env: Interval::zero(),
                },
            )
        };
        let face = |arc: SpiricEdgeQ, closed_by_back: bool| {
            let mut ends = vec![spiric_wall_ends(&arc, Interval::zero()).unwrap()];
            let mut meridians = vec![];
            if closed_by_back {
                let (m, e) = back(&arc);
                meridians.push(m);
                ends.push(e);
            }
            SpiricFaceQ {
                closed_va: [Interval::zero(); 3],
                spirics: vec![arc],
                lever: [Interval::zero(); 3],
                perimeter: 20.0,
                wall: Some(WallChartQ {
                    major: pt(1.2),
                    minor: pt(0.275),
                    rims: vec![],
                    meridians,
                    ends,
                }),
            }
        };
        let run =
            |f: &SpiricFaceQ| spiric_face_rounds::<f64>(f, tol.eps(), band, RoundWindow::SCHEDULE);
        let closed = run(&face(wall(0.05, 0.0, tau), true));
        assert!(closed.is_ok(), "a closed lift measures: {closed:?}");
        let winds = run(&face(wall(0.05, 0.0, tau), false));
        assert!(
            matches!(winds, Err(PropsError::LoopsWindChart)),
            "the oval alone winds the tube: {winds:?}"
        );
        // An arc closed by a meridian in level but at its START's
        // azimuth, so its end meets the meridian `≈ 0.0107` away in `u`.
        let mut f = face(wall(0.05, 0.3, 2.1), true);
        if let Some(chart) = f.wall.as_mut() {
            let u_start = chart.ends[0].start.0;
            chart.ends[1].start.0 = u_start;
            chart.ends[1].end.0 = u_start;
        }
        let open_u = run(&f);
        assert!(
            matches!(open_u, Err(PropsError::ChartLoopOpen)),
            "open in u: {open_u:?}"
        );
        // The oval twice round winds the tube twice.
        let mut twice = face(wall(0.05, 0.0, tau), false);
        if let Some(chart) = twice.wall.as_mut() {
            chart.ends.push(chart.ends[0]);
        }
        twice.spirics.push(twice.spirics[0]);
        let twice = run(&twice);
        assert!(
            matches!(twice, Err(PropsError::LoopsWindChart)),
            "twice round the tube: {twice:?}"
        );
        // A meridian back that starts `2ε` round the tube from the
        // oval's end: within the vertex slack, so the loop closes.
        let mut near = face(wall(0.05, 0.0, tau), true);
        if let Some(chart) = near.wall.as_mut() {
            chart.ends[1].start.1 = chart.ends[1].start.1 + pt(2.0 * tol.eps() / 0.275);
        }
        let near = run(&near);
        assert!(near.is_ok(), "open by 2ε at the junction: {near:?}");
    }

    /// **Non-thin endpoint brackets**: the enclosure holds the integral
    /// for every endpoint pair inside the brackets, and each thin
    /// reading — the pad is the only thing covering the bracket.
    #[test]
    fn a_non_thin_endpoint_bracket_holds_every_endpoint_inside_it() {
        let (big, r, d) = (1.2, 0.225, 0.05);
        let f = |t: f64| ((big + r * t.cos()).powi(2) - d * d).sqrt();
        let (e, _) = framed(
            (big, r, d),
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0; 3],
            [0.0; 3],
            (0.3, 2.1),
            true,
        );
        let e = SpiricEdgeQ {
            t0: Interval::from_bounds(0.29, 0.31),
            t1: Interval::from_bounds(2.05, 2.15),
            ..e
        };
        let w = WallImageQ {
            sense: -1.0,
            v0: pt(0.0),
            env: Interval::zero(),
        };
        let chart = WallChartQ {
            ends: Vec::new(),
            major: pt(big),
            minor: pt(r),
            rims: vec![],
            meridians: vec![],
        };
        let j = cos_moment(&e, 64);
        let va = spiric_vector_area(&e, 64);
        let wb = spiric_wall_terms(&e, &w, &chart, 64);
        for (x0, x1) in [
            (0.29, 2.05),
            (0.31, 2.15),
            (0.29, 2.15),
            (0.31, 2.05),
            (0.3, 2.1),
        ] {
            let want = gl(x0, x1, 2000, |t| f(t) * t.cos());
            assert!(j.contains(want), "J {j:?} misses {want} on [{x0}, {x1}]");
            let thin = SpiricEdgeQ {
                t0: pt(x0),
                t1: pt(x1),
                ..e
            };
            let (tv, tw) = (
                spiric_vector_area(&thin, 256),
                spiric_wall_terms(&thin, &w, &chart, 256),
            );
            for k in 0..3 {
                assert!(
                    va[k].lo() <= tv[k].lo() && tv[k].hi() <= va[k].hi(),
                    "va {k}"
                );
            }
            for k in 0..4 {
                assert!(
                    wb[k].lo() <= tw[k].lo() && tw[k].hi() <= wb[k].hi(),
                    "wall {k}"
                );
            }
        }
    }

    /// **[`trig_range`] holds `(cos t, sin t)` at every sampled `t` of
    /// its range**, ends included, over ranges up to the whole circle:
    /// the widening is what lets a piece's remainder read `g₄` over the
    /// piece rather than at its midpoint.
    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn the_trig_range_holds_cos_and_sin_across_its_range() {
        let mut g = Lcg(31);
        for _ in 0..200 {
            let lo = g.range(-7.0, 7.0);
            let hi = lo + g.range(0.0, 2.5);
            let (c, s) = trig_range(lo, hi);
            for i in 0..=32 {
                let t = lo + (hi - lo) * (i as f64 / 32.0);
                let slack = 4.0 * f64::EPSILON;
                assert!(
                    c.lo() - slack <= t.cos() && t.cos() <= c.hi() + slack,
                    "cos {t} in [{lo}, {hi}]: {c:?}"
                );
                assert!(
                    s.lo() - slack <= t.sin() && t.sin() <= s.hi() + slack,
                    "sin {t} in [{lo}, {hi}]: {s:?}"
                );
            }
        }
    }

    /// **The map-residual pad pays the area the envelope can move**:
    /// with a synthetic envelope `e` (a spiric's own wall image has
    /// none), the flux and area widths grow by at least
    /// `2·(chart length)·e/min(R − r, r)` times `r(R + r)²` and `r(R + r)`
    /// — the chart length from the SAMPLED `sup|u′|` of the cut — and by
    /// no more than sixteen times that: the rate bound pairs the span's
    /// worst `|sin t|` with its worst `f`, which a near-tangent span never
    /// has at once (measured `8×` here). A near-tangent cut
    /// (`f_min = 0.05`) on a torus with `(R + r)² = 18`, so a dropped
    /// density factor or a rate bound that misses the cut's `1/f` lands
    /// outside the window.
    #[test]
    #[allow(clippy::cast_precision_loss)]
    fn the_map_residual_pad_pays_the_area_the_envelope_moves() {
        let (big, r, t0, t1, env) = (3.0_f64, 1.25_f64, 2.6_f64, 3.6_f64, 1e-6_f64);
        let d = ((big - r).powi(2) - 0.05_f64.powi(2)).sqrt();
        let (e, _) = framed(
            (big, r, d),
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0; 3],
            [0.0; 3],
            (t0, t1),
            true,
        );
        let du = |t: f64| {
            let rho = big + r * t.cos();
            (d * r * t.sin() / (rho * (rho * rho - d * d).sqrt())).abs()
        };
        let sup = (0..=4000)
            .map(|i| du(t0 + (t1 - t0) * (i as f64 / 4000.0)))
            .fold(0.0_f64, f64::max);
        let defect = (sup + 1.0) * (t1 - t0) * env / (big - r).min(r);
        let widths = |env: f64| {
            let face = SpiricFaceQ {
                closed_va: [Interval::zero(); 3],
                spirics: vec![SpiricEdgeQ {
                    wall: Some(WallImageQ {
                        sense: 1.0,
                        v0: pt(0.0),
                        env: pt(env),
                    }),
                    ..e
                }],
                lever: [Interval::zero(); 3],
                perimeter: 1.0,
                wall: Some(WallChartQ {
                    ends: Vec::new(),
                    major: pt(big),
                    minor: pt(r),
                    rims: vec![],
                    meridians: vec![],
                }),
            };
            let chart = face.wall.clone().unwrap();
            let got = wall_chart(&face, &chart, 64).unwrap();
            (got[0].width(), got[1].width())
        };
        let ((f1, a1), (f0, a0)) = (widths(env), widths(0.0));
        for (name, grew, density) in [
            ("flux", f1 - f0, r * (big + r).powi(2)),
            ("area", a1 - a0, r * (big + r)),
        ] {
            let want = 2.0 * defect * density;
            assert!(
                want <= grew && grew <= 16.0 * want,
                "{name}: the pad grew the width by {grew}, the envelope moves {want}"
            );
        }
    }

    /// **Near-tangent cuts stay sound** as `f_min` falls to `1e-4`, at
    /// the schedule's coarse and fine ends: a certified `J` holds the
    /// reference (a 5-point rule on 200 000 pieces, good to `1e-9`
    /// there).
    #[test]
    fn near_tangent_cuts_stay_sound_as_f_min_falls() {
        for fmin in [1e-2, 1e-3, 1e-4] {
            let (big, r) = (1.0, 0.5);
            let d = (0.25_f64 - fmin * fmin).sqrt();
            let f = |t: f64| ((big + r * t.cos()).powi(2) - d * d).max(0.0).sqrt();
            let want = gl(2.0, 4.5, 200_000, |t| f(t) * t.cos());
            for pieces in [16, 1024] {
                let (e, _) = framed(
                    (big, r, d),
                    [0.0, 0.0, 1.0],
                    [1.0, 0.0, 0.0],
                    [0.0; 3],
                    [0.0; 3],
                    (2.0, 4.5),
                    true,
                );
                let j = cos_moment(&e, pieces);
                if j.is_certified() {
                    assert!(
                        j.lo() - 1e-9 <= want && want <= j.hi() + 1e-9,
                        "f_min {fmin} at {pieces}: {j:?} vs {want}"
                    );
                }
            }
        }
    }
}

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
//! the node and over the piece come from one Taylor jet ([`Jet`],
//! [`super::taylor`]'s rule) in certification arithmetic; the spiric's
//! integrands need, past the ring operations, only `√` (admitted by C9),
//! and a rational rim's the reciprocal ([`super::rational_rim`]); a piece's width is `g₄`'s width over it times
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
use geom_core::{Band, Bounds, Decide};

use super::PropsError;
use super::bracket::{AbsEnclosure, pt};
use super::closure::{IntervalJunction, interval_loop_turns};
use super::quad::{
    Accept, EDGE_INIT_PIECES, FaceCutBounds, LaneRun, RVec3, RoundWindow, Run, face_extent, rotate,
    spread_cos, spread_sin, trig_at,
};
use super::taylor::{Jet, composite_by, node};

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

/// **The one home of a spiric edge's rate bounds over a window `[lo, hi]`
/// of its parameter**, `sup|sin t|` and `inf f` read apart:
/// [`geom::spiric_rate_bounds`] over `ρ = R + r·cos t` and `|sin t|` off
/// [`trig_range`]. Returns the window's `cos t`, its `ρ` and the speed
/// bound `|P′| ≤ S`. `None` when the window's `f` is not certified
/// positive (`ρ_lo ≤ |d|`).
fn window_rates(e: &SpiricEdgeQ, lo: f64, hi: f64) -> Option<(Interval, Interval, Interval)> {
    if !(e.minor.is_certified() && e.major.is_certified() && e.offset.is_certified()) {
        return None;
    }
    let (c, s) = trig_range(lo, hi);
    let rho = e.major + e.minor * c;
    if !(rho.is_certified() && s.is_certified()) {
        return None;
    }
    let reach = rho.lo() - e.offset.mag();
    if reach.is_nan() || reach <= 0.0 {
        return None;
    }
    let (speed, _) =
        geom::spiric_rate_bounds(e.minor, e.offset, (pt(rho.lo()), pt(rho.hi())), pt(s.mag()));
    Some((c, rho, speed))
}

/// A spiric edge's rate bounds over its whole span `[t₀.lo, t₁.hi]`
/// ([`window_rates`]): the speed `|P′| ≤ S` and the wall image's azimuth
/// rate `|u′| = |d|·r·|sin t|/(ρ·f) ≤ √(S² − r²)/ρ_lo` (since
/// `S² = r²(1 + d²sin²/f²)` at the window's worst point). A cut whose
/// plane passes the tube's hole (`|d| > R − r`) has `f` real on its own
/// arc though not on the whole circle, and is bounded here by that arc.
/// `None` when the window's `f` is not certified positive.
pub fn span_rates(e: &SpiricEdgeQ) -> Option<(Interval, Interval)> {
    if !(e.t0.is_certified() && e.t1.is_certified()) {
        return None;
    }
    let (_, rho, speed) = window_rates(e, e.t0.lo(), e.t1.hi())?;
    if !speed.is_certified() {
        return None;
    }
    let du = (speed.sqr() - e.minor.sqr())
        .clamped_to(0.0, f64::INFINITY)
        .sqrt()
        / pt(rho.lo());
    du.is_certified().then_some((speed, du))
}

/// The piece count a face's perimeter reads its spiric edges' lengths
/// at ([`spiric_length_bound`]). The perimeter is only the extent
/// gate's lever, read once per face with no refinement of its own, so
/// it takes one fixed count: four times the 16 pieces at which the
/// bound already holds within `1.1×` the arc at the pinch.
pub const LENGTH_PIECES: usize = 64;

/// **An upper bound on a spiric edge's length**: `[t₀.lo, t₁.hi]` in
/// `pieces` equal pieces, each charged its width times [`piece_speed`].
/// The bound tightens toward the arc's length as `pieces` grows, the
/// pinch's piece included. `None` when a piece's `f` is not certified
/// positive, or `pieces` is `0`.
#[must_use]
pub fn spiric_length_bound(e: &SpiricEdgeQ, pieces: usize) -> Option<f64> {
    if !(e.t0.is_certified() && e.t1.is_certified()) || pieces == 0 {
        return None;
    }
    let (a, b) = (e.t0.lo(), e.t1.hi());
    if !((b - a).is_finite() && b >= a) {
        return None;
    }
    (0..pieces).try_fold(0.0, |len, i| {
        let (p, q) = (node(a, b, pieces, i), node(a, b, pieces, i + 1));
        let piece = (pt(piece_speed(e, p, q)?) * (pt(q) - pt(p))).hi();
        Some(certified_hi(pt(len) + pt(piece)))
    })
}

/// **`sup|P′|` over `t ∈ [p, q]`**, the smaller of two readings: the
/// window's own, `sup|sin t|` and `inf f` apart ([`window_rates`]), and
/// one with `sin t` and `f` together.
/// `|P′|² = r²(1 + d²sin²t/f²)` ([`geom::spiric_rate_bounds`]); with
/// `x = 1 + cos t`, `sin²t = x(2 − x)` and
/// `f² = (ρ − |d|)(ρ + |d|)`, `ρ − |d| = δ + r·x`, `δ = R − r − |d|`:
///
/// ```text
/// d²sin²t/f² = d²·(2 − x)·φ(x)/(ρ + |d|),   φ(x) = x/(δ + r·x)
/// ```
///
/// `φ` is monotone in `x` wherever `δ + r·x > 0` (its derivative has
/// `δ`'s sign), so its sup over the piece is at one end of the piece's
/// `x`. Near the pinch (`ρ → |d|` as `x → 0`) the factor `x` cancels
/// the vanishing `f²` — `φ ≤ 1/r` for `δ ≥ 0` — where the apart reading
/// grows like `1/f`. Away from it the together reading pairs
/// `(2 − x)`'s sup with `φ`'s, up to 4 times `sin²t` over a wide piece,
/// where the apart reading is near exact. The same derivation for
/// [`geom::spiric_rate_bounds`] itself is CURVED's
/// `the-whole-period-spiric-speed-bound-reads-sin-and-f-apart`. `None`
/// when the piece's `f` is not certified positive.
fn piece_speed(e: &SpiricEdgeQ, p: f64, q: f64) -> Option<f64> {
    let (c, rho, apart) = window_rates(e, p, q)?;
    let d_abs = e.offset.sqr().sqrt();
    let x = (Interval::one() + c).clamped_to(0.0, 2.0);
    let delta = e.major - e.minor - d_abs;
    let phi = |x: f64| {
        let den = delta + e.minor * pt(x);
        (den.is_certified() && den.lo() > 0.0).then(|| (pt(x) / den).hi())
    };
    let together = phi(x.lo()).zip(phi(x.hi())).map(|(lo, hi)| {
        let ratio = e.offset.sqr() * (pt(2.0) - x) * pt(lo.max(hi)) / (rho + d_abs);
        e.minor * (Interval::one() + ratio).sqrt()
    });
    [Some(apart), together]
        .into_iter()
        .flatten()
        .filter(|v| v.is_certified())
        .map(Bounds::hi)
        .reduce(f64::min)
}

/// `∫_a^b g` for `N` integrands at once, `pieces` equal pieces (the rule
/// in the module docs), plus the endpoint-bracket pad: `a`, `b` are the
/// brackets' midpoints, and `widths` their summed half-widths, paid at
/// `sup|g|` over the span's own window; and per integrand the summed
/// width of its remainder terms `g₄·∫(t − x)⁴`, the part of its width a
/// later round removes. The integrands see `t`, `cos t` and `sin t` as
/// jets ([`composite_by`] over them).
fn composite<const N: usize>(
    t0: Interval,
    t1: Interval,
    pieces: usize,
    g: impl Fn(Jet, Jet, Jet) -> [Jet; N],
) -> ([Interval; N], [f64; N]) {
    composite_by(
        t0,
        t1,
        pieces,
        |_, _| (),
        |(), t| {
            let (c, s) = trig_range(t.lo(), t.hi());
            let (tj, cj, sj) = trig_jets(t, c, s);
            g(tj, cj, sj)
        },
    )
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
    /// Every edge of each of its loops on the lift, loop by loop, each in
    /// traversal order: what the closure decision reads
    /// ([`wall_windings`]).
    pub ends: Vec<Vec<ChartEndsQ>>,
    /// How many of the face's spirics, and of `rims` and `meridians`,
    /// each loop holds, loop by loop in [`Self::ends`]' order, each list
    /// stored loop after loop: what the rings' winding reads
    /// ([`require_rings_wound_against`]).
    pub parts: Vec<WallLoopParts>,
}

/// One torus wall loop's share of its face's edge lists
/// ([`WallChartQ::parts`]).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WallLoopParts {
    /// Its spiric edges ([`RimFaceQ::spirics`]).
    pub spirics: usize,
    /// Its rims ([`WallChartQ::rims`]).
    pub rims: usize,
    /// Its meridians ([`WallChartQ::meridians`]).
    pub meridians: usize,
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
pub struct RimFaceQ {
    /// The closed-form edges' vector-area terms, about the reference.
    pub closed_va: RVec3,
    /// The spiric edges, centres relative to the reference.
    pub spirics: Vec<SpiricEdgeQ>,
    /// A cap's rational spline rim edges, about the reference
    /// ([`super::rational_rim`]); empty on a wall.
    pub rationals: Vec<super::rational_rim::RationalEdgeQ>,
    /// The flux lever.
    pub lever: RVec3,
    /// An upper bound on the boundary's length (metres) — the extent
    /// gate's lever.
    pub perimeter: f64,
    /// The torus chart, for a wall; `None` for a cap.
    pub wall: Option<WallChartQ>,
}

/// `J = ∫ f(t)·cos t dt` over the edge's carrier interval.
#[cfg(test)]
fn cos_moment(e: &SpiricEdgeQ, pieces: usize) -> Interval {
    cos_moment_split(e, pieces).0
}

/// `cos_moment`, and the part of its width a later round removes.
fn cos_moment_split(e: &SpiricEdgeQ, pieces: usize) -> (Interval, f64) {
    let (big, r, d) = (e.major, e.minor, e.offset);
    let ([j], [removable]) = composite(e.t0, e.t1, pieces, |_, c, _| {
        let (_, f) = rho_f(big, r, d, c);
        [f.mul(c)]
    });
    (j, removable)
}

/// One spiric edge's vector-area term about the reference (module
/// docs), at `pieces` resolution.
#[must_use]
pub fn spiric_vector_area(e: &SpiricEdgeQ, pieces: usize) -> RVec3 {
    vector_area_split(e, pieces).0
}

/// [`spiric_vector_area`], and the part of its moment `J`'s width a
/// later round removes: the term carries `J` as `n·r·J`.
fn vector_area_split(e: &SpiricEdgeQ, pieces: usize) -> (RVec3, f64) {
    let (big, r, d) = (e.major, e.minor, e.offset);
    let (j, removable) = cos_moment_split(e, pieces);
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
    (if e.forward { va } else { va.map(|x| -x) }, removable)
}

/// One spiric wall edge's Green-form terms `(−∫F du, −∫G du, Δu, Δv)`
/// on its lift (module docs), traversal folded in.
#[cfg(test)]
fn spiric_wall_terms(
    e: &SpiricEdgeQ,
    w: &WallImageQ,
    chart: &WallChartQ,
    pieces: usize,
) -> [Interval; 4] {
    wall_terms_split(e, w, chart, pieces).0
}

/// `spiric_wall_terms`, and the part of the flux term's width a later
/// round removes.
fn wall_terms_split(
    e: &SpiricEdgeQ,
    w: &WallImageQ,
    chart: &WallChartQ,
    pieces: usize,
) -> ([Interval; 4], f64) {
    let (big, r) = (chart.major, chart.minor);
    let d = e.offset;
    let sigma = pt(w.sense);
    let k = sigma * d * r;
    let rr = big.sqr() + r.sqr();
    let three_half = pt(1.5) * big * r;
    let half_br = pt(0.5) * big * r;
    let v0 = w.v0;
    let ([flux, area, du], [removable, _, _]) = composite(e.t0, e.t1, pieces, |t, c, s| {
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
    (if e.forward { terms } else { terms.map(|x| -x) }, removable)
}

/// `F(V)` and `G(V)` at a lifted level (the closed-form lane's
/// antiderivatives).
fn rim_fg(big: Interval, r: Interval, v: Interval) -> (Interval, Interval) {
    let (c, s) = trig_of(v);
    let f = r * ((big.sqr() + r.sqr()) * s + pt(1.5) * big * r * v + pt(0.5) * big * r * s * c);
    let g = r * (big * v + r * s);
    (f, g)
}

/// The wall's chart terms at `pieces`, loop by loop
/// ([`WallChartQ::parts`]): each loop's `(flux, signed area, Σ Δu, Σ Δv)`
/// with its share of the map-residual pad folded into the first two.
/// The face's terms are their sum.
#[cfg(test)]
fn wall_chart(
    face: &RimFaceQ,
    chart: &WallChartQ,
    pieces: usize,
) -> Result<Vec<[Interval; 4]>, PropsError> {
    wall_chart_split(face, chart, pieces).map(|(loops, _)| loops)
}

/// `wall_chart`, and the part of the face's chart flux width a later
/// round removes.
fn wall_chart_split(
    face: &RimFaceQ,
    chart: &WallChartQ,
    pieces: usize,
) -> Result<(Vec<[Interval; 4]>, f64), PropsError> {
    let (big, r) = (chart.major, chart.minor);
    let held = chart
        .parts
        .iter()
        .fold(WallLoopParts::default(), |n, p| WallLoopParts {
            spirics: n.spirics + p.spirics,
            rims: n.rims + p.rims,
            meridians: n.meridians + p.meridians,
        });
    if held
        != (WallLoopParts {
            spirics: face.spirics.len(),
            rims: chart.rims.len(),
            meridians: chart.meridians.len(),
        })
    {
        return Err(PropsError::QuadratureUnsupported {
            what: "a spiric torus wall whose loops' shares do not partition its edges",
        });
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
    let outer = big + r;
    let (mut spirics, mut rims, mut meridians) = (
        face.spirics.iter(),
        chart.rims.iter(),
        chart.meridians.iter(),
    );
    let mut loops = Vec::with_capacity(chart.parts.len());
    let mut removable = 0.0_f64;
    for part in &chart.parts {
        let mut acc = [Interval::zero(); 4];
        // Σ (chart length · envelope), for the map-residual pad.
        let mut defect = Interval::zero();
        for e in spirics.by_ref().take(part.spirics) {
            let Some(w) = e.wall.as_ref() else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a spiric edge of a torus wall carries no lifted wall image",
                });
            };
            let (t, rem) = wall_terms_split(e, w, chart, pieces);
            removable += rem;
            for k in 0..4 {
                acc[k] = acc[k] + t[k];
            }
            defect = defect + spiric_chart_length(e)? * w.env;
        }
        for rim in rims.by_ref().take(part.rims) {
            let (f, g) = rim_fg(big, r, rim.v);
            acc[0] = acc[0] - f * rim.du;
            acc[1] = acc[1] - g * rim.du;
            acc[2] = acc[2] + rim.du;
            defect = defect + pt(rim.du.mag()) * rim.env;
        }
        for mer in meridians.by_ref().take(part.meridians) {
            acc[3] = acc[3] + mer.dv;
            defect = defect + pt(mer.dv.mag()) * mer.env;
        }
        let uv = (defect / pt(floor)).mag();
        let pad_flux = (pt(uv) * r * outer.sqr()).mag();
        let pad_area = (pt(uv) * r * outer).mag();
        acc[0] = acc[0] + Interval::from_bounds(-pad_flux, pad_flux);
        acc[1] = acc[1] + Interval::from_bounds(-pad_area, pad_area);
        loops.push(acc);
    }
    Ok((loops, removable))
}

/// A spiric wall edge's chart length from above: `|u′|` over its span,
/// and `|v′| = 1`.
fn spiric_chart_length(e: &SpiricEdgeQ) -> Result<Interval, PropsError> {
    let Some((_, du_max)) = span_rates(e) else {
        return Err(PropsError::QuadratureUnsupported {
            what: "a spiric wall edge whose span's rate window has no certified \
                   positive f",
        });
    };
    Ok((du_max + Interval::one()) * (e.t1 - e.t0))
}

/// **Every ring of a torus wall winds against its face**: each loop
/// after the first is a hole cut from the face, so its signed chart
/// area (its `wall_chart` terms at `pieces`) must have the sign
/// opposite to the face's whole, as the cylinder's and the planar
/// lanes' rings must (`curved::require_holes_wound_against`, under
/// `props_ring_winding`). A ring wound with its face would be added to
/// the face's area, silently, and refuses
/// [`PropsError::RingWoundWithFace`]; a whole or ring of no width is a
/// [`PropsError::DegenerateFace`]. Every wall loop winds the torus not
/// at all ([`wall_windings`]), so the first loop stored is the face's
/// own, as on a cylinder face with no winding loop.
///
/// Each is read as its mean width `2·A/P`, `P` its metric length from
/// above: `(R + r)` times its chart length (a spiric's
/// [`spiric_chart_length`], a rim's `|Δu|`), and a meridian's `r·|Δv|`.
/// Where a ring lies is not read here: a ring outside its face, or
/// crossing it, is check 9's containment.
fn require_rings_wound_against(
    face: &RimFaceQ,
    chart: &WallChartQ,
    loops: &[[Interval; 4]],
    band: Band,
) -> Result<(), PropsError> {
    if loops.len() < 2 {
        return Ok(());
    }
    let (big, r) = (chart.major, chart.minor);
    let outer = big + r;
    let (mut spirics, mut rims, mut meridians) = (
        face.spirics.iter(),
        chart.rims.iter(),
        chart.meridians.iter(),
    );
    let mut lengths = Vec::with_capacity(chart.parts.len());
    for part in &chart.parts {
        let mut chart_len = Interval::zero();
        for e in spirics.by_ref().take(part.spirics) {
            chart_len = chart_len + spiric_chart_length(e)?.abs_enclosure();
        }
        for rim in rims.by_ref().take(part.rims) {
            chart_len = chart_len + rim.du.abs_enclosure();
        }
        let mut length = outer * chart_len;
        for mer in meridians.by_ref().take(part.meridians) {
            length = length + r * mer.dv.abs_enclosure();
        }
        lengths.push(length);
    }
    let whole = loops
        .iter()
        .zip(&lengths)
        .fold((Interval::zero(), Interval::zero()), |(a, l), (t, &len)| {
            (a + t[1], l + len)
        });
    let holes: Vec<(Interval, Interval)> = loops[1..]
        .iter()
        .zip(&lengths[1..])
        .map(|(t, &len)| (t[1], len))
        .collect();
    super::curved::require_holes_wound_against("props_chart_area_side", whole, &holes, band)
}

/// The face's enclosures at `pieces` resolution, a wall's chart terms
/// loop by loop (`wall_chart`; none for a cap), and the part of the
/// flux's width a later round removes: each spiric's moment `J`, which
/// the flux reads through the lever as `Σₖ leverₖ·nₖ·r·J` — component by
/// component, as the interval sum carries `J`'s width, so a lever in the
/// cap's own plane still carries it — and a wall's chart flux.
/// How a cap's area reads its vector area `A⃗`: `‖A⃗‖` on a plane, and
/// `|â·A⃗|/sin α` on a cone, whose every normal makes the angle `π/2 − α`
/// with its axis `â`.
#[derive(Clone, Copy, Debug)]
enum CapArea {
    Plane,
    Cone { axis: RVec3, sin_a: Interval },
}

fn face_bounds(
    face: &RimFaceQ,
    cap: CapArea,
    pieces: usize,
) -> Result<(FaceCutBounds, Vec<[Interval; 4]>, f64), PropsError> {
    let mut va = face.closed_va;
    let mut removable = 0.0_f64;
    for e in &face.spirics {
        let (t, rem) = vector_area_split(e, pieces);
        removable += (0..3)
            .map(|k| (face.lever[k] * e.normal[k]).mag())
            .sum::<f64>()
            * e.minor.mag()
            * rem;
        va = [0, 1, 2].map(|k| va[k] + t[k]);
    }
    for e in &face.rationals {
        let (t, rem) = super::rational_rim::rational_vector_area_split(e, pieces);
        // The flux reads each component through the lever.
        removable += (0..3).map(|k| face.lever[k].mag() * rem[k]).sum::<f64>();
        va = [0, 1, 2].map(|k| va[k] + t[k]);
    }
    let position = dot(&face.lever, &va);
    Ok(match &face.wall {
        None => (
            FaceCutBounds {
                flux: position,
                area: match cap {
                    CapArea::Plane => (va[0].sqr() + va[1].sqr() + va[2].sqr()).sqrt(),
                    CapArea::Cone { axis, sin_a } => dot(&axis, &va).abs_enclosure() / sin_a,
                },
            },
            Vec::new(),
            removable,
        ),
        Some(chart) => {
            let (loops, chart_rem) = wall_chart_split(face, chart, pieces)?;
            let (flux, area) = loops
                .iter()
                .fold((Interval::zero(), Interval::zero()), |(f, a), t| {
                    (f + t[0], a + t[1])
                });
            (
                FaceCutBounds {
                    flux: flux + position,
                    area: area.abs_enclosure(),
                },
                loops,
                removable + chart_rem,
            )
        }
    })
}

/// **A torus wall's loops each close on its chart and wind it not at
/// all** — the periodic-chart closure decision ([`super::closure`]) on
/// each loop's lifted images ([`WallChartQ::ends`]), the outer loop and
/// every ring alike.
///
/// At each junction the azimuth's gap is metered at the junction's own
/// distance from the axis, `ρ = R + r·cos v`, enclosed over the after
/// edge's start, and the tube angle's at `r`; the slack is the two
/// images' envelopes. Reading `ρ` at one end is sound: the two ends'
/// `ρ` differ by at most `r·|Δv|`, so the azimuth term moves by at most
/// `r·|Δv|·|Δu|`, second order in a gap the decision admits. The wall's
/// Green form is read on the lifted chart, where it bounds a region only
/// when every loop closes there, so each loop's turns must be zero: a
/// face whose loops wind the torus in pairs is a band the lane does not
/// read, and refuses [`PropsError::LoopsWindChart`].
fn wall_windings<T: Decide>(chart: &WallChartQ, band: Band) -> Result<(), PropsError> {
    if chart.ends.is_empty() || chart.ends.iter().any(Vec::is_empty) {
        return Err(PropsError::QuadratureUnsupported {
            what: "a spiric torus wall handed no loop to decide closed",
        });
    }
    let (big, r) = (chart.major, chart.minor);
    for ends in &chart.ends {
        let junctions = ends
            .iter()
            .zip(ends.iter().cycle().skip(1))
            .map(|(e, next)| {
                let (cos_v, _) = trig_of(next.start.1);
                IntervalJunction {
                    delta: [e.end.0 - next.start.0, e.end.1 - next.start.1],
                    lever: [big + r * cos_v, r],
                    slack: certified_hi(e.env + next.env),
                }
            });
        let turns = interval_loop_turns::<T, 2>(junctions, [true, true], band)?;
        if turns != [0, 0] {
            return Err(PropsError::LoopsWindChart);
        }
    }
    Ok(())
}

/// Weight of one piece of one spiric edge, in
/// [`super::quad::FACE_WORK_WALL`]'s unit.
pub(super) const W_SPIRIC: usize = 46;

/// Weight of one piece of one Bézier span of a rational rim edge
/// ([`super::rational_rim`]), in [`super::quad::FACE_WORK_WALL`]'s unit,
/// by the carrier's degree: a piece shifts four power forms of the
/// degree's length to the piece and reads them by Horner twice, so its
/// cost grows with the degree. Measured in release on this box (a timing
/// probe, reverted), 50 calls of 256 pieces on one span of each degree,
/// against a spiric piece's 13.2 µs measured on the same box, which
/// [`W_SPIRIC`] weighs at 46: the weight is the time in that ratio,
/// rounded up. Past degree 7, each degree adds the last step's 60.
///
/// | degree | µs a piece | weight |
/// |---|---|---|
/// | 1 | 34.5 | 120 |
/// | 2 | 44.5 | 155 |
/// | 3 | 63.0 | 220 |
/// | 4 | 77.4 | 270 |
/// | 5 | 90.6 | 316 |
/// | 6 | 116 | 404 |
/// | 7 | 132 | 460 |
pub(super) const fn w_rational(degree: usize) -> usize {
    match degree {
        0 | 1 => 120,
        2 => 155,
        3 => 220,
        4 => 270,
        5 => 316,
        6 => 404,
        7 => 460,
        _ => 460 + 60 * (degree - 7),
    }
}

/// The work of round `round` on a face of `spirics` spiric edges and
/// rational rims weighing `rational` in all ([`w_rational`] per span).
pub(super) fn round_cost(spirics: usize, rational: usize, round: usize) -> usize {
    (W_SPIRIC * spirics + rational) * (EDGE_INIT_PIECES << round)
}

/// **The certified flux and area of a spiric-bounded face** over a
/// [`RoundWindow`] — [`super::quad`]'s schedule and stop (pieces
/// doubling from `EDGE_INIT_PIECES`, the caller's [`Accept`], the
/// lane's floor and work wall, the face-extent gate on the accepted
/// round).
///
/// On a wall, every loop, the outer one and each ring, must close and
/// wind the torus not at all ([`wall_windings`]): the Green form, summed
/// over them all, bounds a region of the lifted chart only then. Each
/// ring must also be wound against the face at the round the call ends
/// ([`require_rings_wound_against`]), or it would add itself to the
/// face's area; an accepted face whose ring straddles the band refines
/// on, and the escalation stands at the last round the window runs, or
/// at once for a ring wholly in the band (`quad::Run::ringed`).
///
/// # Errors
///
/// [`PropsError`] — `accept`'s own refusal, an escalated funnel
/// decision, a degenerate face, a wall loop that does not close
/// ([`PropsError::ChartLoopOpen`]) or winds the torus
/// ([`PropsError::LoopsWindChart`]), or a ring wound with its face
/// ([`PropsError::RingWoundWithFace`]).
pub fn rim_face_rounds<T: Decide>(
    face: &RimFaceQ,
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    rounds::<T>(face, CapArea::Plane, band, window, accept)
}

/// **A cone cap bounded by a rational spline rim**: [`rim_face_rounds`]
/// on `face` read about the apex (`lever` the apex less the centre, every
/// term about the apex), whose area is `|â·A⃗|/sin α`. On a cone `x − A`
/// lies along the generator, in the tangent plane, so the position flux
/// is `A·∫n dS = A·A⃗`, and `A⃗ = ½∮(x − A) × dx` is the boundary's.
///
/// # Errors
///
/// As [`rim_face_rounds`] on a cap.
pub fn cone_cap_rounds<T: Decide>(
    face: &RimFaceQ,
    (axis, sin_a): (RVec3, Interval),
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    if face.wall.is_some() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a cone cap handed a torus wall's chart",
        });
    }
    rounds::<T>(face, CapArea::Cone { axis, sin_a }, band, window, accept)
}

fn rounds<T: Decide>(
    face: &RimFaceQ,
    cap: CapArea,
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    if let Some(chart) = &face.wall {
        wall_windings::<T>(chart, band)?;
        if chart.parts.len() != chart.ends.len() {
            return Err(PropsError::QuadratureUnsupported {
                what: "a spiric torus wall whose loops' shares are not one per loop",
            });
        }
    }
    let rational: usize = face
        .rationals
        .iter()
        .map(|e| w_rational(e.degree()) * e.span_count())
        .sum();
    let cost = |round: usize| round_cost(face.spirics.len(), rational, round);
    let mut run = Run::enter(window, &cost);
    let mut round = run.first();
    let mut pieces = EDGE_INIT_PIECES << round;
    loop {
        let (bounds, loops, removable) = face_bounds(face, cap, pieces)?;
        let (stop, accepted) = run.ask(round, bounds, removable, accept, || {
            face_extent::<T>(bounds.area, face.perimeter, band)
        })?;
        if let Some(out) = run.ringed(round, stop, accepted, bounds, || {
            face.wall.as_ref().map_or(Ok(()), |chart| {
                require_rings_wound_against(face, chart, &loops, band)
            })
        })? {
            return Ok(out);
        }
        round += 1;
        pieces *= 2;
    }
}

#[cfg(test)]
mod length_fuzz;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::props::quad::test_target::to_target;
    use crate::props::quad::{LaneEnd, LaneStop};

    /// One loop's share of a face's edge lists.
    fn part(spirics: usize, rims: usize, meridians: usize) -> WallLoopParts {
        WallLoopParts {
            spirics,
            rims,
            meridians,
        }
    }

    /// The parts of a wall of one loop.
    fn one_loop(spirics: usize, rims: usize, meridians: usize) -> Vec<WallLoopParts> {
        vec![part(spirics, rims, meridians)]
    }

    /// A wall's chart terms summed over its loops ([`wall_chart`]).
    fn summed(loops: Vec<[Interval; 4]>) -> [Interval; 4] {
        loops.iter().fold([Interval::zero(); 4], |acc, t| {
            [0, 1, 2, 3].map(|k| acc[k] + t[k])
        })
    }

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
            parts: Vec::new(),
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

    /// **A lever in the cap's own plane still carries the moment's
    /// width.** The flux reads `J` through `Σₖ leverₖ·nₖ·r·J`, component
    /// by component, so with the oval's normal along `(1, −1, 0)` and a
    /// lever along `(1, 1, 0)` the real product `lever·n` is zero while
    /// each component carries `J`'s width. The lane's removable part
    /// must read it component-wise: read as `|lever·n|` it would be
    /// zero, and the cap would stop at its floor at round 0 with every
    /// round's width still removable.
    #[test]
    fn a_lever_in_the_caps_plane_keeps_the_moments_width_removable() {
        let (_, (c, _)) = cases().swap_remove(5);
        let (mut e, ..) = edge(c);
        let h = pt(core::f64::consts::FRAC_1_SQRT_2);
        e.normal = [h, -h, Interval::zero()];
        let face = RimFaceQ {
            rationals: Vec::new(),
            closed_va: [Interval::zero(); 3],
            spirics: vec![e],
            lever: [pt(3.0), pt(3.0), Interval::zero()],
            perimeter: 1.0,
            wall: None,
        };
        let mut first = None;
        let _ = rim_face_rounds::<f64>(
            &face,
            Band::linear(geom_core::Tol::witness()).unwrap(),
            RoundWindow::SCHEDULE,
            &mut |_, stop| {
                first.get_or_insert(stop);
                Ok(true)
            },
        );
        assert_eq!(
            first,
            Some(LaneStop::Refinable),
            "round 0's moment width is removable, so the cap is not at its floor"
        );
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
    pub(super) fn framed(
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
    /// terms and the length bound all certify, against the definitions;
    /// the length bound holds the arc from above, where `φ` falls with
    /// `x` ([`piece_speed`]).
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
        for ((big, r, d), span) in [
            ((big, r, d), (t0, t1)),
            (
                (
                    1.746_019_885_272_977_8,
                    0.776_110_402_223_510_8,
                    -1.179_434_705_286_893_2,
                ),
                (3.914_462_384_376_397_7, 7.990_064_206_637_532),
            ),
        ] {
            let (e, _) = framed(
                (big, r, d),
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0; 3],
                [0.0; 3],
                span,
                true,
            );
            let arc = walked((big, r, d), span, 4000);
            for pieces in [16, 128] {
                let bound = spiric_length_bound(&e, pieces).unwrap();
                assert!(
                    arc <= bound,
                    "({big}, {r}) cut at {d}, {span:?}, {pieces} pieces: the length bound \
                     {bound} under the arc's {arc}"
                );
            }
        }
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
            parts: one_loop(1, 0, 0),
            ends: Vec::new(),
            major: pt(big),
            minor: pt(r),
            rims: vec![],
            meridians: vec![],
        };
        let face = RimFaceQ {
            rationals: Vec::new(),
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

    /// The length of the polygon through `n + 1` evenly spaced points of
    /// the spiric `(R, r, d)` over `[t0, t1]`, less `1e-12` of itself for
    /// rounding: under the arc's own length. Each side is read in the
    /// oval's plane, `(r·sin t, f)`, from differences that keep their
    /// digits at the pinch: `Δ sin t` and `Δρ` by sum-to-product,
    /// `Δf = Δρ·(ρ_a + ρ_b)/(f_a + f_b)`, and `f² = (ρ − |d|)(ρ + |d|)`
    /// with `ρ − |d| = δ + 2r·cos²(t/2)`, `δ = R − r − |d|` from the
    /// exact two-sum of `R − r`.
    pub(super) fn walked((big, r, d): (f64, f64, f64), (t0, t1): (f64, f64), n: u32) -> f64 {
        let s = big - r;
        let b = s - big;
        let delta = (s - d.abs()) + ((big - (s - b)) + (-r - b));
        let rho = |t: f64| big + r * t.cos();
        let f = |t: f64| {
            let c = (t / 2.0).cos();
            ((delta + 2.0 * r * c * c) * (rho(t) + d.abs())).sqrt()
        };
        let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(n);
        let len: f64 = (0..n)
            .map(|k| {
                let (ta, tb) = (at(k), at(k + 1));
                let (m, h) = ((ta + tb) / 2.0, (tb - ta) / 2.0);
                let dy = 2.0 * r * m.cos() * h.sin();
                let drho = -2.0 * r * m.sin() * h.sin();
                let dz = drho * (rho(ta) + rho(tb)) / (f(ta) + f(tb));
                dy.hypot(dz)
            })
            .sum();
        len * (1.0 - 1e-12)
    }

    /// **The length bound reads the arc at the pinch**: ovals cut `1e-3`
    /// to `1e-12` short of the inner equator, on both sides of the axis,
    /// over arcs through the pinch, along its neck and across a whole
    /// period. The bound stays above a 4000-point polygon of the arc
    /// ([`walked`]) at
    /// every piece count, and within 1.1 times it at 16 pieces and 1.02
    /// times at 128 — where `sup|sin t|` over `inf f`, read apart, is up
    /// to `10⁶` times the arc's speed.
    #[test]
    fn the_length_bound_reads_the_arc_at_the_pinch() {
        for (big, r) in [(2.0, 0.5), (3.0, 1.0), (1.0, 0.9)] {
            for gap in [1e-3, 1e-6, 1e-9, 1e-12] {
                for d in [big - r - gap, gap - big + r] {
                    for span in [
                        (PI - 0.5, PI + 0.3),
                        (PI - 0.1, PI - 1e-4),
                        (PI + 1e-6, PI + 4e-3),
                        (PI - 1e-5, PI + 1e-5),
                        (0.0, 2.0 * PI),
                    ] {
                        let (e, _) = framed(
                            (big, r, d),
                            [0.0, 1.0, 0.0],
                            [1.0, 0.0, 0.0],
                            [0.0; 3],
                            [0.0; 3],
                            span,
                            true,
                        );
                        let arc = walked((big, r, d), span, 4000);
                        for (pieces, slack) in [(1, f64::INFINITY), (16, 1.1), (128, 1.02)] {
                            let bound = spiric_length_bound(&e, pieces).unwrap();
                            assert!(
                                arc <= bound && bound <= slack * arc,
                                "({big}, {r}) cut at {d}, {span:?}, {pieces} pieces: \
                                 bound {bound} against the arc's {arc}"
                            );
                        }
                    }
                }
            }
        }
    }

    /// **The smaller reading is the bound**: a thin tube's generic cut,
    /// `(1, 10⁻³)` at `0.9`, where `f` barely moves and the apart reading
    /// is within `1.35×` the arc over a whole period at one or two pieces.
    /// The together reading pairs `(2 − x)`'s sup with `φ`'s there, `2.47×`
    /// the arc; the bound holds `1.4×`.
    #[test]
    fn a_thin_tubes_generic_cut_is_bounded_within_its_apart_reading() {
        let (big, r, d) = (1.0, 1e-3, 0.9);
        for span in [(0.0, 2.0 * PI), (-PI, PI)] {
            let (e, _) = framed(
                (big, r, d),
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0; 3],
                [0.0; 3],
                span,
                true,
            );
            let arc = walked((big, r, d), span, 4000);
            for pieces in [1, 2] {
                let bound = spiric_length_bound(&e, pieces).unwrap();
                assert!(
                    arc <= bound && bound <= 1.4 * arc,
                    "{span:?}, {pieces} pieces: bound {bound} against the arc's {arc}"
                );
            }
        }
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
                        parts: Vec::new(),
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
                parts: one_loop(1, 2, 1),
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
            let face = RimFaceQ {
                rationals: Vec::new(),
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
            let got = summed(wall_chart(&face, &chart, 512).unwrap());
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
                RimFaceQ {
                    rationals: Vec::new(),
                    closed_va: [Interval::zero(); 3],
                    spirics: vec![arc],
                    lever: [Interval::zero(); 3],
                    perimeter: 20.0,
                    wall: Some(WallChartQ {
                        parts: one_loop(1, 0, 1),
                        major: pt(1.0),
                        minor: pt(r),
                        rims: vec![],
                        meridians: vec![MeridianQ {
                            dv: oval.start.1 - oval.end.1,
                            env: Interval::zero(),
                        }],
                        ends: vec![vec![oval, back]],
                    }),
                }
            };
            let near = face(2.0 * eps);
            let at_f64 =
                rim_face_rounds::<f64>(&near, band, RoundWindow::SCHEDULE, &mut to_target(eps));
            assert!(at_f64.is_ok(), "r = {r}, f64: {at_f64:?}");
            let at_iv = rim_face_rounds::<Interval>(
                &near,
                band,
                RoundWindow::SCHEDULE,
                &mut to_target(eps),
            );
            assert!(at_iv.is_ok(), "r = {r}, interval: {at_iv:?}");
            let far = rim_face_rounds::<f64>(
                &face(20.0 * eps),
                band,
                RoundWindow::SCHEDULE,
                &mut to_target(eps),
            );
            assert!(
                matches!(far, Err(PropsError::ChartLoopOpen)),
                "r = {r}, 20ε: {far:?}"
            );
        }
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
        let face = RimFaceQ {
            rationals: Vec::new(),
            closed_va: [Interval::zero(); 3],
            spirics: vec![arc],
            lever: [Interval::zero(); 3],
            perimeter: 20.0,
            wall: Some(WallChartQ {
                parts: one_loop(1, 0, 1),
                major: pt(1.2),
                minor: pt(0.275),
                rims: vec![],
                meridians: vec![MeridianQ {
                    dv: oval.start.1 - oval.end.1,
                    env: Interval::zero(),
                }],
                ends: vec![vec![
                    oval,
                    ChartEndsQ {
                        start: (wide, oval.end.1),
                        end: (oval.start.0, oval.start.1),
                        env: Interval::zero(),
                    },
                ]],
            }),
        };
        for got in [
            rim_face_rounds::<f64>(&face, band, RoundWindow::SCHEDULE, &mut to_target(eps)),
            rim_face_rounds::<Interval>(&face, band, RoundWindow::SCHEDULE, &mut to_target(eps)),
        ] {
            assert!(
                matches!(got, Err(PropsError::Escalated { .. })),
                "an undecidable junction escalates: {got:?}"
            );
        }
    }

    /// **A wall's ring is decided closed on its own** ([`wall_windings`],
    /// loop by loop), **and wound against its face**
    /// ([`require_rings_wound_against`]). Beside a closed loop (a whole
    /// oval closed by a meridian back), a ring cut from it (the smaller
    /// loop of an oval cut nearer the torus's axis, closed the same way
    /// and run backwards) measures the loop's area less
    /// the ring's. The same ring wound with the loop refuses
    /// [`PropsError::RingWoundWithFace`] rather than add itself to the
    /// face's area, as does the loop's own copy beside it, and the loop's
    /// copy run backwards leaves a whole of no area, on which the read
    /// takes no side (it escalates `props_chart_area_side`); each at both
    /// scalars. A ring wholly in the band escalates
    /// `props_ring_winding` at once; one whose round-0 enclosure
    /// straddles the band's edge escalates at a window ending there, and
    /// the whole schedule refines on until a later round decides it. A ring
    /// whose lift stays open by a part of a period, as a ring crossing
    /// the chart's seam would where its images do not continue across
    /// it, refuses [`PropsError::ChartLoopOpen`]; one whose lift restarts
    /// a whole period across the seam reads as winding the tube, and
    /// refuses [`PropsError::LoopsWindChart`]. Neither is read as closed.
    /// Nor is a band of two loops winding the tube both ways, which
    /// refuses [`PropsError::LoopsWindChart`] typed.
    #[test]
    fn a_ring_on_a_wall_is_decided_closed_on_its_own() {
        let tol = geom_core::Tol::witness();
        let band = Band::linear(tol).unwrap();
        let tau = 2.0 * PI;
        // A whole oval at offset `d`, closed by a meridian back: its edges
        // and its lifted ends, forwards or run backwards.
        let closed = |d: f64, forward: bool| {
            let (oval, _) = framed(
                (1.2, 0.275, d),
                [0.0, 0.0, 1.0],
                [1.0, 0.0, 0.0],
                [0.0; 3],
                [0.0; 3],
                (0.0, tau),
                true,
            );
            let oval = SpiricEdgeQ {
                wall: Some(WallImageQ {
                    sense: 1.0,
                    v0: Interval::zero(),
                    env: Interval::zero(),
                }),
                ..oval
            };
            let ends = spiric_wall_ends(&oval, Interval::zero()).unwrap();
            let back = ChartEndsQ {
                start: ends.end,
                end: (ends.end.0, ends.start.1),
                env: Interval::zero(),
            };
            let meridian = MeridianQ {
                dv: ends.start.1 - ends.end.1,
                env: Interval::zero(),
            };
            if forward {
                (oval, meridian, vec![ends, back])
            } else {
                let rev = |e: ChartEndsQ| ChartEndsQ {
                    start: e.end,
                    end: e.start,
                    env: e.env,
                };
                (
                    SpiricEdgeQ {
                        forward: false,
                        ..oval
                    },
                    MeridianQ {
                        dv: -meridian.dv,
                        env: Interval::zero(),
                    },
                    vec![rev(back), rev(ends)],
                )
            }
        };
        let wall_face = |loops: &[(SpiricEdgeQ, MeridianQ, Vec<ChartEndsQ>)]| RimFaceQ {
            rationals: Vec::new(),
            closed_va: [Interval::zero(); 3],
            spirics: loops.iter().map(|l| l.0).collect(),
            lever: [Interval::zero(); 3],
            perimeter: 40.0,
            wall: Some(WallChartQ {
                parts: loops.iter().map(|_| part(1, 0, 1)).collect(),
                major: pt(1.2),
                minor: pt(0.275),
                rims: vec![],
                meridians: loops.iter().map(|l| l.1).collect(),
                ends: loops.iter().map(|l| l.2.clone()).collect(),
            }),
        };
        // At each scalar, to a reporting target of ε `1e-6` at least: the
        // ring's winding is read at the round returned, whatever the
        // target, and a tighter one only costs rounds.
        let eps = tol.eps().max(1e-6);
        let within = |scalar: usize, f: &RimFaceQ, window: RoundWindow| {
            if scalar == 0 {
                rim_face_rounds::<f64>(f, band, window, &mut to_target(eps))
            } else {
                rim_face_rounds::<Interval>(f, band, window, &mut to_target(eps))
            }
        };
        let at = |scalar: usize, f: &RimFaceQ| within(scalar, f, RoundWindow::SCHEDULE);
        let area = |got: &Result<LaneRun, PropsError>| match got {
            Ok(LaneRun {
                bounds,
                end: LaneEnd::Accepted(_),
                ..
            }) => bounds.area,
            other => panic!("a face that measures: {other:?}"),
        };
        let outer = closed(0.3, true);
        let inner = closed(0.05, true);
        let hole = closed(0.05, false);
        // A ring round 0 leaves straddling the band's escalate edge and
        // round 1 decides. Its round-0 enclosure is some 0.13% of its
        // mean width wide, so the offsets whose enclosure straddles the
        // edge are a window that narrow, about `170.55·ε` to `170.77·ε`
        // at `K = 10`: found from round 0's own readings rather than
        // named, as the two ends of that window, between an offset whose
        // ring is wholly in the band and one round 0 decides. A round's
        // enclosure is about 8× narrower than the one before, so an
        // offset a quarter of the window in from its decided end is
        // decided at round 1.
        let ring_at = |d: f64| closed(d, false);
        let reading = |d: f64| match within(
            0,
            &wall_face(&[outer.clone(), ring_at(d)]),
            RoundWindow::at(0),
        ) {
            Err(PropsError::Escalated { cause, .. })
                if cause.predicate == Some("props_ring_winding") =>
            {
                usize::from(!cause.terminal_sliver)
            }
            _ => 2,
        };
        let edge = |past: usize| {
            let (mut lo, mut hi) = (10.0 * tol.k() * tol.eps(), 30.0 * tol.k() * tol.eps());
            assert!(
                reading(lo) == 0 && reading(hi) == 2,
                "the window lies between"
            );
            for _ in 0..24 {
                let mid = 0.5 * (lo + hi);
                if reading(mid) >= past {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            hi
        };
        let (straddles, decided) = (edge(1), edge(2));
        assert!(
            straddles < decided,
            "round 0 straddles the edge on a window: {straddles} {decided}"
        );
        let straddling = ring_at(decided - 0.25 * (decided - straddles));
        for scalar in 0..2 {
            let alone = |l: &(SpiricEdgeQ, MeridianQ, Vec<ChartEndsQ>)| {
                area(&at(scalar, &wall_face(core::slice::from_ref(l))))
            };
            let (whole, ring) = (alone(&outer), alone(&inner));
            assert!(
                ring.hi() < whole.lo(),
                "the ring is the smaller loop: {ring:?} {whole:?}"
            );
            let cut = area(&at(scalar, &wall_face(&[outer.clone(), hole.clone()])));
            let want = (whole - ring).abs_enclosure();
            assert!(
                cut.lo() <= want.hi() && want.lo() <= cut.hi(),
                "scalar {scalar}: the loop less its ring, {cut:?} against {want:?}"
            );
            // Each read at the first round, the round returned: a face of
            // no area never converges, and would spend the whole budget
            // before its rings are read.
            let first = |loops: &[(SpiricEdgeQ, MeridianQ, Vec<ChartEndsQ>)]| {
                within(scalar, &wall_face(loops), RoundWindow::at(0))
            };
            for (what, loops) in [
                (
                    "the ring wound with the loop",
                    vec![outer.clone(), inner.clone()],
                ),
                (
                    "the loop's own copy beside it",
                    vec![outer.clone(), outer.clone()],
                ),
            ] {
                let got = first(&loops);
                assert!(
                    matches!(got, Err(PropsError::RingWoundWithFace)),
                    "scalar {scalar}, {what}: {got:?}"
                );
            }
            // The loop's copy run backwards leaves a whole of no area,
            // which the winding read does not take a side on: its
            // enclosure holds zero, and the read escalates.
            let got = first(&[outer.clone(), closed(0.3, false)]);
            assert!(
                matches!(&got, Err(PropsError::Escalated { cause, .. })
                    if cause.predicate == Some("props_chart_area_side")),
                "scalar {scalar}, the loop's copy run backwards: {got:?}"
            );
            // A ring whose mean width is wholly in the band escalates at
            // once, at any window: no round decides it, and every
            // window's escalation carries round 0's own enclosure.
            let in_band = wall_face(&[outer.clone(), ring_at(10.0 * tol.k() * tol.eps())]);
            let read = |window| match within(scalar, &in_band, window) {
                Err(PropsError::Escalated { cause, .. })
                    if cause.predicate == Some("props_ring_winding") && cause.terminal_sliver =>
                {
                    cause.margin
                }
                got => panic!("scalar {scalar}, {window:?}, a ring in the band: {got:?}"),
            };
            assert_eq!(
                read(RoundWindow::SCHEDULE),
                read(RoundWindow::at(0)),
                "scalar {scalar}: the escalation is round 0's"
            );
            // The straddling ring escalates at a window ending at round
            // 0, and the whole schedule refines on past the face's
            // convergence until round 1 decides it a hole.
            let face = wall_face(&[outer.clone(), straddling.clone()]);
            let got = within(scalar, &face, RoundWindow::at(0));
            assert!(
                matches!(&got, Err(PropsError::Escalated { cause, .. })
                    if cause.predicate == Some("props_ring_winding")
                        && !cause.terminal_sliver),
                "scalar {scalar}, a ring round 0 leaves straddling: {got:?}"
            );
            for window in [RoundWindow::at(1), RoundWindow::SCHEDULE] {
                let got = within(scalar, &face, window);
                assert!(
                    matches!(
                        got,
                        Ok(LaneRun {
                            end: LaneEnd::Accepted(_),
                            ..
                        })
                    ),
                    "scalar {scalar}, {window:?}, the straddling ring decided: {got:?}"
                );
            }
        }
        let (oval, meridian, loop_ends) = inner;
        let (ends, back) = (loop_ends[0], loop_ends[1]);
        let face = |ring: Vec<ChartEndsQ>| RimFaceQ {
            rationals: Vec::new(),
            closed_va: [Interval::zero(); 3],
            spirics: vec![oval, oval],
            lever: [Interval::zero(); 3],
            perimeter: 40.0,
            wall: Some(WallChartQ {
                parts: vec![part(1, 0, 1), part(1, 0, 1)],
                major: pt(1.2),
                minor: pt(0.275),
                rims: vec![],
                meridians: vec![meridian, meridian],
                ends: vec![vec![ends, back], ring],
            }),
        };
        let run = |f: &RimFaceQ| {
            rim_face_rounds::<f64>(f, band, RoundWindow::SCHEDULE, &mut to_target(tol.eps()))
        };
        // The ring's meridian back lands `0.3` short of its start in `u`.
        let short = ChartEndsQ {
            end: (back.end.0 + pt(0.3), back.end.1),
            ..back
        };
        let open = run(&face(vec![ends, short]));
        assert!(
            matches!(open, Err(PropsError::ChartLoopOpen)),
            "a ring open on the chart: {open:?}"
        );
        // The ring's meridian back restarts one period round in `u`.
        let period = ChartEndsQ {
            end: (back.end.0 + pt(tau), back.end.1),
            ..back
        };
        let winds = run(&face(vec![ends, period]));
        assert!(
            matches!(winds, Err(PropsError::LoopsWindChart)),
            "a ring a period open across the seam: {winds:?}"
        );
        // A band: the oval alone winds the tube once, and the same oval
        // run back winds it once the other way. The two bound a band of
        // the wall, which `require_boundary_windings` admits as a
        // boundary; the lifted chart's Green form does not read it, so
        // the wall refuses it typed rather than misread it
        // (`work/spiricarm/a-spiric-bounded-torus-band-has-no-volume-lane.md`).
        let back_oval = SpiricEdgeQ {
            forward: false,
            ..oval
        };
        let band_face = RimFaceQ {
            rationals: Vec::new(),
            closed_va: [Interval::zero(); 3],
            spirics: vec![oval, back_oval],
            lever: [Interval::zero(); 3],
            perimeter: 40.0,
            wall: Some(WallChartQ {
                parts: vec![part(1, 0, 0), part(1, 0, 0)],
                major: pt(1.2),
                minor: pt(0.275),
                rims: vec![],
                meridians: vec![],
                ends: vec![
                    vec![ends],
                    vec![spiric_wall_ends(&back_oval, Interval::zero()).unwrap()],
                ],
            }),
        };
        let band = run(&band_face);
        assert!(
            matches!(band, Err(PropsError::LoopsWindChart)),
            "a band of two loops winding the tube both ways: {band:?}"
        );
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
            RimFaceQ {
                rationals: Vec::new(),
                closed_va: [Interval::zero(); 3],
                spirics: vec![arc],
                lever: [Interval::zero(); 3],
                perimeter: 20.0,
                wall: Some(WallChartQ {
                    parts: one_loop(1, 0, usize::from(closed_by_back)),
                    major: pt(1.2),
                    minor: pt(0.275),
                    rims: vec![],
                    meridians,
                    ends: vec![ends],
                }),
            }
        };
        let run = |f: &RimFaceQ| {
            rim_face_rounds::<f64>(f, band, RoundWindow::SCHEDULE, &mut to_target(tol.eps()))
        };
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
            let u_start = chart.ends[0][0].start.0;
            chart.ends[0][1].start.0 = u_start;
            chart.ends[0][1].end.0 = u_start;
        }
        let open_u = run(&f);
        assert!(
            matches!(open_u, Err(PropsError::ChartLoopOpen)),
            "open in u: {open_u:?}"
        );
        // The oval twice round winds the tube twice.
        let mut twice = face(wall(0.05, 0.0, tau), false);
        if let Some(chart) = twice.wall.as_mut() {
            let first = chart.ends[0][0];
            chart.ends[0].push(first);
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
            chart.ends[0][1].start.1 = chart.ends[0][1].start.1 + pt(2.0 * tol.eps() / 0.275);
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
            parts: Vec::new(),
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
            let face = RimFaceQ {
                rationals: Vec::new(),
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
                    parts: one_loop(1, 0, 0),
                    ends: Vec::new(),
                    major: pt(big),
                    minor: pt(r),
                    rims: vec![],
                    meridians: vec![],
                }),
            };
            let chart = face.wall.clone().unwrap();
            let got = summed(wall_chart(&face, &chart, 64).unwrap());
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

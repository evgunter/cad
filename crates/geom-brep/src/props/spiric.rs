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
//! # The wall: the spiric arm of the torus chart's Green form
//!
//! A torus wall is the rotational lane's
//! ([`super::quad::rotational_cut_face_rounds`], the torus), which reads
//! `−∮ F(v) du` and `−∮ G(v) du` over its lifted boundary,
//! `F′ = r·ρ(v)·(R cos v + r)` and `G′ = r·ρ(v)`; this module holds its
//! spiric arm's integrand ([`wall_terms`]). A spiric's lifted image is exact
//! (`SpiricImage::Wall` on the carrier's own torus:
//! `u = u₀ + σ·atan2(f, d)`, `v = v₀ + σ·t`, `v₀ = k·τ`), so `sin v = σ sin t`, `cos v = cos t` and
//! `u′ = −σ·d·r·sin t/(ρ·f)`, and its terms are
//!
//! ```text
//! −∫ F(v₀ + σt)·u′ dt,   −∫ G(v₀ + σt)·u′ dt,   ∫ u′ dt  (its Δu)
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
//!
//! A wall's other pads are the rotational lane's.

use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, certified_hi, cross, dot};
use geom_core::{Band, Bounds, Decide};

use super::PropsError;
use super::bracket::{AbsEnclosure, pt};
use super::quad::{
    Accept, EDGE_INIT_PIECES, FaceCutBounds, LaneRun, RVec3, RoundWindow, Run, face_extent,
};
use super::taylor::{Jet, W_SPIRIC, composite, node, trig_of, trig_range, w_rational};

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
}

/// A spiric's lifted image on its OWN torus wall
/// (`SpiricImage::Wall` tagged `WallTorus::Own`):
/// `u = u₀ + σ·atan2(f, d)`, `v = v₀ + σ·t`. A wall on the mirror torus
/// (`WallTorus::Mirror`, azimuth sign `−σ`) has no terms here yet, and
/// `topo`'s quad lane refuses it typed before building one
/// (`the-spiric-wall-quadrature-lane-reads-only-the-own-torus`).
#[derive(Clone, Copy, Debug)]
pub struct WallImageQ {
    /// `σ = ±1`.
    pub sense: f64,
    /// The lifted minor-angle constant `v₀`.
    pub v0: Interval,
    /// The image's certificate envelope (metres).
    pub env: Interval,
}

/// One spiric-bounded cap, bracketed by the caller.
///
/// The cap's vector area about a reference point is `closed_va` (the
/// closed-form edges, already summed) plus each spiric's term; its flux
/// is `lever·A⃗`, `lever` the reference less the centre. Its area is
/// `‖A⃗‖` on a plane.
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
pub(super) fn vector_area_split(e: &SpiricEdgeQ, pieces: usize) -> (RVec3, f64) {
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

/// **One spiric wall edge's Green-form terms** `[−∫F du, −∫G du, Δu]`
/// on its lift (module docs), in its carrier's order: the rotational
/// lane's spiric arm reads them on its torus, `(R, r)`, and folds in the
/// traversal. And the part of the flux term's width a later round
/// removes.
pub(super) fn wall_terms(
    e: &SpiricEdgeQ,
    w: &WallImageQ,
    (big, r): (Interval, Interval),
    pieces: usize,
) -> ([Interval; 3], f64) {
    let sigma = pt(w.sense);
    let rr = big.sqr() + r.sqr();
    let three_half = pt(1.5) * big * r;
    let half_br = pt(0.5) * big * r;
    let v0 = w.v0;
    let ([flux, area, du], [removable, _, _]) = composite(e.t0, e.t1, pieces, |t, c, s| {
        let wgt = wall_weight(e, w, (big, r), c, s);
        let v = Jet::cst(v0).add(t.scale(sigma));
        let ff = s
            .scale(sigma * rr)
            .add(v.scale(three_half))
            .add(s.mul(c).scale(sigma * half_br))
            .scale(r);
        let gg = v.scale(big).add(s.scale(sigma * r)).scale(r);
        [ff.mul(wgt), gg.mul(wgt), wgt.scale(-Interval::one())]
    });
    ([flux, area, du], removable)
}

/// `−u′ = σ·d·r·sin t/(ρ·f)` of a spiric's wall image at the trig jets
/// of `t`: the weight its wall terms read `F` and `G` at.
fn wall_weight(
    e: &SpiricEdgeQ,
    w: &WallImageQ,
    (big, r): (Interval, Interval),
    c: Jet,
    s: Jet,
) -> Jet {
    let (rho, f) = rho_f(big, r, e.offset, c);
    s.mul(rho.mul(f).recip()).scale(pt(w.sense) * e.offset * r)
}

/// **One spiric wall edge's turn `Δu`** on its lift, in its carrier's
/// order, at `pieces`: its wall terms' third alone ([`wall_terms`]).
pub(super) fn wall_turn(
    e: &SpiricEdgeQ,
    w: &WallImageQ,
    radii: (Interval, Interval),
    pieces: usize,
) -> Interval {
    composite(e.t0, e.t1, pieces, |_, c, s| {
        [wall_weight(e, w, radii, c, s).scale(-Interval::one())]
    })
    .0[0]
}

/// How a cap's area reads its vector area `A⃗`: `‖A⃗‖` on a plane, and
/// `|â·A⃗|/sin α` on a cone, whose every normal makes the angle `π/2 − α`
/// with its axis `â`.
#[derive(Clone, Copy, Debug)]
enum CapArea {
    Plane,
    Cone { axis: RVec3, sin_a: Interval },
}

/// The cap's enclosures at `pieces` resolution, and the part of the
/// flux's width a later round removes: each spiric's moment `J`, which
/// the flux reads through the lever as `Σₖ leverₖ·nₖ·r·J` — component by
/// component, as the interval sum carries `J`'s width, so a lever in the
/// cap's own plane still carries it — and each rational rim's.
fn face_bounds(
    face: &RimFaceQ,
    cap: CapArea,
    pieces: usize,
) -> Result<(FaceCutBounds, f64), PropsError> {
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
    Ok((
        FaceCutBounds {
            flux: dot(&face.lever, &va),
            area: match cap {
                CapArea::Plane => (va[0].sqr() + va[1].sqr() + va[2].sqr()).sqrt(),
                CapArea::Cone { axis, sin_a } => dot(&axis, &va).abs_enclosure() / sin_a,
            },
        },
        removable,
    ))
}

/// The work of round `round` on a face of `spirics` spiric edges and
/// rational rims weighing `rational` in all ([`w_rational`] per span).
pub(super) fn round_cost(spirics: usize, rational: usize, round: usize) -> usize {
    (W_SPIRIC * spirics + rational) * (EDGE_INIT_PIECES << round)
}

/// **The certified flux and area of a spiric- or rational-rim-bounded
/// plane cap** over a [`RoundWindow`] — [`super::quad`]'s schedule and
/// stop (pieces doubling from `EDGE_INIT_PIECES`, the caller's
/// [`Accept`], the lane's floor and work wall, the face-extent gate on
/// the accepted round). A torus wall a spiric bounds is the rotational
/// lane's ([`super::quad::rotational_cut_face_rounds`]).
///
/// # Errors
///
/// [`PropsError`] — `accept`'s own refusal, an escalated funnel
/// decision, or a degenerate face.
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
    rounds::<T>(face, CapArea::Cone { axis, sin_a }, band, window, accept)
}

fn rounds<T: Decide>(
    face: &RimFaceQ,
    cap: CapArea,
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
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
        let (bounds, removable) = face_bounds(face, cap, pieces)?;
        let (stop, accepted) = run.ask(round, bounds, removable, accept, || {
            face_extent::<T>(bounds.area, face.perimeter, band)
        })?;
        if let Some(out) = run.ringed(round, stop, accepted, bounds, || Ok(()))? {
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
    use crate::props::quad::LaneStop;

    /// One spiric wall edge's terms `[−∫F du, −∫G du, Δu, Δv]` on its
    /// lift, traversal folded in: [`wall_terms`] as the rotational lane's
    /// spiric arm reads it.
    fn spiric_wall_terms(
        e: &SpiricEdgeQ,
        w: &WallImageQ,
        radii: (Interval, Interval),
        pieces: usize,
    ) -> [Interval; 4] {
        let ([f, g, du], _) = wall_terms(e, w, radii, pieces);
        let terms = [f, g, du, pt(w.sense) * (e.t1 - e.t0)];
        if e.forward { terms } else { terms.map(|x| -x) }
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

    fn edge(
        c: (f64, f64, f64, f64, f64, f64, f64),
    ) -> (SpiricEdgeQ, WallImageQ, (Interval, Interval)) {
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
        };
        (e, wall, (pt(big), pt(r)))
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
                let [flux, area, du, _] = spiric_wall_terms(&e, &w, chart, pieces);
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
        let f = spiric_wall_terms(&e, &w, chart, 64);
        let r = spiric_wall_terms(
            &SpiricEdgeQ {
                forward: false,
                ..e
            },
            &w,
            chart,
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
        assert!(
            spiric_length_bound(&e, LENGTH_PIECES).is_some(),
            "the wall arm's metric length has a rate bound"
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
                    let got = spiric_wall_terms(&e, &w, (pt(big), pt(r)), 256);
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
        let chart = (pt(big), pt(r));
        let j = cos_moment(&e, 64);
        let va = spiric_vector_area(&e, 64);
        let wb = spiric_wall_terms(&e, &w, chart, 64);
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
                spiric_wall_terms(&thin, &w, chart, 256),
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

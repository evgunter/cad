//! The span derivative's coefficient factor, spelled once: the
//! derivative of a degree-`p` B-spline has coefficients
//! `p·(c_{i+1} − c_i)/(u_{i+p+1} − u_{i+1})` (The NURBS Book Eq. 3.4).
//!
//! What a zero knot difference means is the caller's, so
//! [`span_derivative_scale`] takes the two knot values and checks
//! nothing.
//!
//! # The rational speed bound
//!
//! [`rational_speed_sup`] is the one statement of an upper bound
//! on a rational spline's derivative, the **pair-term bound**. On a
//! nonempty span of degree `p`, with `kᵢ = p/(u_{i+p+1} − u_{i+1})`,
//!
//! ```text
//! C′ = Σᵢ N_{i,p−1} · kᵢ · [ w_{i+1}(P_{i+1} − Pᵢ) − (w_{i+1} − wᵢ)(C − Pᵢ) ] / w
//! ```
//!
//! over the span's active pairs `i`. Each bracket is affine in `C`,
//! and `C` lies in the convex hull of the span's active points `P_j`
//! (positive weights), so its norm is largest at one of them; the
//! `N_{i,p−1}` sum to one and `w` is at least the least active weight.
//! So
//!
//! ```text
//! sup ‖C′‖ ≤ max_{i, j} kᵢ·‖w_{i+1}(P_{i+1} − Pᵢ) − (w_{i+1} − wᵢ)(P_j − Pᵢ)‖ / min_j w_j
//! ```
//!
//! with no centre to choose. At unit weights it is exactly the
//! polynomial control-difference bound `max kᵢ·‖P_{i+1} − Pᵢ‖`. A
//! domain's bound is the max over its nonempty spans, a window's over
//! the spans it meets.
//!
//! A tensor patch's `S_u` is, on a nonempty cell, the same sum over the
//! `u` pairs of each of the cell's active columns, with `S` in the hull
//! of the cell's active points: [`rational_stretch_sup`] is the
//! same bound per direction, per cell. Both read one
//! [`RationalPairTerm`].
//!
//! # The rational floors
//!
//! The same sums bound the derivative from **below** along any
//! direction `d`. On a nonempty cell,
//!
//! ```text
//! W·S_u = A_u − W_u·S = Σ_{i, b} λ_{ib} · T_{ib}(S)
//! ```
//!
//! with `A = Σ w P` the homogeneous numerator, `W` its weight, `T_{ib}`
//! the `u` pair term of pair `i` in active column `b` (affine in `S`),
//! and `λ_{ib} = N_{i,p−1}(u)·M_b(v) ≥ 0` summing to one. `S` itself is
//! `Σⱼ νⱼ Pⱼ` over the cell's active points with `νⱼ = Nⱼ Mⱼ wⱼ / W ≥ 0`
//! summing to one, and an affine map carries that combination through,
//! so `W·S_u = Σ λ_{ib} νⱼ T_{ib}(Pⱼ)` is a convex combination of the
//! values `T_{ib}(Pⱼ)`. Its projection on `d` is at least the least
//! projected value `m`, and `W` lies between the cell's least and
//! greatest active weight, so
//!
//! ```text
//! S_u·d ≥ m / w_max   (m ≥ 0)      S_u·d ≥ m / w_min   (m < 0)
//! ```
//!
//! — [`rational_stretch_floor`], the min over the nonempty cells. Each
//! clause carries the proof: `j` runs over **every** active point
//! because `S` can sit anywhere in their hull, and the divisor follows
//! the sign of `m`, the lattice min of both so an interval `m` that
//! straddles zero takes both.
//!
//! **The area element.** With `W·S_v = Σ μ_{ka} U_{ka}(S)` the same way,
//! `W²·(S_u × S_v) = Σ λ_{ib} μ_{ka} · T_{ib}(S) × U_{ka}(S)`. Each
//! cross is **affine** in `S`: writing `T(S) = c − g·S` and
//! `U(S) = e − h·S` (scalars `g`, `h`), the quadratic part is
//! `g·h·(S × S) = 0`. So it too carries `S = Σ νⱼ Pⱼ` through,
//!
//! ```text
//! W²·(S_u × S_v) = Σ λ_{ib} μ_{ka} νⱼ · T_{ib}(Pⱼ) × U_{ka}(Pⱼ)
//! ```
//!
//! a convex combination of the crosses with **both factors at the same
//! active point**. So `(S_u × S_v)·d` is at least their least projection
//! over `w_max²` or `w_min²` by its sign — [`rational_area_floor`]. The
//! element is `‖W·A_u×A_v − W_v·A_u×A − W_u·A×A_v‖ / W³`, a homogeneous
//! polynomial over `W³`; the pair terms read its floor without forming
//! that product spline.
//!
//! At unit weights every `T(Pⱼ)` is the polynomial derivative-net entry,
//! so both floors are the polynomial net's, cell by cell.
//!
//! **No zero knot difference is reachable.** A span is walked only
//! when it is nonempty, and its active pairs are `i ∈ [s − p, s − 1]`
//! ([`Span::first_derived_window`](super::Span::first_derived_window),
//! total): each pair's knot difference `u_{i+p+1} − u_{i+1}` spans
//! `[u_s, u_{s+1}]`, so it is positive. Nothing here checks it.

use core::ops::{Mul, Sub};

use crate::real::Real;

use super::knot_values::Knots;
use super::knots::Span;
use super::locate::SpanLocate;

/// `p / (hi − lo)` in `T`, for the pair whose knot difference runs from
/// `lo = u_{i+1}` to `hi = u_{i+p+1}`.
#[must_use]
pub fn span_derivative_scale<T: Real>(p: usize, lo: T, hi: T) -> T {
    #[allow(clippy::cast_precision_loss)]
    let p = T::from_f64(p as f64);
    p / (hi - lo)
}

/// The derivative net of the degree-`p` coefficients `net`, whose first
/// entry is coefficient `first`: entry `k` is
/// `(net[k+1] − net[k])·`[`span_derivative_scale`] with `i = first + k`.
/// The knot difference is checked positive on the structure `knots` and
/// formed in `T` over the `held` knots. `None` on a non-positive
/// difference or a knot index past the vector.
pub fn derivative_net<T: Real, P: Copy + Sub<Output = V>, V: Mul<T, Output = V>>(
    net: &[P],
    knots: &[f64],
    held: &[T],
    p: usize,
    first: usize,
) -> Option<Vec<V>> {
    net.windows(2)
        .enumerate()
        .map(|(k, pair)| {
            let i = first + k;
            let du = *knots.get(i + p + 1)? - *knots.get(i + 1)?;
            #[allow(clippy::neg_cmp_op_on_partial_ord)]
            if !(du > 0.0) {
                return None;
            }
            Some((pair[1] - pair[0]) * pair_scale(held, p, i)?)
        })
        .collect()
}

/// The Euclidean norm of a `D`-vector, `√Σ xₖ²`.
fn norm<T: Real, const D: usize>(v: [T; D]) -> T {
    v.iter()
        .map(|x| x.powi(2))
        .fold(T::zero(), |a, b| a + b)
        .sqrt()
}

/// **One pair term** of the rational derivative (module docs): the pair
/// `(P₀, w₀) → (P₁, w₁)` with scale `k`, as the affine map
/// `S ↦ k·[lever − dw·(S − p0)]` with `lever = w₁(P₁ − P₀)` and
/// `dw = w₁ − w₀`. The one statement of the term: the sups below
/// evaluate it at points, and a reader that carries it through a rigid
/// map (`geom-brep`'s SSI enclosure) reads it about a centre
/// ([`Self::about`]).
#[derive(Clone, Copy, Debug)]
pub struct RationalPairTerm<T, const D: usize> {
    scale: T,
    lever: [T; D],
    dw: T,
    p0: [T; D],
}

impl<T: Real, const D: usize> RationalPairTerm<T, D> {
    /// The pair `(p0, w0) → (p1, w1)` at scale `scale`.
    #[must_use]
    pub fn new(scale: T, (p0, w0): ([T; D], T), (p1, w1): ([T; D], T)) -> Self {
        Self {
            scale,
            lever: core::array::from_fn(|c| w1 * (p1[c] - p0[c])),
            dw: w1 - w0,
            p0,
        }
    }

    /// The term as an affine function of the offset `q = S − c`:
    /// `(k, g)` with term `= k − g·q`, `k = scale·[lever + dw·(p0 − c)]`
    /// and `g = scale·dw`. Every point enters as a difference with
    /// another, so a translation moves `k` by its rounding width alone.
    #[must_use]
    pub fn about(&self, c: [T; D]) -> ([T; D], T) {
        (
            core::array::from_fn(|i| (self.lever[i] + self.dw * (self.p0[i] - c[i])) * self.scale),
            self.dw * self.scale,
        )
    }

    /// The term at the point `s`: [`Self::about`] `s` itself,
    /// `scale·[lever − dw·(s − p0)]`.
    #[must_use]
    pub fn at(&self, s: [T; D]) -> [T; D] {
        self.about(s).0
    }

    /// The weight step `w₁ − w₀`: exactly zero where the pair's weights
    /// agree, so the term does not read `S`.
    #[must_use]
    pub fn weight_step(&self) -> T {
        self.dw
    }
}

/// The derivative scale of pair `i` of a degree-`p` spline over the
/// `held` knot values: [`span_derivative_scale`] over
/// `u_{i+1} … u_{i+p+1}`, `None` past the vector.
fn pair_scale<T: Real>(held: &[T], p: usize, i: usize) -> Option<T> {
    Some(span_derivative_scale(
        p,
        *held.get(i + 1)?,
        *held.get(i + p + 1)?,
    ))
}

/// `max_{term, point} ‖term(point)‖ / least`, or poison when there is
/// no term.
fn pair_sup<T: Real, const D: usize>(
    terms: impl Iterator<Item = RationalPairTerm<T, D>>,
    points: &[[T; D]],
    least: T,
) -> T {
    let mut sup: Option<T> = None;
    for t in terms {
        for pj in points {
            let n = norm(t.at(*pj));
            sup = Some(sup.map_or(n, |s| s.max(n)));
        }
    }
    sup.map_or_else(|| T::from_f64(f64::NAN), |s| s / least)
}

/// The pair-term bound on `sup ‖C′‖` over one nonempty `span` (module
/// docs: the derivation and why no knot difference is checked), for the
/// rational spline with control `points` and `weights` over the `held`
/// knot values of the vector `span` indexes; poison (NaN) on an index
/// past an array, never a refusal. Private: `span` and `held` must
/// index one vector, which [`rational_speed_sup`] pairs from one
/// [`Knots`].
fn rational_span_speed_sup<T: Real, const D: usize>(
    span: Span<'_>,
    held: &[T],
    points: &[[T; D]],
    weights: &[T],
) -> T {
    let poison = T::from_f64(f64::NAN);
    let p = span.degree();
    let (Some(active), Some(ws)) = (points.get(span.window()), weights.get(span.window())) else {
        return poison;
    };
    let Some(least) = ws.iter().copied().reduce(T::min) else {
        return poison;
    };
    let terms: Option<Vec<RationalPairTerm<T, D>>> = span
        .first_derived_window()
        .map(|i| {
            Some(RationalPairTerm::new(
                pair_scale(held, p, i)?,
                (*points.get(i)?, *weights.get(i)?),
                (*points.get(i + 1)?, *weights.get(i + 1)?),
            ))
        })
        .collect();
    terms.map_or(poison, |t| pair_sup(t.into_iter(), active, least))
}

/// One nonempty cell `(su, sv)` of a rational tensor patch, read along
/// one direction: the pair terms of that direction over each of the
/// cell's active columns, the cell's active points, and the least and
/// greatest active weight. Along `u` the pairs run along `u` in each
/// active column; along `v` the roles swap. The one per-pair builder of
/// the tensor statements: the sup and both floors read it.
struct Cell<T, const D: usize> {
    terms: Vec<RationalPairTerm<T, D>>,
    active: Vec<[T; D]>,
    least: T,
    most: T,
}

/// The [`Cell`] `(su, sv)` of the patch with row-major `points` and
/// `weights` (`[iu·nv + iv]`) over the `held_u` / `held_v` knot values
/// of the vectors the spans index; `None` on an index past an array.
fn cell<T: Real, const D: usize>(
    (su, sv): (Span<'_>, Span<'_>),
    (held_u, held_v): (&[T], &[T]),
    points: &[[T; D]],
    weights: &[T],
    nv: usize,
    along_u: bool,
) -> Option<Cell<T, D>> {
    let at = |iu: usize, iv: usize| -> Option<([T; D], T)> {
        let k = iu.checked_mul(nv)?.checked_add(iv)?;
        Some((*points.get(k)?, *weights.get(k)?))
    };
    let mut active = Vec::with_capacity((su.degree() + 1) * (sv.degree() + 1));
    let mut range: Option<(T, T)> = None;
    for iu in su.window() {
        for iv in sv.window() {
            let (pt, w) = at(iu, iv)?;
            active.push(pt);
            range = Some(range.map_or((w, w), |(lo, hi)| (lo.min(w), hi.max(w))));
        }
    }
    let (least, most) = range?;
    let (run, other, held, p) = if along_u {
        (su, sv, held_u, su.degree())
    } else {
        (sv, su, held_v, sv.degree())
    };
    let terms = run
        .first_derived_window()
        .flat_map(|i| other.window().map(move |b| (i, b)))
        .map(|(i, b)| {
            let (a0, a1) = if along_u {
                (at(i, b)?, at(i + 1, b)?)
            } else {
                (at(b, i)?, at(b, i + 1)?)
            };
            Some(RationalPairTerm::new(pair_scale(held, p, i)?, a0, a1))
        })
        .collect::<Option<Vec<_>>>()?;
    Some(Cell {
        terms,
        active,
        least,
        most,
    })
}

/// `per_cell` over the nonempty cells of the patch over `knots_u` ×
/// `knots_v`, handed each cell's spans, the held values and the `v`
/// control count, folded by `fold`; poison when no cell is nonempty.
fn over_cells<T: Real>(
    (knots_u, knots_v): (&Knots<T>, &Knots<T>),
    mut per_cell: impl FnMut((Span<'_>, Span<'_>), (&[T], &[T]), usize) -> T,
    fold: fn(T, T) -> T,
) -> T {
    let (ku, kv) = (knots_u.structure(), knots_v.structure());
    let nv = kv.control_count();
    let mut acc: Option<T> = None;
    for su in (ku.first_span()..=ku.last_span()).filter_map(|i| ku.span(i)) {
        for sv in (kv.first_span()..=kv.last_span()).filter_map(|i| kv.span(i)) {
            let b = per_cell((su, sv), (knots_u.values(), knots_v.values()), nv);
            acc = Some(acc.map_or(b, |m| fold(m, b)));
        }
    }
    acc.unwrap_or_else(poison)
}

fn poison<T: Real>() -> T {
    T::from_f64(f64::NAN)
}

/// The pair-term bound on `sup ‖C′‖` (module docs) over the nonempty
/// spans `spans` of `knots` (indices into its vector, empty ones
/// skipped): their max, or poison when none is nonempty. The weights
/// are positive by construction; a refinement that rounds one to zero
/// divides by a zero least weight, which reads `+∞` or NaN, never a
/// low finite number.
#[must_use]
pub fn rational_speed_sup<T: SpanLocate, const D: usize>(
    knots: &Knots<T>,
    points: &[[T; D]],
    weights: &[T],
    spans: core::ops::RangeInclusive<usize>,
) -> T {
    let kv = knots.structure();
    spans
        .filter_map(|index| kv.span(index))
        .map(|span| rational_span_speed_sup(span, knots.values(), points, weights))
        .reduce(T::max)
        .unwrap_or_else(|| T::from_f64(f64::NAN))
}

/// The pair-term bound on `sup ‖S_u‖` (`along_u`) or `sup ‖S_v‖` of
/// the rational tensor patch over `knots_u` × `knots_v` (module docs),
/// with row-major `points` and `weights` (`[iu·nv + iv]`): the max over
/// the nonempty cells, or poison when none is nonempty. A zero weight
/// reads as it does in [`rational_speed_sup`].
#[must_use]
pub fn rational_stretch_sup<T: Real, const D: usize>(
    knots: (&Knots<T>, &Knots<T>),
    points: &[[T; D]],
    weights: &[T],
    along_u: bool,
) -> T {
    over_cells(
        knots,
        |spans, held, nv| {
            cell(spans, held, points, weights, nv, along_u).map_or_else(poison, |c| {
                pair_sup(c.terms.into_iter(), &c.active, c.least)
            })
        },
        T::max,
    )
}

fn dot<T: Real, const D: usize>(a: [T; D], b: [T; D]) -> T {
    a.iter().zip(b).fold(T::zero(), |s, (x, y)| s + *x * y)
}

fn cross<T: Real>(a: [T; 3], b: [T; 3]) -> [T; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// The least of `values`, or poison when there is none.
fn least_of<T: Real>(values: impl Iterator<Item = T>) -> T {
    values.reduce(T::min).unwrap_or_else(poison)
}

/// `m/w` at its least over `w ∈ [least, most]` (module docs): `m/most`
/// for `m ≥ 0` and `m/least` for `m < 0`, spelled as their lattice min
/// so an interval `m` that straddles zero takes both.
fn over_weight<T: Real>(m: T, least: T, most: T) -> T {
    (m / most).min(m / least)
}

/// A lower bound on `S_u·d` (`along_u`) or `S_v·d` over the rational
/// tensor patch over `knots_u` × `knots_v` (module docs: the floor), for
/// any `d`, signed: the min over the nonempty cells of the least
/// projected term value over the cell's weight, or poison when no cell
/// is nonempty or an index runs past an array. `points` and `weights`
/// as [`rational_stretch_sup`]'s. For a unit `d` it floors `‖S_u‖` once
/// clamped at zero.
#[must_use]
pub fn rational_stretch_floor<T: Real, const D: usize>(
    knots: (&Knots<T>, &Knots<T>),
    points: &[[T; D]],
    weights: &[T],
    along_u: bool,
    d: [T; D],
) -> T {
    over_cells(
        knots,
        |spans, held, nv| {
            cell(spans, held, points, weights, nv, along_u).map_or_else(poison, |c| {
                let m = least_of(
                    c.terms
                        .iter()
                        .flat_map(|t| c.active.iter().map(move |pj| dot(t.at(*pj), d))),
                );
                over_weight(m, c.least, c.most)
            })
        },
        T::min,
    )
}

/// A lower bound on `(S_u × S_v)·d` over the rational tensor patch over
/// `knots_u` × `knots_v` (module docs: the area element), for any `d`,
/// signed: the min over the nonempty cells of the least projected cross
/// of a `u` and a `v` term, both at one active point, over the cell's
/// weight squared; poison as [`rational_stretch_floor`]. For a unit `d`
/// it floors `‖S_u × S_v‖` once clamped at zero.
#[must_use]
pub fn rational_area_floor<T: Real>(
    knots: (&Knots<T>, &Knots<T>),
    points: &[[T; 3]],
    weights: &[T],
    d: [T; 3],
) -> T {
    over_cells(
        knots,
        |spans, held, nv| {
            let (Some(cu), Some(cv)) = (
                cell(spans, held, points, weights, nv, true),
                cell(spans, held, points, weights, nv, false),
            ) else {
                return poison();
            };
            let mut m: Option<T> = None;
            for pj in &cu.active {
                // `(a × b)·d = a·(b × d)`: one cross per `v` term.
                let e: Vec<[T; 3]> = cv.terms.iter().map(|b| cross(b.at(*pj), d)).collect();
                for a in &cu.terms {
                    let a = a.at(*pj);
                    for eb in &e {
                        let x = dot(a, *eb);
                        m = Some(m.map_or(x, |m| m.min(x)));
                    }
                }
            }
            let m = m.unwrap_or_else(poison);
            over_weight(m, cu.least.powi(2), cu.most.powi(2))
        },
        T::min,
    )
}

#[cfg(test)]
#[allow(clippy::cast_precision_loss)]
mod tests {
    use super::span_derivative_scale;
    use crate::Bounds;
    use crate::dual::Dual;
    use crate::interval::Interval;
    use crate::real::Real;

    /// Knot values at every edge the quotient has: equal (a zero
    /// difference), reversed, signed zeros, subnormal and huge
    /// differences, exact and inexact quotients.
    const KNOTS: [f64; 12] = [
        -1.0,
        -0.0,
        0.0,
        5e-324,
        1e-300,
        0.1,
        1.0 / 3.0,
        0.5,
        1.0,
        3.0,
        1e300,
        f64::MAX,
    ];
    const DEGREES: [usize; 5] = [1, 2, 3, 5, 7];

    /// The spelling every routed site wrote before it called the helper.
    fn literal<T: Real>(p: usize, lo: T, hi: T) -> T {
        T::from_f64(p as f64) / (hi - lo)
    }

    /// **The helper is the sites' own spelling, bit for bit**, at the
    /// three scalars they run at: `p` entered first and the knot
    /// difference divided into it, never `p·(1/Δu)` or any other
    /// association. The goldens downstream do not see the difference,
    /// so this row is what keeps the association from drifting.
    #[test]
    fn the_scale_is_the_sites_spelling_at_every_scalar() {
        let mut asked = 0;
        for p in DEGREES {
            for lo in KNOTS {
                for hi in KNOTS {
                    let (a, b) = (span_derivative_scale(p, lo, hi), literal(p, lo, hi));
                    assert_eq!(a.to_bits(), b.to_bits(), "f64: p = {p}, [{lo}, {hi}]");
                    let (li, hi_i) = (Interval::from_f64(lo), Interval::from_f64(hi));
                    let (x, y) = (span_derivative_scale(p, li, hi_i), literal(p, li, hi_i));
                    let ends = |v: Interval| (v.lo().to_bits(), v.hi().to_bits(), v.is_certified());
                    assert_eq!(ends(x), ends(y), "Interval: p = {p}, [{lo}, {hi}]");
                    let (ld, hd) = (Dual::new(lo, 1.0), Dual::new(hi, 0.5));
                    let (x, y) = (span_derivative_scale(p, ld, hd), literal(p, ld, hd));
                    assert_eq!(
                        (x.value.to_bits(), x.deriv.to_bits()),
                        (y.value.to_bits(), y.deriv.to_bits()),
                        "Dual: p = {p}, [{lo}, {hi}]"
                    );
                    asked += 1;
                }
            }
        }
        assert_eq!(asked, DEGREES.len() * KNOTS.len() * KNOTS.len());
    }
}

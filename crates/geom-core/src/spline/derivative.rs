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
//! [`rational_span_speed_sup`] is the one statement of an upper bound
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
            let scale = span_derivative_scale(p, *held.get(i + 1)?, *held.get(i + p + 1)?);
            Some((pair[1] - pair[0]) * scale)
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

/// The pair-term bound on `sup ‖C′‖` over one nonempty `span` (module
/// docs: the derivation and why no knot difference is checked), for the
/// rational spline with control `points` and `weights` over the `held`
/// knot values of the vector `span` indexes. The weights are the
/// caller's to have checked strictly positive and finite; the answer is
/// poison (NaN) on an index past an array, never a refusal.
#[must_use]
pub fn rational_span_speed_sup<T: Real, const D: usize>(
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
    let mut sup: Option<T> = None;
    for i in span.first_derived_window() {
        let (Some(pi), Some(pn), Some(&wi), Some(&wn), Some(&lo), Some(&hi)) = (
            points.get(i),
            points.get(i + 1),
            weights.get(i),
            weights.get(i + 1),
            held.get(i + 1),
            held.get(i + p + 1),
        ) else {
            return poison;
        };
        let k = span_derivative_scale(p, lo, hi);
        let dw = wn - wi;
        for pj in active {
            let term: [T; D] =
                core::array::from_fn(|c| (wn * (pn[c] - pi[c]) - dw * (pj[c] - pi[c])) * k);
            let n = norm(term);
            sup = Some(sup.map_or(n, |s| s.max(n)));
        }
    }
    sup.map_or(poison, |s| s / least)
}

/// [`rational_span_speed_sup`] over the nonempty spans `spans` of
/// `knots` (indices into its vector, empty ones skipped): their max,
/// or poison when none is nonempty.
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

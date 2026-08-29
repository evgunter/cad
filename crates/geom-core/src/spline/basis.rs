//! Generic B-spline basis evaluation — the **ring-ops half** of the
//! spline substrate (C9): given a span index (structure, supplied by
//! the caller — see [`super::locate`]), basis values and derivatives
//! at a generic scalar `t` use only `+ − × ÷` and
//! [`Real::from_f64`]-lifted structure constants. No comparisons on
//! `t` appear anywhere in this file; every `f64` comparison below
//! reads **knots** (structure), never the parameter.
//!
//! # The span contract (binding, shared by every consumer)
//!
//! `span` must be a nonempty span index in
//! `[kv.first_span(), kv.last_span()]`; that range is checkable
//! structure, and an out-of-range span yields **all-poison** output
//! (fail-loud, D4). Within a valid span the result is the span's
//! polynomial: for `t` outside the span's knot interval the values are
//! the **polynomial extension** — documented garbage-out, like
//! `Mat3::inverse` on singular input, because detecting it would need
//! the comparison [`Real`] deliberately lacks.
//!
//! # Fixed association (D9)
//!
//! The triangular recursion runs ascending level `j = 1..=p`, ascending
//! entry `r = 0..j`, exactly as written in [`basis_funs`]; derivative
//! accumulation runs ascending `j` with the term order written in
//! [`ders_basis_funs`]. Denominators are **pure knot differences
//! computed at `f64`** (structure) and lifted once — not the textbook
//! `(U − t) + (t − U′)` form, whose exact-in-ℝ cancellation of `t`
//! would widen enclosures at the interval scalar (and could put 0 in a
//! denominator enclosure that is structurally positive).

use super::knots::{KnotVector, Span};
use crate::real::Real;

/// One term of a span evaluation: a basis value together with the
/// weighted control point it multiplies.
///
/// Only ever produced in a batch by [`span_terms`], whose length comes
/// from the control window — so "a basis value with no control point"
/// (or the reverse) is not representable, and the evaluation loop needs
/// no `zip` of independently-sized sequences.
#[derive(Clone, Copy, Debug)]
pub struct SpanTerm<'a, T, P> {
    /// `N_{i−p+r,p}(t)` for this term's control point.
    pub basis: T,
    /// The control point's weight (strictly positive `f64` structure).
    pub weight: f64,
    /// The control point itself.
    pub control: &'a P,
}

/// The `p + 1` nonvanishing basis functions on `span`, **already paired
/// with the control points and weights they multiply**.
///
/// Same recursion, same operand forms, and the same fixed association
/// as [`basis_funs`] (module docs, D9) — in particular denominators
/// stay pure knot differences, never the `(U − t) + (t − U′)` form.
/// What differs is only where the length comes from: the accumulator is
/// built *from the control window*, and one control point is consumed
/// per level, so the level count and the final length are both the
/// window's and cannot disagree with it.
///
/// Level `j` of the recursion holds `N_{i−j+r,j}`, i.e. exactly the
/// **last `j + 1`** control points of the window — which is why the
/// pairing can be carried through the fold rather than zipped on at the
/// end.
///
/// Total: a [`Span`] is valid by construction, so unlike [`basis_funs`]
/// there is no poison-on-bad-index path. A poisoned `t` still
/// propagates through the arithmetic as a value.
pub fn span_terms<'a, T: Real, P>(
    kv: &KnotVector,
    span: Span,
    t: T,
    control: &'a [P],
    weights: &'a [f64],
) -> Vec<SpanTerm<'a, T, P>> {
    use core::iter::once;

    // Both windows are the same range of two arrays whose equal length
    // is a construction invariant of every curve/surface type here.
    debug_assert_eq!(
        control.len(),
        weights.len(),
        "control/weight arrays disagree"
    );

    // Base case: N_{i,0} = 1, paired with the window's LAST control
    // point (level 0 is the last 1 of the window). `top_control` is an
    // index the window's `degree + 1` length makes total, so there is
    // no "empty window" case to discharge.
    let top = span.top_control();
    let base = vec![SpanTerm {
        basis: T::one(),
        weight: weights[top],
        control: &control[top],
    }];

    let (lower, back) = (span.lower_window(), span.lower_window());
    let back = weights[lower].iter().copied().zip(&control[back]).rev();

    let (u, i) = (kv.knots(), span.index());
    // One level per remaining control point: no `p` appears, so the
    // number of levels and the final length are the window's.
    back.fold(base, |prev, (weight, control)| {
        let j = prev.len();
        // The recursion's two knot operands, as slices rather than
        // subscripts: `u[i+1+r]` and `u[i+1+r-j]` for r in 0..j. Both
        // bounds are discharged by the `Span` invariant, and both panic
        // loudly rather than truncating if that were ever violated.
        let (hi, lo) = (&u[i + 1..=i + j], &u[i + 1 - j..=i]);
        // Shift-and-add, carrying `saved` exactly as `basis_funs` does:
        // each entry writes `saved + up`, then `saved` becomes that
        // entry's down term, and the high end writes `saved` with **no
        // addition** (`n[j] = saved`) — which is what preserves `-0.0`.
        // Carrying the accumulator instead of zipping two `Option`-
        // padded sequences means the "neither end" combination has no
        // representation to rule out, and the level's length is one
        // push per input plus the final `saved`.
        let (mut values, saved) = prev.iter().zip(hi).zip(lo).fold(
            (Vec::with_capacity(j + 1), T::zero()),
            |(mut values, saved), ((s, &a), &b)| {
                let term = s.basis / T::from_f64(a - b);
                values.push(saved + (T::from_f64(a) - t) * term);
                (values, (t - T::from_f64(b)) * term)
            },
        );
        values.push(saved);
        // The new level's controls: this level's newly admitted one,
        // then the previous level's. Both sides of this zip are
        // `1 + prev.len()` long, from the one source.
        once((weight, control))
            .chain(prev.iter().map(|s| (s.weight, s.control)))
            .zip(values)
            .map(|((weight, control), basis)| SpanTerm {
                basis,
                weight,
                control,
            })
            .collect()
    })
}

/// All-poison row, the total outcome for an invalid span index.
fn poison_row<T: Real>(len: usize) -> Vec<T> {
    vec![T::from_f64(f64::NAN); len]
}

/// The `p + 1` nonvanishing basis functions
/// `N_{span−p,p}(t), …, N_{span,p}(t)` on `span` (Book A2.2 shape,
/// structure-denominator form — module docs).
///
/// Total: an out-of-range or **empty** `span` returns all-poison; a
/// poisoned `t`
/// propagates through the arithmetic. Division safety: every
/// denominator is `knots[span+1+r] − knots[span+1+r−j] ≥
/// knots[span+1] − knots[span] > 0` for a valid (nonempty) span of a
/// validated clamped vector.
pub fn basis_funs<T: Real>(kv: &KnotVector, span: usize, t: T) -> Vec<T> {
    let p = kv.degree();
    if span < kv.first_span() || span > kv.last_span() || !kv.span_is_nonempty(span) {
        return poison_row(p + 1);
    }
    let u = kv.knots();
    let mut n = vec![T::zero(); p + 1];
    n[0] = T::one();
    for j in 1..=p {
        let mut saved = T::zero();
        for r in 0..j {
            // Indexing justified: span + 1 + r ≤ span + j ≤ span + p ≤
            // len − 2, and span + 1 + r − j ≥ span + 1 − p ≥ 1 (valid
            // span range, checked above).
            let denom = u[span + 1 + r] - u[span + 1 + r - j];
            let temp = n[r] / T::from_f64(denom);
            n[r] = saved + (T::from_f64(u[span + 1 + r]) - t) * temp;
            saved = (t - T::from_f64(u[span + 1 + r - j])) * temp;
        }
        n[j] = saved;
    }
    n
}

/// Basis functions **and derivatives** through order `n_ders`:
/// `out[k][r] = N⁽ᵏ⁾_{span−p+r,p}(t)` (`out[0]` is [`basis_funs`]'s
/// row up to rounding-identical arithmetic — it *is* the same
/// recursion). Rows with `k > degree` are exact zeros.
///
/// Derivative coefficients (the Book's Eq. 2.10 ladder) are **pure
/// `f64` structure** — quotients of knot differences with the
/// standard zero-denominator convention (a zero denominator means the
/// corresponding lower-degree basis function vanishes identically on
/// the span; its coefficient is set to exactly 0). The generic scalar
/// enters only in the stored basis triangle and the final ascending-`j`
/// accumulation `Σ a_{k,j}·N_{i+j,p−k}`, scaled by `p!/(p−k)!`.
pub fn ders_basis_funs<T: Real>(kv: &KnotVector, span: usize, t: T, n_ders: usize) -> Vec<Vec<T>> {
    let p = kv.degree();
    if span < kv.first_span() || span > kv.last_span() || !kv.span_is_nonempty(span) {
        return (0..=n_ders).map(|_| poison_row(p + 1)).collect();
    }
    let u = kv.knots();
    // The full basis triangle: tri[j][r] = N_{span−j+r, j}(t),
    // levels j = 0..=p — the same recursion as `basis_funs`, all
    // levels kept (level p−k feeds the k-th derivative).
    let mut tri: Vec<Vec<T>> = Vec::with_capacity(p + 1);
    tri.push(vec![T::one()]);
    for j in 1..=p {
        let mut level = vec![T::zero(); j + 1];
        let mut saved = T::zero();
        for r in 0..j {
            // Indexing justified as in `basis_funs`.
            let denom = u[span + 1 + r] - u[span + 1 + r - j];
            // Indexing justified: tri[j−1] has length j, r < j.
            let temp = tri[j - 1][r] / T::from_f64(denom);
            level[r] = saved + (T::from_f64(u[span + 1 + r]) - t) * temp;
            saved = (t - T::from_f64(u[span + 1 + r - j])) * temp;
        }
        level[j] = saved;
        tri.push(level);
    }

    let mut out: Vec<Vec<T>> = Vec::with_capacity(n_ders + 1);
    // Indexing justified: tri has p + 1 levels.
    out.push(tri[p].clone());
    let mut factor = 1.0f64; // p·(p−1)···(p−k+1), exact for small p
    for k in 1..=n_ders {
        if k > p {
            out.push(vec![T::zero(); p + 1]);
            continue;
        }
        factor *= (p - k + 1) as f64;
        let mut row = Vec::with_capacity(p + 1);
        for r in 0..=p {
            let i = span - p + r; // control index; span ≥ p ⇒ no underflow
            // a-ladder at f64 (structure): a_cur[j] = a_{k,j} for this i.
            let mut a_prev = vec![0.0f64; k + 1];
            let mut a_cur = vec![0.0f64; k + 1];
            a_prev[0] = 1.0;
            for kk in 1..=k {
                // Indexing justified: i + p + 1 ≤ span + p + 1 ≤ len − 1.
                for j in 0..=kk {
                    let (num, den) = if j == 0 {
                        (a_prev[0], u[i + p - kk + 1] - u[i])
                    } else if j == kk {
                        (-a_prev[kk - 1], u[i + p + 1] - u[i + kk])
                    } else {
                        (a_prev[j] - a_prev[j - 1], u[i + p - kk + 1 + j] - u[i + j])
                    };
                    // Structure comparison on a knot difference — the
                    // zero-denominator convention (fn docs).
                    a_cur[j] = if den == 0.0 { 0.0 } else { num / den };
                }
                core::mem::swap(&mut a_prev, &mut a_cur);
            }
            // Σ_j a_{k,j}·N_{i+j, p−k}: ascending j, term order as
            // written; out-of-window basis values are identically zero
            // on this span and are skipped (structure decision).
            let mut acc = T::zero();
            for (j, a) in a_prev.iter().enumerate() {
                let q_signed = (r + j) as isize - k as isize;
                if q_signed < 0 || q_signed > (p - k) as isize {
                    continue;
                }
                // Indexing justified: 0 ≤ q ≤ p − k = tri level length − 1.
                let q = q_signed as usize;
                acc = acc + T::from_f64(*a) * tri[p - k][q];
            }
            row.push(acc * T::from_f64(factor));
        }
        out.push(row);
    }
    out
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::dual::Dual64;

    fn kv(knots: &[f64], p: usize) -> KnotVector {
        KnotVector::clamped(knots.to_vec(), p).unwrap()
    }

    /// Bézier special case: single span, all basis functions are the
    /// Bernstein polynomials — closed binomial form.
    #[test]
    fn bezier_span_matches_bernstein_closed_form() {
        let k = kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3);
        let t = 0.3;
        let n = basis_funs(&k, 3, t);
        let binom = [1.0, 3.0, 3.0, 1.0];
        for (i, b) in binom.iter().enumerate() {
            let expect = b * t.powi(i as i32) * (1.0 - t).powi((3 - i) as i32);
            assert!(
                (n[i] - expect).abs() < 1e-15,
                "N[{i}] = {} vs {expect}",
                n[i]
            );
        }
    }

    #[test]
    fn partition_of_unity_and_derivative_sums() {
        let k = kv(&[0.0, 0.0, 0.0, 1.0, 2.5, 4.0, 4.0, 4.0], 2);
        for t in [0.0, 0.4, 1.0, 1.7, 2.5, 3.9, 4.0] {
            let span = k.find_span(t);
            let ders = ders_basis_funs(&k, span, t, 3);
            let sum0: f64 = ders[0].iter().sum();
            assert!((sum0 - 1.0).abs() < 1e-14, "Σ N = {sum0} at t = {t}");
            for (kk, row) in ders.iter().enumerate().skip(1) {
                let s: f64 = row.iter().sum();
                assert!(s.abs() < 1e-12, "Σ N^({kk}) = {s} at t = {t}");
            }
            // k > p rows are exact zeros.
            assert!(ders[3].iter().all(|d| *d == 0.0));
        }
    }

    #[test]
    fn first_derivative_matches_dual_and_finite_difference() {
        let k = kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 3.0, 3.0, 3.0, 3.0], 3);
        for t in [0.2, 0.99, 1.0, 2.4] {
            let span = k.find_span(t);
            let ders = ders_basis_funs(&k, span, t, 2);
            let dual = basis_funs(&k, span, Dual64::variable(t));
            let h = 1e-7;
            let lo = basis_funs(&k, span, t - h);
            let hi = basis_funs(&k, span, t + h);
            for r in 0..=3 {
                // Dual channel agrees exactly (same recursion).
                assert!(
                    (ders[1][r] - dual[r].deriv).abs() < 1e-9,
                    "ders vs dual at t = {t}, r = {r}: {} vs {}",
                    ders[1][r],
                    dual[r].deriv
                );
                let fd = (hi[r] - lo[r]) / (2.0 * h);
                assert!(
                    (ders[1][r] - fd).abs() < 1e-5,
                    "ders vs FD at t = {t}, r = {r}: {} vs {fd}",
                    ders[1][r]
                );
                // Value row is the basis_funs row, bit-identical
                // (same recursion, same association).
                assert_eq!(ders[0][r].to_bits(), dual[r].value.to_bits());
            }
        }
    }

    #[test]
    fn invalid_span_poisons_totally() {
        let k = kv(&[0.0, 0.0, 1.0, 1.0], 1);
        assert!(basis_funs(&k, 0, 0.5).iter().all(|x: &f64| x.is_nan()));
        assert!(basis_funs(&k, 9, 0.5).iter().all(|x: &f64| x.is_nan()));
        let d = ders_basis_funs(&k, 9, 0.5, 1);
        assert!(d.iter().flatten().all(|x: &f64| x.is_nan()));
    }
}

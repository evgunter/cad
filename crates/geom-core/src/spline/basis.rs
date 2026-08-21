//! Generic B-spline basis evaluation — the **ring-ops half** of the
//! spline substrate (C9): given a [`Span`] (structure, supplied by the
//! caller — see [`super::locate`]), basis values and derivatives
//! at a generic scalar `t` use only `+ − × ÷` and
//! [`Real::from_f64`]-lifted structure constants. No comparisons on
//! `t` appear anywhere in this file; every `f64` comparison below
//! reads **knots** (structure), never the parameter.
//!
//! # The span contract (binding, shared by every consumer)
//!
//! **Checked, not structural.** A [`Span`] is proven nonempty and in
//! range *for the vector it was drawn from*, but it carries no borrow
//! of that vector, so a span of some **other** `KnotVector` is a
//! representable input here. Every entry point below therefore asks
//! [`KnotVector::admits`] first and returns an **all-poison** row for a
//! span this `kv` does not admit (fail-loud, D4) — never an
//! out-of-bounds knot read and never a row divided by a zero knot gap.
//!
//! What the check buys here, exactly. Degree agreement gives
//! `span.index() >= kv.first_span()`, which keeps the low knot read
//! `u[span + 1 + r − j] >= u[span + 1 − p]` at a non-negative index,
//! and makes the returned row the same length as the window the caller
//! will index with. `span.index() <= kv.last_span()` bounds the high
//! reads — `u[span + p]` here, `u[i + p + 1]` in the derivative ladder
//! — at `u.len() − 1`, exactly. Nonemptiness is what the division
//! safety note below rests on: it is the `> 0` in
//! `knots[span+1] − knots[span] > 0`, and a foreign span can be an
//! empty span of *this* vector even when both index bounds hold.
//!
//! What the check does **not** buy is the right vector: a span from a
//! different vector of the same degree and control count, nonempty at
//! that index in both, is admitted, and the answer is then this
//! vector's polynomial on the wrong span rather than a refusal
//! ([`KnotVector::admits`] states the residue).
//!
//! Within a span the result is the span's polynomial: for `t` outside
//! the span's knot interval the values are the **polynomial extension**
//! — documented garbage-out, like `Mat3::inverse` on singular input,
//! because detecting it would need the comparison [`Real`] deliberately
//! lacks.
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

/// All-poison row — the total outcome for a span `kv` does not admit
/// (module docs: the span contract).
fn poison_row<T: Real>(len: usize) -> Vec<T> {
    vec![T::from_f64(f64::NAN); len]
}

/// The `p + 1` nonvanishing basis functions
/// `N_{span−p,p}(t), …, N_{span,p}(t)` on `span` (Book A2.2 shape,
/// structure-denominator form — module docs).
///
/// Total: a span `kv` does not admit ([`KnotVector::admits`]) returns
/// an all-poison row of the same length, and a poisoned `t` propagates
/// through the arithmetic. Division safety: every denominator is
/// `knots[span+1+r] − knots[span+1+r−j] ≥ knots[span+1] − knots[span] > 0`,
/// which is exactly the nonemptiness [`KnotVector::admits`] checks.
///
/// That estimate is why the inner loop stops at `r = j − 1` and the
/// level's high end is written after it as `n[j] = saved`. It needs
/// `span + 1 + r − j ≤ span`, which fails at `r = j`: the denominator
/// becomes `knots[span+1+j] − knots[span+1]`, no longer straddling the
/// span's own gap, and at a clamped end (where those knots coincide) it
/// is exactly zero — so folding the tail write into the loop for
/// uniformity computes `0/0` and poisons the row.
pub fn basis_funs<T: Real>(kv: &KnotVector, span: Span, t: T) -> Vec<T> {
    let p = kv.degree();
    if !kv.admits(span) {
        return poison_row(p + 1);
    }
    // The index, once — the recursion below reads knots around it. Its
    // range facts (`span ≥ p`, `span + 1 < len`) hold against THIS `kv`
    // by the check above, not by the `Span`'s own construction.
    let span = span.index();
    let u = kv.knots();
    let mut n = vec![T::zero(); p + 1];
    n[0] = T::one();
    for j in 1..=p {
        let mut saved = T::zero();
        for r in 0..j {
            // Indexing justified: span + 1 + r ≤ span + j ≤ span + p ≤
            // len − 2, and span + 1 + r − j ≥ span + 1 − p ≥ 1 — the
            // `Span`'s own range invariant, established at construction.
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
///
/// Total in the same way [`basis_funs`] is: a span `kv` does not admit
/// yields `n_ders + 1` all-poison rows.
pub fn ders_basis_funs<T: Real>(kv: &KnotVector, span: Span, t: T, n_ders: usize) -> Vec<Vec<T>> {
    let p = kv.degree();
    if !kv.admits(span) {
        return (0..=n_ders).map(|_| poison_row(p + 1)).collect();
    }
    // The window's base, subtracted once inside `Span`, and the index
    // the knot reads are centred on. Both are in range for THIS `kv` by
    // the check above: the ladder's highest read is `u[i + p + 1]` at
    // `i = base + p = span.index()`, which `admits` bounds at
    // `u.len() − 1` exactly.
    let base = span.first_control();
    let span = span.index();
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
            let i = base + r; // control index — the window's base plus r
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
        let n = basis_funs(&k, k.span(3).unwrap(), t);
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
            let span = k.span_at(t);
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
            let span = k.span_at(t);
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
}

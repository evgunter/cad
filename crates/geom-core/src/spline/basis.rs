//! Generic B-spline basis evaluation — the **ring-ops half** of the
//! spline substrate (C9): given a [`Span`] (structure, supplied by the
//! caller — see [`super::locate`]), basis values and derivatives
//! at a generic scalar `t` use only `+ − × ÷` over the knots, read
//! either as the vector's `f64` values lifted ([`basis_funs`]) or as
//! the values a [`super::Knots`] holds in `T` ([`basis_funs_in`]). No
//! comparisons on `t` appear anywhere in this file; every `f64`
//! comparison below reads **knots** (structure), never the parameter.
//!
//! # The span contract (binding, shared by every consumer)
//!
//! **Structural.** A [`Span`] borrows the knot structure it is a proof
//! about, and every entry point below reads its knots from that
//! borrow: the `f64` entry points through [`Span::knots_f64_interim`],
//! the held (`_in`) ones through the values the same
//! [`super::KnotSpan`] pairs with it. None takes a second vector. So
//! there is no pairing to check and no refusal to answer: the span's
//! own invariants are invariants of the very knots being read.
//!
//! What they give, exactly. `span.index() >= kv.first_span()` (the
//! window base is `index − degree`, subtracted at construction) keeps
//! the low knot read `u[span + 1 + r − j] >= u[span + 1 − p]` at a
//! non-negative index, and makes the returned row the same length as
//! the window the caller will index with. `span.index() <=
//! kv.last_span()` bounds the high reads — `u[span + p]` here,
//! `u[i + p + 1]` in the derivative ladder — at `u.len() − 1`,
//! exactly. Nonemptiness is what the division safety note below rests
//! on: it is the `> 0` in `knots[span+1] − knots[span] > 0`.
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
//! [`ders_basis_funs`]. Denominators are **pure knot differences**,
//! two knots subtracted in `T` — not the textbook
//! `(U − t) + (t − U′)` form, whose exact-in-ℝ cancellation of `t`
//! would widen enclosures at the interval scalar (and could put 0 in a
//! denominator enclosure that is structurally positive).

use super::knot_values::KnotSpan;
use super::knots::Span;
use crate::real::Real;

/// Where a basis reads its knots: the vector's `f64` values lifted
/// ([`Span`]), or the values a [`super::Knots`] holds in the scalar
/// ([`KnotSpan`]). The structure (`kv`) is the same `f64` vector either
/// way, and every decision below reads it alone.
trait KnotRead<T> {
    /// The structural span.
    fn span(&self) -> Span<'_>;
    /// Knot `i` in the scalar.
    fn at(&self, i: usize) -> T;
    /// `knot[hi] − knot[lo]` in the scalar.
    fn gap(&self, hi: usize, lo: usize) -> T
    where
        T: Real,
    {
        self.at(hi) - self.at(lo)
    }
}

/// The `f64` knots lifted: each read is [`Real::from_f64`] of a stored
/// knot.
struct Lifted<'a>(Span<'a>);

impl<T: Real> KnotRead<T> for Lifted<'_> {
    fn span(&self) -> Span<'_> {
        self.0
    }
    fn at(&self, i: usize) -> T {
        T::from_f64(self.0.knots_f64_interim().knots()[i])
    }
}

impl<T: Real> KnotRead<T> for KnotSpan<'_, T> {
    fn span(&self) -> Span<'_> {
        KnotSpan::span(*self)
    }
    fn at(&self, i: usize) -> T {
        self.values()[i]
    }
}

/// The `p + 1` nonvanishing basis functions
/// `N_{span−p,p}(t), …, N_{span,p}(t)` on `span` (Book A2.2 shape,
/// structure-denominator form — module docs), the knots read as the
/// vector's `f64` values.
///
/// Total: a poisoned `t` propagates through the arithmetic, and there
/// is no other route out. Division safety: every denominator is
/// `knots[span+1+r] − knots[span+1+r−j] ≥ knots[span+1] − knots[span] > 0`,
/// which is exactly the nonemptiness the span carries about these
/// knots.
///
/// That estimate is why the inner loop stops at `r = j − 1` and the
/// level's high end is written after it as `n[j] = saved`. It needs
/// `span + 1 + r − j ≤ span`, which fails at `r = j`: the denominator
/// becomes `knots[span+1+j] − knots[span+1]`, no longer straddling the
/// span's own gap, and at a clamped end (where those knots coincide) it
/// is exactly zero — so folding the tail write into the loop for
/// uniformity computes `0/0` and poisons the row.
pub fn basis_funs<T: Real>(span: Span<'_>, t: T) -> Vec<T> {
    basis_row(&Lifted(span), t)
}

/// [`basis_funs`] with the knots read as the values `span`'s
/// [`super::Knots`] holds in `T`: the same recursion, so a knot that is
/// a function of the variables enters the row as that function.
pub fn basis_funs_in<T: Real>(span: KnotSpan<'_, T>, t: T) -> Vec<T> {
    basis_row(&span, t)
}

/// The triangle's level `p`, read through `k` (the recursion of
/// [`basis_funs`]).
fn basis_row<T: Real>(k: &impl KnotRead<T>, t: T) -> Vec<T> {
    let p = k.span().degree();
    // The index, once — the recursion below reads knots around it. Its
    // range facts (`span ≥ p`, `span + 1 < len`) are the `Span`'s own
    // construction invariants, about these very knots.
    let span = k.span().index();
    let mut n = vec![T::zero(); p + 1];
    n[0] = T::one();
    for j in 1..=p {
        let mut saved = T::zero();
        for (r, nr) in n.iter_mut().enumerate().take(j) {
            // Indexing justified: span + 1 + r ≤ span + j ≤ span + p ≤
            // len − 2, and span + 1 + r − j ≥ span + 1 − p ≥ 1 — the
            // `Span`'s own range invariant, established at construction.
            let temp = *nr / k.gap(span + 1 + r, span + 1 + r - j);
            *nr = saved + (k.at(span + 1 + r) - t) * temp;
            saved = (t - k.at(span + 1 + r - j)) * temp;
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
/// Derivative coefficients (the Book's Eq. 2.10 ladder) are quotients
/// of knot differences, with the standard zero-denominator convention:
/// a difference of two equal `f64` knots (structure) means the
/// corresponding lower-degree basis function vanishes identically on
/// the span, and its coefficient is exactly 0. The ladder is computed
/// in `T` over the knots as read, then accumulated
/// `Σ a_{k,j}·N_{i+j,p−k}` in ascending `j` and scaled by `p!/(p−k)!`.
///
/// Total in the same way [`basis_funs`] is.
pub fn ders_basis_funs<T: Real>(span: Span<'_>, t: T, n_ders: usize) -> Vec<Vec<T>> {
    ders_rows(&Lifted(span), t, n_ders)
}

/// [`ders_basis_funs`] with the knots read as the values `span`'s
/// [`super::Knots`] holds in `T` (as [`basis_funs_in`]).
pub fn ders_basis_funs_in<T: Real>(span: KnotSpan<'_, T>, t: T, n_ders: usize) -> Vec<Vec<T>> {
    ders_rows(&span, t, n_ders)
}

/// The rows of [`ders_basis_funs`], read through `k`.
fn ders_rows<T: Real>(k: &impl KnotRead<T>, t: T, n_ders: usize) -> Vec<Vec<T>> {
    let structural = k.span();
    let st = structural.structure();
    let p = st.degree();
    // The window's base, subtracted once inside `Span`, and the index
    // the knot reads are centred on. Both are in range for these knots
    // by the span's construction: the ladder's highest read is
    // `u[i + p + 1]` at `i = base + p = span.index()`, and
    // `index <= last_span()` puts that at `u.len() − 1` exactly.
    let base = structural.first_control();
    let span = structural.index();
    // The full basis triangle: tri[j][r] = N_{span−j+r, j}(t),
    // levels j = 0..=p — the same recursion as `basis_funs`, all
    // levels kept (level p−k feeds the k-th derivative).
    let mut tri: Vec<Vec<T>> = Vec::with_capacity(p + 1);
    tri.push(vec![T::one()]);
    for j in 1..=p {
        let mut level = vec![T::zero(); j + 1];
        let mut saved = T::zero();
        for r in 0..j {
            // Indexing justified as in `basis_funs`; tri[j−1] has
            // length j, r < j.
            let temp = tri[j - 1][r] / k.gap(span + 1 + r, span + 1 + r - j);
            level[r] = saved + (k.at(span + 1 + r) - t) * temp;
            saved = (t - k.at(span + 1 + r - j)) * temp;
        }
        level[j] = saved;
        tri.push(level);
    }

    let mut out: Vec<Vec<T>> = Vec::with_capacity(n_ders + 1);
    // Indexing justified: tri has p + 1 levels.
    out.push(tri[p].clone());
    let mut factor = 1.0f64; // p·(p−1)···(p−k+1), exact for small p
    for kd in 1..=n_ders {
        if kd > p {
            out.push(vec![T::zero(); p + 1]);
            continue;
        }
        factor *= (p - kd + 1) as f64;
        let mut row = Vec::with_capacity(p + 1);
        for r in 0..=p {
            let i = base + r; // control index — the window's base plus r
            // The a-ladder: a_cur[j] = a_{kd,j} for this i.
            let mut a_prev = vec![T::zero(); kd + 1];
            let mut a_cur = vec![T::zero(); kd + 1];
            a_prev[0] = T::one();
            for kk in 1..=kd {
                // Indexing justified: i + p + 1 ≤ span + p + 1 ≤ len − 1.
                for j in 0..=kk {
                    let (num, hi, lo) = if j == 0 {
                        (a_prev[0], i + p - kk + 1, i)
                    } else if j == kk {
                        (-a_prev[kk - 1], i + p + 1, i + kk)
                    } else {
                        (a_prev[j] - a_prev[j - 1], i + p - kk + 1 + j, i + j)
                    };
                    // A knot difference within one run is the zero of
                    // the zero-denominator convention (fn docs): a
                    // structure read, never a position (C6).
                    a_cur[j] = if st.same_run(hi, lo) {
                        T::zero()
                    } else {
                        num / k.gap(hi, lo)
                    };
                }
                core::mem::swap(&mut a_prev, &mut a_cur);
            }
            // Σ_j a_{kd,j}·N_{i+j, p−kd}: ascending j, term order as
            // written; out-of-window basis values are identically zero
            // on this span and are skipped (structure decision).
            let mut acc = T::zero();
            for (j, a) in a_prev.iter().enumerate() {
                let q_signed = (r + j) as isize - kd as isize;
                if q_signed < 0 || q_signed > (p - kd) as isize {
                    continue;
                }
                // Indexing justified: 0 ≤ q ≤ p − kd = tri level length − 1.
                let q = q_signed as usize;
                acc = acc + *a * tri[p - kd][q];
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
    use crate::spline::knots::KnotVector;

    fn kv(knots: &[f64], p: usize) -> KnotVector {
        KnotVector::clamped(knots.to_vec(), p).unwrap()
    }

    /// Bézier special case: single span, all basis functions are the
    /// Bernstein polynomials — closed binomial form.
    #[test]
    fn bezier_span_matches_bernstein_closed_form() {
        let k = kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3);
        let t = 0.3;
        let n = basis_funs(k.span(3).unwrap(), t);
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
            let span = k.span_at(t).expect("a numeric parameter");
            let ders = ders_basis_funs(span, t, 3);
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
            let span = k.span_at(t).expect("a numeric parameter");
            let ders = ders_basis_funs(span, t, 2);
            let dual = basis_funs(span, Dual64::variable(t));
            let h = 1e-7;
            let lo = basis_funs(span, t - h);
            let hi = basis_funs(span, t + h);
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

    /// A spread of vectors: degrees 1 to 5, interior multiplicities up
    /// to the degree, a single span, uneven spacing.
    fn vectors() -> Vec<KnotVector> {
        vec![
            kv(&[0.0, 0.0, 1.0, 1.0], 1),
            kv(&[0.0, 0.0, 0.0, 0.3, 1.7, 4.0, 4.0, 4.0], 2),
            kv(&[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 3.0, 3.0, 3.0, 3.0], 3),
            kv(
                &[-1.0, -1.0, -1.0, -1.0, 0.5, 0.5, 0.5, 2.0, 2.0, 2.0, 2.0],
                3,
            ),
            kv(
                &[
                    0.0, 0.0, 0.0, 0.0, 0.0, 0.25, 0.6, 0.6, 1.0, 1.0, 1.0, 1.0, 1.0,
                ],
                4,
            ),
            kv(
                &[0.0; 6]
                    .iter()
                    .chain(&[0.7, 0.7])
                    .chain(&[1.0; 6])
                    .copied()
                    .collect::<Vec<_>>(),
                5,
            ),
        ]
    }

    /// The held read of a vector's own values is the lifted read: one
    /// recursion over the same numbers, bit-identical at `f64` and at
    /// `Dual<f64>` (a parameter variable), every derivative order up to
    /// `p + 2`, at knots and between them.
    #[test]
    fn held_knots_read_bit_identically_to_the_vector() {
        let bits = |rows: &[Vec<f64>]| -> Vec<u64> {
            rows.iter().flatten().map(|x| x.to_bits()).collect()
        };
        let dual_bits = |rows: &[Vec<Dual64>]| -> Vec<(u64, u64)> {
            rows.iter()
                .flatten()
                .map(|x| (x.value.to_bits(), x.deriv.to_bits()))
                .collect()
        };
        let mut cases = 0;
        for k in vectors() {
            let p = k.degree();
            let held = crate::spline::Knots::from(k.clone());
            let lifted_d = held
                .map(Dual64::constant)
                .expect("a lift keeps every point");
            let (lo, hi) = k.domain();
            let mut ts: Vec<f64> = k.knots().to_vec();
            ts.extend((0..=8).map(|i| lo + (hi - lo) * f64::from(i) / 8.0));
            for t in ts {
                let (span, _) = held.locate(t).expect("a numeric parameter");
                let lifted = ders_basis_funs(span.span(), t, p + 2);
                let read = ders_basis_funs_in(span, t, p + 2);
                assert_eq!(bits(&lifted), bits(&read), "f64 ders, p = {p}, t = {t}");
                let row = basis_funs_in(span, t);
                assert_eq!(
                    bits(&[row]),
                    bits(&lifted[..1]),
                    "f64 row, p = {p}, t = {t}"
                );

                let td = Dual64::variable(t);
                let (dspan, _) = lifted_d.locate(td).expect("a numeric parameter");
                let lifted = ders_basis_funs(dspan.span(), td, p + 2);
                let read = ders_basis_funs_in(dspan, td, p + 2);
                assert_eq!(
                    dual_bits(&lifted),
                    dual_bits(&read),
                    "dual ders, p = {p}, t = {t}"
                );
                cases += 1;
            }
        }
        assert!(cases > 60, "the spread shrank to {cases} cases");
    }

    /// A breakpoint held as a function of a variable enters the row as
    /// that function: its derivative channel is the row's derivative in
    /// the breakpoint, against a central difference of rebuilt `f64`
    /// vectors.
    #[test]
    fn a_held_knot_variable_moves_the_row() {
        let nz = |n| core::num::NonZeroUsize::new(n).unwrap();
        let a = 0.8;
        let c = Dual64::constant;
        let held = crate::spline::Knots::new(
            vec![
                (c(0.0), nz(3)),
                (Dual64::variable(a), nz(1)),
                (c(2.0), nz(3)),
            ],
            2,
        )
        .unwrap();
        let t = c(1.2);
        let (span, _) = held.locate(t).expect("a numeric parameter");
        let row = basis_funs_in(span, t);
        let ders = ders_basis_funs_in(span, t, 1);
        let h = 1e-6;
        let rebuilt = |x: f64| kv(&[0.0, 0.0, 0.0, x, 2.0, 2.0, 2.0], 2);
        let (lo_kv, hi_kv) = (rebuilt(a - h), rebuilt(a + h));
        let lo = ders_basis_funs(lo_kv.span_at(1.2).unwrap(), 1.2, 1);
        let hi = ders_basis_funs(hi_kv.span_at(1.2).unwrap(), 1.2, 1);
        let mut moved = false;
        for r in 0..=2 {
            for (kd, (l, h_row)) in lo.iter().zip(&hi).enumerate() {
                let fd = (h_row[r] - l[r]) / (2.0 * h);
                let got = if kd == 0 {
                    row[r].deriv
                } else {
                    ders[kd][r].deriv
                };
                assert!((got - fd).abs() < 1e-6, "d/da row {kd}[{r}]: {got} vs {fd}");
                moved |= fd.abs() > 1e-3;
            }
        }
        assert!(moved, "the fixture's row does not depend on the knot");
    }

    /// At `Interval` a coefficient over two point knots encloses the
    /// real one. The knots `2⁻⁶⁰` and `1` differ by `1 − 2⁻⁶⁰`, which
    /// rounds to `1` in `f64`; the real slope `1/(1 − 2⁻⁶⁰)` of the
    /// degree-1 basis is above `1`, so the enclosure must reach past it.
    #[test]
    fn an_interval_coefficient_encloses_the_unrounded_knot_gap() {
        use crate::interval::Interval;
        use crate::real::Bounds;
        let tiny = 2f64.powi(-60);
        let k = kv(&[tiny, tiny, 1.0, 1.0], 1);
        let t = Interval::from_bounds(0.5, 0.5);
        let span = k.span_at(0.5).unwrap();
        let ders = ders_basis_funs(span, t, 1);
        assert!(
            Bounds::hi(ders[1][1]) > 1.0,
            "N'_1 = [{}, {}] misses 1/(1 − 2⁻⁶⁰)",
            Bounds::lo(ders[1][1]),
            Bounds::hi(ders[1][1])
        );
        let row = basis_funs(span, t);
        assert!(
            Bounds::hi(row[0]) > 0.5,
            "N_0(0.5) misses its real value above 0.5"
        );
    }
}

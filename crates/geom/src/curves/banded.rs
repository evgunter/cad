//! The banded LU of the interpolating fit's collocation matrix — the
//! solve behind [`super::fit::Collocation`], and nothing else.
//!
//! # The rows it takes
//!
//! Row `i` is the `p + 1` basis values alive at the parameter `ū_i`,
//! standing in columns `first[i] .. first[i] + p + 1` from its span's
//! first control index. Those values are finite and nonnegative, and
//! never `-0.0`: the Cox–de Boor recurrence forms them from sums and
//! products of nonnegative numbers, and the unit-weight rational
//! normalisation divides them by a positive sum. The module is private
//! to the curve stack so that this is the only matrix that reaches it:
//! its identity with the dense solve, and its stability, are facts about
//! these rows and are not claimed for any other matrix.
//!
//! # Shape
//!
//! `first` never decreases from one row to the next (the
//! **first-nonzero property**): at ascending parameters the span never
//! moves left. It is checked, and a row that breaks it refuses typed
//! ([`LsqError::BandShape`]). Under the property Doolittle without
//! pivoting creates no fill outside the windows: `L[i][q]` is zero for
//! `q < first[i]`, and `U[i][j]` is zero for `j ≥ first[i] + width`,
//! because every row above `i` ends its window no later than row `i`
//! does. So the factor lives in the windows themselves, `n · width`
//! values, and costs `O(n · width²)`.
//!
//! # Why no pivoting is safe
//!
//! A B-spline collocation matrix whose parameters satisfy the
//! Schoenberg–Whitney conditions (`u_j < ū_j < u_{j+p+1}` inside the
//! knot range, `N_j(ū_j) ≠ 0`) is totally positive: C. de Boor, *Total
//! positivity of the spline collocation matrix*, Indiana Univ. Math. J.
//! 25 (1976) 541–551. The averaged knot vector the fit builds (Eq. 9.8,
//! `u_{j+p}` the mean of `ū_j … ū_{j+p−1}`) satisfies them at strictly
//! ascending parameters: `u_j` is a mean of parameters below `ū_j` and
//! `u_{j+p+1}` a mean of parameters above it. Gaussian elimination
//! without pivoting on a totally positive matrix computes nonnegative
//! `L` and `U` factors for small enough unit roundoff, so nothing
//! cancels and the backward error is small with no pivoting at all:
//! C. de Boor and A. Pinkus, *Backward error analysis for totally
//! positive linear systems*, Numer. Math. 27 (1977) 485–490, whose
//! stated application is exactly this banded B-spline system.
//!
//! # Identity with the dense solve
//!
//! The factorisation is `geom_core::linalg::lsq::solve_square`'s
//! Doolittle with every loop restricted to the windows: each stored
//! entry is formed from the same terms in the same ascending order, less
//! terms that are an exact `±0 · x` with `x` finite. Dropping such a
//! term changes a running sum only when that sum is `-0.0`
//! (`-0.0 − (−0.0)` is `+0.0`), and a sum that starts at a value other
//! than `-0.0` never becomes `-0.0` under subtraction; every sum in the
//! factorisation starts at a row value, so the factor is bit-identical
//! to the dense one while it stays finite. A factor entry that is not
//! finite refuses ([`factor`]), where the dense elimination would carry
//! it on into NaN.
//!
//! The substitutions start from the right-hand side, which may hold
//! `-0.0` or non-finite values, so [`BandedLu::solve`] applies the dense
//! solve's dropped terms in summary, each where the dense sum meets it:
//! a `-0.0` sum becomes `+0.0` when a dropped product carries a sign
//! bit, and the first dropped product against a non-finite value is
//! subtracted as it stands, which carries that value's NaN payload the
//! way the dense sum does. The solution is then bit-identical to the
//! dense solve's, NaN payloads included.

use geom_core::linalg::lsq::LsqError;

/// A factored collocation matrix ([`factor`]); `L` below the diagonal
/// (unit diagonal implied) and `U` on and above it, row `i`'s window at
/// `i * width ..`, column `j` at offset `j − first[i]`.
#[derive(Clone, Debug)]
pub(crate) struct BandedLu {
    first: Vec<usize>,
    width: usize,
    lu: Vec<f64>,
}

/// Factor the collocation matrix whose row `i` is `rows[i]`, standing in
/// columns `first[i] .. first[i] + rows[i].len()` (the module docs for
/// the rows it takes, the shape, and the identity).
///
/// # Errors
///
/// - [`LsqError::Empty`], [`LsqError::RowLengthMismatch`] for the rows;
/// - [`LsqError::BandShape`] when `first` breaks the shape;
/// - [`LsqError::LsqDegenerate`] on a zero or non-finite pivot, at the
///   index the dense solve names. A row whose window leaves out its own
///   diagonal makes the matrix singular (the rows from there down, or
///   up, outnumber the columns they can reach) and refuses at that row
///   with the pivot `0.0` the dense elimination computes there. A
///   non-finite `L` entry refuses at the pivot it was divided by, and a
///   non-finite `U` entry at its own row, carrying the entry.
// Explicit index loops on purpose: the association order is the
// contract.
#[allow(clippy::needless_range_loop)]
pub(crate) fn factor(first: &[usize], rows: &[Vec<f64>]) -> Result<BandedLu, LsqError> {
    let Some(width) = rows.first().map(Vec::len).filter(|w| *w > 0) else {
        return Err(LsqError::Empty);
    };
    let n = rows.len();
    for (row, r) in rows.iter().enumerate() {
        if r.len() != width {
            return Err(LsqError::RowLengthMismatch {
                row,
                len: r.len(),
                expected: width,
            });
        }
    }
    if first.len() != n {
        return Err(LsqError::BandShape {
            row: first.len().min(n),
        });
    }
    for i in 0..n {
        let rises = i == 0 || first[i - 1] <= first[i];
        let inside = first[i].checked_add(width).is_some_and(|end| end <= n);
        if !rises || !inside {
            return Err(LsqError::BandShape { row: i });
        }
    }
    let mut lu: Vec<f64> = rows.iter().flatten().copied().collect();
    for i in 0..n {
        let f = first[i];
        if i < f || i >= f + width {
            return Err(LsqError::LsqDegenerate {
                pivot_index: i,
                pivot: 0.0,
            });
        }
        let row = i * width;
        // `U[t][j]` is stored when `j < first[t] + width`; past that it
        // is a structural zero.
        for q in f..i {
            let mut s = lu[row + q - f];
            for t in f..q {
                if q < first[t] + width {
                    s -= lu[row + t - f] * lu[t * width + q - first[t]];
                }
            }
            let pivot = lu[q * width + q - first[q]];
            let l = s / pivot;
            if !l.is_finite() {
                return Err(LsqError::LsqDegenerate {
                    pivot_index: q,
                    pivot,
                });
            }
            lu[row + q - f] = l;
        }
        for j in i..f + width {
            let mut s = lu[row + j - f];
            for t in f..i {
                if j < first[t] + width {
                    s -= lu[row + t - f] * lu[t * width + j - first[t]];
                }
            }
            if !s.is_finite() {
                return Err(LsqError::LsqDegenerate {
                    pivot_index: i,
                    pivot: s,
                });
            }
            lu[row + j - f] = s;
        }
        let pivot = lu[row + i - f];
        if pivot == 0.0 {
            return Err(LsqError::LsqDegenerate {
                pivot_index: i,
                pivot,
            });
        }
    }
    Ok(BandedLu {
        first: first.to_vec(),
        width,
        lu,
    })
}

impl BandedLu {
    /// Solve `A·x = b` per right-hand-side column; `b` is `n × k`, the
    /// result `n × k`. Forward substitution sums ascending, back
    /// substitution descending, as the dense solve does, with whose
    /// result this one is bit-identical (module docs).
    ///
    /// # Errors
    ///
    /// [`LsqError::RhsShapeMismatch`], [`LsqError::RowLengthMismatch`]
    /// and [`LsqError::Empty`] for a malformed `b`, as the dense solve
    /// refuses them.
    // Explicit index loops on purpose — see `factor`.
    #[allow(clippy::needless_range_loop)]
    pub(crate) fn solve(&self, b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, LsqError> {
        let n = self.first.len();
        let w = self.width;
        if b.len() != n {
            return Err(LsqError::RhsShapeMismatch {
                rhs_rows: b.len(),
                matrix_rows: n,
            });
        }
        let k = b.first().map_or(0, Vec::len);
        if k == 0 {
            return Err(LsqError::Empty);
        }
        for (row, r) in b.iter().enumerate() {
            if r.len() != k {
                return Err(LsqError::RowLengthMismatch {
                    row,
                    len: r.len(),
                    expected: k,
                });
            }
        }
        let at = |i: usize, j: usize| self.lu[i * w + j - self.first[i]];
        let mut x: Vec<Vec<f64>> = b.to_vec();
        for c in 0..k {
            // `L[i][q]` for `q < first[i]` is `+0.0 / U[q][q]`: a zero
            // carrying the pivot's sign. The dense sum subtracts it
            // times `y_q` BEFORE the window's terms, ascending `q`;
            // `dropped` counts the rows summarised so far, `flips` says
            // whether one of those products is `-0.0`, and `poison`
            // holds the first that is not finite.
            let (mut dropped, mut flips, mut poison) = (0usize, false, None);
            for i in 0..n {
                let f = self.first[i];
                while dropped < f {
                    let y = x[dropped][c];
                    let l = 0.0f64.copysign(at(dropped, dropped));
                    if !y.is_finite() {
                        poison = poison.or(Some((l, y)));
                    } else if (l * y).is_sign_negative() {
                        flips = true;
                    }
                    dropped += 1;
                }
                let mut s = x[i][c];
                if let Some((l, y)) = poison {
                    s -= l * y;
                } else if flips {
                    s -= -0.0;
                }
                for q in f..i {
                    s -= at(i, q) * x[q][c];
                }
                x[i][c] = s;
            }
            // `U[i][j]` for `j ≥ first[i] + width` is `+0.0`; the dense
            // sum subtracts it times `x_j` AFTER the window's terms,
            // ascending `j`, so the first non-finite value it meets is
            // the lowest-indexed one.
            let (mut kept, mut flips, mut poison) = (n, false, None);
            for i in (0..n).rev() {
                let end = self.first[i] + w;
                while kept > end {
                    kept -= 1;
                    let v = x[kept][c];
                    if !v.is_finite() {
                        poison = Some(v);
                    } else if v.is_sign_negative() {
                        flips = true;
                    }
                }
                let mut s = x[i][c];
                for q in (i + 1)..end {
                    s -= at(i, q) * x[q][c];
                }
                if let Some(v) = poison {
                    s -= 0.0 * v;
                } else if flips {
                    s -= -0.0;
                }
                x[i][c] = s / at(i, i);
            }
        }
        Ok(x)
    }
}

/// The dense Doolittle `solve_square` runs, in the same association
/// order term for term, with `U` stored by column so a large `n` stays
/// in cache: the reference for identity rows too large for
/// `solve_square`'s strided reads in a debug build. Pinned to
/// `solve_square` bit for bit by `the_cached_reference_is_solve_square`.
#[cfg(test)]
#[allow(clippy::needless_range_loop)]
pub(crate) fn dense_reference(
    first: &[usize],
    rows: &[Vec<f64>],
    b: &[Vec<f64>],
) -> Result<Vec<Vec<f64>>, LsqError> {
    let n = rows.len();
    let (mut l, mut ut) = (vec![0.0f64; n * n], vec![0.0f64; n * n]);
    for (i, (f, r)) in first.iter().zip(rows).enumerate() {
        for (k, v) in r.iter().enumerate() {
            let j = f + k;
            if j >= i {
                ut[j * n + i] = *v;
            } else {
                l[i * n + j] = *v;
            }
        }
    }
    for i in 0..n {
        for j in i..n {
            let mut s = ut[j * n + i];
            let (li, uj) = (&l[i * n..i * n + i], &ut[j * n..j * n + i]);
            for q in 0..i {
                s -= li[q] * uj[q];
            }
            ut[j * n + i] = s;
        }
        let pivot = ut[i * n + i];
        if pivot == 0.0 || !pivot.is_finite() {
            return Err(LsqError::LsqDegenerate {
                pivot_index: i,
                pivot,
            });
        }
        for r in (i + 1)..n {
            let mut s = l[r * n + i];
            let (lr, ui) = (&l[r * n..r * n + i], &ut[i * n..i * n + i]);
            for q in 0..i {
                s -= lr[q] * ui[q];
            }
            l[r * n + i] = s / pivot;
        }
    }
    let k = b.first().map_or(0, Vec::len);
    let mut x = b.to_vec();
    for c in 0..k {
        for i in 0..n {
            let mut s = x[i][c];
            for q in 0..i {
                s -= l[i * n + q] * x[q][c];
            }
            x[i][c] = s;
        }
        for i in (0..n).rev() {
            let mut s = x[i][c];
            for q in (i + 1)..n {
                s -= ut[q * n + i] * x[q][c];
            }
            x[i][c] = s / ut[i * n + i];
        }
    }
    Ok(x)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::linalg::lsq::solve_square;

    /// The nonzero basis values `N_{s−p..=s}(t)` on a raw knot slice
    /// (`knots[s] ≤ t < knots[s+1]`, the last span closed), weighted and
    /// divided by their weighted sum: a rational collocation row. Raw
    /// knots so an unclamped vector can be built too.
    fn collocation_row(knots: &[f64], p: usize, weights: &[f64], t: f64) -> (usize, Vec<f64>) {
        let last = knots.len() - p - 2;
        let s = (p..=last).rev().find(|&s| knots[s] <= t).unwrap_or(p);
        let mut n = vec![1.0f64];
        for d in 1..=p {
            let mut next = vec![0.0f64; d + 1];
            for (r, v) in n.iter().enumerate() {
                let j = s - (d - 1) + r;
                let (lo, hi) = (knots[j], knots[j + d]);
                let a = (t - lo) / (hi - lo);
                next[r] += (1.0 - a) * v;
                next[r + 1] += a * v;
            }
            n = next;
        }
        let first = s - p;
        let den: f64 = n.iter().zip(&weights[first..]).map(|(v, w)| v * w).sum();
        let row = n
            .iter()
            .zip(&weights[first..])
            .map(|(v, w)| v * w / den)
            .collect();
        (first, row)
    }

    type Band = (Vec<usize>, Vec<Vec<f64>>);

    fn band(knots: &[f64], p: usize, weights: &[f64], params: &[f64]) -> Band {
        params
            .iter()
            .map(|t| collocation_row(knots, p, weights, *t))
            .unzip()
    }

    /// Eq. 9.8's averaged clamped knots for `params`.
    fn averaged(params: &[f64], p: usize) -> Vec<f64> {
        let n = params.len();
        let mut k = vec![0.0f64; p + 1];
        for j in 1..(n - p) {
            let s: f64 = params[j..j + p].iter().sum();
            k.push(s / p as f64);
        }
        k.extend(core::iter::repeat_n(1.0, p + 1));
        k
    }

    fn uniform(n: usize) -> Vec<f64> {
        (0..n).map(|i| i as f64 / (n - 1) as f64).collect()
    }

    fn cubic(n: usize) -> Band {
        let params = uniform(n);
        band(&averaged(&params, 3), 3, &vec![1.0; n], &params)
    }

    /// The same matrix with every nonzero negated: negative pivots, so
    /// the `L` zeros outside a window are `-0.0`. Zeros stay `+0.0`.
    fn negated((first, rows): Band) -> Band {
        let rows = rows
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|v| if v == 0.0 { v } else { -v })
                    .collect()
            })
            .collect();
        (first, rows)
    }

    fn dense_of(first: &[usize], rows: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = rows.len();
        first
            .iter()
            .zip(rows)
            .map(|(f, r)| {
                let mut d = vec![0.0f64; n];
                d[*f..*f + r.len()].copy_from_slice(r);
                d
            })
            .collect()
    }

    /// Every bit, NaN payloads and signs of zero included.
    fn differing_bits(a: &[Vec<f64>], b: &[Vec<f64>]) -> usize {
        a.iter()
            .flatten()
            .zip(b.iter().flatten())
            .filter(|(u, v)| u.to_bits() != v.to_bits())
            .count()
    }

    fn assert_identity(label: &str, (first, rows): &Band, b: &[Vec<f64>]) {
        let dense = solve_square(&dense_of(first, rows), b);
        let banded = factor(first, rows).and_then(|lu| lu.solve(b));
        match (&dense, &banded) {
            (Ok(d), Ok(x)) => assert_eq!(
                differing_bits(d, x),
                0,
                "{label} (n = {}): the banded solution differs in bits from the dense one\n\
                 dense  {d:?}\nbanded {x:?}",
                rows.len()
            ),
            _ => assert_eq!(
                format!("{dense:?}"),
                format!("{banded:?}"),
                "{label}: the solves refuse differently"
            ),
        }
    }

    /// Distinct NaN payloads, so a summary that applies the wrong
    /// non-finite value, or applies it in the wrong place, shows.
    const NAN_A: f64 = f64::from_bits(0x7ff8_0000_0000_00aa);
    const NAN_B: f64 = f64::from_bits(0xfff8_0000_0000_00bb);

    /// Columns that exercise every sign of zero and both poisons: a
    /// smooth column, zeros of mixed sign, a sparse column, and a column
    /// carrying an infinity and two NaN payloads.
    fn columns(n: usize) -> Vec<Vec<f64>> {
        (0..n)
            .map(|i| {
                let t = i as f64;
                let z = if (i * 7) % 5 < 2 { -0.0 } else { 0.0 };
                let sparse = if i % 9 == 4 { -(t.cos()) } else { z };
                let poisoned = match i % 11 {
                    3 => f64::INFINITY,
                    5 => NAN_A,
                    8 => NAN_B,
                    _ => z,
                };
                vec![(0.7 * t).sin() * 3.0 - 1.0, z, sparse, poisoned]
            })
            .collect()
    }

    #[test]
    fn the_cached_reference_is_solve_square() {
        for n in [5usize, 40, 97] {
            let m = cubic(n);
            let b = columns(n);
            let dense = solve_square(&dense_of(&m.0, &m.1), &b).unwrap();
            let cached = dense_reference(&m.0, &m.1, &b).unwrap();
            assert_eq!(differing_bits(&dense, &cached), 0, "n = {n}");
            let neg = negated(m);
            let dense = solve_square(&dense_of(&neg.0, &neg.1), &b).unwrap();
            let cached = dense_reference(&neg.0, &neg.1, &b).unwrap();
            assert_eq!(differing_bits(&dense, &cached), 0, "negated, n = {n}");
        }
    }

    #[test]
    fn the_banded_lu_reproduces_the_dense_solve_bit_for_bit() {
        for &n in &[4usize, 23, 120, 301] {
            let params: Vec<f64> = uniform(n)
                .iter()
                .map(|u| u + 0.04 * (9.0 * u).sin() * u * (1.0 - u))
                .collect();
            let b = columns(n);
            for p in [3usize, 5] {
                if n <= p {
                    continue;
                }
                let ones = vec![1.0; n];
                let clamped = band(&averaged(&params, p), p, &ones, &params);
                assert_identity("clamped", &clamped, &b);
                assert_identity("clamped, negated", &negated(clamped), &b);
                // Rational: positive weights scale rows and columns,
                // which keeps total positivity and the band.
                let w: Vec<f64> = (0..n).map(|i| 1.0 + 0.6 * (i as f64 * 1.3).sin()).collect();
                assert_identity("rational", &band(&averaged(&params, p), p, &w, &params), &b);
            }
            // Unclamped: uniform knots extending past both ends,
            // parameters on the knot-free interior so the matrix stays
            // square.
            let p = 3;
            let knots: Vec<f64> = (0..n + p + 1).map(|i| i as f64).collect();
            let interior: Vec<f64> = (0..n)
                .map(|i| p as f64 + (i as f64 + 0.5) * (n - p) as f64 / n as f64)
                .collect();
            assert_identity("unclamped", &band(&knots, p, &vec![1.0; n], &interior), &b);
        }
    }

    /// The forward summary's witness: `y_0 = −0.0` stands outside the
    /// last row's window, and the dense forward sum subtracts
    /// `+0.0 · −0.0` from the last row's `−0.0`, turning it `+0.0`; the
    /// window's own terms are all `+0.0` and cannot.
    #[test]
    fn a_dropped_negative_zero_still_flips_the_sum_it_would_have_met() {
        let n = 12;
        let m = cubic(n);
        let mut b = vec![vec![0.0]; n];
        b[0][0] = -0.0;
        b[n - 1][0] = -0.0;
        assert_identity("positive pivots", &m, &b);
        let dense = solve_square(&dense_of(&m.0, &m.1), &b).unwrap();
        assert!(
            dense[n - 1][0].is_sign_positive() && dense[0][0].is_sign_negative(),
            "the fixture no longer exercises the flip: {dense:?}"
        );
    }

    /// A lower-bidiagonal matrix, `first[i] = i − 1`: each window ends
    /// at its own diagonal, so `U` is the diagonal alone, every row
    /// above a window is dropped, and every `x` past the diagonal is
    /// dropped from the back substitution. `pivot` sets the diagonal's
    /// sign. Outside what a collocation builds, which is the point: it
    /// reaches the summaries' order and signs where collocation rows
    /// leave them hidden.
    fn bidiagonal(n: usize, pivot: f64) -> Band {
        let first = (0..n).map(|i| i.saturating_sub(1)).collect();
        let mut rows = vec![vec![pivot, 0.0]];
        rows.extend((1..n).map(|_| vec![1.0, pivot]));
        (first, rows)
    }

    /// Every pattern of signed zeros on `n` rows, one per column.
    fn zero_patterns(n: usize) -> Vec<Vec<f64>> {
        (0..n)
            .map(|i| {
                (0..1usize << n)
                    .map(|c| if c >> i & 1 == 1 { -0.0 } else { 0.0 })
                    .collect()
            })
            .collect()
    }

    /// The dropped `L` zero is `+0.0 / U[q][q]`, carrying its pivot's
    /// sign; with negative pivots it is `-0.0`, and which signed-zero
    /// patterns flip depends on it.
    #[test]
    fn a_dropped_zero_carries_its_pivots_sign() {
        for pivot in [1.0, -1.0] {
            assert_identity(
                &format!("pivot {pivot}"),
                &bidiagonal(6, pivot),
                &zero_patterns(6),
            );
        }
        let m = negated(cubic(12));
        assert_identity("negated cubic", &m, &zero_patterns(12));
    }

    /// The back summary meets the LOWEST dropped non-finite value first,
    /// as the dense sum's ascending order does: on the bidiagonal, `x_3`
    /// drops `x_4 = NAN_A` before `x_6 = NAN_B` and must carry `NAN_A`.
    #[test]
    fn the_back_summary_takes_the_lowest_dropped_poison() {
        let m = bidiagonal(7, 1.0);
        let mut b: Vec<Vec<f64>> = (0..7).map(|i| vec![i as f64 + 0.5]).collect();
        b[4][0] = NAN_A;
        b[6][0] = NAN_B;
        assert_identity("two payloads past the window", &m, &b);
        let x = factor(&m.0, &m.1).unwrap().solve(&b).unwrap();
        assert_eq!(x[3][0].to_bits(), NAN_A.to_bits(), "x_3 = {:?}", x[3][0]);
    }

    /// Both poisons, by payload. Forward: row 1's window starts at 1,
    /// so `y_0 = ∞` reaches it only as a dropped `0 · ∞`, which the
    /// dense sum turns into NaN; later rows meet `NAN_A` dropped and
    /// then `NAN_B`, and keep the first. Back: the last rows' NaN
    /// payloads stand past earlier rows' windows, and the dense sum
    /// meets the window's terms first and the lowest dropped row next.
    #[test]
    fn a_non_finite_right_hand_side_poisons_what_and_where_the_dense_solve_does() {
        let n = 14;
        let m = cubic(n);
        assert!(
            m.0[1] >= 1 || m.0[2] >= 1,
            "the fixture's windows stopped moving"
        );
        let mut b: Vec<Vec<f64>> = (0..n).map(|i| vec![i as f64 + 0.5; 3]).collect();
        b[0][0] = f64::INFINITY;
        b[0][1] = NAN_A;
        b[1][1] = NAN_B;
        b[n - 1][2] = NAN_A;
        b[n - 2][2] = NAN_B;
        assert_identity("poisoned", &m, &b);
        let dense = solve_square(&dense_of(&m.0, &m.1), &b).unwrap();
        assert!(
            dense.iter().all(|r| r[0].is_nan()),
            "∞ must poison every row: {dense:?}"
        );
    }

    /// Counterexample search over the matrices the fit can build —
    /// random ascending parameters, degrees 1 to 5, clamped averaged
    /// knots — and their negations, against right-hand sides drawn
    /// from a pool of the values that move bits: signed zeros, extreme
    /// magnitudes, infinities, NaN payloads.
    #[test]
    fn fuzz_the_banded_lu_against_the_dense_solve() {
        const POOL: [f64; 12] = [
            0.0,
            -0.0,
            1e-310,
            -1e300,
            f64::MAX,
            f64::INFINITY,
            f64::NEG_INFINITY,
            NAN_A,
            NAN_B,
            1.0,
            -3.5,
            0.25,
        ];
        let mut rng = test_utils::fuzz::start("banded::fuzz_the_banded_lu_against_the_dense_solve");
        for case in 0..test_utils::fuzz::scaled(400) {
            let p = 1 + rng.below(5);
            let n = p + 1 + rng.below(30);
            let mut params: Vec<f64> = (0..n).map(|_| rng.unit()).collect();
            params.sort_by(f64::total_cmp);
            params[0] = 0.0;
            params[n - 1] = 1.0;
            if params.windows(2).any(|w| w[0] >= w[1]) {
                continue;
            }
            let m = band(&averaged(&params, p), p, &vec![1.0; n], &params);
            let b: Vec<Vec<f64>> = (0..n)
                .map(|_| {
                    (0..3)
                        .map(|_| {
                            if rng.below(3) == 0 {
                                POOL[rng.below(POOL.len())]
                            } else {
                                rng.range(-2.0, 2.0)
                            }
                        })
                        .collect()
                })
                .collect();
            let label = format!(
                "case {case}, p = {p}, n = {n}; {}",
                test_utils::fuzz::replay()
            );
            assert_identity(&label, &m, &b);
            assert_identity(&format!("negated {label}"), &negated(m), &b);
        }
    }

    #[test]
    fn a_band_that_breaks_its_shape_refuses_typed() {
        let rows = vec![vec![1.0, 0.5]; 4];
        let refusal = |first: &[usize]| factor(first, &rows).unwrap_err();
        assert_eq!(
            refusal(&[0, 1, 0, 2]),
            LsqError::BandShape { row: 2 },
            "a window moving left"
        );
        assert_eq!(
            refusal(&[0, 1, 2, 3]),
            LsqError::BandShape { row: 3 },
            "past the last column"
        );
        assert_eq!(
            refusal(&[0, 1, 2]),
            LsqError::BandShape { row: 3 },
            "one window short"
        );
        assert_eq!(
            refusal(&[0, usize::MAX, usize::MAX, usize::MAX]),
            LsqError::BandShape { row: 1 },
            "a window whose end overflows"
        );
    }

    /// A window that leaves out its diagonal is singular, and refuses
    /// where, and as, the dense elimination does.
    #[test]
    fn a_window_off_its_diagonal_refuses_as_the_dense_solve_does() {
        let b = vec![vec![1.0]; 4];
        for first in [[0usize, 0, 0, 2], [0, 2, 2, 2]] {
            let m = (first.to_vec(), vec![vec![1.0, 0.5]; 4]);
            assert_identity(&format!("first = {first:?}"), &m, &b);
            assert!(factor(&m.0, &m.1).is_err(), "first = {first:?} must refuse");
        }
        let m = (vec![0usize, 0, 1], vec![vec![1.0, 1.0]; 3]);
        assert_identity("a numeric zero pivot", &m, &b[..3]);
    }

    /// A pivot too small to divide by refuses at that pivot, where the
    /// dense elimination would carry an infinite `L` on into NaN.
    #[test]
    fn a_non_finite_factor_refuses_at_the_pivot_it_came_from() {
        let tiny = f64::from_bits(1);
        let rows = vec![vec![tiny, 1.0], vec![1.0, 1.0]];
        assert_eq!(
            factor(&[0, 0], &rows).unwrap_err(),
            LsqError::LsqDegenerate {
                pivot_index: 0,
                pivot: tiny
            }
        );
    }

    #[test]
    fn one_factorisation_serves_every_right_hand_side() {
        let n = 40;
        let m = cubic(n);
        let lu = factor(&m.0, &m.1).unwrap();
        let b1: Vec<Vec<f64>> = (0..n).map(|i| vec![(i as f64).sqrt()]).collect();
        let b2: Vec<Vec<f64>> = (0..n).map(|i| vec![1.0 / (1.0 + i as f64), -2.0]).collect();
        let join = |u: &[Vec<f64>], v: &[Vec<f64>]| -> Vec<Vec<f64>> {
            u.iter()
                .zip(v)
                .map(|(a, b)| [&a[..], &b[..]].concat())
                .collect()
        };
        let together = lu.solve(&join(&b1, &b2)).unwrap();
        let apart = join(&lu.solve(&b1).unwrap(), &lu.solve(&b2).unwrap());
        assert_eq!(
            differing_bits(&together, &apart),
            0,
            "columns apart differ from together"
        );
        assert_eq!(
            lu.solve(&b1[..3]).unwrap_err(),
            LsqError::RhsShapeMismatch {
                rhs_rows: 3,
                matrix_rows: n
            }
        );
        assert_eq!(lu.solve(&vec![vec![]; n]).unwrap_err(), LsqError::Empty);
    }
}

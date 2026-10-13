//! **Exact dyadic Bézier pieces of a span**, at any scalar: the span's
//! Bernstein row read off its B-spline coefficients by the blossom,
//! and each piece's row one exact-weight combination of it.
//!
//! A span's polynomial piece is cut into `S` equal pieces, `S` a power
//! of two. Piece `i`'s coefficient `k` is the row's blossom at
//! `((i/S)^{q−k}, ((i+1)/S)^k)`, which de Casteljau's steps at the
//! dyadic breaks write as `Σⱼ w[i][k][j]·rowⱼ` with every weight an
//! exact `f64` ([`pieces_exact`]). So a piece's coefficient is the
//! span row's `q` blossom steps plus one exact-weight combination from
//! the stored net, at any piece index, and at the interval scalar it
//! encloses the exact coefficient of the exact piece: no refined knot
//! vector is rounded on the way.

use core::ops::{Add, Mul, Sub};

use crate::real::Real;

/// `C(n, k)` as an `f64`, formed in integers: exact while it is at most
/// `2⁵³`, which every `n ≤ 56` meets (`C(56, 28) < 2⁵³ < C(57, 28)`).
/// `0` for `k > n`.
#[must_use]
pub fn binomial(n: usize, k: usize) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let exact = (0..k).fold(1_u128, |acc, j| acc * (n - j) as u128 / (j + 1) as u128);
    #[allow(clippy::cast_precision_loss)]
    let exact = exact as f64;
    exact
}

/// The piece count for a degree-`q` Bernstein row under the schedule
/// `splits`: the largest power of two at most `splits` that
/// [`pieces_exact`] admits. Never zero: one piece (the span's own row)
/// is exact at every degree.
#[must_use]
pub fn piece_count(q: usize, splits: usize) -> usize {
    core::iter::successors(Some(splits), |s| (*s > 1).then_some(s / 2))
        .find(|s| pieces_exact(q, *s))
        .unwrap_or(1)
}

/// Whether [`piece_weights`] of degree `q` at `splits` pieces, and the
/// sums of their columns, are exact `f64`s. With `splits` a power of
/// two `S`, every value de Casteljau's recurrence forms at the dyadic
/// breaks `k/S` over `q` levels is an integer over `S^q` no larger than
/// `1`, and a column of `q + 1` of them sums to at most `q + 1`; both
/// are exact while `(q + 1)·S^q ≤ 2⁵³`: to degree 9 at 32 pieces, 12 at
/// 16, 16 at 8, 24 at 4 and 47 at 2 (`tests::pieces_exact_reaches_its_stated_degrees`).
#[must_use]
pub fn pieces_exact(q: usize, splits: usize) -> bool {
    let (Ok(s), Ok(q32), Ok(n)) = (
        u64::try_from(splits),
        u32::try_from(q),
        u64::try_from(q + 1),
    ) else {
        return false;
    };
    s.is_power_of_two()
        && s.checked_pow(q32)
            .and_then(|v| v.checked_mul(n))
            .is_some_and(|v| v <= 1 << 53)
}

/// The piece-restriction weights of a degree-`q` Bernstein row on
/// `[0, 1]`, for `splits` equal pieces `S`: piece `i`'s coefficient `k`
/// is `Σⱼ w[i][k][j]·cⱼ`, the row's blossom at
/// `((i/S)^{q−k}, ((i+1)/S)^k)`, laid out flat as `[piece][k][j]`. Each
/// `w[i][k]` is a convex combination (de Casteljau's steps at ratios in
/// `[0, 1]`), every entry exact ([`pieces_exact`]); `None` where they
/// would not be.
#[must_use]
pub fn piece_weights(q: usize, splits: usize) -> Option<Vec<f64>> {
    if !pieces_exact(q, splits) {
        return None;
    }
    let n = q + 1;
    let mut out = Vec::with_capacity(splits * n.pow(2));
    let mut d = vec![0.0; n];
    #[allow(clippy::cast_precision_loss)]
    for piece in 0..splits {
        let a = piece as f64 / splits as f64;
        let b = (piece + 1) as f64 / splits as f64;
        for k in 0..n {
            for j in 0..n {
                d.iter_mut()
                    .enumerate()
                    .for_each(|(i, x)| *x = if i == j { 1.0 } else { 0.0 });
                for r in 1..=q {
                    let t = if r <= q - k { a } else { b };
                    for i in 0..=(q - r) {
                        d[i] = d[i] * (1.0 - t) + d[i + 1] * t;
                    }
                }
                out.push(d[0]);
            }
        }
    }
    Some(out)
}

/// The blossom of one span's polynomial piece at the arguments
/// `arg(1), …, arg(n)`: de Boor's recurrence on the span's `n + 1`
/// active coefficients `active`, argument `arg(r)` at level `r`, every
/// step a two-point combination at the ratio
/// `(arg − τ_j)/(τ_{j+n+1−r} − τ_j)` formed at `T` over the held knots
/// `τ_j = knots[first + j]`. For a degree-`p` spline on span `s`,
/// `first` is `s − p`; for its derivative, whose knots are the
/// spline's shifted by one, it is `s − p + 1`. The recurrence
/// [`super::CoeffWindow::blossom`] runs on a certification row, at any
/// scalar and on a vector or a scalar row. `None` on a knot index past
/// the vector.
pub fn blossom<T: Real, V: Copy + Add<Output = V> + Sub<Output = V> + Mul<T, Output = V>>(
    active: &[V],
    knots: &[T],
    first: usize,
    arg: impl Fn(usize) -> T,
) -> Option<V> {
    let n = active.len().checked_sub(1)?;
    let mut d = active.to_vec();
    for r in 1..=n {
        let t = arg(r);
        for jj in (r..=n).rev() {
            let j = first + jj;
            let (lo, hi) = (*knots.get(j)?, *knots.get(j + n + 1 - r)?);
            let ratio = (t - lo) / (hi - lo);
            d[jj] = d[jj - 1] + (d[jj] - d[jj - 1]) * ratio;
        }
    }
    d.last().copied()
}

/// The span's Bernstein row of degree `active.len() − 1` on
/// `[u0, u1]`: entry `k` is the [`blossom`] at `(u0^{n−k}, u1^k)`.
pub fn span_row<T: Real, V: Copy + Add<Output = V> + Sub<Output = V> + Mul<T, Output = V>>(
    active: &[V],
    knots: &[T],
    first: usize,
    (u0, u1): (T, T),
) -> Option<Vec<V>> {
    let n = active.len().checked_sub(1)?;
    (0..=n)
        .map(|k| blossom(active, knots, first, |r| if r <= n - k { u0 } else { u1 }))
        .collect()
}

/// `Σⱼ wⱼ·rowⱼ` over the nonzero exact weights `w`, in ascending `j`;
/// `None` when every weight is zero.
pub fn combine<T: Real, V: Copy + Add<Output = V> + Mul<T, Output = V>>(
    w: &[f64],
    row: &[V],
) -> Option<V> {
    w.iter()
        .zip(row)
        .filter(|(wj, _)| **wj != 0.0)
        .map(|(wj, rj)| *rj * T::from_f64(*wj))
        .reduce(|a, b| a + b)
}

/// One piece's Bernstein row: `piece` is that piece's `[k][j]` block of
/// [`piece_weights`] for the degree `row.len() − 1`, and entry `k` is
/// [`combine`]`(piece[k], row)`.
pub fn restrict<T: Real, V: Copy + Add<Output = V> + Mul<T, Output = V>>(
    piece: &[f64],
    row: &[V],
) -> Option<Vec<V>> {
    piece
        .chunks_exact(row.len())
        .map(|w| combine(w, row))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{binomial, pieces_exact};

    /// The degrees [`pieces_exact`]'s docs state, at every count
    /// [`super::piece_count`] falls back to.
    #[test]
    fn pieces_exact_reaches_its_stated_degrees() {
        for (splits, top) in [(32, 9), (16, 12), (8, 16), (4, 24), (2, 47)] {
            assert!(pieces_exact(top, splits), "degree {top} at {splits} pieces");
            assert!(
                !pieces_exact(top + 1, splits),
                "degree {} at {splits} pieces",
                top + 1
            );
        }
        assert!(
            !pieces_exact(2, 24),
            "a split count that is no power of two"
        );
    }

    /// `binomial` is Pascal's triangle in integers, bit for bit, to the
    /// row its docs state, and the first row past it is not an `f64`
    /// integer any more.
    #[test]
    fn binomial_is_pascal_to_56() {
        let mut row: Vec<u64> = vec![1];
        for n in 0..=56 {
            for (k, c) in row.iter().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                let exact = *c as f64;
                assert!(*c <= 1 << 53, "C({n}, {k}) past 2⁵³");
                assert!(binomial(n, k).to_bits() == exact.to_bits(), "C({n}, {k})");
            }
            row = std::iter::once(1)
                .chain(row.windows(2).map(|w| w[0] + w[1]))
                .chain(std::iter::once(1))
                .collect();
        }
        assert!(row[28] > 1 << 53, "C(57, 28) is past 2⁵³");
    }
}

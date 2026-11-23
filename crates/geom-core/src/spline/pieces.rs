//! **A span's Bernstein row**, at any scalar: read off its B-spline
//! coefficients by the blossom ([`span_row`]), with the exact binomial
//! table the Bernstein products read ([`binomial`]). A span's pieces
//! are cut from the products of these rows by de Casteljau halving
//! (`super::derivative`), not from the row here.

use core::ops::{Add, Mul, Sub};

use crate::real::Real;

/// The largest `n` whose every binomial [`binomial`] forms is an exact
/// `f64`: `C(56, 28) < 2⁵³`, and `C(57, 25)` is the first that is not
/// representable. The binomial tables' exactness caps read it
/// (`super::compose`'s rows, `super::derivative`'s degree ceiling).
pub const BINOMIAL_EXACT_MAX: usize = 56;

/// `C(n, k)` as an `f64`, formed in integers: exact for every
/// `n ≤` [`BINOMIAL_EXACT_MAX`]. `0` for `k > n`.
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

#[cfg(test)]
mod tests {
    use super::binomial;

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

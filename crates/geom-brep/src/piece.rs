//! **A function read over a piece of its parameter**, from its jet at
//! the piece's middle: the one home of the Taylor bound the root walks
//! read ([`Taylor`]), and of the bisection that places one sign change
//! inside a piece ([`sign_change`]).

use geom_core::{Real, Rounded};

/// The bisection steps that close on a sign change in its piece: enough
/// to reach the scalar's resolution from a sixteenth of a turn, or from
/// any piece of a unit span.
pub(crate) const BISECTIONS: u32 = 64;

/// **The bracket about `g`'s one sign change on `(l, r)`**, halved
/// [`BISECTIONS`] times: to two adjacent scalars.
pub(crate) fn sign_change<T: Real>(g: &impl Fn(T) -> T, l: T, r: T) -> (T, T) {
    let two = T::from_f64(2.0);
    let (mut lo, mut hi) = (l, r);
    let at_lo = g(l);
    for _ in 0..BISECTIONS {
        let c = (lo + hi) / two;
        let product = g(c) * at_lo;
        lo = product.select_le_zero(lo, c);
        hi = product.select_le_zero(c, hi);
    }
    (lo, hi)
}

/// **Bounds on `f`, `f′` and `f″` over the piece of half-width `w`**
/// about a middle where `f` and its first `N − 1` derivatives read
/// `jet`, each within its bound, and `|f⁽ᴺ⁾|` is at most `top`
/// throughout: Taylor's theorem about the middle, every term read
/// through its magnitude, so each bound holds for the exact `f`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Taylor<T> {
    /// A lower bound on `|f|` over the piece (negative where the bound
    /// reaches zero).
    pub(crate) least: T,
    /// An upper bound on `|f|` over the piece.
    pub(crate) most: T,
    /// A lower bound on `|f′|` over the piece (negative where the bound
    /// reaches zero).
    pub(crate) least_slope: T,
    /// An upper bound on `|f′|` over the piece.
    pub(crate) most_slope: T,
    /// An upper bound on `|f″|` over the piece.
    pub(crate) most_bend: T,
    /// An upper bound on how far `f″` moves from its middle's value over
    /// the piece.
    pub(crate) bend_drift: T,
}

/// `c·wᵏ/k!`, the `k`-th Taylor term of a coefficient bound `c`, for
/// `k` up to five (the longest jet [`Taylor::over`] takes, and its
/// remainder).
fn term<T: Real>(c: T, w: T, k: usize) -> T {
    let factorial = match k {
        0 | 1 => return if k == 0 { c } else { c * w },
        2 => T::from_f64(2.0),
        3 => T::from_f64(6.0),
        4 => T::from_f64(24.0),
        _ => T::from_f64(120.0),
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let power = w.powi(k as i32);
    c * power / factorial
}

impl<T: Real> Taylor<T> {
    /// [`Taylor`] of the jet `jet` (`N` from 3 to 5 terms) and the bound
    /// `top` on the next derivative, over half-width `w`.
    pub(crate) fn over<const N: usize>(jet: [Rounded<T>; N], top: T, w: T) -> Self {
        const { assert!(N >= 3 && N <= 5, "a jet of three to five terms") };
        let magnitude: [T; N] = core::array::from_fn(|k| jet[k].magnitude());
        // The bound on `|f⁽ʲ⁾|` over the piece: its terms from `j` on.
        let swing = |j: usize, from: usize| {
            let tail = (from..N).fold(None, |sum: Option<T>, k| {
                let t = term(magnitude[k], w, k - j);
                Some(sum.map_or(t, |s| s + t))
            });
            let last = term(top, w, N - j);
            tail.map_or(last, |s| s + last)
        };
        let fall = swing(0, 1);
        let turn = swing(1, 2);
        let drift = swing(2, 3);
        Self {
            least: jet[0].room() - fall,
            most: magnitude[0] + fall,
            least_slope: jet[1].room() - turn,
            most_slope: magnitude[1] + turn,
            most_bend: magnitude[2] + drift,
            bend_drift: drift,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// **Each bound holds the exact function over the piece, and each
    /// charges the jet's bounds.** `f = sin` about `m = 0.4`, `w = 0.3`,
    /// its jet read exactly but for a bound of `10⁻³` on each term, `top
    /// = 1`: sampled densely, `|f|`, `|f′|` and `|f″|` stay inside the
    /// bounds, and each bound is within its terms' sum of the samples'
    /// extremes. Read on the values alone, each bound moves by its
    /// bound's share: the least ones by `10⁻³` and `10⁻³·(1 + w)`, the
    /// most ones likewise.
    #[test]
    fn the_bounds_hold_the_function_and_charge_the_jet() {
        let (m, w, e) = (0.4_f64, 0.3_f64, 1e-3);
        let (s, c) = m.sin_cos();
        let jet = [s, c, -s, -c].map(|value| Rounded { value, error: e });
        let b = Taylor::over(jet, 1.0, w);
        let mut extremes = [f64::INFINITY, 0.0, f64::INFINITY, 0.0, 0.0];
        for i in 0..=1000 {
            let t = m - w + 2.0 * w * f64::from(i) / 1000.0;
            let (fs, fc) = t.sin_cos();
            extremes[0] = extremes[0].min(fs.abs());
            extremes[1] = extremes[1].max(fs.abs());
            extremes[2] = extremes[2].min(fc.abs());
            extremes[3] = extremes[3].max(fc.abs());
            extremes[4] = extremes[4].max(fs.abs());
        }
        let [f_lo, f_hi, d_lo, d_hi, bend_hi] = extremes;
        assert!(
            b.least <= f_lo && f_lo - b.least < 0.2,
            "least {b:?} against {f_lo}"
        );
        assert!(
            b.most >= f_hi && b.most - f_hi < 0.2,
            "most {b:?} against {f_hi}"
        );
        assert!(
            b.least_slope <= d_lo && d_lo - b.least_slope < 0.2,
            "least slope {b:?}"
        );
        assert!(
            b.most_slope >= d_hi && b.most_slope - d_hi < 0.2,
            "most slope {b:?}"
        );
        assert!(
            b.most_bend >= bend_hi && b.most_bend - bend_hi < 0.2,
            "most bend {b:?}"
        );
        let bare = Taylor::over(jet.map(|r| Rounded::exact(r.value)), 1.0, w);
        let fall = e * (w + w.powi(2) / 2.0 + w.powi(3) / 6.0);
        let turn = e * (w + w.powi(2) / 2.0);
        let close = |a: f64, b: f64, what: &str| {
            assert!((a - b).abs() < 1e-12, "{what}: {a} against {b}");
        };
        close(
            bare.least - b.least,
            e + fall,
            "the least value charges f's and the fall's bounds",
        );
        close(
            b.most - bare.most,
            e + fall,
            "the most value charges them too",
        );
        close(
            bare.least_slope - b.least_slope,
            e + turn,
            "the least slope charges f′'s and the turn's",
        );
        close(
            b.most_slope - bare.most_slope,
            e + turn,
            "the most slope too",
        );
        close(
            b.most_bend - bare.most_bend,
            e * (1.0 + w),
            "the most bend charges f″'s and f‴'s",
        );
        close(
            b.bend_drift - bare.bend_drift,
            e * w,
            "the bend's drift charges f‴'s",
        );
    }

    /// **The sign change is closed to adjacent scalars.** `g = t − 0.3`
    /// on `(0, 1)`: the bracket holds `0.3` and is at most one ulp wide.
    #[test]
    fn the_sign_change_closes_to_adjacent_scalars() {
        let (lo, hi) = sign_change(&|t: f64| t - 0.3, 0.0, 1.0);
        assert!(lo <= 0.3 && 0.3 <= hi, "({lo}, {hi}) holds the root");
        assert!(hi - lo <= f64::EPSILON, "({lo}, {hi}) is closed");
    }
}

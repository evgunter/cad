//! **The middle of a bracket**, in its two forms. A bracket here is any
//! pair of ends whose middle is read: a parameter interval, a bisection's
//! piece, one coordinate of a box, the enclosure an `Interval` carries.
//! The two forms are the same real number and differ in rounding and at
//! the extremes, and each has a reason, stated at it. A hand spelling of
//! either is refused by `scripts/gates/bracket-mid-by-hand.sh`; a point
//! pair's middle is [`Point3::mid`](crate::Point3::mid) (and `Point2`'s),
//! which reads [`bracket_mid`] per coordinate.
//!
//! Where `hi − lo` is exact (Sterbenz: ends of one sign within a factor
//! of two, so every late step of a bisection) the two forms give the
//! same bits at `f64`; otherwise they can differ in the last bit.

use crate::Real;

/// The middle of the bracket `[t0, t1]`, spelled `(t0 + t1)·½`.
///
/// Each end enters once, so at `T = Interval` the enclosure is as tight
/// as the ends allow (`t0 + (t1 − t0)·½` reads `t0` twice and widens by
/// its width). At `f64` the result lies in `[t0, t1]`; it overflows
/// when `t0 + t1` does, two same-sign ends past `f64::MAX / 2`. At an
/// infinite point `(∞, ∞)` it is that infinity, where [`bracket_mid`]
/// is NaN.
pub fn mid_param<T: Real>(t0: T, t1: T) -> T {
    (t0 + t1) * T::from_f64(0.5)
}

/// The middle of the bracket `(lo, hi)`, spelled `lo + ½(hi − lo)`.
///
/// **The form is load-bearing at an infinite point** `(∞, ∞)`, where it
/// is `∞ + ½(∞ − ∞)` = NaN and [`mid_param`] is `∞`: projection reads
/// a scalar's bracket through it (`geom`'s `projection_policy::mid`), so
/// an overflowed residual reaches the acceptance tests as NaN, loses
/// every comparison and refuses typed rather than reporting a foot at
/// infinite distance. `geom/tests/curves/projection.rs` pins both, this
/// door by name and the exit through the public projection door.
///
/// It is the identity on a finite point, but for `−0.0`, which gives
/// `+0.0`. On a bracket with an infinite end it is not finite: NaN where
/// `hi − lo` is (both ends infinite on one side, or `(−∞, +∞)`), and that
/// infinite end otherwise (`(1, +∞)` gives `+∞`). It overflows when the
/// width does (`(−1.5e308, 1.5e308)` gives an infinity), where
/// [`mid_param`] does not; and does not where two same-sign ends sum
/// past `f64::MAX`, where [`mid_param`] does. A caller for whom a
/// non-finite middle is not a refusal reads the bracket first.
#[must_use]
pub fn bracket_mid<T: Real>((lo, hi): (T, T)) -> T {
    lo + (hi - lo) * T::from_f64(0.5)
}

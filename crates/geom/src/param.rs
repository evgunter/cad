//! Parameter-interval helpers shared by every carrier kind.

use geom_core::Real;

/// The middle of the parameter interval `[t0, t1]`, spelled
/// `(t0 + t1)·½`.
///
/// Each end enters once, so at `T = Interval` the enclosure is as tight
/// as the ends allow (`t0 + (t1 − t0)·½` reads `t0` twice and widens by
/// its width). At `f64` the result lies in `[t0, t1]`; it overflows only
/// when `t0 + t1` does (two same-sign ends near the range limit), which
/// no curve parameter approaches.
pub fn mid_param<T: Real>(t0: T, t1: T) -> T {
    (t0 + t1) * T::from_f64(0.5)
}

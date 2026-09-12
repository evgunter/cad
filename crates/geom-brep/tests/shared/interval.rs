//! Lifting exactly-known `f64` geometry onto the interval scalar.
//!
//! One function under three shapes — a coordinate, a vector, a point —
//! all of them `Interval::from_f64` applied componentwise. Every caller
//! is inside an `#[cfg(feature = "interval")]` suite or module, so the
//! module is gated the same way.
//!
//! **What this module does NOT absorb.** `revolved_point_anchor.rs`'s
//! `w(c)` widens a coordinate into `[c - half, c + half]`: that is the
//! opposite of an exact lift and is the row's subject.

use geom_core::{Interval, Point3, Real, Vec3};

/// The degenerate interval enclosing an exactly-known coordinate.
pub(crate) fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// An exactly-known vector, componentwise.
pub(crate) fn iv3(v: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(iv(v.x), iv(v.y), iv(v.z))
}

/// An exactly-known point, componentwise.
pub(crate) fn ip(p: Point3<f64>) -> Point3<Interval> {
    Point3::new(iv(p.x), iv(p.y), iv(p.z))
}

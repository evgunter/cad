//! Exactly-known coordinates as degenerate `Interval` enclosures,
//! and the three-coordinate vector of them the quadrature door takes.
//!
//! **What this module does NOT absorb.** The `Point3<T>` and `Vec3<T>`
//! constructors that several suites also spell `p`/`p3` are
//! `shared::point`: an `RVec3` is a `[Interval; 3]`, not a point,
//! and it does not go through `Real`.

use geom_brep::props::quad::RVec3;
use geom_core::Interval;

/// The degenerate enclosure of an exactly-known coordinate.
///
/// One home for what two suites spelled through two doors:
/// `Interval::point(x)` and `Interval::from_bounds(x, x)` agree
/// on every `f64` — finite `x` gives `[x, x]` from both, and every
/// non-finite one is poison from both (`from_bounds` poisons a closed
/// side at infinity, and NaN fails its `!(lo <= hi)` test) — so this is
/// the same value, not a choice between two.
pub(crate) fn pt(x: f64) -> Interval {
    Interval::point(x)
}

/// An exactly-known point as the quadrature door's `RVec3`.
pub(crate) fn p3(x: f64, y: f64, z: f64) -> RVec3 {
    [pt(x), pt(y), pt(z)]
}

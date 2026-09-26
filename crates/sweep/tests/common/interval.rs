//! **The `Interval` literals** — a scalar, a sketch point, a space
//! point and a vector from exact `f64` coordinates, read into the
//! certified lane by `Real::from_f64`, which embeds every finite
//! `f64` as the degenerate interval on it. What an interval-lane suite
//! builds its profile and its placements FROM, so it routes here as
//! section authoring (the module's routing rule).
//!
//! **At `Interval`, not generic over `Real`.** The suites call these
//! where nothing else fixes the scalar — a vertex inside a `vec!`, a
//! point handed to a closure — so a `T: Real` spelling would need a
//! turbofish at the sites it exists to shorten. The `f64` suites have
//! their own `p2` (`revolve_common`), and a suite needing both names
//! this module's as `interval::p2`.
//!
//! What this module deliberately did NOT absorb, as the whole list:
//!
//! - the same four spelled in OTHER crates' suites (`geom-brep`'s
//!   `shared::point`, which is generic, and the `topo` and `profile`
//!   copies) — another crate's suites, out of this recipe's scope.

use geom_core::{Interval, Point2, Point3, Real, Vec3};

/// An exact `f64` as the degenerate interval on it.
pub fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// A sketch-plane point from two exact `f64` coordinates.
pub fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(iv(x), iv(y))
}

/// A space point from three exact `f64` coordinates.
pub fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(iv(x), iv(y), iv(z))
}

/// A space vector from three exact `f64` components.
pub fn v3(x: f64, y: f64, z: f64) -> Vec3<Interval> {
    Vec3::new(iv(x), iv(y), iv(z))
}

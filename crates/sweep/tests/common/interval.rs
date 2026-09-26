//! **The `Interval` literals** — a scalar, a sketch point, a space
//! point and two vectors from exact `f64` coordinates, read into the
//! certified lane by `Real::from_f64`, which embeds every finite
//! `f64` as the degenerate interval on it. What an interval-lane suite
//! builds its profile and its placements FROM, so it routes here as
//! section authoring (the module's routing rule). The points and
//! vectors are the componentwise lift through `map`, the linear
//! types' one door for reading a value at another scalar.
//!
//! **At `Interval`, not generic over `Real`.** The suites call these
//! where nothing else fixes the scalar — a vertex inside a `vec!`, a
//! point handed to a closure — so a `T: Real` spelling would need a
//! turbofish at the sites it exists to shorten. The `f64` suites have
//! `f64` `p2`s of their own (`revolve_common`, `mate2_common` and
//! `cone_nappe` each hold one), and a suite needing both names this
//! module's as `interval::p2`.
//!
//! What this module deliberately did NOT absorb, as the whole list:
//!
//! - the scalar-generic lifts in this crate (`sweep::test_support`'s
//!   `corners`, and the `v` vertex helpers over `T`) — they serve
//!   every scalar, and this module serves one;
//! - the same literals spelled in OTHER crates' suites (`geom-brep`'s
//!   `shared::point`, which is generic, and the `topo` and `profile`
//!   copies) — another crate's suites, out of this recipe's scope.

use geom_core::{Interval, Point2, Point3, Real, Vec2, Vec3};

/// An exact `f64` as the degenerate interval on it.
pub fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// A sketch-plane point from two exact `f64` coordinates.
pub fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(x, y).map(iv)
}

/// A sketch-plane vector from two exact `f64` components.
pub fn v2(x: f64, y: f64) -> Vec2<Interval> {
    Vec2::new(x, y).map(iv)
}

/// A space point from three exact `f64` coordinates.
pub fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(x, y, z).map(iv)
}

/// A space vector from three exact `f64` components.
pub fn v3(x: f64, y: f64, z: f64) -> Vec3<Interval> {
    Vec3::new(x, y, z).map(iv)
}

//! A test's lever arm, read off geometry: the radius of a sphere about
//! the origin, `ExtentBall::of_sphere(origin, r).lever()`.
//!
//! A `Lever` has no door from a bare length (`geom-core`'s
//! `predicate.rs`), so a suite that probes a classifier at a chosen arm
//! names the region the arm is the extent of. The ball a sphere is
//! reads back the radius's magnitude, so for `r ≥ 0` the arm's bits are
//! `r`'s: the rows that used to pass the literal read the same value.

use geom_brep::ExtentBall;
use geom_core::{Lever, Point3, Real};

/// The lever of the sphere of radius `r` about the origin.
pub(crate) fn sphere_arm<T: Real>(r: T) -> Lever<T> {
    ExtentBall::of_sphere(Point3::origin(), r).lever()
}

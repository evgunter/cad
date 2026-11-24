//! **Nets with a collapsed side**, the fixtures the collapsed-side rows
//! of `topo` and `step-import` read.

use crate::NurbsSurface;
use geom_core::Point3;
use geom_core::spline::{KnotVector, Knots};

/// A bilinear triangle of a wall: its `u = 0` column from `pole` to
/// `top`, its `u = 1` column from (3, 0, 0) to (3, 2, 0). With `top` the
/// pole it is collapsed at `u = 0`; an ulp off, it is a regular side.
///
/// # Panics
///
/// Never: the counts and weights are a valid net's.
#[must_use]
#[allow(clippy::unwrap_used)]
pub fn triangle(pole: Point3<f64>, top: Point3<f64>) -> NurbsSurface<f64> {
    let kv = || Knots::from(KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap());
    NurbsSurface::new(
        kv(),
        kv(),
        vec![
            pole,
            top,
            Point3::new(3.0, 0.0, 0.0),
            Point3::new(3.0, 2.0, 0.0),
        ],
        vec![1.0; 4],
    )
    .unwrap()
}

/// The point one ulp along `x` from `p`.
#[must_use]
pub fn ulp_along_x(p: Point3<f64>) -> Point3<f64> {
    Point3::new(f64::from_bits(p.x.to_bits() + 1), p.y, p.z)
}

//! **`NurbsCurve3::derivative_at` is `ders1`'s derivative**, read
//! through the hodograph: at `f64` the two agree to a few ulps, and at
//! `Interval` their enclosures meet, on a rational path whose two spans
//! (two quarter circles) join at a double knot, at stations inside a
//! span, at the knot and at both ends.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real};

/// Two quarter circles of radius 2 in `z = 0.5`, rational quadratic,
/// joined at `t = 0.5`.
fn half_circle() -> NurbsCurve3<f64> {
    let r = 2.0;
    NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point3::new(r, 0.0, 0.5),
            Point3::new(r, r, 0.5),
            Point3::new(0.0, r, 0.5),
            Point3::new(-r, r, 0.5),
            Point3::new(-r, 0.0, 0.5),
        ],
        vec![1.0, FRAC_1_SQRT_2, 1.0, FRAC_1_SQRT_2, 1.0],
    )
    .unwrap()
}

#[test]
fn the_hodograph_derivative_is_ders1s_on_a_rational_path() {
    let path = half_circle();
    let lifted = path.map_scalar(Interval::from_f64);
    for at in [0.0, 0.13, 0.37, 0.5, 0.77, 1.0] {
        let (hodograph, recursion) = (path.derivative_at(at), path.ders1(at).1);
        let scale = recursion.norm();
        for (axis, h, d) in [
            ("x", hodograph.x, recursion.x),
            ("y", hodograph.y, recursion.y),
            ("z", hodograph.z, recursion.z),
        ] {
            assert!(
                (h - d).abs() <= 8.0 * f64::EPSILON * scale,
                "t = {at}, {axis}: the hodograph reads {h:e} where ders1 reads {d:e} at f64"
            );
        }
        let (hodograph, recursion) = (
            lifted.derivative_at(at),
            lifted.ders1(Interval::from_f64(at)).1,
        );
        for (axis, h, d) in [
            ("x", hodograph.x, recursion.x),
            ("y", hodograph.y, recursion.y),
            ("z", hodograph.z, recursion.z),
        ] {
            assert!(
                h.lo() <= d.hi() && d.lo() <= h.hi(),
                "t = {at}, {axis}: the enclosures [{:e}, {:e}] and [{:e}, {:e}] miss each other",
                h.lo(),
                h.hi(),
                d.lo(),
                d.hi()
            );
        }
    }
}

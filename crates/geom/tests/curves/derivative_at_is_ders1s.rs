//! **`NurbsCurve3::derivative_at` is `ders1`'s derivative**, read
//! through the hodograph: at `f64` the two agree to a few ulps, and at
//! `Interval` their enclosures meet, on a rational path whose two spans
//! (two quarter circles) join at a double knot, at stations inside a
//! span, at the knot and at both ends. The hodograph's enclosure is
//! also held no wider than `ders1`'s by more than [`WIDER_AT_MOST`], on
//! the 3-space path and on its plane twin (`NurbsCurve2`).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom::{NurbsCurve2, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point2, Point3, Real};

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

/// How much wider than `ders1`'s the hodograph's enclosure of a
/// component may be. On this path it is never wider (the widest ratio
/// measured is printed by the rows); over PR 6952's review of 600
/// random curves, rational ones came within 1.1–1.3× either way and the
/// worst component was 2.3× wider. A width guard, so an enclosure that
/// merely contains the derivative, however wide, does not pass.
const WIDER_AT_MOST: f64 = 2.5;

/// The width bound on one component: `ders1`'s width times
/// [`WIDER_AT_MOST`], plus four ulps of the derivative's scale for a
/// component `ders1` reads exactly.
fn within_width(h: Interval, d: Interval, scale: f64) -> bool {
    h.hi() - h.lo() <= WIDER_AT_MOST * (d.hi() - d.lo()) + 4.0 * f64::EPSILON * scale
}

/// The same two quarter circles in the plane.
fn half_circle_2() -> NurbsCurve2<f64> {
    let r = 2.0;
    NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point2::new(r, 0.0),
            Point2::new(r, r),
            Point2::new(0.0, r),
            Point2::new(-r, r),
            Point2::new(-r, 0.0),
        ],
        vec![1.0, FRAC_1_SQRT_2, 1.0, FRAC_1_SQRT_2, 1.0],
    )
    .unwrap()
}

#[test]
fn the_plane_hodograph_derivative_is_ders1s_and_no_wider() {
    let path = half_circle_2();
    let lifted = path.map_scalar(Interval::from_f64);
    let mut widest = 0.0_f64;
    for at in [0.0, 0.13, 0.37, 0.5, 0.77, 1.0] {
        let (h, d) = (path.derivative_at(at), path.ders1(at).1);
        let scale = d.norm();
        for (axis, h, d) in [("x", h.x, d.x), ("y", h.y, d.y)] {
            assert!(
                (h - d).abs() <= 8.0 * f64::EPSILON * scale,
                "t = {at}, {axis}: the hodograph reads {h:e} where ders1 reads {d:e} at f64"
            );
        }
        let (h, d) = (
            lifted.derivative_at(at),
            lifted.ders1(Interval::from_f64(at)).1,
        );
        for (axis, h, d) in [("x", h.x, d.x), ("y", h.y, d.y)] {
            assert!(
                h.lo() <= d.hi() && d.lo() <= h.hi() && within_width(h, d, scale),
                "t = {at}, {axis}: the hodograph's [{:e}, {:e}] against ders1's [{:e}, {:e}]",
                h.lo(),
                h.hi(),
                d.lo(),
                d.hi()
            );
            if d.hi() > d.lo() {
                widest = widest.max((h.hi() - h.lo()) / (d.hi() - d.lo()));
            }
        }
    }
    println!("plane: the hodograph's widest component is {widest:.3}× ders1's");
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
                h.lo() <= d.hi() && d.lo() <= h.hi() && within_width(h, d, scale),
                "t = {at}, {axis}: the hodograph's [{:e}, {:e}] against ders1's [{:e}, {:e}]",
                h.lo(),
                h.hi(),
                d.lo(),
                d.hi()
            );
        }
    }
}

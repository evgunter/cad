//! **The torus germ row's placements through the join**: a drum, a ball
//! and a donut against a moved donut, at the six placements of
//! `torus-germ-pairs-have-no-section-frame`, each run under ∪ in both
//! member orders.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::germ_pair::cyl;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{ball_poled_y, finished, revolved_about_y};
use topo::{AtRestBody, Body, BooleanError, BooleanResult};

/// The row's six placements: B turned by `angle` about `axis` through
/// the origin, then moved by `t`.
#[allow(clippy::approx_constant)] // the row's second angle is a hair short of a right one
fn placements() -> [(f64, Vec3<f64>, Vec3<f64>); 6] {
    [
        (0.0, Vec3::unit_x(), Vec3::new(0.45, 0.1, 0.05)),
        (1.570_796_3, Vec3::unit_x(), Vec3::new(0.3, 0.2, 0.1)),
        (0.7, Vec3::unit_x(), Vec3::new(0.1, 0.35, 0.2)),
        (
            1.1,
            Vec3::new(0.3, 1.0, 0.2).normalize(),
            Vec3::new(0.42, -0.15, 0.3),
        ),
        (0.4, Vec3::unit_y(), Vec3::new(0.6, 0.0, 0.0)),
        (
            0.9,
            Vec3::new(1.0, 1.0, 0.0).normalize(),
            Vec3::new(0.05, 0.1, 0.02),
        ),
    ]
}

fn placed(b: &Body<f64>, (angle, axis, t): (f64, Vec3<f64>, Vec3<f64>)) -> AtRestBody<f64> {
    let r = Affine3::rotation_about_axis(Point3::origin(), axis, angle);
    let moved = topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::from_parts(r.linear, r.translation + t),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .unwrap();
    finished("the moved shape", moved, Tol::witness())
}

/// The donut: `R = 0.6`, `r = 0.2`, revolved about `y`.
fn donut() -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.6, -0.2), 1.0), (Point2::new(0.6, 0.2), 1.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

/// The ball: radius `0.5` about the origin, poled on `y`.
fn ball() -> Body<f64> {
    ball_poled_y(0.5, Vec3::new(0.0, 0.0, 0.0), Tol::witness())
}

/// The drum: radius `0.5` about `z`, `z ∈ [−1, 1]`.
fn drum() -> Body<f64> {
    cyl(0.5, 1.0)
}

fn at_rest(b: &Body<f64>) -> AtRestBody<f64> {
    finished("the shape", b.clone(), Tol::witness())
}

/// A run's outcome, by variant.
fn outcome(got: &Result<BooleanResult<f64>, BooleanError>) -> String {
    match got {
        Ok(r) => format!("Ok({:?})", r.body().map(|b| b.kind)),
        Err(BooleanError::GermSectionUnbuilt { why, .. }) => format!("GermSectionUnbuilt({why:?})"),
        Err(BooleanError::ResultInvalid { errors }) => format!("ResultInvalid({errors:?})"),
        Err(e) => format!("{e:?}"),
    }
}

#[test]
fn probe_the_germ_rows_placements() {
    let shapes: [(&str, fn() -> Body<f64>); 3] = [("drum", drum), ("ball", ball), ("donut", donut)];
    for (name, a) in shapes {
        for (i, place) in placements().into_iter().enumerate() {
            let (a, b) = (at_rest(&a()), placed(&donut(), place));
            let ab = topo::union(&a, &b, Tol::witness());
            let ba = topo::union(&b, &a, Tol::witness());
            eprintln!("PROBE {name} x donut {i}: {}", outcome(&ab));
            eprintln!("PROBE donut x {name} {i}: {}", outcome(&ba));
        }
    }
}

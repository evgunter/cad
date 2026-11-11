//! **A ball against a donut, through the carried lane onto a torus
//! face.** The ball (radius 1/2, poled along `y`) and the donut
//! (`R = 0.6`, `r = 0.2`, revolved about `y`) at placements of the germ
//! row (`work/sect/torus-germ-pairs-have-no-section-frame.md`):
//!
//! - the donut turned 0.7 rad about `x` and moved to `(0.1, 0.35, 0.2)`
//!   (ball ∪ donut): a carried chord closes a ring on a torus face
//!   (`RingClosure::Chord` on the torus's ring path);
//! - the ball turned 1.1 rad about `(0.3, 1, 0.2)` and moved to
//!   `(0.42, −0.15, 0.3)` (donut ∪ ball): a spline edge in the run of a
//!   torus ring (`run_loop_arcs` on the torus);
//! - the donut turned 0.9 rad about `(1, 1, 0)` and moved to
//!   `(0.05, 0.1, 0.02)`, ∪ in both member orders: the ball cuts both
//!   tube circles of its meridian, two curves about the donut's axis
//!   that no plane or meridian tells apart; each germ's component is its
//!   site's foot on the carried section.
//!
//! Each union builds, its chorded sphere and torus faces measured
//! through the rotational lane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body, BooleanError};

fn ball() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(0.0, -0.5), 1.0),
        (Point2::new(0.0, 0.5), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
        .into_body()
}

fn donut() -> Body<f64> {
    let lp = bulge_loop(vec![
        (Point2::new(0.6, -0.2), 1.0),
        (Point2::new(0.6, 0.2), 1.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
    .into_body()
}

/// `b` turned by `angle` about `axis` through the origin, then moved by `t`.
fn placed(b: &Body<f64>, axis: Vec3<f64>, angle: f64, t: Vec3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let turned = topo::transform_rigid(
        b,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
            tol,
        ),
        tol,
    )
    .unwrap();
    let moved = topo::transform_rigid(
        &turned,
        &geom_core::RigidMotion::translation(t).unwrap(),
        tol,
    )
    .unwrap();
    finished("the placed operand", moved, tol)
}

/// Where a run ends: `"builds"` for one seamed solid, else its refusal.
fn outcome(got: &Result<topo::BooleanResult<f64>, BooleanError>) -> String {
    match got {
        Ok(r)
            if matches!(
                r.body().map(|b| b.kind),
                Some(topo::BooleanResultKind::Seamed)
            ) =>
        {
            "builds".to_owned()
        }
        Ok(r) => format!("{:?}", r.body().map(|b| b.kind)),
        Err(e) => format!("{e:?}"),
    }
}

/// **A ball against a donut at four placements builds**: each union's
/// chorded sphere faces and its torus faces measure through the
/// rotational lane, the torus ring's face and the faces of a ring's run
/// alike. No body here is held to a volume of its own
/// (`work/spiricarm/the-general-pose-torus-germ-unions-that-build-have-no-volume-truth.md`).
#[test]
fn a_ball_against_a_donut_carries_onto_the_torus_and_builds() {
    let tol = Tol::witness();
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let (ball0, donut0) = (
        placed(&ball(), Vec3::unit_x(), 0.0, origin),
        placed(&donut(), Vec3::unit_x(), 0.0, origin),
    );
    let ring = placed(&donut(), Vec3::unit_x(), 0.7, Vec3::new(0.1, 0.35, 0.2));
    let moved = placed(
        &ball(),
        Vec3::new(0.3, 1.0, 0.2),
        1.1,
        Vec3::new(0.42, -0.15, 0.3),
    );
    let about = placed(
        &donut(),
        Vec3::new(1.0, 1.0, 0.0),
        0.9,
        Vec3::new(0.05, 0.1, 0.02),
    );
    let got: Vec<(&str, String, Option<String>)> = [
        (
            "a carried chord closing a ring on a torus face",
            &ball0,
            &ring,
        ),
        ("a spline edge in a torus ring's run", &donut0, &moved),
        ("two curves about the axis, ball ∪ donut", &ball0, &about),
        ("two curves about the axis, donut ∪ ball", &about, &ball0),
    ]
    .into_iter()
    .map(|(what, x, y)| {
        let r = topo::union(x, y, tol);
        (
            what,
            outcome(&r),
            crate::germ_torus_components::union_floor(&r, x, y),
        )
    })
    .collect();
    for (what, end, floor) in &got {
        assert_eq!(end, "builds", "{what}: {got:#?}");
        assert!(floor.is_none(), "{what}: {floor:?}");
    }
}

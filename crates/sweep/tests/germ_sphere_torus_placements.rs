//! **A ball against a donut, through the carried lane onto a torus
//! face.** The ball (radius 1/2, poled along `y`) and the donut
//! (`R = 0.6`, `r = 0.2`, revolved about `y`) at two of the germ row's
//! placements (`work/sect/torus-germ-pairs-have-no-section-frame.md`),
//! each ∪ in both member orders:
//!
//! - the donut turned 0.9 rad about `(1, 1, 0)` and moved to
//!   `(0.05, 0.1, 0.02)`: the ball cuts both tube circles of its
//!   meridian, two curves about the donut's axis, each germ's site told
//!   to its own by the line through the two circles' centres in its
//!   meridian plane (`Sides::Trace`), and no germ facing a germ on the
//!   other curve (∪ in both member orders);
//! - the donut turned 0.7 rad about `x` and moved to `(0.1, 0.35, 0.2)`:
//!   a carried chord closes a ring on a torus face (`RingClosure::Chord`
//!   on the torus's ring path; ball ∪ donut).
//!
//! Every run passes the join and stops at the result gate's volume,
//! a face trimmed by spline chords on a sphere or torus chart
//! (`work/spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane.md`),
//! which turns these rows red the day it lands.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::cylinder_sphere_frame::{Door, door};
use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body};

fn ball() -> AtRestBody<f64> {
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
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    finished("the ball", ball, Tol::witness())
}

fn donut(axis: Vec3<f64>, angle: f64, t: Vec3<f64>) -> AtRestBody<f64> {
    let tol = Tol::witness();
    let lp = bulge_loop(vec![
        (Point2::new(0.6, -0.2), 1.0),
        (Point2::new(0.6, 0.2), 1.0),
    ]);
    let donut: Body<f64> = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, tol)
        .unwrap()
        .body;
    let turned = topo::transform_rigid(
        &donut,
        &geom_core::test_support::rigid(
            &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle),
            tol,
        ),
        tol,
    )
    .unwrap();
    let placed = topo::transform_rigid(
        &turned,
        &geom_core::RigidMotion::translation(t).unwrap(),
        tol,
    )
    .unwrap();
    finished("the donut", placed, tol)
}

#[test]
fn a_ball_against_a_donut_carries_onto_the_torus_and_stops_at_the_volume() {
    let ball = ball();
    for (what, donut) in [
        (
            "about the axis",
            donut(Vec3::new(1.0, 1.0, 0.0), 0.9, Vec3::new(0.05, 0.1, 0.02)),
        ),
        (
            "a torus ring",
            donut(Vec3::unit_x(), 0.7, Vec3::new(0.1, 0.35, 0.2)),
        ),
    ] {
        let orders = [
            (&ball, &donut, "ball ∪ donut"),
            (&donut, &ball, "donut ∪ ball"),
        ];
        // The two curves' facing went wrong only as donut ∪ ball; the
        // ring gate refused in either order.
        let asked = if what == "a torus ring" {
            &orders[..1]
        } else {
            &orders[..]
        };
        for &(x, y, order) in asked {
            let e = topo::union(x, y, Tol::witness())
                .err()
                .unwrap_or_else(|| panic!("{what}, {order}: the union built; re-pin it"));
            assert!(
                matches!(door(&e), Some(Door::Sphere | Door::Ring)),
                "{what}, {order}: {e:?}"
            );
        }
    }
}

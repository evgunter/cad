//! **A ball against a donut, through the carried lane onto a torus
//! face.** The ball (radius 1/2, poled along `y`) and the donut
//! (`R = 0.6`, `r = 0.2`, revolved about `y`) at two of the germ row's
//! placements (`work/sect/torus-germ-pairs-have-no-section-frame.md`):
//!
//! - the donut turned 0.7 rad about `x` and moved to `(0.1, 0.35, 0.2)`
//!   (ball ∪ donut): a carried chord closes a ring on a torus face
//!   (`RingClosure::Chord` on the torus's ring path). The union passes
//!   the join and stops at the result gate's volume, on one face trimmed
//!   by spline chords on a sphere or torus chart
//!   (`work/spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane.md`),
//!   which turns this row red the day it lands; the face is pinned, so a
//!   change to the matcher shows;
//! - the ball turned 1.1 rad about `(0.3, 1, 0.2)` and moved to
//!   `(0.42, −0.15, 0.3)` (donut ∪ ball): a spline edge in the run of a
//!   torus ring (`run_loop_arcs` on the torus), the same door;
//! - the donut turned 0.9 rad about `(1, 1, 0)` and moved to
//!   `(0.05, 0.1, 0.02)`, ∪ in both member orders: the ball cuts both
//!   tube circles of its meridian, two curves about the donut's axis
//!   that no plane or meridian tells apart; each germ's component is its
//!   site's foot on the carried section, and the union stops at the same
//!   volume door.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::finished;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{AtRestBody, Body, BooleanError, MassPropsError, ValidationError};

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

/// Whether `e` is the result gate's volume refusal on the one face
/// `face` (its key's rendering), a conic or spline trim on a sphere or
/// torus chart.
fn trim_refusal_on(e: &BooleanError, face: &str) -> bool {
    matches!(
        e,
        BooleanError::ResultInvalid { errors } if matches!(
            errors.as_slice(),
            [ValidationError::VolumeUncomputable {
                source: MassPropsError::Face {
                    face: f,
                    source: geom_brep::PropsError::QuadratureUnsupported { what },
                },
                ..
            }] if format!("{f:?}") == face
                && what.starts_with("conic trim on a cone/sphere/torus chart")
        )
    )
}

#[test]
fn a_ball_against_a_donut_carries_onto_the_torus_and_stops_at_the_volume() {
    let tol = Tol::witness();
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let (ball0, donut0) = (
        placed(&ball(), Vec3::unit_x(), 0.0, origin),
        placed(&donut(), Vec3::unit_x(), 0.0, origin),
    );
    // A carried chord closing a ring on a torus face.
    let ring = placed(&donut(), Vec3::unit_x(), 0.7, Vec3::new(0.1, 0.35, 0.2));
    let e = topo::union(&ball0, &ring, tol)
        .err()
        .unwrap_or_else(|| panic!("a torus ring: the union built; re-pin it"));
    assert!(trim_refusal_on(&e, "FaceKey(2v1)"), "a torus ring: {e:?}");
    // A spline edge in a torus ring's run.
    let moved = placed(
        &ball(),
        Vec3::new(0.3, 1.0, 0.2),
        1.1,
        Vec3::new(0.42, -0.15, 0.3),
    );
    let e = topo::union(&donut0, &moved, tol)
        .err()
        .unwrap_or_else(|| panic!("a torus ring's run: the union built; re-pin it"));
    assert!(
        trim_refusal_on(&e, "FaceKey(5v5)"),
        "a torus ring's run: {e:?}"
    );
    // Two curves about the axis, which no closed-form reading tells apart.
    let about = placed(
        &donut(),
        Vec3::new(1.0, 1.0, 0.0),
        0.9,
        Vec3::new(0.05, 0.1, 0.02),
    );
    for (x, y, order, kinds) in [
        (
            &ball0,
            &about,
            "ball ∪ donut",
            (geom::SurfaceKind::Sphere, geom::SurfaceKind::Torus),
        ),
        (
            &about,
            &ball0,
            "donut ∪ ball",
            (geom::SurfaceKind::Torus, geom::SurfaceKind::Sphere),
        ),
    ] {
        let e = topo::union(x, y, tol)
            .err()
            .unwrap_or_else(|| panic!("about the axis, {order}: the union built; re-pin it"));
        let trims = matches!(
            &e,
            BooleanError::ResultInvalid { errors } if matches!(
                errors.as_slice(),
                [ValidationError::VolumeUncomputable {
                    source: MassPropsError::Face {
                        source: geom_brep::PropsError::QuadratureUnsupported { what },
                        ..
                    },
                    ..
                }] if what.starts_with("conic trim on a cone/sphere/torus chart")
            )
        );
        assert!(trims, "about the axis, {order} ({kinds:?}): {e:?}");
    }
}

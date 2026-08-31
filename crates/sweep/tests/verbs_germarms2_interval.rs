//! The cyl×cyl germ arm at the CERTIFIED scalar (feature `interval`) —
//! the two-arm pattern for this unit's new decide sites.
//!
//! One predicate is new here, `bool_germ_frame_axes_coplanar`: the
//! signed axis-to-axis gap along `a₁ × a₂`, a LENGTH, and the thing
//! that separates a cylinder pair whose axes meet from a skew one. Both
//! arms are pinned, because the two answers license opposite things —
//! meeting axes take the pinch door, skew axes keep the general rung —
//! and an enclosure too wide to call either would silently turn one
//! into an escalation.
//!
//! The poses are the `f64` suite's, built through the same public
//! doors, so the two lanes differ in the SCALAR and in nothing else.
//! The coplanarity margin is exactly zero at every intersecting pose
//! here — both operands are built centred on the origin, so the
//! axis-to-axis displacement is the zero vector by construction rather
//! than by cancellation — and the skew row's is the dyadic `0.375` along `â₁ × â₂`.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Interval, Point2, Point3, Real, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn cyl(r: f64, h: f64) -> Body<Interval> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(iv(0.0), iv(0.0)), iv(r), tol).unwrap();
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(iv(0.0), iv(0.0), iv(-h))));
    let vp = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(&vp, Extrusion::Distance(iv(2.0 * h)), tol)
        .unwrap()
        .body
}

fn spin(b: &Body<Interval>, axis: Vec3<Interval>, angle: f64) -> Body<Interval> {
    topo::transform_rigid(
        b,
        &Affine3::rotation_about_axis(Point3::new(iv(0.0), iv(0.0), iv(0.0)), axis, iv(angle)),
        Tol::witness(),
    )
    .unwrap()
}

fn x_axis() -> Vec3<Interval> {
    Vec3::new(iv(1.0), iv(0.0), iv(0.0))
}

fn y_axis() -> Vec3<Interval> {
    Vec3::new(iv(0.0), iv(1.0), iv(0.0))
}

fn z_axis() -> Vec3<Interval> {
    Vec3::new(iv(0.0), iv(0.0), iv(1.0))
}

fn repose(b: &Body<Interval>) -> Body<Interval> {
    let r = Affine3::rotation_about_axis(
        Point3::new(iv(0.0), iv(0.0), iv(0.0)),
        Vec3::new(iv(1.0), iv(2.0), iv(3.0)).normalize(),
        iv(0.7),
    );
    topo::transform_rigid(
        b,
        &Affine3::from_parts(
            r.linear,
            r.translation + Vec3::new(iv(0.3), iv(-0.45), iv(0.6)),
        ),
        Tol::witness(),
    )
    .unwrap()
}

fn union_err(a: &Body<Interval>, b: &Body<Interval>) -> BooleanError {
    topo::union(a, b, Tol::witness()).expect_err("this family has no join arm")
}

/// **The meeting-axes arm.** The pose whose seams sit off the pinch
/// reaches the join at the certified scalar too, and stops at the door
/// that names the pinch — the same door the `f64` lane reaches. An
/// escalation here would say the enclosures decided the lane rather
/// than the geometry.
#[test]
fn the_pinch_door_is_reached_at_the_certified_scalar() {
    let a = spin(&cyl(1.0, 1.2), z_axis(), PI / 4.0);
    let b = spin(
        &spin(&cyl(1.0, 1.2), x_axis(), PI / 2.0),
        y_axis(),
        PI / 4.0,
    );
    let err = union_err(&a, &b);
    assert!(
        matches!(err, BooleanError::GermFrameCylinderPinch { .. }),
        "{err:?}"
    );
    assert_eq!(
        format!("{err:?}"),
        format!("{:?}", union_err(&repose(&a), &repose(&b))),
        "the re-posed pose must answer identically at the certified scalar too"
    );
}

/// **The skew arm.** Slide the same pair along the common
/// perpendicular `â₁ × â₂` by the dyadic `0.375` — the one direction
/// that separates the axes — and the coplanarity margin is definitely
/// non-zero: the pair keeps the general rung, never reaches the join,
/// and never wears the pinch door.
#[test]
fn a_skew_pair_stays_off_the_pinch_door_at_the_certified_scalar() {
    let a = cyl(1.0, 2.0);
    let skew = topo::transform_rigid(
        &spin(&cyl(1.0, 2.0), x_axis(), PI / 2.0),
        &Affine3::translation(Vec3::new(iv(0.375), iv(0.0), iv(0.0))),
        Tol::witness(),
    )
    .unwrap();
    let err = union_err(&a, &skew);
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "{err:?}"
    );
}

/// The Steinmetz pose itself keeps the crossing layer's tangency door
/// at the certified scalar: its seams are ON the pinch points, and a
/// tangency is not a crossing at any order this lane reads.
#[test]
fn the_steinmetz_pose_keeps_the_tangency_door_at_the_certified_scalar() {
    let a = cyl(1.0, 2.0);
    let b = spin(&cyl(1.0, 2.0), x_axis(), PI / 2.0);
    let err = union_err(&a, &b);
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "{err:?}"
    );
    assert_eq!(
        format!("{err:?}"),
        format!("{:?}", union_err(&repose(&a), &repose(&b))),
    );
}

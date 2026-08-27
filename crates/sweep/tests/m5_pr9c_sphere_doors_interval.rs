//! M5 PR 9c, item 1: the sphere containment/pierce doors at the
//! CERTIFIED scalar (feature `interval`).
//!
//! The point of the lane is that the new arm's margins are honest
//! enclosures, not f64 luck: the boundary residual
//! `(|q − c|² − r²)/2r` and the ray discriminant `b² − c` are both
//! squares of quantities that STRADDLE ZERO on a probe near the wall,
//! so they go through `powi(2)` (the zero-straddling-square rule) —
//! plain multiplication would hand `decide` a spurious negative lower
//! bound and the whole lane would refuse geometry it should accept.
//! Exact dyadic fixtures decide definitely from point enclosures.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::{Band, Interval, Point2, Point3, Real};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{SolidContainment, point_in_solid};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

fn p3(x: f64, y: f64, z: f64) -> Point3<Interval> {
    Point3::new(iv(x), iv(y), iv(z))
}

/// The unit ball at the certified scalar (the `revolve_ball` fixture).
fn ball() -> topo::Body<Interval> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(iv(0.0), iv(-1.0)), iv(1.0)),
        ProfileVertex::new(Point2::new(iv(0.0), iv(1.0)), iv(0.0)),
    ]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(iv(0.0), iv(0.0)),
        dir: geom_core::Vec2::new(iv(0.0), iv(1.0)),
    };
    revolve(&vp, axis, Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

#[test]
fn interval_sphere_doors_classify_in_out_and_boundary() {
    let body = ball();
    let b = Band::linear(Tol::witness()).unwrap();
    // Dyadic probes: exactly representable, so the enclosures are
    // points and every margin decides definitely.
    assert_eq!(
        point_in_solid(&body, Point3::origin(), b, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid(&body, p3(0.25, 0.125, -0.5), b, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid(&body, p3(2.0, 1.5, 0.25), b, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    // ON the wall: the dyadic Pythagorean point (0.6, 0.8, 0) is not
    // dyadic, so use the axis point — exactly on the unit sphere.
    assert_eq!(
        point_in_solid(&body, p3(0.0, 0.0, 1.0), b, Tol::witness()).unwrap(),
        SolidContainment::OnBoundary
    );
}

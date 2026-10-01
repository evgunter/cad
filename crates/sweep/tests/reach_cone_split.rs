//! The plane × cone split: a cone frustum cut by a tilted plane.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use topo::Body;
use topo::splitting::{SplitPlane, split};

/// Radius 1 at y = 0 narrowing to 1/2 at y = 1, about +y: the apex
/// sits at y = 2 and the half-angle is atan(1/2).
fn frustum() -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.5, 1.0),
        Point2::new(0.0, 1.0),
    ]);
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

#[test]
fn probe_measure() {
    let body = frustum();
    for phi in [0.0f64, 0.3, 0.6] {
        let plane = SplitPlane {
            origin: Point3::new(0.0, 0.5, 0.0),
            normal: Vec3::new(phi.sin(), phi.cos(), 0.0),
        };
        let r = split(&body, &plane, Tol::witness());
        eprintln!("phi {phi}: {:?}", r.as_ref().map(|_| ()));
    }
    // A cylinder (y in 0..1) under a cone (y in 1..2): cut the cylinder only.
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(1.0, 1.0),
        Point2::new(0.5, 2.0),
        Point2::new(0.0, 2.0),
    ]);
    let tower = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    for phi in [0.0f64, 0.3] {
        let plane = SplitPlane {
            origin: Point3::new(0.0, 0.5, 0.0),
            normal: Vec3::new(phi.sin(), phi.cos(), 0.0),
        };
        let r = split(&tower, &plane, Tol::witness());
        eprintln!("miss phi {phi}: {:?}", r.as_ref().map(|_| ()));
    }
}

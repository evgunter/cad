//! M5 PR 10 — the blinded review's Interval-containment probe,
//! ADOPTED into the PR (charter B): dense-grid containment on every
//! loft wall AND on a path-swept wall, where the rotated frames
//! exercise every affine component.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::NurbsSurface;
use geom_brep::SketchSegment;
use geom_core::{Affine3, Bounds, Interval, Point2, Real, Vec3};
use sweep::skin::{lift_surface, loft_geometry, segment_curve, sweep_geometry};

use crate::common;
use common::chain;
use geom_core::Tol;

fn contained(surface: &NurbsSurface<f64>, grid: usize) {
    let lifted = lift_surface::<Interval>(surface).expect("lifts");
    for i in 0..=grid {
        let u = i as f64 / grid as f64;
        for j in 0..=grid {
            let v = j as f64 / grid as f64;
            let a = surface.eval(u, v);
            let b = lifted.eval(Interval::from_f64(u), Interval::from_f64(v));
            for (x, e) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
                assert!(
                    e.lo() <= x && x <= e.hi(),
                    "({u},{v}): {x} escapes [{}, {}]",
                    e.lo(),
                    e.hi()
                );
            }
        }
    }
}

#[test]
fn review_interval_containment_dense_loft_and_sweep() {
    let places = [0.0, 1.0, 2.0].map(|z| Affine3::translation(Vec3::new(0.0, 0.0, z)));
    let loft = loft_geometry(
        &[chain(1.0), chain(1.6), chain(1.0)],
        &places,
        2,
        Tol::witness(),
    )
    .expect("loft");
    for wall in &loft.walls[0] {
        contained(wall, 96);
    }
    // A curved path: rotated frames exercise every affine component.
    let path = segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(3.0, 3.0),
            bulge: 0.4,
        },
        Affine3::identity(),
    )
    .expect("path");
    let swept = sweep_geometry(
        &chain(1.0),
        Affine3::identity(),
        &path,
        5,
        3,
        Tol::witness(),
    )
    .expect("sweeps");
    for wall in &swept.walls[0] {
        contained(wall, 64);
    }
}

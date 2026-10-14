//! Review probes for PR 5089 (reviewer r1). Not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Vec3};

/// The router's PARALLEL arm also reaches `torus_oblique_circle`
/// (axial decided Zero, centred decided apart). There the router has
/// NOT decided the tube-rate magnitude apart, so `unit_sign`'s
/// decided-Zero arm is reachable at a small K: a thin-tube grazer
/// tilted by a band-scale angle passes all five Villarceau gates Zero.
#[test]
fn p1_parallel_arm_reaches_unit_sign_zero_at_small_k() {
    for (k, r, h, delta) in [
        (2.0, 1.8e-9, 0.9e-9, 2.5e-9),
        (3.0, 1.9e-9, 0.95e-9, 2.9e-9),
        (10.0, 1.8e-9, 0.9e-9, 2.5e-9),
    ] {
        let band = Band::new(1e-9, k * 1e-9).unwrap();
        let big = 2.0;
        let torus = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: big,
            minor_radius: r,
            u_ref: Vec3::unit_x(),
        };
        let s = h / big; // R·sinθ = h
        let n = Vec3::new(0.0, -s, (1.0 - s * s).sqrt());
        let c = Curve3::Circle {
            center: Point3::new(delta, 0.0, 0.0),
            axis: n,
            radius: big,
            u_ref: Vec3::unit_x(),
        };
        let got = geom_brep::chart_pcurve(&c, &torus, band);
        println!("K={k}: {got:?}");
    }
}

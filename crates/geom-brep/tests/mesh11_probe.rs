//! Scratch measurement probe (MESH-11). Deleted before the PR.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face, require_iso_rectangle};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn edge(carrier: Curve3<f64>, a: f64, b: f64, start: u32, end: u32) -> LoopEdge<f64> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge::hand_built(carrier, t0, t1, forward, start, end)
}

fn cone_surface() -> Surface<f64> {
    Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn cone_rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, v * s),
            axis: v3(0.0, 0.0, 1.0),
            radius: (v * s).abs(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

fn generator(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let dir = v3(u.cos() * s, u.sin() * s, s);
    edge(
        Curve3::Line {
            origin: p3(0.0, 0.0, 0.0),
            dir,
        },
        v0,
        v1,
        a,
        b,
    )
}

#[test]
fn probe_cone_apex_crossing_generator() {
    let band = Band::linear(Tol::witness()).unwrap();
    let s = cone_surface();
    let bow = vec![
        cone_rim(-1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, -1.0, 1.0, 1, 2),
        cone_rim(1.0, core::f64::consts::PI, 0.0, 2, 3),
        generator(0.0, 1.0, -1.0, 3, 0),
    ];
    println!(
        "PROBE cone bow-tie door: {:?}",
        require_iso_rectangle(&s, &bow, band)
    );
    println!(
        "PROBE cone bow-tie flux: {:?}",
        curved_face(&s, &bow, 1.0, band)
    );
    let ok = vec![
        cone_rim(1.0, 0.0, core::f64::consts::PI, 0, 1),
        generator(core::f64::consts::PI, 1.0, 2.0, 1, 2),
        cone_rim(2.0, core::f64::consts::PI, 0.0, 2, 3),
        generator(0.0, 2.0, 1.0, 3, 0),
    ];
    println!(
        "PROBE cone single-nappe door: {:?}",
        require_iso_rectangle(&s, &ok, band)
    );
    println!(
        "PROBE cone single-nappe flux: {:?}",
        curved_face(&s, &ok, 1.0, band)
    );
}

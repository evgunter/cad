//! **R2 review probe for MESH-11, re-aimed by MESH-12** (issue 1601):
//! the extent fold on a saturated meridian span, `2π + 2δ`, whose
//! north pole sits `δ` inside the span. Written to compile on
//! MESH-11's merge base (it uses only `curved_face`), where it pinned
//! the fold measuring SHORT on 36 of 400 spans as a limitation; the
//! parse now refuses every such span by certification's own bound,
//! and the row pins that instead.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, PropsError, curved_face};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};

const PI: f64 = core::f64::consts::PI;
const RS: f64 = 0.010;

fn edge(carrier: Curve3<f64>, a: f64, b: f64, start: u32, end: u32) -> LoopEdge<f64> {
    let (t0, t1, forward) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge::hand_built(carrier, t0, t1, forward, start, end)
}
fn great(t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, -1.0, 0.0),
            radius: RS,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        },
        t0,
        t1,
        a,
        b,
    )
}

#[test]
fn r2_base_the_saturated_span_refuses_at_the_parse() {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: RS,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let bd = Band::linear(Tol::witness()).unwrap();
    let mut answered = Vec::new();
    let mut n = 0;
    for k in 1..=400 {
        let delta = 0.001 * f64::from(k) + 1e-7 * f64::from(k * k);
        if delta >= 1.0 {
            break;
        }
        n += 1;
        let t0 = PI / 2.0 - delta;
        let t1 = t0 + 2.0 * PI + 2.0 * delta;
        let pair = vec![great(t0, t1, 0, 1), great(t1, t0 + 4.0 * PI, 1, 0)];
        let r = curved_face(&sphere, &pair, 1.0, bd).map(|f| f.area);
        if !matches!(
            r,
            Err(PropsError::NotIsoRectangle {
                what: "props_meridian_span_winding"
            })
        ) {
            answered.push((delta, r));
        }
    }
    println!(
        "R2-BASE-SATURATED eps={:e}: {} of {n} spans not refused by the winding bound",
        Tol::witness().eps(),
        answered.len()
    );
    for (d, r) in answered.iter().take(5) {
        println!("R2-BASE-SATURATED delta={d:.6} {r:?}");
    }
    assert!(
        answered.is_empty(),
        "every saturated span refuses at the parse under `props_meridian_span_winding`"
    );
}

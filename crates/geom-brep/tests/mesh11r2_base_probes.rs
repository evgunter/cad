//! **R2 review probe for MESH-11, re-aimed by MESH-12** (issue 1601):
//! a saturated meridian span, `2π + 2δ`, whose north pole sits `δ`
//! inside the span, through `curved_face` alone: the parse refuses
//! every such span by certification's own bound, at every δ of the
//! review's sweep.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use crate::shared::tol::band;
use crate::shared::topo;
use geom::Surface;
use geom_brep::props::{LoopEdge, PropsError, curved_face};
use geom_core::Tol;
use geom_core::{Point3, Vec3};

const PI: f64 = core::f64::consts::PI;
const RS: f64 = 0.010;

fn great(t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    // The u = 0 case: sin 0 and cos 0 are exact, so the shared
    // builder's axis is bit-identical to the (0, -1, 0) this row
    // used to spell out, and its u_ref to (1, 0, 0).
    topo::sphere_great(RS, 0.0, t0, t1, a, b)
}

#[test]
fn r2_base_the_saturated_span_refuses_at_the_parse() {
    let sphere = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: RS,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let bd = band();
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

//! probe
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common::{peg, volume, wall_decls};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, RawLoop, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::{Body, BooleanOp, BooleanResult};

fn collar() -> Body<f64> {
    let lp = profile::ProfileLoop::polygon([
        Point2::new(0.5, 1.0),
        Point2::new(1.5, 1.0),
        Point2::new(1.5, 2.0),
        Point2::new(0.5, 2.0),
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

fn turned_peg(z0: f64, h: f64) -> Body<f64> {
    let m = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        -core::f64::consts::FRAC_PI_2,
    );
    topo::transform_rigid(&peg(z0, h), &m, Tol::witness()).unwrap()
}

#[test]
fn probe_full_turn_bore_mate() {
    for full in [true, false] {
    let c = if full { collar() } else { crate::mate2_common::collar() };
    for (z0, h) in [(0.5, 2.0), (1.0, 1.0)] {
        let p = if full { turned_peg(z0, h) } else { peg(z0, h) };
        let decls = wall_decls(&c, &p);
        let rdecls = wall_decls(&p, &c);
        println!("decls {}", decls.coincident_faces.len());
        for (op, a, b) in [(BooleanOp::Union, &c, &p), (BooleanOp::Intersect, &c, &p), (BooleanOp::Subtract, &c, &p), (BooleanOp::Subtract, &p, &c)] {
            let decls = if std::ptr::eq(a, &c) { &decls } else { &rdecls };
            let out = topo::boolean_op_with(op, a, b, decls, topo::SweepStrategy::Realized, Tol::witness());
            match out {
                Err(e) => println!("PROBE full={full} {z0} {h} {op:?} REFUSED: {e:?}"),
                Ok(BooleanResult::Empty) => println!("PROBE full={full} {z0} {h} {op:?} EMPTY"),
                Ok(BooleanResult::Body(bb)) => println!(
                    "PROBE {z0} {h} {op:?} volume {} (collar {}, peg {})",
                    volume(&bb.body),
                    volume(&c),
                    volume(&p)
                ),
            }
        }
    }
}
}

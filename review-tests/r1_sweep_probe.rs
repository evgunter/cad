//! Reviewer R1 probes (local, not committed).
#![allow(clippy::unwrap_used, clippy::panic, clippy::print_stdout)]

use crate::common::germ_pair::cyl;
use crate::conic_edge_curved_face::ball;
use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::{AtRestBody, BooleanOp};

fn turned(big: f64, c: [f64; 3], spin: f64) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let a = finished("the cylinder", cyl(0.5, 1.0), Tol::witness());
    let mut b = ball(big, c);
    if spin != 0.0 {
        let turn = Affine3::rotation_about_axis(
            Point3::from_array(c),
            Vec3::new(1.0, 2.0, 3.0).normalize(),
            spin,
        );
        let t = topo::transform_rigid(&b, &geom_core::test_support::rigid(&turn, Tol::witness()), Tol::witness()).unwrap();
        b = finished("the turned ball", t, Tol::witness());
    }
    (a, b)
}

fn key(p: Point3<f64>) -> [i64; 3] {
    [(p.x * 1e7).round() as i64, (p.y * 1e7).round() as i64, (p.z * 1e7).round() as i64]
}

/// Prints each run's segment set (sorted, rounded) and per-op time.
#[test]
fn r1_segments_and_times() {
    let poses = [
        ("turned chart", 0.25, [0.3, -0.35, 0.4], 0.9),
        ("ball wider", 0.699, [0.2, 0.0, 0.0], 0.0),
        ("two loops turned", 0.8, [0.2, 0.1, 0.05], 0.6),
    ];
    for (label, big, c, spin) in poses {
        let (a, b) = turned(big, c, spin);
        for (ol, op, sw) in [
            ("A∪B", BooleanOp::Union, false),
            ("B∪A", BooleanOp::Union, true),
            ("B∩A", BooleanOp::Intersect, true),
            ("B∖A", BooleanOp::Subtract, true),
        ] {
            let (x, y) = if sw { (&b, &a) } else { (&a, &b) };
            let t0 = std::time::Instant::now();
            let s = topo::test_support::boolean_segment_sites(op, x, y, Tol::witness());
            let t_seg = t0.elapsed();
            let mut segs: Vec<[[i64; 3]; 2]> = match &s {
                Ok(Some((_, v))) => v.iter().map(|s| { let mut k = [key(s[0]), key(s[1])]; k.sort(); k }).collect(),
                other => { println!("{label} {ol}: {other:?}"); continue; }
            };
            segs.sort();
            let t1 = std::time::Instant::now();
            let r = match op {
                BooleanOp::Union => topo::union(x, y, Tol::witness()),
                BooleanOp::Intersect => topo::intersect(x, y, Tol::witness()),
                BooleanOp::Subtract => topo::subtract(x, y, Tol::witness()),
            };
            let t_op = t1.elapsed();
            println!("{label} {ol}: segsites {t_seg:?} op {t_op:?} err {:?}", r.err());
            println!("{label} {ol}: SEGS {segs:?}");
        }
    }
}

#[test]
fn r1_ball_faces_share_a_surface() {
    let (_, b) = turned(0.25, [0.3, -0.35, 0.4], 0.9);
    for (k, f) in b.faces() {
        println!("ball face {k:?} surface {:?}", f.surface);
    }
}

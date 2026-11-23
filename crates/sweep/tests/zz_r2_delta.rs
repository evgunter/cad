//! R2 delta probes for PR 7432's fix pass: a junction whose operand's
//! other face across its convex rim edge is crossed by the junction's
//! partner away from the rim.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::PI;

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult};

fn body(what: &str, verts: Vec<(Point2<f64>, f64)>) -> AtRestBody<f64> {
    finished(what, revolved_about_y(verts, Revolution::Full, Tol::witness()), Tol::witness())
}
fn p(x: f64, y: f64, b: f64) -> (Point2<f64>, f64) {
    (Point2::new(x, y), b)
}

/// A rod capped by a cone of height `h` (apex on the axis), and the unit
/// half ball on its rim: the rod's wall and the sphere a π junction over
/// two convex rim edges; the sphere and the cone cross off the rim when
/// `h > 1`.
fn run(h: f64) {
    let q = (PI / 8.0).tan();
    let a = body("coned rod", vec![p(0.0, -2.0, 0.0), p(1.0, -2.0, 0.0), p(1.0, 0.0, 0.0), p(0.0, h, 0.0)]);
    let b = body("half ball", vec![p(0.0, 0.0, 0.0), p(1.0, 0.0, q), p(0.0, 1.0, 0.0)]);
    let n = 400_000;
    let top = h.max(1.0);
    let dy = top / n as f64;
    let f = |y: f64| {
        let s = if y <= 1.0 { (1.0 - y * y).max(0.0).sqrt() } else { 0.0 };
        let c = if y <= h { 1.0 - y / h } else { 0.0 };
        let r = s.max(c);
        PI * r * r
    };
    let mut acc = f(0.0) + f(top);
    for i in 1..n {
        acc += f(i as f64 * dy) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    let want = 2.0 * PI + acc * dy / 3.0;
    if let Ok(rows) = std::panic::catch_unwind(|| topo::boolean::rim_door(&a, &b, Tol::witness())) {
        println!("R2DELTA h={h} door: {rows:?}");
    }
    for (order, x, y) in [("a∪b", &a, &b), ("b∪a", &b, &a)] {
        match topo::union(x, y, Tol::witness()) {
            Ok(BooleanResult::Body(bb)) => {
                let v = topo::test_support::pinned_volume("u", &bb.body, Tol::witness()).volume;
                let t3 = topo::validate_geometric(&bb.body, Tol::witness()).is_ok();
                println!("R2DELTA h={h} {order}: BODY vol={v} want={want} rel={:e} t3={t3}", (v - want).abs() / want);
                assert!((v - want).abs() <= 1e-7 * want, "h={h} {order}: wrong volume {v} vs {want}");
            }
            Ok(_) => println!("R2DELTA h={h} {order}: EMPTY"),
            Err(e) => println!("R2DELTA h={h} {order}: REFUSED {:?} :: {e}", e.kind()),
        }
    }
}

#[test]
fn r2_coned_rod_tall_cone_crosses_the_half_ball() {
    run(2.0);
}

#[test]
fn r2_coned_rod_short_cone_inside_the_half_ball() {
    run(0.5);
}

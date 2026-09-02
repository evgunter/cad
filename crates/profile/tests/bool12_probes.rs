//! **The declared ARRIVAL at the seam** (BOOL-12).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Bulge, Open, PathError, Start};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// MEASUREMENT 1 (spec "FIRST, before the build"): the stadium closed
/// with `.tangent().tangent_arc_to(Start)` — a G1 ARRIVAL at the seam.
#[test]
fn measure_the_stadium_today() {
    let t = Tol::witness();
    let out = Open
        .at(p2(0.0, 0.0))
        .angle(0.0, t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(p2(2.0, 2.0), t)
        .unwrap()
        .line(2.0, t)
        .unwrap()
        .tangent()
        .tangent_arc_to(Start, t);
    panic!("stadium close => {out:?}");
}

/// MEASUREMENT: the canonical D-shape, closing straight into the
/// entry's first side.
#[test]
fn measure_the_d_shape_today() {
    use std::f64::consts::FRAC_PI_2;
    let t = Tol::witness();
    let mk = |close_declared: bool| {
        let p = Open
            .at(p2(0.0, 0.0))
            .angle(FRAC_PI_2, t)
            .unwrap()
            .line(2.0, t)
            .unwrap()
            .arc_to(Bulge { p: p2(0.0, -2.0), b: 1.0 }, t)
            .unwrap()
            .line_to(p2(0.0, -1.0), t)
            .unwrap();
        if close_declared {
            format!("{:?}", p.continue_to(Start, t).map(|_| "CLOSED"))
        } else {
            format!("{:?}", p.line_to(Start, t).map(|_| "CLOSED"))
        }
    };
    panic!("D-shape declared-continuation close => {}\nD-shape line_to(Start) close => {}", mk(true), mk(false));
}

//! **An annular one-segment tube through a plate.** Two one-segment
//! circles about the origin, `r = 1` with its one vertex at azimuth `ao`
//! and `r = 0.5` at `ai`, extruded over `z ∈ [0.5, 2.5]`, against the
//! plate `(−3, 3)² × (0, 1)`. The plate's top face cuts both walls
//! across their seams: two one-site loops in one face, which the
//! wrap-edge arm joins. The section's null face then has two loops whose
//! region faces are the inner disc and the annulus, planar faces every
//! vertex and edge of which lies on the tube's walls, so only a point of
//! a region face's interior decides the roles.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Affine3, Arc2, Point2, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, Segment, SketchPlane};
use sweep::test_support::brick;
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;

use crate::common::differential::{every_op_both_orders, outcome};

fn tol() -> Tol {
    Tol::witness()
}

/// A circle of radius `r` about `(cx, cy)` in `n` equal arcs, its first
/// vertex at azimuth `az`, wound `sweep`'s way.
fn circle(cx: f64, cy: f64, r: f64, az: f64, n: usize, sweep: f64) -> ProfileLoop<f64> {
    RawLoop::new((0..n).map(|k| {
        let a = az + sweep * k as f64 / n as f64;
        (
            Point2::new(cx + r * a.cos(), cy + r * a.sin()),
            Segment::Arc(Arc2 {
                centre: Point2::new(cx, cy),
                radius: r,
                sweep: sweep / n as f64,
            }),
        )
    }))
}

/// `loops` extruded from `z = z0` up by `h`.
fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let profile = Profile::new(SketchPlane::<f64>::xy(), loops)
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: h,
        side: ExtrudeSide::Along,
    };
    let body = extrude(&profile, depth, tol()).unwrap().body;
    let up = Affine3::translation(Vec3::new(0.0, 0.0, z0));
    topo::transform_rigid(&body, &up, tol()).unwrap()
}

fn fin(what: &str, body: Body<f64>) -> topo::AtRestBody<f64> {
    topo::test_support::finished(what, body, tol())
}

/// Probe: every op's outcome line over the witness poses.
#[test]
#[ignore = "probe: cargo nextest run -p sweep --run-ignored all annular_probe"]
fn annular_probe() {
    let plate = brick((-3.0, 3.0), (-3.0, 3.0), (0.0, 1.0), tol());
    for (ao, ai) in [(0.0, 0.0), (0.0, 1.0), (0.0, PI), (1.0, 4.0)] {
        let tube = extruded(
            vec![
                circle(0.0, 0.0, 1.0, ao, 1, TAU),
                circle(0.0, 0.0, 0.5, ai, 1, -TAU),
            ],
            0.5,
            2.0,
        );
        let (a, b) = (fin("tube", tube), fin("plate", plate.clone()));
        let annulus = 0.75 * PI;
        for (op, r, want) in every_op_both_orders(&a, &b, (2.0 * annulus, 36.0, 0.5 * annulus), tol()) {
            println!("({ao}, {ai}) {op}: {}", outcome(r, want, tol()));
        }
    }
}

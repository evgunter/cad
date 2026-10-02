//! probe
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, RawLoop, SketchPlane};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, BooleanOp};

fn ball(r: f64, c: Vec3<f64>) -> Body<f64> {
    let b = revolved_about_y(
        vec![(Point2::new(0.0, -r), 1.0), (Point2::new(0.0, r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    );
    topo::transform_rigid(&b, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn slab(z0: f64, h: f64) -> Body<f64> {
    let p = Profile::new(
        SketchPlane::xy(),
        vec![profile::ProfileLoop::polygon([
            Point2::new(-2.0, -2.0),
            Point2::new(2.0, -2.0),
            Point2::new(2.0, 2.0),
            Point2::new(-2.0, 2.0),
        ])],
    )
    .validate(Tol::witness())
    .unwrap();
    let b = sweep::extrude(&p, sweep::Extrusion::Distance(h), Tol::witness()).unwrap().body;
    topo::transform_rigid(&b, &Affine3::translation(Vec3::new(0.0, 0.0, z0)), Tol::witness()).unwrap()
}

fn cap(r: f64, h: f64) -> f64 {
    PI * h * h * (3.0 * r - h) / 3.0
}

fn lens(r1: f64, r2: f64, d: f64) -> f64 {
    let x = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    cap(r1, r1 - x) + cap(r2, r2 - (d - x))
}

fn probe(pose: &str, a: &Body<f64>, b: &Body<f64>, va: f64, vb: f64, both: f64) {
    for (label, op, x, y, want) in [
        ("A u B", BooleanOp::Union, a, b, va + vb - both),
        ("A n B", BooleanOp::Intersect, a, b, both),
        ("A - B", BooleanOp::Subtract, a, b, va - both),
        ("B - A", BooleanOp::Subtract, b, a, vb - both),
    ] {
        let out = match op {
            BooleanOp::Union => topo::boolean::union(x, y, Tol::witness()),
            BooleanOp::Intersect => topo::boolean::intersect(x, y, Tol::witness()),
            BooleanOp::Subtract => topo::boolean::subtract(x, y, Tol::witness()),
        };
        let out = match out {
            Ok(o) => o,
            Err(e) => {
                eprintln!("PROBE {pose} {label}: refused {e:?}");
                continue;
            }
        };
        let body = &out.body().unwrap().body;
        let p = topo::mass_properties(body, Tol::witness()).map(|p| p.volume);
        eprintln!(
            "PROBE {pose} {label}: faces {} v {:?} c {:?} g {:?} vol {:?} want {want}",
            body.faces().count(),
            topo::validate(body),
            topo::validate_closed(body),
            topo::validate_geometric(body, Tol::witness()),
            p
        );
    }
}

#[test]
fn probe_all() {
    let (r1, r2) = (1.0, 0.7);
    let (va, vb) = (4.0 * PI / 3.0, 4.0 * PI / 3.0 * r2 * r2 * r2);
    probe("X", &ball(1.0, Vec3::new(2.0, 2.0, 0.5)), &ball(1.0, Vec3::new(3.4, 2.0, 0.5)), va, va, lens(1.0, 1.0, 1.4));
    let c = Vec3::new(0.9, 0.3, 0.6);
    let d = c.norm();
    probe("diag", &ball(r1, Vec3::new(0.0, 0.0, 0.0)), &ball(r2, c), va, vb, lens(r1, r2, d));
    let c = Vec3::new(0.0, 0.0, 1.2);
    probe("Zoff", &ball(r1, Vec3::new(0.0, 0.0, 0.0)), &ball(r2, c), va, vb, lens(r1, r2, 1.2));
    let base = Vec3::new(2.0, 2.0, 0.5);
    for (name, r, off) in [
        ("XY", 1.0, Vec3::new(1.2, 0.6, 0.0)),
        ("XZ", 1.0, Vec3::new(1.3, 0.0, 0.2)),
        ("Xsmall", 0.6, Vec3::new(0.9, 0.0, 0.0)),
        ("Xsmall-in", 0.3, Vec3::new(0.9, 0.0, 0.0)),
        ("XYsmall", 0.5, Vec3::new(0.8, 0.4, 0.0)),
    ] {
        let vb = 4.0 * PI / 3.0 * r * r * r;
        probe(name, &ball(1.0, base), &ball(r, base + off), va, vb, lens(1.0, r, off.norm()));
    }
    probe("slab", &slab(0.5, 2.5), &ball(1.0, Vec3::new(0.0, 0.0, 0.0)), 16.0 * 2.5, va, cap(1.0, 0.5));
}

//! Probe: an edge on a ruling of a cylinder face, lying in a planar face
//! of its own solid.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol};
use profile::{SketchPlane, test_support::bulge_loop};
use sweep::test_support::{extruded, finished};
use topo::{AtRestBody, BooleanResult, subtract, union, validate_pseudomanifold};

fn tol() -> Tol {
    Tol::witness()
}

fn prism(what: &str, pts: &[(f64, f64, f64)], z0: f64, h: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    let lp = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(x, y), b))
            .collect(),
    );
    finished(what, extruded(plane, vec![lp], h, tol()), tol())
}

fn body_of(what: &str, r: Result<BooleanResult<f64>, topo::BooleanError>) -> topo::BooleanBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b,
        other => panic!("{what}: {other:?}"),
    }
}

fn c_block() -> topo::BooleanBody<f64> {
    let block = prism(
        "block",
        &[(0.0, 0.0, 0.0), (4.0, 0.0, 0.0), (4.0, 4.0, 0.0), (0.0, 4.0, 0.0)],
        0.0,
        3.0,
    );
    let slot = prism(
        "slot",
        &[(-1.0, 1.0, 0.0), (3.0, 1.0, 0.0), (3.0, 2.5, 0.0), (-1.0, 2.5, 0.0)],
        -1.0,
        5.0,
    );
    body_of("c", subtract(&block, &slot, tol()))
}

#[test]
fn probe() {
    let c = c_block();
    for (what, ridge) in [
        ("planar ridge", vec![(1.5, 1.0, 0.0), (2.1, 3.0, 0.0), (0.9, 3.0, 0.0)]),
        ("transverse arc", vec![(1.5, 1.0, 0.2), (2.1, 3.0, 0.0), (0.9, 3.0, 0.0)]),
        ("transverse arc in", vec![(1.5, 1.0, -0.2), (2.1, 3.0, 0.0), (0.9, 3.0, 0.0)]),
        (
            "tangent arc",
            vec![(1.5, 1.0, 0.414_213_562_373_095_1), (2.5, 2.0, 0.0), (2.5, 3.0, 0.0), (1.5, 3.0, 0.0)],
        ),
    ] {
        let r = prism(what, &ridge, 1.0, 1.0);
        match union(&c.body, &r, tol()) {
            Ok(BooleanResult::Body(b)) => {
                eprintln!(
                    "{what}: solids {} contacts {:?}\n  census {:?}",
                    b.body.solids().count(),
                    b.contacts,
                    validate_pseudomanifold(&b.body, &b.contacts, tol())
                );
            }
            other => eprintln!("{what}: union {other:?}"),
        }
    }
}

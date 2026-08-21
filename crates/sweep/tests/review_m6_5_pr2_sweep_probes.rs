//! Adversarial-review probes for the fillet naming work, sweep side.
//! They touch no naming API, so each runs unchanged at any revision
//! and measures rather than asserts: X4 pins where the boolean
//! refuses a filleted operand, X3b prints a geometry fingerprint two
//! revisions can be diffed on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use profile::RawLoop;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use geom_core::{Band, Point2, Tolerance};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::build::fillet_edges;
use sweep::{Extrusion, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations};
use geom_core::Tol;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn box_at(x0: f64, l: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(x0, 0.0), (x0 + l, 0.0), (x0 + l, l), (x0, l)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(l), Tol::witness()).unwrap().body
}

fn filleted_die() -> Body<f64> {
    let cube0 = box_at(0.0, 1.0);
    let edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    fillet_edges(&cube0, &edges, 0.125, band(), Tol::witness())
        .expect("the fillet")
        .body
}

/// X4: the kernel's boolean refuses a fully filleted operand with
/// `FallbackExtentUnsupported` even when the second operand is far
/// away and DISJOINT — the refusal is about carrying sphere octants at
/// all, not about the cut.
#[test]
fn x4_disjoint_boolean_over_a_filleted_body_refuses_at_the_extent() {
    let a = filleted_die();
    let far = box_at(4.0, 1.0);
    let out = boolean_op_with(
        BooleanOp::Union,
        &a,
        &far,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    );
    match out {
        Err(topo::BooleanError::FallbackExtentUnsupported { .. }) => {}
        other => panic!(
            "expected FallbackExtentUnsupported on a disjoint operand, got: {:?}",
            other.map(|_| "Ok(..)")
        ),
    }
}

/// X3b: a stable fingerprint of the every-edge fillet's geometry —
/// printed so the same probe at two revisions can be diffed. `Debug`
/// of the body covers arenas, keys, surfaces, points.
#[test]
fn x3b_print_every_edge_fillet_geometry_fingerprint() {
    let body = filleted_die();
    let repr = format!("{body:?}");
    let mut h = DefaultHasher::new();
    repr.hash(&mut h);
    println!(
        "EVERY-EDGE-FILLET-FINGERPRINT len={} hash={:016x}",
        repr.len(),
        h.finish()
    );
}

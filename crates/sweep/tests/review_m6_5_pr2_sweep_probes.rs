//! Adversarial-review probes for the fillet naming work, sweep side.
//! They touch no naming API, so each runs unchanged at any revision
//! and measures rather than asserts: X4 pins where the boolean
//! refuses a filleted operand, X3b prints a geometry fingerprint two
//! revisions can be diffed on.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use profile::RawLoop;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use geom_core::Tol;
use geom_core::{Band, Point2};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::build::fillet_edges;
use sweep::{Extrusion, extrude};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations};

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
    extrude(&profile, Extrusion::Distance(l), Tol::witness())
        .unwrap()
        .body
}

fn filleted_die() -> Body<f64> {
    let cube0 = box_at(0.0, 1.0);
    let edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    fillet_edges(&cube0, &edges, 0.125, band(), Tol::witness())
        .expect("the fillet")
        .body
}

/// X4: **the frontier this probe named has MOVED, and the probe now
/// says where to.** Carrying sphere octants was the blocker: the extent
/// scan refused a trimmed sphere face group on sight, because the
/// certificate `center ± r` is the whole group's. That certificate is
/// asked where it is USED now, and a trimmed group is served for every
/// separation test — so this fixture no longer stops there.
///
/// It still stops, at an arm that has nothing to do with trimming and
/// everything to do with where the fixture puts its operand: a fillet
/// corner sphere is TANGENT to the flat faces it blends, and this far
/// box is axis-aligned on the same y and z ranges as the die, so it
/// contributes plane CARRIERS the corner spheres touch exactly. That is
/// the scan's touching-configuration arm, and it is honest.
///
/// `x4b` moves the same operand off those carriers and the union
/// completes — which is the actual measurement of the frontier moving.
#[test]
fn x4_disjoint_boolean_over_a_filleted_body_meets_the_plane_tangency_arm() {
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
        Err(topo::BooleanError::FallbackExtentUnsupported { what, .. }) => {
            assert!(
                what.contains("tangent"),
                "the trimmed-group arm has retired; what stops this fixture is the \
                 plane-carrier tangency the fillet itself creates: {what}"
            );
        }
        other => panic!(
            "expected FallbackExtentUnsupported on a disjoint operand, got: {:?}",
            other.map(|_| "Ok(..)")
        ),
    }
}

/// X4b: the same filleted die and the same far box, translated OFF the
/// die's own plane carriers. Nothing is tangent to anything, and the
/// body carrying eight sphere OCTANTS assembles: two shells, volumes
/// add, tier-3 valid. This is the row X4's instruction asked for.
#[test]
fn x4b_a_filleted_body_assembles_with_an_operand_off_its_carriers() {
    let a = filleted_die();
    // Same far box, translated OFF the die's own plane carriers.
    let far = topo::transform_rigid(
        &box_at(4.0, 1.0),
        &geom_core::Affine3::translation(geom_core::Vec3::new(0.0, 2.0, 2.0)),
        Tol::witness(),
    )
    .unwrap();
    let out = boolean_op_with(
        BooleanOp::Union,
        &a,
        &far,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    );
    let out = out.expect("a filleted body is an operand now");
    let body = &out.body().expect("a body").body;
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(body.shells().count(), 2, "the die and the box, disjoint");
    let want = topo::mass_properties(&a, Tol::witness()).unwrap().volume + 1.0;
    let got = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    assert!(
        (got - want).abs() <= 1e-9 * want,
        "disjoint union adds volumes: {got} vs {want}"
    );
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

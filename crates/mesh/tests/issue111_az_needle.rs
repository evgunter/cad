//! Issue #111 acceptance: the A×Z intersect result must tessellate
//! watertight.
//!
//! The boolean result is exact-kernel-perfect (tiers 1/2/3′ + exact
//! volume oracles, pinned in `sweep/tests/issue93_az_intersect.rs`),
//! but `mesh::tessellate` used to emit a NON-watertight mesh:
//! `check_mesh` → `BoundaryEdge`. Face `19v3` (A's left inner-leg
//! slant) carries FOUR boundary vertices on one carrier line, one of
//! which — the boolean's crossing of the leg-top line with Z's
//! slope-3/5 diagonal, exactly 5/8 but non-dyadic in the arithmetic —
//! lands 1 ulp off that line. The CDT triangulates the resulting
//! 1-ulp-thick needle region and the old centroid-parity filter, a
//! ray-cast from a point ~1e-16 from the boundary, kept an EXTERIOR
//! needle: its shared boundary edge became 3× and two edges 1×.
//!
//! The classification is now combinatorial (constraint-crossing flood
//! fill; see `planar.rs`), so each boundary edge is used by exactly one
//! kept triangle by construction — no in-band float decision remains.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tolerance, Vec3};
use mesh::tessellate;
use mesh::validate::{check_mesh, signed_volume};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanResult};
use geom_core::Tol;

/// 880383/327680: A with the counter as a true inner loop (genus 1).
const ORACLE_COUNTER: f64 = 880_383.0 / 327_680.0;
/// 868511/327680: stencil A (counter + 1/16 crossbar slot).
const ORACLE_STENCIL: f64 = 868_511.0 / 327_680.0;
/// 903207/327680: plain A (no counter).
const ORACLE_PLAIN: f64 = 903_207.0 / 327_680.0;

const A_OUTLINE: [(f64, f64); 8] = [
    (0.0, 0.0),
    (0.625, 0.0),
    (0.8125, 1.0),
    (1.1875, 1.0),
    (1.375, 0.0),
    (2.0, 0.0),
    (1.125, 2.5),
    (0.875, 2.5),
];

const A_COUNTER: [(f64, f64); 3] = [(0.90625, 1.4375), (1.09375, 1.4375), (1.0, 2.0)];

const A_STENCIL: [(f64, f64); 15] = [
    (0.0, 0.0),
    (0.625, 0.0),
    (0.8125, 1.0),
    (0.96875, 1.0),
    (0.96875, 1.4375),
    (0.90625, 1.4375),
    (1.0, 2.0),
    (1.09375, 1.4375),
    (1.03125, 1.4375),
    (1.03125, 1.0),
    (1.1875, 1.0),
    (1.375, 0.0),
    (2.0, 0.0),
    (1.125, 2.5),
    (0.875, 2.5),
];

fn lp(poly: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(
        poly.iter()
            .map(|&(x, y)| Point2::new(x, y))
            .collect::<Vec<_>>(),
    )
}

fn validated(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(plane, loops)
        .validate(Tol::witness())
        .expect("profile validation")
}

fn a_prism(loops: Vec<ProfileLoop<f64>>) -> Body<f64> {
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, -0.0625),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    extrude(&validated(plane, loops), Extrusion::Distance(2.125), Tol::witness())
        .expect("extrude A")
        .body
}

fn z_prism() -> Body<f64> {
    let z_poly = [
        (-0.0625, 0.0),
        (2.5625, 0.0),
        (2.5625, 0.4375),
        (0.6875, 0.4375),
        (2.5625, 1.5625),
        (2.5625, 2.0),
        (-0.0625, 2.0),
        (-0.0625, 1.5625),
        (1.8125, 1.5625),
        (-0.0625, 0.4375),
    ];
    let plane = SketchPlane::from_frame(
        Point3::new(-0.0625, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    );
    extrude(
        &validated(plane, vec![lp(&z_poly)]),
        Extrusion::Distance(2.125),
        Tol::witness(),
    )
    .expect("extrude Z")
    .body
}

fn az(loops: Vec<ProfileLoop<f64>>, label: &str) -> Body<f64> {
    match topo::intersect(&a_prism(loops), &z_prism(), Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("{label}: A×Z intersect did not produce a body ({other:?})"),
    }
}

/// Tessellate at several δ and demand a closed, consistently wound,
/// outward mesh whose divergence-theorem volume matches the boolean's
/// exact oracle (planar faces ⇒ zero deviation, so the match is exact
/// up to summation rounding).
fn watertight_at_every_delta(body: &Body<f64>, oracle: f64, label: &str) {
    // δ-independent by construction (every carrier is a line, so no
    // chord subdivision happens) — sweep δ anyway to pin that.
    for delta in [1e-1, 1e-2, 1e-3] {
        let mesh = tessellate(body, delta, Tol::witness()).unwrap_or_else(|e| {
            panic!("{label} @ δ={delta}: tessellate refused {e:?}");
        });
        assert_eq!(
            check_mesh(&mesh),
            Ok(()),
            "REGRESSION (issue #111): {label} @ δ={delta} tessellated non-watertight"
        );
        let v = signed_volume(&mesh);
        assert!(
            (v - oracle).abs() < 1e-9,
            "{label} @ δ={delta}: mesh volume {v} vs exact oracle {oracle}"
        );
    }
}

/// The #284 companion row: the #111 body through the LIVE planar lane
/// under the boundary-derived chart frame. `planar.rs`'s stored-chart
/// replay pins the historical frame's needle chart forever, but it
/// bypasses `tessellate_planar` — so it cannot see the frame the live
/// path now derives (Newell normal + extent-aligned axes, #284). This
/// row runs the counter-hole body end-to-end twice and demands the
/// re-derived frame's output be byte-identical (D9: the frame is a
/// pure function of the boundary walk) as well as watertight and
/// volume-exact — the needle-carrier face included.
#[test]
fn survives_az_counter_through_rederived_frame_deterministic() {
    let body = az(
        vec![lp(&A_OUTLINE), lp(&A_COUNTER)],
        "A with counter hole (rederived frame)",
    );
    let m1 = tessellate(&body, 1e-2, Tol::witness()).expect("first tessellation");
    let m2 = tessellate(&body, 1e-2, Tol::witness()).expect("second tessellation");
    assert_eq!(
        m1.positions.len(),
        m2.positions.len(),
        "rebuild changed the vertex census"
    );
    for (a, b) in m1.positions.iter().zip(&m2.positions) {
        assert_eq!(
            (a.x.to_bits(), a.y.to_bits(), a.z.to_bits()),
            (b.x.to_bits(), b.y.to_bits(), b.z.to_bits()),
            "rebuild not bit-identical"
        );
    }
    let tris = |m: &mesh::Mesh| {
        m.patches
            .iter()
            .map(|p| p.triangles.clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(tris(&m1), tris(&m2), "rebuild changed the triangles");
    assert_eq!(check_mesh(&m1), Ok(()), "rederived frame not watertight");
    let v = signed_volume(&m1);
    assert!(
        (v - ORACLE_COUNTER).abs() < 1e-9,
        "rederived frame volume {v} vs oracle {ORACLE_COUNTER}"
    );
}

#[test]
fn survives_az_counter_hole_tessellates_watertight() {
    let body = az(vec![lp(&A_OUTLINE), lp(&A_COUNTER)], "A with counter hole");
    watertight_at_every_delta(&body, ORACLE_COUNTER, "A with counter hole");
}

#[test]
fn survives_az_stencil_tessellates_watertight() {
    let body = az(vec![lp(&A_STENCIL)], "stencil A");
    watertight_at_every_delta(&body, ORACLE_STENCIL, "stencil A");
}

#[test]
fn survives_az_plain_tessellates_watertight() {
    let body = az(vec![lp(&A_OUTLINE)], "plain A");
    watertight_at_every_delta(&body, ORACLE_PLAIN, "plain A");
}

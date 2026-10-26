//! M5 S1 adversarial-review probes, ADOPTED (fix pass): the
//! reviewer's e2e falsification fixtures with their observed outcomes
//! pinned tight (each was originally written to accept either an
//! exact build or a typed refusal; what the lane actually does is now
//! the assertion).
//!
//! History note (the review's F1 probe, not runnable against HEAD):
//! at the S1 merge-base the declared crosslap union refused
//! `JoinDesync { what: "every chord arc separates a loose scaffolding
//! pair" }`, and a loose-germ dump showed the root cause the
//! branch claims — one seam segment's two end records carrying
//! DIFFERENT face-pair meta (e.g. `(A:side, B:bottom)` vs
//! `(A:bottom, B:notchwall)`) because a REST germ direction lies in
//! four coincident planes. The reviewer reproduced exactly that at
//! the merge-base (probe `f1_prebranch_declared_crosslap_refusal`);
//! against HEAD the same fixture BUILDS, so the probe survives here
//! as this note plus `crosslap_rest.rs`'s certified pins.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, prism_z};
use geom_core::Tol;
use topo::{
    BooleanResult, BooleanResultKind, mass_properties, subtract, union, validate_geometric,
    validate_pseudomanifold,
};

fn glued(
    a: &topo::AtRestBody<f64>,
    b: &topo::AtRestBody<f64>,
    volume: f64,
) -> topo::BooleanBody<f64> {
    let g = match union(a, b, Tol::witness()).expect("declared union builds") {
        BooleanResult::Body(g) => g,
        BooleanResult::Empty => panic!("REST union cannot be empty"),
    };
    assert_eq!(g.kind, BooleanResultKind::Seamed);
    assert_eq!(
        mass_properties(
            &g.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        volume
    );
    assert_eq!(validate_geometric(&g.body, Tol::witness()), Ok(()));
    assert_eq!(
        validate_pseudomanifold(&g.body, &g.contacts, Tol::witness()),
        Ok(())
    );
    let mesh = mesh::tessellate(&g.body, 1e-2, Tol::witness()).expect("tessellate");
    mesh::validate::check_mesh(&mesh).expect("watertight, consistently oriented");
    let v = mesh::validate::signed_volume(&mesh);
    assert!(
        ((v - volume) / volume).abs() < 1e-9,
        "STL-bound signed volume {v} vs exact {volume}"
    );
    g
}

/// PROBE 1 (review F2e): overhang rest — the contact region is a
/// PROPER SUBSET of both contact faces (plate top and overhanging
/// brick bottom), with one pierce site per solid. Builds exactly.
#[test]
fn probe_overhang_partial_both_faces() {
    let plate = finished(
        "plate",
        brick((0.0, 4.0), (0.0, 4.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    let over = finished(
        "over",
        brick((2.0, 6.0), (2.0, 6.0), (1.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    glued(&plate, &over, 32.0);
}

/// PROBE 2 (review F2a → MAJOR-1's repro): the symmetric two-patch
/// bridge — B's two identical legs rest on A at mirror-symmetric
/// squares. The two-patch glue leaves the merged side faces adjacent
/// across TWO disjoint seam runs, so the declared merge closes a hole
/// (a genuine ring). Pre-fix the intra-face `kemr`'s provisional ring
/// designation shipped INVERTED roles (outer = the hole, area −0.5):
/// every volume gate passed (they are role-invariant), mass
/// properties were exact — and tessellation failed MismatchedWinding
/// with an STL signed volume of ~1.83 vs true 5.5, the silent
/// corrupt-export class. The merge's winding-based role
/// normalization plus tier-3 check 6 close it; this pin holds the
/// door shut: exact volume, watertight mesh, STL-bound signed volume
/// equal to the exact one.
#[test]
fn probe_symmetric_two_patch_bridge() {
    let a = finished(
        "a",
        brick((0.0, 3.0), (0.0, 1.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    let bridge_blank = finished(
        "bridge_blank",
        brick((0.0, 3.0), (0.0, 1.0), (1.0, 2.0), Tol::witness()),
        Tol::witness(),
    );
    let notch = finished(
        "notch",
        brick((1.0, 2.0), (-0.5, 1.5), (0.5, 1.5), Tol::witness()),
        Tol::witness(),
    );
    let BooleanResult::Body(b) = subtract(&bridge_blank, &notch, Tol::witness()).unwrap() else {
        panic!("bridge subtract yields a body");
    };
    let b = b.body;
    assert_eq!(
        mass_properties(
            &b,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        2.5
    );
    let g = glued(&a, &b, 5.5);
    // The hole-closing merges are real: ring-carrying faces exist and
    // their roles agree with the windings (tier-3 check 6 ran inside
    // `glued`; this keeps the shape from silently simplifying away).
    let ring_faces = g.body.faces().filter(|(_, f)| !f.rings.is_empty()).count();
    assert_eq!(
        ring_faces, 2,
        "front and back merged faces carry the window ring"
    );
}

/// PROBE 4 (review F5 → MINOR-1's counterexample): the three-wall
/// notch fill — B exactly plugs A's U-notch, interiors disjoint.
/// The ∖ reaches the containment fallback: every vertex of B lies on
/// A's boundary, and the midpoint of an edge along B's open side
/// (y = 2), such as (1.5, 2, 0), lies outside A, so B classifies Out at
/// the edge tier and A ∖ B is A. The
/// UNION of the same mate builds exactly (three-patch chain glue,
/// volume 6).
#[test]
fn probe_subtract_notch_rests_on_b() {
    let a = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 2.0),
            (2.0, 2.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        0.0,
        1.0,
        Tol::witness(),
    )
    .body;
    let a = finished("a", a, Tol::witness());
    assert_eq!(
        mass_properties(
            &a,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        5.0
    );
    let b = finished(
        "b",
        brick((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), Tol::witness()),
        Tol::witness(),
    );
    let BooleanResult::Body(sub) = subtract(&a, &b, Tol::witness())
        .expect("an edge along B's open side decides it lies outside A")
    else {
        panic!("A ∖ B keeps A's material");
    };
    assert_eq!(sub.kind, BooleanResultKind::OperandA);
    assert_eq!(
        mass_properties(
            &sub.body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness())
        )
        .unwrap()
        .met()
        .expect("the pinned resolution is met")
        .volume,
        5.0
    );
    glued(&a, &b, 6.0);
}

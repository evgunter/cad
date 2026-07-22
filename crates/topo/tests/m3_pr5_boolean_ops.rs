//! M3 PR 5 acceptance: the public boolean ops end to end. The
//! canonical two-brick trace (TOG Fig. 15.4 analogue — the F3 worked
//! example) with hand-derived censuses pinned bitwise (D9), the
//! Fig. 15.1 coplanar-overlap ∩, the A∖B ≡ A∩revert(B) oracle, voids,
//! disjoint/nested/touching operands, and the F7 merge output stage.
//! Every scenario is generic over `T` and runs at f64 (all ε rows via
//! CI) and on the interval lane.
//!
//! Tier-3 posture (the PR 3 gap, same documented posture): boolean
//! outputs carry chord-line descriptions on seam edges, so tier 3 at
//! rest runs through `upgrade_edges_to_intersections` (the review
//! helper posture) — the honest upgrade op is a PR 6 obligation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{prism_z, upgrade_edges_to_intersections};
use geom_core::Decide;
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind, mass_properties, subtract,
    union, validate, validate_closed, validate_geometric,
};

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// Census: (solids, shells, faces, loops, half-edges, edges, vertices).
fn census<T: geom_core::Real>(b: &Body<T>) -> (usize, usize, usize, usize, usize, usize, usize) {
    (
        b.solids().count(),
        b.shells().count(),
        b.faces().count(),
        b.loops().count(),
        b.half_edges().count(),
        b.edges().count(),
        b.vertices().count(),
    )
}

/// Runs one op functionally, checking the operands stayed bitwise
/// untouched and the result passes tier 1 + 2.
fn run<T: Decide>(
    op: fn(&Body<T>, &Body<T>) -> Result<BooleanResult<T>, BooleanError>,
    a: &Body<T>,
    b: &Body<T>,
) -> BooleanResult<T> {
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let out = op(a, b).unwrap();
    assert_eq!(format!("{a:?}"), a0, "operand A untouched");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched");
    if let BooleanResult::Body(body) = &out {
        assert_eq!(validate(&body.body), Ok(()), "tier 1");
        assert_eq!(validate_closed(&body.body), Ok(()), "tier 2");
    }
    out
}

fn body_of<T: Decide>(r: &BooleanResult<T>) -> &BooleanBody<T> {
    r.body().expect("non-empty boolean result")
}

/// Volume/area equality at f64 (exact values for the pinned corpus).
fn assert_props(body: &Body<f64>, volume: f64, area: f64) {
    let m = mass_properties(body).unwrap();
    assert_eq!(m.volume, volume, "exact volume");
    assert_eq!(m.surface_area, area, "exact area");
}

/// Tier-3 at rest via the documented description posture (module
/// docs).
fn assert_tier3_posture(body: &Body<f64>) {
    let mut upgraded = body.clone();
    upgrade_edges_to_intersections(&mut upgraded);
    assert_eq!(validate_geometric(&upgraded), Ok(()));
}

// ---------------------------------------------------------------
// Acceptance (1): the canonical two-brick trace, A=[0,2]³ B=[1,3]³.
// Seam = the staircase hexagon (2,1,1)-(2,2,1)-(1,2,1)-(1,2,2)-
// (1,1,2)-(2,1,2). Censuses hand-derived in the PR derivation.
// ---------------------------------------------------------------

fn two_bricks<T: Decide>() -> (Body<T>, Body<T>) {
    (
        brick::<T>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
        brick::<T>((1.0, 3.0), (1.0, 3.0), (1.0, 3.0)),
    )
}

#[test]
fn two_bricks_intersect() {
    let (a, b) = two_bricks::<f64>();
    let r = run(topo::intersect, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // The [1,2]³ cube: 3 A faces + 3 B faces, hexagon seam.
    assert_eq!(census(&body.body), (1, 1, 6, 6, 24, 12, 8));
    assert_props(&body.body, 1.0, 6.0);
    assert_tier3_posture(&body.body);
    // D9: byte-identical replay.
    let again = run(topo::intersect, &a, &b);
    assert_eq!(
        format!("{:?}", body.body),
        format!("{:?}", body_of(&again).body)
    );
}

#[test]
fn two_bricks_union() {
    let (a, b) = two_bricks::<f64>();
    let r = run(union, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // 7+7 operand corners + 6 seam; 3 full + 3 L faces per operand.
    assert_eq!(census(&body.body), (1, 1, 12, 12, 60, 30, 20));
    assert_props(&body.body, 15.0, 42.0);
    assert_tier3_posture(&body.body);
    let again = run(union, &a, &b);
    assert_eq!(
        format!("{:?}", body.body),
        format!("{:?}", body_of(&again).body)
    );
}

#[test]
fn two_bricks_subtract() {
    let (a, b) = two_bricks::<f64>();
    let r = run(subtract, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // A's union side (7 corners, 3 L + 3 full faces) + reverted BinA
    // (3 squares, corner (1,1,1), 3 split-edge remnants).
    assert_eq!(census(&body.body), (1, 1, 9, 9, 42, 21, 14));
    assert_props(&body.body, 7.0, 24.0);
    assert_tier3_posture(&body.body);
    let again = run(subtract, &a, &b);
    assert_eq!(
        format!("{:?}", body.body),
        format!("{:?}", body_of(&again).body)
    );
}

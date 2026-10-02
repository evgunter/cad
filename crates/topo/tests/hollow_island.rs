//! **Every island a solid of its own**: a boolean whose material falls
//! apart into a component standing inside a cavity of another files that
//! component as a solid of its own (`crates/topo/src/boolean/islands.rs`).
//!
//! Tier 3 admits the one-solid filing on its merits (check 10:
//! `+1 - 1 + 1 = 1` inside the island), so no gate catches a regression
//! here. Each row asserts the GROUPING directly: how many solids, which
//! shells each one holds in which order with their roles, and each
//! solid's volume.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::brick;
use geom_core::Tol;
use topo::{Body, BooleanBody, BooleanResult, BooleanResultKind, ShellRole};

fn tol() -> Tol {
    Tol::witness()
}

fn body_of(r: BooleanResult<f64>) -> BooleanBody<f64> {
    match r {
        BooleanResult::Body(b) => b,
        other => panic!("expected a body, got {other:?}"),
    }
}

fn cube(lo: f64, hi: f64) -> Body<f64> {
    brick((lo, hi), (lo, hi), (lo, hi), tol())
}

/// `cube(lo..hi)` with each `(lo, hi)` cube of `cavities` carved out.
fn hollow(lo: f64, hi: f64, cavities: &[(f64, f64)]) -> Body<f64> {
    cavities.iter().fold(cube(lo, hi), |body, &(a, b)| {
        body_of(topo::subtract(&body, &cube(a, b), tol()).expect("the cavity carves")).body
    })
}

/// One solid of a result: its shells' roles in the solid's own order,
/// and its volume (the sum of its shells' signed volumes).
#[derive(Debug)]
struct Solid {
    roles: Vec<ShellRole>,
    volume: f64,
}

/// Every solid of `body`, in arena order.
fn solids(body: &Body<f64>) -> Vec<Solid> {
    let classes = topo::classify_shells(body, tol()).expect("every shell classifies");
    body.solids()
        .map(|(solid, _)| {
            let shells = body.shells_of_solid(solid).expect("the solid resolves");
            let read: Vec<_> = shells
                .iter()
                .map(|s| {
                    let c = classes.iter().find(|c| c.shell == *s).expect("classified");
                    assert_eq!(c.solid, solid, "the shell's back-pointer names its solid");
                    (c.role, c.volume)
                })
                .collect();
            Solid {
                roles: read.iter().map(|r| r.0).collect(),
                volume: read.iter().map(|r| r.1).sum(),
            }
        })
        .collect()
}

/// Asserts `body`'s grouping is exactly `expected` — `(roles, volume)`
/// per solid, in arena order — and that tier 3 certifies it.
fn assert_grouping(body: &Body<f64>, expected: &[(&[ShellRole], f64)], what: &str) {
    let got = solids(body);
    assert_eq!(
        got.len(),
        expected.len(),
        "{what}: the number of solids ({got:?})"
    );
    for (i, (s, (roles, volume))) in got.iter().zip(expected).enumerate() {
        assert_eq!(&s.roles[..], *roles, "{what}: solid {i}'s shells and roles");
        assert!(
            (s.volume - volume).abs() < 1e-9,
            "{what}: solid {i}'s volume {} (expected {volume})",
            s.volume
        );
    }
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
}

/// Every B face the graft carried into the result still resolves, and
/// the solid each sits in is the one its shell's back-pointer names.
fn assert_graft_lineage(r: &BooleanBody<f64>, what: &str) {
    assert!(!r.naming.graft_faces.is_empty(), "{what}: B was grafted");
    for &(_, face) in &r.naming.graft_faces {
        let data = r
            .body
            .get_face(face)
            .unwrap_or_else(|| panic!("{what}: grafted face {face:?} resolves"));
        let shell = r.body.get_shell(data.shell).expect("its shell resolves");
        assert!(
            r.body
                .shells_of_solid(shell.solid)
                .expect("its solid resolves")
                .contains(&data.shell),
            "{what}: the face's solid lists its shell"
        );
    }
}

use ShellRole::{Outer, Void};

/// **The unit's own case**: `cube(0..6) ∖ shell(cube(2..4), 0.25)`. B's
/// cavity `(2.25..3.75)³` is material of the result standing inside the
/// void B's outer shell leaves, touching none of A's wall.
#[test]
fn subtract_of_a_hollow_operand_files_its_island_as_a_solid_of_its_own() {
    let b = topo::shell(&cube(2.0, 4.0), 0.25, tol())
        .expect("the small cube shells")
        .body;
    let r = body_of(topo::subtract(&cube(0.0, 6.0), &b, tol()).expect("the subtraction runs"));
    assert_eq!(r.kind, BooleanResultKind::Voided);
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 8.0), (&[Outer], 1.5f64.powi(3))],
        "hollow B",
    );
    assert_graft_lineage(&r, "hollow B");
}

/// **B with two cavities**: each cavity is an island of its own.
#[test]
fn each_cavity_of_a_hollow_operand_is_an_island_of_its_own() {
    let b = hollow(1.0, 5.0, &[(1.5, 2.5), (3.5, 4.5)]);
    let r = body_of(topo::subtract(&cube(0.0, 6.0), &b, tol()).expect("the subtraction runs"));
    assert_grouping(
        &r.body,
        &[
            (&[Outer, Void], 216.0 - 64.0),
            (&[Outer], 1.0),
            (&[Outer], 1.0),
        ],
        "two cavities",
    );
    assert_graft_lineage(&r, "two cavities");
}

/// **B with an island inside its own cavity** (one solid, grafted there
/// by the graft door): B's cavity becomes an island of the result, and
/// B's island becomes that island's void, filed with it.
#[test]
fn an_island_of_the_operand_lands_as_the_void_of_its_islands_solid() {
    let mut b = hollow(1.0, 5.0, &[(1.5, 4.5)]);
    let b_solid = b.solids().next().expect("one solid").0;
    topo::graft_disjoint_all_onto_keyed(&mut b, &[b_solid], &cube(2.5, 3.5))
        .expect("the island grafts into B's solid");
    let r = body_of(topo::subtract(&cube(0.0, 6.0), &b, tol()).expect("the subtraction runs"));
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 64.0), (&[Outer, Void], 27.0 - 1.0)],
        "B's own island",
    );
    assert_graft_lineage(&r, "B's own island");
}

/// **A's void inside B's cavity**: the void is A's shell, not B's, and it
/// moves with the island around it — read by nesting, not by operand.
#[test]
fn a_void_of_a_inside_bs_cavity_moves_with_the_island() {
    let a = hollow(0.0, 6.0, &[(2.5, 3.5)]);
    let b = hollow(1.5, 4.5, &[(2.0, 4.0)]);
    let r = body_of(topo::subtract(&a, &b, tol()).expect("the subtraction runs"));
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 27.0), (&[Outer, Void], 8.0 - 1.0)],
        "A's void in B's cavity",
    );
}

/// **∪ with B inside A's cavity**: B is an island of A's cavity, and a
/// hollow B keeps its own void.
#[test]
fn union_files_an_operand_inside_the_others_cavity_as_its_own_solid() {
    let a = hollow(0.0, 6.0, &[(1.0, 5.0)]);
    let r = body_of(topo::union(&a, &cube(2.0, 4.0), tol()).expect("the union runs"));
    assert_eq!(r.kind, BooleanResultKind::Assembly);
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 64.0), (&[Outer], 8.0)],
        "solid B in A's cavity",
    );

    let b = hollow(2.0, 4.0, &[(2.5, 3.5)]);
    let r = body_of(topo::union(&a, &b, tol()).expect("the union runs"));
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 64.0), (&[Outer, Void], 8.0 - 1.0)],
        "hollow B in A's cavity",
    );
}

/// **Side by side is not an island**: a disjoint union keeps its
/// components under one solid (the ruled assembly shape), hollow or not.
#[test]
fn a_disjoint_union_beside_a_hollow_operand_stays_one_solid() {
    let a = hollow(0.0, 6.0, &[(1.0, 5.0)]);
    let r = body_of(topo::union(&a, &cube(10.0, 12.0), tol()).expect("the union runs"));
    assert_eq!(r.kind, BooleanResultKind::Assembly);
    assert_grouping(
        &r.body,
        &[(&[Outer, Void, Outer], 216.0 - 64.0 + 8.0)],
        "side by side",
    );
}

/// **∩ on the same path**: A's cavity inside B keeps its place as the
/// void of the one solid the intersection leaves.
#[test]
fn intersect_of_a_hollow_operand_leaves_one_hollow_solid() {
    let a = hollow(0.0, 6.0, &[(1.0, 5.0)]);
    let r = body_of(topo::intersect(&a, &cube(0.5, 5.5), tol()).expect("the intersection runs"));
    assert_grouping(&r.body, &[(&[Void, Outer], 125.0 - 64.0)], "∩");
}

/// **The seamed path files islands too**: B crosses A's cavity wall, so
/// the two cavities merge, and B's own cavity, sitting in A's material,
/// stands inside the merged cavity as an island.
#[test]
fn a_seamed_subtract_files_the_island_inside_the_merged_cavity() {
    let a = hollow(0.0, 6.0, &[(2.0, 4.0)]);
    let b = hollow(3.0, 5.5, &[(4.5, 5.0)]);
    let r = body_of(topo::subtract(&a, &b, tol()).expect("the subtraction runs"));
    assert_eq!(r.kind, BooleanResultKind::Seamed);
    assert_grouping(
        &r.body,
        &[
            (&[Outer, Void], 216.0 - (8.0 + 2.5f64.powi(3) - 1.0)),
            (&[Outer], 0.125),
        ],
        "seamed",
    );
}

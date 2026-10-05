//! **A solid is one piece of material** (`docs/DESIGN.md`): every
//! boolean result is sorted into one solid per `Outer` shell, each
//! `Void` filed under the piece whose material surrounds it
//! (`crates/topo/src/pieces.rs`), and an operand may hold any number of
//! solids.
//!
//! Each row asserts the GROUPING directly: how many solids, which
//! shells each one holds in which order with their roles, and each
//! solid's volume — plus tier 3, which now refuses two `Outer` shells
//! under one solid.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished};
use geom_core::Tol;
use topo::{AtRestBody, Body, BooleanBody, BooleanResult, BooleanResultKind, ShellRole};

fn tol() -> Tol {
    Tol::witness()
}

fn body_of(r: BooleanResult<f64>) -> BooleanBody<f64> {
    match r {
        BooleanResult::Body(b) => b,
        other => panic!("expected a body, got {other:?}"),
    }
}

fn cube(lo: f64, hi: f64) -> AtRestBody<f64> {
    finished("cube", brick((lo, hi), (lo, hi), (lo, hi), tol()), tol())
}

/// The brick `x × y × z`, finished.
fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("block", brick(x, y, z, tol()), tol())
}

/// `cube(lo..hi)` with each `(lo, hi)` cube of `cavities` carved out.
fn hollow(lo: f64, hi: f64, cavities: &[(f64, f64)]) -> AtRestBody<f64> {
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

fn subtract(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    body_of(topo::subtract(a, b, tol()).expect("the subtraction runs"))
}

fn union(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> BooleanBody<f64> {
    body_of(topo::union(a, b, tol()).expect("the union runs"))
}

/// The unit's own operand pair's result: `cube(0..6) ∖ shell(cube(2..4),
/// 0.25)`.
fn hollow_b_result() -> BooleanBody<f64> {
    let b = finished(
        "the shelled cube",
        topo::shell(&cube(2.0, 4.0), 0.25, tol())
            .expect("the small cube shells")
            .body,
        tol(),
    );
    subtract(&cube(0.0, 6.0), &b)
}

/// `hollow(lo, hi, cavity)` with `cube(island)` standing in its cavity:
/// two pieces, so a body of two solids.
fn hollow_with_island(lo: f64, hi: f64, cavity: (f64, f64), island: (f64, f64)) -> AtRestBody<f64> {
    let r = union(&hollow(lo, hi, &[cavity]), &cube(island.0, island.1));
    assert_eq!(
        r.body.solids().count(),
        2,
        "the island is a piece of its own"
    );
    r.body
}

/// **The unit's own case**: `cube(0..6) ∖ shell(cube(2..4), 0.25)`. B's
/// cavity `(2.25..3.75)³` is material of the result standing inside the
/// void B's outer shell leaves, touching none of A's wall.
#[test]
fn subtract_of_a_hollow_operand_files_its_island_as_a_solid_of_its_own() {
    let r = hollow_b_result();
    assert_eq!(r.kind, BooleanResultKind::Voided);
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 8.0), (&[Outer], 1.5f64.powi(3))],
        "hollow B",
    );
    assert_graft_lineage(&r, "hollow B");
}

/// **That result is an operand.** The two-solid body the unit's case
/// leaves refused `JoinDesync` as a boolean operand before booleans took
/// bodies; now a cut far from the island (the containment fallback) and
/// a cut through A's wall (the seamed path) both build.
#[test]
fn the_two_solid_result_is_the_next_booleans_operand() {
    let r = hollow_b_result().body;
    let far = subtract(&r, &cube(0.5, 1.0));
    assert_grouping(
        &far.body,
        &[
            (&[Outer, Void, Void], 208.0 - 0.125),
            (&[Outer], 1.5f64.powi(3)),
        ],
        "a cavity cut far from the island",
    );
    let crossing = subtract(&r, &block((5.5, 6.5), (0.5, 1.5), (0.5, 1.5)));
    assert_eq!(crossing.kind, BooleanResultKind::Seamed);
    assert_grouping(
        &crossing.body,
        &[(&[Outer, Void], 208.0 - 0.5), (&[Outer], 1.5f64.powi(3))],
        "a notch through A's wall",
    );
}

/// **A channel from outside into the cavity, missing the island**: the
/// cavity opens to the outside and stops being a cavity, and the island
/// stays a solid of its own, now beside the wall rather than inside it.
#[test]
fn a_channel_into_the_cavity_leaves_the_island_its_own_solid() {
    let r = hollow_b_result().body;
    let channel = block((3.9, 6.5), (2.5, 3.5), (2.5, 3.5));
    let cut = subtract(&r, &channel);
    assert_grouping(
        &cut.body,
        &[(&[Outer], 208.0 - 2.0), (&[Outer], 1.5f64.powi(3))],
        "the opened cavity",
    );
}

/// **B with two cavities**: each cavity is an island of its own.
#[test]
fn each_cavity_of_a_hollow_operand_is_an_island_of_its_own() {
    let b = hollow(1.0, 5.0, &[(1.5, 2.5), (3.5, 4.5)]);
    let r = subtract(&cube(0.0, 6.0), &b);
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

/// **B with an island inside its own cavity** — B is a body of two
/// solids, and a boolean takes it: B's cavity becomes an island of the
/// result, and B's island becomes that island's void, filed with it.
#[test]
fn an_island_of_the_operand_lands_as_the_void_of_its_islands_solid() {
    let b = hollow_with_island(1.0, 5.0, (1.5, 4.5), (2.5, 3.5));
    let r = subtract(&cube(0.0, 6.0), &b);
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 64.0), (&[Outer, Void], 27.0 - 1.0)],
        "B's own island",
    );
    assert_graft_lineage(&r, "B's own island");
}

/// **An island inside a void of an island**: three pieces nested, and
/// each cavity filed under the piece around it.
#[test]
fn an_island_inside_a_void_of_an_island_is_a_third_solid() {
    let inner = hollow_with_island(2.0, 6.0, (2.5, 5.5), (3.5, 4.5));
    let r = union(&hollow(0.0, 8.0, &[(1.0, 7.0)]), &inner);
    assert_grouping(
        &r.body,
        &[
            (&[Outer, Void], 512.0 - 216.0),
            (&[Outer, Void], 64.0 - 27.0),
            (&[Outer], 1.0),
        ],
        "three nested pieces",
    );
}

/// **Two voids, each holding an island**: each island is its own solid,
/// and the wall keeps both cavities.
#[test]
fn two_voids_each_holding_an_island_give_three_solids() {
    let wall = hollow(0.0, 6.0, &[(0.5, 2.5), (3.5, 5.5)]);
    let r = union(&union(&wall, &cube(1.0, 2.0)).body, &cube(4.0, 5.0));
    assert_grouping(
        &r.body,
        &[
            (&[Outer, Void, Void], 216.0 - 16.0),
            (&[Outer], 1.0),
            (&[Outer], 1.0),
        ],
        "two islanded voids",
    );
}

/// **A single-operand result is sorted too**: subtracting a far cube
/// from a two-solid body answers with A's material, and A's island
/// stays a solid of its own.
#[test]
fn a_single_operand_result_keeps_its_island_a_solid() {
    let a = hollow_with_island(0.0, 6.0, (1.0, 5.0), (2.0, 4.0));
    let r = subtract(&a, &cube(10.0, 11.0));
    assert_eq!(r.kind, BooleanResultKind::OperandA);
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 64.0), (&[Outer], 8.0)],
        "OperandA",
    );
}

/// **A's void inside B's cavity**: the void is A's shell, not B's, and it
/// moves with the island around it — read by nesting, not by operand.
#[test]
fn a_void_of_a_inside_bs_cavity_moves_with_the_island() {
    let a = hollow(0.0, 6.0, &[(2.5, 3.5)]);
    let b = hollow(1.5, 4.5, &[(2.0, 4.0)]);
    let r = subtract(&a, &b);
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
    let r = union(&a, &cube(2.0, 4.0));
    assert_eq!(r.kind, BooleanResultKind::Assembly);
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 64.0), (&[Outer], 8.0)],
        "solid B in A's cavity",
    );

    let r = union(&a, &hollow(2.0, 4.0, &[(2.5, 3.5)]));
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 64.0), (&[Outer, Void], 8.0 - 1.0)],
        "hollow B in A's cavity",
    );
}

/// **Side by side is two pieces too**: a disjoint union is a body of two
/// solids, and that body is the next boolean's operand — a cut through
/// both pieces at once leaves both, each notched.
#[test]
fn a_disjoint_union_is_two_solids_and_an_operand() {
    let a = hollow(0.0, 6.0, &[(1.0, 5.0)]);
    let r = union(&a, &cube(10.0, 12.0));
    assert_eq!(r.kind, BooleanResultKind::Assembly);
    assert_grouping(
        &r.body,
        &[(&[Outer, Void], 216.0 - 64.0), (&[Outer], 8.0)],
        "side by side",
    );

    let pair = union(&cube(0.0, 1.0), &block((3.0, 4.0), (0.0, 1.0), (0.0, 1.0))).body;
    assert_grouping(&pair, &[(&[Outer], 1.0), (&[Outer], 1.0)], "two cubes");
    let notched = subtract(&pair, &block((0.5, 3.5), (0.25, 0.75), (0.5, 1.5)));
    assert_grouping(
        &notched.body,
        &[(&[Outer], 1.0 - 0.125), (&[Outer], 1.0 - 0.125)],
        "both pieces notched",
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

/// **The seamed path is sorted too**: B crosses A's cavity wall, so the
/// two cavities merge, and B's own cavity, sitting in A's material,
/// stands inside the merged cavity as an island.
#[test]
fn a_seamed_subtract_files_the_island_inside_the_merged_cavity() {
    let a = hollow(0.0, 6.0, &[(2.0, 4.0)]);
    let b = hollow(3.0, 5.5, &[(4.5, 5.0)]);
    let r = subtract(&a, &b);
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

/// A cube, a bar through it (neither's corners inside the other) and a
/// cavity inside both, filed under one solid (the `sweep-testing` merge
/// door): the cavity's two enclosers stand at one depth, so the shells
/// cross and no verb can tell which piece the cavity belongs to.
fn crossing_body() -> Body<f64> {
    let mut body = cube(0.0, 2.0).into_body();
    topo::graft_disjoint_all_keyed(
        &mut body,
        &brick((0.5, 1.5), (-1.0, 3.0), (0.5, 1.5), tol()),
    )
    .unwrap();
    topo::graft_disjoint_all_keyed(&mut body, &cube(0.8, 1.2).revert().unwrap()).unwrap();
    body.with_solids_merged_for_tests()
}

/// **Each verb refuses a body whose pieces cannot be read, typed**: the
/// boolean never takes it, because two outer shells under one solid is
/// not a finished body (the at-rest gate refuses `SolidOuterShells`); the
/// split (a plane missing the body, so one side is all of it) and `shell`
/// (which sorts its operand first) each refuse `Pieces(Crossing)`.
#[test]
fn every_verb_refuses_a_body_whose_pieces_cannot_be_read() {
    let body = crossing_body();
    refused_as_two_outer_shells("the crossing body", &body);
    let plane = topo::test_support::split_plane(
        geom_core::Point3::new(0.0, 0.0, 10.0),
        geom_core::Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    match topo::split(&body, &plane, tol()) {
        Err(topo::SplitError::Pieces(topo::PieceSortError::Crossing { .. })) => {}
        other => panic!("the split: {:?}", other.map(|_| ())),
    }
    match topo::shell(&body, 0.05, tol()) {
        Err(topo::ShellError::Pieces {
            error: topo::PieceSortError::Crossing { .. },
        }) => {}
        other => panic!("the shell: {:?}", other.map(|_| ())),
    }
}

/// **Overlapping material refuses rather than splitting into two solids
/// that overlap**: a cube filed straight inside another cube's solid, no
/// cavity between, is not a finished body, so the boolean never takes
/// it, and the split (a plane missing the body) refuses
/// `Pieces(Overlapping)`.
#[test]
fn a_cube_inside_a_cube_under_one_solid_refuses_as_overlapping() {
    let mut body = cube(0.0, 6.0).into_body();
    topo::graft_disjoint_all_keyed(&mut body, &cube(2.0, 4.0)).unwrap();
    let body = body.with_solids_merged_for_tests();
    refused_as_two_outer_shells("the overlapping body", &body);
    let plane = topo::test_support::split_plane(
        geom_core::Point3::new(0.0, 0.0, 10.0),
        geom_core::Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    match topo::split(&body, &plane, tol()) {
        Err(topo::SplitError::Pieces(topo::PieceSortError::Overlapping { .. })) => {}
        other => panic!("the split: {:?}", other.map(|_| ())),
    }
}

/// `body`'s one solid holds two outer shells, and the at-rest gate
/// refuses it on that alone.
fn refused_as_two_outer_shells(what: &str, body: &topo::Body<f64>) {
    let errors = topo::AtRestBody::validate(body.clone(), tol())
        .expect_err("two outer shells under one solid is not a finished body");
    assert!(
        matches!(
            errors.as_slice(),
            [topo::ValidationError::SolidOuterShells { outer: 2, .. }]
        ),
        "{what}: {errors:?}"
    );
}

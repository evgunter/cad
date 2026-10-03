//! Tier 3's check 10 through the public doors: a solid is one piece of
//! material — one `Outer` shell, and its shells bound winding number 0
//! or 1 everywhere.
//!
//! Every body here is built by kernel verbs — `subtract`, the graft
//! doors and `Body::move_shells_to_new_solid` — and, where a row needs
//! several pieces under one solid (a state no verb produces), by the
//! `sweep-testing` merge door `Body::with_solids_merged_for_tests`.
//!
//! Refused: two `Outer` shells under one solid, side by side, nested
//! with no `Void` between, or an island inside a `Void`
//! (`SolidOuterShells`); and, behind one `Outer`, a `Void` outside it
//! or a `Void` inside a `Void` (`ShellWinding`), each with a positive
//! per-solid total so check 7 passes it. Certified: an ordinary hollow
//! solid.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::brick;
use geom_core::Tol;
use topo::{Body, BooleanResult, ShellKey, ShellRole, SolidKey, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

fn cut(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    match topo::subtract(a, b, tol()).expect("the subtraction runs") {
        BooleanResult::Body(r) => r.body,
        other => panic!("expected a body, got {other:?}"),
    }
}

/// `brick(lo..hi)³` with the cube `(lo + wall)..(hi - wall)` carved out
/// of it: one solid, an `Outer` shell and a `Void`.
fn hollow(lo: f64, hi: f64, wall: f64) -> Body<f64> {
    cut(
        &brick((lo, hi), (lo, hi), (lo, hi), tol()),
        &brick(
            (lo + wall, hi - wall),
            (lo + wall, hi - wall),
            (lo + wall, hi - wall),
            tol(),
        ),
    )
}

fn only_solid(body: &Body<f64>) -> SolidKey {
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let [solid] = solids[..] else {
        panic!("expected one solid, got {}", solids.len())
    };
    solid
}

/// `body`'s shells of `role` under `solid`, read off their own signed
/// volumes — the premise each row asserts before it asserts a verdict.
fn shells_of(body: &Body<f64>, solid: SolidKey, role: ShellRole) -> Vec<ShellKey> {
    topo::classify_shells(body, tol())
        .expect("every shell classifies")
        .into_iter()
        .filter(|c| c.solid == solid && c.role == role)
        .map(|c| c.shell)
        .collect()
}

/// Grafts every solid of `src` into `dst` and files every shell under
/// `target`, `dst`'s first solid: several pieces under one solid.
fn graft_onto(dst: &mut Body<f64>, target: SolidKey, src: &Body<f64>) {
    topo::graft_disjoint_all_keyed(dst, src).expect("the graft");
    *dst = dst.with_solids_merged_for_tests();
    assert_eq!(
        dst.solids().map(|(k, _)| k).collect::<Vec<_>>(),
        vec![target],
        "the target is the first solid, and keeps every shell"
    );
}

fn positive_total(body: &Body<f64>) {
    assert!(
        topo::mass_properties(body, tol())
            .expect("the body measures")
            .volume
            > 0.0,
        "the premise: the total is positive, so check 7 alone would pass"
    );
}

// ---------------------------------------------------------------------
// The three violations.
// ---------------------------------------------------------------------

/// **A `Void` outside every `Outer` of its solid** — winding `-1` in
/// the cavity. A hollow cube's cavity keeps its solid while the cube's
/// own outer wall moves to a solid of its own, and a larger block far
/// away is grafted onto the solid so its total stays positive.
#[test]
fn a_void_outside_every_outer_refuses() {
    let mut body = hollow(0.0, 3.0, 1.0);
    let solid = only_solid(&body);
    let [wall] = shells_of(&body, solid, ShellRole::Outer)[..] else {
        panic!("a hollow cube has one outer wall")
    };
    let [cavity] = shells_of(&body, solid, ShellRole::Void)[..] else {
        panic!("and one cavity")
    };
    graft_onto(
        &mut body,
        solid,
        &brick((10.0, 16.0), (0.0, 6.0), (0.0, 6.0), tol()),
    );
    body.move_shells_to_new_solid(&[wall])
        .expect("the wall leaves");
    assert_eq!(shells_of(&body, solid, ShellRole::Outer).len(), 1);
    assert_eq!(shells_of(&body, solid, ShellRole::Void), vec![cavity]);
    positive_total(&body);

    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::ShellWinding {
            solid,
            shell: cavity,
            winding: 0,
            bounded: -1,
        }]),
        "the cavity sits where the solid's only other shell winds 0"
    );
    assert_eq!(
        topo::validate_geometric_structural(&body, tol()),
        topo::validate_geometric(&body, tol()),
        "the structural door makes check 10 too, through the closed form"
    );
}

/// **An `Outer` inside another `Outer` of its solid, no `Void`
/// between** — two pieces, so two solids' worth of shells under one.
#[test]
fn an_outer_inside_an_outer_refuses() {
    let mut body = brick((0.0, 3.0), (0.0, 3.0), (0.0, 3.0), tol());
    let solid = only_solid(&body);
    let inner = brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), tol());
    graft_onto(&mut body, solid, &inner);
    let outers = shells_of(&body, solid, ShellRole::Outer);
    assert_eq!(outers.len(), 2, "two Outer shells");
    assert!(
        shells_of(&body, solid, ShellRole::Void).is_empty(),
        "no Void"
    );
    positive_total(&body);

    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::SolidOuterShells { solid, outer: 2 }]),
    );
    assert_eq!(
        topo::validate_geometric_structural(&body, tol()),
        topo::validate_geometric(&body, tol()),
        "the structural door makes check 10 too, through the closed form"
    );
}

/// **A `Void` inside a `Void` of its solid, no `Outer` between** —
/// winding `-1` in the inner cavity. A hollow cube is filed into the
/// cavity of a larger one's solid (the island shape, two pieces), and
/// then the island's outer wall moves to a solid of its own, leaving its
/// cavity behind in the larger cube's.
#[test]
fn a_void_inside_a_void_refuses() {
    let mut body = hollow(0.0, 10.0, 1.0);
    let solid = only_solid(&body);
    graft_onto(&mut body, solid, &hollow(3.0, 7.0, 1.0));
    let small_wall = small(&body, &shells_of(&body, solid, ShellRole::Outer));
    let small_cavity = small(&body, &shells_of(&body, solid, ShellRole::Void));
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::SolidOuterShells { solid, outer: 2 }]),
        "before the move it is the island shape: two pieces under one solid"
    );
    body.move_shells_to_new_solid(&[small_wall])
        .expect("the island's wall leaves");
    positive_total(&body);

    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::ShellWinding {
            solid,
            shell: small_cavity,
            winding: 0,
            bounded: -1,
        }]),
        "the inner cavity sits where the wall and the outer cavity wind 1 - 1 = 0"
    );
}

/// The shell of `candidates` with the smaller enclosed volume.
fn small(body: &Body<f64>, candidates: &[ShellKey]) -> ShellKey {
    let classes = topo::classify_shells_of(body, candidates, tol()).expect("classifies");
    assert_eq!(classes.len(), 2, "two candidates");
    classes
        .iter()
        .min_by(|a, b| a.volume.abs().total_cmp(&b.volume.abs()))
        .expect("two candidates")
        .shell
}

// ---------------------------------------------------------------------
// Several pieces under one solid.
// ---------------------------------------------------------------------

/// **Several disjoint `Outer` shells under one solid** — two pieces:
/// the disjoint union is a body of two solids.
#[test]
fn disjoint_outer_shells_under_one_solid_refuse() {
    let mut body = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let solid = only_solid(&body);
    graft_onto(
        &mut body,
        solid,
        &brick((3.0, 4.0), (0.0, 1.0), (0.0, 1.0), tol()),
    );
    assert_eq!(shells_of(&body, solid, ShellRole::Outer).len(), 2);
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::SolidOuterShells { solid, outer: 2 }])
    );
}

/// **An `Outer` island inside a `Void` of its own solid** — the winding
/// admits it (`+1 - 1 + 1 = 1` inside the island), but the island
/// touches none of the wall around it, so it is a piece, and a solid,
/// of its own.
#[test]
fn an_island_inside_a_void_of_its_own_solid_refuses() {
    let mut body = hollow(0.0, 6.0, 1.0);
    let solid = only_solid(&body);
    graft_onto(
        &mut body,
        solid,
        &brick((2.0, 4.0), (2.0, 4.0), (2.0, 4.0), tol()),
    );
    assert_eq!(shells_of(&body, solid, ShellRole::Outer).len(), 2);
    assert_eq!(shells_of(&body, solid, ShellRole::Void).len(), 1);
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::SolidOuterShells { solid, outer: 2 }])
    );
}

/// **An ordinary hollow solid** — one `Outer`, one `Void` inside it.
#[test]
fn an_ordinary_hollow_solid_certifies() {
    let body = hollow(0.0, 3.0, 1.0);
    let solid = only_solid(&body);
    assert_eq!(shells_of(&body, solid, ShellRole::Outer).len(), 1);
    assert_eq!(shells_of(&body, solid, ShellRole::Void).len(), 1);
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
}

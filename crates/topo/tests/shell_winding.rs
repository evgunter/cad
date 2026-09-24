//! Tier 3's check 10 through the public doors: a solid's shells bound
//! winding number 0 or 1 everywhere.
//!
//! Every body here is built by kernel verbs — `subtract`, `shell`, the
//! graft doors and `Body::move_shells_to_new_solid` — never by hand, so
//! each row is a claim about bodies a caller can actually make.
//!
//! Three shapes refuse, each with a positive per-solid total so check 7
//! passes it: a `Void` outside every `Outer` of its solid, an `Outer`
//! inside another `Outer` with no `Void` between, and a `Void` inside a
//! `Void` with no `Outer` between. Three shapes certify: several
//! disjoint `Outer` shells under one solid, an `Outer` island inside a
//! `Void` of the same solid, and an ordinary hollow solid.

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

/// Grafts every solid of `src` onto `target`, so `src`'s shells join
/// that solid rather than minting their own.
fn graft_onto(dst: &mut Body<f64>, target: SolidKey, src: &Body<f64>) {
    let targets = vec![target; src.solids().count()];
    topo::graft_disjoint_all_onto_keyed(dst, &targets, src, tol()).expect("the graft");
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
}

/// **An `Outer` inside another `Outer` of its solid, no `Void`
/// between** — winding `2` inside the inner one. The onto door's own
/// contract says disjointness is the caller's; this caller breaks it.
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
    let enclosed = small(&body, &outers);
    positive_total(&body);

    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::ShellWinding {
            solid,
            shell: enclosed,
            winding: 1,
            bounded: 2,
        }]),
        "the inner cube sits where the outer one already winds 1"
    );
}

/// **A `Void` inside a `Void` of its solid, no `Outer` between** —
/// winding `-1` in the inner cavity. A hollow cube is grafted into the
/// cavity of a larger one (the valid island shape), and then the
/// island's outer wall moves to a solid of its own, leaving its cavity
/// behind in the larger cube's.
#[test]
fn a_void_inside_a_void_refuses() {
    let mut body = hollow(0.0, 10.0, 1.0);
    let solid = only_solid(&body);
    graft_onto(&mut body, solid, &hollow(3.0, 7.0, 1.0));
    let small_wall = small(&body, &shells_of(&body, solid, ShellRole::Outer));
    let small_cavity = small(&body, &shells_of(&body, solid, ShellRole::Void));
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Ok(()),
        "before the move it is the island shape, and valid"
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
// The three shapes that are NOT violations.
// ---------------------------------------------------------------------

/// **Several disjoint `Outer` shells under one solid** — the onto
/// door's product, which a union of separated bodies means in this
/// kernel.
#[test]
fn disjoint_outer_shells_under_one_solid_certify() {
    let mut body = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let solid = only_solid(&body);
    graft_onto(
        &mut body,
        solid,
        &brick((3.0, 4.0), (0.0, 1.0), (0.0, 1.0), tol()),
    );
    assert_eq!(shells_of(&body, solid, ShellRole::Outer).len(), 2);
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
}

/// **An `Outer` island inside a `Void` of its own solid** — the
/// hollow-operand subtraction's product: `+1 - 1 + 1 = 1` inside the
/// island, so the island is material and the body is valid. How the
/// island is GROUPED is the boolean's output convention, not an at-rest
/// invalidity.
#[test]
fn an_island_inside_a_void_of_its_own_solid_certifies() {
    let hollow_operand = topo::shell(
        &brick((2.0, 4.0), (2.0, 4.0), (2.0, 4.0), tol()),
        0.25,
        tol(),
    )
    .expect("the small box shells")
    .body;
    let body = cut(
        &brick((0.0, 6.0), (0.0, 6.0), (0.0, 6.0), tol()),
        &hollow_operand,
    );
    let solid = only_solid(&body);
    assert_eq!(shells_of(&body, solid, ShellRole::Outer).len(), 2);
    assert_eq!(shells_of(&body, solid, ShellRole::Void).len(), 1);
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
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

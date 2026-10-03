//! Tier 3's check 10 on CURVED shells, through the public doors: the
//! planar rows live in `topo`'s `shell_winding` suite, and these hold
//! the check to the sphere arm of the point-in-solid walk and to a
//! role read over curved faces, so a regression that silenced check 10
//! on curved shells could not stay green.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Tol, Vec3};
use sweep::test_support::ball_poled_z;
use topo::{Body, ShellKey, ShellRole, SolidKey, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

fn only_solid(body: &Body<f64>) -> SolidKey {
    let solids: Vec<SolidKey> = body.solids().map(|(k, _)| k).collect();
    let [solid] = solids[..] else {
        panic!("expected one solid, got {}", solids.len())
    };
    solid
}

/// **A ball inside a ball of the same solid, no `Void` between** —
/// winding `2` inside the small ball. The onto door's contract leaves
/// disjointness to the caller, and this caller nests instead.
#[test]
fn a_ball_inside_a_ball_of_its_own_solid_refuses() {
    let mut body = ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let solid = only_solid(&body);
    let small = ball_poled_z(0.5, Vec3::new(0.25, -0.125, 0.375), tol());
    topo::graft_disjoint_all_onto_keyed(&mut body, &[solid], &small).expect("the graft");

    let classes = topo::classify_shells(&body, tol()).expect("both balls classify");
    assert!(
        classes.iter().all(|c| c.role == ShellRole::Outer),
        "two Outer shells, no Void: {classes:?}"
    );
    let enclosed: ShellKey = classes
        .iter()
        .min_by(|a, b| a.volume.total_cmp(&b.volume))
        .expect("two shells")
        .shell;

    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::ShellWinding {
            solid,
            shell: enclosed,
            winding: 1,
            bounded: 2,
        }]),
        "the small ball sits where the large one already winds 1"
    );
}

/// **The same two balls, disjoint** — the onto door's own product, and
/// valid: the control that makes the row above about nesting and not
/// about curved shells under one solid.
#[test]
fn two_disjoint_balls_under_one_solid_certify() {
    let mut body = ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let solid = only_solid(&body);
    let beside = ball_poled_z(0.5, Vec3::new(5.0, -0.125, 0.375), tol());
    topo::graft_disjoint_all_onto_keyed(&mut body, &[solid], &beside).expect("the graft");
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()));
}

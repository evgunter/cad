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

/// `body` with every solid of `other` grafted in and every shell filed
/// under `body`'s first solid (the `sweep-testing` merge door: several
/// pieces under one solid, a state no verb produces).
fn merged(body: &Body<f64>, other: &Body<f64>) -> Body<f64> {
    let mut out = body.clone();
    topo::graft_disjoint_all_keyed(&mut out, other).expect("the graft");
    out.with_solids_merged_for_tests()
}

/// **A ball inside a ball of the same solid, no `Void` between** — two
/// pieces under one solid, which check 10 refuses by count, its role
/// read made over curved faces.
#[test]
fn a_ball_inside_a_ball_of_its_own_solid_refuses() {
    let big = ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let small = ball_poled_z(0.5, Vec3::new(0.25, -0.125, 0.375), tol());
    let body = merged(&big, &small);
    let solid = only_solid(&body);
    let classes =
        // Raw door: `body` is a solid filed with two outer shells, which tier 3 refuses (SolidOuterShells).
        topo::test_support::classify_unvalidated(&body, tol()).expect("both balls classify");
    assert!(
        classes.iter().all(|c| c.role == ShellRole::Outer),
        "two Outer shells, no Void: {classes:?}"
    );
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::SolidOuterShells { solid, outer: 2 }]),
    );
}

/// **A ball cavity inside another cavity of its solid** — every shell
/// but the outer one a `Void`, so the winding is read, over the sphere
/// arm of the point-in-solid walk: the inner cavity sits where the
/// outer ball and the first cavity together wind `0`. (A cavity outside
/// the outer ball is seen from infinity, where check 7's rays meet its
/// one reverted face from the material side and name the solid inside
/// out before the winding is read; inside the first cavity it is out of
/// every ray's reach.)
#[test]
fn a_ball_cavity_inside_another_cavity_refuses() {
    let big = ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let first = ball_poled_z(1.0, Vec3::new(0.0, 0.0, 0.0), tol())
        .revert()
        .unwrap();
    let cavity = ball_poled_z(0.25, Vec3::new(0.25, -0.125, 0.375), tol())
        .revert()
        .unwrap();
    let body = merged(&merged(&big, &first), &cavity);
    let solid = only_solid(&body);
    let classes =
        // Raw door: `body` is a shell wound against its role, which tier 3 refuses (ShellWinding).
        topo::test_support::classify_unvalidated(&body, tol()).expect("the three balls classify");
    // The inner cavity: the Void of least volume.
    let void: ShellKey = classes
        .iter()
        .filter(|c| c.role == ShellRole::Void)
        .min_by(|a, b| {
            a.enclosure
                .volume
                .abs()
                .total_cmp(&b.enclosure.volume.abs())
        })
        .expect("the inner Void")
        .shell;
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::ShellWinding {
            solid,
            shell: void,
            winding: 0,
            bounded: -1,
        }]),
        "the inner cavity sits where the ball and the first cavity wind 0"
    );
}

/// **The same two balls, disjoint, under one solid** — two pieces, so
/// they refuse by count too: the control that the count is not about
/// nesting.
#[test]
fn two_disjoint_balls_under_one_solid_refuse() {
    let big = ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let beside = ball_poled_z(0.5, Vec3::new(5.0, -0.125, 0.375), tol());
    let body = merged(&big, &beside);
    let solid = only_solid(&body);
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::SolidOuterShells { solid, outer: 2 }])
    );
}

/// **A ball cavity outside the ball around it is read inside out**: the
/// ball of radius 2 and a reverted ball of radius 0.5 beside it, merged
/// into one solid. Seen from infinity, check 7's rays meet the cavity's
/// one reverted sphere face from the material side, so the solid is
/// refused `NegativeVolume` before its shells' winding is read (where
/// two half-band faces per sphere read `ShellWinding`; the winding row
/// is [`a_ball_cavity_inside_another_cavity_refuses`]).
#[test]
fn a_ball_cavity_outside_its_solids_ball_is_read_inside_out() {
    let big = ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let cavity = ball_poled_z(0.5, Vec3::new(5.0, -0.125, 0.375), tol())
        .revert()
        .unwrap();
    let body = merged(&big, &cavity);
    let solid = only_solid(&body);
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::NegativeVolume { solid }]),
        "the cavity outside the ball reads the solid inside out"
    );
}

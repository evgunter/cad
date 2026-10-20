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
    let classes = topo::classify_shells(&body, tol()).expect("both balls classify");
    assert!(
        classes.iter().all(|c| c.role == ShellRole::Outer),
        "two Outer shells, no Void: {classes:?}"
    );
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::SolidOuterShells { solid, outer: 2 }]),
    );
}

/// **A ball cavity outside the ball around it** — the cavity sits where
/// the solid's only other shell winds `0`, and check 7's aimed rays read
/// it over the sphere arm of the ray walk.
#[test]
fn a_ball_cavity_outside_its_solids_ball_refuses() {
    let big = ball_poled_z(2.0, Vec3::new(0.0, 0.0, 0.0), tol());
    let cavity = ball_poled_z(0.5, Vec3::new(5.0, -0.125, 0.375), tol())
        .revert()
        .unwrap();
    let body = merged(&big, &cavity);
    let solid = only_solid(&body);
    let classes = topo::classify_shells(&body, tol()).expect("both balls classify");
    let void: ShellKey = classes
        .iter()
        .find(|c| c.role == ShellRole::Void)
        .expect("one Void")
        .shell;
    // Check 7 reads it before check 10 winds it: a ray aimed at the
    // cavity, which sits where the large ball winds 0, counts its way
    // into its material, and one aimed at the large ball counts its way
    // out.
    let read = topo::validate_geometric(&body, tol());
    assert!(
        matches!(
            read.as_ref().map_err(Vec::as_slice),
            Err([ValidationError::OrientationOpposed { solid: s, rays }])
                if *s == solid && rays.inside_out.shell == void && rays.outside.shell != void
        ),
        "the cavity sits where the large ball winds 0: {read:?}"
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

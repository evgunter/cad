//! Adversarial-review probes for the fillet naming work, sweep side.
//! They touch no naming API, so each runs unchanged at any revision:
//! X4 pins where the boolean refuses a filleted operand, and X4b —
//! the same operand moved off the die's own plane carriers — pins that
//! it no longer refuses for carrying sphere octants. (The printed
//! `Debug` fingerprint that used to live here is retired by SMELL
//! T-a's D104 and is not reinstated.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::blend::build::fillet_edges;
use sweep::test_support::brick;
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations};

/// The unit-side box with its low corner at `x0` on the x axis — the
/// `x0 + l` arithmetic done once, so a second placement cannot get it
/// wrong on its own.
fn box_at(x0: f64, l: f64) -> Body<f64> {
    brick((x0, x0 + l), (0.0, l), (0.0, l), Tol::witness())
}

fn filleted_die() -> Body<f64> {
    let cube0 = box_at(0.0, 1.0);
    let edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    fillet_edges(&cube0, &edges, 0.125, Tol::witness())
        .expect("the fillet")
        .body
}

/// X4: **a far box on the die's own plane carriers.** Each fillet
/// corner sphere is TANGENT to the flat faces it blends, and this far
/// box is axis-aligned on the same y and z ranges as the die, so its
/// plane carriers touch the corner spheres — at points on the die, far
/// from every face of the box. The extent scan asks the faces at a
/// touch, finds the touch out of the box's faces, and every op builds
/// in both operand orders against the rounded cube's closed form
/// `a³ + 6a²r + 3πar² + 4πr³/3` (`a = 1 − 2r`) and the box's 1.
#[test]
fn x4_disjoint_boolean_over_a_filleted_body_builds_past_the_plane_touches() {
    use core::f64::consts::PI;
    let a = filleted_die();
    let far = box_at(4.0, 1.0);
    let (r, side) = (0.125_f64, 0.75_f64);
    let v_die = side.powi(3)
        + 6.0 * side.powi(2) * r
        + 3.0 * PI * side * r * r
        + 4.0 / 3.0 * PI * r.powi(3);
    for (op, x, y, want) in [
        (BooleanOp::Union, &a, &far, Some(v_die + 1.0)),
        (BooleanOp::Union, &far, &a, Some(v_die + 1.0)),
        (BooleanOp::Intersect, &a, &far, None),
        (BooleanOp::Intersect, &far, &a, None),
        (BooleanOp::Subtract, &a, &far, Some(v_die)),
        (BooleanOp::Subtract, &far, &a, Some(1.0)),
    ] {
        let out = boolean_op_with(
            op,
            x,
            y,
            &BooleanDeclarations::none(),
            SweepStrategy::Realized,
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{op:?}: refused {e:?}"));
        match (out.body(), want) {
            (None, None) => {}
            (Some(b), Some(want)) => {
                let body = &b.body;
                assert_eq!(topo::validate(body), Ok(()), "{op:?}: tier 1");
                assert_eq!(topo::validate_closed(body), Ok(()), "{op:?}: tier 2");
                assert_eq!(
                    topo::validate_geometric(body, Tol::witness()),
                    Ok(()),
                    "{op:?}: tier 3"
                );
                let got = topo::mass_properties(body, Tol::witness()).unwrap().volume;
                assert!(
                    (got - want).abs() <= 1e-9 * want,
                    "{op:?}: {got} against the closed form {want}"
                );
            }
            (got, want) => panic!("{op:?}: {:?} against {want:?}", got.map(|_| "a body")),
        }
    }
}

/// X4b: the same filleted die and the same far box, translated OFF the
/// die's own plane carriers. Nothing is tangent to anything, and the
/// body carrying eight sphere OCTANTS assembles: two shells, volumes
/// add, tier-3 valid. This is the row X4's instruction asked for.
#[test]
fn x4b_a_filleted_body_assembles_with_an_operand_off_its_carriers() {
    let a = filleted_die();
    // Same far box, translated OFF the die's own plane carriers.
    let far = topo::transform_rigid(
        &box_at(4.0, 1.0),
        &geom_core::Affine3::translation(geom_core::Vec3::new(0.0, 2.0, 2.0)),
        Tol::witness(),
    )
    .unwrap();
    let out = boolean_op_with(
        BooleanOp::Union,
        &a,
        &far,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
        Tol::witness(),
    );
    let out = out.expect("a filleted body is an operand now");
    let body = &out.body().expect("a body").body;
    assert_eq!(
        topo::validate_geometric(body, Tol::witness()),
        Ok(()),
        "tier 3"
    );
    assert_eq!(body.shells().count(), 2, "the die and the box, disjoint");
    let want = topo::mass_properties(&a, Tol::witness()).unwrap().volume + 1.0;
    let got = topo::mass_properties(body, Tol::witness()).unwrap().volume;
    assert!(
        (got - want).abs() <= 1e-9 * want,
        "disjoint union adds volumes: {got} vs {want}"
    );
}

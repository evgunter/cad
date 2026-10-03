//! **The door-A rider** (M6 unit 1): the circle-carrier definite-miss
//! bound `bool_circle_curved_clearance` — the arm that retires M5's
//! UNCONDITIONAL conic-carrier pierce refusal (PR 12 fix pass F4
//! recorded the dishonesty: the reviewer measured 1.6 cm of true
//! clearance and the arm refused anyway).
//!
//! Three rows, the two-tolerance shape on the NEW arm (definite arms
//! included): definite miss (the strategies re-agree on disjoint
//! balls — the divergence `die_pips` documented is retired), definite
//! meet (handed to the circle × sphere roots, never cleared, and cut),
//! and an in-band clearance escalating through the funnel by name.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::Tol;
use geom_core::Vec3;
use sweep::test_support::ball_poled_y;
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, BooleanError};

fn union(a: &Body<f64>, b: &Body<f64>, strategy: SweepStrategy) -> Result<f64, BooleanError> {
    let out = boolean_op_with(
        BooleanOp::Union,
        a,
        b,
        &BooleanDeclarations::none(),
        strategy,
        Tol::witness(),
    )?;
    Ok(
        topo::mass_properties(&out.body().expect("a body").body, Tol::witness())
            .unwrap()
            .volume,
    )
}

/// **Definite miss, both strategies**: two far disjoint balls union
/// under Realized AND Idealized with bit-equal volumes. At M5 the
/// idealized path examined the circle×sphere pair and refused
/// unconditionally — the strategy divergence `die_pips`' module docs
/// recorded, retired exactly as predicted there.
#[test]
fn far_disjoint_balls_union_under_both_strategies() {
    let a = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 0.0), Tol::witness());
    let b = ball_poled_y(1.0, Vec3::new(7.0, 2.0, 0.0), Tol::witness());
    let vr = union(&a, &b, SweepStrategy::Realized).expect("realized: disjoint union");
    let vi = union(&a, &b, SweepStrategy::Idealized)
        .expect("idealized: the definite-miss certificate clears the examined pair");
    assert_eq!(vr.to_bits(), vi.to_bits(), "strategies agree to the bit");
    let want = 2.0 * 4.0 * PI / 3.0;
    assert!((vr - want).abs() <= 1e-9 * want);
}

/// **Definite meet reaches the section, not a guess**: genuinely
/// overlapping balls — the meridian circles straddle the other sphere,
/// no one-sided verdict exists, and the circle × sphere roots find the
/// crossings. The pair's centre line runs along X, across both charts'
/// polar axis (Y), so the section the join hands each side is tilted
/// against both charts; the run-side arc rule takes it and the union
/// meets the two-cap closed form.
#[test]
fn overlapping_balls_union_through_their_tilted_section() {
    let a = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 0.0), Tol::witness());
    let b = ball_poled_y(1.0, Vec3::new(3.2, 2.0, 0.0), Tol::witness());
    let v = union(&a, &b, SweepStrategy::Realized)
        .unwrap_or_else(|e| panic!("the tilted section is cut, got {e:?}"));
    // Two unit balls 1.2 apart share a lens of two caps of height 0.4.
    let h: f64 = 0.4;
    let want = 2.0 * 4.0 * PI / 3.0 - 2.0 * PI * h * h * (3.0 - h) / 3.0;
    assert!(
        (v - want).abs() <= 1e-9 * want,
        "union volume {v}, want {want}"
    );
}

/// **In-band clearance escalates by name** (two-tolerance, the F6
/// discipline): the gap between the balls sits strictly inside the
/// band, so neither "miss" nor "meet" may be asserted. The rider's
/// enclosure escalates first and hands the pair to the circle × sphere
/// roots, whose extreme residual — the same exact quantity — escalates
/// in its turn, so the name on the refusal is the roots'.
#[test]
fn in_band_clearance_escalates_through_the_funnel() {
    let tol = Tol::witness().get();
    let delta = 5.0 * tol.eps; // strictly inside [eps, K*eps)
    let a = ball_poled_y(1.0, Vec3::new(2.0, 2.0, 0.0), Tol::witness());
    let b = ball_poled_y(1.0, Vec3::new(4.0 + delta, 2.0, 0.0), Tol::witness());
    let err =
        union(&a, &b, SweepStrategy::Realized).expect_err("an in-band clearance cannot classify");
    match &err {
        BooleanError::Escalated { diag, .. } => {
            assert_eq!(
                diag.predicate,
                Some("bool_circle_sphere_extreme"),
                "the escalation names the roots' predicate: {diag:?}"
            );
        }
        other => panic!("expected an escalation, got {other:?}"),
    }
}

//! **The door-A rider** (M6 unit 1): the circle-carrier definite-miss
//! bound `bool_circle_curved_clearance` — the arm that retires M5's
//! UNCONDITIONAL conic-carrier pierce refusal (PR 12 fix pass F4
//! recorded the dishonesty: the reviewer measured 1.6 cm of true
//! clearance and the arm refused anyway).
//!
//! Three rows, the two-tolerance shape on the NEW arm (definite arms
//! included): definite miss (the strategies re-agree on disjoint
//! balls — the divergence `die_pips` documented is retired), definite
//! meet (the typed frontier stays for genuinely crossing circles),
//! and an in-band clearance escalating through the funnel by name.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use profile::RawLoop;

use geom_core::{Affine3, Point2, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, BooleanError};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A radius-`r` ball at `c` (poles on +Y; meridian circle in z = 0).
fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -r), 1.0),
        ProfileVertex::new(p2(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full, Tol::witness()).unwrap().body;
    topo::transform_rigid(&ball, &Affine3::translation(c), Tol::witness()).unwrap()
}

fn union(a: &Body<f64>, b: &Body<f64>, strategy: SweepStrategy) -> Result<f64, BooleanError> {
    let out = boolean_op_with(
        BooleanOp::Union,
        a,
        b,
        &BooleanDeclarations::none(),
        strategy,
        Tol::witness(),
    )?;
    Ok(topo::mass_properties(&out.body().expect("a body").body, Tol::witness())
        .unwrap()
        .volume)
}

/// **Definite miss, both strategies**: two far disjoint balls union
/// under Realized AND Idealized with bit-equal volumes. At M5 the
/// idealized path examined the circle×sphere pair and refused
/// unconditionally — the strategy divergence `die_pips`' module docs
/// recorded, retired exactly as predicted there.
#[test]
fn far_disjoint_balls_union_under_both_strategies() {
    let a = ball_at(1.0, Vec3::new(2.0, 2.0, 0.0));
    let b = ball_at(1.0, Vec3::new(7.0, 2.0, 0.0));
    let vr = union(&a, &b, SweepStrategy::Realized).expect("realized: disjoint union");
    let vi = union(&a, &b, SweepStrategy::Idealized)
        .expect("idealized: the definite-miss certificate clears the examined pair");
    assert_eq!(vr.to_bits(), vi.to_bits(), "strategies agree to the bit");
    let want = 2.0 * 4.0 * PI / 3.0;
    assert!((vr - want).abs() <= 1e-9 * want);
}

/// **Definite meet keeps the typed frontier**: genuinely overlapping
/// balls — the meridian circles straddle the other sphere, no
/// one-sided verdict exists, and the curved pierce door stays.
#[test]
fn overlapping_balls_keep_the_pierce_frontier() {
    let a = ball_at(1.0, Vec3::new(2.0, 2.0, 0.0));
    let b = ball_at(1.0, Vec3::new(3.2, 2.0, 0.0));
    let err = union(&a, &b, SweepStrategy::Realized)
        .expect_err("a crossing circle has no one-sided verdict");
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "expected the pierce frontier, got {err:?}"
    );
}

/// **In-band clearance escalates by name** (two-tolerance, the F6
/// discipline): the gap between the balls sits strictly inside the
/// band, so neither "miss" nor "meet" may be asserted.
#[test]
fn in_band_clearance_escalates_through_the_funnel() {
    let tol = Tol::witness().get();
    let delta = 5.0 * tol.eps; // strictly inside [eps, K*eps)
    let a = ball_at(1.0, Vec3::new(2.0, 2.0, 0.0));
    let b = ball_at(1.0, Vec3::new(4.0 + delta, 2.0, 0.0));
    let err =
        union(&a, &b, SweepStrategy::Realized).expect_err("an in-band clearance cannot classify");
    match &err {
        BooleanError::Escalated { diag } => {
            assert_eq!(
                diag.predicate,
                Some("bool_circle_curved_clearance"),
                "the escalation names the rider's predicate: {diag:?}"
            );
        }
        other => panic!("expected an escalation, got {other:?}"),
    }
}

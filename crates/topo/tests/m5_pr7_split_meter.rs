//! M5 PR 7: the rung-3 arm of `split_edge`'s parameter-interiority
//! meter (C12.3), and the boundary that keeps it unreachable at rest.
//!
//! `split_edge` states its interiority margin in metres by multiplying
//! a parameter distance by a per-carrier meter. PR 7 gave the `Nurbs`
//! arm a real one — `NurbsCurve3::speed_lower_bound`, a certified lower
//! bound on `‖C′(t)‖` from the derivative net's convex hull — replacing
//! a poison that could only escalate.
//!
//! **Why there is no end-to-end split row here.** Driving `split_edge`
//! needs an edge in a body, an edge in a body needs an `EdgeCurve`, and
//! an `EdgeCurve` exists only through `EdgeCurve::certify`, whose first
//! check refuses `Nurbs` carriers outright. So the arm is genuinely
//! unreachable from any at-rest body today — not untested, *not
//! constructible* — and the honest row is the one that pins exactly
//! that: the refusal that gates it, and the meter that is correct and
//! waiting behind it. M5 PR 9's curved-boolean zip is what mints the
//! first rung-3 edge at rest; when it flips that check, the end-to-end
//! split row belongs next to this one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::{EdgeCurve, EdgeCurveSpec};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3};
use geom_curves::{Curve3, NurbsCurve3};

/// A gentle cubic advancing steadily in `+x` — the shape a fitted SSI
/// carrier has.
fn nurbs_carrier() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=30)
        .map(|i| {
            let t = f64::from(i) / 30.0;
            Point3::new(t, 0.1 * (t * 4.0).sin(), 0.02 * t)
        })
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

#[test]
fn certification_still_refuses_a_nurbs_carrier_so_the_split_arm_is_unreachable() {
    let n = nurbs_carrier();
    let (p0, p1) = (n.eval(0.0), n.eval(1.0));
    // The description is irrelevant: check 1 rejects the carrier kind
    // before anything else runs.
    let chord = EdgeCurveSpec::line_between(p0, p1);
    let spec = EdgeCurveSpec {
        description: chord.description,
        carrier: Curve3::Nurbs(std::sync::Arc::new(n)),
        param_start: 0.0,
        param_end: 1.0,
    };
    let err = EdgeCurve::certify(spec, p0, p1, |_| None, Band::linear().unwrap())
        .expect_err("Nurbs carriers do not certify in this build");
    let msg = format!("{err}");
    assert!(
        msg.contains("Nurbs"),
        "the refusal must name the carrier kind: {msg}"
    );
}

#[test]
fn the_meter_the_split_arm_will_use_is_a_real_lower_bound() {
    // The arm itself: `split_edge` computes `(t − t₀) · meter` in
    // metres, so the meter must never exceed the true speed — a meter
    // that overstated it would accept a split that is NOT clear of the
    // endpoints, which is the failure the conservative posture exists
    // to prevent (`Circle` ⇒ radius, `Ellipse` ⇒ the MINOR semi-axis).
    let n = nurbs_carrier();
    let m = n.speed_lower_bound();
    assert!(m > 0.0, "a monotone carrier meters positively: {m}");
    for i in 0..=400 {
        let t = f64::from(i) / 400.0;
        assert!(
            n.deriv(t).norm() >= m - 1e-12,
            "meter {m} exceeds the real speed at t = {t}"
        );
    }
    // And the interiority margin the split gate would compute is a
    // genuine length: a parameter distance of 0.25 on this carrier is
    // at least `0.25 · m` metres of arc.
    let span = 0.25;
    let arc: f64 = (0..1000)
        .map(|i| {
            let a = span * f64::from(i) / 1000.0;
            let b = span * f64::from(i + 1) / 1000.0;
            (n.eval(b) - n.eval(a)).norm()
        })
        .sum();
    assert!(
        arc >= span * m - 1e-9,
        "the metered margin {} overstates the arc {arc}",
        span * m
    );
}

#[test]
fn a_rational_carrier_would_poison_the_meter_and_escalate() {
    // The convex-hull argument needs a non-rational derivative net.
    // A rational carrier reports poison, and `split_edge`'s trilean
    // then escalates rather than splitting on an invented scale.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    let c = NurbsCurve3::<f64>::new(kv, control, vec![1.0, 0.5, 1.0]).unwrap();
    assert!(c.speed_lower_bound().is_nan());
}

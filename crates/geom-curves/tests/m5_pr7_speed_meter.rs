//! M5 PR 7: the **certified speed meter** a rung-3 carrier needs at
//! `split_edge` (C12.3).
//!
//! `Body::split_edge` states its interiority margin in metres by
//! multiplying a parameter distance by a per-carrier meter: `1` for a
//! line, the radius for a circle, the **minor** semi-axis for an
//! ellipse — each a conservative lower bound on `‖dP/dt‖`. A fitted SSI
//! carrier had no such meter and poisoned; this is it, and the rows
//! below pin the three things a consumer relies on: it is a genuine
//! lower bound, it is honest (non-positive, not optimistic) when no
//! single direction bounds the curve, and a rational curve refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use geom_core::spline::KnotVector;
use geom_curves::NurbsCurve3;

#[test]
fn the_meter_lower_bounds_the_real_speed() {
    // A gently curved cubic advancing steadily in +x.
    let pts: Vec<Point3<f64>> = (0..=40)
        .map(|i| {
            let t = f64::from(i) / 40.0;
            Point3::new(t, 0.15 * (t * 6.0).sin(), 0.05 * t * t)
        })
        .collect();
    let c = NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap();
    let m = c.speed_lower_bound();
    assert!(
        m > 0.0,
        "a monotone carrier must have a positive meter: {m}"
    );
    // Sampled speeds must never fall below the certified meter.
    for i in 0..=200 {
        let t = f64::from(i) / 200.0;
        let s = c.deriv(t).norm();
        assert!(
            s >= m - 1e-12,
            "meter {m} exceeds the real speed {s} at t = {t}"
        );
    }
}

#[test]
fn a_carrier_that_doubles_back_reports_a_non_positive_meter() {
    // No single direction bounds this curve's velocity from below, so
    // the honest answer is "not positive" — and the caller's trilean
    // then escalates instead of accepting a split it cannot meter.
    let pts: Vec<Point3<f64>> = (0..=40)
        .map(|i| {
            let t = f64::from(i) / 40.0;
            let a = std::f64::consts::TAU * t;
            Point3::new(a.cos(), a.sin(), 0.0)
        })
        .collect();
    let c = NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap();
    assert!(
        c.speed_lower_bound() <= 0.0,
        "a closed loop has no positive one-directional speed bound"
    );
}

#[test]
fn a_rational_carrier_poisons_rather_than_claiming_a_bound() {
    // The convex-hull argument is about the CONTROL NET of a
    // non-rational derivative; a rational curve's derivative is not a
    // convex combination of any net, so no bound is claimed.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    let c = NurbsCurve3::<f64>::new(kv, control, vec![1.0, 0.5, 1.0]).unwrap();
    assert!(c.speed_lower_bound().is_nan());
}

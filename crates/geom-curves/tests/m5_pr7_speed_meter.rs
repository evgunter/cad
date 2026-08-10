//! M5 PR 7: the **certified speed meter** a rung-3 carrier needs at
//! `split_edge` (C12.3).
//!
//! `Body::split_edge` states its interiority margin in metres by
//! multiplying a parameter distance by a per-carrier meter: `1` for a
//! line, the radius for a circle, the **minor** semi-axis for an
//! ellipse — each a conservative lower bound on `‖dP/dt‖`. A fitted SSI
//! carrier had no such meter and poisoned; this is it, and the rows
//! below pin the three things a consumer relies on: it is a genuine
//! lower bound, it is honest (non-positive, not optimistic) when the
//! curve gives it nothing to stand on, and it is TOTAL — a structural
//! violation poisons rather than inventing a scale.
//!
//! **M7: rational carriers now state a real bound.** The original
//! third row asserted that a rational carrier poisons. It no longer
//! does: `speed_lower_bound` grew a second arm, a quotient-rule
//! assembly over the homogeneous control net (its derivation lives on
//! the method). The rows below re-derive that claim from scratch
//! rather than deleting it — real *and* sound on ordinary and
//! adversarial rational carriers, non-positive where the speed truly
//! collapses, poison where the structure is illegal, and one row that
//! states the conservative frontier out loud instead of hiding it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// `!(m > 0.0)` is deliberate everywhere below: the refusal rows accept
// EITHER a non-positive bound or poison, and `m <= 0.0` would let a NaN
// through as a pass. Same NaN-catching idiom as the kernel's own weight
// validation.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

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

/// Both halves of the contract at once: the bound is REAL (strictly
/// positive) and SOUND (never above the densely sampled true minimum
/// speed). Returns `bound / true_min` — how much the conservative
/// assembly gives away.
fn assert_real_and_sound(name: &str, c: &NurbsCurve3<f64>) -> f64 {
    let m = c.speed_lower_bound();
    assert!(m > 0.0, "{name}: the rational meter must be real, got {m}");
    let mut lo = f64::INFINITY;
    for i in 0..=4000 {
        let t = f64::from(i) / 4000.0;
        let s = c.deriv(t).norm();
        assert!(
            s >= m - 1e-12,
            "{name}: meter {m} exceeds the real speed {s} at t = {t}"
        );
        if s < lo {
            lo = s;
        }
    }
    m / lo
}

/// An exact quarter circle of radius `r` — the plainest rational
/// carrier the kernel actually mints (an arc's weights are the arc,
/// not an artifact to be removed).
fn rational_arc(r: f64) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let w = 0.5f64.sqrt();
    NurbsCurve3::new(
        kv,
        vec![
            Point3::new(r, 0.0, 0.0),
            Point3::new(r, r, 0.0),
            Point3::new(0.0, r, 0.0),
        ],
        vec![1.0, w, 1.0],
    )
    .unwrap()
}

/// A shallow quadratic net whose middle weight is the dial.
fn weighted_net(w1: f64) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    NurbsCurve3::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.5, 0.3, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![1.0, w1, 1.0],
    )
    .unwrap()
}

/// A degree-5 single-span net; the weights are the dial.
fn quintic_net(weights: Vec<f64>) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        5,
    )
    .unwrap();
    NurbsCurve3::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.2, 0.1, 0.0),
            Point3::new(0.4, -0.1, 0.05),
            Point3::new(0.6, 0.1, -0.05),
            Point3::new(0.8, -0.1, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        weights,
    )
    .unwrap()
}

#[test]
fn a_rational_carrier_states_a_real_lower_bound() {
    // FLIPPED from `a_rational_carrier_poisons_rather_than_claiming_a_
    // bound` (M5 PR 7 → M7). The convex-hull argument is still about
    // the control net of a NON-rational derivative, and a rational
    // curve's derivative is still not a convex combination of any net
    // — so the bound is no longer read off one. It is the quotient-rule
    // assembly documented at `speed_lower_bound`'s rational arm:
    // `C′ = (Ã′ − (C − c)·w′)/w` with `Ã = A − c·w`, every ingredient a
    // hull of HOMOGENEOUS coefficients over the span, divided by the
    // weight hull's MAXIMUM (a lower bound wants the largest
    // denominator, not the min-weight floor an upper bound wants).
    // The claim is the same one the integral arm makes, and it is
    // checked the same way: real, and never above the true speed.
    //
    // The exact curve the retired poison row used, first.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    let c = NurbsCurve3::<f64>::new(kv, control, vec![1.0, 0.5, 1.0]).unwrap();
    assert_real_and_sound("the retired poison row", &c);

    // And the carrier the kernel actually mints, at two scales — the
    // meter is a rate, so the ratio must not move with the radius.
    let big = assert_real_and_sound("quarter circle r = 1", &rational_arc(1.0));
    let small = assert_real_and_sound("quarter circle r = 0.05", &rational_arc(0.05));
    assert!(
        (big - small).abs() < 1e-9,
        "the bound must be scale-covariant: {big} vs {small}"
    );
}

#[test]
fn the_rational_meter_survives_adversarial_weights() {
    // Extreme ratios (0.01 … 100 against unit end weights), a weight a
    // hair off the non-positive boundary, and higher degrees. Every
    // row states a real, sound bound.
    for w1 in [0.01, 0.1, 10.0, 100.0, 1e-6] {
        assert_real_and_sound(&format!("w1 = {w1}"), &weighted_net(w1));
    }

    // Degree 3, two spans, weights spanning 0.5 … 3.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.25, 0.2, 0.1),
            Point3::new(0.5, -0.1, 0.0),
            Point3::new(0.75, 0.2, -0.1),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![1.0, 2.0, 0.5, 3.0, 1.0],
    )
    .unwrap();
    assert_real_and_sound("degree 3, two spans", &c);

    assert_real_and_sound("degree 5", &quintic_net(vec![1.0, 0.5, 2.0, 0.5, 2.0, 1.0]));
}

#[test]
fn a_rational_carrier_whose_speed_collapses_still_refuses() {
    // The honest half of the flip — the rational analogue of
    // `a_carrier_that_doubles_back_reports_a_non_positive_meter`. A
    // net with a REPEATED interior control point has a genuine cusp:
    // ‖C′‖ really does collapse, and no assembly may manufacture a
    // positive metre-per-parameter for it. The refusal machinery
    // (`nurbs_span_meter`, `split_edge_param_interior`) reads exactly
    // this and escalates.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 0.2, 0.0),
        ],
        vec![1.0, 0.6, 2.0, 1.0],
    )
    .unwrap();
    let m = c.speed_lower_bound();
    assert!(
        !(m > 0.0),
        "a cusped rational carrier has no positive speed bound, got {m}"
    );

    // And the exact turn-around: a net whose ends coincide really does
    // stop and reverse, so no positive rate exists to state.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let closed = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.5, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ],
        vec![1.0, 0.5, 1.0],
    )
    .unwrap();
    let m = closed.speed_lower_bound();
    assert!(
        !(m > 0.0),
        "an out-and-back rational carrier has no positive speed bound, got {m}"
    );
}

#[test]
fn the_conservative_frontier_is_stated_not_hidden() {
    // Where the assembly gives up: a high degree AND alternating
    // extreme weights leave the span hulls too coarse for a positive
    // answer even after the fixed refinement schedule. That is a
    // REFUSAL, never an unsound bound — pinned here so the frontier
    // moves visibly if the schedule constant is ever revisited.
    let c = quintic_net(vec![1.0, 0.01, 100.0, 0.01, 100.0, 1.0]);
    let m = c.speed_lower_bound();
    assert!(
        !(m > 0.0),
        "the stated frontier moved — degree 5 with alternating 0.01/100 weights now \
         meters at {m}. Good news, but re-derive this row (and the schedule constant \
         it pins) rather than deleting it."
    );
    // Sound even where it refuses: the answer never exceeds the truth.
    for i in 0..=200 {
        let t = f64::from(i) / 200.0;
        assert!(c.deriv(t).norm() >= m - 1e-12);
    }
}

#[test]
fn an_illegal_rational_carrier_is_still_poison() {
    // A non-positive weight breaks the convex-combination licence the
    // whole assembly stands on — every hull in it would be a lie. The
    // constructor refuses one outright, so the illegal net cannot even
    // be built, and the meter re-checks the same structure itself
    // rather than trusting that.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 0.3, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    assert!(NurbsCurve3::<f64>::new(kv, control, vec![1.0, -0.5, 1.0]).is_err());
}

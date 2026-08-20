//! R1 review probes for #327 (local to the review lane; not part of
//! the PR). Attacks on stage-1 curve recognition's certifying path.
// Lint allows widened (not rewritten) when this reviewer-authored
// probe module was adopted into the PR branch: the probes' substance is
// kept verbatim.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    clippy::float_cmp,
    clippy::single_match
)]

use crate::recognize_curve::{CurveRecognition, recognize};
use geom::{Curve3, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

const EPS_IN: f64 = 1e-9;

/// The exact 3x120 rational-quadratic full circle (the PR's own
/// fixture form), radius r, in the z = 0.25 plane.
fn rational_circle(radius: f64) -> NurbsCurve3<f64> {
    let tau = core::f64::consts::TAU;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for k in 0..7 {
        let on = k % 2 == 0;
        let theta = tau * (k as f64) / 6.0;
        let r = if on { radius } else { radius * 2.0 };
        control.push(Point3::new(r * theta.cos(), r * theta.sin(), 0.25));
        weights.push(if on { 1.0 } else { 0.5 });
    }
    let s = 3.0f64.sqrt();
    let knots = KnotVector::clamped(
        vec![
            0.0,
            0.0,
            0.0,
            s,
            s,
            2.0 * s,
            2.0 * s,
            3.0 * s,
            3.0 * s,
            3.0 * s,
        ],
        2,
    )
    .unwrap();
    NurbsCurve3::new(knots, control, weights).unwrap()
}

/// P1 — THE TURNING-WITNESS ATTACK. A closed carrier lying EXACTLY on
/// a circle that traverses 0 -> 120 deg -> 0 (out and back over the
/// same 120-degree arc, both spans exact rational-quadratic arcs of
/// the SAME circle, correct corner at 2r with weight 1/2 for a
/// 120-degree arc). Its image covers one third of the circle, so a
/// full-period promotion is a strictly larger locus and MUST refuse.
/// The five witness samples have azimuths 0, 60, 120, 60, 0 deg; the
/// two negative increments wrap to +300 deg and pass "strictly
/// positive", so if the witness is the only gate, this promotes.
#[test]
fn p1_on_circle_doubled_back_arc_must_not_promote() {
    let radius = 0.005f64;
    let z = 0.25;
    let on = |t: f64| Point3::new(radius * t.cos(), radius * t.sin(), z);
    // Correct tangent-intersection corner for a 120-degree arc:
    // r / cos(60 deg) = 2r, at the arc's mid-azimuth (60 deg).
    let corner = Point3::new(
        2.0 * radius * (core::f64::consts::TAU / 6.0).cos(),
        2.0 * radius * (core::f64::consts::TAU / 6.0).sin(),
        z,
    );
    let third = core::f64::consts::TAU / 3.0;
    let control = vec![on(0.0), corner, on(third), corner, on(0.0)];
    let weights = vec![1.0, 0.5, 1.0, 0.5, 1.0];
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
    let curve = NurbsCurve3::new(knots, control, weights).unwrap();
    // Sanity: every dense sample lies on the circle to ~1e-15.
    for k in 0..=200 {
        let p = curve.eval(2.0 * f64::from(k) / 200.0);
        let d = ((p.x * p.x + p.y * p.y).sqrt() - radius)
            .abs()
            .max((p.z - z).abs());
        assert!(d < 1e-12, "carrier must lie on the circle: {d:e} at {k}");
    }
    match recognize(&curve, EPS_IN) {
        CurveRecognition::Promoted { residual, .. } => panic!(
            "FALSE PROMOTION: a carrier covering one third of the circle \
             promoted to the full circle with certified residual {residual:e}"
        ),
        _ => {}
    }
}

/// P2 — dented circle, dent 10x eps: must stay NURBS.
#[test]
fn p2_dented_circle_above_eps_stays_nurbs() {
    let base = rational_circle(0.005);
    let mut control = base.control().to_vec();
    control[2] = control[2] + Vec3::new(1e-8, 0.0, 0.0);
    let curve = NurbsCurve3::new(base.knots().clone(), control, base.weights().to_vec()).unwrap();
    assert!(
        !matches!(recognize(&curve, EPS_IN), CurveRecognition::Promoted { .. }),
        "a 1e-8 dent must not certify at 1e-9"
    );
}

/// P3 — exact ellipses of small eccentricity (x scaled by 1+e): the
/// deviation from the best circle is ~r*e/2. Both must stay NURBS.
#[test]
fn p3_small_eccentricity_ellipses_stay_nurbs() {
    for e in [1e-5f64, 1e-6] {
        let base = rational_circle(0.005);
        let control: Vec<_> = base
            .control()
            .iter()
            .map(|p| Point3::new(p.x * (1.0 + e), p.y, p.z))
            .collect();
        let curve =
            NurbsCurve3::new(base.knots().clone(), control, base.weights().to_vec()).unwrap();
        assert!(
            !matches!(recognize(&curve, EPS_IN), CurveRecognition::Promoted { .. }),
            "an ellipse with a = r(1+{e:e}) must not certify at 1e-9 (dev ~{:e})",
            0.005 * e / 2.0
        );
    }
}

/// P4 — out-of-plane wobble at 10x eps: must stay NURBS.
#[test]
fn p4_out_of_plane_wobble_stays_nurbs() {
    let base = rational_circle(0.005);
    let control: Vec<_> = base
        .control()
        .iter()
        .enumerate()
        .map(|(k, p)| Point3::new(p.x, p.y, p.z + if k % 2 == 0 { 1e-8 } else { -1e-8 }))
        .collect();
    let curve = NurbsCurve3::new(base.knots().clone(), control, base.weights().to_vec()).unwrap();
    assert!(
        !matches!(recognize(&curve, EPS_IN), CurveRecognition::Promoted { .. }),
        "a 1e-8 out-of-plane wobble must not certify at 1e-9"
    );
}

/// P5 — certificate honesty on a promoting carrier: the reported
/// residual must dominate the measured deviation everywhere.
#[test]
fn p5_reported_residual_dominates_measured_deviation() {
    let base = rational_circle(0.005);
    let mut control = base.control().to_vec();
    // A sub-eps dent (5e-11): still certifies; residual must cover it.
    control[3] = control[3] + Vec3::new(0.0, 5e-11, 0.0);
    let curve = NurbsCurve3::new(base.knots().clone(), control, base.weights().to_vec()).unwrap();
    let CurveRecognition::Promoted {
        curve:
            Curve3::Circle {
                center,
                axis,
                radius,
                ..
            },
        residual,
        ..
    } = recognize(&curve, EPS_IN)
    else {
        panic!("a sub-eps dent must still certify");
    };
    let (a, b) = curve.domain();
    let mut worst = 0.0f64;
    for k in 0..=4096 {
        let p = curve.eval(a + (b - a) * f64::from(k) / 4096.0);
        let w = p - center;
        let h = w.dot(axis);
        let q = (w - axis * h).norm();
        worst = worst.max((q - radius).hypot(h));
    }
    assert!(
        worst <= residual * (1.0 + 1e-9) + 1e-18,
        "measured deviation {worst:e} exceeds certified residual {residual:e}"
    );
}

/// P6 — a double-wound closed carrier (two full turns, six 120-degree
/// spans covering the circle twice): documents the promotion outcome.
#[test]
fn p6_double_wound_circle_outcome() {
    let radius = 0.005f64;
    let tau = core::f64::consts::TAU;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for k in 0..13 {
        let on = k % 2 == 0;
        let theta = 2.0 * tau * (k as f64) / 12.0;
        let r = if on { radius } else { radius * 2.0 };
        control.push(Point3::new(r * theta.cos(), r * theta.sin(), 0.0));
        weights.push(if on { 1.0 } else { 0.5 });
    }
    let mut kv = vec![0.0, 0.0, 0.0];
    for k in 1..6 {
        kv.push(f64::from(k));
        kv.push(f64::from(k));
    }
    kv.extend([6.0, 6.0, 6.0]);
    let knots = KnotVector::clamped(kv, 2).unwrap();
    let curve = NurbsCurve3::new(knots, control, weights).unwrap();
    match recognize(&curve, EPS_IN) {
        CurveRecognition::Promoted { residual, .. } => {
            eprintln!("P6: double-wound circle PROMOTES (residual {residual:e})");
        }
        other => eprintln!("P6: double-wound circle does not promote: {other:?}"),
    }
}

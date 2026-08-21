//! ADVERSARIAL review e2e for M5 PR 3 (reviewer-authored, scratch, F9):
//! a real consumer program — build the exact rational-quadratic NURBS
//! circle, wrap it in `Curve3::Nurbs`, evaluate THROUGH THE ENUM ARMS
//! at f64 and Dual64, cross-check against the analytic
//! `Curve3::Circle`; then run the public certification pipeline on an
//! analytic edge (no regression) and confirm the pinned Nurbs refusal.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(missing_docs)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3};
use geom_brep::{CertifyError, EdgeCurve, EdgeCurveSpec};
use geom_core::spline::KnotVector;
use geom_core::{Band, Dual64, Point3, Vec3};
use geom_core::Tol;

const SQRT2_2: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// Unit circle in the xy-plane as the exact 9-point rational quadratic
/// (Book 7.3), parameterized on [0, 1] (one turn).
fn nurbs_unit_circle() -> NurbsCurve3<f64> {
    let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
    let control = vec![
        p(1.0, 0.0),
        p(1.0, 1.0),
        p(0.0, 1.0),
        p(-1.0, 1.0),
        p(-1.0, 0.0),
        p(-1.0, -1.0),
        p(0.0, -1.0),
        p(1.0, -1.0),
        p(1.0, 0.0),
    ];
    let weights = vec![1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0];
    let knots = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    NurbsCurve3::new(knots, control, weights).unwrap()
}

/// F9: the enum arms deliver the payload's geometry at f64 and Dual64,
/// agreeing with the analytic circle (reparameterized: NURBS t in
/// [0,1] maps to angle 2*pi*... NOT affinely — the rational quadratic's
/// parameter is not arc-angle — so compare LOCUS properties instead:
/// radius, plane, and the specific points at the knot parameters where
/// the parameterizations provably coincide).
#[test]
fn f9_enum_arm_nurbs_circle_locus() {
    let n = nurbs_unit_circle();
    let c: Curve3<f64> = Curve3::Nurbs(Arc::new(n));
    // Radius 1 everywhere on a dense schedule (locus property,
    // parameterization-independent).
    for i in 0..=400 {
        let t = i as f64 / 400.0;
        let p = c.eval(t);
        let r = (p.x * p.x + p.y * p.y).sqrt();
        assert!(
            (r - 1.0).abs() < 1e-12,
            "off the unit circle at t={t}: r={r}"
        );
        assert!(p.z.abs() < 1e-15, "left the plane at t={t}");
    }
    // Quarter points coincide with the analytic circle at the knots.
    let analytic = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    for (t, theta) in [
        (0.0, 0.0),
        (0.25, core::f64::consts::FRAC_PI_2),
        (0.5, core::f64::consts::PI),
        (0.75, 3.0 * core::f64::consts::FRAC_PI_2),
        (1.0, core::f64::consts::TAU),
    ] {
        let p = c.eval(t);
        let q = analytic.eval(theta);
        assert!(
            (p.x - q.x).abs() < 1e-12 && (p.y - q.y).abs() < 1e-12 && (p.z - q.z).abs() < 1e-12,
            "quarter point t={t}"
        );
    }
    // Dual64 through the ENUM arm (payload constant-lifted to Dual64):
    // derivative channel is tangent to the circle (perpendicular to the
    // radius vector) and matches deriv() at f64.
    let n64 = nurbs_unit_circle();
    let ctrl_d = n64
        .control()
        .iter()
        .map(|p| {
            Point3::new(
                Dual64::constant(p.x),
                Dual64::constant(p.y),
                Dual64::constant(p.z),
            )
        })
        .collect();
    let nd = geom::NurbsCurve3::new(n64.knots().clone(), ctrl_d, n64.weights().to_vec()).unwrap();
    let cd: Curve3<Dual64> = Curve3::Nurbs(Arc::new(nd));
    for t in [0.1, 0.4, 0.62, 0.9] {
        let d = cd.eval(Dual64::variable(t));
        let dv = c.deriv(t);
        assert!((d.x.deriv - dv.x).abs() < 1e-9 && (d.y.deriv - dv.y).abs() < 1e-9);
        let dot = d.x.value * d.x.deriv + d.y.value * d.y.deriv;
        assert!(
            dot.abs() < 1e-9,
            "tangent not perpendicular to radius at t={t}: {dot}"
        );
    }
    // deriv2 exists and is finite (curvature side-check: for the
    // rational parameterization kappa = |C' x C''| / |C'|^3 == 1/r = 1).
    for t in [0.05, 0.3, 0.55, 0.8] {
        let d1 = c.deriv(t);
        let d2 = c.deriv2(t);
        let cross = d1.cross(d2).norm();
        let kappa = cross / d1.norm().powi(3);
        assert!(
            (kappa - 1.0).abs() < 1e-9,
            "curvature wrong at t={t}: {kappa}"
        );
    }
}

/// F9: the public certification pipeline still certifies an analytic
/// edge (no regression from the Copy-loss ripple) and still REFUSES a
/// Nurbs carrier with the pinned typed error.
#[test]
fn f9_certification_pipeline_and_pinned_nurbs_refusal() {
    let band = Band::linear(Tol::witness()).unwrap();
    // Analytic line edge certifies.
    let p0 = Point3::new(0.25, -1.0, 2.0);
    let p1 = Point3::new(1.25, 0.5, 3.5);
    let spec = EdgeCurveSpec::line_between(p0, p1);
    let edge = EdgeCurve::certify(spec, p0, p1, |_| None, band).unwrap();
    // Arc-length parameterization: [0, |p1 - p0|]; midpoint at len/2.
    let mid = edge.carrier().eval(p0.distance(p1) * 0.5);
    assert!((mid.x - 0.75).abs() < 1e-12, "line midpoint x = {}", mid.x);
    // Nurbs carrier refuses with the typed Unimplemented, even though
    // the payload is now a real, evaluable curve.
    let mut nurbs_spec = EdgeCurveSpec::line_between(p0, p1);
    nurbs_spec.carrier = Curve3::Nurbs(Arc::new(nurbs_unit_circle()));
    let err = EdgeCurve::certify(nurbs_spec, p0, p1, |_| None, band).unwrap_err();
    assert!(
        matches!(err, CertifyError::Unimplemented),
        "refusal changed: {err:?}"
    );
    // The placeholder payload also refuses.
    let mut ph_spec = EdgeCurveSpec::line_between(p0, p1);
    ph_spec.carrier = Curve3::nurbs_placeholder();
    let err = EdgeCurve::certify(ph_spec, p0, p1, |_| None, band).unwrap_err();
    assert!(matches!(err, CertifyError::Unimplemented));
}

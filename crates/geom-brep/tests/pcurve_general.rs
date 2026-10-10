//! **U2's `General` arm** (PCURVE P-1a, spec item 2): the general
//! curve-in-UV certifying at the honest Fitted grade.
//!
//! `PcurveCache::certify_fitted` was callerless, and its own docs
//! named this arm as the waiting consumer. The rows here are the
//! ε-row for the new door — its three outcomes, each drawn rather
//! than asserted.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;
use std::sync::Arc;

use crate::shared::fixture;
use crate::shared::surf;
use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve2, NurbsCurve3, Surface};
use geom_brep::{FittedLane, Pcurve, PcurveCache, PcurveCertifyError};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3};

/// A rational quarter-cylinder wall of radius 1 about the z axis: the
/// `u = 0` boundary column is the ruling `x = 1, y = 0`.
fn quarter_cylinder_wall() -> Surface<f64> {
    Surface::Nurbs(Arc::new(fixture::quarter_cylinder_wall()))
}

/// The wall's `u = 0` ruling as a rung-3 carrier.
fn ruling() -> Curve3<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("knots");
    let n = NurbsCurve3::new(
        k,
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, 0.0, 1.0)],
        vec![1.0, 1.0],
    )
    .expect("the carrier builds");
    Curve3::Nurbs(Arc::new(n))
}

/// The chart image `(0, v)` as a general curve-in-UV.
fn image(u: f64) -> Arc<NurbsCurve2<f64>> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("image knots");
    Arc::new(
        NurbsCurve2::new(
            k,
            vec![Point2::new(u, 0.0), Point2::new(u, 1.0)],
            vec![1.0, 1.0],
        )
        .expect("the image builds"),
    )
}

/// The mate operand: a plane containing the ruling and meeting the
/// wall at 45°, so the pair's uniqueness tube has room to be
/// definitely transverse (a plane tangent along the ruling would be
/// the sliver the tube exists to refuse).
fn mate() -> Surface<f64> {
    let r = FRAC_1_SQRT_2;
    Surface::Plane {
        origin: Point3::new(1.0, 0.0, 0.0),
        normal: geom_core::Vec3::new(r, r, 0.0),
        u_ref: geom_core::Vec3::new(0.0, 0.0, 1.0),
    }
}

/// **ε-row, outcome ESCALATE**: a general image with no mate operand.
/// The uniqueness tube is a statement about the surface PAIR whose
/// intersection minted the carrier, so one surface cannot produce one
/// — the lane says so rather than certifying half a statement.
#[test]
fn general_without_a_mate_escalates_at_the_pair() {
    let got = PcurveCache::certify_general(
        image(0.0),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        None,
        band(),
        Some(FittedLane::certified()),
    );
    assert!(
        matches!(got, Err(PcurveCertifyError::FittedMateMissing)),
        "a general image owes the same pair statement as a fitted one: {got:?}"
    );
}

/// **ε-row, outcome REFUSE**: the wall's OTHER boundary column as the
/// claimed image of this ruling. It is a legal curve-in-UV on a legal
/// chart, and it is not this carrier's image — a definite, measured
/// failure, never an escalation.
#[test]
fn a_general_image_of_the_wrong_column_refuses_definitely() {
    let m = mate();
    let got = PcurveCache::certify_general(
        image(1.0),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        Some(&m),
        band(),
        Some(FittedLane::certified()),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::ResidualExceeded { .. }
                | PcurveCertifyError::FittedCertificate { .. })
        ),
        "the wrong column is a definite refusal, not an escalation: {got:?}"
    );
}

/// The closed-form door refuses a general image by kind: `General` is
/// the fitted GRADE's arm, and `certify` is the closed-form lane's
/// door. Two doors, one grade each — no arm is ever a catch-all for
/// the other.
#[test]
fn the_closed_form_door_refuses_a_general_image() {
    let got = PcurveCache::certify(
        Pcurve::General(image(0.0)),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        band(),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::ImageMismatch {
                image: geom_brep::PcurveKind::General,
                ..
            })
        ),
        "the closed-form door has no general arm: {got:?}"
    );
}

/// **An analytic chart holds no fitted-grade image.** Its image of a
/// carrier with no closed form is the projected one, so the general
/// door refuses a spline carrier's spline image on a cylinder chart by
/// kind, and a circle carrier on a sphere has no fitted class at all.
#[test]
fn an_analytic_chart_refuses_a_fitted_grade_image() {
    let cylinder = surf::cylinder::<f64>(1.0);
    let got = PcurveCache::certify_general(
        image(0.0),
        0.0,
        1.0,
        &ruling(),
        &cylinder,
        Some(&mate()),
        band(),
        Some(FittedLane::certified()),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::ImageMismatch {
                image: geom_brep::PcurveKind::Fitted,
                ..
            })
        ),
        "an analytic chart refuses the fitted grade by kind: {got:?}"
    );
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: geom_core::Vec3::new(0.6_f64.sin(), 0.0, 0.6_f64.cos()),
        radius: 1.0,
        u_ref: geom_core::Vec3::new(0.6_f64.cos(), 0.0, -0.6_f64.sin()),
    };
    let got = PcurveCache::certify_general(
        image(0.0),
        0.0,
        1.0,
        &circle,
        &surf::sphere::<f64>(1.0),
        None,
        band(),
        Some(FittedLane::certified()),
    );
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::UnsupportedCarrier {
                class: geom_brep::UncoveredClass::NoFittedClass,
                ..
            })
        ),
        "a circle has no fitted class: {got:?}"
    );
}

/// **The two fitted-grade doors run one check sequence**: the same
/// inputs through the general and the fitted door produce the same
/// certificate. `General` is the fitted GRADE, and the two doors differ
/// only in what their callers may assume, never in what the kernel
/// measured.
#[test]
fn the_general_and_fitted_doors_certify_alike() {
    let m = mate();
    let general = PcurveCache::certify_general(
        image(0.0),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        Some(&m),
        band(),
        Some(FittedLane::certified()),
    )
    .expect("the ruling certifies through the general door");
    assert!(
        matches!(general.pcurve(), Pcurve::General(_)),
        "the door stores the arm it was entered through"
    );
    let fitted = PcurveCache::certify_fitted(
        image(0.0),
        0.0,
        1.0,
        &ruling(),
        &quarter_cylinder_wall(),
        Some(&m),
        band(),
        FittedLane::certified(),
    )
    .expect("the same inputs certify through the fitted door");
    assert_eq!(
        format!("{:?}", general.certificate()),
        format!("{:?}", fitted.certificate()),
        "the two doors run one check sequence"
    );
}

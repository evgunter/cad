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
use crate::shared::fixture::wide_window as window;
use crate::shared::surf;
use crate::shared::tol::band;
use geom::{Curve3, NurbsCurve2, NurbsCurve3, Surface};
use geom_brep::{Pcurve, PcurveCache, PcurveCertifyError};
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
        window(),
        band(),
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
        window(),
        band(),
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
        window(),
        band(),
    );
    assert!(
        matches!(got, Err(PcurveCertifyError::UnsupportedCarrier)),
        "the closed-form door has no general arm: {got:?}"
    );
}

// ---- The certifying fixture: a GENERAL circle on a sphere chart —
// the tilted plane's section, whose chart image is transcendental in
// both channels and therefore has no closed-form lane anywhere. It is
// the class the fitted grade exists for, and #498's interior/diagonal
// loci are its named siblings. ----

/// The chart sphere: unit radius, polar axis +z.
fn sphere() -> Surface<f64> {
    surf::sphere(1.0)
}

const TILT: f64 = 0.6;

/// The mate: the tilted cutting plane whose section the circle is.
fn tilted_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: geom_core::Vec3::new(TILT.sin(), 0.0, TILT.cos()),
        u_ref: geom_core::Vec3::new(TILT.cos(), 0.0, -TILT.sin()),
    }
}

/// The general circle: neither a parallel nor a meridian of the chart.
fn general_circle() -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::origin(),
        axis: geom_core::Vec3::new(TILT.sin(), 0.0, TILT.cos()),
        radius: 1.0,
        u_ref: geom_core::Vec3::new(TILT.cos(), 0.0, -TILT.sin()),
    }
}

/// The traversed arc: a quarter turn away from the azimuth seam.
const ARC: (f64, f64) = (0.3, 0.3 + core::f64::consts::FRAC_PI_2);

/// The chart image, fitted at `f64` structure on the carrier's own
/// angle parameter — the parameter contract restored by an exact
/// affine knot rescale (a B-spline is invariant under one).
fn fit_image() -> Arc<NurbsCurve2<f64>> {
    let carrier = general_circle();
    let (t0, t1) = ARC;
    let n = 33usize;
    let mut params = Vec::with_capacity(n);
    let mut pts = Vec::with_capacity(n);
    let mut prev_u: Option<f64> = None;
    for i in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (i as f64 / (n - 1) as f64);
        let p = carrier.eval(t);
        let mut u = p.y.atan2(p.x);
        if let Some(pu) = prev_u {
            while u - pu > core::f64::consts::PI {
                u -= core::f64::consts::TAU;
            }
            while pu - u > core::f64::consts::PI {
                u += core::f64::consts::TAU;
            }
        }
        prev_u = Some(u);
        params.push((t - t0) / (t1 - t0));
        pts.push(Point2::new(u, p.z.asin()));
    }
    let fit = NurbsCurve2::interpolate_with_params(&pts, 3, &params).expect("the image fits");
    let knots: Vec<f64> = fit
        .knots()
        .knots()
        .iter()
        .map(|k| t0 + (t1 - t0) * k)
        .collect();
    let kv = KnotVector::clamped(knots, fit.knots().degree()).expect("affine knot rescale");
    Arc::new(
        NurbsCurve2::new(kv, fit.control().to_vec(), fit.weights().to_vec())
            .expect("rescaled image"),
    )
}

/// **ε-row, outcome CERTIFY**: the general circle's chart image on the
/// sphere, against the tilted plane it is the section of. Nothing
/// about the curve's provenance is asserted — the grade is what was
/// measured, which is the whole content of the arm.
#[test]
fn a_general_circle_image_certifies_at_the_fitted_grade() {
    let (t0, t1) = ARC;
    let img = fit_image();
    let carrier = general_circle();
    let w = Pcurve::General(Arc::clone(&img)).chart_box(t0, t1);
    let plane = tilted_plane();
    let cache = PcurveCache::certify_general(
        Arc::clone(&img),
        t0,
        t1,
        &carrier,
        &sphere(),
        Some(&plane),
        w,
        band(),
    )
    .expect("the general circle certifies through the general door");
    assert!(
        matches!(cache.pcurve(), Pcurve::General(_)),
        "the door stores the arm it was entered through"
    );
    // The SAME inputs through the fitted door produce the SAME
    // certificate: `General` is the fitted GRADE, and the two doors
    // differ only in what their callers may assume, never in what the
    // kernel measured.
    let twin =
        PcurveCache::certify_fitted(img, t0, t1, &carrier, &sphere(), Some(&plane), w, band())
            .expect("the same inputs certify through the fitted door");
    assert_eq!(
        format!("{:?}", cache.certificate()),
        format!("{:?}", twin.certificate()),
        "the two doors run one check sequence"
    );
}

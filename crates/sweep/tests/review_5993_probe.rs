//! Review probe (PR 5993, R2): near-perpendicular bars through the donut's
//! tube, the line × torus door's resolvent rung.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::framed_bar;
use crate::revolve_common::{axis_y, donut_profile, validated};
use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};

fn donut() -> topo::AtRestBody<f64> {
    let vp = validated(vec![donut_profile()]);
    let d = revolve(&vp, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body;
    finished("donut", d, Tol::witness())
}

fn tag(r: &Result<topo::BooleanResult<f64>, topo::BooleanError>) -> String {
    match r {
        Ok(_) => "OK".into(),
        Err(topo::BooleanError::ClassificationInvariant { what }) => format!("CI[{what}]"),
        Err(topo::BooleanError::Escalated { decision, diag }) => {
            format!("Esc[{decision:?} {:?}]", diag.predicate)
        }
        Err(e) => {
            let s = format!("{e:?}");
            s.split([' ', '{', '(']).next().unwrap().to_string()
        }
    }
}

#[test]
#[ignore = "review probe"]
fn review_5993_resolvent_probe() {
    let d = donut();
    let eps = Tol::witness().get().eps;
    for &delta in &[3e-3, 1e-3, 3e-4, 1e-4, 3e-5, 1e-5, 3e-6, 1e-6, 0.0] {
        for &phi in &[0.0_f64, 0.13, 0.4, 0.9] {
            for &y0 in &[0.0, 0.1, 0.27] {
                let dir = Vec3::new(phi.cos(), delta, phi.sin()).normalize();
                // through the tube centre circle at azimuth 1.1 rad
                let c = Point3::new(2.0 * 1.1_f64.cos(), y0, 2.0 * 1.1_f64.sin());
                let w = 1e-3_f64.max(200.0 * eps);
                let b = finished("bar", framed_bar(c, dir, -3.5, 0.2, w), Tol::witness());
                let r = topo::union(&d, &b, Tol::witness());
                println!(
                    "PROBE eps={eps:e} delta={delta:e} phi={phi} y0={y0} -> {}",
                    tag(&r)
                );
            }
        }
    }
}

/// The same near-perpendicular bars against a QUARTER donut (`x, z ≥ 0`
/// side of the turn), crossing the torus carrier at azimuth `π`, off the
/// face: the span's meetings with the carrier are all off the face.
#[test]
#[ignore = "review probe"]
fn review_5993_resolvent_off_face_probe() {
    let vp = validated(vec![donut_profile()]);
    let q = revolve(
        &vp,
        axis_y(),
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let q = finished("quarter donut", q, Tol::witness());
    let eps = Tol::witness().get().eps;
    for &delta in &[1e-3, 1e-4, 3e-5, 1e-5, 3e-6] {
        for &phi in &[0.0_f64, 0.13, 0.4] {
            for &y0 in &[0.0, 0.1] {
                let dir = Vec3::new(phi.cos(), delta, phi.sin()).normalize();
                let c = Point3::new(-2.0, y0, 0.0);
                let w = 1e-3_f64.max(200.0 * eps);
                let b = finished("bar", framed_bar(c, dir, -0.8, 0.8, w), Tol::witness());
                let r = topo::union(&q, &b, Tol::witness());
                println!(
                    "OFFFACE eps={eps:e} delta={delta:e} phi={phi} y0={y0} -> {}",
                    tag(&r)
                );
            }
        }
    }
}

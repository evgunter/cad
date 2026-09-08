//! **The sphere parse's span decide against certification AS RUN BY
//! `Body::mev`** (issue 1601): on the same `(t0, t1)`, rung by rung
//! around both band edges, the Euler door's verdict on the edge and
//! the parse's verdict on the hand-built loop agree — admit, escalate
//! (`ParamSpan` / `props_meridian_span_winding`) and refuse
//! (`WindingExceeded` / `NotIsoRectangle`) land on the same rungs, so
//! a span a certified door hands the parse is never refused there and
//! a span it refuses never gets an answer.
//!
//! Every offset is derived from the run's own `Band`: this file is on
//! CI's `eps ∈ {default, 1e-6, 1e-12}` matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_brep::certify::{CertCheck, CertifyError};
use geom_brep::props::{LoopEdge, PropsError, curved_face, require_one_chart_branch};
use geom_core::Tol;
use geom_core::{Band, Point3, Vec3};
use topo::{Body, EulerOpError, FaceSurface, MevSite};

const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;
/// The parse's winding decide.
const NAME: &str = "props_meridian_span_winding";

fn p3(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}
fn v3(x: f64, y: f64, z: f64) -> Vec3<f64> {
    Vec3::new(x, y, z)
}
fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: RS,
        axis: v3(0.0, 0.0, 1.0),
        u_ref: v3(1.0, 0.0, 0.0),
    }
}
/// The meridian great circle in the xz plane:
/// `P(t) = (R cos t, 0, R sin t)`.
fn meridian() -> Curve3<f64> {
    Curve3::Circle {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, -1.0, 0.0),
        radius: RS,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// One verdict on one span, read by name at either door.
#[derive(Debug, PartialEq)]
enum Disp {
    Admit,
    Escalate,
    Refuse,
    Other(String),
}

/// Certification through the Euler door: a lone `mev` on a sphere face
/// carrying the arc `[t0, t1]` of the meridian.
fn mev_disp(t0: f64, t1: f64) -> Disp {
    let tol = Tol::witness();
    let c = meridian();
    let a = c.eval(t0);
    let b = c.eval(t1);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(a).unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(sphere()))
        .unwrap();
    let r = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        b,
        EdgeCurveSpec::arc_of_circle(c, t0, t1).unwrap(),
        tol,
    );
    match r {
        Ok(_) => Disp::Admit,
        Err(EulerOpError::Certification {
            error: CertifyError::WindingExceeded,
        }) => Disp::Refuse,
        Err(EulerOpError::Certification {
            error:
                CertifyError::Escalated {
                    check: CertCheck::ParamSpan,
                    ..
                },
        }) => Disp::Escalate,
        Err(e) => Disp::Other(format!("{e:?}")),
    }
}
fn parse_disp<V: core::fmt::Debug>(r: &Result<V, PropsError>) -> Disp {
    match r {
        Ok(_) => Disp::Admit,
        Err(PropsError::Escalated { cause }) if cause.predicate == Some(NAME) => Disp::Escalate,
        Err(PropsError::NotIsoRectangle { what }) if *what == NAME => Disp::Refuse,
        Err(e) => Disp::Other(format!("{e:?}")),
    }
}

/// **`mev`'s certification and the parse agree rung by rung.** Spans
/// `τ + η` for `η` at `0`, inside the coincidence band, at `zero/R`
/// and `escalate/R` and `±k·ulp(τ)` around each, through the
/// ambiguity band and far past it. The flux lane's verdict is
/// certification's; the branch door's is certification's too, except
/// that on an admitted span it answers its own question — the arc
/// contains a pole (`NotOneChartBranch`).
#[test]
fn mev_certification_and_the_parse_agree_rung_by_rung() {
    let bd = Band::linear(Tol::witness()).unwrap();
    let z = bd.zero() / RS;
    let e = bd.escalate() / RS;
    let ulp = f64::EPSILON * TAU;
    let mut etas = vec![
        0.0,
        0.5 * z,
        0.99 * z,
        z,
        1.01 * z,
        0.5 * (z + e),
        0.99 * e,
        e,
        1.01 * e,
        10.0 * e,
        PI,
    ];
    for k in 1..=4 {
        for &edge in &[z, e] {
            etas.push(edge - f64::from(k) * ulp);
            etas.push(edge + f64::from(k) * ulp);
        }
    }
    let mut mismatches = Vec::new();
    for &eta in &etas {
        let t0 = 0.3;
        let t1 = t0 + TAU + eta;
        let cert = mev_disp(t0, t1);
        let edges = vec![
            LoopEdge::hand_built(meridian(), t0, t1, true, 0, 1),
            LoopEdge::hand_built(meridian(), t1, t0 + 4.0 * PI, true, 1, 0),
        ];
        let flux = parse_disp(&curved_face(&sphere(), &edges, 1.0, bd));
        let door = parse_disp(&require_one_chart_branch(&sphere(), &edges, bd));
        let door_agrees = match (&cert, &door) {
            (Disp::Admit, Disp::Other(s)) => s.contains("NotOneChartBranch"),
            (c, d) => c == d,
        };
        if cert != flux || !door_agrees {
            mismatches.push(format!(
                "eta/z={:.9}: mev {cert:?}, flux lane {flux:?}, branch door {door:?}",
                eta / z
            ));
        }
    }
    assert!(
        mismatches.is_empty(),
        "{} rungs:\n{}",
        etas.len(),
        mismatches.join("\n")
    );
}

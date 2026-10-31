//! **The certificate's `tangent_gap_hull_sup` charges the SUM of the
//! two surfaces' sags.** A unit sphere in a unit cylinder, its centre `δ`
//! off the axis, certified along the half circle halfway between them:
//! the separation is `−δ·cos t`, each residual half of it, so every
//! sample is within ε at `δ ≤ ε` and only the span line, through
//! `gap_sag`, refuses at the band's edge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::surf::arena2;
use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::{
    CertCheck, CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfacePair,
};
use geom_core::{Point3, Vec3};

fn certify_half_circle(
    delta: f64,
) -> (
    geom_brep::TangentSpanBounds<f64>,
    Result<EdgeCurve<f64>, CertifyError>,
) {
    let ball = Surface::Sphere {
        center: Point3::new(delta, 0.0, 0.0),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let bore = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let carrier = Curve3::Circle {
        center: Point3::new(0.5 * delta, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let pi = core::f64::consts::PI;
    let bounds = geom_brep::tangent_span_bounds(&ball, &bore, &carrier, 0.0, pi).unwrap();
    let (k1, k2, map) = arena2(ball, bore);
    let (p0, p1) = (carrier.eval(0.0), carrier.eval(pi));
    let spec = EdgeCurveSpec {
        description: EdgeDescriptionSpec::TangentIntersection {
            pair: SurfacePair::new(k1, k2),
            witness: carrier.eval(0.5 * pi),
        },
        carrier,
        param_start: 0.0,
        param_end: pi,
    };
    (
        bounds,
        EdgeCurve::certify(
            spec,
            p0,
            p1,
            |k| map.get(k).cloned(),
            1.0,
            band(),
            geom_brep::NurbsLane::certified(),
        ),
    )
}

#[test]
fn the_certificates_gap_hull_charges_the_sum_sag() {
    let eps = band().zero();
    let (bounds, got) = certify_half_circle(0.985 * eps);
    // The fixture's premise: the sum sag puts the span line past ε at
    // 0.985ε, the larger residual sag alone would not.
    assert!(0.985 * eps + bounds.gap_sag > eps, "{bounds:?}");
    assert!(0.985 * eps + bounds.residual_sag <= eps, "{bounds:?}");
    let (_, inside) = certify_half_circle(0.5 * eps);
    assert!(inside.is_ok(), "0.5ε certifies: {inside:?}");
    match got {
        Err(CertifyError::Escalated {
            check: CertCheck::TangentHull,
            ..
        }) => {}
        other => panic!("0.985ε: the gap hull refuses on the sag: {other:?}"),
    }
}

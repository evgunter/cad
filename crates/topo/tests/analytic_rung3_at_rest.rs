//! **A plane × cylinder rung-3 edge at rest states its plane side.** The
//! carrier is on the cylinder exactly and on the plane at every
//! schedule sample, and bumps off the plane between samples. The planar
//! face stores no pcurve row, so the between-samples statement against
//! the plane is the edge certificate's (`geom_brep::analytic_rung3`):
//! `mev` refuses the bumped carrier with the measurement and builds the
//! unbumped one, whose rows and edge tier 3 then reads clean. (Ported
//! from PR 4304's review probe.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::{AnalyticRung3Refusal, CertifyError, EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, EulerOpError};

/// The unit quarter circle in `z = 0` as one rational quadratic, its
/// knots refined so one control is supported strictly between the
/// schedule samples `0` and `1/8`, and that control lifted by `bump`.
fn bumped(bump: f64) -> NurbsCurve3<f64> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let q = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ],
        vec![1.0, h, 1.0],
    )
    .unwrap()
    .refine_knots(&[0.02, 0.04, 0.06, 0.08, 0.10])
    .unwrap();
    let mut ctl = q.control().to_vec();
    ctl[3].z += bump;
    NurbsCurve3::new(q.knots().clone(), ctl, q.weights().to_vec()).unwrap()
}

/// `mev` of the carrier between a cylinder face and a plane face.
fn build(c: &Arc<NurbsCurve3<f64>>) -> (Body<f64>, Result<topo::MevCreated, EulerOpError>) {
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let mut body = Body::<f64>::new();
    let (p0, p1) = (c.eval(0.0), c.eval(1.0));
    let seed = body.mvfs(p0, true).unwrap();
    let ck = body
        .set_face_surface(
            seed.face,
            topo::FaceSurface::New {
                surface: cylinder,
                sense: true,
            },
        )
        .unwrap();
    let anchor = body.mvfs(p1, true).unwrap();
    let pk = body
        .set_face_surface(
            anchor.face,
            topo::FaceSurface::New {
                surface: plane,
                sense: true,
            },
        )
        .unwrap();
    let made = body.mev(
        topo::MevSite::Lone {
            r#loop: seed.r#loop,
        },
        p1,
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                s1: ck,
                s2: pk,
                witness: c.eval(0.5),
            },
            carrier: Curve3::Nurbs(Arc::clone(c)),
            param_start: 0.0,
            param_end: 1.0,
        },
        Tol::witness(),
    );
    (body, made)
}

#[test]
fn a_carrier_bumped_off_the_plane_between_samples_refuses_at_mev() {
    let band = Band::linear(Tol::witness()).unwrap();
    for bump in [1e-4, 1e-2] {
        let c = Arc::new(bumped(bump));
        let at_samples = (0..=8)
            .map(|i| c.eval(f64::from(i) / 8.0).z.abs())
            .fold(0.0, f64::max);
        let between = (0..=2000)
            .map(|i| c.eval(f64::from(i) / 2000.0).z.abs())
            .fold(0.0, f64::max);
        assert!(
            at_samples <= band.zero() && between > 10.0 * band.zero(),
            "bump {bump:e}: on the plane at the samples ({at_samples:e}), off it between \
             ({between:e})"
        );
        let (_, made) = build(&c);
        assert!(
            matches!(
                &made,
                Err(EulerOpError::Certification {
                    error: CertifyError::AnalyticRung3(AnalyticRung3Refusal::Limb {
                        operand: geom::SurfaceKind::Plane,
                        margin,
                        ..
                    }),
                }) if upper(*margin) >= between
            ),
            "bump {bump:e} (off the plane by {between:e}): {made:?}"
        );
    }
    let flat = Arc::new(bumped(0.0));
    let (mut body, made) = build(&flat);
    made.expect("the unbumped quarter is on both faces between its samples");
    topo::mint_pcurves(&mut body, Tol::witness()).expect("the mint");
    let findings = topo::pcurves::validate_pcurves(&body, band);
    assert!(findings.is_empty(), "{findings:?}");
}

/// The refusal's bound as the classifier saw it: the point margin, or
/// the enclosure's upper end.
fn upper(margin: geom_core::MarginDiag) -> f64 {
    match margin.diagnostic_f64_for_error_text() {
        geom_core::ErrorTextReading::Value(m)
        | geom_core::ErrorTextReading::Enclosure { hi: m, .. } => m,
        geom_core::ErrorTextReading::Invalid => f64::NAN,
    }
}

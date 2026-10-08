#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
//! Review probe (PR 4304): a plane × cylinder rung-3 carrier that lies on
//! the cylinder exactly and on the plane at every schedule sample, but
//! bumps off the plane between samples.
use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::Body;

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
    .unwrap();
    let q = q.refine_knots(&[0.02, 0.04, 0.06, 0.08, 0.10]).unwrap();
    // N_3 is supported on [0.02, 0.08], strictly between samples 0 and 1/8.
    let mut ctl = q.control().to_vec();
    ctl[3].z += bump;
    NurbsCurve3::new(q.knots().clone(), ctl, q.weights().to_vec()).unwrap()
}

#[test]
fn probe_plane_limb_between_samples() {
    let band = Band::linear(Tol::witness()).unwrap();
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let cyl = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    for bump in [0.0, 1e-4, 1e-2] {
        let c = Arc::new(bumped(bump));
        let worst = (0..=2000)
            .map(|i| c.eval(f64::from(i) / 2000.0).z.abs())
            .fold(0.0, f64::max);
        let at_samples = (0..9)
            .map(|i| c.eval(f64::from(i) / 8.0).z.abs())
            .fold(0.0, f64::max);
        // The pre-PR engine of OnLocusHull: limbs 1, 2 and 3 on both operands.
        let all = geom_brep::ssi::certify_rung3(
            &c,
            None,
            &geom_brep::ssi::SsiOperand::Analytic(&plane),
            &geom_brep::ssi::SsiOperand::Analytic(&cyl),
            geom_brep::ssi::TubeScale::uniform(2.0),
            band,
        );
        let tube = geom_brep::rung3_tube(&c, &plane, &cyl, band);
        let mut body = Body::<f64>::new();
        let (p0, p1) = (c.eval(0.0), c.eval(1.0));
        let seed = body.mvfs(p0, true).unwrap();
        let ck = body
            .set_face_surface(seed.face, topo::FaceSurface::New { surface: cyl.clone(), sense: true })
            .unwrap();
        let anchor = body.mvfs(p1, true).unwrap();
        let pk = body
            .set_face_surface(anchor.face, topo::FaceSurface::New { surface: plane.clone(), sense: true })
            .unwrap();
        let made = body.mev(
            topo::MevSite::Lone { r#loop: seed.r#loop },
            p1,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection {
                    s1: ck,
                    s2: pk,
                    witness: c.eval(0.5),
                },
                carrier: Curve3::Nurbs(Arc::clone(&c)),
                param_start: 0.0,
                param_end: 1.0,
            },
            Tol::witness(),
        );
        let mint = made.as_ref().ok().map(|_| topo::mint_pcurves(&mut body, Tol::witness()));
        let findings = made.as_ref().ok().map(|_| topo::pcurves::validate_pcurves(&body, band));
        if let Ok(m) = made.as_ref() {
            let e = body.get_edge(m.edge).unwrap();
            for he in [e.he_plus, e.he_minus] {
                eprintln!("  row {:?}: {:?}", he, body.pcurve(he).map(|r| (format!("{:?}", r.pcurve()).chars().take(40).collect::<String>(), format!("{:?}", r.certificate()).chars().take(200).collect::<String>())));
            }
        }
        eprintln!(
            "bump {bump:e}: sup|z| {worst:e}, at samples {at_samples:e}\n  certify_rung3(All): {:?}\n  rung3_tube: {:?}\n  mev: {:?}\n  mint: {:?}\n  findings: {:?}",
            all.as_ref().map(|_| "ok").map_err(|e| format!("{e}")),
            tube.as_ref().map(|_| "ok").map_err(|e| format!("{e}")),
            made.as_ref().map(|_| "ok").map_err(|e| format!("{e}")),
            mint,
            findings,
        );
    }
}

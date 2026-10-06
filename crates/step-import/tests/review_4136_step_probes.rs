//! Reviewer probes for PR #4136: STEP export → import of slit-free
//! plane annuli (washer, flange, holed ring, a filleted washer).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use step_export::{StepOptions, step_string};
use step_import::{ImportOptions, StepImport, import_step};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;
use topo::readback::euler_counts;

fn tol() -> Tol {
    Tol::witness()
}

fn rev(loops: &[&[(f64, f64)]]) -> Body<f64> {
    let lps: Vec<ProfileLoop<f64>> = loops
        .iter()
        .map(|l| ProfileLoop::polygon(l.iter().map(|&(x, y)| Point2::new(x, y))))
        .collect();
    let vp = Profile::new(SketchPlane::<f64>::xy(), lps)
        .validate(tol())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&vp, axis, Revolution::Full, tol()).unwrap().body
}

fn census(b: &Body<f64>) -> (i64, i64, i64, i64) {
    let c = euler_counts(b);
    (c.v, c.e, c.f, c.r)
}

#[test]
fn slit_free_annuli_round_trip_through_step() {
    let washer = rev(&[&[(1., 0.), (2., 0.), (2., 1.), (1., 1.)]]);
    let flange = rev(&[&[(1., 0.), (3., 0.), (3., 0.5), (2., 0.5), (2., 2.), (1., 2.)]]);
    let ring = rev(&[
        &[(1., 0.), (4., 0.), (4., 3.), (1., 3.)],
        &[(2., 1.), (3., 1.), (3., 2.), (2., 2.)],
    ]);
    let filleted = {
        let bore: Vec<topo::EdgeKey> = washer
            .edges()
            .filter(|(_, e)| {
                let c = washer.get_curve_geom(e.curve).unwrap().certified().unwrap();
                matches!(*c.carrier(), geom::Curve3::Circle { center, radius, .. }
                    if (radius - 1.0).abs() < 1e-12 && center.y.abs() < 1e-12)
            })
            .map(|(k, _)| k)
            .collect();
        assert_eq!(bore.len(), 1);
        sweep::blend::build::fillet_edges(&washer, &bore, 0.2, tol())
            .unwrap()
            .body
    };
    for (what, body, want_v) in [
        ("washer", &washer, PI * 3.0),
        (
            "flange",
            &flange,
            PI * (9.0 - 1.0) * 0.5 + PI * (4.0 - 1.0) * 1.5,
        ),
        ("holed ring", &ring, PI * 15.0 * 3.0 - PI * 5.0),
        ("filleted washer", &filleted, f64::NAN),
    ] {
        let opts = StepOptions {
            product_name: what.to_owned(),
            uncertainty_m: Some(1e-9),
            ..StepOptions::default()
        };
        let text = match step_string(body, &opts, tol()) {
            Ok(t) => t,
            Err(e) => {
                println!("{what}: export refused {e:?}");
                continue;
            }
        };
        let bounds = text.matches("FACE_BOUND(").count();
        let outers = text.matches("FACE_OUTER_BOUND(").count();
        let back = import_step(&text, &ImportOptions::default(), tol())
            .unwrap_or_else(|e| panic!("{what}: import {e:?}"));
        let StepImport::Solid {
            body: b2,
            enclosure,
            ..
        } = back
        else {
            panic!("{what}: a solid")
        };
        let vol = enclosure.as_ref().map(|m| m.volume).unwrap_or(f64::NAN);
        let v_native = topo::mass_properties(body, tol()).unwrap().volume;
        println!(
            "{what}: native {:?} imported {:?}; FACE_OUTER_BOUND {outers} FACE_BOUND {bounds}; vol native {v_native:.12} imported {vol:.12} closed {want_v:.12}",
            census(body),
            census(&b2)
        );
        assert_eq!(
            census(body),
            census(&b2),
            "{what}: census survives the round trip"
        );
        assert!((vol - v_native).abs() < 1e-9, "{what}: volume survives");
        if want_v.is_finite() {
            assert!((v_native - want_v).abs() < 1e-9, "{what}: closed form");
        }
    }
}

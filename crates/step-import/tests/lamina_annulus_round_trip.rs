//! **A lamina full revolve's unslit plane annuli round-trip through
//! STEP**: each annulus exports as one face with a `FACE_OUTER_BOUND`
//! and its bore as a `FACE_BOUND`, and the import gives back the same
//! (V, E, F, R) census and the same volume — on the washer, the flange
//! and a washer whose bore rim is filleted.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, RawLoop as _, SketchPlane};
use step_export::{StepOptions, step_string};
use step_import::{ImportOptions, StepImport, import_step};
use sweep::{Revolution, RevolveAxis, revolve};
use topo::Body;
use topo::readback::euler_counts;

fn tol() -> Tol {
    Tol::witness()
}

fn rev(outline: &[(f64, f64)]) -> Body<f64> {
    let lp = ProfileLoop::polygon(outline.iter().map(|&(x, y)| Point2::new(x, y)));
    let vp = Profile::new(SketchPlane::<f64>::xy(), vec![lp])
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
fn unslit_plane_annuli_round_trip_through_step() {
    let washer = rev(&[(1., 0.), (2., 0.), (2., 1.), (1., 1.)]);
    let flange = rev(&[(1., 0.), (3., 0.), (3., 0.5), (2., 0.5), (2., 2.), (1., 2.)]);
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
        assert_eq!(bore.len(), 1, "the bottom bore rim is one edge");
        sweep::blend::build::fillet_edges(&washer, &bore, 0.2, tol())
            .unwrap()
            .body
    };
    for (what, body, annuli, closed_form) in [
        ("washer", &washer, 2, Some(PI * 3.0)),
        (
            "flange",
            &flange,
            3,
            Some(PI * (9.0 - 1.0) * 0.5 + PI * (4.0 - 1.0) * 1.5),
        ),
        ("filleted washer", &filleted, 2, None),
    ] {
        let opts = StepOptions {
            product_name: what.to_owned(),
            uncertainty_m: Some(1e-9),
            ..StepOptions::default()
        };
        let text = step_string(body, &opts, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        assert_eq!(
            text.matches("FACE_BOUND(").count(),
            annuli,
            "{what}: each annulus's bore is an inner bound"
        );
        let back = import_step(&text, &ImportOptions::default(), tol())
            .unwrap_or_else(|e| panic!("{what}: import {e:?}"));
        let StepImport::Solid {
            body: imported,
            enclosure,
            ..
        } = back
        else {
            panic!("{what}: a solid")
        };
        assert_eq!(
            census(body),
            census(&imported),
            "{what}: the census survives the round trip"
        );
        let native = topo::mass_properties(body, tol()).unwrap().volume;
        let read = enclosure.expect("the import's enclosure").volume;
        assert!(
            (read - native).abs() < 1e-9,
            "{what}: volume {read} vs {native}"
        );
        if let Some(want) = closed_form {
            assert!(
                (native - want).abs() < 1e-9,
                "{what}: {native} vs Pappus {want}"
            );
        }
    }
}

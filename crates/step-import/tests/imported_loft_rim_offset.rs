//! **An imported loft's cap rims are the one declared conventional
//! edge between distinct surfaces an import produces**, and the offset
//! door derives them when a cap move tilts them.
//!
//! The adoption ladder offers `Intersection` before the conventional
//! rung, but a plane × spline-wall `Intersection` certifies a SPLINE
//! carrier only, and a loft's rim arrives as the line it was swept
//! from. So the rung that adopts it is `MappedCurve`: an image in the
//! cap's chart, declared by the placed segment (`adopt.rs`'s
//! conventional rung, the NURBS-rim exemption). A cap offset of a loft
//! whose walls the move does not carry onto themselves — a twisted one
//! — re-derives that rim as the moved cap's section of its wall and
//! drops the record.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::test_support::bulge_loop;
use step_import::{ImportOptions, StepImport, import_step};

/// A loft between a square and the same square turned 0.3 rad about
/// its centre, one unit up: four bilinear saddle walls.
fn twisted_loft() -> topo::Body<f64> {
    let (s, c) = 0.3f64.sin_cos();
    let at = |x: f64, y: f64| (Point2::new(x, y), 0.0);
    let turned = |x: f64, y: f64| {
        let (dx, dy) = (x - 1.0, y - 1.0);
        at(1.0 + c * dx - s * dy, 1.0 + s * dx + c * dy)
    };
    let square = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let sections = [
        vec![bulge_loop(square.iter().map(|&(x, y)| at(x, y)).collect())],
        vec![bulge_loop(
            square.iter().map(|&(x, y)| turned(x, y)).collect(),
        )],
    ];
    let places = [0.0, 1.0].map(|z| Affine3::translation(Vec3::new(0.0, 0.0, z)));
    sweep::loft_body::<f64>(&sections, &places, 1, Tol::witness())
        .expect("the twisted loft builds")
        .body
}

/// The `z = 1` cap.
fn top_cap(body: &topo::Body<f64>) -> topo::FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z > 0.5 && origin.z > 0.5
            )
        })
        .map(|(k, _)| k)
        .expect("the loft has a top cap")
}

/// The certified curves of `face`'s boundary edges.
fn rims(body: &topo::Body<f64>, face: topo::FaceKey) -> Vec<geom_brep::EdgeCurve<f64>> {
    body.half_edges()
        .filter(|(he, _)| body.face_of_half_edge(*he) == Some(face))
        .map(|(_, h)| {
            let edge = body.get_edge(h.edge).unwrap();
            body.get_curve_geom(edge.curve)
                .and_then(topo::CurveGeom::certified)
                .unwrap()
                .clone()
        })
        .collect()
}

#[test]
fn an_imported_lofts_declared_cap_rims_are_derived_when_the_cap_moves() {
    let text = step_export::step_string(
        &twisted_loft(),
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the twisted loft exports");
    let mut body = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the twisted loft must re-import as a solid, got {other:?}"),
    };
    let cap = top_cap(&body);
    let cap_surface = body.get_face(cap).unwrap().surface;
    let before = rims(&body, cap);
    assert_eq!(before.len(), 4, "the cap has four rims");
    for rim in &before {
        assert!(
            matches!(rim.description(), geom_brep::EdgeDescription::Chart(c) if c.surface == cap_surface)
                && rim.authority().is_declared(),
            "an imported rim adopts as a declared image in the cap's chart: {:?}",
            rim.description()
        );
    }
    topo::replace_face_offset(&mut body, cap, -0.05, Tol::witness())
        .expect("the imported loft's cap moves inward");
    for rim in rims(&body, cap) {
        assert!(
            matches!(
                rim.description(),
                geom_brep::EdgeDescription::Intersection { .. }
            ) && !rim.authority().is_declared(),
            "the moved rim is the cap's section of its wall, its record dropped: {:?}",
            rim.description()
        );
    }
}

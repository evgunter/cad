//! **An imported loft's cap rims adopt as `Intersection`s, and the
//! offset door re-derives them when a cap move tilts them.**
//!
//! The adoption ladder offers `Intersection` before the conventional
//! rung, and a loft's rim arrives as the line it was swept from: the
//! plane × spline-wall certificate takes that line as its carrier, and
//! a straight rim on a bilinear saddle wall has a straight chart image,
//! so the rung certifies it. A cap offset of a loft whose walls the
//! move does not carry onto themselves — a twisted one — re-derives
//! each rim as the moved cap's section of its wall. (A rim the
//! `Intersection` rung refuses, a curved one whose degree-1 image the
//! hull refuses, still adopts as a declared image in the cap's chart;
//! `sweep`'s `offset_restates_a_neighbour_chart_rim` moves those.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use geom_core::{Point2, Tol};
use profile::SketchPlane;
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
    let places = [0.0, 1.0].map(|z| SketchPlane::xy_at(Point3::new(0.0, 0.0, z)));
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
fn an_imported_lofts_cap_rims_are_derived_when_the_cap_moves() {
    let text = step_export::step_string(
        &crate::common::at_rest(&twisted_loft()),
        &step_export::StepOptions::default(),
        Tol::witness(),
    )
    .expect("the twisted loft exports");
    let mut body = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body.into_body(),
        other => panic!("the twisted loft must re-import as a solid, got {other:?}"),
    };
    let cap = top_cap(&body);
    let cap_surface = body.get_face(cap).unwrap().surface;
    let before = rims(&body, cap);
    assert_eq!(before.len(), 4, "the cap has four rims");
    for rim in &before {
        assert!(
            matches!(
                rim.description(),
                geom_brep::EdgeDescription::Intersection { pair, .. } if pair.keys().contains(&cap_surface)
            ) && !rim.authority().is_declared(),
            "an imported rim adopts as the cap's Intersection with its wall: {:?}",
            rim.description()
        );
    }
    // Each rim's height at its parameter midpoint: the cap's `z = 1`
    // before the move, the moved cap's `z = 0.95` after it.
    let mid_z = |rim: &geom_brep::EdgeCurve<f64>| {
        let (t0, t1) = rim.params();
        rim.carrier().eval(0.5 * (t0 + t1)).z
    };
    let on = |z: f64, want: f64| (z - want).abs() <= Tol::witness().get().eps;
    assert!(
        before.iter().all(|r| on(mid_z(r), 1.0)),
        "the imported rims lie on the z = 1 cap: {:?}",
        before.iter().map(mid_z).collect::<Vec<_>>()
    );
    topo::replace_face_offset(&mut body, cap, -0.05, Tol::witness())
        .expect("the imported loft's cap moves inward");
    let moved = body.get_face(cap).unwrap().surface;
    assert_ne!(
        moved, cap_surface,
        "the moved cap is a new surface, not the imported one"
    );
    let after = rims(&body, cap);
    assert_eq!(after.len(), 4, "the moved cap keeps its four rims");
    for rim in &after {
        assert!(
            matches!(
                rim.description(),
                geom_brep::EdgeDescription::Intersection { pair, .. } if pair.keys().contains(&moved)
            ) && !rim.authority().is_declared()
                && on(mid_z(rim), 0.95),
            "the moved rim is the moved cap's section of its wall, at z = 0.95: {:?} at z = {}",
            rim.description(),
            mid_z(rim)
        );
    }
}

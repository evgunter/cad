//! Issue #111, export end: the A×Z intersect must reach a real STL.
//!
//! `check_mesh` was the only thing between the planar CDT's exterior
//! needle triangle (see `mesh::planar`) and a silently leaky STL —
//! nothing downstream of the mesh re-checks watertightness. This pins
//! the whole display pipeline for the class: boolean → tessellate →
//! check_mesh → ASCII and binary STL, with the facet count agreeing
//! across the two encodings and the binary header's declared count.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Vec3};
use mesh::tessellate;
use mesh::validate::{check_mesh, triangle_count};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane, ValidatedProfile};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanResult};
use geom_core::Tol;

const A_OUTLINE: [(f64, f64); 8] = [
    (0.0, 0.0),
    (0.625, 0.0),
    (0.8125, 1.0),
    (1.1875, 1.0),
    (1.375, 0.0),
    (2.0, 0.0),
    (1.125, 2.5),
    (0.875, 2.5),
];

const A_COUNTER: [(f64, f64); 3] = [(0.90625, 1.4375), (1.09375, 1.4375), (1.0, 2.0)];

fn lp(poly: &[(f64, f64)]) -> ProfileLoop<f64> {
    ProfileLoop::polygon(
        poly.iter()
            .map(|&(x, y)| Point2::new(x, y))
            .collect::<Vec<_>>(),
    )
}

fn validated(plane: SketchPlane<f64>, loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(plane, loops)
        .validate(Tol::witness())
        .unwrap()
}

/// A (counter-hole variant) × Z, the #93 acceptance body.
fn az_counter() -> Body<f64> {
    let a = extrude(
        &validated(
            SketchPlane::from_frame(
                Point3::new(0.0, 0.0, -0.0625),
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ),
            vec![lp(&A_OUTLINE), lp(&A_COUNTER)],
        ),
        Extrusion::Distance(2.125),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let z = extrude(
        &validated(
            SketchPlane::from_frame(
                Point3::new(-0.0625, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            vec![lp(&[
                (-0.0625, 0.0),
                (2.5625, 0.0),
                (2.5625, 0.4375),
                (0.6875, 0.4375),
                (2.5625, 1.5625),
                (2.5625, 2.0),
                (-0.0625, 2.0),
                (-0.0625, 1.5625),
                (1.8125, 1.5625),
                (-0.0625, 0.4375),
            ])],
        ),
        Extrusion::Distance(2.125),
        Tol::witness(),
    )
    .unwrap()
    .body;
    match topo::intersect(&a, &z, Tol::witness()) {
        Ok(BooleanResult::Body(bb)) => bb.body,
        other => panic!("A×Z intersect did not produce a body ({other:?})"),
    }
}

#[test]
fn survives_az_intersect_exports_watertight_stl() {
    let mesh = tessellate(&az_counter(), 1e-2, Tol::witness()).expect("tessellate A×Z");
    assert_eq!(
        check_mesh(&mesh),
        Ok(()),
        "REGRESSION (issue #111): A×Z tessellated non-watertight — a leaky STL"
    );

    let facets = triangle_count(&mesh);
    assert!(facets > 0, "empty mesh");

    let mut ascii = Vec::new();
    stl::write_ascii(&mesh, &stl::AsciiOptions::default(), &mut ascii).expect("ascii STL");
    let text = String::from_utf8(ascii).expect("ascii STL is UTF-8");
    assert_eq!(
        text.matches("facet normal").count(),
        facets,
        "ASCII facet count disagrees with the mesh"
    );

    let mut bin = Vec::new();
    stl::write_binary(&mesh, &stl::BinaryOptions::default(), &mut bin).expect("binary STL");
    assert_eq!(bin.len(), 84 + 50 * facets, "binary STL length");
    let declared = u32::from_le_bytes([bin[80], bin[81], bin[82], bin[83]]);
    assert_eq!(declared as usize, facets, "binary header facet count");
}

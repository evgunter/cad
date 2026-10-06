//! **The whole torus adopts as one face** (D1's wrap edge): FreeCAD's
//! `makeTorus(1, 0.25)` states the torus as ONE face whose loop is the
//! fundamental-polygon square, a meridian and a parallel each used
//! twice at one vertex. Each is a wrap edge of that face — the
//! meridian where its chart closes in `u`, the parallel where it closes
//! in `v` — and the body is that face as stated: no re-tessellation.
//!
//! The fixture is read in metres (its `MILLI` prefix dropped), so the
//! boolean rows meet it at a scale the default band can see.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Band, Mat3, Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use step_import::{ImportOptions, StepImport, import_step};
use topo::{Body, SolidContainment};

const R: f64 = 1.0;
const MINOR: f64 = 0.25;

fn tol() -> Tol {
    Tol::witness()
}

fn whole_torus() -> Body<f64> {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/torus.step"
    ))
    .expect("the fixture reads")
    .replace("SI_UNIT(.MILLI.,.METRE.)", "SI_UNIT($,.METRE.)");
    match import_step(&text, &ImportOptions::default(), tol()) {
        Ok(StepImport::Solid {
            body,
            normalizations,
            ..
        }) => {
            assert!(
                normalizations.is_empty(),
                "the whole torus adopts as stated: {normalizations:?}"
            );
            body
        }
        Ok(StepImport::Wireframe { .. }) => panic!("the torus imported as a wireframe"),
        Err(e) => panic!("the torus imports: {e}"),
    }
}

/// An axis-aligned box `x × y × z`.
fn boxed(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    let plane = SketchPlane::new(Affine3::from_parts(
        Mat3::identity(),
        Vec3::new(0.0, 0.0, z.0),
    ));
    let vp = Profile::new(plane, vec![lp]).validate(tol()).unwrap();
    sweep::extrude(
        &vp,
        sweep::Extrusion::Distance {
            depth: z.1 - z.0,
            side: sweep::ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body
}

/// One face, two edges, one vertex: each edge has both halves in the
/// face and is described as its wrap edge. Tiers 1–3 pass, the volume
/// is `2π²Rr²` and the area `4π²Rr`, and the mesh closes with its
/// volume within the chordal deviation of the closed form.
#[test]
fn the_whole_torus_is_one_face_cut_once_each_way() {
    let body = whole_torus();
    let faces: Vec<_> = body.faces().map(|(k, _)| k).collect();
    assert_eq!(
        (faces.len(), body.edges().count(), body.vertices().count()),
        (1, 2, 1),
        "census"
    );
    for (edge, _) in body.edges() {
        let sides = topo::readback::edge_sides(&body, edge).unwrap();
        assert_eq!(sides.faces(), (faces[0], faces[0]), "{edge:?}: both halves bound the face");
        let e = body.get_edge(edge).unwrap();
        let curve = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
        assert!(
            curve.description().chart().is_some_and(|c| c.seam),
            "{edge:?} is the face's wrap edge: {:?}",
            curve.description()
        );
    }
    assert_eq!(topo::validate(&body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&body), Ok(()), "tier 2");
    assert_eq!(topo::validate_geometric(&body, tol()), Ok(()), "tier 3");
    let mp = topo::mass_properties(&body, tol()).unwrap();
    let (volume, area) = (2.0 * PI * PI * R * MINOR * MINOR, 4.0 * PI * PI * R * MINOR);
    assert!((mp.volume - volume).abs() < 1e-12, "volume {} vs {volume}", mp.volume);
    assert!((mp.surface_area - area).abs() < 1e-12, "area {} vs {area}", mp.surface_area);
    let chordal = 1e-3;
    let mesh = mesh::tessellate(&body, chordal, tol()).expect("the whole torus meshes");
    assert_eq!(mesh::validate::check_mesh(&mesh), Ok(()), "the mesh closes");
    let got = mesh::validate::signed_volume(&mesh);
    assert!(
        got < volume && volume - got < chordal * area,
        "mesh volume {got} vs {volume}"
    );
}

/// The boolean's torus arms read the one face: point containment in
/// the tube, the hole and outside, and a disjoint union.
#[test]
fn the_whole_torus_answers_containment_and_a_disjoint_union() {
    let body = whole_torus();
    let band = Band::linear(tol()).unwrap();
    for (what, q, want) in [
        ("on the tube's centre circle", Point3::new(R, 0.0, 0.0), SolidContainment::In),
        ("in the tube", Point3::new(-0.7, 0.7, 0.1), SolidContainment::In),
        ("in the hole", Point3::new(0.0, 0.0, 0.0), SolidContainment::Out),
        ("past the rim", Point3::new(2.0, 0.0, 0.0), SolidContainment::Out),
        ("over the tube", Point3::new(R, 0.0, 0.5), SolidContainment::Out),
    ] {
        assert_eq!(
            topo::point_in_solid(&body, q, band, tol()).unwrap(),
            want,
            "{what}"
        );
    }
    let a = topo::test_support::finished("the whole torus", body, tol());
    let far = topo::test_support::finished(
        "a far box",
        boxed((3.0, 4.0), (0.0, 1.0), (0.0, 1.0)),
        tol(),
    );
    let out = topo::union(&a, &far, tol()).expect("a disjoint union builds");
    let b = &out.body().expect("not empty").body;
    assert_eq!(topo::validate_geometric(b, tol()), Ok(()), "the union's tier 3");
    let v = topo::mass_properties(b, tol()).unwrap().volume;
    let want = 2.0 * PI * PI * R * MINOR * MINOR + 1.0;
    assert!((v - want).abs() < 1e-12, "union volume {v} vs {want}");
}

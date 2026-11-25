//! A ball of radius 10 mm with poles on ±z, the cap `x > 8` cut away and
//! its sphere split at the equator into two faces on ONE spherical
//! surface (`side_cut_equator_split.step`), or with its lower face split
//! again along the meridian through the south pole
//! (`side_cut_meridian_split.step`). Each file puts a chart pole inside a
//! face, or inside an edge, without making it a vertex. A chart
//! singularity in a face is a vertex of it, so import makes each pole one
//! (`chart_singularity_vertex.rs`): a face holding a pole reaches it by
//! one wrap edge walked out and back, and an edge through one is split
//! there. Tier 3 reads the sphere's seam region's side, so a flipped
//! sphere face is named, alone or with the rest.
#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

/// Each file, with the number of faces its sphere comes in: the two
/// halves the equator split states, or those with the lower half split
/// again along its meridian.
const FILES: [(&str, usize); 2] = [
    ("side_cut_equator_split.step", 2),
    ("side_cut_meridian_split.step", 3),
];

fn import(name: &str) -> topo::Body<f64> {
    let path = format!(
        "{}/tests/fixtures/seam-region/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap();
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { body, .. }) => body.into_body(),
        other => panic!("{name} does not import as a solid: {other:?}"),
    }
}

/// The faces of the sphere: the one surface more than one face carries.
fn sphere_faces(body: &topo::Body<f64>) -> Vec<topo::FaceKey> {
    let faces: Vec<_> = body.faces().map(|(k, f)| (k, f.surface)).collect();
    faces
        .iter()
        .filter(|(_, s)| faces.iter().filter(|(_, t)| t == s).count() > 1)
        .map(|(k, _)| *k)
        .collect()
}

#[test]
fn a_side_cut_ball_holding_its_poles_inside_its_faces_imports_and_measures() {
    let tol = Tol::witness();
    let (r, h) = (0.010_f64, 0.002_f64);
    let exact = core::f64::consts::PI * (4.0 * r * r * r - h * h * (3.0 * r - h)) / 3.0;
    for (name, _) in FILES {
        let body = import(name);
        topo::validate_geometric(&body, tol)
            .unwrap_or_else(|e| panic!("{name}: tier 3 refuses it: {e:?}"));
        let volume = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", body.clone(), tol),
            tol,
        )
        .volume;
        assert!(
            (volume - exact).abs() <= 1e-9 * exact,
            "{name}: measures {volume}, the closed form is {exact}"
        );
    }
}

/// Each sphere face flipped alone: the region's side is read at a pole
/// on one of its interior edges, and the flipped face is named.
#[test]
fn a_flipped_sphere_face_is_named_by_its_regions_side() {
    let tol = Tol::witness();
    for (name, faces) in FILES {
        let body = import(name);
        let sphere = sphere_faces(&body);
        assert_eq!(sphere.len(), faces, "{name}: the sphere's faces: {sphere:?}");
        for face in sphere {
            let flipped = body
                .flipped_face_sense_for_tests(face, geom_core::Tol::witness())
                .unwrap();
            let errs = topo::validate_geometric(&flipped, tol).unwrap_err();
            assert!(
                errs.iter().any(
                    |e| matches!(e, topo::ValidationError::CurvedSenseInverted { face: f } if *f == face)
                ),
                "{name}: {face:?} flipped is not named CurvedSenseInverted: {errs:?}"
            );
        }
    }
}

/// Every sphere face flipped: each claims its complement, which holds
/// the pole the face does not reach, off any vertex of it, so each face
/// is named twice: by the region's side and by that pole. (The cover
/// census is not read here: a flipped face's lift no longer closes at
/// its wrap tip, and check 6 exempts a refused derivation.)
#[test]
fn the_side_cut_ball_flipped_whole_names_each_face_by_its_side_and_its_pole() {
    let tol = Tol::witness();
    for (name, faces) in FILES {
        let body = import(name);
        let sphere = sphere_faces(&body);
        assert_eq!(sphere.len(), faces, "{name}: the sphere's faces: {sphere:?}");
        let flipped = sphere.iter().fold(body.clone(), |b, &k| {
            b.flipped_face_sense_for_tests(k, geom_core::Tol::witness())
                .unwrap()
        });
        let errs = topo::validate_geometric(&flipped, tol).unwrap_err();
        for face in sphere {
            assert!(
                errs.iter().any(
                    |e| matches!(e, topo::ValidationError::CurvedSenseInverted { face: f } if *f == face)
                ),
                "{name}: {face:?} is not named CurvedSenseInverted: {errs:?}"
            );
            assert!(
                errs.iter().any(|e| matches!(
                    e,
                    topo::ValidationError::ChartSingularityNotAVertex { face: f, .. } if *f == face
                )),
                "{name}: {face:?} is not named by the pole its complement holds: {errs:?}"
            );
        }
    }
}

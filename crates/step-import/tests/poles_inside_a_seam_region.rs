//! A ball of radius 10 mm with poles on ±z, the cap `x > 8` cut away and
//! its sphere split at the equator into two faces on ONE spherical
//! surface. Each face holds a chart pole off its loop, and the region
//! the two form across the equator has the cut circle alone as its
//! boundary, with neither pole on an interior edge. Validity does not
//! yet refuse a chart singularity inside a face
//! (`work/restfront/validity-refuses-an-interior-chart-singularity.md`),
//! so the region's boundary encodes no side tier 3 may read, and the
//! body imports, passes tier 3 and measures exactly.
#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn a_seam_region_holding_its_poles_inside_its_faces_imports_and_measures() {
    let tol = Tol::witness();
    let path = format!(
        "{}/tests/fixtures/seam-region/side_cut_equator_split.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap();
    let (r, h) = (0.010_f64, 0.002_f64);
    let exact = core::f64::consts::PI * (4.0 * r * r * r - h * h * (3.0 * r - h)) / 3.0;
    let body = match import_step(&text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the side-cut ball does not import as a solid: {other:?}"),
    };
    topo::validate_geometric(&body, tol).unwrap_or_else(|e| panic!("tier 3 refuses it: {e:?}"));
    let volume = topo::mass_properties(&body, tol).unwrap().volume;
    assert!(
        (volume - exact).abs() <= 1e-9 * exact,
        "the side-cut ball measures {volume}, the closed form is {exact}"
    );
}

/// The same ball with its lower face also split, along the meridian in
/// `y = 0` from the equator through the south pole to the cut. The upper
/// face holds the north pole off its loop, so its own loop encodes no
/// side. The south pole lies on the meridian edge the two lower faces
/// share, interior to the region: the region's side is read there, off
/// a member other than the face being checked, and a flip of the upper
/// face is named by it.
#[test]
fn a_seam_region_reads_its_side_at_a_pole_on_another_members_edge() {
    let tol = Tol::witness();
    let path = format!(
        "{}/tests/fixtures/seam-region/side_cut_meridian_split.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap();
    let (r, h) = (0.010_f64, 0.002_f64);
    let exact = core::f64::consts::PI * (4.0 * r * r * r - h * h * (3.0 * r - h)) / 3.0;
    let body = match import_step(&text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the split side-cut ball does not import as a solid: {other:?}"),
    };
    topo::validate_geometric(&body, tol).unwrap_or_else(|e| panic!("tier 3 refuses it: {e:?}"));
    let volume = topo::mass_properties(&body, tol).unwrap().volume;
    assert!(
        (volume - exact).abs() <= 1e-9 * exact,
        "the split side-cut ball measures {volume}, the closed form is {exact}"
    );
    // The upper face: the one whose loop's vertices all lie on the equator.
    let points: Vec<_> = body.vertex_points().collect();
    let on_equator = |h: topo::HalfEdgeKey| {
        let v = body.get_half_edge(h).unwrap().start;
        points.iter().find(|(k, _)| *k == v).unwrap().1.z.abs() < 1e-9
    };
    let upper: Vec<_> = body
        .faces()
        .filter(|(_, f)| {
            topo::props::loop_edges(&body, f.outer)
                .unwrap()
                .1
                .into_iter()
                .all(on_equator)
        })
        .map(|(k, _)| k)
        .collect();
    let [upper] = upper[..] else {
        panic!("one face has its loop on the equator: {upper:?}");
    };
    let flipped = body.flipped_face_sense_for_tests(upper).unwrap();
    let errs = topo::validate_geometric(&flipped, tol).unwrap_err();
    assert!(
        errs.iter().any(
            |e| matches!(e, topo::ValidationError::CurvedSenseInverted { face } if *face == upper)
        ),
        "the upper face flipped is not named CurvedSenseInverted: {errs:?}"
    );
}

/// Both sphere faces of the side-cut ball flipped. Their region's
/// boundary holds neither pole, and both poles lie inside faces rather
/// than on an interior edge, so tier 3 reads no side for the region:
/// reading a pole inside a face as inside the region would rest on the
/// rule that a chart singularity inside a face is a vertex of it, which
/// validity does not yet enforce
/// (`work/restfront/validity-refuses-an-interior-chart-singularity.md`).
/// The flip is the volume check's: the body is inside out.
#[test]
fn the_side_cut_ball_flipped_whole_is_left_to_the_volume_check() {
    let tol = Tol::witness();
    let path = format!(
        "{}/tests/fixtures/seam-region/side_cut_equator_split.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).unwrap();
    let body = match import_step(&text, &ImportOptions::default(), tol) {
        Ok(StepImport::Solid { body, .. }) => body,
        other => panic!("the side-cut ball does not import as a solid: {other:?}"),
    };
    // The two faces of one surface entity: the sphere's.
    let faces: Vec<_> = body.faces().map(|(k, f)| (k, f.surface)).collect();
    let sphere: Vec<_> = faces
        .iter()
        .filter(|(_, s)| faces.iter().filter(|(_, t)| t == s).count() == 2)
        .map(|(k, _)| *k)
        .collect();
    assert_eq!(sphere.len(), 2, "two faces of one sphere: {faces:?}");
    let flipped = sphere.iter().fold(topo::Body::clone(&body), |b, &k| {
        b.flipped_face_sense_for_tests(k).unwrap()
    });
    let errs = topo::validate_geometric(&flipped, tol).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, topo::ValidationError::NegativeVolume { .. })),
        "the flipped ball is inside out: {errs:?}"
    );
    assert!(
        !errs
            .iter()
            .any(|e| matches!(e, topo::ValidationError::CurvedSenseInverted { .. })),
        "no side is read for a region whose poles lie inside its faces: {errs:?}"
    );
}

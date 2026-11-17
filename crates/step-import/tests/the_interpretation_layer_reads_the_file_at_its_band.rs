//! **The interpretation layer reads the file at its band** (`[import]`,
//! D7): what the file's statements are evidence of is read through the
//! kernel's readings at the file's band, every placement is built by
//! `build_axes`, two records of one surface become one, and what the
//! interpretation moves or refuses names the file entity and a distance.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{
    ImportOptions, InterpretationRefusal, NormalizationKind, StepImport, StepImportError,
    import_step,
};

use crate::common::freecad_fixture;

fn import(text: &str) -> Result<StepImport, StepImportError> {
    import_step(text, &ImportOptions::default(), Tol::witness())
}

/// `text` with `old` replaced by `new`, which must occur.
fn with(text: &str, old: &str, new: &str) -> String {
    assert!(text.contains(old), "the fixture states {old}");
    text.replace(old, new)
}

/// A surface placement's axis stated as a ratio vector twice unit: the
/// frame is built by `build_axes` (the cylinder imports on a unit axis),
/// and the move is reported keyed by the `AXIS2_PLACEMENT_3D`.
#[test]
fn a_surface_placement_stated_off_unit_is_built_and_reported() {
    let text = with(
        &freecad_fixture("cylinder"),
        "#48 = DIRECTION('',(0.,0.,1.));",
        "#48 = DIRECTION('',(0.,0.,2.));",
    );
    let imported = import(&text).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let moved: Vec<(u64, f64)> = imported
        .normalizations()
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::PlacementReMinted {
                transform,
                moved_by,
            } => Some((transform, moved_by)),
            _ => None,
        })
        .collect();
    let [(46, moved_by)] = moved.as_slice() else {
        panic!("one report, keyed by the cylinder's placement #46: {moved:?}");
    };
    assert!(*moved_by > 0.0, "the move is a distance: {moved_by}");
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    let axis = body
        .surfaces()
        .find_map(|(_, s)| match s {
            geom::Surface::Cylinder { axis, .. } => Some(*axis),
            _ => None,
        })
        .expect("the cylinder");
    assert!(
        (axis.norm() - 1.0).abs() < 1e-15,
        "the built axis is unit: {axis:?}"
    );
    // The same file stated unit reports nothing.
    assert!(
        import(&freecad_fixture("cylinder"))
            .unwrap()
            .normalizations()
            .is_empty(),
        "a unit placement moves nothing"
    );
}

/// A negative radius is a root no construction takes: refused at
/// interpretation, naming the surface record and the value, before any
/// edge certification reads it.
#[test]
fn a_negative_surface_radius_refuses_as_an_invalid_root() {
    let text = with(
        &freecad_fixture("cylinder"),
        "#45 = CYLINDRICAL_SURFACE('',#46,0.5);",
        "#45 = CYLINDRICAL_SURFACE('',#46,-0.5);",
    );
    match import(&text) {
        Err(
            ref e @ StepImportError::Interpretation {
                id,
                refusal: InterpretationRefusal::InvalidRoot { value, .. },
                ..
            },
        ) => {
            assert_eq!(id, 45, "the surface record is named");
            assert!(value < 0.0, "the file's value: {value}");
            assert!(
                e.render().contains("re-export the file from a valid model"),
                "{}",
                e.render()
            );
        }
        other => panic!("an invalid root, got {other:?}"),
    }
}

/// A `VERTEX_LOOP`'s point 0.1 mm off its sphere misses past the file's
/// band: an evidence miss naming the vertex and the ε_in that would read
/// the two as one.
#[test]
fn a_vertex_loop_point_off_its_sphere_is_an_evidence_miss() {
    let text = with(
        &freecad_fixture("sphere"),
        "#21 = CARTESIAN_POINT('',(6.123233995737E-17,-1.499759782662E-32,-1.));",
        "#21 = CARTESIAN_POINT('',(6.123233995737E-17,-1.499759782662E-32,-1.1));",
    );
    match import(&text) {
        Err(
            ref e @ StepImportError::Interpretation {
                id,
                refusal: InterpretationRefusal::EvidenceMiss { miss, .. },
                ..
            },
        ) => {
            assert_eq!(id, 20, "the VERTEX_POINT is named");
            assert!((miss - 1e-4).abs() < 1e-12, "a 0.1 mm miss: {miss}");
            assert!(e.render().contains("import with ε_in ≥ "), "{}", e.render());
        }
        other => panic!("an evidence miss, got {other:?}"),
    }
}

/// A boolean union's re-tiled sides are two `PLANE` records of one
/// locus: read at the faces, each pair becomes one record, the earlier
/// kept, and the body wears one surface per side.
#[test]
fn two_records_of_one_plane_become_one() {
    let imported = import(&freecad_fixture("fuse_boxes")).unwrap();
    let joins: Vec<(u64, u64, f64)> = imported
        .normalizations()
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::OneRecord {
                kept,
                joined,
                moved_by,
            } => Some((kept, joined, moved_by)),
            _ => None,
        })
        .collect();
    assert_eq!(joins.len(), 4, "the union's four re-tiled sides: {joins:?}");
    for &(kept, joined, moved_by) in &joins {
        assert!(kept < joined, "the earlier record stays: {kept} {joined}");
        assert!(
            moved_by <= imported.eps_in().max(Tol::witness().eps()),
            "the faces agree within the band's zero: {moved_by}"
        );
    }
    let StepImport::Solid { body, .. } = &imported else {
        panic!("a solid")
    };
    let mut planes = std::collections::BTreeSet::new();
    for (_, face) in body.faces() {
        if matches!(
            body.get_surface(face.surface),
            Some(geom::Surface::Plane { .. })
        ) {
            planes.insert(face.surface);
        }
    }
    // Two overlapping unit boxes, offset by half a unit in x and y: an
    // eight-sided outline, a top and a bottom, each one surface (the
    // file states fourteen faces on fourteen records).
    assert_eq!(planes.len(), 10, "one surface per flat side");
}

//! **Two records of one surface become one, by what the faces are**
//! (`step_import`'s `one_record`): a face's facing is its record's normal
//! composed with its sense, faces facing opposite ways are never asked,
//! the locus is read at the faces both ways and never at the records'
//! stored origins, and a distance in the file's band refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{
    ImportOptions, InterpretationRefusal, NormalizationKind, StepImport, StepImportError,
    import_step,
};

use crate::common::freecad_fixture;

fn import_at(text: &str, eps_in: Option<f64>) -> Result<StepImport, StepImportError> {
    let options = ImportOptions { eps_in };
    import_step(text, &options, Tol::witness())
}

fn import(text: &str) -> Result<StepImport, StepImportError> {
    import_at(text, None)
}

/// `text` with `old` replaced by `new`, which must occur.
fn with(text: &str, old: &str, new: &str) -> String {
    assert!(text.contains(old), "the fixture states {old}");
    text.replace(old, new)
}

/// The one-record joins an import reports.
fn joins(imported: &StepImport) -> Vec<(u64, u64)> {
    imported
        .normalizations()
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::OneRecord { kept, joined, .. } => Some((kept, joined)),
            _ => None,
        })
        .collect()
}

/// The distinct plane surfaces the shipped body wears.
fn planes(imported: &StepImport) -> usize {
    let StepImport::Solid { body, .. } = imported else {
        panic!("a solid")
    };
    let mut planes = std::collections::BTreeSet::new();
    for (_, face) in body.faces() {
        if matches!(
            body.get_surface(face.surface),
            Ok(geom::Surface::Plane { .. })
        ) {
            planes.insert(face.surface);
        }
    }
    planes.len()
}

/// The band's zero the FreeCAD fixtures are read at: their declared
/// 1e-7 mm, floored at the run's ε.
fn zero() -> f64 {
    1e-10_f64.max(Tol::witness().eps())
}

/// FreeCAD states opposite sides of a box on planes of one normal with
/// opposite senses. Those faces face opposite ways, so however coarse
/// the file's band (here up to twice the box's side), they are never
/// asked whether they are one locus: no join, no ambiguity.
#[test]
fn a_boxs_opposite_sides_are_never_asked() {
    for eps_in in [2e-4, 5e-4, 2e-3] {
        match import_at(&freecad_fixture("box"), Some(eps_in)) {
            Ok(imported) => assert!(
                joins(&imported).is_empty(),
                "ε_in {eps_in}: no join of opposite sides: {:?}",
                joins(&imported)
            ),
            Err(e) => assert!(
                !e.render().contains("step_one_record"),
                "ε_in {eps_in}: opposite sides are never asked: {}",
                e.render()
            ),
        }
    }
}

/// The union's top face `#202` restated on a plane of the opposite
/// normal with its sense flipped: the same face, facing the same way, so
/// it joins its coplanar sibling as before, its sense restated so its
/// material side is unchanged, and the body is the union's.
#[test]
fn a_face_restated_on_an_antiparallel_record_joins_facing_the_same_way() {
    let restated = with(
        &with(
            &freecad_fixture("fuse_boxes"),
            "#224 = DIRECTION('',(0.,0.,1.));",
            "#224 = DIRECTION('',(0.,0.,-1.));",
        ),
        "#202 = ADVANCED_FACE('',(#203),#221,.T.);",
        "#202 = ADVANCED_FACE('',(#203),#221,.F.);",
    );
    let imported = import(&restated).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    let plain = import(&freecad_fixture("fuse_boxes")).unwrap();
    assert_eq!(joins(&imported).len(), 4, "the four joins stand");
    assert_eq!(planes(&imported), 10, "one surface per flat side");
    let volume = |i: &StepImport| {
        let StepImport::Solid { body, .. } = i else {
            panic!("a solid")
        };
        topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished(
                "the measured body",
                body.clone().into_body(),
                Tol::witness(),
            ),
            Tol::witness(),
        )
        .volume
    };
    let (v, w) = (volume(&imported), volume(&plain));
    assert!(
        (v - w).abs() <= 1e-12 * w.abs(),
        "the same solid: {v} vs {w}"
    );
}

/// The records' stored origins say nothing: the top's plane record
/// restated with its origin moved within the plane joins as before.
#[test]
fn a_records_origin_off_the_faces_within_the_plane_still_joins() {
    let moved = with(
        &freecad_fixture("fuse_boxes"),
        "#223 = CARTESIAN_POINT('',(0.,0.,1.));",
        "#223 = CARTESIAN_POINT('',(5.,-3.,1.));",
    );
    let imported = import(&moved).unwrap_or_else(|e| panic!("imports: {}", e.render()));
    assert_eq!(joins(&imported).len(), 4, "the four joins stand");
}

/// The top's plane record raised three band zeros off its faces: its
/// faces and its sibling's read in the file's band, so the import
/// refuses that ambiguity at the record, neither joining nor keeping two.
#[test]
fn a_record_in_the_files_band_of_its_sibling_refuses() {
    let dz_mm = 3.0 * zero() * 1e3;
    let raised = with(
        &freecad_fixture("fuse_boxes"),
        "#223 = CARTESIAN_POINT('',(0.,0.,1.));",
        &format!("#223 = CARTESIAN_POINT('',(0.,0.,{:e}));", 1.0 + dz_mm),
    );
    match import(&raised) {
        Err(StepImportError::Interpretation {
            refusal: InterpretationRefusal::Ambiguous { reading, gap, .. },
            ..
        }) => {
            assert_eq!(reading, "step_one_record");
            assert!(
                (gap - 3.0 * zero()).abs() <= 0.01 * zero(),
                "the gap is the raise: {gap}"
            );
        }
        other => panic!("an in-band one-record ambiguity, got {other:?}"),
    }
}

/// One of the top's plane records tilted about the x axis through its
/// origin, so its own faces still lie on it at their edges but its
/// siblings' do not: read one way the two are one, read both ways a
/// sibling stands a few band zeros off at its far corner, so the import
/// refuses the ambiguity. Asked of the record kept (`#123`, the earliest
/// in the file) and of one joined to it (`#221`), so each way is read.
#[test]
fn the_locus_is_read_both_ways() {
    // A sibling's far corner stands up to 1.5 mm from the record's
    // origin along y.
    let tilt = 3.0 * zero() / 1.5e-3;
    for direction in ["#126", "#224"] {
        let tilted = with(
            &freecad_fixture("fuse_boxes"),
            &format!("{direction} = DIRECTION('',(0.,0.,1.));"),
            &format!("{direction} = DIRECTION('',(0.,{tilt:e},1.));"),
        );
        match import(&tilted) {
            Err(StepImportError::Interpretation {
                refusal: InterpretationRefusal::Ambiguous { reading, .. },
                ..
            }) => assert_eq!(reading, "step_one_record", "{direction} tilted"),
            other => panic!("{direction} tilted: an in-band one-record ambiguity, got {other:?}"),
        }
    }
}

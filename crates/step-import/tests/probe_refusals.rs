//! REVIEW PROBE (B7): refusal preservation beyond the committed rows —
//! mixed content, orphan curve set, 2D-context-only files.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImportError, import_step};

fn box_step() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/box.step"
    );
    std::fs::read_to_string(path).unwrap()
}

#[test]
fn mixed_solid_and_wireframe_refuses() {
    // A LINE-carried trimmed curve set beside the box's solid.
    let m = box_step().replace(
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15),#165);",
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15),#165);\n\
         #950 = GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION('',(#951),#165);\n\
         #951 = GEOMETRIC_CURVE_SET('',(#952));\n\
         #952 = CIRCLE('',#11,1.);",
    );
    match import_step(&m, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::Structure { what, .. }) => {
            println!("mixed content refuses: {what}");
            assert!(what.contains("wireframe"), "{what}");
        }
        other => panic!("mixed content must refuse typed, got {other:?}"),
    }
}

#[test]
fn orphan_curve_set_refuses() {
    let m = box_step().replace(
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15),#165);",
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15),#165);\n\
         #951 = GEOMETRIC_CURVE_SET('',(#952));\n\
         #952 = CIRCLE('',#11,1.);",
    );
    match import_step(&m, &ImportOptions::default(), Tol::witness()) {
        Err(e) => println!("orphan curve set refuses typed: {e}"),
        other => panic!("an orphan curve set must refuse, got {other:?}"),
    }
}

#[test]
fn a_two_d_context_only_file_is_nothing_to_import() {
    let text = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\n\
                FILE_NAME('x','',(''),(''),'','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\n\
                ENDSEC;\nDATA;\n\
                #1 = ( GEOMETRIC_REPRESENTATION_CONTEXT(2) PARAMETRIC_REPRESENTATION_CONTEXT() REPRESENTATION_CONTEXT('2D SPACE','') );\n\
                #2 = CARTESIAN_POINT('',(0.,0.));\n\
                ENDSEC;\nEND-ISO-10303-21;\n";
    match import_step(text, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::NothingToImport) => println!("2D-only: NothingToImport"),
        other => panic!("expected NothingToImport, got {other:?}"),
    }
}

fn cylinder_step() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/freecad/cylinder.step"
    );
    std::fs::read_to_string(path).unwrap()
}

/// The cylinder's top rim `#21` is a self-loop — `#22` at both ends —
/// on `CIRCLE #24`. `endpoint_params` returns the full period for a
/// self-loop from the start angle alone, so a start angle that is not
/// finite is the only thing standing between such an edge and a
/// `(NaN, NaN)` interval; the refusal must fire on that arm and not
/// only on the two-vertex one.
///
/// Both routes below make the eccentric-anomaly `atan2` answer NaN.
/// Without the refusal each reaches the adoption gate instead and is
/// declined there as `Adoption { attempts: [.. ParamSpan Indeterminate
/// ..] }`, which names an interval span rather than the vertex data
/// that is actually wrong.
#[test]
fn a_conic_self_loop_with_a_nonfinite_angle_refuses_typed() {
    let base = cylinder_step();
    // (a) The circle's centre `#26` is read by nothing else in the
    // file, so an overflowing coordinate there makes `p - center`
    // non-finite for the rim's vertex and for no other edge.
    let overflowing_centre = base.replace(
        "#26 = CARTESIAN_POINT('',(0.,0.,1.));",
        "#26 = CARTESIAN_POINT('',(0.,0.,1.E400));",
    );
    // (b) A semi-axis of zero: the in-plane component divided by it is
    // `0.0 / 0.0` for a vertex on the other axis.
    let zero_semi_axis = base
        .replace("#24 = CIRCLE('',#25,0.5);", "#24 = ELLIPSE('',#25,0.5,0.);")
        .replace(
            "#23 = CARTESIAN_POINT('',(0.5,-1.224646799147E-16,1.));",
            "#23 = CARTESIAN_POINT('',(0.5,0.,1.));",
        );
    for (route, text) in [
        ("overflowing centre", &overflowing_centre),
        ("zero semi-axis", &zero_semi_axis),
    ] {
        assert_ne!(text, &base, "{route}: the substitution must apply");
        match import_step(text, &ImportOptions::default(), Tol::witness()) {
            Err(StepImportError::Topology { id, what }) => {
                println!("{route}: #{id}: {what}");
                assert_eq!(id, 21, "{route}: the rim edge is the offender");
                assert!(what.contains("not finite"), "{route}: {what}");
            }
            other => panic!("{route} must refuse Topology, got {other:?}"),
        }
    }
}

/// The refusal above names non-finite data, and must not name the
/// conic's centre: `atan2(0.0, 0.0)` is `0.0`, so a vertex exactly at
/// the centre yields a finite (and wrong) angle, and is declined by
/// the adoption gate's endpoint residual instead.
#[test]
fn a_vertex_at_the_conic_centre_is_a_residual_refusal_not_a_finiteness_one() {
    let m = cylinder_step().replace(
        "#23 = CARTESIAN_POINT('',(0.5,-1.224646799147E-16,1.));",
        "#23 = CARTESIAN_POINT('',(0.,0.,1.));",
    );
    match import_step(&m, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::Adoption { id, .. }) => println!("centre vertex: #{id}"),
        other => panic!("a centre vertex must reach the adoption gate, got {other:?}"),
    }
}

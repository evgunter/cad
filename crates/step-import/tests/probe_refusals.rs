//! REVIEW PROBE (B7): refusal preservation beyond the committed rows —
//! mixed content, orphan curve set, 2D-context-only files.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use step_import::{ImportOptions, StepImportError, import_step};
use geom_core::Tol;

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

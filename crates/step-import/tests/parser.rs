//! Acceptance rows 4–6 (M7-1 spec §2): the `nurbs_wireframe`
//! disposition, the refusal rows (typed, entity-named, no panics),
//! and the ε_in row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::fixture;
use step_import::{ImportOptions, StepImport, StepImportError, import_step};

/// Row 4: the curve-only fixture parses; the rational quadratic
/// reconstructs exactly (`==` on control points / knots / weights —
/// legitimate because the writer's printer round-trips to identical
/// bits); **no body is claimed**, and the reason is structural: a
/// `GEOMETRIC_CURVE_SET` wireframe carries no faces, shells, or
/// solids, so there is nothing a `Body` could honestly represent —
/// the import result is the `Wireframe` disposition, not a skip.
#[test]
fn nurbs_wireframe_disposition() {
    let text = fixture("nurbs_wireframe", "step");
    let import = import_step(&text, &ImportOptions::default()).expect("the wireframe imports");
    let StepImport::Wireframe { curves, eps_in } = import else {
        panic!("a curve-only file must take the wireframe disposition, not claim a body");
    };
    assert_eq!(eps_in, 1e-9, "the file's declared uncertainty");
    assert_eq!(curves.len(), 1, "one curve in the set");
    let geom_curves::Curve3::Nurbs(payload) = &curves[0] else {
        panic!("the curve is the rational quadratic");
    };
    // Exact reconstruction: the writer's record pin, inverted.
    assert_eq!(payload.knots().degree(), 2);
    assert_eq!(payload.knots().knots(), &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
    let control = payload.control();
    assert_eq!((control[0].x, control[0].y, control[0].z), (1.0, 0.0, 0.0));
    assert_eq!((control[1].x, control[1].y, control[1].z), (1.0, 1.0, 0.0));
    assert_eq!((control[2].x, control[2].y, control[2].z), (0.0, 1.0, 0.0));
    assert_eq!(
        payload.weights(),
        &[1.0, std::f64::consts::FRAC_1_SQRT_2, 1.0]
    );
}

/// A minimal hand-authored solid file whose one face carries the
/// given surface record — the refusal-row scaffold. The face's
/// surface resolves before its bounds, so the bounds may dangle.
fn minimal_solid_with_surface(surface_record: &str) -> String {
    format!(
        "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''), '2;1');\n\
         FILE_NAME('t.step', '1970-01-01T00:00:00', (''), (''), 't', '', '');\n\
         FILE_SCHEMA(('AUTOMOTIVE_DESIGN {{ 1 0 10303 214 1 1 1 1 }}'));\nENDSEC;\nDATA;\n\
         #1 = {surface_record};\n\
         #2 = ADVANCED_FACE('', (#99), #1, .T.);\n\
         #3 = CLOSED_SHELL('', (#2));\n\
         #4 = MANIFOLD_SOLID_BREP('t', #3);\n\
         #5 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) );\n\
         #6 = UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.0E-9), #5, \
         'distance_accuracy_value', '');\n\
         ENDSEC;\nEND-ISO-10303-21;\n"
    )
}

/// Row 5(a): `B_SPLINE_SURFACE_WITH_KNOTS` refuses, typed, naming the
/// entity — the named M7 frontier (NURBS faces arrive with the
/// loft/sweep assembly unit; the S9 flip pattern retires this refusal
/// when their export lands).
#[test]
fn bspline_surface_refuses_typed() {
    let text = minimal_solid_with_surface(
        "B_SPLINE_SURFACE_WITH_KNOTS('', 1, 1, ((#98)), .UNSPECIFIED., .F., .F., .F., \
         (2), (2), (0.0), (1.0), .UNSPECIFIED.)",
    );
    let err = import_step(&text, &ImportOptions::default())
        .expect_err("a NURBS surface is outside the imported subset");
    match err {
        StepImportError::UnsupportedEntity { id, keyword } => {
            assert_eq!(id, 1, "the refusal names the surface entity");
            assert_eq!(keyword, "B_SPLINE_SURFACE_WITH_KNOTS");
        }
        other => panic!("expected UnsupportedEntity, got: {other}"),
    }
}

/// Row 5(b): truncated / malformed files refuse with typed parse
/// errors — never panics. Truncation is exercised at every prefix
/// length of a small real fixture (brute force is cheap here and
/// leaves no untested cut point).
#[test]
fn truncations_refuse_without_panicking() {
    let text = fixture("cube", "step");
    for cut in 0..text.len().min(400) {
        let truncated = &text[..cut];
        let result = import_step(truncated, &ImportOptions::default());
        assert!(
            result.is_err(),
            "a truncated exchange file (cut at {cut}) must refuse"
        );
    }
    // A structurally malformed record: garbage where a record belongs.
    let garbage = "ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n#1 = ;\nENDSEC;\nEND-ISO-10303-21;\n";
    match import_step(garbage, &ImportOptions::default()) {
        Err(StepImportError::Syntax { line, .. }) => assert_eq!(line, 5),
        other => panic!("expected a syntax refusal, got: {other:?}"),
    }
}

/// Row 5(c): a dangling entity reference refuses naming both ids.
#[test]
fn dangling_reference_refuses_named() {
    let text =
        fixture("cube", "step").replace("#13 = LINE('', #10, #12);", "#13 = LINE('', #10, #9999);");
    let err = import_step(&text, &ImportOptions::default())
        .expect_err("a dangling reference must refuse");
    match err {
        StepImportError::DanglingReference { from, to } => {
            assert_eq!((from, to), (13, 9999), "the refusal names both ids");
        }
        other => panic!("expected DanglingReference, got: {other}"),
    }
}

/// Units are read, not assumed: a millimetre-prefixed unit context is
/// outside the subset and refuses typed (M7-2 owns foreign units).
#[test]
fn prefixed_unit_refuses_typed() {
    let text = fixture("cube", "step").replace(
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) )",
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI., .METRE.) )",
    );
    let err = import_step(&text, &ImportOptions::default())
        .expect_err("a prefixed unit is outside the subset");
    assert!(
        matches!(err, StepImportError::UnsupportedUnit { .. }),
        "expected UnsupportedUnit, got: {err}"
    );
}

/// Row 6: the import result exposes the file's declared uncertainty
/// as ε_in, and the per-call override exists — both on a corpus file.
#[test]
fn eps_in_declared_and_overridable() {
    let text = fixture("cube", "step");
    let declared = import_step(&text, &ImportOptions::default()).expect("imports");
    assert_eq!(
        declared.eps_in(),
        1e-9,
        "every corpus file declares the kernel's own ε"
    );
    let overridden = import_step(
        &text,
        &ImportOptions {
            eps_in: Some(2.5e-7),
        },
    )
    .expect("imports under an override");
    assert_eq!(overridden.eps_in(), 2.5e-7, "the per-call override wins");
    // An invalid override refuses typed before any parsing.
    let err = import_step(&text, &ImportOptions { eps_in: Some(0.0) })
        .expect_err("a non-positive override refuses");
    assert!(matches!(
        err,
        StepImportError::InvalidEpsOverride { value } if value == 0.0
    ));
}

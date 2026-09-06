//! Acceptance rows 4–6 (M7-1 spec §2): the `nurbs_wireframe`
//! disposition, the refusal rows (typed, entity-named, no panics),
//! and the ε_in row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::fixture;
use geom_core::Tol;
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
    let import = import_step(&text, &ImportOptions::default(), Tol::witness())
        .expect("the wireframe imports");
    let StepImport::Wireframe { curves, eps_in, .. } = import else {
        panic!("a curve-only file must take the wireframe disposition, not claim a body");
    };
    assert_eq!(eps_in, 1e-9, "the file's declared uncertainty");
    assert_eq!(curves.len(), 1, "one curve in the set");
    let geom::Curve3::Nurbs(payload) = &curves[0] else {
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
         #7 = ( GEOMETRIC_REPRESENTATION_CONTEXT(3) \
         GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#6)) \
         GLOBAL_UNIT_ASSIGNED_CONTEXT((#5)) REPRESENTATION_CONTEXT('c', '3D') );\n\
         #8 = ADVANCED_BREP_SHAPE_REPRESENTATION('', (#4), #7);\n\
         ENDSEC;\nEND-ISO-10303-21;\n"
    )
}

/// Row 5(a): the arm's VALIDATION (the NURBS-face acceptance rows
/// live in `roundtrip.rs` and `nurbs_import.rs`). A
/// `B_SPLINE_SURFACE_WITH_KNOTS` whose knot multiplicities do not
/// sum to ISO 10303-42's `n + d + 1` refuses typed at the same
/// pre-allocation budget gate the curve twin owns, naming the
/// surface entity — never a lenient re-interpretation, never a
/// panic.
#[test]
fn bspline_surface_knot_budget_refuses_typed() {
    // A resolvable 2×2 net at degree 1×1 whose u-multiplicities sum
    // to 2 where the budget fixes 4 (2 + 1 + 1).
    let text = minimal_solid_with_surface(
        "B_SPLINE_SURFACE_WITH_KNOTS('', 1, 1, ((#97, #98), (#96, #98)), .UNSPECIFIED., \
         .F., .F., .F., (2), (2), (0.0), (1.0), .UNSPECIFIED.);\n\
         #96 = CARTESIAN_POINT('', (0.0, 1.0, 0.0));\n\
         #97 = CARTESIAN_POINT('', (0.0, 0.0, 0.0));\n\
         #98 = CARTESIAN_POINT('', (1.0, 0.0, 0.0))",
    );
    let err = import_step(&text, &ImportOptions::default(), Tol::witness())
        .expect_err("a NURBS surface with a broken knot budget must refuse");
    match err {
        StepImportError::MalformedRecord { id, expected } => {
            assert_eq!(id, 1, "the refusal names the surface entity");
            assert!(
                expected.contains("knot multiplicities"),
                "the refusal is the knot-budget gate, got: {expected}"
            );
        }
        other => panic!("expected MalformedRecord, got: {other}"),
    }
}

/// Row 5(b): truncated / malformed files refuse with typed parse
/// errors — never panics. Truncation is exercised at **every strict
/// prefix** of a real fixture (brute force is cheap and leaves no
/// untested cut point).
/// The one prefix that legitimately imports is the cut dropping only
/// the trailing newline — a semantically complete exchange file.
#[test]
fn truncations_refuse_without_panicking() {
    let text = fixture("cube", "step");
    for cut in 0..text.len() {
        let truncated = &text[..cut];
        let result = import_step(truncated, &ImportOptions::default(), Tol::witness());
        if cut + 1 == text.len() {
            assert!(
                result.is_ok(),
                "the trailing-newline cut is a complete file"
            );
        } else {
            assert!(
                result.is_err(),
                "a truncated exchange file (cut at {cut}) must refuse"
            );
        }
    }
    // A structurally malformed record: garbage where a record belongs.
    let garbage = "ISO-10303-21;\nHEADER;\nENDSEC;\nDATA;\n#1 = ;\nENDSEC;\nEND-ISO-10303-21;\n";
    match import_step(garbage, &ImportOptions::default(), Tol::witness()) {
        Err(StepImportError::Syntax { line, .. }) => assert_eq!(line, 5),
        other => panic!("expected a syntax refusal, got: {other:?}"),
    }
}

/// Row 5(c): a dangling entity reference refuses naming both ids.
#[test]
fn dangling_reference_refuses_named() {
    let text =
        fixture("cube", "step").replace("#13 = LINE('', #10, #12);", "#13 = LINE('', #10, #9999);");
    let err = import_step(&text, &ImportOptions::default(), Tol::witness())
        .expect_err("a dangling reference must refuse");
    match err {
        StepImportError::DanglingReference { from, to } => {
            assert_eq!((from, to), (13, 9999), "the refusal names both ids");
        }
        other => panic!("expected DanglingReference, got: {other}"),
    }
}

/// **The M7-1 prefixed-unit refusal, flipped** (the S9 pattern: a
/// refusal a later unit retires by building the thing it named). M7-1
/// refused `SI_UNIT(.MILLI., .METRE.)` typed and said in so many words
/// that M7-2 owns foreign units; M7-2 Leg A builds the prefix table, so
/// the same file now IMPORTS — and the assertion the flip owes is that
/// it imports *scaled*, not merely accepted.
///
/// The same cube in millimetres is the same solid a thousand times
/// smaller in each direction: its certified volume must be 1e-9 of the
/// metre file's, and the declared uncertainty — a length like any
/// other — 1e-3 of it.
#[test]
fn prefixed_unit_scales_instead_of_refusing() {
    let metres = fixture("cube", "step");
    let millimetres = metres.replace(
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) )",
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI., .METRE.) )",
    );
    assert_ne!(metres, millimetres, "the unit record must actually differ");
    let of = |text: &str| -> (f64, f64) {
        let import =
            import_step(text, &ImportOptions::default(), Tol::witness()).expect("the cube imports");
        let StepImport::Solid { body, eps_in, .. } = import else {
            panic!("expected a solid");
        };
        (
            topo::mass_properties(&body, Tol::witness()).unwrap().volume,
            eps_in,
        )
    };
    let (v_m, eps_m) = of(&metres);
    let (v_mm, eps_mm) = of(&millimetres);
    // 1e-9 is exact in binary only up to rounding, so the comparison is
    // relative and generous by the volume's own accumulated roundoff.
    assert!(
        (v_mm - v_m * 1e-9).abs() <= 8.0 * f64::EPSILON * v_m * 1e-9,
        "millimetre cube volume {v_mm} m^3 vs the metre cube's {v_m} m^3 scaled"
    );
    assert!(
        (eps_mm - eps_m * 1e-3).abs() <= f64::EPSILON * eps_m,
        "declared uncertainty {eps_mm} vs {eps_m} scaled: eps_in is a length too"
    );
}

/// **The S9 flip of review MAJOR-1's case (M7-4 Leg B).** A
/// `CONVERSION_BASED_UNIT` length context — an inch file's normal
/// form, carrying **no** `SI_UNIT` record on the length at all — used
/// to refuse typed, because a reader that assumed metres there would
/// import the part at 1/25.4 scale without saying so. It now
/// resolves, and the whole content of the flip is WHERE the number
/// comes from: the conversion expression the file itself states. The
/// history the refusal was protecting is kept by the second half of
/// this test — a conversion whose factor is missing, or is not a
/// length, still refuses typed and names the unit.
#[test]
fn a_conversion_based_unit_resolves_from_the_file_s_own_factor() {
    let plain = fixture("cube", "step");
    let volume = |text: &str| {
        let StepImport::Solid { body, .. } =
            import_step(text, &ImportOptions::default(), Tol::witness()).unwrap()
        else {
            panic!("a solid")
        };
        topo::mass_properties(&body, Tol::witness()).unwrap().volume
    };
    // The same metre unit, re-declared as an inch OVER that metre.
    let inch = plain.replace(
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) )",
        "( CONVERSION_BASED_UNIT('INCH', #9990) LENGTH_UNIT() NAMED_UNIT(#9991) );\n\
         #9990 = LENGTH_MEASURE_WITH_UNIT(LENGTH_MEASURE(0.0254), #9992);\n\
         #9991 = DIMENSIONAL_EXPONENTS(1., 0., 0., 0., 0., 0., 0.);\n\
         #9992 = ( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) )",
    );
    let ratio = volume(&inch) / volume(&plain);
    assert!(
        (ratio - 0.0254_f64.powi(3)).abs() <= 1e-9 * 0.0254_f64.powi(3),
        "every length scaled by the file's own 0.0254: volume ratio {ratio}"
    );
    // A file that declared a DIFFERENT inch would import at ITS
    // number — the point of reading the expression rather than the
    // name. (Nothing here ever looks at the string 'INCH'.)
    let odd = inch.replace("LENGTH_MEASURE(0.0254)", "LENGTH_MEASURE(0.0253)");
    let ratio = volume(&odd) / volume(&plain);
    assert!(
        (ratio - 0.0253_f64.powi(3)).abs() <= 1e-9 * 0.0253_f64.powi(3),
        "the file's arithmetic, not ours: volume ratio {ratio}"
    );

    // The history: a conversion factor that is not there is not a
    // conversion, and neither is one that measures the wrong thing.
    let dangling = plain.replace(
        "( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($, .METRE.) )",
        "( CONVERSION_BASED_UNIT('INCH', #9990) LENGTH_UNIT() NAMED_UNIT(#9991) )",
    );
    assert!(
        matches!(
            import_step(&dangling, &ImportOptions::default(), Tol::witness())
                .expect_err("no factor"),
            StepImportError::DanglingReference { to: 9990, .. }
        ),
        "a conversion whose factor entity is missing refuses typed"
    );
    let angular = inch.replace(
        "#9990 = LENGTH_MEASURE_WITH_UNIT(LENGTH_MEASURE(0.0254), #9992);",
        "#9990 = PLANE_ANGLE_MEASURE_WITH_UNIT(PLANE_ANGLE_MEASURE(0.0254), #9993);\n\
         #9993 = ( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($, .RADIAN.) );",
    );
    let err = import_step(&angular, &ImportOptions::default(), Tol::witness())
        .expect_err("a length declared over an angle is not a length");
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
    let declared = import_step(&text, &ImportOptions::default(), Tol::witness()).expect("imports");
    assert_eq!(
        declared.eps_in(),
        1e-9,
        "every corpus file declares the kernel's own ε"
    );
    let overridden = import_step(
        &text,
        &ImportOptions {
            eps_in: Some(2.5e-7),
            ..ImportOptions::default()
        },
        Tol::witness(),
    )
    .expect("imports under an override");
    assert_eq!(overridden.eps_in(), 2.5e-7, "the per-call override wins");
    // An invalid override refuses typed before any parsing.
    let err = import_step(
        &text,
        &ImportOptions {
            eps_in: Some(0.0),
            ..ImportOptions::default()
        },
        Tol::witness(),
    )
    .expect_err("a non-positive override refuses");
    assert!(matches!(
        err,
        StepImportError::InvalidEpsOverride { value } if value == 0.0
    ));
}

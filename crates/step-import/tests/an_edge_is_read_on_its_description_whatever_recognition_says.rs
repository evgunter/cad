//! **An edge is read on its description, whatever curve recognition
//! says** (`docs/DESIGN.md` `[import]` step 2; `crates/step-import/src/on_built.rs`):
//! the build constructs each edge's carrier from its description and
//! certifies the printed carrier on it over its whole trim, so whether a
//! printed spline promotes decides nothing about whether its edge
//! imports.
//!
//! - `declined/cyl_declined`: the closed-rim cylinder with its top rim a
//!   32-span rational circle, its first span stretched in parameter and
//!   its middle control nudged 3e-11 m outward. The rim lies within
//!   1.5e-11 m of the plane × cylinder circle, but the three samples curve
//!   recognition estimates its circle from all fall in that span, and the
//!   estimate misses the rest of the rim by about 6e-9 m (at a coarse
//!   band, the samples are too nearly collinear to estimate from): the
//!   rim stays a spline at every band, and the solid
//!   imports on its analytic walls all the same, each piece of the rim
//!   built on the walls' circle and the nudged one reporting its move.
//! - `declined/side_column_declined`: the re-timed-rim cube with one
//!   straight rim stated as a degree-2 spline whose middle control
//!   stands one rounding off the line. The line's locus residual, read
//!   through the composite's square root, is past the file's band, so it
//!   stays a spline, not bit for bit the wall's column; the column's own
//!   law carries it all the same.
//! - The cube with one edge's `LINE` stated as a cubic that bulges off
//!   its two planes' line between the ends and middle of its trim, where
//!   it lies on them: the certificate reads the bulge and refuses.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

fn fixture(name: &str) -> String {
    std::fs::read_to_string(format!(
        "{}/tests/fixtures/declined/{name}.step",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

/// Whether edge `id` is reported as built on a carrier that is not the
/// printed one.
fn built(normalizations: &[step_import::StructureNormalization], id: u64) -> bool {
    normalizations
        .iter()
        .any(|n| n.face == id && matches!(n.kind, NormalizationKind::CarrierBuilt { .. }))
}

#[test]
fn a_rim_recognition_declines_imports_on_its_analytic_walls() {
    let tol = Tol::witness();
    let import = import_step(&fixture("cyl_declined"), &ImportOptions::default(), tol)
        .unwrap_or_else(|e| panic!("the cylinder imports: {e}"));
    let promoted = import.curve_promotions().iter().any(|p| p.curve == 24);
    let StepImport::Solid { normalizations, .. } = import else {
        panic!("a solid");
    };
    // At a fine band the estimate's miss reads past it; at a coarse one
    // (a run tolerance of 1e-6) its three samples, all in one short span,
    // read too nearly collinear to estimate a circle from. Either way the
    // rim stays a spline at every row.
    assert!(!promoted, "the nudged rim stays a spline");
    // A spline is cut at its double knots before the build (the C⁰-knot
    // split reads the printed carrier's kind), and each piece is then
    // built on the circle: the nudged span reports its move.
    assert!(
        normalizations
            .iter()
            .any(|n| matches!(n.kind, NormalizationKind::EdgeSplitAtC0Knot)),
        "the spline rim is split at its knots: {normalizations:?}"
    );
    let moves: Vec<f64> = normalizations
        .iter()
        .filter_map(|n| match n.kind {
            NormalizationKind::CarrierBuilt { moved_by } => Some(moved_by),
            _ => None,
        })
        .collect();
    assert!(
        moves.iter().any(|&m| (5e-12..5e-11).contains(&m)),
        "the nudged span is carried on the circle and reports its move: {moves:?}"
    );
}

#[test]
fn a_straight_rim_recognition_declines_is_carried_on_its_column() {
    let normalizations = match import_step(
        &fixture("side_column_declined"),
        &ImportOptions::default(),
        Tol::witness(),
    ) {
        Ok(StepImport::Solid { normalizations, .. }) => normalizations,
        Ok(StepImport::Wireframe { .. }) => panic!("a solid"),
        Err(e) => panic!("the cube imports: {e}"),
    };
    assert!(
        built(&normalizations, 95),
        "the rim is carried on the wall's column: {normalizations:?}"
    );
}

/// The cube fixture with edge `#14`'s `LINE` (from `(0, 1, 1)` to
/// `(0, 0, 1)` on planes `x = 0` and `z = 1`) stated as a cubic whose two
/// inner controls stand `±off` along `x`: the bulge `off·3t(1−t)(1−2t)`
/// vanishes at the trim's ends and middle.
fn bulged(off: f64) -> String {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../step-export/tests/fixtures/cube.step"
    ))
    .expect("the cube fixture reads");
    let line = "#13 = LINE('', #10, #12);";
    assert!(src.contains(line), "edge #14's carrier is #13");
    src.replace(
        line,
        &format!(
            "#901 = CARTESIAN_POINT('', ({off:?}, 0.6666666666666666, 1.0));\n\
             #902 = CARTESIAN_POINT('', ({:?}, 0.3333333333333333, 1.0));\n\
             #903 = CARTESIAN_POINT('', (0.0, 0.0, 1.0));\n\
             #13 = B_SPLINE_CURVE_WITH_KNOTS('', 3, (#10, #901, #902, #903), .UNSPECIFIED., \
             .F., .F., (4, 4), (0.0, 1.0), .UNSPECIFIED.);",
            -off
        ),
    )
}

#[test]
fn a_printed_curve_off_its_description_between_its_samples_refuses() {
    let tol = Tol::witness();
    assert!(
        matches!(
            import_step(&bulged(0.0), &ImportOptions::default(), tol),
            Ok(StepImport::Solid { .. })
        ),
        "the cubic stated straight imports"
    );
    // A bulge of about 8.7e-5 m: past the band at every run tolerance.
    let Err(err) = import_step(&bulged(3e-4), &ImportOptions::default(), tol) else {
        panic!("a printed edge 8.7e-5 m off its description must refuse");
    };
    let message = err.to_string();
    assert!(
        message.contains("#14") && message.contains("(step_edge_on_description)"),
        "refused at the edge's description: {message}"
    );
}

/// **A declined rim the certificate reads too loosely refuses at the
/// fine band** (`declined/cyl_declined_loose`: the rim of
/// `cyl_declined` nudged 8e-11 m instead of 3e-11 m). At a run tolerance
/// of 1e-12 the file is read at its own ε_in of 1e-10 m, and the circle
/// composite reads the nudged span's stand-off (about 4e-11 m) as
/// 1.07e-10 m, in band, where three samples read it Zero. This pins
/// today's over-refusal, filed as
/// `work/exch/the-printed-on-built-certificate-reads-curved-pairs-loosely.md`;
/// its fix turns this row.
#[test]
fn a_declined_rim_read_loosely_refuses_at_the_fine_band() {
    let tol = Tol::witness();
    let result = import_step(
        &fixture("cyl_declined_loose"),
        &ImportOptions::default(),
        tol,
    );
    if tol.eps() > 1e-10 {
        assert!(
            matches!(result, Ok(StepImport::Solid { .. })),
            "at a band of 1e-9 or coarser the rim reads Zero and imports"
        );
        return;
    }
    let Err(err) = result else {
        panic!("at the fine band the loose bound refuses today");
    };
    let message = err.to_string();
    assert!(
        message.contains("(step_edge_on_description), in the band it is read at"),
        "refused in band at the edge's description: {message}"
    );
}

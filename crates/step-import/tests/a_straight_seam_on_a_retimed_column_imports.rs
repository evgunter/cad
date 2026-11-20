//! **A straight seam on a re-timed straight column imports.** The
//! committed `nonuniform_loft` with one seam made straight
//! (`fixtures/retimed-seam/`): its two walls' shared column, and the
//! seam's own spline, run along the line `x = y = −1` with the middle
//! control at `z = 0.6`, off the Greville position `z = 1.5`, so the
//! column traces the line non-affinely in its parameter.
//!
//! - `seam_line` states the seam as a `LINE`. The column matches it as a
//!   locus, but is not the affine map a `Line`'s iso image would read it
//!   as, so the edge is carried on the column itself and reports that
//!   carrier (`CarrierBuilt`).
//! - `seam_spline` states the seam as the spline. It does not promote at
//!   this file's band (its line's locus residual, read through the
//!   composite's square root, is past the band), so it is the column bit
//!   for bit: carried on the column, with nothing to report.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

/// The seam's `EDGE_CURVE` and its printed curve entity.
const SEAM: u64 = 93;
const SEAM_CURVE: u64 = 92;

fn ledger(name: &str) -> Vec<step_import::StructureNormalization> {
    let path = format!(
        "{}/tests/fixtures/retimed-seam/{name}.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(path).unwrap();
    match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(StepImport::Solid { normalizations, .. }) => normalizations,
        Ok(StepImport::Wireframe { .. }) => panic!("{name}: a solid"),
        Err(e) => panic!("{name}: the loft imports: {e}"),
    }
}

#[test]
fn either_statement_of_the_seam_imports_on_its_column() {
    let built = |n: &step_import::StructureNormalization| {
        n.face == SEAM && matches!(n.kind, NormalizationKind::CarrierBuilt { .. })
    };
    let line = ledger("seam_line");
    assert!(
        line.iter().any(built),
        "seam_line: the LINE seam is carried on its column: {line:?}"
    );
    let spline = ledger("seam_spline");
    assert!(
        !spline.iter().any(built),
        "seam_spline: the spline seam IS its column: {spline:?}"
    );
    assert!(
        !spline
            .iter()
            .any(|n| n.face == SEAM_CURVE
                && matches!(n.kind, NormalizationKind::CurvePromoted { .. })),
        "seam_spline: the seam's spline does not promote at the file's band"
    );
}

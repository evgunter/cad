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
//! - `seam_spline` states the seam as the spline, and what it reports
//!   depends on the band. At the file's own (`ε_in = 1e-9`) it does not
//!   promote — its line's locus residual, read through the composite's
//!   square root, is past the band — so it is the column bit for bit:
//!   carried on it, with nothing to report. At a band of `1e-6` it
//!   promotes to its line, the edge is carried on the column all the
//!   same and reports that carrier (`CarrierBuilt`), and the promotion
//!   no edge carries is not reported.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

/// The seam's `EDGE_CURVE` and its printed curve entity.
const SEAM: u64 = 93;
const SEAM_CURVE: u64 = 92;

fn ledger(name: &str, eps_in: Option<f64>) -> Vec<step_import::StructureNormalization> {
    let path = format!(
        "{}/tests/fixtures/retimed-seam/{name}.step",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(path).unwrap();
    let options = ImportOptions { eps_in };
    match import_step(&text, &options, Tol::witness()) {
        Ok(StepImport::Solid { normalizations, .. }) => normalizations,
        Ok(StepImport::Wireframe { .. }) => panic!("{name}: a solid"),
        Err(e) => panic!("{name}: the loft imports: {e}"),
    }
}

fn built(n: &step_import::StructureNormalization) -> bool {
    n.face == SEAM && matches!(n.kind, NormalizationKind::CarrierBuilt { .. })
}

fn promoted(n: &step_import::StructureNormalization) -> bool {
    n.face == SEAM_CURVE && matches!(n.kind, NormalizationKind::CurvePromoted { .. })
}

#[test]
fn either_statement_of_the_seam_imports_on_its_column() {
    let line = ledger("seam_line", None);
    assert!(
        line.iter().any(built),
        "seam_line: the LINE seam is carried on its column: {line:?}"
    );
    // The file's band, or the run's where that is coarser: the spline
    // promotes at 1e-6 and not at 1e-9 (the CI rows are 1e-12, 1e-9 and
    // 1e-6).
    let spline = ledger("seam_spline", None);
    let coarse = Tol::witness().eps() >= 1e-6;
    assert_eq!(
        spline.iter().any(built),
        coarse,
        "seam_spline at the file's band: carried on the column, reporting a built carrier \
         exactly where the band promotes its spline: {spline:?}"
    );
    assert!(
        !spline.iter().any(promoted),
        "seam_spline: no edge carries a promotion of the seam's spline: {spline:?}"
    );
    // At a band of 1e-6 the spline promotes whatever the run's ε.
    let wide = ledger("seam_spline", Some(1e-6));
    assert!(
        wide.iter().any(built) && !wide.iter().any(promoted),
        "seam_spline at 1e-6: promoted, carried on its column, the promotion unreported: \
         {wide:?}"
    );
}

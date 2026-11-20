//! **A straight seam on a re-timed straight column imports.** The
//! committed `nonuniform_loft` with one seam made straight
//! (`fixtures/retimed-seam/`): its two walls' shared column, and the
//! seam's own spline, run along the line `x = y = −1` with the middle
//! control at `z = 0.6`, off the Greville position `z = 1.5`, so the
//! column traces the line non-affinely in its parameter. `seam_spline`
//! states the seam as that spline (it promotes to its line: promotion
//! owes the locus, not the timing); `seam_line` states it as a `LINE`.
//! Either way the edge is the wall's column, and the column is not the
//! affine map an iso image of a `Line` would read it as, so the edge
//! ships the column itself rather than a line no chart image
//! reproduces.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, StepImport, import_step};

#[test]
fn either_statement_of_the_seam_imports_on_its_column() {
    for name in ["seam_spline", "seam_line"] {
        let path = format!(
            "{}/tests/fixtures/retimed-seam/{name}.step",
            env!("CARGO_MANIFEST_DIR")
        );
        let text = std::fs::read_to_string(path).unwrap();
        match import_step(&text, &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid { .. }) => {}
            Ok(StepImport::Wireframe { .. }) => panic!("{name}: a solid"),
            Err(e) => panic!("{name}: the loft imports: {e}"),
        }
    }
}

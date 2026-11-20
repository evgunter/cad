//! **A straight rim on a re-timed straight boundary of a spline wall
//! imports on that boundary** (`fixtures/retimed-rim/`): the M7-8 cube,
//! its bowed wall's two straight boundaries moved off their Greville
//! middles (the middle control at `z = 0.2`, not `0.5`), so each traces
//! its line non-affinely. The file states those two rims as two-point
//! splines, which promote to their lines.
//!
//! The straight boundaries are the wall's COLUMNS, the iso rung's
//! (`build::column_match`). Its line arm (`build::line_on_column`, which
//! `build::cap_row` shares) reads each as the segment's locus, not the
//! straight map an affine iso image of a `Line` would read it as, so each
//! rim is carried on the wall's own column and reports that carrier.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

#[test]
fn each_straight_rim_is_carried_on_its_wall_boundary() {
    for name in ["side_column"] {
        let path = format!(
            "{}/tests/fixtures/retimed-rim/{name}.step",
            env!("CARGO_MANIFEST_DIR")
        );
        let text = std::fs::read_to_string(path).unwrap();
        let normalizations = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid { normalizations, .. }) => normalizations,
            Ok(StepImport::Wireframe { .. }) => panic!("{name}: a solid"),
            Err(e) => panic!("{name}: the cube imports: {e}"),
        };
        for rim in [95, 101] {
            assert!(
                normalizations
                    .iter()
                    .any(|n| n.face == rim
                        && matches!(n.kind, NormalizationKind::CarrierBuilt { .. })),
                "{name}: rim #{rim} is carried on the wall's boundary: {normalizations:?}"
            );
        }
    }
}

//! **An edge on a promoted open arc trims inside the arc its spline
//! covers.** A quarter-circle wedge (`fixtures/arc-trim/`):
//! its bottom rim's carrier is a rational-quadratic arc promoted to its
//! circle, and the rim edge's vertices are `(R, 0, 0)` → `(0, R, 0)`.
//!
//! - `w_semi`: the carrier runs 0° → 180°; the edge's 0° → 90° lies on
//!   it, in its direction, and the wedge imports.
//! - `w_comp`: the carrier runs 90° → 360°, so the same vertices name
//!   the 90° the spline does NOT run along — every point of that edge
//!   up to `2R` from the printed carrier. It refuses, naming the trim.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, PromotedCurveKind, StepImport, import_step};

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/arc-trim/{name}.step",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(p).unwrap()
}

#[test]
fn a_sub_arc_of_the_spline_imports_and_its_complement_refuses() {
    let semi = import_step(
        &fixture("w_semi"),
        &ImportOptions::default(),
        Tol::witness(),
    )
    .expect("a rim inside its spline's arc imports");
    assert!(matches!(semi, StepImport::Solid { .. }), "a solid");
    assert!(
        semi.curve_promotions()
            .iter()
            .any(|p| p.kind == PromotedCurveKind::Circle),
        "the half-circle spline promotes: {:?}",
        semi.curve_promotions()
    );
    let Err(err) = import_step(
        &fixture("w_comp"),
        &ImportOptions::default(),
        Tol::witness(),
    ) else {
        panic!("a rim on the arc its spline does not cover must refuse");
    };
    let message = err.to_string();
    assert!(
        message.contains("outside the piece the file's spline covers"),
        "refused at the trim: {message}"
    );
}

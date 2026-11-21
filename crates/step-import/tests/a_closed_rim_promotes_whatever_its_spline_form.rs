//! **A cylinder's closed rims promote whatever spline form states
//! them** (`fixtures/closed-rim/`): the kernel's plain cylinder
//! with each rim CIRCLE swapped for one spline of the same circle, in
//! three spellings — the 3×120° rational quadratic (`cyl_quad`), the
//! 4×90° form degree-elevated to cubics (`cyl_cubic`), and the 3×120°
//! form with its first span split at a single interior knot
//! (`cyl_single`).
//! Promotion owes the locus, its coverage and its sense, not the
//! spline's timing, so each rim promotes to its circle and the body
//! imports.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, PromotedCurveKind, StepImport, import_step};

#[test]
fn every_spelling_of_the_rim_promotes_and_the_cylinder_imports() {
    for (name, promoted) in [("cyl_quad", 2), ("cyl_cubic", 2), ("cyl_single", 2)] {
        let path = format!(
            "{}/tests/fixtures/closed-rim/{name}.step",
            env!("CARGO_MANIFEST_DIR")
        );
        let text = std::fs::read_to_string(path).unwrap();
        let import = import_step(&text, &ImportOptions::default(), Tol::witness())
            .unwrap_or_else(|e| panic!("{name}: the cylinder imports: {e}"));
        assert!(
            matches!(import, StepImport::Solid { .. }),
            "{name}: a solid"
        );
        let circles = import
            .curve_promotions()
            .iter()
            .filter(|p| p.kind == PromotedCurveKind::Circle)
            .count();
        assert_eq!(circles, promoted, "{name}: the rims that promote");
    }
}

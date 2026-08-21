//! **The curve-promotion REPORTING surface, exercised end to end**
//! (#327; both PR-#391 reviews measured that deleting the reporting
//! insert in `Resolver::nurbs` broke nothing, because no test anywhere
//! read `curve_promotions`).
//!
//! Why a `GEOMETRIC_CURVE_SET` fixture and not a solid: promotion is a
//! property of the CARRIER, decided at the sole `NurbsCurve3`
//! construction site, and the wireframe lane reaches that site with
//! nothing else in the way — no adoption ladder, no pcurve mint, no
//! at-rest gate. dm1, the corpus's only file with a certifying
//! carrier, refuses at the at-rest gate for an unrelated reason
//! (#390), so it cannot witness the report. This fixture states dm1's
//! carrier VERBATIM in form — degree 2, 7 control points, weights
//! `1, ½, …`, knots at multiples of √3, the 3×120° construction,
//! r = 5 mm — as a curve set, so the report has an executable witness
//! that does not wait on #390.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::float_cmp
)]

use step_import::{ImportOptions, PromotedCurveKind, StepImport, import_step};
use geom_core::Tol;

fn fixture() -> String {
    let path: std::path::PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "curveset_rational_circle.step",
    ]
    .iter()
    .collect();
    std::fs::read_to_string(&path).unwrap()
}

/// The promotion is REPORTED: one record, keyed by the carrier entity,
/// naming its kind and carrying a residual that is a real number in
/// metres and inside the file's own ε_in.
#[test]
fn a_certifying_carrier_is_reported_as_data() {
    let import = import_step(&fixture(), &ImportOptions::default(), Tol::witness()).expect("the curve set imports");
    let eps_in = import.eps_in();
    let promotions = import.curve_promotions();
    assert_eq!(
        promotions.len(),
        1,
        "exactly one carrier certifies in this file, got {promotions:?}"
    );
    let p = promotions[0];
    assert_eq!(
        p.curve, 8,
        "keyed by the CARRIER entity (#8), not the shape"
    );
    assert_eq!(p.kind, PromotedCurveKind::Circle);
    assert_eq!(p.kind.to_string(), "circle");
    assert!(
        p.residual.is_finite() && p.residual >= 0.0 && p.residual <= eps_in,
        "a certified residual in metres inside ε_in: {:e} vs {eps_in:e}",
        p.residual
    );
    // An exact rational-quadratic circle: the residual is rounding.
    assert!(p.residual < 1e-14, "residual {:e}", p.residual);
}

/// And the promotion REACHES THE CARRIER: the wireframe lane hands
/// back `Curve3::Circle`, not the stated NURBS. (Before this pin the
/// wireframe lane promoted silently — it had no report field at all.)
#[test]
fn the_reported_promotion_is_the_carrier_that_ships() {
    let import = import_step(&fixture(), &ImportOptions::default(), Tol::witness()).expect("the curve set imports");
    let StepImport::Wireframe { ref curves, .. } = import else {
        panic!("a GEOMETRIC_CURVE_SET file is a wireframe");
    };
    assert_eq!(curves.len(), 1);
    let geom::Curve3::Circle { radius, axis, .. } = curves[0] else {
        panic!("the carrier must ship as a Circle, got {:?}", curves[0]);
    };
    assert!((radius - 0.005).abs() < 1e-15, "r = 5 mm: {radius:e}");
    assert!((axis.z.abs() - 1.0).abs() < 1e-12, "axis {axis:?}");
    // The report and the shipped carrier agree about the kind — the
    // record is not a parallel opinion.
    assert_eq!(import.curve_promotions()[0].kind, PromotedCurveKind::Circle);
}

/// The NEGATIVE half, so the report cannot go green by always firing
/// — and the ε_in plumbing, pinned: the SAME file at a budget below
/// its own residual promotes nothing, ships the stated NURBS carrier,
/// and reports nothing. One fixture, both outcomes, the difference
/// being only the interpretation budget.
#[test]
fn below_its_own_residual_nothing_is_promoted_and_nothing_is_reported() {
    let options = ImportOptions {
        eps_in: Some(1e-18),
        ..ImportOptions::default()
    };
    let import = import_step(&fixture(), &options, Tol::witness()).expect("the curve set still imports");
    assert!(
        import.curve_promotions().is_empty(),
        "nothing certifies at 1e-18, got {:?}",
        import.curve_promotions()
    );
    let StepImport::Wireframe { ref curves, .. } = import else {
        panic!("a GEOMETRIC_CURVE_SET file is a wireframe");
    };
    assert!(
        matches!(curves[0], geom::Curve3::Nurbs(_)),
        "the carrier must ship exactly as stated when nothing certifies"
    );
}

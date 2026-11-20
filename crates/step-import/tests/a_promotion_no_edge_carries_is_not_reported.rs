//! **A promotion no edge carries is not reported**
//! (`fixtures/promoted-rim/`): the C⁰-knot suite's half cone with its
//! bottom rim restated as the wall's own boundary row, a rational
//! quadratic half circle. The spline promotes to its circle, and the
//! build carries the rim on the wall's row (a cap's rim the plane
//! holds), so the body holds the spline: the curve has no
//! `CurvePromoted` record, and the edges' `CarrierBuilt` records say what
//! they hold. The top rim, a stated `CIRCLE`, is no promotion.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

#[test]
fn a_rim_the_build_carries_on_its_row_reports_its_carrier_not_a_promotion() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/promoted-rim/half_cone_spline_rim.step"
    ))
    .unwrap();
    let import = match import_step(&text, &ImportOptions::default(), Tol::witness()) {
        Ok(import @ StepImport::Solid { .. }) => import,
        Ok(StepImport::Wireframe { .. }) => panic!("a solid"),
        Err(e) => panic!("the half cone imports: {e}"),
    };
    assert!(
        import.curve_promotions().is_empty(),
        "no edge carries the promoted circle: {:?}",
        import.curve_promotions()
    );
    let StepImport::Solid {
        ref normalizations,
        ref body,
        ..
    } = import
    else {
        unreachable!("matched above")
    };
    // The rim PROMOTED: a promoted carrier has no knots, so the C⁰-knot
    // split cuts no edge (the stated spline, unpromoted, is cut at its
    // double knot, `EdgeSplitAtC0Knot`); the wall's split cuts the rim,
    // and its pieces, carried on the wall's row, report the carriers they
    // hold.
    assert!(
        !normalizations
            .iter()
            .any(|n| matches!(n.kind, NormalizationKind::EdgeSplitAtC0Knot)),
        "the rim's spline promoted, so no edge is cut at a knot: {normalizations:?}"
    );
    let built = normalizations
        .iter()
        .filter(|n| matches!(n.kind, NormalizationKind::CarrierBuilt { .. }))
        .count();
    assert!(built > 0, "the edges report the carriers they hold");
    assert!(
        body.edges().any(|(_, e)| matches!(
            body.get_curve_geom(e.curve),
            Some(topo::CurveGeom::Certified(c)) if matches!(c.carrier(), geom::Curve3::Nurbs(_))
        )),
        "the body holds the row"
    );
}

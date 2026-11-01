//! **A line beside a straight spline on one locus joins.** `split-iso`'s
//! `split_axis.step` with one of the two collinear halves of its
//! cylinder-side edge restated as a straight degree-1 B-spline: a LINE
//! and a spline on one plane × cylinder locus meeting at a vertex
//! nothing else meets. The spline ends at that vertex at its clamped
//! end, a crease of its own carrier, but the joined edge runs on the
//! line's carrier through it, which has none: the join keeps the line
//! whichever of the two the file lists first (`mixed_lower` states the
//! spline on the lower half, `mixed_upper` on the upper).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/line-beside-spline/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(p).unwrap()
}

#[test]
fn a_line_kept_over_a_straight_spline_joins_either_way_round() {
    for name in ["mixed_lower.step", "mixed_upper.step"] {
        let imported = import_step(&fixture(name), &ImportOptions::default(), Tol::witness())
            .unwrap_or_else(|e| panic!("{name} imports: {e}"));
        let StepImport::Solid {
            body,
            normalizations,
            ..
        } = imported
        else {
            panic!("{name}: a solid");
        };
        let b = body.body();
        assert_eq!(
            (b.faces().count(), b.edges().count(), b.vertices().count()),
            (4, 6, 4),
            "{name}: the split_axis census, the needless vertex joined away"
        );
        assert_eq!(
            normalizations
                .iter()
                .filter(|n| n.kind == NormalizationKind::JoinedEdges)
                .count(),
            1,
            "{name}: one join"
        );
        let splines = b
            .edges()
            .filter_map(|(_, e)| b.get_curve_geom(e.curve))
            .filter_map(topo::CurveGeom::certified)
            .filter(|c| matches!(c.carrier(), geom::Curve3::Nurbs(_)))
            .count();
        assert_eq!(splines, 0, "{name}: the joined edge runs on the line");
    }
}

/// An interior knot run of multiplicity one past the degree (a gap in
/// the spline) is refused where the carrier is read, typed, before any
/// split could see it.
#[test]
fn a_knot_run_past_the_degree_refuses_typed() {
    let text = fixture("mixed_lower.step").replace(
        "B_SPLINE_CURVE_WITH_KNOTS('',1,(#49,#54),.UNSPECIFIED.,.F.,.F.,(2,2),(0.,10.)",
        "B_SPLINE_CURVE_WITH_KNOTS('',1,(#49,#49,#54,#54),.UNSPECIFIED.,.F.,.F.,(2,2,2),(0.,5.,10.)",
    );
    assert!(text.contains("(2,2,2)"), "the gap is stated");
    let refused = import_step(&text, &ImportOptions::default(), Tol::witness());
    assert!(
        matches!(
            refused,
            Err(step_import::StepImportError::MalformedRecord { id: 52, .. })
        ),
        "the spline's own record refuses: {:?}",
        refused.err()
    );
}

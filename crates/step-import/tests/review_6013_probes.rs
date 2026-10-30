//! Review probes for PR 6013 (`crease_at` in the join). `split-iso`'s
//! `split_axis.step` with one of the two collinear halves of its
//! cylinder-side edge restated as a straight degree-1 B-spline: a LINE
//! and a spline on one plane × cylinder locus meeting at a vertex
//! nothing else meets.
//!
//! On the merge base `516450f1` `mixed_lower.step` imports 4/6/4 with
//! one `JoinedEdges` (the LINE kept, the spline the `gone` edge) and
//! `mixed_upper.step` refuses `JoinCarrierUnsupported` (the spline
//! kept). On PR 6013's head `8e3c6dab` both refuse at tier 3
//! (`VolumeUncomputable`: the unjoined spline edge's projected pcurve on
//! the cylinder): `crease_at` reads the spline's clamped end as a
//! crease and refuses the join the LINE would have carried. This row
//! pins the merge base's import, so it is red on that head.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use step_import::{ImportOptions, NormalizationKind, StepImport, import_step};

fn fixture(name: &str) -> String {
    let p = format!(
        "{}/tests/fixtures/review-6013/{name}",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(p).unwrap()
}

#[test]
fn review_6013_a_line_kept_over_a_straight_spline_still_joins() {
    let imported = import_step(
        &fixture("mixed_lower.step"),
        &ImportOptions::default(),
        Tol::witness(),
    )
    .unwrap_or_else(|e| panic!("mixed_lower imports, as on the merge base: {e}"));
    let StepImport::Solid {
        body,
        normalizations,
        ..
    } = imported
    else {
        panic!("a solid");
    };
    let b = body.body();
    assert_eq!(
        (b.faces().count(), b.edges().count(), b.vertices().count()),
        (4, 6, 4)
    );
    assert_eq!(
        normalizations
            .iter()
            .filter(|n| n.kind == NormalizationKind::JoinedEdges)
            .count(),
        1
    );
}

/// Both files' dispositions, printed (a census, never red).
#[test]
fn review_6013_a_line_and_a_straight_spline_on_one_locus_join() {
    for name in ["mixed_upper.step", "mixed_lower.step"] {
        match import_step(&fixture(name), &ImportOptions::default(), Tol::witness()) {
            Ok(StepImport::Solid {
                body,
                normalizations,
                ..
            }) => {
                let b = body.body();
                let joins = normalizations
                    .iter()
                    .filter(|n| n.kind == NormalizationKind::JoinedEdges)
                    .count();
                eprintln!(
                    "PROBE {name}: F/E/V {}/{}/{} joins {joins} kinds {:?}",
                    b.faces().count(),
                    b.edges().count(),
                    b.vertices().count(),
                    normalizations.iter().map(|n| n.kind).collect::<Vec<_>>()
                );
            }
            Ok(_) => eprintln!("PROBE {name}: not a solid"),
            Err(e) => eprintln!("PROBE {name}: refused {e}"),
        }
    }
}

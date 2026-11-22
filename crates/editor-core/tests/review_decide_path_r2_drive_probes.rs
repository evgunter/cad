//! Reviewer probe (PR 7394, claim 12): the slab's race drives at the
//! starved budgets, sequential, memo on and memo off; prints how many
//! leaves certify and the drive's frozen count. Run at the merge base,
//! the merge base with the check's walks skipped, and the head.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::analysis::{AnalysisPolicy, analyzed_box};
use editor_core::drive::{DriveConfig, drive};
use geom_core::Tol;

use crate::m10_3_r1_probes_interval::bounded_chamber;

#[test]
fn review_r2_the_race_drives_memo_on_and_off() {
    let eps = Tol::witness().eps();
    let doc = bounded_chamber(60.0 * eps, 30.0 * eps, 100.0 * eps);
    let tol = Tol::witness();
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    for (leaves, terms, degree) in [(8, 8, 4), (8, 8, 3), (12, 8, 3), (64, 8, 3)] {
        for memo in [true, false] {
            let cfg = DriveConfig {
                max_leaves: leaves,
                parallel: false,
                plain_memo: memo,
                symbolic: editor_core::drive::SymbolicDials {
                    max_terms: terms,
                    max_degree: degree,
                    ..editor_core::drive::SymbolicDials::default()
                },
                ..DriveConfig::default()
            };
            let v = drive(&doc, &analyzed, &cfg, &editor_core::Stop::default(), tol).unwrap();
            let need: Vec<u64> = v
                .certified()
                .iter()
                .map(|l| l.decisions.frozen)
                .chain(v.refused().iter().map(|l| l.decisions.frozen))
                .collect();
            println!(
                "R2PROBE leaves {leaves} budget {terms}/{degree} memo {memo}: certified {} refused {} frozen {} need {need:?}",
                v.certified().len(),
                v.refused().len(),
                v.decisions().frozen
            );
        }
    }
}

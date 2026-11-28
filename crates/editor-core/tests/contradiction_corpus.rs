//! **The contradiction instrument over the documents the tier is measured
//! on** (ERROR-DESIGN E12, "the two channels are checked where an answer
//! is returned").
//!
//! A definite numeric sign is the answer and the decision asks the tier
//! nothing about it, so whether the tier's algebra or a registration
//! would have claimed `Zero` on a margin the numbers decided is asked
//! here, after each build, by `geom_core::sym::contradiction` over the
//! decisions the re-valuation recorder kept, at `Sym<Interval>`, where a
//! report is a soundness defect in one of the two channels:
//!
//! - the five `m10_9_pins_interval::measured_studies` (the plate, the
//!   annulus, R2's link, bracket and pad), each built in an eager session
//!   at the drive's budget, rules and retry ladder over its analyzed box
//!   at the scale it certifies at (past each ceiling is
//!   `sym11_exact_channel_rows`);
//! - the slab's racing drives (`m10_sym_drive_memo_interval`), leaf by
//!   leaf: a drive's leaf sessions close inside `drive`, so the row runs
//!   the drive, then replays each leaf's box in a session it opens at the
//!   drive's dials and asks the instrument there.
//!
//! Each asserts it asked (definite decisions on nodes the session holds)
//! and found nothing. Beside it, the refuter's instrument
//! (`geom_core::sym::point::check`) climbs every undecided decision the
//! point reading refuted, and a `Zero` there is a kernel defect. The measured documents' drives, the rest of the
//! corpus and `Sym<f64>` over it are not asked
//! (`work/sym/the-contradiction-instrument-asks-no-corpus-drive-and-no-point-lane.md`
//! holds the measurements that cut them).
//!
//! In the slow set; ci.yml's `sym instrument` row runs it on a pull
//! request that touches the tier (`crates/geom-core/src/sym*`), and the
//! nightly's full suite runs it at every ε row.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, ParamBox, analyzed_box};
use editor_core::drive::{DriveConfig, SymbolicDials, drive};
use editor_core::{CancelToken, EvalOptions, Evaluation, ProfileDoc, evaluate};
use geom_core::sym::contradiction::{self, Report};
use geom_core::sym::point::{self, Refuted};
use geom_core::sym::revalue::record;
use geom_core::{Interval, Sym, SymBudget, SymCounts, Tol};

use crate::m10_9_pins_interval::measured_studies;

/// `doc` built at `Sym<Interval>` over `box_` inside an eager session at
/// `dials`, with the session's counts and the two instruments' reports of
/// its decisions.
fn instrumented(
    doc: &ProfileDoc,
    box_: ParamBox,
    dials: &SymbolicDials,
    tol: Tol,
) -> (SymCounts, Report, Refuted) {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_)),
        ..EvalOptions::default()
    };
    let budget = SymBudget {
        max_terms: dials.max_terms,
        max_degree: dials.max_degree,
    };
    let (report, counts) =
        geom_core::sym::with_session_retry(budget, dials.rules, dials.retry, || {
            let ((), decisions) = record(|| {
                let _: Evaluation<Sym<Interval>> =
                    evaluate(doc, None, &CancelToken::new(), &opts, tol);
            });
            (
                contradiction::check(&decisions).expect("inside the session"),
                point::check(&decisions).expect("inside the session"),
            )
        });
    (counts, report.0, report.1)
}

/// What `report` and `refuted` say is wrong with `what`, if anything:
/// nothing asked, a node it could not read, a contradiction, or a refuted
/// margin the tier proves zero.
fn findings(what: &str, report: &Report, refuted: &Refuted) -> Vec<String> {
    let mut found = Vec::new();
    if report.checked == 0 {
        found.push(format!("{what}: no definite decision was asked"));
    }
    if report.absent != 0 {
        found.push(format!(
            "{what}: {} definite decisions on nodes the session does not hold",
            report.absent
        ));
    }
    for (predicate, node) in &refuted.zeros {
        found.push(format!(
            "{what}: `{predicate}` was refuted at the point and the tier proves it zero \
             (node {:032x})",
            node.bits()
        ));
    }
    for c in &report.contradictions {
        found.push(format!(
            "{what}: decision {} `{}` decided {:?} and the tier proves it zero ({:?}, {:?} rung, \
             attempt {})",
            c.index, c.predicate, c.sign, c.event, c.rung, c.attempt
        ));
    }
    found
}

/// The measured documents over their analyzed boxes.
#[test]
fn the_measured_documents_contradict_nothing_at_sym_interval() {
    let tol = Tol::witness();
    let mut found = Vec::new();
    let mut asked_refuted = 0;
    for study in measured_studies(tol) {
        let doc = (study.at)(study.certifies_at * tol.eps());
        let box_ = ParamBox::of(&analyzed_box(&doc, &AnalysisPolicy::default()));
        let (counts, report, refuted) = instrumented(&doc, box_, &SymbolicDials::default(), tol);
        println!("  {}: {counts:?} | {report:?} | {refuted:?}", study.name);
        found.extend(findings(study.name, &report, &refuted));
        asked_refuted += refuted.refuted;
    }
    assert!(asked_refuted > 0, "no refuted decision was climbed");
    assert!(
        found.is_empty(),
        "the instrument found the two channels in contradiction, or asked nothing:\n{}",
        found.join("\n")
    );
}

/// The slab's three racing drives, every leaf replayed and asked. The
/// replayed leaf takes the drive's decisions (its symbolic and numeric
/// counts are the leaf's own, so the replay is the leaf and not a
/// neighbour of it).
#[test]
fn the_slabs_drives_contradict_nothing_leaf_by_leaf() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let doc = crate::m10_3_r1_probes_interval::bounded_chamber(60.0 * eps, 30.0 * eps, 100.0 * eps);
    let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
    let mut found = Vec::new();
    for (leaves, terms, degree) in [(8, 8, 3), (12, 8, 3), (64, 8, 3)] {
        let config = DriveConfig {
            max_leaves: leaves,
            symbolic: SymbolicDials {
                max_terms: terms,
                max_degree: degree,
                ..SymbolicDials::default()
            },
            ..DriveConfig::default()
        };
        let verdict = drive(&doc, &analyzed, &config, &editor_core::Stop::default(), tol).unwrap();
        let leaves_of = verdict
            .certified()
            .iter()
            .map(|l| (l.box_.clone(), l.decisions))
            .chain(
                verdict
                    .refused()
                    .iter()
                    .map(|l| (l.box_.clone(), l.decisions)),
            );
        let mut asked = 0;
        for (n, (box_, drove)) in leaves_of.enumerate() {
            let (counts, report, refuted) = instrumented(&doc, box_, &config.symbolic, tol);
            let what = format!("{leaves}-leaf drive at {terms}/{degree}, leaf {n}");
            assert_eq!(
                (counts.symbolic, counts.numeric),
                (drove.symbolic, drove.numeric),
                "{what}: the replay decided as the drive's leaf did"
            );
            found.extend(findings(&what, &report, &refuted));
            asked += 1;
        }
        println!(
            "  {leaves}-leaf drive: {asked} leaves asked, {:?}",
            verdict.receipt()
        );
    }
    assert!(
        found.is_empty(),
        "the instrument found the two channels in contradiction, or asked nothing:\n{}",
        found.join("\n")
    );
}

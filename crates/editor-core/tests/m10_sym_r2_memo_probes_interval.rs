//! **SYM-7 R2 probes** — the drive-scoped plain memo exercised on a
//! document the unit did not measure, across schedules AND thread
//! counts, with the dial on and off.
//!
//! The unit's own schedule row and the M10-3 receipt-identity row both
//! drive the SLAB, whose drive freezes nothing (`frozen` 0 on both
//! lanes), so the one receipt column the unit moved — `frozen`, now the
//! distinct nodes frozen over the drive — is pinned across schedules
//! only at zero. The rows here drive documents that FREEZE (the plate;
//! a boss on a derived frame over a tilted datum) sequentially, in
//! parallel on one, two and four workers, and with the dial off, and
//! ask that everything be byte-identical with `frozen` non-zero.
//!
//! The basename carries `interval` so `ci-filter.py` pins the lane.
#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-core/src/sym.rs",
    "crates/geom-core/src/sym/",
    "crates/editor-core/src/drive.rs",
    "crates/editor-core/tests/fixture/",
    "crates/editor-core/tests/m10_derived_frame_tilted_interval.rs",
    "crates/editor-core/tests/m10_7_plate.rs",
];

use std::time::Instant;

use editor_core::ProfileDoc;
use editor_core::analysis::{AnalysisPolicy, AnalyzedBox, analyzed_box};
use editor_core::drive::{DriveConfig, ParamBoxVerdict, drive};
use geom_core::{SymCounts, Tol};

use crate::m10_7_plate::plate;
use crate::m10_derived_frame_tilted_interval::boss_on_tilted;

/// The leaf budget the rows drive at — the unit's own pin budget.
const LEAVES: usize = 48;

/// A drive on a rayon pool of `threads` workers; `install` binds the
/// pool for the closure, so `drive`'s `par_iter` runs on it
/// (`sweep/tests/common::on_pool`'s shape).
fn on_pool<R: Send>(threads: usize, run: impl Fn() -> R + Send + Sync) -> R {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("the pool builds")
        .install(run)
}

fn run(
    doc: &ProfileDoc,
    analyzed: &AnalyzedBox,
    parallel: bool,
    plain_memo: bool,
) -> ParamBoxVerdict {
    let cfg = DriveConfig {
        max_leaves: LEAVES,
        parallel,
        plain_memo,
        ..DriveConfig::default()
    };
    drive(doc, analyzed, &cfg, Tol::witness()).unwrap()
}

/// The decision columns without `frozen`, which is a per-leaf WORK
/// measure the memo moves by design (`SymCounts::frozen`).
fn decisions_of(c: SymCounts) -> SymCounts {
    SymCounts { frozen: 0, ..c }
}

fn assert_same(label: &str, a: &ParamBoxVerdict, b: &ParamBoxVerdict) {
    assert_eq!(a.serialize(), b.serialize(), "{label}: serialization");
    assert_eq!(a.content_key(), b.content_key(), "{label}: content key");
    assert_eq!(a.receipt(), b.receipt(), "{label}: receipt");
    assert_eq!(
        a.decisions(),
        b.decisions(),
        "{label}: drive decisions, frozen included"
    );
    assert_eq!(
        a.certified().len(),
        b.certified().len(),
        "{label}: certified count"
    );
    assert_eq!(
        a.refused().len(),
        b.refused().len(),
        "{label}: refused count"
    );
    for (i, (x, y)) in a.certified().iter().zip(b.certified()).enumerate() {
        assert_eq!(x.box_, y.box_, "{label}: certified {i} box");
        assert_eq!(
            x.verdict_vector_key, y.verdict_vector_key,
            "{label}: certified {i} key"
        );
        assert_eq!(x.results, y.results, "{label}: certified {i} results");
        assert_eq!(
            decisions_of(x.decisions),
            decisions_of(y.decisions),
            "{label}: certified {i} decisions"
        );
    }
    for (i, (x, y)) in a.refused().iter().zip(b.refused()).enumerate() {
        assert_eq!(x.box_, y.box_, "{label}: refused {i} box");
        assert_eq!(x.reason, y.reason, "{label}: refused {i} reason");
        assert_eq!(
            decisions_of(x.decisions),
            decisions_of(y.decisions),
            "{label}: refused {i} decisions"
        );
    }
    // The memo's SIZE is a function of the drive too when the dial is
    // the same on both sides (a set of forms and a set of frozen ids).
    if a.plain_memo().forms > 0 && b.plain_memo().forms > 0 {
        assert_eq!(a.plain_memo(), b.plain_memo(), "{label}: memo size");
    }
}

fn leaves_own_frozen(v: &ParamBoxVerdict) -> u64 {
    v.certified()
        .iter()
        .map(|l| l.decisions.frozen)
        .chain(v.refused().iter().map(|l| l.decisions.frozen))
        .sum()
}

/// **R2's own document, and the plate, across schedules, thread counts
/// and the dial** — every receipt byte-identical, `frozen` non-zero.
#[test]
fn r2_a_freezing_drive_is_identical_across_schedules_thread_counts_and_the_dial() {
    let tol = Tol::witness();
    for (label, doc) in [
        ("tilted-derived", boss_on_tilted(1.0e-3, true)),
        ("plate", plate(5.0e-5, 1.0e-5, tol).0),
    ] {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        let t0 = Instant::now();
        let seq_on = run(&doc, &analyzed, false, true);
        let t_seq_on = t0.elapsed();
        println!(
            "{label}: seq memo-on wall {t_seq_on:?} receipt {:?} decisions {:?} memo {:?} \
             leaves' own frozen {}",
            seq_on.receipt(),
            seq_on.decisions(),
            seq_on.plain_memo(),
            leaves_own_frozen(&seq_on)
        );
        assert!(
            seq_on.decisions().frozen > 0,
            "{label}: the row is about the moved column and it must be non-zero here"
        );
        assert!(
            seq_on.plain_memo().forms > 0,
            "{label}: the memo held forms"
        );

        let t0 = Instant::now();
        let seq_off = run(&doc, &analyzed, false, false);
        println!(
            "{label}: seq memo-off wall {:?} memo {:?} leaves' own frozen {}",
            t0.elapsed(),
            seq_off.plain_memo(),
            leaves_own_frozen(&seq_off)
        );
        assert_same(&format!("{label} seq on/off"), &seq_on, &seq_off);

        for threads in [1, 2, 4] {
            let t0 = Instant::now();
            let par_on = on_pool(threads, || run(&doc, &analyzed, true, true));
            println!(
                "{label}: par@{threads} memo-on wall {:?} leaves' own frozen {}",
                t0.elapsed(),
                leaves_own_frozen(&par_on)
            );
            assert_same(
                &format!("{label} par@{threads} on vs seq on"),
                &par_on,
                &seq_on,
            );
        }
        let par2_off = on_pool(2, || run(&doc, &analyzed, true, false));
        assert_same(&format!("{label} par@2 off vs seq on"), &par2_off, &seq_on);
        let par2_on_again = on_pool(2, || run(&doc, &analyzed, true, true));
        assert_same(&format!("{label} par@2 on twice"), &par2_on_again, &seq_on);
    }
}

/// **Is the unit's opaque-sequence pin vacuous?** The unit's row asserts
/// every leaf's `Opaque` set equals leaf 0's and never that any set is
/// non-empty; two empty sets agree. This row prints the sizes on both
/// documents at the pin budget and asks that at least one leaf of each
/// minted one.
#[test]
fn r2_the_opaque_sets_the_premise_row_compares_are_not_all_empty() {
    use geom_core::sym::profile::{start_profile, take_profile};
    let tol = Tol::witness();
    let mut empty = Vec::new();
    for (label, doc) in [
        (
            "slab",
            crate::m10_3_r1_probes_interval::bounded_chamber(
                60.0 * tol.eps(),
                30.0 * tol.eps(),
                100.0 * tol.eps(),
            ),
        ),
        ("plate", plate(5.0e-5, 1.0e-5, tol).0),
    ] {
        let analyzed = analyzed_box(&doc, &AnalysisPolicy::default());
        start_profile();
        drive(
            &doc,
            &analyzed,
            &DriveConfig {
                max_leaves: LEAVES,
                ..DriveConfig::default()
            },
            tol,
        )
        .unwrap();
        let sets = take_profile().opaque_ids;
        let total: usize = sets.iter().map(std::collections::BTreeSet::len).sum();
        println!(
            "{label}: {} leaves (sessions), Opaque ids over every leaf: {total}",
            sets.len()
        );
        if total == 0 {
            empty.push(label);
        }
    }
    assert!(
        empty.is_empty(),
        "{empty:?}: every leaf's Opaque set is EMPTY, so the premise row compares nothing there"
    );
}

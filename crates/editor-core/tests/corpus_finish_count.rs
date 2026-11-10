//! **How many finishes the corpus runs**: every registered corpus
//! document evaluated at `f64` and run through the default checks,
//! counting each certifying finish where it runs
//! (`topo::finishes_on_this_thread`). The count is deterministic, so it
//! is pinned. A finished body moved by an isometry is not finished
//! again (`topo::FinishedBody::moved`), and a move of the count either
//! way is a change in where the kernel derives verdicts, to be read
//! and stated, not silenced.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use editor_core::ChecksConfig;
use geom_core::Tol;

/// The corpus's finishes, per document in the registry's order.
#[cfg(debug_assertions)]
fn finishes() -> Vec<(&'static str, u64)> {
    corpus::documents()
        .into_iter()
        .map(|d| {
            let before = topo::finishes_on_this_thread();
            let run = corpus::eval::<f64>(&d.doc);
            editor_core::run_checks(
                &d.doc,
                &run,
                &ChecksConfig::default(),
                &editor_core::Stop::default(),
                Tol::witness(),
            )
            .unwrap_or_else(|e| panic!("{}: the checks run: {e}", d.name));
            (d.name, topo::finishes_on_this_thread() - before)
        })
        .collect()
}

#[cfg(debug_assertions)]
#[test]
fn the_corpus_runs_its_pinned_count_of_finishes() {
    let per_document = finishes();
    let total: u64 = per_document.iter().map(|(_, n)| n).sum();
    assert_eq!(
        total, PINNED,
        "the corpus ran {total} finishes, pinned {PINNED}: {per_document:?}"
    );
}

/// The count at the default ε. PR 6371 measured 505 by hand over the
/// same corpus before a finished body's move carried its verdict.
#[cfg(debug_assertions)]
const PINNED: u64 = 389;

//! **The parallel node map records the serial walk's `probe` samples, at
//! any thread count.**
//!
//! `EvalOptions::parallel` runs each level's nodes as an indexed map on
//! rayon workers. The `probe` sample sink is a thread-local installed on
//! the CALLER's thread, so a node decided on a worker records into that
//! worker's sink unless the map hands its recording back and the fold
//! splices it. The margin population `docs/K-REPORT.md` and
//! `tools/k-lint` are computed from is then a property of the document,
//! not of the schedule: element for element the serial walk's, in the
//! serial walk's order, at one thread and at four.
//!
//! The document is `fixture::two_blocks_and_their_union`, whose serial
//! order and level schedule disagree, so a fold that spliced level by
//! level — the same population in a schedule-shaped order — reds here as
//! surely as one that dropped a worker's samples.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{on_pool, two_blocks_and_their_union};

use editor_core::{CancelToken, EvalOptions, Evaluation, NodeResult, evaluate};
use geom_core::Tol;
use geom_core::k_stats::{self, MarginSample, Probe};

/// One sample, with every float read by its bits: a NaN margin (an
/// `Invalid` outcome) is equal to itself here, as it is to the
/// recording.
fn bits(s: &MarginSample) -> (&'static str, u64, u64, u64, String) {
    (
        s.predicate,
        s.margin.to_bits(),
        s.band_zero.to_bits(),
        s.band_escalate.to_bits(),
        format!("{:?}", s.outcome),
    )
}

/// The document evaluated at `Probe` under `parallel`, and every sample
/// the caller's sink received.
fn recorded(parallel: bool) -> (Evaluation<Probe>, Vec<MarginSample>) {
    let (doc, _) = two_blocks_and_their_union("parallel-node-map-probe");
    let opts = EvalOptions {
        parallel,
        ..EvalOptions::default()
    };
    k_stats::start_recording();
    let ev = evaluate::<Probe>(&doc, None, &CancelToken::new(), &opts, Tol::witness());
    (ev, k_stats::take_samples())
}

#[test]
fn the_sample_population_is_the_serial_walks_at_one_and_four_threads() {
    let (serial_ev, serial) = recorded(false);
    for (&id, result) in &serial_ev.nodes {
        assert!(
            matches!(result, NodeResult::Ok(_)),
            "node {} must evaluate for the population to mean anything",
            id.0
        );
    }
    assert!(
        !serial.is_empty(),
        "the serial walk recorded no sample: this row compares nothing"
    );
    let serial: Vec<_> = serial.iter().map(bits).collect();
    for threads in [1, 4] {
        let (_, parallel) = on_pool(threads, || recorded(true));
        let parallel: Vec<_> = parallel.iter().map(bits).collect();
        assert_eq!(
            parallel.len(),
            serial.len(),
            "{threads} thread(s): the parallel schedule recorded {} samples against the serial \
             walk's {}",
            parallel.len(),
            serial.len()
        );
        assert!(
            parallel == serial,
            "{threads} thread(s): the parallel schedule recorded the serial walk's samples in \
             another order (first difference at {:?})",
            parallel.iter().zip(&serial).position(|(a, b)| a != b)
        );
    }
}

//! **Under a symbolic session or a shape report the parallel node map is
//! the serial walk.**
//!
//! A symbolic session (`geom_core::sym::with_session_rules`) and the
//! shape report (`geom_core::sym::report`) are thread-locals of the
//! CALLING thread that a decision reads or writes in place: a node
//! decided on a rayon worker would decide without the identity tier,
//! count nothing into the session's receipt and report nothing. Neither
//! composes back the way a K-funnel frame does, so `evaluate` asks
//! `geom_core::sym::decisions_are_thread_portable` and walks serially
//! while either is installed. What that buys, read here at an explicit
//! one thread and four:
//!
//! * **the session's decisions and receipt** — every node's content key
//!   and verdict log, and the `SymCounts` the session returns, are the
//!   serial walk's;
//! * **the shape report** — installed with no session, every decision's
//!   row is the serial walk's, in its order.
//!
//! The two widths catch different things. On a one-worker pool the map
//! runs inline on the thread that installed the session, so the session
//! row passes there with or without the fallback and it is the
//! four-thread row that shows the decisions depended on the width. The
//! report row reds at one thread too: the map visits the nodes level by
//! level, which is not the serial order on this document.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{on_pool, two_blocks_and_their_union};

use editor_core::{CancelToken, ContentKey, EvalOptions, Evaluation, NodeResult, evaluate};
use geom_core::k_stats::Verdict;
use geom_core::sym::report::{DecisionShape, start_shape_report, take_shape_report};
use geom_core::{Interval, Sym, SymBudget, SymCounts, SymRules, Tol};

type Lane = Sym<Interval>;

/// Every node's content key and verdict log, in node order — what "the
/// same decisions" means for a whole evaluation.
fn decisions(ev: &Evaluation<Lane>) -> Vec<(u64, ContentKey, Vec<Verdict>)> {
    ev.nodes
        .iter()
        .map(|(&id, result)| match result {
            NodeResult::Ok(v) => (id.0, v.content_key, v.verdicts.to_vec()),
            NodeResult::Failed(e) => panic!("node {} failed: {}", id.0, e.kind),
            NodeResult::Poisoned { through } => {
                panic!("node {} poisoned through node {}", id.0, through.0)
            }
        })
        .collect()
}

fn opts(parallel: bool) -> EvalOptions {
    EvalOptions {
        parallel,
        ..EvalOptions::default()
    }
}

/// The document evaluated at `Sym<Interval>` inside a session: its
/// decisions and the session's receipt.
fn in_a_session(parallel: bool) -> (Vec<(u64, ContentKey, Vec<Verdict>)>, SymCounts) {
    let (doc, _) = two_blocks_and_their_union("parallel-node-map-session");
    let budget = SymBudget {
        max_terms: editor_core::drive::DEFAULT_SYM_MAX_TERMS,
        max_degree: editor_core::drive::DEFAULT_SYM_MAX_DEGREE,
    };
    let (decided, counts) = geom_core::sym::with_session_rules(budget, SymRules::shipped(), || {
        let ev = evaluate::<Lane>(
            &doc,
            None,
            &CancelToken::new(),
            &opts(parallel),
            Tol::witness(),
        );
        decisions(&ev)
    });
    (decided, counts)
}

#[test]
fn a_session_decides_and_counts_as_the_serial_walk_at_one_and_four_threads() {
    let (serial, serial_counts) = in_a_session(false);
    assert!(
        serial_counts.decisions() > 0,
        "the session counted no decision: this row compares nothing"
    );
    for threads in [1, 4] {
        let (parallel, counts) = on_pool(threads, || in_a_session(true));
        assert_eq!(
            counts, serial_counts,
            "{threads} thread(s): the session's receipt under the parallel schedule"
        );
        assert!(
            parallel == serial,
            "{threads} thread(s): a node decided differently under the parallel schedule"
        );
    }
}

/// One report row, floats by their bits.
fn row(s: &DecisionShape) -> String {
    format!(
        "{} {:?} {:?} {:?}",
        s.predicate,
        s.outcome,
        s.form,
        s.enclosure.map(|(lo, hi)| (lo.to_bits(), hi.to_bits()))
    )
}

/// The document evaluated at `Sym<Interval>` with the shape report
/// installed and NO session, and the report's rows.
fn reported(parallel: bool) -> Vec<String> {
    let (doc, _) = two_blocks_and_their_union("parallel-node-map-report");
    start_shape_report();
    let ev = evaluate::<Lane>(
        &doc,
        None,
        &CancelToken::new(),
        &opts(parallel),
        Tol::witness(),
    );
    let rows = take_shape_report();
    // Every node evaluated: `decisions` refuses a failed or poisoned one.
    let _ = decisions(&ev);
    rows.iter().map(row).collect()
}

#[test]
fn the_shape_report_is_the_serial_walks_at_one_and_four_threads() {
    let serial = reported(false);
    assert!(
        !serial.is_empty(),
        "the report recorded no decision: this row compares nothing"
    );
    for threads in [1, 4] {
        let parallel = on_pool(threads, || reported(true));
        assert_eq!(
            parallel.len(),
            serial.len(),
            "{threads} thread(s): the shape report's row count under the parallel schedule"
        );
        assert!(
            parallel == serial,
            "{threads} thread(s): the shape report's rows under the parallel schedule (first \
             difference at {:?})",
            parallel.iter().zip(&serial).position(|(a, b)| a != b)
        );
    }
}

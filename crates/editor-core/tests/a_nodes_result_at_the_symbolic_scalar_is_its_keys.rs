//! **A node's result at the symbolic scalar is a function of its key**,
//! on a document whose replays run the retry ladder and on one whose
//! constructors register identities.
//!
//! The rows replay a document's leaves as the driver does, each in a
//! session of its own on one drive's table, and read every node's
//! result: its verdict rows, escalations and marks, and what it left in
//! the symbolic session (`NodeValue::sym`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use editor_core::analysis::{AnalysisPolicy, BoxAxis, ParamBox, analyzed_box};
use editor_core::drive::{
    DEFAULT_SYM_MAX_DEGREE, DEFAULT_SYM_MAX_TERMS, DriveConfig, ParamBoxVerdict, drive,
};
use editor_core::{CancelToken, EvalOptions, Evaluation, NodeResult, ProfileDoc, evaluate};
use geom_core::sym::{DriveMemo, with_session_memo_retry};
use geom_core::{Interval, Sym, SymBudget, SymCounts, SymRetry, SymRules, Tol};

fn budget() -> SymBudget {
    SymBudget {
        max_terms: DEFAULT_SYM_MAX_TERMS,
        max_degree: DEFAULT_SYM_MAX_DEGREE,
    }
}

/// A document whose replays run the retry ladder: R2's filleted
/// bracket, at ten ε.
fn retrying(tol: Tol) -> ProfileDoc {
    crate::m10_7_r2_probes_interval::bracket(1.0e1 * tol.eps(), tol).0
}

/// A document whose constructors register identities: the two-hole
/// plate, whose arcs state their rims.
fn registering(tol: Tol) -> ProfileDoc {
    crate::m10_7_plate::plate(5.0e-5, 1.0e-5, tol).0
}

/// The document's box and, for each axis that varies, its two halves:
/// a node that does not read an axis has one key in both.
fn boxes(doc: &ProfileDoc) -> Vec<ParamBox> {
    let root = ParamBox::of(&analyzed_box(doc, &AnalysisPolicy::default()));
    let halves = root.varying().flat_map(|(var, lo, hi)| {
        let mid = 0.5 * (lo + hi);
        [(lo, mid), (mid, hi)].map(|(lo, hi)| {
            let mut axes = root.axes().clone();
            axes.insert(var, BoxAxis::Varying { lo, hi });
            ParamBox::from_axes(axes)
        })
    });
    core::iter::once(root.clone()).chain(halves).collect()
}

/// One leaf replayed on the drive's table, as `drive::classify` does.
fn leaf(
    doc: &ProfileDoc,
    box_: &ParamBox,
    table: &Arc<DriveMemo>,
    tol: Tol,
) -> (Evaluation<Sym<Interval>>, SymCounts) {
    let opts = EvalOptions {
        param_box: Some(Arc::new(box_.clone())),
        ..EvalOptions::default()
    };
    with_session_memo_retry(
        budget(),
        SymRules::shipped(),
        SymRetry::kept_atom(),
        table,
        || evaluate(doc, None, &CancelToken::new(), &opts, tol),
    )
}

/// What a row reads off a document's replays.
#[derive(Debug, Default)]
struct Read {
    /// Under each (node, content key, naming key), every distinct
    /// result text.
    results: BTreeMap<String, BTreeSet<String>>,
    /// Results read under a key already seen.
    repeats: usize,
    retried: u64,
    registrations: usize,
    /// Per leaf: the session's counts, and the sum of its nodes'
    /// records.
    receipts: Vec<(SymCounts, SymCounts)>,
}

fn read(doc: &ProfileDoc, tol: Tol) -> Read {
    let table = Arc::new(DriveMemo::new(budget(), SymRules::shipped()));
    let mut out = Read::default();
    for box_ in boxes(doc) {
        let (ev, session) = leaf(doc, &box_, &table, tol);
        let mut nodes = SymCounts::default();
        for id in &ev.order {
            let record = match ev.result(*id) {
                Some(NodeResult::Ok(v)) => {
                    let key = format!("{id:?} {:?} {:?}", v.content_key, v.naming_key);
                    let text = format!(
                        "{:?} | {:?} | {:?} | {:?}",
                        v.verdicts, v.escalations, v.marks, v.sym
                    );
                    let seen = out.results.entry(key).or_default();
                    out.repeats += usize::from(!seen.is_empty());
                    seen.insert(text);
                    &v.sym
                }
                Some(NodeResult::Failed(e)) => &e.sym,
                Some(NodeResult::Poisoned { .. }) | None => continue,
            };
            nodes.absorb(record.counts());
            out.retried += record.counts().retried;
            out.registrations += record.registrations();
        }
        out.receipts.push((
            SymCounts {
                frozen: 0,
                ..session
            },
            nodes,
        ));
    }
    out
}

fn assert_one_result_per_key(label: &str, read: &Read) {
    assert!(
        read.repeats > 0,
        "{label}: no node repeats a key across the leaves"
    );
    for (key, results) in &read.results {
        assert_eq!(
            results.len(),
            1,
            "{label}: node {key} has {} results under one key",
            results.len()
        );
    }
}

/// **One key, one result, where the retry ladder runs.** Every node of
/// the bracket that has one content key in two leaves has one
/// result in them: the same verdict rows, escalations and marks, and
/// the same symbolic record, its retried decisions among them.
///
/// Mutant: one retry guard over the replay (the nodes sharing the
/// attempts' memos) under a guard small enough to bind gives a node a
/// different record after a node that filled the memos.
#[test]
fn one_key_one_result_in_a_drive_that_retries() {
    let tol = Tol::witness();
    let read = read(&retrying(tol), tol);
    println!(
        "retrying: {} keys, {} repeats, retried {}, registrations {}",
        read.results.len(),
        read.repeats,
        read.retried,
        read.registrations
    );
    assert!(read.retried > 0, "the ladder closed a decision");
    assert_one_result_per_key("retrying", &read);
}

/// **One key, one result, where identities are registered**: the plate.
///
/// Mutant: a node's scope opened over every earlier node's
/// registrations gives a node that re-builds a registered expression a
/// record that depends on what ran before it.
#[test]
fn one_key_one_result_in_a_drive_that_registers() {
    let tol = Tol::witness();
    let read = read(&registering(tol), tol);
    println!(
        "registering: {} keys, {} repeats, retried {}, registrations {}",
        read.results.len(),
        read.repeats,
        read.retried,
        read.registrations
    );
    assert!(read.registrations > 0, "a constructor registered");
    assert_one_result_per_key("registering", &read);
}

/// **A leaf's receipt is the sum of its nodes' records**: nothing is
/// decided at a symbolic scalar outside a node's scope, so the counts a
/// leaf's session answers are the counts its node results carry.
#[test]
fn a_leafs_receipt_is_the_sum_of_its_nodes_records() {
    let tol = Tol::witness();
    for (label, doc) in [
        ("retrying", retrying(tol)),
        ("registering", registering(tol)),
    ] {
        for (i, (session, nodes)) in read(&doc, tol).receipts.iter().enumerate() {
            assert!(session.decisions() > 0, "{label} leaf {i} decided");
            assert_eq!(session, nodes, "{label} leaf {i}");
        }
    }
}

fn driven(doc: &ProfileDoc, parallel: bool, plain_memo: bool, tol: Tol) -> ParamBoxVerdict {
    drive(
        doc,
        &analyzed_box(doc, &AnalysisPolicy::default()),
        &DriveConfig {
            max_leaves: 8,
            parallel,
            plain_memo,
            ..DriveConfig::default()
        },
        &editor_core::Stop::default(),
        tol,
    )
    .expect("the document drives")
}

/// **One thread against many, and the drive's table serving against
/// not**: the verdict, every leaf with its receipt, and the serialized
/// drive are the same, on the retrying document and the registering
/// one.
#[test]
fn a_drive_is_the_same_on_one_thread_or_many_and_with_the_table_or_without() {
    let tol = Tol::witness();
    // The bracket past the box it certifies whole, so the drive splits.
    let wide_bracket = crate::m10_7_r2_probes_interval::bracket(1.0e3 * tol.eps(), tol).0;
    for (label, doc) in [
        ("retrying", wide_bracket),
        ("registering", registering(tol)),
    ] {
        let reference = driven(&doc, false, true, tol);
        assert!(
            reference.certified().len() + reference.refused().len() > 1,
            "{label}: the drive has more than one leaf"
        );
        assert!(
            label != "retrying" || reference.decisions().retried > 0,
            "the ladder closed a decision in the retrying drive"
        );
        for (parallel, plain_memo) in [(true, true), (false, false), (true, false)] {
            let other = driven(&doc, parallel, plain_memo, tol);
            let what = format!("{label}: parallel {parallel}, the table serving {plain_memo}");
            assert_eq!(reference.serialize(), other.serialize(), "{what}");
            assert_eq!(reference.certified(), other.certified(), "{what}");
            assert_eq!(reference.refused(), other.refused(), "{what}");
        }
    }
}

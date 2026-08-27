//! **Failed and Poisoned badges carry the TYPED payload's message.**
//!
//! GQ2's per-node result DAG is the whole subject: a failing node is
//! `Failed(NodeError)` and its descendants are `Poisoned { through }`,
//! independent subgraphs complete, and what a badge says is the typed
//! error's own rendering rather than a sentence this crate wrote.
//!
//! The row that matters most is the last one: the poisoned badge's
//! message must be the FAILED ANCESTOR's error. A tree that showed
//! "upstream failed" and nothing else would be green under every other
//! assertion here and useless to a user.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{CancelToken, EvalOptions, NodeResult, evaluate};
use pncad::geom_core::Tol;
use viewer::session::DocSession;
use viewer::tree::{self, RowStatus};

#[test]
fn a_failing_document_renders_failed_and_poisoned_from_the_typed_payloads() {
    let tol = Tol::witness();
    let (doc, extrude, moved) = common::broken_document(tol);
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let rows = session.tree_rows();
    assert!(tree::has_faults(&rows));

    let failed = rows
        .iter()
        .find(|row| row.id == extrude)
        .expect("the extrude has a row");
    let RowStatus::Failed { message } = &failed.status else {
        panic!("expected Failed, got {:?}", failed.status);
    };
    assert_eq!(failed.status.badge(), "FAILED");
    assert!(!message.is_empty());

    // The message is the payload's, not this crate's: it must be
    // byte-identical to the shipped error's own rendering.
    let evaluation = session.evaluation().expect("a result landed");
    let Some(NodeResult::Failed(error)) = evaluation.result(extrude) else {
        panic!("the evaluation should report the extrude as failed");
    };
    assert_eq!(message, &error.to_string());

    let poisoned = rows
        .iter()
        .find(|row| row.id == moved)
        .expect("the transform has a row");
    match &poisoned.status {
        RowStatus::Poisoned { through, message } => {
            assert_eq!(*through, extrude, "poison names the failure it came from");
            assert_eq!(
                message.as_deref(),
                Some(error.to_string().as_str()),
                "a poisoned row shows the ROOT CAUSE, not a placeholder"
            );
        }
        other => panic!("expected Poisoned, got {other:?}"),
    }
    assert_eq!(poisoned.status.badge(), "POISONED");
}

#[test]
fn an_independent_subgraph_completes_beside_a_failure() {
    let tol = Tol::witness();
    let (doc, extrude, _moved) = common::broken_document(tol);
    // A second, unrelated body in the same document: GQ2's ratified
    // "a failure poisons only its descendants".
    let (doc, other_profile) = common::inserted(&doc, common::square(0.02), tol);
    let (doc, other_extrude) = common::inserted(
        &doc,
        pncad::document::Node::Extrude {
            profile: other_profile,
            distance: common::len(0.005),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    let rows = session.tree_rows();

    let status_of = |id| {
        rows.iter()
            .find(|row| row.id == id)
            .map(|row| row.status.clone())
            .expect("the node has a row")
    };
    assert!(matches!(status_of(extrude), RowStatus::Failed { .. }));
    assert_eq!(status_of(other_extrude), RowStatus::Ok);
    assert_eq!(status_of(other_profile), RowStatus::Ok);
}

#[test]
fn rows_before_the_first_result_read_as_unevaluated_rather_than_ok() {
    let tol = Tol::witness();
    let (doc, _profile, extrude) = common::parametric_plate(tol);
    let session = DocSession::inline(doc, tol);
    // Deliberately NOT pumped: nothing has been evaluated yet.
    let rows = session.tree_rows();
    assert!(rows.iter().all(|row| row.status == RowStatus::Unevaluated));
    assert!(
        !tree::has_faults(&rows),
        "unevaluated is not a fault; it is an absence of measurement"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.id == extrude)
            .map(|row| row.status.badge()),
        Some("—")
    );
}

#[test]
fn a_canceled_runs_missing_tail_reads_as_unevaluated() {
    // A canceled evaluation carries the completed PREFIX only, so
    // nodes past it have no entry at all. The tree must show that as
    // an absence rather than as success.
    let tol = Tol::witness();
    let (doc, _profile, _extrude) = common::parametric_plate(tol);
    let cancel = CancelToken::new();
    cancel.cancel();
    let evaluation = evaluate::<f64>(&doc, None, &cancel, &EvalOptions::default(), tol);
    let rows = tree::rows(&doc, Some(&evaluation));
    assert!(!rows.is_empty());
    assert!(rows.iter().all(|row| row.status == RowStatus::Unevaluated));
    assert!(!tree::has_faults(&rows));
}

#[test]
fn the_tree_marks_the_documents_product_roots() {
    let tol = Tol::witness();
    let (doc, profile, extrude) = common::parametric_plate(tol);
    let rows = tree::rows(&doc, None);
    let root_ids: Vec<_> = rows
        .iter()
        .filter(|row| row.root)
        .map(|row| row.id)
        .collect();
    assert_eq!(
        root_ids,
        vec![extrude],
        "the extrude is the product; the profile it consumes is not"
    );
    assert_eq!(
        rows.iter()
            .find(|row| row.id == profile)
            .map(|row| row.kind),
        Some("Profile")
    );
}

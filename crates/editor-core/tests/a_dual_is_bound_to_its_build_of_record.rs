//! **A dual evaluates only bound to its build of record** (DUAL-DESIGN
//! DL3). The tier-3 premise a dual operand carries no evidence of is
//! asserted here, at the evaluation service: `evaluate_recorded` refuses
//! every node its record built no value at, before the node runs, so no
//! topo reading a dual reaches leans on a gate the record failed
//! (`topo::Replay`'s "A tier-3 premise at a dual"). The same binding
//! inside a part is pinned beside the part fixtures, in
//! `asm2a_instantiate`.

#![allow(clippy::panic, clippy::expect_used)]

use editor_core::{
    CancelToken, EvalOptions, EvalOutcome, Evaluation, NodeError, NodeErrorKind, NodeResult,
    RecordMispairing, evaluate, evaluate_recorded,
};
use geom_core::{Dual64, Tol};

fn nominal(doc: &editor_core::ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default().of_record(),
        Tol::witness(),
    )
}

fn dual(doc: &editor_core::ProfileDoc, record: &Evaluation<f64>) -> Evaluation<Dual64> {
    evaluate_recorded(
        doc,
        None,
        record,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A two-node chain, `label`'s: a square profile and its extrusion,
/// which reads it. The record's refusal at the profile must reach both.
/// Built here rather than found in the corpus, so each row evaluates one
/// small document.
fn a_chain_of(label: &str) -> (editor_core::ProfileDoc, editor_core::RecipeNodeId) {
    use crate::fixture::{insert, len, on_frame, square};
    let doc =
        editor_core::ProfileDoc::empty(editor_core::DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, _) = insert(
        doc,
        editor_core::Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let record = nominal(&doc);
    assert!(
        record.order.len() >= 2
            && record
                .nodes
                .values()
                .all(|r| matches!(r, NodeResult::Ok(_))),
        "the chain builds"
    );
    let first = record.order[0];
    (doc, first)
}

fn a_chain() -> (editor_core::ProfileDoc, editor_core::RecipeNodeId) {
    a_chain_of("a-dual-is-bound-chain")
}

/// **A node the record refused is refused at the dual by the record**,
/// typed, and every node downstream is poisoned through it. Where the
/// record built every node, the dual does too: the binding refuses
/// only what the record refused.
#[test]
fn a_node_the_record_refused_is_refused_at_the_dual() {
    let (doc, first) = a_chain();
    let mut record = nominal(&doc);
    let clean = dual(&doc, &record);
    for (id, r) in &clean.nodes {
        assert!(
            !matches!(
                r,
                NodeResult::Failed(NodeError {
                    kind: NodeErrorKind::RecordRefused,
                    ..
                })
            ),
            "node {id:?}: a record that built it binds nothing"
        );
    }
    // The record refuses its first node, as a tier-3 refusal at the
    // base scalar would.
    let NodeResult::Ok(_) = record.nodes[&first] else {
        unreachable!("the chain builds")
    };
    record.nodes.insert(
        first,
        NodeResult::Failed(NodeError {
            node: first,
            kind: NodeErrorKind::MissingInput { input: first },
            escalations: Default::default(),
        }),
    );
    let bound = dual(&doc, &record);
    assert!(
        matches!(
            bound.nodes[&first],
            NodeResult::Failed(NodeError { kind: NodeErrorKind::RecordRefused, node, .. })
                if node == first
        ),
        "the record's refusal is the dual's: {:?}",
        bound.nodes[&first]
    );
    // Every other node either built, as at the record, or is poisoned
    // through the refused one: nothing else refuses.
    let mut poisoned = 0;
    for (id, r) in bound.nodes.iter().filter(|(id, _)| **id != first) {
        match r {
            NodeResult::Ok(_) => {}
            NodeResult::Poisoned { through } if *through == first => poisoned += 1,
            other => panic!("node {id:?}: {other:?}"),
        }
    }
    assert!(
        poisoned >= 1,
        "a node downstream is poisoned through the refused one"
    );
}

/// **A record of another document binds nothing of this one**: every
/// node refuses, before any runs (DI3).
#[test]
fn a_record_of_another_document_refuses_every_node() {
    let (a, _) = a_chain();
    let (b, _) = a_chain_of("a-dual-is-bound-foreign");
    let (a, b) = (&a, &b);
    let foreign = nominal(b);
    let bound = dual(a, &foreign);
    assert!(!bound.nodes.is_empty(), "the document has nodes");
    for (id, r) in &bound.nodes {
        assert!(
            matches!(
                r,
                NodeResult::Failed(NodeError {
                    kind: NodeErrorKind::RecordMispaired(RecordMispairing::Document(m)),
                    ..
                }) if m.expected == a.id() && m.found == b.id()
            ),
            "node {id:?}: {r:?}"
        );
    }
}

/// Every node of `bound` refused as `RecordMispaired(why)`.
fn every_node_mispaired(bound: &Evaluation<Dual64>, why: &RecordMispairing) {
    assert!(!bound.nodes.is_empty(), "the document has nodes");
    for (id, r) in &bound.nodes {
        assert!(
            matches!(
                r,
                NodeResult::Failed(NodeError { kind: NodeErrorKind::RecordMispaired(m), .. })
                    if m == why
            ),
            "node {id:?}: {r:?}, expected {why:?}"
        );
    }
}

/// **A record of this document that is not a build of this run's input
/// binds nothing**: one that carries no pin, so whose version is not
/// known, and one built over other options, each refuses every node,
/// naming what it read otherwise. The document id alone does not make a
/// record this run's.
#[test]
fn a_record_of_this_document_but_another_build_refuses_every_node() {
    let (doc, _) = a_chain();
    let unpinned = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    every_node_mispaired(&dual(&doc, &unpinned), &RecordMispairing::Unpinned);
    let symbolic = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions {
            symbols: true,
            ..EvalOptions::default().of_record()
        },
        Tol::witness(),
    );
    if symbolic.refused_whole() {
        every_node_mispaired(&dual(&doc, &symbolic), &RecordMispairing::NotABuild);
    } else {
        every_node_mispaired(&dual(&doc, &symbolic), &RecordMispairing::Options);
    }
}

/// **A record that did not complete built nothing past its prefix**: a
/// node it never reached is refused at the dual, as one it refused is.
#[test]
fn a_node_a_canceled_record_never_reached_is_refused_at_the_dual() {
    let (doc, first) = a_chain();
    let mut record = nominal(&doc);
    let last = *record.order.last().expect("the chain has nodes");
    assert_ne!(last, first, "the chain has two nodes");
    record.nodes.remove(&last);
    record.outcome = EvalOutcome::Canceled;
    let bound = dual(&doc, &record);
    assert!(
        matches!(
            bound.nodes[&last],
            NodeResult::Failed(NodeError { kind: NodeErrorKind::RecordRefused, node, .. })
                if node == last
        ),
        "the node the record never reached: {:?}",
        bound.nodes[&last]
    );
    assert!(
        matches!(bound.nodes[&first], NodeResult::Ok(_)),
        "the node the record built is the dual's own: {:?}",
        bound.nodes[&first]
    );
}

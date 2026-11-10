//! **A dual evaluates only bound to its build of record** (DUAL-DESIGN
//! DL3). The tier-3 premise a dual operand carries no evidence of is
//! asserted here, at the evaluation service: `evaluate_recorded` refuses
//! every node its record refused, before the node runs, so no topo
//! reading a dual reaches leans on a gate the record failed
//! (`topo::Replay`'s "A tier-3 premise at a dual").

#![allow(clippy::panic, clippy::expect_used)]

use crate::corpus;
use editor_core::{
    CancelToken, EvalOptions, Evaluation, NodeError, NodeErrorKind, NodeResult, evaluate,
    evaluate_recorded,
};
use geom_core::{Dual64, Tol};

fn nominal(doc: &editor_core::ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
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

/// A corpus document whose build has a node with another node
/// downstream of it: the record's refusal there must reach both.
fn a_chain() -> (editor_core::ProfileDoc, editor_core::RecipeNodeId) {
    for d in corpus::documents() {
        let record = nominal(&d.doc);
        if record.order.len() >= 2
            && record
                .nodes
                .values()
                .all(|r| matches!(r, NodeResult::Ok(_)))
        {
            return (d.doc, record.order[0]);
        }
    }
    unreachable!("the corpus holds a document of two nodes that builds")
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
    let docs = corpus::documents();
    let a = &docs[0].doc;
    let b = &docs
        .iter()
        .find(|d| d.doc.id() != a.id())
        .expect("the corpus holds two documents")
        .doc;
    let foreign = nominal(b);
    let bound = dual(a, &foreign);
    assert!(!bound.nodes.is_empty(), "the document has nodes");
    for (id, r) in &bound.nodes {
        assert!(
            matches!(
                r,
                NodeResult::Failed(NodeError { kind: NodeErrorKind::RecordMispaired(m), .. })
                    if m.expected == a.id() && m.found == b.id()
            ),
            "node {id:?}: {r:?}"
        );
    }
}

//! **The delete door's companion query**: which nodes a delete must
//! take with it, and in what order.
//!
//! `DocEdit::DeleteNode` refuses to dangle a live reference, which in
//! a chain-shaped recipe means it can delete exactly one node. The
//! sequence [`cascade_delete_order`] hands back is the whole dependent
//! cone in an order the door accepts, so a caller can delete a feature
//! from the middle of a model without the primitive ever softening.
//!
//! What it does NOT do is reconnect the target's consumers to its
//! input — splice, open as issue #1324. These rows pin that: a node
//! whose only tie to the target is that it FED the target survives.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Dimension, Doc, DocEdit, EditError, Expr, Node, RecipeNodeId, apply, cascade_delete_order,
};
use geom_core::Tol;

/// The opaque profile payload: this suite never looks inside `P`.
#[derive(Debug, Clone, PartialEq)]
struct FakeProfile(&'static str);
impl editor_core::ProfilePayload for FakeProfile {}

type TDoc = Doc<FakeProfile>;
type TEdit = DocEdit<FakeProfile>;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}

fn insert(doc: &TDoc, node: Node<FakeProfile>) -> (TDoc, RecipeNodeId) {
    let applied = apply(doc, &TEdit::InsertNode { node }, Tol::witness()).unwrap();
    let id = applied.record.minted.unwrap();
    (applied.doc, id)
}

fn extrude(doc: &TDoc, profile: RecipeNodeId) -> (TDoc, RecipeNodeId) {
    insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(0.01),
        },
    )
}

/// profile → extrude → two independent fillets over the same extrude:
/// a fork, so the closure has to gather more than a path.
fn fork() -> (TDoc, [RecipeNodeId; 4]) {
    let doc = TDoc::empty_derived("cascade_delete", Tol::witness());
    let (doc, profile) = insert(&doc, Node::Profile(FakeProfile("square")));
    let (doc, body) = extrude(&doc, profile);
    let fillet = |target| Node::Fillet {
        target,
        radius: len(0.001),
        selection: Vec::new(),
    };
    let (doc, left) = insert(&doc, fillet(body));
    let (doc, right) = insert(&doc, fillet(body));
    (doc, [profile, body, left, right])
}

#[test]
fn the_order_is_consumers_first_and_the_target_last() {
    let (doc, [profile, body, left, right]) = fork();
    assert_eq!(
        cascade_delete_order(&doc, profile),
        vec![right, left, body, profile],
        "reverse document order, which is a topological order of the cone"
    );
}

/// The point of the order: every edit in it is accepted by the
/// unchanged primitive, one at a time.
#[test]
fn every_step_of_the_order_is_accepted_by_the_delete_door() {
    let (mut doc, [profile, ..]) = fork();
    let order = cascade_delete_order(&doc, profile);
    assert_eq!(order.len(), 4);
    for id in order {
        doc = apply(&doc, &TEdit::DeleteNode { id }, Tol::witness())
            .expect("the cone's order never dangles a reference")
            .doc;
    }
    assert!(doc.order().is_empty(), "the whole cone is gone");
}

/// Deleting the fork's tip takes the tip and nothing else — the
/// sibling branch and the shared body are not downstream of it.
#[test]
fn a_leaf_is_its_own_whole_cascade() {
    let (doc, [_, _, left, _]) = fork();
    assert_eq!(cascade_delete_order(&doc, left), vec![left]);
}

/// A node's INPUTS are not in its cascade. That is the gap splice
/// (#1324) would close: deleting the body leaves the profile behind
/// rather than rejoining it to the body's consumers.
#[test]
fn inputs_of_the_target_survive_it() {
    let (doc, [profile, body, left, right]) = fork();
    let order = cascade_delete_order(&doc, body);
    assert_eq!(order, vec![right, left, body]);
    assert!(!order.contains(&profile));
}

/// An id the document does not hold has no cascade; the typed refusal
/// is the door's to give, not this query's.
#[test]
fn an_absent_node_has_an_empty_cascade() {
    let (doc, _) = fork();
    let absent = RecipeNodeId(9_999);
    assert!(cascade_delete_order(&doc, absent).is_empty());
    assert_eq!(
        apply(&doc, &TEdit::DeleteNode { id: absent }, Tol::witness()).unwrap_err(),
        EditError::UnknownNode { id: absent }
    );
}

/// The refusal a user can still meet says which way the reference
/// runs and what to do about it — bare ids and the word "dangle" told
/// them neither.
#[test]
fn the_dangle_refusal_states_the_remedy() {
    let (doc, [profile, body, ..]) = fork();
    let refusal = apply(&doc, &TEdit::DeleteNode { id: profile }, Tol::witness()).unwrap_err();
    assert_eq!(
        refusal,
        EditError::DeleteWouldDangle {
            id: profile,
            referenced_by: body,
        }
    );
    let sentence = refusal.to_string();
    assert!(
        sentence.contains(&format!("node {} is still an input to node {}", profile.0, body.0)),
        "the direction of the reference is stated: {sentence}"
    );
    assert!(
        sentence.contains(&format!("delete node {} first", body.0)),
        "and the immediate remedy: {sentence}"
    );
    assert!(
        sentence.contains("everything downstream of it"),
        "and the cascade: {sentence}"
    );
}

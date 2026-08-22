//! ExprPath stability, RecipeNodeId permanence, and graph-validation
//! refusals (spec D5 + D8) — the contracts GeomSource (PR 5) and the
//! naming layer (N1) will lean on.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Datum, Dimension, Doc, DocEdit, EditError, Expr, ExprPath, Node, RecipeNodeId, SlotId,
};
use geom_core::Tol;

/// The opaque profile payload for tests: this crate never looks
/// inside `P` (spec D1/D3 — PR 2 instantiates the real profile type).
#[derive(Debug, Clone, PartialEq)]
struct FakeProfile(&'static str);
// The v4 payload trait: fake payloads take the slot-free, check-free
// defaults (LIB-SWITCH §4c — exactly the retired opaque behavior).
impl editor_core::ProfilePayload for FakeProfile {}

type TDoc = Doc<FakeProfile>;
type TEdit = DocEdit<FakeProfile>;

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).unwrap()
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).unwrap()
}

/// profile + extrude(distance = a + b), returning (doc, profile id,
/// extrude id).
fn profile_and_extrude() -> (TDoc, RecipeNodeId, RecipeNodeId) {
    let doc = TDoc::empty_derived("m4_pr1_paths", Tol::witness());
    let a = doc
        .apply(
            &TEdit::InsertNode {
                node: Node::Profile(FakeProfile("square")),
            },
            Tol::witness(),
        )
        .unwrap();
    let profile = a.record.minted.unwrap();
    let distance = Expr::add(len(0.010), len(0.005)).unwrap();
    let b = a
        .doc
        .apply(
            &TEdit::InsertNode {
                node: Node::Extrude { profile, distance },
            },
            Tol::witness(),
        )
        .unwrap();
    (b.doc, profile, b.record.minted.unwrap())
}

#[test]
fn expr_path_survives_edits_to_other_expressions() {
    let (doc, _profile, extrude) = profile_and_extrude();
    // Address the second addend of the extrude distance.
    let path = ExprPath {
        node: extrude,
        slot: SlotId::Distance,
        path: vec![1],
    };
    let before = doc.expr_at(&path).unwrap().clone();

    // Insert an unrelated datum point and edit ITS expression: the
    // ExprPath's referent is untouched (spec D5).
    let c = doc
        .apply(
            &TEdit::InsertNode {
                node: Node::Datum(Datum::Point {
                    position: [len(0.0), len(0.0), len(0.0)],
                }),
            },
            Tol::witness(),
        )
        .unwrap();
    let datum = c.record.minted.unwrap();
    let d = c
        .doc
        .apply(
            &TEdit::SetParam {
                node: datum,
                slot: SlotId::Origin(editor_core::Axis3::X),
                expr: len(0.042),
            },
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(d.doc.expr_at(&path).unwrap(), &before);
}

#[test]
fn expr_path_survives_edits_to_unrelated_subtrees() {
    let (doc, _profile, extrude) = profile_and_extrude();
    let second = ExprPath {
        node: extrude,
        slot: SlotId::Distance,
        path: vec![1],
    };
    let before = doc.expr_at(&second).unwrap().clone();

    // Replace the FIRST addend (sibling subtree) via SetExpression:
    // the second addend's referent is intact (spec D5).
    let first = ExprPath {
        node: extrude,
        slot: SlotId::Distance,
        path: vec![0],
    };
    let e = doc
        .apply(
            &TEdit::SetExpression {
                path: first.clone(),
                expr: len(0.020),
            },
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(e.doc.expr_at(&second).unwrap(), &before);
    assert_eq!(e.doc.expr_at(&first).unwrap(), &len(0.020));
    // The whole-slot expression still type-checks as Length.
    assert_eq!(
        e.doc
            .expr_at(&ExprPath {
                node: extrude,
                slot: SlotId::Distance,
                path: vec![]
            })
            .unwrap()
            .dim(),
        Dimension::Length
    );
}

#[test]
fn recipe_node_ids_are_never_reused() {
    // Delete then insert: the freed id must NOT come back (spec D3 —
    // N1's substrate contract).
    let doc = TDoc::empty_derived("m4_pr1_paths", Tol::witness());
    let a = doc
        .apply(
            &TEdit::InsertNode {
                node: Node::Profile(FakeProfile("p0")),
            },
            Tol::witness(),
        )
        .unwrap();
    let first = a.record.minted.unwrap();
    let b = a
        .doc
        .apply(&TEdit::DeleteNode { id: first }, Tol::witness())
        .unwrap();
    let c = b
        .doc
        .apply(
            &TEdit::InsertNode {
                node: Node::Profile(FakeProfile("p1")),
            },
            Tol::witness(),
        )
        .unwrap();
    let second = c.record.minted.unwrap();
    assert_ne!(first, second);
    assert!(second.0 > first.0, "ids are monotone");
    assert!(c.doc.node(first).is_none());
}

#[test]
fn dangling_ref_rejected() {
    let doc = TDoc::empty_derived("m4_pr1_paths", Tol::witness());
    let ghost = RecipeNodeId(99);
    let err = doc
        .apply(
            &TEdit::InsertNode {
                node: Node::Extrude {
                    profile: ghost,
                    distance: len(0.01),
                },
            },
            Tol::witness(),
        )
        .unwrap_err();
    assert_eq!(err, EditError::UnresolvedInput { input: ghost });
}

#[test]
fn self_reference_cannot_forge_the_next_id() {
    // Guessing the about-to-mint id is still an unresolved ref: refs
    // must resolve among EXISTING nodes, so insertion cannot cycle.
    let doc = TDoc::empty_derived("m4_pr1_paths", Tol::witness());
    let guessed = RecipeNodeId(0); // empty doc will mint 0 next
    let err = doc
        .apply(
            &TEdit::InsertNode {
                node: Node::Extrude {
                    profile: guessed,
                    distance: len(0.01),
                },
            },
            Tol::witness(),
        )
        .unwrap_err();
    assert_eq!(err, EditError::UnresolvedInput { input: guessed });
}

#[test]
fn delete_of_referenced_node_rejected() {
    let (doc, profile, extrude) = profile_and_extrude();
    let err = doc
        .apply(&TEdit::DeleteNode { id: profile }, Tol::witness())
        .unwrap_err();
    assert_eq!(
        err,
        EditError::DeleteWouldDangle {
            id: profile,
            referenced_by: extrude
        }
    );
}

#[test]
fn structural_and_continuous_edit_arms_are_disjoint() {
    let (doc, _profile, extrude) = profile_and_extrude();
    // SetStructuralParam on a continuous slot: typed refusal.
    let err = doc
        .apply(
            &TEdit::SetStructuralParam {
                node: extrude,
                slot: SlotId::Distance,
                expr: len(0.01),
            },
            Tol::witness(),
        )
        .unwrap_err();
    assert_eq!(
        err,
        EditError::NotStructuralSlot {
            slot: SlotId::Distance
        }
    );
    // Slot dimension is enforced on SetParam.
    let err = doc
        .apply(
            &TEdit::SetParam {
                node: extrude,
                slot: SlotId::Distance,
                expr: scl(1.0),
            },
            Tol::witness(),
        )
        .unwrap_err();
    assert_eq!(
        err,
        EditError::SlotDimensionMismatch {
            slot: SlotId::Distance,
            expected: Dimension::Length,
            found: Dimension::Scalar,
        }
    );
}

#[test]
fn set_expression_path_off_tree_rejected() {
    let (doc, _profile, extrude) = profile_and_extrude();
    let bad = ExprPath {
        node: extrude,
        slot: SlotId::Distance,
        path: vec![7],
    };
    let err = doc
        .apply(
            &TEdit::SetExpression {
                path: bad.clone(),
                expr: len(0.01),
            },
            Tol::witness(),
        )
        .unwrap_err();
    assert_eq!(err, EditError::PathOffTree { path: bad });
}

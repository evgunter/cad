//! **A PAYLOAD expression names a declared document parameter, and
//! reads it at the dimension the declaration carries, at EVERY door**
//! (spec D6, `Doc::param_ref_fault`).
//!
//! The expressions no slot addresses — a `Node::Measure`'s
//! `MeasureExpr` value leaves and a `Node::Assertion`'s bound
//! (`node::payload_exprs`) — ask the same one predicate the slot
//! expressions ask. The two doors only name its answer: the edit door
//! as `EditError::UnknownPayloadParam` and
//! `EditError::PayloadParamDimensionMismatch`, the load door as
//! `SnapshotError::PayloadUnknownDocParam` and
//! `SnapshotError::PayloadDocParamDimension`. So a file cannot carry a
//! payload expression an edit door would have refused.
//!
//! **One row per FACT, naming both doors' refusals for it**: a
//! predicate dropped at either door reds the fact's row and the panic
//! says which door let the document through.
//!
//! What is NOT here: the payload expressions' DIMENSIONS, which are
//! fixed at construction — a `MeasureExpr` runs the F1 checker at every
//! constructor, and an assertion's bound is checked against its
//! measure's dimension (`Node::assertion_bound_fault`, the load door's
//! `SnapshotError::AssertionBound`). This suite is the param TABLE's
//! half of the same address, and nothing here re-checks those.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    Dimension, DocEdit, DocParam, EditError, Expr, MeasureExpr, Node, ParamName, PersistError,
    ProfileDoc, RecipeNodeId, SlotId, SnapshotError, apply, load, save,
};
use fixture::{insert, len, on_frame, square};
use geom_core::Tol;

/// Wire surgery by path, as `load_door_slot_dimension::doctored`.
fn doctored(text: &str, edit: impl FnOnce(&mut serde_json::Value)) -> String {
    let split = text.find('{').expect("the JSON body follows the id header");
    let (header, body) = text.split_at(split);
    let mut wire: serde_json::Value = serde_json::from_str(body).expect("the body parses");
    edit(&mut wire);
    let out = format!("{header}{wire}");
    assert_ne!(out, text, "the corruption really landed");
    out
}

/// A frame, a profile, an extrude and a LENGTH document parameter
/// `depth` — the ground every row below builds its payload node on,
/// keeping the EXTRUDE's id for the row that needs a slot to break.
fn with_depth() -> (ProfileDoc, ParamName, RecipeNodeId) {
    let name = ParamName::new("depth");
    let (doc, profile) = on_frame(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("payload-param-ref"),
            Tol::witness(),
        ),
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::continuous(Dimension::Length, 1.0),
        },
        Tol::witness(),
    )
    .expect("a well-formed length parameter declares")
    .doc;
    (doc, name, extrude)
}

/// [`with_depth`] plus a measure whose expression reads `depth`.
fn measuring_depth() -> (ProfileDoc, ParamName, RecipeNodeId) {
    let (doc, name, _) = with_depth();
    let (doc, measure) = insert(
        doc,
        Node::Measure {
            expr: MeasureExpr::value(Expr::param(name.clone(), Dimension::Length)),
            refs: Vec::new(),
        },
    );
    (doc, name, measure)
}

/// The declaration removed from the wire, out from under whatever reads
/// it.
fn undeclare(text: &str, name: &ParamName) -> String {
    doctored(text, |wire| {
        let params = wire["snapshot"]["params"]
            .as_object_mut()
            .expect("the params are a map");
        assert!(
            params.remove(&name.0).is_some(),
            "the surgery is aimed at the declaration the payload reads"
        );
    })
}

/// The declaration RETYPED on the wire, its display unit moved with it
/// so the document is broken in exactly one way: the pairing between a
/// declaration and the dimension an expression reads it at.
fn retype_to_angle(text: &str, name: &ParamName) -> String {
    doctored(text, |wire| {
        let decl = &mut wire["snapshot"]["params"][&name.0]["Continuous"];
        assert_eq!(
            decl["dim"],
            serde_json::json!("Length"),
            "the surgery is aimed at the declared dimension"
        );
        decl["dim"] = serde_json::json!("Angle");
        decl["display_unit"] = serde_json::json!("rad");
    })
}

/// **A measured expression reading an undeclared parameter — both
/// doors.** The edit door refuses the node as it is written; the load
/// door refuses the file whose declaration is gone from under it.
///
/// This row is the flip of the measurement that disclosed the gap
/// (`work/edit/load-door-does-not-check-payload-expression-param-refs`):
/// the same saved document loaded clean while the edit door refused
/// the same node, and this is that sentence with the load door's
/// answer in it.
#[test]
fn a_measure_expression_reading_an_undeclared_parameter_refuses_to_load() {
    let (doc, name, measure) = measuring_depth();

    // The edit door, over the node as written.
    let missing = ParamName::new("nowhere");
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Measure {
                expr: MeasureExpr::value(Expr::param(missing.clone(), Dimension::Length)),
                refs: Vec::new(),
            },
        },
        Tol::witness(),
    ) {
        Err(EditError::UnknownPayloadParam { name: n, .. }) => assert_eq!(n, missing),
        other => panic!("the edit door must refuse an undeclared payload param, got {other:?}"),
    }

    // The load door, over the file whose declaration was removed.
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    match load(&undeclare(&text, &name), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PayloadUnknownDocParam { node, name: n })) => {
            assert_eq!((node, n), (measure, name));
        }
        other => panic!("the load door must refuse an undeclared payload param, got {other:?}"),
    }
}

/// **A measured expression reading a parameter at another dimension
/// than it is declared with — both doors.** The edit door re-asks the
/// rule of every node when a declaration lands, so the redeclaration
/// is what it refuses; the load door reads the same broken pairing off
/// a file whose declaration was retyped after the fact.
#[test]
fn a_measure_expression_reading_a_parameter_at_the_wrong_dimension_refuses_to_load() {
    let (doc, name, measure) = measuring_depth();

    match apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name.clone(),
            value: DocParam::continuous(Dimension::Angle, 1.0),
        },
        Tol::witness(),
    ) {
        Err(EditError::PayloadParamDimensionMismatch {
            name: n,
            node,
            declared,
            referenced,
        }) => {
            assert_eq!((n, node), (name.clone(), measure));
            assert_eq!(
                (declared, referenced),
                (Dimension::Angle, Dimension::Length)
            );
        }
        other => panic!("the edit door must refuse the redeclaration, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    match load(&retype_to_angle(&text, &name), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PayloadDocParamDimension {
            node,
            name: n,
            declared,
            referenced,
        })) => {
            assert_eq!((node, n), (measure, name));
            assert_eq!(
                (declared, referenced),
                (Dimension::Angle, Dimension::Length)
            );
        }
        other => panic!("the load door must refuse the broken pairing, got {other:?}"),
    }
}

/// **An assertion's BOUND is a payload expression too — both doors.**
/// The second of the two expressions no slot addresses, so the walk
/// that misses it misses half the vocabulary.
#[test]
fn an_assertion_bound_reading_an_undeclared_parameter_refuses_to_load() {
    let (doc, name, _) = with_depth();
    let (doc, measure) = insert(
        doc,
        Node::Measure {
            expr: MeasureExpr::value(len(1.0)),
            refs: Vec::new(),
        },
    );
    let bound = |n: &ParamName| Node::Assertion {
        measure,
        bound: Expr::param(n.clone(), Dimension::Length),
        dir: editor_core::AssertionDir::AtLeast,
    };
    let (doc, assertion) = insert(doc, bound(&name));

    let missing = ParamName::new("nowhere");
    match apply(
        &doc,
        &DocEdit::InsertNode {
            node: bound(&missing),
        },
        Tol::witness(),
    ) {
        Err(EditError::UnknownPayloadParam { name: n, .. }) => assert_eq!(n, missing),
        other => panic!("the edit door must refuse an undeclared bound param, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    match load(&undeclare(&text, &name), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::PayloadUnknownDocParam { node, name: n })) => {
            assert_eq!((node, n), (assertion, name));
        }
        other => panic!("the load door must refuse an undeclared bound param, got {other:?}"),
    }
}

/// **A well-formed payload reference round-trips.** The walk refuses a
/// broken pairing and nothing else: a measure reading a declared
/// parameter at its declared dimension saves, loads, and comes back
/// bit for bit.
#[test]
fn a_payload_reference_the_table_answers_round_trips() {
    let (doc, _, _) = measuring_depth();
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let loaded = load(&text, Tol::witness()).expect("the fixture loads").doc;
    assert!(
        loaded.bit_eq(&doc),
        "a payload expression the param table answers round-trips"
    );
}

/// **The walk ORDER, pinned**: a document broken in a SLOT expression
/// and in a PAYLOAD expression at once reads the SLOT refusal, because
/// the slot param-ref walk runs first (`persist::check::Walk::ORDER`).
///
/// Moving the payload walk ahead of the slot walk changes the
/// diagnosis of every file broken both ways, and this row is what says
/// so out loud.
#[test]
fn a_document_broken_in_a_slot_and_in_a_payload_reads_the_slot_refusal() {
    let (doc, name, extrude) = with_depth();
    let doc = apply(
        &doc,
        &DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: Expr::param(name.clone(), Dimension::Length),
        },
        Tol::witness(),
    )
    .expect("a length parameter drives a length slot")
    .doc;
    let (doc, _) = insert(
        doc,
        Node::Measure {
            expr: MeasureExpr::value(Expr::param(name.clone(), Dimension::Length)),
            refs: Vec::new(),
        },
    );

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    match load(&undeclare(&text, &name), Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotUnknownDocParam {
            node,
            slot,
            name: n,
        })) => assert_eq!((node, slot, n), (extrude, SlotId::Distance, name)),
        other => panic!(
            "a file broken in a slot AND in a payload must read the slot walk's refusal — the \
             walk order `validate_document` documents. Got {other:?}"
        ),
    }
}

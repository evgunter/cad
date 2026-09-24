//! **A slot's expression carries the dimension the slot address fixes,
//! at EVERY door and for every node kind** (spec D6,
//! [`editor_core::SlotId::dimension`]).
//!
//! One predicate answers it — `Node::slot_dimension_fault`, over
//! `Node::slots()` — and the two doors only name that answer: the edit
//! door as `EditError::SlotDimensionMismatch`, the load door as
//! `SnapshotError::SlotDimension`. So a file cannot carry a slot
//! expression an edit door would have refused, whatever kind of node
//! holds it.
//!
//! **One row per FACT, naming both doors' refusals for it.** A
//! predicate dropped at either door reds the fact's row and the panic
//! says which door let the document through; a door that mis-maps the
//! shared answer onto the wrong arm reds it too, because each refusal
//! is read by arm and by payload — node, slot and both dimensions.
//!
//! The PARAM TABLE's half of the same address is here too
//! (`Doc::param_ref_fault`): a slot expression names a declared
//! parameter and reads it at the dimension it was declared with, at
//! both doors, because a redeclaration that moves a dimension breaks
//! every expression referencing it and a file can be written with the
//! pairing already broken.
//!
//! The kinds below are the two the narrowed walk could not see: an
//! extrude and a frame datum. A PROFILE program's own step argument is
//! the same question at the same door, pinned by
//! `m4_pr6_refusal::program_structure_doors_refuse_typed_at_load`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use crate::wire::doctored;
use editor_core::{
    Axis3, Dimension, DocEdit, EditError, Node, PersistError, ProfileDoc, RecipeNodeId, SlotId,
    SnapshotError, apply, load, save,
};
use fixture::{ang, len, on_frame_keeping, square};
use geom_core::Tol;

/// A one-extrude document on an explicit frame: the frame's origin
/// components are lengths, the extrude's distance is a length, and
/// both are slots whose addresses fix that.
fn doc() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, frame, profile) = on_frame_keeping(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("slot-dim-both-doors"),
            Tol::witness(),
        ),
        [0.25, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(0.0, 0.0, 0.5)],
    );
    let (doc, extrude) = fixture::insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    (doc, frame, extrude)
}

/// Retypes one literal from `Length`/`m` to `Angle`/`rad`. BOTH halves
/// move, so the literal stays well-formed through
/// `Expr::literal_with_unit` and the display-unit walk has nothing to
/// say — the only rule left to refuse it is the slot's own.
fn retype_to_angle(literal: &mut serde_json::Value) {
    let lit = &mut literal["Literal"];
    assert_eq!(
        lit["dim"],
        serde_json::json!("Length"),
        "the surgery is aimed at a length literal"
    );
    lit["dim"] = serde_json::json!("Angle");
    lit["unit"] = serde_json::json!("rad");
}

/// **An extrude's distance — both doors.** The measured asymmetry this
/// row was opened for: the load door's slot walk used to cover profile
/// nodes only, so this file loaded clean while the edit door refused
/// the same node.
#[test]
fn a_retyped_extrude_distance_is_refused_at_both_doors() {
    let (doc, _, extrude) = doc();
    match apply(
        &doc,
        &DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: ang(1.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        }) => {
            assert_eq!(slot, SlotId::Distance);
            assert_eq!((expected, found), (Dimension::Length, Dimension::Angle));
        }
        other => panic!("the edit door must refuse an angle distance, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        retype_to_angle(
            &mut wire["snapshot"]["nodes"][extrude.0.to_string()]["Extrude"]["distance"],
        )
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotDimension {
            node,
            slot,
            expected,
            found,
        })) => {
            assert_eq!((node, slot), (extrude, SlotId::Distance));
            assert_eq!((expected, found), (Dimension::Length, Dimension::Angle));
        }
        other => panic!("the load door must refuse an angle distance, got {other:?}"),
    }
}

/// **A frame datum's origin component — both doors.** A second node
/// kind, and a slot whose address is a COMPONENT rather than a whole
/// quantity: the walk is over `Node::slots()`, so nothing about it is
/// per-kind.
#[test]
fn a_retyped_frame_origin_is_refused_at_both_doors() {
    let (doc, frame, _) = doc();
    let slot = SlotId::Origin(Axis3::X);
    match apply(
        &doc,
        &DocEdit::SetParam {
            node: frame,
            slot,
            expr: ang(0.25),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::SlotDimensionMismatch {
            slot: refused,
            expected,
            found,
        }) => {
            assert_eq!(refused, slot);
            assert_eq!((expected, found), (Dimension::Length, Dimension::Angle));
        }
        other => panic!("the edit door must refuse an angle origin, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        retype_to_angle(
            &mut wire["snapshot"]["nodes"][frame.0.to_string()]["Datum"]["Frame"]["origin"][0],
        )
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotDimension {
            node,
            slot: refused,
            expected,
            found,
        })) => {
            assert_eq!((node, refused), (frame, slot));
            assert_eq!((expected, found), (Dimension::Length, Dimension::Angle));
        }
        other => panic!("the load door must refuse an angle origin, got {other:?}"),
    }
}

/// A one-extrude document whose distance is the PARAMETER `depth`,
/// declared as a length.
fn parameterized() -> (ProfileDoc, RecipeNodeId, editor_core::ParamName) {
    let name = editor_core::ParamName::literal("depth");
    let (doc, _, extrude) = doc();
    let doc = apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name.clone(),
            value: editor_core::DocParam::continuous(Dimension::Length, 1.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a well-formed length parameter declares")
    .doc;
    let doc = apply(
        &doc,
        &DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: editor_core::Expr::param(name.clone(), Dimension::Length),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a length parameter drives a length slot")
    .doc;
    (doc, extrude, name)
}

/// **A slot expression naming an undeclared parameter — both doors.**
/// The edit door refuses the expression as it is written; the load
/// door refuses the file whose declaration is gone from under it.
#[test]
fn a_slot_reading_an_undeclared_parameter_is_refused_at_both_doors() {
    let (doc, extrude, name) = parameterized();
    let missing = editor_core::ParamName::literal("nowhere");
    match apply(
        &doc,
        &DocEdit::SetParam {
            node: extrude,
            slot: SlotId::Distance,
            expr: editor_core::Expr::param(missing.clone(), Dimension::Length),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::SlotUnknownDocParam {
            name: n,
            node,
            slot,
        }) => {
            assert_eq!((n, node, slot), (missing, extrude, SlotId::Distance));
        }
        other => panic!("the edit door must refuse an undeclared parameter, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        let params = wire["snapshot"]["params"]
            .as_object_mut()
            .expect("the params are a map");
        assert!(
            params.remove(name.as_str()).is_some(),
            "the surgery is aimed at the declaration the slot reads"
        );
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotUnknownDocParam {
            node,
            slot,
            name: n,
        })) => {
            assert_eq!((node, slot, n), (extrude, SlotId::Distance, name));
        }
        other => panic!("the load door must refuse an undeclared parameter, got {other:?}"),
    }
}

/// **A slot expression reading a parameter at another dimension than
/// it is declared with — both doors.** The edit door re-asks the rule
/// of every slot when a declaration lands, so the redeclaration is
/// what it refuses; the load door reads the same broken pairing off a
/// file whose declaration was retyped after the fact.
#[test]
fn a_slot_reading_a_parameter_at_the_wrong_dimension_is_refused_at_both_doors() {
    let (doc, extrude, name) = parameterized();
    match apply(
        &doc,
        &DocEdit::SetDocParam {
            name: name.clone(),
            value: editor_core::DocParam::continuous(Dimension::Angle, 1.0),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::SlotDocParamDimension {
            name: n,
            node,
            slot,
            declared,
            referenced,
        }) => {
            assert_eq!((n, node, slot), (name.clone(), extrude, SlotId::Distance));
            assert_eq!(
                (declared, referenced),
                (Dimension::Angle, Dimension::Length)
            );
        }
        other => panic!("the edit door must refuse the redeclaration, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    let corrupt = doctored(&text, |wire| {
        let decl = &mut wire["snapshot"]["params"][name.as_str()]["Continuous"];
        assert_eq!(
            decl["dim"],
            serde_json::json!("Length"),
            "the surgery is aimed at the declared dimension"
        );
        decl["dim"] = serde_json::json!("Angle");
        decl["display_unit"] = serde_json::json!("rad");
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotDocParamDimension {
            node,
            slot,
            name: n,
            declared,
            referenced,
        })) => {
            assert_eq!((node, slot, n), (extrude, SlotId::Distance, name));
            assert_eq!(
                (declared, referenced),
                (Dimension::Angle, Dimension::Length)
            );
        }
        other => panic!("the load door must refuse the broken pairing, got {other:?}"),
    }
}

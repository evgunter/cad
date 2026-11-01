//! **The slot walks' third node kind and the walk ORDER** — written by
//! the review lane `onepred3-rv` to falsify the unit's claims, and
//! adopted by it: each row measures something no other row in the
//! suite does.
//!
//! - A Count-dimensioned STRUCTURAL slot on a third node kind, refused
//!   at both doors.
//! - The walk ORDER as a contract: a file broken twice reads the
//!   EARLIER walk's refusal, which is what makes a re-ordering a
//!   change to every such file's diagnosis.
//!
//! The payload-expression half of the param-table rule was measured
//! here as a GAP and is a contract now: its rows, reading both doors'
//! answers over one fixture, are `load_door_payload_param_ref`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use crate::wire::doctored;
use editor_core::{
    Dimension, DocEdit, EditError, Node, OperandSlot, PersistError, ProfileDoc, RecipeNodeId,
    SlotId, SnapshotError, apply, load, save,
};
use fixture::{desc, insert, len, square};
use geom_core::Tol;

/// The index of a placement's read of one member — a Count-typed
/// STRUCTURAL slot.
const INDEX: SlotId = SlotId::Index {
    seat: OperandSlot::Source,
    k: 0,
};

/// An extrude, a row of it, and a copy of the row's member 1 — the
/// copy's index ([`INDEX`]) is a Count-typed STRUCTURAL slot, a node
/// kind beside the extrude the unit's own suite covers. Answers the
/// document, the extrude and the copy.
fn patterned() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, profile) = insert(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("rv-onepred3"),
            Tol::witness(),
        ),
        Node::Profile(desc(vec![square(0.0, 0.0, 0.5)])),
    );
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    );
    let (doc, _, row) = crate::fixture::linear(doc, extrude, [1.0, 0.0, 0.0], 3.0, 3);
    let (doc, read) = crate::fixture::member(doc, row, 1);
    (doc, extrude, read)
}

/// PROBE 1 — a THIRD node kind with a typed slot, and a Count one:
/// a member read's index. Claim 1 of the review brief.
#[test]
fn rv_a_retyped_member_index_is_refused_at_both_doors() {
    let (doc, _, pattern) = patterned();
    match apply(
        &doc,
        &DocEdit::SetStructuralParam {
            node: pattern,
            slot: INDEX,
            expr: len(3.0),
            fresh: Vec::new(),
        },
        Tol::witness(),
    ) {
        Err(EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        }) => assert_eq!(
            (slot, expected, found),
            (
                INDEX,
                editor_core::SlotKind::Is(editor_core::VarKind::Count),
                Dimension::Length
            )
        ),
        other => panic!("the edit door must refuse a length index, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    // The index slot reads a Count variable; the surgery swaps it for
    // a well-formed LENGTH variable, so the only rule left to refuse it
    // is the slot's own.
    let corrupt = doctored(&text, |wire| {
        let var = wire["snapshot"]["nodes"][pattern.0.to_string()]["Place"]["source"]["at"][0]
            .as_str()
            .expect("the index slot reads a variable")
            .to_owned();
        let held = &mut wire["snapshot"]["vars"][var.as_str()];
        assert_eq!(held["kind"], serde_json::json!("Count"));
        *held = serde_json::json!({
            "kind": "Length",
            "def": { "Free": { "Continuous": { "dim": "Length", "value": 3.0, "display_unit": "m" } } }
        });
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotVarKind {
            node,
            slot,
            found: declared,
            expected: editor_core::SlotKind::Is(referenced),
            ..
        })) => assert_eq!(
            (node.id(), slot, declared, referenced),
            (
                pattern,
                INDEX,
                editor_core::VarKind::Length,
                editor_core::VarKind::Count
            )
        ),
        other => panic!("the load door must refuse a length index, got {other:?}"),
    }
}

/// **The walk order is a contract**: a document broken in two ways at
/// once is refused by the EARLIER walk, so that walk's refusal is the
/// one every caller comparing the two doors reads.
///
/// This file is broken in a non-profile slot AND in its recorded ε,
/// and it reads the SLOT refusal — `validate_document`'s slot walk
/// runs before `validate_snapshot`, which is the order its docs name.
/// Moving a walk changes the diagnosis of every file broken both ways,
/// and this row is what says so out loud.
#[test]
fn rv_the_slot_walk_shadows_a_structural_refusal_it_did_not_shadow_before() {
    let (doc, extrude, _) = patterned();
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let corrupt = doctored(&text, |wire| {
        // (a) a non-profile slot retyped: distance Length -> Angle.
        crate::wire::retype_slot_var(
            wire,
            |wire| &wire["snapshot"]["nodes"][extrude.0.to_string()]["Extrude"]["distance"],
            "Angle",
            "rad",
        );
        // (b) the recorded ε broken too — `EpsilonInvalid`, a
        // `validate_snapshot` refusal.
        wire["snapshot"]["epsilon"] = serde_json::json!(-1.0);
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Snapshot(SnapshotError::SlotVarKind {
            node,
            slot,
            found: declared,
            expected: editor_core::SlotKind::Is(referenced),
            ..
        })) => assert_eq!(
            (node.id(), slot, declared, referenced),
            (
                extrude,
                SlotId::Distance,
                editor_core::VarKind::Angle,
                editor_core::VarKind::Length
            ),
            "the earlier walk's refusal, at the address it is about"
        ),
        other => panic!(
            "a file broken in a slot AND in its ε must read the slot walk's refusal — the walk \
             order `validate_document` documents. Got {other:?}"
        ),
    }
}

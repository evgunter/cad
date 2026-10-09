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
use editor_core::ExtrudeSide;

use crate::wire::doctored;
use editor_core::{
    Dimension, DocEdit, EditError, Formula, Node, PatternKind, PersistError, ProfileDoc,
    RecipeNodeId, SlotId, SnapshotError, apply, load, save,
};
use fixture::{insert, len, on_frame_keeping, scl, square};
use geom_core::Tol;

/// An extrude, patterned linearly — the pattern's `count` is a
/// Count-typed STRUCTURAL slot, the third node kind (after the extrude
/// and the frame datum the unit's own suite covers).
fn patterned() -> (ProfileDoc, RecipeNodeId) {
    let (doc, _, profile) = on_frame_keeping(
        ProfileDoc::empty(
            editor_core::DocumentId::derive("rv-onepred3"),
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
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: extrude.into(),
            count: Formula::count(3),
            kind: PatternKind::Linear {
                direction: [scl(1.0), scl(0.0), scl(0.0)],
                spacing: len(3.0),
            },
        },
    );
    (doc, pattern)
}

/// PROBE 1 — a THIRD node kind with a typed slot, and a Count one:
/// the pattern's instance count. Claim 1 of the review brief.
#[test]
fn rv_a_retyped_pattern_count_is_refused_at_both_doors() {
    let (doc, pattern) = patterned();
    match apply(
        &doc,
        &DocEdit::SetStructuralParam {
            node: pattern,
            slot: SlotId::Count,
            expr: len(3.0),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::SlotDimensionMismatch {
            slot,
            expected,
            found,
        }) => assert_eq!(
            (slot, expected, found),
            (
                SlotId::Count,
                editor_core::SlotKind::Is(editor_core::VarKind::Count),
                Dimension::Length
            )
        ),
        other => panic!("the edit door must refuse a length count, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    // The count slot reads a free Count variable; the surgery swaps it
    // for a well-formed LENGTH variable, so the only rule left to
    // refuse it is the slot's own.
    let corrupt = doctored(&text, |wire| {
        let var = wire["snapshot"]["nodes"][pattern.0.to_string()]["Pattern"]["count"]
            .as_str()
            .expect("the count slot reads a variable")
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
                SlotId::Count,
                editor_core::VarKind::Length,
                editor_core::VarKind::Count
            )
        ),
        other => panic!("the load door must refuse a length count, got {other:?}"),
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
    let (doc, pattern) = patterned();
    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let corrupt = doctored(&text, |wire| {
        // (a) a non-profile slot retyped: spacing Length -> Angle.
        crate::wire::retype_slot_var(
            wire,
            |wire| &wire["snapshot"]["nodes"][pattern.0.to_string()]["Pattern"]["kind"]["Linear"]["spacing"],
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
                pattern,
                SlotId::Spacing,
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

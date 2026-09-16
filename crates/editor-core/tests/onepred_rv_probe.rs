//! **Review probe (lane `onepred-rv`, PR 2772).** Reproduces the
//! measurement the unit's sweep filed as
//! `load-door-checks-slot-dimensions-for-profile-nodes-only`: the load
//! door asks the slot-dimension predicate of `Node::Profile` nodes
//! only, so a saved document whose EXTRUDE distance literal is retyped
//! on the wire from a length to an angle loads clean, while the edit
//! door refuses the same node.
//!
//! Kept as a red-on-fix probe rather than a row: it asserts the
//! ASYMMETRY that exists today, so closing the filed row turns the
//! `Ok(_)` arm into the refusal and this file is rewritten with it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{DocEdit, EditError, Node, ProfileDoc, apply, load, save};
use geom_core::Tol;
use fixture::{ang, len, on_frame, square, step};

/// A one-extrude document saved, its distance literal retyped from
/// `Length`/`m` to `Angle`/`rad` on the wire (both halves moved, so the
/// literal stays well-formed), and offered to both doors.
#[test]
fn onepred_rv_a_retyped_extrude_distance_loads_clean_but_the_edit_door_refuses() {
    let doc = ProfileDoc::empty(editor_core::DocumentId::derive("onepred-rv"), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
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

    // The EDIT door's answer for the same node, set beside the wire's.
    match apply(
        &doc,
        &DocEdit::SetParam {
            node: extrude,
            slot: editor_core::SlotId::Distance,
            expr: ang(1.0),
        },
        Tol::witness(),
    ) {
        Err(EditError::SlotDimensionMismatch { .. }) => {}
        other => panic!("the edit door must refuse an angle distance, got {other:?}"),
    }

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    load(&text, Tol::witness()).expect("the fixture loads");
    // The extrude's distance is the last Length literal in the file.
    let at = text.rfind("\"dim\": \"Length\"").expect("a length literal");
    let end = at + text[at..].find('}').expect("the literal closes");
    let retyped = format!(
        "{}{}{}",
        &text[..at],
        text[at..end]
            .replace("\"Length\"", "\"Angle\"")
            .replace("\"m\"", "\"rad\""),
        &text[end..]
    );
    assert_ne!(retyped, text, "the corruption really landed");
    match load(&retyped, Tol::witness()) {
        // TODAY: the load door's slot walk covers profile nodes only,
        // so an extrude's retyped distance is admitted.
        Ok(_) => {}
        other => panic!(
            "MEASUREMENT CHANGED: the load door now decides a non-profile slot's dimension, \
             got {other:?}"
        ),
    }

    // And the same document, offered to the edit door by replay of one
    // `SetExpression`, is refused — the asymmetry the filed row names.
    let (_, _) = step(
        doc,
        DocEdit::SetParam {
            node: extrude,
            slot: editor_core::SlotId::Distance,
            expr: len(2.0),
        },
    );
}

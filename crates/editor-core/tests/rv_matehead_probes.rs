//! **Review probes for `edit/mate-head-kind`** (lane `matehead-rv`).
//!
//! Not rows the branch owes — the reviewer's falsification attempts,
//! kept on the review branch so the fix pass can adopt or delete them.
//! Each says which claim of PR #2799 it is aimed at.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CapEnd, ContactClass, ContentPin, DocEdit, DocRef, DocumentId, EntityKey, EntityKind,
    EntityRef, InterfaceCrossing, InterfaceRecord, NameTable, Node, ProfileDoc, RecipeNodeId,
    RoleSeg, StableName, apply, load, save,
};
use geom_core::Tol;

fn face_name(node: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(cap)],
    }
}

/// **CLAIM 1 / CLAIM 8.** `Node::instantiate_part_with` is public and
/// an `InterfaceCrossing::Mate`'s `outer`/`inner` are bare
/// `StableName`s, so an EDGE-headed crossing is buildable IN MEMORY,
/// inserts, saves and LOADS. That is the filed row
/// `interface-crossing-heads-are-bare-stable-names` reproduced — and
/// it is wider than the row says: the row calls it "a hole in what a
/// FILE may carry", but no file is needed.
#[test]
fn probe_an_edge_headed_interface_crossing_inserts_saves_and_loads() {
    let doc_ref = DocRef {
        id: DocumentId::derive("rv-matehead-part"),
        pin: ContentPin([9u8; 32]),
    };
    let edge = StableName {
        kind: EntityKind::Edge,
        node: RecipeNodeId(0),
        path: vec![RoleSeg::Cap(CapEnd::End)],
    };
    let record = InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            mate: RecipeNodeId(0),
            class: ContactClass::Rest,
            outer: edge,
            inner: face_name(RecipeNodeId(1), CapEnd::Start),
        }],
    };
    let doc = apply(
        &ProfileDoc::empty(DocumentId::derive("rv-matehead-xing"), Tol::witness()),
        &DocEdit::InsertNode {
            node: Node::instantiate_part_with(doc_ref, record),
        },
        Tol::witness(),
    )
    .expect("an EDGE-headed crossing INSERTS")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("it saves");
    load(&text, Tol::witness())
        .expect("and it LOADS: the crossing record did not follow the head's type");
}

/// **CLAIM 3.** `RefusedRef::NotAFace` is said to now guard
/// `NameTable::insert`'s rule that a row's kind is its name's. That
/// rule is enforced by the one public door, so the state the arm
/// answers cannot be built from outside the crate: the arm is
/// unreachable, and what it documents is a crate bug, not a refusal.
#[test]
fn probe_the_name_table_refuses_the_only_row_not_a_face_could_guard() {
    let mut table = NameTable::default();
    let refused = table.insert(
        face_name(RecipeNodeId(0), CapEnd::End),
        EntityRef {
            body: 0,
            key: EntityKey::Body,
        },
    );
    assert!(
        refused.is_err(),
        "the ONE public door that could seat the row `NotAFace` answers \
         admitted it, so the arm would be reachable"
    );
}

/// **CLAIM 1 (the rebind door).** `rebind_payload_names` carries a
/// `debug_assert!(false)` for a `to` that is not a face, justified by
/// "its door refuses that pair". It does: `DocEdit::Rebind` answers
/// `RebindKindMismatch` before any payload is touched, so the assert
/// is unreachable through the one caller.
#[test]
fn probe_a_cross_kind_rebind_refuses_at_its_own_door() {
    use editor_core::EditError;
    let doc = ProfileDoc::empty(DocumentId::derive("rv-matehead-rebind"), Tol::witness());
    let from = face_name(RecipeNodeId(0), CapEnd::End);
    let to = StableName {
        kind: EntityKind::Edge,
        ..face_name(RecipeNodeId(0), CapEnd::Start)
    };
    match apply(&doc, &DocEdit::Rebind { from, to }, Tol::witness()) {
        Err(EditError::RebindKindMismatch { from, to }) => {
            assert_eq!((from, to), (EntityKind::Face, EntityKind::Edge));
        }
        other => panic!("the rebind door must refuse the pair first, got {other:?}"),
    }
}

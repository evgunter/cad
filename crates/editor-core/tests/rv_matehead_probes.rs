//! **What a mate head's type does and does not decide.**
//!
//! Three rows around the edge of the claim that a mate head is a
//! `FaceName`, each measuring one thing that claim does NOT reach:
//!
//! - an `InterfaceCrossing::Mate`'s `outer`/`inner` are bare
//!   `StableName`s, so an EDGE-headed crossing is buildable in memory
//!   through `Node::instantiate_part_with`, inserts, saves and loads.
//!   The row that owns the repair is
//!   `work/edit/interface-crossing-heads-are-bare-stable-names`; this
//!   is the measurement that pins the hole open until it is taken, and
//!   it is wider than a file: no file is needed to build one.
//! - the name table refuses a row whose KEY disagrees with its name's
//!   kind, at both of its two seating doors. That is why the assembly
//!   gate's refusal vocabulary has no kind arm: the state such an arm
//!   would have answered cannot be seated, so what it documented was a
//!   crate bug rather than a refusal.
//! - `DocEdit::Rebind` refuses a cross-kind pair at its own door,
//!   before any payload is touched, which is what makes the mate arm
//!   of `Node::rebind_payload_names` a `debug_assert` rather than a
//!   refusal it would have to invent.

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

/// **An interface crossing's heads are not the mate's type.** A mate's
/// two heads are `SitedFace`s, so a mate naming an edge does not
/// compile and a file carrying one refuses at the wire door's
/// constructor. `InterfaceCrossing::Mate`'s `outer`/`inner` did not
/// follow: they are bare `StableName`s written by the split out of two
/// heads, so an edge-headed crossing INSERTS, SAVES and LOADS — no
/// file needed, `Node::instantiate_part_with` is public.
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

/// **A row's kind is its name's, at both seating doors.** The assembly
/// gate resolves a head — a face by its type — against a name table,
/// and reads the answer's key as a face. It may, because
/// `NameTable::insert` and `NameTable::insert_tied` are the only doors
/// that seat a row and each refuses a key whose kind disagrees with
/// the name's. So the state a kind arm in that gate's vocabulary would
/// have answered is unseatable, which is why the gate asserts it
/// instead of naming it.
#[test]
fn probe_the_name_table_refuses_a_key_that_disagrees_with_its_name() {
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
        "`insert` seated a BODY key under a face name, so a kind arm at the gate \
         would be reachable"
    );

    let refused = table.insert_tied(
        face_name(RecipeNodeId(1), CapEnd::End),
        vec![
            EntityRef {
                body: 0,
                key: EntityKey::Body,
            },
            EntityRef {
                body: 1,
                key: EntityKey::Body,
            },
        ],
    );
    assert!(
        refused.is_err(),
        "`insert_tied` seated BODY keys under a face name, so a tie at the gate could \
         hold a non-face"
    );
}

/// **A rebind never crosses entity kinds.** `DocEdit::Rebind` answers
/// `RebindKindMismatch` before any payload is touched, so the mate arm
/// of `Node::rebind_payload_names` — which must produce a `FaceName`
/// for `to` — meets a face whenever it can replace a head at all. That
/// is what its `debug_assert` stands on, and this is the door it
/// stands on.
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

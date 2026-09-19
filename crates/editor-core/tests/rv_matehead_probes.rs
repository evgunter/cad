//! **What a mate head's type does and does not decide.**
//!
//! Two rows around the edge of the claim that a mate head is a
//! `FaceName`, each measuring one thing that claim does NOT reach:
//!
//! - the name table refuses a row whose KEY disagrees with its name's
//!   kind, at both of its two seating doors. That is why the assembly
//!   gate's refusal vocabulary has no kind arm: the state such an arm
//!   would have answered cannot be seated, so what it documented was a
//!   crate bug rather than a refusal.
//! - `DocEdit::Rebind` refuses a cross-kind pair at its own door,
//!   before any payload is touched, which is what lets the mate arm of
//!   `Node::rebind_payload_names` take `to`'s DERIVATION and keep the
//!   head's kind — no second refusal to invent, and no arm to assert
//!   away.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CapEnd, DocEdit, DocumentId, EntityKey, EntityKind, EntityRef, NameTable, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, apply,
};
use geom_core::Tol;

fn face_name(node: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(cap)],
    }
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
/// is what lets it rewrite the head's DERIVATION and keep the kind the
/// type holds, and this is the door it stands on.
#[test]
fn probe_a_cross_kind_rebind_refuses_at_its_own_door() {
    use editor_core::EditError;
    let doc = ProfileDoc::empty(DocumentId::derive("rv-matehead-rebind"), Tol::witness());
    let from = face_name(RecipeNodeId(0), CapEnd::End);
    let to = StableName {
        kind: EntityKind::Edge,
        ..face_name(RecipeNodeId(0), CapEnd::Start)
    };
    match apply(
        &doc,
        &DocEdit::Rebind { from, to },
        Tol::witness(),
        &editor_core::RefusingReach,
    ) {
        Err(EditError::RebindKindMismatch { from, to }) => {
            assert_eq!((from, to), (EntityKind::Face, EntityKind::Edge));
        }
        other => panic!("the rebind door must refuse the pair first, got {other:?}"),
    }
}

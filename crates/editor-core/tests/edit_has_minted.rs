//! **`Doc::has_minted` — the mint log's one reading of a node id.**
//!
//! `crates/editor-core/IDENTITY.md` DI1 rules that a layer-3 hold of a
//! `RecipeNodeId` carries the id plus the history entry that minted
//! it, and that the entry is computed by walking up the history until
//! the document no longer holds the id. The walk is VIEW's; the reading
//! it walks on is this crate's, and `Doc::has_minted` is it.
//!
//! The rows below are the four facts that walk rests on:
//!
//! - a fresh document has minted NOTHING, so the walk's predicate is
//!   false at the root of a history that starts empty;
//! - an insert mints one id and the predicate turns true for it, and
//!   stays false for the id the next insert would mint — which is what
//!   makes the walk's step a decision rather than a guess;
//! - a DELETE does not move it back: ids are never reused (spec D3;
//!   the mint log keeps a deleted node's id), so the predicate answers
//!   minting and not liveness, and a walk that read it as liveness
//!   would be reading the wrong question;
//! - a save/load round trip preserves it, because the mint is part of
//!   the document VALUE (ASM-1 D-3: it is in the content pin's
//!   preimage) — a history entry reconstituted from a file answers
//!   the walk the same way the entry in memory did.
//!
//! What these rows do NOT pin: the mint log's own layout. A test that
//! read it would be asserting from beside the predicate rather than
//! through it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/doc.rs",
    "crates/editor-core/src/edit.rs",
    "crates/editor-core/tests/fixture/",
    "crates/editor-core/src/test_support.rs",
];

use crate::fixture;
use editor_core::ExtrudeSide;

use editor_core::{DocEdit, DocumentId, Node, ProfileDoc, RecipeNodeId, load, save};
use fixture::{desc, insert, len, next_mint, square, step, xy_frame};
use geom_core::Tol;

/// A fresh document under a derived id — nothing inserted, nothing
/// minted.
fn fresh(label: &str) -> ProfileDoc {
    ProfileDoc::empty(DocumentId::derive(label), Tol::witness())
}

/// Row 1 — the empty document has minted nothing: before any insert,
/// not even the id its first insert will mint is hers.
#[test]
fn a_fresh_document_has_minted_nothing() {
    let doc = fresh("has-minted-fresh");
    let first = next_mint(&doc);
    assert!(
        !doc.has_minted(first),
        "the empty document has minted nothing, and {first:?} is nothing"
    );
    for raw in 0..4u64 {
        assert!(
            !doc.has_minted(RecipeNodeId(raw)),
            "the empty document has minted nothing, and {raw} is nothing"
        );
    }
}

/// Row 2 — an insert mints exactly one id: the predicate is true at
/// the minted id and false at the one the next insert would mint.
#[test]
fn an_insert_mints_that_id_and_not_the_next_one() {
    let doc = fresh("has-minted-insert");
    let (doc, plane) = insert(doc, xy_frame());
    assert!(
        doc.has_minted(plane),
        "the id the insert returned is the id it minted"
    );
    let upcoming = next_mint(&doc);
    assert!(
        !doc.has_minted(upcoming),
        "the next id is not minted until the next insert mints it"
    );

    let (doc, profile) = insert(doc, Node::Profile(desc(plane, vec![square(0.0, 0.0, 1.0)])));
    assert_ne!(profile, plane, "each insert mints its own id");
    for id in [plane, profile] {
        assert!(doc.has_minted(id), "{id:?} was minted and stays so");
    }
    assert!(
        !doc.has_minted(next_mint(&doc)),
        "the next insert's id is not already minted"
    );
}

/// Row 3 — a delete does not un-mint. The node is gone from
/// `Doc::node` and the id stays minted, which is the divide DI1's
/// rule spells as two questions: descent, then liveness.
#[test]
fn a_delete_leaves_the_id_minted() {
    let doc = fresh("has-minted-delete");
    let (doc, plane) = insert(doc, xy_frame());
    let (doc, profile) = insert(doc, Node::Profile(desc(plane, vec![square(0.0, 0.0, 1.0)])));
    let (deleted, _) = step(doc, DocEdit::DeleteNode { id: profile });

    assert!(
        deleted.node(profile).is_none(),
        "the delete took the node out of the document"
    );
    assert!(
        deleted.has_minted(profile),
        "and left its id minted — ids are never reused (D3)"
    );
    assert!(
        !deleted.has_minted(next_mint(&deleted)),
        "the next insert's id is not already minted"
    );

    // And the next insert takes a fresh id, not the freed one: the
    // predicate's answer above is the reason, spelled as behaviour.
    let (reinserted, next) = insert(
        deleted,
        Node::Profile(desc(plane, vec![square(0.0, 0.0, 2.0)])),
    );
    assert_ne!(next, profile, "the deleted id is not handed out again");
    assert!(reinserted.has_minted(next));
}

/// Row 4 — the mint is part of the value, so it survives a save
/// and a load: a history entry rebuilt from a file answers the walk
/// exactly as the entry in memory did. The deleted id is the sharp
/// case, because nothing in the loaded document's node map names it.
#[test]
fn the_answer_survives_a_save_load_round_trip() {
    let doc = fresh("has-minted-round-trip");
    let (doc, plane) = insert(doc, xy_frame());
    let (doc, profile) = insert(doc, Node::Profile(desc(plane, vec![square(0.0, 0.0, 1.0)])));
    let (doc, extrude) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(0.5),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, scratch) = insert(doc, Node::Profile(desc(plane, vec![square(4.0, 4.0, 1.0)])));
    let (doc, _) = step(doc, DocEdit::DeleteNode { id: scratch });

    let text = save(&doc, &[], Tol::witness()).expect("the fixture saves");
    let loaded = load(&text, Tol::witness()).expect("and loads");

    for id in [plane, profile, extrude, scratch] {
        assert_eq!(
            loaded.doc.has_minted(id),
            doc.has_minted(id),
            "the round trip changed the minting answer for {id:?}"
        );
        assert!(
            loaded.doc.has_minted(id),
            "{id:?} was minted before the save"
        );
    }
    assert!(
        loaded.doc.node(scratch).is_none(),
        "the deleted node did not come back"
    );
    assert!(
        !loaded.doc.has_minted(next_mint(&doc)),
        "and the load did not advance the mint past what was saved"
    );
}

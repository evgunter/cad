//! **The interface record on the wire** (ASM-R2b D-4, discharging the
//! obligation ASM-4 wrote at `InterfaceCrossing`).
//!
//! While `InterfaceCrossing` was uninhabited the instantiate node's
//! `interface` key could not appear on the wire at all: every record
//! was provably empty, `skip_serializing_if` dropped it, and it fed no
//! content key. Now a split that a mate crosses writes a populated
//! record, and that record is file data — so it must round-trip, and
//! an empty record must still cost no bytes. (The format carries no
//! schema version — the persist module docs say why — so there is no
//! version pin here and no older golden to refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CapEnd, ContactClass, ContentPin, DocEdit, DocRef, DocumentId, EntityKind, InterfaceCrossing,
    InterfaceRecord, Node, ProfileDoc, RecipeNodeId, RoleSeg, StableName, apply, load, save,
};
use geom_core::Tol;

/// A document carrying an INHABITED record, built by hand so the row
/// does not depend on the split that normally mints one.
fn doc_with_a_crossing() -> ProfileDoc {
    let doc_ref = DocRef {
        id: DocumentId::derive("asm-r2b-schema-part"),
        pin: ContentPin([9u8; 32]),
    };
    let face = |node, cap| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::Cap(cap)],
    };
    let record = InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            mate: RecipeNodeId(0),
            class: ContactClass::Rest,
            outer: face(RecipeNodeId(0), CapEnd::Top),
            inner: face(RecipeNodeId(1), CapEnd::Bottom),
        }],
    };
    apply(
        &ProfileDoc::empty(DocumentId::derive("asm-r2b-schema"), Tol::witness()),
        &DocEdit::InsertNode {
            node: Node::instantiate_part_with(doc_ref, record),
        },
        Tol::witness(),
    )
    .expect("an instance with a record inserts")
    .doc
}

/// The record is ON THE WIRE (it was unspellable while the enum was
/// uninhabited), and it round-trips bit for bit.
#[test]
fn an_inhabited_interface_record_round_trips() {
    let doc = doc_with_a_crossing();
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(
        text.contains("crossings"),
        "the record is on the wire: {text}"
    );
    assert!(
        text.contains("\"rest\""),
        "the class rides the SAME stable spelling a Declare pair's and a \
         Mate's do: {text}"
    );
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the record round-trips bit for bit");
}

/// INVARIANT: an EMPTY record still costs no bytes and moves no pin —
/// inhabiting the enum did not make every instance pay for the
/// feature (`skip_serializing_if` on the empty state, unchanged).
#[test]
fn an_empty_record_stays_absent_from_the_wire() {
    let doc_ref = DocRef {
        id: DocumentId::derive("asm-r2b-schema-part"),
        pin: ContentPin([9u8; 32]),
    };
    let doc = apply(
        &ProfileDoc::empty(DocumentId::derive("asm-r2b-schema-empty"), Tol::witness()),
        &DocEdit::InsertNode {
            node: Node::instantiate_part(doc_ref),
        },
        Tol::witness(),
    )
    .expect("an instance inserts")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(
        !text.contains("crossings"),
        "an authored instance crosses nothing, and says nothing: {text}"
    );
}

// The content-key half of ASM-4's obligation is pinned in
// `asm_r2b_assembly` (`row6_a_crossing_record_edit_moves_the_content_key`),
// where a resolver exists so both documents' instantiate nodes actually
// evaluate and their keys are observable.

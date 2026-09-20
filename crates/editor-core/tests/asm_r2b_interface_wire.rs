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

use crate::wire::{doctored, wire_body};
use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, ContentPin, DocEdit, DocRef, DocumentId,
    EntityKind, FaceName, InterfaceCrossing, InterfaceRecord, MateFrame, MatePrimitive, Node,
    PersistError, ProfileDoc, RecipeNodeId, RefusingReach, RoleSeg, SitedFace, StableName, apply,
    load, save,
};
use geom_core::Tol;

/// A document carrying an INHABITED record, built by hand so the row
/// does not depend on the split that normally mints one.
fn doc_with_a_crossing() -> ProfileDoc {
    let doc_ref = DocRef {
        id: DocumentId::derive("asm-r2b-schema-part"),
        pin: ContentPin([9u8; 32]),
    };
    let face = |node, cap| {
        FaceName::new(StableName {
            kind: EntityKind::Face,
            node,
            path: vec![RoleSeg::Cap(cap)],
        })
        .expect("a crossing's references are face names")
    };
    // The crossing's one reference INTO this document — its `outer` —
    // is a payload name the insert door checks is live, so the record
    // rides a LAST instance, behind the two mate ends and the mate
    // itself. That is the shape a split leaves behind.
    let mut host = ProfileDoc::empty(DocumentId::derive("asm-r2b-schema"), Tol::witness());
    // Inserts alone — a Join at most, never a moved gauge — so the
    // reach is never asked and the refusing one serves.
    let push = |doc: &ProfileDoc, node| {
        apply(
            doc,
            &DocEdit::InsertNode { node },
            Tol::witness(),
            &RefusingReach,
        )
        .expect("the fixture's nodes insert")
        .doc
    };
    host = push(&host, Node::instantiate_part(doc_ref));
    host = push(&host, Node::instantiate_part(doc_ref));
    let sited = |node, cap| SitedFace {
        at: node,
        name: face(node, cap),
    };
    let frame = MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    };
    host = push(
        &host,
        Node::Mate {
            a: sited(RecipeNodeId(0), CapEnd::End),
            b: sited(RecipeNodeId(1), CapEnd::Start),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: frame,
                b: frame,
                primitive: MatePrimitive::FrameCoincidence,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
    );
    let record = InterfaceRecord {
        crossings: vec![InterfaceCrossing::Mate {
            class: ContactClass::Rest,
            outer: face(RecipeNodeId(0), CapEnd::End),
            // The `inner` is spelled in the PART's id space, and the
            // value is chosen to make that visible: `RecipeNodeId(7)`
            // is not a live node of this document at all, so a wire
            // that round-trips it round-trips a reference NO door
            // here resolves — which is the distinction the record
            // exists to carry across the seam.
            inner: face(RecipeNodeId(7), CapEnd::Start),
        }],
    };
    push(&host, Node::instantiate_part_with(doc_ref, record))
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
        &editor_core::RefusingReach,
    )
    .expect("an instance inserts")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(
        !text.contains("crossings"),
        "an authored instance crosses nothing, and says nothing: {text}"
    );
}

/// The saved fixture's one crossing, as the wire object it is — the
/// path the two rows below read and corrupt.
///
/// BY PATH, not by search: the instance carrying the record is the
/// document's LAST node, so a fixture change breaks the surgery loudly
/// instead of landing it on a neighbour.
fn crossing_of(wire: &mut serde_json::Value, instance: RecipeNodeId) -> &mut serde_json::Value {
    &mut wire["snapshot"]["nodes"][instance.0.to_string()]["InstantiatePart"]["interface"]["crossings"]
        [0]["Mate"]
}

/// The fixture, saved, with the id of the instance carrying its
/// record.
fn saved_crossing() -> (String, RecipeNodeId) {
    let doc = doc_with_a_crossing();
    let instance = *doc.order().last().expect("the fixture has nodes");
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    (text, instance)
}

/// **A crossing is THREE fields on the wire** — the class it declares
/// and its two references, and nothing else.
///
/// A crossing carries no provenance — `InterfaceCrossing::Mate`'s own
/// doc argues why — and this is that absence as BYTES: a field no door
/// reads is a cost every file with a record would pay. Read off the
/// SAVED document rather than a serialized value, because a file is
/// what the row below corrupts and what a stale writer produces.
#[test]
fn a_crossing_is_three_fields_on_the_wire() {
    let (text, instance) = saved_crossing();
    let mut wire = wire_body(&text);
    let mut keys: Vec<String> = crossing_of(&mut wire, instance)
        .as_object()
        .expect("a crossing is an object")
        .keys()
        .cloned()
        .collect();
    keys.sort();
    assert_eq!(
        keys,
        ["class", "inner", "outer"],
        "the crossing's wire form is its class and its two references"
    );
}

/// **A file whose crossing carries a FOURTH field refuses at the load
/// door**, in the door's own `Unreadable` class.
///
/// `InterfaceCrossing` is `deny_unknown_fields`, so its shape is
/// CLOSED on the wire: a file carrying a field this build does not
/// know is refused rather than read with the field dropped. The
/// refusal is TYPED BY THE DOOR — serde_json classifies the failure
/// `Data`, and `persist`'s one seam maps that class to
/// `PersistError::Unreadable` — while the sentence inside it is
/// serde's own.
#[test]
fn a_file_whose_crossing_carries_a_fourth_field_refuses_at_the_load_door() {
    let (text, instance) = saved_crossing();
    let corrupt = doctored(&text, |wire| {
        let crossing = crossing_of(wire, instance);
        assert!(
            crossing.get("mate").is_none(),
            "the surgery adds a field the format does not have: {crossing}"
        );
        crossing["mate"] = serde_json::json!(2);
    });
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => assert!(
            detail.contains("mate"),
            "the refusal names the field the file carried: {detail:?}"
        ),
        other => panic!("a crossing carrying a fourth field must refuse at load, got {other:?}"),
    }
}

// The content-key half of ASM-4's obligation is pinned in
// `asm_r2b_assembly` (`row6_a_crossing_record_edit_moves_the_content_key`),
// where a resolver exists so both documents' instantiate nodes actually
// evaluate and their keys are observable.

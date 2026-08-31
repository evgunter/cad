//! **The interface record inhabited** (ASM-R2b D-4, discharging the
//! obligation ASM-4 wrote at `InterfaceCrossing`) — schema **v14**,
//! and the gate that refuses everything older.
//!
//! Before v14 the instantiate node's `interface` key could not appear
//! on the wire at all: `InterfaceCrossing` was uninhabited, so every
//! record was provably empty, `skip_serializing_if` dropped it, and it
//! fed no content key. Now a split that a mate crosses writes a
//! populated record, and that record is file data. A v13 reader handed
//! such a file meets a key with a shape it has no variant for and dies
//! inside serde rather than at the version door — which is exactly the
//! direction the gate buys. The other direction is forgiving by
//! construction (a v13 file has no crossings), so the disposition is
//! the family's: the older file refuses TYPED with the regenerate
//! recourse, and the migration table stays empty.
//!
//! **Why 14.** Read by eye from main's constant at the final re-merge
//! (`git show origin/main:crates/editor-core/src/persist/mod.rs | grep
//! SCHEMA_VERSION`), because three consecutive units have now had a
//! same-number claim merge CLEAN — both sides write the identical
//! line, so git never conflicts. The claim also lives as prose in
//! `docs/MODEL-AB-LOG.md`, where a second claimant collides.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CapEnd, ContactClass, ContentPin, DocEdit, DocRef, DocumentId, EntityKind, InterfaceCrossing,
    InterfaceRecord, Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, RecipeNodeId, RoleSeg,
    SCHEMA_VERSION, StableName, apply, load, save,
};
use geom_core::Tol;

/// The prior live golden, kept as the REFUSAL fixture: a break nobody
/// can demonstrate is a break nobody can trust.
const V13: &str = include_str!("golden/v13_golden.cad");
/// One further back, to show the gate has no notion of "nearly
/// current".
const V12: &str = include_str!("golden/v12_golden.cad");

#[test]
fn schema_version_is_current() {
    // Named for the PROPERTY, not the number (the `lbret_schema_v8`
    // precedent): ASM-R2b's own bump was v14; M10-1 took v15 for
    // parameter distributions and LIB-G16 v16, and the number is what
    // keeps moving.
    assert_eq!(SCHEMA_VERSION, 19);
}

#[test]
fn the_checked_in_older_goldens_are_really_older() {
    assert_eq!(V13.lines().next(), Some("schema: 13"));
    assert_eq!(V12.lines().next(), Some("schema: 12"));
}

/// The break, demonstrated in the direction that matters: a v13 file
/// refuses TYPED at the version door, naming the version found, the
/// version supported, and the step that does not exist.
#[test]
fn v13_refuses_too_old() {
    match load(V13, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 13);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 13,
                "the 13 → 14 step is the one that does not exist"
            );
        }
        other => panic!("v13 must refuse SchemaTooOld, got {other:?}"),
    }
}

#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    for (label, bytes) in [("v13", V13), ("v12", V12)] {
        let msg = match load(bytes, Tol::witness()) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("{label} must refuse"),
        };
        assert!(msg.contains(REGENERATE_RECOURSE), "{label}: {msg}");
    }
}

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

/// The other direction, stated: a v14 writer's header is the current
/// number, the record is ON THE WIRE (it was unspellable before), and
/// it round-trips bit for bit.
#[test]
fn an_inhabited_interface_record_round_trips_at_v14() {
    let doc = doc_with_a_crossing();
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text.lines().next(),
        Some(&format!("schema: {SCHEMA_VERSION}")[..]),
        "a fresh save carries the CURRENT version"
    );
    assert!(
        text.contains("crossings"),
        "the record is on the wire — the v14 format change itself: {text}"
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

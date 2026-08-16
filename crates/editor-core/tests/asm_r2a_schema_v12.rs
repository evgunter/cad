//! **The mate-node clean break** (ASSEMBLY-DESIGN A3/A12, ratified
//! #522; ASM-R2a D-1) — schema **v12**, and the gate that refuses
//! everything older.
//!
//! [`Node::Mate`] is new node vocabulary: a v11 reader handed a v12
//! file meets a variant it has no arm for, and would die inside serde
//! rather than at the version door. That direction is what the gate
//! buys — the other one is forgiving by construction, since a v11 file
//! simply contains no mate. It is the same case v7 bumped for (the
//! `InstantiatePart` arm) and v2/v3/v8 before it, so the disposition
//! is the same: the older file refuses TYPED with the regenerate
//! recourse, and the migration table stays empty.
//!
//! **Why 12, and how it was checked.** This unit set its constant
//! after reading main's BY EYE (`git show
//! origin/main:crates/editor-core/src/persist/mod.rs`), which held 11
//! — M9-1 PR-2's declaration-class break, merged while this branch was
//! open. The unit had claimed 11 for itself and moved. The check is an
//! explicit step and not a merge outcome: the ledger records the
//! finding that two branches claiming the same number merge CLEANLY,
//! because the constant is one line of identical text and git sees no
//! conflict at all.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, ContentPin, DocEdit, DocRef, DocumentId,
    EntityKind, MateFrame, MatePrimitive, Node, PersistError, ProfileDoc, REGENERATE_RECOURSE,
    RecipeNodeId, RoleSeg, SCHEMA_VERSION, StableName, apply, load, save,
};

/// The prior live golden, kept as the REFUSAL fixture: a break nobody
/// can demonstrate is a break nobody can trust (the M5 PR 10
/// precedent, verbatim).
const V11: &str = include_str!("golden/v11_golden.cad");
/// Two further back, to show the gate has no notion of "nearly
/// current".
const V10: &str = include_str!("golden/v10_golden.cad");

#[test]
fn schema_version_is_twelve() {
    assert_eq!(SCHEMA_VERSION, 12);
}

#[test]
fn the_checked_in_v11_file_is_really_v11() {
    assert_eq!(V11.lines().next(), Some("schema: 11"));
    assert_eq!(V10.lines().next(), Some("schema: 10"));
}

/// The break, demonstrated: a v11 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the step
/// that does not exist.
#[test]
fn v11_refuses_too_old() {
    match load(V11) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 11);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 11,
                "the 11 → 12 step is the one that does not exist"
            );
        }
        other => panic!("v11 must refuse SchemaTooOld, got {other:?}"),
    }
}

#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    for (label, bytes) in [("v11", V11), ("v10", V10)] {
        let msg = match load(bytes) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("{label} must refuse"),
        };
        assert!(msg.contains(REGENERATE_RECOURSE), "{label}: {msg}");
    }
}

/// The other direction, stated: a v12 writer's header is the current
/// number, and a document carrying a mate round-trips through it.
#[test]
fn a_mate_bearing_document_round_trips_at_v12() {
    let doc_ref = DocRef {
        id: DocumentId::derive("asm-r2a-schema-part"),
        pin: ContentPin([7u8; 32]),
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive("asm-r2a-schema"));
    let mut ids = Vec::new();
    for _ in 0..2 {
        let applied = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Node::instantiate_part(doc_ref),
            },
        )
        .expect("an instance inserts");
        ids.push(applied.record.minted.expect("a minted id"));
        doc = applied.doc;
    }
    let name = |node| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(CapEnd::Bottom)],
            }),
        }],
    };
    let f = MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    };
    let doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Mate {
                a: name(ids[0]),
                b: name(ids[1]),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a: f,
                    b: f,
                    primitive: MatePrimitive::PlanarRest { offset: 0.5 },
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            },
        },
    )
    .expect("a mate inserts")
    .doc;
    let text = save(&doc, &[]).expect("saves");
    assert_eq!(
        text.lines().next(),
        Some(&format!("schema: {SCHEMA_VERSION}")[..]),
        "a fresh save carries the CURRENT version"
    );
    assert!(
        text.contains("\"rest\""),
        "the class rides the SAME stable spelling a Declare pair's does: {text}"
    );
    let back = load(&text).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the mate round-trips bit for bit");
}

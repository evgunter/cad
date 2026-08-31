//! **The mate-node clean break** (ASSEMBLY-DESIGN A3/A12, ratified
//! #522; ASM-R2a D-1) — schema **v13**, and the gate that refuses
//! everything older.
//!
//! [`Node::Mate`] is new node vocabulary: a v12 reader handed a v13
//! file meets a variant it has no arm for, and would die inside serde
//! rather than at the version door. That direction is what the gate
//! buys — the other one is forgiving by construction, since a v11 file
//! simply contains no mate. It is the same case v7 bumped for (the
//! `InstantiatePart` arm) and v2/v3/v8 before it, so the disposition
//! is the same: the older file refuses TYPED with the regenerate
//! recourse, and the migration table stays empty.
//!
//! **Why 13, and how it was checked TWICE.** This unit claimed 11,
//! moved to 12 when M9-1 PR-2 merged with 11, and moved to 13 when
//! LIB-PLACEDUNION merged with 12. Both shifts were caught by an
//! explicit read of main's constant at the re-merge (`git show
//! origin/main:crates/editor-core/src/persist/mod.rs | grep
//! SCHEMA_VERSION`) and by nothing else: neither produced a merge
//! conflict, because both sides had written the identical line. Three
//! consecutive units have now reproduced that failure mode, which is
//! why the claim lives as prose in the shared ledger — prose collides,
//! a one-line constant does not.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, ContentPin, DocEdit, DocRef, DocumentId,
    EntityKind, MateFrame, MatePrimitive, Node, PersistError, ProfileDoc, REGENERATE_RECOURSE,
    RecipeNodeId, RoleSeg, SCHEMA_VERSION, StableName, apply, load, save,
};
use geom_core::Tol;

/// The prior live golden, kept as the REFUSAL fixture: a break nobody
/// can demonstrate is a break nobody can trust (the M5 PR 10
/// precedent, verbatim).
const V12: &str = include_str!("golden/v12_golden.cad");
/// Two further back, to show the gate has no notion of "nearly
/// current".
const V11: &str = include_str!("golden/v11_golden.cad");

#[test]
fn schema_version_is_current() {
    // Named for the PROPERTY, not the number (the `lbret_schema_v8`
    // precedent): ASM-R2a's own bump was v13; ASM-R2b took v14 when it
    // inhabited the interface record and M10-1 took v15 for parameter
    // distributions, and the number is exactly what keeps moving.
    assert_eq!(SCHEMA_VERSION, 18);
}

#[test]
fn the_checked_in_v12_file_is_really_v12() {
    assert_eq!(V12.lines().next(), Some("schema: 12"));
    assert_eq!(V11.lines().next(), Some("schema: 11"));
}

/// The break, demonstrated: a v12 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the step
/// that does not exist.
#[test]
fn v12_refuses_too_old() {
    match load(V12, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 12);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 12,
                "the 12 → 13 step is the one that does not exist"
            );
        }
        other => panic!("v12 must refuse SchemaTooOld, got {other:?}"),
    }
}

#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    for (label, bytes) in [("v12", V12), ("v11", V11)] {
        let msg = match load(bytes, Tol::witness()) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("{label} must refuse"),
        };
        assert!(msg.contains(REGENERATE_RECOURSE), "{label}: {msg}");
    }
}

/// The other direction, stated: a v13 writer's header is the current
/// number, and a document carrying a mate round-trips through it.
#[test]
fn a_mate_bearing_document_round_trips_at_v13() {
    let doc_ref = DocRef {
        id: DocumentId::derive("asm-r2a-schema-part"),
        pin: ContentPin([7u8; 32]),
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive("asm-r2a-schema"), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let applied = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Node::instantiate_part(doc_ref),
            },
            Tol::witness(),
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
        Tol::witness(),
    )
    .expect("a mate inserts")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert_eq!(
        text.lines().next(),
        Some(&format!("schema: {SCHEMA_VERSION}")[..]),
        "a fresh save carries the CURRENT version"
    );
    assert!(
        text.contains("\"rest\""),
        "the class rides the SAME stable spelling a Declare pair's does: {text}"
    );
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the mate round-trips bit for bit");
}

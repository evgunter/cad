//! **The mate on the wire** (ASSEMBLY-DESIGN A3/A12, ratified #522;
//! ASM-R2a D-1): a mate is a constraint in a placement's bundle, node
//! vocabulary, so a document carrying one must cross the persistence
//! door as itself.
//! This suite is that round trip. (The format carries no schema
//! version — the persist module docs say why — so there is no version
//! pin here and no older golden to refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, ContentPin, DocEdit, DocRef, DocumentId,
    EntityKind, MateFrame, MatePrimitive, Node, ProfileDoc, RecipeNodeId, RoleSeg, StableName,
    apply, load, save,
};
use geom_core::Tol;

/// A document carrying a mate round-trips through the persistence
/// door bit for bit, and the mate's class rides the same stable
/// spelling a declared pair's does.
#[test]
fn a_mate_bearing_document_round_trips() {
    let doc_ref = DocRef {
        id: DocumentId::derive("asm-r2a-schema-part"),
        pin: ContentPin([7u8; 32]),
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive("asm-r2a-schema"), Tol::witness());
    let mut ids = Vec::new();
    // The first instance is pinned to the world; the second sits where
    // the mate puts it, so the mate is its bundle.
    for node in [
        Node::instantiate_part(doc_ref),
        crate::fixture::mated_instance(doc_ref),
    ] {
        let applied = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("an instance inserts");
        ids.push(applied.record.minted.expect("a minted id"));
        doc = applied.doc;
    }
    let name = |node| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, 1),
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    };
    let f = MateFrame::authored(
        [0.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        geom_core::Tol::witness(),
    )
    .expect("a definite frame");
    let doc = apply(
        &doc,
        &DocEdit::AddConstraint {
            placement: ids[1],
            mate: Box::new(crate::fixture::mate(
                crate::fixture::head(name(ids[0])),
                crate::fixture::head(name(ids[1])),
                ContactClass::Rest,
                Alignment {
                    a: f.clone(),
                    b: f,
                    primitive: MatePrimitive::PlanarRest { offset: 0.5 },
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            )),
            fresh: Vec::new(),
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a mate inserts")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(
        text.contains("\"rest\""),
        "the class rides the SAME stable spelling a Declare pair's does: {text}"
    );
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the mate round-trips bit for bit");
}

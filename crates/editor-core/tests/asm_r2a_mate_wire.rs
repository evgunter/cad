//! **The mate on the wire** (ASSEMBLY-DESIGN A3/A12, ratified #522;
//! ASM-R2a D-1): a mate is a constraint in a placement's bundle, node
//! vocabulary, so a document carrying one must cross the persistence
//! door as itself.
//! This suite is that round trip. (The format carries no schema
//! version — the persist module docs say why — so there is no version
//! pin here and no older golden to refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{flip, head, len, plane};
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, ContentPin, DocEdit, DocRef, DocumentId, EntityKind, Operand, ProfileDoc, RecipeNodeId,
    RoleSeg, StableName, apply, load, save,
};
use geom_core::Tol;

/// A document carrying a mate round-trips through the persistence
/// door bit for bit — a world read, a flip and a standoff among its
/// poses.
#[test]
fn a_mate_bearing_document_round_trips() {
    let doc_ref = DocRef {
        id: DocumentId::derive("asm-r2a-schema-part"),
        pin: ContentPin([7u8; 32]),
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive("asm-r2a-schema"), Tol::witness());
    let mut ids = Vec::new();
    // The first instance rests on the world's xy plane; the second
    // sits where the mate puts it, so the mate is its bundle.
    for node in [
        crate::fixture::mated_instance(doc_ref),
        crate::fixture::mated_instance(doc_ref),
    ] {
        let applied = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("an instance inserts");
        ids.push(applied.record.minted.expect("a minted id"));
        doc = applied.doc;
    }
    // A part's cap, read at the instance whose copy carries it.
    let name = |node| crate::fixture::resolver::PartName {
        at: node,
        name: StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId::new(0, 1),
            path: vec![RoleSeg::Cap(CapEnd::Start)],
        },
    };
    let add = |doc: &ProfileDoc, placement, mate| {
        apply(
            doc,
            // The mate and a zero on each freedom it leaves, written
            // whole: every bundle pins its copy.
            &crate::fixture::squared(doc, mate, &[]).write(placement),
            Tol::witness(),
        )
        .expect("a mate inserts")
        .doc
    };
    let doc = crate::fixture::stand_again(doc, ids[0]);
    // The second's start cap rests half a unit off the first's: the
    // first's plane slid along its own normal, flipped to face it.
    let a = plane(head(name(ids[0])));
    let standoff = Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(a.clone()),
        along: RoleRead::through(a, PoseRole::Normal),
        by: len(0.5),
    }));
    let doc = add(
        &doc,
        ids[1],
        crate::fixture::mate(plane(head(name(ids[1]))), flip(standoff)),
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    assert!(
        !text.contains("\"class\""),
        "a mate writes no class: {text}"
    );
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the mate round-trips bit for bit");
}

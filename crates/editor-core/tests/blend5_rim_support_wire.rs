//! **The rim-support name vocabulary on the wire** (issue #961).
//!
//! [`editor_core::RimSupport`] spells its two variants `Host` and
//! `Mate`, the roles the annulus surgery resolved (an earlier spelling
//! named the support's KIND). The variant is persisted — a frozen
//! selection is recipe data, and a selection can name a band trimline
//! — so the spelling is file data: the round trip pins it, and a body
//! carrying the retired spelling shows the deserializer's own refusal,
//! naming the variant. (The format carries no schema version — the
//! persist module docs say why — so there is no version pin here and
//! no older golden to refuse.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    EntityKind, Node, PersistError, ProfileDoc, RecipeNodeId, RimSupport, RoleSeg, StableName,
    load, save,
};
use fixture::{desc, insert, len};
use geom_core::Tol;

/// A band trimline's name, on the support role `support`.
fn trim_name(support: RimSupport) -> StableName {
    StableName {
        kind: EntityKind::Edge,
        node: RecipeNodeId(0),
        path: vec![RoleSeg::BandTrim {
            edge: Box::new(StableName {
                kind: EntityKind::Edge,
                node: RecipeNodeId(0),
                path: vec![RoleSeg::OutputBody],
            }),
            support,
        }],
    }
}

/// A document whose RECIPE names both rim-support roles: a fillet
/// whose selection is two band trimlines of an earlier carve. That is
/// where this vocabulary reaches a file — a stable name persists as
/// recipe data (a frozen selection), never as an appearance key, which
/// the appearance door restricts to face and body names.
///
/// Nothing here is evaluated: these rows are about the WIRE.
fn both_roles() -> ProfileDoc {
    let doc = ProfileDoc::empty_derived("blend5-schema-v18", Tol::witness());
    let (doc, profile) = insert(
        doc,
        Node::Profile(desc(
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        )),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(1.0),
        },
    );
    let (doc, _fillet) = insert(
        doc,
        Node::Fillet {
            target: block,
            radius: len(0.05),
            selection: vec![trim_name(RimSupport::Host), trim_name(RimSupport::Mate)],
        },
    );
    doc
}

/// **The new spelling is what the wire carries**, and it round-trips:
/// the two roles are two DISTINCT keys, which is the property a rim
/// between two supports of the same kind would lose under a
/// kind-valued vocabulary.
#[test]
fn both_rim_roles_round_trip() {
    let doc = both_roles();
    let text = save(&doc, &[], Tol::witness()).expect("the document saves");
    assert!(text.contains("\"Host\""), "the host role is on the wire");
    assert!(text.contains("\"Mate\""), "the mate role is on the wire");
    assert!(
        !text.contains("\"Curved\""),
        "the retired kind vocabulary is gone: {text}"
    );
    let back = load(&text, Tol::witness()).expect("its own bytes load").doc;
    let selection = match back.node(RecipeNodeId(2)) {
        Some(Node::Fillet { selection, .. }) => selection.clone(),
        other => panic!("expected the fillet, got {other:?}"),
    };
    assert_eq!(
        selection,
        vec![trim_name(RimSupport::Host), trim_name(RimSupport::Mate)],
        "both roles survive the round trip, and stay two distinct names"
    );
    let again = save(&back, &[], Tol::witness()).expect("the reloaded document saves");
    assert_eq!(text, again, "save . load is not a fixpoint");
}

/// **The vocabulary's own refusal**: a body carrying the retired
/// spelling — what a document written before the re-spell looks like
/// — is unreadable by this build, and the refusal names the variant.
///
/// It dies because an externally-tagged enum rejects a variant name it
/// does not know, unconditionally — `deny_unknown_fields` is inert on
/// unit-only variants and is not what fires here.
#[test]
fn the_retired_spelling_is_unreadable_by_this_build() {
    let text = save(&both_roles(), &[], Tol::witness()).expect("the document saves");
    let retired = text.replace("\"Host\"", "\"Plane\"");
    assert_ne!(retired, text, "the substitution must actually land");
    match load(&retired, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => {
            // The deserializer's message is the ONLY place the name
            // exists (serde exposes no structured accessor), so reading
            // it back is the assertion, not message sniffing; the
            // fuller phrase keeps a short word from matching by accident.
            assert!(
                detail.contains("unknown variant `Plane`"),
                "the refusal names the variant it could not read: {detail}"
            );
        }
        other => panic!("a retired variant name must refuse unreadable, got {other:?}"),
    }
}

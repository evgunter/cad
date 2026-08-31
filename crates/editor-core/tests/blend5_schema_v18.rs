//! **The rim-support name vocabulary on the wire** (issue #961) —
//! schema **v18**, and the gate that refuses everything older.
//!
//! Before v18 [`editor_core::RimSupport`] spelled its two variants
//! `Plane` and `Curved`, a claim about the support's KIND; it now
//! spells them `Host` and `Mate`, the roles the annulus surgery
//! resolved. The variant is persisted — a frozen selection is recipe
//! data, and a selection can name a band trimline — so the spelling is
//! file data and the break runs in BOTH directions.
//!
//! **Which gate fires, in order.** A real v17 file never reaches serde:
//! the VERSION DOOR refuses it first (`SchemaTooOld`, regenerate
//! recourse), and a file from the future is refused there too
//! (`UnknownSchema`) — `blend5_r1_probes` pins that ordering. The
//! vocabulary's own refusal covers only what the header cannot see: a
//! HYBRID, v18 in the header with a retired variant in the body, which
//! `the_retired_spelling_dies_inside_serde` builds and executes. That
//! one dies because an externally-tagged enum rejects an unknown
//! variant name UNCONDITIONALLY — not because of
//! `deny_unknown_fields`, which is inert on unit-only variants.
//!
//! The disposition is the family's: the older file refuses TYPED at
//! the version door with the regenerate recourse, and the migration
//! table stays empty.
//!
//! **The mapping the break does not write.** `Plane` → `Host` and
//! `Curved` → `Mate` is total and meaning-preserving, because the host
//! IS the planar support wherever a rim has one — the old spelling was
//! right on every rim it could name honestly, and only a
//! curved-on-curved rim made it lie. What stops the migration being
//! written is the standing rule (LQ7a), not the mapping. Both halves
//! are executed below: the round trip pins the new spelling, and a
//! hand-built v17 name shows the old one refusing.
//!
//! **Why 18.** Read by eye from main's constant at the final re-merge
//! (`git show origin/main:crates/editor-core/src/persist/mod.rs | grep
//! SCHEMA_VERSION`), because units have repeatedly had a same-number
//! claim merge CLEAN — both sides write the identical line, so git
//! never conflicts.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    EntityKind, Node, PersistError, ProfileDoc, REGENERATE_RECOURSE, RecipeNodeId, RimSupport,
    RoleSeg, SCHEMA_VERSION, StableName, load, save,
};
use fixture::{desc, insert, len};
use geom_core::Tol;

/// The prior live golden, kept as the REFUSAL fixture: a break nobody
/// can demonstrate is a break nobody can trust.
const V17: &str = include_str!("golden/v17_golden.cad");
/// One further back, to show the gate has no notion of "nearly
/// current".
const V16: &str = include_str!("golden/v16_golden.cad");

#[test]
fn schema_version_is_current() {
    assert_eq!(SCHEMA_VERSION, 18);
}

#[test]
fn the_checked_in_older_goldens_are_really_older() {
    assert_eq!(V17.lines().next(), Some("schema: 17"));
    assert_eq!(V16.lines().next(), Some("schema: 16"));
}

/// The break, in the direction that matters: a v17 file refuses TYPED
/// at the version door, naming the version found, the version
/// supported, and the step that does not exist.
#[test]
fn v17_refuses_too_old() {
    match load(V17, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 17);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 17,
                "the 17 -> 18 step is the one that does not exist"
            );
        }
        other => panic!("v17 must refuse SchemaTooOld, got {other:?}"),
    }
}

#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    for (label, bytes) in [("v17", V17), ("v16", V16)] {
        let msg = match load(bytes, Tol::witness()) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("{label} must refuse"),
        };
        assert!(msg.contains(REGENERATE_RECOURSE), "{label}: {msg}");
    }
}

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
fn both_rim_roles_round_trip_at_v18() {
    let doc = both_roles();
    let text = save(&doc, &[], Tol::witness()).expect("the document saves");
    assert_eq!(
        text.lines().next(),
        Some(&format!("schema: {SCHEMA_VERSION}")[..])
    );
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

/// **The vocabulary's own refusal, on the one input the version door
/// cannot catch**: a HYBRID — this build's header over a body carrying
/// the retired spelling, which is what a hand-edited or half-migrated
/// file looks like. A real v17 file never gets this far (the version
/// door refuses it first; `blend5_r1_probes` pins that ordering), so
/// this row is deliberately built rather than taken from a golden.
///
/// It dies because an externally-tagged enum rejects a variant name it
/// does not know, unconditionally — `deny_unknown_fields` is inert on
/// unit-only variants and is not what fires here.
#[test]
fn the_retired_spelling_dies_inside_serde() {
    let text = save(&both_roles(), &[], Tol::witness()).expect("the document saves");
    let v17ish = text.replace("\"Host\"", "\"Plane\"");
    assert_ne!(v17ish, text, "the substitution must actually land");
    match load(&v17ish, Tol::witness()) {
        Err(PersistError::Parse { message, .. }) => {
            assert!(
                message.contains("Plane"),
                "the refusal names the variant it could not read: {message}"
            );
        }
        other => panic!("a retired variant name must refuse in serde, got {other:?}"),
    }
}

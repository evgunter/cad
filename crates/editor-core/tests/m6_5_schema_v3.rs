//! M6-5 — the schema-v3 CLEAN BREAK, pinned.
//!
//! The call (Evan, #217): `Node::Fillet` grew a required `selection`
//! field, and no `migrate` step is written for 2 → 3. A v2 fillet
//! meant "every edge of the target" — a set that depends on an
//! evaluation the FILE does not carry — so there is no honest default
//! to migrate to, and none is invented. A v2 file refuses TYPED with
//! the regenerate recourse, exactly as v1 does; the v2 BYTES stay
//! checked in as the refusal fixture, because a break nobody can
//! demonstrate is a break nobody can trust (the M5 PR 10 precedent,
//! verbatim).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};

/// The pre-break bytes, kept verbatim as the refusal fixture.
const V2: &str = include_str!("golden/v2_golden.cad");
/// The regenerated live golden.
const V3: &str = include_str!("golden/v3_golden.cad");

#[test]
fn schema_version_is_three() {
    assert_eq!(SCHEMA_VERSION, 3);
}

#[test]
fn the_checked_in_v3_file_is_really_v3() {
    assert_eq!(V3.lines().next(), Some("schema: 3"));
}

/// The break, demonstrated: a v2 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the
/// step that does not exist.
#[test]
fn v2_refuses_too_old() {
    match load(V2) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 2);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(missing, 2, "the 2 → 3 step is the one that does not exist");
        }
        other => panic!("v2 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The message names all three facts and ends on the ONE shared
/// recourse carrier, composed exactly once.
#[test]
fn the_too_old_message_names_the_recourse_exactly_once() {
    let msg = PersistError::SchemaTooOld {
        found: 2,
        supported: SCHEMA_VERSION,
        missing: 2,
    }
    .to_string();
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
    assert!(msg.contains("v2"), "{msg}");
    assert!(msg.contains(&format!("v{SCHEMA_VERSION}")), "{msg}");
}

/// The version door still precedes the body parse: a v2 header over
/// nonsense reports the version, never a parse position.
#[test]
fn too_old_beats_a_broken_body() {
    let text = "schema: 2\nnot json at all\n";
    assert!(
        matches!(load(text), Err(PersistError::SchemaTooOld { found: 2, .. })),
        "version door must precede the body parse"
    );
}

/// The v3 golden carries the new field, and carries it CANONICAL: the
/// fixture hands `Node::fillet` its two names out of order and the
/// committed bytes show them sorted. A file whose selection is NOT
/// canonical is corrupt, and refuses at the shared validator rather
/// than being quietly re-sorted (which would move the node's content
/// key behind the caller's back).
#[test]
fn the_v3_golden_pins_a_canonical_selection() {
    assert!(V3.contains("\"selection\""), "the v3 bytes carry the field");
    let first = V3.find("\"segment\": 0").expect("segment 0 present");
    let second = V3.find("\"segment\": 2").expect("segment 2 present");
    let sel = V3.find("\"selection\"").expect("the selection block");
    assert!(sel < first && first < second, "stored in canonical order");

    let corrupt = V3.replace("\"segment\": 0", "\"segment\": 9");
    match load(&corrupt) {
        Err(PersistError::Snapshot(editor_core::SnapshotError::FilletSelectionNotCanonical {
            ..
        })) => {}
        // The golden pins ε = 1e-9, so under another ambient row the ε
        // door could fire first — but it is the LAST door, after the
        // validator, so a non-canonical file cannot reach it.
        other => panic!("a non-canonical selection must refuse typed, got {other:?}"),
    }
}

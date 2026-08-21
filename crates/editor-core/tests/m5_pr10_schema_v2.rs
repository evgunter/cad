//! M5 PR 10 §4 — the schema-v2 CLEAN BREAK, pinned.
//!
//! The call (Evan, #148, 2026-07-31): no `migrate` step is written for
//! 1 → 2; a v1 file refuses TYPED with the regenerate recourse; the
//! repo's own v1 golden was regenerated once, in this PR. The v1 BYTES
//! stay checked in — as the refusal fixture, `tests/golden/v1_golden.cad`
//! — because a break nobody can demonstrate is a break nobody can trust.
//!
//! Version comparison is exact integer arithmetic: there is no in-band
//! twin of this refusal, so the two-tolerance discipline (D4 ¶1
//! addendum) does not apply here. The shared *recourse* discipline
//! still does — the message composes [`REGENERATE_RECOURSE`] exactly
//! once.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};
use geom_core::Tol;

/// The pre-break bytes, kept verbatim as the refusal fixture.
const V1: &str = include_str!("golden/v1_golden.cad");
/// The v2 bytes — the live golden when this file was written, kept as
/// the SECOND refusal fixture since M6-5's 2 → 3 break
/// (`m6_5_schema_v3.rs` pins that one).
const V2: &str = include_str!("golden/v2_golden.cad");

#[test]
fn the_checked_in_v1_file_is_really_v1() {
    assert_eq!(V1.lines().next(), Some("schema: 1"));
    assert_eq!(V2.lines().next(), Some("schema: 2"));
}

/// The break, demonstrated: a v1 file refuses TYPED at the version
/// door — naming the version found, the version supported, and the
/// FIRST step that does not exist (1 → 2, even though 2 → 3 is
/// missing too: the walk reports the earliest gap, which is the one
/// the user's file actually hit). Nothing is best-effort loaded.
#[test]
fn v1_refuses_too_old() {
    match load(V1, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 1);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(missing, 1);
        }
        other => panic!("v1 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The message names all three facts and ends on the ONE shared
/// recourse carrier, composed exactly once.
#[test]
fn the_too_old_message_names_the_recourse_exactly_once() {
    let msg = PersistError::SchemaTooOld {
        found: 1,
        supported: SCHEMA_VERSION,
        missing: 1,
    }
    .to_string();
    assert_eq!(msg.matches(REGENERATE_RECOURSE).count(), 1, "{msg}");
    assert!(msg.contains("v1"), "{msg}");
    assert!(msg.contains(&format!("v{SCHEMA_VERSION}")), "{msg}");
}

/// The door refuses on VERSION, before the body is parsed: a v1 header
/// over a body that is not even JSON still reports the version, never
/// a parse position. (The clean break's whole point — the diagnostics
/// must name the situation the user is actually in.)
#[test]
fn too_old_beats_a_broken_body() {
    let text = "schema: 1\nnot json at all\n";
    assert!(
        matches!(load(text, Tol::witness()), Err(PersistError::SchemaTooOld { found: 1, .. })),
        "version door must precede the body parse"
    );
}

/// Forward-only is unchanged (D6.3): a NEWER schema is still
/// `UnknownSchema`, a different situation with a different recourse.
#[test]
fn newer_schema_is_still_unknown_not_too_old() {
    let text = format!("schema: {}\n{{}}\n", SCHEMA_VERSION + 1);
    match load(&text, Tol::witness()) {
        Err(PersistError::UnknownSchema { found, newest }) => {
            assert_eq!(found, u64::from(SCHEMA_VERSION) + 1);
            assert_eq!(newest, SCHEMA_VERSION);
        }
        other => panic!("a newer schema must refuse UnknownSchema, got {other:?}"),
    }
}

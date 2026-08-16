//! ASM-UPD — the schema-v9 bump, pinned.
//!
//! The call: the EDIT vocabulary grew `DocEdit::UpdateReference`
//! (ASSEMBLY-DESIGN A13, ratified #544). A saved document carries its
//! unreplayed edit log, so an edit arm is wire shape exactly as a node
//! arm is — the same case v8 bumped for, one level over in the
//! vocabulary. The addition is forward-additive, so nothing here
//! claims a v8 file is unreadable in principle; what the gate buys is
//! the other direction, the one that fails badly: a v9 file handed to
//! a v8 reader must refuse at the version door with the regenerate
//! recourse, not reach serde and die on an unknown variant. The v8
//! bytes stay checked in as the refusal fixture, because a break
//! nobody can demonstrate is a break nobody can trust (the M5 PR 10
//! precedent, verbatim).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};

/// The pre-bump bytes, kept verbatim as the refusal fixture (the file
/// `m4_pr6_golden.rs` pinned as LIVE until this bump).
const V8: &str = include_str!("golden/v8_golden.cad");

#[test]
fn schema_version_is_nine() {
    assert_eq!(SCHEMA_VERSION, 9);
}

#[test]
fn the_checked_in_v8_file_is_really_v8() {
    assert_eq!(V8.lines().next(), Some("schema: 8"));
}

/// The break, demonstrated: a v8 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the
/// step that does not exist.
#[test]
fn v8_refuses_too_old() {
    match load(V8) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 8);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(missing, 8, "the 8 → 9 step is the one that does not exist");
        }
        other => panic!("v8 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The recourse is the standing one — regenerate, never a shim.
#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    let msg = match load(V8) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("v8 must refuse"),
    };
    assert!(msg.contains(REGENERATE_RECOURSE), "{msg}");
}

//! LIB-LBRET — the schema-v7 bump, pinned.
//!
//! The call: the chain step vocabulary grew `ProgramStep::AtToward`
//! (PATHS-DESIGN §2b's LB10 route 3, ratified on #386), which is the
//! same shape v2 and v3 bumped for — new vocabulary a previous
//! reader's enum cannot name. The addition is forward-additive, so
//! nothing here claims a v6 file is unreadable in principle; what the
//! gate buys is the OTHER direction, which is the one that fails
//! badly: a v7 file handed to a v6 reader must refuse at the version
//! door with the regenerate recourse, not reach serde and die on an
//! unknown variant. The v6 bytes stay checked in as the refusal
//! fixture, because a break nobody can demonstrate is a break nobody
//! can trust (the M5 PR 10 precedent, verbatim).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};

/// The pre-bump bytes, kept verbatim as the refusal fixture (the file
/// `m4_pr6_golden.rs` pinned as LIVE until this bump).
const V6: &str = include_str!("golden/v6_golden.cad");

#[test]
fn schema_version_is_seven() {
    assert_eq!(SCHEMA_VERSION, 7);
}

#[test]
fn the_checked_in_v6_file_is_really_v6() {
    assert_eq!(V6.lines().next(), Some("schema: 6"));
}

/// The break, demonstrated: a v6 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the
/// step that does not exist.
#[test]
fn v6_refuses_too_old() {
    match load(V6) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 6);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(missing, 6, "the 6 → 7 step is the one that does not exist");
        }
        other => panic!("v6 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The recourse is the standing one — regenerate, never a shim.
#[test]
fn the_refusal_names_the_regenerate_recourse() {
    let err = load(V6).expect_err("v6 refuses");
    assert!(
        err.to_string().contains(REGENERATE_RECOURSE),
        "the refusal must carry the regenerate recourse: {err}"
    );
}

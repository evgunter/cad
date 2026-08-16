//! LIB-RESPELL — the schema-v9 bump, pinned.
//!
//! The call: PATHS-DESIGN §2c (ratified on #419) RE-SPELLED the chain
//! step vocabulary — the compound register's `AtOn`/`AtToward`/
//! `CloseToOn` and the three separate arc-leg forms retired; the one
//! unified arc-spec record and the fused verbs (`FilletArc`/
//! `ArcFillet`/`ArcFilletArc`) arrived. Unlike v8's forward-additive
//! growth this is a break in BOTH directions: a v8 file's retired
//! variants are unknown to today's reader exactly as today's fused
//! variants are unknown to a v8 reader — so the gate refuses TYPED
//! with the regenerate recourse (the v3 precedent), the migration
//! table stays empty, and the v8 bytes stay checked in as the refusal
//! fixture (a break nobody can demonstrate is a break nobody can
//! trust).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};

/// The pre-bump bytes, kept verbatim as the refusal fixture (the file
/// `m4_pr6_golden.rs` pinned as LIVE until this bump).
const V8: &str = include_str!("golden/v8_golden.cad");

#[test]
fn schema_version_is_nine() {
    assert_eq!(SCHEMA_VERSION, 10);
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
fn the_refusal_names_the_regenerate_recourse() {
    let err = load(V8).expect_err("v8 refuses");
    assert!(
        err.to_string().contains(REGENERATE_RECOURSE),
        "the refusal must carry the regenerate recourse: {err}"
    );
}

/// The other direction: a file claiming a FUTURE version refuses as
/// unknown — the newest this build supports is named.
#[test]
fn a_future_version_refuses_unknown() {
    let future = V8.replacen("schema: 8", "schema: 11", 1);
    match load(&future) {
        Err(PersistError::UnknownSchema { found, newest }) => {
            assert_eq!(found, 11);
            assert_eq!(newest, SCHEMA_VERSION);
        }
        other => panic!("a v11 file must refuse UnknownSchema, got {other:?}"),
    }
}

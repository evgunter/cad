//! LIB-LBRET — the schema-v8 bump, pinned.
//!
//! The call: the chain step vocabulary grew `ProgramStep::AtToward`
//! (PATHS-DESIGN §2b's LB10 route 3, ratified on #386), which is the
//! same shape v2 and v3 bumped for — new vocabulary a previous
//! reader's enum cannot name. The addition is forward-additive, so
//! nothing here claims a v7 file is unreadable in principle; what the
//! gate buys is the OTHER direction, which is the one that fails
//! badly: a v8 file handed to a v7 reader must refuse at the version
//! door with the regenerate recourse, not reach serde and die on an
//! unknown variant. The v7 bytes stay checked in as the refusal
//! fixture, because a break nobody can demonstrate is a break nobody
//! can trust (the M5 PR 10 precedent, verbatim).
//!
//! **Why 8 and not 7**: ASM-2A (#414) and this unit each concluded 7
//! was theirs, each having re-merged main before the other's bump
//! landed. ASM-2A merged first, so v7 is InstantiatePart and this
//! vocabulary growth takes 8. Two meanings never share one version —
//! and the merge that could have hidden it resolved the one-line
//! constant CLEANLY, which is why the ledger says so out loud.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};
use geom_core::Tol;

/// The pre-bump bytes, kept verbatim as the refusal fixture (the file
/// `m4_pr6_golden.rs` pinned as LIVE until this bump).
const V7: &str = include_str!("golden/v7_golden.cad");

#[test]
fn schema_version_is_current() {
    // Moved four times since this row was written (LIB-RESPELL's v9
    // §2c re-spell, ASM-UPD's v10 `UpdateReference` arm, M9-1's v11
    // declaration class, LIB-PLACEDUNION's v12 group boolean, then
    // ASM-R2a's v13 `Node::Mate` arm) — the convention is that a bump
    // updates every pin it invalidates, so the number stays exact
    // here. Named for the PROPERTY rather than the number, since the
    // number is exactly what keeps moving.
    assert_eq!(SCHEMA_VERSION, 14);
}

#[test]
fn the_checked_in_v7_file_is_really_v7() {
    assert_eq!(V7.lines().next(), Some("schema: 7"));
}

/// The break, demonstrated: a v7 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the
/// step that does not exist.
#[test]
fn v7_refuses_too_old() {
    match load(V7, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 7);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(missing, 7, "the 7 → 8 step is the one that does not exist");
        }
        other => panic!("v7 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The recourse is the standing one — regenerate, never a shim.
#[test]
fn the_refusal_names_the_regenerate_recourse() {
    let err = load(V7, Tol::witness()).expect_err("v7 refuses");
    assert!(
        err.to_string().contains(REGENERATE_RECOURSE),
        "the refusal must carry the regenerate recourse: {err}"
    );
}

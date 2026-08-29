//! ASM-UPD — the schema-v10 bump, pinned.
//!
//! The call: the EDIT vocabulary grew `DocEdit::UpdateReference`
//! (ASSEMBLY-DESIGN A13, ratified #544). A saved document carries its
//! unreplayed edit log, so an edit arm is wire shape exactly as a node
//! arm is — the same case v8 bumped for, one level over in the
//! vocabulary. The addition is forward-additive, so nothing here
//! claims a v9 file is unreadable in principle; what the gate buys is
//! the other direction, the one that fails badly: a v10 file handed to
//! a v9 reader must refuse at the version door with the regenerate
//! recourse, not reach serde and die on an unknown variant. The v9
//! bytes stay checked in as the refusal fixture, because a break
//! nobody can demonstrate is a break nobody can trust (the M5 PR 10
//! precedent, verbatim).
//!
//! **Why 10 and not 9**: LIB-RESPELL (#531) and this unit each
//! concluded 9 was theirs, each having re-merged main before the
//! other's bump landed — the v7/v8 collision, repeated. RESPELL
//! merged first, so v9 is the §2c re-spell and the `UpdateReference`
//! arm takes 10. Two meanings never share one version; the
//! SCHEMA_VERSION ledger carries the full entry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{PersistError, REGENERATE_RECOURSE, SCHEMA_VERSION, load};
use geom_core::Tol;

/// The pre-bump bytes, kept verbatim as the refusal fixture (the file
/// `m4_pr6_golden.rs` pinned as LIVE until this bump).
const V9: &str = include_str!("golden/v9_golden.cad");

#[test]
fn schema_version_is_current() {
    // Named for the PROPERTY, not the number (the `lbret_schema_v8`
    // precedent): ASM-UPD's own bump was v10; M9-1 took v11,
    // LIB-PLACEDUNION v12, ASM-R2a v13, ASM-R2b v14 and M10-1 v15, and
    // the number is what keeps moving.
    assert_eq!(SCHEMA_VERSION, 16);
}

#[test]
fn the_checked_in_v9_file_is_really_v9() {
    assert_eq!(V9.lines().next(), Some("schema: 9"));
}

/// The break, demonstrated: a v9 file refuses TYPED at the version
/// door, naming the version found, the version supported, and the
/// step that does not exist.
#[test]
fn v9_refuses_too_old() {
    match load(V9, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 9);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(missing, 9, "the 9 → 10 step is the one that does not exist");
        }
        other => panic!("v9 must refuse SchemaTooOld, got {other:?}"),
    }
}

/// The recourse is the standing one — regenerate, never a shim.
#[test]
fn the_refusal_carries_the_regenerate_recourse() {
    let msg = match load(V9, Tol::witness()) {
        Err(e) => e.to_string(),
        Ok(_) => panic!("v9 must refuse"),
    };
    assert!(msg.contains(REGENERATE_RECOURSE), "{msg}");
}

//! **The v16 → v17 demonstration row.** A file saved under the
//! PREVIOUS schema refuses typed under this one, and the refusal
//! carries the regenerate recourse — the whole content of a break
//! with no migration machinery (LQ7a), asserted rather than assumed.
//!
//! # Why this row exists at all
//!
//! `v16_golden.cad` is included by no other test: once its version is
//! no longer current, every suite that reads a golden reads the new
//! one, and the old bytes become a file nothing executes. A bump's
//! entire user-visible promise — "your old file will not silently
//! misload, it will refuse and tell you what to do" — then rests on
//! code no row runs. The G16 bump shipped without this row and a
//! reviewer wrote it; this one ships with it.
//!
//! # Red-capable, and by what
//!
//! Not a tautology over `SCHEMA_VERSION`: the row pins the LITERAL 17
//! and the literal 16 against the golden's own header line, so it goes
//! red three separate ways — if the constant is rolled back, if the
//! v16 bytes are regenerated under a newer number (which would erase
//! the very artifact under test), or if `load` starts accepting an old
//! file instead of refusing it. Mutation-checked: reverting
//! `SCHEMA_VERSION` to 16 turns the `SchemaTooOld` match into a
//! `ToleranceConflict`/success and this row fails; deleting the
//! `regenerate` clause from the refusal's `Display` fails the last
//! assertion alone.
#![allow(clippy::unwrap_used, clippy::panic)]
use editor_core::{PersistError, load, persist::SCHEMA_VERSION};
use geom_core::Tol;

const V16: &str = include_str!("golden/v16_golden.cad");

#[test]
fn v16_refuses_too_old_under_v17() {
    assert_eq!(
        V16.lines().next(),
        Some("schema: 16"),
        "the file under test must be the PREVIOUS version's bytes"
    );
    assert_eq!(SCHEMA_VERSION, 17);
    match load(V16, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 16);
            assert_eq!(supported, 17);
            assert_eq!(
                missing, 16,
                "the 16 -> 17 step is the one that does not exist"
            );
            let msg = PersistError::SchemaTooOld {
                found,
                supported,
                missing,
            }
            .to_string();
            println!("v16 -> v17 refusal text: {msg}");
            assert!(
                msg.contains("regenerate"),
                "must carry the regenerate recourse: {msg}"
            );
        }
        other => panic!("a v16 file must refuse SchemaTooOld under v17, got {other:?}"),
    }
}

/// The other direction, which the row above cannot see: this build's
/// OWN golden is at the current number, so a bump that forgot to
/// regenerate the golden — leaving v17's fixture carrying v16 bytes —
/// fails here rather than passing as "the old file refuses, good".
#[test]
fn the_current_golden_is_at_the_current_version() {
    let current = include_str!("golden/v17_golden.cad");
    assert_eq!(current.lines().next(), Some("schema: 17"));
}

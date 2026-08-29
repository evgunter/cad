//! lib-g16-r1 review probe: does a v15 file refuse TYPED under the
//! CURRENT schema, and does the refusal carry the regenerate
//! recourse? Nothing else in the tree exercises this — `v15_golden.cad`
//! is included by no other test.
//!
//! Carried across M10-2's v16 -> v17 bump. The probe used to pin
//! `supported == 16` by literal, which made a probe about the version
//! DOOR fail on the next bump for a reason that has nothing to do with
//! what it asks. `supported` is `SCHEMA_VERSION` by definition, so it
//! is asserted as that; `missing` stays a literal 15, because "the
//! 15 -> 16 step does not exist" is a claim about the empty migration
//! table and does not move when the head version does.
#![allow(clippy::unwrap_used, clippy::panic)]
use editor_core::{PersistError, load, persist::SCHEMA_VERSION};
use geom_core::Tol;

const V15: &str = include_str!("golden/v15_golden.cad");

#[test]
fn v15_refuses_too_old_under_the_current_schema() {
    assert_eq!(V15.lines().next(), Some("schema: 15"));
    match load(V15, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 15);
            assert_eq!(supported, SCHEMA_VERSION);
            assert_eq!(
                missing, 15,
                "the 15 -> 16 step is the one that does not exist"
            );
            let msg = PersistError::SchemaTooOld {
                found,
                supported,
                missing,
            }
            .to_string();
            println!("R1 PROBE v15 refusal text: {msg}");
            assert!(
                msg.contains("regenerate"),
                "must carry the regenerate recourse: {msg}"
            );
        }
        other => panic!("v15 must refuse SchemaTooOld, got {other:?}"),
    }
}

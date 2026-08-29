//! lib-g16-r1 review probe: does a v15 file refuse TYPED under v16,
//! and does the refusal carry the regenerate recourse? Nothing in the
//! tree exercises this — `v15_golden.cad` is included by no test.
#![allow(clippy::unwrap_used, clippy::panic)]
use editor_core::{PersistError, load, persist::SCHEMA_VERSION};
use geom_core::Tol;

const V15: &str = include_str!("golden/v15_golden.cad");

#[test]
fn v15_refuses_too_old_under_v16() {
    assert_eq!(V15.lines().next(), Some("schema: 15"));
    assert_eq!(SCHEMA_VERSION, 17);
    match load(V15, Tol::witness()) {
        Err(PersistError::SchemaTooOld {
            found,
            supported,
            missing,
        }) => {
            assert_eq!(found, 15);
            assert_eq!(supported, 16);
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

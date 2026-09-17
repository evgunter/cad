//! **The sweep half of the same two questions the profile probes ask.**
//!
//! `recourse_roster` measures routed-or-listed by looking for the gap
//! sentence, so a `fillet3_*` name rewired to another arm's recourse is
//! still "routed". The first row pins the pairing, name by name.
//!
//! The second row pins the reader's precondition: its `decide*` scan
//! skips a call whose token suffix is not alphanumeric, so
//! `decide::<f64>("…")` is read as neither a name nor an indirect site.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Indeterminate, MarginDiag, Tol};
use sweep::blend::{
    BlendError, BlendSite, FILLET3_CHAIN_RECOURSE, FILLET3_CLEARANCE_RECOURSE,
    FILLET3_CORNER_RECOURSE, FILLET3_RADIUS_RECOURSE, FILLET3_RING_RECOURSE,
    FILLET3_SPINE_KIND_RECOURSE, FILLET3_SPINE_RECOURSE, FILLET3_TANGENTIAL_RECOURSE,
};

/// Each routed name with the recourse constant its arm owns.
fn table() -> Vec<(&'static str, &'static str)> {
    vec![
        ("fillet3_radius_headroom", FILLET3_RADIUS_RECOURSE),
        ("fillet3_face_clearance", FILLET3_CLEARANCE_RECOURSE),
        ("fillet3_spine_regularity", FILLET3_SPINE_RECOURSE),
        ("fillet3_chain_g1", FILLET3_CHAIN_RECOURSE),
        ("fillet3_chain_arm", FILLET3_CHAIN_RECOURSE),
        ("fillet3_convexity_sign", FILLET3_TANGENTIAL_RECOURSE),
        ("fillet3_ring_clearance", FILLET3_RING_RECOURSE),
        ("fillet3_support_coaxiality", FILLET3_SPINE_KIND_RECOURSE),
        ("fillet3_corner_independence", FILLET3_CORNER_RECOURSE),
        ("fillet3_cap_transverse", FILLET3_CORNER_RECOURSE),
    ]
}

fn rendered(name: &'static str) -> String {
    let band = Band::linear(Tol::witness()).expect("the run's band forms");
    BlendError::Escalated {
        site: BlendSite::Chain,
        source: Indeterminate {
            margin: MarginDiag::Value((band.zero() + band.escalate()) / 2.0),
            band,
            predicate: Some(name),
        },
    }
    .to_string()
}

/// **A routed name renders the recourse its own arm owns, and no
/// other's.**
#[test]
fn every_routed_name_renders_the_recourse_its_own_arm_owns() {
    let table = table();
    for (name, recourse) in &table {
        let text = rendered(name);
        assert!(
            text.contains(recourse),
            "`{name}` should carry its own arm's recourse; it renders: {text}"
        );
        for (_, other) in &table {
            assert!(
                std::ptr::eq(*other, *recourse) || !text.contains(other),
                "`{name}` carries another arm's recourse: {text}"
            );
        }
    }
}

/// **No `decide` call in this crate's `src` is spelled with a
/// turbofish** — the spelling the roster's reader skips without
/// recording it as a site it could not read.
#[test]
fn no_decide_call_in_src_is_spelled_with_a_turbofish() {
    let src = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut found = Vec::new();
    for path in test_utils::source::rust_sources(&src) {
        let text = std::fs::read_to_string(&path).expect("a readable source file");
        let code = test_utils::source::code_and_literals(&text);
        if code.contains("decide::<") || code.contains("decide ::<") {
            found.push(path.display().to_string());
        }
    }
    assert!(
        found.is_empty(),
        "a `decide` call is spelled with a turbofish, which the recourse roster's reader \
         skips without recording: {found:?}"
    );
}

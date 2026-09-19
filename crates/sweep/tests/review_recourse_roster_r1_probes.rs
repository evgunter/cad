//! **The sweep half of the question the profile probe asks.**
//!
//! `recourse_roster` measures routed-or-listed by looking for the gap
//! sentence, so a `fillet3_*` name rewired to another arm's recourse is
//! still "routed". The row here pins the pairing, name by name.

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

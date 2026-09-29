//! **An edge resting in a face is cut where the face's boundary
//! crosses it**, at the public door.
//!
//! The lap seat is a post under a shelf whose `y = 0.30` underside edge
//! runs across the post's cap, entering and leaving it through the
//! cap's side edges away from any vertex; the shelf edge's own midpoint
//! lies outside the cap. Each resting stretch is found, reported where
//! it lies, and — declared — backed at the region-confined strength.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

/// The cap `[0.1, 0.3] × [0.2, 0.42]` at `z = 0.5` under the shelf
/// `[0, 0.9] × [0, 0.30] × [0.5, 0.54]`: the cap face and the shelf's
/// underside.
fn lap_seat() -> (Body<f64>, FaceKey, FaceKey) {
    let tol = Tol::witness();
    let post: common::Prism<f64> = common::prism_z(
        &[(0.1, 0.2), (0.3, 0.2), (0.3, 0.42), (0.1, 0.42)],
        0.0,
        0.5,
        tol,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        tol,
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, tol).unwrap();
    (body, post.top_face, keys.face(shelf.bottom_face).unwrap())
}

fn errors(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

/// Every edge-on-face overlap reported on `face`, by witness.
fn overlaps_on(errors: &[ValidationError], face: FaceKey) -> Vec<String> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UndeclaredContact {
                contact: CensusContact::EdgeFaceOverlap { face: f, .. },
                witness,
            } if *f == face => Some(witness.clone()),
            _ => None,
        })
        .collect()
}

/// Bare, the seat reports each resting stretch at a point inside the
/// face it rests in: the shelf edge on the cap between the cap's two
/// sides (`x = 0.2`), the cap's lower edge on the underside, and each
/// cap side edge on the shelf's underside
/// from its lower corner up to the shelf edge (`y = 0.25`) — never a
/// stretch's far part outside the face, and none missed because the
/// edge's own midpoint lies outside.
#[test]
fn the_bare_lap_seat_reports_each_stretch_inside_its_face() {
    let (body, cap, underside) = lap_seat();
    let found = errors(&body, &ContactRecords::default());
    assert_eq!(
        overlaps_on(&found, cap),
        vec!["(0.19999999999999996, 0.3, 0.5)".to_owned()],
        "{found:?}"
    );
    let mut on_underside = overlaps_on(&found, underside);
    on_underside.sort();
    assert_eq!(
        on_underside,
        vec![
            "(0.1, 0.25, 0.5)".to_owned(),
            "(0.2, 0.2, 0.5)".to_owned(),
            "(0.3, 0.25, 0.5)".to_owned(),
        ],
        "{found:?}"
    );
}

/// Declared, the seat certifies outright: the crossings are backed by
/// the crossing rung, and every overlap's bounds lie inside the
/// declared pair's verified interface.
#[test]
fn the_declared_lap_seat_certifies() {
    let (body, cap, underside) = lap_seat();
    let records = ContactRecords {
        patches: vec![PatchContact {
            face_a: cap,
            face_b: underside,
        }],
        ..ContactRecords::default()
    };
    let found = errors(&body, &records);
    assert!(found.is_empty(), "{found:?}");
}

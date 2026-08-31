//! **The edge-on-face bound rung at a vertex-on-edge bound** — the
//! interior arm of `ef_bound_backed`.
//!
//! An edge-on-face overlap whose cell bound falls on the edge's
//! INTERIOR carries no vertex of that edge: the bound is a
//! vertex-on-edge event, a boundary vertex of the face resting on the
//! edge. That event is face-backable, so the overlap's bound reads the
//! same rung one incidence step out — the declared face pair holding
//! the vertex on one boundary and naming a face the edge bounds.
//!
//! The seat is the one a user draws: a post whose cap overhangs the
//! shelf's side edge, declared `post_cap ~ shelf_underside`. The
//! shelf's boundary edge dives through the cap's interior between two
//! cap vertices resting on that edge's interior.
//!
//! The rung consults DECLARATIONS only: the bare seat and a seat
//! declared between the wrong faces both stay as loud as before.
//!
//! Straddling crossings (`EdgeEdgeCross`) are a different question and
//! are untouched here — the fence row pins that configuration's
//! findings character for character.
//!
//! ε posture: every coincidence is a shared f64 literal (the cap's
//! vertices sit on `y = 0.30` exactly, the two cap faces on `z = 0.5`
//! exactly), every separation is ≥ 0.05 — a twentieth of a metre, far
//! outside any band under test.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

/// The overhang seat (issue 973's section (a), the review probe's
/// fixture): the cap crosses the shelf's `y = 0.30` side edge at the
/// vertices H and B and touches it tangentially at T, so the shelf's
/// boundary edge lies in the cap's plane and dives through the cap's
/// interior over the cell `H..B`. Both bounds of that cell are on the
/// shelf edge's interior — no vertex of the edge is there.
fn overhang_seat() -> (Body<f64>, FaceKey, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[
            (0.20, 0.20), // A (below the line)
            (0.40, 0.30), // B (crossing vertex, on the shelf edge)
            (0.60, 0.42), // C1 (above)
            (0.70, 0.30), // T (tangent vertex, on the shelf edge)
            (0.80, 0.42), // C2 (above)
            (0.85, 0.50), // G2 (above, clears the spikes)
            (0.15, 0.50), // G (above)
            (0.25, 0.30), // H (crossing vertex, on the shelf edge)
        ],
        0.0,
        0.5,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    let shelf_top = keys.face(shelf.top_face).unwrap();
    (body, post.top_face, shelf_bottom, shelf_top)
}

/// Issue 973's section (b) configuration, verbatim: a rectangular cap
/// `[0.30, 0.60] x [0.20, 0.42]` straddling the shelf's `y = 0.30`
/// boundary edge, declared. Its two cap side edges cross that edge
/// properly; nothing here reaches the bound rung.
fn straddle_seat() -> (Body<f64>, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    (body, post.top_face, shelf_bottom)
}

fn declared(a: FaceKey, b: FaceKey) -> ContactRecords {
    ContactRecords {
        patches: vec![PatchContact {
            face_a: a,
            face_b: b,
        }],
        ..ContactRecords::default()
    }
}

fn errors(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

fn undeclared(errors: &[ValidationError]) -> Vec<CensusContact> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UndeclaredContact { contact, .. } => Some(*contact),
            _ => None,
        })
        .collect()
}

fn count(cs: &[CensusContact], f: impl Fn(&CensusContact) -> bool) -> usize {
    cs.iter().filter(|c| f(c)).count()
}

/// The unit's claim: nothing in the declared overhang seat is a hard
/// `UndeclaredContact` any more. The edge-on-face overlap's two bounds
/// (H and B, on the shelf edge's interior) are vertex-on-edge events
/// the declared pair holds, so the overlap is backed like any other.
///
/// What the seat is left with is a DIFFERENT door's refusal: the
/// declared patch's own region-overlap confirm refuses
/// `TouchingBoundary`, because the cap's boundary meets the shelf
/// underside's boundary at vertices rather than crossing it. That is a
/// `CensusUnsupported` on the pair's face — `Attribution::Declined`,
/// the `Uncertified` frontier — not the `Unattributed` hard error this
/// unit retires, and closing it is the chart predicate's question, not
/// the census rung's.
#[test]
fn the_declared_overhang_seat_keeps_no_hard_finding() {
    let (body, post_top, shelf_bottom, _) = overhang_seat();
    let found = errors(&body, &declared(post_top, shelf_bottom));
    assert!(
        undeclared(&found).is_empty(),
        "every finding of the declared overhang seat is answered: {found:?}"
    );
    assert!(
        matches!(
            found.as_slice(),
            [ValidationError::CensusUnsupported {
                entity: topo::EntityId::Face(f),
            }] if *f == post_top
        ),
        "and the residue is the declared pair's own region door: {found:?}"
    );
}

/// The rung reads declarations, never the geometry's own agreement
/// with itself: undeclared, the seat is exactly as loud as it was.
#[test]
fn the_bare_overhang_seat_still_refuses() {
    let (body, _, _, _) = overhang_seat();
    let found = undeclared(&errors(&body, &ContactRecords::default()));
    assert_eq!(
        count(&found, |c| matches!(c, CensusContact::VertexOnEdge { .. })),
        3,
        "H, B crossing and T tangent, on the shelf edge's interior: {found:?}"
    );
    assert_eq!(
        count(&found, |c| matches!(
            c,
            CensusContact::EdgeFaceOverlap { .. }
        )),
        3,
        "the shelf edge's dive through the cap interior, and the two \
         cap edges resting inside the shelf's underside: {found:?}"
    );
}

/// A declaration between the WRONG faces (the cap against the shelf's
/// TOP) holds neither bound: the overlap stays a hard finding, and the
/// census is exactly as loud as with no declaration at all.
#[test]
fn a_wrong_pair_backs_no_ef_bound() {
    let (body, post_top, _, shelf_top) = overhang_seat();
    let bare = undeclared(&errors(&body, &ContactRecords::default()));
    let wrong = undeclared(&errors(&body, &declared(post_top, shelf_top)));
    assert_eq!(
        count(&wrong, |c| matches!(
            c,
            CensusContact::EdgeFaceOverlap { .. }
        )),
        3,
        "{wrong:?}"
    );
    assert_eq!(
        format!("{bare:?}"),
        format!("{wrong:?}"),
        "a wrong pair changes nothing"
    );
}

/// **The fence.** Issue 973's section (b) — the straddling cap's two
/// proper `EdgeEdgeCross` findings — is a separate design question and
/// is NOT this unit's. The whole error list is pinned character for
/// character, witnesses included, so a leak into (b) cannot pass
/// silently: not a finding gained, not a finding lost, not a witness
/// moved.
#[test]
fn the_straddle_crossings_are_untouched() {
    let (body, post_top, shelf_bottom) = straddle_seat();
    let found = errors(&body, &declared(post_top, shelf_bottom));
    assert_eq!(
        format!("{found:?}"),
        "[UndeclaredContact { contact: EdgeEdgeCross { a: EdgeKey(10v1), \
         b: EdgeKey(15v1) }, witness: \"Point3 { x: 0.6, y: 0.3, z: 0.5 }\" }, \
         UndeclaredContact { contact: EdgeEdgeCross { a: EdgeKey(12v1), \
         b: EdgeKey(15v1) }, witness: \"Point3 { x: 0.3, y: 0.3, z: 0.5 }\" }]",
        "issue 973 part (b) stays exactly as it was"
    );
}

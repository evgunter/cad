//! **Reviewer probes for the at-rest face rung** (PR #969, lane r1).
//!
//! Three attacks the shipped rows do not carry:
//!
//! 1. `the_lemma_probe_*`: a POSITIVE-overlap declared pair whose own
//!    seat also induces a vertex-on-edge event genuinely outside the
//!    closure of the positive-area overlap (a tangent spike, no proper
//!    crossings). The two faces' closures meet there although their
//!    overlap does not reach it, so the pair does not answer for it:
//!    the touch is decided as a pair of its own (`[assertions]`).
//! 2. The same fixture reaches `ef_bound_backed`'s interior arm: the
//!    shelf's boundary edge dives through the cap's interior between
//!    two cap vertices resting on the edge's interior. A flush seat
//!    cannot reach that arm; this declared coplanar rest seat (an
//!    overhanging post) does, and the arm reads the vertex-on-edge
//!    rung at each bound, so the overlap is backed with the events.
//! 3. `a_wrong_pair_backs_nothing`: the rung consults exactly the
//!    declared pair's own incidence — a declaration between the wrong
//!    faces (post cap x shelf TOP) backs none of the seat's events.
//!
//! ε posture: all coincidences are shared f64 literals, and the
//! separations the CENSUS rungs turn on are ≥ a twentieth of a metre.
//! The governing margin of the overhang seat is smaller than that and
//! is not one of them: `pm_census_ee_parallel` reads 8.944e-3 on the
//! near-parallel cap and shelf edges, which lands in band at
//! `CAD_TOLERANCE_EPS=1e-3` and escalates honestly there. That is why
//! `the_lemma_probe_declared`'s whole-list assertion below is a
//! GATED-BAND row — green at default, 1e-6 and 1e-12, red at 1e-3,
//! which the matrix does not run.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, FaceKey, ValidationError, VertexKey};

/// Post under the shelf's SIDE edge (y = 0.30), overhanging it: the
/// cap has positive-area overlap with the shelf's underside on the
/// left (triangle A-B-H, x <= 0.40), pokes above the edge line
/// elsewhere, and dips a single tangent vertex T = (0.70, 0.30) onto
/// the edge's interior — 0.30 in x away from the overlap's closure.
/// The boundary meets the line y = 0.30 only at vertices (H and B
/// crossing, T tangent): no proper edge-edge crossings anywhere.
/// Returns the body, the cap, the shelf's underside and top, and T.
fn overhang_seat() -> (Body<f64>, FaceKey, FaceKey, FaceKey, VertexKey) {
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
        Tol::witness(),
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        Tol::witness(),
    );
    let tangent = post.top[3];
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    let shelf_top = keys.face(shelf.top_face).unwrap();
    (body, post.top_face, shelf_bottom, shelf_top, tangent)
}

/// The unit's own flush seat, rebuilt verbatim, plus the shelf's TOP
/// face key for the wrong-pair probe.
fn flush_seat() -> (Body<f64>, FaceKey, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.09), (0.12, 0.09), (0.12, 0.21), (0.0, 0.21)],
        0.0,
        0.5,
        Tol::witness(),
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        Tol::witness(),
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    let shelf_top = keys.face(shelf.top_face).unwrap();
    (body, post.top_face, shelf_bottom, shelf_top)
}

fn declared(body: &Body<f64>, a: FaceKey, b: FaceKey) -> ContactRecords {
    topo::ContactRecords::from_iter(vec![topo::Cited::new(
        topo::Contact::new(body, topo::Cell::Face(a), topo::Cell::Face(b)).unwrap(),
        topo::Cites::decided(0),
    )])
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
            ValidationError::UnrecordedContact { contact, .. } => Some(*contact),
            _ => None,
        })
        .collect()
}

fn count(cs: &[CensusContact], f: impl Fn(&CensusContact) -> bool) -> usize {
    cs.iter().filter(|c| f(c)).count()
}

/// Undeclared baseline: the overhang seat's own census is one face
/// pair, the cap on the underside, and T on the shelf edge, outside the
/// pair's overlap. Read where no pair can be confirmed
/// (the structural twin, which holds no region door), three cap vertices
/// rest on the shelf edge's interior (H, B, T) and the shelf edge dives
/// through the cap's interior.
#[test]
fn the_lemma_probe_undeclared_baseline() {
    let (body, _, _, _, tangent) = overhang_seat();
    let certified = undeclared(&errors(&body, &ContactRecords::default()));
    assert_eq!(
        count(&certified, |c| matches!(
            c,
            CensusContact::ConformalPatch { .. }
        )),
        1,
        "one face pair: {certified:?}"
    );
    assert_eq!(
        only_vertex_on_edge(&certified),
        tangent,
        "and T, its own pair: {certified:?}"
    );
    let found = undeclared(
        &topo::validate_pseudomanifold_structural(
            &body,
            &ContactRecords::default(),
            Tol::witness(),
        )
        .unwrap_err(),
    );
    assert_eq!(
        count(&found, |c| matches!(c, CensusContact::VertexOnEdge { .. })),
        3,
        "H, B crossing and T tangent, all on the shelf edge's interior: {found:?}"
    );
    assert!(
        count(&found, |c| matches!(
            c,
            CensusContact::EdgeFaceOverlap { .. }
        )) > 0,
        "the shelf edge's dive through the cap interior is an \
         edge-on-face overlap: {found:?}"
    );
}

/// Declared, the pair answers for the two vertices that rest on the
/// shelf edge where it bounds the overlap (H and B), and for the edge's
/// run through the cap between them, but not for the tangent T: the two
/// faces' closures meet there, 0.30 away from the closure of their
/// overlap, so the touch is decided as a pair of its own and, unrecorded,
/// is the one finding.
///
/// The edge-on-face overlap between the same two entities is backed:
/// its two bounds fall on the shelf edge's INTERIOR, where cap vertices
/// rest, so each bound is a vertex-on-edge event and reads that lane's
/// rung — `ef_bound_backed`'s interior arm — inside the overlap's
/// closure.
#[test]
fn the_lemma_probe_declared() {
    let (body, post_top, shelf_bottom, _, tangent) = overhang_seat();
    let all = errors(&body, &declared(&body, post_top, shelf_bottom));
    let found = undeclared(&all);
    assert_eq!(
        only_vertex_on_edge(&found),
        tangent,
        "the tangent outside the overlap is its own pair, the crossing \
         vertices the pair's: {found:?}"
    );
    assert_eq!(
        count(&found, |c| matches!(
            c,
            CensusContact::EdgeFaceOverlap { .. }
        )),
        0,
        "and the ef interior arm reads the same rung at each bound: \
         {found:?}"
    );
    // Gated-band row (see the header): the whole-list form is red at
    // 1e-3, where four honest `pm_census_ee_parallel` escalations join
    // the list. The two census assertions above hold at every band.
    assert_eq!(
        all.len(),
        1,
        "and door 2's rescue rung finds the overlap, so T is all: {all:?}"
    );
}

/// The one vertex-on-edge finding's vertex.
fn only_vertex_on_edge(found: &[CensusContact]) -> VertexKey {
    let ve: Vec<_> = found
        .iter()
        .filter_map(|c| match c {
            CensusContact::VertexOnEdge { vertex, .. } => Some(*vertex),
            _ => None,
        })
        .collect();
    let [v] = ve[..] else {
        panic!("one vertex on an edge: {found:?}");
    };
    v
}

/// A pair naming the wrong shelf face (its TOP) answers for none of the
/// flush seat's events: the census is exactly as loud as with no
/// declaration at all, one face pair the cap on the underside, and the
/// wrong pair is the confirm pass's own finding.
#[test]
fn a_wrong_pair_backs_nothing() {
    let (body, post_top, shelf_bottom, shelf_top) = flush_seat();
    let bare = undeclared(&errors(&body, &ContactRecords::default()));
    let wrong = undeclared(&errors(&body, &declared(&body, post_top, shelf_top)));
    let [CensusContact::ConformalPatch { finding }] = wrong[..] else {
        panic!("one face pair: {wrong:?}");
    };
    let mut pair = [finding.pair.a, finding.pair.b];
    pair.sort();
    let mut seat = [post_top, shelf_bottom];
    seat.sort();
    assert_eq!(pair, seat, "the pair is the cap on the underside");
    assert_eq!(bare, wrong, "a wrong pair changes nothing");
}

/// The unit's flush seat with the graft order SWAPPED (post grafted
/// into the shelf's arena), so the shelf's boundary edge is `ea` and
/// the cap's vertices resolve on `eb` — the `(None, Some)` arm of
/// `ee_bound_backed`, which no committed row reaches.
#[test]
fn the_swapped_graft_order_flush_seat_exercises_the_other_arm() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.09), (0.12, 0.09), (0.12, 0.21), (0.0, 0.21)],
        0.0,
        0.5,
        Tol::witness(),
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        Tol::witness(),
    );
    let mut body = shelf.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &post.body).unwrap();
    let post_top = keys.face(post.top_face).unwrap();
    let found = undeclared(&errors(
        &body,
        &declared(&body, post_top, shelf.bottom_face),
    ));
    assert!(
        found.is_empty(),
        "backing must not depend on which body was grafted first: {found:?}"
    );
}

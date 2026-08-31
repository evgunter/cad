//! **Reviewer probes for the at-rest face rung** (PR #969, lane r1).
//!
//! Three attacks the shipped rows do not carry:
//!
//! 1. `the_lemma_probe_*`: a POSITIVE-overlap declared pair whose own
//!    seat also induces a vertex-on-edge event genuinely outside the
//!    closure of the positive-area overlap (a tangent spike, no proper
//!    crossings). If the rung backs it, the "positive overlap confines
//!    every induced v-on-e to the interface closure" lemma is false —
//!    the region-unconfined strength is exercised by a positive-area
//!    pair, not only by the zero-area fixture the strength row uses.
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
//! ε posture: as the unit's own suite — all coincidences are shared
//! f64 literals, all separations ≥ a twentieth of a metre.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

/// Post under the shelf's SIDE edge (y = 0.30), overhanging it: the
/// cap has positive-area overlap with the shelf's underside on the
/// left (triangle A-B-H, x <= 0.40), pokes above the edge line
/// elsewhere, and dips a single tangent vertex T = (0.70, 0.30) onto
/// the edge's interior — 0.30 in x away from the overlap's closure.
/// The boundary meets the line y = 0.30 only at vertices (H and B
/// crossing, T tangent): no proper edge-edge crossings anywhere.
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

/// The unit's own flush seat, rebuilt verbatim, plus the shelf's TOP
/// face key for the wrong-pair probe.
fn flush_seat() -> (Body<f64>, FaceKey, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.09), (0.12, 0.09), (0.12, 0.21), (0.0, 0.21)],
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

/// Undeclared baseline: the overhang seat's own census. Three cap
/// vertices rest on the shelf edge's interior (H, B, T).
#[test]
fn the_lemma_probe_undeclared_baseline() {
    let (body, _, _, _) = overhang_seat();
    let found = undeclared(&errors(&body, &ContactRecords::default()));
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

/// The attack: declared, every vertex-on-edge event is backed —
/// INCLUDING the tangent T, which lies 0.30 away from the closure of
/// the pair's positive-area overlap. A positive-overlap pair backs an
/// event outside its interface closure; the confinement lemma does not
/// hold, and region-unconfinement is not a zero-area-only phenomenon.
///
/// AND: the edge-on-face overlap between the same two entities is
/// backed too. Its two bounds fall on the shelf edge's INTERIOR, where
/// cap vertices rest, so each bound is a vertex-on-edge event and reads
/// that lane's rung — `ef_bound_backed`'s interior arm. No finding of
/// this seat is unattributed any more.
///
/// This seat still does not CERTIFY, through a door that is not the
/// census's rungs at all: the declared patch's region-overlap confirm
/// runs `interior_witness`'s rescue rung on a `Definite` door-1 verdict
/// and declines because that rung's fixed candidate schedule misses
/// this overlap (measured) — `CensusUnsupported`,
/// `Attribution::Declined`, the `Uncertified` frontier, where before it
/// was an `Unattributed` hard error. That is the fixture's outcome, not
/// the class's: a same-class seat whose overlap the schedule lands
/// certifies outright (`r1_mate4a_probes`). The schedule's aim is issue
/// 1435. This probe pins the rung.
#[test]
fn the_lemma_probe_declared() {
    let (body, post_top, shelf_bottom, _) = overhang_seat();
    let found = undeclared(&errors(&body, &declared(post_top, shelf_bottom)));
    assert_eq!(
        count(&found, |c| matches!(c, CensusContact::VertexOnEdge { .. })),
        0,
        "the rung backs all three, the out-of-interface tangent \
         included: {found:?}"
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
}

/// The rung consults the DECLARED pair's incidence, nothing else: a
/// pair naming the wrong shelf face (its TOP) backs none of the flush
/// seat's events — the census is exactly as loud as with no
/// declaration at all.
#[test]
fn a_wrong_pair_backs_nothing() {
    let (body, post_top, _, shelf_top) = flush_seat();
    let bare = undeclared(&errors(&body, &ContactRecords::default()));
    let wrong = undeclared(&errors(&body, &declared(post_top, shelf_top)));
    assert_eq!(
        count(&wrong, |c| matches!(c, CensusContact::VertexOnEdge { .. })),
        2,
        "{wrong:?}"
    );
    assert_eq!(
        count(&wrong, |c| matches!(
            c,
            CensusContact::EdgeEdgeOverlap { .. }
        )),
        1,
        "{wrong:?}"
    );
    assert_eq!(
        bare.len(),
        wrong.len(),
        "a wrong pair changes nothing: {wrong:?}"
    );
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
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let mut body = shelf.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &post.body, Tol::witness()).unwrap();
    let post_top = keys.face(post.top_face).unwrap();
    let found = undeclared(&errors(&body, &declared(post_top, shelf.bottom_face)));
    assert!(
        found.is_empty(),
        "backing must not depend on which body was grafted first: {found:?}"
    );
}

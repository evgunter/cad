//! **The at-rest face rung for vertex-on-edge events** (#943).
//!
//! A seat whose two faces share a boundary induces vertex-on-edge and
//! edge-edge-overlap events by construction. In the boolean lane those
//! never survive to records — reduction splits the other edge first —
//! but at rest nothing refines, so they arrive raw at the certifier.
//! The declared FACE pair the seat's mate mints is what backs them:
//! the rung holds the vertex on one side of the interface and the edge
//! on the other, inside the closure of the pair's overlap, exactly as it
//! holds a coincident vertex pair there (census module docs, D3/D4).
//!
//! The rows here pin both directions of that, plus the rung's
//! STRENGTH: a pair answers only once both doors confirm it, so an
//! event on a pair whose regions share no area is the census's own.
//!
//! **ε posture**: every row asserts a hard outcome because no margin
//! in these fixtures rides a band at any ε the matrix runs. The
//! coincidences are exact — the seat's shared coordinates are the same
//! f64 literals on both bodies, so the residuals are zeros, not small
//! numbers — and every separation the sweeps decide is a tenth of a
//! metre against a 1e-6 ceiling. A row here going indeterminate would
//! be news about the predicates, not about the tolerance.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, FaceKey, ValidationError};

/// The seat, as one body: a post standing under a shelf, both built as
/// prisms and grafted into one arena. Returns the body and the two
/// faces the seat's mate would name — the post's top cap and the
/// shelf's underside.
fn seat(post: &[(f64, f64)], shelf: &[(f64, f64)]) -> (Body<f64>, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(post, 0.0, 0.5, Tol::witness());
    let shelf: common::Prism<f64> = common::prism_z(shelf, 0.5, 0.54, Tol::witness());
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    (body, post.top_face, shelf_bottom)
}

/// The #943 repro: the post's top cap seated FLUSH with the shelf's
/// end, so the cap's end edge lies inside the shelf underside's own
/// boundary edge. The post is inset in y (it is only the x = 0 end
/// that is flush).
///
/// One physical scene, stated independently at three layers: here at
/// census granularity, at the assembly gate (`flush_seat` in
/// `editor-core/tests/asm_r2b_assembly.rs`), and as a user's document
/// (the bench-stand dimensions in `demos/tour/src/assembly.rs`). The
/// numbers agree by construction and are deliberately NOT shared — a
/// kernel row reading a demo's dimensions would make the demo
/// load-bearing for the kernel, and each layer's fixture has to be
/// readable as the thing that layer is about.
fn flush_seat() -> (Body<f64>, FaceKey, FaceKey) {
    seat(
        &[(0.0, 0.09), (0.12, 0.09), (0.12, 0.21), (0.0, 0.21)],
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
    )
}

/// The declaration a `Rest` mate mints for a seat: one face-pair
/// record, said once.
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

/// Every unrecorded-contact finding, by its census contact.
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

/// The INSET seat: the post wholly within the shelf's underside, its
/// cap sharing no boundary with it — how legs are actually set under a
/// top, and the shape the demo had to author while gap 2 was open.
/// Its trims overlap in plain AREA, so the region walk decides it
/// outright and the interior-witness rung is never consulted; that is
/// what makes it the control for the flush row beside it.
fn inset_seat() -> (Body<f64>, FaceKey, FaceKey) {
    seat(
        &[(0.10, 0.09), (0.22, 0.09), (0.22, 0.21), (0.10, 0.21)],
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
    )
}

/// The line-contact seat: the post stands just OFF the shelf's side,
/// its top cap coplanar with the shelf's underside and sharing exactly
/// one boundary line with it. The two faces touch along a segment and
/// their regions overlap in no area at all — so every event the
/// configuration induces lies outside the declared pair's own
/// interface.
fn line_contact_seat() -> (Body<f64>, FaceKey, FaceKey) {
    seat(
        &[(0.0, 0.30), (0.12, 0.30), (0.12, 0.42), (0.0, 0.42)],
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
    )
}

/// The residue a declared face pair leaves when its two trims share no
/// AREA: the confirm door refuses the pair typed. A boundary touch is
/// not an overlap the area machinery decides in either direction, and
/// the door says so rather than certifying or refuting.
///
/// Since #1063 this is no longer what a CHART-divergent pair earns —
/// the world carrier answers those (`census_g2_carrier.rs`) — so a row
/// reaching it now says something about the GEOMETRY.
fn pair_declined(errors: &[ValidationError], face_a: FaceKey, face_b: FaceKey) {
    assert_eq!(
        errors,
        [ValidationError::CensusUnsupported {
            subject: topo::CensusSubject::FacePair(face_a, face_b),
            // The CAUSE is pinned, not `..`-ed past. "Declined" is
            // the same word for a dozen unrelated refusals, and the
            // one this seat earns is the geometric one the docstring
            // above claims: the trims touch, so the area is not
            // decidable at this ε. A schedule exhaustion here would
            // mean the row had stopped testing what it says it tests.
            cause: topo::CensusUnsupportedCause::ChartRegion(
                topo::ChartRegionError::TouchingBoundary
            ),
        }],
        "beside the seat's events, the declared pair's own confirmation \
         is the residue: the pair is declined, not refuted"
    );
}

/// INVARIANT: a declared face pair backs every event its own seat
/// induces, vertex-on-edge and the edge-edge overlap those events
/// bound included. Nothing about the seat is a hard finding.
///
/// **And since #1063 the pair itself CERTIFIES**, so the flush seat —
/// the obvious way to draw a post under a shelf — leaves no residue at
/// all. The two descriptions share no `SurfaceKey`; their shared
/// world carrier is the chart, the region walk refuses the shared
/// trim edge as a touching boundary, and the interior-witness
/// rung proves the disc. This is #943's whole repro, certified.
#[test]
fn a_declared_flush_seat_leaves_no_unrecorded_contact() {
    let (body, post_top, shelf_bottom) = flush_seat();
    let errors = errors(&body, &declared(&body, post_top, shelf_bottom));
    assert!(
        undeclared(&errors).is_empty(),
        "the seat's own induced events are backed by the pair that \
         declared it: {errors:?}"
    );
    assert_eq!(
        errors,
        [],
        "the declared flush seat certifies whole — no residue: {errors:?}"
    );
}

/// INVARIANT (argument-order symmetry at the CENSUS, not just at the
/// predicate): the seat's declaration says the same thing whichever
/// face it names first. `world_carrier` takes A's frame as the pair's
/// representative, and the lemma at that function is what makes the
/// choice invisible in the answer.
#[test]
fn the_flush_seat_certifies_in_both_argument_orders() {
    let (body, post_top, shelf_bottom) = flush_seat();
    assert_eq!(
        errors(&body, &declared(&body, post_top, shelf_bottom)),
        [],
        "cap first",
    );
    assert_eq!(
        errors(&body, &declared(&body, shelf_bottom, post_top)),
        [],
        "underside first",
    );
}

/// INVARIANT (acceptance 4: certifying flush must not certify
/// everything): an INSET seat — the post wholly inside the shelf's
/// underside, sharing no boundary — certifies too, and by the ordinary
/// region walk rather than the witness rung. The two seats agreeing is
/// what retires the demo's inset.
#[test]
fn the_inset_seat_certifies_too() {
    let (body, post_top, shelf_bottom) = inset_seat();
    assert_eq!(
        errors(&body, &declared(&body, post_top, shelf_bottom)),
        [],
        "an inset seat's trims overlap in plain area",
    );
}

/// INVARIANT (the scan-to-bless ban, F1): the census reads
/// DECLARATIONS, never the geometry's agreement with itself. The SAME
/// configuration with nothing declared is the hard error: the seat is
/// one area contact, the cap on the underside, so it is one face-pair
/// finding, and it answers for the touches of the corners and edges
/// bounding it, all in its overlap's closure (`[assertions]`).
#[test]
fn the_same_flush_seat_undeclared_is_the_hard_error() {
    let (body, post_top, shelf_bottom) = flush_seat();
    let found = undeclared(&errors(&body, &ContactRecords::default()));
    let [CensusContact::ConformalPatch { finding }] = found[..] else {
        panic!("one face pair: {found:?}");
    };
    let mut pair = [finding.pair.a, finding.pair.b];
    pair.sort();
    let mut seat = [post_top, shelf_bottom];
    seat.sort();
    assert_eq!(pair, seat, "the pair is the cap on the underside");
}

/// INVARIANT — the rung's STRENGTH, stated as a row rather than left
/// to be read off the code: a declared pair answers only for its
/// verified interface. Here the pair's two faces are coplanar and touch
/// along one line, so their regions overlap in NO area, both doors do
/// not confirm the pair, and it answers for nothing: the events on the
/// shared line stand as they do undeclared, and the pair itself is
/// declined by the confirm pass.
#[test]
fn the_rung_backs_no_event_outside_the_declared_pairs_overlap_region() {
    let (body, post_top, shelf_bottom) = line_contact_seat();
    let errors = errors(&body, &declared(&body, post_top, shelf_bottom));
    let found = undeclared(&errors);
    assert_eq!(
        count(&found, |c| matches!(c, CensusContact::VertexOnEdge { .. })),
        1,
        "{errors:?}"
    );
    assert_eq!(
        count(&found, |c| matches!(
            c,
            CensusContact::EdgeEdgeOverlap { .. }
        )),
        1,
        "{errors:?}"
    );
    assert_eq!(
        count(&found, |c| matches!(c, CensusContact::VertexVertex { .. })),
        1,
        "{errors:?}"
    );
    let rest: Vec<ValidationError> = errors
        .iter()
        .filter(|e| !matches!(e, ValidationError::UnrecordedContact { .. }))
        .cloned()
        .collect();
    pair_declined(&rest, post_top, shelf_bottom);
}

/// The same configuration undeclared: one cap corner on the shelf
/// edge's interior, its coincident twin at the shelf's corner, and the
/// overlap they bound — all hard findings without a declaration.
#[test]
fn the_line_contact_seat_undeclared_is_the_hard_error() {
    let (body, _, _) = line_contact_seat();
    let found = undeclared(&errors(&body, &ContactRecords::default()));
    assert_eq!(
        count(&found, |c| matches!(c, CensusContact::VertexOnEdge { .. })),
        1,
        "{found:?}"
    );
    assert_eq!(
        count(&found, |c| matches!(
            c,
            CensusContact::EdgeEdgeOverlap { .. }
        )),
        1,
        "{found:?}"
    );
    assert_eq!(
        count(&found, |c| matches!(c, CensusContact::VertexVertex { .. })),
        1,
        "{found:?}"
    );
}

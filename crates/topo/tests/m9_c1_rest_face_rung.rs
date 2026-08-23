//! **The at-rest face rung for vertex-on-edge events** (#943).
//!
//! A seat whose two faces share a boundary induces vertex-on-edge and
//! edge-edge-overlap events by construction. In the boolean lane those
//! never survive to records — reduction splits the other edge first —
//! but at rest nothing refines, so they arrive raw at the certifier.
//! The declared FACE pair the seat's mate mints is what backs them:
//! the rung holds the vertex on one side of the interface and the edge
//! on the other, exactly as `vv_face_backed` holds a coincident vertex
//! pair (census module docs, D3/D4).
//!
//! The rows here pin both directions of that, plus the rung's
//! STRENGTH: it is structural-incidence and region-unconfined, like
//! the two rungs it is built from.
//!
//! **ε posture**: every row asserts a hard outcome because no margin
//! in these fixtures rides a band at any ε the matrix runs. The
//! coincidences are exact — the seat's shared coordinates are the same
//! f64 literals on both bodies, so the residuals are zeros, not small
//! numbers — and every separation the sweeps decide is a tenth of a
//! metre against a 1e-6 ceiling. A row here going indeterminate would
//! be news about the predicates, not about the tolerance.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

/// The seat, as one body: a post standing under a shelf, both built as
/// prisms and grafted into one arena. Returns the body and the two
/// faces the seat's mate would name — the post's top cap and the
/// shelf's underside.
fn seat(post: &[(f64, f64)], shelf: &[(f64, f64)]) -> (Body<f64>, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(post, 0.0, 0.5);
    let shelf: common::Prism<f64> = common::prism_z(shelf, 0.5, 0.54);
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
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

/// Every undeclared-contact finding, by its census contact.
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

/// The residue a declared face pair leaves at this door whatever the
/// geometry: the pair's own confirmation, which the cross-key chart
/// door declines rather than answering. Read in ONE place, so the day
/// that door answers, every row here moves together.
fn pair_declined(errors: &[ValidationError], face_a: FaceKey) {
    assert_eq!(
        errors,
        [ValidationError::CensusUnsupported {
            entity: topo::EntityId::Face(face_a)
        }],
        "the declared pair's own confirmation is the whole residue: \
         every event the seat induces is backed, and the pair itself \
         is declined, not refuted"
    );
}

/// INVARIANT: a declared face pair backs every event its own seat
/// induces, vertex-on-edge and the edge-edge overlap those events
/// bound included. Nothing about the seat is a hard finding.
#[test]
fn a_declared_flush_seat_leaves_no_undeclared_contact() {
    let (body, post_top, shelf_bottom) = flush_seat();
    let errors = errors(&body, &declared(post_top, shelf_bottom));
    assert!(
        undeclared(&errors).is_empty(),
        "the seat's own induced events are backed by the pair that \
         declared it: {errors:?}"
    );
    pair_declined(&errors, post_top);
}

/// INVARIANT (the scan-to-bless ban, F1): the rung consults
/// DECLARATIONS, never the geometry's agreement with itself. The SAME
/// configuration with nothing declared is the hard error — every event
/// of it, the two vertex-on-edge findings and the overlap they bound.
#[test]
fn the_same_flush_seat_undeclared_is_the_hard_error() {
    let (body, _, _) = flush_seat();
    let found = undeclared(&errors(&body, &ContactRecords::default()));
    assert_eq!(
        count(&found, |c| matches!(c, CensusContact::VertexOnEdge { .. })),
        2,
        "both cap corners rest on the shelf edge's interior: {found:?}"
    );
    assert_eq!(
        count(&found, |c| matches!(
            c,
            CensusContact::EdgeEdgeOverlap { .. }
        )),
        1,
        "the cap's end edge overlaps the shelf's: {found:?}"
    );
    assert!(
        count(&found, |c| matches!(c, CensusContact::VertexOnFace { .. })) > 0,
        "and the rest of the seat is undeclared too: {found:?}"
    );
}

/// INVARIANT — the rung's STRENGTH, stated as a row rather than left
/// to be read off the code: backing is STRUCTURAL INCIDENCE and is not
/// confined to the declared pair's overlap region. Here the pair's two
/// faces are coplanar and touch along one line, so their regions
/// overlap in NO area — and the vertex-on-edge event on their shared
/// boundary is backed all the same, because the pair holds the vertex
/// on one boundary and the edge on the other.
///
/// This is the same strength the two older face rungs have (neither
/// asks where on the pair an event lies), and it is deliberate:
/// whether the declared pair is itself sound is the confirm pass's
/// question, in its own direction of the certification diff, and this
/// pair is not certified here either.
#[test]
fn the_rung_backs_an_event_outside_the_declared_pairs_overlap_region() {
    let (body, post_top, shelf_bottom) = line_contact_seat();
    let errors = errors(&body, &declared(post_top, shelf_bottom));
    assert!(
        undeclared(&errors).is_empty(),
        "the pair holds the entities, so it backs the event — even \
         where the pair's own regions share no area: {errors:?}"
    );
    pair_declined(&errors, post_top);
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

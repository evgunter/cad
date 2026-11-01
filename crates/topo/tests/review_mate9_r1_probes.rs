//! MATE-9 review, lane R1 — adversarial probes against the
//! `EdgeEdgeCross` backing rung (`census.rs::ee_cross_backed`,
//! written at frozen head b873d7831; both rows INVERTED by the fix
//! pass to pin the screens they forced, red-first order
//! adopt → fix → invert).
//!
//! Probe 1 attacked the ratified fork itself: the ruling
//! (`crates/topo/README.md`) legalizes an IN-CONTACT-PLANE
//! crossing and calls a TRANSVERSE crossing interpenetration, and the
//! spec's rung backs "a crossing of two coplanar boundary edges" —
//! but at the frozen head nothing checked that the crossing edges lie
//! in the pair's shared carrier: a vertical edge DIVING through the
//! contact plane, crossing a boundary edge of the seat exactly at
//! plane level, was backed by the seat's own (genuinely verified,
//! opposite-sides) declaration even though the diving edge's lower
//! half sits inside the other solid's material. The fix pass's edge
//! screen (`pair_holds_edges`) refuses it; the row now pins the
//! refusal.
//!
//! Probe 2 attacked the imported sense algebra's preconditions:
//! `classify_material_pairing`'s contract is "two faces at an
//! on-locus point where the tangent planes already classified
//! Smooth" (one shared carrier). At the frozen head the census's only
//! screen was point-on-both-planes, so two faces whose carriers meet
//! at 45° through the crossing point answered a DEFINITE Aligned and
//! the refusal named "side verdict: same-side … of the shared
//! carrier" where no shared carrier exists — a false named verdict on
//! the channel the PR designates as C6's future admission evidence.
//! The edge screen refuses the skew pair before any side question is
//! posed (two crossing lines span ONE plane, so no skew pair can hold
//! them both), and the Smooth-precondition gate stands behind it for
//! whatever the screens ever let through; the row now pins the
//! silence.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, CrossingSideVerdict, FaceKey, ValidationError};

fn declared(body: &Body<f64>, pairs: &[(FaceKey, FaceKey)]) -> ContactRecords {
    topo::test_support::records(
        body,
        pairs
            .iter()
            .map(|&(a, b)| (topo::Cell::Face(a), topo::Cell::Face(b))),
    )
}

fn errors(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

/// [`errors`] where no face pair can be confirmed: the structural twin
/// holds no region door, so no pair is confirmed and every crossing on
/// a seat's boundary is read through the crossing rung's screens, which
/// the certified census does not reach once it confirms the seat's own
/// pair, which answers for the touches of its boundary inside its
/// overlap's closure (`[assertions]`).
fn structural(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold_structural(body, records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

/// The `EdgeEdgeCross` refusals of a list, as their witness strings
/// and side verdicts.
fn crossing_witnesses(errors: &[ValidationError]) -> Vec<(String, Option<CrossingSideVerdict>)> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UnrecordedContact {
                contact: CensusContact::EdgeEdgeCross { side, .. },
                witness,
                ..
            } => Some((witness.clone(), *side)),
            _ => None,
        })
        .collect()
}

/// **Probe 1 — a transverse (carrier-diving) crossing is NOT backed**
/// (INVERTED; at the frozen head it was).
///
/// The MATE-9 straddle fixture (post `[0.30,0.60] x [0.20,0.42]`,
/// z 0..0.5; shelf `[0,0.9] x [0,0.30]`, z 0.5..0.54; seat plane
/// z = 0.5), plus a triangular spike over `(0.45, 0.30)`,
/// `(0.52, 0.37)`, `(0.38, 0.37)`, z 0.40..0.60. The spike
/// interpenetrates the post (e.g. (0.45, 0.35, 0.45) is interior to
/// both), and its vertical corner edge at (0.45, 0.30) DIVES through
/// the seat plane, crossing the shelf-bottom boundary edge (y = 0.30,
/// z = 0.5) exactly at plane level — a transverse crossing,
/// interpenetration evidence by the ratified fork, and the lower half
/// of the diving edge runs inside the post's material. The spike's
/// faces are slanted, so none of them lies on the shelf's side as a
/// face pair whose boundary the census would ask nothing of.
///
/// Bare, the census reports them. Declared with ONLY the genuine seat
/// pair (post_top, shelf_bottom) — a pair verified at rest whose
/// region holds the two dive points ((0.45, 0.30) and (0.55, 0.30)
/// are interior to the post cap and on the shelf's closed rim) — the
/// frozen-head rung backed both diving crossings: `pair_holds_point`
/// checked only the POINT, the side test read only the DECLARED
/// pair's senses (opposed — it is a real rest seat), and nothing
/// asked whether the crossing edges lie in the carrier. INVERTED by
/// the fix pass: `pair_holds_edges` decides the diving edges' far
/// endpoints against the seat carrier (residuals ∓0.1 m, definitely
/// off), the seat pair answers for nothing about them, and both
/// transverse crossings STAY hard — the ruling's fork enforced.
#[test]
fn r1_a_diving_edge_crossing_is_not_backed_by_the_seat_pair() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
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
    let spike: common::Prism<f64> = common::prism_z(
        &[(0.45, 0.30), (0.52, 0.37), (0.38, 0.37)],
        0.40,
        0.60,
        Tol::witness(),
    );
    let post_top = post.top_face;
    let mut body = post.body;
    let skeys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let shelf_bottom = skeys.face(shelf.bottom_face).unwrap();
    topo::graft_disjoint_all_keyed(&mut body, &spike.body).unwrap();

    // The witness is a coordinate triple, and a crossing witness may
    // carry a trailing side-verdict clause after it — so both probes
    // match INSIDE the triple rather than anchoring on its end. The
    // `)` in the z probe is what separates z = 0.5 from z = 0.54.
    let dive_witness = |w: &String| w.contains("(0.45,");
    let at_seat_level = |w: &String| w.contains(", 0.5)");

    // Bare: the diving crossing at seat level is reported
    // (alongside the cap's two in-plane crossings and the z = 0.54
    // rim crossings, which are not at issue here).
    let bare = crossing_witnesses(&errors(&body, &ContactRecords::default()));
    let bare_dives: Vec<_> = bare
        .iter()
        .filter(|(w, _)| dive_witness(w) && at_seat_level(w))
        .collect();
    assert_eq!(
        bare_dives.len(),
        1,
        "bare, the diving crossing at z = 0.5 is a hard finding: {bare:?}"
    );

    // Declared with the genuine seat pair only: the diving crossing
    // SURVIVES, plain — the edge screen refuses the pair (the diving
    // edges' far endpoints are definitely off the seat carrier), so
    // no side question is posed and nothing is backed. Inverted from
    // the frozen-head measurement, where both vanished.
    let found = crossing_witnesses(&errors(
        &body,
        &declared(&body, &[(post_top, shelf_bottom)]),
    ));
    let surviving_dives: Vec<_> = found
        .iter()
        .filter(|(w, _)| dive_witness(w) && at_seat_level(w))
        .collect();
    assert_eq!(
        surviving_dives.len(),
        1,
        "the seat declaration backs nothing about the spike's \
         transverse diving crossing: {found:?}"
    );
    assert!(
        surviving_dives.iter().all(|(_, side)| side.is_none()),
        "and names no verdict for them — the pair never spoke: \
         {surviving_dives:?}"
    );
}

/// **Probe 2 — a 45-degree skew pair names NO side verdict**
/// (INVERTED; at the frozen head it named a false SAME-SIDE). The
/// straddle fixture plus a wedge prism whose side face lies in the
/// plane `x = y` (carrier through the crossing point (0.30, 0.30,
/// 0.5) at 45 degrees to the post's `x = 0.30` side face). Declaring
/// (post_side_x030, wedge_45_face) — two faces with NO shared
/// carrier — the pair holds the crossing POINT (on both planes, in
/// both closed trims), and at the frozen head
/// `classify_material_pairing` answered a definite verdict from
/// normals at 45 degrees (dot = ±cos 45, far outside every band),
/// naming "same-side … of the shared carrier" on the C6
/// admission-evidence channel — the sense algebra consumed outside
/// its stated Smooth-tangent-plane precondition. The fix pass's edge
/// screen refuses the pair first (the cap's crossing edge is
/// definitely off the `x = y` carrier: residuals up to 0.12 m), so
/// no side question is posed and the crossings stay plain; the
/// Smooth gate stands behind the screen for anything that ever
/// passes it.
#[test]
fn r1_a_skew_pair_names_no_side_verdict_without_a_shared_carrier() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
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
    // A triangular wedge whose segment 0 runs (0.40,0.40)→(0.20,0.20):
    // carrier x = y, outward normal (-1, 1, 0)/√2.
    let wedge: common::Prism<f64> = common::prism_z(
        &[(0.40, 0.40), (0.20, 0.20), (0.40, 0.10)],
        0.40,
        0.60,
        Tol::witness(),
    );
    // post.side_faces[3] spans (0.30, 0.42) → (0.30, 0.20): the plane
    // x = 0.30, outward normal (-1, 0, 0).
    let post_side_x030 = post.side_faces[3];
    let wedge_45 = wedge.side_faces[0];
    let mut body = post.body;
    let _ = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let wkeys = topo::graft_disjoint_all_keyed(&mut body, &wedge.body).unwrap();
    let wedge_45 = wkeys.face(wedge_45).unwrap();

    let found = structural(&body, &declared(&body, &[(post_side_x030, wedge_45)]));
    let witnesses = crossing_witnesses(&found);
    assert_eq!(
        witnesses.len(),
        2,
        "the straddle crossings stay hard findings: {found:?}"
    );
    assert!(
        witnesses.iter().all(|(_, side)| side.is_none()),
        "and the skew (45-degree) declared pair names NOTHING for \
         them — the edge screen refuses a pair whose carrier does not \
         hold the crossing edges, so the side question is never posed \
         outside its precondition: {witnesses:?}"
    );
    assert!(
        !found
            .iter()
            .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
        "no escalation either — screened out is not asked-and-failed: \
         {found:?}"
    );
}

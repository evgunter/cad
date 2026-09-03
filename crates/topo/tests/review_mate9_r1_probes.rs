//! MATE-9 review, lane R1 — adversarial probes against the
//! `EdgeEdgeCross` backing rung (`census.rs::ee_cross_backed`,
//! written at frozen head b873d7831; both rows INVERTED by the fix
//! pass to pin the screens they forced, red-first order
//! adopt → fix → invert).
//!
//! Probe 1 attacked the ratified fork itself: the ruling
//! (`docs/MATE-4B-CROSSING-DESIGN.md`) legalizes an IN-CONTACT-PLANE
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
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

fn declared(pairs: &[(FaceKey, FaceKey)]) -> ContactRecords {
    ContactRecords {
        patches: pairs
            .iter()
            .map(|&(a, b)| PatchContact {
                face_a: a,
                face_b: b,
            })
            .collect(),
        ..ContactRecords::default()
    }
}

fn errors(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

/// The `EdgeEdgeCross` refusals of a list, as their witness strings.
fn crossing_witnesses(errors: &[ValidationError]) -> Vec<String> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UndeclaredContact {
                contact: CensusContact::EdgeEdgeCross { .. },
                witness,
            } => Some(witness.clone()),
            _ => None,
        })
        .collect()
}

/// **Probe 1 — a transverse (carrier-diving) crossing is NOT backed**
/// (INVERTED; at the frozen head it was).
///
/// The MATE-9 straddle fixture (post `[0.30,0.60] x [0.20,0.42]`,
/// z 0..0.5; shelf `[0,0.9] x [0,0.30]`, z 0.5..0.54; seat plane
/// z = 0.5), plus a spike `[0.45,0.55] x [0.30,0.38]`, z 0.40..0.60.
/// The spike interpenetrates the post (e.g. (0.5, 0.34, 0.45) is
/// interior to both), and its two vertical corner edges at
/// (0.45, 0.30) and (0.55, 0.30) DIVE through the seat plane, each
/// crossing the shelf-bottom boundary edge (y = 0.30, z = 0.5)
/// exactly at plane level — transverse crossings, interpenetration
/// evidence by the ratified fork, and the lower half of each diving
/// edge runs inside the post's material.
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
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let spike: common::Prism<f64> = common::prism_z(
        &[(0.45, 0.30), (0.55, 0.30), (0.55, 0.38), (0.45, 0.38)],
        0.40,
        0.60,
    );
    let post_top = post.top_face;
    let mut body = post.body;
    let skeys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = skeys.face(shelf.bottom_face).unwrap();
    topo::graft_disjoint_all_keyed(&mut body, &spike.body, Tol::witness()).unwrap();

    let dive_witness = |w: &String| w.contains("x: 0.45,") || w.contains("x: 0.55,");
    let at_seat_level = |w: &String| w.contains("z: 0.5 ");

    // Bare: the two diving crossings at seat level are reported
    // (alongside the cap's two in-plane crossings and the z = 0.54
    // rim crossings, which are not at issue here).
    let bare = crossing_witnesses(&errors(&body, &ContactRecords::default()));
    let bare_dives: Vec<_> = bare
        .iter()
        .filter(|w| dive_witness(w) && at_seat_level(w))
        .collect();
    assert_eq!(
        bare_dives.len(),
        2,
        "bare, both diving crossings at z = 0.5 are hard findings: {bare:?}"
    );

    // Declared with the genuine seat pair only: the diving crossings
    // SURVIVE, plain — the edge screen refuses the pair (the diving
    // edges' far endpoints are definitely off the seat carrier), so
    // no side question is posed and nothing is backed. Inverted from
    // the frozen-head measurement, where both vanished.
    let found = crossing_witnesses(&errors(&body, &declared(&[(post_top, shelf_bottom)])));
    let surviving_dives: Vec<_> = found
        .iter()
        .filter(|w| dive_witness(w) && at_seat_level(w))
        .collect();
    assert_eq!(
        surviving_dives.len(),
        2,
        "the seat declaration backs nothing about the spike's \
         transverse diving crossings: {found:?}"
    );
    assert!(
        surviving_dives.iter().all(|w| !w.contains("side verdict")),
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
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    // A triangular wedge whose segment 0 runs (0.40,0.40)→(0.20,0.20):
    // carrier x = y, outward normal (-1, 1, 0)/√2.
    let wedge: common::Prism<f64> =
        common::prism_z(&[(0.40, 0.40), (0.20, 0.20), (0.40, 0.10)], 0.40, 0.60);
    // post.side_faces[3] spans (0.30, 0.42) → (0.30, 0.20): the plane
    // x = 0.30, outward normal (-1, 0, 0).
    let post_side_x030 = post.side_faces[3];
    let wedge_45 = wedge.side_faces[0];
    let mut body = post.body;
    let _ = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let wkeys = topo::graft_disjoint_all_keyed(&mut body, &wedge.body, Tol::witness()).unwrap();
    let wedge_45 = wkeys.face(wedge_45).unwrap();

    let found = errors(&body, &declared(&[(post_side_x030, wedge_45)]));
    let witnesses = crossing_witnesses(&found);
    assert_eq!(
        witnesses.len(),
        2,
        "the straddle crossings stay hard findings: {found:?}"
    );
    assert!(
        witnesses.iter().all(|w| !w.contains("side verdict")),
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

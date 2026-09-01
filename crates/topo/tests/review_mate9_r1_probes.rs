//! MATE-9 review, lane R1 — adversarial probes against the
//! `EdgeEdgeCross` backing rung (`census.rs::ee_cross_backed`,
//! frozen head b873d7831).
//!
//! Probe 1 attacks the ratified fork itself: the ruling
//! (`docs/MATE-4B-CROSSING-DESIGN.md`) legalizes an IN-CONTACT-PLANE
//! crossing and calls a TRANSVERSE crossing interpenetration, and the
//! spec's rung backs "a crossing of two coplanar boundary edges" —
//! but the implementation never checks that the crossing edges lie in
//! the pair's shared carrier (nor that they relate to the pair at
//! all). A vertical edge DIVING through the contact plane, crossing a
//! boundary edge of the seat exactly at plane level, is backed by the
//! seat's own (genuinely verified, opposite-sides) declaration even
//! though the diving edge's lower half sits inside the other solid's
//! material — a transverse crossing certified.
//!
//! Probe 2 attacks the imported sense algebra's preconditions:
//! `classify_material_pairing`'s contract is "two faces at an
//! on-locus point where the tangent planes already classified
//! Smooth" (one shared carrier). The census's only screen is
//! point-on-both-planes, so two faces whose carriers meet at 45°
//! through the crossing point answer a DEFINITE Aligned, and the
//! refusal names "side verdict: same-side … of the shared carrier"
//! where no shared carrier exists — a false named verdict on the
//! channel the PR designates as C6's future admission evidence.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

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

/// **Probe 1 — a transverse (carrier-diving) crossing is BACKED.**
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
/// rung backs both diving crossings: `pair_holds_point` checks only
/// the POINT, the side test reads only the DECLARED pair's senses
/// (opposed — it is a real rest seat), and nothing asks whether the
/// crossing edges lie in the carrier. If the rung ever learns the
/// coplanarity screen the spec's own words ("two coplanar boundary
/// edges") imply, this probe goes red at the second assertion — which
/// is the point.
#[test]
fn r1_a_diving_edge_crossing_is_backed_by_the_seat_pair() {
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
    // VANISH — backed by a verified pair whose carrier neither edge
    // lies in. A transverse crossing certified, against the ruling's
    // fork.
    let found = crossing_witnesses(&errors(&body, &declared(&[(post_top, shelf_bottom)])));
    let backed_dives: Vec<_> = found
        .iter()
        .filter(|w| dive_witness(w) && at_seat_level(w))
        .collect();
    assert_eq!(
        backed_dives.len(),
        0,
        "the seat declaration backs the spike's transverse diving \
         crossings (this assertion documents the defect; a coplanarity \
         screen would red it): {found:?}"
    );
}

/// **Probe 2 — a 45-degree skew pair names a false SAME-SIDE
/// verdict.** The straddle fixture plus a wedge prism whose side face
/// lies in the plane `x = y` (carrier through the crossing point
/// (0.30, 0.30, 0.5) at 45 degrees to the post's `x = 0.30` side
/// face). Declaring (post_side_x030, wedge_45_face) — two faces with
/// NO shared carrier — the pair "holds" the crossing point (it is on
/// both planes and in both closed trims), and
/// `classify_material_pairing` answers a definite verdict from
/// normals at 45 degrees (dot = ±cos 45, far outside every band):
/// the refusal then NAMES a side verdict for a pair the confirm
/// pass's Door 1 would have screened out for carrier non-identity —
/// the sense algebra consumed outside its stated Smooth-tangent-plane
/// precondition. Whichever wedge orientation, the named verdict (or
/// an escalation) is manufactured from a question that was never
/// validly posed; this probe pins the orientation that yields
/// "same-side", the C6 admission-evidence channel.
#[test]
fn r1_a_skew_pair_names_a_side_verdict_without_a_shared_carrier() {
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
    let wedge: common::Prism<f64> = common::prism_z(
        &[(0.40, 0.40), (0.20, 0.20), (0.40, 0.10)],
        0.40,
        0.60,
    );
    // post.side_faces[3] spans (0.30, 0.42) → (0.30, 0.20): the plane
    // x = 0.30, outward normal (-1, 0, 0).
    let post_side_x030 = post.side_faces[3];
    let wedge_45 = wedge.side_faces[0];
    let mut body = post.body;
    let _ = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let wkeys = topo::graft_disjoint_all_keyed(&mut body, &wedge.body, Tol::witness()).unwrap();
    let wedge_45 = wkeys.face(wedge_45).unwrap();

    let found = errors(&body, &declared(&[(post_side_x030, wedge_45)]));
    let named: Vec<String> = crossing_witnesses(&found)
        .into_iter()
        .filter(|w| w.contains("x: 0.3,") && w.contains("side verdict"))
        .collect();
    assert!(
        !named.is_empty(),
        "the skew (45-degree) declared pair speaks for the crossing \
         at (0.30, 0.30, 0.5): {found:?}"
    );
    assert!(
        named.iter().all(|w| w.contains("side verdict: same-side")),
        "and what it says is SAME-SIDE 'of the shared carrier' — for \
         two carriers meeting at 45 degrees (no shared carrier \
         exists; the pairing question is outside \
         classify_material_pairing's stated precondition): {named:?}"
    );
}

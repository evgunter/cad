//! **R2 review probes for MATE-9** (the `EdgeEdgeCross` backing rung,
//! written against PR #1496 at `b873d783`; probe 2 INVERTED by the
//! fix pass to pin its fix, red-first order adopt → fix → invert).
//!
//! Two probes (the pairing is asked only of a tangency token, so a
//! pairing at a transverse crossing is no longer a state to probe):
//!
//! 2. **Order-sensitivity, now order-freedom** (inverted). At the
//!    frozen head the rung pushed `CensusEscalated` from inside its
//!    candidate loop: duplicates per stored orientation, silenced by
//!    whichever backing pair sorted first. The two-pass rung walks
//!    unordered pairs, collects, and emits once; the perpendicular
//!    fixture is screened before its invalid question is posed. The
//!    row pins both.
//!
//! 3. **`pair_region_verified` was unguarded.** Bypassing the entire
//!    verified-interface half of the rung (Door 1
//!    `rest_pair_verdict` + Door 2 `declared_overlap`, MATE-8's
//!    witness rung included) left the whole topo suite AND the viewer
//!    windmill story green at the frozen head. Probe 3 is the
//!    conjunction control (a verified-but-remote pair backs nothing)
//!    and stays as written; the Door-2-ISOLATING row it says the unit
//!    owes now exists —
//!    `mate9_crossing_rung::an_unverified_point_holding_pair_backs_no_crossing`,
//!    the kitty-corner pair that fails ONLY region verification and
//!    goes red under the `pair_region_verified → true` mutation.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, FaceKey, ValidationError};

/// The straddle seat of the MATE-9 suite, verbatim: a cap
/// `[0.30, 0.60] x [0.20, 0.42]` under a shelf `[0, 0.9] x [0, 0.30]`,
/// contact plane `z = 0.5`, the cap's side edges crossing the shelf's
/// `y = 0.30` boundary at `(0.30, 0.30)` and `(0.60, 0.30)`.
fn straddle_parts() -> (Body<f64>, FaceKey, FaceKey, FaceKey, FaceKey) {
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
    let post_side_x030 = post.side_faces[3];
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    let shelf_side_y030 = keys.face(shelf.side_faces[2]).unwrap();
    (
        body,
        post.top_face,
        post_side_x030,
        shelf_bottom,
        shelf_side_y030,
    )
}

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

fn crossings(errors: &[ValidationError]) -> Vec<(CensusContact, String)> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UnrecordedContact {
                contact: contact @ CensusContact::EdgeEdgeCross { .. },
                witness,
                ..
            } => Some((*contact, witness.clone())),
            _ => None,
        })
        .collect()
}

/// **Probe 2 — the rung's posture is a total function of the
/// geometry** (INVERTED; at the frozen head it pinned a duplicated,
/// order-silenced escalation).
///
/// At head b873d783 the side test pushed `CensusEscalated` from
/// inside the candidate loop: one perpendicular pair pushed TWO
/// identical escalations (both stored orientations ran), and adding
/// an unrelated backing pair made the loop `return` before the
/// escalating candidate — the side question's fate depended on
/// `FaceKey` sort order. The fix pass restructured the rung two-pass
/// (one walk per UNORDERED pair, outcomes collected, emission once)
/// AND screened the perpendicular pair out before any side question
/// is posed (`pair_holds_edges` — the pair's carrier does not hold
/// the crossing edges). Pinned now:
///
/// 1. the perpendicular declaration alone raises NO escalation and
///    names nothing — the invalid question is never asked (its own
///    `ContactContradicted` at the confirm pass is its refusal);
/// 2. the combined outcome is identical WHICHEVER order the two
///    declarations are recorded in — nothing about the census's
///    answer reads declaration order.
#[test]
fn r2_the_side_question_is_order_free_and_never_duplicated() {
    let (body, post_top, post_side_x030, shelf_bottom, shelf_side_y030) = straddle_parts();

    let perp_only = structural(
        &body,
        &declared(&body, &[(post_side_x030, shelf_side_y030)]),
    );
    let escalations = perp_only
        .iter()
        .filter(|e| matches!(e, ValidationError::CensusEscalated { .. }))
        .count();
    assert_eq!(
        escalations, 0,
        "the perpendicular pair is screened before the side question \
         exists — no escalation, duplicated or otherwise: {perp_only:?}"
    );
    assert_eq!(
        crossings(&perp_only).len(),
        2,
        "and the crossings stay plain hard findings: {perp_only:?}"
    );

    // The seat certifies alone through the resting pair.
    let clean = errors(&body, &declared(&body, &[(post_top, shelf_bottom)]));
    assert!(clean.is_empty(), "the seat certifies alone: {clean:?}");

    // Both declarations together, in BOTH record orders: identical
    // output — the crossings are backed by the seat pair, the
    // perpendicular pair contributes exactly its own confirm-pass
    // refusal, and nothing depends on which pair sorts first.
    let ab = errors(
        &body,
        &declared(
            &body,
            &[(post_top, shelf_bottom), (post_side_x030, shelf_side_y030)],
        ),
    );
    let ba = errors(
        &body,
        &declared(
            &body,
            &[(post_side_x030, shelf_side_y030), (post_top, shelf_bottom)],
        ),
    );
    assert_eq!(
        format!("{ab:?}"),
        format!("{ba:?}"),
        "the census's answer is order-free in the declarations"
    );
    assert!(crossings(&ab).is_empty(), "crossings backed: {ab:?}");
    assert!(
        !ab.iter()
            .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
        "and still no escalation — same screened question, backed or \
         not: {ab:?}"
    );
    assert!(
        ab.iter()
            .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
        "the perpendicular declaration's own refusal is what remains: \
         {ab:?}"
    );
}

/// **Probe 3 — the coverage row `pair_region_verified` does not have.**
///
/// Replacing the whole `pair_region_verified(..)` call with `true` (the
/// two doors, `rest_pair_verdict` + `declared_overlap` with
/// MATE-8's witness rung, deleted) leaves `cargo test -p topo` at
/// 445/445 and the viewer windmill story green — measured at this
/// head. Nothing in the tree goes red when the verified-interface half
/// of the unified strength is removed.
///
/// This is the row that would: an OPPOSED, point-holding, but
/// UNVERIFIED declared pair. The post's top face rests at `z = 0.5`
/// under the shelf; a SECOND shelf sits at `z = 0.5` too but its
/// footprint `[0.62, 0.9] x [0, 0.30]` shares no area with the post
/// `[0.30, 0.60] x [0.20, 0.42]` — the pair is coplanar and opposed in
/// sense, and its region overlap is EMPTY, so Door 2 refuses it. It
/// holds no crossing point either, so the row is a control on the
/// conjunction rather than on Door 2 in isolation; a Door-2-isolating
/// fixture needs a pair whose closed regions both hold the crossing
/// point while their overlap has no positive area, which is the row
/// this unit owes.
#[test]
fn r2_an_unverified_opposed_pair_backs_no_crossing() {
    let (body_base, post_top, _, shelf_bottom, _) = straddle_parts();
    let _ = (post_top, shelf_bottom);
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
    let far: common::Prism<f64> = common::prism_z(
        &[(0.62, 0.02), (0.86, 0.02), (0.86, 0.26), (0.62, 0.26)],
        0.2,
        0.5,
        Tol::witness(),
    );
    let mut body = post.body;
    let skeys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let shelf_b = skeys.face(shelf.bottom_face).unwrap();
    let fkeys = topo::graft_disjoint_all_keyed(&mut body, &far.body).unwrap();
    let far_top = fkeys.face(far.top_face).unwrap();
    let _ = body_base;

    // The far post genuinely rests under the shelf, so declaring it is
    // honest; what it must NOT do is answer for the OTHER post's
    // crossings. Those lie on the boundary of that post's own seat,
    // which the census confirms whether or not it is recorded, so they
    // are no finding with the far pair declared or without it.
    let bare = crossings(&errors(&body, &ContactRecords::default()));
    let with = crossings(&errors(&body, &declared(&body, &[(far_top, shelf_b)])));
    assert_eq!(
        format!("{bare:?}"),
        format!("{with:?}"),
        "a pair that does not hold the crossing points answers for \
         nothing about them"
    );
    assert!(bare.is_empty(), "{bare:?}");
}

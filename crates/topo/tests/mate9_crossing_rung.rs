//! **The `EdgeEdgeCross` backing rung** — the unified strength's
//! first instance (issue 973 part (b), stage 1;
//! `crates/topo/README.md`, option A, planar-first).
//!
//! The ratified geometric fork, exercised row by row: an
//! in-contact-plane crossing at a declared seat is LEGAL — the two
//! bodies' material lies on opposite sides of the shared carrier
//! (an overhanging seat, ordinary authoring) — while a TRANSVERSE
//! crossing (material on ONE side) is interpenetration. The rung's
//! side verdict is deliberately THREE-valued: opposite-sides backs
//! (the seat certifies), same-side refuses carrying the verdict as
//! the refusal's `side` field (between two copies at rest the editor's
//! gate reads it as interference evidence, ASSEMBLY A5), undecided
//! escalates typed. The first arm is reached below. The second is not
//! reached on this corpus either: a crossing whose two faces lie on one
//! plane facing the same way lies in the closure of that aligned pair's
//! shared area, which the face-pair arm decides as one
//! `SameSideInterference` that answers for it, so the same-side verdict
//! is left for a pair whose overlap the arm cannot decide (in band, or a
//! face with an arc). The THIRD is not reachable on this corpus, and
//! that is a statement about the fix-pass screens rather than a
//! coverage gap: undecided now requires a candidate that HOLDS the
//! whole crossing (point and both edges in its carrier) yet cannot
//! validly reach a side answer — an in-band margin, or a
//! Smooth-precondition failure AFTER the carrier screens passed —
//! and exact-literal fixtures decide every such margin. The
//! perpendicular- and skew-pair fixtures that used to reach a
//! verdict here are exactly what the edge screen now refuses
//! silently (`review_mate9_r1_probes`, `review_mate9_r2_probes`
//! carry that history).
//!
//! The rung's confinement is exercised from every side: the
//! verified-elsewhere control (a verified pair that does not hold the
//! crossing point answers for nothing), the Door-2 isolator (a pair
//! that holds the point, passes the side test, and still backs
//! nothing because its overlap region has no positive area), and the
//! perpendicular control (a pair that holds the POINT but not the
//! EDGES names nothing at all). So the rung is born WITHOUT the reach
//! the grandfathered rungs carry (`review_mate4a_r2_probes`' probe 1
//! pins that reach where it still lives).
//!
//! `EdgeFacePierce` takes no rung at all: a transverse dive is
//! interpenetration, which no declaration admits (C6). The pierce pin
//! below holds that door shut.
//!
//! The re-blessed (b) fence — the declared straddle seat certifying
//! outright, and its bare control byte-pinned — lives with the
//! MATE-4a rows in `mate4a_ef_bound_rung.rs`; the seat itself is
//! [`common::straddle_seat`], one builder for both suites.
//!
//! ε posture (issue-1356 discipline): every coincidence is a shared
//! f64 literal and every fixture here is AXIS-ALIGNED, so the margins
//! the crossing point (`pm_census_ee_gap`/`_span`), the region
//! decisions (`pm_census_confined_carrier`, `contfp`'s rows, the
//! chart-region rows behind `declared_overlap`), the dihedral gate
//! and the side test (`dihedral_wedge`, `material_wedge_side`)
//! read are exact zeros or plain coordinate differences: the
//! smallest nonzero separation any fixture in THIS file carries is
//! 0.04 m (the shelf slab's thickness; the next are 0.05 and 0.08 —
//! the flush block's and corner block's offsets), six orders above
//! the loosest gated band's escalate threshold (1e-5, at the 1e-6
//! row). The often-quoted 8.944e-3 near-parallel margin belongs to
//! the MATE-4a OVERHANG seat's slanted cap and does not occur here.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, CrossingSideVerdict, FaceKey, ValidationError};

/// [`common::straddle_seat`] as this file's tuple:
/// `(body, post_top, post_side_x030, shelf_bottom, shelf_side_y030)`.
fn straddle_parts() -> (Body<f64>, FaceKey, FaceKey, FaceKey, FaceKey) {
    let seat = common::straddle_seat(Tol::witness());
    (
        seat.body,
        seat.post_top,
        seat.post_side_x030,
        seat.shelf_bottom,
        seat.shelf_side_y030,
    )
}

fn declared<T: geom_core::Real>(body: &Body<T>, pairs: &[(FaceKey, FaceKey)]) -> ContactRecords {
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

/// A crossing's side verdict.
fn side(contact: &CensusContact) -> Option<CrossingSideVerdict> {
    match contact {
        CensusContact::EdgeEdgeCross { side, .. } => *side,
        _ => None,
    }
}

/// The `EdgeEdgeCross` refusals of a list, as `(finding, witness)`.
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

/// **The headline row, both ways (frame invariance).** The declared
/// overhang seat — boundary edges crossing INSIDE the declared
/// region, material on opposite sides of the shared carrier — was a
/// hard `EdgeEdgeCross` refusal since the census existed; the rung
/// backs it and the seat certifies outright, whichever face the
/// record names first. The world-carrier arm reads the FIRST face's
/// frame, so the two orders exercise both frames; certified answers
/// are frame-invariant (`world_carrier`'s lemma), and this row is the
/// at-rest pin of that lemma for the crossing rung.
#[test]
fn the_declared_crossing_seat_certifies_both_ways() {
    let (body, post_top, _, shelf_bottom, _) = straddle_parts();
    for pair in [(post_top, shelf_bottom), (shelf_bottom, post_top)] {
        let found = errors(&body, &declared(&body, &[pair]));
        assert!(
            found.is_empty(),
            "the declared overhang seat certifies with the record in \
             either order (declared {pair:?}): {found:?}"
        );
    }
}

/// **The transverse fork: a same-side seat is one interference.** The
/// same straddle in plan view, but the post sits ABOVE the contact
/// plane — its material and the shelf's both lie above `z = 0.5`, so
/// the seat is interpenetration, not rest. The two bottoms lie on one
/// plane facing the same way and share interior: the face-pair arm
/// decides the overlap as one `SameSideInterference`, and the two
/// crossing points, in the closure of that shared area, are the pair's
/// and no finding of their own, declared or not — the interference the
/// at-rest gate reads between copies. The lying declaration itself is
/// `ContactContradicted` (aligned senses), independently; run both ways
/// for the frame.
#[test]
fn a_transverse_crossing_is_the_aligned_pairs_interference() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.5,
        1.0,
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

    // One overlap: the aligned pair decides it, beside the pierces of
    // the shelf's edges through the post that decide the same pair, and
    // nothing refuses the pair as undecidable.
    let one_overlap = |found: &[ValidationError]| {
        let placements = common::placement_findings(&body, found);
        let pairs: std::collections::BTreeSet<_> = placements
            .iter()
            .filter_map(|e| topo::census_interference_solids(&body, e))
            .map(|(x, y)| (x.min(y), x.max(y)))
            .collect();
        crossings(found).is_empty()
            && pairs.len() == 1
            && placements
                .iter()
                .all(|e| topo::census_interference_solids(&body, e).is_some())
            && placements
                .iter()
                .filter(|e| matches!(e, ValidationError::SameSideInterference { .. }))
                .count()
                == 1
    };
    let bare = errors(&body, &ContactRecords::default());
    assert!(one_overlap(&bare), "{bare:?}");

    for pair in [
        (post.bottom_face, shelf_bottom),
        (shelf_bottom, post.bottom_face),
    ] {
        let found = errors(&body, &declared(&body, &[pair]));
        assert!(one_overlap(&found), "declared {pair:?}: {found:?}");
        assert!(
            found
                .iter()
                .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
            "and the aligned-sense declaration is contradicted on its \
             own: {found:?}"
        );
    }
}

/// **The region half**: a declared pair that is GENUINELY VERIFIED — a
/// second post resting flush under the shelf, door 1 and door 2 both
/// answering — but whose interface does not hold the crossing points
/// changes nothing about them. The crossings lie on the boundary of the
/// seat's own pair, cap on underside, which the census confirms whether
/// or not it is recorded and which answers for them, inside its
/// overlap's closure, so they are no finding
/// either way; the seat's pair is the one finding, unrecorded. The
/// rung's own confinement against a confirmed pair elsewhere is
/// `census::tests::a_confirmed_pair_elsewhere_backs_no_crossing`.
#[test]
fn a_verified_pair_elsewhere_backs_no_crossing() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
        Tol::witness(),
    );
    let flush: common::Prism<f64> = common::prism_z(
        &[(0.70, 0.05), (0.80, 0.05), (0.80, 0.25), (0.70, 0.25)],
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
    let fkeys = topo::graft_disjoint_all_keyed(&mut body, &flush.body).unwrap();
    let flush_top = fkeys.face(flush.top_face).unwrap();
    let skeys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let shelf_bottom = skeys.face(shelf.bottom_face).unwrap();

    let bare = errors(&body, &ContactRecords::default());
    let found = errors(&body, &declared(&body, &[(flush_top, shelf_bottom)]));
    let crossed = crossings(&found);
    assert_eq!(
        format!("{:?}", crossings(&bare)),
        format!("{crossed:?}"),
        "a verified pair whose region does not hold the crossing \
         points changes nothing about them"
    );
    assert!(crossed.is_empty(), "{found:?}");
    let seat = |errors: &[ValidationError]| {
        errors
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    ValidationError::UnrecordedContact {
                        contact: CensusContact::ConformalPatch { .. },
                        ..
                    }
                )
            })
            .count()
    };
    assert_eq!(seat(&found), 1, "the post's own seat: {found:?}");
    assert_eq!(seat(&bare), 2, "both seats, bare: {bare:?}");
    // And the flush pair's own seat IS answered: its declaration is
    // neither contradicted nor stale, and no finding names its faces.
    assert!(
        !found.iter().any(|e| matches!(
            e,
            ValidationError::ContactContradicted { .. }
                | ValidationError::StaleContactDeclaration { .. }
        )),
        "the elsewhere pair is genuinely verified: {found:?}"
    );
}

/// **The EDGE screen: a pair that holds the POINT but not the EDGES
/// names nothing at all.** The post's `x = 0.30` side face against
/// the shelf's `y = 0.30` side face — two PERPENDICULAR planes that
/// both pass through the crossing point `(0.30, 0.30, 0.5)` — holds
/// that point in both closed regions, and before the fix pass it
/// reached the sense algebra there: a question about "the shared
/// carrier" asked of a pair that HAS no shared carrier, answered
/// `MarginKind::Invalid` at every band and mis-named "undecided at
/// this ε" (`review_mate9_r2_probes` probe 1 is that record). The
/// edge screen now refuses the pair before any side question is
/// posed — the cap's crossing edge does not lie in the shelf-side
/// plane — so: no escalation, no named verdict, both crossings stay
/// PLAIN hard findings, and the only refusal the perpendicular
/// declaration earns is its own (`ContactContradicted`, Door 1's
/// carrier non-identity, at the confirm pass — loudness lives where
/// the pair's own state is reported).
#[test]
fn a_perpendicular_pair_holding_the_point_names_nothing() {
    let (body, _, post_side_x030, _, shelf_side_y030) = straddle_parts();
    let pair = (post_side_x030, shelf_side_y030);
    let found = structural(&body, &declared(&body, &[pair]));
    let crossed = crossings(&found);
    assert_eq!(crossed.len(), 2, "{found:?}");
    assert!(
        crossed.iter().all(|(c, _)| side(c).is_none()),
        "no side question is posed for a pair whose carrier does not \
         hold the crossing edges: {crossed:?}"
    );
    assert!(
        !found
            .iter()
            .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
        "and nothing escalates — the invalid question is screened out, \
         not asked and apologized for: {found:?}"
    );
    assert!(
        found
            .iter()
            .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
        "the perpendicular declaration's own refusal stands at the \
         confirm pass: {found:?}"
    );
}

/// **The Door-2 isolator — the row `pair_region_verified` did not
/// have** (R2's MAJ-2: mutating that call to `true` left the whole
/// tree green). A corner block hangs beside the shelf, its underside
/// at `z = 0.5` spanning `[0.10, 0.30] x [0.30, 0.50]` — kitty-corner
/// to the cap across the crossing point `(0.30, 0.30, 0.5)`. Declared
/// `(post_top, block_bottom)`, the pair passes EVERYTHING up to the
/// doors: coplanar carriers holding the point (cap `OnEdge`, block
/// `OnVertex` — the closed containment), both crossing edges in the
/// carrier, a Smooth site, material genuinely OPPOSED (post below,
/// block above). What fails is the verified interface itself: the two
/// regions share only the boundary segment `x = 0.30,
/// y ∈ [0.30, 0.42]` — zero area — so the region walk refuses
/// `TouchingBoundary`, the witness rung finds no interior point, and
/// Door 2 never answers `PositiveArea`. The pair answers for nothing;
/// the events on the boundary segment the two share stand as they do
/// with nothing declared. (The crossing it corner-touches lies on the
/// boundary of the post's own seat under the shelf, which the census
/// confirms and which answers for it, inside its overlap's closure.)
///
/// GUARD CONTRACT: this row goes red under the mutation
/// `pair_region_verified(..) → true` — the verified-interface
/// conjunct's coverage. The point-half's control mutation
/// (`pair_holds_point(..) → true`) is covered by
/// `a_verified_pair_elsewhere_backs_no_crossing`. Both mutations'
/// measured red counts are recorded in the PR.
#[test]
fn an_unverified_point_holding_pair_backs_no_crossing() {
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
    let block: common::Prism<f64> = common::prism_z(
        &[(0.10, 0.30), (0.30, 0.30), (0.30, 0.50), (0.10, 0.50)],
        0.5,
        0.54,
        Tol::witness(),
    );
    let post_top = post.top_face;
    let mut body = post.body;
    let _ = topo::graft_disjoint_all_keyed(&mut body, &shelf.body).unwrap();
    let bkeys = topo::graft_disjoint_all_keyed(&mut body, &block.body).unwrap();
    let block_bottom = bkeys.face(block.bottom_face).unwrap();

    let unrecorded = |errors: &[ValidationError]| -> Vec<CensusContact> {
        errors
            .iter()
            .filter_map(|e| match e {
                ValidationError::UnrecordedContact { contact, .. } => Some(*contact),
                _ => None,
            })
            .collect()
    };
    let bare = unrecorded(&errors(&body, &ContactRecords::default()));
    let found = errors(&body, &declared(&body, &[(post_top, block_bottom)]));
    let with = unrecorded(&found);
    assert_eq!(
        bare, with,
        "a point-holding, edge-holding, opposite-sided pair whose \
         overlap region has no positive area answers for nothing"
    );
    assert_eq!(
        with.iter()
            .filter(|c| matches!(c, CensusContact::VertexOnEdge { .. }))
            .count(),
        2,
        "the two corners on the shared segment stand: {found:?}"
    );
    assert!(crossings(&found).is_empty(), "{found:?}");
}

/// **The pierce pin.** `EdgeFacePierce` consults NO rung: a
/// transverse dive through a face's interior is interpenetration
/// however the seat is declared: no declaration admits one (C6), and
/// between two copies at rest it is an interference finding, not a
/// contact. A tall post run through the
/// thin shelf pierces both shelf faces at its two sub-shelf corners'
/// vertical edges; declaring the pierced face against the piercing
/// edge's own side face changes not one pierce finding.
#[test]
fn the_pierce_stays_categorical() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        1.0,
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
    let pierces = |records: &ContactRecords| -> Vec<String> {
        errors(&body, records)
            .iter()
            .filter_map(|e| match e {
                ValidationError::UnrecordedContact {
                    contact: CensusContact::EdgeFacePierce { edge, face },
                    witness,
                    ..
                } => Some(format!("{edge:?} through {face:?} at {witness}")),
                _ => None,
            })
            .collect()
    };
    let bare = pierces(&ContactRecords::default());
    assert!(
        !bare.is_empty(),
        "the tall post pierces the shelf somewhere"
    );
    let with_declaration = pierces(&declared(&body, &[(post.side_faces[0], shelf_bottom)]));
    assert_eq!(
        bare, with_declaration,
        "no declaration reaches a pierce: the class is categorical"
    );
}

/// **The `None` path through the public door, at the scalar that has no
/// other.** The straddle seat rebuilt at `Dual64` and declared, run
/// through `validate_pseudomanifold_structural` — the tier-3′ door a
/// dual body goes by — passes tiers 1–3 (it is the certified seat's
/// value channel, bit for bit) and reaches the census, whose pass
/// holds no region door: the declared pair is exactly one
/// `CensusLaneUnsupported` naming it, and the two crossings the
/// certified door sees backed are the hard findings the undeclared
/// seat has. The `_structural` door at `f64` says the same on the
/// same seat, which is the point: the refusal is the DOOR's, and the
/// dual is only the scalar that has no other door.
#[test]
fn the_structural_door_at_a_dual_refuses_the_declared_seat_typed_and_backs_no_crossing() {
    use geom_core::Dual64;
    let tol = Tol::witness();
    let post: common::Prism<Dual64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
        tol,
    );
    let shelf: common::Prism<Dual64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
        tol,
    );
    let mut dual = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut dual, &shelf.body).unwrap();
    let pair = (post.top_face, keys.face(shelf.bottom_face).unwrap());

    fn lane_refusals(errors: &[ValidationError]) -> Vec<&ValidationError> {
        errors
            .iter()
            .filter(|e| matches!(e, ValidationError::CensusLaneUnsupported { .. }))
            .collect()
    }
    let structural = |body: &Body<Dual64>, records: &ContactRecords| -> Vec<ValidationError> {
        topo::validate_pseudomanifold_structural(body, records, tol).expect_err(
            "the census pass at a dual holds no region door, so the declared seat refuses",
        )
    };

    let bare = structural(&dual, &ContactRecords::default());
    let bare_crossings = crossings(&bare);
    assert_eq!(
        bare_crossings.len(),
        2,
        "the undeclared seat's two crossings: {bare:?}"
    );
    assert!(
        lane_refusals(&bare).is_empty(),
        "no declaration, no consult: {bare:?}"
    );

    let found = structural(&dual, &declared(&dual, &[pair]));
    let refusals = lane_refusals(&found);
    assert_eq!(
        refusals.len(),
        1,
        "one declared pair, one refusal: {found:?}"
    );
    assert_eq!(
        *refusals[0],
        ValidationError::CensusLaneUnsupported {
            subject: topo::CensusSubject::FacePair(pair.0, pair.1),
        }
    );
    assert_eq!(
        crossings(&found),
        bare_crossings,
        "the declared pair backs nothing through a door that holds no region door: {found:?}"
    );

    // The same door at `f64`, on the same seat, keyed the same way:
    // the refusal is the door's and not the scalar's.
    let (seat, post_top, _, shelf_bottom, _) = straddle_parts();
    assert_eq!(
        (post_top, shelf_bottom),
        pair,
        "the two seats are built in one arena order"
    );
    let at_f64 = topo::validate_pseudomanifold_structural(&seat, &declared(&seat, &[pair]), tol)
        .expect_err("the `_structural` door at f64 holds no region door either");
    let shape = |errors: &[ValidationError]| -> Vec<String> {
        errors
            .iter()
            .map(|e| match e {
                ValidationError::UnrecordedContact { contact, .. } => format!("{contact:?}"),
                other => format!("{other:?}"),
            })
            .collect()
    };
    assert_eq!(
        shape(&at_f64),
        shape(&found),
        "the `_structural` verdict is the same at f64 and at the dual, the witnesses' scalar \
         rendering aside"
    );
    assert!(
        errors(&seat, &declared(&seat, &[pair])).is_empty(),
        "and the certified door, holding the region door, certifies the seat"
    );
}

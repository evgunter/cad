//! **The `EdgeEdgeCross` backing rung** — the unified strength's
//! first instance (issue 973 part (b), stage 1;
//! `docs/MATE-4B-CROSSING-DESIGN.md`, option A, planar-first).
//!
//! The ratified geometric fork, exercised row by row: an
//! in-contact-plane crossing at a declared seat is LEGAL — the two
//! bodies' material lies on opposite sides of the shared carrier
//! (an overhanging seat, ordinary authoring) — while a TRANSVERSE
//! crossing (material on ONE side) is interpenetration. The rung's
//! side verdict is deliberately THREE-valued: opposite-sides backs
//! (the seat certifies), same-side refuses NAMING the verdict (the
//! C6 declared-interpenetration hook — a future class consumes it as
//! admission evidence), undecided escalates typed. The first two arms
//! are reached below; the THIRD is not reachable on this corpus, and
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
//! interpenetration until C6's recorded gate-skips exist, and the
//! MATE-4b staging defers that arm to C6's era BY NAME (stage 2).
//! The pierce pin below holds that door shut.
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
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

/// [`common::straddle_seat`] as this file's tuple:
/// `(body, post_top, post_side_x030, shelf_bottom, shelf_side_y030)`.
fn straddle_parts() -> (Body<f64>, FaceKey, FaceKey, FaceKey, FaceKey) {
    let seat = common::straddle_seat();
    (
        seat.body,
        seat.post_top,
        seat.post_side_x030,
        seat.shelf_bottom,
        seat.shelf_side_y030,
    )
}

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

/// The `EdgeEdgeCross` refusals of a list, as `(finding, witness)`.
fn crossings(errors: &[ValidationError]) -> Vec<(CensusContact, String)> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UndeclaredContact {
                contact: contact @ CensusContact::EdgeEdgeCross { .. },
                witness,
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
        let found = errors(&body, &declared(&[pair]));
        assert!(
            found.is_empty(),
            "the declared overhang seat certifies with the record in \
             either order (declared {pair:?}): {found:?}"
        );
    }
}

/// **The transverse fork: same-side refuses, NAMING the verdict.**
/// The same straddle in plan view, but the post sits ABOVE the
/// contact plane — its material and the shelf's both lie above
/// `z = 0.5`, so the seat is interpenetration, not rest. The declared
/// pair holds both crossing points (the geometry is unchanged in
/// plan), the side test answers SAME-SIDE, and the refusal carries
/// the verdict by name — the declared-interpenetration hook: C6's
/// future class consumes exactly this verdict as admission evidence,
/// which is why the finding must never regress to a plain undeclared
/// crossing. The lying declaration itself is `ContactContradicted`
/// (aligned senses), independently — both refusals stand, run both
/// ways for the frame.
#[test]
fn a_transverse_crossing_refuses_naming_the_side_verdict() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.5,
        1.0,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();

    // Bare control: the crossings refuse PLAIN — no verdict is named
    // where no declared pair answered.
    let bare = crossings(&errors(&body, &ContactRecords::default()));
    assert_eq!(bare.len(), 2, "{bare:?}");
    assert!(
        bare.iter().all(|(_, w)| !w.contains("side verdict")),
        "an unanswered crossing names no verdict: {bare:?}"
    );

    for pair in [
        (post.bottom_face, shelf_bottom),
        (shelf_bottom, post.bottom_face),
    ] {
        let found = errors(&body, &declared(&[pair]));
        let crossed = crossings(&found);
        assert_eq!(crossed.len(), 2, "declared {pair:?}: {found:?}");
        assert!(
            crossed
                .iter()
                .all(|(_, w)| w.contains("side verdict: same-side")),
            "a transverse crossing refuses with the verdict NAMED \
             (declared {pair:?}): {crossed:?}"
        );
        assert!(
            found
                .iter()
                .any(|e| matches!(e, ValidationError::ContactContradicted { .. })),
            "and the aligned-sense declaration is contradicted on its \
             own: {found:?}"
        );
    }
}

/// **The region half, attacked the way the grandfathered rungs give
/// in** (`review_mate4a_r2_probes` probe 1): a declared pair that is
/// GENUINELY VERIFIED — a second post resting flush under the shelf,
/// door 1 and door 2 both answering — but whose interface does not
/// hold the crossing points. The rung is born region-confined: the
/// verified-elsewhere pair answers for nothing here, the crossings
/// stay hard, and their witnesses are byte-identical to the bare
/// seat's (no verdict named — the pair never spoke).
#[test]
fn a_verified_pair_elsewhere_backs_no_crossing() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
    );
    let flush: common::Prism<f64> = common::prism_z(
        &[(0.70, 0.05), (0.80, 0.05), (0.80, 0.25), (0.70, 0.25)],
        0.0,
        0.5,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let mut body = post.body;
    let fkeys = topo::graft_disjoint_all_keyed(&mut body, &flush.body, Tol::witness()).unwrap();
    let flush_top = fkeys.face(flush.top_face).unwrap();
    let skeys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = skeys.face(shelf.bottom_face).unwrap();

    let bare = crossings(&errors(&body, &ContactRecords::default()));
    let found = errors(&body, &declared(&[(flush_top, shelf_bottom)]));
    let crossed = crossings(&found);
    assert_eq!(
        format!("{bare:?}"),
        format!("{crossed:?}"),
        "a verified pair whose region does not hold the crossing \
         points changes nothing about them"
    );
    assert_eq!(crossed.len(), 2, "{found:?}");
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
/// `MarginDiag::Invalid` at every band and mis-named "undecided at
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
    let found = errors(&body, &declared(&[pair]));
    let crossed = crossings(&found);
    assert_eq!(crossed.len(), 2, "{found:?}");
    assert!(
        crossed.iter().all(|(_, w)| !w.contains("side verdict")),
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
/// the crossing it corner-touches stays a hard finding.
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
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let block: common::Prism<f64> = common::prism_z(
        &[(0.10, 0.30), (0.30, 0.30), (0.30, 0.50), (0.10, 0.50)],
        0.5,
        0.54,
    );
    let post_top = post.top_face;
    let mut body = post.body;
    let _ = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let bkeys = topo::graft_disjoint_all_keyed(&mut body, &block.body, Tol::witness()).unwrap();
    let block_bottom = bkeys.face(block.bottom_face).unwrap();

    let bare = crossings(&errors(&body, &ContactRecords::default()));
    let found = errors(&body, &declared(&[(post_top, block_bottom)]));
    let with = crossings(&found);
    assert_eq!(
        format!("{bare:?}"),
        format!("{with:?}"),
        "a point-holding, edge-holding, opposite-sided pair whose \
         overlap region has no positive area answers for nothing"
    );
    assert_eq!(with.len(), 2, "{found:?}");
    assert!(
        with.iter().all(|(_, w)| !w.contains("side verdict")),
        "and no verdict is named — the pair never verified, so it \
         never spoke: {with:?}"
    );
}

/// **The pierce pin.** `EdgeFacePierce` consults NO rung: a
/// transverse dive through a face's interior is interpenetration
/// however the seat is declared, and the vocabulary that could admit
/// one — C6's recorded interference gate-skips — does not exist yet.
/// The MATE-4b ruling defers that arm to C6's interference-fit era BY
/// NAME (staging, stage 2); this unit lands stage 1 only, and issue
/// 973 stays OPEN over the pierce arm. A tall post run through the
/// thin shelf pierces both shelf faces at its two sub-shelf corners'
/// vertical edges; declaring the pierced face against the piercing
/// edge's own side face changes not one pierce finding.
#[test]
fn the_pierce_stays_categorical() {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        1.0,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    let pierces = |records: &ContactRecords| -> Vec<String> {
        errors(&body, records)
            .iter()
            .filter_map(|e| match e {
                ValidationError::UndeclaredContact {
                    contact: CensusContact::EdgeFacePierce { edge, face },
                    witness,
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
    let with_declaration = pierces(&declared(&[(post.side_faces[0], shelf_bottom)]));
    assert_eq!(
        bare, with_declaration,
        "no declaration reaches a pierce: the class is categorical \
         until C6's era, by name"
    );
}

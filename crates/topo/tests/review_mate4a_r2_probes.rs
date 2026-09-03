//! **R2 review probes for MATE-4a** (PR 1432, frozen head 72140467).
//!
//! Adversarial probes against the unit's claims. These are review
//! artifacts, not a proposed contract: each one records a MEASURED
//! answer at the frozen head, and the assertions pin what was measured
//! so a later reader can see whether it moved.
//!
//! 1. `r2_an_unrelated_declared_pair_backs_the_ef_bound` — the new
//!    interior arm reads `ve_face_backed(w, e)`, which asks only that
//!    some declared pair hold a face incident to `w` against a face
//!    the EDGE bounds. The face `f` whose overlap is being certified is
//!    never named by the declaration. Two patches between the POST's
//!    vertical side faces and the SHELF's vertical side face — neither
//!    of which is the resting pair, and which are not even coplanar
//!    with each other — back both bounds and retire the hard
//!    `EdgeFaceOverlap`.
//! 2. `r2_the_touching_boundary_residue_is_not_the_new_arms_doing` —
//!    the `CensusUnsupported`/`TouchingBoundary` refusal was a property
//!    of the DECLARED PAIR's region relationship and of
//!    `chart_region::interior_witness`'s candidate schedule, not of the
//!    new arm: a notched cap that never reaches the interior arm at all
//!    landed the same refusal, and now certifies alongside the seat for
//!    the same reason. Measured at the frozen head by instrumenting
//!    `interior_witness`: on the PR's seat Door 1 is `Definite`, the
//!    rung RUNS, and it declined because the schedule of the day (each
//!    outer trim's vertex centroid + ear midpoints) never landed a
//!    point in the H-A-B overlap triangle — an overlap of ~7.5e-3 m2,
//!    not an undecidable area. That schedule now searches the trims'
//!    own arrangement and lands it.
//! 3. `r2_the_new_rows_hold_at_the_default_band` — the seat's two
//!    answers restated so the review's band sweep has a row: the
//!    binary was run at `CAD_TOLERANCE_EPS` = unset (1e-9), 1e-12 and
//!    1e-6 (the three gated rows) with the same answers at all three,
//!    and at 1e-3 (outside the gate) where the answers survive but
//!    four honest `CensusEscalated`s from `pm_census_ee_parallel`
//!    (margin 8.94e-3 in band [1e-3, 1e-2]) join the list. Re-measured
//!    at all four after the schedule completion: the declared seat's
//!    answer lost its one `CensusUnsupported` at every band, leaving an
//!    empty list at the three gated ones and those four escalations
//!    alone at 1e-3, and the bare seat's census is unmoved throughout.
//!    The row below asserts the GATED answer, as it did before — it was
//!    red at 1e-3 then too, for the same reason: 1e-3 is not a band the
//!    matrix runs, and the escalations are honest there.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom_core::Tol;
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

/// The PR's overhang seat, plus the side-face keys its own suite does
/// not expose: `post.side_faces[7]` is H→A, `[0]` is A→B (the two cap
/// side faces incident to the bound vertices H and B), and the shelf's
/// `side_faces[2]` is its `y = 0.30` face — the second face of the very
/// edge that dives through the cap.
fn overhang_seat_full() -> (Body<f64>, FaceKey, FaceKey, FaceKey, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[
            (0.20, 0.20), // A
            (0.40, 0.30), // B
            (0.60, 0.42), // C1
            (0.70, 0.30), // T
            (0.80, 0.42), // C2
            (0.85, 0.50), // G2
            (0.15, 0.50), // G
            (0.25, 0.30), // H
        ],
        0.0,
        0.5,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let post_side_ha = post.side_faces[7];
    let post_side_ab = post.side_faces[0];
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    let shelf_side_y030 = keys.face(shelf.side_faces[2]).unwrap();
    (
        body,
        post.top_face,
        shelf_bottom,
        post_side_ha,
        post_side_ab,
        shelf_side_y030,
    )
}

fn records(pairs: &[(FaceKey, FaceKey)]) -> ContactRecords {
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

fn errors(body: &Body<f64>, recs: &ContactRecords, tol: Tol) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, recs, tol) {
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

fn count_ef(cs: &[CensusContact]) -> usize {
    cs.iter()
        .filter(|c| matches!(c, CensusContact::EdgeFaceOverlap { .. }))
        .count()
}

/// **Probe 1 — the rung's reach, now the GRANDFATHER's documented
/// anomaly.** The declaration names the post's two vertical side
/// faces against the shelf's vertical side face. It does NOT name
/// `post_top` (the face the edge-on-face overlap is ON) and it does
/// NOT name `shelf_bottom`. Nothing in it asserts a coplanar rest.
///
/// Measured at head 72140467: it backs both bounds anyway, and the
/// hard `EdgeFaceOverlap` on `post_top` is gone. `ve_face_backed(w,
/// e)` asks for a pair holding SOME face incident to `w` against SOME
/// face of `e`; the overlap's own face is constrained only by `w`
/// lying on its boundary.
///
/// Under the UNIFIED strength (CONTACT-DESIGN C3/C4's annotation;
/// MATE-9) this reach is exactly what `ef_bound_backed`'s
/// grandfathering carries: the rung's region-confined variant was
/// implemented and MEASURED, and it refuses the overlap lane's cell
/// bounds wherever the cut schedule's reach gap puts a bound outside
/// the interface — the declared straddle seat's own dive cell
/// regresses — so the rung stays grandfathered, this row stays green
/// as the anomaly's pin, and the migration waits on
/// boundary-crossing cuts (the grandfather note names it). The
/// measurement is in MATE-9's PR and in this branch's history.
#[test]
fn r2_an_unrelated_declared_pair_backs_the_ef_bound() {
    let (body, post_top, _shelf_bottom, side_ha, side_ab, shelf_side) = overhang_seat_full();
    let bare = undeclared(&errors(&body, &ContactRecords::default(), Tol::witness()));
    assert_eq!(count_ef(&bare), 3, "the bare baseline: {bare:?}");

    let unrelated = errors(
        &body,
        &records(&[(side_ha, shelf_side), (side_ab, shelf_side)]),
        Tol::witness(),
    );
    // Measured full list at the frozen head: the two declarations are
    // themselves refused `ContactContradicted` (`bool_plane_parallel`
    // — they are not parallel planes), yet they still silenced the
    // census rung. The document refuses either way; what moved is
    // which lane refuses it.
    let contacts = undeclared(&unrelated);
    let dived: Vec<_> = contacts
        .iter()
        .filter(|c| matches!(c, CensusContact::EdgeFaceOverlap { face, .. } if *face == post_top))
        .collect();
    assert!(
        dived.is_empty(),
        "a declaration that never names post_top backed its edge-on-face \
         overlap anyway: {unrelated:?}"
    );
}

/// **Probe 2 — the `TouchingBoundary` residue is the declared pair's,
/// not the new arm's.** The PR body reads the seat's surviving
/// `CensusUnsupported` as the state the new arm parks this class in.
/// Measured here: the notched cap below, which touches the shelf's
/// `y = 0.30` edge at two vertices but never dives along it — so it has
/// no edge-on-face overlap on that edge and never reaches the interior
/// arm — moved in LOCKSTEP with the seat. At the frozen head both
/// raised `CensusUnsupported`; both are now certified outright, by
/// `interior_witness`'s completed schedule finding the sub-shelf
/// triangle (~6e-3 m² here) that its fixed landmarks used to miss. The
/// probe's claim is unchanged and is if anything better evidenced: a
/// cap that never reaches the interior arm answers whatever the seat
/// answers, so the residue was door 2's the whole time.
#[test]
fn r2_the_touching_boundary_residue_is_not_the_new_arms_doing() {
    // A cap that only TOUCHES the shelf's y = 0.30 edge at two vertices
    // and never dives along it: the segment H..B is replaced by a
    // notch that stays strictly below the edge between them.
    let post: common::Prism<f64> = common::prism_z(
        &[
            (0.25, 0.30),  // on the shelf edge
            (0.325, 0.22), // dips below — no collinear dive
            (0.40, 0.30),  // on the shelf edge
            (0.60, 0.50),
            (0.15, 0.50),
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
    let found = errors(
        &body,
        &records(&[(post.top_face, shelf_bottom)]),
        Tol::witness(),
    );
    assert!(
        undeclared(&found).is_empty(),
        "the notched cap has no hard finding: {found:?}"
    );
    assert!(
        found.is_empty(),
        "and it answers door 2 exactly as the seat does, with no \
         interior arm in sight: {found:?}"
    );
}

/// **Probe 3 — the seat's answers as a band-sweep row.** ε is a process
/// global read from `CAD_TOLERANCE_EPS`, so a row cannot pick its own
/// band; this one restates the seat's two answers and the binary was
/// run at `CAD_TOLERANCE_EPS` = default (1e-9), 1e-12, 1e-6 and 1e-3,
/// getting the same answers at all four — before the schedule
/// completion and, re-measured, after it. The declared seat's answer is
/// the one that moved, and it moved at every band together: the witness
/// the completed schedule lands sits ~1.9e-2 m from the nearest trim
/// boundary, three orders clear of the widest band swept. What this row
/// asserts is the answer at the three GATED bands; at 1e-3 the four
/// `pm_census_ee_parallel` escalations above are all that is left of
/// the list, and this row was red there before this change as well.
#[test]
fn r2_the_new_rows_hold_at_the_default_band() {
    let (body, post_top, shelf_bottom, _, _, _) = overhang_seat_full();
    let declared_seat = errors(&body, &records(&[(post_top, shelf_bottom)]), Tol::witness());
    assert!(
        undeclared(&declared_seat).is_empty(),
        "declared seat at the default band: {declared_seat:?}"
    );
    assert!(
        declared_seat.is_empty(),
        "and the seat certifies at the default band: {declared_seat:?}"
    );

    let bare = undeclared(&errors(&body, &ContactRecords::default(), Tol::witness()));
    assert_eq!(
        count_ef(&bare),
        3,
        "bare seat at the default band: {bare:?}"
    );
    assert_eq!(
        bare.iter()
            .filter(|c| matches!(c, CensusContact::VertexOnEdge { .. }))
            .count(),
        3,
        "bare seat at the default band: {bare:?}"
    );
}

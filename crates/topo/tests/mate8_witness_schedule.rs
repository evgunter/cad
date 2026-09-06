//! **The interior-witness rung's candidate schedule, completed** —
//! issue 1435.
//!
//! The rung rescues a declared planar pair whose region walk refused
//! `TouchingBoundary`: a point certifiably strictly inside BOTH trims
//! proves the intersection holds a disc of positive radius. Its
//! schedule used to be a fixed handful of points per trim (each outer's
//! vertex centroid and ear midpoints), and a handful of points is not a
//! search: two geometrically equivalent legal declared seats decided
//! OPPOSITELY on where those points happened to land.
//!
//! That bifurcation is what this file pins. Both seats are the same
//! class — a prism cap resting on a shelf underside, overhanging the
//! shelf's boundary edge, declared — and the only difference between
//! them is the shape of the sub-region left under the shelf:
//!
//! - `spike_overhang_seat`: the cap is mostly UNDER the shelf, so the
//!   overlap is large and convex-ish, and the shelf rectangle's own
//!   centroid falls in it. It certified before this unit and certifies
//!   after (its first candidate is unchanged).
//! - `overhang_seat`: the cap is mostly ABOVE the shelf, and what is
//!   left under it is one thin triangle of ~7.5e-3 m² — seven orders
//!   above ε, nothing undecidable about it, and missed by every one of
//!   the old schedule's 14 candidates. It parked at
//!   `CensusUnsupported`.
//!
//! Both certify now, and both certify with the declaration written
//! EITHER WAY ROUND. The order matters because the pair's chart is the
//! FIRST face's plane (`world_carrier` takes A's frame as
//! representative), so swapping the arguments re-expresses both trims
//! in a different — rotated, possibly reflected — chart. The schedule
//! is a decomposition of that chart, so the candidate points genuinely
//! differ between the two runs; what does not differ is the verdict,
//! which is the frame-invariance claim the lemma at `world_carrier`
//! makes and this row measures.
//!
//! ε posture, and the band this file is honest only within. Every
//! incidence in both seats is a shared `f64` literal (cap vertices
//! exactly on `y = 0.30`, both cap faces exactly on `z = 0.5`), so what
//! the RUNG decides are exact zeros against a ~7.5e-3 m² overlap, and
//! the witness the schedule finds in the thin seat sits ~1.9e-2 m from
//! the nearest trim boundary. The rung's own answers therefore hold at
//! every ε the matrix runs.
//!
//! **The rows below assert more than the rung, and that is where they
//! stop travelling.** They assert the seat's WHOLE validation list is
//! empty, and other predicates in that list are not so far from the
//! band: at `CAD_TOLERANCE_EPS=1e-3` the seat carries four honest
//! `pm_census_ee_parallel` escalations (margin 8.944e-3, band
//! [1e-3, 1e-2]) — the near-parallel cap and shelf edges, nothing to do
//! with the witness schedule — so these rows are RED at 1e-3 by
//! construction. 1e-3 is not a band the gate runs; the three that are
//! (default, 1e-6, 1e-12) are green. Stated because "the seat is not
//! near any band edge" would be false as a claim about the whole list,
//! and it is the whole list these rows read.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{Body, ContactRecords, FaceKey, PatchContact, ValidationError};

/// The thin seat (MATE-4a's overhang seat, verbatim): the cap crosses
/// the shelf's `y = 0.30` boundary edge at H and B and touches it at T,
/// leaving the triangle H-A-B (~7.5e-3 m²) under the shelf.
fn overhang_seat() -> (Body<f64>, FaceKey, FaceKey) {
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
    shelf_over(post)
}

/// The fat seat (`r1_mate4a_probes`'s probe fixture, verbatim): the cap
/// is mostly under the shelf with one spike overhanging, so the overlap
/// is large and the OLD schedule's first candidate already landed in
/// it.
fn spike_overhang_seat() -> (Body<f64>, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[
            (0.10, 0.10),
            (0.70, 0.10),
            (0.70, 0.20),
            (0.55, 0.30), // crossing vertex, on the shelf edge interior
            (0.45, 0.40), // spike top (overhangs)
            (0.35, 0.30), // crossing vertex, on the shelf edge interior
            (0.10, 0.20),
        ],
        0.0,
        0.5,
    );
    shelf_over(post)
}

/// The shelf both seats rest under, and the two faces of the interface.
fn shelf_over(post: common::Prism<f64>) -> (Body<f64>, FaceKey, FaceKey) {
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    (body, post.top_face, shelf_bottom)
}

fn errors(body: &Body<f64>, a: FaceKey, b: FaceKey) -> Vec<ValidationError> {
    let records = ContactRecords {
        patches: vec![PatchContact {
            face_a: a,
            face_b: b,
        }],
        ..ContactRecords::default()
    };
    match topo::validate_pseudomanifold(body, &records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

/// The bifurcation, both halves, both ways round. On the old schedule
/// the first row is empty and the second is one `CensusUnsupported` —
/// two legal seats of one class, opposite outcomes.
#[test]
fn both_halves_of_the_bifurcation_certify_either_way_round() {
    for (name, seat) in [
        ("spike (fat overlap)", spike_overhang_seat as fn() -> _),
        ("overhang (thin overlap)", overhang_seat as fn() -> _),
    ] {
        let (body, cap, shelf) = seat();
        let forward = errors(&body, cap, shelf);
        assert!(
            forward.is_empty(),
            "{name}: the declared seat certifies (cap named first): {forward:?}"
        );
        let reversed = errors(&body, shelf, cap);
        assert!(
            reversed.is_empty(),
            "{name}: and with the shelf named first, which re-expresses \
             both trims in the shelf's chart: {reversed:?}"
        );
    }
}

/// The control that the certification above is the DECLARATION's doing
/// and not a census that went quiet: undeclared, both seats stay loud.
#[test]
fn both_seats_bare_are_still_loud() {
    for (name, seat) in [
        ("spike", spike_overhang_seat as fn() -> _),
        ("overhang", overhang_seat as fn() -> _),
    ] {
        let (body, _, _) = seat();
        let found = match topo::validate_pseudomanifold(
            &body,
            &ContactRecords::default(),
            Tol::witness(),
        ) {
            Ok(()) => Vec::new(),
            Err(e) => e,
        };
        // Pin the KIND, not merely the count: "some error" would stay
        // green if the census fell silent and an unrelated finding took
        // its place, which is exactly the failure this control exists
        // to catch. Undeclared, the cap resting on the shelf underside
        // must still be reported as an unattributed contact.
        let contacts: Vec<_> = found
            .iter()
            .filter(|e| matches!(e, ValidationError::UndeclaredContact { .. }))
            .collect();
        assert!(
            !contacts.is_empty(),
            "{name}: undeclared, the seat refuses with hard contact \
             findings: {found:?}"
        );
    }
}

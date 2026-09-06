//! R1 review probes for MATE-4a (PR #1432). NOT part of the unit.
//!
//! Probe 1 (`a_spike_overhang_certifies_outright`): attacks the PR's
//! structural sentence "the new arm fires only when a vertex sits on
//! both declared faces' boundaries, which is exactly what the
//! chart-region predicate answers TouchingBoundary to ... parks it on
//! the chart predicate's touching posture." A differently-shaped
//! overhang — the cap mostly UNDER the shelf, a small spike
//! overhanging — still reaches `ef_bound_backed`'s interior arm (the
//! shelf edge dives through the cap between two cap vertices resting
//! on its interior), but the declared pair's overlap is large enough
//! that the #1063 interior-witness schedule's very first candidate
//! proves PositiveArea, so door 2 CERTIFIES. Measured, because the
//! order is easy to get backwards: stage 1 walks `[uv_a.outer,
//! uv_b.outer]`, so FACE A's five candidates all precede face B's, and
//! the winner here is face A — the CAP — on its candidate #1, its own
//! vertex centroid, which in the carrier chart sits at
//! (0.0, -1.586e-17) because a plane description's origin is the face's
//! centroid. The shelf rectangle's candidates are never reached.
//! So: the class's outcome bifurcated on the fixed witness
//! schedule; `TouchingBoundary`→`CensusUnsupported` was this fixture's
//! outcome, not the class's. This row is unmoved by the schedule's
//! completion (issue 1435) precisely because it never needed it: the
//! seat it bifurcated against certifies now too, and
//! `mate8_witness_schedule` runs the two together.
//!
//! Probe 2 (`the_spike_seat_bare_is_loud`): the same seat undeclared
//! stays loud (the rung consults declarations) — the control that
//! probe 1's empty list is the declaration's doing.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{Body, ContactRecords, FaceKey, PatchContact, ValidationError};

/// Cap mostly under the shelf (`y <= 0.30`), one spike overhanging
/// through the shelf's `y = 0.30` boundary edge between the two
/// crossing vertices (0.35, 0.30) and (0.55, 0.30), both on the shelf
/// edge's interior.
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

fn errors(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

#[test]
fn a_spike_overhang_certifies_outright() {
    let (body, post_top, shelf_bottom) = spike_overhang_seat();
    let found = errors(
        &body,
        &ContactRecords {
            patches: vec![PatchContact {
                face_a: post_top,
                face_b: shelf_bottom,
            }],
            ..ContactRecords::default()
        },
    );
    assert!(
        found.is_empty(),
        "a member of the same class certifies Ok — no TouchingBoundary \
         parking, no CensusUnsupported: {found:?}"
    );
}

#[test]
fn the_spike_seat_bare_is_loud() {
    let (body, _, _) = spike_overhang_seat();
    let found = errors(&body, &ContactRecords::default());
    assert!(
        !found.is_empty(),
        "undeclared, the seat must refuse: {found:?}"
    );
}

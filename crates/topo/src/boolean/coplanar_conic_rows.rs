//! **A conic edge lying in a plane face's plane, at the sweep.**
//!
//! The conic × plane lane's parallel frame has two cases: the conic in
//! the face's plane, whose endpoints take the line lane's `(Zero, Zero)`
//! posture through `vertex_on_face`, and the conic in a parallel plane
//! off it, a certified miss. The rows hold both, and the refusal
//! between them, on the vertex no neighbour evidences: a valence-2
//! vertex between two arcs of one rim, which only this lane sees.
//!
//! The operand is a half cylinder-wall sheet (`u ∈ [0, π]`,
//! `z ∈ [−1, 1]`) with its top rim split at its midpoint `(0, 1, 1)`;
//! the partner is a brick whose top face stands over that vertex and
//! clear of both struts. `sweep_direction` runs directly, so the rows
//! read the contact records the sweep leaves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::{ContactAcc, SweepKnobs, SweepStrategy, sweep_direction};
use crate::boolean::{BooleanError, ContactRecords, DeclaredPairs, Operand, VfContact};
use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet};
use crate::{Body, FaceKey, VertexKey};
use geom::Curve3;
use geom_core::{Band, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The sheet with its top rim split, and the valence-2 vertex.
fn split_sheet() -> (Body<f64>, VertexKey) {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.0, core::f64::consts::PI),
        (-1.0, 1.0),
        tol,
    );
    let rim = body
        .edges()
        .map(|(k, _)| k)
        .find(|&e| {
            let curve = body.get_edge(e).unwrap().curve;
            match body.get_curve_geom(curve).unwrap() {
                crate::CurveGeom::Certified(c) => {
                    matches!(c.carrier(), Curve3::Circle { center, .. } if center.z > 0.5)
                }
                _ => false,
            }
        })
        .expect("the sheet has a top rim");
    let (t0, t1) = match body.get_curve_geom(body.get_edge(rim).unwrap().curve) {
        Some(crate::CurveGeom::Certified(c)) => c.params(),
        _ => unreachable!(),
    };
    let w = body.split_edge(rim, 0.5 * (t0 + t1), tol).unwrap().vertex;
    // The fixture's premise: `w` sits at `(0, 1, 1)`, and its only
    // edges are the two halves of the rim.
    let pw = *body.get_point(body.get_vertex(w).unwrap().point).unwrap();
    assert!(
        (pw - geom_core::Point3::new(0.0, 1.0, 1.0)).norm() < 1e-12,
        "the split vertex is the rim's midpoint: {pw:?}"
    );
    let edges = body.edges_of_vertex(w).unwrap();
    assert_eq!(edges.len(), 2, "the split vertex has valence 2");
    for e in edges {
        let curve = body.get_edge(e).unwrap().curve;
        assert!(
            matches!(
                body.get_curve_geom(curve),
                Some(crate::CurveGeom::Certified(c)) if matches!(c.carrier(), Curve3::Circle { .. })
            ),
            "every edge at the split vertex is a rim arc"
        );
    }
    (body, w)
}

/// A brick over `x ∈ [−0.3, 0.3]`, `y ∈ [0.7, 1.3]`, `z ∈ [0, top]`, and
/// its top face.
fn brick_under(top: f64) -> (Body<f64>, FaceKey) {
    let b: Body<f64> = brick((-0.3, 0.3), (0.7, 1.3), (0.0, top), Tol::witness());
    let g = b
        .faces()
        .map(|(k, _)| k)
        .find(|&f| {
            super::face_plane(&b, f).is_some_and(|p| {
                p.normal.z > 0.5 && (p.origin.z - top).abs() < 1e-12
            })
        })
        .expect("the brick has a top face");
    (b, g)
}

fn sweep(x: &Body<f64>, y: &Body<f64>) -> Result<ContactRecords, BooleanError> {
    let (mut x, mut y) = (x.clone(), y.clone());
    let mut acc = ContactAcc::default();
    sweep_direction(
        &mut x,
        &mut y,
        Operand::A,
        &DeclaredPairs::default(),
        &mut acc,
        band(),
        SweepStrategy::Realized,
        &SweepKnobs::default(),
        None,
        Tol::witness(),
    )?;
    Ok(acc.finish())
}

/// **The rim in the face's plane records its valence-2 vertex.** No
/// other edge reaches `w`: the struts stand clear of the brick, and the
/// brick's side faces meet the rim at `x = ±0.3`, not at `w`. Before
/// the lane gave the parallel frame endpoint treatment, this contact
/// was never recorded and nothing refused.
///
/// Mutant: the arm's in-plane branch back to `continue` leaves
/// `a_on_b` without `(w, G)`.
#[test]
fn a_rim_arc_in_the_face_plane_records_its_valence_two_vertex() {
    let (a, w) = split_sheet();
    let (b, g) = brick_under(1.0);
    let contacts = sweep(&a, &b).expect("the sweep runs");
    assert!(
        contacts.a_on_b.contains(&VfContact { vertex: w, face: g }),
        "the valence-2 vertex is recorded on the top face: {:?}",
        contacts.a_on_b
    );
}

/// **The rim in a parallel plane off the face's is a miss.** The same
/// brick lowered by 0.1: the rim runs 0.1 above its top face, and the
/// sweep records no contact of `A`'s at all (the side faces place both
/// rim roots `Out`, above the brick).
///
/// Mutant: the offset decision dropped (every parallel frame taken as
/// in the plane) records `w` against the top face it does not touch.
#[test]
fn a_rim_arc_in_a_parallel_plane_off_the_face_is_a_miss() {
    let (a, _) = split_sheet();
    let (b, _) = brick_under(0.9);
    let contacts = sweep(&a, &b).expect("the sweep runs");
    assert!(
        contacts.a_on_b.is_empty() && contacts.vv.is_empty(),
        "no contact: a_on_b {:?}, vv {:?}",
        contacts.a_on_b,
        contacts.vv
    );
}

/// **An offset the band cannot decide refuses, named.** The brick's
/// top raised into the ambiguity band above the rim.
///
/// Mutant: the offset's undecided arm read as a miss answers `Ok`.
#[test]
fn a_rim_arc_at_an_undecided_offset_refuses_typed() {
    let (a, _) = split_sheet();
    let b_ = band();
    let (b, _) = brick_under(1.0 + 0.5 * (b_.zero() + b_.escalate()));
    match sweep(&a, &b) {
        Err(BooleanError::Escalated { diag }) => assert_eq!(
            diag.predicate,
            Some("bool_conic_face_plane_offset"),
            "the refusal names the offset: {diag:?}"
        ),
        other => panic!("expected the offset's escalation, got {other:?}"),
    }
}

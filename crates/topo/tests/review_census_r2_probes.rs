//! Review probes (PR 5605, R2): a confirmed planar face pair answers
//! for a touch of its faces' boundaries only inside the closure of
//! their overlap; a touch elsewhere — on a non-convex face, away from
//! the overlap — is decided as its own pair.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use std::collections::BTreeMap;
use topo::{Body, ContactRecords, SolidKey, ValidationError};

fn census(body: &Body<f64>) -> impl FnMut(&ContactRecords) -> Vec<ValidationError> + '_ {
    |records| {
        topo::validate_pseudomanifold(body, records, Tol::witness())
            .err()
            .unwrap_or_default()
    }
}

fn by_solid(body: &Body<f64>) -> BTreeMap<SolidKey, u32> {
    body.solids().map(|(s, _)| s).zip(0..).collect()
}

fn two(
    base: &[(f64, f64)],
    (z0, z1): (f64, f64),
    top: &[(f64, f64)],
    (w0, w1): (f64, f64),
) -> Body<f64> {
    let mut body = common::prism_z::<f64>(base, z0, z1, Tol::witness()).body;
    let t = common::prism_z::<f64>(top, w0, w1, Tol::witness()).body;
    topo::graft_disjoint_all_keyed(&mut body, &t).unwrap();
    body
}

/// (face rows, other rows, findings, flush rows, flush findings)
fn read(body: &Body<f64>) -> (usize, Vec<String>, Vec<ValidationError>, usize, usize) {
    let groups = by_solid(body);
    let recorded = topo::record_at_rest(
        body,
        &ContactRecords::default(),
        census(body),
        &|s| groups.get(&s).copied(),
        0,
    );
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let flush =
        <f64 as topo::AtRestPolicy>::flush_at_rest(body, &|s| groups.get(&s).copied(), band);
    let is_face = |c: &topo::RowCell| {
        matches!(
            c,
            topo::RowCell::Result {
                cell: topo::Cell::Face(_)
            }
        )
    };
    let faces = recorded
        .rows
        .iter()
        .filter(|r| r.cells.iter().all(is_face))
        .count();
    let other: Vec<String> = recorded
        .rows
        .iter()
        .filter(|r| !r.cells.iter().all(is_face))
        .map(|r| format!("{:?}", r.cells))
        .collect();
    if recorded.findings.is_empty() {
        assert_eq!(
            topo::validate_pseudomanifold(body, &recorded.records, Tol::witness()),
            Ok(()),
            "the records confirm"
        );
    }
    (
        faces,
        other,
        recorded.findings,
        flush.rows.len(),
        flush.findings.len(),
    )
}

/// A U: x ∈ [0, 3], y ∈ [0, 2], notch x ∈ [1, 2], y ∈ [1, 2].
const U: [(f64, f64); 8] = [
    (0.0, 0.0),
    (3.0, 0.0),
    (3.0, 2.0),
    (2.0, 2.0),
    (2.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
];

/// The block's bottom covers part of the left prong and its right
/// bottom edge lies on the right prong's notch edge x = 2, across the
/// notch from the overlap: a collinear edge touch outside the closure.
#[test]
fn r2_collinear_touch_across_a_notch_is_its_own_pair() {
    let block = [(0.2, 1.2), (2.0, 1.2), (2.0, 1.8), (0.2, 1.8)];
    let body = two(&U, (0.0, 1.0), &block, (1.0, 2.0));
    let (faces, other, findings, _, _) = read(&body);
    assert!(findings.is_empty(), "{findings:?}");
    assert_eq!(faces, 1);
    assert!(!other.is_empty(), "the touch at x = 2 is its own row");
}

/// A pentagon whose tip (2, 1.5) rests on the notch edge x = 2.
#[test]
fn r2_tip_on_an_edge_across_a_notch_is_its_own_pair() {
    let block = [(0.2, 1.2), (1.5, 1.2), (2.0, 1.5), (1.5, 1.8), (0.2, 1.8)];
    let body = two(&U, (0.0, 1.0), &block, (1.0, 2.0));
    let (faces, other, findings, _, _) = read(&body);
    assert!(findings.is_empty(), "{findings:?}");
    assert_eq!(faces, 1);
    assert_eq!(
        other.len(),
        1,
        "the tip is one vertex-on-edge row: {other:?}"
    );
}

/// A quadrilateral whose corner (2, 2) is the right prong's corner.
#[test]
fn r2_corner_on_a_corner_across_a_notch_is_its_own_pair() {
    let block = [(0.2, 1.2), (1.5, 1.2), (2.0, 2.0), (0.2, 2.0)];
    let body = two(&U, (0.0, 1.0), &block, (1.0, 2.0));
    let (faces, other, findings, _, _) = read(&body);
    assert!(findings.is_empty(), "{findings:?}");
    assert_eq!(faces, 1);
    assert_eq!(
        other.len(),
        1,
        "the corner is one vertex-vertex row: {other:?}"
    );
}

/// An L-shaped overlap (block over an L's reflex corner): every touch
/// is on the overlap's closure, so one face pair and nothing else.
#[test]
fn r2_an_l_shaped_overlap_is_one_face_pair() {
    let l = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let block = [(0.5, 0.5), (1.5, 0.5), (1.5, 1.5), (0.5, 1.5)];
    let body = two(&l, (0.0, 1.0), &block, (1.0, 2.0));
    let (faces, other, findings, _, _) = read(&body);
    assert!(findings.is_empty(), "{findings:?}");
    assert_eq!(faces, 1);
    assert!(other.is_empty(), "{other:?}");
}

/// The block over the L's reflex corner with its corner exactly at it:
/// the overlap [0.5, 1] × [0.5, 1] meets the L's reflex vertex at its
/// own corner. Every touch is on the closure.
#[test]
fn r2_touch_exactly_at_the_overlap_corner() {
    let l = [
        (0.0, 0.0),
        (2.0, 0.0),
        (2.0, 1.0),
        (1.0, 1.0),
        (1.0, 2.0),
        (0.0, 2.0),
    ];
    let block = [(0.5, 0.5), (1.0, 0.5), (1.0, 1.0), (0.5, 1.0)];
    let body = two(&l, (0.0, 1.0), &block, (1.0, 2.0));
    let (faces, other, findings, _, _) = read(&body);
    assert!(findings.is_empty(), "{findings:?}");
    assert_eq!(faces, 1);
    assert!(other.is_empty(), "{other:?}");
}

/// Two faces of one carrier meeting at one corner point only: no pair.
#[test]
fn r2_faces_meeting_at_a_point_are_no_face_pair() {
    let base = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let block = [(2.0, 2.0), (3.0, 2.0), (3.0, 3.0), (2.0, 3.0)];
    let body = two(&base, (0.0, 1.0), &block, (1.0, 2.0));
    let (faces, other, findings, flush, _) = read(&body);
    assert!(findings.is_empty(), "{findings:?}");
    assert_eq!(faces, 0);
    assert_eq!(other.len(), 1, "{other:?}");
    assert_eq!(flush, 0);
}

/// A block on the base with one side collinear across the corner: the
/// front faces share a segment, a flush pair.
#[test]
fn r2_an_overhanging_flush_front_is_one_flush_row() {
    let base = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)];
    let block = [(1.0, 0.0), (3.0, 0.0), (3.0, 1.0), (1.0, 1.0)];
    let body = two(&base, (0.0, 1.0), &block, (1.0, 2.0));
    let (faces, other, findings, flush, ff) = read(&body);
    assert!(findings.is_empty(), "{findings:?}");
    assert_eq!(faces, 1);
    assert!(other.is_empty(), "{other:?}");
    assert_eq!((flush, ff), (1, 0));
}

/// Two blocks overlapping by half their height, side walls on one
/// plane facing one way and overlapping over an AREA: no flush pair
/// (interiors meet). Red if a flush row is minted for them.
#[test]
fn r2_side_walls_overlapping_in_area_are_no_flush_row() {
    let sq = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let body = two(&sq, (0.0, 1.0), &sq, (0.5, 1.5));
    let (_, _, _, flush, ff) = read(&body);
    assert_eq!((flush, ff), (0, 0));
}

/// The heat sink's shape: a fin sunk 1/16 into the base, beside the
/// same fin as the union's (here a block on the base's top), each its
/// own solid.
#[test]
fn r2_a_sunk_fin_beside_its_union_twin_is_no_flush_row() {
    let fin = [
        (0.25, 0.125),
        (0.4375, 0.125),
        (0.4375, 0.875),
        (0.25, 0.875),
    ];
    let body = two(&fin, (0.25, 1.0), &fin, (0.1875, 1.0));
    let (_, _, _, flush, ff) = read(&body);
    assert_eq!((flush, ff), (0, 0));
}

/// The heat sink itself, in miniature: the base ∪ a sunk fin, as one
/// solid, beside the same fin as its own solid (the pattern's copy).
/// The fin's walls and the union's fin walls lie on one plane facing
/// one way and overlap over an area above the base.
#[test]
fn r2_the_heat_sink_union_beside_its_pattern_fin_mints_no_flush_row() {
    let tol = Tol::witness();
    let base = common::finished(
        "base",
        common::prism_z::<f64>(
            &[(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)],
            0.0,
            0.25,
            tol,
        )
        .body,
        tol,
    );
    let fin_p = [
        (0.25, 0.125),
        (0.4375, 0.125),
        (0.4375, 0.875),
        (0.25, 0.875),
    ];
    let fin = common::finished(
        "fin",
        common::prism_z::<f64>(&fin_p, 0.1875, 1.0, tol).body,
        tol,
    );
    let topo::BooleanResult::Body(out) = topo::union(&base, &fin, tol).unwrap() else {
        panic!("a body");
    };
    let mut body: Body<f64> = (*out.body).clone();
    let twin = common::prism_z::<f64>(&fin_p, 0.1875, 1.0, tol).body;
    topo::graft_disjoint_all_keyed(&mut body, &twin).unwrap();
    let (_, _, _, flush, ff) = read(&body);
    assert_eq!((flush, ff), (0, 0));
}

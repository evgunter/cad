//! Review 4740 (R1) probes: the reduction's edge-edge row for a touch
//! along a line, on op arms and overlap shapes the PR's own rows do not
//! draw — an intersection's touch, partial and contained overlaps, and
//! the no-touch exits where the result keeps one edge only.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, prism_z};
use geom_core::Tol;
use topo::{
    AtRestBody, BooleanBody, BooleanError, BooleanResult, intersect, subtract, union,
    validate_pseudomanifold,
};

type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, Tol) -> Result<BooleanResult<f64>, BooleanError>;

fn tol() -> Tol {
    Tol::witness()
}

fn block(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished("block", brick(x, y, z, tol()), tol())
}

fn prism(p: &[(f64, f64)], z: (f64, f64)) -> AtRestBody<f64> {
    finished("prism", prism_z(p, z.0, z.1, tol()).body, tol())
}

fn run(op: Op, a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> Option<BooleanBody<f64>> {
    match op(a, b, tol()).expect("the op builds") {
        BooleanResult::Body(r) => Some(r),
        BooleanResult::Empty => None,
    }
}

/// The result's edge-edge row count, after asserting it passes 3′ with
/// its own records; and how many `EdgeEdgeOverlap` refusals stripping
/// those rows leaves, every other refusal failing the row.
fn ee_rows(what: &str, r: &BooleanBody<f64>) -> (usize, usize) {
    let verdict = validate_pseudomanifold(&r.body, &r.contacts, tol());
    assert!(
        verdict.is_ok(),
        "{what}: 3′ with its records: {verdict:?}\n{:?}",
        r.contacts
    );
    let bounds: topo::ContactRecords = r
        .contacts
        .rows()
        .iter()
        .filter(|c| !matches!(c.record.kind(), topo::ContactKind::EdgeEdge(..)))
        .cloned()
        .collect();
    let errs = validate_pseudomanifold(&r.body, &bounds, tol())
        .err()
        .unwrap_or_default();
    assert!(
        errs.iter().all(|e| matches!(
            e,
            topo::ValidationError::UnrecordedContact {
                contact: topo::CensusContact::EdgeEdgeOverlap { .. },
                ..
            }
        )),
        "{what}: stripped: {errs:?}"
    );
    (r.contacts.edge_edge().count(), errs.len())
}

/// Asserts `got` is the result gate's refusal of overlapping wedges on
/// the z-axis over (0, 1).
fn refuses_overlap_on_the_axis(what: &str, got: Result<BooleanResult<f64>, BooleanError>) {
    match &got {
        Err(BooleanError::LineWedges {
            fault: topo::LineWedgeFault::Overlap { .. },
            from,
            to,
        }) => {
            let mut ends = [from.as_str(), to.as_str()];
            ends.sort_unstable();
            assert_eq!(
                ends,
                ["(0.0, 0.0, 0.0)", "(0.0, 0.0, 1.0)"],
                "{what}: the line"
            );
        }
        other => panic!("{what}: {other:?}, not the overlap's refusal"),
    }
}

/// A at the origin's reflex corner (the quadrant x>0, y>0 cut away),
/// B a larger reflex block (the quadrant x<0, y<0 cut away): A ∩ B is
/// two quarter blocks touching along the z-axis over (0, 1), each a 90°
/// wedge there. The join pairs each operand's two faces into an edge, so
/// the result carries the axis as two edges of 270° each, overlapping
/// by 180° (every tier passed it); the result gate refuses, in both
/// orders, until the re-pairing lands
/// (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).
#[test]
fn an_intersection_of_two_reflex_blocks_refuses_its_overlapping_wedges() {
    let a = prism(
        &[
            (-1.0, -1.0),
            (1.0, -1.0),
            (1.0, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
            (-1.0, 1.0),
        ],
        (0.0, 1.0),
    );
    let b = prism(
        &[
            (0.0, -2.0),
            (2.0, -2.0),
            (2.0, 2.0),
            (-2.0, 2.0),
            (-2.0, 0.0),
            (0.0, 0.0),
        ],
        (-1.0, 2.0),
    );
    for (what, x, y) in [("a ∩ b", &a, &b), ("b ∩ a", &b, &a)] {
        refuses_overlap_on_the_axis(what, intersect(x, y, tol()));
    }
}

/// The same two quarter blocks as a subtraction of the cut-away
/// quadrant from A: B's convex wedge inside A's reflex one, and the same
/// two 270° edges, refused.
#[test]
fn a_subtraction_leaving_two_quarters_refuses_its_overlapping_wedges() {
    let a = prism(
        &[
            (-1.0, -1.0),
            (1.0, -1.0),
            (1.0, 0.0),
            (0.0, 0.0),
            (0.0, 1.0),
            (-1.0, 1.0),
        ],
        (0.0, 1.0),
    );
    let q = block((-2.0, 0.0), (-2.0, 0.0), (-1.0, 2.0));
    refuses_overlap_on_the_axis("a − q", subtract(&a, &q, tol()));
}

/// Two bricks kissing along an edge over part of it only: B's edge
/// overhangs A's at one end, or lies strictly within it.
#[test]
fn a_partial_edge_kiss_records_its_touch() {
    let a = block((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    for z in [(0.5, 1.5), (0.25, 0.75), (-0.5, 1.5)] {
        let b = block((1.0, 2.0), (1.0, 2.0), z);
        for (what, x, y) in [("a ∪ b", &a, &b), ("b ∪ a", &b, &a)] {
            let r = run(union, x, y).expect("non-empty");
            assert_eq!(
                ee_rows(&format!("{what} {z:?}"), &r),
                (1, 1),
                "{what} {z:?}"
            );
        }
    }
}

/// The no-touch exits: each result keeps one solid's edge along the
/// line and no other, so no edge-edge row: subtracting a kissing
/// brick, a union with a wedge nested in the cube's corner, and an
/// intersection with a brick kissing from outside. Subtracting the
/// nested wedge leaves two wedges on the line, which the result gate
/// refuses (the re-pairing is not built).
#[test]
fn a_kiss_the_op_drops_records_no_edge_edge_row() {
    let a = block((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let kiss = block((1.0, 2.0), (1.0, 2.0), (0.0, 1.0));
    let r = run(subtract, &a, &kiss).expect("non-empty");
    assert_eq!(ee_rows("a − kiss", &r).0, 0);
    let r = run(subtract, &kiss, &a).expect("non-empty");
    assert_eq!(ee_rows("kiss − a", &r).0, 0);
    // A wedge 200°–250° about the cube's corner edge (1, 1, z), strictly
    // inside the cube's 180°–270° wedge there.
    let at = |deg: f64| {
        (
            1.0 + deg.to_radians().cos() * 0.5,
            1.0 + deg.to_radians().sin() * 0.5,
        )
    };
    let nested = prism(&[(1.0, 1.0), at(200.0), at(250.0)], (0.0, 1.0));
    let r = run(union, &a, &nested).expect("non-empty");
    assert_eq!(ee_rows("a ∪ nested", &r).0, 0);
    let r = run(intersect, &a, &nested).expect("non-empty");
    assert_eq!(ee_rows("a ∩ nested", &r).0, 0);
    // `a − nested` leaves a pinch of two 20° wedges; the result carries
    // them as edges of 90° and 310°, overlapping by 40°, and refuses.
    match subtract(&a, &nested, tol()) {
        Err(BooleanError::LineWedges {
            fault: topo::LineWedgeFault::Overlap { .. },
            ..
        }) => {}
        other => panic!("a − nested: {other:?}, not the overlap's refusal"),
    }
}

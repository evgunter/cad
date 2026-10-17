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

/// A at the origin's reflex corner (the quadrant x>0, y>0 cut away),
/// B a larger reflex block (the quadrant x<0, y<0 cut away): A ∩ B is
/// two quarter blocks touching along the z-axis over (0, 1) — the
/// intersection's touch, both wedges reflex, each one's flankers
/// inside the other.
#[test]
fn an_intersection_of_two_reflex_blocks_records_its_touch() {
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
        let r = run(intersect, x, y).expect("non-empty");
        let v = topo::mass_properties(&r.body, tol()).unwrap().volume;
        assert!((v - 2.0).abs() < 1e-9, "{what}: volume {v}");
        assert_eq!(ee_rows(what, &r).0, 1, "{what}");
    }
}

/// The same two quarter blocks as a subtraction of the cut-away
/// quadrant from A: B's convex wedge inside A's reflex one.
#[test]
fn a_subtraction_leaving_two_quarters_records_its_touch() {
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
    let r = run(subtract, &a, &q).expect("non-empty");
    assert_eq!(ee_rows("a − q", &r).0, 1);
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
/// intersection with a brick kissing from outside.
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
    let r = run(subtract, &a, &nested).expect("non-empty");
    assert_eq!(
        ee_rows("a − nested", &r).0,
        1,
        "a − nested leaves a pinch of two wedges"
    );
}

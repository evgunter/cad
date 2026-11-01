//! Boolean results whose pieces pinch at one operand vertex: on these
//! poses the op's copies of that vertex do not reach rest as two
//! touching vertices, and the result passes the pseudomanifold door with
//! its own records (f64 and Interval). The pinches along one operand
//! edge refuse at the result gate: the result's two edges on the line
//! bound overlapping wedges.
//!
//! That is not universal: a pinch is one vertex per cone, so a two-run
//! pierce whose copies lie in two cones keeps them apart on their one
//! point key.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, prism_z};
use geom_core::{Decide, Tol};
use topo::{
    AtRestBody, Body, BooleanBody, BooleanError, BooleanResult, subtract, union, validate_closed,
    validate_pseudomanifold,
};

type BoolOp<T> = fn(&AtRestBody<T>, &AtRestBody<T>, Tol) -> Result<BooleanResult<T>, BooleanError>;

fn run<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    op: BoolOp<T>,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
) -> BooleanBody<T> {
    match op(a, b, Tol::witness()).unwrap() {
        BooleanResult::Body(body) => body,
        BooleanResult::Empty => panic!("expected a non-empty boolean result"),
    }
}

/// A brick at rest.
fn block<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    what: &str,
    x: (f64, f64),
    y: (f64, f64),
    z: (f64, f64),
) -> AtRestBody<T> {
    finished(what, brick::<T>(x, y, z, Tol::witness()), Tol::witness())
}

/// The wedge whose edge runs along the cube's convex edge `(1, 1, z)`,
/// from `z0` up.
fn wedge<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(z0: f64) -> AtRestBody<T> {
    let profile = [(1.0, 1.0), (-1.0, 0.2), (0.2, -1.0)];
    let body = prism_z::<T>(&profile, z0, 2.0, Tol::witness()).body;
    finished("wedge", body, Tol::witness())
}

/// The pinch results, by name.
fn pinches<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>()
-> Vec<(&'static str, BooleanBody<T>)> {
    let t = Tol::witness();
    let ell = prism_z::<T>(
        &[
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 2.0),
            (0.0, 2.0),
        ],
        0.0,
        1.0,
        t,
    )
    .body;
    let ell = finished("ell", ell, t);
    vec![
        // A cube kissing the L's reflex edge at its top vertex.
        (
            "reflex_vertex_kiss",
            run(
                union,
                &ell,
                &block("kissing cube", (1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
            ),
        ),
    ]
}

/// **The pinches along one operand edge refuse at the result gate.**
/// Cutting the notch's reflex edge `(1, 1, z)` by a second notch leaves
/// two quarters pinched along it, each a 90° wedge there; a wedge whose
/// edge runs along the cube's convex edge leaves two 21.8° wedges. The
/// results carry the line as two edges of 270° each (overlapping by
/// 180°) and of 90° and 313.6° (overlapping by 43.6°), every tier
/// passing them, so the gate refuses each, over the whole edge and over
/// `z ∈ [0.5, 1]` only. They return with the re-pairing
/// (`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).
fn assert_edge_pinches_refuse<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>() {
    let t = Tol::witness();
    let notched = run(
        subtract,
        &block::<T>("slab", (0.0, 2.0), (0.0, 2.0), (0.0, 1.0)),
        &block("first notch", (1.0, 3.0), (-1.0, 1.0), (-1.0, 2.0)),
    )
    .body;
    let cube = block::<T>("cube", (0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    for (name, got) in [
        (
            "reflex_edge_split",
            subtract(
                &notched,
                &block("second notch", (-1.0, 1.0), (1.0, 3.0), (-1.0, 2.0)),
                t,
            ),
        ),
        (
            "reflex_edge_split_upper",
            subtract(
                &notched,
                &block("upper notch", (-1.0, 1.0), (1.0, 3.0), (0.5, 2.0)),
                t,
            ),
        ),
        ("wedge_on_edge", subtract(&cube, &wedge(-1.0), t)),
        ("wedge_on_edge_upper", subtract(&cube, &wedge(0.5), t)),
    ] {
        assert!(
            matches!(
                got,
                Err(BooleanError::LineWedges {
                    fault: topo::LineWedgeFault::Overlap { .. },
                    ..
                })
            ),
            "{name}: {got:?}, not the overlap's refusal"
        );
    }
}

#[test]
fn pinches_along_an_operand_edge_refuse_their_overlapping_wedges() {
    assert_edge_pinches_refuse::<f64>();
}

#[test]
fn pinches_along_an_operand_edge_refuse_their_overlapping_wedges_interval() {
    assert_edge_pinches_refuse::<geom_core::Interval>();
}

/// Each pinch passes tiers 2 and 3′ with its own records, and records
/// each touch along a line: the kissing cube's two bottom edges lie
/// along the L's notch rim. Red if the reduction writes the edge-edge
/// row only where both wedges read Out.
fn assert_pinches_pass<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>() {
    for (name, r) in pinches::<T>() {
        let lines = 2;
        assert_eq!(
            r.contacts.edge_edge().count(),
            lines,
            "{name}: edge-edge rows: {:?}",
            r.contacts
        );
        assert_eq!(validate_closed(&r.body), Ok(()), "{name}: tier 2");
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()),
            Ok(()),
            "{name}: tier 3′ with the result's own records"
        );
    }
}

/// At each pinch, one vertex per point: the copies were fused or
/// killed, or (where two operands meet there) recorded, and then on one
/// point key with the vertex their record ties them to.
#[test]
fn pinch_copies_do_not_reach_rest() {
    assert_pinches_pass::<f64>();
    let at = |b: &Body<f64>, (x, y, z)| {
        b.vertex_points()
            .filter(|(_, p)| {
                let p = *p;
                (p.x, p.y, p.z) == (x, y, z)
            })
            .count()
    };
    for (name, r) in pinches::<f64>() {
        let recorded: slotmap::SecondaryMap<topo::VertexKey, ()> = r
            .contacts
            .vertex_vertex()
            .flat_map(|(a, b)| [(a, ()), (b, ())])
            .collect();
        let mut on_key: slotmap::SecondaryMap<topo::PointKey, Vec<_>> =
            slotmap::SecondaryMap::new();
        for (k, v) in r.body.vertices() {
            on_key
                .entry(v.point)
                .expect("a live point key")
                .or_default()
                .push(k);
        }
        let copies: Vec<_> = on_key
            .values()
            .filter(|vs| vs.len() > 1)
            .flatten()
            .filter(|&&k| !recorded.contains_key(k))
            .collect();
        assert_eq!(
            copies,
            Vec::<&topo::VertexKey>::new(),
            "{name}: no two vertices on one point key but those a record ties"
        );
        // Cross-operand: the L's vertex and the cube's, declared.
        let expected = 2;
        assert_eq!(
            at(&r.body, (1.0, 1.0, 1.0)),
            expected,
            "{name}: at (1, 1, 1)"
        );
        assert_eq!(at(&r.body, (1.0, 1.0, 0.0)), 1, "{name}: at (1, 1, 0)");
    }
}

#[test]
fn pinch_copies_do_not_reach_rest_interval() {
    assert_pinches_pass::<geom_core::Interval>();
}

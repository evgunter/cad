//! Boolean results whose pieces pinch along one operand edge or at
//! one operand vertex: the op's copies of that vertex do not reach
//! rest as two touching vertices, and the result passes the
//! pseudomanifold door with its own records (f64 and Interval).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, prism_z};
use geom_core::{Decide, Tol};
use topo::{
    AtRestBody, Body, BooleanBody, BooleanError, BooleanResult, subtract_with, union_with,
    validate_closed, validate_pseudomanifold,
};

type BoolOp<T> = fn(
    &AtRestBody<T>,
    &AtRestBody<T>,
    &topo::BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<T>, BooleanError>;

fn run<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>(
    op: BoolOp<T>,
    a: &AtRestBody<T>,
    b: &AtRestBody<T>,
) -> BooleanBody<T> {
    match op(
        a,
        b,
        &common::flush_declarations(a, b, Tol::witness()),
        Tol::witness(),
    )
    .unwrap()
    {
        BooleanResult::Body(body) => body,
        BooleanResult::Empty => panic!("expected a non-empty boolean result"),
    }
}

/// The pinch results, by name.
fn pinches<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>()
-> Vec<(&'static str, BooleanBody<T>)> {
    let t = Tol::witness();
    let block = |what, x, y, z| finished(what, brick::<T>(x, y, z, t), t);
    let slab = block("slab", (0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let notched = run(
        subtract_with,
        &slab,
        &block("first notch", (1.0, 3.0), (-1.0, 1.0), (-1.0, 2.0)),
    );
    let cube = block("cube", (0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let wedge = |z0| {
        finished(
            "wedge",
            prism_z::<T>(&[(1.0, 1.0), (-1.0, 0.2), (0.2, -1.0)], z0, 2.0, t).body,
            t,
        )
    };
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
        // The notch's reflex edge (1, 1, z) is cut by a second notch:
        // two quarters pinched along it.
        (
            "reflex_edge_split",
            run(
                subtract_with,
                &notched.body,
                &block("second notch", (-1.0, 1.0), (1.0, 3.0), (-1.0, 2.0)),
            ),
        ),
        // The same over z in [0.5, 1] only.
        (
            "reflex_edge_split_upper",
            run(
                subtract_with,
                &notched.body,
                &block("upper notch", (-1.0, 1.0), (1.0, 3.0), (0.5, 2.0)),
            ),
        ),
        // A wedge whose edge runs along the cube's convex edge (1, 1, z).
        ("wedge_on_edge", run(subtract_with, &cube, &wedge(-1.0))),
        (
            "wedge_on_edge_upper",
            run(subtract_with, &cube, &wedge(0.5)),
        ),
        // A cube kissing the L's reflex edge at its top vertex.
        (
            "reflex_vertex_kiss",
            run(
                union_with,
                &ell,
                &block("kissing cube", (1.0, 2.0), (1.0, 2.0), (1.0, 2.0)),
            ),
        ),
    ]
}

fn assert_pinches_pass<T: Decide + geom_core::CertifiedBounds + topo::AtRestPolicy>() {
    for (name, r) in pinches::<T>() {
        assert_eq!(validate_closed(&r.body), Ok(()), "{name}: tier 2");
        assert_eq!(
            validate_pseudomanifold(&r.body, &r.contacts, Tol::witness()),
            Ok(()),
            "{name}: tier 3′ with the result's own records"
        );
    }
}

/// At each pinch, one vertex per point: the copies were fused or
/// killed, or (where two operands meet there) declared.
#[test]
fn pinch_copies_do_not_reach_rest() {
    assert_pinches_pass::<f64>();
    let at = |b: &Body<f64>, (x, y, z)| {
        b.vertices()
            .filter(|(_, v)| {
                let p = *b.get_point(v.point).unwrap();
                (p.x, p.y, p.z) == (x, y, z)
            })
            .count()
    };
    for (name, r) in pinches::<f64>() {
        let shared = {
            let mut seen = std::collections::HashSet::new();
            r.body
                .vertices()
                .filter(|(_, v)| !seen.insert(v.point))
                .count()
        };
        assert_eq!(shared, 0, "{name}: no two vertices on one point");
        let expected = match name {
            // Cross-operand: the L's vertex and the cube's, declared.
            "reflex_vertex_kiss" => 2,
            _ => 1,
        };
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

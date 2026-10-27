//! **A boss drawn to the edge of a block's top unions, gluing its two
//! coincidences.**
//!
//! The block is `40 × 20 × 10` centred on the world xy frame; the boss
//! is `x ∈ [10, 20]`, `y ∈ [−5, 5]`, standing 4 tall on the block's
//! top, so its `+x` wall lies in the block's `+x` wall plane, facing the
//! same way. The two solids touch at two faces: the resting cap pair
//! (`Rest`) and the flush walls (a continuation). Each pair is one
//! carrier by margin, so the union glues it, at the closed-form volume
//! `40·20·10 + 10·10·4 = 8400`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, BooleanError, BooleanResult};

/// The block and the boss, finished as operands.
fn scene() -> (AtRestBody<f64>, AtRestBody<f64>) {
    let tol = Tol::witness();
    (
        finished(
            "the block",
            brick((-20.0, 20.0), (-10.0, 10.0), (0.0, 10.0), tol),
            tol,
        ),
        finished(
            "the boss",
            brick((10.0, 20.0), (-5.0, 5.0), (10.0, 14.0), tol),
            tol,
        ),
    )
}

/// A built union: its volume against the closed form, one shell, tier 3
/// and the census.
fn assert_built(tag: &str, r: Result<BooleanResult<f64>, BooleanError>, want: f64) {
    let tol = Tol::witness();
    let bb = match r {
        Ok(BooleanResult::Body(bb)) => bb,
        other => panic!("{tag}: the union builds, got {other:?}"),
    };
    let got = topo::test_support::pinned_volume("the measured body", &bb.body, tol).volume;
    assert!(
        (got - want).abs() <= 1e-12 * want,
        "{tag}: volume {got} vs {want}"
    );
    assert_eq!(bb.body.shells().count(), 1, "{tag}: one shell");
    assert_eq!(
        topo::validate_geometric(&bb.body, tol),
        Ok(()),
        "{tag}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol),
        Ok(()),
        "{tag}: the census"
    );
}

/// **The boss unions at its closed form**, in both operand orders: the
/// `+x` walls a continuation, the caps a `Rest`.
#[test]
fn the_boss_unions_at_its_closed_form() {
    let (block, boss) = scene();
    for (order, a, b) in [
        ("block ∪ boss", &block, &boss),
        ("boss ∪ block", &boss, &block),
    ] {
        assert_built(order, topo::union(a, b, Tol::witness()), 8400.0);
    }
}

/// **A cylinder through the block, its caps flush with the block's top
/// and bottom, unions to the block**: the cylinder lies inside the
/// block, so the union is the block, `40 · 20 · 10 = 8000`. Its rim
/// authored as two arcs and as three, in both operand orders.
#[test]
fn a_cylinder_through_the_block_with_flush_caps_unions_to_the_block() {
    let tol = Tol::witness();
    let block = finished(
        "the block",
        brick((-20.0, 20.0), (-10.0, 10.0), (0.0, 10.0), tol),
        tol,
    );
    for n in [2, 3] {
        let cylinder = finished(
            "the cylinder",
            sweep::test_support::disc_of_arcs(n, 5.0, 10.0, tol),
            tol,
        );
        for (order, a, b) in [
            ("block ∪ cylinder", &block, &cylinder),
            ("cylinder ∪ block", &cylinder, &block),
        ] {
            let tag = format!("{n} arcs, {order}");
            assert_built(&tag, topo::union(a, b, tol), 8000.0);
        }
    }
}

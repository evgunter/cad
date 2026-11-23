//! Reviewer probes for PR 7474 (face-region). Not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};

use crate::entity::LoopBoundary;
use crate::test_support_fixtures::{box_with_a_lone_ring, brick};

fn tol() -> Tol {
    Tol::witness()
}

/// Claim 6: the raw measuring door's pre-check reads the walk with
/// `loop_cycle` (no claimant check) but `CycleLoop::cycle` walks with
/// `loop_walk_of` (claimants checked) and panics. A half-edge whose
/// `parent_loop` names another loop passes the pre-check and panics.
#[test]
fn raw_door_on_a_claimant_tear_refuses_rather_than_panics() {
    let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let faces: Vec<_> = body.faces().map(|(k, f)| (k, f.outer)).collect();
    let (a, b) = (faces[0].1, faces[1].1);
    let LoopBoundary::Cycle { first } = body.loops[a].boundary else {
        panic!()
    };
    let second = body.half_edges[first].next;
    body.half_edges[second].parent_loop = b;
    let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::test_support::measure_unvalidated(
            &body,
            tol(),
            crate::test_support::pinned_resolution(tol()),
        )
        .map(|_| ())
    }));
    match got {
        Ok(r) => println!("no panic: {r:?}"),
        Err(_) => panic!("the raw door panicked on a claimant tear"),
    }
}

/// Claim 5: public `props::loop_edges` on a torn walk. Base answered
/// `LoopEdgesError::Corrupt { "broken half-edge cycle" }`; this asserts
/// it still refuses rather than panics.
#[test]
fn loop_edges_on_a_torn_walk_refuses() {
    let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let lk = body.faces().next().unwrap().1.outer;
    let cycle = body.loop_halves(lk).unwrap();
    body.half_edges.remove(cycle[1]);
    let lp = body.cycle_loop(lk).expect("the record is still a Cycle");
    let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crate::props::loop_edges(lp).map(|_| ())
    }));
    assert!(matches!(got, Ok(Err(_))), "panicked or answered: {:?}", got.is_err());
}

/// Claim 2: the public measure reads past a lone ring.
#[test]
fn a_lone_ring_box_measures_unit_volume() {
    let (body, _, _) = box_with_a_lone_ring(tol());
    let m = crate::test_support::measure_unvalidated(
        &body,
        tol(),
        crate::test_support::pinned_resolution(tol()),
    );
    println!("{:?}", m.as_ref().map(|m| format!("{:?}", m.enclosure)));
    assert!(m.is_ok(), "{:?}", m.err());
}

/// Claim 2: point_in_solid through the lone ring's vertex (the +z ray
/// from the centre passes it) still answers In, the graze re-casting.
#[test]
fn point_in_solid_below_the_lone_ring_is_inside() {
    let (body, _, _) = box_with_a_lone_ring(tol());
    let band = Band::linear(tol()).unwrap();
    for q in [
        Point3::new(0.5, 0.5, 0.5),
        Point3::new(0.5, 0.5, 0.999),
        Point3::new(0.5, 0.5, 1.5),
    ] {
        let got = crate::point_in_solid(&body, q, band, tol());
        println!("{q:?}: {got:?}");
        assert!(got.is_ok(), "{q:?}: {got:?}");
    }
}

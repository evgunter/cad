//! **A face's region read on torn and lone-loop bodies**: the raw
//! measuring door refuses a tear rather than panicking, a torn loop is
//! refused at the door that hands out a [`crate::CycleLoop`], and a lone
//! ring leaves the measure and the solid containment read.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};

use crate::entity::LoopBoundary;
use crate::test_support_fixtures::{box_with_a_lone_ring, brick};

fn tol() -> Tol {
    Tol::witness()
}

/// The raw measuring door reads a body no validator certified: a
/// half-edge whose `parent_loop` names another loop is a tear the one
/// claims walk sees ([`crate::live`]'s `prove_cycle`), and the door
/// refuses it rather than panicking.
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
        // Raw door: the measured body has a claimant tear, which tier 1 refuses, and whether the door refuses it rather than panicking.
        crate::test_support::measure_unvalidated(
            &body,
            tol(),
            crate::test_support::pinned_resolution(tol()),
        )
        .map(|_| ())
    }));
    assert!(
        matches!(got, Ok(Err(crate::MassPropsError::Corrupt { .. }))),
        "the raw door refuses a claimant tear Corrupt: {:?}",
        got.as_ref().map(|r| r.as_ref().err())
    );
}

/// A torn walk is refused, typed, at the door that hands out the loop
/// (`NoCycle::Torn`), so the public `props::loop_edges` never meets it
/// and nothing panics.
#[test]
fn loop_edges_on_a_torn_walk_refuses() {
    let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol());
    let lk = body.faces().next().unwrap().1.outer;
    let cycle = body.loop_halves(lk).unwrap();
    body.half_edges.remove(cycle[1]);
    let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        body.cycle_loop(lk)
            .map(|lp| crate::props::loop_edges(lp).map(|_| ()))
    }));
    assert!(
        matches!(got, Ok(Err(crate::NoCycle::Torn))),
        "panicked or answered: {:?}",
        got.as_ref().map(|r| r.as_ref().err())
    );
}

/// The raw measure reads past a lone ring: the box's volume is one.
#[test]
fn a_lone_ring_box_measures_unit_volume() {
    let (body, _, _) = box_with_a_lone_ring(tol());
    // Raw door: the measured body holds a lone-vertex ring, which tier 2 refuses at rest (ScaffoldingEmptyLoop), and whether the measure reads past it.
    let m = crate::test_support::measure_unvalidated(
        &body,
        tol(),
        crate::test_support::pinned_resolution(tol()),
    );
    let volume = m.as_ref().map(|m| m.enclosure.volume);
    assert!(
        volume.as_ref().is_ok_and(|v| (v - 1.0).abs() < 1e-9),
        "{volume:?}"
    );
}

/// Points under and over the lone ring's vertex read through it (the +z
/// ray from below grazes the vertex and is re-cast).
#[test]
fn point_in_solid_reads_past_the_lone_ring() {
    let (body, _, _) = box_with_a_lone_ring(tol());
    let band = Band::linear(tol()).unwrap();
    for q in [
        Point3::new(0.5, 0.5, 0.5),
        Point3::new(0.5, 0.5, 0.999),
        Point3::new(0.5, 0.5, 1.5),
    ] {
        let got = crate::point_in_solid(&body, q, band, tol());
        assert!(got.is_ok(), "{q:?}: {got:?}");
    }
}

//! Acceptance (e): revolve's typed refusals — half-plane crossings,
//! sliver radii (micro-radius revolve), degenerate/sliver/full-range/
//! poisoned angles, degenerate/poisoned axes, non-manifold axis
//! contact, multiple axis runs, holed full revolves, horn/spindle
//! toroids, and arc-interior half-plane violations. Every case is a
//! typed [`RevolveError`], never a panic or silent approximation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod revolve_common;

use core::f64::consts::{FRAC_PI_8, PI};
use profile::RawLoop;

use geom_core::Tol;
use geom_core::Vec2;
use profile::{ProfileLoop, ProfileVertex};
use revolve_common::*;
use sweep::{Revolution, RevolveAxis, RevolveError, revolve};

fn washer() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)])
}

#[test]
fn vertex_across_the_axis_is_typed() {
    let lp = ProfileLoop::polygon([p2(-1.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(-1.0, 1.0)]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::VertexCrossesAxis { .. }), "{e:?}");
}

#[test]
fn sliver_radius_is_typed() {
    // Left edge a sliver away from the axis: r = 3ε sits inside the
    // (ε, Kε) band at every ε row — a micro-radius revolve.
    let r = 3.0 * eps();
    let lp = ProfileLoop::polygon([p2(r, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(r, 1.0)]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::SliverRadius { .. }), "{e:?}");
}

#[test]
fn degenerate_and_sliver_and_poisoned_angles_are_typed() {
    let vp = validated(vec![washer()]);
    let e = revolve(&vp, axis_y(), Revolution::Partial(0.0), Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::DegenerateAngle), "{e:?}");
    // Sliver: θ·r_max in the band (r_max = 2).
    let e = revolve(
        &vp,
        axis_y(),
        Revolution::Partial(1.5 * eps()),
        Tol::witness(),
    )
    .unwrap_err();
    assert!(matches!(e, RevolveError::AngleEscalated { .. }), "{e:?}");
    // Poisoned angle: decides nothing, escalates loudly.
    let e = revolve(&vp, axis_y(), Revolution::Partial(f64::NAN), Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::AngleEscalated { .. }), "{e:?}");
}

#[test]
fn full_range_partial_angles_are_typed() {
    let vp = validated(vec![washer()]);
    for theta in [2.0 * PI, -2.0 * PI, 3.0 * PI] {
        let e = revolve(&vp, axis_y(), Revolution::Partial(theta), Tol::witness()).unwrap_err();
        assert!(matches!(e, RevolveError::FullRangeAngle), "{theta}: {e:?}");
    }
}

#[test]
fn degenerate_and_poisoned_axes_are_typed() {
    let vp = validated(vec![washer()]);
    let zero = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 0.0),
    };
    let e = revolve(&vp, zero, Revolution::Full, Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::DegenerateAxis), "{e:?}");
    let poison = RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(f64::NAN, f64::NAN),
    };
    let e = revolve(&vp, poison, Revolution::Full, Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::AxisEscalated { .. }), "{e:?}");
}

#[test]
fn isolated_axis_vertex_in_full_revolve_is_non_manifold() {
    // Triangle touching the axis at exactly one vertex: revolving
    // fully would pinch the boundary at that point.
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0)]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(
        matches!(e, RevolveError::NonManifoldAxisContact { .. }),
        "{e:?}"
    );
    // The same profile revolves fine PARTIALLY (axis vertex ordinary).
    let t = revolve(&vp, axis_y(), Revolution::Partial(1.0), Tol::witness()).unwrap();
    assert_all_tiers(&t.body);
}

#[test]
fn two_axis_runs_in_full_revolve_are_typed() {
    // A "C" against the axis: two disjoint on-axis runs (y ∈ [0,1]
    // and y ∈ [2,3]) separated by an inward notch.
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(1.0, 0.0),
        p2(1.0, 3.0),
        p2(0.0, 3.0),
        p2(0.0, 2.0),
        p2(0.5, 2.0),
        p2(0.5, 1.0),
        p2(0.0, 1.0),
    ]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::MultipleAxisRuns { .. }), "{e:?}");
}

#[test]
fn holed_full_revolve_is_typed() {
    let hole = ProfileLoop::polygon([
        p2(1.25, 0.25),
        p2(1.75, 0.25),
        p2(1.75, 0.75),
        p2(1.25, 0.75),
    ]);
    let vp = validated(vec![washer(), hole]);
    let e = revolve(&vp, axis_y(), Revolution::Full, Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::FullRevolveHoles), "{e:?}");
}

#[test]
fn axis_crossing_tube_is_an_unsupported_toroid() {
    // Quarter arc on a carrier centered at (0.5, 0) with radius 0.6:
    // every arc point stays at r > 0, but the carrier reaches across
    // the axis — a spindle torus, refused per D3's ring convention.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.1, 0.0), FRAC_PI_8.tan()),
        ProfileVertex::new(p2(0.5, 0.6), 0.0),
        ProfileVertex::new(p2(0.5, 0.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Partial(1.0), Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::UnsupportedToroid { .. }), "{e:?}");
}

#[test]
fn arc_interior_across_the_axis_is_typed() {
    // A clockwise semicircle from (0, −1) to (0, 1) bulging through
    // (−1, 0): endpoints on the axis, apex definitely across it.
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), -1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    let vp = validated(vec![lp]);
    let e = revolve(&vp, axis_y(), Revolution::Partial(1.0), Tol::witness()).unwrap_err();
    assert!(matches!(e, RevolveError::ArcCrossesAxis { .. }), "{e:?}");
}

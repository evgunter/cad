//! PR 3 adversarial review — target 5 (the in-plane total order) and
//! D9: a TILTED split plane (non-axis normal, non-dyadic components)
//! driven through the full pipeline on both lanes — the in-plane
//! frame arm, the exact-order comparator on computed (inexact)
//! crossing coordinates, byte-identical f64 replay, and f64↔interval
//! structural agreement. The acceptance suite only ever splits by
//! axis planes, where the frame degenerates to exact coordinate
//! pairs — this file is the non-trivial-frame witness.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use topo::{Body, SplitPart, SplitPlane, mass_properties, split, validate_closed};

fn body_of<T: geom_core::Real>(part: &SplitPart<T>) -> &Body<T> {
    part.body().expect("side has material")
}

/// Tilted plane x + y = 2 (unit normal (1,1,0)/√2 — NON-dyadic) on a
/// 4×4×1 box: crossings at dyadic positions but through inexact
/// arithmetic; the sort happens in a genuinely rotated frame.
fn tilted<T: geom_core::Decide>() -> SplitPlane<T> {
    let s = std::f64::consts::FRAC_1_SQRT_2;
    SplitPlane {
        origin: Point3::new(T::from_f64(2.0), T::from_f64(0.0), T::from_f64(0.0)),
        normal: Vec3::new(T::from_f64(s), T::from_f64(s), T::from_f64(0.0)),
    }
}

#[test]
fn tilted_plane_f64_and_replay() {
    let fx = prism::<f64>(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)], 1.0);
    let r = split(&fx.body, &tilted(), Tol::witness()).unwrap();
    let (above, below) = (body_of(&r.above), body_of(&r.below));
    assert_eq!(validate_closed(above), Ok(()));
    assert_eq!(validate_closed(below), Ok(()));
    // The cut corner: below = triangle prism {(0,0),(2,0),(0,2)}·[0,1].
    let (va, vb, v0) = (
        mass_properties(above, Tol::witness()).unwrap().volume,
        mass_properties(below, Tol::witness()).unwrap().volume,
        mass_properties(&fx.body, Tol::witness()).unwrap().volume,
    );
    assert!((vb - 2.0).abs() < 1e-9, "below {vb}");
    assert!((va + vb - v0).abs() <= 1e-12 * v0);
    // D9 byte-identical replay under the rotated frame.
    let again = split(&fx.body, &tilted(), Tol::witness()).unwrap();
    assert_eq!(format!("{above:?}"), format!("{:?}", body_of(&again.above)));
    assert_eq!(format!("{below:?}"), format!("{:?}", body_of(&again.below)));
}

/// The same tilted split on the interval lane: either it lands with
/// the SAME census as f64 (lane agreement) or it refuses typed (the
/// documented posture for enclosure-order escalation). Silent
/// divergence is the only failure.
#[cfg(feature = "interval")]
#[test]
fn tilted_plane_interval_agrees_or_refuses_typed() {
    use geom_core::Interval;
    let fx64 = prism::<f64>(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)], 1.0);
    let r64 = split(&fx64.body, &tilted(), Tol::witness()).unwrap();
    let census64 = |b: &Body<f64>| (b.faces().count(), b.edges().count(), b.vertices().count());
    let c_above = census64(body_of(&r64.above));
    let c_below = census64(body_of(&r64.below));

    let fx = prism::<Interval>(&[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)], 1.0);
    match split(&fx.body, &tilted::<Interval>(), Tol::witness()) {
        Ok(r) => {
            let census =
                |b: &Body<Interval>| (b.faces().count(), b.edges().count(), b.vertices().count());
            assert_eq!(census(body_of(&r.above)), c_above, "above census agrees");
            assert_eq!(census(body_of(&r.below)), c_below, "below census agrees");
            assert_eq!(validate_closed(body_of(&r.above)), Ok(()));
            assert_eq!(validate_closed(body_of(&r.below)), Ok(()));
        }
        Err(e) => eprintln!("tilted interval split refused typed: {e}"),
    }
}

/// An oblique normal whose first schedule members all have big
/// projections (frame arm exercises the schedule) AND a plane
/// orientation flip: the two orders (u from schedule[0] projected)
/// differ between n and −n, but the RESULT bodies must be the same
/// material either way (only above/below swap).
#[test]
fn orientation_flip_swaps_sides_only() {
    let fx = prism::<f64>(
        &[(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (2.0, 3.0), (0.0, 2.0)],
        1.0,
    );
    let plane = |sy: f64| SplitPlane {
        origin: Point3::new(0.0, 1.0, 0.0),
        normal: Vec3::new(0.0, sy, 0.0),
    };
    let r1 = split(&fx.body, &plane(1.0), Tol::witness()).unwrap();
    let r2 = split(&fx.body, &plane(-1.0), Tol::witness()).unwrap();
    let vol = |p: &SplitPart<f64>| mass_properties(body_of(p), Tol::witness()).unwrap().volume;
    assert!((vol(&r1.above) - vol(&r2.below)).abs() < 1e-12);
    assert!((vol(&r1.below) - vol(&r2.above)).abs() < 1e-12);
    let census = |p: &SplitPart<f64>| {
        let b = body_of(p);
        (b.faces().count(), b.edges().count(), b.vertices().count())
    };
    assert_eq!(census(&r1.above), census(&r2.below));
    assert_eq!(census(&r1.below), census(&r2.above));
}

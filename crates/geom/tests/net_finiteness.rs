//! A NURBS net poisoned in ONE channel of every control point is not
//! finite: it is corrupt geometry, and it must fail loudly as such.
//!
//! The crate docs' totality-and-poison section states the rule: a net
//! describes something, and the one question asked of it is whether
//! every channel of every control point is a finite number
//! (`NurbsSurface::is_finite`). What this suite pins, at `f64` and
//! `Dual<f64>`:
//!
//! - a net whose every point has a poisoned `x` and a finite `y`/`z`
//!   is not finite, and neither is its mirror;
//! - the answer survives a lift;
//! - what such a net is downstream: evaluation carries the poison in
//!   the one channel and finite values in the others, and the
//!   projection door refuses rather than returning a foot;
//! - the box screen answers the poison box for it, and for a net with
//!   a `±∞` coordinate.
//!
//! The interval lane's twin is `net_finiteness_interval.rs`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::{Knots, SpanLocate};
use std::sync::Arc;

use bvh::{Aabb, Axis};
use geom::curves::boxes::nurbs_curve_aabb;
use geom::surfaces::boxes::nurbs_surface_aabb;
use geom::{NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Dual, Point3, Real};

/// A clamped quadratic knot vector over five control points.
fn knots5() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// A bilinear knot vector over a 2x2 net.
fn knots2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// The masquerade on a curve carrier: every control point carries a
/// poisoned `x` and a finite `y`/`z`. Structurally valid — the constructor
/// validates counts and weights, and a coordinate is data.
fn masquerading_curve<T: SpanLocate>() -> NurbsCurve3<T> {
    let control: Vec<Point3<T>> = (0..5)
        .map(|i| {
            Point3::new(
                T::from_f64(f64::NAN),
                T::from_f64(1.0),
                T::from_f64(f64::from(i)),
            )
        })
        .collect();
    NurbsCurve3::new(
        Knots::from(knots5()).lift(geom_core::Real::from_f64),
        control,
        vec![Real::one(); 5],
    )
    .unwrap()
}

/// A bilinear net over the four points given, unit weights.
fn bilinear<T: SpanLocate>(control: Vec<Point3<T>>) -> NurbsSurface<T> {
    NurbsSurface::new(
        Knots::from(knots2()).lift(geom_core::Real::from_f64),
        Knots::from(knots2()).lift(geom_core::Real::from_f64),
        control,
        vec![Real::one(); 4],
    )
    .unwrap()
}

/// The masquerade on a surface: every control point carries a poisoned
/// `x` and a finite `y`/`z`.
fn masquerading_surface<T: SpanLocate>() -> NurbsSurface<T> {
    bilinear(
        (0..4)
            .map(|i| {
                Point3::new(
                    T::from_f64(f64::NAN),
                    T::from_f64(f64::from(i)),
                    T::from_f64(2.0),
                )
            })
            .collect(),
    )
}

/// The mirror: a FINITE first channel over a poisoned rest.
fn mirrored_surface<T: SpanLocate>() -> NurbsSurface<T> {
    bilinear(
        (0..4)
            .map(|i| {
                Point3::new(
                    T::from_f64(f64::from(i)),
                    T::from_f64(f64::NAN),
                    T::from_f64(f64::NAN),
                )
            })
            .collect(),
    )
}

/// A net whose every control point has a poisoned `x` and a finite
/// `y`/`z` is not finite.
#[test]
fn a_net_poisoned_in_one_channel_is_not_finite() {
    let s = masquerading_surface::<f64>();
    assert!(
        !s.is_finite(),
        "a control net with finite y and z and a poisoned x is corrupt, not finite"
    );
    // The enum doors carry the same answer: this is what a consumer
    // matching on `Surface::Nurbs(payload)` asks.
    let e = Surface::Nurbs(Arc::new(s));
    assert!(matches!(&e, Surface::Nurbs(n) if !n.is_finite()));
}

/// Its mirror, a finite first channel over a poisoned rest, is not
/// finite either, at `f64` and at the dual scalar.
#[test]
fn the_mirror_with_a_finite_first_channel_is_not_finite_either() {
    let s = mirrored_surface::<f64>();
    assert!(!s.is_finite());
    assert!(!s.map_scalar(Dual::constant).is_finite());
}

/// What the masquerade is downstream, at the doors a consumer reaches
/// through. Evaluation is **partly finite**, and that is the whole
/// hazard: the poisoned channel poisons, the others answer real
/// numbers, so a consumer's y/z margin still decides a sign. The
/// surface projection door, by contrast, refuses rather than returning
/// a foot. Both halves are pinned here because the first is what makes
/// a per-axis consumer wrong and the second is what a certifying one
/// does right.
#[test]
fn the_masquerade_evaluates_partly_finite_and_refuses_projection() {
    let c = masquerading_curve::<f64>();
    let p = c.eval(0.5);
    assert!(p.x.is_nan(), "the poisoned channel evaluates to poison");
    assert!(
        p.y.is_finite() && p.z.is_finite(),
        "the other channels evaluate finitely — this is the masquerade's real shape, \
         and a partially poisoned answer is what a consumer then decides against"
    );
    let s = masquerading_surface::<f64>();
    let q = s.eval(0.5, 0.5);
    assert!(q.x.is_nan() && q.y.is_finite() && q.z.is_finite());
    // The chart-image lane's own door (a foot point on the wall): a
    // poisoned Newton system converges nowhere and says so.
    assert!(
        s.project(Point3::new(0.0, 0.0, 0.0)).is_err(),
        "projection onto a poisoned chart must refuse, never return a best-effort foot"
    );
}

/// A net that is not finite lifts to one that is not finite.
#[test]
fn the_widened_answer_survives_a_lift() {
    let s = Surface::Nurbs(Arc::new(masquerading_surface::<f64>()));
    let sd = s.map_scalar(Dual::constant);
    assert!(matches!(&sd, Surface::Nurbs(n) if !n.is_finite()));
}

/// The box doors are the second member of the same class, and the one
/// that lives in this crate: a certified-conservative AABB folded over
/// a net poisoned in ONE channel is poison on that axis and FINITE on
/// the others, and `Aabb::overlaps` is a per-axis test — so the finite
/// axes witness disjointness and the box PRUNES. A box that prunes on
/// geometry it cannot bound is unsound, and the module's own contract
/// (poison never prunes) is what it violates.
#[test]
fn a_net_poisoned_in_one_channel_yields_the_poison_box_on_every_axis() {
    // A box far away on y and z, disjoint from the masquerade's finite
    // lanes: the witness a partially poisoned box would answer with.
    let elsewhere = Aabb::from_points([
        Point3::new(0.0, 500.0, 500.0),
        Point3::new(1.0, 501.0, 501.0),
    ])
    .unwrap();

    for (what, b) in [
        (
            "surface",
            nurbs_surface_aabb(&masquerading_surface::<f64>()),
        ),
        ("curve", nurbs_curve_aabb(&masquerading_curve::<f64>())),
    ] {
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            assert!(
                b.min(axis).is_nan() && b.max(axis).is_nan(),
                "{what}: a net carrying poison anywhere has no honest box on any axis"
            );
        }
        assert!(
            b.overlaps(&elsewhere),
            "{what}: the poison box must never prune — that is the whole of its contract"
        );
    }

    // A net with an infinite coordinate is not finite either, and the
    // screen answers the poison box for it too.
    let inf = bilinear(
        (0..4)
            .map(|i| Point3::new(f64::from(i), f64::INFINITY, 2.0))
            .collect(),
    );
    assert!(!inf.is_finite());
    let b = nurbs_surface_aabb(&inf);
    assert!(b.min(Axis::X).is_nan() && b.overlaps(&elsewhere));

    // And a fully described net still gets its real, pruning box.
    let described = bilinear(
        (0..4)
            .map(|i| Point3::new(f64::from(i), 1.0, 2.0))
            .collect(),
    );
    assert!(
        !nurbs_surface_aabb(&described).overlaps(&elsewhere),
        "the screen must not cost a described net its pruning power"
    );
}

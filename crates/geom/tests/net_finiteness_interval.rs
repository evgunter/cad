//! The finiteness question at the interval scalar, where poison is NaI
//! or the empty interval rather than NaN.
//!
//! The default lane's twin (`net_finiteness.rs`) carries the
//! argument; this lane exists because `NurbsSurface::is_finite` has to
//! see the interval scalar's poison and its unbounded enclosures. A
//! poisoned channel here is `Interval::from_f64(f64::NAN)`, whose
//! enclosure stands for no real number at all.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::Knots;
use std::sync::Arc;

use bvh::{Aabb, Axis};
use geom::surfaces::boxes::nurbs_surface_aabb;
use geom::{Curve3, NurbsCurve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Interval, Point3, Real};

fn knots5() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap()
}

fn knots2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// The interval lane's poison, stated once here: the lift of NaN.
fn poison() -> Interval {
    Interval::from_f64(f64::NAN)
}

/// The masquerade at the interval scalar: every control point's `x`
/// enclosure is poison, `y`/`z` are point brackets of finite values.
#[test]
fn a_net_poisoned_in_one_channel_is_not_finite_at_the_interval_scalar() {
    assert!(poison().is_poison(), "the interval lane's poison, named");
    let s_control: Vec<Point3<Interval>> = (0..4)
        .map(|i| {
            Point3::new(
                poison(),
                Interval::from_f64(f64::from(i)),
                Interval::from_f64(2.0),
            )
        })
        .collect();
    let s = NurbsSurface::new(
        Knots::from(knots2()).lift(geom_core::Real::from_f64),
        Knots::from(knots2()).lift(geom_core::Real::from_f64),
        s_control,
        vec![Real::one(); 4],
    )
    .unwrap();
    assert!(
        !s.is_finite(),
        "a net corrupt in one channel is not finite at the scalar every certified \
         consumer runs at"
    );
}

/// A carrier poisoned in `x` lifts from `f64` to the interval scalar
/// channel by channel, and its enclosures show the shape: the poisoned
/// channel encloses nothing, the others bracket their finite values.
#[test]
fn the_masquerade_lifts_to_the_interval_scalar_channel_by_channel() {
    let control: Vec<Point3<f64>> = (0..5)
        .map(|i| Point3::new(f64::NAN, 1.0, f64::from(i)))
        .collect();
    let c = NurbsCurve3::new(
        Knots::from(knots5()).lift(geom_core::Real::from_f64),
        control,
        vec![Real::one(); 5],
    )
    .unwrap();
    let e = Curve3::Nurbs(Arc::new(c));
    let ei: Curve3<Interval> = e.map_scalar(Interval::from_f64);
    let p = ei.eval(Interval::from_f64(0.5));
    assert!(p.x.is_poison(), "the poisoned channel encloses no real");
    assert!(
        !p.y.is_poison() && !p.z.is_poison(),
        "the finite channels still enclose their values — the partial answer a consumer \
         then decides against"
    );
}

/// The box doors at the interval scalar: the same screen, and the same
/// executed consequence — a box poisoned on one axis and finite on the
/// others prunes on the finite ones.
#[test]
fn a_net_poisoned_in_one_channel_yields_the_poison_box_at_interval() {
    let control: Vec<Point3<Interval>> = (0..4)
        .map(|i| {
            Point3::new(
                poison(),
                Interval::from_f64(f64::from(i)),
                Interval::from_f64(2.0),
            )
        })
        .collect();
    let s = NurbsSurface::new(
        Knots::from(knots2()).lift(geom_core::Real::from_f64),
        Knots::from(knots2()).lift(geom_core::Real::from_f64),
        control,
        vec![Real::one(); 4],
    )
    .unwrap();
    let b = nurbs_surface_aabb(&s);
    for axis in [Axis::X, Axis::Y, Axis::Z] {
        assert!(b.min(axis).is_nan() && b.max(axis).is_nan());
    }
    let elsewhere = Aabb::from_points([
        Point3::new(0.0, 500.0, 500.0),
        Point3::new(1.0, 501.0, 501.0),
    ])
    .unwrap();
    assert!(b.overlaps(&elsewhere), "the poison box never prunes");
}

/// The interval scalar's poison is NaI **or** the empty interval, and a
/// net carrying an empty enclosure in any one channel is not finite.
#[test]
fn a_net_with_an_empty_enclosure_is_not_finite_at_interval() {
    // `sqrt` of a wholly negative enclosure is empty (its own doc).
    let empty = Interval::from_f64(-4.0).sqrt();
    assert!(
        empty.is_poison(),
        "the empty interval is this scalar's poison"
    );
    let bilinear = |control: Vec<Point3<Interval>>| {
        NurbsSurface::new(
            Knots::from(knots2()).lift(geom_core::Real::from_f64),
            Knots::from(knots2()).lift(geom_core::Real::from_f64),
            control,
            vec![Real::one(); 4],
        )
        .unwrap()
    };
    assert!(!bilinear(vec![Point3::new(empty, empty, empty); 4]).is_finite());
    // And one empty channel over finite ones is not finite either.
    let mixed = bilinear(vec![
        Point3::new(
            empty,
            Interval::from_f64(1.0),
            Interval::from_f64(2.0)
        );
        4
    ]);
    assert!(!mixed.is_finite());
}

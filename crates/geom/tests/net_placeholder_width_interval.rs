//! The placeholder discriminator's width at the interval scalar, where
//! poison is NaI or the empty interval rather than NaN.
//!
//! The default lane's twin (`net_placeholder_width.rs`) carries the
//! argument; this lane exists because the widening has to SEE the
//! interval scalar's poison through the same `Real::is_poison` door,
//! and because every consumer of the discriminator compiles in both
//! lanes. A poisoned channel here is `Interval::from_f64(f64::NAN)`,
//! whose enclosure stands for no real number at all.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use bvh::{Aabb, Axis};
use geom::surfaces::boxes::nurbs_surface_aabb;
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
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
fn a_net_poisoned_in_one_channel_is_described_at_the_interval_scalar() {
    assert!(poison().is_poison(), "the interval lane's poison, named");
    let control: Vec<Point3<Interval>> = (0..5)
        .map(|i| {
            Point3::new(
                poison(),
                Interval::from_f64(1.0),
                Interval::from_f64(f64::from(i)),
            )
        })
        .collect();
    let c = NurbsCurve3::new(knots5(), control, vec![1.0; 5]).unwrap();
    assert!(
        !c.is_placeholder(),
        "a described net corrupt in one channel must not read as the benign placeholder \
         at the scalar every certified consumer runs at"
    );
    let s_control: Vec<Point3<Interval>> = (0..4)
        .map(|i| {
            Point3::new(
                poison(),
                Interval::from_f64(f64::from(i)),
                Interval::from_f64(2.0),
            )
        })
        .collect();
    let s = NurbsSurface::new(knots2(), knots2(), s_control, vec![1.0; 4]).unwrap();
    assert!(!s.is_placeholder());
    assert!(matches!(Surface::Nurbs(Arc::new(s)), Surface::Nurbs(ref n) if !n.is_placeholder()));
}

/// The state the predicate names still answers it at this scalar, and
/// the f64 placeholder still lifts to it.
#[test]
fn the_minted_placeholder_still_reads_as_the_placeholder_at_interval() {
    assert!(NurbsCurve3::<Interval>::placeholder().is_placeholder());
    assert!(NurbsSurface::<Interval>::placeholder().is_placeholder());
    let lifted = Curve3::<f64>::nurbs_placeholder().map_scalar(Interval::from_f64);
    assert!(matches!(&lifted, Curve3::Nurbs(n) if n.is_placeholder()));
    let lifted_s = Surface::<f64>::nurbs_placeholder().map_scalar(Interval::from_f64);
    assert!(matches!(&lifted_s, Surface::Nurbs(n) if n.is_placeholder()));
}

/// The masquerade lifts from `f64` to the interval scalar as a
/// described net, and its enclosures show the shape: the poisoned
/// channel encloses nothing, the others bracket their finite values.
#[test]
fn the_masquerade_lifts_to_a_described_interval_net() {
    let control: Vec<Point3<f64>> = (0..5)
        .map(|i| Point3::new(f64::NAN, 1.0, f64::from(i)))
        .collect();
    let c = NurbsCurve3::new(knots5(), control, vec![1.0; 5]).unwrap();
    assert!(!c.is_placeholder());
    let e = Curve3::Nurbs(Arc::new(c));
    let ei: Curve3<Interval> = e.map_scalar(Interval::from_f64);
    assert!(
        matches!(&ei, Curve3::Nurbs(n) if !n.is_placeholder()),
        "the lift neither narrows nor widens the rule"
    );
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
    let s = NurbsSurface::new(knots2(), knots2(), control, vec![1.0; 4]).unwrap();
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

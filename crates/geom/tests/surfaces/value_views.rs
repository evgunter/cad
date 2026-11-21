//! A geometry type's value channel (`values()`) and a place evaluated on
//! it (`geom_core::place`): at a dual the view is a copy whose point at a
//! place is the dual's value bit for bit, the tangent the dual carried
//! notwithstanding; at an enclosure it encloses the exact point; at a
//! scalar that is its own value channel the view is the value itself.

#![allow(clippy::unwrap_used)]

use geom::{Curve3, NurbsSurface};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Bounds, Dual64, DualInterval, Interval, Point3, Real, ValueView, place};

/// A biquadratic-by-linear patch with an interior `u` knot at `0.5`, its
/// control points lifted by `lift` (a dual's carries a tangent).
fn patch<T: geom_core::SpanLocate>(lift: impl Fn(f64) -> T) -> NurbsSurface<T> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..4 {
        for j in 0..2 {
            let p = Point3::new(
                f64::from(i) * 0.7,
                f64::from(j),
                f64::from(i * j) * 0.3 + 0.1,
            );
            control.push(p.map(&lift));
            weights.push(lift(1.0 + 0.25 * f64::from(i % 2)));
        }
    }
    NurbsSurface::new(
        Knots::from(ku).lift(&lift),
        Knots::from(kv).lift(&lift),
        control,
        weights,
    )
    .unwrap()
}

/// The places read: the domain's ends, the knot `0.5` and the floats
/// either side of it (where the span changes), and interior points.
fn places() -> Vec<f64> {
    vec![
        0.0,
        1.0,
        0.5,
        0.5f64.next_down(),
        0.5f64.next_up(),
        0.125,
        0.3,
        0.875,
        1.0f64.next_down(),
    ]
}

#[test]
fn a_dual_views_its_value_part_and_a_place_reads_its_value_bit_for_bit() {
    let dual = patch(Dual64::variable);
    let view = dual.values();
    assert!(
        matches!(view, ValueView::Stripped(_)),
        "a dual's view is a copy with its tangent removed"
    );
    for u in places() {
        for v in places() {
            let at_dual = dual.eval(Dual64::from_f64(u), Dual64::from_f64(v));
            let on_view = view.eval(place(u), place(v));
            for (d, x) in [
                (at_dual.x, on_view.x),
                (at_dual.y, on_view.y),
                (at_dual.z, on_view.z),
            ] {
                assert_eq!(
                    d.value.to_bits(),
                    x.to_bits(),
                    "the view's point at ({u}, {v}) is the dual's value part"
                );
            }
        }
    }
}

#[test]
fn a_dual_interval_view_encloses_the_exact_point_at_a_place() {
    let dual = patch(|x| DualInterval::variable(Interval::from_f64(x)));
    let plain = patch(|x| x);
    let view = dual.values();
    for u in places() {
        for v in places() {
            let p = plain.eval(u, v);
            let q = view.eval(place(u), place(v));
            for (x, e) in [(p.x, q.x), (p.y, q.y), (p.z, q.z)] {
                assert!(
                    e.lo() <= x && x <= e.hi(),
                    "the view at ({u}, {v}) encloses the f64 point: {x} in [{}, {}]",
                    e.lo(),
                    e.hi()
                );
            }
        }
    }
}

#[test]
fn a_scalar_that_is_its_own_value_channel_views_itself() {
    let plain = patch(|x| x);
    assert!(
        matches!(plain.values(), ValueView::Same(s) if core::ptr::eq(s, &plain)),
        "at f64 the view borrows the surface"
    );
    let enclosed = patch(Interval::from_f64);
    assert!(
        matches!(enclosed.values(), ValueView::Same(s) if core::ptr::eq(s, &enclosed)),
        "at Interval the view borrows the surface"
    );
    let curve = Curve3::line(
        Point3::new(
            Dual64::variable(1.0),
            Dual64::from_f64(0.0),
            Dual64::from_f64(0.0),
        ),
        geom_core::Vec3::new(
            Dual64::from_f64(1.0),
            Dual64::from_f64(0.0),
            Dual64::from_f64(0.0),
        ),
    );
    let on = curve.values();
    let at = curve.eval(Dual64::from_f64(0.25));
    assert_eq!(
        on.eval(place(0.25)).x.to_bits(),
        at.x.value.to_bits(),
        "a curve's view reads the dual's value part at a place"
    );
}

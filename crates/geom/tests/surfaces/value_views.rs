//! A geometry without its tangent (`value_view()`) and a place handed to
//! its evaluator as an `f64` (`eval_at`): at a dual the view is a copy
//! whose point at a place is the dual's value part bit for bit, the
//! tangent the dual carried notwithstanding; at `Dual<Interval>` it is
//! the plain `Interval` evaluation bit for bit; at a scalar that carries
//! no tangent the view is the geometry itself; at `Sym` each place is its
//! own unknown.

#![allow(clippy::unwrap_used)]

use geom::{Curve3, NurbsSurface};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Bounds, Dual64, DualInterval, Interval, Point3, Real, ValueView};

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
    let view = dual.value_view();
    assert!(
        matches!(view, ValueView::Stripped(_)),
        "a dual's view is a copy with its tangent removed"
    );
    for u in places() {
        for v in places() {
            let at_dual = dual.eval(Dual64::from_f64(u), Dual64::from_f64(v));
            let on_view = view.eval_at(u, v);
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
fn a_dual_interval_view_is_the_plain_interval_evaluation_at_a_place() {
    let dual = patch(|x| DualInterval::variable(Interval::from_f64(x)));
    let plain = patch(Interval::from_f64);
    let view = dual.value_view();
    for u in places() {
        for v in places() {
            let p = plain.eval_at(u, v);
            let q = view.eval_at(u, v);
            for (x, e) in [(p.x, q.x), (p.y, q.y), (p.z, q.z)] {
                assert_eq!(
                    (x.lo().to_bits(), x.hi().to_bits()),
                    (e.lo().to_bits(), e.hi().to_bits()),
                    "the view at ({u}, {v}) is the plain Interval evaluation"
                );
            }
        }
    }
}

#[test]
fn a_scalar_that_is_its_own_value_channel_views_itself() {
    let plain = patch(|x| x);
    assert!(
        matches!(plain.value_view(), ValueView::Same(s) if core::ptr::eq(s, &plain)),
        "at f64 the view borrows the surface"
    );
    let enclosed = patch(Interval::from_f64);
    assert!(
        matches!(enclosed.value_view(), ValueView::Same(s) if core::ptr::eq(s, &enclosed)),
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
    let on = curve.value_view();
    let at = curve.eval(Dual64::from_f64(0.25));
    assert_eq!(
        on.eval_at(0.25).x.to_bits(),
        at.x.value.to_bits(),
        "a curve's view reads the dual's value part at a place"
    );
}

#[test]
fn at_sym_the_view_borrows_and_each_place_is_its_own_unknown() {
    use geom_core::sym::with_session;
    use geom_core::{Sym, SymBudget};
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    let ((), _) = with_session(budget, || {
        let s = patch(Sym::<f64>::from_f64);
        assert!(
            matches!(s.value_view(), ValueView::Same(v) if core::ptr::eq(v, &s)),
            "over f64 a Sym surface's view borrows"
        );
        let (a, b) = (s.eval_at(0.3, 0.6), s.eval_at(0.3, 0.6));
        assert_eq!(a.x.value.to_bits(), b.x.value.to_bits(), "the same number");
        assert!(a.x.node() != b.x.node(), "one unknown per place");
    });
}

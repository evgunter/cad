//! Review probes for PR 7393 (the iso image holds its line): what
//! [`Lift::stored_corners`] reads at the END of a row the lift moved one
//! period, and what [`Period::moved`] reads at a scalar that carries a
//! derivative.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use super::{ChartPeriods, Lift, LiftedRow};
use crate::joint::{Deck, Winding};
use geom::Surface;
use geom_brep::{Pcurve, Period};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Dual, Interval, Point3, Real};

fn length<T: Real>() -> T {
    T::from_f64(1.2) + T::one()
}

fn row<T: Decide>(v: f64, (u0, u1): (T, T), span: (T, T)) -> LiftedRow<T> {
    LiftedRow::stated_for_tests(Pcurve::iso_row(T::from_f64(v), (u0, u1), span), span, true)
}

fn column<T: Decide>(u: T, (v0, v1): (f64, f64), span: (T, T)) -> LiftedRow<T> {
    let f = T::from_f64;
    LiftedRow::stated_for_tests(Pcurve::iso_column(u, (f(v0), f(v1)), span), span, true)
}

fn patch<T: Decide>(lo: f64, hi: f64) -> Surface<T> {
    let ku = KnotVector::clamped(vec![lo, lo, hi, hi], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, y, 0.0))
        .to_vec();
    let net = geom::NurbsSurface::new(ku, kv, control, vec![1.0; 4]).unwrap();
    Surface::Nurbs(std::sync::Arc::new(net.map_scalar(T::from_f64)))
}

/// A whole-period row stored over `[lo, hi]` and lifted one period runs
/// `[hi, hi + P]` in the lift, between the seam's column moved once
/// (`hi`, exact) and the seam's column moved twice (`Period::moved(hi,
/// 1)`, the sum). Its stored corners: the start un-moves to `lo` itself;
/// the end un-moves through the sum, and is not `hi`.
#[test]
fn a_lifted_whole_period_rows_far_stored_corner_is_a_computed_sum() {
    fn at<T: Decide + Bounds>(scalar: &str) -> ((f64, f64), (f64, f64)) {
        let f = T::from_f64;
        let (lo, hi) = (0.4, 1.7);
        let surface = patch::<T>(lo, hi);
        let periods = ChartPeriods::stated(&surface);
        let period = periods.u().unwrap();
        let span = (T::zero(), length::<T>());
        let once = Deck {
            u: 1,
            v: 0,
            twin: false,
        };
        let stored = row::<T>(0.0, (f(lo), f(hi)), span);
        let mut lifted = stored.clone().moved_for_tests(once);
        lifted.pcurve = once.apply(&stored.pcurve, &surface, periods).unwrap();
        // The seam at `lo`, moved once and moved twice, as `Period::moved`
        // moves a held value.
        let once_moved = period.moved(f(lo), 1);
        let twice_moved = period.moved(once_moved, 1);
        let seam = column::<T>(once_moved, (1.0, 0.0), span);
        let far = column::<T>(twice_moved, (0.0, 1.0), span);
        let top = row::<T>(1.0, (twice_moved, once_moved), span);
        let lift = Lift::stated_for_tests(
            vec![lifted, far, top, seam],
            Some(Winding {
                deck: Deck::IDENTITY,
            }),
        );
        let [start, end] = lift.stored_corners(0, periods).map(|c| c.unwrap());
        println!(
            "PROBE {scalar} stored_corners start=[{:e},{:e}] end=[{:e},{:e}] (lo={lo:e} hi={hi:e})",
            start.lo(),
            start.hi(),
            end.lo(),
            end.hi()
        );
        ((start.lo(), start.hi()), (end.lo(), end.hi()))
    }
    let (s, e) = at::<f64>("f64");
    assert_eq!(s, (0.4, 0.4));
    assert_ne!(e, (1.7, 1.7), "f64: the far stored corner misses hi");
    let (s, e) = at::<Interval>("Interval");
    assert_eq!(s, (0.4, 0.4));
    assert!(e.0 < e.1, "Interval: the far stored corner has width");
}

/// `Period::moved` at a scalar that carries a derivative: a value equal
/// to the low end in its value channel and moving with the parameter is
/// read as "exactly the end", and the result is the other end, whose
/// derivative is the end's and not the value's.
#[test]
fn a_moved_value_equal_to_an_end_in_value_only_drops_its_derivative() {
    let (lo, hi) = (Dual::constant(0.4_f64), Dual::constant(1.7_f64));
    let period = Period::of_domain(lo, hi);
    let x = Dual::variable(0.4_f64);
    let moved = period.moved(x, 1);
    let sum = x + period.times(1);
    println!(
        "PROBE Dual moved value={:e} deriv={:e}; sum value={:e} deriv={:e}",
        moved.value, moved.deriv, sum.value, sum.deriv
    );
    assert_eq!(moved.value, 1.7);
    assert_eq!(
        moved.deriv, 0.0,
        "the shortcut returns hi, derivative and all"
    );
    assert_eq!(
        sum.deriv, 1.0,
        "the arithmetic it skips carries the value's derivative"
    );
    // One step off the end in value, the derivative is kept.
    let near = Dual::variable(f64::from_bits(0.4_f64.to_bits() + 1));
    assert_eq!(period.moved(near, 1).deriv, 1.0);
}

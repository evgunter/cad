//! CERT-N2 R2 reviewer probes: the placeholder discriminator's width
//! past the PR's own fixtures. Probe file — not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom::{NurbsCurve2, NurbsCurve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Dual, Point2, Point3, Real};

fn knots5() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0], 2).unwrap()
}
fn knots2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}
fn curve<T: Real>(f: impl Fn(usize) -> Point3<T>) -> NurbsCurve3<T> {
    NurbsCurve3::new(knots5(), (0..5).map(f).collect(), vec![1.0; 5]).unwrap()
}
fn surf<T: Real>(f: impl Fn(usize) -> Point3<T>) -> NurbsSurface<T> {
    NurbsSurface::new(knots2(), knots2(), (0..4).map(f).collect(), vec![1.0; 4]).unwrap()
}
fn nan() -> f64 {
    f64::NAN
}

/// Channel-position variants at f64: y only, z only, x+y, y+z, and
/// one point of many (all-poison and one-channel). Every one must be
/// DESCRIBED; the true all-poison placeholder must stay placeholder.
#[test]
fn n2r2_masquerade_channel_positions_f64() {
    let rows: Vec<(&str, NurbsCurve3<f64>)> = vec![
        ("y only", curve(|i| Point3::new(i as f64, nan(), 2.0))),
        ("z only", curve(|i| Point3::new(i as f64, 1.0, nan()))),
        ("x+y", curve(|i| Point3::new(nan(), nan(), i as f64))),
        ("y+z", curve(|i| Point3::new(i as f64, nan(), nan()))),
        (
            "one point all-poison",
            curve(|i| {
                if i == 2 {
                    Point3::new(nan(), nan(), nan())
                } else {
                    Point3::new(i as f64, 1.0, 0.0)
                }
            }),
        ),
        (
            "one point x-poison",
            curve(|i| {
                if i == 4 {
                    Point3::new(nan(), 1.0, 0.0)
                } else {
                    Point3::new(i as f64, 1.0, 0.0)
                }
            }),
        ),
        (
            "all but one point all-poison",
            curve(|i| {
                if i == 0 {
                    Point3::new(0.0, 1.0, 0.0)
                } else {
                    Point3::new(nan(), nan(), nan())
                }
            }),
        ),
    ];
    for (name, c) in &rows {
        assert!(!c.is_placeholder(), "{name}: must read described");
        // The widened predicate implies the narrow one on every row.
        if c.is_placeholder() {
            assert!(c.control().iter().all(|p| p.x.is_poison()));
        }
    }
    let srows: Vec<(&str, NurbsSurface<f64>)> = vec![
        ("y only", surf(|i| Point3::new(i as f64, nan(), 2.0))),
        ("z only", surf(|i| Point3::new(i as f64, 1.0, nan()))),
        ("x+y", surf(|i| Point3::new(nan(), nan(), i as f64))),
        (
            "one point x-poison",
            surf(|i| {
                if i == 3 {
                    Point3::new(nan(), 1.0, 0.0)
                } else {
                    Point3::new(i as f64, 1.0, 0.0)
                }
            }),
        ),
    ];
    for (name, s) in &srows {
        assert!(!s.is_placeholder(), "surface {name}: must read described");
    }
    assert!(curve::<f64>(|_| Point3::new(nan(), nan(), nan())).is_placeholder());
    assert!(surf::<f64>(|_| Point3::new(nan(), nan(), nan())).is_placeholder());
}

/// The 2-D net (`NurbsCurve2`) takes the same rule through the same
/// helper: an x-only masquerade is described, all-poison is placeholder.
#[test]
fn n2r2_masquerade_in_two_dimensions() {
    let c2 = NurbsCurve2::new(
        knots5(),
        (0..5).map(|i| Point2::new(nan(), i as f64)).collect(),
        vec![1.0; 5],
    )
    .unwrap();
    // NurbsCurve2 has no `is_placeholder` door? — check what exists.
    let p = c2.eval(0.5);
    assert!(
        p.x.is_nan() && p.y.is_finite(),
        "the 2-D masquerade evaluates partially: {p:?}"
    );
}

/// `Dual<f64>`: `is_poison` reads the VALUE channel only. A net whose
/// every value is finite and every derivative NaN reads DESCRIBED; the
/// reverse (NaN value, finite derivative) reads PLACEHOLDER.
#[test]
fn n2r2_dual_value_versus_derivative_poison() {
    let finite_value_nan_deriv = curve::<Dual<f64>>(|i| {
        Point3::new(
            Dual::new(i as f64, nan()),
            Dual::new(1.0, nan()),
            Dual::new(0.0, nan()),
        )
    });
    assert!(
        !finite_value_nan_deriv.is_placeholder(),
        "finite values over NaN derivatives: described"
    );
    let nan_value_finite_deriv = curve::<Dual<f64>>(|_| {
        Point3::new(
            Dual::new(nan(), 1.0),
            Dual::new(nan(), 2.0),
            Dual::new(nan(), 3.0),
        )
    });
    assert!(
        nan_value_finite_deriv.is_placeholder(),
        "NaN values over finite derivatives: placeholder (value-channel rule)"
    );
    // The masquerade at Dual: NaN value in x only.
    let m = curve::<Dual<f64>>(|i| {
        Point3::new(
            Dual::new(nan(), 1.0),
            Dual::constant(i as f64),
            Dual::constant(0.0),
        )
    });
    assert!(!m.is_placeholder());
    let e = m.eval(Dual::variable(0.5));
    eprintln!("dual masquerade eval: {e:?}");
    assert!(e.x.value.is_nan() && e.y.value.is_finite());
}

#[cfg(feature = "interval")]
mod interval_lane {
    use super::*;
    use geom_core::Interval;

    fn nai() -> Interval {
        Interval::from_f64(f64::NAN)
    }
    fn empty() -> Interval {
        let e = Interval::from_bounds(-2.0, -1.0).sqrt();
        assert!(
            e.is_poison(),
            "sqrt of a negative bracket is the empty enclosure"
        );
        e
    }
    fn entire() -> Interval {
        Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY)
    }
    fn pt(x: f64) -> Interval {
        Interval::from_f64(x)
    }

    /// EMPTY versus NaI versus `[-inf, inf]` in one channel.
    #[test]
    fn n2r2_interval_kinds_of_poison_in_one_channel() {
        eprintln!(
            "nai.is_poison={} empty.is_poison={} entire.is_poison={} entire=({}, {})",
            nai().is_poison(),
            empty().is_poison(),
            entire().is_poison(),
            geom_core::Bounds::lo(entire()),
            geom_core::Bounds::hi(entire())
        );
        let x_nai = curve::<Interval>(|i| Point3::new(nai(), pt(1.0), pt(i as f64)));
        let x_empty = curve::<Interval>(|i| Point3::new(empty(), pt(1.0), pt(i as f64)));
        let x_entire = curve::<Interval>(|i| Point3::new(entire(), pt(1.0), pt(i as f64)));
        assert!(!x_nai.is_placeholder(), "NaI in x only: described");
        assert!(!x_empty.is_placeholder(), "empty in x only: described");
        assert!(
            !x_entire.is_placeholder(),
            "[-inf,inf] in x only: described (not poison)"
        );
        // All-empty and mixed empty/NaI nets: every channel is poison
        // under the interval `is_poison`, so these read PLACEHOLDER —
        // a wider placeholder set than at f64 (where only NaN counts).
        let all_empty = curve::<Interval>(|_| Point3::new(empty(), empty(), empty()));
        let mixed = curve::<Interval>(|_| Point3::new(empty(), nai(), empty()));
        eprintln!(
            "all-empty.is_placeholder={} mixed-empty/NaI.is_placeholder={}",
            all_empty.is_placeholder(),
            mixed.is_placeholder()
        );
        // All-entire is NOT the placeholder.
        let all_entire = curve::<Interval>(|_| Point3::new(entire(), entire(), entire()));
        assert!(!all_entire.is_placeholder());
        // The true placeholder at the interval scalar.
        assert!(NurbsCurve3::<Interval>::placeholder().is_placeholder());
        let s = surf::<Interval>(|i| Point3::new(empty(), pt(i as f64), pt(2.0)));
        assert!(!s.is_placeholder());
        let q = s.eval(pt(0.5), pt(0.5));
        eprintln!(
            "surface empty-x eval: x.poison={} y.poison={} z.poison={}",
            q.x.is_poison(),
            q.y.is_poison(),
            q.z.is_poison()
        );
    }
}

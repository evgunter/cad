//! CERT-N2 R1 reviewer probes — the masquerade shapes the lane's two
//! suites do not mint, and the poison notions the widened predicate
//! rests on. Read-only probes: nothing here is a proposed fix.

// Adoption annotation (CERT-N2 fix pass), not the reviewer's: the
// shape table below is a `[(&str, fn(f64) -> Point3<f64>); 6]`, which
// `type_complexity` denies inside the aggregated test binary. The row
// is kept as written.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::type_complexity
)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::Knots;
use geom_core::spline::{KnotVector, SpanLocate};
use geom_core::{Dual, Point3, Real};

/// A bilinear patch over the four points given, unit weights.
fn surface_from<T: SpanLocate>(control: Vec<Point3<T>>) -> NurbsSurface<T> {
    let kv = || Knots::from(KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap());
    NurbsSurface::new(
        kv().lift(geom_core::Real::from_f64),
        kv().lift(geom_core::Real::from_f64),
        control,
        vec![T::one(); 4],
    )
    .unwrap()
}

/// Poison in y only, z only, and in two channels — every one of these
/// must read DESCRIBED. The lane only mints the `x`-poisoned shape.
#[test]
fn probe_every_partial_channel_pattern_is_described() {
    let nan = f64::NAN;
    let shapes: [(&str, fn(f64) -> Point3<f64>); 6] = [
        ("x only", |i| Point3::new(f64::NAN, 1.0, i)),
        ("y only", |i| Point3::new(i, f64::NAN, 1.0)),
        ("z only", |i| Point3::new(i, 1.0, f64::NAN)),
        ("x and y", |i| Point3::new(f64::NAN, f64::NAN, i)),
        ("y and z", |i| Point3::new(i, f64::NAN, f64::NAN)),
        ("x and z", |i| Point3::new(f64::NAN, i, f64::NAN)),
    ];
    for (name, f) in shapes {
        let c = surface_from((0..4).map(|i| f(f64::from(i))).collect());
        assert!(!c.is_placeholder(), "{name} must read described");
        assert!(
            !c.map_scalar(Dual::constant).is_placeholder(),
            "{name} at Dual"
        );
    }
    let _ = nan;
    // The true all-poison net is still the placeholder.
    let all = surface_from((0..4).map(|_| Point3::new(nan, nan, nan)).collect());
    assert!(all.is_placeholder());
}

/// ONE point of four all-poison, the rest finite: described, both
/// before and after. (The lane's battery has this row but never asserts
/// the answer directly.)
#[test]
fn probe_one_all_poison_point_of_four_is_described() {
    let mut ctl: Vec<Point3<f64>> = (0..4)
        .map(|i| Point3::new(f64::from(i), 1.0, 0.0))
        .collect();
    ctl[2] = Point3::new(f64::NAN, f64::NAN, f64::NAN);
    assert!(!surface_from(ctl).is_placeholder());
}

/// **`Dual`'s `is_poison` is the VALUE channel only.** A net whose
/// every coordinate has a finite value and a poisoned DERIVATIVE is
/// described (right, per D8); the reverse — poisoned value, finite
/// derivative, in every channel — is the placeholder. The widened
/// predicate did not change either answer, and both are the same
/// notion `f64` uses one level down.
#[test]
fn probe_dual_poison_is_the_value_channel_in_both_directions() {
    let poisoned_deriv: Vec<Point3<Dual<f64>>> = (0..4)
        .map(|i| {
            let d = |v: f64| Dual {
                value: v,
                deriv: f64::NAN,
            };
            Point3::new(d(f64::from(i)), d(1.0), d(0.0))
        })
        .collect();
    let c = surface_from(poisoned_deriv);
    assert!(
        !c.is_placeholder(),
        "a poisoned tangent is not a poisoned number (D8)"
    );

    let poisoned_value: Vec<Point3<Dual<f64>>> = (0..4)
        .map(|_| {
            let d = Dual {
                value: f64::NAN,
                deriv: 1.0,
            };
            Point3::new(d, d, d)
        })
        .collect();
    let c2 = surface_from(poisoned_value);
    assert!(
        c2.is_placeholder(),
        "value-channel poison in every channel reads placeholder at Dual — \
         a net no mint produces, but the predicate says so"
    );
    // And it is NOT the net `placeholder()` mints: that one's derivative
    // is zero. Two different values, one answer.
    assert!(NurbsSurface::<Dual<f64>>::placeholder().is_placeholder());
}

/// The enum door carries the answer for the surface — what a consumer
/// matching `Surface::Nurbs` asks.
#[test]
fn probe_enum_doors_agree_on_every_partial_shape() {
    let s = surface_from(
        (0..4)
            .map(|i| Point3::new(f64::from(i), 1.0, f64::NAN))
            .collect(),
    );
    let se = Surface::Nurbs(std::sync::Arc::new(s));
    assert!(matches!(&se, Surface::Nurbs(n) if !n.is_placeholder()));
}

/// The direction claim on a battery the lane's own property row does
/// not run: wide ⇒ narrow over every 3-channel poison pattern of a
/// 4-point net (2^3 patterns × uniform application), plus the empty
/// and single-point degenerate shapes the predicate can see.
#[test]
fn probe_wide_implies_narrow_over_every_channel_pattern() {
    for mask in 0u8..8 {
        let ctl: Vec<Point3<f64>> = (0..4)
            .map(|i| {
                let v = |bit: u8, fallback: f64| {
                    if mask & bit != 0 { f64::NAN } else { fallback }
                };
                Point3::new(v(1, f64::from(i)), v(2, 1.0), v(4, 2.0))
            })
            .collect();
        let c = surface_from(ctl);
        let narrow = c.control().iter().all(|p| p.x.is_poison());
        if c.is_placeholder() {
            assert!(
                narrow,
                "mask {mask}: wide answered placeholder, narrow did not"
            );
        }
        assert_eq!(
            c.is_placeholder(),
            mask == 7,
            "mask {mask}: only the all-channel mask is the placeholder"
        );
    }
}

mod interval {
    use super::{Point3, surface_from};
    use geom_core::{Interval, Real};

    /// The three interval shapes the dispatch names, told apart:
    /// `[NaN,NaN]` (NaI) and the EMPTY interval are poison; the entire
    /// interval `[-inf, +inf]` is NOT. A net whose every channel is
    /// entire is therefore DESCRIBED — the enclosure that claims
    /// nothing but is not poison.
    #[test]
    fn probe_empty_versus_nai_versus_entire() {
        let nai = Interval::from_f64(f64::NAN);
        let empty = Real::sqrt(Interval::from_bounds(-4.0, -1.0));
        let entire = Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY);
        assert!(nai.is_poison(), "NaI is poison");
        assert!(empty.is_poison(), "empty is poison");
        assert!(
            !entire.is_poison(),
            "the entire interval is a legal enclosure, not poison"
        );

        let net = |c: Interval| surface_from((0..4).map(|_| Point3::new(c, c, c)).collect());
        assert!(net(nai).is_placeholder(), "all-NaI reads placeholder");
        assert!(
            net(empty).is_placeholder(),
            "all-EMPTY reads placeholder too — a DIFFERENT value from the one \
             `placeholder()` mints, and the predicate cannot tell them apart"
        );
        assert!(
            !net(entire).is_placeholder(),
            "all-entire is described (claims nothing, but is not poison)"
        );

        // One channel empty, the others point brackets: described.
        let mixed = surface_from(
            (0..4)
                .map(|i| {
                    Point3::new(
                        empty,
                        Interval::from_f64(1.0),
                        Interval::from_f64(f64::from(i)),
                    )
                })
                .collect(),
        );
        assert!(!mixed.is_placeholder());
    }
}

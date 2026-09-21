//! The certified [`Interval`] scalar is nameable, constructible and
//! **decides** in a DEFAULT build: `geom_core::interval` compiles with
//! the `interval` cargo feature OFF, which gates the kernel's
//! instantiation at the scalar (the lane impls above this crate and the
//! interval test files) and not the scalar itself.
//!
//! Deliberately NOT `#![cfg(feature = "interval")]`: a gated file would
//! say nothing about the build this suite is a claim about. Deliberately
//! ε-free as well — every band here is a literal [`Band::new`] and no
//! row reads the global `Tolerance`, so the suite reads identically at
//! every eps row and needs no process of its own.
//!
//! The rows are the doors this crate ungated, one each: the type and
//! its arithmetic, the [`Decide`] impl, the `bit_identity` arm and the
//! sealed [`SpanLocate`] impl — plus [`DualInterval`], the
//! dual-over-interval alias.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::{KnotVector, SpanLocate};
use geom_core::{Band, Bounds, Decide, Dual, DualInterval, Interval, Real, Sign};

/// Construction, arithmetic and the certified decision door, feature
/// off. The band is 1e-9/1e-8 and the values are orders away from it in
/// each direction, so no row here is a tolerance claim.
#[test]
fn interval_constructs_decides_and_encloses_in_a_default_build() {
    let band = Band::new(1e-9, 1e-8).unwrap();

    let x = Interval::from_f64(1e-3);
    assert_eq!(Bounds::lo(x), 1e-3, "from_f64 is the point enclosure");
    assert_eq!(Bounds::hi(x), 1e-3, "from_f64 is the point enclosure");
    assert_eq!(x.sign_within(band), Ok(Sign::Positive));
    assert_eq!(Interval::from_f64(0.0).sign_within(band), Ok(Sign::Zero));
    assert_eq!(Interval::from_f64(-1e-3).sign_within(band), Ok(Sign::Negative));

    // The arithmetic encloses: the true 1e-6 lies inside the product's
    // endpoints, which outward rounding may widen but never lose.
    let sq = x * x;
    assert!(
        Bounds::lo(sq) <= 1e-6 && 1e-6 <= Bounds::hi(sq),
        "[{}, {}] must enclose 1e-6",
        Bounds::lo(sq),
        Bounds::hi(sq)
    );

    // An enclosure straddling both thresholds has no certifiable sign —
    // the escalation the enclosure scalar exists to produce.
    assert!(
        Interval::from_bounds(-1e-3, 1e-3)
            .sign_within(band)
            .is_err(),
        "an enclosure spanning zero cannot certify a sign"
    );
}

/// The `repr_bits` arm for `Interval` is compiled, so the identity
/// channel answers `Some` at this scalar. A gated-out arm falls through
/// to `None` — the answer a channel-less scalar gives — and that is the
/// value this row would catch.
#[test]
fn the_bit_identity_channel_reads_intervals_in_a_default_build() {
    let a = Interval::from_f64(2.0);
    let b = Interval::from_f64(2.0);
    let c = Interval::from_f64(3.0);
    assert_eq!(geom_core::bit_identity::eq_bits(&a, &b), Some(true));
    assert_eq!(geom_core::bit_identity::eq_bits(&a, &c), Some(false));
}

/// The sealed [`SpanLocate`] impl is compiled, and it is the
/// interval-natured one: an enclosure straddling the interior knot
/// overlaps BOTH spans, where a point scalar's locator answers one.
#[test]
fn interval_locates_a_span_range_in_a_default_build() {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
    let straddling = Interval::from_bounds(0.5, 1.5).locate_spans(&knots);
    assert_eq!(
        (straddling.first.index(), straddling.last.index()),
        (2, 3),
        "[0.5, 1.5] straddles the knot at 1.0"
    );
    let inside = Interval::from_f64(0.5).locate_spans(&knots);
    assert_eq!((inside.first.index(), inside.last.index()), (2, 2));
}

/// `DualInterval` — the dual-over-interval alias Q1 ratifies — names a
/// type in a default build, and its two channels carry their own
/// enclosures.
#[test]
fn dual_interval_is_nameable_in_a_default_build() {
    let d: DualInterval = Dual::new(Interval::from_f64(2.0), Interval::from_f64(1.0));
    assert_eq!(Bounds::lo(d.value), 2.0);
    assert_eq!(Bounds::hi(d.deriv), 1.0);
}

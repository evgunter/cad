//! RING-2 review probe (R1): the crossing carries the decoration, and
//! what a consumer that does NOT ask the refusal by name would read.

#![cfg(feature = "interval")]

use geom_core::{Interval, Real, RingInterval};

#[test]
fn ring2_r1_the_crossing_carries_real_endpoints_at_trv() {
    let s = Interval::from_bounds(-1.0, 4.0).sqrt();
    let r = RingInterval::from_certified(s);
    println!(
        "R1CROSS sqrt(-1,4) ring lo={:?} hi={:?} poison={}",
        r.lo(),
        r.hi(),
        r.is_poison()
    );
    assert!(r.is_poison(), "the crossing must refuse a Trv scalar");
    assert!(
        r.lo().is_finite() && r.hi().is_finite(),
        "and keep its endpoints"
    );
    assert!(r.hi() <= 2.0 && r.lo() >= 0.0);

    let ok = RingInterval::from_certified(Interval::from_bounds(1.0, 4.0).sqrt());
    println!(
        "R1CROSS sqrt(1,4) ring lo={:?} hi={:?} poison={}",
        ok.lo(),
        ok.hi(),
        ok.is_poison()
    );
    assert!(!ok.is_poison());

    let good = RingInterval::from_bounds(0.0, 1.0);
    assert!(
        RingInterval::hull(r, good).is_poison(),
        "hull keeps the refusal"
    );
    assert!(RingInterval::hull(good, r).is_poison(), "from either side");
    assert!(r.clamped_to(0.0, 1.0).is_poison(), "clamp keeps the refusal");
    assert!(r.width().is_nan() && r.mag().is_nan() && !r.contains(1.0));
    for x in [
        r + good,
        good - r,
        r * good,
        good / r,
        -r,
        r.sqr(),
        r.powi(2),
        r.powi(0),
    ] {
        assert!(x.is_poison());
    }
}

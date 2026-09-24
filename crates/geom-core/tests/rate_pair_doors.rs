//! **The rate pair's conversions are the bare arithmetic.**
//!
//! `SupSpeed`/`InfSpeed` carry a bound DIRECTION, not a computation:
//! `to_meters(span) = span * s` and `to_param(m) = m / s` are one
//! operation each, so typing a shipped site moves no margin's bits
//! (D9, and the same acceptance every `Margin` door was rolled out
//! under). These rows are what makes that a checked claim rather than
//! a sentence: each drives its door against the bare operator over a
//! sample that includes subnormals, both infinities, both signed
//! zeros and NaN, and compares BITS — `to_bits`, not `==`, because
//! `==` cannot see a signed zero and calls every NaN unequal.
//!
//! **Poison flows through values, not around them.** The pair is a
//! tag, not a positivity witness, so a NaN or zero rate must arrive at
//! the site's own guard exactly as it would have before; a door that
//! sanitized one would hide a collapsed carrier from the trilean that
//! is supposed to escalate on it. The NaN and zero entries in the
//! sample are that claim.
//!
//! The `Interval` row is the certified lane's: there the doors are the
//! interval product and quotient, so it pins TWO things — the
//! enclosure property (the answer brackets the real product/quotient
//! of points drawn from the operands), which a widened but still
//! enclosing door would also pass, and the `repr_bits` identity with
//! the bare `*` and `/`, which is the half that catches a widening. A
//! bit comparison against an `f64` operator is not the claim there and
//! is not made.
//!
//! **What no row here can see, stated because it looks like a gap.**
//! Commuting a door's product to `rate * span` reds nothing, at either
//! scalar: IEEE multiplication is commutative on every value in the
//! sample, signed zeros and NaN included, so the two spellings are
//! bit-equal by the standard and no sample can tell them apart. "One
//! operation" is what these rows pin; "this operand order" is not a
//! property they can carry, and adding a sample would not give them
//! one.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use geom_core::{InfSpeed, Margin, SupSpeed};

/// Every value a rate or a span can be, including the ones that only
/// a bit comparison can tell apart. Deliberately written down rather
/// than drawn: this is a witness set, not a counterexample search
/// (`memories/test-suite-cost.md`), so it is the same set every run.
fn sample() -> Vec<f64> {
    vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        3.0,
        1e-9,
        1e9,
        f64::MIN_POSITIVE,
        f64::MIN_POSITIVE / 2.0, // subnormal
        -f64::MIN_POSITIVE / 2.0,
        f64::MIN_POSITIVE * 1.5, // subnormal product/quotient territory
        f64::MAX,
        f64::MIN,
        f64::EPSILON,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
    ]
}

/// `a` and `b` are the same `f64` down to the bits — so `-0.0` is not
/// `0.0` and NaN is itself, both of which `==` gets wrong.
fn same_bits(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits()
}

#[test]
fn sup_to_meters_is_the_bare_product() {
    for &span in &sample() {
        for &rate in &sample() {
            let door = SupSpeed::new(rate).to_meters(span);
            assert!(
                same_bits(door, span * rate),
                "SupSpeed::to_meters({span:e}, {rate:e}) = {door:e}, bare = {:e}",
                span * rate
            );
            // And the door that takes it computes nothing further.
            assert!(same_bits(
                Margin::metered_sup(span, SupSpeed::new(rate)).value(),
                span * rate
            ));
        }
    }
}

#[test]
fn inf_to_meters_is_the_bare_product() {
    for &span in &sample() {
        for &rate in &sample() {
            let door = InfSpeed::new(rate).to_meters(span);
            assert!(
                same_bits(door, span * rate),
                "InfSpeed::to_meters({span:e}, {rate:e}) = {door:e}, bare = {:e}",
                span * rate
            );
            assert!(same_bits(
                Margin::metered(span, InfSpeed::new(rate)).value(),
                span * rate
            ));
        }
    }
}

#[test]
fn sup_to_param_is_the_bare_quotient() {
    for &meters in &sample() {
        for &rate in &sample() {
            let door = SupSpeed::new(rate).to_param(meters);
            assert!(
                same_bits(door, meters / rate),
                "SupSpeed::to_param({meters:e}, {rate:e}) = {door:e}, bare = {:e}",
                meters / rate
            );
        }
    }
}

/// The tag is not a witness: a rate of zero, of infinity or of poison
/// goes in and comes out, so each site's own guard sees what it always
/// saw. Named separately from the bit rows above because it is a
/// different claim — those say the door computes the same thing, this
/// says the door refuses to sanitize.
#[test]
fn a_poisoned_or_collapsed_rate_passes_straight_through() {
    for rate in [0.0_f64, -0.0, f64::INFINITY, f64::NAN] {
        assert!(same_bits(SupSpeed::new(rate).get(), rate));
        assert!(same_bits(InfSpeed::new(rate).get(), rate));
        // A poison rate poisons the metred margin (the classifier then
        // answers `Invalid`); a zero rate collapses it to zero or NaN
        // exactly as the bare product does.
        assert!(same_bits(
            Margin::metered(1.0, InfSpeed::new(rate)).value(),
            1.0 * rate
        ));
        assert!(same_bits(SupSpeed::new(rate).to_param(1.0), 1.0_f64 / rate));
    }
}

/// The certified lane: the doors are the interval product and
/// quotient, so this row pins the ENCLOSURE property — the answer
/// contains the real product (quotient) of any point of the span and
/// any point of the rate — AND the `repr_bits` identity with the bare
/// ops, which is what a widened-but-enclosing door would fail. The
/// zero, straddling and poison rates at the end are the certified
/// lane's half of the pass-through claim.
#[cfg(feature = "interval")]
#[test]
fn the_doors_are_the_interval_ops() {
    use geom_core::{Bounds, Interval, Real};

    let spans = [(-1.0, 2.0), (0.5, 0.5), (0.0, 1e-9), (-3.0, -1.0)];
    let rates = [(1.0, 4.0), (0.25, 0.25), (1e6, 2e6)];
    for (slo, shi) in spans {
        for (rlo, rhi) in rates {
            let span = Interval::from_bounds(slo, shi);
            let rate = Interval::from_bounds(rlo, rhi);
            let meters = SupSpeed::new(rate).to_meters(span);
            let back = SupSpeed::new(rate).to_param(span);
            for s in [slo, shi, f64::midpoint(slo, shi)] {
                for r in [rlo, rhi, f64::midpoint(rlo, rhi)] {
                    assert!(
                        meters.lo() <= s * r && s * r <= meters.hi(),
                        "to_meters must enclose {s} * {r}"
                    );
                    assert!(
                        back.lo() <= s / r && s / r <= back.hi(),
                        "to_param must enclose {s} / {r}"
                    );
                }
            }
            // And the metric doors are those same two operations.
            assert_eq!(
                Margin::metered_sup(span, SupSpeed::new(rate))
                    .value()
                    .repr_bits(),
                (span * rate).repr_bits()
            );
            assert_eq!(
                Margin::metered(span, InfSpeed::new(rate))
                    .value()
                    .repr_bits(),
                (span * rate).repr_bits()
            );
            // The parameter side is the bare quotient in the same
            // sense — the same representation, not merely an
            // enclosure that happens to contain it.
            assert_eq!(
                SupSpeed::new(rate).to_param(span).repr_bits(),
                (span / rate).repr_bits()
            );
        }
    }

    // A collapsed or poisoned rate reaches the site's own guard at the
    // certified scalar too: an interval straddling zero, one pinned at
    // zero and a poisoned one all pass through both doors as the bare
    // operation's own answer.
    let span = Interval::from_bounds(1.0, 2.0);
    for rate in [
        Interval::from_bounds(-1.0, 1.0),
        Interval::from_f64(0.0),
        // `from_f64` maps a non-real to NaI, which is the certified
        // lane's poison.
        Interval::from_f64(f64::NAN),
    ] {
        assert_eq!(
            Margin::metered_sup(span, SupSpeed::new(rate))
                .value()
                .repr_bits(),
            (span * rate).repr_bits()
        );
        assert_eq!(
            SupSpeed::new(rate).to_param(span).repr_bits(),
            (span / rate).repr_bits()
        );
    }
}

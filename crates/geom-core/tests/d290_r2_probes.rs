//! D290 review lane R2's probes for `KnotVector::on_domain`.
//!
//! Independent of the unit's own rows: each row here derives its
//! expectation from the door's stated contract rather than from the
//! implementation's expression.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::spline::{KnotVector, KnotVectorIssue, SplineError};

fn kv(k: &[f64], p: usize) -> KnotVector {
    KnotVector::clamped(k.to_vec(), p).expect("fixture is valid")
}

fn bits(v: &[f64]) -> Vec<u64> {
    v.iter().map(|x| x.to_bits()).collect()
}

/// R2-P1. The door is NOT the identity on its own domain when that
/// domain is not `[0, 1]`: `a + (b - a)*((k - a)/(b - a))` re-rounds
/// every interior knot. The ends still come back exact (they are
/// assigned), so the drift is interior-only.
#[test]
fn on_domain_onto_its_own_domain_is_not_the_identity_off_the_unit_interval() {
    // On the unit domain the map is exactly the identity:
    // k - 0.0 = k and k / 1.0 = k, both exact for every finite k.
    let unit = kv(&[0.0, 0.0, 0.0, 0.3, 0.7, 1.0, 1.0, 1.0], 2);
    assert_eq!(
        bits(unit.on_domain(0.0, 1.0).unwrap().knots()),
        bits(unit.knots()),
        "on the unit domain the map is k -> 0 + 1*((k - 0)/1) = k"
    );

    // Off it, it is not: found by search over 2e5 random (a, b, k)
    // triples, where it fails for roughly one in 1500.
    let (a, b) = (7.345_351_394_140_087e-5, 0.013_328_300_386_333_754);
    let interior = 0.000_521_411_171_497_765_2;
    let src = kv(&[a, a, interior, b, b], 1);
    let out = src.on_domain(a, b).unwrap();
    assert_eq!(out.domain(), (a, b), "ends are assigned, so they survive");
    assert_ne!(
        out.knots()[2].to_bits(),
        interior.to_bits(),
        "the interior knot drifts one ulp under a round trip through its OWN domain"
    );
    assert_eq!(
        i128::from(out.knots()[2].to_bits()) - i128::from(interior.to_bits()),
        1
    );
}

/// R2-P2. A finite `[lo, hi]` whose WIDTH overflows to `+inf`
/// (`lo = -f64::MAX`, `hi = f64::MAX`) passes the `DomainInvalid`
/// guard - both ends are finite and increasing - and the interior
/// images become non-finite. The door must still refuse, not mint.
#[test]
fn on_domain_refuses_a_domain_whose_width_overflows() {
    let src = kv(&[0.0, 0.0, 0.5, 1.0, 1.0], 1);
    let err = src
        .on_domain(-f64::MAX, f64::MAX)
        .expect_err("an infinite span cannot produce a finite vector");
    assert!(
        matches!(err, SplineError::KnotVectorInvalid { .. }),
        "{err:?}"
    );
}

/// R2-P3. The narrowest non-refused domain: a span of one ulp
/// collapses every interior knot onto an end, so the door refuses
/// through a clamp clause - it never mints a vector whose interior
/// equals an end.
#[test]
fn on_domain_on_an_ulp_wide_domain_refuses_through_a_clamp_clause() {
    let src = kv(&[0.0, 0.0, 0.5, 1.0, 1.0], 1);
    let lo = 1.0_f64;
    let hi = f64::from_bits(lo.to_bits() + 1);
    let err = src.on_domain(lo, hi).expect_err("one ulp of room");
    assert!(
        matches!(
            err,
            SplineError::KnotVectorInvalid {
                reason: KnotVectorIssue::StartNotClamped
                    | KnotVectorIssue::EndNotClamped
                    | KnotVectorIssue::InteriorMultiplicityTooHigh { .. }
                    | KnotVectorIssue::Decreasing { .. }
            }
        ),
        "{err:?}"
    );
}

/// R2-P4. Signed zero: `(-0.0, 0.0)` is a COLLAPSED domain (`-0.0 < 0.0`
/// is false), and the door names it as such rather than admitting it
/// and tripping a clamp clause.
#[test]
fn on_domain_treats_negative_zero_to_zero_as_collapsed() {
    let src = kv(&[0.0, 0.0, 0.5, 1.0, 1.0], 1);
    assert!(matches!(
        src.on_domain(-0.0, 0.0),
        Err(SplineError::KnotVectorInvalid {
            reason: KnotVectorIssue::DomainInvalid { .. }
        })
    ));
    let out = src.on_domain(-0.0, 1.0).unwrap();
    assert_eq!(out.domain().0.to_bits(), (-0.0_f64).to_bits());
}

/// R2-P5. Equal source values can never split: the door preserves
/// interior multiplicity wherever it returns at all.
#[test]
fn on_domain_never_splits_an_equal_pair() {
    let src = kv(&[0.0, 0.0, 0.0, 0.25, 0.25, 0.6, 1.0, 1.0, 1.0], 2);
    for (lo, hi) in [(0.3, 0.9), (-7.5, 12.25), (1e-8, 3e-8), (1e300, 2e300)] {
        let out = src.on_domain(lo, hi).unwrap();
        let k = out.knots();
        assert_eq!(k[3].to_bits(), k[4].to_bits(), "({lo}, {hi})");
        assert_eq!(out.control_count(), src.control_count());
        assert_eq!(out.degree(), src.degree());
        assert_eq!(out.knots().len(), src.knots().len());
    }
}

/// R2-P6. The ends pin is not specific to one literal: a search over
/// many pairs finds plenty where the computed end misses, and the door
/// lands on `hi` bit for bit at every one of them.
#[test]
fn on_domain_pins_the_ends_over_a_search_not_one_literal() {
    let src = kv(&[0.0, 0.0, 0.0, 0.25, 0.5, 1.0, 1.0, 1.0], 2);
    let mut misses = 0_u32;
    for i in 1..400_u32 {
        // lo and hi are built INDEPENDENTLY: if hi were derived as
        // `lo + w`, `hi - lo` would recover `w` exactly and the row
        // would never exercise the pin.
        let lo = f64::from(i) / 7.0;
        let hi = f64::from(i) / 3.0 + 1.0;
        assert!(lo.is_finite() && hi.is_finite() && lo < hi);
        if (lo + (hi - lo)).to_bits() != hi.to_bits() {
            misses += 1;
        }
        let out = src.on_domain(lo, hi).unwrap();
        let (olo, ohi) = out.domain();
        assert_eq!(
            (olo.to_bits(), ohi.to_bits()),
            (lo.to_bits(), hi.to_bits()),
            "({lo}, {hi})"
        );
    }
    assert!(
        misses > 0,
        "the search found no pair where the computed end misses"
    );
}

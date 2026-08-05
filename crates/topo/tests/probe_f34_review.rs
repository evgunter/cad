//! Adversarial review probes for the F3+F4 dimensional unit (review/f34),
//! ADOPTED BY MERGE into the unit — authorship kept, conclusions updated
//! where the fix pass moved them.
//!
//! T2 found the unit's MAJOR: the area-normalized backstop margin
//! DEMOTES a macroscopic localized wrong-component defect to
//! indeterminate (pass) on a large-area body, where the old raw-volume
//! comparand refused decisively. The finding stands exactly as measured
//! — and the fix pass answered it with the dual-arm gate (a
//! sign-certain inequality violation refuses through the exact band,
//! whatever its metered magnitude). This file keeps the reviewer's
//! demonstration of the demotion AND pins the arm that now catches it;
//! the end-to-end refusal through the real gate is
//! `ops::tests::volume_backstop_refuses_a_wrong_component_hidden_by_a_large_area`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Band, Decide, Sign};

#[test]
fn t2_wrong_component_hides_behind_large_area() {
    // Default band (the one the mm-skip finding was measured against).
    let band = Band::new(1e-9, 1e-8).expect("band");
    // A wrongly-kept 3 mm cube on a 2 m x 2 m x 0.1 m plate:
    // dV = 2.7e-8 m^3, summed areas ~ 17.6 m^2.
    let dv = 2.7e-8_f64;
    let areas = 17.6_f64;
    // OLD comparand (raw volume margin): decisively negative -> REFUSED.
    assert_eq!((-dv).sign_within(band), Ok(Sign::Negative));
    // NEW comparand (mean displacement): lands IN BAND -> indeterminate,
    // so a MAGNITUDE-ONLY gate would take `Err(_) => Ok(())` and PASS
    // the violation. This is the review's finding, unretouched.
    let metered = -dv / areas; // -1.53e-9
    assert!(
        (-metered) > 1e-9 && (-metered) < 1e-8,
        "in band: {metered:e}"
    );
    assert!(
        (metered).sign_within(band).is_err(),
        "indeterminate -> a magnitude-only backstop passes"
    );
    // Smaller wrong component (1 mm cube, 1e-9 m^3): metered margin
    // 5.7e-11 < band.zero -> certified Zero -> would pass as exact.
    assert_eq!((-1e-9 / areas).sign_within(band), Ok(Sign::Zero));

    // THE RESOLUTION (fix pass): the bound is an INEQUALITY, so the
    // gate also asks the dimension-free question — is the sign certain?
    // — against the exact bit-hairline band, where no epsilon enters.
    // Both defects above are decisively negative there, so both refuse.
    let exact = Band::new(f64::from_bits(1), f64::from_bits(2)).expect("exact band");
    assert_eq!(metered.sign_within(exact), Ok(Sign::Negative));
    assert_eq!((-1e-9 / areas).sign_within(exact), Ok(Sign::Negative));
    // And the non-strict pass direction is untouched: an exactly-equal
    // result is Zero at the hairline, not a violation.
    assert_eq!(0.0_f64.sign_within(exact), Ok(Sign::Zero));
    // The sign question is invariant under the positive lever — which
    // is why arm 1 may consume the metered length and keep the recorded
    // margin dimensionally honest.
    assert_eq!(
        (-dv).sign_within(exact),
        metered.sign_within(exact),
        "dividing by a positive area must not move the sign"
    );
}

#[test]
fn t1_poison_and_sign_preservation() {
    let band = Band::new(1e-9, 1e-8).expect("band");
    // Deviation 1's semantics: 0/0 -> NaN -> Invalid poison (typed).
    let nan = f64::NAN; // the 0/0 the zero-perimeter run produces
    let e = nan.sign_within(band).unwrap_err();
    assert!(matches!(e.margin, geom_core::MarginDiag::Invalid));
    // Complement operand: area is unsigned (props.rs), so V/A keeps
    // V's sign -- a negative flux volume still classifies Negative.
    assert_eq!((-8.0_f64 / 24.0).sign_within(band), Ok(Sign::Negative));
    // Thin-sliver operand: V/A -> mean thickness below eps -> Zero or
    // in-band -> "not certifiably bounded" -> bound skipped (the skip
    // zone survives, now dimensionally honest: sub-resolution THICKNESS).
    assert_eq!((4e-10_f64).sign_within(band), Ok(Sign::Zero));
}

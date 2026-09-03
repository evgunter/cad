//! R1 review probe — adversarial points for the CERT-4 claim that
//! `reduce_periodic` and `reduce_periodic_centred` agree BITWISE on
//! `[0, pi)`.
//!
//! The unit's own row samples `k/4000 * (TAU/2)` for k in 0..4000, so
//! its largest sample is `3999/4000 * pi`; the identity row multiplies
//! its samples by `0.999_999`. Both shave exactly the top of the range.
//! These rows walk the actual floats there.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Real;

const TAU: f64 = core::f64::consts::TAU;
const PI: f64 = core::f64::consts::PI;

fn nextbelow(x: f64, n: u32) -> f64 {
    let mut v = x;
    for _ in 0..n {
        v = f64::from_bits(v.to_bits() - 1);
    }
    v
}

/// FALSIFIES the PR body's "the two windows agree BITWISE on [0, pi)".
/// They do not: at the top of the half-open range the centred window's
/// `x/tau + 0.5` rounds up to exactly 1.0 (round-half-to-even), its
/// floor is 1 rather than 0, and the reduction returns `x - tau`.
/// The fit margin `extent - setback` is then TAU, not 0.
#[test]
fn r1_the_two_windows_disagree_at_the_top_of_the_shared_interior() {
    let mut divergent = Vec::new();
    for n in 1..4096u32 {
        let x = nextbelow(PI, n);
        let ext = <f64 as Real>::reduce_periodic(x, TAU);
        let sgn = <f64 as Real>::reduce_periodic_centred(x, TAU);
        if ext.to_bits() != sgn.to_bits() {
            divergent.push((n, x, ext - sgn));
        }
    }
    // Reviewer's observed answer at this head: exactly one float
    // strictly below pi diverges, and it diverges by a whole period.
    assert_eq!(
        divergent.len(),
        1,
        "expected exactly one divergent float below pi, got {divergent:?}"
    );
    let (n, x, margin) = divergent[0];
    assert_eq!(n, 1, "the divergence is at nextbelow(pi)");
    assert_eq!(margin, TAU, "the fit margin is a whole period, not zero");
    assert!(x < PI, "and the point is strictly inside [0, pi)");
}

/// The identity row's own doc claims the centred window returns its
/// argument bitwise "across the whole of (-pi, pi)". It does not, at
/// the same point.
#[test]
fn r1_the_centred_window_is_not_the_identity_at_the_top_of_its_interior() {
    let x = nextbelow(PI, 1);
    let back = <f64 as Real>::reduce_periodic_centred(x, TAU);
    assert_ne!(back.to_bits(), x.to_bits());
    assert!(back < 0.0, "it comes back on the other branch: {back}");
}

/// Adversarial points the unit's rows do not carry: negative zero,
/// denormals, and the smallest normals. These DO behave.
#[test]
fn r1_denormals_and_negative_zero_round_trip() {
    for x in [
        -0.0f64,
        0.0,
        f64::MIN_POSITIVE,
        -f64::MIN_POSITIVE,
        5e-324,
        -5e-324,
        1e-308,
    ] {
        let back = <f64 as Real>::reduce_periodic_centred(x, TAU);
        assert_eq!(back.to_bits(), x.to_bits(), "x = {x:e} did not come back");
    }
}

/// The negative end of the centred window is clean: nothing near -pi
/// diverges from the identity, so the defect is one-sided.
#[test]
fn r1_the_negative_end_of_the_centred_window_is_clean() {
    for n in 0..4096u32 {
        let x = -nextbelow(PI, n);
        let back = <f64 as Real>::reduce_periodic_centred(x, TAU);
        assert_eq!(back.to_bits(), x.to_bits(), "x = {x:e} moved");
    }
}

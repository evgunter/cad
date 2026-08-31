//! The centred period fold, at `f64` and at `Interval` — the class rows
//! for issue 1191.
//!
//! [`Real::reduce_periodic`] and [`Real::reduce_periodic_centred`] are
//! the same construction with their jumps in different places, and
//! which jump a call site gets is the whole of the defect this suite
//! pins. Both are honest: a `floor` is a step function and a box
//! spanning one of its steps encloses two integers, so the reduction
//! comes back a whole period wide. The rows below say where each
//! window's step is, that the two agree bitwise on the interior they
//! share, and — the one that matters at interval type — that folding a
//! raw difference ONCE keeps a hairline a hairline where folding it
//! through `[0, τ)` first does not.
//!
//! **No row here consults a tolerance.** The width ceilings are widths,
//! not bands: they separate "the enclosure is as wide as its input"
//! from "the enclosure is as wide as the period" by twelve orders, and
//! nothing about any verdict moves with the run's ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Real;

const TAU: f64 = core::f64::consts::TAU;

/// The centred window returns its argument **bitwise** on its
/// interior — and this row pins where that interior actually ENDS,
/// which is not where the mathematics says.
///
/// The identity holds exactly while `fl(fl(x/τ) + ½) < 1`. Rounding
/// carries that sum up to `1` a float or two below the half period,
/// so the top of `(−π, π)` is NOT in the identity's domain. The
/// boundary rows below assert what is true at each named float rather
/// than sampling a grid that steps over them: a `×0.999_999` grid
/// never lands within an ulp of π, which is precisely how a claim one
/// ulp too wide survives a suite.
#[test]
fn the_centred_window_is_the_identity_up_to_its_rounding_boundary() {
    let mut xs = vec![0.0f64, -0.0, 1e-300, -1e-300, f64::EPSILON, -f64::EPSILON];
    for k in 1..2000 {
        let t = f64::from(k) / 2000.0;
        xs.push(t * (TAU / 2.0) * 0.999_999);
        xs.push(-t * (TAU / 2.0) * 0.999_999);
    }
    for x in xs {
        assert_eq!(
            <f64 as Real>::reduce_periodic_centred(x, TAU).to_bits(),
            x.to_bits(),
            "x = {x:e} did not come back bitwise"
        );
    }
}

/// **The boundary, pinned float by float.** `π`, the float below it and
/// the float below THAT — the identity fails on the top two and holds
/// on the third, and the failure is the honest output of the written
/// formula, not a defect to branch around in evaluation code.
#[test]
fn the_centred_windows_identity_ends_two_floats_below_pi() {
    let pi = core::f64::consts::PI;
    let below1 = f64::from_bits(pi.to_bits() - 1);
    let below2 = f64::from_bits(pi.to_bits() - 2);

    // `fl(π/τ)` is exactly ½ — τ is `2·fl(π)` and doubling is exact —
    // so the sum is exactly 1 and the floor steps.
    assert_eq!(pi / TAU, 0.5);
    assert_eq!(pi / TAU + 0.5, 1.0);
    assert_eq!(<f64 as Real>::periodic_branch(pi, TAU), 1.0);
    assert_eq!(<f64 as Real>::reduce_periodic_centred(pi, TAU), pi - TAU);

    // One float lower the quotient rounds DOWN, and `+ ½` rounds
    // half-to-even back up to exactly 1.0. Same branch.
    assert_eq!(below1 / TAU, 0.499_999_999_999_999_94);
    assert_eq!(below1 / TAU + 0.5, 1.0);
    assert_eq!(<f64 as Real>::periodic_branch(below1, TAU), 1.0);

    // Two floats lower it does not, and the identity holds.
    assert_eq!(below2 / TAU + 0.5, 0.999_999_999_999_999_8);
    assert_eq!(<f64 as Real>::periodic_branch(below2, TAU), 0.0);
    assert_eq!(
        <f64 as Real>::reduce_periodic_centred(below2, TAU).to_bits(),
        below2.to_bits()
    );
}

/// The two windows agree **bitwise** on the interior they share, and
/// this row states that interior as the rounding condition it really
/// is: `[0, π)` MINUS its top ulp. An extent taken with
/// [`Real::reduce_periodic`] and a setback taken with
/// [`Real::reduce_periodic_centred`] from the SAME difference subtract
/// to exactly zero there — which is the fillet fit gate's structural
/// zero — and on the two floats above it they differ by a whole
/// period, which is what the gate's callers are entitled to know.
///
/// The exhaustive half is the point: the grid this row used to be
/// could not see the boundary at all.
#[test]
fn the_two_windows_agree_bitwise_below_the_boundary_and_not_at_it() {
    for k in 0..4000 {
        let x = f64::from(k) / 4000.0 * (TAU / 2.0);
        let extent = <f64 as Real>::reduce_periodic(x, TAU);
        let signed = <f64 as Real>::reduce_periodic_centred(x, TAU);
        assert_eq!(extent.to_bits(), signed.to_bits(), "x = {x:e}");
        assert_eq!(
            extent - signed,
            0.0,
            "the fit margin is not structurally zero"
        );
    }

    // Exhaustive over the top 4097 floats of [0, π]: exactly two
    // diverge, and they are the top two.
    let pi = core::f64::consts::PI;
    let mut diverged = Vec::new();
    for b in (pi.to_bits() - 4096)..=pi.to_bits() {
        let x = f64::from_bits(b);
        let extent = <f64 as Real>::reduce_periodic(x, TAU);
        let signed = <f64 as Real>::reduce_periodic_centred(x, TAU);
        if extent.to_bits() != signed.to_bits() {
            diverged.push(x);
            assert_eq!(
                extent - signed,
                TAU,
                "a divergence at {x:?} that is not a whole period"
            );
        }
    }
    assert_eq!(
        diverged,
        vec![f64::from_bits(pi.to_bits() - 1), pi],
        "the divergent set is the top two floats of [0, pi] and nothing else"
    );
}

/// **The signed-zero caveat**, pinned rather than left to be
/// rediscovered: the two windows return oppositely-signed zeros at
/// `−0.0`. Equal in value — so a margin built from them is `0` and
/// classifies Zero either way — and NOT bit-identical, which is why
/// the agreement claim above is about `[0, …)` and says "bitwise".
///
/// `−0.0` is reachable, not hypothetical: a zero swept angle on a
/// clockwise leg is `(+0.0)·(−1.0)`.
#[test]
fn the_two_windows_return_oppositely_signed_zeros_at_negative_zero() {
    // The multiplication is the SUBJECT of this row, not a clumsy way
    // to write a literal: `(+0.0)·turn` with a clockwise `turn = −1` is
    // how a zero swept angle acquires a negative sign in the fillet
    // path. Spelling `-0.0` here would assert the caveat while deleting
    // the evidence that anything reaches it.
    #[allow(clippy::neg_multiply)]
    let neg_zero = 0.0f64 * -1.0f64;
    assert_eq!(neg_zero.to_bits(), (-0.0f64).to_bits(), "the route is real");

    let extent = <f64 as Real>::reduce_periodic(neg_zero, TAU);
    let signed = <f64 as Real>::reduce_periodic_centred(neg_zero, TAU);
    assert_eq!(
        extent.to_bits(),
        0.0f64.to_bits(),
        "the extent window gives +0"
    );
    assert_eq!(
        signed.to_bits(),
        (-0.0f64).to_bits(),
        "the centred one gives -0"
    );
    assert_eq!(extent, signed, "equal in value");
    assert_eq!(extent - signed, 0.0, "so the margin is structurally zero");
}

/// The branch pin: `raw + (near − raw).periodic_branch(p)·p` lands on
/// the branch of `raw` nearest `near`, i.e. within half a period of it.
#[test]
fn the_branch_pin_lands_within_half_a_period_of_its_reference() {
    for i in -50..50 {
        for j in -37..37 {
            let raw = f64::from(i) * 0.137;
            let near = f64::from(j) * 2.9;
            let k = <f64 as Real>::periodic_branch(near - raw, TAU);
            let pinned = raw + k * TAU;
            assert!(
                (pinned - near).abs() <= TAU / 2.0 + 1e-12,
                "raw {raw} pinned to {pinned}, {} from {near}",
                pinned - near
            );
        }
    }
}

#[cfg(feature = "interval")]
mod interval {
    use super::TAU;
    use geom_core::{Bounds, Interval, Real};

    fn iv(lo: f64, hi: f64) -> Interval {
        Interval::from_bounds(lo, hi)
    }

    /// **The defect, and its fix, measured side by side on one box.**
    ///
    /// A hairline box straddling zero — the shape an `atan2` difference
    /// of two separately-rounded coincident points takes — comes out of
    /// the centred fold at its own width, and out of the composed
    /// spelling (`[0, τ)` first, then the centred fold) at twice the
    /// period. The composed spelling is computed inline here rather
    /// than kept anywhere in the tree: it is the shape this row exists
    /// to keep out.
    #[test]
    fn a_box_straddling_zero_keeps_its_width_and_the_composition_does_not() {
        let tau = Interval::tau();
        for w in [1e-16, 1e-12, 1e-8, 1e-3] {
            let x = iv(-w, w);
            let direct = x.reduce_periodic_centred(tau);
            let direct_w = direct.hi() - direct.lo();
            assert!(
                direct_w <= 4.0 * w + 1e-15,
                "the centred fold widened a {w:e} box to {direct_w:e}"
            );

            let forward = x.reduce_periodic(tau);
            let composed = forward.reduce_periodic_centred(tau);
            let composed_w = composed.hi() - composed.lo();
            assert!(
                composed_w >= TAU,
                "the composed spelling is supposed to be the wide one; it gave \
                 {composed_w:e} on a {w:e} box"
            );
        }
    }

    /// The centred fold's OWN jump, at ±τ/2, widens honestly — a box
    /// straddling it encloses both signs of the representative, because
    /// the representative genuinely takes both there. Stated so the row
    /// above is not read as a claim that the fold never widens.
    #[test]
    fn the_centred_folds_own_jump_widens_and_that_is_the_honest_answer() {
        let tau = Interval::tau();
        let x = iv(TAU / 2.0 - 1e-12, TAU / 2.0 + 1e-12);
        let r = x.reduce_periodic_centred(tau);
        assert!(
            r.hi() - r.lo() >= TAU,
            "a box across the half-period must enclose both signs, got [{}, {}]",
            r.lo(),
            r.hi()
        );
    }

    /// Containment: the enclosure of the centred reduction contains the
    /// `f64` reduction of a contained point. It composes exact ops only
    /// (÷, +, floor, ·, −), so this follows the same way
    /// [`Real::reduce_periodic`]'s does — pinned rather than assumed.
    #[test]
    fn the_centred_reduction_contains_the_f64_lanes_answer() {
        let tau = Interval::tau();
        for k in -500..500 {
            let a = f64::from(k) * 0.0731;
            let enc = Interval::from_f64(a).reduce_periodic_centred(tau);
            let exact = <f64 as Real>::reduce_periodic_centred(a, TAU);
            assert!(
                enc.lo() <= exact && exact <= enc.hi(),
                "[{}, {}] excludes {exact}",
                enc.lo(),
                enc.hi()
            );
        }
    }

    /// The branch index straddles two integers exactly at the tie — a
    /// reference half a period from the raw coordinate — and nowhere
    /// else. That is the configuration in which the pin has no answer,
    /// and the honest report of it is the two-integer enclosure.
    #[test]
    fn the_branch_index_is_a_singleton_away_from_the_tie_and_a_pair_at_it() {
        let tau = Interval::tau();
        let clear = Interval::from_f64(0.1).periodic_branch(tau);
        assert_eq!((clear.lo(), clear.hi()), (0.0, 0.0));

        let tie = iv(TAU / 2.0 - 1e-12, TAU / 2.0 + 1e-12).periodic_branch(tau);
        assert_eq!((tie.lo(), tie.hi()), (0.0, 1.0), "the tie must report both");
    }
}

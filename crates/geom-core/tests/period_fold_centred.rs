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

/// The centred window returns its argument **bitwise** across the whole
/// of `(−π, π)`: the floor's argument is `x/τ + ½ ∈ (0, 1)`, a constant
/// zero, so the result is `x − τ·0 = x`. That is the structural zero
/// the fillet fit gate stands on — an exact tangency reads exactly
/// zero, and reads it by construction rather than by two roundings
/// cancelling.
#[test]
fn the_centred_window_is_the_identity_on_its_interior_bitwise() {
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

/// The two windows agree **bitwise** on the interior they share,
/// `[0, π)` — which is the whole domain a fillet's setback occupies, a
/// tangent point never being more than half a turn from its corner. So
/// an extent taken with [`Real::reduce_periodic`] and a setback taken
/// with [`Real::reduce_periodic_centred`] from the SAME difference
/// subtract to exactly zero.
#[test]
fn the_two_windows_agree_bitwise_on_their_shared_interior() {
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

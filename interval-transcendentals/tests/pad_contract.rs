//! **The upper constraint on the pads**: every enclosure this crate
//! returns is at most the DERIVED number of representable steps wider
//! than the value its backend produced.
//!
//! `lib.rs`'s tightness section states a contract — arithmetic and
//! `sqrt` within 1 step per inexact endpoint, transcendentals within
//! `PAD_ULPS = 4` (`docs/derivations.md` §2) — and until this file the
//! only thing that computed with it was `Tightness::report`, which
//! prints. Containment alone cannot substitute: it gets EASIER as the
//! enclosure degrades, so a pad raised "to be safe" widened every
//! enclosure in the kernel with every lane green. These rows go red in
//! that direction, and they need no oracle and no C toolchain, so they
//! run in the tier the kernel's own pipeline runs.
//!
//! **What makes a row here go red:** any endpoint further from the
//! backend's own value than the derived pad — i.e. raising `PAD_ULPS`
//! past 4, padding an arithmetic endpoint twice, or dropping one of the
//! range clips so a bound escapes outward. The bound is `<=`, not `==`,
//! deliberately: a future tightening (a `sin` that skips the pad on a
//! provably exact endpoint, say) must pass, not fail.
//!
//! **What it cannot see:** the other direction. A pad that is too SMALL
//! is a containment defect, and for the transcendentals the truth is not
//! computable without a multi-precision reference — that is
//! `certify.rs`' job, under `oracle-certify`. For `× ÷ sqrt` the cheap
//! tier does see it, exactly, in `review_fuzz_exact.rs`.
//!
//! Every input to a PAD row below is chosen in the interior of its
//! function's ordinary regime — no extremum capture, no pole, no domain clamp, no
//! range clip — because those are the documented paths that widen on
//! purpose. The rows assert that no clip or capture fired rather than
//! skipping when one does, so a change that starts clipping here is
//! visible.
//!
//! **Two limits of this instrument, stated here rather than in a PR
//! description.**
//!
//! 1. *It is fixtures, not draws.* Every other sweep in this crate runs
//!    through `test_utils::fuzz` with a varying seed and the
//!    `CAD_FUZZ_EFFORT` dial, including `review_fuzz_exact.rs` alongside
//!    it. This file is ~50 hand-chosen numbers, deliberately: the claim
//!    is a CONTRACT with an exact boundary (a point input pads exactly
//!    `PAD_ULPS`), so a fixture at the boundary is the tightest possible
//!    red and a random draw is not sharper. What fixtures cannot do is
//!    find the input class nobody thought of, and nothing here does.
//! 2. *The oracle tier's counterpart is a RATIO, and a ratio is
//!    scale-free.* Between them these two instruments do NOT add up to a
//!    two-sided `assert_contains`: a fixed absolute over-widening on a
//!    box whose oracle width is already large moves no ratio and is not
//!    a fixture here. The box rows below narrow that gap for the
//!    monotone operations; the residue — non-monotone shapes on wide
//!    boxes — is open, and is recorded as **S134**.

mod common;

use common::steps;
use interval_transcendentals::DInterval;

/// The derived transcendental pad (`docs/derivations.md` §2): libm's CI
/// bit-distance bound is 1 for the sin family and 2 for `atan2`, and
/// Lemma P3 turns k bit-steps into k+1 outward steps, so the requirement
/// is 2 and 3 respectively. 4 is what `src/trig.rs` ships, with margin 2
/// and 1.
///
/// **This number is copied from the derivation ON PURPOSE, and unifying
/// it with `trig.rs`'s `PAD_ULPS` would disarm this file.** The whole
/// instrument is that the constant here and the constant there are
/// independent: raising `PAD_ULPS` then reds against a value the
/// derivation fixed. Import `PAD_ULPS` and every row below becomes a
/// tautology that passes at any pad. This repo's standing habit is to
/// give a duplicated constant one home; here that habit is the wrong
/// move, which is why the reason is written at the constant rather than
/// left to be re-derived by whoever next runs that sweep. (The same
/// applies to `2^-960` in `review_fuzz_exact.rs`, for the same reason.)
/// `trig.rs` guards the other side — a compile-time assert that
/// `PAD_ULPS` is at least the derived minimum of 3.
const TRANSCENDENTAL_PAD: i128 = 4;

/// The derived arithmetic pad (`docs/derivations.md` §1, Lemma P1): a
/// correctly-rounded-to-nearest result is at most one representable step
/// from the true value, so one step outward encloses it. Copied
/// deliberately, as above.
const ARITHMETIC_PAD: i128 = 1;

/// `steps` (`common::steps`, shared with the oracle tier's `Tightness`)
/// is the metric this whole file rests on, so it is pinned here rather
/// than trusted — including at the zero crossing, which is the rung the
/// obvious bit-pattern implementation gets wrong (it counts `-0.0` and
/// `+0.0` as two). Unlike the pad constants above, sharing a METRIC
/// disarms nothing: it is not the claim being tested.
#[test]
fn the_step_metric_matches_the_ladder_the_pads_climb() {
    for x in [1.0, -1.0, 0.0, -0.0, f64::MIN_POSITIVE, 1e300, -3.75e-9] {
        assert_eq!(steps(x, x.next_up()), 1, "next_up from {x:e}");
        assert_eq!(steps(x, x.next_down()), -1, "next_down from {x:e}");
        assert_eq!(steps(x, x), 0, "identity at {x:e}");
    }
    // Across zero, on the ladder itself: four steps down from +0.0 is
    // four steps by this metric too.
    let mut y = 0.0_f64;
    for _ in 0..4 {
        y = y.next_down();
    }
    assert_eq!(steps(y, 0.0), 4, "four rungs below zero");
    assert!(y < 0.0);
}

/// The contract itself: `[iv.lo(), iv.hi()]` brackets `reference` and is
/// no more than `pad` steps outward from it on either side.
fn assert_pad_within(ctx: &str, iv: DInterval, reference: f64, pad: i128) {
    assert!(
        !iv.is_empty() && !iv.is_nai(),
        "{ctx}: expected an ordinary enclosure, got {iv:?}"
    );
    let below = steps(iv.lo(), reference);
    let above = steps(reference, iv.hi());
    assert!(
        (0..=pad).contains(&below),
        "{ctx}: lo is {below} steps below the reference {reference:e} \
         (contract: 0..={pad}); iv={iv:?}"
    );
    assert!(
        (0..=pad).contains(&above),
        "{ctx}: hi is {above} steps above the reference {reference:e} \
         (contract: 0..={pad}); iv={iv:?}"
    );
}

/// The same contract on a NON-DEGENERATE box, where the two endpoints
/// have different references. This is what a point-only row cannot say:
/// the oracle-tier counterpart is a width RATIO, and a ratio stops
/// noticing a fixed absolute over-widening once the oracle's own width
/// is large.
fn assert_pad_within_box(ctx: &str, iv: DInterval, lo_ref: f64, hi_ref: f64, pad: i128) {
    assert!(
        !iv.is_empty() && !iv.is_nai(),
        "{ctx}: expected an ordinary enclosure, got {iv:?}"
    );
    let below = steps(iv.lo(), lo_ref);
    let above = steps(hi_ref, iv.hi());
    assert!(
        (0..=pad).contains(&below),
        "{ctx}: lo is {below} steps below its reference {lo_ref:e} \
         (contract: 0..={pad}); iv={iv:?}"
    );
    assert!(
        (0..=pad).contains(&above),
        "{ctx}: hi is {above} steps above its reference {hi_ref:e} \
         (contract: 0..={pad}); iv={iv:?}"
    );
}

/// Boxes wide enough that the enclosure's width is dominated by the
/// function's own variation rather than by the pad — the regime the
/// oracle tier's ratio is blind in. All are inside the ordinary regime
/// of every function they are fed to.
const BOXES: [(f64, f64); 6] = [
    (0.25, 0.75),
    (-0.9, -0.1),
    (0.1, 0.9),
    (1.0e-6, 0.5),
    (-0.5, 0.5),
    (0.3, 0.3125),
];

#[test]
fn the_pad_contract_holds_on_wide_boxes_too() {
    for (a, b) in BOXES {
        let x = DInterval::from_bounds(a, b);
        // Monotone increasing on these boxes, and no clip can reach:
        // every image lies well inside the function's range.
        for (name, iv, lo_ref, hi_ref) in [
            ("asin", x.asin(), libm::asin(a), libm::asin(b)),
            ("atan", x.atan(), libm::atan(a), libm::atan(b)),
            ("tan", x.tan(), libm::tan(a), libm::tan(b)),
        ] {
            assert_pad_within_box(
                &format!("{name}([{a}, {b}])"),
                iv,
                lo_ref,
                hi_ref,
                TRANSCENDENTAL_PAD,
            );
        }
        // Monotone DECREASING: the lower bound comes from the upper
        // endpoint. Asserting it this way round is also what would catch
        // the two being swapped.
        assert_pad_within_box(
            &format!("acos([{a}, {b}])"),
            x.acos(),
            libm::acos(b),
            libm::acos(a),
            TRANSCENDENTAL_PAD,
        );
        // sin and cos on a monotone piece — no extremum in [-0.9, 0.9]
        // for sin, and cos is monotone on each side of 0, so the box
        // straddling zero is excluded for cos only by taking the max
        // reference at the endpoint nearer 0.
        let (sa, sb) = (libm::sin(a), libm::sin(b));
        let sin_iv = x.sin();
        assert!(
            sin_iv.lo() != -1.0 && sin_iv.hi() != 1.0,
            "sin([{a}, {b}]): an extremum was captured"
        );
        assert_pad_within_box(
            &format!("sin([{a}, {b}])"),
            sin_iv,
            sa.min(sb),
            sa.max(sb),
            TRANSCENDENTAL_PAD,
        );
        // sqrt, on the nonnegative boxes only.
        if a >= 0.0 {
            assert_pad_within_box(
                &format!("sqrt([{a}, {b}])"),
                x.sqrt(),
                a.sqrt(),
                b.sqrt(),
                ARITHMETIC_PAD,
            );
        }
    }
}

#[test]
fn arithmetic_on_wide_boxes_pads_at_most_one_step_per_endpoint() {
    for (a0, a1) in BOXES {
        for (b0, b1) in BOXES {
            let (x, y) = (
                DInterval::from_bounds(a0, a1),
                DInterval::from_bounds(b0, b1),
            );
            assert_pad_within_box(
                &format!("[{a0},{a1}] + [{b0},{b1}]"),
                x + y,
                a0 + b0,
                a1 + b1,
                ARITHMETIC_PAD,
            );
            assert_pad_within_box(
                &format!("[{a0},{a1}] - [{b0},{b1}]"),
                x - y,
                a0 - b1,
                a1 - b0,
                ARITHMETIC_PAD,
            );
            let corners = [a0 * b0, a0 * b1, a1 * b0, a1 * b1];
            assert_pad_within_box(
                &format!("[{a0},{a1}] * [{b0},{b1}]"),
                x * y,
                corners.into_iter().fold(f64::INFINITY, f64::min),
                corners.into_iter().fold(f64::NEG_INFINITY, f64::max),
                ARITHMETIC_PAD,
            );
            // Division needs a divisor that does not touch zero.
            if b0 > 0.0 {
                let q = [a0 / b0, a0 / b1, a1 / b0, a1 / b1];
                assert_pad_within_box(
                    &format!("[{a0},{a1}] / [{b0},{b1}]"),
                    x / y,
                    q.into_iter().fold(f64::INFINITY, f64::min),
                    q.into_iter().fold(f64::NEG_INFINITY, f64::max),
                    ARITHMETIC_PAD,
                );
            }
        }
    }
}

/// Interior points for the sin family: none within a padded enclosure's
/// reach of `k·π/2`, so no extremum is captured and no pole is possible,
/// and all small enough that the grid test still localizes.
const TRIG_INTERIOR: [f64; 10] = [
    0.3, 0.75, 1.25, 2.0, 2.9, -0.4, -1.1, -3.9, 123.456, -9_999.5,
];

#[test]
fn sin_and_cos_pad_at_most_the_derived_four_steps() {
    for x in TRIG_INTERIOR {
        let p = DInterval::point(x);
        let (s, c) = (p.sin(), p.cos());
        // `sin_cos` is the kernel's primitive and the pair must BE the
        // components; assert that here rather than testing the pair and
        // hoping the singles still delegate.
        let (ps, pc) = p.sin_cos();
        assert_eq!((ps.lo(), ps.hi()), (s.lo(), s.hi()), "sin_cos.0 != sin");
        assert_eq!((pc.lo(), pc.hi()), (c.lo(), c.hi()), "sin_cos.1 != cos");
        // No extremum captured: capture pins a bound at exactly ±1.
        assert!(
            s.lo() != -1.0 && s.hi() != 1.0 && c.lo() != -1.0 && c.hi() != 1.0,
            "x={x}: an extremum was captured, so this row is not testing the pad"
        );
        assert_pad_within(&format!("sin({x})"), s, libm::sin(x), TRANSCENDENTAL_PAD);
        assert_pad_within(&format!("cos({x})"), c, libm::cos(x), TRANSCENDENTAL_PAD);
    }
}

#[test]
fn tan_pads_at_most_the_derived_four_steps_on_pole_free_points() {
    for x in TRIG_INTERIOR {
        let t = DInterval::point(x).tan();
        assert!(
            t.lo().is_finite() && t.hi().is_finite(),
            "x={x}: tan refused (a pole was possible), so this row is not testing the pad"
        );
        assert_pad_within(&format!("tan({x})"), t, libm::tan(x), TRANSCENDENTAL_PAD);
    }
}

#[test]
fn inverse_trig_pads_at_most_the_derived_four_steps() {
    // Strictly inside (-1, 1), so no domain clamp; the clip checks below
    // assert the rest rather than asserting it in prose.
    for x in [0.0, 0.125, -0.25, 0.5, -0.6, 0.875, -0.9] {
        let p = DInterval::point(x);
        // A range clip pulls a bound INWARD, which only shrinks the
        // distances `assert_pad_within` measures — so that helper can
        // never notice one. Assert directly that no bound sits on a clip
        // sentinel, the way the sin/cos and tan rows do.
        let (as_, ac) = (p.asin(), p.acos());
        let half_pi_hi = core::f64::consts::FRAC_PI_2.next_up();
        assert!(
            as_.lo() != -half_pi_hi && as_.hi() != half_pi_hi,
            "asin({x}): a range clip fired, so this row is not testing the pad"
        );
        assert!(
            ac.lo() != 0.0 && ac.hi() != core::f64::consts::PI.next_up(),
            "acos({x}): a range clip fired, so this row is not testing the pad"
        );
        assert_pad_within(
            &format!("asin({x})"),
            p.asin(),
            libm::asin(x),
            TRANSCENDENTAL_PAD,
        );
        assert_pad_within(
            &format!("acos({x})"),
            p.acos(),
            libm::acos(x),
            TRANSCENDENTAL_PAD,
        );
    }
    for x in [0.0, 0.5, -1.5, 42.0, -1e6] {
        let a = DInterval::point(x).atan();
        let half_pi_hi = core::f64::consts::FRAC_PI_2.next_up();
        assert!(
            a.lo() != -half_pi_hi && a.hi() != half_pi_hi,
            "atan({x}): a range clip fired, so this row is not testing the pad"
        );
        assert_pad_within(&format!("atan({x})"), a, libm::atan(x), TRANSCENDENTAL_PAD);
    }
    // Quadrant interiors: origin excluded, branch cut not crossed, and
    // the result nowhere near the ±π range clip.
    for (y, x) in [
        (1.0, 2.0),
        (-1.0, 2.0),
        (0.5, 0.5),
        (-3.0, 1.5),
        (1e-3, 7.0),
        (2.0, -1.0),
    ] {
        let a = DInterval::point(y).atan2(DInterval::point(x));
        assert!(
            a.lo() != (-core::f64::consts::PI).next_down()
                && a.hi() != core::f64::consts::PI.next_up(),
            "atan2({y}, {x}): a range clip fired, so this row is not testing the pad"
        );
        assert_pad_within(
            &format!("atan2({y}, {x})"),
            a,
            libm::atan2(y, x),
            TRANSCENDENTAL_PAD,
        );
    }
}

#[test]
fn arithmetic_and_sqrt_pad_at_most_one_step() {
    // Pairs whose exact results are a mix of representable and not, so
    // both the padded and the witnessed-exact paths are exercised.
    for (a, b) in [
        (1.0, 3.0),
        (0.1, 0.7),
        (-2.5, 8.125),
        (1e17, 3.0),
        (7.0, 0.5),
        (1.0, 1.0),
        (-6.75, -1e-3),
        (5e-300, 4.0),
    ] {
        let (ia, ib) = (DInterval::point(a), DInterval::point(b));
        assert_pad_within(&format!("{a} + {b}"), ia + ib, a + b, ARITHMETIC_PAD);
        assert_pad_within(&format!("{a} - {b}"), ia - ib, a - b, ARITHMETIC_PAD);
        assert_pad_within(&format!("{a} * {b}"), ia * ib, a * b, ARITHMETIC_PAD);
        assert_pad_within(&format!("{a} / {b}"), ia / ib, a / b, ARITHMETIC_PAD);
    }
    for a in [0.25, 2.0, 3.0, 1e-8, 1.5e100, 7.421875] {
        assert_pad_within(
            &format!("sqrt({a})"),
            DInterval::point(a).sqrt(),
            a.sqrt(),
            ARITHMETIC_PAD,
        );
    }
}

/// `powi` is deliberately not above. Its enclosure is a COMPOSITION of
/// `mul` pads (directed binary exponentiation, plus a division for a
/// negative exponent), so the number of steps it is entitled to is a
/// function of the exponent, not a constant — and each component step is
/// already bounded by the row above. That is a reason for not writing a
/// CONSTANT here, not a reason for writing nothing: an
/// exponent-dependent bound is derivable, is not derived, and is
/// scheduled as smell-scan **S134** / §D row **D78**. What IS a fixed
/// contract here is the one `powi` carries for soundness: an even power of a zero-straddling
/// interval has lower bound exactly `0.0`, no pad, so a downstream
/// `sqrt` never sees a spurious negative.
#[test]
fn powi_even_straddle_keeps_its_exact_zero_floor() {
    for n in [2, 4, 12, 30] {
        // Every row straddles zero — `(0.0, 5.0)` used to be here and is
        // not a straddle: its `lo() == 0.0` holds through the ordinary
        // monotone arm, so it tested the branch's name rather than the
        // branch. It moved to the row below.
        for (lo, hi) in [(-1.0, 2.0), (-3.5, 0.25), (-1e-9, 1e-9), (-0.0, 5.0)] {
            let p = DInterval::from_bounds(lo, hi).powi(n);
            assert_eq!(
                p.lo(),
                0.0,
                "powi({n}) of [{lo}, {hi}]: the straddle floor must be exactly 0.0"
            );
            assert!(p.lo().is_sign_positive(), "and not -0.0");
        }
        // The monotone arm, for contrast: a nonnegative box's lower
        // bound is `pow_mag_lo` of its lower endpoint, which is exactly
        // 0 only because 0^n is exact — not because of the straddle
        // branch.
        let m = DInterval::from_bounds(0.0, 5.0).powi(n);
        assert_eq!(m.lo(), 0.0);
        assert!(m.hi() >= libm::pow(5.0, f64::from(n)));
    }
}

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
//! Every input below is chosen in the interior of its function's
//! ordinary regime — no extremum capture, no pole, no domain clamp, no
//! range clip — because those are the documented paths that widen on
//! purpose. The rows assert that no clip fired rather than skipping when
//! one does, so a change that starts clipping here is visible.

use interval_transcendentals::DInterval;

/// The derived transcendental pad (`src/trig.rs`, `docs/derivations.md`
/// §2): libm's CI bit-distance bound is 1 for the sin family and 2 for
/// `atan2`, and Lemma P3 turns k bit-steps into k+1 outward steps, so
/// the requirement is 2 and 3 respectively. 4 is what the crate ships,
/// with margin 2 and 1.
const TRANSCENDENTAL_PAD: i128 = 4;

/// The derived arithmetic pad (`src/round.rs`, Lemma P1): a
/// correctly-rounded-to-nearest result is at most one representable step
/// from the true value, so one step outward encloses it.
const ARITHMETIC_PAD: i128 = 1;

/// Signed distance in representable steps from `from` to `to`, counted
/// on the same ladder `f64::next_up`/`next_down` walk — the ladder
/// `round.rs`'s `step_up`/`step_down` climb, so the number this returns
/// is the number of pad steps taken. `steps(x, x.next_up()) == 1` for
/// every finite `x`, and the two zeros are ONE rung, because that ladder
/// steps from `+0.0` straight to `-MIN_SUBNORMAL`.
fn steps(from: f64, to: f64) -> i128 {
    fn key(x: f64) -> i128 {
        assert!(x.is_finite(), "step distance is only defined on finites");
        let m = i128::from(x.abs().to_bits());
        if x < 0.0 { -m } else { m }
    }
    key(to) - key(from)
}

/// `steps` is the metric the whole file rests on, so it is pinned rather
/// than trusted — including at the zero crossing, which is the rung the
/// obvious bit-pattern implementation gets wrong (it counts `-0.0` and
/// `+0.0` as two).
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
        let (s, c) = p.sin_cos();
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
    // Strictly inside (-1, 1), so no domain clamp; and far enough from
    // the ends that neither range clip (±π/2 for asin, [0, π] for acos)
    // can bite.
    for x in [0.0, 0.125, -0.25, 0.5, -0.6, 0.875, -0.9] {
        let p = DInterval::point(x);
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
        assert_pad_within(
            &format!("atan({x})"),
            DInterval::point(x).atan(),
            libm::atan(x),
            TRANSCENDENTAL_PAD,
        );
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
        assert_pad_within(
            &format!("atan2({y}, {x})"),
            DInterval::point(y).atan2(DInterval::point(x)),
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
/// already bounded by the row above. What the composition costs in
/// practice is a measurement, and the thing that measures it is
/// `Tightness` in the oracle tier. What IS a fixed contract here is the
/// one `powi` carries for soundness: an even power of a zero-straddling
/// interval has lower bound exactly `0.0`, no pad, so a downstream
/// `sqrt` never sees a spurious negative.
#[test]
fn powi_even_straddle_keeps_its_exact_zero_floor() {
    for n in [2, 4, 12, 30] {
        for (lo, hi) in [(-1.0, 2.0), (-3.5, 0.25), (-1e-9, 1e-9), (0.0, 5.0)] {
            let p = DInterval::from_bounds(lo, hi).powi(n);
            assert_eq!(
                p.lo(),
                0.0,
                "powi({n}) of [{lo}, {hi}]: the straddle floor must be exactly 0.0"
            );
            assert!(p.lo().is_sign_positive(), "and not -0.0");
        }
    }
}

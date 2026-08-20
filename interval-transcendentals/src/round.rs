//! Directed-rounding substitutes: outward padding of round-to-nearest
//! results. Portable Rust cannot set the FPU rounding mode, so every
//! inexact operation is padded outward by representable-neighbor steps.
//! Soundness proofs are in `docs/derivations.md` (§1). **A lemma more
//! than one helper rests on has exactly one home**: the 2Prod
//! exactness test is [`two_prod_witness`], which `mul`, `div` and
//! `sqrt` all call rather than respell, and the validity floor it gates
//! on is [`TWO_PROD_VALID_MIN`]. A helper resting on a lemma of its own
//! states it in its own doc.
//!
//! Nothing mechanical enforces that split — a fifth helper respelling
//! `fma(x, y, -z) == 0.0` inline would compile and pass. What makes it
//! discoverable is that `docs/derivations.md` §3 derives ONE floor for
//! all three witnesses, so a second constant contradicts the
//! derivation, and the crate's one `grep mul_add` is short.

/// One representable step down, saturating at `-inf` (`next_down(-inf) =
/// -inf`, which is the correct lower bound for an already-unbounded side).
#[inline]
pub(crate) fn down1(x: f64) -> f64 {
    x.next_down()
}

/// One representable step up, saturating at `+inf`.
#[inline]
pub(crate) fn up1(x: f64) -> f64 {
    x.next_up()
}

/// `k` representable steps down.
///
/// Soundness comes from the lemmas in docs/derivations.md §1: P3 (the
/// load-bearing one for libm pads — k bit-steps from the correctly
/// rounded reference need k+1 outward steps) and P2 (general k·ulp(t)
/// errors need 2k steps).
#[inline]
pub(crate) fn step_down(x: f64, k: u32) -> f64 {
    let mut y = x;
    for _ in 0..k {
        y = y.next_down();
    }
    y
}

/// `k` representable steps up. Mirror of [`step_down`].
#[inline]
pub(crate) fn step_up(x: f64, k: u32) -> f64 {
    let mut y = x;
    for _ in 0..k {
        y = y.next_up();
    }
    y
}

/// Lower bound of `a + b`: exact when the rounded sum is exact (decided by
/// the always-valid TwoSum error term — the rounding error of an f64
/// addition is itself representable, with no underflow caveat), else one
/// step down (Lemma P1: for any correctly-rounded-to-nearest result `c` of
/// a real `t`, `next_down(c) <= t`, because `c` nearest means no
/// representable number lies strictly between `t` and `c`).
#[inline]
pub(crate) fn add_lo(a: f64, b: f64) -> f64 {
    let s = a + b;
    if two_sum_err(a, b, s) == 0.0 {
        s
    } else {
        down1(s)
    }
}

/// Upper bound of `a + b`. Mirror of [`add_lo`].
#[inline]
pub(crate) fn add_hi(a: f64, b: f64) -> f64 {
    let s = a + b;
    if two_sum_err(a, b, s) == 0.0 {
        s
    } else {
        up1(s)
    }
}

/// Lower bound of `a - b` (via [`add_lo`] on `-b`; f64 negation is exact).
#[inline]
pub(crate) fn sub_lo(a: f64, b: f64) -> f64 {
    add_lo(a, -b)
}

/// Upper bound of `a - b`.
#[inline]
pub(crate) fn sub_hi(a: f64, b: f64) -> f64 {
    add_hi(a, -b)
}

/// TwoSum (Knuth) rounding-error term of `s = RN(a + b)`; `0.0` iff the
/// addition was exact. Exact for ALL finite doubles — no non-underflow
/// proviso, unlike products — by `docs/derivations.md` §1 **Lemma P0**,
/// which is where that theorem is stated and is the reason this witness
/// needs no magnitude gate while [`two_prod_witness`] does. Infinite `s`
/// (overflow) yields a NaN error term, which compares `!= 0.0` and
/// therefore correctly takes the padded path.
#[inline]
fn two_sum_err(a: f64, b: f64, s: f64) -> f64 {
    let bp = s - a;
    let ap = s - bp;
    (a - ap) + (b - bp)
}

/// Lower bound of `a * b`, with the Kahan corner-product convention
/// `0 · anything := 0` built in (an exact-zero factor means the true
/// corner value is the real number 0, even against an infinite bound, so
/// `0.0` is exact — this is what interval multiplication needs and it
/// sidesteps the `0 × inf = NaN` trap). Exactness test: the FMA residual
/// `fma(a, b, -r)` equals the true rounding error whenever the product
/// clears the 2Prod validity floor ([`TWO_PROD_VALID_MIN`]); zero factors
/// are exact by inspection; everything else pads one step (Lemma P1).
#[inline]
pub(crate) fn mul_lo(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        return 0.0;
    }
    let r = a * b;
    if mul_exact(a, b, r) { r } else { down1(r) }
}

/// Upper bound of `a * b`. Mirror of [`mul_lo`].
#[inline]
pub(crate) fn mul_hi(a: f64, b: f64) -> f64 {
    if a == 0.0 || b == 0.0 {
        return 0.0;
    }
    let r = a * b;
    if mul_exact(a, b, r) { r } else { up1(r) }
}

/// 2Prod validity floor: the FMA residual `x·y − z` is guaranteed exactly
/// representable only when the product magnitude is `>= 2^-969`
/// (Ogita-Rump-Oishi; below it the residual itself can underflow and a
/// nonzero true residual can round to 0.0, making the witness LIE). We
/// gate at `2^-960` — the literature floor is ~2^-969; the extra 9
/// binades absorb every boundary quibble (rounded vs exact product
/// magnitude, subnormal factors) at zero practical cost. Exactly
/// `2^-960` (bit pattern: biased exponent 1023 − 960 = 63, zero
/// mantissa).
///
/// Found live by the differential harness: a barely-NORMAL product of a
/// subnormal factor passed an `is_normal()` guard while its residual
/// underflowed — a 1-ulp containment violation vs the oracle.
///
/// One constant for all three witnesses (`mul`, `div`, `sqrt`), because
/// derivations.md §3 derives one number for all three: the condition is
/// on the FMA's product term, whichever operation supplies it.
pub(crate) const TWO_PROD_VALID_MIN: f64 = f64::from_bits(0x03F0_0000_0000_0000);

/// The 2Prod exactness witness, stated once: `fma(x, y, -z)` evaluates
/// `x·y − z` with a SINGLE rounding, and the true residual is exactly
/// representable whenever the product clears [`TWO_PROD_VALID_MIN`] — so
/// a `0.0` result then PROVES `x·y = z` exactly, and no outward pad is
/// owed. Below the floor, or on a non-finite operand, the witness may
/// lie and the caller must pad.
///
/// `mag` is the caller's handle on the product magnitude the floor is
/// about, and it is a parameter rather than `(x * y).abs()` because each
/// caller already holds a cheaper exact one: `|r|` for `mul` (the
/// rounded product itself), `|a|` for `div` (where `q·b ≈ a`), and the
/// radicand for `sqrt` (where `s·s ≈ a`).
#[inline]
pub(crate) fn two_prod_witness(x: f64, y: f64, z: f64, mag: f64) -> bool {
    mag >= TWO_PROD_VALID_MIN && f64::mul_add(x, y, -z) == 0.0
}

#[inline]
fn mul_exact(a: f64, b: f64, r: f64) -> bool {
    // `r == ±inf` is rejected by the FMA check alone (fma(a, b, ∓inf) is
    // ±inf or NaN, never 0.0); the magnitude gate rejects every
    // subnormal/underflow-adjacent product, where the witness is invalid.
    two_prod_witness(a, b, r, r.abs())
}

/// Lower bound of `a / b` (`b != 0`), padded one step unless the
/// quotient is provably exact ([`div_exact`]) or the numerator is
/// exactly zero.
#[inline]
pub(crate) fn div_lo(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        return 0.0;
    }
    let q = a / b;
    if div_exact(a, b, q) { q } else { down1(q) }
}

/// Upper bound of `a / b` (`b != 0`). Mirror of [`div_lo`].
#[inline]
pub(crate) fn div_hi(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        return 0.0;
    }
    let q = a / b;
    if div_exact(a, b, q) { q } else { up1(q) }
}

/// Exactness witness for `q = RN(a / b)`, the division mirror of
/// [`mul_exact`]: `fma(q, b, -a)` evaluates `q·b − a` with a SINGLE
/// rounding, and by the same 2Prod representability theorem that backs
/// `mul_exact` the true residual `q·b − a` is exactly representable
/// whenever the product clears [`TWO_PROD_VALID_MIN`] (here `q·b ≈ a`,
/// so the gate is on `|a|`). A representable residual is returned
/// unrounded, so a `0.0` result means `q·b = a` exactly, i.e. `a/b = q`
/// with no rounding error and no pad needed. Non-finite `q` (overflow,
/// or `b` infinite) and underflow-adjacent magnitudes fail the test and
/// take the padded path, as does every genuinely inexact quotient.
///
/// Why this is worth having rather than padding unconditionally: exact
/// quotients are not a curiosity in geometry code — `v / |v|` for an
/// axis-aligned `v` is the motivating case, and a 1-ulp pad there turns
/// an exactly-unit frame vector into a 2-ulp-wide one, which propagates
/// into every coordinate computed against that frame. The kernel's
/// exact-order band (`topo`'s null-edge sort, whose whole design rests
/// on axis-aligned splits over dyadic geometry classifying EXACTLY)
/// escalates without it — found live by that suite during the M5 PR 1
/// backend swap.
#[inline]
fn div_exact(a: f64, b: f64, q: f64) -> bool {
    q.is_finite() && two_prod_witness(q, b, a, a.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_of_provably_exact_quotients_takes_no_pad() {
        // The motivating case: normalizing an already-unit component.
        assert_eq!(div_lo(1.0, 1.0), 1.0);
        assert_eq!(div_hi(1.0, 1.0), 1.0);
        // Powers of two and their multiples divide exactly.
        for (a, b, q) in [(1.0, 2.0, 0.5), (3.0, 4.0, 0.75), (-7.0, 0.5, -14.0)] {
            assert_eq!(div_lo(a, b), q, "div_lo({a}, {b})");
            assert_eq!(div_hi(a, b), q, "div_hi({a}, {b})");
        }
        // An exactly-zero numerator was already exact.
        assert_eq!(div_lo(0.0, 3.0), 0.0);
    }

    #[test]
    fn inexact_quotients_still_pad_outward_around_the_truth() {
        // 1/3 is not representable: the enclosure must straddle it.
        let (lo, hi) = (div_lo(1.0, 3.0), div_hi(1.0, 3.0));
        let q = 1.0_f64 / 3.0;
        assert_eq!(lo, q.next_down());
        assert_eq!(hi, q.next_up());
        assert!(lo < hi);
    }

    #[test]
    fn div_witness_refuses_non_finite_and_underflow_adjacent_cases() {
        // Overflow: the quotient is not finite, so the padded path runs
        // (and saturates at the infinity that already bounds the truth).
        assert_eq!(div_hi(f64::MAX, 0.5), f64::INFINITY);
        // Infinite divisor: quotient 0, but the FMA witness is NaN — no
        // exactness may be claimed, and the pad brackets the true 0.
        assert!(div_lo(1.0, f64::INFINITY) < 0.0);
        assert!(div_hi(1.0, f64::INFINITY) > 0.0);
        // Below the 2Prod validity floor the witness is not trusted even
        // when the quotient happens to be exact.
        let tiny = f64::MIN_POSITIVE; // 2^-1022, far below the 2^-960 floor
        assert!(div_lo(tiny, 2.0) < tiny / 2.0);
    }
}

//! Directed-rounding substitutes: outward padding of round-to-nearest
//! results. Portable Rust cannot set the FPU rounding mode, so every
//! inexact operation is padded outward by representable-neighbor steps.
//! Soundness proofs are in `docs/derivations.md` (§1); the load-bearing
//! lemmas are restated at each helper.

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
/// Lemma P2 (derivations §1.2): if a computed value `c` satisfies
/// `|c - t| <= k·ulp(t)` for the true value `t`, then `step_down(c, k) <= t`
/// (consecutive-float gaps on the segment between `t` and `c` are all
/// `>= ulp(t)` on the side that matters, so `k` steps cross the whole
/// error distance).
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
/// addition was exact. Exact for ALL finite doubles (the error of a
/// rounded sum is always representable; no non-underflow proviso, unlike
/// products). Infinite `s` (overflow) yields a NaN error term, which
/// compares `!= 0.0` and therefore correctly takes the padded path.
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
/// `fma(a, b, -r)` equals the true rounding error whenever `r` is normal
/// (2Prod validity requires no underflow — Ogita–Rump–Oishi); zero factors
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

#[inline]
fn mul_exact(a: f64, b: f64, r: f64) -> bool {
    // `is_normal()` excludes r == ±inf (overflow: pad; the pad saturates
    // to MAX/inf correctly) and subnormal r (2Prod residual can itself
    // underflow there, so the test would be unreliable — always pad).
    r.is_normal() && f64::mul_add(a, b, -r) == 0.0
}

/// Lower bound of `a / b` (`b != 0`); always pads (division is rarely
/// exact and a reliable exactness witness needs preconditions we don't
/// track), except for an exactly-zero numerator.
#[inline]
pub(crate) fn div_lo(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        return 0.0;
    }
    down1(a / b)
}

/// Upper bound of `a / b` (`b != 0`). Mirror of [`div_lo`].
#[inline]
pub(crate) fn div_hi(a: f64, b: f64) -> f64 {
    if a == 0.0 {
        return 0.0;
    }
    up1(a / b)
}

//! **Error-free transforms** — the primitives that hand a caller the
//! exact real result of a floating-point operation as a pair of `f64`s,
//! so a decision about the REAL value is available where the rounded
//! value would decide something weaker.
//!
//! A rounded comparison answers a different question from the one a
//! structural door usually asks. `fl(a + b) == fl(c + d)` is implied by
//! `a + b = c + d` in ℝ and does not imply it: on `lo = 0`, `hi = 1`
//! the pair `(½, ½ + 2⁻⁵³)` rounds to exactly `1.0` while its real sum
//! is half an ulp above it. A door that admits structure on the rounded
//! answer therefore admits structure that is an ulp away from what it
//! claims. [`two_sum`] is how such a door decides the real identity
//! instead, in six correctly-rounded operations and without any wider
//! arithmetic type.

/// Knuth's 2Sum: the exact sum of two `f64`s as the unevaluated pair
/// `(s, e)` with `s = fl(a + b)` and `s + e = a + b` **in ℝ**.
///
/// # Exactness precondition
///
/// The identity `s + e = a + b` holds whenever `a`, `b` and the HEAD
/// `s` are all finite. When the head overflows, `e` is a NaN and the
/// pair carries no information about the real sum — a caller that can
/// meet an infinite head must say what it does about that case rather
/// than read the residual. (There is no underflow caveat: the
/// subtractions below are exact in the subnormal range too.)
///
/// # Why the pair is a function of the operands' bits
///
/// Six IEEE-754 binary64 operations, each correctly rounded, none of
/// them a sum of three terms a compiler could contract into an FMA. So
/// the result does not vary with target or optimization level, and two
/// callers on different machines reach the same verdict.
///
/// # Deciding a real identity with it
///
/// Two such pairs agree componentwise exactly when the two real sums
/// agree: rounding is a function of the real value, so equal sums give
/// equal heads, and the residual is then determined by the head and
/// that sum. `two_sum(a, b) == two_sum(c, d)` is therefore
/// `a + b == c + d` in ℝ, for finite heads — and is `false` whenever a
/// head overflows, since a NaN residual compares equal to nothing,
/// including itself.
#[must_use]
pub fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let a_head = s - b;
    let b_head = s - a_head;
    (s, (a - a_head) + (b - b_head))
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    /// The residual is the real remainder, and it is what separates a
    /// pair whose real sum misses a target from one that hits it even
    /// though both round to the target.
    #[test]
    fn the_residual_separates_sums_a_rounded_compare_conflates() {
        let half_up = f64::from_bits(0.5f64.to_bits() + 1);
        assert_eq!(0.5 + half_up, 1.0, "the rounded sums agree");
        assert_ne!(
            two_sum(0.5, half_up),
            two_sum(0.0, 1.0),
            "the exact sums do not: the residual carries the missing 2⁻⁵³"
        );
        assert_eq!(
            two_sum(0.4, 0.6),
            two_sum(0.0, 1.0),
            "a pair whose real sum IS 1 agrees in both components"
        );
    }

    /// An overflowing head has a NaN residual, so the pair never
    /// compares equal — not even to itself.
    #[test]
    fn an_overflowing_head_carries_a_nan_residual() {
        let r = two_sum(1e308, 1.5e308);
        assert!(r.0.is_infinite(), "the head overflows");
        assert!(r.1.is_nan(), "so the residual is meaningless, and says so");
        assert_ne!(r, r, "a pair with a NaN component equals nothing");
    }

    /// Subnormal operands need no caveat: the identity is exact there.
    #[test]
    fn the_subnormal_range_is_exact() {
        let tiny = f64::from_bits(1);
        let (s, e) = two_sum(tiny, tiny * 3.0);
        assert_eq!((s, e), (tiny * 4.0, 0.0), "exact, with no residual left");
    }
}

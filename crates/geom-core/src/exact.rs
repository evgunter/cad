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

/// The direction [`sum_rounded`] rounds the exact sum in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rounding {
    /// To nearest, ties to even: the correctly rounded sum.
    Nearest,
    /// Toward −∞: the largest `f64` not above the exact sum.
    Down,
    /// Toward +∞: the smallest `f64` not below the exact sum.
    Up,
}

/// **The rounded-once sum of a sequence**: the exact real sum of
/// `terms`, rounded once in the direction asked.
///
/// The result is a function of the exact sum alone, so it is invariant
/// under every permutation of `terms` — the property the kernel's
/// measurement folds rely on (`Real::sum_rounded_once`). Two exceptions
/// to "a function of the real sum", both themselves order-free:
///
/// - **Non-finite terms** follow IEEE addition: any NaN, or both
///   infinities, gives NaN; otherwise an infinite term gives its
///   infinity.
/// - **A zero sum is `+0.0`** unless every term is `-0.0` (an empty
///   sequence included as `+0.0`), whatever the direction.
///
/// # Method
///
/// Shewchuk's grow-expansion with zero elimination over [`two_sum`]
/// keeps a nonoverlapping expansion whose components sum to the terms
/// exactly; the nearest rounding is Python's `math.fsum` final step
/// (the half-way correction included), and a directed rounding steps
/// one ulp from the nearest when the residual `exact − nearest` has
/// the wrong sign, read off the expansion of that residual.
///
/// An intermediate head that overflows, with finite terms whose exact
/// sum may not, is the one case the expansion cannot carry; there the
/// terms are summed scaled by 2⁻⁶⁴, which is exact unless some nonzero
/// term is below 2⁻¹⁰¹⁰ in magnitude, and a directed result is stepped
/// one further ulp outward when one is.
#[must_use]
pub fn sum_rounded(terms: &[f64], rounding: Rounding) -> f64 {
    if let Some(special) = non_finite_sum(terms) {
        return special;
    }
    if !terms.is_empty() && terms.iter().all(|t| t.to_bits() == (-0.0f64).to_bits()) {
        return -0.0;
    }
    if let Some(parts) = expansion(terms.iter().copied()) {
        return round_expansion(parts, rounding);
    }
    let scale = 2f64.powi(64);
    let Some(scaled) = expansion(terms.iter().map(|t| t / scale)) else {
        unreachable!(
            "terms below 2^960 in magnitude cannot overflow a sum of fewer than 2^63 terms"
        )
    };
    let lossy = terms
        .iter()
        .any(|t| *t != 0.0 && t.abs() < 2f64.powi(-1010));
    let r = round_expansion(scaled, rounding) * scale;
    match (rounding, lossy) {
        (Rounding::Down, _) if r == f64::INFINITY => f64::MAX,
        (Rounding::Up, _) if r == f64::NEG_INFINITY => f64::MIN,
        (Rounding::Down, true) => r.next_down(),
        (Rounding::Up, true) => r.next_up(),
        _ => r,
    }
}

/// IEEE addition's answer when a term is not finite, `None` otherwise.
fn non_finite_sum(terms: &[f64]) -> Option<f64> {
    let nan = terms.iter().any(|t| t.is_nan());
    let pos = terms.contains(&f64::INFINITY);
    let neg = terms.contains(&f64::NEG_INFINITY);
    match (nan || (pos && neg), pos, neg) {
        (true, _, _) => Some(f64::NAN),
        (false, true, _) => Some(f64::INFINITY),
        (false, _, true) => Some(f64::NEG_INFINITY),
        (false, false, false) => None,
    }
}

/// A nonoverlapping expansion, in increasing magnitude, whose
/// components sum exactly to `terms`; `None` when a head overflows.
fn expansion(terms: impl Iterator<Item = f64>) -> Option<Vec<f64>> {
    let mut parts: Vec<f64> = Vec::new();
    for mut x in terms {
        let mut kept = 0;
        for j in 0..parts.len() {
            let (hi, lo) = two_sum(x, parts[j]);
            if !hi.is_finite() {
                return None;
            }
            if lo != 0.0 {
                parts[kept] = lo;
                kept += 1;
            }
            x = hi;
        }
        parts.truncate(kept);
        parts.push(x);
    }
    Some(parts)
}

/// An expansion's exact sum rounded once.
fn round_expansion(mut parts: Vec<f64>, rounding: Rounding) -> f64 {
    let r = round_nearest(&parts);
    let step_toward = match rounding {
        Rounding::Nearest => return r,
        Rounding::Down if r == f64::INFINITY => return f64::MAX,
        Rounding::Up if r == f64::NEG_INFINITY => return f64::MIN,
        _ if !r.is_finite() => return r,
        Rounding::Down => -1.0,
        Rounding::Up => 1.0,
    };
    parts.push(-r);
    let residual = expansion(parts.into_iter())
        .and_then(|e| e.into_iter().rev().find(|p| *p != 0.0))
        .unwrap_or(0.0);
    if residual * step_toward > 0.0 {
        if step_toward < 0.0 {
            r.next_down()
        } else {
            r.next_up()
        }
    } else {
        r
    }
}

/// The nearest-rounded sum of a nonoverlapping expansion (Python's
/// `math.fsum` final step, ties to even).
fn round_nearest(parts: &[f64]) -> f64 {
    let Some((&top, rest)) = parts.split_last() else {
        return 0.0;
    };
    let mut hi = top;
    let mut lo = 0.0;
    let mut n = rest.len();
    while n > 0 {
        let x = hi;
        n -= 1;
        let y = parts[n];
        hi = x + y;
        lo = y - (hi - x);
        if lo != 0.0 {
            break;
        }
    }
    if n > 0 && ((lo < 0.0 && parts[n - 1] < 0.0) || (lo > 0.0 && parts[n - 1] > 0.0)) {
        let y = lo * 2.0;
        let x = hi + y;
        if y == x - hi {
            hi = x;
        }
    }
    hi
}

#[cfg(test)]
mod tests {
    use super::{Rounding, sum_rounded, two_sum};

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

    test_utils::gated_to![
        "crates/geom-core/src/interval.rs",
        "crates/geom-core/src/real.rs"
    ];

    /// An exact sum in `i128` units of 2⁻⁴⁰, and the three roundings of
    /// it read off `i128 → f64`, which Rust rounds to nearest-even.
    fn oracle(units: &[i128]) -> [f64; 3] {
        let ulp = 2f64.powi(-40);
        let sum: i128 = units.iter().sum();
        let near = sum as f64;
        #[allow(clippy::cast_possible_truncation)]
        let back = near as i128;
        let down = if back > sum { near.next_down() } else { near };
        let up = if back < sum { near.next_up() } else { near };
        [near * ulp, down * ulp, up * ulp]
    }

    /// Counterexample search: every rounding agrees with the exact
    /// oracle, and every permutation of the terms gives the same bits.
    #[test]
    fn the_rounded_once_sum_is_the_exact_sum_rounded_and_order_free() {
        let mut rng = test_utils::fuzz::start("exact::sum_rounded");
        for _ in 0..test_utils::fuzz::scaled(4000) {
            let n = 1 + rng.below(24);
            let units: Vec<i128> = (0..n)
                .map(|_| {
                    let bits = 1 + rng.below(62);
                    #[allow(clippy::cast_possible_wrap)]
                    let k = (rng.next_u64() >> (64 - bits)) as i128;
                    if rng.below(2) == 0 { k } else { -k }
                })
                .collect();
            #[allow(clippy::cast_precision_loss)]
            let mut terms: Vec<f64> = units.iter().map(|&k| k as f64 * 2f64.powi(-40)).collect();
            // `k as f64` rounds a k wider than 53 bits; re-read the term
            // it became so the oracle sums what the door sums.
            #[allow(clippy::cast_possible_truncation)]
            let units: Vec<i128> = terms.iter().map(|t| (t * 2f64.powi(40)) as i128).collect();
            let want = oracle(&units);
            let modes = [Rounding::Nearest, Rounding::Down, Rounding::Up];
            for (mode, want) in modes.into_iter().zip(want) {
                let got = sum_rounded(&terms, mode);
                assert_eq!(
                    got.to_bits(),
                    want.to_bits(),
                    "{mode:?} of {terms:?}: {got:e} against the exact {want:e}; {}",
                    test_utils::fuzz::replay()
                );
                terms.reverse();
                let k = rng.below(terms.len());
                terms.rotate_left(k);
                assert_eq!(
                    sum_rounded(&terms, mode).to_bits(),
                    got.to_bits(),
                    "{mode:?} moved under a permutation of {terms:?}; {}",
                    test_utils::fuzz::replay()
                );
            }
        }
    }

    /// The cases the oracle's range does not reach: cancellation across
    /// the whole exponent range, a tie, the signed zeros, the
    /// non-finite terms and an intermediate overflow.
    #[test]
    fn the_rounded_once_sum_edge_table() {
        let tiny = f64::from_bits(1);
        let half_ulp_of_one = 2f64.powi(-53);
        let cases: [(&[f64], Rounding, f64); 12] = [
            (&[1e300, 1.0, -1e300], Rounding::Nearest, 1.0),
            (&[1e300, tiny, -1e300], Rounding::Down, tiny),
            (&[1.0, half_ulp_of_one], Rounding::Nearest, 1.0),
            (
                &[1.0, half_ulp_of_one, tiny],
                Rounding::Nearest,
                1.0f64.next_up(),
            ),
            (&[1.0, half_ulp_of_one], Rounding::Up, 1.0f64.next_up()),
            (&[1.0, -tiny], Rounding::Down, 1.0f64.next_down()),
            (&[-0.0, -0.0], Rounding::Nearest, -0.0),
            (&[1.0, -1.0], Rounding::Down, 0.0),
            (&[], Rounding::Up, 0.0),
            (&[f64::INFINITY, 1.0], Rounding::Down, f64::INFINITY),
            (
                &[f64::MAX, f64::MAX, -f64::MAX],
                Rounding::Nearest,
                f64::MAX,
            ),
            (&[f64::MAX, f64::MAX], Rounding::Down, f64::MAX),
        ];
        for (terms, mode, want) in cases {
            let got = sum_rounded(terms, mode);
            assert_eq!(
                got.to_bits(),
                want.to_bits(),
                "{mode:?} of {terms:?}: {got:e}, want {want:e}"
            );
        }
        assert!(
            sum_rounded(&[f64::INFINITY, f64::NEG_INFINITY], Rounding::Nearest).is_nan(),
            "both infinities give NaN, as IEEE addition does"
        );
    }

    /// Subnormal operands need no caveat: the identity is exact there.
    #[test]
    fn the_subnormal_range_is_exact() {
        let tiny = f64::from_bits(1);
        let (s, e) = two_sum(tiny, tiny * 3.0);
        assert_eq!((s, e), (tiny * 4.0, 0.0), "exact, with no residual left");
    }
}

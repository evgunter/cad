//! **One power of two on every weight**, exactly.
//!
//! A rational curve or surface is invariant under multiplying all its
//! weights by one constant. A power of two multiplies an `f64` exactly
//! unless the product overflows or leaves bits below the subnormal
//! range, so a net rescaled by one, when every weight survives the
//! round trip, describes the same geometry bit for bit. What the
//! rescale buys is range: an enclosure that forms powers of the
//! weights underflows or overflows on a net whose weights all sit near
//! one end of the `f64` range, and refuses a sound description.

/// `w · 2^k`, applied as two halves so that a `k` past the exponent
/// range — a subnormal carried to one, say — still scales. Exact
/// wherever the result neither overflows nor falls below the normal
/// range.
pub fn pow2_scaled(w: f64, k: i32) -> f64 {
    w * crate::pow2(k / 2) * crate::pow2(k - k / 2)
}

/// The binary exponent `e` of a positive finite `x`: `2^e ≤ x < 2^(e+1)`,
/// read off the bits, subnormals included.
fn exponent(x: f64) -> i32 {
    let bits = x.to_bits();
    #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    let biased = ((bits >> 52) & 0x7ff) as i32;
    if biased == 0 {
        // Subnormal: `x = m · 2^-1074`, `m` the mantissa field.
        #[allow(clippy::cast_possible_wrap)]
        let top_bit = 63 - (bits & ((1 << 52) - 1)).leading_zeros() as i32;
        top_bit - 1074
    } else {
        biased - 1023
    }
}

/// The weights scaled by the one power of two `2^k` that carries the
/// largest into `[1, 2)`, with `k` — or `None` when there is nothing to
/// do (`k = 0`) or when the scaling would drop a bit of some weight
/// (a spread wider than the `f64` range carried down past the
/// subnormal floor). `None` therefore means "read the weights as
/// described". Every weight is expected positive and finite (the
/// curve and surface doors' own condition); a net with any other
/// weight is `None`.
pub fn unit_scale_exact(weights: &[f64]) -> Option<(i32, Vec<f64>)> {
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if weights.iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return None;
    }
    let top = weights.iter().copied().fold(0.0, f64::max);
    if top == 0.0 {
        return None;
    }
    let k = -exponent(top);
    if k == 0 {
        return None;
    }
    let scaled: Vec<f64> = weights.iter().map(|&w| pow2_scaled(w, k)).collect();
    scaled
        .iter()
        .zip(weights)
        .all(|(&s, &w)| pow2_scaled(s, -k) == w)
        .then_some((k, scaled))
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::{exponent, pow2_scaled, unit_scale_exact};

    /// The exponent read off the bits brackets its value, at the
    /// smallest subnormal, the subnormal/normal seam and both ends.
    #[test]
    fn the_exponent_brackets_its_value() {
        for x in [
            f64::from_bits(1),
            f64::from_bits(3),
            f64::MIN_POSITIVE / 2.0,
            f64::MIN_POSITIVE,
            0.75,
            1.0,
            1.0 - f64::EPSILON / 2.0,
            3.0,
            f64::MAX,
        ] {
            let e = exponent(x);
            assert!(
                pow2_scaled(1.0, e) <= x && (e == 1023 || x < pow2_scaled(1.0, e + 1)),
                "{x:e}: exponent {e}"
            );
        }
    }

    /// Weights below one are carried up into `[1, 2)` by one power of
    /// two, whatever their spread: scaling up never drops a bit.
    #[test]
    fn a_net_below_one_is_carried_up_by_one_power_of_two() {
        let tiny = f64::from_bits(1);
        let ws = [tiny, 1.0e-300, tiny, 3.0e-301];
        let (k, scaled) = unit_scale_exact(&ws).expect("scaling up is exact");
        let top = scaled.iter().copied().fold(0.0, f64::max);
        assert!(
            (1.0..2.0).contains(&top),
            "the largest weight landed at {top:e}"
        );
        for (s, w) in scaled.iter().zip(ws) {
            assert_eq!(*s, pow2_scaled(w, k), "{w:e} was not scaled by 2^{k}");
            assert_eq!(
                pow2_scaled(*s, -k),
                w,
                "{w:e} did not survive the round trip"
            );
        }
    }

    /// A spread wider than the range cannot be carried down without
    /// dropping its smallest weight, so it is read as described; a net
    /// already in `[1, 2)` has nothing to do; and a net of weights all
    /// one power of two becomes all ones.
    #[test]
    fn the_rescale_is_taken_only_where_it_is_exact_and_does_something() {
        let tiny = f64::from_bits(1);
        assert_eq!(unit_scale_exact(&[tiny, 4.0, tiny]), None, "a lossy spread");
        assert_eq!(unit_scale_exact(&[tiny, 1.0e2]), None, "a lossy spread");
        assert_eq!(
            unit_scale_exact(&[1.0, 1.5, 0.7]),
            None,
            "already in [1, 2)"
        );
        assert_eq!(unit_scale_exact(&[0.0, 1.0]), None, "a zero weight");
        assert_eq!(
            unit_scale_exact(&[2.0, 2.0]),
            Some((-1, vec![1.0, 1.0])),
            "a uniform power of two"
        );
    }
}

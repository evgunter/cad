//! Part 21 real formatting: shortest round-trip printing, normalized
//! to the ISO 10303-21 real-literal grammar.
//!
//! Rust's `{:?}` for `f64` is the shortest decimal that parses back to
//! the identical bits (the Grisu/Ryū guarantee `core::fmt` provides).
//! Part 21 additionally requires a decimal point in the mantissa and
//! spells the exponent marker `E`, so this module rewrites the token
//! shape without touching the digits: the printed string still parses
//! (via `str::parse::<f64>`) to the input's exact bits — the property
//! the round-trip tests pin, including over random bit patterns.
//!
//! NaN and ±∞ have no Part 21 representation; they refuse with a typed
//! [`StepExportError::NonFiniteReal`] naming the quantity being
//! printed (fail loud — poison never lands in a file).

use crate::StepExportError;

/// Formats `value` as a Part 21 real literal (module docs).
///
/// # Errors
///
/// [`StepExportError::NonFiniteReal`] when `value` is NaN or ±∞.
pub(crate) fn fmt_real(value: f64, context: &'static str) -> Result<String, StepExportError> {
    if !value.is_finite() {
        return Err(StepExportError::NonFiniteReal { value, context });
    }
    let repr = format!("{value:?}");
    // `{:?}` emits either `ddd.ddd` or `d[.ddd]e±ddd`; normalize both
    // to the Part 21 grammar (mantissa must contain '.', marker 'E').
    Ok(match repr.split_once('e') {
        Some((mantissa, exponent)) => {
            if mantissa.contains('.') {
                format!("{mantissa}E{exponent}")
            } else {
                format!("{mantissa}.0E{exponent}")
            }
        }
        None => {
            if repr.contains('.') {
                repr
            } else {
                // Unreachable with today's `{:?}` (non-exponent floats
                // always print a point), kept total rather than trusted.
                format!("{repr}.0")
            }
        }
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use proptest::prelude::*;

    use super::fmt_real;
    use crate::StepExportError;

    /// The exact-bits round-trip property on one value.
    fn round_trips(x: f64) {
        let printed = fmt_real(x, "test").unwrap();
        let back: f64 = printed.parse().unwrap();
        assert_eq!(back.to_bits(), x.to_bits(), "{x:?} printed as {printed}");
    }

    #[test]
    fn representative_pins() {
        for (x, expected) in [
            (1.0, "1.0"),
            (-0.0, "-0.0"),
            (0.0, "0.0"),
            (0.1, "0.1"),
            (-2.5, "-2.5"),
            (1e-7, "1.0E-7"),
            (1.5e300, "1.5E300"),
            (5e-324, "5.0E-324"), // smallest subnormal
            (f64::MAX, "1.7976931348623157E308"),
        ] {
            assert_eq!(fmt_real(x, "pin").unwrap(), expected);
            round_trips(x);
        }
    }

    #[test]
    fn non_finite_refuses_typed() {
        for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            match fmt_real(x, "poison") {
                Err(StepExportError::NonFiniteReal {
                    context: "poison", ..
                }) => {}
                other => panic!("expected NonFiniteReal, got {other:?}"),
            }
        }
    }

    proptest! {
        /// Any finite f64 (drawn from raw bit patterns, so subnormals
        /// and extreme exponents are exercised) prints to a token that
        /// parses back to the identical bits.
        #[test]
        fn round_trip_exact_bits(bits in any::<u64>()) {
            let x = f64::from_bits(bits);
            prop_assume!(x.is_finite());
            round_trips(x);
        }

        /// Every printed token matches the Part 21 real grammar:
        /// sign? digits '.' digits? ('E' sign? digits)?.
        #[test]
        fn grammar_shape(bits in any::<u64>()) {
            let x = f64::from_bits(bits);
            prop_assume!(x.is_finite());
            let s = fmt_real(x, "test").unwrap();
            let (mantissa, exponent) = match s.split_once('E') {
                Some((m, e)) => (m, Some(e)),
                None => (s.as_str(), None),
            };
            let m = mantissa.strip_prefix('-').unwrap_or(mantissa);
            let (int, frac) = m.split_once('.').expect("mantissa has a point");
            prop_assert!(!int.is_empty() && int.bytes().all(|b| b.is_ascii_digit()));
            prop_assert!(frac.bytes().all(|b| b.is_ascii_digit()));
            if let Some(e) = exponent {
                let e = e.strip_prefix('-').unwrap_or(e);
                prop_assert!(!e.is_empty() && e.bytes().all(|b| b.is_ascii_digit()));
            }
        }
    }
}

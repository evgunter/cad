//! A decimal: what a person writes a number in a unit as.

use core::fmt;

use crate::exact::Nat;

/// **A decimal number**: a sign, digits and a power of ten. A number
/// written in a unit is one of these, not a float of the unit: what it
/// is in canonical units is [`crate::UnitDef::canonical`], the decimal
/// times the unit's exact factor rounded once.
///
/// Held normalized, so two decimals of one value and sign are equal:
/// no leading or trailing zero digit, and zero as no digits. A zero
/// keeps its sign, as `-0 mm` does.
///
/// Its `Display` is positional for `1e-4 ≤ |d| < 1e16` and for zero,
/// and scientific (`1.5e308`, `1e-300`) outside them; the decimal a
/// float reads as ([`Decimal::of_float`]) displays as that float's
/// `Debug` does, less a trailing `.0`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Decimal {
    negative: bool,
    /// ASCII digits, the first and last nonzero; empty for zero.
    digits: String,
    /// The value is `digits × 10^exponent`.
    exponent: i64,
}

/// The magnitude an exponent is held to: past it every decimal the
/// digits could make is past the format at either end.
const EXPONENT_BOUND: i64 = 1 << 60;

impl Decimal {
    /// Zero, positive.
    pub const ZERO: Self = Self {
        negative: false,
        digits: String::new(),
        exponent: 0,
    };

    /// `±digits × 10^exponent`, normalized.
    fn new(negative: bool, digits: &str, exponent: i64) -> Self {
        let lead = digits.bytes().take_while(|&b| b == b'0').count();
        let digits = &digits[lead..];
        let trail = digits.bytes().rev().take_while(|&b| b == b'0').count();
        let kept = &digits[..digits.len() - trail];
        if kept.is_empty() {
            return Self {
                negative,
                ..Self::ZERO
            };
        }
        Self {
            negative,
            digits: kept.to_owned(),
            exponent: exponent
                .saturating_add(trail as i64)
                .clamp(-EXPONENT_BOUND, EXPONENT_BOUND),
        }
    }

    /// `±mantissa × 10^exponent`.
    pub(crate) fn from_parts(negative: bool, mantissa: u64, exponent: i64) -> Self {
        Self::new(negative, &mantissa.to_string(), exponent)
    }

    /// The decimal `text` spells: digits with at most one point among
    /// them, at least one digit, an optional sign before, and an
    /// optional exponent after (`e` or `E`, a sign, digits). `None` for
    /// anything else, `inf` and `NaN` among it.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let (negative, unsigned) = match text.as_bytes().first()? {
            b'-' => (true, &text[1..]),
            b'+' => (false, &text[1..]),
            _ => (false, text),
        };
        let (mantissa, exponent) = match unsigned.find(['e', 'E']) {
            Some(at) => (&unsigned[..at], Some(&unsigned[at + 1..])),
            None => (unsigned, None),
        };
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let all_digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
        if whole.len() + fraction.len() == 0 || !all_digits(whole) || !all_digits(fraction) {
            return None;
        }
        let exponent = match exponent {
            None => 0,
            Some(written) => {
                let (sign, digits) = match written.as_bytes().first()? {
                    b'-' => (-1, &written[1..]),
                    b'+' => (1, &written[1..]),
                    _ => (1, written),
                };
                if digits.is_empty() || !all_digits(digits) {
                    return None;
                }
                let magnitude = digits.bytes().fold(0i64, |acc, b| {
                    acc.saturating_mul(10)
                        .saturating_add(i64::from(b - b'0'))
                        .min(EXPONENT_BOUND)
                });
                sign * magnitude
            }
        };
        let digits = format!("{whole}{fraction}");
        Some(Self::new(
            negative,
            &digits,
            exponent.saturating_sub(fraction.len() as i64),
        ))
    }

    /// The decimal a finite float reads as: its shortest digits that
    /// read back as it (std's), with its sign. `None` for NaN and ±∞.
    #[must_use]
    pub fn of_float(x: f64) -> Option<Self> {
        if !x.is_finite() {
            return None;
        }
        let parsed = Self::parse(&format!("{x:e}"));
        let Some(decimal) = parsed else {
            unreachable!("std's exponent form of a finite float, {x:e}, is a decimal")
        };
        Some(decimal)
    }

    /// Whether the sign is negative (a negative zero included).
    #[must_use]
    pub const fn is_negative(&self) -> bool {
        self.negative
    }

    /// Whether this is zero, of either sign.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.digits.is_empty()
    }

    /// The number of significant digits; none for zero.
    #[must_use]
    pub fn figures(&self) -> usize {
        self.digits.len()
    }

    /// This decimal with its sign flipped.
    #[must_use]
    pub fn negated(self) -> Self {
        Self {
            negative: !self.negative,
            ..self
        }
    }

    /// This decimal without its sign.
    #[must_use]
    pub fn magnitude(self) -> Self {
        Self {
            negative: false,
            ..self
        }
    }

    /// The decimal exponent of the leading digit; `None` for zero.
    pub(crate) fn leading_exponent(&self) -> Option<i64> {
        (!self.is_zero()).then(|| self.exponent + self.digits.len() as i64 - 1)
    }

    /// The first `figures` significant digits, the rest dropped.
    pub(crate) fn truncated(&self, figures: usize) -> Self {
        let kept = figures.min(self.digits.len());
        let dropped = (self.digits.len() - kept) as i64;
        Self::new(
            self.negative,
            &self.digits[..kept],
            self.exponent.saturating_add(dropped),
        )
    }

    /// This decimal with a `5` after its last digit: a point strictly
    /// inside the step of its last digit above it.
    pub(crate) fn with_sticky(&self) -> Self {
        Self::new(
            self.negative,
            &format!("{}5", self.digits),
            self.exponent.saturating_sub(1),
        )
    }

    /// This decimal's magnitude one unit of its last digit up.
    pub(crate) fn stepped_up(&self) -> Self {
        let mut digits = self.digits.clone().into_bytes();
        let carried = digits.iter_mut().rev().all(|d| {
            if *d == b'9' {
                *d = b'0';
                true
            } else {
                *d += 1;
                false
            }
        });
        if carried {
            digits.insert(0, b'1');
        }
        let Ok(digits) = String::from_utf8(digits) else {
            unreachable!("ASCII digits stay ASCII")
        };
        Self::new(self.negative, &digits, self.exponent)
    }

    /// The product of two decimals, exactly.
    #[must_use]
    pub fn times(&self, other: &Self) -> Self {
        let product = Nat::from_digits(&self.digits).mul(&Nat::from_digits(&other.digits));
        Self::new(
            self.negative != other.negative,
            &product.to_digits(),
            self.exponent.saturating_add(other.exponent),
        )
    }

    /// The float nearest this decimal, ties to even (the canonical
    /// unit's door, [`crate::UnitDef::canonical`]).
    #[must_use]
    pub fn to_f64(&self) -> f64 {
        crate::M.def().canonical(self)
    }

    pub(crate) fn digits(&self) -> &str {
        &self.digits
    }

    pub(crate) const fn exponent(&self) -> i64 {
        self.exponent
    }
}

/// Positional where that is short, scientific where it is not (the
/// type's docs).
impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.negative {
            f.write_str("-")?;
        }
        let Some(decade) = self.leading_exponent() else {
            return f.write_str("0");
        };
        let digits = self.digits.as_str();
        if (-4..16).contains(&decade) {
            if self.exponent >= 0 {
                f.write_str(digits)?;
                return (0..self.exponent).try_for_each(|_| f.write_str("0"));
            }
            let point = digits.len() as i64 + self.exponent;
            if point > 0 {
                let (whole, fraction) = digits.split_at(point as usize);
                return write!(f, "{whole}.{fraction}");
            }
            f.write_str("0.")?;
            (0..-point).try_for_each(|_| f.write_str("0"))?;
            return f.write_str(digits);
        }
        let (lead, rest) = digits.split_at(1);
        f.write_str(lead)?;
        if !rest.is_empty() {
            write!(f, ".{rest}")?;
        }
        write!(f, "e{decade}")
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]

    use super::Decimal;

    fn d(text: &str) -> Decimal {
        Decimal::parse(text).unwrap_or_else(|| panic!("{text} is a decimal"))
    }

    #[test]
    fn a_decimal_is_held_normalized() {
        assert_eq!(d("300.0400"), d("30004e-2"));
        assert_eq!(d("0.000"), Decimal::ZERO);
        assert!(d("-0").is_negative());
        assert_eq!(d("+1.5E3"), d("1500"));
        assert_eq!(d(".5"), d("5e-1"));
        assert_eq!(d("5."), d("5"));
        for bad in [
            "", "-", ".", "1e", "1e+", "inf", "NaN", "1.2.3", "1e5.0", "0x10",
        ] {
            assert_eq!(Decimal::parse(bad), None, "{bad:?}");
        }
    }

    /// The decimal a float reads as displays as its `Debug` does, less
    /// a trailing `.0`, at every binade and on both sides of the
    /// positional range's two ends.
    #[test]
    fn a_floats_decimal_displays_as_its_debug() {
        let mut x = f64::from_bits(1);
        let mut values = vec![
            0.0,
            -0.0,
            1e-4,
            1e16,
            9.999_999_999_999_999e-5,
            1e-4f64.next_down(),
        ];
        while x.is_finite() {
            values.extend([x, -x, x * 1.5, x.next_up(), 3.0 * x / 7.0]);
            x *= 2.0;
        }
        values.extend([1e16f64.next_down(), f64::MAX, -f64::MAX]);
        for v in values.into_iter().filter(|v| v.is_finite()) {
            let debug = format!("{v:?}");
            let want = debug.strip_suffix(".0").unwrap_or(&debug);
            let decimal = Decimal::of_float(v).unwrap_or_else(|| panic!("{v:e} is finite"));
            assert_eq!(decimal.to_string(), want, "{v:e}");
            assert_eq!(want.parse::<f64>().map(f64::to_bits), Ok(v.to_bits()));
        }
    }
}

//! **The exact rational constant** (VARIABLES-DESIGN VR5): what a
//! number inside a formula is, where a written quantity is a variable.

use geom_core::Real;

use crate::expr::DimensionError;

/// The bound on a constant's reduced numerator and denominator: both
/// embed exactly in an `f64`, so its value is one correctly-rounded
/// division of exact operands.
pub(crate) const RATIO_BOUND: u64 = 1 << 53;

/// **An exact rational constant**: `num / den` in lowest terms, with
/// `den ≥ 1` and `|num|, den ≤ 2^53`.
///
/// The fields are private and every constructor reduces and bounds, so
/// two equal values are one representation and `==` is value equality.
/// No arithmetic exists on it: an operator over constants stays a tree
/// node, so a constant out of range is refused where it is written and
/// nowhere else.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ratio {
    num: i64,
    den: u64,
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl Ratio {
    /// `num / den`, reduced.
    ///
    /// # Errors
    ///
    /// [`DimensionError::ConstantOutOfRange`] for a zero denominator,
    /// or where the reduced numerator or denominator exceeds 2^53.
    pub fn new(num: i64, den: u64) -> Result<Self, DimensionError> {
        let out = || DimensionError::ConstantOutOfRange {
            text: format!("{num}/{den}"),
        };
        Self::of_parts(num < 0, u128::from(num.unsigned_abs()), u128::from(den)).ok_or_else(out)
    }

    /// The integer `n`.
    ///
    /// # Errors
    ///
    /// [`DimensionError::ConstantOutOfRange`] past 2^53.
    pub fn integer(n: i64) -> Result<Self, DimensionError> {
        Self::new(n, 1)
    }

    /// `num / den` exactly as written: in lowest terms already, which
    /// is the one spelling the load door reads.
    ///
    /// # Errors
    ///
    /// [`Self::new`]'s, and [`DimensionError::RatioNotReduced`] for a
    /// ratio that reduces.
    pub fn reduced(num: i64, den: u64) -> Result<Self, DimensionError> {
        let ratio = Self::new(num, den)?;
        if ratio.num == num && ratio.den == den {
            Ok(ratio)
        } else {
            Err(DimensionError::RatioNotReduced { num, den })
        }
    }

    /// The exact value of a decimal numeral — digits, an optional
    /// fraction and an optional exponent, with an optional leading
    /// `-` (`0.1` is 1/10, `2.5e-3` is 1/400).
    ///
    /// # Errors
    ///
    /// [`DimensionError::ConstantOutOfRange`] for text that is not
    /// such a numeral, or whose value reduced exceeds the bound.
    pub fn from_decimal(text: &str) -> Result<Self, DimensionError> {
        let out = || DimensionError::ConstantOutOfRange {
            text: text.to_string(),
        };
        let (negative, body) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let (mantissa, exponent) = match body.find(['e', 'E']) {
            Some(at) => (&body[..at], body[at + 1..].parse::<i32>().map_err(|_| out())?),
            None => (body, 0),
        };
        let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
        let digits = format!("{whole}{fraction}");
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(out());
        }
        // value = digits · 10^scale, every trailing zero moved into the
        // scale so the digits fit wherever the value can.
        let trimmed = digits.trim_end_matches('0');
        let zeros = i32::try_from(digits.len() - trimmed.len()).map_err(|_| out())?;
        let places = i32::try_from(fraction.len()).map_err(|_| out())?;
        let scale = exponent
            .checked_add(zeros)
            .and_then(|s| s.checked_sub(places))
            .ok_or_else(out)?;
        let trimmed = trimmed.trim_start_matches('0');
        if trimmed.is_empty() {
            return Ok(Self { num: 0, den: 1 });
        }
        let mut num: u128 = trimmed.parse().map_err(|_| out())?;
        let mut den: u128 = 1;
        let ten = |n: u128, times: u32| {
            (0..times).try_fold(n, |n, _| n.checked_mul(10).filter(|&n| n <= u128::from(RATIO_BOUND)))
        };
        if scale >= 0 {
            num = ten(num, scale.unsigned_abs()).ok_or_else(out)?;
        } else {
            // 10^-k = 1 / (2^k · 5^k): cancel each factor against the
            // numerator as it is multiplied in, so the denominator never
            // holds more than the reduced value needs.
            for _ in 0..scale.unsigned_abs() {
                for p in [2, 5] {
                    if num % p == 0 {
                        num /= p;
                    } else {
                        den = den.checked_mul(p).filter(|&d| d <= u128::from(RATIO_BOUND)).ok_or_else(out)?;
                    }
                }
            }
        }
        Self::of_parts(negative, num, den).ok_or_else(out)
    }

    /// `±magnitude / den` reduced, `None` where `den` is zero or the
    /// reduced value exceeds the bound.
    fn of_parts(negative: bool, magnitude: u128, den: u128) -> Option<Self> {
        if den == 0 {
            return None;
        }
        let g = gcd(magnitude, den);
        let (magnitude, den) = (magnitude / g, den / g);
        let bound = u128::from(RATIO_BOUND);
        if magnitude > bound || den > bound {
            return None;
        }
        let magnitude = i64::try_from(magnitude).ok()?;
        Some(Self {
            num: if negative { -magnitude } else { magnitude },
            den: u64::try_from(den).ok()?,
        })
    }

    /// The numerator, carrying the sign.
    #[must_use]
    pub fn num(self) -> i64 {
        self.num
    }

    /// The denominator, at least 1.
    #[must_use]
    pub fn den(self) -> u64 {
        self.den
    }

    /// Whether the value is below zero.
    #[must_use]
    pub fn is_negative(self) -> bool {
        self.num < 0
    }

    /// **The value at `T`**: one division of exact operands, so the
    /// correctly-rounded double at `f64` (the bits the decimal's own
    /// parse gives), an outward-rounded enclosure at an interval, a
    /// zero tangent at a dual and the exact rational at the symbolic
    /// tier. An integer skips the division.
    #[allow(clippy::cast_precision_loss)]
    pub fn eval<T: Real>(self) -> T {
        // Exact: both operands are at most 2^53 in magnitude.
        let num = T::from_f64(self.num as f64);
        if self.den == 1 {
            num
        } else {
            num / T::from_f64(self.den as f64)
        }
    }

    /// The decimal text of this value where its denominator is a
    /// product of twos and fives and the text reads back to it.
    fn decimal(self) -> Option<String> {
        let mut den = self.den;
        for p in [2, 5] {
            while den % p == 0 {
                den /= p;
            }
        }
        if den != 1 {
            return None;
        }
        let magnitude = self.num.unsigned_abs();
        let mut text = String::new();
        if self.num < 0 {
            text.push('-');
        }
        text.push_str(&(magnitude / self.den).to_string());
        let mut rest = u128::from(magnitude % self.den);
        if rest != 0 {
            text.push('.');
            while rest != 0 {
                rest *= 10;
                let digit = rest / u128::from(self.den);
                text.push(char::from(b'0' + u8::try_from(digit).ok()?));
                rest %= u128::from(self.den);
            }
        } else {
            // A bare integer is a count in the text grammar.
            text.push_str(".0");
        }
        (Self::from_decimal(&text) == Ok(self)).then_some(text)
    }
}

/// The text the formula grammar reads back as this constant: a decimal
/// where the denominator is a product of twos and fives, `p/q`
/// otherwise.
impl core::fmt::Display for Ratio {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.decimal() {
            Some(text) => f.write_str(&text),
            None => write!(f, "{}/{}", self.num, self.den),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]
    use super::*;

    #[test]
    fn construction_reduces_and_bounds() {
        assert_eq!(Ratio::new(2, 4), Ratio::new(1, 2));
        assert_eq!(Ratio::new(-6, 3).map(|r| (r.num(), r.den())), Ok((-2, 1)));
        assert_eq!(Ratio::new(1 << 54, 2).map(Ratio::num), Ok(1 << 53));
        assert!(matches!(
            Ratio::new(1 << 54, 1),
            Err(DimensionError::ConstantOutOfRange { .. })
        ));
        assert!(matches!(
            Ratio::new(1, 0),
            Err(DimensionError::ConstantOutOfRange { .. })
        ));
        assert_eq!(
            Ratio::reduced(2, 4),
            Err(DimensionError::RatioNotReduced { num: 2, den: 4 })
        );
    }

    #[test]
    fn a_decimal_is_its_exact_value() {
        let r = |t| Ratio::from_decimal(t).map(|r| (r.num(), r.den()));
        assert_eq!(r("0.1"), Ok((1, 10)));
        assert_eq!(r("2.5e-3"), Ok((1, 400)));
        assert_eq!(r("-1.50"), Ok((-3, 2)));
        assert_eq!(r("1e3"), Ok((1000, 1)));
        assert_eq!(r("0.000"), Ok((0, 1)));
        assert!(r("1e-30").is_err());
        assert!(r("1e30").is_err());
        assert!(r("1.2.3").is_err());
    }

    #[test]
    fn the_f64_value_is_the_decimal_parse() {
        for text in ["0.1", "0.3", "2.5e-3", "1e-15", "123.456", "-7.77", "0.999999"] {
            let ratio = Ratio::from_decimal(text).unwrap();
            assert_eq!(
                ratio.eval::<f64>().to_bits(),
                text.parse::<f64>().unwrap().to_bits(),
                "{text}"
            );
        }
    }

    #[test]
    fn the_text_reads_back() {
        for (num, den, want) in [
            (1, 10, "0.1"),
            (-3, 2, "-1.5"),
            (2, 1, "2.0"),
            (1, 3, "1/3"),
            (-2, 7, "-2/7"),
            (1, 1 << 53, "0.00000000000000011102230246251565404236316680908203125"),
            (1, 3 << 50, "1/3377699720527872"),
        ] {
            let ratio = Ratio::new(num, den).unwrap();
            assert_eq!(ratio.to_string(), want);
        }
    }
}

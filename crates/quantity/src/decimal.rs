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
/// `Debug` does, less a trailing `.0`. [`Decimal::positional`] is the
/// same digits positional at every magnitude.
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

    /// **This decimal spelled positionally at every magnitude**: its
    /// digits with the point among them, `0.` and zeros before them
    /// below one, and zeros after them past their last place, never an
    /// exponent. It is the text an editable field shows
    /// (`crates/geom-core/README.md`, "A number told to a person"),
    /// which is edited one character at a time, and it reads back as
    /// this decimal exactly. A zero keeps its sign, as `Display` does.
    /// Its width is the magnitude's: `1e-300` is 302 characters, and a
    /// finite float's decimal in any unit of [`crate::UNITS`] is at
    /// most 330.
    ///
    /// **For a spelling of a finite float.** [`Decimal::parse`] admits
    /// exponents far past any float's, and a positional spelling of one
    /// is as long as its exponent; so a decimal whose leading digit is
    /// past [`POSITIONAL_DECADES`] in either direction, which no finite
    /// float's decimal in a unit reaches, is written as `Display` writes
    /// it, in exponent form, rather than as a string that size.
    #[must_use]
    pub const fn positional(&self) -> Positional<'_> {
        Positional(self)
    }

    /// The decimal exponent of the leading digit; `None` for zero.
    pub(crate) fn leading_exponent(&self) -> Option<i64> {
        (!self.is_zero()).then(|| self.exponent + self.digits.len() as i64 - 1)
    }

    /// The step of the `figures`-th significant figure that this
    /// decimal's magnitude lies strictly inside, for a decimal of more
    /// figures than that: its two ends (the first `figures` digits, and
    /// those one unit of the last up) and a point strictly between
    /// them (the first `figures` digits with a `5` after them). The
    /// figures are positional: a zero among the first `figures` is a
    /// figure, so the step is always that of the `figures`-th digit.
    pub(crate) fn bracket(&self, figures: usize) -> [Self; 3] {
        assert!(
            self.digits.len() > figures,
            "a decimal of {} figures is inside no step of its {figures}-th",
            self.digits.len()
        );
        let head = &self.digits[..figures];
        let dropped = (self.digits.len() - figures) as i64;
        let exponent = self.exponent.saturating_add(dropped);
        let mut up = head.as_bytes().to_vec();
        let carried = up.iter_mut().rev().all(|d| {
            if *d == b'9' {
                *d = b'0';
                true
            } else {
                *d += 1;
                false
            }
        });
        if carried {
            up.insert(0, b'1');
        }
        let Ok(up) = String::from_utf8(up) else {
            unreachable!("ASCII digits stay ASCII")
        };
        [
            Self::new(self.negative, head, exponent),
            Self::new(self.negative, &up, exponent),
            Self::new(
                self.negative,
                &format!("{head}5"),
                exponent.saturating_sub(1),
            ),
        ]
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

    /// The float nearest this decimal times `factor`, ties to even: the
    /// exact product rounded once, as [`Decimal::times`] then
    /// [`Decimal::to_f64`] would give it.
    ///
    /// Where the product's digits are an integer below `2^53` and its
    /// power of ten is at most 22 either way, both are floats and one
    /// float multiply or divide is the product correctly rounded (IEEE
    /// 754); where the digits fit `u128` the quotient is rounded in
    /// integers without building the product's decimal; past both, the
    /// exact path.
    #[must_use]
    pub fn times_to_f64(&self, factor: &Self) -> f64 {
        const POWERS: [f64; 23] = [
            1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15,
            1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22,
        ];
        let sign = if self.negative != factor.negative {
            -1.0
        } else {
            1.0
        };
        let digits = |d: &Self| {
            (d.digits.len() <= 19)
                .then(|| d.digits.parse::<u64>().ok())
                .flatten()
        };
        if self.is_zero() || factor.is_zero() {
            return sign * 0.0;
        }
        if let (Some(a), Some(b)) = (digits(self), digits(factor)) {
            let product = u128::from(a) * u128::from(b);
            let exponent = self.exponent.saturating_add(factor.exponent);
            let power = usize::try_from(exponent.unsigned_abs())
                .ok()
                .and_then(|k| POWERS.get(k));
            if let Some(&power) = power
                && product < 1 << 53
            {
                let n = product as f64;
                return sign * if exponent >= 0 { n * power } else { n / power };
            }
            if (-36..=0).contains(&exponent) {
                let d = Nat::pow10(exponent.unsigned_abs());
                return sign * crate::exact::round_to_f64(&Nat::from_u128(product), &d);
            }
        }
        self.times(factor).to_f64()
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
        let Some(decade) = self
            .leading_exponent()
            .filter(|decade| !(-4..16).contains(decade))
        else {
            return Positional(self).fmt(f);
        };
        if self.negative {
            f.write_str("-")?;
        }
        let digits = self.digits.as_str();
        let (lead, rest) = digits.split_at(1);
        f.write_str(lead)?;
        if !rest.is_empty() {
            write!(f, ".{rest}")?;
        }
        write!(f, "e{decade}")
    }
}

/// The decades of the leading digit [`Decimal::positional`] spells
/// positionally: every finite float's decimal in a unit of
/// [`crate::UNITS`] (the float format spans 1e-324 to 1e309, and no
/// unit is more than a few decades from canonical), with room.
pub const POSITIONAL_DECADES: i64 = 1000;

/// A [`Decimal`] displayed positionally ([`Decimal::positional`]).
#[derive(Clone, Copy, Debug)]
pub struct Positional<'a>(&'a Decimal);

impl fmt::Display for Positional<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Decimal {
            negative,
            digits,
            exponent,
        } = self.0;
        if self
            .0
            .leading_exponent()
            .is_some_and(|decade| decade.abs() > POSITIONAL_DECADES)
        {
            // `Display` is scientific there.
            return fmt::Display::fmt(self.0, f);
        }
        if *negative {
            f.write_str("-")?;
        }
        if self.0.is_zero() {
            return f.write_str("0");
        }
        if *exponent >= 0 {
            f.write_str(digits)?;
            return (0..*exponent).try_for_each(|_| f.write_str("0"));
        }
        let point = digits.len() as i64 + exponent;
        if point > 0 {
            let (whole, fraction) = digits.split_at(point as usize);
            return write!(f, "{whole}.{fraction}");
        }
        f.write_str("0.")?;
        (0..-point).try_for_each(|_| f.write_str("0"))?;
        f.write_str(digits)
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

    /// **A positional spelling is the decimal, with no exponent, at
    /// every magnitude**: it reads back as the same decimal, it is the
    /// `Display` wherever that is positional, and its zeros are the
    /// magnitude's (`1e-5` is `0.00001`, `1.5e17` is eighteen digits).
    #[test]
    fn a_decimal_past_every_floats_decades_is_not_spelled_out() {
        for text in ["1e1001", "-2.5e-1001", "1e100000"] {
            assert_eq!(d(text).positional().to_string(), d(text).to_string());
        }
        assert_eq!(d("1e1000").positional().to_string().len(), 1001);
    }

    #[test]
    fn a_positional_spelling_is_the_decimal_at_every_magnitude() {
        for (text, positional) in [
            ("1e-5", "0.00001"),
            ("-4e-5", "-0.00004"),
            ("1.5e17", "150000000000000000"),
            ("123.45", "123.45"),
            ("-0", "-0"),
            ("0", "0"),
        ] {
            assert_eq!(d(text).positional().to_string(), positional, "{text}");
        }
        let mut x = f64::from_bits(1);
        while x.is_finite() {
            for v in [x, -x, x * 1.5, 3.0 * x / 7.0]
                .into_iter()
                .filter(|v| v.is_finite())
            {
                let decimal = Decimal::of_float(v).unwrap_or_else(|| panic!("{v:e} is finite"));
                let spelled = decimal.positional().to_string();
                assert!(!spelled.contains('e'), "{v:e} is spelled {spelled}");
                assert_eq!(Decimal::parse(&spelled), Some(decimal.clone()), "{v:e}");
                if (1e-4..1e16).contains(&v.abs()) {
                    assert_eq!(spelled, decimal.to_string(), "{v:e}");
                }
            }
            x *= 2.0;
        }
    }

    /// A product's float takes one of three paths (one float operation,
    /// a rounding in integers, the exact decimal), and each answers as
    /// the exact one does: over a seeded draw of decimals of 1 to 19
    /// figures and the factors a file states (powers of ten, an inch in
    /// metres and in millimetres, a degree).
    #[test]
    fn a_products_float_is_the_exact_products_on_every_path() {
        let factors = [
            "1e-3",
            "1e-2",
            "1e3",
            "1e-18",
            "0.0254",
            "25.4e-3",
            "0.0174532925199433",
        ];
        let mut state = 0x9e37_79b9_7f4a_7c15_u64;
        let mut next = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..20_000 {
            let figures = 1 + (next() % 19) as u32;
            let digits = next() % 10u64.pow(figures);
            let exponent = (next() % 80) as i64 - 50;
            let negative = next() % 2 == 0;
            let number = Decimal::from_parts(negative, digits, exponent);
            for factor in factors {
                let factor = d(factor);
                assert_eq!(
                    number.times_to_f64(&factor).to_bits(),
                    number.times(&factor).to_f64().to_bits(),
                    "{number} times {factor}"
                );
            }
        }
    }
}

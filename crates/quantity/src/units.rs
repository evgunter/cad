//! The unit table: symbols, quantities, and exact conversion factors
//! into canonical kernel units — as DATA, shaped after the one prior
//! unit table in the tree (step-import's `units.rs` prefix/factor
//! tables).
//!
//! Factors are canonical-units-per-one-of-this-unit, held EXACTLY:
//!
//! * `MM`/`CM` are PREFIXED METRES, 1/1000 and 1/100. Lengths take
//!   prefixes; angles do not (no `mdeg` row can exist here) — the
//!   census's lengths-take-prefixes rule, mirrored from step-import's
//!   `si_unit_kind`, where a prefixed `RADIAN` refuses.
//! * `IN` is EXACTLY 25.4 mm (the international inch), 254/10000.
//! * `DEG` is π/180, with true π: no float or decimal is it, so a number
//!   in degrees is decided against enclosures of π (`exact`'s docs).
//! * `PI` is the half-turn, π — the unit that is not a unit. It
//!   measures an angle exactly as `DEG` does and is carried here for
//!   the same reason every other row is: a literal REMEMBERS the unit
//!   it was authored in, so `0.5 pi rad` is how a user says "write this
//!   angle as a multiple of π" and get it back that way. Nothing
//!   downstream distinguishes it from `DEG` — an angle is canonical
//!   radians whichever row produced it. Its symbol is TWO WORDS,
//!   `pi rad`, and that is the whole surface spelling: the multiplier
//!   and the unit it multiplies, so a picker offering it beside `deg`
//!   and `rad` reads as "multiples of π radians" rather than as the
//!   number π. It is the only multi-word symbol in the table, and the
//!   text door's unit suffix is a longest match over adjacent
//!   identifiers because of it.
//! * `M`/`RAD` are the canonical units themselves, factor exactly 1.
//!
//! A number in a unit is a decimal, and what it is in canonical units
//! is [`UnitDef::canonical`]: the decimal times the exact factor,
//! rounded once. [`UnitDef::factor`] is the float nearest the factor,
//! what one of the unit is, and nothing multiplies by it.

use crate::decimal::Decimal;
use crate::exact::{Magnitude, Nat, Real};

/// The SI prefix 10⁻³ as the float nearest it — what `1 mm` is in
/// metres. Prefixes apply to the metre only (module docs).
pub const MILLI: f64 = 1e-3;

/// The SI prefix 10⁻² as the float nearest it — what `1 cm` is.
pub const CENTI: f64 = 1e-2;

/// What a unit measures — the continuous half of the dimension set
/// (`Count` has no units at all: a count is a bare integer).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnitQuantity {
    /// A length; factors carry into canonical meters.
    Length,
    /// A plane angle; factors carry into canonical radians.
    Angle,
    /// A dimensionless real. **Exactly one row measures it** — [`ONE`],
    /// whose symbol is the EMPTY string, because the way a
    /// dimensionless number is written is with no suffix at all.
    ///
    /// It is a row rather than an absence so that every stored literal
    /// can name the notation it was written in, leaving no "no
    /// notation" state for two readers to interpret differently.
    ///
    /// A chooser has nothing to offer for it — one option is not a
    /// choice — but that is a rule for whatever draws the chooser, not
    /// one this table keeps.
    Scalar,
}

impl UnitQuantity {
    /// Every row of [`UNITS`] that measures this quantity, in table
    /// order — read off the table, so a row added there is a row here.
    pub fn units(self) -> impl Iterator<Item = UnitDef> {
        UNITS.into_iter().filter(move |row| row.quantity() == self)
    }
}

/// One row of the unit table: a display/parse symbol, the quantity it
/// measures, and the exact factor into canonical units.
///
/// **SEALED — a `UnitDef` is a row of [`UNITS`], not a triple of three
/// independent fields (issue #650).** The fields are private and there
/// is no constructor at all — not a public one, not a `#[doc(hidden)]`
/// test-only one. The only values of this type a caller can hold are
/// copies of the table's own rows: [`UNITS`] itself, [`unit_by_symbol`],
/// and whatever hands one back (`Expr::display_unit`).
///
/// **The invariant this buys, which downstream crates rely on: the
/// symbol DETERMINES the other two fields.** A row whose `symbol` is
/// `"mm"` and whose `quantity` is `Angle` is not merely refused, it is
/// unrepresentable. That is what closes #650, where such a row defeated
/// `Formula::literal_with_unit`'s dimension guard — the guard read the
/// CALLER's `quantity`, then stored the table's row unchecked — and
/// produced an `Expr` that serialized into a document editor-core's own
/// load door then refused (`DisplayUnitMismatch`): a round-trip break,
/// not a rejected call. D2 addendum: the answer chosen is "make the
/// illegal state unrepresentable", not row 1's typed error, because the
/// input was never legitimate in the first place.
///
/// The two typed views [`LengthUnit`] and [`AngleUnit`] are sealed by a
/// stronger mechanism (issue #669): each is a private INDEX into
/// [`UNITS`], so "is a row of the table" is what the value is, not a
/// convention about how it is built. [`UnitDef::as_length`] and
/// [`UnitDef::as_angle`] are the public route from a row to its view;
/// [`LengthUnit::def`] / [`AngleUnit::def`] are the inverse, and are
/// public because the seal never depended on their visibility — a view
/// IS an index into this table, so handing back the row it indexes
/// cannot produce a pairing the table does not have.
///
/// **This rustdoc is the one home of that narrative.** Everything else
/// that touches the seal points here rather than restating it.
///
/// The seal is a claim about what COMPILES, so it is pinned by rows
/// that must fail to compile. These are library doctests — they run
/// under `cargo test -p quantity --doc`, so undoing the seal turns them
/// red rather than merely dating a comment.
///
/// #650's counterexample, a row assembled with a foreign quantity, no
/// longer builds, and the struct-update escape from a real row is the
/// one spelling of it that names only fields a caller can see (a full
/// literal must also name the exact factor, whose type is private, so
/// it fails whatever the fields' visibility and pins nothing):
///
/// ```compile_fail,E0451
/// let mm = quantity::unit_by_symbol("mm").unwrap();
/// let bogus = quantity::UnitDef { quantity: quantity::UnitQuantity::Angle, ..mm };
/// ```
///
/// It is refused on PRIVACY (`E0451`), which is the property that has
/// to hold: it reddens the moment the field is made public, and a row
/// that merely named a field that does not exist would not.
///
/// A `compile_fail` row proves only that the snippet does not build,
/// not that it fails for the intended reason — a typo would pass it
/// just as well, and stable rustdoc checks only that the block fails,
/// never that it fails with the `,E0451` named beside it. The block
/// above therefore has its legal twin here, differing from it in
/// exactly one respect: the twin never names the field, so a typo
/// shared by both reddens the twin. The code was read off `rustc`
/// directly on the snippet at the pinned toolchain (1.97.0).
///
/// ```
/// // The path `quantity::UnitDef` names this exported type, the row
/// // and the enum variant exist, and every field is READABLE through
/// // its accessor. Naming the type binds the twin to it, so a rename
/// // or an un-export reddens here rather than quietly satisfying the
/// // `compile_fail` blocks for the wrong reason.
/// let mm: quantity::UnitDef = quantity::unit_by_symbol("mm").expect("mm is a table row");
/// assert_eq!(mm.symbol(), "mm");
/// assert_eq!(mm.quantity(), quantity::UnitQuantity::Length);
/// assert_eq!(mm.factor(), quantity::MILLI);
/// assert_eq!(quantity::UnitQuantity::Angle, quantity::UnitQuantity::Angle);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnitDef {
    symbol: &'static str,
    quantity: UnitQuantity,
    factor: f64,
    exact: Factor,
}

/// A unit's exact factor into canonical units: `num / den`, times π
/// where `pi`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Factor {
    num: u16,
    den: u16,
    pi: bool,
}

impl Factor {
    const fn rational(num: u16, den: u16) -> Self {
        Self {
            num,
            den,
            pi: false,
        }
    }

    const fn times_pi(num: u16, den: u16) -> Self {
        Self { num, den, pi: true }
    }

    /// Whether the factor is a power of ten (`mm`, `cm`, `m`, `rad`, the
    /// dimensionless row).
    const fn is_power_of_ten(self) -> bool {
        !self.pi && self.num == 1 && matches!(self.den, 1 | 10 | 100 | 1000 | 10000)
    }

    /// `|number| × factor` as a quotient of two floats that are each an
    /// integer below `2^53`, where it is one and the factor is rational.
    fn small_terms(self, number: &Decimal) -> Option<(f64, f64)> {
        const EXACT: u64 = 1 << 53;
        if self.pi || number.figures() > 15 {
            return None;
        }
        let digits: u64 = number.digits().parse().ok()?;
        let power = 10u64.checked_pow(u32::try_from(number.exponent().unsigned_abs()).ok()?)?;
        let (mut n, mut d) = (
            digits.checked_mul(u64::from(self.num))?,
            u64::from(self.den),
        );
        if number.exponent() >= 0 {
            n = n.checked_mul(power)?;
        } else {
            d = d.checked_mul(power)?;
        }
        (n < EXACT && d < EXACT).then_some((n as f64, d as f64))
    }

    /// `|number| × factor`, exactly.
    fn times(self, number: &Decimal) -> Real {
        let digits = Nat::from_digits(number.digits()).mul(&Nat::from_u64(u64::from(self.num)));
        let den = Nat::from_u64(u64::from(self.den));
        let shift = Nat::pow10(number.exponent().unsigned_abs());
        let (n, d) = if number.exponent() >= 0 {
            (digits.mul(&shift), den)
        } else {
            (digits, den.mul(&shift))
        };
        Real {
            n,
            d,
            pi: i8::from(self.pi),
        }
    }

    /// `|canonical| / factor`, exactly, for a finite nonzero `canonical`.
    fn over(self, canonical: f64) -> Real {
        let bits = canonical.abs().to_bits();
        let (field, fraction) = ((bits >> 52) as i64, bits & ((1 << 52) - 1));
        let (mantissa, exponent) = if field == 0 {
            (fraction, -1074)
        } else {
            (fraction | (1 << 52), field - 1075)
        };
        let power = Nat::from_u64(1).shl(exponent.unsigned_abs());
        let (up, down) = if exponent >= 0 {
            (power, Nat::from_u64(1))
        } else {
            (Nat::from_u64(1), power)
        };
        Real {
            n: Nat::from_u64(mantissa)
                .mul(&up)
                .mul(&Nat::from_u64(u64::from(self.den))),
            d: down.mul(&Nat::from_u64(u64::from(self.num))),
            pi: -i8::from(self.pi),
        }
    }
}

/// How a number is rounded to a decimal of some number of figures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rounding {
    /// To the nearer, ties to the even last digit.
    Nearest,
    /// Toward +∞.
    Up,
    /// Toward −∞.
    Down,
}

/// The figures of a decimal that can decide its rounding in a unit
/// whose factor is a power of ten, with a margin ([`UnitDef::canonical`]).
pub const DECIDING_FIGURES: usize = 800;

/// The most significant figures [`UnitDef::shortest`] needs: every
/// finite float has a decimal of at most this many that, written in
/// any unit of the table, is that float (the door's docs argue it).
pub const ROUND_TRIP_FIGURES: usize = 17;

/// `|canonical|` told in a unit: the exact quotient by the factor and
/// the decade of its leading digit.
#[derive(Clone, Debug)]
struct Quotient {
    real: Real,
    decade: i64,
    /// The quotient is `10^decade` exactly.
    is_power: bool,
    negative: bool,
}

impl Quotient {
    /// For a finite nonzero `canonical`.
    fn of(canonical: f64, unit: UnitDef) -> Self {
        let real = unit.exact.over(canonical);
        let estimate = (canonical.abs().log10() - unit.factor.log10()).floor() as i64;
        let (decade, is_power) = real.decade(estimate);
        Self {
            real,
            decade,
            is_power,
            negative: canonical.is_sign_negative(),
        }
    }

    /// The decimal of `figures` significant digits, rounded as
    /// `rounding` says.
    fn at(&self, figures: usize, rounding: Rounding) -> Decimal {
        let magnitude = match (rounding, self.negative) {
            (Rounding::Nearest, _) => Magnitude::Nearest,
            (Rounding::Up, false) | (Rounding::Down, true) => Magnitude::Ceil,
            (Rounding::Up, true) | (Rounding::Down, false) => Magnitude::Floor,
        };
        self.rounded(figures, magnitude)
    }

    /// The decimal of `figures` significant digits, its magnitude
    /// rounded as `magnitude` says.
    fn rounded(&self, figures: usize, magnitude: Magnitude) -> Decimal {
        assert!(
            (1..=19).contains(&figures),
            "a decimal of {figures} figures is not one a u64 holds"
        );
        let k = self.decade - (figures as i64 - 1);
        let (floor, rest) = self.real.scaled(k);
        Decimal::from_parts(self.negative, magnitude.round(floor, rest), k)
    }

    /// The decimals of `figures` significant digits about the quotient:
    /// nearest (ties away from zero), and the floor and ceiling of its
    /// magnitude, from one division.
    fn about(&self, figures: usize) -> [Decimal; 3] {
        let k = self.decade - (figures as i64 - 1);
        let (floor, rest) = self.real.scaled(k);
        [Magnitude::NearestAway, Magnitude::Floor, Magnitude::Ceil]
            .map(|magnitude| Decimal::from_parts(self.negative, magnitude.round(floor, rest), k))
    }
}

/// **A canonical float as an exact number of a unit**
/// ([`UnitDef::in_unit`]), for a caller that asks several questions of
/// one value: its decimal at several numbers of figures, its decade.
/// The quotient and its decade are found once.
#[derive(Clone, Debug)]
pub struct InUnit {
    /// `None` for a zero, which keeps its sign.
    quotient: Option<Quotient>,
    negative: bool,
}

impl InUnit {
    /// The decimal of `figures` significant digits nearest, above or
    /// below the exact number, as `rounding` says; a zero of either
    /// sign is itself.
    ///
    /// # Panics
    ///
    /// Where `figures` is not in `1..=19`: a caller's bug.
    #[must_use]
    pub fn decimal(&self, figures: usize, rounding: Rounding) -> Decimal {
        match &self.quotient {
            Some(quotient) => quotient.at(figures, rounding),
            None => Decimal::from_parts(self.negative, 0, 0),
        }
    }

    /// The decimal exponent of the exact number's leading digit, and
    /// whether the number is that power of ten exactly.
    ///
    /// # Panics
    ///
    /// For a zero, which has no decade: a caller's bug.
    #[must_use]
    pub fn decade(&self) -> (i64, bool) {
        let Some(quotient) = &self.quotient else {
            unreachable!("a caller asked the decade of zero, which has none")
        };
        (quotient.decade, quotient.is_power)
    }
}

impl UnitDef {
    /// The surface symbol (`"mm"`), as parsed and as displayed.
    pub const fn symbol(&self) -> &'static str {
        self.symbol
    }

    /// The quantity this unit measures.
    pub const fn quantity(&self) -> UnitQuantity {
        self.quantity
    }

    /// Canonical units (meters/radians) per one of this unit, as the
    /// float nearest the exact factor: what `1` of this unit is. A
    /// number of the unit is not this times the number; it is
    /// [`UnitDef::canonical`].
    pub const fn factor(&self) -> f64 {
        self.factor
    }

    /// **What `number` written in this unit is**: the decimal times the
    /// unit's exact factor (1/1000 for `mm`, 254/10000 for `in`, π/180
    /// for `deg` with true π), rounded once to the nearest float, ties
    /// to even; ±∞ past the format and a zero of the decimal's sign
    /// below it.
    ///
    /// This is the one door for a number in a unit: the text door, the
    /// Rust and Python `number * unit` ([`UnitDef::canonical_of_float`])
    /// and a told number's reading back all go through it.
    ///
    /// **Its cost is bounded by [`DECIDING_FIGURES`], not by the length
    /// of the decimal**, save in one case. A decimal of more figures is
    /// read as its first [`DECIDING_FIGURES`] with the rest as a sticky
    /// unit: its value lies strictly between that head and the head one
    /// unit of its last place up (the rest is not zero, a decimal being
    /// held without trailing zeros), and where both of those round to
    /// one float, so does it. In a unit whose factor is a power of ten
    /// they always do: every float and every midpoint between floats is
    /// a decimal of at most 769 significant figures (2^−1075 has 752,
    /// and an odd mantissa adds at most 17), and a power of ten moves no
    /// figure, so no rounding boundary lies strictly inside a step of
    /// the [`DECIDING_FIGURES`]-th figure, and the head with a `5` after
    /// it rounds as the decimal does. In `in`, `deg` and `pi rad`
    /// a boundary is a repeating or irrational decimal, and a decimal
    /// of more than [`DECIDING_FIGURES`] figures that agrees with one
    /// through all of them is decided at its full length, at a cost
    /// quadratic in that length (the module docs of `exact` give the
    /// measured figures). No number a person types or pastes, a float
    /// spells, or a file carries agrees with such a boundary by chance.
    #[must_use]
    pub fn canonical(self, number: &Decimal) -> f64 {
        let sign = if number.is_negative() { -1.0 } else { 1.0 };
        let Some(decade) = number.leading_exponent() else {
            return sign * 0.0;
        };
        // Past these the value is past the format whatever the factor
        // (every factor is within [1/1000, π]).
        if decade > 400 {
            return sign * f64::INFINITY;
        }
        if decade < -400 {
            return sign * 0.0;
        }
        if number.figures() > DECIDING_FIGURES {
            let [head, up, inside] = number.bracket(DECIDING_FIGURES);
            if self.exact.is_power_of_ten() {
                // No boundary strictly inside the step: any point of it
                // rounds as the decimal does.
                return sign * self.magnitude_of(&inside);
            }
            let (below, above) = (self.magnitude_of(&head), self.magnitude_of(&up));
            if below.to_bits() == above.to_bits() {
                return sign * below;
            }
        }
        sign * self.magnitude_of(number)
    }

    /// `|number| × factor` rounded once.
    fn magnitude_of(self, number: &Decimal) -> f64 {
        // Both terms of the quotient exact floats: one division rounds
        // it correctly (IEEE 754), the common case of a short literal.
        if let Some((n, d)) = self.exact.small_terms(number) {
            return n / d;
        }
        self.exact.times(number).to_f64()
    }

    /// [`UnitDef::canonical`] of a float written in this unit, which is
    /// read as its shortest decimal ([`Decimal::of_float`]): `300.04 *
    /// MM` is the float nearest 0.30004. NaN and ±∞ are themselves.
    #[must_use]
    pub fn canonical_of_float(self, number: f64) -> f64 {
        Decimal::of_float(number).map_or(number, |decimal| self.canonical(&decimal))
    }

    /// The decimal of `figures` significant digits nearest, above or
    /// below the exact number of this unit that `canonical` is, as
    /// `rounding` says; a zero of either sign is itself.
    ///
    /// # Panics
    ///
    /// Where `canonical` is not finite, or `figures` is not in
    /// `1..=19`: a caller's bug.
    #[must_use]
    pub fn decimal(self, canonical: f64, figures: usize, rounding: Rounding) -> Decimal {
        self.in_unit(canonical).decimal(figures, rounding)
    }

    /// **The shortest decimal that, written in this unit, is `canonical`
    /// to the bit** ([`UnitDef::canonical`]), and of two as short the
    /// nearer the exact quotient, the one away from zero where they are
    /// as near (std's choice for a float's digits). It has at most
    /// [`ROUND_TRIP_FIGURES`] figures.
    ///
    /// The decimals that are `canonical` are an interval of the exact
    /// quotient's neighbourhood: those whose product with the factor
    /// rounds to it. Its width is the float's rounding interval over
    /// the factor, so relative to the quotient it is more than `2^−53`
    /// (more than `3 · 2^−54` at a power of two), wider than the step
    /// of a seventeen-figure decimal at any leading digit, at most
    /// `10^−16` of it, whatever the factor. So it holds a decimal of
    /// seventeen figures, and the floor or the ceiling of the quotient
    /// at the shortest length that has one is in it.
    ///
    /// # Panics
    ///
    /// Where `canonical` is not finite: a caller's bug.
    #[must_use]
    pub fn shortest(self, canonical: f64) -> Decimal {
        assert!(canonical.is_finite(), "{canonical} has no decimal");
        if canonical == 0.0 {
            return Decimal::from_parts(canonical.is_sign_negative(), 0, 0);
        }
        let quotient = Quotient::of(canonical, self);
        let reads = |d: &Decimal| self.canonical(d).to_bits() == canonical.to_bits();
        let at = |figures: usize| {
            let [nearest, floor, ceil] = quotient.about(figures);
            if reads(&nearest) {
                return Some(nearest);
            }
            [floor, ceil]
                .into_iter()
                .find(|d| *d != nearest && reads(d))
        };
        // Having a decimal of some length is monotone in the length (a
        // decimal of n figures is one of n + 1), so the shortest is
        // found by bisection.
        let Some(mut best) = at(ROUND_TRIP_FIGURES) else {
            unreachable!(
                "{canonical:e} in {} has a decimal of {ROUND_TRIP_FIGURES} figures (the docs)",
                self.symbol
            )
        };
        let (mut lo, mut hi) = (1, ROUND_TRIP_FIGURES);
        while lo < hi {
            let mid = (lo + hi) / 2;
            match at(mid) {
                Some(found) => {
                    best = found;
                    hi = mid;
                }
                None => lo = mid + 1,
            }
        }
        best
    }

    /// The decimal exponent of the leading digit of the exact number of
    /// this unit that `canonical` is, and whether that number is the
    /// power of ten exactly.
    ///
    /// # Panics
    ///
    /// Where `canonical` is zero or not finite: a caller's bug.
    #[must_use]
    pub fn decade(self, canonical: f64) -> (i64, bool) {
        self.in_unit(canonical).decade()
    }

    /// `canonical` as an exact number of this unit, to ask several
    /// questions of ([`InUnit`]).
    ///
    /// # Panics
    ///
    /// Where `canonical` is not finite: a caller's bug.
    #[must_use]
    pub fn in_unit(self, canonical: f64) -> InUnit {
        assert!(canonical.is_finite(), "{canonical} is no number of a unit");
        InUnit {
            quotient: (canonical != 0.0).then(|| Quotient::of(canonical, self)),
            negative: canonical.is_sign_negative(),
        }
    }

    /// The float nearest the exact number of this unit that `canonical`
    /// is (one rounding): ±∞ past the format and a signed zero below
    /// it. A value that is not finite is itself.
    #[must_use]
    pub fn quotient(self, canonical: f64) -> f64 {
        if !canonical.is_finite() || canonical == 0.0 {
            return canonical;
        }
        let magnitude = self.exact.over(canonical).to_f64();
        if canonical < 0.0 {
            -magnitude
        } else {
            magnitude
        }
    }

    /// [`UnitDef::quotient`], and `None` where it carries a finite value
    /// past the format or a nonzero one to zero, so the unit has no
    /// float for it.
    #[must_use]
    pub fn number_of(self, canonical: f64) -> Option<f64> {
        let q = self.quotient(canonical);
        let lost = canonical.is_finite() && (!q.is_finite() || (q == 0.0 && canonical != 0.0));
        (!lost).then_some(q)
    }

    /// This row as a [`LengthUnit`], or `None` when the row measures an
    /// angle — the public route from a table row to the typed view that
    /// builds a [`crate::Length`].
    ///
    /// Partial, and necessarily so: the quantity is what decides which
    /// view a row has, and it is data. The refusal can only answer
    /// `None` for a row whose own `quantity` says so — never for a
    /// caller-assembled pair, which no longer exists.
    pub const fn as_length(self) -> Option<LengthUnit> {
        match self.quantity {
            UnitQuantity::Length => Some(LengthUnit::of_row(self.symbol)),
            UnitQuantity::Angle | UnitQuantity::Scalar => None,
        }
    }

    /// This row as an [`AngleUnit`], or `None` when the row does not
    /// measure an angle — the mirror of [`UnitDef::as_length`].
    pub const fn as_angle(self) -> Option<AngleUnit> {
        match self.quantity {
            UnitQuantity::Angle => Some(AngleUnit::of_row(self.symbol)),
            UnitQuantity::Length | UnitQuantity::Scalar => None,
        }
    }

    /// This row as a [`ScalarUnit`], or `None` when it measures
    /// something — total on the one dimensionless row.
    pub const fn as_scalar(self) -> Option<ScalarUnit> {
        match self.quantity {
            UnitQuantity::Scalar => Some(ScalarUnit::of_row(self.symbol)),
            UnitQuantity::Length | UnitQuantity::Angle => None,
        }
    }
}

/// The whole closed unit table, as data — the expression text parser's
/// suffix vocabulary and the formatter's display vocabulary are both
/// exactly this list, and a typed view is an index into it.
pub const UNITS: [UnitDef; 8] = [
    // Millimeter: a `MILLI`-prefixed metre.
    UnitDef {
        symbol: "mm",
        quantity: UnitQuantity::Length,
        factor: MILLI,
        exact: Factor::rational(1, 1000),
    },
    // Centimeter: a `CENTI`-prefixed metre.
    UnitDef {
        symbol: "cm",
        quantity: UnitQuantity::Length,
        factor: CENTI,
        exact: Factor::rational(1, 100),
    },
    // Meter — the canonical length unit, factor exactly 1.0.
    UnitDef {
        symbol: "m",
        quantity: UnitQuantity::Length,
        factor: 1.0,
        exact: Factor::rational(1, 1),
    },
    // International inch: EXACTLY 25.4 mm (module docs) — the factor
    // is the correctly-rounded f64 of the exact decimal 0.0254.
    UnitDef {
        symbol: "in",
        quantity: UnitQuantity::Length,
        factor: 0.0254,
        exact: Factor::rational(127, 5000),
    },
    // Degree: π/180 radians — inexact by nature (module docs).
    UnitDef {
        symbol: "deg",
        quantity: UnitQuantity::Angle,
        factor: core::f64::consts::PI / 180.0,
        exact: Factor::times_pi(1, 180),
    },
    // Radian — the canonical angle unit, factor exactly 1.0.
    UnitDef {
        symbol: "rad",
        quantity: UnitQuantity::Angle,
        factor: 1.0,
        exact: Factor::rational(1, 1),
    },
    // Half-turn: π radians — the "unit" that is a notation (module
    // docs). Inexact for the same reason `deg` is, and by the same
    // constant. The symbol is two words: it names the multiplier AND
    // the unit multiplied, which is what makes it read as a unit.
    UnitDef {
        symbol: "pi rad",
        quantity: UnitQuantity::Angle,
        factor: core::f64::consts::PI,
        exact: Factor::times_pi(1, 1),
    },
    // The dimensionless row: no suffix, factor exactly 1.0.
    UnitDef {
        symbol: "",
        quantity: UnitQuantity::Scalar,
        factor: 1.0,
        exact: Factor::rational(1, 1),
    },
];

// A typed view is one byte, so the table it indexes must fit in one —
// the same bound `editor_core::expr::UnitSym` asserts over the same
// table. A table grown past 255 rows fails the BUILD here rather than
// truncating an index.
const _: () = assert!(
    UNITS.len() <= u8::MAX as usize,
    "a typed unit view is one byte: this table has outgrown its index space"
);

/// Byte-wise `str` equality, because `==` is not available in a const
/// fn on this toolchain.
const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// The position of `symbol` in [`UNITS`] — the ONE mint for a typed
/// view, and the reason `LengthUnit`/`AngleUnit` cannot name a unit the
/// table does not have.
///
/// Two callers, both of which make the miss unreachable rather than
/// merely unlikely: the unit constants, where a miss is a
/// compile-time const-eval failure, and [`UnitDef::as_length`] /
/// [`UnitDef::as_angle`], where the argument is a sealed row and so is
/// a copy of an entry of this very array. D2 addendum row 4 — a kernel
/// bug observable in a branch, announced, never discarded. Message-less
/// because a const fn cannot format one, so the invariant is stated
/// here instead of at the macro.
const fn row_index(symbol: &str) -> u8 {
    let mut i = 0;
    while i < UNITS.len() {
        if str_eq(UNITS[i].symbol, symbol) {
            return i as u8;
        }
        i += 1;
    }
    unreachable!()
}

/// A length unit — a typed view of a [`UnitDef`] row, so `25.0 * MM`
/// can only build a [`crate::Length`] (never an angle), and the row it
/// views is a row of [`UNITS`] **by construction**: the value is that
/// row's index and nothing else.
///
/// **SEALED (issue #669) — the narrative lives on [`UnitDef`].** Both
/// halves of the pair are typed: the DIMENSION by which view this is,
/// the symbol/factor pairing by the index. Obtain one from a unit
/// constant ([`MM`], [`CM`], [`M`], [`IN`]) or from
/// [`UnitDef::as_length`].
///
/// The seal has two halves with different mechanisms, and it is worth
/// saying which pins which. **Outside the crate** the field is private,
/// so no value can be built at all — pinned by the `compile_fail` rows
/// below. **Inside the crate** the type lives behind a private module,
/// so the only mints are `of_row` and the two `UnitDef::as_*` doors,
/// and `of_row`'s two refusals are const-evaluated: a symbol [`UNITS`]
/// does not have, or a row of the wrong quantity, fails the BUILD at
/// the constant that names it. Nothing here rests on a convention that
/// this file happens to keep.
///
/// **Eight public functions read a number in a unit or write one, and
/// print its symbol, without checking either** — which is why the
/// pairing has to be typed rather than validated. Four take a
/// `LengthUnit`: [`crate::fmt_length`], [`crate::Length::in_unit`], and
/// the two `Mul` impls (`f64 * LengthUnit` and `LengthUnit * f64`, the
/// two spellings of the unit's one door); [`AngleUnit`] mirrors all
/// four. The `Mul` impls ARE the
/// D6 typed-units boundary, and `fmt.rs`'s `parse(fmt(x, unit))` pin
/// makes the formatter's suffix parser input, so a symbol paired with a
/// foreign factor would be a wrong VALUE reaching an `Expr` and a
/// document — not a display string. Issue #669 has the executed
/// reproductions.
///
/// The seal is a claim about what COMPILES, so it is pinned by rows
/// that must fail to compile — and each pins a PRIVACY refusal, so that
/// opening the field reddens it: `E0423` (*cannot initialize a tuple
/// struct which contains private fields*) for the mint, `E0616`
/// (*field `0` of struct `LengthUnit` is private*) for the read and
/// for the write. A row that named a field this type does not have
/// would fail forever, however open the seal became, and is worth
/// nothing.
///
/// The index cannot be supplied:
///
/// ```compile_fail,E0423
/// let bogus = quantity::LengthUnit(0);
/// ```
///
/// Nor read off a real constant and re-used:
///
/// ```compile_fail,E0616
/// let stolen = quantity::MM.0;
/// ```
///
/// Nor written through:
///
/// ```compile_fail,E0616
/// let mut mm = quantity::MM;
/// mm.0 = 4;
/// ```
///
/// The legal twin of all three: the type is exported, both readers
/// exist, and the twin differs from each block above in exactly one
/// respect — it never names the field. The twin is what makes the
/// codes honest: stable rustdoc checks only that a `compile_fail`
/// block fails, never that it fails with the code named beside it.
/// The three codes were read off `rustc` directly on each snippet at
/// the pinned toolchain (1.97.0).
///
/// ```
/// let mm: quantity::LengthUnit = quantity::MM;
/// assert_eq!(mm.symbol(), "mm");
/// assert_eq!(mm.factor(), quantity::MILLI);
/// assert_eq!((25.0 * mm).meters(), 0.025);
///
/// // And the row → view door, which is the other way in.
/// let row = quantity::unit_by_symbol("mm").expect("mm is a table row");
/// assert_eq!(row.as_length(), Some(quantity::MM));
/// assert_eq!(row.as_angle(), None);
/// ```
pub use view::LengthUnit;

/// An angle unit — the typed view for [`crate::Angle`] construction, an
/// index into [`UNITS`].
///
/// **SEALED for the reason and by the mechanism on [`LengthUnit`]**;
/// obtain one from [`DEG`], [`RAD`], or [`UnitDef::as_angle`]. The same
/// three privacy refusals pin it — `E0423`, `E0616`, `E0616` — so the
/// two types stay symmetric, and the twin below carries the same
/// caveat about the codes.
///
/// ```compile_fail,E0423
/// let bogus = quantity::AngleUnit(4);
/// ```
///
/// ```compile_fail,E0616
/// let stolen = quantity::DEG.0;
/// ```
///
/// ```compile_fail,E0616
/// let mut deg = quantity::DEG;
/// deg.0 = 5;
/// ```
///
/// Their legal twin:
///
/// ```
/// let deg: quantity::AngleUnit = quantity::DEG;
/// assert_eq!(deg.symbol(), "deg");
/// assert!(((90.0 * deg).radians() - core::f64::consts::FRAC_PI_2).abs() < 1e-15);
/// let row = quantity::unit_by_symbol("deg").expect("deg is a table row");
/// assert_eq!(row.as_angle(), Some(quantity::DEG));
/// assert_eq!(row.as_length(), None);
/// ```
pub use view::AngleUnit;

/// The dimensionless "unit" — a typed view of the one `Scalar` row,
/// sealed by the mechanism on [`LengthUnit`].
///
/// It carries no arithmetic and no formatter, because there is nothing
/// to convert: its factor is exactly 1.0 and its symbol is empty. What
/// it exists for is to be NAMEABLE, so that a stored literal always has
/// a notation and no reader has to invent one.
///
/// ```
/// let one: quantity::ScalarUnit = quantity::ONE;
/// assert_eq!(one.symbol(), "");
/// assert_eq!(one.factor(), 1.0);
/// let row = quantity::unit_by_symbol("").expect("the dimensionless row");
/// assert_eq!(row.as_scalar(), Some(quantity::ONE));
/// assert_eq!(row.as_length(), None);
/// assert_eq!(row.as_angle(), None);
/// ```
pub use view::ScalarUnit;

/// The typed views live behind a private module boundary, so the mints
/// below are the only code ANYWHERE — inside this file included — that
/// can build one. That is what makes "a `LengthUnit` is a `Length` row
/// of [`UNITS`]" a compiler-enforced invariant rather than a convention
/// this file keeps: a hand-written `LengthUnit(4)` does not compile
/// here either.
mod view {
    use super::{UNITS, UnitDef, UnitQuantity, row_index};

    /// See [`super::LengthUnit`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LengthUnit(u8);

    /// See [`super::AngleUnit`].
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AngleUnit(u8);

    /// See [`super::ScalarUnit`].
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct ScalarUnit(u8);

    impl LengthUnit {
        /// The length view of the [`UNITS`] row named `symbol` — the
        /// ONE mint. Both refusals are const-evaluated at every
        /// caller: a symbol the table does not have and a row of the
        /// wrong quantity are compile errors, so neither a seventh
        /// unit nor a mislabelled one can be written.
        pub(super) const fn of_row(symbol: &str) -> Self {
            let i = row_index(symbol);
            assert!(
                matches!(UNITS[i as usize].quantity(), UnitQuantity::Length),
                "a LengthUnit can only view a Length row of UNITS"
            );
            Self(i)
        }

        /// The surface symbol (`"mm"`), as displayed and as parsed back.
        pub const fn symbol(self) -> &'static str {
            self.def().symbol()
        }

        /// Meters per one of this unit.
        pub const fn factor(self) -> f64 {
            self.def().factor()
        }

        /// The table row this view indexes — the inverse of
        /// [`UnitDef::as_length`], and total, because the view IS an
        /// index. Public: it can only ever answer a row of [`UNITS`],
        /// which is what the seal is about.
        pub const fn def(self) -> UnitDef {
            UNITS[self.0 as usize]
        }
    }

    impl ScalarUnit {
        /// The dimensionless mirror of [`LengthUnit::of_row`].
        pub(super) const fn of_row(symbol: &str) -> Self {
            let i = row_index(symbol);
            assert!(
                matches!(UNITS[i as usize].quantity(), UnitQuantity::Scalar),
                "a ScalarUnit can only view the Scalar row of UNITS"
            );
            Self(i)
        }

        /// The surface symbol — the EMPTY string, which is how a
        /// dimensionless number is written.
        pub const fn symbol(self) -> &'static str {
            self.def().symbol()
        }

        /// Exactly 1.0: there is nothing to convert.
        pub const fn factor(self) -> f64 {
            self.def().factor()
        }

        /// The table row this view indexes — see [`LengthUnit::def`].
        pub const fn def(self) -> UnitDef {
            UNITS[self.0 as usize]
        }
    }

    impl AngleUnit {
        /// The angle mirror of [`LengthUnit::of_row`].
        pub(super) const fn of_row(symbol: &str) -> Self {
            let i = row_index(symbol);
            assert!(
                matches!(UNITS[i as usize].quantity(), UnitQuantity::Angle),
                "an AngleUnit can only view an Angle row of UNITS"
            );
            Self(i)
        }

        /// The surface symbol (`"deg"`), as displayed and as parsed back.
        pub const fn symbol(self) -> &'static str {
            self.def().symbol()
        }

        /// Radians per one of this unit.
        pub const fn factor(self) -> f64 {
            self.def().factor()
        }

        /// The table row this view indexes — see [`LengthUnit::def`].
        pub const fn def(self) -> UnitDef {
            UNITS[self.0 as usize]
        }
    }
}

/// A unit is a table row, and a row is a KEY: hashing one hashes its
/// symbol.
///
/// The symbol DETERMINES the row (the seal on [`UnitDef`]), so the
/// symbol is exactly as fine a partition as the derived [`PartialEq`]
/// above it: two values that compare equal are the same row and carry
/// the same symbol, so they hash alike, and two that hash alike are
/// the same row. That is why the hash can be written by hand over one
/// field while the comparison stays derived over three.
///
/// [`UnitDef`]'s [`Eq`] is by hand for the same reason. The derived
/// comparison reads an `f64` factor, so it is not derivable — but
/// every value of the type is a row of [`UNITS`], each factor a
/// finite literal, so the relation is reflexive on every value that
/// can exist and the equivalence [`Eq`] promises holds.
impl core::hash::Hash for UnitDef {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(&self.symbol, state);
    }
}

impl Eq for UnitDef {}

/// See [`UnitDef`]'s hash: the view indexes a row, and the row's
/// symbol is what a hash of it reads.
impl core::hash::Hash for LengthUnit {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(self.symbol(), state);
    }
}

/// See [`UnitDef`]'s hash.
impl core::hash::Hash for AngleUnit {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        core::hash::Hash::hash(self.symbol(), state);
    }
}

/// Millimeter: a `MILLI`-prefixed metre.
pub const MM: LengthUnit = LengthUnit::of_row("mm");

/// Centimeter: a `CENTI`-prefixed metre.
pub const CM: LengthUnit = LengthUnit::of_row("cm");

/// Meter — the canonical length unit, factor exactly 1.0.
pub const M: LengthUnit = LengthUnit::of_row("m");

/// International inch: EXACTLY 25.4 mm (module docs).
pub const IN: LengthUnit = LengthUnit::of_row("in");

/// Degree: π/180 radians — inexact by nature (module docs).
pub const DEG: AngleUnit = AngleUnit::of_row("deg");

/// Radian — the canonical angle unit, factor exactly 1.0.
pub const RAD: AngleUnit = AngleUnit::of_row("rad");

/// Half-turn: π radians, so `0.5 * PI` is a right angle and `2.0 * PI`
/// a full turn. The notation-as-a-unit row, spelled `pi rad` (module
/// docs).
pub const PI: AngleUnit = AngleUnit::of_row("pi rad");

/// The dimensionless "unit": the empty symbol, factor exactly 1.0.
///
/// It is how a `Scalar` literal names its notation, and the notation it
/// names is "written with no suffix" — see [`UnitQuantity::Scalar`] for
/// why the absence is a row rather than a `None`.
pub const ONE: ScalarUnit = ScalarUnit::of_row("");

/// The table row for a surface symbol, or `None` when the symbol is
/// not one the table carries.
pub fn unit_by_symbol(symbol: &str) -> Option<UnitDef> {
    UNITS.iter().find(|u| u.symbol == symbol).copied()
}

//! A number told to a person: the one type a sentence quotes a scalar
//! through (the crate README, "A number told to a person").
//!
//! A [`Told`] is built where the payload is built, while the scalar is
//! in hand, and from then on it only renders. It holds the *reading*
//! (a value, an enclosure, or unreadable), the [`Dimension`] and the
//! role, and its `Display` writes the number and the unit word. It has
//! no arithmetic and no ordering, and a computed number does not come
//! back out of it: its `Debug` writes what it tells, and two are equal
//! when they tell the same thing in the same role.
//!
//! # What is spelled
//!
//! Every spelling is the float a decimal of some number of significant
//! digits reads back as, written the way [`Readable`] writes that
//! float. The role picks the digits:
//!
//! - **given**: the float itself, so the shortest digits that read back;
//! - **computed**: the float rounded to the *grid*, trailing zeros
//!   dropped;
//! - **offered**: the float rounded on the grid toward the side that
//!   works when typed, and never past a far edge the builder states;
//! - **compared** ([`Told::beside`]): both numbers at the first
//!   precision, from the grid up, at which they read in their true
//!   order.
//!
//! The grid at `x` is four significant figures, and for a length the
//! finer of that and a tenth of [`DEFAULT_EPS`], held no finer than the
//! gap between `x` and the next float: a digit below that gap spells
//! the float's own rounding, not the length. It has no absolute floor,
//! so a nonzero number never reads `0`. The cap is the compiled default
//! and not the run's tolerance: one payload reads the same at every
//! tolerance a run is configured with.
//!
//! "On a side" and "contains" are stated of the float a spelling reads
//! back as, compared exactly with the float held: an offered `at least`
//! spelling parses to a float not below the bound, and an enclosure's
//! spelled ends parse to floats not inside its ends.
//!
//! ```compile_fail,E0369
//! use geom_core::{Dimension, Told};
//! let a = Told::computed(&1.0, Dimension::Length);
//! let b = Told::computed(&2.0, Dimension::Length);
//! // No ordering, and no number to order.
//! let _less = a < b;
//! ```
//!
//! ```compile_fail,E0369
//! use geom_core::{Dimension, Told};
//! let a = Told::computed(&1.0, Dimension::Length);
//! let b = Told::computed(&2.0, Dimension::Length);
//! // No arithmetic.
//! let _sum = a + b;
//! ```
//!
//! ```compile_fail,E0277
//! use geom_core::{Dimension, Told};
//! let a = Told::computed(&1.0, Dimension::Length);
//! let _x: f64 = a.into();
//! ```

use core::cmp::Ordering;
use core::fmt;

use crate::tolerance::DEFAULT_EPS;
use crate::{Readable, Real, fold_signed_zero};

/// The most significant digits a spelling carries: a decimal of this
/// many names one `f64`.
const ROUND_TRIP_DIGITS: usize = 17;

/// The significant figures of the grid where no length governs.
const GRID_FIGURES: i32 = 4;

/// The decade of a length's absolute grid step, a tenth of
/// [`DEFAULT_EPS`]: `10^LENGTH_GRID_DECADE` metres.
const LENGTH_GRID_DECADE: i32 = -10;

/// What a scalar reads as: its value, its enclosure, or nothing.
///
/// Minted by [`Real::told_reading`], once per scalar type, and read
/// only by [`Told`]: it has no accessor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Reading(Read);

#[derive(Clone, Copy, PartialEq)]
enum Read {
    /// One number; never NaN.
    Value(f64),
    /// An enclosure with `lo < hi`; neither end NaN.
    Enclosure { lo: f64, hi: f64 },
    /// Nothing was measured: NaN, or an ill-formed enclosure.
    Unreadable,
}

/// The shape of the reading, without its numbers.
impl fmt::Debug for Read {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Value(_) => "Value(..)",
            Self::Enclosure { .. } => "Enclosure(..)",
            Self::Unreadable => "Unreadable",
        })
    }
}

impl Reading {
    /// A scalar that is one number. NaN is unreadable.
    #[must_use]
    pub fn value(x: f64) -> Self {
        Self(if x.is_nan() {
            Read::Unreadable
        } else {
            Read::Value(x)
        })
    }

    /// A scalar that is an enclosure with these ends: one number where
    /// they coincide, unreadable where either is NaN or they are out of
    /// order.
    #[must_use]
    pub fn enclosure(lo: f64, hi: f64) -> Self {
        if lo == hi {
            Self::value(lo)
        } else if lo < hi {
            Self(Read::Enclosure { lo, hi })
        } else {
            Self(Read::Unreadable)
        }
    }
}

/// What a told number measures, which fixes its unit word and whether
/// the tolerance (a length) caps its grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dimension {
    /// Metres.
    Length,
    /// Radians.
    Angle,
    /// A ratio, a count held as a scalar, a component of a direction.
    Dimensionless,
}

impl Dimension {
    /// The unit as a sentence writes it after the number, with its
    /// leading space; empty for a dimensionless number.
    const fn unit_word(self) -> &'static str {
        match self {
            Self::Length => " m",
            Self::Angle => " rad",
            Self::Dimensionless => "",
        }
    }
}

/// The side of its bound an offered number is spelled on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Side {
    /// Not below the bound.
    AtLeast,
    /// Not above the bound.
    AtMost,
}

impl Side {
    fn holds(self, spelled: f64, bound: f64) -> bool {
        match self {
            Self::AtLeast => spelled >= bound,
            Self::AtMost => spelled <= bound,
        }
    }
}

/// What is held, by role.
#[derive(Clone, Copy)]
enum Held {
    Given(Read),
    Computed(Read),
    /// A bound to type back: the one finite number past which every
    /// value works (`None` where the scalar gave none), its side, and
    /// the far edge of the values that work where the builder stated
    /// one.
    Offered {
        bound: Option<f64>,
        side: Side,
        edge: Option<f64>,
    },
}

/// What is spelled: one float, or the two ends of an enclosure.
#[derive(Clone, Copy, PartialEq)]
enum Spelled {
    One(f64),
    Ends(f64, f64),
}

impl Spelled {
    /// The extent as `(least, greatest)`, by distance from zero where
    /// `magnitudes`.
    fn extent(self, magnitudes: bool) -> (f64, f64) {
        let (lo, hi) = match self {
            Self::One(y) => (y, y),
            Self::Ends(lo, hi) => (lo, hi),
        };
        if !magnitudes {
            (lo, hi)
        } else if lo <= 0.0 && hi >= 0.0 {
            (0.0, lo.abs().max(hi.abs()))
        } else {
            (lo.abs().min(hi.abs()), lo.abs().max(hi.abs()))
        }
    }

    /// How this stands to `other`: before it, after it, or the same
    /// single number. `None` where they overlap without being one
    /// number, which has no order.
    fn order(self, other: Self, magnitudes: bool) -> Option<Ordering> {
        let ((a_lo, a_hi), (b_lo, b_hi)) = (self.extent(magnitudes), other.extent(magnitudes));
        if a_hi < b_lo {
            Some(Ordering::Less)
        } else if a_lo > b_hi {
            Some(Ordering::Greater)
        } else if a_lo == a_hi && b_lo == b_hi {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

/// What a [`Told`] holds.
#[derive(Clone, Copy)]
struct Quantity {
    held: Held,
    dimension: Dimension,
}

impl Quantity {
    fn offered(reading: Reading, dimension: Dimension, side: Side, edge: Option<f64>) -> Self {
        // An enclosure offers the end past which every value works, and
        // a bound that is not a finite number is no offer.
        let bound = match (reading.0, side) {
            (Read::Value(x), _) => Some(x),
            (Read::Enclosure { hi, .. }, Side::AtLeast) => Some(hi),
            (Read::Enclosure { lo, .. }, Side::AtMost) => Some(lo),
            (Read::Unreadable, _) => None,
        }
        .filter(|bound| bound.is_finite());
        // An edge on the wrong side of the bound states no window.
        let edge = edge.filter(|&edge| bound.is_some_and(|bound| side.holds(edge, bound)));
        Self {
            held: Held::Offered { bound, side, edge },
            dimension,
        }
    }

    /// What is held, as the floats it was built from.
    fn exact(&self) -> Option<Spelled> {
        match self.held {
            Held::Given(read) | Held::Computed(read) => match read {
                Read::Value(x) => Some(Spelled::One(x)),
                Read::Enclosure { lo, hi } => Some(Spelled::Ends(lo, hi)),
                Read::Unreadable => None,
            },
            Held::Offered { bound, .. } => bound.map(Spelled::One),
        }
    }

    /// What is spelled at `extra` digits past the grid; `None` for a
    /// reading that does not read.
    fn spelled(&self, extra: usize) -> Option<Spelled> {
        let digits = |x: f64| grid_digits(x, self.dimension) + extra;
        match self.held {
            Held::Given(_) => self.exact(),
            Held::Computed(Read::Value(x)) => Some(Spelled::One(nearest(x, digits(x)))),
            Held::Computed(Read::Enclosure { lo, hi }) => Some(Spelled::Ends(
                toward(lo, digits(lo), Side::AtMost),
                toward(hi, digits(hi), Side::AtLeast),
            )),
            Held::Computed(Read::Unreadable) => None,
            Held::Offered { bound, side, edge } => bound.map(|x| {
                Spelled::One(
                    (digits(x)..ROUND_TRIP_DIGITS)
                        .map(|digits| toward(x, digits, side))
                        .find(|&y| edge.is_none_or(|edge| !side.holds(y, edge) || y == edge))
                        .unwrap_or(x),
                )
            }),
        }
    }

    fn write_number(&self, f: &mut fmt::Formatter<'_>, extra: usize) -> fmt::Result {
        match self.spelled(extra) {
            Some(Spelled::One(y)) => write_float(y, f),
            Some(Spelled::Ends(lo, hi)) => {
                f.write_str("[")?;
                write_float(lo, f)?;
                f.write_str(", ")?;
                write_float(hi, f)?;
                f.write_str("]")
            }
            None => f.write_str("NaN"),
        }
    }

    fn write(&self, f: &mut fmt::Formatter<'_>, extra: usize) -> fmt::Result {
        self.write_number(f, extra)?;
        f.write_str(self.dimension.unit_word())
    }

    /// The role as a word.
    const fn role(&self) -> &'static str {
        match self.held {
            Held::Given(_) => "given",
            Held::Computed(_) => "computed",
            Held::Offered {
                side: Side::AtLeast,
                ..
            } => "at least",
            Held::Offered {
                side: Side::AtMost, ..
            } => "at most",
        }
    }
}

/// The role and what is told, never the float held.
impl fmt::Debug for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \"", self.role())?;
        self.write(f, 0)?;
        f.write_str("\"")
    }
}

/// Two are equal when they tell the same thing in the same role: the
/// same spelling and unit. Two computed numbers that round to one
/// spelling are equal.
impl PartialEq for Quantity {
    fn eq(&self, other: &Self) -> bool {
        let bits = |spelled: Option<Spelled>| {
            spelled.map(|spelled| {
                let (lo, hi) = spelled.extent(false);
                (
                    matches!(spelled, Spelled::One(_)),
                    fold_signed_zero(lo).to_bits(),
                    fold_signed_zero(hi).to_bits(),
                )
            })
        };
        self.dimension == other.dimension
            && self.role() == other.role()
            && bits(self.spelled(0)) == bits(other.spelled(0))
    }
}

/// **A number as a sentence tells it** (the module docs).
///
/// One pointer wide, so a payload that holds it is no larger than one
/// that held the scalar: a refusal's size is what every `Result` that
/// carries it pays.
#[derive(Clone, Debug, PartialEq)]
pub struct Told(Box<Quantity>);

impl Told {
    /// The caller's or the file's own number, told exactly as received.
    #[must_use]
    pub fn given<T: Real>(x: &T, dimension: Dimension) -> Self {
        Self::given_reading(x.told_reading(), dimension)
    }

    /// A number the kernel derived, told on the grid.
    #[must_use]
    pub fn computed<T: Real>(x: &T, dimension: Dimension) -> Self {
        Self::computed_reading(x.told_reading(), dimension)
    }

    /// A bound the person is asked to type back, where every value not
    /// below it works: spelled on the grid, not below the bound. An
    /// enclosure offers its upper end, and a bound that is not a finite
    /// number is unreadable.
    #[must_use]
    pub fn at_least<T: Real>(bound: &T, dimension: Dimension) -> Self {
        Self::quantity(Quantity::offered(
            bound.told_reading(),
            dimension,
            Side::AtLeast,
            None,
        ))
    }

    /// A bound the person is asked to type back, where every value not
    /// above it works: spelled on the grid, not above the bound. An
    /// enclosure offers its lower end, and a bound that is not a finite
    /// number is unreadable.
    #[must_use]
    pub fn at_most<T: Real>(bound: &T, dimension: Dimension) -> Self {
        Self::at_most_reading(bound.told_reading(), dimension, None)
    }

    /// [`Told::at_least`], where the values that work are a window and
    /// not a half-line: spelled no farther above the bound than `edge`.
    /// An `edge` below the bound, or one that does not read, states no
    /// window (an enclosure's lower end is the edge).
    #[must_use]
    pub fn at_least_within<T: Real>(bound: &T, edge: &T, dimension: Dimension) -> Self {
        let edge = match edge.told_reading().0 {
            Read::Value(x) | Read::Enclosure { lo: x, .. } => Some(x),
            Read::Unreadable => None,
        };
        Self::quantity(Quantity::offered(
            bound.told_reading(),
            dimension,
            Side::AtLeast,
            edge,
        ))
    }

    /// [`Told::at_most`], where the values that work are a window and
    /// not a half-line: spelled no farther below the bound than `edge`.
    /// An `edge` above the bound, or one that does not read, states no
    /// window (an enclosure's upper end is the edge).
    #[must_use]
    pub fn at_most_within<T: Real>(bound: &T, edge: &T, dimension: Dimension) -> Self {
        let edge = match edge.told_reading().0 {
            Read::Value(x) | Read::Enclosure { hi: x, .. } => Some(x),
            Read::Unreadable => None,
        };
        Self::at_most_reading(bound.told_reading(), dimension, edge)
    }

    /// The number a **given** one was received as: the caller's own
    /// number back. `None` for a computed or offered number, and for a
    /// given one that is not a single value.
    #[must_use]
    pub fn as_given(&self) -> Option<f64> {
        match self.0.held {
            Held::Given(Read::Value(x)) => Some(x),
            _ => None,
        }
    }

    /// **A position in a plane's chart**, `(x, y)`: each coordinate a
    /// length told as this one is, the pair written without a unit
    /// word, as a sentence that locates something writes it.
    #[must_use]
    pub fn position(x: &Self, y: &Self) -> impl fmt::Display {
        let (x, y) = (*x.0, *y.0);
        fmt::from_fn(move |f| {
            f.write_str("(")?;
            x.write_number(f, 0)?;
            f.write_str(", ")?;
            y.write_number(f, 0)?;
            f.write_str(")")
        })
    }

    /// The pair `(self, other)` as a sentence that states their order
    /// tells them.
    #[must_use]
    pub fn beside(&self, other: &Self) -> Compared {
        Compared::new(*self.0, *other.0, false)
    }

    /// [`Told::beside`], for a sentence that states the order of the
    /// two **sizes** (a signed lever "shorter than" a length): each is
    /// told with its sign, at the first precision at which their
    /// distances from zero read in their true order.
    #[must_use]
    pub fn beside_by_magnitude(&self, other: &Self) -> Compared {
        Compared::new(*self.0, *other.0, true)
    }

    fn quantity(quantity: Quantity) -> Self {
        Self(Box::new(quantity))
    }

    /// A given number from a reading already taken.
    pub(crate) fn given_reading(reading: Reading, dimension: Dimension) -> Self {
        Self::quantity(Quantity {
            held: Held::Given(reading.0),
            dimension,
        })
    }

    /// A computed number from a reading already taken.
    pub(crate) fn computed_reading(reading: Reading, dimension: Dimension) -> Self {
        Self::quantity(Quantity {
            held: Held::Computed(reading.0),
            dimension,
        })
    }

    /// An offered upper bound from a reading already taken, and its far
    /// edge where the values that work are a window.
    pub(crate) fn at_most_reading(
        reading: Reading,
        dimension: Dimension,
        edge: Option<f64>,
    ) -> Self {
        Self::quantity(Quantity::offered(reading, dimension, Side::AtMost, edge))
    }

    /// The float this number is spelled as, where it is one number.
    pub(crate) fn spelled(&self) -> Option<f64> {
        match self.0.spelled(0)? {
            Spelled::One(y) => Some(y),
            Spelled::Ends(..) => None,
        }
    }

    /// The number without its unit word.
    pub(crate) fn number(&self) -> impl fmt::Display {
        let quantity = *self.0;
        fmt::from_fn(move |f| quantity.write_number(f, 0))
    }
}

/// The numbers behind a told quantity, for a suite that pins a payload
/// against the scalar its door computed. No production build has it.
#[cfg(any(test, feature = "test-support"))]
impl Told {
    /// The reading's ends: a value twice, an enclosure's two, NaN twice
    /// where it does not read.
    #[must_use]
    pub fn pinned_ends(&self) -> (f64, f64) {
        self.0
            .exact()
            .map_or((f64::NAN, f64::NAN), |held| held.extent(false))
    }

    /// The far edge an offered bound was built with
    /// ([`Told::at_least_within`], [`Told::at_most_within`]).
    #[must_use]
    pub fn pinned_edge(&self) -> Option<f64> {
        match self.0.held {
            Held::Offered { edge, .. } => edge,
            Held::Given(_) | Held::Computed(_) => None,
        }
    }
}

/// The number and its unit word.
impl fmt::Display for Told {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.write(f, 0)
    }
}

/// **Two numbers whose order the sentence states**, or a reading beside
/// its own threshold ([`Told::beside`]): each is told at the first
/// precision, from its own grid up, at which the two read in their true
/// order, in every role. Two numbers that differ never read alike, and
/// two enclosures that do not meet read apart and in order.
///
/// A pair with no order is told as each member is alone, and the form
/// claims nothing of it: two enclosures that overlap, an enclosure that
/// holds the number beside it, or a member that does not read.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Compared {
    first: Quantity,
    second: Quantity,
    /// Digits past each member's grid.
    extra: usize,
}

impl Compared {
    fn new(first: Quantity, second: Quantity, magnitudes: bool) -> Self {
        let order = |a: Option<Spelled>, b: Option<Spelled>| a?.order(b?, magnitudes);
        let extra = order(first.exact(), second.exact())
            .and_then(|held| {
                (0..=ROUND_TRIP_DIGITS)
                    .find(|&extra| order(first.spelled(extra), second.spelled(extra)) == Some(held))
            })
            .unwrap_or(0);
        Self {
            first,
            second,
            extra,
        }
    }

    /// The first number and its unit word.
    #[must_use]
    pub fn first(&self) -> impl fmt::Display {
        let (told, extra) = (self.first, self.extra);
        fmt::from_fn(move |f| told.write(f, extra))
    }

    /// The second number and its unit word.
    #[must_use]
    pub fn second(&self) -> impl fmt::Display {
        let (told, extra) = (self.second, self.extra);
        fmt::from_fn(move |f| told.write(f, extra))
    }
}

/// One float as a sentence spells it: [`Readable`]'s notation, a zero
/// of either sign as `0`.
fn write_float(y: f64, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt::Display::fmt(&Readable(fold_signed_zero(y)), f)
}

/// The decimal exponent of a finite nonzero `x`.
fn decade(x: f64) -> i32 {
    let spelled = format!("{x:e}");
    let Some(exponent) = spelled.rsplit_once('e').and_then(|(_, e)| e.parse().ok()) else {
        unreachable!("the exponent form of a finite float, {spelled}, ends in a decimal exponent")
    };
    exponent
}

/// The decade of a length's grid step at `x`: a tenth of the default
/// tolerance, or where the floats at `x` are sparser than that, the
/// first power of ten at or above the gap to the next float.
fn length_step_decade(x: f64) -> i32 {
    let gap = x.abs().next_up() - x.abs();
    if !gap.is_finite() {
        // The largest floats: no decimal step is finer than the format.
        return decade(x);
    }
    // A gap is a power of two, so it is a power of ten only at one.
    let carried = decade(gap) + i32::from(gap != 1.0);
    LENGTH_GRID_DECADE.max(carried)
}

/// The significant digits of the grid at `x` (the module docs).
fn grid_digits(x: f64, dimension: Dimension) -> usize {
    let figures = match dimension {
        Dimension::Length if x != 0.0 && x.is_finite() => {
            GRID_FIGURES.max(decade(x) - length_step_decade(x) + 1)
        }
        Dimension::Length | Dimension::Angle | Dimension::Dimensionless => GRID_FIGURES,
    };
    usize::try_from(figures)
        .unwrap_or(ROUND_TRIP_DIGITS)
        .min(ROUND_TRIP_DIGITS)
}

/// A decimal of `digits` significant digits: `±mantissa × 10^exponent`.
#[derive(Clone, Copy)]
struct Decimal {
    negative: bool,
    mantissa: u64,
    exponent: i32,
}

impl Decimal {
    /// `x` rounded to nearest at `digits` significant digits, exactly
    /// (std's formatter rounds the exact binary value).
    fn nearest(x: f64, digits: usize) -> Option<Self> {
        let places = digits.checked_sub(1)?;
        let spelled = format!("{:.places$e}", x.abs());
        let (mantissa, exponent) = spelled.split_once('e')?;
        let mantissa: String = mantissa.chars().filter(char::is_ascii_digit).collect();
        Some(Self {
            negative: x.is_sign_negative(),
            mantissa: mantissa.parse().ok()?,
            exponent: exponent.parse::<i32>().ok()? - i32::try_from(places).ok()?,
        })
    }

    /// The adjacent decimal of as many digits, away from zero or toward
    /// it: one unit of the last digit over, and below a power of ten
    /// one unit of the digit after it, where the decimals are ten times
    /// as dense.
    fn stepped(self, away_from_zero: bool) -> Option<Self> {
        if away_from_zero {
            let mantissa = self.mantissa.checked_add(1)?;
            return Some(Self { mantissa, ..self });
        }
        let leads_its_decade = (0..=19).any(|k| 10u64.checked_pow(k) == Some(self.mantissa));
        let (mantissa, exponent) = if leads_its_decade && self.mantissa > 1 {
            (
                self.mantissa.checked_mul(10)?,
                self.exponent.checked_sub(1)?,
            )
        } else {
            (self.mantissa, self.exponent)
        };
        Some(Self {
            mantissa: mantissa.checked_sub(1)?,
            exponent,
            ..self
        })
    }

    /// The float this decimal reads back as, where that is finite: a
    /// decimal past the largest float reads back as no number.
    fn read_back(self) -> Option<f64> {
        let sign = if self.negative { "-" } else { "" };
        format!("{sign}{}e{}", self.mantissa, self.exponent)
            .parse()
            .ok()
            .filter(|y: &f64| y.is_finite())
    }
}

/// `x` rounded to nearest at `digits` significant digits, read back.
/// A zero, a non-finite value and a count at which a decimal names one
/// float are `x` itself; a rounding past the largest float is the grid
/// point below it.
fn nearest(x: f64, digits: usize) -> f64 {
    if x == 0.0 || !x.is_finite() || digits >= ROUND_TRIP_DIGITS {
        return x;
    }
    Decimal::nearest(x, digits)
        .and_then(|near| {
            near.read_back()
                .or_else(|| near.stepped(false)?.read_back())
        })
        .unwrap_or(x)
}

/// `x` rounded at `digits` significant digits to the adjacent decimal on
/// `side`, read back: the nearest decimal where that already lies on the
/// side, one unit of its last digit over where it does not. Where
/// neither reads back on the side (digits finer than the floats at
/// `x`, or a step past the largest float), `x` itself, which is on both.
fn toward(x: f64, digits: usize, side: Side) -> f64 {
    if x == 0.0 || !x.is_finite() || digits >= ROUND_TRIP_DIGITS {
        return x;
    }
    let Some(near) = Decimal::nearest(x, digits) else {
        return x;
    };
    let away_from_zero = (side == Side::AtLeast) != near.negative;
    [Some(near), near.stepped(away_from_zero)]
        .into_iter()
        .flatten()
        .filter_map(Decimal::read_back)
        .find(|&y| side.holds(y, x))
        .unwrap_or(x)
}

// The grid's absolute step is a tenth of the default tolerance.
const _: () = assert!(DEFAULT_EPS == 1e-9 && LENGTH_GRID_DECADE == -10);

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Dimension, Told};
    use crate::{Dual, Interval, Readable, Real};

    const DIMENSIONS: [Dimension; 3] = [
        Dimension::Length,
        Dimension::Angle,
        Dimension::Dimensionless,
    ];

    /// Floats over the whole format, both signs: every `stride`th binade
    /// and decade, the binades at both ends of the format, and every
    /// decade where a length's grid changes arm; in each, mantissas that
    /// are round, that sit one float off round, and that fill every
    /// digit.
    fn magnitudes(stride: usize) -> Vec<f64> {
        let binades: Vec<f64> = core::iter::successors(Some(f64::from_bits(1)), |x| Some(x * 2.0))
            .take_while(|x| x.is_finite())
            .collect();
        let ends = (0..4).chain(binades.len() - 4..binades.len());
        let mut out = Vec::new();
        for k in (0..binades.len()).step_by(stride).chain(ends) {
            let x = binades[k];
            for v in [x, x.next_up(), x * 1.5, 3.0 * x / 7.0, x * 1.999_999_9] {
                if v.is_finite() && v != 0.0 {
                    out.extend([v, -v]);
                }
            }
        }
        for decade in (-300..=300).step_by(stride).chain(-12..=8) {
            let d: f64 = format!("1e{decade}").parse().unwrap();
            for m in [
                1.0,
                1.0004,
                1.2345,
                9.9996,
                9.999_949,
                0.1 + 0.2,
                8.000_000_000_000_002,
            ] {
                out.extend([m * d, -m * d, (m * d).next_down()]);
            }
        }
        out
    }

    /// The largest four-figure decimal that is a float.
    const LAST_GRID_POINT: f64 = 1.797e308;

    /// The number a spelling carries, without its unit word.
    fn number(told: &str) -> &str {
        told.split(' ').next().unwrap()
    }

    fn read_back(told: &str) -> f64 {
        number(told)
            .parse()
            .unwrap_or_else(|e| panic!("{told:?} does not read back: {e}"))
    }

    /// The significant digits a spelling writes.
    fn figures(told: &str) -> usize {
        let mantissa = number(told).split('e').next().unwrap();
        let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
        digits.trim_start_matches('0').trim_end_matches('0').len()
    }

    /// The step of a spelling's last digit that a grid of `figures`
    /// significant figures allows at `x`, as an upper bound a rounding
    /// may reach but not pass: one decade of slack over `x`'s own.
    fn step(x: f64, figures: i32) -> f64 {
        let unit: f64 = format!("1e{}", decade_of(x) - figures + 1).parse().unwrap();
        unit * (1.0 + 1e-12) + spacing(x)
    }

    /// The decimal exponent of `x`, read off its exponent form.
    fn decade_of(x: f64) -> i32 {
        let spelled = format!("{x:e}");
        spelled.rsplit_once('e').unwrap().1.parse().unwrap()
    }

    /// The gap between `x` and the next float away from zero: what
    /// reading a decimal back can add to its distance from `x`.
    fn spacing(x: f64) -> f64 {
        x.abs().next_up() - x.abs()
    }

    #[test]
    fn a_given_number_reads_back_as_itself_at_every_magnitude() {
        for x in magnitudes(32) {
            for dimension in DIMENSIONS {
                let said = Told::given(&x, dimension).to_string();
                assert_eq!(read_back(&said).to_bits(), x.to_bits(), "{x:e}: {said}");
                assert_eq!(number(&said), Readable(x).to_string(), "{x:e}");
            }
        }
    }

    /// The spacing limit is relative: no nonzero number reads `0`,
    /// subnormals included, and a spelling is never farther from its
    /// number than half a step of the fourth figure.
    #[test]
    fn a_computed_number_is_on_the_grid_and_never_reads_zero() {
        for x in magnitudes(64) {
            for dimension in DIMENSIONS {
                let said = Told::computed(&x, dimension).to_string();
                let y = read_back(&said);
                assert!(y != 0.0, "{x:e} reads {said}");
                assert_eq!(y.is_sign_negative(), x.is_sign_negative(), "{x:e}: {said}");
                // Past the format's last grid point the nearest one is
                // the one below, up to a whole step off.
                let reach = if x.abs() > LAST_GRID_POINT { 1.0 } else { 0.5 };
                assert!(
                    (y - x).abs() <= step(x, 4) * reach,
                    "{x:e} reads {said}, off the four-figure grid"
                );
                if dimension != Dimension::Length {
                    assert!(figures(&said) <= 4, "{x:e} reads {said}");
                }
            }
        }
        assert_eq!(
            Told::computed(&f64::MAX, Dimension::Angle).to_string(),
            "1.797e308 rad"
        );
    }

    /// A length's grid is the finer of four figures and a tenth of the
    /// default tolerance, so two lengths the kernel tells apart at a
    /// kilometre are told apart; an angle's is four figures at every
    /// size.
    #[test]
    fn the_tolerance_caps_a_lengths_grid_and_no_other() {
        let length = |x: f64| Told::computed(&x, Dimension::Length).to_string();
        assert_eq!(length(1_234.500_000_000_4), "1234.5000000004 m");
        assert_eq!(length(0.008_000_000_000_000_002), "0.008 m");
        assert_eq!(length(0.361_629_480_311_458), "0.3616294803 m");
        assert_eq!(length(1.234_567_89e-7), "1.235e-7 m");
        assert_eq!(length(3.3e-13), "3.3e-13 m");
        assert_eq!(length(f64::from_bits(1)), "5e-324 m");
        let angle = |x: f64| Told::computed(&x, Dimension::Angle).to_string();
        assert_eq!(angle(1_234.500_000_000_4), "1235 rad");
        assert_eq!(angle(core::f64::consts::PI), "3.142 rad");
        let ratio = |x: f64| Told::computed(&x, Dimension::Dimensionless).to_string();
        assert_eq!(ratio(0.1 + 0.2), "0.3");
        assert_eq!(ratio(1.0004), "1");
        for x in magnitudes(16) {
            let y = read_back(&length(x));
            let tenth_eps = 1e-10 * (1.0 + 1e-12);
            assert!(
                (y - x).abs() <= (tenth_eps / 2.0).max(5.0 * spacing(x)) + spacing(x),
                "{x:e} reads {}",
                length(x)
            );
        }
    }

    #[test]
    fn a_zero_of_either_sign_reads_zero_in_every_role() {
        for zero in [0.0, -0.0] {
            for dimension in DIMENSIONS {
                for told in [
                    Told::given(&zero, dimension),
                    Told::computed(&zero, dimension),
                    Told::at_least(&zero, dimension),
                    Told::at_most(&zero, dimension),
                ] {
                    assert_eq!(number(&told.to_string()), "0", "{told:?}");
                }
            }
        }
        let straddle = Interval::from_bounds(-0.0, 1.0);
        assert_eq!(
            Told::computed(&straddle, Dimension::Length).to_string(),
            "[0, 1] m"
        );
    }

    /// An offered bound typed back lies on its side of the bound, as
    /// floats compare, and within one step of the grid of it.
    #[test]
    fn an_offered_bound_typed_back_lies_on_its_side() {
        for x in magnitudes(64).into_iter().step_by(4) {
            for dimension in DIMENSIONS {
                // Four figures, and for a length as many as reach a tenth
                // of the tolerance.
                let figures = if dimension == Dimension::Length {
                    usize::try_from(decade_of(x) + 11).map_or(4, |n| n.clamp(4, 17))
                } else {
                    4
                };
                let up = Told::at_least(&x, dimension).to_string();
                let down = Told::at_most(&x, dimension).to_string();
                let (hi, lo) = (read_back(&up), read_back(&down));
                assert!(hi >= x, "at least {x:e} reads {up}");
                assert!(lo <= x, "at most {x:e} reads {down}");
                assert!(hi - x <= step(x, 4), "at least {x:e} reads {up}");
                assert!(x - lo <= step(x, 4), "at most {x:e} reads {down}");
                // Past the format's last grid point there is no grid
                // point on the far side, and the bound is its own.
                let past = |x: f64| x > LAST_GRID_POINT;
                assert!(past(x) || self::figures(&up) <= figures, "{x:e}: {up}");
                assert!(past(-x) || self::figures(&down) <= figures, "{x:e}: {down}");
            }
        }
        let at_most = |x: f64| Told::at_most(&x, Dimension::Length).to_string();
        assert_eq!(at_most(5e-10), "5e-10 m");
        assert_eq!(at_most(5.000_49e-10), "5e-10 m");
        assert_eq!(at_most(4.999_95e-10), "4.999e-10 m");
        // Below a power of ten the grid is ten times as dense.
        assert_eq!(at_most(9.999_95e-10), "9.999e-10 m");
        let at_least = |x: f64| Told::at_least(&x, Dimension::Length).to_string();
        assert_eq!(at_least(0.300_000_000_04), "0.3000000001 m");
        assert_eq!(at_least(0.3), "0.3 m");
    }

    /// A stated window holds the spelling at every width, down to the
    /// bound's own float.
    #[test]
    fn an_offered_bound_stays_inside_its_stated_window() {
        for x in magnitudes(128).into_iter().step_by(8) {
            for reach in [1e-3, 1e-9, 1e-13, 0.0] {
                let width = x.abs() * reach;
                for (told, lo, hi) in [
                    (
                        Told::at_least_within(&x, &(x + width), Dimension::Length),
                        x,
                        x + width,
                    ),
                    (
                        Told::at_most_within(&x, &(x - width), Dimension::Length),
                        x - width,
                        x,
                    ),
                ] {
                    let said = told.to_string();
                    let y = read_back(&said);
                    assert!(lo <= y && y <= hi, "{x:e} within {reach:e}: {said}");
                }
            }
        }
        // An edge on the wrong side states no window.
        let within = |edge: f64| Told::at_least_within(&1.000_04, &edge, Dimension::Angle);
        assert_eq!(within(0.5), Told::at_least(&1.000_04, Dimension::Angle));
        assert_eq!(within(0.5).to_string(), "1.001 rad");
        assert_eq!(within(1.000_05).to_string(), "1.00004 rad");
        assert_eq!(within(1.0005).to_string(), "1.0001 rad");
    }

    /// An enclosure's spelled ends read back as floats that contain its
    /// ends, each within one step of the grid, at every magnitude and
    /// width.
    #[test]
    fn an_enclosures_spelled_ends_contain_its_ends() {
        for x in magnitudes(128).into_iter().step_by(8) {
            for width in [f64::EPSILON, 1e-9, 1e-3, 1.0] {
                let (a, b) = (x, x + x.abs() * width);
                let (lo, hi) = (a.min(b), a.max(b));
                if lo == hi || !hi.is_finite() {
                    continue;
                }
                let enclosure = Interval::from_bounds(lo, hi);
                for dimension in DIMENSIONS {
                    let said = Told::computed(&enclosure, dimension).to_string();
                    let ends = said
                        .trim_end_matches(" m")
                        .trim_end_matches(" rad")
                        .strip_prefix('[')
                        .and_then(|s| s.strip_suffix(']'))
                        .and_then(|s| s.split_once(", "))
                        .unwrap_or_else(|| panic!("[{lo:e}, {hi:e}] reads {said}"));
                    let (told_lo, told_hi) = (read_back(ends.0), read_back(ends.1));
                    assert!(told_lo <= lo, "[{lo:e}, {hi:e}] reads {said}");
                    assert!(told_hi >= hi, "[{lo:e}, {hi:e}] reads {said}");
                    assert!(lo - told_lo <= step(lo, 4), "[{lo:e}, {hi:e}] reads {said}");
                    assert!(told_hi - hi <= step(hi, 4), "[{lo:e}, {hi:e}] reads {said}");
                }
            }
        }
        let third = Interval::from_bounds(1.0 / 3.0, 2.0 / 3.0);
        assert_eq!(
            Told::computed(&third, Dimension::Dimensionless).to_string(),
            "[0.3333, 0.6667]"
        );
        assert_eq!(
            Told::given(&third, Dimension::Dimensionless).to_string(),
            "[0.3333333333333333, 0.6666666666666666]"
        );
    }

    /// Two compared numbers read in their true order at every gap down
    /// to adjacent floats, and so never alike when they differ; two
    /// that are equal read alike.
    #[test]
    fn compared_numbers_read_in_their_true_order() {
        for x in magnitudes(128).into_iter().step_by(16) {
            let others = [x.next_up(), x * (1.0 - 1e-12), x * 1.0004, -x, x];
            for y in others.into_iter().filter(|y| y.is_finite()) {
                for dimension in DIMENSIONS {
                    let pair = Told::computed(&x, dimension).beside(&Told::computed(&y, dimension));
                    let (first, second) = (pair.first().to_string(), pair.second().to_string());
                    assert_eq!(
                        read_back(&first).partial_cmp(&read_back(&second)),
                        x.partial_cmp(&y),
                        "{x:e} beside {y:e} reads {first} and {second}"
                    );
                    assert_eq!(first == second, x == y, "{x:e} beside {y:e}: {first}");
                }
            }
        }
        let beside = |a: f64, b: f64| {
            let pair =
                Told::computed(&a, Dimension::Angle).beside(&Told::computed(&b, Dimension::Angle));
            (pair.first().to_string(), pair.second().to_string())
        };
        assert_eq!(beside(1.0, 2.0), ("1 rad".into(), "2 rad".into()));
        assert_eq!(
            beside(1.234_51, 1.234_52),
            ("1.23451 rad".into(), "1.23452 rad".into())
        );
        // A reading beside its own threshold, which is given.
        let pair = Told::computed(&1.000_000_4e-9, Dimension::Length)
            .beside(&Told::given(&1e-9, Dimension::Length));
        assert_eq!(pair.first().to_string(), "1.0000004e-9 m");
        assert_eq!(pair.second().to_string(), "1e-9 m");
    }

    /// A scalar that is not an `f64` reads as its value or its
    /// enclosure, never as a `Debug` struct.
    #[test]
    fn every_scalar_reads_as_a_value_or_an_enclosure() {
        let dual = Dual::variable(0.008_000_000_000_000_002_f64);
        assert_eq!(
            Told::computed(&dual, Dimension::Length).to_string(),
            "0.008 m"
        );
        let point = Interval::from_f64(0.25);
        assert_eq!(
            Told::computed(&point, Dimension::Length).to_string(),
            "0.25 m"
        );
        let dual_interval = Dual::variable(Interval::from_bounds(1.0, 2.0));
        assert_eq!(
            Told::computed(&dual_interval, Dimension::Angle).to_string(),
            "[1, 2] rad"
        );
        assert_eq!(
            Told::at_least(&dual_interval, Dimension::Length).to_string(),
            "2 m"
        );
        assert_eq!(
            Told::at_most(&dual_interval, Dimension::Length).to_string(),
            "1 m"
        );
        for told in [
            Told::computed(&f64::NAN, Dimension::Length),
            Told::given(&Interval::from_f64(f64::NAN), Dimension::Length),
            Told::at_most(&f64::NAN, Dimension::Length),
        ] {
            assert_eq!(told.to_string(), "NaN m");
        }
        assert_eq!(
            Told::given(&f64::INFINITY, Dimension::Dimensionless).to_string(),
            "inf"
        );
        assert_eq!(
            Told::computed(&f64::NEG_INFINITY, Dimension::Length).to_string(),
            "-inf m"
        );
    }

    /// Every pair `beside` accepts reads in its true order, whatever
    /// role each member has: an offered member is spelled finer on its
    /// own side until the two separate.
    #[test]
    fn compared_numbers_read_in_their_true_order_in_every_role() {
        type Mk = fn(&f64, Dimension) -> Told;
        let roles: [Mk; 4] = [Told::given, Told::computed, Told::at_least, Told::at_most];
        for x in magnitudes(128).into_iter().step_by(48) {
            for y in [x.next_up(), x * 1.000_04, -x, 0.0] {
                if !y.is_finite() || y == x {
                    continue;
                }
                for dimension in [Dimension::Length, Dimension::Angle] {
                    for a in roles {
                        for b in roles {
                            let pair = a(&x, dimension).beside(&b(&y, dimension));
                            let (first, second) =
                                (pair.first().to_string(), pair.second().to_string());
                            assert_eq!(
                                read_back(&first).partial_cmp(&read_back(&second)),
                                x.partial_cmp(&y),
                                "{x:e} beside {y:e} reads {first} and {second}"
                            );
                        }
                    }
                }
            }
        }
        let pair = Told::at_least(&1.000_01, Dimension::Dimensionless)
            .beside(&Told::computed(&1.0004, Dimension::Dimensionless));
        assert_eq!(
            (pair.first().to_string(), pair.second().to_string()),
            ("1.0001".into(), "1.0004".into())
        );
        let pair = Told::computed(&1.000_01, Dimension::Angle)
            .beside(&Told::at_most(&1.000_015, Dimension::Angle));
        assert_eq!(
            (pair.first().to_string(), pair.second().to_string()),
            ("1.00001 rad".into(), "1.000015 rad".into())
        );
    }

    /// Two enclosures that do not meet read apart and in order, as does
    /// an enclosure beside a number outside it; two that overlap are
    /// told as each is alone.
    #[test]
    fn compared_enclosures_read_apart_where_they_are_apart() {
        let ends = |told: String| {
            let (lo, hi) = told
                .trim_end_matches(" m")
                .trim_matches(['[', ']'])
                .split_once(", ")
                .map(|(lo, hi)| (lo.to_owned(), hi.to_owned()))
                .unwrap_or_else(|| (told.trim_end_matches(" m").to_owned(), String::new()));
            let lo: f64 = lo.parse().unwrap();
            (lo, hi.parse().unwrap_or(lo))
        };
        let enclosure =
            |lo: f64, hi: f64| Told::computed(&Interval::from_bounds(lo, hi), Dimension::Length);
        let length = |x: f64| Told::computed(&x, Dimension::Length);
        for (first, second) in [
            (enclosure(0.300_01, 0.300_02), enclosure(0.300_03, 0.300_04)),
            (
                enclosure(1.000_000_01e-9, 1.000_000_02e-9),
                length(1.000_000_03e-9),
            ),
            (
                length(0.5),
                enclosure(0.5_f64.next_up(), 0.5_f64.next_up().next_up()),
            ),
        ] {
            let pair = first.beside(&second);
            let (a, b) = (
                ends(pair.first().to_string()),
                ends(pair.second().to_string()),
            );
            assert!(
                a.1 < b.0,
                "{first:?} beside {second:?} reads {a:?} and {b:?}"
            );
            let pair = second.beside(&first);
            let (b, a) = (
                ends(pair.first().to_string()),
                ends(pair.second().to_string()),
            );
            assert!(
                a.1 < b.0,
                "{second:?} beside {first:?} reads {b:?} and {a:?}"
            );
        }
        // No order to state: each reads as it does alone.
        let (wide, inside) = (enclosure(1.0, 2.0), enclosure(1.25, 1.75));
        let pair = wide.beside(&inside);
        assert_eq!(pair.first().to_string(), wide.to_string());
        assert_eq!(pair.second().to_string(), inside.to_string());
        let pair = wide.beside(&length(1.5));
        assert_eq!(pair.first().to_string(), "[1, 2] m");
        assert_eq!(pair.second().to_string(), "1.5 m");
    }

    /// A pair compared by size reads its sizes in their true order,
    /// each member keeping its sign.
    #[test]
    fn a_pair_compared_by_magnitude_reads_its_sizes_in_order() {
        let length = |x: f64| Told::computed(&x, Dimension::Length);
        let (rho, least) = (length(-1.000_01e-9), length(1.000_04e-9));
        let signed = rho.beside(&least);
        assert_eq!(
            (signed.first().to_string(), signed.second().to_string()),
            ("-1e-9 m".into(), "1e-9 m".into()),
            "in signed order the two are apart at the grid"
        );
        let sized = rho.beside_by_magnitude(&least);
        assert_eq!(
            (sized.first().to_string(), sized.second().to_string()),
            ("-1.00001e-9 m".into(), "1.00004e-9 m".into())
        );
        for x in magnitudes(128).into_iter().step_by(24) {
            for y in [-x.next_up(), -x * 1.000_04, x * 1.3] {
                if !y.is_finite() {
                    continue;
                }
                let pair = length(x).beside_by_magnitude(&length(y));
                let (first, second) = (pair.first().to_string(), pair.second().to_string());
                assert_eq!(
                    read_back(&first)
                        .abs()
                        .partial_cmp(&read_back(&second).abs()),
                    x.abs().partial_cmp(&y.abs()),
                    "{x:e} beside {y:e} reads {first} and {second}"
                );
                assert_eq!(read_back(&first).is_sign_negative(), x.is_sign_negative());
                assert_eq!(read_back(&second).is_sign_negative(), y.is_sign_negative());
            }
        }
    }

    /// `Debug` writes what is told and equality compares it: neither
    /// hands back the float a computed number was built from.
    #[test]
    fn debug_and_equality_are_of_what_is_told() {
        let noisy = Told::computed(&(0.1 + 0.2), Dimension::Length);
        assert_eq!(format!("{noisy:?}"), "Told(computed \"0.3 m\")");
        assert_eq!(noisy, Told::computed(&0.3, Dimension::Length));
        assert_ne!(noisy, Told::given(&0.3, Dimension::Length), "another role");
        assert_ne!(
            noisy,
            Told::computed(&0.3, Dimension::Angle),
            "another unit"
        );
        assert_ne!(noisy, Told::computed(&0.3001, Dimension::Length));
        assert_eq!(
            format!("{:?}", Told::at_most(&0.300_04, Dimension::Angle)),
            "Told(at most \"0.3 rad\")"
        );
        let pair = noisy.beside(&Told::computed(&0.300_000_000_04, Dimension::Length));
        assert!(
            !format!("{pair:?}").contains("0.30000000000000004"),
            "{pair:?}"
        );
        assert!(
            !format!("{:?}", (0.1_f64 + 0.2).told_reading()).contains("0.3"),
            "a reading's Debug is its shape"
        );
    }

    /// A reading that is no enclosure, and a bound that is no finite
    /// number, do not read.
    #[test]
    fn an_ill_formed_reading_and_a_non_finite_offer_are_unreadable() {
        assert_eq!(
            super::Reading::enclosure(2.0, 1.0),
            super::Reading::value(f64::NAN)
        );
        let unbounded = Interval::from_bounds(1.0, f64::INFINITY);
        assert_eq!(
            Told::computed(&unbounded, Dimension::Length).to_string(),
            "[1, inf] m"
        );
        for told in [
            Told::at_least(&unbounded, Dimension::Length),
            Told::at_most(
                &Interval::from_bounds(f64::NEG_INFINITY, 1.0),
                Dimension::Length,
            ),
            Told::at_least(&f64::INFINITY, Dimension::Length),
            Told::at_most_within(&f64::NAN, &1.0, Dimension::Length),
        ] {
            assert_eq!(told.to_string(), "NaN m", "{told:?}");
        }
        assert_eq!(
            Told::at_most(&unbounded, Dimension::Length).to_string(),
            "1 m"
        );
    }

    /// A length is told no finer than the floats at it can carry: past
    /// about half a kilometre the tolerance's grid is below the gap
    /// between adjacent floats, and the float's own rounding is not
    /// spelled.
    #[test]
    fn a_large_length_is_told_no_finer_than_its_floats() {
        let length = |x: f64| Told::computed(&x, Dimension::Length).to_string();
        assert_eq!(length(3e6 * (0.1 + 0.2) / 0.3), "3000000 m");
        assert_eq!(length(600_000.1 + 0.2), "600000.3 m");
        assert_eq!(length(12_345_678.9 + 1e-9), "12345678.9 m");
        assert_eq!(length(1e3 / 3.0), "333.3333333333 m");
        for x in magnitudes(64) {
            let said = length(x);
            let step: f64 = format!("1e{}", decade_of(x) + 1 - figures(&said) as i32)
                .parse()
                .unwrap();
            assert!(
                figures(&said) <= 4 || step >= spacing(x),
                "{x:e} reads {said}, finer than its floats"
            );
        }
    }

    /// A position is its two coordinates as lengths, without a unit
    /// word.
    #[test]
    fn a_position_reads_as_its_two_coordinates() {
        let at = |x: f64| Told::computed(&x, Dimension::Length);
        assert_eq!(
            Told::position(&at(0.25), &at(-0.0)).to_string(),
            "(0.25, 0)"
        );
        assert_eq!(
            Told::position(&at(1.234_567), &at(0.1 + 0.2)).to_string(),
            "(1.234567, 0.3)"
        );
    }

    /// Only a given number comes back out.
    #[test]
    fn only_a_given_number_is_read_back() {
        let x = 0.1 + 0.2;
        assert_eq!(Told::given(&x, Dimension::Length).as_given(), Some(x));
        assert_eq!(Told::computed(&x, Dimension::Length).as_given(), None);
        assert_eq!(Told::at_least(&x, Dimension::Length).as_given(), None);
        assert_eq!(Told::at_most(&x, Dimension::Length).as_given(), None);
        let enclosure = Interval::from_bounds(1.0, 2.0);
        assert_eq!(Told::given(&enclosure, Dimension::Length).as_given(), None);
        assert_eq!(
            <f64 as Real>::from_f64(x).told_reading(),
            super::Reading::value(x)
        );
    }
}

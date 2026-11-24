//! A number told to a person: the one type a sentence quotes a scalar
//! through (the crate README, "A number told to a person").
//!
//! A [`Told`] is built where the payload is built, while the scalar is
//! in hand, and from then on it only renders. It holds the *reading*
//! (a value, an enclosure, or unreadable), the [`Dimension`] and the
//! role, and it writes the number and the unit word. It has no
//! arithmetic and no ordering, and a computed number does not come
//! back out of it. Two are equal when their role, dimension and reading
//! are, and its `Debug` shows the reading exactly.
//!
//! # Whose notation
//!
//! A told quantity is held in metres or radians and rendered in a
//! [`Notation`]: [`Tell::tell`] takes one, and `Display` is that
//! rendering in [`Notation::CANONICAL`]. A sentence that quotes told
//! quantities implements [`Tell`] once and its `Display` forwards, so
//! its text is written once and reads in any notation
//! ([`Tell::told_in`]).
//!
//! Every rule below holds **in the unit rendered**, and every unit says
//! every quantity. The number written is a decimal of that unit, and
//! what it "reads back as" is what typing it in that unit gives: the
//! decimal times the unit's exact factor, rounded once
//! ([`UnitDef::canonical`]). A length's tolerance cap is a length, so it
//! is the same distance in every unit and a different decimal step in
//! each.
//!
//! The tolerance is set in metres, so a quantity that names it
//! ([`Told::of_the_tolerance`]) is told in metres in every notation.
//!
//! # What is spelled
//!
//! Every spelling is a decimal of the unit of some number of
//! significant figures, written as [`quantity::Decimal`] writes one. The
//! role picks the figures:
//!
//! - **given**: the shortest decimal that reads back as the float;
//! - **computed**: the exact number of the unit rounded to the *grid*;
//! - **offered**: that number rounded on the grid toward the side that
//!   works when typed, and never past a far edge the builder states;
//! - **compared** ([`Told::beside`]): both numbers at the first
//!   precision, from the grid up, at which they read in their true
//!   order.
//!
//! The grid at `x` is four significant figures, and for a length the
//! finer of that and the largest power of ten of the unit that is no
//! more than a tenth of [`DEFAULT_EPS`], held no finer than the gap
//! between `x` and the next float: a digit below that gap spells the
//! float's own rounding, not the length. It has no absolute floor,
//! so a nonzero number never reads `0`. The cap is the compiled default
//! and not the run's tolerance: one payload reads the same at every
//! tolerance a run is configured with.
//!
//! "On a side" and "contains" are stated of the float a spelling reads
//! back as, compared exactly with the float held: an offered `at least`
//! spelling types back to a float not below the bound, and an
//! enclosure's spelled ends type back to floats not inside its ends.
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
use std::sync::Arc;

use quantity::{Decimal, InUnit, Notation, ROUND_TRIP_FIGURES, Rounding, UnitDef, UnitQuantity};

use crate::Real;
use crate::tolerance::DEFAULT_EPS;

/// The significant figures of the grid where no length governs.
const GRID_FIGURES: i64 = 4;

/// The decade of a length's absolute grid step, a tenth of
/// [`DEFAULT_EPS`]: `10^LENGTH_GRID_DECADE` metres.
const LENGTH_GRID_DECADE: i64 = -10;

/// The most characters one told number is written in, its sign
/// included and its unit word aside: seventeen significant figures, a
/// point, and `e-324`. A surface that boxes one number sizes the box by
/// this; an enclosure is two numbers.
pub const WIDEST_NUMBER: usize = 24;

/// What a scalar reads as: its value, its enclosure, or nothing.
///
/// Minted by [`Real::told_reading`], once per scalar type, and read
/// only by [`Told`]: it has no accessor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Reading(Read);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Read {
    /// One number; never NaN.
    Value(f64),
    /// An enclosure with `lo < hi`; neither end NaN.
    Enclosure { lo: f64, hi: f64 },
    /// Nothing was measured: NaN, or an ill-formed enclosure.
    Unreadable,
}

impl Reading {
    /// The reading's ends: a value twice, an enclosure's two, `None`
    /// where nothing was measured.
    pub(crate) fn ends(&self) -> Option<(f64, f64)> {
        match self.0 {
            Read::Value(v) => Some((v, v)),
            Read::Enclosure { lo, hi } => Some((lo, hi)),
            Read::Unreadable => None,
        }
    }

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

/// What a told number measures, which fixes the unit it is held in,
/// the unit of a notation it is told in, and whether the tolerance (a
/// length) caps its grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dimension {
    /// Held in metres.
    Length,
    /// Held in radians.
    Angle,
    /// A ratio, a count held as a scalar, a component of a direction.
    Dimensionless,
}

impl Dimension {
    /// The quantity of the unit table a number of this dimension is
    /// held in — the dimensions are that table's quantities, told.
    pub const fn quantity(self) -> UnitQuantity {
        match self {
            Self::Length => UnitQuantity::Length,
            Self::Angle => UnitQuantity::Angle,
            Self::Dimensionless => UnitQuantity::Scalar,
        }
    }

    /// The dimension a number held in `quantity` is told as —
    /// [`Self::quantity`]'s inverse.
    pub const fn of(quantity: UnitQuantity) -> Self {
        match quantity {
            UnitQuantity::Length => Self::Length,
            UnitQuantity::Angle => Self::Angle,
            UnitQuantity::Scalar => Self::Dimensionless,
        }
    }

    /// The unit of `notation` a number of this dimension is told in.
    const fn unit(self, notation: Notation) -> UnitDef {
        notation.unit_of(self.quantity())
    }
}

/// The unit as a sentence writes it after the number: a space and the
/// symbol, and nothing for the dimensionless row.
fn write_unit_word(unit: UnitDef, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match unit.symbol() {
        "" => Ok(()),
        symbol => write!(f, " {symbol}"),
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
#[derive(Clone, Copy, Debug, PartialEq)]
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

/// One number as a unit spells it: a decimal of the unit, or an
/// infinity, which reads `inf` in every unit.
#[derive(Clone, Debug, PartialEq)]
enum Number {
    Decimal(Decimal),
    Infinite { negative: bool },
}

impl Number {
    /// The float this reads back as in `unit`.
    fn read_back(&self, unit: UnitDef) -> f64 {
        match self {
            Self::Decimal(decimal) => unit.canonical(decimal),
            Self::Infinite { negative: true } => f64::NEG_INFINITY,
            Self::Infinite { negative: false } => f64::INFINITY,
        }
    }

    /// The number as a sentence writes it: a zero of either sign as `0`.
    fn write(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decimal(decimal) if decimal.is_zero() => f.write_str("0"),
            Self::Decimal(decimal) => fmt::Display::fmt(decimal, f),
            Self::Infinite { negative: true } => f.write_str("-inf"),
            Self::Infinite { negative: false } => f.write_str("inf"),
        }
    }
}

/// What is spelled: one number, or the two ends of an enclosure.
#[derive(Clone, Debug, PartialEq)]
enum Spelled {
    One(Number),
    Ends(Number, Number),
}

impl Spelled {
    /// The floats this reads back as in `unit`, as `(lo, hi)`.
    fn read_back(&self, unit: UnitDef) -> (f64, f64) {
        match self {
            Self::One(y) => (y.read_back(unit), y.read_back(unit)),
            Self::Ends(lo, hi) => (lo.read_back(unit), hi.read_back(unit)),
        }
    }
}

/// The extent `(lo, hi)` as `(least, greatest)`, by distance from zero
/// where `magnitudes`.
fn extent((lo, hi): (f64, f64), magnitudes: bool) -> (f64, f64) {
    if !magnitudes {
        (lo, hi)
    } else if lo <= 0.0 && hi >= 0.0 {
        (0.0, lo.abs().max(hi.abs()))
    } else {
        (lo.abs().min(hi.abs()), lo.abs().max(hi.abs()))
    }
}

/// How the extent `a` stands to `b`: before it, after it, or the same
/// single number. `None` where they overlap without being one number,
/// which has no order.
fn order(a: (f64, f64), b: (f64, f64), magnitudes: bool) -> Option<Ordering> {
    let ((a_lo, a_hi), (b_lo, b_hi)) = (extent(a, magnitudes), extent(b, magnitudes));
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

/// What a [`Told`] holds.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Quantity {
    held: Held,
    dimension: Dimension,
    /// Told in the canonical unit in every notation
    /// ([`Told::of_the_tolerance`]).
    names_the_tolerance: bool,
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
            names_the_tolerance: false,
        }
    }

    /// The unit this is told in under `notation`.
    const fn unit(&self, notation: Notation) -> UnitDef {
        if self.names_the_tolerance {
            self.dimension.unit(Notation::CANONICAL)
        } else {
            self.dimension.unit(notation)
        }
    }

    /// The floats held, as `(lo, hi)`; `None` for a reading that does
    /// not read.
    fn held_extent(&self) -> Option<(f64, f64)> {
        match self.held {
            Held::Given(read) | Held::Computed(read) => match read {
                Read::Value(x) => Some((x, x)),
                Read::Enclosure { lo, hi } => Some((lo, hi)),
                Read::Unreadable => None,
            },
            Held::Offered { bound, .. } => bound.map(|x| (x, x)),
        }
    }

    /// What is spelled in `unit` at `extra` figures past the grid;
    /// `None` for a reading that does not read.
    fn spelled_in(&self, unit: UnitDef, extra: usize) -> Option<Spelled> {
        self.spelled_at(&self.floats(unit), unit, extra)
    }

    /// The floats held as exact numbers of `unit` with their grids, for
    /// the roles that round: a value or an enclosure's two ends.
    fn floats(&self, unit: UnitDef) -> [Option<Float>; 2] {
        let float = |x: f64| Some(Float::of(x, unit, self.dimension));
        match self.held {
            Held::Computed(Read::Value(x)) => [float(x), None],
            Held::Computed(Read::Enclosure { lo, hi }) => [float(lo), float(hi)],
            Held::Offered { bound: Some(x), .. } => [float(x), None],
            Held::Given(_)
            | Held::Computed(Read::Unreadable)
            | Held::Offered { bound: None, .. } => [None, None],
        }
    }

    /// [`Quantity::spelled_in`] from floats already built.
    fn spelled_at(
        &self,
        floats: &[Option<Float>; 2],
        unit: UnitDef,
        extra: usize,
    ) -> Option<Spelled> {
        let [first, second] = floats;
        Some(match self.held {
            Held::Given(Read::Value(x)) => Spelled::One(given(x, unit)),
            Held::Given(Read::Enclosure { lo, hi }) => {
                Spelled::Ends(given(lo, unit), given(hi, unit))
            }
            Held::Given(Read::Unreadable) | Held::Computed(Read::Unreadable) => return None,
            Held::Computed(Read::Value(_)) => {
                let x = first.as_ref()?;
                Spelled::One(x.nearest(x.figures(extra)))
            }
            Held::Computed(Read::Enclosure { .. }) => {
                let (lo, hi) = (first.as_ref()?, second.as_ref()?);
                Spelled::Ends(
                    lo.toward(lo.figures(extra), Side::AtMost),
                    hi.toward(hi.figures(extra), Side::AtLeast),
                )
            }
            Held::Offered { side, edge, .. } => {
                let x = first.as_ref()?;
                let within = |y: &Number| {
                    let typed = y.read_back(unit);
                    edge.is_none_or(|edge| !side.holds(typed, edge) || typed == edge)
                };
                // At round-trip figures the offer is the bound itself,
                // which is inside its window.
                let Some(offer) = (x.figures(extra)..=ROUND_TRIP_FIGURES)
                    .map(|figures| x.toward(figures, side))
                    .find(within)
                else {
                    unreachable!("an offer at round-trip figures is its own bound")
                };
                Spelled::One(offer)
            }
        })
    }

    fn write(&self, f: &mut fmt::Formatter<'_>, notation: Notation, extra: usize) -> fmt::Result {
        let unit = self.unit(notation);
        write_spelled(self.spelled_in(unit, extra).as_ref(), f)?;
        write_unit_word(unit, f)
    }
}

/// What is spelled as a sentence writes it: one number, the two ends
/// of an enclosure, or `NaN` for a reading that does not read.
fn write_spelled(spelled: Option<&Spelled>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match spelled {
        Some(Spelled::One(y)) => y.write(f),
        Some(Spelled::Ends(lo, hi)) => {
            f.write_str("[")?;
            lo.write(f)?;
            f.write_str(", ")?;
            hi.write(f)?;
            f.write_str("]")
        }
        None => f.write_str("NaN"),
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

    /// This quantity as one that **names the tolerance**: the
    /// tolerance is set in metres, so it is told in metres in every
    /// notation (the crate README, "A number told to a person").
    #[must_use]
    pub fn of_the_tolerance(mut self) -> Self {
        self.0.names_the_tolerance = true;
        self
    }

    /// **A position in a plane's chart**, `(x, y)`: each coordinate a
    /// length told as this one is, as a sentence that locates
    /// something writes it.
    ///
    /// # Panics
    ///
    /// Where the two are of two dimensions, or one names the tolerance
    /// and the other does not: they are then not one position, and the
    /// builder that paired them has a bug.
    #[must_use]
    pub fn position(x: &Self, y: &Self) -> Position {
        assert!(
            x.0.dimension == y.0.dimension && x.0.names_the_tolerance == y.0.names_the_tolerance,
            "a position's coordinates are told in one unit"
        );
        Position { x: *x.0, y: *y.0 }
    }

    /// The pair `(self, other)` as a sentence that states their order
    /// tells them.
    ///
    /// # Panics
    ///
    /// Where the two are of two dimensions: no sentence states the
    /// order of a length and an angle, and the builder that paired them
    /// has a bug.
    #[must_use]
    pub fn beside(&self, other: &Self) -> Compared {
        Compared::new(*self.0, *other.0, false)
    }

    /// [`Told::beside`], for a sentence that states the order of the
    /// two **sizes** (a signed lever "shorter than" a length): each is
    /// told with its sign, at the first precision at which their
    /// distances from zero read in their true order.
    ///
    /// # Panics
    ///
    /// As [`Told::beside`].
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
            names_the_tolerance: false,
        })
    }

    /// A computed number from a reading already taken.
    pub(crate) fn computed_reading(reading: Reading, dimension: Dimension) -> Self {
        Self::quantity(Quantity {
            held: Held::Computed(reading.0),
            dimension,
            names_the_tolerance: false,
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

    /// The float this number's canonical spelling reads back as, where
    /// it is one number.
    pub(crate) fn spelled(&self) -> Option<f64> {
        let unit = self.0.dimension.unit(Notation::CANONICAL);
        match self.0.spelled_in(unit, 0)? {
            Spelled::One(y) => Some(y.read_back(unit)),
            Spelled::Ends(..) => None,
        }
    }

    /// The number alone as it is told in `notation`, for a surface
    /// that writes the unit itself beside it.
    #[must_use]
    pub fn number_in(&self, notation: Notation) -> impl fmt::Display + use<> {
        let quantity = *self.0;
        let spelled = quantity.spelled_in(quantity.unit(notation), 0);
        fmt::from_fn(move |f| write_spelled(spelled.as_ref(), f))
    }

    /// The number in the canonical unit, without its unit word.
    pub(crate) fn number(&self) -> impl fmt::Display {
        self.number_in(Notation::CANONICAL)
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
        self.0.held_extent().unwrap_or((f64::NAN, f64::NAN))
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

/// **Text that quotes told quantities, written once and read in any
/// notation** (the module docs, "Whose notation").
///
/// A refusal, a recourse or a readout implements this where it would
/// have implemented `Display`, writing each quantity it holds through
/// [`Tell::told_in`], and its `Display` forwards to
/// [`Notation::CANONICAL`]. A sentence that wraps another forwards the
/// notation to it; one that formats the inner sentence with `{}`
/// instead still tells the truth, in metres and radians.
pub trait Tell {
    /// Writes this in `notation`.
    ///
    /// # Errors
    ///
    /// The formatter's own.
    fn tell(&self, notation: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    /// This, as text in `notation`.
    fn told_in(&self, notation: Notation) -> ToldIn<'_, Self>
    where
        Self: Sized,
    {
        ToldIn {
            what: self,
            notation,
        }
    }
}

impl<S: Tell + ?Sized> Tell for &S {
    fn tell(&self, notation: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).tell(notation, f)
    }
}

/// [`Tell::told_in`]'s text.
#[derive(Clone, Copy, Debug)]
pub struct ToldIn<'a, S> {
    what: &'a S,
    notation: Notation,
}

impl<S: Tell> fmt::Display for ToldIn<'_, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.what.tell(self.notation, f)
    }
}

/// Text that quotes no quantity reads the same in every notation.
impl Tell for str {
    fn tell(&self, _: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

impl Tell for String {
    fn tell(&self, _: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self)
    }
}

/// What a [`Sentence`] keeps: a sentence that tells and dumps.
trait Kept: Tell + fmt::Debug + Send + Sync {}

impl<S: Tell + fmt::Debug + Send + Sync> Kept for S {}

/// **A sentence kept to be told later**: what a value that outlives
/// the refusal it reports holds in place of the refusal's text, so the
/// notation is still the reader's to choose when it is shown. A
/// `String` taken from a refusal's `Display` has already chosen metres.
///
/// Its `Display` is the canonical text, and its `Debug` is the kept
/// sentence's own. It has no equality: what it keeps is any sentence,
/// and two sentences are equal by what they hold, which a kept one
/// cannot be asked.
#[derive(Clone)]
pub struct Sentence(Arc<dyn Kept>);

impl Sentence {
    /// `said`, kept.
    #[must_use]
    pub fn new(said: impl Tell + fmt::Debug + Send + Sync + 'static) -> Self {
        Self(Arc::new(said))
    }
}

impl From<String> for Sentence {
    fn from(text: String) -> Self {
        Self::new(text)
    }
}

impl From<&str> for Sentence {
    fn from(text: &str) -> Self {
        Self::new(text.to_owned())
    }
}

impl Tell for Sentence {
    fn tell(&self, notation: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.tell(notation, f)
    }
}

impl fmt::Display for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tell(Notation::CANONICAL, f)
    }
}

impl fmt::Debug for Sentence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.0, f)
    }
}

/// The number and its unit word.
impl Tell for Told {
    fn tell(&self, notation: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.write(f, notation, 0)
    }
}

impl fmt::Display for Told {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tell(Notation::CANONICAL, f)
    }
}

/// **A position in a plane's chart** ([`Told::position`]): `(x, y)`,
/// both coordinates in one unit. In the canonical unit the pair carries
/// no unit word; in any other unit the word follows the pair.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    x: Quantity,
    y: Quantity,
}

impl Tell for Position {
    fn tell(&self, notation: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unit = self.x.unit(notation);
        f.write_str("(")?;
        write_spelled(self.x.spelled_in(unit, 0).as_ref(), f)?;
        f.write_str(", ")?;
        write_spelled(self.y.spelled_in(unit, 0).as_ref(), f)?;
        f.write_str(")")?;
        if unit != self.x.dimension.unit(Notation::CANONICAL) {
            write_unit_word(unit, f)?;
        }
        Ok(())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tell(Notation::CANONICAL, f)
    }
}

/// **Two numbers whose order the sentence states**, or a reading beside
/// its own threshold ([`Told::beside`]): each is told at the first
/// precision, from its own grid up, at which the two read in their true
/// order, in every role and in the unit each is told in. Two numbers
/// that differ never read alike, and two enclosures that do not meet
/// read apart and in order.
///
/// A pair with no order is told as each member is alone, and the form
/// claims nothing of it: two enclosures that overlap, an enclosure that
/// holds the number beside it, or a member that does not read.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Compared {
    first: Quantity,
    second: Quantity,
    /// The order stated is of the distances from zero.
    magnitudes: bool,
}

impl Compared {
    fn new(first: Quantity, second: Quantity, magnitudes: bool) -> Self {
        assert_eq!(
            first.dimension, second.dimension,
            "a compared pair is of one dimension"
        );
        Self {
            first,
            second,
            magnitudes,
        }
    }

    /// The figures past each member's grid at which the pair is told in
    /// `notation`: the first at which the two read in their true order,
    /// and none where they have no order.
    fn extra(&self, notation: Notation) -> usize {
        let (first, second) = (&self.first, &self.second);
        let held = first
            .held_extent()
            .zip(second.held_extent())
            .and_then(|(a, b)| order(a, b, self.magnitudes));
        let Some(held) = held else {
            return 0;
        };
        let (first_unit, second_unit) = (first.unit(notation), second.unit(notation));
        let (first_floats, second_floats) = (first.floats(first_unit), second.floats(second_unit));
        let reads = |extra: usize| {
            let a = first
                .spelled_at(&first_floats, first_unit, extra)?
                .read_back(first_unit);
            let b = second
                .spelled_at(&second_floats, second_unit, extra)?
                .read_back(second_unit);
            order(a, b, self.magnitudes)
        };
        let found = (0..=ROUND_TRIP_FIGURES).find(|&extra| reads(extra) == Some(held));
        // At round-trip figures each member reads back as what it holds.
        let Some(extra) = found else {
            unreachable!("a pair at round-trip figures reads back as it is held")
        };
        extra
    }

    /// [`Compared::extra`], found once for the two members of one pair
    /// told in one notation: a sentence tells its first member and then
    /// its second, and the second reads what the first found.
    fn extra_once(&self, notation: Notation) -> usize {
        thread_local! {
            static LAST: core::cell::Cell<Option<(Compared, Notation, usize)>> =
                const { core::cell::Cell::new(None) };
        }
        LAST.with(|last| match last.get() {
            Some((pair, at, extra)) if pair == *self && at == notation => extra,
            _ => {
                let extra = self.extra(notation);
                last.set(Some((*self, notation, extra)));
                extra
            }
        })
    }

    /// The first number and its unit word.
    #[must_use]
    pub fn first(&self) -> Member {
        Member {
            pair: *self,
            second: false,
        }
    }

    /// The second number and its unit word.
    #[must_use]
    pub fn second(&self) -> Member {
        Member {
            pair: *self,
            second: true,
        }
    }
}

/// One number of a [`Compared`] pair and its unit word.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Member {
    pair: Compared,
    second: bool,
}

impl Tell for Member {
    fn tell(&self, notation: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let member = if self.second {
            &self.pair.second
        } else {
            &self.pair.first
        };
        member.write(f, notation, self.pair.extra_once(notation))
    }
}

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tell(Notation::CANONICAL, f)
    }
}

/// One float a quantity holds, as an exact number of the unit it is
/// told in, with the figures of its grid there (the module docs) and
/// the figures at which a decimal of the unit steps no finer than the
/// floats at it, past which a digit spells the float's own rounding:
/// there it is told as received. Neither is more than
/// [`ROUND_TRIP_FIGURES`]. Built once for a quantity told at several
/// precisions.
///
/// A length's grid step is the largest power of ten of the unit that is
/// no more than a tenth of the default tolerance, or where the floats
/// at `x` are sparser than that, the first power of ten at or above the
/// gap to the next float; at the top of the format, where there is no
/// next float, the step is the number's own decade, so it reads at four
/// figures.
#[derive(Clone, Debug)]
struct Float {
    x: f64,
    unit: UnitDef,
    /// `None` for a zero or a value that is not finite.
    number: Option<InUnit>,
    grid: usize,
    own: usize,
}

impl Float {
    fn of(x: f64, unit: UnitDef, dimension: Dimension) -> Self {
        if x == 0.0 || !x.is_finite() {
            return Self {
                x,
                unit,
                number: None,
                grid: GRID_FIGURES as usize,
                own: ROUND_TRIP_FIGURES,
            };
        }
        let number = unit.in_unit(x);
        let decade = number.decade().0;
        let (gap, top) = match x.abs().next_up() - x.abs() {
            up if up.is_finite() => (up, false),
            _ => (x.abs() - x.abs().next_down(), true),
        };
        let (gap_decade, exact) = unit.decade(gap);
        // 10^k of the unit is at most 10^LENGTH_GRID_DECADE metres from
        // k = LENGTH_GRID_DECADE plus the decade of a metre in the unit.
        let cap = LENGTH_GRID_DECADE + unit.decade(1.0).0;
        let step = if top {
            decade
        } else {
            cap.max(gap_decade + i64::from(!exact))
        };
        let grid = match dimension {
            Dimension::Length => GRID_FIGURES.max(decade - step + 1),
            Dimension::Angle | Dimension::Dimensionless => GRID_FIGURES,
        };
        let clamp = |figures: i64| {
            usize::try_from(figures.max(1))
                .unwrap_or(ROUND_TRIP_FIGURES)
                .min(ROUND_TRIP_FIGURES)
        };
        Self {
            x,
            unit,
            number: Some(number),
            grid: clamp(grid),
            own: clamp(decade - gap_decade + 1),
        }
    }

    /// The figures at `extra` past the grid: round-trip figures where
    /// that reaches what the floats carry.
    fn figures(&self, extra: usize) -> usize {
        if self.grid + extra >= self.own {
            ROUND_TRIP_FIGURES
        } else {
            self.grid + extra
        }
    }

    /// The decimal at `figures`, where the float is a nonzero number.
    fn decimal(&self, figures: usize, rounding: Rounding) -> Option<Decimal> {
        Some(self.number.as_ref()?.decimal(figures, rounding))
    }

    /// Rounded to nearest at `figures`; at round-trip figures, or for a
    /// zero or a value that is not finite, as received. A rounding past
    /// the largest float is the grid point toward zero from it.
    fn nearest(&self, figures: usize) -> Number {
        let near = (figures < ROUND_TRIP_FIGURES)
            .then(|| self.decimal(figures, Rounding::Nearest))
            .flatten();
        let Some(near) = near else {
            return given(self.x, self.unit);
        };
        if self.unit.canonical(&near).is_finite() {
            return Number::Decimal(near);
        }
        let toward_zero = if self.x < 0.0 {
            Rounding::Up
        } else {
            Rounding::Down
        };
        self.decimal(figures, toward_zero)
            .map_or_else(|| given(self.x, self.unit), Number::Decimal)
    }

    /// At `figures`, typing back on `side` of the float: the nearest
    /// decimal where that already does, and the one rounded toward the
    /// side where it does not. At round-trip figures, for a zero or a
    /// value that is not finite, or where the rounding toward the side
    /// types back past the largest float, as received.
    fn toward(&self, figures: usize, side: Side) -> Number {
        if figures >= ROUND_TRIP_FIGURES {
            return given(self.x, self.unit);
        }
        let directed = match side {
            Side::AtLeast => Rounding::Up,
            Side::AtMost => Rounding::Down,
        };
        [Rounding::Nearest, directed]
            .into_iter()
            .filter_map(|rounding| self.decimal(figures, rounding))
            .find(|y| {
                let typed = self.unit.canonical(y);
                typed.is_finite() && side.holds(typed, self.x)
            })
            .map_or_else(|| given(self.x, self.unit), Number::Decimal)
    }
}

/// `x` in `unit` exactly as received: the shortest decimal that reads
/// back as it.
fn given(x: f64, unit: UnitDef) -> Number {
    if x.is_finite() {
        Number::Decimal(unit.shortest(x))
    } else {
        Number::Infinite { negative: x < 0.0 }
    }
}

// The grid's absolute step is a tenth of the default tolerance.
const _: () = assert!(DEFAULT_EPS == 1e-9 && LENGTH_GRID_DECADE == -10);

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{Dimension, Tell, Told, WIDEST_NUMBER};
    use crate::{Dual, Interval, Readable, Real};
    use quantity::{CM, DEG, Decimal, IN, M, MM, Notation, PI, RAD, UnitDef};

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
        assert_eq!(within(0.5).pinned_edge(), None);
        assert_eq!(within(0.5).to_string(), "1.001 rad");
        assert_eq!(within(1.000_05).pinned_edge(), Some(1.000_05));
        assert_eq!(
            Told::at_most_within(&1.000_04, &1.5, Dimension::Angle).pinned_edge(),
            None
        );
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
        for x in magnitudes(128).into_iter().step_by(48) {
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

    /// Two told quantities are equal when their role, dimension and
    /// reading are, and `Debug` shows the reading exactly; two that read
    /// alike are compared by their telling.
    #[test]
    fn equality_and_debug_are_of_what_is_held() {
        let noisy = Told::computed(&(0.1 + 0.2), Dimension::Length);
        assert!(
            format!("{noisy:?}").contains("Value(0.30000000000000004)"),
            "{noisy:?}"
        );
        assert_eq!(noisy, Told::computed(&(0.1 + 0.2), Dimension::Length));
        let plain = Told::computed(&0.3, Dimension::Length);
        assert_ne!(noisy, plain, "another reading");
        assert_eq!(
            noisy.told_in(Notation::CANONICAL).to_string(),
            plain.told_in(Notation::CANONICAL).to_string(),
            "which reads alike"
        );
        assert_ne!(plain, Told::given(&0.3, Dimension::Length), "another role");
        assert_ne!(
            plain,
            Told::computed(&0.3, Dimension::Angle),
            "another dimension"
        );
        assert_ne!(
            plain,
            Told::computed(&0.3, Dimension::Length).of_the_tolerance(),
            "a number that names the tolerance has another role"
        );
        assert_ne!(
            Told::at_least(&0.3, Dimension::Length),
            Told::at_least_within(&0.3, &0.4, Dimension::Length),
            "another window"
        );
        assert!(
            format!("{:?}", (0.1_f64 + 0.2).told_reading()).contains("0.30000000000000004"),
            "a reading's Debug is its number"
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

    /// Every length unit and every angle unit of the table, in four
    /// notations; the first is the canonical one.
    const NOTATIONS: [Notation; 4] = [
        Notation::CANONICAL,
        Notation {
            length: MM,
            angle: DEG,
        },
        Notation {
            length: IN,
            angle: PI,
        },
        Notation {
            length: CM,
            angle: RAD,
        },
    ];

    /// What a told text says: its decimals (one, or an enclosure's two)
    /// and the unit its word names.
    fn said(told: &str) -> (Vec<Decimal>, UnitDef) {
        let (numbers, symbol) = match told.strip_prefix('[') {
            Some(rest) => {
                let (ends, word) = rest.split_once(']').unwrap();
                (ends.split(", ").collect::<Vec<_>>(), word.trim_start())
            }
            None => {
                let (number, word) = told.split_once(' ').unwrap_or((told, ""));
                (vec![number], word)
            }
        };
        let unit = quantity::unit_by_symbol(symbol)
            .unwrap_or_else(|| panic!("{told:?} names no unit of the table"));
        let numbers = numbers
            .into_iter()
            .map(|n| Decimal::parse(n).unwrap_or_else(|| panic!("{told:?}: {n} is no decimal")))
            .collect();
        (numbers, unit)
    }

    /// The float nearest a decimal, as a number of its unit.
    fn float(number: &Decimal) -> f64 {
        number.to_string().parse().unwrap()
    }

    /// What typing a told text back gives, canonical: each decimal
    /// through its unit's one door, the text door's arithmetic.
    fn typed_back(told: &str) -> Vec<f64> {
        let (numbers, unit) = said(told);
        numbers.iter().map(|n| unit.canonical(n)).collect()
    }

    /// The largest power of ten of a length unit that is no more than
    /// a tenth of the default tolerance, written out per unit.
    fn tolerance_step(unit: UnitDef) -> f64 {
        let step = match unit.symbol() {
            "m" => 1e-10,
            "cm" => 1e-8,
            "mm" => 1e-7,
            "in" => 1e-9,
            other => panic!("{other} is not a length unit"),
        };
        let metres = unit.canonical(&Decimal::of_float(step).unwrap());
        assert!(metres <= 1e-10 * (1.0 + 1e-12) && metres * 10.0 > 1e-10);
        step
    }

    /// The step of a spelled number's last written digit.
    fn last_digit_step(number: &str) -> f64 {
        let (mantissa, exponent) = number.split_once('e').unwrap_or((number, "0"));
        let decimals = mantissa.split_once('.').map_or(0, |(_, d)| d.len() as i32);
        let trailing_zeros = if decimals == 0 {
            (mantissa.len() - mantissa.trim_end_matches('0').len()) as i32
        } else {
            0
        };
        let exponent: i32 = exponent.parse().unwrap();
        format!("1e{}", exponent - decimals + trailing_zeros)
            .parse()
            .unwrap()
    }

    #[test]
    fn display_is_the_rendering_in_metres_and_radians() {
        let enclosure = Interval::from_bounds(0.3, 0.300_000_4);
        for x in magnitudes(64).into_iter().step_by(3) {
            for dimension in DIMENSIONS {
                let near = x * 1.000_01;
                for told in [
                    Told::given(&x, dimension),
                    Told::computed(&x, dimension),
                    Told::at_least(&x, dimension),
                    Told::at_most_within(&x, &(x - x.abs() * 1e-9), dimension),
                    Told::computed(&enclosure, dimension),
                    Told::computed(&x, dimension).of_the_tolerance(),
                ] {
                    let canonical = told.told_in(Notation::CANONICAL).to_string();
                    assert_eq!(told.to_string(), canonical, "{told:?}");
                    let pair = told.beside(&Told::computed(&near, dimension));
                    for (member, at) in [(pair.first(), "first"), (pair.second(), "second")] {
                        assert_eq!(
                            member.to_string(),
                            member.told_in(Notation::CANONICAL).to_string(),
                            "{at} of {pair:?}"
                        );
                    }
                }
            }
        }
        let (x, y) = (
            Told::computed(&0.25, Dimension::Length),
            Told::computed(&-1.5, Dimension::Length),
        );
        let at = Told::position(&x, &y);
        assert_eq!(at.to_string(), "(0.25, -1.5)");
        assert_eq!(at.told_in(Notation::CANONICAL).to_string(), "(0.25, -1.5)");
    }

    /// The rule's own examples, in the units a person reads.
    #[test]
    fn a_quantity_reads_in_the_readers_units() {
        let mm_deg = NOTATIONS[1];
        let in_pi = NOTATIONS[2];
        let length = |x: f64| Told::computed(&x, Dimension::Length);
        let angle = |x: f64| Told::computed(&x, Dimension::Angle);
        let in_ = |told: &Told, notation| told.told_in(notation).to_string();
        assert_eq!(in_(&length(0.008_000_000_000_000_002), mm_deg), "8 mm");
        // A tenth of the tolerance is 1e-7 mm and 1e-9 in.
        assert_eq!(
            in_(&length(0.361_629_480_311_458), mm_deg),
            "361.6294803 mm"
        );
        assert_eq!(
            in_(&length(0.025_4 * 1.234_567_891_234), in_pi),
            "1.234567891 in"
        );
        assert_eq!(in_(&length(1.234_567_89e-7), mm_deg), "0.0001235 mm");
        assert_eq!(
            in_(&length(1_234.500_000_000_4), mm_deg),
            "1234500.0000004 mm"
        );
        assert_eq!(in_(&angle(core::f64::consts::FRAC_PI_2), mm_deg), "90 deg");
        assert_eq!(in_(&angle(1.0), mm_deg), "57.3 deg");
        assert_eq!(
            in_(&angle(core::f64::consts::PI / 3.0), in_pi),
            "0.3333 pi rad"
        );
        let ratio = Told::computed(&(0.1 + 0.2), Dimension::Dimensionless);
        assert_eq!(in_(&ratio, mm_deg), "0.3");
        assert_eq!(
            in_(&Told::given(&0.025, Dimension::Length), mm_deg),
            "25 mm"
        );
        assert_eq!(in_(&Told::given(&0.0254, Dimension::Length), in_pi), "1 in");
        // 300.04 typed in millimetres is the float nearest 0.30004 m,
        // so the offer is the bound's own decimal.
        assert_eq!((300.04 * MM).meters(), 0.300_04);
        assert_eq!(
            in_(&Told::at_most(&0.300_04, Dimension::Length), mm_deg),
            "300.04 mm"
        );
        assert_eq!(
            in_(&Told::at_most(&0.000_13, Dimension::Length), mm_deg),
            "0.13 mm"
        );
        assert_eq!(
            in_(&Told::at_most(&0.012_5, Dimension::Length), mm_deg),
            "12.5 mm"
        );
        assert_eq!(
            in_(
                &Told::at_least(&0.300_000_000_04, Dimension::Length),
                mm_deg
            ),
            "300.0000001 mm"
        );
        let enclosure = Told::computed(&Interval::from_bounds(0.1, 0.2), Dimension::Length);
        assert_eq!(in_(&enclosure, mm_deg), "[100, 200] mm");
        let pair = length(0.250_001).beside(&length(0.250_002));
        assert_eq!(pair.first().told_in(mm_deg).to_string(), "250.001 mm");
        assert_eq!(pair.second().told_in(mm_deg).to_string(), "250.002 mm");
        let at = Told::position(&length(0.25), &length(-0.0));
        assert_eq!(at.told_in(mm_deg).to_string(), "(250, 0) mm");
        assert_eq!(
            in_(&Told::computed(&f64::NAN, Dimension::Length), mm_deg),
            "NaN mm"
        );
        assert_eq!(
            in_(&Told::computed(&f64::INFINITY, Dimension::Length), mm_deg),
            "inf mm"
        );
    }

    /// The tolerance is set in metres and said in metres, whatever the
    /// reader's units.
    #[test]
    fn a_quantity_that_names_the_tolerance_is_told_in_metres() {
        for x in magnitudes(128) {
            let tolerance = Told::at_most(&x, Dimension::Length).of_the_tolerance();
            let length = Told::computed(&x, Dimension::Length);
            for notation in NOTATIONS {
                assert_eq!(
                    tolerance.told_in(notation).to_string(),
                    tolerance.to_string(),
                    "{x:e}"
                );
                // Beside a length it alone is in metres.
                let pair = length.beside(&tolerance);
                let (first, second) = (
                    pair.first().told_in(notation).to_string(),
                    pair.second().told_in(notation).to_string(),
                );
                assert_eq!(said(&first).1, notation.length.def(), "{first}");
                assert_eq!(said(&second).1, M.def(), "{second}");
            }
        }
        let tolerance = Told::at_most(&2e-10, Dimension::Length).of_the_tolerance();
        assert_eq!(tolerance.told_in(NOTATIONS[1]).to_string(), "2e-10 m");
        assert_ne!(tolerance, Told::at_most(&2e-10, Dimension::Length));
    }

    /// A round metre bound offered in millimetres is its own decimal
    /// there: the decimal of the unit types back as the float nearest
    /// it (`work/scalar/an-offer-in-a-decimal-unit-reads-a-run-of-nines.md`).
    #[test]
    fn a_round_metre_offer_in_millimetres_reads_its_own_decimal() {
        for k in 1..=2_000_u32 {
            let written = format!("{}e-5", k * 31 + 7);
            let b: f64 = written.parse().unwrap();
            let millimetres = MM.def().shortest(b);
            for told in [
                Told::at_most(&b, Dimension::Length),
                Told::at_least(&b, Dimension::Length),
            ] {
                let text = told.told_in(NOTATIONS[1]).to_string();
                assert_eq!(text, format!("{millimetres} mm"), "{written} m");
            }
        }
    }

    /// The number alone is the told number without its word, in every
    /// unit.
    #[test]
    fn the_number_alone_is_the_told_number_without_its_word() {
        let number = |told: &Told, notation| told.number_in(notation).to_string();
        let length = Told::computed(&0.025, Dimension::Length);
        assert_eq!(number(&length, NOTATIONS[1]), "25");
        assert_eq!(number(&length, Notation::CANONICAL), "0.025");
        let huge = Told::computed(&1e306, Dimension::Length);
        assert_eq!(number(&huge, NOTATIONS[1]), "1e309");
        assert_eq!(number(&huge, Notation::CANONICAL), "1e306");
        let unreadable = Told::computed(&f64::NAN, Dimension::Length);
        assert_eq!(number(&unreadable, NOTATIONS[1]), "NaN");
    }

    /// A kept sentence is told in the notation of whoever reads it,
    /// whenever that is.
    #[test]
    fn a_kept_sentence_reads_in_the_notation_it_is_shown_in() {
        let kept = super::Sentence::new(Told::computed(&0.25, Dimension::Length));
        assert_eq!(kept.to_string(), "0.25 m");
        assert_eq!(kept.told_in(NOTATIONS[1]).to_string(), "250 mm");
        assert_eq!(
            format!("{kept:?}"),
            format!("{:?}", Told::computed(&0.25, Dimension::Length)),
            "its dump is the kept sentence's"
        );
        assert_eq!(
            super::Sentence::from("no number")
                .told_in(NOTATIONS[1])
                .to_string(),
            "no number"
        );
    }

    /// A given number typed back in the unit it is told in is the
    /// number received, to the bit, in the notation's own unit.
    #[test]
    fn a_given_number_types_back_exactly_in_every_notation() {
        for x in magnitudes(32) {
            for notation in NOTATIONS {
                for dimension in DIMENSIONS {
                    let text = Told::given(&x, dimension).told_in(notation).to_string();
                    let (_, unit) = said(&text);
                    assert_eq!(typed_back(&text)[0].to_bits(), x.to_bits(), "{x:e}: {text}");
                    assert_eq!(unit, dimension.unit(notation), "{text}");
                }
            }
        }
        // A number authored in a unit reads as it was authored.
        for (authored, unit, notation) in [
            (12.7, MM, NOTATIONS[1]),
            (0.1, MM, NOTATIONS[1]),
            (3.3, IN, NOTATIONS[2]),
            (2.54, CM, NOTATIONS[3]),
            (7.0, M, NOTATIONS[0]),
        ] {
            let told = Told::given(&(authored * unit).meters(), Dimension::Length);
            assert_eq!(
                told.told_in(notation).to_string(),
                format!("{authored} {}", unit.symbol())
            );
        }
        for (authored, unit, notation) in [(45.0, DEG, NOTATIONS[1]), (0.25, PI, NOTATIONS[2])] {
            let told = Told::given(&(authored * unit).radians(), Dimension::Angle);
            assert_eq!(
                told.told_in(notation).to_string(),
                format!("{authored} {}", unit.symbol())
            );
        }
    }

    /// An offered bound typed back in the unit it is told in lies on
    /// its side of the bound and inside its window, and is on that
    /// unit's grid: within one step of the bound, the step no finer
    /// than a tenth of the tolerance asks for.
    #[test]
    fn an_offered_bound_typed_back_lies_on_its_side_in_every_notation() {
        for x in magnitudes(64).into_iter().step_by(4) {
            for notation in NOTATIONS {
                for dimension in DIMENSIONS {
                    let up = Told::at_least(&x, dimension).told_in(notation).to_string();
                    let down = Told::at_most(&x, dimension).told_in(notation).to_string();
                    assert!(typed_back(&up)[0] >= x, "at least {x:e} reads {up}");
                    assert!(typed_back(&down)[0] <= x, "at most {x:e} reads {down}");
                    for text in [&up, &down] {
                        let (numbers, unit) = said(text);
                        let Some(q) = unit.number_of(x).filter(|q| q.is_finite()) else {
                            continue;
                        };
                        let reach = step(q, 4) + 2.0 * spacing(q);
                        assert!(
                            (float(&numbers[0]) - q).abs() <= reach || typed_back(text)[0] == x,
                            "{x:e} is offered as {text}"
                        );
                    }
                }
                for reach in [1e-3, 1e-9, 1e-13, 0.0] {
                    let width = x.abs() * reach;
                    let within = Told::at_least_within(&x, &(x + width), Dimension::Length);
                    let y = typed_back(&within.told_in(notation).to_string())[0];
                    assert!(
                        x <= y && y <= x + width,
                        "{x:e} within {reach:e} types {y:e}"
                    );
                    let within = Told::at_most_within(&x, &(x - width), Dimension::Length);
                    let y = typed_back(&within.told_in(notation).to_string())[0];
                    assert!(
                        x - width <= y && y <= x,
                        "{x:e} within {reach:e} types {y:e}"
                    );
                }
            }
        }
    }

    /// A computed length is within a twentieth of the tolerance of its
    /// number in every unit (or as near as its floats allow), and is
    /// written no finer than that unit's power of ten at a tenth of the
    /// tolerance unless four figures need it; an angle is four figures
    /// of the unit it is told in.
    #[test]
    fn the_grid_is_the_same_length_in_every_unit() {
        for x in magnitudes(32) {
            for notation in NOTATIONS {
                let text = Told::computed(&x, Dimension::Length)
                    .told_in(notation)
                    .to_string();
                let (_, unit) = said(&text);
                let y = typed_back(&text)[0];
                assert!(y != 0.0, "{x:e} reads {text}");
                let twentieth = 1e-10 / 2.0 * (1.0 + 1e-9);
                assert!(
                    (y - x).abs() <= twentieth.max(5.0 * spacing(x)) + 2.0 * spacing(x),
                    "{x:e} reads {text}"
                );
                let number = number(&text);
                let past_the_grid = |q: Option<f64>| q.is_none_or(|q| q.abs() > LAST_GRID_POINT);
                assert!(
                    figures(&text) <= 4
                        || last_digit_step(number) >= tolerance_step(unit) * (1.0 - 1e-9)
                        || past_the_grid(unit.number_of(x)),
                    "{x:e} reads {text}, finer than a tenth of the tolerance"
                );
                let text = Told::computed(&x, Dimension::Angle)
                    .told_in(notation)
                    .to_string();
                let (numbers, unit) = said(&text);
                let q = unit.number_of(x);
                assert!(
                    past_the_grid(q) || figures(&text) <= 4,
                    "{x:e} reads {text}"
                );
                if let Some(q) = q.filter(|q| q.is_finite()) {
                    let reach = if past_the_grid(Some(q)) { 1.0 } else { 0.5 };
                    assert!(
                        (float(&numbers[0]) - q).abs() <= step(q, 4) * reach
                            || typed_back(&text)[0] == x,
                        "{x:e} reads {text}"
                    );
                }
            }
        }
    }

    /// An enclosure's ends, typed back in the unit they are told in,
    /// contain its ends.
    #[test]
    fn an_enclosure_typed_back_contains_its_ends_in_every_notation() {
        for x in magnitudes(128).into_iter().step_by(8) {
            for width in [f64::EPSILON, 1e-9, 1e-3, 1.0] {
                let (a, b) = (x, x + x.abs() * width);
                let (lo, hi) = (a.min(b), a.max(b));
                if lo == hi || !hi.is_finite() {
                    continue;
                }
                let enclosure = Interval::from_bounds(lo, hi);
                for notation in NOTATIONS {
                    for dimension in DIMENSIONS {
                        let text = Told::computed(&enclosure, dimension)
                            .told_in(notation)
                            .to_string();
                        let (numbers, unit) = said(&text);
                        let ends = typed_back(&text);
                        assert!(ends[0] <= lo, "[{lo:e}, {hi:e}] reads {text}");
                        assert!(ends[1] >= hi, "[{lo:e}, {hi:e}] reads {text}");
                        let (Some(q_lo), Some(q_hi)) = (unit.number_of(lo), unit.number_of(hi))
                        else {
                            continue;
                        };
                        if !(q_lo.is_finite() && q_hi.is_finite()) {
                            continue;
                        }
                        let slack = |q: f64| 2.0 * spacing(q);
                        let (told_lo, told_hi) = (float(&numbers[0]), float(&numbers[1]));
                        // Within a step of the grid, or where the floats are
                        // sparser than the grid, the end as received.
                        assert!(
                            q_lo - told_lo <= step(q_lo, 4) + slack(q_lo) || ends[0] == lo,
                            "{text}"
                        );
                        assert!(
                            told_hi - q_hi <= step(q_hi, 4) + slack(q_hi) || ends[1] == hi,
                            "{text}"
                        );
                    }
                }
            }
        }
    }

    /// Two compared numbers read in one unit and in their true order
    /// in every notation and every role, down to adjacent floats.
    #[test]
    fn compared_numbers_read_in_their_true_order_in_every_notation() {
        type Mk = fn(&f64, Dimension) -> Told;
        let roles: [Mk; 4] = [Told::given, Told::computed, Told::at_least, Told::at_most];
        for x in magnitudes(128).into_iter().step_by(48) {
            for y in [x.next_up(), x * 1.000_04, x * (1.0 - 1e-12), -x, 0.0, x] {
                if !y.is_finite() {
                    continue;
                }
                for notation in NOTATIONS {
                    for dimension in DIMENSIONS {
                        for a in roles {
                            for b in roles {
                                let pair = a(&x, dimension).beside(&b(&y, dimension));
                                let first = pair.first().told_in(notation).to_string();
                                let second = pair.second().told_in(notation).to_string();
                                let ((_, unit), (_, other)) = (said(&first), said(&second));
                                assert_eq!(unit, other, "{first} beside {second}");
                                assert_eq!(unit, dimension.unit(notation), "{first}");
                                assert_eq!(
                                    typed_back(&first)[0].partial_cmp(&typed_back(&second)[0]),
                                    x.partial_cmp(&y),
                                    "{x:e} beside {y:e} reads {first} and {second}"
                                );
                            }
                        }
                    }
                }
            }
        }
        let by_size = Told::computed(&-1.000_01e-9, Dimension::Length)
            .beside_by_magnitude(&Told::computed(&1.000_04e-9, Dimension::Length));
        assert_eq!(
            by_size.first().told_in(NOTATIONS[1]).to_string(),
            "-1.00001e-6 mm"
        );
        assert_eq!(
            by_size.second().told_in(NOTATIONS[1]).to_string(),
            "1.00004e-6 mm"
        );
    }

    /// A value whose number of a unit is past the format of a float is
    /// told in that unit all the same, alone, in a pair and in a
    /// position.
    #[test]
    fn a_unit_says_a_value_its_number_has_no_float_for() {
        let (mm_deg, in_pi) = (NOTATIONS[1], NOTATIONS[2]);
        let huge = Told::computed(&1e306, Dimension::Length);
        assert_eq!(huge.told_in(mm_deg).to_string(), "1e309 mm");
        assert_eq!(
            Told::at_least(&1e306, Dimension::Length)
                .told_in(mm_deg)
                .to_string(),
            "1e309 mm"
        );
        let least = f64::from_bits(1);
        let text = Told::computed(&least, Dimension::Angle)
            .told_in(in_pi)
            .to_string();
        assert_eq!(typed_back(&text), [least], "{text}");
        assert!(text.ends_with(" pi rad"), "{text}");
        let small = Told::computed(&0.25, Dimension::Length);
        let pair = small.beside(&huge);
        assert_eq!(pair.first().told_in(mm_deg).to_string(), "250 mm");
        assert_eq!(pair.second().told_in(mm_deg).to_string(), "1e309 mm");
        assert_eq!(
            Told::position(&small, &huge).told_in(mm_deg).to_string(),
            "(250, 1e309) mm"
        );
    }

    /// A pair of two dimensions is a builder's bug, and panics where the
    /// pair is made, which is where a sentence tells it.
    #[test]
    #[should_panic = "a compared pair is of one dimension"]
    fn a_pair_of_two_dimensions_panics() {
        let _ = Told::computed(&0.25, Dimension::Length)
            .beside(&Told::computed(&1.0, Dimension::Angle));
    }

    /// So is a position whose coordinates are of two dimensions.
    #[test]
    #[should_panic = "a position's coordinates are told in one unit"]
    fn a_position_of_two_dimensions_panics() {
        let _ = Told::position(
            &Told::computed(&0.25, Dimension::Length),
            &Told::computed(&1.0, Dimension::Angle),
        );
    }

    /// The gate's sample of the every-notation sweeps (which are in the
    /// slow set): a handful of values in two notations that are not
    /// the canonical one, in every role, and a pair of each pairing of
    /// roles. A given number types back as itself, an offer on its
    /// side, an enclosure around its ends, and a pair in its true
    /// order, each in the notation's own unit.
    #[test]
    fn a_handful_of_quantities_reads_true_in_two_notations() {
        type Mk = fn(&f64, Dimension) -> Told;
        let roles: [Mk; 4] = [Told::given, Told::computed, Told::at_least, Told::at_most];
        let values = [0.300_04, -1.0 / 3.0, 1e-7, 2.5e6, f64::from_bits(1), 1e306];
        for notation in [NOTATIONS[1], NOTATIONS[2]] {
            for dimension in [Dimension::Length, Dimension::Angle] {
                let unit = dimension.unit(notation);
                let in_unit = |text: &str| {
                    assert_eq!(said(text).1, unit, "{text}");
                    typed_back(text)
                };
                for &x in &values {
                    let told = |make: Mk| make(&x, dimension).told_in(notation).to_string();
                    assert_eq!(
                        in_unit(&told(Told::given))[0].to_bits(),
                        x.to_bits(),
                        "{x:e}"
                    );
                    assert!(in_unit(&told(Told::at_least))[0] >= x, "{x:e}");
                    assert!(in_unit(&told(Told::at_most))[0] <= x, "{x:e}");
                    // An enclosure a thousandth wide, or a float where
                    // that is narrower.
                    let far = (x + x.abs() * 1e-3).max(x.next_up());
                    let wide = Interval::from_bounds(x, far);
                    let ends = in_unit(
                        &Told::computed(&wide, dimension)
                            .told_in(notation)
                            .to_string(),
                    );
                    assert!(ends[0] <= x && ends[1] >= far, "{x:e}");
                    for y in [x.next_up(), x * 1.000_04, -x] {
                        for a in roles {
                            for b in roles {
                                let pair = a(&x, dimension).beside(&b(&y, dimension));
                                let first = in_unit(&pair.first().told_in(notation).to_string());
                                let second = in_unit(&pair.second().told_in(notation).to_string());
                                assert_eq!(
                                    first[0].partial_cmp(&second[0]),
                                    x.partial_cmp(&y),
                                    "{x:e} beside {y:e}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    /// No told number is wider than [`WIDEST_NUMBER`], in any role or
    /// unit, and the bound is reached.
    #[test]
    fn one_told_number_is_never_wider_than_the_stated_bound() {
        let mut widest = 0;
        for x in magnitudes(64) {
            for notation in NOTATIONS {
                for dimension in DIMENSIONS {
                    let pair = Told::computed(&x, dimension)
                        .beside(&Told::computed(&x.next_up(), dimension));
                    for text in [
                        Told::given(&x, dimension).told_in(notation).to_string(),
                        Told::computed(&x, dimension).told_in(notation).to_string(),
                        Told::at_least(&x, dimension).told_in(notation).to_string(),
                        Told::at_most(&x, dimension).told_in(notation).to_string(),
                        pair.first().told_in(notation).to_string(),
                    ] {
                        let width = number(&text).chars().count();
                        assert!(width <= WIDEST_NUMBER, "{text} is {width} characters");
                        widest = widest.max(width);
                    }
                }
            }
        }
        assert_eq!(widest, WIDEST_NUMBER);
    }
}

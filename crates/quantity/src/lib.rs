//! Typed lengths, angles and counts for the API boundary.
//!
//! You build a quantity by multiplying a number by a unit, and you
//! get a plain `f64` back at the accessor. Units exist at the
//! boundary and nowhere else — the kernel only ever sees canonical
//! metres and radians.
//!
//! ```
//! use quantity::{Angle, DEG, Length, MM, IN};
//!
//! let thickness = 25.0 * MM;
//! assert_eq!(thickness.meters(), 0.025);
//! assert_eq!(thickness.in_unit(MM), 25.0);
//!
//! // Same-dimension arithmetic only; mixing dimensions is a type
//! // error at compile time, not a runtime surprise.
//! let doubled: Length = thickness + thickness;
//! assert_eq!(doubled.meters(), 0.05);
//!
//! let quarter: Angle = 90.0 * DEG;
//! assert!((quarter.radians() - core::f64::consts::FRAC_PI_2).abs() < 1e-15);
//!
//! // The inch is exact by definition, not a rounded decimal.
//! assert_eq!((1.0 * IN).meters(), 0.0254);
//! ```
//!
//! In Python the same layer is spelled `25 * mm` and `90 * deg`, and
//! mixing dimensions there raises a typed `DimensionError` because
//! Python has no compile step to catch it. The guide's quickstart
//! (`docs/GUIDE.md`) shows both.
//!
//! # Design notes
//!
//! The D6 API-boundary quantity layer (LIB-U8a): hand-rolled quantity
//! newtypes, the unit table, and the display formatter.
//!
//! Spec D6: kernel-internal code is raw `T` in fixed canonical units
//! (meters/radians, no dimensional types inside); the PUBLIC API uses
//! hand-rolled newtypes that convert on entry — hand-rolled rather
//! than `uom` because the closed set is ~three quantities, not the SI
//! lattice. [`Length`] and [`Angle`] wrap canonical `f64`
//! meters/radians; [`Count`] wraps an exact `i64`. Values are built
//! from unit constants (`25.0 * MM`), and units erase at the accessor
//! doors ([`Length::meters`], [`Angle::radians`]) — the same GQ5
//! erasure boundary the expression sublanguage has.
//!
//! NAME NOTE: `geom_core::predicate::Margin<T>` (renamed from
//! `Length<T>`, LB9) is the kernel-internal classify-seam margin type
//! (D4's margin convention, no unit algebra) — a DIFFERENT thing from
//! this crate's [`Length`], which is D6's API-boundary quantity. The
//! pncad prelude exports THIS type and has never exported the seam
//! type, which no library user touches.
//!
//! The unit table is data ([`UNITS`]), shaped after the one prior unit
//! table in the tree (step-import's `units.rs`): symbol + measured
//! quantity + exact factor into canonical units. Lengths take SI
//! prefixes ([`MILLI`], [`CENTI`] applied to the metre); angles and
//! the inch do not — the census's lengths-take-prefixes rule, mirrored
//! from step-import's `si_unit_kind` (prefixed angles refuse there).
//!
//! Finiteness: the newtypes are plain value wrappers and do not refuse
//! non-finite floats themselves; the fail-loud doors are where values
//! enter recipe data or the kernel (`Expr::literal`'s ruled door 1),
//! exactly as for raw `f64` authoring today.

mod fmt;
mod units;

pub use fmt::{FmtQuantityError, fmt_angle, fmt_length};
pub use units::{
    AngleUnit, CENTI, CM, DEG, IN, LengthUnit, M, MILLI, MM, RAD, UNITS, UnitDef, UnitQuantity,
    unit_by_symbol,
};

use core::ops::{Add, Div, Mul, Neg, Sub};

/// A physical length at the API boundary — canonical METERS inside
/// (D6). Build one from a unit constant: `25.0 * MM` (or `MM * 25.0`).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Length(f64);

/// A plane angle at the API boundary — canonical RADIANS inside (D6).
/// Build one from a unit constant: `90.0 * DEG`.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f64);

/// An exact integer count at the API boundary (structural: pattern
/// counts, indices). No unit constants — a count is a bare integer;
/// promotion to a continuous value is explicit everywhere (spec D4).
///
/// Deliberately NO arithmetic impls on this newtype: D4's checked
/// count algebra (closed add/sub/mul/neg/min/max with typed i64
/// overflow refusal) lives in the `Expr` layer, and duplicating it
/// here unchecked would undercut it. This type carries a value across
/// the boundary; arithmetic happens where the checks are.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Count(i64);

impl Length {
    /// A length from canonical meters — the erasure door's inverse,
    /// for values that are already kernel-units data.
    pub const fn from_meters(meters: f64) -> Self {
        Self(meters)
    }

    /// The canonical value in meters — units erase here (GQ5/D6).
    pub const fn meters(self) -> f64 {
        self.0
    }

    /// The value expressed in `unit` (display-side conversion; the
    /// canonical value stays what it is).
    ///
    /// `unit`'s factor is the table's, not the caller's — [`LengthUnit`]
    /// wraps a [`UnitDef`] row (the seal, whose narrative lives there).
    pub fn in_unit(self, unit: LengthUnit) -> f64 {
        self.0 / unit.factor()
    }
}

impl Angle {
    /// An angle from canonical radians — the erasure door's inverse.
    pub const fn from_radians(radians: f64) -> Self {
        Self(radians)
    }

    /// The canonical value in radians — units erase here (GQ5/D6).
    pub const fn radians(self) -> f64 {
        self.0
    }

    /// The value expressed in `unit` (display-side conversion); the
    /// factor is the table's, as on [`Length::in_unit`].
    pub fn in_unit(self, unit: AngleUnit) -> f64 {
        self.0 / unit.factor()
    }
}

impl Count {
    /// A count from an exact integer.
    pub const fn new(value: i64) -> Self {
        Self(value)
    }

    /// The exact integer value.
    pub const fn get(self) -> i64 {
        self.0
    }
}

/// `25.0 * MM` — the primary construction spelling (L4: `25 * mm`).
///
/// This is the D6 typed-units boundary itself: the factor applied is
/// the one the unit table pairs with the symbol, because a
/// [`LengthUnit`] is a table row (the seal on [`UnitDef`]). A unit
/// whose factor disagreed with its symbol would make this multiply
/// produce a correct-looking `Length` of the wrong magnitude.
impl Mul<LengthUnit> for f64 {
    type Output = Length;
    fn mul(self, unit: LengthUnit) -> Length {
        Length(self * unit.factor())
    }
}

/// `MM * 25.0` — commuted spelling.
impl Mul<f64> for LengthUnit {
    type Output = Length;
    fn mul(self, value: f64) -> Length {
        Length(value * self.factor())
    }
}

/// `90.0 * DEG`.
impl Mul<AngleUnit> for f64 {
    type Output = Angle;
    fn mul(self, unit: AngleUnit) -> Angle {
        Angle(self * unit.factor())
    }
}

/// `DEG * 90.0` — commuted spelling.
impl Mul<f64> for AngleUnit {
    type Output = Angle;
    fn mul(self, value: f64) -> Angle {
        Angle(value * self.factor())
    }
}

// The closed quantity algebra the newtypes carry themselves (same-
// dimension add/sub, negation, scaling by a dimensionless f64) — the
// F1 restrictive lattice's infallible subset. Everything fallible
// (mixed dimensions, products of dimensions) has no impl to call:
// refused at the type level, which is the whole point of D6.
macro_rules! quantity_ops {
    ($ty:ident) => {
        impl Add for $ty {
            type Output = $ty;
            fn add(self, rhs: $ty) -> $ty {
                $ty(self.0 + rhs.0)
            }
        }
        impl Sub for $ty {
            type Output = $ty;
            fn sub(self, rhs: $ty) -> $ty {
                $ty(self.0 - rhs.0)
            }
        }
        impl Neg for $ty {
            type Output = $ty;
            fn neg(self) -> $ty {
                $ty(-self.0)
            }
        }
        impl Mul<f64> for $ty {
            type Output = $ty;
            fn mul(self, rhs: f64) -> $ty {
                $ty(self.0 * rhs)
            }
        }
        impl Mul<$ty> for f64 {
            type Output = $ty;
            fn mul(self, rhs: $ty) -> $ty {
                $ty(self * rhs.0)
            }
        }
        impl Div<f64> for $ty {
            type Output = $ty;
            fn div(self, rhs: f64) -> $ty {
                $ty(self.0 / rhs)
            }
        }
    };
}
quantity_ops!(Length);
quantity_ops!(Angle);

#[cfg(test)]
mod tests;

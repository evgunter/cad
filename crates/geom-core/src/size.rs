//! **A size is a type** (DESIGN `[sizes]`): a length the kernel stores
//! as a size is a [`Size`], finite and strictly positive, with one
//! fallible mint.
//!
//! The mint's predicate is two exact reads of the representation, in
//! this order:
//!
//! - **Bounded** ([`Real::is_bounded`]): every value the scalar carries
//!   is a finite number. At `Interval` an enclosure with an end at `±∞`
//!   is unbounded even where its width is not poison (`[MAX, ∞]` from
//!   an overflow), which [`crate::is_finite_length`] would wave
//!   through. Finiteness is decided here and nowhere else.
//! - **Not at or below zero at every point** ([`Real::select_le_zero`]).
//!   At `f64` that is `x > 0`, so both signed zeros refuse. At
//!   `Interval` it is `hi > 0`: an enclosure that straddles zero is a
//!   size, and `[0, 0]` and `[−1, 0]` are not.
//!
//! Neither read is a band decision. Whether a computed size exists is
//! its producer's question, decided before it mints; the mint refuses
//! only what no size is. A refusal is typed, and the mint never panics.
//!
//! This is not [`quantity::Length`], the user-facing length, which
//! `[sizes]` keeps apart.
//!
//! No path builds a `Size` except [`Size::new`] and the constants it
//! mints. A struct literal does not compile from outside the module:
//!
//! ```compile_fail,E0423
//! let _ = geom_core::Size(1.0_f64);
//! ```
//!
//! nor does a conversion or a default:
//!
//! ```compile_fail,E0277
//! let _: geom_core::Size<f64> = 1.0_f64.into();
//! ```
//!
//! ```compile_fail,E0599
//! let _ = geom_core::Size::<f64>::default();
//! ```

use core::fmt;

use quantity::Notation;

use crate::Real;
use crate::told::{Dimension, Tell, Told};

/// **A finite, strictly positive length**: bounded, and not at or below
/// zero at every point it carries (the module docs). Minted only by
/// [`Size::new`].
#[derive(Clone, Copy, Debug)]
pub struct Size<T>(T);

impl<T: Real> Size<T> {
    /// **The one mint**: `x` as a size, or which of the four ways it is
    /// not one.
    ///
    /// # Errors
    ///
    /// [`SizeRefusal::NaN`] where `x` is the scalar's poison;
    /// [`SizeRefusal::Unbounded`] where it carries a value that is not a
    /// finite number; [`SizeRefusal::Negative`] where it is below zero
    /// at every point; [`SizeRefusal::Zero`] where it is at or below
    /// zero at every point and reaches zero.
    pub fn new(x: T) -> Result<Self, SizeRefusal> {
        if x.is_poison() {
            return Err(SizeRefusal::NaN);
        }
        if !x.is_bounded() {
            return Err(SizeRefusal::Unbounded { size: told(x) });
        }
        if le_zero(x) == Some(true) {
            return Err(if le_zero(T::zero() - x) == Some(false) {
                SizeRefusal::Negative { size: told(x) }
            } else {
                SizeRefusal::Zero { size: told(x) }
            });
        }
        Ok(Self(x))
    }

    /// The unit size.
    ///
    /// # Panics
    ///
    /// Never: one is a size at every scalar, which the mint table's
    /// constant row pins. A refusal here is a defect in the mint.
    #[must_use]
    pub fn one() -> Self {
        #[allow(clippy::expect_used)]
        Self::new(T::one()).expect("one is a size at every scalar")
    }

    /// The length.
    #[must_use]
    pub fn get(self) -> T {
        self.0
    }
}

/// Whether `x` is at or below zero at every point it carries
/// (`Some(true)`) or above it at every point (`Some(false)`); `None`
/// where it straddles zero. Each candidate is read against the poison,
/// so a straddle, which hulls both, reads as neither side decided.
fn le_zero<T: Real>(x: T) -> Option<bool> {
    let poison = T::from_f64(f64::NAN);
    let le = !x.select_le_zero(T::zero(), poison).is_poison();
    let gt = !x.select_le_zero(poison, T::zero()).is_poison();
    match (le, gt) {
        (true, false) => Some(true),
        (false, true) => Some(false),
        _ => None,
    }
}

/// The refused value, told exactly as the mint received it.
fn told<T: Real>(x: T) -> Told {
    Told::given(&x, Dimension::Length)
}

/// **Why a value is not a [`Size`]**: which of the four failed, with
/// the value where it reads.
#[derive(Clone, Debug, PartialEq)]
pub enum SizeRefusal {
    /// At or below zero at every point, and reaching zero: `±0` at a
    /// point scalar, an enclosure whose upper end is zero.
    Zero {
        /// The value refused.
        size: Told,
    },
    /// Below zero at every point.
    Negative {
        /// The value refused.
        size: Told,
    },
    /// The scalar's poison: NaN, or an empty or ill-formed enclosure.
    NaN,
    /// Some value it carries is not a finite number.
    Unbounded {
        /// The value refused.
        size: Told,
    },
}

impl Tell for SizeRefusal {
    fn tell(&self, notation: Notation, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero { size } => write!(f, "size {} is zero", size.told_in(notation)),
            Self::Negative { size } => write!(f, "size {} is negative", size.told_in(notation)),
            Self::NaN => f.write_str("size is not a number"),
            Self::Unbounded { size } => {
                write!(f, "size {} is not a finite length", size.told_in(notation))
            }
        }
    }
}

impl fmt::Display for SizeRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.tell(Notation::CANONICAL, f)
    }
}

impl core::error::Error for SizeRefusal {}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests;

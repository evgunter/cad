//! Authored quantities: a canonical value **plus the notation it was
//! written in**.
//!
//! [`Length`] and [`Angle`] are canonical-units data with a closed
//! arithmetic (D6): `25.0 * MM` is `Length(0.025)`, and the `MM` is
//! gone the instant the multiply happens. That erasure is correct for
//! everything downstream of the boundary — the kernel only ever sees
//! metres and radians — and wrong for exactly one thing: recording
//! what a person TYPED, so a document can be read back the way it was
//! written.
//!
//! These two types are that record: the canonical value beside the
//! typed unit view it was authored in.
//!
//! # The notation is not optional
//!
//! A value authored through these doors ALWAYS names the unit it is
//! written in, [`WrittenLength::from_meters`] included — that one
//! names the metre. There is no "canonical, notation unknown" state,
//! and the point of not having one is that a document says how it is
//! written instead of leaning on whatever fallback its reader happens
//! to apply.
//!
//! # These are authoring data, not arithmetic data
//!
//! **Deliberately NO arithmetic impls**, the reason [`crate::Count`]
//! has none: there is no answer to what notation the sum of a
//! millimetre and an inch is written in, and a type that silently
//! picked one would be inventing an authored fact nobody authored.
//! Compute on the [`Length`] inside ([`WrittenLength::length`]), where
//! the algebra is closed and the units have already erased.
//!
//! # The dimension pairing is unrepresentable, not refused
//!
//! A [`WrittenLength`] holds a [`LengthUnit`], which is an INDEX into a
//! `Length` row of [`crate::UNITS`] (the #669 seal). So "a length
//! written in degrees" is not a value this type can hold and no door
//! has to refuse it — the #650 ruling ("make the illegal state
//! unrepresentable") applied one layer out from the table. What that
//! buys concretely: `Expr::literal_with_unit`'s `DisplayUnitMismatch`
//! is unreachable through these types, because the only way to build
//! one is to name a unit whose quantity already agrees.
//!
//! ```
//! use quantity::{M, MM, WrittenLength};
//!
//! // Written in millimetres: the multiply happens at the door, and
//! // the notation survives it.
//! let thickness = WrittenLength::in_unit(25.0, MM);
//! assert_eq!(thickness.meters(), 0.025);
//! assert_eq!(thickness.unit(), MM);
//!
//! // Canonical — which is to say, written in metres, said out loud.
//! let plain = WrittenLength::from_meters(0.025);
//! assert_eq!(plain.meters(), 0.025);
//! assert_eq!(plain.unit(), M);
//! assert_eq!(plain, WrittenLength::in_unit(0.025, M));
//! ```

use crate::{Angle, AngleUnit, Length, LengthUnit, M, RAD};

/// A length as it was authored: canonical metres, plus the notation it
/// was written in. See the module docs for why the notation is not
/// optional and why this type has no arithmetic.
///
/// Equality compares BOTH halves — unlike the stored literal this feeds,
/// where the display unit is presentation metadata excluded from
/// expression identity. The difference is the point: here the notation
/// is the payload, so the same magnitude authored in two units is two
/// authorings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrittenLength {
    canonical: Length,
    unit: LengthUnit,
}

/// An angle as it was authored — [`WrittenLength`]'s mirror, canonical
/// radians plus its notation. Everything that type's docs say holds
/// here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrittenAngle {
    canonical: Angle,
    unit: AngleUnit,
}

impl WrittenLength {
    /// `value` written in `unit` — the inverse of [`Length::in_unit`],
    /// and the one door here that MULTIPLIES. The factor applied is the
    /// table's, because a [`LengthUnit`] is a table row.
    ///
    /// This is the library authoring spelling:
    /// `WrittenLength::in_unit(25.0, MM)` is `25.0 * MM` that remembers
    /// the `MM`.
    pub fn in_unit(value: f64, unit: LengthUnit) -> Self {
        Self {
            canonical: value * unit,
            unit,
        }
    }

    /// Canonical meters, written in meters — the plain spelling, and
    /// the same value as `in_unit(meters, M)` without the multiply by
    /// one. Named for [`Length::from_meters`], whose job it is doing.
    pub const fn from_meters(meters: f64) -> Self {
        Self {
            canonical: Length::from_meters(meters),
            unit: M,
        }
    }

    /// An ALREADY-canonical value that remembers `unit` as its
    /// notation — no multiply, because the caller's arithmetic has
    /// happened.
    ///
    /// This is the shape a form has: a draft field holds canonical
    /// meters whatever unit is on screen (the picker re-writes the
    /// display, never the value), so the authoring op carries the
    /// draft and the picker's choice side by side rather than
    /// re-deriving one from the other.
    pub const fn canonical_in(meters: f64, unit: LengthUnit) -> Self {
        Self {
            canonical: Length::from_meters(meters),
            unit,
        }
    }

    /// The canonical value — the erasure door, past which nothing
    /// knows this was authored in anything.
    pub const fn length(self) -> Length {
        self.canonical
    }

    /// The canonical value in metres, for callers already holding raw
    /// kernel-units data ([`Self::length`] then [`Length::meters`]).
    pub const fn meters(self) -> f64 {
        self.canonical.meters()
    }

    /// The notation this was authored in.
    pub const fn unit(self) -> LengthUnit {
        self.unit
    }
}

impl WrittenAngle {
    /// `value` written in `unit` — [`WrittenLength::in_unit`]'s mirror,
    /// and the one door here that multiplies.
    pub fn in_unit(value: f64, unit: AngleUnit) -> Self {
        Self {
            canonical: value * unit,
            unit,
        }
    }

    /// Canonical radians, written in radians —
    /// [`WrittenLength::from_meters`]'s mirror.
    pub const fn from_radians(radians: f64) -> Self {
        Self {
            canonical: Angle::from_radians(radians),
            unit: RAD,
        }
    }

    /// An already-canonical angle that remembers `unit` — see
    /// [`WrittenLength::canonical_in`] for the form's shape this serves.
    pub const fn canonical_in(radians: f64, unit: AngleUnit) -> Self {
        Self {
            canonical: Angle::from_radians(radians),
            unit,
        }
    }

    /// The canonical value — the erasure door.
    pub const fn angle(self) -> Angle {
        self.canonical
    }

    /// The canonical value in radians ([`Self::angle`] then
    /// [`Angle::radians`]).
    pub const fn radians(self) -> f64 {
        self.canonical.radians()
    }

    /// The notation this was authored in.
    pub const fn unit(self) -> AngleUnit {
        self.unit
    }
}

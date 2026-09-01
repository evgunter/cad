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
//! These two types are that record. Each carries the canonical value
//! and an `Option` of the typed unit view it was authored in, which is
//! precisely the pair a stored literal holds
//! (`editor_core::Expr::literal_with_unit` and its `None` twin).
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
//! # `None` is a state, not a default
//!
//! `None` means **no authored notation** — "render this however the
//! reader renders unmarked values" — and it is not the same as
//! `Some(M)` or `Some(RAD)`. For lengths the two happen to coincide,
//! because the canonical fallback IS the metre at factor 1. For angles
//! they do not: an editor whose unmarked fallback is the half-turn
//! shows `None` as `pi rad` and `Some(RAD)` as `rad`. Collapsing the
//! `Option` would merge two states a document can tell apart, and
//! would make these types unable to say what the literal they feed can
//! store.
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
//! use quantity::{MM, WrittenLength};
//!
//! // Written in millimetres: the multiply happens at the door, and
//! // the notation survives it.
//! let thickness = WrittenLength::in_unit(25.0, MM);
//! assert_eq!(thickness.length().meters(), 0.025);
//! assert_eq!(thickness.unit(), Some(MM));
//!
//! // Canonical, with nothing authored about how to write it.
//! let plain = WrittenLength::metres(0.025);
//! assert_eq!(plain.length().meters(), 0.025);
//! assert_eq!(plain.unit(), None);
//! ```

use crate::{Angle, AngleUnit, Length, LengthUnit};

/// A length as it was authored: canonical metres, plus the notation it
/// was written in (`None` for none). See the module docs for why the
/// `Option` is a state rather than a default, and why this type has no
/// arithmetic.
///
/// Equality compares BOTH halves — unlike the stored literal this feeds,
/// where the display unit is presentation metadata excluded from
/// expression identity. The difference is the point: here the notation
/// is the payload, so two authorings of the same magnitude in different
/// units are different authorings.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrittenLength {
    canonical: Length,
    unit: Option<LengthUnit>,
}

/// An angle as it was authored — [`WrittenLength`]'s mirror, canonical
/// radians plus its notation. Everything that type's docs say holds
/// here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WrittenAngle {
    canonical: Angle,
    unit: Option<AngleUnit>,
}

impl WrittenLength {
    /// `value` written in `unit` — the inverse of [`Length::in_unit`],
    /// and the one door here that MULTIPLIES. The factor applied is the
    /// table's, because a [`LengthUnit`] is a table row.
    ///
    /// This is the library authoring spelling: `WrittenLength::in_unit(25.0, MM)`
    /// is `25.0 * MM` that remembers the `MM`.
    pub fn in_unit(value: f64, unit: LengthUnit) -> Self {
        Self {
            canonical: value * unit,
            unit: Some(unit),
        }
    }

    /// Canonical metres with **no** authored notation.
    pub const fn metres(metres: f64) -> Self {
        Self {
            canonical: Length::from_meters(metres),
            unit: None,
        }
    }

    /// An ALREADY-canonical value that remembers `unit` as its
    /// notation — no multiply, because the caller's arithmetic has
    /// happened.
    ///
    /// This is the shape a form has: a draft field holds canonical
    /// metres whatever unit is on screen (the picker re-writes the
    /// display, never the value), so the authoring op carries the
    /// draft and the picker's choice side by side rather than
    /// re-deriving one from the other.
    pub const fn metres_in(metres: f64, unit: Option<LengthUnit>) -> Self {
        Self {
            canonical: Length::from_meters(metres),
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

    /// The notation this was authored in, if any.
    pub const fn unit(self) -> Option<LengthUnit> {
        self.unit
    }
}

impl WrittenAngle {
    /// `value` written in `unit` — [`WrittenLength::in_unit`]'s mirror,
    /// and the one door here that multiplies.
    pub fn in_unit(value: f64, unit: AngleUnit) -> Self {
        Self {
            canonical: value * unit,
            unit: Some(unit),
        }
    }

    /// Canonical radians with **no** authored notation.
    pub const fn radians(radians: f64) -> Self {
        Self {
            canonical: Angle::from_radians(radians),
            unit: None,
        }
    }

    /// An already-canonical angle that remembers `unit` — see
    /// [`WrittenLength::metres_in`] for the form's shape this serves.
    pub const fn radians_in(radians: f64, unit: Option<AngleUnit>) -> Self {
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
    pub const fn radians_value(self) -> f64 {
        self.canonical.radians()
    }

    /// The notation this was authored in, if any.
    pub const fn unit(self) -> Option<AngleUnit> {
        self.unit
    }
}

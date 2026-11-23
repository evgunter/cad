//! The notation a person reads and writes in: one length unit and one
//! angle unit.

use crate::units::{AngleUnit, LengthUnit, M, ONE, PI, RAD, UnitDef, UnitQuantity};

/// **A notation**: the length unit and the angle unit a number is read
/// and written in by one person. A value somebody wrote remembers its
/// own unit ([`crate::WrittenLength`], [`crate::WrittenAngle`]); a
/// value nobody wrote (a computed slot, a measure, a number a refusal
/// quotes) reads in the notation of whoever is reading it.
///
/// It is held per person and never enters a document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Notation {
    /// Every length reads and is written in this.
    pub length: LengthUnit,
    /// Every angle reads and is written in this.
    pub angle: AngleUnit,
}

impl Notation {
    /// The notation a person who has chosen none works in: metres and
    /// half turns (`pi rad`).
    pub const DEFAULT: Self = Self {
        length: M,
        angle: PI,
    };

    /// The canonical units, metres and radians: the notation of text
    /// with no reader in particular (a log, a `Display`).
    pub const CANONICAL: Self = Self {
        length: M,
        angle: RAD,
    };

    /// The unit this notation writes `quantity` in — [`Self::with`]'s
    /// reading half: the dimensionless row for a bare scalar, which no
    /// notation chooses.
    #[must_use]
    pub const fn unit_of(self, quantity: UnitQuantity) -> UnitDef {
        match quantity {
            UnitQuantity::Length => self.length.def(),
            UnitQuantity::Angle => self.angle.def(),
            UnitQuantity::Scalar => ONE.def(),
        }
    }

    /// This notation with `unit` in the place of its own quantity's
    /// unit; the dimensionless row changes nothing.
    #[must_use]
    pub const fn with(self, unit: UnitDef) -> Self {
        match unit.quantity() {
            UnitQuantity::Length => match unit.as_length() {
                Some(length) => Self { length, ..self },
                None => self,
            },
            UnitQuantity::Angle => match unit.as_angle() {
                Some(angle) => Self { angle, ..self },
                None => self,
            },
            UnitQuantity::Scalar => self,
        }
    }
}

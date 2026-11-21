//! The quantity display formatter — `fmt_real`'s DISPLAY sibling
//! (step-export's `fmt_real` is a bit-exact Part 21 printer with no
//! unit vocabulary; this one renders a canonical-units value IN a
//! display unit, with the unit suffix, for humans and for the text
//! parser to read back).
//!
//! THE PIN: `parse(fmt(x, unit))` recovers `x` BIT-EXACTLY, where
//! `parse` is the expression text parser's literal semantics — the
//! decimal times the unit's exact factor, rounded once
//! ([`crate::UnitDef::canonical`]). The formatter renders the shortest
//! decimal that reads back so ([`crate::UnitDef::shortest`]), in the
//! unit asked for: every unit says every finite value.
//!
//! NaN/±∞ refuse typed ([`FmtQuantityError::NonFinite`]) exactly as
//! `fmt_real` refuses: poison never lands in display text.

use crate::units::{AngleUnit, LengthUnit, UnitDef};

/// Typed refusal from the display formatter.
#[derive(Debug, Clone, PartialEq)]
pub enum FmtQuantityError {
    /// The value is NaN or ±∞ — non-finite quantities are poison and
    /// have no display form (fail loud, the fmt_real posture).
    NonFinite {
        /// The refused value.
        value: f64,
    },
}

// The human-readable rendering (LIB-DOORS F6 shape): the arm states
// the PROBLEM in this module's vocabulary — a value with no display
// form — and points where the fix is. The recourse is upstream by
// construction: nothing this formatter could be asked differently
// turns poison into text, so the sentence sends the caller to the
// operation that produced the value, as the predicate layer's
// poisoned-margin sentence does.
impl core::fmt::Display for FmtQuantityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NonFinite { value } => write!(
                f,
                "quantity display: {value} has no display form — a non-finite \
                 quantity is poison and never lands in display text; check the \
                 operation's inputs upstream"
            ),
        }
    }
}

impl core::error::Error for FmtQuantityError {}

/// Formats a canonical-METERS value in `unit` (module docs; the pin:
/// the text parses back to `meters`' exact bits).
///
/// `unit` is not checked here and does not need to be: a
/// [`crate::LengthUnit`] is an INDEX into [`crate::UNITS`] (the seal,
/// whose narrative lives on [`crate::UnitDef`]), so its symbol and its
/// factor are one row's and the rendered suffix names the factor that
/// was applied. That is what makes this module's `parse(fmt(x, unit))`
/// pin a statement about the whole surface rather than about
/// well-behaved callers.
///
/// # Errors
///
/// [`FmtQuantityError::NonFinite`] when `meters` is NaN or ±∞.
pub fn fmt_length(meters: f64, unit: LengthUnit) -> Result<String, FmtQuantityError> {
    fmt_in(meters, unit.def())
}

/// Formats a canonical-RADIANS value in `unit` (module docs; the pin:
/// the text parses back to `radians`' exact bits).
///
/// `unit` needs no check, for the reason on [`fmt_length`].
///
/// # Errors
///
/// [`FmtQuantityError::NonFinite`] when `radians` is NaN or ±∞.
pub fn fmt_angle(radians: f64, unit: AngleUnit) -> Result<String, FmtQuantityError> {
    fmt_in(radians, unit.def())
}

/// The shared engine: the shortest decimal in `unit` (module docs).
fn fmt_in(value: f64, unit: UnitDef) -> Result<String, FmtQuantityError> {
    if !value.is_finite() {
        return Err(FmtQuantityError::NonFinite { value });
    }
    Ok(format!("{} {}", unit.shortest(value), unit.symbol()))
}

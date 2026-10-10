//! The upper end of a reporting margin, for the rows that pin a limb
//! refusal's number against the displacement they planted.
//!
//! A limb refusal carries its number once, as the classifier saw it
//! ([`MarginDiag`]): a point at `f64`, an enclosure at `Interval`. The
//! rows ask whether the refusal's bound dominates a planted miss, which
//! is the enclosure's upper end, so every suite reading it reads it here
//! rather than each matching the two shapes its own way.

use geom_core::{ErrorTextReading, MarginDiag};

/// The point margin, or the enclosure's upper end; `NaN` for a poisoned
/// reading, which no bound comparison passes.
pub(crate) fn upper(margin: MarginDiag) -> f64 {
    match margin.diagnostic_f64_for_error_text() {
        ErrorTextReading::Value(m) | ErrorTextReading::Enclosure { hi: m, .. } => m,
        ErrorTextReading::Invalid => f64::NAN,
    }
}

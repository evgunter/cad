//! Test vocabulary over this crate's types, for the suites downstream.

use crate::{ErrorTextReading, MarginDiag};

/// The upper end of a reporting margin: the point margin, or the
/// enclosure's upper end; `NaN` for a poisoned reading, which no bound
/// comparison passes. The rows that pin a refusal's bound against a
/// planted miss read it here.
#[must_use]
pub fn upper(margin: MarginDiag) -> f64 {
    match margin.diagnostic_f64_for_error_text() {
        ErrorTextReading::Value(m) | ErrorTextReading::Enclosure { hi: m, .. } => m,
        ErrorTextReading::Invalid => f64::NAN,
    }
}

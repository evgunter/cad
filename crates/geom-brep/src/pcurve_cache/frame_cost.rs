//! **The refusals at ε 1e-12 the carrier frames cost** (PR 7131, Ev
//! approved 2026-11-20), as the pcurve rows pin them. Two causes, each
//! named by the row that pins it:
//!
//! - **polar**: a stored frame read as its polar enclosure (decision 1,
//!   #7131). A frame stored at `f64` a rounding off rigid reads at
//!   `Interval` about an ulp wide per entry, and an envelope levered by
//!   the span carries that past the 1e-12 band. Mutation A (the read
//!   answers the stored entries as points) certifies every such row.
//! - **bracket**: a frame built at `Interval` from bracket inputs, a
//!   rigid-motion claim (fork 274). Mutation F1 on the bracket path
//!   (store the stated vectors raw) certifies every such row.
//!
//! Each row keeps its certificate at 1e-9 and 1e-6.

use geom_core::{ErrorTextReading, Tol};

use super::PcurveCertifyError;

/// Whether the witness band is the least the rows pin, ε ≤ 1e-12.
pub(super) fn at_the_least_band() -> bool {
    Tol::witness().eps() <= 1e-12
}

/// Whether `error` is the sampled envelope's escalation
/// (`pcurve_envelope`, on the envelope or on one of its terms) with its
/// margin's upper end in `(lo, hi]`.
pub(super) fn envelope_escalates(error: &PcurveCertifyError, (lo, hi): (f64, f64)) -> bool {
    match error {
        PcurveCertifyError::Escalated {
            check: super::PcurveCheck::Envelope | super::PcurveCheck::EnvelopeTerm(_),
            cause,
            ..
        } if cause.predicate == Some("pcurve_envelope") => {
            let m = match cause.margin.diagnostic_f64_for_error_text() {
                ErrorTextReading::Value(m) => m,
                ErrorTextReading::Enclosure { hi, .. } => hi,
                ErrorTextReading::Invalid => f64::NAN,
            };
            m > lo && m <= hi
        }
        _ => false,
    }
}

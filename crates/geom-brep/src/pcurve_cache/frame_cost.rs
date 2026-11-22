//! **The refusals at ε 1e-12 the carrier frames cost** (PR 7131), as the
//! pcurve rows pin them: their causes and mutants are
//! [`geom_core::test_support::at_the_frame_cost_band`]'s. Each row keeps
//! its certificate at 1e-9 and 1e-6.

use super::PcurveCertifyError;

/// [`geom_core::test_support::at_the_frame_cost_band`] at the witness band.
pub(super) fn at_the_least_band() -> bool {
    geom_core::test_support::at_the_frame_cost_band(geom_core::Tol::witness())
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
            let m = geom_core::test_support::upper(cause.margin);
            m > lo && m <= hi
        }
        _ => false,
    }
}

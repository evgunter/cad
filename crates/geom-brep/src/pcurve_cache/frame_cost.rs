//! **The refusals at ε 1e-12 the carrier frames cost** (PR 7131, Ev
//! approved 2026-11-20), as the pcurve rows pin them. Four causes, each
//! named by the row that pins it:
//!
//! - **polar**: a stored frame read as its polar enclosure (decision 1,
//!   #7131). A frame stored at `f64` a rounding off rigid reads at
//!   `Interval` about an ulp wide per entry, and a read levered by a
//!   far extent carries that past the 1e-12 band. Mutant **A** (the read
//!   answers the stored entries as points) removes it.
//! - **bracket**: a frame built at `Interval` from bracket inputs, a
//!   rigid-motion claim (fork 274). Squaring the brackets carries the
//!   squaring's width, which main's raw, unsquared frame did not.
//!   Mutant **F1X** (that squaring, at an `Exact` kind off the
//!   stated-point path, stores the stated vectors raw) removes it.
//! - **f64 squaring**: a frame squared at `f64` from stated directions, a
//!   rigid-motion claim (fork 274), where main stored them raw: a
//!   rounding move of the frame, read across a band's edge. Mutant
//!   **F1F** (squaring at an `Inexact` kind stores the stated vectors
//!   raw) removes it.
//! - **both, jointly**: polar and bracket, each alone under the band, so
//!   A alone and F1X alone each remove it.
//!
//! The mutants are a patch in PR 7131's body; each row names its cause
//! and the mutants that turn its pin red.
//!
//! Each row keeps its certificate at 1e-9 and 1e-6.

use super::PcurveCertifyError;

/// Whether the witness band is the one the cost rows pin.
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

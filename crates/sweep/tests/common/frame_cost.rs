//! **The refusals at ε 1e-12 the carrier frames cost** (PR 7131, Ev
//! approved 2026-11-20). Four causes, each named by the row that pins it:
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
//! Each row keeps its build at 1e-9 and 1e-6. What a row checks of
//! the refusal lives here so each spells it once.

use geom_brep::props::PropsError;
use geom_core::Indeterminate;
use topo::{MassPropsError, ValidationError};

pub use geom_core::test_support::at_the_frame_cost_band as at_the_least_band;

/// Whether `cause` escalated `predicate` with its margin's upper end in
/// `(lo, hi]`.
#[must_use]
pub fn escalated(cause: &Indeterminate, predicate: &str, (lo, hi): (f64, f64)) -> bool {
    let m = geom_core::test_support::upper(cause.margin);
    cause.predicate == Some(predicate) && m > lo && m <= hi
}

/// Whether every finding is a face's volume escalating `predicate` with
/// its margin in `(lo, hi]`, and there is at least one.
#[must_use]
pub fn volume_escalates(findings: &[ValidationError], predicate: &str, band: (f64, f64)) -> bool {
    !findings.is_empty()
        && findings.iter().all(|f| {
            matches!(
                f,
                ValidationError::VolumeUncomputable {
                    source: MassPropsError::Face {
                        source: PropsError::Escalated { cause, .. },
                        ..
                    },
                    ..
                } if escalated(cause, predicate, band)
            )
        })
}

/// Whether a pcurve pass refused on one half-edge's certification
/// escalating `predicate` with its margin in `(lo, hi]`.
#[must_use]
pub fn pcurve_escalates(error: &topo::PcurveMintError, predicate: &str, band: (f64, f64)) -> bool {
    matches!(
        error,
        topo::PcurveMintError::Certify {
            error: geom_brep::PcurveCertifyError::Escalated { cause, .. },
            ..
        } if escalated(cause, predicate, band)
    )
}

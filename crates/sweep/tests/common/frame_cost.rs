//! **The refusals at ε 1e-12 the carrier frames cost** (PR 7131): their
//! causes and mutants are
//! [`geom_core::test_support::at_the_frame_cost_band`]'s. Each row keeps
//! its build at 1e-9 and 1e-6. What a row checks of the refusal lives
//! here so each spells it once.

use geom_brep::props::PropsError;
use geom_core::Indeterminate;
use topo::{MassPropsError, ValidationError};

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

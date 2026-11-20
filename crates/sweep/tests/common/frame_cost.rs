//! **The refusals at ε 1e-12 the carrier frames cost** (PR 7131, Ev
//! approved 2026-11-20). Two causes, each named by the row that pins it:
//!
//! - **polar**: a stored frame read as its polar enclosure (decision 1,
//!   #7131). A frame stored at `f64` a rounding off rigid reads at
//!   `Interval` about an ulp wide per entry, and a read levered by a
//!   far extent carries that past the 1e-12 band. Mutation A (the read
//!   answers the stored entries as points) builds every such row.
//! - **bracket**: a frame built at `Interval` from bracket inputs, a
//!   rigid-motion claim (fork 274). Squaring the brackets carries the
//!   squaring's width, which main's raw, unsquared frame did not.
//!   Mutation F1 on the bracket path (store the stated vectors raw)
//!   builds every such row.
//!
//! Each row keeps its build at 1e-9 and 1e-6. What a row checks of
//! the refusal lives here so each spells it once.

use geom_brep::props::PropsError;
use geom_core::{ErrorTextReading, Indeterminate, Tol};
use topo::{MassPropsError, ValidationError};

/// Whether the witness band is the least the rows pin, ε ≤ 1e-12.
#[must_use]
pub fn at_the_least_band() -> bool {
    Tol::witness().eps() <= 1e-12
}

/// The upper end of an escalation's margin, as its error text reads it.
#[must_use]
pub fn margin_hi(cause: &Indeterminate) -> f64 {
    match cause.margin.diagnostic_f64_for_error_text() {
        ErrorTextReading::Value(m) => m,
        ErrorTextReading::Enclosure { hi, .. } => hi,
        ErrorTextReading::Invalid => f64::NAN,
    }
}

/// Whether `cause` escalated `predicate` with its margin's upper end in
/// `(lo, hi]`.
#[must_use]
pub fn escalated(cause: &Indeterminate, predicate: &str, (lo, hi): (f64, f64)) -> bool {
    let m = margin_hi(cause);
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

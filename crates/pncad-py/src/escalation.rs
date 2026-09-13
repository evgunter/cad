//! The classifier's escalation, projected once for the two doors that
//! publish it.
//!
//! A predicate that could not certify a sign refuses with an
//! [`Indeterminate`]: the margin it saw, the band it was classified
//! against, and the predicate's own name. Two doors on this surface
//! carry one — the frame constructors' degenerate arm
//! (`FrameError::Degenerate`) and the mate solve's escalation
//! (`MateFault::Indeterminate`, and the frame refusal a mate's datum
//! wraps) — and both publish it under the SAME attribute words.
//!
//! One projection rather than one per door: the shape of the margin
//! is a fork (a value, an enclosure's two bounds, or a poisoned
//! margin that is no number at all) and two doors forking separately
//! is two spellings of one fact that can drift. This is the fork,
//! written once.
//!
//! # Why it lives outside `py`
//!
//! `crate::py` compiles only under the `python` feature, so a
//! projection sited there is one the default build cannot test. Sited
//! here both doors reach it under every feature — the
//! `crate::mate_payload` argument, one rung down.

use pncad::geom_core::{Indeterminate, MarginDiag};

/// What the classifier saw, flattened: every field present, `None`
/// where the margin's own arm does not carry one.
///
/// The three margin fields are the arms of [`MarginDiag`] and exactly
/// one of them is set at a time — a value, an enclosure's pair, or
/// none at all for a poisoned margin. Reading WHICH is not branching
/// on the margin: what the escalation contract forbids is recovering
/// the number to make the sign decision the classifier refused; what
/// the arms separate is whether there was a number at all.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Escalation {
    /// The in-band margin, when the classifier saw a value.
    pub margin: Option<f64>,
    /// The enclosure's lower bound, when it saw an enclosure.
    pub margin_low: Option<f64>,
    /// Its upper bound.
    pub margin_high: Option<f64>,
    /// The band's coincidence threshold.
    pub zero: f64,
    /// Its escalation threshold.
    pub escalate: f64,
    /// The predicate that was being decided, where the kernel
    /// attached a name.
    pub predicate: Option<&'static str>,
}

/// Project one escalation.
///
/// The match over [`MarginDiag`] is exhaustive with no wildcard: an
/// arm added kernel-side is a compile error here rather than a
/// margin that silently reaches both doors as three `None`s.
pub fn escalation(diag: &Indeterminate) -> Escalation {
    let (margin, margin_low, margin_high) = match diag.margin {
        MarginDiag::Value(m) => (Some(m), None, None),
        MarginDiag::Enclosure { lo, hi } => (None, Some(lo), Some(hi)),
        // A poisoned margin is the absence of a number, not a
        // number: the band still crosses.
        MarginDiag::Invalid => (None, None, None),
    };
    Escalation {
        margin,
        margin_low,
        margin_high,
        zero: diag.band.zero(),
        escalate: diag.band.escalate(),
        predicate: diag.predicate,
    }
}

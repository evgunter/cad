//! **An escalation whose margin the band cannot place** — the one home
//! of the `MarginKind::Invalid` diagnostic a topology door raises by
//! itself rather than receiving from `decide`.
//!
//! A door reaches for it when it has a question it must refuse and no
//! in-band margin to refuse it on. Its callers mean one of three
//! things, and each says which at its site:
//!
//! - **an impossible sign**: a nonnegative quantity (a distance, a
//!   length) classified negative, so two rows of one quantity disagree
//!   and no answer can stand on them;
//! - **a question not validly posed**: an upstream verdict that
//!   contradicts the premise the question needs (the census's material
//!   side, asked of a dihedral already decided transverse);
//! - **two sound bounds straddling the band**: a lower and an upper
//!   bound on one quantity, the one within the zero band and the other
//!   beyond the escalation band. No single margin states that, so the
//!   escalation names its own predicate (a `*_straddle` or
//!   `*_disagreement` name that never reaches the funnel) rather than
//!   the row whose margin it is not.

use geom_core::{Band, Indeterminate, MarginDiag};

/// The escalation `predicate` raises without an in-band margin — see
/// the module docs for the three things a caller may mean by it.
pub(crate) fn invalid(band: Band, predicate: &'static str) -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::INVALID,
        band,
        predicate: Some(predicate),
    }
}

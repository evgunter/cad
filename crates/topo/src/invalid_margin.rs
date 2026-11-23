//! **An escalation whose margin the band cannot place** — the one home
//! of the `MarginKind::Invalid` diagnostic a topology door raises by
//! itself rather than receiving from `decide`.
//!
//! A door reaches for it when it has a question it must refuse and no
//! in-band margin to refuse it on. Its callers mean one of two
//! things, and each says which at its site:
//!
//! - **an impossible sign**: a nonnegative quantity (a distance, a
//!   length) classified negative, so two rows of one quantity disagree
//!   and no answer can stand on them;
//! - **two sound bounds straddling the band**: a lower and an upper
//!   bound on one quantity, the one within the zero band and the other
//!   beyond the escalation band. No single margin states that, so the
//!   escalation names its own predicate rather than the row whose margin
//!   it is not. Where both bounds' readings are in hand,
//!   `geom_core::k_stats::straddle` carries their enclosure instead.

use geom_core::{Band, Indeterminate, MarginDiag};

/// The escalation `predicate` raises without an in-band margin — see
/// the module docs for the two things a caller may mean by it.
pub(crate) fn invalid(band: Band, predicate: &'static str) -> Indeterminate {
    Indeterminate {
        margin: MarginDiag::INVALID,
        band,
        predicate: Some(predicate),
        terminal_sliver: false,
    }
}

//! A request's identity across the two seams: a monotone counter and
//! nothing else.
//!
//! **This module depends on nothing** — not the kernel, not another
//! module of this crate — and that is the whole reason it is a module
//! rather than a type inside one. [`Generation`] is what both seams
//! key their answers by and what the session, the frame policies, the
//! pick index and the app all compare; sited inside either seam it
//! makes every one of those readers import a seam to name a counter,
//! and it made `evalseam` and `pickcache` import each other
//! (`crates/viewer/README.md`, *Module boundaries*).
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

/// A request's identity: a monotone counter minted by the session on
/// every submit, and compared by both seams.
///
/// Distinct from the shipped evaluation `Epoch`, which identifies the
/// RUN. This identifies the REQUEST, and the session mints a fresh one
/// for every submit — including a re-submit of an unchanged document
/// (`SessionOp::Reevaluate`). That is deliberately stricter than
/// "identifies the document version": a result may land only against
/// the request that asked for it, so a run canceled and then re-asked
/// can never have its abandoned answer accepted for the new ask.
///
/// **The counter itself is not readable.** Every consumer compares
/// generations and none displays one, so there is no accessor and no
/// `u64` constructor; `Debug` is what puts the number in a log or a
/// debugger. An accessor here would be a door with nothing on the
/// other side of it, and adding one when a display need arrives costs
/// three lines.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Generation(u64);

impl Generation {
    /// The first generation.
    pub const FIRST: Self = Self(0);

    /// The next generation after this one.
    ///
    /// Saturating, not wrapping. A wrap would make a stale result
    /// compare equal to the current request — the one thing this type
    /// exists to prevent — so the arithmetic that cannot produce that
    /// failure is the one to write. The ceiling it saturates at is not
    /// reachable: [`Generation::FIRST`] is zero and this is the only
    /// way to advance one, so getting there is 2^64 submits.
    pub fn next(self) -> Self {
        Self(self.0.saturating_add(1))
    }
}

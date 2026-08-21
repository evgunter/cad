//! **The run's ε, and where it came from** (S22, ruled 2026-08-19).
//!
//! ε is a *declared run parameter*: the model is a pure function of
//! (parameter vector, ε), one ε per process, committed once and never
//! loosened (D4 ¶1). The commitment is a `OnceLock` in
//! [`geom_core::tolerance`] — deliberately, because that lock is the
//! only thing structurally enforcing "one ε per process". Threading ε
//! to the predicate funnels was considered and **rejected**: it
//! deletes that enforcement in exchange for documentation, and the two
//! crates that already thread it (`profile`, `mesh`) measured 256 call
//! sites of which zero pass anything but the ambient value.
//!
//! What the lock owed the caller was an answer to *where the committed
//! value came from*. A stale `CAD_TOLERANCE_EPS` in a shell changes
//! what "coincident" means, and before this module nothing in a run's
//! output said so (issues #415, #497). So:
//!
//! ```
//! // At a point where the answer is interesting — after the document
//! // is loaded, or at the end of a build.
//! if let Some(report) = pncad::tolerance::committed_report() {
//!     println!("{report}");
//! }
//! ```
//!
//! which prints one line naming ε, its [`EpsilonSource`], K, and any
//! environment value that was **rejected** on the way (a rejected
//! value never reaches a predicate — the run is at the compiled
//! default, and the line says both halves).
//!
//! # Report without deciding
//!
//! [`committed_report`] is the door to reach for. It does **not**
//! commit: asking is not deciding. [`report`] does commit (exactly as
//! [`Tolerance::get`] does), so calling it at the top of a program
//! that later loads a document would bootstrap ε from the ambient
//! environment and turn that load into a `ToleranceConflict`. Reach
//! for [`report`] only where the ambient value is already the answer.
//!
//! # This decides nothing
//!
//! Provenance is a channel, not a decision. The ranking is unchanged —
//! a document's recorded ε outranks an unread `CAD_TOLERANCE_EPS`,
//! disagreement refuses at load, a malformed env value is recorded and
//! fails the suite loudly — and no kernel predicate reads
//! [`EpsilonSource`] at all.

pub use geom_core::tolerance::{
    DEFAULT_EPS, DEFAULT_K, ENV_EPS, ENV_K, EpsilonSource, Tol, Tolerance, ToleranceEnvError,
    ToleranceEnvErrorKind, ToleranceError, ToleranceReport,
};

/// The run's committed ε, its provenance, and K — or `None` if nothing
/// has committed an ε yet.
///
/// Never commits: safe to call anywhere, including before a document
/// load. See the [module docs](self).
pub fn committed_report() -> Option<ToleranceReport> {
    Tolerance::committed_report()
}

/// The same report, committing the ambient bootstrap if nothing has —
/// exactly [`Tolerance::get`]'s behaviour, and the same hazard: a
/// program that later loads a document should use [`committed_report`]
/// instead.
pub fn report() -> ToleranceReport {
    Tolerance::report()
}

/// Which channel supplied the run's committed ε. Commits the ambient
/// bootstrap if nothing has, as [`report`] does.
pub fn eps_source() -> EpsilonSource {
    Tolerance::eps_source()
}

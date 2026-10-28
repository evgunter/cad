//! **A red row pinned at what main produces**, while the row that owns
//! its fix stays open: at a pinned ε the row must stop on a panic that
//! says the pinned text, so the pin goes red the day the row passes or
//! stops some other way; at any other ε it runs as written.
//!
//! [`kernel_limit`](crate::kernel_limit) is the same pin for the one
//! refusal it names; this one takes the stop's text from the caller,
//! who cites the open row beside it.

/// Every ε row CI draws from: the default, 1e-6 and 1e-12.
pub const EVERY_ROW: [f64; 3] = [1e-9, 1e-6, 1e-12];

/// **A whole row pinned to stop**: at an ε in `stops_at` the row `f`
/// must panic with a message containing `says`.
///
/// # Panics
///
/// At a pinned ε, when `f` passes or panics without `says`; at any
/// other, as `f` does.
#[track_caller]
pub fn row(
    eps: f64,
    stops_at: &[f64],
    what: &str,
    says: &str,
    f: impl FnOnce() + std::panic::UnwindSafe,
) {
    crate::kernel_limit::stops_on(eps, stops_at, what, says, f, |message| {
        message.contains(says)
    });
}

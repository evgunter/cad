//! **A red row pinned at what main produces**, while the row that owns
//! its fix stays open: at a pinned ε the row must stop on a panic that
//! says the pinned text, so the pin goes red the day the row passes or
//! stops some other way; at any other ε it runs as written.
//!
//! [`kernel_limit`](crate::kernel_limit) is the same pin for the one
//! refusal it names; this one takes the stop's text from the caller,
//! who cites the open row beside it.

#![allow(clippy::panic)]

/// Every ε row CI draws from: the default (`geom_core::DEFAULT_EPS`),
/// 1e-6 and 1e-12, as the nightly's full-suite loop spells them
/// (`.github/workflows/nightly.yml`, `for eps in default 1e-6 1e-12`).
/// This crate depends on nothing, so it cannot read them from there.
pub const EVERY_ROW: [f64; 3] = [1e-9, 1e-6, 1e-12];

/// **A whole row pinned to stop**: at an ε in `stops_at` the row `f`
/// must panic with a message containing every piece of `says`. Name
/// main's stop specifically (the refusal, its values), so that a
/// different stop is red too.
///
/// # Panics
///
/// At a pinned ε, when `f` passes or panics without one of `says`; at
/// any other, as `f` does.
#[track_caller]
pub fn row(
    eps: f64,
    stops_at: &[f64],
    what: &str,
    says: &[&str],
    f: impl FnOnce() + std::panic::UnwindSafe,
) {
    stops_on(eps, stops_at, what, &format!("{says:?}"), f, |message| {
        says.iter().all(|piece| message.contains(piece))
    });
}

/// The row `f` run under a pin: at a pinned ε its panic message must
/// satisfy `holds`; at any other it runs as written.
#[track_caller]
pub(crate) fn stops_on(
    eps: f64,
    stops_at: &[f64],
    what: &str,
    pin: &str,
    f: impl FnOnce() + std::panic::UnwindSafe,
    holds: impl FnOnce(&str) -> bool,
) {
    if !crate::kernel_limit::at_row(eps, stops_at) {
        f();
        return;
    }
    match crate::panic_capture::caught(f) {
        Some(message) => assert!(
            holds(&message),
            "{what}: pinned to stop on {pin} at ε = {eps:e}, it stopped otherwise: {message}"
        ),
        None => panic!("{what}: pinned to stop on {pin} at ε = {eps:e}, and it passed"),
    }
}

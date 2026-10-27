//! **A fixture pinned to refuse as limb 2's kernel approximation limit**
//! at the tolerance rows its geometry cannot certify at
//! (`geom_brep::AnalyticRung3Refusal::KernelLimit`, C2 in the geom-brep
//! README): a fixture that builds at the default ε but whose carrier's
//! certified distance bound cannot get under a finer ε.
//!
//! The refusal is read off its `Debug` rendering or its `Display`
//! sentence, since the fixtures wrap it in every door's own error; this
//! crate depends on nothing, so it cannot name the type.
//!
//! **At an enclosure scalar the same stall is an escalation** (fork row
//! 203): limb 2 decides the enclosure `[floor, upper]` of the carrier's
//! distance there, and an upper bound that stalls past the band is that
//! decision's `Indeterminate`, refused as
//! `AnalyticRung3Refusal::Escalated` on `HullSup` with no cause and no
//! terminality claimed. A fixture pinned to the kernel limit reads
//! either: the limit at a point scalar, the stall at an enclosure one.

#![allow(clippy::panic)]

use core::fmt::Debug;

/// The `Debug` marker of the refusal.
const DEBUG_MARK: &str = "AnalyticRung3(KernelLimit {";

/// The `Display` phrase of the refusal.
const DISPLAY_MARK: &str = "does not clear the run tolerance";

/// The `Debug` marks of the stall at an enclosure scalar: limb 2's
/// escalation on its distance bound.
const STALL_MARKS: [&str; 2] = ["AnalyticRung3(Escalated {", "limb: HullSup"];

/// The `Display` phrase of the stall at an enclosure scalar.
const STALL_DISPLAY: &str = "limb 2 (control-hull sup-norm bound) against the";

/// Whether `refusal` holds limb 2's kernel limit, or at an enclosure
/// scalar its stall (module docs), read off its `Debug` rendering.
#[must_use]
pub fn is_kernel_limit(refusal: &dyn Debug) -> bool {
    let text = format!("{refusal:?}");
    text.contains(DEBUG_MARK) || STALL_MARKS.iter().all(|m| text.contains(m))
}

/// Whether a refusal's sentence is limb 2's kernel limit, or at an
/// enclosure scalar its stall (module docs).
#[must_use]
pub fn says_kernel_limit(sentence: &str) -> bool {
    sentence.contains(DISPLAY_MARK)
        || (sentence.contains(STALL_DISPLAY) && sentence.contains("escalated"))
}

/// Whether `eps` is one of `rows` (each read to a part in a thousand).
#[must_use]
pub fn at_row(eps: f64, rows: &[f64]) -> bool {
    rows.iter().any(|&r| (eps - r).abs() <= 1e-3 * r)
}

/// **A fixture's build, pinned**: at an ε in `refused_at` it must refuse
/// as the kernel limit, and `None` comes back for the row to stop on; at
/// any other ε it must build, and its value comes back.
///
/// # Panics
///
/// When the build refuses otherwise, refuses at an unpinned ε, or
/// builds at a pinned one.
#[track_caller]
pub fn pinned<T, E: Debug>(
    got: Result<T, E>,
    eps: f64,
    refused_at: &[f64],
    what: &str,
) -> Option<T> {
    match got {
        Ok(value) => {
            assert!(
                !at_row(eps, refused_at),
                "{what}: pinned to refuse as limb 2's kernel limit at ε = {eps:e}, and it built"
            );
            Some(value)
        }
        Err(e) => {
            assert!(
                at_row(eps, refused_at) && is_kernel_limit(&e),
                "{what}: at ε = {eps:e} (pinned to the kernel limit at {refused_at:?}): {e:?}"
            );
            None
        }
    }
}

/// **[`pinned`] for a fixture whose bound sits near the band at a row**:
/// at an ε in `may_refuse_at` a kernel-limit refusal is accepted (`None`)
/// and so is a build; at any other ε it must build. For a sweep of
/// cases some of which read just under the band at the row and some just
/// over it.
///
/// # Panics
///
/// When the build refuses otherwise, or refuses at an ε outside
/// `may_refuse_at`.
#[track_caller]
pub fn may_refuse<T, E: Debug>(
    got: Result<T, E>,
    eps: f64,
    may_refuse_at: &[f64],
    what: &str,
) -> Option<T> {
    match got {
        Ok(value) => Some(value),
        Err(e) => {
            assert!(
                at_row(eps, may_refuse_at) && is_kernel_limit(&e),
                "{what}: at ε = {eps:e} (may refuse as the kernel limit at {may_refuse_at:?}): {e:?}"
            );
            None
        }
    }
}

/// **A whole row pinned**: at an ε in `refused_at` the row `f` must stop
/// on limb 2's kernel limit (its panic message carries the refusal), so a
/// fixture built deep inside it with `unwrap` needs no reshaping; at any
/// other ε it runs as written.
///
/// # Panics
///
/// At a pinned ε, when `f` passes or panics for another reason; at any
/// other, as `f` does.
#[track_caller]
pub fn pinned_row(
    eps: f64,
    refused_at: &[f64],
    what: &str,
    f: impl FnOnce() + std::panic::UnwindSafe,
) {
    if !at_row(eps, refused_at) {
        f();
        return;
    }
    match crate::panic_capture::caught(f) {
        Some(message) => assert!(
            message.contains(DEBUG_MARK)
                || STALL_MARKS.iter().all(|m| message.contains(m))
                || says_kernel_limit(&message),
            "{what}: pinned to stop on limb 2's kernel limit at ε = {eps:e}, it stopped otherwise: \
             {message}"
        ),
        None => {
            panic!("{what}: pinned to stop on limb 2's kernel limit at ε = {eps:e}, and it passed")
        }
    }
}

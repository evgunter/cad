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
//! 204): limb 2 decides the enclosure `[floor, upper]` of the carrier's
//! distance there, and an upper bound that stalls past the band is that
//! decision's `Indeterminate`, refused as `AnalyticRung3Refusal::Stalled`
//! with no cause and no terminality claimed — a variant of its own, which
//! limb 2 alone mints and only at an enclosure scalar, so no other
//! escalation (a cone's nappe, a chart's lever) reads as it. A pin names
//! its fixture's [`Scalar`] and accepts that scalar's shape alone: the
//! limit at a point scalar, the stall at an enclosure one.
//!
//! **Neither is a distance measured in the band.** Where limb 2 measures
//! a distance whose enclosure excludes zero in the ambiguity band, or
//! straddling its edge (a line's end against a plane), the input is
//! ambiguous at that tolerance and limb 2 escalates on the measured
//! value (`Escalated` on `HullValue`), which no kernel-limit pin
//! accepts.

#![allow(clippy::panic)]

use core::fmt::Debug;

/// The `Debug` marker of the refusal.
const DEBUG_MARK: &str = "AnalyticRung3(KernelLimit {";

/// The `Display` phrase of the refusal.
const DISPLAY_MARK: &str = "does not clear the run tolerance";

/// The `Debug` mark of the stall at an enclosure scalar.
const STALL_MARK: &str = "AnalyticRung3(Stalled {";

/// The `Display` phrase of the stall at an enclosure scalar.
const STALL_DISPLAY: &str = "is undecided over this input";

/// **The scalar a fixture reads at**, which names the one shape its
/// stall takes: [`Scalar::Point`] (`f64`, the probe) refuses as the
/// kernel limit, [`Scalar::Enclosure`] (`Interval`, `Sym<Interval>`)
/// as the stall. A pin accepts only its own scalar's shape, so a point
/// fixture that stalls, or an enclosure one that claims the limit, is
/// red.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scalar {
    /// A point scalar: the kernel limit.
    Point,
    /// An enclosure scalar: the stall.
    Enclosure,
}

impl Scalar {
    /// The `Debug` mark and `Display` phrase of this scalar's stall.
    const fn marks(self) -> (&'static str, &'static str) {
        match self {
            Self::Point => (DEBUG_MARK, DISPLAY_MARK),
            Self::Enclosure => (STALL_MARK, STALL_DISPLAY),
        }
    }
}

/// Whether `refusal` holds limb 2's kernel limit, read off its `Debug`
/// rendering — at a point scalar; [`is_kernel_limit_at`] names the
/// scalar.
#[must_use]
pub fn is_kernel_limit(refusal: &dyn Debug) -> bool {
    is_kernel_limit_at(Scalar::Point, refusal)
}

/// Whether `refusal` holds limb 2's stall as `scalar` refuses it
/// ([`Scalar`]), read off its `Debug` rendering.
#[must_use]
pub fn is_kernel_limit_at(scalar: Scalar, refusal: &dyn Debug) -> bool {
    format!("{refusal:?}").contains(scalar.marks().0)
}

/// Whether a refusal's sentence is limb 2's kernel limit, at a point
/// scalar.
#[must_use]
pub fn says_kernel_limit(sentence: &str) -> bool {
    says_kernel_limit_at(Scalar::Point, sentence)
}

/// Whether a refusal's sentence is limb 2's stall as `scalar` refuses
/// it ([`Scalar`]).
#[must_use]
pub fn says_kernel_limit_at(scalar: Scalar, sentence: &str) -> bool {
    sentence.contains(scalar.marks().1)
}

/// Whether `eps` is one of `rows` (each read to a part in a thousand).
#[must_use]
pub fn at_row(eps: f64, rows: &[f64]) -> bool {
    rows.iter().any(|&r| (eps - r).abs() <= 1e-3 * r)
}

/// **A fixture's build, pinned** at a point scalar: [`pinned_at`] with
/// [`Scalar::Point`].
///
/// # Panics
///
/// As [`pinned_at`].
#[track_caller]
pub fn pinned<T, E: Debug>(
    got: Result<T, E>,
    eps: f64,
    refused_at: &[f64],
    what: &str,
) -> Option<T> {
    pinned_at(Scalar::Point, got, eps, refused_at, what)
}

/// **A fixture's build, pinned**: at an ε in `refused_at` it must refuse
/// as limb 2's stall at `scalar`, and `None` comes back for the row to
/// stop on; at any other ε it must build, and its value comes back.
///
/// # Panics
///
/// When the build refuses otherwise, refuses at an unpinned ε, or
/// builds at a pinned one.
#[track_caller]
pub fn pinned_at<T, E: Debug>(
    scalar: Scalar,
    got: Result<T, E>,
    eps: f64,
    refused_at: &[f64],
    what: &str,
) -> Option<T> {
    match got {
        Ok(value) => {
            assert!(
                !at_row(eps, refused_at),
                "{what}: pinned to refuse as limb 2's kernel limit ({scalar:?} scalar) at \
                 ε = {eps:e}, and it built"
            );
            Some(value)
        }
        Err(e) => {
            assert!(
                at_row(eps, refused_at) && is_kernel_limit_at(scalar, &e),
                "{what}: at ε = {eps:e} (pinned to the kernel limit, {scalar:?} scalar, at \
                 {refused_at:?}): {e:?}"
            );
            None
        }
    }
}

/// **A whole row pinned** at a point scalar: [`pinned_row_at`] with
/// [`Scalar::Point`].
///
/// # Panics
///
/// As [`pinned_row_at`].
#[track_caller]
pub fn pinned_row(
    eps: f64,
    refused_at: &[f64],
    what: &str,
    f: impl FnOnce() + std::panic::UnwindSafe,
) {
    pinned_row_at(Scalar::Point, eps, refused_at, what, f);
}

/// **A whole row pinned**: at an ε in `refused_at` the row `f` must stop
/// on limb 2's stall at `scalar` (its panic message carries the
/// refusal), so a fixture built deep inside it with `unwrap` needs no
/// reshaping; at any other ε it runs as written.
///
/// # Panics
///
/// At a pinned ε, when `f` passes or panics for another reason; at any
/// other, as `f` does.
#[track_caller]
pub fn pinned_row_at(
    scalar: Scalar,
    eps: f64,
    refused_at: &[f64],
    what: &str,
    f: impl FnOnce() + std::panic::UnwindSafe,
) {
    crate::pinned_red::stops_on(
        eps,
        refused_at,
        what,
        "limb 2's kernel limit",
        f,
        |message| {
            let (debug, display) = scalar.marks();
            message.contains(debug) || message.contains(display)
        },
    );
}

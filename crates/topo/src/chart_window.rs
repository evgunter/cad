//! **A face's azimuth window on its chart**: the first-channel hull of
//! its outer loop's lift ([`crate::pcurves::Lift`]), read from the rows
//! the body stores. The lift is the face's boundary in the chart,
//! through a pole or an apex along the face's turn there
//! (`crate::joint`), so the window is a method on it
//! ([`crate::pcurves::Lift::azimuth_window`]) and this home walks
//! nothing and pins nothing.
//!
//! Its home follows `[topo.homes]` (`crates/topo/README.md`): a shared
//! reading, refusing in its own error type.

use geom_core::Decide;

use crate::body::Body;
use crate::entity::FaceKey;
use crate::pcurves::{Lift, LiftGap};

/// Why [`face_azimuth_window`] read no window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartWindowError {
    /// `face`, the caller's key, does not resolve.
    Stale(FaceKey),
    /// The face's outer loop has no lift to read: a row or a joint
    /// element is missing ([`Lift::read`]).
    Gap(LiftGap),
}

impl core::fmt::Display for ChartWindowError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Stale(face) => write!(f, "the face key {face:?} does not resolve"),
            Self::Gap(LiftGap { half_edge }) => write!(
                f,
                "the face's outer loop has no lift to read: half-edge {half_edge:?} stores no \
                 chart image, or no element on the joint into it"
            ),
        }
    }
}

impl std::error::Error for ChartWindowError {}

/// The azimuth window of `face`'s outer loop on its chart: the hull of
/// its stored lift's first channel. `None` for an empty loop, or one
/// with an image that has no closed-form span box (a projected, spiric
/// or fitted image), which the window does not read.
///
/// # Errors
///
/// [`ChartWindowError`].
///
/// # Panics
///
/// On a torn hop past `face` ([`Lift::read`]).
pub(crate) fn face_azimuth_window<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Option<(T, T)>, ChartWindowError> {
    let outer = body
        .get_face(face)
        .ok_or(ChartWindowError::Stale(face))?
        .outer;
    Lift::read(body, outer)
        .map(|lift| lift.azimuth_window())
        .map_err(ChartWindowError::Gap)
}

/// [`face_azimuth_window`], read from outside the crate so a suite can
/// hold the interval lane's window against the `f64` replay's.
/// `sweep-testing` only, never production surface.
///
/// # Errors
///
/// As [`face_azimuth_window`].
///
/// # Panics
///
/// As [`face_azimuth_window`].
#[cfg(feature = "sweep-testing")]
pub fn face_azimuth_window_traces<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<Option<(T, T)>, ChartWindowError> {
    face_azimuth_window(body, face)
}

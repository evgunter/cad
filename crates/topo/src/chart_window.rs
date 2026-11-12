//! **A face's azimuth window on its chart**: the first-channel hull of
//! its outer loop's lift ([`crate::pcurves::Lift`]). The lift is the
//! face's boundary in the chart, through a pole or an apex along the
//! face's turn there (`crate::joint`), so the window is a method on it
//! ([`crate::pcurves::Lift::azimuth_window`]) and this home walks
//! nothing and pins nothing.
//!
//! Its readers run mid-operation as well as at rest, on bodies whose
//! rows are not all written, so the lift is the decided one
//! ([`crate::pcurves::Lift::decided`]), at the reader's band.
//!
//! Its home follows `[topo.homes]` (`crates/topo/README.md`): a shared
//! reading, refusing in its own error type.

use geom_core::{Band, Decide, Indeterminate};

use crate::body::Body;
use crate::entity::FaceKey;
use crate::pcurves::{Lift, LiftRefusal, PinMiss};

/// Why [`face_azimuth_window`] read no window.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ChartWindowError {
    /// `face`, the caller's key, does not resolve.
    Stale(FaceKey),
    /// A decision of the lift escalated at the band.
    Escalated(Indeterminate),
    /// The face's outer loop has no lift the walk decides: an edge with
    /// no closed-form image on the chart, or a joint that names no
    /// branch.
    Unread,
}

impl core::fmt::Display for ChartWindowError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Stale(face) => write!(f, "the face key {face:?} does not resolve"),
            Self::Escalated(cause) => {
                write!(f, "the face's outer loop's lift escalated: {cause}")
            }
            Self::Unread => f.write_str(
                "the face's outer loop has no lift on its chart: an edge of it has no \
                 closed-form image there, or a joint of it names no branch",
            ),
        }
    }
}

impl std::error::Error for ChartWindowError {}

impl From<LiftRefusal> for ChartWindowError {
    fn from(refusal: LiftRefusal) -> Self {
        use geom_brep::{PcurveCertifyError, PcurveCheck};
        match refusal {
            LiftRefusal::Joint(PinMiss::Escalated(cause) | PinMiss::NetStretchUndecided(cause)) => {
                Self::Escalated(cause)
            }
            // A class reading or a lane not built is a lane choice, and
            // an incidence refusal ends by its read: neither is the
            // window's escalation.
            LiftRefusal::Image(PcurveCertifyError::Escalated {
                check: PcurveCheck::ChartClass | PcurveCheck::GeneralLane,
                ..
            }) => Self::Unread,
            LiftRefusal::Image(error) if geom_brep::IncidenceRead::of(&error).is_some() => {
                Self::Unread
            }
            LiftRefusal::Image(PcurveCertifyError::Escalated { cause, .. }) => {
                Self::Escalated(cause)
            }
            LiftRefusal::Image(_)
            | LiftRefusal::Joint(PinMiss::Discontinuity | PinMiss::Singular(_))
            | LiftRefusal::Placeholder
            | LiftRefusal::NotClosed => Self::Unread,
        }
    }
}

/// The azimuth window of `face`'s outer loop on its chart: the hull of
/// its decided lift's first channel. `None` for a loop that is not a
/// cycle, or one with an image that has no closed-form span box (a
/// projected or spiric image), which the window does not read.
///
/// # Errors
///
/// [`ChartWindowError`].
///
/// # Panics
///
/// On a torn hop past `face` ([`crate::live::FaceView::outer_cycle`],
/// [`Lift::decided`]).
pub(crate) fn face_azimuth_window<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Option<(T, T)>, ChartWindowError> {
    let view = body.face(face).ok_or(ChartWindowError::Stale(face))?;
    let Some(halves) = view.outer_cycle() else {
        return Ok(None);
    };
    let lift = Lift::decided(body, face, view.surface(), &halves, true, band)?;
    Ok(lift.azimuth_window())
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
    band: Band,
) -> Result<Option<(T, T)>, ChartWindowError> {
    face_azimuth_window(body, face, band)
}

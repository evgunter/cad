//! **A stated chart image's certificate**: one door that certifies an
//! image a caller holds (a description's stated image, a stored row
//! restated at rest) through the pcurve certificate door its kind names
//! ([`Pcurve::certifying_door`]), so its miss is bounded over the span by
//! that door's envelope, a sup, wherever it is read.

use std::sync::Arc;

use geom::{Curve3, Surface};
use geom_core::{Band, Decide};

use super::{CertifyingDoor, Pcurve, PcurveCache, PcurveCertifyError};

impl<T: Decide> PcurveCache<T> {
    /// Certifies `image`, a chart image of `carrier` on `surface` that a
    /// caller holds, over `[t0, t1]` through the door its kind names
    /// ([`Pcurve::certifying_door`]): the closed-form door
    /// ([`PcurveCache::certify`]), or the fitted door, `fitted`, where the
    /// scalar holds one ([`crate::FittedLane`]). Every door it reaches
    /// ends a refusal by the carrier's incidence read, so a carrier off
    /// its face ends as that read.
    ///
    /// # Errors
    ///
    /// The door's refusal; [`PcurveCertifyError::FittedLaneUnsupported`]
    /// for a fitted image at a scalar with no fitted door, ended by the
    /// read as every door's refusal is.
    pub fn certify_stated_image(
        image: &Pcurve<T>,
        (t0, t1): (T, T),
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
        fitted: Option<crate::FittedLane<T>>,
    ) -> Result<Self, PcurveCertifyError> {
        let door = image.certifying_door();
        if door == CertifyingDoor::ClosedForm {
            return PcurveCache::certify(image.clone(), t0, t1, carrier, surface, band);
        }
        match image {
            Pcurve::Fitted(image) => match fitted {
                Some(lane) => PcurveCache::certify_fitted(
                    Arc::clone(image),
                    t0,
                    t1,
                    carrier,
                    surface,
                    band,
                    lane,
                ),
                None => super::incidence_ended_unwitnessed(
                    carrier,
                    (t0, t1),
                    (t0, t1),
                    surface,
                    band,
                    |read| read,
                    || Err(PcurveCertifyError::FittedLaneUnsupported { scalar: T::NAME }),
                ),
            },
            Pcurve::General(image) => PcurveCache::certify_general(
                Arc::clone(image),
                t0,
                t1,
                carrier,
                surface,
                band,
                fitted,
            ),
            Pcurve::Projected(image) => PcurveCache::certify_projected(
                (**image).clone(),
                t0,
                t1,
                carrier,
                surface,
                band,
                fitted,
            ),
            Pcurve::Harmonic { .. }
            | Pcurve::FocalSection(_)
            | Pcurve::Spiric { .. }
            | Pcurve::IsoLine { .. }
            | Pcurve::IsoArc { .. } => {
                unreachable!(
                    "certify_stated_image: a closed-form image took its door above: {door:?}"
                )
            }
        }
    }
}

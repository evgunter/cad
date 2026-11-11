//! **A stated chart image's certificate**: one door that certifies an
//! image a caller holds (a description's stated image, a stored row
//! restated at rest) through the pcurve certificate door its kind names
//! ([`Pcurve::certifying_door`]), so its miss is bounded over the span by
//! that door's envelope, a sup, wherever it is read.
//!
//! The edge certificate reaches a stated image's bound through
//! [`StatedDoor`]: the door that certifies the image's class on its
//! chart, or none, where the class has no door yet.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, Surface};
use geom_core::spline::{Knots, SpanLocate};
use geom_core::{Band, Decide, Point2, Real};

use super::patch::{PatchClass, PatchImage};
use super::{
    CertifyingDoor, EnvelopeStatement, Pcurve, PcurveCache, PcurveCertificate, PcurveCertifyError,
    PcurveCheck, Record,
};
use crate::certify::CERT_SAMPLES;

/// **The door that bounds a stated image's miss from its carrier over
/// the span**, by the image's class on its chart, holding the image in
/// the form that door reads: decided once, by structure (the chart's
/// kind, the image's kind and shape), so a class is routed the same way
/// at every scalar and every band.
#[derive(Clone, Copy, Debug)]
pub(crate) enum StatedDoor<'a, T: Real> {
    /// The image's own pcurve door ([`PcurveCache::certify_stated_image`]):
    /// every image on a spline chart, and a projected image on an
    /// analytic one, which is that door's class there.
    Pcurve(&'a Pcurve<T>),
    /// The fitted door's composite against the bilinear patch that agrees
    /// with an analytic chart along the image ([`PatchClass`]): an iso line
    /// or a curve in UV on a plane, or on a cylinder's or cone's ruling.
    Patch {
        /// The image as stated, which the schedule reads.
        image: &'a Pcurve<T>,
        /// The patch's class.
        class: PatchClass,
        /// The image as the composite reads it.
        form: PatchImage<'a, T>,
    },
}

impl<'a, T: Real> StatedDoor<'a, T> {
    /// The door of `image` stated on `surface`, or `None` where no door
    /// certifies its class: a closed-form image (harmonic, spiric, focal
    /// section, iso arc) on an analytic chart, and a curve in UV on a
    /// curved analytic chart off one ruling. Those are metered at the
    /// edge certificate's samples
    /// (`work/pcert/a-closed-form-image-stated-on-an-analytic-chart-is-metered-at-nine-samples.md`).
    pub(crate) fn of(surface: &Surface<T>, image: &'a Pcurve<T>) -> Option<Self> {
        if surface.spline_chart().is_some() || matches!(image, Pcurve::Projected(_)) {
            return Some(Self::Pcurve(image));
        }
        PatchClass::of(surface, image).map(|(class, form)| Self::Patch { image, class, form })
    }
}

/// **The span a stated image is certified over**: the edge's interval
/// and its outer ends, the interval's `f64` structure (C6).
#[derive(Clone, Copy, Debug)]
pub(crate) struct EdgeSpan<T> {
    /// `[t0, t1]` at the certifying scalar.
    pub(crate) params: (T, T),
    /// Its outer ends.
    pub(crate) outer: (f64, f64),
}

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

    /// **A stated image's certificate through the door its class names**
    /// ([`StatedDoor`]), over `span`: the certificate's envelope bounds
    /// `|C(t) − S(P(t))|` over the whole span.
    ///
    /// On a patch, the fitted lane's checks 3 and 4: the schedule, then
    /// the composite's sup against the patch, which agrees with the chart
    /// wherever the image runs, so the sup bounds the image's miss with no
    /// remainder. An iso line enters as the degree-1 curve in UV, the
    /// same map, over a spline carrier's knot domain or else the span's
    /// outer ends. The carrier's incidence is the caller's to read first:
    /// this door reads the image against the carrier, not the carrier
    /// against the chart.
    ///
    /// # Errors
    ///
    /// The door's refusal: [`PcurveCertifyError::ResidualExceeded`] or
    /// [`PcurveCertifyError::Escalated`] at a sample, the composite's
    /// ([`PcurveCertifyError::FittedLimb`],
    /// [`PcurveCertifyError::FittedEscalated`], and its structural
    /// refusals) over the span.
    pub(crate) fn certify_stated_bound(
        door: StatedDoor<'_, T>,
        span: EdgeSpan<T>,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
        lane: crate::FittedLane<T>,
    ) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
        let (t0, t1) = span.params;
        let (image, class, form) = match door {
            StatedDoor::Pcurve(image) => {
                return Self::certify_stated_image(
                    image,
                    span.params,
                    carrier,
                    surface,
                    band,
                    Some(lane),
                )
                .map(|cache| cache.certificate);
            }
            StatedDoor::Patch { image, class, form } => (image, class, form),
        };
        let net = match form {
            // A spline carrier pairs with the curve in UV on its own knot
            // domain (the composite's one parameter), any other on the
            // span's outer ends.
            PatchImage::Iso(p0, pl) => {
                let domain = match carrier {
                    Curve3::Nurbs(net) => net.domain(),
                    _ => span.outer,
                };
                iso_net(p0, pl, domain)?
            }
            PatchImage::Net(net) => Arc::clone(net),
        };
        let mut max_residual = T::zero();
        super::schedule_residuals(
            Record::Verdict,
            image,
            t0,
            t1,
            carrier,
            surface,
            band,
            &mut max_residual,
        )?;
        let chart = super::FittedChart::Patch(surface, class);
        let sup = lane.fitted_certificate(carrier, &net, chart, span.params, band)?;
        super::envelope_verdict(sup, band, || PcurveCheck::Envelope)?;
        Ok(PcurveCertificate::closed(
            CERT_SAMPLES,
            max_residual,
            sup,
            EnvelopeStatement::MapResidualComposite,
        ))
    }
}

/// **An iso line as a curve in UV**: `p0 + pl·t` over `[lo, hi]`, the
/// degree-1 curve through its values at the two ends, which is the same
/// map on the same parameter.
fn iso_net<T: SpanLocate>(
    p0: Point2<T>,
    pl: geom_core::Vec2<T>,
    (lo, hi): (f64, f64),
) -> Result<Arc<NurbsCurve2<T>>, PcurveCertifyError> {
    let lift = |t: f64| T::from_f64(t);
    let at = |t: f64| p0 + pl * lift(t);
    let two = std::num::NonZeroUsize::MIN.saturating_add(1);
    let knots = Knots::new(vec![(lift(lo), two), (lift(hi), two)], 1)
        .map_err(|_| PcurveCertifyError::IntervalNotForward)?;
    NurbsCurve2::new(knots, vec![at(lo), at(hi)], vec![T::one(), T::one()])
        .map(Arc::new)
        .map_err(|_| PcurveCertifyError::IntervalNotForward)
}

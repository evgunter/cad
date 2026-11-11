//! **A stated chart image's certificate**: one door that certifies an
//! image a caller holds (a description's stated image, a stored row
//! restated at rest) through the pcurve certificate door its kind names
//! ([`Pcurve::certifying_door`]), so its miss is bounded over the span by
//! that door's envelope, a sup, wherever it is read.
//!
//! The edge certificate reaches a stated image's bound through
//! [`StatedDoor`]: the door that certifies the image's class on its
//! chart.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, Surface};
use geom_core::spline::{Knots, SpanLocate};
use geom_core::{Band, Decide, Point2, Real};

use super::closed_bound::ClosedForm;
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
    /// every image on a spline chart, and on an analytic one a projected
    /// image, which is that door's class there, and an iso arc, which is
    /// a spline chart's image and which that door refuses on any other.
    Pcurve(&'a Pcurve<T>),
    /// A closed-form image on an analytic chart (harmonic, spiric, focal
    /// section), bounded by the coefficients of its residual against the
    /// carrier ([`super::closed_bound`]).
    Closed(&'a Pcurve<T>, ClosedForm<T>),
    /// An iso line or a curve in UV on an analytic chart: the fitted
    /// door's composite against the patch of its [`PatchClass`], which
    /// agrees with the chart along the image (a plane; a cylinder's or
    /// cone's ruling) or encloses the chart over the image's window.
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
    /// The door of `image` stated on `surface`: every stated image has
    /// one.
    pub(crate) fn of(surface: &Surface<T>, image: &'a Pcurve<T>) -> Self {
        if surface.spline_chart().is_some() {
            return Self::Pcurve(image);
        }
        if let Some(form) = ClosedForm::of(image) {
            return Self::Closed(image, form);
        }
        match PatchClass::of(surface, image) {
            Some((class, form)) => Self::Patch { image, class, form },
            None => Self::Pcurve(image),
        }
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
    /// A closed form: its residual's coefficient bound
    /// ([`super::closed_bound`]). On a patch, the fitted lane's checks 3
    /// and 4: the schedule, then the composite's sup against the patch,
    /// which agrees with the chart wherever the image runs or encloses it
    /// with its remainder in its controls, so the sup bounds the image's
    /// miss. An iso line enters as the degree-1 curve in UV, the
    /// same map, over a spline carrier's knot domain or else the span's
    /// outer ends. The carrier's incidence is the caller's to read first:
    /// this door reads the image against the carrier, not the carrier
    /// against the chart.
    ///
    /// # Errors
    ///
    /// The door's refusal: [`PcurveCertifyError::ResidualExceeded`] or
    /// [`PcurveCertifyError::Escalated`] at a sample or of a bound,
    /// [`PcurveCertifyError::MissLocated`] at a located point, the
    /// composite's
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
        #[cfg(test)]
        if let Some(image) = sampled_mutant(door, surface) {
            return sampled(image, span, carrier, surface, band);
        }
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
            StatedDoor::Closed(image, form) => {
                return super::closed_bound::certify((image, form), span, carrier, surface, band);
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

/// The image a mutant meters at the schedule's samples again, not
/// over the span: an analytic chart's stated image
/// (`analytic-image-sampled`), or its closed form alone
/// (`closed-image-sampled`).
#[cfg(test)]
fn sampled_mutant<'a, T: Real>(
    door: StatedDoor<'a, T>,
    surface: &Surface<T>,
) -> Option<&'a Pcurve<T>> {
    let analytic = surface.spline_chart().is_none() && super::mutants::analytic_image_sampled();
    match door {
        StatedDoor::Closed(image, _) if analytic || super::mutants::closed_image_sampled() => {
            Some(image)
        }
        StatedDoor::Pcurve(image) | StatedDoor::Patch { image, .. } if analytic => Some(image),
        StatedDoor::Pcurve(_) | StatedDoor::Closed(..) | StatedDoor::Patch { .. } => None,
    }
}

/// The schedule's samples alone, as the mutants read a stated image.
#[cfg(test)]
fn sampled<T: Decide>(
    image: &Pcurve<T>,
    span: EdgeSpan<T>,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    let mut max_residual = T::zero();
    let (t0, t1) = span.params;
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
    Ok(PcurveCertificate::closed(
        CERT_SAMPLES,
        max_residual,
        T::zero(),
        EnvelopeStatement::MapResidualComposite,
    ))
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

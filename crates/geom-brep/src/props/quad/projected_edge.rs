//! **A projected cylinder image as the rotational lane reads it**
//! ([`ProjectedEdgeQ::of_image`]): the image's carrier and deck map
//! crossed into certification arithmetic, by the caller's lift, at any
//! scalar the image holds.

use geom_core::interval::Interval;
use geom_core::{Bounds, Real};

use super::rotational::{FramedQ, ProjectedEdgeQ};
use crate::pcurve_cache::{FramedCarrier, ProjectedChart, ProjectedImage};
use crate::props::PropsError;
use crate::props::bracket::{mid, pt};
use crate::props::rational_rim::rational_edge_q_split;

impl ProjectedEdgeQ {
    /// **A projected cylinder image as the lane reads it**: `image`'s
    /// carrier in the chart's frame over the edge's carrier interval
    /// `(t0, t1)`, cut at its sector breaks, and its deck map's level
    /// part, each scalar crossed into certification arithmetic by `iv`;
    /// `ends`, the image's chart points at `t0` and `t1` as the loop reads
    /// it (its lift), are the caller's, read by the image's own evaluator
    /// at its scalar.
    ///
    /// # Errors
    ///
    /// [`PropsError::QuadratureUnsupported`] for an image whose inverse is
    /// not the cylinder's, a deck sign other than exactly `±1`, inexact
    /// carrier interval ends, or a net the curved rims' rule cannot
    /// decompose ([`crate::props::rational_rim::rational_edge_q_split`]).
    pub fn of_image<T: Real>(
        image: &ProjectedImage<T>,
        (t0, t1): (Interval, Interval),
        forward: bool,
        ends: [(Interval, Interval); 2],
        env: Interval,
        iv: impl Fn(T) -> Interval,
    ) -> Result<Self, PropsError> {
        let unsupported = |what| PropsError::QuadratureUnsupported { what };
        let exact = |x: Interval| (x.is_certified() && x.lo() == x.hi()).then_some(x.lo());
        if !matches!(image.chart(), ProjectedChart::Cylinder) {
            return Err(unsupported(
                "a projected image on the cylinder lane whose inverse is another chart's",
            ));
        }
        let (_, v_off, v_sign) = image.deck();
        let v_sign = match exact(iv(v_sign)) {
            Some(s) if s == 1.0 || s == -1.0 => s,
            _ => {
                return Err(unsupported(
                    "a projected image whose deck map's level sign is not exactly ±1",
                ));
            }
        };
        let (Some(lo), Some(hi)) = (exact(t0), exact(t1)) else {
            return Err(unsupported(
                "a projected edge whose carrier interval ends are not exact parameters",
            ));
        };
        let breaks = image.breaks().interior();
        let vec = |v: geom_core::Vec3<T>| [iv(v.x), iv(v.y), iv(v.z)];
        // A circle's or a line's breaks are on `s = (t − origin)/span`,
        // read where the image placed them: any `f64` cut tiles the
        // interval exactly.
        let cuts = |origin: T, span: T| {
            let (origin, span) = (iv(origin), iv(span));
            let mut cuts = vec![lo];
            cuts.extend(
                breaks
                    .iter()
                    .map(|&s| mid(origin + span * pt(s)))
                    .filter(|&t| lo < t && t < hi),
            );
            cuts.push(hi);
            cuts
        };
        let carrier = match image.carrier() {
            FramedCarrier::Net(net) => FramedQ::Net(rational_edge_q_split(
                &net.map_scalar(&iv),
                (t0, t1),
                true,
                geom_core::Point3::origin(),
                breaks,
            )?),
            FramedCarrier::Circle {
                centre,
                a,
                b,
                origin,
                span,
            } => FramedQ::Circle {
                centre: vec(*centre),
                a: vec(*a),
                b: vec(*b),
                cuts: cuts(*origin, *span),
            },
            FramedCarrier::Line {
                point,
                dir,
                origin,
                span,
            } => FramedQ::Line {
                point: vec(*point),
                dir: vec(*dir),
                cuts: cuts(*origin, *span),
            },
        };
        Ok(Self {
            carrier,
            v_off: iv(v_off),
            v_sign,
            forward,
            ends,
            env,
        })
    }
}

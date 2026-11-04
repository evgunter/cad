//! **A projected cylinder or sphere image as the rotational lane reads
//! it** ([`ProjectedEdgeQ::of_image`]): the image's carrier and deck map
//! crossed into certification arithmetic, by the caller's lift, at any
//! scalar the image holds.

use geom_core::Real;
use geom_core::interval::Interval;
use geom_core::Bounds;

use super::rotational::{FramedQ, ProjectedEdgeQ, RotationalChartQ};
use crate::pcurve_cache::{FramedCarrier, ProjectedChart, ProjectedImage};
use crate::props::PropsError;
use crate::props::bracket::{AbsEnclosure, exact_point as exact, mid, pt, tau_bracket};
use crate::props::rational_rim::rational_edge_q_split;

impl ProjectedEdgeQ {
    /// **A projected image on `chart` as the lane reads it**: `image`'s
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
    /// not `chart`'s, a deck sign other than exactly `±1`, on the sphere a
    /// deck level that is not a name of the latitude, inexact
    /// carrier interval ends, or a net the curved rims' rule cannot
    /// decompose ([`crate::props::rational_rim::rational_edge_q_split`]).
    pub fn of_image<T: Real>(
        image: &ProjectedImage<T>,
        chart: RotationalChartQ,
        (t0, t1): (Interval, Interval),
        forward: bool,
        ends: [(Interval, Interval); 2],
        env: Interval,
        iv: impl Fn(T) -> Interval,
    ) -> Result<Self, PropsError> {
        let unsupported = |what| PropsError::QuadratureUnsupported { what };
        let sphere = match (image.chart(), chart) {
            (ProjectedChart::Cylinder, RotationalChartQ::Cylinder { .. }) => false,
            (ProjectedChart::Sphere, RotationalChartQ::Sphere { .. }) => true,
            _ => {
                return Err(unsupported(
                    "a projected image on the rotational lane whose inverse is another chart's",
                ));
            }
        };
        let (_, v_off, v_sign) = image.deck();
        let v_sign = match exact(iv(v_sign)) {
            Some(s) if s == 1.0 || s == -1.0 => s,
            _ => {
                return Err(unsupported(
                    "a projected image whose deck map's level sign is not exactly ±1",
                ));
            }
        };
        // The sphere's terms read the latitude off the frame point, which
        // every name of the point shares: `v ↦ v + 2πk` and the twin's
        // `v ↦ π − v + 2πk`, so the deck's level is one of those.
        if sphere && !latitude_name(iv(v_off), v_sign) {
            return Err(unsupported(
                "a projected sphere image whose deck map's level is not a name of the latitude",
            ));
        }
        let (Some(lo), Some(hi)) = (exact(t0), exact(t1)) else {
            return Err(unsupported(
                "a projected edge whose carrier interval ends are not exact parameters",
            ));
        };
        let breaks = image.breaks().interior();
        let vec = |v: geom_core::Vec3<T>| [iv(v.x), iv(v.y), iv(v.z)];
        // A circle's or a line's breaks are on `s = (t − origin)/span`,
        // read where the image placed them: any `f64` cut tiles the
        // interval exactly. They guard no soundness (the rotational
        // lane's module docs, the edge terms): the rule's integrand reads
        // no branch.
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

/// Whether the deck level `v_off + v_sign·v` names the latitude `v`'s
/// point: `v_off` a whole number of turns (`v_sign = 1`) or a half turn
/// past one (`v_sign = −1`, the twin), to within the rounding of the
/// deck's own composition.
fn latitude_name(v_off: Interval, v_sign: f64) -> bool {
    let tau = tau_bracket();
    let base = if v_sign > 0.0 { pt(0.0) } else { pt(0.5) * tau };
    let turns = (v_off - base) / tau;
    let k = mid(turns).round();
    let off = (turns - pt(k)).abs_enclosure();
    off.is_certified() && off.hi() <= 1e-12 * (1.0 + k.abs())
}

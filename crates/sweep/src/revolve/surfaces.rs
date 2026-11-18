//! Placing the classified wall catalog into 3-space, plus the one edge
//! spec that is a revolve's own: the latitude strut/rim, whose carrier
//! is the rotation trajectory of a sketch point. Meridian chain edges
//! and cap points come from the shared lowering
//! ([`crate::swept`]) — a revolve places them, it does not derive
//! them differently.
//!
//! Every revolution surface takes the call's **shared azimuthal
//! frame** ([`AxisFrame::azimuth`]) — `axis = +a₃`, `u_ref = u₃`,
//! re-anchored on the placed axis line via [`AxisFrame::foot3`] and
//! never re-squared — so identical-by-construction surfaces are bitwise
//! identical and every seam sits on the angle-0 meridian (module docs).

use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, MappedSource};
use geom_core::{Point2, Real, Sign};

use super::SweptSeg;
use super::axis::{AxisFrame, WallKind};

/// The wall surface a classified off-axis segment sweeps, in the
/// call's shared azimuthal frame (file docs).
pub(super) fn wall_surface<T: Real>(
    kind: &WallKind<T>,
    seg: &SweptSeg<T>,
    frame: &AxisFrame<T>,
) -> Surface<T> {
    match *kind {
        WallKind::Plane { .. } => Surface::Plane {
            frame: frame.azimuth.with_origin(frame.world(seg.a)),
        },
        WallKind::Cylinder { radius } => Surface::Cylinder {
            frame: frame.azimuth.with_origin(frame.foot3(seg.a)),
            radius,
        },
        WallKind::Cone {
            apex_sk,
            half_angle,
        } => Surface::Cone {
            frame: frame.azimuth.with_origin(frame.foot3(apex_sk)),
            half_angle,
        },
        WallKind::Sphere { center_sk, radius } => Surface::Sphere {
            frame: frame.azimuth.with_origin(frame.foot3(center_sk)),
            radius,
        },
        WallKind::Torus {
            center_sk,
            major,
            minor,
        } => Surface::Torus {
            frame: frame.azimuth.with_origin(frame.foot3(center_sk)),
            major_radius: major,
            minor_radius: minor,
        },
        WallKind::Spindle {
            center_sk,
            major,
            minor,
        } => Surface::Spindle {
            frame: frame.azimuth.with_origin(frame.foot3(center_sk)),
            major_radius: major,
            minor_radius: minor,
        },
    }
}

/// A latitude strut/rim spec: the sketch point's trajectory under the
/// rotation family (`MappedSource::RevolvedPoint`), carrier the
/// latitude circle. `axis_c` is the θ sign of the carrier axis (forward
/// interval `(0, |θ|]`); `u_ref` points at the sketch point placed,
/// which lies on the seam side of the axis (`r > 0`), so it is the
/// frame's `u₃`.
///
/// Not `extrude`'s `extruded_strut_spec`, which mints the same
/// topological edge out of the translation family: a line carrier over
/// `0..w_norm` under an `ExtrudedPoint` description. Neither spec is
/// derivable from the other, so they do not share a name.
pub(super) fn revolved_strut_spec<T: Real>(
    point: Point2<T>,
    radius: T,
    frame: &AxisFrame<T>,
    theta: T,
    axis_c: Sign,
) -> EdgeCurveSpec<T> {
    let center = frame.foot3(point);
    EdgeCurveSpec {
        description: EdgeDescriptionSpec::Scaffold(MappedCurve::whole(
            MappedSource::RevolvedPoint {
                point,
                place: frame.place,
                axis: frame.line3(),
                angle: theta,
            },
        )),
        carrier: frame.latitude(center, radius, axis_c, frame.motion.into()),
        param_start: T::zero(),
        param_end: theta.abs(),
    }
}

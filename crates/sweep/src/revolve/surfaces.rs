//! Placing the classified wall catalog into 3-space, plus the one edge
//! spec that is a revolve's own: the latitude strut/rim, whose carrier
//! is the rotation trajectory of a sketch point. Meridian chain edges
//! and cap points come from the shared lowering
//! ([`crate::swept`]) — a revolve places them, it does not derive
//! them differently.
//!
//! Every revolution surface takes the call's **shared azimuthal
//! frame** — `axis = +a₃`, `u_ref = u₃`, anchor on the placed axis
//! line via [`AxisFrame::foot3`] — so identical-by-construction
//! surfaces are bitwise identical and every seam sits on the angle-0
//! meridian (module docs).

use geom::Curve3;
use geom::Surface;
use geom_brep::{EdgeCurveSpec, EdgeGeometry, MappedCurve};
use geom_core::{Point2, Point3, Real, Vec3};

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
        WallKind::Plane => Surface::Plane {
            origin: frame.world(seg.a),
            normal: frame.a3,
            u_ref: frame.u3,
        },
        WallKind::Cylinder { radius } => Surface::Cylinder {
            origin: frame.foot3(seg.a),
            axis: frame.a3,
            radius,
            u_ref: frame.u3,
        },
        WallKind::Cone {
            apex_sk,
            half_angle,
        } => Surface::Cone {
            apex: frame.foot3(apex_sk),
            axis: frame.a3,
            half_angle,
            u_ref: frame.u3,
        },
        WallKind::Sphere { center_sk, radius } => Surface::Sphere {
            center: frame.foot3(center_sk),
            radius,
            axis: frame.a3,
            u_ref: frame.u3,
        },
        WallKind::Torus {
            center_sk,
            major,
            minor,
        } => Surface::Torus {
            center: frame.foot3(center_sk),
            axis: frame.a3,
            major_radius: major,
            minor_radius: minor,
            u_ref: frame.u3,
        },
    }
}

/// A latitude strut/rim spec: the sketch point's trajectory under the
/// rotation family (`MappedCurve::RevolvedPoint`), carrier the
/// latitude circle. `axis_c` is the θ-signed carrier axis (forward
/// interval `(0, |θ|]`; `u_ref` points at the start point `q` — the
/// carrier-frame convention, distinct from the surfaces' shared `u₃`).
///
/// The extrude family's strut spec is `extrude`'s
/// `extruded_strut_spec`: a different description, a different carrier
/// class and a different parameter interval, sharing only the
/// topological role. The two carry distinct names because a reader who
/// greps one must not be handed the other's contract.
pub(super) fn revolved_strut_spec<T: Real>(
    point: Point2<T>,
    radius: T,
    q: Point3<T>,
    frame: &AxisFrame<T>,
    theta: T,
    axis_c: Vec3<T>,
) -> EdgeCurveSpec<T> {
    let center = frame.foot3(point);
    EdgeCurveSpec {
        description: EdgeGeometry::MappedCurve(MappedCurve::RevolvedPoint {
            point,
            place: frame.place,
            axis_origin: frame.o3,
            axis_dir: frame.a3,
            angle: theta,
        }),
        carrier: Curve3::Circle {
            center,
            axis: axis_c,
            radius,
            u_ref: (q - center).normalize(),
        },
        param_start: T::zero(),
        param_end: theta.abs(),
    }
}

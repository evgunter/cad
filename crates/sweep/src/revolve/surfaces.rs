//! Placing the classified wall catalog into 3-space, plus the edge
//! specs a revolve mints: meridian chain edges (`PlacedSegment`),
//! latitude struts/rims (`RevolvedPoint` circles), cap points.
//!
//! Every revolution surface takes the call's **shared azimuthal
//! frame** — `axis = +a₃`, `u_ref = u₃`, anchor on the placed axis
//! line via [`AxisFrame::foot3`] — so identical-by-construction
//! surfaces are bitwise identical and every seam sits on the angle-0
//! meridian (module docs).

use geom_brep::{EdgeCurveSpec, EdgeGeometry, MappedCurve};
use geom_core::{Affine3, Band, Decide, Indeterminate, Point2, Point3, Real, Sign, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

use super::axis::{AxisFrame, WallKind};
use super::{SweptKind, SweptSeg, arc_span, decide, turn_axis};

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
pub(super) fn strut_spec<T: Real>(
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

/// A meridian chain-edge spec (`PlacedSegment` under `place_eff`):
/// line or circle carrier per the extrude carrier conventions (arc
/// axis = turn-signed effective plane normal, span θ = 4·atan|bulge|).
/// The start chain passes the sketch placement; a partial end chain
/// passes the rotated placement (and rotated points/normal); a full
/// revolve's copied chain passes the sketch placement verbatim (full
/// period is definitionally the identity — module docs).
pub(super) fn chain_spec<T: Real>(
    seg: &SweptSeg<T>,
    place_eff: Affine3<T>,
    normal_eff: Vec3<T>,
    q_from: Point3<T>,
    q_to: Point3<T>,
) -> EdgeCurveSpec<T> {
    let description = EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
        segment: seg.sketch_segment(),
        place: place_eff,
    });
    match seg.kind {
        SweptKind::Line => EdgeCurveSpec {
            description,
            carrier: Curve3::Line {
                origin: q_from,
                dir: (q_to - q_from).normalize(),
            },
            param_start: T::zero(),
            param_end: q_from.distance(q_to),
        },
        SweptKind::Arc {
            center,
            radius,
            turn,
        } => {
            let c_world = place_eff.transform_point(Point3::new(center.x, center.y, T::zero()));
            EdgeCurveSpec {
                description,
                carrier: Curve3::Circle {
                    center: c_world,
                    axis: turn_axis(turn, normal_eff),
                    radius,
                    u_ref: (q_from - c_world).normalize(),
                },
                param_start: T::zero(),
                param_end: arc_span(seg.bulge),
            }
        }
    }
}

/// The points determining a wedge cap plane, in forward swept order:
/// every loop vertex plus every arc segment's apex (keeps 2-vertex
/// loops plane-determining; extrude's `cap_points` mirrored). `qs` are
/// the (possibly rotated) world vertices, `place_eff` the matching
/// placement for apex points.
pub(super) fn cap_points<T: Real>(
    segs: &[SweptSeg<T>],
    qs: &[Point3<T>],
    place_eff: Affine3<T>,
) -> Vec<Point3<T>> {
    let mut pts = Vec::with_capacity(segs.len() * 2);
    for (j, s) in segs.iter().enumerate() {
        pts.push(qs[j]);
        if matches!(s.kind, SweptKind::Arc { .. }) {
            let apex = super::arc_apex(s);
            pts.push(place_eff.transform_point(Point3::new(apex.x, apex.y, T::zero())));
        }
    }
    pts
}

/// Decides whether two consecutive segments sweep the
/// identical-by-construction revolution surface: collinear lines share
/// one plane/cylinder/cone, same-carrier same-turn arcs share one
/// sphere/torus — revolution surfaces of identical generators about
/// one axis coincide, so the sketch-level margins are exactly
/// extrude's (mirrored; named for this crate's funnel). Mixed kinds
/// and opposite turns never share (structural).
pub(super) fn cosurface<T: Decide>(
    prev: &SweptSeg<T>,
    next: &SweptSeg<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    match (&prev.kind, &next.kind) {
        (SweptKind::Line, SweptKind::Line) => {
            // Margin: perpendicular distance of the next chord's far
            // endpoint from the previous chord's carrier line.
            let t = (prev.b - prev.a).normalize();
            let d = next.b - prev.a;
            let margin = t.perp_dot(d);
            Ok(matches!(
                decide("wall_lines_cosurface", margin, band)?,
                Sign::Zero
            ))
        }
        (
            SweptKind::Arc {
                center: c1,
                radius: r1,
                turn: t1,
            },
            SweptKind::Arc {
                center: c2,
                radius: r2,
                turn: t2,
            },
        ) => {
            if t1 != t2 {
                return Ok(false);
            }
            // Margin: center distance plus radius difference (meters).
            let margin = c1.distance(*c2) + (*r1 - *r2).abs();
            Ok(matches!(
                decide("wall_arcs_cosurface", margin, band)?,
                Sign::Zero
            ))
        }
        _ => Ok(false),
    }
}

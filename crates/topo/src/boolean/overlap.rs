//! **Whether two faces on one carrier meet in area** ([`overlap`]).
//!
//! The two faces are ones a verdict already put on one carrier, in one
//! sense: two faces of an n-ary union's members a merge glued, or a kept
//! face and the coincident face it holds. The door never re-decides that.
//! It looks for a witness that their regions share area, of three kinds:
//!
//! - a vertex of one face strictly inside the other (the containment
//!   doors: [`contfp`] on a plane, [`curved_face_placement`] on a curved
//!   carrier);
//! - a point of one face's boundary edge strictly inside the other: each
//!   edge is cut where it meets the other face's boundary (the curve ×
//!   face boundary pass, [`geom_brep::curve_face::read`], which reads curve against
//!   curve on any carrier) and each piece's middle is placed, so a proper
//!   crossing of two boundaries is found as the piece of either edge that
//!   runs inside the other face;
//! - a boundary point of one face lying on the other's boundary, at a
//!   vertex or along an edge, where the two faces' corners there overlap,
//!   read as the boolean's fate cuts read a direction against a corner
//!   (`fate::corner_holds`): an inscribed triangle in a hexagon has
//!   no vertex strictly inside and no crossing, and two faces meeting
//!   along a shared edge from one side are the same corner.
//!
//! With no witness the regions share no area: a positive-area meeting of
//! two regions either holds a boundary point of one inside the other, or
//! is bounded only by boundary they share, whose corners the third kind
//! reads. Every margin is decided through `k_stats` at the caller's band;
//! one in band refuses typed ([`OverlapRefusal`]), as does a face on a
//! spline carrier, whose trim is read on its chart.

use geom_brep::LeverBounds;
use geom_brep::curve_face::BoundaryRefusal;
use geom_brep::curve_pair::{Hit, Span};
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};

use super::BooleanError;
use super::contain::{
    ContainError, CurvedPlacement, FaceContainment, contfp, curved_face_placement,
};
use super::fate::{Holds, Leaving, corner_holds, departure};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::live::BoundaryMember;

/// **Why [`overlap`] gave no answer.**
#[derive(Clone, Debug)]
pub enum OverlapRefusal {
    /// A face lies on a spline carrier: its trim is read on its chart,
    /// which this door does not read.
    SplineCarrier {
        /// The face.
        face: FaceKey,
    },
    /// A face key that does not resolve in its body.
    StaleFace {
        /// The face.
        face: FaceKey,
    },
    /// A point's placement in a face refused: in band, or a reading of
    /// the containment door with no verdict.
    Placement(ContainError),
    /// The boundary pass over an edge refused: a meeting in band, or a
    /// reading that could not run.
    Meeting(BoundaryRefusal),
    /// Two corners' bounds read in band against each other.
    Corner(std::sync::Arc<BooleanError>),
    /// A reading the door has no verdict for: what it is.
    Unread {
        /// The reading.
        what: &'static str,
    },
}

impl core::fmt::Display for OverlapRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SplineCarrier { face } => write!(
                f,
                "face {face:?} lies on a spline carrier, whose trim the overlap door does not read"
            ),
            Self::StaleFace { face } => write!(f, "face {face:?} does not resolve"),
            Self::Placement(e) => write!(f, "placing a point in a face: {e}"),
            Self::Meeting(e) => write!(f, "reading an edge against a face's boundary: {e:?}"),
            Self::Corner(e) => write!(f, "reading two corners against each other: {e}"),
            Self::Unread { what } => write!(f, "no verdict: {what}"),
        }
    }
}

impl core::error::Error for OverlapRefusal {}

/// **Whether faces `a` and `b`, which a verdict put on one carrier in one
/// sense, meet in positive area** (module docs). The two may be faces of
/// two bodies.
///
/// # Errors
///
/// [`OverlapRefusal`]: a decision in band, a face on a spline carrier, a
/// key that does not resolve, or a reading with no verdict.
///
/// # Panics
///
/// Where a record past a resolved face does not resolve (D2 row 4).
pub fn overlap<T: Decide>(
    a: (&Body<T>, FaceKey),
    b: (&Body<T>, FaceKey),
    band: Band,
) -> Result<bool, OverlapRefusal> {
    for (body, face) in [a, b] {
        let f = body
            .get_face(face)
            .ok_or(OverlapRefusal::StaleFace { face })?;
        if matches!(
            body.face_surface_linked(face, f).kind(),
            geom::SurfaceKind::Nurbs | geom::SurfaceKind::Approx
        ) {
            return Err(OverlapRefusal::SplineCarrier { face });
        }
    }
    for (x, y) in [(a, b), (b, a)] {
        if vertex_witness(x, y, band)? || edge_witness(x, y, band)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Where point `q`, on face `face`'s carrier, lies against the face.
fn place<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
    q: Point3<T>,
    band: Band,
) -> Result<FaceContainment, OverlapRefusal> {
    let f = body
        .get_face(face)
        .ok_or(OverlapRefusal::StaleFace { face })?;
    match *body.face_surface_linked(face, f) {
        geom::Surface::Plane { normal, .. } => {
            contfp(body, face, normal, q, band).map_err(OverlapRefusal::Placement)
        }
        _ => match curved_face_placement(body, face, q, band).map_err(OverlapRefusal::Placement)? {
            CurvedPlacement::Trim(Some(at)) => Ok(at),
            CurvedPlacement::Trim(None) => Err(OverlapRefusal::Unread {
                what: "the curved face's region reading has no verdict for the point",
            }),
            CurvedPlacement::OffCarrier => Err(OverlapRefusal::Unread {
                what: "a point of one face lies definitely off the other's carrier, which a \
                       verdict put both on",
            }),
        },
    }
}

/// Each loop of `face`, as its edge half-edges in walk order.
fn loops<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
) -> Result<Vec<Vec<HalfEdgeKey>>, OverlapRefusal> {
    let f = body
        .get_face(face)
        .ok_or(OverlapRefusal::StaleFace { face })?;
    let mut out = Vec::new();
    for (_, members) in body.face_boundary_by_loop(face, f) {
        let mut cycle = Vec::with_capacity(members.len());
        for member in members {
            match member {
                BoundaryMember::Edge { he, .. } => cycle.push(he),
                BoundaryMember::Isolated { .. } => {
                    return Err(OverlapRefusal::Unread {
                        what: "a face holding a lone vertex",
                    });
                }
            }
        }
        out.push(cycle);
    }
    Ok(out)
}

/// A half-edge's start vertex and its point.
fn start<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> (VertexKey, Point3<T>) {
    let h = body.get_half_edge(he).unwrap_or_else(|| {
        unreachable!("half-edge {he:?} of a resolved face's loop does not resolve")
    });
    let p = body.half_edge_start_point(he).unwrap_or_else(|| {
        unreachable!("half-edge {he:?} of a resolved face's loop has no start point")
    });
    (h.start, p)
}

/// The corner of `face` at its vertex `v`: each, where its loops pass `v`
/// more than once.
fn vertex_corners<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
    v: VertexKey,
) -> Result<Vec<(Leaving<T>, Leaving<T>)>, OverlapRefusal> {
    let unread = || OverlapRefusal::Unread {
        what: "a corner bound with no certified curve",
    };
    let mut out = Vec::new();
    for cycle in loops((body, face))? {
        for (i, &he) in cycle.iter().enumerate() {
            if start(body, he).0 != v {
                continue;
            }
            let into = cycle[(i + cycle.len() - 1) % cycle.len()];
            let back = body.mate(into).ok_or_else(unread)?;
            out.push((
                departure(body, he).ok_or_else(unread)?,
                departure(body, back).ok_or_else(unread)?,
            ));
        }
    }
    Ok(out)
}

/// The corner of `face` at `p`, a point inside its boundary edge `e`: the
/// half-turn on the face's side of the edge, once per half-edge of the
/// face's loops along `e`.
fn edge_corners<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
    e: EdgeKey,
    p: Point3<T>,
) -> Result<Vec<(Leaving<T>, Leaving<T>)>, OverlapRefusal> {
    let mut out = Vec::new();
    for cycle in loops((body, face))? {
        for &he in &cycle {
            if body.get_half_edge(he).is_some_and(|h| h.edge == e) {
                out.push(half_plane(body, he, p)?);
            }
        }
    }
    Ok(out)
}

/// The half-turn on the left of half-edge `he` at `p`, a point inside its
/// edge: the edge's tangent there forward, then back.
fn half_plane<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
    p: Point3<T>,
) -> Result<(Leaving<T>, Leaving<T>), OverlapRefusal> {
    let unread = |what| OverlapRefusal::Unread { what };
    let h = body
        .half_edges
        .get(he)
        .ok_or_else(|| unread("a boundary half-edge that does not resolve"))?;
    let edge = body
        .get_edge(h.edge)
        .ok_or_else(|| unread("a boundary edge that does not resolve"))?;
    let curve = body
        .get_curve_geom(edge.curve)
        .and_then(|c| c.certified())
        .ok_or_else(|| unread("a boundary edge with no certified curve"))?;
    let carrier = curve.carrier();
    let (t0, t1) = curve.params();
    let t = carrier
        .param_near(p, geom::mid_param(t0, t1))
        .ok_or_else(|| unread("a boundary edge whose carrier places no parameter"))?;
    let d = carrier.deriv(t);
    let speed = d.norm();
    let forward = he == edge.he_plus;
    let unit = if forward { d / speed } else { -(d / speed) };
    let bend = match carrier {
        geom::Curve3::Line(_) => Some(Vec3::new(T::zero(), T::zero(), T::zero())),
        geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
            let dd = carrier.deriv2(t);
            let u = d / speed;
            Some((dd - u * dd.dot(u)) / (speed * speed))
        }
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => None,
    };
    let reach = |ta: T, tb: T| match carrier {
        geom::Curve3::Line(_) => LeverBounds::Exact((carrier.eval(tb) - p).norm()),
        _ => geom_brep::edge_reach_bounds(carrier, ta, tb, p),
    };
    let (ahead, behind) = if forward { (t1, t0) } else { (t0, t1) };
    Ok((
        Leaving {
            dir: unit,
            reach: reach(t, ahead),
            bend,
        },
        Leaving {
            dir: -unit,
            reach: reach(t, behind),
            bend,
        },
    ))
}

/// The corners of face `y` at `q`, where `at` places `q` on its
/// boundary; none for a point strictly inside or outside.
fn corners_at<T: Decide>(
    y: (&Body<T>, FaceKey),
    at: FaceContainment,
    q: Point3<T>,
) -> Result<Vec<(Leaving<T>, Leaving<T>)>, OverlapRefusal> {
    match at {
        FaceContainment::OnVertex(v) => vertex_corners(y, v),
        FaceContainment::OnEdge(e) => edge_corners(y, e, q),
        FaceContainment::In | FaceContainment::Out => Ok(Vec::new()),
    }
}

/// Whether some corner of `xs` and some of `ys`, both at `p` on face
/// `x`'s carrier, overlap.
fn any_corners_meet<T: Decide>(
    x: (&Body<T>, FaceKey),
    xs: &[(Leaving<T>, Leaving<T>)],
    ys: &[(Leaving<T>, Leaving<T>)],
    p: Point3<T>,
    band: Band,
) -> Result<bool, OverlapRefusal> {
    if xs.is_empty() || ys.is_empty() {
        return Ok(false);
    }
    let n = crate::face_normal::face_outward_normal_at(x.0, x.1, p, band)
        .map_err(|_| OverlapRefusal::Unread {
            what: "the face's outward normal at a boundary point",
        })?
        .ok_or(OverlapRefusal::StaleFace { face: x.1 })?
        .vec();
    for &cx in xs {
        for &cy in ys {
            if corners_meet(cx, cy, n, band)? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// **Whether two open corners at one point meet**, each swept
/// counterclockwise about `n` from its first bound to its second: one's
/// bound lies strictly inside the other, or the two share a first bound
/// or a second, read one order down where they leave along one ray.
/// Two corners that meet do one of these, since the ends of their
/// meeting are bounds of one lying inside or on the other.
fn corners_meet<T: Decide>(
    x: (Leaving<T>, Leaving<T>),
    y: (Leaving<T>, Leaving<T>),
    n: Vec3<T>,
    band: Band,
) -> Result<bool, OverlapRefusal> {
    for (d, first, corner) in [
        (y.0, true, x),
        (y.1, false, x),
        (x.0, true, y),
        (x.1, false, y),
    ] {
        match corner_holds(d.ray(), corner, || Ok(false), n, band)
            .map_err(|e| OverlapRefusal::Corner(std::sync::Arc::new(e)))?
        {
            Holds::In => return Ok(true),
            Holds::Tied { along_out } if along_out == first => return Ok(true),
            Holds::Out | Holds::Tied { .. } => {}
            Holds::Along => {
                return Err(OverlapRefusal::Unread {
                    what: "two corner bounds leave along one ray with no bend to part them",
                });
            }
        }
    }
    Ok(false)
}

/// Whether a vertex of face `x` is a witness in face `y`: strictly
/// inside it, or on its boundary where their corners meet.
fn vertex_witness<T: Decide>(
    x: (&Body<T>, FaceKey),
    y: (&Body<T>, FaceKey),
    band: Band,
) -> Result<bool, OverlapRefusal> {
    for cycle in loops(x)? {
        for he in cycle {
            let (v, p) = start(x.0, he);
            let at = place(y, p, band)?;
            if at == FaceContainment::In {
                return Ok(true);
            }
            let ys = corners_at(y, at, p)?;
            if !ys.is_empty() && any_corners_meet(x, &vertex_corners(x, v)?, &ys, p, band)? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// Whether a point inside a boundary edge of face `x` is a witness in
/// face `y`: each edge cut where it meets `y`'s boundary, and each
/// piece's middle placed in `y`, a witness strictly inside or on `y`'s
/// boundary where their corners meet.
fn edge_witness<T: Decide>(
    x: (&Body<T>, FaceKey),
    y: (&Body<T>, FaceKey),
    band: Band,
) -> Result<bool, OverlapRefusal> {
    let boundary = super::carrier_cross::face_boundary(y.0, y.1)
        .map_err(|_| OverlapRefusal::StaleFace { face: y.1 })?
        .ok_or(OverlapRefusal::Unread {
            what: "a boundary edge with no certified curve",
        })?;
    if !boundary.lone.is_empty() {
        return Err(OverlapRefusal::Unread {
            what: "a face holding a lone vertex",
        });
    }
    for cycle in loops(x)? {
        for he in cycle {
            let h = x.0.get_half_edge(he).unwrap_or_else(|| {
                unreachable!("half-edge {he:?} of a resolved face's loop does not resolve")
            });
            let Some(curve) =
                x.0.get_edge(h.edge)
                    .and_then(|e| x.0.get_curve_geom(e.curve))
                    .and_then(|c| c.certified())
            else {
                return Err(OverlapRefusal::Unread {
                    what: "a boundary edge with no certified curve",
                });
            };
            let carrier = curve.carrier();
            let (t0, t1) = curve.params();
            let span = Span::new(carrier, (t0, t1));
            let reading = boundary
                .read(&span, band)
                .map_err(OverlapRefusal::Meeting)?;
            let mut cuts: Vec<T> = reading
                .meetings
                .iter()
                .filter_map(|m| match m.on_span {
                    Hit::On { t, .. } => Some(t),
                    Hit::End(_) => None,
                })
                .collect();
            let speed = carrier.deriv(geom::mid_param(t0, t1)).norm();
            sort_params(&mut cuts, speed, band)?;
            cuts.insert(0, t0);
            cuts.push(t1);
            for w in cuts.windows(2) {
                let q = carrier.eval(geom::mid_param(w[0], w[1]));
                let at = place(y, q, band)?;
                if at == FaceContainment::In {
                    return Ok(true);
                }
                let ys = corners_at(y, at, q)?;
                if !ys.is_empty() && any_corners_meet(x, &[half_plane(x.0, he, q)?], &ys, q, band)?
                {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

/// `params` in ascending order, by decided differences metred at the
/// carrier's `speed`: the scalars carry no order of their own. Two that
/// read equal are one cut.
fn sort_params<T: Decide>(params: &mut Vec<T>, speed: T, band: Band) -> Result<(), OverlapRefusal> {
    let mut out: Vec<T> = Vec::with_capacity(params.len());
    'next: for &p in params.iter() {
        let mut at = out.len();
        for (i, &q) in out.iter().enumerate() {
            match crate::validate::decide(
                "bool_overlap_cut_order",
                Margin::of((p - q) * speed),
                band,
            ) {
                Ok(Sign::Negative) => {
                    at = i;
                    break;
                }
                Ok(Sign::Zero) => continue 'next,
                Ok(Sign::Positive) => {}
                Err(diag) => {
                    return Err(OverlapRefusal::Meeting(BoundaryRefusal::Escalated(diag)));
                }
            }
        }
        out.insert(at, p);
    }
    *params = out;
    Ok(())
}

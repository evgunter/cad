//! **Whether two faces on one carrier meet in area** ([`overlap`]).
//!
//! The two faces are ones a verdict already put on one carrier: two faces
//! of an n-ary union's members a merge glued, or a kept face and the
//! coincident face it holds. The door never re-decides the carrier. It
//! does read the two faces' senses at one point, and refuses two of
//! opposite sense ([`OverlapRefusal::OppositeSenses`]): their sides of a
//! shared boundary are read about one normal, which that premise makes
//! the same.
//!
//! Each boundary edge of either face is cut where it meets the other
//! face's boundary (the curve × face boundary pass,
//! [`geom_brep::curve_face::read`], which reads curve against curve on
//! any carrier), and each piece's middle is placed in the other face
//! through the kernel's one point dispatch (`face_placement`). A piece
//! strictly inside the other face is a witness: the face's area beside
//! it lies in the other's. A piece lying along the other face's boundary
//! is a witness where the two faces lie on one side of it, read as the
//! boolean's fate cuts read a direction against a corner
//! (`fate::corner_holds`).
//!
//! With no witness the regions share no area. Where two regions meet in
//! area, their meeting is bounded by boundary of one or the other; a
//! stretch of one face's boundary inside the other's region is a piece
//! some cut leaves strictly inside it, and a meeting bounded only by
//! boundary the two share has its sides read along that boundary. So a
//! vertex of one face strictly inside the other is never needed: the
//! pieces of its edges beside it are inside too, and are read. The pieces
//! of both faces are read, so a refusal at one piece does not stop the
//! search: the door answers `true` wherever a witness decides, and
//! refuses only where none decides and some reading refused.
//!
//! Every margin is decided through `k_stats` at the caller's band; one in
//! band refuses typed ([`OverlapRefusal`]), as does a face on a spline
//! carrier, whose trim is read on its chart.

use std::sync::Arc;

use geom_brep::LeverBounds;
use geom_brep::curve_face::BoundaryRefusal;
use geom_brep::curve_pair::{Hit, Span};
use geom_core::{Band, Decide, Indeterminate, Lever, Margin, Point3, Sign, Vec3};

use super::BooleanError;
use super::contain::{CurvedPlacement, FaceContainment, PlacementError, face_placement};
use super::fate::{Holds, Leaving, corner_holds, departure};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, VertexKey};
use crate::face_normal::NormalAtError;
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
    /// The two faces face opposite ways where their carrier is read: the
    /// door's premise, one sense, does not hold.
    OppositeSenses,
    /// The two faces' senses read in band against each other.
    Senses(Indeterminate),
    /// A point's placement in a face refused: in band, or a reading of
    /// the containment door with no verdict.
    Placement(super::ContainError),
    /// A point of one face read in band of the other's plane.
    OnPlane(Indeterminate),
    /// The boundary pass over an edge refused: a meeting in band, or a
    /// reading that could not run.
    Meeting(BoundaryRefusal),
    /// A face's boundary could not be read as the boundary pass reads it.
    Boundary(Arc<BooleanError>),
    /// Two corners' bounds read in band against each other.
    Corner(Arc<BooleanError>),
    /// A face's outward normal at a point of its boundary could not be
    /// read.
    Normal(NormalAtError),
    /// A reading the door has no verdict for: what it is.
    Unread {
        /// The reading.
        what: &'static str,
    },
}

impl OverlapRefusal {
    fn write(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        at: Option<geom_brep::recourse::ReadAt>,
    ) -> core::fmt::Result {
        use geom_brep::recourse::read_at;
        match self {
            Self::SplineCarrier { face } => write!(
                f,
                "face {face:?} lies on a spline carrier, whose trim the overlap door does not read"
            ),
            Self::StaleFace { face } => write!(f, "face {face:?} does not resolve"),
            Self::OppositeSenses => {
                write!(f, "the two faces face opposite ways on their one carrier")
            }
            Self::Senses(d) => write!(f, "the two faces' senses read in band: {d}"),
            Self::Placement(e) => write!(f, "placing a point in a face: {}", read_at(e, at)),
            Self::OnPlane(d) => write!(f, "a point read in band of a face's plane: {d}"),
            Self::Meeting(r) => match r {
                BoundaryRefusal::Escalated(d) => {
                    write!(f, "a boundary vertex's place on an edge read in band: {d}")
                }
                BoundaryRefusal::Meeting(d) => {
                    write!(
                        f,
                        "an edge's meeting with a boundary edge read in band: {d}"
                    )
                }
                BoundaryRefusal::Touch(d) => {
                    write!(f, "an edge and a boundary edge meet in band: {d}")
                }
                BoundaryRefusal::Unread(_) => {
                    write!(f, "an edge's meeting with the boundary could not be read")
                }
                BoundaryRefusal::Fault(_) => {
                    write!(f, "the meeting table misread a carrier (a kernel fault)")
                }
            },
            Self::Boundary(e) => {
                write!(f, "reading a face's boundary: {}", read_at(e.as_ref(), at))
            }
            Self::Corner(e) => write!(
                f,
                "reading two corners against each other: {}",
                read_at(e.as_ref(), at)
            ),
            Self::Normal(e) => match e {
                NormalAtError::Escalated { diag, .. } => {
                    write!(f, "a face's normal at a point read in band: {diag}")
                }
                NormalAtError::DegenerateTorus { .. } => {
                    write!(f, "a face's torus has no regular normal on its tube")
                }
                NormalAtError::OffSurface => write!(f, "a point lies off the face's surface"),
                NormalAtError::AtConeApex { .. } => {
                    write!(f, "a point at a cone's apex, where the face has no normal")
                }
                NormalAtError::SplineCrease { .. } | NormalAtError::SplineIrregular { .. } => {
                    write!(f, "a point where a spline face's chart has no one normal")
                }
            },
            Self::Unread { what } => write!(f, "no verdict: {what}"),
        }
    }
}

geom_brep::ended_by_write!(OverlapRefusal);

impl core::error::Error for OverlapRefusal {}

/// **Whether faces `a` and `b`, which a verdict put on one carrier,
/// meet in positive area** (module docs). The two may be faces of two
/// bodies.
///
/// # Errors
///
/// [`OverlapRefusal`]: two faces of opposite sense, a face on a spline
/// carrier, a key that does not resolve, or, where no witness decides, a
/// decision in band or a reading with no verdict.
///
/// # Panics
///
/// Where a record past a resolved face does not resolve (D2 row 4).
pub fn overlap<T: Decide>(
    a: (&Body<T>, FaceKey),
    b: (&Body<T>, FaceKey),
    band: Band,
) -> Result<bool, OverlapRefusal> {
    overlap_as(a, (b, false), band)
}

/// [`overlap`], `b` read in the sense it has in `a`'s frame: as its own
/// body states it, or with `reversed` the other way, as a subtract's
/// result holds the tool's faces. Its region is the same either way; its
/// side of its own boundary, and its normal, turn.
///
/// # Errors
///
/// As [`overlap`].
pub(crate) fn overlap_as<T: Decide>(
    a: (&Body<T>, FaceKey),
    (b, reversed): ((&Body<T>, FaceKey), bool),
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
    one_sense(a, (b, reversed), band)?;
    let mut refused = None;
    for (x, y) in [((a, false), (b, reversed)), ((b, reversed), (a, false))] {
        if edge_witness(x, y, band, &mut refused)? {
            return Ok(true);
        }
    }
    refused.map_or(Ok(false), Err)
}

/// The middle of `a`'s first boundary edge, and a's and b's outward
/// normals there, which must face one way.
fn one_sense<T: Decide>(
    a: (&Body<T>, FaceKey),
    (b, reversed): ((&Body<T>, FaceKey), bool),
    band: Band,
) -> Result<(), OverlapRefusal> {
    let Some(&he) = loops(a)?.iter().flatten().next() else {
        return Err(OverlapRefusal::Unread {
            what: "a face with no boundary edge",
        });
    };
    let (carrier, (t0, t1)) = edge_span(a.0, he)?;
    let p = carrier.eval(geom::mid_param(t0, t1));
    let na = normal_at(a, p, band)?;
    let nb = turned(normal_at(b, p, band)?, reversed);
    match crate::validate::decide(
        "bool_overlap_senses",
        // Unsourced lever: the unit normals' cosine is read at a unit arm,
        // not over the faces' extent; the holdings row owns its source.
        Margin::levered(na.dot(nb), Lever::unsourced(T::one())),
        band,
    ) {
        Ok(Sign::Positive) => Ok(()),
        Ok(Sign::Negative) => Err(OverlapRefusal::OppositeSenses),
        Ok(Sign::Zero) => Err(OverlapRefusal::Unread {
            what: "the two faces' normals stand square to each other, so no one carrier holds both",
        }),
        Err(diag) => Err(OverlapRefusal::Senses(diag)),
    }
}

/// `n`, turned where `reversed`.
fn turned<T: Decide>(n: Vec3<T>, reversed: bool) -> Vec3<T> {
    if reversed { -n } else { n }
}

/// A face's corner as read `reversed`: the same sweep about the turned
/// normal is the other way round about this one.
fn as_read<T: Decide>((out, back): Corner<T>, reversed: bool) -> Corner<T> {
    if reversed { (back, out) } else { (out, back) }
}

/// Face `face`'s outward normal at `p`, a point of its carrier.
fn normal_at<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
    p: Point3<T>,
    band: Band,
) -> Result<Vec3<T>, OverlapRefusal> {
    crate::face_normal::face_outward_normal_at(body, face, p, band)
        .map_err(OverlapRefusal::Normal)?
        .map(geom_brep::OutwardNormal::vec)
        .ok_or(OverlapRefusal::StaleFace { face })
}

/// Where point `q`, on face `face`'s carrier, lies against the face.
fn place<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
    q: Point3<T>,
    band: Band,
) -> Result<FaceContainment, OverlapRefusal> {
    match face_placement(body, face, q, band, "bool_overlap_on_plane") {
        Ok(CurvedPlacement::Trim(Some(at))) => Ok(at),
        Ok(CurvedPlacement::Trim(None)) => Err(OverlapRefusal::Unread {
            what: "the curved face's region reading has no verdict for the point",
        }),
        Ok(CurvedPlacement::OffCarrier) => Err(OverlapRefusal::Unread {
            what: "a point of one face lies definitely off the other's carrier, which a verdict \
                   put both on",
        }),
        Err(PlacementError::OnPlane(diag)) => Err(OverlapRefusal::OnPlane(diag)),
        Err(PlacementError::Door(e)) => Err(OverlapRefusal::Placement(e)),
    }
}

/// An open corner at a point: swept counterclockwise about the face's
/// outward normal from its first bound to its second.
type Corner<T> = (Leaving<T>, Leaving<T>);

/// Each loop of `face`, as its edge half-edges in walk order.
fn loops<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
) -> Result<Vec<Vec<HalfEdgeKey>>, OverlapRefusal> {
    let f = body
        .get_face(face)
        .ok_or(OverlapRefusal::StaleFace { face })?;
    let mut out = Vec::new();
    for (_, members, _) in body.face_boundary_by_loop(face, f) {
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

/// The certified carrier and span of half-edge `he`'s edge.
fn edge_span<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> Result<(&geom::Curve3<T>, (T, T)), OverlapRefusal> {
    let curve = body
        .get_half_edge(he)
        .and_then(|h| body.get_edge(h.edge))
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(|c| c.certified())
        .ok_or(OverlapRefusal::Unread {
            what: "a boundary edge with no certified curve",
        })?;
    Ok((curve.carrier(), curve.params()))
}

/// The corner of `face` at its vertex `v`: each, where its loops pass `v`
/// more than once.
fn vertex_corners<T: Decide>(
    (body, face): (&Body<T>, FaceKey),
    v: VertexKey,
) -> Result<Vec<Corner<T>>, OverlapRefusal> {
    let unread = || OverlapRefusal::Unread {
        what: "a corner bound with no certified curve",
    };
    let mut out = Vec::new();
    for cycle in loops((body, face))? {
        for (i, &he) in cycle.iter().enumerate() {
            if body.get_half_edge(he).is_none_or(|h| h.start != v) {
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
) -> Result<Vec<Corner<T>>, OverlapRefusal> {
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
/// edge: the edge's tangent there forward, then back, each with the
/// edge's curvature vector there.
///
/// `fate::leaving` reads the same at an edge's end, from the conic's own
/// departure there; this reads it at any point of the span, from the
/// carrier's derivatives, so the two do not share a home.
fn half_plane<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
    p: Point3<T>,
) -> Result<Corner<T>, OverlapRefusal> {
    let forward = body
        .get_half_edge(he)
        .and_then(|h| body.get_edge(h.edge))
        .is_some_and(|e| e.he_plus == he);
    let (carrier, (t0, t1)) = edge_span(body, he)?;
    let t = carrier
        .param_near(p, geom::mid_param(t0, t1))
        .ok_or(OverlapRefusal::Unread {
            what: "a boundary edge whose carrier places no parameter",
        })?;
    let d = carrier.deriv(t);
    let speed = d.norm();
    let unit = if forward { d / speed } else { -(d / speed) };
    let bend = match carrier {
        geom::Curve3::Line(_) => Some(Vec3::new(T::zero(), T::zero(), T::zero())),
        geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
            let dd = carrier.deriv2(t);
            let u = d / speed;
            Some((dd - u * dd.dot(u)) / speed.powi(2))
        }
        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => None,
    };
    // The piece from `p` to a vertex: the edge's start at `t0`, its end
    // at `t1`.
    let edge = body
        .get_half_edge(he)
        .and_then(|h| body.get_edge(h.edge))
        .ok_or(OverlapRefusal::Unread {
            what: "a boundary edge that does not resolve",
        })?;
    let vertex = |end| {
        body.half_edge_start_point(end)
            .ok_or(OverlapRefusal::Unread {
                what: "a boundary edge whose vertex does not resolve",
            })
    };
    let (at_t0, at_t1) = (vertex(edge.he_plus)?, vertex(edge.he_minus)?);
    let reach = |ta: T, (tb, end): (T, Point3<T>)| match carrier {
        geom::Curve3::Line(_) => {
            LeverBounds::Exact(geom_brep::ExtentBall::point(carrier.eval(tb)).lever_from(p))
        }
        _ => super::boxes::piece_reach_bounds(carrier, (ta, tb), (p, end), p),
    };
    let (ahead, behind) = if forward {
        ((t1, at_t1), (t0, at_t0))
    } else {
        ((t0, at_t0), (t1, at_t1))
    };
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
) -> Result<Vec<Corner<T>>, OverlapRefusal> {
    match at {
        FaceContainment::OnVertex(v) => vertex_corners(y, v),
        FaceContainment::OnEdge(e) => edge_corners(y, e, q),
        FaceContainment::In | FaceContainment::Out => Ok(Vec::new()),
    }
}

/// **Whether two open corners at one point meet**, each swept
/// counterclockwise about `n` from its first bound to its second: one's
/// bound lies strictly inside the other, or the two share a first bound
/// or a second, read one order down where they leave along one ray.
/// Two corners that meet do one of these, since the ends of their
/// meeting are bounds of one lying inside or on the other.
fn corners_meet<T: Decide>(
    x: Corner<T>,
    y: Corner<T>,
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
            .map_err(|e| OverlapRefusal::Corner(Arc::new(e)))?
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

/// Whether the piece of half-edge `he` of face `x` about `q`, a point
/// inside it that no cut of `y`'s boundary reaches, is a witness in face
/// `y`: strictly inside it, or along its boundary with the two faces on
/// one side.
fn piece_witness<T: Decide>(
    (x, x_rev): ((&Body<T>, FaceKey), bool),
    he: HalfEdgeKey,
    (y, y_rev): ((&Body<T>, FaceKey), bool),
    q: Point3<T>,
    band: Band,
) -> Result<bool, OverlapRefusal> {
    let at = place(y, q, band)?;
    if at == FaceContainment::In {
        return Ok(true);
    }
    let ys = corners_at(y, at, q)?;
    if ys.is_empty() {
        return Ok(false);
    }
    let n = turned(normal_at(x, q, band)?, x_rev);
    let mine = as_read(half_plane(x.0, he, q)?, x_rev);
    for cy in ys {
        if corners_meet(mine, as_read(cy, y_rev), n, band)? {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Whether a piece of a boundary edge of face `x` is a witness in face
/// `y` (module docs). A piece whose reading refused is passed over, the
/// first such refusal kept in `refused`; a boundary of `x` or `y` that
/// cannot be read refuses at once.
fn edge_witness<T: Decide>(
    (x, x_rev): ((&Body<T>, FaceKey), bool),
    (y, y_rev): ((&Body<T>, FaceKey), bool),
    band: Band,
    refused: &mut Option<OverlapRefusal>,
) -> Result<bool, OverlapRefusal> {
    let boundary = super::carrier_cross::face_boundary(y.0, y.1)
        .map_err(|e| OverlapRefusal::Boundary(Arc::new(e)))?
        .ok_or(OverlapRefusal::Unread {
            what: "a boundary edge with no certified curve",
        })?;
    if !boundary.lone.is_empty() {
        return Err(OverlapRefusal::Unread {
            what: "a face holding a lone vertex",
        });
    }
    let mut keep = |r: OverlapRefusal| {
        refused.get_or_insert(r);
    };
    for cycle in loops(x)? {
        for he in cycle {
            let (carrier, (t0, t1)) = edge_span(x.0, he)?;
            let span = Span::new(carrier, (t0, t1));
            let reading = match boundary.read(&span, band) {
                Ok(reading) => reading,
                Err(r) => {
                    keep(OverlapRefusal::Meeting(r));
                    continue;
                }
            };
            let mut cuts: Vec<T> = reading
                .meetings
                .iter()
                .filter_map(|m| match m.on_span {
                    Hit::On { t, .. } => Some(t),
                    Hit::End(_) => None,
                })
                .collect();
            let speed = carrier.deriv(geom::mid_param(t0, t1)).norm();
            if let Err(r) = sort_params(&mut cuts, speed, band) {
                keep(r);
                continue;
            }
            cuts.insert(0, t0);
            cuts.push(t1);
            for w in cuts.windows(2) {
                let q = carrier.eval(geom::mid_param(w[0], w[1]));
                match piece_witness((x, x_rev), he, (y, y_rev), q, band) {
                    Ok(true) => return Ok(true),
                    Ok(false) => {}
                    Err(r) => keep(r),
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

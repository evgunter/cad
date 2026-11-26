//! **An equal cylinder pair's crossing, read through its bisector
//! planes.** For two walls of one radius `r` whose axes `â₁`, `â₂` meet
//! at `p`, a point `x` on wall A has `|w|² − (w·â₁)² = r²` (`w = x − p`),
//! so wall B's residual there factors exactly:
//!
//! ```text
//! |w|² − (w·â₂)² − r² = (w·â₁)² − (w·â₂)² = (w·(â₁ − â₂))·(w·(â₁ + â₂))
//! ```
//!
//! An edge lying on one wall therefore meets the partner wall exactly
//! where it crosses one of the two bisector planes, each crossing a
//! first-order root of a line or a conic against a plane, read by
//! `geom_brep::curve_face::cross` against the plane, landing on the
//! ellipse that plane carries; and it lies on the partner wall where it
//! lies in a plane. Near a node, where the walls are tangent, the quadric
//! reads a seam `δ` off it through a discriminant of order `δ²`; here its
//! two crossings stand `2δ/sin θ` apart, linear in the offset, so what
//! refuses closes to one band about the node.
//!
//! Where the read cannot decide it refuses typed and never hands the span
//! back to the quadric door (C5): two crossings, one on each plane, within
//! the zero band of each other are the edge passing through a node
//! (`NodeUnbuilt`), and in the band's gap they escalate; a conic whose
//! plane is within the band of a bisector plane's, but not decided on the
//! partner wall, escalates; a carrier with no plane lane is the
//! frontier.

use geom_core::{Band, Decide, Margin, Point3, Sign};

use super::{CurvedEvent, Landing, Placement, edge_face_read, landing, vertex_on_curved_face};
use crate::body::Body;
use crate::boolean::{BooleanDecision, BooleanError, Coincide, Operand, node};
use crate::entity::{Edge, EdgeKey, FaceKey, VertexKey};
use crate::validate::decide;

/// The predicate a conic lying in a bisector plane is decided on the
/// partner wall by: its farthest point's distance from the plane, times the
/// plane's lever, in metres off the partner.
pub(in crate::boolean) const FACTORED_IN_BISECTOR: &str = "bool_factored_in_bisector";

/// The predicate an end's distance from a bisector plane is decided on:
/// whether the end lies on the partner wall, read through the planes.
pub(in crate::boolean) const FACTORED_PLANE_SIDE: &str = "bool_factored_plane_side";

/// **The crossing of `face`, a wall of an equal cylinder pair with
/// meeting axes, by an edge of `x` lying on the pair's other wall**,
/// read through the two bisector planes (module docs). `Ok(None)` is no
/// such pair: the face and the edge's walls are not an equal cylinder
/// pair with meeting axes.
///
/// A conic lying in a bisector plane, decided on the partner wall in
/// metres (`bool_factored_in_bisector`: its farthest point from the plane
/// times the plane's lever, `2|â₁·n̂|`), lies on the partner wall, and is
/// answered as the curved lane answers an arc lying on a face's carrier
/// ([`lying_in_bisector`]). Otherwise the answer takes the quadric route's
/// arms. Of the crossings
/// strictly inside the span, taken in plane order (the `â₁ − â₂` plane's
/// first, then the other's), the first the trim places in the face (on
/// its boundary included) is a pierce, which the sweep splits and
/// re-queues; with none, each end on a plane, so on the partner wall,
/// is placed in the face (the endpoint posture,
/// [`vertex_on_curved_face`]), and the span is eventless here when every
/// such end is placed elsewhere. An end on both planes is a node at an
/// operand vertex: where the face holds it, it refuses as a node on the
/// boundary.
///
/// # Errors
///
/// [`BooleanError::NodeUnbuilt`]: an end at a node the face does not
/// place outside it (`OnBoundary`), or two crossings within the zero band
/// of each other (`GermOnNode`); [`BooleanError::Escalated`]: two
/// crossings whose distance is in the gap (`VertexOnVertex`), a plane's
/// pose or root in band, or a conic in band of lying on the partner wall
/// (`EdgeOnCurvedFace`); an end's plane distance in band
/// (`VertexOnCurvedFace`); a landing the containment door escalates; the
/// typed frontier where a landing or an end has no verdict, or the carrier
/// has no plane lane.
#[allow(clippy::too_many_arguments)]
pub(super) fn factored_crossing<T: Decide + crate::props::AtRestPolicy>(
    (x, x_is): (&Body<T>, Operand),
    (edge_key, edge, carrier, (t0, t1)): (EdgeKey, &Edge, &geom::Curve3<T>, (T, T)),
    [(u, pu), (v, pv)]: [(VertexKey, Point3<T>); 2],
    (y, face): (&mut Body<T>, FaceKey),
    declared: &super::super::GluedPairs<T>,
    contacts: &mut super::ContactAcc,
    frontier: impl Fn() -> BooleanError,
    band: Band,
    tol: geom_core::Tol,
) -> Result<Option<CurvedEvent<T>>, BooleanError> {
    let walls = [edge.he_plus, edge.he_minus].map(|he| x.face_of_half_edge(he));
    let mut found = None;
    for wall in walls.into_iter().flatten() {
        if let Some(p) = node::bisectors(&declared.sections, (x, x_is, wall), (y, face))? {
            found = Some((wall, p));
            break;
        }
    }
    let Some((wall, planes)) = found else {
        return Ok(None);
    };
    let read = |which| edge_face_read(x, x_is, edge, face, declared, which);
    let on_face = |diag| {
        let which = Coincide::VertexOnCurvedFace;
        BooleanError::coincidence(which, read(which), diag)
    };
    // Each end's signed distance from each plane, decided: an end on
    // either plane lies on the partner wall.
    let dist = |pw: Point3<T>, (centre, normal): node::Bisector<T>| {
        geom_core::k_stats::decide_reported(
            FACTORED_PLANE_SIDE,
            Margin::of((pw - centre).dot(normal)),
            band,
        )
        .map_err(on_face)
    };
    let ends_of = || -> Result<[[geom_core::Decided; 2]; 2], BooleanError> {
        Ok([
            [dist(pu, planes[0])?, dist(pu, planes[1])?],
            [dist(pv, planes[0])?, dist(pv, planes[1])?],
        ])
    };
    let line = matches!(carrier, geom::Curve3::Line(_));
    // A carrier with no root lane against a plane (a spiric, a spline):
    // the factored read cannot read its crossings, and C5 hands no span
    // back to the quadric door, so it refuses typed, at the frontier.
    if !line && carrier.conic().is_none() {
        ends_of()?;
        return Err(frontier());
    }
    // **A line's ends are decided first, a conic's after its span**: a
    // line's posture against a plane is its ends' sides, as on the planar
    // lane. A conic's ends are read only once the span is not lying in a
    // plane: a conic the plane's pose reads lying in it has its ends on the
    // plane by that verdict, though an end may round to a reading in the
    // band's gap (a point of the conic read about `2ε` off), which would
    // escalate a span the pose has decided, or disagree with it.
    let line_ends = if line { Some(ends_of()?) } else { None };
    let span = geom_brep::curve_pair::Span::new(carrier, (t0, t1));
    // The edge's own wall's axis, the lever's datum.
    let axis = x
        .proven_face(wall)
        .expect_described()
        .axis_line()
        .ok_or(BooleanError::ClassificationInvariant {
            what: "the factored read's wall has an axis",
        })?
        .dir
        .get();
    // A window of a plane's answer, as the planar lane reads one
    // ([`super::plane_refused`]): a rung in band is the edge's coincidence
    // with the plane, and a root the placement cannot place or order its
    // crossing decision.
    let refused = |why| {
        super::plane_refused::<T>(
            why,
            |diag| BooleanError::Escalated {
                decision: BooleanDecision::Coincidence(
                    Coincide::EdgeOnCurvedFace,
                    read(Coincide::EdgeOnCurvedFace),
                ),
                diag,
            },
            &frontier,
        )
    };
    // Each plane's crossings strictly inside the span, read off the
    // carrier by `cross` against the plane (C5: the one reading of a
    // crossing): a conic's roots, and a line's one root, posed from the
    // span's own data rather than from a far line origin.
    let mut roots: [Vec<T>; 2] = [Vec::new(), Vec::new()];
    let mut lies = None;
    for (i, (centre, normal)) in planes.into_iter().enumerate() {
        use geom_brep::curve_face::{Crossings, OnSpan, Pose};
        let plane = geom::Surface::Plane {
            // Its reference direction the wall's axis less its part along
            // the normal: no bisector plane holds the axis's normal, so the
            // two are never parallel (`|â₁·n̂| = |â₁ ∓ â₂|/2 < 1`).
            frame: geom::Frame::new(centre, normal, axis - normal * axis.dot(normal)),
        };
        let inside = match geom_brep::curve_face::cross(&span, &plane, band) {
            Ok(Crossings {
                pose: Pose::Apart, ..
            }) => Vec::new(),
            // **The span in the plane lies on the partner wall** (C5): on
            // its own wall the partner's residual is the product of its
            // distances from the two planes, and one of them is zero all
            // along it. "In the plane" is the plane's pose's `Lies`
            // (`curve_face_pose_plane_parallel`, then `_offset`, decided in
            // metres); a point `δ` off the bisector plane `Π₋` stands
            // `δ·2cos(θ/2)` off the partner wall, and off `Π₊` `δ·2sin(θ/2)`,
            // `θ` the angle between the axes. A line in the plane has its
            // ends read `Zero` on it, and the endpoint posture below reads
            // them; a conic in it is answered as the curved lane answers
            // an arc lying on the face's carrier, below.
            Ok(Crossings {
                pose: Pose::Lies, ..
            }) => {
                if let Some(conic) = carrier.conic() {
                    // **On the partner wall, decided in metres there**
                    // (`bool_factored_in_bisector`): the conic's farthest
                    // point from the plane, `|d₀| + r` over its whole turn
                    // (`d₀` its centre's offset, `r` the amplitude of its
                    // height along the normal, `|d₀| = r − belly` read
                    // through the conic), times the plane's lever, the
                    // partner's distance per metre off the plane,
                    // `|â₁ ∓ â₂| = 2|â₁·n̂|` (`2cos(θ/2)` or `2sin(θ/2)`):
                    // in the zero band the span lies on the partner wall;
                    // in the gap the coincidence escalates; past the band
                    // the plane's pose and the partner disagree, which keeps
                    // the frontier.
                    let (cp, sp) = conic.along(normal);
                    let r = (cp.powi(2) + sp.powi(2)).sqrt();
                    let far = r + (r - conic.belly_along(centre, normal));
                    let lever = axis.dot(normal).abs() * T::from_f64(2.0);
                    match geom_core::k_stats::decide_reported(
                        FACTORED_IN_BISECTOR,
                        Margin::of(far * lever),
                        band,
                    ) {
                        Ok(d) if d.sign == Sign::Zero => lies = Some(i),
                        Ok(_) => return Err(frontier()),
                        Err(diag) => {
                            return Err(BooleanError::Escalated {
                                decision: BooleanDecision::Coincidence(
                                    Coincide::EdgeOnCurvedFace,
                                    read(Coincide::EdgeOnCurvedFace),
                                ),
                                diag,
                            });
                        }
                    }
                }
                Vec::new()
            }
            Ok(Crossings { meetings, .. }) => meetings
                .into_iter()
                .filter_map(|m| match m.on_span {
                    OnSpan::Inside { t } => Some(t),
                    OnSpan::End(_) => None,
                })
                .collect(),
            Err(refusal) => {
                // Every window refuses: the plane is no face whose trim
                // could set one aside.
                refused(refusal.leading().why)?;
                Vec::new()
            }
        };
        if let geom::Curve3::Line(_) = carrier {
            // **A line's ends decide its posture against the plane**, as on
            // the planar lane: a crossing strictly inside the span exactly
            // where they straddle it. An end the record puts on the plane
            // owns the meeting there, so a root just inside that end is the
            // end's, never a split beside it.
            let ends = line_ends.ok_or(BooleanError::ClassificationInvariant {
                what: "a line's ends were not read against the bisector planes",
            })?;
            let straddle = matches!(
                (ends[0][i].sign, ends[1][i].sign),
                (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive)
            );
            if straddle {
                if inside.is_empty() {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "a line whose ends straddle a bisector plane was answered with no \
                               crossing strictly inside it",
                    });
                }
                roots[i] = inside;
            }
        } else {
            roots[i] = inside;
        }
    }
    if let Some(i) = lies {
        return lying_in_bisector(
            (x, x_is, edge_key, edge),
            [(u, pu), (v, pv)],
            (y, face, wall),
            planes[1 - i],
            (dist, &frontier),
            contacts,
            band,
            tol,
        )
        .map(Some);
    }
    let ends = match line_ends {
        Some(ends) => ends,
        None => ends_of()?,
    };
    // A crossing of one plane within the band of a crossing of the
    // other is the edge passing through a node.
    for &t in &roots[0] {
        for &s in &roots[1] {
            match decide(
                "bool_factored_crossings_apart",
                Margin::norm3(carrier.eval(t) - carrier.eval(s)),
                band,
            ) {
                Ok(Sign::Positive) => {}
                Ok(Sign::Zero | Sign::Negative) => {
                    let (a_face, b_face) = match x_is {
                        Operand::A => (wall, face),
                        Operand::B => (face, wall),
                    };
                    return Err(BooleanError::NodeUnbuilt {
                        a_face,
                        b_face,
                        why: crate::boolean::NodeWhy::GermOnNode,
                    });
                }
                Err(diag) => {
                    return Err(BooleanError::Escalated {
                        decision: BooleanDecision::VertexOnVertex,
                        diag,
                    });
                }
            }
        }
    }
    for (i, &t) in roots
        .iter()
        .enumerate()
        .flat_map(|(i, ts)| ts.iter().map(move |t| (i, t)))
    {
        let p = carrier.eval(t);
        match landing(y, face, p, band)? {
            Landing::At(at, on) => {
                return Ok(Some(CurvedEvent::Pierce {
                    t,
                    p,
                    at,
                    on,
                    plane: Some(i),
                    chart: None,
                    on_edge: None,
                }));
            }
            Landing::Out => {}
            Landing::Unplaced => return Err(frontier()),
        }
    }
    let mut placed = [None, None];
    for (k, (w, pw)) in [(u, pu), (v, pv)].into_iter().enumerate() {
        if k == 1 && v == u {
            break;
        }
        if ends[k].iter().all(|d| d.sign == Sign::Zero) {
            match landing(y, face, pw, band)? {
                Landing::Out => {}
                Landing::At(..) | Landing::Unplaced => {
                    let (a_face, b_face) = match x_is {
                        Operand::A => (wall, face),
                        Operand::B => (face, wall),
                    };
                    return Err(BooleanError::NodeUnbuilt {
                        a_face,
                        b_face,
                        why: crate::boolean::NodeWhy::OnBoundary(x_is),
                    });
                }
            }
        }
        let on = ends[k]
            .into_iter()
            .enumerate()
            .find(|(_, d)| d.sign == Sign::Zero);
        if let Some((i, on)) = on {
            placed[k] = Some(vertex_on_curved_face(
                x_is, y, w, pw, face, on.margin, contacts, band, tol,
            )?);
            if ends[k][1 - i].sign != Sign::Zero {
                contacts.planes.insert((x_is, w), i);
            }
        }
    }
    if placed.iter().all(Option::is_none) {
        return Ok(Some(CurvedEvent::None));
    }
    Placement::undeclared_no_interior(placed)
        .map(Some)
        .ok_or_else(frontier)
}

/// **A conic of `x` lying in bisector plane `i`, so on the partner wall
/// `face`** ([`factored_crossing`]; C5): the curved lane's answer for an
/// arc lying on a face's carrier ([`super::lying_on`]), its parents
/// decided distinct from the face first ([`super::parents_distinct_from`]).
/// Its ends lie on the plane by the span's verdict, so they carry no
/// margin of their own. An end on the other plane too is at a node,
/// where the walls are tangent, and the node pass reads it: one the face
/// does not place outside refuses [`BooleanError::NodeUnbuilt`]. A node
/// strictly inside the span is the lying arc's own reading's
/// ([`super::lying_on`]: where the face's boundary meets the arc).
///
/// # Errors
///
/// [`BooleanError::NodeUnbuilt`] at a node; an end's distance from the
/// other plane in band; a landing's escalation; the frontier where the
/// parents are not decided distinct or the lying arc has no reading.
#[allow(clippy::too_many_arguments)] // one parameter per named duty
fn lying_in_bisector<T: Decide + crate::props::AtRestPolicy>(
    (x, x_is, edge_key, edge): (&Body<T>, Operand, EdgeKey, &Edge),
    [(u, pu), (v, pv)]: [(VertexKey, Point3<T>); 2],
    (y, face, wall): (&mut Body<T>, FaceKey, FaceKey),
    other: node::Bisector<T>,
    (dist, frontier): (
        impl Fn(Point3<T>, node::Bisector<T>) -> Result<geom_core::Decided, BooleanError>,
        &impl Fn() -> BooleanError,
    ),
    contacts: &mut super::ContactAcc,
    band: Band,
    tol: geom_core::Tol,
) -> Result<CurvedEvent<T>, BooleanError> {
    let (a_face, b_face) = match x_is {
        Operand::A => (wall, face),
        Operand::B => (face, wall),
    };
    let node = |why| BooleanError::NodeUnbuilt {
        a_face,
        b_face,
        why,
    };
    for (k, (_, pw)) in [(u, pu), (v, pv)].into_iter().enumerate() {
        if k == 1 && v == u {
            break;
        }
        if dist(pw, other)?.sign == Sign::Zero {
            match landing(y, face, pw, band)? {
                Landing::Out => {}
                Landing::At(..) | Landing::Unplaced => {
                    return Err(node(crate::boolean::NodeWhy::OnBoundary(x_is)));
                }
            }
        }
    }
    if !super::parents_distinct_from(x, edge, y, face, band) {
        return Err(frontier());
    }
    let arc = super::ArcOnCarrier {
        x,
        x_is,
        edge_key,
        ends: [(u, pu, None), (v, pv, None)],
        face,
        side: Coincide::VertexOnCurvedFace,
        band,
        tol,
    };
    super::lying_on(&arc, y, contacts)?.ok_or_else(frontier)
}

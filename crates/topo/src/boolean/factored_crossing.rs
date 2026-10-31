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
//! first-order root of a line or a conic against a plane, the planar
//! lanes' own (`splitting::plane_crossing_lane`), landing on the ellipse
//! that plane carries. Near a node, where the walls are tangent, the
//! quadric reads a seam `δ` off it through a discriminant of order `δ²`;
//! here its two crossings stand `2δ/sin θ` apart, linear in the offset,
//! so what refuses closes to one band about the node.
//!
//! Where an edge has two crossings, one on each plane, standing within
//! the band of each other, it passes through a node, and this read
//! declines: the edge keeps the quadric's route. In band, it escalates.

use geom_core::{Band, Decide, Margin, Point3, Sign};

use super::{CurvedEvent, Landing, Placement, edge_face_read, landing, vertex_on_curved_face};
use crate::body::Body;
use crate::boolean::{BooleanDecision, BooleanError, Coincide, Operand, node};
use crate::entity::{Edge, FaceKey, VertexKey};
use crate::splitting::{ConicPlaneMeet, PlaneCrossingLane};
use crate::validate::decide;

/// The predicate an end's distance from a bisector plane is decided on:
/// whether the end lies on the partner wall, read through the planes.
pub(in crate::boolean) const FACTORED_PLANE_SIDE: &str = "bool_factored_plane_side";

/// **The crossing of `face`, a wall of an equal cylinder pair with
/// meeting axes, by an edge of `x` lying on the pair's other wall**,
/// read through the two bisector planes (module docs). `Ok(None)` leaves
/// the pair to the quadric's route: no such pair, a conic lying in or in
/// band of a bisector plane (on a branch of the section), a carrier no
/// planar lane takes, or two crossings that coincide, the edge passing
/// through a node.
///
/// Otherwise the answer takes the quadric route's arms. Of the crossings
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
/// [`BooleanError::NodeUnbuilt`] (`OnBoundary`) for an end at a node the
/// face does not place outside it; [`BooleanError::Escalated`]: two
/// crossings whose distance is in band
/// (`VertexOnVertex`), or a planar lane's root in band; an end's plane
/// distance in band (`VertexOnCurvedFace`); a landing the containment
/// door escalates; the typed frontier where a landing or an end has no
/// verdict.
#[allow(clippy::too_many_arguments)]
pub(super) fn factored_crossing<T: Decide + crate::props::AtRestPolicy>(
    (x, x_is): (&Body<T>, Operand),
    (edge, carrier, (t0, t1)): (&Edge, &geom::Curve3<T>, (T, T)),
    [(u, pu), (v, pv)]: [(VertexKey, Point3<T>); 2],
    (y, face): (&mut Body<T>, FaceKey),
    declared: &super::super::GluedPairs<T>,
    contacts: &mut super::ContactAcc,
    frontier: impl Fn() -> BooleanError,
    band: Band,
    tol: geom_core::Tol,
) -> Result<Option<CurvedEvent<T>>, BooleanError> {
    let walls = [edge.he_plus, edge.he_minus].map(|he| x.face_of_half_edge(he));
    let Some((wall, planes)) = walls
        .into_iter()
        .flatten()
        .find_map(|wall| node::bisectors((x, x_is, wall), (y, face), band).map(|p| (wall, p)))
    else {
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
    let ends = [
        [dist(pu, planes[0])?, dist(pu, planes[1])?],
        [dist(pv, planes[0])?, dist(pv, planes[1])?],
    ];
    // Each plane's crossings strictly inside the span.
    let mut roots: [Vec<T>; 2] = [Vec::new(), Vec::new()];
    for (i, (centre, normal)) in planes.into_iter().enumerate() {
        match crate::splitting::plane_crossing_lane(carrier, t0, t1, centre, normal, band) {
            // Affine along a line: one root, strictly inside exactly
            // where the ends straddle the plane.
            PlaneCrossingLane::Line => {
                if matches!(
                    (ends[0][i].sign, ends[1][i].sign),
                    (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive)
                ) {
                    // Read off the carrier, not interpolated between the
                    // ends: the ends' two distances share their rounding,
                    // which an enclosure would count twice.
                    let geom::Curve3::Line { origin, dir } = *carrier else {
                        return Ok(None);
                    };
                    roots[i].push((centre - origin).dot(normal) / dir.dot(normal));
                }
            }
            PlaneCrossingLane::Conic(ConicPlaneMeet::Miss) => {}
            PlaneCrossingLane::Conic(ConicPlaneMeet::Parallel { offset }) => {
                match decide("bool_conic_face_plane_offset", Margin::of(offset), band) {
                    Ok(Sign::Positive | Sign::Negative) => {}
                    Ok(Sign::Zero) | Err(_) => return Ok(None),
                }
            }
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Ok(ts))) => roots[i] = ts,
            PlaneCrossingLane::Conic(ConicPlaneMeet::Roots(Err(fault))) => {
                return Err(BooleanError::Escalated {
                    decision: BooleanDecision::of_conic_root(
                        fault,
                        read(Coincide::EdgeOnCurvedFace),
                    ),
                    diag: fault.diag(),
                });
            }
            PlaneCrossingLane::Unlaned => return Ok(None),
        }
    }
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
                Ok(Sign::Zero | Sign::Negative) => return Ok(None),
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
                    branch: Some(branch_of(i)),
                    chart: None,
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
                contacts.branches.insert((x_is, w), branch_of(i));
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

/// The branch of the plane at `index` in [`node::bisectors`]' order:
/// the `â₁ − â₂` plane's ellipse is the bisector split's positive loop
/// (`ring_path::LoopSplit::Bisector`), the `â₁ + â₂` plane's its
/// negative one.
fn branch_of(index: usize) -> crate::boolean::Branch {
    if index == 0 {
        crate::boolean::Branch::Positive
    } else {
        crate::boolean::Branch::Negative
    }
}

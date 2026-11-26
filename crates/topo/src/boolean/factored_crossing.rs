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
//! lanes' own (`geom_brep::curve_face::plane_crossing`), landing on the ellipse
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
    let ends = [
        [dist(pu, planes[0])?, dist(pu, planes[1])?],
        [dist(pv, planes[0])?, dist(pv, planes[1])?],
    ];
    // A carrier with no root lane against a plane (a spiric, a spline):
    // the factored read has no crossing to read, so no such pair here, and
    // the quadric's route reads the edge.
    if !matches!(carrier, geom::Curve3::Line(_)) && carrier.conic().is_none() {
        return Ok(None);
    }
    let span = geom_brep::curve_pair::Span::new(carrier, (t0, t1));
    // A window of a plane's answer, as the planar lane reads one
    // ([`super::plane_refused`]): a rung in band is the edge's coincidence
    // with the plane, and a root the placement cannot place or order its
    // crossing decision.
    let refused = |why| {
        super::plane_refused::<T>(
            why,
            |diag| BooleanError::Escalated {
                decision: BooleanDecision::Coincidence(
                    Coincide::EdgeOnPlane,
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
    for (i, (centre, normal)) in planes.into_iter().enumerate() {
        use geom_brep::curve_face::{Crossings, OnSpan, Pose};
        let plane = geom::Surface::Plane {
            frame: geom::Frame::new(centre, normal, normal.orthonormal_basis().0),
        };
        let inside = match geom_brep::curve_face::cross(&span, &plane, band) {
            Ok(Crossings {
                pose: Pose::Apart, ..
            }) => Vec::new(),
            // A line in the plane lies on both walls along it: its ends
            // read `Zero` on the plane, and the endpoint posture below
            // reads them. A conic in the plane is one of the section's
            // ellipses, on both walls: the factored read crosses nothing
            // along the section, and refuses it typed, naming the plane.
            Ok(Crossings {
                pose: Pose::Lies, ..
            }) => {
                if carrier.conic().is_some() {
                    let (a_face, b_face) = match x_is {
                        Operand::A => (wall, face),
                        Operand::B => (face, wall),
                    };
                    return Err(BooleanError::NodeUnbuilt {
                        a_face,
                        b_face,
                        why: crate::boolean::NodeWhy::OnBisector {
                            operand: x_is,
                            plane: i,
                        },
                    });
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

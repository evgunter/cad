//! **Two planar faces on one plane, apart as regions** — arm 1's clear
//! for a cross-solid pair on one carrier that no direction separates: a
//! block in a plate's reflex notch, the plate's top and the block's base
//! in one plane, the plate wrapping two sides of the block. A plane that
//! parted the faces would cut theirs in a line that parts the regions,
//! and none does; neither does a depth, where the block's corner lies on
//! the notch fillet's axis.
//!
//! **The reading.** Two faces on one plane are compact regions of it.
//! Where no edge of one meets an edge of the other, each boundary loop
//! lies wholly inside or wholly outside the other face: it is connected,
//! and misses that face's boundary. And where the regions meet, their
//! intersection is a nonempty compact set of the plane, so it has a
//! boundary point; that point lies on one face's boundary and in the
//! other face, off the other's boundary, so in its interior, and the
//! loop through it lies inside. So every edge pair apart, and one point
//! of every loop of each face placed `Out` of the other, prove the
//! faces disjoint. That is the verdict, and the only one.
//!
//! - **One plane**: the carrier reading decides the two planes one
//!   carrier, a margin decided Zero, with the senses either way
//!   ([`crate::boolean::carrier_pair::carrier_pair_reading`], the planar
//!   face-pair arm's Door 1). Two planes that are not one cross on a
//!   line, where loops apart and vertices outside prove nothing, and a
//!   carrier in band does not hold the regions in one plane.
//! - **Edges apart**: each edge of one face and each of the other are
//!   apart along a direction ([`crate::boolean::separating::apart_by`]):
//!   the axis between their anchors, the world axes, and each arc's
//!   chord normal and the offset from its centre to the other edge's
//!   anchor. The gap is less both edges' [`sweep_pad`], as the face
//!   pair's own is.
//! - **Loops outside**: a vertex of each loop is placed against the
//!   other face ([`face_placement`]), on its plane decided Zero, and
//!   only a definite `Out` counts.
//!
//! A pair that touches has an edge pair with no gap, and one that
//! overlaps has a loop inside the other face, so neither is cleared;
//! anything not read (a lone-vertex loop, an edge with no reach, a
//! placement the containment door refuses) leaves the pair to arm 1's
//! refusal.

use geom_core::{Band, Decide, Point3, UnitVec3, Vec3, is_finite_length};

use crate::body::Body;
use crate::boolean::boxes::{EdgeBoxRule, edge_box_rule, edge_end_point, sweep_pad};
use crate::boolean::separating::{Item, anchor, apart_by};
use crate::boolean::{CarrierRelation, CurvedPlacement, FaceContainment, face_placement};
use crate::entity::{EdgeKey, EntityId, FaceKey};
use crate::live::proven;

/// K name: a loop vertex's offset from the other face's plane, read by
/// the point placement before it places the vertex.
pub(crate) const REGION_ON_PLANE: &str = "census_backstop_region_on_plane";

/// K name: an arc's chord normal, or the offset from its centre to the
/// other edge's anchor (metres), before it is read as a unit direction.
pub(crate) const REGION_ACROSS: &str = "census_backstop_region_across";

/// One loop of a face: its edges, and one of its vertices' points.
struct Loop<T: geom_core::Real> {
    edges: Vec<EdgeKey>,
    point: Point3<T>,
}

/// `face`'s loops; `None` where one is a lone vertex, which bounds no
/// edge to read.
fn loops<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec<Loop<T>>> {
    crate::pcurves::face_loop_walks(body, face)
        .into_iter()
        .map(|(_, cycle)| {
            let &first = cycle.first()?;
            let he = proven(&body.half_edges, first, EntityId::HalfEdge);
            let point = edge_end_point(body, he.edge, first, "start");
            let edges = cycle
                .iter()
                .map(|&h| proven(&body.half_edges, h, EntityId::HalfEdge).edge)
                .collect();
            Some(Loop { edges, point })
        })
        .collect()
}

/// The directions an arc edge `e` aims at the edge `other`: its chord
/// normal in its conic's plane, and the offset from its centre to
/// `other`'s anchor. None for a line.
fn arc_directions<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    other: EdgeKey,
    band: Band,
) -> Vec<UnitVec3<T>> {
    let edge = proven(&body.edges, e, EntityId::Edge);
    let EdgeBoxRule::ConicAmplitude(arc) = edge_box_rule(body.edge_curve_linked(e, edge).certified())
    else {
        return Vec::new();
    };
    let chord = edge_end_point(body, e, edge.he_minus, "he_minus")
        - edge_end_point(body, e, edge.he_plus, "he_plus");
    let unit = |v: Vec3<T>| {
        is_finite_length(v.norm())
            .then(|| UnitVec3::new(v, REGION_ACROSS, band).ok())
            .flatten()
    };
    let mut out = Vec::new();
    out.extend(unit((arc.axis / arc.axis.norm()).cross(chord)));
    if let Some(q) = anchor(body, Item::Edge(other)) {
        out.extend(unit(q - arc.center));
    }
    out
}

/// Whether the edges `x`, `y` are apart (module docs).
fn edges_apart<T: Decide>(body: &Body<T>, x: EdgeKey, y: EdgeKey, band: Band) -> bool {
    let mut axes = vec![UnitVec3::exact_x(), UnitVec3::exact_y(), UnitVec3::exact_z()];
    axes.extend(arc_directions(body, x, y, band));
    axes.extend(arc_directions(body, y, x, band));
    apart_by(
        (body, Item::Edge(x)),
        (body, Item::Edge(y)),
        &axes,
        sweep_pad(band),
        band,
        &|body, f, frame| super::face_reach_in(body, f, band, frame),
    )
}

/// Whether `p` lies definitely outside `face`, on its plane.
fn outside<T: Decide>(body: &Body<T>, face: FaceKey, p: Point3<T>, band: Band) -> bool {
    matches!(
        face_placement(body, face, p, band, REGION_ON_PLANE),
        Ok(CurvedPlacement::Trim(Some(FaceContainment::Out)))
    )
}

/// **Whether the planar faces `a`, `b` lie on one plane, apart as
/// regions** (module docs).
pub(super) fn apart_in_plane<T: Decide>(body: &Body<T>, a: FaceKey, b: FaceKey, band: Band) -> bool {
    let one_plane = matches!(
        crate::boolean::carrier_pair::carrier_pair_reading(body, a, body, b, false, band),
        Ok(Ok((
            CarrierRelation::SameOriented | CarrierRelation::SameOpposite,
            crate::contact::ContactVerdict::Definite,
            _
        )))
    );
    if !one_plane {
        return false;
    }
    let (Some(la), Some(lb)) = (loops(body, a), loops(body, b)) else {
        return false;
    };
    la.iter().all(|x| outside(body, b, x.point, band))
        && lb.iter().all(|y| outside(body, a, y.point, band))
        && la.iter().flat_map(|x| &x.edges).all(|&ea| {
            lb.iter()
                .flat_map(|y| &y.edges)
                .all(|&eb| edges_apart(body, ea, eb, band))
        })
}

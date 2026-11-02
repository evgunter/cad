//! **A planar face against the plane of another, apart where it meets
//! that plane** — arm 1's clear for a cross-solid pair no direction
//! separates: a block in a plate's reflex notch, the plate wrapping two
//! sides of the block. The block's base lies in the plate top's plane,
//! and each of its walls stands on that plane along an edge. A plane that
//! parted such a pair would cut the top's plane in a line that parts the
//! top from the base, or from the wall's foot, and none does; neither
//! does a depth, where the block's corner lies on the notch fillet's
//! axis.
//!
//! **The reading.** Let `A` be a planar face on the plane `P`, and `B` a
//! planar face. `A ∩ B` lies in `B ∩ P`, which is read in `P`. Every
//! decision is the band's: a Zero is a margin within `z` (the band's
//! zero threshold) of nought, not nought, and each argument below
//! carries that `z`.
//!
//! - **`B` on `P`**: the carrier reading decides the two planes one
//!   carrier, a margin decided Zero, either sense (the planar face-pair
//!   arm's Door 1, [`crate::boolean::carrier_pair_decided`]),
//!   which the census reads as one plane wherever it decides it. On that
//!   plane the two faces are compact regions. Where no edge of one meets
//!   an edge of the other, each boundary loop lies wholly inside or
//!   wholly outside the other face: it is connected, and misses that
//!   face's boundary. Where the regions meet, their intersection is a
//!   nonempty compact set of the plane, so it has a boundary point; that
//!   point lies on one face's boundary and in the other face, off the
//!   other's boundary, so in its interior, and the loop through it lies
//!   inside. So every edge pair apart, and one point of every loop of each
//!   face placed `Out` of the other, put the faces apart. This is not the
//!   planar face-pair arm's interiors reading
//!   (`census::planar_interiors_meet`): that one asks whether two
//!   polygons' interiors meet, on aligned faces, and a shared boundary
//!   point answers it no; this one asks whether the closures meet, on
//!   either sense, with arcs, and only a gap answers it no.
//! - **`B` standing on `P`**: no vertex of `B` is decided on the far side
//!   of `P`, some vertex is decided off it, every arc of `B` lies
//!   definitely off it (its reach along `P`'s normal), and `B` meets `P`
//!   at a slope `sin θ = |n_P × n_B|` decided past ½. Then `B` lies in
//!   the hull of its boundary, above `−z` in height over `P`. From a
//!   point of `B ∩ P`, the steepest way down `B` falls at `sin θ` and
//!   leaves `B` at most `z / sin θ ≤ 2z` away, at a boundary point no
//!   higher, so on a line edge with an end decided on `P` (a line whose
//!   ends both lie above `P` lies above it throughout, and an arc does).
//!   So `B ∩ P` lies within `2z` of those edges. Each is apart from every
//!   edge of `A` by more than both pads, which exceed `2z`, so its
//!   `2z`-neighbourhood in `P` is connected and misses `A`'s boundary;
//!   its end on `P`, placed `Out` of `A`, lies within `z` of `P` and in
//!   that neighbourhood. So the neighbourhood lies outside `A`, and so
//!   does `B ∩ P`. A face that meets `P` shallower than that (nearly
//!   flat on it, its foot within the band) has a section the band does
//!   not place, and refuses.
//!
//! Either role is tried for each face. An edge pair is apart along a
//! direction ([`crate::boolean::separating::apart`]): the axis between
//! their anchors, the world axes, and the directions the pair's faces
//! propose (`depth_gap::toward`, `depth_gap::across`); the gap is less
//! both edges' [`sweep_pad`], as a face pair's is. A placement counts only
//! where it decides the point on the plane and `Out` ([`face_placement`]).
//! That is the verdict, and the only one.
//!
//! A pair that touches has an edge pair with no gap, or a vertex on the
//! other face, and one that overlaps has a loop inside the other face or
//! crosses its plane, so neither is cleared; anything not read (a
//! lone-vertex loop, an edge with no reach, a side, a slope or a
//! placement in band, a door's refusal) leaves the pair to arm 1's
//! refusal.

use geom_core::{Band, Decide, Margin, Point3, Sign, UnitVec3, Vec3};

use crate::body::Body;
use crate::boolean::boxes::{BoxFrame, EdgeBoxRule, edge_box_rule, edge_end_point, sweep_pad};
use crate::boolean::separating::{Item, apart, planar_axis, world_axes};
use crate::boolean::{CarrierRelation, CurvedPlacement, FaceContainment, face_placement};
use crate::entity::{EdgeKey, EntityId, FaceKey};
use crate::live::proven;
use crate::validate::decide;

/// K name: a vertex's offset from the other face's plane, read by the
/// point placement before it places the vertex.
pub(crate) const REGION_ON_PLANE: &str = "census_backstop_region_on_plane";

/// K name: a vertex's offset from the other face's plane (metres), whose
/// sign says which side of the plane it lies on.
pub(crate) const REGION_SIDE: &str = "census_backstop_region_side";

/// K name: the gap between an arc's reach along the other face's normal
/// and that face's plane (metres).
pub(crate) const REGION_ARC_SIDE: &str = "census_backstop_region_arc_side";

/// K name: the sine of the angle a face meets the other's plane at, less
/// ½, levered by the face's reach diagonal (metres).
pub(crate) const REGION_SLOPE: &str = "census_backstop_region_slope";

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

/// Whether `p` lies definitely outside `face`, on its plane.
fn outside<T: Decide>(body: &Body<T>, face: FaceKey, p: Point3<T>, band: Band) -> bool {
    matches!(
        face_placement(body, face, p, band, REGION_ON_PLANE),
        Ok(CurvedPlacement::Trim(Some(FaceContainment::Out)))
    )
}

/// **Whether the planar faces `a`, `b` are apart where each meets the
/// other's plane** (module docs), each edge pair read along the world
/// axes and `proposed`.
pub(super) fn apart_in_plane<T: Decide>(
    body: &Body<T>,
    a: FaceKey,
    b: FaceKey,
    proposed: &[UnitVec3<T>],
    band: Band,
) -> bool {
    let (Some(la), Some(lb)) = (loops(body, a), loops(body, b)) else {
        return false;
    };
    let mut axes = world_axes().to_vec();
    axes.extend_from_slice(proposed);
    let edges_apart = |x: EdgeKey, y: EdgeKey| {
        apart(
            (body, Item::Edge(x)),
            (body, Item::Edge(y)),
            &axes,
            sweep_pad(band),
            band,
        )
    };
    let one_plane = matches!(
        crate::boolean::carrier_pair_decided(body, a, body, b, band),
        Ok(Ok((
            CarrierRelation::SameOriented | CarrierRelation::SameOpposite,
            Some(_)
        )))
    );
    if one_plane {
        return la.iter().all(|x| outside(body, b, x.point, band))
            && lb.iter().all(|y| outside(body, a, y.point, band))
            && la.iter().flat_map(|x| &x.edges).all(|&ea| {
                lb.iter()
                    .flat_map(|y| &y.edges)
                    .all(|&eb| edges_apart(ea, eb))
            });
    }
    standing(body, (a, &la), (b, &lb), &edges_apart, band)
        || standing(body, (b, &lb), (a, &la), &edges_apart, band)
}

/// A face's levered outward normal ([`planar_axis`]) and its reach
/// diagonal's length.
fn normal<T: Decide>(body: &Body<T>, f: FaceKey, band: Band) -> Option<(UnitVec3<T>, T)> {
    let (lo, hi) = super::face_reach(body, f, band)?;
    Some((
        planar_axis(body, f, || Some((lo, hi)), band)?,
        (hi - lo).norm(),
    ))
}

/// The second reading (module docs): `b`, bounded by `lb`, standing on
/// `a`'s plane, apart from `a` where it meets that plane.
fn standing<T: Decide>(
    body: &Body<T>,
    (a, la): (FaceKey, &[Loop<T>]),
    (b, lb): (FaceKey, &[Loop<T>]),
    edges_apart: &dyn Fn(EdgeKey, EdgeKey) -> bool,
    band: Band,
) -> bool {
    let face = proven(&body.faces, a, EntityId::Face);
    let geom::Surface::Plane { origin, .. } = *body.face_surface_linked(a, face) else {
        return false;
    };
    let (Some((n, _)), Some((nb, lever))) = (normal(body, a, band), normal(body, b, band)) else {
        return false;
    };
    let half = T::from_f64(0.5);
    let steep = (n.get().cross(nb.get()).norm() - half) * lever;
    if !matches!(
        decide(REGION_SLOPE, Margin::of(steep), band),
        Ok(Sign::Positive)
    ) {
        return false;
    }
    let level = Vec3::new(origin.x, origin.y, origin.z).dot(n.get());
    let side = |p: Point3<T>| decide(REGION_SIDE, Margin::of((p - origin).dot(n.get())), band).ok();
    let mut sense = None;
    let mut arcs = Vec::new();
    let mut touching = Vec::new();
    for e in lb.iter().flat_map(|y| &y.edges).copied() {
        let edge = proven(&body.edges, e, EntityId::Edge);
        let ends = [edge.he_plus, edge.he_minus].map(|he| edge_end_point(body, e, he, "end"));
        let [Some(s0), Some(s1)] = ends.map(side) else {
            return false;
        };
        for s in [s0, s1] {
            if s != Sign::Zero && *sense.get_or_insert(s) != s {
                return false;
            }
        }
        if !matches!(
            edge_box_rule(body.edge_curve_linked(e, edge).certified()),
            EdgeBoxRule::Chord
        ) {
            arcs.push(e);
        } else if s0 == Sign::Zero || s1 == Sign::Zero {
            touching.push((e, ends, [s0, s1]));
        }
    }
    let Some(sense) = sense else {
        return false;
    };
    let off = |e: EdgeKey| {
        let Some((lo, hi)) = super::edge_reach_in(body, e, &BoxFrame::aimed(n)) else {
            return false;
        };
        let gap = match sense {
            Sign::Positive => lo.x - level,
            _ => level - hi.x,
        };
        matches!(
            decide(REGION_ARC_SIDE, Margin::of(gap), band),
            Ok(Sign::Positive)
        )
    };
    arcs.into_iter().all(off)
        && touching.iter().all(|&(e, ends, signs)| {
            ends.iter()
                .zip(signs)
                .all(|(&p, s)| s != Sign::Zero || outside(body, a, p, band))
                && la
                    .iter()
                    .flat_map(|x| &x.edges)
                    .all(|&ea| edges_apart(e, ea))
        })
}

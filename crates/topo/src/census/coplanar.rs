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
//! planar face. `A ∩ B` lies in `B ∩ P`, and that is read in `P`:
//!
//! - **`B` on `P`**: the carrier reading decides the two planes one
//!   carrier, a margin decided Zero, either sense (the planar face-pair
//!   arm's Door 1, [`crate::boolean::carrier_pair::carrier_pair_reading`]).
//!   Two faces on one plane are compact regions of it. Where no edge of
//!   one meets an edge of the other, each boundary loop lies wholly
//!   inside or wholly outside the other face: it is connected, and misses
//!   that face's boundary. Where the regions meet, their intersection is
//!   a nonempty compact set of the plane, so it has a boundary point; that
//!   point lies on one face's boundary and in the other face, off the
//!   other's boundary, so in its interior, and the loop through it lies
//!   inside. So every edge pair apart, and one point of every loop of each
//!   face placed `Out` of the other, prove the faces disjoint.
//! - **`B` on one side of `P`**: every vertex of `B` is decided on `P` or
//!   on one side of it, not all on it, and every arc of `B` lies
//!   definitely on that side (its reach along `P`'s normal). A planar
//!   face lies in the hull of its boundary, so `B` lies in the closed
//!   half-space, in a half-plane of its own plane bounded by the line it
//!   shares with `P`; so `B ∩ P` lies in `B`'s boundary, in its line edges
//!   with an end on `P`. One with one end on `P` meets it there alone; one
//!   with both lies in it. So each such edge apart from every edge of
//!   `A`, and each of its ends on `P` placed `Out` of `A`, prove `A ∩ B`
//!   empty: each such edge is connected, misses `A`'s boundary, and has an
//!   end outside `A`.
//!
//! Either role is tried for each face. An edge pair is apart along a
//! direction ([`crate::boolean::separating::apart_by`]): the axis between
//! their anchors, the world axes, and each arc's chord normal and the
//! offset from its centre to the other edge's anchor; the gap is less
//! both edges' [`sweep_pad`], as a face pair's is. A placement counts only
//! where it decides the point on the plane and `Out` ([`face_placement`]).
//! That is the verdict, and the only one.
//!
//! A pair that touches has an edge pair with no gap, or a vertex on the
//! other face, and one that overlaps has a loop inside the other face or
//! crosses its plane, so neither is cleared; anything not read (a
//! lone-vertex loop, an edge with no reach, a side or placement in band,
//! a door's refusal) leaves the pair to arm 1's refusal.

use geom_core::{Band, Decide, Margin, Point3, Sign, UnitVec3, Vec3, is_finite_length};

use crate::body::Body;
use crate::boolean::boxes::{BoxFrame, EdgeBoxRule, edge_box_rule, edge_end_point, sweep_pad};
use crate::boolean::separating::{Item, anchor, apart_by, planar_axis};
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
    let EdgeBoxRule::ConicAmplitude(arc) =
        edge_box_rule(body.edge_curve_linked(e, edge).certified())
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
    let mut axes = vec![
        UnitVec3::exact_x(),
        UnitVec3::exact_y(),
        UnitVec3::exact_z(),
    ];
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

/// **Whether the planar faces `a`, `b` are apart where each meets the
/// other's plane** (module docs).
pub(super) fn apart_in_plane<T: Decide>(
    body: &Body<T>,
    a: FaceKey,
    b: FaceKey,
    band: Band,
) -> bool {
    if mutant("off") {
        return false;
    }
    let (Some(la), Some(lb)) = (loops(body, a), loops(body, b)) else {
        return false;
    };
    let one_plane = matches!(
        crate::boolean::carrier_pair::carrier_pair_reading(body, a, body, b, false, band),
        Ok(Ok((
            CarrierRelation::SameOriented | CarrierRelation::SameOpposite,
            crate::contact::ContactVerdict::Definite,
            _
        )))
    );
    let probe = std::env::var("PROBE5918").is_ok();
    let one_plane = one_plane || mutant("one_plane");
    if one_plane {
        let r = regions_apart(body, (a, &la), (b, &lb), band);
        if probe && r { eprintln!("PROBE5918 reading1 apart {a:?} {b:?}"); }
        return r;
    }
    let r1 = beside(body, (a, &la), &lb, band);
    let r2 = beside(body, (b, &lb), &la, band);
    if probe && (r1 || r2) { eprintln!("PROBE5918 reading2 apart {a:?} {b:?} a_as_P={r1} b_as_P={r2}"); }
    r1 || r2
}

/// The first reading (module docs): `a` and `b` on one plane, apart as
/// regions.
fn regions_apart<T: Decide>(
    body: &Body<T>,
    (a, la): (FaceKey, &[Loop<T>]),
    (b, lb): (FaceKey, &[Loop<T>]),
    band: Band,
) -> bool {
    la.iter().all(|x| outside(body, b, x.point, band))
        && lb.iter().all(|y| outside(body, a, y.point, band))
        && (mutant("r1_noedges") || la.iter().flat_map(|x| &x.edges).all(|&ea| {
            lb.iter()
                .flat_map(|y| &y.edges)
                .all(|&eb| edges_apart(body, ea, eb, band))
        }))
}

fn mutant(name: &str) -> bool {
    std::env::var("MUT5918").is_ok_and(|v| v == name)
}

/// The second reading (module docs): the face bounded by `lb` on one
/// side of `a`'s plane, apart from `a` where it meets that plane.
fn beside<T: Decide>(
    body: &Body<T>,
    (a, la): (FaceKey, &[Loop<T>]),
    lb: &[Loop<T>],
    band: Band,
) -> bool {
    let face = proven(&body.faces, a, EntityId::Face);
    let geom::Surface::Plane { origin, .. } = *body.face_surface_linked(a, face) else {
        return false;
    };
    let Some(n) = planar_axis(body, a, || super::face_reach(body, a, band), band) else {
        return false;
    };
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
    (mutant("no_arc") || arcs.into_iter().all(off))
        && touching.iter().all(|&(e, ends, signs)| {
            ends.iter()
                .zip(signs)
                .all(|(&p, s)| s != Sign::Zero || mutant("r2_noplace") || outside(body, a, p, band))
                && la
                    .iter()
                    .flat_map(|x| &x.edges)
                    .all(|&ea| edges_apart(body, e, ea, band))
        })
}

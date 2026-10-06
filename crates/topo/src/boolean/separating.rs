//! **The narrow phase behind a world-box overlap**: whether a face or
//! an edge of one operand and a face of the other are apart along a
//! direction that turns with the operands.
//!
//! Two padded world boxes that overlap separate their loci along `x̂`,
//! `ŷ` and `ẑ` only, and the world box of a turned face widens by how
//! it is turned ([`super::boxes::BoxFrame`]). So an overlap read alone
//! makes "may these two meet" a fact about the pose. Two disjoint
//! convex hulls have a separating direction, and [`apart`] proposes
//! directions fixed to the pair rather than to the world:
//!
//! - the axis between the two items' anchors (the mean of each item's
//!   boundary vertices, which move with its body);
//! - every planar face's outward normal on either operand
//!   ([`operand_axes`]) — a planar partner's normal, and the turned
//!   operand's own axes.
//!
//! and reads both items' reaches along each through the census lane's
//! rule ([`crate::census::face_reach_in`],
//! [`crate::census::edge_reach_in`]) in the frame that direction aims
//! ([`super::boxes::BoxFrame::aimed`]). The first coordinate of that
//! reach is the item's support along the direction, so the gap between
//! two of them does not depend on how the pair is turned.
//!
//! **Soundness.** Every reach is a superset of its item's locus (the
//! `boxes` module contract, at the census lane's scalar), so a gap
//! decided positive after both items' pads proves the loci disjoint:
//! the same certificate a non-overlap of the two padded world boxes
//! gives, read along another direction. A reach the rule cannot bound
//! (`None`), an anchor offset the band cannot orient, or a gap in the
//! band answers "not apart", which keeps the caller's conservative
//! verdict.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, UnitVec3, Vec3};

use super::boxes::BoxFrame;
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey};
use crate::live::BoundaryMember;
use crate::validate::decide;

/// K name: a candidate separating direction's length before it is read
/// as a unit axis — the offset between two anchors (metres), or a
/// planar carrier's normal.
const PAIR_AXIS: &str = "bool_pair_axis";

/// K name: two reaches' gap along a candidate direction, less both
/// items' pads (metres).
const PAIR_GAP: &str = "bool_pair_gap";

/// One item a reach is read for: a face, an edge (the sweep's piercing
/// side), or a full circle no body holds (the extent scan's section of
/// a sphere by a face's carrier plane).
#[derive(Clone, Copy, Debug)]
pub(crate) enum Item<T: Real> {
    /// A face of its body.
    Face(FaceKey),
    /// An edge of its body.
    Edge(EdgeKey),
    /// The circle about `center` of radius `radius` in the plane of the
    /// orthonormal `u_ref`, `v_ref`; its body is not read.
    Circle {
        /// The centre.
        center: Point3<T>,
        /// One in-plane unit direction.
        u_ref: Vec3<T>,
        /// The other, perpendicular to it.
        v_ref: Vec3<T>,
        /// The radius.
        radius: T,
    },
}

/// The outward normal of every planar face of `a` and of `b`, as unit
/// directions: the candidate axes that turn with the
/// operands. A normal the band cannot read as a direction is left out,
/// which only drops a candidate.
pub(crate) fn operand_axes<T: Decide>(a: &Body<T>, b: &Body<T>, band: Band) -> Vec<UnitVec3<T>> {
    let mut axes: Vec<UnitVec3<T>> = Vec::new();
    for body in [a, b] {
        for (key, _) in body.faces() {
            let Some(n) = crate::face_normal::face_outward_normal(body, key) else {
                continue;
            };
            if let Ok(unit) = UnitVec3::new(n.vec(), PAIR_AXIS, band) {
                axes.push(unit);
            }
        }
    }
    axes
}

/// Whether `x`'s item and `y`'s item are certainly apart: their reaches,
/// each widened by `pad`, separate along the axis between their anchors
/// or along one of `axes` ([`operand_axes`]).
///
/// # Panics
///
/// As [`crate::census::face_reach_in`]: each item is one the caller
/// read out of its body, and a torn hop past it is a kernel bug.
pub(crate) fn apart<T: Decide>(
    (x, xi): (&Body<T>, Item<T>),
    (y, yi): (&Body<T>, Item<T>),
    axes: &[UnitVec3<T>],
    pad: f64,
    band: Band,
) -> bool {
    let between = match (anchor(x, xi), anchor(y, yi)) {
        (Some(p), Some(q)) => UnitVec3::new(q - p, PAIR_AXIS, band).ok(),
        _ => None,
    };
    between
        .iter()
        .chain(axes)
        .any(|&dir| apart_along(x, xi, y, yi, dir, pad, band))
}

/// [`apart`] along one direction.
fn apart_along<T: Decide>(
    x: &Body<T>,
    xi: Item<T>,
    y: &Body<T>,
    yi: Item<T>,
    dir: UnitVec3<T>,
    pad: f64,
    band: Band,
) -> bool {
    let frame = BoxFrame::aimed(dir);
    let (Some((xl, xh)), Some((yl, yh))) = (reach(x, xi, band, &frame), reach(y, yi, band, &frame))
    else {
        return false;
    };
    let gap = (yl.x - xh.x).max(xl.x - yh.x) - T::from_f64(2.0 * pad);
    matches!(decide(PAIR_GAP, Margin::of(gap), band), Ok(Sign::Positive))
}

/// An item's reach in `frame`.
fn reach<T: Decide>(
    body: &Body<T>,
    item: Item<T>,
    band: Band,
    frame: &BoxFrame<T>,
) -> Option<(Point3<T>, Point3<T>)> {
    use super::boxes::{SpanBox, conic_extent};
    match item {
        Item::Face(f) => crate::census::face_reach_in(body, f, band, frame),
        Item::Edge(e) => crate::census::edge_reach_in(body, e, frame),
        Item::Circle {
            center,
            u_ref,
            v_ref,
            radius,
        } => {
            let b = conic_extent(
                &SpanBox::point(frame.point(center)),
                &SpanBox::vector(frame.vector(u_ref)),
                &SpanBox::vector(frame.vector(v_ref)),
                radius,
                radius,
            );
            Some((
                Point3::new(b.x.lo, b.y.lo, b.z.lo),
                Point3::new(b.x.hi, b.y.hi, b.z.hi),
            ))
        }
    }
}

/// The mean of an item's boundary vertices, or a circle's centre — a
/// point that moves with its body. `None` for a face whose boundary has
/// no vertex.
fn anchor<T: Decide>(body: &Body<T>, item: Item<T>) -> Option<Point3<T>> {
    use super::boxes::edge_end_point;
    let mut sum = Vec3::new(T::zero(), T::zero(), T::zero());
    let mut n = 0u32;
    let mut add = |p: Point3<T>| {
        sum = sum + Vec3::new(p.x, p.y, p.z);
        n += 1;
    };
    match item {
        Item::Edge(ek) => {
            let e = crate::live::proven(&body.edges, ek, crate::entity::EntityId::Edge);
            add(edge_end_point(body, ek, e.he_plus, "he_plus"));
            add(edge_end_point(body, ek, e.he_minus, "he_minus"));
        }
        Item::Face(f) => {
            let face = crate::live::proven(&body.faces, f, crate::entity::EntityId::Face);
            for member in body.face_boundary_linked(f, face) {
                match member {
                    BoundaryMember::Isolated(p) => add(p),
                    BoundaryMember::Edge { ek, edge } => {
                        add(edge_end_point(body, ek, edge.he_plus, "he_plus"));
                    }
                }
            }
        }
        Item::Circle { center, .. } => return Some(center),
    }
    (n > 0).then(|| {
        let k = T::one() / T::from_f64(f64::from(n));
        Point3::new(sum.x * k, sum.y * k, sum.z * k)
    })
}

//! **The narrow phase behind a world-box overlap**: whether a face or
//! an edge of one operand and a face of the other are apart along a
//! direction that turns with the operands.
//!
//! Two padded world boxes that overlap separate their loci along `x̂`,
//! `ŷ` and `ẑ` only, and the world box of a turned face widens by how
//! it is turned. So an overlap read alone
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
//! and reads both items' supports either way along each, off their
//! covers ([`crate::cover::face_cover`], [`crate::cover::edge_cover`]),
//! so the gap between two of them does not depend on how the pair is
//! turned.
//!
//! For two faces, [`faces_apart`] reads on past [`apart`]: along the
//! directions the pair's own geometry aims (the census's,
//! [`crate::census::pair_directions`]), each face's cover read along the
//! direction ([`crate::cover::face_cover_toward`]), as the census's
//! arm 1 reads a pair at rest.
//!
//! **Soundness.** Every cover holds its item's locus, so a gap decided
//! positive after both items' pads proves the loci disjoint: the same
//! certificate a non-overlap of the two padded world boxes gives, read
//! along another direction. An item with no cover, an anchor offset the
//! band cannot orient, or a gap in the band answers "not apart", which
//! keeps the caller's conservative verdict.

use geom::cover::{Cover, ParamWindow};
use geom_core::{Band, Decide, Margin, Point3, Real, Sign, UnitVec3, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey};
use crate::live::BoundaryMember;
use crate::validate::decide;

/// K name: the offset between two items' anchors (metres), before it
/// is read as a unit axis.
pub(crate) const PAIR_AXIS: &str = "bool_pair_axis";

/// K name: a planar face's normal, a pure number levered by the face's
/// extent from below ([`planar_axis`], [`UnitVec3::levered`]), before it
/// is read as a unit axis.
pub(crate) const PAIR_NORMAL: &str = "bool_pair_normal";

/// K name: two reaches' gap along a candidate direction, less both
/// items' pads (metres).
pub(crate) const PAIR_GAP: &str = "bool_pair_gap";

/// One item a reach is read for: a face, an edge (the sweep's piercing
/// side), or a full circle or ball no body holds (the extent scan's
/// section of a sphere by a face's carrier plane, and the ball whose
/// escape it asks after).
#[derive(Clone, Copy, Debug)]
pub(crate) enum Item<T: Real> {
    /// A face of its body.
    Face(FaceKey),
    /// An edge of its body.
    Edge(EdgeKey),
    /// A full circle; its body is not read.
    Circle(Circle<T>),
    /// The whole solid ball about `center` of radius `radius`, whatever
    /// part of its sphere a body's faces keep; its body is not read.
    Ball {
        /// The centre.
        center: Point3<T>,
        /// The radius.
        radius: T,
    },
}

/// The full circle about `center` of radius `radius` in the plane of
/// the orthonormal `u_ref`, `v_ref`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Circle<T: Real> {
    /// The centre.
    pub(crate) center: Point3<T>,
    /// One in-plane unit direction.
    pub(crate) u_ref: Vec3<T>,
    /// The other, perpendicular to it.
    pub(crate) v_ref: Vec3<T>,
    /// The radius.
    pub(crate) radius: T,
}

impl<T: Real> Circle<T> {
    /// The circle's cover: its arc over the whole turn. `None` for a
    /// datum that is not a finite number.
    #[expect(
        clippy::disallowed_methods,
        reason = "a conic restated on a frame the caller holds, or a point read about a centre an arc-held ellipse derives: owed a reading through the conic (work/oneround/one-stored-form-waits-on-the-readers-of-a-conics-centre.md)"
    )]
    pub(crate) fn cover(&self) -> Option<Cover<T>> {
        Cover::arc(
            &geom::Frame::new(self.center, self.u_ref.cross(self.v_ref), self.u_ref),
            (self.radius, self.radius),
            // The circle's one turn (a circle's period).
            ParamWindow::new(T::zero(), geom::conic_period::<T>().length()),
        )
    }
}

/// The outward normal of every planar face of `a` and of `b`
/// ([`planar_axis`]): the candidate axes that turn with the operands.
/// One the band cannot read, or a face with no certified boundary edge, is left out,
/// which only drops a candidate. So is one whose `key` is a kept axis's, or its
/// negation's: a gap along `−n` is the gap along `n` (a box's six faces
/// give three axes). That merge is exact-identity de-duplication of one
/// computed value, with no tolerance, and never a predicate: it drops
/// only an axis whose gaps were already read, so no verdict depends on
/// it, though which axes are read does (two normals a hair apart are
/// two axes).
fn operand_axes<T: Decide>(
    a: &Body<T>,
    b: &Body<T>,
    band: Band,
    key: AxisKey<T>,
) -> Vec<UnitVec3<T>> {
    let faces = a.faces().map(|(f, _)| (a, f));
    face_axes(faces.chain(b.faces().map(|(f, _)| (b, f))), band, Some(key))
}

/// [`operand_axes`] over any faces: each planar face's outward normal
/// ([`planar_axis`]), one axis per `key` up to sign. With no `key` (a
/// scalar that carries no bracket) every normal is kept, so a direction
/// and its negation may both be read: a second read of one span, which
/// no verdict depends on.
pub(crate) fn face_axes<'b, T: Decide + 'b>(
    faces: impl IntoIterator<Item = (&'b Body<T>, FaceKey)>,
    band: Band,
    key: Option<AxisKey<T>>,
) -> Vec<UnitVec3<T>> {
    let mut axes: Vec<UnitVec3<T>> = Vec::new();
    let mut kept: Vec<[(f64, f64); 3]> = Vec::new();
    for (body, face) in faces {
        let Some(unit) = planar_axis(body, face, band) else {
            continue;
        };
        if let Some(key) = key {
            let (plus, minus) = (key(unit.get()), key(-unit.get()));
            if kept.iter().any(|k| *k == plus || *k == minus) {
                continue;
            }
            kept.push(plus);
        }
        axes.push(unit);
    }
    axes
}

/// The world's axes, the directions a world box reads.
pub(crate) fn world_axes<T: Decide>() -> [UnitVec3<T>; 3] {
    [
        UnitVec3::exact_x(),
        UnitVec3::exact_y(),
        UnitVec3::exact_z(),
    ]
}

/// A planar face's outward normal as a unit direction, levered by how
/// far apart two of the face's points stand at least
/// ([`crate::cover::face_extent_below`]), the length it is consumed
/// over: the normal is a direction only where that reads definitely
/// positive, so the length is read from below. `None` for a face that is
/// not planar, one with no certified boundary edge, or a normal the band
/// cannot read.
pub(crate) fn planar_axis<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Option<UnitVec3<T>> {
    let n = crate::face_normal::face_outward_normal(body, face)?;
    let arm = crate::cover::face_extent_below(body, face)?;
    UnitVec3::levered(n.vec(), PAIR_NORMAL, band, arm).ok()
}

/// What [`operand_axes`] compares two axes by: two equal keys are one
/// axis. The bracket lane's (`boxes::axis_key`), handed in by a caller
/// that reads brackets, so this module decides and does not bracket.
pub(crate) type AxisKey<T> = fn(Vec3<T>) -> [(f64, f64); 3];

/// **One operand pair's candidate axes** ([`operand_axes`]), read on
/// first use and then shared by every item pair the caller asks about,
/// so a stage that reads many pairs walks the operands' faces once. An
/// all-planar operation whose world boxes never overlap reads none.
///
/// Every candidate direction is sound to read (the certificate is the
/// gap along it), so a stage whose bodies are split between two reads
/// keeps the first read's axes: the split faces lie on the carriers
/// that gave them.
pub(crate) struct OperandAxes<T: Real> {
    axes: std::cell::OnceCell<Vec<UnitVec3<T>>>,
    key: AxisKey<T>,
}

impl<T: Decide> OperandAxes<T> {
    /// Not yet read; `key` compares two axes.
    pub(crate) fn new(key: AxisKey<T>) -> Self {
        Self {
            axes: std::cell::OnceCell::new(),
            key,
        }
    }

    /// The axes of the pair `a`, `b`: read now on the first call, and
    /// the first call's thereafter.
    pub(crate) fn of(&self, a: &Body<T>, b: &Body<T>, band: Band) -> &[UnitVec3<T>] {
        self.axes.get_or_init(|| operand_axes(a, b, band, self.key))
    }
}

/// Whether `x`'s item and `y`'s item are certainly apart: their reaches,
/// each widened by `pad`, separate along the axis between their anchors
/// or along one of `axes` ([`operand_axes`]).
///
/// # Panics
///
/// As [`crate::cover::face_cover`]: each item is one the caller read
/// out of its body, and a torn hop past it is a kernel bug.
pub(crate) fn apart<T: Decide>(
    x: (&Body<T>, Item<T>),
    y: (&Body<T>, Item<T>),
    axes: &[UnitVec3<T>],
    pad: f64,
    band: Band,
) -> bool {
    apart_by(x, y, axes, pad, band, &|body, f, _| {
        crate::cover::face_cover(body, f, band).ok()
    })
}

/// **Whether two faces, each of its own body, are certainly apart**:
/// [`apart`] along `axes`, and past it every direction the pair's own
/// geometry aims ([`crate::census::pair_directions`]: an axis or
/// centre toward the other face's nearest boundary segment point, an
/// arc's chord normal, the line between two carriers' axes or centres)
/// with `axes` again, each face's cover read along the direction
/// ([`crate::cover::face_cover_toward`], a sphere face's pole-side
/// cut). That second read is arm 1's of the census
/// (`census::backstop_apart`), so a face pair the census parts at rest
/// these doors part too. It costs a boundary walk per face, and a point
/// placement per pole of a sphere face, so it runs only on a pair the
/// first read leaves.
///
/// The pole-side cut places a point in a face through its region
/// ([`crate::contain::curved_face_placement`]), and narrows only on a
/// definite `Out`. Its callers read operands at rest, or a sphere
/// re-cut's operands re-entered whole. The boolean's curved sweep arm
/// places its pierce and end points through the same door
/// (`boolean::reduce::curved_face_arm`), in the faces of the operand it
/// sweeps against, and the reverse sweep in the other's, so every face
/// these callers place into is one the operation already places into by
/// that door.
///
/// # Panics
///
/// As [`apart`].
pub(crate) fn faces_apart<T: Decide>(
    (x, fx): (&Body<T>, FaceKey),
    (y, fy): (&Body<T>, FaceKey),
    axes: &[UnitVec3<T>],
    pad: f64,
    band: Band,
) -> bool {
    let (xi, yi) = ((x, Item::Face(fx)), (y, Item::Face(fy)));
    apart(xi, yi, axes, pad, band) || {
        let mut dirs = crate::census::pair_directions((x, fx), (y, fy), band);
        dirs.extend_from_slice(axes);
        apart_by(xi, yi, &dirs, pad, band, &|body, f, aim| {
            crate::cover::face_cover_toward(body, f, band, aim).ok()
        })
    }
}

/// A face's cover read along an aim: [`crate::cover::face_cover`], or
/// [`crate::cover::face_cover_toward`], which reads one more constraint
/// off the aim. `None` for a face with no cover.
pub(crate) type FaceCoverAlong<'r, T> =
    dyn Fn(&Body<T>, FaceKey, UnitVec3<T>) -> Option<Cover<T>> + 'r;

/// [`apart`], with each face's cover along a direction read by
/// `face_cover`.
///
/// # Panics
///
/// As [`apart`], and as `face_cover`.
pub(crate) fn apart_by<T: Decide>(
    (x, xi): (&Body<T>, Item<T>),
    (y, yi): (&Body<T>, Item<T>),
    axes: &[UnitVec3<T>],
    pad: f64,
    band: Band,
    face_cover: &FaceCoverAlong<'_, T>,
) -> bool {
    let between = match (anchor(x, xi), anchor(y, yi)) {
        (Some(p), Some(q)) => UnitVec3::new(q - p, PAIR_AXIS, band).ok(),
        _ => None,
    };
    between
        .iter()
        .chain(axes)
        .any(|&dir| apart_along(x, xi, y, yi, dir, pad, band, face_cover))
}

/// [`apart_by`] along one direction.
#[allow(clippy::too_many_arguments)] // the pair, the direction, the pad, the band and the cover rule
fn apart_along<T: Decide>(
    x: &Body<T>,
    xi: Item<T>,
    y: &Body<T>,
    yi: Item<T>,
    dir: UnitVec3<T>,
    pad: f64,
    band: Band,
    face_cover: &FaceCoverAlong<'_, T>,
) -> bool {
    let (Some((xl, xh)), Some((yl, yh))) = (
        span_by(x, [xi], dir, face_cover),
        span_by(y, [yi], dir, face_cover),
    ) else {
        return false;
    };
    let gap = (yl - xh).max(xl - yh) - T::from_f64(2.0 * pad);
    matches!(decide(PAIR_GAP, Margin::of(gap), band), Ok(Sign::Positive))
}

/// The span of `items` along `dir`: the least and greatest support any
/// of their covers has there. `None` where an item has no cover, or
/// there is no item.
pub(crate) fn support_along<T: Decide>(
    body: &Body<T>,
    items: impl IntoIterator<Item = Item<T>>,
    dir: UnitVec3<T>,
    band: Band,
) -> Option<(T, T)> {
    span_by(body, items, dir, &|body, f, _| {
        crate::cover::face_cover(body, f, band).ok()
    })
}

/// [`support_along`], with each face's cover read by `face_cover`.
fn span_by<T: Decide>(
    body: &Body<T>,
    items: impl IntoIterator<Item = Item<T>>,
    dir: UnitVec3<T>,
    face_cover: &FaceCoverAlong<'_, T>,
) -> Option<(T, T)> {
    let d = dir.get();
    let mut span: Option<(T, T)> = None;
    for item in items {
        let cover = cover(body, item, dir, face_cover)?;
        let (lo, hi) = cover.range_along(d);
        span = Some(span.map_or((lo, hi), |(l, h)| (l.min(lo), h.max(hi))));
    }
    span
}

/// An item's cover, a face's by `face_cover` along `dir`; a ball's is
/// its tube with no spine.
fn cover<T: Decide>(
    body: &Body<T>,
    item: Item<T>,
    dir: UnitVec3<T>,
    face_cover: &FaceCoverAlong<'_, T>,
) -> Option<Cover<T>> {
    match item {
        Item::Face(f) => face_cover(body, f, dir),
        Item::Edge(e) => crate::cover::edge_cover(body, e).ok(),
        Item::Circle(circle) => circle.cover(),
        Item::Ball { center, radius } => Cover::tube(
            &geom::Frame::new(center, Vec3::unit_z(), Vec3::unit_x()),
            T::zero(),
            radius,
        ),
    }
}

/// The mean of an item's boundary vertices, or a circle's or ball's
/// centre — a point that moves with its body. `None` for a face whose
/// boundary has no vertex.
pub(crate) fn anchor<T: Decide>(body: &Body<T>, item: Item<T>) -> Option<Point3<T>> {
    use crate::cover::edge_end_point;
    let mut points = Vec::new();
    match item {
        Item::Edge(ek) => {
            let e = crate::live::proven(&body.edges, ek, crate::entity::EntityId::Edge);
            points.push(edge_end_point(body, ek, e.he_plus, "he_plus"));
            points.push(edge_end_point(body, ek, e.he_minus, "he_minus"));
        }
        Item::Face(f) => {
            let face = body.proven_face(f);
            for member in face.boundary() {
                points.push(match member {
                    BoundaryMember::Isolated { point, .. } => point,
                    BoundaryMember::Edge { edge, .. } => {
                        let ek = edge.key();
                        edge_end_point(body, ek, edge.record().he_plus, "he_plus")
                    }
                });
            }
        }
        Item::Circle(Circle { center, .. }) | Item::Ball { center, .. } => return Some(center),
    }
    mean(points)
}

/// The mean of `points`; `None` for none.
pub(crate) fn mean<T: Real>(points: impl IntoIterator<Item = Point3<T>>) -> Option<Point3<T>> {
    let mut sum = Vec3::new(T::zero(), T::zero(), T::zero());
    let mut n = 0u32;
    for p in points {
        sum = sum + Vec3::new(p.x, p.y, p.z);
        n += 1;
    }
    (n > 0).then(|| {
        let k = T::one() / T::from_f64(f64::from(n));
        Point3::new(sum.x * k, sum.y * k, sum.z * k)
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use geom::Surface;
    use geom_core::{RigidMotion, Tol};

    use super::*;
    use crate::test_support_fixtures::geometric_cube;

    /// **The axis merge is exact identity, not a tolerance.** Two copies
    /// of a cube read three axes, each face's normal merged with its
    /// copy's and its opposite's; turned by one ulp of a radian, the
    /// copy's normals are other values and are kept, six in all. Red
    /// where axes are merged within a tolerance, which would make the
    /// merge a decision taken outside the funnel.
    #[test]
    fn the_axis_merge_keeps_two_normals_one_ulp_apart() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let cube = geometric_cube::<f64>(tol).body;
        let axis = UnitVec3::new(Vec3::new(1.0, 2.0, 3.0), PAIR_AXIS, band).unwrap();
        let turned = crate::transform::transform_rigid(
            &cube,
            &RigidMotion::rotation(axis, f64::EPSILON).unwrap(),
        )
        .unwrap();
        let key = crate::boxes::axis_key;
        assert_eq!(
            operand_axes(&cube, &cube, band, key).len(),
            3,
            "one value, merged"
        );
        assert_eq!(
            operand_axes(&cube, &turned, band, key).len(),
            6,
            "one ulp apart, kept"
        );
    }

    /// **A planar face's normal is levered at the face's extent from
    /// below**, its longest certified boundary edge's extent. The unit
    /// cube's top: its edges are 1 long and its points stand up to `√2`
    /// apart, so the arm is 1, not the face's diameter, and not less.
    ///
    /// The arm cannot turn a built face's verdict: a plane's normal is
    /// unit by construction, and an edge shorter than the band's
    /// escalation does not certify, so `|n|·arm` clears the band on every
    /// face there is. The row pins the arm.
    #[test]
    fn a_planar_faces_normal_is_levered_at_its_extent_from_below() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let body = geometric_cube::<f64>(tol).body;
        let top = body
            .faces()
            .map(|(k, _)| k)
            .find(|&k| {
                matches!(body.proven_face(k).surface(),
                    Surface::Plane { frame } if frame.axis().z > 0.5)
            })
            .unwrap();
        let arm = crate::cover::face_extent_below(&body, top)
            .unwrap()
            .length();
        assert!(
            (arm - 1.0).abs() < 1e-12,
            "the top's arm is its edges' extent, 1, got {arm}"
        );
        assert!(
            planar_axis(&body, top, band).is_some(),
            "the top reads its normal"
        );
    }
}

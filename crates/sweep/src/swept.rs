//! The lowering every profile sweep shares: the carrier class of one
//! swept segment, the sketch-level quantities derived from it (apex,
//! span, turn-signed axis), the edge spec a placed segment mints, the
//! cap-plane point list, the cosurface decision, and the two
//! crate-wide accessors (the classification funnel, a face's surface
//! key).
//!
//! This module is a sibling of the sweep verbs, not a member of one:
//! its consumers are `extrude`, `revolve` and `loft`, and a core
//! hosted inside one of its consumers is the shape that drifts.
//!
//! What is deliberately NOT here: anything a verb decides
//! differently. The wall-orientation sense is per-verb (extrude reads
//! the canonical turn, revolve reads the wall class, loft is uniform),
//! and the strut carrier is per-verb (a translation trajectory is a
//! line, a rotation trajectory is a circle). The swept-traversal
//! builder is not here either, but for a weaker reason: extrude's has
//! a reversal arm and mints the orientation bit, revolve's has
//! neither, and loft imports extrude's rather than owning one — so it
//! is two implementations serving three verbs, not three.

use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeGeometry, MappedCurve, SketchSegment};
use geom_core::{
    Affine3, Band, Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Vec2, Vec3,
};
use topo::{Body, EulerOpError, FaceKey, SurfaceKey};

/// The classification funnel of this shared lowering, and of `extrude`
/// and `revolve` above it (the `geom-brep` pattern). It is not the
/// crate's only one: `fillet` keeps a second copy, and `loft` and
/// three other sites reach past a funnel to the recorder directly.
/// Delegates to the unified recorder funnel
/// [`geom_core::k_stats::decide`], so every decision's predicate name
/// reaches the margin-telemetry recorder. The name is a parameter —
/// each verb keeps its own predicate names through one shared body.
pub(crate) fn decide<T: Decide>(
    name: &'static str,
    margin: Margin<T>,
    band: Band,
) -> Result<Sign, Indeterminate> {
    geom_core::k_stats::decide(name, margin, band)
}

/// A segment's carrier class in swept traversal order (the canonical
/// classification carried through any reversal — never re-decided from
/// scalar data here).
#[derive(Clone, Copy, Debug)]
pub(crate) enum SweptKind<T: Real> {
    Line,
    Arc {
        center: Point2<T>,
        radius: T,
        /// Turn sense in swept traversal: `Positive` = counterclockwise
        /// in sketch coordinates. Never `Zero` (upstream classification;
        /// kept total — a `Zero` would take the `Positive` arm).
        turn: Sign,
    },
}

/// The sketch-level chord data this module's lowering reads from a
/// swept segment: endpoints in swept traversal order, the bulge in
/// that order, and the carrier class.
///
/// It is a trait rather than a struct because the verbs' segment
/// records are not the same type and must not become one — extrude's
/// carries a wall-orientation bit derived from the canonical turn that
/// would be wrong for the other verbs. Everything below this line is
/// the same for all of them; everything above it is not.
pub(crate) trait SweptChord<T: Real> {
    /// Start point, sketch coordinates.
    fn a(&self) -> Point2<T>;
    /// End point, sketch coordinates.
    fn b(&self) -> Point2<T>;
    /// The bulge in swept traversal order.
    fn bulge(&self) -> T;
    /// The carrier class in swept traversal order.
    fn kind(&self) -> SweptKind<T>;
}

/// The segment as a `geom-brep` sketch segment (the description's
/// authoritative source data).
///
/// A free function over the accessors rather than a provided method:
/// the point of the trait is that the four accessors are all a verb
/// gets to supply, and a provided method is one an impl may quietly
/// override — which would put the body back to two.
pub(crate) fn sketch_segment<T: Real, S: SweptChord<T>>(seg: &S) -> SketchSegment<T> {
    match seg.kind() {
        SweptKind::Line => SketchSegment::Line {
            a: seg.a(),
            b: seg.b(),
        },
        SweptKind::Arc { .. } => SketchSegment::Arc {
            a: seg.a(),
            b: seg.b(),
            bulge: seg.bulge(),
        },
    }
}

/// The arc apex (the profile crate's exact sagitta closed form:
/// `midpoint − n̂·(L·b/2)`, n̂ the left normal of the chord direction) —
/// an on-carrier interior point of the segment, and the point that
/// keeps a 2-vertex loop plane-determining.
///
/// Takes the raw chord data rather than a [`SweptChord`]: the axis
/// classification needs the apex of a canonical profile segment, which
/// is not part of any swept traversal.
pub(crate) fn arc_apex<T: Real>(a: Point2<T>, b: Point2<T>, bulge: T) -> Point2<T> {
    let chord = b - a;
    let len = chord.norm();
    let u = chord.normalize();
    let nhat = Vec2::new(T::zero() - u.y, u.x);
    let mid = a.lerp(b, T::from_f64(0.5));
    mid - nhat * (len * bulge * T::from_f64(0.5))
}

/// The arc parameter span θ = 4·atan|bulge| (the sanctioned bulge
/// re-inspection — never endpoint `atan2`).
pub(crate) fn arc_span<T: Real>(bulge: T) -> T {
    T::from_f64(4.0) * bulge.abs().atan()
}

/// The turn-signed carrier axis (crate docs): `+normal` for a
/// counterclockwise segment, `−normal` for a clockwise one. `Zero` is
/// unreachable for classified arcs (a zero turn classified as a line);
/// kept total by taking the positive arm.
pub(crate) fn turn_axis<T: Real>(turn: Sign, normal: Vec3<T>) -> Vec3<T> {
    match turn {
        Sign::Positive | Sign::Zero => normal,
        Sign::Negative => Vec3::zero() - normal,
    }
}

/// The edge spec of a profile segment carried into 3-space by one
/// placement: `PlacedSegment` description, line or circle carrier per
/// the crate docs' carrier conventions (arc axis = turn-signed plane
/// normal, span θ = 4·atan|bulge| from the stored bulge).
///
/// `place` and `normal` are the placement the segment is lowered
/// through and its plane normal — the sketch placement for a base
/// lamina, the translated or rotated one for the swept copy.
pub(crate) fn placed_segment_spec<T: Real, S: SweptChord<T>>(
    seg: &S,
    place: Affine3<T>,
    normal: Vec3<T>,
    q_from: Point3<T>,
    q_to: Point3<T>,
) -> EdgeCurveSpec<T> {
    let description = EdgeGeometry::MappedCurve(MappedCurve::PlacedSegment {
        segment: sketch_segment(seg),
        place,
    });
    match seg.kind() {
        SweptKind::Line => EdgeCurveSpec {
            description,
            carrier: Curve3::Line {
                origin: q_from,
                dir: (q_to - q_from).normalize(),
            },
            param_start: T::zero(),
            param_end: q_from.distance(q_to),
        },
        SweptKind::Arc {
            center,
            radius,
            turn,
        } => {
            let c_world = place.transform_point(Point3::new(center.x, center.y, T::zero()));
            EdgeCurveSpec {
                description,
                carrier: Curve3::Circle {
                    center: c_world,
                    axis: turn_axis(turn, normal),
                    radius,
                    u_ref: (q_from - c_world).normalize(),
                },
                param_start: T::zero(),
                param_end: arc_span(seg.bulge()),
            }
        }
    }
}

/// The world points determining a cap plane, in forward swept order:
/// every loop vertex, plus every arc segment's apex. The apexes keep
/// 2-vertex loops (the minimal circle) plane-determining — Newell needs
/// three points and a 2-vertex cap has only two vertices — and they
/// carry the traversal's winding faithfully (each sits between its
/// segment's endpoints in loop order).
///
/// `qs` are the world vertices and `place` the matching placement, so
/// a rotated or translated cap passes the rotated or translated pair.
pub(crate) fn cap_points<T: Real, S: SweptChord<T>>(
    segs: &[S],
    qs: &[Point3<T>],
    place: Affine3<T>,
) -> Vec<Point3<T>> {
    let mut pts = Vec::with_capacity(segs.len() * 2);
    for (j, s) in segs.iter().enumerate() {
        pts.push(qs[j]);
        if matches!(s.kind(), SweptKind::Arc { .. }) {
            let apex = arc_apex(s.a(), s.b(), s.bulge());
            pts.push(place.transform_point(Point3::new(apex.x, apex.y, T::zero())));
        }
    }
    pts
}

/// The predicate names one verb's cosurface decision reports under —
/// the K recorder meters each sweep's line and arc margins separately,
/// so the names are per-verb data passed into the shared body rather
/// than a property of the body.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CosurfaceNames {
    /// The line/line predicate (chord collinearity).
    pub(crate) lines: &'static str,
    /// The arc/arc predicate (carrier identity).
    pub(crate) arcs: &'static str,
}

/// Decides whether two consecutive segments sweep the
/// identical-by-construction surface (crate docs, cosurface sharing):
/// collinear lines share one surface, same-carrier same-turn arcs
/// share one. The sweep's own surface family does not enter — swept
/// surfaces of identical generators under one motion coincide, so the
/// margins are the sketch-level ones for every verb. Mixed kinds never
/// share (structurally); arcs with opposite turns never share
/// (structurally — a same-carrier opposite-turn pair is an overlap the
/// profile validator already refused; checked defensively).
pub(crate) fn cosurface<T: Decide, S: SweptChord<T>>(
    prev: &S,
    next: &S,
    names: CosurfaceNames,
    band: Band,
) -> Result<bool, Indeterminate> {
    match (prev.kind(), next.kind()) {
        (SweptKind::Line, SweptKind::Line) => {
            // Margin: perpendicular distance of the next chord's far
            // endpoint from the previous chord's carrier line (meters,
            // direct displacement).
            let t = (prev.b() - prev.a()).normalize();
            let d = next.b() - prev.a();
            let margin = t.perp_dot(d);
            Ok(matches!(
                decide(names.lines, Margin::of(margin), band)?,
                Sign::Zero
            ))
        }
        (
            SweptKind::Arc {
                center: c1,
                radius: r1,
                turn: t1,
            },
            SweptKind::Arc {
                center: c2,
                radius: r2,
                turn: t2,
            },
        ) => {
            if t1 != t2 {
                return Ok(false);
            }
            // Margin: center distance plus radius difference (meters,
            // direct — the profile crate's carrier-identity pattern).
            let margin = c1.distance(c2) + (r1 - r2).abs();
            Ok(matches!(
                decide(names.arcs, Margin::of(margin), band)?,
                Sign::Zero
            ))
        }
        _ => Ok(false),
    }
}

/// Resolves a face's surface key (total: a stale key surfaces as the
/// operator-layer typed error, which every sweep verb's error enum
/// absorbs through its `From<EulerOpError>`).
pub(crate) fn face_surface_key<T: Real>(
    body: &Body<T>,
    face: FaceKey,
) -> Result<SurfaceKey, EulerOpError> {
    Ok(body
        .get_face(face)
        .ok_or(EulerOpError::StaleKey {
            key: topo::EntityId::Face(face),
        })?
        .surface)
}

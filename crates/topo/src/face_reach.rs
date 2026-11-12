//! **How far a face reaches**: the lever arms a reading over a face
//! divides its angular residuals by, read off the face's boundary from
//! a point ([`face_extent`], [`face_reach_from`],
//! [`face_reach_round_from`]), along an axis ([`face_axial_range`]) and
//! against a cone's axis ([`cone_axial_window`], [`cone_face_reach`]).

use geom_core::{Band, Decide, Lever, Point3, Real};

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, VertexKey};
use crate::live::{BoundaryMember, Proven};

/// The face-extent lever arm for the coplanarity/sense predicates: the
/// farthest distance from the base vertex to any point of the face's
/// boundary — its vertices, and each curved edge's far reach
/// ([`face_reach_from`]), so a one-vertex face bounded by a closed edge has
/// the arm its edge spans, not zero — the largest displacement a
/// normal-angle error can induce across this face (D4 ¶1's "face
/// extent" arm, computed, named).
///
/// The arm must be an OVER-estimate of that displacement: it divides
/// out of the caller's angular residuals ([`Margin::levered`]), so an
/// under-claimed arm shrinks the margin and makes a face that is not
/// parallel to the split plane likelier to decide `Zero` — a wrong
/// answer, not a loud one. Two loop shapes have to be read with that
/// in mind rather than walked past:
///
/// - A [`LoopBoundary::Empty`] **ring** is an isolated vertex, and
///   that vertex is part of the face's boundary — it contributes its
///   own distance like any other boundary vertex.
/// - A [`LoopBoundary::Empty`] **outer** loop means the face has no
///   outer boundary at all, so its locus is unbounded and no finite
///   arm over-estimates anything. That is refused, not measured.
///   `validate_closed`'s tier-2 check 1 rejects every empty loop, so a
///   validated operand cannot carry one. The split's doors run tier 2
///   on an operand that carries no verdict, but this read does not
///   assume its caller's gate (a test-support door reaches it past
///   that gate), which is why the refusal is here rather than assumed.
///
/// The refusal is [`UnboundedFace`], naming the face
/// and the loop's lone vertex. Every caller resolves `vertex` and
/// `face` in the same `&Body` call, by a refusal of its own or as a
/// record it just read, so every hop past them is a link, and a miss
/// panics.
///
/// [`LoopBoundary::Empty`]: crate::entity::LoopBoundary::Empty
/// [`Margin::levered`]: geom_core::Margin::levered
pub(crate) fn face_extent<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    face: FaceKey,
) -> Result<Lever<T>, UnboundedFace> {
    face_reach_from(body, face, body.resolve_vertex_point(vertex, Proven))
}

/// [`face_extent`] from any point `at`: the farthest `face`'s boundary
/// stands from it, each certified edge levered as its
/// [`geom_brep::Reach::Span`] is. The refusal is [`face_extent`]'s.
///
/// On a plane or a cylinder that is the face's own reach: the line or
/// ruling through an interior point meets the boundary both ways, and a
/// distance is convex along it. So an edge there is read over the span
/// it holds ([`geom_brep::Reach::span_reach_from`]: a circle arc
/// exactly, an ellipse arc by its quarters' chords and bulges, a
/// segment's ends). Elsewhere the face
/// can stand farthest inside its boundary (a sphere's or a torus's far
/// side, a cone's apex), and an edge is levered round its whole carrier
/// ([`geom_brep::Reach::lever_from`]).
pub(crate) fn face_reach_from<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: Point3<T>,
) -> Result<Lever<T>, UnboundedFace> {
    let over_span = matches!(
        body.proven_face(face).surface(),
        geom::Surface::Plane { .. } | geom::Surface::Cylinder { .. }
    );
    boundary_lever(body, face, at, |curve| {
        let span = span_of(curve);
        if over_span {
            span.span_reach_from(at)
        } else {
            span.lever_from(at)
        }
    })
}

/// [`face_reach_from`] with every edge levered round its whole carrier
/// ([`geom_brep::Reach::lever_from`]) whatever the face it bounds: the
/// measure the germ frame reads beside the span's, serving only where
/// the two agree. The refusal is [`face_extent`]'s.
pub(crate) fn face_reach_round_from<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: Point3<T>,
) -> Result<Lever<T>, UnboundedFace> {
    boundary_lever(body, face, at, |curve| span_of(curve).lever_from(at))
}

/// **How far a face reaches from `at` either way along `axis`** (unit):
/// `(below, above)`, the farthest `−(x − at)·axis` and `(x − at)·axis`
/// over `face`'s boundary, each at least zero: its vertices, and each
/// certified edge's carrier over the span it holds
/// ([`geom_brep::Reach::range_along`] of its [`geom_brep::Reach::Span`]),
/// so a curved edge's bulge past every vertex is reached. A coordinate
/// along an axis has no interior extremum on a plane or a cylinder about
/// that axis, so the boundary's bounds are the face's. They lever a tilt
/// of `axis` read at `at`'s foot on it, whose reading moves by the tilt
/// times the axial distance: never inside the face, never past
/// [`face_reach_from`] the same point, and exact wherever the boundary's
/// edges are segments and conic arcs. A spiric edge is read at its
/// torus's support and a spline at its whole control net, which can
/// reach past the span the edge holds.
///
/// The refusal is [`face_extent`]'s, on the same loop shapes.
pub(crate) fn face_axial_range<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: Point3<T>,
    axis: geom_core::Vec3<T>,
) -> Result<(T, T), UnboundedFace> {
    let along = |p: Point3<T>| (p - at).dot(axis);
    let above = boundary_reach(body, face, along, |curve| {
        span_of(curve).range_along(at, axis).1
    })?;
    let below = boundary_reach(
        body,
        face,
        |p| -along(p),
        |curve| -span_of(curve).range_along(at, axis).0,
    )?;
    Ok((below, above))
}

/// **How far along a cone's axis a face reaches**: bounds `(lo, hi)` on
/// `(x − apex)·axis` over `face`, read off its boundary as
/// [`face_axial_range`] reads it, each edge over the span it holds, so
/// a working body's null scaffolding is read as its chord.
///
/// A face on the cone itself can hold the apex inside its boundary (a
/// tip bounded by its rim alone), where its range is not its
/// boundary's: the axis coordinate peaks on a smooth surface only at
/// the boundary, and the apex is not smooth. So a face on a cone keeps
/// its boundary's range only where the face cover's apex rule
/// (`cover::cone_may_hold_apex`: every loop closes in the chart) says
/// it holds no apex; anywhere else the range is widened to hold it. A
/// face on any other surface keeps its boundary's range.
///
/// # Errors
///
/// A face whose outer loop is a lone vertex ([`face_extent`]).
pub(crate) fn cone_axial_window<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    cone: &geom::Surface<T>,
    band: Band,
) -> Result<(T, T), UnboundedFace> {
    let &geom::Surface::Cone { apex, axis, .. } = cone else {
        unreachable!("a window along a cone's axis is asked of a cone")
    };
    // A floor under every boundary reading, so the folds below start
    // past nothing the boundary holds.
    let floor = T::zero() - face_reach_from(body, face, apex)?.length();
    let along = |p: Point3<T>| (p - apex).dot(axis);
    let hi = boundary_fold(
        body,
        face,
        floor,
        (along, |curve| span_of(curve).range_along(apex, axis).1),
        Real::max,
    )?;
    let lo = T::zero()
        - boundary_fold(
            body,
            face,
            floor,
            (
                |p| T::zero() - along(p),
                |curve| T::zero() - span_of(curve).range_along(apex, axis).0,
            ),
            Real::max,
        )?;
    let own @ geom::Surface::Cone { .. } = body.proven_face(face).surface() else {
        return Ok((lo, hi));
    };
    if crate::cover::cone_may_hold_apex(body, face, own, band) {
        Ok((lo.min(T::zero()), hi.max(T::zero())))
    } else {
        Ok((lo, hi))
    }
}

/// **A face as the plane×cone arm's reach** ([`geom_brep::Reach::Face`]),
/// measured from `at` against `cone`'s axis: its window along the axis
/// ([`cone_axial_window`]), narrowed to `within` where the caller knows
/// the consumed region lies inside a second window too, and
/// [`face_reach_from`] `at` as its lever.
///
/// # Errors
///
/// A face whose outer loop is a lone vertex ([`face_extent`]).
pub(crate) fn cone_face_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: Point3<T>,
    (cone, within): (&geom::Surface<T>, Option<(T, T)>),
    band: Band,
) -> Result<geom_brep::Reach<T>, UnboundedFace> {
    let &geom::Surface::Cone { apex, axis, .. } = cone else {
        unreachable!("the plane×cone arm's reach is asked of a cone")
    };
    let (lo, hi) = cone_axial_window(body, face, cone, band)?;
    let (lo, hi) = within.map_or((lo, hi), |(from, to)| (lo.max(from), hi.min(to)));
    let at_axis = (at - apex).dot(axis);
    Ok(geom_brep::Reach::of_face(
        at,
        (at_axis - lo, hi - at_axis),
        face_reach_from(body, face, at)?,
    ))
}

/// A certified edge as the reach of the span it holds.
fn span_of<T: Decide>(curve: &geom_brep::EdgeCurve<T>) -> geom_brep::Reach<T> {
    let (t0, t1) = curve.params();
    geom_brep::Reach::Span {
        carrier: curve.carrier().clone(),
        t0,
        t1,
    }
}

/// The farthest `face`'s boundary reaches by a measure: `point` of each
/// boundary vertex, `edge` of each certified edge, refusing a face whose
/// outer loop is a lone vertex ([`face_extent`]'s docs).
fn boundary_reach<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    point: impl Fn(Point3<T>) -> T,
    edge: impl Fn(&geom_brep::EdgeCurve<T>) -> T,
) -> Result<T, UnboundedFace> {
    boundary_fold(body, face, T::zero(), (point, edge), Real::max)
}

/// The farthest `face`'s boundary reaches from `at`, as a lever: each
/// boundary vertex's distance, and `edge` of each certified edge, folded
/// as a read of their union ([`Lever::max`]). The refusal is
/// [`boundary_reach`]'s.
fn boundary_lever<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    at: Point3<T>,
    edge: impl Fn(&geom_brep::EdgeCurve<T>) -> Lever<T>,
) -> Result<Lever<T>, UnboundedFace> {
    let vertex = |p| geom_brep::ExtentBall::point(p).lever_from(at);
    let floor = geom_brep::ExtentBall::point(at).lever();
    boundary_fold(body, face, floor, (vertex, edge), Lever::max)
}

/// [`boundary_reach`] from `floor` rather than zero: the greatest, by
/// `max`, of `floor` and every boundary reading.
fn boundary_fold<T: Decide, R: Copy>(
    body: &Body<T>,
    face: FaceKey,
    floor: R,
    (point, edge): (
        impl Fn(Point3<T>) -> R,
        impl Fn(&geom_brep::EdgeCurve<T>) -> R,
    ),
    max: impl Fn(R, R) -> R,
) -> Result<R, UnboundedFace> {
    let view = body.proven_face(face);
    let mut extent = floor;
    for (_, members, role) in view.boundary_by_loop() {
        for member in members {
            let p = match member {
                // An unbounded face has no finite lever arm.
                BoundaryMember::Isolated { vertex: lone, .. } if role.is_outer() => {
                    return Err(UnboundedFace { face, vertex: lone });
                }
                BoundaryMember::Isolated { point, .. } => point,
                BoundaryMember::Edge {
                    he,
                    half,
                    ek,
                    edge: data,
                } => {
                    if let Some(curve) = body.edge_curve_linked(ek, data).certified() {
                        extent = max(extent, edge(curve));
                    }
                    body.linked_vertex_point(half.start, EntityId::HalfEdge(he), "start")
                }
            };
            extent = max(extent, point(p));
        }
    }
    Ok(extent)
}

/// [`face_extent`]'s refusal: `face`'s outer loop is the lone `vertex`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct UnboundedFace {
    pub(crate) face: FaceKey,
    pub(crate) vertex: VertexKey,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Tol;

    /// **The face extent panics on a ring link that does not resolve**,
    /// where it stepped over it.
    #[test]
    fn the_face_extent_panics_on_a_torn_ring_link() {
        use crate::live::OPERATORS_KEEP_LINKS;
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics, tear_ring};
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let vertex = body.vertices().next().map(|(k, _)| k).unwrap();
        assert!(face_extent(&body, vertex, face).is_ok(), "a bounded face");
        let named = tear_ring(&mut body, face);
        assert_torn_op_panics(
            "face_extent",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| face_extent(b, vertex, face),
        );
    }

    /// **A face's axial range reaches its rim's bulge past every vertex,
    /// and not round the wall.** The wall about `z` trimmed at `φ`, its
    /// rim one closed ellipse on the seam vertex (`oblique_rim_wall`), the
    /// rim's highest point at `φ > 0` and its lowest at `φ < 0`: from that
    /// vertex the rim reaches `2·tan |φ|` down the axis (its low crest) or
    /// up it (its high one) and nothing the other way, which the vertex
    /// alone (zero) misses, and [`face_extent`]'s distance round the rim,
    /// `2/cos φ`, over-states.
    #[test]
    fn a_faces_axial_extent_reaches_its_rims_bulge() {
        for phi in [0.2, core::f64::consts::FRAC_PI_4, 1.2, -0.2, -1.2] {
            let (body, face, vertex) = crate::test_support_fixtures::oblique_rim_wall(phi);
            let at = body.resolve_vertex_point(vertex, Proven);
            let (below, above) =
                face_axial_range(&body, face, at, geom_core::Vec3::unit_z()).unwrap();
            let euclid = face_extent(&body, vertex, face).unwrap().length();
            let bulge = 2.0 * phi.tan().abs();
            let (far, near) = if phi > 0.0 {
                (below, above)
            } else {
                (above, below)
            };
            assert!(
                (far - bulge).abs() <= 1e-12 * bulge && near.abs() <= 1e-12,
                "φ = {phi}: the rim reaches {below} below and {above} above, not the bulge {bulge}"
            );
            assert!(
                (euclid - 2.0 / phi.cos()).abs() <= 1e-12 * euclid,
                "φ = {phi}: the face extent {euclid} is the distance round the rim"
            );
        }
    }

    /// **A face's reach reads an arc over its span only where the
    /// boundary bounds the face.** One face whose boundary is the arc of
    /// the unit circle about `z` from `(1, 0, 0)` a hundredth of a turn
    /// round, read from its start. On a cylinder about `z` (a rim arc)
    /// the face reaches the arc's far end, its chord. On the unit sphere
    /// (an equator arc) a face can stand farthest inside its boundary,
    /// out to the far side 2 away, so the arc is levered round its whole
    /// turn.
    #[test]
    fn a_faces_reach_reads_an_arc_over_its_span_only_on_a_ruled_face() {
        use geom_core::Vec3;
        let carrier = geom::Curve3::circle(Point3::origin(), Vec3::unit_z(), 1.0, Vec3::unit_x());
        let t1 = 0.01 * core::f64::consts::TAU;
        let chord = (carrier.eval(t1) - carrier.eval(0.0)).norm();
        for (surface, want) in [
            (
                geom::Surface::Cylinder {
                    origin: Point3::origin(),
                    axis: Vec3::unit_z(),
                    radius: 1.0,
                    u_ref: Vec3::unit_x(),
                },
                chord,
            ),
            (
                geom::Surface::Sphere {
                    center: Point3::origin(),
                    radius: 1.0,
                    u_ref: Vec3::unit_x(),
                    axis: Vec3::unit_z(),
                },
                2.0,
            ),
        ] {
            let mut body = Body::<f64>::new();
            let seed = body.mvfs(carrier.eval(0.0), true).unwrap();
            body.set_face_surface(
                seed.face,
                crate::construction::FaceSurface::New {
                    surface: surface.clone(),
                    sense: true,
                },
            )
            .unwrap();
            let wall = body.get_face(seed.face).unwrap().surface;
            let rim_plane = body.add_surface(geom::Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            });
            body.mev(
                crate::construction::MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                carrier.eval(t1),
                geom_brep::EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::Intersection {
                        pair: geom_brep::SurfacePair::new(wall, rim_plane),
                        selector: geom_brep::CurveSelector::Witness(carrier.mid_point(0.0, t1)),
                    },
                    carrier: carrier.clone(),
                    param_start: 0.0,
                    param_end: t1,
                },
                Tol::witness(),
            )
            .unwrap();
            let got = face_extent(&body, seed.vertex, seed.face).unwrap().length();
            let slack = (1.0 - (t1 / 8.0).cos()) + 1e-15;
            assert!(
                got >= want - 1e-15 && got <= want + slack,
                "{surface:?}: the face reaches {got} from the arc's start, not {want}"
            );
        }
    }

    /// **An unbounded face has no lever arm.** `mvfs` seeds a face
    /// whose OUTER loop is a lone vertex, so the face's locus is the
    /// whole carrier: no finite distance over-estimates the
    /// displacement an angular error induces across it. `face_extent`
    /// used to walk past that loop and answer `0`, which is the
    /// under-claiming direction — a zero arm makes every angular
    /// residual decide `Zero`, i.e. coplanar.
    ///
    /// The zero answer was loud at ONE caller by accident (`apply_rule_a`
    /// gates on `split_sector_extent` being definitely positive) and
    /// silent at the other two, in `chord_join`, which pass the extent
    /// straight into `section_case`. The refusal is at the source.
    #[test]
    fn an_unbounded_face_has_no_lever_arm() {
        let mut body = Body::<f64>::new();
        let seed = body
            .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0), true)
            .unwrap();
        assert!(matches!(
            body.get_loop(seed.r#loop).unwrap().boundary,
            crate::entity::LoopBoundary::Empty { .. }
        ));
        assert!(
            matches!(
                face_extent(&body, seed.vertex, seed.face),
                Err(UnboundedFace { face, vertex }) if face == seed.face && vertex == seed.vertex
            ),
            "an empty OUTER loop refuses at its own lone vertex; it must not answer zero"
        );
    }

    /// **An isolated RING vertex is boundary, so it contributes.** The
    /// same `continue` also walked past a lone-vertex ring, whose
    /// vertex is a real point of the face's boundary and can be the
    /// farthest one. Planted on the cube's seed face, at the corner
    /// diagonally opposite the base vertex, it is strictly farther
    /// than every vertex of that face's own cycle — so the row is not
    /// vacuous, and it asserts that gap rather than just the maximum.
    #[test]
    fn an_isolated_ring_vertex_contributes_its_distance() {
        let mut cube = crate::test_support_fixtures::declined_cube::<f64>(Tol::witness());
        // The BOTTOM face, whose own cycle stops at the far bottom
        // corner; the seed (top) face already reaches the body's
        // farthest vertex, which would make the row vacuous.
        let face = cube.mefs[0].face;
        let base = cube.seed.vertex;
        let before = face_extent(&cube.body, base, face).unwrap().length();
        // The farthest vertex in the whole body from `base`.
        let p_base = *cube
            .body
            .get_point(cube.body.get_vertex(base).unwrap().point)
            .unwrap();
        let (far, far_d) = cube
            .body
            .vertex_points()
            .map(|(k, p)| (k, (p - p_base).norm()))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();
        assert!(
            far_d > before,
            "the fixture's precondition: the planted vertex must be farther \
             than the face's own cycle ({far_d} vs {before})"
        );
        let ring = cube.body.add_loop(
            crate::entity::Loop {
                boundary: crate::entity::LoopBoundary::Empty { vertex: far },
                face,
            },
            crate::provenance::Provenance::Primordial { op: "h14-row" },
        );
        cube.body.get_face_mut(face).unwrap().rings.push(ring);
        let after = face_extent(&cube.body, base, face).unwrap().length();
        assert!(
            (after - far_d).abs() < 1e-12,
            "the ring's lone vertex sets the arm: {after} vs {far_d}"
        );
    }
}

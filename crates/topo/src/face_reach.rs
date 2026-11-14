//! **How far a face reaches from one of its vertices**: the lever arm a
//! reading over a face divides its angular residuals by ([`face_extent`]),
//! its cover's far distance ([`crate::cover::face_reach_from`]). The
//! reaches along an axis and against a cone's axis are the cover's too
//! ([`crate::cover::face_axial_range`], [`crate::cover::cone_face_reach`]).

use geom_core::{Band, Decide, Lever};

use crate::body::Body;
use crate::entity::{FaceKey, VertexKey};
use crate::live::Proven;

/// The face-extent lever arm for the coplanarity/sense predicates: the
/// farthest any point of the face stands from the base vertex, its
/// cover's far distance ([`crate::cover::face_reach_from`]), the largest
/// displacement a normal-angle error can induce across this face (D4
/// ¶1's "face extent" arm, computed, named).
///
/// The arm must be an OVER-estimate of that displacement: it divides
/// out of the caller's angular residuals ([`Margin::levered`]), so an
/// under-claimed arm shrinks the margin and makes a face that is not
/// parallel to the split plane likelier to decide `Zero`: a wrong
/// answer, not a loud one. A face with no cover has no arm, and is
/// refused ([`UncoveredFace`]): a ruled face whose outer loop is a lone
/// vertex, whose locus is unbounded, or one whose boundary holds an edge
/// with no cover (null scaffolding among them).
///
/// [`Margin::levered`]: geom_core::Margin::levered
pub(crate) fn face_extent<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    face: FaceKey,
    band: Band,
) -> Result<Lever<T>, UncoveredFace> {
    crate::cover::face_reach_from(body, face, body.resolve_vertex_point(vertex, Proven), band)
        .map_err(|why| UncoveredFace { face, why })
}

/// [`face_extent`]'s refusal: `face` has no cover, and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct UncoveredFace {
    pub(crate) face: FaceKey,
    pub(crate) why: crate::cover::FaceUncovered,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::{Point3, Tol};

    fn wband() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **The face extent panics on a ring link that does not resolve**,
    /// where it stepped over it.
    #[test]
    fn the_face_extent_panics_on_a_torn_ring_link() {
        use crate::live::OPERATORS_KEEP_LINKS;
        use crate::review_d18::{ROW_FOUR, assert_torn_op_panics, tear_ring};
        let mut body = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness()).body;
        let face = body.faces().next().map(|(k, _)| k).unwrap();
        let vertex = body.vertices().next().map(|(k, _)| k).unwrap();
        assert!(
            face_extent(&body, vertex, face, wband()).is_ok(),
            "a bounded face"
        );
        let named = tear_ring(&mut body, face);
        assert_torn_op_panics(
            "face_extent",
            &mut body,
            &[&named, ROW_FOUR, OPERATORS_KEEP_LINKS],
            |b| face_extent(b, vertex, face, wband()),
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
                crate::cover::face_axial_range(&body, face, at, geom_core::Vec3::unit_z(), wband())
                    .unwrap();
            let euclid = face_extent(&body, vertex, face, wband()).unwrap().length();
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
            let got = face_extent(&body, seed.vertex, seed.face, wband())
                .unwrap()
                .length();
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
                face_extent(&body, seed.vertex, seed.face, wband()),
                Err(UncoveredFace { face, .. }) if face == seed.face
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
        let mut cube = crate::test_support_fixtures::geometric_cube::<f64>(Tol::witness());
        // The BOTTOM face, whose own cycle stops at the far bottom
        // corner; the seed (top) face already reaches the body's
        // farthest vertex, which would make the row vacuous.
        let face = cube.mefs[0].face;
        let base = cube.seed.vertex;
        let before = face_extent(&cube.body, base, face, wband())
            .unwrap()
            .length();
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
        let after = face_extent(&cube.body, base, face, wband())
            .unwrap()
            .length();
        assert!(
            (after - far_d).abs() < 1e-12,
            "the ring's lone vertex sets the arm: {after} vs {far_d}"
        );
    }
}

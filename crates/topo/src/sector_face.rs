//! The vertex-neighborhood **sector face**: which face a sector belongs
//! to, and that face's outward normal at the base vertex — one
//! implementation, shared by the two lanes that ask this question.
//!
//! # What the question is
//!
//! Both the splitting lane (`splitting::neighborhood`) and the boolean
//! lane ([`crate::boolean::sectors`]) walk a vertex orbit and, for the
//! sector CW-after orbit half-edge `he`, must resolve two things before
//! any predicate can run:
//!
//! 1. **Whose corner is this?** The sector after `he` is the corner of
//!    `face(loop(mate(he)))` — the traversal is the same on both sides,
//!    and its derivation lives in `splitting::neighborhood`'s module
//!    docs, which own it.
//! 2. **Which way is out, HERE?** Sector predicates meter against the
//!    face's outward normal **at the base vertex**, which for a charted
//!    face is a local quantity: a plane hands back its stored normal, a
//!    cylinder its chart-outward radial at the vertex point, a sphere
//!    the radial from its centre, a torus the offset from the vertex's
//!    foot on its core circle, a cone its chart normal off the
//!    generator through the vertex, a spline face its chart's
//!    `S_u × S_v` at the vertex's chart parameters.
//!
//! **Every arm folds in the face's `sense` bit** (S10) by minting an
//! [`OutwardNormal`]: each reads a CHART normal and returns it as the
//! face's OUTWARD normal, and the chart is the only encoding of
//! orientation the face has. This function is therefore the chokepoint
//! for both vertex-neighborhood lanes: everything downstream —
//! `splitting::rules::apply_rule_a`'s `enters_material` call, the
//! boolean's `within` / `side_code` / `sector_overlap` / `germ_dir` /
//! `pierce_germ_dir`, and the sector-shape rungs on both sides — is
//! sense-invariant GIVEN this value and must NOT multiply again. Those
//! sites pair the normal with the STORED orbit order, which `revert`
//! reverses in the same breath as the sense bit, so a second
//! sense fold would cancel this one.
//!
//! # Why the code is here and not in either lane
//!
//! This walk used to exist **twice**, once per lane, and the splitting
//! copy's own doc called itself *"the twin of"* the
//! boolean's. They differed in exactly three things, none of them
//! geometric: the error type they refuse into, whether the caller is
//! told the face was planar, and whether a `Sphere` face has a wired
//! arm. The first two are adaptation the callers do; the third is
//! reported here as a [`SectorCarrier`] and refused by the caller that
//! has no arm for it. What is left — the traversal and all three
//! normals — is one body.
//!
//! Like [`crate::sector_shape`], this module is a **top-level sibling**
//! of `boolean/` and `splitting/` precisely so neither half hosts the
//! other's core; both lanes already depend on this scope
//! (`crate::body`, `crate::entity`), so sharing here adds no dependency
//! edge between the halves and no public API.
//!
//! `sector_shape`'s docs name this unification as the trigger to
//! re-open whether that module belongs in `geom-brep` instead. **It
//! fires and resolves the other way**: this body takes a [`Body`] and
//! three arena keys, so `geom-brep` — which has no `Body` and no arena
//! — cannot host it at any price. The crate root is the deepest scope
//! that can, and the two shared sector modules belong together.

use geom::SurfaceKind;
use geom_brep::OutwardNormal;
use geom_core::{Band, Decide};

use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, VertexKey};
use crate::face_normal::face_outward_normal;

/// Which charted carrier a sector's face sits on — the arms with a
/// wired outward-normal-at-the-vertex construction.
///
/// This is a **capability** report, not a surface taxonomy: the kinds
/// with no arm never reach it (they refuse as
/// [`SectorFaceError::Unsupported`]), and a caller whose own pipeline
/// does not execute one of these arms refuses on it by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SectorCarrier {
    /// A plane: the normal is the stored chart normal.
    Plane,
    /// A cylinder: the normal is the chart-outward radial at the vertex.
    Cylinder,
    /// A sphere: the normal is the radial from the centre.
    Sphere,
    /// A ring torus: the normal is the point minus its foot on the
    /// core circle.
    Torus,
    /// A cone: the normal is the chart-outward one off the generator
    /// through the vertex, on that vertex's nappe.
    Cone,
    /// A spline chart (a `Nurbs` or an `Approx` face, the kind named):
    /// the normal is `S_u × S_v` at the vertex's chart parameters, read
    /// off the face's own pcurve of the edge into the vertex.
    Spline(SurfaceKind),
}

/// The resolved sector face.
#[derive(Debug)]
pub(crate) struct SectorFace<T: geom_core::Real> {
    /// The face the sector is a corner of.
    pub face: FaceKey,
    /// That face's OUTWARD unit normal at the base vertex, minted here
    /// from the chart normal and the face's `sense` bit — the type
    /// carries the obligation not to re-apply the sense downstream.
    pub normal: OutwardNormal<T>,
    /// Which arm produced the normal.
    pub carrier: SectorCarrier,
}

/// Why a sector face could not be resolved. Each lane maps this onto
/// its own error type — the boolean's twins additionally carry the
/// `Operand`, which is why the adaptation stays with the callers.
#[derive(Clone, Copy, Debug)]
pub(crate) enum SectorFaceError {
    /// The face's surface has no wired sector arm.
    Unsupported {
        /// The face whose carrier has no arm.
        face: FaceKey,
        /// The kind that has none.
        kind: SurfaceKind,
    },
    /// A spline face's chart has no one normal at the vertex: the vertex
    /// lies within the band of an irregular point of the chart (a pole,
    /// coincident control rows) or of a line its normal is not continuous
    /// across (an interior knot run of multiplicity at least the degree,
    /// a closed chart's seam), where a normal read off one side of the
    /// line is not the other's ([`geom_brep::chart_normal_at`]).
    Crease {
        /// The spline face.
        face: FaceKey,
    },
    /// A spline face's half-edge into the vertex has no chart image to
    /// read the vertex's chart parameters off: it stores no pcurve, or
    /// its edge no certified carrier.
    Unplaced {
        /// The spline face.
        face: FaceKey,
    },
}

/// The face orbit half-edge `he`'s sector at `vertex` is a corner of:
/// the face of its mate's loop. `vertex` and `he` are as for
/// [`resolve`].
#[track_caller]
pub(crate) fn face_of<T: Decide>(body: &Body<T>, vertex: VertexKey, he: HalfEdgeKey) -> FaceKey {
    let mate = body.proven_mate(he, crate::live::link(EntityId::Vertex(vertex), "orbit"));
    body.face_of_linked(mate.mate)
}

/// Resolves the sector CW-after `he` (`face(loop(mate(he)))`) to its
/// face, that face's outward normal at `vertex`, and the carrier arm
/// that produced the normal.
///
/// Named `resolve` rather than `sector_face`: the module is the noun,
/// and each lane's thin wrapper keeps that name for its own callers,
/// so `sector_face` in prose means one thing per scope instead of
/// three.
///
/// `vertex` and `he` are the caller's orbit walk's — a vertex it
/// resolved and a member of its orbit — so every hop from them is a
/// link of a record, and a miss panics naming it (D2 row 4).
///
/// `band` decides whether a spline face's vertex lies on a crease of
/// its chart.
///
/// # Errors
///
/// [`SectorFaceError`] — a carrier with no arm, or a spline face with no
/// one normal at the vertex or no chart image to read it at.
#[track_caller]
pub(crate) fn resolve<T: Decide>(
    body: &Body<T>,
    vertex: VertexKey,
    he: HalfEdgeKey,
    band: Band,
) -> Result<SectorFace<T>, SectorFaceError> {
    let face = face_of(body, vertex, he);
    // The planar arm goes through the crate's ONE sense-flip door
    // ([`crate::face_normal`]) rather than re-deriving the flip, and
    // reads nothing else: a plane's outward normal is a property of
    // the face alone.
    if let Some(normal) = face_outward_normal(body, face) {
        return Ok(SectorFace {
            face,
            normal,
            carrier: SectorCarrier::Plane,
        });
    }
    // The charted arms need the face's own chart and the base vertex.
    let face_data = crate::live::proven(&body.faces, face, EntityId::Face);
    let sense = face_data.sense;
    let point = || {
        body.point_of(
            vertex,
            crate::live::proven(&body.vertices, vertex, EntityId::Vertex),
        )
    };
    let charted = |chart, carrier| {
        Ok(SectorFace {
            face,
            normal: OutwardNormal::from_chart(chart, sense),
            carrier,
        })
    };
    let surface = body.face_surface_linked(face, face_data);
    if let Some(chart) = surface.spline_chart() {
        let kind = surface.kind();
        let at = (vertex, point());
        let normal = spline_normal(body, at, he, (face, sense), (chart, kind), band)?;
        return Ok(SectorFace {
            face,
            normal,
            carrier: SectorCarrier::Spline(kind),
        });
    }
    match surface {
        geom::Surface::Cylinder { origin, axis, .. } => {
            let w = point() - *origin;
            charted(
                (w - *axis * w.dot(*axis)).normalize(),
                SectorCarrier::Cylinder,
            )
        }
        geom::Surface::Sphere { center, .. } => {
            charted((point() - *center).normalize(), SectorCarrier::Sphere)
        }
        // The chart normal of a ring torus at `p` is `p` minus its foot
        // on the core circle, over `r` — the implicit gradient, read from
        // its one home rather than re-derived here. It is `0/0` only on
        // the axis, which no point of a ring torus's tube reaches
        // (`ρ ≥ R − r > 0`); a vertex there poisons the normal, and
        // poison escalates typed at the first decide that reads it.
        s @ geom::Surface::Torus { .. } => charted(
            geom_brep::implicit_gradient(s, point()).normalize(),
            SectorCarrier::Torus,
        ),
        // A cone's implicit gradient IS its chart normal, nappe sign
        // included. It poisons on the axis — at the apex, where no
        // tangent plane exists — and the poison escalates typed at the
        // first decide that reads it.
        s @ geom::Surface::Cone { .. } => charted(
            geom_brep::implicit_gradient(s, point()).normalize(),
            SectorCarrier::Cone,
        ),
        // The planar and spline arms returned above; anything else has
        // no arm.
        s => Err(SectorFaceError::Unsupported {
            face,
            kind: s.kind(),
        }),
    }
}

/// A spline face's outward normal at the vertex `at`, which ends the
/// face's half-edge `mate(he)`.
///
/// The vertex's chart parameters are that half-edge's pcurve (C4) at the
/// edge's parameter of the vertex, not a projection foot: the pcurve is
/// the edge's image in this face's own chart, so the parameters are on
/// the face's sheet by construction, where a foot of the point could
/// land on another sheet of the carrier that passes within the band.
/// The normal is read only where the chart has one there
/// ([`geom_brep::regular_jet`], [`geom_brep::chart_normal_at`]), over the
/// face's farthest reach from the vertex, its control net's.
fn spline_normal<T: Decide>(
    body: &Body<T>,
    (vertex, at): (VertexKey, geom_core::Point3<T>),
    he: HalfEdgeKey,
    (face, sense): (FaceKey, bool),
    (chart, kind): (&geom::NurbsSurface<T>, SurfaceKind),
    band: Band,
) -> Result<OutwardNormal<T>, SectorFaceError> {
    if chart.is_placeholder() {
        return Err(SectorFaceError::Unsupported { face, kind });
    }
    let into = body
        .proven_mate(he, crate::live::link(EntityId::Vertex(vertex), "orbit"))
        .mate;
    let unplaced = SectorFaceError::Unplaced { face };
    let he_data = crate::live::proven(&body.half_edges, he, EntityId::HalfEdge);
    let edge = crate::live::linked(
        &body.edges,
        he_data.edge,
        EntityId::Edge,
        EntityId::HalfEdge(he),
        "edge",
    );
    let curve = body
        .edge_curve_linked(he_data.edge, edge)
        .certified()
        .ok_or(unplaced)?;
    // `he` leaves the vertex, so the vertex is where it starts: the
    // carrier's first end where `he` walks it forward.
    let (t0, t1) = curve.params();
    let t = if he == edge.he_plus { t0 } else { t1 };
    let uv = body.pcurve(into).ok_or(unplaced)?.pcurve().eval(t);
    let uv = (uv.x, uv.y);
    let reach = chart
        .control()
        .iter()
        .map(|&c| (c - at).norm())
        .fold(T::zero(), T::max);
    let crease = |_| SectorFaceError::Crease { face };
    let jet = geom_brep::regular_jet(chart, uv, reach, band).map_err(crease)?;
    geom_brep::chart_normal_at(chart, sense, &jet, uv, reach, band)
        .map(|(normal, _)| normal)
        .map_err(crease)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::fixtures::raw_prism;
    use geom_core::Tol;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A sphere face, installed on a prism's side face so the orbit
    /// around one of its vertices walks a sphere-carried sector.
    fn sphere_sided_prism() -> (crate::Body<f64>, crate::entity::FaceKey) {
        let p = raw_prism(3, Tol::witness());
        let face = p.face_side[0];
        let mut body = p.body;
        body.set_face_surface(
            face,
            crate::FaceSurface::New {
                surface: geom::Surface::Sphere {
                    center: geom_core::Point3::new(0.0, 0.0, 0.0),
                    radius: 2.0,
                    axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
                    u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
                },
                sense: true,
            },
        )
        .unwrap();
        (body, face)
    }

    /// A torn link panics naming the record that holds it: the orbit
    /// member's `parent_loop`, not the base vertex the lanes' public
    /// arms are limited to.
    #[test]
    fn a_torn_link_panics_naming_its_record() {
        let p = raw_prism(3, Tol::witness());
        let mut body = p.body;
        let (orbit_he, _) = body.half_edges().next().expect("a half-edge");
        let vertex = body.get_half_edge(orbit_he).unwrap().start;
        let mate = body.mate(orbit_he).unwrap();
        body.get_half_edge_mut(mate).unwrap().parent_loop = crate::entity::LoopKey::default();
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = resolve(&body, vertex, orbit_he, band());
        }));
        let premise = format!(
            "{}'s parent_loop names {}, which does not resolve",
            EntityId::HalfEdge(mate),
            EntityId::Loop(crate::entity::LoopKey::default())
        );
        assert!(
            report.contains(&premise) && report.contains(crate::live::NAMES_ONLY_LIVE),
            "{report}"
        );
    }

    /// The charted arms read the base vertex, which the caller's walk
    /// resolved, so a vertex that does not resolve panics.
    #[test]
    fn a_charted_arm_panics_on_a_vertex_that_does_not_resolve() {
        let (body, face) = sphere_sided_prism();
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        // The sector CW-after `mate(he)` is this face's corner.
        let orbit_he = body.mate(he).unwrap();
        let bogus = crate::entity::VertexKey::default();
        let report = crate::surgery::tests::panic_message(std::panic::AssertUnwindSafe(|| {
            let _ = resolve(&body, bogus, orbit_he, band());
        }));
        assert!(
            report.contains(&format!(
                "{}, which this call resolved or read out of a record, does not resolve",
                EntityId::Vertex(bogus)
            )),
            "{report}"
        );
    }

    /// The sphere arm is WIRED here (the boolean lane executes it) and
    /// reports its carrier, so the lane without a sphere arm can refuse
    /// by name rather than by kind-matching a surface itself.
    #[test]
    fn the_sphere_arm_is_wired_and_names_its_carrier() {
        let (body, face) = sphere_sided_prism();
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        let orbit_he = body.mate(he).unwrap();
        let vertex = body.get_half_edge(orbit_he).unwrap().start;
        let resolved = resolve(&body, vertex, orbit_he, band()).expect("the sphere arm resolves");
        assert_eq!(resolved.carrier, SectorCarrier::Sphere);
        assert_eq!(resolved.face, face);
    }

    /// **The torus arm answers the tube's outward normal at the base
    /// vertex.** A torus is installed on a prism side face so that one
    /// of the face's vertices sits on the tube at a general minor angle
    /// `v`; the chart normal there is `cos v·x̂ + sin v·ẑ` (the offset
    /// from the vertex's foot on the core circle, over `r`). The pose
    /// separates it from both wrong constructions a torus arm could
    /// fall into — the axis radial (`x̂`, a cylinder's) and the offset
    /// from the centre (a sphere's) — and the face's sense bit is
    /// folded exactly once.
    #[test]
    fn the_torus_arm_is_the_tube_normal_at_the_base_vertex() {
        let p = raw_prism(3, Tol::witness());
        let face = p.face_side[0];
        let mut body = p.body;
        let he = body
            .get_loop(body.get_face(face).unwrap().outer)
            .and_then(|l| match l.boundary {
                crate::entity::LoopBoundary::Cycle { first } => Some(first),
                crate::entity::LoopBoundary::Empty { .. } => None,
            })
            .unwrap();
        let orbit_he = body.mate(he).unwrap();
        let vertex = body.get_half_edge(orbit_he).unwrap().start;
        let at = *body
            .get_point(body.get_vertex(vertex).unwrap().point)
            .unwrap();
        let (big_r, r, v) = (2.0, 0.5, 2.3_f64);
        let (sv, cv) = v.sin_cos();
        let x = geom_core::Vec3::new(1.0, 0.0, 0.0);
        let z = geom_core::Vec3::new(0.0, 0.0, 1.0);
        let center = at - x * (big_r + r * cv) - z * (r * sv);
        body.set_face_surface(
            face,
            crate::FaceSurface::New {
                surface: geom::Surface::Torus {
                    center,
                    axis: z,
                    major_radius: big_r,
                    minor_radius: r,
                    u_ref: x,
                },
                sense: true,
            },
        )
        .unwrap();
        let resolved = resolve(&body, vertex, orbit_he, band()).expect("the torus arm resolves");
        assert_eq!(resolved.carrier, SectorCarrier::Torus);
        assert_eq!(resolved.face, face);
        let tube = x * cv + z * sv;
        let want = OutwardNormal::from_chart(tube, body.get_face(face).unwrap().sense).vec();
        assert!(
            (resolved.normal.vec() - want).norm() < 1e-12,
            "{:?} is not the tube's outward normal {want:?}",
            resolved.normal.vec()
        );
    }

    /// The bilinear-in-`v` chart over the unit square at `z = 1` whose
    /// `u` columns stand at `xs` on the knots `ku`, `u` running along `x`
    /// and `v` along `y`: the plane `z = 1`, chart normal `+z`.
    fn planar_chart(ku: Vec<f64>, degree_u: usize, xs: &[f64]) -> geom::NurbsSurface<f64> {
        use geom_core::spline::KnotVector;
        let ku = KnotVector::clamped(ku, degree_u).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control: Vec<_> = xs
            .iter()
            .flat_map(|&x| [0.0, 1.0].map(|y| geom_core::Point3::new(x, y, 1.0)))
            .collect();
        let n = control.len();
        geom::NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap()
    }

    /// The unit cube with its top face recarried on `chart` at `sense`,
    /// its pcurves minted, and the top face.
    fn spline_topped_cube(chart: geom::NurbsSurface<f64>, sense: bool) -> (Body<f64>, FaceKey) {
        let cube = crate::test_support_fixtures::prism::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            1.0,
            Tol::witness(),
        );
        let mut body = cube.body;
        body.set_face_surface_unvouched_for_tests(
            cube.top_face,
            crate::FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(chart)),
                sense,
            },
        )
        .unwrap();
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        (body, cube.top_face)
    }

    /// The orbit half-edge at `vertex` whose sector is a corner of `face`.
    fn sector_of(body: &Body<f64>, vertex: VertexKey, face: FaceKey) -> HalfEdgeKey {
        body.vertex_orbit_linked(vertex)
            .into_iter()
            .find(|&he| face_of(body, vertex, he) == face)
            .expect("the vertex is a corner of the face")
    }

    /// The vertices of `face`'s outer loop.
    fn corners(body: &Body<f64>, face: FaceKey) -> Vec<VertexKey> {
        let crate::entity::LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(face).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("the face's outer loop is a cycle");
        };
        body.loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| body.get_half_edge(he).unwrap().start)
            .collect()
    }

    /// **A spline face's sector normal is its chart's `S_u × S_v` at the
    /// vertex, with the face's sense folded once.** The cube's top face on
    /// a planar spline chart reads `+z` at each corner whichever way the
    /// chart runs: on the chart along `(x, y)`, normal `+z`, sense true;
    /// and on its transpose, normal `−z`, sense false.
    #[test]
    fn a_spline_face_reads_its_outward_chart_normal_at_each_corner() {
        let chart = planar_chart(vec![0.0, 0.0, 1.0, 1.0], 1, &[0.0, 1.0]);
        for (chart, sense) in [(chart.clone(), true), (chart.transposed(), false)] {
            let (body, top) = spline_topped_cube(chart, sense);
            let vertices = corners(&body, top);
            assert_eq!(vertices.len(), 4, "the top face has four corners");
            for v in vertices {
                let resolved = resolve(&body, v, sector_of(&body, v, top), band())
                    .unwrap_or_else(|e| panic!("sense {sense}, corner {v:?}: {e:?}"));
                assert_eq!(resolved.face, top);
                assert_eq!(
                    resolved.carrier,
                    SectorCarrier::Spline(SurfaceKind::Nurbs),
                    "sense {sense}"
                );
                let n = resolved.normal.vec();
                assert!(
                    (n - geom_core::Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-12,
                    "sense {sense}, corner {v:?}: {n:?} is not the outward +z"
                );
            }
        }
    }

    /// **A vertex on a crease of the chart refuses; the same vertex on a
    /// smooth knot reads.** The top face's `y = 0` edge is split at
    /// `x = ½`, where the chart's `u` knot `½` lies. On a degree-1 chart
    /// that knot is a crease run (multiplicity at least the degree), where
    /// the face has a normal on each side, so the vertex refuses typed. On
    /// a degree-2 chart the same simple knot is smooth, and the vertex
    /// reads `+z`.
    #[test]
    fn a_vertex_on_a_crease_of_the_chart_refuses_and_one_on_a_smooth_knot_reads() {
        let creased = planar_chart(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1, &[0.0, 0.5, 1.0]);
        let smooth = planar_chart(
            vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0],
            2,
            &[0.0, 0.25, 0.75, 1.0],
        );
        for (chart, crease) in [(creased, true), (smooth, false)] {
            let (mut body, top) = spline_topped_cube(chart, true);
            let edge = body
                .edges()
                .map(|(e, _)| e)
                .find(|&e| {
                    let c = body.stored_curve(e).unwrap();
                    let (t0, t1) = c.params();
                    let (a, b) = (c.carrier().eval(t0), c.carrier().eval(t1));
                    a.z == 1.0 && b.z == 1.0 && a.y == 0.0 && b.y == 0.0
                })
                .expect("the top face's y = 0 edge");
            let (t0, t1) = body.stored_curve(edge).unwrap().params();
            let mid = body
                .split_edge(edge, 0.5 * (t0 + t1), Tol::witness())
                .unwrap()
                .vertex;
            let got = resolve(&body, mid, sector_of(&body, mid, top), band());
            if crease {
                assert!(
                    matches!(got, Err(SectorFaceError::Crease { face }) if face == top),
                    "a vertex on the crease refuses: {got:?}"
                );
            } else {
                let n = got.expect("a vertex on a smooth knot reads").normal.vec();
                assert!(
                    (n - geom_core::Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-12,
                    "{n:?}"
                );
            }
        }
    }

    /// **A vertex at an irregular point of the chart refuses.** The top
    /// face of the prism over `(0, 0), (1, 0), (½, ½), (0, 1)` on the
    /// bilinear chart through those four corners: its Jacobian is
    /// `1 − (u + v)/2`, which vanishes at the corner `(½, ½)` alone, where
    /// `S_u` and `S_v` run back along the two collinear edges that meet
    /// there. `S_u × S_v` has no direction at that corner and the face
    /// no normal to read, though it is a plane; the other three corners
    /// read `+z`.
    #[test]
    fn a_vertex_at_an_irregular_point_of_the_chart_refuses() {
        use geom_core::Point3;
        use geom_core::spline::KnotVector;
        let lin = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let chart = geom::NurbsSurface::new(
            lin(),
            lin(),
            vec![
                Point3::new(0.0, 0.0, 1.0),
                Point3::new(0.0, 1.0, 1.0),
                Point3::new(1.0, 0.0, 1.0),
                Point3::new(0.5, 0.5, 1.0),
            ],
            vec![1.0; 4],
        )
        .unwrap();
        let prism = crate::test_support_fixtures::prism::<f64>(
            &[(0.0, 0.0), (1.0, 0.0), (0.5, 0.5), (0.0, 1.0)],
            1.0,
            Tol::witness(),
        );
        let (mut body, top) = (prism.body, prism.top_face);
        body.set_face_surface_unvouched_for_tests(
            top,
            crate::FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(chart)),
                sense: true,
            },
        )
        .unwrap();
        crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
        let vertices = corners(&body, top);
        assert_eq!(vertices.len(), 4, "the top face has four corners");
        for v in vertices {
            let at = *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let got = resolve(&body, v, sector_of(&body, v, top), band());
            if at.x == 0.5 {
                assert!(
                    matches!(got, Err(SectorFaceError::Crease { face }) if face == top),
                    "{at:?}: the irregular corner refuses: {got:?}"
                );
            } else {
                let n = got.expect("a regular corner reads").normal.vec();
                assert!(
                    (n - geom_core::Vec3::new(0.0, 0.0, 1.0)).norm() < 1e-12,
                    "{at:?}: {n:?}"
                );
            }
        }
    }
}

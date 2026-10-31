//! **A vertex piercing a spline face** ([`super::classify_vertex_on_face`]):
//! the pierced face's outward normal is its chart's at the chart point
//! the sweep placed the pierce at, in the face's sense
//! ([`crate::face_normal::spline_outward_normal_at`]).
//!
//! The pierced body is the unit cube with its top recarried on the bump
//! `z = 1 + 0.4·u(1 − u)` (`x = u`, `y = v`), whose normal at `u = ¼`
//! leans `(−0.2, 0, 1)`. The piercing body is a thin triangular prism
//! whose corner sits on the bump at `(¼, ½)`: its two horizontal edges
//! run toward `−x` or `+x`, where the bump falls away below or rises
//! above them, and its third edge is vertical. Against `+z`, the bump's
//! corners' plane, both horizontal edges would read `On`.
//!
//! The ring the pierce mints ([`super::ring_vertex_in`]) is read on a
//! minted solid: the unit cube with its top on [`dome`], whose edges lie
//! on the net, so the wall stores its rows.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::boolean::{BooleanError, BooleanOp, GluedPairs, Operand, ReductionContacts, SideCode};
use crate::{Body, FaceKey, VertexKey};
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

const UV: (f64, f64) = (0.25, 0.5);

fn bump() -> geom::NurbsSurface<f64> {
    use geom_core::spline::KnotVector;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<_> = [(0.0, 1.0), (0.5, 1.2), (1.0, 1.0)]
        .iter()
        .flat_map(|&(x, z)| [0.0, 1.0].map(|y| Point3::new(x, y, z)))
        .collect();
    geom::NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap()
}

/// The unit cube with its top on [`bump`] at `sense`, and the top face.
fn bumped_cube(sense: bool) -> (Body<f64>, FaceKey) {
    let cube = crate::test_support_fixtures::prism::<f64>(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
        Tol::witness(),
    );
    let mut body = cube.body;
    body.set_face_surface_unvouched_for_tests(
        cube.top_face,
        crate::construction::FaceSurface::New {
            surface: geom::Surface::Nurbs(std::sync::Arc::new(bump())),
            sense,
        },
    )
    .unwrap();
    (body, cube.top_face)
}

/// The thin prism whose corner is the bump's point at [`UV`]: above it,
/// its horizontal edges toward `−x` (`toward = −1`), or below it, toward
/// `+x` (`toward = 1`). The body and the corner.
fn prism_at_the_bump(toward: f64) -> (Body<f64>, VertexKey) {
    let p = bump().eval(UV.0, UV.1);
    let profile = [
        (p.x, p.y),
        (p.x + 0.2 * toward, p.y - 0.06 * toward),
        (p.x + 0.2 * toward, p.y + 0.06 * toward),
    ];
    let (z0, z1) = if toward < 0.0 {
        (p.z, p.z + 0.2)
    } else {
        (p.z - 0.2, p.z)
    };
    let prism = crate::test_support_fixtures::prism_z::<f64>(&profile, z0, z1, Tol::witness());
    let corner = if toward < 0.0 {
        prism.bottom[0]
    } else {
        prism.top[0]
    };
    let at = *prism
        .body
        .get_point(prism.body.get_vertex(corner).unwrap().point)
        .unwrap();
    assert!(
        (at - p).norm() < 1e-12,
        "the corner is the bump's point: {at:?}"
    );
    (prism.body, corner)
}

fn classify(
    piercing: &mut Body<f64>,
    corner: VertexKey,
    (pierced, face): (&Body<f64>, FaceKey),
    chart: Option<(f64, f64)>,
) -> Result<Vec<SideCode>, BooleanError> {
    super::classify_vertex_on_face(
        piercing,
        pierced,
        Operand::A,
        crate::boolean::VfContact {
            vertex: corner,
            face,
        },
        BooleanOp::Union,
        &GluedPairs::default(),
        &ReductionContacts::default(),
        None,
        None,
        chart,
        &crate::sector_shape::SlitTips::default(),
        band(),
    )
    .map(|out| out.classes.into_iter().map(|(_, c)| c).collect())
}

/// **A vertex piercing a NURBS wall reads each of its edges on the
/// side the chart's normal at the placed point puts it, in either
/// sense of the face.** Above the bump every edge is outside the cube's
/// material and below it every edge is inside; the `false` face's
/// material is the other side, and every class swaps. A normal read as
/// `+z` reads the horizontal edges `On`; one read without the sense, or
/// with it folded twice, swaps one sense's classes.
#[test]
fn a_vertex_piercing_a_nurbs_wall_reads_its_side_in_either_sense() {
    for sense in [true, false] {
        let (cube, top) = bumped_cube(sense);
        for (toward, above) in [(-1.0, true), (1.0, false)] {
            let (mut prism, corner) = prism_at_the_bump(toward);
            let classes = classify(&mut prism, corner, (&cube, top), Some(UV))
                .unwrap_or_else(|e| panic!("sense {sense}, above {above}: {e:?}"));
            let want = if above == sense {
                SideCode::Out
            } else {
                SideCode::In
            };
            assert!(
                classes.len() == 3 && classes.iter().all(|&c| c == want),
                "sense {sense}, above {above}: the corner's three edges read {classes:?}, \
                 not all {want:?}"
            );
        }
    }
}

/// **A pierce of a spline face the sweep placed at no chart point
/// refuses C5's typed refusal, naming the kind**, as it did before the
/// face had an arm: the pass reads no foot of its own.
#[test]
fn a_pierce_of_a_spline_face_with_no_chart_point_refuses_the_kind() {
    let (cube, top) = bumped_cube(true);
    let (mut prism, corner) = prism_at_the_bump(-1.0);
    let got = classify(&mut prism, corner, (&cube, top), None);
    assert!(
        matches!(
            got,
            Err(BooleanError::CurvedBooleanUnsupported {
                operand: Operand::B,
                face,
                kind: geom::SurfaceKind::Nurbs,
            }) if face == top
        ),
        "{got:?}"
    );
}

/// **A pierce on a crease of the pierced chart refuses by name**
/// ([`BooleanError::NormalAtSplineCrease`]): the ridge chart's double
/// knot at `u = ½` folds the face, and a pierce placed on it has a
/// normal on each side.
#[test]
fn a_pierce_on_a_crease_of_the_pierced_chart_refuses_by_name() {
    use geom_core::spline::KnotVector;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<_> = [(0.0, 1.0), (0.25, 1.1), (0.5, 1.2), (0.75, 1.1), (1.0, 1.0)]
        .iter()
        .flat_map(|&(x, z)| [0.0, 1.0].map(|y| Point3::new(x, y, z)))
        .collect();
    let ridge = geom::NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap();
    let (mut cube, top) = bumped_cube(true);
    cube.set_face_surface_unvouched_for_tests(
        top,
        crate::construction::FaceSurface::New {
            surface: geom::Surface::Nurbs(std::sync::Arc::new(ridge.clone())),
            sense: true,
        },
    )
    .unwrap();
    let p = ridge.eval(0.5, 0.5);
    let prism = crate::test_support_fixtures::prism_z::<f64>(
        &[(p.x, p.y), (p.x - 0.2, p.y + 0.06), (p.x - 0.2, p.y - 0.06)],
        p.z,
        p.z + 0.2,
        Tol::witness(),
    );
    let (mut body, corner) = (prism.body, prism.bottom[0]);
    let got = classify(&mut body, corner, (&cube, top), Some((0.5, 0.5)));
    assert!(
        matches!(
            got,
            Err(BooleanError::NormalAtSplineCrease { operand: Operand::B, face, .. }) if face == top
        ),
        "{got:?}"
    );
}

/// The biquadratic net over the unit square (`x = u`, `y = v`) with its
/// boundary rows flat at `z = 1` and its centre control point raised to
/// `1.4`, so the cube's top edges lie on it.
fn dome() -> geom::NurbsSurface<f64> {
    use geom_core::spline::KnotVector;
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control: Vec<_> = [0.0, 0.5, 1.0]
        .iter()
        .flat_map(|&x| {
            [0.0, 0.5, 1.0].map(move |y| {
                let z = if x == 0.5 && y == 0.5 { 1.4 } else { 1.0 };
                Point3::new(x, y, z)
            })
        })
        .collect();
    geom::NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
}

/// The unit cube with its top on [`dome`], minted, and the top face.
fn domed_cube() -> (Body<f64>, FaceKey) {
    let cube = crate::test_support_fixtures::prism::<f64>(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
        Tol::witness(),
    );
    let mut body = cube.body;
    // Each top edge, re-described on the side plane it keeps.
    let outer = body.get_face(cube.top_face).unwrap().outer;
    let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
    else {
        unreachable!("the cube's top is a cycle")
    };
    let redescriptions: Vec<_> = body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| {
            let h = body.get_half_edge(he).unwrap();
            let edge = body.get_edge(h.edge).unwrap();
            let mate = if edge.he_plus == he {
                edge.he_minus
            } else {
                edge.he_plus
            };
            let side = body
                .get_loop(body.get_half_edge(mate).unwrap().parent_loop)
                .unwrap()
                .face;
            let point = |he| {
                let v = body.get_half_edge(he).unwrap().start;
                *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
            };
            let mut spec =
                geom_brep::EdgeCurveSpec::line_between(point(edge.he_plus), point(edge.he_minus));
            spec.description =
                geom_brep::EdgeDescriptionSpec::chart(body.get_face(side).unwrap().surface);
            (h.edge, spec)
        })
        .collect();
    body.lifting_rechart_refusals_for_tests(|b| {
        b.set_face_surfaces_describing(
            vec![crate::construction::Rechart::new(
                geom::Surface::Nurbs(std::sync::Arc::new(dome())),
                cube.top_face,
                true,
            )],
            &redescriptions,
            Tol::witness(),
        )
    })
    .unwrap();
    crate::pcurves::mint_pcurves(&mut body, Tol::witness()).unwrap();
    (body, cube.top_face)
}

/// Every row `face` stores, image and joint element by half-edge, in
/// loop and cycle order, as text: the byte-level record a door leaves.
fn rows_of(body: &Body<f64>, face: FaceKey) -> Vec<String> {
    let data = body.get_face(face).unwrap();
    core::iter::once(data.outer)
        .chain(data.rings.iter().copied())
        .filter_map(|lk| match body.get_loop(lk).unwrap().boundary {
            crate::entity::LoopBoundary::Cycle { first } => Some(first),
            crate::entity::LoopBoundary::Empty { .. } => None,
        })
        .flat_map(|first| body.loop_cycle(first).unwrap())
        .map(|he| format!("{he:?} {:?} {:?}", body.pcurve(he), body.joint(he)))
        .collect()
}

/// [`super::ring_vertex_in`] at `p` in `face`, checking the vertex is
/// alone in a new `Empty` ring of the face at `p`; the body it leaves.
fn ring_at(mut body: Body<f64>, face: FaceKey, p: Point3<f64>, what: &str) -> Body<f64> {
    let (vertex, ring) = super::ring_vertex_in(&mut body, face, p, Tol::witness())
        .unwrap_or_else(|e| panic!("{what}: {e:?}"));
    let data = body.get_face(face).unwrap();
    assert!(
        data.rings.contains(&ring)
            && body.get_loop(ring).unwrap().boundary
                == crate::entity::LoopBoundary::Empty { vertex },
        "{what}: {ring:?} is no empty ring of the face holding {vertex:?}"
    );
    let at = *body
        .get_point(body.get_vertex(vertex).unwrap().point)
        .unwrap();
    assert!(
        [at.x, at.y, at.z] == [p.x, p.y, p.z],
        "{what}: the ring vertex sits at {at:?}, not {p:?}"
    );
    body
}

/// [`ring_at`] on a minted face, which must leave the face's rows exactly
/// as found and complete, and the pcurve pass finding nothing on the
/// body.
fn rings_as_found(body: Body<f64>, face: FaceKey, p: Point3<f64>, what: &str) {
    let before = rows_of(&body, face);
    assert!(
        crate::pcurves::stored_rows(&body, face).complete(),
        "{what}: the fixture's face is minted"
    );
    let body = ring_at(body, face, p, what);
    assert_eq!(
        rows_of(&body, face),
        before,
        "{what}: the face's rows moved"
    );
    assert!(
        crate::pcurves::stored_rows(&body, face).complete(),
        "{what}: the face leaves half-minted"
    );
    let findings = crate::pcurves::validate_pcurves(&body, band());
    assert!(
        findings.is_empty(),
        "{what}: the pcurve pass finds {findings:?}"
    );
}

/// **A ring vertex struck into a minted NURBS wall leaves the wall
/// minted, in either sense**: its rows stand as found, complete, the
/// joint into the strut's anchor included, and the vertex is the
/// chart's point at the sweep's chart point. A strut put in by a bare
/// `mev` refuses `SplineChart` on the complete wall; one whose kill
/// leaves the anchor's joint as `kemr` sums it leaves a gap there.
#[test]
fn a_ring_vertex_in_a_nurbs_wall_leaves_it_minted_in_either_sense() {
    let (cube, top) = domed_cube();
    let p = dome().eval(UV.0, UV.1);
    let reverted = cube.revert().unwrap();
    assert!(
        !reverted.get_face(top).unwrap().sense,
        "the reverted wall is the sense-false one"
    );
    rings_as_found(cube, top, p, "sense true");
    rings_as_found(reverted, top, p, "sense false");
}

/// **The analytic arm mints the ring by a chord strut, through the
/// strut's site mint**: on a minted cylinder wall the line chord does
/// not certify against the chart, so the face leaves rowless, never
/// half-minted. The join's keys-only re-parenting doors read that face
/// as one they owe no row; one that kept its rows here refuses
/// `KeysOnly` there (`sweep`'s `a_rim_inside_a_ball_face`).
#[test]
fn a_ring_vertex_in_a_cylinder_wall_goes_through_the_struts_site_mint() {
    let mut body = crate::test_support_fixtures::fresh_keyed_body::<f64>();
    let face = crate::test_support_fixtures::cyl_wall_sheet(
        &mut body,
        crate::test_support_fixtures::CylFrame::canonical(1.0),
        (0.0, 1.0),
        (0.0, 1.0),
        Tol::witness(),
    );
    assert!(
        crate::pcurves::stored_rows(&body, face).complete(),
        "the fixture's wall is minted"
    );
    let p = Point3::new(0.5f64.cos(), 0.5f64.sin(), 0.5);
    let body = ring_at(body, face, p, "cylinder");
    let rows = crate::pcurves::stored_rows(&body, face);
    assert!(
        !rows.stores,
        "the cylinder wall keeps rows the chord's site mint clears: {:?}",
        rows_of(&body, face)
    );
}

/// **[`crate::Body::ring_vertex`] is `mev` then `kemr`**: on a planar
/// face, where neither operator owes a row, it leaves the body the two
/// operators called in turn leave, key for key, record for record and
/// row for row, returning the same vertex and ring. A door that mints its
/// vertex and ring by its own mutation reds here, as does one that runs
/// the pair in another minting order.
#[test]
fn a_ring_vertex_is_mev_then_kemr() {
    let cube = crate::test_support_fixtures::prism::<f64>(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
        Tol::witness(),
    );
    let outer = cube.body.get_face(cube.top_face).unwrap().outer;
    let crate::entity::LoopBoundary::Cycle { first: anchor } =
        cube.body.get_loop(outer).unwrap().boundary
    else {
        unreachable!("the cube's top is a cycle")
    };
    let p = Point3::new(0.4, 0.3, 1.0);
    let mut fused = cube.body.clone();
    let got = fused.ring_vertex(anchor, p, Tol::witness()).unwrap();
    let mut pair = cube.body;
    let start = pair.get_half_edge(anchor).unwrap().start;
    let p_u = *pair
        .get_point(pair.get_vertex(start).unwrap().point)
        .unwrap();
    let strut = pair
        .mev(
            crate::euler::MevSite::Fan {
                he1: anchor,
                he2: anchor,
            },
            p,
            geom_brep::EdgeCurveSpec::line_between(p_u, p),
            Tol::witness(),
        )
        .unwrap();
    let ring = pair.kemr(strut.he_plus, strut.he_minus).unwrap().ring;
    assert_eq!(
        got,
        (strut.vertex, ring),
        "the vertex and ring the door returns"
    );
    assert_eq!(
        format!("{fused:?}"),
        format!("{pair:?}"),
        "the door's body is not the pair's"
    );
}

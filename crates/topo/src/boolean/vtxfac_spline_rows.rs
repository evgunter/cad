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

// ---- review 6166 rows: the door is `mev` then `kemr` (keys, slot
// versions, provenance, next mint), and a refusal consumes no slot ----

fn all_halves(body: &Body<f64>) -> Vec<crate::HalfEdgeKey> {
    body.half_edges.iter().map(|(k, _)| k).collect()
}

fn keys_only(mut body: Body<f64>) -> String {
    let hs = all_halves(&body);
    body.drop_rows(hs);
    format!("{body:?}")
}

fn outer_anchor(body: &Body<f64>, face: FaceKey) -> crate::HalfEdgeKey {
    let outer = body.get_face(face).unwrap().outer;
    let crate::entity::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary
    else {
        unreachable!()
    };
    first
}

/// The door against `mev` then `kemr` run in turn on the same face with
/// its rows dropped (so `mev`'s site mint is a no-op): same returned
/// keys, same arenas, slot versions and provenance, and the same keys
/// minted by the next operator.
fn door_equals_mev_then_kemr(body: Body<f64>, face: FaceKey, p: Point3<f64>, what: &str) {
    let tol = Tol::witness();
    let anchor = outer_anchor(&body, face);
    let mut a = body.clone();
    let (va, la) = a.ring_vertex(anchor, p, tol).unwrap();
    let mut b = body;
    let hs = all_halves(&b);
    b.drop_rows(hs);
    let u = b.get_half_edge(anchor).unwrap().start;
    let pu = *b.get_point(b.get_vertex(u).unwrap().point).unwrap();
    let chord = b
        .mev(
            crate::construction::MevSite::Fan {
                he1: anchor,
                he2: anchor,
            },
            p,
            geom_brep::EdgeCurveSpec::line_between(pu, p),
            tol,
        )
        .unwrap_or_else(|e| panic!("{what}: mev on the rowless face: {e:?}"));
    let k = b.kemr(chord.he_plus, chord.he_minus).unwrap();
    assert_eq!((va, la), (chord.vertex, k.ring), "{what}: returned keys");
    assert_eq!(
        keys_only(a.clone()),
        keys_only(b.clone()),
        "{what}: arenas differ"
    );
    // The next mint reuses the freed slots alike.
    let q = Point3::new(p.x + 0.01, p.y, p.z);
    let next = |body: &mut Body<f64>, ring| {
        body.mev(
            crate::construction::MevSite::Lone { r#loop: ring },
            q,
            geom_brep::EdgeCurveSpec::line_between(p, q),
            tol,
        )
        .map(|m| format!("{m:?}"))
        .unwrap_or_else(|e| format!("{e:?}"))
    };
    let ha = all_halves(&a);
    a.drop_rows(ha);
    let (na, nb) = (next(&mut a, la), next(&mut b, k.ring));
    assert!(na.starts_with("MevCreated"), "{what}: {na}");
    assert_eq!(na, nb, "{what}: the next mint differs");
}

#[test]
fn review6166_ring_vertex_mints_what_mev_then_kemr_mint() {
    let (cube, top) = domed_cube();
    door_equals_mev_then_kemr(cube.clone(), top, dome().eval(UV.0, UV.1), "dome");
    let reverted = cube.revert().unwrap();
    door_equals_mev_then_kemr(reverted, top, dome().eval(UV.0, UV.1), "dome reverted");
    let flat = crate::test_support_fixtures::prism::<f64>(
        &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
        1.0,
        Tol::witness(),
    );
    door_equals_mev_then_kemr(
        flat.body,
        flat.top_face,
        Point3::new(0.3, 0.6, 1.0),
        "plane",
    );
}

#[test]
fn review6166_a_refused_ring_vertex_consumes_no_slot() {
    let (cube, top) = domed_cube();
    let anchor = outer_anchor(&cube, top);
    let u = cube.get_half_edge(anchor).unwrap().start;
    let pu = *cube.get_point(cube.get_vertex(u).unwrap().point).unwrap();
    for (what, at) in [
        ("degenerate chord", pu),
        ("nan", Point3::new(f64::NAN, 0.5, 1.0)),
    ] {
        let mut body = cube.clone();
        let got = body.ring_vertex(anchor, at, Tol::witness());
        assert!(got.is_err(), "{what}: {got:?}");
        assert_eq!(
            format!("{body:?}"),
            format!("{cube:?}"),
            "{what}: body moved"
        );
        let p = dome().eval(UV.0, UV.1);
        let mut fresh = cube.clone();
        assert_eq!(
            format!("{:?}", body.ring_vertex(anchor, p, Tol::witness())),
            format!("{:?}", fresh.ring_vertex(anchor, p, Tol::witness())),
            "{what}: the next mint differs"
        );
    }
}

#[test]
fn review6166_tiers_after_the_door_and_revert() {
    let (cube, top) = domed_cube();
    let p = dome().eval(UV.0, UV.1);
    for (what, body) in [
        ("as built", cube.clone()),
        ("reverted", cube.revert().unwrap()),
    ] {
        let mut body = body;
        let anchor = outer_anchor(&body, top);
        body.ring_vertex(anchor, p, Tol::witness()).unwrap();
        crate::validate::validate(&body).unwrap_or_else(|e| panic!("{what}: tier 1 {e:?}"));
        let back = body.revert().unwrap();
        crate::validate::validate(&back).unwrap_or_else(|e| panic!("{what} back: tier 1 {e:?}"));
        assert!(
            crate::pcurves::validate_pcurves(&back, band()).is_empty(),
            "{what} back"
        );
        assert!(
            crate::pcurves::stored_rows(&back, top).complete(),
            "{what} back"
        );
    }
}

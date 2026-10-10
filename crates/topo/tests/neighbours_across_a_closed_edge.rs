//! **Two coplanar faces meeting along one closed circle**: the merge's
//! declared-pair rung glues them, and the maximal-faces gate (F7,
//! `boolean::reduce::gate_maximal_faces`) refuses them as coplanar
//! neighbours, through the public union.
//!
//! A brick's top with a disc planted in it on a plane key of its own:
//! the two faces meet along one closed circle, whose ends are one
//! vertex, and along nothing else. Its chord is 0, so neither the merge
//! nor the gate reads it: the merge levers its orientation rung at the
//! pair's reach and the gate at the circle's extent, its diameter, and
//! each decides which way the two faces face. A disc whose diameter lies within the band still leaves
//! the orientation undecided, with the gate's lever; such a disc does
//! not finish, so that row asks the gate itself. (An open edge that
//! short does not certify, so no public door builds one; its row is
//! the gate's own, `boolean::reduce`'s tests.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Point3, Tol};
use topo::{
    Body, BooleanDecision, BooleanError, LoopBoundary, MergeCoplanarError, Operand, PlaneRung,
};

/// The brick `[0, 4s]² × [0, s]`, its top face and the first half-edge
/// of that face's outer loop.
fn brick(s: f64) -> (Body<f64>, topo::FaceKey, topo::HalfEdgeKey) {
    let p = common::prism_z::<f64>(
        &[
            (0.0, 0.0),
            (4.0 * s, 0.0),
            (4.0 * s, 4.0 * s),
            (0.0, 4.0 * s),
        ],
        0.0,
        s,
        Tol::witness(),
    );
    let outer = p.body.get_face(p.top_face).unwrap().outer;
    let LoopBoundary::Cycle { first } = p.body.get_loop(outer).unwrap().boundary else {
        panic!("the top face's outer loop is a cycle");
    };
    (p.body, p.top_face, first)
}

/// The brick at scale `s` with a disc of `radius` planted at the centre
/// of its top.
fn disc_in_top(s: f64, radius: f64) -> (Body<f64>, topo::FaceKey, topo::FaceKey) {
    let (mut body, top, at) = brick(s);
    let disc = common::plant_disc_face(
        &mut body,
        at,
        Point3::new(2.0 * s, 2.0 * s, s),
        radius,
        Tol::witness(),
    );
    topo::validate_closed(&body).expect("a planted disc leaves a closed solid");
    (body, top, disc.face)
}

/// **(a)**: the top and the disc, declared one plane, glue into one
/// face: the orientation rung reads them over the pair's reach, not the
/// circle's chord, and the circle's vertex, left a lone vertex of the
/// top once the circle is killed, is deleted with its ring.
#[test]
fn a_disc_declared_on_its_hosts_plane_merges_across_the_circle() {
    let (mut body, top, disc) = disc_in_top(1.0, 1.0);
    let keys = (
        body.get_face(top).unwrap().surface,
        body.get_face(disc).unwrap().surface,
    );
    let disc_outline = body.get_face(disc).unwrap().outer;
    let LoopBoundary::Cycle { first } = body.get_loop(disc_outline).unwrap().boundary else {
        panic!("the disc's outline is the circle");
    };
    let circle_vertex = body.get_half_edge(first).unwrap().start;
    let outcome = body
        .merge_coplanar_faces_declared(&[keys], Tol::witness())
        .unwrap_or_else(|err| panic!("the declared pair glues: {err:?}\n{err}"));
    let [group] = &outcome.groups[..] else {
        panic!("one group: {outcome:?}");
    };
    assert!(outcome.skipped.is_empty(), "nothing skipped: {outcome:?}");
    assert_eq!(
        (group.kept, &group.absorbed[..]),
        (top, &[disc][..]),
        "the top absorbs the disc"
    );
    assert_eq!(
        group.killed_vertices,
        [circle_vertex],
        "the circle's vertex goes with it, and only it: {group:?}"
    );
    assert_eq!(body.faces().count(), 6, "the brick has six faces again");
    assert!(
        body.get_face(top).unwrap().rings.is_empty(),
        "the top holds no ring"
    );
    topo::validate_closed(&body).expect("the merged brick is a closed solid");
}

/// `a` with its circle at rest in the top's chart, and the circle. The
/// plant leaves the circle a scaffold, which the at-rest gate refuses
/// (asserted here); the circle lies in the top's plane exactly, so it
/// rests in the top's chart, traversed the way the top's outer
/// boundary runs it (axis `−z`). The delta review's
/// `d_coplanar_neighbours_with_an_at_rest_edge_against_the_finished_gate`.
fn circle_at_rest(
    a: &Body<f64>,
    top: topo::FaceKey,
    s: f64,
    radius: f64,
) -> (Body<f64>, topo::EdgeKey) {
    let tol = Tol::witness();
    let circle: Vec<_> = a
        .edges()
        .filter(|(_, e)| {
            a.get_curve_geom(e.curve)
                .and_then(topo::null::CurveGeom::certified)
                .is_some_and(|c| c.description().is_scaffold())
        })
        .map(|(k, _)| k)
        .collect();
    let [edge] = circle[..] else {
        panic!("the plant leaves one scaffold, the circle: {circle:?}");
    };
    let errors =
        topo::AtRestBody::validate(a.clone(), tol).expect_err("the planted circle is a scaffold");
    assert!(
        errors.contains(&topo::ValidationError::ScaffoldAtRest { edge }),
        "the at-rest gate names the circle's scaffold: {errors:?}"
    );
    let mut rested = a.clone();
    let chart = rested.get_face(top).unwrap().surface;
    let carrier = geom::Curve3::Circle {
        center: Point3::new(2.0 * s, 2.0 * s, s),
        axis: geom_core::Vec3::new(0.0, 0.0, -1.0),
        radius,
        u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
    };
    rested
        .set_edge_curve(
            edge,
            geom_brep::EdgeCurveSpec::arc_of_circle(carrier, 0.0, core::f64::consts::TAU)
                .unwrap()
                .at_rest_in_chart(chart, false),
            tol,
        )
        .expect("the circle rests in the top's chart");
    (rested, edge)
}

/// **(b)**: the top and a disc on its plane meet along a circle whose
/// chord is 0, at scales 1e-3, 1 and 1e3 (the disc's radius the
/// brick's height), the body finished and united with a far brick
/// through the public door. Levered at the circle's extent, the gate
/// decides they face the same way and lie on one plane, and refuses
/// the operand as not maximal across the circle (a pair the merge
/// glues); levered at the chord, it would leave the orientation
/// undecided at margin 0.
#[test]
fn a_disc_on_its_hosts_plane_refuses_as_not_maximal() {
    for s in [1e-3, 1.0, 1e3] {
        let (body, top, _) = disc_in_top(s, s);
        let (rested, circle) = circle_at_rest(&body, top, s, s);
        let tol = Tol::witness();
        let far = common::brick::<f64>((10.0 * s, 11.0 * s), (0.0, s), (0.0, s), tol);
        let err = topo::union(
            &common::finished("the disc in a top", rested, tol),
            &common::finished("the far brick", far, tol),
            tol,
        )
        .expect_err("the gate refuses the operand");
        let BooleanError::NonMaximalFaces { operand, edge } = err else {
            panic!("at scale {s}, the gate decides the pair coplanar: {err:?}\n{err}");
        };
        assert_eq!(
            operand,
            Operand::A,
            "at scale {s}, the disc is the first operand's"
        );
        assert_eq!(edge, circle, "at scale {s}, the refusal names the circle");
    }
}

/// **(c)**: a circle that spans less than the band still leaves the
/// orientation undecided: a disc whose diameter lies in the middle of
/// the run's ambiguity band. The refusal is
/// the gate's orientation rung, with its lever and no tolerance. No
/// public door reaches it: the at-rest gate's own dihedral read on a
/// circle that short is undecided too, so the body does not finish
/// (asserted), and the row asks the gate itself
/// (`test_support::maximal_faces_gate`).
#[test]
fn an_edge_spanning_less_than_the_band_still_refuses_undecided() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let diameter = (band.zero() + band.escalate()) / 2.0;
    let (body, top, _) = disc_in_top(1.0, diameter / 2.0);
    let (rested, circle) = circle_at_rest(&body, top, 1.0, diameter / 2.0);
    let errors = topo::AtRestBody::validate(rested.clone(), Tol::witness())
        .expect_err("a circle that short does not finish");
    assert!(
        matches!(
            errors[..],
            [topo::ValidationError::SliverDihedral { edge, .. }] if edge == circle
        ),
        "the at-rest gate's own dihedral read on the circle is undecided: {errors:?}"
    );
    let err = common::maximal_faces_gate(&rested, Operand::A, Tol::witness())
        .expect_err("the gate refuses the pair");
    let BooleanError::Escalated {
        decision: BooleanDecision::Neighbours(PlaneRung::Orientation),
        ..
    } = err
    else {
        panic!("the gate's orientation rung is undecided: {err:?}\n{err}");
    };
    let text = err.to_string();
    assert!(
        text.contains("along an edge spanning clearly more than the tolerance")
            && !text.contains("tighten"),
        "the rung ends in the gate's lever alone: {text}"
    );
}

/// A unit sphere cut at its equator: one closed circle, the whole
/// outline of each hemisphere, both on one surface key.
fn hemispheres() -> Body<f64> {
    let tol = Tol::witness();
    let x = geom_core::Vec3::new(1.0, 0.0, 0.0);
    let z = geom_core::Vec3::new(0.0, 0.0, 1.0);
    let origin = Point3::new(0.0, 0.0, 0.0);
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(origin + x, true).unwrap();
    body.set_face_surface(
        seed.face,
        topo::FaceSurface::New {
            surface: geom::Surface::Sphere {
                center: origin,
                radius: 1.0,
                axis: z,
                u_ref: x,
            },
            sense: true,
        },
    )
    .unwrap();
    let equator = geom::Curve3::Circle {
        center: origin,
        axis: z,
        radius: 1.0,
        u_ref: x,
    };
    body.mef(
        topo::MefSite::Lone {
            r#loop: seed.r#loop,
        },
        geom_brep::EdgeCurveSpec::arc_of_circle(equator, 0.0, core::f64::consts::TAU).unwrap(),
        topo::FaceSurface::Inherit,
        tol,
    )
    .unwrap();
    topo::validate_closed(&body).expect("the cut sphere is a closed solid");
    body
}

/// Two faces sharing one closed outline: killing the circle would leave
/// the survivor's outline a lone vertex. The merge refuses that before
/// the `kef`, as it refuses a pruned ring that is the outline; the
/// curved group records it as a skip and the body is left as it was.
#[test]
fn two_faces_sharing_one_closed_outline_refuse_before_the_kef() {
    let mut body = hemispheres();
    let before: Vec<_> = body.faces().map(|(f, _)| f).collect();
    let equator = body.edges().map(|(e, _)| e).collect::<Vec<_>>();
    let outcome = body
        .merge_coplanar_faces(Tol::witness())
        .unwrap_or_else(|err| panic!("the run records a skip: {err:?}\n{err}"));
    assert!(outcome.groups.is_empty(), "nothing merges: {outcome:?}");
    let [skip] = &outcome.skipped[..] else {
        panic!("one skip: {outcome:?}");
    };
    let mut faces = skip.faces.clone();
    faces.sort();
    assert_eq!(faces, before, "the skip names both hemispheres");
    assert_eq!(
        skip.reason,
        MergeCoplanarError::UnsupportedConfiguration { edge: equator[0] },
        "refused on the equator, before the kef"
    );
    assert_eq!(
        body.faces().map(|(f, _)| f).collect::<Vec<_>>(),
        before,
        "the body keeps both hemispheres"
    );
    topo::validate_closed(&body).expect("the skipped body is still closed");
}

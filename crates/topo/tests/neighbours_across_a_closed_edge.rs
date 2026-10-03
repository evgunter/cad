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
//! the orientation undecided, with the gate's lever. (An open edge that
//! short does not certify, so no public door builds one; its row is
//! the gate's own, `boolean::reduce`'s tests.)

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Point3, Tol};
use topo::{
    Body, BooleanDecision, BooleanError, LoopBoundary, NeighbourOffset, Operand, PlaneRung,
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
        group.killed_vertices.len(),
        1,
        "the circle's vertex goes with it: {group:?}"
    );
    assert_eq!(body.faces().count(), 6, "the brick has six faces again");
    assert!(
        body.get_face(top).unwrap().rings.is_empty(),
        "the top holds no ring"
    );
    topo::validate_closed(&body).expect("the merged brick is a closed solid");
}

/// `a`, built at scale `s`, united with a brick far from it, so the
/// refusal is `a`'s gate's.
fn union_far(a: &Body<f64>, s: f64) -> BooleanError {
    let far = common::brick::<f64>((10.0 * s, 11.0 * s), (0.0, s), (0.0, s), Tol::witness());
    topo::union(a, &far, Tol::witness()).expect_err("the gate refuses the operand")
}

/// **(b)**: the top and a disc on its plane meet along a circle whose
/// chord is 0, at scales 1e-3, 1 and 1e3 (the disc's radius the
/// brick's height). Levered at the circle's extent, the gate
/// decides they face the same way and lie on one plane, and refuses
/// them as coplanar neighbours with a decided zero offset; levered at
/// the chord, it would leave the orientation undecided at margin 0.
#[test]
fn a_disc_on_its_hosts_plane_refuses_as_coplanar_neighbours() {
    for s in [1e-3, 1.0, 1e3] {
        let (body, top, disc) = disc_in_top(s, s);
        let err = union_far(&body, s);
        let BooleanError::CoplanarNeighbours {
            operand,
            mut faces,
            offset: NeighbourOffset::Zero(_),
        } = err
        else {
            panic!("at scale {s}, the gate decides the pair coplanar: {err:?}\n{err}");
        };
        assert_eq!(
            operand,
            Operand::A,
            "at scale {s}, the disc is the first operand's"
        );
        faces.sort();
        let mut want = [top, disc];
        want.sort();
        assert_eq!(
            faces, want,
            "at scale {s}, the refusal names the top and the disc"
        );
    }
}

/// **(c)**: a circle that spans less than the band still leaves the
/// orientation undecided: a disc whose diameter lies in the middle of
/// the run's ambiguity band. The refusal is
/// the gate's orientation rung, with its lever and no tolerance.
#[test]
fn an_edge_spanning_less_than_the_band_still_refuses_undecided() {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    let diameter = (band.zero() + band.escalate()) / 2.0;
    let (body, _, _) = disc_in_top(1.0, diameter / 2.0);
    let err = union_far(&body, 1.0);
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

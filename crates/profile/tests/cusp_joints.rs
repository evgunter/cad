//! **A validated loop keeps which declared joints are cusps.** The
//! `.cusp()` door emits the same declaration `.tangent()` does, so the
//! kind is decided at validation — once per declared joint, with the
//! path door's own `path_junction_side` question — and recorded as
//! `ValidatedLoop::cusp_joints`: canonical, carried through the
//! reversal remap, the lift and the guided replay, and read by
//! `blend_arcs`, which must not list an arc a cusp bounds.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{
    ClosedLoop, ConstructedProfile, Decision, Open, Profile, ProfileError, ProfileLoop, RawLoop,
    SketchPlane, Start, ValidatedProfile, replay_guided, replay_recording,
};

fn validated(loops: Vec<ProfileLoop<f64>>) -> ValidatedProfile<f64> {
    Profile::new(SketchPlane::xy(), loops)
        .validate(Tol::witness())
        .expect("validates")
}

/// The lune through the `.cusp()` door: the kiss is canonical joint 2.
fn lune() -> ProfileLoop<f64> {
    lune_closed().into()
}

/// [`lune`] with the program that built it.
fn lune_closed() -> ClosedLoop<f64> {
    let tol = Tol::witness();
    Open.at(Point2::new(0.0, 4.0))
        .angle(-std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .line(2.0, tol)
        .unwrap()
        .turn(std::f64::consts::FRAC_PI_2, tol)
        .unwrap()
        .tangent_arc_to(Point2::new(0.0, 0.0), tol)
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol)
        .unwrap()
}

/// The same lune authored RAW and wound the other way (clockwise), so
/// canonicalization reverses it: the kiss at (0, 0) is input joint 2.
fn raw_lune() -> ProfileLoop<f64> {
    bulge_loop(vec![
        (Point2::new(0.0, 4.0), 0.0),
        (Point2::new(0.0, 2.0), -1.0),
        (Point2::new(0.0, 0.0), 1.0),
    ])
    .with_tangent_joints(vec![2])
}

/// The canonical index of the vertex at `at`.
fn index_of(v: &ValidatedProfile<f64>, li: usize, at: Point2<f64>) -> usize {
    v.loops()[li]
        .vertices()
        .iter()
        .position(|q| q.x == at.x && q.y == at.y)
        .expect("the vertex survives canonicalization")
}

#[test]
fn the_cusp_door_records_its_joint_and_the_tangent_door_does_not() {
    let v = validated(vec![lune()]);
    assert_eq!(v.loops()[0].tangent_joints(), &[2]);
    assert_eq!(v.loops()[0].cusp_joints(), &[2]);

    // A rounded rectangle: eight declared joints, every one smooth.
    let q = 0.25;
    let b = core::f64::consts::FRAC_PI_8.tan();
    let rounded = bulge_loop(vec![
        (Point2::new(q, 0.0), 0.0),
        (Point2::new(1.0 - q, 0.0), b),
        (Point2::new(1.0, q), 0.0),
        (Point2::new(1.0, 1.0 - q), b),
        (Point2::new(1.0 - q, 1.0), 0.0),
        (Point2::new(q, 1.0), b),
        (Point2::new(0.0, 1.0 - q), 0.0),
        (Point2::new(0.0, q), b),
    ])
    .with_tangent_joints((0..8).collect());
    let v = validated(vec![rounded]);
    assert_eq!(v.loops()[0].tangent_joints().len(), 8);
    assert_eq!(v.loops()[0].cusp_joints(), &[] as &[usize]);
    assert_eq!(v.loops()[0].blend_arcs().len(), 4);
}

/// The kind travels with its vertex through the canonical reversal —
/// as an outer loop, and as a hole (whose canonical winding is the
/// other one).
#[test]
fn a_reversed_loop_keeps_its_cusp_at_the_same_vertex() {
    let kiss = Point2::new(0.0, 0.0);
    let v = validated(vec![raw_lune()]);
    assert_eq!(v.loops()[0].cusp_joints(), &[index_of(&v, 0, kiss)]);
    let plate = bulge_loop(vec![
        (Point2::new(-1.0, -1.0), 0.0),
        (Point2::new(3.0, -1.0), 0.0),
        (Point2::new(3.0, 5.0), 0.0),
        (Point2::new(-1.0, 5.0), 0.0),
    ]);
    for hole in [lune(), raw_lune()] {
        let v = validated(vec![plate.clone(), hole]);
        assert_eq!(v.loops()[0].cusp_joints(), &[] as &[usize]);
        assert_eq!(v.loops()[1].cusp_joints(), &[index_of(&v, 1, kiss)]);
    }
}

/// An arbelos: a big semicircle over two small ones, three cusps and
/// no smooth joint. Every arc is declared tangent at both ends, and
/// none is a fillet.
#[test]
fn an_arbelos_has_three_cusps_and_no_blend_arc() {
    let arbelos = bulge_loop(vec![
        (Point2::new(0.0, 0.0), -1.0),
        (Point2::new(1.0, 0.0), -1.0),
        (Point2::new(2.0, 0.0), 1.0),
    ])
    .with_tangent_joints(vec![0, 1, 2]);
    let v = validated(vec![arbelos]);
    assert_eq!(v.loops()[0].tangent_joints(), &[0, 1, 2]);
    assert_eq!(v.loops()[0].cusp_joints(), &[0, 1, 2]);
    assert!(
        v.loops()[0].blend_arcs().is_empty(),
        "an arc between cusps is not a blend: {:?}",
        v.loops()[0].blend_arcs()
    );
}

#[test]
fn the_lift_carries_the_cusps() {
    let v = validated(vec![lune()]);
    let lifted: ValidatedProfile<f64> = v.clone().lift_onto(SketchPlane::xy());
    assert_eq!(lifted.loops()[0].cusp_joints(), v.loops()[0].cusp_joints());
}

/// The heading is re-decided under guidance, so the guided pass
/// reproduces the record and refuses one that disagrees.
#[test]
fn the_guided_pass_compares_the_cusps_against_the_record() {
    // The guided door takes a loop the guided replay built, so the lune
    // goes through its own program: recorded at f64 (pass 1), then
    // replayed guided by that record.
    let program = lune_closed().program;
    let (pass1, record) = replay_recording(&program, Tol::witness()).unwrap();
    let guided_loop = replay_guided(&program, &record, Tol::witness()).unwrap();
    let p = ConstructedProfile::new(SketchPlane::xy(), vec![pass1]);
    let replayed = ConstructedProfile::new(SketchPlane::xy(), vec![guided_loop]);
    let (recorded, mut canonical) = p.validate_recording(Tol::witness()).unwrap();
    assert_eq!(canonical.loops[0].cusp_joints, vec![2]);
    let guided = replayed
        .validate_guided(Tol::witness(), &canonical)
        .unwrap();
    assert_eq!(
        guided.loops()[0].cusp_joints(),
        recorded.loops()[0].cusp_joints()
    );
    canonical.loops[0].cusp_joints.clear();
    match replayed.validate_guided(Tol::witness(), &canonical) {
        Err(ProfileError::Structure(r)) => {
            assert_eq!(r.decision, Decision::CuspJoints { loop_: 0 });
        }
        other => panic!("a record without the cusp must refuse: {other:?}"),
    }
}

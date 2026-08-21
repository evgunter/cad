//! The kernel read-back doors exercised from a consumer's seat, on
//! bodies built through the public sweep ops.
//!
//! The doors themselves live in `topo::readback` — a `Body` plus a key
//! in, values out — so their own doctests can only reach bodies `topo`
//! can build from its Euler primitives. These are the same questions
//! asked of real swept bodies: nothing here transcribes a literal to
//! learn where a cap landed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tolerance, Vec2, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{
    Extrusion, Revolution, RevolveAxis, Section, WedgeCapsError, extrude, loft_body, revolve,
    revolved_caps,
};
use topo::readback::{edge_pose, face_pose, vertex_point};
use geom_core::Tol;

fn unit_square() -> Profile<f64> {
    let square = ProfileLoop::polygon(
        [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
            .iter()
            .map(|&(x, y)| Point2::new(x, y)),
    );
    Profile::new(SketchPlane::xy(), vec![square])
}

#[test]
fn face_pose_reads_an_extruded_cap_plane() {
    let profile = unit_square()
        .validate(Tol::witness())
        .expect("the unit square validates");
    let block = extrude::<f64>(&profile, Extrusion::Distance(1.0)).expect("it extrudes");

    // The top cap sits on the sketch plane translated by the
    // extrusion — z = 1, normal along +z. No literal was transcribed
    // to learn that.
    let top = face_pose(&block.body, block.top).expect("a planar cap");
    assert_eq!(top.origin.z, 1.0);
    assert_eq!(top.axis.z, 1.0);
    // A plane fixes its in-plane reference direction; the triad is
    // right-handed.
    assert!(top.u_ref.is_some());
    assert_eq!(top.v_ref().expect("a complete triad").y, 1.0);
}

#[test]
fn vertex_point_reads_every_corner_of_a_block() {
    let profile = unit_square()
        .validate(Tol::witness())
        .expect("the unit square validates");
    let block = extrude::<f64>(&profile, Extrusion::Distance(1.0)).expect("it extrudes");

    let mut zs: Vec<f64> = block
        .body
        .vertices()
        .map(|(k, _)| vertex_point(&block.body, k).expect("a live vertex").z)
        .collect();
    zs.sort_by(f64::total_cmp);
    assert_eq!(zs, vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]);
}

#[test]
fn edge_pose_reads_a_line_without_inventing_a_perpendicular() {
    let profile = unit_square()
        .validate(Tol::witness())
        .expect("the unit square validates");
    let block = extrude::<f64>(&profile, Extrusion::Distance(1.0)).expect("it extrudes");

    // Every edge of a block is straight: a direction, and honestly no
    // reference perpendicular.
    for (k, _) in block.body.edges() {
        let pose = edge_pose(&block.body, k).expect("a certified carrier");
        assert!(pose.u_ref.is_none());
        assert!(pose.v_ref().is_none());
    }
}

#[test]
fn both_extrusion_caps_read_off_the_result_s_own_handles() {
    let profile = unit_square()
        .validate(Tol::witness())
        .expect("the unit square validates");
    let block = extrude::<f64>(&profile, Extrusion::Distance(2.0)).expect("it extrudes");

    let bottom = face_pose(&block.body, block.bottom).expect("a planar cap");
    let top = face_pose(&block.body, block.top).expect("a planar cap");
    assert_eq!(bottom.origin.z, 0.0);
    assert_eq!(top.origin.z, 2.0);
    // Each cap's chart normal points out of its own end: the door
    // reports the CHART's direction, not the face's outward sense
    // (which is a separate question it deliberately does not answer).
    assert_eq!(bottom.axis.z, -1.0);
    assert_eq!(top.axis.z, 1.0);
}

#[test]
fn both_loft_caps_read_off_the_result_s_own_handles() {
    let quad = |pts: [(f64, f64); 4]| -> Section {
        vec![ProfileLoop::polygon(
            pts.iter().map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(square), quad(square)];
    let places: Vec<Affine3<f64>> = [0.0, 1.0, 2.5]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    let loft = loft_body::<f64>(&sections, &places, 2).expect("it lofts");

    let bottom = face_pose(&loft.body, loft.bottom).expect("a planar cap");
    let top = face_pose(&loft.body, loft.top).expect("a planar cap");
    assert_eq!(bottom.origin.z, 0.0);
    assert_eq!(top.origin.z, 2.5);
    // And the v-parameters those sections landed at, from the same
    // result — no hand derivation.
    assert_eq!(loft.section_params.first(), Some(&0.0));
    assert_eq!(loft.section_params.last(), Some(&1.0));
}

#[test]
fn a_full_revolve_has_no_caps_to_read() {
    let circle = profile::circle(Point2::new(5.0, 0.0), 0.5).expect("a positive radius");
    let sketch = Profile::new(SketchPlane::xy(), vec![circle.into()])
        .validate(Tol::witness())
        .expect("the circle validates");
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let torus = revolve::<f64>(&sketch, axis, Revolution::Full).expect("the torus revolves");

    assert!(
        matches!(revolved_caps(&torus), Err(WedgeCapsError::NoCaps)),
        "a full revolve closes on itself"
    );
}

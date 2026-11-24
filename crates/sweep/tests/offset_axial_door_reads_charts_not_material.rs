//! **The axial door moves charts, not material** — the body-of-revolution
//! row of `crates/topo/tests/offset_doors_read_charts_not_material.rs`.
//! `offset_charts_together` takes construction state and reads each
//! move along its chart's stored normal; no face's sense decides it.
//!
//! The revolve door winds its result from the profile it is given, so a
//! clockwise profile builds the same right-way drum as the
//! counterclockwise one, and the two answer alike. The inside-out drum
//! is the drum reverted: its wall wears the same cylinder with its
//! sense flipped, so the same move grows the same region and the signed
//! volume changes by the negation. The inside-out operand and its
//! result alike refuse `NegativeVolume` at `AtRestBody::validate`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::charts::{charts, moves_by};
use geom::Surface;
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, ValidationError};

const R: f64 = 3.0 / 64.0;
const H: f64 = 8.0 / 64.0;
const D: f64 = 1.0 / 64.0;

/// The drum `r = R`, `h = H` revolved from its meridian rectangle,
/// counterclockwise when `ccw`.
fn drum(ccw: bool) -> Body<f64> {
    let mut meridian = vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(R, 0.0), 0.0),
        (Point2::new(R, H), 0.0),
        (Point2::new(0.0, H), 0.0),
    ];
    if !ccw {
        meridian.reverse();
    }
    revolved_about_y(meridian, Revolution::Full, Tol::witness())
}

/// `body`'s signed volume: through the typed door for the right-way
/// drum, and through the raw door for the reverted one, which tier 3
/// refuses ([`finishes_iff`] pins which).
fn volume(body: &Body<f64>, right_way: bool) -> f64 {
    if right_way {
        topo::mass_properties(
            &topo::finish(topo::Built::of(body.clone()), Tol::witness())
                .expect("the drum is finished"),
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
    } else {
        // Raw door: the reverted drum, which tier 3 refuses (NegativeVolume, as `finishes_iff` pins); its signed volume is the subject.
        topo::test_support::measure_unvalidated(
            body,
            Tol::witness(),
            topo::test_support::pinned_resolution(Tol::witness()),
        )
    }
    .unwrap()
    .enclosure
    .volume
}

/// `body` finishes when `right_way`, and refuses on check 7 alone when
/// not.
fn finishes_iff(what: &str, body: &Body<f64>, right_way: bool) {
    match topo::finish(topo::Built::of(body.clone()), Tol::witness()) {
        Ok(_) => assert!(right_way, "{what}: the inside-out body finished"),
        Err(errors) => assert!(
            !right_way
                && errors.len() == 1
                && matches!(errors[0], ValidationError::NegativeVolume { .. }),
            "{what}: want NegativeVolume alone on the inside-out body only, got {errors:?}"
        ),
    }
}

/// The cap charts' stored normals, which face the same way: one cap
/// wears its plane reversed, so a move along the stored normal shifts
/// the drum's height rather than changing it.
fn caps_face_alike(what: &str, body: &Body<f64>) {
    let normals: Vec<f64> = body
        .faces()
        .filter_map(|(fk, _)| match body.face_surface(fk) {
            Some(Surface::Plane { frame, .. }) => {
                let normal = frame.axis();
                Some(normal.y)
            }
            _ => None,
        })
        .collect();
    assert!(
        normals.len() == 2 && normals[0] == normals[1] && normals[0].abs() == 1.0,
        "{what}: the caps' stored normals are one axial direction, got {normals:?}"
    );
}

#[test]
fn offset_charts_together_moves_the_charts_in_either_winding() {
    let grown = core::f64::consts::PI * ((R + D).powi(2) - R * R) * H;
    let rows = [
        ("the counterclockwise drum", drum(true), true, grown),
        ("the clockwise drum", drum(false), true, grown),
        (
            "the reverted drum",
            drum(true).revert().unwrap(),
            false,
            -grown,
        ),
    ];
    for (what, mut body, right_way, want) in rows {
        caps_face_alike(what, &body);
        let before = volume(&body, right_way);
        let want_before = core::f64::consts::PI * R * R * H;
        assert!(
            (before - if right_way { want_before } else { -want_before }).abs() < 1e-15,
            "{what}: signed volume {before}"
        );
        finishes_iff(&format!("{what}, the operand"), &body, right_way);
        let moves = moves_by(charts(&body), D);
        topo::offset_charts_together(
            &mut body,
            &moves,
            Band::linear(Tol::witness()).unwrap(),
            Tol::witness(),
        )
        .expect("every chart offsets");
        let delta = volume(&body, right_way) - before;
        assert!(
            (delta - want).abs() < 1e-15,
            "{what}: signed ΔV {delta}, want {want}"
        );
        finishes_iff(&format!("{what}, the result"), &body, right_way);
    }
}

/// **A cap tilted into the gap between its scope's two reaches
/// escalates at the axial gate.** A unit drum whose wall bulges out to
/// radius 2 at mid-height (a torus band between the two rims), its
/// bottom cap's plane restated through the cap's centre and tilted by
/// `sin θ = 0.6·ε`. From the centre the scope's vertices, on the rims,
/// reach `√2`, and its faces' covers reach about 2.09 (the bulge): levered
/// at the first the tilt reads Zero and the drum would hollow through the
/// axial door, at the second it is in the band. Read at both, the gate
/// escalates. The cap's own rim stands at most `θ` off the tilted plane,
/// inside the tolerance, so the operand finishes.
#[test]
fn a_cap_tilted_between_its_scopes_two_reaches_escalates_at_the_gate() {
    let tol = Tol::witness();
    let theta = 0.6 * Band::linear(tol).unwrap().zero();
    let drum = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 2.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol,
    );
    let (key, cap) = drum
        .surfaces()
        .find_map(|(k, s)| match s {
            Surface::Plane { frame } if frame.origin().y == 0.0 => Some((k, *frame)),
            _ => None,
        })
        .expect("the drum has a bottom cap");
    let n = cap.axis();
    let tilted = n * (1.0 - theta * theta).sqrt() + Vec3::unit_x().cross(n) * theta;
    let body = drum
        .with_surface_moved_for_tests(
            key,
            Surface::Plane {
                frame: geom::Frame::new(Point3::origin(), tilted, cap.u_ref()),
            },
        )
        .expect("the cap's key is live");
    let finished =
        topo::finish(topo::Built::of(body), tol).expect("the tilt is within the tolerance");
    match topo::shell(&finished, 0.01, tol) {
        Err(topo::ShellError::Face { error, .. })
            if matches!(*error, topo::ReplaceFaceError::Escalated { .. }) =>
        {
            let topo::ReplaceFaceError::Escalated { source } = *error else {
                unreachable!("matched above")
            };
            assert_eq!(
                source.predicate,
                Some("offset_axial_alignment"),
                "the cap's alignment is in the band at the scope's reach from above"
            );
        }
        other => panic!(
            "the cap's alignment escalates at the gate, got {:?}",
            other.map(|_| "a hollow")
        ),
    }
}

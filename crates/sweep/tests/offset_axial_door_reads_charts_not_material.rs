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
use geom_core::{Band, Point2, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{AtRestBody, Body, ValidationError, mass_properties};

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

fn volume(body: &Body<f64>) -> f64 {
    mass_properties(body, Tol::witness()).unwrap().volume
}

/// `body` finishes when `right_way`, and refuses on check 7 alone when
/// not.
fn finishes_iff(what: &str, body: &Body<f64>, right_way: bool) {
    match AtRestBody::validate(body.clone(), Tol::witness()) {
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
        .filter_map(|(_, f)| match body.get_surface(f.surface) {
            Some(Surface::Plane { normal, .. }) => Some(normal.y),
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
        ("the reverted drum", drum(true).revert(), false, -grown),
    ];
    for (what, mut body, right_way, want) in rows {
        caps_face_alike(what, &body);
        let before = volume(&body);
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
        let delta = volume(&body) - before;
        assert!(
            (delta - want).abs() < 1e-15,
            "{what}: signed ΔV {delta}, want {want}"
        );
        finishes_iff(&format!("{what}, the result"), &body, right_way);
    }
}

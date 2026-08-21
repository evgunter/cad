//! M2 PR 6 adversarial review — walk state machine and tie rules
//! (assignments 4 and 5): wedge angles near π, axis-touching wedges,
//! pole junctions on partial revolves, downward (mirror-nappe) cone
//! walls, pole-to-pole band area-vector disambiguation, and the
//! outward-shell assumption through the public API.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{axis_y, check_mesh_acceptance, p2, validated};
use mesh::validate::signed_volume;
use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
use sweep::{Revolution, revolve};
use topo::Body;
use geom_core::Tol;

const PI: f64 = core::f64::consts::PI;

fn rev(lp: ProfileLoop<f64>, r: Revolution<f64>) -> Body<f64> {
    revolve(&validated(vec![lp]), axis_y(), r, Tol::witness()).unwrap().body
}

fn washer_profile() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(2.0, 1.0), p2(1.0, 1.0)])
}

fn cone_profile() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(0.0, 1.0)])
}

fn half_disc() -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), 1.0),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ])
}

/// Downward-opening cone attached to a cylinder (mirror nappe under
/// partial revolve): triangle (1,0)-(2,1)-(1,2) again, but revolved
/// by an angle, so the nappe walls carry junction u/v assignments.
fn diamond_profile() -> ProfileLoop<f64> {
    ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 1.0), p2(1.0, 2.0)])
}

#[test]
fn survives_wedge_angles_near_pi_and_extremes() {
    // θ near π is the half-period tie neighborhood; tiny and near-2π
    // wedges stress the unwrap and the loop closure.
    for theta in [0.05, PI / 2.0, PI - 0.01, PI, PI + 0.5, 2.0 * PI - 0.05] {
        let body = rev(washer_profile(), Revolution::Partial(theta));
        let v_exact = theta / 2.0 * 3.0; // (R2²−R1²)/2·h·θ = 3θ/2
        let a_exact = 2.0 * (1.5 * theta) + 2.0 * theta + theta + 2.0 * 2.0;
        for delta in [0.3, 0.04] {
            let mesh = check_mesh_acceptance(&body, delta, Some((v_exact, a_exact)));
            assert!(signed_volume(&mesh) > 0.0);
        }
    }
}

#[test]
fn survives_axis_touching_wedge_angles() {
    // Unit square touching the axis, revolved: the axis edge is an
    // ordinary boundary edge; cylinder wall + 2 wedge caps + 2 discs.
    for theta in [0.05, PI - 0.01, PI, PI + 0.5] {
        let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
        let body = rev(lp, Revolution::Partial(theta));
        let v_exact = theta / 2.0;
        // A = cylinder wall θ·1 + two square caps + two discs θ/2.
        let a_exact = theta + 2.0 + theta;
        for delta in [0.3, 0.04] {
            check_mesh_acceptance(&body, delta, Some((v_exact, a_exact)));
        }
    }
}

#[test]
fn survives_cone_wedges_apex_junctions_below_three_half_pi() {
    // Apex-touching wedges: the apex junction's u assignment under
    // partial angles either side of π (the unwrap_tie band edges) and
    // up to the edge of the working window (θ ≤ 3π/2).
    for theta in [0.3, PI - 0.01, PI + 0.01, 1.5 * PI - 0.01] {
        let body = rev(cone_profile(), Revolution::Partial(theta));
        for delta in [0.25, 0.04] {
            check_mesh_acceptance(&body, delta, None);
        }
    }
}

/// Cylinder-plus-hemispherical-dome profile (r = 1, wall h = 1): the
/// dome-cap face's loop carries a rim AND a pole junction.
fn silo_profile() -> ProfileLoop<f64> {
    let b = (PI / 8.0).tan();
    let mut lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 1.0), b),
        ProfileVertex::new(p2(0.0, 2.0), 0.0),
    ]);
    // The dome cap leaves the cylinder wall tangentially at (1, 1) --
    // intended smooth cap, declared (#101).
    lp = lp.with_tangent_joints(vec![2]);
    lp
}

#[test]
fn survives_near_full_wedge_pole_junction_unwrap() {
    // Formerly finding_near_full_wedge_pole_junction_unwrap_fails: for
    // θ ∈ (3π/2, 2π) a loop with a pole/apex junction AND a rim used
    // to unwrap the final meridian's column nearest prev_u, where the
    // wrong branch (2π − θ < π/2 away) wins; the polygon self-crossed
    // and the CDT constraint pre-check refused the body. Fixed: the
    // final traversal's column contains the loop's closing vertex, so
    // it unwraps nearest the closing anchor (out[0].u) — exact closure
    // for every wedge angle. Sweep the formerly-failing window up to
    // 2π − 0.01 with full acceptance oracles (watertight, certified
    // chordal bound, exact volume band, byte-identical rebuild).
    let thetas = [
        1.5 * PI + 0.01,
        4.72, // reviewer's verified failing boundary point
        5.5,
        2.0 * PI - 0.1,
        2.0 * PI - 0.01,
    ];
    for theta in thetas {
        // Cone wall (apex junction): V = θ/6; A = base sector θ/2 +
        // lateral √2·θ/2 + two unit right-triangle caps.
        let body = rev(cone_profile(), Revolution::Partial(theta));
        let v_exact = theta / 6.0;
        let a_exact = theta / 2.0 + core::f64::consts::SQRT_2 * theta / 2.0 + 1.0;
        for delta in [0.25, 0.04] {
            check_mesh_acceptance(&body, delta, Some((v_exact, a_exact)));
        }

        // Sphere dome cap (rim + pole junction in one loop): V =
        // cylinder θ/2 + hemisphere θ/3; A = base sector θ/2 + wall θ
        // + dome θ + two caps (unit square + quarter disc).
        let body = rev(silo_profile(), Revolution::Partial(theta));
        let v_exact = theta / 2.0 + theta / 3.0;
        let a_exact = theta / 2.0 + theta + theta + 2.0 * (1.0 + PI / 4.0);
        for delta in [0.25, 0.04] {
            check_mesh_acceptance(&body, delta, Some((v_exact, a_exact)));
        }
    }
}

#[test]
fn survives_sphere_wedges_pole_to_pole_bands() {
    // Sphere wedge bands: two profile-copy meridians + two poles, no
    // rim — the area-vector azimuth-half rule under many angles.
    for theta in [0.2, PI / 2.0, PI - 0.02, PI, PI + 1.0, 2.0 * PI - 0.08] {
        let body = rev(half_disc(), Revolution::Partial(theta));
        for delta in [0.3, 0.05] {
            let mesh = check_mesh_acceptance(&body, delta, None);
            // Orientation must be outward: positive volume close to
            // the exact wedge of a ball.
            let v = signed_volume(&mesh);
            let v_exact = theta / (2.0 * PI) * 4.0 * PI / 3.0;
            assert!(
                (v - v_exact).abs() < 4.0 * PI * (delta + common::eps()),
                "band volume {v} vs {v_exact} at theta {theta}"
            );
        }
    }
}

#[test]
fn survives_mirror_nappe_wedges() {
    for theta in [0.4, PI - 0.05, PI + 0.05] {
        let body = rev(diamond_profile(), Revolution::Partial(theta));
        for delta in [0.3, 0.04] {
            check_mesh_acceptance(&body, delta, None);
        }
    }
}

#[test]
fn survives_many_segment_dome_wedges() {
    // Multi-band dome under partial revolve: many meridian junctions
    // per loop (walk order stress: rim→meridian→rim chains).
    let t = |theta: f64| (theta / 4.0).tan();
    let a1 = 0.8f64;
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), t(a1)),
        ProfileVertex::new(p2(a1.sin(), -a1.cos()), t(a1)),
        ProfileVertex::new(p2((2.0 * a1).sin(), -(2.0 * a1).cos()), t(PI - 2.0 * a1)),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    for theta in [PI - 0.01, PI + 0.7] {
        let body = rev(lp.clone(), Revolution::Partial(theta));
        for delta in [0.3, 0.05] {
            check_mesh_acceptance(&body, delta, None);
        }
    }
}

#[test]
fn survives_outward_shell_assumption_via_public_api() {
    // The area-vector rule assumes outward-oriented shells. Feed the
    // public API deliberately CW (inward-looking) profiles — the
    // profile layer canonicalizes, so every constructible body must
    // still mesh with positive signed volume.
    let cw_tri = ProfileLoop::polygon([p2(0.0, 0.0), p2(0.0, 1.0), p2(1.0, 0.0)]); // CW
    let cw_half_disc = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 1.0), -1.0),
        ProfileVertex::new(p2(0.0, -1.0), 0.0),
    ]); // CW traversal of the same half disc
    for lp in [cw_tri, cw_half_disc] {
        for r in [Revolution::Full, Revolution::Partial(2.0)] {
            let body = rev(lp.clone(), r);
            let mesh = check_mesh_acceptance(&body, 0.1, None);
            assert!(signed_volume(&mesh) > 0.0, "inward-oriented shell escaped");
        }
    }
}

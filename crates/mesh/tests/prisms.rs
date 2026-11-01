//! Acceptance: extruded prisms — the all-planar L-prism, the genus-1
//! holed prism (cap rings), and the rounded-square prism (cylinder
//! wall patches) — across a coarse→fine δ sweep.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::*;

const DELTAS: [f64; 3] = [0.1, 0.01, 0.001];

#[test]
fn l_prism_tessellates_exactly() {
    let body = l_prism();
    // All-planar: the mesh volume is exact (area 3 × height 1).
    for delta in DELTAS {
        let mesh = check_mesh_acceptance(&body, delta, Some((3.0, 14.0)));
        let v = mesh::validate::signed_volume(&mesh);
        assert!(
            (v - 3.0).abs() < 1e-9,
            "planar volume must be exact, got {v}"
        );
    }
}

#[test]
fn holed_prism_tessellates_with_rings() {
    let body = holed_prism();
    // 3×3 minus 1×1, height 1 ⇒ V = 8 exactly (all planar).
    // A = 2·8 + 4·3 + 4·1 = 32.
    for delta in DELTAS {
        let mesh = check_mesh_acceptance(&body, delta, Some((8.0, 32.0)));
        let v = mesh::validate::signed_volume(&mesh);
        assert!(
            (v - 8.0).abs() < 1e-9,
            "planar volume must be exact, got {v}"
        );
    }
}

#[test]
fn rounded_prism_tessellates_cylinder_patches() {
    let body = rounded_prism();
    // Area = 4 − (4 − π)·r² with r = 1/2; height 1.
    let r: f64 = 0.5;
    let v_exact = 4.0 - (4.0 - core::f64::consts::PI) * r * r;
    // Lateral: 4 straight walls of length 1 + full circle of radius r,
    // caps 2·area.
    let a_exact = 2.0 * v_exact + 4.0 + core::f64::consts::TAU * r;
    for delta in DELTAS {
        check_mesh_acceptance(&body, delta, Some((v_exact, a_exact)));
    }
}

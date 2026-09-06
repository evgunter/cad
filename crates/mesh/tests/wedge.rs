//! Acceptance: partial revolves — the ordinary θ-wedge (off-axis
//! rectangle) and the axis-touching wedge (axis edge shared by the two
//! wedge caps).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::*;

#[test]
fn wedge_tessellates_theta_patches() {
    let body = wedge();
    let pi = core::f64::consts::PI;
    // Quarter of the washer: V = 3π/4.
    // A = 2 caps (area 1) + quarter annuli (2·3π/4) + quarter
    // cylinders ((4π + 2π)/4).
    let a = 2.0 + 1.5 * pi + 1.5 * pi;
    for delta in [0.1, 0.01, 0.001] {
        check_mesh_acceptance(&body, delta, Some((0.75 * pi, a)));
    }
}

#[test]
fn axis_wedge_tessellates_with_axis_edge() {
    let body = axis_wedge();
    let pi = core::f64::consts::PI;
    // Quarter of a cylinder of radius 1, height 1: V = π/4.
    // A = 2 caps (1 each) + quarter shell (π/2) + quarter discs (π/2).
    let a = 2.0 + 0.5 * pi + 0.5 * pi;
    for delta in [0.1, 0.01, 0.001] {
        check_mesh_acceptance(&body, delta, Some((0.25 * pi, a)));
    }
}

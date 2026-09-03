//! M5 S11 acceptance row: the concave-notched body tessellates
//! watertight — the mesh half of the S11 end-to-end row.
//!
//! S11 makes the sweep constructors mint `sense: false` on walls whose
//! material lies against the chart normal (concave arc walls, hole
//! walls, a revolve's bore/under-side walls). The mesh crate is
//! Category B almost everywhere — triangle winding comes from the
//! loops' stored windings, which were already material-true, so the
//! emitted geometry of the notched body must be exactly as honest as
//! before: watertight, manifold, consistently wound, positive signed
//! volume converging on the analytic 3.0 (and the hole plate on
//! 16 − π). If any consumer secretly multiplied a shoelace by
//! `sense_sign`, the reversed walls would emit inward triangles and
//! `check_mesh` would refuse the mixed-sense body here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use core::f64::consts::{FRAC_PI_8, PI};
use profile::RawLoop;

use common::*;
use geom_core::Tol;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::Body;

/// The S11 concave-notched prism (area exactly 3, height 1). Exact
/// boundary area: 3 sides of the box (2 + 1.5 + 1.5), two arc walls of
/// length (π/2)·√2 each, and two caps of area 3.
fn notched() -> Body<f64> {
    let b = FRAC_PI_8.tan();
    let lp = profile::Open
        .at(p2(0.0, 0.0))
        .arc_to(profile::Bulge { p: p2(2.0, 0.0), b }, Tol::witness())
        .unwrap()
        .line_to(p2(2.0, 1.5), Tol::witness())
        .unwrap()
        .arc_to(
            profile::Bulge {
                p: p2(0.0, 1.5),
                b: -b,
            },
            Tol::witness(),
        )
        .unwrap()
        .line_to(profile::Start, Tol::witness())
        .unwrap()
        .loop_;
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(geom_core::Tol::witness())
        .unwrap();
    extrude(&vp, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// The hole plate: 4×4×1 with a unit-radius through hole — the hole
/// walls are the reversed-sense population.
fn hole_plate() -> Body<f64> {
    let outer = ProfileLoop::polygon([p2(0.0, 0.0), p2(4.0, 0.0), p2(4.0, 4.0), p2(0.0, 4.0)]);
    let hole = ProfileLoop::new(vec![
        ProfileVertex::new(p2(1.0, 2.0), 1.0),
        ProfileVertex::new(p2(3.0, 2.0), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![outer, hole])
        .validate(geom_core::Tol::witness())
        .unwrap();
    extrude(&vp, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

#[test]
fn notched_body_tessellates_watertight_and_converges() {
    let body = notched();
    let v = 3.0;
    let a = 2.0 * 3.0 + (2.0 + 1.5 + 1.5) + 2.0 * (PI / 2.0) * 2.0_f64.sqrt();
    for delta in [0.1, 0.01, 0.002] {
        check_mesh_acceptance(&body, delta, Some((v, a)));
    }
}

#[test]
fn hole_plate_tessellates_watertight_and_converges() {
    let body = hole_plate();
    let v = 16.0 - PI;
    // Caps 2(16 − π), outer walls 16, hole wall 2π·1·1.
    let a = 2.0 * (16.0 - PI) + 16.0 + 2.0 * PI;
    for delta in [0.1, 0.01, 0.002] {
        check_mesh_acceptance(&body, delta, Some((v, a)));
    }
}

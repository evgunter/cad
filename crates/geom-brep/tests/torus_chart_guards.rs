//! **The torus chart Green form's three refusals, each red on its own.**
//!
//! `geom_brep::props::curved`'s torus arm integrates `−∮ F(v) du` over
//! each loop's lift, read against an anchor at the middle of that
//! loop's rim levels. A loop whose lift does not close has no region
//! under it, and the anchored sum then reads a number that is not the
//! face's: a single-rim loop that winds the torus contributes exactly
//! zero (its one rim IS its anchor), so a seamless band would measure
//! as nothing, silently. The rows below hand each guard a face only it
//! can refuse, and pin the refusal by name, so removing any one of
//! `props_torus_lift_closed_u`, `props_torus_lift_closed_v` or the
//! torus `props_ring_winding` turns its row red.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo::edge;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{
    LoopEdge, PropsError, boundary_material_sign, boundary_material_sign_loops, curved_face,
    curved_face_loops,
};
use std::f64::consts::{PI, TAU};

const RR: f64 = 0.020;
const R0: f64 = 0.005;

fn torus() -> Surface<f64> {
    Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: RR,
        minor_radius: R0,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The parallel at tube angle `v`, walked in `u` from `u0` to `u1`.
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, R0 * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: RR + R0 * v.cos(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The meridian at azimuth `u`, walked in `v` from `v0` to `v1`.
fn mer(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(RR * u.cos(), RR * u.sin(), 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: R0,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        v0,
        v1,
        a,
        b,
    )
}

/// The chart area of `[u0, u1] × [v0, v1]`.
fn band_area(du: f64, v0: f64, v1: f64) -> f64 {
    R0 * du * (RR * (v1 - v0) + R0 * (v1.sin() - v0.sin()))
}

fn assert_refused(row: &str, got: Result<impl std::fmt::Debug, PropsError>, what: &str) {
    match got {
        Err(PropsError::NotIsoRectangle { what: w }) if w == what => {}
        other => panic!("{row}: must refuse by {what}: {other:?}"),
    }
}

/// **A loop that winds the torus in `u` refuses
/// (`props_torus_lift_closed_u`).** A staircase loop once round the
/// axis: half a turn at `v = 0.2`, up a meridian, the other half at
/// `v = 0.7`, and down the meridian it started on. It bounds no region
/// of the chart, and its anchored sum would read a nonzero area that
/// is no face's. The seamless band of two full parallels is the case a
/// hand-built body would hand over: each rim is its own loop's anchor
/// and reads zero, so without this guard the band refuses only by
/// accident of its rings' zero sides, and by another name.
#[test]
fn a_loop_winding_the_torus_in_u_refuses() {
    let (s, b) = (torus(), band());
    let stair = vec![
        rim(0.2, 0.0, PI, 0, 1),
        mer(PI, 0.2, 0.7, 1, 2),
        rim(0.7, PI, TAU, 2, 3),
        mer(TAU, 0.7, 0.2, 3, 0),
    ];
    assert_refused(
        "staircase flux",
        curved_face(&s, &stair, true, b),
        "props_torus_lift_closed_u",
    );
    assert_refused(
        "staircase side",
        boundary_material_sign(&s, &stair, b),
        "props_torus_lift_closed_u",
    );
    let lower = vec![rim(0.2, 0.0, TAU, 0, 0)];
    let upper = vec![rim(0.7, TAU, 0.0, 1, 1)];
    assert_refused(
        "seamless band flux",
        curved_face_loops(&s, &[&lower, &upper], true, b),
        "props_torus_lift_closed_u",
    );
    assert_refused(
        "seamless band side",
        boundary_material_sign_loops(&s, &[&lower, &upper], b),
        "props_torus_lift_closed_u",
    );
}

/// **A loop that winds the torus in `v` refuses
/// (`props_torus_lift_closed_v`).** A staircase once round the tube:
/// up half a turn of the meridian at `u = 0.3`, back along a parallel,
/// up the other half at `u = 0`, and back along the first parallel a
/// period above where it began. Its `u`-steps close, so only the `v`
/// guard can see it; unguarded it reads the area between its two
/// parallels as if it bounded it.
#[test]
fn a_loop_winding_the_torus_in_v_refuses() {
    let (s, b) = (torus(), band());
    let (v0, v1) = (0.2, 0.2 + PI);
    let stair = vec![
        rim(v0, 0.0, 0.3, 0, 1),
        mer(0.3, v0, v1, 1, 2),
        rim(v1, 0.3, 0.0, 2, 3),
        mer(0.0, v1, v0 + TAU, 3, 0),
    ];
    assert_refused(
        "staircase flux",
        curved_face(&s, &stair, true, b),
        "props_torus_lift_closed_v",
    );
    assert_refused(
        "staircase side",
        boundary_material_sign(&s, &stair, b),
        "props_torus_lift_closed_v",
    );
}

/// **A ring is a hole only when wound against its face
/// (`props_ring_winding`).** The rectangle `[0, 1.1] × [0.2, 0.7]` with
/// the hole `[0.3, 0.6] × [0.3, 0.5]`: wound against the outer loop it
/// is subtracted, at the exact difference; handed the same way it
/// would be ADDED, so it refuses.
#[test]
fn a_torus_ring_wound_with_its_face_refuses() {
    let (s, b) = (torus(), band());
    let outer = vec![
        rim(0.2, 0.0, 1.1, 0, 1),
        mer(1.1, 0.2, 0.7, 1, 2),
        rim(0.7, 1.1, 0.0, 2, 3),
        mer(0.0, 0.7, 0.2, 3, 0),
    ];
    let hole = vec![
        mer(0.3, 0.3, 0.5, 10, 11),
        rim(0.5, 0.3, 0.6, 11, 12),
        mer(0.6, 0.5, 0.3, 12, 13),
        rim(0.3, 0.6, 0.3, 13, 10),
    ];
    let fc = curved_face_loops(&s, &[&outer, &hole], true, b)
        .unwrap_or_else(|e| panic!("the holed torus face was refused: {e:?}"));
    let area = band_area(1.1, 0.2, 0.7) - band_area(0.3, 0.3, 0.5);
    assert!(
        ((fc.area - area) / area).abs() <= 1e-12,
        "holed area {} against {area}",
        fc.area
    );
    let same = vec![
        rim(0.3, 0.3, 0.6, 10, 11),
        mer(0.6, 0.3, 0.5, 11, 12),
        rim(0.5, 0.6, 0.3, 12, 13),
        mer(0.3, 0.5, 0.3, 13, 10),
    ];
    assert_refused(
        "same-wound ring flux",
        curved_face_loops(&s, &[&outer, &same], true, b),
        "props_ring_winding",
    );
    assert_refused(
        "same-wound ring side",
        boundary_material_sign_loops(&s, &[&outer, &same], b),
        "props_ring_winding",
    );
}

//! **A cone face with a hole measures its outer loop less its rings.**
//! The cone's closed form is the boundary's vector area
//! (`cone_face_closed_form`), summed over every loop; these rows pin
//! the ring certificate it rests on: a ring is a hole, contractible on
//! the nappe and wound against its face, or the face refuses.
//!
//! Areas are asserted against closed forms; this file is on CI's
//! `eps ∈ {default, 1e-6, 1e-12}` matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::{band, eps};
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face_loops};

/// `sin α = cos α` for the cone under every row: apex at `(0, 0, −1)`,
/// axis `+Z`, half-angle 45°.
const SIN_A: f64 = core::f64::consts::FRAC_1_SQRT_2;
const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
const APEX_Z: f64 = -1.0;

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: p3(0.0, 0.0, APEX_Z),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The rim at slant `v`, traversed in azimuth from `u0` to `u1`.
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, APEX_Z + v * SIN_A),
            axis: v3(0.0, 0.0, 1.0),
            radius: v * SIN_A,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The generator at azimuth `u`, from slant `t0` to `t1`.
fn generator(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::edge(
        Curve3::Line {
            origin: p3(0.0, 0.0, APEX_Z),
            dir: v3(u.cos() * SIN_A, u.sin() * SIN_A, SIN_A),
        },
        t0,
        t1,
        a,
        b,
    )
}

/// The window `[v0, v1] × [u0, u1]`, traversed `+u` along its low rim
/// when `with` and `−u` otherwise.
fn window(v: (f64, f64), u: (f64, f64), with: bool) -> Vec<LoopEdge<f64>> {
    let ((v0, v1), (u0, u1)) = (v, u);
    let edges = vec![
        rim(v0, u0, u1, 0, 1),
        generator(u1, v0, v1, 1, 2),
        rim(v1, u1, u0, 2, 3),
        generator(u0, v1, v0, 3, 0),
    ];
    if with {
        edges
    } else {
        vec![
            generator(u0, v0, v1, 0, 3),
            rim(v1, u0, u1, 3, 2),
            generator(u1, v1, v0, 2, 1),
            rim(v0, u1, u0, 1, 0),
        ]
    }
}

/// The area of the window `[v0, v1] × Δu` on the cone,
/// `sin α·Δu·(v1² − v0²)/2`.
fn area(v: (f64, f64), du: f64) -> f64 {
    SIN_A * du * (v.1 * v.1 - v.0 * v.0) * 0.5
}

fn measure(loops: &[Vec<LoopEdge<f64>>]) -> Result<(f64, f64), PropsError> {
    let loops: Vec<&[LoopEdge<f64>]> = loops.iter().map(Vec::as_slice).collect();
    curved_face_loops(&cone(), &loops, true, band()).map(|fc| (fc.flux, fc.area))
}

const OUTER: ((f64, f64), (f64, f64)) = ((1.0, 2.0), (0.0, PI));
const HOLE: ((f64, f64), (f64, f64)) = ((1.3, 1.6), (1.0, 2.0));

/// **A window with a hole is the window less the hole**, in both
/// traversals: its area, and its flux `apex·A⃗ = −A_z`, whose magnitude
/// is the area times `sin α`. Red against the mutant "ring skipped",
/// which answers the window's whole area.
#[test]
fn a_window_with_a_hole_measures_the_window_less_the_hole() {
    let want = area(OUTER.0, PI) - area(HOLE.0, 1.0);
    for with in [true, false] {
        let (flux, got) = measure(&[
            window(OUTER.0, OUTER.1, with),
            window(HOLE.0, HOLE.1, !with),
        ])
        .unwrap_or_else(|e| panic!("traversal {with}: refused {e:?}"));
        assert!(
            (got - want).abs() <= 1e-12 * want,
            "traversal {with}: area {got} against {want}"
        );
        assert!(
            (flux.abs() - want * SIN_A).abs() <= 1e-12 * want,
            "traversal {with}: flux {flux} against ±{}",
            want * SIN_A
        );
    }
}

/// **A hole whose joints close within the band is a hole.** The hole's
/// inner rim starts `δ` of azimuth past its generator's foot, a gap of
/// `0.95·ε` there, which the loop's closure admits; summed over its
/// edges alone, the ring's winding is `−δ`, which at the outer rim's
/// arm is `1.17·ε`, inside the escalation band. The winding is a whole
/// number of turns, decided against half a turn, so the face measures
/// the window less the hole, within the `ε` the gap moves it by. Red at
/// a zero test of the winding.
#[test]
fn a_hole_whose_joints_close_within_the_band_is_a_hole() {
    let ((v0, v1), (u0, u1)) = HOLE;
    let delta = 0.95 * eps() / (v0 * SIN_A);
    let against = vec![
        generator(u0, v0, v1, 0, 3),
        rim(v1, u0, u1, 3, 2),
        generator(u1, v1, v0, 2, 1),
        rim(v0, u1 + delta, u0, 1, 0),
    ];
    let with = vec![
        rim(v0, u0, u1 + delta, 0, 1),
        generator(u1, v0, v1, 1, 2),
        rim(v1, u1, u0, 2, 3),
        generator(u0, v1, v0, 3, 0),
    ];
    let want = area(OUTER.0, PI) - area(HOLE.0, 1.0);
    for (outer, hole) in [(true, against), (false, with)] {
        let (_, got) = measure(&[window(OUTER.0, OUTER.1, outer), hole])
            .unwrap_or_else(|e| panic!("traversal {outer}: refused {e:?}"));
        assert!(
            (got - want).abs() <= eps().max(1e-12 * want),
            "traversal {outer}: area {got} against {want}"
        );
    }
}

/// **A ring wound with its face refuses.** Its vector area would be
/// ADDED to the face's, silently (the window's area plus the hole's).
#[test]
fn a_ring_wound_with_its_face_refuses() {
    for with in [true, false] {
        let got = measure(&[window(OUTER.0, OUTER.1, with), window(HOLE.0, HOLE.1, with)]);
        assert!(
            matches!(
                got,
                Err(PropsError::NotIsoRectangle {
                    what: "props_ring_winding"
                })
            ),
            "traversal {with}: {got:?}"
        );
    }
}

/// **A ring that winds the axis refuses.** A band between two whole
/// rims is a face, but which rim is its outer loop is not a fact about
/// the band, so the winding test cannot read it: the second rim
/// refuses as a ring that is not contractible, in both traversals.
#[test]
fn a_ring_that_winds_the_axis_refuses() {
    for (lo, hi) in [(1.0, 2.0), (2.0, 1.0)] {
        let outer = vec![rim(lo, 0.0, PI, 0, 1), rim(lo, PI, TAU, 1, 0)];
        let ring = vec![rim(hi, TAU, PI, 0, 1), rim(hi, PI, 0.0, 1, 0)];
        let got = measure(&[outer, ring]);
        assert!(
            matches!(
                got,
                Err(PropsError::NotIsoRectangle {
                    what: "props_cone_ring_contractible"
                })
            ),
            "outer at {lo}: {got:?}"
        );
    }
}

//! **Every lane on a periodic chart decides closure one way**
//! (`geom_brep::props`' closure decision): a loop that does not close is
//! [`PropsError::ChartLoopOpen`], loops that wind as no face's boundary
//! does are [`PropsError::LoopsWindChart`], and a legal face whose
//! junction is open by less than the vertex slack measures.
//!
//! These rows are the cylinder's and the torus's closed forms; the
//! cylinder quadrature's are in `props::quad`'s unit rows
//! (`a_junction_open_by_the_vertex_tolerance_closes` and its
//! neighbours), and the spiric wall's in `props::spiric`'s.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::{band, eps};
use crate::shared::topo::edge;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, PropsError, curved_face, curved_face_loops};
use std::f64::consts::TAU;

const RR: f64 = 0.020;
const R0: f64 = 0.005;

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

fn torus() -> Surface<f64> {
    Surface::Torus {
        center: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        major_radius: RR,
        minor_radius: R0,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The unit cylinder's rim at height `z`, walked in `u` from `u0` to `u1`.
fn crim(z: f64, u0: f64, u1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, z),
            axis: v3(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

/// The unit cylinder's ruling at azimuth `u`, from height `z0` to `z1`.
fn ruling(u: f64, z0: f64, z1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Line {
            origin: p3(u.cos(), u.sin(), 0.0),
            dir: v3(0.0, 0.0, 1.0),
        },
        z0,
        z1,
        0,
        0,
    )
}

/// The torus's parallel at tube angle `v`, walked in `u` from `u0` to `u1`.
fn trim(v: f64, u0: f64, u1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, R0 * v.sin()),
            axis: v3(0.0, 0.0, 1.0),
            radius: RR + R0 * v.cos(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        0,
        0,
    )
}

/// The torus's meridian at azimuth `u`, walked in `v` from `v0` to `v1`.
fn tmer(u: f64, v0: f64, v1: f64) -> LoopEdge<f64> {
    edge(
        Curve3::Circle {
            center: p3(RR * u.cos(), RR * u.sin(), 0.0),
            axis: v3(u.sin(), -u.cos(), 0.0),
            radius: R0,
            u_ref: v3(u.cos(), u.sin(), 0.0),
        },
        v0,
        v1,
        0,
        0,
    )
}

/// The cylinder rectangle `u ∈ [τ − 1, τ]`, `z ∈ [0, 1]`, its ruling on
/// the seam starting `gap` above the bottom rim's end.
fn cylinder_rectangle(gap: f64) -> Vec<LoopEdge<f64>> {
    vec![
        crim(0.0, TAU - 1.0, TAU),
        ruling(0.0, gap, 1.0),
        crim(1.0, TAU, TAU - 1.0),
        ruling(-1.0, 1.0, 0.0),
    ]
}

/// The torus rectangle `u ∈ [τ − 1, τ]`, `v ∈ [0.2, 0.7]`, its meridian
/// on the seam starting `gap` metres round the tube from the lower
/// parallel's end.
fn torus_rectangle(gap: f64) -> Vec<LoopEdge<f64>> {
    let (v0, v1) = (0.2, 0.7);
    vec![
        trim(v0, TAU - 1.0, TAU),
        tmer(TAU, v0 + gap / R0, v1),
        trim(v1, TAU, TAU - 1.0),
        tmer(TAU - 1.0, v1, v0),
    ]
}

/// **A legal face with a junction on the seam, open by `2ε`, measures**
/// on both closed forms: the two carriers' ends about their vertex are
/// within the slack the decision states (`3ε`), whatever whole turn the
/// seam puts between their azimuths. Without that slack the gap sits in
/// the band and escalates.
#[test]
fn a_legal_face_with_a_junction_on_the_seam_measures() {
    let gap = 2.0 * eps();
    let cyl = curved_face(&cylinder(), &cylinder_rectangle(gap), true, band())
        .unwrap_or_else(|e| panic!("cylinder: {e:?}"));
    assert!(
        (cyl.area - 1.0).abs() < 1e-6,
        "cylinder: area {} vs 1",
        cyl.area
    );
    let tor = curved_face(&torus(), &torus_rectangle(gap), true, band())
        .unwrap_or_else(|e| panic!("torus: {e:?}"));
    let want = R0 * (RR * 0.5 + R0 * (0.7_f64.sin() - 0.2_f64.sin()));
    assert!(
        (tor.area - want).abs() < 1e-6 * want,
        "torus: area {} vs {want}",
        tor.area
    );
}

/// **A loop that does not close is [`PropsError::ChartLoopOpen`]** on
/// both closed forms: the seam ruling or meridian starts `0.1` m off the
/// edge before's end.
#[test]
fn an_open_loop_is_chart_loop_open_on_every_closed_form() {
    assert_eq!(
        curved_face(&cylinder(), &cylinder_rectangle(0.1), true, band()).map(|_| ()),
        Err(PropsError::ChartLoopOpen),
        "cylinder"
    );
    assert_eq!(
        curved_face(&torus(), &torus_rectangle(0.1 * R0), true, band()).map(|_| ()),
        Err(PropsError::ChartLoopOpen),
        "torus"
    );
}

/// **Loops winding twice each way are [`PropsError::LoopsWindChart`]**
/// on both closed forms: two whole rims at one level make a loop that
/// turns twice, and its partner at another level turns twice back. The
/// face's loops sum to no turn, so only the one-turn check sees it; read
/// as a band, it measures twice the band's area.
#[test]
fn a_two_turn_pair_winds_as_no_boundary_on_every_closed_form() {
    let twice = |rim: fn(f64, f64, f64) -> LoopEdge<f64>, level: f64, forward: bool| {
        let (a, b) = if forward { (0.0, TAU) } else { (TAU, 0.0) };
        vec![rim(level, a, b), rim(level, a, b)]
    };
    let (lower, upper) = (twice(crim, 0.0, true), twice(crim, 1.0, false));
    assert_eq!(
        curved_face_loops(&cylinder(), &[&lower, &upper], true, band()).map(|_| ()),
        Err(PropsError::LoopsWindChart),
        "cylinder"
    );
    let (lower, upper) = (twice(trim, 0.2, true), twice(trim, 0.7, false));
    assert_eq!(
        curved_face_loops(&torus(), &[&lower, &upper], true, band()).map(|_| ()),
        Err(PropsError::LoopsWindChart),
        "torus"
    );
}

/// **Four whole rims of one cylinder bound no single face**
/// ([`PropsError::LoopsWindChart`]), at `f64` and at the interval
/// scalar. Wound `+ + − −` or `+ − − +` up the axis, each turns once and
/// together they turn zero times, but a face is one connected region
/// and has at most two winding loops on its boundary: read as one face
/// the first measures `8π`, the middle band twice, and the second `0`.
#[test]
fn four_winding_rims_bound_no_single_face() {
    fn rims<T: geom_core::Real>(senses: [bool; 4]) -> Vec<Vec<LoopEdge<T>>> {
        senses
            .iter()
            .enumerate()
            .map(|(i, &up)| {
                let (a, b) = if up { (0.0, TAU) } else { (TAU, 0.0) };
                vec![edge(
                    Curve3::Circle {
                        center: p3(0.0, 0.0, i as f64),
                        axis: v3(0.0, 0.0, 1.0),
                        radius: T::one(),
                        u_ref: v3(1.0, 0.0, 0.0),
                    },
                    a,
                    b,
                    0,
                    0,
                )]
            })
            .collect()
    }
    fn measure<T: geom_core::Decide>(senses: [bool; 4]) -> Result<(), PropsError> {
        let loops = rims::<T>(senses);
        let refs: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
        let s = Surface::Cylinder {
            origin: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: T::one(),
            u_ref: v3(1.0, 0.0, 0.0),
        };
        curved_face_loops(&s, &refs, true, band()).map(|_| ())
    }
    for senses in [[true, true, false, false], [true, false, false, true]] {
        assert_eq!(
            measure::<f64>(senses),
            Err(PropsError::LoopsWindChart),
            "{senses:?} at f64"
        );
        assert_eq!(
            measure::<geom_core::Interval>(senses),
            Err(PropsError::LoopsWindChart),
            "{senses:?} at the interval scalar"
        );
    }
}

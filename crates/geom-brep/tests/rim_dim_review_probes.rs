//! ADVERSARIAL REVIEW PROBES (branch review/rim-dim, not for merge).
//!
//! R1: does the per-kind metering change flip a legitimate GROUPING
//! verdict, and is the post-fix verdict the honest one in BOTH
//! directions? Cylinder wall patch whose top rim is split into two
//! arcs at axial levels differing by δ:
//!
//! - small body (r = 1e-4 m), δ = 5e-8 m = 50ε: the true point
//!   deviation is decisively nonzero, so the honest verdict is
//!   REFUSAL (three groups, du inconsistent). Pre-fix the area
//!   comparand δ·r = 5e-12 < ε silently GROUPED them (accepts
//!   50ε-separated geometry); post-fix must refuse.
//! - large body (r = 1e3 m), δ = 5e-10 m = 0.5ε: legitimately
//!   coincident at tolerance, honest verdict is COMPUTE. Pre-fix the
//!   area comparand δ·r = 5e-7 >> Kε decisively SPLIT them (spurious
//!   refusal); post-fix must compute.
//!
//! Runs under the default ε = 1e-9 (no env override in CI).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::props::{LoopEdge, curved_face};
use geom_core::k_stats::Probe;
use geom_core::{Band, Point3, Vec3};
use geom_curves::Curve3;
use geom_surfaces::Surface;

fn p(x: f64, y: f64, z: f64) -> Point3<Probe> {
    Point3::new(Probe(x), Probe(y), Probe(z))
}

fn v3(x: f64, y: f64, z: f64) -> Vec3<Probe> {
    Vec3::new(Probe(x), Probe(y), Probe(z))
}

/// Cylinder wall patch u ∈ [0, π/2], v ∈ [0, h]: bottom rim one arc
/// at v = 0, TOP rim split into two arcs at v = h and v = h + delta.
fn split_rim_patch(r: f64, h: f64, delta: f64) -> (Surface<Probe>, Vec<LoopEdge<Probe>>) {
    let u1 = core::f64::consts::FRAC_PI_2;
    let um = u1 / 2.0;
    let surface = Surface::Cylinder {
        origin: p(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        radius: Probe(r),
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let rim = |z: f64, t0: f64, t1: f64, forward: bool, tags: (u32, u32)| LoopEdge {
        carrier: Curve3::Circle {
            center: p(0.0, 0.0, z),
            axis: v3(0.0, 0.0, 1.0),
            radius: Probe(r),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        t0: Probe(t0),
        t1: Probe(t1),
        forward,
        start: tags.0,
        end: tags.1,
    };
    let meridian = |u: f64, z0: f64, z1: f64, forward: bool, tags: (u32, u32)| LoopEdge {
        carrier: Curve3::Line {
            origin: p(r * u.cos(), r * u.sin(), 0.0),
            dir: v3(0.0, 0.0, 1.0),
        },
        t0: Probe(z0),
        t1: Probe(z1),
        forward,
        start: tags.0,
        end: tags.1,
    };
    let edges = vec![
        rim(0.0, 0.0, u1, true, (0, 1)),
        meridian(u1, 0.0, h + delta, true, (1, 2)),
        rim(h + delta, um, u1, false, (2, 3)),
        rim(h, 0.0, um, false, (3, 4)),
        meridian(0.0, 0.0, h, false, (4, 0)),
    ];
    (surface, edges)
}

/// Small body, split 50ε: honest verdict is typed refusal.
#[test]
fn small_body_rims_50eps_apart_refuse() {
    let (surface, edges) = split_rim_patch(1e-4, 1e-3, 5e-8);
    let got = curved_face(&surface, &edges, Probe(1.0), Band::linear().unwrap());
    assert!(
        got.is_err(),
        "50eps-separated split rim must NOT silently group (pre-fix area \
         comparand 5e-12 < eps grouped it): {got:?}"
    );
}

/// Large body, split 0.5ε: honest verdict is compute (coincident at
/// tolerance).
#[test]
fn large_body_rims_half_eps_apart_compute() {
    let (surface, edges) = split_rim_patch(1e3, 1e4, 5e-10);
    let got = curved_face(&surface, &edges, Probe(1.0), Band::linear().unwrap());
    assert!(
        got.is_ok(),
        "0.5eps-separated split rim is coincident at tolerance; pre-fix the \
         inflated area comparand 5e-7 >> K*eps spuriously refused: {got:?}"
    );
}

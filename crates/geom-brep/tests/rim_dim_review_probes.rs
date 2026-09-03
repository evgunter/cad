//! ADVERSARIAL REVIEW PROBES (authored on branch review/rim-dim,
//! ADOPTED BY MERGE into the unit — authorship kept).
//!
//! R1: does the per-kind metering change flip a legitimate GROUPING
//! verdict, and is the post-fix verdict the honest one in BOTH
//! directions? Cylinder wall patch whose top rim is split into two
//! arcs at axial levels differing by δ, derived from the AMBIENT ε
//! (fix pass: originally hardcoded at the default ε = 1e-9, which
//! inverted both probes on the hosted 1e-6/1e-12 rows):
//!
//! - small body (r = 1e-4 m), δ = 50ε: the true point deviation is
//!   decisively nonzero, so the honest verdict is REFUSAL (three
//!   groups, du inconsistent). Pre-fix the area comparand δ·r =
//!   50ε·1e-4 < ε silently GROUPED them (accepts 50ε-separated
//!   geometry); post-fix must refuse.
//! - large body (r = 1e3 m), δ = 0.5ε: legitimately coincident at
//!   tolerance, honest verdict is COMPUTE. Pre-fix the area comparand
//!   δ·r = 500ε >> Kε decisively SPLIT them (spurious refusal);
//!   post-fix must compute.
//!
//! **CI EXECUTES THIS SUITE.** It is rostered in
//! `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run under the
//! DEFAULT selection by `scripts/k_probe_sweep.sh`, whose tally is floored
//! by `--check-executed`, so every assertion below is a gate and a red here
//! fails the merge. By hand:
//! `cargo test -p geom-brep --features probe --test all -- rim_dim_review_probes::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3 as p, v3};
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, curved_face};
use geom_core::Band;
use geom_core::Tol;
use geom_core::k_stats::Probe;

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
    let rim = |z: f64, t0: f64, t1: f64, forward: bool, tags: (u32, u32)| {
        LoopEdge::hand_built(
            Curve3::Circle {
                center: p(0.0, 0.0, z),
                axis: v3(0.0, 0.0, 1.0),
                radius: Probe(r),
                u_ref: v3(1.0, 0.0, 0.0),
            },
            Probe(t0),
            Probe(t1),
            forward,
            tags.0,
            tags.1,
        )
    };
    let meridian = |u: f64, z0: f64, z1: f64, forward: bool, tags: (u32, u32)| {
        LoopEdge::hand_built(
            Curve3::Line {
                origin: p(r * u.cos(), r * u.sin(), 0.0),
                dir: v3(0.0, 0.0, 1.0),
            },
            Probe(z0),
            Probe(z1),
            forward,
            tags.0,
            tags.1,
        )
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

/// Small body, split 50ε (ε ambient): honest verdict is typed refusal.
#[test]
fn small_body_rims_50eps_apart_refuse() {
    let eps = geom_core::Tol::witness().get().eps;
    let (surface, edges) = split_rim_patch(1e-4, 1e-3, 50.0 * eps);
    let got = curved_face(
        &surface,
        &edges,
        Probe(1.0),
        Band::linear(Tol::witness()).unwrap(),
    );
    assert!(
        got.is_err(),
        "50eps-separated split rim must NOT silently group (pre-fix the area \
         comparand 50eps*1e-4 < eps grouped it): {got:?}"
    );
}

/// Large body, split 0.5ε (ε ambient): honest verdict is compute
/// (coincident at tolerance).
#[test]
fn large_body_rims_half_eps_apart_compute() {
    let eps = geom_core::Tol::witness().get().eps;
    let (surface, edges) = split_rim_patch(1e3, 1e4, 0.5 * eps);
    let got = curved_face(
        &surface,
        &edges,
        Probe(1.0),
        Band::linear(Tol::witness()).unwrap(),
    );
    assert!(
        got.is_ok(),
        "0.5eps-separated split rim is coincident at tolerance; pre-fix the \
         inflated area comparand 500eps >> K*eps spuriously refused: {got:?}"
    );
}

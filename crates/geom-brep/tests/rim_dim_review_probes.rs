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
//!   decisively nonzero, so the honest verdict is REFUSAL. Pre-fix the
//!   area comparand δ·r = 50ε·1e-4 < ε silently GROUPED them (accepts
//!   50ε-separated geometry). Since the cylinder's flux became the
//!   chart Green form (TANG, PR 3851) there is no grouping: the loop
//!   as first written is OPEN by δ, and refuses on its closure; closed
//!   by its step ruling it is a notched wall and measures exactly.
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
use crate::shared::tol::band;
use geom::Curve3;
use geom::Surface;
use geom_brep::props::{LoopEdge, curved_face};
use geom_core::k_stats::Probe;

/// Cylinder wall patch u ∈ [0, π/2], v ∈ [0, h]: bottom rim one arc
/// at v = 0, TOP rim split into two arcs at v = h and v = h + delta.
fn split_rim_patch(
    r: f64,
    h: f64,
    delta: f64,
    closed: bool,
) -> (Surface<Probe>, Vec<LoopEdge<Probe>>) {
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
    // `closed` steps down from the split's upper arc to its lower one
    // along the ruling at `um`; without it the loop is OPEN there, by
    // `delta`.
    let mut edges = vec![
        rim(0.0, 0.0, u1, true, (0, 1)),
        meridian(u1, 0.0, h + delta, true, (1, 2)),
        rim(h + delta, um, u1, false, (2, 3)),
    ];
    if closed {
        edges.push(meridian(um, h, h + delta, false, (3, 5)));
        edges.push(rim(h, 0.0, um, false, (5, 4)));
    } else {
        edges.push(rim(h, 0.0, um, false, (3, 4)));
    }
    edges.push(meridian(0.0, 0.0, h, false, (4, 0)));
    (surface, edges)
}

/// The closed step's exact area: the `[0, u1] × [0, h]` rectangle and
/// the `[um, u1] × [h, h + delta]` step above it.
fn step_area(r: f64, h: f64, delta: f64) -> f64 {
    let u1 = core::f64::consts::FRAC_PI_2;
    r * (u1 * h + (u1 - u1 / 2.0) * delta)
}

/// Small body, split 50ε (ε ambient), the split left OPEN: the honest
/// verdict is a typed refusal, and it is the loop's closure that
/// refuses — the cylinder's Green form checks it, because its sum is
/// anchor-free only over a closed boundary.
#[test]
fn small_body_rims_50eps_apart_refuse() {
    let eps = geom_core::Tol::witness().get().eps;
    let (surface, edges) = split_rim_patch(1e-4, 1e-3, 50.0 * eps, false);
    let got = curved_face(&surface, &edges, true, band());
    assert!(
        matches!(
            got,
            Err(geom_brep::props::PropsError::NotIsoRectangle {
                what: "props_loop_closed"
            })
        ),
        "a loop open by 50eps must refuse on its closure, not measure: {got:?}"
    );
}

/// The same small body with the split CLOSED by its step ruling: a
/// notched wall, which the Green form measures at its exact area —
/// the 50ε step is real geometry, not grouped away.
#[test]
fn small_body_rims_50eps_apart_closed_by_a_step_measure() {
    let eps = geom_core::Tol::witness().get().eps;
    let (r, h, delta) = (1e-4, 1e-3, 50.0 * eps);
    let (surface, edges) = split_rim_patch(r, h, delta, true);
    let got = curved_face(&surface, &edges, true, band())
        .unwrap_or_else(|e| panic!("the closed step was refused: {e:?}"));
    let exact = step_area(r, h, delta);
    assert!(
        ((got.area.0 - exact) / exact).abs() < 1e-12,
        "area {:?} against the step's {exact}",
        got.area
    );
}

/// Large body, split 0.5ε (ε ambient): honest verdict is compute
/// (coincident at tolerance), open or closed.
#[test]
fn large_body_rims_half_eps_apart_compute() {
    let eps = geom_core::Tol::witness().get().eps;
    for closed in [false, true] {
        let (surface, edges) = split_rim_patch(1e3, 1e4, 0.5 * eps, closed);
        let got = curved_face(&surface, &edges, true, band());
        assert!(
            got.is_ok(),
            "0.5eps-separated split rim (closed: {closed}) is coincident at tolerance; \
             pre-fix the inflated area comparand 500eps >> K*eps spuriously refused: {got:?}"
        );
    }
}

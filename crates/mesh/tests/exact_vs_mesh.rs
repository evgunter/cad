//! M2 PR 7 cross-check: the exact-B-rep divergence volume
//! (`topo::mass_properties`) against the tessellation's
//! `mesh::validate::signed_volume` — a δ-consistency sanity bound,
//! never certification (the mesh is the approximation; the exact form
//! is the source of truth).
//!
//! Bound: every mesh vertex lies within δ+ε of its face's surface and
//! chords cut inward, so the enclosed-volume defect is O(δ·A). The
//! asserted band `|V_exact − V_mesh| ≤ 3·δ·A_exact` is deliberately
//! generous (sagitta-shaped defects integrate to well under A·δ).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use mesh::tessellate;
use mesh::validate::signed_volume;
use topo::mass_properties;

fn check_at(body: &topo::Body<f64>, what: &str, delta: f64) {
    let props = mass_properties(body).expect("mass properties must compute");
    let mesh = tessellate(body, delta).expect("tessellation must succeed");
    let v_mesh = signed_volume(&mesh);
    assert!(v_mesh > 0.0, "{what}: mesh volume must be positive");
    let bound = 3.0 * delta * props.surface_area;
    assert!(
        (props.volume - v_mesh).abs() <= bound,
        "{what}: exact {} vs mesh {} differ by {} > bound {bound}",
        props.volume,
        v_mesh,
        (props.volume - v_mesh).abs()
    );
}

#[test]
fn exact_volume_delta_consistent_with_mesh() {
    // δ = 1e-3 everywhere except the donut, whose CDT pays the
    // documented quadratic wall-clock (mesh crate docs) — it runs at
    // 1e-2, which still puts ~18k triangles on the torus. That triangle
    // count is an observation, not a contract: nothing asserts it and
    // nothing re-takes it, so a mesher change may move it freely. What
    // the row asserts is the volume agreement below, at whatever count
    // 1e-2 produces.
    check_at(&common::l_prism(), "L-prism", 1e-3);
    check_at(&common::holed_prism(), "holed prism", 1e-3);
    check_at(&common::rounded_prism(), "rounded prism", 1e-3);
    check_at(&common::ball(), "ball", 1e-3);
    check_at(&common::cone(), "cone", 1e-3);
    check_at(&common::washer(), "washer", 1e-3);
    check_at(&common::donut(), "donut", 1e-2);
    check_at(&common::wedge(), "wedge", 1e-3);
    check_at(&common::axis_wedge(), "axis wedge", 1e-3);
}

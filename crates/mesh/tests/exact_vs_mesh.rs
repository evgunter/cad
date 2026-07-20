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

const DELTA: f64 = 1e-3;

fn check(body: &topo::Body<f64>, what: &str) {
    let props = mass_properties(body).expect("mass properties must compute");
    let mesh = tessellate(body, DELTA).expect("tessellation must succeed");
    let v_mesh = signed_volume(&mesh);
    assert!(v_mesh > 0.0, "{what}: mesh volume must be positive");
    let bound = 3.0 * DELTA * props.surface_area;
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
    check(&common::l_prism(), "L-prism");
    check(&common::holed_prism(), "holed prism");
    check(&common::rounded_prism(), "rounded prism");
    check(&common::ball(), "ball");
    check(&common::cone(), "cone");
    check(&common::washer(), "washer");
    check(&common::donut(), "donut");
    check(&common::wedge(), "wedge");
    check(&common::axis_wedge(), "axis wedge");
}

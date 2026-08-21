//! Acceptance: genus-1 full revolves — the washer (full-2π cylinder
//! walls + slit annulus planes, double-seam boundaries) and the donut
//! (a single torus surface, both meridians `Seam`, wrapped minor
//! coordinate on one face).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::*;
use geom_core::Tol;

#[test]
fn washer_tessellates_full_period_patches() {
    let body = washer();
    let pi = core::f64::consts::PI;
    // Rectangle [1,2]×[0,1] revolved: V = π(2² − 1²)·1 = 3π;
    // A = 2·annulus + outer cyl + inner cyl = 2·3π + 4π + 2π = 12π.
    for delta in [0.1, 0.01, 0.001] {
        check_mesh_acceptance(&body, delta, Some((3.0 * pi, 12.0 * pi)));
    }
}

#[test]
fn donut_tessellates_the_torus() {
    let body = donut();
    let pi = core::f64::consts::PI;
    // R = 2, r = 1/2: V = 2π²Rr² = π², A = 4π²Rr = 4π².
    for delta in [0.1, 0.03] {
        check_mesh_acceptance(&body, delta, Some((pi * pi, 4.0 * pi * pi)));
    }
}

#[test]
fn donut_faces_share_one_torus_surface() {
    let body = donut();
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).unwrap();
    // Both wall faces lie on the single torus surface; per-face
    // patches stay separate (no face merging, ratified).
    let torus_patches: Vec<_> = mesh
        .patches
        .iter()
        .filter(|p| {
            let f = body.get_face(p.face).unwrap();
            matches!(
                body.get_surface(f.surface).unwrap(),
                geom::Surface::Torus { .. }
            )
        })
        .collect();
    assert_eq!(torus_patches.len(), 2);
    for p in &torus_patches {
        assert!(!p.triangles.is_empty());
    }
    let keys: std::collections::HashSet<_> = torus_patches
        .iter()
        .map(|p| body.get_face(p.face).unwrap().surface)
        .collect();
    assert_eq!(keys.len(), 1, "identical-by-construction surface is shared");
}

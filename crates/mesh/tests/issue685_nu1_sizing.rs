//! Issue 685 acceptance, generalised: a cone face with bare sides takes
//! no interior rows at any column count, pinned on the π/6 cone wedge
//! and an apex-free frustum wedge.
//!
//! Issue 685 decided it for one column: the cone is ruled in v, so the
//! single strip already certifies, and the rows only multiplied the
//! patch's triangles (5–9× at identical densely sampled deviation,
//! 2.409e-2 m wherever the rim is one chord). The cone's theorem
//! (`mesh::sizing::cone_grid_step`) extends that to every column count:
//! with both rims chorded on the column step and nothing on the sides,
//! the strip argument holds every triangle to one column, so the patch
//! is its rim chords fanned to the apex or zipped to the other rim, and
//! its triangle count is the rims' chord count. `check_mesh_acceptance`
//! carries the rest: watertight, certified by dense sampling against
//! the exact cone, byte-identical rebuild.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::*;
use geom::Surface;
use geom_core::{Point2, Tol};
use profile::RawLoop as _;

/// The wedge's cone-patch triangle count at one δ, through the full
/// acceptance battery.
fn cone_patch_triangles(body: &topo::Body<f64>, delta: f64) -> usize {
    let mesh = check_mesh_acceptance(body, delta, None);
    mesh.patches
        .iter()
        .filter(|p| {
            let face = body.get_face(p.face).expect("patch face is live");
            matches!(
                body.get_surface(face.surface).unwrap(),
                Surface::Cone { .. }
            )
        })
        .map(|p| p.triangles.len())
        .sum()
}

/// The wedge's counts over the measured δ sweep: one column (one rim
/// chord) down to δ ≈ 0.05, then the rim's chord count — its own
/// sagitta at radius 1, or the column step `2·acos(1 − δ/(ρ_max·cos α))`,
/// whichever is finer — fanned to the apex, with no row at any δ.
#[test]
fn the_pi6_cone_wedge_is_pinned_at_the_measured_deltas() {
    let body = cone_wedge(1.0, core::f64::consts::FRAC_PI_6);
    for (delta, expected) in [
        (0.25, 1),
        (0.1, 1),
        (0.05, 1),
        (0.025, 2),
        (0.01, 2),
        (0.004, 3),
        (0.001, 6),
    ] {
        assert_eq!(
            cone_patch_triangles(&body, delta),
            expected,
            "cone patch triangle count at delta = {delta}"
        );
    }
}

/// The apex-free member of the class: a frustum wedge whose cone face
/// carries no pole entry. The decision does not read the pole bit, so
/// a single-column frustum patch is the same single strip — and it
/// passes the same acceptance battery (the strip's triangles span the
/// full ruling extent between the two chorded rims).
#[test]
fn an_apex_free_single_column_cone_patch_is_the_same_strip() {
    // Triangle revolved π/6: cone face from (2, 0)–(1, 1), apex off
    // the profile at (0, 2), ρ_max = 2, α = π/4, vspan = √2. At
    // δ = 0.25, hu = 2·acos(1 − 0.25/√2) is the π/4 cap ≥ π/6: one
    // column. At δ = 0.04, hu ≈ 0.477 < π/6: two columns, both rims two
    // chords, and no row: the two rims zip into four triangles.
    let lp = profile::ProfileLoop::polygon([
        Point2::new(1.0, 0.0),
        Point2::new(2.0, 0.0),
        Point2::new(1.0, 1.0),
    ]);
    let body = sweep::revolve(
        &validated(vec![lp]),
        axis_y(),
        sweep::Revolution::Partial(core::f64::consts::FRAC_PI_6),
        Tol::witness(),
    )
    .unwrap()
    .body;
    assert_eq!(
        cone_patch_triangles(&body, 0.25),
        2,
        "one column, no apex: the boundary rectangle's two triangles"
    );
    assert_eq!(
        cone_patch_triangles(&body, 0.04),
        4,
        "two columns at the finer delta, and no row"
    );
}

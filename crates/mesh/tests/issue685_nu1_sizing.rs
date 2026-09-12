//! Issue 685 acceptance: the `nu == 1` sizing decision, pinned on the
//! π/6 cone wedge at the δ values the MESH-5 measurement swept.
//!
//! The decision (at `curved::grid_counts`' cone arm): one azimuth
//! column takes no interior rows — the cone is ruled in v, `hu` is
//! sized at the patch's largest radius, so the single strip already
//! certifies, and the v-schedule the empty grid range would discard is
//! not computed. The measurement behind it: emitting the scheduled
//! rows multiplied the patch's triangles 5–9× at identical densely
//! sampled deviation (2.409e-2 m at every `nu == 1` δ below — the rim
//! chord's azimuthal sagitta, which rows in the ruling direction
//! cannot touch).
//!
//! These rows pin BOTH sides of the decision boundary through the
//! public door: the single-strip counts at `nu == 1` (which are also
//! the D9 no-change pin — they are the counts the tree emitted before
//! the decision), and the honoured schedule's counts at `nu >= 2`
//! (issue 678's fence, one δ step finer). `check_mesh_acceptance`
//! carries the rest: watertight, certified by dense sampling against
//! the exact cone, byte-identical rebuild.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::*;
use geom::Surface;
use geom_core::Tol;
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

/// The measurement's own δ sweep, pinned. The first four rows are the
/// `nu == 1` regime (single strip; the boundary sits at δ ≈ 0.0682,
/// where `hu = 2·acos(1 − δ/2)` meets the π/6 span); the rest are the
/// honoured schedule, 678's pole floor included (`nu` = 3 at 0.05
/// through 0.025, then rising).
#[test]
fn the_pi6_cone_wedge_is_pinned_at_the_measured_deltas() {
    let body = cone_wedge(1.0, core::f64::consts::FRAC_PI_6);
    for (delta, expected) in [
        (0.25, 1),
        (0.1, 1),
        (0.07, 1),
        (0.0682, 1),
        (0.05, 14),
        (0.025, 18),
        (0.01, 31),
        (0.004, 93),
        (0.001, 361),
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
    // the profile at (0, 2), ρ_max = 2, vspan = √2. At δ = 0.25,
    // hu = 2·acos(1 − 0.125/2) ≈ 0.710842 ≥ π/6: one column — and at
    // this ρ_max the discarded schedule's own nv was already 1, but
    // only by a 0.53% margin (hv = ρ_max·hu ≈ 1.421685 against
    // vspan ≈ 1.414214), so that parenthetical is one small sizing
    // drift from flipping while this row's pins stand either way. At
    // δ = 0.1, hu ≈ 0.448 < π/6: two columns, the schedule honoured
    // (nv = 2, one interior point).
    let lp = profile::ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 0.0), p2(1.0, 1.0)]);
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
        cone_patch_triangles(&body, 0.1),
        5,
        "two columns at the finer delta: the honoured schedule"
    );
}

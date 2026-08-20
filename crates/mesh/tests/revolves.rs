//! Acceptance: full revolves with chart singularities — the ball
//! (two-band sphere, pole fans, no rims in either loop) and the cone
//! (apex fan, wire-case half-rims, base disc from swept radial
//! segments).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::*;

#[test]
fn ball_tessellates_with_pole_fans() {
    let body = ball();
    let (v, a) = (
        4.0 * core::f64::consts::PI / 3.0,
        4.0 * core::f64::consts::PI,
    );
    for delta in [0.1, 0.01, 0.002] {
        check_mesh_acceptance(&body, delta, Some((v, a)));
    }
}

#[test]
fn ball_pole_fan_structure() {
    // Both poles are single mesh vertices with a closed triangle fan:
    // every triangle edge at a pole is shared by exactly two triangles
    // (already covered by check_mesh; here pin that the poles ARE mesh
    // vertices of some triangles — the fans exist).
    let body = ball();
    let mesh = mesh::tessellate(&body, 0.05).unwrap();
    let mut pole_ids = Vec::new();
    for (i, p) in mesh.positions.iter().enumerate() {
        if (p.x.abs() < 1e-12) && (p.z.abs() < 1e-12) && ((p.y.abs() - 1.0).abs() < 1e-12) {
            pole_ids.push(u32::try_from(i).unwrap());
        }
    }
    // The two topology pole vertices are positions 0 and 1 (vertex
    // minting order) — dedup: exactly two pole positions total.
    assert_eq!(pole_ids.len(), 2, "expected exactly the two pole vertices");
    for pid in pole_ids {
        let n_incident: usize = mesh
            .patches
            .iter()
            .flat_map(|p| p.triangles.iter())
            .filter(|t| t.contains(&pid))
            .count();
        assert!(n_incident >= 3, "pole {pid} has no fan ({n_incident})");
    }
}

#[test]
fn cone_tessellates_with_apex_fan() {
    let body = cone();
    let pi = core::f64::consts::PI;
    // V = πr²h/3; A = π·r·slant + π·r² with r = h = 1.
    let (v, a) = (pi / 3.0, pi * 2.0f64.sqrt() + pi);
    for delta in [0.1, 0.01, 0.002] {
        check_mesh_acceptance(&body, delta, Some((v, a)));
    }
}

/// **Issue #678 regression — a silently non-watertight apex fan.**
///
/// A partial revolve whose cone face contains the apex meshed
/// NON-manifold whenever the azimuth grid sized to exactly two steps.
/// `nu == 2` puts a single interior column at `u = (u0 + u1)/2`,
/// equidistant from the walk's two pole entries, so the CDT gives each
/// of them a fan over its upper half; both fans are the same mesh
/// vertex, so every `(apex, column)` edge in the overlap is used FOUR
/// times. `tessellate` returned `Ok` — it does not run `check_mesh` —
/// and the issue's as-authored `s = 0.01` body reported
/// `NonManifoldEdge { edge: (1, 68), count: 4 }` over 63 bad edges.
///
/// The two row groups are the two ways to reach `nu == 2`, and the
/// second is why the issue's "slender wedge" framing understated it:
/// an ORDINARY unit cone at δ = 0.05 is dirty at θ = π/4 and θ = π/6.
/// Aspect ratio was never the variable — `uspan <= 2·hu` is.
///
/// Reds on a revert of `curved::pole_columns`.
#[test]
fn apex_wedges_never_size_to_a_single_azimuth_column() {
    let pi = core::f64::consts::PI;
    // The issue's shape: a slender wedge, dirty at every δ tested
    // because the sagitta cap already rules on a sub-δ cone radius.
    for s in [0.01, 0.001] {
        check_mesh_acceptance(&cone_wedge(s, pi / 2.0), 0.01, None);
    }
    // Ordinary aspect (base radius = height = 1); the wedge angle is
    // what lands `uspan/hu` in (1, 2].
    for (theta, delta) in [(pi / 4.0, 0.05), (pi / 6.0, 0.05)] {
        check_mesh_acceptance(&cone_wedge(1.0, theta), delta, None);
    }
}

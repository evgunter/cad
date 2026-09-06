//! Acceptance: full revolves with chart singularities — the ball
//! (two-band sphere, pole fans, no rims in either loop) and the cone
//! (apex fan, wire-case half-rims, base disc from swept radial
//! segments).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::*;
use geom_core::Tol;

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
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).unwrap();
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
/// The rows are chosen for what each one PINS, because `nu == 2` is
/// reached from two directions and one of them is a float knife edge.
///
/// - The issue's authored bodies (`theta = pi/2`) are kept as the
///   historical reproducer and NOT relied on: `uspan` is
///   `1.57079632679489656` against `hu = 7.85398163397448279e-1`, a
///   ratio of exactly 2.0, so one ulp of drift in `walk`'s azimuth
///   would flip them to `nu = 3` and they would go green without the
///   fix. That knife edge is the PR's own "not fixed here" item; a row
///   standing on it is evidence about placement, not about the defect.
/// - The two ordinary-aspect rows pin the ANGLE variable, which is the
///   one the issue's "slender wedge" framing missed: base radius =
///   height = 1, and `uspan/hu` lands at ~1.75 (`pi/4`) and ~1.17
///   (`pi/6`) — robustly interior to `(1, 2]`, nowhere near a `ceil`
///   boundary.
/// - Both of those sit at `nv = 4`, the MINIMUM dirty configuration
///   (`nv <= 3` is clean because the overlap has too few interior rows
///   to contain an edge), so a sizing tweak moving `nv` 4 -> 3 would
///   turn them green while the defect stayed live at finer delta. The
///   `theta = pi/3` row is the monotone-the-right-way one: `hu`
///   saturates at the `pi/4` cap on a sub-delta cone radius, so
///   `uspan/hu` is `4/3` at EVERY delta, and `nv` is in the hundreds.
///   Measured on a reverted `pole_columns`, it is dirty at delta =
///   0.05, 0.025 AND 0.01 — unlike `cone_wedge(1, pi/6)`, which goes
///   clean at delta = 0.01 because `nu` reaches 3 on its own.
///
/// Reds on a revert of `curved::pole_columns`.
#[test]
fn apex_wedges_never_size_to_a_single_azimuth_column() {
    let pi = core::f64::consts::PI;
    // The issue's shape: a slender wedge, dirty at every δ tested
    // because the sagitta cap already rules on a sub-δ cone radius.
    // Knife-edge rows (`uspan/hu == 2.0` exactly) — see the doc above.
    for s in [0.01, 0.001] {
        check_mesh_acceptance(&cone_wedge(s, pi / 2.0), 0.01, None);
    }
    // Ordinary aspect (base radius = height = 1); the wedge angle is
    // what lands `uspan/hu` in (1, 2]. `nv = 4` on both: one row from
    // the clean side of the defect.
    for (theta, delta) in [(pi / 4.0, 0.05), (pi / 6.0, 0.05)] {
        check_mesh_acceptance(&cone_wedge(1.0, theta), delta, None);
    }
    // Far from BOTH boundaries: ratio 4/3 at every δ, `nv` in the
    // hundreds.
    check_mesh_acceptance(&cone_wedge(0.01, pi / 3.0), 0.01, None);
}

/// **Issue #678, the sphere arm** — the call site nothing else reaches.
///
/// `grep Revolution::Partial crates/mesh/tests` finds washers, cones,
/// silos and half-discs, and not one of them lands a sphere pole face
/// on `nu == 2`; deleting `pole_columns` from `curved::grid_counts`'s
/// SPHERE arm left the whole suite green. **This row guards that
/// deletion**: it now goes red here.
///
/// This face is `nu = 2, nv = 8` before the floor — seven interior
/// column vertices, an overlap that could hold six edges — and it is
/// **watertight anyway**. That is what makes the sphere arm
/// prophylactic rather than corrective: a chorded ARC meridian always
/// carries the interior points that occlude the cross-fan, which a
/// cone's straight ruling never does. So this row does NOT red on a
/// revert of `pole_columns` and is not pretending to; the count is
/// what pins it. Drop the sphere arm and the floor stops firing, `nu`
/// goes back to 2, and the count moves — deliberately, and visibly,
/// rather than silently.
#[test]
fn sphere_pole_faces_are_floored_too_though_they_never_needed_it() {
    let body = sphere_wedge(core::f64::consts::FRAC_PI_4);
    check_mesh_acceptance(&body, 0.05, None);
    let tris: usize = mesh::tessellate(&body, 0.05, Tol::witness())
        .expect("sphere wedge tessellates")
        .patches
        .iter()
        .map(|p| p.triangles.len())
        .sum();
    assert_eq!(
        tris, 56,
        "sphere wedge at δ = 0.05 should carry the pole floor's nu = 3 \
         columns; 56 is that mesh"
    );
}

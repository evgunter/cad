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

/// **A frustum's small rim, joined to a far row, certifies within δ**
/// (`work/tess/a-cone-rim-chord-joined-to-a-higher-row-certifies-above-delta-s.md`).
/// Radii 0.05 and 1.5 at half-angle 40°, at a coarse δ: the small rim
/// sized on its own radius takes a chord spacing several grid columns
/// wide, and the first grid row sits at about four times its radius,
/// so a rim-chord triangle certifies at about `cos α · (ρ_row/ρ_small)`
/// of the rim's own sag. The chord pass chords a cone's rims on the
/// face's grid step, so every triangle stays inside one column and the
/// cone theorem (`sizing::cone_grid_step`) covers it.
#[test]
fn a_frustum_small_rim_joined_to_a_far_row_certifies() {
    use geom_core::Point2;
    use profile::RawLoop;
    let (small, big) = (0.05_f64, 1.5_f64);
    let height = (big - small) / 40.0_f64.to_radians().tan();
    let lp = profile::ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(big, 0.0),
        Point2::new(small, height),
        Point2::new(0.0, height),
    ]);
    let body = sweep::revolve(
        &validated(vec![lp]),
        axis_y(),
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let pi = core::f64::consts::PI;
    let slant = (big - small).hypot(height);
    let volume = pi * height / 3.0 * (big * big + big * small + small * small);
    let area = pi * (big + small) * slant + pi * (big * big + small * small);
    for delta in [0.0085, 0.008, 0.005, 0.002] {
        check_mesh_acceptance(&body, delta, Some((volume, area)));
    }
}

/// **The sphere's azimuth step is sized at the face's widest latitude**
/// (`sizing::sphere_grid_steps`, `sphere_face_widest_cos`). A polar cap
/// above latitude 1.0 (a quarter-arc profile revolved fully) has
/// `cos 1.0 ≈ 0.54` at its widest, so its azimuth step is the lattice's
/// aspect bound `√2` times its latitude step, not the equal steps a
/// sphere spanning the equator takes. Its sphere patch is pinned
/// at two δ, and the count that equal steps would cut is computed
/// beside it so the row reds if the face's latitude stops being read.
#[test]
fn a_polar_cap_sizes_its_azimuth_at_its_widest_latitude() {
    use geom_core::Point2;
    use profile::test_support::bulge_loop;
    let lat = 1.0_f64;
    let half = (core::f64::consts::FRAC_PI_2 - lat) / 2.0;
    let lp = bulge_loop(vec![
        (Point2::new(0.0, lat.sin()), 0.0),
        (Point2::new(lat.cos(), lat.sin()), (half / 2.0).tan()),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let body = sweep::revolve(
        &validated(vec![lp]),
        axis_y(),
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    for (delta, pinned) in [(0.01, 100), (0.002, 506)] {
        let mesh = check_mesh_acceptance(&body, delta, None);
        let tris: usize = mesh
            .patches
            .iter()
            .filter(|p| {
                matches!(
                    body.get_surface(body.get_face(p.face).unwrap().surface),
                    Some(geom::Surface::Sphere { .. })
                )
            })
            .map(|p| p.triangles.len())
            .sum();
        assert_eq!(tris, pinned, "the cap's sphere patch at δ {delta}");
        // Equal steps `2·acos(1 − δ/2)`: a pole fan of `nu` triangles and
        // `nv − 1` full rows of `2·nu`.
        let h = 2.0 * (1.0 - delta / 2.0).acos();
        let (nu, nv) = (
            (core::f64::consts::TAU / h).ceil(),
            ((core::f64::consts::FRAC_PI_2 - lat) / h).ceil(),
        );
        #[allow(clippy::cast_precision_loss)]
        let equal = nu * (2.0 * nv - 1.0);
        assert!(
            (tris as f64) < equal,
            "δ {delta}: {tris} triangles, no coarser than equal steps' {equal}"
        );
    }
}

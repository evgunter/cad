//! **Every triangle has a winding, and the lattice keeps clear of the
//! boundary** (`crates/mesh/README.md` `[mesh.winding]` and
//! `[mesh.sizing]`), on the bodies that settled the two rules
//! (`work/tesslane/log.md`, 2026-11-14, "Design fork, three designers"):
//! the frustum whose small rim is twenty ε wide, whole and cut, and the
//! faces that cut a triangle of no area between a lattice node and a
//! boundary point.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::HashSet;

use geom::SurfaceKind;
use geom_core::{Point3, Vec3};
use mesh::TessellateError;
use sweep::test_support::{ball_poled_y, brick, finished};
use topo::Body;

use crate::reach_probes::{Got, geometry, got, kind_of, revolved, rod, six, tol};

/// The frustum of radius `1 → 20ε` over `y ∈ [0, 1]`: its small rim
/// inherits its cone's column count
/// (`work/tesslane/a-small-rim-inherits-its-cones-column-count.md`), so
/// the cap inside that rim is cut into triangles on three chord points
/// whose height is about `r·δ/2`.
fn needle_frustum() -> Body<f64> {
    let r = 20.0 * tol().eps();
    revolved(&[(0.0, 0.0), (1.0, 0.0), (r, 1.0), (0.0, 1.0)])
}

/// The δ at which [`needle_frustum`]'s cap triangles stand `0.64` of a
/// rounding allowance high: `10⁻⁸` at the default ε, and in inverse
/// proportion to the rim's radius at another.
fn unresolved_delta() -> f64 {
    1e-8 * (1e-9 / tol().eps())
}

/// The ids of a mesh's boundary chord points and vertices: a corner
/// outside them is a lattice node.
fn boundary_ids(mesh: &mesh::Mesh) -> HashSet<u32> {
    mesh.boundaries
        .iter()
        .flat_map(|b| b.points.iter().copied())
        .collect()
}

/// A triangle's height over its longest side.
fn least_height(mesh: &mesh::Mesh, t: [u32; 3]) -> f64 {
    let [a, b, c] = t.map(|i| mesh.positions[i as usize]);
    let longest = (b - a).norm().max((c - b).norm()).max((a - c).norm());
    (b - a).cross(c - a).norm() / longest
}

/// **The whole frustum refuses `UnresolvedWinding` in its window, from
/// its lanes and from the memo alike, and meshes outside it.** At
/// [`unresolved_delta`] the cap inside the small rim holds triangles on
/// three boundary chord points whose winding is inside the rounding of
/// their coordinates (0.64 of an allowance at the least): `tessellate`
/// refuses the cap, a plane face, where its patch joins the mesh, so the
/// trimmed lane's own winding census is not what catches it. No lattice
/// node is involved and no ε is read: the cap's corners are the chord
/// pass's own points.
///
/// The refusal is taken through `tessellate_with`, twice. The first run's
/// lanes fill the memo, the cap's patch among them, and it refuses at the
/// join; the second answers every face through the cap from the memo and
/// refuses the same face. A join that read only a lane's own patch would
/// return the second mesh.
///
/// At twice that δ the least height is 1.27 allowances and the body
/// meshes, with every winding resolved; at a hundred times it meshes
/// too.
///
/// **By ε.** The rim is twenty ε wide, so the window moves with ε as
/// `1/ε`: `10⁻⁸` at the default, `10⁻⁵` at ε `10⁻¹²`. At ε `10⁻⁶` it is
/// `10⁻¹¹`, where the cone face's lattice is past what a row meshes
/// (about `10⁶` columns): the refusal cells are printed and skipped
/// there, and the body is meshed at δ `10⁻³` and `10⁻⁵`, far outside
/// its window.
#[test]
fn the_whole_frustum_refuses_in_its_window_and_meshes_outside_it() {
    let body = needle_frustum();
    let window = unresolved_delta();
    if tol().eps() > 1e-8 {
        eprintln!(
            "the whole frustum at ε {:e}: its window is δ {window:e}, past what this row \
             meshes; the refusal cells are skipped",
            tol().eps()
        );
    } else {
        let mut memo = mesh::PatchMemo::new();
        let refused =
            |memo: &mut mesh::PatchMemo| match mesh::tessellate_with(&body, window, tol(), memo) {
                Err(TessellateError::UnresolvedWinding { face, unresolved }) => {
                    let share = unresolved.height / unresolved.allowance;
                    assert!(
                        (0.5..0.8).contains(&share),
                        "the refusal names a triangle {share} of its allowance high"
                    );
                    face
                }
                other => panic!(
                    "the whole frustum at δ = {window:e} refuses UnresolvedWinding, got {:?}",
                    other.map(|t| t.mesh.patches.len())
                ),
            };
        let first = refused(&mut memo);
        assert_eq!(kind_of(&body, first), SurfaceKind::Plane);
        let (hits, misses) = (memo.hits(), memo.misses());
        assert!(
            hits == 0 && misses > 0,
            "the first run runs its lanes: {hits} hits, {misses} misses"
        );
        assert_eq!(
            refused(&mut memo),
            first,
            "the same face refuses from the memo"
        );
        assert_eq!(
            (memo.hits(), memo.misses()),
            (misses, misses),
            "the second run answers every face through the refusing one from the memo"
        );
    }
    let outside = if tol().eps() > 1e-8 {
        vec![1e-3, 1e-5]
    } else {
        vec![2.0 * window, 100.0 * window, 1e-3]
    };
    for delta in outside {
        let mesh = mesh::tessellate(&body, delta, tol())
            .unwrap_or_else(|e| panic!("the whole frustum at δ = {delta:e} meshes, got {e:?}"));
        mesh::validate::check_mesh(&mesh).expect("a closed mesh");
        mesh::validate::check_winding(&mesh).expect("every winding resolved");
    }
}

/// **The cut frustum does not refuse: a triangle may be thin, and small
/// beside ε.** [`needle_frustum`] split by the tilted plane through
/// `(0, 0.5, 0)` meshes on both sides at `δ = 10⁻⁴`, and the mesh is the
/// body's ([`geometry`]). At the default ε the half above keeps 226
/// certified triangles with a lattice corner whose height is at or under
/// ε (the least `5.6·10⁻¹⁰` m, 4·10³ rounding allowances): the small rim's
/// chords are `r·θ` long, so no clearance in lattice steps makes them
/// taller, and the promise does not ask it to.
///
/// **By ε.** The count is the default row's. At ε `10⁻⁶` and `10⁻¹²`
/// the rim is another width and the row asserts the mesh and its
/// windings alone.
#[test]
fn the_cut_frustum_meshes_with_thin_triangles_resolved() {
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.5, 0.0),
        Vec3::new(0.3f64.sin(), 0.3f64.cos(), 0.0),
        tol(),
    );
    let whole = finished("the frustum", needle_frustum(), tol());
    let split = topo::splitting::split(&whole, &plane, tol()).expect("the frustum splits");
    let delta = 1e-4;
    for (side, part) in [("above", &split.above), ("below", &split.below)] {
        let half = part.body().expect("a body");
        let mesh = mesh::tessellate(half, delta, tol())
            .unwrap_or_else(|e| panic!("the cut frustum, {side}: meshes, got {e:?}"));
        mesh::validate::check_mesh(&mesh).expect("a closed mesh");
        geometry(half, &mesh, delta).unwrap_or_else(|e| panic!("the cut frustum, {side}: {e}"));
        if side == "above" && tol().eps() == geom_core::tolerance::DEFAULT_EPS {
            let boundary = boundary_ids(&mesh);
            let thin: Vec<f64> = mesh
                .patches
                .iter()
                .flat_map(|p| &p.triangles)
                .filter(|t| t.iter().any(|i| !boundary.contains(i)))
                .map(|&t| least_height(&mesh, t))
                .filter(|&h| h <= tol().eps())
                .collect();
            let least = thin.iter().copied().fold(f64::INFINITY, f64::min);
            assert_eq!(
                thin.len(),
                226,
                "triangles with a lattice corner at or under ε in height"
            );
            assert!(
                (5e-10..6e-10).contains(&least),
                "the least of them stands {least:e} m"
            );
        }
    }
}

/// **The faces that cut a triangle of no area mesh with every winding
/// resolved**
/// (`work/tesslane/a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area.md`).
/// The box over `x ∈ (0.5, 3)` against the `y`-poled unit ball, over
/// `z ∈ (0, 2)` and over `z ∈ (−2, 2)`: at `δ = 0.04` the section circle
/// meets the equator at azimuth `π/3`, where a lattice column of the
/// sphere face crosses its row `v = 0`, `3·10⁻¹⁶` from the loop's vertex.
/// The lattice lays no node within its clearance of the loop, so every
/// op meshes and its mesh is the body's ([`geometry`], which reads
/// `validate::check_winding`). The torus ring of the same row is
/// `reach_probes`' `a_torus_wall_holding_a_spiric_ring_meshes_on_the_torus_row`.
#[test]
fn a_node_within_rounding_of_a_loop_vertex_is_not_laid() {
    let ball = finished("the ball", ball_poled_y(1.0, Vec3::zero(), tol()), tol());
    for z in [(0.0, 2.0), (-2.0, 2.0)] {
        let block = finished(
            "the box",
            brick::<f64>((0.5, 3.0), (-2.0, 2.0), z, tol()),
            tol(),
        );
        for (op, run) in six(&block, &ball) {
            let what = format!("the box over z ∈ {z:?} and the ball, {op}");
            let built = run.unwrap_or_else(|e| panic!("{what}: the op builds, got {e:?}"));
            let body = &built
                .body()
                .unwrap_or_else(|| panic!("{what}: a body"))
                .body;
            for delta in [0.04, 1e-2] {
                assert_eq!(got(body, delta), Got::Meshed, "{what}, δ = {delta:e}");
            }
        }
    }
}

/// **A face a twentieth of a lattice row high meshes, and its mesh is
/// the body's.** A rod's wall between two parallel tilted cuts `w` apart
/// is a band oblique to the chart, far narrower than a lattice row at
/// the widths below, so its triangles stand on the two cuts' chord points
/// and on any point the join's split adds (`mesh::join`), and each is
/// certified within δ as any other ([`geometry`]). What the row holds is that such a face
/// does not refuse; which of its nodes are laid is `mesh::trimmed`'s
/// `lay` rows' to hold.
#[test]
fn a_band_narrower_than_its_clearance_meshes_within_its_certificate() {
    let n = Vec3::new(0.5f64.sin(), 0.0, 0.5f64.cos());
    for (w, deltas) in [(0.002, &[1e-3][..]), (0.0005, &[1e-3, 1e-4][..])] {
        let whole = finished("the rod", rod(0.5, -1.0, 2.0), tol());
        let cut = |at: f64| topo::test_support::split_plane(Point3::origin() + n * at, n, tol());
        let above = topo::splitting::split(&whole, &cut(0.0), tol()).expect("the rod splits");
        let above = finished(
            "the half above",
            above.above.body().expect("a body").clone().into_body(),
            tol(),
        );
        let band = topo::splitting::split(&above, &cut(w), tol()).expect("the half splits");
        let band = band.below.body().expect("a body");
        for &delta in deltas {
            assert_eq!(
                got(band, delta),
                Got::Meshed,
                "the band of width {w}, δ = {delta:e}"
            );
        }
    }
}

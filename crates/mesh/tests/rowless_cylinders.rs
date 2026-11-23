//! **The chart lane lays no rows on a cylinder face**
//! (`chart_table::AnalyticRow::lattice_rows`): a pipe wall notched or
//! holed, or cut by a tilted plane, is triangulated on its boundary
//! points alone, each triangle held by its certificate. Each row pins a
//! relation to a reference meshed in the same run: the plain wall of the
//! same pipe on the rectangle lane at the chart lane's target, half of δ
//! (`trimmed::UNPROVED_HEADROOM`, [`TARGET_SHARE`]).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Affine3, Point3, Vec3};
use sweep::test_support::{brick, finished};
use topo::{AtRestBody, Body};

use crate::reach_probes::{geometry, op_of, posed, rod, tol};
use crate::split_rulings::{rulings, split_and_placed};

/// The share of δ the chart lane sizes a cylinder face at
/// (`trimmed::face_target`): the reference wall is meshed at it.
const TARGET_SHARE: f64 = 0.5;

/// The pipe every row cuts: radius 1 over `z ∈ (−2, 2)`.
const PIPE: (f64, f64, f64) = (1.0, -2.0, 4.0);

/// The triangles `mesh` holds on `body`'s cylinder faces.
fn cylinder_triangles(body: &Body<f64>, mesh: &mesh::Mesh) -> usize {
    mesh.patches
        .iter()
        .filter(|p| body.face_surface(p.face).unwrap().kind() == SurfaceKind::Cylinder)
        .map(|p| p.triangles.len())
        .sum()
}

/// `body` at `delta`: it meshes, `check_mesh` and the body's geometry
/// hold ([`geometry`]), and its cylinder faces' triangles.
fn meshed_cylinder_triangles(what: &str, body: &Body<f64>, delta: f64) -> usize {
    let mesh = mesh::tessellate(body, delta, tol())
        .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: meshes, got {e:?}"));
    mesh::validate::check_mesh(&mesh)
        .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: check_mesh, got {e:?}"));
    geometry(body, &mesh, delta).unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: {e}"));
    cylinder_triangles(body, &mesh)
}

/// A bar `x × (−w, w) × z` turned `turn` about the pipe's axis.
fn bar(x: (f64, f64), w: f64, z: (f64, f64), turn: f64) -> AtRestBody<f64> {
    posed(
        "the bar",
        brick(x, (-w, w), z, tol()),
        Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), turn),
    )
}

/// The notched and holed walls of a pipe of radius `r` over
/// `z ∈ (z0, z0 + h)`, each named: a notch down from the top rim across
/// the chart's seam and beside it, a narrow and a wide one, two notches,
/// and a bar through (a notch across the seam and a hole).
fn walls(r: f64, z0: f64, h: f64) -> Vec<(String, Body<f64>)> {
    let pipe = finished("the pipe", rod(r, z0, h), tol());
    let top = z0 + h;
    let notch = |w: f64, depth: f64, turn: f64| {
        bar(
            (0.0, 1.1 * r),
            w * r,
            (top - depth * h, top + 0.5 * h),
            turn,
        )
    };
    let mut out: Vec<(String, Body<f64>)> = [
        ("a notch across the seam", notch(0.3, 0.5, 0.0)),
        ("a notch beside the seam", notch(0.3, 0.5, 1.0)),
        ("a narrow deep notch", notch(0.1, 0.875, 1.0)),
        ("a wide shallow notch", notch(0.6, 0.125, 1.0)),
    ]
    .into_iter()
    .map(|(what, b)| (what.to_owned(), op_of(&pipe, &b, "A ∖ B")))
    .collect();
    let two = topo::union(&notch(0.3, 0.5, 1.0), &notch(0.3, 0.5, 2.5), tol()).unwrap();
    let two = finished("two bars", Body::clone(&two.body().unwrap().body), tol());
    out.push(("two notches".to_owned(), op_of(&pipe, &two, "A ∖ B")));
    let through = bar(
        (-1.1 * r, 1.1 * r),
        0.3 * r,
        (z0 + 0.4 * h, z0 + 0.6 * h),
        0.0,
    );
    out.push(("a bar through".to_owned(), op_of(&pipe, &through, "A ∖ B")));
    out
}

/// FNV-1a over a mesh's positions' bits, its patches' faces and
/// triangles, and its boundary polylines.
fn digest(mesh: &mesh::Mesh) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |x: u64| {
        for b in x.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    };
    for p in &mesh.positions {
        eat(p.x.to_bits());
        eat(p.y.to_bits());
        eat(p.z.to_bits());
    }
    for patch in &mesh.patches {
        for t in &patch.triangles {
            for &i in t {
                eat(u64::from(i));
            }
        }
    }
    for b in &mesh.boundaries {
        for &i in &b.points {
            eat(u64::from(i));
        }
    }
    h
}

/// The pipe of radius `r` over `z ∈ (z0, z0 + h)` below a plane through
/// its middle tilted `tilt` about `y`.
fn tilted(r: f64, z0: f64, h: f64, tilt: f64) -> Body<f64> {
    let pipe = finished("the pipe", rod(r, z0, h), tol());
    let below = posed(
        "the half space",
        brick(
            (-3.0 * r, 3.0 * r),
            (-3.0 * r, 3.0 * r),
            (z0 - h, z0 + 0.5 * h),
            tol(),
        ),
        Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, z0 + 0.5 * h),
            Vec3::new(0.0, 1.0, 0.0),
            tilt,
        ),
    );
    op_of(&pipe, &below, "A ∩ B")
}

/// The plain wall's triangles at the chart lane's target: the
/// rectangle lane's, which lays one column strip per sagitta step.
fn reference(delta: f64) -> usize {
    let (r, z0, h) = PIPE;
    meshed_cylinder_triangles("the plain pipe", &rod(r, z0, h), delta * TARGET_SHARE)
}

/// **A notched or holed wall takes the plain wall's triangles, and a
/// hole's span twice.** Each wall of [`walls`] at δ `10⁻²` and `10⁻³`
/// meshes, is the body's, and holds at most 1.1 times the plain wall's
/// triangles at the lane's target, or 1.4 times with a hole: its pieces
/// are strips between two rows of boundary points, and a hole's span is
/// two strips (measured 0.875–1.062 and 1.24–1.375). Laid rows made 4 to 220
/// times.
#[test]
fn a_notched_or_holed_wall_takes_its_strips_triangles() {
    let (r, z0, h) = PIPE;
    for delta in [1e-2, 1e-3] {
        let reference = reference(delta);
        for (what, body) in walls(r, z0, h) {
            let n = meshed_cylinder_triangles(&what, &body, delta);
            let share = if what == "a bar through" { 1.4 } else { 1.1 };
            #[allow(clippy::cast_precision_loss)]
            let ratio = n as f64 / reference as f64;
            assert!(
                ratio <= share,
                "{what}, δ = {delta:e}: {n} triangles on its walls, {ratio:.3} times the plain \
                 wall's {reference} at the lane's target, past {share}"
            );
        }
    }
}

/// **A wall cut by a plane tilted `φ` takes the plain wall's triangles
/// over `cos φ`.** The tilted rim is an ellipse whose chart image is a
/// sinusoid (no iso line), chorded at the lane's step along its length,
/// which is the rim's over `cos φ` at most; the wall is one strip between
/// it and the plain rim. At δ `10⁻²` and `10⁻³`, `φ` 0.05, 0.3 and 0.8
/// (measured 1.000, 1.03–1.04 and 1.36–1.375 against 1.001, 1.047 and
/// 1.435).
#[test]
fn a_wall_cut_on_a_tilt_takes_the_plain_walls_triangles_over_its_cosine() {
    let (r, z0, h) = PIPE;
    for delta in [1e-2, 1e-3] {
        let reference = reference(delta);
        for tilt in [0.05f64, 0.3, 0.8] {
            let n = meshed_cylinder_triangles("the tilted wall", &tilted(r, z0, h, tilt), delta);
            #[allow(clippy::cast_precision_loss)]
            let ratio = n as f64 / reference as f64;
            assert!(
                ratio <= 1.0 / tilt.cos(),
                "a wall cut on a tilt {tilt}, δ = {delta:e}: {n} triangles, {ratio:.3} times the \
                 plain wall's {reference}, past 1/cos φ = {:.3}",
                1.0 / tilt.cos()
            );
        }
    }
}

/// **A notched wall with every line edge split takes one triangle more
/// per split point at most.** Each wall of [`walls`] with each of its
/// body's line edges split at its middle, placed obliquely
/// (`split_rulings::split_and_placed`): its cylinder faces hold no fewer
/// triangles than the unsplit wall's and at most one more per edge split,
/// at δ `10⁻²` (measured: six more where six split points lie on the
/// wall's rulings, twelve where twelve do).
#[test]
fn a_notched_wall_with_split_rulings_takes_a_triangle_per_point() {
    let (r, z0, h) = PIPE;
    let delta = 1e-2;
    for (what, body) in walls(r, z0, h) {
        let n = meshed_cylinder_triangles(&what, &body, delta);
        let lines = rulings(&body);
        let split = split_and_placed(&body, &lines, &[0.5]);
        let mesh = mesh::tessellate(&split, delta, tol())
            .unwrap_or_else(|e| panic!("{what}, split: meshes, got {e:?}"));
        mesh::validate::check_mesh(&mesh)
            .unwrap_or_else(|e| panic!("{what}, split: check_mesh, got {e:?}"));
        mesh::validate::check_winding(&mesh)
            .unwrap_or_else(|e| panic!("{what}, split: winding, got {e:?}"));
        let ns = cylinder_triangles(&split, &mesh);
        assert!(
            n <= ns && ns <= n + lines.len(),
            "{what}: split {ns} triangles against {n} unsplit, {} edges split",
            lines.len()
        );
    }
}

/// **The mesh is a function of the body and δ.** A notched wall, a holed
/// one and one cut on a tilt at δ `10⁻²`, digested (positions' bits,
/// triangles, boundaries), are the same bytes at every ε the suite runs
/// (1e-9, 1e-6 and 1e-12 read, as below): nothing the chart lane does on
/// a cylinder face reads ε.
#[test]
fn a_rowless_cylinder_face_meshes_the_same_bytes_at_every_eps() {
    let (r, z0, h) = PIPE;
    let walls = walls(r, z0, h);
    let named = |what: &str| &walls.iter().find(|(w, _)| w == what).unwrap().1;
    for (what, body, want) in [
        (
            "a notch across the seam",
            named("a notch across the seam"),
            0xafb1_7b20_d96d_aab5u64,
        ),
        (
            "a bar through",
            named("a bar through"),
            0xa2e2_4aa6_9d8c_0247,
        ),
        (
            "a wall cut on a tilt 0.3",
            &tilted(r, z0, h, 0.3),
            0x3958_884a_9d9e_9a15,
        ),
    ] {
        let got = digest(&mesh::tessellate(body, 1e-2, tol()).unwrap());
        assert_eq!(got, want, "{what}: the mesh's digest {got:#018x}");
    }
}

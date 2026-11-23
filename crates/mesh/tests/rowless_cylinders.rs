//! **The chart lane lays no rows on a cylinder face**
//! (`chart_table::AnalyticRow::lattice_rows`): a pipe wall notched,
//! holed or cut on a tilt is triangulated on its boundary points, and a
//! triangle whose certificate refuses gets a point inside it and the face
//! is triangulated again (`trimmed`, "The retry loop").
//!
//! **What such a face holds, exactly.** A constrained triangulation of a
//! polygon with `n` vertices, `h` holes and `p` points inside it has
//! `T = n − 2 + 2h + 2p` triangles: it is a connected planar graph with
//! `V = n + p` vertices, `n` boundary edges, every inner face a triangle
//! and `h + 1` outer ones, so `E = (3T + n)/2` and `V − E + (T + 1 + h)
//! = 2`. On this lane `n` is the trim polygon's points, each traversal of
//! an edge its chord points but the last (a seam the loop walks twice
//! counts twice, as the polygon holds two copies of it at two azimuths);
//! no triangle with two copies of one vertex is dropped on a cylinder,
//! which has no singular point; and `p` is every point the lane inserted:
//! with no lattice, the shared-side cure's points and the correction's
//! ([`common::rowless::count`]). So `p = 0` is "no rows and nothing
//! refused", and each point the correction adds costs two triangles.
//! The count does not cover a pinch, or a vertex a loop meets again on
//! one branch: that is one vertex of the triangulation counted twice in
//! `n`. No body of these rows has one.
//!
//! What the counts come to against the plain wall of the same pipe on
//! the rectangle lane at the lane's target (half of δ) is measured, not
//! held: a notched wall 0.88–1.06 times, a wall with a hole 1.24–1.38, a
//! bar of half-width 0.6 or 0.9 of the radius through it 1.48–1.63 and
//! 1.76–1.88, a comb of six notches 1.30 at δ 10⁻², and a wall cut on a
//! tilt `φ` about `(1 + cos^{−3/2} φ)/2` (1.00 to 5.35 for `φ` 0.05 to
//! 1.35): the count follows the boundary's points, and those follow the
//! boundary's length round the axis.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use topo::Body;

use crate::common::rowless::{Count, count, tilted, two_holes, walls};
use crate::reach_probes::{geometry, geometry_but_the_volume, tol};
use crate::split_rulings::{rulings, split_and_placed};

/// `body` at `delta` meshes, passes `check_mesh` and is the body's
/// ([`geometry`]); each cylinder face's [`Count`].
fn counts(what: &str, body: &Body<f64>, delta: f64) -> Vec<Count> {
    counts_reading(what, body, delta, true)
}

/// [`counts`], reading the volume only where `volume` (a body whose
/// line edges are split is not finished, and its mass properties are not
/// read: [`geometry_but_the_volume`]).
fn counts_reading(what: &str, body: &Body<f64>, delta: f64, volume: bool) -> Vec<Count> {
    let mesh = mesh::tessellate(body, delta, tol())
        .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: meshes, got {e:?}"));
    mesh::validate::check_mesh(&mesh)
        .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: check_mesh, got {e:?}"));
    if volume {
        geometry(body, &mesh, delta)
    } else {
        geometry_but_the_volume(body, &mesh, delta)
    }
    .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: {e}"));
    mesh.patches
        .iter()
        .filter(|p| body.face_surface(p.face).unwrap().kind() == SurfaceKind::Cylinder)
        .map(|p| count(body, &mesh, p.face))
        .collect()
}

/// Each count is Euler's, and `inserted` holds of its `p`.
fn holds(what: &str, delta: f64, counts: &[Count], inserted: impl Fn(usize) -> bool) {
    assert!(!counts.is_empty(), "{what}: a cylinder face");
    for c in counts {
        assert_eq!(
            c.triangles,
            c.euler(),
            "{what}, δ = {delta:e}: {c:?} is no triangulation of its points"
        );
        assert!(
            inserted(c.p),
            "{what}, δ = {delta:e}: {c:?} inserted {} points",
            c.p
        );
    }
}

/// **A notched, holed or tilted wall is its boundary points and nothing
/// else.** The walls of `common::rowless::walls` (notches across and
/// beside the seam, narrow, wide, two, a comb of six; bars through of
/// half-width 0.3, 0.6 and 0.9 of the radius) and the wall cut on a tilt
/// of 0.05, 0.3, 0.8, 1.25 and 1.35, at δ 10⁻¹, 10⁻² and 10⁻³: every
/// cylinder face holds `n − 2 + 2h` triangles, no point inserted.
#[test]
fn a_cut_wall_is_its_boundary_points_alone() {
    for delta in [1e-1, 1e-2, 1e-3] {
        for (what, body) in walls() {
            holds(what, delta, &counts(what, &body, delta), |p| p == 0);
        }
        for tilt in [0.05, 0.3, 0.8, 1.25, 1.35] {
            let what = format!("a wall cut on a tilt {tilt}");
            holds(&what, delta, &counts(&what, &tilted(tilt), delta), |p| {
                p == 0
            });
        }
    }
}

/// **A split ruling is a boundary point more.** Each wall of
/// `common::rowless::walls` with each of its body's line edges split at
/// its middle, placed obliquely (`split_rulings::split_and_placed`), at
/// δ 10⁻²: every cylinder face is still its boundary points alone, the
/// split points among them.
#[test]
fn a_notched_wall_with_split_rulings_is_its_boundary_points_alone() {
    let delta = 1e-2;
    for (what, body) in walls() {
        let split = split_and_placed(&body, &rulings(&body), &[0.5]);
        holds(
            what,
            delta,
            &counts_reading(what, &split, delta, false),
            |p| p == 0,
        );
    }
}

/// **Two small holes a column and a half apart mesh by the correction,
/// at two triangles a point.** `common::rowless::two_holes`: with no
/// point between the holes, the triangulation of the boundary points
/// joins their facing corners across the gap, a side wider than the
/// target's columns, and its certificate refuses; the lane puts a point
/// inside each refused triangle and triangulates again. The cells are
/// those that refuse with the correction off, at three scales, and the
/// two of the first probe that refuse on the lane's former rows (`r =
/// 10`, δ 0.1), which mesh here at the first attempt. Main refuses cells
/// of the class at `r = 10`: those 2 of the first probe, and 10 of 108 of
/// a second that PR 7526's review ran (gap 1.5 to 1.6, two and three
/// holes). Measured: one round, four points, on every
/// refusing cell, after which the worst certificate reads 0.48·δ to
/// 0.49·δ. Every cell builds and meshes at every ε the suite runs.
#[test]
fn two_holes_across_a_column_and_a_half_mesh_by_the_correction() {
    for (r, delta, h, gap, inserted) in [
        (1.0, 1e-2, 1.0, 1.5, 4),
        (1.0, 1e-2, 1.0, 1.6, 4),
        (1.0, 1e-2, 1.9, 1.5, 4),
        (1.0, 1e-3, 1.33, 1.5, 4),
        (1.0, 1e-3, 1.33, 1.6, 4),
        (1e-3, 1e-5, 1e-3, 1.5, 4),
        (1e-3, 1e-5, 1e-3, 1.6, 4),
        (1e-3, 1e-5, 1.9e-3, 1.5, 4),
        (10.0, 0.1, 10.0, 1.5, 0),
        (10.0, 0.1, 13.3, 1.5, 0),
    ] {
        let what = format!("two holes, r {r}, h {h}, gap {gap}");
        let body = two_holes(r, h, gap, delta);
        let counts = counts(&what, &body, delta);
        let total: usize = counts.iter().map(|c| c.p).sum();
        holds(&what, delta, &counts, |_| true);
        assert_eq!(
            total, inserted,
            "{what}, δ = {delta:e}: the correction's points"
        );
    }
}

//! The rows, end to end: a real body tessellated with the meter armed,
//! turned into the rows a sweep writes.
//!
//! What is checked is what this crate is responsible for — that EVERY
//! face gets a row (the question "which face IS the scene's cost" is
//! unanswerable if only the Hessian-sized lane reports), that the rows
//! account for every triangle in the mesh, and that the sizing columns
//! belong to the Hessian-sized lane and to no other.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Vec3};
use mesh::budget::{self, Mode};
use profile::{ProfileLoop, RawLoop as _};
use sweep::loft_body;
use tess_meter::{Chart, face_rows};
use topo::Body;

/// The `loft_prism` corpus body (#212): squares at z = 0 and 2, the
/// non-affine trapezoid at z = 1, v-degree 2 — NURBS walls and planar
/// caps in one body, which is the mix the row rules are about.
fn loft_prism() -> Body<f64> {
    let quad = |pts: [(f64, f64); 4]| -> sweep::Section {
        vec![ProfileLoop::polygon(
            pts.iter().map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let sections = vec![
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
        quad([(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]),
    ];
    let places: Vec<Affine3<f64>> = [0.0, 1.0, 2.0]
        .iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect();
    loft_body::<f64>(&sections, &places, 2)
        .expect("the corpus loft builds")
        .body
}

#[test]
fn every_face_gets_a_row_and_only_nurbs_faces_get_sizing() {
    let body = loft_prism();
    budget::arm(Mode::Sizing);
    let mesh = mesh::tessellate(&body, 6e-3).expect("tessellates");
    let measures = budget::take();
    let rows = face_rows(6e-3, &body, &mesh, &measures);

    assert_eq!(
        rows.len(),
        body.faces().count(),
        "one row per face, planar caps included"
    );
    assert_eq!(
        rows.iter().map(|r| r.triangles).sum::<usize>(),
        mesh.patches
            .iter()
            .map(|p| p.triangles.len())
            .sum::<usize>(),
        "the rows account for every triangle in the mesh"
    );
    for r in &rows {
        assert_eq!(
            r.nurbs.is_some(),
            r.chart == Chart::Nurbs,
            "sizing columns belong to the Hessian-sized lane and to no other"
        );
    }

    let walls: Vec<_> = rows.iter().filter_map(|r| r.nurbs).collect();
    assert!(!walls.is_empty(), "the loft's walls are NURBS faces");
    for n in &walls {
        // The per-cell ideal must not cost more than the schedule it
        // is the ideal FOR. This one can fail: `span_opt_cells` sums
        // each cell's own cheapest split over its own clipped extent,
        // while `grid_cells` is the BANDED schedule's sum over the
        // whole trim box — two derivations that a banding change can
        // invert.
        //
        // Its former sibling, `opt_cells <= patch_cells`, is GONE
        // because it could not fail: `patch_cells` is exactly the
        // product `best_split_steps` seeds its running minimum with,
        // so the inequality held by construction of the loop rather
        // than by anything about the answer.
        assert!(
            n.span_opt_cells <= n.grid_cells,
            "the per-cell ideal cannot cost more than the schedule it \
             is the ideal for: {n:?}"
        );
        assert!(n.grid_cells > 0.0 && n.span_opt_cells > 0.0, "{n:?}");
        // (No `grid_cells <= patch_cells` assertion on purpose: the
        // per-cell schedule pays a `ceil` per cell, and a face with
        // many near-empty cells can honestly cost a few cells MORE
        // than the whole-patch grid — #547 measured exactly that on
        // the swept blades, span 0.9x.)
        assert!(
            n.worst_cert.is_finite() && n.worst_cert > 0.0,
            "the face's worst certificate is recorded: {n:?}"
        );
        assert!(
            n.worst_dev.is_nan() && n.dev_samples == 0,
            "Mode::Sizing does not resample: {n:?}"
        );
    }
}

/// A face the meter said nothing about still gets a row, and its row
/// is the empty-tailed shape — a zero in a sizing column would read as
/// a measured zero.
#[test]
fn a_face_with_no_measurements_gets_an_empty_tailed_row() {
    let body = loft_prism();
    let mesh = mesh::tessellate(&body, 6e-3).expect("tessellates");
    let rows = face_rows(6e-3, &body, &mesh, &[]);
    assert_eq!(rows.len(), body.faces().count());
    assert!(rows.iter().all(|r| r.nurbs.is_none()));
    let cols = tess_meter::CSV_HEADER.split(',').count();
    for r in &rows {
        assert_eq!(r.csv_row("s/b").split(',').count(), cols);
    }
}

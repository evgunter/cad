//! The rows, end to end: a real body tessellated with the meter armed,
//! turned into the rows a sweep writes.
//!
//! What is checked is what this crate is responsible for — that EVERY
//! face gets a row (the question "which face IS the scene's cost" is
//! unanswerable if only the Hessian-sized lane reports), that the rows
//! account for every triangle in the mesh, and that the sizing columns
//! belong to the Hessian-sized lane and to no other.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use mesh::budget::{self, Mode};
use profile::{ProfileLoop, RawLoop as _};
use sweep::loft_body;
use tess_meter::{Chart, face_rows};
use topo::Body;

/// The `loft_prism` corpus body (#212): squares at z = 0 and 2, the
/// non-affine trapezoid at z = 1, v-degree 2 — NURBS walls and planar
/// caps in one body, which is the mix the row rules are about.
fn loft_prism(tol: Tol) -> Body<f64> {
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
    loft_body::<f64>(&sections, &places, 2, tol)
        .expect("the corpus loft builds")
        .body
}

#[test]
fn every_face_gets_a_row_and_only_nurbs_faces_get_sizing() {
    let tol = Tol::witness();
    let body = loft_prism(tol);
    budget::arm(Mode::Sizing);
    let mesh = mesh::tessellate(&body, 6e-3, tol).expect("tessellates");
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
        assert!(n.grid_cells > 0.0 && n.span_opt_cells > 0.0, "{n:?}");
        // (No ordering assertion between `span_opt_cells` and either
        // cell count. Both candidates are vacuous HERE, for one
        // mechanism: `best_split_cells` seeds its running minimum with
        // the lane's own steps, so an inequality against the schedule
        // those steps produced holds by construction of the loop, not
        // by anything about the answer. `opt_cells <= patch_cells` is
        // vacuous for every body; `span_opt_cells <= grid_cells`
        // separates from its seed only when a face has more than one
        // knot-span cell, so that the BAND takes a max the per-cell
        // ideal does not — and every NURBS face of this fixture has
        // `cells = 1`. Multi-cell faces are not rare (56 of the
        // committed tour baseline's 64 NURBS rows), but a fixture
        // built from one would buy a guard rather than a detector:
        // over those 56 the ratio `span_opt_cells / grid_cells` runs
        // 0.16 to 0.73, and the one mechanism that could invert it is
        // the per-cell `ceil` named just below.)
        //
        // (No `grid_cells <= patch_cells` assertion either: the
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
    let tol = Tol::witness();
    let body = loft_prism(tol);
    let mesh = mesh::tessellate(&body, 6e-3, tol).expect("tessellates");
    let rows = face_rows(6e-3, &body, &mesh, &[]);
    assert_eq!(rows.len(), body.faces().count());
    assert!(rows.iter().all(|r| r.nurbs.is_none()));
    let cols = tess_meter::CSV_HEADER.split(',').count();
    for r in &rows {
        assert_eq!(r.csv_row("s/b").split(',').count(), cols);
    }
}

//! **The tessellation budget meter, end to end** (issue #320): the
//! meter is inert unless armed, records one row per face when it is,
//! and does not change the mesh it measures.
//!
//! The body is the `loft_prism` corpus fixture (#212), constant for
//! constant with the trimmed-NURBS suite's — degree 1×2 walls, which
//! is exactly the anisotropic class the split-slack column is about.

// The meter is opt-in (`mesh`'s `budget` feature, gated at the module
// boundary — see `mesh::budget`), so this suite is too: without it
// there is no `arm`/`take` to drive. CI runs it in its own row, the
// way the `interval` lane's rows work.
#![cfg(feature = "budget")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Tolerance, Vec3};
use mesh::budget::{self, Chart, Mode};
use mesh::validate::triangle_count;
use sweep::loft_body;
use topo::Body;

mod common;
use common::quad;

/// The `loft_prism` corpus body (#212): squares at z = 0 and 2, the
/// non-affine trapezoid at z = 1, v-degree 2.
fn loft_prism() -> Body<f64> {
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

/// Disarmed is the normal state, and it records nothing — the meter
/// must not be a thing a caller can accidentally pay for.
#[test]
fn a_disarmed_meter_records_nothing() {
    let body = loft_prism();
    assert!(!budget::armed());
    mesh::tessellate(&body, 6e-3).expect("tessellates");
    assert!(budget::take().is_empty());
}

/// Armed: one row per face, every chart, and the sizing columns filled
/// exactly on the Hessian-sized lane.
#[test]
fn every_face_gets_a_row_and_only_nurbs_faces_get_sizing() {
    let body = loft_prism();
    budget::arm(Mode::Sizing);
    let mesh = mesh::tessellate(&body, 6e-3).expect("tessellates");
    let rows = budget::take();
    assert_eq!(
        rows.len(),
        body.faces().count(),
        "one row per face, planar caps included"
    );
    assert_eq!(
        rows.iter().map(|r| r.triangles).sum::<usize>(),
        triangle_count(&mesh),
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
        // Every comparison grid is counted over the same box, so the
        // counterfactual can never be the cheapest by construction —
        // but it must never come out CHEAPER than the optimum either,
        // which would mean the comparison is not a comparison.
        assert!(
            n.opt_cells <= n.patch_cells && n.span_opt_cells <= n.span_cells,
            "a cheaper schedule cannot cost more: {n:?}"
        );
        assert!(n.grid_cells > 0.0 && n.span_opt_cells > 0.0, "{n:?}");
        // TESS-SPAN: `span_cells` sums the SAME band_schedule the lane
        // consumes, so this asserts the lane REALISED the schedule
        // (candidate generation, dedup, counting) — not an independent
        // re-derivation (mesh::budget module docs name what guards the
        // schedule itself).
        assert!(
            (n.grid_cells - n.span_cells).abs() <= 0.01 * n.span_cells + 1.0,
            "the lane's realised cell count and the schedule's sum disagree: {n:?}"
        );
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

/// The measurement does not perturb what it measures: the mesh is
/// byte-identical armed and disarmed (D9 determinism, which the meter
/// must not be an exception to).
#[test]
fn arming_the_meter_does_not_change_the_mesh() {
    let body = loft_prism();
    let plain = mesh::tessellate(&body, 6e-3).expect("tessellates");
    budget::arm(Mode::SizingAndDeviation);
    let metered = mesh::tessellate(&body, 6e-3).expect("tessellates");
    let rows = budget::take();
    assert_eq!(plain.positions.len(), metered.positions.len());
    for (a, b) in plain.positions.iter().zip(&metered.positions) {
        assert_eq!(
            (a.x.to_bits(), a.y.to_bits(), a.z.to_bits()),
            (b.x.to_bits(), b.y.to_bits(), b.z.to_bits()),
            "a metered tessellation must be bit-identical to an unmetered one"
        );
    }
    for (a, b) in plain.patches.iter().zip(&metered.patches) {
        assert_eq!(a.triangles, b.triangles);
    }
    // …and the deviation mode actually sampled something dominated by
    // the certificate it is measured against — at δ + ε, the crate's
    // documented promise, because f64 EVALUATION rounding sits outside
    // the bound. On this fixture that is not a technicality: the flat
    // walls certify at ~5e-17 and sample ~5e-16, both pure rounding
    // dust, and a bare `dev <= cert` would read that as a violation.
    let eps = Tolerance::get().eps;
    let walls: Vec<_> = rows.iter().filter_map(|r| r.nurbs).collect();
    assert!(walls.iter().all(|n| n.dev_samples > 0), "resampling ran");
    for n in &walls {
        assert!(
            n.worst_dev <= n.worst_cert + eps,
            "a sampled deviation above the face's worst certificate would be a \
             certificate violation, not a budget finding: dev {:e} cert {:e} {n:?}",
            n.worst_dev,
            n.worst_cert
        );
    }
}

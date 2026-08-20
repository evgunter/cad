//! **The budget meter's kernel half, end to end** (issue #320): the
//! meter is inert unless armed, measures every NURBS face when it is,
//! and does not change the mesh it measures.
//!
//! What the numbers MEAN is `tools/tess-meter`'s, and so are its
//! tests. What is checked here is what only this crate can check: that
//! arming is opt-in, that the measurements are attributed to the right
//! faces, and that taking them perturbs nothing.
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

use geom_core::{Affine3, Vec3};
use geom_surfaces::Surface;
use mesh::budget::{self, Mode};
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

/// The body's described-NURBS faces, in arena order.
fn nurbs_faces(body: &Body<f64>) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface).expect("face surface"),
                Surface::Nurbs(_)
            )
        })
        .map(|(fk, _)| fk)
        .collect()
}

/// Disarmed is the normal state, and it records nothing — the meter
/// must not be a thing a caller can accidentally pay for.
#[test]
fn a_disarmed_meter_records_nothing() {
    let body = loft_prism();
    assert!(!budget::armed());
    assert!(budget::deviation_samples().is_none());
    mesh::tessellate(&body, 6e-3).expect("tessellates");
    assert!(budget::take().is_empty());
}

/// Armed: one measurement per NURBS face, attributed by key, with the
/// certified bounds and the built grid on it. Non-NURBS faces are the
/// consumer's to describe (`tools/tess-meter`) and the meter says
/// nothing about them.
#[test]
fn every_nurbs_face_is_measured_once_and_by_key() {
    let body = loft_prism();
    let walls = nurbs_faces(&body);
    assert!(!walls.is_empty(), "the loft's walls are NURBS faces");
    budget::arm(Mode::Sizing);
    mesh::tessellate(&body, 6e-3).expect("tessellates");
    let measures = budget::take();
    assert_eq!(
        measures.iter().map(|m| m.face).collect::<Vec<_>>(),
        walls,
        "one measurement per NURBS face, in face-arena order, keyed to its own face"
    );
    for m in &measures {
        assert!(m.grid_cells > 0, "the built grid is counted: {m:?}");
        assert!(
            !m.cells.is_empty(),
            "the analysis cells are reported: {m:?}"
        );
        assert!(
            m.muu.is_finite() && m.muv.is_finite() && m.mvv.is_finite(),
            "the whole-patch bound is certified and finite: {m:?}"
        );
        assert!(
            m.worst_cert.is_finite() && m.worst_cert > 0.0,
            "the face's worst certificate is recorded: {m:?}"
        );
        assert!(
            m.worst_dev.is_nan() && m.worst_ratio.is_nan() && m.dev_samples == 0,
            "Mode::Sizing does not resample: {m:?}"
        );
        assert!(
            m.u.1 > m.u.0 && m.v.1 > m.v.0,
            "the trim box is a box: {m:?}"
        );
    }
}

/// The deviation pass samples, and every sample is dominated by its
/// own triangle's certificate — `worst_ratio` is that check for the
/// whole face, one number, carried out of the kernel instead of
/// asserted inside it.
#[test]
fn the_deviation_pass_samples_and_stays_under_its_certificates() {
    let body = loft_prism();
    budget::arm(Mode::Deviation {
        samples_per_edge: 6,
    });
    assert_eq!(budget::deviation_samples(), Some(6));
    mesh::tessellate(&body, 6e-3).expect("tessellates");
    let measures = budget::take();
    assert!(!measures.is_empty());
    for m in &measures {
        assert!(m.dev_samples > 0, "resampling ran: {m:?}");
        // The falsification, in the one form that carries it:
        // `worst_ratio` is the largest `d / (cert + eps)` over every
        // sample on every triangle, so `<= 1` says each sample was
        // dominated by ITS OWN triangle's certificate.
        //
        // `worst_dev <= worst_dev_cert + eps` is deliberately NOT
        // asserted beside it: `worst_dev_cert` is the certificate of
        // the triangle `worst_dev` came from, so that inequality is
        // one term of the maximum above and cannot fail unless this
        // one does.
        assert!(
            m.worst_ratio <= 1.0,
            "a triangle's samples exceeded its own certificate: {m:?}"
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
    budget::arm(Mode::Deviation {
        samples_per_edge: 6,
    });
    let metered = mesh::tessellate(&body, 6e-3).expect("tessellates");
    let measures = budget::take();
    // Non-emptiness FIRST: `all()` on an empty slice is `true`, so a
    // meter that recorded nothing would sail through the next line.
    assert!(!measures.is_empty(), "the loft's NURBS walls were measured");
    assert!(measures.iter().all(|m| m.dev_samples > 0), "resampling ran");
    assert_eq!(plain.positions.len(), metered.positions.len());
    for (a, b) in plain.positions.iter().zip(&metered.positions) {
        assert_eq!(
            (a.x.to_bits(), a.y.to_bits(), a.z.to_bits()),
            (b.x.to_bits(), b.y.to_bits(), b.z.to_bits()),
            "a metered tessellation must be bit-identical to an unmetered one"
        );
    }
    // `zip` stops at the shorter side, so a metered run that dropped a
    // patch would compare only the survivors and pass.
    assert_eq!(plain.patches.len(), metered.patches.len());
    for (a, b) in plain.patches.iter().zip(&metered.patches) {
        assert_eq!(a.face, b.face);
        assert_eq!(a.triangles, b.triangles);
    }
}

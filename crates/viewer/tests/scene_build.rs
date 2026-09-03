//! The document → evaluation → tessellation → drawable-scene path.
//!
//! This is the half of the spike a screenshot would otherwise be the
//! only evidence for, so the properties that a picture would show are
//! asserted numerically instead: the mesh closes, its triangles wind
//! outward, and δ is the lever that changes its fidelity.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]

use bvh::Axis;
use pncad::geom_core::Tol;
use viewer::camera::Camera;
use viewer::scene::{self, DisplayTolerance, SceneError};

use crate::common;

/// The plate's nominal dimensions, read from the scene rather than
/// restated beside it — a copy here is a fixture that keeps testing a
/// box the scene no longer has.
use viewer::scene::PLATE_EXTENT as PLATE;

fn delta(value: f64) -> DisplayTolerance {
    DisplayTolerance::new(value).expect("a positive display tolerance")
}

/// The signed volume the triangles enclose, by the divergence
/// theorem: `Σ a · (b × c) / 6` over the outward-wound triangles.
///
/// Positive is the whole point — it is the numerical statement of
/// "the winding is outward", which is otherwise something only a
/// rendered picture would show.
fn enclosed_volume(scene: &viewer::SceneMesh) -> f64 {
    let positions = scene.positions();
    let mut total = 0.0f64;
    for triangle in positions.chunks_exact(3) {
        let [a, b, c] = [triangle[0], triangle[1], triangle[2]];
        let a = [f64::from(a[0]), f64::from(a[1]), f64::from(a[2])];
        let b = [f64::from(b[0]), f64::from(b[1]), f64::from(b[2])];
        let c = [f64::from(c[0]), f64::from(c[1]), f64::from(c[2])];
        let cross = [
            b[1] * c[2] - b[2] * c[1],
            b[2] * c[0] - b[0] * c[2],
            b[0] * c[1] - b[1] * c[0],
        ];
        total += a[0] * cross[0] + a[1] * cross[1] + a[2] * cross[2];
    }
    total / 6.0
}

/// The spike's document evaluates, gathers a product and tessellates
/// — the whole public-API path the viewport runs on every frame.
#[test]
fn the_spike_document_becomes_a_drawable_scene() {
    let tol = Tol::witness();
    let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
    assert_eq!(
        doc.order().len(),
        3,
        "the sketch frame, one profile node and one extrude"
    );
    assert_eq!(doc.roots().len(), 1, "the extrude is the only sink");

    let mesh = scene::scene_of(&doc, delta(1.0e-4), tol).expect("the plate tessellates");
    let stats = mesh.stats();
    assert!(stats.triangles > 0);
    assert_eq!(stats.display_delta, 1.0e-4);
    assert_eq!(
        mesh.positions().len(),
        stats.triangles * 3,
        "flat shading gives every triangle its own three corners"
    );
    assert_eq!(mesh.normals().len(), mesh.positions().len());
    assert_eq!(mesh.indices().len(), mesh.positions().len());

    let bounds = mesh.bounds();
    for (axis, extent) in [
        (Axis::X, PLATE[0]),
        (Axis::Y, PLATE[1]),
        (Axis::Z, PLATE[2]),
    ] {
        let measured = bounds.max(axis) - bounds.min(axis);
        assert!(
            (measured - extent).abs() < 1e-12,
            "extent on {axis:?}: {measured} vs {extent}"
        );
    }
}

/// Every normal is a unit vector — a zero or NaN normal is the one
/// vertex-buffer defect that poisons the shading of everything it is
/// blended with.
#[test]
fn every_normal_is_a_unit_vector() {
    let tol = Tol::witness();
    let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
    let mesh = scene::scene_of(&doc, delta(1.0e-4), tol).expect("the plate tessellates");
    for (i, n) in mesh.normals().iter().enumerate() {
        let len = (f64::from(n[0]) * f64::from(n[0])
            + f64::from(n[1]) * f64::from(n[1])
            + f64::from(n[2]) * f64::from(n[2]))
        .sqrt();
        assert!(
            (len - 1.0).abs() < 1e-5,
            "normal {i} is not unit: {n:?} (length {len})"
        );
    }
}

/// The triangles wind outward, and the volume they enclose is the
/// plate's minus the hole's — within the chordal error the inscribed
/// hole polygon accounts for.
#[test]
fn the_triangles_wind_outward_and_enclose_the_right_volume() {
    let tol = Tol::witness();
    let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
    let mesh = scene::scene_of(&doc, delta(1.0e-5), tol).expect("the plate tessellates");
    let nominal = common::plate_volume();
    let enclosed = enclosed_volume(&mesh);
    assert!(
        enclosed > 0.0,
        "the winding is inward: enclosed volume {enclosed}"
    );
    // The hole is an inscribed polygon, so the mesh encloses slightly
    // MORE material than the exact body. A δ of 10 µm on a 12 mm
    // radius leaves well under a percent.
    assert!(
        enclosed >= nominal && (enclosed - nominal) / nominal < 0.01,
        "enclosed {enclosed} against nominal {nominal}"
    );
}

/// δ is the fidelity lever and it points the way it claims to:
/// halving it never coarsens the mesh, and the volume error shrinks.
#[test]
fn a_finer_delta_never_coarsens_the_mesh() {
    let tol = Tol::witness();
    let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
    let nominal = common::plate_volume();
    let mut previous: Option<(usize, f64)> = None;
    for exponent in [3.0f64, 4.0, 5.0, 6.0] {
        let d = delta(10f64.powf(-exponent));
        let mesh = scene::scene_of(&doc, d, tol).expect("the plate tessellates");
        let error = (enclosed_volume(&mesh) - nominal).abs();
        if let Some((coarser_triangles, coarser_error)) = previous {
            assert!(
                mesh.stats().triangles >= coarser_triangles,
                "a finer δ produced fewer triangles: {} then {}",
                coarser_triangles,
                mesh.stats().triangles
            );
            assert!(
                error <= coarser_error,
                "a finer δ produced a worse volume: {coarser_error} then {error}"
            );
        }
        previous = Some((mesh.stats().triangles, error));
    }
    let (finest, _) = previous.expect("at least one tessellation");
    assert!(finest > 0);
}

/// A display tolerance that is not a length is refused at the door,
/// not four calls later inside the tessellator.
#[test]
fn a_display_tolerance_that_is_not_a_length_is_refused() {
    for bad in [0.0, -1.0e-4, f64::NAN, f64::INFINITY] {
        assert!(matches!(
            DisplayTolerance::new(bad),
            Err(SceneError::InvalidDisplayTolerance { .. })
        ));
    }
    let fine = delta(1.0e-4);
    assert_eq!(fine.scaled(2.0).expect("still a length").get(), 2.0e-4);
    assert!(fine.scaled(0.0).is_err());
}

/// The scene and the camera agree: a camera framed on the scene's own
/// bounds contains every vertex the scene will draw. This is the join
/// between the two halves of this crate, and the property a viewport
/// that opens on an empty view would violate.
#[test]
fn a_camera_framed_on_the_scene_contains_every_vertex() {
    let tol = Tol::witness();
    let (doc, _root) = scene::plate_with_hole(tol).expect("the plate authors");
    let mesh = scene::scene_of(&doc, delta(1.0e-4), tol).expect("the plate tessellates");
    let aspect = 16.0 / 9.0;
    let camera = Camera::framing(&mesh.bounds(), aspect).expect("the scene frames");
    for (i, p) in mesh.positions().iter().enumerate() {
        let point =
            pncad::geom_core::Point3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2]));
        let ndc = camera
            .project(point, aspect)
            .expect("a finite aspect")
            .expect("every vertex is in front of the eye");
        assert!(
            ndc[0].abs() <= 1.0 && ndc[1].abs() <= 1.0 && (0.0..=1.0).contains(&ndc[2]),
            "vertex {i} at {point:?} projects to {ndc:?}, outside the frustum"
        );
    }
}

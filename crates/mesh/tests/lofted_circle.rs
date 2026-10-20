//! **A lofted circle meshes** (`work/tess/lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`).
//!
//! A circle section is two half-turn arcs, and `sweep::skin::segment_curve`
//! cuts each into two rational quarter arcs joined at a knot of
//! multiplicity two: each lateral wall of `sweep::test_support`'s
//! `lofted_cylinder` carries a C⁰ crease in `u`, at the arc's
//! mid-angle, which is `x = 0` on both walls. The trimmed lane cuts its
//! triangles along it (`mesh::trimmed`'s `Creases`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::Tol;
use mesh::validate::{check_mesh, signed_volume};
use sweep::test_support::lofted_cylinder;

/// The wall patches of `mesh`: the faces on a NURBS surface.
fn walls<'m>(body: &topo::Body<f64>, mesh: &'m mesh::Mesh) -> Vec<&'m mesh::FacePatch> {
    mesh.patches
        .iter()
        .filter(|p| {
            let face = body.get_face(p.face).unwrap();
            matches!(body.get_surface(face.surface), Some(Surface::Nurbs(_)))
        })
        .collect()
}

/// Red on `main` before the crease lines: the wall refused at the C¹
/// gate (`UnsupportedNurbsFace`, "a sharp crease inside it"), and with
/// the gate open alone, at the certificate (`CertificateExceeded`,
/// bound `inf`), because uniform grid columns straddle the crease.
///
/// Not every δ meshes: between about 8.6e-4 and 1.6e-3 a wall refuses
/// typed, at sliver triangles in a dense row strip away from the
/// crease, the class `work/tess/lofted-rational-walls-refuse-at-some-deltas`
/// records (a crease-free four-arc loft refuses near 4.3e-4).
#[test]
fn a_loft_of_two_circles_tessellates() {
    let body = lofted_cylinder(1.0, Tol::witness());
    for delta in [3e-2, 1e-2, 2e-3, 5e-4] {
        let mesh = mesh::tessellate(&body, delta, Tol::witness())
            .unwrap_or_else(|e| panic!("δ = {delta:e}: the lofted circle tessellates: {e:?}"));
        check_mesh(&mesh).expect("watertight, manifold, outward");
        let v = signed_volume(&mesh);
        let pi = core::f64::consts::PI;
        assert!(
            ((v - pi) / pi).abs() < 3.0 * delta,
            "δ = {delta:e}: mesh volume {v} vs π"
        );
    }
}

/// No wall triangle crosses the crease: on both walls it is the line
/// `x = 0`, so a triangle's corners may touch it but never lie strictly
/// on both sides of it. The corners on the crease are within rounding
/// of `x = 0` (the surface evaluated at the crease, or the rim circle at
/// its joint angle), hence the band.
#[test]
fn no_wall_triangle_crosses_the_crease() {
    let body = lofted_cylinder(1.0, Tol::witness());
    let mesh = mesh::tessellate(&body, 1e-2, Tol::witness()).unwrap();
    let band = 1e-12;
    let mut on = 0;
    for patch in walls(&body, &mesh) {
        for t in &patch.triangles {
            let xs = t.map(|i| mesh.positions[i as usize].x);
            let (lo, hi) = (xs[0].min(xs[1]).min(xs[2]), xs[0].max(xs[1]).max(xs[2]));
            assert!(
                lo >= -band || hi <= band,
                "face {:?}: triangle {t:?} crosses the crease, x in [{lo:e}, {hi:e}]",
                patch.face
            );
            on += usize::from(xs.iter().any(|x| x.abs() <= band));
        }
    }
    assert!(on > 0, "no wall triangle has a corner on the crease");
}

/// Watertight AT the crease point: each wall's crease meets each rim at
/// one point, `(0, ±1, z)`, and the wall and the cap emit the identical
/// mesh vertex there — the rim's chord point at its joint angle, one id
/// both faces read.
#[test]
fn wall_and_cap_share_the_vertex_where_the_crease_meets_the_rim() {
    let body = lofted_cylinder(1.0, Tol::witness());
    let mesh = mesh::tessellate(&body, 1e-2, Tol::witness()).unwrap();
    let wall_faces: Vec<_> = walls(&body, &mesh).iter().map(|p| p.face).collect();
    for (y, z) in [(1.0, 0.0), (-1.0, 0.0), (1.0, 1.0), (-1.0, 1.0)] {
        let at: Vec<u32> = (0..mesh.positions.len())
            .filter(|&i| {
                let p = mesh.positions[i];
                p.x.abs() <= 1e-12 && (p.y - y).abs() <= 1e-12 && (p.z - z).abs() <= 1e-12
            })
            .map(|i| i as u32)
            .collect();
        assert_eq!(
            at.len(),
            1,
            "({y}, {z}): one mesh vertex at the crease point, got {at:?}"
        );
        let users: Vec<bool> = mesh
            .patches
            .iter()
            .filter(|p| p.triangles.iter().any(|t| t.contains(&at[0])))
            .map(|p| wall_faces.contains(&p.face))
            .collect();
        assert!(
            users.contains(&true) && users.contains(&false),
            "({y}, {z}): the crease point is used by a wall and a cap, got wall flags {users:?}"
        );
    }
}

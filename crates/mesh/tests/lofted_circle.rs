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
/// The refusal rows below take it through twenty δ.
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

/// A straight loft of a circle of radius `r0` at `z = 0` to one of
/// radius `r1` at `z = 1`, each authored as `arcs` equal arcs (bulge
/// `tan(π/(2·arcs))`), skinned at degree 1. Two arcs is a semicircle per
/// wall, which carries a crease (as [`lofted_cylinder`]); four or more
/// carry none.
fn lofted_round(r0: f64, r1: f64, arcs: usize) -> topo::Body<f64> {
    let tau = core::f64::consts::TAU;
    let bulge = (tau / (4.0 * arcs as f64)).tan();
    let section = |r: f64| {
        vec![profile::test_support::bulge_loop(
            (0..arcs)
                .map(|k| {
                    let a = tau * k as f64 / arcs as f64;
                    (geom_core::Point2::new(r * a.cos(), r * a.sin()), bulge)
                })
                .collect(),
        )]
    };
    sweep::loft_body::<f64>(
        &[section(r0), section(r1)],
        &sweep::test_support::stacked_at(&[0.0, 1.0]),
        1,
        Tol::witness(),
    )
    .expect("the round lofts")
    .body
}

/// The review's twenty δ of `lofted-rational-walls-refuse-at-some-deltas`:
/// log-spaced from 5e-2 down to 3e-4.
fn review_deltas() -> impl Iterator<Item = f64> {
    let (hi, lo) = (5e-2_f64, 3e-4_f64);
    (0..20).map(move |k| hi * (lo / hi).powf(k as f64 / 19.0))
}

/// Every δ meshes, watertight and at the solid's volume, or the row
/// names each δ that refused.
fn meshes_at_every_review_delta(name: &str, body: &topo::Body<f64>, volume: f64) {
    let mut refused = Vec::new();
    for delta in review_deltas() {
        match mesh::tessellate(body, delta, Tol::witness()) {
            Ok(mesh) => {
                check_mesh(&mesh).unwrap_or_else(|e| panic!("{name} at δ = {delta:e}: {e:?}"));
                let v = signed_volume(&mesh);
                assert!(
                    ((v - volume) / volume).abs() < 3.0 * delta,
                    "{name} at δ = {delta:e}: mesh volume {v} vs {volume}"
                );
            }
            Err(e) => refused.push(format!("δ = {delta:.3e}: {e:?}")),
        }
    }
    assert!(
        refused.is_empty(),
        "{name} refuses at {} of 20 δ:\n  {}",
        refused.len(),
        refused.join("\n  ")
    );
}

/// **The refusal rows** of `lofted-rational-walls-refuse-at-some-deltas`.
/// In raw UV a ruled wall's strip is about 30 times denser in `u` than
/// in `v`, so a seam chord point between two rows fanned a sliver
/// across a dozen columns, and centroid refinement walked it rather
/// than removing it: the creased frustum refused at 9 of the 10 δ
/// ≤ 3.4e-3, the cylinder at 5 of 20. The trimmed lane now runs its CDT
/// in the face's lattice metric (`mesh::nurbs_cert::lattice_metric`),
/// where the strip is near square.
#[test]
fn a_creased_round_loft_meshes_at_every_review_delta() {
    let pi = core::f64::consts::PI;
    meshes_at_every_review_delta(
        "frustum, two arcs",
        &lofted_round(1.0, 0.5, 2),
        pi * 1.75 / 3.0,
    );
    meshes_at_every_review_delta(
        "cylinder, two arcs",
        &lofted_cylinder(1.0, Tol::witness()),
        pi,
    );
}

/// The controls: the same frustum and cylinder with no crease (four
/// and eight arcs per circle). Four arcs also refused at some δ in raw
/// UV, so they are refusal rows of the same class as well.
#[test]
fn crease_free_round_lofts_mesh_at_every_review_delta() {
    let pi = core::f64::consts::PI;
    for arcs in [4, 8] {
        meshes_at_every_review_delta(
            &format!("frustum, {arcs} arcs"),
            &lofted_round(1.0, 0.5, arcs),
            pi * 1.75 / 3.0,
        );
        meshes_at_every_review_delta(
            &format!("cylinder, {arcs} arcs"),
            &lofted_round(1.0, 1.0, arcs),
            pi,
        );
    }
}

/// A degree-2 loft of four-arc circles of radii `r` through stations at
/// z = 0, `e`, `2e`, 0.5, 1: a first knot span about `1.5e` tall beside
/// spans about 0.25 tall, so its bands' realized aspects spread far
/// further than one face-wide lattice metric can square.
fn uneven_station_round(r: [f64; 5], e: f64) -> topo::Body<f64> {
    let tau = core::f64::consts::TAU;
    let bulge = (tau / 16.0).tan();
    let section = |r: f64| {
        vec![profile::test_support::bulge_loop(
            (0..4)
                .map(|k| {
                    let a = tau * f64::from(k) / 4.0;
                    (geom_core::Point2::new(r * a.cos(), r * a.sin()), bulge)
                })
                .collect(),
        )]
    };
    sweep::loft_body::<f64>(
        &r.map(section),
        &sweep::test_support::stacked_at(&[0.0, e, 2.0 * e, 0.5, 1.0]),
        2,
        Tol::witness(),
    )
    .expect("the uneven-station round lofts")
    .body
}

/// **Bands are not column-aligned, and the outcome stays sound**
/// (`work/tess/the-lower-malign-arm-is-unwitnessed`). The round through
/// stations 0, 0.01, 0.02, 0.5, 1 has bands whose residual aspect over
/// the face's lattice metric lies beyond 5 and below 1/5, the bands a
/// column alignment would project. Each band keeps its own columns, and
/// at every review δ the face is a watertight outward mesh or refuses
/// typed `CertificateExceeded`; anything else is red. Measured: all
/// twenty mesh.
#[test]
fn an_uneven_station_loft_meshes_or_refuses_typed() {
    let body = uneven_station_round([1.0, 0.98, 0.96, 0.8, 0.5], 0.01);
    let mut meshed = 0;
    for delta in review_deltas() {
        match mesh::tessellate(&body, delta, Tol::witness()) {
            Ok(mesh) => {
                check_mesh(&mesh).unwrap_or_else(|e| panic!("δ = {delta:e}: {e:?}"));
                assert!(signed_volume(&mesh) > 0.0, "δ = {delta:e}: outward");
                meshed += 1;
            }
            Err(mesh::TessellateError::CertificateExceeded { .. }) => {}
            Err(e) => panic!("δ = {delta:e}: an untyped or other refusal: {e:?}"),
        }
    }
    assert!(
        meshed > 0,
        "the uneven-station round meshes at some review δ"
    );
}

/// **A band too narrow-columned for its rows takes rows**
/// (`work/tess/cylinder-lofted-through-uneven-stations-refuses-at-fine-delta`).
/// The same stations on a constant-radius cylinder spread their bands'
/// aspects 166:1 (56 at the thin first span, 0.34 at the thick ones), and
/// the one face-wide metric leaves the thick bands at residual 0.085: a
/// trim point between two of their rows fans a sliver the certificate
/// bounds above δ, and the face refused typed at four review δ. Raised to
/// the residual line (`nurbs_cert::MAX_RESIDUAL_SQ`), the thick bands
/// carry enough rows that every δ meshes, watertight, at π within the
/// chordal sag. The refusal itself stays the backstop, pinned at the
/// verdict (`trimmed`'s `a_certificate_above_delta_or_nan_does_not_certify`).
#[test]
fn an_uneven_station_cylinder_meshes_at_every_review_delta() {
    let body = uneven_station_round([1.0; 5], 0.01);
    let pi = core::f64::consts::PI;
    for delta in review_deltas() {
        let mesh = mesh::tessellate(&body, delta, Tol::witness())
            .unwrap_or_else(|e| panic!("δ = {delta:e}: the uneven-station cylinder meshes: {e:?}"));
        check_mesh(&mesh).unwrap_or_else(|e| panic!("δ = {delta:e}: {e:?}"));
        let v = signed_volume(&mesh);
        assert!(
            ((v - pi) / pi).abs() < 3.0 * delta,
            "δ = {delta:e}: mesh volume {v} vs π"
        );
    }
}

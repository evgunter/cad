//! **The trimmed-NURBS tessellation lane, end to end** (M7, the
//! montage skin-scenes unit): described non-rational NURBS faces —
//! the loft/sweep walls — route through `trimmed` with the
//! hull-derived Hessian certificate (`nurbs_cert`), and the meshes
//! are watertight, volume-sane, deterministic, and inside the δ + ε
//! promise BY MEASUREMENT.
//!
//! The bodies are the #212 corpus fixtures' constructions, constant
//! for constant (`step-export/tests/common/mod.rs::{loft_prism,
//! swept_elbow}`; `sweep/tests/m7_skin_integral.rs`'s elbow): the
//! degree-1×2 and 1×3 wall classes the lane was promoted for.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::Surface;
use geom_core::{Affine3, Point2, Point3, Tolerance, Vec3};
use mesh::cert::dist_point_triangle;
use mesh::validate::{check_mesh, signed_volume, triangle_count};
use sweep::{SketchSegment, loft_body, segment_curve, sweep_body};
use topo::Body;

mod common;
use common::quad;
use geom_core::Tol;

/// The `loft_prism` corpus body (#212): squares at z = 0 and 2, the
/// non-affine trapezoid at z = 1, v-degree 2 — walls degree 1×2,
/// derived V = 9 m³ exactly (`sweep/tests/m6_loft_body.rs`).
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
    loft_body::<f64>(&sections, &places, 2, Tol::witness())
        .expect("shape (iii) loft builds")
        .body
}

/// The `swept_elbow` corpus body (#212 / #210): a 0.5 m square carried
/// through a 90° arc of radius 3 at nine stations, v-degree 3 — walls
/// degree 1×3; Pappus volume of the limiting quarter torus
/// (2h)²·R·π/2.
fn swept_elbow() -> Body<f64> {
    let (r, h) = (3.0, 0.25);
    let path = segment_curve(
        0,
        SketchSegment::Arc {
            a: Point2::new(0.0, 0.0),
            b: Point2::new(r, r),
            bulge: (core::f64::consts::PI / 8.0).tan(),
        },
        Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            -FRAC_PI_2,
        ),
    )
    .expect("the elbow path is a well-formed quarter arc");
    sweep_body::<f64>(
        &quad([(-h, -h), (h, -h), (h, h), (-h, h)]),
        Affine3::identity(),
        &path,
        9,
        3,
        Tol::witness(),
    )
    .expect("the curved-path sweep body builds")
    .body
}

#[test]
fn loft_prism_tessellates_watertight_and_volume_sane() {
    let body = loft_prism();
    let mesh = mesh::tessellate(&body, 6e-3, Tol::witness()).expect("the NURBS-walled loft tessellates");
    check_mesh(&mesh).expect("watertight, manifold, outward");
    let v = signed_volume(&mesh);
    // Derived exact volume 9 m³ (m6_loft_body.rs); the chordal mesh
    // sits within a fraction of a percent at δ = 6e-3.
    assert!(
        ((v - 9.0) / 9.0).abs() < 5e-3,
        "mesh volume {v} vs derived 9.0"
    );
}

#[test]
fn swept_elbow_tessellates_watertight_and_volume_sane() {
    let body = swept_elbow();
    let mesh = mesh::tessellate(&body, 3e-3, Tol::witness()).expect("the swept elbow tessellates");
    check_mesh(&mesh).expect("watertight, manifold, outward");
    let v = signed_volume(&mesh);
    let pappus = 0.5 * 0.5 * 3.0 * FRAC_PI_2;
    // The walls interpolate nine stations of circular motion, so the
    // exact solid differs from the quarter torus by ~4e-6 relative
    // (m7_skin_integral.rs); the chordal gap dominates here.
    assert!(
        ((v - pappus) / pappus).abs() < 5e-3,
        "mesh volume {v} vs Pappus {pappus}"
    );
}

/// The δ + ε promise, EXERCISED (D4's tessellation contract): a
/// coarse/fine δ pair on the loft, with the deviation MEASURED —
/// surface samples of every NURBS wall against that wall's own patch —
/// and the certified bound dominating the measurement both times.
#[test]
fn delta_pair_measured_deviation_is_dominated_by_the_promise() {
    let body = loft_prism();
    let eps = Tol::witness().get().eps;
    let coarse = 3e-2;
    let fine = 6e-3;
    let measure = |delta: f64| -> (f64, usize) {
        let mesh = mesh::tessellate(&body, delta, Tol::witness()).expect("tessellates");
        check_mesh(&mesh).expect("watertight");
        let mut worst = 0.0f64;
        for patch in &mesh.patches {
            let face = body.get_face(patch.face).unwrap();
            let Some(Surface::Nurbs(payload)) = body.get_surface(face.surface) else {
                continue;
            };
            let (u0, u1) = payload.knots_u().domain();
            let (v0, v1) = payload.knots_v().domain();
            let n = 24;
            for i in 0..=n {
                for j in 0..=n {
                    let u = u0 + (u1 - u0) * f64::from(i) / f64::from(n);
                    let v = v0 + (v1 - v0) * f64::from(j) / f64::from(n);
                    let p = payload.eval(u, v);
                    let mut best = f64::INFINITY;
                    for t in &patch.triangles {
                        let d = dist_point_triangle(
                            p,
                            mesh.positions[t[0] as usize],
                            mesh.positions[t[1] as usize],
                            mesh.positions[t[2] as usize],
                        );
                        if d < best {
                            best = d;
                        }
                    }
                    worst = worst.max(best);
                }
            }
        }
        (worst, triangle_count(&mesh))
    };
    let (dev_coarse, tris_coarse) = measure(coarse);
    let (dev_fine, tris_fine) = measure(fine);
    // The measurement is real (nonzero) and the certified promise
    // dominates it at both tolerances — honestly δ + ε (crate docs:
    // boundary vertices carry the ≤ ε carrier residual).
    assert!(dev_coarse > 0.0 && dev_fine > 0.0);
    assert!(
        dev_coarse <= coarse + eps,
        "measured {dev_coarse} vs promise {coarse} + ε"
    );
    assert!(
        dev_fine <= fine + eps,
        "measured {dev_fine} vs promise {fine} + ε"
    );
    // The pair is a genuine pair: tightening δ tightens both the mesh
    // and the measured deviation.
    assert!(tris_fine > tris_coarse);
    assert!(dev_fine < dev_coarse);
}

/// Byte-identical output for identical (body, δ) — the D9 determinism
/// contract, on the new lane.
#[test]
fn nurbs_tessellation_is_deterministic() {
    let body = swept_elbow();
    let a = mesh::tessellate(&body, 1e-2, Tol::witness()).expect("tessellates");
    let b = mesh::tessellate(&body, 1e-2, Tol::witness()).expect("tessellates");
    assert_eq!(a.positions.len(), b.positions.len());
    for (pa, pb) in a.positions.iter().zip(&b.positions) {
        assert_eq!(pa.x.to_bits(), pb.x.to_bits());
        assert_eq!(pa.y.to_bits(), pb.y.to_bits());
        assert_eq!(pa.z.to_bits(), pb.z.to_bits());
    }
    for (qa, qb) in a.patches.iter().zip(&b.patches) {
        assert_eq!(qa.triangles, qb.triangles);
    }
}

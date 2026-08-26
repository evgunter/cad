//! M2 PR 6 adversarial review — certificate soundness (assignment 1)
//! and back-reference integrity (assignment 7).
//!
//! Independent exact-distance oracles (derived from scratch, not the
//! crate's or the suite's): every emitted triangle is densely sampled
//! and its true distance to the KEYED face's surface locus must be
//! ≤ δ + ε + rounding. Hard shapes: apex fans, coarse-δ torus,
//! near-pole sphere caps, tall thin cylinder patches, mirror-nappe
//! cones, many-band domes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{axis_y, ball, cone, donut, eps, p2, validated, washer};
use geom::Surface;
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use mesh::tessellate;
use mesh::validate::{check_mesh, signed_volume};
use profile::RawLoop;
use profile::{ProfileLoop, ProfileVertex};
use sweep::{Extrusion, Revolution, extrude, revolve};
use topo::Body;

// ---- independent exact-distance oracles (re-derived) ----------------

/// Distance from p to the full cylinder locus: | |p⊥| − r |.
fn my_dist_cylinder(origin: Point3<f64>, axis: Vec3<f64>, r: f64, p: Point3<f64>) -> f64 {
    let w = p - origin;
    let h = w.dot(axis);
    let perp = w - axis * h;
    (perp.norm() - r).abs()
}

/// Distance from p to the sphere locus: | |p−c| − r |.
fn my_dist_sphere(center: Point3<f64>, r: f64, p: Point3<f64>) -> f64 {
    ((p - center).norm() - r).abs()
}

/// Distance from p to the full double-napped cone locus, computed as
/// the 2-D distance in the (ρ, h) half-plane from (ρ_p, h_p) to the
/// two rays {(s·sinα, ±s·cosα) : s ≥ 0} (independent derivation: the
/// cone is the revolution of those rays; distance to a revolved curve
/// from a point is the in-half-plane distance).
fn my_dist_cone(apex: Point3<f64>, axis: Vec3<f64>, alpha: f64, p: Point3<f64>) -> f64 {
    let w = p - apex;
    let h = w.dot(axis);
    let rho = (w - axis * h).norm();
    let (s, c) = alpha.sin_cos();
    let mut best = (rho * rho + h * h).sqrt(); // the apex
    for dir in [(s, c), (s, -c)] {
        let t = rho * dir.0 + h * dir.1; // projection onto the ray
        if t > 0.0 {
            let dx = rho - t * dir.0;
            let dy = h - t * dir.1;
            best = best.min((dx * dx + dy * dy).sqrt());
        }
    }
    best
}

/// Distance from p to the torus locus: distance in the (ρ, h)
/// half-plane from (ρ_p, h_p) to the minor circle at (R, 0).
fn my_dist_torus(center: Point3<f64>, axis: Vec3<f64>, big_r: f64, r: f64, p: Point3<f64>) -> f64 {
    let w = p - center;
    let h = w.dot(axis);
    let rho = (w - axis * h).norm();
    let dx = rho - big_r;
    (((dx * dx + h * h).sqrt()) - r).abs()
}

fn my_dist(surface: &Surface<f64>, p: Point3<f64>) -> f64 {
    match *surface {
        Surface::Plane { origin, normal, .. } => (p - origin).dot(normal).abs(),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => my_dist_cylinder(origin, axis, radius, p),
        Surface::Sphere { center, radius, .. } => my_dist_sphere(center, radius, p),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => my_dist_cone(apex, axis, half_angle, p),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => my_dist_torus(center, axis, major_radius, minor_radius, p),
        // No closed-form distance oracle for a spline locus or a fit.
        Surface::Nurbs(_) | Surface::Approx(_) => f64::NAN,
    }
}

/// Densely samples every triangle of every patch (barycentric grid,
/// n=8 ⇒ 45 points per triangle) against the KEYED face's surface —
/// the honest promise is δ + ε (+ f64 rounding).
fn hunt_chordal_violation(body: &Body<f64>, delta: f64) {
    let mesh = tessellate(body, delta, Tol::witness()).unwrap();
    assert_eq!(check_mesh(&mesh), Ok(()));
    assert!(signed_volume(&mesh) > 0.0);
    let slack = delta + eps() + 1e-9;
    let n = 8u32;
    let mut worst: f64 = 0.0;
    for patch in &mesh.patches {
        let face = body.get_face(patch.face).expect("face key live");
        let surface = body.get_surface(face.surface).expect("surface key live");
        for tri in &patch.triangles {
            let [a, b, c] = [
                mesh.positions[tri[0] as usize],
                mesh.positions[tri[1] as usize],
                mesh.positions[tri[2] as usize],
            ];
            for i in 0..=n {
                for j in 0..=(n - i) {
                    let (li, lj) = (f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                    let lk = 1.0 - li - lj;
                    let p = Point3::new(
                        a.x * li + b.x * lj + c.x * lk,
                        a.y * li + b.y * lj + c.y * lk,
                        a.z * li + b.z * lj + c.z * lk,
                    );
                    let d = my_dist(surface, p);
                    worst = worst.max(d);
                    assert!(
                        d <= slack,
                        "chordal violation: dist {d} > {slack} (delta {delta}) on {:?}",
                        patch.face
                    );
                }
            }
        }
    }
    assert!(worst.is_finite());
}

// ---- hard bodies ----------------------------------------------------

/// Tall thin cylinder patches: rounded bar 1×0.5, corners r=0.2,
/// extruded to height 25 (nv = 1 makes v-long triangles).
fn tall_thin_bar() -> Body<f64> {
    let b = (core::f64::consts::FRAC_PI_8).tan();
    let r = 0.2;
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(p2(x, y), bulge);
    let mut lp = ProfileLoop::new(vec![
        v(r, 0.0, 0.0),
        v(1.0 - r, 0.0, b),
        v(1.0, r, 0.0),
        v(1.0, 0.5 - r, b),
        v(1.0 - r, 0.5, 0.0),
        v(r, 0.5, b),
        v(0.0, 0.5 - r, 0.0),
        v(0.0, r, b),
    ]);
    // All eight joints are exact corner-arc/side tangencies (#101).
    let n = lp.vertices().len();
    lp = lp.with_tangent_joints((0..n).collect());
    extrude(
        &validated(vec![lp]),
        Extrusion::Distance(25.0),
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Mirror-nappe ring: triangle (1,0)-(2,1)-(1,2) revolved fully —
/// an upward cone wall, a downward (mirror-nappe) cone wall, and a
/// cylinder wall; genus 1, no axis contact.
fn diamond_ring() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(1.0, 0.0), p2(2.0, 1.0), p2(1.0, 2.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Megaphone: very wide cone (half-angle atan 3 ≈ 71.6°) + top disc.
fn megaphone() -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(3.0, 1.0), p2(0.0, 1.0)]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Silo: cylinder wall + quarter-arc dome cap onto the axis pole.
fn silo() -> Body<f64> {
    let b = (core::f64::consts::FRAC_PI_8).tan(); // quarter circle
    let mut lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 0.0), 0.0),
        ProfileVertex::new(p2(1.0, 1.0), b),
        ProfileVertex::new(p2(0.0, 2.0), 0.0),
    ]);
    // The dome cap leaves the cylinder wall tangentially at (1, 1) --
    // intended smooth cap, declared (#101).
    lp = lp.with_tangent_joints(vec![2]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// Many-band dome: three unit-circle arcs pole to pole (stacked
/// sphere zones sharing rims, pole fans at both ends).
fn dome() -> Body<f64> {
    let t = |theta: f64| (theta / 4.0).tan();
    let a1 = 1.0; // radians of arc per band
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(p2(0.0, -1.0), t(a1)),
        ProfileVertex::new(p2(a1.sin(), -a1.cos()), t(a1)),
        ProfileVertex::new(
            p2((2.0 * a1).sin(), -(2.0 * a1).cos()),
            t(core::f64::consts::PI - 2.0 * a1),
        ),
        ProfileVertex::new(p2(0.0, 1.0), 0.0),
    ]);
    revolve(
        &validated(vec![lp]),
        axis_y(),
        Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

// ---- the hunts ------------------------------------------------------

#[test]
fn survives_chordal_hunt_stock_bodies_coarse_and_fine() {
    for body in [ball(), cone(), donut(), washer()] {
        for delta in [0.4, 0.1, 0.02] {
            hunt_chordal_violation(&body, delta);
        }
    }
}

#[test]
fn survives_chordal_hunt_tall_thin_cylinder() {
    let body = tall_thin_bar();
    for delta in [0.45, 0.05] {
        hunt_chordal_violation(&body, delta);
    }
}

#[test]
fn survives_chordal_hunt_mirror_nappe_cones() {
    let body = diamond_ring();
    for delta in [0.5, 0.05, 0.01] {
        hunt_chordal_violation(&body, delta);
    }
}

#[test]
fn survives_chordal_hunt_megaphone_and_silo() {
    for body in [megaphone(), silo()] {
        for delta in [0.5, 0.05] {
            hunt_chordal_violation(&body, delta);
        }
    }
}

#[test]
fn survives_chordal_hunt_many_band_dome() {
    let body = dome();
    for delta in [0.3, 0.03] {
        hunt_chordal_violation(&body, delta);
    }
}

#[test]
fn survives_chordal_hunt_near_pole_cap_fine() {
    // Near-pole sphere caps at fine δ: the pole fan triangles are the
    // certificate's hard case (no UV involvement, exact 3-D bound).
    let body = ball();
    hunt_chordal_violation(&body, 0.004);
}

// ---- back-reference integrity (assignment 7) ------------------------

#[test]
fn survives_backref_integrity_and_patch_separability() {
    let body = diamond_ring();
    let mesh = tessellate(&body, 0.05, Tol::witness()).unwrap();
    // One patch per face, one polyline per edge, in arena order.
    let face_keys: Vec<_> = body.faces().map(|(k, _)| k).collect();
    let patch_keys: Vec<_> = mesh.patches.iter().map(|p| p.face).collect();
    assert_eq!(face_keys, patch_keys);
    let edge_keys: Vec<_> = body.edges().map(|(k, _)| k).collect();
    let bp_keys: Vec<_> = mesh.boundaries.iter().map(|b| b.edge).collect();
    assert_eq!(edge_keys, bp_keys);
    // Polyline endpoints are the vertex points bitwise.
    for bp in &mesh.boundaries {
        let (s, e) = (bp.start_vertex, bp.end_vertex);
        for (vk, idx) in [(s, bp.points[0]), (e, *bp.points.last().unwrap())] {
            let vp = *body.get_point(body.get_vertex(vk).unwrap().point).unwrap();
            let mp = mesh.positions[idx as usize];
            assert_eq!(vp.x.to_bits(), mp.x.to_bits());
            assert_eq!(vp.y.to_bits(), mp.y.to_bits());
            assert_eq!(vp.z.to_bits(), mp.z.to_bits());
        }
    }
    // Separability: patches are owned values — mutating a clone of one
    // patch cannot affect another (no shared mutable structure).
    let mut m2 = mesh.clone();
    let before = mesh.patches[1].triangles.clone();
    m2.patches[0].triangles.clear();
    assert_eq!(mesh.patches[1].triangles, before);
    assert!(!mesh.patches[0].triangles.is_empty());
}

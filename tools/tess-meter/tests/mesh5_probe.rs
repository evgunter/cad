//! The issue 685 measurement instrument: the π/6 cone wedge δ-sweep
//! through the budget instrument, printing per-δ element counts,
//! watertightness, densely sampled deviation, and the budget CSV rows
//! — plus the class's sibling cases (sphere `nu == 1`, torus
//! `nu == 1`, cone `nv == 1`). A green reporter in the `r2_bytes.rs`
//! shape: run it at two revisions and diff the tables. The decision
//! it fed — one azimuth column takes no rows — lives at
//! `mesh::curved::grid_counts`' cone arm; the pinned rows live in
//! `mesh/tests/issue685_nu1_sizing.rs`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::print_stdout
)]

use geom::Surface;
use geom_core::{Point2, Point3, Tol, Vec2};
use mesh::budget::{self, Mode};
use mesh::validate::check_mesh;
use profile::{Profile, ProfileLoop, RawLoop as _, SketchPlane};
use sweep::{Revolution, RevolveAxis, revolve};
use tess_meter::face_rows;
use topo::Body;

fn cone_wedge(s: f64, theta: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(s, 0.0),
        Point2::new(0.0, 1.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    revolve(&profile, axis, Revolution::Partial(theta), Tol::witness())
        .unwrap()
        .body
}

/// Exact distance to the complete cone locus (both nappes; apex
/// fallback) — the acceptance suite's own oracle, inlined.
fn dist_to_cone(
    apex: Point3<f64>,
    axis: geom_core::Vec3<f64>,
    half_angle: f64,
    p: Point3<f64>,
) -> f64 {
    let w = p - apex;
    let h = w.dot(axis);
    let rho = (w - axis * h).norm();
    let (s, c) = half_angle.sin_cos();
    let mut best = f64::INFINITY;
    for hh in [h, -h] {
        if rho * s + hh * c >= 0.0 {
            best = best.min((rho * c - hh * s).abs());
        }
    }
    best.min(w.norm())
}

/// The cone arm's schedule, re-derived for the REPORT ONLY (the
/// production spelling is `mesh::curved::grid_counts`, private):
/// hu = sagitta_step(delta_s, rho_max) capped at pi/4;
/// nu_raw = ceil(uspan/hu); nv = ceil(vspan/(rho_max*hu)).
fn schedule(delta: f64, rho_max: f64, uspan: f64, vspan: f64) -> (usize, usize) {
    let ds = delta * 0.5;
    let cap = core::f64::consts::FRAC_PI_4;
    let hu = if ds < rho_max {
        let h = 2.0 * (1.0 - ds / rho_max).acos();
        if h < cap { h } else { cap }
    } else {
        cap
    };
    let nu = (uspan / hu).ceil().max(1.0) as usize;
    let nv = (vspan / (rho_max * hu)).ceil().max(1.0) as usize;
    (nu, nv)
}

/// Exact distance to the complete analytic locus (the acceptance
/// suite's oracle, inlined).
fn dist_to_surface(surface: &Surface<f64>, p: Point3<f64>) -> f64 {
    match *surface {
        Surface::Plane { origin, normal, .. } => (p - origin).dot(normal).abs(),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let w = p - origin;
            ((w - axis * w.dot(axis)).norm() - radius).abs()
        }
        Surface::Sphere { center, radius, .. } => ((p - center).norm() - radius).abs(),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => dist_to_cone(apex, axis, half_angle, p),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let w = p - center;
            let h = w.dot(axis);
            let rho = (w - axis * h).norm();
            (((rho - major_radius).powi(2) + h * h).sqrt() - minor_radius).abs()
        }
        Surface::Nurbs(_) | Surface::Approx(_) => f64::NAN,
    }
}

/// Per-curved-patch report: (tris, max densely sampled deviation).
fn curved_report(body: &Body<f64>, mesh: &mesh::Mesh) -> (usize, f64) {
    let mut tris = 0usize;
    let mut max_dev = 0.0_f64;
    for patch in &mesh.patches {
        let face = body.get_face(patch.face).unwrap();
        let surface = body.get_surface(face.surface).unwrap();
        if matches!(surface, Surface::Plane { .. }) {
            continue;
        }
        tris += patch.triangles.len();
        for t in &patch.triangles {
            let [a, b, c] = [
                mesh.positions[t[0] as usize],
                mesh.positions[t[1] as usize],
                mesh.positions[t[2] as usize],
            ];
            let n = 8u32;
            for i in 0..=n {
                for j in 0..=(n - i) {
                    let k = n - i - j;
                    let (li, lj, lk) = (
                        f64::from(i) / f64::from(n),
                        f64::from(j) / f64::from(n),
                        f64::from(k) / f64::from(n),
                    );
                    let p = Point3::origin()
                        + (a - Point3::origin()) * li
                        + (b - Point3::origin()) * lj
                        + (c - Point3::origin()) * lk;
                    max_dev = max_dev.max(dist_to_surface(surface, p));
                }
            }
        }
    }
    (tris, max_dev)
}

/// The class-sweep siblings: sphere nu == 1, torus nu == 1, cone
/// nv == 1 — same empty-range grid, certificate as the backstop.
#[test]
fn mesh5_sibling_cases() {
    // Sphere wedge theta = 0.3 (nu = 1 at delta in {0.1, 0.05}).
    let half = ProfileLoop::new(vec![
        profile::ProfileVertex::new(Point2::new(0.0, -1.0), 1.0),
        profile::ProfileVertex::new(Point2::new(0.0, 1.0), 0.0),
    ]);
    let sphere = revolve(
        &Profile::new(SketchPlane::xy(), vec![half])
            .validate(Tol::witness())
            .unwrap(),
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(0.3),
        Tol::witness(),
    )
    .unwrap()
    .body;
    // Torus wedge theta = 0.05 (nu = 1 at delta = 0.1).
    let arc = ProfileLoop::new(vec![
        profile::ProfileVertex::new(Point2::new(2.0, -0.5), 1.0),
        profile::ProfileVertex::new(Point2::new(2.0, 0.5), 1.0),
    ]);
    let torus = revolve(
        &Profile::new(SketchPlane::xy(), vec![arc])
            .validate(Tol::witness())
            .unwrap(),
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(0.05),
        Tol::witness(),
    )
    .unwrap()
    .body;
    // Cone nv == 1 mirror: short slant, wide azimuth (nu = 3, nv = 1
    // at delta = 0.1: rho_max = 1.2, hu ~= 0.578, hv ~= 0.694 > vspan
    // ~= 0.283).
    let band = ProfileLoop::polygon([
        Point2::new(1.0, 0.0),
        Point2::new(1.2, 0.0),
        Point2::new(1.0, 0.2),
    ]);
    let cone_band = revolve(
        &Profile::new(SketchPlane::xy(), vec![band])
            .validate(Tol::witness())
            .unwrap(),
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
    .unwrap()
    .body;
    println!("MESH-5 siblings: curved-patch tris / watertight / max_dev vs delta");
    for (name, body, deltas) in [
        ("sphere_wedge(0.3)", &sphere, [0.1, 0.05].as_slice()),
        ("torus_wedge(0.05)", &torus, [0.1].as_slice()),
        ("cone_band nv==1", &cone_band, [0.1].as_slice()),
    ] {
        for &delta in deltas {
            match mesh::tessellate(body, delta, Tol::witness()) {
                Ok(mesh) => {
                    let wt = check_mesh(&mesh).is_ok();
                    let (tris, max_dev) = curved_report(body, &mesh);
                    println!(
                        "{name:>20} d={delta:<6} tris={tris:<5} watertight={wt} \
                         max_dev={max_dev:.3e} dev/delta={:.3}",
                        max_dev / delta
                    );
                }
                Err(e) => println!("{name:>20} d={delta:<6} REFUSED {e:?}"),
            }
        }
    }
}

#[test]
fn mesh5_cone_wedge_sweep() {
    let theta = core::f64::consts::FRAC_PI_6;
    let body = cone_wedge(1.0, theta);
    let sqrt2 = 2.0_f64.sqrt();
    println!("MESH-5 sweep: cone_wedge(1.0, pi/6); rho_max=1, uspan=pi/6, vspan=sqrt(2)");
    println!(
        "{:>8} {:>10} {:>9} {:>9} {:>10} {:>12} {:>9}",
        "delta", "(nu,nv)raw", "cone_tris", "all_tris", "watertight", "max_dev", "dev/delta"
    );
    let mut csv = vec![tess_meter::CSV_HEADER.to_string()];
    for &delta in &[0.25, 0.1, 0.07, 0.0682, 0.05, 0.025, 0.01, 0.004, 0.001] {
        budget::arm(Mode::Deviation {
            samples_per_edge: 6,
        });
        let mesh = mesh::tessellate(&body, delta, Tol::witness()).unwrap();
        let measures = budget::take();
        let rows = face_rows(delta, &body, &mesh, &measures);
        for r in &rows {
            csv.push(r.csv_row(&format!("cone_wedge_pi6_d{delta}")));
        }
        let wt = check_mesh(&mesh).is_ok();
        let total: usize = mesh.patches.iter().map(|p| p.triangles.len()).sum();
        // The cone patch and its dense sampled deviation.
        let mut cone_tris = 0usize;
        let mut max_dev = 0.0_f64;
        for patch in &mesh.patches {
            let face = body.get_face(patch.face).unwrap();
            let Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            } = *body.get_surface(face.surface).unwrap()
            else {
                continue;
            };
            cone_tris += patch.triangles.len();
            for t in &patch.triangles {
                let [a, b, c] = [
                    mesh.positions[t[0] as usize],
                    mesh.positions[t[1] as usize],
                    mesh.positions[t[2] as usize],
                ];
                let n = 8u32;
                for i in 0..=n {
                    for j in 0..=(n - i) {
                        let k = n - i - j;
                        let (li, lj, lk) = (
                            f64::from(i) / f64::from(n),
                            f64::from(j) / f64::from(n),
                            f64::from(k) / f64::from(n),
                        );
                        let p = Point3::origin()
                            + (a - Point3::origin()) * li
                            + (b - Point3::origin()) * lj
                            + (c - Point3::origin()) * lk;
                        max_dev = max_dev.max(dist_to_cone(apex, axis, half_angle, p));
                    }
                }
            }
        }
        let (nu, nv) = schedule(delta, 1.0, theta, sqrt2);
        println!(
            "{:>8} {:>10} {:>9} {:>9} {:>10} {:>12.3e} {:>9.3}",
            delta,
            format!("({nu},{nv})"),
            cone_tris,
            total,
            wt,
            max_dev,
            max_dev / delta
        );
    }
    println!("--- budget rows ---");
    for line in csv {
        println!("{line}");
    }
}

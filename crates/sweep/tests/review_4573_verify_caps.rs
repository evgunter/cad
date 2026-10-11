//! Verifier probe for PR 4573's fix pass: R1's thin-cap precision
//! probe extended to h/R = 1e-6, axial and tilted chart, run on the
//! head and on main at the merge base.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::panic)]

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled, brick, finished};

#[test]
fn verify_4573_thin_cap_precision() {
    let tol = Tol::witness();
    let none = topo::BooleanDeclarations::none();
    let pi = core::f64::consts::PI;
    for (pname, p, tilt) in [
        ("axial", Vec3::new(0.0, 0.0, 1.0), 0.0),
        ("tilted", Vec3::new(0.3, 0.2, 0.93), 0.0),
        ("side", Vec3::new(1.0, 0.0, 0.0), 0.0),
        ("rotslab", Vec3::new(0.0, 0.0, 1.0), 0.37),
        ("rotslab-tilted", Vec3::new(0.3, 0.2, 0.93), 0.37),
    ] {
        let ball = finished("ball", ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), p, tol), tol);
        for h0 in [1e-4_f64, 3e-5, 1e-5, 3e-6, 1e-6] {
            let slab = brick((-2.0, 2.0), (-2.0, 2.0), (1.0 - h0, 3.0), tol);
            let slab = if tilt == 0.0 {
                slab
            } else {
                topo::transform_rigid(
                    &slab,
                    &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), tilt),
                    tol,
                )
                .unwrap()
            };
            let slab = finished("slab", slab, tol);
            let h = 1.0 - (1.0 - h0);
            let ev = pi * h * h * (3.0 - h) / 3.0;
            let ea = 2.0 * pi * h + pi * (2.0 * h - h * h);
            let sphere_a = 2.0 * pi * h;
            match topo::intersect_with(&slab, &ball, &none, tol) {
                Ok(r) => {
                    let body = r.body().unwrap().body.clone();
                    match topo::mass_properties(&body, tol) {
                        Ok(mp) => println!(
                            "CAP {pname:6} h={h0:.0e}: vol rel {:+.3e} (x eps/h {:+.2}) area rel {:+.3e} (x eps {:+.2}; sphere-part GB-only would be ~{:.1e})",
                            (mp.volume - ev) / ev,
                            (mp.volume - ev) / ev / (f64::EPSILON / h),
                            (mp.surface_area - ea) / ea,
                            (mp.surface_area - ea) / ea / f64::EPSILON,
                            f64::EPSILON * 2.0 * pi / sphere_a,
                        ),
                        Err(e) => println!("CAP {pname:6} h={h0:.0e}: mp {e:?}"),
                    }
                }
                Err(e) => println!("CAP {pname:6} h={h0:.0e}: bool {e:?}"),
            }
        }
    }
}

/// Why the rotated thin cap's area sits at Gauss–Bonnet's rounding:
/// the cap face's own edges, their residuals, and the face re-measured
/// through the public door with the carriers re-projected onto the
/// sphere.
#[test]
fn verify_4573_rotated_cap_edges() {
    use geom::{Curve3, Surface};
    let tol = Tol::witness();
    let none = topo::BooleanDeclarations::none();
    let band = geom_core::Band::linear(tol).unwrap();
    let pi = core::f64::consts::PI;
    let ball = finished("ball", ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), tol), tol);
    for (tilt, h0) in [(0.0, 1e-5_f64), (0.37, 1e-4), (0.37, 3e-5), (0.37, 1e-5), (0.37, 1e-6), (1e-3, 1e-5), (1e-6, 1e-5)] {
        let slab = brick((-2.0, 2.0), (-2.0, 2.0), (1.0 - h0, 3.0), tol);
        let slab = if tilt == 0.0 {
            slab
        } else {
            topo::transform_rigid(
                &slab,
                &Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), tilt),
                tol,
            )
            .unwrap()
        };
        let slab = finished("slab", slab, tol);
        let body = match topo::intersect_with(&slab, &ball, &none, tol) {
            Ok(r) => r.body().unwrap().body.clone(),
            Err(e) => {
                println!("ROT tilt={tilt} h={h0:e}: bool {e:?}");
                continue;
            }
        };
        let h = 1.0 - (1.0 - h0);
        let (mut sum, mut sum_fixed) = (0.0, 0.0);
        let mut want = 2.0 * pi * h;
        // The cap as the stored rim states it: `2π(1 − d)`, `1 − d = ρ²/(1 + d)`.
        for (_, f) in body.faces() {
            let s = body.get_surface(f.surface).unwrap().clone();
            let Surface::Sphere { center, .. } = s else { continue };
            let (edges, _) = topo::props::loop_edges(&body, f.outer).unwrap();
            for e in &edges {
                if let Curve3::Circle { center: cc, axis, radius: rc, .. } = e.carrier {
                    if (e.t1 - e.t0) > 3.0 {
                        let d = (cc - center).dot(axis) / axis.norm();
                        let reach = (d * d + rc * rc).sqrt();
                        let d = d / reach;
                        want = 2.0 * pi * (rc * rc / (reach * reach)) / (1.0 + d.abs());
                    }
                }
            }
        }
        for (_, f) in body.faces() {
            let s = body.get_surface(f.surface).unwrap().clone();
            let Surface::Sphere { center, radius, .. } = s else { continue };
            let (edges, _) = topo::props::loop_edges(&body, f.outer).unwrap();
            let got = geom_brep::props::curved_face_loops(&s, &[&edges[..]], f.sense, band).unwrap();
            sum += got.area;
            println!("ROT tilt={tilt} h={h0:e}: {} arcs, sense {}, area rel {:+.3e} (abs sr {:+.2e})", edges.len(), f.sense, (got.area - want) / want, got.area - want);
            let mut fixed = edges.clone();
            for e in &edges {
                if let Curve3::Circle { center: cc, axis, radius: rc, u_ref } = e.carrier {
                    let w = cc - center;
                    println!(
                        "   arc span {:.6} fwd {} | fit {:+.2e} on {:.2e} |n|-1 {:+.2e} u.n {:+.2e} |u|-1 {:+.2e}",
                        e.t1 - e.t0,
                        e.forward,
                        (w.norm_squared() + rc * rc).sqrt() - radius,
                        w.cross(axis).norm(),
                        axis.norm() - 1.0,
                        u_ref.dot(axis),
                        u_ref.norm() - 1.0
                    );
                }
            }
            // Re-project: unit axis, centre on the radial line, radius on the sphere.
            for e in &mut fixed {
                if let Curve3::Circle { center: cc, axis, radius: rc, u_ref } = e.carrier {
                    let n = axis / axis.norm();
                    let off = (cc - center).dot(n);
                    let rho = (radius * radius - off * off).sqrt();
                    let u = u_ref - n * u_ref.dot(n);
                    let u = u / u.norm();
                    let _ = rc;
                    e.carrier = Curve3::Circle { center: center + n * off, axis: n, radius: rho, u_ref: u };
                }
            }
            match geom_brep::props::curved_face_loops(&s, &[&fixed[..]], f.sense, band) {
                Ok(g) => sum_fixed += g.area,
                Err(e) => println!("   re-projected: {e:?}"),
            }
        }
        println!("SUM tilt={tilt} h={h0:e}: sphere area rel {:+.3e} (GB scale {:.1e}); re-projected {:+.3e}", (sum - want) / want, 2.0 * f64::EPSILON * 2.0 * pi / want, (sum_fixed - want) / want);
    }
}

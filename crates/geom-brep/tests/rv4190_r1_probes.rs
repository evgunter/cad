//! Review probes for PR 4190 (lane r1): near-tangent configurations on
//! both SSI lanes, run on the PR head and on main for a differential.
//! Output lines start `R1PROBE`; nothing here asserts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn binom(n: usize, k: usize) -> f64 {
    let mut r = 1.0;
    for i in 0..k {
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r
}

/// Power-basis coefficients (degree <= n) to Bernstein degree n.
fn to_bernstein(a: &[f64], n: usize) -> Vec<f64> {
    (0..=n)
        .map(|k| {
            (0..=k.min(a.len() - 1))
                .map(|i| binom(k, i) / binom(n, i) * a[i])
                .sum()
        })
        .collect()
}

fn mul(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut r = vec![0.0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            r[i + j] += x * y;
        }
    }
    r
}

/// The wall `z = x²/(2ρ)` over `y ∈ [0, 0.02]`, its `u` chart
/// `x = w·(−1 + 2(1+μ)u − 2μu²)` (μ = 0 is uniform), degree 4 × 1.
fn parabola_wall(rho: f64, w: f64, mu: f64) -> NurbsSurface<f64> {
    let x = [-w, 2.0 * (1.0 + mu) * w, -2.0 * mu * w];
    let z: Vec<f64> = mul(&x, &x).iter().map(|c| c / (2.0 * rho)).collect();
    let n = 4;
    let (xb, zb) = (to_bernstein(&x, n), to_bernstein(&z, n));
    let ku = KnotVector::clamped(vec![0.0; 5].into_iter().chain([1.0; 5]).collect(), n).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::new();
    for i in 0..=n {
        for j in 0..=1 {
            control.push(Point3::new(xb[i], 0.02 * j as f64, zb[i]));
        }
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 10]).unwrap()
}

/// The near-tangent parabola past the filed row's `d ≤ 100ε`: two
/// lines at `x = ±d`, the pose `δ = d²/(2ρ)` from tangent, swept so
/// that δ crosses the band. `μ` bends the chart (main's chart arm
/// shrinks; the shape-operator arm does not).
#[test]
fn r1_near_tangent_parabola_sweep() {
    let eps = band().zero();
    let rho = 1e-3;
    let w = 0.01;
    let mus: Vec<f64> = std::env::var("R1_MU")
        .ok()
        .map(|v| v.split(',').map(|s| s.parse().unwrap()).collect())
        .unwrap_or_else(|| vec![0.0, 0.9]);
    let ks: Vec<f64> = std::env::var("R1_K")
        .ok()
        .map(|v| v.split(',').map(|s| s.parse().unwrap()).collect())
        .unwrap_or_else(|| vec![300.0, 1000.0, 1500.0, 2000.0, 3000.0, 4500.0, 7000.0]);
    for &mu in &mus {
        let wall = parabola_wall(rho, w, mu);
        for &k in &ks {
            let d = k * eps;
            let delta = d * d / (2.0 * rho);
            let plane = Surface::Plane {
                origin: Point3::new(0.0, 0.0, delta),
                normal: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let domain = SsiDomain {
                center: Point3::new(0.0, 0.01, 0.0),
                half_extent: 1.0,
                extent: 0.02,
                floor_scale: 1.0,
            };
            let tag = format!(
                "parabola mu={mu} d={k}eps delta/eps={:.3} sin*rho={:.2e} eps={eps:e}",
                delta / eps,
                d
            );
            match ssi::plane_nurbs_ssi(&plane, &wall, domain, band()) {
                Ok(out) => {
                    let desc: Vec<String> = out
                        .branches
                        .iter()
                        .map(|b| {
                            let (p, q) = (b.carrier.eval(b.params.0), b.carrier.eval(b.params.1));
                            format!(
                                "x {:.3e}->{:.3e} y {:.2e}->{:.2e} tubeT {:.2e} {:?}",
                                p.x, q.x, p.y, q.y, b.certificate.tube_transversality, b.certificate.tube
                            )
                        })
                        .collect();
                    println!("R1PROBE {tag}: OK {} {desc:?}", out.branches.len());
                }
                Err(e) => println!("R1PROBE {tag}: ERR {e}"),
            }
        }
    }
}

/// Cylinder × sphere near the external graze: the sphere penetrates the
/// cylinder by `g`, so the locus is a small loop about the near
/// contact. The pose gate reads `g`; main's tube levered by the sphere
/// radius, the PR's by the extent.
#[test]
fn r1_cylinder_sphere_graze_sweep() {
    let eps = band().zero();
    let (r, big_r) = (0.08, 0.02);
    let ks: Vec<f64> = std::env::var("R1_G")
        .ok()
        .map(|v| v.split(',').map(|s| s.parse().unwrap()).collect())
        .unwrap_or_else(|| vec![20.0, 100.0, 1e3, 1e4, 1e5, 1e6]);
    for internal in [false, true] {
        for &k in &ks {
            let g = k * eps;
            // External: d − r − R = −g. Internal (cylinder inside the
            // sphere, touching off-axis): d + r − R = −g, R > r.
            let (rr, d) = if internal {
                (0.2, 0.2 - r - g)
            } else {
                (big_r, r + big_r - g)
            };
            let cyl = Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: r,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let sph = Surface::Sphere {
                center: Point3::new(d, 0.0, 0.0),
                radius: rr,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let dom = SsiDomain {
                center: Point3::new(0.0, 0.0, 0.0),
                half_extent: 0.5,
                extent: 2.0,
                floor_scale: 1.0,
            };
            let tag = format!("cs internal={internal} g={k}eps eps={eps:e}");
            match ssi::cylinder_sphere_ssi(&cyl, &sph, dom, band()) {
                Ok(out) => {
                    let desc: Vec<String> = out
                        .branches
                        .iter()
                        .map(|b| {
                            format!(
                                "tubeT {:.3e} tube {:?} samples {}",
                                b.certificate.tube_transversality,
                                b.certificate.tube,
                                b.certificate.samples
                            )
                        })
                        .collect();
                    println!("R1PROBE {tag}: OK {} {desc:?}", out.branches.len());
                }
                Err(e) => println!("R1PROBE {tag}: ERR {e}"),
            }
        }
    }
}

/// A quarter cylinder of radius `rho` (`z ∈ [0, h]`), its arc's weights
/// Möbius-rescaled by `c` (`1, c·√2/2, c²`): the same surface, a chart
/// that runs `c²` times faster at one end.
fn quarter_cylinder(rho: f64, h: f64, c: f64) -> NurbsSurface<f64> {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let k2 = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let k1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(rho, 0.0, 0.0),
        Point3::new(rho, 0.0, h),
        Point3::new(rho, rho, 0.0),
        Point3::new(rho, rho, h),
        Point3::new(0.0, rho, 0.0),
        Point3::new(0.0, rho, h),
    ];
    let w = vec![1.0, 1.0, c * s, c * s, c * c, c * c];
    NurbsSurface::new(k2, k1, control, w).unwrap()
}

/// A plane through the cylinder's ruling at angle `phi`, turned by `θ`
/// from its tangent plane there: it cuts the rulings at `phi` and
/// `phi + 2θ`, `2ρ sin θ` apart, the pose `ρ(1 − cos θ)` from tangent.
#[test]
fn r1_near_tangent_cylinder_ssi_sweep() {
    let eps = band().zero();
    let rho = 1e-3;
    let ks: Vec<f64> = std::env::var("R1_CK")
        .ok()
        .map(|v| v.split(',').map(|s| s.parse().unwrap()).collect())
        .unwrap_or_else(|| vec![2.0, 5.0, 10.0, 20.0, 50.0, 200.0, 1000.0]);
    for c in [1.0, 10.0, 30.0] {
        let wall = quarter_cylinder(rho, 0.01, c);
        for phi in [0.3_f64, 0.8, 1.2] {
            for &k in &ks {
                let sin = k * eps / rho;
                let th = sin.asin();
                let p = Point3::new(rho * phi.cos(), rho * phi.sin(), 0.0);
                let plane = Surface::Plane {
                    origin: p,
                    normal: Vec3::new((phi + th).cos(), (phi + th).sin(), 0.0),
                    u_ref: Vec3::new(0.0, 0.0, 1.0),
                };
                let dom = SsiDomain {
                    center: Point3::new(0.0, 0.0, 0.005),
                    half_extent: 1.0,
                    extent: 0.01,
                    floor_scale: 1.0,
                };
                let tag = format!(
                    "cyl c={c} phi={phi} sin*rho={k}eps gap/eps={:.2e} eps={eps:e}",
                    rho * (1.0 - th.cos()) / eps
                );
                match ssi::plane_nurbs_ssi(&plane, &wall, dom, band()) {
                    Ok(out) => {
                        let desc: Vec<String> = out
                            .branches
                            .iter()
                            .map(|b| {
                                let (a, z) = (b.carrier.eval(b.params.0), b.carrier.eval(b.params.1));
                                format!(
                                    "ang {:.6e}->{:.6e} z {:.1e}->{:.1e}",
                                    a.y.atan2(a.x) - phi,
                                    z.y.atan2(z.x) - phi,
                                    a.z,
                                    z.z
                                )
                            })
                            .collect();
                        println!("R1PROBE {tag}: OK {} {desc:?} 2th={:.3e}", out.branches.len(), 2.0 * th);
                    }
                    Err(e) => println!("R1PROBE {tag}: ERR {e}"),
                }
            }
        }
    }
}

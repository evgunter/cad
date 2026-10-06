//! Review probes for PR 4190 (lane r2). Print-only: each probe reports
//! the outcome per configuration so main and the PR head can be diffed.


use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain, SsiError, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

fn band_at(eps: f64) -> Band {
    Band::linear_at(Tol::witness(), eps).unwrap()
}

fn label(r: &Result<SsiOutcome, SsiError>) -> String {
    match r {
        Ok(o) => format!(
            "OK branches={} boundary={} tt=[{}]",
            o.branches.len(),
            o.boundary.len(),
            o.branches
                .iter()
                .map(|b| format!("{:.3e}", b.certificate.tube_transversality))
                .collect::<Vec<_>>()
                .join(",")
        ),
        Err(e) => {
            let s = format!("{e:?}");
            let head: String = s.chars().take(60).collect();
            format!("ERR {head}")
        }
    }
}

/// The exact sphere octant of radius `r` as a biquadratic rational
/// surface of revolution: `u` the longitude quarter, `v` the meridian
/// from the equator to the pole (a collapsed row).
fn sphere_octant(r: f64) -> NurbsSurface<f64> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    let lon = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
    let prof = [(r, 0.0), (r, r), (0.0, r)];
    let w = [1.0, h, 1.0];
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            control.push(Point3::new(
                prof[j].0 * lon[i].0,
                prof[j].0 * lon[i].1,
                prof[j].1,
            ));
            weights.push(w[i] * w[j]);
        }
    }
    NurbsSurface::new(k.clone(), k, control, weights).unwrap()
}

/// P1: a plane `δ` below the pole of a sphere octant. The latitude
/// circle it cuts has radius √(2Rδ); `δ < ε` is a sub-ε pose.
#[test]
fn p1_plane_near_the_pole_of_a_rational_sphere() {
    for eps in [1e-6, 1e-9, 1e-12] {
        for r in [1.0, 1e-2] {
            for k in [0.1, 0.4, 2.0, 20.0, 1e3] {
                let delta = k * eps;
                let wall = sphere_octant(r);
                let plane = Surface::Plane {
                    origin: Point3::new(0.0, 0.0, r - delta),
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                };
                let dom = SsiDomain {
                    center: Point3::new(0.0, 0.0, r - delta),
                    half_extent: r,
                    extent: 1.0,
                    floor_scale: 1.0,
                };
                let t = std::time::Instant::now();
                let res = ssi::plane_nurbs_ssi(&plane, &wall, dom, band_at(eps));
                println!(
                    "P1 eps={eps:e} R={r:e} delta={delta:e} ({k}eps) -> {} [{:.1}s]",
                    label(&res),
                    t.elapsed().as_secs_f64()
                );
            }
        }
    }
}

/// P2: cylinder × sphere near the internal (Viviani) and external
/// tangencies, with radii much smaller than the extent, so the old ℝ³
/// tube arm (the radius) and the new one (the extent) differ.
#[test]
fn p2_cylinder_sphere_near_tangent_small_radii() {
    for eps in [1e-6, 1e-9, 1e-12] {
      for ext in [1.0, 100.0] {
        let big_r = 1.0;
        let r = 0.5;
        for (kind, base, sign) in [("internal", big_r - r, 1.0), ("external", big_r + r, -1.0)] {
            for k in [-30.0, -3.0, 3.0, 30.0, 300.0] {
                let g = k * eps;
                let d = base + sign * g;
                let s = Surface::Sphere {
                    center: Point3::new(0.0, 0.0, 0.0),
                    radius: big_r,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                };
                let c = Surface::Cylinder {
                    origin: Point3::new(d, 0.0, 0.0),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    radius: r,
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                };
                let dom = SsiDomain {
                    center: Point3::new(0.0, 0.0, 0.0),
                    half_extent: 2.5,
                    extent: ext,
                    floor_scale: 1.0,
                };
                // truth: internal: d + r < R two loops, > R one loop;
                // external: d - r < R one loop, > R none.
                let truth = match kind {
                    "internal" => {
                        if d + r < big_r {
                            2
                        } else {
                            1
                        }
                    }
                    _ => {
                        if d - r < big_r {
                            1
                        } else {
                            0
                        }
                    }
                };
                let t = std::time::Instant::now();
                let res = ssi::cylinder_sphere_ssi(&c, &s, dom, band_at(eps));
                println!(
                    "P2 eps={eps:e} E={ext} {kind} gap={g:e} ({k}eps) truth={truth} -> {} [{:.1}s]",
                    label(&res),
                    t.elapsed().as_secs_f64()
                );
            }
        }
      }
    }
}

fn pmul(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut out = vec![0.0; a.len() + b.len() - 1];
    for (i, x) in a.iter().enumerate() {
        for (j, y) in b.iter().enumerate() {
            out[i + j] += x * y;
        }
    }
    out
}

fn binom(n: usize, k: usize) -> f64 {
    let mut r = 1.0;
    for i in 0..k {
        r = r * (n - i) as f64 / (i + 1) as f64;
    }
    r
}

/// Power-basis coefficients in `s` (padded to degree `n`) to Bernstein.
fn bern(a: &[f64], n: usize) -> Vec<f64> {
    let mut a = a.to_vec();
    a.resize(n + 1, 0.0);
    (0..=n)
        .map(|i| (0..=i).map(|k| binom(i, k) / binom(n, k) * a[k]).sum())
        .collect()
}

/// The dome `z = h − (x² + y²)/(2ρ)` with `x = A(w + c·w³)`, `w = 2s − 1`
/// (bunched about the apex for large `c`), `y = b(2t − 1)`; a degree-6
/// by degree-2 Bézier patch.
fn bunched_dome(rho: f64, h: f64, big_a: f64, c: f64, b: f64) -> NurbsSurface<f64> {
    let w = [-1.0, 2.0];
    let w3 = pmul(&pmul(&w, &w), &w);
    let mut x = vec![0.0; 4];
    for k in 0..2 {
        x[k] += big_a * w[k];
    }
    for k in 0..4 {
        x[k] += big_a * c * w3[k];
    }
    let xx = pmul(&x, &x);
    let y = [-b, 2.0 * b];
    let yy = pmul(&y, &y);
    let (nu, nv) = (6, 2);
    let bx = bern(&x, nu);
    let bxx = bern(&xx, nu);
    let by = bern(&y, nv);
    let byy = bern(&yy, nv);
    let ku = KnotVector::clamped([vec![0.0; 7], vec![1.0; 7]].concat(), 6).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::new();
    for i in 0..=nu {
        for j in 0..=nv {
            control.push(Point3::new(
                bx[i],
                by[j],
                h - (bxx[i] + byy[j]) / (2.0 * rho),
            ));
        }
    }
    let n = control.len();
    NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap()
}

/// P3: the plane `z = 0` cuts the dome near its apex (`h = δ`), a
/// circle of radius √(2ρδ). With `c` large the chart's parameter lines
/// bunch at the apex, so main's chart arm is far below ρ there; the
/// shape-operator arm is ρ. `δ < ε` is a sub-ε pose.
#[test]
fn p3_bunched_dome_near_tangent() {
    for eps in [1e-6, 1e-9, 1e-12] {
        for (c, big_a) in [(0.0, 0.5), (1e3, f64::sqrt(0.1 * 6.0 * 1e3 * eps))] {
            for k in [0.1, 0.4, 3.0, 30.0] {
                let rho = 1.0;
                let delta = k * eps;
                let wall = bunched_dome(rho, delta, big_a, c, 0.5);
                let plane = Surface::Plane {
                    origin: Point3::new(0.0, 0.0, 0.0),
                    normal: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                };
                let dom = SsiDomain {
                    center: Point3::new(0.0, 0.0, 0.0),
                    half_extent: 2.0,
                    extent: 1.0,
                    floor_scale: 1.0,
                };
                let t = std::time::Instant::now();
                let res = ssi::plane_nurbs_ssi(&plane, &wall, dom, band_at(eps));
                println!(
                    "P3 eps={eps:e} c={c} A={big_a:.3e} delta={delta:e} ({k}eps) circle_r={:.2e} -> {} [{:.1}s]",
                    (2.0 * rho * delta).sqrt(),
                    label(&res),
                    t.elapsed().as_secs_f64()
                );
            }
        }
    }
}

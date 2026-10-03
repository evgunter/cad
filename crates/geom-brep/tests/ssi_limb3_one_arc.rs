//! Limb 3's uniqueness tube must prove its window holds one arc.

use crate::shared::tol::{band, eps};
use geom::{NurbsSurface, Surface};
use geom_brep::ssi::{self, SsiDomain, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

/// A biquadratic Bézier graph `z = g(x) + h(y)` over `[x0,x1]×[0,l]`,
/// `g` and `h` quadratics given by their values and end slopes.
fn graph_wall(
    (x0, x1): (f64, f64),
    l: f64,
    g: impl Fn(f64) -> f64,
    dg0: f64,
    h: impl Fn(f64) -> f64,
    dh0: f64,
) -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let dx = x1 - x0;
    let gb = [g(x0), g(x0) + 0.5 * dx * dg0, g(x1)];
    let hb = [h(0.0), h(0.0) + 0.5 * l * dh0, h(l)];
    let xs = [x0, 0.5 * (x0 + x1), x1];
    let ys = [0.0, 0.5 * l, l];
    let mut control = Vec::new();
    for i in 0..3 {
        for j in 0..3 {
            control.push(Point3::new(xs[i], ys[j], gb[i] + hb[j]));
        }
    }
    NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap()
}

fn ground() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The dense truth's zeros of `φ` on a grid of `y` lines, against the
/// outcome's carriers: how many zeros, and how many lie farther than
/// `far` from every carrier.
fn lost_zeros(
    phi: &impl Fn(f64, f64) -> f64,
    (x0, x1): (f64, f64),
    l: f64,
    out: &SsiOutcome,
    far: f64,
) -> (usize, usize) {
    let pts: Vec<Point3<f64>> = out
        .branches
        .iter()
        .flat_map(|b| {
            let (t0, t1) = b.params;
            (0..=4000).map(move |k| b.carrier.eval(t0 + (t1 - t0) * f64::from(k) / 4000.0))
        })
        .collect();
    let (mut n, mut lost) = (0, 0);
    let mut check = |x: f64, y: f64| {
        n += 1;
        let d = pts
            .iter()
            .map(|p| ((p.x - x).powi(2) + (p.y - y).powi(2)).sqrt())
            .fold(f64::INFINITY, f64::min);
        if d > far {
            lost += 1;
        }
    };
    let (nx, ny) = (400, 400);
    for j in 0..=ny {
        let y = l * f64::from(j) / f64::from(ny);
        for i in 0..nx {
            let (a, b) = (
                x0 + (x1 - x0) * f64::from(i) / f64::from(nx),
                x0 + (x1 - x0) * f64::from(i + 1) / f64::from(nx),
            );
            if phi(a, y).signum() != phi(b, y).signum() {
                check(0.5 * (a + b), y);
            }
        }
    }
    (n, lost)
}

#[test]
fn reviewer_fold_probe() {
    let e = eps();
    let beta = e;
    let c = 80.0 * e;
    let a = 0.28 * c * c / beta;
    let w = beta / c;
    for lf in [1.2, 1.5] {
        let l = lf * w;
        let xr = (-1.5 * w, 1.8 * w);
        let g = move |x: f64| c * x + a * x * x;
        let h = move |y: f64| 4.0 * beta * y * (l - y) / (l * l);
        let wall = graph_wall(xr, l, g, c + 2.0 * a * xr.0, h, 4.0 * beta / l);
        let dom = SsiDomain {
            center: Point3::new(0.0, 0.0, 0.0),
            half_extent: 1.0,
            extent: 1.0,
            floor_scale: 1.0,
        };
        let r = ssi::plane_nurbs_ssi(&ground(), &wall, dom, band());
        match r {
            Ok(o) => {
                let phi = |x: f64, y: f64| g(x) + h(y);
                let (n, lost) = lost_zeros(&phi, xr, l, &o, 1e-3);
                eprintln!(
                    "L={lf}w: Ok {} branches, {:?}; zeros {n}, lost {lost}",
                    o.branches.len(),
                    o.branches
                        .iter()
                        .map(|b| (b.end, b.certificate.tube))
                        .collect::<Vec<_>>()
                );
            }
            Err(err) => eprintln!("L={lf}w: Err {err}"),
        }
    }
}

#[test]
fn fold_sweep_probe() {
    let e = eps();
    for (af, lf, cf, xlo, xhi) in [
        (0.28, 1.2, 80.0, -1.5, 1.8), (0.28, 0.5, 80.0, -1.5, 1.8), (0.28, 0.8, 80.0, -1.5, 1.8), (0.28, 2.0, 80.0, -1.5, 1.8),
        (0.28, 3.0, 80.0, -1.5, 1.8), (0.25, 1.2, 80.0, -1.5, 1.8), (0.26, 1.2, 80.0, -1.9, 1.8),
        (0.28, 1.2, 8.0, -1.5, 1.8), (0.28, 1.2, 800.0, -1.5, 1.8), (0.28, 1.2, 8000.0, -1.5, 1.8),
        (0.28, 0.3, 80.0, -1.5, 1.8), (0.28, 1.2, 80.0, -1.5, 0.3), (0.2, 1.2, 80.0, -2.4, 1.0),
    ] {
        let beta = e;
        let c = cf * e;
        let a = af * c * c / beta;
        let w = beta / c;
        let l = lf * w;
        let xr = (xlo * w, xhi * w);
        let g = move |x: f64| c * x + a * x * x;
        let h = move |y: f64| 4.0 * beta * y * (l - y) / (l * l);
        let wall = graph_wall(xr, l, g, c + 2.0 * a * xr.0, h, 4.0 * beta / l);
        let dom = SsiDomain {
            center: Point3::new(0.0, 0.0, 0.0),
            half_extent: 1.0,
            extent: 1.0,
            floor_scale: 1.0,
        };
        match ssi::plane_nurbs_ssi(&ground(), &wall, dom, band()) {
            Ok(o) => {
                let phi = |x: f64, y: f64| g(x) + h(y);
                let (n, lost) = lost_zeros(&phi, xr, l, &o, 0.02 * w);
                let (_, lost3) = lost_zeros(&phi, xr, l, &o, 0.3 * w);
                let ends: Vec<String> = o.branches.iter().map(|b| match b.end {
                    geom_brep::ssi::BranchEnd::Crossings { from, to } => format!("{:?}{:?}{:.2}->{:?}{:?}{:.2}", from.side.fixed, from.side.end, from.t, to.side.fixed, to.side.end, to.t),
                    other => format!("{other:?}"),
                }).collect();
                eprintln!("{af} {lf} {cf} {xlo} {xhi}: Ok {} br, zeros {n} lost {lost} lost.3w {lost3} {ends:?}", o.branches.len());
            }
            Err(err) => eprintln!("{af} {lf} {cf} {xlo} {xhi}: Err {err}"),
        }
    }
}

#[test]
fn r3_short_arc_probe() {
    let cyl = Surface::Cylinder {
        origin: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let sph = Surface::Sphere {
        center: Point3::new(0.0, 0.0, 0.0),
        radius: 3.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    for (alpha, beta, ext) in [(5.0f64, 11.0f64, 4.8), (3.0, 9.0, 4.8), (5.0, 13.0, 4.8), (3.0, 11.0, 2.4), (5.0, 11.0, 8.0), (5.0, 25.0, 4.8)] {
        let h = alpha.to_radians().cos();
        let k = beta.to_radians().sin();
        let half = 2.0;
        let dom = SsiDomain {
            center: Point3::new(h - half, k - half, 2.0),
            half_extent: half,
            extent: ext,
            floor_scale: 1.0,
        };
        match ssi::cylinder_sphere_ssi(&cyl, &sph, dom, band()) {
            Ok(o) => {
                // The short arc: angles alpha..beta on the circle at z0.
                let z0 = 8.0f64.sqrt();
                let mid = (0.5 * (alpha + beta)).to_radians();
                let p = Point3::new(mid.cos(), mid.sin(), z0);
                let near = o.branches.iter().any(|b| {
                    let (t0, t1) = b.params;
                    (0..=2000).any(|i| {
                        let q = b.carrier.eval(t0 + (t1 - t0) * f64::from(i) / 2000.0);
                        (q - p).norm() < 1e-3
                    })
                });
                eprintln!(
                    "a{alpha} b{beta} ext{ext}: Ok {} br, tubes {:?}, short arc carried: {near}",
                    o.branches.len(),
                    o.branches.iter().map(|b| b.certificate.tube).collect::<Vec<_>>()
                );
            }
            Err(e) => eprintln!("a{alpha} b{beta} ext{ext}: Err {e}"),
        }
    }
}

//! PR 4026 review r2's holed-block battery, ported to `AtRestBody`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[path = "../tests/common/differential.rs"]
#[allow(dead_code)]
mod differential;
use differential::outcome;
use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult, mass_properties};
type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, &BooleanDeclarations, Tol) -> Result<BooleanResult<f64>, BooleanError>;
fn run_all(label: &str, x: &Body<f64>, y: &Body<f64>, vx: f64, vy: f64, common: f64) {
    let x = AtRestBody::validate(x.clone(), tol()).unwrap();
    let y = AtRestBody::validate(y.clone(), tol()).unwrap();
    let decls = BooleanDeclarations::default();
    for (order, p, q, vp) in [("pc", &x, &y, vx), ("cp", &y, &x, vy)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, vx + vy - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vp - common),
        ];
        for (op, run, want) in ops {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| outcome(run(p, q, &decls, tol()), want, tol()).to_string()))
                .unwrap_or_else(|_| "PANIC".into());
            println!("{label} {order} {op}: {r}");
        }
    }
}
const SIDE: f64 = 4.0;
fn tol() -> Tol {
    Tol::witness()
}

fn unit(m: [f64; 3]) -> [f64; 3] {
    let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
    m.map(|c| c / n)
}

fn dot(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn frame(m: [f64; 3]) -> [[f64; 3]; 3] {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(seed, m));
    let w = cross(m, u);
    [u, w, m]
}

/// The cube `v + a·u + b·w + c·m` over `(−2, −2, 0) + [0, SIDE]³`: `v`
/// inside its near face, whose outward normal is `−m`.
fn cube(v: [f64; 3], f: [[f64; 3]; 3]) -> Body<f64> {
    let [u, w, m] = f;
    fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-2.0 + SIDE * x, -2.0 + SIDE * y, SIDE * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

fn cube_planes(v: [f64; 3], f: [[f64; 3]; 3]) -> Vec<([f64; 3], f64)> {
    let lo = [-2.0, -2.0, 0.0];
    let mut out = Vec::new();
    for (axis, &dir) in f.iter().enumerate() {
        let base = dot(dir, v);
        out.push((dir.map(|c| -c), -(base + lo[axis])));
        out.push((dir, base + lo[axis] + SIDE));
    }
    out
}

/// The PR's `convex_volume`, copied: the volume of a bounded convex
/// `{x : n·x ≤ d}`.
fn convex_volume(planes: &[([f64; 3], f64)]) -> f64 {
    const EPS: f64 = 1e-9;
    let mut pts: Vec<[f64; 3]> = Vec::new();
    let n = planes.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (a, b, c) = (planes[i], planes[j], planes[k]);
                let det = dot(a.0, cross(b.0, c.0));
                if det.abs() < 1e-12 {
                    continue;
                }
                let bc = cross(b.0, c.0);
                let ca = cross(c.0, a.0);
                let ab = cross(a.0, b.0);
                let p = [0, 1, 2].map(|t| (a.1 * bc[t] + b.1 * ca[t] + c.1 * ab[t]) / det);
                if planes.iter().all(|&(nn, d)| dot(nn, p) <= d + EPS)
                    && !pts
                        .iter()
                        .any(|q| (0..3).all(|t| (q[t] - p[t]).abs() < EPS))
                {
                    pts.push(p);
                }
            }
        }
    }
    if pts.len() < 4 {
        return 0.0;
    }
    let inner = [0, 1, 2].map(|t| pts.iter().map(|p| p[t]).sum::<f64>() / pts.len() as f64);
    let mut vol = 0.0;
    for &(nn, d) in planes {
        let on: Vec<[f64; 3]> = pts
            .iter()
            .copied()
            .filter(|&p| (dot(nn, p) - d).abs() < EPS)
            .collect();
        if on.len() < 3 {
            continue;
        }
        let c = [0, 1, 2].map(|t| on.iter().map(|p| p[t]).sum::<f64>() / on.len() as f64);
        let e1 = unit([0, 1, 2].map(|t| on[0][t] - c[t]));
        let e2 = cross(unit(nn), e1);
        let mut ring: Vec<(f64, [f64; 3])> = on
            .iter()
            .map(|&p| {
                let r = [0, 1, 2].map(|t| p[t] - c[t]);
                (dot(r, e2).atan2(dot(r, e1)), p)
            })
            .collect();
        ring.sort_by(|a, b| a.0.total_cmp(&b.0));
        let area: f64 = (0..ring.len())
            .map(|i| {
                let (p, q) = (ring[i].1, ring[(i + 1) % ring.len()].1);
                let r1 = [0, 1, 2].map(|t| p[t] - c[t]);
                let r2 = [0, 1, 2].map(|t| q[t] - c[t]);
                dot(cross(r1, r2), unit(nn)) / 2.0
            })
            .sum();
        let h = d / dot(nn, nn).sqrt() - dot(unit(nn), inner);
        vol += area.abs() * h / 3.0;
    }
    vol
}
fn cube_side(v: [f64; 3], f: [[f64; 3]; 3], side: f64) -> (Body<f64>, Vec<([f64; 3], f64)>) {
    let [u, w, m] = f;
    let h = side / 2.0;
    let body = fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-h + side * x, -h + side * y, side * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    );
    let lo = [-h, -h, 0.0];
    let mut planes = Vec::new();
    for (axis, &dir) in f.iter().enumerate() {
        let base = dot(dir, v);
        planes.push((dir.map(|c| -c), -(base + lo[axis])));
        planes.push((dir, base + lo[axis] + side));
    }
    (body, planes)
}

fn box_planes(lo: [f64; 3], hi: [f64; 3]) -> Vec<([f64; 3], f64)> {
    let mut out = Vec::new();
    for t in 0..3 {
        let mut e = [0.0; 3];
        e[t] = 1.0;
        out.push((e, hi[t]));
        out.push((e.map(|c| -c), -lo[t]));
    }
    out
}

/// **The holed block's inner corner on a face.** `[0,2]³` less the
/// through-hole `[0.5,1.5]² × [0,2]`; `v = (0.5, 0.5, 2)` is a hole
/// corner, locally the L-prism's reflex corner. With `m` positive and
/// gentle enough that the plane leaves the hole column through its
/// walls, the plane's section of the block closes round the hole's
/// part `H`, which meets the rest of the plane at `v` alone: an
/// island of the pierced face pinched at the pierce.
fn main() {
    let mut block = topo::test_support::holed_block::<f64>(2.0, &[1.0], tol());
    topo::test_support::describe_as_intersections(&mut block, tol());
    let va = mass_properties(&block, tol()).unwrap().volume;
    assert!((va - 6.0).abs() < 1e-9, "{va}");
    let mut dirs: Vec<(String, [f64; 3])> = Vec::new();
    for i in 0..8 {
        for (j, &phi) in [0.45f64, 0.6, 0.75, 0.9, 1.05, 1.2, 1.35].iter().enumerate() {
            let theta = std::f64::consts::FRAC_PI_2 * (f64::from(i) + 0.5) / 8.0;
            dirs.push((
                format!("q{i}.{j}"),
                [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()],
            ));
        }
    }
    for i in 0..12 {
        for j in 0..7 {
            let theta = std::f64::consts::TAU * (f64::from(i) + 0.37) / 12.0;
            let phi = (f64::from(j) - 3.0) * 0.4 + 0.05;
            dirs.push((
                format!("g{i}.{j}"),
                [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()],
            ));
        }
    }
    for (corner, v) in [("c00", [0.5, 0.5, 2.0]), ("c11", [1.5, 1.5, 2.0])] {
        for (tag, m) in &dirs {
            // c11's hole quadrant is −x, −y: turn m to match.
            let m = if corner == "c11" { [-m[0], -m[1], m[2]] } else { *m };
            let f = frame(m);
            for side in [4.0, 12.0] {
                let (b, cp) = cube_side(v, f, side);
                let mut outer = box_planes([0.0; 3], [2.0; 3]);
                outer.extend(cp.iter().copied());
                let mut hole = box_planes([0.5, 0.5, 0.0], [1.5, 1.5, 2.0]);
                hole.extend(cp.iter().copied());
                let common = convex_volume(&outer) - convex_volume(&hole);
                let vb = side * side * side;
                run_all(&format!("holed {corner} side={side} {tag}"), &block, &b, va, vb, common);
            }
        }
    }
}

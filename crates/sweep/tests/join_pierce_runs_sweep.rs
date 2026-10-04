//! **The pierce-runs sweep**: the L-prism's reflex top corner
//! `v = (1, 1, 1)` on a cube of side 4, placed so that `v` lies on the
//! cube's near face (`face`), on one of its edges (`edge`) or at one of
//! its corners (`corner`), the cube turned over a grid of directions.
//! Depending on the direction the corner has zero, one or two Out runs
//! against the face, and the edge and corner placements reach the
//! same corner through the vertex-edge and vertex-vertex lanes. Every
//! op in both operand orders prints one [`outcome`] line, against an
//! oracle that clips each of the prism's two boxes by the cube's six
//! half-spaces (`convex_volume`), independent of the kernel.
//!
//! `cargo test -p sweep --release --test all pierce_runs_battery --
//! --ignored --nocapture`, on two trees, and diff the lines.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult, mass_properties};

use crate::common::differential::outcome;

const PROFILE: [(f64, f64); 6] = [
    (0.0, 0.0),
    (2.0, 0.0),
    (2.0, 1.0),
    (1.0, 1.0),
    (1.0, 2.0),
    (0.0, 2.0),
];
const V: [f64; 3] = [1.0, 1.0, 1.0];
const SIDE: f64 = 4.0;
/// The prism as two boxes with disjoint interiors, `(lo, hi)`.
const BOXES: [([f64; 3], [f64; 3]); 2] = [
    ([0.0, 0.0, 0.0], [2.0, 1.0, 1.0]),
    ([0.0, 1.0, 0.0], [1.0, 2.0, 1.0]),
];

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

/// The right-handed frame `(u, w, m)` with `m` the unit of `m` and `u`
/// turned by `psi` about it.
fn frame(m: [f64; 3], psi: f64) -> [[f64; 3]; 3] {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u0 = unit(cross(seed, m));
    let w0 = cross(m, u0);
    let (c, s) = (psi.cos(), psi.sin());
    let u = [0, 1, 2].map(|i| c * u0[i] + s * w0[i]);
    let w = cross(m, u);
    [u, w, m]
}

/// The cube `V + a·u + b·w + c·m`, `(a, b, c)` over `lo + [0, SIDE]³`:
/// `lo = (−2, −2, 0)` puts `V` inside the near face, `(0, −2, 0)` on
/// its edge along `w`, `(0, 0, 0)` at its corner.
fn cube(f: [[f64; 3]; 3], lo: [f64; 3]) -> Body<f64> {
    let [u, w, m] = f;
    fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (lo[0] + SIDE * x, lo[1] + SIDE * y, lo[2] + SIDE * z);
            Point3::new(
                V[0] + a * u[0] + b * w[0] + c * m[0],
                V[1] + a * u[1] + b * w[1] + c * m[1],
                V[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

/// The cube's six half-spaces `n·x ≤ d`.
fn cube_planes(f: [[f64; 3]; 3], lo: [f64; 3]) -> Vec<([f64; 3], f64)> {
    let mut out = Vec::new();
    for (axis, &dir) in f.iter().enumerate() {
        let base = dot(dir, V);
        out.push((dir.map(|c| -c), -(base + lo[axis])));
        out.push((dir, base + lo[axis] + SIDE));
    }
    out
}

/// The volume of `{x : n·x ≤ d for every plane}`, bounded and convex:
/// its vertices are the planes' feasible triple meets, each face is
/// the vertices on its plane fanned about their centroid, and the
/// volume sums the pyramids over the faces from an interior point.
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

/// The prism's volume inside the cube.
fn shared(planes: &[([f64; 3], f64)]) -> f64 {
    BOXES
        .iter()
        .map(|&(lo, hi)| {
            let mut all = planes.to_vec();
            for t in 0..3 {
                let mut e = [0.0; 3];
                e[t] = 1.0;
                all.push((e, hi[t]));
                all.push((e.map(|c| -c), -lo[t]));
            }
            convex_volume(&all)
        })
        .sum()
}

type Op = fn(
    &Body<f64>,
    &Body<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

#[test]
#[ignore = "differential battery; run with --ignored --nocapture"]
fn pierce_runs_battery() {
    let prism = fixtures::prism::<f64>(&PROFILE, 1.0, tol()).body;
    let va = mass_properties(&prism, tol()).unwrap().volume;
    let vb = SIDE * SIDE * SIDE;
    let decls = BooleanDeclarations::default();
    let placements: [(&str, [f64; 3], &[f64]); 3] = [
        ("face", [-2.0, -2.0, 0.0], &[0.0]),
        ("edge", [0.0, -2.0, 0.0], &[0.0, 1.0, 2.2, 4.0]),
        ("corner", [0.0, 0.0, 0.0], &[0.0, 1.0, 2.2, 4.0]),
    ];
    for (place, lo, psis) in placements {
        for i in 0..12 {
            for j in 0..7 {
                let theta = std::f64::consts::TAU * (f64::from(i) + 0.37) / 12.0;
                let phi = (f64::from(j) - 3.0) * 0.4 + 0.05;
                let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
                for &psi in psis {
                    let f = frame(m, psi);
                    let b = cube(f, lo);
                    let common = shared(&cube_planes(f, lo));
                    for (order, x, y, vx) in [("pc", &prism, &b, va), ("cp", &b, &prism, vb)] {
                        let ops: [(&str, Op, f64); 3] = [
                            ("U", topo::union_with, va + vb - common),
                            ("I", topo::intersect_with, common),
                            ("S", topo::subtract_with, vx - common),
                        ];
                        for (op, run, want) in ops {
                            println!(
                                "{place} i={i} j={j} psi={psi} {order} {op}: {}",
                                outcome(run(x, y, &decls, tol()), want, tol())
                            );
                        }
                    }
                }
            }
        }
    }
}

/// The oracle against the kernel-free closed form the strut-facing row
/// reads: a face placement at the bare tilt holds the prism's cut by
/// the plane through `v`, and the empty and whole cases read 0 and 3.
#[test]
fn the_sweep_oracle_reads_the_prism() {
    let f = frame([1.0, 1.3, -0.7], 0.0);
    let face = shared(&cube_planes(f, [-2.0, -2.0, 0.0]));
    let opposite = shared(&cube_planes(
        frame([-1.0, -1.3, 0.7], 0.0),
        [-2.0, -2.0, 0.0],
    ));
    assert!(
        (face + opposite - 3.0).abs() < 1e-9,
        "{face} + {opposite} is not the prism's 3"
    );
    assert!(face > 0.5 && opposite > 0.5, "both cuts substantial");
    let away = shared(&cube_planes(frame([0.0, 0.0, 1.0], 0.0), [-2.0, -2.0, 0.0]));
    assert!(
        away.abs() < 1e-12,
        "a cube above the top face holds nothing: {away}"
    );
    let all = shared(&cube_planes(
        frame([0.0, 0.0, -1.0], 0.0),
        [-2.0, -2.0, -0.5],
    ));
    assert!(
        (all - 3.0).abs() < 1e-9,
        "a cube around the prism holds it whole: {all}"
    );
}

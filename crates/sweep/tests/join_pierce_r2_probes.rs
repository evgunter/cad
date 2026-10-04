//! **Review r2 probes for PR 4026** (`join/pierce-two-out-runs`): a
//! prism corner placed on a planar cube face or on a ball, the pierced
//! face turned over a grid of directions and near-tangent tilts, every
//! op in both orders, each line one [`outcome`] against a kernel-free
//! oracle: the prism's convex pieces clipped by the cube's half-spaces
//! (`convex_volume`), or each axis box of the L-prism integrated
//! against the ball (`box_ball`, an exact disc∩rectangle area
//! integrated in z).
//!
//! `cargo test -p sweep --release --test all r2_ -- --ignored
//! --nocapture`, on two trees, and diff the lines.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use topo::test_support as fixtures;
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult, mass_properties};

use crate::common::differential::outcome;

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

/// A prism over `profile` (counter-clockwise), `z ∈ [0, 1]`, with its
/// convex pieces (each counter-clockwise) and the corner `v` probed.
struct Shape {
    name: &'static str,
    profile: Vec<(f64, f64)>,
    pieces: Vec<Vec<(f64, f64)>>,
    v: [f64; 3],
}

/// The half-spaces of one extruded convex piece.
fn piece_planes(piece: &[(f64, f64)]) -> Vec<([f64; 3], f64)> {
    let mut out = vec![([0.0, 0.0, 1.0], 1.0), ([0.0, 0.0, -1.0], 0.0)];
    for i in 0..piece.len() {
        let (p, q) = (piece[i], piece[(i + 1) % piece.len()]);
        let n = [q.1 - p.1, p.0 - q.0, 0.0];
        out.push((n, n[0] * p.0 + n[1] * p.1));
    }
    out
}

fn mirror(s: &Shape, name: &'static str) -> Shape {
    let flip = |p: &Vec<(f64, f64)>| -> Vec<(f64, f64)> { p.iter().rev().map(|&(x, y)| (y, x)).collect() };
    Shape {
        name,
        profile: flip(&s.profile),
        pieces: s.pieces.iter().map(flip).collect(),
        v: [s.v[1], s.v[0], s.v[2]],
    }
}

fn shapes() -> Vec<Shape> {
    let l = Shape {
        name: "L270",
        profile: vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
        pieces: vec![
            vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
            vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
        ],
        v: [1.0, 1.0, 1.0],
    };
    let r315 = Shape {
        name: "R315",
        profile: vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (1.0, 1.0), (3.0, 3.0), (0.0, 3.0)],
        pieces: vec![
            vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)],
            vec![(0.0, 1.0), (1.0, 1.0), (3.0, 3.0), (0.0, 3.0)],
        ],
        v: [1.0, 1.0, 1.0],
    };
    let r225 = Shape {
        name: "R225",
        profile: vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (1.0, 1.0), (0.0, 2.0)],
        pieces: vec![
            vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)],
            vec![(0.0, 1.0), (1.0, 1.0), (0.0, 2.0)],
        ],
        v: [1.0, 1.0, 1.0],
    };
    let mut out = vec![
        mirror(&r315, "R315m"),
        mirror(&r225, "R225m"),
        Shape {
            name: "Lbot",
            v: [1.0, 1.0, 0.0],
            profile: l.profile.clone(),
            pieces: l.pieces.clone(),
        },
        Shape {
            name: "Lcvx",
            v: [2.0, 1.0, 1.0],
            profile: l.profile.clone(),
            pieces: l.pieces.clone(),
        },
    ];
    out.insert(0, r225);
    out.insert(0, r315);
    out.insert(0, l);
    out
}

type Op = fn(
    &Body<f64>,
    &Body<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

fn run_all(label: &str, x: &Body<f64>, y: &Body<f64>, vx: f64, vy: f64, common: f64) {
    let decls = BooleanDeclarations::default();
    for (order, p, q, vp) in [("pc", x, y, vx), ("cp", y, x, vy)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, vx + vy - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vp - common),
        ];
        for (op, run, want) in ops {
            println!("{label} {order} {op}: {}", outcome(run(p, q, &decls, tol()), want, tol()));
        }
    }
}

/// The directions: a 24 × 9 grid, then near-tangent tilts about every
/// face normal and every edge-perpendicular of the corner.
fn directions(s: &Shape) -> Vec<(String, [f64; 3])> {
    let mut out = Vec::new();
    for i in 0..24 {
        for j in 0..9 {
            let theta = std::f64::consts::TAU * (f64::from(i) + 0.21) / 24.0;
            let phi = (f64::from(j) - 4.0) * 0.33 + 0.03;
            out.push((
                format!("g{i}.{j}"),
                [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()],
            ));
        }
    }
    // The corner's face normals and edge directions, read off the
    // profile at v.
    let n = s.profile.len();
    let k = s
        .profile
        .iter()
        .position(|&(x, y)| x == s.v[0] && y == s.v[1])
        .unwrap();
    let (p, q) = (s.profile[(k + n - 1) % n], s.profile[(k + 1) % n]);
    let e_in = unit([s.v[0] - p.0, s.v[1] - p.1, 0.0]);
    let e_out = unit([q.0 - s.v[0], q.1 - s.v[1], 0.0]);
    let zf = if s.v[2] == 1.0 { 1.0 } else { -1.0 };
    let normals = [
        [0.0, 0.0, zf],
        [e_in[1], -e_in[0], 0.0],
        [e_out[1], -e_out[0], 0.0],
    ];
    let edges = [[0.0, 0.0, 1.0], e_in, e_out];
    let kicks = [[0.3, 0.7, 0.2], [-0.6, 0.1, 0.5], [0.2, -0.4, -0.8]];
    for (tag, base) in normals.iter().map(|n| ("nf", *n)).chain(
        // a plane nearly containing an edge: perpendicular to it.
        edges.iter().map(|e| ("ne", unit(cross(*e, [0.31, 0.57, 0.76])))),
    ) {
        for eps in [1e-2, 1e-4, 1e-6] {
            for (ki, kick) in kicks.iter().enumerate() {
                for sign in [1.0, -1.0] {
                    let m = [0, 1, 2].map(|t| sign * base[t] + eps * kick[t]);
                    out.push((format!("{tag}{eps:e}k{ki}s{sign}"), m));
                }
            }
        }
    }
    let _ = edges;
    out
}

#[test]
#[ignore = "review probe battery; run with --ignored --nocapture"]
fn r2_shapes_battery() {
    for s in shapes() {
        let prism = fixtures::prism::<f64>(&s.profile, 1.0, tol()).body;
        let va = mass_properties(&prism, tol()).unwrap().volume;
        let va_oracle: f64 = s.pieces.iter().map(|p| convex_volume(&piece_planes(p))).sum();
        assert!((va - va_oracle).abs() < 1e-9, "{}: pieces {va_oracle} vs {va}", s.name);
        for (tag, m) in directions(&s) {
            let f = frame(m);
            let b = cube(s.v, f);
            let cp = cube_planes(s.v, f);
            let common: f64 = s
                .pieces
                .iter()
                .map(|p| {
                    let mut all = piece_planes(p);
                    all.extend(cp.iter().copied());
                    convex_volume(&all)
                })
                .sum();
            run_all(&format!("{} {tag}", s.name), &prism, &b, va, SIDE * SIDE * SIDE, common);
        }
    }
}

/// `∫ sqrt(R² − t²) dt`.
fn s_int(t: f64, r: f64) -> f64 {
    let t = t.clamp(-r, r);
    0.5 * (t * (r * r - t * t).max(0.0).sqrt() + r * r * (t / r).clamp(-1.0, 1.0).asin())
}

/// The exact area of the disc `(c, r)` inside `[x0, x1] × [y0, y1]`.
fn disc_rect(c: (f64, f64), r: f64, x: (f64, f64), y: (f64, f64)) -> f64 {
    if r <= 0.0 {
        return 0.0;
    }
    let (a, b) = (x.0.max(c.0 - r), x.1.min(c.0 + r));
    if a >= b {
        return 0.0;
    }
    let mut cuts = vec![a, b];
    for yy in [y.0, y.1] {
        let d = (yy - c.1).abs();
        if d < r {
            let h = (r * r - d * d).sqrt();
            for t in [c.0 - h, c.0 + h] {
                if t > a && t < b {
                    cuts.push(t);
                }
            }
        }
    }
    cuts.sort_by(f64::total_cmp);
    let mut area = 0.0;
    for w in cuts.windows(2) {
        let (t0, t1) = (w[0], w[1]);
        let tm = 0.5 * (t0 + t1);
        let sm = (r * r - (tm - c.0).powi(2)).max(0.0).sqrt();
        let up_clamped = c.1 + sm > y.1;
        let lo_clamped = c.1 - sm < y.0;
        let upper = if up_clamped { y.1 } else { c.1 + sm };
        let lower = if lo_clamped { y.0 } else { c.1 - sm };
        if upper <= lower {
            continue;
        }
        let si = s_int(t1 - c.0, r) - s_int(t0 - c.0, r);
        let up = if up_clamped { y.1 * (t1 - t0) } else { c.1 * (t1 - t0) + si };
        let lo = if lo_clamped { y.0 * (t1 - t0) } else { c.1 * (t1 - t0) - si };
        area += up - lo;
    }
    area
}

fn simpson(f: &dyn Fn(f64) -> f64, a: f64, b: f64, fa: f64, fm: f64, fb: f64, whole: f64, eps: f64, depth: u32) -> f64 {
    let m = 0.5 * (a + b);
    let (lm, rm) = (0.5 * (a + m), 0.5 * (m + b));
    let (flm, frm) = (f(lm), f(rm));
    let left = (m - a) / 6.0 * (fa + 4.0 * flm + fm);
    let right = (b - m) / 6.0 * (fm + 4.0 * frm + fb);
    if depth == 0 || (left + right - whole).abs() <= 15.0 * eps {
        return left + right + (left + right - whole) / 15.0;
    }
    simpson(f, a, m, fa, flm, fm, left, eps / 2.0, depth - 1)
        + simpson(f, m, b, fm, frm, fb, right, eps / 2.0, depth - 1)
}

/// The volume of the axis box `lo..hi` inside the ball `(c, r)`.
fn box_ball(lo: [f64; 3], hi: [f64; 3], c: [f64; 3], r: f64) -> f64 {
    let (z0, z1) = (lo[2].max(c[2] - r), hi[2].min(c[2] + r));
    if z0 >= z1 {
        return 0.0;
    }
    let f = |z: f64| {
        let rr = (r * r - (z - c[2]).powi(2)).max(0.0).sqrt();
        disc_rect((c[0], c[1]), rr, (lo[0], hi[0]), (lo[1], hi[1]))
    };
    // Pre-split into panels so the adaptive pass sees every kink.
    let k = 64;
    let mut v = 0.0;
    for i in 0..k {
        let a = z0 + (z1 - z0) * f64::from(i) / f64::from(k);
        let b = z0 + (z1 - z0) * f64::from(i + 1) / f64::from(k);
        let (fa, fm, fb) = (f(a), f(0.5 * (a + b)), f(b));
        let whole = (b - a) / 6.0 * (fa + 4.0 * fm + fb);
        v += simpson(&f, a, b, fa, fm, fb, whole, 1e-14, 40);
    }
    v
}

#[test]
fn r2_ball_oracle_reads_known_volumes() {
    let ball = 4.0 / 3.0 * std::f64::consts::PI * 8.0;
    let all = box_ball([-5.0; 3], [5.0; 3], [0.1, 0.2, 0.3], 2.0);
    assert!((all - ball).abs() < 1e-10, "{all} vs {ball}");
    let inside = box_ball([0.0; 3], [1.0; 3], [0.5, 0.5, 0.5], 2.0);
    assert!((inside - 1.0).abs() < 1e-12, "{inside}");
    // Half-space cut: the box below z = c.z holds half the ball.
    let half = box_ball([-5.0, -5.0, -5.0], [5.0, 5.0, 0.3], [0.1, 0.2, 0.3], 2.0);
    assert!((half - ball / 2.0).abs() < 1e-10, "{half}");
    // A cap: height h holds π h² (3r − h) / 3.
    let h = 0.7;
    let cap = box_ball([-5.0, -5.0, 2.3 - h], [5.0, 5.0, 5.0], [0.1, 0.2, 0.3], 2.0);
    let want = std::f64::consts::PI * h * h * (6.0 - h) / 3.0;
    assert!((cap - want).abs() < 1e-10, "{cap} vs {want}");
    // A quarter: x ≥ cx and y ≥ cy.
    let q = box_ball([0.1, 0.2, -5.0], [5.0, 5.0, 5.0], [0.1, 0.2, 0.3], 2.0);
    assert!((q - ball / 4.0).abs() < 1e-10, "{q}");
}

#[test]
#[ignore = "review probe battery; run with --ignored --nocapture"]
fn r2_ball_battery() {
    let l = &shapes()[0];
    let boxes = [([0.0, 0.0, 0.0], [2.0, 1.0, 1.0]), ([0.0, 1.0, 0.0], [1.0, 2.0, 1.0])];
    let prism = fixtures::prism::<f64>(&l.profile, 1.0, tol()).body;
    let va = mass_properties(&prism, tol()).unwrap().volume;
    for r in [1.5, 3.0] {
        for i in 0..16 {
            for j in 0..7 {
                let theta = std::f64::consts::TAU * (f64::from(i) + 0.29) / 16.0;
                let phi = (f64::from(j) - 3.0) * 0.4 + 0.07;
                let m = unit([theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]);
                // The ball on m's side, v on it: its outward normal at
                // v is −m, as the cube's near face.
                let c = [0, 1, 2].map(|t| l.v[t] + r * m[t]);
                let pole = unit(cross(m, [0.43, -0.71, 0.55]));
                let ball = sweep::test_support::ball_poled(
                    r,
                    Vec3::new(c[0], c[1], c[2]),
                    Vec3::new(pole[0], pole[1], pole[2]),
                    tol(),
                );
                let vb = mass_properties(&ball, tol()).unwrap().volume;
                let vb_exact = 4.0 / 3.0 * std::f64::consts::PI * r * r * r;
                let common: f64 = boxes.iter().map(|&(lo, hi)| box_ball(lo, hi, c, r)).sum();
                println!("ball r={r} i={i} j={j} vb_err={:.1e}", vb - vb_exact);
                run_all(&format!("ball r={r} i={i} j={j}"), &prism, &ball, va, vb_exact, common);
            }
        }
    }
}

/// The cube of side `side`, centred on `v` in its near face.
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
#[test]
#[ignore = "review probe battery; run with --ignored --nocapture"]
fn r2_holed_battery() {
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

/// One holed pose's results, read at `v`: the vertices on `v`'s point
/// and, per face through them, how many times each of its loops
/// passes them, plus the face count of the plane's faces.
#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_holed_inspect() {
    let mut block = topo::test_support::holed_block::<f64>(2.0, &[1.0], tol());
    topo::test_support::describe_as_intersections(&mut block, tol());
    let v = [0.5, 0.5, 2.0];
    let spec = std::env::var("R2_POSE").unwrap_or_else(|_| "0,0.75".into());
    let (i, phi): (f64, f64) = {
        let mut it = spec.split(',').map(|s| s.parse::<f64>().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    };
    let theta = std::f64::consts::FRAC_PI_2 * (i + 0.5) / 8.0;
    let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
    let f = frame(m);
    let (b, _) = cube_side(v, f, 12.0);
    let decls = BooleanDeclarations::default();
    let ops: [(&str, Op); 3] = [
        ("U", topo::union_with),
        ("I", topo::intersect_with),
        ("S", topo::subtract_with),
    ];
    for (order, p, q) in [("pc", &block, &b), ("cp", &b, &block)] {
        for (op, run) in ops {
            let Ok(r) = run(p, q, &decls, tol()) else {
                println!("{order} {op}: refused");
                continue;
            };
            let Some(bb) = r.body() else {
                println!("{order} {op}: empty");
                continue;
            };
            let body = &bb.body;
            let at: Vec<_> = body
                .vertices()
                .filter(|(_, vd)| {
                    let pt = body.get_point(vd.point).unwrap();
                    (pt.x - v[0]).abs() < 1e-9 && (pt.y - v[1]).abs() < 1e-9 && (pt.z - v[2]).abs() < 1e-9
                })
                .map(|(k, _)| k)
                .collect();
            let mut lines = Vec::new();
            let mut faces = std::collections::BTreeSet::new();
            for &k in &at {
                for fk in body.faces_of_vertex(k).unwrap() {
                    faces.insert(fk);
                }
            }
            for fk in faces {
                let fd = body.get_face(fk).unwrap();
                let mut per_loop = Vec::new();
                for (li, &l) in std::iter::once(&fd.outer).chain(&fd.rings).enumerate() {
                    if let topo::LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                        let passes: Vec<usize> = at
                            .iter()
                            .map(|&k| {
                                body.loop_cycle(first)
                                    .unwrap()
                                    .iter()
                                    .filter(|&&he| body.get_half_edge(he).unwrap().start == k)
                                    .count()
                            })
                            .collect();
                        per_loop.push(format!("{}{passes:?}", if li == 0 { "o" } else { "r" }));
                    }
                }
                lines.push(format!("{fk:?} rings={} {}", fd.rings.len(), per_loop.join(" ")));
            }
            println!("{order} {op}: {} vertices at v; faces: {}", at.len(), lines.join(" | "));
        }
    }
}

/// The strut-facing row's tilts, every op and order, one line each:
/// for the mutant runs (`R2_MUT`) and main.
#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_row_poses() {
    let l = &shapes()[0];
    let prism = fixtures::prism::<f64>(&l.profile, 1.0, tol()).body;
    let va = mass_properties(&prism, tol()).unwrap().volume;
    let poses: [(&str, [f64; 3]); 9] = [
        ("two", [1.0, 1.3, 0.7]),
        ("two", [1.0, 1.3, 0.4]),
        ("two", [1.2, 1.0, 0.3]),
        ("wide", [2.0, 0.4, 1.0]),
        ("wide", [1.0, 0.2, 0.5]),
        ("edge", [-1.0, -0.9, -1.3]),
        ("edge", [-1.0, -0.6, -1.2]),
        ("edge", [-0.8, -0.7, -1.0]),
        ("bare", [1.0, 1.3, -0.7]),
    ];
    for (name, m) in poses {
        let f = frame(m);
        let b = cube(l.v, f);
        let cp = cube_planes(l.v, f);
        let common: f64 = l
            .pieces
            .iter()
            .map(|p| {
                let mut all = piece_planes(p);
                all.extend(cp.iter().copied());
                convex_volume(&all)
            })
            .sum();
        run_all(&format!("{name} {m:?}"), &prism, &b, va, SIDE * SIDE * SIDE, common);
    }
}

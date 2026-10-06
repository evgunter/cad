//! PR 4139 review r2's probes: pinches with three cones at one point, and
//! cones holding several runs of each operand, at a cube's corner.
//!
//! `A` is the cube `[0, 2]³`; `B` the parallelepiped spanned from the
//! origin by `L·d_i`, `d_1 = (−e1, 1, 1)`, `d_2 = (1, −e2, 1)`,
//! `d_3 = (1, 1, −e3)`. With every `e_i > 0`, `B`'s tip pokes out
//! through each of `A`'s three corner faces: `A ∖ B` and `B ∖ A` hold
//! three cones at the origin, `A ∪ B` and `A ∩ B` one cone whose
//! boundary alternates three arcs of each operand. Signs mixed give one
//! or two cones. Both bodies are turned and moved by one rigid map.
//!
//! The cone count is read independently of the kernel: the op's region
//! is sampled on a small sphere about the point (a cube-map grid), and
//! its boundary curves number `components(in) + components(out) − 1`.
//! A line prints the kernel's vertices at the point beside that count.
//!
//! `cargo test -p sweep --release --test all r2_pinch_cones -- --ignored
//! --nocapture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, VecDeque};

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{AtRestBody, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::outcome;

fn tol() -> Tol {
    Tol::witness()
}

type V3 = [f64; 3];
type Plane = (V3, f64);

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn unit(m: V3) -> V3 {
    let n = dot(m, m).sqrt();
    m.map(|c| c / n)
}

/// A rotation from a seed: Rodrigues about `axis` by `ang`.
fn rot(axis: V3, ang: f64) -> [V3; 3] {
    let k = unit(axis);
    let (c, s) = (ang.cos(), ang.sin());
    let mut r = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            let kk = k[i] * k[j] * (1.0 - c);
            let delta = if i == j { c } else { 0.0 };
            let skew = match (i, j) {
                (0, 1) => -k[2] * s,
                (0, 2) => k[1] * s,
                (1, 0) => k[2] * s,
                (1, 2) => -k[0] * s,
                (2, 0) => -k[1] * s,
                (2, 1) => k[0] * s,
                _ => 0.0,
            };
            r[i][j] = kk + delta + skew;
        }
    }
    r
}

fn apply(r: &[V3; 3], x: V3) -> V3 {
    [dot(r[0], x), dot(r[1], x), dot(r[2], x)]
}

/// The parallelepiped `{ M u : u ∈ [0, 1]³ }`, `M`'s columns `cols`: its
/// half-spaces `n·x ≤ d` (through `M⁻¹`'s rows).
fn ppd_planes(cols: [V3; 3]) -> Vec<Plane> {
    let [a, b, c] = cols;
    let det = dot(a, cross(b, c));
    let rows = [cross(b, c), cross(c, a), cross(a, b)].map(|r| r.map(|x| x / det));
    let mut out = Vec::new();
    for r in rows {
        out.push((r.map(|x| -x), 0.0));
        out.push((r, 1.0));
    }
    out
}

/// The rigid map `x ↦ R x + t` applied to half-spaces.
fn moved(planes: &[Plane], r: &[V3; 3], t: V3) -> Vec<Plane> {
    planes
        .iter()
        .map(|&(n, d)| {
            let rn = apply(r, n);
            (rn, d + dot(rn, t))
        })
        .collect()
}

fn inside(planes: &[Plane], p: V3) -> bool {
    planes.iter().all(|&(n, d)| dot(n, p) <= d)
}

/// The volume of `{x : n·x ≤ d}` (bounded, convex): pyramids over the
/// faces from the vertices' centroid.
fn convex_volume(planes: &[Plane]) -> f64 {
    const EPS: f64 = 1e-9;
    let mut pts: Vec<V3> = Vec::new();
    let n = planes.len();
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                let (a, b, c) = (planes[i], planes[j], planes[k]);
                let det = dot(a.0, cross(b.0, c.0));
                if det.abs() < 1e-12 {
                    continue;
                }
                let (bc, ca, ab) = (cross(b.0, c.0), cross(c.0, a.0), cross(a.0, b.0));
                let p = [0, 1, 2].map(|t| (a.1 * bc[t] + b.1 * ca[t] + c.1 * ab[t]) / det);
                if planes.iter().all(|&(nn, d)| dot(nn, p) <= d + EPS)
                    && !pts.iter().any(|q| (0..3).all(|t| (q[t] - p[t]).abs() < EPS))
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
        let on: Vec<V3> = pts
            .iter()
            .copied()
            .filter(|&p| (dot(nn, p) - d).abs() < EPS * dot(nn, nn).sqrt().max(1.0))
            .collect();
        if on.len() < 3 {
            continue;
        }
        let c = [0, 1, 2].map(|t| on.iter().map(|p| p[t]).sum::<f64>() / on.len() as f64);
        let e1 = unit([0, 1, 2].map(|t| on[0][t] - c[t]));
        let e2 = cross(unit(nn), e1);
        let mut ring: Vec<(f64, V3)> = on
            .iter()
            .map(|&p| {
                let r = [0, 1, 2].map(|t| p[t] - c[t]);
                (dot(r, e2).atan2(dot(r, e1)), p)
            })
            .collect();
        ring.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
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

/// The op's membership from the operands'.
fn member(op: &str, a: bool, b: bool) -> bool {
    match op {
        "U" => a || b,
        "I" => a && b,
        "S" => a && !b,
        _ => unreachable!(),
    }
}

/// The boundary curves of the op's region round `at` (radius `r`),
/// counted on a cube-map grid of `n` cells per half-edge of each face:
/// `components(in) + components(out) − 1`, 0 where the point is not on
/// the boundary.
fn cones_at(
    at: V3,
    r: f64,
    n: i64,
    within: &dyn Fn(V3) -> bool,
) -> usize {
    // A small fixed turn so no sample lies on a face plane by symmetry.
    let q = rot([0.31, -0.77, 0.52], 0.4137);
    let mut cell: BTreeMap<[i64; 3], bool> = BTreeMap::new();
    for axis in 0..3 {
        for side in [-n, n] {
            for i in -n..=n {
                for j in -n..=n {
                    let mut k = [0; 3];
                    k[axis] = side;
                    k[(axis + 1) % 3] = i;
                    k[(axis + 2) % 3] = j;
                    if cell.contains_key(&k) {
                        continue;
                    }
                    let d = unit(apply(&q, k.map(|x| x as f64)));
                    let p = [0, 1, 2].map(|t| at[t] + r * d[t]);
                    cell.insert(k, within(p));
                }
            }
        }
    }
    let mut seen: BTreeMap<[i64; 3], ()> = BTreeMap::new();
    let (mut ins, mut outs) = (0, 0);
    let keys: Vec<[i64; 3]> = cell.keys().copied().collect();
    for k0 in keys {
        if seen.contains_key(&k0) {
            continue;
        }
        let v = cell[&k0];
        if v {
            ins += 1;
        } else {
            outs += 1;
        }
        let mut queue = VecDeque::from([k0]);
        seen.insert(k0, ());
        while let Some(k) = queue.pop_front() {
            for t in 0..3 {
                for s in [-1, 1] {
                    let mut m = k;
                    m[t] += s;
                    if let Some(&w) = cell.get(&m) {
                        if w == v && !seen.contains_key(&m) {
                            seen.insert(m, ());
                            queue.push_back(m);
                        }
                    }
                }
            }
        }
    }
    if ins == 0 || outs == 0 {
        0
    } else {
        ins + outs - 1
    }
}

fn finished(what: &str, body: topo::Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"))
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

/// One body's facts at `at`: vertices there, shells and solids, mesh.
fn facts(bb: &topo::BooleanBody<f64>, at: V3) -> String {
    let body = &bb.body;
    let here = body
        .vertex_points()
        .filter(|(_, p)| ((p.x - at[0]).powi(2) + (p.y - at[1]).powi(2) + (p.z - at[2]).powi(2)).sqrt() < 1e-9)
        .count();
    let solids = body.solids().count();
    let shells = body.shells().count();
    let mesh = match mesh::tessellate(body, 0.05, tol()) {
        Ok(m) => match mesh::validate::check_mesh(&m) {
            Ok(()) => "mesh=ok".to_string(),
            Err(e) => format!("mesh=BAD({e:?})"),
        },
        Err(e) => {
            let s = format!("{e:?}");
            format!("mesh=refused({})", s.chars().take(50).collect::<String>())
        }
    };
    format!("verts@p={here} solids={solids} shells={shells} {mesh}")
}

/// Every op both orders at one pose; prints one line per run.
fn tri_cone_pose(tag: &str, e: V3, len: f64, r: &[V3; 3], t: V3) {
    let a_cols = [[2.0, 0.0, 0.0], [0.0, 2.0, 0.0], [0.0, 0.0, 2.0]];
    let b_cols = [
        [-e[0], 1.0, 1.0].map(|x| x * len),
        [1.0, -e[1], 1.0].map(|x| x * len),
        [1.0, 1.0, -e[2]].map(|x| x * len),
    ];
    let body_of = |cols: [V3; 3]| {
        let r = *r;
        fixtures::mapped_cube::<f64>(
            move |x, y, z| {
                let p = [0, 1, 2].map(|k| x * cols[0][k] + y * cols[1][k] + z * cols[2][k]);
                let q = apply(&r, p);
                Point3::new(q[0] + t[0], q[1] + t[1], q[2] + t[2])
            },
            tol(),
        )
    };
    let a = finished("A", body_of(a_cols));
    let b = finished("B", body_of(b_cols));
    let (ap, bp) = (moved(&ppd_planes(a_cols), r, t), moved(&ppd_planes(b_cols), r, t));
    let va = convex_volume(&ap);
    let vb = convex_volume(&bp);
    let common = convex_volume(&[ap.clone(), bp.clone()].concat());
    let decls = BooleanDeclarations::default();
    for (order, x, y, px, py, vx) in [("ab", &a, &b, &ap, &bp, va), ("ba", &b, &a, &bp, &ap, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, run, want) in ops {
            let cones = cones_at(t, 1e-3, 120, &|p| member(op, inside(px, p), inside(py, p)));
            let res = run(x, y, &decls, tol());
            let f = res
                .as_ref()
                .ok()
                .and_then(BooleanResult::body)
                .map(|bb| facts(bb, t))
                .unwrap_or_default();
            println!("R2C {tag} {order} {op} cones={cones} {f} => {}", outcome(res, want, tol()));
        }
    }
}

/// The tri-cone poses: sign patterns of `e`, magnitudes from 0.3 down to
/// near-tangent, several rigid turns.
#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn r2_pinch_cones_battery() {
    let turns: Vec<([V3; 3], V3, &str)> = vec![
        (rot([0.0, 0.0, 1.0], 0.0), [0.0; 3], "id"),
        (rot([0.3, 0.9, -0.2], 0.71), [0.4, -0.3, 1.1], "t1"),
        (rot([-0.8, 0.1, 0.55], 2.3), [-1.0, 0.25, 0.5], "t2"),
        (rot([0.2, -0.4, 0.9], 4.1), [0.1, 0.2, 0.3], "t3"),
    ];
    let mags = [0.3, 0.05, 1e-3, 1e-6];
    let signs: [[f64; 3]; 4] = [[1.0, 1.0, 1.0], [1.0, 1.0, -1.0], [1.0, -1.0, -1.0], [-1.0, -1.0, -1.0]];
    let lens = [1.5, 3.0];
    for (r, t, tn) in &turns {
        for m in mags {
            for s in signs {
                for len in lens {
                    // Uneven magnitudes so no two cones mirror each other.
                    let e = [s[0] * m, s[1] * m * 1.37, s[2] * m * 0.71];
                    let tag = format!("{tn} m={m} s={:?} L={len}", s.map(|x| x as i32));
                    tri_cone_pose(&tag, e, len, r, *t);
                }
            }
        }
    }
}

/// One pose by `R2C_CASE="<turn 0..3> <m> <s0> <s1> <s2> <L>"`.
#[test]
#[ignore = "review probe detail"]
fn r2_pinch_cones_detail() {
    let w: Vec<f64> = std::env::var("R2C_CASE")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let turns = [
        (rot([0.0, 0.0, 1.0], 0.0), [0.0; 3]),
        (rot([0.3, 0.9, -0.2], 0.71), [0.4, -0.3, 1.1]),
        (rot([-0.8, 0.1, 0.55], 2.3), [-1.0, 0.25, 0.5]),
        (rot([0.2, -0.4, 0.9], 4.1), [0.1, 0.2, 0.3]),
    ];
    let (r, t) = turns[w[0] as usize];
    let m = w[1];
    tri_cone_pose("detail", [w[2] * m, w[3] * m * 1.37, w[4] * m * 0.71], w[5], &r, t);
}

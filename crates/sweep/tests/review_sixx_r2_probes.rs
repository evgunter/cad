//! Review r2 probes for PR 4050 (a six-crossing vertex pair nests its
//! pairing in B). A prism's top corner (notches of several angles,
//! mirrored, wedges) meets a cube's edge or corner, or a second posed
//! prism's corner, every op in both orders. Each body is read by
//! [`outcome`] against a kernel-free oracle: each operand is a signed
//! sum of convex pieces (half-spaces), and the common volume is the sum
//! of the pairwise clipped volumes. Each line also carries the number
//! of crossings of the two links, counted here on the sphere from the
//! face arcs, independently of the kernel.
//!
//! `SX_SWEEP=<grid|near|pair|hex> SX_SHAPES=<a,b,..> [SX_MIN=6]
//! cargo test -p sweep --release --test all review_sixx -- --ignored --nocapture`;
//! lines go to `SX_OUT` (default stdout).

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::type_complexity,
    dead_code
)]

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult};

use crate::common::differential::outcome;
use crate::join_pierce_runs_sweep::{convex_volume, frame};

const SIDE: f64 = 4.0;
const TAU: f64 = std::f64::consts::TAU;

fn tol() -> Tol {
    Tol::witness()
}

type V3 = [f64; 3];
type Half = (V3, f64);

fn unit(m: V3) -> V3 {
    let n = m.iter().map(|c| c * c).sum::<f64>().sqrt();
    m.map(|c| c / n)
}
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
fn rot(x: V3, k: V3, ang: f64) -> V3 {
    let k = unit(k);
    let (c, s) = (ang.cos(), ang.sin());
    let kx = cross(k, x);
    let kd = dot(k, x);
    [0, 1, 2].map(|i| x[i] * c + kx[i] * s + k[i] * kd * (1.0 - c))
}
/// `R x` where `R`'s columns are the frame's axes.
fn apply(f: [V3; 3], x: V3) -> V3 {
    [0, 1, 2].map(|i| f[0][i] * x[0] + f[1][i] * x[1] + f[2][i] * x[2])
}

/// A link arc on the unit sphere: start, unit normal, angle swept
/// counterclockwise about the normal.
type Arc = (V3, V3, f64);

fn arc(a: V3, b: V3, n: V3) -> Arc {
    let (a, b, n) = (unit(a), unit(b), unit(n));
    let ang = dot(cross(a, b), n).atan2(dot(a, b)).rem_euclid(TAU);
    (a, n, ang)
}

/// Where along `arc` the point `p` (on its circle) lies, in [0, τ).
fn along((s, n, _): Arc, p: V3) -> f64 {
    dot(cross(s, p), n).atan2(dot(s, p)).rem_euclid(TAU)
}

/// Crossings of two links, and the smallest angular margin of any arc
/// point from an arc end (near zero: a near-tangent pose).
fn crossings(la: &[Arc], lb: &[Arc]) -> (usize, f64) {
    let mut count = 0;
    let mut margin = f64::INFINITY;
    for &a in la {
        for &b in lb {
            let q = cross(a.1, b.1);
            let len = dot(q, q).sqrt();
            if len < 1e-12 {
                margin = 0.0;
                continue;
            }
            for sgn in [1.0, -1.0] {
                let p = q.map(|c| sgn * c / len);
                let (ta, tb) = (along(a, p), along(b, p));
                let on_a = ta <= a.2;
                let on_b = tb <= b.2;
                let m = ta.min(a.2 - ta).abs().min(tb.min(b.2 - tb).abs());
                let m = if on_a && on_b {
                    m
                } else {
                    // Distance outside: how far from becoming a crossing.
                    let da = if on_a { 0.0 } else { (ta - a.2).min(TAU - ta) };
                    let db = if on_b { 0.0 } else { (tb - b.2).min(TAU - tb) };
                    da.max(db)
                };
                margin = margin.min(m);
                if on_a && on_b {
                    count += 1;
                }
            }
        }
    }
    (count, margin)
}

/// A prism with its top corner at `v` (world), possibly posed.
#[derive(Clone)]
struct Shape {
    name: String,
    profile: Vec<(f64, f64)>,
    /// The corner's index in the profile.
    c: usize,
}

impl Shape {
    fn corner(&self) -> V3 {
        let (x, y) = self.profile[self.c];
        [x, y, 1.0]
    }
    /// Posed: local x ↦ f (x − corner) + at.
    fn map(&self, f: [V3; 3], at: V3) -> impl Fn(V3) -> V3 + '_ {
        let c = self.corner();
        move |x| {
            let r = apply(f, [x[0] - c[0], x[1] - c[1], x[2] - c[2]]);
            [r[0] + at[0], r[1] + at[1], r[2] + at[2]]
        }
    }
    fn body(&self, f: [V3; 3], at: V3) -> AtRestBody<f64> {
        let mut b = Body::<f64>::new();
        let m = self.map(f, at);
        fixtures::prism_ops(
            &mut b,
            &self.profile,
            (0.0, 1.0),
            |x, y, z| {
                let p = m([x, y, z]);
                Point3::new(p[0], p[1], p[2])
            },
            fixtures::FaceGeometry::Certified,
            tol(),
        );
        fixtures::describe_as_intersections(&mut b, tol());
        AtRestBody::validate(b, tol()).unwrap_or_else(|e| panic!("{}: {e:?}", self.name))
    }
    /// Convex pieces: triangles fanned from the corner (each profile here
    /// is star-shaped from it), posed.
    fn pieces(&self, f: [V3; 3], at: V3) -> Vec<Vec<Half>> {
        let n = self.profile.len();
        let p = |k: usize| self.profile[(self.c + k) % n];
        let c = self.corner();
        let mut out = Vec::new();
        for k in 1..n - 1 {
            let tri = [p(0), p(k), p(k + 1)];
            let mut hs: Vec<Half> = vec![([0.0, 0.0, -1.0], 0.0), ([0.0, 0.0, 1.0], 1.0)];
            for e in 0..3 {
                let (a, b) = (tri[e], tri[(e + 1) % 3]);
                let nn = [b.1 - a.1, -(b.0 - a.0), 0.0];
                hs.push((nn, nn[0] * a.0 + nn[1] * a.1));
            }
            // Pose: (R n)·x ≤ d − n·c + (R n)·at.
            let posed = hs
                .into_iter()
                .map(|(nn, d)| {
                    let rn = apply(f, nn);
                    (rn, d - dot(nn, c) + dot(rn, at))
                })
                .collect();
            out.push(posed);
        }
        out
    }
    fn link(&self, f: [V3; 3]) -> Vec<Arc> {
        let n = self.profile.len();
        let (cx, cy) = self.profile[self.c];
        let (px, py) = self.profile[(self.c + n - 1) % n];
        let (nx, ny) = self.profile[(self.c + 1) % n];
        let ep = apply(f, [px - cx, py - cy, 0.0]);
        let en = apply(f, [nx - cx, ny - cy, 0.0]);
        let d = apply(f, [0.0, 0.0, -1.0]);
        let up = apply(f, [0.0, 0.0, 1.0]);
        vec![
            arc(en, ep, up),
            arc(en, d, cross(en, d)),
            arc(ep, d, cross(ep, d)),
        ]
    }
}

/// The CCW profile of a wedge of `[−2, 2]²` between rays at `twist` and
/// `twist + alpha` degrees, corner first (as review r2 of PR 4036).
fn wedge(name: &str, alpha: f64, twist: f64) -> Shape {
    let at = |t: f64| {
        let t = (t + twist).to_radians();
        let r = 2.0 / t.cos().abs().max(t.sin().abs());
        (r * t.cos(), r * t.sin())
    };
    let mut prof = vec![(0.0, 0.0), at(0.0)];
    let first = (45.0 - twist).rem_euclid(90.0);
    let mut c = first;
    while c < alpha - 1e-9 {
        if c > 1e-9 {
            prof.push(at(c));
        }
        c += 90.0;
    }
    prof.push(at(alpha));
    Shape {
        name: name.into(),
        profile: prof,
        c: 0,
    }
}

/// The PR's notch343 family: the square `[0, 2]²` notched to `(1, 1)`
/// between `(2, 1 − h)` and `(2, 1 + h)`.
fn notch(name: &str, h: f64) -> Shape {
    Shape {
        name: name.into(),
        profile: vec![
            (0.0, 0.0),
            (2.0, 0.0),
            (2.0, 1.0 - h),
            (1.0, 1.0),
            (2.0, 1.0 + h),
            (2.0, 2.0),
            (0.0, 2.0),
        ],
        c: 3,
    }
}

/// Mirrored through y: reversed to stay counterclockwise.
fn mirrored(s: Shape) -> Shape {
    let n = s.profile.len();
    let profile: Vec<(f64, f64)> = (0..n)
        .map(|k| s.profile[(n - k) % n])
        .map(|(x, y)| (x, -y))
        .collect();
    let c = (n - s.c) % n;
    Shape {
        name: format!("m{}", s.name),
        profile,
        c,
    }
}

fn shape(name: &str) -> Shape {
    if let Some(rest) = name.strip_prefix('m') {
        return mirrored(shape(rest));
    }
    match name {
        "notch343" => notch(name, 0.15),
        "notch330" => notch(name, 0.27),
        "notch352" => notch(name, 0.07),
        "notch300" => notch(name, 0.58),
        "notch" => wedge(name, 345.0, 0.0),
        "notch5" => wedge(name, 355.0, 10.0),
        "reflex315" => wedge(name, 315.0, 0.0),
        "w300" => wedge(name, 300.0, 17.0),
        "w335" => wedge(name, 335.0, 31.0),
        "w350" => wedge(name, 350.0, 4.0),
        "w359" => wedge(name, 359.0, 22.0),
        "L" => wedge(name, 270.0, 0.0),
        _ => panic!("unknown shape {name}"),
    }
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

/// B: a cube placed with its edge midpoint or corner on A's corner, or a
/// second shape's corner.
#[derive(Clone)]
enum Other {
    Cube { edge: bool },
    Shape(Shape),
}

fn cube_link(f: [V3; 3], edge: bool) -> Vec<Arc> {
    let [u, w, m] = f;
    if edge {
        // Vertex on the edge along w; the two faces hold w and u, w and m.
        let wm = w.map(|c| -c);
        vec![arc(w, wm, cross(w, u)), arc(w, wm, cross(w, m))]
            .into_iter()
            .zip([u, m])
            .map(|(a, mid)| {
                if along(a, mid) <= a.2 {
                    a
                } else {
                    (a.0, a.1.map(|c| -c), a.2)
                }
            })
            .collect()
    } else {
        vec![
            arc(u, w, cross(u, w)),
            arc(w, m, cross(w, m)),
            arc(m, u, cross(m, u)),
        ]
    }
}

fn cube_parts(v: V3, f: [V3; 3], edge: bool) -> (AtRestBody<f64>, Vec<Half>) {
    let lo: V3 = if edge { [0.0, -2.0, 0.0] } else { [0.0; 3] };
    let [u, w, m] = f;
    let body = fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (lo[0] + SIDE * x, lo[1] + SIDE * y, lo[2] + SIDE * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    );
    let mut planes = Vec::new();
    for (axis, &dir) in f.iter().enumerate() {
        let base = dot(dir, v);
        planes.push((dir.map(|c| -c), -(base + lo[axis])));
        planes.push((dir, base + lo[axis] + SIDE));
    }
    (AtRestBody::validate(body, tol()).unwrap(), planes)
}

const IDENT: [V3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

/// A pose's crossing count and margin, without building anything.
fn count(a: &Shape, other: &Other, f: [V3; 3]) -> (usize, f64) {
    let la = a.link(IDENT);
    let lb = match other {
        Other::Cube { edge } => cube_link(f, *edge),
        Other::Shape(s) => s.link(f),
    };
    crossings(&la, &lb)
}

fn pose_lines(a: &Shape, other: &Other, f: [V3; 3], what: &str) -> Vec<String> {
    let v = a.corner();
    let (n, margin) = count(a, other, f);
    let ab = a.body(IDENT, v);
    let ap: Vec<(f64, Vec<Half>)> = a.pieces(IDENT, v).into_iter().map(|p| (1.0, p)).collect();
    let (bb, bp, bname): (AtRestBody<f64>, Vec<(f64, Vec<Half>)>, String) = match other {
        Other::Cube { edge } => {
            let (b, planes) = cube_parts(v, f, *edge);
            (
                b,
                vec![(1.0, planes)],
                if *edge { "edge" } else { "corner" }.into(),
            )
        }
        Other::Shape(s) => (
            s.body(f, v),
            s.pieces(f, v).into_iter().map(|p| (1.0, p)).collect(),
            s.name.clone(),
        ),
    };
    let vol = |ps: &[(f64, Vec<Half>)]| ps.iter().map(|(s, p)| s * convex_volume(p)).sum::<f64>();
    let (va, vb) = (vol(&ap), vol(&bp));
    let mut common = 0.0;
    for (s, p) in &ap {
        for (t, q) in &bp {
            let mut all = p.clone();
            all.extend_from_slice(q);
            common += s * t * convex_volume(&all);
        }
    }
    let d = BooleanDeclarations::default();
    let mut out = Vec::new();
    for (order, x, y, vx) in [("ab", &ab, &bb, va), ("ba", &bb, &ab, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, run, want) in ops {
            let tag = format!("{} {bname} {what} n={n} {order} {op}", a.name);
            if std::env::var("SX_THREADS").is_ok_and(|t| t == "1") {
                eprintln!("SXRUN {tag}");
            }
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(x, y, &d, tol())));
            let line = match r {
                Ok(r) => {
                    let why = match &r {
                        Ok(res) => res.body().and_then(|bb| {
                            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                                .err()
                                .map(|e| format!(" T3WHY {e:?}"))
                        }),
                        Err(_) => None,
                    };
                    let l = outcome(r, want, tol()) + &why.unwrap_or_default();
                    if std::env::var("SX_THREADS").is_ok_and(|t| t == "1") {
                        eprintln!("SXRES {l}");
                    }
                    l
                }
                Err(_) => "PANIC".into(),
            };
            out.push(format!("{tag} m={margin:.1e}: {line}"));
        }
    }
    out
}

fn grid_dir(i: u32, j: u32) -> V3 {
    let theta = TAU * (f64::from(i) + 0.11) / 12.0;
    let phi = (f64::from(j) - 3.0) * 0.43 + 0.02;
    [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]
}

/// The poses of a sweep for shape `a` against `other`, filtered to at
/// least `min` crossings: `(frame, label)`.
fn poses(sweep: &str, a: &Shape, other: &Other, min: usize) -> Vec<([V3; 3], String)> {
    let mut out = Vec::new();
    let keep = |f: [V3; 3]| count(a, other, f).0 >= min;
    match sweep {
        "grid" => {
            for i in 0..24u32 {
                for j in 0..13u32 {
                    let theta = TAU * (f64::from(i) + 0.11) / 24.0;
                    let phi = (f64::from(j) - 6.0) * 0.23 + 0.02;
                    let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
                    for k in 0..24u32 {
                        let psi = (f64::from(k) + 0.3) * TAU / 24.0;
                        let f = frame(m, psi);
                        if keep(f) {
                            out.push((f, format!("i={i} j={j} k={k}")));
                        }
                    }
                }
            }
        }
        "near" => {
            // Bisect every change of the crossing count along psi to a
            // tangency, then pose just either side of it.
            for i in 0..24u32 {
                for j in 0..13u32 {
                    let theta = TAU * (f64::from(i) + 0.11) / 24.0;
                    let phi = (f64::from(j) - 6.0) * 0.23 + 0.02;
                    let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
                    let steps = 96u32;
                    let at = |k: f64| frame(m, k * TAU / f64::from(steps));
                    for k in 0..steps {
                        let (k0, k1) = (f64::from(k), f64::from(k + 1));
                        let (c0, c1) = (count(a, other, at(k0)).0, count(a, other, at(k1)).0);
                        if c0 == c1 || c0.max(c1) < min {
                            continue;
                        }
                        let (mut lo, mut hi) = (k0, k1);
                        for _ in 0..60 {
                            let mid = 0.5 * (lo + hi);
                            if count(a, other, at(mid)).0 == c0 {
                                lo = mid;
                            } else {
                                hi = mid;
                            }
                        }
                        let star = 0.5 * (lo + hi) * TAU / f64::from(steps);
                        for e in [1e-3, 1e-5, 1e-7, 1e-9] {
                            for s in [-1.0, 1.0] {
                                let f = frame(m, star + s * e);
                                if keep(f) {
                                    out.push((f, format!("i={i} j={j} k={k} d={:+e}", s * e)));
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => panic!("unknown sweep {sweep}"),
    }
    out
}

fn other(name: &str) -> Other {
    match name {
        "edge" => Other::Cube { edge: true },
        "corner" => Other::Cube { edge: false },
        s => Other::Shape(shape(s)),
    }
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn review_sixx_sweep() {
    let sweep = std::env::var("SX_SWEEP").unwrap_or_else(|_| "grid".into());
    let names = std::env::var("SX_SHAPES").unwrap_or_else(|_| "notch343".into());
    let others = std::env::var("SX_OTHERS").unwrap_or_else(|_| "edge,corner".into());
    let min: usize = std::env::var("SX_MIN")
        .ok()
        .map_or(6, |m| m.parse().unwrap());
    let limit: usize = std::env::var("SX_LIMIT")
        .ok()
        .map_or(usize::MAX, |m| m.parse().unwrap());
    let count_only = std::env::var("SX_COUNT").is_ok();
    let threads = std::env::var("SX_THREADS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(4usize);
    let mut work = Vec::new();
    for name in names.split(',') {
        let a = shape(name);
        for o in others.split(',') {
            let b = other(o);
            let ps = poses(&sweep, &a, &b, min);
            let mut hist = std::collections::BTreeMap::new();
            for (f, _) in &ps {
                *hist.entry(count(&a, &b, *f).0).or_insert(0) += 1;
            }
            eprintln!("{name} vs {o}: {} poses, by n {hist:?}", ps.len());
            // Spread a limited sample evenly.
            let stride = ps.len().div_ceil(limit.max(1)).max(1);
            let only = std::env::var("SX_ONLY").ok();
            for (f, what) in ps.into_iter().step_by(stride) {
                if only.as_ref().is_some_and(|o| &what != o) {
                    continue;
                }
                work.push((a.clone(), b.clone(), f, what));
            }
        }
    }
    if count_only {
        return;
    }
    let next = AtomicUsize::new(0);
    let lines = Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| {
                loop {
                    let k = next.fetch_add(1, Ordering::SeqCst);
                    let Some((a, b, f, what)) = work.get(k) else {
                        break;
                    };
                    let ls = pose_lines(a, b, *f, what);
                    lines.lock().unwrap().extend(ls);
                }
            });
        }
    });
    let mut lines = lines.into_inner().unwrap();
    lines.sort();
    let text = lines.join("\n") + "\n";
    match std::env::var("SX_OUT") {
        Ok(p) => std::fs::write(p, text).unwrap(),
        Err(_) => print!("{text}"),
    }
}

/// The crossing counter agrees with the PR's traced pose: notch343 on
/// the cube's edge at the PR grid's `i=3 j=0 psi=1` crosses six times.
#[test]
fn review_sixx_counter_reads_the_pr_pose() {
    let theta: f64 = TAU * (3.0 + 0.37) / 12.0;
    let phi: f64 = -3.0 * 0.4 + 0.05;
    let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
    let a = shape("notch343");
    assert_eq!(count(&a, &Other::Cube { edge: true }, frame(m, 1.0)).0, 6);
}

fn rest_rows(records: &topo::ContactRecords) -> Vec<topo::CarriedVv> {
    records.carried(topo::ContactClass::Rest).vv
}

/// **The nested plan at a shared vertex.** A pinch: a reflex wedge and a
/// small one touching along the z-axis (two vertices at the top corner),
/// cut by a cube whose corner sits there, crossing the reflex wedge six
/// times and the small one at least twice. Every op, both orders.
#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn review_sixx_pinch() {
    let spans: Vec<(f64, f64)> = std::env::var("SX_PINCH")
        .unwrap_or_else(|_| "0:290,300:40".into())
        .split(',')
        .map(|s| {
            let (a, b) = s.split_once(':').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let limit: usize = std::env::var("SX_LIMIT")
        .ok()
        .map_or(40, |m| m.parse().unwrap());
    // (twist, alpha) wedges.
    let ws: Vec<Shape> = spans
        .iter()
        .enumerate()
        .map(|(k, &(tw, al))| wedge(&format!("p{k}"), al, tw))
        .collect();
    let v = [0.0, 0.0, 1.0];
    let bodies: Vec<AtRestBody<f64>> = ws.iter().map(|w| w.body(IDENT, v)).collect();
    let decl = fixtures::flush_declarations(&bodies[0], &bodies[1], tol());
    let BooleanResult::Body(pinch) =
        topo::union_with(&bodies[0], &bodies[1], &decl, tol()).expect("the pinch builds")
    else {
        panic!("empty pinch");
    };
    eprintln!("pinch contacts vv {}", pinch.contacts.vv.len());
    let carried = rest_rows(&pinch.contacts);
    let pp: Vec<Vec<Half>> = ws.iter().flat_map(|w| w.pieces(IDENT, v)).collect();
    let va: f64 = pp.iter().map(|p| convex_volume(p)).sum();
    let mut found = Vec::new();
    for i in 0..24u32 {
        for j in 0..13u32 {
            let theta = TAU * (f64::from(i) + 0.11) / 24.0;
            let phi = (f64::from(j) - 6.0) * 0.23 + 0.02;
            let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
            for k in 0..24u32 {
                let f = frame(m, (f64::from(k) + 0.3) * TAU / 24.0);
                let lb = cube_link(f, false);
                let n0 = crossings(&ws[0].link(IDENT), &lb).0;
                let n1 = crossings(&ws[1].link(IDENT), &lb).0;
                if n0 >= 6 && n1 >= 2 {
                    found.push((f, format!("i={i} j={j} k={k} n0={n0} n1={n1}")));
                }
            }
        }
    }
    eprintln!("{} pinch poses", found.len());
    let stride = found.len().div_ceil(limit).max(1);
    for (f, what) in found.into_iter().step_by(stride) {
        let (cube, planes) = cube_parts(v, f, false);
        let vb = SIDE * SIDE * SIDE;
        let common: f64 = pp
            .iter()
            .map(|p| {
                let mut all = p.clone();
                all.extend_from_slice(&planes);
                convex_volume(&all)
            })
            .sum();
        let mut ab = fixtures::flush_declarations(&pinch.body, &cube, tol());
        ab.carried_a.vv.clone_from(&carried);
        let mut ba = fixtures::flush_declarations(&cube, &pinch.body, tol());
        ba.carried_b.vv.clone_from(&carried);
        let p = &pinch.body;
        let runs: [(
            &str,
            Op,
            &AtRestBody<f64>,
            &AtRestBody<f64>,
            &BooleanDeclarations,
            f64,
        ); 6] = [
            ("pc U", topo::union_with, p, &cube, &ab, va + vb - common),
            ("pc I", topo::intersect_with, p, &cube, &ab, common),
            ("pc S", topo::subtract_with, p, &cube, &ab, va - common),
            ("cp U", topo::union_with, &cube, p, &ba, va + vb - common),
            ("cp I", topo::intersect_with, &cube, p, &ba, common),
            ("cp S", topo::subtract_with, &cube, p, &ba, vb - common),
        ];
        for (tag, run, x, y, d, want) in runs {
            if std::env::var("SX_THREADS").is_ok_and(|t| t == "1") {
                eprintln!("SXRUN pinch {what} {tag}");
            }
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(x, y, d, tol())));
            let line = match r {
                Ok(r) => {
                    let why = match &r {
                        Ok(res) => res.body().and_then(|bb| {
                            topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol())
                                .err()
                                .map(|e| format!(" T3WHY {e:?}"))
                        }),
                        Err(_) => None,
                    };
                    outcome(r, want, tol()) + &why.unwrap_or_default()
                }
                Err(_) => "PANIC".into(),
            };
            println!("pinch {what} {tag}: {line}");
        }
    }
}

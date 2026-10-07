//! Review r2 probes for PR 4036 (a four-germ vertex pair reads each
//! solid's own walk order). A corner of several shapes — the L prism's
//! reflex corner, a notch, a shallow reflex, convex wedges, a 4-valent
//! pyramid apex and a 4-valent roof-cross valley — placed on a cube's
//! edge or at its corner, every op in both orders, each body read by
//! [`outcome`] against a kernel-free oracle: each shape is a signed sum
//! of convex pieces given as half-spaces, clipped by the cube's six.
//!
//! `R2_SWEEP=<grid|tilt|psi> R2_SHAPES=<a,b,..> cargo test -p sweep
//! --release --test all review_r2 -- --ignored --nocapture`; the lines
//! go to `R2_OUT` (default stdout). `R2_THREADS=1` runs serially and
//! marks each run on stderr, so instrumentation lines attribute.
//! `R2_POSE="edge i=10 j=0 psi=1.3"` runs that one pose.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use geom_core::{Point3, Tol};
use topo::test_support as fixtures;
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult, mass_properties};

use crate::common::differential::outcome;

const SIDE: f64 = 4.0;

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
/// `x` turned by `ang` about the unit `k` (Rodrigues).
fn rot(x: V3, k: V3, ang: f64) -> V3 {
    let k = unit(k);
    let (c, s) = (ang.cos(), ang.sin());
    let kx = cross(k, x);
    let kd = dot(k, x);
    [0, 1, 2].map(|i| x[i] * c + kx[i] * s + k[i] * kd * (1.0 - c))
}

/// As `join_pierce_runs_sweep::frame`.
fn frame(m: V3, psi: f64) -> [V3; 3] {
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

fn cube_body(v: V3, f: [V3; 3], lo: V3) -> Body<f64> {
    let [u, w, m] = f;
    fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (lo[0] + SIDE * x, lo[1] + SIDE * y, lo[2] + SIDE * z);
            Point3::new(
                v[0] + a * u[0] + b * w[0] + c * m[0],
                v[1] + a * u[1] + b * w[1] + c * m[1],
                v[2] + a * u[2] + b * w[2] + c * m[2],
            )
        },
        tol(),
    )
}

fn cube_planes(v: V3, f: [V3; 3], lo: V3) -> Vec<Half> {
    let mut out = Vec::new();
    for (axis, &dir) in f.iter().enumerate() {
        let base = dot(dir, v);
        out.push((dir.map(|c| -c), -(base + lo[axis])));
        out.push((dir, base + lo[axis] + SIDE));
    }
    out
}

/// As `join_pierce_runs_sweep::convex_volume`.
fn convex_volume(planes: &[Half]) -> f64 {
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
        let on: Vec<V3> = pts
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
        let mut ring: Vec<(f64, V3)> = on
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

/// A corner-carrying operand: its body, its corner, and itself as a
/// signed sum of convex pieces.
struct Shape {
    name: String,
    body: AtRestBody<f64>,
    v: V3,
    pieces: Vec<(f64, Vec<Half>)>,
}

impl Shape {
    fn volume_with(&self, extra: &[Half]) -> f64 {
        self.pieces
            .iter()
            .map(|(s, p)| {
                let mut all = p.clone();
                all.extend_from_slice(extra);
                s * convex_volume(&all)
            })
            .sum()
    }
}

fn finished(what: &str, body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).unwrap_or_else(|e| panic!("{what}: {e:?}"))
}

/// The wedge of `[−2, 2]²` between the rays at 0 and `alpha` degrees
/// from the origin, CCW, extruded z ∈ [0, 1]; corner `(0, 0, 1)`.
fn wedge(alpha: f64, twist: f64) -> Shape {
    let at = |t: f64| {
        let t = (t + twist).to_radians();
        let r = 2.0 / t.cos().abs().max(t.sin().abs());
        (r * t.cos(), r * t.sin())
    };
    let mut prof = vec![(0.0, 0.0), at(0.0)];
    // The square's corners strictly between the rays, in angle order.
    let first = (45.0 - twist).rem_euclid(90.0);
    let mut c = first;
    while c < alpha - 1e-9 {
        if c > 1e-9 {
            prof.push(at(c));
        }
        c += 90.0;
    }
    prof.push(at(alpha));
    let body = fixtures::prism::<f64>(&prof, 1.0, tol()).body;
    let mut pieces = Vec::new();
    for k in 1..prof.len() - 1 {
        let tri = [prof[0], prof[k], prof[k + 1]];
        let mut hs: Vec<Half> = vec![([0.0, 0.0, -1.0], 0.0), ([0.0, 0.0, 1.0], 1.0)];
        for e in 0..3 {
            let (p, q) = (tri[e], tri[(e + 1) % 3]);
            let n = [q.1 - p.1, -(q.0 - p.0), 0.0];
            hs.push((n, n[0] * p.0 + n[1] * p.1));
        }
        pieces.push((1.0, hs));
    }
    Shape {
        name: format!("wedge{alpha}"),
        body: finished("the wedge", body),
        v: [0.0, 0.0, 1.0],
        pieces,
    }
}

/// The two roofs whose ridges cross at `(0, 0, 1)`: along x and along y.
fn roofs() -> (Body<f64>, Body<f64>) {
    let tri = [(-2.0, 0.0), (2.0, 0.0), (0.0, 1.0)];
    let mut w1 = Body::<f64>::new();
    fixtures::prism_ops(
        &mut w1,
        &tri,
        (-2.5, 2.5),
        |p, q, t| Point3::new(t, p, q),
        fixtures::FaceGeometry::Certified,
        tol(),
    );
    fixtures::describe_as_intersections(&mut w1, tol());
    let tri2 = [(-2.4, -0.2), (2.4, -0.2), (0.0, 1.0)];
    let mut w2 = Body::<f64>::new();
    fixtures::prism_ops(
        &mut w2,
        &tri2,
        (-2.5, 2.5),
        |p, q, t| Point3::new(p, -t, q),
        fixtures::FaceGeometry::Certified,
        tol(),
    );
    fixtures::describe_as_intersections(&mut w2, tol());
    (w1, w2)
}

fn roof_halves(along_x: bool) -> Vec<Half> {
    let (a, b) = if along_x {
        ([0.0, 0.5, 1.0], [0.0, -0.5, 1.0])
    } else {
        ([0.5, 0.0, 1.0], [-0.5, 0.0, 1.0])
    };
    let (ext, lat) = if along_x { (0, 1) } else { (1, 0) };
    let (half, floor) = if along_x { (2.0, 0.0) } else { (2.4, 0.2) };
    let mut e = [0.0; 3];
    e[ext] = 1.0;
    let mut l = [0.0; 3];
    l[lat] = 1.0;
    vec![
        (a, 1.0),
        (b, 1.0),
        ([0.0, 0.0, -1.0], floor),
        (e, 2.5),
        (e.map(|c| -c), 2.5),
        (l, half),
        (l.map(|c| -c), half),
    ]
}

/// The pyramid apex (roof ∩ roof, 4-valent, convex) or the cross
/// valley (roof ∪ roof, 4-valent at the ridge crossing).
fn roof_cross(union: bool) -> Option<Shape> {
    let (w1, w2) = roofs();
    let (w1, w2) = (finished("roof x", w1), finished("roof y", w2));
    let d = BooleanDeclarations::default();
    let r = if union {
        topo::union_with(&w1, &w2, &d, tol())
    } else {
        topo::intersect_with(&w1, &w2, &d, tol())
    };
    let body = match r {
        Ok(r) => r.body()?.body.clone(),
        Err(e) => {
            eprintln!("roof cross union={union} refused: {e:?}");
            return None;
        }
    };
    let (hx, hy) = (roof_halves(true), roof_halves(false));
    let both: Vec<Half> = hx.iter().chain(&hy).copied().collect();
    let pieces = if union {
        vec![(1.0, hx), (1.0, hy), (-1.0, both)]
    } else {
        vec![(1.0, both)]
    };
    let s = Shape {
        name: if union { "valley4" } else { "apex4" }.into(),
        body,
        v: [0.0, 0.0, 1.0],
        pieces,
    };
    let kv = mass_properties(&s.body, tol()).unwrap().volume;
    let ov = s.volume_with(&[]);
    assert!((kv - ov).abs() < 1e-9, "{} oracle {ov} kernel {kv}", s.name);
    Some(s)
}

fn shape(name: &str) -> Option<Shape> {
    match name {
        "L" => Some(wedge(270.0, 0.0)),
        "notch" => Some(wedge(345.0, 0.0)),
        "notch5" => Some(wedge(355.0, 10.0)),
        "reflex315" => Some(wedge(315.0, 0.0)),
        "shallow" => Some(wedge(195.0, 0.0)),
        "flat181" => Some(wedge(181.0, 7.0)),
        "flat180" => Some(wedge(180.0, 0.0)),
        "flat180t" => Some(wedge(180.0, 13.0)),
        "flat179" => Some(wedge(179.0, 3.0)),
        "convex90" => Some(wedge(90.0, 20.0)),
        "box" => Some(wedge(90.0, 0.0)),
        "convex120" => Some(wedge(120.0, 0.0)),
        "acute20" => Some(wedge(20.0, 5.0)),
        "apex4" => roof_cross(false),
        "valley4" => roof_cross(true),
        _ => panic!("unknown shape {name}"),
    }
}

/// The axis of a shape's corner cone, where it has an obvious one.
fn center(name: &str) -> Option<V3> {
    match name {
        "box" => Some([1.0, 1.0, -1.0]),
        "apex4" | "valley4" => Some([0.0, 0.0, -1.0]),
        "acute20" => Some([0.94, 0.17, -0.5]),
        "convex120" => Some([0.5, 0.87, -1.0]),
        _ => None,
    }
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

const EDGE: V3 = [0.0, -2.0, 0.0];
const CORNER: V3 = [0.0, 0.0, 0.0];

/// The cube's low corner, in its own frame, for a placement name. The
/// `rv-*` placements (review of PR 4272) slide the valley corner along
/// the cube's edge or push it generically off it.
fn place_lo(place: &str) -> V3 {
    match place {
        "edge" => EDGE,
        "corner" => CORNER,
        "rv-e1" => [0.0, -1.0, 0.0],
        "rv-e3" => [0.0, -3.5, 0.0],
        "rv-in" => [0.3, -2.0, 0.3],
        "rv-out" => [-0.3, -2.0, -0.3],
        "rv-mix" => [0.3, -2.0, -0.3],
        _ => panic!("unknown placement {place}"),
    }
}

/// One pose's six runs: `(tag, line)`.
fn pose_lines(s: &Shape, place: &str, f: [V3; 3], what: &str) -> Vec<String> {
    let lo = place_lo(place);
    let cube = finished("the cube", cube_body(s.v, f, lo));
    let va = s.volume_with(&[]);
    let vb = SIDE * SIDE * SIDE;
    let common = s.volume_with(&cube_planes(s.v, f, lo));
    let d = BooleanDeclarations::default();
    let mut out = Vec::new();
    for (order, x, y, vx) in [("sc", &s.body, &cube, va), ("cs", &cube, &s.body, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, run, want) in ops {
            let tag = format!("{} {place} {what} {order} {op}", s.name);
            if serial() {
                eprintln!("R2RUN {tag}");
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
                    outcome(r, want, tol()) + &why.unwrap_or_default()
                }
                Err(_) => "PANIC".into(),
            };
            out.push(format!("{tag}: {line}"));
        }
    }
    out
}

fn serial() -> bool {
    std::env::var("R2_THREADS")
        .map(|t| t == "1")
        .unwrap_or(false)
}

/// The poses of one sweep: `(place, frame, label)`.
fn poses(sweep: &str, shape_center: Option<V3>) -> Vec<(&'static str, [V3; 3], String)> {
    let mut out = Vec::new();
    match sweep {
        "grid" => {
            for place in ["edge", "corner"] {
                for i in 0..12u32 {
                    for j in 0..7u32 {
                        let theta = std::f64::consts::TAU * (f64::from(i) + 0.11) / 12.0;
                        let phi = (f64::from(j) - 3.0) * 0.43 + 0.02;
                        let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
                        for psi in [0.3, 1.3, 2.5, 3.7, 4.9] {
                            out.push((place, frame(m, psi), format!("i={i} j={j} psi={psi}")));
                        }
                    }
                }
            }
        }
        "tilt" => {
            // The 24 proper axis-aligned frames, each tilted by eps about
            // two fixed generic axes.
            let axes: [V3; 6] = [
                [1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, -1.0],
            ];
            let mut frames = Vec::new();
            for &u in &axes {
                for &w in &axes {
                    if dot(u, w).abs() > 0.5 {
                        continue;
                    }
                    frames.push([u, w, cross(u, w)]);
                }
            }
            assert_eq!(frames.len(), 24);
            for place in ["edge", "corner"] {
                for (k, fr) in frames.iter().enumerate() {
                    out.push((place, *fr, format!("frame={k} eps=0")));
                    for eps in [1e-2, 1e-4, 1e-6, 1e-8] {
                        for (t, ax) in [[0.3, 0.8, 0.52], [-0.7, 0.2, 0.68]].iter().enumerate() {
                            let f = fr.map(|x| rot(x, *ax, eps));
                            out.push((place, f, format!("frame={k} eps={eps:e} ax={t}")));
                        }
                    }
                }
            }
        }
        "psi" => {
            // Fine rotation about the cube's own axis at the PR's
            // four-crossing directions and a few of the grid's.
            let dirs: [(&'static str, u32, u32); 4] = [
                ("corner", 0, 0),
                ("edge", 2, 0),
                ("edge", 6, 0),
                ("corner", 3, 1),
            ];
            for (place, i, j) in dirs {
                let theta = std::f64::consts::TAU * (f64::from(i) + 0.37) / 12.0;
                let phi = (f64::from(j) - 3.0) * 0.4 + 0.05;
                let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
                for k in 0..120 {
                    let psi = f64::from(k) * std::f64::consts::TAU / 120.0;
                    out.push((place, frame(m, psi), format!("i={i} j={j} psik={k}")));
                }
            }
        }
        "rv" => {
            // Review of PR 4272: the grid's directions near the
            // valley4 witness, at placements off the plain edge.
            for place in ["rv-e1", "rv-e3", "rv-in", "rv-out", "rv-mix"] {
                for i in 0..12u32 {
                    for j in 0..3u32 {
                        let theta = std::f64::consts::TAU * (f64::from(i) + 0.11) / 12.0;
                        let phi = (f64::from(j) - 3.0) * 0.43 + 0.02;
                        let m = [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()];
                        for psi in [0.3, 1.3, 3.7] {
                            out.push((place, frame(m, psi), format!("i={i} j={j} psi={psi}")));
                        }
                    }
                }
            }
        }
        "hex" => {
            // The cube's corner on the shape's corner, its diagonal
            // along `center` (or tilted off it), turned about it: two
            // corners sharing a cone axis cross up to six times.
            let center: V3 = match shape_center {
                Some(c) => unit(c),
                None => unit([-0.3, -0.3, -1.0]),
            };
            let diag = unit([1.0, 1.0, 1.0]);
            for (t, tilt) in [[0.0, 0.0, 0.0], [0.05, -0.02, 0.0], [-0.03, 0.06, 0.01]]
                .iter()
                .enumerate()
            {
                let c = unit([0, 1, 2].map(|i| center[i] + tilt[i]));
                let ax = cross(diag, c);
                let ang = dot(diag, c).clamp(-1.0, 1.0).acos();
                let base: [V3; 3] = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]].map(|e| {
                    if ang.abs() < 1e-12 {
                        e
                    } else {
                        rot(e, ax, ang)
                    }
                });
                for k in 0..36 {
                    let phi = (f64::from(k) * 10.0 + 3.0).to_radians();
                    let f = base.map(|e| rot(e, c, phi));
                    out.push(("corner", f, format!("t={t} phik={k}")));
                }
            }
        }
        _ => panic!("unknown sweep {sweep}"),
    }
    out
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn review_r2_vv_sweep() {
    let sweep = std::env::var("R2_SWEEP").unwrap_or_else(|_| "grid".into());
    let names = std::env::var("R2_SHAPES").unwrap_or_else(|_| "L".into());
    let shard: Option<(usize, usize)> = std::env::var("R2_SHARD").ok().map(|s| {
        let (k, n) = s.split_once('/').unwrap();
        (k.parse().unwrap(), n.parse().unwrap())
    });
    let pose = std::env::var("R2_POSE").ok();
    let threads = std::env::var("R2_THREADS")
        .ok()
        .and_then(|t| t.parse().ok())
        .unwrap_or(4usize);
    let mut work = Vec::new();
    for name in names.split(',') {
        if shape(name).is_none() {
            println!("{name}: SHAPE UNBUILT");
            continue;
        }
        for (k, (place, f, what)) in poses(&sweep, center(name)).into_iter().enumerate() {
            let picked = pose
                .as_ref()
                .is_none_or(|p| *p == format!("{place} {what}"));
            if picked && shard.is_none_or(|(s, n)| k % n == s) {
                work.push((name.to_string(), place, f, what));
            }
        }
    }
    let next = AtomicUsize::new(0);
    let lines = Mutex::new(Vec::new());
    std::thread::scope(|sc| {
        for _ in 0..threads {
            sc.spawn(|| {
                let mut cache: Option<Shape> = None;
                loop {
                    let k = next.fetch_add(1, Ordering::SeqCst);
                    let Some((name, place, f, what)) = work.get(k) else {
                        break;
                    };
                    if cache
                        .as_ref()
                        .is_none_or(|s| &s.name != name && !s.name.is_empty())
                    {
                        cache = shape(name).map(|mut s| {
                            s.name.clone_from(name);
                            s
                        });
                    }
                    let s = cache.as_ref().unwrap();
                    let ls = pose_lines(s, place, *f, what);
                    lines.lock().unwrap().extend(ls);
                }
            });
        }
    });
    let mut lines = lines.into_inner().unwrap();
    lines.sort();
    let text = lines.join("\n") + "\n";
    match std::env::var("R2_OUT") {
        Ok(p) => std::fs::write(p, text).unwrap(),
        Err(_) => print!("{text}"),
    }
}

/// The roof-cross valley on the cube's edge: valley4's roof face is
/// pinched at the valley corner, and the cube's two edges pierce it
/// either side of the first chord, so every loose half on both of that
/// chord's arcs has its partner on a pierce ring. Every op in both
/// orders builds, read SOUND against the roofs' convex halves.
#[test]
fn a_roof_cross_valley_on_a_cube_edge_builds() {
    let s = shape("valley4").expect("valley4 builds");
    let (place, f, what) = poses("grid", None)
        .into_iter()
        .find(|(place, _, what)| *place == "edge" && what == "i=10 j=0 psi=1.3")
        .expect("the witness pose");
    let lines = pose_lines(&s, place, f, &what);
    assert_eq!(lines.len(), 6, "{lines:#?}");
    for line in &lines {
        assert!(line.contains(": OK SOUND "), "{line}");
    }
}

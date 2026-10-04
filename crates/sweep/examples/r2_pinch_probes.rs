//! Review r2 of PR 4038: pierce and pinch poses beyond the PR's
//! batteries.
//!
//! A prism corner `v` on a pierced face whose outward normal at `v` is
//! `−m`, the pierced solid on `m`'s side:
//! - `cube`: a side-4 cube with `v` on its face (`face`), on an edge
//!   (`edge`) or at a corner (`corner`), the cube turned by `psi` about
//!   `m`. Oracle: each convex piece of the prism clipped by the cube's
//!   half-spaces, the volume by the divergence theorem.
//! - `cyl`: a cylinder of radius 3 and length 8 whose side holds `v`,
//!   its axis `⊥ m` turned by `psi`. Oracle: each convex piece mapped
//!   into the cylinder's frame, sliced along the axis, each slice's
//!   polygon ∩ disk area exact, integrated by adaptive Gauss–Kronrod.
//!
//! Every op in both orders prints `differential::outcome` (tier 2,
//! tier 3′, the certificate, a legal operand, the volume to 1e-7) and,
//! for a built body, `pts=k` (points holding two or more vertices) and
//! `FACE2V` where a face's loops meet two vertices at one point.
//!
//! `cargo run -p sweep --release --example r2_pinch_probes <set> [prism]`,
//! sets `cube`, `nt` (near-tangent tilts, every placement), `cyl`.
//! `R2P_SHARD=k/n` runs one shard of the prisms × directions.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "../tests/common/differential.rs"]
#[allow(dead_code)]
mod differential;

use differential::outcome;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::test_support as fixtures;
use topo::{
    AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult, LoopBoundary,
    mass_properties,
};

type V3 = [f64; 3];
type Plane = (V3, f64); // n·x ≤ d

fn tol() -> Tol {
    Tol::witness()
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
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scale(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}
fn unit(a: V3) -> V3 {
    scale(a, 1.0 / dot(a, a).sqrt())
}
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}
fn turn(u: V3, w: V3, psi: f64) -> (V3, V3) {
    let (c, s) = (psi.cos(), psi.sin());
    (add(scale(u, c), scale(w, s)), add(scale(u, -s), scale(w, c)))
}

struct Prism {
    name: &'static str,
    profile: Vec<(f64, f64)>,
    pieces: Vec<Vec<(f64, f64)>>,
    h: f64,
    v: V3,
}

impl Prism {
    fn piece_planes(&self) -> Vec<Vec<Plane>> {
        self.pieces
            .iter()
            .map(|poly| {
                let n = poly.len();
                let mut out: Vec<Plane> = (0..n)
                    .map(|i| {
                        let (p, q) = (poly[i], poly[(i + 1) % n]);
                        let nn = [q.1 - p.1, -(q.0 - p.0), 0.0];
                        (nn, nn[0] * p.0 + nn[1] * p.1)
                    })
                    .collect();
                out.push(([0.0, 0.0, 1.0], self.h));
                out.push(([0.0, 0.0, -1.0], 0.0));
                out
            })
            .collect()
    }
    fn inside(&self, x: V3) -> bool {
        self.piece_planes()
            .iter()
            .any(|pl| pl.iter().all(|&(n, d)| dot(n, x) <= d + 1e-15))
    }
    /// Arcs of the plane through `v` normal to `m` inside the cone.
    fn lobes(&self, m: V3) -> usize {
        let (u, w) = basis(m);
        let k = 3600;
        let ins: Vec<bool> = (0..k)
            .map(|i| {
                let t = std::f64::consts::TAU * (i as f64 + 0.5) / k as f64;
                let d = add(scale(u, t.cos()), scale(w, t.sin()));
                self.inside(add(self.v, scale(d, 1e-7)))
            })
            .collect();
        if ins.iter().all(|&b| b) {
            return 99;
        }
        (0..k).filter(|&i| ins[i] && !ins[(i + 1) % k]).count()
    }
    fn corner_edges(&self) -> Vec<V3> {
        let n = self.profile.len();
        let i = self
            .profile
            .iter()
            .position(|&(x, y)| (x - self.v[0]).abs() < 1e-12 && (y - self.v[1]).abs() < 1e-12)
            .unwrap();
        let (p, a, b) = (self.profile[i], self.profile[(i + n - 1) % n], self.profile[(i + 1) % n]);
        let vz = if self.v[2] > 0.5 * self.h { -1.0 } else { 1.0 };
        vec![[a.0 - p.0, a.1 - p.1, 0.0], [b.0 - p.0, b.1 - p.1, 0.0], [0.0, 0.0, vz]]
    }
}

// ---- clipping oracle -------------------------------------------------

type Poly = Vec<Vec<V3>>;

fn clip(poly: &Poly, (n, d): Plane) -> Poly {
    let mut out: Poly = Vec::new();
    let mut cap: Vec<V3> = Vec::new();
    for f in poly {
        let mut g = Vec::new();
        for i in 0..f.len() {
            let (p, q) = (f[i], f[(i + 1) % f.len()]);
            let (sp, sq) = (dot(n, p) - d, dot(n, q) - d);
            if sp <= 0.0 {
                g.push(p);
            }
            if (sp < 0.0 && sq > 0.0) || (sp > 0.0 && sq < 0.0) {
                let t = sp / (sp - sq);
                let x = add(p, scale(sub(q, p), t));
                g.push(x);
                cap.push(x);
            }
            if sp == 0.0 {
                cap.push(p);
            }
        }
        if g.len() >= 3 {
            out.push(g);
        }
    }
    if cap.len() >= 3 {
        let c = scale(cap.iter().fold([0.0; 3], |a, &b| add(a, b)), 1.0 / cap.len() as f64);
        let nu = unit(n);
        let (e1, e2) = basis(nu);
        let mut pts: Vec<(f64, V3)> = cap
            .iter()
            .map(|&p| {
                let r = sub(p, c);
                (dot(r, e2).atan2(dot(r, e1)), p)
            })
            .collect();
        pts.sort_by(|a, b| a.0.total_cmp(&b.0));
        pts.dedup_by(|a, b| dot(sub(a.1, b.1), sub(a.1, b.1)) < 1e-24);
        out.push(pts.into_iter().map(|p| p.1).collect());
    }
    out
}

fn volume(poly: &Poly) -> f64 {
    poly.iter()
        .map(|f| (1..f.len() - 1).map(|i| dot(f[0], cross(f[i], f[i + 1]))).sum::<f64>())
        .sum::<f64>()
        / 6.0
}

/// The cube `v + a u + b w + c m`, a ∈ [a0, a0+4], b ∈ [b0, b0+4], c ∈ [0, 4].
fn cube_poly(v: V3, u: V3, w: V3, m: V3, a0: f64, b0: f64) -> Poly {
    let p = |a: f64, b: f64, c: f64| add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
    let c = |i: usize| {
        let (a, b, z) = ((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64);
        p(a0 + 4.0 * a, b0 + 4.0 * b, 4.0 * z)
    };
    let quads = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]];
    quads.iter().map(|q| q.iter().map(|&i| c(i)).collect()).collect()
}

fn common_cube(pr: &Prism, cube: &Poly) -> f64 {
    pr.piece_planes()
        .iter()
        .map(|pl| volume(&pl.iter().fold(cube.clone(), |acc, &p| clip(&acc, p))))
        .sum()
}

// ---- cylinder oracle -------------------------------------------------

/// Signed area of triangle (0, p, q) ∩ disk(0, r).
fn tri_disk(p: (f64, f64), q: (f64, f64), r: f64) -> f64 {
    let d = (q.0 - p.0, q.1 - p.1);
    let (a, b, c) = (d.0 * d.0 + d.1 * d.1, 2.0 * (p.0 * d.0 + p.1 * d.1), p.0 * p.0 + p.1 * p.1 - r * r);
    let mut ts = vec![0.0, 1.0];
    if a > 0.0 {
        let disc = b * b - 4.0 * a * c;
        if disc > 0.0 {
            let s = disc.sqrt();
            for t in [(-b - s) / (2.0 * a), (-b + s) / (2.0 * a)] {
                if t > 0.0 && t < 1.0 {
                    ts.push(t);
                }
            }
        }
    }
    ts.sort_by(f64::total_cmp);
    let at = |t: f64| (p.0 + t * d.0, p.1 + t * d.1);
    let mut s = 0.0;
    for k in 0..ts.len() - 1 {
        let (x, y) = (at(ts[k]), at(ts[k + 1]));
        let mid = at(0.5 * (ts[k] + ts[k + 1]));
        let cr = x.0 * y.1 - x.1 * y.0;
        if mid.0 * mid.0 + mid.1 * mid.1 <= r * r {
            s += 0.5 * cr;
        } else {
            s += 0.5 * r * r * cr.atan2(x.0 * y.0 + x.1 * y.1);
        }
    }
    s
}

fn clip2(poly: Vec<(f64, f64)>, n: (f64, f64), d: f64) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let k = poly.len();
    for i in 0..k {
        let (p, q) = (poly[i], poly[(i + 1) % k]);
        let (sp, sq) = (n.0 * p.0 + n.1 * p.1 - d, n.0 * q.0 + n.1 * q.1 - d);
        if sp <= 0.0 {
            out.push(p);
        }
        if (sp < 0.0 && sq > 0.0) || (sp > 0.0 && sq < 0.0) {
            let t = sp / (sp - sq);
            out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
        }
    }
    out
}

/// Area of the slice `z = t` of the polytope `planes` (frame coords) ∩ disk(0, r).
fn slice_area(planes: &[Plane], t: f64, r: f64) -> f64 {
    let big = 50.0;
    let mut poly = vec![(-big, -big), (big, -big), (big, big), (-big, big)];
    for &(n, d) in planes {
        let dd = d - n[2] * t;
        if n[0].abs() + n[1].abs() < 1e-14 {
            if dd < 0.0 {
                return 0.0;
            }
            continue;
        }
        poly = clip2(poly, (n[0], n[1]), dd);
        if poly.len() < 3 {
            return 0.0;
        }
    }
    let k = poly.len();
    (0..k).map(|i| tri_disk(poly[i], poly[(i + 1) % k], r)).sum::<f64>().abs()
}

fn gk(f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> (f64, f64) {
    const X: [f64; 8] = [
        0.991455371120813, 0.949107912342759, 0.864864423359769, 0.741531185599394,
        0.586087235467691, 0.405845151377397, 0.207784955007898, 0.0,
    ];
    const WK: [f64; 8] = [
        0.022935322010529, 0.063092092629979, 0.104790010322250, 0.140653259715525,
        0.169004726639267, 0.190350578064785, 0.204432940075298, 0.209482141084728,
    ];
    const WG: [f64; 4] = [0.129484966168870, 0.279705391489277, 0.381830050505119, 0.417959183673469];
    let (c, h) = (0.5 * (a + b), 0.5 * (b - a));
    let (mut k, mut g) = (0.0, 0.0);
    for i in 0..8 {
        let x = X[i];
        let fv = if x == 0.0 { f(c) } else { f(c - h * x) + f(c + h * x) };
        k += WK[i] * fv;
        if i % 2 == 1 {
            g += WG[i / 2] * fv;
        }
    }
    (k * h, (k - g).abs() * h)
}

fn integrate(f: &dyn Fn(f64) -> f64, a: f64, b: f64, eps: f64, depth: u32) -> f64 {
    let (k, e) = gk(f, a, b);
    if e < eps || depth == 0 {
        return k;
    }
    let m = 0.5 * (a + b);
    integrate(f, a, m, eps / 2.0, depth - 1) + integrate(f, m, b, eps / 2.0, depth - 1)
}

// ---- prisms ----------------------------------------------------------

fn prisms() -> Vec<Prism> {
    let l = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let l_pieces = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
    ];
    let notch = vec![(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
    let notch_pieces = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 2.0)],
        vec![(2.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0)],
    ];
    let p = |name, profile: Vec<(f64, f64)>, pieces, v| Prism { name, profile, pieces, h: 1.0, v };
    vec![
        p("Ltop", l.clone(), l_pieces.clone(), [1.0, 1.0, 1.0]),
        p("Lbot", l.clone(), l_pieces.clone(), [1.0, 1.0, 0.0]),
        p("notch307", notch.clone(), notch_pieces.clone(), [2.0, 1.0, 1.0]),
        p("notchbot", notch, notch_pieces, [2.0, 1.0, 0.0]),
        p(
            "shallow200",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6), (0.0, 1.0)],
            vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.6), (0.0, 1.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6)],
            ],
            [2.0, 0.6, 1.0],
        ),
        // A deep vee, ≈300° reflex.
        p(
            "vee300",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)],
            vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.5), (0.0, 4.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5)],
            ],
            [2.0, 0.5, 1.0],
        ),
        // A wide vee, ≈224° reflex, at the bottom.
        p(
            "vee224bot",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.2), (0.0, 1.0)],
            vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.2), (0.0, 1.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.2)],
            ],
            [2.0, 0.2, 0.0],
        ),
        // An asymmetric reflex corner.
        p(
            "asym",
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0), (0.0, 1.5)],
            vec![
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.5)],
                vec![(1.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0)],
            ],
            [1.0, 1.0, 1.0],
        ),
    ]
}

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

/// Points holding two or more vertices, and faces meeting two at one.
fn vertex_check(body: &Body<f64>) -> String {
    let pts: Vec<_> = body.vertex_points().collect();
    let mut clusters: Vec<Vec<topo::VertexKey>> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for (i, &(k, p)) in pts.iter().enumerate() {
        if seen.contains(&i) {
            continue;
        }
        let mut c = vec![k];
        for (j, &(k2, q)) in pts.iter().enumerate().skip(i + 1) {
            if (p - q).norm() < 1e-9 {
                c.push(k2);
                seen.insert(j);
            }
        }
        if c.len() > 1 {
            clusters.push(c);
        }
    }
    let mut f2v = 0;
    for (_, f) in body.faces() {
        let mut met = std::collections::BTreeSet::new();
        for &l in std::iter::once(&f.outer).chain(&f.rings) {
            if let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                for he in body.loop_cycle(first).unwrap() {
                    met.insert(body.get_half_edge(he).unwrap().start);
                }
            }
        }
        if clusters.iter().any(|c| c.iter().filter(|k| met.contains(k)).count() > 1) {
            f2v += 1;
            if std::env::var("R2P_DUMP").is_ok() {
                for c in &clusters {
                    let ps: Vec<_> = c.iter().map(|&k| body.vertex_points().find(|x| x.0 == k).unwrap().1).collect();
                    eprintln!("  cluster {c:?} at {ps:?}");
                }
                let mut loops = Vec::new();
                for &l in std::iter::once(&f.outer).chain(&f.rings) {
                    if let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                        let vs: Vec<_> = body.loop_cycle(first).unwrap().iter().map(|&he| body.get_half_edge(he).unwrap().start).collect();
                        loops.push((l == f.outer, vs));
                    }
                }
                eprintln!("  face loops (outer?, vertices): {loops:?}");
            }
        }
    }
    format!(
        "pts={}{}",
        clusters.len(),
        if f2v > 0 { format!(" FACE2V={f2v}") } else { String::new() }
    )
}

fn run_all(tag: &str, prism: &AtRestBody<f64>, other: &AtRestBody<f64>, va: f64, vb: f64, common: f64) {
    let decls = BooleanDeclarations::default();
    for (order, x, y, vx) in [("pc", prism, other, va), ("cp", other, prism, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, f, want) in ops {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let r = f(x, y, &decls, tol());
                let vc = match &r {
                    Ok(res) => res.body().map(|bb| vertex_check(&bb.body)).unwrap_or_default(),
                    Err(_) => String::new(),
                };
                format!("{} {vc}", outcome(r, want, tol()))
            }))
            .unwrap_or_else(|_| "PANIC".into());
            println!("{tag} {order} {op}: {r}");
        }
    }
}

fn directions(n: usize) -> Vec<(String, V3)> {
    let ga = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    (0..n)
        .map(|i| {
            let z = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
            let r = (1.0 - z * z).sqrt();
            let t = ga * i as f64;
            (format!("fib{i}"), [r * t.cos(), r * t.sin(), z])
        })
        .collect()
}

fn near_tangent(edges: &[V3], ds: &[f64]) -> Vec<(String, V3)> {
    let mut out = Vec::new();
    for (j, &e) in edges.iter().enumerate() {
        let e = unit(e);
        let (p1, p2) = basis(e);
        for a in 0..8 {
            let al = std::f64::consts::TAU * (a as f64 + 0.25) / 8.0;
            let base = add(scale(p1, al.cos()), scale(p2, al.sin()));
            for &d in ds {
                out.push((format!("nt e{j} a{a} d{d:e}"), unit(add(base, scale(e, d)))));
            }
        }
    }
    out
}

fn to_p(p: V3) -> Point3<f64> {
    Point3::new(p[0], p[1], p[2])
}
fn to_v(p: V3) -> Vec3<f64> {
    Vec3::new(p[0], p[1], p[2])
}

fn main() {
    let set = std::env::args().nth(1).unwrap_or_else(|| "cube".into());
    let only = std::env::args().nth(2);
    let shard = std::env::var("R2P_SHARD").unwrap_or_else(|_| "0/1".into());
    let (sk, sn): (usize, usize) = {
        let w: Vec<usize> = shard.split('/').map(|x| x.parse().unwrap()).collect();
        (w[0], w[1])
    };
    let mut idx = 0usize;
    let pick = std::env::var("R2P_PICK").ok();
    for pr in prisms() {
        if only.as_deref().is_some_and(|o| o != pr.name) {
            continue;
        }
        let body = AtRestBody::validate(fixtures::prism::<f64>(&pr.profile, pr.h, tol()).body, tol())
            .unwrap();
        let va = mass_properties(&body, tol()).unwrap().volume;
        let dirs = match set.as_str() {
            "cube" => directions(120),
            "nt" => near_tangent(&pr.corner_edges(), &[1e-3, -1e-3, 1e-6, -1e-6, 1e-8]),
            "cyl" => directions(60),
            _ => panic!("set"),
        };
        for (dn, m) in dirs {
            idx += 1;
            if idx % sn != sk {
                continue;
            }
            if pick.as_deref().is_some_and(|p| p != dn) {
                continue;
            }
            let m = unit(m);
            let lobes = pr.lobes(m);
            let (u0, w0) = basis(m);
            if set == "cyl" {
                for psi in [0.0, 0.9, 2.2] {
                    for (sname, zoff) in [("off", 0.9), ("seam", 0.0)] {
                        cyl_case(&pr, &body, va, m, u0, psi, zoff, &format!("{} cyl {dn} psi={psi} {sname} lobes={lobes}", pr.name));
                    }
                }
                continue;
            }
            for (place, a0, b0, psis) in [
                ("face", -2.0, -2.0, &[0.0][..]),
                ("edge", 0.0, -2.0, &[0.3, 1.9, 4.0][..]),
                ("corner", 0.0, 0.0, &[0.3, 1.9, 4.0][..]),
            ] {
                for &psi in psis {
                    let (u, w) = turn(u0, w0, psi);
                    let v = pr.v;
                    let cube = fixtures::mapped_cube::<f64>(
                        move |x, y, z| {
                            let (a, b, c) = (a0 + 4.0 * x, b0 + 4.0 * y, 4.0 * z);
                            to_p(add(v, add(scale(u, a), add(scale(w, b), scale(m, c)))))
                        },
                        tol(),
                    );
                    let Ok(cube) = AtRestBody::validate(cube, tol()) else {
                        println!("{} {dn} {place} psi={psi}: CUBE-INVALID", pr.name);
                        continue;
                    };
                    let common = common_cube(&pr, &cube_poly(v, u, w, m, a0, b0));
                    run_all(
                        &format!("{} {dn} {place} psi={psi} lobes={lobes}", pr.name),
                        &body,
                        &cube,
                        va,
                        64.0,
                        common,
                    );
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn cyl_case(pr: &Prism, body: &AtRestBody<f64>, va: f64, m: V3, u0: V3, psi: f64, zoff: f64, tag: &str) {
    let r = 3.0;
    let half = 4.0;
    // The axis ⊥ m, turned by psi about m.
    let (a, _) = turn(u0, cross(m, u0), psi);
    let c = add(pr.v, scale(m, r));
    // Rotation taking z to a.
    let z = [0.0, 0.0, 1.0];
    let ax = cross(z, a);
    let ang = dot(z, a).clamp(-1.0, 1.0).acos();
    let r1 = if dot(ax, ax).sqrt() < 1e-12 {
        if dot(z, a) > 0.0 {
            Affine3::identity()
        } else {
            Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), std::f64::consts::PI)
        }
    } else {
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), to_v(unit(ax)), ang)
    };
    // v's azimuth before r1: the junctions sit `zoff` from it.
    let back = r1.inverse().transform_vec(to_v(scale(m, -1.0)));
    let phi = back.y.atan2(back.x);
    let rz = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), phi - zoff);
    let map = Affine3::translation(to_v(c)) * r1 * rz;
    let raw = sweep::test_support::cylinder_of_arcs_at(4, r, Point2::new(0.0, 0.0), -half, 2.0 * half, tol());
    let cyl = match topo::transform_rigid(&raw, &map, tol()).map(|b| AtRestBody::validate(b, tol())) {
        Ok(Ok(b)) => b,
        _ => {
            println!("{tag}: CYL-INVALID");
            return;
        }
    };
    let vb = std::f64::consts::PI * r * r * 2.0 * half;
    // Oracle: each piece in the cylinder's frame, y = M⁻¹ (x − c).
    let common: f64 = pr
        .piece_planes()
        .iter()
        .map(|pl| {
            let fp: Vec<Plane> = pl
                .iter()
                .map(|&(n, d)| {
                    // n·x ≤ d, x = map(y): n·(M y + c) ≤ d
                    let nn = map.inverse().transform_vec(to_v(n));
                    ([nn.x, nn.y, nn.z], d - dot(n, c))
                })
                .collect();
            let f = |t: f64| slice_area(&fp, t, r);
            let k = 64;
            (0..k)
                .map(|i| {
                    let (a0, b0) = (-half + 2.0 * half * i as f64 / k as f64, -half + 2.0 * half * (i + 1) as f64 / k as f64);
                    integrate(&f, a0, b0, 1e-13, 18)
                })
                .sum::<f64>()
        })
        .sum();
    run_all(tag, body, &cyl, va, vb, common);
}

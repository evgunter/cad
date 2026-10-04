//! Review r1 of PR 4038: pierce and pinch poses beyond the PR's
//! batteries (PR 4026's r1 harness, `r1_pierce_probes.rs`, extended).
//!
//! A prism corner `v` on a pierced face whose tangent plane at `v` has
//! normal `m`, the pierced solid on `m`'s side. Every op in both orders
//! prints one `differential::outcome` line (tier 2, tier 3′, the
//! certificate, a legal operand, the volume to 1e-7) against a
//! kernel-free oracle, plus `twov=k`: the number of faces of the built
//! body that run through two distinct vertices at one point (the
//! shared-point ruling allows two vertices at one point only where no
//! face meets both). A line is `BAD` if `outcome` says so or `twov > 0`.
//!
//! Sets:
//! - `cube`: the pierced solid a 4-cube; the corners are r1's six plus
//!   rotated L's (`L{top,bot}_r{deg}`), a 327° notch, a U's inner
//!   corner beside another reflex corner, and `v` mid-way along the L's
//!   reflex vertical edge. Oracle: each convex piece clipped by the
//!   cube's half-spaces, divergence-theorem volume.
//! - `cyl`: the pierced solid a cylinder (radius 5, axis y, a curved
//!   pierced face), `v` on its wall; the prism's corner rotated about
//!   `v`. Oracle: slices in z of each convex piece clipped to the
//!   cylinder's strip, tanh-sinh between the breakpoints.
//!
//! `cargo run -p sweep --release --example r1b_pinch_probes <set> [prism]`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "../tests/common/differential.rs"]
#[allow(dead_code)]
mod differential;

use differential::{area, clip_convex, outcome};
use geom_core::{Point2, Point3, Tol};
use sweep::test_support::finished;
use topo::test_support as fixtures;
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult, LoopBoundary};

type V3 = [f64; 3];
type Plane = (V3, f64); // n·x ≤ d

fn tol() -> Tol {
    Tol::witness()
}
fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
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

#[derive(Clone)]
struct Prism {
    name: String,
    profile: Vec<(f64, f64)>,
    pieces: Vec<Vec<(f64, f64)>>,
    z0: f64,
    z1: f64,
    v: V3,
    /// Edge directions at `v` for the near-tangent tilts.
    edges: Vec<V3>,
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
                out.push(([0.0, 0.0, 1.0], self.z1));
                out.push(([0.0, 0.0, -1.0], -self.z0));
                out
            })
            .collect()
    }
    fn inside(&self, x: V3) -> bool {
        self.piece_planes()
            .iter()
            .any(|pl| pl.iter().all(|&(n, d)| dot(n, x) <= d + 1e-15))
    }
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
    /// Turned `deg` about the vertical through `v`, then moved by `d`.
    fn turned(&self, deg: f64, d: V3, name: String) -> Prism {
        let (c, s) = (deg.to_radians().cos(), deg.to_radians().sin());
        let (cx, cy) = (self.v[0], self.v[1]);
        let f = |&(x, y): &(f64, f64)| {
            let (dx, dy) = (x - cx, y - cy);
            (cx + c * dx - s * dy + d[0], cy + s * dx + c * dy + d[1])
        };
        let r = |e: V3| [c * e[0] - s * e[1], s * e[0] + c * e[1], e[2]];
        Prism {
            name,
            profile: self.profile.iter().map(f).collect(),
            pieces: self.pieces.iter().map(|p| p.iter().map(f).collect()).collect(),
            z0: self.z0 + d[2],
            z1: self.z1 + d[2],
            v: add(self.v, d),
            edges: self.edges.iter().map(|&e| r(e)).collect(),
        }
    }
    fn body(&self) -> AtRestBody<f64> {
        finished(
            "the prism",
            fixtures::prism_z::<f64>(&self.profile, self.z0, self.z1, tol()).body,
            tol(),
        )
    }
    fn volume(&self) -> f64 {
        area(&self.profile) * (self.z1 - self.z0)
    }
}

fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

// ---- cube clipping oracle (r1's) ------------------------------------

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
        let (e1, e2) = basis(unit(n));
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

fn cube_poly(v: V3, u: V3, w: V3, m: V3) -> Poly {
    let p = |a: f64, b: f64, c: f64| add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
    let c = |i: usize| {
        let (a, b, z) = ((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64);
        p(-2.0 + 4.0 * a, -2.0 + 4.0 * b, 4.0 * z)
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

fn tanh_sinh(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    if b <= a {
        return 0.0;
    }
    let h = 1.0 / 64.0;
    let (c, r) = (0.5 * (a + b), 0.5 * (b - a));
    let mut s = 0.0;
    let mut k = -260i32;
    while k <= 260 {
        let t = f64::from(k) * h;
        let u = std::f64::consts::FRAC_PI_2 * t.sinh();
        let x = u.tanh();
        let w = std::f64::consts::FRAC_PI_2 * t.cosh() / u.cosh().powi(2);
        let xx = c + r * x;
        if xx > a && xx < b && w > 0.0 {
            s += w * f(xx);
        }
        k += 1;
    }
    s * h * r
}

/// The cylinder `x² + z² ≤ R²`, `0 ≤ y ≤ H`, against the prism: slices
/// in z, each piece clipped to the strip `|x| ≤ √(R² − z²)`, `y ∈ [0, H]`.
fn common_cyl(pr: &Prism, rr: f64, hh: f64) -> f64 {
    let (za, zb) = (pr.z0.max(-rr), pr.z1.min(rr));
    let mut total = 0.0;
    for piece in &pr.pieces {
        let slice = |z: f64| {
            let hw = (rr * rr - z * z).max(0.0).sqrt();
            let strip = [(-hw, 0.0), (hw, 0.0), (hw, hh), (-hw, hh)];
            let c = clip_convex(piece, &strip);
            if c.len() < 3 { 0.0 } else { area(&c) }
        };
        let mut cuts = vec![za, zb];
        for &(x, _) in piece {
            if x.abs() < rr {
                let z = (rr * rr - x * x).sqrt();
                for z in [-z, z] {
                    if z > za && z < zb {
                        cuts.push(z);
                    }
                }
            }
        }
        cuts.sort_by(f64::total_cmp);
        total += (0..cuts.len() - 1).map(|k| tanh_sinh(slice, cuts[k], cuts[k + 1])).sum::<f64>();
    }
    total
}

// ---- prisms ----------------------------------------------------------

fn base_prisms() -> Vec<Prism> {
    let l = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let lp = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
    ];
    let mk = |name: &str, profile: &Vec<(f64, f64)>, pieces: &Vec<Vec<(f64, f64)>>, v: V3, edges: Vec<V3>| Prism {
        name: name.into(),
        profile: profile.clone(),
        pieces: pieces.clone(),
        z0: 0.0,
        z1: 1.0,
        v,
        edges,
    };
    let ltop = mk("Ltop", &l, &lp, [1.0, 1.0, 1.0], vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]]);
    let lbot = mk("Lbot", &l, &lp, [1.0, 1.0, 0.0], vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    let lmid = mk("Lmid", &l, &lp, [1.0, 1.0, 0.5], vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    let n327 = vec![(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.3, 2.0), (2.0, 1.0), (1.7, 2.0), (0.0, 2.0)];
    let n327p = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.7, 2.0), (0.0, 2.0)],
        vec![(2.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.3, 2.0), (2.0, 1.0)],
    ];
    let notch = mk("notch327", &n327, &n327p, [2.0, 1.0, 1.0], vec![[0.3, 1.0, 0.0], [-0.3, 1.0, 0.0], [0.0, 0.0, -1.0]]);
    // A U: inner corners (1,1) and (2,1).
    let u = vec![(0.0, 0.0), (3.0, 0.0), (3.0, 2.0), (2.0, 2.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let up = vec![
        vec![(0.0, 0.0), (3.0, 0.0), (3.0, 1.0), (0.0, 1.0)],
        vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
        vec![(2.0, 1.0), (3.0, 1.0), (3.0, 2.0), (2.0, 2.0)],
    ];
    let uin = mk("Uin", &u, &up, [1.0, 1.0, 1.0], vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0]]);
    let mut out = vec![lmid, notch, uin];
    for deg in [-45.0, 30.0, 100.0, 200.0] {
        out.push(ltop.turned(deg, [0.0; 3], format!("Ltop_r{deg}")));
        out.push(lbot.turned(deg, [0.0; 3], format!("Lbot_r{deg}")));
    }
    out
}

type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, &BooleanDeclarations, Tol) -> Result<BooleanResult<f64>, BooleanError>;

/// Faces running through two distinct vertices at one point.
fn twov(body: &Body<f64>) -> usize {
    let mut bad = 0;
    for (fk, f) in body.faces() {
        let _ = fk;
        let mut pts: Vec<(topo::VertexKey, V3)> = Vec::new();
        for &l in std::iter::once(&f.outer).chain(&f.rings) {
            if let LoopBoundary::Cycle { first } = body.get_loop(l).unwrap().boundary {
                for he in body.loop_cycle(first).unwrap() {
                    let v = body.get_half_edge(he).unwrap().start;
                    let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
                    pts.push((v, [p.x, p.y, p.z]));
                }
            }
        }
        let hit = pts.iter().enumerate().any(|(i, a)| {
            pts[i + 1..].iter().any(|b| b.0 != a.0 && dot(sub(a.1, b.1), sub(a.1, b.1)) < 1e-20)
        });
        bad += usize::from(hit);
    }
    bad
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
                let tv = match &r {
                    Ok(res) => res.body().map_or(0, |bb| twov(&bb.body)),
                    Err(_) => 0,
                };
                let mut line = outcome(r, want, tol());
                if tv > 0 {
                    line = line.replace("SOUND", "BAD");
                }
                if std::env::var("R1_DIAG").is_ok() && line.contains("operand=false") {
                    if let Ok(res) = f(x, y, &decls, tol()) {
                        let bb = res.body().unwrap();
                        let far = finished(
                            "far",
                            sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()),
                            tol(),
                        );
                        let e = topo::union(&bb.body, &far, tol()).err();
                        line = format!("{line} UNION-ERR {e:?}");
                    }
                }
                format!("{line} twov={tv}")
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
        for a in 0..16 {
            let al = std::f64::consts::TAU * (a as f64 + 0.25) / 16.0;
            let base = add(scale(p1, al.cos()), scale(p2, al.sin()));
            for &d in ds {
                out.push((format!("nt e{j} a{a} d{d:e}"), unit(add(base, scale(e, d)))));
            }
        }
    }
    out
}

fn main() {
    let set = std::env::args().nth(1).unwrap_or_else(|| "cube".into());
    let only = std::env::args().nth(2);
    let ds: Vec<f64> = std::env::var("R1_NT_D")
        .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
        .unwrap_or_else(|_| vec![1e-3, -1e-3, 1e-6, -1e-6]);
    match set.as_str() {
        "cube" => {
            for pr in base_prisms() {
                if only.as_deref().is_some_and(|o| o != pr.name) {
                    continue;
                }
                let body = pr.body();
                let va = pr.volume();
                let mut dirs = directions(240);
                dirs.extend(near_tangent(&pr.edges, &ds));
                for (dn, m) in dirs {
                    let m = unit(m);
                    let lobes = pr.lobes(m);
                    let (u, w) = basis(m);
                    let v = pr.v;
                    let cube = finished(
                        "the cube",
                        fixtures::mapped_cube::<f64>(
                            move |x, y, z| {
                                let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
                                let p = add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
                                Point3::new(p[0], p[1], p[2])
                            },
                            tol(),
                        ),
                        tol(),
                    );
                    let common = common_cube(&pr, &cube_poly(v, u, w, m));
                    run_all(&format!("{} {dn} lobes={lobes}", pr.name), &body, &cube, va, 64.0, common);
                }
            }
        }
        "cyl" => {
            let (rr, hh) = (5.0, 10.0);
            let cyl = finished(
                "the cylinder",
                sweep::test_support::revolved_about_y(
                    vec![
                        (Point2::new(0.0, 0.0), 0.0),
                        (Point2::new(rr, 0.0), 0.0),
                        (Point2::new(rr, hh), 0.0),
                        (Point2::new(0.0, hh), 0.0),
                    ],
                    sweep::Revolution::Full,
                    tol(),
                ),
                tol(),
            );
            let vb = std::f64::consts::PI * rr * rr * hh;
            let vm = topo::mass_properties(&cyl, tol()).unwrap().volume;
            println!("# cylinder volume {vm:.12} exact {vb:.12}");
            let bases = {
                let all = base_prisms();
                let l = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
                let lp = vec![
                    vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
                    vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
                ];
                let mut v = vec![
                    Prism { name: "Ltop".into(), profile: l.clone(), pieces: lp.clone(), z0: 0.0, z1: 1.0, v: [1.0, 1.0, 1.0], edges: vec![] },
                    Prism { name: "Lbot".into(), profile: l, pieces: lp, z0: 0.0, z1: 1.0, v: [1.0, 1.0, 0.0], edges: vec![] },
                ];
                v.extend(all.into_iter().filter(|p| p.name == "notch327"));
                v
            };
            // v on the wall at angle φ in xz (seam meridian at φ = 0
            // avoided), the prism turned about v by θ.
            for base in &bases {
                for phi_deg in [40.0f64, 75.0, 110.0, 160.0, 230.0, 300.0] {
                    let phi = phi_deg.to_radians();
                    let target = [rr * phi.cos(), 5.0, rr * phi.sin()];
                    let m = [-phi.cos(), 0.0, -phi.sin()];
                    for k in 0..24 {
                        let deg = 15.0 * k as f64;
                        let name = format!("{} phi{phi_deg} th{deg}", base.name);
                        if only.as_deref().is_some_and(|o| !name.starts_with(o)) {
                            continue;
                        }
                        let pr = base.turned(deg, sub(target, base.v), name.clone());
                        let lobes = pr.lobes(m);
                        if lobes != 2 && std::env::var("R1_ALL").is_err() {
                            continue;
                        }
                        let common = common_cyl(&pr, rr, hh);
                        run_all(&format!("{name} lobes={lobes}"), &pr.body(), &cyl, pr.volume(), vb, common);
                    }
                }
            }
        }
        "u2" => {
            // The U's two inner vertical edges: corners at both ends; the
            // cube's face plane through two of them, so one op may pinch
            // twice.
            let base = base_prisms().into_iter().find(|p| p.name == "Uin").unwrap();
            let body = base.body();
            let va = base.volume();
            let pairs: [(&str, V3, V3); 3] = [
                ("t1b2", [1.0, 1.0, 1.0], [2.0, 1.0, 0.0]),
                ("b1t2", [1.0, 1.0, 0.0], [2.0, 1.0, 1.0]),
                ("t1t2", [1.0, 1.0, 1.0], [2.0, 1.0, 1.0]),
            ];
            for (pn, a, bb) in pairs {
                let d = unit(sub(bb, a));
                let (e1, e2) = basis(d);
                let mid = scale(add(a, bb), 0.5);
                let k = 720;
                let hits: Vec<(usize, V3, usize, usize)> = (0..k)
                    .map(|i| {
                        let t = std::f64::consts::TAU * (i as f64 + 0.37) / k as f64;
                        let m = add(scale(e1, t.cos()), scale(e2, t.sin()));
                        let mut pa = base.clone();
                        pa.v = a;
                        let mut pb = base.clone();
                        pb.v = bb;
                        (i, m, pa.lobes(m), pb.lobes(m))
                    })
                    .filter(|h| std::env::var("R1_ALL").is_ok() || (h.2 == 2 && h.3 == 2))
                    .collect();
                let step = (hits.len() / 60).max(1);
                for &(i, m, la, lb) in hits.iter().step_by(step) {
                    let (u, w) = basis(m);
                    let v = mid;
                    let cube = finished(
                        "the cube",
                        fixtures::mapped_cube::<f64>(
                            move |x, y, z| {
                                let (aa, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
                                let p = add(v, add(scale(u, aa), add(scale(w, b), scale(m, c))));
                                Point3::new(p[0], p[1], p[2])
                            },
                            tol(),
                        ),
                        tol(),
                    );
                    let common = common_cube(&base, &cube_poly(v, u, w, m));
                    run_all(&format!("U2 {pn} b{i} lobes={la},{lb}"), &body, &cube, va, 64.0, common);
                }
            }
        }
        _ => panic!("set"),
    }
}

//! Review r1 of PR 4026: pierce poses beyond `pierce_runs_battery`.
//!
//! A prism corner `v` placed on a pierced face — a cube's near face
//! (planar) or a ball's surface (curved) — whose tangent plane at `v`
//! has normal `m`, the pierced solid on `m`'s side. Every op in both
//! orders prints one `outcome` line against a kernel-free oracle:
//! - polytopes: each convex piece of the prism clipped by the cube's
//!   half-spaces, the volume by the divergence theorem (a clipping
//!   oracle, a different algorithm from the PR's vertex enumeration);
//! - the ball: box ∩ ball as an exact chord integral in x and y and a
//!   tanh-sinh quadrature in z between the area's breakpoints.
//!
//! Each line also carries `lobes=k`, the number of arcs of the plane
//! through `v` that lie inside the prism's tangent cone (sampled), so
//! the two-run poses can be picked out without reading the kernel.
//!
//! `cargo run -p sweep --release --example r1_pierce_probes [set]`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "../tests/common/differential.rs"]
#[allow(dead_code)]
mod differential;

use differential::outcome;
use geom_core::{Point3, Tol, Vec3};
use topo::test_support as fixtures;
use topo::{Body, BooleanDeclarations, BooleanError, BooleanResult, mass_properties};

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

/// A prism: CCW profile, height, convex pieces (CCW polygons), corner.
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
}

fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}

// ---- clipping oracle -------------------------------------------------

type Poly = Vec<Vec<V3>>; // faces, each CCW seen from outside

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
        // outward is +n: CCW about n
        out.push(pts.into_iter().map(|p| p.1).collect());
    }
    out
}

fn volume(poly: &Poly) -> f64 {
    poly.iter()
        .map(|f| {
            (1..f.len() - 1)
                .map(|i| dot(f[0], cross(f[i], f[i + 1])))
                .sum::<f64>()
        })
        .sum::<f64>()
        / 6.0
}

/// The cube `v + a u + b w + c m`, a, b ∈ [−2, 2], c ∈ [0, 4], as faces.
fn cube_poly(v: V3, u: V3, w: V3, m: V3) -> Poly {
    let p = |a: f64, b: f64, c: f64| add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
    let c = |i: usize| {
        let (a, b, z) = ((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64);
        p(-2.0 + 4.0 * a, -2.0 + 4.0 * b, 4.0 * z)
    };
    // (u, w, m) right-handed; faces CCW from outside
    let quads = [
        [0, 2, 3, 1], // c=0 (outward −m)
        [4, 5, 7, 6], // c=1
        [0, 1, 5, 4], // b=0
        [2, 6, 7, 3], // b=1
        [0, 4, 6, 2], // a=0
        [1, 3, 7, 5], // a=1
    ];
    quads.iter().map(|q| q.iter().map(|&i| c(i)).collect()).collect()
}

fn common_cube(pr: &Prism, cube: &Poly) -> f64 {
    pr.piece_planes()
        .iter()
        .map(|pl| volume(&pl.iter().fold(cube.clone(), |acc, &p| clip(&acc, p))))
        .sum()
}

// ---- ball oracle -----------------------------------------------------

/// ∫ sqrt(ρ² − t²) dt antiderivative.
fn g(t: f64, rho: f64) -> f64 {
    let t = t.clamp(-rho, rho);
    0.5 * (t * (rho * rho - t * t).max(0.0).sqrt() + rho * rho * (t / rho).asin())
}

/// Area of the rectangle ∩ the disk centred (cx, cy) radius ρ, exact.
fn rect_disk(x0: f64, x1: f64, y0: f64, y1: f64, cx: f64, cy: f64, rho: f64) -> f64 {
    if rho <= 0.0 {
        return 0.0;
    }
    let (a, b) = ((x0 - cx).max(-rho), (x1 - cx).min(rho));
    if a >= b {
        return 0.0;
    }
    let (lo, hi) = (y0 - cy, y1 - cy);
    let mut cuts = vec![a, b];
    for d in [lo, hi] {
        if d.abs() < rho {
            let s = (rho * rho - d * d).sqrt();
            for t in [-s, s] {
                if t > a && t < b {
                    cuts.push(t);
                }
            }
        }
    }
    cuts.sort_by(f64::total_cmp);
    let mut area = 0.0;
    for k in 0..cuts.len() - 1 {
        let (s0, s1) = (cuts[k], cuts[k + 1]);
        if s1 <= s0 {
            continue;
        }
        let mid = 0.5 * (s0 + s1);
        let sm = (rho * rho - mid * mid).max(0.0).sqrt();
        let top_is_rect = hi <= sm;
        let bot_is_rect = lo >= -sm;
        let gi = g(s1, rho) - g(s0, rho);
        let len = s1 - s0;
        let tm = if top_is_rect { hi } else { sm };
        let bm = if bot_is_rect { lo } else { -sm };
        if tm <= bm {
            continue;
        }
        let top = if top_is_rect { hi * len } else { gi };
        let bot = if bot_is_rect { lo * len } else { -gi };
        area += top - bot;
    }
    area
}

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

/// Volume of the axis box ∩ ball(c, r).
fn box_ball(lo: V3, hi: V3, c: V3, r: f64) -> f64 {
    let area = |z: f64| {
        let rho2 = r * r - (z - c[2]) * (z - c[2]);
        if rho2 <= 0.0 {
            0.0
        } else {
            rect_disk(lo[0], hi[0], lo[1], hi[1], c[0], c[1], rho2.sqrt())
        }
    };
    let (za, zb) = (lo[2].max(c[2] - r), hi[2].min(c[2] + r));
    if za >= zb {
        return 0.0;
    }
    let mut cuts = vec![za, zb];
    let dx = [lo[0] - c[0], hi[0] - c[0]];
    let dy = [lo[1] - c[1], hi[1] - c[1]];
    let mut ds: Vec<f64> = dx.iter().chain(&dy).map(|d| d.abs()).collect();
    for x in dx {
        for y in dy {
            ds.push((x * x + y * y).sqrt());
        }
    }
    for d in ds {
        if d < r {
            let s = (r * r - d * d).sqrt();
            for z in [c[2] - s, c[2] + s] {
                if z > za && z < zb {
                    cuts.push(z);
                }
            }
        }
    }
    cuts.sort_by(f64::total_cmp);
    (0..cuts.len() - 1)
        .map(|k| tanh_sinh(area, cuts[k], cuts[k + 1]))
        .sum()
}

// ---- prisms ----------------------------------------------------------

fn prisms() -> Vec<Prism> {
    let l = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    let l_pieces = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
        vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
    ];
    let mirror = |p: &[(f64, f64)]| -> Vec<(f64, f64)> {
        let mut q: Vec<(f64, f64)> = p.iter().map(|&(x, y)| (-x, y)).collect();
        q.reverse();
        q
    };
    vec![
        Prism { name: "Ltop", profile: l.clone(), pieces: l_pieces.clone(), h: 1.0, v: [1.0, 1.0, 1.0] },
        Prism { name: "Lbot", profile: l.clone(), pieces: l_pieces.clone(), h: 1.0, v: [1.0, 1.0, 0.0] },
        Prism {
            name: "Lmirror",
            profile: mirror(&l),
            pieces: l_pieces.iter().map(|p| mirror(p)).collect(),
            h: 1.0,
            v: [-1.0, 1.0, 1.0],
        },
        Prism {
            name: "notch307",
            profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)],
            pieces: vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 2.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0)],
            ],
            h: 1.0,
            v: [2.0, 1.0, 1.0],
        },
        Prism {
            name: "shallow200",
            profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6), (0.0, 1.0)],
            pieces: vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.6), (0.0, 1.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 1.0), (2.0, 0.6)],
            ],
            h: 1.0,
            v: [2.0, 0.6, 1.0],
        },
        Prism { name: "convex", profile: l.clone(), pieces: l_pieces, h: 1.0, v: [2.0, 0.0, 1.0] },
    ]
}

type Op = fn(&Body<f64>, &Body<f64>, &BooleanDeclarations, Tol) -> Result<BooleanResult<f64>, BooleanError>;

fn run_all(tag: &str, prism: &Body<f64>, other: &Body<f64>, va: f64, vb: f64, common: f64) {
    let decls = BooleanDeclarations::default();
    for (order, x, y, vx) in [("pc", prism, other, va), ("cp", other, prism, vb)] {
        let ops: [(&str, Op, f64); 3] = [
            ("U", topo::union_with, va + vb - common),
            ("I", topo::intersect_with, common),
            ("S", topo::subtract_with, vx - common),
        ];
        for (op, f, want) in ops {
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                outcome(f(x, y, &decls, tol()), want, tol())
            }))
            .unwrap_or_else(|_| "PANIC".into());
            println!("{tag} {order} {op}: {r}");
            if std::env::var("R1_DIAG").is_ok() {
                if let Ok(res) = f(x, y, &decls, tol()) {
                    if let Some(bb) = res.body() {
                        println!("  t3p: {:?}", topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()));
                        println!("  shells/faces/verts: {:?}", topo::validate_closed(&bb.body));
                    }
                }
            }
        }
    }
}

fn directions() -> Vec<(String, V3)> {
    let mut out = Vec::new();
    let n = 240;
    let ga = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    for i in 0..n {
        let z = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
        let r = (1.0 - z * z).sqrt();
        let t = ga * i as f64;
        out.push((format!("fib{i}"), [r * t.cos(), r * t.sin(), z]));
    }
    out
}

/// Near-tangent tilts: the plane nearly holds one of the corner's edges.
fn near_tangent(edges: &[V3]) -> Vec<(String, V3)> {
    let mut out = Vec::new();
    for (j, &e) in edges.iter().enumerate() {
        let e = unit(e);
        let (p1, p2) = basis(e);
        for a in 0..16 {
            let al = std::f64::consts::TAU * (a as f64 + 0.25) / 16.0;
            let base = add(scale(p1, al.cos()), scale(p2, al.sin()));
            let ds: Vec<f64> = std::env::var("R1_NT_D")
                .map(|s| s.split(',').map(|x| x.parse().unwrap()).collect())
                .unwrap_or_else(|_| vec![1e-3, -1e-3, 1e-6, -1e-6]);
            for d in ds {
                out.push((format!("nt e{j} a{a} d{d:e}"), unit(add(base, scale(e, d)))));
            }
        }
    }
    out
}

fn corner_edges(pr: &Prism) -> Vec<V3> {
    let n = pr.profile.len();
    let i = pr
        .profile
        .iter()
        .position(|&(x, y)| (x - pr.v[0]).abs() < 1e-12 && (y - pr.v[1]).abs() < 1e-12)
        .unwrap();
    let (p, a, b) = (pr.profile[i], pr.profile[(i + n - 1) % n], pr.profile[(i + 1) % n]);
    let vz = if pr.v[2] > 0.5 { -1.0 } else { 1.0 };
    vec![[a.0 - p.0, a.1 - p.1, 0.0], [b.0 - p.0, b.1 - p.1, 0.0], [0.0, 0.0, vz]]
}

fn main() {
    let set = std::env::args().nth(1).unwrap_or_else(|| "cube".into());
    let only = std::env::args().nth(2);
    for pr in prisms() {
        if only.as_deref().is_some_and(|o| o != pr.name) {
            continue;
        }
        let body = fixtures::prism::<f64>(&pr.profile, pr.h, tol()).body;
        let va = mass_properties(&body, tol()).unwrap().volume;
        let mut dirs = if std::env::var("R1_NT_D").is_ok() { Vec::new() } else { directions() };
        dirs.extend(near_tangent(&corner_edges(&pr)));
        for (dn, m) in dirs {
            let m = unit(m);
            let lobes = pr.lobes(m);
            if let Some(want) = std::env::var("R1_DIAG").ok() {
                if want != dn {
                    continue;
                }
            }
            if set == "cube2" && lobes != 2 {
                continue;
            }
            let (u, w) = basis(m);
            match set.as_str() {
                "cube" | "cube2" => {
                    let v = pr.v;
                    let cube = fixtures::mapped_cube::<f64>(
                        move |x, y, z| {
                            let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
                            let p = add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
                            Point3::new(p[0], p[1], p[2])
                        },
                        tol(),
                    );
                    let common = common_cube(&pr, &cube_poly(v, u, w, m));
                    run_all(&format!("{} {dn} lobes={lobes}", pr.name), &body, &cube, va, 64.0, common);
                }
                "ball" => {
                    if lobes != 2 {
                        continue;
                    }
                    let r = 3.0;
                    let c = add(pr.v, scale(m, r));
                    // pole ⊥ m: v on the chart's equator
                    let ball = sweep::test_support::ball_poled(
                        r,
                        Vec3::new(c[0], c[1], c[2]),
                        Vec3::new(u[0], u[1], u[2]),
                        tol(),
                    );
                    let vb = mass_properties(&ball, tol()).unwrap().volume;
                    let exact = 4.0 / 3.0 * std::f64::consts::PI * r * r * r;
                    let common: f64 = if pr.name.starts_with('L') || pr.name == "convex" {
                        let sgn = if pr.name == "Lmirror" { -1.0 } else { 1.0 };
                        let boxes = [([0.0, 0.0, 0.0], [2.0, 1.0, 1.0]), ([0.0, 1.0, 0.0], [1.0, 2.0, 1.0])];
                        boxes
                            .iter()
                            .map(|&(lo, hi): &(V3, V3)| {
                                let (lo, hi) = if sgn < 0.0 {
                                    ([-hi[0], lo[1], lo[2]], [-lo[0], hi[1], hi[2]])
                                } else {
                                    (lo, hi)
                                };
                                box_ball(lo, hi, c, r)
                            })
                            .sum()
                    } else {
                        continue;
                    };
                    println!("# ball vb={vb:.12} exact={exact:.12}");
                    run_all(&format!("{} ball {dn} lobes={lobes}", pr.name), &body, &ball, va, exact, common);
                }
                _ => panic!("set"),
            }
        }
    }
}

//! Review of PR 4051: island faces pinched to their hole's ring,
//! beyond the PR's 18 (deeper holes, two islands, an island pinched
//! twice, cylinder walls). Helpers from review r2 of PR 4038.

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

type Op = fn(
    &AtRestBody<f64>,
    &AtRestBody<f64>,
    &BooleanDeclarations,
    Tol,
) -> Result<BooleanResult<f64>, BooleanError>;

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


// ---- holed blocks ----------------------------------------------------

/// A right prism over the convex `outer` on `[0, h]`, less through-holes
/// drilled top to bottom. Each hole is its rim (CCW) and the convex
/// pieces that tile it.
struct Holed {
    name: &'static str,
    outer: Vec<(f64, f64)>,
    holes: Vec<(Vec<(f64, f64)>, Vec<Vec<(f64, f64)>>)>,
    h: f64,
}

fn piece_planes(poly: &[(f64, f64)], h: f64) -> Vec<Plane> {
    let n = poly.len();
    let mut out: Vec<Plane> = (0..n)
        .map(|i| {
            let (p, q) = (poly[i], poly[(i + 1) % n]);
            let nn = [q.1 - p.1, -(q.0 - p.0), 0.0];
            (nn, nn[0] * p.0 + nn[1] * p.1)
        })
        .collect();
    out.push(([0.0, 0.0, 1.0], h));
    out.push(([0.0, 0.0, -1.0], 0.0));
    out
}

impl Holed {
    /// `(sign, planes)`: + the outer prism, − each hole piece.
    fn signed(&self) -> Vec<(f64, Vec<Plane>)> {
        let mut out = vec![(1.0, piece_planes(&self.outer, self.h))];
        for (_, pieces) in &self.holes {
            for p in pieces {
                out.push((-1.0, piece_planes(p, self.h)));
            }
        }
        out
    }
    fn build(&self) -> Body<f64> {
        let mut body = Body::<f64>::new();
        let ops = fixtures::prism_ops(
            &mut body,
            &self.outer,
            (0.0, self.h),
            fixtures::identity_map::<f64>,
            fixtures::FaceGeometry::Declined,
            tol(),
        );
        let entry = ops.sides[0].he_plus;
        for (rim, _) in &self.holes {
            let top: Vec<_> = rim.iter().map(|&(x, y)| Point3::new(x, y, self.h)).collect();
            let bot: Vec<_> = rim.iter().map(|&(x, y)| Point3::new(x, y, 0.0)).collect();
            fixtures::drill_hole(&mut body, entry, ops.bottom.face, &top, &bot, tol());
        }
        fixtures::plane_every_face(&mut body, tol());
        fixtures::describe_as_intersections(&mut body, tol());
        body
    }
    fn inside(&self, x: V3) -> bool {
        let inp = |pl: &Vec<Plane>| pl.iter().all(|&(n, d)| dot(n, x) <= d + 1e-15);
        let s = self.signed();
        inp(&s[0].1) && !s[1..].iter().any(|(_, pl)| {
            pl.iter().all(|&(n, d)| dot(n, x) < d - 1e-15)
        })
    }
    /// Arcs of the plane through `v` normal to `m` inside the solid.
    fn lobes(&self, v: V3, m: V3) -> usize {
        let (u, w) = basis(m);
        let k = 3600;
        let ins: Vec<bool> = (0..k)
            .map(|i| {
                let t = std::f64::consts::TAU * (i as f64 + 0.5) / k as f64;
                let d = add(scale(u, t.cos()), scale(w, t.sin()));
                self.inside(add(v, scale(d, 1e-7)))
            })
            .collect();
        if ins.iter().all(|&b| b) {
            return 99;
        }
        (0..k).filter(|&i| ins[i] && !ins[(i + 1) % k]).count()
    }
}

fn sq(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<(f64, f64)> {
    vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]
}

fn std_hole() -> (Vec<(f64, f64)>, Vec<Vec<(f64, f64)>>) {
    (sq(0.5, 0.5, 1.5, 1.5), vec![sq(0.5, 0.5, 1.5, 1.5)])
}

fn u_hole() -> (Vec<(f64, f64)>, Vec<Vec<(f64, f64)>>) {
    let rim = vec![
        (1.0, 1.0), (3.0, 1.0), (3.0, 2.8), (2.75, 3.0), (2.5, 2.8),
        (2.5, 1.8), (1.5, 1.8), (1.5, 2.8), (1.25, 3.0), (1.0, 2.8),
    ];
    let pieces = vec![
        sq(1.0, 1.0, 3.0, 1.8),
        vec![(1.0, 1.8), (1.5, 1.8), (1.5, 2.8), (1.25, 3.0), (1.0, 2.8)],
        vec![(2.5, 1.8), (3.0, 1.8), (3.0, 2.8), (2.75, 3.0), (2.5, 2.8)],
    ];
    (rim, pieces)
}

/// The cube `v + a u + b w + c m`, a, b ∈ [−s/2, s/2], c ∈ [0, s].
fn cube_poly(v: V3, u: V3, w: V3, m: V3, s: f64) -> Poly {
    let p = |a: f64, b: f64, c: f64| add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
    let c = |i: usize| {
        let (a, b, z) = ((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64);
        p(-s / 2.0 + s * a, -s / 2.0 + s * b, s * z)
    };
    let quads = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]];
    quads.iter().map(|q| q.iter().map(|&i| c(i)).collect()).collect()
}

fn cube_case(hd: &Holed, body: &AtRestBody<f64>, va: f64, v: V3, m: V3, psi: f64, s: f64, tag: &str) {
    let (u0, w0) = basis(m);
    let (u, w) = turn(u0, w0, psi);
    let cube = fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-s / 2.0 + s * x, -s / 2.0 + s * y, s * z);
            to_p(add(v, add(scale(u, a), add(scale(w, b), scale(m, c)))))
        },
        tol(),
    );
    let Ok(cube) = AtRestBody::validate(cube, tol()) else {
        println!("{tag}: CUBE-INVALID");
        return;
    };
    let cp = cube_poly(v, u, w, m, s);
    let common: f64 = hd
        .signed()
        .iter()
        .map(|(sg, pl)| sg * volume(&pl.iter().fold(cp.clone(), |acc, &p| clip(&acc, p))))
        .sum();
    run_all(tag, body, &cube, va, s * s * s, common);
}

#[allow(clippy::too_many_arguments)]
fn cyl_case(hd: &Holed, body: &AtRestBody<f64>, va: f64, v: V3, m: V3, r: f64, psi: f64, zoff: f64, tag: &str) {
    let half = 4.0;
    let (u0, _) = basis(m);
    let (a, _) = turn(u0, cross(m, u0), psi);
    let c = add(v, scale(m, r));
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
    let common: f64 = hd
        .signed()
        .iter()
        .map(|(sg, pl)| {
            let fp: Vec<Plane> = pl
                .iter()
                .map(|&(n, d)| {
                    let nn = map.inverse().transform_vec(to_v(n));
                    ([nn.x, nn.y, nn.z], d - dot(n, c))
                })
                .collect();
            let f = |t: f64| slice_area(&fp, t, r);
            let k = 64;
            sg * (0..k)
                .map(|i| {
                    let (a0, b0) = (-half + 2.0 * half * i as f64 / k as f64, -half + 2.0 * half * (i + 1) as f64 / k as f64);
                    integrate(&f, a0, b0, 1e-13, 18)
                })
                .sum::<f64>()
        })
        .sum();
    run_all(tag, body, &cyl, va, vb, common);
}

fn holed_dirs() -> Vec<(String, V3)> {
    let mut dirs: Vec<(String, V3)> = Vec::new();
    for i in 0..8 {
        for (j, &phi) in [0.45f64, 0.6, 0.75, 0.9, 1.05, 1.2, 1.35].iter().enumerate() {
            let theta = std::f64::consts::FRAC_PI_2 * (f64::from(i) + 0.5) / 8.0;
            dirs.push((format!("q{i}.{j}"), [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]));
        }
    }
    for i in 0..12 {
        for j in 0..7 {
            let theta = std::f64::consts::TAU * (f64::from(i) + 0.37) / 12.0;
            let phi = (f64::from(j) - 3.0) * 0.4 + 0.05;
            dirs.push((format!("g{i}.{j}"), [theta.cos() * phi.cos(), theta.sin() * phi.cos(), phi.sin()]));
        }
    }
    dirs
}

/// Unit normals of the planes holding the line through `p` and `q`.
fn pencil(p: V3, q: V3, k: usize) -> Vec<(String, V3)> {
    let d = unit(sub(q, p));
    let (e1, e2) = basis(d);
    (0..k)
        .map(|i| {
            let t = std::f64::consts::TAU * (i as f64 + 0.37) / k as f64;
            (format!("th{i}"), add(scale(e1, t.cos()), scale(e2, t.sin())))
        })
        .collect()
}

fn to_p(p: V3) -> Point3<f64> {
    Point3::new(p[0], p[1], p[2])
}
fn to_v(p: V3) -> Vec3<f64> {
    Vec3::new(p[0], p[1], p[2])
}

fn main() {
    let set = std::env::args().nth(1).unwrap_or_else(|| "deep".into());
    let shard = std::env::var("IP_SHARD").unwrap_or_else(|_| "0/1".into());
    let (sk, sn): (usize, usize) = {
        let w: Vec<usize> = shard.split('/').map(|x| x.parse().unwrap()).collect();
        (w[0], w[1])
    };
    let mut idx = 0usize;
    let mut take = || {
        idx += 1;
        idx % sn == sk
    };
    let mk = |hd: &Holed| {
        let b = AtRestBody::validate(hd.build(), tol()).unwrap();
        let va = mass_properties(&b, tol()).unwrap().volume;
        let want: f64 = hd.signed().iter().map(|(s, pl)| s * volume(&pl.iter().fold(cube_poly([-50.0, -50.0, -50.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 0.0), |a, &p| clip(&a, p)))).sum();
        let _ = want;
        (b, va)
    };
    match set.as_str() {
        // The holed block, its hole deeper: h = 2 (the PR's), 6 and 0.6.
        "deep" => {
            for h in [2.0, 6.0, 0.6] {
                let hd = Holed { name: "deep", outer: sq(0.0, 0.0, 2.0, 2.0), holes: vec![std_hole()], h };
                let (b, va) = mk(&hd);
                assert!((va - 3.0 * h).abs() < 1e-9, "{va}");
                for (corner, v) in [("c00", [0.5, 0.5, h]), ("c11", [1.5, 1.5, h])] {
                    for (tag, m) in holed_dirs() {
                        let m = unit(if corner == "c11" { [-m[0], -m[1], m[2]] } else { m });
                        for s in [4.0, 12.0] {
                            if !take() {
                                continue;
                            }
                            let lobes = hd.lobes(v, m);
                            cube_case(&hd, &b, va, v, m, 0.0, s, &format!("{} h={h} {corner} side={s} {tag} lobes={lobes}", hd.name));
                        }
                    }
                }
            }
        }
        // Two holes, one plane through a corner of each: two islands.
        "two" => {
            let hd = Holed {
                name: "two",
                outer: sq(0.0, 0.0, 5.0, 2.4),
                holes: vec![
                    (sq(0.5, 0.5, 1.5, 1.5), vec![sq(0.5, 0.5, 1.5, 1.5)]),
                    (sq(2.5, 0.7, 3.5, 1.7), vec![sq(2.5, 0.7, 3.5, 1.7)]),
                ],
                h: 2.0,
            };
            let (b, va) = mk(&hd);
            let (p, q) = ([1.5, 0.5, 2.0], [3.5, 0.7, 2.0]);
            let mid = scale(add(p, q), 0.5);
            for (tag, m) in pencil(p, q, 72) {
                for (anchor, v) in [("mid", mid), ("p", p)] {
                    for s in [6.0, 12.0] {
                        for psi in [0.0, 0.7] {
                            if !take() {
                                continue;
                            }
                            let lobes = format!("{}/{}", hd.lobes(p, m), hd.lobes(q, m));
                            cube_case(&hd, &b, va, v, m, psi, s, &format!("two {anchor} side={s} psi={psi} {tag} lobes={lobes}"));
                        }
                    }
                }
            }
        }
        // A U hole, one plane through both arm tips: one island pinched
        // twice; and every Fibonacci direction at one tip.
        "u" => {
            let hd = Holed { name: "u", outer: sq(0.0, 0.0, 4.0, 4.0), holes: vec![u_hole()], h: 2.0 };
            let (b, va) = mk(&hd);
            let (p, q) = ([1.25, 3.0, 2.0], [2.75, 3.0, 2.0]);
            let mid = scale(add(p, q), 0.5);
            for (tag, m) in pencil(p, q, 72) {
                for (anchor, v) in [("mid", mid), ("p", p)] {
                    for s in [6.0, 12.0] {
                        for psi in [0.0, 0.7] {
                            if !take() {
                                continue;
                            }
                            let lobes = format!("{}/{}", hd.lobes(p, m), hd.lobes(q, m));
                            cube_case(&hd, &b, va, v, m, psi, s, &format!("u2tip {anchor} side={s} psi={psi} {tag} lobes={lobes}"));
                        }
                    }
                }
            }
            let ga = std::f64::consts::PI * (3.0 - 5f64.sqrt());
            for i in 0..120 {
                let z = 1.0 - 2.0 * (i as f64 + 0.5) / 120.0;
                let r = (1.0 - z * z).sqrt();
                let m = [r * (ga * i as f64).cos(), r * (ga * i as f64).sin(), z];
                for s in [4.0, 12.0] {
                    if !take() {
                        continue;
                    }
                    let lobes = hd.lobes(p, m);
                    cube_case(&hd, &b, va, p, m, 0.0, s, &format!("utip fib{i} side={s} lobes={lobes}"));
                }
            }
        }
        // The holed block against a cylinder whose wall holds the corner.
        "cyl" => {
            for h in [2.0, 6.0] {
                let hd = Holed { name: "cyl", outer: sq(0.0, 0.0, 2.0, 2.0), holes: vec![std_hole()], h };
                let (b, va) = mk(&hd);
                let v = [0.5, 0.5, h];
                for (tag, m) in holed_dirs().into_iter().filter(|(t, _)| t.starts_with('q')) {
                    let m = unit(m);
                    for r in [3.0, 1.0] {
                        for psi in [0.0, 0.9, 2.2] {
                            for (sname, zoff) in [("off", 0.9), ("seam", 0.0)] {
                                if !take() {
                                    continue;
                                }
                                let lobes = hd.lobes(v, m);
                                cyl_case(&hd, &b, va, v, m, r, psi, zoff, &format!("cyl h={h} r={r} {tag} psi={psi} {sname} lobes={lobes}"));
                            }
                        }
                    }
                }
            }
        }
        _ => panic!("set"),
    }
}

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
                    Ok(res) => res.body().map(|bb| {
                        let t3 = if std::env::var("R2P_T3").is_ok() {
                            format!(" T3={:?}", topo::validate_pseudomanifold(&bb.body, &bb.contacts, tol()).err())
                        } else { String::new() };
                        let chi = if std::env::var("R2P_CHI").is_ok() {
                            let b = &bb.body;
                            let (mut vv, mut ee) = (std::collections::BTreeSet::new(), std::collections::BTreeSet::new());
                            let (mut ff, mut rr) = (0, 0);
                            let mut dbl = 0;
                            for (_, fd) in b.faces() {
                                ff += 1;
                                rr += fd.rings.len();
                                for &l in std::iter::once(&fd.outer).chain(&fd.rings) {
                                    if let LoopBoundary::Cycle { first } = b.get_loop(l).unwrap().boundary {
                                        let c = b.loop_cycle(first).unwrap();
                                        let mut seen = std::collections::BTreeSet::new();
                                        for he in c {
                                            let h = b.get_half_edge(he).unwrap();
                                            if !seen.insert(h.start) { dbl += 1; }
                                            vv.insert(h.start);
                                            ee.insert(h.edge);
                                        }
                                    }
                                }
                            }
                            format!(" V={} E={} F={ff} R={rr} chi={} loopdbl={dbl}", vv.len(), ee.len(), vv.len() as i64 - ee.len() as i64 + ff as i64 - rr as i64)
                        } else { String::new() };
                        format!("{}{t3}{chi}", vertex_check(&bb.body))
                    }).unwrap_or_default(),
                    Err(_) => String::new(),
                };
                format!("{} {vc}", outcome(r, want, tol()))
            }))
            .unwrap_or_else(|_| "PANIC".into());
            println!("{tag} {order} {op}: {r}");
        }
    }
}


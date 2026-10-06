//! Review r1 of PR 4139 ("a pinch is one vertex per cone"): pinch poses
//! beyond the PR's batteries, each line read through
//! `differential::outcome`, tessellated with `check_mesh`, and checked
//! against a kernel-independent cone count at every pinch point.
//!
//! **The cone count.** Near a point `p` the result is a cone over its
//! link, a region of the unit sphere cut out by the great circles of the
//! oracle planes through `p` (a cylinder by its tangent plane). Each cell
//! of that arrangement is in or out as a whole; cells whose sign vectors
//! differ in one circle share an arc. The in and out cells' components
//! form a tree on the sphere whose edges are the link's boundary
//! circles, so the boundary's cones number `in + out − 1`. The ruling
//! wants exactly that many vertices at `p`. `nm` marks a point where in
//! and out cells meet checkerboard-wise (an edge through `p` that is not
//! manifold), where the count is not defined.
//!
//! `cargo run -p sweep --release --example r1_4139_probes <set>`, sets
//! `multi`, `pair`, `stair3`, `islnotch`, `cyl`, `nt`. `R1_SHARD=k/n`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[path = "../tests/common/differential.rs"]
#[allow(dead_code)]
mod differential;

use differential::outcome;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::test_support as fixtures;
use topo::{AtRestBody, Body, BooleanDeclarations, BooleanError, BooleanResult};

type V3 = [f64; 3];
type Plane = (V3, f64); // n·x ≤ d
type Pieces = Vec<Vec<Plane>>;

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
fn norm(a: V3) -> f64 {
    dot(a, a).sqrt()
}
fn unit(a: V3) -> V3 {
    scale(a, 1.0 / norm(a))
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
fn to_p(p: V3) -> Point3<f64> {
    Point3::new(p[0], p[1], p[2])
}
fn to_v(p: V3) -> Vec3<f64> {
    Vec3::new(p[0], p[1], p[2])
}

// ---- polytope volume (clip a big box) ---------------------------------

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

fn poly_volume(poly: &Poly) -> f64 {
    poly.iter()
        .map(|f| (1..f.len() - 1).map(|i| dot(f[0], cross(f[i], f[i + 1]))).sum::<f64>())
        .sum::<f64>()
        / 6.0
}

fn big_box() -> Poly {
    let s = 100.0;
    let c = |i: usize| {
        let b = |k: usize| if (i >> k) & 1 == 1 { s } else { -s };
        [b(0), b(1), b(2)]
    };
    let quads = [[0, 2, 3, 1], [4, 5, 7, 6], [0, 1, 5, 4], [2, 6, 7, 3], [0, 4, 6, 2], [1, 3, 7, 5]];
    quads.iter().map(|q| q.iter().map(|&i| c(i)).collect()).collect()
}

fn convex(planes: &[Plane]) -> f64 {
    poly_volume(&planes.iter().fold(big_box(), |acc, &p| clip(&acc, p)))
}

fn pieces_volume(p: &Pieces) -> f64 {
    p.iter().map(|q| convex(q)).sum()
}

fn common_volume(a: &Pieces, b: &Pieces) -> f64 {
    a.iter()
        .flat_map(|p| b.iter().map(move |q| convex(&[p.clone(), q.clone()].concat())))
        .sum()
}

// ---- the cone count ----------------------------------------------------

/// A solid's link at `p`: its pieces' planes through `p` and whether a
/// piece holds `p` at all; `None` pieces miss `p`.
fn link_pieces(p: V3, pieces: &Pieces) -> Vec<Vec<V3>> {
    let mut out = Vec::new();
    for q in pieces {
        let mut through = Vec::new();
        let mut holds = true;
        for &(n, d) in q {
            let s = (dot(n, p) - d) / norm(n);
            if s > 1e-9 {
                holds = false;
                break;
            }
            if s.abs() <= 1e-9 {
                through.push(unit(n));
            }
        }
        if holds {
            out.push(through);
        }
    }
    out
}

fn in_link(lp: &[Vec<V3>], s: V3) -> bool {
    lp.iter().any(|q| q.iter().all(|&n| dot(n, s) < 0.0))
}

#[derive(Clone, Copy)]
enum OpK {
    U,
    I,
    S, // first minus second
}

/// The result's cones at `p` (`in + out − 1`) and how many of them are
/// flat (bounded by one great circle, so they need no vertex), `Err`
/// where undefined.
fn cones(p: V3, x: &Pieces, y: &Pieces, op: OpK) -> Result<(usize, usize), &'static str> {
    let (lx, ly) = (link_pieces(p, x), link_pieces(p, y));
    let inside = |s: V3| {
        let (a, b) = (in_link(&lx, s), in_link(&ly, s));
        match op {
            OpK::U => a || b,
            OpK::I => a && b,
            OpK::S => a && !b,
        }
    };
    let mut circles: Vec<V3> = Vec::new();
    for n in lx.iter().chain(&ly).flatten() {
        let mut dup = false;
        for c in &circles {
            let x = norm(cross(*c, *n));
            if x < 1e-12 {
                dup = true;
            } else if x < 1e-7 {
                return Err("near");
            }
        }
        if !dup {
            circles.push(*n);
        }
    }
    let sign = |s: V3| -> Option<Vec<bool>> {
        circles
            .iter()
            .map(|&c| {
                let v = dot(c, s);
                if v.abs() < 1e-14 { None } else { Some(v > 0.0) }
            })
            .collect()
    };
    let mut cells: Vec<(Vec<bool>, bool)> = Vec::new();
    let mut push = |s: V3, cells: &mut Vec<(Vec<bool>, bool)>| -> Result<(), &'static str> {
        let sv = sign(s).ok_or("degenerate sample")?;
        if !cells.iter().any(|c| c.0 == sv) {
            cells.push((sv, inside(s)));
        }
        Ok(())
    };
    match circles.len() {
        0 => return Ok((0, 0)),
        1 => {
            push(circles[0], &mut cells)?;
            push(scale(circles[0], -1.0), &mut cells)?;
        }
        _ => {
            for i in 0..circles.len() {
                for j in i + 1..circles.len() {
                    for sgn in [1.0, -1.0] {
                        let v = scale(unit(cross(circles[i], circles[j])), sgn);
                        let (e1, e2) = basis(v);
                        let mut angs: Vec<f64> = Vec::new();
                        for &c in &circles {
                            if dot(c, v).abs() < 1e-12 {
                                let t = unit(cross(c, v));
                                for tt in [t, scale(t, -1.0)] {
                                    angs.push(dot(tt, e2).atan2(dot(tt, e1)));
                                }
                            }
                        }
                        angs.sort_by(f64::total_cmp);
                        let mut ins = Vec::new();
                        for k in 0..angs.len() {
                            let (a0, a1) = (angs[k], if k + 1 < angs.len() { angs[k + 1] } else { angs[0] + std::f64::consts::TAU });
                            let am = 0.5 * (a0 + a1);
                            let dir = add(scale(e1, am.cos()), scale(e2, am.sin()));
                            let s = unit(add(v, scale(dir, 1e-5)));
                            push(s, &mut cells)?;
                            ins.push(inside(s));
                        }
                        let changes = (0..ins.len()).filter(|&k| ins[k] != ins[(k + 1) % ins.len()]).count();
                        if changes > 2 {
                            return Err("nm");
                        }
                    }
                }
            }
        }
    }
    let n = cells.len();
    let mut root: Vec<usize> = (0..n).collect();
    fn find(r: &mut [usize], mut i: usize) -> usize {
        while r[i] != i {
            r[i] = r[r[i]];
            i = r[i];
        }
        i
    }
    for i in 0..n {
        for j in i + 1..n {
            if cells[i].1 == cells[j].1 && cells[i].0.iter().zip(&cells[j].0).filter(|(a, b)| a != b).count() == 1 {
                let (a, b) = (find(&mut root, i), find(&mut root, j));
                root[a] = b;
            }
        }
    }
    let mut comps = [std::collections::BTreeSet::new(), std::collections::BTreeSet::new()];
    for i in 0..n {
        let r = find(&mut root, i);
        comps[usize::from(cells[i].1)].insert(r);
    }
    let (ni, no) = (comps[1].len(), comps[0].len());
    if ni == 0 || no == 0 {
        return Ok((0, 0));
    }
    // A flat cone: some hemisphere's cells are exactly one component.
    let mut members: std::collections::BTreeMap<usize, std::collections::BTreeSet<usize>> = Default::default();
    for i in 0..n {
        let r = find(&mut root, i);
        members.entry(r).or_default().insert(i);
    }
    let mut flat = 0;
    for c in 0..circles.len() {
        let mut hit = false;
        for s in [true, false] {
            let h: std::collections::BTreeSet<usize> = (0..n).filter(|&i| cells[i].0[c] == s).collect();
            if members.values().any(|m| *m == h) {
                hit = true;
            }
        }
        if hit {
            flat += 1;
        }
    }
    Ok((ni + no - 1, flat))
}

fn verts_at(body: &Body<f64>, p: V3) -> usize {
    body.vertex_points().filter(|(_, q)| norm(sub([q.x, q.y, q.z], p)) < 1e-9).count()
}

// ---- solids --------------------------------------------------------------

struct Solid {
    body: AtRestBody<f64>,
    pieces: Pieces,
    vol: f64,
}

fn validate(what: &str, b: Body<f64>) -> Option<AtRestBody<f64>> {
    match AtRestBody::validate(b, tol()) {
        Ok(b) => Some(b),
        Err(e) => {
            println!("{what}: OPERAND-INVALID {e:?}");
            None
        }
    }
}

/// A convex CCW polygon's prism over `z ∈ [z0, z1]` as half-spaces.
fn polygon_prism(poly: &[(f64, f64)], (z0, z1): (f64, f64)) -> Vec<Plane> {
    let mut planes: Vec<Plane> = (0..poly.len())
        .map(|i| {
            let (p, q) = (poly[i], poly[(i + 1) % poly.len()]);
            let out = [q.1 - p.1, p.0 - q.0, 0.0];
            (out, out[0] * p.0 + out[1] * p.1)
        })
        .collect();
    planes.push(([0.0, 0.0, 1.0], z1));
    planes.push(([0.0, 0.0, -1.0], -z0));
    planes
}

/// A prism with a corner of interest, and its convex pieces.
#[derive(Clone)]
struct Corner {
    name: &'static str,
    profile: Vec<(f64, f64)>,
    pieces: Vec<Vec<(f64, f64)>>,
    v: V3,
}

fn wedge(name: &'static str, alpha: f64) -> Corner {
    let at = |t: f64| {
        let t = t.to_radians();
        let r = 2.0 / t.cos().abs().max(t.sin().abs());
        (r * t.cos(), r * t.sin())
    };
    let mut profile = vec![(0.0, 0.0), at(0.0)];
    let mut c = 45.0;
    while c < alpha {
        profile.push(at(c));
        c += 90.0;
    }
    profile.push(at(alpha));
    let pieces = (1..profile.len() - 1).map(|k| vec![profile[0], profile[k], profile[k + 1]]).collect();
    Corner { name, profile, pieces, v: [0.0, 0.0, 1.0] }
}

fn corners() -> Vec<Corner> {
    let l = vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)];
    vec![
        Corner {
            name: "Ltop",
            profile: l.clone(),
            pieces: vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)],
                vec![(0.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)],
            ],
            v: [1.0, 1.0, 1.0],
        },
        Corner {
            name: "vee300",
            profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5), (0.0, 4.0)],
            pieces: vec![
                vec![(0.0, 0.0), (2.0, 0.0), (2.0, 0.5), (0.0, 4.0)],
                vec![(2.0, 0.0), (4.0, 0.0), (4.0, 4.0), (2.0, 0.5)],
            ],
            v: [2.0, 0.5, 1.0],
        },
        Corner {
            name: "asym",
            profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0), (0.0, 1.5)],
            pieces: vec![
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.5)],
                vec![(1.0, 0.0), (4.0, 0.0), (4.0, 3.0), (1.0, 1.0)],
            ],
            v: [1.0, 1.0, 1.0],
        },
        wedge("w345", 345.0),
        wedge("w60", 60.0),
    ]
}

fn corner_pieces(c: &Corner) -> Pieces {
    c.pieces.iter().map(|p| polygon_prism(p, (0.0, 1.0))).collect()
}

fn corner_at_rest(c: &Corner) -> Option<Solid> {
    let body = validate(c.name, fixtures::prism::<f64>(&c.profile, 1.0, tol()).body)?;
    let pieces = corner_pieces(c);
    let vol = pieces_volume(&pieces);
    Some(Solid { body, pieces, vol })
}

/// `c` turned by the rows of `f` about its corner and moved to `at`.
fn corner_posed(c: &Corner, f: [V3; 3], at: V3) -> Option<Solid> {
    let map = |x: V3| {
        let r = sub(x, c.v);
        add(at, add(scale(f[0], r[0]), add(scale(f[1], r[1]), scale(f[2], r[2]))))
    };
    let mut b = Body::<f64>::new();
    fixtures::prism_ops(&mut b, &c.profile, (0.0, 1.0), |x, y, z| to_p(map([x, y, z])), fixtures::FaceGeometry::Certified, tol());
    fixtures::describe_as_intersections(&mut b, tol());
    let body = validate("posed corner", b)?;
    let turnn = |n: V3| add(scale(f[0], n[0]), add(scale(f[1], n[1]), scale(f[2], n[2])));
    let pieces: Pieces = corner_pieces(c)
        .into_iter()
        .map(|q| q.into_iter().map(|(n, d)| (turnn(n), d - dot(n, c.v) + dot(turnn(n), at))).collect())
        .collect();
    let vol = pieces_volume(&pieces);
    Some(Solid { body, pieces, vol })
}

/// The cube `v + a u + b w + c m`, `(a, b, c) ∈ lo + [0, side]³`.
fn cube(v: V3, [u, w, m]: [V3; 3], lo: V3, side: f64) -> Option<Solid> {
    let b = fixtures::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (lo[0] + side * x, lo[1] + side * y, lo[2] + side * z);
            to_p(add(v, add(scale(u, a), add(scale(w, b), scale(m, c)))))
        },
        tol(),
    );
    let body = validate("cube", b)?;
    let mut planes = Vec::new();
    for (axis, dir) in [u, w, m].into_iter().enumerate() {
        let base = dot(dir, v);
        planes.push((scale(dir, -1.0), -(base + lo[axis])));
        planes.push((dir, base + lo[axis] + side));
    }
    Some(Solid { body, pieces: vec![planes], vol: side.powi(3) })
}

/// A block over `outer` (CCW) less a prism hole `rim` (CCW), z ∈ [0, h],
/// with the convex pieces tiling it.
fn holed(outer: &[(f64, f64)], rim: &[(f64, f64)], tiles: &[Vec<(f64, f64)>], h: f64) -> Option<Solid> {
    let mut block = Body::<f64>::new();
    let ops = fixtures::prism_ops(&mut block, outer, (0.0, h), fixtures::identity_map::<f64>, fixtures::FaceGeometry::Declined, tol());
    let at = |z: f64| -> Vec<Point3<f64>> { rim.iter().map(|&(x, y)| Point3::new(x, y, z)).collect() };
    fixtures::drill_hole(&mut block, ops.sides[0].he_plus, ops.bottom.face, &at(h), &at(0.0), tol());
    fixtures::plane_every_face(&mut block, tol());
    fixtures::describe_as_intersections(&mut block, tol());
    let body = validate("holed block", block)?;
    let pieces: Pieces = tiles.iter().map(|t| polygon_prism(t, (0.0, h))).collect();
    let vol = pieces_volume(&pieces);
    Some(Solid { body, pieces, vol })
}

// ---- running -------------------------------------------------------------

type Op = fn(&AtRestBody<f64>, &AtRestBody<f64>, &BooleanDeclarations, Tol) -> Result<BooleanResult<f64>, BooleanError>;

fn mesh_col(body: &Body<f64>) -> String {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| mesh::tessellate(body, 0.05, tol()).map(|m| mesh::validate::check_mesh(&m)))) {
        Ok(Ok(Ok(()))) => "mesh=ok".into(),
        Ok(Ok(Err(e))) => format!("mesh=CHECKFAIL({:?})", format!("{e:?}").chars().take(60).collect::<String>()),
        Ok(Err(e)) => format!("mesh=refuse({})", format!("{e:?}").split(['(', ' ', '{']).next().unwrap_or("")),
        Err(_) => "mesh=PANIC".into(),
    }
}

/// Every op in both orders; `points` the pinch points to count at;
/// `cyl` substitutes the second solid's link pieces (a curved operand).
fn run_all(tag: &str, x: &Solid, y: &Solid, common: f64, points: &[V3], link_y: Option<&Pieces>) {
    let decls = BooleanDeclarations::default();
    let ly = link_y.unwrap_or(&y.pieces);
    for (order, p, q, lp, lq, vp) in [("xy", x, y, &x.pieces, ly, x.vol), ("yx", y, x, ly, &x.pieces, y.vol)] {
        let ops: [(&str, Op, f64, OpK); 3] = [
            ("U", topo::union_with, x.vol + y.vol - common, OpK::U),
            ("I", topo::intersect_with, common, OpK::I),
            ("S", topo::subtract_with, vp - common, OpK::S),
        ];
        for (op, f, want, k) in ops {
            let line = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let r = f(&p.body, &q.body, &decls, tol());
                let extra = match &r {
                    Ok(res) => res
                        .body()
                        .map(|bb| {
                            let mut s = Vec::new();
                            for &pt in points {
                                let nv = verts_at(&bb.body, pt);
                                match cones(pt, lp, lq, k) {
                                    Ok((c, fl)) if c - fl <= nv && nv <= c => s.push(format!("c{c}f{fl}v{nv}")),
                                    Ok((c, fl)) => s.push(format!("c{c}f{fl}v{nv}!CONEMISMATCH")),
                                    Err(e) => s.push(format!("c?{e}v{nv}")),
                                }
                            }
                            format!("{} {}", s.join(","), mesh_col(&bb.body))
                        })
                        .unwrap_or_default(),
                    Err(_) => points
                        .iter()
                        .map(|&pt| match cones(pt, lp, lq, k) {
                            Ok((c, fl)) => format!("c{c}f{fl}"),
                            Err(e) => format!("c?{e}"),
                        })
                        .collect::<Vec<_>>()
                        .join(","),
                };
                format!("{} | {extra}", outcome(r, want, tol()))
            }))
            .unwrap_or_else(|_| "PANIC".into());
            println!("{tag} {order} {op}: {line}");
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

fn frame_of(m: V3, psi: f64) -> [V3; 3] {
    let m = unit(m);
    let (u0, w0) = basis(m);
    let (u, w) = turn(u0, w0, psi);
    [u, w, m]
}

/// A seeded rotation (xorshift), as three orthonormal rows.
fn rotation(seed: u64) -> [V3; 3] {
    let mut s = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut r = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    };
    let a = unit([r(), r(), r()]);
    let b0 = [r(), r(), r()];
    let b = unit(sub(b0, scale(a, dot(a, b0))));
    [a, b, cross(a, b)]
}

fn shard() -> (usize, usize) {
    let s = std::env::var("R1_SHARD").unwrap_or_else(|_| "0/1".into());
    let w: Vec<usize> = s.split('/').map(|x| x.parse().unwrap()).collect();
    (w[0], w[1])
}

fn main() {
    let set = std::env::args().nth(1).unwrap_or_else(|| "multi".into());
    let (sk, sn) = shard();
    let mut idx = 0usize;
    let mut mine = || {
        idx += 1;
        idx % sn == sk
    };
    match set.as_str() {
        // A corner at a cube's edge or corner: up to six or eight
        // crossings, so up to three or four cones at one point and cones
        // holding several runs of one operand.
        "multi" => {
            for c in corners() {
                let Some(x) = corner_at_rest(&c) else { continue };
                for (dn, m) in directions(48) {
                    for (place, lo, psis) in [
                        ("face", [-2.0, -2.0, 0.0], &[0.0][..]),
                        ("edge", [0.0, -2.0, 0.0], &[0.3, 1.9, 4.0][..]),
                        ("corner", [0.0, 0.0, 0.0], &[0.3, 1.9, 4.0][..]),
                    ] {
                        for &psi in psis {
                            if !mine() {
                                continue;
                            }
                            let Some(y) = cube(c.v, frame_of(m, psi), lo, 4.0) else { continue };
                            let common = common_volume(&x.pieces, &y.pieces);
                            run_all(&format!("{} {dn} {place} psi={psi}", c.name), &x, &y, common, &[c.v], None);
                        }
                    }
                }
            }
        }
        // Two corners, one posed on the other by a seeded rotation.
        "pair" => {
            let cs = corners();
            for a in &cs {
                for b in &cs {
                    let Some(x) = corner_at_rest(a) else { continue };
                    for k in 0..60u64 {
                        if !mine() {
                            continue;
                        }
                        let Some(y) = corner_posed(b, rotation(k + 1), a.v) else { continue };
                        let common = common_volume(&x.pieces, &y.pieces);
                        run_all(&format!("{} {} k={k}", a.name, b.name), &x, &y, common, &[a.v], None);
                    }
                }
            }
        }
        // Two sliver corners whose top planes meet at a right angle, each
        // sliver crossing the lune the planes bound: the slivers cut it
        // into up to four cones of the intersection at one point.
        "x4" => {
            let cs = corners();
            for a in cs.iter().filter(|c| c.name != "w60") {
                for b in cs.iter().filter(|c| c.name != "w60") {
                    let Some(x) = corner_at_rest(a) else { continue };
                    for (ta, tb) in [(0.0, 0.0), (0.3, 0.0), (0.0, 0.3), (-0.3, 0.2), (0.2, -0.3)] {
                        for k in 0..72 {
                            if !mine() {
                                continue;
                            }
                            let f2 = unit([-1.0, ta, tb]);
                            let beta = std::f64::consts::TAU * (k as f64 + 0.31) / 72.0;
                            let (p, q) = basis(f2);
                            let (f0, _) = turn(p, q, beta);
                            let f1 = cross(f2, f0);
                            let Some(y) = corner_posed(b, [f0, f1, f2], a.v) else { continue };
                            let common = common_volume(&x.pieces, &y.pieces);
                            run_all(&format!("{} {} t=({ta},{tb}) k={k}", a.name, b.name), &x, &y, common, &[a.v], None);
                        }
                    }
                }
            }
        }
        // Three collinear reflex corners; the cube's near face in a plane
        // through all three, turned about their line.
        "stair3" => {
            let c = Corner {
                name: "stair3",
                profile: vec![(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (3.0, 1.0), (3.0, 2.0), (2.0, 2.0), (2.0, 3.0), (1.0, 3.0), (1.0, 4.0), (0.0, 4.0)],
                pieces: vec![
                    vec![(0.0, 0.0), (4.0, 0.0), (4.0, 1.0), (0.0, 1.0)],
                    vec![(0.0, 1.0), (3.0, 1.0), (3.0, 2.0), (0.0, 2.0)],
                    vec![(0.0, 2.0), (2.0, 2.0), (2.0, 3.0), (0.0, 3.0)],
                    vec![(0.0, 3.0), (1.0, 3.0), (1.0, 4.0), (0.0, 4.0)],
                ],
                v: [2.0, 2.0, 1.0],
            };
            let Some(x) = corner_at_rest(&c) else { return };
            let pts = [[3.0, 1.0, 1.0], [2.0, 2.0, 1.0], [1.0, 3.0, 1.0]];
            let e = unit([-1.0, 1.0, 0.0]);
            let (e1, e2) = basis(e);
            for t in 0..360 {
                if !mine() {
                    continue;
                }
                let th = std::f64::consts::TAU * (t as f64 + 0.37) / 360.0;
                let m = add(scale(e1, th.cos()), scale(e2, th.sin()));
                for side in [8.0] {
                    let Some(y) = cube(pts[1], frame_of(m, 0.0), [-side / 2.0, -side / 2.0, 0.0], side) else { continue };
                    let common = common_volume(&x.pieces, &y.pieces);
                    run_all(&format!("stair3 t={t}"), &x, &y, common, &pts, None);
                }
            }
        }
        // An L block's reflex corner and a square hole's corner on one
        // plane, turned about their line: a notch pinch and an island
        // pinch on one cube face. `diag` also passes the far hole corner.
        "islnotch" => {
            let outer = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 2.0), (2.0, 4.0), (0.0, 4.0)];
            let rim = [(0.5, 0.5), (1.5, 0.5), (1.5, 1.5), (0.5, 1.5)];
            let tiles = vec![
                vec![(0.0, 0.0), (4.0, 0.0), (4.0, 0.5), (0.0, 0.5)],
                vec![(0.0, 0.5), (0.5, 0.5), (0.5, 1.5), (0.0, 1.5)],
                vec![(1.5, 0.5), (4.0, 0.5), (4.0, 1.5), (1.5, 1.5)],
                vec![(0.0, 1.5), (4.0, 1.5), (4.0, 2.0), (0.0, 2.0)],
                vec![(0.0, 2.0), (2.0, 2.0), (2.0, 4.0), (0.0, 4.0)],
            ];
            let h = 1.0;
            let Some(x) = holed(&outer, &rim, &tiles, h) else { return };
            for (line, p, q, extra) in [
                ("diag", [2.0, 2.0, h], [1.5, 1.5, h], vec![[0.5, 0.5, h]]),
                ("skew", [2.0, 2.0, h], [1.5, 0.5, h], vec![]),
                ("hole2", [1.5, 0.5, h], [0.5, 1.5, h], vec![]),
            ] {
                let e = unit(sub(q, p));
                let (e1, e2) = basis(e);
                let mut pts = vec![p, q];
                pts.extend(extra);
                for t in 0..240 {
                    if !mine() {
                        continue;
                    }
                    let th = std::f64::consts::TAU * (t as f64 + 0.37) / 240.0;
                    let m = add(scale(e1, th.cos()), scale(e2, th.sin()));
                    let mid = scale(add(p, q), 0.5);
                    let Some(y) = cube(mid, frame_of(m, 0.0), [-5.0, -5.0, 0.0], 10.0) else { continue };
                    let common = common_volume(&x.pieces, &y.pieces);
                    run_all(&format!("islnotch {line} t={t}"), &x, &y, common, &pts, None);
                }
            }
        }
        // A corner on a cylinder's wall: the curved face passes the pinch.
        "cyl" => {
            for c in corners() {
                let Some(x) = corner_at_rest(&c) else { continue };
                for (dn, m) in directions(40) {
                    for psi in [0.0, 0.9, 2.2] {
                        if !mine() {
                            continue;
                        }
                        cyl_case(&c, &x, unit(m), psi, &format!("{} cyl {dn} psi={psi}", c.name));
                    }
                }
            }
        }
        // Near-tangent tilts of the cube's face about the corner's edges.
        "nt" => {
            for c in corners() {
                let Some(x) = corner_at_rest(&c) else { continue };
                let n = c.profile.len();
                let i = c.profile.iter().position(|&(a, b)| a == c.v[0] && b == c.v[1]).unwrap();
                let (pp, a, b) = (c.profile[i], c.profile[(i + n - 1) % n], c.profile[(i + 1) % n]);
                let edges = [[a.0 - pp.0, a.1 - pp.1, 0.0], [b.0 - pp.0, b.1 - pp.1, 0.0], [0.0, 0.0, -1.0]];
                for (j, &ed) in edges.iter().enumerate() {
                    let ed = unit(ed);
                    let (p1, p2) = basis(ed);
                    for al in 0..8 {
                        let th = std::f64::consts::TAU * (al as f64 + 0.25) / 8.0;
                        let base = add(scale(p1, th.cos()), scale(p2, th.sin()));
                        for d in [1e-3, -1e-3, 1e-5, -1e-5, 1e-7] {
                            for (place, lo) in [("face", [-2.0, -2.0, 0.0]), ("edge", [0.0, -2.0, 0.0])] {
                                if !mine() {
                                    continue;
                                }
                                let m = unit(add(base, scale(ed, d)));
                                let Some(y) = cube(c.v, frame_of(m, 0.7), lo, 4.0) else { continue };
                                let common = common_volume(&x.pieces, &y.pieces);
                                run_all(&format!("{} nt e{j} a{al} d{d:e} {place}", c.name), &x, &y, common, &[c.v], None);
                            }
                        }
                    }
                }
            }
        }
        _ => panic!("set"),
    }
}

// ---- the cylinder ----------------------------------------------------------

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
    const X: [f64; 8] = [0.991455371120813, 0.949107912342759, 0.864864423359769, 0.741531185599394, 0.586087235467691, 0.405845151377397, 0.207784955007898, 0.0];
    const WK: [f64; 8] = [0.022935322010529, 0.063092092629979, 0.104790010322250, 0.140653259715525, 0.169004726639267, 0.190350578064785, 0.204432940075298, 0.209482141084728];
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

/// A cylinder of radius 3, length 8, whose side holds `c.v`, its outward
/// normal there `−m`, its axis ⊥ `m` turned by `psi`.
fn cyl_case(c: &Corner, x: &Solid, m: V3, psi: f64, tag: &str) {
    let r = 3.0;
    let half = 4.0;
    let (u0, _) = basis(m);
    let (a, _) = turn(u0, cross(m, u0), psi);
    let centre = add(c.v, scale(m, r));
    let z = [0.0, 0.0, 1.0];
    let ax = cross(z, a);
    let ang = dot(z, a).clamp(-1.0, 1.0).acos();
    let r1 = if norm(ax) < 1e-12 {
        Affine3::identity()
    } else {
        Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), to_v(unit(ax)), ang)
    };
    let back = r1.inverse().transform_vec(to_v(scale(m, -1.0)));
    let phi = back.y.atan2(back.x);
    let rz = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), phi - 0.9);
    let map = Affine3::translation(to_v(centre)) * r1 * rz;
    let raw = sweep::test_support::cylinder_of_arcs_at(4, r, Point2::new(0.0, 0.0), -half, 2.0 * half, tol());
    let Ok(Ok(body)) = topo::transform_rigid(&raw, &map, tol()).map(|b| AtRestBody::validate(b, tol())) else {
        println!("{tag}: CYL-INVALID");
        return;
    };
    let vol = std::f64::consts::PI * r * r * 2.0 * half;
    let common: f64 = x
        .pieces
        .iter()
        .map(|pl| {
            let fp: Vec<Plane> = pl
                .iter()
                .map(|&(n, d)| {
                    let nn = map.inverse().transform_vec(to_v(n));
                    ([nn.x, nn.y, nn.z], d - dot(n, centre))
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
    // The link: the tangent plane at v, the cylinder on m's side.
    let link = vec![vec![(scale(m, -1.0), dot(scale(m, -1.0), c.v))]];
    let y = Solid { body, pieces: vec![], vol };
    run_all(tag, x, &y, common, &[c.v], Some(&link));
}

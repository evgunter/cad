//! R1 review probes for PR 6266 (cone-apex unit A). Not for merge.
//!
//! Oracle, independent of kernel classification: the expected volume of
//! cone ∩ box by slicing (disc ∩ convex polygon, clipped per height and
//! integrated), the result's volume from its tessellation (divergence
//! theorem), and membership at a lattice by the mesh's generalized
//! winding number against the analytic region. Every verdict is printed
//! as one `ROW` line; the run never asserts, so one bad pose does not
//! hide the others.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use crate::common::wrap_tip::one_wrap_edge;
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use std::panic::{AssertUnwindSafe, catch_unwind};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

fn row(line: &str) {
    use std::io::Write;
    let name = std::thread::current().name().unwrap_or("x").replace("::", "-");
    let dir = std::env::var("R1_ROWS").unwrap_or_else(|_| "/home/user/r1-rows".into());
    let _ = std::fs::create_dir_all(&dir);
    let eps = Tol::witness().eps();
    let mut f = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{dir}/{eps:e}-{name}.log"))
        .unwrap();
    writeln!(f, "{line}").unwrap();
}

fn tol() -> Tol {
    Tol::witness()
}

const APEX: Point3<f64> = Point3::new(0.0, 1.0, 0.0);

fn cone_body(alpha: f64, rev: Revolution<f64>) -> topo::Body<f64> {
    let r = alpha.to_radians().tan();
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(r, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        rev,
        tol(),
    )
}

/// The cone kinds: `T` the tipped (one wrap edge), `P` the plain revolve
/// (two meridians at the apex).
pub fn cone(alpha: f64, kind: char) -> AtRestBody<f64> {
    match kind {
        'T' => finished(
            "tipped",
            one_wrap_edge(cone_body(alpha, Revolution::Full), true).0,
            tol(),
        ),
        _ => finished("plain", cone_body(alpha, Revolution::Full), tol()),
    }
}

/// A box `[0, l] × [−l/2, l/2]²` moved so its face `x = 0` lies on the
/// plane through the apex with normal `n` (the box beyond it), the local
/// point `(0, a, b)` on the apex, spun `spin` about `n`.
pub struct Placed {
    pub body: AtRestBody<f64>,
    pub inv: Affine3<f64>,
    pub lo: [f64; 3],
    pub hi: [f64; 3],
}

pub fn placed_box(n: Vec3<f64>, (a, b): (f64, f64), spin: f64, l: f64) -> Placed {
    let n = n.normalize();
    let x = Vec3::new(1.0, 0.0, 0.0);
    let align = if x.cross(n).norm() < 1e-15 {
        if n.x > 0.0 {
            Affine3::identity()
        } else {
            Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), PI)
        }
    } else {
        Affine3::rotation_about_axis(
            Point3::origin(),
            x.cross(n).normalize(),
            x.dot(n).clamp(-1.0, 1.0).acos(),
        )
    };
    let m = Affine3::translation(APEX - Point3::origin())
        * Affine3::rotation_about_axis(Point3::origin(), n, spin)
        * align
        * Affine3::translation(Vec3::new(0.0, -a, -b));
    let (lo, hi) = ([0.0, -l / 2.0, -l / 2.0], [l, l / 2.0, l / 2.0]);
    let brick = topo::test_support::brick((lo[0], hi[0]), (lo[1], hi[1]), (lo[2], hi[2]), tol());
    let moved =
        topo::transform_rigid(&brick, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap();
    Placed {
        body: finished("box", moved, tol()),
        inv: m.inverse(),
        lo,
        hi,
    }
}

fn local(inv: &Affine3<f64>, p: Point3<f64>) -> [f64; 3] {
    let q = inv.transform_point(p);
    [q.x, q.y, q.z]
}

/// Signed distance-like: > 0 inside the box.
fn box_f(pl: &Placed, p: Point3<f64>) -> f64 {
    let q = local(&pl.inv, p);
    (0..3)
        .map(|i| (q[i] - pl.lo[i]).min(pl.hi[i] - q[i]))
        .fold(f64::INFINITY, f64::min)
}

/// > 0 inside the cone (apex up, height 1), in true distance units.
fn cone_f(alpha: f64, p: Point3<f64>) -> f64 {
    let a = alpha.to_radians();
    let r = a.tan();
    ((r * (1.0 - p.y) - p.x.hypot(p.z)) * a.cos())
        .min(p.y)
        .min(1.0 - p.y)
}

fn clip(poly: &[(f64, f64)], (a, b, d): (f64, f64, f64)) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    let n = poly.len();
    for i in 0..n {
        let p = poly[i];
        let q = poly[(i + 1) % n];
        let fp = a * p.0 + b * p.1 - d;
        let fq = a * q.0 + b * q.1 - d;
        if fp <= 0.0 {
            out.push(p);
        }
        if (fp < 0.0 && fq > 0.0) || (fp > 0.0 && fq < 0.0) {
            let t = fp / (fp - fq);
            out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
        }
    }
    out
}

fn area(poly: &[(f64, f64)]) -> f64 {
    let n = poly.len();
    (0..n)
        .map(|i| {
            let (p, q) = (poly[i], poly[(i + 1) % n]);
            p.0 * q.1 - q.0 * p.1
        })
        .sum::<f64>()
        .abs()
        / 2.0
}

/// The volume of cone ∩ box, sliced.
fn cone_and_box(alpha: f64, pl: &Placed) -> f64 {
    let r = alpha.to_radians().tan();
    solid_and_box(&|y| r * (1.0 - y), (0.0, 1.0), pl)
}

/// The volume of the solid of revolution about `y` of radius `rad(y)` over
/// `[y0, y1]`, ∩ box, sliced.
fn solid_and_box(rad: &dyn Fn(f64) -> f64, (y0, y1): (f64, f64), pl: &Placed) -> f64 {
    let o = pl.inv.transform_point(Point3::origin());
    let w: Vec<Vec3<f64>> = [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ]
    .iter()
    .map(|&e| pl.inv.transform_vec(e))
    .collect();
    let arr = |v: Vec3<f64>| [v.x, v.y, v.z];
    let w: Vec<[f64; 3]> = w.into_iter().map(arr).collect();
    let (oc, wc) = ([o.x, o.y, o.z], |i: usize, j: usize| w[j][i]);
    let nseg = 1024;
    let steps = 1200;
    let mut total = 0.0;
    let h = (y1 - y0) / steps as f64;
    for k in 0..steps {
        let y = y0 + (k as f64 + 0.5) * h;
        let rad = rad(y);
        // Scale so the polygon's area is the disc's.
        let s = rad * ((2.0 * PI / nseg as f64) / (2.0 * PI / nseg as f64).sin()).sqrt();
        let mut poly: Vec<(f64, f64)> = (0..nseg)
            .map(|i| {
                let t = 2.0 * PI * i as f64 / nseg as f64;
                (s * t.cos(), s * t.sin())
            })
            .collect();
        for i in 0..3 {
            // g_i(p) = wc(i,0) x + wc(i,1) y + wc(i,2) z + oc[i]
            let (ax, ay, az) = (wc(i, 0), wc(i, 1), wc(i, 2));
            let base = ay * y + oc[i];
            // g ≤ hi
            poly = clip(&poly, (ax, az, pl.hi[i] - base));
            if poly.is_empty() {
                break;
            }
            // g ≥ lo
            poly = clip(&poly, (-ax, -az, base - pl.lo[i]));
            if poly.is_empty() {
                break;
            }
        }
        total += area(&poly) * h;
    }
    total
}

struct Tri {
    tris: Vec<[Point3<f64>; 3]>,
}

fn mesh_of(body: &topo::Body<f64>, delta: f64) -> Result<Tri, String> {
    let m = mesh::tessellate(body, delta, tol()).map_err(|e| format!("tess {e:?}"))?;
    let check = mesh::validate::check_mesh(&m).map_err(|e| format!("mesh {e:?}"));
    let tris = m
        .patches
        .iter()
        .flat_map(|p| p.triangles.iter())
        .map(|t| t.map(|i| m.positions[i as usize]))
        .collect();
    check?;
    Ok(Tri { tris })
}

fn volume(t: &Tri) -> f64 {
    t.tris
        .iter()
        .map(|[a, b, c]| {
            let o = Point3::origin();
            let (a, b, c) = (*a - o, *b - o, *c - o);
            a.dot(b.cross(c)) / 6.0
        })
        .sum()
}

fn winding(t: &Tri, p: Point3<f64>) -> f64 {
    let mut w = 0.0;
    for [a, b, c] in &t.tris {
        let (a, b, c) = (*a - p, *b - p, *c - p);
        let (la, lb, lc) = (a.norm(), b.norm(), c.norm());
        let num = a.dot(b.cross(c));
        let den = la * lb * lc + a.dot(b) * lc + b.dot(c) * la + c.dot(a) * lb;
        w += 2.0 * num.atan2(den);
    }
    w / (4.0 * PI)
}

pub enum Op {
    CnB,
    BnC,
    CuB,
    BuC,
    CmB,
    BmC,
}

/// Runs the six ops of the cone against the placed box, printing a row
/// each. `extra`: a label.
pub fn six(label: &str, alpha: f64, c: &AtRestBody<f64>, pl: &Placed, exact: Option<f64>) {
    let r = alpha.to_radians().tan();
    let vc = PI * r * r / 3.0;
    let l = pl.hi[0] - pl.lo[0];
    let vb = l * l * l;
    let i = exact.unwrap_or_else(|| cone_and_box(alpha, pl));
    let delta = 2e-4;
    let lateral = PI * r * (r * r + 1.0).sqrt();
    let vtol = 3.0 * delta * lateral + 1e-4;
    let eps = tol().eps();
    let ops: [(&str, Op, f64); 6] = [
        ("c∩b", Op::CnB, i),
        ("b∩c", Op::BnC, i),
        ("c∪b", Op::CuB, vc + vb - i),
        ("b∪c", Op::BuC, vc + vb - i),
        ("c∖b", Op::CmB, vc - i),
        ("b∖c", Op::BmC, vb - i),
    ];
    for (name, op, want) in ops {
        let got = catch_unwind(AssertUnwindSafe(|| match op {
            Op::CnB => intersect(c, &pl.body, tol()),
            Op::BnC => intersect(&pl.body, c, tol()),
            Op::CuB => union(c, &pl.body, tol()),
            Op::BuC => union(&pl.body, c, tol()),
            Op::CmB => subtract(c, &pl.body, tol()),
            Op::BmC => subtract(&pl.body, c, tol()),
        }));
        let region = |p: Point3<f64>| -> f64 {
            let (cf, bf) = (cone_f(alpha, p), box_f(pl, p));
            match op {
                Op::CnB | Op::BnC => cf.min(bf),
                Op::CuB | Op::BuC => cf.max(bf),
                Op::CmB => cf.min(-bf),
                Op::BmC => bf.min(-cf),
            }
        };
        let verdict = match got {
            Err(_) => "PANIC".to_string(),
            Ok(Err(e)) => format!("REFUSE {e:?}"),
            Ok(Ok(BooleanResult::Empty)) => {
                if want.abs() < 1e-6 {
                    "OK empty".to_string()
                } else {
                    format!("WRONG empty want {want:.6}")
                }
            }
            Ok(Ok(BooleanResult::Body(b))) => {
                let shells = b.body.shells().count();
                match mesh_of(&b.body, delta) {
                    Err(e) => {
                        let kv = catch_unwind(AssertUnwindSafe(|| {
                            topo::test_support::pinned_volume("r1", &b.body, tol()).volume
                        }))
                        .map_or_else(|_| "kvol-panic".to_string(), |v| format!("kvol {v:.3e}"));
                        let coarse = mesh_of(&b.body, 1e-2).map(|t| format!("{:.3e}", volume(&t)));
                        format!("BUILT-UNMESHED shells {shells} want {want:.3e} {kv} coarse {coarse:?} {e}")
                    }
                    Ok(t) => {
                        let v = volume(&t);
                        let mut bad = 0;
                        let mut probed = 0;
                        let k = 8;
                        let ext = r + 0.15;
                        for ix in 0..=k {
                            for iy in 0..=k {
                                for iz in 0..=k {
                                    let f = |j: usize| j as f64 / k as f64;
                                    let p = Point3::new(
                                        -ext + 2.0 * ext * f(ix) + 0.0123,
                                        -0.15 + 1.3 * f(iy) + 0.0071,
                                        -ext + 2.0 * ext * f(iz) - 0.0093,
                                    );
                                    let g = region(p);
                                    if g.abs() < 0.02 {
                                        continue;
                                    }
                                    probed += 1;
                                    let inside = winding(&t, p) > 0.5;
                                    if inside != (g > 0.0) {
                                        bad += 1;
                                    }
                                }
                            }
                        }
                        let census = topo::validate_pseudomanifold(&b.body, &b.contacts, tol())
                            .map_or_else(|e| format!("census-err {}", e.len()), |()| "census-ok".into());
                        let tag = if (v - want).abs() <= vtol && bad == 0 {
                            "OK"
                        } else {
                            "WRONG"
                        };
                        format!(
                            "{tag} vol {v:.6} want {want:.6} dv {:.2e} member-bad {bad}/{probed} shells {shells} {census}",
                            v - want
                        )
                    }
                }
            }
        };
        row(&format!("ROW eps={eps:e} {label} {name} {verdict}"));
    }
}

fn n_of(theta_deg: f64, psi: f64) -> Vec3<f64> {
    let t = theta_deg.to_radians();
    Vec3::new(t.sin() * psi.cos(), t.cos(), t.sin() * psi.sin())
}

/// The sweep for one half-angle: normals at θ from the axis and azimuth
/// ψ, off the graze by more than a degree; a big box (other faces clear)
/// and a small one (other faces cutting the cone too).
fn sweep_alpha(alpha: f64) {
    for kind in ['T', 'P'] {
        let c = cone(alpha, kind);
        for theta in [3.0, 20.0, 40.0, 62.0, 89.0, 90.0, 91.0, 118.0, 140.0, 177.0] {
            if (theta - (90.0 - alpha)).abs() < 1.0 || (theta - (90.0 + alpha)).abs() < 1.0 {
                continue;
            }
            for psi in [0.0, 0.37, 2.2, PI, 4.1] {
                let n = n_of(theta, psi);
                for (bx, ab, spin, l) in [
                    ("big", (0.31, -0.17), 0.23, 9.0),
                    ("small", (0.12, 0.27), 0.71, 1.3),
                ] {
                    let pl = placed_box(n, ab, spin, l);
                    six(
                        &format!("α={alpha} {kind} θ={theta} ψ={psi:.3} {bx}"),
                        alpha,
                        &c,
                        &pl,
                        None,
                    );
                }
            }
        }
    }
}

#[test]
fn r1_sweep_15() {
    sweep_alpha(15.0);
}
#[test]
fn r1_sweep_30() {
    sweep_alpha(30.0);
}
#[test]
fn r1_sweep_45() {
    sweep_alpha(45.0);
}
#[test]
fn r1_sweep_60() {
    sweep_alpha(60.0);
}
#[test]
fn r1_sweep_75() {
    sweep_alpha(75.0);
}

/// Grazes and near-grazes: the tangent plane along ruling φ, either side,
/// tilted `d` about the ruling's circumferential direction.
/// The cone of half-angle `alpha` beyond `n·(p − apex) > 0`, exactly.
fn beyond(alpha: f64, n: Vec3<f64>) -> f64 {
    let r = alpha.to_radians().tan();
    let n = n.normalize();
    let m = n.x.hypot(n.z);
    if m < 1e-15 {
        return if n.y < 0.0 { PI * r * r / 3.0 } else { 0.0 };
    }
    let d = n.y / m;
    let seg = if d >= r {
        0.0
    } else if d <= -r {
        PI * r * r
    } else {
        r * r * (d / r).acos() - d * (r * r - d * d).sqrt()
    };
    seg / 3.0
}

#[test]
fn r1_grazes_20() {
    grazes(20.0);
}
#[test]
fn r1_grazes_45() {
    grazes(45.0);
}
#[test]
fn r1_grazes_70() {
    grazes(70.0);
}

fn grazes(alpha: f64) {
    {
        let a = alpha.to_radians();
        for kind in ['T', 'P'] {
            let c = cone(alpha, kind);
            for phi in [0.0f64, 0.8, PI / 2.0, PI, 4.0] {
                let tn = Vec3::new(a.cos() * phi.cos(), a.sin(), a.cos() * phi.sin());
                let t = Vec3::new(-phi.sin(), 0.0, phi.cos());
                for d in [0.0, 1e-7, -1e-7, 1e-5, -1e-5, 1e-3, -1e-3, 3e-2, -3e-2] {
                    let tilted = Affine3::rotation_about_axis(Point3::origin(), t, d).transform_vec(tn);
                    for side in [1.0, -1.0] {
                        let n = tilted * side;
                        let pl = placed_box(n, (0.31, -0.17), 0.23, 9.0);
                        six(
                            &format!("GRAZE α={alpha} {kind} φ={phi:.2} d={d:e} side={side}"),
                            alpha,
                            &c,
                            &pl,
                            Some(beyond(alpha, n)),
                        );
                    }
                }
            }
        }
    }
}

/// A box `lo..hi` placed by `m`.
fn placed_by(m: Affine3<f64>, lo: [f64; 3], hi: [f64; 3]) -> Placed {
    let brick = topo::test_support::brick((lo[0], hi[0]), (lo[1], hi[1]), (lo[2], hi[2]), tol());
    let moved =
        topo::transform_rigid(&brick, &geom_core::test_support::rigid(&m, tol()), tol()).unwrap();
    Placed {
        body: finished("box", moved, tol()),
        inv: m.inverse(),
        lo,
        hi,
    }
}

/// Runs the six ops of a solid of revolution `s` (radius `rad`, over
/// `[y0, y1]`, its region `sf`) against `pl`.
fn six_general(
    label: &str,
    s: &AtRestBody<f64>,
    (rad, span, vs): (&dyn Fn(f64) -> f64, (f64, f64), f64),
    sf: &dyn Fn(Point3<f64>) -> f64,
    pl: &Placed,
    lattice: (Point3<f64>, Point3<f64>),
) {
    let l: Vec<f64> = (0..3).map(|i| pl.hi[i] - pl.lo[i]).collect();
    let vb = l[0] * l[1] * l[2];
    let i = solid_and_box(rad, span, pl);
    let delta = 2e-4;
    let eps = tol().eps();
    let ops: [(&str, Op, f64); 6] = [
        ("s∩b", Op::CnB, i),
        ("b∩s", Op::BnC, i),
        ("s∪b", Op::CuB, vs + vb - i),
        ("b∪s", Op::BuC, vs + vb - i),
        ("s∖b", Op::CmB, vs - i),
        ("b∖s", Op::BmC, vb - i),
    ];
    for (name, op, want) in ops {
        let got = catch_unwind(AssertUnwindSafe(|| match op {
            Op::CnB => intersect(s, &pl.body, tol()),
            Op::BnC => intersect(&pl.body, s, tol()),
            Op::CuB => union(s, &pl.body, tol()),
            Op::BuC => union(&pl.body, s, tol()),
            Op::CmB => subtract(s, &pl.body, tol()),
            Op::BmC => subtract(&pl.body, s, tol()),
        }));
        let region = |p: Point3<f64>| -> f64 {
            let (cf, bf) = (sf(p), box_f(pl, p));
            match op {
                Op::CnB | Op::BnC => cf.min(bf),
                Op::CuB | Op::BuC => cf.max(bf),
                Op::CmB => cf.min(-bf),
                Op::BmC => bf.min(-cf),
            }
        };
        let verdict = match got {
            Err(_) => "PANIC".to_string(),
            Ok(Err(e)) => format!("REFUSE {e:?}"),
            Ok(Ok(BooleanResult::Empty)) => format!("EMPTY want {want:.6}"),
            Ok(Ok(BooleanResult::Body(b))) => match mesh_of(&b.body, delta) {
                Err(e) => {
                    let kv = catch_unwind(AssertUnwindSafe(|| {
                        topo::test_support::pinned_volume("r1", &b.body, tol()).volume
                    }))
                    .map_or_else(|_| "kvol-panic".to_string(), |v| format!("{v:.6}"));
                    let tier = topo::validate_geometric(&b.body, tol()).map_or_else(|e| format!("tier3-err {e:?}"), |()| "tier3-ok".into());
                    format!("BUILT-UNMESHED want {want:.6} kvol {kv} shells {} {tier} {e}", b.body.shells().count())
                }
                Ok(t) => {
                    let v = volume(&t);
                    let (mut bad, mut probed) = (0, 0);
                    let k = 10;
                    for ix in 0..=k {
                        for iy in 0..=k {
                            for iz in 0..=k {
                                let f = |j: usize, a: f64, b: f64| a + (b - a) * j as f64 / k as f64;
                                let p = Point3::new(
                                    f(ix, lattice.0.x, lattice.1.x) + 0.00123,
                                    f(iy, lattice.0.y, lattice.1.y) + 0.0071,
                                    f(iz, lattice.0.z, lattice.1.z) - 0.00093,
                                );
                                let g = region(p);
                                if g.abs() < 0.004 {
                                    continue;
                                }
                                probed += 1;
                                if (winding(&t, p) > 0.5) != (g > 0.0) {
                                    bad += 1;
                                }
                            }
                        }
                    }
                    format!(
                        "BUILT vol {v:.6} want {want:.6} dv {:.2e} member-bad {bad}/{probed} shells {}",
                        v - want,
                        b.body.shells().count()
                    )
                }
            },
        };
        row(&format!("ROW eps={eps:e} {label} {name} {verdict}"));
    }
}

/// **Claim 3, non-cone**: a plane cutting a thin cylinder along two
/// parallel rulings, the box's slanted side crossing one ruling at `y = 1`
/// and the other at `y = 10`: a germ at `y = 0` on the long ruling faces
/// the short ruling's end across, nearer than its own partner.
#[test]
fn r1_cylinder_two_rulings_across() {
    let rr = 0.1;
    let cyl = finished(
        "cyl",
        revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(rr, 0.0), 0.0),
                (Point2::new(rr, 12.0), 0.0),
                (Point2::new(0.0, 12.0), 0.0),
            ],
            Revolution::Full,
            tol(),
        ),
        tol(),
    );
    let sf = move |p: Point3<f64>| (rr - p.x.hypot(p.z)).min(p.y).min(12.0 - p.y);
    let zcs: Vec<f64> = std::env::var("R1_ZC").map_or(vec![0.05, 0.0, -0.03, 0.08], |v| v.split(',').map(|x| x.parse().unwrap()).collect());
    for zc in zcs {
        let xr = (rr * rr - zc * zc).sqrt();
        for (y_short, y_long) in [(1.0, 10.0), (3.0, 7.0), (5.0, 5.5), (0.5, 11.0)] {
            // The slanted side through (−xr, y_short) and (xr, y_long), in z = zc.
            let t = (2.0 * xr).atan2(y_long - y_short);
            // Local x across the side into the box, local y along it.
            let m = Affine3::translation(Vec3::new(-xr, y_short, zc))
                * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.0, 0.0, 1.0), -t);
            // Box material at local x < 0 (toward −x, so the long ruling at
            // +x... check both) and z ≥ 0 local.
            for (side, lo, hi) in [
                ("left", [-4.0, -30.0, 0.0], [0.0, 30.0, 3.0]),
                ("right", [0.0, -30.0, 0.0], [4.0, 30.0, 3.0]),
                ("left-down", [-4.0, -30.0, -3.0], [0.0, 30.0, 0.0]),
                ("right-down", [0.0, -30.0, -3.0], [4.0, 30.0, 0.0]),
            ] {
                let pl = placed_by(m, lo, hi);
                six_general(
                    &format!("CYL zc={zc} y=({y_short},{y_long}) {side}"),
                    &cyl,
                    (&|_| rr, (0.0, 12.0), PI * rr * rr * 12.0),
                    &sf,
                    &pl,
                    (Point3::new(-0.13, -0.3, -0.13), Point3::new(0.13, 12.3, 0.13)),
                );
            }
        }
    }
}

/// Prints any op's verdict, kind only (claim 2's refusal and panic hunt).
fn verdicts(label: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>) {
    let eps = tol().eps();
    let ops: [(&str, Op); 6] = [
        ("a∩b", Op::CnB),
        ("b∩a", Op::BnC),
        ("a∪b", Op::CuB),
        ("b∪a", Op::BuC),
        ("a∖b", Op::CmB),
        ("b∖a", Op::BmC),
    ];
    for (name, op) in ops {
        let got = catch_unwind(AssertUnwindSafe(|| match op {
            Op::CnB => intersect(a, b, tol()),
            Op::BnC => intersect(b, a, tol()),
            Op::CuB => union(a, b, tol()),
            Op::BuC => union(b, a, tol()),
            Op::CmB => subtract(a, b, tol()),
            Op::BmC => subtract(b, a, tol()),
        }));
        let v = match got {
            Err(e) => format!(
                "PANIC {}",
                e.downcast_ref::<String>()
                    .cloned()
                    .or_else(|| e.downcast_ref::<&str>().map(|s| (*s).to_string()))
                    .unwrap_or_default()
            ),
            Ok(Err(e)) => format!("REFUSE {e:?}"),
            Ok(Ok(BooleanResult::Empty)) => "EMPTY".into(),
            Ok(Ok(BooleanResult::Body(b))) => {
                let kv = catch_unwind(AssertUnwindSafe(|| {
                    topo::test_support::pinned_volume("r1", &b.body, tol()).volume
                }))
                .map_or_else(|_| "kvol-panic".to_string(), |v| format!("{v:.6}"));
                format!("BUILT kvol {kv} shells {}", b.body.shells().count())
            }
        };
        row(&format!("ROW eps={eps:e} {label} {name} {v}"));
    }
}

fn wedge(theta: f64, alpha: f64) -> AtRestBody<f64> {
    finished("wedge", cone_body(alpha, Revolution::Partial(theta)), tol())
}

/// Wedges of the cone against general planes through the apex (not only
/// those holding the axis), and planes holding a wedge's flat face.
#[test]
fn r1_wedges_general() {
    for alpha in [30.0f64, 60.0] {
        for theta in [0.7, PI / 2.0, 2.5, PI, 4.5, 5.9] {
            let w = wedge(theta, alpha);
            let r = alpha.to_radians().tan();
            // In (X, Y) = (x, −z): the revolve sweeps from +x toward −z.
            let in_wedge = move |p: Point3<f64>| {
                let (start, end) = (-p.z, theta.sin() * p.x + theta.cos() * p.z);
                let side = if theta <= PI { start.min(end) } else { start.max(end) };
                cone_f(alpha, p).min(side * 0.0 + side)
            };
            let vw = theta / (2.0 * PI) * PI * r * r / 3.0;
            for (th, psi) in [(25.0, 0.4), (60.0, 2.0), (100.0, 3.5), (150.0, 5.0), (90.0, 1.1), (75.0, 4.4)] {
                let n = n_of(th, psi);
                let pl = placed_box(n, (0.31, -0.17), 0.23, 9.0);
                // Volume of wedge ∩ half-space by sampling the closed form numerically:
                // integrate over the base sector the 1/3 cone over the region beyond.
                let share = {
                    // base points q = (ρ cos φ, 0, −ρ sin φ) for φ in [0, θ] (X, Y frame).
                    let (nphi, nrho) = (2000, 400);
                    let mut s = 0.0;
                    for i in 0..nphi {
                        let phi = (i as f64 + 0.5) * theta / nphi as f64;
                        for j in 0..nrho {
                            let rho = (j as f64 + 0.5) * r / nrho as f64;
                            let q = Point3::new(rho * phi.cos(), 0.0, -rho * phi.sin());
                            if n.dot(q - APEX) > 0.0 {
                                s += rho * (theta / nphi as f64) * (r / nrho as f64);
                            }
                        }
                    }
                    s / 3.0
                };
                let label = format!("WEDGE α={alpha} θ={theta:.3} n=({th},{psi})");
                verdicts(&label, &w, &pl.body);
                row(&format!(
                    "WANT {label} ∩ {share:.6} w∖b {:.6} vw {vw:.6}",
                    vw - share
                ));
                let _ = &in_wedge;
            }
            // A box face holding the wedge's flat face at azimuth 0 (the
            // plane z = 0), the box beyond either side.
            for n in [Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0)] {
                let pl = placed_box(n, (0.31, -0.17), 0.23, 9.0);
                verdicts(&format!("WEDGEFLAT α={alpha} θ={theta:.3} n={n:?}"), &w, &pl.body);
            }
        }
    }
}

/// The apex against a box edge and a box corner, another cone's apex at
/// several turns, a sphere through the apex, a cylinder through it, and
/// another cone's lateral face through it.
#[test]
fn r1_apex_pairs_and_curved() {
    for kind in ['T', 'P'] {
        let c = cone(45.0, kind);
        // Box edge through the apex: the box's edge along local z at
        // local (0, l/2) placed on the apex, the edge's direction tilted.
        for (tilt, spin) in [(0.0, 0.0), (0.3, 0.9), (1.2, 2.0), (2.0, 4.0)] {
            let n = n_of(90.0 + tilt * 20.0, spin);
            let pl = placed_box(n, (4.5, 0.0), spin, 9.0);
            verdicts(&format!("APEXONEDGE {kind} t={tilt} s={spin}"), &c, &pl.body);
            let pl = placed_box(n, (4.5, 4.5), spin, 9.0);
            verdicts(&format!("APEXONCORNER {kind} t={tilt} s={spin}"), &c, &pl.body);
        }
        // Another cone's apex on the apex, the copy turned.
        for (axis, ang) in [
            (Vec3::new(1.0, 0.0, 0.0), PI),
            (Vec3::new(1.0, 0.0, 0.0), PI / 2.0),
            (Vec3::new(0.3, 0.0, 1.0), 2.0),
            (Vec3::new(0.0, 1.0, 0.0), 0.7),
        ] {
            let m = Affine3::rotation_about_axis(APEX, axis, ang);
            let copy = finished(
                "copy",
                topo::transform_rigid(
                    &cone_body(45.0, Revolution::Full),
                    &geom_core::test_support::rigid(&m, tol()),
                    tol(),
                )
                .unwrap(),
                tol(),
            );
            verdicts(&format!("APEXONAPEX {kind} axis={axis:?} ang={ang:.2}"), &c, &copy);
        }
        // A sphere through the apex, either side.
        for (u, rad) in [
            (Vec3::new(0.0, 1.0, 0.0), 0.5),
            (Vec3::new(0.0, -1.0, 0.0), 0.3),
            (Vec3::new(1.0, 0.0, 0.0), 0.3),
            (Vec3::new(0.4, 1.0, 0.2), 0.4),
        ] {
            let ctr = APEX + u.normalize() * rad;
            let cy = ctr.y;
            let at = sweep::test_support::ball_poled_y(rad, Vec3::new(0.0, 0.0, 0.0), tol());
            let moved = topo::transform_rigid(
                &at,
                &geom_core::RigidMotion::translation(ctr - Point3::origin()).unwrap(),
                tol(),
            )
            .unwrap();
            let s = finished("sphere", moved, tol());
            verdicts(&format!("SPHERE {kind} u={u:?} cy={cy:.2} r={rad}"), &c, &s);
        }
    }
}

/// The small-box poses that refuse `ResultInvalid`, with the box's face
/// shifted off the apex by `±0.02` along its normal: whether the props
/// refusal is the apex's.
#[test]
fn r1_small_box_off_the_apex() {
    for (alpha, theta, psi) in [(15.0, 90.0, 2.2), (15.0, 89.0, 3.142), (30.0, 62.0, 0.37), (45.0, 62.0, 2.2)] {
        let c = cone(alpha, 'T');
        let n = n_of(theta, psi).normalize();
        for shift in [0.0, 0.02, -0.02] {
            let pl0 = placed_box(n, (0.12, 0.27), 0.71, 1.3);
            let m = Affine3::translation(n * shift) * pl0.inv.inverse();
            let pl = placed_by(m, pl0.lo, pl0.hi);
            six(
                &format!("SHIFT α={alpha} T θ={theta} ψ={psi} shift={shift}"),
                alpha,
                &c,
                &pl,
                None,
            );
        }
    }
}

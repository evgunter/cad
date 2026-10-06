//! R2 review probe (PR #4135), end to end through
//! `topo::sweep_split_admitting_cones`: widening, narrowing and full
//! cones, posed off-axis and rotated, at three scales, against turned
//! cubes, bricks and tilted rods, both operand orders. Oracle: the
//! double cone's signed residual `ρ cos α − |h| sin α` in the cone's own
//! frame (the pose inverted), sampled along each of the other operand's
//! edges and bisected, kept on the face's axial window, plus each cap.
//! Prints a census; a split whose new vertices disagree is WRONG.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanError};

#[derive(Clone, Copy, Debug)]
struct Frustum {
    y0: f64,
    y1: f64,
    r0: f64,
    k: f64,
}

impl Frustum {
    fn r(self, y: f64) -> f64 {
        self.r0 + self.k * (y - self.y0)
    }
    fn apex_y(self) -> f64 {
        self.y0 - self.r0 / self.k
    }
    fn body(self, pose: Affine3<f64>) -> AtRestBody<f64> {
        let mut pts = vec![(0.0, self.y0), (self.r0, self.y0)];
        let r1 = self.r(self.y1);
        if r1 > 1e-15 * self.y1.abs().max(1.0) {
            pts.push((r1, self.y1));
        }
        pts.push((0.0, self.y1));
        let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
        let body = revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness())
            .unwrap()
            .body;
        let body = topo::transform_rigid(&body, &pose, Tol::witness()).unwrap();
        finished("the frustum", body, Tol::witness())
    }
    /// Residual of the double cone, in the frustum's own frame.
    fn res(self, q: Point3<f64>) -> f64 {
        let alpha = self.k.abs().atan();
        let rho = q.x.hypot(q.z);
        rho * alpha.cos() - (q.y - self.apex_y()).abs() * alpha.sin()
    }
}

fn sign_changes(f: &impl Fn(f64) -> f64, t0: f64, t1: f64) -> Vec<f64> {
    let n = 50_000u32;
    let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(n);
    let mut out = Vec::new();
    let mut prev = f(at(0));
    for k in 0..n {
        let next = f(at(k + 1));
        if (prev < 0.0) != (next < 0.0) {
            let (mut a, mut b) = (at(k), at(k + 1));
            let sa = f(a) < 0.0;
            for _ in 0..90 {
                let m = 0.5 * (a + b);
                if (f(m) < 0.0) == sa {
                    a = m;
                } else {
                    b = m;
                }
            }
            out.push(0.5 * (a + b));
        }
        prev = next;
    }
    out
}

/// The oracle's crossings (world frame) along every edge of `other`.
fn oracle(f: Frustum, pose: Affine3<f64>, other: &Body<f64>) -> Vec<Point3<f64>> {
    let inv = pose.inverse();
    let mut out = Vec::new();
    for (_, e) in other.edges() {
        let c = other
            .get_curve_geom(e.curve)
            .and_then(topo::CurveGeom::certified)
            .expect("a certified edge");
        let (t0, t1) = c.params();
        let (t0, t1) = (t0.min(t1), t0.max(t1));
        let local = |t: f64| inv.transform_point(c.carrier().eval(t));
        for t in sign_changes(&|t| f.res(local(t)), t0, t1) {
            let q = local(t);
            if q.y > f.y0 && q.y < f.y1 {
                out.push(c.carrier().eval(t));
            }
        }
        for y in [f.y0, f.y1] {
            let r = f.r(y);
            if r <= 0.0 {
                continue;
            }
            for t in sign_changes(&|t| local(t).y - y, t0, t1) {
                let q = local(t);
                if q.x.hypot(q.z) < r {
                    out.push(c.carrier().eval(t));
                }
            }
        }
    }
    out
}

fn new_vertices(before: &Body<f64>, split: &Body<f64>, sc: f64) -> Vec<Point3<f64>> {
    let old: Vec<Point3<f64>> = before.vertex_points().map(|(_, p)| p).collect();
    split
        .vertex_points()
        .map(|(_, p)| p)
        .filter(|p| old.iter().all(|q| (*p - *q).norm() > 1e-12 * sc.max(1.0)))
        .collect()
}

#[derive(Default)]
struct Census {
    fixtures_unfinished: usize,
    ok: usize,
    refused: std::collections::BTreeMap<String, usize>,
    wrong: Vec<String>,
    max_dist_over_eps: f64,
}

fn kind(e: &BooleanError) -> String {
    let s = format!("{e:?}");
    s.split([' ', '{', '(']).next().unwrap_or("").to_string()
        + &if let BooleanError::Escalated { decision, diag } = e {
            let d = format!("{diag:?}");
            let row = d.split("predicate: ").nth(1).map(|r| r.split(',').next().unwrap_or("")).unwrap_or("");
            format!("({decision:?} row {row})")
        } else {
            String::new()
        }
}

fn run_inner(label: &str, f: Frustum, pose: Affine3<f64>, other: &AtRestBody<f64>, sc: f64, cen: &mut Census) {
    let tol = Tol::witness();
    let cone = f.body(pose);
    let truth = oracle(f, pose, other);
    for swapped in [false, true] {
        let (a, b) = if swapped { (&**other, &*cone) } else { (&*cone, &**other) };
        match topo::sweep_split_admitting_cones(a, b, tol) {
            Err(e) => {
                let k = kind(&e);
                if k.contains("bool_line_cone_lead") && cen.refused.get(&k).copied().unwrap_or(0) < 3 {
                    println!("LEAD-ESC {label} swapped={swapped}: {e:?}");
                }
                *cen.refused.entry(k).or_default() += 1;
            }
            Ok((sa, sb, _, _)) => {
                let split = if swapped { sa } else { sb };
                let mut got = new_vertices(other, &split, sc);
                if got.len() != truth.len() {
                    cen.wrong.push(format!(
                        "{label} swapped={swapped}: {} new vertices vs oracle {}: {got:?} vs {truth:?}",
                        got.len(),
                        truth.len()
                    ));
                    continue;
                }
                cen.ok += 1;
                for q in &truth {
                    let (i, d) = got
                        .iter()
                        .enumerate()
                        .map(|(i, p)| (i, (*p - *q).norm()))
                        .min_by(|x, y| x.1.total_cmp(&y.1))
                        .unwrap();
                    cen.max_dist_over_eps = cen.max_dist_over_eps.max(d / tol.eps());
                    if d > tol.eps().max(1e-10 * sc) {
                        cen.wrong.push(format!("{label} swapped={swapped}: oracle {q:?} is {d:e} from nearest vertex"));
                    }
                    got.swap_remove(i);
                }
            }
        }
    }
}

fn run(label: &str, f: Frustum, pose: Affine3<f64>, other: impl FnOnce() -> AtRestBody<f64>, sc: f64, cen: &mut Census) {
    let built = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| (other(), f.body(pose))));
    match built {
        Ok((o, _)) => run_inner(label, f, pose, &o, sc, cen),
        Err(_) => cen.fixtures_unfinished += 1,
    }
}

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn r(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.f()
    }
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    sweep::test_support::brick(x, y, z, Tol::witness())
}

fn rod(r: f64, h: f64) -> Body<f64> {
    let b = sweep::test_support::prism_at(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        -h / 2.0,
        h,
        Tol::witness(),
    );
    let onto_y = Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1.0, 0.0, 0.0), -core::f64::consts::FRAC_PI_2);
    topo::transform_rigid(&b, &onto_y, Tol::witness()).unwrap()
}

fn place(b: &Body<f64>, rot_axis: Vec3<f64>, angle: f64, c: Point3<f64>) -> AtRestBody<f64> {
    let t = Affine3::rotation_about_axis(Point3::origin(), rot_axis.normalize(), angle);
    let b = topo::transform_rigid(b, &t, Tol::witness()).unwrap();
    let to = Affine3::translation(Vec3::new(c.x, c.y, c.z));
    finished("the other", topo::transform_rigid(&b, &to, Tol::witness()).unwrap(), Tol::witness())
}

#[test]
fn probe_e2e_random_poses() {
    let mut rng = Rng(0x0bad_cafe_1234_5678);
    let mut cen = Census::default();
    for &sc in &[1e-3, 1.0, 1e3] {
        let kinds = [
            ("widening", Frustum { y0: 0.0, y1: sc, r0: 0.5 * sc, k: 0.5 }),
            ("narrowing", Frustum { y0: 0.0, y1: sc, r0: sc, k: -0.5 }),
            ("full", Frustum { y0: 0.0, y1: sc, r0: sc, k: -1.0 }),
            ("narrow-angle", Frustum { y0: 0.0, y1: sc, r0: 0.2 * sc, k: 0.05 }),
            ("wide-angle", Frustum { y0: 0.0, y1: 0.2 * sc, r0: 0.1 * sc, k: 8.0 }),
        ];
        for (name, f) in kinds {
            for j in 0..8 {
                let pose = if j % 2 == 0 {
                    Affine3::identity()
                } else {
                    let rot = Affine3::rotation_about_axis(
                        Point3::origin(),
                        Vec3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0)).normalize(),
                        rng.r(0.1, 3.0),
                    );
                    let tr = Affine3::translation(Vec3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0)) * sc);
                    tr * rot
                };
                // A point near the wall at a random height, in the frustum frame.
                let y = rng.r(f.y0, f.y1);
                let phi = rng.r(0.0, 6.283);
                let r = f.r(y).max(0.0);
                let local = Point3::new(r * phi.cos(), y, r * phi.sin());
                let at = pose.transform_point(local);
                let s = sc * rng.r(0.1, 0.3);
                let axis = Vec3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0));
                let angle = rng.r(0.2, 2.5);
                let other = move || match j % 3 {
                    0 => place(&brick((-s, s), (-s, s), (-s, s)), axis, angle, at),
                    1 => place(&brick((-0.5 * s, 0.5 * s), (-2.0 * s, 2.0 * s), (-0.3 * s, 0.3 * s)), axis, angle, at),
                    _ => place(&rod(0.4 * s, 3.0 * s), axis, angle, at),
                };
                run(&format!("{name} s{sc} j{j}"), f, pose, other, sc, &mut cen);
            }
        }
    }
    println!("PROBE e2e eps={:e}: unfinished fixtures {} ok {} wrong {} max dist/eps {:e}", Tol::witness().eps(), cen.fixtures_unfinished, cen.ok, cen.wrong.len(), cen.max_dist_over_eps);
    for (k, n) in &cen.refused {
        println!("  REFUSED {n} × {k}");
    }
    for w in cen.wrong.iter().take(20) {
        println!("  WRONG {w}");
    }
}

/// Edges running near the apex, along generators, and both nappes.
#[test]
fn probe_e2e_apex_and_generators() {
    let mut cen = Census::default();
    for &sc in &[1e-3, 1.0, 1e3] {
        let full = Frustum { y0: 0.0, y1: sc, r0: sc, k: -1.0 };
        // Thin bricks through the apex region at offset δ from the axis.
        for p in 0..14 {
            let d = sc * 10f64.powf(-13.0 + f64::from(p));
            if d > 0.3 * sc {
                break;
            }
            let other = move || finished(
                "thin",
                brick((d, d + 0.02 * sc), (0.5 * sc, 1.5 * sc), (-0.01 * sc, 0.01 * sc)),
                Tol::witness(),
            );
            run(&format!("apex-pass s{sc} δ{d:e}"), full, Affine3::identity(), other, sc, &mut cen);
        }
        // A brick edge lying along a generator of the full cone (x = 1 − y, z = 0),
        // rotated so one edge is the generator, then offset by δ outward.
        for p in 0..8 {
            let d = sc * 10f64.powf(-12.0 + 1.5 * f64::from(p));
            let b = brick((0.0, 0.1 * sc), (0.0, 0.3 * sc), (-0.05 * sc, 0.05 * sc));
            let other = move || place(&b, Vec3::new(0.0, 0.0, 1.0), core::f64::consts::FRAC_PI_4, Point3::new(0.6 * sc + d, 0.4 * sc, 0.0));
            run(&format!("generator s{sc} δ{d:e}"), full, Affine3::identity(), other, sc, &mut cen);
        }
    }
    println!("PROBE e2e-apex eps={:e}: unfinished fixtures {} ok {} wrong {} max dist/eps {:e}", Tol::witness().eps(), cen.fixtures_unfinished, cen.ok, cen.wrong.len(), cen.max_dist_over_eps);
    for (k, n) in &cen.refused {
        println!("  REFUSED {n} × {k}");
    }
    for w in cen.wrong.iter().take(20) {
        println!("  WRONG {w}");
    }
}

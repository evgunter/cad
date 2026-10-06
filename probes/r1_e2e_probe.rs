//! Reviewer probe (reach-dual4135-r1): finished bodies through
//! `topo::sweep_split_admitting_cones`, widened beyond the PR's poses
//! (narrow and wide cones, frusta, a full cone, off-axis and rotated
//! poses, cubes, bricks, rods, tilted cylinders, edges near the apex and
//! along a generator, both operand orders, scales 1e-3 / 1 / 1e3).
//! Oracle: the double cone's elevation `ρ cos α − |h| sin α` and each
//! cap's height, sampled along every edge of the other operand in the
//! canonical frame, sign changes bisected, kept on the face; mapped by
//! the pose. Mounted temporarily into `crates/sweep/tests/all.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::test_support::finished;
use sweep::{Revolution, revolve};
use topo::{AtRestBody, Body, BooleanError};

#[derive(Clone, Copy, Debug)]
struct Fr {
    y0: f64,
    y1: f64,
    r0: f64,
    k: f64,
}
impl Fr {
    fn r(self, y: f64) -> f64 {
        self.r0 + self.k * (y - self.y0)
    }
    fn apex_y(self) -> f64 {
        self.y0 - self.r0 / self.k
    }
    fn body(self) -> Body<f64> {
        let mut pts = vec![(0.0, self.y0), (self.r0, self.y0)];
        let r1 = self.r(self.y1);
        if r1 > 1e-15 {
            pts.push((r1, self.y1));
        }
        pts.push((0.0, self.y1));
        let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
        revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness()).unwrap().body
    }
    fn elev(self, q: Point3<f64>) -> f64 {
        let a = self.k.abs().atan();
        let rho = q.x.hypot(q.z);
        let h = q.y - self.apex_y();
        rho * a.cos() - h.abs() * a.sin()
    }
    fn crossings(self, p: &dyn Fn(f64) -> Point3<f64>, t0: f64, t1: f64) -> Vec<Point3<f64>> {
        let mut out = Vec::new();
        for t in roots(&|t| self.elev(p(t)), t0, t1) {
            let q = p(t);
            if q.y > self.y0 && q.y < self.y1 {
                out.push(q);
            }
        }
        for y in [self.y0, self.y1] {
            let r = self.r(y);
            if r <= 0.0 {
                continue;
            }
            for t in roots(&|t| p(t).y - y, t0, t1) {
                let q = p(t);
                if q.x.hypot(q.z) < r {
                    out.push(q);
                }
            }
        }
        out
    }
}

fn roots(f: &dyn Fn(f64) -> f64, t0: f64, t1: f64) -> Vec<f64> {
    let n = 40_000;
    let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(n);
    let mut out = Vec::new();
    for k in 0..n {
        let (mut a, mut b) = (at(k), at(k + 1));
        let fa = f(a);
        if (fa < 0.0) == (f(b) < 0.0) {
            continue;
        }
        for _ in 0..90 {
            let m = 0.5 * (a + b);
            if (f(m) < 0.0) == (fa < 0.0) {
                a = m;
            } else {
                b = m;
            }
        }
        out.push(0.5 * (a + b));
    }
    out
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    sweep::test_support::brick(x, y, z, Tol::witness())
}
fn rod(r: f64, h: f64, tilt: f64, c: [f64; 3]) -> Body<f64> {
    let b = sweep::test_support::prism_at(
        vec![(Point2::new(-r, 0.0), 1.0), (Point2::new(r, 0.0), 1.0)],
        -h / 2.0,
        h,
        Tol::witness(),
    );
    let tol = Tol::witness();
    let b = topo::transform_rigid(&b, &Affine3::rotation_about_axis(Point3::origin(), Vec3::new(1., 0., 0.), -core::f64::consts::FRAC_PI_2), tol).unwrap();
    let b = topo::transform_rigid(&b, &Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0., 0., 1.), tilt), tol).unwrap();
    topo::transform_rigid(&b, &Affine3::translation(Vec3::new(c[0], c[1], c[2])), tol).unwrap()
}
fn turn(b: &Body<f64>, about: Point3<f64>, axis: Vec3<f64>, ang: f64) -> Body<f64> {
    topo::transform_rigid(b, &Affine3::rotation_about_axis(about, axis.normalize(), ang), Tol::witness()).unwrap()
}

#[derive(Default)]
struct Out {
    ok: usize,
    refused: Vec<String>,
    wrong: Vec<String>,
}

/// One pose: `cone` (canonical), `other` (canonical), then `pose` applied
/// to both. Checks both operand orders.
fn run(label: &str, f: Fr, other: &Body<f64>, pose: &Affine3<f64>, out: &mut Out) {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut truth = Vec::new();
    for (_, e) in other.edges() {
        let c = other.get_curve_geom(e.curve).and_then(topo::CurveGeom::certified).expect("edge");
        let (t0, t1) = c.params();
        truth.extend(f.crossings(&|t| c.carrier().eval(t), t0.min(t1), t0.max(t1)));
    }
    let truth: Vec<Point3<f64>> = truth.into_iter().map(|p| pose.transform_point(p)).collect();
    let cone = finished("cone", topo::transform_rigid(&f.body(), pose, tol).unwrap(), tol);
    let oth: AtRestBody<f64> = finished("other", topo::transform_rigid(other, pose, tol).unwrap(), tol);
    let scale = f.y1.abs().max(f.r0.abs()).max(1.0);
    let reach = 10.0 * eps + 1e-13 * scale;
    for swapped in [false, true] {
        let lab = format!("{label} [{}]", if swapped { "other first" } else { "cone first" });
        let (a, b) = if swapped { (&*oth, &*cone) } else { (&*cone, &*oth) };
        match topo::sweep_split_admitting_cones(a, b, tol) {
            Err(e) => out.refused.push(format!("{lab}: {}", short(&e))),
            Ok((sa, sb, _, _)) => {
                let split = if swapped { sa } else { sb };
                let old: Vec<Point3<f64>> = oth.vertex_points().map(|(_, p)| p).collect();
                let mut got: Vec<Point3<f64>> = split
                    .vertex_points()
                    .map(|(_, p)| p)
                    .filter(|p| old.iter().all(|q| (*p - *q).norm() > 1e-12 * scale))
                    .collect();
                let n_got = got.len();
                let mut bad = Vec::new();
                for q in &truth {
                    match got.iter().enumerate().map(|(i, p)| (i, (*p - *q).norm())).min_by(|x, y| x.1.total_cmp(&y.1)) {
                        Some((i, d)) if d <= reach => {
                            got.swap_remove(i);
                        }
                        Some((_, d)) => bad.push(format!("oracle {q:?} nearest new vertex {d:e}")),
                        None => bad.push(format!("oracle {q:?} unmatched")),
                    }
                }
                if !got.is_empty() {
                    bad.push(format!("{} extra new vertices {:?}", got.len(), got));
                }
                if bad.is_empty() {
                    out.ok += 1;
                    let on_wall = truth.iter().filter(|q| { let c = pose.inverse().transform_point(**q); c.y > f.y0 + 1e-12 * scale && c.y < f.y1 - 1e-12 * scale && (c.x.hypot(c.z) - f.r(c.y)).abs() < 1e-6 * scale }).count();
                    eprintln!("R1E2E-OK {lab}: {} oracle crossings, {on_wall} on the cone wall", truth.len());
                } else {
                    out.wrong.push(format!("{lab}: {n_got} new vs {} oracle: {bad:?}", truth.len()));
                }
            }
        }
    }
}

fn short(e: &BooleanError) -> String {
    let s = format!("{e:?}");
    s.chars().take(160).collect()
}

#[test]
fn r1_cone_lane_end_to_end() {
    let y = Vec3::new(0.0, 1.0, 0.0);
    let poses: Vec<(&str, Affine3<f64>)> = vec![
        ("identity", Affine3::identity()),
        ("off-axis rotated", Affine3::translation(Vec3::new(0.37, -1.3, 2.1)) * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, 0.8, -0.5).normalize(), 0.9)),
    ];
    let mut out = Out::default();
    for s in [1e-3, 1.0, 1e3] {
        let wide_fr = Fr { y0: 0.0, y1: 0.04 * s, r0: 0.1 * s, k: 20.0 };
        let narrow_fr = Fr { y0: 0.0, y1: 1.0 * s, r0: 0.3 * s, k: 0.02 };
        let narrowing = Fr { y0: 0.0, y1: 1.0 * s, r0: 1.0 * s, k: -0.5 };
        let full = Fr { y0: 0.0, y1: 1.0 * s, r0: 1.0 * s, k: -1.0 };
        let full_narrow = Fr { y0: 0.0, y1: 1.0 * s, r0: 0.05 * s, k: -0.05 };
        let built = std::panic::catch_unwind(|| { let cases: Vec<(String, Fr, Body<f64>)> = vec![
            (format!("s={s} narrow wall × brick"), narrow_fr, brick((0.25 * s, 0.6 * s), (0.3 * s, 0.7 * s), (-0.1 * s, 0.1 * s))),
            (format!("s={s} narrow wall × tilted rod"), narrow_fr, rod(0.05 * s, 0.4 * s, 0.7, [0.32 * s, 0.5 * s, 0.0])),
            (format!("s={s} wide wall × brick"), wide_fr, brick((0.5 * s, 1.5 * s), (0.01 * s, 0.03 * s), (-0.2 * s, 0.2 * s))),
            (format!("s={s} wide wall × flat rod"), wide_fr, rod(0.008 * s, 2.0 * s, core::f64::consts::FRAC_PI_2, [0.0, 0.02 * s, 0.3 * s])),
            (format!("s={s} narrowing × turned brick"), narrowing, turn(&brick((-0.2 * s, 0.2 * s), (-0.15 * s, 0.15 * s), (-0.1 * s, 0.1 * s)), Point3::origin(), Vec3::new(1., 1., 1.), 0.7)
                .pipe(|b| topo::transform_rigid(&b, &Affine3::translation(Vec3::new(-0.7 * s, 0.5 * s, 0.1 * s)), Tol::witness()).unwrap())),
            (format!("s={s} full cone × tilted rod near rim"), full, rod(0.1 * s, 0.6 * s, -0.5, [0.55 * s, 0.4 * s, 0.1 * s])),
            (format!("s={s} full narrow cone × thin brick through"), full_narrow, brick((-0.2 * s, 0.2 * s), (0.4 * s, 0.6 * s), (0.01 * s, 0.015 * s))),
            (format!("s={s} full cone × brick past apex d=1e-3"), full, brick((1e-3 * s, 0.2 * s), (0.9 * s, 1.1 * s), (-0.1 * s, 0.1 * s))),
            (format!("s={s} full cone × brick past apex d=1e-6"), full, brick((1e-6 * s, 0.2 * s), (0.9 * s, 1.1 * s), (-0.1 * s, 0.1 * s))),
            (format!("s={s} full cone × brick past apex d=1e-8"), full, brick((1e-8 * s, 0.2 * s), (0.9 * s, 1.1 * s), (-0.1 * s, 0.1 * s))),
            (format!("s={s} full cone × brick edge on a generator"), full, turn(&brick((0.2 * s, 0.8 * s), (-0.05 * s, 0.0), (0.0, 0.05 * s)), Point3::origin(), Vec3::new(0., 0., 1.), -core::f64::consts::FRAC_PI_4)
                .pipe(|b| topo::transform_rigid(&b, &Affine3::translation(Vec3::new(0.0, 1.0 * s, 0.0)), Tol::witness()).unwrap())),
            (format!("s={s} full cone × brick edge 1e-4 off a generator"), full, turn(&brick((0.2 * s, 0.8 * s), (-0.05 * s, 0.0), (1e-4 * s, 0.05 * s)), Point3::origin(), Vec3::new(0., 0., 1.), -core::f64::consts::FRAC_PI_4)
                .pipe(|b| topo::transform_rigid(&b, &Affine3::translation(Vec3::new(0.0, 1.0 * s, 0.0)), Tol::witness()).unwrap())),
        ]; cases });
        let Ok(cases) = built else { eprintln!("R1E2E fixtures at s={s} panicked"); continue; };
        for (label, f, other) in &cases {
            for (pl, pose) in &poses {
                let pose = if *pl == "identity" { *pose } else { Affine3::translation(Vec3::new(0.37 * s, -1.3 * s, 2.1 * s)) * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.3, 0.8, -0.5).normalize(), 0.9) };
                let lab = format!("{label} @{pl}");
                let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let mut o = Out::default();
                    run(&lab, *f, other, &pose, &mut o);
                    o
                }));
                match r {
                    Ok(o) => {
                        out.ok += o.ok;
                        out.refused.extend(o.refused);
                        out.wrong.extend(o.wrong);
                    }
                    Err(_) => out.refused.push(format!("{lab}: FIXTURE construction panicked")),
                }
            }
        }
    }
    let _ = y;
    eprintln!("R1E2E eps={:e}: ok {} refused {} WRONG {}", Tol::witness().eps(), out.ok, out.refused.len(), out.wrong.len());
    for r in &out.refused {
        eprintln!("R1E2E-REFUSED {r}");
    }
    for w in &out.wrong {
        eprintln!("R1E2E-WRONG {w}");
    }
}

trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}

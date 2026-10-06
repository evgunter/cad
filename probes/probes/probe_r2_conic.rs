//! R2 review probe (PR #4135): the conic × cone door against an
//! independent oracle — the double cone's signed residual
//! `ρ cos α − |h| sin α` read from each point, sampled densely round the
//! whole turn and bisected — and the exact distance to the double cone
//! in the meridian half-plane. Prints a census; panics on nothing, so a
//! wrong answer is counted rather than hidden behind the first one.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use core::f64::consts::{PI, TAU};

use super::*;
use geom_core::{Point3, Tol, Vec3};

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
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.r(-1.0, 1.0), self.r(-1.0, 1.0), self.r(-1.0, 1.0));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Cone {
    apex: Point3<f64>,
    axis: Vec3<f64>,
    alpha: f64,
}

impl Cone {
    fn surface(self) -> geom::Surface<f64> {
        geom::Surface::Cone {
            apex: self.apex,
            axis: self.axis,
            half_angle: self.alpha,
            u_ref: self.axis.orthonormal_basis().0,
        }
    }
    fn res(self, p: Point3<f64>) -> f64 {
        let q = p - self.apex;
        let h = q.dot(self.axis);
        let rho = (q - self.axis * h).norm();
        rho * self.alpha.cos() - h.abs() * self.alpha.sin()
    }
    /// Exact distance to the double cone.
    fn dist(self, p: Point3<f64>) -> f64 {
        let q = p - self.apex;
        let h = q.dot(self.axis);
        let rho = (q - self.axis * h).norm();
        let (s, c) = self.alpha.sin_cos();
        let mut best = f64::INFINITY;
        for g in [(s, c), (s, -c)] {
            let along = rho * g.0 + h * g.1;
            let d = if along <= 0.0 {
                rho.hypot(h)
            } else {
                (rho * g.1.abs() - h * g.1.signum() * s).abs()
            };
            best = best.min(d);
        }
        best
    }
}

fn ellipse(c: Point3<f64>, n: Vec3<f64>, u: Vec3<f64>, a: f64, b: f64) -> geom::Curve3<f64> {
    let u = (u - n * u.dot(n)).normalize();
    if a == b {
        geom::Curve3::Circle {
            center: c,
            axis: n,
            radius: a,
            u_ref: u,
        }
    } else {
        geom::Curve3::Ellipse {
            center: c,
            axis: n,
            major: a,
            minor: b,
            u_ref: u,
        }
    }
}

fn bisect(f: &impl Fn(f64) -> f64, mut a: f64, mut b: f64) -> f64 {
    let sa = f(a) < 0.0;
    for _ in 0..90 {
        let m = 0.5 * (a + b);
        if (f(m) < 0.0) == sa {
            a = m;
        } else {
            b = m;
        }
    }
    0.5 * (a + b)
}

/// Sign changes of `f` on `[t0, t1]`: sampled, and at every sampled
/// local minimum of `|f|` whose neighbours share its sign, the extremum
/// is refined by golden section; one that crosses zero is bisected on
/// both sides, so a root pair inside one sample cell is found too.
fn sign_changes(f: &impl Fn(f64) -> f64, t0: f64, t1: f64, n: u32) -> Vec<f64> {
    let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(n);
    let v: Vec<f64> = (0..=n).map(|k| f(at(k))).collect();
    let mut out = Vec::new();
    for k in 0..n {
        let (i, j) = (k as usize, k as usize + 1);
        if (v[i] < 0.0) != (v[j] < 0.0) {
            out.push(bisect(f, at(k), at(k + 1)));
        } else if k > 0
            && (v[i - 1] < 0.0) == (v[i] < 0.0)
            && v[i].abs() <= v[i - 1].abs()
            && v[i].abs() <= v[j].abs()
        {
            let low = v[i] > 0.0;
            let (mut a, mut b) = (at(k - 1), at(k + 1));
            let g = (5f64.sqrt() - 1.0) / 2.0;
            for _ in 0..120 {
                let (x1, x2) = (b - g * (b - a), a + g * (b - a));
                if (f(x1) < f(x2)) == low {
                    b = x2;
                } else {
                    a = x1;
                }
            }
            let x = 0.5 * (a + b);
            if (f(x) < 0.0) != (v[i] < 0.0) {
                out.push(bisect(f, at(k - 1), x));
                out.push(bisect(f, x, at(k + 1)));
            }
        }
    }
    out.sort_by(f64::total_cmp);
    out
}

#[derive(Default, Debug)]
struct Census {
    certified: usize,
    miss: usize,
    on: usize,
    refused: usize,
    wrong: Vec<String>,
    max_arc_err_over_eps: f64,
    refused_clear: usize,
    unresolved: usize,
    why: std::collections::BTreeMap<String, usize>,
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Judge one door answer against the oracle; `clear` is the least |res|
/// at the residual's sampled local extrema (how far from tangency).
fn judge(label: &str, e: &geom::Curve3<f64>, k: Cone, cen: &mut Census) {
    let eps = Tol::witness().eps();
    let (t0, t1) = (0.3, 1.3);
    let mid = 0.8;
    let s = k.surface();
    let ans = conic_quadric_roots(e, t0, t1, &s, band());
    let f = |t: f64| k.res(e.eval(t));
    let n = 8_000;
    let truth = sign_changes(&f, mid - PI, mid + PI, n);
    // Least |res| at local extrema, and least distance to the apex.
    let at = |i: u32| mid - PI + TAU * f64::from(i) / f64::from(n);
    let vals: Vec<f64> = (0..=n).map(|i| f(at(i))).collect();
    let mut clear = f64::INFINITY;
    for i in 1..n as usize {
        if vals[i].abs() <= vals[i - 1].abs() && vals[i].abs() <= vals[i + 1].abs() {
            clear = clear.min(vals[i].abs());
        }
    }
    let apex_d = (0..=n)
        .map(|i| (e.eval(at(i)) - k.apex).norm())
        .fold(f64::INFINITY, f64::min);
    let speed = |t: f64| (e.eval(t + 1e-7) - e.eval(t - 1e-7)).norm() / 2e-7;
    match ans {
        Ok(CircleRoots::Certified { count, thetas }) => {
            cen.certified += 1;
            let got = &thetas[..count];
            if count != truth.len() {
                cen.wrong.push(format!(
                    "{label}: count {count} vs oracle {} (clear {clear:e}, apex {apex_d:e}) got {got:?} truth {truth:?}",
                    truth.len()
                ));
                return;
            }
            for &g in got {
                let g = mid + (g - mid + PI).rem_euclid(TAU) - PI;
                let best = truth
                    .iter()
                    .map(|&t| {
                        let d = (g - t).abs();
                        d.min(TAU - d)
                    })
                    .fold(f64::INFINITY, f64::min);
                // The oracle's own resolution: f64 rounding of the
                // residual (~1e-15 of the coordinates' scale) over its
                // slope along the arc there.
                let scale = (e.eval(g) - Point3::origin()).norm().max((k.apex - Point3::origin()).norm()).max(1.0);
                let slope = ((f(g + 1e-6) - f(g - 1e-6)) / 2e-6).abs() / speed(g);
                if 4e-16 * scale / slope > 0.1 * eps {
                    cen.unresolved += 1;
                    continue;
                }
                let arc = best * speed(g);
                let r = arc / eps;
                if r > cen.max_arc_err_over_eps {
                    cen.max_arc_err_over_eps = r;
                }
                if arc > eps {
                    cen.wrong.push(format!(
                        "{label}: root {g} is {arc:e} m along the conic from the oracle's (eps {eps:e}, dist {:e}, apex {apex_d:e})",
                        k.dist(e.eval(g))
                    ));
                }
            }
        }
        Ok(CircleRoots::Miss) => {
            cen.miss += 1;
            if !truth.is_empty() {
                cen.wrong.push(format!(
                    "{label}: Miss but oracle has {} crossings (clear {clear:e})",
                    truth.len()
                ));
            }
        }
        Ok(CircleRoots::OnSurface) => {
            cen.on += 1;
            let worst = (0..=2000)
                .map(|i| k.dist(e.eval(TAU * f64::from(i) / 2000.0)))
                .fold(0.0, f64::max);
            if worst > eps {
                cen.wrong.push(format!(
                    "{label}: OnSurface but the conic strays {worst:e} from the cone (eps {eps:e})"
                ));
            }
        }
        other => {
            let w = match &other {
                Ok(r) => format!("{r:?}"),
                Err(BooleanError::Escalated { decision, diag }) => format!("Escalated {decision:?} {}", format!("{diag:?}").split(',').next().unwrap_or("")),
                Err(e) => format!("{e:?}").chars().take(60).collect(),
            };
            *cen.why.entry(w).or_default() += 1;
            cen.refused += 1;
            if clear > 1e3 * eps && apex_d > 1e3 * eps {
                cen.refused_clear += 1;
            }
        }
    }
}

const ALPHAS: [f64; 7] = [0.002, 0.01, 0.2, 0.785, 1.2, 1.55, 1.568];
const SCALES: [f64; 3] = [1e-3, 1.0, 1e3];

/// A: random conics through random cones, narrow to wide, three scales.
#[test]
fn probe_random_conics() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let mut cen = Census::default();
    for i in 0..3000 {
        let alpha = ALPHAS[i % ALPHAS.len()];
        let sc = SCALES[(i / ALPHAS.len()) % 3];
        let axis = rng.unit();
        let apex = Point3::from_array({let v = Vec3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0)) * sc; [v.x, v.y, v.z]});
        let k = Cone { apex, axis, alpha };
        // A conic whose centre is near the cone at height H on a random nappe.
        let hh = rng.r(0.2, 2.0) * sc * if rng.f() < 0.3 { -1.0 } else { 1.0 };
        let perp = (rng.unit() - axis * rng.unit().dot(axis)).normalize();
        let rad = hh.abs() * alpha.tan().min(1e3);
        let c = apex + axis * hh + perp * (rad * rng.r(0.0, 1.5));
        let a = rng.r(0.1, 2.0) * sc.max(rad.min(10.0 * sc));
        let b = if rng.f() < 0.5 { a } else { a * rng.r(0.2, 1.0) };
        let e = ellipse(c, rng.unit(), rng.unit(), a, b);
        judge(&format!("A{i} α{alpha} s{sc}"), &e, k, &mut cen);
    }
    report("A random", &cen);
}

/// B: (near-)coaxial circles, on, beside and tilted off the cone.
#[test]
fn probe_coaxial() {
    let eps = Tol::witness().eps();
    let mut rng = Rng(0x1234_5678_9abc_def1);
    let mut cen = Census::default();
    let mut i = 0;
    for &alpha in &ALPHAS {
        for &sc in &SCALES {
            for &tilt in &[0.0, 1e-12, 1e-9, 1e-6, 1e-3] {
                for &off in &[0.0, 0.3, -0.3, 3.0, -3.0, 30.0, -30.0, 1e4] {
                    i += 1;
                    let axis = rng.unit();
                    let apex = Point3::from_array({let v = Vec3::new(rng.r(-1.0, 1.0), 0.3, -0.2) * sc; [v.x, v.y, v.z]});
                    let k = Cone { apex, axis, alpha };
                    let hh = sc * rng.r(0.5, 1.5) * if i % 2 == 0 { 1.0 } else { -1.0 };
                    // Radius on the cone, then moved `off` bands radially.
                    let rad = hh.abs() * alpha.tan() + off * eps;
                    if rad <= 0.0 {
                        continue;
                    }
                    let side = axis.orthonormal_basis().0;
                    let n = (axis + side * tilt).normalize();
                    let e = ellipse(apex + axis * hh, n, axis.orthonormal_basis().1, rad, rad);
                    judge(
                        &format!("B α{alpha} s{sc} tilt{tilt:e} off{off}"),
                        &e,
                        k,
                        &mut cen,
                    );
                }
            }
        }
    }
    report("B coaxial", &cen);
}

/// C: circles crossing the cone and passing at distance δ from its apex,
/// in a plane through near the axis: the edge of the apex region.
#[test]
fn probe_apex_approach() {
    let eps = Tol::witness().eps();
    let mut rng = Rng(0xdead_beef_cafe_f00d);
    let mut cen = Census::default();
    let mut first_answer: Vec<(f64, f64, String)> = Vec::new();
    for &alpha in &[0.01, 0.3, 0.785, 1.3, 1.56] {
        for &sc in &SCALES {
            for j in 0..6 {
                let axis = rng.unit();
                let apex = Point3::from_array({let v = Vec3::new(0.1, -0.4, 0.7) * sc; [v.x, v.y, v.z]});
                let k = Cone { apex, axis, alpha };
                // A plane containing a direction tilted from the axis.
                let side = (rng.unit() - axis * 0.0).normalize();
                let n = (side - axis * side.dot(axis)).normalize();
                let inplane = n.cross(axis).normalize();
                let tiltd = (axis * (0.3 + 0.1 * j as f64) + inplane).normalize();
                let r = sc;
                let mut smallest_answered = f64::INFINITY;
                for p in 0..24 {
                    let delta = sc * 10f64.powf(-14.0 + 0.5 * p as f64);
                    // Circle through apex + tiltd*δ·something: centre at
                    // apex + tiltd·(r + δ), in the plane with normal n.
                    let c = apex + tiltd * (r + delta);
                    let e = ellipse(c, n, tiltd, r, r);
                    let before = cen.certified + cen.miss + cen.on;
                    judge(
                        &format!("C α{alpha} s{sc} j{j} δ{delta:e}"),
                        &e,
                        k,
                        &mut cen,
                    );
                    if cen.certified + cen.miss + cen.on > before {
                        smallest_answered = smallest_answered.min(delta / sc);
                    }
                }
                first_answer.push((
                    smallest_answered,
                    smallest_answered * sc / eps,
                    format!("α{alpha} s{sc} j{j}"),
                ));
            }
        }
    }
    for (rel, bands, l) in &first_answer {
        println!("APEX-EDGE {l}: nearest answered δ = {rel:e}·scale = {bands:e} bands");
    }
    report("C apex", &cen);
}

fn report(name: &str, c: &Census) {
    println!(
        "PROBE {name} eps={:e}: certified {} miss {} on {} refused {} (refused-while-clear {}) oracle-unresolved roots {} max arc err/eps {:e} WRONG {}",
        Tol::witness().eps(),
        c.certified,
        c.miss,
        c.on,
        c.refused,
        c.refused_clear,
        c.unresolved,
        c.max_arc_err_over_eps,
        c.wrong.len()
    );
    for (w, n) in &c.why {
        println!("  WHY {n} × {w}");
    }
    for w in c.wrong.iter().take(25) {
        println!("  WRONG {w}");
    }
}

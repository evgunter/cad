//! Review probe for PR #4159: every `Section::Touch` the torus arms
//! answer, over random near-tangent poses, against an independent
//! sampling oracle on the torus (never the kernel's own margins).
//!
//! Oracle, per Touch: `at` lies on both carriers; `at` is an elliptic
//! point of the torus (cos v > 0); and on the touch side the partner's
//! signed level `g` has no other local minimum (refined from a dense
//! grid) at or below `K·ε`, and no point of the torus (refined) deeper
//! than `K·ε` on that side — i.e. the partner does not also cross or
//! touch elsewhere.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::*;
use geom::Surface;
use geom_core::{Band, Point3, Tol, Vec3};

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.f()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
                self.range(-1.0, 1.0),
            );
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v / n;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Tor {
    c: Point3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    a: Vec3<f64>,
    big_r: f64,
    r: f64,
}

impl Tor {
    fn at(&self, u: f64, v: f64) -> Point3<f64> {
        let rho = self.big_r + self.r * v.cos();
        self.c + (self.e1 * u.cos() + self.e2 * u.sin()) * rho + self.a * (self.r * v.sin())
    }
    fn normal(&self, u: f64, v: f64) -> Vec3<f64> {
        (self.e1 * u.cos() + self.e2 * u.sin()) * v.cos() + self.a * v.sin()
    }
    fn uv(&self, x: Point3<f64>) -> (f64, f64) {
        let w = x - self.c;
        let (px, py, z) = (w.dot(self.e1), w.dot(self.e2), w.dot(self.a));
        let u = py.atan2(px);
        let v = z.atan2(px.hypot(py) - self.big_r);
        (u, v)
    }
    fn surface(&self) -> Surface<f64> {
        Surface::Torus {
            center: self.c,
            axis: self.a,
            major_radius: self.big_r,
            minor_radius: self.r,
            u_ref: self.e1,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Partner {
    Plane(Point3<f64>, Vec3<f64>),
    Sphere(Point3<f64>, f64),
    Wall(Point3<f64>, Vec3<f64>, f64),
}

impl Partner {
    fn g(&self, x: Point3<f64>) -> f64 {
        match *self {
            Partner::Plane(p, n) => n.dot(x - p),
            Partner::Sphere(c, rho) => (x - c).norm() - rho,
            Partner::Wall(o, d, rc) => {
                let w = x - o;
                (w - d * w.dot(d)).norm() - rc
            }
        }
    }
    fn surface(&self) -> Surface<f64> {
        match *self {
            Partner::Plane(origin, normal) => Surface::Plane {
                origin,
                normal,
                u_ref: normal.orthonormal_basis().0,
            },
            Partner::Sphere(center, radius) => Surface::Sphere {
                center,
                radius,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Partner::Wall(origin, axis, radius) => Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref: axis.orthonormal_basis().0,
            },
        }
    }
}

/// Pattern search descent of `f(u, v)` from `(u, v)` with step `h`.
fn descend(f: &impl Fn(f64, f64) -> f64, mut u: f64, mut v: f64, mut h: f64) -> (f64, f64, f64) {
    let mut best = f(u, v);
    while h > 1e-15 {
        let mut moved = false;
        for (du, dv) in [
            (h, 0.0),
            (-h, 0.0),
            (0.0, h),
            (0.0, -h),
            (h, h),
            (-h, -h),
            (h, -h),
            (-h, h),
        ] {
            let y = f(u + du, v + dv);
            if y < best {
                best = y;
                u += du;
                v += dv;
                moved = true;
                break;
            }
        }
        if !moved {
            h *= 0.5;
        }
    }
    (u, v, best)
}

#[derive(Debug)]
struct Bad {
    why: String,
}

/// The oracle on one Touch.
fn oracle(t: &Tor, p: &Partner, at: Point3<f64>, kband: f64) -> Result<(), Bad> {
    let scale = t.big_r + t.r;
    let (ua, va) = t.uv(at);
    let on_t = (t.at(ua, va) - at).norm();
    if on_t > kband + 1e-12 * scale || p.g(at).abs() > kband + 1e-12 * scale {
        return Err(Bad {
            why: format!("at off a carrier: torus {on_t:e}, partner {:e}", p.g(at)),
        });
    }
    if va.cos() <= 0.0 {
        return Err(Bad {
            why: format!("at hyperbolic: v = {va}"),
        });
    }
    // The touch side: the side on which `g` near `at` is the extreme.
    // Probe a small step along the torus normal is not enough on its own;
    // take the side whose refined local minimum at `at` is ~0.
    let (nu, nv) = (360usize, 180usize);
    let du = core::f64::consts::TAU / nu as f64;
    let dv = core::f64::consts::TAU / nv as f64;
    let mut ok_side = None;
    let mut reports = Vec::new();
    for s in [1.0, -1.0] {
        let f = |u: f64, v: f64| s * p.g(t.at(u, v));
        let (_, _, m_at) = descend(&f, ua, va, 1e-3);
        if m_at < -kband {
            reports.push(format!("side {s}: refined min near at = {m_at:e}"));
            continue;
        }
        // Other local minima on the grid, refined.
        let grid: Vec<f64> = (0..nu * nv)
            .map(|k| f((k % nu) as f64 * du, (k / nu) as f64 * dv))
            .collect();
        let gi = |i: isize, j: isize| {
            let i = i.rem_euclid(nu as isize) as usize;
            let j = j.rem_euclid(nv as isize) as usize;
            grid[j * nu + i]
        };
        let mut other = None;
        for j in 0..nv as isize {
            for i in 0..nu as isize {
                let y = gi(i, j);
                let is_min = (-1..=1).all(|a| (-1..=1).all(|b| (a == 0 && b == 0) || gi(i + a, j + b) >= y));
                if !is_min {
                    continue;
                }
                let (u, v, m) = descend(&f, i as f64 * du, j as f64 * dv, du);
                let dist = (t.at(u, v) - at).norm();
                if dist > 1e-4 * scale && m <= kband {
                    other = Some(format!(
                        "side {s}: another min {m:e} at {:?} ({dist:e} from at)",
                        t.at(u, v)
                    ));
                    break;
                }
            }
            if other.is_some() {
                break;
            }
        }
        match other {
            Some(o) => reports.push(o),
            None => {
                ok_side = Some(s);
                break;
            }
        }
    }
    match ok_side {
        Some(_) => Ok(()),
        None => Err(Bad {
            why: reports.join("; "),
        }),
    }
}

#[derive(Default, Debug)]
struct Tally {
    touch: usize,
    tangent: usize,
    other: usize,
    bad: Vec<String>,
    touch_names: std::collections::BTreeMap<&'static str, usize>,
}

fn run(t: &Tor, p: &Partner, band: Band, tally: &mut Tally, label: &str) {
    let reach = Reach {
        centre: t.c,
        radius: 2.0 * (t.big_r + t.r),
    };
    let kband = band.escalate();
    for (sec, swapped_) in [
        (classify(&t.surface(), &p.surface(), reach, band), false),
        (classify(&p.surface(), &t.surface(), reach, band), true),
    ] {
        match sec {
            Section::Touch(touch) => {
                tally.touch += 1;
                *tally.touch_names.entry(touch.name).or_default() += 1;
                if let Err(b) = oracle(t, p, touch.at, kband) {
                    tally
                        .bad
                        .push(format!("{label} swapped={swapped_} {}: {} :: {p:?}", touch.name, b.why));
                }
            }
            Section::Tangent(_) => tally.tangent += 1,
            _ => tally.other += 1,
        }
    }
}

fn random_torus(rng: &mut Rng, scale: f64) -> Tor {
    let a = rng.unit();
    let (e1, e2) = a.orthonormal_basis();
    let big_r = rng.range(0.5, 3.0) * scale;
    let k = match (rng.f() * 3.0) as u32 {
        0 => rng.range(0.02, 0.1),
        1 => rng.range(0.85, 0.98),
        _ => rng.range(0.1, 0.85),
    };
    let c = Point3::new(rng.range(-1.0, 1.0), rng.range(-1.0, 1.0), rng.range(-1.0, 1.0)) + Vec3::new(0.0, 0.0, 0.0);
    let c = Point3::origin() + (c - Point3::origin()) * scale;
    Tor {
        c,
        e1,
        e2,
        a,
        big_r,
        r: k * big_r,
    }
}

/// A `v` biased to the boundary cases: the outer equator, the top and
/// bottom parallels (where K changes sign), the inner equator.
fn random_v(rng: &mut Rng, eps: f64) -> f64 {
    use core::f64::consts::FRAC_PI_2;
    let pick = (rng.f() * 6.0) as u32;
    let jitter = [0.0, eps, 10.0 * eps, 1e-6, 1e-4, 1e-2][(rng.f() * 6.0) as usize];
    let sgn = if rng.f() < 0.5 { 1.0 } else { -1.0 };
    match pick {
        0 => sgn * jitter,
        1 => FRAC_PI_2 + sgn * jitter,
        2 => -FRAC_PI_2 + sgn * jitter,
        3 => core::f64::consts::PI + sgn * jitter,
        _ => rng.range(-3.2, 3.2),
    }
}

#[test]
fn probe_torus_touch_soundness() {
    let band = Band::linear(Tol::witness()).unwrap();
    let eps = band.zero();
    let n: usize = std::env::var("PROBE_N").ok().and_then(|s| s.parse().ok()).unwrap_or(300);
    let mut tally = Tally::default();
    let mut rng = Rng(0x4159);
    for scale in [1e-3, 1.0, 1e3] {
        for case in 0..n {
            let t = random_torus(&mut rng, scale);
            let u = rng.range(-3.2, 3.2);
            let v = random_v(&mut rng, 1e-9);
            let x = t.at(u, v);
            let nrm = t.normal(u, v);
            let delta = [0.0, 1.0, -1.0, 10.0, -10.0, 100.0, -100.0, 0.5, -0.5][(rng.f() * 9.0) as usize] * eps;
            let label = format!("scale {scale} case {case} v {v:.12} delta {delta:e}");
            // Plane tangent at x, offset by delta.
            run(&t, &Partner::Plane(x + nrm * delta, nrm), band, &mut tally, &label);
            // Spheres tangent at x: outside (centre along +n), inside the
            // tube or about the torus (centre along -n).
            for (side, rho) in [
                (1.0, rng.range(0.05, 3.0) * scale),
                (-1.0, rng.range(0.05, 0.99) * t.r),
                (-1.0, rng.range(1.0, 4.0) * (t.big_r + t.r)),
            ] {
                let cs = x + nrm * (side * rho);
                run(&t, &Partner::Sphere(cs, rho + delta), band, &mut tally, &label);
            }
            // Walls parallel to the axis tangent on an equator.
            for v_eq in [0.0, core::f64::consts::PI] {
                let x = t.at(u, v_eq);
                let nrm = t.normal(u, v_eq);
                for (side, rc) in [
                    (1.0, rng.range(0.05, 2.0) * scale),
                    (-1.0, rng.range(1.0, 3.0) * (t.big_r + t.r)),
                    (-1.0, rng.range(0.05, 0.99) * (t.big_r - t.r)),
                ] {
                    let o = x + nrm * (side * rc) + t.a * rng.range(-1.0, 1.0) * scale;
                    run(&t, &Partner::Wall(o, t.a, rc + delta), band, &mut tally, &label);
                }
            }
        }
    }
    eprintln!(
        "touch {} tangent {} other {} names {:?}",
        tally.touch, tally.tangent, tally.other, tally.touch_names
    );
    for b in tally.bad.iter().take(40) {
        eprintln!("BAD {b}");
    }
    assert!(tally.bad.is_empty(), "{} bad touches", tally.bad.len());
}

/// The plane touch's `at` near the top parallel: a plane tangent to the
/// donut at `(u, v = π/2 − t)`, `t` small but the elliptic margin
/// `σ·d·s = r·sin t` decided. `at` should lie on both carriers within
/// the margin (`Touch::at`'s docs); measure how far it lies off each.
#[test]
fn probe_plane_touch_at_off_its_carriers_near_the_top_parallel() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut worst: f64 = 0.0;
    for scale in [1.0, 1e3] {
        for (u, axis) in [(0.9, Vec3::unit_z()), (0.9, Vec3::new(1.0, 2.0, 3.0)), (2.3, Vec3::new(-0.3, 0.2, 1.0))] {
            for t in [1e-5, 1e-6, 1e-7, 1e-8] {
                let a = axis / axis.norm();
                let (e1, e2) = a.orthonormal_basis();
                let tor = Tor {
                    c: Point3::origin(),
                    e1,
                    e2,
                    a,
                    big_r: 2.0 * scale,
                    r: 0.5 * scale,
                };
                let v = core::f64::consts::FRAC_PI_2 - t;
                let (x, n) = (tor.at(u, v), tor.normal(u, v));
                let pl = Partner::Plane(x, n);
                let reach = Reach { centre: tor.c, radius: 5.0 * scale };
                match classify(&tor.surface(), &pl.surface(), reach, band) {
                    Section::Touch(tt) => {
                        let (ua, va) = tor.uv(tt.at);
                        let off_t = (tor.at(ua, va) - tt.at).norm();
                        let off_p = pl.g(tt.at).abs();
                        let from_true = (tt.at - x).norm();
                        worst = worst.max(off_t.max(off_p) / band.zero());
                        eprintln!(
                            "scale {scale:e} axis {axis:?} u {u} t {t:e}: at off torus {off_t:.2e}, off plane {off_p:.2e}, from the true touch {from_true:.2e} (ε = {:e})",
                            band.zero()
                        );
                    }
                    other => eprintln!("scale {scale:e} u {u} t {t:e}: {other:?}"),
                }
            }
        }
    }
    eprintln!("worst off-carrier distance: {worst:.1} ε");
    assert!(worst <= 1.0, "Touch::at stands {worst:.1} ε off a carrier");
}

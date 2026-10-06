//! Reviewer probe (reach-dual4135-r1): the conic × cone door against an
//! independent oracle. Mounted temporarily as a `#[cfg(test)]` child of
//! `topo::boolean::conic_quadric` via `#[path]`; never committed into the
//! crate. Oracle: the cone's own residual `ρ cos α − |h| sin α` (the
//! double cone) evaluated from the point, dense sampling + bisection; a
//! point's distance from the double cone read in its meridian half-plane.
#![allow(clippy::unwrap_used, clippy::panic, clippy::cast_precision_loss)]
use super::*;
use core::f64::consts::{PI, TAU};
use geom_core::{Point3, Tol, Vec3};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
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
    a: f64,
}
impl Cone {
    fn surface(self) -> geom::Surface<f64> {
        geom::Surface::Cone {
            apex: self.apex,
            axis: self.axis,
            half_angle: self.a,
            u_ref: self.axis.orthonormal_basis().0,
        }
    }
    /// Signed elevation off the double cone (exact distance for every
    /// point whose foot is on a ray; the residual's sign everywhere).
    fn res(self, p: Point3<f64>) -> f64 {
        let q = p - self.apex;
        let h = q.dot(self.axis);
        let rho = (q - self.axis * h).norm();
        rho * self.a.cos() - h.abs() * self.a.sin()
    }
    fn dist(self, p: Point3<f64>) -> f64 {
        let q = p - self.apex;
        let h = q.dot(self.axis);
        let rho = (q - self.axis * h).norm();
        let (sa, ca) = self.a.sin_cos();
        [ca, -ca]
            .into_iter()
            .map(|c| {
                if rho * sa + h * c <= 0.0 {
                    rho.hypot(h)
                } else {
                    (rho * c - h * sa).abs()
                }
            })
            .fold(f64::INFINITY, f64::min)
    }
}

fn eval(c: &geom::Curve3<f64>, t: f64) -> Point3<f64> {
    c.eval(t)
}

/// Sign changes of `f` round the whole turn about `mid`, bisected.
fn changes(f: &dyn Fn(f64) -> f64, mid: f64, n: usize) -> Vec<f64> {
    let at = |k: usize| mid - PI + TAU * k as f64 / n as f64;
    let mut out = Vec::new();
    for k in 0..n {
        let (mut a, mut b) = (at(k), at(k + 1));
        let (fa, fb) = (f(a), f(b));
        if (fa < 0.0) == (fb < 0.0) {
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

#[derive(Default, Debug)]
struct Tally {
    certified: usize,
    miss: usize,
    on: usize,
    apex: usize,
    refused: usize,
    wrong: Vec<String>,
    worst_root_over_eps: f64,
    worst_miss_clear_over_eps: f64,
}

fn check(label: &str, e: &geom::Curve3<f64>, k: Cone, band: Band, eps: f64, t: &mut Tally) {
    let s = k.surface();
    let mid = 0.3;
    let got = conic_quadric_roots(e, mid - 0.5, mid + 0.5, &s, band);
    let f = |th: f64| k.res(eval(e, th));
    let speed = match *e {
        geom::Curve3::Circle { radius, .. } => radius,
        geom::Curve3::Ellipse { major, minor, .. } => major.abs().max(minor.abs()),
        _ => unreachable!(),
    };
    let truth = changes(&f, mid, 40_000);
    match got {
        Ok(CircleRoots::Certified { count, thetas }) => {
            t.certified += 1;
            let mut roots = thetas[..count].to_vec();
            roots.sort_by(f64::total_cmp);
            // Refine the oracle around each kernel root for a pair the
            // grid could have stepped over.
            let mut truth = truth.clone();
            if truth.len() != count {
                let fine = changes(&f, mid, 4_000_000);
                truth = fine;
            }
            if truth.len() != count {
                t.wrong.push(format!(
                    "{label}: certified {count} roots {roots:?}, oracle {} {truth:?}",
                    truth.len()
                ));
                return;
            }
            for &r in &roots {
                let off = k.dist(eval(e, r));
                let near = truth
                    .iter()
                    .map(|&o| ((r - o + PI).rem_euclid(TAU) - PI).abs() * speed)
                    .fold(f64::INFINITY, f64::min);
                let worst = (off.max(0.0)).max(near) / eps;
                t.worst_root_over_eps = t.worst_root_over_eps.max(worst);
                if near > eps || off > eps {
                    t.wrong.push(format!(
                        "{label}: root {r} is {near:e} m of arc from the oracle's, {off:e} off the cone (eps {eps:e})"
                    ));
                }
                if (eval(e, r) - k.apex).norm() <= eps {
                    t.wrong.push(format!("{label}: certified root within eps of the apex"));
                }
            }
        }
        Ok(CircleRoots::Miss) => {
            t.miss += 1;
            let fine = if truth.is_empty() { changes(&f, mid, 4_000_000) } else { truth };
            let least = (0..50_000)
                .map(|i| k.dist(eval(e, mid - PI + TAU * i as f64 / 50_000.0)))
                .fold(f64::INFINITY, f64::min);
            t.worst_miss_clear_over_eps = if t.miss == 1 {
                least / eps
            } else {
                t.worst_miss_clear_over_eps.min(least / eps)
            };
            if !fine.is_empty() {
                t.wrong.push(format!("{label}: Miss but the oracle crosses {} times", fine.len()));
            }
        }
        Ok(CircleRoots::OnSurface) => {
            t.on += 1;
            let worst = (0..20_000)
                .map(|i| k.dist(eval(e, TAU * i as f64 / 20_000.0)))
                .fold(0.0, f64::max);
            if worst > eps {
                t.wrong.push(format!("{label}: OnSurface but the carrier leaves the cone by {worst:e} (eps {eps:e})"));
            }
        }
        Ok(CircleRoots::AtApex) => t.apex += 1,
        Ok(_) | Err(_) => t.refused += 1,
    }
}

fn circle(c: Point3<f64>, n: Vec3<f64>, r: f64) -> geom::Curve3<f64> {
    geom::Curve3::Circle { center: c, axis: n, radius: r, u_ref: n.orthonormal_basis().0 }
}
fn ellipse(c: Point3<f64>, n: Vec3<f64>, a: f64, b: f64, spin: f64) -> geom::Curve3<f64> {
    let (u0, v0) = n.orthonormal_basis();
    let u = u0 * spin.cos() + v0 * spin.sin();
    geom::Curve3::Ellipse { center: c, axis: n, major: a, minor: b, u_ref: u }
}

/// A random cone, conic and pose class; `class` 0 generic crossing,
/// 1 coaxial / near-coaxial circles, 2 near-tangent (moved off by k·ε),
/// 3 near the apex, 4 plane sections (ellipses lying on the cone).
fn draw(rng: &mut Rng, class: u32, scale: f64, eps: f64) -> (geom::Curve3<f64>, Cone, String) {
    let alphas = [0.003, 0.01, 0.05, 0.3, 0.785, 1.2, 1.5, 1.56, 1.567];
    let a = alphas[(rng.next() * alphas.len() as f64) as usize % alphas.len()];
    let axis = rng.unit();
    let apex = Point3::new(rng.range(-1., 1.) * scale, rng.range(-1., 1.) * scale, rng.range(-1., 1.) * scale);
    let k = Cone { apex, axis, a };
    let (p, _) = axis.orthonormal_basis();
    let nappe = if rng.next() < 0.5 { 1.0 } else { -1.0 };
    // A point on the cone at slant distance `s` along a random generator.
    let on_cone = |rng: &mut Rng, s: f64| {
        let phi = rng.range(0., TAU);
        let q = axis.cross(p);
        let radial = p * phi.cos() + q * phi.sin();
        apex + (axis * (nappe * a.cos()) + radial * a.sin()) * s
    };
    let label;
    let curve = match class {
        0 => {
            let sl = rng.range(0.3, 2.0) * scale;
            let c = on_cone(rng, sl);
            let n = rng.unit();
            let r = rng.range(0.05, 1.0) * scale;
            label = format!("generic a={a} scale={scale}");
            if rng.next() < 0.5 {
                circle(c, n, r)
            } else {
                ellipse(c, n, r, r * rng.range(0.2, 0.95), rng.range(0., TAU))
            }
        }
        1 => {
            let h = nappe * rng.range(0.2, 2.0) * scale;
            let tilt = [0.0, 1e-9, 1e-6, 1e-3][(rng.next() * 4.0) as usize % 4];
            let n = (axis + p * tilt).normalize();
            let rr = h.abs() * a.tan() * rng.range(0.5, 1.5);
            let rr = if rng.next() < 0.3 { h.abs() * a.tan() } else { rr };
            label = format!("coaxial a={a} tilt={tilt} scale={scale}");
            circle(apex + axis * h + p * (rng.range(-1., 1.) * tilt * scale), n, rr)
        }
        2 => {
            let sl = rng.range(0.3, 2.0) * scale;
            let c = on_cone(rng, sl);
            let n = rng.unit();
            let r = rng.range(0.05, 1.0) * scale;
            let mut e = if rng.next() < 0.5 {
                circle(c, n, r)
            } else {
                ellipse(c, n, r, r * rng.range(0.2, 0.95), rng.range(0., TAU))
            };
            // Find the turn's least |residual| point and move the conic
            // along the cone's normal there to a chosen offset.
            let off = [-10.0, -3.0, -1.0, -0.3, 0.0, 0.3, 1.0, 3.0, 10.0, 100.0]
                [(rng.next() * 10.0) as usize % 10]
                * eps;
            for _ in 0..3 {
                let (th, _) = (0..20_000)
                    .map(|i| TAU * i as f64 / 20_000.0)
                    .map(|th| (th, k.res(e.eval(th))))
                    .min_by(|x, y| x.1.total_cmp(&y.1))
                    .unwrap();
                // refine by golden search on |res| near th
                let (mut lo, mut hi) = (th - TAU / 20_000.0, th + TAU / 20_000.0);
                for _ in 0..200 {
                    let m1 = lo + (hi - lo) / 3.0;
                    let m2 = hi - (hi - lo) / 3.0;
                    if k.res(e.eval(m1)) < k.res(e.eval(m2)) {
                        hi = m2;
                    } else {
                        lo = m1;
                    }
                }
                let th = 0.5 * (lo + hi);
                let pt = e.eval(th);
                let q = pt - apex;
                let h = q.dot(axis);
                let w = q - axis * h;
                let radial = if w.norm() > 0.0 { w / w.norm() } else { p };
                let normal = radial * a.cos() - axis * (h.signum() * a.sin());
                let shift = normal * (off - k.res(pt));
                e = match e {
                    geom::Curve3::Circle { center, axis, radius, u_ref } => {
                        geom::Curve3::Circle { center: center + shift, axis, radius, u_ref }
                    }
                    geom::Curve3::Ellipse { center, axis, major, minor, u_ref } => {
                        geom::Curve3::Ellipse { center: center + shift, axis, major, minor, u_ref }
                    }
                    _ => unreachable!(),
                };
            }
            label = format!("graze off={:e} a={a} scale={scale}", off);
            e
        }
        3 => {
            // A circle through the apex region, passing at `d` from it.
            let d = [0.0, 0.5, 2.0, 5.0, 30.0, 1e3][(rng.next() * 6.0) as usize % 6] * eps;
            let n = rng.unit();
            let r = rng.range(0.05, 1.0) * scale;
            let (u, _) = n.orthonormal_basis();
            let c = apex + u * (r + d);
            label = format!("apex d={:e} a={a} scale={scale}", d);
            circle(c, n, r)
        }
        _ => {
            // A plane section ellipse: plane through the axis point at
            // height h with normal tilted by beta < pi/2 - a.
            let h = nappe * rng.range(0.3, 2.0) * scale;
            let beta = rng.range(0.0, (PI / 2.0 - a) * 0.9);
            let q = axis.cross(p);
            let n = axis * beta.cos() + p * beta.sin();
            // The section of the cone by the plane through apex+axis*h
            // with normal n: compute it numerically from its two
            // vertices along the in-plane direction m (in span(axis,p)).
            let m = p * beta.cos() - axis * beta.sin();
            let base = apex + axis * h;
            // points base + m*t on the cone: solve res=0 on the line by bisection
            let line = |t: f64| base + m * t;
            let root = |lo: f64, hi: f64| {
                let (mut lo, mut hi) = (lo, hi);
                let flo = k.res(line(lo));
                for _ in 0..200 {
                    let mm = 0.5 * (lo + hi);
                    if (k.res(line(mm)) < 0.0) == (flo < 0.0) {
                        lo = mm;
                    } else {
                        hi = mm;
                    }
                }
                0.5 * (lo + hi)
            };
            let big = 1e3 * h.abs() / (PI / 2.0 - a - beta).max(1e-3);
            let t1 = root(0.0, big);
            let t2 = root(0.0, -big);
            let center = line(0.5 * (t1 + t2));
            let major = 0.5 * (t1 - t2);
            // minor: the half-chord along q at the center.
            let lq = |t: f64| center + q * t;
            let (mut lo, mut hi) = (0.0, 2.0 * major);
            let flo = k.res(lq(lo));
            for _ in 0..200 {
                let mm = 0.5 * (lo + hi);
                if (k.res(lq(mm)) < 0.0) == (flo < 0.0) {
                    lo = mm;
                } else {
                    hi = mm;
                }
            }
            let minor = 0.5 * (lo + hi);
            label = format!("section beta={beta} a={a} scale={scale}");
            geom::Curve3::Ellipse { center, axis: n, major, minor, u_ref: m }
        }
    };
    (curve, k, label)
}

#[test]
fn r1_conic_cone_door_against_the_oracle() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let eps = band.zero();
    let seed = 0x9e37_79b9_7f4a_7c15_u64 ^ (eps.to_bits());
    let mut rng = Rng(seed);
    let only: Option<u32> = std::env::var("R1_CLASS").ok().and_then(|v| v.parse().ok());
    for class in 0..6 {
        if only.is_some_and(|c| c != class) {
            continue;
        }
        for scale in [1e-3, 1.0, 1e3] {
            let mut t = Tally::default();
            if class == 5 {
                // The apex region's edge: circles passing at d = 10^k eps.
                for kx in 0..10 {
                    let d = 10f64.powi(kx) * eps;
                    let mut t = Tally::default();
                    for i in 0..60 {
                        let (e0, k, _) = draw(&mut rng, 3, scale, eps);
                        let geom::Curve3::Circle { axis: n, radius: r, .. } = e0 else { unreachable!() };
                        let (u, _) = n.orthonormal_basis();
                        let e = circle(k.apex + u * (r + d), n, r);
                        check(&format!("#{i} apex d={d:e} a={} scale={scale}", k.a), &e, k, band, eps, &mut t);
                    }
                    eprintln!("R1APEX eps={eps:e} scale={scale:e} d/eps=1e{kx}: cert {} miss {} refused {} atapex {} WRONG {}", t.certified, t.miss, t.refused, t.apex, t.wrong.len());
                    for w in t.wrong.iter().take(3) { eprintln!("R1WRONG {w}"); }
                }
                continue;
            }
            for i in 0..200 {
                let (e, k, label) = draw(&mut rng, class, scale, eps);
                check(&format!("#{i} {label}"), &e, k, band, eps, &mut t);
            }
            eprintln!(
                "R1PROBE eps={eps:e} class={class} scale={scale:e}: cert {} miss {} on {} apex {} refused {} | worst root/eps {:.3e} least miss clearance/eps {:.3e} | WRONG {}",
                t.certified, t.miss, t.on, t.apex, t.refused, t.worst_root_over_eps, t.worst_miss_clear_over_eps, t.wrong.len()
            );
            for w in t.wrong.iter().take(6) {
                eprintln!("R1WRONG {w}");
            }
        }
    }
}

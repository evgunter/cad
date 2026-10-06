//! Reviewer probe for PR 4128 (not a kernel row). Mounted into
//! `crates/topo/src/boolean/carrier_touch.rs` by
//! `#[cfg(test)] #[path = "../../../../probes/carrier_touch_localization.rs"] mod review_probe;`
//! It checks `clusters` against an INDEPENDENT oracle: the curve is
//! sampled densely, its signed distance from the carrier taken in closed
//! form here (not the kernel's `distance`), and every sample within the
//! band of the carrier (or a sign change) must fall in a returned cluster.
#![allow(clippy::unwrap_used, clippy::panic, clippy::print_stdout)]

use super::*;
use geom_core::Tol;

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn s(&mut self) -> f64 {
        2.0 * self.f() - 1.0
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.s(), self.s(), self.s());
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
    fn perp(&mut self, n: Vec3<f64>) -> Vec3<f64> {
        let v = self.unit();
        let p = v - n * v.dot(n);
        p / p.norm()
    }
}

/// Oracle signed distance, written independently of the kernel.
fn oracle(s: &geom::Surface<f64>, q: Point3<f64>) -> f64 {
    match *s {
        geom::Surface::Sphere { center, radius, .. } => (q - center).norm() - radius,
        geom::Surface::Cylinder { origin, axis, radius, .. } => {
            let a = axis / axis.norm();
            let w = q - origin;
            (w - a * w.dot(a)).norm() - radius
        }
        geom::Surface::Torus { center, axis, major_radius, minor_radius, .. } => {
            let a = axis / axis.norm();
            let w = q - center;
            let z = w.dot(a);
            let rho = (w - a * z).norm();
            ((rho - major_radius).powi(2) + z * z).sqrt() - minor_radius
        }
        _ => unreachable!(),
    }
}

/// A surface and a point on it with its outward normal.
fn surface(rng: &mut Rng, kind: usize, scale: f64) -> (geom::Surface<f64>, Point3<f64>, Vec3<f64>) {
    let c = Point3::new(rng.s() * scale, rng.s() * scale, rng.s() * scale);
    let axis = rng.unit();
    let u_ref = rng.perp(axis);
    let v_ref = axis.cross(u_ref);
    match kind {
        0 => {
            let r = (0.3 + rng.f()) * scale;
            let n = rng.unit();
            (geom::Surface::Sphere { center: c, radius: r, axis, u_ref }, c + n * r, n)
        }
        1 => {
            let r = (0.3 + rng.f()) * scale;
            let th = rng.f() * 6.283;
            let n = u_ref * th.cos() + v_ref * th.sin();
            let h = rng.s() * scale;
            (geom::Surface::Cylinder { origin: c, axis, radius: r, u_ref }, c + axis * h + n * r, n)
        }
        _ => {
            let big = (1.0 + rng.f()) * scale;
            let small = (0.1 + 0.8 * rng.f()) * big;
            let (u, v) = (rng.f() * 6.283, rng.f() * 6.283);
            let radial = u_ref * u.cos() + v_ref * u.sin();
            let n = radial * v.cos() + axis * v.sin();
            let p = c + radial * big + n * small;
            (
                geom::Surface::Torus { center: c, axis, major_radius: big, minor_radius: small, u_ref },
                p,
                n,
            )
        }
    }
}

/// A curve through `p` tangent to `tangent` at parameter `t*`, curving
/// toward `bend_dir` (perpendicular to `tangent`); returns curve and t*.
fn curve(rng: &mut Rng, kind: usize, p: Point3<f64>, tangent: Vec3<f64>, bend_dir: Vec3<f64>, scale: f64) -> (geom::Curve3<f64>, f64) {
    match kind {
        0 => {
            let speed = (0.5 + rng.f()) * scale;
            let ts = rng.s();
            (geom::Curve3::Line { origin: p - tangent * (speed * ts), dir: tangent * speed }, ts)
        }
        1 => {
            let r = (0.2 + 3.0 * rng.f()) * scale;
            let center = p + bend_dir * r;
            let u = (p - center) / r;
            let axis = u.cross(tangent);
            (geom::Curve3::Circle { center, axis, radius: r, u_ref: u }, 0.0)
        }
        _ => {
            let a = (0.3 + 2.0 * rng.f()) * scale;
            let b = a * (0.1 + 0.85 * rng.f());
            // At t = 0 the point is centre + u·a, tangent ∝ v.
            let u = -bend_dir;
            let center = p - u * a;
            let axis = u.cross(tangent);
            (geom::Curve3::Ellipse { center, axis, major: a, minor: b, u_ref: u }, 0.0)
        }
    }
}

#[test]
fn clusters_hold_every_band_meeting() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let (mut cases, mut misses, mut nones) = (0, 0, 0);
    for scale in [1e-3, 1.0, 1e3] {
        for sk in 0..3 {
            for ck in 0..3 {
                for _ in 0..60 {
                    let (s, p, n) = surface(&mut rng, sk, scale);
                    let delta = [-1e-6, -1e-8, -2e-9, 0.0, 2e-9, 1e-8, 1e-6][(rng.f() * 7.0) as usize] * scale;
                    let tangent = rng.perp(n);
                    let side = n.cross(tangent);
                    let w = rng.s();
                    let bend = n * w + side * (1.0 - w * w).sqrt();
                    let (c, ts) = curve(&mut rng, ck, p + n * delta, tangent, bend, scale);
                    let half = 0.2 + rng.f();
                    let (t0, t1) = (ts - half, ts + half * (0.3 + rng.f()));
                    let Some(kappa) = reach(&s) else { continue };
                    let Some(sb) = speed_bound(&c) else { continue };
                    cases += 1;
                    let Some(cl) = clusters(&s, &c, sb, kappa, (t0, t1), band) else {
                        nones += 1;
                        continue;
                    };
                    const N: usize = 200_000;
                    let mut prev = oracle(&s, c.eval(t0));
                    for i in 1..=N {
                        let t = t0 + (t1 - t0) * i as f64 / N as f64;
                        let d = oracle(&s, c.eval(t));
                        if d.abs() <= band.zero() || d.signum() != prev.signum() {
                            let tl = t - (t1 - t0) / N as f64;
                            let held = cl.iter().any(|&(a, b)| a <= t && tl <= b);
                            if !held {
                                misses += 1;
                                println!(
                                    "MISS sk={sk} ck={ck} scale={scale} delta={delta:e} t={t} d={d:e} clusters={cl:?} curve={c:?} surf={s:?}"
                                );
                                break;
                            }
                        }
                        prev = d;
                    }
                }
            }
        }
    }
    println!("cases {cases}, budget-exhausted {nones}, misses {misses}");
    assert_eq!(misses, 0);
}

/// An ellipse stored minor > major (STEP's ELLIPSE carries no order;
/// `geom::Curve3::Ellipse`'s doc: "readers past the constructor take the
/// semi-axes as magnitudes in either order"). The curve truly crosses the
/// sphere twice; a sound localization must keep both crossings.
#[test]
fn a_swapped_ellipse_keeps_its_crossings() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut misses = 0;
    for (i, &(major, minor)) in [(0.1, 1.0), (0.05, 2.0), (0.3, 1.0)].iter().enumerate() {
        for k in 0..20 {
            let tc = 0.3 + 0.12 * k as f64;
            let c = geom::Curve3::Ellipse {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major,
                minor,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let p = c.eval(tc);
            // A small sphere centred just off the curve: crossed twice.
            let s = geom::Surface::Sphere {
                center: p + Vec3::new(0.0, 0.0, 0.01),
                radius: 0.02,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let (t0, t1) = (0.0, 3.0);
            let cl = clusters(&s, &c, speed_bound(&c).unwrap(), reach(&s).unwrap(), (t0, t1), band);
            let Some(cl) = cl else { continue };
            // The two crossings straddle tc: a cluster on each side.
            let held = cl.iter().any(|&(_, b)| b <= tc) && cl.iter().any(|&(a, _)| a >= tc);
            if !held {
                misses += 1;
                println!("SWAPPED MISS case {i} tc={tc}: clusters {cl:?}");
            }
        }
    }
    println!("swapped misses {misses}");
    assert_eq!(misses, 0);
}

/// A line over a torus's axis, tangent to its top at two points: the
/// first piece's middle lies ON the axis, where the closed-form foot
/// divides by zero.
#[test]
fn a_midpoint_on_the_torus_axis() {
    let band = Band::linear(Tol::witness()).unwrap();
    let s = geom::Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let c = geom::Curve3::Line { origin: Point3::new(0.0, 0.0, 0.5), dir: Vec3::new(1.0, 0.0, 0.0) };
    let cl = clusters(&s, &c, speed_bound(&c).unwrap(), reach(&s).unwrap(), (-3.0, 3.0), band);
    println!("torus axis clusters: {cl:?}");
    let cl = cl.expect("localizes");
    assert!(cl.iter().any(|&(a, b)| a <= 2.0 && 2.0 <= b));
    assert!(cl.iter().any(|&(a, b)| a <= -2.0 && -2.0 <= b));
}

/// The control for the swapped probe: the SAME geometric ellipse stored
/// major > minor (u_ref turned a quarter, so t shifts by π/2).
#[test]
fn the_same_ellipse_ordered_keeps_its_crossings() {
    let band = Band::linear(Tol::witness()).unwrap();
    let mut misses = 0;
    for &(major, minor) in &[(0.1, 1.0), (0.05, 2.0), (0.3, 1.0)] {
        for k in 0..20 {
            let tc = 0.3 + 0.12 * k as f64;
            let swapped = geom::Curve3::Ellipse {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major,
                minor,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let p = swapped.eval(tc);
            let c = geom::Curve3::Ellipse {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major: minor,
                minor: major,
                u_ref: Vec3::new(0.0, 1.0, 0.0),
            };
            let tq = tc - core::f64::consts::FRAC_PI_2;
            assert!((c.eval(tq) - p).norm() < 1e-12);
            let s = geom::Surface::Sphere {
                center: p + Vec3::new(0.0, 0.0, 0.01),
                radius: 0.02,
                axis: Vec3::new(0.0, 0.0, 1.0),
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let cl = clusters(&s, &c, speed_bound(&c).unwrap(), reach(&s).unwrap(), (-1.6, 1.4), band);
            let Some(cl) = cl else { continue };
            if !(cl.iter().any(|&(_, b)| b <= tq) && cl.iter().any(|&(a, _)| a >= tq)) {
                misses += 1;
                println!("ORDERED MISS tc={tc}: {cl:?}");
            }
        }
    }
    println!("ordered misses {misses}");
    assert_eq!(misses, 0);
}

/// The ball `off_face` reads for the item's touch (a line tangent to the
/// unit sphere), against the radius the near-miss rows are placed by.
#[test]
fn the_touch_ball_against_the_rows_radius() {
    for eps in [1e-9, 1e-6, 1e-12] {
        let band = Band::linear_at(Tol::witness(), eps).unwrap();
        let s = geom::Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let c = geom::Curve3::Line { origin: Point3::new(0.0, -1.0, 0.0), dir: Vec3::new(1.0, 0.0, 0.0) };
        let (speed, bend) = speed_bound(&c).unwrap();
        let cl = clusters(&s, &c, (speed, bend), 1.0, (-0.25, 0.25), band).unwrap();
        let rows = (2.0 * (band.zero() + band.escalate())).sqrt();
        for (a, b) in cl {
            let m = c.eval((a + b) * 0.5);
            let (d, _, _) = distance(&s, m).unwrap();
            let radius = speed * (b - a).abs() * 0.5 + d.abs() + band.escalate();
            println!("BALL eps {eps:e}: kernel radius {radius:e}, rows' radius {rows:e}, ratio {:.2}", radius / rows);
        }
    }
}

// Reviewer probe (reach-dual3805-r1): appended locally to crates/topo/src/boolean/ellipse_roots.rs.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod r1_probe {
    use super::*;
    use core::f64::consts::{PI, TAU};
    use geom_core::{Point3, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }
    fn ellipse(c: Vec3<f64>, n: Vec3<f64>, u: Vec3<f64>, a: f64, b: f64) -> geom::Curve3<f64> {
        let n = n.normalize();
        geom::Curve3::Ellipse {
            center: Point3::new(c.x, c.y, c.z),
            axis: n,
            major: a,
            minor: b,
            u_ref: (u - n * u.dot(n)).normalize(),
        }
    }
    fn dist(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
        match *s {
            geom::Surface::Sphere { center, radius, .. } => (p - center).norm() - radius,
            geom::Surface::Cylinder { origin, axis, radius, .. } => {
                let w = p - origin;
                (w - axis * w.dot(axis)).norm() - radius
            }
            _ => unreachable!(),
        }
    }
    fn oracle(e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64, steps: u32) -> (Vec<f64>, f64) {
        let f = |t: f64| dist(s, e.eval(t));
        let mut out = Vec::new();
        let mut minabs = f64::INFINITY;
        for k in 0..steps {
            let at = |k: u32| t0 + (t1 - t0) * f64::from(k) / f64::from(steps);
            let (mut a, mut b) = (at(k), at(k + 1));
            minabs = minabs.min(f(a).abs());
            if f(a).signum() == f(b).signum() {
                continue;
            }
            for _ in 0..80 {
                let m = (a + b) / 2.0;
                if f(m).signum() == f(a).signum() { a = m } else { b = m }
            }
            out.push((a + b) / 2.0);
        }
        (out, minabs)
    }
    struct Rng(u64);
    impl Rng {
        fn f(&mut self) -> f64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            (self.0 >> 11) as f64 / (1u64 << 53) as f64
        }
        fn v(&mut self, s: f64) -> Vec3<f64> {
            Vec3::new(self.f() * 2.0 - 1.0, self.f() * 2.0 - 1.0, self.f() * 2.0 - 1.0) * s
        }
    }

    fn judge(label: &str, e: &geom::Curve3<f64>, s: &geom::Surface<f64>, t0: f64, t1: f64, stats: &mut [usize; 6]) {
        let mid = (t0 + t1) / 2.0;
        let r = ellipse_roots(e, t0, t1, s, band());
        let (truth, minabs) = oracle(e, s, mid - PI, mid + PI, 40_000);
        match r {
            Ok(CircleRoots::Certified { count, thetas }) => {
                stats[0] += 1;
                let mut got = thetas[..count].to_vec();
                got.sort_by(f64::total_cmp);
                if got.len() != truth.len() {
                    let (fine, _) = oracle(e, s, mid - PI, mid + PI, 2_000_000);
                    if got.len() != fine.len() { eprintln!("BAD {label}: COUNT {got:?} vs {fine:?}"); stats[5] += 1; }
                    return;
                }
                for (a, b) in got.iter().zip(&truth) {
                    if (a - b).abs() >= 1e-8 { let sp = |t: f64| { let h = 1e-6; (e.eval(t + h) - e.eval(t - h)).norm() / (2.0 * h) }; eprintln!("BAD {label}: PLACE {a} vs {b}: arc error {:e} m, |d| at got {:e}", (a - b).abs() * sp(*b), dist(s, e.eval(*a)).abs()); stats[5] += 1; }
                }
            }
            Ok(CircleRoots::Miss) => {
                stats[1] += 1;
                if !(truth.is_empty() && minabs > 0.0) { eprintln!("BAD {label}: MISS but oracle {} crossings, min|d| {minabs:e}", truth.len()); stats[5] += 1; }
            }
            Ok(CircleRoots::OnSurface) => {
                stats[2] += 1;
                assert!(truth.is_empty() || minabs < 1e-8, "{label}: ONSURFACE but crossings {truth:?} min {minabs}");
                let (_, _) = (0, 0);
                let maxabs = (0..1000).map(|k| dist(s, e.eval(TAU * f64::from(k) / 1000.0)).abs()).fold(0.0, f64::max);
                assert!(maxabs < 1e-8, "{label}: ONSURFACE but off by {maxabs}");
            }
            Ok(_) => stats[3] += 1,
            Err(BooleanError::Escalated { .. }) => stats[4] += 1,
            Err(e) => panic!("{label}: {e:?}"),
        }
    }

    #[test]
    fn fuzz_random_ellipses() {
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        let mut stats = [0usize; 6];
        for i in 0..6000 {
            let b = 0.05 + rng.f();
            let ecc = [1.0001, 1.5, 4.0, 20.0][i % 4];
            let a = b * ecc;
            let scale = [1e-3, 1.0, 1e3][(i / 4) % 3];
            let e = ellipse(rng.v(0.5 * scale), rng.v(1.0), rng.v(1.0), a * scale, b * scale);
            let r = (0.2 + 1.5 * rng.f()) * scale;
            let c = rng.v(1.0 * scale);
            let s = if i % 2 == 0 {
                geom::Surface::Sphere { center: Point3::new(c.x, c.y, c.z), radius: r, axis: Vec3::new(0.0, 0.0, 1.0), u_ref: Vec3::new(1.0, 0.0, 0.0) }
            } else {
                let ax = rng.v(1.0).normalize();
                let x = Vec3::new(1.0, 0.0, 0.0);
                let u = (x - ax * x.dot(ax)).normalize();
                geom::Surface::Cylinder { origin: Point3::new(c.x, c.y, c.z), axis: ax, radius: r, u_ref: u }
            };
            let t0 = rng.f() * 12.0 - 6.0;
            let t1 = t0 + 0.1 + rng.f() * 6.0;
            if i == 4815 || i == 288 { eprintln!("DUMP case {i}: {e:?} {s:?} t0 {t0} t1 {t1}"); }
            judge(&format!("case {i}"), &e, &s, t0, t1, &mut stats);
        }
        eprintln!("bad {} certified {} miss {} on {} uncertain {} escalated {}", stats[5], stats[0], stats[1], stats[2], stats[3], stats[4]);
    }

    /// The first-harmonic arm: a wall's tilted section, re-centred off the
    /// axis (so its projection is a circle that crosses the wall), with the
    /// major axis perturbed by `δ` (A₂ ∝ δ, into the band, the gap, past it).
    #[test]
    fn first_harmonic_arm_against_bisection() {
        let mut stats = [0usize; 6];
        for phi in [0.3_f64, 1.2] {
            for off in [0.0, 0.3, 0.49999, 0.5, 0.7, 0.99, 1.0, 1.01] {
                for delta in [0.0, 1e-13, 1e-11, 1e-10, 1e-9, 1e-8, 1e-6, 1e-3] {
                    for rad in [0.5, 5e-4, 500.0] {
                        let k = rad / 0.5;
                        let e = ellipse(
                            Vec3::new(off * k, 0.0, 0.5),
                            Vec3::new(phi.sin(), 0.0, phi.cos()),
                            Vec3::new(1.0, 0.0, 0.0),
                            rad / phi.cos() + delta * k,
                            rad,
                        );
                        let w = geom::Surface::Cylinder { origin: Point3::new(0.0, 0.0, 0.0), axis: Vec3::new(0.0, 0.0, 1.0), radius: rad, u_ref: Vec3::new(1.0, 0.0, 0.0) };
                        for t0 in [0.0, 2.0] {
                            judge(&format!("phi {phi} off {off} delta {delta} rad {rad} t0 {t0}"), &e, &w, t0, t0 + 1.5, &mut stats);
                        }
                    }
                }
            }
        }
        eprintln!("FH: bad {} certified {} miss {} on {} uncertain {} escalated {}", stats[5], stats[0], stats[1], stats[2], stats[3], stats[4]);
    }

    /// Near-tangent ellipses against a sphere: the sphere grows through
    /// the ellipse's farthest point.
    #[test]
    fn near_tangent_sweep() {
        let mut stats = [0usize; 6];
        let e = ellipse(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0), 1.0, 0.2);
        for rr in [0.999_999_9, 0.999_999_999, 1.0, 1.000_000_001, 1.000_000_1, 0.2 - 1e-9, 0.2, 0.2 + 1e-9, 0.2 + 1e-7] {
            let s = geom::Surface::Sphere { center: Point3::new(0.0, 0.0, 0.0), radius: rr, axis: Vec3::new(0.0, 0.0, 1.0), u_ref: Vec3::new(1.0, 0.0, 0.0) };
            for t0 in [-0.5, 1.0, 2.9] {
                judge(&format!("tangent rr {rr} t0 {t0}"), &e, &s, t0, t0 + 1.0, &mut stats);
            }
            let w = geom::Surface::Cylinder { origin: Point3::new(0.0, 0.0, 0.0), axis: Vec3::new(0.0, 0.0, 1.0), radius: rr, u_ref: Vec3::new(1.0, 0.0, 0.0) };
            for t0 in [-0.5, 1.0, 2.9] {
                judge(&format!("tangent wall rr {rr} t0 {t0}"), &e, &w, t0, t0 + 1.0, &mut stats);
            }
        }
        eprintln!("NT: bad {} certified {} miss {} on {} uncertain {} escalated {}", stats[5], stats[0], stats[1], stats[2], stats[3], stats[4]);
    }

    /// Tangency sweep: the wall's own section with its major axis grown by
    /// `δ` touches the wall at θ = ±π/2 and lies outside elsewhere (min
    /// distance exactly 0). A certified `Miss` is a tangency misread. The
    /// circle door's analogue: a circle of the wall's radius tilted about
    /// a diameter, inside the wall, touching at the diameter's ends.
    #[test]
    fn tangency_sweep_ellipse_and_circle() {
        for rad in [0.5, 5.0, 50.0, 500.0, 5000.0] {
            for rel in [1e-12, 1e-11, 1e-10, 1e-9, 1e-8, 1e-6] {
                let delta = rel * rad;
                let phi = 0.3f64;
                let e = ellipse(Vec3::new(0.0, 0.0, 0.0), Vec3::new(phi.sin(), 0.0, phi.cos()), Vec3::new(1.0, 0.0, 0.0), rad / phi.cos() + delta, rad);
                let w = geom::Surface::Cylinder { origin: Point3::new(0.0, 0.0, 0.0), axis: Vec3::new(0.0, 0.0, 1.0), radius: rad, u_ref: Vec3::new(1.0, 0.0, 0.0) };
                let er = ellipse_roots(&e, 1.0, 2.2, &w, band());
                // circle: radius rad, tilted by psi with cos psi = 1 - rel
                let psi = (1.0 - rel).acos();
                let c = geom::Curve3::Circle { center: Point3::new(0.0, 0.0, 0.0), axis: Vec3::new(psi.sin(), 0.0, psi.cos()), radius: rad, u_ref: Vec3::new(psi.cos(), 0.0, -psi.sin()) };
                let cr = super::super::circle_cylinder::circle_cylinder_roots(&c, 1.0, 2.2, &w, band());
                let ctag = |r: &Result<CircleRoots<f64>, BooleanError>| match r { Ok(CircleRoots::Miss) => "MISS".to_string(), Ok(CircleRoots::Certified{count,..}) => format!("CERT{count}"), Ok(CircleRoots::Uncertain) => "unc".into(), Ok(CircleRoots::OnSurface) => "on".into(), Ok(_) => "other".into(), Err(_) => "esc".into() };
                eprintln!("TAN rad {rad} rel {rel:e}: ellipse(outside-touch) {} circle(inside-touch) {}", ctag(&er), ctag(&cr));
            }
        }
    }
}

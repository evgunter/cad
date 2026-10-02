//! Reviewer probe (reach-dual3805-r1): conic enclosures against dense
//! sampling of the residual, on eccentric ellipses near tori, spheres and
//! walls, short arcs near the major/minor vertices.
#![allow(clippy::unwrap_used, clippy::panic)]
use geom::Surface;
use geom_brep::{Conic, conic_arc_residual_range, conic_residual_extremes, implicit_residual};
use geom_core::{Point3, Vec3};

struct Rng(u64);
impl Rng {
    fn f(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn v(&mut self) -> Vec3<f64> {
        Vec3::new(self.f() * 2.0 - 1.0, self.f() * 2.0 - 1.0, self.f() * 2.0 - 1.0)
    }
}

#[test]
fn enclosures_hold_under_fuzz() {
    let mut rng = Rng(0xdead_beef_cafe_f00d);
    let mut worst = 0.0f64;
    let mut n = 0;
    for i in 0..3000 {
        let n_ = rng.v().normalize();
        let u = rng.v();
        let u = (u - n_ * u.dot(n_)).normalize();
        let b = 0.02 + 0.5 * rng.f();
        let a = b * [1.0, 1.3, 5.0, 40.0][i % 4];
        let c = rng.v() * 0.6;
        let conic = Conic { center: Point3::new(c.x, c.y, c.z), axis: n_, u_ref: u, major: a, minor: b };
        let sc = rng.v() * 0.5;
        let ax = rng.v().normalize();
        let x = Vec3::new(1.0, 0.0, 0.0);
        let ur = (x - ax * x.dot(ax)).normalize();
        let s = match i % 3 {
            0 => Surface::Sphere { center: Point3::new(sc.x, sc.y, sc.z), radius: 0.1 + rng.f(), axis: ax, u_ref: ur },
            1 => Surface::Cylinder { origin: Point3::new(sc.x, sc.y, sc.z), axis: ax, radius: 0.1 + rng.f(), u_ref: ur },
            _ => {
                let rr = 0.3 + rng.f();
                Surface::Torus { center: Point3::new(sc.x, sc.y, sc.z), axis: ax, major_radius: rr, minor_radius: rr * (0.1 + 0.8 * rng.f()), u_ref: ur }
            }
        };
        // arcs: around the vertices (0, π/2) and arbitrary
        let base = [0.0, core::f64::consts::FRAC_PI_2, rng.f() * 6.28][i % 3];
        let span = [1e-3, 0.05, 0.5, 3.0, 6.28][(i / 3) % 5];
        let t0 = base - span * rng.f();
        let t1 = t0 + span;
        let Some((lo, hi)) = conic_arc_residual_range(&s, &conic, t0, t1) else { continue };
        if !lo.is_finite() || !hi.is_finite() {
            continue;
        }
        n += 1;
        let k = 20_000;
        for j in 0..=k {
            let t = t0 + (t1 - t0) * f64::from(j) / f64::from(k);
            let r = implicit_residual(&s, conic.point(t));
            let out = (lo - r).max(r - hi);
            worst = worst.max(out);
            assert!(out <= 1e-12, "case {i} {s:?} a {a} b {b} [{t0},{t1}]: residual {r} outside [{lo},{hi}]");
        }
        if i % 3 != 2 {
            let (lo, hi) = conic_residual_extremes(&s, &conic).unwrap();
            for j in 0..=20_000 {
                let t = core::f64::consts::TAU * f64::from(j) / 20_000.0;
                let r = implicit_residual(&s, conic.point(t));
                assert!(r >= lo - 1e-12 && r <= hi + 1e-12, "case {i}: whole-turn {r} outside [{lo},{hi}]");
            }
        }
    }
    eprintln!("enclosures: {n} finite arcs checked, worst excess {worst:e}");
}

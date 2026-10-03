//! Reviewer r1 differential probe for PR #3973: the circle × torus and
//! circle × sphere doors' verdicts on one pose stream, printed bit-exact as
//! `R1CD` lines, built on the merge base and on the frozen head and diffed.
//! Mounted by appending `#[cfg(test)] #[path = "../../../../probes/r1_circle_diff.rs"]
//! mod r1_diff;` to `crates/topo/src/boolean/circle_torus.rs` in each tree.
#![allow(clippy::unwrap_used, clippy::panic)]
use crate::boolean::circle_roots::CircleRoots;
use core::f64::consts::{PI, TAU};
use geom_core::{Band, Point3, Vec3};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 { a + (b - a) * self.next() }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
            let n = v.norm();
            if n > 0.1 && n < 1.0 { return v / n; }
        }
    }
    fn perp(&mut self, a: Vec3<f64>) -> Vec3<f64> { let w = self.unit(); (w - a * w.dot(a)).normalize() }
}

fn fmt(r: &Result<CircleRoots<f64>, crate::boolean::BooleanError>) -> String {
    match r {
        Ok(CircleRoots::Certified { count, thetas }) => {
            let ts: Vec<String> = thetas[..*count].iter().map(|t| format!("{:016x}", t.to_bits())).collect();
            format!("certified {}", ts.join(","))
        }
        Ok(other) => format!("{other:?}"),
        Err(e) => format!("err {e:?}"),
    }
}

#[test]
#[ignore = "reviewer differential dump"]
fn r1_circle_diff() {
    let mut rng = Rng(0xD1FF_0003_973A_u64);
    let per = std::env::var("R1N").map(|s| s.parse::<usize>().unwrap()).unwrap_or(60);
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for scale in [1e-3, 1.0, 1e3] {
            for fam in 0..4 {
                for i in 0..per {
                    let rad = scale * rng.range(0.1, 2.0);
                    let n = rng.unit();
                    let u = rng.perp(n);
                    let c0 = if fam == 3 { Point3::new(0.0, 0.0, 0.0) + rng.unit() * (1e3 * scale) } else { Point3::new(0.0, 0.0, 0.0) };
                    let circle = geom::Curve3::Circle { center: c0, axis: n, radius: rad, u_ref: u };
                    let theta = rng.range(0.0, TAU);
                    let p = circle.eval(theta);
                    let tan = circle.deriv(theta).normalize();
                    let nrm = rng.perp(tan);
                    let gap = if fam == 0 { -scale * rng.range(0.0, 0.3) } else { eps * rng.range(-40.0, 40.0) };
                    let surf = p - nrm * gap;
                    // Torus: tube radius `small` at tube angle v, axis chosen square-ish.
                    let small = scale * rng.range(0.03, 0.5);
                    let big = small * rng.range(1.2, 6.0);
                    let v = if fam == 1 { 0.0 } else if fam == 2 { PI } else { rng.range(0.0, TAU) };
                    let side = rng.perp(nrm);
                    let core = surf - nrm * small;
                    // Core circle through `core`, tangent `side`; hub toward -nrm rotated by v.
                    let (sv, cv) = v.sin_cos();
                    let binorm = nrm.cross(side);
                    let radial = nrm * cv + binorm * (-sv); // outward-from-axis direction at core
                    let axis = side.cross(radial).normalize();
                    let hub = core - radial * big;
                    let x = Vec3::new(1.0, 0.0, 0.0);
                    let x = if axis.cross(x).norm() < 0.1 { Vec3::new(0.0, 1.0, 0.0) } else { x };
                    let torus = geom::Surface::Torus { center: hub, axis, major_radius: big, minor_radius: small, u_ref: (x - axis * x.dot(axis)).normalize() };
                    let sphere = geom::Surface::Sphere { center: surf - nrm * small, radius: small, u_ref: side, axis: binorm };
                    let t0 = rng.range(-TAU, TAU);
                    let t1 = t0 + rng.range(0.1, TAU);
                    let rt = super::circle_torus_roots(&circle, t0, t1, &torus, band);
                    let rs = crate::boolean::circle_sphere::circle_sphere_roots(&circle, t0, t1, &sphere, band);
                    println!("R1CD eps={eps:e} scale={scale:e} fam={fam} i={i} torus: {} | sphere: {}", fmt(&rt), fmt(&rs));
                    if std::env::var("R1POSE").is_ok() {
                        let v = |p: Vec3<f64>| format!("[{:?},{:?},{:?}]", p.x, p.y, p.z);
                        let pt = |p: Point3<f64>| format!("[{:?},{:?},{:?}]", p.x, p.y, p.z);
                        println!("R1CP {{\"tag\":\"eps={eps:e} scale={scale:e} fam={fam} i={i}\",\"fam\":\"circle\",\"scale\":{scale:?},\"eps\":{eps:?},\"c\":{},\"n\":{},\"u\":{},\"a\":{rad:?},\"b\":{rad:?},\"hub\":{},\"ax\":{},\"R\":{big:?},\"r\":{small:?},\"t0\":{t0:?},\"t1\":{t1:?},\"ans\":\"{}\",\"roots\":[{}]}}", pt(c0), v(n), v(u), pt(hub), v(axis), match &rt { Ok(CircleRoots::Certified{..}) => "certified", Ok(CircleRoots::Miss) => "miss", Ok(CircleRoots::Uncertain) => "uncertain", _ => "err" }, match &rt { Ok(CircleRoots::Certified{count, thetas}) => thetas[..*count].iter().map(|t| format!("{t:?}")).collect::<Vec<_>>().join(","), _ => String::new() });
                    }
                }
            }
        }
    }
}

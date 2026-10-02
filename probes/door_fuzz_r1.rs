// Reviewer probe (PR 3847 dual, lane r1). Spliced into
// crates/topo/src/boolean/circle_sphere.rs as
//   #[cfg(test)] #[path = "../../../../probes/door_fuzz_r1.rs"] mod door_fuzz_r1;
// Emits one line per (pose, eps): inputs as round-trip f64, the verdict,
// the certified thetas and the door's slack. The oracle is
// probes/oracle_r1.py (mpmath, 60 digits), never the kernel.
use super::*;
use geom_core::{Point3, Vec3};
use std::io::Write;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn sym(&mut self) -> f64 {
        2.0 * self.next() - 1.0
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.sym(), self.sym(), self.sym());
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
}

fn phase_term(h: &geom_brep::CircleSphereHarmonic<f64>, rho: f64) -> f64 {
    rho * h.phase_error / (h.e_u.powi(2) + h.e_v.powi(2)).sqrt()
}

fn slack_of(h: &geom_brep::CircleSphereHarmonic<f64>, rho: f64) -> f64 {
    let swing = h.hi - h.lo;
    let slope = (-h.lo * h.hi).max(0.0).sqrt();
    let at_root = (h.hi * h.lo_error - h.lo * h.hi_error) / swing;
    let phase = h.phase_error / (h.e_u.powi(2) + h.e_v.powi(2)).sqrt();
    rho * (at_root / slope + phase + rounding_charge(std::f64::consts::TAU))
}

#[test]
fn emit() {
    let path = std::env::var("PROBE_OUT").unwrap_or("/tmp/door_fuzz_r1.txt".into());
    let n: usize = std::env::var("PROBE_N").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
    let mut out = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    let switch = std::env::var("PROBE_SWITCH").is_ok();
    for k in 0..n {
        let scale = [1e-3, 1.0, 1e3][(k / 3) % 3];
        let tilted = k % 4 != 0;
        let (axis, u_ref) = if tilted {
            let a = rng.unit();
            let t = rng.unit();
            let u = (t - a * t.dot(a)).normalize();
            (a, u)
        } else {
            (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0))
        };
        let v = axis.cross(u_ref);
        let center = Point3::new(rng.sym() * scale, rng.sym() * scale, rng.sym() * scale);
        let rho = scale * (0.05 + 3.0 * rng.next());
        // sphere centre offset in the frame
        let m = scale * 4.0 * rng.next();
        let en = scale * rng.sym() * if k % 5 == 0 { 0.0 } else { 1.0 };
        let ang = rng.sym() * std::f64::consts::PI;
        let e = u_ref * (m * ang.cos()) + v * (m * ang.sin()) + axis * en;
        let s_center = center - e;
        let dm = ((m - rho).powi(2) + en * en).sqrt();
        let dp = ((m + rho).powi(2) + en * en).sqrt();
        // kind: 0 near-tangent at the near point (r = D- + δ), 1 near-tangent
        // at the far point (r = D+ − δ), 2 generic crossing
        let kind = k % 3;
        let delta = scale * 10f64.powf(-4.0 - 12.0 * rng.next());
        let delta = std::env::var("PROBE_DELTA").ok().and_then(|s| s.parse::<f64>().ok()).map_or(delta, |d| d * scale);
        let kind = if switch { 3 } else { kind };
        let r0 = (m * m + rho * rho + en * en).sqrt(); // c0 = 0: lo + hi = 0
        let r = match kind {
            3 => r0 * (1.0 + (k as f64 % 41.0 - 20.0) * f64::EPSILON),
            0 => dm + delta,
            1 => dp - delta,
            _ => dm + (dp - dm) * (0.02 + 0.96 * rng.next()),
        };
        if !(r > 0.0) {
            continue;
        }
        let carrier = geom::Curve3::Circle { center, axis, radius: rho, u_ref };
        let sphere = geom::Surface::Sphere {
            center: s_center,
            radius: r,
            axis: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let h = geom_brep::circle_sphere_harmonic(center, axis, rho, u_ref, s_center, r);
        let slack = slack_of(&h, rho);
        for eps in [1e-12, 1e-9, 1e-6] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            let got = circle_sphere_roots(&carrier, -3.0, 3.0, &sphere, band);
            let (verdict, th) = match got {
                Ok(CircleRoots::Certified { count, thetas }) => (format!("C{count}"), thetas),
                Ok(CircleRoots::Miss) => ("M".into(), [0.0; 4]),
                Ok(CircleRoots::Uncertain) => ("U".into(), [0.0; 4]),
                Ok(other) => (format!("O:{other:?}").replace(' ', ""), [0.0; 4]),
                Err(_) => ("E".into(), [0.0; 4]),
            };
            let nums = [
                center.x, center.y, center.z, axis.x, axis.y, axis.z, u_ref.x, u_ref.y, u_ref.z,
                s_center.x, s_center.y, s_center.z, rho, r, th[0], th[1], slack, h.lo, h.hi,
                h.lo_error, h.hi_error, phase_term(&h, rho), rho * rounding_charge(std::f64::consts::TAU),
            ];
            let nums: Vec<String> = nums.iter().map(|x| format!("{x:?}")).collect();
            writeln!(out, "{k} {kind} {eps:?} {verdict} {}", nums.join(" ")).unwrap();
        }
    }
}

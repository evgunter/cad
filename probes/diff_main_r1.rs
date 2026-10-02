// Reviewer probe (PR 3847 dual, lane r1): compiles on main AND the head.
// Spliced into crates/topo/src/boolean/circle_sphere.rs in each tree as
//   #[cfg(test)] #[path = ".../probes/diff_main_r1.rs"] mod diff_main_r1;
// Prints, per pose and band, the verdicts and the root BITS of the
// circle × sphere door and the circle × cylinder door (square arm:
// coaxial-parallel frames, tilt in the zero band), plus the harmonic's
// plain fields' bits, so `diff` of the two trees' outputs is the claim.
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
    fn sym(&mut self) -> f64 { 2.0 * self.next() - 1.0 }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.sym(), self.sym(), self.sym());
            let n = v.norm();
            if n > 0.2 && n < 1.0 { return v / n; }
        }
    }
}

fn show(r: Result<CircleRoots<f64>, BooleanError>) -> String {
    match r {
        Ok(CircleRoots::Certified { count, thetas }) => format!(
            "C{count}:{:016x}:{:016x}", thetas[0].to_bits(), thetas[1].to_bits()),
        Ok(o) => format!("{o:?}").replace(' ', ""),
        Err(_) => "E".into(),
    }
}

#[test]
fn emit() {
    let path = std::env::var("PROBE_OUT").unwrap();
    let mut out = std::io::BufWriter::new(std::fs::File::create(path).unwrap());
    let mut rng = Rng(0x2545_f491_4f6c_dd1d);
    for k in 0..6000usize {
        let scale = [1e-3, 1.0, 1e3][(k / 2) % 3];
        let axis = if k % 4 == 0 { Vec3::new(0.0, 0.0, 1.0) } else { rng.unit() };
        let t = rng.unit();
        let u_ref = (t - axis * t.dot(axis)).normalize();
        let v = axis.cross(u_ref);
        let center = Point3::new(rng.sym() * scale, rng.sym() * scale, rng.sym() * scale);
        let rho = scale * (0.05 + 3.0 * rng.next());
        let m = scale * 4.0 * rng.next();
        let en = scale * rng.sym();
        let ang = rng.sym() * std::f64::consts::PI;
        let e = u_ref * (m * ang.cos()) + v * (m * ang.sin()) + axis * en;
        let s_center = center - e;
        let dm = ((m - rho).powi(2) + en * en).sqrt();
        let dp = ((m + rho).powi(2) + en * en).sqrt();
        let delta = scale * 10f64.powf(-3.0 - 12.0 * rng.next());
        let r = match k % 3 { 0 => dm + delta, 1 => dp - delta, _ => dm + (dp - dm) * rng.next() };
        let carrier = geom::Curve3::Circle { center, axis, radius: rho, u_ref };
        let sphere = geom::Surface::Sphere {
            center: s_center, radius: r,
            axis: Vec3::new(0.0, 1.0, 0.0), u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // the cylinder: parallel to the circle's axis, its section circle
        // in the circle's plane at distance m, radius r_c
        let w_origin = center - (u_ref * (m * ang.cos()) + v * (m * ang.sin()));
        let rc = match k % 3 { 0 => (m - rho).abs() + delta, 1 => m + rho - delta, _ => (m - rho).abs() + (2.0 * rho.min(m)) * rng.next() };
        let wall = geom::Surface::Cylinder { origin: w_origin, axis, radius: rc, u_ref };
        let h = geom_brep::circle_sphere_harmonic(center, axis, rho, u_ref, s_center, r);
        let plain = format!("{:016x}{:016x}{:016x}{:016x}{:016x}",
            h.c0.to_bits(), h.a1.to_bits(), h.e_u.to_bits(), h.e_v.to_bits(), h.terms.to_bits());
        for eps in [1e-12, 1e-9, 1e-6] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            let s = show(circle_sphere_roots(&carrier, -3.0, 3.0, &sphere, band));
            let c = show(super::super::circle_cylinder::circle_cylinder_roots(&carrier, -3.0, 3.0, &wall, band));
            writeln!(out, "{k} {} {eps:?} {plain} S={s} Y={c}", k % 3).unwrap();
        }
    }
}

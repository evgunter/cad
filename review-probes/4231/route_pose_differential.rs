// Differential probe: route_pose on cone x cylinder over edge spans.
use geom::{Curve3, Surface};
use geom_brep::Reach;
use geom_brep::intersect::route_pose;
use geom_core::{Band, Point3, Tol, Vec3};
use std::sync::Arc;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.next()
    }
    fn pick<T: Copy>(&mut self, xs: &[T]) -> T {
        xs[(self.next() * xs.len() as f64) as usize % xs.len()]
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
            let n = v.norm();
            if n > 0.1 && n < 1.0 {
                return v * (1.0 / n);
            }
        }
    }
}

fn perp(a: Vec3<f64>, r: &mut Rng) -> Vec3<f64> {
    loop {
        let v = r.unit();
        let w = v - a * v.dot(a);
        if w.norm() > 0.2 {
            return w * (1.0 / w.norm());
        }
    }
}

fn outcome(g: Result<geom_brep::intersect::PairRoute, geom_brep::SectionError>) -> String {
    match g {
        Ok(r) if r.implemented => "S".into(),
        Ok(r) => {
            if r.note.contains("OFF it") {
                "R_off".into()
            } else if r.note.contains("tilted") {
                "R_tilt".into()
            } else {
                format!("R_other")
            }
        }
        Err(geom_brep::SectionError::Escalated(d)) => format!("E_{}", d.predicate.unwrap_or("?")),
        Err(e) => format!("X_{:?}", std::mem::discriminant(&e)),
    }
}

fn main() {
    let n: usize = std::env::args().nth(1).map(|s| s.parse().unwrap()).unwrap_or(20000);
    let band = Band::linear(Tol::witness()).unwrap();
    let zero = band.zero();
    let esc = band.escalate();
    let mut r = Rng(0x5eed_1234);
    for id in 0..n {
        // Cone.
        let apex = Point3::new(r.range(-5., 5.), r.range(-5., 5.), r.range(-5., 5.));
        let a = r.unit();
        let alpha = r.range(0.1, 1.4);
        let ux = perp(a, &mut r);
        let vy = a.cross(ux);
        let cone = Surface::Cone { apex, axis: a, half_angle: alpha, u_ref: ux };
        let on_cone = |u: f64, v: f64| {
            apex + a * (v * alpha.cos()) + (ux * u.cos() + vy * u.sin()) * (v * alpha.sin())
        };
        // Edge on the cone.
        let scale = r.pick(&[1.0, 1.0, 10.0, 100.0, 500.0]);
        let sign = if r.next() < 0.25 { -1.0 } else { 1.0 };
        let v0 = sign * r.range(0.01, 1.0) * scale;
        let v1 = v0 + sign * r.range(0.001, 0.5) * scale.min(10.0);
        let u = r.range(0.0, 6.28);
        let kind = r.pick(&[0u8, 1, 2]);
        let (carrier, t0, t1) = match kind {
            0 => {
                let p0 = on_cone(u, v0);
                let p1 = on_cone(u, v1);
                let len = (p1 - p0).norm();
                (Curve3::Line { origin: p0, dir: (p1 - p0) * (1.0 / len) }, 0.0, len)
            }
            1 => {
                let c = apex + a * (v0 * alpha.cos());
                (
                    Curve3::Circle { center: c, axis: a, radius: (v0 * alpha.sin()).abs(), u_ref: ux },
                    0.0,
                    r.range(0.3, 6.28),
                )
            }
            _ => {
                let m = r.range(0.1, 0.9);
                let vm = v0 + (v1 - v0) * r.next();
                let ctrl = vec![on_cone(u, v0), on_cone(u, vm), on_cone(u, v1)];
                let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, m, 1.0, 1.0], 1).unwrap();
                (
                    Curve3::Nurbs(Arc::new(geom::NurbsCurve3::new(kv, ctrl, vec![1.0; 3]).unwrap())),
                    0.0,
                    1.0,
                )
            }
        };
        let samples: Vec<Point3<f64>> = (0..=200).map(|i| carrier.eval(t0 + (t1 - t0) * i as f64 / 200.0)).collect();
        let true_l = samples.iter().fold(0.0f64, |m, &p| m.max((p - apex).norm()));
        // Cylinder: tilt theta, offset d at the apex, stored origin s along.
        let kt = r.pick(&[0.0, 0.3, 0.7, 0.9, 1.1, 1.5, 2.0, 3.0, 10.0, 100.0, 1e4]);
        let kd = r.pick(&[0.0, 0.0, 0.3, 0.7, 0.9, 1.1, 1.5, 3.0, 100.0]);
        let theta = kt * zero / true_l;
        let tdir = perp(a, &mut r);
        let mut b = a * theta.cos() + tdir * theta.sin();
        if r.next() < 0.3 {
            b = -b;
        }
        let ddir = perp(b, &mut r);
        let p0 = apex + ddir * (kd * zero); // nearest-ish point of the cyl axis to apex
        let s = r.pick(&[0.0, 1.0, -1.0, 10.0, -10.0, 100.0, -100.0, 1000.0, -1000.0, 1e4]);
        let rad = if r.next() < 0.5 { (v0 * alpha.sin()).abs() } else { r.range(0.05, 3.0) * scale.min(10.0) };
        let cyl = Surface::Cylinder { origin: p0 + b * s, axis: b, radius: rad, u_ref: perp(b, &mut r) };
        let reach = Reach::Span { carrier: carrier.clone(), t0, t1 };
        let swap = r.next() < 0.5;
        let got = if swap { route_pose(&cyl, &cone, &reach, band) } else { route_pose(&cone, &cyl, &reach, band) };
        let o = outcome(got);
        // Truth: the cylinder axis's distance from the cone axis at each consumed station.
        let dev = samples.iter().fold(0.0f64, |m, &x| {
            let t = (x - apex).dot(a);
            let lam = (t - (p0 - apex).dot(a)) / b.dot(a);
            let p = p0 + b * lam;
            let w = p - apex - a * t;
            m.max(w.norm())
        });
        let tilt = a.cross(b).norm();
        let lev_head = reach.lever_from(apex);
        let lev_origin = reach.lever_from(p0 + b * s);
        let lev_main = lev_head.max(lev_origin);
        println!(
            "{id} {o} kind={kind} kt={kt} kd={kd} s={s} scale={scale} sign={sign} dev_over_zero={:.4} tiltL_over_zero={:.4} truel={true_l:.6e} lev_head={lev_head:.6e} lev_main={lev_main:.6e} esc_over_zero={:.3}",
            dev / zero,
            tilt * true_l / zero,
            esc / zero
        );
    }
}

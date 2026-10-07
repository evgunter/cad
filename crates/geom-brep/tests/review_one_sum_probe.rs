//! Review probe (PR 4280): one deterministic pose stream run on main and
//! head; prints one line per pose — `P <family> <i> <label> <truth/zero>`.
//! Not a suite: `#[ignore]`d, run by name with `--ignored --nocapture`.

#![allow(clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::intersect::{
    ConeCylinderSection, EqualCylinderSection, PlaneConeSection, PlaneCylinderSection,
    PlaneTorusSection, RadiusEvidence, cone_cylinder_section, cylinder_cylinder_section,
    plane_cone_section, plane_cylinder_section, plane_torus_section,
};
use geom_brep::{ExtentBall, Reach, SectionError, TangentLocus, tangent_locus};
use geom_core::{Point3, Vec3};

struct Rng(u64);
impl Rng {
    fn u(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn r(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.u()
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.r(-1., 1.), self.r(-1., 1.), self.r(-1., 1.));
            let n = v.norm();
            if n > 0.2 && n < 1.0 {
                return v / n;
            }
        }
    }
    /// A term near the band: 70% in ±2.5·zero, 30% in ±1.5·K·zero.
    fn term(&mut self, z: f64, k: f64) -> f64 {
        if self.u() < 0.7 { self.r(-2.5, 2.5) * z } else { self.r(-1.5, 1.5) * k * z }
    }
}

fn perp(a: Vec3<f64>, rng: &mut Rng) -> Vec3<f64> {
    loop {
        let v = rng.unit();
        let w = v - a * v.dot(a);
        if w.norm() > 0.3 {
            return w.normalize();
        }
    }
}

fn esc<T>(r: &Result<T, SectionError>) -> String {
    match r {
        Err(SectionError::Escalated(d)) => format!("ESC:{}", d.predicate.unwrap_or("?")),
        Err(e) => format!("ERR:{}", format!("{e:?}").split([' ', '(', '{']).next().unwrap_or("")),
        Ok(_) => unreachable!(),
    }
}

fn head_of(s: String) -> String {
    s.split([' ', '(', '{']).next().unwrap_or("").to_string()
}

fn circle_pts(c: &Curve3<f64>, n: usize) -> Vec<Point3<f64>> {
    match *c {
        Curve3::Circle { center, axis, radius, u_ref } => {
            let u = (u_ref - axis * u_ref.dot(axis)).normalize();
            let v = axis.cross(u);
            (0..n)
                .map(|i| {
                    let t = std::f64::consts::TAU * i as f64 / n as f64;
                    center + u * (radius * t.cos()) + v * (radius * t.sin())
                })
                .collect()
        }
        _ => panic!("not a circle"),
    }
}

fn line_pts(c: &Curve3<f64>, from: Point3<f64>, len: f64, n: usize) -> Vec<Point3<f64>> {
    match *c {
        Curve3::Line { origin, dir } => {
            let d = dir.normalize();
            let t0 = (from - origin).dot(d);
            (0..=n)
                .map(|i| origin + d * (t0 - len + 2.0 * len * i as f64 / n as f64))
                .collect()
        }
        _ => panic!("not a line"),
    }
}

fn d_plane(p: Point3<f64>, q: Point3<f64>, n: Vec3<f64>) -> f64 {
    (p - q).dot(n).abs()
}
fn d_axis(p: Point3<f64>, o: Point3<f64>, a: Vec3<f64>) -> f64 {
    let w = p - o;
    (w - a * w.dot(a)).norm()
}
fn d_torus(p: Point3<f64>, c: Point3<f64>, a: Vec3<f64>, big: f64, r: f64) -> f64 {
    let w = p - c;
    let h = w.dot(a);
    let rho = (w - a * h).norm();
    (((rho - big).powi(2) + h * h).sqrt() - r).abs()
}
fn d_cone(p: Point3<f64>, apex: Point3<f64>, a: Vec3<f64>, al: f64) -> f64 {
    let w = p - apex;
    let x = w.dot(a);
    let rho = (w - a * x).norm();
    (rho * al.cos() - x * al.sin()).abs()
}

#[test]
#[ignore = "review probe"]
fn review_one_sum_probe() {
    let b = band();
    let z = b.zero();
    let k = b.escalate() / z;
    let n: usize = 6000;
    let mut rng = Rng(0x4280);
    for i in 0..n {
        let s = [1e-3, 1.0, 1e3][i % 3];
        let shift = rng.unit() * 1e3;
        // ---------------- plane × torus, cap lane
        {
            let a = rng.unit();
            let c = Point3::origin() + shift;
            let big = s * rng.r(1.0, 3.0);
            let r = s * rng.r(0.1, 0.9);
            let tilt = rng.r(0.0, 1.5) * z / big;
            let x = perp(a, &mut rng);
            let nrm = a * tilt.cos() + x * tilt.sin();
            let sign = if rng.u() < 0.5 { 1.0 } else { -1.0 };
            let lift = rng.term(z, k);
            let q = c + a * (sign * (r + lift));
            let extent = (big + r) * rng.r(1.0, 2.0);
            let torus = Surface::Torus { center: c, axis: a, major_radius: big, minor_radius: r, u_ref: x };
            let plane = Surface::Plane { origin: q, normal: nrm, u_ref: perp(nrm, &mut rng) };
            let got = plane_torus_section(&plane, &torus, extent, b);
            let (label, truth) = match &got {
                Ok(PlaneTorusSection::TangentCircle(cv)) => {
                    let t = circle_pts(cv, 720)
                        .into_iter()
                        .map(|p| d_plane(p, q, nrm).max(d_torus(p, c, a, big, r)))
                        .fold(0.0, f64::max);
                    ("TangentCircle".to_string(), t)
                }
                Ok(v) => (head_of(format!("{v:?}")), f64::NAN),
                e => (esc(e), f64::NAN),
            };
            println!("P ptcap {i} {label} {:.4}", truth / z);
        }
        // ---------------- plane × torus, axis-in-plane lane (meridian / two-ovals)
        {
            let a = rng.unit();
            let c = Point3::origin() + shift;
            let big = s * rng.r(1.0, 3.0);
            let r = s * rng.r(0.1, 0.9);
            let extent = (big + r) * rng.r(1.0, 2.0);
            let tiltv = rng.r(-1.5, 1.5) * z / extent;
            let x = perp(a, &mut rng);
            let nrm = x * tiltv.cos() + a * tiltv.sin();
            let ovals = rng.u() < 0.4;
            let d = if ovals {
                (big - r) - rng.term(z, k)
            } else {
                rng.term(z, k)
            };
            let q = c + x * d + perp(x, &mut rng) * (s * rng.r(-1.0, 1.0));
            let torus = Surface::Torus { center: c, axis: a, major_radius: big, minor_radius: r, u_ref: x };
            let plane = Surface::Plane { origin: q, normal: nrm, u_ref: perp(nrm, &mut rng) };
            let got = plane_torus_section(&plane, &torus, extent, b);
            let (label, truth) = match &got {
                Ok(PlaneTorusSection::MeridianCircles { c1, c2 }) => {
                    let t = circle_pts(c1, 720)
                        .into_iter()
                        .chain(circle_pts(c2, 720))
                        .map(|p| d_plane(p, q, nrm).max(d_torus(p, c, a, big, r)))
                        .fold(0.0, f64::max);
                    ("MeridianCircles".to_string(), t)
                }
                Ok(v) => (head_of(format!("{v:?}")), f64::NAN),
                e => (esc(e), f64::NAN),
            };
            println!("P pt{} {i} {label} {:.4}", if ovals { "oval" } else { "mer" }, truth / z);
        }
        // ---------------- plane × cone, apex lane
        {
            let a = rng.unit();
            let apex = Point3::origin() + shift;
            let al = rng.r(0.2, 1.2);
            let extent = s * rng.r(0.5, 3.0);
            let x = perp(a, &mut rng);
            // A plane through the apex tangent along a generator has its
            // normal at angle π/2 − α... from the axis; lean it by δ.
            let delta = rng.term(z, k) / extent;
            let beta = std::f64::consts::FRAC_PI_2 - al + delta;
            let nrm = a * beta.cos() + x * beta.sin();
            let g = rng.r(-1.5, 1.5) * z;
            let q = apex + nrm * g;
            let cone = Surface::Cone { apex, axis: a, half_angle: al, u_ref: x };
            let plane = Surface::Plane { origin: q, normal: nrm, u_ref: perp(nrm, &mut rng) };
            let got = plane_cone_section(&plane, &cone, extent, b);
            let (label, truth) = match &got {
                Ok(PlaneConeSection::ApexTangentLine(cv)) => {
                    let t = line_pts(cv, apex, extent, 400)
                        .into_iter()
                        .map(|p| d_plane(p, q, nrm).max(d_cone(p, apex, a, al).min(d_cone(p, apex, -a, al))))
                        .fold(0.0, f64::max);
                    ("ApexTangentLine".to_string(), t)
                }
                Ok(v) => (head_of(format!("{v:?}")), f64::NAN),
                e => (esc(e), f64::NAN),
            };
            println!("P pnapex {i} {label} {:.4} {:.4} {:.4} {s}", truth / z, delta * extent / z, g / z);
        }
        // ---------------- cylinder pair, parallel lane (+ witness)
        {
            let a1 = rng.unit();
            let o1 = Point3::origin() + shift;
            let r1 = s * rng.r(0.1, 1.0);
            let internal = rng.u() < 0.3;
            let r2 = if internal { r1 * rng.r(1.5, 3.0) } else { r1 };
            let y = perp(a1, &mut rng);
            let rad = s * rng.r(0.3, 2.0);
            let lever = r1 + rad; // rough
            let tilt = rng.r(0.0, 1.5) * z / lever;
            let zt = perp(y, &mut rng);
            let zt = (zt - a1 * zt.dot(a1)).normalize();
            let a2 = a1 * tilt.cos() + zt * tilt.sin();
            let coax = rng.u() < 0.15;
            let d = if coax {
                rng.r(0.0, 2.5) * z
            } else if internal {
                (r2 - r1) + rng.term(z, k)
            } else {
                (r1 + r2) + rng.term(z, k)
            };
            let o2 = o1 + y * d + a2 * (s * rng.r(-1e3, 1e3) / s.max(1.0));
            let c1 = Surface::Cylinder { origin: o1, axis: a1, radius: r1, u_ref: y };
            let c2 = Surface::Cylinder { origin: o2, axis: a2, radius: if coax { r1 } else { r2 }, u_ref: perp(a2, &mut rng) };
            let ball = ExtentBall::new(o1 + y * (r1 * rng.r(0.5, 1.0)) + a1 * (s * rng.r(-1.0, 1.0)), rad);
            let got = cylinder_cylinder_section(&c1, &c2, RadiusEvidence::Declared, &Reach::Ball(ball), b);
            let label = match &got {
                Ok(v) => head_of(format!("{v:?}")),
                e => esc(e),
            };
            let w = tangent_locus(&c1, &c2, ball, b);
            let wl = match &w {
                Ok(TangentLocus::Line { .. }) => "Line".to_string(),
                Ok(v) => head_of(format!("{v:?}")),
                Err(e) => match e {
                    geom_brep::TangentLocusError::Escalated(d) => format!("ESC:{}", d.predicate.unwrap_or("?")),
                    other => head_of(format!("{other:?}")),
                },
            };
            let _ = EqualCylinderSection::<f64>::Empty;
            println!("P cc {i} {label} nan");
            println!("P ccw {i} {wl} nan");
        }
        // ---------------- plane × cylinder
        {
            let a = rng.unit();
            let o = Point3::origin() + shift;
            let r = s * rng.r(0.1, 1.0);
            let nx = perp(a, &mut rng);
            let rad = s * rng.r(0.3, 2.0);
            let tilt = rng.r(0.0, 1.5) * z / (r + rad);
            let nrm = nx * tilt.cos() + a * tilt.sin();
            let gap = r + rng.term(z, k);
            let q = o - nrm * gap;
            let wall = Surface::Cylinder { origin: o + a * (s * rng.r(-1e3, 1e3) / s.max(1.0)), axis: a, radius: r, u_ref: nx };
            let plane = Surface::Plane { origin: q, normal: nrm, u_ref: perp(nrm, &mut rng) };
            let ball = ExtentBall::new(o - nrm * (r * rng.r(0.5, 1.0)) + a * (s * rng.r(-1.0, 1.0)), rad);
            let got = plane_cylinder_section(&plane, &wall, &Reach::Ball(ball), b);
            let label = match &got {
                Ok(v) => head_of(format!("{v:?}")),
                e => esc(e),
            };
            let _ = PlaneCylinderSection::<f64>::Empty;
            println!("P pc {i} {label} nan");
        }
        // ---------------- cone × cylinder
        {
            let a = rng.unit();
            let apex = Point3::origin() + shift;
            let al = rng.r(0.2, 1.2);
            let extent = s * rng.r(1.0, 3.0);
            let rr = extent * al.tan() * rng.r(0.1, 0.8) * al.cos();
            let tilt = rng.r(0.0, 1.5) * z / extent;
            let x = perp(a, &mut rng);
            let b2 = a * tilt.cos() + x * tilt.sin();
            let off = rng.r(0.0, 2.5) * z;
            let y = perp(b2, &mut rng);
            let cone = Surface::Cone { apex, axis: a, half_angle: al, u_ref: x };
            let wall = Surface::Cylinder { origin: apex + y * off + b2 * (s * rng.r(-3.0, 3.0)), axis: b2, radius: rr, u_ref: y };
            let got = cone_cylinder_section(&cone, &wall, extent, b);
            let (label, truth) = match &got {
                Ok(ConeCylinderSection::CoaxialCircles { c1, c2 }) => {
                    let t = circle_pts(c1, 720)
                        .into_iter()
                        .chain(circle_pts(c2, 720))
                        .map(|p| (d_axis(p, apex + y * off, b2) - rr).abs().max(d_cone(p, apex, a, al).min(d_cone(p, apex, -a, al))))
                        .fold(0.0, f64::max);
                    ("CoaxialCircles".to_string(), t)
                }
                Ok(v) => (head_of(format!("{v:?}")), f64::NAN),
                e => (esc(e), f64::NAN),
            };
            println!("P coc {i} {label} {:.4}", truth / z);
        }
    }
}

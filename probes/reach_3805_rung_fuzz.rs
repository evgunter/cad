//! Reviewer probe for PR #3805, third delta review (head 7b8f75faf6).
//!
//! Mounted temporarily as a child of `crates/topo/src/boolean/reduce.rs`:
//!
//! ```text
//! #[cfg(test)]
//! #[path = "../../../../probes/reach_3805_rung_fuzz.rs"]
//! mod reviewer_rung_probe;
//! ```
//!
//! then `PROBE_OUT=/path/out.tsv cargo nextest run -p topo reviewer_rung_probe --no-capture`.
//!
//! It drives the CLEARANCE RUNG itself (`super::conic_clearance`), and
//! on a non-`Positive` answer the root door `wall_crossing` would ask
//! (circle × sphere/cylinder/torus, ellipse × sphere/cylinder; the rest
//! are `Unsettled`), and writes every pose and both answers as one TSV
//! row, the f64 inputs in round-trip `{:?}` form. The oracle is
//! `probes/reach_3805_rung_oracle.py` (60-digit decimal, stored data
//! read exactly); nothing here judges the answers.
//!
//! For the merge-base run (old charge), `RUNG` below is swapped for
//! the restated old body (the same two `geom_brep` reads; the signature
//! took an `EdgeCurve` then).
//!
//! Poses: circles (radius sign drawn) and ellipses in all 8 stored
//! orders/signs, eccentricity ≤ 40, size S·[0.5, 5] with S ∈ {1 m, 1 km,
//! 100 km}, centre up to 1 m / 1 km / 100 km out; grazed at a random
//! vertex by a sphere, a cylinder, a torus or a cone set off along a
//! normal of the carrier at the vertex (in-plane outward, or tilted out
//! of plane) by `gap` = ε·[−40, 40]; arc = vertex ± 0.5 rad.

use core::f64::consts::FRAC_PI_2;
use std::io::Write as _;

use super::*;
use geom_core::{Point3, Vec3};

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
    fn below(&mut self, n: u32) -> u32 {
        (self.next() * f64::from(n)) as u32 % n
    }
    fn unit(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1.0, 1.0), self.range(-1.0, 1.0), self.range(-1.0, 1.0));
            if v.norm() > 0.2 && v.norm() < 1.0 {
                return v.normalize();
            }
        }
    }
}

fn rung(
    s: &geom::Surface<f64>,
    conic: &geom_brep::Conic<f64>,
    t0: f64,
    t1: f64,
    band: Band,
) -> Option<Result<Sign, geom_core::Indeterminate>> {
    super::conic_clearance(s, conic, (t0, t1), band) // RUNG
}

fn v(p: Vec3<f64>) -> String {
    format!("{:?},{:?},{:?}", p.x, p.y, p.z)
}
fn pt(p: Point3<f64>) -> String {
    format!("{:?},{:?},{:?}", p.x, p.y, p.z)
}

fn envf(k: &str) -> Option<f64> {
    std::env::var(k).ok().and_then(|s| s.parse().ok())
}

#[test]
fn reviewer_rung_fuzz() {
    let n: u32 = std::env::var("PROBE_N").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);
    let out = std::env::var("PROBE_OUT").unwrap_or_else(|_| "/tmp/rung.tsv".into());
    let mut f = std::io::BufWriter::new(std::fs::File::create(out).unwrap());
    for eps in [1e-12, 1e-9, 1e-6] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for kind in 0..envf("PROBE_KINDS").map_or(4, |k| k as u32) {
            let mut rng = Rng(0x9E37_79B9_7F4A_7C15 ^ u64::from(kind) ^ eps.to_bits());
            for i in 0..n {
                let n_ax = rng.unit();
                let u = rng.unit();
                let u_ref = (u - n_ax * u.dot(n_ax)).normalize();
                let size = envf("PROBE_SIZE").unwrap_or([1.0, 1e3, 1e5][rng.below(3) as usize]);
                let big = size * rng.range(0.5, 5.0);
                let circle = i % 5 == 4;
                let combo = i % 8;
                let small = if circle { big } else { big / rng.range(1.0, 40.0) };
                let (mut major, mut minor) = if combo & 1 == 0 { (big, small) } else { (small, big) };
                if combo & 2 != 0 {
                    major = -major;
                }
                if !circle && combo & 4 != 0 {
                    minor = -minor;
                }
                let far = envf("PROBE_FAR").unwrap_or([1.0, 1e3, 1e5][rng.below(3) as usize]);
                let center = Point3::new(rng.range(-far, far), rng.range(-far, far), rng.range(-far, far));
                let e = if circle {
                    geom::Curve3::Circle { center, axis: n_ax, radius: major, u_ref }
                } else {
                    geom::Curve3::Ellipse { center, axis: n_ax, major, minor, u_ref }
                };
                let conic = geom_brep::Conic::of(&e).unwrap();
                let k = rng.below(4);
                let vertex = FRAC_PI_2 * f64::from(k);
                let p = e.eval(vertex);
                let outward = (p - center).normalize();
                let tangent = n_ax.cross(outward);
                let psi = if rng.below(2) == 0 { 0.0 } else { rng.range(-1.2, 1.2) };
                let nrm = outward * psi.cos() + n_ax * psi.sin();
                let gap = match envf("PROBE_GAPLOG") {
                    Some(hi) => eps * 10f64.powf(rng.range(1.5, hi)),
                    None => eps * rng.range(-40.0, 40.0),
                };
                let r = 10f64.powf(rng.range(envf("PROBE_RLOG").unwrap_or(-6.0), 0.0));
                let x = Vec3::new(1.0, 0.0, 0.0);
                let perp = |a: Vec3<f64>| (x - a * x.dot(a)).normalize();
                let (s, desc) = match kind {
                    0 => {
                        let c = p + nrm * (r + gap);
                        (
                            geom::Surface::Sphere { center: c, radius: r, axis: n_ax, u_ref: perp(n_ax) },
                            format!("S\t{}\t{:?}", pt(c), r),
                        )
                    }
                    1 => {
                        let w = if rng.below(2) == 0 {
                            tangent
                        } else {
                            let vv = rng.unit();
                            (vv - nrm * vv.dot(nrm)).normalize()
                        };
                        let o = p + nrm * (r + gap);
                        (
                            geom::Surface::Cylinder { origin: o, axis: w, radius: r, u_ref: nrm },
                            format!("C\t{}\t{}\t{:?}", pt(o), v(w), r),
                        )
                    }
                    2 => {
                        let big_r = r * rng.range(1.5, 10.0);
                        let vv = rng.unit();
                        let w2 = (vv - nrm * vv.dot(nrm)).normalize();
                        let t0c = p + nrm * (r + gap);
                        let axis = nrm.cross(w2).normalize();
                        let c = t0c + nrm * big_r;
                        (
                            geom::Surface::Torus {
                                center: c,
                                axis,
                                major_radius: big_r,
                                minor_radius: r,
                                u_ref: perp(axis),
                            },
                            format!("T\t{}\t{}\t{:?}\t{:?}", pt(c), v(axis), big_r, r),
                        )
                    }
                    _ => {
                        let alpha = rng.range(0.2, 1.2);
                        let vv = rng.unit();
                        let g = (vv - nrm * vv.dot(nrm)).normalize();
                        let axis = (g * alpha.cos() + nrm * alpha.sin()).normalize();
                        let apex = p + nrm * gap - g * (10.0 * r);
                        (
                            geom::Surface::Cone { apex, axis, half_angle: alpha, u_ref: perp(axis) },
                            format!("K\t{}\t{}\t{:?}", pt(apex), v(axis), alpha),
                        )
                    }
                };
                let half = envf("PROBE_ARC").unwrap_or(0.5);
                let (t0, t1) = (vertex - half, vertex + half);
                let got = rung(&s, &conic, t0, t1, band);
                let rung_s = match got {
                    None => "none".to_string(),
                    Some(Ok(Sign::Positive)) => "pos".into(),
                    Some(Ok(Sign::Negative)) => "neg".into(),
                    Some(Ok(Sign::Zero)) => "zero".into(),
                    Some(Err(_)) => "esc".into(),
                };
                let door = if matches!(got, Some(Ok(Sign::Positive))) {
                    "-".to_string()
                } else {
                    let d = match (&e, kind) {
                        (geom::Curve3::Circle { .. }, 0) => {
                            Some(crate::boolean::circle_sphere::circle_sphere_roots(&e, t0, t1, &s, band))
                        }
                        (geom::Curve3::Circle { .. }, 1) => {
                            Some(crate::boolean::circle_cylinder::circle_cylinder_roots(&e, t0, t1, &s, band))
                        }
                        (geom::Curve3::Circle { .. }, 2) => {
                            Some(crate::boolean::circle_torus::circle_torus_roots(&e, t0, t1, &s, band))
                        }
                        (geom::Curve3::Ellipse { .. }, 0 | 1) => {
                            Some(crate::boolean::ellipse_roots::ellipse_roots(&e, t0, t1, &s, band))
                        }
                        _ => None,
                    };
                    match d {
                        None => "unsettled".into(),
                        Some(Ok(CircleRoots::Miss)) => "miss".into(),
                        Some(Ok(CircleRoots::Certified { count, .. })) => format!("roots{count}"),
                        Some(Ok(_)) => "other".into(),
                        Some(Err(_)) => "err".into(),
                    }
                };
                writeln!(
                    f,
                    "{eps:?}\t{i}\t{}\t{combo}\t{}\t{}\t{:?}\t{:?}\t{}\t{:?}\t{:?}\t{:?}\t{:?}\t{size:?}\t{far:?}\t{psi:?}\t{rung_s}\t{door}\t{desc}",
                    if circle { "circle" } else { "ellipse" },
                    pt(center),
                    v(n_ax),
                    major,
                    minor,
                    v(u_ref),
                    t0,
                    t1,
                    vertex,
                    gap,
                )
                .unwrap();
            }
        }
    }
}

/// One TSV row (env `PROBE_ROW`, sphere or cylinder) replayed: both
/// enclosures and the rung, printed.
#[test]
fn reviewer_rung_replay() {
    let Ok(row) = std::env::var("PROBE_ROW") else { return };
    let f: Vec<&str> = row.trim_end().split('\t').collect();
    let num = |s: &str| s.parse::<f64>().unwrap();
    let v3 = |s: &str| {
        let c: Vec<f64> = s.split(',').map(num).collect();
        Vec3::new(c[0], c[1], c[2])
    };
    let p3 = |s: &str| Point3::origin() + v3(s);
    let eps = num(f[0]);
    let band = Band::new(eps, 10.0 * eps).unwrap();
    let (center, axis, major, minor, u_ref) = (p3(f[4]), v3(f[5]), num(f[6]), num(f[7]), v3(f[8]));
    let e = if f[2] == "circle" {
        geom::Curve3::Circle { center, axis, radius: major, u_ref }
    } else {
        geom::Curve3::Ellipse { center, axis, major, minor, u_ref }
    };
    let x = Vec3::new(1.0, 0.0, 0.0);
    let s = match f[18] {
        "S" => geom::Surface::Sphere { center: p3(f[19]), radius: num(f[20]), axis, u_ref: (x - axis * x.dot(axis)).normalize() },
        "C" => geom::Surface::Cylinder { origin: p3(f[19]), axis: v3(f[20]), radius: num(f[21]), u_ref: (x - v3(f[20]) * x.dot(v3(f[20]))).normalize() },
        _ => panic!("sphere or cylinder rows only"),
    };
    let conic = geom_brep::Conic::of(&e).unwrap();
    let (t0, t1) = (num(f[9]), num(f[10]));
    println!("extremes {:?}", geom_brep::conic_residual_extremes(&s, &conic));
    println!("arc range {:?}", geom_brep::conic_arc_residual_range(&s, &conic, t0, t1));
    println!("rung {:?}", rung(&s, &conic, t0, t1, band));
}

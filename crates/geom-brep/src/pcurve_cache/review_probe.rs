//! Review probes for PR 4227 (local only).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use super::{
    FocalImage, Pcurve, PcurveCache, PcurveCertifyError, carrier_harmonic, chart_pcurve,
    focal_section_envelope,
};
use geom::{Curve3, Surface};
use geom_core::predicate::Band;
use geom_core::tolerance::Tol;
use geom_core::{Point3, Vec3};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.unit()
    }
    fn sign(&mut self) -> f64 {
        if self.next() & 1 == 0 { 1.0 } else { -1.0 }
    }
    fn dir(&mut self) -> Vec3<f64> {
        loop {
            let v = Vec3::new(self.range(-1., 1.), self.range(-1., 1.), self.range(-1., 1.));
            if v.norm() > 0.2 {
                return v.normalize();
            }
        }
    }
}

fn villarceau(s: &mut Rng, major: f64, minor: f64) -> (Surface<f64>, Curve3<f64>) {
    let axis = s.dir();
    let u_ref = axis.cross(s.dir()).normalize();
    let center = Point3::new(s.range(-3., 3.), s.range(-3., 3.), s.range(-3., 3.));
    let phi = s.range(-PI, PI);
    let d = u_ref * phi.cos() + axis.cross(u_ref) * phi.sin();
    let tilt = (minor / major).asin();
    let lean = d.cross(axis) * tilt.cos() + axis * (s.sign() * tilt.sin());
    let psi = s.range(-PI, PI);
    let side = s.sign();
    (
        Surface::Torus { center, axis, major_radius: major, minor_radius: minor, u_ref },
        Curve3::Circle {
            center: center + d * (minor * side),
            axis: d.cross(lean) * s.sign(),
            radius: major,
            u_ref: d * psi.cos() + lean * psi.sin(),
        },
    )
}

fn cone_section(s: &mut Rng) -> Option<(Surface<f64>, Curve3<f64>)> {
    let axis = s.dir();
    let u_ref = axis.cross(s.dir()).normalize();
    let apex = Point3::new(s.range(-3., 3.), s.range(-3., 3.), s.range(-3., 3.));
    let half_angle = s.range(0.15, 1.3);
    let cone = Surface::Cone { apex, axis, half_angle, u_ref };
    let tilt = s.range(0.02, 0.8) * (FRAC_PI_2 - half_angle);
    let side = axis.cross(u_ref);
    let normal = (axis * tilt.cos() + (u_ref * s.range(-1., 1.) + side).normalize() * tilt.sin())
        .normalize();
    let plane = Surface::Plane {
        origin: apex + axis * (s.range(0.5, 3.0) * s.sign()),
        normal,
        u_ref: s.dir().cross(normal).normalize(),
    };
    match crate::intersect::plane_cone_section(&plane, &cone, 4.0, band()) {
        Ok(crate::intersect::PlaneConeSection::TiltedEllipse(e)) => Some((cone, e)),
        _ => None,
    }
}

fn sup(p: &Pcurve<f64>, s: &Surface<f64>, c: &Curve3<f64>, t0: f64, t1: f64, n: u32) -> (f64, f64) {
    let mut best = (0.0, t0);
    for k in 0..=n {
        let t = t0 + (t1 - t0) * (f64::from(k) / f64::from(n));
        let q = p.eval(t);
        let d = s.eval(q.x, q.y).distance(c.eval(t));
        if d > best.0 {
            best = (d, t);
        }
    }
    best
}

/// P1: exactness and certification over scale extremes.
#[test]
fn probe_villarceau_exact_over_scales() {
    let mut s = Rng(0x9E37_79B9_7F4A_7C15);
    let mut refusals = Vec::new();
    let mut worst_rel = 0.0f64;
    for trial in 0..3000 {
        let major = 10f64.powf(s.range(-2.0, 3.0));
        let ratio = if trial % 5 == 0 { s.range(0.9, 0.995) } else { s.range(0.005, 0.9) };
        let minor = major * ratio;
        let (torus, c) = villarceau(&mut s, major, minor);
        match chart_pcurve(&c, &torus, band()) {
            Ok(p) => {
                let (r, _) = sup(&p, &torus, &c, -7.0, 7.0, 4096);
                worst_rel = worst_rel.max(r / (major + 4.0));
                let t0 = s.range(-20.0, 20.0);
                let t1 = t0 + s.range(0.1, TAU);
                if let Err(e) = PcurveCache::certify(p.clone(), t0, t1, &c, &torus, band()) {
                    refusals.push(format!("certify R={major:e} r/R={ratio} [{t0},{t1}]: {e}"));
                }
            }
            Err(e) => refusals.push(format!("derive R={major:e} r/R={ratio}: {e:?}")),
        }
    }
    println!("[probe P1] worst residual/scale {worst_rel:e}; {} refusals", refusals.len());
    for r in refusals.iter().take(30) {
        println!("  {r}");
    }
}

/// P2: lemma under large perturbations of every field, both instances.
#[test]
fn probe_focal_lemma_large_perturbations() {
    let mut s = Rng(0xD1B5_4A32_D192_ED03);
    let mut violations = Vec::new();
    let mut checked = [0usize; 2];
    for trial in 0..6000 {
        let torus = trial % 2 == 0;
        let Some((chart, exact)) = (if torus {
            let major = s.range(0.5, 3.0);
            let minor = major * s.range(0.05, 0.9);
            Some(villarceau(&mut s, major, minor))
        } else {
            cone_section(&mut s)
        }) else {
            continue;
        };
        let Ok(derived) = chart_pcurve(&exact, &chart, band()) else { continue };
        let im = FocalImage::of(&derived).unwrap();
        let mut f = [im.u0, im.t0, im.v0, im.va, im.vb, im.vl, im.beta, im.sense];
        let mode = s.next() % 4;
        let big = [1e-6, 1e-3, 0.1, 1.0][mode as usize];
        let pick = s.next() % 9;
        for (i, x) in f.iter_mut().enumerate() {
            if pick == 8 || pick as usize == i {
                *x += big * s.range(-1.0, 1.0);
            }
        }
        if s.next() % 5 == 0 {
            f[7] = -f[7];
        }
        if s.next() % 5 == 0 {
            f[5] = -f[5];
        }
        if f[6].abs() >= 0.97 {
            f[6] = 0.97 * f[6].signum();
        }
        let stored = Pcurve::FocalSection {
            u0: f[0], t0: f[1], v0: f[2], va: f[3], vb: f[4], vl: f[5], beta: f[6], sense: f[7],
        };
        let si = FocalImage::of(&stored).unwrap();
        let t0 = s.range(-25.0, 25.0);
        let t1 = t0 + s.range(0.05, 2.0 * TAU);
        let reach = t0.abs().max(t1.abs());
        let v_sup = stored.chart_box(t0, t1).v_reach();
        let form = carrier_harmonic(&exact).unwrap();
        let env = focal_section_envelope(&si, form, &chart, v_sup, reach);
        let (sp, at) = sup(&stored, &chart, &exact, t0, t1, 8192);
        if env < sp * (1.0 - 1e-9) - 1e-12 {
            violations.push(format!(
                "trial {trial} torus={torus} mode={mode} pick={pick} env={env:e} sup={sp:e} at t={at} span [{t0},{t1}] fields {f:?}"
            ));
        }
        checked[usize::from(torus)] += 1;
    }
    println!("[probe P2] cone {} torus {} checked; {} violations", checked[0], checked[1], violations.len());
    for v in violations.iter().take(20) {
        println!("  {v}");
    }
    assert!(violations.is_empty());
}

/// P3: grazers and gates. Circles near Villarceau and parallels.
#[test]
fn probe_grazers() {
    let (major, minor) = (2.0, 0.7);
    let mut s = Rng(42);
    let k = Tol::witness().k() * Tol::witness().eps();
    println!("[probe P3] K·eps = {k:e}, band zero {:e}, escalate {:e}", band().zero(), band().escalate());
    for &delta in &[0.0, 0.25, 0.5, 1.0, 2.0, 4.0, 10.0, 100.0, 1e4] {
        let (torus, c) = villarceau(&mut s, major, minor);
        let Curve3::Circle { center, axis, radius, u_ref } = c else { unreachable!() };
        let moves: [(&str, Curve3<f64>); 4] = [
            ("radius", Curve3::Circle { center, axis, radius: radius + delta * k, u_ref }),
            ("centre-along-n", Curve3::Circle { center: center + axis * (delta * k), axis, radius, u_ref }),
            ("tilt", {
                let n = (axis + u_ref * (delta * k / major)).normalize();
                let u = (u_ref - n * u_ref.dot(n)).normalize();
                Curve3::Circle { center, axis: n, radius, u_ref: u }
            }),
            ("centre-in-plane", {
                let w = axis.cross(u_ref);
                Curve3::Circle { center: center + w * (delta * k), axis, radius, u_ref }
            }),
        ];
        for (what, m) in moves {
            let got = chart_pcurve(&m, &torus, band());
            let off = (0..512)
                .map(|i| {
                    let Surface::Torus { center: tc, axis: ta, .. } = torus else { unreachable!() };
                    let q = m.eval(f64::from(i) * TAU / 512.0) - tc;
                    let h = q.dot(ta);
                    let rho = (q - ta * h).norm();
                    ((rho - major).hypot(h) - minor).abs()
                })
                .fold(0.0, f64::max);
            let tag = match &got {
                Ok(p) => {
                    let cert = PcurveCache::certify(p.clone(), 0.0, TAU - 1e-9, &m, &torus, band());
                    format!("Ok -> certify {:?}", cert.as_ref().map(|c| c.certificate().envelope).map_err(|e| format!("{e}").chars().take(120).collect::<String>()))
                }
                Err(PcurveCertifyError::CarrierGrazesChart { .. }) => "Grazes".into(),
                Err(PcurveCertifyError::CarrierOffChart { .. }) => "OffChart".into(),
                Err(e) => format!("{e:?}").chars().take(100).collect(),
            };
            println!("  δ={delta:>6} {what:>16}: off-torus {off:.3e} → {tag}");
        }
    }
    // Crest parallels centred off axis.
    for &delta in &[0.5, 1.0, 2.0, 1e2, 1e3, 1e4, 1e5] {
        let torus = Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: major, minor_radius: minor, u_ref: Vec3::unit_x() };
        let c = Curve3::Circle { center: Point3::new(delta * k, 0.0, minor), axis: Vec3::unit_z(), radius: major, u_ref: Vec3::unit_x() };
        let off = (0..512).map(|i| { let q = c.eval(f64::from(i) * TAU / 512.0); let rho = (q.x*q.x+q.y*q.y).sqrt(); ((rho - major).hypot(q.z) - minor).abs() }).fold(0.0, f64::max);
        let got = chart_pcurve(&c, &torus, band());
        println!("  crest δ={delta}·Kε off {off:.3e}: {:?}", got.map(|_| "Ok").map_err(|e| format!("{e:?}").chars().take(60).collect::<String>()));
    }
}

/// P4: in-band circles decided definitively off by the one-sided incidence margin.
#[test]
fn probe_incidence_one_sided() {
    let mut s = Rng(7);
    for &ratio in &[0.1, 0.35, 0.6, 0.9] {
        for &dist in &[2e-10, 5e-10, 9e-10, 3e-9] {
            let (major, minor) = (2.0, 2.0 * ratio);
            let (torus, c) = villarceau(&mut s, major, minor);
            let Curve3::Circle { center, axis, radius, u_ref } = c else { unreachable!() };
            // Move the centre along the torus axis: every point moves by `dist` axially.
            let Surface::Torus { axis: ta, center: tc, .. } = torus else { unreachable!() };
            let m = Curve3::Circle { center: center + ta * dist, axis, radius, u_ref };
            let off = (0..4096)
                .map(|i| {
                    let q = m.eval(f64::from(i) * TAU / 4096.0) - tc;
                    let h = q.dot(ta);
                    let rho = (q - ta * h).norm();
                    ((rho - major).hypot(h) - minor).abs()
                })
                .fold(0.0, f64::max);
            let got = chart_pcurve(&m, &torus, band());
            println!(
                "[probe P4] r/R={ratio} moved {dist:e}: true max distance {off:.3e} → {}",
                match got {
                    Ok(_) => "Ok".to_string(),
                    Err(e) => format!("{e:?}").chars().take(70).collect(),
                }
            );
        }
    }
}

/// P5: the cone remainder's t0 rotation is load-bearing (carrier = H, t0 = π/2).
#[test]
fn probe_cone_remainder_t0_rotation() {
    let alpha = 0.4_f64;
    let (axis, u_ref) = (Vec3::unit_z(), Vec3::unit_x());
    let surface = Surface::Cone { apex: Point3::origin(), axis, half_angle: alpha, u_ref };
    let (beta, v0, t0) = (0.3_f64, 2.0_f64, FRAC_PI_2);
    let bb = beta * beta;
    let (e, q) = (2.0 * beta / (1.0 + bb), (1.0 - bb) / (1.0 + bb));
    let (va, vb) = (-e * v0, 0.0);
    let stored = Pcurve::FocalSection { u0: 0.0, t0, v0, va, vb, vl: 0.0, beta, sense: 1.0 };
    let (sa, ca) = alpha.sin_cos();
    let d0 = u_ref;
    let d1 = axis.cross(u_ref);
    let lever = sa * v0;
    let (x, y) = (d0 * lever, d1 * (lever * q));
    let k = Point3::origin() + axis * (ca * v0) - d0 * (lever * e);
    let (ct, st) = (t0.cos(), t0.sin());
    let h_a = axis * (ca * va) + x * ct - y * st;
    let h_b = axis * (ca * vb) + x * st + y * ct;
    assert!(h_a.norm() >= h_b.norm());
    let n = h_a.cross(h_b).normalize();
    let carrier = Curve3::Ellipse { center: k, axis: n, major: h_a.norm(), minor: h_b.norm(), u_ref: h_a.normalize() };
    let (s0, s1) = (-3.0, 3.0);
    let env = focal_section_envelope(&FocalImage::of(&stored).unwrap(), carrier_harmonic(&carrier).unwrap(), &surface, stored.chart_box(s0, s1).v_reach(), 3.0);
    let (sp, _) = sup(&stored, &surface, &carrier, s0, s1, 8192);
    println!("[probe P5] envelope {env:e} sampled sup {sp:e}");
    assert!(env >= sp * (1.0 - 1e-9), "envelope under the residual");
}

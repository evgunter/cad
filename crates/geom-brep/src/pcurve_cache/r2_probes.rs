//! Review 4842 R2 probes (not for merge).
use core::f64::consts::{FRAC_1_SQRT_2, PI};
use geom_core::{Interval, Tol};
use super::*;

fn band() -> Band { Band::linear(Tol::witness()).unwrap() }
fn zero() -> f64 { band().zero() }

fn circle(center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>) -> Curve3<f64> {
    Curve3::Circle { center, axis, radius, u_ref }
}
fn tilted_circle(center: Point3<f64>, radius: f64, phi: f64) -> Curve3<f64> {
    let k = Vec3::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.0);
    let (s, c) = phi.sin_cos();
    let turn = |v: Vec3<f64>| v * c + k.cross(v) * s + k * (k.dot(v) * (1.0 - c));
    circle(center, turn(Vec3::unit_z()), radius, turn(Vec3::unit_x()))
}
fn sampled(img: &Pcurve<f64>, s: &Surface<f64>, c: &Curve3<f64>, t0: f64, t1: f64) -> f64 {
    (0..=8192).map(|i| { let t = t0 + (t1 - t0) * f64::from(i) / 8192.0; let q = img.eval(t);
        s.eval(q.x, q.y).distance(c.eval(t)) }).fold(0.0, f64::max)
}

/// P1: radial (radius gap G) and axial (tilt T) halves of one pair, read
/// in separate slots and summed: G + T against √(G² + T²).
#[test]
fn p1_radius_gap_plus_tilt_split() {
    let sphere = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let cone = Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_z(), half_angle: 0.45, u_ref: Vec3::unit_x() };
    for frac in [0.55, 0.6, 0.65] {
      for (what, s, cz, rad) in [("sphere", &sphere, 0.6, 0.8), ("cone", &cone, 1.0, 0.45f64.tan())] {
        let (g, t) = (frac * zero(), frac * zero());
        let center = Point3::new(0.0, 0.0, cz);
        let flat = circle(center, Vec3::unit_z(), rad, Vec3::unit_x());
        let image = chart_pcurve(&flat, s, band()).unwrap();
        let carrier = tilted_circle(center, rad + g, (t / (rad + g)).asin());
        let sup = sampled(&image, s, &carrier, 0.0, PI);
        let r = PcurveCache::certify(image.clone(), 0.0, PI, &carrier, s, band());
        let derived = chart_pcurve(&carrier, s, band());
        let rd = derived.as_ref().ok().map(|d| PcurveCache::certify(d.clone(), 0.0, PI, &carrier, s, band()).map(|c| c.certificate().envelope / zero()));
        eprintln!("P1 {what} frac {frac}: sampled/zero {:.4}; stored-flat: {:?}; derived-from-carrier: {:?}",
          sup / zero(), r.as_ref().map(|c| c.certificate().envelope / zero()).map_err(|e| format!("{e:?}").chars().take(160).collect::<String>()),
          rd.map(|x| x.map_err(|e| format!("{e:?}").chars().take(160).collect::<String>())));
      }
    }
}

/// P1b: cone rim winding Zero, a small circle about the axis tilted 45°:
/// Drift σ(a_r,b_r)=d plus Tilt d·sin φ against the exact d.
#[test]
fn p1b_still_winding_tilted() {
    let cone = Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_z(), half_angle: 0.01, u_ref: Vec3::unit_x() };
    for frac in [0.5, 0.6, 0.7] {
        let d = frac * zero();
        let h = 5.0 * zero();
        let c = Point3::new(0.0, 0.0, h);
        let phi = core::f64::consts::FRAC_PI_4;
        let carrier = Curve3::Circle { center: c, axis: Vec3::new(0.0, -phi.sin(), phi.cos()), radius: d, u_ref: Vec3::unit_x() };
        let span = (0.0, 2.0 * band().escalate() / d);
        let image = chart_pcurve(&carrier, &cone, band());
        let res = image.as_ref().map(|im| (sampled(im, &cone, &carrier, span.0, span.1) / zero(),
            PcurveCache::certify(im.clone(), span.0, span.1, &carrier, &cone, band()).map(|c| c.certificate().envelope / zero()).map_err(|e| format!("{e:?}").chars().take(200).collect::<String>())));
        eprintln!("P1b frac {frac}: {:?}", res.map_err(|e| format!("{e:?}").chars().take(200).collect::<String>()));
    }
}

/// P4: class-gate movement past the band: tilt σ in [esc/√2, esc).
#[test]
fn p4_gate_region_b() {
    let sphere = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let cone = Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_z(), half_angle: 0.45, u_ref: Vec3::unit_x() };
    let torus = Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: 1.0, minor_radius: 0.25, u_ref: Vec3::unit_x() };
    for frac in [0.5, 0.8, 0.95, 1.2] {
        for (what, s, cz, rad) in [("sphere", &sphere, 0.6, 0.8), ("cone", &cone, 1.0, 0.45f64.tan()), ("torus", &torus, 0.0, 1.25)] {
            let off = frac * band().escalate();
            let carrier = tilted_circle(Point3::new(0.0, 0.0, cz), rad, (off / rad).asin());
            let r = chart_pcurve(&carrier, s, band());
            eprintln!("P4 {what} tilt/esc {frac}: {}", match r { Ok(p) => format!("Ok {:?}", p).chars().take(80).collect::<String>(), Err(e) => format!("Err {e:?}").chars().take(140).collect() });
        }
    }
}

/// P3: does the row's "straddling" fuzz straddle?
#[test]
fn p3_fuzz_straddles() {
    let fuzz = Interval::from_f64(1e-17) - Interval::from_f64(1e-17 * (1.0 + f64::EPSILON));
    eprintln!("P3 fuzz = {fuzz:?}");
    let s = Interval::from_bounds(-1e-20, 1e-20);
    let v = Vec3::new(s, s, s);
    let h = harmonic_sup(v, v);
    eprintln!("P3 straddle sup = {h:?} decide {:?}", decide("p3", Margin::of(h), band()));
}

/// P2: adversarial numerics, f64 vs Interval vs a 4096-bit-free reference
/// (sampled in f64 with a scaled pair).
#[test]
fn p2_numerics() {
    let cases: Vec<(Vec3<f64>, Vec3<f64>)> = vec![
        (Vec3::new(1e-170, 0.0, 0.0), Vec3::new(0.0, 1e-170, 0.0)),
        (Vec3::new(1e154, 0.0, 0.0), Vec3::new(0.0, 1e154, 0.0)),
        (Vec3::new(1.0, 1e-9, 0.0), Vec3::new(-1.0, 1e-9, 0.0)),
        (Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0 + 1e-15, 2.0, 3.0)),
        (Vec3::new(1.0, 0.0, 0.0), Vec3::new(1e-8, 1.0 - 1e-16, 0.0)),
    ];
    for (a, b) in cases {
        let f = harmonic_sup(a, b);
        let iv = |v: Vec3<f64>| Vec3::new(Interval::from_f64(v.x), Interval::from_f64(v.y), Interval::from_f64(v.z));
        let i = harmonic_sup(iv(a), iv(b));
        // reference: scale to unit, sample densely + refine
        let s = a.norm().max(b.norm());
        let (au, bu) = (a / s, b / s);
        let mut best = 0.0f64; let mut bt = 0.0;
        for k in 0..=200000 { let t = f64::from(k) * core::f64::consts::TAU / 200000.0; let v = (au * t.cos() + bu * t.sin()).norm(); if v > best { best = v; bt = t; } }
        for k in -2000..=2000 { let t = bt + f64::from(k) * 1e-9; let v = (au * t.cos() + bu * t.sin()).norm(); if v > best { best = v; } }
        eprintln!("P2 a {a:?} b {b:?}: f64 {f:e} interval {i:?} ref {:e} f64/ref-1 {:e}", best * s, f / (best * s) - 1.0);
    }
}

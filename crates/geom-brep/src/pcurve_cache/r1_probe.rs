//! Review 4952 R1 probe: incidence (and whole-envelope) domination at a
//! wide band, so algebra slips show far above rounding.
use super::*;
use core::f64::consts::TAU;
use geom_core::{Bounds, Interval};

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn r(&mut self, lo: f64, hi: f64) -> f64 { lo + (hi - lo) * self.next() }
    fn v(&mut self, s: f64) -> Vec3<f64> { Vec3::new(self.r(-s, s), self.r(-s, s), self.r(-s, s)) }
}

fn wide() -> Band { Band::new(1e-3, 3e-3).unwrap() }

fn residual(image: &Pcurve<f64>, carrier: &Curve3<f64>, s: &Surface<f64>) -> f64 {
    (0..=20000).map(|i| {
        let t = TAU * f64::from(i) / 20000.0;
        let q = image.eval(t);
        s.eval(q.x, q.y).distance(carrier.eval(t))
    }).fold(0.0, f64::max)
}

fn perp(n: Vec3<f64>) -> Vec3<f64> {
    let x = if n.x.abs() < 0.9 { Vec3::unit_x() } else { Vec3::unit_y() };
    (x - n * n.dot(x)).normalize()
}

fn conic(rng: &mut Lcg, center: Point3<f64>, normal: Vec3<f64>, r: f64, d: f64, ellipse: bool) -> Curve3<f64> {
    let n = (normal + rng.v(d)).normalize();
    let c = center + rng.v(d);
    let ang = rng.r(0.0, TAU);
    let p = perp(n);
    let q = n.cross(p);
    let u_ref = p * ang.cos() + q * ang.sin();
    if ellipse {
        let (a, b) = (r + rng.r(0.0, d), r - rng.r(0.0, d));
        Curve3::Ellipse { center: c, axis: n, major: a, minor: b, u_ref }
    } else {
        Curve3::Circle { center: c, axis: n, radius: r + rng.r(-d, d), u_ref }
    }
}

fn run(name: &str, surface: &Surface<f64>, mut gen: impl FnMut(&mut Lcg) -> Curve3<f64>, want: fn(&Derivation) -> bool) {
    let mut rng = Lcg(0x4952_0001 ^ name.len() as u64 * 7919);
    let (mut n, mut worst_f, mut worst_i, mut tight) = (0, f64::INFINITY, f64::INFINITY, 0.0f64);
    let ideal = orthonormal_chart(surface);
    let f = Interval::from_f64;
    for _ in 0..4000 {
        let carrier = gen(&mut rng);
        let Ok((image, derivation)) = derive_harmonic(&carrier, &ideal, wide()) else { continue };
        if !want(&derivation) { continue; }
        let form = carrier_harmonic(&carrier).unwrap();
        let mut terms = EnvelopeTerms::new();
        incidence(form, &ideal, derivation, TAU, &mut terms);
        let inc = terms.total();
        let res = residual(&image, &carrier, &ideal);
        // Interval
        let ci = carrier.map_scalar(f);
        let si = ideal.map_scalar(f);
        let Ok((_, di)) = derive_harmonic(&ci, &si, wide()) else { continue };
        let mut ti = EnvelopeTerms::new();
        incidence(carrier_harmonic(&ci).unwrap(), &si, di, f(TAU), &mut ti);
        let inc_i = ti.total().hi();
        n += 1;
        worst_f = worst_f.min(inc - res);
        worst_i = worst_i.min(inc_i - res);
        if res > 1e-6 { tight = tight.max(res / inc); }
        assert!(inc >= res - 1e-12, "{name}: f64 incidence {inc:e} < residual {res:e} ({derivation:?}) {carrier:?}");
        assert!(inc_i >= res - 1e-12, "{name}: Interval incidence {inc_i:e} < residual {res:e} ({derivation:?}) {carrier:?}");
    }
    eprintln!("R1PROBE {name}: n={n} min(inc-res) f64={worst_f:e} iv={worst_i:e} max res/inc={tight:.4}");
    assert!(n > 50, "{name}: only {n} rows reached the class");
}

const D: f64 = 4e-4;

#[test]
fn r1_cylinder_moving() {
    let s = Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() };
    run("cyl moving ellipse", &s, |g| { let h = g.r(-1.0, 1.0); let e = g.next() < 0.6; conic(g, Point3::new(0.0, 0.0, h), Vec3::unit_z(), 1.0, D, e) },
        |d| matches!(d, Derivation::CylinderMoving { beta: Winding::Pos | Winding::Neg }));
}

#[test]
fn r1_cone_rim() {
    let a = 0.5f64;
    let s = Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_z(), half_angle: a, u_ref: Vec3::unit_x() };
    run("cone rim", &s, |g| { let h = g.r(0.5, 2.0) * if g.next() < 0.5 { -1.0 } else { 1.0 }; let e = g.next() < 0.6; conic(g, Point3::new(0.0, 0.0, h), Vec3::unit_z(), h.abs() * a.tan(), D, e) },
        |d| matches!(d, Derivation::ConeRim { beta: Winding::Pos | Winding::Neg, .. }));
}

#[test]
fn r1_sphere_parallel() {
    let s = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    run("sphere parallel", &s, |g| { let h = g.r(-0.95, 0.95); conic(g, Point3::new(0.0, 0.0, h), Vec3::unit_z(), (1.0 - h * h).sqrt(), D, false) },
        |d| matches!(d, Derivation::SphereParallel { beta: Winding::Pos | Winding::Neg }));
}

#[test]
fn r1_sphere_meridian() {
    let s = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    run("sphere meridian", &s, |g| { let az = g.r(0.0, TAU); conic(g, Point3::origin(), Vec3::new(-az.sin(), az.cos(), 0.0), 1.0, D, false) },
        |d| matches!(d, Derivation::SphereMeridian { pole: false, .. }));
}

#[test]
fn r1_sphere_pole_start() {
    let s = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    run("sphere pole start", &s, |g| {
        let az = g.r(0.0, TAU);
        let n = (Vec3::new(-az.sin(), az.cos(), 0.0) + g.v(D)).normalize();
        let u_ref = (Vec3::unit_z() * if g.next() < 0.5 { 1.0 } else { -1.0 } + g.v(D * 0.5));
        let u_ref = (u_ref - n * n.dot(u_ref)).normalize();
        Curve3::Circle { center: Point3::origin() + g.v(D), axis: n, radius: 1.0 + g.r(-D, D), u_ref }
    }, |d| matches!(d, Derivation::SphereMeridian { pole: true, .. }));
}

#[test]
fn r1_torus_parallel() {
    let (big, r) = (2.0, 0.5);
    let s = Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: big, minor_radius: r, u_ref: Vec3::unit_x() };
    run("torus parallel", &s, |g| { let v = g.r(0.0, TAU); conic(g, Point3::new(0.0, 0.0, r * v.sin()), Vec3::unit_z(), big + r * v.cos(), D, false) },
        |d| matches!(d, Derivation::TorusParallel { beta: Winding::Pos | Winding::Neg }));
}

#[test]
fn r1_torus_meridian() {
    let (big, r) = (2.0, 0.5);
    let s = Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: big, minor_radius: r, u_ref: Vec3::unit_x() };
    run("torus meridian", &s, |g| { let az = g.r(0.0, TAU); conic(g, Point3::new(big * az.cos(), big * az.sin(), 0.0), Vec3::new(-az.sin(), az.cos(), 0.0), r, D, false) },
        |d| matches!(d, Derivation::TorusMeridian { .. }));
}

/// Whole periodic envelope with a stored image displaced in both
/// channels: fidelity hypot domination on cylinder / sphere / torus,
/// including sphere near the pole and torus inner equator.
#[test]
fn r1_fidelity_hypot() {
    let mut g = Lcg(77);
    let f = Interval::from_f64;
    let cases: Vec<(&str, Surface<f64>, Box<dyn Fn(&mut Lcg) -> Curve3<f64>>)> = vec![
        ("cyl", Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() },
            Box::new(|g: &mut Lcg| { let h = g.r(-1.0, 1.0); Curve3::Circle { center: Point3::new(0.0, 0.0, h), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() } })),
        ("sphere near pole", Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() },
            Box::new(|g: &mut Lcg| { let h: f64 = g.r(0.97, 0.999); Curve3::Circle { center: Point3::new(0.0, 0.0, h), axis: Vec3::unit_z(), radius: (1.0 - h * h).sqrt(), u_ref: Vec3::unit_x() } })),
        ("sphere meridian", Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() },
            Box::new(|g: &mut Lcg| { let az = g.r(0.0, TAU); Curve3::Circle { center: Point3::origin(), axis: Vec3::new(-az.sin(), az.cos(), 0.0), radius: 1.0, u_ref: Vec3::new(az.cos(), az.sin(), 0.0) } })),
        ("torus inner", Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: 2.0, minor_radius: 0.5, u_ref: Vec3::unit_x() },
            Box::new(|g: &mut Lcg| { let v = core::f64::consts::PI + g.r(-0.3, 0.3); Curve3::Circle { center: Point3::new(0.0, 0.0, 0.5 * v.sin()), axis: Vec3::unit_z(), radius: 2.0 + 0.5 * v.cos(), u_ref: Vec3::unit_x() } })),
        ("torus meridian", Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: 2.0, minor_radius: 0.5, u_ref: Vec3::unit_x() },
            Box::new(|g: &mut Lcg| { let az = g.r(0.0, TAU); Curve3::Circle { center: Point3::new(2.0 * az.cos(), 2.0 * az.sin(), 0.0), axis: Vec3::new(-az.sin(), az.cos(), 0.0), radius: 0.5, u_ref: Vec3::new(az.cos(), az.sin(), 0.0) } })),
    ];
    for (name, s, gen) in cases {
        let mut n = 0; let mut tight = 0.0f64;
        for _ in 0..400 {
            let carrier = gen(&mut g);
            let Ok(Pcurve::Harmonic { p0, pa, pb, pl }) = chart_pcurve(&carrier, &s, wide()) else { continue };
            let (du, dv) = (g.r(-3e-4, 3e-4), g.r(-3e-4, 3e-4));
            let (ea, eb) = (Vec2::new(g.r(-1e-4, 1e-4), g.r(-1e-4, 1e-4)), Vec2::new(g.r(-1e-4, 1e-4), g.r(-1e-4, 1e-4)));
            let stored = Pcurve::Harmonic { p0: Point2::new(p0.x + du, p0.y + dv), pa: pa + ea, pb: pb + eb, pl };
            let res = residual(&stored, &carrier, &s);
            let form = carrier_harmonic(&carrier).unwrap();
            let Ok(terms) = periodic_envelope(&stored, &carrier, form, &s, (0.0, TAU), TAU, wide()) else { continue };
            assert!(terms.1, "{name}: flag set");
            let env = terms.total();
            let lift = |v: Vec2<f64>| Vec2::new(f(v.x), f(v.y));
            let Pcurve::Harmonic { p0: q0, pa: qa, pb: qb, pl: ql } = stored else { unreachable!() };
            let stored_i = Pcurve::Harmonic { p0: Point2::new(f(q0.x), f(q0.y)), pa: lift(qa), pb: lift(qb), pl: lift(ql) };
            let ci = carrier.map_scalar(f);
            let Ok(ti) = periodic_envelope(&stored_i, &ci, carrier_harmonic(&ci).unwrap(), &s.map_scalar(f), (f(0.0), f(TAU)), f(TAU), wide()) else { continue };
            let env_i = ti.total().hi();
            n += 1; tight = tight.max(res / env);
            assert!(env >= res && env_i >= res, "{name}: envelope {env:e}/{env_i:e} < residual {res:e}");
        }
        eprintln!("R1PROBE fidelity {name}: n={n} max res/env={tight:.4}");
        assert!(n > 50);
    }
}

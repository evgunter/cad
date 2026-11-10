//! Review probe for PR 6524 (not for merge): an independent search of
//! the ellipse lane's envelope, shaped apart from the PR's, dumped for a
//! 50-digit residual oracle.

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::{Bounds, Interval};
use test_utils::fuzz;

use super::harmonic_sup_rows::{band, sampled_residual};
use super::*;

fn frame_of(surface: &Surface<f64>) -> (Point3<f64>, [Vec3<f64>; 3]) {
    projected::chart_frame(surface).unwrap()
}
fn w(s: &Surface<f64>, p: Vec3<f64>) -> Point3<f64> {
    let (o, e) = frame_of(s);
    o + e[0] * p.x + e[1] * p.y + e[2] * p.z
}
fn wd(s: &Surface<f64>, v: Vec3<f64>) -> Vec3<f64> {
    let (_, e) = frame_of(s);
    e[0] * v.x + e[1] * v.y + e[2] * v.z
}
fn unit(s: &mut fuzz::Rng) -> Vec3<f64> {
    Vec3::new(s.range(-1.0, 1.0), s.range(-1.0, 1.0), s.range(-1.0, 1.0)).normalize()
}
/// chart-frame ellipse c + a u cos + b v sin; perturbed second semi-axis
/// by `squash`, offset by `off`, sign flips.
fn ell(
    s: &mut fuzz::Rng,
    surface: &Surface<f64>,
    c: Vec3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    a: f64,
    b: f64,
) -> Curve3<f64> {
    let (u, v) = (wd(surface, u).normalize(), wd(surface, v).normalize());
    let mut v = (v - u * u.dot(v)).normalize();
    // rotate the in-plane pair by a random angle half the time (an
    // ellipse's axes are then not along u, v: re-pick by rotating both)
    let (mut a, mut b) = (a, b);
    if s.below(4) == 0 {
        core::mem::swap(&mut a, &mut b);
        let t = u;
        // keep orientation: (u, v) -> (v, -u)
        let nu = v;
        v = t * -1.0;
        return finish(s, surface, c, nu, v, a, b);
    }
    finish(s, surface, c, u, v, a, b)
}
fn finish(
    s: &mut fuzz::Rng,
    surface: &Surface<f64>,
    c: Vec3<f64>,
    u: Vec3<f64>,
    v: Vec3<f64>,
    a: f64,
    b: f64,
) -> Curve3<f64> {
    let (a, b) = match s.below(6) {
        0 => (-a, b),
        1 => (a, -b),
        _ => (a, b),
    };
    Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(w(surface, c), u.cross(v).normalize(), u),
        a,
        b,
    ))
}

struct Probe {
    family: &'static str,
    surface: Surface<f64>,
    carrier: Curve3<f64>,
    span: (f64, f64),
}

fn draw(s: &mut fuzz::Rng) -> Probe {
    let scale = s.log_uniform(1e-6, 1e3);
    let far = match s.below(3) {
        0 => 0.0,
        1 => s.log_uniform(1.0, 1e3) * scale,
        _ => s.log_uniform(1.0, 1e5),
    };
    let place = Point3::new(far * 0.6, -0.8 * far, 0.1 * far);
    let axis = unit(s);
    let u_ref = axis.cross(unit(s)).normalize();
    let squash = |s: &mut fuzz::Rng| {
        if s.below(4) == 0 {
            1.0
        } else {
            1.0 - s.log_uniform(1e-14, 0.5)
        }
    };
    let offset = |s: &mut fuzz::Rng, scale: f64| -> Vec3<f64> {
        if s.below(3) == 0 {
            Vec3::zero()
        } else {
            unit(s) * (scale * s.log_uniform(1e-16, 1e-5))
        }
    };
    let span = if s.below(3) == 0 {
        (0.0, TAU)
    } else {
        let t0 = s.range(-PI, PI);
        (t0, t0 + s.log_uniform(1e-3, TAU))
    };
    let fam = s.below(11);
    let (family, surface, carrier) = match fam {
        // sphere near a pole: a small parallel squashed
        0 => {
            let r = scale;
            let surface = Surface::Sphere { center: place, radius: r, axis, u_ref };
            let th = s.log_uniform(1e-9, 1e-1);
            let pole = if s.below(2) == 0 { 1.0 } else { -1.0 };
            let phi = s.range(0.0, TAU);
            let u = Vec3::new(phi.cos(), phi.sin(), 0.0);
            let v = Vec3::new(-phi.sin(), phi.cos(), 0.0);
            let c = Vec3::new(0.0, 0.0, pole * r * th.cos()) + offset(s, scale);
            let k = squash(s);
            let carrier = ell(s, &surface, c, u, v, r * th.sin(), r * th.sin() * k);
            ("sphere-pole", surface, carrier)
        }
        // sphere: a circle of a random plane, squashed
        1 => {
            let r = scale;
            let surface = Surface::Sphere { center: place, radius: r, axis, u_ref };
            let n = unit(s);
            let th = s.log_uniform(1e-7, FRAC_PI_2);
            let u = n.cross(unit(s)).normalize();
            let v = n.cross(u);
            let c = n * (r * th.cos()) + offset(s, scale);
            let k = squash(s);
            let carrier = ell(s, &surface, c, u, v, r * th.sin(), r * th.sin() * k);
            ("sphere-plane", surface, carrier)
        }
        // sphere: a tangent-plane needle
        2 => {
            let r = scale;
            let surface = Surface::Sphere { center: place, radius: r, axis, u_ref };
            let n = unit(s);
            let u = n.cross(unit(s)).normalize();
            let v = n.cross(u);
            let a = r * s.log_uniform(1e-9, 1e-2);
            let b = a * s.log_uniform(1e-12, 1.0);
            let c = n * r + offset(s, scale);
            let carrier = ell(s, &surface, c, u, v, a, b);
            ("sphere-needle", surface, carrier)
        }
        // cylinder: oblique section of eccentricity near 1
        3 => {
            let r = scale;
            let surface = Surface::Cylinder { origin: place, axis, radius: r, u_ref };
            let tau = FRAC_PI_2 - s.log_uniform(1e-8, 1.5);
            let phi = s.range(0.0, TAU);
            let n = Vec3::new(tau.sin() * phi.cos(), tau.sin() * phi.sin(), tau.cos());
            let minor = Vec3::new(-phi.sin(), phi.cos(), 0.0);
            let major = n.cross(minor).normalize();
            let z0 = s.range(-2.0, 2.0) * scale;
            let c = Vec3::new(0.0, 0.0, z0) + offset(s, scale);
            let k = squash(s);
            let carrier = ell(s, &surface, c, minor, major, r * k, r / tau.cos());
            ("cyl-oblique", surface, carrier)
        }
        // cylinder: tangent-plane ellipse, any orientation in the plane
        4 => {
            let r = scale;
            let surface = Surface::Cylinder { origin: place, axis, radius: r, u_ref };
            let phi = s.range(0.0, TAU);
            let nr = Vec3::new(phi.cos(), phi.sin(), 0.0);
            let t = Vec3::new(-phi.sin(), phi.cos(), 0.0);
            let rot = s.range(0.0, PI);
            let u = t * rot.cos() + Vec3::unit_z() * rot.sin();
            let v = nr.cross(u);
            let a = r * s.log_uniform(1e-9, 1e-1);
            let b = a * s.log_uniform(1e-12, 1.0);
            let c = nr * r + Vec3::new(0.0, 0.0, s.range(-1.0, 1.0) * scale) + offset(s, scale);
            let carrier = ell(s, &surface, c, u, v, a, b);
            ("cyl-tangent", surface, carrier)
        }
        // cone: a near-apex section toward the parabola
        5 | 6 => {
            let alpha = if s.below(2) == 0 {
                s.log_uniform(1e-4, 1.0)
            } else {
                FRAC_PI_2 - s.log_uniform(1e-4, 0.6)
            };
            let surface = Surface::Cone { apex: place, axis, half_angle: alpha, u_ref };
            let h = scale * s.log_uniform(1e-10, 1.0) * if s.below(2) == 0 { 1.0 } else { -1.0 };
            let tmax = FRAC_PI_2 - alpha;
            let tau = tmax * (1.0 - s.log_uniform(1e-9, 1.0));
            let ta = alpha.tan();
            let am = tau.cos().powi(2) - tau.sin().powi(2) * ta * ta;
            let s0 = h * tau.sin() * ta * ta / am;
            let k = h * h * ta * ta + am * s0 * s0;
            let c = Vec3::new(0.0, s0 * tau.cos(), h + s0 * tau.sin()) + offset(s, scale);
            let kk = squash(s);
            let carrier = ell(
                s,
                &surface,
                c,
                Vec3::unit_x(),
                Vec3::new(0.0, tau.cos(), tau.sin()),
                k.sqrt() * kk,
                (k / am).sqrt(),
            );
            ("cone-section", surface, carrier)
        }
        // cone: a tangent-plane ellipse near the apex
        7 => {
            let alpha = s.log_uniform(1e-3, 1.5);
            let surface = Surface::Cone { apex: place, axis, half_angle: alpha, u_ref };
            let sl = scale * s.log_uniform(1e-8, 1.0) * if s.below(2) == 0 { 1.0 } else { -1.0 };
            let phi = s.range(0.0, TAU);
            let g = Vec3::new(alpha.sin() * phi.cos(), alpha.sin() * phi.sin(), alpha.cos());
            let t = Vec3::new(-phi.sin(), phi.cos(), 0.0);
            let rot = s.range(0.0, PI);
            let u = t * rot.cos() + g * rot.sin();
            let v = g.cross(t).cross(u) * -1.0;
            let a = sl.abs() * s.log_uniform(1e-9, 0.5);
            let b = a * s.log_uniform(1e-12, 1.0);
            let c = g * sl + offset(s, scale);
            let carrier = ell(s, &surface, c, u, v.normalize(), a, b);
            ("cone-tangent", surface, carrier)
        }
        // torus: a Villarceau circle squashed (across the inner equator)
        8 => {
            let big = scale;
            let rr = big * if s.below(3) == 0 { 1.0 - s.log_uniform(1e-9, 1e-2) } else { s.log_uniform(1e-4, 0.9) };
            let surface = Surface::Torus { center: place, axis, major_radius: big, minor_radius: rr, u_ref };
            let th = (rr / big).asin();
            let phi = s.range(0.0, TAU);
            let rotz = |p: Vec3<f64>| Vec3::new(p.x * phi.cos() - p.y * phi.sin(), p.x * phi.sin() + p.y * phi.cos(), p.z);
            let sg = if s.below(2) == 0 { 1.0 } else { -1.0 };
            let c = rotz(Vec3::new(0.0, sg * rr, 0.0)) + offset(s, scale);
            let u = rotz(Vec3::new(th.cos(), 0.0, th.sin()));
            let v = rotz(Vec3::unit_y());
            let k = squash(s);
            let carrier = ell(s, &surface, c, u, v, big, big * k);
            ("torus-villarceau", surface, carrier)
        }
        // torus: the inner equator squashed, or a parallel near it
        9 => {
            let big = scale;
            let rr = big * s.log_uniform(1e-4, 1.0 - 1e-6);
            let surface = Surface::Torus { center: place, axis, major_radius: big, minor_radius: rr, u_ref };
            let psi = PI + s.range(-1.0, 1.0) * s.log_uniform(1e-9, 1.0);
            let rho = big + rr * psi.cos();
            let z = rr * psi.sin();
            let phi = s.range(0.0, TAU);
            let u = Vec3::new(phi.cos(), phi.sin(), 0.0);
            let v = Vec3::new(-phi.sin(), phi.cos(), 0.0);
            let k = squash(s);
            let c = Vec3::new(0.0, 0.0, z) + offset(s, scale);
            let carrier = ell(s, &surface, c, u, v, rho, rho * k);
            ("torus-inner", surface, carrier)
        }
        // torus: a tangent-plane ellipse at a saddle point
        _ => {
            let big = scale;
            let rr = big * s.log_uniform(1e-3, 0.95);
            let surface = Surface::Torus { center: place, axis, major_radius: big, minor_radius: rr, u_ref };
            let psi = PI + s.range(-1.2, 1.2);
            let phi = s.range(0.0, TAU);
            let rh = Vec3::new(phi.cos(), phi.sin(), 0.0);
            let n = rh * psi.cos() + Vec3::unit_z() * psi.sin();
            let p = rh * big + n * rr;
            let t = Vec3::new(-phi.sin(), phi.cos(), 0.0);
            let m = n.cross(t);
            let rot = s.range(0.0, PI);
            let u = t * rot.cos() + m * rot.sin();
            let v = n.cross(u);
            let a = rr * s.log_uniform(1e-9, 0.3);
            let b = a * s.log_uniform(1e-12, 1.0);
            let c = p + offset(s, scale);
            let carrier = ell(s, &surface, c, u, v, a, b);
            ("torus-saddle", surface, carrier)
        }
    };
    Probe { family, surface, carrier, span }
}

fn row(p: &Probe, image: &projected::ProjectedImage<f64>, env: f64, lo: f64, hi: f64, sampled: f64) -> String {
    let v = |x: Vec3<f64>| format!("{:e} {:e} {:e}", x.x, x.y, x.z);
    let pt = |x: Point3<f64>| v(x - Point3::origin());
    let Curve3::Ellipse(e) = &p.carrier else { unreachable!() };
    let chart = match p.surface {
        Surface::Sphere { center, radius, axis, .. } => format!("sphere {} {} {radius:e} 0", pt(center), v(axis)),
        Surface::Cylinder { origin, axis, radius, .. } => format!("cylinder {} {} {radius:e} 0", pt(origin), v(axis)),
        Surface::Cone { apex, axis, half_angle, .. } => format!("cone {} {} {half_angle:e} 0", pt(apex), v(axis)),
        Surface::Torus { center, axis, major_radius, minor_radius, .. } => {
            format!("torus {} {} {major_radius:e} {minor_radius:e}", pt(center), v(axis))
        }
        _ => unreachable!(),
    };
    let nappe = match image.chart.nappe() {
        Some(crate::offset::Nappe::Opening) => 1,
        Some(crate::offset::Nappe::Mirror) => -1,
        None => 0,
    };
    format!(
        "PROBE {} | {chart} | {} {} {} {:e} {:e} | {:e} {:e} | {nappe} | {env:e} {lo:e} {hi:e} {sampled:e}",
        p.family,
        pt(e.frame.origin),
        v(e.frame.axis),
        v(e.frame.u_ref),
        e.semi_u,
        e.semi_v,
        p.span.0,
        p.span.1
    )
}

#[test]
#[ignore = "review probe"]
fn review_probe_6524() {
    let mut s = fuzz::start("review_probe_6524");
    let b = band();
    let f = Interval::from_f64;
    let n: usize = std::env::var("PROBE_N").ok().and_then(|x| x.parse().ok()).unwrap_or(4000);
    let mut tally = std::collections::BTreeMap::<&str, [usize; 4]>::new();
    for _ in 0..n {
        let p = draw(&mut s);
        let (t0, t1) = p.span;
        let t = tally.entry(p.family).or_default();
        let Ok(image) = projected::project(&p.carrier, Some((t0, t1)), &p.surface, b) else {
            t[0] += 1;
            continue;
        };
        let (ci, si) = (p.carrier.map_scalar(f), p.surface.map_scalar(f));
        let image_i = projected::project(&ci, Some((f(t0), f(t1))), &si, b);
        let e64 = projected::projected_envelope(&image, t0, t1, &image.chart_box(t0, t1), &p.carrier, &p.surface, b, None);
        let ei = image_i.as_ref().ok().map(|pi| {
            projected::projected_envelope(pi, f(t0), f(t1), &pi.chart_box(f(t0), f(t1)), &ci, &si, b, None)
        });
        let (env, lo, hi) = match (e64, ei) {
            (Ok((t64, _)), Some(Ok((ti, _)))) => (t64.total(), ti.total().lo(), ti.total().hi()),
            (Ok((t64, _)), _) => {
                t[2] += 1;
                (t64.total(), f64::NAN, f64::NAN)
            }
            (Err(_), _) => {
                t[1] += 1;
                continue;
            }
        };
        t[3] += 1;
        let stored = Pcurve::Projected(Box::new(image.clone()));
        let sampled = sampled_residual(&stored, (t0, t1), &p.carrier, &p.surface, 1024);
        println!("{}", row(&p, &image, env, lo, hi, sampled));
    }
    for (k, v) in tally {
        println!("[probe] {k}: no image {}, envelope refused {}, interval missing {}, read {}", v[0], v[1], v[2], v[3]);
    }
}

/// Door endings for ellipses in band and definitely off on each chart,
/// through the mint path (chart_pcurve_over, then certify with
/// minted_refusal), printed at the three doors.
#[test]
#[ignore = "review probe"]
fn review_probe_6524_endings() {
    use crate::recourse::{ReadAt, Reading};
    use geom_core::predicate::FileCoincidence;
    let b = band();
    let f = Interval::from_f64;
    let sphere = Surface::Sphere { center: Point3::new(0.1, 0.2, -0.3), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let cyl = Surface::Cylinder { origin: Point3::new(0.1, 0.2, -0.3), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() };
    let torus = Surface::Torus { center: Point3::new(0.1, 0.2, -0.3), axis: Vec3::unit_z(), major_radius: 2.0, minor_radius: 0.5, u_ref: Vec3::unit_x() };
    let cone = Surface::Cone { apex: Point3::new(0.1, 0.2, -0.3), axis: Vec3::unit_z(), half_angle: 0.5, u_ref: Vec3::unit_x() };
    let mid = 0.5 * (b.zero() + b.escalate());
    for (label, d) in [("in band", mid), ("off", 4.0 * b.escalate()), ("on", 0.3 * b.zero())] {
        let mk = |s: &Surface<f64>, c: Vec3<f64>, u: Vec3<f64>, v: Vec3<f64>, a: f64, bb: f64| {
            let (o, _) = frame_of(s);
            Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(o + c, u.cross(v).normalize(), u), a, bb))
        };
        // sphere: parallel at z=0.6, rho 0.8, squashed by d/0.8
        let sp = mk(&sphere, Vec3::new(0.0, 0.0, 0.6), Vec3::unit_x(), Vec3::unit_y(), 0.8, 0.8 - d / 0.8);
        // cylinder: oblique section tilt 0.7, minor squashed by d
        let tau: f64 = 0.7;
        let n = Vec3::new(tau.sin(), 0.0, tau.cos());
        let minor = Vec3::unit_y();
        let major = n.cross(minor).normalize();
        let cy = mk(&cyl, Vec3::new(0.0, 0.0, 0.4), minor, major, 1.0 - d, 1.0 / tau.cos());
        // torus: crest ellipse z = r, semi R and R - dd, off ~ dd^2/2r -> dd = sqrt(2 r d)
        let dd = (2.0 * 0.5 * d).sqrt();
        let to = mk(&torus, Vec3::new(0.0, 0.0, 0.5), Vec3::unit_x(), Vec3::unit_y(), 2.0, 2.0 - dd);
        // cone: rim at z=1 (radius tan .5) squashed: off ~ d*cos(alpha)
        let r = 0.5f64.tan();
        let co = mk(&cone, Vec3::new(0.0, 0.0, 1.0), Vec3::unit_x(), Vec3::unit_y(), r, r - d / 0.5f64.cos());
        // a tilted cone ellipse (no class): a rim tilted by 1e-3 then squashed
        for (what, s, c) in [("sphere", &sphere, &sp), ("cylinder", &cyl, &cy), ("torus", &torus, &to), ("cone", &cone, &co)] {
            for span in [(0.3, 2.5), (0.0, TAU)] {
                let (t0, t1) = span;
                let r64 = chart_pcurve_over(c, t0, t1, s, b).and_then(|image| {
                    PcurveCache::certify(image.clone(), t0, t1, c, s, b)
                        .map(|cache| (format!("{:?}", core::mem::discriminant(&image)), cache.certificate().envelope))
                        .map_err(|e| minted_refusal(e, &image, c, s))
                });
                let (ci, si) = (c.map_scalar(f), s.map_scalar(f));
                let ri = chart_pcurve_over(&ci, f(t0), f(t1), &si, b).and_then(|image| {
                    PcurveCache::certify(image.clone(), f(t0), f(t1), &ci, &si, b)
                        .map(|cache| cache.certificate().envelope.hi())
                        .map_err(|e| minted_refusal(e, &image, &ci, &si))
                });
                let show = |r: &Result<_, PcurveCertifyError>| match r {
                    Ok(_) => "certified".to_string(),
                    Err(e) => format!(
                        "{e:?}\n      BUILD: {}\n      REST: {}\n      IMPORT: {}",
                        e.ending(Reading::Build),
                        e.ending(Reading::AtRest),
                        e.ending(ReadAt::File(FileCoincidence::new(1e-12)))
                    ),
                };
                println!("ENDING [{label} d={d:e}] {what} {span:?}\n   f64: {}\n   iv: {}", show(&r64.as_ref().map(|_| ()).map_err(|e| e.clone())), show(&ri.as_ref().map(|_| ()).map_err(|e| e.clone())));
            }
        }
    }
}

/// A parallel of the unit sphere stored as an ellipse with one semi-axis
/// negative (the same point set), certified through the mint path.
#[test]
#[ignore = "review probe"]
fn review_probe_6524_negative_semi() {
    use crate::recourse::Reading;
    let b = band();
    let sphere = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    for (sa, sb) in [(0.8, 0.8 - 1e-9), (-0.8, 0.8 - 1e-9), (0.8, -(0.8 - 1e-9)), (-0.8, -(0.8 - 1e-9))] {
        let c = Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(Point3::new(0.0, 0.0, 0.6), Vec3::unit_z(), Vec3::unit_x()), sa, sb));
        let (t0, t1) = (0.3, 2.5);
        let r = chart_pcurve_over(&c, t0, t1, &sphere, b).and_then(|image| {
            PcurveCache::certify(image.clone(), t0, t1, &c, &sphere, b)
                .map(|cache| cache.certificate().envelope)
                .map_err(|e| minted_refusal(e, &image, &c, &sphere))
        });
        match r {
            Ok(env) => println!("NEG ({sa}, {sb}): certified, envelope {env:e}"),
            Err(e) => println!("NEG ({sa}, {sb}): {e:?}\n   BUILD: {}", e.ending(Reading::Build)),
        }
    }
}

/// A conic in the tangent plane along a ruling (orientation Zero) whose
/// sagitta is in band / 4·escalate / inside zero: the endings.
#[test]
#[ignore = "review probe"]
fn review_probe_6524_zero_orientation() {
    use crate::recourse::Reading;
    let b = band();
    let cyl = Surface::Cylinder { origin: Point3::new(0.1, 0.2, -0.3), axis: Vec3::unit_z(), radius: 0.5, u_ref: Vec3::unit_x() };
    for (label, sag) in [("zero", 0.4 * b.zero()), ("in band", 0.5 * (b.zero() + b.escalate())), ("off", 4.0 * b.escalate())] {
        for ratio in [1.0, 0.3, 3.0] {
            // semi-axis across the ruling `a` gives sagitta a^2/(2R); along the ruling `a*ratio`
            let a = (2.0 * 0.5 * sag).sqrt();
            let c = Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(Point3::new(0.6, 0.2, -0.3), Vec3::unit_x(), Vec3::unit_y()),
                a,
                a * ratio,
            ));
            for (t0, t1) in [(0.0, TAU), (0.3, 2.0)] {
                let r = chart_pcurve_over(&c, t0, t1, &cyl, b).and_then(|image| {
                    let kind = format!("{:?}", core::mem::discriminant(&image));
                    PcurveCache::certify(image.clone(), t0, t1, &c, &cyl, b)
                        .map(|cache| (kind, cache.certificate().envelope))
                        .map_err(|e| minted_refusal(e, &image, &c, &cyl))
                });
                match r {
                    Ok((k, env)) => println!("ZO [{label}] ratio {ratio} span ({t0},{t1}): certified {k} envelope {env:e}"),
                    Err(e) => println!("ZO [{label}] ratio {ratio} span ({t0},{t1}): {}\n   BUILD: {}\n   REST: {}", format!("{e:?}").chars().take(160).collect::<String>(), e.ending(Reading::Build), e.ending(Reading::AtRest)),
                }
            }
        }
    }
}

/// CC2 delta: the semi-axis gate on edge values (NaN, ±0, −inf, a
/// subnormal negative) and on a non-point Interval straddling zero.
#[test]
#[ignore = "review probe"]
fn review_probe_6524_semi_gate_edges() {
    use crate::recourse::{ReadAt, Reading};
    use crate::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, SurfaceKey};
    let sphere = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let mut surfaces: slotmap::SlotMap<SurfaceKey, Surface<f64>> = slotmap::SlotMap::with_key();
    let key = surfaces.insert(sphere);
    let frame = geom::Frame::new(Point3::new(0.0, 0.0, 0.6), Vec3::unit_z(), Vec3::unit_x());
    let f = Interval::from_f64;
    let (t0, t1) = (0.3, 2.5);
    let show = |what: &str, lane: &str, got: Result<(), crate::CertifyError>| match got {
        Ok(()) => println!("GATE {what} ({lane}): certified"),
        Err(e) => println!(
            "GATE {what} ({lane}): {}\n   BUILD: {:?}\n   REST: {:?}\n   IMPORT: {:?}",
            format!("{e:?}").chars().take(200).collect::<String>(),
            e.ending(Reading::Build),
            e.ending(Reading::AtRest),
            e.ending(ReadAt::File(geom_core::FileCoincidence::new(1e-6)))
        ),
    };
    for (what, a, b) in [
        ("NaN second", 0.8, f64::NAN),
        ("-0.0 second", 0.8, -0.0),
        ("-inf second", 0.8, f64::NEG_INFINITY),
        ("+inf second", 0.8, f64::INFINITY),
        ("subnormal negative second", 0.8, -5e-324),
    ] {
        let c = Curve3::Ellipse(geom::Ellipse::new(frame, a, b));
        let got = EdgeCurve::certify(
            EdgeCurveSpec { description: EdgeDescriptionSpec::chart(key), carrier: c.clone(), param_start: t0, param_end: t1 },
            c.eval(t0), c.eval(t1), |k| surfaces.get(k).cloned(),
            geom_core::test_support::fixture_lever(1.0), band(), crate::NurbsLane::certified(),
        ).map(|_| ());
        show(what, "f64", got);
        let ci = c.map_scalar(f);
        let got = EdgeCurve::certify(
            EdgeCurveSpec { description: EdgeDescriptionSpec::chart(key), carrier: ci.clone(), param_start: f(t0), param_end: f(t1) },
            ci.eval(f(t0)), ci.eval(f(t1)), |k| surfaces.get(k).map(|s| s.map_scalar(f)),
            geom_core::test_support::fixture_lever(f(1.0)), band(), crate::NurbsLane::certified(),
        ).map(|_| ());
        show(what, "Interval", got);
    }
    // a non-point Interval semi-axis straddling zero
    let fi = geom::Frame::new(Point3::new(f(0.0), f(0.0), f(0.6)), Vec3::new(f(0.0), f(0.0), f(1.0)), Vec3::new(f(1.0), f(0.0), f(0.0)));
    let ci = Curve3::Ellipse(geom::Ellipse::new(fi, f(0.8), Interval::from_bounds(-1e-20, 0.8)));
    let got = EdgeCurve::certify(
        EdgeCurveSpec { description: EdgeDescriptionSpec::chart(key), carrier: ci.clone(), param_start: f(t0), param_end: f(t1) },
        ci.eval(f(t0)), ci.eval(f(t1)), |k| surfaces.get(k).map(|s| s.map_scalar(f)),
        geom_core::test_support::fixture_lever(f(1.0)), band(), crate::NurbsLane::certified(),
    ).map(|_| ());
    show("straddling [-1e-20, 0.8]", "Interval", got);
}

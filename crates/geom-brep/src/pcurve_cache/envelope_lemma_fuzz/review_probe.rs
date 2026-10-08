//! Review probe (PR 4304, reviewer lane): the projected envelope at the
//! INTERVAL scalar against a certified LOWER bound of the true
//! displacement, on generic nets (controls are chart points along a
//! path, so the curve leaves the chart between them), near the singular
//! sets, both nappes, the torus inner side, with independent
//! per-control moves of the carrier and the stored net, a tilted frame
//! and a moved deck.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz;

use super::super::Pcurve;
use super::super::projected::{FramedCarrier, project, projected_envelope};
use super::{band, frame, unit};

#[derive(Clone, Copy, Debug)]
enum Chart {
    Cylinder,
    Cone,
    Sphere,
    Torus,
}

fn surface(chart: Chart, s: &mut fuzz::Rng) -> Surface<f64> {
    let (axis, u_ref) = frame(s);
    let o = Point3::new(s.range(-2.0, 2.0), s.range(-2.0, 2.0), s.range(-2.0, 2.0));
    match chart {
        Chart::Cylinder => Surface::Cylinder { origin: o, axis, radius: s.range(0.3, 2.0), u_ref },
        Chart::Cone => Surface::Cone { apex: o, axis, half_angle: s.range(0.1, 1.4), u_ref },
        Chart::Sphere => Surface::Sphere { center: o, radius: s.range(0.3, 2.0), axis, u_ref },
        Chart::Torus => {
            let major = s.range(0.8, 2.0);
            Surface::Torus {
                center: o,
                axis,
                major_radius: major,
                minor_radius: major * s.range(0.15, 0.9),
                u_ref,
            }
        }
    }
}

/// A `(u, v)` path start and sweep, biased toward the hard regions.
fn path(chart: Chart, s: &mut fuzz::Rng) -> ((f64, f64), (f64, f64)) {
    let u0 = s.range(0.0, std::f64::consts::TAU);
    let du = s.range(-2.5, 2.5);
    match chart {
        Chart::Cylinder => ((u0, s.range(-1.0, 1.0)), (du, s.range(-1.0, 1.0))),
        Chart::Cone => {
            // Slant near the apex on either nappe, never crossing it.
            let sg = if s.below(2) == 0 { 1.0 } else { -1.0 };
            let v0 = sg * 10f64.powf(s.range(-2.0, 0.3));
            let v1 = sg * 10f64.powf(s.range(-2.0, 0.3));
            ((u0, v0), (du, v1 - v0))
        }
        Chart::Sphere => {
            let lat = |s: &mut fuzz::Rng| {
                let sg = if s.below(2) == 0 { 1.0 } else { -1.0 };
                sg * (std::f64::consts::FRAC_PI_2 - 10f64.powf(s.range(-2.5, 0.0)))
            };
            let a = lat(s);
            let b = lat(s);
            ((u0, a), (du, b - a))
        }
        Chart::Torus => {
            // Inner side (v near π) half the time.
            let v0 = if s.below(2) == 0 { std::f64::consts::PI + s.range(-0.8, 0.8) } else { s.range(-3.0, 3.0) };
            ((u0, v0), (du, s.range(-2.0, 2.0)))
        }
    }
}

fn net_on_path(surf: &Surface<f64>, chart: Chart, s: &mut fuzz::Rng) -> NurbsCurve3<f64> {
    let ((u0, v0), (du, dv)) = path(chart, s);
    let n = 4 + s.below(5);
    let ctl: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            let f = i as f64 / (n - 1) as f64;
            // A wiggle off the straight path in the chart.
            let w = s.range(-0.15, 0.15);
            surf.eval(u0 + du * f + w * du.abs().min(1.0), v0 + dv * f)
        })
        .collect();
    let deg = 3.min(n - 1);
    let inner = n - deg - 1;
    let mut knots = vec![0.0; deg + 1];
    for k in 1..=inner {
        knots.push(k as f64 / (inner + 1) as f64);
    }
    knots.extend(vec![1.0; deg + 1]);
    let w: Vec<f64> = (0..n).map(|_| s.range(0.7, 1.4)).collect();
    NurbsCurve3::new(KnotVector::clamped(knots, deg).unwrap(), ctl, w).unwrap()
}

fn lift_s(s: &Surface<f64>) -> Surface<Interval> {
    s.map_scalar(Interval::from_f64)
}

#[test]
fn review_projected_envelope_dominates_a_certified_lower_bound_at_interval() {
    let mut s = fuzz::start("review_4304_projected_iv");
    let b = band();
    let lane = crate::FittedLane::<Interval>::certified();
    let (mut tried, mut certified_terms, mut worst) = (0usize, 0usize, 0.0f64);
    let mut refusals = std::collections::BTreeMap::<String, usize>::new();
    for _ in 0..fuzz::scaled(60) {
        for chart in [Chart::Cylinder, Chart::Cone, Chart::Sphere, Chart::Torus] {
            tried += 1;
            let surf = surface(chart, &mut s);
            let mut net = net_on_path(&surf, chart, &mut s);
            // Independent per-control moves of the carrier (incidence).
            let size = 10f64.powf(s.range(-7.0, -2.0));
            if s.below(2) == 0 {
                let moves: Vec<Vec3<f64>> = (0..net.control().len()).map(|_| unit(&mut s) * size * s.range(0.0, 1.0)).collect();
                let ctl = net.control().iter().zip(&moves).map(|(p, m)| *p + *m).collect();
                net = NurbsCurve3::new(net.knots().clone(), ctl, net.weights().to_vec()).unwrap();
            }
            let carrier = Curve3::Nurbs(Arc::new(net.clone()));
            let ci = carrier.map_scalar(Interval::from_f64);
            // A frame move: derive on the pose, certify on a tilted copy.
            let tilt = if s.below(3) == 0 { 10f64.powf(s.range(-9.0, -4.0)) } else { 0.0 };
            let si = lift_s(&surf);
            let mut image = match project(&ci, None, &si, b) {
                Ok(im) => im,
                Err(e) => {
                    *refusals.entry(format!("{chart:?} derive {:?}", std::mem::discriminant(&e))).or_default() += 1;
                    continue;
                }
            };
            let certify_on = if tilt > 0.0 {
                let mut t = |v: Vec3<f64>| (v + unit(&mut s) * tilt) * (1.0 + tilt);
                lift_s(&match surf {
                    Surface::Cylinder { origin, axis, radius, u_ref } => Surface::Cylinder { origin, axis: t(axis), radius, u_ref: t(u_ref) },
                    Surface::Cone { apex, axis, half_angle, u_ref } => Surface::Cone { apex, axis: t(axis), half_angle, u_ref: t(u_ref) },
                    Surface::Sphere { center, radius, axis, u_ref } => Surface::Sphere { center, radius, axis: t(axis), u_ref: t(u_ref) },
                    Surface::Torus { center, axis, major_radius, minor_radius, u_ref } => Surface::Torus { center, axis: t(axis), major_radius, minor_radius, u_ref: t(u_ref) },
                    other => other,
                })
            } else {
                si.clone()
            };
            // A stored-net move (fidelity) and a deck move.
            if s.below(2) == 0 {
                let FramedCarrier::Net(sn) = &image.carrier else { unreachable!() };
                let fsz = 10f64.powf(s.range(-8.0, -3.0));
                let ctl = sn.control().iter().map(|p| {
                    let m = unit(&mut s) * fsz * s.range(0.0, 1.0);
                    Point3::new(p.x + Interval::from_f64(m.x), p.y + Interval::from_f64(m.y), p.z + Interval::from_f64(m.z))
                }).collect();
                image.carrier = FramedCarrier::Net(Arc::new(NurbsCurve3::new(sn.knots().clone(), ctl, sn.weights().to_vec()).unwrap()));
            }
            if s.below(4) == 0 {
                image.u_off = image.u_off + Interval::from_f64(10f64.powf(s.range(-9.0, -4.0)));
            }
            let (t0, t1) = (Interval::from_f64(0.0), Interval::from_f64(1.0));
            let pc = Pcurve::Projected(Box::new(image.clone()));
            let boxed = pc.chart_box(t0, t1);
            let terms = match projected_envelope(&image, t0, t1, &boxed, &ci, &certify_on, b, Some(lane)) {
                Ok(t) => t,
                Err(e) => {
                    *refusals.entry(format!("{chart:?} env {e:?}").chars().take(90).collect()).or_default() += 1;
                    continue;
                }
            };
            let env = terms.total().hi();
            certified_terms += 1;
            let mut low = 0.0f64;
            for i in 0..=1024 {
                let t = Interval::from_f64(f64::from(i) / 1024.0);
                let q = pc.eval(t);
                let d = certify_on.eval(q.x, q.y).distance(ci.eval(t));
                if d.lo().is_finite() {
                    low = low.max(d.lo());
                }
            }
            if low > 0.0 && env.is_finite() && low / env > worst {
                worst = low / env;
                eprintln!("[review P2] new worst {worst:.4} {chart:?} low {low:e} env {env:e} terms {:?}", terms.0.iter().map(|x| x.hi()).collect::<Vec<_>>());
            }
            assert!(
                low <= env || !env.is_finite(),
                "{chart:?}: certified displacement lower bound {low:e} exceeds envelope {env:e}; terms {:?} — {}",
                terms.0.iter().map(|x| x.hi()).collect::<Vec<_>>(),
                fuzz::replay()
            );
        }
    }
    eprintln!("[review P2] tried {tried}, enveloped {certified_terms}, worst lower/envelope {worst:.4}");
    for (k, v) in &refusals {
        eprintln!("[review P2] {v:5} {k}");
    }
}

fn sphere1() -> Surface<f64> {
    Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() }
}

/// A tilted small circle on the unit sphere, its plane at distance `d`
/// from the centre along `n` (tilt `tilt` from the axis).
fn tilted(tilt: f64, d: f64) -> Curve3<f64> {
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    Curve3::Circle {
        center: Point3::origin() + n * d,
        axis: n,
        radius: (1.0 - d * d).sqrt(),
        u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
    }
}

fn certify_both(c: &Curve3<f64>, t0: f64, t1: f64) -> (String, String) {
    let b = band();
    let s = sphere1();
    let f64r = match super::super::chart_pcurve_over(c, t0, t1, &s, b) {
        Ok(img) => format!("{:?}", super::super::PcurveCache::certify(img, t0, t1, c, &s, b).map(|c| c.certificate().envelope)),
        Err(e) => format!("derive {e:?}"),
    };
    let (ci, si) = (c.map_scalar(Interval::from_f64), lift_s(&s));
    let (i0, i1) = (Interval::from_f64(t0), Interval::from_f64(t1));
    let ivr = match super::super::chart_pcurve_over(&ci, i0, i1, &si, b) {
        Ok(img) => format!("{:?}", super::super::PcurveCache::certify(img, i0, i1, &ci, &si, b).map(|c| c.certificate().envelope.hi())),
        Err(e) => format!("derive {e:?}"),
    };
    (f64r, ivr)
}

/// P3: a full turn of a tilted circle that encircles the axis; near-pole
/// passes; and the refinement's termination.
#[test]
fn review_p3_full_turn_and_pole() {
    let tau = std::f64::consts::TAU;
    for (name, c, t0, t1) in [
        ("encircling full turn tilt .3 d .2", tilted(0.3, 0.2), 0.0, tau),
        ("encircling 0.95 turn", tilted(0.3, 0.2), 0.0, 0.95 * tau),
        ("encircling 0.8 turn", tilted(0.3, 0.2), 0.0, 0.8 * tau),
        ("encircling 0.7 turn", tilted(0.3, 0.2), 0.0, 0.7 * tau),
        ("non-encircling full turn tilt 1.2 d .5", tilted(1.2, 0.5), 0.0, tau),
    ] {
        let (a, b) = certify_both(&c, t0, t1);
        eprintln!("[review P3] {name}: f64 {a} | iv {b}");
    }
    // A great circle through the pole has tilt π/2; pass at distance δ.
    for delta in [1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 3e-8, 1e-8, 3e-9] {
        // Tilt so that the circle (d = 0) comes within delta of the pole:
        // a great circle tilted π/2 − δ from the axis.
        let c = tilted(std::f64::consts::FRAC_PI_2 - delta, 0.0);
        let t = std::time::Instant::now();
        let (a, b) = certify_both(&c, -0.5, 0.5);
        eprintln!("[review P3] great-circle arc δ {delta:e} ({:?}): f64 {a} | iv {b}", t.elapsed());
    }
}

/// P5: the words a `Rung3Tube` refusal renders for an ANALYTIC pair.
#[test]
fn review_p5_rung3_tube_refusal_wording() {
    let b = band();
    let cyl = Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() };
    let sph = Surface::Sphere { center: Point3::new(0.0, 0.0, 0.3), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let half = 0.25_f64;
    let net = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, half.tan(), 0.0), Point3::new((2.0 * half).cos(), (2.0 * half).sin(), 0.0)],
        vec![1.0, half.cos(), 1.0],
    ).unwrap();
    for (name, a, bb) in [("cylinder × cylinder", &cyl, &cyl), ("cylinder × sphere", &cyl, &sph)] {
        match crate::rung3_tube(&net, a, bb, b) {
            Ok(()) => eprintln!("[review P5] {name}: Ok"),
            Err(e) => eprintln!("[review P5] {name}: {}", crate::CertifyError::Rung3Tube(e)),
        }
    }
}

/// P7: the deck maps on a projected row — whole-period shift, the
/// sphere's twin, a half-period shift and a reflection on a cylinder.
#[test]
fn review_p7_deck_maps() {
    use super::super::PcurveCache;
    use geom_core::{Point2, Vec2};
    let b = band();
    let pi = std::f64::consts::PI;
    let tau = std::f64::consts::TAU;
    // A sphere general circle arc.
    let c = tilted(1.0, 0.3);
    let s = sphere1();
    let (t0, t1) = (0.3, 2.1);
    let img = super::super::chart_pcurve_over(&c, t0, t1, &s, b).unwrap();
    let cert = |p: Pcurve<f64>| PcurveCache::certify(p, t0, t1, &c, &s, b).map(|x| x.certificate().envelope);
    let twin = img.map_affine(|p| Point2::new(p.x + pi, pi - p.y), |v| Vec2::new(v.x, -v.y));
    let dense = |p: &Pcurve<f64>| (0..=512).map(|i| { let t = t0 + (t1 - t0) * f64::from(i) / 512.0; let q = p.eval(t); s.eval(q.x, q.y).distance(c.eval(t)) }).fold(0.0, f64::max);
    eprintln!("[review P7] sphere: base {:?} dense {:e}", cert(img.clone()), dense(&img));
    eprintln!("[review P7] sphere twin: {:?} dense {:e}", cert(twin.clone()), dense(&twin));
    eprintln!("[review P7] sphere twin twice: {:?}", cert(twin.map_affine(|p| Point2::new(p.x + pi, pi - p.y), |v| Vec2::new(v.x, -v.y))));
    eprintln!("[review P7] sphere shift 2τ: {:?}", cert(img.shift_branch(2.0, tau)));
    eprintln!("[review P7] sphere shift π: {:?}", cert(img.shift_branch(1.0, pi)));
    eprintln!("[review P7] sphere v-only reflection: {:?} dense {:e}", cert(img.mirror_v().unwrap()), dense(&img.mirror_v().unwrap()));
    // A cylinder net.
    let cyl = Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() };
    let half = 0.6_f64;
    let net = Curve3::Nurbs(Arc::new(NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![Point3::new(1.0, 0.0, 0.0), Point3::new(1.0, half.tan(), 0.5), Point3::new((2.0 * half).cos(), (2.0 * half).sin(), 0.2)],
        vec![1.0, half.cos(), 1.0],
    ).unwrap()));
    let lane = crate::FittedLane::<f64>::certified();
    let pimg = super::super::chart_pcurve_over(&net, 0.0, 1.0, &cyl, b).unwrap();
    let pc = |p: Pcurve<f64>| { let Pcurve::Projected(x) = p else { unreachable!() }; PcurveCache::certify_projected(*x, 0.0, 1.0, &net, &cyl, b, Some(lane)).map(|x| x.certificate().envelope) };
    eprintln!("[review P7] cylinder net: {:?}", pc(pimg.clone()));
    eprintln!("[review P7] cylinder shift -3τ: {:?}", pc(pimg.shift_branch(-3.0, tau)));
    eprintln!("[review P7] cylinder shift π: {:?}", pc(pimg.shift_branch(1.0, pi)));
    eprintln!("[review P7] cylinder mirror_v: {:?}", pc(pimg.mirror_v().unwrap()));
    eprintln!("[review P7] cylinder v shift 1e-3: {:?}", pc(pimg.map_affine(|p| Point2::new(p.x, p.y + 1e-3), |v| v)));
}

/// P8: the plane arm's envelope against the dense truth (a spline
/// carrier off the plane between its controls), and which channel the
/// wrong-nappe row refuses on.
#[test]
fn review_p8_plane_envelope_and_nappe_channel() {
    let b = band();
    let plane = Surface::Plane { origin: Point3::origin(), normal: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    // A cubic whose middle controls leave the plane by h.
    let h = 1e-3;
    let net = NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.3, 0.2, h), Point3::new(0.7, -0.1, h), Point3::new(1.0, 0.1, 0.0)],
        vec![1.0, 1.0, 1.0, 1.0],
    ).unwrap();
    let c = Curve3::Nurbs(Arc::new(net));
    let image = project(&c, None, &plane, b).unwrap();
    let pc = Pcurve::Projected(Box::new(image.clone()));
    let boxed = pc.chart_box(0.0, 1.0);
    let env = projected_envelope(&image, 0.0, 1.0, &boxed, &c, &plane, b, None).map(|t| t.total());
    let dense = (0..=2048).map(|i| { let t = f64::from(i) / 2048.0; let q = pc.eval(t); plane.eval(q.x, q.y).distance(c.eval(t)) }).fold(0.0, f64::max);
    eprintln!("[review P8] plane: envelope {env:?}, dense sup {dense:e}");
    assert!(env.unwrap() >= dense, "plane envelope under the dense sup");
    // The wrong-nappe row, verbatim from the PR's test, printing its error.
    let cone = Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_z(), half_angle: 0.5, u_ref: Vec3::unit_x() };
    let half = 0.5_f64;
    let (r, z) = (0.5_f64.sin(), 0.5_f64.cos());
    let at = |th: f64| Point3::new(r * th.cos(), r * th.sin(), z);
    let h2 = 0.5;
    let mid = Point3::new(r * (0.3 + h2).cos() / h2.cos(), r * (0.3 + h2).sin() / h2.cos(), z);
    let rim = Curve3::Nurbs(Arc::new(NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![at(0.3), mid, at(1.3)],
        vec![1.0, h2.cos(), 1.0],
    ).unwrap()));
    let _ = half;
    let (rim, cone) = (rim.map_scalar(Interval::from_f64), cone.map_scalar(Interval::from_f64));
    let mut image = project(&rim, None, &cone, b).expect("upper nappe rim");
    if let super::super::ProjectedChart::Cone { ref mut nappe, .. } = image.chart {
        *nappe = crate::Nappe::Mirror;
    }
    let lane = crate::FittedLane::<Interval>::certified();
    let err = super::super::PcurveCache::certify_projected(image, Interval::from_f64(0.0), Interval::from_f64(1.0), &rim, &cone, b, Some(lane));
    eprintln!("[review P8] wrong nappe refuses: {:?}", err.map(|_| "Ok"));
}

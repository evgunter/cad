//! The projected envelope's counterexample searches, from PR 4304's
//! review: random nets near each curved chart with controls pushed off
//! at random, nets near the singular sets (the cone's apex, a sphere's
//! pole, the torus's inner side), and generic nets moved in several
//! respects at once against a certified LOWER bound of the true
//! displacement at the interval scalar. Each asserts the envelope
//! dominates.

use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz;

use super::super::projected::{FramedCarrier, project, projected_envelope};
use super::super::{Pcurve, PcurveCache};
use super::{band, frame, unit};

fn lift(c: &Curve3<f64>) -> Curve3<Interval> {
    c.map_scalar(Interval::from_f64)
}

/// The rational quadratic arc of `c + r·(cos θ·e₁ + sin θ·e₂)` over
/// `[a, a + span]` on `[0, 1]`.
fn arc(
    c: Point3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    r: f64,
    a: f64,
    span: f64,
) -> NurbsCurve3<f64> {
    let at = |th: f64| c + (e1 * th.cos() + e2 * th.sin()) * r;
    let h = 0.5 * span;
    let mid = c + (e1 * (a + h).cos() + e2 * (a + h).sin()) * (r / h.cos());
    NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![at(a), mid, at(a + span)],
        vec![1.0, h.cos(), 1.0],
    )
    .unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Chart {
    Cylinder,
    Cone,
    Sphere,
    Torus,
}

const CURVED: [Chart; 4] = [Chart::Cylinder, Chart::Cone, Chart::Sphere, Chart::Torus];

fn surface(chart: Chart, s: &mut fuzz::Rng) -> Surface<f64> {
    let (axis, u_ref) = frame(s);
    let o = Point3::new(s.range(-2.0, 2.0), s.range(-2.0, 2.0), s.range(-2.0, 2.0));
    match chart {
        Chart::Cylinder => Surface::Cylinder {
            origin: o,
            axis,
            radius: s.range(0.3, 2.0),
            u_ref,
        },
        Chart::Cone => Surface::Cone {
            apex: o,
            axis,
            half_angle: s.range(0.1, 1.4),
            u_ref,
        },
        Chart::Sphere => Surface::Sphere {
            center: o,
            radius: s.range(0.3, 2.0),
            axis,
            u_ref,
        },
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

/// A chart path `(u, v)` start and sweep, biased toward the hard
/// regions: near the cone's apex on either nappe, near a sphere's
/// poles, the torus's inner side.
fn path(chart: Chart, s: &mut fuzz::Rng) -> ((f64, f64), (f64, f64)) {
    let u0 = s.range(0.0, TAU);
    let du = s.range(-2.5, 2.5);
    match chart {
        Chart::Cylinder => ((u0, s.range(-1.0, 1.0)), (du, s.range(-1.0, 1.0))),
        Chart::Cone => {
            let sg = if s.below(2) == 0 { 1.0 } else { -1.0 };
            let v0 = sg * 10f64.powf(s.range(-2.0, 0.3));
            let v1 = sg * 10f64.powf(s.range(-2.0, 0.3));
            ((u0, v0), (du, v1 - v0))
        }
        Chart::Sphere => {
            let lat = |s: &mut fuzz::Rng| {
                let sg = if s.below(2) == 0 { 1.0 } else { -1.0 };
                sg * (FRAC_PI_2 - 10f64.powf(s.range(-2.5, 0.0)))
            };
            let (a, b) = (lat(s), lat(s));
            ((u0, a), (du, b - a))
        }
        Chart::Torus => {
            let v0 = if s.below(2) == 0 {
                PI + s.range(-0.8, 0.8)
            } else {
                s.range(-3.0, 3.0)
            };
            ((u0, v0), (du, s.range(-2.0, 2.0)))
        }
    }
}

/// A generic net whose controls are chart points along a wiggled path,
/// so the curve leaves the chart between them.
fn net_on_path(surf: &Surface<f64>, chart: Chart, s: &mut fuzz::Rng) -> NurbsCurve3<f64> {
    let ((u0, v0), (du, dv)) = path(chart, s);
    let n = 4 + s.below(5);
    let ctl: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let f = i as f64 / (n - 1) as f64;
            let w = s.range(-0.15, 0.15);
            surf.eval(u0 + du * f + w * du.abs().min(1.0), v0 + dv * f)
        })
        .collect();
    let deg = 3.min(n - 1);
    let inner = n - deg - 1;
    let mut knots = vec![0.0; deg + 1];
    for k in 1..=inner {
        #[allow(clippy::cast_precision_loss)]
        knots.push(k as f64 / (inner + 1) as f64);
    }
    knots.extend(vec![1.0; deg + 1]);
    let w: Vec<f64> = (0..n).map(|_| s.range(0.7, 1.4)).collect();
    NurbsCurve3::new(KnotVector::clamped(knots, deg).unwrap(), ctl, w).unwrap()
}

/// **The envelope dominates a certified lower bound of the displacement
/// at the interval scalar**: generic nets near the singular sets, with
/// the carrier's controls moved off the chart, the stored net moved off
/// the carrier's, the frame tilted and the deck moved, independently.
#[test]
fn the_envelope_dominates_a_certified_lower_bound_at_interval() {
    let mut s = fuzz::start("projected_interval_lower_bound");
    let b = band();
    let lane = crate::FittedLane::<Interval>::certified();
    for _ in 0..fuzz::scaled(20) {
        for chart in CURVED {
            let surf = surface(chart, &mut s);
            let mut net = net_on_path(&surf, chart, &mut s);
            let size = 10f64.powf(s.range(-7.0, -2.0));
            if s.below(2) == 0 {
                let ctl = net
                    .control()
                    .iter()
                    .map(|p| *p + unit(&mut s) * (size * s.range(0.0, 1.0)))
                    .collect();
                net = NurbsCurve3::new(net.knots().clone(), ctl, net.weights().to_vec()).unwrap();
            }
            let carrier = Curve3::Nurbs(Arc::new(net));
            let ci = lift(&carrier);
            let si = surf.map_scalar(Interval::from_f64);
            let Ok(mut image) = project(&ci, None, &si, b) else {
                continue;
            };
            let tilt = if s.below(3) == 0 {
                10f64.powf(s.range(-9.0, -4.0))
            } else {
                0.0
            };
            let certify_on = if tilt > 0.0 {
                let mut t = |v: Vec3<f64>| (v + unit(&mut s) * tilt) * (1.0 + tilt);
                match surf {
                    Surface::Cylinder {
                        origin,
                        axis,
                        radius,
                        u_ref,
                    } => Surface::Cylinder {
                        origin,
                        axis: t(axis),
                        radius,
                        u_ref: t(u_ref),
                    },
                    Surface::Cone {
                        apex,
                        axis,
                        half_angle,
                        u_ref,
                    } => Surface::Cone {
                        apex,
                        axis: t(axis),
                        half_angle,
                        u_ref: t(u_ref),
                    },
                    Surface::Sphere {
                        center,
                        radius,
                        axis,
                        u_ref,
                    } => Surface::Sphere {
                        center,
                        radius,
                        axis: t(axis),
                        u_ref: t(u_ref),
                    },
                    Surface::Torus {
                        center,
                        axis,
                        major_radius,
                        minor_radius,
                        u_ref,
                    } => Surface::Torus {
                        center,
                        axis: t(axis),
                        major_radius,
                        minor_radius,
                        u_ref: t(u_ref),
                    },
                    other => other,
                }
                .map_scalar(Interval::from_f64)
            } else {
                si.clone()
            };
            if s.below(2) == 0 {
                let FramedCarrier::Net(sn) = &image.carrier else {
                    unreachable!("a net's image")
                };
                let fsz = 10f64.powf(s.range(-8.0, -3.0));
                let ctl = sn
                    .control()
                    .iter()
                    .map(|p| {
                        let m = unit(&mut s) * (fsz * s.range(0.0, 1.0));
                        Point3::new(
                            p.x + Interval::from_f64(m.x),
                            p.y + Interval::from_f64(m.y),
                            p.z + Interval::from_f64(m.z),
                        )
                    })
                    .collect();
                image.carrier = FramedCarrier::Net(Arc::new(
                    NurbsCurve3::new(sn.knots().clone(), ctl, sn.weights().to_vec()).unwrap(),
                ));
            }
            if s.below(4) == 0 {
                image.u_off = image.u_off + Interval::from_f64(10f64.powf(s.range(-9.0, -4.0)));
            }
            let (t0, t1) = (Interval::from_f64(0.0), Interval::from_f64(1.0));
            let pc = Pcurve::Projected(Box::new(image.clone()));
            let boxed = pc.chart_box(t0, t1);
            let Ok(terms) =
                projected_envelope(&image, t0, t1, &boxed, &ci, &certify_on, b, Some(lane))
            else {
                continue;
            };
            let env = terms.total().hi();
            let low = (0..=1024)
                .filter_map(|i| {
                    let t = Interval::from_f64(f64::from(i) / 1024.0);
                    let q = pc.eval(t);
                    let lo = certify_on.eval(q.x, q.y).distance(ci.eval(t)).lo();
                    lo.is_finite().then_some(lo)
                })
                .fold(0.0, f64::max);
            assert!(
                low <= env || !env.is_finite(),
                "{chart:?}: certified lower bound {low:e} over the envelope {env:e} — {}",
                fuzz::replay()
            );
        }
    }
}

/// **The envelope dominates on cubic nets pushed off each chart**, and
/// near its singular sets: random controls along an on-chart carrier,
/// pushed off at random, against the densely sampled displacement, at
/// the interval scalar.
#[test]
fn the_envelope_dominates_pushed_nets_and_corners() {
    let mut s = fuzz::start("projected_pushed_and_corners");
    let b = band();
    let lane = crate::FittedLane::<Interval>::certified();
    for _ in 0..fuzz::scaled(60) {
        let which = s.below(3);
        let delta = 10f64.powf(s.range(-6.0, -1.0));
        let (surf, base): (Surface<f64>, NurbsCurve3<f64>) = match which {
            0 => {
                let ha = s.range(0.2, 1.3);
                let sign = if s.below(2) == 0 { 1.0 } else { -1.0 };
                let a = s.range(0.0, TAU);
                (
                    Surface::Cone {
                        apex: Point3::origin(),
                        axis: Vec3::unit_z(),
                        half_angle: ha,
                        u_ref: Vec3::unit_x(),
                    },
                    arc(
                        Point3::new(0.0, 0.0, sign * delta * ha.cos()),
                        Vec3::unit_x(),
                        Vec3::unit_y(),
                        delta * ha.sin(),
                        a,
                        s.range(0.3, 2.0),
                    ),
                )
            }
            1 => {
                let tilt = s.range(0.0, 0.5);
                let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
                let r = delta.min(0.5);
                let e1 = Vec3::new(tilt.cos(), 0.0, -tilt.sin());
                (
                    Surface::Sphere {
                        center: Point3::origin(),
                        radius: 1.0,
                        axis: Vec3::unit_z(),
                        u_ref: Vec3::unit_x(),
                    },
                    arc(
                        Point3::origin() + n * (1.0 - r * r).sqrt(),
                        e1,
                        n.cross(e1),
                        r,
                        s.range(0.0, TAU),
                        s.range(0.3, 2.0),
                    ),
                )
            }
            _ => {
                let (big, small) = (1.0, s.range(0.2, 0.9));
                let v = PI + s.range(-0.5, 0.5);
                (
                    Surface::Torus {
                        center: Point3::origin(),
                        axis: Vec3::unit_z(),
                        major_radius: big,
                        minor_radius: small,
                        u_ref: Vec3::unit_x(),
                    },
                    arc(
                        Point3::new(0.0, 0.0, small * v.sin()),
                        Vec3::unit_x(),
                        Vec3::unit_y(),
                        big + small * v.cos(),
                        s.range(0.0, TAU),
                        s.range(0.3, 2.0),
                    ),
                )
            }
        };
        let push = delta * 10f64.powf(s.range(-10.0, -4.0));
        let ctl: Vec<Point3<f64>> = base
            .control()
            .iter()
            .map(|&p| p + unit(&mut s) * (push * s.range(0.0, 1.0)))
            .collect();
        let c = Curve3::Nurbs(Arc::new(
            NurbsCurve3::new(base.knots().clone(), ctl, base.weights().to_vec()).unwrap(),
        ));
        let Ok(image64) = project(&c, None, &surf, b) else {
            continue;
        };
        let p64 = Pcurve::Projected(Box::new(image64));
        let dense = (0..=4000)
            .map(|i| {
                let t = f64::from(i) / 4000.0;
                let q = p64.eval(t);
                surf.eval(q.x, q.y).distance(c.eval(t))
            })
            .fold(0.0, f64::max);
        let (ci, si) = (lift(&c), surf.map_scalar(Interval::from_f64));
        let Ok(image) = project(&ci, None, &si, b) else {
            continue;
        };
        let (i0, i1) = (Interval::from_f64(0.0), Interval::from_f64(1.0));
        if let Ok(cache) = PcurveCache::certify_projected(image, i0, i1, &ci, &si, b, Some(lane)) {
            // `dense` is sampled at f64: its evaluations' own rounding,
            // a few ulps of the unit-scale coordinates, is not the
            // carrier's displacement, and is allowed for.
            let rounding = 64.0 * f64::EPSILON;
            let env = cache.certificate().envelope.hi();
            assert!(
                env + rounding >= dense,
                "which {which} δ {delta:e}: envelope {env:e} under {dense:e} — {}",
                fuzz::replay()
            );
        }
    }
}

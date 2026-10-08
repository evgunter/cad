//! The projected image's lemma, swept (`pcurve_cache::projected`'s
//! module docs): on every analytic chart, a carrier exactly on the chart
//! images exactly, to rounding, at `f64` and at `Interval`; `S(ψ(ξ))` is
//! the chart's nearest point to `ξ`; and the envelope's terms dominate
//! the true sup of `|S(P(t)) − C(t)|` with each term load-bearing in a
//! row where only its own respect is moved. The sector condition and the
//! cone's lever refuse typed rather than bound.

use std::f64::consts::{FRAC_PI_2, PI, TAU};
use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::tolerance::Tol;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz;

use super::super::projected::{FramedCarrier, SectorChannel, project, projected_envelope};
use super::super::{EnvelopeTerm, Pcurve, PcurveCache, PcurveCertifyError};
use super::{band, frame, unit};

/// The five charts, in a random pose.
#[derive(Clone, Copy, Debug)]
enum Chart {
    Plane,
    Cylinder,
    Cone,
    Sphere,
    Torus,
}

const CHARTS: [Chart; 5] = [
    Chart::Plane,
    Chart::Cylinder,
    Chart::Cone,
    Chart::Sphere,
    Chart::Torus,
];

fn surface(chart: Chart, s: &mut fuzz::Rng) -> Surface<f64> {
    let (axis, u_ref) = frame(s);
    let o = Point3::new(s.range(-2.0, 2.0), s.range(-2.0, 2.0), s.range(-2.0, 2.0));
    match chart {
        Chart::Plane => Surface::Plane {
            origin: o,
            normal: axis,
            u_ref,
        },
        Chart::Cylinder => Surface::Cylinder {
            origin: o,
            axis,
            radius: s.range(0.3, 2.0),
            u_ref,
        },
        Chart::Cone => Surface::Cone {
            apex: o,
            axis,
            half_angle: s.range(0.2, 1.3),
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
                minor_radius: major * s.range(0.15, 0.7),
                u_ref,
            }
        }
    }
}

/// The rational quadratic arc of the circle `c + r·(cos θ·e₁ + sin θ·e₂)`
/// over `[a, a + span]`, `span ≤ 2π/3`, on the parameter `[0, 1]`.
fn arc(c: Point3<f64>, e1: Vec3<f64>, e2: Vec3<f64>, r: f64, a: f64, span: f64) -> Curve3<f64> {
    let at = |th: f64| c + (e1 * th.cos() + e2 * th.sin()) * r;
    let h = 0.5 * span;
    let mid = c + (e1 * (a + h).cos() + e2 * (a + h).sin()) * (r / h.cos());
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![at(a), mid, at(a + span)],
            vec![1.0, h.cos(), 1.0],
        )
        .unwrap(),
    ))
}

fn segment(p: Point3<f64>, q: Point3<f64>) -> Curve3<f64> {
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![p, q],
            vec![1.0, 1.0],
        )
        .unwrap(),
    ))
}

fn rad(axis: Vec3<f64>, u_ref: Vec3<f64>, u: f64) -> Vec3<f64> {
    u_ref * u.cos() + axis.cross(u_ref) * u.sin()
}

/// A spline carrier exactly on `surf` (a rational arc of one of its
/// circles, a segment of one of its lines, or a planar cubic).
fn on_chart(surf: &Surface<f64>, s: &mut fuzz::Rng) -> Curve3<f64> {
    let span = s.range(0.3, 2.0);
    let a = s.range(0.0, TAU);
    match *surf {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let v_ref = normal.cross(u_ref);
            // Monotone in `u`, so the net's speed bound is positive.
            let ctl: Vec<Point3<f64>> = (0..4)
                .map(|i| {
                    origin
                        + u_ref * (f64::from(i) * 0.6 + s.range(-0.1, 0.1))
                        + v_ref * s.range(-0.5, 0.5)
                })
                .collect();
            Curve3::Nurbs(Arc::new(
                NurbsCurve3::new(
                    KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
                    ctl,
                    vec![1.0, s.range(0.8, 1.25), s.range(0.8, 1.25), 1.0],
                )
                .unwrap(),
            ))
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let h = s.range(-1.0, 1.0);
            if s.below(2) == 0 {
                arc(origin + axis * h, u_ref, axis.cross(u_ref), radius, a, span)
            } else {
                let p = origin + rad(axis, u_ref, a) * radius;
                segment(p + axis * h, p + axis * (h + span))
            }
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let sign = if s.below(2) == 0 { 1.0 } else { -1.0 };
            let v = sign * s.range(0.4, 2.0);
            if s.below(2) == 0 {
                let (sa, ca) = half_angle.sin_cos();
                arc(
                    apex + axis * (v * ca),
                    u_ref,
                    axis.cross(u_ref),
                    v.abs() * sa,
                    a,
                    span,
                )
            } else {
                let g = |w: f64| {
                    apex + (axis * half_angle.cos() + rad(axis, u_ref, a) * half_angle.sin()) * w
                };
                segment(g(v), g(v + sign * span))
            }
        }
        Surface::Sphere { center, radius, .. } => {
            let n = unit(s);
            let d = radius * s.range(-0.8, 0.8);
            let e1 = n.cross(unit(s)).normalize();
            arc(
                center + n * d,
                e1,
                n.cross(e1),
                (radius * radius - d * d).sqrt(),
                a,
                span,
            )
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            if s.below(2) == 0 {
                let v = s.range(0.0, TAU);
                arc(
                    center + axis * (minor_radius * v.sin()),
                    u_ref,
                    axis.cross(u_ref),
                    major_radius + minor_radius * v.cos(),
                    a,
                    span,
                )
            } else {
                let r = rad(axis, u_ref, s.range(0.0, TAU));
                arc(center + r * major_radius, r, axis, minor_radius, a, span)
            }
        }
        Surface::Nurbs(_) | Surface::Approx(_) => unreachable!("analytic charts only"),
    }
}

/// A circle on `sphere` that is neither a parallel nor a meridian, with
/// an arc of it.
fn general_circle(sphere: &Surface<f64>, s: &mut fuzz::Rng) -> (Curve3<f64>, f64, f64) {
    let Surface::Sphere { center, radius, .. } = *sphere else {
        unreachable!()
    };
    let n = unit(s);
    let d = radius * s.range(-0.8, 0.8);
    let u_ref = n.cross(unit(s)).normalize();
    let t0 = s.range(-TAU, TAU);
    (
        Curve3::Circle {
            center: center + n * d,
            axis: n,
            radius: (radius * radius - d * d).sqrt(),
            u_ref,
        },
        t0,
        t0 + s.range(0.3, 4.0),
    )
}

fn domain(c: &Curve3<f64>) -> (f64, f64) {
    match c {
        Curve3::Nurbs(n) => n.domain(),
        _ => unreachable!(),
    }
}

/// The densely sampled `max |S(P(t)) − C(t)|`.
fn sampled(p: &Pcurve<f64>, c: &Curve3<f64>, surf: &Surface<f64>, t0: f64, t1: f64) -> f64 {
    (0..=256)
        .map(|i| {
            let t = t0 + (t1 - t0) * f64::from(i) / 256.0;
            let q = p.eval(t);
            surf.eval(q.x, q.y).distance(c.eval(t))
        })
        .fold(0.0, f64::max)
}

/// The smallest distance of the carrier from the chart's singular set
/// (a sphere's poles' axis, the cone's axis), sampled — what makes a
/// sector refusal honest rather than a counterexample.
fn axis_clearance(c: &Curve3<f64>, surf: &Surface<f64>, t0: f64, t1: f64) -> f64 {
    let (o, axis) = match *surf {
        Surface::Sphere { center, axis, .. } => (center, axis),
        Surface::Cone { apex, axis, .. } => (apex, axis),
        _ => return f64::INFINITY,
    };
    (0..=256)
        .map(|i| {
            let w = c.eval(t0 + (t1 - t0) * f64::from(i) / 256.0) - o;
            (w - axis * w.dot(axis)).norm()
        })
        .fold(f64::INFINITY, f64::min)
}

fn lift(c: &Curve3<f64>) -> Curve3<Interval> {
    c.map_scalar(Interval::from_f64)
}

/// **Exactness**: on each chart, random poses and carriers on the chart
/// — splines on all five, general circles on the sphere — image to
/// rounding and certify with a rounding-sized envelope, at `f64` and at
/// `Interval`. A counterexample search.
#[test]
fn a_carrier_on_its_chart_images_exactly_at_f64_and_interval() {
    let mut s = fuzz::start("projected_exactness");
    let b = band();
    let lane64 = crate::FittedLane::<f64>::certified();
    let lane_iv = crate::FittedLane::<Interval>::certified();
    for _ in 0..fuzz::scaled(12) {
        for chart in CHARTS {
            let surf = surface(chart, &mut s);
            let (carrier, t0, t1) = if matches!(chart, Chart::Sphere) && s.below(2) == 0 {
                general_circle(&surf, &mut s)
            } else {
                let c = on_chart(&surf, &mut s);
                let (t0, t1) = domain(&c);
                (c, t0, t1)
            };
            let what = format!(
                "{chart:?} {carrier:?} [{t0}, {t1}] on {surf:?} — {}",
                fuzz::replay()
            );
            let image = match super::super::chart_pcurve_over(&carrier, t0, t1, &surf, b) {
                Ok(image) => image,
                Err(PcurveCertifyError::SectorRefused { .. })
                    if axis_clearance(&carrier, &surf, t0, t1) < 1e-6 =>
                {
                    continue;
                }
                Err(e) => panic!("{what}: the derivation refused: {e:?}"),
            };
            assert!(matches!(image, Pcurve::Projected(_)), "{what}: {image:?}");
            let worst = sampled(&image, &carrier, &surf, t0, t1);
            assert!(worst < 1e-12, "{what}: sampled displacement {worst:e}");
            // The period headroom reads the image's per-piece boxes, which
            // overstate the sweep near a pole: an arc whose sampled
            // azimuth already sweeps most of a turn may refuse it
            // conservatively, and is not this row's subject.
            let us: Vec<f64> = (0..=256)
                .map(|i| image.eval(t0 + (t1 - t0) * f64::from(i) / 256.0).x)
                .collect();
            let sweep = us.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                - us.iter().copied().fold(f64::INFINITY, f64::min);
            if sweep > 1.5 * PI {
                continue;
            }
            let cache = PcurveCache::certify(image.clone(), t0, t1, &carrier, &surf, b)
                .or_else(|_| {
                    let Pcurve::Projected(p) = image.clone() else {
                        unreachable!()
                    };
                    PcurveCache::certify_projected(*p, t0, t1, &carrier, &surf, b, Some(lane64))
                })
                .unwrap_or_else(|e| panic!("{what}: f64 certification refused: {e:?}"));
            let env = cache.certificate().envelope;
            assert!(env < 1e-10, "{what}: f64 envelope {env:e}");
            // The f64 lane rounds both, so the comparison has rounding room.
            assert!(
                env >= worst - 1e-13,
                "{what}: envelope {env:e} under sampled {worst:e}"
            );
            // The interval scalar: the same pair, lifted.
            let (ci, si) = (lift(&carrier), surf.map_scalar(Interval::from_f64));
            let (i0, i1) = (Interval::from_f64(t0), Interval::from_f64(t1));
            let image = super::super::chart_pcurve_over(&ci, i0, i1, &si, b)
                .unwrap_or_else(|e| panic!("{what}: interval derivation refused: {e:?}"));
            let Pcurve::Projected(p) = image else {
                panic!("{what}: interval image {image:?}")
            };
            let cache = PcurveCache::certify_projected(*p, i0, i1, &ci, &si, b, Some(lane_iv))
                .unwrap_or_else(|e| panic!("{what}: interval certification refused: {e:?}"));
            let env = cache.certificate().envelope;
            assert!(env.hi() < 1e-9, "{what}: interval envelope {:e}", env.hi());
            // Every sampled point's displacement encloses the f64 truth.
            for i in 0..=8 {
                let t = Interval::from_f64(t0 + (t1 - t0) * f64::from(i) / 8.0);
                let q = cache.pcurve().eval(t);
                let d = si.eval(q.x, q.y).distance(ci.eval(t));
                assert!(
                    d.hi() < 1e-9,
                    "{what}: interval displacement at {i}: {:e}; image {q:?} on {si:?}",
                    d.hi()
                );
            }
        }
    }
}

/// **`S(ψ(ξ))` is the nearest point**: for points near each chart, the
/// projected image's point maps to a point whose offset from `ξ` is
/// normal to the chart there and is the chart's own distance.
#[test]
fn the_projection_is_the_nearest_point_on_every_chart() {
    let mut s = fuzz::start("projected_nearest_point");
    let b = band();
    for _ in 0..fuzz::scaled(40) {
        for chart in CHARTS {
            let surf = surface(chart, &mut s);
            let base = on_chart(&surf, &mut s);
            let (t0, t1) = domain(&base);
            let on = base.eval(t0 + (t1 - t0) * s.range(0.1, 0.9));
            let off = unit(&mut s) * s.range(0.0, 0.05);
            let xi = on + off;
            let point = segment(xi, xi + unit(&mut s) * 1e-3);
            let Ok(image) = project(&point, None, &surf, b) else {
                continue;
            };
            let q = Pcurve::Projected(Box::new(image)).eval(0.0);
            let foot = surf.eval(q.x, q.y);
            let h = 1e-6;
            let du = surf.eval(q.x + h, q.y) - surf.eval(q.x - h, q.y);
            let dv = surf.eval(q.x, q.y + h) - surf.eval(q.x, q.y - h);
            let gap = xi - foot;
            let what = format!("{chart:?} ξ = {xi:?} — {}", fuzz::replay());
            for (name, d) in [("u", du), ("v", dv)] {
                if d.norm() > 1e-9 {
                    let cos = gap.dot(d) / (d.norm() * gap.norm().max(1e-300));
                    assert!(
                        gap.norm() < 1e-12 || cos.abs() < 1e-5,
                        "{what}: the offset is not normal along {name}: cos {cos:e}"
                    );
                }
            }
            assert!(
                gap.norm() <= off.norm() + 1e-12,
                "{what}: the foot {:e} is farther than the point it came from {:e}",
                gap.norm(),
                off.norm()
            );
        }
    }
}

/// One respect a dominance row moves.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Moved {
    /// The carrier off the chart (`Incidence`).
    Carrier,
    /// The stored net off the carrier's (`Fidelity`).
    Net,
    /// The chart's frame off orthonormal (`Frame`).
    Frame,
    /// The deck map off a deck transformation (`FidelityU`).
    Deck,
}

impl Moved {
    fn term(self) -> EnvelopeTerm {
        match self {
            Self::Carrier => EnvelopeTerm::Incidence,
            Self::Net => EnvelopeTerm::Fidelity,
            Self::Frame => EnvelopeTerm::Frame,
            Self::Deck => EnvelopeTerm::FidelityU,
        }
    }
}

/// One dominance row on a curved chart: `(sampled sup, envelope, the
/// moved term's value)`, or `None` where the moved pair refuses before
/// any term (a sector the move broke).
fn dominance_row(chart: Chart, moved: Moved, s: &mut fuzz::Rng) -> Option<(f64, f64, f64)> {
    let b = band();
    let mut surf = surface(chart, s);
    let carrier = on_chart(&surf, s);
    let (t0, t1) = domain(&carrier);
    let size = 10f64.powf(s.range(-7.0, -4.0));
    let carrier = match moved {
        Moved::Carrier => {
            let Curve3::Nurbs(n) = &carrier else {
                unreachable!()
            };
            let push = unit(s) * size;
            Curve3::Nurbs(Arc::new(n.map_points(|p| p + push)))
        }
        _ => carrier,
    };
    if moved == Moved::Frame {
        let tilt = |v: Vec3<f64>, s: &mut fuzz::Rng| (v + unit(s) * size) * (1.0 + size);
        surf = match surf {
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Surface::Cylinder {
                origin,
                axis: tilt(axis, s),
                radius,
                u_ref: tilt(u_ref, s),
            },
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Surface::Cone {
                apex,
                axis: tilt(axis, s),
                half_angle,
                u_ref: tilt(u_ref, s),
            },
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Surface::Sphere {
                center,
                radius,
                axis: tilt(axis, s),
                u_ref: tilt(u_ref, s),
            },
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Surface::Torus {
                center,
                axis: tilt(axis, s),
                major_radius,
                minor_radius,
                u_ref: tilt(u_ref, s),
            },
            other => other,
        };
    }
    // A frame row's stored net is the twin frame's, so only the map
    // differs from the lemma's.
    let derive_on = if moved == Moved::Frame {
        super::super::orthonormal_chart(&surf)
    } else {
        surf.clone()
    };
    let mut image = project(&carrier, None, &derive_on, b).ok()?;
    match moved {
        Moved::Net => {
            let FramedCarrier::Net(net) = &image.carrier else {
                unreachable!()
            };
            let control = net.control().iter().map(|&p| p + unit(s) * size).collect();
            image.carrier = FramedCarrier::Net(Arc::new(
                NurbsCurve3::new(net.knots().clone(), control, net.weights().to_vec()).unwrap(),
            ));
        }
        Moved::Deck => image.u_off += size,
        Moved::Carrier | Moved::Frame => {}
    }
    let pcurve = Pcurve::Projected(Box::new(image.clone()));
    let boxed = pcurve.chart_box(t0, t1);
    let lane = crate::FittedLane::<f64>::certified();
    let terms = projected_envelope(&image, t0, t1, &boxed, &carrier, &surf, b, Some(lane)).ok()?;
    Some((
        sampled(&pcurve, &carrier, &surf, t0, t1),
        terms.total(),
        terms.0[moved.term().slot()],
    ))
}

const CURVED: [Chart; 4] = [Chart::Cylinder, Chart::Cone, Chart::Sphere, Chart::Torus];
const MOVES: [Moved; 4] = [Moved::Carrier, Moved::Net, Moved::Frame, Moved::Deck];

/// **The envelope dominates**: every row moved in one respect bounds
/// its densely sampled displacement. A counterexample search.
#[test]
fn the_projected_envelope_dominates_the_sampled_displacement() {
    let mut s = fuzz::start("projected_dominance");
    for _ in 0..fuzz::scaled(10) {
        for chart in CURVED {
            for moved in MOVES {
                if let Some((sup, env, _)) = dominance_row(chart, moved, &mut s) {
                    assert!(
                        env >= sup * (1.0 - 1e-9) - 1e-15,
                        "{chart:?} moved {moved:?}: envelope {env:e} under sampled {sup:e} — {}",
                        fuzz::replay()
                    );
                }
            }
        }
    }
}

/// **Each term is load-bearing**: on every curved chart, for each
/// respect, some row of a pinned sweep needs that term — the envelope
/// less it falls under the sampled displacement. Red if a term is
/// dropped or under-stated. Pinned: a coverage witness, not a search.
#[test]
fn every_projected_term_is_load_bearing_in_a_pinned_sweep() {
    let mut s = fuzz::pinned("projected_load_bearing", 0x5eed_0c4a_2d1f_7e01);
    for chart in CURVED {
        for moved in MOVES {
            let bearing = (0..24).any(|_| {
                dominance_row(chart, moved, &mut s)
                    .is_some_and(|(sup, env, term)| term > 0.0 && env - term < sup)
            });
            assert!(
                bearing,
                "{chart:?}: the {:?} term is never load-bearing",
                moved.term()
            );
        }
    }
}

/// **The sector condition refuses typed**: a piece's branch centre
/// turned a half period off, and an arc through a sphere's pole.
#[test]
fn a_broken_sector_and_an_arc_through_the_pole_refuse() {
    let b = band();
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let tilt = 0.4_f64;
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    let circle = Curve3::Circle {
        center: Point3::origin(),
        axis: n,
        radius: 1.0,
        u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
    };
    let image = project(&circle, Some((0.2, 1.4)), &sphere, b).expect("clear of the poles");
    let mut turned = image.clone();
    turned.azimuth[0] += 2;
    let err = PcurveCache::certify(
        Pcurve::Projected(Box::new(turned)),
        0.2,
        1.4,
        &circle,
        &sphere,
        b,
    )
    .expect_err("a turned branch centre");
    assert!(
        matches!(
            err,
            PcurveCertifyError::SectorRefused {
                channel: SectorChannel::Azimuth | SectorChannel::Branch,
                ..
            }
        ),
        "{err:?}"
    );
    // A great circle through the pole: its arc over it has no branch.
    let polar = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.3, 0.9, 0.0).normalize(),
        radius: 1.0,
        u_ref: Vec3::unit_z(),
    };
    let err = project(&polar, Some((-0.5, 0.5)), &sphere, b).expect_err("through the pole");
    assert!(
        matches!(err, PcurveCertifyError::SectorRefused { .. }),
        "{err:?}"
    );
    let _ = FRAC_PI_2;
}

/// **The cone's lever refuses typed**: a stored image whose nappe is not
/// the carrier's. At the interval scalar, where no schedule runs ahead
/// of the envelope.
#[test]
fn the_wrong_nappe_refuses_on_the_cone_lever() {
    let b = band();
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let rim = arc(
        Point3::new(0.0, 0.0, 0.5_f64.cos()),
        Vec3::unit_x(),
        Vec3::unit_y(),
        0.5_f64.sin(),
        0.3,
        1.0,
    );
    let (rim, cone) = (lift(&rim), cone.map_scalar(Interval::from_f64));
    let mut image = project(&rim, None, &cone, b).expect("the upper nappe's rim");
    if let super::super::ProjectedChart::Cone { ref mut nappe, .. } = image.chart {
        *nappe = crate::Nappe::Mirror;
    }
    let lane = crate::FittedLane::<Interval>::certified();
    let (t0, t1) = (Interval::from_f64(0.0), Interval::from_f64(1.0));
    let err = PcurveCache::certify_projected(image, t0, t1, &rim, &cone, b, Some(lane))
        .expect_err("the wrong nappe");
    assert!(
        matches!(
            err,
            PcurveCertifyError::SectorRefused {
                channel: SectorChannel::Lever | SectorChannel::Azimuth,
                ..
            }
        ),
        "{err:?}"
    );
}

/// **An off-chart spline refuses by incidence**: a spline a millimetre
/// off the torus is read off the chart before any image is built.
#[test]
fn an_off_chart_spline_refuses_by_incidence() {
    let b = band();
    let torus = Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 1.0,
        minor_radius: 0.3,
        u_ref: Vec3::unit_x(),
    };
    let off = arc(
        Point3::origin(),
        Vec3::unit_x(),
        Vec3::unit_y(),
        1.3 + 1e-3,
        0.2,
        1.5,
    );
    let err = super::super::chart_pcurve(&off, &torus, b).expect_err("off the torus");
    assert!(
        matches!(err, PcurveCertifyError::CarrierOffChart { .. }),
        "{err:?}"
    );
    let _ = Tol::witness();
}

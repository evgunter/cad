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
use super::super::{EnvelopeTerm, Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck};
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
/// dropped or under-stated. The two conditions that refuse rather than
/// bound are covered with them: a piece grazing its branch's half-plane
/// refuses its sector, and a wrong nappe refuses on the cone's lever.
/// Pinned: a coverage witness, not a search.
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
    assert!(
        matches!(
            grazing_piece_refusal(false),
            PcurveCertifyError::SectorRefused {
                channel: SectorChannel::Azimuth,
                ..
            }
        ),
        "the sector condition is load-bearing"
    );
    assert!(
        matches!(
            wrong_nappe_refusal(),
            PcurveCertifyError::SectorRefused {
                channel: SectorChannel::Lever,
                ..
            }
        ),
        "the cone's lever is load-bearing"
    );
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
    // The piece holding the pole refuses its sector, definitely or, where
    // the refinement leaves its margin within the band (a coarse ε), by
    // escalating that same decision.
    let polar = Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.3, 0.9, 0.0).normalize(),
        radius: 1.0,
        u_ref: Vec3::unit_z(),
    };
    let err = project(&polar, Some((-0.5, 0.5)), &sphere, b).expect_err("through the pole");
    assert!(
        matches!(
            err,
            PcurveCertifyError::SectorRefused { .. }
                | PcurveCertifyError::Escalated {
                    check: PcurveCheck::Sector,
                    ..
                }
        ),
        "{err:?}"
    );
    let _ = FRAC_PI_2;
}

/// A cone rim's image with its nappe turned to the other one, and every
/// branch centre turned with it (the nappe flips the radial part, so a
/// half turn puts each piece back in its sector): the image is a valid
/// branch, and only the cone's lever, `ρ·cos α + (σz)·sin α`, is read
/// on the wrong nappe.
fn wrong_nappe_refusal() -> PcurveCertifyError {
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
    for m in &mut image.azimuth {
        *m += 2;
    }
    let lane = crate::FittedLane::<Interval>::certified();
    let (t0, t1) = (Interval::from_f64(0.0), Interval::from_f64(1.0));
    PcurveCache::certify_projected(image, t0, t1, &rim, &cone, b, Some(lane))
        .expect_err("the wrong nappe")
}

/// **The cone's lever refuses typed, and on the lever**: a stored image
/// on the wrong nappe with its branch centres turned to hold their
/// sectors ([`wrong_nappe_refusal`]). At the interval scalar, where no
/// schedule runs ahead of the envelope.
#[test]
fn the_wrong_nappe_refuses_on_the_cone_lever() {
    let err = wrong_nappe_refusal();
    assert!(
        matches!(
            err,
            PcurveCertifyError::SectorRefused {
                channel: SectorChannel::Lever,
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

/// Rational quadratic arcs of the circle `c + r·(cos θ·e₁ + sin θ·e₂)`,
/// one per entry of `spans` (each `≤ 2π/3`), joined at double knots on
/// the parameter `[0, 1]`: one carrier sweeping their sum.
fn arcs(
    c: Point3<f64>,
    e1: Vec3<f64>,
    e2: Vec3<f64>,
    r: f64,
    a: f64,
    spans: &[f64],
) -> Curve3<f64> {
    let at = |th: f64| c + (e1 * th.cos() + e2 * th.sin()) * r;
    let mut ctl = vec![at(a)];
    let mut w = vec![1.0];
    let mut knots = vec![0.0, 0.0, 0.0];
    let mut from = a;
    for (j, span) in spans.iter().enumerate() {
        let h = 0.5 * span;
        ctl.push(c + (e1 * (from + h).cos() + e2 * (from + h).sin()) * (r / h.cos()));
        w.push(h.cos());
        from += span;
        ctl.push(at(from));
        w.push(1.0);
        if j + 1 < spans.len() {
            #[allow(clippy::cast_precision_loss)]
            let k = (j + 1) as f64 / spans.len() as f64;
            knots.extend([k, k]);
        }
    }
    knots.extend([1.0, 1.0, 1.0]);
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(KnotVector::clamped(knots, 2).unwrap(), ctl, w).unwrap(),
    ))
}

fn unit_sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// A tilted circle on the sphere of radius `r` about the origin, its
/// plane at `d` from the centre along `n` (`tilt` from the axis).
fn tilted_on(r: f64, tilt: f64, d: f64) -> Curve3<f64> {
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    Curve3::Circle {
        center: Point3::origin() + n * d,
        axis: n,
        radius: (r * r - d * d).sqrt(),
        u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
    }
}

/// The envelope of `c` over `[t0, t1]` on `surf`, derived and certified
/// at `f64` and at `Interval`.
fn certify_both(
    c: &Curve3<f64>,
    surf: &Surface<f64>,
    t0: f64,
    t1: f64,
) -> [Result<f64, PcurveCertifyError>; 2] {
    let b = band();
    let lane = crate::FittedLane::<f64>::certified();
    let at_f64 = super::super::chart_pcurve_over(c, t0, t1, surf, b).and_then(|image| {
        let Pcurve::Projected(p) = image else {
            unreachable!("a carrier with no closed form")
        };
        PcurveCache::certify_projected(*p, t0, t1, c, surf, b, Some(lane))
            .map(|cache| cache.certificate().envelope)
    });
    let (ci, si) = (lift(c), surf.map_scalar(Interval::from_f64));
    let (i0, i1) = (Interval::from_f64(t0), Interval::from_f64(t1));
    let lane = crate::FittedLane::<Interval>::certified();
    let at_iv = super::super::chart_pcurve_over(&ci, i0, i1, &si, b).and_then(|image| {
        let Pcurve::Projected(p) = image else {
            unreachable!("a carrier with no closed form")
        };
        PcurveCache::certify_projected(*p, i0, i1, &ci, &si, b, Some(lane))
            .map(|cache| cache.certificate().envelope.hi())
    });
    [at_f64, at_iv]
}

/// **The period gate reads the sweep, not the piece boxes** (check 2).
/// A tilted circle that encircles the sphere's axis certifies over 0.7,
/// 0.8, 0.95 and a whole turn, at `f64` and at `Interval`, and so does a
/// whole turn of one that does not; a sweep past a turn refuses, on a
/// circle and on a net that winds 1.2 times around a cylinder.
#[test]
fn the_period_gate_reads_the_sweep_and_refuses_past_a_turn() {
    let sphere = unit_sphere();
    for (c, t1) in [
        (tilted_on(1.0, 0.3, 0.2), 0.7 * TAU),
        (tilted_on(1.0, 0.3, 0.2), 0.8 * TAU),
        (tilted_on(1.0, 0.3, 0.2), 0.95 * TAU),
        (tilted_on(1.0, 0.3, 0.2), TAU),
        (tilted_on(1.0, 1.2, 0.5), TAU),
    ] {
        for (scalar, got) in ["f64", "Interval"]
            .into_iter()
            .zip(certify_both(&c, &sphere, 0.0, t1))
        {
            let env = got.unwrap_or_else(|e| panic!("{scalar}, {t1}: {e:?}"));
            assert!(env <= band().zero(), "{scalar}, {t1}: envelope {env:e}");
        }
    }
    let past = tilted_on(1.0, 0.3, 0.2);
    let [got, _] = certify_both(&past, &sphere, 0.0, 1.05 * TAU);
    assert!(
        matches!(got, Err(PcurveCertifyError::AzimuthPeriodExceeded)),
        "a circle past a turn: {got:?}"
    );
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let winding = arcs(
        Point3::origin(),
        Vec3::unit_x(),
        Vec3::unit_y(),
        1.0,
        0.1,
        &[1.2 * TAU / 5.0; 5],
    );
    let [got, _] = certify_both(&winding, &cylinder, 0.0, 1.0);
    assert!(
        matches!(got, Err(PcurveCertifyError::AzimuthPeriodExceeded)),
        "a net winding 1.2 turns: {got:?}"
    );
}

/// **An edge reads only its own interval of the net** (C4: a span
/// reaching a pole refuses; the net's geometry past the edge's ends is
/// not the edge's). A great-circle arc whose far part runs over the
/// sphere's pole: an edge on its first fifth derives and certifies, and
/// the whole net refuses on the pole.
#[test]
fn an_edge_reads_only_its_own_interval_of_the_net() {
    let sphere = unit_sphere();
    let c = arc(
        Point3::origin(),
        Vec3::unit_x(),
        Vec3::new(0.0, 0.0, -1.0),
        1.0,
        0.6,
        1.4,
    );
    for (scalar, got) in ["f64", "Interval"]
        .into_iter()
        .zip(certify_both(&c, &sphere, 0.0, 0.2))
    {
        let env = got.unwrap_or_else(|e| panic!("{scalar}: the edge clear of the pole: {e:?}"));
        assert!(env <= band().zero(), "{scalar}: envelope {env:e}");
    }
    let [whole, _] = certify_both(&c, &sphere, 0.0, 1.0);
    assert!(
        matches!(
            whole,
            Err(PcurveCertifyError::SectorRefused { .. }
                | PcurveCertifyError::Escalated {
                    check: PcurveCheck::Sector,
                    ..
                })
        ),
        "the whole net over the pole: {whole:?}"
    );
}

/// The unit quarter circle in `z = 0` as 16 rational quadratic
/// sub-arcs joined at double knots `j/16`, every control from the
/// ninth sub-arc on (`t > ½`) scaled radially by `1 + h`: on the unit
/// cylinder over `[0, ½]`, and the circle of radius `1 + h` — exactly
/// `h` off it — over `(½, 1]`.
fn quarter_off_past_half(h: f64) -> Curve3<f64> {
    let n = 16_usize;
    #[allow(clippy::cast_precision_loss)]
    let span = FRAC_PI_2 / n as f64;
    let half = 0.5 * span;
    let mut ctl = vec![Point3::new(1.0, 0.0, 0.0)];
    let mut w = vec![1.0];
    let mut knots = vec![0.0, 0.0, 0.0];
    for j in 0..n {
        #[allow(clippy::cast_precision_loss)]
        let a = span * j as f64;
        let m = a + half;
        ctl.push(Point3::new(m.cos() / half.cos(), m.sin() / half.cos(), 0.0));
        w.push(half.cos());
        ctl.push(Point3::new((a + span).cos(), (a + span).sin(), 0.0));
        w.push(1.0);
        if j + 1 < n {
            #[allow(clippy::cast_precision_loss)]
            let k = (j + 1) as f64 / n as f64;
            knots.extend([k, k]);
        }
    }
    knots.extend([1.0, 1.0, 1.0]);
    // Control 16 is `C(½)`, shared by the eighth and ninth sub-arcs.
    for p in &mut ctl[17..] {
        *p = Point3::new(p.x * (1.0 + h), p.y * (1.0 + h), 0.0);
    }
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(KnotVector::clamped(knots, 2).unwrap(), ctl, w).unwrap(),
    ))
}

/// **A spline edge's incidence reads only the knot spans its interval
/// meets** (`net_incidence`'s `met`). A carrier on the cylinder over
/// `[0, ½]` and `h` off it past `½`: the edge over `[0, ½]` certifies
/// inside the band at `f64` and `Interval`, and the edge over `[0, 1]`
/// reads the offset, or refuses.
#[test]
fn a_spline_edges_incidence_reads_only_the_spans_its_interval_meets() {
    let h = 1e-4;
    let c = quarter_off_past_half(h);
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    for (scalar, got) in ["f64", "Interval"]
        .into_iter()
        .zip(certify_both(&c, &cylinder, 0.0, 0.5))
    {
        let env = got.unwrap_or_else(|e| panic!("{scalar}: the edge on the cylinder: {e:?}"));
        assert!(env <= band().zero(), "{scalar}: envelope {env:e}");
    }
    for (scalar, got) in ["f64", "Interval"]
        .into_iter()
        .zip(certify_both(&c, &cylinder, 0.0, 1.0))
    {
        assert!(
            got.as_ref().is_err() || got.as_ref().is_ok_and(|&env| env >= h * (1.0 - 1e-6)),
            "{scalar}: the whole net, {h:e} off past ½: {got:?}"
        );
    }
}

/// A carrier at distance exactly `delta` from `surf`, on each chart:
/// the plane's segment, a cylinder's, sphere's and torus's parallel
/// circles moved off along the normal, a cone rim moved off its
/// generator (its arc near azimuth 45°, where a radial floor read off
/// the box's corners is least tight).
fn parallel(chart: Chart, delta: f64) -> (Surface<f64>, Curve3<f64>) {
    let (z, x, y) = (Vec3::unit_z(), Vec3::unit_x(), Vec3::unit_y());
    match chart {
        Chart::Plane => (
            Surface::Plane {
                origin: Point3::origin(),
                normal: z,
                u_ref: x,
            },
            segment(Point3::new(0.1, 0.2, delta), Point3::new(1.3, -0.4, delta)),
        ),
        Chart::Cylinder => (
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: z,
                radius: 1.3,
                u_ref: x,
            },
            arc(Point3::new(0.0, 0.0, 0.2), x, y, 1.3 + delta, 0.4, 1.5),
        ),
        Chart::Sphere => {
            let (r, phi) = (1.1 + delta, 0.5_f64);
            (
                Surface::Sphere {
                    center: Point3::origin(),
                    radius: 1.1,
                    axis: z,
                    u_ref: x,
                },
                arc(
                    Point3::new(0.0, 0.0, r * phi.sin()),
                    x,
                    y,
                    r * phi.cos(),
                    0.4,
                    1.5,
                ),
            )
        }
        Chart::Cone => {
            let alpha = 0.6_f64;
            let z0 = 1.0;
            (
                Surface::Cone {
                    apex: Point3::origin(),
                    axis: z,
                    half_angle: alpha,
                    u_ref: x,
                },
                arc(
                    Point3::new(0.0, 0.0, z0),
                    x,
                    y,
                    z0 * alpha.tan() + delta / alpha.cos(),
                    0.7,
                    0.17,
                ),
            )
        }
        Chart::Torus => {
            let (big, small, v) = (1.2, 0.4_f64, 0.7_f64);
            (
                Surface::Torus {
                    center: Point3::origin(),
                    axis: z,
                    major_radius: big,
                    minor_radius: small,
                    u_ref: x,
                },
                arc(
                    Point3::new(0.0, 0.0, (small + delta) * v.sin()),
                    x,
                    y,
                    big + (small + delta) * v.cos(),
                    0.4,
                    1.5,
                ),
            )
        }
    }
}

/// **Each chart's metre conversion is tight**: a carrier at distance
/// exactly `δ` from the chart has an incidence term in `[δ, 1.5·δ]` —
/// sound, and within a factor well under 2 of the truth, so a halved or
/// a loosened conversion goes red. On every chart, and for a tilted
/// circle `δ` off a sphere through its closed form.
#[test]
fn each_charts_conversion_is_tight_on_a_parallel_carrier() {
    let b = band();
    let lane = crate::FittedLane::<f64>::certified();
    for chart in CHARTS {
        for delta in [1e-6, 1e-4] {
            let (surf, c) = parallel(chart, delta);
            let image = project(&c, None, &surf, b).unwrap_or_else(|e| panic!("{chart:?}: {e:?}"));
            let pcurve = Pcurve::Projected(Box::new(image.clone()));
            let boxed = pcurve.chart_box(0.0, 1.0);
            let terms = projected_envelope(&image, 0.0, 1.0, &boxed, &c, &surf, b, Some(lane))
                .unwrap_or_else(|e| panic!("{chart:?}: {e:?}"));
            let incidence = terms.0[EnvelopeTerm::Incidence.slot()];
            assert!(
                incidence >= delta * (1.0 - 1e-6) && incidence <= 1.5 * delta,
                "{chart:?} δ {delta:e}: incidence {incidence:e}"
            );
            let sup = sampled(&pcurve, &c, &surf, 0.0, 1.0);
            assert!(terms.total() >= sup * (1.0 - 1e-9), "{chart:?} δ {delta:e}");
        }
    }
    let sphere = unit_sphere();
    for delta in [1e-6, 1e-4] {
        let c = tilted_on(1.0 + delta, 0.7, 0.3);
        let image = project(&c, Some((0.3, 2.1)), &sphere, b).expect("clear of the poles");
        let pcurve = Pcurve::Projected(Box::new(image.clone()));
        let boxed = pcurve.chart_box(0.3, 2.1);
        let terms = projected_envelope(&image, 0.3, 2.1, &boxed, &c, &sphere, b, None)
            .expect("a circle reads no door");
        let incidence = terms.0[EnvelopeTerm::Incidence.slot()];
        assert!(
            incidence >= delta * (1.0 - 1e-6) && incidence <= 1.5 * delta,
            "a circle δ {delta:e} off the sphere: incidence {incidence:e}"
        );
    }
}

/// **The conversions read the floors they state** (white box): on a
/// hull of one piece and one part, each chart's incidence is exactly its
/// formula on the part's own floors — the cone's lever from the twin's
/// angle on the stored nappe, `(σz)_min` read on that nappe — and the
/// Lipschitz floor is the part's `ρ_min` less the stored net's distance;
/// a lever on the wrong side refuses on the lever, and an uncertified
/// part refuses.
#[test]
fn each_conversion_reads_its_stated_floors() {
    use super::super::projected::{
        PieceHull, ProjectedChart, ProjectedHull, SpanHull, net_incidence,
    };
    let b = band();
    let (f, d) = (1e-3, 0.25);
    let hull = |rho_lo: f64, z: (f64, f64)| ProjectedHull {
        pieces: vec![PieceHull {
            x_lo: 1.0,
            tube_lo: 1.0,
        }],
        spans: vec![SpanHull {
            range: (0.0, 1.0),
            f_sup: f,
            rho_lo,
            z,
        }],
    };
    let close = |got: f64, want: f64| (got - want).abs() <= 1e-12 * want.abs();
    let alpha = 0.6_f64;
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    };
    // The stored scalars are deliberately wrong: the lever reads the
    // twin's angle, not the row's.
    let stored = |nappe| ProjectedChart::Cone {
        sin: 0.0,
        cos: 0.0,
        nappe,
    };
    let (inc, floor) = net_incidence(
        &hull(1.0, (0.5, 2.0)),
        (0.0, 1.0),
        &stored(crate::Nappe::Opening),
        &cone,
        d,
        b,
    )
    .expect("a positive lever");
    let lever = alpha.cos() + 0.5 * alpha.sin();
    assert!(
        close(inc, f / lever) && close(floor, 1.0 - d),
        "cone: {inc:e}, {floor}"
    );
    let mirror = net_incidence(
        &hull(0.1, (0.5, 2.0)),
        (0.0, 1.0),
        &stored(crate::Nappe::Mirror),
        &cone,
        d,
        b,
    );
    assert!(
        matches!(
            mirror,
            Err(PcurveCertifyError::SectorRefused {
                channel: SectorChannel::Lever,
                ..
            })
        ),
        "{mirror:?}"
    );
    let rho = 0.9;
    for (surf, want) in [
        (
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            f / (1.0 + rho.max(1.0 - f)),
        ),
        (
            Surface::Sphere {
                center: Point3::origin(),
                radius: 1.0,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            f / (1.0 + rho.max(1.0 - f)),
        ),
        (
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: 1.2,
                minor_radius: 0.4,
                u_ref: Vec3::unit_x(),
            },
            {
                let outer = (rho + 1.2_f64).powi(2) - 0.4_f64.powi(2);
                let first = f / (0.4 * outer);
                f / ((0.4 + (0.4 - first).max(0.0)) * outer)
            },
        ),
    ] {
        let chart = match surf {
            Surface::Torus { major_radius, .. } => ProjectedChart::Torus {
                major: major_radius,
            },
            Surface::Sphere { .. } => ProjectedChart::Sphere,
            _ => ProjectedChart::Cylinder,
        };
        let (inc, floor) = net_incidence(&hull(rho, (0.0, 0.0)), (0.0, 1.0), &chart, &surf, d, b)
            .unwrap_or_else(|e| panic!("{surf:?}: {e:?}"));
        assert!(close(inc, want), "{surf:?}: {inc:e} against {want:e}");
        assert!(close(floor, rho - d), "{surf:?}: floor {floor}");
    }
    let mut uncertified = hull(1.0, (0.5, 2.0));
    uncertified.spans[0].f_sup = f64::NAN;
    let got = net_incidence(
        &uncertified,
        (0.0, 1.0),
        &stored(crate::Nappe::Opening),
        &cone,
        d,
        b,
    );
    assert!(
        matches!(got, Err(PcurveCertifyError::ImageMismatch { .. })),
        "{got:?}"
    );
}

/// **The stored chart scalars, frame and deck are bounded**: a row whose
/// stored cone angle, torus major radius, circle frame or `v` offset is
/// moved off the chart's dominates its sampled displacement, so a term
/// that dropped any of them goes red.
#[test]
fn moved_stored_scalars_are_dominated() {
    use super::super::projected::ProjectedChart;
    let b = band();
    let lane = crate::FittedLane::<f64>::certified();
    // The cone's stored angle moves its foot along the generator at
    // second order on the cone (the slant is stationary in the angle
    // there), so its move is the larger one.
    let (eta, cone_eta) = (1e-6, 1e-3);
    let check = |what: &str,
                 image: super::super::projected::ProjectedImage<f64>,
                 c: &Curve3<f64>,
                 surf: &Surface<f64>,
                 t0: f64,
                 t1: f64| {
        let pcurve = Pcurve::Projected(Box::new(image.clone()));
        let boxed = pcurve.chart_box(t0, t1);
        let env = projected_envelope(&image, t0, t1, &boxed, c, surf, b, Some(lane))
            .unwrap_or_else(|e| panic!("{what}: {e:?}"))
            .total();
        let sup = sampled(&pcurve, c, surf, t0, t1);
        assert!(sup > 1e-8, "{what}: the move is real ({sup:e})");
        assert!(
            env >= sup * (1.0 - 1e-9),
            "{what}: envelope {env:e} under {sup:e}"
        );
    };
    let (surf, c) = parallel(Chart::Cone, 0.0);
    let mut image = project(&c, None, &surf, b).unwrap();
    if let ProjectedChart::Cone {
        ref mut sin,
        ref mut cos,
        ..
    } = image.chart
    {
        (*sin, *cos) = (0.6 + cone_eta).sin_cos();
    }
    check("the cone's stored angle", image, &c, &surf, 0.0, 1.0);
    let (surf, c) = parallel(Chart::Torus, 0.0);
    let mut image = project(&c, None, &surf, b).unwrap();
    if let ProjectedChart::Torus { ref mut major } = image.chart {
        *major += eta;
    }
    check(
        "the torus's stored major radius",
        image,
        &c,
        &surf,
        0.0,
        1.0,
    );
    let (surf, c) = parallel(Chart::Cylinder, 0.0);
    let mut image = project(&c, None, &surf, b).unwrap();
    image.v_off += eta;
    check("the cylinder's v offset", image, &c, &surf, 0.0, 1.0);
    let sphere = unit_sphere();
    let c = tilted_on(1.0, 0.7, 0.3);
    let mut image = project(&c, Some((0.3, 2.1)), &sphere, b).unwrap();
    if let FramedCarrier::Circle { ref mut centre, .. } = image.carrier {
        *centre = *centre + Vec3::new(eta, -eta, eta);
    }
    check("the circle's stored centre", image, &c, &sphere, 0.3, 2.1);
}

/// A net's one-piece image (a 120° arc in one knot span) with its
/// branch centre turned a half period: `atan2`'s cut then lies inside
/// the piece, where the image jumps a period, which `S` cannot see. On
/// a torus the arc is a meridian and the turned centre the tube angle's.
fn turned_piece_refusal(torus: bool) -> PcurveCertifyError {
    let b = band();
    let sweep = 120f64.to_radians();
    let (surf, c) = if torus {
        (
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: 1.2,
                minor_radius: 0.4,
                u_ref: Vec3::unit_x(),
            },
            arc(
                Point3::new(1.2, 0.0, 0.0),
                Vec3::unit_x(),
                Vec3::unit_z(),
                0.4,
                -0.5 * sweep,
                sweep,
            ),
        )
    } else {
        (
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            arc(
                Point3::origin(),
                Vec3::unit_x(),
                Vec3::unit_y(),
                1.0,
                -0.5 * sweep,
                sweep,
            ),
        )
    };
    let mut image = project(&c, None, &surf, b).expect("on the chart");
    assert_eq!(image.pieces(), 1, "a 120° arc holds one sector");
    if torus {
        image.tube[0] += 2;
    } else {
        image.azimuth[0] += 2;
    }
    let lane = crate::FittedLane::<f64>::certified();
    PcurveCache::certify_projected(image, 0.0, 1.0, &c, &surf, b, Some(lane))
        .expect_err("the cut inside the piece")
}

/// A net's image merged onto one branch centre whose half-plane its
/// arc grazes: a 120° arc in one knot span starting `1e-14` short of the
/// branch's boundary (the azimuth's, or on a torus the tube angle's on a
/// meridian), imaged on several pieces and then stored on one. The
/// image is continuous and its sweep is a third of a turn, so only the
/// sector condition, decided against the band, refuses it.
fn grazing_piece_refusal(torus: bool) -> PcurveCertifyError {
    let b = band();
    let (from, sweep) = (-FRAC_PI_2 + 1e-14, 120f64.to_radians());
    let (surf, c) = if torus {
        (
            Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::unit_z(),
                major_radius: 1.2,
                minor_radius: 0.4,
                u_ref: Vec3::unit_x(),
            },
            arc(
                Point3::new(1.2, 0.0, 0.0),
                Vec3::unit_x(),
                Vec3::unit_z(),
                0.4,
                from,
                sweep,
            ),
        )
    } else {
        (
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            arc(
                Point3::origin(),
                Vec3::unit_x(),
                Vec3::unit_y(),
                1.0,
                from,
                sweep,
            ),
        )
    };
    let mut image = project(&c, None, &surf, b).expect("on the chart");
    image.breaks = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    image.azimuth = vec![0];
    if torus {
        image.tube = vec![0];
    }
    let lane = crate::FittedLane::<f64>::certified();
    PcurveCache::certify_projected(image, 0.0, 1.0, &c, &surf, b, Some(lane))
        .expect_err("a piece grazing its half-plane")
}

/// **A piece grazing its half-plane refuses its sector**, on the
/// azimuth and on a torus's tube angle: the one row only the sector
/// condition reads (a piece across the cut is also over the period).
#[test]
fn a_piece_grazing_its_half_plane_refuses_its_sector() {
    for (torus, channel) in [(false, SectorChannel::Azimuth), (true, SectorChannel::Tube)] {
        let err = grazing_piece_refusal(torus);
        assert!(
            matches!(err, PcurveCertifyError::SectorRefused { channel: c, .. } if c == channel),
            "{channel:?}: {err:?}"
        );
    }
}

/// **A τ jump inside a piece, and one at a break, refuse.** A piece
/// whose branch centre puts the azimuth's cut inside it, or the tube
/// angle's on a torus, refuses its sector (the image jumps a period
/// there and `S` cannot see it); a last piece turned a whole period
/// refuses the branch at the break, though each piece holds its own
/// sector.
#[test]
fn a_period_jump_inside_a_piece_or_at_a_break_refuses() {
    // The cut inside the piece: a period gate reads the jump first, a
    // sweep of a whole period, and the sector refuses it where no gate
    // runs first ([`a_piece_grazing_its_half_plane_refuses_its_sector`]).
    for torus in [false, true] {
        let err = turned_piece_refusal(torus);
        assert!(
            matches!(
                err,
                PcurveCertifyError::AzimuthPeriodExceeded
                    | PcurveCertifyError::TubePeriodExceeded
                    | PcurveCertifyError::SectorRefused {
                        channel: SectorChannel::Azimuth | SectorChannel::Tube,
                        ..
                    }
            ),
            "torus {torus}: {err:?}"
        );
    }
    let b = band();
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    // A sweep that DEcreases, so the last piece turned a whole period up
    // lands a whole period above the joint it leaves, and the image's
    // sweep reads exactly a period: the gate lets it pass, and only the
    // branch condition reads the jump.
    let c = arcs(
        Point3::origin(),
        Vec3::unit_x(),
        Vec3::new(0.0, -1.0, 0.0),
        1.0,
        0.3,
        &[1.0, 1.0],
    );
    let image = project(&c, None, &cylinder, b).expect("on the cylinder");
    let mut turned = image.clone();
    *turned.azimuth.last_mut().unwrap() += 4;
    let lane = crate::FittedLane::<f64>::certified();
    let err = PcurveCache::certify_projected(turned, 0.0, 1.0, &c, &cylinder, b, Some(lane))
        .expect_err("a whole-turn jump at a break");
    assert!(
        matches!(
            err,
            PcurveCertifyError::SectorRefused {
                channel: SectorChannel::Branch,
                ..
            }
        ),
        "{err:?}"
    );
}

/// **The plane's envelope dominates**: a cubic whose middle controls
/// leave the plane, against its densely sampled distance from it.
#[test]
fn the_plane_envelope_dominates_a_carrier_off_the_plane() {
    let b = band();
    let plane = Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let h = 1e-3;
    let c = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap(),
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.3, 0.2, h),
                Point3::new(0.7, -0.1, h),
                Point3::new(1.0, 0.1, 0.0),
            ],
            vec![1.0; 4],
        )
        .unwrap(),
    ));
    let image = project(&c, None, &plane, b).expect("a plane takes every net");
    let pcurve = Pcurve::Projected(Box::new(image.clone()));
    let boxed = pcurve.chart_box(0.0, 1.0);
    let env = projected_envelope(&image, 0.0, 1.0, &boxed, &c, &plane, b, None)
        .expect("the plane reads no door")
        .total();
    let dense = (0..=2048)
        .map(|i| {
            let t = f64::from(i) / 2048.0;
            let q = pcurve.eval(t);
            plane.eval(q.x, q.y).distance(c.eval(t))
        })
        .fold(0.0, f64::max);
    assert!(
        env >= dense && dense > 0.5 * h,
        "envelope {env:e}, dense {dense:e}"
    );
}

/// **The cone's lever reads the smallest `σz` of a part** (behaviour,
/// not formula). A segment near the apex whose distance from the cone
/// falls from `d` to zero across the first incidence part while the
/// lever `ρ cos α + σz sin α` grows thirteenfold: `|f| = distance ·
/// lever` peaks inside the part, so a lever read at the part's largest
/// `σz` divides the peak by about half the part's growth. The incidence
/// term of the edge over that part alone — the carrier's distance from
/// the chart — dominates the sampled displacement, `d` at its start.
#[test]
fn a_cone_carrier_leaving_the_apex_is_dominated() {
    let b = band();
    let lane = crate::FittedLane::<f64>::certified();
    let alpha = 0.6_f64;
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    };
    let generator = Vec3::new(alpha.sin(), 0.0, alpha.cos());
    let normal = Vec3::new(alpha.cos(), 0.0, -alpha.sin());
    let d = 1e-3;
    // On the cone at `t = 1/8`, the end of the first of the knot span's
    // `INCIDENCE_CUTS` parts.
    let c = segment(
        Point3::origin() + generator * 0.02 + normal * d,
        Point3::origin() + generator * 2.02 - normal * (7.0 * d),
    );
    let (t0, t1) = (0.0, 0.125);
    let image = project(&c, None, &cone, b).unwrap();
    let pcurve = Pcurve::Projected(Box::new(image.clone()));
    let sup = sampled(&pcurve, &c, &cone, t0, t1);
    assert!((sup - d).abs() < 1e-9, "the start is {d:e} off: {sup:e}");
    let boxed = pcurve.chart_box(t0, t1);
    let terms = projected_envelope(&image, t0, t1, &boxed, &c, &cone, b, Some(lane))
        .expect("the lever is positive on the nappe");
    let incidence = terms.0[EnvelopeTerm::Incidence.slot()];
    assert!(
        incidence >= sup * (1.0 - 1e-9),
        "incidence {incidence:e} under the displacement {sup:e}"
    );
}

/// **A part's uncertified bound refuses where the edge reads it, and
/// only there** (behaviour, on the lane's own hull). The hull of a
/// spline on the cylinder; a part's composite bound or floor made NaN
/// inside the edge's interval refuses, and the same past the edge's
/// end is not the edge's.
#[test]
fn an_uncertified_part_refuses_only_inside_the_edges_interval() {
    let b = band();
    let lane = crate::FittedLane::<f64>::certified();
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let c = quarter_off_past_half(0.0);
    let image = project(&c, None, &cylinder, b).unwrap();
    let FramedCarrier::Net(net) = &image.carrier else {
        unreachable!("a spline carrier")
    };
    let twin = super::super::orthonormal_chart(&cylinder);
    let hull = lane.projected_hull(&image, net, &twin).unwrap();
    let edge = (0.0, 0.5);
    let read = |hull: &super::super::projected::ProjectedHull| {
        super::super::projected::net_incidence(hull, edge, &image.chart, &twin, 0.0, b)
    };
    let (clean, _) = read(&hull).expect("the clean hull");
    assert!(clean <= b.zero(), "{clean:e}");
    let inside = hull.spans.iter().position(|s| s.range.1 <= 0.5).unwrap();
    let past = hull.spans.iter().position(|s| s.range.0 >= 0.5).unwrap();
    for poison in [
        |s: &mut super::super::projected::SpanHull| s.f_sup = f64::NAN,
        |s: &mut super::super::projected::SpanHull| s.rho_lo = f64::NAN,
    ] {
        let mut h = hull.clone();
        poison(&mut h.spans[past]);
        read(&h).expect("an uncertified part past the edge's end is not the edge's");
        let mut h = hull.clone();
        poison(&mut h.spans[inside]);
        let got = read(&h);
        assert!(
            matches!(got, Err(PcurveCertifyError::ImageMismatch { .. })),
            "an uncertified part the edge reads: {got:?}"
        );
    }
}

/// **A met piece's uncertified sector bound escalates** (behaviour, on
/// the lane's own hull). The hull of a spline on the torus; each
/// piece's azimuth and tube bounds decide; either made NaN — what the
/// hull lane reads for a bracket that does not certify — escalates the
/// sector check rather than passing it.
#[test]
fn an_uncertified_sector_bound_escalates() {
    use super::super::projected::hull_sector;
    let b = band();
    let lane = crate::FittedLane::<f64>::certified();
    let (torus, c) = parallel(Chart::Torus, 0.0);
    let image = project(&c, None, &torus, b).unwrap();
    let FramedCarrier::Net(net) = &image.carrier else {
        unreachable!("a spline carrier")
    };
    let twin = super::super::orthonormal_chart(&torus);
    let hull = lane.projected_hull(&image, net, &twin).unwrap();
    for (k, piece) in hull.pieces.iter().enumerate() {
        hull_sector(piece, k, &image.chart, b).expect("the clean hull's sectors hold");
    }
    for poison in [
        |p: &mut super::super::projected::PieceHull| p.x_lo = f64::NAN,
        |p: &mut super::super::projected::PieceHull| p.tube_lo = f64::NAN,
    ] {
        let mut piece = hull.pieces[0];
        poison(&mut piece);
        let got = hull_sector(&piece, 0, &image.chart, b);
        assert!(
            matches!(
                got,
                Err(PcurveCertifyError::Escalated {
                    check: PcurveCheck::Sector,
                    ..
                })
            ),
            "{got:?}"
        );
    }
}

/// **A corrupt net escalates its sector wherever the bad control's
/// support reaches** (behaviour, through the lane, at the `f64` lane's
/// scalar). The torus spline's stored net with one control's coordinate
/// poison, which the lane's crossing into certification arithmetic
/// refuses: the piece it lies on escalates its sector check, whichever
/// channel the poison sits in.
#[test]
fn the_hull_lane_reads_an_uncertified_control_as_escalating() {
    use super::super::projected::hull_sector;
    let b = band();
    let lane = crate::FittedLane::<f64>::certified();
    let (torus, c) = parallel(Chart::Torus, 0.0);
    let mut image = project(&c, None, &torus, b).unwrap();
    let FramedCarrier::Net(net) = &image.carrier else {
        unreachable!("a spline carrier")
    };
    let mut control = net.control().to_vec();
    control[1].x = f64::NAN;
    let corrupt = NurbsCurve3::new(net.knots().clone(), control, net.weights().to_vec()).unwrap();
    image.carrier = FramedCarrier::Net(Arc::new(corrupt.clone()));
    let twin = super::super::orthonormal_chart(&torus);
    let hull = lane.projected_hull(&image, &corrupt, &twin).unwrap();
    let got = hull_sector(&hull.pieces[0], 0, &image.chart, b);
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::Escalated {
                check: PcurveCheck::Sector,
                ..
            })
        ),
        "{got:?}"
    );
}

/// **A corrupt net escalates the azimuth sector off the torus**
/// (behaviour, through the lane, at the `f64` lane's scalar). The
/// cylinder spline's stored net with one control poison in every
/// coordinate: the piece it lies on reads no tube channel, so its
/// azimuth bound alone must escalate, not clear the sector.
#[test]
fn a_corrupt_net_escalates_the_azimuth_sector_on_a_cylinder() {
    use super::super::projected::hull_sector;
    let b = band();
    let lane = crate::FittedLane::<f64>::certified();
    let cylinder = Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let c = quarter_off_past_half(0.0);
    let mut image = project(&c, None, &cylinder, b).unwrap();
    let FramedCarrier::Net(net) = &image.carrier else {
        unreachable!("a spline carrier")
    };
    let mut control = net.control().to_vec();
    control[1] = Point3::new(f64::NAN, f64::NAN, f64::NAN);
    let corrupt = NurbsCurve3::new(net.knots().clone(), control, net.weights().to_vec()).unwrap();
    image.carrier = FramedCarrier::Net(Arc::new(corrupt.clone()));
    let twin = super::super::orthonormal_chart(&cylinder);
    let hull = lane.projected_hull(&image, &corrupt, &twin).unwrap();
    assert!(hull.pieces[0].x_lo.is_nan(), "{:?}", hull.pieces[0]);
    let got = hull_sector(&hull.pieces[0], 0, &image.chart, b);
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::Escalated {
                check: PcurveCheck::Sector,
                ..
            })
        ),
        "{got:?}"
    );
}

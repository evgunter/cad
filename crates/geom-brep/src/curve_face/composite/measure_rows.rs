//! A one-shot measurement: the table's ring-composite arm
//! ([`crate::curve_face::cross()`]) against the interval composite
//! ([`super::crossings`]) on families of spline spans against every
//! analytic form, at three tolerances. Run with
//! `cargo nextest run -p geom-brep --run-ignored only -E 'test(composite_arm_against_the_interval_composite)' --no-capture`.

#![allow(clippy::unwrap_used, clippy::panic, clippy::cast_precision_loss)]

use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Interval, Point3, Vec3};

use crate::curve_face::{EndReading, OnSpan, cross};
use crate::curve_pair::Span;

struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next()
    }
}

fn frame(o: Point3<f64>) -> geom::Frame<f64> {
    geom::Frame::new(o, Vec3::unit_z(), Vec3::unit_x())
}

fn surfaces() -> Vec<(&'static str, Surface<f64>)> {
    vec![
        ("plane", Surface::Plane { frame: frame(Point3::origin()) }),
        (
            "cylinder",
            Surface::Cylinder {
                frame: frame(Point3::new(0.5, 0.0, 0.0)),
                radius: 0.4,
            },
        ),
        (
            "sphere",
            Surface::Sphere {
                frame: frame(Point3::new(0.5, 0.0, 0.0)),
                radius: 0.45,
            },
        ),
        (
            "cone",
            Surface::Cone {
                frame: frame(Point3::new(0.5, 0.0, -0.8)),
                half_angle: 0.4,
            },
        ),
        (
            "torus",
            Surface::Torus {
                frame: frame(Point3::new(0.5, 0.0, 0.0)),
                major_radius: 0.35,
                minor_radius: 0.12,
            },
        ),
    ]
}

/// A cubic B-spline over `[0, 1]` with `n` control points, `x` rising
/// across `[0, 1]`, `y` and `z` drawn, a weight drawn where `rational`.
fn carrier(rng: &mut Lcg, n: usize, rational: bool, scale: f64) -> NurbsCurve3<f64> {
    let p = 3;
    let mut knots = vec![0.0; p + 1];
    for k in 1..n - p {
        knots.push(k as f64 / (n - p) as f64);
    }
    knots.extend(vec![1.0; p + 1]);
    let kv = KnotVector::clamped(knots, p).unwrap();
    let control = (0..n)
        .map(|i| {
            Point3::new(
                i as f64 / (n - 1) as f64,
                rng.range(-scale, scale),
                rng.range(-scale, scale),
            )
        })
        .collect();
    let weights = (0..n)
        .map(|_| if rational { rng.range(0.5, 2.0) } else { 1.0 })
        .collect();
    NurbsCurve3::new(kv, control, weights).unwrap()
}

/// What a reading found: interior crossings, and a meeting at each end.
#[derive(Debug, Default)]
struct Read {
    inside: Vec<f64>,
    ends: [bool; 2],
    /// An end's stretch past the end itself (the old composite's
    /// `Within { until }` off the end).
    stretch: [bool; 2],
}

fn old(net: &NurbsCurve3<f64>, span: (f64, f64), s: &Surface<f64>, band: Band) -> Option<Read> {
    let found = super::crossings(net, span, s, band, Interval::from_certified).ok()?;
    let mut read = Read {
        inside: found.roots,
        ..Read::default()
    };
    for (k, end) in found.ends.into_iter().enumerate() {
        let at = [span.0, span.1][k];
        if let EndReading::Within { until } = end {
            if until == at {
                read.ends[k] = true;
            } else {
                read.stretch[k] = true;
            }
        }
    }
    Some(read)
}

/// The new arm at `Interval`, on the same net lifted to points.
fn new_at_interval(
    net: &NurbsCurve3<f64>,
    span: (f64, f64),
    s: &Surface<f64>,
    band: Band,
) -> Option<Read> {
    let lift = |x: f64| Interval::from_bounds(x, x);
    let net = net.map_scalar(lift);
    let curve = Curve3::Nurbs(std::sync::Arc::new(net));
    let surface = s.map_scalar(lift);
    let found = cross(&Span::new(&curve, (lift(span.0), lift(span.1))), &surface, band).ok()?;
    let mut read = Read::default();
    for m in found.meetings {
        match m.on_span {
            OnSpan::Inside { t } => read.inside.push(geom::bracket_mid((
                geom_core::Bounds::lo(t),
                geom_core::Bounds::hi(t),
            ))),
            OnSpan::End(crate::curve_pair::SpanEnd::Start) => read.ends[0] = true,
            OnSpan::End(crate::curve_pair::SpanEnd::Stop) => read.ends[1] = true,
        }
    }
    Some(read)
}

fn new(net: &NurbsCurve3<f64>, span: (f64, f64), s: &Surface<f64>, band: Band) -> Option<Read> {
    let curve = Curve3::Nurbs(std::sync::Arc::new(net.clone()));
    let found = cross(&Span::new(&curve, span), s, band).ok()?;
    let mut read = Read::default();
    for m in found.meetings {
        match m.on_span {
            OnSpan::Inside { t } => read.inside.push(t),
            OnSpan::End(crate::curve_pair::SpanEnd::Start) => read.ends[0] = true,
            OnSpan::End(crate::curve_pair::SpanEnd::Stop) => read.ends[1] = true,
        }
    }
    Some(read)
}

#[test]
#[ignore = "a one-shot measurement; run by hand"]
fn composite_arm_against_the_interval_composite() {
    for eps in [1e-6, 1e-9, 1e-12] {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for (name, surface) in surfaces() {
            let mut rng = Lcg(0x5eed_f00d);
            let (mut n, mut agree, mut both_refuse, mut old_only, mut new_only) = (0, 0, 0, 0, 0);
            let (mut differ, mut end_shift, mut stretch_cases) = (0, 0, 0);
            let mut examples = Vec::new();
            let mut old_only_by = std::collections::BTreeMap::new();
            let mut free_examples = Vec::new();
            // f64 against Interval: agree, differ, f64 only, Interval only, both refuse.
            let mut iv = [0usize; 5];
            let mut iv_by = std::collections::BTreeMap::new();
            let mut iv_examples = Vec::new();
            for case in 0..400 {
                let rational = case % 3 == 0;
                let scale = [0.5, 0.2, 0.05][case % 3];
                let net = carrier(&mut rng, 4 + case % 5, rational, scale);
                // Spans: the whole carrier, or a sub-span, some ending at
                // a crossing of the old reading.
                let span = if case % 2 == 0 {
                    (0.0, 1.0)
                } else {
                    let a = rng.range(0.0, 0.4);
                    (a, rng.range(a + 0.2, 1.0))
                };
                let mut spans = vec![(span, "free")];
                if let Some(r) = old(&net, span, &surface, band)
                    && let Some(&t) = r.inside.first()
                {
                    // Ends beside a crossing: a few bands from it, and on it.
                    for (d, tag) in [(0.0, "on"), (3.0 * eps, "+3"), (30.0 * eps, "+30"), (-3.0 * eps, "-3")] {
                        if t + d > span.0 + 1e-3 {
                            spans.push(((span.0, t + d), tag));
                        }
                    }
                }
                for (span, tag) in spans {
                    n += 1;
                    match (new(&net, span, &surface, band), new_at_interval(&net, span, &surface, band)) {
                        (Some(a), Some(b)) => {
                            let same = a.inside.len() == b.inside.len()
                                && a.inside.iter().zip(&b.inside).all(|(x, y)| (x - y).abs() <= 1e-6)
                                && a.ends == b.ends;
                            if same { iv[0] += 1 } else { iv[1] += 1 }
                        }
                        (Some(_), None) => {
                            iv[2] += 1;
                            *iv_by.entry(tag).or_insert(0) += 1;
                            if iv_examples.len() < 2 {
                                let lift = |x: f64| Interval::from_bounds(x, x);
                                let curve = Curve3::Nurbs(std::sync::Arc::new(net.map_scalar(lift)));
                                let r = cross(&Span::new(&curve, (lift(span.0), lift(span.1))), &surface.map_scalar(lift), band).err().map(|r| format!("{:?}", r.leading().why));
                                iv_examples.push(format!("{tag} {span:?}: {r:?}"));
                            }
                        }
                        (None, Some(_)) => iv[3] += 1,
                        (None, None) => iv[4] += 1,
                    }
                    match (old(&net, span, &surface, band), new(&net, span, &surface, band)) {
                        (None, None) => both_refuse += 1,
                        (Some(o), None) => {
                            old_only += 1;
                            *old_only_by.entry(tag).or_insert(0) += 1;
                            if tag == "free" && free_examples.len() < 4 {
                                let curve = Curve3::Nurbs(std::sync::Arc::new(net.clone()));
                                let why = cross(&Span::new(&curve, span), &surface, band)
                                    .err()
                                    .map(|r| format!("{:?}", r.windows));
                                free_examples.push(format!("case {case} {span:?} old {o:?}: {why:?}"));
                            }
                        }
                        (None, Some(_)) => new_only += 1,
                        (Some(o), Some(w)) => {
                            if o.stretch.iter().any(|&s| s) {
                                stretch_cases += 1;
                            }
                            let close = |a: &[f64], b: &[f64]| {
                                a.len() == b.len()
                                    && a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1e-6)
                            };
                            if close(&o.inside, &w.inside) && o.ends == w.ends {
                                agree += 1;
                            } else if o.inside.len() + usize::from(o.ends[0]) + usize::from(o.ends[1])
                                == w.inside.len() + usize::from(w.ends[0]) + usize::from(w.ends[1])
                                && o.stretch == [false; 2]
                            {
                                end_shift += 1;
                            } else if o.stretch != [false; 2]
                                && w.inside.len() >= o.inside.len()
                                && close(&o.inside, &w.inside[..o.inside.len()])
                            {
                                end_shift += 1;
                            } else {
                                differ += 1;
                                if examples.len() < 6 {
                                    examples.push(format!(
                                        "case {case} span {span:?}: old {o:?} new {w:?}"
                                    ));
                                }
                            }
                        }
                    }
                }
            }
            println!(
                "eps {eps:e} {name}: {n} spans, agree {agree}, end placed otherwise {end_shift}, \
                 differ {differ}, both refuse {both_refuse}, old only {old_only}, new only \
                 {new_only} (old stretch cases {stretch_cases})"
            );
            println!("    old only by span: {old_only_by:?}");
            println!("    new arm f64 / Interval: agree, differ, f64 only, Interval only, both refuse {iv:?}, f64 only by span {iv_by:?}");
            for e in &iv_examples {
                println!("      {}", &e[..e.len().min(500)]);
            }
            for e in free_examples.iter().chain(&examples) {
                println!("    {e}");
            }
        }
    }
}

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v * (1.0 / v.norm())
}

fn segment(degree: usize, a: Point3<f64>, b: Point3<f64>) -> NurbsCurve3<f64> {
    let mut k = vec![0.0; degree + 1];
    k.extend(vec![1.0; degree + 1]);
    let kv = KnotVector::clamped(k, degree).unwrap();
    let control = (0..=degree)
        .map(|i| a + (b - a) * (i as f64 / degree as f64))
        .collect();
    NurbsCurve3::new(kv, control, vec![1.0; degree + 1]).unwrap()
}

fn residual_root(net: &NurbsCurve3<f64>, surface: &Surface<f64>) -> f64 {
    let r = |t: f64| crate::implicit_residual(surface, net.eval(t));
    let (mut lo, mut hi) = (0.0, 1.0);
    let left = r(lo) > 0.0;
    for _ in 0..80 {
        let c = 0.5 * (lo + hi);
        if !(lo < c && c < hi) {
            break;
        }
        if (r(c) > 0.0) == left {
            lo = c;
        } else {
            hi = c;
        }
    }
    0.5 * (lo + hi)
}

/// The conditioning probe: a segment leaving a thin torus's outer
/// equator (and a far cylinder, a far plane) at a shallow sine, the
/// table's arm at `f64` and the interval composite, each root's place
/// along the edge from the residual's own root in zero bands.
#[test]
#[ignore = "a one-shot measurement; run by hand"]
fn the_arm_on_ill_conditioned_forms() {
    let zero = 1e-9;
    let band = Band::new(zero, 10.0 * zero).unwrap();
    let n = unit(Vec3::new(0.3, 0.5, 0.81));
    let e1 = unit(n.cross(Vec3::unit_z()));
    let e2 = n.cross(e1);
    let q = Point3::new(0.1, 0.2, 0.3);
    for (name, surface, reach) in [
        ("torus 10 x 1", Surface::Torus { frame: geom::Frame::new(q - e1 * 11.0, n, e1), major_radius: 10.0, minor_radius: 1.0 }, 11.0),
        ("torus 10 x 0.01", Surface::Torus { frame: geom::Frame::new(q - e1 * 10.01, n, e1), major_radius: 10.0, minor_radius: 0.01 }, 10.01),
        ("torus 100 x 0.1", Surface::Torus { frame: geom::Frame::new(q - e1 * 100.1, n, e1), major_radius: 100.0, minor_radius: 0.1 }, 100.1),
        ("torus 100 x 0.01", Surface::Torus { frame: geom::Frame::new(q - e1 * 100.01, n, e1), major_radius: 100.0, minor_radius: 0.01 }, 100.01),
        ("cylinder 1 km", Surface::Cylinder { frame: geom::Frame::new(q - e1 * 1000.0, n, e2), radius: 1000.0 }, 1000.0),
        ("sphere 1 km", Surface::Sphere { frame: geom::Frame::new(q - e1 * 1000.0, n, e2), radius: 1000.0 }, 1000.0),
        ("plane origin 10 km off", Surface::Plane { frame: geom::Frame::new(q + e2 * 1e4, e1, e2) }, 1.0),
    ] {
        for sin in [1e-2, 1e-3] {
            for degree in [3, 7] {
                let d = unit(e2 * (1.0f64 - sin * sin).sqrt() + e1 * sin);
                let len = (0.5 * reach * sin).max(1e-3);
                let net = segment(degree, q - d * (0.37 * len), q + d * (0.63 * len));
                let root = residual_root(&net, &surface);
                let curve = Curve3::Nurbs(std::sync::Arc::new(net.clone()));
                let new = cross(&Span::new(&curve, (0.0, 1.0)), &surface, band);
                let new = match new {
                    Ok(found) => format!(
                        "{:?}",
                        found
                            .meetings
                            .iter()
                            .map(|m| match m.on_span {
                                OnSpan::Inside { t } => format!("{:.2} bands", (t - root) * len / zero),
                                OnSpan::End(e) => format!("{e:?}"),
                            })
                            .collect::<Vec<_>>()
                    ),
                    Err(r) => format!("refused {:?}", r.leading().why),
                };
                let old = match super::crossings(&net, (0.0, 1.0), &surface, band, Interval::from_certified) {
                    Ok(found) => format!(
                        "{:?}",
                        found.roots.iter().map(|t| format!("{:.2} bands", (t - root) * len / zero)).collect::<Vec<_>>()
                    ),
                    Err(r) => format!("refused {r:?}"),
                };
                println!("{name}, sin {sin:e}, degree {degree}: new {new}; old {old}");
            }
        }
    }
}

#[test]
#[ignore = "a one-shot diagnosis; run by hand"]
fn why_the_interval_arm_refuses() {
    let band = Band::new(1e-9, 1e-8).unwrap();
    let (_, surface) = surfaces().remove(0);
    let mut rng = Lcg(0x5eed_f00d);
    let mut shown = 0;
    for case in 0..400 {
        let rational = case % 3 == 0;
        let scale = [0.5, 0.2, 0.05][case % 3];
        let net = carrier(&mut rng, 4 + case % 5, rational, scale);
        let span = (0.0, 1.0);
        if case % 2 == 1 {
            let a = rng.range(0.0, 0.4);
            let _ = rng.range(a + 0.2, 1.0);
            continue;
        }
        if new(&net, span, &surface, band).is_some() && new_at_interval(&net, span, &surface, band).is_none() {
            let lift = |x: f64| Interval::from_bounds(x, x);
            let curve = Curve3::Nurbs(std::sync::Arc::new(net.map_scalar(lift)));
            let r = cross(&Span::new(&curve, (lift(0.0), lift(1.0))), &surface.map_scalar(lift), band).unwrap_err();
            println!("case {case}: {:?}", r.windows);
            shown += 1;
            if shown > 6 {
                break;
            }
        }
    }
}

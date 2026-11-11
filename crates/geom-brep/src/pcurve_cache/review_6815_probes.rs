//! Review probes for PR 6815 (`review/6815-full`): not rows, findings.
//!
//! - `analytic_*`: R2's carrier on ANALYTIC charts (plane, cylinder,
//!   cone), stated by a chart image, through `EdgeCurve::certify`
//!   (claim 3: is the remainder P1?).
//! - `wall_bump_*`: a LINE carrier crossing a narrow bump in a spline
//!   wall (own_fractions adds nothing for a line), rational and not
//!   (claim 1).
//! - `in_space_*`: a bump carrier that IS in its iso column's spline
//!   space, on a wall refined to hold it (claim 1, the iso lane).
//! - `no_false_off_*`: on-wall and near-band-edge carriers with random
//!   own knots never read Off (claim 2).

#![allow(clippy::float_cmp)]

test_utils::gated_to![
    "crates/geom-brep/src/certify.rs",
    "crates/geom-brep/src/pcurve_cache/stated.rs",
    "crates/geom-brep/src/pcurve_cache/spline_incidence.rs",
];

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Decide, Interval, Point2, Point3, Real, Vec2, Vec3};

use super::Pcurve;
use super::spline_door_rows::{band, in_band, ring};
use crate::{
    CertifyError, EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec, IncidenceRead, NurbsLane,
    SurfaceKey,
};

const A: f64 = 1.0 / 128.0 - 1.0 / 256.0;
const B: f64 = 1.0 / 128.0;
const C: f64 = 1.0 / 128.0 + 1.0 / 256.0;

fn p3(p: [f64; 3]) -> Point3<f64> {
    Point3::new(p[0], p[1], p[2])
}

/// A degree-1 polyline `foot + dir·t` over `t ∈ [0, 1]` with its vertex
/// at `B` pushed `h` along `n` (vertices at `0, A, B, C, 1`).
fn bump_line(foot: [f64; 3], dir: [f64; 3], n: [f64; 3], h: f64) -> NurbsCurve3<f64> {
    let ts = [0.0, A, B, C, 1.0];
    let control = ts
        .iter()
        .map(|&t| {
            let hh = if t == B { h } else { 0.0 };
            p3([
                foot[0] + dir[0] * t + n[0] * hh,
                foot[1] + dir[1] * t + n[1] * hh,
                foot[2] + dir[2] * t + n[2] * hh,
            ])
        })
        .collect();
    NurbsCurve3::new(
        KnotVector::clamped(vec![0.0, 0.0, A, B, C, 1.0, 1.0], 1).unwrap(),
        control,
        vec![1.0; 5],
    )
    .unwrap()
}

fn general_line(p0: [f64; 2], p1: [f64; 2]) -> NurbsCurve2<f64> {
    NurbsCurve2::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        vec![Point2::new(p0[0], p0[1]), Point2::new(p1[0], p1[1])],
        vec![1.0, 1.0],
    )
    .unwrap()
}

#[derive(Clone, Copy, Debug)]
enum Img {
    Iso,
    General,
    Derived,
}

fn spec_of<T: Decide>(
    key: SurfaceKey,
    img: Img,
    p0: [f64; 2],
    p1: [f64; 2],
    carrier: Curve3<T>,
) -> EdgeCurveSpec<T> {
    let c = T::from_f64;
    let description = match img {
        Img::Iso => EdgeDescriptionSpec::chart_image(
            key,
            Pcurve::IsoLine {
                p0: Point2::new(c(p0[0]), c(p0[1])),
                pl: Vec2::new(c(p1[0] - p0[0]), c(p1[1] - p0[1])),
            },
        ),
        Img::General => EdgeDescriptionSpec::chart_image(
            key,
            Pcurve::General(Arc::new(general_line(p0, p1).map_scalar(T::from_f64))),
        ),
        Img::Derived => EdgeDescriptionSpec::chart(key),
    };
    EdgeCurveSpec {
        description,
        carrier,
        param_start: c(0.0),
        param_end: c(1.0),
    }
}

fn certify_on<T: Decide + geom_core::CertifiedBounds>(
    surface: Surface<T>,
    spec: EdgeCurveSpec<T>,
) -> Result<(), CertifyError> {
    let c = T::from_f64;
    let mut table: slotmap::SlotMap<SurfaceKey, Surface<T>> = slotmap::SlotMap::with_key();
    let _ = table.insert(surface);
    let (start, end) = (spec.carrier.eval(c(0.0)), spec.carrier.eval(c(1.0)));
    EdgeCurve::certify(
        spec,
        start,
        end,
        |k| table.get(k).cloned(),
        geom_core::test_support::fixture_lever(T::one()),
        band(),
        NurbsLane::certified(),
    )
    .map(|_| ())
}

fn key_of<T: Real>(s: &Surface<T>) -> SurfaceKey {
    let mut table: slotmap::SlotMap<SurfaceKey, Surface<T>> = slotmap::SlotMap::with_key();
    table.insert(s.clone())
}

/// An analytic chart, a ruling-like carrier on it, and the image of
/// that carrier: (name, surface, foot, dir, normal, image p0, image p1).
type Analytic = (
    &'static str,
    Surface<f64>,
    [f64; 3],
    [f64; 3],
    [f64; 3],
    [f64; 2],
    [f64; 2],
);

fn analytic_cases() -> Vec<Analytic> {
    let a: f64 = 0.3;
    vec![
        (
            "plane",
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            [0.0, 0.3, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.3],
            [1.0, 0.3],
        ),
        (
            "cylinder",
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            [1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0, 0.0],
            [0.0, 1.0],
        ),
        (
            "cone",
            Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: a,
                u_ref: Vec3::unit_x(),
            },
            [a.sin(), 0.0, a.cos()],
            [a.sin(), 0.0, a.cos()],
            [a.cos(), 0.0, -a.sin()],
            [0.0, 1.0],
            [0.0, 2.0],
        ),
    ]
}

fn map_surface<T: Real>(s: &Surface<f64>) -> Surface<T> {
    s.map_scalar(T::from_f64)
}

fn analytic_at<T: Decide + geom_core::CertifiedBounds>() -> Vec<String> {
    let mut out = Vec::new();
    for (name, s, foot, dir, n, p0, p1) in analytic_cases() {
        for img in [Img::Iso, Img::General, Img::Derived] {
            for h in [0.0, 1e-3] {
                let surface = map_surface::<T>(&s);
                let key = key_of(&surface);
                let carrier = Curve3::Nurbs(Arc::new(
                    bump_line(foot, dir, n, h).map_scalar(T::from_f64),
                ));
                let got = certify_on(surface, spec_of(key, img, p0, p1, carrier));
                let line = format!(
                    "{} {name} {img:?} h={h:e}: {}",
                    T::NAME,
                    match &got {
                        Ok(()) => "CERTIFIED".to_owned(),
                        Err(e) => format!("{e:?}").chars().take(160).collect(),
                    }
                );
                eprintln!("{line}");
                if h > 0.0 && got.is_ok() {
                    out.push(line);
                }
            }
        }
    }
    out
}

/// Claim 3: a carrier a millimetre off its ANALYTIC chart between both
/// sample sets, stated by its chart image, must not certify.
#[test]
fn analytic_bump_between_the_samples_f64() {
    let bad = analytic_at::<f64>();
    assert!(bad.is_empty(), "certified off-chart:\n{}", bad.join("\n"));
}

#[test]
fn analytic_bump_between_the_samples_interval() {
    let bad = analytic_at::<Interval>();
    assert!(bad.is_empty(), "certified off-chart:\n{}", bad.join("\n"));
}

/// Same through the pcurve door directly (`PcurveCache::certify_stated`).
fn analytic_pcurve_door<T: Decide + geom_core::CertifiedBounds>() -> Vec<String> {
    let mut out = Vec::new();
    for (name, s, foot, dir, n, p0, p1) in analytic_cases() {
        for img in [Img::Iso, Img::General] {
            let surface = map_surface::<T>(&s);
            let key = key_of(&surface);
            let carrier = Curve3::Nurbs(Arc::new(
                bump_line(foot, dir, n, 1e-3).map_scalar(T::from_f64),
            ));
            let spec = spec_of(key, img, p0, p1, carrier.clone());
            let EdgeDescriptionSpec::Chart {
                image: Some(image), ..
            } = &spec.description
            else {
                continue;
            };
            let got = super::PcurveCache::certify_stated(
                image,
                (T::zero(), T::one()),
                &carrier,
                &surface,
                band(),
                Some(crate::FittedLane::certified()),
            );
            let line = format!("{} {name} {img:?} pcurve door: {:?}", T::NAME, got.as_ref().err());
            eprintln!("{line}");
            if got.is_ok() {
                out.push(line);
            }
        }
    }
    out
}

#[test]
fn analytic_bump_through_the_pcurve_door() {
    let mut bad = analytic_pcurve_door::<f64>();
    bad.extend(analytic_pcurve_door::<Interval>());
    assert!(bad.is_empty(), "certified off-chart:\n{}", bad.join("\n"));
}

/// A bilinear wall over `x = u ∈ [0, 1]`, `z = v ∈ [0, 1]`, `y = 0`
/// but for the u row at `B`, pushed `h` in y (weight `w` on it).
fn bump_wall<T: Real>(h: f64, w: f64) -> Surface<T> {
    let us = [0.0, A, B, C, 1.0];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for &u in &us {
        for v in [0.0, 1.0] {
            control.push(Point3::new(u, if u == B { h } else { 0.0 }, v));
            weights.push(if u == B { w } else { 1.0 });
        }
    }
    let net = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, A, B, C, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

fn wall_bump_at<T: Decide + geom_core::CertifiedBounds>() -> Vec<String> {
    let mut out = Vec::new();
    let b = band();
    for w in [1.0, 3.0] {
        for h in [0.0, 1e-3, 4.0 * b.escalate(), in_band()] {
            for img in [Img::Iso, Img::General] {
                let surface = bump_wall::<T>(h, w);
                let key = key_of(&surface);
                let carrier = Curve3::line(
                    Point3::new(T::zero(), T::zero(), T::from_f64(0.5)),
                    Vec3::new(T::one(), T::zero(), T::zero()),
                );
                let got = certify_on(surface, spec_of(key, img, [0.0, 0.5], [1.0, 0.5], carrier));
                let line = format!(
                    "{} wall w={w} h={h:e} {img:?}: {}",
                    T::NAME,
                    match &got {
                        Ok(()) => "CERTIFIED".to_owned(),
                        Err(e) => format!("{e:?}").chars().take(200).collect(),
                    }
                );
                eprintln!("{line}");
                if h > b.escalate() && got.is_ok() {
                    out.push(line);
                }
            }
        }
    }
    out
}

/// Claim 1: a LINE carrier crossing a narrow wall bump between every
/// sample (own_fractions adds none for a line).
#[test]
fn wall_bump_under_a_line_f64() {
    let bad = wall_bump_at::<f64>();
    assert!(bad.is_empty(), "certified off-wall:\n{}", bad.join("\n"));
}

#[test]
fn wall_bump_under_a_line_interval() {
    let bad = wall_bump_at::<Interval>();
    assert!(bad.is_empty(), "certified off-wall:\n{}", bad.join("\n"));
}

/// A flat wall `y = 0`, `x = u`, `z = v`, refined in v at the bump's
/// knots so a bump carrier along the column `u = ½` is in its spline
/// space.
fn refined_wall<T: Real>() -> Surface<T> {
    let vs = [0.0, A, B, C, 1.0];
    let mut control = Vec::new();
    for u in [0.0, 1.0] {
        for &v in &vs {
            control.push(Point3::new(u, 0.0, v));
        }
    }
    let net = NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, A, B, C, 1.0, 1.0], 1).unwrap(),
        control,
        vec![1.0; 10],
    )
    .unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

fn in_space_at<T: Decide + geom_core::CertifiedBounds>() -> Vec<String> {
    let mut out = Vec::new();
    let b = band();
    for h in [0.0, 1e-3, 4.0 * b.escalate(), in_band()] {
        for img in [Img::Iso, Img::General] {
            let surface = refined_wall::<T>();
            let key = key_of(&surface);
            let carrier = Curve3::Nurbs(Arc::new(
                bump_line([0.5, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], h)
                    .map_scalar(T::from_f64),
            ));
            let got = certify_on(surface, spec_of(key, img, [0.5, 0.0], [0.5, 1.0], carrier));
            let line = format!(
                "{} in-space h={h:e} {img:?}: {}",
                T::NAME,
                match &got {
                    Ok(()) => "CERTIFIED".to_owned(),
                    Err(e) => format!("{e:?}").chars().take(200).collect(),
                }
            );
            eprintln!("{line}");
            if h > b.escalate() && got.is_ok() {
                out.push(line.clone());
            }
            if h == 0.0 && got.is_err() {
                out.push(format!("ON-WALL REFUSED: {line}"));
            }
        }
    }
    out
}

#[test]
fn in_space_bump_f64() {
    let bad = in_space_at::<f64>();
    assert!(bad.is_empty(), "\n{}", bad.join("\n"));
}

#[test]
fn in_space_bump_interval() {
    let bad = in_space_at::<Interval>();
    assert!(bad.is_empty(), "\n{}", bad.join("\n"));
}

/// Claim 2: a column of the ring, as a spline of random degree with
/// random interior knots (so own_fractions adds samples), at a radial
/// offset short of the band's escalate edge, never reads Off.
fn no_false_off_at<T: Decide + geom_core::CertifiedBounds>(seed: &str) -> Vec<String> {
    let b = band();
    let mut rng = test_utils::fuzz::start(seed);
    let mut bad = Vec::new();
    let mut tally = std::collections::BTreeMap::<String, usize>::new();
    for _ in 0..test_utils::fuzz::scaled(40) {
        let p = 1 + rng.below(3);
        let k = rng.below(12);
        let mut interior: Vec<f64> = (0..k).map(|_| rng.range(1e-4, 1.0 - 1e-4)).collect();
        interior.sort_by(f64::total_cmp);
        interior.dedup();
        let mut knots = vec![0.0; p + 1];
        knots.extend(&interior);
        knots.extend(vec![1.0; p + 1]);
        let n = knots.len() - p - 1;
        let d = [0.0, 0.5 * b.zero(), b.zero(), in_band(), 0.999 * b.escalate()][rng.below(5)];
        let a: f64 = 0.6;
        let r = 1.0 + d;
        // Collinear control at the Greville abscissae: the line itself.
        let control: Vec<Point3<f64>> = (0..n)
            .map(|i| {
                let xi = knots[i + 1..=i + p].iter().sum::<f64>() / p as f64;
                Point3::new(r * a.cos(), r * a.sin(), xi)
            })
            .collect();
        let net = NurbsCurve3::new(KnotVector::clamped(knots, p).unwrap(), control, vec![1.0; n])
            .unwrap();
        let surface: Surface<T> = ring::<T>();
        let key = key_of(&surface);
        let carrier = Curve3::Nurbs(Arc::new(net.map_scalar(T::from_f64)));
        let img = if rng.below(2) == 0 {
            Img::Iso
        } else {
            Img::General
        };
        let got = certify_on(surface, spec_of(key, img, [0.5, 0.0], [0.5, 1.0], carrier));
        let tag = match &got {
            Ok(()) => "ok".to_owned(),
            Err(e) => match e.incidence_read() {
                Some(IncidenceRead::Off(_)) => "OFF".to_owned(),
                Some(IncidenceRead::AtPose(_)) => "atpose".to_owned(),
                Some(IncidenceRead::Undecided(_)) => "lever".to_owned(),
                None => format!("{e:?}").chars().take(40).collect(),
            },
        };
        *tally.entry(format!("d={d:e} {img:?} {tag}")).or_default() += 1;
        if matches!(got.as_ref().err().and_then(CertifyError::incidence_read), Some(IncidenceRead::Off(_))) {
            bad.push(format!("p={p} k={k} d={d:e} {img:?}: {got:?}"));
        }
    }
    eprintln!("{} tally: {tally:#?}", T::NAME);
    bad
}

#[test]
fn no_false_off_f64() {
    let bad = no_false_off_at::<f64>("review_6815_no_false_off");
    assert!(bad.is_empty(), "false Off:\n{}", bad.join("\n"));
}

#[test]
fn no_false_off_interval() {
    let bad = no_false_off_at::<Interval>("review_6815_no_false_off_i");
    assert!(bad.is_empty(), "false Off:\n{}", bad.join("\n"));
}

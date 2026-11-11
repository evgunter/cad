//! Review probes for PR 6819 (`review/6819-full`): the measured-miss
//! climb of the mint's `General` image. Not product rows.
//!
//! - `p1_*`: carriers that leave the wall between feet (a narrow normal
//!   bump at a node and at a midpoint, a weighted ring arc off its
//!   cylinder). Asserted: no carrier past `band.escalate()` certifies,
//!   and none ends `LaneGap::Image`.
//! - `p4_*`: the trigger. Printed, not asserted: the needless climb, the
//!   standoff read as a norm (a tangential chord miss beside a normal
//!   standoff), and a tangential wiggle between a node and a midpoint.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Decide, Interval, Point3, Vec3};

use super::spline_door_rows::band;
use super::{IncidenceRead, LaneGap, Pcurve, PcurveCache, PcurveCertifyError, minted_refusal};
use crate::FittedLane;
use crate::recourse::Reading;

fn flat_wall() -> NurbsSurface<f64> {
    let k = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(
        k(),
        k(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
        ],
        vec![1.0; 4],
    )
    .unwrap()
}

fn uneven_wall() -> NurbsSurface<f64> {
    let mut control = Vec::new();
    for x in [0.0, 1.0] {
        for z in [0.0, 0.1, 1.0] {
            control.push(Point3::new(x, 0.0, z));
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        control,
        vec![1.0; 6],
    )
    .unwrap()
}

fn ring_wall() -> NurbsSurface<f64> {
    let h = 0.6_f64;
    let mut ku = vec![0.0; 3];
    ku.extend([0.5; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = f64::from(j) * h / 2.0;
        let (reach, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for z in [-0.5, 0.5] {
            control.push(Point3::new(reach * theta.cos(), reach * theta.sin(), z));
            weights.push(w);
        }
    }
    NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap()
}

/// The ring's arc with its middle weight moved so its midpoint sits
/// `delta` outside the cylinder (radially); its ends stay on.
fn ring_arc_off(delta: f64) -> Curve3<f64> {
    let h = 0.6_f64;
    let c = h.cos();
    let w = (1.0 + delta - c) / (1.0 / c - 1.0 - delta);
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, h.tan(), 0.0),
                Point3::new((2.0 * h).cos(), (2.0 * h).sin(), 0.0),
            ],
            vec![1.0, w, 1.0],
        )
        .unwrap(),
    ))
}

/// A uniform cubic spline on 256 spans tracing `base + dir·s`, `s ∈ [0,1]`
/// (linear precision), with one control point pushed by `1.5·bump` so the
/// curve reaches `bump` at its Greville abscissa `(j−1)/256`, support
/// `[(j−3)/256, (j+1)/256]`, narrower than the 33-foot schedule's step.
fn bumped(base: Point3<f64>, dir: Vec3<f64>, j: usize, bump: Vec3<f64>) -> Curve3<f64> {
    let n = 256usize;
    let mut knots = vec![0.0; 4];
    knots.extend((1..n).map(|i| i as f64 / n as f64));
    knots.extend([1.0; 4]);
    let kv = KnotVector::clamped(knots.clone(), 3).unwrap();
    let control: Vec<Point3<f64>> = (0..n + 3)
        .map(|k| {
            let g = (knots[k + 1] + knots[k + 2] + knots[k + 3]) / 3.0;
            let p = base + dir * g;
            if k == j { p + bump * 1.5 } else { p }
        })
        .collect();
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(kv, control, vec![1.0; n + 3]).unwrap(),
    ))
}

struct Minted<T: geom_core::Real> {
    image: NurbsCurve2<T>,
    certified: Result<(), PcurveCertifyError>,
}

fn mint<T: Decide>(
    lane: FittedLane<T>,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    wall: &NurbsSurface<f64>,
    image_band: Band,
) -> Result<Minted<T>, PcurveCertifyError> {
    let lift = |x: f64| T::from_f64(x);
    let carrier = carrier.map_scalar(lift);
    let wall = wall.map_scalar(lift);
    let (t0, t1) = (lift(span.0), lift(span.1));
    let image = lane.general_image(&carrier, (t0, t1), &wall, image_band)?;
    let surface = Surface::Nurbs(Arc::new(wall));
    let certified = PcurveCache::certify_general(
        Arc::new(image.clone()),
        t0,
        t1,
        &carrier,
        &surface,
        band(),
        Some(lane),
    )
    .map(|_| ())
    .map_err(|e| {
        minted_refusal(
            e,
            &Pcurve::General(Arc::new(image.clone())),
            &carrier,
            &surface,
        )
    });
    Ok(Minted { image, certified })
}

fn describe<T: geom_core::Real>(m: &Minted<T>) -> String {
    let ending = match &m.certified {
        Ok(()) => "CERTIFIED".to_string(),
        Err(e) => {
            let read =
                IncidenceRead::of(e).map(|r| format!(" read={r:?} definite={}", r.is_definite()));
            let end = e.ending(Reading::Build);
            let tag = if end.contains(geom_core::NOT_YET_ENDING) {
                "NOT-YET"
            } else if end.contains(geom_core::KERNEL_DEFECT_ENDING) {
                "DEFECT"
            } else {
                "OTHER"
            };
            format!("{tag} {e:?}{}", read.unwrap_or_default())
        }
    };
    format!(
        "deg {} ctrl {} :: {}",
        m.image.degree(),
        m.image.control().len(),
        ending
    )
}

/// No carrier past `band.escalate()` certifies, and none ends the
/// lane's gap ("not yet").
fn sound<T: geom_core::Real>(name: &str, delta: f64, m: &Minted<T>) {
    let b = band();
    eprintln!(
        "[p1 {} eps={:e}] {name} δ={:e}: {}",
        T::NAME,
        b.zero(),
        delta,
        describe(m)
    );
    if delta > b.escalate() {
        assert!(
            m.certified.is_err(),
            "{name} δ={delta:e} certified at {}",
            T::NAME
        );
        let e = m.certified.as_ref().unwrap_err();
        assert!(
            !matches!(
                e,
                PcurveCertifyError::NoGeneralLane {
                    gap: LaneGap::Image,
                    ..
                }
            ),
            "{name} δ={delta:e} at {}: a carrier off its wall ends not-yet: {e:?}",
            T::NAME
        );
    }
}

fn p1<T: Decide>(lane: FittedLane<T>) {
    let eps = band().zero();
    let flat_base = Point3::new(0.1, 0.0, 0.2);
    let flat_dir = Vec3::new(0.6, 0.0, 0.8);
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let uneven_base = Point3::new(0.05, 0.0, 0.05);
    let uneven_dir = Vec3::new(r * 1.2, 0.0, r * 1.2);
    for k in [0.5, 2.0, 10.0, 1e3, 1e6] {
        let d = k * eps;
        let y = Vec3::new(0.0, d, 0.0);
        for (where_, j) in [("node 1/2", 129usize), ("midpoint 33/64", 133usize)] {
            let flat = bumped(flat_base, flat_dir, j, y);
            match mint::<T>(lane, &flat, (0.0, 1.0), &flat_wall(), band()) {
                Ok(m) => sound(&format!("flat, normal bump at {where_}"), d, &m),
                Err(e) => eprintln!("[p1] flat bump {where_} δ={d:e}: producer refused {e:?}"),
            }
            let uneven = bumped(uneven_base, uneven_dir, j, y);
            match mint::<T>(lane, &uneven, (0.0, 1.0), &uneven_wall(), band()) {
                Ok(m) => sound(&format!("uneven, normal bump at {where_}"), d, &m),
                Err(e) => eprintln!("[p1] uneven bump {where_} δ={d:e}: producer refused {e:?}"),
            }
        }
        match mint::<T>(lane, &ring_arc_off(d), (0.0, 1.0), &ring_wall(), band()) {
            Ok(m) => sound("ring arc bulged off", d, &m),
            Err(e) => eprintln!("[p1] ring δ={d:e}: producer refused {e:?}"),
        }
    }
}

#[test]
fn p1_off_wall_carriers_never_certify_or_end_not_yet_at_f64() {
    p1(FittedLane::<f64>::certified());
}

#[test]
fn p1_off_wall_carriers_never_certify_or_end_not_yet_at_interval() {
    p1(FittedLane::<Interval>::certified());
}

/// The degree-1 image's sampled miss (64 stations per foot step) in
/// metres, at f64.
fn sampled_miss(
    image: &NurbsCurve2<f64>,
    carrier: &Curve3<f64>,
    span: (f64, f64),
    wall: &NurbsSurface<f64>,
) -> f64 {
    let n = 32 * 64;
    (0..=n)
        .map(|i| {
            let t = span.0 + (span.1 - span.0) * i as f64 / n as f64;
            let p = image.eval(t);
            (wall.eval(p.x, p.y) - carrier.eval(t)).norm()
        })
        .fold(0.0, f64::max)
}

fn p4<T: Decide>(lane: FittedLane<T>) {
    let eps = band().zero();
    let never = Band::new(1e3, 2e3).unwrap();
    let always = Band::new(1e-300, 2e-300).unwrap();
    let r = core::f64::consts::FRAC_1_SQRT_2;
    let line_at = |y: f64| Curve3::line(Point3::new(0.05, y, 0.05), Vec3::new(r, 0.0, r));
    let wall = uneven_wall();
    for l in [1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2] {
        for standoff in [0.0, 0.5, 0.9] {
            let carrier = line_at(standoff * eps);
            let span = (0.0, l);
            let tag = format!(
                "[p4 {} eps={eps:e}] uneven line L={l:e} standoff={standoff}ε",
                T::NAME
            );
            let Ok(d1) = mint::<f64>(FittedLane::<f64>::certified(), &carrier, span, &wall, never)
            else {
                continue;
            };
            let c = sampled_miss(&d1.image, &carrier, span, &wall);
            let real = mint::<T>(lane, &carrier, span, &wall, band());
            let forced1 = mint::<T>(lane, &carrier, span, &wall, never);
            let climbed = mint::<T>(lane, &carrier, span, &wall, always);
            let show = |m: &Result<Minted<T>, PcurveCertifyError>| match m {
                Ok(m) => describe(m),
                Err(e) => format!("producer refused {e:?}"),
            };
            eprintln!(
                "{tag}: sampled degree-1 miss {:.3}ε\n    producer: {}\n    forced deg-1: {}\n    forced climb: {}",
                c / eps,
                show(&real),
                show(&forced1),
                show(&climbed)
            );
        }
    }
    // A carrier on the flat wall that wiggles along it (in the wall's
    // plane, across its direction) between node 1/2 and midpoint 33/64.
    for k in [0.25, 1.0, 10.0] {
        let carrier = bumped(
            Point3::new(0.1, 0.0, 0.2),
            Vec3::new(0.6, 0.0, 0.8),
            131,
            Vec3::new(0.8 * k * eps, 0.0, -0.6 * k * eps),
        );
        let span = (0.0, 1.0);
        let real = mint::<T>(lane, &carrier, span, &flat_wall(), band());
        let climbed = mint::<T>(lane, &carrier, span, &flat_wall(), always);
        let show = |m: &Result<Minted<T>, PcurveCertifyError>| match m {
            Ok(m) => describe(m),
            Err(e) => format!("producer refused {e:?}"),
        };
        eprintln!(
            "[p4 {} eps={eps:e}] tangential wiggle {k}ε at 0.5+1/128:\n    producer: {}\n    forced climb: {}",
            T::NAME,
            show(&real),
            show(&climbed)
        );
    }
}

#[test]
fn p4_threshold_at_f64() {
    p4(FittedLane::<f64>::certified());
}

#[test]
fn p4_threshold_at_interval() {
    p4(FittedLane::<Interval>::certified());
}

//! **The spline chart's floor is a lower bound, and the door ends by
//! it.** A counterexample search compares each sample's floor with a
//! chart point's distance, enclosed at `Interval` (an upper bound on the
//! sample's distance from the chart), over rational and polynomial
//! patches of degree one to four with up to four interior knots of any
//! multiplicity, and near-degenerate nets: a row collapsed to a pole, a
//! net flattened to a micron, coincident neighbours, weights four
//! decades apart. Built with `PCERT_RADIAL_PAIR_MUTANT=spline-floor-net`
//! (the floor read as the nearest control point's distance, no hull) it
//! is red. The fixed rows read the door on a rational ring wall: a
//! circle on it, off it, and at a band-scale distance from it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to!["crates/geom-brep/src/pcurve_cache/spline_incidence.rs"];

use geom::{Curve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Bounds, Interval, Point3, Tol, Vec3};
use std::sync::Arc;

use super::{Net, Read};
use crate::{PcurveCertifyError, PcurveCheck};

fn knots(rng: &mut test_utils::fuzz::Rng, p: usize) -> KnotVector {
    let mut interior: Vec<f64> = (0..rng.below(5)).map(|_| rng.range(0.05, 0.95)).collect();
    interior.sort_by(f64::total_cmp);
    let mut v = vec![0.0; p + 1];
    for x in interior {
        v.extend(core::iter::repeat_n(x, 1 + rng.below(p)));
    }
    v.extend(core::iter::repeat_n(1.0, p + 1));
    KnotVector::clamped(v, p).unwrap()
}

/// A random patch (module docs), its mode named for the message.
fn patch(rng: &mut test_utils::fuzz::Rng) -> (NurbsSurface<f64>, &'static str) {
    let (pu, pv) = (1 + rng.below(4), 1 + rng.below(4));
    let (ku, kv) = (knots(rng, pu), knots(rng, pv));
    let (nu, nv) = (ku.control_count(), kv.control_count());
    let scale = 10f64.powf(rng.range(-2.0, 3.0));
    let offset = Vec3::new(
        rng.range(-5.0, 5.0),
        rng.range(-5.0, 5.0),
        rng.range(-5.0, 5.0),
    ) * scale;
    let mode = ["smooth", "folded", "pole", "flat", "coincident"][rng.below(5)];
    let mut control = Vec::with_capacity(nu * nv);
    for i in 0..nu {
        for j in 0..nv {
            let (x, y) = (i as f64 / nu as f64, j as f64 / nv as f64);
            let z = match mode {
                "folded" => rng.range(-1.0, 1.0),
                "flat" => rng.range(-1.0, 1.0) * 1e-6,
                _ => 0.3 * (3.0 * x).sin() * (2.0 * y).cos() + rng.range(-0.05, 0.05),
            };
            let (x, y) = match mode {
                "folded" => (x + rng.range(-0.5, 0.5), y + rng.range(-0.5, 0.5)),
                "pole" if i == 0 => (0.0, 0.5),
                _ => (x, y),
            };
            control.push(Point3::new(x * scale, y * scale, z * scale) + offset);
        }
    }
    if mode == "coincident" {
        for i in (1..nu).step_by(2) {
            for j in 0..nv {
                control[i * nv + j] = control[(i - 1) * nv + j];
            }
        }
    }
    let weights = match rng.below(3) {
        0 => vec![1.0; nu * nv],
        1 => (0..nu * nv)
            .map(|_| 10f64.powf(rng.range(-1.0, 1.0)))
            .collect(),
        _ => (0..nu * nv)
            .map(|_| 10f64.powf(rng.range(-2.0, 2.0)))
            .collect(),
    };
    (NurbsSurface::new(ku, kv, control, weights).unwrap(), mode)
}

fn search_band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// **No sample is read off by more than a chart point's distance from
/// it** (module docs), at `f64` and at `Interval`. A sample is a chart
/// point at a random parameter, moved by `0` or by a distance from
/// `1e-10` to `10` times the patch's scale in a random direction.
#[test]
fn a_samples_floor_is_below_a_chart_points_distance() {
    let mut rng = test_utils::fuzz::start("spline-chart-floor");
    let band = search_band();
    let (mut off, mut on, mut unsettled) = (0usize, 0usize, 0usize);
    for case in 0..test_utils::fuzz::scaled(400) {
        let (chart, mode) = patch(&mut rng);
        let lifted = chart.map_scalar(<Interval as geom_core::Real>::from_f64);
        let ((u0, u1), (v0, v1)) = (
            chart.knot_values_u().domain(),
            chart.knot_values_v().domain(),
        );
        let (u, v) = (rng.range(u0, u1), rng.range(v0, v1));
        let foot = chart.eval(u, v);
        let dir = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let reach = (foot - Point3::origin()).norm().max(1.0);
        let d = if case % 4 == 0 {
            0.0
        } else {
            10f64.powf(rng.range(-10.0, 1.0)) * reach
        };
        let q = foot + dir.normalize() * d;
        let qi = q.map(<Interval as geom_core::Real>::from_f64);
        let upper = (qi
            - lifted.eval(
                <Interval as geom_core::Real>::from_f64(u),
                <Interval as geom_core::Real>::from_f64(v),
            ))
        .norm()
        .hi();
        let what = || {
            format!(
                "case {case} ({mode}, degrees {:?}, {} weights): q {q:?} at most {upper:e} from \
                 the chart; {}",
                (
                    chart.knot_values_u().degree(),
                    chart.knot_values_v().degree()
                ),
                if chart.constant_weight().is_some() {
                    "constant"
                } else {
                    "varying"
                },
                test_utils::fuzz::replay()
            )
        };
        match Net::of(&chart, band).expect("the patch cuts").read(q, band) {
            Read::Off(floor) => {
                off += 1;
                assert!(
                    floor <= upper,
                    "f64 floor {floor:e} above the distance: {}",
                    what()
                );
            }
            Read::On => on += 1,
            Read::Unsettled(_) => unsettled += 1,
        }
        if let Read::Off(floor) = Net::of(&lifted, band)
            .expect("the patch cuts")
            .read(qi, band)
        {
            assert!(
                floor.lo() <= upper,
                "Interval floor {floor:?} above the distance: {}",
                what()
            );
        }
    }
    eprintln!("[spline-chart-floor] off {off}, on {on}, unsettled {unsettled}");
}

/// The unit cylinder `z ∈ [0, 1]` over `[0, 1.2]` rad from `x̂`, in two
/// rational quadratic sub-arcs.
fn ring<T: geom_core::Real>() -> Surface<T> {
    let h = 1.2 / 2.0;
    let mut ku = vec![0.0; 3];
    ku.extend([0.5; 2]);
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=4 {
        let theta = j as f64 * h / 2.0;
        let (r, w) = if j % 2 == 1 {
            (1.0 / (h / 2.0).cos(), (h / 2.0).cos())
        } else {
            (1.0, 1.0)
        };
        for z in [0.0, 1.0] {
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    let net = NurbsSurface::new(
        KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
        control,
        weights,
    )
    .unwrap();
    Surface::Nurbs(Arc::new(net.map_scalar(T::from_f64)))
}

/// A circle about the ring's axis at `z = 0.5`, of radius `r`, over the
/// wall's turn less a tenth at each end.
fn read<T: geom_core::Decide>(r: f64) -> Result<(), PcurveCertifyError> {
    let band = Band::linear(Tol::witness()).unwrap();
    let c = |x: f64| T::from_f64(x);
    let circle = Curve3::circle(
        Point3::new(c(0.0), c(0.0), c(0.5)),
        Vec3::new(c(0.0), c(0.0), c(1.0)),
        c(r),
        Vec3::new(c(1.0), c(0.0), c(0.0)),
    );
    crate::read_edge_incidence(&circle, c(0.1), c(1.1), &ring::<T>(), band)
}

fn the_door_reads_a_circle_against_a_ring_wall<T: geom_core::Decide>() {
    let band = Band::linear(Tol::witness()).unwrap();
    assert_eq!(
        read::<T>(1.0),
        Ok(()),
        "a circle on the wall, at {}",
        T::NAME
    );
    let off = read::<T>(1.0 + 1e-3);
    assert!(
        matches!(
            off,
            Err(PcurveCertifyError::CarrierOffChart {
                chart: geom::SurfaceKind::Nurbs,
                carrier: geom::CurveKind::Circle,
                ..
            })
        ),
        "a circle a millimetre off the wall, at {}: {off:?}",
        T::NAME
    );
    let near = read::<T>(1.0 + 0.5 * (band.zero() + band.escalate()));
    assert!(
        matches!(
            near,
            Err(PcurveCertifyError::Escalated {
                check: PcurveCheck::ChartIncidence,
                ..
            })
        ),
        "a circle at a band-scale distance from the wall, at {}: {near:?}",
        T::NAME
    );
}

#[test]
fn the_door_reads_a_circle_against_a_ring_wall_at_f64() {
    the_door_reads_a_circle_against_a_ring_wall::<f64>();
}

#[test]
fn the_door_reads_a_circle_against_a_ring_wall_at_interval() {
    the_door_reads_a_circle_against_a_ring_wall::<Interval>();
}

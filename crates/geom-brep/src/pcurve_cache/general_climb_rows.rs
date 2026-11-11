//! **The mint's `General` image climbs on its measured miss** (D3), not on
//! its carrier's class: `general_image_lane` takes the degree-1 image the
//! edge certificate reads, reads its miss midway between feet, and climbs
//! `edge_nurbs::curved_chart_image`'s rungs where that miss exceeds the
//! rung's share of the band. A refusal of the image the mint produced is
//! the lane's gap ([`super::minted_refusal`]), never a tolerance offer.
//!
//! - A curved spline carrier, the rational one-span arc of a ring's
//!   circle on that ring, and a line on a flat wall whose `v` is not
//!   affine in height (control rows at `z = 0, 0.1, 1`) bend on their
//!   walls and climb: each certifies at `f64` and at `Interval`.
//! - A line and a degree-1 spline on a flat bilinear wall, and a short
//!   arc of a circle on a cylinder wall, follow their degree-1 images
//!   and keep them: no climb.
//!
//! The `general-image-class-gate` mutant (the climb and the relabel keyed
//! on a curved carrier with no net) reds the two bending rows: each takes
//! the degree-1 image, and its refusal keeps its own term.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{CertifiedBounds, Decide, Interval, Point3, Vec3};

use super::spline_door_rows::band;
use super::{LaneGap, Pcurve, PcurveCache, PcurveCertifyError, minted_refusal};
use crate::FittedLane;
use crate::edge_nurbs::{PXN_FIT_SAMPLES, PXN_IMAGE_DEGREE};
use crate::recourse::Reading;

/// The unit cylinder over `z ∈ [−½, ½]` through `1.2` rad from `x̂`, in
/// two rational quadratic sub-arcs: its `u = ½` knot line is the joint.
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

/// The ring's circle at `z = 0` through its whole `1.2` rad, as the
/// rational one-span quadratic arc: a spline carrier that bends.
fn ring_arc() -> Curve3<f64> {
    let h = 0.6_f64;
    Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
            vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, h.tan(), 0.0),
                Point3::new((2.0 * h).cos(), (2.0 * h).sin(), 0.0),
            ],
            vec![1.0, h.cos(), 1.0],
        )
        .unwrap(),
    ))
}

/// The plane `y = 0` over `x ∈ [0, 1]`, its `v` quadratic in height:
/// control rows at `z = 0, 0.1, 1`, so `z(v) = 0.2v − 0.1v² + … ` is
/// not affine and a straight line's feet bend in the chart.
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

/// A diagonal line across [`uneven_wall`], over `t ∈ [0, 1.2]`.
fn uneven_line() -> Curve3<f64> {
    let r = core::f64::consts::FRAC_1_SQRT_2;
    Curve3::line(Point3::new(0.05, 0.0, 0.05), Vec3::new(r, 0.0, r))
}

/// The plane `y = 0` over `[0, 1]²` as a bilinear wall: its chart is
/// affine, so every line's feet are straight in it.
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

/// What the mint does with `carrier` on `wall` over `span` at `T`: the
/// image `general_image_lane` derives, and its certificate's refusal as
/// the mint ends it ([`minted_refusal`]).
struct Minted<T: geom_core::Real> {
    image: NurbsCurve2<T>,
    certified: Result<(), PcurveCertifyError>,
}

fn mint<T: Decide + CertifiedBounds>(
    carrier: &Curve3<f64>,
    span: (f64, f64),
    wall: &NurbsSurface<f64>,
) -> Minted<T> {
    let lift = |x: f64| T::from_f64(x);
    let lane = FittedLane::<T>::certified();
    let carrier = carrier.map_scalar(lift);
    let wall = wall.map_scalar(lift);
    let (t0, t1) = (lift(span.0), lift(span.1));
    let image = lane
        .general_image(&carrier, (t0, t1), &wall, band())
        .unwrap_or_else(|e| panic!("the producer images the carrier at {}: {e:?}", T::NAME));
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
    Minted { image, certified }
}

/// Whether `image` is the degree-1 image of [`PXN_FIT_SAMPLES`] feet.
fn degree_one<T: geom_core::Real>(image: &NurbsCurve2<T>) -> bool {
    image.degree() == PXN_IMAGE_DEGREE && image.control().len() == PXN_FIT_SAMPLES as usize
}

/// A carrier whose feet bend on its wall climbs past the degree-1 image.
/// Where `certifies`, the climbed image certifies; anywhere else its
/// refusal ends the lane's gap, "not yet" ([`LaneGap::Image`], or the
/// same check escalated in band), never a tolerance offer.
fn bends_and_climbs<T: Decide + CertifiedBounds>(name: &str, minted: &Minted<T>, certifies: bool) {
    if let Err(got) = &minted.certified {
        let ending = got.ending(Reading::Build);
        assert!(
            matches!(
                got,
                PcurveCertifyError::NoGeneralLane {
                    gap: LaneGap::Image,
                    ..
                } | PcurveCertifyError::Escalated {
                    check: super::PcurveCheck::GeneralLane,
                    ..
                }
            ) && ending.contains(geom_core::NOT_YET_ENDING)
                && !ending.contains("loosen"),
            "{name} at {}: a refused climb is the lane's gap: {got:?} :: {ending}",
            T::NAME
        );
    }
    assert!(
        !certifies || minted.certified.is_ok(),
        "{name} at {}: the climbed image certifies: {:?}",
        T::NAME,
        minted.certified
    );
    assert!(
        !degree_one(&minted.image),
        "{name} at {}: the feet bend, so the image climbs past degree 1 \
         (degree {}, {} control points)",
        T::NAME,
        minted.image.degree(),
        minted.image.control().len()
    );
}

fn the_bending_carriers_climb<T: Decide + CertifiedBounds>() {
    bends_and_climbs(
        "the ring's spline arc",
        &mint::<T>(&ring_arc(), (0.0, 1.0), &ring_wall()),
        true,
    );
    // Below `ε = 1e-10` the hull over-reads the climbed image's miss on
    // this stretched chart (`a-climbed-general-image-on-a-stretched-chart-
    // reads-its-hull-wide-at-fine-eps`), which ends the lane's gap.
    bends_and_climbs(
        "a line on the uneven wall",
        &mint::<T>(&uneven_line(), (0.0, 1.2), &uneven_wall()),
        band().zero() >= 1e-10,
    );
}

#[test]
fn the_bending_carriers_climb_at_f64() {
    the_bending_carriers_climb::<f64>();
}

#[test]
fn the_bending_carriers_climb_at_interval() {
    the_bending_carriers_climb::<Interval>();
}

/// A carrier whose degree-1 image follows it keeps that image, the one
/// its edge certificate reads, and certifies.
fn straight_feet_keep_degree_one<T: Decide + CertifiedBounds>() {
    let line = Curve3::line(Point3::new(0.1, 0.0, 0.2), Vec3::new(0.6, 0.0, 0.8));
    let spline = Curve3::Nurbs(Arc::new(
        NurbsCurve3::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point3::new(0.1, 0.0, 0.2), Point3::new(0.7, 0.0, 1.0)],
            vec![1.0; 2],
        )
        .unwrap(),
    ));
    for (name, carrier) in [("the line", line), ("the degree-1 spline", spline)] {
        let minted = mint::<T>(&carrier, (0.0, 1.0), &flat_wall());
        assert!(
            degree_one(&minted.image) && minted.certified.is_ok(),
            "{name} at {}: the degree-1 image, certified (degree {}, {} control points): {:?}",
            T::NAME,
            minted.image.degree(),
            minted.image.control().len(),
            minted.certified
        );
    }
}

#[test]
fn straight_feet_keep_degree_one_at_f64() {
    straight_feet_keep_degree_one::<f64>();
}

#[test]
fn straight_feet_keep_degree_one_at_interval() {
    straight_feet_keep_degree_one::<Interval>();
}

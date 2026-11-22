#![allow(clippy::unwrap_used)]

//! **A joint at a spline net's collapsed side** (DESIGN `[extensibility]`):
//! where the side is one stored point the channel along it is free and
//! the element is the identity, and where its two ends are one ulp
//! apart the side is regular and the joint's chart gap, a whole span
//! along it, refuses.

use super::{ChartPeriods, DescribedChart, PinMiss, decide_plain_joint};
use crate::joint::JointElement;
use geom::{NurbsSurface, Surface};
use geom_brep::Pcurve;
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Band, Point2, Point3, Vec2};
use std::sync::Arc;

fn band() -> Band {
    Band::new(1e-9, 1e-8).unwrap()
}

/// A triangle of a wall: its `u = 0` column from `pole` to `top`, its
/// `u = 1` column from (3, 0, 0) to (3, 2, 0).
fn triangle(pole: Point3<f64>, top: Point3<f64>) -> Surface<f64> {
    let kv = || Knots::from(KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap());
    let control = vec![
        pole,
        top,
        Point3::new(3.0, 0.0, 0.0),
        Point3::new(3.0, 2.0, 0.0),
    ];
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(kv(), kv(), control, vec![1.0; 4]).unwrap(),
    ))
}

/// The joint into the bottom row `(0, 0) → (1, 0)` from the top row's
/// end at `(0, 1)`, at the vertex `pole`.
fn joint(surface: &Surface<f64>, pole: Point3<f64>) -> Result<JointElement, PinMiss> {
    let image = Pcurve::IsoLine {
        p0: Point2::new(0.0, 0.0),
        pl: Vec2::new(1.0, 0.0),
        ends: None,
    };
    decide_plain_joint(
        DescribedChart::of(surface).unwrap(),
        &image,
        0.0,
        Point2::new(0.0, 1.0),
        pole,
        ChartPeriods::of(surface, band()),
        band(),
    )
}

/// The pole at (1, 1, 0); one ulp off, the column's top a ulp along `x`.
#[test]
fn a_joint_on_a_collapsed_side_is_the_identity_and_one_ulp_off_it_is_a_gap() {
    let pole = Point3::new(1.0, 1.0, 0.0);
    let one = triangle(pole, pole);
    assert!(
        matches!(joint(&one, pole), Ok(JointElement::IDENTITY)),
        "one stored point: the identity, read {:?}",
        joint(&one, pole).ok()
    );
    let ulp = Point3::new(f64::from_bits(1.0_f64.to_bits() + 1), 1.0, 0.0);
    let apart = triangle(pole, ulp);
    assert!(
        geom_brep::chart_singularity::singular_points(&apart).is_empty(),
        "one ulp apart: no singular point"
    );
    assert!(
        matches!(joint(&apart, pole), Err(PinMiss::Discontinuity)),
        "one ulp apart: the chart gap refuses, read {:?}",
        joint(&apart, pole).ok()
    );
}

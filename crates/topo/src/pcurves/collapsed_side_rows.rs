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

/// A NURBS cone about `z`: its `u` row the rational circle, closed, and
/// its side `v = 0` the apex, one stored point.
fn spline_cone() -> Surface<f64> {
    let ku = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let s = std::f64::consts::FRAC_1_SQRT_2;
    let ring = [
        (1.0, 0.0, 1.0),
        (1.0, 1.0, s),
        (0.0, 1.0, 1.0),
        (-1.0, 1.0, s),
        (-1.0, 0.0, 1.0),
        (-1.0, -1.0, s),
        (0.0, -1.0, 1.0),
        (1.0, -1.0, s),
        (1.0, 0.0, 1.0),
    ];
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for &(x, y, w) in &ring {
        control.push(Point3::new(0.0, 0.0, 0.0));
        control.push(Point3::new(x, y, -1.0));
        weights.extend([w, w]);
    }
    Surface::Nurbs(Arc::new(
        NurbsSurface::new(Knots::from(ku), Knots::from(kv), control, weights).unwrap(),
    ))
}

/// **A collapsed side along a channel the net closes refuses typed**: at
/// a NURBS cone's apex the turn about the point and the face's side of
/// it have no spline reading yet, so the joint refuses
/// [`super::SingularMiss::ClosedSide`] rather than writing an element.
#[test]
fn a_collapsed_side_along_a_closed_channel_refuses_typed() {
    let cone = spline_cone();
    let apex = Point3::new(0.0, 0.0, 0.0);
    assert!(
        ChartPeriods::of(&cone, band()).u().is_some(),
        "the cone's u closes"
    );
    let image = Pcurve::IsoLine {
        p0: Point2::new(0.25, 0.0),
        pl: Vec2::new(0.0, 1.0),
        ends: None,
    };
    let out = decide_plain_joint(
        DescribedChart::of(&cone).unwrap(),
        &image,
        0.0,
        Point2::new(0.75, 0.0),
        apex,
        ChartPeriods::of(&cone, band()),
        band(),
    );
    assert!(
        matches!(out, Err(PinMiss::Singular(super::SingularMiss::ClosedSide))),
        "the apex refuses as unbuilt, read {:?}",
        out.ok()
    );
}

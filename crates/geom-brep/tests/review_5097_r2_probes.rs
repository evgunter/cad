//! Review probes for PR 5097 (r2): the closed-form carrier lanes at
//! `Interval`, and a short circle arc across the 2π seam.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use crate::shared::fixture::quarter_cylinder_wall;
use crate::shared::tol::{band, eps};
use geom::{Curve3, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::{PlaneNurbsRefusal, nurbs_pair_limbs, plane_nurbs_limbs};
use geom_core::interval::Interval;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Real, Vec3};

fn bilinear(corners: [Point3<f64>; 4]) -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    NurbsSurface::new(k.clone(), k, corners.to_vec(), vec![1.0; 4]).unwrap()
}
fn flat_wall() -> NurbsSurface<f64> {
    let p = |x, y| Point3::new(x, y, 0.0);
    bilinear([p(0.0, 0.0), p(0.0, 1.0), p(1.0, 0.0), p(1.0, 1.0)])
}
fn diagonal() -> Curve3<f64> {
    Curve3::Line { origin: Point3::new(0.1, 0.1, 0.0), dir: Vec3::new(0.6, 0.8, 0.0) }
}
fn diagonal_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.1, 0.1, 0.0),
        normal: Vec3::new(0.8, -0.6, 0.0),
        u_ref: Vec3::new(0.6, 0.8, 0.0),
    }
}
fn diagonal_wall() -> NurbsSurface<f64> {
    let at = |t: f64, z| Point3::new(0.1 + 0.6 * t, 0.1 + 0.8 * t, z);
    bilinear([at(-0.1, -0.5), at(-0.1, 0.5), at(1.1, -0.5), at(1.1, 0.5)])
}
fn circle(z: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.0, 0.0, z),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
fn horizontal(h: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, h),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}
fn short(from: f64, scale: f64) -> (f64, f64) {
    (from, from + scale * (eps() / 1e-9).sqrt().min(20.0))
}
fn itv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// P1: the Interval lane on the closed forms the f64 rows certify.
#[test]
fn p1_closed_forms_at_interval() {
    let line = diagonal().map_scalar(itv);
    let got = plane_nurbs_limbs(
        &line,
        (itv(0.0), itv(1.0)),
        &diagonal_plane().map_scalar(itv),
        &flat_wall().map_scalar(itv),
        Interval::one(),
        band(),
    );
    println!("P1 line plane×wall Interval: {:?}", got.as_ref().map(|l| (l.hull_sup, l.on_locus_max)));
    assert!(got.is_ok(), "{got:?}");
    let got = nurbs_pair_limbs(
        &line,
        (itv(0.0), itv(1.0)),
        &flat_wall().map_scalar(itv),
        &diagonal_wall().map_scalar(itv),
        Interval::one(),
        band(),
    );
    println!("P1 line wall×wall Interval: {:?}", got.as_ref().map(|l| l.hull_sup));
    assert!(got.is_ok(), "{got:?}");
    let (a, b) = short(0.7, 0.002);
    let got = plane_nurbs_limbs(
        &circle(0.5).map_scalar(itv),
        (itv(a), itv(b)),
        &horizontal(0.5).map_scalar(itv),
        &quarter_cylinder_wall().map_scalar(itv),
        Interval::one(),
        band(),
    );
    println!("P1 short circle Interval: {:?}", got.as_ref().map(|l| l.hull_sup));
    assert!(got.is_ok(), "{got:?}");
    let got = plane_nurbs_limbs(
        &circle(0.5).map_scalar(itv),
        (itv(0.1), itv(1.4)),
        &horizontal(0.5).map_scalar(itv),
        &quarter_cylinder_wall().map_scalar(itv),
        Interval::one(),
        band(),
    );
    println!("P1 1.3 rad circle Interval: {got:?}");
    assert!(matches!(got, Err(PlaneNurbsRefusal::Limb { .. })), "{got:?}");
}

/// P2: the circle's parameter past 2π: the quarter-cylinder wall's arc
/// read at `t + 2π` is the same locus.
#[test]
fn p2_short_arc_past_two_pi() {
    let tau = core::f64::consts::TAU;
    let (a, b) = short(0.7, 0.002);
    let shifted = plane_nurbs_limbs(
        &circle(0.5),
        (a + tau, b + tau),
        &horizontal(0.5),
        &quarter_cylinder_wall(),
        1.0,
        band(),
    );
    let base = plane_nurbs_limbs(&circle(0.5), (a, b), &horizontal(0.5), &quarter_cylinder_wall(), 1.0, band());
    println!(
        "P2 base {:?} shifted {:?}",
        base.as_ref().map(|l| l.hull_sup),
        shifted.as_ref().map(|l| l.hull_sup)
    );
    assert!(base.is_ok() && shifted.is_ok(), "{base:?} {shifted:?}");
}

/// P3: a circle 3ε OFF the wall radially (radius 1 + 3ε), short arc:
/// limb 1 must see it. And a circle on the wall but whose plane is
/// tilted so it leaves the wall between samples is covered by the
/// rows; this one checks the off-locus carrier refuses at every lane.
#[test]
fn p3_short_circle_off_the_wall_refuses() {
    let (a, b) = short(0.7, 0.002);
    let off = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.5),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0 + 30.0 * eps(),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let got = plane_nurbs_limbs(&off, (a, b), &horizontal(0.5), &quarter_cylinder_wall(), 1.0, band());
    println!("P3 off circle: {got:?}");
    assert!(got.is_err(), "{got:?}");
    let _ = NurbsCurve3::<f64>::new;
    let _ = Arc::new(0);
}

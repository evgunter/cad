//! Containment pins for the certified-conservative curve boxes
//! (`geom_curves::boxes`): sampled carrier points lie inside the box,
//! span-tightness actually prunes (an arc's box excludes the far side
//! of its circle), and poison never prunes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bvh::Aabb;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
use geom_curves::boxes::{circle_arc_aabb, nurbs_curve_aabb};
use geom_curves::{Curve3, NurbsCurve3};
use proptest::prelude::*;

/// Sample-noise slack for the assertions only (libm evaluation of a
/// sample may exceed the true locus by a few ulps; the box bounds the
/// TRUE locus). Far below any consumer pad.
const SLACK: f64 = 1e-12;

fn contains(b: &Aabb, p: Point3<f64>, slack: f64) -> bool {
    p.x >= b.min_x - slack
        && p.x <= b.max_x + slack
        && p.y >= b.min_y - slack
        && p.y <= b.max_y + slack
        && p.z >= b.min_z - slack
        && p.z <= b.max_z + slack
}

fn unit_frame(seed: f64) -> (Vec3<f64>, Vec3<f64>) {
    // A deterministic axis/u_ref orthonormal pair from a seed angle.
    let axis = Vec3::new(seed.cos() * 0.6, seed.sin() * 0.6, 0.8).normalize();
    let helper = if axis.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u_ref = axis.cross(helper).normalize();
    (axis, u_ref)
}

proptest! {
    /// Every sampled arc point lies in the arc's box.
    #[test]
    fn arc_box_contains_samples(
        seed in -3.0..3.0f64,
        cx in -5.0..5.0f64, cy in -5.0..5.0f64, cz in -5.0..5.0f64,
        r in 0.01..10.0f64,
        t0 in -7.0..7.0f64,
        dt in 0.0..7.0f64,
    ) {
        let (axis, u_ref) = unit_frame(seed);
        let carrier = Curve3::Circle {
            center: Point3::new(cx, cy, cz),
            axis,
            radius: r,
            u_ref,
        };
        let t1 = t0 + dt;
        let (e0, e1) = (carrier.eval(t0), carrier.eval(t1));
        let b = circle_arc_aabb(&carrier, t0, t1, e0, e1).unwrap();
        let slack = SLACK * (1.0 + r + cx.abs() + cy.abs() + cz.abs());
        for i in 0..=32u32 {
            let t = t0 + dt * (f64::from(i) / 32.0);
            prop_assert!(contains(&b, carrier.eval(t), slack));
        }
    }
}

#[test]
fn quarter_arc_box_is_span_tight() {
    // Unit circle at origin, standard frame; arc θ ∈ [0.1, 0.4]:
    // no axis extremum lies in the span, so the box is the endpoint
    // hull (plus outward slop) — nowhere near the full circle.
    let carrier = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (t0, t1) = (0.1, 0.4);
    let b = circle_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1)).unwrap();
    assert!(b.max_x < 1.0 - 1e-3, "max_x pruned: {}", b.max_x);
    assert!(b.min_x > 0.5, "min_x pruned: {}", b.min_x);
    assert!(b.min_y > 0.0, "min_y pruned: {}", b.min_y);
    assert!(b.max_y < 0.5, "max_y pruned: {}", b.max_y);
}

#[test]
fn full_period_box_is_the_circle_box() {
    let carrier = Curve3::Circle {
        center: Point3::new(1.0, 2.0, 3.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 2.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (t0, t1) = (0.0, core::f64::consts::TAU);
    let b = circle_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1)).unwrap();
    assert!(b.min_x <= -1.0 && b.max_x >= 3.0);
    assert!(b.min_y <= 0.0 && b.max_y >= 4.0);
    // The circle is planar: z stays certified-thin around 3.
    assert!(b.min_z > 2.99 && b.max_z < 3.01);
}

#[test]
fn non_circle_carrier_is_refused() {
    let line = Curve3::Line {
        origin: Point3::new(0.0, 0.0, 0.0),
        dir: Vec3::new(1.0, 0.0, 0.0),
    };
    let p = Point3::new(0.0, 0.0, 0.0);
    assert!(circle_arc_aabb(&line, 0.0, 1.0, p, p).is_none());
}

#[test]
fn nurbs_box_is_the_control_hull_and_contains_samples() {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, -1.0, 1.0),
        Point3::new(3.0, 0.5, 0.0),
    ];
    let curve = NurbsCurve3::new(knots, control, vec![1.0, 0.5, 2.0, 1.0]).unwrap();
    let b = nurbs_curve_aabb(&curve);
    assert_eq!((b.min_x, b.max_x), (0.0, 3.0));
    assert_eq!((b.min_y, b.max_y), (-1.0, 2.0));
    assert_eq!((b.min_z, b.max_z), (0.0, 1.0));
    let c3 = Curve3::Nurbs(std::sync::Arc::new(curve));
    for i in 0..=16u32 {
        let t = f64::from(i) / 16.0;
        assert!(contains(&b, c3.eval(t), SLACK));
    }
}

#[test]
fn placeholder_nurbs_box_is_poison() {
    let Curve3::Nurbs(placeholder) = Curve3::<f64>::nurbs_placeholder() else {
        panic!("placeholder is a Nurbs variant");
    };
    let b = nurbs_curve_aabb(&placeholder);
    assert!(b.min_x.is_nan());
    // Poison never prunes.
    assert!(b.overlaps(&Aabb {
        min_x: 9e9,
        min_y: 9e9,
        min_z: 9e9,
        max_x: 9e9,
        max_y: 9e9,
        max_z: 9e9,
    }));
}

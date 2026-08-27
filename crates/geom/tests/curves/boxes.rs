//! Containment pins for the certified-conservative curve boxes
//! (`geom::curves::boxes`): sampled carrier points lie inside the box,
//! span-tightness actually prunes (an arc's box excludes the far side
//! of its circle), and poison never prunes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use bvh::Aabb;
use geom::curves::boxes::{circle_arc_aabb, ellipse_arc_aabb, nurbs_curve_aabb};
use geom::{Curve3, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};
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

proptest! {
    /// Every sampled ellipse-arc point lies in the arc's box (M5 PR 5 —
    /// the circle fuzz row, generalized to distinct semi-axes).
    #[test]
    fn ellipse_arc_box_contains_samples(
        seed in -3.0..3.0f64,
        cx in -5.0..5.0f64, cy in -5.0..5.0f64, cz in -5.0..5.0f64,
        minor in 0.01..5.0f64,
        stretch in 1.01..8.0f64,
        t0 in -7.0..7.0f64,
        dt in 0.0..7.0f64,
    ) {
        let (axis, u_ref) = unit_frame(seed);
        let major = minor * stretch;
        let carrier = Curve3::Ellipse {
            center: Point3::new(cx, cy, cz),
            axis,
            major,
            minor,
            u_ref,
        };
        let t1 = t0 + dt;
        let (e0, e1) = (carrier.eval(t0), carrier.eval(t1));
        let b = ellipse_arc_aabb(&carrier, t0, t1, e0, e1).unwrap();
        let slack = SLACK * (1.0 + major + cx.abs() + cy.abs() + cz.abs());
        for i in 0..=32u32 {
            let t = t0 + dt * (f64::from(i) / 32.0);
            prop_assert!(contains(&b, carrier.eval(t), slack));
        }
    }
}

#[test]
fn ellipse_arc_box_is_span_tight_and_full_period_reaches_extrema() {
    // Axis-aligned ellipse a = 2, b = 0.5 at the origin. The
    // DISCRIMINATING SPAN [0.1, 0.4] contains no axis extremum: the box
    // is the endpoint hull, far from the full ellipse's ±2 extent.
    let carrier = Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major: 2.0,
        minor: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let (t0, t1) = (0.1, 0.4);
    let b = ellipse_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1)).unwrap();
    assert!(b.max_x < 2.0 - 1e-3, "max_x pruned: {}", b.max_x);
    assert!(b.min_x > 1.0, "min_x pruned: {}", b.min_x);
    assert!(b.max_y < 0.25, "max_y pruned: {}", b.max_y);

    // Full period: the box reaches ±major on x, ±minor on y, and stays
    // certified-thin on z (planar locus).
    let (t0, t1) = (0.0, core::f64::consts::TAU);
    let b = ellipse_arc_aabb(&carrier, t0, t1, carrier.eval(t0), carrier.eval(t1)).unwrap();
    assert!(b.min_x <= -2.0 && b.max_x >= 2.0);
    assert!(b.min_y <= -0.5 && b.max_y >= 0.5);
    assert!(b.min_z > -0.01 && b.max_z < 0.01);
}

#[test]
fn non_ellipse_carrier_is_refused_by_the_ellipse_lane() {
    let circle = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 0.0),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: 1.0,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let p = Point3::new(1.0, 0.0, 0.0);
    assert!(ellipse_arc_aabb(&circle, 0.0, 1.0, p, p).is_none());
}

/// The wide-bracket row for the ellipse lane (the PR 8 fix's shape,
/// M5-PR5 spec §1): a wide `u_ref.x` bracket with a discriminating span
/// — the corner-interval extremal evaluation must cover every f64
/// realization.
#[cfg(feature = "interval")]
#[test]
fn wide_bracket_ellipse_box_contains_every_realization() {
    use geom_core::{Interval, Real};
    let iv = <Interval as Real>::from_f64;
    for w in [1e-6, 1e-4, 1e-2, 1e-1] {
        for (t0, t1) in [(0.0, 0.9), (0.0, 1.5), (1.2, 4.0), (-2.0, 0.5), (0.0, 7.0)] {
            let carrier_iv: Curve3<Interval> = Curve3::Ellipse {
                center: Point3::new(iv(1.0), iv(-2.0), iv(0.5)),
                axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                major: iv(2.0),
                minor: iv(0.75),
                u_ref: Vec3::new(Interval::from_bounds(-w, w), iv(1.0), iv(0.0)),
            };
            let (e0, e1) = (carrier_iv.eval(iv(t0)), carrier_iv.eval(iv(t1)));
            let b = ellipse_arc_aabb(&carrier_iv, iv(t0), iv(t1), e0, e1).unwrap();
            for i in 0..=8u32 {
                let d = -w + 2.0 * w * f64::from(i) / 8.0;
                let real: Curve3<f64> = Curve3::Ellipse {
                    center: Point3::new(1.0, -2.0, 0.5),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    major: 2.0,
                    minor: 0.75,
                    u_ref: Vec3::new(d, 1.0, 0.0),
                };
                for j in 0..=64u32 {
                    let t = t0 + (t1 - t0) * f64::from(j) / 64.0;
                    let p = real.eval(t);
                    assert!(
                        contains(&b, p, 1e-9 * (1.0 + w)),
                        "w={w:e} d={d:e} t={t}: sample {p:?} escaped {b:?}"
                    );
                }
            }
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

/// Fix-pass item 3: the WIDE-BRACKET row — `u_ref.x` enters as a wide
/// interval `[−w, w]` (half-widths up to 1e-1, far beyond the angular
/// slop), and the box built from the bracketed inputs must contain
/// every f64 realization's samples. This is exactly where a
/// midpoint-angle extremum test under-covers; the corner-interval
/// evaluation shipped in `boxes.rs` is pinned here.
#[cfg(feature = "interval")]
#[test]
fn wide_bracket_arc_box_contains_every_realization() {
    use geom_core::{Interval, Real};
    let iv = <Interval as Real>::from_f64;
    for w in [1e-6, 1e-4, 1e-2, 1e-1] {
        // (0.0, 1.5) is the discriminating span: the min-x extremal
        // angle sits at pi/2 shifted by up to atan(w) — inside the
        // span for one end of the bracket, outside for the midpoint —
        // exactly the case a midpoint-angle test under-covers.
        for (t0, t1) in [(0.0, 0.9), (0.0, 1.5), (1.2, 4.0), (-2.0, 0.5), (0.0, 7.0)] {
            // Bracketed carrier: u_ref.x = [−w, w], the rest exact.
            let carrier_iv: Curve3<Interval> = Curve3::Circle {
                center: Point3::new(iv(1.0), iv(-2.0), iv(0.5)),
                axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                radius: iv(2.0),
                u_ref: Vec3::new(Interval::from_bounds(-w, w), iv(1.0), iv(0.0)),
            };
            let (e0, e1) = (carrier_iv.eval(iv(t0)), carrier_iv.eval(iv(t1)));
            let b = circle_arc_aabb(&carrier_iv, iv(t0), iv(t1), e0, e1).unwrap();
            // Dense f64 realizations across the bracket.
            for i in 0..=8u32 {
                let d = -w + 2.0 * w * f64::from(i) / 8.0;
                let real: Curve3<f64> = Curve3::Circle {
                    center: Point3::new(1.0, -2.0, 0.5),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    radius: 2.0,
                    u_ref: Vec3::new(d, 1.0, 0.0),
                };
                for j in 0..=64u32 {
                    let t = t0 + (t1 - t0) * f64::from(j) / 64.0;
                    let p = real.eval(t);
                    assert!(
                        contains(&b, p, 1e-9 * (1.0 + w)),
                        "w={w:e} d={d:e} t={t}: sample {p:?} escaped {b:?}"
                    );
                }
            }
        }
    }
}

//! The counterexample search behind [`super::super::carrier_box`]: arcs
//! of every curved carrier kind nobody chose, in frames nobody chose,
//! sampled densely against the box the clearance gate folds for them.

test_utils::gated_to![
    "crates/topo/src/splitting/containment.rs",
    "crates/geom/src/curves.rs",
    "crates/geom/src/curves/",
    "crates/geom-core/src/linalg/",
];

use super::super::{InPlane, carrier_box};
use geom_core::{Point3, Vec3};
use std::sync::Arc;
use test_utils::fuzz::Rng;

/// A random orthonormal pair.
fn pair(rng: &mut Rng) -> (Vec3<f64>, Vec3<f64>) {
    loop {
        let a = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let b = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        if a.norm() < 0.2 {
            continue;
        }
        let a = a * (1.0 / a.norm());
        let b = b - a * b.dot(a);
        if b.norm() < 0.2 {
            continue;
        }
        return (a, b * (1.0 / b.norm()));
    }
}

fn point(rng: &mut Rng, reach: f64) -> Point3<f64> {
    Point3::new(
        rng.range(-reach, reach),
        rng.range(-reach, reach),
        rng.range(-reach, reach),
    )
}

/// A radius spread over six decades.
fn radius(rng: &mut Rng) -> f64 {
    10f64.powf(rng.range(-4.0, 2.0))
}

/// A parameter window: short, near-full, a whole turn or arbitrary,
/// one in seven reversed.
fn window(rng: &mut Rng) -> (f64, f64) {
    let t0 = rng.range(-7.0, 7.0);
    let width = match rng.below(4) {
        0 => rng.range(1e-3, 0.3),
        1 => rng.range(6.0, core::f64::consts::TAU),
        2 => core::f64::consts::TAU,
        _ => rng.range(0.0, core::f64::consts::TAU),
    };
    if rng.below(7) == 0 {
        (t0 + width, t0)
    } else {
        (t0, t0 + width)
    }
}

/// A carrier of each curved kind, by `kind`, with the window its samples
/// are drawn from.
fn carrier(rng: &mut Rng, kind: usize) -> (geom::Curve3<f64>, (f64, f64)) {
    let center = point(rng, 60.0);
    let (axis, u_ref) = pair(rng);
    match kind {
        0 => (
            geom::Curve3::Circle {
                center,
                axis,
                radius: radius(rng),
                u_ref,
            },
            window(rng),
        ),
        1 => {
            let (a, b) = (radius(rng), radius(rng));
            (
                geom::Curve3::Ellipse {
                    center,
                    axis,
                    major: a.max(b),
                    minor: a.min(b),
                    u_ref,
                },
                window(rng),
            )
        }
        2 => {
            let r = radius(rng);
            let big = r * rng.range(1.2, 5.0);
            let offset = (big - r) * rng.range(-0.95, 0.95);
            (
                geom::Curve3::Spiric {
                    center,
                    axis,
                    u_ref,
                    major_radius: big,
                    minor_radius: r,
                    offset,
                },
                window(rng),
            )
        }
        _ => {
            let n = 2 + rng.below(5);
            let degree = (n - 1).min(3);
            let mut knots = vec![0.0; degree + 1];
            for i in 1..n - degree {
                knots.push(i as f64 / (n - degree) as f64);
            }
            knots.extend(vec![1.0; degree + 1]);
            let control: Vec<_> = (0..n)
                .map(|_| center + (point(rng, 1.0) - Point3::new(0.0, 0.0, 0.0)) * radius(rng))
                .collect();
            let weights = (0..n).map(|_| rng.range(0.1, 10.0)).collect();
            let kv = geom_core::spline::KnotVector::clamped(knots, degree).unwrap();
            (
                geom::Curve3::Nurbs(Arc::new(
                    geom::NurbsCurve3::new(kv, control, weights).unwrap(),
                )),
                (0.0, 1.0),
            )
        }
    }
}

/// **Every sampled point of an arc lies in the box the gate folds for
/// it**, in a frame of its own, for circles, ellipses, spirics and
/// rational splines.
#[test]
fn every_arc_lies_in_its_footprint_box() {
    let mut rng = test_utils::fuzz::start("shell footprint box");
    for i in 0..test_utils::fuzz::scaled(2000) {
        let kind = i % 4;
        let (curve, (t0, t1)) = carrier(&mut rng, kind);
        let (u, n) = pair(&mut rng);
        let frame = InPlane {
            origin: point(&mut rng, 60.0),
            u,
            v: n.cross(u),
            n,
        };
        let [(ulo, vlo), (uhi, vhi)] = carrier_box(&curve, (t0, t1), &frame)
            .unwrap_or_else(|| panic!("#{i}: a curved carrier has a box"));
        for k in 0..=400 {
            let p = frame.point(curve.eval(t0 + (t1 - t0) * k as f64 / 400.0));
            let scale = 1e-9 * (1.0 + p.x.abs().max(p.y.abs()));
            assert!(
                p.x >= ulo - scale
                    && p.x <= uhi + scale
                    && p.y >= vlo - scale
                    && p.y <= vhi + scale,
                "#{i} (kind {kind}): sample {k} at ({}, {}) is outside [{ulo}, {uhi}] x [{vlo}, {vhi}]; {}",
                p.x,
                p.y,
                test_utils::fuzz::replay()
            );
        }
    }
}

//! The test oracles the boolean's conic rows share — the ellipse door's
//! rows, the subdivision's guard rows and the clearance rung's rows: a
//! random unit vector, the TRUE signed distance from a sphere, a wall or
//! a torus, and the sign changes along a sampled function.

use geom_core::{Point3, Vec3};
use test_utils::fuzz;

/// A uniformly drawn unit vector, away from the degenerate short ones.
pub(super) fn unit(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        if v.norm() > 0.2 && v.norm() < 1.0 {
            return v.normalize();
        }
    }
}

/// The true signed distance from a sphere, a cylinder wall or a torus
/// of the point whose offset from a given point is `from(anchor)`, read
/// from the surface's own STORED anchor (its centre or origin). A
/// caller far from the origin hands an offset it computes as
/// `(a − anchor) + …` from stored values a few metres apart, which is
/// exact, rather than a point it rounded at the scale of its
/// coordinates.
pub(super) fn distance_from_anchor(
    s: &geom::Surface<f64>,
    from: impl Fn(Point3<f64>) -> Vec3<f64>,
) -> f64 {
    match *s {
        geom::Surface::Sphere { center, radius, .. } => from(center).norm() - radius,
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let q = from(origin);
            (q - axis * q.dot(axis)).norm() - radius
        }
        geom::Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let q = from(center);
            let h = q.dot(axis);
            let rho = (q - axis * h).norm();
            (rho - major_radius).hypot(h) - minor_radius
        }
        _ => unreachable!("the oracle reads spheres, walls and tori"),
    }
}

/// The true signed distance of `p` from a sphere, a wall or a torus.
pub(super) fn distance(s: &geom::Surface<f64>, p: Point3<f64>) -> f64 {
    distance_from_anchor(s, |anchor| p - anchor)
}

/// How many times consecutive samples change sign.
pub(super) fn sign_changes(samples: &[f64]) -> usize {
    samples
        .windows(2)
        .filter(|w| (w[0] < 0.0) != (w[1] < 0.0))
        .count()
}

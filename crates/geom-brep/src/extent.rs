//! **The consumed extent** — a ball enclosing every point at which a
//! carrier verdict is consumed, and the lever arm it gives an angular
//! datum (D4 ¶1: an angle means the displacement it induces at the
//! extent over which the decision is consumed).
//!
//! A ladder that pins a carrier's position at a PIVOT (the plane's
//! foot from the world origin, a cylinder's axis point, a torus's
//! centre) and its direction by an angle reads a relative tilt θ as a
//! displacement of at most `θ · |x − pivot|` at a consumed point `x`.
//! [`ExtentBall::lever_from`] is the supremum of `|x − pivot|` over
//! the ball, so a tilt levered there that reads inside the band stands
//! inside the band everywhere the ball covers. An extent that
//! UNDER-states the consumed region makes a tilt read smaller than it
//! is, which is the wrong-answer direction, so every constructor here
//! encloses: a looser ball only refuses more.
//!
//! Everything is comparison-free: `max` is the [`Real`] lattice
//! operation.

use geom::{Curve3, Surface};
use geom_core::{Point3, Real};

/// A closed ball `|x − center| ≤ radius` enclosing a consumed region
/// (module docs).
#[derive(Clone, Copy, Debug)]
pub struct ExtentBall<T: Real> {
    center: Point3<T>,
    radius: T,
}

impl<T: Real> ExtentBall<T> {
    /// The ball of `radius` about `center`. `radius` is a length the
    /// caller vouches encloses the region it names.
    #[must_use]
    pub fn new(center: Point3<T>, radius: T) -> Self {
        Self { center, radius }
    }

    /// The single point `p`.
    #[must_use]
    pub fn point(p: Point3<T>) -> Self {
        Self::new(p, T::zero())
    }

    /// A ball enclosing both: centred on `self`'s centre, wide enough
    /// to reach the far side of `other`.
    #[must_use]
    pub fn hull(self, other: Self) -> Self {
        let reach = (other.center - self.center).norm() + other.radius;
        Self::new(self.center, self.radius.max(reach))
    }

    /// The farthest the ball reaches from `pivot`: the lever arm at
    /// which an angular datum pinned at `pivot` is metered.
    #[must_use]
    pub fn lever_from(self, pivot: Point3<T>) -> T {
        (pivot - self.center).norm() + self.radius
    }

    /// The ball a BOUNDED carrier fits in, which encloses every face on
    /// it whatever its trim: a sphere's own ball, a torus's ball of
    /// radius `R + r` about its centre, a NURBS patch's control-net
    /// ball (the convex-hull property, weights positive by
    /// validation).
    ///
    /// `None` for the RULED kinds — plane, cylinder, cone. Every point
    /// of such a carrier lies on a line in it, and the squared distance
    /// to a fixed point is convex along a line, so over a compact face
    /// its maximum sits on the face's boundary:
    /// [`ExtentBall::of_curve`] over the boundary edges encloses the
    /// face.
    #[must_use]
    pub fn of_carrier(surface: &Surface<T>) -> Option<Self> {
        match surface {
            Surface::Plane { .. } | Surface::Cylinder { .. } | Surface::Cone { .. } => None,
            Surface::Sphere { center, radius, .. } => Some(Self::new(*center, *radius)),
            Surface::Torus {
                center,
                major_radius,
                minor_radius,
                ..
            } => Some(Self::new(*center, *major_radius + *minor_radius)),
            Surface::Nurbs(patch) => Some(Self::of_points(patch.control())),
            Surface::Approx(a) => Some(Self::of_points(a.fit().control())),
        }
    }

    /// A ball enclosing the edge `carrier` traces from `start` to
    /// `end`: the chord's ball for a line, the whole conic for a circle
    /// or an ellipse, the host torus's ball for a spiric, the control
    /// polygon's ball for a NURBS curve.
    #[must_use]
    pub fn of_curve(carrier: &Curve3<T>, start: Point3<T>, end: Point3<T>) -> Self {
        match carrier {
            Curve3::Line { .. } => {
                let half = (end - start) * T::from_f64(0.5);
                Self::new(start + half, half.norm())
            }
            Curve3::Circle { center, radius, .. } => Self::new(*center, *radius),
            Curve3::Ellipse {
                center,
                major,
                minor,
                ..
            } => Self::new(*center, major.max(*minor)),
            Curve3::Spiric {
                center,
                major_radius,
                minor_radius,
                ..
            } => Self::new(*center, *major_radius + *minor_radius),
            Curve3::Nurbs(curve) => Self::of_points(curve.control()),
        }
    }

    /// The ball about the first point reaching every other; an empty
    /// slice is the origin point (a validated net is never empty).
    fn of_points(points: &[Point3<T>]) -> Self {
        let Some(first) = points.first() else {
            return Self::point(Point3::origin());
        };
        points
            .iter()
            .fold(Self::point(*first), |ball, p| ball.hull(Self::point(*p)))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Vec3;

    /// The torus's lever from its own centre is `R + r`: the farthest
    /// any point of the ring stands from the pivot its axis turns on.
    #[test]
    fn a_torus_levers_at_its_outer_radius() {
        let torus = Surface::Torus {
            center: Point3::new(1.0, 2.0, 3.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            major_radius: 2.0,
            minor_radius: 0.5,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let ball = ExtentBall::of_carrier(&torus).unwrap();
        assert_eq!(ball.lever_from(Point3::new(1.0, 2.0, 3.0)), 2.5);
        assert_eq!(ball.lever_from(Point3::new(1.0, 2.0, 7.0)), 6.5);
    }

    /// A hull reaches the far side of both balls, and a line's ball is
    /// its chord's.
    #[test]
    fn a_hull_encloses_both_and_a_chord_is_its_own_ball() {
        let line = Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, 0.0, 0.0),
        };
        let a = ExtentBall::of_curve(&line, Point3::new(0.0, 0.0, 0.0), Point3::new(10.0, 0.0, 0.0));
        assert_eq!(a.lever_from(Point3::new(5.0, 0.0, 0.0)), 5.0);
        let b = ExtentBall::new(Point3::new(-3.0, 0.0, 0.0), 1.0);
        let both = a.hull(b);
        assert_eq!(both.lever_from(Point3::new(5.0, 0.0, 0.0)), 9.0);
        assert_eq!(b.hull(a).lever_from(Point3::new(-3.0, 0.0, 0.0)), 13.0);
    }
}

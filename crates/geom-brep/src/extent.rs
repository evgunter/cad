//! **The consumed extent** — a ball enclosing every point at which a
//! carrier verdict is consumed, and the lever arm it gives an angular
//! datum (D4 ¶1: an angle means the displacement it induces at the
//! extent over which the decision is consumed).
//!
//! A ladder that pins a carrier's position at a PIVOT and its
//! direction by an angle reads a relative tilt θ as a displacement of
//! at most `θ · |x − pivot|` at a consumed point `x`, on top of what
//! the position datum reads at the pivot.
//! [`ExtentBall::lever_from`] is the supremum of `|x − pivot|` over
//! the ball, so a tilt levered there that reads inside the band stands
//! inside the band everywhere the ball covers. The tightest such lever
//! reads the position datum at the pivot nearest the ball's centre
//! ([`ExtentBall::foot_on`] for an axis), where the lever is little
//! more than the ball's radius.
//!
//! An extent that UNDER-states the consumed region makes a tilt read
//! smaller than it is, which is the wrong-answer direction, so every
//! constructor here encloses: a looser ball only refuses more.
//!
//! Everything is comparison-free: `max` and `min` are the [`Real`]
//! lattice operations.

use geom::Surface;
use geom_core::{Point3, Real, Vec3};

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

    /// A ball enclosing every ball of `parts`: about the mean of their
    /// centres, reaching the far side of each. `None` for no parts.
    #[must_use]
    pub fn enclosing(parts: &[Self]) -> Option<Self> {
        let first = parts.first()?;
        let n = T::from_f64(parts.len() as f64);
        let sum = parts
            .iter()
            .fold(Vec3::new(T::zero(), T::zero(), T::zero()), |acc, b| {
                acc + (b.center - first.center)
            });
        let center = first.center + sum / n;
        let radius = parts
            .iter()
            .fold(T::zero(), |r, b| r.max(b.lever_from(center)));
        Some(Self::new(center, radius))
    }

    /// The ball's centre.
    #[must_use]
    pub fn center(self) -> Point3<T> {
        self.center
    }

    /// The farthest the ball reaches from `pivot`: the lever arm at
    /// which an angular datum pinned at `pivot` is metered.
    #[must_use]
    pub fn lever_from(self, pivot: Point3<T>) -> T {
        (pivot - self.center).norm() + self.radius
    }

    /// The point of the line `origin + s·axis` (`axis` unit) nearest
    /// the ball's centre: where a datum on that line is read so that
    /// the tilt's lever from it is least.
    #[must_use]
    pub fn foot_on(self, origin: Point3<T>, axis: Vec3<T>) -> Point3<T> {
        origin + axis * (self.center - origin).dot(axis)
    }

    /// The ball enclosing the box `[lo, hi]`: its centre, out to a
    /// corner.
    #[must_use]
    pub fn of_box(lo: Point3<T>, hi: Point3<T>) -> Self {
        let half = (hi - lo) * T::from_f64(0.5);
        Self::new(lo + half, half.norm())
    }

    /// The ball a sphere or a torus fits in, which encloses every face
    /// on it whatever its trim: the sphere's own, the torus's of radius
    /// `R + r` about its centre. `None` for every other kind, whose
    /// faces a box of their own encloses.
    #[must_use]
    pub fn of_carrier(surface: &Surface<T>) -> Option<Self> {
        match surface {
            Surface::Sphere { center, radius, .. } => Some(Self::new(*center, *radius)),
            Surface::Torus {
                center,
                major_radius,
                minor_radius,
                ..
            } => Some(Self::new(*center, *major_radius + *minor_radius)),
            Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Nurbs(_)
            | Surface::Approx(_) => None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    fn xyz(p: Point3<f64>) -> [f64; 3] {
        [p.x, p.y, p.z]
    }

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

    /// The enclosing ball sits at the parts' mean and reaches each
    /// one's far side: a square's corners give its circumscribed ball,
    /// and a circle beside them widens it to the circle's far side.
    #[test]
    fn an_enclosing_ball_reaches_every_parts_far_side() {
        let corners = [(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]
            .map(|(x, y)| ExtentBall::point(Point3::new(x, y, 0.0)));
        let square = ExtentBall::enclosing(&corners).unwrap();
        assert_eq!(xyz(square.center()), [1.0, 1.0, 0.0]);
        assert_eq!(square.lever_from(square.center()), 2.0f64.sqrt());
        let mut parts = corners.to_vec();
        parts.push(ExtentBall::new(Point3::new(6.0, 1.0, 0.0), 1.0));
        let both = ExtentBall::enclosing(&parts).unwrap();
        assert_eq!(xyz(both.center()), [2.0, 1.0, 0.0]);
        assert_eq!(both.lever_from(both.center()), 5.0);
        assert!(ExtentBall::<f64>::enclosing(&[]).is_none());
        let slab = ExtentBall::of_box(Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 2.0, 0.0));
        assert_eq!(xyz(slab.center()), xyz(square.center()));
        assert_eq!(slab.lever_from(slab.center()), 2.0f64.sqrt());
    }
}

//! **The consumed extent** — a ball enclosing every point at which a
//! carrier verdict is consumed, and the lever arm it gives an angular
//! datum (D4 ¶1: an angle means the displacement it induces at the
//! extent over which the decision is consumed).
//!
//! A ladder that pins a carrier's position at a PIVOT and its
//! direction by an angle reads a relative tilt θ as a displacement of
//! at most `θ · |x − pivot|` at a consumed point `x`, ON TOP OF what
//! the position datum reads at the pivot. [`ExtentBall::lever_from`] is
//! the supremum of `|x − pivot|` over the ball, so the displacement at
//! every point the ball covers is at most the position datum plus the
//! tilt levered there: a reader that bridges a residue decides that
//! SUM, never the two terms one at a time (each just inside the band
//! would sum to nearly twice it). The tightest lever reads the position
//! datum at the pivot nearest the ball's centre ([`ExtentBall::foot_on`]
//! for an axis), where the lever is little more than the ball's radius.
//!
//! An extent that UNDER-states the consumed region makes a tilt read
//! smaller than it is, which is the wrong-answer direction, so every
//! constructor here encloses: a looser ball only escalates more. The
//! converse does not hold — a ball says nothing about where the
//! consumed region actually reaches, so a displacement read at its far
//! side is an upper bound only, and never evidence that a consumed
//! point stands that far off.
//!
//! Everything is comparison-free: `max` and `min` are the [`Real`]
//! lattice operations.

use geom::Surface;
use geom_core::{Point3, Real, Vec3, is_finite_length};

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

    /// The ball's radius: its lever from its own centre.
    #[must_use]
    pub fn radius(self) -> T {
        self.radius
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

    /// The ball, if it reads: `None` where its centre or radius is
    /// poison or infinite (a box with no claim to make), whose lever
    /// would meter nothing.
    #[must_use]
    pub fn readable(self) -> Option<Self> {
        let c = self.center;
        [c.x, c.y, c.z, self.radius]
            .into_iter()
            .all(is_finite_length)
            .then_some(self)
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
    /// A lopsided pair: a point beside a far larger ball, inside it. The
    /// enclosure sits at the mean of the centres, so it is looser than
    /// the minimal ball (the large ball itself) by the distance from the
    /// mean to that ball's centre — and still reaches the large ball's
    /// far side, which is the direction that matters.
    #[test]
    fn a_lopsided_enclosure_reaches_the_large_parts_far_side() {
        let point = ExtentBall::point(Point3::new(0.0, 0.0, 0.0));
        let large = ExtentBall::new(Point3::new(10.0, 0.0, 0.0), 100.0);
        let both = ExtentBall::enclosing(&[point, large]).unwrap();
        assert_eq!(xyz(both.center()), [5.0, 0.0, 0.0]);
        assert_eq!(both.radius(), 105.0, "the mean's lever to the far side");
        for far in [
            Point3::new(110.0, 0.0, 0.0),
            Point3::new(-90.0, 0.0, 0.0),
            Point3::new(10.0, 100.0, 0.0),
        ] {
            assert!(
                (far - both.center()).norm() <= both.radius(),
                "{far:?} of the large ball is enclosed"
            );
        }
    }

    /// **Rounding.** Parts far from the origin, at coordinates whose
    /// ulp is coarse next to their radii: the mean rounds, and each
    /// lever rounds. Sample points on every part's sphere stand within
    /// a few ulps of the radius — a rounding the band dwarfs, where a
    /// centre or radius that dropped a part's far side would miss by
    /// that part's whole radius.
    #[test]
    fn an_enclosure_far_from_the_origin_rounds_within_ulps() {
        let base = 1.0e8;
        let parts = [
            ExtentBall::new(Point3::new(base + 0.3, base - 0.7, 1.0), 0.1),
            ExtentBall::new(Point3::new(base + 3.1, base + 0.2, -2.0), 1.7),
            ExtentBall::point(Point3::new(base - 1.9, base + 2.3, 0.5)),
        ];
        let ball = ExtentBall::enclosing(&parts).unwrap();
        let slack = 4.0 * f64::EPSILON * (base + ball.radius());
        let dirs = [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ];
        for part in parts {
            for d in dirs {
                let p = part.center() + d * part.radius();
                let over = (p - ball.center()).norm() - ball.radius();
                assert!(
                    over <= slack,
                    "{p:?} stands {over:e} past the enclosure (slack {slack:e})"
                );
            }
        }
    }
}

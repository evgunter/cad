//! The planar line carrier: `origin + dir·t` in the plane, the sibling
//! of [`crate::Arc2`] and the planar counterpart of `geom::Line`.
//!
//! [`Line2::at`] is the point at `t`. The **parameter readings**
//! ([`Line2::param`], [`Line2::param_within`], [`Line2::foot_within`])
//! divide by `|dir|²`, as `geom::Line`'s do, so they hold for any
//! nonzero `dir`, a chord `b − a` read over the span `(0, 1)` among
//! them. The **metric readings** ([`Line2::height`], [`Line2::side`],
//! [`Line2::distance`], and [`Line2::circle_heights`] from them)
//! divide by `|dir|`, metres for any nonzero `dir`; `side` is the
//! planar one `geom::Line` has no counterpart of, signed by the
//! perp-dot. **`dir` must be nonzero**: on a zero `dir` every reading
//! divides `0/0`.
//!
//! [`Line2::meet`] is where two lines cross: the caller decides their
//! determinant `self.dir × other.dir` through the band under its own
//! gate, and only a determinant decided nonzero is divided by
//! ([`LineMeet`]).

use crate::linalg::{Point2, Vec2};
use crate::{Real, Sign};

/// **Where two planar lines meet** ([`Line2::meet`]).
#[derive(Clone, Copy, Debug)]
pub enum LineMeet<T: Real> {
    /// The lines cross once, at `self.at(t)` and `other.at(s)`. `turn`
    /// is the determinant's decided sign, positive where `other.dir`
    /// turns left of `self.dir`.
    Cross {
        /// The crossing's parameter on `self`.
        t: T,
        /// The crossing's parameter on `other`.
        s: T,
        /// The decided sign of `det`, never `Zero`.
        turn: Sign,
        /// `self.dir × other.dir`.
        det: T,
    },
    /// The determinant was decided zero: the lines are parallel to
    /// within the band, and nothing was divided by it.
    Parallel {
        /// `self.dir × other.dir`.
        det: T,
    },
}

/// A line in the plane, `origin + dir·t` (module docs).
#[derive(Clone, Copy, Debug)]
pub struct Line2<T: Real> {
    /// The point at `t = 0`.
    pub origin: Point2<T>,
    /// The direction increasing `t` runs along.
    pub dir: Vec2<T>,
}

impl<T: Real> Line2<T> {
    /// The line's point at `t`: `origin + dir·t`.
    #[must_use]
    pub fn at(&self, t: T) -> Point2<T> {
        self.origin + self.dir * t
    }

    /// The parameter of `p`'s foot: `(p − origin)·dir / |dir|²`.
    #[must_use]
    pub fn param(&self, p: Point2<T>) -> T {
        (p - self.origin).dot(self.dir) / self.dir.norm_squared()
    }

    /// **The parameter of `p`'s nearest point of the span `(t0, t1)`**,
    /// `t0 ≤ t1`: [`Self::param`] held to the span by `max`/`min`, no
    /// comparison. A segment from `a` to `b` is the span `(0, 1)` of
    /// `Line2 { origin: a, dir: b − a }`.
    #[must_use]
    pub fn param_within(&self, p: Point2<T>, (t0, t1): (T, T)) -> T {
        self.param(p).max(t0).min(t1)
    }

    /// **`p`'s foot on the span `(t0, t1)`**: the line's point at
    /// [`Self::param_within`].
    #[must_use]
    pub fn foot_within(&self, p: Point2<T>, span: (T, T)) -> Point2<T> {
        self.at(self.param_within(p, span))
    }

    /// **`p`'s height along the line**: `(p − origin)·dir / |dir|`, metres
    /// from `origin`. Where `dir` is unit it is [`Self::param`].
    #[must_use]
    pub fn height(&self, p: Point2<T>) -> T {
        (p - self.origin).dot(self.dir) / self.dir.norm()
    }

    /// **`p`'s signed distance from the line**: `dir × (p − origin) / |dir|`
    /// (the perp-dot), positive on the left of `dir`.
    #[must_use]
    pub fn side(&self, p: Point2<T>) -> T {
        self.dir.perp_dot(p - self.origin) / self.dir.norm()
    }

    /// **`p`'s distance from the line**: [`Self::side`]'s magnitude.
    #[must_use]
    pub fn distance(&self, p: Point2<T>) -> T {
        self.side(p).abs()
    }

    /// **Where this line and `other` cross**, by Cramer's rule:
    /// `t = w × other.dir / det` and `s = w × self.dir / det`, with
    /// `w = other.origin − self.origin` and `det = self.dir × other.dir`.
    /// `decide` reads `det` through the band under the caller's own gate
    /// and lever, and returns its sign (or the caller's refusal). A
    /// sign decided `Zero` is [`LineMeet::Parallel`], and the parameters
    /// are not formed. A caller that knows the sign already (two
    /// segments whose ends were decided on opposite sides of each
    /// other's line) passes that decided sign.
    ///
    /// # Errors
    ///
    /// Whatever `decide` refuses with.
    pub fn meet<E>(
        &self,
        other: &Self,
        decide: impl FnOnce(T) -> Result<Sign, E>,
    ) -> Result<LineMeet<T>, E> {
        let det = self.dir.perp_dot(other.dir);
        let turn = decide(det)?;
        if turn == Sign::Zero {
            return Ok(LineMeet::Parallel { det });
        }
        let w = other.origin - self.origin;
        Ok(LineMeet::Cross {
            t: w.perp_dot(other.dir) / det,
            s: w.perp_dot(self.dir) / det,
            turn,
            det,
        })
    }

    /// **The heights at which the line meets the circle** about `centre`
    /// of squared radius `r_sq`, lower first: the centre's
    /// [`Self::height`] `∓ √(r² − s²)`, `s` its [`Self::side`]. The
    /// squared radius goes in as given, so no `√(r²)²` is taken on the
    /// way. Where `dir` is unit the heights are parameters for
    /// [`Self::at`]. The root is NaN where the line misses the circle:
    /// the caller decides the clearance `r − |s|` first.
    #[must_use]
    pub fn circle_heights(&self, centre: Point2<T>, r_sq: T) -> [T; 2] {
        let along = self.height(centre);
        let half = (r_sq - self.side(centre).powi(2)).sqrt();
        [along - half, along + half]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A line through `(1, 2)` along a non-unit `(3, 4)`, `|dir| = 5`.
    fn slanted() -> Line2<f64> {
        Line2 {
            origin: Point2::new(1.0, 2.0),
            dir: Vec2::new(3.0, 4.0),
        }
    }

    #[test]
    fn side_is_positive_on_the_left_of_dir_in_metres() {
        let line = slanted();
        // The left normal of (3, 4) is (−4, 3); 2 m along it, and 2 m
        // along the right normal (4, −3).
        let left = line.at(0.5) + Vec2::new(-4.0, 3.0) * (2.0 / 5.0);
        let right = line.at(0.5) + Vec2::new(4.0, -3.0) * (2.0 / 5.0);
        assert!((line.side(left) - 2.0).abs() < 1e-15, "{}", line.side(left));
        assert!(
            (line.side(right) + 2.0).abs() < 1e-15,
            "{}",
            line.side(right)
        );
        assert!((line.distance(right) - 2.0).abs() < 1e-15);
        assert!(line.side(line.at(-7.0)).abs() < 1e-14);
    }

    #[test]
    fn param_within_clamps_to_the_span_and_foot_within_lands_there() {
        let line = slanted();
        let off = Vec2::new(-4.0, 3.0);
        for (t, held) in [(-0.5, 0.0), (0.25, 0.25), (1.75, 1.0)] {
            let p = line.at(t) + off;
            assert!((line.param(p) - t).abs() < 1e-15, "param {t}");
            let got = line.param_within(p, (0.0, 1.0));
            assert!((got - held).abs() < 1e-15, "param_within {t}: {got}");
            let foot = line.foot_within(p, (0.0, 1.0));
            assert!(
                (foot - line.at(held)).norm() < 1e-14,
                "foot_within {t}: {foot:?}"
            );
        }
    }

    #[test]
    fn height_is_metres_and_circle_heights_meet_the_circle() {
        let line = slanted();
        assert!((line.height(line.at(2.0)) - 10.0).abs() < 1e-14);
        // A circle of radius 5 about a point 3 m left of the line, 5 m
        // along it: the meetings stand 4 m either side.
        let unit = Line2 {
            origin: line.origin,
            dir: line.dir / 5.0,
        };
        let centre = unit.at(5.0) + Vec2::new(-4.0, 3.0) * (3.0 / 5.0);
        let [lo, hi] = line.circle_heights(centre, 25.0);
        assert!(
            (lo - 1.0).abs() < 1e-14 && (hi - 9.0).abs() < 1e-14,
            "{lo} {hi}"
        );
        for h in [lo, hi] {
            assert!(((unit.at(h) - centre).norm() - 5.0).abs() < 1e-14);
        }
        assert!(line.circle_heights(centre, 4.0)[0].is_nan());
    }
}

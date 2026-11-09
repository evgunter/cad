//! The planar line carrier: `origin + dir·t` in the plane, the sibling
//! of [`crate::Arc2`] and the planar counterpart of `geom::Line`.
//!
//! Its readings follow `geom::Line`'s. The **parameter readings**
//! ([`Line2::param`], [`Line2::param_within`], [`Line2::foot_within`])
//! divide by `|dir|²`, so they hold for any nonzero `dir`, a chord
//! `b − a` read over the span `(0, 1)` among them. The **metric readings**
//! ([`Line2::height`], [`Line2::distance`]) divide by `|dir|`, metres for
//! any nonzero `dir`. **`dir` must be nonzero**: on a zero `dir` every
//! reading divides `0/0`.

use crate::Real;
use crate::linalg::{Point2, Vec2};

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
}

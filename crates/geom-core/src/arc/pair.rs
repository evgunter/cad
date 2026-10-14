//! Two circles in one plane: the margins that decide how they sit, and
//! the radical foot and half-chord that place where they meet, read from
//! one set of fields.
//!
//! [`Circles`] forms the centre distance `d`, `r₁ + r₂` and `r₁ − r₂`
//! once. The identity, external and internal margins are read from
//! those fields, and so are the four factors of the half-chord's square,
//! so a caller that decides the margins and then places the crossings
//! takes factors that are its decided margins bit for bit. [`Circles2`]
//! holds the two centres beside it and places the foot and the
//! crossings in the plane; a caller on a plane in space places them in
//! its own frame from [`Circles::radical`] and [`Circles::half_chord`].
//!
//! The textbook placement, `a = (d² + r₁² − r₂²)/2d` and
//! `h = √(r₁² − a²)`, is two differences of squares. Where the radii
//! differ by orders of magnitude, `r₁² − a²` is a difference of numbers
//! near `r₁²` whose true value is at most `r₂²`, so at `f64` it takes the
//! crossings off the smaller circle by up to its whole radius. Here
//! `r₁ − a = (r₁ + r₂ − d)(d − r₁ + r₂)/2d` and
//! `h² = (r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`:
//! each factor rounds at the inputs' own scale, and the crossings sit on
//! both circles to a few ulps of the largest of `d`, the radii and the
//! centres' coordinates.

use crate::Real;
use crate::linalg::{Point2, Vec2};

/// Two circles in one plane, by their centre distance and radii: `d`,
/// the first radius, and `r₁ + r₂` and `r₁ − r₂`, each formed once.
#[derive(Clone, Copy, Debug)]
pub struct Circles<T: Real> {
    d: T,
    r1: T,
    sum: T,
    diff: T,
}

impl<T: Real> Circles<T> {
    /// The circles of radii `r1` and `r2` whose centres stand `d` apart.
    #[must_use]
    pub fn new(d: T, r1: T, r2: T) -> Self {
        Self {
            d,
            r1,
            sum: r1 + r2,
            diff: r1 - r2,
        }
    }

    /// The centre distance.
    #[must_use]
    pub fn d(self) -> T {
        self.d
    }

    /// The identity margin, `d + |r₁ − r₂|`: zero exactly when the two
    /// are one circle.
    #[must_use]
    pub fn identity(self) -> T {
        self.d + self.diff.abs()
    }

    /// The external clearance, `d − (r₁ + r₂)`: positive when each lies
    /// outside the other, zero at external tangency.
    #[must_use]
    pub fn external(self) -> T {
        self.d - self.sum
    }

    /// The internal clearance, `d − |r₁ − r₂|`: negative when one lies
    /// inside the other, zero at internal tangency.
    #[must_use]
    pub fn internal(self) -> T {
        self.d - self.diff.abs()
    }

    /// The nesting margin, `|r₁ − r₂| − d`: positive when one lies
    /// strictly inside the other. It is [`Self::internal`] negated, and
    /// is formed as its own difference so that an exact zero reads `+0`
    /// where the negation would read `−0`.
    #[must_use]
    pub fn nested(self) -> T {
        self.diff.abs() - self.d
    }

    /// `(r₁ − a)·2d = (r₁ + r₂ − d)(d − r₁ + r₂)` and `2d`. The first
    /// factor is minus the external clearance, and the second is the
    /// internal clearance or the identity margin as `r₁ − r₂` is
    /// non-negative or negative.
    fn inner(self) -> (T, T) {
        let two_d = self.d + self.d;
        ((self.sum - self.d) * (self.d - self.diff), two_d)
    }

    /// The radical line's signed distance `a` from the first centre
    /// along the line of centres, `r₁ − (r₁ + r₂ − d)(d − r₁ + r₂)/2d`:
    /// the foot of the common chord, and an internal tangency's contact
    /// point. `d` must be definitely positive.
    #[must_use]
    pub fn radical(self) -> T {
        let (inner, two_d) = self.inner();
        self.r1 - inner / two_d
    }

    /// The half-chord's square,
    /// `(r₁ + r₂ − d)(d − r₁ + r₂)(d + r₁ − r₂)(d + r₁ + r₂)/4d²`.
    ///
    /// Where the caller has decided the external clearance definitely
    /// negative and the internal one definitely positive (a proper
    /// secant), each factor is a definite margin or a sum of
    /// non-negatives, so the product is positive at every scalar.
    /// Elsewhere it may be negative: within the band of a tangency, the
    /// circles may stand apart or nested by up to the band.
    #[must_use]
    pub fn half_chord_squared(self) -> T {
        let (inner, two_d) = self.inner();
        inner * ((self.d + self.diff) * (self.sum + self.d)) / two_d.powi(2)
    }

    /// The half-chord `h`, the square root of
    /// [`Self::half_chord_squared`] with no clamp: called outside a
    /// decided secant, a negative square is the poison (NaN at `f64`, an
    /// out-of-domain enclosure at `Interval`), which every decision
    /// downstream reads as indeterminate rather than as a point.
    #[must_use]
    pub fn half_chord(self) -> T {
        self.half_chord_squared().sqrt()
    }
}

/// [`Circles`] with its two centres, placing the radical foot and the
/// crossings in the plane.
#[derive(Clone, Copy, Debug)]
pub struct Circles2<T: Real> {
    c1: Point2<T>,
    c2: Point2<T>,
    pair: Circles<T>,
}

impl<T: Real> Circles2<T> {
    /// The circles `(centre, radius)`; `d` is `c1.distance(c2)`.
    #[must_use]
    pub fn new((c1, r1): (Point2<T>, T), (c2, r2): (Point2<T>, T)) -> Self {
        Self {
            c1,
            c2,
            pair: Circles::new(c1.distance(c2), r1, r2),
        }
    }

    /// The margins and the scalar placement.
    #[must_use]
    pub fn pair(self) -> Circles<T> {
        self.pair
    }

    /// The radical foot, `c₁ + û·a` on the line of centres: an internal
    /// tangency's contact point. `d` must be definitely positive.
    #[must_use]
    pub fn foot(self) -> Point2<T> {
        self.c1 + ((self.c2 - self.c1) / self.pair.d) * self.pair.radical()
    }

    /// The two crossings of a proper secant, either side of the radical
    /// foot by the half-chord ([`Circles::half_chord`]); the first is to
    /// the left of `c₁ → c₂`. `d` must be definitely positive, and the
    /// crossings are only points where the caller has decided the pair a
    /// secant.
    #[must_use]
    pub fn crossings(self) -> [Point2<T>; 2] {
        let a = self.pair.radical();
        let h = self.pair.half_chord();
        let u = (self.c2 - self.c1) / self.pair.d;
        let n = Vec2::new(-u.y, u.x);
        let foot = self.c1 + u * a;
        [foot + n * h, foot - n * h]
    }
}

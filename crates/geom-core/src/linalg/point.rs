//! Points of the 2-D/3-D affine spaces.
//!
//! See the [module docs](super) for the affine/linear split: these are the
//! location types. The algebra is the torsor structure and nothing more —
//! `Point − Point = Vec`, `Point ± Vec = Point`, affine combinations
//! ([`Point2::lerp`] / [`Point3::lerp`]). `Point + Point` and `Point * s`
//! do not typecheck, which is the type system enforcing "locations have no
//! linear structure" for us.

use core::ops::{Add, Sub};

use crate::linalg::{Vec2, Vec3};
use crate::real::Real;

/// A point of the 2-D affine space.
#[derive(Clone, Copy, Debug)]
pub struct Point2<T: Real> {
    /// The x coordinate (relative to [`Point2::origin`]).
    pub x: T,
    /// The y coordinate (relative to [`Point2::origin`]).
    pub y: T,
}

/// A point of the 3-D affine space.
#[derive(Clone, Copy, Debug)]
pub struct Point3<T: Real> {
    /// The x coordinate (relative to [`Point3::origin`]).
    pub x: T,
    /// The y coordinate (relative to [`Point3::origin`]).
    pub y: T,
    /// The z coordinate (relative to [`Point3::origin`]).
    pub z: T,
}

impl<T: Real> Point2<T> {
    /// Builds a point from its coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// The same point read at another scalar: `f` applied to each
    /// coordinate, in `x, y` order. A structural map — no arithmetic,
    /// so it is exact whenever `f` is (`Real::from_f64`,
    /// `Dual::constant`).
    #[must_use]
    pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> Point2<U> {
        Point2::new(f(self.x), f(self.y))
    }

    /// The coordinate origin — the base point of the chart that
    /// identifies the affine space with its coordinate tuples. Nothing
    /// geometric is special about it; it is the reference point the
    /// coordinate fields are relative to.
    pub fn origin() -> Self {
        Self::new(T::zero(), T::zero())
    }

    /// The affine combination `(1 − t)·self + t·other`, computed exactly
    /// as `self + (other − self) * t`.
    ///
    /// That formula choice (one difference, one scale, one add — not the
    /// two-products form) is deliberate: at `t = 0` the result is `self`
    /// bit-exactly (adding the zero vector; up to the sign of a zero
    /// coordinate), and at `t = 1` it is `other` up to one rounding of
    /// the difference-and-add round trip — the endpoint behavior curve
    /// evaluation wants. This is the one affine combination consumers
    /// need for M2 curve evaluation (parameter-line and chord points);
    /// `t` outside [0, 1] extrapolates on the same line, no clamping.
    pub fn lerp(self, other: Self, t: T) -> Self {
        self + (other - self) * t
    }

    /// The componentwise minimum — the lower bounding-box corner of two
    /// points. Inherits [`Real::min`]'s lattice-not-control-flow
    /// contract: ties keep `self`'s coordinate, and **NaN propagates**
    /// per component (a poisoned coordinate poisons the corner).
    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    /// The componentwise maximum — the upper bounding-box corner. Same
    /// contract as [`Point2::min`].
    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y))
    }

    /// The squared distance to `other`: `(other − self).norm_squared()`
    /// (difference first, then the fixed-association dot — see
    /// [`Vec2::dot`]).
    pub fn distance_squared(self, other: Self) -> T {
        (other - self).norm_squared()
    }

    /// The distance to `other`: `(other − self).norm()`.
    pub fn distance(self, other: Self) -> T {
        (other - self).norm()
    }
}

impl<T: Real> Point3<T> {
    /// Builds a point from its coordinates.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// The same point read at another scalar: `f` applied to each
    /// coordinate, in `x, y, z` order (see [`Point2::map`]).
    #[must_use]
    pub fn map<U: Real>(self, f: impl Fn(T) -> U) -> Point3<U> {
        Point3::new(f(self.x), f(self.y), f(self.z))
    }

    /// The coordinate origin — the base point of the chart that
    /// identifies the affine space with its coordinate tuples. Nothing
    /// geometric is special about it; it is the reference point the
    /// coordinate fields are relative to.
    pub fn origin() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    /// The affine combination `(1 − t)·self + t·other`, computed exactly
    /// as `self + (other − self) * t`.
    ///
    /// That formula choice (one difference, one scale, one add — not the
    /// two-products form) is deliberate: at `t = 0` the result is `self`
    /// bit-exactly (adding the zero vector; up to the sign of a zero
    /// coordinate), and at `t = 1` it is `other` up to one rounding of
    /// the difference-and-add round trip — the endpoint behavior curve
    /// evaluation wants. This is the one affine combination consumers
    /// need for M2 curve evaluation (parameter-line and chord points);
    /// `t` outside [0, 1] extrapolates on the same line, no clamping.
    pub fn lerp(self, other: Self, t: T) -> Self {
        self + (other - self) * t
    }

    /// The componentwise minimum — the lower bounding-box corner of two
    /// points. Inherits [`Real::min`]'s lattice-not-control-flow
    /// contract: ties keep `self`'s coordinate, and **NaN propagates**
    /// per component (a poisoned coordinate poisons the corner).
    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }

    /// The componentwise maximum — the upper bounding-box corner. Same
    /// contract as [`Point3::min`].
    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }

    /// The squared distance to `other`: `(other − self).norm_squared()`
    /// (difference first, then the fixed-association dot — see
    /// [`Vec3::dot`]).
    pub fn distance_squared(self, other: Self) -> T {
        (other - self).norm_squared()
    }

    /// The distance to `other`: `(other − self).norm()`.
    pub fn distance(self, other: Self) -> T {
        (other - self).norm()
    }
}

/// `Point − Point = Vec`: the displacement from `rhs` to `self` — the
/// defining map of the torsor structure.
impl<T: Real> Sub for Point2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Self) -> Vec2<T> {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// `Point + Vec = Point`: translation of a location by a displacement
/// (the torsor action; the vector goes on the right).
impl<T: Real> Add<Vec2<T>> for Point2<T> {
    type Output = Self;

    fn add(self, rhs: Vec2<T>) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

/// `Point − Vec = Point`: translation by the negated displacement,
/// componentwise subtraction (not `self + (−rhs)`; same value, one
/// rounding either way, but the direct form is the documented one).
impl<T: Real> Sub<Vec2<T>> for Point2<T> {
    type Output = Self;

    fn sub(self, rhs: Vec2<T>) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

/// `Point − Point = Vec`: the displacement from `rhs` to `self` — the
/// defining map of the torsor structure.
impl<T: Real> Sub for Point3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: Self) -> Vec3<T> {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// `Point + Vec = Point`: translation of a location by a displacement
/// (the torsor action; the vector goes on the right).
impl<T: Real> Add<Vec3<T>> for Point3<T> {
    type Output = Self;

    fn add(self, rhs: Vec3<T>) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

/// `Point − Vec = Point`: translation by the negated displacement,
/// componentwise subtraction (not `self + (−rhs)`; same value, one
/// rounding either way, but the direct form is the documented one).
impl<T: Real> Sub<Vec3<T>> for Point3<T> {
    type Output = Self;

    fn sub(self, rhs: Vec3<T>) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// Coordinate strategy — see `vec.rs::tests::coord` for the range
    /// rationale (session-box-like magnitudes, no −0.0).
    fn coord() -> impl Strategy<Value = f64> {
        (1.0e-3..1.0e3f64, any::<bool>()).prop_map(|(m, neg)| if neg { -m } else { m })
    }

    fn point3() -> impl Strategy<Value = Point3<f64>> {
        (coord(), coord(), coord()).prop_map(|(x, y, z)| Point3::new(x, y, z))
    }

    /// A generic consumer written against `Real` alone: the circumradius
    /// of a triangle, R = (|AB|·|BC|·|CA|) / (2·|AB × BC|) — in 2-D the
    /// cross magnitude is |perp_dot|. Exercises Point−Point, dot/norm,
    /// perp_dot, abs, and scalar arithmetic through the trait, proving
    /// the module is usable with no bound beyond `Real`.
    fn circumradius<T: Real>(a: Point2<T>, b: Point2<T>, c: Point2<T>) -> T {
        let ab = b - a;
        let bc = c - b;
        let ca = a - c;
        let twice_area = ab.perp_dot(bc).abs();
        (ab.norm() * bc.norm() * ca.norm()) / (T::from_f64(2.0) * twice_area)
    }

    #[test]
    fn generic_consumer_circumradius_at_f64() {
        // Right triangle with legs 3 and 4: hypotenuse 5, circumradius
        // 5/2 (the hypotenuse is a diameter). All arithmetic on these
        // inputs is exact: integer coordinates, exact squares and sums
        // (9+16=25), exact sqrt(25), product 60, division 60/24.
        let r = circumradius(
            Point2::new(0.0, 0.0),
            Point2::new(3.0, 0.0),
            Point2::new(3.0, 4.0),
        );
        assert_eq!(r, 2.5);
    }

    #[test]
    fn point_vec_roundtrip_bit_exact_on_dyadic_samples() {
        // (p + v) − v == p is NOT a floating-point identity in general
        // (p + v rounds). It IS exact when every intermediate is exactly
        // representable; dyadic coordinates with few significand bits
        // and like exponents guarantee that, so the round trip must be
        // bit-exact on these samples.
        let p = Point3::new(1.5, -2.25, 1024.0);
        let v = Vec3::new(0.25, 4.5, -512.0);
        let q = (p + v) - v;
        assert_eq!(q.x.to_bits(), p.x.to_bits());
        assert_eq!(q.y.to_bits(), p.y.to_bits());
        assert_eq!(q.z.to_bits(), p.z.to_bits());
        let r = (p - v) + v;
        assert_eq!(r.x.to_bits(), p.x.to_bits());
        assert_eq!(r.y.to_bits(), p.y.to_bits());
        assert_eq!(r.z.to_bits(), p.z.to_bits());
    }

    #[test]
    fn distance_three_four_five() {
        // Exact arithmetic throughout (integer coordinates, 9+16=25).
        let a = Point2::new(1.0, 2.0);
        let b = Point2::new(4.0, 6.0);
        assert_eq!(a.distance_squared(b), 25.0);
        assert_eq!(a.distance(b), 5.0);
        assert_eq!(b.distance(a), 5.0);
    }

    #[test]
    fn min_max_propagate_nan() {
        let a = Point2::new(f64::NAN, 1.0);
        let b = Point2::new(0.0, 2.0);
        let lo = a.min(b);
        let hi = a.max(b);
        assert!(lo.x.is_nan());
        assert_eq!(lo.y, 1.0);
        assert!(hi.x.is_nan());
        assert_eq!(hi.y, 2.0);
    }

    proptest! {
        /// lerp at t = 0 reproduces `self` bit-exactly: (other − self)·0
        /// is a signed zero in each component, and x + (±0.0) == x
        /// bitwise for every x except x == −0.0 (excluded by generation).
        #[test]
        fn lerp_at_zero_is_bit_exact(a in point3(), b in point3()) {
            let l = a.lerp(b, 0.0);
            prop_assert_eq!(l.x.to_bits(), a.x.to_bits());
            prop_assert_eq!(l.y.to_bits(), a.y.to_bits());
            prop_assert_eq!(l.z.to_bits(), a.z.to_bits());
        }

        /// lerp at t = 1 reproduces `other` to within the difference-and-
        /// add round trip: fl(b − a) errs ≤ u·|b − a| ≤ u·2e3 ≈ 2.2e-13
        /// (u = EPSILON/2, coordinates ≤ 1e3), the ·1.0 is exact, and the
        /// final add rounds once more at ≤ u·1e3 ≈ 1.1e-13 — a ~3.3e-13
        /// budget per coordinate. Asserted at 5e-12 for slack. This is
        /// the documented "near-exact at t = 1" endpoint behavior.
        #[test]
        fn lerp_at_one_is_near_exact(a in point3(), b in point3()) {
            let l = a.lerp(b, 1.0);
            prop_assert!((l.x - b.x).abs() <= 5e-12);
            prop_assert!((l.y - b.y).abs() <= 5e-12);
            prop_assert!((l.z - b.z).abs() <= 5e-12);
        }

        /// The torsor axiom (a + (b − a)) = b, in floating point only up
        /// to one rounding each in difference and add — the same budget
        /// as `lerp_at_one_is_near_exact` (lerp at t = 1 *is* this
        /// expression with an exact ·1.0 in the middle).
        #[test]
        fn difference_then_translate_recovers_target(a in point3(), b in point3()) {
            let r = a + (b - a);
            prop_assert!((r.x - b.x).abs() <= 5e-12);
            prop_assert!((r.y - b.y).abs() <= 5e-12);
            prop_assert!((r.z - b.z).abs() <= 5e-12);
        }

        /// min/max corners bound both points componentwise, and agree
        /// with the scalar lattice (selection: each corner coordinate is
        /// one of the inputs' coordinates).
        #[test]
        fn min_max_are_bbox_corners(a in point3(), b in point3()) {
            let lo = a.min(b);
            let hi = a.max(b);
            prop_assert!(lo.x <= a.x && lo.x <= b.x && (lo.x == a.x || lo.x == b.x));
            prop_assert!(lo.y <= a.y && lo.y <= b.y && (lo.y == a.y || lo.y == b.y));
            prop_assert!(lo.z <= a.z && lo.z <= b.z && (lo.z == a.z || lo.z == b.z));
            prop_assert!(hi.x >= a.x && hi.x >= b.x && (hi.x == a.x || hi.x == b.x));
            prop_assert!(hi.y >= a.y && hi.y >= b.y && (hi.y == a.y || hi.y == b.y));
            prop_assert!(hi.z >= a.z && hi.z >= b.z && (hi.z == a.z || hi.z == b.z));
        }

        /// distance is symmetric as a value: (b − a) and (a − b) are
        /// exact negations componentwise (IEEE: fl(x − y) = −fl(y − x)
        /// as values), squares kill the signs, and the summation order
        /// is unchanged — so even the bits agree here; asserted as
        /// values for robustness.
        #[test]
        fn distance_symmetric(a in point3(), b in point3()) {
            prop_assert_eq!(a.distance(b), b.distance(a));
            prop_assert_eq!(a.distance_squared(b), b.distance_squared(a));
        }
    }
}

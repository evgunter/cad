//! Vectors of the 2-D/3-D linear (tangent) spaces.
//!
//! See the [module docs](super) for the affine/linear split: these are the
//! tangent-side types — displacements, directions, normals — the things
//! that add, negate, and scale. Locations are [`super::Point2`] /
//! [`super::Point3`].

use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::real::Real;

/// A vector of the 2-D linear (tangent) space.
#[derive(Clone, Copy, Debug)]
pub struct Vec2<T: Real> {
    /// The x component.
    pub x: T,
    /// The y component.
    pub y: T,
}

/// A vector of the 3-D linear (tangent) space.
#[derive(Clone, Copy, Debug)]
pub struct Vec3<T: Real> {
    /// The x component.
    pub x: T,
    /// The y component.
    pub y: T,
    /// The z component.
    pub z: T,
}

impl<T: Real> Vec2<T> {
    /// Builds a vector from its components.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// The zero vector (the additive identity).
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }

    /// The first standard basis vector, (1, 0).
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero())
    }

    /// The second standard basis vector, (0, 1).
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one())
    }

    /// The dot product, evaluated exactly as `(x·x′) + (y·y′)` — one
    /// product per component, one addition; the association is fixed (D9).
    /// Because IEEE multiplication is commutative and the summation order
    /// is unchanged under swapping the arguments, `a.dot(b)` and `b.dot(a)`
    /// are bit-identical.
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    /// The perp-dot product `x·y′ − y·x′` (evaluated exactly in that fixed
    /// order) — the scalar 2-D analogue of the cross product: the signed
    /// area of the parallelogram spanned by `self` and `rhs`, positive when
    /// `rhs` lies counterclockwise of `self`. Equivalently: the dot of
    /// `self`'s +90° rotation (−y, x) with `rhs`.
    pub fn perp_dot(self, rhs: Self) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    /// The squared Euclidean norm, `self.dot(self)` (same fixed
    /// association as [`Vec2::dot`]).
    pub fn norm_squared(self) -> T {
        self.dot(self)
    }

    /// The Euclidean norm, exactly `self.norm_squared().sqrt()` — no fused
    /// hypot (see `real.rs` on why fused conveniences are excluded).
    pub fn norm(self) -> T {
        self.norm_squared().sqrt()
    }

    /// The unit vector in this direction, exactly `self / self.norm()`
    /// (one division per component).
    ///
    /// **Total.** The zero vector yields all-NaN components (0/0), and a
    /// poisoned input propagates poison — per the crate's totality policy
    /// (`real.rs`): poison flows through values, and the predicate layer /
    /// residual certification is where it is caught. Components beyond
    /// ~1e154 overflow `norm_squared` to ∞ and collapse the result toward
    /// zero; symmetrically, components below ~1e-162 underflow
    /// `norm_squared` to 0 and blow the result up to ±∞ (not NaN). Both
    /// ends are far outside the session box (D4 ¶4), same posture as
    /// `powi`'s extreme-exponent note.
    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    /// The componentwise minimum (bounding-box support). Inherits
    /// [`Real::min`]'s lattice-not-control-flow contract: ties keep
    /// `self`'s component, and **NaN propagates** per component — a
    /// poisoned coordinate poisons the bound rather than being silently
    /// dropped.
    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y))
    }

    /// The componentwise maximum. Same contract as [`Vec2::min`]: ties
    /// keep `self`'s component, NaN propagates per component.
    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y))
    }
}

impl<T: Real> Vec3<T> {
    /// Builds a vector from its components.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// The zero vector (the additive identity).
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero(), T::zero())
    }

    /// The first standard basis vector, (1, 0, 0).
    pub fn unit_x() -> Self {
        Self::new(T::one(), T::zero(), T::zero())
    }

    /// The second standard basis vector, (0, 1, 0).
    pub fn unit_y() -> Self {
        Self::new(T::zero(), T::one(), T::zero())
    }

    /// The third standard basis vector, (0, 0, 1).
    pub fn unit_z() -> Self {
        Self::new(T::zero(), T::zero(), T::one())
    }

    /// The dot product, evaluated exactly as `((x·x′) + (y·y′)) + (z·z′)`
    /// — the association is fixed (D9). Because IEEE multiplication is
    /// commutative and swapping the arguments permutes nothing in that
    /// sum, `a.dot(b)` and `b.dot(a)` are bit-identical.
    pub fn dot(self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// The cross product, each component evaluated exactly as the fixed
    /// two-product difference
    /// `(y·z′ − z·y′, z·x′ − x·z′, x·y′ − y·x′)`.
    ///
    /// Antisymmetry holds up to the sign of zero: IEEE negation of a
    /// rounded difference is exact (`fl(u − w) = −fl(w − u)` as values),
    /// so `a.cross(b)` and `−b.cross(a)` are value-equal componentwise,
    /// differing at most in zero signs where products cancel exactly.
    pub fn cross(self, rhs: Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// The squared Euclidean norm, `self.dot(self)` (same fixed
    /// association as [`Vec3::dot`]).
    pub fn norm_squared(self) -> T {
        self.dot(self)
    }

    /// The Euclidean norm, exactly `self.norm_squared().sqrt()` — no fused
    /// hypot (see `real.rs` on why fused conveniences are excluded).
    pub fn norm(self) -> T {
        self.norm_squared().sqrt()
    }

    /// The unit vector in this direction, exactly `self / self.norm()`
    /// (one division per component).
    ///
    /// **Total.** The zero vector yields all-NaN components (0/0), and a
    /// poisoned input propagates poison — per the crate's totality policy
    /// (`real.rs`). Components beyond ~1e154 overflow `norm_squared` to ∞
    /// and collapse the result toward zero; symmetrically, components
    /// below ~1e-162 underflow `norm_squared` to 0 and blow the result up
    /// to ±∞ (not NaN). Both ends are far outside the session box
    /// (D4 ¶4), same posture as `powi`'s extreme-exponent note.
    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    /// The componentwise minimum (bounding-box support). Inherits
    /// [`Real::min`]'s lattice-not-control-flow contract: ties keep
    /// `self`'s component, and **NaN propagates** per component — a
    /// poisoned coordinate poisons the bound rather than being silently
    /// dropped.
    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.x.min(rhs.x), self.y.min(rhs.y), self.z.min(rhs.z))
    }

    /// The componentwise maximum. Same contract as [`Vec3::min`]: ties
    /// keep `self`'s component, NaN propagates per component.
    pub fn max(self, rhs: Self) -> Self {
        Self::new(self.x.max(rhs.x), self.y.max(rhs.y), self.z.max(rhs.z))
    }
}

impl<T: Real> Add for Vec2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T: Real> Sub for Vec2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T: Real> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

/// Right scalar multiplication `v * s` (left multiplication `s * v` is
/// deliberately absent — see the [module docs](super)).
impl<T: Real> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

/// Componentwise scalar division `v / s` — one division per component,
/// not a reciprocal-then-multiply (one rounding per component, and the
/// natural reading at every planned scalar type). Total: `s` zero or
/// poisoned yields poison components.
impl<T: Real> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T: Real> Add for Vec3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Real> Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Real> Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

/// Right scalar multiplication `v * s` (left multiplication `s * v` is
/// deliberately absent — see the [module docs](super)).
impl<T: Real> Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

/// Componentwise scalar division `v / s` — one division per component,
/// not a reciprocal-then-multiply (one rounding per component, and the
/// natural reading at every planned scalar type). Total: `s` zero or
/// poisoned yields poison components.
impl<T: Real> Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    /// A coordinate with magnitude in [1e-3, 1e3] and independent sign.
    ///
    /// The range is session-box-like (D4 ¶4): it caps every product and
    /// sum within ~9 orders of magnitude of 1, so error bounds of the
    /// form `C · f64::EPSILON · magnitude` stay tight and no overflow or
    /// subnormal behavior muddies the analysis. The sign is a separate
    /// flag so `-0.0` can never be generated — bit-exactness assertions
    /// below rely on that.
    fn coord() -> impl Strategy<Value = f64> {
        (1.0e-3..1.0e3f64, any::<bool>()).prop_map(|(m, neg)| if neg { -m } else { m })
    }

    fn vec2() -> impl Strategy<Value = Vec2<f64>> {
        (coord(), coord()).prop_map(|(x, y)| Vec2::new(x, y))
    }

    fn vec3() -> impl Strategy<Value = Vec3<f64>> {
        (coord(), coord(), coord()).prop_map(|(x, y, z)| Vec3::new(x, y, z))
    }

    fn max_abs3(v: Vec3<f64>) -> f64 {
        v.x.abs().max(v.y.abs()).max(v.z.abs())
    }

    #[test]
    fn basis_vectors_are_orthonormal_exactly() {
        // Products of exact 0s and 1s and their two-term sums are exact.
        let (ex, ey, ez) = (
            Vec3::<f64>::unit_x(),
            Vec3::<f64>::unit_y(),
            Vec3::<f64>::unit_z(),
        );
        assert_eq!(ex.dot(ex), 1.0);
        assert_eq!(ey.dot(ey), 1.0);
        assert_eq!(ez.dot(ez), 1.0);
        assert_eq!(ex.dot(ey), 0.0);
        assert_eq!(ey.dot(ez), 0.0);
        // The right-handed frame: x × y = z exactly (products of 0/1).
        let c = ex.cross(ey);
        assert_eq!((c.x, c.y, c.z), (0.0, 0.0, 1.0));
        let p = Vec2::<f64>::unit_x().perp_dot(Vec2::unit_y());
        assert_eq!(p, 1.0);
    }

    #[test]
    fn zero_is_additive_identity_bit_exact() {
        // v + 0: each component is x + (+0.0), which is bit-exact for
        // every x except x == -0.0; the sample avoids -0.0.
        let v = Vec3::new(1.5, -2.25, 3.0e3);
        let w = v + Vec3::zero();
        assert_eq!(w.x.to_bits(), v.x.to_bits());
        assert_eq!(w.y.to_bits(), v.y.to_bits());
        assert_eq!(w.z.to_bits(), v.z.to_bits());
    }

    #[test]
    fn normalize_zero_vector_is_all_nan() {
        // 0 / sqrt(0) = 0/0 = NaN in every component: the documented
        // totality outcome — poison, not a panic and not a silent zero.
        let n = Vec3::<f64>::zero().normalize();
        assert!(n.x.is_nan() && n.y.is_nan() && n.z.is_nan());
        let n2 = Vec2::<f64>::zero().normalize();
        assert!(n2.x.is_nan() && n2.y.is_nan());
    }

    #[test]
    fn min_max_propagate_nan() {
        // One poisoned coordinate poisons exactly that component of the
        // bound (Real::min/max NaN propagation, not IEEE minNum).
        let v = Vec3::new(f64::NAN, 1.0, 2.0);
        let w = Vec3::new(0.0, f64::NAN, 5.0);
        let lo = v.min(w);
        let hi = v.max(w);
        assert!(lo.x.is_nan() && lo.y.is_nan());
        assert_eq!(lo.z, 2.0);
        assert!(hi.x.is_nan() && hi.y.is_nan());
        assert_eq!(hi.z, 5.0);
    }

    proptest! {
        /// Dot symmetry is bit-exact: each componentwise product commutes
        /// exactly (IEEE multiplication is commutative), and swapping the
        /// arguments does not permute the fixed summation order
        /// ((x·x′ + y·y′) + z·z′ keeps its shape under the swap), so both
        /// sides round identically at every step.
        #[test]
        fn dot_symmetry_bit_exact(a in vec3(), b in vec3()) {
            prop_assert_eq!(a.dot(b).to_bits(), b.dot(a).to_bits());
        }

        /// Same argument in 2-D.
        #[test]
        fn dot2_symmetry_bit_exact(a in vec2(), b in vec2()) {
            prop_assert_eq!(a.dot(b).to_bits(), b.dot(a).to_bits());
        }

        /// Cross antisymmetry as *values* (==), not bits: each component
        /// of `b × a` is fl(u) − fl(w) where `a × b`'s is fl(w) − fl(u),
        /// and IEEE subtraction satisfies fl(u − w) = −fl(w − u) exactly —
        /// except that an exactly cancelling difference gives +0 on both
        /// sides, whose negation is −0. Hence value equality, not
        /// bit equality, is the honest assertion.
        #[test]
        fn cross_antisymmetry(a in vec3(), b in vec3()) {
            let ab = a.cross(b);
            let ba = -b.cross(a);
            prop_assert_eq!(ab.x, ba.x);
            prop_assert_eq!(ab.y, ba.y);
            prop_assert_eq!(ab.z, ba.z);
        }

        /// perp_dot is antisymmetric as values, same argument as
        /// `cross_antisymmetry` (one fixed two-product difference).
        #[test]
        fn perp_dot_antisymmetry(a in vec2(), b in vec2()) {
            prop_assert_eq!(a.perp_dot(b), -b.perp_dot(a));
        }

        /// (a × b) · a = 0 exactly in real arithmetic. Error budget with
        /// m = max |component|: each cross component is ≤ 2m² with
        /// absolute error ≤ ~2·(u·m²) from the two products plus
        /// ≤ u·2m² from the subtraction (u = half an ulp = EPSILON/2),
        /// so ≤ 2·EPSILON·m². The dot contributes |a|·(those errors)
        /// ≤ 3·(2·EPSILON·m²)·m plus its own product/sum roundings on
        /// terms ≤ 2m³, another ≤ ~3·EPSILON·m³ — call it ≤ 12·EPSILON·m³
        /// total. Asserted at 64·EPSILON·m³ for constant-factor slack.
        #[test]
        fn cross_is_orthogonal_to_operands(a in vec3(), b in vec3()) {
            let m = max_abs3(a).max(max_abs3(b));
            let bound = 64.0 * f64::EPSILON * m * m * m;
            prop_assert!(a.cross(b).dot(a).abs() <= bound);
            prop_assert!(a.cross(b).dot(b).abs() <= bound);
        }

        /// norm_squared is a sum of squares: every term is ≥ 0 (or NaN,
        /// excluded by generation), and adding nonnegatives rounds to a
        /// nonnegative.
        #[test]
        fn norm_squared_nonnegative(a in vec3()) {
            prop_assert!(a.norm_squared() >= 0.0);
        }

        /// |normalize(v)| = 1 to a few ulps: norm_squared carries ≤ ~2
        /// roundings, sqrt ≤ half an ulp, each component division ≤ half
        /// an ulp, and the outer norm the same again — a relative error
        /// budget of ≈ 5·EPSILON ≈ 1.1e-15. Asserted at 1e-14.
        #[test]
        fn normalize_produces_unit_norm(a in vec3()) {
            prop_assert!((a.normalize().norm() - 1.0).abs() <= 1e-14);
        }

        /// Scaling and adding are componentwise and exact per IEEE op:
        /// (v * s) / s recovers v within 1 ulp per component (two
        /// correctly rounded operations, no cancellation). Bounded, not
        /// bit-exact: v·s rounds.
        #[test]
        fn mul_div_roundtrip(v in vec3(), s in 1.0e-3..1.0e3f64) {
            let r = (v * s) / s;
            prop_assert!((r.x - v.x).abs() <= 2.0 * f64::EPSILON * v.x.abs());
            prop_assert!((r.y - v.y).abs() <= 2.0 * f64::EPSILON * v.y.abs());
            prop_assert!((r.z - v.z).abs() <= 2.0 * f64::EPSILON * v.z.abs());
        }
    }
}

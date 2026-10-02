//! **Running error bounds** — a value carried with a first-order bound
//! on its accumulated rounding, for the meters that charge an
//! evaluation chain's own error rather than a fixed count of ulps of a
//! term bound.
//!
//! Each correctly rounded operation adds `u·|result|`, `u` the unit
//! roundoff ([`UNIT_ROUNDOFF`]), and carries its operands' bounds
//! through its partial derivatives; second-order terms (`u²` of the
//! same magnitudes) are dropped. A product of exact zeros, or a
//! subtraction that lands on an exact zero, therefore charges nothing,
//! which is what lets an axis-aligned chain report the zero error it
//! has. Each operation evaluates its value as the plain expression
//! does, so a shadowed value is bit-identical to the plain one.
//!
//! It is a rounding ESTIMATE in the `f64` lane's sense, not an
//! enclosure: at `Interval` the value is already an enclosure of the
//! chain and the bound rides along as a second one.

use core::ops::{Add, Mul, Sub};

use crate::{Real, Vec3};

/// The unit roundoff of `f64`, `u = 2⁻⁵³`: half an ulp of `1`, the
/// relative error bound of one correctly rounded operation.
pub const UNIT_ROUNDOFF: f64 = f64::EPSILON * 0.5;

/// A value and a first-order bound on its accumulated rounding (module
/// docs).
#[derive(Debug, Clone, Copy)]
pub struct Rounded<T> {
    /// The value, bit-identical to the plain expression's.
    pub value: T,
    /// A first-order bound on `|value − exact|`, `exact` the same
    /// expression in real arithmetic on the same inputs.
    pub error: T,
}

impl<T: Real> Rounded<T> {
    /// An input taken as exact.
    pub fn exact(value: T) -> Self {
        Self {
            value,
            error: T::zero(),
        }
    }

    fn charged(value: T, error: T) -> Self {
        Self {
            value,
            error: error + T::from_f64(UNIT_ROUNDOFF) * value.abs(),
        }
    }

    /// `self / d` for an EXACT divisor `d`.
    #[must_use]
    pub fn div_exact(self, d: T) -> Self {
        Self::charged(self.value / d, self.error / d.abs())
    }

    /// `self²`, as `self.value.powi(2)` (the tight square).
    #[must_use]
    pub fn square(self) -> Self {
        Self::charged(
            self.value.powi(2),
            T::from_f64(2.0) * self.value.abs() * self.error,
        )
    }

    /// `√(a² + b²)`, as `(a.powi(2) + b.powi(2)).sqrt()`. The norm is
    /// 1-Lipschitz in each argument, and its own evaluation (two
    /// squares, a sum of non-negatives, a square root) is off by at most
    /// `2u` of the result — a bound with no division, so it holds at
    /// the origin.
    #[must_use]
    pub fn hypot(self, o: Self) -> Self {
        let value = (self.value.powi(2) + o.value.powi(2)).sqrt();
        Self {
            value,
            error: self.error + o.error + T::from_f64(2.0 * UNIT_ROUNDOFF) * value,
        }
    }

    /// `when_le` where `decision ≤ 0`, else `when_gt`: [`Real::select_le_zero`]
    /// on the value and the bound alike.
    #[must_use]
    pub fn select_le_zero(decision: T, when_le: Self, when_gt: Self) -> Self {
        Self {
            value: decision.select_le_zero(when_le.value, when_gt.value),
            error: decision.select_le_zero(when_le.error, when_gt.error),
        }
    }

    /// An upper bound on `|exact|`: the magnitude plus the bound.
    pub fn magnitude(self) -> T {
        self.value.abs() + self.error
    }
}

impl<T: Real> Add for Rounded<T> {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        Self::charged(self.value + o.value, self.error + o.error)
    }
}

impl<T: Real> Sub for Rounded<T> {
    type Output = Self;

    fn sub(self, o: Self) -> Self {
        Self::charged(self.value - o.value, self.error + o.error)
    }
}

impl<T: Real> Mul for Rounded<T> {
    type Output = Self;

    fn mul(self, o: Self) -> Self {
        Self::charged(
            self.value * o.value,
            self.value.abs() * o.error + o.value.abs() * self.error,
        )
    }
}

/// The components of an exact vector.
pub fn exact_vec<T: Real>(v: Vec3<T>) -> [Rounded<T>; 3] {
    [v.x, v.y, v.z].map(Rounded::exact)
}

/// [`Vec3::dot`]'s order, with its running bound.
pub fn dot<T: Real>(a: [Rounded<T>; 3], b: [Rounded<T>; 3]) -> Rounded<T> {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// [`Vec3::cross`]'s order, with its running bound.
pub fn cross<T: Real>(a: [Rounded<T>; 3], b: [Rounded<T>; 3]) -> [Rounded<T>; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// `|v|² − 1` for an EXACT vector `v`, pivoted on its largest component
/// `m` as `(m − 1)(m + 1) + (the other two squares)`: a unit axis
/// vector then evaluates with no charge at all, where `Σvᵢ² − 1` would
/// charge three roundings of `1`.
pub fn unit_defect<T: Real>(v: Vec3<T>) -> Rounded<T> {
    let one = Rounded::exact(T::one());
    let [x, y, z] = exact_vec(v);
    let pivoted = |m: Rounded<T>, a: Rounded<T>, b: Rounded<T>| {
        (m - one) * (m + one) + (a.square() + b.square())
    };
    let (ax, ay, az) = (v.x.abs(), v.y.abs(), v.z.abs());
    let on_xy = Rounded::select_le_zero(ax - ay, pivoted(y, x, z), pivoted(x, y, z));
    Rounded::select_le_zero(ax.max(ay) - az, pivoted(z, x, y), on_xy)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// **An axis-aligned chain reports no error; a rounded one reports
    /// at least its rounding.** The unit defect of `ẑ` is exactly zero
    /// with a zero bound. For `(0.6, 0.8, 0)`, whose squares round, the
    /// bound covers the gap between the `f64` value and the exact
    /// defect `0.6² + 0.8² − 1` of the stored (rounded) components,
    /// evaluated here in exact dyadic arithmetic on the bits.
    #[test]
    fn the_bound_is_zero_on_exact_chains_and_covers_rounded_ones() {
        let z = unit_defect(Vec3::new(0.0, 0.0, 1.0));
        assert_eq!((z.value, z.error), (0.0, 0.0), "ẑ's defect is exact");
        let neg = unit_defect(Vec3::new(0.0, -1.0, 0.0));
        assert_eq!((neg.value, neg.error), (0.0, 0.0), "−ŷ's defect is exact");
        let (a, b) = (0.6_f64, 0.8_f64);
        let d = unit_defect(Vec3::new(a, b, 0.0));
        // Exact: a = ma·2^ea, b likewise; a² + b² − 1 over 2^(2·min e).
        let exact = |x: f64| {
            let bits = x.to_bits();
            let e = i64::try_from((bits >> 52) & 0x7ff).unwrap() - 1075;
            let m = i128::from((bits & ((1 << 52) - 1)) | (1 << 52));
            (m, e)
        };
        let ((ma, ea), (mb, eb)) = (exact(a), exact(b));
        assert_eq!(ea, eb, "the fixture's components share an exponent");
        let scale = -2 * ea; // the squares are m²·2^(2e); 1 is 2^scale·2^(2e)
        let num = ma * ma + mb * mb - (1_i128 << scale);
        let exact_defect = num as f64 * 2f64.powi(i32::try_from(2 * ea).unwrap());
        assert!(
            (d.value - exact_defect).abs() <= d.error && d.error > 0.0,
            "(0.6, 0.8): f64 defect {} ± {} against the exact {exact_defect}",
            d.value,
            d.error
        );
    }
}

//! An unbounded exact rational, the oracle the spline layer's
//! containment rows compare certification arithmetic against. Shared
//! by this binary's suites and `geom_core::spline::compose`'s unit
//! tests (which include this file by path), so one oracle answers both.
#![allow(dead_code)]

use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::ToPrimitive;

/// An exact rational, `num/den` with `den > 0`, normalized. Only the
/// operations the Boehm combination needs — the point is that the
/// SHADOW of the ring fold carries no rounding at all, so a ring
/// bound that fails to contain it is unsound by construction.
///
/// **Why this is not [`crate::sym::Rat`]**, which is this crate's
/// own exact rational over the same `num-bigint`: `Rat` is BOUNDED
/// at `sym::rational`'s `COEFF_BITS` (256) and freezes the form it
/// belongs to when a coefficient would exceed it. That is correct
/// for the symbolic tier, whose budget is the point, and fatal
/// here: the shadow's denominator picks up a factor of `Δ` per
/// insertion, so a fold to full multiplicity over a clustered
/// vector runs past 256 bits well before it ends, and a freeze
/// would silently stop the oracle being exact — the one property it
/// exists for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Q {
    pub(crate) num: BigInt,
    pub(crate) den: BigInt,
}

impl Q {
    pub(crate) fn new(num: BigInt, den: BigInt) -> Self {
        assert!(
            den != BigInt::from(0),
            "exact rational with zero denominator"
        );
        let (num, den) = if den < BigInt::from(0) {
            (-num, -den)
        } else {
            (num, den)
        };
        let g = num.gcd(&den);
        if g == BigInt::from(0) {
            return Self { num, den };
        }
        Self {
            num: num / &g,
            den: den / &g,
        }
    }

    /// Exact: an `f64` IS a dyadic rational, so this loses nothing.
    pub(crate) fn from_f64(x: f64) -> Self {
        assert!(x.is_finite(), "exact rational from {x}");
        if x == 0.0 {
            return Self {
                num: BigInt::from(0),
                den: BigInt::from(1),
            };
        }
        let bits = x.to_bits();
        let sign = if bits >> 63 == 1 { -1i32 } else { 1 };
        let raw_exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = bits & ((1u64 << 52) - 1);
        // Subnormals carry no implicit leading bit and sit one
        // exponent up; the fixture families below reach them.
        let (mant, exp) = if raw_exp == 0 {
            (frac, -1074i32)
        } else {
            (frac | (1u64 << 52), raw_exp - 1075)
        };
        let m = BigInt::from(mant) * BigInt::from(sign);
        if exp >= 0 {
            Self::new(m << (exp as usize), BigInt::from(1))
        } else {
            Self::new(m, BigInt::from(1) << ((-exp) as usize))
        }
    }

    pub(crate) fn add(&self, o: &Self) -> Self {
        Self::new(&self.num * &o.den + &o.num * &self.den, &self.den * &o.den)
    }

    pub(crate) fn sub(&self, o: &Self) -> Self {
        Self::new(&self.num * &o.den - &o.num * &self.den, &self.den * &o.den)
    }

    pub(crate) fn mul(&self, o: &Self) -> Self {
        Self::new(&self.num * &o.num, &self.den * &o.den)
    }

    pub(crate) fn div(&self, o: &Self) -> Self {
        assert!(o.num != BigInt::from(0), "exact division by zero");
        Self::new(&self.num * &o.den, &self.den * &o.num)
    }

    pub(crate) fn cmp_f64(&self, x: f64) -> core::cmp::Ordering {
        let y = Self::from_f64(x);
        (&self.num * &y.den).cmp(&(&y.num * &self.den))
    }

    /// A reported-only `f64` rendering, total over the magnitudes
    /// this shadow reaches. Dividing `num.to_f64()` by
    /// `den.to_f64()` is NOT total: both overflow to `inf` once the
    /// fold's denominator passes 1024 bits, and `inf / inf` is
    /// `NaN` — which silently dropped 45 of the 487 slack samples
    /// from the figure the row reports. Scaling the numerator first
    /// and dividing in `BigInt` keeps the ratio.
    ///
    /// Never read by an assertion about containment: that is
    /// [`Q::cmp_f64`], which is exact.
    pub(crate) fn to_f64_report(&self) -> f64 {
        #[allow(clippy::cast_possible_wrap)]
        let (bn, bd) = (self.num.bits() as i64, self.den.bits() as i64);
        let shift = (64 - (bn - bd)).max(0);
        let q = (&self.num << usize::try_from(shift).unwrap_or(0)) / &self.den;
        let Some(v) = q.to_f64() else {
            return f64::NAN;
        };
        v * 2f64.powi(i32::try_from(-shift).unwrap_or(i32::MIN))
    }
}

/// One coefficient's TRUE value set: an exact rational interval.
/// The Boehm combination is affine with both barycentric
/// coefficients in `(0, 1)`, so it is monotone in each source and
/// the extremes of the true set are attained at the sources' own
/// endpoints — which is why two rationals describe it exactly.
#[derive(Clone, Debug)]
pub(crate) struct QInt {
    pub(crate) lo: Q,
    pub(crate) hi: Q,
}

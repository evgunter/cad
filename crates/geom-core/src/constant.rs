//! **A constant of the model enters by where it comes from** (the crate
//! README, DL5.2, the table's last row). Two of its doors live here: the
//! one a number written in the source enters by ([`Lit`], [`lit!`](crate::lit)) and
//! the one a count enters by ([`Count`]). A band enters as
//! [`BandWidth`](crate::BandWidth), on the tolerance type, and an
//! authored value through the document's parameter door.
//!
//! None of them takes a bare `f64`, so a value the kernel computed
//! cannot be handed to one: a [`Lit`] is built only from a
//! [`Constant`]'s associated `const`, and a [`Count`] only from an
//! integer.
//!
//! What each is at every scalar ([`Real::lit`], [`Real::count`],
//! [`Real::count_ratio`]):
//!
//! - `f64`: the number;
//! - an enclosure scalar: its point;
//! - [`Sym`](crate::Sym): a literal, the claim that it is the same
//!   number at every valuation of the document's unknowns. A number in
//!   the source is. A count is while the structure it counts is the one
//!   the valuation was taken on, which is what a symbolic identity is
//!   stated over;
//! - a dual: a zero tangent, the derivative of a number that no
//!   parameter moves.
//!
//! A runtime `f64` has no way in:
//!
//! ```compile_fail,E0435
//! fn computed(x: f64) -> geom_core::Lit {
//!     geom_core::lit!(x)
//! }
//! ```
//!
//! ```compile_fail,E0015
//! fn read() -> f64 {
//!     0.5
//! }
//! let _ = geom_core::lit!(read());
//! ```
//!
//! ```compile_fail,E0423
//! let _ = geom_core::Lit(0.5);
//! ```
//!
//! ```compile_fail,E0277
//! use geom_core::Real;
//! let _ = f64::count(0.5_f64);
//! ```
//!
//! and beside each, what does enter:
//!
//! ```
//! use geom_core::{Real, lit};
//! const QUARTER: f64 = 0.25;
//! const fn twice(x: f64) -> f64 {
//!     2.0 * x
//! }
//! assert_eq!(f64::lit(lit!(twice(QUARTER))), 0.5);
//! assert_eq!(f64::count(3_usize), 3.0);
//! assert_eq!(f64::count_ratio(1_u32, 4_u32), 0.25);
//! ```
//!
//! What the type does not hold, so the reader does: a `const fn` of any
//! shape is a constant; an immutable `static` reads as one; a marker
//! generic over a `const` parameter is a constant per instantiation, so
//! a runtime integer can pick among finitely many of them, as it can
//! index a table of [`Lit`]s. Each is a selection among numbers the
//! source wrote, never a number the run computed.

use crate::Real;

/// A marker whose number is fixed when the crate is compiled: the one
/// thing a [`Lit`] is built from. [`lit!`](crate::lit) writes the marker.
pub trait Constant {
    /// The number.
    const VALUE: f64;
}

/// **A number the source wrote**: a literal, a `const`, a `const fn` of
/// them. It has no constructor that takes a value, so a run cannot make
/// one of a number it computed; it is `Copy` and can sit in a `const`
/// table, which a run may index.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Lit(f64);

impl Lit {
    /// The constant `K` names.
    #[must_use]
    pub const fn of<K: Constant>() -> Self {
        Self(K::VALUE)
    }

    /// The number, for the `f64` structure that reads the same table
    /// (a comparison, a schedule's index).
    #[must_use]
    pub const fn get(self) -> f64 {
        self.0
    }

    /// The product of two constants, rounded as `f64` rounds it.
    #[must_use]
    pub const fn times(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }

    /// The negation, exact.
    #[must_use]
    pub const fn neg(self) -> Self {
        Self(-self.0)
    }
}

/// A [`Lit`] of a constant expression: the expression is bound to an
/// associated `const`, so one that reads a runtime value does not
/// compile. It cannot name a generic parameter of the function it
/// stands in (an item sees none); a constant that depends on one is a
/// [`Constant`] written by hand.
#[macro_export]
macro_rules! lit {
    ($value:expr) => {{
        struct K;
        impl $crate::Constant for K {
            const VALUE: f64 = $value;
        }
        $crate::Lit::of::<K>()
    }};
}

mod sealed {
    pub trait Integer: Copy {
        /// The integer as an `f64`.
        fn to_f64(self) -> f64;
    }
}

/// **An integer that counts**: a number of pieces, an index, a degree,
/// a winding. Implemented for the integer types and nothing else, so
/// what enters through [`Real::count`] was an integer when it entered.
///
/// Exact for every magnitude up to 2⁵³, which the narrow types cannot
/// pass and no count of things held in memory reaches; a wider integer
/// past it enters as the nearest `f64`.
pub trait Count: sealed::Integer {}

macro_rules! count_lossless {
    ($($int:ty),*) => {$(
        impl sealed::Integer for $int {
            fn to_f64(self) -> f64 {
                f64::from(self)
            }
        }
        impl Count for $int {}
    )*};
}
macro_rules! count_wide {
    ($($int:ty),*) => {$(
        impl sealed::Integer for $int {
            #[allow(clippy::cast_precision_loss)]
            fn to_f64(self) -> f64 {
                self as f64
            }
        }
        impl Count for $int {}
    )*};
}
count_lossless!(u8, u16, u32, i8, i16, i32);
count_wide!(u64, i64, u128, i128, usize, isize);

/// [`Real::lit`]'s body, the one step from a [`Lit`] to a scalar.
pub(crate) fn lit<T: Real>(c: Lit) -> T {
    T::from_f64(c.0)
}

/// [`Real::count`]'s body, the one step from a count to a scalar.
pub(crate) fn count<T: Real, N: Count>(n: N) -> T {
    T::from_f64(n.to_f64())
}

/// [`Real::count_ratio`]'s body.
pub(crate) fn count_ratio<T: Real, N: Count, D: Count>(num: N, den: D) -> T {
    T::from_f64(num.to_f64() / den.to_f64())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Dual64, Interval, Sym};

    const SHARES: [Lit; 2] = [lit!(0.25), lit!(3.0 / 4.0)];

    /// Whether a `Sym` value is one literal node of this number, which
    /// is what each door claims there.
    fn is_literal(x: Sym<f64>, value: f64) -> bool {
        x.node() == <Sym<f64> as Real>::from_f64(value).node()
    }

    #[test]
    fn a_constant_is_the_number_at_f64_and_its_point_at_an_interval() {
        assert_eq!(f64::lit(lit!(0.1)).to_bits(), 0.1_f64.to_bits());
        let i = Interval::lit(lit!(0.1));
        assert_eq!((crate::Bounds::lo(i), crate::Bounds::hi(i)), (0.1, 0.1));
        assert_eq!(f64::lit(SHARES[1]), 0.75);
        assert_eq!(
            lit!(3.0).times(lit!(f64::EPSILON)).get(),
            3.0 * f64::EPSILON
        );
        assert_eq!(lit!(1.0).neg().get(), -1.0);
    }

    #[test]
    fn a_constant_a_count_and_a_count_ratio_are_literals_at_sym() {
        assert!(is_literal(Sym::<f64>::lit(lit!(0.5)), 0.5));
        assert!(is_literal(Sym::<f64>::count(7_usize), 7.0));
        assert!(is_literal(Sym::<f64>::count(-3_i32), -3.0));
        assert!(is_literal(Sym::<f64>::count_ratio(1_u32, 3_u32), 1.0 / 3.0));
        // An opaque unknown of the same value is a different node: what
        // the doors would be if they entered through `from_computed`.
        assert!(!is_literal(Sym::<f64>::from_computed(0.5), 0.5));
    }

    #[test]
    fn a_constant_and_a_count_carry_no_tangent() {
        let parts = |d: Dual64| (d.value, d.deriv);
        assert_eq!(parts(<Dual64 as Real>::lit(lit!(0.5))), (0.5, 0.0));
        assert_eq!(parts(Dual64::count(4_u32)), (4.0, 0.0));
        assert_eq!(parts(Dual64::count_ratio(1_u32, 4_u32)), (0.25, 0.0));
    }

    #[test]
    fn a_count_is_the_integer_exactly_up_to_two_to_the_53() {
        assert_eq!(f64::count(u32::MAX), 4_294_967_295.0);
        assert_eq!(f64::count(i32::MIN), -2_147_483_648.0);
        assert_eq!(f64::count(1_u64 << 53), 9_007_199_254_740_992.0);
        assert_eq!(f64::count(0_usize).to_bits(), 0.0_f64.to_bits());
        let i = Interval::count(3_u8);
        assert_eq!((crate::Bounds::lo(i), crate::Bounds::hi(i)), (3.0, 3.0));
    }

    #[test]
    fn a_count_ratio_is_the_f64_quotient_as_a_point() {
        let q = Interval::count_ratio(1_usize, 3_usize);
        let third = 1.0_f64 / 3.0;
        assert_eq!((crate::Bounds::lo(q), crate::Bounds::hi(q)), (third, third));
        assert_eq!(f64::count_ratio(2_u32, 4_u32), 0.5);
    }
}

//! The **`Real`-level bit-identity door** — the ONE sanctioned seam
//! through which generic code may ask "are these two scalars the *same
//! description*, bit for bit?" (M3 PR 4; deferred here by the PR 1
//! review, which rejected the `Debug`-string channel with a NaN-payload
//! exploit and localized the interim downcast ladder in
//! `topo::merge_faces`).
//!
//! # What this is, and is not
//!
//! This is an **identity channel, not a comparison door**: no
//! coincidence between two cells consults it (see the retirement note
//! below) — a coincidence is its margin's to decide (D10). What remains
//! is exact-representation plumbing, and one decision DESIGN
//! `[extensibility]` makes on stored values: a spline net's collapsed
//! side, a boundary iso whose control points are all one stored point
//! ([`one_stored_point`]), which is a chart singularity there. It never
//! orders values, never bands, and is deliberately NOT part of the
//! [`Real`](crate::Real) trait surface — evaluation code stays
//! comparison-free.
//!
//! # Fencing (Ev, #53/#57/#58)
//!
//! - The type-punning plumbing (`Any` downcast + raw bit extraction)
//!   lives in THIS file only; the CI "bit-identity channel tripwire"
//!   step fails if the punning idioms appear anywhere else in crate
//!   sources (allowlist: this file, plus `interval.rs` which defines
//!   `Interval::repr_bits` over its own storage).
//! - **Retired from production coincidence (NAMING-DESIGN N6)**: no
//!   production path checks a coincidence between two cells through
//!   this channel. The CI tripwire (`scripts/gates/bit-identity-consumer.sh`)
//!   stays armed: every file that reaches the channel is listed there
//!   with what it reads it for, scalar plumbing or the collapsed-side
//!   reading and the knot algebra that keeps it.
//!
//! A scalar type without an arm below (e.g. `Dual`, which no `Body`
//! instantiates) yields `None`: no bit channel, so a caller takes the
//! conservative branch (never-equal).

/// The bit-faithful identity of one scalar: `f64` ⇒ `(to_bits, 0, 0)`;
/// `Probe` ⇒ its inner `f64`'s bits; the interval scalar ⇒ its
/// `(inf_bits, sup_bits, decoration)` triple (`Interval::repr_bits`,
/// which — unlike `Bounds` — keeps NaI and empty apart). NaN payloads
/// are distinguished (the PR 1 exploit is unrepresentable here);
/// bit-identical NaNs still compare equal.
pub type ScalarBits = (u64, u64, u64);

/// The identity of `x`, if its concrete type has a bit channel.
pub fn repr_bits<T: crate::Real>(x: &T) -> Option<ScalarBits> {
    let any: &dyn core::any::Any = x;
    if let Some(v) = any.downcast_ref::<f64>() {
        return Some((v.to_bits(), 0, 0));
    }
    #[cfg(feature = "probe")]
    if let Some(v) = any.downcast_ref::<crate::Probe>() {
        return Some((v.0.to_bits(), 0, 0));
    }
    if let Some(v) = any.downcast_ref::<crate::Interval>() {
        let (lo, hi, dec) = v.repr_bits();
        return Some((lo, hi, u64::from(dec)));
    }
    None
}

/// Bit-identity of two same-type scalars: `Some(true)` iff their
/// descriptions are bit-equal; `None` if the type has no channel
/// (callers take the never-equal branch).
///
/// A `Sym` over `f64` or the interval scalar has no fixed-size
/// [`ScalarBits`], so [`repr_bits`] answers `None` for it, but its
/// description is still one: the same node (content-addressed, so the
/// same expression) and a bit-equal numeric channel. Any other `Sym`
/// value type has no channel.
///
/// **An identity, never a coincidence verdict.** A caller reads it to
/// choose between two sound paths, where a false "no" costs the slower
/// one, or to read which description is stored, as [`one_stored_point`]
/// reads a collapsed side; a coincidence between two cells is still its
/// margin's to decide (D10).
pub fn eq_bits<T: crate::Real>(a: &T, b: &T) -> Option<bool> {
    if let Some(same) = eq_sym::<f64, T>(a, b).or_else(|| eq_sym::<crate::Interval, T>(a, b)) {
        return same;
    }
    Some(repr_bits(a)? == repr_bits(b)?)
}

/// **Whether `a` and `b` are one stored point**: every coordinate the
/// same description ([`eq_bits`]) or both a zero of either sign (`−0`
/// and `+0` are one value, so a net revolved from the axis, whose apex
/// row is `0·x` for `x` of either sign, holds one point there), and
/// `false` at a scalar with no channel. The one reading of a spline net's collapsed side (DESIGN
/// `[extensibility]`: a boundary iso whose control points are all one
/// stored point is a chart singularity), which the singular set's home
/// reads, and which the knot algebra keeps by carrying such a pair's
/// blend as the point itself.
pub fn one_stored_point<T: crate::Real>(a: &crate::Point3<T>, b: &crate::Point3<T>) -> bool {
    let zero =
        |x: &T| eq_bits(x, &T::zero()) == Some(true) || eq_bits(x, &-T::zero()) == Some(true);
    [(a.x, b.x), (a.y, b.y), (a.z, b.z)]
        .iter()
        .all(|(p, q)| eq_bits(p, q) == Some(true) || (zero(p) && zero(q)))
}

/// [`eq_bits`] of two `Sym<V>`s, or `None` when `T` is not `Sym<V>`.
fn eq_sym<V: crate::Real, T: crate::Real>(a: &T, b: &T) -> Option<Option<bool>> {
    let (a, b): (&dyn core::any::Any, &dyn core::any::Any) = (a, b);
    let (a, b) = (
        a.downcast_ref::<crate::Sym<V>>()?,
        b.downcast_ref::<crate::Sym<V>>()?,
    );
    Some(if a.node() == b.node() {
        eq_bits(&a.value(), &b.value())
    } else {
        Some(false)
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;

    /// f64: bits decide — NaN payloads stay distinct (the PR 1 exploit
    /// class), −0.0 ≠ +0.0 (distinct descriptions), equal bits equal.
    #[test]
    fn f64_channel() {
        assert_eq!(eq_bits(&1.5f64, &1.5f64), Some(true));
        assert_eq!(eq_bits(&0.0f64, &-0.0f64), Some(false));
        let nan1 = f64::from_bits(0x7ff8_0000_0000_0001);
        let nan2 = f64::from_bits(0x7ff8_0000_0000_0002);
        assert_eq!(eq_bits(&nan1, &nan2), Some(false));
        assert_eq!(eq_bits(&nan1, &nan1), Some(true));
    }

    /// **One stored point**: the same bits, or zeros of either sign, in
    /// every coordinate; a point one ulp off is another.
    #[test]
    fn one_stored_point_reads_bits_and_either_zero() {
        use crate::Point3;
        let p = Point3::new(0.0, -0.0, 1.0);
        assert!(
            one_stored_point(&p, &Point3::new(-0.0, 0.0, 1.0)),
            "zeros of either sign"
        );
        let ulp = Point3::new(0.0, 0.0, f64::from_bits(1.0_f64.to_bits() + 1));
        assert!(!one_stored_point(&p, &ulp), "one ulp off");
    }

    /// A channel-less scalar yields `None` — the conservative rung.
    #[test]
    fn dual_has_no_channel() {
        use crate::Real;
        let a = crate::Dual64::from_f64(1.0);
        assert_eq!(eq_bits(&a, &a), None);
    }

    /// A `Sym` is its node and its value: the same node and value bits
    /// are one description, and another node is another, whatever the
    /// value reads.
    #[test]
    fn sym_channel_reads_the_node_and_the_value() {
        use crate::{ParamSymbol, Real, Sym};
        let x = Sym::<f64>::param(ParamSymbol::new(0), 0.5);
        assert_eq!(eq_bits(&x, &x), Some(true));
        assert_eq!(eq_bits(&x, &Sym::<f64>::from_f64(0.5)), Some(false));
        let i = Sym::<crate::Interval>::from_f64(2.0);
        assert_eq!(
            eq_bits(&i, &Sym::<crate::Interval>::from_f64(2.0)),
            Some(true)
        );
        assert_eq!(
            eq_bits(&i, &Sym::<crate::Interval>::from_f64(3.0)),
            Some(false)
        );
    }

    #[test]
    fn interval_channel_distinguishes_bounds_and_decoration() {
        use crate::Real;
        let a = crate::Interval::from_f64(2.0);
        let b = crate::Interval::from_f64(2.0);
        assert_eq!(eq_bits(&a, &b), Some(true));
        let c = crate::Interval::from_f64(3.0);
        assert_eq!(eq_bits(&a, &c), Some(false));
    }
}

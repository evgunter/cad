//! The **`Real`-level bit-identity door** — the ONE sanctioned seam
//! through which generic code may ask "are these two scalars the *same
//! description*, bit for bit?" (M3 PR 4; deferred here by the PR 1
//! review, which rejected the `Debug`-string channel with a NaN-payload
//! exploit and localized the interim downcast ladder in
//! `topo::merge_faces`).
//!
//! # What this is, and is not
//!
//! This is an **identity channel, not a comparison door**: through M3
//! it powered the *declared* rung of the round-8 coincidence ladder
//! (bit-equal descriptions arising from shared recipe data decided
//! coincidence exactly). Since M4 PR 5 that rung is a
//! `topo::GeomSource` lookup and NO production coincidence path
//! consults this channel (see the retirement note below) — what
//! remains is exact-representation plumbing (content-key hashing)
//! and the debug assertion behind the source lookup. It never orders
//! values, never bands, and is deliberately NOT part of the
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
//! - **Retirement LANDED (M4 PR 5, NAMING-DESIGN N6)**: production
//!   bit-identity coincidence checking is GONE — the declared rung is
//!   a `topo::GeomSource` lookup (two descriptions share a recipe
//!   source), and the bit compare survives exactly as N6 promised: a
//!   `cfg(debug_assertions)` assertion behind the lookup
//!   (`topo::source`, the "records agree with bits" check). The CI
//!   tripwires stay armed with an EMPTY production-consumer
//!   allowlist; the remaining allowlisted files are non-consumers
//!   (scalar plumbing / the debug assertion).
//!
//! A scalar type without an arm below (e.g. `Dual`, which no `Body`
//! instantiates) yields `None`: no bit channel ⇒ declared coincidence
//! cannot be certified ⇒ callers must take the conservative branch
//! (never-equal), which is the ladder's safe direction.

/// The bit-faithful identity of one scalar: `f64` ⇒ `(to_bits, 0, 0)`;
/// `Probe` ⇒ its inner `f64`'s bits; the interval scalar ⇒ its
/// `(inf_bits, sup_bits, decoration)` triple (`Interval::repr_bits`,
/// which — unlike `Bounds` — keeps NaI and empty apart). NaN payloads
/// are distinguished (the PR 1 exploit is unrepresentable here);
/// bit-identical NaNs still compare equal — a declared coincidence of
/// garbage, refused downstream by tier 3, never laundered here.
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
    #[cfg(feature = "interval")]
    if let Some(v) = any.downcast_ref::<crate::Interval>() {
        let (lo, hi, dec) = v.repr_bits();
        return Some((lo, hi, u64::from(dec)));
    }
    None
}

/// Bit-identity of two same-type scalars: `Some(true)` iff their
/// descriptions are bit-equal; `None` if the type has no channel
/// (callers take the never-equal branch).
pub fn eq_bits<T: crate::Real>(a: &T, b: &T) -> Option<bool> {
    Some(repr_bits(a)? == repr_bits(b)?)
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

    /// A channel-less scalar yields `None` — the conservative rung.
    #[test]
    fn dual_has_no_channel() {
        use crate::Real;
        let a = crate::Dual64::from_f64(1.0);
        assert_eq!(eq_bits(&a, &a), None);
    }

    #[cfg(feature = "interval")]
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

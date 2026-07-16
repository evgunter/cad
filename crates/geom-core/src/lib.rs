//! The scalar / tolerance / predicate substrate of the CAD kernel.
//!
//! `geom-core` is the bottom layer everything else stands on: the [`Real`]
//! scalar trait (instantiated at `f64` here and — behind the `interval`
//! cargo feature — at the certified interval scalar over inari; dual
//! numbers land in a later M0 PR), the single global [`Tolerance`] value, the
//! trilean predicate machinery ([`Decide`] / [`Sign`] / [`Band`] — the
//! single door from numbers to decisions), and the small fixed-dimension
//! linear algebra that the geometry layers build upon (upcoming, M0
//! PR 6). It carries the determinism charter (D9) from the first line —
//! no panics on input, typed errors, transcendentals via `libm`,
//! essentially no unsafe. See `docs/DESIGN.md` (decisions D4, D9, and
//! open question Q1) for the design contract this crate implements.

#[cfg(feature = "interval")]
pub mod interval;
pub mod predicate;
pub mod real;
pub mod tolerance;

#[cfg(feature = "interval")]
pub use interval::Interval;
pub use predicate::{
    AMBIGUITY_K, Band, BandError, BandField, Decide, Indeterminate, MarginDiag, Sign,
};
pub use real::{Bounds, Real};
pub use tolerance::{Tolerance, ToleranceEnvError, ToleranceEnvErrorKind, ToleranceError};

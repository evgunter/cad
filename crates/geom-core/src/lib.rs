//! The scalar / tolerance / predicate substrate of the CAD kernel.
//!
//! `geom-core` is the bottom layer everything else stands on: the [`Real`]
//! scalar trait (instantiated at `f64` here; intervals and dual numbers
//! land in later M0 PRs), the single global [`Tolerance`] value, the
//! trilean predicate machinery ([`Decide`] / [`Sign`] / [`Band`] — the
//! single door from numbers to decisions), and the small fixed-dimension
//! linear algebra that the geometry layers build upon (upcoming, M0
//! PR 6). It carries the determinism charter (D9) from the first line —
//! no panics on input, typed errors, transcendentals via `libm`,
//! essentially no unsafe. See `docs/DESIGN.md` (decisions D4, D9, and
//! open question Q1) for the design contract this crate implements.

pub mod predicate;
pub mod real;
pub mod tolerance;

pub use predicate::{
    AMBIGUITY_K, Band, BandError, BandField, Decide, Indeterminate, MarginDiag, Sign,
};
pub use real::Real;
pub use tolerance::{Tolerance, ToleranceEnvError, ToleranceEnvErrorKind, ToleranceError};

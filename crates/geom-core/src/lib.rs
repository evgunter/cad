//! The scalar / tolerance / predicate substrate of the CAD kernel.
//!
//! `geom-core` is the bottom layer everything else stands on: the [`Real`]
//! scalar trait (instantiated at `f64` here, at forward-mode [`dual`]
//! numbers, and — behind the `interval` cargo feature — at the certified
//! interval scalar over inari, including the dual-over-interval
//! combination), the single global [`Tolerance`] value, the
//! trilean predicate machinery ([`Decide`] / [`Sign`] / [`Band`] — the
//! single door from numbers to decisions), and the small fixed-dimension
//! [`linalg`] layer — vectors, points, matrices, affine maps, generic
//! over [`Real`] — that the geometry layers build upon. It carries the
//! determinism charter (D9) from the first line — no panics on input,
//! typed errors, transcendentals via `libm`, essentially no unsafe. See
//! `docs/DESIGN.md` (decisions D4, D9, and open question Q1) for the
//! design contract this crate implements.

pub mod dual;
#[cfg(feature = "interval")]
pub mod interval;
pub mod k_stats;
pub mod linalg;
pub mod predicate;
pub mod real;
pub mod tolerance;

#[cfg(feature = "interval")]
pub use dual::DualInterval;
pub use dual::{Dual, Dual64};
#[cfg(feature = "interval")]
pub use interval::Interval;
pub use k_stats::{MarginSample, Probe, SampleOutcome};
pub use linalg::{Affine2, Affine3, Mat2, Mat3, Point2, Point3, Vec2, Vec3};
pub use predicate::{
    Band, BandError, BandField, DEFAULT_K, Decide, Indeterminate, MarginDiag, Sign,
};
pub use real::{Bounds, Real};
pub use tolerance::{Tolerance, ToleranceEnvError, ToleranceEnvErrorKind, ToleranceError};

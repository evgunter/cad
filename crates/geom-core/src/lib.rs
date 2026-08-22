//! The scalar / tolerance / predicate substrate of the CAD kernel.
//! MEASUREMENT TOUCH — reverted before this branch merges.
//!
//! `geom-core` is the bottom layer everything else stands on: the [`Real`]
//! scalar trait (instantiated at `f64` here, at forward-mode [`dual`]
//! numbers, and — behind the `interval` cargo feature — at the certified
//! interval scalar over `interval-transcendentals`, including the
//! dual-over-interval
//! combination), the single global [`Tolerance`] value, the
//! trilean predicate machinery ([`Decide`] / [`Sign`] / [`Band`] — the
//! single door from numbers to decisions), and the small fixed-dimension
//! [`linalg`] layer — vectors, points, matrices, affine maps, generic
//! over [`Real`] — that the geometry layers build upon. It carries the
//! determinism charter (D9) from the first line — no panics on input,
//! typed errors, transcendentals via `libm`, essentially no unsafe. See
//! `docs/DESIGN.md` (decisions D4, D9, and open question Q1) for the
//! design contract this crate implements.

pub mod bit_identity;
pub mod dual;
#[cfg(feature = "interval")]
pub mod interval;
pub mod k_stats;
pub mod linalg;
pub mod predicate;
pub mod real;
pub mod ring_interval;
pub mod spline;
pub mod tolerance;

#[cfg(feature = "interval")]
pub use dual::DualInterval;
pub use dual::{Dual, Dual64};
#[cfg(feature = "interval")]
pub use interval::Interval;
#[cfg(feature = "probe")]
pub use k_stats::{MarginSample, Probe, SampleOutcome};
pub use linalg::{Affine3, FrameError, FrameInput, Mat3, Point2, Point3, Vec2, Vec3};
pub use predicate::{
    Band, BandError, BandField, COINCIDENCE_RECOURSE, DEFAULT_K, Decide, Indeterminate,
    IndeterminatePayload, Margin, MarginDiag, Sign,
};
pub use real::{Bounds, CertifiedBounds, CertifiedEnclosure, Enclosure, Real};
pub use ring_interval::RingInterval;
pub use spline::{KnotVector, SpanLocate, SpanSet, SplineError};
pub use tolerance::{
    EpsilonSource, Tol, Tolerance, ToleranceEnvError, ToleranceEnvErrorKind, ToleranceError,
    ToleranceReport,
};

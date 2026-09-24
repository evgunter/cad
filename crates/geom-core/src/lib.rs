//! The scalar / tolerance / predicate substrate of the CAD kernel.
//!
//! `geom-core` is the bottom layer everything else stands on: the [`Real`]
//! scalar trait (instantiated at `f64` here, at forward-mode [`dual`]
//! numbers, and at the certified [`Interval`] scalar over
//! `interval-transcendentals`, including the dual-over-interval
//! combination; the `interval` cargo feature gates the lane-trait impls
//! above this crate and the interval test files, not the scalar and not
//! the generic bodies that take it), the single global [`Tolerance`]
//! value, the trilean predicate machinery ([`Decide`] / [`Sign`] /
//! [`Band`] — the single door from numbers to decisions), and the small
//! fixed-dimension [`linalg`] layer — vectors, points, matrices, affine
//! maps, generic over [`Real`] — that the geometry layers build upon. It carries the
//! determinism charter (D9) from the first line — no panics on input,
//! typed errors, transcendentals via `libm`, essentially no unsafe. See
//! `docs/DESIGN.md` (decisions D4, D9, and open question Q1) for the
//! design contract this crate implements.

pub mod bit_identity;
pub mod dual;
pub mod exact;
pub mod interval;
pub mod k_stats;
pub mod linalg;
pub mod predicate;
pub mod real;
pub mod ring_interval;
pub mod spline;
pub mod sym;
pub mod tolerance;

pub use dual::{Dual, Dual64, DualInterval};
pub use interval::Interval;
#[cfg(feature = "probe")]
pub use k_stats::{MarginSample, Probe, SampleOutcome};
pub use linalg::{
    Affine3, FrameError, FrameInput, FrameVector, Mat3, OrthoAxis, OrthoFrame, OrthoFrameError,
    Point2, Point3, UnitVec3, UnitVec3Error, Vec2, Vec3, decide_unit_direction,
};
pub use predicate::{
    Band, BandError, BandField, COINCIDENCE_RECOURSE, DEFAULT_K, Decide, Indeterminate,
    IndeterminatePayload, InfSpeed, Margin, MarginDiag, MissingRecourse, Sign, SupSpeed,
};
pub use real::{
    Bounds, CertifiedBounds, CertifiedEnclosure, Enclosure, Real, is_finite_length,
    is_underflowed_length,
};
pub use ring_interval::RingInterval;
pub use spline::{KnotVector, SpanLocate, SpanSet, SplineError};
pub use sym::{ParamSymbol, Sym, SymBudget, SymCounts, SymId, SymRetry, SymRules};
pub use tolerance::{
    EpsilonSource, Tol, Tolerance, ToleranceEnvError, ToleranceEnvErrorKind, ToleranceError,
    ToleranceReport,
};

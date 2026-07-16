//! The scalar / tolerance / predicate substrate of the CAD kernel.
//!
//! `geom-core` is the bottom layer everything else stands on: the [`Real`]
//! scalar trait (instantiated at `f64` here; intervals and dual numbers
//! land in later M0 PRs), the single global [`Tolerance`] value, the
//! trilean robust predicates (upcoming, M0 PR 3), and the small
//! fixed-dimension [`linalg`] layer — vectors, points, matrices, affine
//! maps, generic over [`Real`] — that the geometry layers build upon.
//! It carries the determinism charter (D9) from the
//! first line — no panics on input, typed errors, transcendentals via
//! `libm`, essentially no unsafe. See `docs/DESIGN.md` (decisions D4, D9,
//! and open question Q1) for the design contract this crate implements.

pub mod linalg;
pub mod real;
pub mod tolerance;

pub use linalg::{Mat2, Mat3, Point2, Point3, Vec2, Vec3};
pub use real::Real;
pub use tolerance::{
    Tolerance, ToleranceEnvError, ToleranceEnvErrorKind, ToleranceError, ToleranceField,
};

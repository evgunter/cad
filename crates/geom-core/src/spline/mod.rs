//! The B-spline/NURBS **structure substrate** (M5 PR 3): validated
//! clamped knot vectors, span/basis machinery, and the knot-algebra
//! plans that insertion/refinement/removal/degree-elevation share.
//!
//! # The C6 boundary, kept visible in the module layout
//!
//! Everything that *selects structure* — knot validation, span search,
//! alpha/lambda coefficient schedules — is plain `f64` code with raw
//! comparisons, deterministic per D9, and lives in [`knots`] and
//! [`algebra`]. Everything that *evaluates* — basis values and
//! derivatives at a generic scalar — is ring-ops-only generic code in
//! [`basis`], with the span index passed in as structure. The
//! [`locate`] seam is the one bridge: per-scalar span selection,
//! sealed, with each instantiation's semantics documented there. No
//! topology decision exists anywhere in this module (nothing routes
//! through `k_stats`).
//!
//! # Conventions (binding, stated once)
//!
//! - **Clamped-v1**: end-knot multiplicity exactly `degree + 1`;
//!   interior multiplicity ≤ `degree`. Periodic and unclamped forms
//!   are a **designed absence** until a consumer exists.
//! - **Structure vs payload**: knots, weights, degrees, and every
//!   combination coefficient are `f64` (C6: cache shape is an f64-lane
//!   artifact); control points are the only generically-typed data.
//! - **Positive weights**: `w > 0`, enforced at construction — the
//!   convex-hull property every C9 hull bound stands on.
//! - **The span contract**: a [`Span`] borrows the [`KnotVector`] it is
//!   a proof about, and every door restricted to a span reads its
//!   knots from that borrow and takes no second vector — so evaluation
//!   in a span is total for every scalar with no pairing check and no
//!   refusal, and outside the span's interval it is the polynomial
//!   extension (documented garbage-out). What a span does **not**
//!   relate to its vector is the coefficient array beside it: that is
//!   a relation of length alone, so a same-length array from another
//!   curve is a wrong answer rather than a refusal. The [`Span`] docs
//!   state that residue in full.

pub mod algebra;
pub mod basis;
pub mod compose;
pub mod hull;
pub mod knots;
pub mod locate;
pub mod net;

pub use algebra::{CurvePlan, KnotAlgebraError, RemovalStep};
pub use compose::{BernsteinSpans, ComposeError, CompositeForm, CurveRingData, ImplicitSurface};
pub use hull::{CoeffWindow, SplineCoeffs};
pub use knots::{KnotVector, KnotVectorIssue, Span, SplineError, derivative_knot_slice};
pub use locate::{SpanLocate, SpanSet};
pub use net::TensorNet;

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
//! sealed, with each instantiation's semantics documented there. The
//! derivative's coefficient factor and net, at a generic scalar, have
//! their one home in [`derivative`], with the quotient bound every
//! first-order rational derivative bound reads, on pieces halved from
//! the quotient formed once over each span or cell. No
//! topology decision exists anywhere in this module (nothing routes
//! through `k_stats`).
//!
//! # Conventions (binding, stated once)
//!
//! - **Clamped-v1**: end-knot multiplicity exactly `degree + 1`;
//!   interior multiplicity ≤ `degree`. Periodic and unclamped forms
//!   are a **designed absence** until a consumer exists.
//! - **Structure vs values**: degrees, multiplicities, spans and every
//!   decision a plan takes are `f64` structure, read off the values'
//!   value channel; a carrier's knots ([`Knots`]), weights and control
//!   points are values, held in the scalar (C6). A plan's combination
//!   coefficients are formed at the applier's scalar from the values
//!   ([`CurvePlan::apply_rational`]).
//! - **Positive weights**: `w > 0`, enforced at construction — the
//!   convex-hull property every C9 hull bound stands on.
//! - **The span contract**: a [`Span`] borrows the [`KnotVector`] it is
//!   a proof about, and every door restricted to a span reads its
//!   knots from that borrow and takes no second vector — so evaluation
//!   in a span is total for every scalar with no pairing check and no
//!   refusal, and outside the span's interval it is the polynomial
//!   extension (documented garbage-out). One level down, a coefficient
//!   array takes the same shape: a [`SplineCoeffs`] (or, with the
//!   weights that license a rational claim, a [`RationalCoeffs`])
//!   borrows the vector it was fitted against, is minted only by that
//!   vector with the count checked once, and mints the [`Span`] its
//!   hull doors read — so a same-length array from another curve has
//!   no door to reach, and a span of another vector beside the pair
//!   has no spelling ([`hull`]'s docs).

pub mod algebra;
pub mod basis;
pub mod blossom;
pub mod compose;
pub mod derivative;
pub mod hull;
pub mod knot_values;
pub mod knots;
pub mod locate;
pub mod net;
pub mod pieces;
pub mod plan_inputs;
pub mod projective;
pub mod range;
pub mod weight_scale;
pub mod weights;

pub use algebra::{CurvePlan, KnotAlgebraError, RemovalStep, RingRefinement};
pub use blossom::{BlossomError, SpanRow, diagonal_in_span, restrict_segment};
pub use compose::{
    BernsteinSpans, CanonicalSurface, ComposeError, CompositeForm, CurveCertData, ImplicitSurface,
};
pub use derivative::{
    AreaSlopes, AxisCut, CellAxis, CellBlock, CellFirstQuotients, CellMeasureQuotients,
    CellQuotients, CellSecondQuotients, IntervalPieces, QUOTIENT_MAX_DEGREE, QuotientPiece,
    accel_pieces, cell_first_quotients, cell_measure_quotients, cell_quotients,
    cell_second_quotients, derivative_net, polynomial_area_pieces, span_derivative_scale,
    speed_pieces, stretch_pieces,
};
pub use hull::{
    C1Break, CoeffWindow, Derived, RationalCoeffs, RationalWindow, SplineCoeffs, SplineCoeffsBuf,
};
pub use knot_values::{KnotSpan, Knots};
pub use knots::{
    KnotMirrorError, KnotStructure, KnotVector, KnotVectorIssue, LooseKnots, Span, SplineError,
    check_weight_value, check_weight_values, knot_difference, weight_in_regime,
};
pub use locate::{SpanLocate, SpanSet, poison_from, structure_ascends, structure_nonzero};
pub use net::{TensorChannels, TensorCoeffs, TensorNet};
pub use projective::{ProjectivePoint, ProjectiveScalar, ProjectiveSplit};
pub use range::{Param, ParamRange};
pub use weights::{Weights, constant_weight};

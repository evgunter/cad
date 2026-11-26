//! Analytic and NURBS geometry: the [`Curve3`] and [`Surface`] closed
//! enums, their evaluators, the validated NURBS payloads behind their
//! universal-fallback variants, and the fitting, projection,
//! composition and box machinery those payloads carry.
//!
//! The two halves live in [`curves`] and [`surfaces`]; each module's
//! docs carry the conventions specific to its own entity kinds. What
//! they share is stated once and only once: the parameterization
//! conventions below; the §6.1 point-projection policy, whose four
//! constants are re-exported here; the azimuthal frame convention;
//! and the control-net helpers the three NURBS payloads are built
//! from. The last three live in interior modules — they carry the
//! argument, not API, and the names a consumer needs are at this
//! root.
//!
//! One door is deliberately absent, and is named here so its absence
//! reads as a decision rather than an omission: **iso-curve
//! extraction** from a NURBS payload belongs to the EdgeDescription
//! layer, under a placement rule stated and argued in `geom-brep`'s
//! `nurbs_iso` module docs.
//!
//! **One roster of the readers' funnel rows** lives in the
//! module [`rows`]: every reader of the ring convention
//! ([`RingReader`], the only minter of [`RingSites`]) and of the spiric
//! regime ([`SpiricReader`], the only minter of [`SpiricRegimeSites`]),
//! read off the two enums and checked distinct when this crate compiles.
//!
//! # Conventions (normative, stated once)
//!
//! These are the parameterization conventions every consumer — edges,
//! pcurves, tessellation — derives from. They are conventions in D2's
//! sense: data like `u_ref` *carries* a convention (where θ = 0 lives)
//! and is therefore never recomputable from the locus alone.
//!
//! - **Units (D6):** lengths in meters, angles in radians.
//! - **Entities are complete loci.** A [`Curve3`] is the *whole*
//!   infinite line or full circle — never a segment or arc; a
//!   [`Surface`] is the whole infinite plane, the full cylinder, both
//!   cone nappes. Bounds are the *topology's*: an edge bounds a
//!   carrier curve by a parameter interval its construction states from
//!   what defines it, certified against its **vertices** (the
//!   [`curves`] module docs), and a face bounds a carrier surface by its
//!   loops.
//! - **Evaluators do not range-reduce.** `sin_cos` is total on ℝ and
//!   evaluation is the same fixed formula at every parameter — no
//!   comparison, no seam special-case. As loci the azimuthal kinds are
//!   2π-periodic exactly, in the reals. Consumers that need a
//!   canonical representative reduce explicitly with
//!   [`geom_core::Real::reduce_periodic`] (θ mod 2π, seam blur
//!   documented there).
//! - **The bit-identity policy for periodicity, stated honestly:**
//!   floating-point evaluation promises **no** bit-level periodicity.
//!   2π is not representable, so `θ + k·fl(τ)` is a *different real
//!   parameter* than `θ + 2πk`; evaluations at the two f64 parameters
//!   agree to rounding (ulps scaled by `k` and by `r`), never bitwise —
//!   and range reduction itself rounds, so reduced evaluation is also
//!   value-close, not bit-identical. What IS promised: evaluation is a
//!   pure function (same input bits → same output bits, D9), and at the
//!   interval scalar the enclosure of an evaluation contains the true
//!   image of every parameter in the input enclosure — the containment
//!   form of periodicity (evaluate over `θ + k·Real::tau()` at interval
//!   type and the true periodic image is enclosed).
//! - **A carrier's frame is a rigid motion; a line's direction is a unit
//!   vector.** The `axis`/`u_ref` frame of every analytic kind but the
//!   line (a plane's `normal`/`u_ref` among them) is a [`Frame`], which
//!   holds a [`geom_core::RigidMotion`]: unit and square as a property of
//!   the type. A construction mints its frame, or states one and squares
//!   it with no decision ([`Frame::new`]), where a degenerate statement
//!   is poison. A line's `dir` is a [`geom_core::UnitVec3`], so its
//!   parameter is arc length: a construction hands it a frame's axis, a
//!   chord over its length ([`Line::chord`], the segment its span
//!   `(0, length)`), or an isometry's image of a unit direction or a
//!   swing in one frame's plane, each through its own door, which
//!   measures its rounding at a point scalar ([`geom_core::UnitVec3`]'s
//!   module docs); a stated `f64` direction
//!   keeps its bits where it is unit to rounding and fails loud where it
//!   is not ([`geom_core::UnitVec3::stated`]'s `None`, which the import
//!   refuses typed).
//!   **At rest, `topo`'s tier-3 check 1 certifies what moves a
//!   locus a datum can lever**: no direction may be the zero vector,
//!   and each scalar datum lies inside its convention (a radius above
//!   zero, a cone's half-angle inside `(0, π/2)`)
//!   ([`Surface::representability_margins`],
//!   [`Curve3::representability_margins`]). It reads no frame: the
//!   `axis`/`u_ref` frame of a cylinder, cone, sphere, torus, circle,
//!   ellipse or spiric is a [`Frame`], a `RigidMotion`, unit and square
//!   by its type (fork 274 decision 1), so no gate reads it back from
//!   its entries. At an enclosure scalar those entries enclose a
//!   rotation; at `f64` they hold one to their rounding, which the
//!   readers that lever it charge ([`Frame::rounding`]). Nor does it
//!   certify a line's `dir`, unit by its type, or a plane's unit
//!   `normal` — each spans the same locus at any length. A spline's stored datum is its
//!   control net, and the same check refuses a net — a face's or an
//!   edge carrier's — carrying a control point that is not a finite
//!   number (NaN or `±∞`); its weights and knots are refused
//!   non-finite on their value channel at construction.
//!
//! # Totality and poison (geom-core's policy, inherited)
//!
//! Every evaluator is **total**: no panic, no `Result`. Out-of-domain
//! cases produce the scalar's poison value (NaN at `f64`, NaI/empty at
//! the interval scalar) which flows through values and is caught at the
//! predicate/certification layer (D4 ¶2).
//!
//! **A NURBS net describes something.** A curve or surface value is
//! always a description: "this face has no surface yet" is a fact about
//! a body mid-construction and lives on the body's surface slot in
//! `topo`, never in a net. So poison in a net is only ever corruption,
//! and the one question asked of a net is whether every control point
//! is a finite number at the scalar's own finiteness
//! ([`NurbsSurface::is_finite`], [`NurbsCurve3::is_finite`]: NaN and
//! `±∞` at `f64`, an enclosure reaching `±∞` or empty at `Interval`).
//! A net that is not finite fails loudly at every consumer that
//! evaluates it, and the at-rest validator refuses it by name. The box
//! constructors screen on the same question: a net with one
//! non-finite coordinate bounds its locus on no axis, so it yields the
//! poison box, which overlaps everything.
//!
//! # Evaluation-code discipline
//!
//! All evaluation *arithmetic* here is comparison-free ring/trig code:
//! `sin_cos` is the trig primitive, no fused operations, fixed
//! documented association orders (D9). The enum evaluators are bounded
//! by [`geom_core::spline::SpanLocate`] (as the **sole bound** — a
//! sealed `Real` subtrait, the same style rule as `Bounds`): NURBS
//! evaluation needs per-instantiation knot-span *selection*, a
//! structure decision the seam localizes per scalar (its module docs
//! carry each instantiation's semantics and the `Dual` kink
//! convention). The bound adds no comparison surface to generic code.
//! Everything instantiates at `f64`, `Probe`, `Dual<f64>`, `Interval`,
//! and `Dual<Interval>` — the derivative-vs-dual consistency and
//! enclosure-containment test axes rely on exactly that.

#![cfg_attr(not(test), deny(clippy::disallowed_methods))]
mod azimuth;
pub mod carrier_speed;
mod chart_point;
mod convention;
pub mod cover;
pub mod curves;
mod datum;
mod frame;
mod memo;
mod net;
mod periodic;
pub mod places;
pub mod pole_form;
mod projection_policy;
pub mod rows;
mod scalar_lift;
pub mod surfaces;
// Test fixtures; see the module's docs. `doc(hidden)` because the
// rustdoc gate runs `--all-features`.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub mod test_support;

pub use azimuth::{
    ANGLE_WINDOW_START, Azimuth, LATITUDE_WINDOW_START, Quarter, Window, radial_of_legs, separation,
};
pub use chart_point::{
    ChannelRead, ChartAngle, ChartArm, ChartChannels, ChartInverse, ChartPoint, NoClosedInverse,
};
pub use convention::{ConventionEnd, RepresentabilityMargin, SquareFrame};
pub use curves::second_derivative::{SecondDerivativeUnbounded, nonrational_second_derivative_sup};
#[cfg(any(test, feature = "test-support"))]
pub use curves::stated_direction;
pub use curves::{
    CarrierEscalation, CarrierRefusal, Circle, Clearance, Collocation, ComposeError, Conic, Curve3,
    CurveData, CurveDatum, CurveKind, CurveWindow2, CurveWindow3, Ellipse, EllipseInvalid,
    FIT_REMOVAL_BUDGET, FitError, FitOutcome, Foot, Line, LinePair, MajorFrame, NurbsCurve2,
    NurbsCurve3, OnCarrier, PERIODIC_DEGREE, PERIODIC_MIN_KNOTS, ParamSide, PeriodicError,
    PeriodicNurbs3, PeriodicViewError, PieceRead, PieceStep, PieceWalkRefusal, Projection2,
    Projection3, ProjectionInconclusive, Ray, RefitSkip, SeamSide, SpiricInvalid, SpiricRegime,
    compose_chain, conic_period, ellipse_level_clearance, spiric_curvature_sup, spiric_f_range,
    spiric_period, spiric_period_rate_bounds, spiric_radial, spiric_rate_bounds, spiric_regime,
    spiric_window_rate_bounds,
};
pub use datum::{AnalyticData, DatumValue};
pub use frame::{Frame, plane_offset, plane_offset_rounded};
pub use geom_core::{bracket_mid, mid_param};
pub use periodic::periodic_window_may_hold;
pub use rows::{RingReader, RingSites, SpiricReader, SpiricRegimeSites};
// The §6.1 policy module is interior — its body is the argument for
// these four values, not API — but the values themselves are the
// public names both halves' callers have always used.
pub use projection_policy::{
    PROJECT_EPS_COSINE, PROJECT_EPS_POINT, PROJECT_MAX_ITERS, PROJECT_SEEDS_PER_SPAN,
};
pub use surfaces::{
    AnalyticPairs, AnglePeriods, ApproxSurface, ApproxWindow, NotARing, NurbsSurface,
    OffsetCertificate, RingTorus, RunSide, SheetApex, SpanBeside, SpindleEnd, SpindleMiss, Surface,
    SurfaceData, SurfaceDatum, SurfaceDescription, SurfaceJet, SurfaceJet3, SurfaceKind,
    SurfacePairing, SurfaceProjection, SurfaceProjectionInconclusive, SurfaceSpec, SurfaceWindow,
    is_ring_torus, meridian_side, require_ring_torus, ring_torus, sheet_apex, sheet_distance,
    spindle_sheet, torus_tube,
};

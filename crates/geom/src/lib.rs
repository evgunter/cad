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
//!   carrier curve by its **vertices**, and a face bounds a carrier
//!   surface by its loops.
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
//! - **Unit-vector and orthogonality fields are conventional data,
//!   unchecked here.** `dir`, `axis`, `normal`, `u_ref` are unit by
//!   convention, `u_ref ⊥ axis` (⊥ `normal`) by convention.
//!   Constructors do not renormalize (a hidden normalize would
//!   silently reparameterize; D6's meters-per-parameter contract is the
//!   caller's to establish) and evaluators consume the fields as given.
//!   Tier-3 geometric validation certifies the invariants at rest;
//!   violating them yields well-defined garbage (a non-arc-length
//!   parameterization, an elliptical "circle"), not poison and not a
//!   panic.
//!
//! # Totality and poison (geom-core's policy, inherited)
//!
//! Every evaluator is **total**: no panic, no `Result`. Out-of-domain
//! and undescribed cases produce the scalar's poison value (NaN at
//! `f64`, NaI/empty at the interval scalar) which flows through values
//! and is caught at the predicate/certification layer — in particular,
//! evaluating a [`Curve3::nurbs_placeholder`] or
//! [`Surface::nurbs_placeholder`] "no description yet" state yields an
//! all-poison point (representable ≠ described; the poison fails every
//! downstream certification loudly, per D4 ¶2).
//!
//! **Telling the two states apart is one rule with one spelling.** The
//! discriminator is the control net's poison: a placeholder's every
//! control point is all-poison by construction, a described net is
//! finite data. `all`, not `any` — a described net with one poisoned
//! point is corrupt *described* geometry and must fail loudly as such
//! (certification, +V, export), never masquerade as the benign
//! placeholder. The two payloads that carry the state expose it as
//! [`NurbsCurve3::is_placeholder`] and
//! [`NurbsSurface::is_placeholder`]; every consumer that tells the
//! states apart goes through one of those rather than re-deriving the
//! test inline.
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

mod azimuth;
pub mod curves;
mod net;
mod projection;
#[cfg(test)]
mod scalar_lift;
pub mod surfaces;

pub use curves::{
    ComposeError, Curve3, EllipseInvalid, FIT_REMOVAL_BUDGET, FitError, FitOutcome, NurbsCurve2,
    NurbsCurve3, Projection2, Projection3, ProjectionInconclusive, RefitSkip, SeamSide,
    compose_chain,
};
// The §6.1 policy module is interior — its body is the argument for
// these four values, not API — but the values themselves are the
// public names both halves' callers have always used.
pub use projection::{
    PROJECT_EPS_COSINE, PROJECT_EPS_POINT, PROJECT_MAX_ITERS, PROJECT_SEEDS_PER_SPAN,
};
pub use surfaces::{
    ApproxSurface, ChartWindow, NurbsSurface, OffsetCertificate, Surface, SurfaceDescription,
    SurfaceJet, SurfaceJet3, SurfaceProjection, SurfaceProjectionInconclusive, SurfaceSpec,
    SurfaceWindow,
};

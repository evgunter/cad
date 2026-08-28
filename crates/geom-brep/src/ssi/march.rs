//! The Hoffmann §6.2 stepper — **untrusted** (M5 PR 7 spec §1, C3).
//!
//! # Nothing here is trusted, and that is the design
//!
//! This module is a *candidate generator*. Its product is a polyline of
//! states (plus frames) handed to the PR 4 fitting stack; the **C2
//! certificate is the only gate** ([`super::certify`]). A branch jump
//! is not something a step predicate catches — no local datum can prove
//! the global property "no other branch within reach" — it is a
//! certificate refusal: the uniqueness tube fails to be a graph, or
//! transversality dies, and the refusal is typed. C3 inverts the
//! planning question on purpose.
//!
//! # The stepper (realized)
//!
//! At the current state `x`, with `A` the [`LocalSystem`] Jacobian:
//!
//! 1. `Svd<M, N>` of `A`. The null direction is the unit tangent `d₁`,
//!    oriented against the previous step; `σ_min` is Hoffmann's
//!    transversality signal (p. 217), reported as diagnostic.
//! 2. **`ssi_transversality`**: the *decision* uses the dimensionally
//!    honest form `sin θ · arm` in meters — θ the angle between the two
//!    surface normals, `arm` the folded curvature/extent lever arm —
//!    exactly `dihedral_wedge`'s shape (D4 ¶1). `Sign::Zero` is the
//!    **sliver band**: refuse toward C7 (`TangentIntersection`, PR 9).
//!    We never desingularize; Hoffmann §6.5's quadratic-transformation
//!    tracing through singular points is deliberately not adopted.
//! 3. `A·d₂ = b₂` solved **minimum-norm** — which *is* Frenet's γ₂ = 0
//!    (see [`geom_core::linalg::svd`]'s module docs for the one-line
//!    derivation from unit speed).
//! 4. `A·d₃ = b₃` minimum-norm, then `d₃ ← d₃ − κ²·d₁` with
//!    `κ = ‖d₂‖` — Frenet's γ₃ = −κ².
//! 5. **Step size by the small-contribution heuristic** (p. 215): the
//!    quadratic and cubic terms of the approximant may each deviate by
//!    at most [`SSI_STEP_DEVIATION`]·ε in meters, giving
//!    `h ≤ √(2δ/κ)` and `h ≤ ∛(6δ/‖d₃‖)`; clamped into
//!    `[h_min, h_max]`. **`ssi_step_progress`** refuses if the step
//!    collapses into the band — a stepper that cannot move at this
//!    tolerance says so.
//! 6. Advance by the cubic approximant, then **Newton refinement** to
//!    the surface pair: a fixed [`SSI_NEWTON_ITERS`] cap of
//!    minimum-norm corrections, early-exiting on
//!    [`SSI_NEWTON_TOL`]·ε (an f64 structure branch, C6 lane).
//!
//! # The idealized stepper (the differential spec, T4/PERF-PLAN §4.4)
//!
//! [`StepperMode::Idealized`] replaces steps 3–5 with a **tangent-line
//! step of fixed tiny length** [`SSI_IDEALIZED_STEP`] — ten readable
//! lines that *define* the traced locus. Everything else (the
//! transversality band, Newton refinement, the closure trio, the
//! domain clipping) is shared, so the differential suite compares the
//! two steppers and nothing else.
//!
//! **What the differential pin can and cannot be.** PERF-PLAN §4.2's
//! usual pin is byte-equality; it is not available here and pretending
//! otherwise would be the dishonest option. The two steppers place
//! *different numbers of samples at different arc lengths* by
//! construction — that is the entire difference between them — so no
//! output object is common to both. The honest executable pin, which
//! the suite uses, is: identical **branch topology** (count, closure
//! verdict, end kind per branch) and every idealized sample lying
//! within the realized branch's own **certified** residual band of the
//! realized carrier. A divergence in either is a definite bug in the
//! realized stepper, which is what §4.3's correlated-error argument
//! asks the pin to catch.
//!
//! # Closure and loop topology are margined decisions
//!
//! Three named Q1 trileans, never raw comparisons (C3):
//!
//! - **`ssi_closure_return`** — "the trace returned to its start": the
//!   margin is `h − ‖x − x₀‖` in meters, so closure means *the next
//!   step would step over the seed*. Armed only after the trace has
//!   left a three-step neighborhood of its seed (a structural latch,
//!   not a decision).
//! - **`ssi_closure_tangent`** — "this branch closed, it did not merely
//!   arrive somewhere": the margin is `cos φ · arc`, `φ` the angle
//!   between the returning tangent and the seed tangent, `arc` the
//!   branch's own arc length as the lever arm. `Positive` is the
//!   affirmative (the trace came back running the same way it left);
//!   `Zero` means the two directions are perpendicular to within the
//!   band and `Negative` means it came back *reversed* — a cusp or a
//!   retrace, either way not a loop, and both refuse.
//!
//!   The margin is deliberately **not** `sin φ · arc`. A discretely
//!   marched loop closes at a point a step away from its seed, so its
//!   returning tangent differs from the seed tangent by `O(hκ)` —
//!   metering *that* against ε would demand nanoradian agreement from a
//!   stepper the design explicitly does not trust, and every honest
//!   loop would refuse. What this predicate is for is telling a closure
//!   apart from a *crossing*, and that distinction lives at `O(1)` in
//!   the angle, which is exactly where `cos φ` reads it. The
//!   fine-grained question — "is there really only one arc here?" — is
//!   not a stepper question at all: it is limb 3's, where a genuine
//!   self-crossing makes the uniqueness tube's enclosure straddle.
//! - **`ssi_branch_open_end`** — "the branch ends on the domain
//!   boundary": the margin is the signed distance to the named domain
//!   in meters. `Negative` ends the branch open; `Zero` also ends it
//!   and **labels the end in-band** ([`BranchEnd::BoundaryInBand`]).
//!   The label is a report, not a mechanism: nothing keys off it. A
//!   region no tube covers — including one past an in-band end — is
//!   refined by the accounting pass and refuses typed at the floor,
//!   which is the guarantee, and it holds whether or not the end was
//!   in band.

use geom_core::linalg::svd::Svd;
use geom_core::{Band, Indeterminate, Margin, Point3, Sign, Vec3};

use crate::dihedral::decide;

use super::SsiError;
use super::system::LocalSystem;

/// The **candidate generator's** step tolerance, in meters.
///
/// The marcher is deliberately untrusted: it runs in `f64`, it makes no
/// trilean decision, and every number it produces is re-derived by the
/// certificate through the run's [`Band`] before anything is claimed.
/// [`SSI_NEWTON_TOL`] and [`SSI_STEP_DEVIATION`] scale this value into
/// that generator's convergence tolerance and between-sample deviation
/// budget.
///
/// It is a distinct type — not a bare `f64` beside the `Band` — because
/// the two are the *same quantity* on every door that certifies, and a
/// second `f64` copy of the run tolerance is exactly the shape that
/// lets a caller march at one tolerance and certify at another.
///
/// **The type does not leave this crate.** No public door takes or
/// returns one — the uncertified trace, the only door that names the
/// generator's tolerance at all, takes a bare `f64` and mints one
/// here. Inside `ssi` the only constructor a certifying door can reach
/// is [`MarchTol::from_band`]; a tolerance that is not the run band's
/// comes from [`MarchTol::decoupled`], which that one door owns.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct MarchTol(f64);

impl MarchTol {
    /// The generator's tolerance derived from the run's band — the run
    /// tolerance ε itself, since a linear [`Band`]'s `zero()` **is** ε,
    /// stored unmodified by `Band::new`.
    ///
    /// A tolerance built any other way cannot cross the certifying
    /// seam — the finishers refuse with
    /// [`SsiError::MarchTolMismatch`] — so the marcher's spacing, the
    /// accounting floor and the certificate's floors are one number by
    /// enforcement rather than by intent.
    #[must_use]
    pub(crate) fn from_band(band: Band) -> Self {
        Self(band.zero())
    }

    /// A generator tolerance **deliberately decoupled** from the run
    /// band, for the door that returns no certificate.
    ///
    /// Private to `ssi` on purpose: a `pub` version of this is the
    /// second ε entering under a nicer name, and a maintainer editing a
    /// certifying door is the caller who would reach for it. The only
    /// legitimate use is measuring the marcher against itself — how the
    /// fitted pair's deviation scales as the generator is tightened,
    /// independently of the ambient tolerance the run is banded at.
    ///
    /// A decoupled march is **not** a hole in the certificate. Every
    /// floor and every trilean downstream is stated in the `Band`, so a
    /// generator run at some other tolerance can only produce a *worse
    /// carrier*, which then certifies honestly at the band or refuses
    /// on one of the three limbs.
    ///
    /// # Errors
    ///
    /// [`SsiError::InvalidMarchTol`] when `meters` is not finite and
    /// strictly positive — a typed refusal, never a silent clamp.
    pub(super) fn decoupled(meters: f64) -> Result<Self, SsiError> {
        if !(meters.is_finite() && meters > 0.0) {
            return Err(SsiError::InvalidMarchTol { value: meters });
        }
        Ok(Self(meters))
    }

    /// The tolerance in meters — the `f64` the untrusted lane consumes.
    #[must_use]
    pub(crate) fn meters(self) -> f64 {
        self.0
    }
}

/// Fixed Newton-refinement cap per step (D9 — never data-dependent).
pub const SSI_NEWTON_ITERS: usize = 8;

/// Newton's early-exit residual, as a fraction of ε: refinement stops
/// once the state satisfies both surfaces two orders inside tolerance.
/// A structure branch on `f64` (C6's lane), not a predicate.
pub const SSI_NEWTON_TOL: f64 = 1.0e-2;

/// Hoffmann's "keep the higher contributions small" (p. 215) as a named
/// constant: the quadratic and cubic terms of the approximant may each
/// reach this fraction of the **linear** term. Dimensionless, because
/// the heuristic is about the local model staying a good model, not
/// about tolerance.
pub const SSI_STEP_RELATIVE: f64 = 0.1;

/// The share of ε the *between-sample* error of the eventual cubic fit
/// may consume — the second, separately-named step cap.
///
/// Hoffmann's relative heuristic alone is not enough here, and saying
/// why matters: it keeps the *approximant* honest, but our samples are
/// then handed to a cubic fitting stack whose product must be within ε
/// of the locus **between** them (C2.2). The standard interpolation
/// bound `‖C − fit‖ ≲ h⁴·‖C⁗‖/384` with `‖C⁗‖ ≈ κ³` for a curve of
/// slowly-varying curvature turns that into a cap on `h`, which is the
/// one that actually binds on a small tight loop. Both caps are
/// applied; the step is the smaller.
///
/// Two caveats keep this a *design target* rather than a bound, and
/// the certificate — never this constant — is what refuses when the
/// target is missed. First, `‖C⁗‖ ≈ κ³` holds for slowly-varying
/// curvature and understates a curve whose curvature swings. Second,
/// limb 2's control-hull enclosure is conservative over the true
/// deviation by a factor measured at roughly 4–20× on the M5 fixtures.
/// The value is therefore set well below ε rather than at it.
pub const SSI_STEP_DEVIATION: f64 = 0.02;

/// The idealized stepper's fixed step, as a fraction of the caller's
/// named extent. Tiny by construction: the tangent-line step's own
/// truncation is `O(h²κ)`, so at a thousandth of the feature extent it
/// is far below ε on anything with sane curvature, and Newton removes
/// what remains.
pub const SSI_IDEALIZED_STEP: f64 = 1.0e-3;

/// The realized stepper's largest step, as a fraction of the extent —
/// a cap so a nearly-straight branch still gets sampled densely enough
/// for the fit to have data.
pub const SSI_STEP_MAX: f64 = 1.0 / 32.0;

/// Which stepper — the realized third-order approximant or the
/// idealized tangent-line spec (module docs, PERF-PLAN §4.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StepperMode {
    /// The Hoffmann third-order Frenet approximant with adaptive step.
    Realized,
    /// Tangent-line steps of fixed tiny length — the differential spec.
    Idealized,
}

impl StepperMode {
    /// The mode's display name (for typed refusals).
    pub fn name(self) -> &'static str {
        match self {
            Self::Realized => "realized (third-order Frenet approximant)",
            Self::Idealized => "idealized (tangent-line, fixed tiny step)",
        }
    }
}

/// How a traced branch ended.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BranchEnd {
    /// The trace returned to its seed on a matching tangent — a loop.
    Closed,
    /// The trace left the named domain (definitely).
    Boundary,
    /// The trace reached the domain boundary **in band**: the end is
    /// within ε of the boundary and the honest report is that we do not
    /// know which side it fell on.
    ///
    /// This is a *label on the branch*, not a special case anywhere
    /// else: there is no accounting mechanism keyed on it. What
    /// actually happens to the region past such an end is what happens
    /// to any region no tube covers — [`super::exhaust`]'s accounting
    /// pass fails to contain those cells, refines them, and refuses
    /// typed at the floor. That is the honest guarantee, and it does
    /// not depend on this label at all.
    BoundaryInBand,
}

/// One traced branch: the state polyline plus its topology verdicts.
#[derive(Clone, Debug)]
pub struct Trace<const N: usize> {
    /// The states, in march order, seed first.
    pub states: Vec<[f64; N]>,
    /// How the branch ended.
    pub end: BranchEnd,
    /// The smallest `sin θ · arm` (meters) seen along the trace — the
    /// transversality headroom, reported so a consumer can see how
    /// close to the C7 regime this branch ran.
    pub min_transversality: f64,
    /// The smallest σ_min seen — Hoffmann's own signal, diagnostic.
    pub min_sigma: f64,
    /// Steps consumed.
    pub steps: usize,
}

/// Everything the stepper needs that is not the system: the named
/// domain, the lever arms, and the budgets.
#[derive(Clone, Copy, Debug)]
pub(crate) struct MarchContext<const N: usize> {
    /// The domain box in state coordinates, `[lo, hi]` per coordinate.
    pub domain: [[f64; 2]; N],
    /// The caller's named feature extent, in meters — the lever arm of
    /// last resort and the scale of the step clamps.
    pub extent: f64,
    /// The candidate generator's step tolerance. Derived from the run
    /// band on every certifying door — see [`MarchTol`].
    pub tol: MarchTol,
    /// Step budget.
    pub max_steps: usize,
}

/// The unit normal of the locus's first surface at a state, and of the
/// second — the pair whose angle is the transversality margin.
pub(crate) type NormalPair = (Vec3<f64>, Vec3<f64>);

/// A system that can also report the two surface normals and the
/// curvature lever arm at a state — everything the *decisions* need,
/// separated from the algebra the stepper needs so the trilean's
/// margin stays dimensionally honest (meters) at both trace shapes.
pub(crate) trait TransversalityData<const N: usize> {
    /// Unit normals of the two operands at the state.
    fn normals(&self, x: &[f64; N]) -> NormalPair;
    /// The folded curvature lever arm at the state, in meters
    /// (`f64::MAX` where no curvature bounds it — the plane identity).
    fn lever_arm(&self, x: &[f64; N]) -> f64;
}

/// March one branch from `seed` (module docs).
///
/// # Errors
///
/// [`SsiError::TransversalityBand`] in the σ₂ sliver band (toward C7),
/// [`SsiError::StepCollapsed`], [`SsiError::StepBudget`],
/// [`SsiError::SelfCrossingLocus`], [`SsiError::Escalated`] for any
/// other in-band trilean, [`SsiError::SeedRefinementFailed`] when the
/// seed will not settle onto the locus.
pub(crate) fn march<const M: usize, const N: usize, S>(
    sys: &S,
    seed: [f64; N],
    ctx: MarchContext<N>,
    mode: StepperMode,
    direction: f64,
    band: Band,
) -> Result<Trace<N>, SsiError>
where
    S: LocalSystem<M, N> + TransversalityData<N>,
{
    let mut x = newton_refine(sys, seed, ctx.tol)
        .ok_or(SsiError::SeedRefinementFailed { mode: mode.name() })?;
    let seed_state = x;
    let mut states = vec![x];
    let mut prev_tangent: Option<[f64; N]> = None;
    let mut seed_tangent: Option<[f64; N]> = None;
    let mut min_transversality = f64::INFINITY;
    let mut min_sigma = f64::INFINITY;
    let mut left_start = false;
    let mut steps = 0usize;

    while steps < ctx.max_steps {
        // ---- 1. the local decomposition ----
        let a = sys.jacobian(&x);
        let svd = Svd::<M, N>::new(a);
        let sigma = svd.sigma_min();
        if sigma < min_sigma {
            min_sigma = sigma;
        }

        // ---- 2. ssi_transversality (the σ₂ sliver band ⇒ C7) ----
        let (n1, n2) = sys.normals(&x);
        let sin_theta = n1.cross(n2).norm() / (n1.norm() * n2.norm());
        let arm = sys.lever_arm(&x).min(ctx.extent);
        match decide("ssi_transversality_arm", Margin::of(arm), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => {
                return Err(SsiError::Escalated(Indeterminate {
                    margin: geom_core::MarginDiag::Invalid,
                    band,
                    predicate: Some("ssi_transversality_arm"),
                }));
            }
            Err(diag) => return Err(SsiError::Escalated(diag)),
        }
        let transversality = Margin::levered(sin_theta, arm);
        if transversality.value() < min_transversality {
            min_transversality = transversality.value();
        }
        match decide("ssi_transversality", transversality, band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero) => {
                // The sliver band: a tangential (or in-band tangential)
                // contact along the candidate locus. C7's regime.
                return Err(SsiError::TransversalityBand {
                    sin_theta,
                    arm,
                    sigma_min: sigma,
                });
            }
            Ok(Sign::Negative) => {
                // sin θ · arm is a magnitude; a definite negative can
                // only be poison arithmetic. Refuse rather than march.
                return Err(SsiError::TransversalityBand {
                    sin_theta,
                    arm,
                    sigma_min: sigma,
                });
            }
            Err(diag) => return Err(SsiError::Escalated(diag)),
        }

        // ---- 3. the tangent, oriented along the march ----
        let mut d1 = svd.null_direction();
        if let Some(prev) = prev_tangent {
            if dot(&d1, &prev) < 0.0 {
                d1 = neg(&d1);
            }
        } else {
            if direction < 0.0 {
                d1 = neg(&d1);
            }
            seed_tangent = Some(d1);
        }
        prev_tangent = Some(d1);

        // Meters per unit of the march parameter (the state-space
        // tangent is a unit vector; the 3-D speed converts it).
        let speed = sys.tangent_speed(&x, &d1);
        if speed.is_nan() || speed <= 0.0 {
            // A collapsed chart speed: the state moves and the point
            // does not. Nothing downstream can be stated in meters,
            // so refuse rather than divide by it.
            return Err(SsiError::StepCollapsed {
                mode: mode.name(),
                step_meters: 0.0,
            });
        }

        // ---- 4./5. the step ----
        let (dx, h_meters) = match mode {
            StepperMode::Idealized => {
                // The spec: a tangent line of fixed tiny length.
                let h = (SSI_IDEALIZED_STEP * ctx.extent) / speed;
                (scale(&d1, h), h * speed)
            }
            StepperMode::Realized => {
                let b2 = sys.rhs2(&x, &d1);
                let d2 = svd.solve_min_norm(&b2);
                let kappa = norm(&d2);
                let b3 = sys.rhs3(&x, &d1, &d2);
                let mut d3 = svd.solve_min_norm(&b3);
                // Frenet: γ₃ = −κ².
                for (i, v) in d3.iter_mut().enumerate() {
                    *v -= kappa * kappa * d1[i];
                }
                let n3 = norm(&d3);
                // (a) Hoffmann's relative heuristic: |h²κ/2| ≤ ρ·h and
                //     |h³‖d₃‖/6| ≤ ρ·h.
                let h_quad = if kappa > 0.0 {
                    2.0 * SSI_STEP_RELATIVE / kappa
                } else {
                    f64::INFINITY
                };
                let h_cub = if n3 > 0.0 {
                    (6.0 * SSI_STEP_RELATIVE / n3).sqrt()
                } else {
                    f64::INFINITY
                };
                // (b) the fit's between-sample budget (module docs).
                // κ and ‖d₃‖ are in state units; the curvature that
                // governs the 3-D fit is κ/speed², so convert once.
                let kappa3d = kappa / (speed * speed);
                let h_fit = if kappa3d > 0.0 {
                    // ¼ power as TWO square roots, not `powf(0.25)`:
                    // `f64::sqrt` is IEEE-correctly-rounded and so is
                    // its composition, while `powf` is a libm routine
                    // with no such guarantee and no libm-only contract
                    // behind it. Same value to within an ulp, but the
                    // D9 promise ("libm-only, bit-replayable") is only
                    // true of the sqrt route.
                    ((24.0 * SSI_STEP_DEVIATION * ctx.tol.meters()) / (kappa3d * kappa3d * kappa3d))
                        .sqrt()
                        .sqrt()
                        / speed
                } else {
                    f64::INFINITY
                };
                let h_max = (SSI_STEP_MAX * ctx.extent) / speed;
                let h = h_quad.min(h_cub).min(h_fit).min(h_max);
                let mut step = [0.0f64; N];
                for (i, s) in step.iter_mut().enumerate() {
                    *s = h * d1[i] + 0.5 * h * h * d2[i] + (h * h * h / 6.0) * d3[i];
                }
                (step, h * speed)
            }
        };

        // The stepper must be able to move at this tolerance.
        match decide("ssi_step_progress", Margin::of(h_meters), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => {
                return Err(SsiError::StepCollapsed {
                    mode: mode.name(),
                    step_meters: h_meters,
                });
            }
            Err(diag) => return Err(SsiError::Escalated(diag)),
        }

        let mut next = x;
        for (i, v) in next.iter_mut().enumerate() {
            *v += dx[i];
        }

        // ---- 6. Newton refinement to the surface pair ----
        let Some(refined) = newton_refine(sys, next, ctx.tol) else {
            // A step that will not settle is a step into nothing: end
            // the branch honestly rather than record a bad sample.
            return Err(SsiError::SeedRefinementFailed { mode: mode.name() });
        };
        next = refined;
        steps += 1;

        // ---- ssi_branch_open_end ----
        let inside = domain_margin(&next, &ctx, sys, &x);
        match decide("ssi_branch_open_end", Margin::of(inside), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero) => {
                push_boundary(sys, &mut states, x, next, &ctx);
                return Ok(Trace {
                    states,
                    end: BranchEnd::BoundaryInBand,
                    min_transversality,
                    min_sigma,
                    steps,
                });
            }
            Ok(Sign::Negative) => {
                push_boundary(sys, &mut states, x, next, &ctx);
                return Ok(Trace {
                    states,
                    end: BranchEnd::Boundary,
                    min_transversality,
                    min_sigma,
                    steps,
                });
            }
            Err(diag) => return Err(SsiError::Escalated(diag)),
        }

        // ---- the closure pair ----
        let back = distance_meters(sys, &next, &seed_state);
        if !left_start && back > 3.0 * h_meters {
            left_start = true;
        }
        if left_start {
            match decide("ssi_closure_return", Margin::of(h_meters - back), band) {
                Ok(Sign::Positive) | Ok(Sign::Zero) => {
                    // Returned. Is it a closure or a crossing?
                    let t0 = seed_tangent.unwrap_or(d1);
                    let cos_phi = dot(&d1, &t0).clamp(-1.0, 1.0);
                    let arc = arc_length(sys, &states);
                    match decide("ssi_closure_tangent", Margin::levered(cos_phi, arc), band) {
                        Ok(Sign::Positive) => {
                            // Close exactly onto the seed. If the last
                            // marched state is already essentially the
                            // seed, replace it rather than append: a
                            // zero-length chord is what the fitting
                            // stack refuses, and a loop that closed
                            // well is not an error.
                            if let Some(last) = states.last().copied() {
                                let mut d = 0.0f64;
                                for (i, v) in last.iter().enumerate() {
                                    let g = v - seed_state[i];
                                    d += g * g;
                                }
                                if d.sqrt() <= 1.0e-12 * h_meters.max(1.0) {
                                    states.pop();
                                }
                            }
                            states.push(seed_state);
                            return Ok(Trace {
                                states,
                                end: BranchEnd::Closed,
                                min_transversality,
                                min_sigma,
                                steps,
                            });
                        }
                        Ok(Sign::Zero | Sign::Negative) => {
                            return Err(SsiError::SelfCrossingLocus {
                                cos_phi,
                                arc_length: arc,
                            });
                        }
                        Err(diag) => return Err(SsiError::Escalated(diag)),
                    }
                }
                Ok(Sign::Negative) => {}
                Err(diag) => return Err(SsiError::Escalated(diag)),
            }
        }

        states.push(next);
        x = next;
    }
    Err(SsiError::StepBudget {
        mode: mode.name(),
        budget: ctx.max_steps,
    })
}

/// Minimum-norm Newton onto `F = 0`, fixed cap, early exit at
/// [`SSI_NEWTON_TOL`]·ε. `None` when the iteration poisons or fails to
/// settle — never a best-effort state.
pub(crate) fn newton_refine<const M: usize, const N: usize, S>(
    sys: &S,
    mut x: [f64; N],
    step_tol: MarchTol,
) -> Option<[f64; N]>
where
    S: LocalSystem<M, N>,
{
    let tol = SSI_NEWTON_TOL * step_tol.meters();
    for _ in 0..SSI_NEWTON_ITERS {
        let f = sys.residual(&x);
        let mut worst = 0.0f64;
        for v in f.iter() {
            let a = v.abs();
            if a.is_nan() {
                return None;
            }
            if a > worst {
                worst = a;
            }
        }
        if worst <= tol {
            return Some(x);
        }
        let svd = Svd::<M, N>::new(sys.jacobian(&x));
        let mut rhs = [0.0f64; M];
        for (i, r) in rhs.iter_mut().enumerate() {
            *r = -f[i];
        }
        let dx = svd.solve_min_norm(&rhs);
        for (i, v) in x.iter_mut().enumerate() {
            *v += dx[i];
            if !v.is_finite() {
                return None;
            }
        }
    }
    // One last look: the cap is fixed, so a state that arrived inside
    // tolerance on the final correction still counts.
    let f = sys.residual(&x);
    if f.iter().all(|v| v.abs() <= tol) {
        Some(x)
    } else {
        None
    }
}

fn dot<const N: usize>(a: &[f64; N], b: &[f64; N]) -> f64 {
    let mut acc = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        acc += x * y;
    }
    acc
}

fn norm<const N: usize>(a: &[f64; N]) -> f64 {
    dot(a, a).sqrt()
}

fn neg<const N: usize>(a: &[f64; N]) -> [f64; N] {
    let mut o = *a;
    for v in o.iter_mut() {
        *v = -*v;
    }
    o
}

fn scale<const N: usize>(a: &[f64; N], k: f64) -> [f64; N] {
    let mut o = *a;
    for v in o.iter_mut() {
        *v *= k;
    }
    o
}

/// Distance between two states, in meters, through the coordinate
/// scales.
fn distance_meters<const M: usize, const N: usize, S>(sys: &S, a: &[f64; N], b: &[f64; N]) -> f64
where
    S: LocalSystem<M, N>,
{
    let scales = sys.coordinate_scale(a);
    let mut acc = 0.0f64;
    for (i, s) in scales.iter().enumerate() {
        let d = (a[i] - b[i]) * s;
        acc += d * d;
    }
    acc.sqrt()
}

/// The polyline's 3-D arc length so far, in meters — the closure
/// trilean's lever arm.
fn arc_length<const M: usize, const N: usize, S>(sys: &S, states: &[[f64; N]]) -> f64
where
    S: LocalSystem<M, N>,
{
    let mut acc = 0.0f64;
    for w in states.windows(2) {
        acc += (sys.point(&w[1]) - sys.point(&w[0])).norm();
    }
    acc
}

/// The signed distance from the state to the domain box, in meters
/// (positive inside), through the coordinate scales.
fn domain_margin<const M: usize, const N: usize, S>(
    x: &[f64; N],
    ctx: &MarchContext<N>,
    sys: &S,
    at: &[f64; N],
) -> f64
where
    S: LocalSystem<M, N>,
{
    let scales = sys.coordinate_scale(at);
    let mut worst = f64::INFINITY;
    for (i, s) in scales.iter().enumerate() {
        let lo = (x[i] - ctx.domain[i][0]) * s;
        let hi = (ctx.domain[i][1] - x[i]) * s;
        worst = worst.min(lo).min(hi);
    }
    worst
}

/// Fixed bisection count for the domain-crossing search (D9).
pub const SSI_BOUNDARY_BISECTIONS: usize = 32;

/// Whether a state is inside the named domain box (a structure test on
/// the raw coordinates — C6's lane, not a predicate).
fn within<const N: usize>(x: &[f64; N], domain: &[[f64; 2]; N]) -> bool {
    x.iter()
        .enumerate()
        .all(|(i, v)| *v >= domain[i][0] && *v <= domain[i][1])
}

/// Push the branch's boundary endpoint.
///
/// **Clipping, not clamping**, and the difference is a two-millimetre
/// residual: coordinate-wise clamping of the overshooting state moves
/// it *off the locus*, and that fabricated point then goes into the fit
/// and shows up as an on-locus certificate failure at the branch's own
/// end.
///
/// So the step is bisected (fixed count, D9) for the largest fraction
/// whose **Newton-refined** state is still inside the box. Testing
/// insideness *after* refinement rather than before is the whole
/// subtlety: Newton moves the crossing point along the locus, and on a
/// branch that runs to a surface's own knot-domain edge it moves it
/// *out* — by a micron on the M5 wall fixture, which is a thousand ε
/// and duly failed limb 1. The endpoint this produces is on the locus
/// **and** inside the domain, within one bisection of the boundary.
///
/// A crossing that will not refine, or one that lands on top of the
/// previous state, is dropped: the branch then ends at its last
/// certified state, which is honest.
fn push_boundary<const M: usize, const N: usize, S>(
    sys: &S,
    states: &mut Vec<[f64; N]>,
    inside: [f64; N],
    outside: [f64; N],
    ctx: &MarchContext<N>,
) where
    S: LocalSystem<M, N>,
{
    let mut lo = 0.0f64;
    let mut hi = 1.0f64;
    let mut best: Option<[f64; N]> = None;
    for _ in 0..SSI_BOUNDARY_BISECTIONS {
        let m = 0.5 * (lo + hi);
        let mut t = inside;
        for (i, v) in t.iter_mut().enumerate() {
            *v = inside[i] + (outside[i] - inside[i]) * m;
        }
        match newton_refine(sys, t, ctx.tol) {
            Some(r) if within(&r, &ctx.domain) => {
                lo = m;
                best = Some(r);
            }
            _ => hi = m,
        }
    }
    let Some(end) = best else {
        return;
    };
    // Reject a duplicate of the previous state (zero-length chord).
    let mut d = 0.0f64;
    for (i, v) in end.iter().enumerate() {
        let g = v - inside[i];
        d += g * g;
    }
    if d.sqrt() > 0.0 {
        states.push(end);
    }
}

/// The 3-D points of a trace — the fitting stack's input.
pub(crate) fn trace_points<const M: usize, const N: usize, S>(
    sys: &S,
    trace: &Trace<N>,
) -> Vec<Point3<f64>>
where
    S: LocalSystem<M, N>,
{
    trace.states.iter().map(|s| sys.point(s)).collect()
}

/// March **both ways** from a seed and splice, unless the forward march
/// already closed the loop.
///
/// A seed lands in the middle of whatever branch it is on. Marching one
/// direction covers half of an open branch, which is not a branch — it
/// is half a lie, and it would arrive at the fitting stack looking like
/// a complete carrier. So an open forward march is followed by a
/// backward one from the same seed, and the two are spliced with the
/// seed as the join. A closed forward march needs no second pass: it
/// already covered the component.
///
/// # Errors
///
/// As [`march`]; a refusal in either direction is the operation's.
pub(crate) fn march_both<const M: usize, const N: usize, S>(
    sys: &S,
    seed: [f64; N],
    ctx: MarchContext<N>,
    mode: StepperMode,
    band: Band,
) -> Result<Trace<N>, SsiError>
where
    S: LocalSystem<M, N> + TransversalityData<N>,
{
    let fwd = march::<M, N, S>(sys, seed, ctx, mode, 1.0, band)?;
    if fwd.end == BranchEnd::Closed {
        return Ok(fwd);
    }
    let bwd = march::<M, N, S>(sys, seed, ctx, mode, -1.0, band)?;
    let mut states = bwd.states;
    states.reverse();
    // `states` now runs backward-end → seed; append the forward half
    // without repeating the seed.
    states.extend_from_slice(&fwd.states[1..]);
    Ok(Trace {
        states,
        // If either end is in-band at the boundary, the branch's end is
        // in-band: the honest verdict is the weaker of the two.
        end: if fwd.end == BranchEnd::BoundaryInBand || bwd.end == BranchEnd::BoundaryInBand {
            BranchEnd::BoundaryInBand
        } else {
            BranchEnd::Boundary
        },
        min_transversality: fwd.min_transversality.min(bwd.min_transversality),
        min_sigma: fwd.min_sigma.min(bwd.min_sigma),
        steps: fwd.steps + bwd.steps,
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{MarchTol, SsiError};
    use geom_core::Band;

    /// The bridge, stated as a row: a marcher tolerance derived from a
    /// band **is** that band's coincidence threshold. Nothing scales it,
    /// pads it, or rounds it — the marcher's step rule and the
    /// certificate's floors are the same number, so a run cannot march
    /// at one tolerance and certify at another.
    #[test]
    fn a_derived_march_tolerance_is_the_bands_own_zero() {
        for zero in [1.0e-3_f64, 1.0e-6, 1.0e-9, 1.0e-12] {
            let band = Band::new(zero, 10.0 * zero).unwrap();
            assert_eq!(MarchTol::from_band(band).meters(), band.zero());
        }
    }

    /// The decoupled constructor is the only other door, and it refuses
    /// typed rather than clamping: a generator tolerance that is not a
    /// length is a caller error, not a value to repair silently.
    #[test]
    fn a_decoupled_march_tolerance_refuses_typed_on_a_non_length() {
        for bad in [0.0_f64, -1.0e-9, f64::NAN, f64::INFINITY] {
            match MarchTol::decoupled(bad) {
                Err(SsiError::InvalidMarchTol { value }) => {
                    assert!(value.is_nan() || value == bad, "{value:e} vs {bad:e}");
                    let msg = format!("{}", SsiError::InvalidMarchTol { value });
                    assert!(
                        msg.contains("MarchTol::from_band"),
                        "the recourse sentence: {msg}"
                    );
                }
                other => panic!("expected a typed refusal for {bad:e}, got {other:?}"),
            }
        }
        assert_eq!(MarchTol::decoupled(1.0e-9).unwrap().meters(), 1.0e-9);
    }
}

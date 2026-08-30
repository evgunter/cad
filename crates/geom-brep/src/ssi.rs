//! **Rung 3 of the C1 ladder: surface–surface intersection by
//! march-then-certify, with in-op exhaustiveness** (M5 PR 7 — C2, C3,
//! C12.8).
//!
//! # The shape of the operation
//!
//! ```text
//!   subdivision  ──►  seeds  ──►  marcher  ──►  fit  ──►  C2 certificate
//!   (exhaustive)      (never      (UNTRUSTED)   (PR 4)    (three limbs,
//!        │             luck)                              the only gate)
//!        └────────────────── accounting ◄─── uniqueness tubes ──┘
//! ```
//!
//! Four modules, four obligations, and the boundary between them is the
//! whole design:
//!
//! - [`march`] generates candidates and is **trusted for nothing**. A
//!   branch jump is not caught by a step predicate (no local datum can
//!   prove "no other branch within reach"); it becomes a certificate
//!   refusal.
//! - [`certify`] is the only gate: on-locus residuals, a control-hull
//!   sup-norm bound, and a uniqueness tube that *proves* one-arc-ness
//!   over a box chain. All three, always (OQ2).
//! - [`exhaust`] closes the never-silence obligation: every cell of the
//!   bounded domain is excluded, accounted, or the operation refuses
//!   typed at the named floor. It is also the seed generator, so
//!   "marching finds it" never depends on luck.
//! - [`enclose`] supplies every certified bound, in the C9 ring only.
//!
//! # The two arms wired here (spec §5, minimal by rule)
//!
//! Exactly the surface pairs the M5 acceptance shapes need. Every other
//! rung-3 arm of the C5 table keeps its typed refusal **citing its
//! unimplemented trace shape** — per-arm retirement, never wholesale
//! (C12.1).
//!
//! | arm | trace shape | why |
//! |---|---|---|
//! | [`cylinder_sphere_ssi`] | **ℝ³** implicit pair, 2×3 SVD | shape (iv)'s planted small loop and the σ₂-sliver row — **retired**, all three limbs (M5 PR 7) |
//! | [`plane_nurbs_ssi`] | **ℝ⁴** parametric×parametric, 3×4 SVD | shape (iii)'s substrate — **retired 2026-07-31**, all three limbs (M5 PR 7b) |
//!
//! # The plane×NURBS arm's retirement (M5 PR 7b)
//!
//! Everything the ℝ⁴ shape needs landed in PR 7 and is exercised: the
//! 3×4 SVD, third-order surface jets, the trace itself, the
//! shared-parameter fit of the carrier and **both** pcurves (the OQ4
//! identity, which [`trace_plane_nurbs_uncertified`] exposes and the
//! acceptance suite pins), certified surface foot points for limb 1,
//! the chart-form uniqueness tube for limb 3, and UV-domain
//! exhaustiveness. The one missing limb — **limb 2 against the NURBS
//! operand**, a *tight* between-samples sup bound — refused typed: the
//! per-span first-order enclosure was sound but scaled like the span
//! width (~1e-2 m reported where the true residual is ~1e-10 m),
//! because it enclosed the curve's and the surface image's variations
//! separately and threw away the cancellation that is the whole
//! content of `S(P(t)) = C(t)`.
//!
//! M5 PR 7b landed the machinery that refusal named: **tensor-product
//! Bernstein composition** (`geom_core::spline::compose::tensor`),
//! which encloses the difference `S(P(t)) − C(t)` as ONE composite at
//! the coefficient level, so the bound tracks the residual's own scale
//! (measured within ~1% of a 2·10⁵-sample dense scan on the wall
//! fixture). Limb 2 flipped to that bound, the `implemented` flag
//! flipped, and nothing else here changed — the arm retired **with its
//! proof** (C12.1), never as a "sampled max pretending to be a bound"
//! (C2.2).
//!
//! One honesty note the tight bound surfaced: the certificate now sees
//! the *fit pair's* real between-samples deviation, which the loose
//! bound used to drown. Where the wall's section curvature crosses
//! zero, the step rule's fit rung (`h_fit ∝ (ε/κ³)^¼`) unbinds and the
//! realized deviation of the two independent fits can genuinely exceed
//! ε (measured 3.8e-9 m at march-ε = 1e-9 on a gently inflected wall)
//! — the certificate then refuses **in-band at `ssi_hull_sup_chart`**,
//! which is the honest verdict about that carrier, not a bound
//! artifact (the bound sits within ~1% of the dense-scan truth there).
//! That deviation is **phase-dependent and non-monotone in march-ε**,
//! not a fixed cap (PR 7b review measurement: 4× tighter march-ε →
//! 4.36× better, 16× → 16.92× better reaching 2.25e-10 m, 64× → only
//! 6.83× better — where the samples land relative to the crossing
//! decides), so a marcher-side fix must price the crossing span
//! itself rather than assume a global scaling law. That is marcher
//! work, out of PR 7b's scope by its spec §6. A wall with
//! slowly-varying section curvature certifies with two orders of
//! headroom.
//!
//! Practical breadth today, stated plainly: the certification the
//! retired arm delivers end-to-end is **gentle single-cell walls near
//! the origin** — an interior-knot (multi-cell) wall currently
//! refuses at limb 1 (march/fit quality at the knot line), a span
//! window straddling a knot line or leaving the domain hulls the
//! neighbor cell's polynomial extension into the bound or poisons,
//! and far-from-origin operands hit the projection and exhaustiveness
//! machinery's own representation floors — every one of these is a
//! loud, typed refusal, never a silent miscertification.
//!
//! **Why cylinder×sphere and not cylinder×torus** (the spec's own "or
//! equivalent"): both operands' polynomial composites convert to meters
//! **exactly** (`÷2R`), so limb 2 certifies with no invented scale
//! factor. The torus's composite is quartic (m⁴) and its conversion
//! back to meters needs a certified reciprocal of `A + 4Rρ`, which
//! needs a square root the C9 ring deliberately does not have. Shipping
//! the pair whose certificate is *exact* rather than the pair whose
//! certificate would need a new unratified mechanism is the same
//! judgment C12.1 makes everywhere: retire arms one at a time, with
//! their proofs. The fixture property the spec actually asks for —
//! a branch naive marching MISSES — is delivered by an offset cylinder
//! cutting a sphere in two loops of very different size, of which the
//! small one touches no domain boundary.
//!
//! # Booleans are not here
//!
//! This PR proves carriers, certificates, and exhaustiveness on the
//! **intersection layer**, plus the `split_edge` meter that lets PR 5's
//! lanes consume a rung-3 carrier. Zipping curved booleans end to end,
//! and the tangency regime the σ₂ band refuses toward, are PR 9.

pub mod certify;
pub mod enclose;
pub mod exhaust;
pub mod jet;
pub mod march;
pub mod system;

use geom::{Curve3, FitError, NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::{Band, Indeterminate, Margin, Point3};

pub use certify::{SSI_CERT_SPANS, SSI_TUBE_RADIUS, SsiCertificate, SsiLimb};
pub use exhaust::{Exhaustiveness, SSI_FLOOR, SSI_MAX_CELLS, SSI_SEED_FLOOR};
pub use march::{
    BranchEnd, SSI_IDEALIZED_STEP, SSI_NEWTON_ITERS, SSI_NEWTON_TOL, SSI_STEP_DEVIATION,
    SSI_STEP_MAX, StepperMode,
};

use enclose::{Box3, NurbsBoxes};
use exhaust::UvRect;
use march::{MarchContext, MarchTol, Trace, march_both, trace_points};
use system::{Chart, ImplicitPairR3, ParametricPairR4};

/// The two chart-parameter sample sequences of an ℝ⁴ trace — the
/// coordinate projections that become the two pcurves, on the carrier's
/// own parameter (the OQ4 identity).
type ChartSamples<'a> = (&'a [geom_core::Point2<f64>], &'a [geom_core::Point2<f64>]);

/// A fitted rung-3 branch's curves: the 3-D carrier, and the two
/// pcurves when the trace shape produced them (the ℝ⁴ shape does; the
/// ℝ³ shape has no chart to project onto).
type FittedBranch = (
    NurbsCurve3<f64>,
    Option<NurbsCurve2<f64>>,
    Option<NurbsCurve2<f64>>,
);

/// The ℝ⁴ trace's fitted triple: the carrier and both pcurves, on one
/// shared parameter.
pub type TracedTriple = (NurbsCurve3<f64>, NurbsCurve2<f64>, NurbsCurve2<f64>);

/// The degree of a fitted rung-3 carrier (C1 rung 3 / C11): cubic, the
/// standard choice for a curve that must carry curvature and torsion.
pub const SSI_FIT_DEGREE: usize = 3;

/// The marcher's step budget per branch. Fixed (D9); exceeding it is a
/// typed refusal, never a truncated branch.
pub const SSI_MAX_STEPS: usize = 20_000;

/// How many marched samples one branch's fit may consume.
///
/// The step rule spaces samples so a cubic through them stays inside ε
/// between them, which makes the count scale as `(L·κ^{3/4})·ε^{−1/4}`
/// — at ε = 1e-12 a 0.08 m loop wants ~4000 of them. The fit's solve is
/// **cubic** in that count, so a tolerance three decades finer than the
/// default turns a two-second operation into an hour-long one. That is
/// a resource wall, and this kernel's rule for resource walls is the
/// same everywhere ([`SSI_MAX_STEPS`], [`SSI_MAX_CELLS`]): a named
/// budget and a typed refusal, never a silent truncation and never a
/// carrier fitted from a sample set too coarse for its own tolerance.
///
/// Raising it is not the fix when it fires; the fix is the banked
/// compaction work (least-squares fitting on a chosen knot structure
/// via `approximate_with_params`, whose cost is linear in the samples),
/// which is why the refusal names it.
pub const SSI_MAX_FIT_SAMPLES: usize = 1200;

/// An operand of a rung-3 intersection, tagged by which certificate
/// machinery its limbs use.
///
/// Generic since M6-2: the *tracing* of a branch is `f64` by design
/// (untrusted candidate generation — `jet`/`march`/`system` stay
/// `f64`-only), but *certifying* one is the consumer's own scalar, and
/// a body at rest holds `Surface<T>`.
pub enum SsiOperand<'a, T: geom_core::Real> {
    /// An analytic surface: implicit residuals and `compose` hulls.
    Analytic(&'a Surface<T>),
    /// A NURBS surface: certified foot points and chart hulls.
    Nurbs(&'a NurbsSurface<T>),
}

/// The two lengths a rung-3 certificate is stated over: the lever arm
/// the transversality margin is levered by, and the feature extent that
/// sets the tube ladder's widest rung.
///
/// **One type rather than two parallel parameters**, for the reason
/// that made the tolerance one type: at three of the four call sites
/// they are the same quantity, so as a pair of positional arguments
/// they are a second copy waiting to drift — the same shape S25 named
/// for ε, one line below it. [`TubeScale::uniform`] is the common
/// case; [`TubeScale::split`] is for the caller that genuinely has a
/// curvature arm narrower than the feature it sits on.
#[derive(Clone, Copy, Debug)]
pub struct TubeScale<T> {
    /// The lever arm the transversality margin is stated over.
    pub(crate) arm: T,
    /// The feature extent, in meters — the ladder's widest rung.
    pub(crate) extent: f64,
}

impl<T: geom_core::Bounds> TubeScale<T> {
    /// One length for both: the caller's named feature, used as the
    /// transversality lever arm and as the ladder's widest rung.
    #[must_use]
    pub fn uniform(arm: T) -> Self {
        Self {
            arm,
            extent: geom_core::Bounds::hi(arm),
        }
    }

    /// A transversality lever arm distinct from the feature extent —
    /// the ℝ³ analytic arm, where the folded curvature radius is
    /// genuinely tighter than the domain's named extent.
    #[must_use]
    pub fn split(arm: T, extent: f64) -> Self {
        Self { arm, extent }
    }
}

/// A typed rung-3 refusal — D4 ¶3: actionable, closed, never silence.
#[derive(Clone, Debug, PartialEq)]
pub enum SsiError {
    /// The transversality margin `sin θ · arm` landed in the sliver
    /// band along the candidate locus. This is the C7 regime
    /// (`TangentIntersection`, M5 PR 9): tangential contact is a
    /// construction, not something to march through, and in-band
    /// contact is a genuine sliver of the operand pair (F6). We never
    /// desingularize — Hoffmann §6.5 is deliberately not adopted.
    TransversalityBand {
        /// The sine of the angle between the operand normals.
        sin_theta: f64,
        /// The lever arm folded against it, in meters.
        arm: f64,
        /// Hoffmann's own σ₂ signal at the same state (diagnostic).
        sigma_min: f64,
    },
    /// The subdivision reached its named floor with a cell it could
    /// neither exclude nor account for. **This is the never-silence
    /// obligation firing**: there may be a branch in that cell and we
    /// decline to pretend otherwise.
    ExhaustivenessInconclusive {
        /// The offending cell's width, in meters (or chart units).
        cell_width: f64,
        /// The floor it hit.
        floor: f64,
        /// Cells examined before the refusal.
        examined: u32,
    },
    /// The cell enumeration exceeded its budget — a refusal, never a
    /// silently truncated search.
    CellBudget {
        /// The budget.
        budget: usize,
    },
    /// A branch exceeded the step budget.
    StepBudget {
        /// Which stepper.
        mode: &'static str,
        /// The budget.
        budget: usize,
    },
    /// The step size collapsed into the tolerance band: the stepper
    /// cannot make progress at this ε.
    StepCollapsed {
        /// Which stepper.
        mode: &'static str,
        /// The collapsed step, in meters.
        step_meters: f64,
    },
    /// Newton refinement would not settle a state onto the surface
    /// pair (a seed outside every basin, or poisoned arithmetic).
    SeedRefinementFailed {
        /// Which stepper.
        mode: &'static str,
    },
    /// The trace arrived back at its seed running the **wrong way** —
    /// perpendicular to, or reversed from, the direction it left in.
    /// That is a cusp or a self-crossing of the candidate locus, which
    /// is a degenerate operand pair, not a loop to close.
    SelfCrossingLocus {
        /// The cosine of the angle between the returning and seed
        /// tangents (≈ +1 for a genuine closure).
        cos_phi: f64,
        /// The branch's arc length, the lever arm it was measured over.
        arc_length: f64,
    },
    /// A C2 limb refused. The limb is named so a consumer — and the
    /// acceptance suite's corrupted-cache rows — can tell them apart.
    CertificateLimb {
        /// Which limb.
        limb: SsiLimb,
        /// The offending value, in meters.
        value: f64,
    },
    /// Limb 3 never ran: the tube ladder is EMPTY. Every rung radius
    /// `SSI_TUBE_RADIUS_MAX·extent / 2^k` sits below the ladder floor
    /// `SSI_TUBE_RADIUS·ε`, which happens when the carrier's extent is
    /// small against the run's tolerance — a structural fact about the
    /// pairing of geometry and ε, decided before any box is probed.
    ///
    /// It is NOT a limb exceedance, and it does not carry a margin:
    /// nothing was measured. This variant exists because the case used
    /// to be reported as `CertificateLimb { limb: Tube, value: NaN }`
    /// — a structural refusal wearing a limb-exceeded costume, whose
    /// NaN payload then had to be laundered by every consumer that
    /// tried to render it.
    TubeLadderEmpty {
        /// The carrier extent the ladder was scaled from (meters).
        extent: f64,
        /// The floor every rung fell below (meters).
        floor: f64,
    },
    /// Limb 3 ran and never got an answer: the ladder had rungs, but
    /// no rung's tube probe produced an enclosure at all (each
    /// returned nothing rather than a margin). Structural, like
    /// [`SsiError::TubeLadderEmpty`], and likewise carrying no margin
    /// — the distinction between them is exactly whether any radius
    /// was ever tried.
    TubeProbeSilent {
        /// How many rungs were offered and answered with nothing.
        rungs: u32,
    },
    /// Limb 3's transversality enclosure straddles zero over a tube
    /// box: two branches pass within the band of each other. A genuine
    /// sliver (F6), not a resolution failure to retry.
    TubeStraddles {
        /// The certified margin (zero when the enclosure straddles).
        margin: f64,
        /// Boxes in the chain.
        boxes: u32,
    },
    /// A certified foot point would not converge, so limb 1 has no
    /// residual to check against a NURBS operand.
    FootPointInconclusive {
        /// The schedule parameter it failed at.
        t: f64,
        /// The last distance the projection saw.
        last_distance: f64,
    },
    /// The fitting stack refused the marched polyline.
    Fit(FitError),
    /// One branch's marched polyline exceeded
    /// [`SSI_MAX_FIT_SAMPLES`]. The tolerance and the operand
    /// curvature together demand more samples than the (cubic) fit can
    /// afford — refused, never fitted from a coarser set than the
    /// certificate would need.
    FitSampleBudget {
        /// Samples the branch produced.
        samples: usize,
        /// The budget.
        budget: usize,
    },
    /// This configuration routes to a certificate mechanism this build
    /// does not have — a documented per-arm boundary (C12.1), never a
    /// fallback.
    UnsupportedCertificate {
        /// What is missing, in the caller's terms.
        what: &'static str,
    },
    /// The caller routed the wrong kinds into an arm.
    WrongLane {
        /// What the arm expects.
        expected: &'static str,
    },
    /// A named trilean landed in the ambiguity band or poisoned (F6).
    Escalated(Indeterminate),
    /// Band construction refused.
    Band(geom_core::BandError),
    /// A decoupled marcher step tolerance was not a usable length.
    InvalidMarchTol {
        /// The offending value, in meters.
        value: f64,
    },
    /// A branch reached the certifying seam having been marched at a
    /// tolerance that is not the run band's. **This is S25's divergence
    /// caught at the one place both numbers are in scope**: the
    /// certified doors derive the generator's tolerance from the band
    /// and can only reach this by being edited, so it refuses rather
    /// than certifying a carrier generated at some other ε.
    MarchTolMismatch {
        /// The tolerance the carrier was marched at, in meters.
        marched: f64,
        /// The run band's coincidence threshold, in meters.
        band_zero: f64,
    },
}

impl From<FitError> for SsiError {
    fn from(e: FitError) -> Self {
        Self::Fit(e)
    }
}

impl From<geom_core::BandError> for SsiError {
    fn from(e: geom_core::BandError) -> Self {
        Self::Band(e)
    }
}

impl core::fmt::Display for SsiError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TransversalityBand {
                sin_theta,
                arm,
                sigma_min,
            } => write!(
                f,
                "ssi: transversality died along the candidate locus \
                 (sin θ = {sin_theta:e}, arm = {arm:e} m, σ₂ = {sigma_min:e}) — this is \
                 the tangency regime (TangentIntersection), not a locus to \
                 march through; separate the operands, declare the tangency, or lower \
                 the tolerance"
            ),
            Self::ExhaustivenessInconclusive {
                cell_width,
                floor,
                examined,
            } => write!(
                f,
                "ssi: exhaustiveness inconclusive — after {examined} cells a cell of \
                 width {cell_width:e} at the refinement floor {floor:e} could be \
                 neither excluded nor accounted for, so a branch may be hiding in it; \
                 the operation refuses rather than report a possibly incomplete \
                 intersection"
            ),
            Self::CellBudget { budget } => write!(
                f,
                "ssi: the exhaustiveness subdivision exceeded its {budget}-cell budget \
                 — refused rather than truncated"
            ),
            Self::StepBudget { mode, budget } => write!(
                f,
                "ssi: the {mode} stepper exceeded its {budget}-step budget on one branch"
            ),
            Self::StepCollapsed { mode, step_meters } => write!(
                f,
                "ssi: the {mode} stepper's step collapsed to {step_meters:e} m, inside \
                 the tolerance band — no progress is possible at this ε"
            ),
            Self::SeedRefinementFailed { mode } => write!(
                f,
                "ssi: Newton refinement would not settle a {mode} state onto the \
                 surface pair"
            ),
            Self::SelfCrossingLocus {
                cos_phi,
                arc_length,
            } => write!(
                f,
                "ssi: the trace returned to its start running the wrong way \
                 (cos φ = {cos_phi:e} over {arc_length:e} m of arc) — the candidate \
                 locus cusps or crosses itself, which is a degenerate operand pair"
            ),
            Self::CertificateLimb { limb, value } => write!(
                f,
                "ssi: the fitted carrier failed {} at {value:e} m — the cache is not \
                 within tolerance of the locus it claims",
                limb.name()
            ),
            Self::TubeLadderEmpty { extent, floor } => write!(
                f,
                "ssi: the uniqueness tube's radius ladder is empty — every rung scaled \
                 from the carrier's {extent:e} m extent falls below the {floor:e} m \
                 ladder floor, so limb 3 never ran. The carrier is too short against \
                 this run's tolerance to carry a certified tube; nothing was measured"
            ),
            Self::TubeProbeSilent { rungs } => write!(
                f,
                "ssi: none of the uniqueness tube's {rungs} ladder rungs produced an \
                 enclosure, so limb 3 has nothing to decide — a structural refusal, \
                 with no margin behind it"
            ),
            Self::TubeStraddles { margin, boxes } => write!(
                f,
                "ssi: the uniqueness tube's transversality enclosure straddles zero \
                 over its {boxes}-box chain (margin {margin:e} m) — two branches pass \
                 within the band of each other, which is a genuine sliver of the \
                 operand pair, not a resolution to refine away"
            ),
            Self::FootPointInconclusive { t, last_distance } => write!(
                f,
                "ssi: the certified foot point at t = {t} would not converge (last \
                 distance {last_distance:e} m), so the on-locus residual against the \
                 NURBS operand cannot be stated"
            ),
            Self::Fit(e) => write!(f, "ssi: the fitting stack refused the marched trace: {e}"),
            Self::FitSampleBudget { samples, budget } => write!(
                f,
                "ssi: one branch marched {samples} samples against a {budget}-sample fit \
                 budget — at this tolerance and this operand curvature the carrier needs \
                 more control points than the fit's cubic solve can afford; raise the \
                 tolerance, or wait for the compaction work that fits a chosen knot \
                 structure by least squares instead of interpolating every sample"
            ),
            Self::UnsupportedCertificate { what } => {
                write!(f, "ssi: {what}")
            }
            Self::WrongLane { expected } => write!(
                f,
                "ssi: wrong dispatch lane — this arm traces {expected} (caller bug)"
            ),
            // The Indeterminate Display carries the shared two-tolerance
            // recourse exactly once (S6).
            Self::Escalated(diag) => write!(
                f,
                "ssi: a trace or certificate trilean escalated — an ill-conditioned \
                 operand pair at this tolerance: {diag}"
            ),
            Self::Band(e) => write!(f, "ssi: {e}"),
            Self::InvalidMarchTol { value } => write!(
                f,
                "ssi: the marcher's step tolerance {value:e} m is not a usable length \
                 (finite and > 0); derive it from the run band with `MarchTol::from_band`"
            ),
            Self::MarchTolMismatch { marched, band_zero } => write!(
                f,
                "ssi: the carrier was marched at {marched:e} m but the run band's \
                 tolerance is {band_zero:e} m — a certified branch may not be \
                 generated at one tolerance and certified at another; build the \
                 marcher's tolerance with `MarchTol::from_band`"
            ),
        }
    }
}

impl std::error::Error for SsiError {}

/// One certified rung-3 branch of an intersection locus.
#[derive(Clone, Debug)]
pub struct SsiBranch {
    /// The fitted carrier — a `Curve3::Nurbs` (C1 rung 3).
    pub carrier: Curve3<f64>,
    /// The carrier's parameter span.
    pub params: (f64, f64),
    /// How the branch ends.
    pub end: BranchEnd,
    /// The three-limb certificate. An `SsiBranch` cannot be built
    /// without one.
    pub certificate: SsiCertificate<f64>,
    /// The witness, `carrier(mid)` — unchanged from M2
    /// (`WitnessMidpoint`; S2 stays discharged).
    pub witness: Point3<f64>,
    /// The first operand's pcurve, when the trace produced one (the ℝ⁴
    /// shape does; the ℝ³ shape does not). **On the carrier's own
    /// parameter** — the PR 6 identity contract.
    pub pcurve_a: Option<NurbsCurve2<f64>>,
    /// The second operand's pcurve, same contract.
    pub pcurve_b: Option<NurbsCurve2<f64>>,
    /// The smallest transversality margin the march saw, in meters.
    pub min_transversality: f64,
    /// The generator's step tolerance this carrier was marched at, in
    /// meters — the receipt that makes the tie observable rather than
    /// merely intended. Equal to the run band's coincidence threshold
    /// on every certified door, enforced at the certifying seam
    /// ([`SsiError::MarchTolMismatch`]).
    pub march_tol: f64,
}

/// The result of a rung-3 intersection: the certified branches **and**
/// the proof that they are all of them.
#[derive(Clone, Debug)]
pub struct SsiOutcome {
    /// The certified branches.
    pub branches: Vec<SsiBranch>,
    /// The subdivision's accounting — the never-silence receipt.
    pub exhaustiveness: Exhaustiveness,
    /// How many seeds the subdivision produced.
    pub seeds: u32,
}

/// Knobs a caller must name rather than have guessed: the bounded
/// domain and the feature extent.
///
/// **The tolerance is not among them.** Every door that takes an
/// `SsiDomain` also takes the run's [`Band`], whose `zero()` *is* the
/// run's ε, and the floors this struct derives read it from there — so
/// a caller cannot size the accounting floor at one tolerance and
/// decide against another.
#[derive(Clone, Copy, Debug)]
pub struct SsiDomain {
    /// The session-box slab (ℝ³ lane) or the plane window (ℝ⁴ lane),
    /// as a center and half-extent in meters.
    pub center: Point3<f64>,
    /// Half-extent of the slab, in meters.
    pub half_extent: f64,
    /// The caller's named feature extent — the lever arm of last
    /// resort (D4 ¶1), in meters.
    pub extent: f64,
    /// Multiplier on [`SSI_FLOOR`] for the **accounting** floor only
    /// (seeding is unaffected — see [`SsiDomain::seed_floor`]). `1.0`
    /// is the standard floor; the acceptance suite raises it far above
    /// the tube radius to demonstrate the typed floor refusal on a
    /// fixture that otherwise succeeds — the found-AND-floor-refused
    /// variant, where branches ARE found and the operation still
    /// declines to claim they are all of them.
    pub floor_scale: f64,
}

impl SsiDomain {
    /// The slab as a ring box.
    fn slab(&self) -> Box3 {
        Box3::around(self.center, self.half_extent)
    }

    /// The accounting floor, in meters.
    ///
    /// Stated in the run band's ε and nowhere else: the floor is a
    /// proof obligation, so the tolerance it is stated in is the one
    /// the trileans decide against, by construction rather than by
    /// convention at the call site.
    fn floor(&self, band: Band) -> f64 {
        SSI_FLOOR * band.zero() * self.floor_scale
    }

    /// The [`SsiDomain::floor_scale`] that names an accounting floor of
    /// `metres` under `band` — the inverse of [`SsiDomain::floor`].
    ///
    /// A caller whose premise is about a **width** states the width. A
    /// literal multiplier does not: the floor is `SSI_FLOOR · band.zero()
    /// · floor_scale`, so one literal names a different width at every ε,
    /// and a fixture placed by literal is placed only at the tolerance
    /// whoever wrote it happened to run.
    #[must_use]
    pub fn floor_scale_for(metres: f64, band: Band) -> f64 {
        metres / (SSI_FLOOR * band.zero())
    }

    /// The seeding floor, in meters — a fraction of the **extent**,
    /// not of ε.
    ///
    /// A seed only has to land in Newton's basin, whose size is set by
    /// the geometry's curvature, not by the tolerance. Tying the seed
    /// floor to ε would refine the seeding tree to 1e-9 m cells and
    /// produce millions of seeds for the same branches; tying it to the
    /// extent produces a handful per branch, which is what seeding is
    /// for. The **accounting** floor stays tied to ε (C3's "a named
    /// constant tied to ε") because that one is a proof obligation.
    fn seed_floor(&self) -> f64 {
        SSI_SEED_FLOOR * self.extent
    }
}

/// Fit a marched polyline into a cubic NURBS carrier and its pcurves,
/// on **one shared parameter** (the OQ4 contract).
///
/// The 3-D curve's own chord-length parameters are computed once and
/// then *given* to the pcurve interpolations, so `P(t)` and `C(t)` are
/// the same `t` by construction rather than by coincidence. That is
/// exactly what PR 6's cache door checks
/// (`|S(P(t)) − C(t)| ≤ ε`), and it is why the ℝ⁴ trace discharges OQ4
/// without any re-plumbing.
fn fit_branch(
    points: &[Point3<f64>],
    charts: Option<ChartSamples<'_>>,
) -> Result<FittedBranch, SsiError> {
    if points.len() > SSI_MAX_FIT_SAMPLES {
        return Err(SsiError::FitSampleBudget {
            samples: points.len(),
            budget: SSI_MAX_FIT_SAMPLES,
        });
    }
    let params = NurbsCurve3::<f64>::chord_parameters(points)?;
    // **Interpolation, not approximation, and the reason is the
    // marcher's step rule.** The stepper already chose its spacing so
    // that a cubic through the samples is within ε of the locus
    // between them (`SSI_STEP_DEVIATION`), so the samples are at the
    // density a cubic needs — running the A9.10 removal loop on top of
    // that spends O(n²) work per removal attempt to rediscover a
    // structure the step rule already picked, and on a tight loop at
    // ε = 1e-9 that is the operation's whole runtime. Cache *shape* is
    // an f64 selection decision (C6), and this is the selection: the
    // certificate is what governs, and it is computed against the
    // curve that comes out either way. Compacting a rung-3 carrier
    // through `approximate_with_params` (which exists, and takes the
    // same shared parameters) is a profiled optimization for a later
    // PR, not a semantic change.
    let carrier = NurbsCurve3::<f64>::interpolate_with_params(points, SSI_FIT_DEGREE, &params)?;
    let (mut pa, mut pb) = (None, None);
    if let Some((a, b)) = charts {
        // Pcurves interpolate the traced parameter samples ON THE
        // CARRIER'S PARAMETERS. Interpolation (not approximation) so
        // the map is exact at every sample; compaction of a pcurve is
        // an optimization, and the certificate is what governs.
        pa = Some(NurbsCurve2::<f64>::interpolate_with_params(
            a,
            SSI_FIT_DEGREE,
            &params,
        )?);
        pb = Some(NurbsCurve2::<f64>::interpolate_with_params(
            b,
            SSI_FIT_DEGREE,
            &params,
        )?);
    }
    Ok((carrier, pa, pb))
}

/// The ℝ³ box chain of a certified branch — the tubes the accounting
/// pass consumes.
fn branch_tubes(branch: &SsiBranch) -> Vec<Box3> {
    let Curve3::Nurbs(ref c) = branch.carrier else {
        return Vec::new();
    };
    certify::tube_boxes(c, branch.certificate.tube_radius)
}

/// **cylinder × sphere** — the ℝ³ implicit-pair arm (2×3 SVD, module
/// docs). Shape (iv)'s planted small loop lives here.
///
/// # Errors
///
/// Any [`SsiError`]; in particular
/// [`SsiError::ExhaustivenessInconclusive`] when the domain cannot be
/// proved exhausted at the floor, and [`SsiError::TransversalityBand`]
/// when the pair is (near-)tangent along the locus.
pub fn cylinder_sphere_ssi(
    a: &Surface<f64>,
    b: &Surface<f64>,
    domain: SsiDomain,
    band: Band,
) -> Result<SsiOutcome, SsiError> {
    match (a, b) {
        (Surface::Cylinder { .. }, Surface::Sphere { .. })
        | (Surface::Sphere { .. }, Surface::Cylinder { .. }) => {}
        _ => {
            return Err(SsiError::WrongLane {
                expected: "a cylinder and a sphere in ℝ³ on their implicit pair",
            });
        }
    }
    // ---- C5's rule: the within-pair degeneracy trilean runs BEFORE
    // any rung. A sphere is tangent to a cylinder when the distance
    // from its centre to the cylinder's surface equals its radius —
    // |d − r| = R for a sphere resting against the wall, d + r = R for
    // a cylinder inscribed in it (the coaxial equal-radius case). Both
    // are C7 constructions, and classifying them here rather than
    // discovering them mid-march is not an optimization: near a
    // tangency the exhaustiveness subdivision's surviving set is a
    // three-dimensional shell rather than a curve, so a marcher that
    // "found out later" would first exhaust its cell budget. ----
    let (sphere, cyl) = match (a, b) {
        (Surface::Sphere { .. }, Surface::Cylinder { .. }) => (a, b),
        _ => (b, a),
    };
    let (
        Surface::Sphere { center, radius, .. },
        &Surface::Cylinder {
            origin,
            axis,
            radius: r,
            ..
        },
    ) = (sphere, cyl)
    else {
        return Err(SsiError::WrongLane {
            expected: "a cylinder and a sphere in ℝ³ on their implicit pair",
        });
    };
    let (big_r, c) = (*radius, *center);
    let q = c - origin;
    let d = (q - axis * q.dot(axis)).norm();
    let tangency = ((d - r).abs() - big_r).abs().min((d + r - big_r).abs());
    match crate::dihedral::decide("ssi_cs_tangency", Margin::of(tangency), band) {
        Ok(geom_core::Sign::Positive | geom_core::Sign::Negative) => {}
        Ok(geom_core::Sign::Zero) => {
            return Err(SsiError::TransversalityBand {
                sin_theta: 0.0,
                arm: big_r.min(r),
                sigma_min: 0.0,
            });
        }
        Err(diag) => return Err(SsiError::Escalated(diag)),
    }

    let sys = ImplicitPairR3 { a, b };
    let slab = domain.slab();

    // ---- seeds: the subdivision, asked for its seeding duty ----
    let seeds = exhaust::seed_r3(a, b, slab, domain.seed_floor())?;
    let seed_count = seeds.len() as u32;

    // ---- march, fit, certify ----
    let ctx = MarchContext::<3> {
        domain: [
            [slab.x.lo(), slab.x.hi()],
            [slab.y.lo(), slab.y.hi()],
            [slab.z.lo(), slab.z.hi()],
        ],
        extent: domain.extent,
        tol: MarchTol::from_band(band),
        max_steps: SSI_MAX_STEPS,
    };
    let mut branches: Vec<SsiBranch> = Vec::new();
    let mut tubes: Vec<Box3> = Vec::new();
    for seed in seeds.iter() {
        // Dedup against where the seed LANDS, not where it starts.
        //
        // A cell centre is not on the locus — Newton moves it there,
        // and it can move it a long way. Testing the raw centre against
        // the tubes therefore misses the case that matters: a seed
        // outside every tube whose refined landing point is squarely
        // inside one, which would re-march a branch already found and
        // hand back a duplicate `SsiBranch` (two carriers for one
        // component, and an exhaustiveness receipt that double-counts
        // the same tube). Refine first, then test.
        let state = [seed.x, seed.y, seed.z];
        let landed = march::newton_refine::<2, 3, _>(&sys, state, ctx.tol);
        let probe = landed.map_or(*seed, |x| Point3::new(x[0], x[1], x[2]));
        if tubes
            .iter()
            // The band, not the marcher: this box decides whether two
            // seeds name one component, and the tubes it gates feed the
            // exhaustiveness accounting. It is a proof-relevant length,
            // so it is stated where every other proof-relevant length
            // in this door is stated.
            .any(|t| Box3::around(probe, band.zero()).contained_in(*t))
        {
            continue;
        }
        let trace = match march_both::<2, 3, _>(&sys, state, ctx, StepperMode::Realized, band) {
            Ok(t) => t,
            // A seed that will not settle is not a branch; the
            // subdivision's accounting pass is what decides whether
            // that was a miss. Every other refusal propagates.
            Err(SsiError::SeedRefinementFailed { .. }) => continue,
            Err(e) => return Err(e),
        };
        let branch = finish_r3(&sys, &trace, a, b, &domain, ctx.tol, band)?;
        tubes.extend(branch_tubes(&branch));
        branches.push(branch);
    }

    // ---- accounting: the same subdivision, asked for its proof duty.
    // An empty `tubes` here means nothing was found, so nothing is
    // proved. ----
    let exhaustiveness = exhaust::account_r3(a, b, slab, &tubes, domain.floor(band))?;
    Ok(SsiOutcome {
        branches,
        exhaustiveness,
        seeds: seed_count,
    })
}

/// **The certifying seam.** A carrier may not cross from the untrusted
/// generator into the certificate unless the tolerance it was generated
/// at is the run band's own. Returns the receipt on success.
///
/// This is the runtime counterpart of the signature rule: the doors
/// derive their generator tolerance from the band and cannot reach this
/// refusal, but a future edit to a door *can*, and that edit is exactly
/// the divergence S25 is about. A door is maintained by someone, and a
/// sentence in a doc comment does not stop them.
fn seam_tol(tol: MarchTol, band: Band) -> Result<f64, SsiError> {
    if tol != MarchTol::from_band(band) {
        return Err(SsiError::MarchTolMismatch {
            marched: tol.meters(),
            band_zero: band.zero(),
        });
    }
    Ok(tol.meters())
}

fn finish_r3(
    sys: &ImplicitPairR3<'_>,
    trace: &Trace<3>,
    a: &Surface<f64>,
    b: &Surface<f64>,
    domain: &SsiDomain,
    tol: MarchTol,
    band: Band,
) -> Result<SsiBranch, SsiError> {
    let march_tol = seam_tol(tol, band)?;
    let points = trace_points::<2, 3, _>(sys, trace);
    let (carrier, _, _) = fit_branch(&points, None)?;
    let arm = crate::implicit::curvature_lever_arm(a, points[0])
        .min(crate::implicit::curvature_lever_arm(b, points[0]))
        .min(domain.extent);
    let cert = certify::certify_branch(
        &carrier,
        None,
        &SsiOperand::Analytic(a),
        &SsiOperand::Analytic(b),
        TubeScale::split(arm, domain.extent),
        band,
    )?;
    let params = carrier.domain();
    let witness = certify::witness(&carrier);
    Ok(SsiBranch {
        carrier: Curve3::Nurbs(std::sync::Arc::new(carrier)),
        params,
        end: trace.end,
        certificate: cert,
        witness,
        pcurve_a: None,
        pcurve_b: None,
        min_transversality: trace.min_transversality,
        march_tol,
    })
}

/// **plane × NURBS wall** — the ℝ⁴ parametric×parametric arm (3×4 SVD,
/// module docs). **Retired 2026-07-31 (M5 PR 7b)**: marches, fits, and
/// proves all three C2 limbs — limb 2 through the tensor-product
/// Bernstein composite of `S(P(t)) − C(t)`
/// (`geom_core::spline::compose::tensor`; module docs carry the
/// retirement record, the C5 table's `(Plane, Nurbs)` note carries it
/// where a caller reads it). PR 7b turned the arm on by deleting
/// nothing: the refusal path IS the certification path, minus the
/// refusal.
///
/// # Errors
///
/// Any [`SsiError`]. Notably: [`SsiError::Escalated`] naming
/// `ssi_hull_sup_chart` when the fitted pair's real between-samples
/// deviation lands in the band (an inflected wall can genuinely earn
/// this — module docs), and [`SsiError::FitSampleBudget`] at
/// tolerances whose sample demand exceeds the fit budget.
pub fn plane_nurbs_ssi(
    plane: &Surface<f64>,
    wall: &NurbsSurface<f64>,
    domain: SsiDomain,
    band: Band,
) -> Result<SsiOutcome, SsiError> {
    let Surface::Plane {
        origin: p0,
        normal,
        u_ref,
    } = *plane
    else {
        return Err(SsiError::WrongLane {
            expected: "a plane and a NURBS surface traced in ℝ⁴ on their charts",
        });
    };
    let half = domain.half_extent;
    let Some(chart_a) = Chart::plane_of(plane, (-half, half), (-half, half)) else {
        return Err(SsiError::WrongLane {
            expected: "a plane and a NURBS surface traced in ℝ⁴ on their charts",
        });
    };
    let chart_b = Chart::Nurbs(wall);
    // The march domain is the two charts' own named windows — the
    // plane's from the caller's extent (a plane is unbounded, so a
    // window is a caller obligation, never a guess), the wall's from
    // its knot domain.
    let ((pu, pv), (ud, vd)) = (chart_a.domain(), chart_b.domain());
    let sys = ParametricPairR4 {
        a: chart_a,
        b: chart_b,
    };
    let root = UvRect { u: ud, v: vd };

    // The chart floors: meters ÷ a certified chart-speed bound, so a
    // floor stated in meters means the same thing in both lanes.
    let nb = NurbsBoxes::new(wall);
    let du = nb.deriv_box(ud.0, ud.1, vd.0, vd.1, true);
    let dv = nb.deriv_box(ud.0, ud.1, vd.0, vd.1, false);
    let mag =
        |b: Box3| (b.x.mag() * b.x.mag() + b.y.mag() * b.y.mag() + b.z.mag() * b.z.mag()).sqrt();
    let speed = nan_propagating_max(mag(du), mag(dv));
    // A speed OUTSIDE the positive-finite class can never translate a
    // floor: `floor / ∞` is exactly zero — a floor no cell can ever
    // reach — so a non-finite speed would let the sweep run to its
    // cell budget and answer in this guard's place with the wrong
    // diagnosis. Positive finite is NECESSARY, not sufficient: a
    // finite speed of ~1e150 passes here and drives the translated
    // floors to ~1e-152, where the budget still answers — the
    // finite-but-unusable window is issue 1238's.
    if !speed.is_finite() {
        return Err(SsiError::UnsupportedCertificate {
            what: "the NURBS wall's certified chart speed is not finite — its \
                   derivative bound overflowed or is poison — so no floor in \
                   meters can be translated into its parameter domain",
        });
    }
    if speed <= 0.0 {
        return Err(SsiError::UnsupportedCertificate {
            what: "the NURBS wall's certified chart speed is zero, so no floor in \
                   meters can be translated into its parameter domain",
        });
    }

    // ---- seeds ----
    let seeds = exhaust::seed_chart_plane(wall, p0, normal, root, domain.seed_floor() / speed)?;
    let seed_count = seeds.len() as u32;

    let v_ref = normal.cross(u_ref);
    let to_plane_chart = |p: Point3<f64>| {
        let q = p - p0;
        (q.dot(u_ref), q.dot(v_ref))
    };
    let ctx = MarchContext::<4> {
        domain: [[pu.0, pu.1], [pv.0, pv.1], [ud.0, ud.1], [vd.0, vd.1]],
        extent: domain.extent,
        tol: MarchTol::from_band(band),
        max_steps: SSI_MAX_STEPS,
    };
    let mut branches: Vec<SsiBranch> = Vec::new();
    let mut tubes: Vec<UvRect> = Vec::new();
    for (u, v) in seeds.iter() {
        if tubes
            .iter()
            .any(|t| *u >= t.u.0 && *u <= t.u.1 && *v >= t.v.0 && *v <= t.v.1)
        {
            continue;
        }
        let p = wall.eval(*u, *v);
        let (pu, pv) = to_plane_chart(p);
        let state = [pu, pv, *u, *v];
        let trace = match march_both::<3, 4, _>(&sys, state, ctx, StepperMode::Realized, band) {
            Ok(t) => t,
            Err(SsiError::SeedRefinementFailed { .. }) => continue,
            Err(e) => return Err(e),
        };
        let branch = finish_r4(&sys, &trace, plane, wall, &domain, ctx.tol, band)?;
        if let Some(ref pc) = branch.pcurve_b {
            // The tube the CERTIFICATE earned, in chart units — the
            // same region limb 3 proved one-arc-ness over.
            let pad = branch.certificate.tube_radius / speed;
            tubes.extend(pcurve_windows(pc, pad, pad));
        }
        branches.push(branch);
    }

    let exhaustiveness =
        exhaust::account_chart_plane(wall, p0, normal, root, &tubes, domain.floor(band) / speed)?;
    Ok(SsiOutcome {
        branches,
        exhaustiveness,
        seeds: seed_count,
    })
}

/// The chart lane's uniqueness tube, as the accounting pass sees it:
/// **one padded rectangle per span** of the pcurve.
///
/// Per span, deliberately — not one rectangle around the whole pcurve.
/// The accounting pass counts a cell as "accounted" when the cell lies
/// inside a tube, and a single bounding box of a curve that wanders
/// across its domain contains vast regions the curve never enters. That
/// would silently account for cells nobody proved anything about, which
/// is exactly the failure this whole obligation exists to prevent. The
/// per-span chain is also precisely the region limb 3's chart form
/// proved a single arc over, so the two agree by construction.
fn pcurve_windows(p: &NurbsCurve2<f64>, pad_u: f64, pad_v: f64) -> Vec<UvRect> {
    let coords = p.ring_coords();
    let kv = p.knots();
    let mut out = Vec::new();
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and span validation are one step.
        let Some(span) = kv.span(index) else { continue };
        let hu = geom_core::spline::hull::span_hull(kv, &coords[0], span);
        let hv = geom_core::spline::hull::span_hull(kv, &coords[1], span);
        if hu.is_poison() || hv.is_poison() {
            // A window this pass cannot bound is not banked. Dropping
            // it only ever SHRINKS the accounted set, so the accounting
            // pass gets strictly harder: the failure direction is the
            // typed refusal, never a false `Ok`. A tube is a claim, and
            // an unbounded hull supports none.
            continue;
        }
        out.push(UvRect {
            u: (hu.lo() - pad_u, hu.hi() + pad_u),
            v: (hv.lo() - pad_v, hv.hi() + pad_v),
        });
    }
    out
}

fn finish_r4(
    sys: &ParametricPairR4<'_>,
    trace: &Trace<4>,
    plane: &Surface<f64>,
    wall: &NurbsSurface<f64>,
    domain: &SsiDomain,
    tol: MarchTol,
    band: Band,
) -> Result<SsiBranch, SsiError> {
    let march_tol = seam_tol(tol, band)?;
    let points = trace_points::<3, 4, _>(sys, trace);
    let chart_a: Vec<geom_core::Point2<f64>> = trace
        .states
        .iter()
        .map(|s| geom_core::Point2::new(s[0], s[1]))
        .collect();
    let chart_b: Vec<geom_core::Point2<f64>> = trace
        .states
        .iter()
        .map(|s| geom_core::Point2::new(s[2], s[3]))
        .collect();
    let (carrier, pa, pb) = fit_branch(&points, Some((&chart_a, &chart_b)))?;
    let cert = certify::certify_branch(
        &carrier,
        pb.as_ref(),
        &SsiOperand::Analytic(plane),
        &SsiOperand::Nurbs(wall),
        TubeScale::uniform(domain.extent),
        band,
    )?;
    let params = carrier.domain();
    let witness = certify::witness(&carrier);
    Ok(SsiBranch {
        carrier: Curve3::Nurbs(std::sync::Arc::new(carrier)),
        params,
        end: trace.end,
        certificate: cert,
        witness,
        pcurve_a: pa,
        pcurve_b: pb,
        min_transversality: trace.min_transversality,
        march_tol,
    })
}

/// The ℝ⁴ trace's fitted product **without a certificate** — the OQ4
/// demonstration's door, and nothing else's.
///
/// Returns the 3-D carrier and both pcurves, fitted on **one shared
/// parameterization** (the carrier's own chord parameters, handed to
/// the pcurve fits), which is the parameter-identity contract PR 6
/// ratified for caches: `P(t)` and `C(t)` are the same `t` by
/// construction rather than by coincidence.
///
/// **Nothing may build a body from this.** It is uncertified by
/// definition — the certified product of this module is [`SsiBranch`],
/// which cannot be constructed without [`SsiCertificate`]. This entry
/// exists so the OQ4 identity can be *shown* independently of any
/// certificate, and it is the certified arm's comparison substrate:
/// PR 7b's acceptance rows dense-scan the triple it returns against
/// the tensor composite bound that retired limb 2 (the fixture role
/// the PR 7 refusal reserved for it).
///
/// **The only door that names the generator's tolerance separately.**
/// `march_tol` is the marcher's step tolerance in meters; every
/// certifying door derives its own from `band` and has no such
/// parameter. This door takes a bare `f64` and mints the private
/// `MarchTol` itself, which is what keeps a decoupled tolerance
/// unmintable anywhere else — including inside a certifying door, whose
/// maintainer is the caller who would otherwise reach for it.
///
/// Naming it here is what lets the marcher be measured against itself
/// at a tolerance the run is not banded at, which is the one legitimate
/// reason the two numbers ever differ. A carrier so generated may still
/// be handed to [`certify_rung3`]; it certifies at the `Band` like any
/// other, so a decoupled march can only cost carrier quality, never
/// buy a weaker certificate.
///
/// # Errors
///
/// Any [`SsiError`] the trace or the fit can produce, plus
/// [`SsiError::InvalidMarchTol`] when `march_tol` is not a length.
pub fn trace_plane_nurbs_uncertified(
    plane: &Surface<f64>,
    wall: &NurbsSurface<f64>,
    seed_uv: (f64, f64),
    domain: SsiDomain,
    march_tol: f64,
    band: Band,
) -> Result<TracedTriple, SsiError> {
    let tol = MarchTol::decoupled(march_tol)?;
    let Surface::Plane {
        origin: p0,
        normal,
        u_ref,
    } = *plane
    else {
        return Err(SsiError::WrongLane {
            expected: "a plane and a NURBS surface traced in ℝ⁴ on their charts",
        });
    };
    let half = domain.half_extent;
    let Some(chart_a) = Chart::plane_of(plane, (-half, half), (-half, half)) else {
        return Err(SsiError::WrongLane {
            expected: "a plane and a NURBS surface traced in ℝ⁴ on their charts",
        });
    };
    let chart_b = Chart::Nurbs(wall);
    let ((pu, pv), (ud, vd)) = (chart_a.domain(), chart_b.domain());
    let sys = ParametricPairR4 {
        a: chart_a,
        b: chart_b,
    };
    let ctx = MarchContext::<4> {
        domain: [[pu.0, pu.1], [pv.0, pv.1], [ud.0, ud.1], [vd.0, vd.1]],
        extent: domain.extent,
        tol,
        max_steps: SSI_MAX_STEPS,
    };
    let v_ref = normal.cross(u_ref);
    let p = wall.eval(seed_uv.0, seed_uv.1);
    let q = p - p0;
    let state = [q.dot(u_ref), q.dot(v_ref), seed_uv.0, seed_uv.1];
    let trace = march_both::<3, 4, _>(&sys, state, ctx, StepperMode::Realized, band)?;
    let points = trace_points::<3, 4, _>(&sys, &trace);
    let chart_a_pts: Vec<geom_core::Point2<f64>> = trace
        .states
        .iter()
        .map(|s| geom_core::Point2::new(s[0], s[1]))
        .collect();
    let chart_b_pts: Vec<geom_core::Point2<f64>> = trace
        .states
        .iter()
        .map(|s| geom_core::Point2::new(s[2], s[3]))
        .collect();
    let (carrier, pa, pb) = fit_branch(&points, Some((&chart_a_pts, &chart_b_pts)))?;
    match (pa, pb) {
        (Some(a), Some(b)) => Ok((carrier, a, b)),
        _ => Err(SsiError::UnsupportedCertificate {
            what: "the ℝ⁴ trace must produce both pcurves",
        }),
    }
}

/// Certify an already-fitted rung-3 carrier against an operand pair —
/// the public door onto [`certify::certify_branch`].
///
/// Exists for two consumers: PR 9's at-rest re-certification (a cache
/// is re-derived, never trusted), and the adversarial suite, which
/// plants a corruption in a carrier and requires **each limb to refuse
/// separately**. Both need the certificate reachable without re-running
/// a march.
///
/// # Errors
///
/// As [`certify::certify_branch`].
///
/// **There is no `eps` parameter.** The certificate's own floors — the
/// tube ladder's, in particular — are stated in `band.zero()`, which
/// *is* the run tolerance, so the tolerance a branch is certified at
/// cannot differ from the one its trileans decide against.
///
/// That identity is **contingent on `band` being a linear band**
/// ([`Band::linear`]), whose `zero()` is the run's ε in meters, stored
/// unmodified. An angular band ([`Band::angular_at`]) carries ε/r in
/// radians, and this door would then floor the tube ladder in radians.
/// Every caller passes a linear band today; the door is `pub`, so this
/// is a stated contract, not a proof.
pub fn certify_rung3<T: geom_core::Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    pcurve_b: Option<&NurbsCurve2<T>>,
    a: &SsiOperand<'_, T>,
    b: &SsiOperand<'_, T>,
    scale: TubeScale<T>,
    band: Band,
) -> Result<SsiCertificate<T>, SsiError> {
    certify::certify_branch(carrier, pcurve_b, a, b, scale, band)
}

/// The idealized stepper's trace of an analytic pair from an explicit
/// seed — the differential suite's entry (PERF-PLAN §4.4). Returns the
/// polyline, not a certified branch: the *spec* is what the locus is,
/// and the suite compares it against the realized carrier.
///
/// # Errors
///
/// Any [`SsiError`] the march can produce.
pub fn idealized_trace_r3(
    a: &Surface<f64>,
    b: &Surface<f64>,
    seed: Point3<f64>,
    domain: SsiDomain,
    band: Band,
) -> Result<(Vec<Point3<f64>>, BranchEnd), SsiError> {
    let sys = ImplicitPairR3 { a, b };
    let slab = domain.slab();
    let ctx = MarchContext::<3> {
        domain: [
            [slab.x.lo(), slab.x.hi()],
            [slab.y.lo(), slab.y.hi()],
            [slab.z.lo(), slab.z.hi()],
        ],
        extent: domain.extent,
        tol: MarchTol::from_band(band),
        max_steps: SSI_MAX_STEPS,
    };
    let trace = march_both::<2, 3, _>(
        &sys,
        [seed.x, seed.y, seed.z],
        ctx,
        StepperMode::Idealized,
        band,
    )?;
    let pts = trace_points::<2, 3, _>(&sys, &trace);
    Ok((pts, trace.end))
}

/// `max` that PROPAGATES NaN — `f64::max` returns the non-NaN operand,
/// so a lone poisoned fold input would be dropped before any guard
/// with an `is_finite`/`is_nan` arm could see it.
///
/// At its one call site (the seeding guard's chart-speed fold) the
/// difference from `f64::max` is defensive rather than reachable
/// today: a poisoned derivative box needs a zero-touching weight hull
/// or a malformed net — both refused at construction — and an
/// OVERFLOWED box saturates its `mag` to `+∞`, which both folds hand
/// to the same not-finite refusal. The pin below is therefore on this
/// helper by name; the reachability argument lives here so that a
/// future producer of one-sided poison (a new box source, a widened
/// constructor) finds the fold already stated as load-bearing.
fn nan_propagating_max(a: f64, b: f64) -> f64 {
    if a.is_nan() || b.is_nan() {
        f64::NAN
    } else {
        a.max(b)
    }
}

#[cfg(test)]
#[allow(clippy::panic)]
mod fold_tests {
    use super::nan_propagating_max;

    /// Red under the exact corruption a review executed: reverting the
    /// fold to `f64::max`, which drops a lone NaN — every ssi row
    /// stayed green under that revert, so the fold's contract gets its
    /// own executable pin.
    #[test]
    fn the_chart_speed_fold_propagates_a_lone_nan() {
        assert!(nan_propagating_max(f64::NAN, 1.0).is_nan());
        assert!(nan_propagating_max(1.0, f64::NAN).is_nan());
        assert!(nan_propagating_max(f64::NAN, f64::NAN).is_nan());
        assert_eq!(nan_propagating_max(1.0, 2.0), 2.0);
        assert_eq!(nan_propagating_max(f64::INFINITY, 1.0), f64::INFINITY);
    }
}

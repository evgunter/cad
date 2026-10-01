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
//! - [`enclose`] supplies every certified bound, in certification arithmetic only.
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
//! neighbor cell's polynomial extension into the bound or refuses,
//! and far-from-origin operands hit the projection and exhaustiveness
//! machinery's own representation floors — every one of these is a
//! loud, typed refusal, never a silent miscertification.
//!
//! **Why cylinder×sphere and not cylinder×torus** (the spec's own "or
//! equivalent"): both operands' polynomial composites convert to meters
//! **exactly** (`÷2R`), so limb 2 certifies with no invented scale
//! factor. The torus's composite is quartic (m⁴) and its conversion
//! back to meters needs a certified reciprocal of `A + 4Rρ`, which
//! needs a square root certification arithmetic deliberately does not have. Shipping
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
use geom_core::Bounds;
use geom_core::{
    Band, Indeterminate, KERNEL_LIMIT_LAST_RESORT, KERNEL_LIMIT_RECOURSE, Margin, Point3, Real,
    SizedPass,
};

use crate::certify::CertCheck;
use crate::recourse::{
    Reading, Refused, RefusedArm, SizedDecision, StoredDefinite, Unsized, defect_ending,
};

pub use certify::{SSI_CERT_SPANS, SSI_TUBE_RADIUS, SsiCertificate, SsiLimb, SsiTube};
pub use exhaust::{
    ExhaustLane, Exhaustiveness, ExhaustivenessRefusal, FloorFault, FloorKind, FloorRefusal,
    SSI_FLOOR, SSI_MAX_CELLS, SSI_SEED_FLOOR,
};
pub use march::{
    BranchEnd, ReachBound, SSI_IDEALIZED_STEP, SSI_NEWTON_ITERS, SSI_NEWTON_TOL,
    SSI_QUADRIC_NOISE_ULPS, SSI_SETTLE_MAX, SSI_SPLINE_NOISE_ULPS, SSI_STEP_DEVIATION,
    SSI_STEP_MAX, SettlingRefusal, StepFault, StepperMode,
};

use enclose::{Box3, NurbsBoxes};
use exhaust::{RateClause, SweepFloor, UvRect};
use march::{MarchContext, MarchTol, Readout, Trace, march_both, trace_points};
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
    /// A NURBS surface: certified foot points and chart hulls, carried
    /// with the chart speeds its tube pad crosses into chart units by.
    Nurbs(ChartedNurbs<'a, T>),
}

impl<'a, T: geom_core::CertifiedBounds> SsiOperand<'a, T> {
    /// A NURBS operand, its chart speeds minted ([`ChartedNurbs::mint`]).
    ///
    /// # Errors
    ///
    /// [`SsiError::ChartSpeed`], naming the axis whose speed is zero or
    /// not finite.
    pub fn nurbs(surface: &'a NurbsSurface<T>) -> Result<Self, SsiError> {
        ChartedNurbs::mint(surface).map(Self::Nurbs)
    }
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
    /// The lever arm the transversality margin is stated over, in
    /// meters.
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
///
/// No `PartialEq`: [`ExhaustLane`] carries a [`SupSpeed`](geom_core::SupSpeed),
/// which has none by the `Real` surface's rule that a tagged rate is
/// never compared without `get()`. Deriving one here would have to
/// compare rates, and the payloads it would compare include a
/// deliberate `f64::NAN` ([`Self::CertificateLimb`]'s `value`), which
/// no derived equality can call equal to itself.
#[derive(Clone, Debug)]
pub enum SsiError {
    /// The transversality margin `sin θ · arm` landed in the sliver
    /// band along the candidate locus. This is the C7 regime
    /// (`TangentIntersection`, M5 PR 9): tangential contact is a
    /// construction, not something to march through, and in-band
    /// contact is a genuine sliver of the operand pair (F6). We never
    /// desingularize — Hoffmann §6.5 is deliberately not adopted.
    ///
    /// The refused arm of the march's transversality decision
    /// (`ssi_transversality`), which passes on a positive sign; it ends
    /// by [`CertCheck::Transversality`], as limb 3 and the edge
    /// certifier's transversality do ([`SsiError::ending`]).
    TransversalityBand {
        /// The sine of the angle between the operand normals.
        sin_theta: f64,
        /// The lever arm folded against it, in meters.
        arm: f64,
        /// Hoffmann's own σ₂ signal at the same state (diagnostic).
        sigma_min: f64,
        /// The verdict on `sin θ · arm`, with the margin it classified.
        verdict: Refused,
    },
    /// A cylinder × sphere pair is tangent somewhere to tolerance:
    /// externally (`|d − r| = R`, the sphere against the wall) or
    /// internally (`d + r = R`, the cylinder inside the sphere touching
    /// it — around a whole circle when coaxial with `r = R`, at one point
    /// off-axis, where the pair still crosses elsewhere, as Viviani's
    /// figure-eight does). Decided by the pair's own tangency gap before
    /// any rung runs (`ssi_cs_tangency`, C5's within-pair degeneracy
    /// rule): the C7 regime, like [`SsiError::TransversalityBand`],
    /// reached by a different decision whose margin is the gap from the
    /// nearer tangent pose.
    PairTangent {
        /// The verdict on the gap, with the margin it classified.
        verdict: Refused,
    },
    /// An operand's stored datum is not a finite number, so the operand
    /// describes no surface. Refused at the door, as that operand's own
    /// fault, before any sweep or march reads it.
    OperandNotFinite {
        /// Which operand, in the door's own words.
        operand: &'static str,
        /// The first datum that is not finite.
        datum: OperandDatum,
    },
    /// The subdivision reached its named floor with a cell it could
    /// neither exclude nor account for. **This is the never-silence
    /// obligation firing**: there may be a branch in that cell and we
    /// decline to pretend otherwise.
    ExhaustivenessInconclusive(ExhaustivenessRefusal),
    /// The subdivision's floor is one its domain cannot resolve: not a
    /// positive finite width, or narrower than the finest cell bisection
    /// can cut there. Refused where the floor is minted, before any
    /// sweep runs, so the cell budget never answers for a floor no cell
    /// can reach.
    FloorUnresolvable(FloorRefusal),
    /// The march cannot settle its states well inside the tolerance: the
    /// coordinates its residual is read in are too coarse where they
    /// reach furthest. Refused where the march's tolerance is minted,
    /// before any march, so Newton never answers that the march lost its
    /// branch in the scale's place.
    SettlingUnresolvable(SettlingRefusal),
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
        /// The march speed it was minted at: metres per unit of the
        /// march parameter.
        speed: f64,
    },
    /// The stepper's step, minted from the march speed at a state, is
    /// not one the trace can take: the speed converts no step into
    /// metres, or the step is not finite, or it does not move the
    /// state. Refused where the step is minted, naming the speed.
    StepUnusable {
        /// Which stepper.
        mode: &'static str,
        /// The march speed at the state: metres per unit of the march
        /// parameter.
        speed: f64,
        /// What is wrong with the step.
        fault: StepFault,
    },
    /// Newton refinement would not settle a seed onto the surface pair:
    /// a seed outside every basin, or poisoned arithmetic. The seed is
    /// then no branch, which the accounting pass decides was or was not
    /// a miss.
    SeedRefinementFailed {
        /// Which stepper.
        mode: &'static str,
    },
    /// A step from a state already on the locus would not settle back
    /// onto the surface pair, so the march lost the branch it was
    /// tracing.
    StepRefinementFailed {
        /// Which stepper.
        mode: &'static str,
        /// The step that was taken, in metres.
        step_meters: f64,
    },
    /// The trace arrived back at its seed running the **wrong way** —
    /// perpendicular to, or reversed from, the direction it left in.
    /// That is a cusp or a self-crossing of the candidate locus, which
    /// is a degenerate operand pair, not a loop to close.
    SelfCrossingLocus {
        /// The cosine of the angle between the returning and seed
        /// tangents (≈ +1 for a genuine closure).
        cos_phi: f64,
        /// The branch's arc length in meters, the lever arm it was
        /// measured over.
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
    /// Limb 3's transversality is not certified clear of the zero band
    /// over the tube chain (its enclosure straddles zero, or its
    /// clearance lies inside the band): two branches pass within the
    /// band of each other. A genuine sliver (F6), not a resolution
    /// failure to retry.
    TubeStraddles {
        /// The verdict on the certified transversality clearance: a
        /// dimensionless sine-like lower bound levered by the tube
        /// scale's arm, so a length in metres (zero when the enclosure
        /// straddles).
        verdict: crate::recourse::Refused,
        /// Boxes in the chain.
        boxes: u32,
    },
    /// A certified foot point would not converge, so limb 1 has no
    /// residual to check against a NURBS operand.
    FootPointInconclusive {
        /// The schedule parameter it failed at.
        t: f64,
        /// The last distance the projection saw, in meters.
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
    /// A NURBS operand's chart cannot carry a length in metres into its
    /// parameters: a chart speed along one axis is zero or has no finite
    /// bound, or the chart is constant across the traced locus. Refused
    /// by the axis it lands on.
    ChartSpeed(ChartSpeedRefusal),
    /// Limb 3's chart tube cannot be probed at any rung: the window
    /// fact it names does not depend on the pad.
    TubeDegenerate(TubeDegeneracy),
    /// The caller routed the wrong kinds into an arm.
    WrongLane {
        /// What the arm expects.
        expected: &'static str,
    },
    /// A trace's own decision landed in the ambiguity band or poisoned
    /// (F6). It ends by the decision it names ([`TraceDecision::ending`]).
    Escalated {
        /// The decision.
        decision: TraceDecision,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// A certificate limb's trilean landed in the ambiguity band or
    /// poisoned (F6) — [`SsiError::CertificateLimb`]'s undecided
    /// sibling, ending by the same limb's decision
    /// ([`SsiLimb::check`]).
    CertificateEscalated {
        /// Which limb.
        limb: SsiLimb,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// Band construction refused.
    Band(geom_core::BandError),
    /// A knob of the caller's [`SsiDomain`] is not usable: refused at
    /// the door, before anything reads it.
    DomainUnusable {
        /// Which knob.
        field: DomainField,
        /// Its value (for the centre, the first coordinate that is not
        /// finite).
        value: f64,
    },
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
                ..
            } => write!(
                f,
                "ssi: the surfaces meet nearly tangentially along the traced locus \
                 (sin θ = {sin_theta:e}, arm = {arm:e} m, σ₂ = {sigma_min:e}): the tangency \
                 regime (TangentIntersection), not a locus to march"
            ),
            Self::PairTangent { verdict } => write!(
                f,
                "ssi: the sphere and the cylinder are tangent to each other within the \
                 tolerance (gap {:e} m from a tangent pose): the tangency regime \
                 (TangentIntersection), which the march does not pass",
                verdict.margin()
            ),
            Self::OperandNotFinite { operand, datum } => write!(
                f,
                "ssi: the {operand}'s {datum} is not a finite number, so it describes no \
                 surface and nothing was traced"
            ),
            Self::ExhaustivenessInconclusive(r) => {
                let (cell_width, floor, examined) = (r.cell_width, r.floor, r.examined);
                match r.lane {
                    ExhaustLane::R3 => write!(
                        f,
                        "ssi: exhaustiveness inconclusive on the ℝ³ lane — after \
                         {examined} cells a cell of width {cell_width:e} m at the \
                         refinement floor {floor:e} m could be neither excluded nor \
                         accounted for, so a branch may be hiding in it; the \
                         operation refuses rather than report a possibly incomplete \
                         intersection"
                    ),
                    ExhaustLane::Chart { speed } => {
                        write!(
                            f,
                            "ssi: exhaustiveness inconclusive on the chart lane — \
                             after {examined} cells a cell of width "
                        )?;
                        exhaust::write_chart_length(f, cell_width, speed, RateClause::Omit)?;
                        write!(f, " at the refinement floor ")?;
                        exhaust::write_chart_length(f, floor, speed, RateClause::Name)?;
                        write!(
                            f,
                            " could be neither excluded nor accounted for, so a branch \
                             may be hiding in it; the operation refuses rather than \
                             report a possibly incomplete intersection"
                        )
                    }
                }
            }
            Self::FloorUnresolvable(r) => write!(f, "ssi: {r}"),
            Self::SettlingUnresolvable(r) => {
                let SettlingRefusal {
                    lane,
                    reach,
                    gap,
                    bound,
                    settle,
                    tolerance,
                } = *r;
                let what = match bound {
                    ReachBound::Geometry => "geometry",
                    ReachBound::Domain => "domain",
                };
                write!(
                    f,
                    "ssi: the march can settle a state no finer than {settle:e} m, not well \
                     inside the {tolerance:e} m tolerance: "
                )?;
                match lane {
                    ExhaustLane::R3 => write!(
                        f,
                        "where the {what} reaches {reach:e} m from zero, adjacent coordinates \
                         are {gap:e} m apart"
                    ),
                    ExhaustLane::Chart { speed } => write!(
                        f,
                        "where the spline face's parameters reach {reach:e} from zero, adjacent \
                         parameters are {gap:e} apart, {:e} m at a certified chart speed of \
                         {:e} m per chart unit",
                        speed.to_meters(gap),
                        speed.get()
                    ),
                }
            }
            Self::CellBudget { budget } => write!(
                f,
                "ssi: the exhaustiveness subdivision exceeded its {budget}-cell budget \
                 — refused rather than truncated"
            ),
            Self::StepBudget { mode, budget } => write!(
                f,
                "ssi: the {mode} stepper exceeded its {budget}-step budget on one branch"
            ),
            Self::StepCollapsed {
                mode,
                step_meters,
                speed,
            } => write!(
                f,
                "ssi: the {mode} stepper's step collapsed to {step_meters:e} m at a march \
                 speed of {speed:e} m per unit of the march parameter, inside the tolerance \
                 band — no progress is possible at this ε"
            ),
            Self::StepUnusable { mode, speed, fault } => {
                let what = match fault {
                    StepFault::SpeedUnusable => "converts no step into metres",
                    StepFault::NotFinite => {
                        "gives a step that is not a finite number in the state's coordinates"
                    }
                    StepFault::DoesNotMove => {
                        "gives a step below the resolution of the state's coordinates, which \
                         does not move the trace"
                    }
                };
                write!(
                    f,
                    "ssi: the {mode} stepper's march speed of {speed:e} m per unit of the \
                     march parameter {what}, so the trace cannot be advanced"
                )
            }
            Self::SeedRefinementFailed { mode } => write!(
                f,
                "ssi: Newton refinement would not settle a {mode} seed onto the \
                 surface pair"
            ),
            Self::StepRefinementFailed { mode, step_meters } => write!(
                f,
                "ssi: a {mode} step of {step_meters:e} m from a state on the locus would \
                 not settle back onto the surface pair, so the march lost the branch it \
                 was tracing"
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
            Self::TubeStraddles { verdict, boxes } => write!(
                f,
                "ssi: over its {boxes}-box chain the uniqueness tube cannot certify the surfaces \
                 crossing clear of the tolerance band (certified clearance {:e} m), so a second \
                 branch may lie inside the tube",
                verdict.margin()
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
                "ssi: a branch marched {samples} samples against a {budget}-sample fit \
                 budget; this tolerance and curvature need more control points than the \
                 fit affords"
            ),
            Self::UnsupportedCertificate { what } => {
                write!(f, "ssi: {what}")
            }
            Self::ChartSpeed(r) => write!(f, "ssi: {}", r.what()),
            Self::TubeDegenerate(d) => write!(f, "ssi: {}", d.what()),
            Self::WrongLane { expected } => write!(
                f,
                "ssi: wrong dispatch lane — this arm traces {expected} (caller bug)"
            ),
            Self::Escalated { decision, cause } => write!(
                f,
                "ssi: whether {} is too close to call: {}",
                decision.question(),
                cause.payload()
            ),
            Self::CertificateEscalated { limb, cause } => write!(
                f,
                "ssi: the fitted carrier's {} escalated: {}",
                limb.name(),
                cause.payload()
            ),
            Self::Band(e) => write!(f, "ssi: {e}"),
            Self::DomainUnusable { field, value } => {
                let (name, must) = field.words();
                write!(
                    f,
                    "ssi: the domain's {name} reads {value:e}, which is not {must}, so the \
                     domain names no region to search"
                )
            }
            Self::InvalidMarchTol { value } => write!(
                f,
                "ssi: the marcher's step tolerance {value:e} m is not a usable length \
                 (finite and > 0)"
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

impl SsiError {
    /// The ending this refusal's decision gives it, read at `reading`
    /// (D4 ¶1 (i)), or `None` for a refusal whose decision has no
    /// ending yet (`work/ssi/ssi-refusals-whose-decision-has-no-ending.md`).
    ///
    /// `Display` renders the payload alone, as [`crate::CertifyError`]'s
    /// does; the door that reports the refusal appends this, or renders
    /// both through [`SsiError::render`]. No SSI door takes a
    /// declaration, so no ending offers one.
    #[must_use]
    pub fn ending(&self, reading: Reading) -> Option<String> {
        Some(match self {
            Self::TransversalityBand { verdict, .. } => {
                crate::certify::recourse(CertCheck::Transversality, verdict.arm(), reading)
            }
            Self::PairTangent { verdict } => PAIR_TANGENCY.recourse(verdict.arm(), reading),
            Self::Escalated { decision, cause } => decision.ending(cause, reading),
            // Each limb ends by its own decision on every arm.
            Self::CertificateEscalated { limb, cause } => {
                crate::certify::recourse(limb.check(), RefusedArm::Undecided(cause), reading)
            }
            Self::CertificateLimb { limb, .. } => {
                crate::certify::recourse(limb.check(), RefusedArm::SignCertain, reading)
            }
            Self::TubeStraddles { verdict, .. } => {
                crate::certify::recourse(SsiLimb::Tube.check(), verdict.arm(), reading)
            }
            Self::SelfCrossingLocus { .. } => {
                SELF_CROSSING.recourse(RefusedArm::SignCertain, reading)
            }
            // Only a certifying door edited to march at another tolerance
            // reaches it.
            Self::MarchTolMismatch { .. } => defect_ending(reading).to_owned(),
            Self::InvalidMarchTol { .. } => {
                "Recourse: name a march tolerance that is a positive finite length".to_owned()
            }
            Self::OperandNotFinite { operand, datum } => {
                format!("Recourse: give the {operand} a finite {datum}")
            }
            Self::DomainUnusable { field, .. } => {
                let (name, must) = field.words();
                format!("Recourse: give the domain a {name} that is {must}")
            }
            // A floor too fine for the domain is the geometry's scale,
            // decided exactly: no band, so no tolerance to name. One that
            // is not a length is the caller's knobs.
            Self::FloorUnresolvable(r) => r.ending(reading),
            // The same decision as a floor too fine for its domain: the
            // geometry's scale, decided exactly.
            Self::SettlingUnresolvable(r) => {
                let scale = match (r.lane, r.bound) {
                    (ExhaustLane::R3, ReachBound::Geometry) => exhaust::R3_SCALE,
                    (ExhaustLane::R3, ReachBound::Domain) => DOMAIN_SCALE,
                    (ExhaustLane::Chart { .. }, _) => exhaust::CHART_SCALE,
                };
                scale.recourse(RefusedArm::SignCertain, reading)
            }
            // The same decision, and so the same ending, as the edge
            // lane's refusal of the same fact.
            Self::ChartSpeed(r) => {
                crate::certify::recourse(r.check(), RefusedArm::SignCertain, reading)
            }
            // A step that cannot be taken or that collapses into the
            // band is the operands' scale against the domain the caller
            // named, decided without a margin to name. A march speed
            // that is not positive and finite is no input's: the chart
            // mint refuses a degenerate chart before any march.
            Self::StepCollapsed { .. }
            | Self::StepUnusable {
                fault: StepFault::NotFinite | StepFault::DoesNotMove,
                ..
            } => STEP_SCALE.recourse(RefusedArm::SignCertain, reading),
            Self::StepUnusable {
                fault: StepFault::SpeedUnusable,
                ..
            } => defect_ending(reading).to_owned(),
            // A kernel approximation limit: the user holds no lever but
            // the tolerance (D4 ¶1 (i)'s last resort).
            Self::FitSampleBudget { budget, .. } => format!(
                "Recourse: loosen the tolerance until a branch needs at most {budget} samples, \
                 {KERNEL_LIMIT_LAST_RESORT}"
            ),
            // `march_both` re-marches a trace too short for the cubic at
            // the branch's own length, so one that is still short is the
            // kernel's limit, with no lever of the caller's behind it.
            Self::Fit(FitError::TooFewPoints { .. }) => KERNEL_LIMIT_RECOURSE.to_owned(),
            // Decisions not yet given an ending
            // (`work/ssi/ssi-refusals-whose-decision-has-no-ending.md`).
            Self::ExhaustivenessInconclusive(_)
            | Self::CellBudget { .. }
            | Self::StepBudget { .. }
            | Self::SeedRefinementFailed { .. }
            | Self::StepRefinementFailed { .. }
            | Self::TubeLadderEmpty { .. }
            | Self::TubeProbeSilent { .. }
            | Self::FootPointInconclusive { .. }
            | Self::Fit(_)
            | Self::UnsupportedCertificate { .. }
            | Self::TubeDegenerate(_)
            | Self::WrongLane { .. }
            | Self::Band(_) => return None,
        })
    }

    /// The payload and, where its decision gives one, the ending read at
    /// `reading` ([`SsiError::ending`]).
    #[must_use]
    pub fn render(&self, reading: Reading) -> String {
        match self.ending(reading) {
            Some(ending) => format!("{self}. {ending}"),
            None => self.to_string(),
        }
    }
}

/// Which stored datum of an operand is not a finite number
/// ([`SsiError::OperandNotFinite`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperandDatum {
    /// A field of an analytic surface.
    Field(geom::SurfaceDatum),
    /// A spline operand's control point, by its index in the net.
    ControlPoint(usize),
}

impl core::fmt::Display for OperandDatum {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Field(datum) => f.write_str(datum.name()),
            Self::ControlPoint(index) => write!(f, "control point {index}"),
        }
    }
}

/// A parameter axis of a NURBS operand's chart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartAxis {
    /// The first parameter.
    U,
    /// The second parameter.
    V,
}

/// Why a NURBS operand's chart cannot carry a length in metres into its
/// parameters ([`SsiError::ChartSpeed`]).
///
/// The chart speed over the wall's domain is minted once per axis
/// ([`ChartedNurbs::mint`]); a floor or a tube pad crosses into chart
/// units by dividing by it, so both a zero and a non-finite speed leave
/// nothing to divide by. Neither refining the search nor narrowing the
/// tube cures either, so each refuses at the mint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartSpeedRefusal {
    /// The certified chart speed along `axis` is zero: the wall is
    /// constant along that axis, a degenerate chart.
    Zero {
        /// The axis.
        axis: ChartAxis,
    },
    /// The chart speed along `axis` has no finite certified bound: its
    /// derivative bound overflowed or was refused.
    NotFinite {
        /// The axis.
        axis: ChartAxis,
    },
}

impl ChartSpeedRefusal {
    /// The decision this refusal is the sign-certain arm of, read by
    /// every door that reports it: the mint's verdict is exact, a speed
    /// bound that is zero or not finite, with no band between.
    #[must_use]
    pub fn check(self) -> CertCheck {
        match self {
            Self::Zero { .. } => CertCheck::PlaneNurbsChartSpeed,
            Self::NotFinite { .. } => CertCheck::PlaneNurbsChartSpeedBound,
        }
    }

    /// The refusal's sentence, without the `ssi:` prefix — what
    /// [`SsiError`]'s `Display` renders and what a lane that reports it
    /// in its own vocabulary carries.
    #[must_use]
    pub fn what(self) -> &'static str {
        match self {
            Self::Zero { axis: ChartAxis::U } => {
                "the NURBS wall is constant along u — its certified chart speed along u is \
                 zero, a degenerate chart — so no length in metres can be translated into \
                 its parameter domain"
            }
            Self::Zero { axis: ChartAxis::V } => {
                "the NURBS wall is constant along v — its certified chart speed along v is \
                 zero, a degenerate chart — so no length in metres can be translated into \
                 its parameter domain"
            }
            Self::NotFinite { axis: ChartAxis::U } => {
                "there is no finite bound on the NURBS wall's chart speed along u — its \
                 derivative bound overflowed or is refused — so no length in metres can be \
                 translated into its parameter domain"
            }
            Self::NotFinite { axis: ChartAxis::V } => {
                "there is no finite bound on the NURBS wall's chart speed along v — its \
                 derivative bound overflowed or is refused — so no length in metres can be \
                 translated into its parameter domain"
            }
        }
    }
}

/// Why limb 3's chart tube cannot be probed at any rung
/// ([`SsiError::TubeDegenerate`]): a fact about one span window of the
/// traced pcurve that does not depend on the pad, so no narrower tube
/// cures it and the ladder refuses at the first rung that meets it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TubeDegeneracy {
    /// The wall's certified chart stretch transverse to the pcurve is
    /// zero over a window: the wall is constant across the locus there.
    /// A narrower window lies inside this one, where the stretch bound
    /// is zero as well.
    WallConstantAcrossLocus,
    /// The pcurve's tangent at a span midpoint is zero or not finite, so
    /// it names no direction to read the wall across. The tangent is the
    /// pcurve's alone, whatever the pad.
    PcurveTangentUnusable,
}

impl TubeDegeneracy {
    /// The refusal's sentence, without the `ssi:` prefix.
    #[must_use]
    pub fn what(self) -> &'static str {
        match self {
            Self::WallConstantAcrossLocus => {
                "the uniqueness tube cannot be probed: the NURBS wall is constant across the \
                 traced locus over a span of its pcurve (its certified chart stretch \
                 transverse to the pcurve is zero), which no narrower tube cures"
            }
            Self::PcurveTangentUnusable => {
                "the uniqueness tube cannot be probed: the traced pcurve's tangent at a span \
                 midpoint is zero or not finite, so it names no direction to read the wall \
                 across, which no narrower tube cures"
            }
        }
    }
}

/// A NURBS operand together with its chart speeds, minted once over
/// its whole domain ([`ChartedNurbs::mint`]). The fields are private:
/// the only way to hold one is to have minted it from the surface it
/// carries.
#[derive(Clone, Copy, Debug)]
pub struct ChartedNurbs<'a, T: geom_core::Real> {
    surface: &'a NurbsSurface<T>,
    speeds: enclose::ChartSpeeds,
}

impl<'a, T: geom_core::CertifiedBounds> ChartedNurbs<'a, T> {
    /// Mints the wall's `{u, v}` chart speeds over its domain.
    ///
    /// # Errors
    ///
    /// [`SsiError::ChartSpeed`], naming the axis whose speed is zero or
    /// not finite.
    pub fn mint(surface: &'a NurbsSurface<T>) -> Result<Self, SsiError> {
        let speeds = NurbsBoxes::new(surface).chart_speeds()?;
        Ok(Self { surface, speeds })
    }
}

impl<'a, T: geom_core::Real> ChartedNurbs<'a, T> {
    /// The surface.
    #[must_use]
    pub fn surface(&self) -> &'a NurbsSurface<T> {
        self.surface
    }

    /// The chart speeds it was minted with.
    pub(crate) fn speeds(&self) -> enclose::ChartSpeeds {
        self.speeds
    }
}

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
    /// The door every SSI operation asks first: the centre is a finite
    /// point, and the half-extent, the feature extent and the floor
    /// scale are positive finite numbers. A zero half-extent is an
    /// empty domain, which no answer about it is evidence of.
    fn check(&self) -> Result<(), SsiError> {
        let c = self.center;
        let fields = [
            (
                DomainField::Center,
                c.to_array().into_iter().find(|v| !v.is_finite()),
            ),
            (DomainField::HalfExtent, positive_finite(self.half_extent)),
            (DomainField::Extent, positive_finite(self.extent)),
            (DomainField::FloorScale, positive_finite(self.floor_scale)),
        ];
        match fields.into_iter().find_map(|(f, bad)| bad.map(|v| (f, v))) {
            Some((field, value)) => Err(SsiError::DomainUnusable { field, value }),
            None => Ok(()),
        }
    }

    /// The slab as a enclosure box.
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

/// `Some(x)` when `x` is not a positive finite number.
fn positive_finite(x: f64) -> Option<f64> {
    (!(x.is_finite() && x > 0.0)).then_some(x)
}

/// Which knob of an [`SsiDomain`] a [`SsiError::DomainUnusable`] names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomainField {
    /// [`SsiDomain::center`]: a coordinate is not finite.
    Center,
    /// [`SsiDomain::half_extent`]: not positive and finite.
    HalfExtent,
    /// [`SsiDomain::extent`]: not positive and finite.
    Extent,
    /// [`SsiDomain::floor_scale`]: not positive and finite.
    FloorScale,
}

impl DomainField {
    /// The knob's name, and what it must be.
    fn words(self) -> (&'static str, &'static str) {
        match self {
            Self::Center => ("centre", "a finite point"),
            Self::HalfExtent => ("half-extent", "a positive finite length"),
            Self::Extent => ("feature extent", "a positive finite length"),
            Self::FloorScale => ("floor scale", "a positive finite multiplier"),
        }
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
/// pass consumes. A branch with no spatial tube banks nothing, which
/// only makes the accounting harder.
fn branch_tubes(branch: &SsiBranch) -> Vec<Box3> {
    let (Curve3::Nurbs(c), SsiTube::Spatial { radius }) =
        (&branch.carrier, branch.certificate.tube)
    else {
        return Vec::new();
    };
    certify::tube_boxes(c, radius)
}

/// The chart windows of a certified plane × NURBS branch — the tubes
/// the chart accounting pass consumes: [`certify::chart_tube_windows`]
/// at the pad the certificate recorded, so accounting banks the region
/// limb 3 proved and no other. A branch with no chart tube, or whose
/// pcurve has a refused hull, banks nothing, which only makes the
/// accounting harder.
fn branch_chart_tubes(branch: &SsiBranch) -> Vec<UvRect> {
    let (Some(pc), SsiTube::Chart { pad_u, pad_v, .. }) =
        (&branch.pcurve_b, branch.certificate.tube)
    else {
        return Vec::new();
    };
    certify::chart_tube_windows(pc, (pad_u, pad_v))
        .into_iter()
        .flatten()
        .map(|w| w.rect)
        .collect()
}

/// **The decision a trace's escalation names** ([`SsiError::Escalated`]):
/// the march's own decisions and the cylinder × sphere door's tangency,
/// a closed type, so the ending is an exhaustive match (D4 ¶1 (i)).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TraceDecision {
    /// `ssi_transversality_arm`: the lever arm the crossing angle is
    /// levered by (the operands' curvature radius, or the feature
    /// extent) is a positive length — the gate on
    /// [`TraceDecision::Transversality`].
    TransversalityArm,
    /// `ssi_transversality`: the surfaces cross at a clear angle at a
    /// marched state ([`SsiError::TransversalityBand`] is its verdict).
    Transversality,
    /// `ssi_step_progress`: the step clears the tolerance band
    /// ([`SsiError::StepCollapsed`] is its verdict).
    StepProgress,
    /// `ssi_branch_open_end`: the branch has left the domain. Every
    /// definite sign passes.
    BranchOpenEnd,
    /// `ssi_closure_return`: the trace has come back to its start.
    /// Every definite sign passes.
    ClosureReturn,
    /// `ssi_closure_tangent`: it came back running the way it left
    /// ([`SsiError::SelfCrossingLocus`] is its verdict).
    ClosureTangent,
    /// `ssi_cs_tangency`: the cylinder × sphere pair's gap from a
    /// tangent pose ([`SsiError::PairTangent`] is its verdict).
    PairTangency,
}

impl TraceDecision {
    /// This decision's escalation on `cause`.
    pub(crate) fn escalated(self, cause: Indeterminate) -> SsiError {
        SsiError::Escalated {
            decision: self,
            cause,
        }
    }

    /// The question the decision asks, as the refusal states it.
    fn question(self) -> &'static str {
        match self {
            Self::TransversalityArm => {
                "the crossing angle's lever arm (the surfaces' curvature radius, or the feature \
                 extent) is a positive length"
            }
            Self::Transversality => "the surfaces cross at a clear angle along the traced locus",
            Self::StepProgress => "the march's step clears the tolerance band",
            Self::BranchOpenEnd => "the traced branch has left the domain",
            Self::ClosureReturn => "the trace has come back to its start",
            Self::ClosureTangent => "the trace came back to its start running the way it left",
            Self::PairTangency => "the sphere and the cylinder are tangent",
        }
    }

    /// The ending this decision's escalation on `cause` carries, read at
    /// `reading`: the one its decided verdicts carry (D4 ¶1 (iv)).
    ///
    /// - The transversality decision ends by [`CertCheck::Transversality`],
    ///   and its arm gate as the decision it guards, as the edge
    ///   certifier's does.
    /// - The step and the closure angle are the march's own, no size a
    ///   caller intends, so they end in their lever alone, as
    ///   [`SsiError::StepCollapsed`] and [`SsiError::SelfCrossingLocus`] do.
    /// - The open end passes on every definite sign, so only a marched
    ///   state landing within the band of the domain's boundary refuses
    ///   it: moving the boundary, or the geometry under it, moves that.
    /// - The return passes on every definite sign too, so only where an
    ///   untrusted marched sample landed against the seed refuses it,
    ///   which no lever the caller holds moves: the last resort.
    /// - Where the step, the closure angle or the open end read a
    ///   poisoned margin, no lever of theirs is what refused: the march's
    ///   own arithmetic went unreadable, which is the kernel's.
    #[must_use]
    pub fn ending(self, cause: &Indeterminate, reading: Reading) -> String {
        let arm = RefusedArm::Undecided(cause);
        match self {
            Self::TransversalityArm | Self::Transversality => {
                crate::certify::recourse(CertCheck::Transversality, arm, reading)
            }
            Self::PairTangency => PAIR_TANGENCY.recourse(arm, reading),
            Self::StepProgress | Self::ClosureTangent | Self::BranchOpenEnd
                if cause.margin.is_invalid() =>
            {
                defect_ending(reading).to_owned()
            }
            Self::StepProgress => STEP_SCALE.recourse(RefusedArm::SignCertain, reading),
            Self::ClosureTangent => SELF_CROSSING.recourse(RefusedArm::SignCertain, reading),
            Self::BranchOpenEnd => OPEN_END.recourse(RefusedArm::SignCertain, reading),
            Self::ClosureReturn => Unsized::LastResort.recourse(arm, reading),
        }
    }
}

/// The march's settling against a domain that reaches further from zero
/// than its coordinates resolve, where no operand bounds the states
/// nearer ([`ReachBound::Domain`]).
const DOMAIN_SCALE: SizedDecision = SizedDecision {
    lever: "name a domain around the feature traced, nearer the origin, or move the geometry \
            nearer the origin, where its coordinates resolve the tolerance",
    size: "scale",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The open end (`ssi_branch_open_end`), refused only where a marched
/// state lands within the band of the domain's boundary. The boundary
/// is the caller's domain, or a spline operand's own edge, which moves
/// with the geometry. Read on its sign-certain arm alone: where a state
/// lands is the march's own, no size a caller intends.
const OPEN_END: SizedDecision = SizedDecision {
    lever: "move the domain's boundary, or the geometry, a little, so the traced branch does \
            not leave the domain within a few tolerances of a marched state",
    size: "boundary distance",
    passes: SizedPass::AnySign,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The closure's tangent decision (`ssi_closure_tangent`), refused when
/// the trace comes back across or against the way it left: the locus
/// cusps or crosses itself. Read on its sign-certain arm alone, since
/// the closure angle is the march's own.
const SELF_CROSSING: SizedDecision = SizedDecision {
    lever: "move the surfaces so their intersection does not cusp or cross itself",
    size: "closure angle",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The stepper's step against the operands and the domain the caller
/// named ([`SsiError::StepCollapsed`], [`SsiError::StepUnusable`]): a
/// step that collapses into the band, overflows, or does not move the
/// state comes from operands outside the model's size range or a domain
/// far larger than the feature traced.
const STEP_SCALE: SizedDecision = SizedDecision {
    lever: "bring the operands within the model's size range, and name a domain and feature \
            extent near the size of the feature traced",
    size: "scale",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// The cylinder × sphere tangency decision (`ssi_cs_tangency`): the
/// gap from the nearer tangent pose, a magnitude, which passes positive.
const PAIR_TANGENCY: SizedDecision = SizedDecision {
    lever: "separate the sphere and the cylinder, or move one so they are clearly not tangent",
    size: "near-tangency",
    passes: SizedPass::Positive,
    stored: StoredDefinite::Lever,
    at_zero: None,
};

/// `surface`, the door's `operand`, refused when one of its stored data
/// is not a finite number ([`SsiError::OperandNotFinite`]): an analytic
/// kind's fields, or a spline's control points ([`finite_net`]).
fn finite_operand(operand: &'static str, surface: &Surface<f64>) -> Result<(), SsiError> {
    let data = match surface.data() {
        geom::SurfaceData::Analytic(data) => data,
        geom::SurfaceData::Nurbs(net) => return finite_net(operand, net),
        geom::SurfaceData::Approx(_) => return Ok(()),
    };
    match data
        .into_iter()
        .find(|(_, value)| !value.scalars().all(f64::is_finite))
    {
        Some((datum, _)) => Err(SsiError::OperandNotFinite {
            operand,
            datum: OperandDatum::Field(datum),
        }),
        None => Ok(()),
    }
}

/// [`finite_operand`] for a spline operand a door takes as a net: its
/// first control point that is not finite. Its weights are the
/// constructor's to refuse.
fn finite_net(operand: &'static str, net: &NurbsSurface<f64>) -> Result<(), SsiError> {
    match net
        .control()
        .iter()
        .position(|p| !(p.x.is_finite() && p.y.is_finite() && p.z.is_finite()))
    {
        Some(index) => Err(SsiError::OperandNotFinite {
            operand,
            datum: OperandDatum::ControlPoint(index),
        }),
        None => Ok(()),
    }
}

/// **cylinder × sphere** — the ℝ³ implicit-pair arm (2×3 SVD, module
/// docs). Shape (iv)'s planted small loop lives here.
///
/// # Errors
///
/// Any [`SsiError`]; in particular
/// [`SsiError::ExhaustivenessInconclusive`] when the domain cannot be
/// proved exhausted at the floor, [`SsiError::PairTangent`] when the
/// pair is tangent somewhere to tolerance, [`SsiError::TransversalityBand`]
/// when a march meets a near-tangent crossing, and
/// [`SsiError::OperandNotFinite`] for an operand that is not finite.
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
    // |d − r| = R for a sphere against the wall, d + r = R for a
    // cylinder inside the sphere touching it (around a circle when
    // coaxial with r = R; at one point off-axis, where the pair still
    // crosses elsewhere). Both are C7 constructions, and classifying
    // them here rather than
    // discovering them mid-march is not an optimization: near a
    // tangency the exhaustiveness subdivision's surviving set is a
    // three-dimensional shell rather than a curve, so a marcher that
    // "found out later" would first exhaust its cell budget. ----
    let (sphere, cyl) = match (a, b) {
        (Surface::Sphere { .. }, Surface::Cylinder { .. }) => (a, b),
        _ => (b, a),
    };
    finite_operand("sphere", sphere)?;
    finite_operand("cylinder", cyl)?;
    domain.check()?;
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
    let tangency = Real::min(((d - r).abs() - big_r).abs(), (d + r - big_r).abs());
    match crate::dihedral::decide_reported("ssi_cs_tangency", Margin::of(tangency), band) {
        Ok(geom_core::Decided {
            sign: geom_core::Sign::Positive | geom_core::Sign::Negative,
            ..
        }) => {}
        Ok(geom_core::Decided {
            sign: geom_core::Sign::Zero,
            margin,
        }) => {
            return Err(SsiError::PairTangent {
                verdict: Refused::Zero(crate::recourse::Classified { margin, band }),
            });
        }
        Err(diag) => return Err(TraceDecision::PairTangency.escalated(diag)),
    }

    let sys = ImplicitPairR3 { a, b };
    let slab = domain.slab();
    // Both floors and the march's tolerance are minted over the slab
    // before any sweep or march runs, the proof obligation's first.
    // The march settles to what the slab's coordinates resolve.
    let account_floor = SweepFloor::r3(slab, domain.floor(band), FloorKind::Accounting)?;
    let seed_floor = SweepFloor::r3(slab, domain.seed_floor(), FloorKind::Seeding)?;
    let tol = MarchTol::from_band(band, quadric_readout(a, b, slab))?;

    // ---- seeds: the subdivision, asked for its seeding duty ----
    let seeds = exhaust::seed_r3(a, b, seed_floor)?;
    let seed_count = seeds.len() as u32;

    // ---- march, fit, certify ----
    let ctx = MarchContext::<3> {
        domain: [
            [slab.x.lo(), slab.x.hi()],
            [slab.y.lo(), slab.y.hi()],
            [slab.z.lo(), slab.z.hi()],
        ],
        extent: domain.extent,
        tol,
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
        let state = seed.to_array();
        let landed = march::newton_refine::<2, 3, _>(&sys, state, ctx.tol);
        let probe = landed.map_or(*seed, Point3::from_array);
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
            // that was a miss. Every other refusal propagates, a march
            // that lost its branch mid-trace included.
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
    let exhaustiveness = exhaust::account_r3(a, b, &tubes, account_floor)?;
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
    if !tol.is_of(band) {
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
    let arm = crate::dihedral::folded_lever_arm(a, b, points[0], domain.extent);
    let cert = certify::certify_branch(
        &carrier,
        None,
        &SsiOperand::Analytic(a),
        &SsiOperand::Analytic(b),
        TubeScale::split(arm, domain.extent),
        band,
    )?;
    let params = carrier.domain();
    let carrier = Curve3::Nurbs(std::sync::Arc::new(carrier));
    let witness = carrier.mid_point(params.0, params.1);
    Ok(SsiBranch {
        carrier,
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

/// **The quadric lane's readout**, at every door that marches an
/// analytic pair: the domain's `slab`, cut down to the box of each
/// operand that is bounded. Every state lies on both operands inside the
/// slab, so their coordinates reach no further than that.
fn quadric_readout(a: &Surface<f64>, b: &Surface<f64>, slab: Box3) -> Readout {
    let operands = [a, b]
        .into_iter()
        .filter_map(analytic_box)
        .reduce(Box3::meet);
    Readout::spatial(slab, operands, SSI_QUADRIC_NOISE_ULPS)
}

/// The box an analytic surface lies in, where it is bounded.
fn analytic_box(surface: &Surface<f64>) -> Option<Box3> {
    match *surface {
        Surface::Sphere { center, radius, .. } => Some(Box3::around(center, radius.abs())),
        Surface::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => Some(Box3::around(
            center,
            major_radius.abs() + minor_radius.abs(),
        )),
        _ => None,
    }
}

/// **The spline lane's readout**, at every door that marches a plane ×
/// NURBS pair: the plane's `window` cut down to the wall's control
/// hull, where every state's residual is read, and the wall's chart
/// `root`, through which Newton moves, whichever is louder.
///
/// The chart's spacing crosses into metres at the sup speed, which
/// over-states its noise, so the march settles no finer than either
/// resolves and may refuse a wall a slower axis would carry. On a
/// rational wall the over-statement is larger: the sup speed grows with
/// the wall's absolute coordinates
/// (`work/ssi/rational-chart-sup-speed-grows-with-translation.md`).
fn spline_readout(
    window: Box3,
    wall: &NurbsSurface<f64>,
    root: UvRect,
    speed: geom_core::SupSpeed<f64>,
) -> Readout {
    let hull = wall
        .control()
        .iter()
        .map(|&p| Box3::between(p, p))
        .reduce(Box3::hull);
    Readout::spatial(window, hull, SSI_SPLINE_NOISE_ULPS).or_chart(
        root,
        speed,
        SSI_SPLINE_NOISE_ULPS,
    )
}

/// The ℝ³ box a plane chart's `±half` window spans: every state the ℝ⁴
/// trace settles lies on the plane inside it.
fn window_reach(chart: &Chart<'_>, half: f64) -> Box3 {
    let corner = |u, v| chart.eval(u, v);
    Box3::between(corner(-half, -half), corner(half, half))
        .hull(Box3::between(corner(-half, half), corner(half, -half)))
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
/// Any [`SsiError`]. Notably: [`SsiError::CertificateEscalated`] naming
/// [`SsiLimb::HullSup`] when the fitted pair's real between-samples
/// deviation lands in the band (an inflected wall can genuinely earn
/// this — module docs), [`SsiError::FitSampleBudget`] at tolerances
/// whose sample demand exceeds the fit budget,
/// [`SsiError::OperandNotFinite`] for a plane or wall that is not
/// finite, and [`SsiError::ChartSpeed`] for a wall whose chart speed
/// along an axis is zero or has no finite bound.
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
    // Before any sweep: the chart sweep lifts the plane's origin and
    // normal into intervals, which refuse a non-finite point, and its
    // refusal arm can only name the wall's net.
    finite_operand("plane", plane)?;
    finite_net("NURBS wall", wall)?;
    domain.check()?;
    let half = domain.half_extent;
    let Some(chart_a) = Chart::plane_of(plane, (-half, half), (-half, half)) else {
        return Err(SsiError::WrongLane {
            expected: "a plane and a NURBS surface traced in ℝ⁴ on their charts",
        });
    };
    let window = window_reach(&chart_a, half);
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

    // The wall's chart speeds, minted once per axis; the mint refuses
    // a zero or non-finite axis by name. The chart floors are metres ÷
    // the larger of the two, so a floor stated in metres means the same
    // thing in both lanes, and dividing by a sup UNDER-states the
    // parameter reach — the safe side of a floor. Each floor is minted
    // once over the wall's domain, which refuses one the domain cannot
    // resolve, the proof obligation's first.
    let charted = ChartedNurbs::mint(wall)?;
    let speed = charted.speeds().max();
    let wall_op = SsiOperand::Nurbs(charted);
    let account_floor = SweepFloor::chart(root, domain.floor(band), speed, FloorKind::Accounting)?;
    let seed_floor = SweepFloor::chart(root, domain.seed_floor(), speed, FloorKind::Seeding)?;
    let tol = MarchTol::from_band(band, spline_readout(window, wall, root, speed))?;

    // ---- seeds ----
    let seeds = exhaust::seed_chart_plane(wall, p0, normal, seed_floor)?;
    let seed_count = seeds.len() as u32;

    let v_ref = normal.cross(u_ref);
    let to_plane_chart = |p: Point3<f64>| {
        let q = p - p0;
        (q.dot(u_ref), q.dot(v_ref))
    };
    let ctx = MarchContext::<4> {
        domain: [[pu.0, pu.1], [pv.0, pv.1], [ud.0, ud.1], [vd.0, vd.1]],
        extent: domain.extent,
        tol,
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
            // As in `cylinder_sphere_ssi`: only a seed that will not
            // settle is no branch.
            Err(SsiError::SeedRefinementFailed { .. }) => continue,
            Err(e) => return Err(e),
        };
        let branch = finish_r4(&sys, &trace, plane, &wall_op, &domain, ctx.tol, band)?;
        tubes.extend(branch_chart_tubes(&branch));
        branches.push(branch);
    }

    let exhaustiveness = exhaust::account_chart_plane(wall, p0, normal, &tubes, account_floor)?;
    Ok(SsiOutcome {
        branches,
        exhaustiveness,
        seeds: seed_count,
    })
}

fn finish_r4(
    sys: &ParametricPairR4<'_>,
    trace: &Trace<4>,
    plane: &Surface<f64>,
    wall: &SsiOperand<'_, f64>,
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
        wall,
        TubeScale::uniform(domain.extent),
        band,
    )?;
    let params = carrier.domain();
    let carrier = Curve3::Nurbs(std::sync::Arc::new(carrier));
    let witness = carrier.mid_point(params.0, params.1);
    Ok(SsiBranch {
        carrier,
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
    finite_operand("plane", plane)?;
    finite_net("NURBS wall", wall)?;
    domain.check()?;
    let half = domain.half_extent;
    let Some(chart_a) = Chart::plane_of(plane, (-half, half), (-half, half)) else {
        return Err(SsiError::WrongLane {
            expected: "a plane and a NURBS surface traced in ℝ⁴ on their charts",
        });
    };
    let window = window_reach(&chart_a, half);
    let chart_b = Chart::Nurbs(wall);
    let ((pu, pv), (ud, vd)) = (chart_a.domain(), chart_b.domain());
    let speed = ChartedNurbs::mint(wall)?.speeds().max();
    let root = UvRect { u: ud, v: vd };
    let tol = MarchTol::decoupled(march_tol, spline_readout(window, wall, root, speed))?;
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
    finite_operand("first operand", a)?;
    finite_operand("second operand", b)?;
    domain.check()?;
    let sys = ImplicitPairR3 { a, b };
    let slab = domain.slab();
    let ctx = MarchContext::<3> {
        domain: [
            [slab.x.lo(), slab.x.hi()],
            [slab.y.lo(), slab.y.hi()],
            [slab.z.lo(), slab.z.hi()],
        ],
        extent: domain.extent,
        tol: MarchTol::from_band(band, quadric_readout(a, b, slab))?,
        max_steps: SSI_MAX_STEPS,
    };
    let trace = march_both::<2, 3, _>(&sys, seed.to_array(), ctx, StepperMode::Idealized, band)?;
    let pts = trace_points::<2, 3, _>(&sys, &trace);
    Ok((pts, trace.end))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod ending_tests {
    use geom_core::{
        Band, Indeterminate, KERNEL_DEFECT_ENDING, KERNEL_LIMIT_LAST_RESORT, KERNEL_LIMIT_RECOURSE,
        KERNEL_OR_FILE_DEFECT_ENDING, MarginDiag,
    };

    use super::{SSI_MAX_FIT_SAMPLES, SsiError, SsiLimb, TraceDecision};
    use crate::recourse::{Classified, Reading, Refused};

    fn band() -> Band {
        Band::new(1e-9, 1e-8).unwrap()
    }

    /// An escalation of `margin` against [`band`].
    fn cause(margin: MarginDiag) -> Indeterminate {
        Indeterminate {
            margin,
            band: band(),
            predicate: None,
            terminal_sliver: false,
        }
    }

    /// The transversality decision's one lever, read at every door.
    const CROSS: &str = "Recourse: move the geometry so the surfaces cross at a clearer angle";

    /// **Every trace escalation ends by the decision it names, as that
    /// decision's verdicts end**, and its `Display` is the payload alone,
    /// never the coincidence menu (no SSI door takes a declaration):
    ///
    /// - the transversality decision, its arm gate and limb 3 — the
    ///   march's verdict, the tube's verdict and both escalations — all
    ///   end in the one lever, with the tolerance an in-band margin
    ///   gives;
    /// - the pair's tangency escalation ends as its verdict does;
    /// - the step and the closure angle end exactly as their decided
    ///   refusals (`StepCollapsed`, `SelfCrossingLocus`);
    /// - the open end and the return end in the last resort.
    #[test]
    fn every_trace_escalation_ends_as_its_decisions_verdict() {
        let in_band = cause(MarginDiag::value(5e-9));
        let tighten = ", or, if this angle is intended, tighten the tolerance below 5e-10 m";
        let escalated = |decision| SsiError::Escalated {
            decision,
            cause: in_band,
        };
        let zero = Refused::Zero(Classified {
            margin: MarginDiag::value(5e-9),
            band: band(),
        });
        let transversal = [
            escalated(TraceDecision::Transversality),
            escalated(TraceDecision::TransversalityArm),
            SsiError::TransversalityBand {
                sin_theta: 5e-9,
                arm: 1.0,
                sigma_min: 1e-9,
                verdict: zero,
            },
            SsiError::TubeStraddles {
                verdict: zero,
                boxes: 4,
            },
            SsiError::CertificateEscalated {
                limb: SsiLimb::Tube,
                cause: in_band,
            },
        ];
        for error in &transversal {
            assert_eq!(
                error.ending(Reading::Build).as_deref(),
                Some(format!("{CROSS}{tighten}").as_str()),
                "{error:?}"
            );
        }
        let pair = escalated(TraceDecision::PairTangency).ending(Reading::Build);
        assert_eq!(
            pair,
            SsiError::PairTangent { verdict: zero }.ending(Reading::Build),
            "the pair's tangency"
        );
        let step = SsiError::StepCollapsed {
            mode: "realized",
            step_meters: 5e-9,
            speed: 1.0,
        };
        let crossing = SsiError::SelfCrossingLocus {
            cos_phi: -1.0,
            arc_length: 1.0,
        };
        for reading in [Reading::Build, Reading::AtRest] {
            assert_eq!(
                escalated(TraceDecision::StepProgress).ending(reading),
                step.ending(reading),
                "the step at {reading:?}"
            );
            assert_eq!(
                escalated(TraceDecision::ClosureTangent).ending(reading),
                crossing.ending(reading),
                "the closure angle at {reading:?}"
            );
        }
        assert_eq!(
            crossing.ending(Reading::Build).as_deref(),
            Some("Recourse: move the surfaces so their intersection does not cusp or cross itself")
        );
        assert_eq!(
            escalated(TraceDecision::ClosureReturn)
                .ending(Reading::Build)
                .as_deref(),
            Some(KERNEL_LIMIT_RECOURSE),
            "the return"
        );
        let open = escalated(TraceDecision::BranchOpenEnd).ending(Reading::Build);
        assert_eq!(
            open.as_deref(),
            Some(
                "Recourse: move the domain's boundary, or the geometry, a little, so the traced \
                 branch does not leave the domain within a few tolerances of a marched state"
            ),
            "the open end names the domain"
        );
        for decision in [
            TraceDecision::TransversalityArm,
            TraceDecision::Transversality,
            TraceDecision::StepProgress,
            TraceDecision::BranchOpenEnd,
            TraceDecision::ClosureReturn,
            TraceDecision::ClosureTangent,
            TraceDecision::PairTangency,
        ] {
            let error = escalated(decision);
            let shown = error.to_string();
            assert!(
                shown.starts_with("ssi: whether ")
                    && shown.ends_with(&format!("too close to call: {}", in_band.payload()))
                    && !shown.contains("Recourse")
                    && !shown.contains("declare"),
                "payload only: {shown}"
            );
            let rendered = error.render(Reading::Build);
            assert!(!rendered.contains("declare"), "{rendered}");
            let words = rendered.split_whitespace().count();
            assert!(words < 75, "{words} words: {rendered}");
        }
    }

    /// **A certificate limb's definite refusal ends by its limb**, as its
    /// undecided sibling does: a fitted residual's at a build is the
    /// last resort, and over stored geometry the defect of the kernel or
    /// the file. The tube's refusal is held to the concision standard.
    #[test]
    fn a_definite_limb_refusal_ends_by_its_limb() {
        for limb in [SsiLimb::OnLocus, SsiLimb::HullSup] {
            let refusal = SsiError::CertificateLimb { limb, value: 3e-9 };
            assert_eq!(
                refusal.ending(Reading::Build).as_deref(),
                Some(KERNEL_LIMIT_RECOURSE),
                "{limb:?}"
            );
            assert_eq!(
                refusal.ending(Reading::AtRest).as_deref(),
                Some(KERNEL_OR_FILE_DEFECT_ENDING),
                "{limb:?}"
            );
        }
        let tube = SsiError::TubeStraddles {
            verdict: Refused::Zero(Classified {
                margin: MarginDiag::value(5e-10),
                band: band(),
            }),
            boxes: 12,
        };
        let shown = tube.to_string();
        let literal = shown.split_whitespace().count();
        assert!(literal < 35, "{literal} words: {shown}");
        let rendered = tube.render(Reading::Build);
        assert!(
            rendered.starts_with(&format!("{shown}. {CROSS}")),
            "{rendered}"
        );
        let mismatch = SsiError::MarchTolMismatch {
            marched: 1e-9,
            band_zero: 2e-9,
        };
        assert_eq!(
            mismatch.ending(Reading::Build).as_deref(),
            Some(KERNEL_DEFECT_ENDING)
        );
    }

    /// The SSI refusals that end by a decision end by its table, and
    /// `Display` is the payload alone: the transversality death names
    /// the transversality decision's lever and the pair's tangency its
    /// own (separating the operands among them), each with the tolerance
    /// their in-band margin gives (`m/K`, here `K = 10`), never a
    /// declaration (the doors take none); the spent fit budget ends in
    /// the loosening clause and the last resort, within 50 words
    /// rendered.
    #[test]
    fn each_ssi_ending_is_its_decisions() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let zero = Refused::Zero(Classified {
            margin: MarginDiag::value(5e-10),
            band,
        });
        let death = SsiError::TransversalityBand {
            sin_theta: 5e-10,
            arm: 1.0,
            sigma_min: 1e-10,
            verdict: zero,
        };
        let pair = SsiError::PairTangent { verdict: zero };
        let budget = SsiError::FitSampleBudget {
            samples: 4 * SSI_MAX_FIT_SAMPLES,
            budget: SSI_MAX_FIT_SAMPLES,
        };
        for (error, ending) in [
            (
                &death,
                format!(
                    "{CROSS}, or, if this angle is intended, tighten the tolerance below 5e-11 m"
                ),
            ),
            (
                &pair,
                "Recourse: separate the sphere and the cylinder, or move one so they are \
                 clearly not tangent, or, if this near-tangency is intended, tighten the \
                 tolerance below 5e-11 m"
                    .to_owned(),
            ),
            (
                &budget,
                format!(
                    "Recourse: loosen the tolerance until a branch needs at most \
                     {SSI_MAX_FIT_SAMPLES} samples, {KERNEL_LIMIT_LAST_RESORT}"
                ),
            ),
        ] {
            let shown = error.to_string();
            assert!(!shown.contains("Recourse"), "payload only: {shown}");
            let rendered = error.render(Reading::Build);
            assert_eq!(rendered, format!("{shown}. {ending}"));
            assert!(!rendered.contains("declare"), "{rendered}");
        }
        let rendered = budget.render(Reading::Build);
        assert!(
            rendered.contains(&(4 * SSI_MAX_FIT_SAMPLES).to_string()),
            "{rendered}"
        );
        let words = rendered.split_whitespace().count();
        assert!(words < 50, "{words} words: {rendered}");
    }
}

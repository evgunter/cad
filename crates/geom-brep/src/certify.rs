//! Certified carriers: the D4 ¶2 attachment gate for edge geometry.
//!
//! An edge's concrete 3-D curve (its *carrier*) is a derived cache of
//! its intensional description ([`crate::EdgeDescription`]). This module
//! is the only way to marry the two: [`EdgeCurve::certify`] takes an
//! uncertified [`EdgeCurveSpec`] plus the edge's endpoint points and
//! either returns a certified [`EdgeCurve`] — whose fields are private,
//! so **an uncertified carrier is unrepresentable** — or fails with a
//! typed [`CertifyError`] (operation-time enforcement, D4 ¶3). The
//! tier-3 validator re-runs the same checks at rest
//! ([`EdgeCurve::recertify`]).
//!
//! # The deterministic sampling schedule (D9)
//!
//! Certification is closed-form residual evaluation at a **fixed
//! schedule**: [`CERT_SAMPLES`] = 9 parameters
//! `t_i = t₀ + (t₁ − t₀)·(i/8)`, i = 0…8 — endpoints included and
//! ASSIGNED (`t₀` and `t₁` themselves, [`schedule_param`]), the
//! fractions exact dyadics, the arithmetic a fixed association order —
//! so two runs over the same body produce byte-identical
//! [`Certificate`]s. Interior samples (i = 1…7) additionally carry the
//! transversality check for `Intersection` descriptions (endpoint
//! vertices may legitimately sit on chart/surface singularities — cone
//! apexes, sphere poles — where angular classification honestly
//! poisons; residual checks, which never need a gradient, still run at
//! the endpoints).
//!
//! # Dimensional honesty
//!
//! Every classified residual is a **length in meters** against the
//! run's linear band: point-to-point distances directly, implicit
//! surface residuals in the linearized forms of [`crate::implicit`],
//! angular transversality as displacement through its lever arm
//! ([`crate::classify_dihedral`]'s margin). No squared bands, no
//! dimensionless margins.
//!
//! # The stored parameter interval — a certified cache, not an authority
//!
//! `geom`'s ratified curve convention stands: an edge's bounds are
//! *derived from its vertices* — the authority is the vertex geometry.
//! The [`EdgeCurve`] nevertheless **stores** the parameter interval,
//! as a certified derived cache in exactly the carrier's sense: the
//! endpoint-pinning checks of every certification pin
//! `carrier(t₀)`/`carrier(t₁)` to the endpoint points within ε, the
//! span checks enforce the ratified forward direction (t₁ > t₀) and —
//! for circle carriers — the at-most-one-period winding bound, and the
//! Intersection mid-parameter pin ties the interval's interior to the
//! stored witness; all at attachment and again at tier 3. Storing
//! the certified interval is what keeps periodic carriers total
//! (a full-period scaffolding edge's `(0, τ)` is not recoverable from
//! its coincident endpoints) and keeps bound recovery evaluation-free
//! (no atan2 branch selection). It can never disagree with the vertices
//! by more than ε without failing loudly — a cache, never a peer.

use geom::Curve3;
use geom::Surface;
use geom_core::period::{Exact, Extent, NotForward, Period, Turn, TurnRows, TurnUndecided, Within};
use geom_core::spline::SpanLocate;
use geom_core::{
    Band, BandError, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign,
};

use crate::description::{
    ChartCurve, CurveSelector, EdgeAuthority, EdgeDescription, EdgeDescriptionSpec, Structural,
    SurfacePair, authority_of,
};
use crate::dihedral::{
    DihedralClass, WedgeEscalation, decide_positive, decide_reported, wedge_decided,
};
use crate::implicit::implicit_residual;
use crate::keys::SurfaceKey;
use crate::pcurve_cache::{Pcurve, PcurveCertifyError, PcurveCheck, chart_pcurve_over};
use crate::recourse::{
    Classified, ReadAt, Refused, RefusedArm, RefusedSide, SizedDecision, SizedPass, SpanDecision,
    Unsized,
};

// **The sized decisions' nouns, each spelled once.** A size noun is the
// head its decision's ending names ("if this {size} is intended"); every
// check word and refusal that tells the decision composes it, through
// these literals for `concat!`.
macro_rules! length_noun {
    () => {
        "length"
    };
}
macro_rules! arc_noun {
    () => {
        "arc"
    };
}
macro_rules! angle_noun {
    () => {
        "angle"
    };
}
macro_rules! curvature_noun {
    () => {
        "curvature difference"
    };
}
macro_rules! headroom_noun {
    () => {
        "headroom"
    };
}
macro_rules! full_turn {
    () => {
        "one full turn of its carrier"
    };
}
macro_rules! crossing_angle {
    () => {
        concat!("the crossing ", angle_noun!())
    };
}
macro_rules! curvature_difference {
    () => {
        concat!("the ", curvature_noun!())
    };
}
pub(crate) use {arc_noun, length_noun};

/// The fixed certification sample count (module docs): 9 uniform
/// parameters, endpoints included.
pub const CERT_SAMPLES: u32 = 9;

/// The `sample` a [`CertifyError`] carries when the failing check is
/// **not a sampled one** — currently the chart-image mint, which runs
/// once, before the schedule.
///
/// It is a value no schedule index can take ([`CERT_SAMPLES`] counts
/// 0…8), so a reader can tell "this check has no sample" from "this
/// check failed at sample 0". Reporting `0` there states a schedule
/// point that was never visited, which is a fabricated diagnostic
/// however small it looks.
///
/// The interval checks ([`CertCheck::ParamSpan`]) carry it for the
/// same reason the mint does: the span decision runs ONCE, before the
/// schedule exists.
pub const NOT_A_SAMPLE: u32 = u32::MAX;

/// Which certification check a [`CertifyError`] names — the residual
/// taxonomy, one variant per documented check.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CertCheck {
    /// The stored parameter interval runs forward over a clear length,
    /// metered as arc length (and a spline carrier's own meter is
    /// positive) — named on
    /// escalations of that decision; its definite failure is
    /// [`CertifyError::IntervalNotForward`], which every public door
    /// reports as its input's [`CertifyError::StatedInterval`].
    ParamSpan,
    /// The lower bound on a spline carrier's length from its knot domain,
    /// metered at its certified speed floor, is definitely positive —
    /// the metre scale [`CertCheck::ParamSpan`] converts the stored
    /// interval by; its definite failure is
    /// [`CertifyError::SpanMeterCollapsed`].
    ParamSpanMeter,
    /// A periodic carrier's stored arc is not over one full turn — short
    /// of one or a whole one ([`geom_core::period::Period::turn`]) — named
    /// on escalations of that decision; its definite failure is
    /// [`CertifyError::WindingExceeded`], which every public door reports
    /// as its input's [`CertifyError::StatedInterval`].
    ParamWinding,
    /// The edge's chart image on a spline wall (an arc rim's exact image,
    /// [`crate::pcurve_cache::PcurveCheck::ParamSpan`]) runs forward over
    /// a clear length — [`CertCheck::ParamSpan`]'s decision on the image, named
    /// on its escalations.
    ImageSpan,
    /// `|carrier(t₀) − start point|` (check 1).
    EndpointStart,
    /// `|carrier(t₁) − end point|` (check 2).
    EndpointEnd,
    /// Intersection: implicit residual against `surface`, one of the
    /// pair, at a sample.
    SurfaceResidual {
        /// The surface the residual is measured against.
        surface: SurfaceKey,
    },
    /// Intersection: the witness point's residual against `surface`,
    /// one of the pair.
    WitnessSurfaceResidual {
        /// The surface the residual is measured against.
        surface: SurfaceKey,
    },
    /// Intersection: `|carrier((t₀ + t₁)/2) − witness|` — the witness
    /// **is** the edge's mid-parameter point (the M2 PR 3 fix-pass
    /// sharpening of the witness contract): together with endpoint
    /// pinning and the circle winding bound this pins *which* arc of
    /// the intersection locus the interval traverses, and in which
    /// direction — the component/side selection the 9 surface-residual
    /// samples alone cannot see (every point of the wrong arc also
    /// lies on both surfaces).
    WitnessMidpoint,
    /// Intersection: the crossing angle at an interior sample, levered
    /// to the dihedral displacement margin — must be definitely
    /// transverse.
    Transversality,
    /// Intersection, rung 3: the uniqueness tube's lower bound on the
    /// crossing angle over its box chain, its certified clearance from
    /// zero — `0` wherever an enclosure straddles, so it never shows the
    /// angle's own size or sign.
    TransversalityTube,
    /// Intersection: the folded lever arm the transversality margin is
    /// metered over at an interior sample, which must be definitely
    /// positive before any angle is read there
    /// ([`crate::DIHEDRAL_ARM`]).
    TransversalityArm,
    /// Intersection: the two surfaces' tangent planes the
    /// transversality margin compares at an interior sample, which must
    /// exist: each surface's implicit gradient nonzero and defined over
    /// the sample point's enclosure. Named when the margin is undefined
    /// because one is not (a point on a cylinder's axis, a sphere's
    /// centre, or an enclosure reaching one). A cone's apex never reaches
    /// this check: the arm gate refuses there first (`dihedral_arm`).
    TangentPlanes,
    /// TangentIntersection: the normal-parallelism defect at an
    /// interior sample — `sin θ` metered at the lever arm `1/κ_rel`
    /// (D2's derived angular threshold ε·κ_rel; C7 jet schedule), or,
    /// where the curvature difference refuses and so no such lever
    /// exists, at the folded lever arm, as the cause of that refusal.
    /// The folded-arm reading also names a refused orientation of the
    /// shared normal ([`CertCheck::TangentGap`]) at any sample, ends
    /// included.
    TangentParallel,
    /// TangentIntersection: the two surfaces' separation along their
    /// shared normal at a sample, `s = r₁ − σ·r₂` (σ the sign of the
    /// two implicit gradients' alignment) — the touch itself, a fact
    /// about the pair rather than the carrier (C7).
    TangentGap,
    /// TangentIntersection: the curvature difference at an interior
    /// sample — the relative transverse normal curvature `|κ_rel|`
    /// metered as the displacement it induces at the folded lever arm
    /// (D4 ¶1), which must be definitely positive (the jet system's
    /// IFT denominator).
    TangentSecondOrder,
    /// TangentIntersection: the C2.2 between-samples statements — the
    /// worst sampled residual plus the certified quadratic sag bound
    /// (`tangent_hull_sup`), and the worst sampled separation plus the
    /// sum of both surfaces' sags (`tangent_gap_hull_sup`), each within
    /// ε (a sup bound, never a sampled max pretending to be one).
    TangentHull,
    /// TangentIntersection: the C2.3-style tube statement on the JET
    /// system — the sampled `|κ_rel|` minus its certified
    /// between-samples drift stays definitely positive over every
    /// span, so the second-order separation (and with it local
    /// uniqueness of the locus) holds along the WHOLE edge, not just
    /// at the schedule points.
    TangentTube,
    /// Scaffold: `|carrier(t_i) − description(s_i)|` at a sample —
    /// the fenced scaffolding arm's meter, the one conventional
    /// residual that has no chart to state itself against (D3).
    MappedSource,
    /// The **mint step** of the collapsed conventional description
    /// (D4): deriving the chart image the meter is stated against.
    /// Not a sampled check — like [`CertCheck::ParamSpan`] it runs
    /// once, and the `sample` field of a [`CertifyError`] naming it
    /// carries the module's not-a-sample sentinel rather than an
    /// index that was never visited.
    ChartImage,
    /// **The class reading** of the mint step: which of the chart's own
    /// curves the carrier is, the lane choice between a closed-form
    /// image and the general one (D3). It records nothing and ends as no
    /// defect: where the class is not shown and no general lane is
    /// written for the pair, the refusal is lever-only "not yet"
    /// ([`crate::recourse::not_yet`]).
    ChartClass,
    /// **The incidence reading** at the mint step: whether the carrier
    /// lies on the chart its description names. A definite miss ends by
    /// the door that reads it, and a read in band in its lever at every
    /// door ([`crate::IncidenceRead::ending`]).
    ChartIncidence,
    /// **The general lane's own reading** at the mint step, where the
    /// class reading sent an on-chart carrier there and the lane cannot
    /// image this pose yet ([`crate::PcurveCheck::GeneralLane`]): "not
    /// yet", as [`CertCheck::ChartClass`].
    GeneralLane,
    /// **The unified conventional meter** (D1): `|C(tᵢ) − S(P(tᵢ))|`
    /// at a sample, C4 verbatim — the ONE statement every collapsed
    /// conventional description makes, whatever certification lane its
    /// [`crate::Pcurve`] belongs to.
    ChartResidual,
    /// Intersection, plane × NURBS (M7-8): limb 1's on-locus residual
    /// over the schedule's samples, on either operand — the closed-form plane
    /// distance or the certified foot distance on the wall.
    PlaneNurbsOnLocus,
    /// Intersection, plane × NURBS (M7-8): limb 2's certified
    /// **sup-norm** bound over the whole span — the number that
    /// certifies (a bound, never a sampled max).
    PlaneNurbsHull,
    /// Intersection, limb 2 against a NURBS operand:
    /// `|S(P(t)) − C(t)|` at a break of limb 2's composite — the
    /// carrier against the surface point its pcurve names, which bounds
    /// the carrier's miss from the surface from above (C2 limb 1). It is
    /// [`CertCheck::PlaneNurbsHull`]'s quantity read at a point, so past
    /// the band it shows that bound cannot clear, and like it it is a
    /// bound on the miss, never the miss.
    PlaneNurbsHullValue,
    /// Intersection, limb 2 against an analytic operand: the carrier's
    /// residual against it at a break of limb 2's composite — limb 1's
    /// own residual, measured where the composite's bound did not clear:
    /// the implicit form linearized to metres at the search, and a
    /// certified lower bound on the carrier's distance from the surface
    /// at the edge certificate (`crate::analytic_edge_limbs`).
    AnalyticBreakResidual,
    /// Intersection of two analytic surfaces: the carrier's certified
    /// distance bound from an operand over the edge's whole interval
    /// (`crate::analytic_edge_limbs`'s limb 2, for a carrier of any
    /// kind) — a bound on the miss, never a sampled one.
    AnalyticHull,
    /// Intersection, plane × NURBS (M7-8): the lane's reported
    /// transversality, the minimum sine over the interior samples that
    /// each decided transverse — named when that aggregate is poisoned,
    /// which no geometry and no tolerance reaches.
    PlaneNurbsReportedTransversality,
    /// Intersection, plane × NURBS: the spline face's chart speed along
    /// a parameter direction, which must be positive for a length in
    /// metres to cross into its parameters — zero where the face is
    /// constant along that direction.
    PlaneNurbsChartSpeed,
    /// Intersection, plane × NURBS: the bound on the spline face's chart
    /// speed along a parameter direction, which must be finite — not,
    /// where the face's net is too large for its derivative bound.
    PlaneNurbsChartSpeedBound,
}

/// The check's own name — the noun a refusal about it writes (the
/// three check-naming arms of [`CertifyError`]'s `Display`).
///
/// Written here, on the declaring row, rather than at the refusal: the
/// taxonomy's words are the type's to say once, not each consumer's to
/// re-derive from a variant identifier. The match is exhaustive with no
/// wildcard, so a check the taxonomy gains has no word until someone
/// writes one.
///
/// **Each word is the phrase a person would write, not the variant
/// identifier**, and the sentence around it is what decides that. Every
/// other arm of [`CertifyError`]'s `Display` is English prose about the
/// geometry, so a CamelCase identifier standing in the neighbouring
/// sentence is the odd spelling, not the house one. These rows are also
/// not doors anyone calls: a reader told
/// `EndpointStart` has nothing to reach for with that word, which is
/// what makes an identifier the right rendering where the vocabulary
/// names doors (`verbs`' `VerbKind`) and the wrong one
/// here. The typed field is what a programmatic consumer matches on;
/// this is the sentence for the person reading it.
///
/// **The word carries the KIND of quantity the check meters**, because
/// the sentence cannot. Not every check that reaches
/// [`CertifyError::ResidualExceeded`] meters a residual: some meter a
/// sup bound ([`CertCheck::bounds_a_miss`], today
/// [`CertCheck::TangentHull`]) and [`CertCheck::TangentParallel`] a
/// parallelism defect. A noun owned by the sentence is a noun the
/// sentence cannot get right for every check that reaches it.
///
/// **A sized decision is told in one noun.** Its word names the
/// quantity its definite refusals tell their margin of, on the size its
/// ending names ("if this {size} is intended"), and every spelling
/// composes those nouns from one home: the undecided arm renders through
/// the word, so a word in another noun would name one decision two ways.
impl core::fmt::Display for CertCheck {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // The surface rides the typed field, not the words: a refusal
        // names what it is about without an arena key.
        f.write_str(match self {
            Self::SurfaceResidual { .. } => "the residual against one of the edge's surfaces",
            Self::WitnessSurfaceResidual { .. } => {
                "the witness point's residual against one of the edge's surfaces"
            }
            Self::ParamSpan => {
                return write!(f, "{STORED_INTERVAL}'s {}", SpanDecision::Forward.size());
            }
            Self::ParamWinding => STORED_ARC_HEADROOM,
            Self::ImageSpan => {
                return write!(
                    f,
                    "the {} of the edge's image on its wall's chart",
                    SpanDecision::Forward.size()
                );
            }
            Self::ParamSpanMeter => SPAN_METER,
            Self::EndpointStart => "the start-endpoint residual",
            Self::EndpointEnd => "the end-endpoint residual",
            Self::WitnessMidpoint => "the witness-midpoint residual",
            Self::Transversality => CROSSING_ANGLE,
            Self::TransversalityTube => TUBE_CROSSING_BOUND,
            Self::TransversalityArm => crate::DIHEDRAL_ARM_NOUN,
            Self::TangentPlanes => "the surfaces' tangent planes",
            Self::TangentParallel => "the normal-parallelism defect",
            Self::TangentGap => "the surfaces' separation along their shared normal",
            Self::TangentSecondOrder => CURVATURE_DIFFERENCE,
            Self::TangentHull => "the between-samples sag bound",
            Self::TangentTube => TUBE_CURVATURE_BOUND,
            Self::MappedSource => "the mapped-source residual",
            Self::ChartImage => "the chart-image mint",
            Self::ChartClass => "the chart-image class reading",
            Self::ChartIncidence => "the carrier's incidence on its chart",
            Self::GeneralLane => "the chart image's general lane",
            Self::ChartResidual => "the unified conventional residual",
            Self::PlaneNurbsOnLocus => "the spline Intersection lane's on-locus residual",
            Self::PlaneNurbsHull => "the spline Intersection lane's sup-norm bound",
            Self::PlaneNurbsHullValue => {
                "the carrier against the surface point its pcurve names, at a break: a \
                 bound on the carrier's distance from the surface"
            }
            Self::AnalyticBreakResidual => {
                "the carrier's linearized residual against an analytic surface, at a break"
            }
            Self::AnalyticHull => "the carrier's certified distance bound from an analytic surface",
            Self::PlaneNurbsReportedTransversality => {
                concat!(
                    "the spline Intersection lane's reported minimum crossing ",
                    angle_noun!()
                )
            }
            Self::PlaneNurbsChartSpeed => "the spline face's chart speed",
            Self::PlaneNurbsChartSpeedBound => "the spline face's chart-speed bound",
        })
    }
}

/// Typed certification failure (D4 ¶3): actionable, closed enum. The
/// body-side attachment gates (`topo`'s operators and setters) wrap
/// this; the tier-3 validator carries it inside its own error.
///
/// A refused decision carries what its classifier saw: the reporting
/// margin ([`geom_core::MarginDiag`], every lane's own view, `Dual`'s by
/// delegation to its value part) rides [`CertifyError::Escalated`]'s
/// [`Indeterminate`] and every band-decided verdict ([`Refused`]), for
/// the words of the refusal only. A residual's sign-certain exceedance
/// names the check and sample and carries no magnitude: the residual
/// decisions are unsized, so no number changes their ending.
///
/// [`CertifyError::decision`] hands a door the verdict as structure,
/// so an ending it composes for its own reading reads the decision
/// rather than a rendered string.
#[derive(Clone, Copy, Debug, PartialEq)]
// The variant roster `topo`'s sample-coverage row reads (this
// crate's `test-support` feature, test builds only).
#[cfg_attr(
    feature = "test-support",
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(CertifyErrorKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum CertifyError {
    /// The collapsed conventional description's **chart image could
    /// not be derived** for this (chart, carrier) pair (D4's mint
    /// door, `crate::chart_pcurve`).
    ///
    /// This is a statement about the GEOMETRY, not about a missing
    /// feature: the named carrier is not a locus of any certified
    /// image class of the named chart — an ellipse on a cone, say,
    /// which no cone chart image can be. It is deliberately distinct
    /// from [`CertifyError::Unimplemented`], which means a described
    /// surface or carrier is of a kind this module refuses wholesale;
    /// collapsing the two would answer "not implemented" to a caller
    /// whose description is simply wrong.
    ChartImageUnavailable {
        /// The chart kind the description named.
        chart: geom::SurfaceKind,
        /// The carrier kind offered against it.
        carrier: geom::CurveKind,
    },
    /// The carrier is not shown off the chart and no closed form images
    /// it, and its general image is not available
    /// ([`PcurveCertifyError::NoGeneralLane`]), for the reason `gap` names:
    /// no general lane is written for this (chart, carrier) pair
    /// ([`crate::LaneGap::Unwritten`], [`CertCheck::ChartClass`]), or the
    /// written lane does not image this pose yet ([`crate::LaneGap::Pose`],
    /// [`CertCheck::GeneralLane`]). Either ends lever-only "not yet", never
    /// as the description's defect.
    NoGeneralLane {
        /// The chart kind the description named.
        chart: geom::SurfaceKind,
        /// The carrier kind offered against it.
        carrier: geom::CurveKind,
        /// Why there is no general image ([`crate::LaneGap`]).
        gap: crate::LaneGap,
    },
    /// The carrier is not shown on the chart its description names
    /// ([`PcurveCertifyError::CarrierOffChart`], [`CertCheck::ChartIncidence`]):
    /// decided off it, or read within the band at a pose where the chart
    /// holds no curve of its kind.
    CarrierOffChart {
        /// The chart kind the description named.
        chart: geom::SurfaceKind,
        /// The carrier's kind.
        carrier: geom::CurveKind,
        /// Why the mint concluded it, in the mint's words
        /// ([`PcurveCertifyError::CarrierOffChart`]'s `why`).
        why: &'static str,
        /// The margin the incidence decision read, where it read one, and
        /// which side of the band it fell ([`crate::OffMargin`]).
        margin: Option<crate::OffMargin>,
    },
    /// A surface key in the description did not resolve in the owning
    /// body (stale, or the surface does not exist yet — attach the
    /// intrinsic description once its surfaces are in the arena).
    UnresolvedSurface {
        /// The unresolved key.
        key: SurfaceKey,
    },
    /// A described surface is the `Nurbs` kind (no implicit form —
    /// its residual story is the SSI foot-point machinery, not this
    /// module's), or a `Nurbs` carrier arrived under a conventional
    /// (`MappedCurve`/`Seam`) description.
    ///
    /// THREE `Intersection` rungs certify, and this variant is what is
    /// left over. The analytic rung: both operands analytic (M5 PR 9,
    /// C12.3 — the class the curved-boolean zip mints). The **plane ×
    /// NURBS** rung (M7-8): exactly one PLANE and one described NURBS
    /// wall, declare-and-check, through the [`NurbsLane`].
    /// The **NURBS × NURBS** rung: two spline charts, through the same
    /// lane. A spline beside a curved analytic surface has none.
    Unimplemented,
    /// A two-surface description (`Intersection`, `TangentIntersection`,
    /// `OnPair`) names one surface twice — a locus on one surface is an
    /// image in its chart, never a pair's.
    IntersectionSameSurface {
        /// The doubly-named key.
        key: SurfaceKey,
    },
    /// A wrap flag on a chart image of a non-periodic surface (a
    /// plane): its chart closes in no direction, so nothing wraps.
    WrapOnNonPeriodic,
    /// The stored parameter interval is not forward: t₁ − t₀, metered
    /// as arc length, did not classify positive. The ratified
    /// vertices-derive-bounds convention — increasing parameter runs
    /// start → end of `he_plus` — is violated by a definitely reversed
    /// interval, a sign-certain refusal.
    ///
    /// A **zero** verdict is band-decided, like every sized decision's
    /// (D4 ¶1 (i)), and what it says depends on the span's own margin.
    /// A span `0 < m ≤ ε` is a real edge shorter than the tolerance, a
    /// size a user may intend, and a tolerance below `m/K` decides it
    /// forward. A span `m ≤ 0` — none, or reversed within the tolerance
    /// — has no size to tighten below. Every public door reports one as
    /// its input's ([`CertifyError::StatedInterval`]), so none returns
    /// this verdict on such a span; built by hand it ends as that refusal
    /// does at a build, in the lever
    /// ([`crate::recourse::RefusedSide::Unminted`]), and at rest it is a
    /// defect of the kernel or the file.
    IntervalNotForward {
        /// The span's verdict, with its reporting margin.
        verdict: Refused,
    },
    /// A spline carrier's knot domain, metered at its certified speed
    /// floor, did not classify positive, so the stored interval has no
    /// metre scale to be measured by. The floor is a conservative lower
    /// bound on the carrier's speed: it reaches zero or below where the
    /// spline stalls or turns back on itself, and where the bound itself
    /// falls short. The knot domain is never the cause: a
    /// [`geom_core::spline::KnotVector`] is strictly forward by
    /// construction, so this verdict is always the floor's.
    SpanMeterCollapsed {
        /// The metered length's verdict, with its reporting margin.
        verdict: Refused,
    },
    /// A periodic carrier's stored interval (a circle's, an ellipse's, a
    /// spiric's) spans definitely more than one full period beyond
    /// tolerance. This closes the 9-sample winding alias (M2 PR 3
    /// fix pass, S1): the uniform schedule `s = i/8` is aliased exactly
    /// by intervals wrong by `8kτ` (the per-sample discrepancy
    /// `(Δ − θ)·i/8` vanishes mod τ iff `Δ = θ + 8kτ`), and with
    /// `0 < Δt ≤ τ` enforced no `k ≠ 0` alias is representable — the
    /// pointwise sample matches then pin the winding *within* the
    /// period. Every public door reports one as its input's
    /// ([`CertifyError::StatedInterval`]), so none returns this refusal;
    /// built by hand it ends as that refusal does at a build, in the
    /// lever ([`crate::recourse::RefusedSide::Unminted`]), and at rest it
    /// is a defect of the kernel or the file.
    WindingExceeded,
    /// **The interval a door was handed runs no clear length forward, or
    /// past one full turn**: the span checks' reading is certainly on
    /// their refused side ([`CertCheck::ParamSpan`],
    /// [`CertCheck::ParamWinding`]), and at a public door
    /// ([`EdgeCurve::certify`], [`EdgeCurve::recertify`] and their `_via`
    /// doors) that is its input — a zero, reversed or past-a-turn
    /// `param_start … param_end`, a chord between coincident points, a
    /// circle or ellipse carrier of no positive radius, or a stored arc
    /// re-certified at a band finer than the one it was minted at, where
    /// its headroom read as zero. At a build it ends in the check's
    /// lever; at the import door it ends as any reading of the check's
    /// refused side does there. Where the input is data the kernel
    /// holds, the kernel's call site panics on it instead: a kernel bug.
    StatedInterval {
        /// What the span checks refused, with the reading they refused it
        /// on.
        refused: StatedSpan,
    },
    /// A residual definitely exceeded the escalation threshold: the
    /// cache does not represent the description (D4 ¶2's `residual ≤ ε`
    /// kernel invariant violated beyond doubt).
    ResidualExceeded {
        /// The check that failed.
        check: CertCheck,
        /// The schedule sample index (0…[`CERT_SAMPLES`]−1; 0 for the
        /// witness/endpoint checks).
        sample: u32,
        /// What the classifier saw of the miss, for error reporting only
        /// ([`MarginDiag`]): it rides [`CertifyError::decision`]'s
        /// sign-certain arm into [`Unsized::residual_in_file`], the import
        /// door's words on a definite miss.
        margin: MarginDiag,
    },
    /// `Intersection` only: the levered angle between the tangent planes
    /// did not classify positive at a sample — the transversality
    /// precondition fails, so the locus is not an
    /// `Intersection` (a tangential contact is `TangentIntersection`
    /// territory; a seam is `Seam`).
    NotTransverse {
        /// The interior sample index.
        sample: u32,
        /// Which length levered the angle, on the plane × NURBS lane
        /// ([`crate::PlaneNurbsRefusal::NotTransverse`]). `None` on the
        /// analytic lane, whose arm is the dihedral's folded one
        /// ([`crate::folded_lever_arm`]).
        lever: Option<crate::ssi::PointLever>,
        /// The verdict on the levered angle, with its reporting margin.
        verdict: Refused,
    },
    /// `Intersection` only: the folded lever arm the transversality
    /// margin is metered over was decided not positive at a sample — the
    /// faces' extent there, clamped by how tightly they curve (a cone at
    /// its apex), leaves no width to measure an angle between them over —
    /// the definite arm of [`CertCheck::TransversalityArm`], whose
    /// undecided arm is [`CertifyError::Escalated`].
    ArmCollapsed {
        /// The interior sample index.
        sample: u32,
        /// The arm's verdict, with the margin its ending quotes.
        verdict: Refused,
    },
    /// `TangentIntersection` only: the curvature difference (relative
    /// transverse normal curvature, metered at the folded lever arm)
    /// did not classify positive at a sample — the surfaces
    /// under-determine the locus there (a G2 conventional join's
    /// zero-side margin, or an osculating patch), so the intrinsic
    /// tangent description is not certifiable; the honest description is
    /// conventional (`MappedCurve`), exactly D2's G2-join split. The
    /// **definite** half of a two-tolerance pair: one band-width away the
    /// same margin escalates as [`CertifyError::Escalated`] under
    /// [`CertCheck::TangentSecondOrder`] instead (F6 — an osculating pair
    /// is a sliver at this ε).
    NotSecondOrderSeparated {
        /// The sample index.
        sample: u32,
        /// The verdict on the curvature difference, with its reporting
        /// margin and, on the zero arm, the band (the two-tolerance
        /// pair's shared frame).
        verdict: Refused,
    },
    /// `TangentIntersection` only: the uniqueness tube's margin — the
    /// sampled `|κ_rel|` less its certified drift over the spans, as a
    /// sagitta at the folded arm — did not classify positive. The margin
    /// is a conservative LOWER bound on a non-negative quantity, so
    /// neither verdict is a stored contradiction: the certificate could
    /// not prove separation along the edge.
    TubeNotSeparated {
        /// `Zero` where the lower bound is within the zero band;
        /// `Negative` where the drift definitely exceeds the sampled
        /// separation; with its reporting margin.
        verdict: Refused,
    },
    /// `TangentIntersection` only: the (carrier kind, surface kinds)
    /// triple is outside the jet certificate's certified span-bound
    /// lane (`Line` carriers on `Plane`/`Cylinder`/`Sphere` pairs —
    /// the class the C5 tangent arms mint at M5). A routing boundary
    /// (C12.1: per-class retirement with its proof), never a runtime
    /// fallback.
    TangentCertificateUnsupported,
    /// A classification escalated: the margin landed in the sliver band
    /// or was poisoned (D4 ¶3's escalate-never-guess).
    ///
    /// It renders the classifier's payload; the door that reports it
    /// appends the one ending its decision gives every refused arm
    /// ([`CertifyError::ending`], keyed by `check`). Certification takes
    /// no declaration, so no ending offers one.
    Escalated {
        /// The check that escalated.
        check: CertCheck,
        /// The schedule sample index.
        sample: u32,
        /// The classifier's diagnostic (band + margin view).
        cause: Indeterminate,
    },
    /// The linear band could not be built from the run's tolerance
    /// (absurd ε — see `Band::linear`'s error docs).
    Band(BandError),
    /// `Intersection` with a described spline wall on one side (a
    /// plane) or both: the declare-and-check lane refused, carrying its
    /// own measured number. The carrier was adopted as EVIDENCE; a limb
    /// 2 refusal bounds it through the lane's own image of it on a
    /// spline wall ([`crate::edge_nurbs::PlaneNurbsRefusal::Limb`]).
    PlaneNurbs(crate::edge_nurbs::PlaneNurbsRefusal),
    /// `Intersection` of two analytic surfaces: C2's limbs (the
    /// carrier's distance from each operand, for every carrier kind, and
    /// the uniqueness tube, for a spline carrier) refused, carrying
    /// their own verdict ([`crate::analytic_edge_limbs`]).
    AnalyticRung3(crate::edge_nurbs::AnalyticRung3Refusal),
    /// `Intersection` under a shared row: the row refused on `surface`,
    /// one of the edge's walls ([`RowRefusal`]).
    Row {
        /// The wall the row refused on.
        surface: SurfaceKey,
        /// What refused.
        refusal: RowRefusal,
    },
}

/// Why a shared row refused on one of its walls
/// ([`CertifyError::Row`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RowRefusal {
    /// The wall's certificate of its own row image refused definitely, at
    /// its check (`None` where the refusal was no check's): the row is not
    /// the edge on that wall.
    Image(Option<crate::pcurve_cache::PcurveCheck>),
    /// The wall's certificate of its own row image escalated, at its
    /// check.
    ImageEscalated {
        /// The row certificate's check that escalated.
        check: crate::pcurve_cache::PcurveCheck,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// The row is not on a side of the wall's chart domain. The shared
    /// row is built for boundary rows; a structural selector for any
    /// other row or column known in advance (D2) has no certificate yet.
    OffSide,
    /// The wall's chart has no speeds to read the walls' crossing along
    /// the row in ([`crate::ssi::ChartSpeedRefusal`]).
    ChartSpeed(crate::ssi::ChartSpeedRefusal),
}

impl RowRefusal {
    /// The decision this refusal is a refused arm of, `None` for the
    /// structural [`RowRefusal::OffSide`]. The row image is an `IsoArc`'s
    /// or an `IsoLine`'s certificate, so its check reads as an arc rim's
    /// does ([`arc_rim_check`]): definite on the incidence limb it names
    /// (the hull bound for one that names none), undecided at the same
    /// routing; a chart speed reads as the spline lanes' does.
    #[must_use]
    pub fn decision(&self) -> Option<(CertCheck, RefusedArm<'_>)> {
        Some(match self {
            Self::Image(check) => (
                check.map_or(CertCheck::PlaneNurbsHull, arc_rim_check),
                RefusedArm::SignCertain(None),
            ),
            Self::ImageEscalated { check, cause } => {
                (arc_rim_check(*check), RefusedArm::Undecided(cause))
            }
            Self::ChartSpeed(r) => (r.check(), RefusedArm::SignCertain(None)),
            Self::OffSide => return None,
        })
    }
}

/// **The walls cross along the whole row**, read over a cover of it
/// ([`crate::ssi::row_crossing`]). Where the reading turns, the walls'
/// tangent planes coincide at a state of the row: a zero crossing, decided
/// as the exact zero it is under its own name (`shared_row_turn`),
/// definite. Otherwise the least margin, levered over the edge's extent as
/// the tube's is, is a lower bound only: it passes clear of the band, and
/// in the band escalates, since a lower bound there tells no tangency from
/// a crossing too shallow to read. A cover with an unread piece is fed to
/// the same gate as the invalid margin it is, which escalates.
fn row_crossing_verdict<T: Decide>(
    crossing: crate::ssi::RowCrossing,
    extent: T,
    band: Band,
) -> Result<(), CertifyError> {
    let escalated = |cause| CertifyError::Escalated {
        check: CertCheck::Transversality,
        sample: NOT_A_SAMPLE,
        cause,
    };
    let least = match crossing {
        crate::ssi::RowCrossing::Turns => {
            let zero = Margin::levered(T::from_computed(0.0), extent);
            let decided = crate::dihedral::decide_reported("shared_row_turn", zero, band)
                .map_err(escalated)?;
            return Err(CertifyError::NotTransverse {
                sample: NOT_A_SAMPLE,
                lever: None,
                verdict: Refused::Zero(crate::recourse::Classified {
                    margin: decided.margin,
                    band,
                }),
            });
        }
        crate::ssi::RowCrossing::Clear(least) => least,
        crate::ssi::RowCrossing::Unread => f64::NAN,
    };
    let margin = Margin::levered(T::from_computed(least), extent);
    crate::dihedral::decide_positive("shared_row_crossing", margin, band).map_err(escalated)
}

/// A shared row's certificate on `surface` refused: escalated or definite,
/// named as that wall's row check.
fn row_image_refusal(surface: SurfaceKey, refusal: PcurveCertifyError) -> CertifyError {
    let refusal = match refusal {
        PcurveCertifyError::Escalated { check, cause, .. } => {
            RowRefusal::ImageEscalated { check, cause }
        }
        refused => RowRefusal::Image(refused.refused_check()),
    };
    CertifyError::Row { surface, refusal }
}

/// The certification check a chart image's `check` restates where an
/// exact image on a spline chart is refused at it: an arc rim's exact
/// image escalated ([`CertifyError::Escalated`]), or a shared row's image
/// on one of its walls refused ([`RowRefusal::decision`]). Its incidence
/// limb's ([`crate::pcurve_cache::PcurveCheck::incidence_limb`]), and
/// the image's span decision for the span check ([`CertCheck::ImageSpan`]),
/// not the edge's, whose interval it does not decide. A check no `IsoArc`
/// or `IsoLine` certificate runs reads as the hull bound it would feed.
#[must_use]
pub fn arc_rim_check(check: crate::pcurve_cache::PcurveCheck) -> CertCheck {
    match check {
        crate::pcurve_cache::PcurveCheck::ParamSpan => CertCheck::ImageSpan,
        other => other
            .incidence_limb()
            .map_or(CertCheck::PlaneNurbsHull, crate::ssi::SsiLimb::check),
    }
}

impl core::fmt::Display for CertifyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnresolvedSurface { key } => {
                write!(f, "surface key {key:?} does not resolve")
            }
            Self::ChartImageUnavailable { chart, carrier } => write!(
                f,
                "a conventional description on a {} chart has no \
                 certified chart image for a {} carrier — the locus this \
                 description claims is not one this chart can state",
                chart.name(),
                carrier.name()
            ),
            Self::NoGeneralLane {
                chart,
                carrier,
                gap: crate::LaneGap::Unwritten,
            } => write!(
                f,
                "a conventional description's {} carrier is on its {} chart but is none of \
                 the chart's own curves, and its general image on that chart is not written yet",
                carrier.name(),
                chart.name(),
            ),
            Self::NoGeneralLane {
                chart,
                carrier,
                gap: crate::LaneGap::Pose { piece, channel },
            } => write!(
                f,
                "a conventional description's {} carrier is on its {} chart but is none of \
                 the chart's own curves, and its general image on that chart does not yet cover \
                 this pose: piece {piece}: {}. Recourse: {}",
                carrier.name(),
                chart.name(),
                channel.describe(),
                crate::pcurve_cache::POSE_LEVER,
            ),
            Self::CarrierOffChart {
                chart,
                carrier,
                why,
                margin,
            } => match crate::IncidenceRead::of_margin(*margin) {
                // Not shown off the chart: read AT a pose where the chart
                // holds no curve of its kind, which says nothing about
                // whether it lies on it.
                crate::IncidenceRead::AtPose(_) => write!(
                    f,
                    "the {} is no curve of the {} chart its description names at this \
                     tolerance: it sits, within the band, at a pose where that chart holds no {} \
                     ({why})",
                    carrier.name(),
                    chart.name(),
                    carrier.name()
                ),
                crate::IncidenceRead::Off(_) => write!(
                    f,
                    "the {} does not lie on the {} chart its description names ({why})",
                    carrier.name(),
                    chart.name()
                ),
                crate::IncidenceRead::Undecided(_) => {
                    unreachable!("a decided incidence refusal carries no undecided read")
                }
            },
            Self::Unimplemented => write!(
                f,
                "a Nurbs described surface, or a Nurbs carrier under a \
                 conventional description, cannot be certified in this build — rung-3 \
                 carriers certify as the Intersection of two analytic surfaces, or of \
                 one plane and one described NURBS wall, or of two spline charts, through \
                 the declare-and-check lane; a spline beside a curved analytic surface has \
                 no certificate"
            ),
            Self::IntersectionSameSurface { key } => write!(
                f,
                "a two-surface description names surface {key:?} twice (a locus \
                 on one surface is an image in its chart)"
            ),
            Self::WrapOnNonPeriodic => write!(
                f,
                "wrap edge described on a non-periodic surface (a plane's \
                 chart closes in no direction)"
            ),
            Self::IntervalNotForward { verdict } => {
                StatedSpan::Forward(ForwardReading::Decided(*verdict)).tell(STORED_INTERVAL, f)
            }
            Self::SpanMeterCollapsed { verdict } => write!(
                f,
                "{SPAN_METER} is {} zero (margin {} m), so the stored interval has no proven \
                 metre scale",
                verdict.worded("not shown above", "below"),
                verdict.margin().readable()
            ),
            Self::ArmCollapsed { sample, verdict } => write!(
                f,
                "at sample {sample} the edge is {} {} (margin {} m)",
                verdict.worded("not shown", "not"),
                crate::DIHEDRAL_ARM_CLAUSE,
                verdict.margin().readable()
            ),
            Self::WindingExceeded => {
                StatedSpan::Winding(WindingReading::Exceeded).tell(STORED_INTERVAL, f)
            }
            Self::StatedInterval { refused } => write!(f, "{refused}"),
            // The check says its own noun ([`CertCheck`]'s `Display`);
            // this sentence decides only the grammar around it. Not
            // every check that reaches this arm meters a residual (a sup
            // bound, `CertCheck::bounds_a_miss`; a parallelism defect),
            // so the noun is not the sentence's to write.
            Self::ResidualExceeded { check, sample, .. } => {
                if *sample == NOT_A_SAMPLE {
                    write!(f, "{check} (not a sampled check)")?;
                } else {
                    write!(f, "{check} at sample {sample}")?;
                }
                let claim = if check.bounds_a_miss() {
                    "the certificate cannot show the cache represents the description"
                } else {
                    "the cache does not represent the description"
                };
                write!(f, " definitely exceeds the tolerance band ({claim}, D4 ¶2)")
            }
            // An arc rim's refusal is told in its own words: the plane
            // lane's `OnPair` reads one as well as the `Intersection` lanes.
            Self::PlaneNurbs(refusal @ crate::edge_nurbs::PlaneNurbsRefusal::ArcRim(_)) => {
                write!(f, "{refusal}")
            }
            Self::PlaneNurbs(refusal) => {
                write!(f, "the spline Intersection lane refused — {refusal}")
            }
            Self::AnalyticRung3(refusal) => write!(
                f,
                "the analytic Intersection's rung-3 certificate refused — {refusal}"
            ),
            Self::Row {
                surface,
                refusal: RowRefusal::OffSide,
            } => write!(
                f,
                "the shared row on surface {surface:?} is not on a side of its chart's domain: \
                 the shared row is built for boundary rows, and D2's structural selector for \
                 any other row or column known in advance has no certificate yet"
            ),
            Self::Row {
                surface,
                refusal: RowRefusal::Image(Some(check)),
            } => write!(
                f,
                "the shared row's image on surface {surface:?} is not the edge: its \
                 certificate refused at its {check:?} check"
            ),
            Self::Row {
                surface,
                refusal: RowRefusal::Image(None),
            } => write!(
                f,
                "the shared row's image on surface {surface:?} is not the edge: its \
                 certificate refused"
            ),
            Self::Row {
                surface,
                refusal: RowRefusal::ImageEscalated { check, cause },
            } => write!(
                f,
                "the shared row's image on surface {surface:?} could not be certified: its \
                 {check:?} check was undecided — {cause}"
            ),
            Self::Row {
                surface,
                refusal: RowRefusal::ChartSpeed(r),
            } => write!(
                f,
                "the walls' crossing along the shared row could not be read on surface \
                 {surface:?}: {}",
                r.what()
            ),
            Self::NotTransverse {
                sample,
                lever,
                verdict,
            } => {
                write!(
                    f,
                    "at sample {sample} the faces {} cross (margin {} m of {CROSSING_ANGLE}",
                    verdict.worded("are not shown to", "do not"),
                    verdict.margin().readable()
                )?;
                match lever {
                    Some(lever) => write!(f, ", levered by {lever}")?,
                    None => write!(f, ", levered by {FOLDED_ARM}")?,
                }
                write!(f, "), though the edge's description says they do")
            }
            Self::NotSecondOrderSeparated { sample, verdict } => write!(
                f,
                "at sample {sample} the faces {} curve apart at second order (margin {} m of \
                 {CURVATURE_DIFFERENCE}, levered by {FOLDED_ARM}), so they {} fix where the edge \
                 runs, though its description says they do",
                verdict.worded("are not shown to", "do not"),
                verdict.margin().readable(),
                verdict.worded("might not", "cannot")
            ),
            Self::TubeNotSeparated { verdict } => write!(
                f,
                "along the whole edge {TUBE_CURVATURE_BOUND} is {} zero (margin {} m, levered by \
                 {FOLDED_ARM}), so the faces are not proved to fix where it runs, though its \
                 description says they do",
                verdict.worded("not shown above", "below"),
                verdict.margin().readable()
            ),
            Self::TangentCertificateUnsupported => write!(
                f,
                "this (carrier, surface-pair) class is outside the jet \
                 certificate's certified span-bound lane — the certified class is Line \
                 carriers on Plane/Cylinder/Sphere pairs (classes retire one at a time, \
                 each with its proof; no runtime fallback)"
            ),
            Self::Escalated {
                check: CertCheck::TangentPlanes,
                sample,
                ..
            } => write!(
                f,
                "{} at sample {sample} are undefined: {TANGENT_PLANES_UNDEFINED}",
                CertCheck::TangentPlanes
            ),
            Self::Escalated {
                check,
                sample,
                cause,
            } => {
                // The not-a-sample sentinel renders as words, not as
                // 4294967295: a diagnostic whose whole purpose is to
                // stop claiming a schedule point it never visited
                // should not then print a number that looks like one.
                if *sample == NOT_A_SAMPLE {
                    write!(f, "{check} (not a sampled check) escalated: ")?;
                } else {
                    write!(f, "{check} at sample {sample} escalated: ")?;
                }
                write!(f, "{}", cause.payload())
            }
            Self::Band(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for CertifyError {}

impl CertifyError {
    /// The decision this refusal is a refused arm of, and which arm, or
    /// `None` for a refusal that is no decision's (an unresolved key, an
    /// unimplemented kind, a band the tolerance cannot form).
    ///
    /// This is the verdict as structure: a door that composes its own
    /// ending reads it here, with the arm's reporting margin, rather
    /// than from a rendered string.
    #[must_use]
    pub fn decision(&self) -> Option<(CertCheck, RefusedArm<'_>)> {
        Some(match self {
            Self::ResidualExceeded { check, margin, .. } => {
                (*check, RefusedArm::SignCertain(Some(*margin)))
            }
            Self::NotTransverse { verdict, .. } => (CertCheck::Transversality, verdict.arm()),
            Self::NotSecondOrderSeparated { verdict, .. } => {
                (CertCheck::TangentSecondOrder, verdict.arm())
            }
            Self::TubeNotSeparated { verdict } => (CertCheck::TangentTube, verdict.arm()),
            Self::IntervalNotForward { verdict } => (CertCheck::ParamSpan, verdict.arm()),
            Self::SpanMeterCollapsed { verdict } => (CertCheck::ParamSpanMeter, verdict.arm()),
            Self::ArmCollapsed { verdict, .. } => (CertCheck::TransversalityArm, verdict.arm()),
            Self::WindingExceeded => (CertCheck::ParamWinding, RefusedArm::SignCertain(None)),
            Self::Escalated { check, cause, .. } => (*check, RefusedArm::Undecided(cause)),
            Self::ChartImageUnavailable { .. } => {
                (CertCheck::ChartImage, RefusedArm::SignCertain(None))
            }
            Self::NoGeneralLane {
                gap: crate::LaneGap::Unwritten,
                ..
            } => (CertCheck::ChartClass, RefusedArm::SignCertain(None)),
            Self::NoGeneralLane {
                gap: crate::LaneGap::Pose { .. },
                ..
            } => (CertCheck::GeneralLane, RefusedArm::SignCertain(None)),
            Self::CarrierOffChart { margin, .. } => (
                CertCheck::ChartIncidence,
                RefusedArm::SignCertain(margin.map(crate::OffMargin::diag)),
            ),
            Self::PlaneNurbs(refusal) => return refusal.decision(),
            Self::AnalyticRung3(refusal) => return refusal.decision(),
            Self::Row { refusal, .. } => return refusal.decision(),
            Self::UnresolvedSurface { .. }
            | Self::Unimplemented
            | Self::IntersectionSameSurface { .. }
            | Self::WrapOnNonPeriodic
            | Self::TangentCertificateUnsupported
            | Self::StatedInterval { .. }
            | Self::Band(_) => return None,
        })
    }

    /// The ending this refusal's decision gives it, read at `at`
    /// ([`recourse`] over [`CertifyError::decision`]), or `None` for a
    /// refusal that is no decision's refused arm. The exceptions have no
    /// `decision()` and end by their own: the lanes' one-arc proof
    /// (`TubeNotOneArc`, [`crate::ssi::OneArcRefusal::ending`]), and a
    /// door's input interval ([`StatedSpan::ending`]).
    ///
    /// `Display` renders the payload alone: where a refusal is read
    /// decides its ending (D4 ¶1 (i)), so the door that reports it
    /// appends this, or renders both through [`CertifyError::render`].
    #[must_use]
    pub fn ending(&self, at: impl Into<ReadAt>) -> Option<String> {
        let at = at.into();
        match self {
            Self::PlaneNurbs(refusal) => refusal.ending(at),
            Self::AnalyticRung3(refusal) => refusal.ending(at),
            Self::StatedInterval { refused } => Some(refused.ending(at)),
            // A carrier's incidence ends by what was read, through the one
            // map every incidence ending goes through
            // ([`crate::IncidenceRead::ending`]).
            _ if let Some(read) = self.incidence_read() => Some(read.ending(at)),
            _ => self.decision().map(|(check, arm)| recourse(check, arm, at)),
        }
    }

    /// **The incidence read this refusal carries**
    /// ([`crate::IncidenceRead`]): a carrier off its chart, read by its
    /// margin, an incidence reading escalated, or the test's residual
    /// exceeded; `None` for a refusal of anything else. It agrees arm for
    /// arm with the pcurve layer's classifier ([`crate::IncidenceRead::of`]),
    /// which every incidence refusal reaches it through, and is the one
    /// reading of it [`Self::ending`] and a door's classification share.
    #[must_use]
    pub fn incidence_read(&self) -> Option<crate::IncidenceRead> {
        match self {
            Self::CarrierOffChart { margin, .. } => Some(crate::IncidenceRead::of_margin(*margin)),
            Self::Escalated {
                check: CertCheck::ChartIncidence,
                cause,
                ..
            } => Some(crate::IncidenceRead::Undecided(*cause)),
            // The test's residual decided past the band: a definite miss,
            // read as the pcurve layer's (`IncidenceRead::of`) reads it.
            // Its margin is the residual, not a pose's height, so it is
            // not quoted as one.
            Self::ResidualExceeded {
                check: CertCheck::ChartIncidence,
                ..
            } => Some(crate::IncidenceRead::Off(None)),
            _ => None,
        }
    }

    /// The payload and, where its decision gives one, the ending read at
    /// `at` ([`CertifyError::ending`]).
    #[must_use]
    pub fn render(&self, at: impl Into<ReadAt>) -> String {
        self.with_ending(self.ending(at))
    }

    fn with_ending(&self, ending: Option<String>) -> String {
        match ending {
            Some(ending) => format!("{self}. {ending}"),
            None => self.to_string(),
        }
    }
}

/// Why the transversality margin is undefined when its tangent planes
/// are ([`CertCheck::TangentPlanes`]): the sentence its refusal says in
/// place of the margin's numbers, which read nothing.
const TANGENT_PLANES_UNDEFINED: &str = "a surface's gradient is zero or undefined somewhere over \
     the enclosure of the point there, so no tangent plane, and no angle between the surfaces, \
     is defined over it";

/// The recourse of [`CertCheck::TangentPlanes`]: a gradient vanishes on
/// a cylinder's axis and at a sphere's centre, and an enclosure reaches one
/// when the point and the surface's axis are enclosed apart by more
/// than the radius, as two images of one widened map are.
const TANGENT_PLANES_RECOURSE: &str = "keep the edge clear of each surface's axis or centre; over \
     a parameter box, a narrower box encloses the edge and its surfaces closer to where they are";

/// How one decision's refusals end: the one table [`recourse`] reads.
#[derive(Debug)]
enum Ending {
    /// A decision on a size the user may intend ([`SizedDecision`]).
    Sized(SizedDecision),
    /// A decision with no size ([`Unsized`]).
    Unsized(Unsized),
    /// A residual: a decision with no size that passes only at zero, so
    /// its refused margin is a miss ([`Unsized::residual_in_file`]).
    Residual(Unsized),
    /// A quantity undefined over the enclosure it was read on: the
    /// recourse says what is undefined and why, whatever the margin's
    /// numbers, which are not a reading of it.
    Undefined(&'static str),
    /// A lane choice with no general lane written yet (D3): lever-only
    /// "not yet" on every arm.
    NotYet,
    /// A carrier's incidence (D3): a definite miss ended by the door, a
    /// read in band by its lever ([`crate::IncidenceRead::ending`]). A
    /// refused arm does not say which read it was (off and at a pose both
    /// refuse sign-certain), so a refusal that carries its read ends by it
    /// ([`CertifyError::ending`]); the check alone reads it back off the
    /// arm ([`crate::IncidenceRead::arm_ending`]).
    Incidence,
}

/// **What a span check refused of a door's input**
/// ([`CertifyError::StatedInterval`]): the decision, each carrying the
/// reading it refused on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StatedSpan {
    /// [`CertCheck::ParamSpan`]'s decision refused.
    Forward(ForwardReading),
    /// [`CertCheck::ParamWinding`]'s decision refused.
    Winding(WindingReading),
}

/// The reading [`CertCheck::ParamSpan`] refused a door's input on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ForwardReading {
    /// Decided past the band ([`CertifyError::IntervalNotForward`]).
    Decided(Refused),
    /// In band wholly on the refused side ([`CertifyError::Escalated`]).
    InBand(Indeterminate),
}

/// The reading [`CertCheck::ParamWinding`] refused a door's input on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WindingReading {
    /// Decided past the band ([`CertifyError::WindingExceeded`]), which
    /// keeps no number.
    Exceeded,
    /// Undecided with its margin wholly on the refused side, an overlap
    /// past a turn ([`CertifyError::Escalated`]). A turn left unsettled
    /// with its margin not wholly over — a short edge whose two readings
    /// straddle the band — is no reading of the input's, and stays the
    /// check's own escalation.
    InBand(Indeterminate),
}

impl StatedSpan {
    /// The span check's refusal `error` is ([`CertCheck::span`]), where
    /// its refused side is one no kernel construction produces and its
    /// reading is certainly on that side.
    fn of(error: &CertifyError) -> Option<Self> {
        let refused = match *error {
            CertifyError::IntervalNotForward { verdict } => {
                Self::Forward(ForwardReading::Decided(verdict))
            }
            CertifyError::WindingExceeded => Self::Winding(WindingReading::Exceeded),
            CertifyError::Escalated {
                check: CertCheck::ParamSpan,
                cause,
                ..
            } => Self::Forward(ForwardReading::InBand(cause)),
            CertifyError::Escalated {
                check: CertCheck::ParamWinding,
                cause,
                ..
            } => Self::Winding(WindingReading::InBand(cause)),
            _ => return None,
        };
        let (decision, side @ RefusedSide::Unminted) = refused.check().span()? else {
            return None;
        };
        decision
            .on(side)
            .certainly_refused(refused.arm())
            .then_some(refused)
    }

    /// **A public door's reading of a refusal**: a span check's refusal
    /// certainly on its refused side is the door's input
    /// ([`CertifyError::StatedInterval`]); every other refusal stands.
    fn at_a_door(error: CertifyError) -> CertifyError {
        Self::of(&error).map_or(error, |refused| CertifyError::StatedInterval { refused })
    }

    /// The check that refused.
    #[must_use]
    pub const fn check(self) -> CertCheck {
        match self {
            Self::Forward(_) => CertCheck::ParamSpan,
            Self::Winding(_) => CertCheck::ParamWinding,
        }
    }

    /// The span decision that refused ([`CertCheck::span`] of
    /// [`StatedSpan::check`]).
    #[must_use]
    pub fn decision(self) -> SpanDecision {
        self.check().span_check().0
    }

    /// The refused arm its reading is.
    fn arm(&self) -> RefusedArm<'_> {
        match self {
            Self::Forward(ForwardReading::Decided(verdict)) => verdict.arm(),
            Self::Winding(WindingReading::Exceeded) => RefusedArm::SignCertain(None),
            Self::Forward(ForwardReading::InBand(cause))
            | Self::Winding(WindingReading::InBand(cause)) => RefusedArm::Undecided(cause),
        }
    }

    /// The ending it carries, read at `at` (D4 ¶1 (iv)): the check's
    /// lever at a build; at the import door, within ε_in the door's one
    /// sentence and the lever, and past it as the check's refused side
    /// is there.
    #[must_use]
    pub fn ending(&self, at: impl Into<ReadAt>) -> String {
        self.check().span_ending().recourse(self.arm(), at)
    }
}

/// The subject a refusal of the edge's own stored interval tells its span
/// decision of ([`CertifyError::IntervalNotForward`],
/// [`CertifyError::WindingExceeded`]).
const STORED_INTERVAL: &str = "the stored parameter interval";

/// The subject a door's refusal of its input tells its span decision of
/// ([`StatedSpan`]'s `Display`).
const GIVEN_INTERVAL: &str = "this edge's parameter interval";

/// What [`SpanDecision::Winding`]'s arc is short of.
const FULL_TURN: &str = full_turn!();

/// What [`SpanDecision::Winding`]'s margin measures on its arc: the
/// label of that margin in a refusal of it.
const HEADROOM: &str = headroom_noun!();

/// The quantity [`CertCheck::ParamWinding`] decides, on the arc its size
/// names: its word.
const STORED_ARC_HEADROOM: &str = concat!(
    "the stored ",
    arc_noun!(),
    "'s ",
    headroom_noun!(),
    " to ",
    full_turn!()
);

/// The quantity [`CertCheck::ParamSpanMeter`] decides: its word, and the
/// subject of [`CertifyError::SpanMeterCollapsed`].
const SPAN_METER: &str = concat!(
    "the lower bound on the spline carrier's ",
    length_noun!(),
    " from its metered knot domain"
);

/// The quantity [`CertCheck::Transversality`] decides: its word, and the
/// noun of its margin in [`CertifyError::NotTransverse`].
pub(crate) const CROSSING_ANGLE: &str = crossing_angle!();

/// The quantity [`CertCheck::TransversalityTube`] decides: its word, and
/// the subject of each refusal of it (`TubeStraddles` at the plane ×
/// NURBS lane, the analytic rung 3 and the SSI door).
pub(crate) const TUBE_CROSSING_BOUND: &str =
    concat!("the uniqueness tube's lower bound on ", crossing_angle!());

/// The quantity [`CertCheck::TangentSecondOrder`] decides: its word, and
/// the noun of its margin in [`CertifyError::NotSecondOrderSeparated`].
const CURVATURE_DIFFERENCE: &str = curvature_difference!();

/// The quantity [`CertCheck::TangentTube`] decides: its word, and the
/// subject of [`CertifyError::TubeNotSeparated`].
const TUBE_CURVATURE_BOUND: &str = concat!(
    "the second-order tube's lower bound on ",
    curvature_difference!()
);

/// What an analytic-lane margin of the crossing angle or the curvature
/// difference is in metres: the quantity levered by the arm it is read
/// over (the faces' folded arm, [`crate::folded_lever_arm`]).
const FOLDED_ARM: &str = "the arm it is read over";

impl StatedSpan {
    /// **The span decisions' one sentence**, told of `given`: a door's
    /// input ([`GIVEN_INTERVAL`]) or the stored interval
    /// ([`STORED_INTERVAL`]). The words come from the verdict, never its
    /// number: only a decided negative runs backwards by an amount.
    fn tell(self, given: &str, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Forward(ForwardReading::Decided(Refused::Negative { margin })) => write!(
                f,
                "{given} runs backwards by {} m",
                margin.readable_below_zero()
            ),
            Self::Forward(reading) => {
                let length = match reading {
                    ForwardReading::Decided(verdict) => verdict.margin(),
                    ForwardReading::InBand(cause) => cause.margin,
                };
                write!(
                    f,
                    "{given} is not shown to run forward ({} {} m)",
                    SpanDecision::Forward.size(),
                    length.readable()
                )
            }
            Self::Winding(WindingReading::Exceeded) => {
                write!(f, "{given} spans more than {FULL_TURN}")
            }
            Self::Winding(WindingReading::InBand(cause)) => write!(
                f,
                "{given} is not shown to stay within {FULL_TURN} ({HEADROOM} {} m)",
                cause.margin.readable()
            ),
        }
    }
}

impl core::fmt::Display for StatedSpan {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.tell(GIVEN_INTERVAL, f)
    }
}

/// The lever of both transversality decisions: the sampled angle and
/// the uniqueness tube's bound are moved by the same change.
const CROSS_LEVER: &str = "move the geometry so the surfaces cross at a clearer angle";

impl CertCheck {
    /// Whether this check refuses on a certified upper bound on a miss
    /// rather than on the miss: its refusal says only that the
    /// certificate cannot show the cache is the description.
    #[must_use]
    pub fn bounds_a_miss(self) -> bool {
        matches!(self.ending(), Ending::Residual(Unsized::Bound))
    }

    /// **The span decision this check reads, and the side it refuses
    /// on** — the one map from a check to a [`SpanDecision`].
    ///
    /// - The edge's interval ([`CertCheck::ParamSpan`],
    ///   [`CertCheck::ParamWinding`]): a side no kernel construction
    ///   produces. Every public door reports a reading certainly on it as
    ///   its input's ([`CertifyError::StatedInterval`]), and where that
    ///   input is data the kernel holds, the kernel's call site panics on
    ///   it as a kernel bug.
    /// - The span of the edge's image on a spline wall
    ///   ([`CertCheck::ImageSpan`]): a stored contradiction. The kernel
    ///   derives the image at the band the operation passes, where a
    ///   span its edge's mint decided forward can read in band at a
    ///   looser one.
    pub(crate) const fn span(self) -> Option<(SpanDecision, RefusedSide)> {
        match self {
            Self::ParamSpan => Some((SpanDecision::Forward, RefusedSide::Unminted)),
            Self::ParamWinding => Some((SpanDecision::Winding, RefusedSide::Unminted)),
            Self::ImageSpan => Some((SpanDecision::Forward, RefusedSide::StoredContradiction)),
            _ => None,
        }
    }

    /// [`CertCheck::span`] of a check the caller holds to be a span
    /// check; any other check here is a kernel bug.
    fn span_check(self) -> (SpanDecision, RefusedSide) {
        let Some(span) = self.span() else {
            unreachable!("{self:?} is a span check")
        };
        span
    }

    /// The decision a span check's refusal ends by: its span decision
    /// on its refused side.
    fn span_ending(self) -> SizedDecision {
        let (decision, side) = self.span_check();
        decision.on(side)
    }

    /// How this decision's refusals end: its own lever, and what it
    /// passes on, where it passes on a nonzero sign; otherwise the way
    /// its definite refusal ends.
    fn ending(self) -> Ending {
        match self {
            Self::ParamSpan | Self::ParamWinding | Self::ImageSpan => {
                Ending::Sized(self.span_ending())
            }
            // The floor is a conservative lower bound, so a definite
            // refusal of it is the certificate's limit, which the lever
            // reaches, never a stored contradiction.
            Self::ParamSpanMeter => Ending::Sized(SizedDecision {
                lever: "move the geometry so this spline edge runs steadily forward, never \
                        stalling or turning back",
                size: length_noun!(),
                passes: SizedPass::Positive,
                side: RefusedSide::Lever,
                no_size: Some(
                    "if it does neither, the speed floor has reached its limit, which is worth \
                     reporting",
                ),
            }),
            Self::Transversality => Ending::Sized(SizedDecision {
                lever: CROSS_LEVER,
                size: angle_noun!(),
                passes: SizedPass::Positive,
                side: RefusedSide::StoredContradiction,
                no_size: None,
            }),
            // The tube's clearance is a conservative lower bound, so a
            // definite refusal of it is the certificate's limit, which the
            // lever reaches, never a stored contradiction.
            Self::TransversalityTube => Ending::Sized(SizedDecision {
                lever: CROSS_LEVER,
                size: angle_noun!(),
                passes: SizedPass::Positive,
                side: RefusedSide::Lever,
                no_size: None,
            }),
            Self::TransversalityArm => Ending::Sized(crate::DIHEDRAL_ARM),
            Self::TangentPlanes => Ending::Undefined(TANGENT_PLANES_RECOURSE),
            Self::TangentSecondOrder => Ending::Sized(SizedDecision {
                lever: "move the geometry so the faces curve apart more clearly where they touch",
                size: curvature_noun!(),
                passes: SizedPass::Positive,
                side: RefusedSide::StoredContradiction,
                no_size: None,
            }),
            // The tube's margin is a conservative lower bound, so a
            // definite refusal of it is the certificate's limit, which
            // the lever reaches, never a stored contradiction.
            Self::TangentTube => Ending::Sized(SizedDecision {
                lever: "move the geometry so the faces curve apart more clearly where they touch",
                size: curvature_noun!(),
                passes: SizedPass::Positive,
                side: RefusedSide::Lever,
                no_size: None,
            }),
            // Exact constructions: the endpoints and witness the kernel
            // placed, the seam and the declared source it restates, the
            // closed-form chart image and its coefficients, the tangent
            // carrier's own normals, and the touch of the two surfaces a
            // tangency places (the gap reads the pair, not the carrier,
            // so no fit is in it).
            Self::EndpointStart
            | Self::EndpointEnd
            | Self::WitnessSurfaceResidual { .. }
            | Self::WitnessMidpoint
            | Self::TangentParallel
            | Self::TangentGap
            | Self::MappedSource
            | Self::ChartResidual => Ending::Residual(Unsized::Defect),
            Self::ChartImage => Ending::Unsized(Unsized::Defect),
            // A lane choice (D3): no defect and no tolerance on any arm.
            Self::ChartClass | Self::GeneralLane => Ending::NotYet,
            // A residual, not a class gate (D3), ended by its read.
            Self::ChartIncidence => Ending::Incidence,
            // A fold over samples that each decided transverse: a poison
            // that survives it is the kernel's.
            Self::PlaneNurbsReportedTransversality => Ending::Unsized(Unsized::Defect),
            // A face constant along a parameter direction is the face's
            // own degeneracy, and the stored face is what the lever
            // edits, so it ends in the lever at rest too.
            Self::PlaneNurbsChartSpeed => Ending::Sized(SizedDecision {
                lever: "move the geometry so the spline face varies along both of its \
                        parameter directions",
                size: "degenerate face",
                passes: SizedPass::Positive,
                side: RefusedSide::Lever,
                no_size: None,
            }),
            // An unbounded derivative is the face's scale: the lever is
            // the model's size.
            Self::PlaneNurbsChartSpeedBound => Ending::Sized(SizedDecision {
                lever: "move the geometry to a scale where the spline face's control points \
                        stay well inside the range of finite numbers",
                size: "scale",
                passes: SizedPass::Positive,
                side: RefusedSide::Lever,
                no_size: None,
            }),
            // Approximations: a fitted intersection carrier on its
            // surfaces, and the plane × NURBS lane's on-locus limb. The
            // surface residuals take the fit's ending for EVERY carrier,
            // the exact analytic ones too, where a miss would be a defect:
            // the routing reads the decision alone and cannot see which
            // kind of carrier it measured.
            Self::SurfaceResidual { .. }
            | Self::PlaneNurbsOnLocus
            | Self::AnalyticBreakResidual => Ending::Residual(Unsized::Fit),
            // Certified bounds on the miss: the tangent lane's sag bound,
            // the two hull limbs, and the NURBS hull's value at a break.
            Self::TangentHull
            | Self::PlaneNurbsHull
            | Self::PlaneNurbsHullValue
            | Self::AnalyticHull => Ending::Residual(Unsized::Bound),
        }
    }
}

/// The one ending a refusal of `check` carries on `arm`, read at `at`
/// (D4 ¶1 (i)): computed from the decision and its verdict, never from a
/// predicate name.
///
/// - A decision on a size (it passes on a nonzero sign) ends as
///   [`SizedDecision::recourse`] gives: its lever, the conditional
///   tolerance on a band-decided arm, and on a reading certainly on its
///   refused side what that side is ([`crate::recourse::RefusedSide`],
///   D4 ¶1 (iv)).
/// - A residual or a form selection has no size. Every arm ends in the
///   kernel-defect ending where the kernel built the geometry exactly
///   (the file's too, at rest). Where the kernel
///   approximated, an arm read at a build, and an undecided arm read at
///   rest, end in the last resort; a definite arm at rest ends in the
///   file's defect ending, since no loosening repairs a stored
///   contradiction. A certified bound on the miss ends in the last resort
///   on every arm: a loose bound contradicts nothing stored.
/// - At the STEP import door ([`ReadAt::File`]) the file's declared
///   coincidence distance picks the words: a sized decision's as
///   [`SizedDecision::recourse`] gives; a residual's miss within ε_in but
///   beyond ε, or a bound on it wholly within ε_in, names setting ε to
///   ε_in as a stopgap ([`Unsized::residual_in_file`]); and past ε_in a
///   defect ending names the file ([`ReadAt::culprit`]), sized or not.
#[must_use]
pub(crate) fn recourse(check: CertCheck, arm: RefusedArm<'_>, at: impl Into<ReadAt>) -> String {
    let at = at.into();
    match (check.ending(), at) {
        (Ending::Sized(sized), at) => sized.recourse(arm, at),
        (Ending::Residual(residual), ReadAt::File(file)) => residual.residual_in_file(arm, file),
        (Ending::Unsized(no_size) | Ending::Residual(no_size), at) => no_size.recourse(arm, at),
        (Ending::Undefined(recourse), _) => geom_core::lever_recourse(recourse, None),
        (Ending::NotYet, at) => crate::recourse::not_yet(arm, at),
        (Ending::Incidence, at) => crate::IncidenceRead::arm_ending(arm, at),
    }
}

/// The tangent lane's uniqueness tube: the sampled `|κ_rel|` minimum less
/// its certified drift, as a sagitta at the folded arm, must classify
/// positive. Both refusing verdicts are reachable: the drift is a bound
/// independent of the sampled minimum, so it can exceed it.
fn tube_separation<T: Decide>(
    kappa_min: T,
    kappa_drift: T,
    arm: T,
    band: Band,
) -> Result<(), CertifyError> {
    let tube = Margin::sagitta(kappa_min - kappa_drift, arm);
    match decide_reported("tangent_tube_margin", tube, band).map(|d| Refused::of(d, band)) {
        Ok(None) => Ok(()),
        Ok(Some(verdict)) => Err(CertifyError::TubeNotSeparated { verdict }),
        Err(cause) => Err(CertifyError::Escalated {
            check: CertCheck::TangentTube,
            sample: NOT_A_SAMPLE,
            cause,
        }),
    }
}

/// The uncertified input to [`EdgeCurve::certify`]: description,
/// carrier cache, and the carrier-parameter interval, exactly as the
/// construction prepared them. Plain data — the certified product is
/// [`EdgeCurve`].
#[derive(Clone, Debug)]
pub struct EdgeCurveSpec<T: Real> {
    /// The intensional description (authoritative), as the
    /// construction states it — [`EdgeDescriptionSpec`].
    pub description: EdgeDescriptionSpec<T>,
    /// The carrier cache to certify against it.
    pub carrier: Curve3<T>,
    /// Carrier parameter at `start(he_plus)` — the `he_plus` forward
    /// contract's t₀ (increasing parameter runs start → end).
    pub param_start: T,
    /// Carrier parameter at `end(he_plus)` — t₁.
    pub param_end: T,
}

impl<T: Real> EdgeCurveSpec<T> {
    /// The straight-chord SCAFFOLDING spec between two points:
    /// carrier the line from `p0` to `p1` (arc-length parameters
    /// `0 … |p1 − p0|`), description the honest pushforward — `p0`'s
    /// trajectory under the translation by `p1 − p0`
    /// ([`crate::MappedSource::ExtrudedPoint`] with the sketch origin
    /// placed at `p0`) — through the scaffolding door (D3).
    ///
    /// By calling this the caller asserts the edge's locus **is** the
    /// straight chord; a construction whose edge follows any other
    /// locus (an arc trajectory, a placed profile segment) builds its
    /// spec explicitly. Coincident endpoints yield a span of no length,
    /// which certification refuses as its caller's
    /// ([`CertifyError::StatedInterval`]).
    ///
    /// **The door is for TRANSIENT edges.** An edge built here that
    /// comes to rest between two faces must be re-stated where it
    /// rests — [`EdgeCurveSpec::at_rest_in_chart`] — or tier 3 refuses
    /// it (U2's transience fence).
    pub fn line_between(p0: Point3<T>, p1: Point3<T>) -> Self {
        use geom_core::{Affine3, Point2};
        let len = p0.distance(p1);
        Self {
            description: EdgeDescriptionSpec::Scaffold(crate::mapped::MappedCurve::whole(
                crate::mapped::MappedSource::ExtrudedPoint {
                    point: Point2::new(T::zero(), T::zero()),
                    place: Affine3::translation(p0 - Point3::origin()),
                    vec: p1 - p0,
                },
            )),
            carrier: Curve3::Line {
                origin: p0,
                dir: (p1 - p0) / len,
            },
            param_start: T::zero(),
            param_end: len,
        }
    }

    /// The ARC SCAFFOLDING spec along an existing CIRCLE carrier
    /// between the given parameters: the carrier and interval are kept
    /// verbatim, and the description is the honest pushforward —
    /// the start point's trajectory under the rotation about the
    /// carrier's own axis by the swept angle
    /// ([`crate::MappedSource::RevolvedPoint`], the same
    /// geometry-derived posture as [`Self::line_between`]'s
    /// `ExtrudedPoint`). Like every scaffold it describes a TRANSIENT
    /// edge: one that comes to rest is restated in a chart
    /// ([`Self::at_rest_in_chart`]), and an edge already at rest is
    /// re-homed by [`EdgeCurve::restated_in_chart`], which declares
    /// nothing the edge did not.
    ///
    /// `None` for a non-circle carrier: no other kind has this
    /// rotation pushforward, and the caller owns its own honest
    /// refusal there.
    pub fn arc_of_circle(carrier: Curve3<T>, t0: T, t1: T) -> Option<Self>
    where
        T: SpanLocate,
    {
        use geom_core::{Affine3, Point2, Point3};
        let Curve3::Circle { center, axis, .. } = carrier else {
            return None;
        };
        let start = carrier.eval(t0);
        Some(Self {
            description: EdgeDescriptionSpec::Scaffold(crate::mapped::MappedCurve::whole(
                crate::mapped::MappedSource::RevolvedPoint {
                    point: Point2::new(T::zero(), T::zero()),
                    place: Affine3::translation(start - Point3::origin()),
                    axis_origin: center,
                    axis_dir: axis,
                    angle: t1 - t0,
                },
            )),
            carrier,
            param_start: t0,
            param_end: t1,
        })
    }

    /// The straight SCAFFOLDING spec along an existing LINE carrier
    /// between the given parameters: carrier and interval kept verbatim,
    /// description the start point's trajectory under the translation to
    /// the end ([`crate::MappedSource::ExtrudedPoint`], as
    /// [`Self::line_between`] states it). `None` for a non-line carrier.
    pub fn segment_of_line(carrier: Curve3<T>, t0: T, t1: T) -> Option<Self>
    where
        T: SpanLocate,
    {
        use geom_core::{Affine3, Point2, Point3};
        let Curve3::Line { .. } = carrier else {
            return None;
        };
        let start = carrier.eval(t0);
        Some(Self {
            description: EdgeDescriptionSpec::Scaffold(crate::mapped::MappedCurve::whole(
                crate::mapped::MappedSource::ExtrudedPoint {
                    point: Point2::new(T::zero(), T::zero()),
                    place: Affine3::translation(start - Point3::origin()),
                    vec: carrier.eval(t1) - start,
                },
            )),
            carrier,
            param_start: t0,
            param_end: t1,
        })
    }

    /// The same spec with a SCAFFOLDING description re-stated as an
    /// image in `surface`'s chart, the pushforward demoted to the
    /// authority record it always was (U2 Q3). `wrap` says the edge is
    /// a wrap edge of a face on that chart (D1).
    ///
    /// This is the transience fence's one conversion (D3): the
    /// scaffolding constructors above describe a locus for an edge
    /// whose surfaces do not exist yet, and an edge that comes to REST
    /// between two faces has surfaces — so it is described where it
    /// rests, in a chart, and the sketch entity that DECLARED it (if
    /// any) is recorded rather than re-purposed as a description.
    /// Carrier and interval are untouched: only the description moves.
    ///
    /// **This is the scaffolding door's conversion, and NOTHING else.**
    /// On a spec that already names a chart it returns unchanged — and
    /// that means **both arguments are discarded**, `surface`
    /// included. It is idempotent in the only sense a construction
    /// needs (call it on whatever you built without first asking what
    /// that was) and it is emphatically NOT a way to re-home an image
    /// into a different chart: an already-described edge is re-homed
    /// by STATING the chart, `EdgeDescriptionSpec::chart(k)` plus
    /// `declared_by` where a declaration is carried.
    ///
    /// Spelled out because the silence bit this unit's own author: a
    /// test re-homed a strut with `restated_spec().at_rest_in_chart(
    /// other_chart, false)`, got the original chart back, and read the
    /// resulting assertion failure as a defect in the kernel. The
    /// `debug_assert` below turns that silence into a panic wherever
    /// assertions are on — which is every test binary, i.e. exactly
    /// where the mistake gets made.
    #[must_use]
    pub fn at_rest_in_chart(self, surface: SurfaceKey, wrap: bool) -> Self {
        debug_assert!(
            match &self.description {
                EdgeDescriptionSpec::Chart {
                    surface: already,
                    wrap: already_wrap,
                    ..
                } => *already == surface && *already_wrap == wrap,
                EdgeDescriptionSpec::OnPair { .. } => false,
                EdgeDescriptionSpec::Scaffold(_)
                | EdgeDescriptionSpec::Intersection { .. }
                | EdgeDescriptionSpec::TangentIntersection { .. } => true,
            },
            "at_rest_in_chart on a spec that already names a chart or a pair discards both \
             arguments; state the description instead of re-asking the scaffolding door"
        );
        let description = match self.description {
            EdgeDescriptionSpec::Scaffold(mc) => {
                let chart = if wrap {
                    EdgeDescriptionSpec::wrap(surface)
                } else {
                    EdgeDescriptionSpec::chart(surface)
                };
                chart.declared_by(mc)
            }
            other => other,
        };
        Self {
            description,
            ..self
        }
    }

    /// [`EdgeCurveSpec::at_rest_in_chart`]'s twin for an edge at rest
    /// between two DISTINCT surfaces that under-determine its locus: a
    /// SCAFFOLDING description re-stated as
    /// [`EdgeDescriptionSpec::OnPair`] over `pair`, the pushforward kept
    /// as the authority record. Carrier and interval are untouched.
    ///
    /// The same conversion and nothing else: every other spec returns
    /// unchanged, and one naming a chart or another pair is a caller
    /// asking the scaffolding door to re-home a described edge, which
    /// the `debug_assert` turns into a panic.
    #[must_use]
    pub fn at_rest_on_pair(self, pair: SurfacePair) -> Self {
        debug_assert!(
            match &self.description {
                EdgeDescriptionSpec::Scaffold(_)
                | EdgeDescriptionSpec::Intersection { .. }
                | EdgeDescriptionSpec::TangentIntersection { .. } => true,
                EdgeDescriptionSpec::OnPair { pair: already, .. } => *already == pair,
                EdgeDescriptionSpec::Chart { .. } => false,
            },
            "at_rest_on_pair re-states a scaffold; state the description instead of \
             re-asking the scaffolding door"
        );
        let description = match self.description {
            EdgeDescriptionSpec::Scaffold(mc) => EdgeDescriptionSpec::on_pair(pair).declared_by(mc),
            other => other,
        };
        Self {
            description,
            ..self
        }
    }

    /// A scaffold at rest where `rest` says it rests
    /// ([`crate::RestLocus`]): [`EdgeCurveSpec::at_rest_in_chart`] or
    /// [`EdgeCurveSpec::at_rest_on_pair`], as the one home decided.
    #[must_use]
    pub fn at_rest(self, rest: crate::RestLocus) -> Self {
        match rest {
            crate::RestLocus::Chart(surface) => self.at_rest_in_chart(surface, false),
            crate::RestLocus::Pair(pair) => self.at_rest_on_pair(pair),
        }
    }

    /// The canonical full-period self-loop spec at `p`: a unit circle
    /// through `p` (center `p + x̂`, axis `ẑ`, parameters `0 … τ`),
    /// described as `p`'s trajectory under a full revolution about
    /// that center.
    ///
    /// **Scaffolding convention**: null edges at `p` need *some*
    /// certified closed carrier during construction sequences whose
    /// real geometry arrives later (or never reaches rest — ring
    /// scaffolding is typically consumed by `kemr`). Two shapes
    /// qualify: self-loop edges (both endpoints one vertex — `mef`'s
    /// circular/lone sites), and null edges between two **distinct**
    /// bitwise-coincident vertices (sweep's zip closure; the book's
    /// `loopglue` does the same) — the endpoint-pin certification
    /// holds either way, because the full-period parameter interval
    /// makes both endpoint evaluations the *same* identity, forcing
    /// bitwise coincidence. This is that deterministic choice: honest
    /// data (the carrier truly is this circle and truly closes at
    /// `p`), deliberately arbitrary geometry — the stored parameter
    /// interval `(0, τ)` is exactly the certified-cache case the
    /// module docs cover.
    pub fn self_loop_circle_at(p: Point3<T>) -> Self {
        use geom_core::{Affine3, Point2, Vec3};
        let center = p + Vec3::unit_x();
        Self {
            description: EdgeDescriptionSpec::Scaffold(crate::mapped::MappedCurve::whole(
                crate::mapped::MappedSource::RevolvedPoint {
                    point: Point2::new(T::zero(), T::zero()),
                    place: Affine3::translation(p - Point3::origin()),
                    axis_origin: center,
                    axis_dir: Vec3::unit_z(),
                    angle: T::tau(),
                },
            )),
            carrier: Curve3::Circle {
                center,
                axis: Vec3::unit_z(),
                radius: T::one(),
                // u_ref points from the center back at p.
                u_ref: Vec3::new(T::zero() - T::one(), T::zero(), T::zero()),
            },
            param_start: T::zero(),
            param_end: T::tau(),
        }
    }
}

/// The certification record stored with a certified carrier: the
/// schedule that ran and the worst distance residual it observed
/// (meters). Byte-identical across replays of the same construction
/// (D9) — under test via its `Debug` form.
#[derive(Clone, Copy, Debug)]
pub struct Certificate<T: Real> {
    /// The sample count of the schedule that ran ([`CERT_SAMPLES`]).
    pub samples: u32,
    /// The maximum magnitude over every value the schedule decides as
    /// coincident with zero — the endpoint, surface, mapped-source,
    /// chart and witness residuals, and the tangent lane's parallelism
    /// defect and sag bound — together with the plane × NURBS lane's
    /// limb-1 on-locus maximum and limb-2 sup bound. Transversality
    /// margins (clearance margins, not residuals) and the analytic
    /// rung-3 limbs are not folded. Certified ≤ ε by construction.
    ///
    /// **This number may MOVE at the conventional arms across the U2
    /// collapse** (D2): the three pre-collapse forms did not measure
    /// the same thing, and a `Pcurve` is a function of the carrier's
    /// own parameter where the iso arm's `v` walked the schedule
    /// fraction. The move is measured per fixture AND PER DRIFT SCALE
    /// and pinned — see `D2_SWEEP` in this module's tests, whose two
    /// arms move in opposite directions with scale — never laundered
    /// by re-associating a meter to make a number match.
    pub max_residual: T,
}

/// **A certified edge's turn**: its parameter interval, its extent
/// decided forward from that interval, and where the extent stands
/// against one period ([`Within`]) — never over one, which does not
/// certify. Minted by check 2 alone, so a reader of a conic span reads
/// the interval and its extent from one source.
#[derive(Clone, Copy, Debug)]
pub struct CertifiedTurn<T> {
    params: (T, T),
    extent: Extent<T>,
    turn: Within,
}

impl<T: Copy> CertifiedTurn<T> {
    /// The certified interval `(param_start, param_end)`.
    pub fn params(&self) -> (T, T) {
        self.params
    }

    /// The edge's extent, `param_end − param_start`, decided forward.
    pub fn extent(&self) -> Extent<T> {
        self.extent
    }

    /// Where the extent stands against one period.
    pub fn turn(&self) -> Within {
        self.turn
    }
}

/// Check 2's forward row, every carrier's.
const FORWARD_ROW: &str = "interval_span_forward";

/// Check 2's winding rows: the headroom to one period, and the straddle
/// of a bounded rate's two readings.
const WINDING_ROWS: TurnRows<&str> =
    TurnRows::bounded("interval_span_winding", "interval_span_winding_straddle");

/// An edge carrier: the intensional description, its cached [`Curve3`]
/// carrier with its parameter interval and turn, and what the scalar's
/// door gave it to carry ([`Real::Carries`]): at a certifying scalar the
/// [`Certificate`] of the attachment-time run, and at a tangent-transport
/// scalar nothing. Constructible only by the scalar's own door
/// ([`EdgeCurve::certify`] where the scalar certifies,
/// [`EdgeCurve::uncertified`] where it never does), or from such a value
/// by a map that carries what it carries
/// ([`EdgeCurve::with_remapped_surfaces`],
/// [`EdgeCurve::with_chart_v_mirrored`]) — fields are private, so an
/// uncertified value at a certifying scalar is unrepresentable (D4 ¶2
/// made structural).
#[derive(Clone, Debug)]
pub struct EdgeCurve<T: Real> {
    /// U2's collapsed description — ONE conventional form, minted and
    /// metered at certification (D1/D4).
    description: EdgeDescription<T>,
    /// U2 Q3's per-edge authority record: who determined the locus.
    authority: EdgeAuthority<T>,
    carrier: Curve3<T>,
    param_start: T,
    param_end: T,
    /// Check 2's turn: the interval's extent against one period.
    turn: Option<CertifiedTurn<T>>,
    certificate: T::Carries<Certificate<T>>,
}

impl<T: Decide> EdgeCurve<T> {
    /// **Mints** `spec`: the canonical description, carrier and
    /// interval, which is construction at any scalar that decides —
    /// checks 1 and 2 of [`EdgeCurve::certify`] and the canonical
    /// description, in that door's order. No sample is read and nothing
    /// is certified, so the result is no edge: a certifying scalar's
    /// door certifies it ([`EdgeCurve::certify`]), and a
    /// tangent-transport scalar's door holds it as minted
    /// ([`EdgeCurve::uncertified`]).
    ///
    /// # Errors
    ///
    /// Check 1's and check 2's refusals, and the chart image's.
    pub fn mint(
        spec: EdgeCurveSpec<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        band: Band,
    ) -> Result<MintedEdge<T>, CertifyError> {
        let (description, turn) =
            mint_checks(&spec, &surfaces, band).map_err(StatedSpan::at_a_door)?;
        Ok(MintedEdge {
            authority: authority_of(&spec.description),
            description,
            carrier: spec.carrier,
            param_start: spec.param_start,
            param_end: spec.param_end,
            turn,
        })
    }
}

/// **A minted edge**: the canonical description, carrier and interval
/// [`EdgeCurve::mint`] constructed, and the turn check 2 decided. It is
/// no [`EdgeCurve`]: only a scalar's own door makes one of it.
#[derive(Clone, Debug)]
pub struct MintedEdge<T: Real> {
    description: EdgeDescription<T>,
    authority: EdgeAuthority<T>,
    carrier: Curve3<T>,
    param_start: T,
    param_end: T,
    turn: Option<CertifiedTurn<T>>,
}

impl<T: geom_core::TangentTransport> EdgeCurve<T> {
    /// **A tangent-transport scalar's edge** (DUAL-DESIGN DL1, DL3): the
    /// minted edge, carrying no certificate. A dual's door mints and
    /// never checks; what it measures is read against the input-identical
    /// certifying evaluation, which holds the certificate.
    #[must_use]
    pub fn uncertified(minted: MintedEdge<T>) -> Self {
        Self {
            description: minted.description,
            authority: minted.authority,
            carrier: minted.carrier,
            param_start: minted.param_start,
            param_end: minted.param_end,
            turn: minted.turn,
            certificate: T::uncertified(),
        }
    }
}

impl<T: Decide + geom_core::CertifiedBounds> EdgeCurve<T> {
    /// Certifies `spec` against the edge's endpoint points and the
    /// owning body's surfaces, returning the certified carrier: the
    /// mint ([`EdgeCurve::mint`]'s steps, in its order) and every check
    /// below, C2's limbs through `lane`. Bounded on the certification
    /// trait, so a scalar that may not certify (a dual, DL1) cannot
    /// name this door:
    ///
    /// ```compile_fail,E0599
    /// use geom_brep::{EdgeCurve, EdgeCurveSpec, NurbsLane};
    /// use geom_core::{Band, Dual64, Point3, Real};
    /// fn at<T: geom_core::Decide>(spec: EdgeCurveSpec<T>, p: Point3<T>, band: Band, lane: NurbsLane<T>) {
    ///     let _ = EdgeCurve::<T>::certify(spec, p, p, |_| None, T::from_f64(1.0), band, lane);
    /// }
    /// ```
    ///
    /// The row differs from the one below, which builds, in the bound
    /// alone: a door generic over a deciding scalar, dual included,
    /// cannot certify through a lane it was handed.
    ///
    /// ```
    /// use geom_brep::{EdgeCurve, EdgeCurveSpec, NurbsLane};
    /// use geom_core::{Band, Point3, Real};
    /// fn at<T: geom_core::Decide + geom_core::CertifiedBounds>(
    ///     spec: EdgeCurveSpec<T>,
    ///     p: Point3<T>,
    ///     band: Band,
    ///     lane: NurbsLane<T>,
    /// ) {
    ///     let _ = EdgeCurve::<T>::certify(spec, p, p, |_| None, T::from_f64(1.0), band, lane);
    /// }
    /// ```
    ///
    /// `start`/`end` are the points of `start(he_plus)` and
    /// `end(he_plus)` (the `he_plus` forward contract); `surfaces`
    /// resolves the description's arena keys (injected by the owning
    /// body — this layer never touches arenas, see [`crate::keys`]);
    /// `band` is the run's linear band (callers build it once at
    /// operation entry via `Band::linear(tol)`).
    ///
    /// The check sequence (fixed order, D9; every check's margin is
    /// meters against `band`):
    ///
    /// 1. Implementedness: described surfaces resolve and are not
    ///    `Nurbs`, except the described NURBS wall of a plane × NURBS
    ///    `Intersection` (M7-8, through the [`NurbsLane`]) and
    ///    the chart of an image the construction states; a `Nurbs`
    ///    carrier certifies only under an
    ///    `Intersection` description (the rung-3 class, M5 PR 9 —
    ///    its span is metered through `speed_lower_bound`, gated
    ///    definitely-positive by `nurbs_span_meter`); `Intersection`'s
    ///    two surfaces are distinct; `Seam`'s surface is periodic.
    /// 2. Interval span (M2 PR 3 fix pass): the stored interval is
    ///    **forward** — its arc length `(t₁ − t₀)` (`·r` for circle
    ///    carriers, metering radians into meters) is definitely
    ///    positive ([`CertifyError::IntervalNotForward`] otherwise —
    ///    decreasing *and* degenerate zero-span intervals refuse); for
    ///    periodic carriers additionally the extent is not over one
    ///    turn ([`geom_core::period::Period::turn`]: `(τ − Δt)` at the
    ///    circle's radius, or between an ellipse's or a spiric's speed
    ///    floor and its sup over the turn's defect;
    ///    [`CertifyError::WindingExceeded`] where it is over) — at most
    ///    one full period, which closes the 9-sample `8kτ` winding-alias
    ///    family (see that variant's docs) — and the certificate keeps
    ///    whether it is short of a turn or a whole one
    ///    ([`EdgeCurve::turn`]). In-band/poisoned span margins
    ///    escalate under [`CertCheck::ParamSpan`], an unsettled turn
    ///    under [`CertCheck::ParamWinding`]. The interval
    ///    is this door's input, so a reading certainly on either check's
    ///    refused side refuses as [`CertifyError::StatedInterval`], here
    ///    and at every re-certification ([`EdgeCurve::recertify`]). A spline
    ///    carrier's metre scale is decided first
    ///    ([`CertCheck::ParamSpanMeter`];
    ///    [`CertifyError::SpanMeterCollapsed`] where it is not there).
    /// 3. Endpoint pinning: `|carrier(t₀) − start| ≤ ε`,
    ///    `|carrier(t₁) − end| ≤ ε`.
    /// 4. Per-sample description residuals, samples i = 0…8 in order
    ///    (per-sample check order as listed in [`CertCheck`]). Every
    ///    sample is a carrier point, the two ends included: the
    ///    certificate bounds the carrier (D4 ¶2), and a vertex on the
    ///    surfaces does not put the carrier's end, up to ε from it by
    ///    check 3, on them.
    ///    - `Intersection`: implicit residual vs `s1`, then `s2`; at
    ///      interior samples (i = 1…7) the transversality margin
    ///      (definitely transverse required), levered at `reach`, the
    ///      faces' extent ([`crate::folded_lever_arm`]); a
    ///      `TangentIntersection`'s second-order arm folds the same
    ///      `reach`.
    ///    - `Scaffold` (a mapped source): `|carrier(t_i) − description(i/8)|`.
    /// 5. `Intersection`: the witness's implicit residuals vs both
    ///    surfaces, then the **mid-parameter pin**
    ///    `|carrier.mid_point(t₀, t₁) − witness| ≤ ε`
    ///    ([`CertCheck::WitnessMidpoint`]): the witness contract is
    ///    that the stored witness IS the edge's mid-parameter point —
    ///    constructors mint it as `carrier(mid)` (the upgrade helpers'
    ///    chord midpoint) — which pins the traversed arc and winding
    ///    direction between the pinned endpoints. Residual freedom
    ///    after checks 2–5: for circle carriers the interval is
    ///    determined up to a joint whole-period translation of both
    ///    ends (geometrically invisible); for lines it is fully
    ///    determined. *Which branch* of the intersection
    ///    locus the witness selects remains unverifiable before
    ///    marching exists (M3) — the mid-pin verifies the carrier
    ///    traverses the witness's arc, not that the witness sits on
    ///    the branch the modeler intended.
    /// 6. `Intersection` of two analytic surfaces, over a carrier of
    ///    any kind, through `lane`: C2's limbs over `[t₀, t₁]`
    ///    ([`crate::analytic_edge_limbs`]).
    ///
    /// # Errors
    ///
    /// The first failing check, as a typed [`CertifyError`] (D4 ¶3).
    pub fn certify(
        spec: EdgeCurveSpec<T>,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        reach: T,
        band: Band,
        lane: NurbsLane<T>,
    ) -> Result<Self, CertifyError> {
        Self::certify_stated(spec, start, end, surfaces, reach, band, lane, None)
    }

    /// [`EdgeCurve::certify`] with a chart image the caller already
    /// holds for the edge on one of its surfaces: the pcurve a mint
    /// stored there. An `OnPair` on a plane × NURBS pair whose wall is
    /// that surface certifies its wall side through that image as it
    /// stands, rather than one derived here
    /// ([`crate::plane_nurbs_incidence`]); any other description ignores
    /// it.
    ///
    /// # Errors
    ///
    /// As [`EdgeCurve::certify`].
    #[allow(clippy::too_many_arguments)] // `certify`'s, plus the stated image
    pub fn certify_stated(
        spec: EdgeCurveSpec<T>,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        reach: T,
        band: Band,
        lane: NurbsLane<T>,
        stated: Option<&StatedImage<T>>,
    ) -> Result<Self, CertifyError> {
        let (certificate, canonical, turn) =
            run_checks(&spec, start, end, &surfaces, reach, lane, stated, band)
                .map_err(StatedSpan::at_a_door)?;
        Ok(Self {
            authority: authority_of(&spec.description),
            description: canonical,
            carrier: spec.carrier,
            param_start: spec.param_start,
            param_end: spec.param_end,
            turn,
            certificate: T::carried(certificate),
        })
    }

    /// Re-runs the full certification of this carrier at `band`. Same
    /// checks, same schedule, same errors as [`EdgeCurve::certify`]: this
    /// edge and `band` are the door's input, so a span check's reading
    /// certainly on its refused side refuses as
    /// [`CertifyError::StatedInterval`] — an arc minted at a coarser band,
    /// whose headroom to a full turn that band read as zero, reads past
    /// one at a finer band. The stored certificate is not consulted
    /// (re-certification re-derives, it does not trust).
    ///
    /// # Errors
    ///
    /// As [`EdgeCurve::certify`].
    pub fn recertify(
        &self,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        reach: T,
        band: Band,
        lane: NurbsLane<T>,
    ) -> Result<Certificate<T>, CertifyError> {
        self.recertify_stated(start, end, surfaces, reach, band, lane, None)
    }

    /// [`EdgeCurve::recertify`] with the chart image the body stores
    /// for the edge on one of its surfaces, read as
    /// [`EdgeCurve::certify_stated`] reads it: re-certification
    /// meters the image the op certified, not one derived anew, so an
    /// edge the op admitted through its stored image (a split child of
    /// an arc rim, whose parent's image covers it) is not refused at
    /// rest through a different one.
    ///
    /// # Errors
    ///
    /// As [`EdgeCurve::recertify`].
    #[allow(clippy::too_many_arguments)] // `recertify`'s, plus the stated image
    pub fn recertify_stated(
        &self,
        start: Point3<T>,
        end: Point3<T>,
        surfaces: impl Fn(SurfaceKey) -> Option<Surface<T>>,
        reach: T,
        band: Band,
        lane: NurbsLane<T>,
        stated: Option<&StatedImage<T>>,
    ) -> Result<Certificate<T>, CertifyError> {
        run_checks(
            &self.spec(),
            start,
            end,
            &surfaces,
            reach,
            lane,
            stated,
            band,
        )
        .map(|(cert, _, _)| cert)
        .map_err(StatedSpan::at_a_door)
    }
}

/// A chart image a caller holds for an edge on one of its surfaces
/// ([`EdgeCurve::certify_stated`]).
#[derive(Clone, Debug)]
pub struct StatedImage<T: Real> {
    /// The surface the image is in.
    pub surface: SurfaceKey,
    /// The image.
    pub pcurve: crate::pcurve_cache::Pcurve<T>,
}

/// **The certification lane** — C2's limbs, which this module cannot
/// discharge from `T: Decide` alone, as a value.
///
/// Limbs 2 and 3 are C9 certification hull bounds and a spline wall's
/// foot point is a bracket read, so the honest derivations
/// ([`crate::plane_nurbs_limbs`], [`crate::analytic_edge_limbs`]) need
/// `T: Decide + Bounds + CertifiedEnclosure`, which a `Dual` does not
/// meet. [`EdgeCurve::certify`] and [`EdgeCurve::recertify`] take it as
/// a plain argument, and are themselves bounded on
/// [`geom_core::CertifiedBounds`]: every door that certifies holds one,
/// and a scalar that may not certify reaches no such door.
///
/// Its one constructor is [`NurbsLane::certified`], bounded on
/// [`geom_core::CertifiedBounds`], so holding a value of this type IS
/// the statement that the scalar it is parameterised by may certify,
/// and the limbs a door records are the ones the lane's functions
/// derived and decided: [`crate::plane_nurbs_limbs`],
/// [`crate::nurbs_pair_limbs`] and [`crate::plane_nurbs_incidence`]
/// (an `OnPair`'s incidence alone), with [`crate::analytic_edge_limbs`]'s
/// verdict. Its fields are private and no other constructor exists. A
/// scalar that may not certify cannot write the value:
///
/// ```compile_fail,E0599
/// use geom_brep::NurbsLane;
/// use geom_core::Dual64;
/// let _ = NurbsLane::<Dual64>::certified();
/// ```
///
/// and a caller at any scalar cannot hand a door limbs it made up — a
/// closure is not a lane:
///
/// ```compile_fail,E0308
/// use geom_brep::{EdgeCurve, EdgeCurveSpec, NurbsLane, PlaneNurbsLimbs, PlaneNurbsRefusal};
/// use geom::{Curve3, NurbsSurface, Surface};
/// use geom_core::{Band, Point3};
/// fn forge(
///     spec: EdgeCurveSpec<f64>,
///     ends: (Point3<f64>, Point3<f64>),
///     surfaces: impl Fn(geom_brep::keys::SurfaceKey) -> Option<Surface<f64>>,
///     band: Band,
/// ) {
///     // Honest limbs for some other pair, with the two limbs a door
///     // records zeroed.
///     let forged = |c: &Curve3<f64>, t: (f64, f64), p: &Surface<f64>, w: &NurbsSurface<f64>,
///                   e: f64, b: Band|
///      -> Result<PlaneNurbsLimbs<f64>, PlaneNurbsRefusal> {
///         let mut limbs = geom_brep::plane_nurbs_limbs(c, t, p, w, e, b)?;
///         limbs.on_locus_max = 0.0;
///         limbs.hull_sup = 0.0;
///         Ok(limbs)
///     };
///     let _ = EdgeCurve::certify(spec, ends.0, ends.1, surfaces, 1.0, band, &forged);
/// }
/// ```
///
/// The codes are statements beside the fences, not checks: stable
/// rustdoc verifies only that each block fails to build. The rows below
/// are what say each fails for the reason given: the first differs
/// from the Dual row in the scalar alone, and the second from the
/// forging row in the lane it hands the door alone.
///
/// ```
/// use geom_brep::NurbsLane;
/// let _ = NurbsLane::<f64>::certified();
/// ```
///
/// ```
/// use geom_brep::{EdgeCurve, EdgeCurveSpec, NurbsLane, PlaneNurbsLimbs, PlaneNurbsRefusal};
/// use geom::{Curve3, NurbsSurface, Surface};
/// use geom_core::{Band, Point3};
/// fn forge(
///     spec: EdgeCurveSpec<f64>,
///     ends: (Point3<f64>, Point3<f64>),
///     surfaces: impl Fn(geom_brep::keys::SurfaceKey) -> Option<Surface<f64>>,
///     band: Band,
/// ) {
///     // Honest limbs for some other pair, with the two limbs a door
///     // records zeroed.
///     let forged = |c: &Curve3<f64>, t: (f64, f64), p: &Surface<f64>, w: &NurbsSurface<f64>,
///                   e: f64, b: Band|
///      -> Result<PlaneNurbsLimbs<f64>, PlaneNurbsRefusal> {
///         let mut limbs = geom_brep::plane_nurbs_limbs(c, t, p, w, e, b)?;
///         limbs.on_locus_max = 0.0;
///         limbs.hull_sup = 0.0;
///         Ok(limbs)
///     };
///     let _ = forged;
///     let lane = NurbsLane::certified();
///     let _ = EdgeCurve::certify(spec, ends.0, ends.1, surfaces, 1.0, band, lane);
/// }
/// ```
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct NurbsLane<T: Real> {
    /// [`crate::plane_nurbs_limbs`] at `T`.
    limbs:
        fn(
            &Curve3<T>,
            (T, T),
            &Surface<T>,
            &geom::NurbsSurface<T>,
            T,
            Band,
        )
            -> Result<crate::edge_nurbs::PlaneNurbsLimbs<T>, crate::edge_nurbs::PlaneNurbsRefusal>,
    /// [`crate::nurbs_pair_limbs`] at `T`.
    pair_limbs:
        fn(
            &Curve3<T>,
            (T, T),
            &geom::NurbsSurface<T>,
            &geom::NurbsSurface<T>,
            T,
            Band,
        )
            -> Result<crate::edge_nurbs::PlaneNurbsLimbs<T>, crate::edge_nurbs::PlaneNurbsRefusal>,
    /// [`crate::analytic_edge_limbs`] at `T`.
    analytic_limbs: fn(
        &Curve3<T>,
        (T, T),
        &Surface<T>,
        &Surface<T>,
        Band,
    ) -> Result<(), crate::edge_nurbs::AnalyticRung3Refusal>,
    /// [`crate::plane_nurbs_incidence`] at `T`.
    incidence: fn(
        &Curve3<T>,
        (T, T),
        &Surface<T>,
        &Surface<T>,
        Option<&crate::pcurve_cache::Pcurve<T>>,
        Band,
    ) -> Result<
        crate::edge_nurbs::PlaneNurbsIncidence<T>,
        crate::edge_nurbs::PlaneNurbsRefusal,
    >,
    /// [`crate::curve_face::composite::crossings`] at `T`, its scalars
    /// crossed by their certified enclosures.
    composite: fn(
        &geom::NurbsCurve3<T>,
        (T, T),
        &Surface<T>,
        Band,
    ) -> Result<Vec<T>, crate::curve_face::composite::CompositeRefusal>,
    /// [`NurbsLane::spline_face_meetings`] at `T`.
    spline_face: fn(
        &Curve3<T>,
        (T, T),
        &geom::NurbsSurface<T>,
        Band,
    ) -> Result<
        Vec<crate::curve_face::spline_face::ChartMeeting<T>>,
        crate::curve_face::spline_face::SplineFaceRefusal,
    >,
    /// [`geom::NurbsSurface::project`] at `T`.
    surface_foot: fn(
        &geom::NurbsSurface<T>,
        Point3<T>,
    )
        -> Result<geom::SurfaceProjection<T>, geom::SurfaceProjectionInconclusive>,
    /// [`NurbsLane::chart_arm`] at `T`.
    chart_arm: fn(&geom::NurbsSurface<T>, Point3<T>, T) -> Result<f64, crate::ChartArmRefusal>,
}

impl<T: Decide + geom_core::CertifiedBounds> NurbsLane<T> {
    /// The certification lane, and the only constructor there is: the
    /// limbs' free functions instantiated at `T`.
    #[must_use]
    pub const fn certified() -> Self {
        Self {
            limbs: crate::edge_nurbs::plane_nurbs_limbs::<T>,
            pair_limbs: crate::edge_nurbs::nurbs_pair_limbs::<T>,
            analytic_limbs: crate::edge_nurbs::analytic_edge_limbs::<T>,
            incidence: crate::edge_nurbs::plane_nurbs_incidence::<T>,
            composite: Self::composite_crossings,
            spline_face: Self::spline_face_meetings,
            surface_foot: geom::NurbsSurface::project,
            chart_arm: Self::certified_chart_arm,
        }
    }

    /// [`crate::offset_meters::chart_arm`] at `T`, read where every
    /// control coordinate, weight and knot of the chart, the point and the
    /// reach are each one point at `T`. A wider value is a set of charts or
    /// points, whose cells this door does not enclose, so it reads as
    /// poison and no arm is read.
    fn certified_chart_arm(
        chart: &geom::NurbsSurface<T>,
        p: Point3<T>,
        reach: T,
    ) -> Result<f64, crate::ChartArmRefusal> {
        let point = |x: T| match x.certified_bracket() {
            Some((lo, hi)) if lo == hi => lo,
            _ => f64::NAN,
        };
        crate::offset_meters::chart_arm(&chart.map_scalar(point), p.map(point), point(reach))
    }

    /// [`crate::curve_face::composite::crossings`], each scalar crossed
    /// into certification arithmetic by its certified enclosure.
    fn composite_crossings(
        net: &geom::NurbsCurve3<T>,
        params: (T, T),
        surface: &Surface<T>,
        band: Band,
    ) -> Result<Vec<T>, crate::curve_face::composite::CompositeRefusal> {
        crate::curve_face::composite::crossings(
            net,
            params,
            surface,
            band,
            geom_core::Interval::from_certified,
        )
    }

    /// The curve × face table's spline-face arm: the walk
    /// ([`crate::curve_face::spline_face::isolate`]), in certification
    /// arithmetic on a NURBS carrier's ruled sheet
    /// ([`crate::curve_face::spline_face::SpanCarrier`]), then its meetings placed at `T`
    /// ([`crate::curve_face::spline_face::settle`]).
    fn spline_face_meetings(
        carrier: &Curve3<T>,
        params: (T, T),
        chart: &geom::NurbsSurface<T>,
        band: Band,
    ) -> Result<
        Vec<crate::curve_face::spline_face::ChartMeeting<T>>,
        crate::curve_face::spline_face::SplineFaceRefusal,
    > {
        use crate::curve_face::spline_face::{SpanCarrier, isolate, settle};
        let isolated = isolate(&SpanCarrier::of(carrier), params, chart, band)?;
        settle(carrier, chart, &isolated, band)
    }
}

/// **The spline foot** — the parameter of a point moved along a NURBS
/// carrier, read by a door that moves an endpoint along one (the offset
/// door's re-anchor). Newton's foot is a bracket read written at the
/// certified-bracket scalars ([`geom::NurbsCurve3::project_from_seed`]),
/// so its one constructor is bounded there and a scalar outside them
/// holds none.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct SplineFoot<T: Real> {
    foot: fn(
        &geom::NurbsCurve3<T>,
        Point3<T>,
        T,
    ) -> Result<geom::Projection3<T>, geom::ProjectionInconclusive>,
}

impl<T: geom_core::CertifiedBounds> SplineFoot<T> {
    /// The foot, instantiated at `T`.
    #[must_use]
    pub const fn certified() -> Self {
        Self {
            foot: Self::seeded_foot,
        }
    }

    /// [`geom::NurbsCurve3::project_from_seed`] seeded at the bracket
    /// midpoint of `seed`.
    fn seeded_foot(
        carrier: &geom::NurbsCurve3<T>,
        point: Point3<T>,
        seed: T,
    ) -> Result<geom::Projection3<T>, geom::ProjectionInconclusive> {
        let (lo, hi) = (seed.lo(), seed.hi());
        carrier.project_from_seed(point, lo + 0.5 * (hi - lo))
    }
}

impl<T: Real> NurbsLane<T> {
    /// The foot of `point` on a NURBS surface ([`geom::NurbsSurface::project`]):
    /// the projection carries its own distance and certifies nothing;
    /// the caller decides that distance.
    ///
    /// # Errors
    ///
    /// [`geom::SurfaceProjectionInconclusive`] when Newton's budget
    /// expires.
    pub fn surface_foot(
        self,
        surface: &geom::NurbsSurface<T>,
        point: Point3<T>,
    ) -> Result<geom::SurfaceProjection<T>, geom::SurfaceProjectionInconclusive> {
        (self.surface_foot)(surface, point)
    }

    /// **A certified lower bound on `chart`'s length scale at `p`, over
    /// `reach`** (`crate::offset_meters`' `chart_arm`, which names its
    /// slack): the largest radius within `reach` over which a certified
    /// bound on the curvature of the chart's cells stays at most its
    /// reciprocal. The true arm lies between it and `reach`, and
    /// [`crate::classify_seam`] brackets a spline face's arm between the
    /// two, so the slack, which has no bound, costs an escalation and never
    /// a class.
    ///
    /// # Errors
    ///
    /// [`crate::ChartArmRefusal`] where no bound is read: a chart the
    /// patch bound refuses, a C⁰ crease within `reach`, an unbounded
    /// curvature at `p` (a pole), or data wider than a point at `T`
    /// (`Poisoned`).
    pub fn chart_arm(
        self,
        chart: &geom::NurbsSurface<T>,
        p: Point3<T>,
        reach: T,
    ) -> Result<f64, crate::ChartArmRefusal> {
        (self.chart_arm)(chart, p, reach)
    }

    /// The lane's limbs for a declared carrier of any kind on a plane ×
    /// NURBS pair, over the edge's interval `params`.
    fn limbs(
        self,
        carrier: &Curve3<T>,
        params: (T, T),
        plane: &Surface<T>,
        wall: &geom::NurbsSurface<T>,
        extent: T,
        band: Band,
    ) -> Result<crate::edge_nurbs::PlaneNurbsLimbs<T>, crate::edge_nurbs::PlaneNurbsRefusal> {
        (self.limbs)(carrier, params, plane, wall, extent, band)
    }

    /// The lane's incidence alone for a carrier of any kind on a plane ×
    /// NURBS pair ([`crate::plane_nurbs_incidence`]), over `params`.
    fn incidence(
        self,
        carrier: &Curve3<T>,
        params: (T, T),
        plane: &Surface<T>,
        wall: &Surface<T>,
        stated: Option<&crate::pcurve_cache::Pcurve<T>>,
        band: Band,
    ) -> Result<crate::edge_nurbs::PlaneNurbsIncidence<T>, crate::edge_nurbs::PlaneNurbsRefusal>
    {
        (self.incidence)(carrier, params, plane, wall, stated, band)
    }

    /// The lane's limbs for a declared carrier of any kind between two
    /// spline charts ([`crate::nurbs_pair_limbs`]), over `params`.
    fn pair_limbs(
        self,
        carrier: &Curve3<T>,
        params: (T, T),
        a: &geom::NurbsSurface<T>,
        b: &geom::NurbsSurface<T>,
        extent: T,
        band: Band,
    ) -> Result<crate::edge_nurbs::PlaneNurbsLimbs<T>, crate::edge_nurbs::PlaneNurbsRefusal> {
        (self.pair_limbs)(carrier, params, a, b, extent, band)
    }

    /// C2's limbs of a carrier of any kind between two analytic
    /// surfaces ([`crate::analytic_edge_limbs`]).
    fn analytic_limbs(
        self,
        carrier: &Curve3<T>,
        params: (T, T),
        s1: &Surface<T>,
        s2: &Surface<T>,
        band: Band,
    ) -> Result<(), crate::edge_nurbs::AnalyticRung3Refusal> {
        (self.analytic_limbs)(carrier, params, s1, s2, band)
    }

    /// **A spline span against an analytic face's carrier**: the
    /// crossings strictly inside `params`, ascending, by the curve × face
    /// table's ring-composite arm
    /// ([`crate::curve_face::composite::crossings`]).
    ///
    /// # Errors
    ///
    /// [`crate::curve_face::composite::CompositeRefusal`], as its
    /// variants say.
    pub fn surface_crossings(
        self,
        net: &geom::NurbsCurve3<T>,
        params: (T, T),
        surface: &Surface<T>,
        band: Band,
    ) -> Result<Vec<T>, crate::curve_face::composite::CompositeRefusal> {
        (self.composite)(net, params, surface, band)
    }

    /// **A span against a spline chart**: the meetings strictly
    /// inside `params`, ascending, each with its chart point, by the
    /// curve × face table's spline-face arm
    /// ([`crate::curve_face::spline_face::isolate`], then
    /// [`crate::curve_face::spline_face::settle`]).
    ///
    /// # Errors
    ///
    /// [`crate::curve_face::spline_face::SplineFaceRefusal`], as its
    /// variants say.
    pub fn chart_meetings(
        self,
        carrier: &Curve3<T>,
        params: (T, T),
        chart: &geom::NurbsSurface<T>,
        band: Band,
    ) -> Result<
        Vec<crate::curve_face::spline_face::ChartMeeting<T>>,
        crate::curve_face::spline_face::SplineFaceRefusal,
    > {
        (self.spline_face)(carrier, params, chart, band)
    }
}

impl<T: Real> SplineFoot<T> {
    /// The foot of `point` on a NURBS `carrier`, by Newton from `seed`:
    /// the parameter of a point moved along the carrier, read on the
    /// branch it was moved from. The foot carries its own residuals and
    /// certifies nothing; the caller gates the distance. A foot the
    /// domain clamp stopped at an end is the carrier's nearest END, not
    /// a point of it the move reached.
    ///
    /// # Errors
    ///
    /// [`geom::ProjectionInconclusive`] when Newton's budget expires.
    pub fn carrier_foot(
        self,
        carrier: &geom::NurbsCurve3<T>,
        point: Point3<T>,
        seed: T,
    ) -> Result<geom::Projection3<T>, geom::ProjectionInconclusive> {
        (self.foot)(carrier, point, seed)
    }
}

impl<T: Decide> EdgeCurve<T> {
    /// The carrier's derivative where a walk along the edge leaves and
    /// where it arrives, each in the direction of travel: `he_plus`
    /// walks `t₀ → t₁`, the other half `t₁ → t₀` with both negated.
    /// Neither is normalized.
    pub fn walk_tangents(&self, he_plus: bool) -> (geom_core::Vec3<T>, geom_core::Vec3<T>) {
        let (t0, t1) = self.params();
        if he_plus {
            (self.carrier.deriv(t0), self.carrier.deriv(t1))
        } else {
            (-self.carrier.deriv(t1), -self.carrier.deriv(t0))
        }
    }

    /// The carrier's second derivative where a walk along the edge
    /// leaves ([`Self::walk_tangents`]' walk). Reversing the walk flips
    /// the first derivative only: position along it is `c(t₁ − τ)`, so
    /// `d²/dτ² = c″(t₁)`.
    pub fn walk_departure_deriv2(&self, he_plus: bool) -> geom_core::Vec3<T> {
        let (t0, t1) = self.params();
        self.carrier.deriv2(if he_plus { t0 } else { t1 })
    }
}

impl<T: Real> EdgeCurve<T> {
    /// The intensional description (authoritative, D2/U2): D2's two
    /// intrinsic arms, ONE conventional form, and the fenced
    /// scaffolding door.
    ///
    /// Handed out by reference, never by value: [`EdgeDescription`]
    /// carries a [`crate::Pcurve`] and is therefore not `Copy` — an
    /// edge description is read, not moved around.
    pub fn description(&self) -> &EdgeDescription<T> {
        &self.description
    }

    /// The **authority record** (U2 Q3): whether a modeler DECLARED
    /// this locus, and with what sketch source.
    pub fn authority(&self) -> EdgeAuthority<T> {
        self.authority
    }

    /// The cached carrier curve (a certified derived cache, D4 ¶2).
    pub fn carrier(&self) -> &Curve3<T> {
        &self.carrier
    }

    /// The certified carrier-parameter interval `(t₀, t₁)` —
    /// `he_plus`-forward, a certified cache of the vertex-derived
    /// bounds (module docs).
    pub fn params(&self) -> (T, T) {
        (self.param_start, self.param_end)
    }

    /// What the scalar's door gave this edge to carry
    /// ([`Real::Carries`]): the attachment-time certification record at
    /// a certifying scalar, nothing at a tangent-transport one.
    pub fn certificate(&self) -> &T::Carries<Certificate<T>> {
        &self.certificate
    }

    /// Where the edge's extent stands against one period of its
    /// carrier, as check 2 decided it at the mint: `None` on a carrier
    /// with no period (a line, a spline).
    pub fn turn(&self) -> Option<&CertifiedTurn<T>> {
        self.turn.as_ref()
    }

    /// The same certified carrier with its description's **SURFACE
    /// KEYS** rewritten, for a transplant into another body's arenas.
    ///
    /// A surface key is an arena handle, not geometry: the two
    /// intrinsic arms and the chart arm name the surfaces their locus
    /// is stated against, and a graft that re-creates
    /// those surfaces BITWISE under fresh keys has changed the handles
    /// and nothing else. The certificate — a residual over the
    /// description, the carrier, the interval and the surfaces' VALUES
    /// — is therefore still the certificate of exactly this geometry,
    /// and travels verbatim.
    ///
    /// One of two doors that mint an `EdgeCurve` without a run of the
    /// schedule (the other is [`EdgeCurve::with_chart_v_mirrored`]),
    /// and it is narrow on purpose: nothing but the keys may differ,
    /// so it cannot express a geometry change. Its
    /// existence is what lets a transplant carry descriptions whose
    /// surfaces the certification lanes cannot re-certify at all (a
    /// rational NURBS wall certifies nowhere — see
    /// `CertifyError::Unimplemented`), which is not a licence to
    /// invent a certificate: the source body's run is the certificate.
    ///
    /// `None` when `remap` does not answer for a key the description
    /// names — a dangling handle is never written.
    #[must_use]
    pub fn with_remapped_surfaces(
        &self,
        mut remap: impl FnMut(crate::keys::SurfaceKey) -> Option<crate::keys::SurfaceKey>,
    ) -> Option<Self> {
        let description = match self.description {
            EdgeDescription::Intersection { pair, ref selector } => {
                let (pair, selector) = selector.remapped(pair, &mut remap)?;
                EdgeDescription::Intersection { pair, selector }
            }
            EdgeDescription::TangentIntersection { pair, witness } => {
                EdgeDescription::TangentIntersection {
                    pair: pair.try_map(|k| remap(k).ok_or(())).ok()?,
                    witness,
                }
            }
            EdgeDescription::OnPair { pair } => EdgeDescription::OnPair {
                pair: pair.try_map(|k| remap(k).ok_or(())).ok()?,
            },
            // The chart IMAGE travels verbatim, because it is geometry
            // stated in chart COORDINATES, which a bitwise re-creation
            // of the surface leaves untouched. Only the handle moves.
            EdgeDescription::Chart(ref c) => EdgeDescription::Chart(ChartCurve {
                surface: remap(c.surface)?,
                pcurve: c.pcurve.clone(),
                wrap: c.wrap,
            }),
            // A scaffold names no surface — there is none yet.
            EdgeDescription::Scaffold(m) => EdgeDescription::Scaffold(m),
        };
        Some(Self {
            description,
            authority: self.authority,
            carrier: self.carrier.clone(),
            param_start: self.param_start,
            param_end: self.param_end,
            turn: self.turn,
            certificate: self.certificate,
        })
    }

    /// The same certified carrier with its **chart image mirrored in
    /// `v`** ([`crate::Pcurve::mirror_v`]), for a chart whose second
    /// frame axis was negated — the certificate travels verbatim, the
    /// carrier, interval, authority and wrap flag untouched. A
    /// description that carries no chart image (the two intrinsic
    /// arms and the scaffolding door) states nothing in chart
    /// coordinates, so the map is the identity on it and the curve
    /// comes back as it is.
    ///
    /// The second of the two doors that mint an `EdgeCurve` without a
    /// run of the schedule, and narrow for the same reason as
    /// [`EdgeCurve::with_remapped_surfaces`]: it cannot express a
    /// geometry change. Why the certificate of the source is the
    /// certificate of the result is stated once, on
    /// [`crate::Pcurve::mirror_v`] — the mirrored image on the
    /// mirrored chart evaluates to the same 3-D points, so
    /// `|C(t) − S(P(t))|` at every sample is the number the run
    /// produced. Whoever negates a chart's frame owes this
    /// re-statement to every image on it, which is what makes an
    /// orientation reversal a certification-preserving map rather
    /// than a geometry change with a stale certificate beside it.
    ///
    /// # Errors
    ///
    /// `None` exactly when [`crate::Pcurve::mirror_v`] answers `None`
    /// — a spiric WALL image, which has no reflected locus. That door
    /// carries the derivation and the reason no caller reaches it: a
    /// wall image lives on a torus chart, and the one producer of this
    /// call (`topo::revert`) mirrors plane charts only.
    #[must_use]
    pub fn with_chart_v_mirrored(&self) -> Option<Self> {
        let description = match self.description {
            EdgeDescription::Chart(ref c) => EdgeDescription::Chart(ChartCurve {
                surface: c.surface,
                pcurve: c.pcurve.mirror_v()?,
                wrap: c.wrap,
            }),
            ref other @ (EdgeDescription::Intersection { .. }
            | EdgeDescription::TangentIntersection { .. }
            | EdgeDescription::OnPair { .. }
            | EdgeDescription::Scaffold(_)) => other.clone(),
        };
        Some(Self {
            description,
            authority: self.authority,
            carrier: self.carrier.clone(),
            param_start: self.param_start,
            param_end: self.param_end,
            turn: self.turn,
            certificate: self.certificate,
        })
    }

    /// The carrier parameter at schedule sample `i` (i ∈ 0…8): the
    /// module-doc schedule ([`sample_param`]). Exposed so the
    /// tier-3 validator samples the *same* parameters the certification
    /// did (D9).
    pub fn sample_param(&self, i: u32) -> T {
        sample_param(self.param_start, self.param_end, i)
    }
}

impl<T: SpanLocate> EdgeCurve<T> {
    /// The carrier point at the middle of the certified parameter
    /// interval ([`Curve3::mid_point`]) — a point ON the edge, interior
    /// to it, whatever the carrier kind.
    pub fn mid_point(&self) -> Point3<T> {
        self.carrier.mid_point(self.param_start, self.param_end)
    }

    /// The two **uncertified child specs** of splitting this certified
    /// carrier at interior parameter `t` (M3 PR 1, for `split_edge`):
    /// the carrier is unchanged and the interval splits at `t`
    /// (`[t₀, t]` / `[t, t₁]` — both forward for interior `t`, so the
    /// forward-span gate is untouched); the description splits
    /// honestly per kind — `MappedCurve` via
    /// [`crate::MappedCurve::restrict`] at the interval fractions,
    /// `Intersection` keeps its surfaces with each child's witness
    /// re-minted as its own mid-parameter carrier point (the witness
    /// contract, bitwise the certification schedule's middle sample),
    /// a chart image keeps its chart, its parent's image and its wrap flag.
    ///
    /// A `TangentIntersection` child is the restriction, not yet its
    /// description: the must-carry rule
    /// ([`crate::must_carry_over_edge`]) meters its sagitta over the
    /// child's own extent, so a short child may owe a chart image, or a
    /// refusal, instead. `topo`'s `Body::split_edge` asks the rule of
    /// each child before certifying it.
    ///
    /// The children are *specs*, not certified carriers: the caller
    /// must run each through [`EdgeCurve::certify`] against its own
    /// endpoints (D4 ¶2 — the restriction arithmetic is verified, not
    /// trusted). Interiority of `t` is likewise the caller's trilean
    /// obligation; this function is total arithmetic.
    pub fn split_specs(&self, t: T) -> (EdgeCurveSpec<T>, EdgeCurveSpec<T>) {
        let (t0, t1) = (self.param_start, self.param_end);
        let span = t1 - t0;
        let a = (t - t0) / span;
        let child = |s0: T, s1: T, ta: T, tb: T| -> EdgeCurveSpec<T> {
            let description = match self.description {
                EdgeDescription::Intersection { pair, ref selector } => {
                    EdgeDescriptionSpec::Intersection {
                        pair,
                        selector: selector.repinned(&self.carrier, ta, tb),
                    }
                }
                // TangentIntersection splits exactly as Intersection:
                // surfaces kept, witness re-minted at the child's own
                // mid-parameter (the witness contract, one order up).
                EdgeDescription::TangentIntersection { pair, .. } => {
                    EdgeDescriptionSpec::TangentIntersection {
                        pair,
                        witness: self.carrier.mid_point(ta, tb),
                    }
                }
                EdgeDescription::Scaffold(mc) => EdgeDescriptionSpec::Scaffold(mc.restrict(s0, s1)),
                // The pair is the whole description: each child lies on
                // both surfaces as its parent did.
                EdgeDescription::OnPair { pair } => EdgeDescriptionSpec::OnPair {
                    pair,
                    declared: match self.authority {
                        EdgeAuthority::Declared(mc) => Some(mc.restrict(s0, s1)),
                        EdgeAuthority::Derived => None,
                    },
                },
                // A chart image is a function of the CARRIER's own
                // parameter, and splitting an edge changes the
                // interval, not the carrier — so the child's image is
                // the parent's image, verbatim. Stating it exactly is
                // what keeps the sub-arc's description the restriction
                // of its parent's rather than a re-derivation that can
                // land a few ulps away from it.
                EdgeDescription::Chart(ref c) => EdgeDescriptionSpec::Chart {
                    surface: c.surface,
                    image: Some(c.pcurve.clone()),
                    wrap: c.wrap,
                    declared: match self.authority {
                        EdgeAuthority::Declared(mc) => Some(mc.restrict(s0, s1)),
                        EdgeAuthority::Derived => None,
                    },
                },
            };
            EdgeCurveSpec {
                description,
                carrier: self.carrier.clone(),
                param_start: ta,
                param_end: tb,
            }
        };
        (child(T::zero(), a, t0, t), child(a, T::one(), t, t1))
    }

    /// This carrier's spec view (for re-certification).
    fn spec(&self) -> EdgeCurveSpec<T> {
        self.restated_spec()
    }
}

impl<T: Real> EdgeCurve<T> {
    /// This edge restated as the SPEC a construction would hand in —
    /// description ([`EdgeCurve::restated_description`]), carrier and
    /// interval, all verbatim. The door a consumer goes through to
    /// move an edge's DESCRIPTION without touching its geometry.
    #[must_use]
    pub fn restated_spec(&self) -> EdgeCurveSpec<T> {
        EdgeCurveSpec {
            description: self.restated_description(),
            carrier: self.carrier.clone(),
            param_start: self.param_start,
            param_end: self.param_end,
        }
    }

    /// This edge re-homed in `surface`'s chart: an image the door
    /// derives from the carrier, carrier and interval verbatim, and the
    /// authority record as it was. A declared locus stays declared; a
    /// derived one is not given a declaration. The conventional
    /// restatement for an edge at rest whose description no longer
    /// names a chart it lies in.
    #[must_use]
    pub fn restated_in_chart(&self, surface: SurfaceKey) -> EdgeCurveSpec<T> {
        let chart = EdgeDescriptionSpec::chart(surface);
        EdgeCurveSpec {
            description: match self.authority {
                EdgeAuthority::Declared(mc) => chart.declared_by(mc),
                EdgeAuthority::Derived => chart,
            },
            ..self.restated_spec()
        }
    }

    /// This edge re-stated where `rest` says it rests
    /// ([`crate::RestLocus`]): [`EdgeCurve::restated_in_chart`] or
    /// [`EdgeCurve::restated_on_pair`], as the one home decided.
    #[must_use]
    pub fn restated_at(&self, rest: crate::RestLocus) -> EdgeCurveSpec<T> {
        match rest {
            crate::RestLocus::Chart(surface) => self.restated_in_chart(surface),
            crate::RestLocus::Pair(pair) => self.restated_on_pair(pair),
        }
    }

    /// This edge re-stated as the pair `pair` under-determines
    /// (`OnPair`): carrier and interval verbatim, and the authority
    /// record as it was. The conventional restatement for an edge at
    /// rest between two distinct surfaces whose description no longer
    /// names them.
    #[must_use]
    pub fn restated_on_pair(&self, pair: crate::SurfacePair) -> EdgeCurveSpec<T> {
        let declared = match self.authority {
            EdgeAuthority::Declared(mc) => Some(mc),
            EdgeAuthority::Derived => None,
        };
        EdgeCurveSpec {
            description: EdgeDescriptionSpec::OnPair { pair, declared },
            ..self.restated_spec()
        }
    }

    /// This edge's certified description stated back the way a
    /// CONSTRUCTION states one — the door every consumer that rebuilds
    /// a spec from a certified edge goes through (a transplant, a
    /// re-anchor, a re-certification at rest).
    ///
    /// Restated in its own chart, a chart image travels EXACTLY, a wrap
    /// edge's included: restating a description must re-meter the image
    /// the edge already carries, never derive a second one and meter
    /// that. A door that MOVES the chart states the image anew instead
    /// (`topo`'s face replacement shifts an image with its chart, and
    /// derives a wrap edge's against the moved one).
    #[must_use]
    pub fn restated_description(&self) -> EdgeDescriptionSpec<T> {
        let declared = match self.authority {
            EdgeAuthority::Declared(mc) => Some(mc),
            EdgeAuthority::Derived => None,
        };
        match self.description {
            EdgeDescription::Intersection { pair, ref selector } => {
                EdgeDescriptionSpec::Intersection {
                    pair,
                    selector: selector.clone(),
                }
            }
            EdgeDescription::TangentIntersection { pair, witness } => {
                EdgeDescriptionSpec::TangentIntersection { pair, witness }
            }
            EdgeDescription::OnPair { pair } => EdgeDescriptionSpec::OnPair { pair, declared },
            EdgeDescription::Chart(ref c) => EdgeDescriptionSpec::Chart {
                surface: c.surface,
                image: Some(c.pcurve.clone()),
                wrap: c.wrap,
                declared,
            },
            EdgeDescription::Scaffold(mc) => EdgeDescriptionSpec::Scaffold(mc),
        }
    }
}

/// The certification schedule's parameter at sample `i` — the
/// [`CERT_SAMPLES`]-point [`schedule_param`].
///
/// Public because the contact vocabulary's tangency verification runs
/// the SAME schedule over a locus that is not an edge yet
/// (`topo::boolean::contact_verify`): two samplers over one schedule
/// is the twin this export exists to prevent.
pub fn sample_param<T: Real>(t0: T, t1: T, i: u32) -> T {
    schedule_param(t0, t1, i, CERT_SAMPLES)
}

/// Station `i` of a uniform `samples`-point schedule over a carrier's
/// held domain `held`, whose value channel is `domain`: the ends are
/// the held ends themselves (the carrier's own data), and every other
/// station is the `f64` [`schedule_param`] over `domain`, a point the
/// schedule chose, entered as the constant it is. Bit for bit
/// [`schedule_param`] at `f64`; at an enclosure scalar an interior
/// station stays a point rather than widening by the station's own
/// arithmetic, which a check at a chosen point has no use for.
pub(crate) fn held_station<T: Real>(domain: (f64, f64), held: (T, T), i: u32, samples: u32) -> T {
    match schedule_fraction(i, samples) {
        0.0 => held.0,
        1.0 => held.1,
        _ => T::from_f64(schedule_param(domain.0, domain.1, i, samples)),
    }
}

/// The fraction `i / (samples − 1)` of a uniform `samples`-point
/// schedule — an exact dyadic when `samples − 1` is a power of two, as
/// every fixed schedule in this crate's certificates is. The same
/// precondition as [`schedule_param`].
pub(crate) fn schedule_fraction(i: u32, samples: u32) -> f64 {
    f64::from(i) / f64::from(samples - 1)
}

/// **The one uniform sample schedule** over a parameter interval: sample
/// `i` of `samples` (both ends included) over `[t₀, t₁]`.
///
/// The stations the interval itself names are ASSIGNED, not computed:
/// `t₀` at fraction 0, `t₁` at fraction 1, and at fraction ½ the
/// interval's middle, [`geom::mid_param`] — the one middle every
/// reader of an edge shares, so the middle sample is the parameter
/// [`Curve3::mid_point`] evaluates at and a certificate that samples
/// the carrier and checks its midpoint witness evaluates that point
/// once. Every other station is `t₀ + (t₁ − t₀)·(i/(samples − 1))` in
/// that association order (D9). `t₀ + (t₁ − t₀)·1` is not `t₁` in
/// `f64` in general, and an image re-expressed on `[t₀, t₁]` exactly
/// has its last knot at `t₁` itself, so a last sample computed rather
/// than assigned sits an ulp outside it.
///
/// Every fixed sample schedule over a carrier's parameter interval
/// reads this: the certification schedule ([`sample_param`], and
/// through it the SSI certificate's limb 1, tier 3, the contact
/// verifier, the importer's plane and arc-rim checks and the blend
/// battery), the plane × NURBS chart image's foot schedule and the
/// parameter its refusals report, and the ellipse-on-cylinder pcurve
/// fit. A tessellation's chords and a quadrature's pieces partition an
/// interval rather than sample it, and assign their ends themselves.
///
/// # Panics
///
/// When `samples < 2` or `i ≥ samples`: a schedule names both of its
/// ends, and every caller passes a fixed count.
pub fn schedule_param<T: Real>(t0: T, t1: T, i: u32, samples: u32) -> T {
    assert!(
        samples >= 2 && i < samples,
        "schedule_param: sample {i} of a {samples}-point schedule"
    );
    match schedule_fraction(i, samples) {
        0.0 => t0,
        0.5 => geom::mid_param(t0, t1),
        1.0 => t1,
        f => t0 + (t1 - t0) * T::from_f64(f),
    }
}

/// An edge about to be described as the transverse `Intersection` of
/// its two faces' surfaces, read once: the `witness` the dihedral
/// classifies at, and the carrier and interval the description is
/// certified against if the class is transverse.
///
/// A curved certified carrier ([`Curve3::is_curved`]) is kept with its
/// interval — only the description changes — and is read at its
/// [`EdgeCurve::mid_point`]. Anything else (a line, or no certified
/// carrier yet) becomes the chord `p0 → p1`
/// ([`EdgeCurveSpec::line_between`]), whose midpoint is on it.
#[derive(Clone, Debug)]
pub struct IntersectionDraft<T: Real> {
    /// The point the dihedral classifies at and the description pins.
    pub witness: Point3<T>,
    carrier: Curve3<T>,
    param_start: T,
    param_end: T,
    kept: bool,
}

impl<T: SpanLocate> IntersectionDraft<T> {
    /// Reads the edge from its certified curve, if it has one, and the
    /// points of `start(he_plus)` and `end(he_plus)`.
    pub fn of(existing: Option<&EdgeCurve<T>>, p0: Point3<T>, p1: Point3<T>) -> Self {
        match existing.filter(|c| c.carrier().is_curved()) {
            Some(c) => {
                let (t0, t1) = c.params();
                Self {
                    witness: c.mid_point(),
                    carrier: c.carrier().clone(),
                    param_start: t0,
                    param_end: t1,
                    kept: true,
                }
            }
            None => {
                let chord = EdgeCurveSpec::line_between(p0, p1);
                Self {
                    witness: p0.lerp(p1, T::from_f64(0.5)),
                    carrier: chord.carrier,
                    param_start: chord.param_start,
                    param_end: chord.param_end,
                    kept: false,
                }
            }
        }
    }
}

impl<T: Real> IntersectionDraft<T> {
    /// The carrier the description would be certified against, and its
    /// interval: the edge whose faces' extent levers the dihedral.
    pub fn span(&self) -> (&Curve3<T>, (T, T)) {
        (&self.carrier, (self.param_start, self.param_end))
    }

    /// The `Intersection` of `pair` at this draft's witness: the
    /// description alone where the draft kept the edge's own carrier and
    /// interval, the whole chord spec where it drew one.
    pub fn into_intersection(self, pair: SurfacePair) -> DraftedIntersection<T> {
        let witness = CurveSelector::Witness(self.witness);
        self.into_selected(pair, witness)
    }

    /// The `Intersection` of `pair` that `selector` picks, over this
    /// draft's carrier.
    pub fn into_selected(
        self,
        pair: SurfacePair,
        selector: CurveSelector<T>,
    ) -> DraftedIntersection<T> {
        let description = EdgeDescriptionSpec::Intersection { pair, selector };
        if self.kept {
            DraftedIntersection::Kept(description)
        } else {
            DraftedIntersection::Chord(EdgeCurveSpec {
                description,
                carrier: self.carrier,
                param_start: self.param_start,
                param_end: self.param_end,
            })
        }
    }
}

impl<T: Decide> IntersectionDraft<T> {
    /// **D2's question of a seam between two spline walls: is its curve
    /// known in advance?** `Some` where it is a row on each wall's chart
    /// boundary: `walls` are the seam's two surfaces, each with the edge's
    /// image on it, and each image is an iso row along a domain side of
    /// that wall's chart and certifies there over the draft's own carrier
    /// and interval (the wall's chart-image certificate, which the
    /// structural selector's own certificate reads). `None` otherwise,
    /// and for a draft that drew a chord: no row is stated over one.
    ///
    /// Asked of the geometry alone: the images are what the walls' charts
    /// state for this carrier, whoever built the edge.
    ///
    /// Built for boundary rows, which a rim between two walls grafted at a
    /// shared section needs. D2's structural selector covers any row or
    /// column known in advance; the domain-side restriction is this
    /// reader's scope, not the selector's definition.
    pub fn shared_row(
        &self,
        pair: SurfacePair,
        walls: [(SurfaceKey, &Surface<T>, Pcurve<T>); 2],
        band: Band,
    ) -> Option<CurveSelector<T>> {
        if !self.kept {
            return None;
        }
        let [(ka, wa, ia), (kb, wb, ib)] = walls;
        let span = (self.param_start, self.param_end);
        let on_side = |wall: &Surface<T>, image: &Pcurve<T>| {
            side_row_certifies(wall, image, (&self.carrier, span), band)
        };
        if !(on_side(wa, &ia) && on_side(wb, &ib)) {
            return None;
        }
        let row = crate::description::SharedRow::new(pair, (ka, ia), (kb, ib))?;
        Some(CurveSelector::Structural(Structural::SharedRow(row)))
    }
}

/// **A shared row's one admission test, per wall**: `image` is a row on a
/// domain side of `wall`'s spline chart (`description::on_domain_side`)
/// and certifies there as the image of `carrier` over `span`
/// ([`crate::PcurveCache::certify`]). [`IntersectionDraft::shared_row`]
/// reads each wall by it, and so does the boolean's interior-loop guard
/// when it hands a rim two spline faces share to their section's reading.
pub fn side_row_certifies<T: Decide>(
    wall: &Surface<T>,
    image: &Pcurve<T>,
    (carrier, (t0, t1)): (&Curve3<T>, (T, T)),
    band: Band,
) -> bool {
    wall.spline_chart()
        .is_some_and(|chart| crate::description::on_domain_side(chart, image))
        && crate::pcurve_cache::PcurveCache::certify(image.clone(), t0, t1, carrier, wall, band)
            .is_ok()
}

/// An [`IntersectionDraft`]'s `Intersection`, by whose carrier it is
/// stated over.
#[derive(Clone, Debug)]
pub enum DraftedIntersection<T: Real> {
    /// The edge's own certified carrier and interval, kept: only the
    /// description, which restates them.
    Kept(EdgeDescriptionSpec<T>),
    /// The chord drawn between the edge's ends: a spec of its own.
    Chord(EdgeCurveSpec<T>),
}

/// Folds a residual into the running max and classifies it: must be
/// coincident with zero (|r| ≤ ε). Positive/Negative beyond the band ⇒
/// [`CertifyError::ResidualExceeded`]; in-band or poisoned ⇒
/// [`CertifyError::Escalated`].
/// The declaration's meter at sample `i`: the carrier at `p` against the
/// mapped source at the sample's schedule fraction
/// (`carrier_matches_mapped_source`, [`CertCheck::MappedSource`]), where
/// a declaration is carried.
fn mapped_source<T: Decide>(
    declared: Option<&crate::mapped::MappedCurve<T>>,
    i: u32,
    p: Point3<T>,
    band: Band,
    max_residual: &mut T,
) -> Result<(), CertifyError> {
    let Some(mc) = declared else {
        return Ok(());
    };
    let frac = T::from_f64(schedule_fraction(i, CERT_SAMPLES));
    check_residual(
        "carrier_matches_mapped_source",
        CertCheck::MappedSource,
        i,
        Margin::of(p.distance(mc.eval(frac))),
        band,
        max_residual,
    )
}

fn check_residual<T: Decide>(
    name: &'static str,
    check: CertCheck,
    sample: u32,
    residual: Margin<T>,
    band: Band,
    max_residual: &mut T,
) -> Result<(), CertifyError> {
    *max_residual = max_residual.max(residual.value().abs());
    match decide_reported(name, residual, band) {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok(()),
        Ok(Decided { margin, .. }) => Err(CertifyError::ResidualExceeded {
            check,
            sample,
            margin,
        }),
        Err(cause) => Err(CertifyError::Escalated {
            check,
            sample,
            cause,
        }),
    }
}

/// **σ, the orientation of the shared normal** at a tangent sample: the
/// sign of the two unit implicit gradients' alignment, which must be
/// definite (`decide_nonzero`). At a tangency the normals are parallel,
/// so the cosine is `±1`, as far from zero as it can be; it is metered
/// over the edge's extent, the ordinary angular door, so a surface
/// curving on a scale near ε (whose folded arm puts every angle in
/// band) still has an orientation to read the gap along.
///
/// No definite sign means no shared normal to read the separation
/// along. A definite parallelism defect at the folded arm is then the
/// cause, named as itself (the second-order arm's renaming, below);
/// otherwise the orientation's own refusal escalates under
/// [`CertCheck::TangentGap`]. Both readings run detached, so the log
/// carries only the escalation the error carries; the naming reading
/// never refuses, it only names.
fn gap_orientation<T: Decide>(
    surf1: &Surface<T>,
    surf2: &Surface<T>,
    p: Point3<T>,
    extent: T,
    sample: u32,
    band: Band,
) -> Result<T, CertifyError> {
    let (g1, g2) = (
        crate::implicit::implicit_gradient(surf1, p),
        crate::implicit::implicit_gradient(surf2, p),
    );
    let (n1, n2) = (g1.normalize(), g2.normalize());
    let (oriented, orientation) = geom_core::k_stats::detached(|| {
        geom_core::k_stats::decide_nonzero(
            "tangent_gap_orientation",
            Margin::levered(n1.dot(n2), extent),
            band,
        )
    });
    let cause = match oriented {
        Ok(sign) => {
            geom_core::k_stats::splice(orientation);
            return Ok(match sign {
                geom_core::k_stats::NonzeroSign::Positive => T::from_f64(1.0),
                geom_core::k_stats::NonzeroSign::Negative => T::from_f64(-1.0),
            });
        }
        Err(cause) => cause,
    };
    let arm = crate::folded_lever_arm(surf1, surf2, p, extent);
    if let Some(margin) = named_by_parallelism(crate::tangent::normal_sine(g1, g2), arm, band) {
        geom_core::k_stats::splice_superseded(orientation);
        return Err(CertifyError::ResidualExceeded {
            check: CertCheck::TangentParallel,
            sample,
            margin,
        });
    }
    geom_core::k_stats::splice(orientation);
    Err(CertifyError::Escalated {
        check: CertCheck::TangentGap,
        sample,
        cause,
    })
}

/// **The parallelism defect at the folded lever arm**: `sin θ` metered
/// over [`crate::folded_lever_arm`], the length the normals' angle is
/// consumed over where no lever `1/κ_rel` exists. One spelling for its
/// three readers: the certificate's two refusal namings
/// ([`named_by_parallelism`]) and the declared-contact verifier's
/// first-order fallback (`topo::boolean::contact_verify`), where it is
/// the verdict itself.
#[must_use]
pub fn folded_parallelism<T: Real>(sin_theta: T, arm: T) -> Margin<T> {
    Margin::levered(sin_theta, arm)
}

/// **A refusal already made, named by a definite parallelism defect**
/// ([`folded_parallelism`], `tangent_normal_parallel`): the margin where
/// the defect is definite, `None` otherwise. The reading never refuses,
/// it only names, so it runs detached and its log is spliced as
/// superseded whatever it reads; the caller supersedes the refusal it
/// names where it names one.
fn named_by_parallelism<T: Decide>(sin_theta: T, arm: T, band: Band) -> Option<MarginDiag> {
    let (renamed, naming) = geom_core::k_stats::detached(|| {
        match decide_reported(
            "tangent_normal_parallel",
            folded_parallelism(sin_theta, arm),
            band,
        ) {
            Ok(Decided {
                sign: Sign::Positive | Sign::Negative,
                margin,
            }) => Some(margin),
            Ok(_) | Err(_) => None,
        }
    });
    geom_core::k_stats::splice_superseded(naming);
    renamed
}

/// The plane × NURBS lane's refusal in this module's vocabulary: a
/// per-sample transversality refusal or escalation keeps its sample, and a limb's
/// escalation or the poisoned aggregate, which no schedule point
/// carries, names none ([`NOT_A_SAMPLE`]). The per-sample refusal keeps
/// its sample and the length that levered its angle.
fn from_plane_nurbs(e: crate::edge_nurbs::PlaneNurbsRefusal) -> CertifyError {
    use crate::edge_nurbs::PlaneNurbsRefusal as P;
    match e {
        P::NotTransverse {
            sample,
            lever,
            verdict,
        } => CertifyError::NotTransverse {
            sample,
            lever: Some(lever),
            verdict,
        },
        P::TransversalityEscalated { sample, cause } => CertifyError::Escalated {
            check: CertCheck::Transversality,
            sample,
            cause,
        },
        P::Escalated { limb, cause } => CertifyError::Escalated {
            check: limb.check(),
            sample: NOT_A_SAMPLE,
            cause,
        },
        P::ArcRim(crate::edge_nurbs::ArcRimRefusal::Escalated { check, cause }) => {
            CertifyError::Escalated {
                check: arc_rim_check(check),
                sample: NOT_A_SAMPLE,
                cause,
            }
        }
        P::ReportedTransversalityPoisoned(cause) => CertifyError::Escalated {
            check: CertCheck::PlaneNurbsReportedTransversality,
            sample: NOT_A_SAMPLE,
            cause,
        },
        other => CertifyError::PlaneNurbs(other),
    }
}

/// **Check 1 and the description's resolution**: whether the
/// carrier's class certifies under its description at all, and the
/// description's surfaces resolved, in key order. Construction, at any
/// deciding scalar.
fn resolve_spec<T: Decide>(
    spec: &EdgeCurveSpec<T>,
    surfaces: &impl Fn(SurfaceKey) -> Option<Surface<T>>,
) -> Result<Resolved<T>, CertifyError> {
    // ---- Check 1: implementedness / description well-formedness. ----
    // Rung-3 (`Nurbs`) carriers certify under an intrinsic description
    // of two ANALYTIC surfaces — the class the curved boolean zip
    // mints (M5 PR 9, C12.3; the fitted SSI branch) — and under a
    // chart description whose image the CONSTRUCTION states (M6-3: the
    // loft/sweep wall–wall seam class, whose residual is the genuinely
    // metric `|C(t) − S(P(t))|`). A `Nurbs` carrier under a chart
    // description whose image would have to be DERIVED stays refused,
    // and so does one under the scaffolding door: nothing mints
    // either, and neither has a certified meter — a derived image
    // needs the analytic chart machinery, and a fitted carrier
    // "matching" a mapped source states no residual at all. An `OnPair`
    // passes here and is decided where its pair resolves: the plane ×
    // NURBS lane's limb 2 bounds any carrier between its samples, and
    // two analytic surfaces' per-sample residuals do not.
    if matches!(spec.carrier, Curve3::Nurbs(_))
        && !matches!(
            spec.description,
            EdgeDescriptionSpec::Intersection { .. }
                | EdgeDescriptionSpec::TangentIntersection { .. }
                | EdgeDescriptionSpec::OnPair { .. }
                | EdgeDescriptionSpec::Chart { image: Some(_), .. }
        )
    {
        return Err(CertifyError::Unimplemented);
    }
    // `Approx` refuses here with `Nurbs`, and for the same reason: the
    // descriptions this resolver serves state their residual through
    // the IMPLICIT form, which a spline does not have. Admitting one
    // would meter poison. A plane × spline `Intersection`, fitted or
    // not, never reaches it: `plane_nurbs_pair` takes that pair first.
    let resolve = |key: SurfaceKey| -> Result<Surface<T>, CertifyError> {
        let s = surfaces(key).ok_or(CertifyError::UnresolvedSurface { key })?;
        if !crate::edge_nurbs::is_analytic(&s) {
            return Err(CertifyError::Unimplemented);
        }
        Ok(s)
    };
    // The iso lane's resolver (M6-3): an `IsoCurve` description names
    // its surface as the CHART of the residual, so a DESCRIBED
    // `Surface::Nurbs` is exactly what it evaluates against — admitted
    // here and nowhere else. The mvfs placeholder (all-poison control)
    // is not a described surface and keeps refusing.
    let resolve_iso = |key: SurfaceKey| -> Result<Surface<T>, CertifyError> {
        let s = surfaces(key).ok_or(CertifyError::UnresolvedSurface { key })?;
        if let Surface::Nurbs(ref payload) = s
            && payload.is_placeholder()
        {
            return Err(CertifyError::Unimplemented);
        }
        Ok(s)
    };
    Ok(match spec.description {
        EdgeDescriptionSpec::Intersection {
            pair,
            selector: CurveSelector::Structural(Structural::SharedRow(ref row)),
        } => {
            let [k1, k2] = pair.keys();
            if k1 == k2 {
                return Err(CertifyError::IntersectionSameSurface { key: k1 });
            }
            // Each wall states its row on its own chart, which is
            // evaluated as a stated chart image is: a described spline
            // chart and nothing else.
            let wall = |k: SurfaceKey, image: &Pcurve<T>| {
                let s = resolve_iso(k)?;
                let Some(chart) = s.spline_chart() else {
                    return Err(CertifyError::Unimplemented);
                };
                if !crate::description::on_domain_side(chart, image) {
                    return Err(CertifyError::Row {
                        surface: k,
                        refusal: RowRefusal::OffSide,
                    });
                }
                Ok((k, s, image.clone()))
            };
            let [ia, ib] = row.images();
            Resolved::SharedRow {
                walls: [wall(k1, ia)?, wall(k2, ib)?],
            }
        }
        EdgeDescriptionSpec::Intersection {
            pair,
            selector: CurveSelector::Witness(witness),
        } => {
            let [k1, k2] = pair.keys();
            if k1 == k2 {
                return Err(CertifyError::IntersectionSameSurface { key: k1 });
            }
            // The plane × NURBS lane (M7-8) is tried FIRST, because it
            // is one of the two readings under which a described `Nurbs`
            // or a fitted `Approx` operand certifies at all: `resolve`
            // below refuses one typed. The other is two such charts, the
            // same lane's NURBS × NURBS limbs.
            if let Some((plane_key, plane, _, wall)) = plane_nurbs_pair(pair, surfaces) {
                Resolved::PlaneNurbs {
                    plane_key,
                    plane,
                    wall,
                    witness,
                }
            } else if let Some((a, b)) = nurbs_pair(pair, surfaces) {
                Resolved::PairNurbs { a, b, witness }
            } else {
                let surfs = [(k1, resolve(k1)?), (k2, resolve(k2)?)];
                Resolved::Intersection { surfs, witness }
            }
        }
        EdgeDescriptionSpec::TangentIntersection { pair, witness } => {
            // A same-surface "tangency" is a seam exactly as a
            // same-surface intersection is (D2's taxonomy).
            let [k1, k2] = pair.keys();
            if k1 == k2 {
                return Err(CertifyError::IntersectionSameSurface { key: k1 });
            }
            Resolved::Tangent {
                surfs: [(k1, resolve(k1)?), (k2, resolve(k2)?)],
                witness,
            }
        }
        EdgeDescriptionSpec::OnPair { pair, declared } => {
            // One surface on both sides is a chart's locus, not a pair's.
            let [k1, k2] = pair.keys();
            if k1 == k2 {
                return Err(CertifyError::IntersectionSameSurface { key: k1 });
            }
            // A plane and a spline chart meter incidence through the
            // plane × NURBS lane, tried first as `Intersection` tries
            // it: `resolve` refuses the spline side, which has no
            // implicit form. Two spline charts have no incidence lane
            // yet and refuse there.
            if let Some((_, plane, wall_key, wall)) = plane_nurbs_pair(pair, surfaces) {
                Resolved::OnPairNurbs {
                    plane,
                    wall_key,
                    wall,
                    declared,
                }
            } else {
                // Two analytic surfaces' `OnPair` holds no lane for a
                // spline carrier yet
                // (`work/select/onpair-on-two-analytic-surfaces-refuses-a-spline-carrier.md`).
                if matches!(spec.carrier, Curve3::Nurbs(_)) {
                    return Err(CertifyError::Unimplemented);
                }
                Resolved::OnPair {
                    surfs: [(k1, resolve(k1)?), (k2, resolve(k2)?)],
                    declared,
                }
            }
        }
        EdgeDescriptionSpec::Scaffold(mc) => Resolved::Scaffold(mc),
        // The chart arm resolves through the door its IMAGE needs. An
        // image the construction states is evaluated on the chart
        // itself, so a described spline chart is exactly what it needs
        // and is admitted here and nowhere else (the mvfs placeholder,
        // an all-poison control, is not a described surface and keeps
        // refusing); a derived image comes from the analytic chart
        // machinery. A wrap edge names a chart that closes on itself,
        // which the plane never does.
        EdgeDescriptionSpec::Chart {
            surface,
            ref image,
            wrap,
            ref declared,
        } => {
            let s = if image.is_some() {
                resolve_iso(surface)?
            } else {
                resolve(surface)?
            };
            if wrap && matches!(s, Surface::Plane { .. }) {
                return Err(CertifyError::WrapOnNonPeriodic);
            }
            Resolved::Chart {
                surface: s,
                key: surface,
                image: image.clone(),
                wrap,
                declared: *declared,
            }
        }
    })
}

/// **Check 2**: the interval's span, decided forward, and where a
/// periodic carrier's extent stands against one period. Construction:
/// the turn is the minted edge's, read by every scalar's readers.
fn decide_turn<T: Decide>(
    spec: &EdgeCurveSpec<T>,
    band: Band,
) -> Result<Option<CertifiedTurn<T>>, CertifyError> {
    let (t0, t1) = (spec.param_start, spec.param_end);
    // ---- Check 2: interval span (forward direction; winding bound) —
    // see the check-sequence docs. Spans are metered as arc length so
    // they classify against the linear band like every other margin
    // (dimensional honesty). The forward claim is "definitely apart",
    // the inf side, at the CARRIER'S OWN parameter rate — a circle's
    // radius, an ellipse's smaller semi-axis, a spiric's minor radius,
    // a net's `speed_lower_bound` — the numbers `pcurve_cache::param_rate`
    // mints for the same kinds. The winding bound is the turn rule
    // (`geom_core::period`), whose `Whole` reads the sup side.
    let span = t1 - t0;
    // The span decision runs once, before the schedule: not a sample.
    let span_escalated = |cause: Indeterminate| CertifyError::Escalated {
        check: CertCheck::ParamSpan,
        sample: NOT_A_SAMPLE,
        cause,
    };
    let not_forward = |refused: NotForward| match refused {
        NotForward::Undecided(cause) => span_escalated(cause),
        NotForward::Zero(margin) => CertifyError::IntervalNotForward {
            verdict: Refused::Zero(Classified { margin, band }),
        },
        NotForward::Reversed(margin) => CertifyError::IntervalNotForward {
            verdict: Refused::Negative { margin },
        },
    };
    let forward = |span: Margin<T>| -> Result<(), CertifyError> {
        let decided = decide_reported(FORWARD_ROW, span, band).map_err(span_escalated)?;
        match Refused::of(decided, band) {
            None => Ok(()),
            Some(verdict) => Err(CertifyError::IntervalNotForward { verdict }),
        }
    };
    // Zero (exactly a full period, the scaffolding/rim case) and Positive
    // (a partial arc) certify, and the certificate keeps which; Over is
    // the alias family. A turn the rule cannot settle escalates.
    let wound = |turn: Result<Turn, TurnUndecided>| match turn {
        Ok(turn) => turn.within().ok_or(CertifyError::WindingExceeded),
        Err(undecided) => Err(CertifyError::Escalated {
            check: CertCheck::ParamWinding,
            sample: NOT_A_SAMPLE,
            cause: undecided.diag,
        }),
    };
    let one_turn = Period::new(T::tau());
    // A periodic carrier's span is forward at the carrier's speed floor
    // (`geom::carrier_speed::speed_floor`, the one spelling the conic
    // readers share): a circle's radius; an ellipse's smaller semi-axis
    // magnitude (|dP/dθ| ≥ min(a, b), so the span times it is a
    // certified lower bound on the child's arc length — the semi-axes'
    // ORDER is not this gate's to decide, `Conic` — with a non-positive
    // `minor` refused as `IntervalNotForward`, a door's `StatedInterval`,
    // and a negative `major` (its `u_ref` flipped, the same locus)
    // admitted as it always has, for tier 3 to refuse on its value); a
    // spiric's MINOR radius (`|dP/dv| ≥ r`). The winding bound is the
    // 8kτ sample-alias argument about the parameter period, which all
    // three share.
    Ok(match &spec.carrier {
        periodic @ (Curve3::Circle { .. } | Curve3::Ellipse { .. } | Curve3::Spiric { .. }) => {
            let Some(floor) = geom::carrier_speed::speed_floor(periodic) else {
                unreachable!("check 2: `speed_floor` answers every periodic carrier")
            };
            let extent = Extent::forward(FORWARD_ROW, t0, t1, floor, band).map_err(not_forward)?;
            let turn = if let Curve3::Circle { .. } = periodic {
                let rows = TurnRows::exact(WINDING_ROWS.turn);
                one_turn.turn(rows, extent, Exact::new(floor.get()), band)
            } else {
                // The ellipse and the spiric read the turn between the
                // inf and the sup over the turn's defect of this window.
                let rate = geom::carrier_speed::turn_defect_rate(periodic, t0, extent);
                one_turn.turn(WINDING_ROWS, extent, rate, band)
            };
            Some(CertifiedTurn {
                params: (t0, t1),
                extent,
                turn: wound(turn)?,
            })
        }
        Curve3::Line { .. } => {
            forward(Margin::of(span))?;
            None
        }
        // Rung-3 carriers (M5 PR 9): the span is metered through the
        // certified speed lower bound (`m/parameter`, the split-meter
        // substrate — C12.3). The meter is gated definitely-positive
        // FIRST (the collapsed-arm idiom): a zero/negative meter (a
        // carrier whose speed genuinely collapses) or poison (a
        // malformed net) cannot convert the span to metres, and no
        // forward verdict may be fabricated from it — refuse, never
        // guess. RATIONAL carriers used to land here unconditionally;
        // since M7 they have their own arm of the meter and state a
        // real bound (`speed_lower_bound`'s rational derivation).
        //
        // The gated quantity is a LENGTH, not the bare rate: the rate
        // is metres per parameter unit, so what must be definitely
        // positive is the metre extent it subtends over the carrier's
        // OWN knot domain — a lower bound on the net's arc length.
        // That comparand is reparametrization-invariant (a carrier
        // reparametrized `t → 2t` halves the rate and doubles the
        // domain), which the bare rate is not, and it is the quantity
        // ε classifies under D4. The two failure modes stay distinct:
        // a collapsed meter is `SpanMeterCollapsed` and an undecided one
        // escalates under `ParamSpanMeter`, while a backwards or zero
        // span is `IntervalNotForward` below.
        //
        // The domain needs no orientation check of its own: `KnotVector`
        // mints only `d0 < d1` (non-decreasing knots, end runs of exactly
        // `degree + 1`), and `d1 - d0` of two distinct finite `f64`s in
        // that order is positive, so the sign this gate refuses is the
        // meter's alone.
        Curve3::Nurbs(n) => {
            let meter = n.speed_lower_bound();
            let (d0, d1) = n.knot_values().domain();
            let net_length = Margin::metered(d1 - d0, meter);
            decide_positive("nurbs_span_meter", net_length, band).map_err(|cause| {
                match Refused::rejected(&cause) {
                    Some(verdict) => CertifyError::SpanMeterCollapsed { verdict },
                    None => CertifyError::Escalated {
                        check: CertCheck::ParamSpanMeter,
                        sample: NOT_A_SAMPLE,
                        cause,
                    },
                }
            })?;
            forward(Margin::metered(span, meter))?;
            None
        }
    })
}

/// **The canonical description** (U2/D4): a chart edge's image minted,
/// every other description restated. Construction, run after the
/// interval checks so no chart image is derived from an interval the
/// kernel has not accepted.
fn canonical_of<T: Decide>(
    spec: &EdgeCurveSpec<T>,
    resolved: &Resolved<T>,
    band: Band,
) -> Result<EdgeDescription<T>, CertifyError> {
    // ---- The chart image (U2/D4): the certified description. ----
    // A derived image is minted HERE, after the interval checks and
    // before the residual schedule, so a degenerate span still refuses
    // in its own order and no chart image is derived from an interval
    // the kernel has not yet accepted.
    Ok(match resolved {
        Resolved::Intersection { witness, .. }
        | Resolved::PlaneNurbs { witness, .. }
        | Resolved::PairNurbs { witness, .. } => {
            let EdgeDescriptionSpec::Intersection { pair, .. } = spec.description else {
                // Unreachable: exactly one spec arm resolves either
                // way. Typed rather than assumed (D4 ¶2).
                return Err(CertifyError::Unimplemented);
            };
            EdgeDescription::Intersection {
                pair,
                selector: CurveSelector::Witness(*witness),
            }
        }
        Resolved::SharedRow { .. } => {
            let EdgeDescriptionSpec::Intersection { pair, ref selector } = spec.description else {
                return Err(CertifyError::Unimplemented);
            };
            EdgeDescription::Intersection {
                pair,
                selector: selector.clone(),
            }
        }
        Resolved::Tangent { witness, .. } => {
            let EdgeDescriptionSpec::TangentIntersection { pair, .. } = spec.description else {
                return Err(CertifyError::Unimplemented);
            };
            EdgeDescription::TangentIntersection {
                pair,
                witness: *witness,
            }
        }
        Resolved::Scaffold(mc) => EdgeDescription::Scaffold(*mc),
        Resolved::OnPair { .. } | Resolved::OnPairNurbs { .. } => {
            let EdgeDescriptionSpec::OnPair { pair, .. } = spec.description else {
                return Err(CertifyError::Unimplemented);
            };
            EdgeDescription::OnPair { pair }
        }
        Resolved::Chart {
            surface,
            key,
            image,
            wrap,
            declared: _,
        } => {
            let pcurve = match image {
                // D4, the stated half: a spline chart's image IS the
                // constructor's own iso data, exactly, and a
                // re-certification restates the image the edge already
                // carries. `v` is affine in the carrier parameter, so
                // the chart image is `P(t) = (u, v0 + slope·(t − t0))`
                // written on the carrier's own parameter — the form
                // every stored cache of this class already carries,
                // which is what makes description and cache the same
                // object.
                Some(p) => {
                    // A stated image is metered against the chart below;
                    // whether the carrier lies on the chart is read first,
                    // as the mint reads it (D3), so a carrier off its face
                    // ends as that read, never as the residual's meter.
                    crate::pcurve_cache::read_incidence(
                        &spec.carrier,
                        spec.param_start,
                        spec.param_end,
                        surface,
                        band,
                    )
                    .map_err(|e| mint_refusal(e, surface, &spec.carrier))?;
                    p.clone()
                }
                // D4, the minted half: an analytic chart's image is
                // derived from the carrier through the one door that
                // derives chart images anywhere in this kernel.
                None => chart_pcurve_over(
                    &spec.carrier,
                    spec.param_start,
                    spec.param_end,
                    surface,
                    band,
                )
                .map_err(|e| mint_refusal(e, surface, &spec.carrier))?,
            };
            EdgeDescription::Chart(ChartCurve {
                surface: *key,
                pcurve,
                wrap: *wrap,
            })
        }
    })
}

/// **A chart image's refusal, as the edge certificate's**: an incidence
/// read ended by its read (D3), decided or in band; a class reading or
/// the general lane's own, "not yet"; any other, the pair having no
/// certified image. The mint and a stated image's incidence read refuse
/// through it alike.
fn mint_refusal<T: Real>(
    e: PcurveCertifyError,
    surface: &Surface<T>,
    carrier: &Curve3<T>,
) -> CertifyError {
    use crate::{IncidenceRead, OffMargin};
    // A carrier not shown on its chart ends by its read (D3), decided or
    // in band: the one classifier, so this map and every reader of the
    // refusal agree.
    if let Some(read) = IncidenceRead::of(&e) {
        let why = match e {
            PcurveCertifyError::CarrierOffChart { why, .. } => why,
            _ => "the carrier's distance from the chart is past the band",
        };
        let off = |margin| CertifyError::CarrierOffChart {
            chart: surface.kind(),
            carrier: carrier.kind(),
            why,
            margin,
        };
        return match read {
            IncidenceRead::Off(margin) => off(margin.map(OffMargin::Past)),
            IncidenceRead::AtPose(margin) => off(Some(OffMargin::InBand(margin))),
            // The mint runs ONCE, before the schedule, so the sample
            // field carries the not-a-sample sentinel: reporting `0`
            // would name a schedule point that was never visited.
            IncidenceRead::Undecided(cause) => CertifyError::Escalated {
                check: CertCheck::ChartIncidence,
                sample: NOT_A_SAMPLE,
                cause,
            },
        };
    }
    match e {
        // The class reading is a lane choice (D3): in band it stays its
        // own check, which ends "not yet", never the mint's defect.
        PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartClass,
            cause,
            ..
        } => CertifyError::Escalated {
            check: CertCheck::ChartClass,
            sample: NOT_A_SAMPLE,
            cause,
        },
        // The general lane's own reading where the lane choice sent the
        // carrier: its capability gap, "not yet" as well.
        PcurveCertifyError::Escalated {
            check: PcurveCheck::GeneralLane,
            cause,
            ..
        } => CertifyError::Escalated {
            check: CertCheck::GeneralLane,
            sample: NOT_A_SAMPLE,
            cause,
        },
        PcurveCertifyError::Escalated { cause, .. } => CertifyError::Escalated {
            check: CertCheck::ChartImage,
            sample: NOT_A_SAMPLE,
            cause,
        },
        PcurveCertifyError::NoGeneralLane {
            chart,
            carrier,
            gap,
        } => CertifyError::NoGeneralLane {
            chart,
            carrier,
            gap,
        },
        // Read above: an incidence refusal never reaches here.
        PcurveCertifyError::CarrierOffChart { .. } => {
            unreachable!("mint_refusal: IncidenceRead::of reads every CarrierOffChart")
        }
        // Every other refusal says the same thing: this (chart, carrier)
        // pair has no certified chart image. That is a fact about the
        // DESCRIPTION's geometry, and it is reported as one — not as
        // `Unimplemented`, which means something else and would send the
        // caller looking for a missing feature instead of a wrong locus.
        PcurveCertifyError::UnsupportedChart { .. }
        | PcurveCertifyError::ImageMismatch { .. }
        | PcurveCertifyError::FittedLaneUnsupported { .. }
        | PcurveCertifyError::SectorRefused { .. }
        | PcurveCertifyError::IsoUnsupported { .. }
        | PcurveCertifyError::ChartRow { .. }
        | PcurveCertifyError::FittedLimb { .. }
        | PcurveCertifyError::FittedCertificate { .. }
        | PcurveCertifyError::CarrierDomain(_)
        | PcurveCertifyError::CarrierEnclosure(_)
        | PcurveCertifyError::FittedEscalated { .. }
        | PcurveCertifyError::IntervalNotForward
        | PcurveCertifyError::ChartWindingUnsupported
        | PcurveCertifyError::ChannelKindMismatch { .. }
        | PcurveCertifyError::WindingMismatch { .. }
        | PcurveCertifyError::PlaceholderChart
        | PcurveCertifyError::AzimuthPeriodExceeded
        | PcurveCertifyError::PlacementTurnsExceeded
        | PcurveCertifyError::TubePeriodExceeded
        | PcurveCertifyError::ResidualExceeded { .. }
        | PcurveCertifyError::Band(_) => CertifyError::ChartImageUnavailable {
            chart: surface.kind(),
            carrier: carrier.kind(),
        },
    }
}

/// **The mint** — what every scalar's edge door constructs: check 1,
/// the resolution, check 2 and the canonical description, in the order
/// [`run_checks`] runs them. No sample is read and nothing is certified.
fn mint_checks<T: Decide>(
    spec: &EdgeCurveSpec<T>,
    surfaces: &impl Fn(SurfaceKey) -> Option<Surface<T>>,
    band: Band,
) -> Result<(EdgeDescription<T>, Option<CertifiedTurn<T>>), CertifyError> {
    let resolved = resolve_spec(spec, surfaces)?;
    let turn = decide_turn(spec, band)?;
    let canonical = canonical_of(spec, &resolved, band)?;
    Ok((canonical, turn))
}

/// What the certification engine derives: the certificate, the
/// canonical description and check 2's turn.
type Checked<T> = (Certificate<T>, EdgeDescription<T>, Option<CertifiedTurn<T>>);

/// The certification engine (check sequence documented on
/// [`EdgeCurve::certify`]): the mint's steps, with the endpoint pins
/// between check 2 and the canonical description, then the sampled
/// checks and C2's limbs through `lane`.
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
fn run_checks<T: Decide + geom_core::CertifiedBounds>(
    spec: &EdgeCurveSpec<T>,
    start: Point3<T>,
    end: Point3<T>,
    surfaces: &impl Fn(SurfaceKey) -> Option<Surface<T>>,
    reach: T,
    lane: NurbsLane<T>,
    stated: Option<&StatedImage<T>>,
    band: Band,
) -> Result<Checked<T>, CertifyError> {
    let resolved = resolve_spec(spec, surfaces)?;
    let turn = decide_turn(spec, band)?;
    let mut max_residual = T::zero();
    let (t0, t1) = (spec.param_start, spec.param_end);

    // ---- Check 3: endpoint pinning. ----
    check_residual(
        "carrier_endpoint_start",
        CertCheck::EndpointStart,
        0,
        Margin::of(spec.carrier.eval(t0).distance(start)),
        band,
        &mut max_residual,
    )?;
    check_residual(
        "carrier_endpoint_end",
        CertCheck::EndpointEnd,
        CERT_SAMPLES - 1,
        Margin::of(spec.carrier.eval(t1).distance(end)),
        band,
        &mut max_residual,
    )?;

    let canonical = canonical_of(spec, &resolved, band)?;

    // ---- Check 4: per-sample description residuals. ----
    // Jet-schedule accumulators (TangentIntersection only): the worst
    // sampled residual for the C2.2 hull statement, and the weakest
    // sampled second-order data for the tube statement.
    let mut tangent_resid_max = T::zero();
    let mut tangent_gap_max = T::zero();
    let mut tangent_prev: Option<(T, T, T)> = None;
    let mut tangent_kappa_min = T::from_f64(f64::MAX);
    let mut tangent_arm_min = T::from_f64(f64::MAX);
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        // Each arm takes the point through the door it needs: the
        // `Tangent` arm's interior samples read the tangent too, from
        // the same jet pass as the point; every other arm, and that
        // arm's end samples, take the point alone, so no pass computes
        // a tangent that is then discarded.
        match &resolved {
            Resolved::Intersection {
                surfs: [(k1, surf1), (k2, surf2)],
                ..
            } => {
                let p = spec.carrier.eval(t);
                check_residual(
                    "carrier_on_surface_1",
                    CertCheck::SurfaceResidual { surface: *k1 },
                    i,
                    Margin::of(implicit_residual(surf1, p)),
                    band,
                    &mut max_residual,
                )?;
                check_residual(
                    "carrier_on_surface_2",
                    CertCheck::SurfaceResidual { surface: *k2 },
                    i,
                    Margin::of(implicit_residual(surf2, p)),
                    band,
                    &mut max_residual,
                )?;
                if i > 0 && i < CERT_SAMPLES - 1 {
                    match wedge_decided(surf1, surf2, p, reach, band) {
                        Ok((DihedralClass::Transverse, _)) => {}
                        Ok((DihedralClass::Smooth, margin)) => {
                            let verdict = Refused::Zero(Classified { margin, band });
                            return Err(CertifyError::NotTransverse {
                                sample: i,
                                lever: None,
                                verdict,
                            });
                        }
                        Err(WedgeEscalation::Lever(escalation)) => {
                            if let Some(verdict) = escalation.collapsed_arm() {
                                return Err(CertifyError::ArmCollapsed { sample: i, verdict });
                            }
                            return Err(CertifyError::Escalated {
                                check: match escalation.rung() {
                                    crate::LeverRung::Arm => CertCheck::TransversalityArm,
                                    crate::LeverRung::Reading => CertCheck::Transversality,
                                },
                                sample: i,
                                cause: escalation.diag(),
                            });
                        }
                        Err(WedgeEscalation::NoTangentPlane(cause)) => {
                            return Err(CertifyError::Escalated {
                                check: CertCheck::TangentPlanes,
                                sample: i,
                                cause,
                            });
                        }
                    }
                }
            }
            // The C7 jet schedule, per sample: the pair's separation
            // along the shared normal within ε (the touch, read before
            // the witness), both implicit residuals within ε; at
            // interior samples the second-order margin
            // definitely positive FIRST (the IFT denominator — and the
            // parallelism lever arm's own validity gate), then normal
            // parallelism within the derived threshold at lever arm
            // r = 1/κ_rel (D2 verbatim, D4 ¶1). A second-order refusal
            // is named by a definite parallelism defect at the folded
            // arm when there is one, so a definite first-order defect is
            // reported as itself whatever the surfaces' order or the
            // scalar.
            Resolved::Tangent {
                surfs: [(k1, surf1), (k2, surf2)],
                ..
            } => {
                let (p, tau) = if i > 0 && i < CERT_SAMPLES - 1 {
                    let (p, tau) = spec.carrier.ders1(t);
                    (p, Some(tau))
                } else {
                    (spec.carrier.eval(t), None)
                };
                let r1 = implicit_residual(surf1, p);
                let r2 = implicit_residual(surf2, p);
                let sigma = gap_orientation(surf1, surf2, p, reach, i, band)?;
                // The span line reads `s` under one σ per span: each end
                // under its own σ and under the other end's, so the bound
                // holds over the span whichever orientation the shared
                // normal has there.
                tangent_gap_max = tangent_gap_max.max((r1 - sigma * r2).abs());
                if let Some((p1, p2, prev_sigma)) = tangent_prev {
                    tangent_gap_max = tangent_gap_max
                        .max((p1 - sigma * p2).abs())
                        .max((r1 - prev_sigma * r2).abs());
                }
                tangent_prev = Some((r1, r2, sigma));
                tangent_resid_max = tangent_resid_max.max(r1.abs()).max(r2.abs());
                check_residual(
                    "tangent_gap",
                    CertCheck::TangentGap,
                    i,
                    Margin::of(r1 - sigma * r2),
                    band,
                    &mut max_residual,
                )?;
                check_residual(
                    "tangent_on_surface_1",
                    CertCheck::SurfaceResidual { surface: *k1 },
                    i,
                    Margin::of(r1),
                    band,
                    &mut max_residual,
                )?;
                check_residual(
                    "tangent_on_surface_2",
                    CertCheck::SurfaceResidual { surface: *k2 },
                    i,
                    Margin::of(r2),
                    band,
                    &mut max_residual,
                )?;
                if let Some(tau) = tau {
                    // The must-carry rule's one spelling
                    // (`crate::tangent_second_order`): the constructor
                    // that stores a `TangentIntersection` and this
                    // certificate that accepts one must read the same
                    // sagitta under the same predicate name, or the
                    // stored set and the certified set are two sets.
                    //
                    // The frame's escalation log carries exactly the
                    // escalation the returned error carries, so both
                    // readings run detached and two are spliced without
                    // theirs (`k_stats::splice_superseded`):
                    // - the naming reading's: it never refuses, it only
                    //   names a refusal already made;
                    // - a renamed refusal's second-order one: no sub-box
                    //   can certify. The defect is definite at the folded
                    //   arm on every sub-box, and where a sub-box's
                    //   sagitta resolves, its lever `1/|κ_rel|` is at
                    //   least half that arm (`|κ_rel| ≤ κ₁ + κ₂`, each
                    //   `κᵢ ≤ 1/curvature_lever_arm`), so the defect there
                    //   is above `K/2 · ε > ε`. Two premises are not
                    //   invariants: K > 2 is the ratified K = 10 (`Tol`
                    //   asks only K > 1), and `κᵢ ≤ 1/curvature_lever_arm`
                    //   fails on a fat torus (R < 2r). Where they fail,
                    //   a sub-box could certify, and the drop is still
                    //   safe: a failed node with an empty log bisects.
                    let (so, second_order) = geom_core::k_stats::detached(|| {
                        crate::tangent_second_order(surf1, surf2, p, tau, reach, band)
                    });
                    let (jet, arm) = (so.jet, so.arm);
                    let refused = match so.verdict {
                        Ok(decided) => Refused::of(decided, band).map(Ok),
                        Err(cause) => Some(Err(cause)),
                    };
                    match refused {
                        None => geom_core::k_stats::splice(second_order),
                        Some(refused) => {
                            // No lever `1/κ_rel` exists, so the
                            // parallelism defect is metered at the folded
                            // arm the sagitta was read over; a DEFINITE
                            // defect there is the refusal's cause (why
                            // `κ_rel` alone cannot tell a crossing from an
                            // osculating pair: `crate::TangentJet`). An
                            // in-band or zero reading is no more definite
                            // than the second-order one, which stands.
                            let renamed = named_by_parallelism(jet.sin_theta, arm, band);
                            if renamed.is_some() {
                                geom_core::k_stats::splice_superseded(second_order);
                            } else {
                                geom_core::k_stats::splice(second_order);
                            }
                            return Err(match (renamed, refused) {
                                (Some(margin), _) => CertifyError::ResidualExceeded {
                                    check: CertCheck::TangentParallel,
                                    sample: i,
                                    margin,
                                },
                                (None, Err(cause)) => CertifyError::Escalated {
                                    check: CertCheck::TangentSecondOrder,
                                    sample: i,
                                    cause,
                                },
                                // A magnitude margin (`≥ 0`): Zero is the
                                // G2/osculating zero side; Negative is
                                // unreachable for a true magnitude.
                                (None, Ok(verdict)) => {
                                    CertifyError::NotSecondOrderSeparated { sample: i, verdict }
                                }
                            });
                        }
                    }
                    tangent_kappa_min = tangent_kappa_min.min(jet.kappa_rel.abs());
                    tangent_arm_min = tangent_arm_min.min(arm);
                    check_residual(
                        "tangent_normal_parallel",
                        CertCheck::TangentParallel,
                        i,
                        Margin::levered_inv(jet.sin_theta, jet.kappa_rel.abs()),
                        band,
                        &mut max_residual,
                    )?;
                }
            }
            // The fenced scaffolding arm (D3): the ONE conventional
            // residual with no chart to state itself against, because
            // a transient scaffolding edge has no surfaces yet. It
            // keeps its own meter for exactly as long as the fence
            // keeps it legal.
            Resolved::Scaffold(mc) => {
                let p = spec.carrier.eval(t);
                let s = T::from_f64(schedule_fraction(i, CERT_SAMPLES));
                check_residual(
                    "carrier_matches_mapped_source",
                    CertCheck::MappedSource,
                    i,
                    Margin::of(p.distance(mc.eval(s))),
                    band,
                    &mut max_residual,
                )?;
            }
            // **U2's one conventional arm, D1's one meter.**
            // `|C(t) − S(P(t))| ≤ ε` (C4 verbatim), on the carrier's
            // own parameter — the same statement, the same predicate
            // name and the same dimension as every stored pcurve
            // cache's schedule (`pcurve_map_residual`), so an edge
            // cannot certify as a description and refuse as a cache.
            // `S(P(t))` is a point ON the surface, so the meter is never
            // under `dist(C, surface)`, with equality where the image
            // names the foot point: a cone's ruling and rim images
            // project on the generator and read the perpendicular; a
            // sphere's and a small-radius cylinder's read within bits
            // of the implicit form (`tests/r2_probes.rs`).
            Resolved::Chart {
                surface, declared, ..
            } => {
                let Some(chart) = canonical.chart() else {
                    return Err(CertifyError::Unimplemented);
                };
                let p = spec.carrier.eval(t);
                let q = chart.pcurve.eval(t);
                check_residual(
                    "pcurve_map_residual",
                    CertCheck::ChartResidual,
                    i,
                    Margin::of(p.distance(surface.eval(q.x, q.y))),
                    band,
                    &mut max_residual,
                )?;
                // **The declaration is metered too, or it is not a
                // record of anything** (P-1b, reinstated in review).
                // A declared locus says a sketch entity under a sweep
                // map determined THIS curve; the claim is checkable
                // and pre-collapse it was checked on every certify,
                // because the pushforward was the description. It is
                // the same meter — same name, same `CertCheck`, same
                // quantity — applied where the datum went.
                //
                // Two of this unit's own defects were exactly what it
                // catches: a re-anchor that left the datum ending at
                // the wall's old radius, and an offset lane that
                // dropped it. Both would have been caught here rather
                // than by a row someone thought to write.
                mapped_source(declared.as_ref(), i, p, band, &mut max_residual)?;
            }
            // Incidence on both surfaces, as `Intersection` meters it,
            // and nothing about how they meet: admissibility is tier
            // 3's must-carry predicate.
            Resolved::OnPair {
                surfs: [(k1, surf1), (k2, surf2)],
                declared,
            } => {
                let p = spec.carrier.eval(t);
                check_residual(
                    "carrier_on_surface_1",
                    CertCheck::SurfaceResidual { surface: *k1 },
                    i,
                    Margin::of(implicit_residual(surf1, p)),
                    band,
                    &mut max_residual,
                )?;
                check_residual(
                    "carrier_on_surface_2",
                    CertCheck::SurfaceResidual { surface: *k2 },
                    i,
                    Margin::of(implicit_residual(surf2, p)),
                    band,
                    &mut max_residual,
                )?;
                mapped_source(declared.as_ref(), i, p, band, &mut max_residual)?;
            }
            // Both surfaces are the lane's, after the loop: its limbs
            // bound the plane's residual over the whole span as well as
            // the wall's. The declaration is metered here.
            Resolved::OnPairNurbs { declared, .. } => {
                let p = spec.carrier.eval(t);
                mapped_source(declared.as_ref(), i, p, band, &mut max_residual)?;
            }
            // The plane × NURBS lane owns its OWN schedule (denser
            // than this one, and shared with the certificate's foot
            // points), so it runs once after the loop rather than
            // per-sample here — see the block below check 4.
            Resolved::PlaneNurbs { .. } | Resolved::PairNurbs { .. } => {}
            // Each wall's row certificate owns its own schedule, read
            // once after the loop.
            Resolved::SharedRow { .. } => {}
        }
    }

    // ---- TangentIntersection only: the span-wide jet statements
    // (M5 PR 9). C2.2: the worst sampled residual plus the certified
    // quadratic sag stays within ε — a SUP bound, not a sampled max.
    // C2.3-style tube on the jet system: the sampled |κ_rel| minus
    // its certified drift stays definitely positive over every span,
    // so second-order separation (local uniqueness of the tangency
    // locus — the jet system's IFT margin) holds along the whole
    // edge. Outside the certified span-bound lane: typed refusal,
    // never a fallback. ----
    if let Resolved::Tangent {
        surfs: [(_, surf1), (_, surf2)],
        ..
    } = &resolved
    {
        let Some(bounds) = crate::tangent::tangent_span_bounds(surf1, surf2, &spec.carrier, t0, t1)
        else {
            return Err(CertifyError::TangentCertificateUnsupported);
        };
        check_residual(
            "tangent_hull_sup",
            CertCheck::TangentHull,
            NOT_A_SAMPLE,
            Margin::of(tangent_resid_max + bounds.residual_sag),
            band,
            &mut max_residual,
        )?;
        check_residual(
            "tangent_gap_hull_sup",
            CertCheck::TangentHull,
            NOT_A_SAMPLE,
            Margin::of(tangent_gap_max + bounds.gap_sag),
            band,
            &mut max_residual,
        )?;
        tube_separation(tangent_kappa_min, bounds.kappa_drift, tangent_arm_min, band)?;
    }

    // The spline lanes lever their transversality and size their
    // uniqueness tube at the edge's own extent, beside the analytic
    // arms' faces' `reach`.
    let extent = crate::edge_extent_inf(&spec.carrier, t0, t1, start.distance(end));
    // ---- Intersection, plane × NURBS only (M7-8), over a carrier of
    // any kind: the whole declare-and-check certificate (C2's three
    // limbs, limbs 1–2 reading the carrier through the
    // carrier-enclosure door) — the closed-form plane residual,
    // the certified foot residual on the wall, the between-samples
    // sup bound, the per-sample transversality and the uniqueness
    // tube. The lane refuses typed WITH its measured bound; a
    // transversality failure lands in this module's existing
    // vocabulary, exactly as the analytic arm's does. ----
    if let Resolved::PlaneNurbs { plane, wall, .. } = &resolved {
        // `plane_nurbs_pair` admits only a wall with a chart.
        let Some(wall) = wall.spline_chart() else {
            return Err(CertifyError::Unimplemented);
        };
        // The lane decided every residual folded into both limbs at
        // `band`; they enter the certificate's worst residual and are
        // not decided again.
        let limbs = lane
            .limbs(&spec.carrier, (t0, t1), plane, wall, extent, band)
            .map_err(from_plane_nurbs)?;
        max_residual = max_residual
            .max(limbs.on_locus_max.abs())
            .max(limbs.hull_sup.abs());
    }

    // ---- OnPair, plane × NURBS, over a carrier of any kind: the lane's
    // incidence on both operands (limbs 1–2), and nothing about how
    // they meet. The wall side is metered against its spline chart, an
    // `Approx` wall's fit; incidence on that wall's description adds the
    // fit's own recorded distance, which this certificate does not
    // read (`work/tangtail/an-approx-walls-edge-residual-is-recorded-against-its-fit.md`,
    // the `Intersection` lane's posture too). ----
    if let Resolved::OnPairNurbs {
        plane,
        wall,
        wall_key,
        ..
    } = &resolved
    {
        // The wall's image a caller states (the pcurve the mint stored
        // for this edge on the wall), certified as it stands.
        let stated = stated.filter(|s| s.surface == *wall_key).map(|s| &s.pcurve);
        let incidence = lane
            .incidence(&spec.carrier, (t0, t1), plane, wall, stated, band)
            .map_err(from_plane_nurbs)?;
        max_residual = max_residual
            .max(incidence.on_locus_max.abs())
            .max(incidence.hull_sup.abs());
    }

    // ---- Intersection, NURBS × NURBS, over a carrier of any kind:
    // the same lane's certificate against both charts, its tube the
    // two-chart one. ----
    if let Resolved::PairNurbs { a, b, .. } = &resolved {
        let (Some(a), Some(b)) = (a.spline_chart(), b.spline_chart()) else {
            return Err(CertifyError::Unimplemented);
        };
        let limbs = lane
            .pair_limbs(&spec.carrier, (t0, t1), a, b, extent, band)
            .map_err(from_plane_nurbs)?;
        max_residual = max_residual
            .max(limbs.on_locus_max.abs())
            .max(limbs.hull_sup.abs());
    }

    // ---- Intersection under a shared row (D2's structural selector):
    // each wall's certificate of its own row image, limbs 1 and 2 on
    // that chart (C2), and the walls definitely transverse along the
    // row at the feet it states. No limb 3: the row is the curve. ----
    if let Resolved::SharedRow { walls } = &resolved {
        for (key, wall, image) in walls {
            let cache = crate::pcurve_cache::PcurveCache::certify(
                image.clone(),
                t0,
                t1,
                &spec.carrier,
                wall,
                band,
            )
            .map_err(|e| row_image_refusal(*key, e))?;
            let (on_locus, hull) = cache.certificate().limbs();
            max_residual = max_residual.max(on_locus.abs()).max(hull.abs());
        }
        let [(_, a, image_a), (_, b, image_b)] = walls;
        let (Some(a), Some(b)) = (a.spline_chart(), b.spline_chart()) else {
            return Err(CertifyError::Unimplemented);
        };
        crate::edge_nurbs::row_transversality((t0, t1), (a, image_a), (b, image_b), extent, band)
            .map_err(|e| from_plane_nurbs(e.lane()))?;
        let span = (t0.lo(), t1.hi());
        // A row image moves monotonically along one coordinate
        // (`description::row_of`, the admission guard), so over a piece it
        // lies in the hull of its two ends' images.
        let read = |lo: f64, hi: f64| {
            let (lo_t, hi_t) = (T::from_computed(lo), T::from_computed(hi));
            let bracket = |x: T| (x.lo(), x.hi());
            let ends = |image: &Pcurve<T>, k: usize| {
                let at = |t: T| {
                    let p = image.eval(t);
                    bracket(if k == 0 { p.x } else { p.y })
                };
                [at(lo_t), at(hi_t)]
            };
            crate::ssi::row_piece([
                ends(image_a, 0),
                ends(image_a, 1),
                ends(image_b, 0),
                ends(image_b, 1),
            ])
        };
        let crossing = crate::ssi::row_crossing((a, b), span, read).map_err(
            |crate::ssi::RowUnreadable(wall, refusal)| CertifyError::Row {
                surface: walls[wall].0,
                refusal: RowRefusal::ChartSpeed(refusal),
            },
        )?;
        row_crossing_verdict(crossing, extent, band)?;
    }

    // ---- Intersection of two analytic surfaces: C2's limbs, through
    // the lane, which holds certification arithmetic — the carrier's
    // distance from each operand over the edge's interval for every
    // kind, and the uniqueness tube over it for a spline. A spline
    // operand's pair is the NURBS pairs' lane above, whose limbs are
    // its own. ----
    if let Resolved::Intersection {
        surfs: [(_, surf1), (_, surf2)],
        ..
    } = &resolved
    {
        lane.analytic_limbs(&spec.carrier, (t0, t1), surf1, surf2, band)
            .map_err(CertifyError::AnalyticRung3)?;
    }

    // ---- Check 5: witness residuals + mid-parameter pin
    // (Intersection and TangentIntersection; see the check-sequence
    // docs for the witness contract: the witness IS the edge's
    // mid-parameter point). ----
    if let Resolved::Intersection {
        surfs: [(k1, surf1), (k2, surf2)],
        witness,
        ..
    }
    | Resolved::Tangent {
        surfs: [(k1, surf1), (k2, surf2)],
        witness,
    } = &resolved
    {
        check_residual(
            "witness_on_surface_1",
            CertCheck::WitnessSurfaceResidual { surface: *k1 },
            NOT_A_SAMPLE,
            Margin::of(implicit_residual(surf1, *witness)),
            band,
            &mut max_residual,
        )?;
        check_residual(
            "witness_on_surface_2",
            CertCheck::WitnessSurfaceResidual { surface: *k2 },
            NOT_A_SAMPLE,
            Margin::of(implicit_residual(surf2, *witness)),
            band,
            &mut max_residual,
        )?;
        let mid = spec.carrier.mid_point(t0, t1);
        check_residual(
            "witness_at_mid_parameter",
            CertCheck::WitnessMidpoint,
            (CERT_SAMPLES - 1) / 2,
            Margin::of(mid.distance(*witness)),
            band,
            &mut max_residual,
        )?;
    }

    // The plane × NURBS witness (M7-8): the plane side is the same
    // closed-form residual the analytic arm checks; the wall side is
    // discharged by the lane's own schedule, which contains the
    // mid-parameter exactly (`PXN_FIT_SAMPLES` is odd). The
    // mid-parameter pin is unchanged — the witness contract does not
    // move at this rung.
    // Two charts: both sides are discharged by the lane's schedule, and
    // the mid-parameter pin is the same.
    if let Resolved::PairNurbs { witness, .. } = &resolved {
        let mid = spec.carrier.mid_point(t0, t1);
        check_residual(
            "witness_at_mid_parameter",
            CertCheck::WitnessMidpoint,
            (CERT_SAMPLES - 1) / 2,
            Margin::of(mid.distance(*witness)),
            band,
            &mut max_residual,
        )?;
    }

    if let Resolved::PlaneNurbs {
        plane_key,
        plane,
        witness,
        ..
    } = &resolved
    {
        check_residual(
            "witness_on_surface_1",
            CertCheck::WitnessSurfaceResidual {
                surface: *plane_key,
            },
            NOT_A_SAMPLE,
            Margin::of(implicit_residual(plane, *witness)),
            band,
            &mut max_residual,
        )?;
        let mid = spec.carrier.mid_point(t0, t1);
        check_residual(
            "witness_at_mid_parameter",
            CertCheck::WitnessMidpoint,
            (CERT_SAMPLES - 1) / 2,
            Margin::of(mid.distance(*witness)),
            band,
            &mut max_residual,
        )?;
    }

    Ok((
        Certificate {
            samples: CERT_SAMPLES,
            max_residual,
        },
        canonical,
        turn,
    ))
}

// The pair's two surfaces resolved, in key order: the checks run
// in that order and each names its surface by key.
enum Resolved<T: Real> {
    Intersection {
        surfs: [(SurfaceKey, Surface<T>); 2],
        witness: Point3<T>,
    },
    Tangent {
        surfs: [(SurfaceKey, Surface<T>); 2],
        witness: Point3<T>,
    },
    /// D2's conventional pair: incidence on both surfaces, metered
    /// as `Intersection` meters it, and the declaration's meter as
    /// on [`Resolved::Chart`].
    OnPair {
        surfs: [(SurfaceKey, Surface<T>); 2],
        declared: Option<crate::mapped::MappedCurve<T>>,
    },
    /// D2's conventional pair on a PLANE and a spline chart (a
    /// described NURBS wall, or an `Approx` one's fit): the lane's
    /// incidence on both over the whole span, and the declaration's
    /// meter at each sample as on [`Resolved::OnPair`].
    OnPairNurbs {
        plane: Surface<T>,
        wall_key: SurfaceKey,
        wall: Surface<T>,
        declared: Option<crate::mapped::MappedCurve<T>>,
    },
    /// D3's fenced scaffolding door: a pushforward standing in
    /// as a description while the edge is TRANSIENT.
    Scaffold(crate::mapped::MappedCurve<T>),
    /// U2's ONE conventional form: the chart, its arena key, and
    /// the chart-image data the pcurve is minted from (D4 — the
    /// mint runs after the interval checks, so a degenerate span
    /// still refuses in its own order).
    Chart {
        surface: Surface<T>,
        key: SurfaceKey,
        /// `Some` for an image the CONSTRUCTION states (D4:
        /// spline charts take the image from the caller, which
        /// every iso constructor already knows, and a
        /// re-certification restates the image the edge already
        /// carries); `None` for an analytic chart, whose image is
        /// minted here through [`crate::chart_pcurve`].
        image: Option<Pcurve<T>>,
        /// D1's wrap edge ([`ChartCurve::wrap`]).
        wrap: bool,
        /// U2 Q3's authority payload, carried here for ONE reason: so
        /// the meter that used to run on it still runs. No ending
        /// reads it (D3: provenance only).
        ///
        /// Pre-collapse a declared locus WAS the description and
        /// took the scaffolding arm, whose
        /// `carrier_matches_mapped_source` residual ran on every
        /// certify. The collapse moved the payload out of the
        /// description and left the meter behind on an arm the
        /// payload no longer takes — so a declaration could be
        /// arbitrarily false and nothing said so (measured by R1:
        /// a placement ~1000 units off the body certified clean
        /// and passed tier 3). This is the SAME check, the same
        /// `CertCheck::MappedSource`, the same quantity and the
        /// same predicate name, following the datum to where it
        /// now lives — not a new predicate.
        declared: Option<crate::mapped::MappedCurve<T>>,
    },
    /// `Intersection` of a PLANE and a described NURBS wall or a
    /// fitted one (M7-8): the declare-and-check lane's shape, with
    /// the lane that derives its limbs. `wall` has a spline chart.
    PlaneNurbs {
        plane_key: SurfaceKey,
        plane: Surface<T>,
        wall: Surface<T>,
        witness: Point3<T>,
    },
    /// `Intersection` under a shared row: each wall, its key and its
    /// row image, in key order.
    SharedRow {
        walls: [(SurfaceKey, Surface<T>, Pcurve<T>); 2],
    },
    /// `Intersection` of two spline charts, each described or
    /// fitted: the same lane's NURBS × NURBS limbs.
    PairNurbs {
        a: Surface<T>,
        b: Surface<T>,
        witness: Point3<T>,
    },
}

/// The NURBS × NURBS pairing: two charts each a described NURBS
/// surface or a fit ([`Surface::spline_chart`]), the mvfs placeholder
/// never one.
fn nurbs_pair<T: Real>(
    pair: SurfacePair,
    surfaces: &impl Fn(SurfaceKey) -> Option<Surface<T>>,
) -> Option<(Surface<T>, Surface<T>)> {
    let [k1, k2] = pair.keys();
    let (a, b) = (surfaces(k1)?, surfaces(k2)?);
    let chart = |s: &Surface<T>| match s {
        Surface::Nurbs(n) => !n.is_placeholder(),
        _ => s.spline_chart().is_some(),
    };
    (chart(&a) && chart(&b)).then_some((a, b))
}

/// The plane × NURBS pairing: exactly one PLANE (returned with its
/// key) and exactly one **described** NURBS wall (the mvfs placeholder
/// is a mid-surgery "no description yet" fact, never an operand), or one
/// fitted (`Approx`) wall, which is its fit here
/// ([`Surface::spline_chart`]): the fit's distance from its description
/// is the face's claim, re-derived per face at rest, and the edge's
/// limbs are measured against the fit alone.
///
/// `None` for every other pair, which then takes the analytic path and
/// its existing refusals verbatim.
fn plane_nurbs_pair<T: Real>(
    pair: SurfacePair,
    surfaces: &impl Fn(SurfaceKey) -> Option<Surface<T>>,
) -> Option<(SurfaceKey, Surface<T>, SurfaceKey, Surface<T>)> {
    let wall = |s: &Surface<T>| match s {
        Surface::Nurbs(n) => !n.is_placeholder(),
        _ => s.spline_chart().is_some(),
    };
    pair.keys().into_iter().find_map(|k| {
        let plane = surfaces(k).filter(|s| matches!(s, Surface::Plane { .. }))?;
        let other_key = pair.other(k)?;
        let other = surfaces(other_key).filter(|s| wall(s))?;
        Some((k, plane, other_key, other))
    })
}

/// **The lane's WIRING** — the rows that say which free function
/// [`NurbsLane::certified`]'s one field holds, rather than what that
/// function answered.
///
/// Why a wiring row compares pointers rather than outputs:
/// `crates/topo/tests/certified_enclosure_impl_census.rs`'s module doc.
/// The helper is instantiated once per certifying scalar, and that
/// census counts the instantiations against the tree's
/// `CertifiedEnclosure` impls.
/// [`SplineFoot::carrier_foot`]'s seed is what picks the foot: a point
/// moved along a carrier is read on the branch it was moved from, not
/// wherever a fixed start converges.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod foot_rows {
    use geom::NurbsCurve3;
    use geom_core::Point3;
    use geom_core::spline::KnotVector;

    use super::SplineFoot;

    /// A U-shaped cubic, `x = 6t(1 − t)`, `y = 3t² − 2t³`. The distance
    /// from its point at `t = 0.9`, `(0.54, 0.972)`, has a second local
    /// minimum near `t = 0.1` on the lower leg, about 0.94 m away, which
    /// is where Newton seeded at the domain start converges.
    #[test]
    fn the_seed_decides_which_stationary_point_is_the_foot() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
        ];
        let carrier = NurbsCurve3::new(knots, control, vec![1.0; 4]).unwrap();
        let target = carrier.eval(0.9);
        let foot = SplineFoot::<f64>::certified();
        let moved_from = foot.carrier_foot(&carrier, target, 0.85).unwrap();
        assert!(
            (moved_from.t - 0.9).abs() < 1e-9 && moved_from.distance < 1e-9,
            "seeded at the old parameter 0.85 the foot is the point itself, got t = {} at {} m",
            moved_from.t,
            moved_from.distance
        );
        let from_the_start = foot.carrier_foot(&carrier, target, 0.0).unwrap();
        assert!(
            from_the_start.t < 0.3 && from_the_start.distance > 0.5,
            "seeded at the domain start the foot is the lower leg's minimum, got t = {} at {} m",
            from_the_start.t,
            from_the_start.distance
        );
    }
}

#[cfg(test)]
mod wiring_rows {
    use super::{NurbsLane, SplineFoot};
    use crate::edge_nurbs::plane_nurbs_limbs;

    /// `Ok(())` when the fields hold the lane's certified functions;
    /// otherwise the first wrong field's name.
    fn holds_the_certified_nurbs_lane<T: geom_core::Decide + geom_core::CertifiedBounds>()
    -> Result<(), &'static str> {
        let lane = NurbsLane::<T>::certified();
        if !std::ptr::fn_addr_eq(
            lane.limbs,
            plane_nurbs_limbs::<T> as fn(_, _, _, _, _, _) -> _,
        ) {
            return Err("limbs is not `edge_nurbs::plane_nurbs_limbs`");
        }
        if !std::ptr::fn_addr_eq(
            lane.spline_face,
            NurbsLane::<T>::spline_face_meetings as fn(_, _, _, _) -> _,
        ) {
            return Err("spline_face is not `NurbsLane::spline_face_meetings`");
        }
        if !std::ptr::fn_addr_eq(
            lane.analytic_limbs,
            crate::edge_nurbs::analytic_edge_limbs::<T> as fn(_, _, _, _, _) -> _,
        ) {
            return Err("analytic_limbs is not `edge_nurbs::analytic_edge_limbs`");
        }
        if !std::ptr::fn_addr_eq(
            lane.pair_limbs,
            crate::edge_nurbs::nurbs_pair_limbs::<T> as fn(_, _, _, _, _, _) -> _,
        ) {
            return Err("pair_limbs is not `edge_nurbs::nurbs_pair_limbs`");
        }
        if !std::ptr::fn_addr_eq(
            lane.incidence,
            crate::edge_nurbs::plane_nurbs_incidence::<T> as fn(_, _, _, _, _, _) -> _,
        ) {
            return Err("incidence is not `edge_nurbs::plane_nurbs_incidence`");
        }
        if !std::ptr::fn_addr_eq(
            lane.composite,
            NurbsLane::<T>::composite_crossings as fn(_, _, _, _) -> _,
        ) {
            return Err("composite is not `NurbsLane::composite_crossings`");
        }
        if !std::ptr::fn_addr_eq(
            lane.surface_foot,
            geom::NurbsSurface::<T>::project as fn(_, _) -> _,
        ) {
            return Err("surface_foot is not `NurbsSurface::project`");
        }
        if !std::ptr::fn_addr_eq(
            lane.chart_arm,
            NurbsLane::<T>::certified_chart_arm as fn(_, _, _) -> _,
        ) {
            return Err("chart_arm is not `NurbsLane::certified_chart_arm`");
        }
        Ok(())
    }

    #[test]
    fn f64_is_wired_to_the_certified_nurbs_lane() {
        assert_eq!(
            holds_the_certified_nurbs_lane::<f64>(),
            Ok(()),
            "`NurbsLane::<f64>::certified()` holds something other than `plane_nurbs_limbs`, `nurbs_pair_limbs`, `plane_nurbs_incidence`, `analytic_edge_limbs` and `composite_crossings`"
        );
    }

    /// The symbolic tier holds the base scalar's lane: the same body,
    /// instantiated at `Sym<f64>`.
    #[test]
    fn sym_over_f64_is_wired_to_the_certified_nurbs_lane() {
        assert_eq!(
            holds_the_certified_nurbs_lane::<geom_core::Sym<f64>>(),
            Ok(()),
            "`NurbsLane::<Sym<f64>>::certified()` holds something other than its routines"
        );
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_is_wired_to_the_certified_nurbs_lane() {
        assert_eq!(
            holds_the_certified_nurbs_lane::<geom_core::Probe>(),
            Ok(()),
            "`NurbsLane::<Probe>::certified()` holds something other than its routines"
        );
    }

    #[test]
    fn interval_is_wired_to_the_certified_nurbs_lane() {
        assert_eq!(
            holds_the_certified_nurbs_lane::<geom_core::interval::Interval>(),
            Ok(()),
            "`NurbsLane::<Interval>::certified()` holds something other than its routines"
        );
    }

    /// `Ok(())` when the foot holds `SplineFoot::seeded_foot`.
    fn holds_the_certified_spline_foot<T: geom_core::CertifiedBounds>() -> Result<(), &'static str>
    {
        let foot = SplineFoot::<T>::certified();
        if !std::ptr::fn_addr_eq(foot.foot, SplineFoot::<T>::seeded_foot as fn(_, _, _) -> _) {
            return Err("foot is not `SplineFoot::seeded_foot`");
        }
        Ok(())
    }

    #[test]
    fn f64_is_wired_to_the_certified_spline_foot() {
        assert_eq!(holds_the_certified_spline_foot::<f64>(), Ok(()));
    }

    #[test]
    fn sym_over_f64_is_wired_to_the_certified_spline_foot() {
        assert_eq!(
            holds_the_certified_spline_foot::<geom_core::Sym<f64>>(),
            Ok(())
        );
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_is_wired_to_the_certified_spline_foot() {
        assert_eq!(
            holds_the_certified_spline_foot::<geom_core::Probe>(),
            Ok(())
        );
    }

    #[test]
    fn interval_is_wired_to_the_certified_spline_foot() {
        assert_eq!(
            holds_the_certified_spline_foot::<geom_core::interval::Interval>(),
            Ok(())
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::Tol;
    use geom_core::spline::KnotVector;
    use geom_core::{
        Affine3, Arc2, DAMAGED_FILE_ENDING, KERNEL_DEFECT_ENDING, KERNEL_LIMIT_RECOURSE,
        KERNEL_OR_FILE_DEFECT_ENDING, Point2, Vec3,
    };
    use geom_core::{FileCoincidence, MarginDiag};

    use crate::recourse::Reading;

    use crate::mapped::{MappedCurve, MappedSource, SketchSegment};

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    fn eps() -> f64 {
        Tol::witness().get().eps
    }

    /// The one schedule's ends are the interval's own bits on an
    /// interval where the computed last sample misses `t₁` by an ulp,
    /// and the chart image's 33-point schedule passes through every
    /// certification sample bit for bit — the superset limb 1
    /// re-projects on.
    #[test]
    fn the_schedule_assigns_its_ends_and_the_image_schedule_contains_the_certificates() {
        let (t0, t1) = (0.3_f64, 0.9_f64);
        let computed = t0 + (t1 - t0) * 1.0;
        assert_ne!(computed.to_bits(), t1.to_bits(), "the fixture must miss t1");
        let fit = crate::edge_nurbs::PXN_FIT_SAMPLES;
        let stride = (fit - 1) / (CERT_SAMPLES - 1);
        for k in 0..CERT_SAMPLES {
            let cert = sample_param(t0, t1, k);
            let image = schedule_param(t0, t1, k * stride, fit);
            assert_eq!(
                cert.to_bits(),
                image.to_bits(),
                "sample {k}: {cert} vs {image}"
            );
        }
        assert_eq!(sample_param(t0, t1, 0).to_bits(), t0.to_bits(), "first end");
        assert_eq!(
            sample_param(t0, t1, CERT_SAMPLES - 1).to_bits(),
            t1.to_bits(),
            "last end"
        );
    }

    /// The middle station of every odd schedule is the parameter
    /// [`Curve3::mid_point`] evaluates at, bit for bit, on an interval
    /// where the uniform spelling `t₀ + (t₁ − t₀)·½` lands an ulp away.
    #[test]
    fn the_middle_station_is_the_edges_mid_param() {
        let (t0, t1) = (0.3_f64, 0.9_f64);
        let mid = geom::mid_param(t0, t1);
        let uniform = t0 + (t1 - t0) * 0.5;
        assert_ne!(
            mid.to_bits(),
            uniform.to_bits(),
            "the fixture must split the spellings"
        );
        for samples in [CERT_SAMPLES, crate::edge_nurbs::PXN_FIT_SAMPLES] {
            let station = schedule_param(t0, t1, (samples - 1) / 2, samples);
            assert_eq!(
                station.to_bits(),
                mid.to_bits(),
                "{samples}-point schedule: middle station {station} vs mid_param {mid}"
            );
        }
    }

    /// Every member of the residual taxonomy, for the two censuses
    /// below. Held total against the enum by
    /// [`all_is_the_whole_taxonomy`]'s compile-time visit, not by
    /// review.
    fn all_checks() -> [CertCheck; 32] {
        let surface = SurfaceKey::default();
        [
            CertCheck::ParamSpan,
            CertCheck::ParamSpanMeter,
            CertCheck::ParamWinding,
            CertCheck::ImageSpan,
            CertCheck::EndpointStart,
            CertCheck::EndpointEnd,
            CertCheck::SurfaceResidual { surface },
            CertCheck::WitnessSurfaceResidual { surface },
            CertCheck::WitnessMidpoint,
            CertCheck::Transversality,
            CertCheck::TransversalityTube,
            CertCheck::TransversalityArm,
            CertCheck::TangentPlanes,
            CertCheck::TangentParallel,
            CertCheck::TangentGap,
            CertCheck::TangentSecondOrder,
            CertCheck::TangentHull,
            CertCheck::TangentTube,
            CertCheck::MappedSource,
            CertCheck::ChartImage,
            CertCheck::ChartClass,
            CertCheck::ChartIncidence,
            CertCheck::GeneralLane,
            CertCheck::ChartResidual,
            CertCheck::PlaneNurbsOnLocus,
            CertCheck::PlaneNurbsHull,
            CertCheck::PlaneNurbsHullValue,
            CertCheck::AnalyticBreakResidual,
            CertCheck::AnalyticHull,
            CertCheck::PlaneNurbsReportedTransversality,
            CertCheck::PlaneNurbsChartSpeed,
            CertCheck::PlaneNurbsChartSpeedBound,
        ]
    }

    /// **[`all_checks`] is the WHOLE taxonomy**, pinned against a
    /// compile-time visit rather than reviewed.
    ///
    /// The match below is exhaustive with no wildcard, so a check added
    /// to [`CertCheck`] makes this file fail to compile until it is
    /// visited here, and every arm names the same total, so visiting it
    /// means writing the new count — which then reds until
    /// [`all_checks`] has grown too.
    ///
    /// The no-repeats half is what makes the count a census: with every
    /// entry distinct, a `len` equal to the number of rows means the
    /// list holds each of them exactly once.
    #[test]
    fn all_is_the_whole_taxonomy() {
        let rows = match CertCheck::ParamSpan {
            CertCheck::ParamSpan => 32,
            CertCheck::ParamSpanMeter => 32,
            CertCheck::ParamWinding => 32,
            CertCheck::ImageSpan => 32,
            CertCheck::EndpointStart => 32,
            CertCheck::EndpointEnd => 32,
            CertCheck::SurfaceResidual { .. } => 32,
            CertCheck::WitnessSurfaceResidual { .. } => 32,
            CertCheck::WitnessMidpoint => 32,
            CertCheck::Transversality => 32,
            CertCheck::TransversalityTube => 32,
            CertCheck::TransversalityArm => 32,
            CertCheck::TangentPlanes => 32,
            CertCheck::TangentParallel => 32,
            CertCheck::TangentGap => 32,
            CertCheck::TangentSecondOrder => 32,
            CertCheck::TangentHull => 32,
            CertCheck::TangentTube => 32,
            CertCheck::MappedSource => 32,
            CertCheck::ChartImage => 32,
            CertCheck::ChartClass => 32,
            CertCheck::ChartIncidence => 32,
            CertCheck::GeneralLane => 32,
            CertCheck::ChartResidual => 32,
            CertCheck::PlaneNurbsOnLocus => 32,
            CertCheck::PlaneNurbsHull => 32,
            CertCheck::PlaneNurbsHullValue => 32,
            CertCheck::AnalyticBreakResidual => 32,
            CertCheck::AnalyticHull => 32,
            CertCheck::PlaneNurbsReportedTransversality => 32,
            CertCheck::PlaneNurbsChartSpeed => 32,
            CertCheck::PlaneNurbsChartSpeedBound => 32,
        };
        for (i, check) in all_checks().iter().enumerate() {
            assert!(
                !all_checks()[..i].contains(check),
                "{check:?} appears twice in all_checks()"
            );
        }
        assert_eq!(
            all_checks().len(),
            rows,
            "all_checks() has drifted from the taxonomy — it holds {} rows, the taxonomy has \
             {rows}",
            all_checks().len()
        );
    }

    /// **Every check says itself by one phrase no other check says, and
    /// no check's phrase is its variant identifier.**
    ///
    /// The `Display` arms are an exhaustive match, so the words cannot
    /// fall BEHIND the taxonomy — a check without a word does not
    /// compile. What hand-written phrases CAN do is collide,
    /// and several of these are one token apart by design (the
    /// carrier's residual against the witness point's), so a literal copied onto a neighbouring row is the
    /// live failure mode, and a refusal naming a phrase two checks
    /// share cannot say which check refused.
    ///
    /// The second half is what tells the word apart from `Debug` at run
    /// time, and it is the row a bug breaks: the phrase is what a
    /// person would write, never the taxonomy's coordinate for the
    /// check, so a `Display` deleted, forwarded to `Debug`, or written
    /// to spell the identifier reds on the first row.
    #[test]
    fn the_taxonomy_says_each_check_by_one_unshared_phrase() {
        let mut said: Vec<(String, CertCheck)> = Vec::new();
        for check in all_checks() {
            let word = check.to_string();
            let shared = said.iter().find(|(w, _)| *w == word).map(|(_, o)| *o);
            assert!(
                shared.is_none(),
                "{check:?} shares its phrase \"{word}\" with {shared:?} — a refusal naming \
                 that phrase cannot say which check refused"
            );
            assert_ne!(
                word,
                format!("{check:?}"),
                "{check:?} says its own identifier; a check is named to the reader in the \
                 words the surrounding sentence is written in, not by the taxonomy's \
                 coordinate for it"
            );
            said.push((word, check));
        }
    }

    /// The three refusal sentences that name a check say it in words.
    ///
    /// The definite arm is pinned whole: it is this module's own
    /// sentence end to end. The two escalated arms are pinned by
    /// PREFIX, up to and including the check's phrase — their tail is
    /// [`Indeterminate`]'s `Display`, whose own docs say message pins
    /// take it with `contains` and never whole.
    #[test]
    fn the_refusal_sentences_name_the_check_in_words() {
        assert_eq!(
            CertifyError::ResidualExceeded {
                check: CertCheck::TangentHull,
                sample: 4,
                margin: MarginDiag::value(3e-8),
            }
            .render(Reading::Build),
            "the between-samples sag bound at sample 4 definitely exceeds the tolerance \
             band (the certificate cannot show the cache represents the description, D4 ¶2). \
             Recourse: loosen the tolerance, as a last resort; this refusal may indicate a \
             kernel bug worth reporting"
        );

        let cause = Indeterminate {
            margin: MarginDiag::value(0.0),
            band: band(),
            predicate: Some("a_probe"),
            terminal_sliver: false,
        };
        let not_sampled = CertifyError::Escalated {
            check: CertCheck::ParamSpan,
            sample: NOT_A_SAMPLE,
            cause,
        }
        .to_string();
        assert!(
            not_sampled.starts_with(
                "the stored parameter interval's length (not a sampled check) escalated: "
            ),
            "the not-a-sample escalation reads {not_sampled:?}"
        );
        let sampled = CertifyError::Escalated {
            check: CertCheck::Transversality,
            sample: 3,
            cause,
        }
        .to_string();
        assert!(
            sampled.starts_with("the crossing angle at sample 3 escalated: "),
            "the sampled escalation reads {sampled:?}"
        );
    }

    /// **A sized decision is told in one noun**: its check's word, which
    /// its undecided arm renders, names its size, and each of its definite
    /// payloads names the word's quantity, at every door that refuses it.
    /// Literals pin today's words; the relations pin the rule, so a pair
    /// that drifts together in both still goes red.
    #[test]
    fn each_decision_is_told_in_one_noun() {
        use crate::edge_nurbs::{AnalyticRung3Refusal, PlaneNurbsRefusal};
        use crate::ssi::{PointLever, SsiError};

        let band = Band::new(1e-9, 1e-8).unwrap();
        let zero = Refused::Zero(Classified {
            margin: MarginDiag::value(0.0),
            band,
        });
        let below = Refused::Negative {
            margin: MarginDiag::value(-5e-9),
        };
        let cause = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band,
            predicate: Some("a_probe"),
            terminal_sliver: false,
        };
        // (check, its word, its size, what the word and every payload
        // say, the definite payloads)
        let rows: [(CertCheck, &str, &str, &str, Vec<String>); 8] = [
            (
                CertCheck::ParamSpan,
                "the stored parameter interval's length",
                "length",
                "length",
                vec![CertifyError::IntervalNotForward { verdict: zero }.to_string()],
            ),
            (
                CertCheck::ImageSpan,
                "the length of the edge's image on its wall's chart",
                "length",
                "length",
                vec![
                    CertifyError::PlaneNurbs(crate::edge_nurbs::PlaneNurbsRefusal::ArcRim(
                        crate::edge_nurbs::ArcRimRefusal::Refused {
                            iso: Some(crate::pcurve_cache::PcurveCheck::ParamSpan),
                            limb: crate::ssi::SsiLimb::HullSupChart,
                            bound: MarginDiag::value(5e-6),
                        },
                    ))
                    .to_string(),
                ],
            ),
            (
                CertCheck::ParamWinding,
                "the stored arc's headroom to one full turn of its carrier",
                "arc",
                "one full turn of its carrier",
                vec![
                    CertifyError::WindingExceeded.to_string(),
                    StatedSpan::Winding(WindingReading::InBand(cause)).to_string(),
                ],
            ),
            (
                CertCheck::ParamSpanMeter,
                "the lower bound on the spline carrier's length from its metered knot domain",
                "length",
                "the lower bound on the spline carrier's length from its metered knot domain",
                vec![
                    CertifyError::SpanMeterCollapsed { verdict: zero }.to_string(),
                    CertifyError::SpanMeterCollapsed { verdict: below }.to_string(),
                ],
            ),
            (
                CertCheck::Transversality,
                "the crossing angle",
                "angle",
                "the crossing angle",
                vec![
                    CertifyError::NotTransverse {
                        sample: 4,
                        lever: None,
                        verdict: zero,
                    }
                    .to_string(),
                    PlaneNurbsRefusal::NotTransverse {
                        sample: 4,
                        lever: PointLever::Extent,
                        verdict: below,
                    }
                    .to_string(),
                ],
            ),
            (
                CertCheck::TransversalityTube,
                "the uniqueness tube's lower bound on the crossing angle",
                "angle",
                "the uniqueness tube's lower bound on the crossing angle",
                vec![
                    PlaneNurbsRefusal::TubeStraddles {
                        verdict: zero,
                        boxes: 8,
                    }
                    .to_string(),
                    AnalyticRung3Refusal::TubeStraddles {
                        verdict: zero,
                        boxes: 8,
                    }
                    .to_string(),
                    SsiError::TubeStraddles {
                        verdict: zero,
                        boxes: 8,
                    }
                    .to_string(),
                ],
            ),
            (
                CertCheck::TangentSecondOrder,
                "the curvature difference",
                "curvature difference",
                "the curvature difference",
                vec![
                    CertifyError::NotSecondOrderSeparated {
                        sample: 4,
                        verdict: zero,
                    }
                    .to_string(),
                    CertifyError::NotSecondOrderSeparated {
                        sample: 4,
                        verdict: below,
                    }
                    .to_string(),
                ],
            ),
            (
                CertCheck::TangentTube,
                "the second-order tube's lower bound on the curvature difference",
                "curvature difference",
                "the second-order tube's lower bound on the curvature difference",
                vec![CertifyError::TubeNotSeparated { verdict: zero }.to_string()],
            ),
        ];
        for (check, word, size, noun, payloads) in rows {
            assert_eq!(check.to_string(), word, "{check:?}'s word");
            let Ending::Sized(decision) = check.ending() else {
                panic!("{check:?} is a sized decision")
            };
            assert_eq!(decision.size, size, "{check:?}'s size");
            // The rule itself: the word names the size its ending names,
            // and the noun every payload carries is the word's own.
            assert!(
                word.contains(decision.size),
                "{check:?}'s word {word:?} names its size {:?}",
                decision.size
            );
            assert!(
                word.contains(noun),
                "{check:?}'s word carries its payloads' {noun:?}"
            );
            let escalated = CertifyError::Escalated {
                check,
                sample: 2,
                cause,
            }
            .to_string();
            assert!(
                escalated.starts_with(&format!("{word} at sample 2 escalated: ")),
                "{check:?}'s undecided arm reads {escalated:?}"
            );
            for payload in payloads {
                assert!(
                    payload.contains(noun),
                    "{check:?}'s definite payload names {noun:?}: {payload:?}"
                );
            }
        }
        // A margin of an angle or a curvature difference in metres names
        // what levers it.
        for payload in [
            CertifyError::NotTransverse {
                sample: 4,
                lever: None,
                verdict: zero,
            },
            CertifyError::NotSecondOrderSeparated {
                sample: 4,
                verdict: zero,
            },
            CertifyError::TubeNotSeparated { verdict: below },
        ] {
            let payload = payload.to_string();
            assert!(
                payload.contains(", levered by the arm it is read over)"),
                "the analytic lane's margin in metres names its arm: {payload:?}"
            );
        }
        // The plane × NURBS lane's undecided crossing is certification's
        // sentence, word for word.
        assert_eq!(
            PlaneNurbsRefusal::TransversalityEscalated { sample: 2, cause }.to_string(),
            "the crossing angle at sample 2 escalated: margin 5e-9 lies inside the ambiguity \
             band (1e-9, 1e-8)"
        );
    }

    /// **An undefined tangent plane says what is undefined and why**,
    /// not the margin's numbers: its margin is invalid because the
    /// quantity it reads does not exist over the enclosure, so neither
    /// "margin is invalid" nor "may indicate a kernel bug" is the
    /// sentence. Pinned whole, at every reading.
    #[test]
    fn an_undefined_tangent_plane_names_its_cause() {
        let refusal = CertifyError::Escalated {
            check: CertCheck::TangentPlanes,
            sample: 4,
            cause: Indeterminate {
                margin: MarginDiag::INVALID,
                band: band(),
                predicate: Some("dihedral_wedge"),
                terminal_sliver: false,
            },
        };
        for reading in [Reading::Build, Reading::AtRest] {
            assert_eq!(
                refusal.render(reading),
                "the surfaces' tangent planes at sample 4 are undefined: a surface's gradient \
                 is zero or undefined somewhere over the enclosure of the point there, so no \
                 tangent plane, and no angle between the surfaces, is defined over it. \
                 Recourse: keep the edge clear of each surface's axis or centre; over a \
                 parameter box, a narrower box encloses the edge and its surfaces closer to \
                 where they are",
                "{reading:?}"
            );
        }
    }

    /// A resolver over a tiny fixed table (keys minted through a local
    /// slotmap, mirroring how a Body resolves).
    fn table(
        surfs: Vec<Surface<f64>>,
    ) -> (Vec<SurfaceKey>, impl Fn(SurfaceKey) -> Option<Surface<f64>>) {
        let mut map: slotmap::SlotMap<SurfaceKey, Surface<f64>> = slotmap::SlotMap::with_key();
        let keys: Vec<SurfaceKey> = surfs.into_iter().map(|s| map.insert(s)).collect();
        (keys, move |k| map.get(k).cloned())
    }

    fn line_spec(p0: Point3<f64>, p1: Point3<f64>) -> EdgeCurveSpec<f64> {
        EdgeCurveSpec::line_between(p0, p1)
    }

    // ------------------------------------------------------------------
    // **D2's bit-diff row**: the collapse's effect on
    // `Certificate.max_residual`, MEASURED per fixture rather than
    // asserted.
    //
    // The three pre-collapse conventional forms did not measure the
    // same thing — a seam stated its on-surface residual through the
    // IMPLICIT form, an iso curve through `|C − S(u, v)|` with `v`
    // affine in the SCHEDULE FRACTION, a mapped source against a
    // pushforward with no surface at all — so a single meter cannot
    // reproduce all three bit for bit, and pinning it per lane would
    // re-import the per-class branching the collapse exists to
    // remove. What is owed instead is honesty: each fixture's delta
    // is measured here, in ULPs, and moves only when the arithmetic
    // moves.
    //
    // Each row states the legacy expression VERBATIM — not a
    // re-association chosen to make a number match — and compares it
    // against what certification now records.
    // ------------------------------------------------------------------

    /// The bit-diff row's OWN band, built from the fixture's drift
    /// rather than from the run's ε.
    ///
    /// The band's zero threshold is four times the drift at every
    /// scale, so each row sits at the same fraction of its own band
    /// and the SWEPT variable is the drift alone. Deriving the band
    /// from the run's ε instead would make the row a different
    /// measurement on every ε point in the matrix, which is how the
    /// first version of this row passed locally and failed hosted.
    fn d2_band(drift: f64) -> Band {
        Band::new(4.0 * drift, 40.0 * drift).expect("the bit-diff row's own band")
    }

    /// ULP distance between two finite same-sign `f64`s.
    fn ulps(a: f64, b: f64) -> i64 {
        let (x, y) = (a.to_bits() as i64, b.to_bits() as i64);
        (x - y).abs()
    }

    /// The pre-collapse SEAM meter, verbatim: endpoint pins, then per
    /// sample the implicit-form residual and the two seam predicates.
    fn legacy_seam_max(
        spec: &EdgeCurveSpec<f64>,
        surface: &Surface<f64>,
        start: Point3<f64>,
        end: Point3<f64>,
    ) -> f64 {
        let (t0, t1) = (spec.param_start, spec.param_end);
        let mut m = spec.carrier.eval(t0).distance(start);
        m = m.max(spec.carrier.eval(t1).distance(end));
        for i in 0..CERT_SAMPLES {
            let p = spec.carrier.eval(sample_param(t0, t1, i));
            m = m.max(implicit_residual(surface, p).abs());
            if let Surface::Cylinder {
                origin,
                axis,
                u_ref,
                ..
            } = *surface
            {
                let d = p - origin;
                let w = d - axis * d.dot(axis);
                m = m.max(w.dot(axis.cross(u_ref)).abs());
                m = m.max((0.0 - w.dot(u_ref)).max(0.0).abs());
            }
        }
        m
    }

    /// The pre-collapse ISO meter, verbatim — `v0 + (v1 − v0)·frac`
    /// on the schedule FRACTION, which is the arithmetic order the
    /// collapse necessarily changes (a `Pcurve` is a function of the
    /// carrier's own parameter).
    fn legacy_iso_max(
        spec: &EdgeCurveSpec<f64>,
        surface: &Surface<f64>,
        u: f64,
        v0: f64,
        v1: f64,
        start: Point3<f64>,
        end: Point3<f64>,
    ) -> f64 {
        let (t0, t1) = (spec.param_start, spec.param_end);
        let mut m = spec.carrier.eval(t0).distance(start);
        m = m.max(spec.carrier.eval(t1).distance(end));
        for i in 0..CERT_SAMPLES {
            let p = spec.carrier.eval(sample_param(t0, t1, i));
            let frac = schedule_fraction(i, CERT_SAMPLES);
            let v = v0 + (v1 - v0) * frac;
            m = m.max(p.distance(surface.eval(u, v)));
        }
        m
    }

    /// **Acceptance 1**: `geom-brep` carries ONE conventional
    /// description form. The two pre-collapse conventional writings —
    /// a periodic seam and a chart iso curve — certify onto the same
    /// arm, and the fenced scaffolding door is the only other
    /// conventional shape that exists.
    #[test]
    fn the_collapse_leaves_one_conventional_form() {
        let r = 2.0;
        let (keys, lookup) = table(vec![Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }]);
        let (p0, p1) = (Point3::new(r, 0.0, 0.0), Point3::new(r, 0.0, 3.0));
        let seam = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::wrap(keys[0]),
                carrier: Curve3::Line {
                    origin: p0,
                    dir: Vec3::unit_z(),
                },
                param_start: 0.0,
                param_end: 3.0,
            },
            p0,
            p1,
            &lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .expect("the seam certifies");
        let chart = seam.description().chart().expect("a seam IS a chart image");
        assert_eq!(chart.surface, keys[0]);
        assert!(chart.wrap, "a seam carries D1's obligation");

        let plane = Surface::Plane {
            origin: Point3::new(0.25, -0.5, 1.0),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let (pk, plookup) = table(vec![plane]);
        let (u, v0, v1) = (0.3_f64, 0.7_f64, 2.9_f64);
        let s_at = |v: f64| plookup(pk[0]).unwrap().eval(u, v);
        let (q0, q1) = (s_at(v0), s_at(v1));
        let len = q0.distance(q1);
        let iso = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::iso(pk[0], u, v0, v1, 0.0, len),
                carrier: Curve3::Line {
                    origin: q0,
                    dir: (q1 - q0) / len,
                },
                param_start: 0.0,
                param_end: len,
            },
            q0,
            q1,
            &plookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .expect("the iso curve certifies");
        let iso_chart = iso
            .description()
            .chart()
            .expect("an iso curve IS a chart image");
        assert!(
            !iso_chart.wrap,
            "an iso boundary owes the meter and nothing else"
        );

        // The fenced scaffolding door is the only other conventional
        // shape, and it is NOT a chart image — it has no surface.
        let (a, b) = (Point3::new(-1.0, 0.25, 0.5), Point3::new(2.0, -3.0, 4.0));
        let (_, empty) = table(vec![]);
        let scaffold = EdgeCurve::certify(
            line_spec(a, b),
            a,
            b,
            &empty,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .expect("the scaffolding line certifies");
        assert!(scaffold.description().chart().is_none());
        assert!(matches!(
            scaffold.description(),
            EdgeDescription::Scaffold(_)
        ));
    }

    /// **The authority record** (U2 Q3): a declared locus answers
    /// `true` and carries its source; a derived one answers `false`.
    #[test]
    fn the_authority_record_says_who_declared_the_locus() {
        let (a, b) = (Point3::new(-1.0, 0.25, 0.5), Point3::new(2.0, -3.0, 4.0));
        let (_, empty) = table(vec![]);
        let declared = EdgeCurve::certify(
            line_spec(a, b),
            a,
            b,
            &empty,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .expect("the scaffolding line certifies");
        assert!(declared.authority().is_declared());
        assert!(matches!(
            declared.authority(),
            EdgeAuthority::Declared(MappedCurve {
                source: MappedSource::ExtrudedPoint { .. },
                ..
            })
        ));

        let r = 2.0;
        let (keys, lookup) = table(vec![Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }]);
        let (p0, p1) = (Point3::new(r, 0.0, 0.0), Point3::new(r, 0.0, 3.0));
        let derived = EdgeCurve::certify(
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::wrap(keys[0]),
                carrier: Curve3::Line {
                    origin: p0,
                    dir: Vec3::unit_z(),
                },
                param_start: 0.0,
                param_end: 3.0,
            },
            p0,
            p1,
            &lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .expect("the seam certifies");
        assert!(!derived.authority().is_declared());
    }

    /// One row of the DRIFT sweep: the seam class's ULP delta and the
    /// scaffolding control's, at a given in-band drift, in the order
    /// (cylinder-seam, mapped-line).
    ///
    /// The iso class is swept separately, over the variable it
    /// actually moves in — see [`d2_iso_delta`].
    fn d2_row(drift: f64) -> (i64, i64) {
        // ---- Fixture "cylinder-seam": the seam ruling of a radius-2
        // cylinder about +z, seam at +x. Legacy: implicit residual +
        // the two predicates. Now: |C − S(P)| + the same two
        // predicates (D1 keeps them).
        let r = 2.0;
        let cyl = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let (keys, lookup) = table(vec![cyl]);
        // Perturbed INSIDE the band: an exact ruling makes every meter
        // answer a bitwise zero, which measures nothing. `d` is the
        // radial drift a real construction leaves behind, and it is
        // what the two meters disagree about.
        let d = drift;
        let (p0, p1) = (Point3::new(r + d, 0.0, 0.0), Point3::new(r + d, 0.0, 3.0));
        let seam_spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::wrap(keys[0]),
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_z(),
            },
            param_start: 0.0,
            param_end: 3.0,
        };
        let cert = EdgeCurve::certify(
            seam_spec.clone(),
            p0,
            p1,
            &lookup,
            1.0,
            d2_band(drift),
            NurbsLane::certified(),
        )
        .expect("the seam certifies")
        .certificate;
        let legacy = legacy_seam_max(&seam_spec, &lookup(keys[0]).unwrap(), p0, p1);
        let seam_delta = ulps(cert.max_residual, legacy);

        // ---- Fixture "mapped-line": the fenced scaffolding arm. Its
        // meter is untouched by the collapse, so its delta is the
        // control: it must be exactly zero.
        let (a, b) = (Point3::new(-1.0, 0.25, 0.5), Point3::new(2.0, -3.0, 4.0));
        let mapped = line_spec(a, b);
        let (_, empty) = table(vec![]);
        let mapped_cert = EdgeCurve::certify(
            mapped.clone(),
            a,
            b,
            &empty,
            1.0,
            d2_band(drift),
            NurbsLane::certified(),
        )
        .expect("the mapped line certifies")
        .certificate;
        let mut mapped_legacy = mapped.carrier.eval(mapped.param_start).distance(a);
        mapped_legacy = mapped_legacy.max(mapped.carrier.eval(mapped.param_end).distance(b));
        let EdgeDescriptionSpec::Scaffold(mc) = mapped.description else {
            panic!("line_between describes a MappedCurve");
        };
        for i in 0..CERT_SAMPLES {
            let p = mapped
                .carrier
                .eval(sample_param(mapped.param_start, mapped.param_end, i));
            let s = schedule_fraction(i, CERT_SAMPLES);
            mapped_legacy = mapped_legacy.max(p.distance(mc.eval(s)));
        }
        let mapped_delta = ulps(mapped_cert.max_residual, mapped_legacy);

        (seam_delta, mapped_delta)
    }

    /// The ISO class's ULP delta at a given parameter anchor, with
    /// **no drift at all**.
    ///
    /// Zero drift is the point. With a drift the residual is
    /// `√(drift² + dz²)` and the mint's own arithmetic enters only
    /// quadratically, so a re-association of the mint is swamped and
    /// the row goes blind — which is how the first version of this
    /// fixture read 0 ULP while a laundering mutant survived. At zero
    /// drift the residual IS `|dz|`, linear in the mint's error, and
    /// any re-association shows at full strength.
    fn d2_iso_delta(anchor: f64) -> (i64, ChartCurve<f64>) {
        // Fixture "nurbs-iso": an iso curve of a described NURBS
        // wall over an interval that does NOT start at zero.
        //
        // **The chart is a spline one on purpose.** On every ANALYTIC
        // chart the v-channel is unit-speed, so a boundary iso's slope
        // `(v1 − v0)/(t1 − t0)` is exactly ±1 — and at slope 1 the
        // collapsed mint `(v0 − slope·t0) + slope·t` re-associates to
        // the legacy `v0 + (v1 − v0)·frac` bit for bit, which makes
        // the row BLIND to exactly the laundering D2 forbids. A
        // spline chart's v runs over its knot domain while the carrier
        // runs over arc length, so the slope is a real number; here it
        // is 1/3, asserted below in bits so the blindness cannot come
        // back unnoticed.
        let wall = {
            let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("u knots");
            let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("v knots");
            let control = vec![
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 3.0),
                Point3::new(2.0, 0.0, 0.0),
                Point3::new(2.0, 0.0, 3.0),
            ];
            Surface::Nurbs(std::sync::Arc::new(
                geom::NurbsSurface::new(ku, kv, control, vec![1.0; 4]).expect("wall builds"),
            ))
        };
        let (pk, plookup) = table(vec![wall]);
        // `v0` is deliberately NOT zero and the endpoints are
        // deliberately not round: at `v0 = 0` the mint's
        // `(v0 − slope·t0) + slope·t` collapses to `slope·t` and the
        // two expressions agree bitwise again — a second way for this
        // row to go blind, found by measuring rather than by reasoning
        // about it.
        let (u, v0, v1) = (0.0_f64, 0.13_f64, 0.83_f64);
        let s_at = |v: f64| plookup(pk[0]).unwrap().eval(u, v);
        let (q0, q1) = (s_at(v0), s_at(v1));
        let len = q0.distance(q1);
        let t1 = anchor + len;
        let dir = (q1 - q0) / len;
        assert!(
            (v1 - v0) / (t1 - anchor) != 1.0,
            "the iso fixture's slope must not be 1.0 — at slope 1 the two mint \
             expressions coincide bitwise and the row measures nothing"
        );
        // The in-band drift, off the chart along its own normal.
        let off = Vec3::zero();
        let iso_spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::iso(pk[0], u, v0, v1, anchor, t1),
            carrier: Curve3::Line {
                origin: q0 + off - dir * anchor,
                dir,
            },
            param_start: anchor,
            param_end: t1,
        };
        let iso_cert = EdgeCurve::certify(
            iso_spec.clone(),
            q0 + off,
            q1 + off,
            &plookup,
            1.0,
            d2_band(2.5e-10),
            NurbsLane::certified(),
        )
        .expect("the nurbs iso certifies");
        let iso_chart = iso_cert
            .description()
            .chart()
            .expect("an iso description IS a chart image")
            .clone();
        let iso_cert = iso_cert.certificate;
        let iso_legacy = legacy_iso_max(
            &iso_spec,
            &plookup(pk[0]).unwrap(),
            u,
            v0,
            v1,
            q0 + off,
            q1 + off,
        );
        let iso_delta = ulps(iso_cert.max_residual, iso_legacy);

        (iso_delta, iso_chart)
    }

    /// The iso fixture's minted chart image at a given anchor — the
    /// mint's own output, for the bit-pin row.
    fn d2_iso_chart(anchor: f64) -> (ChartCurve<f64>, f64) {
        let (delta, chart) = d2_iso_delta(anchor);
        #[allow(clippy::cast_precision_loss)]
        (chart, delta as f64)
    }

    /// **D2's bit-diff row, swept over drift scale.**
    ///
    /// A delta measured at one operating point is true where it was
    /// taken and says nothing about the meter; the sweep is what turns
    /// it into a measurement. Three decades, each pinned:
    ///
    /// | drift (m) | cylinder-seam | mapped-line (control) |
    /// |---|---|---|
    /// | 2.5e-7  | 293 601 280 | 0 |
    /// | 2.5e-10 | 0 | 0 |
    /// | 2.5e-13 | 0 | 0 |
    ///
    /// **The seam column is not scale-invariant**, so it may not be
    /// read as a statement about its meter: "the seam arm does not
    /// move" is true only below the coarse decade. Any claim about the
    /// size of the move is a claim about a scale, and this table is
    /// the only support for one.
    ///
    /// The mechanism is **not derived here** — the arm is
    /// bitwise-identical at both fine decades and moves only at the
    /// coarse one, which is the signature of a cancellation floor in
    /// the legacy `|radial| − r` rather than of the collapse.
    ///
    /// **What this table does NOT measure**, named so it is not
    /// over-read. Not the iso arm: that arm moves in the parameter
    /// ANCHOR, not in the drift, and is swept by
    /// [`d2_iso_move_is_unbounded_in_the_anchor_offset`] at zero
    /// drift. Not the seam meter across CHART KINDS, measured in
    /// `tests/r2_probes.rs`.
    ///
    /// The mapped-line column is the control: the fenced scaffolding
    /// arm's meter is untouched by the collapse, so it must read zero
    /// at every scale, and a nonzero entry there means the sweep
    /// itself is measuring the wrong thing.
    const D2_SWEEP: [(f64, (i64, i64)); 3] = [
        (2.5e-7, (D2_S0_SEAM, 0)),
        (2.5e-10, (D2_S1_SEAM, 0)),
        (2.5e-13, (D2_S2_SEAM, 0)),
    ];

    /// Drift 2.5e-7 m, cylinder-seam: **293 601 280 ULP**. The seam
    /// arm's two expressions do NOT agree at this scale — the legacy
    /// implicit form computes `|radial| − r`, a subtractive
    /// cancellation against the chart radius, where `|C − S(P)|`
    /// forms the same length directly. The move is toward the
    /// directly-formed one.
    const D2_S0_SEAM: i64 = 293_601_280;
    /// Drift 2.5e-10 m, cylinder-seam: **0 ULP**.
    const D2_S1_SEAM: i64 = 0;
    /// Drift 2.5e-13 m, cylinder-seam: **0 ULP**.
    const D2_S2_SEAM: i64 = 0;

    /// **R1's M4, executed**: the iso arm's move is unbounded in the
    /// ANCHOR OFFSET, so no single number is "the size of the move".
    ///
    /// The mint writes the chart image on the carrier's own parameter
    /// as `(v0 − slope·t0) + slope·t`. Both terms grow with `t0` while
    /// their difference does not, so the cancellation — and with it
    /// the departure from the legacy `v0 + (v1 − v0)·frac` — grows
    /// without bound as the edge's parameter anchor moves away from
    /// the origin. At `t0 = 0` the two expressions coincide exactly.
    ///
    /// **The direction is toward truth, and here is the bound.** The
    /// legacy expression evaluates `v` at the schedule FRACTION, which
    /// is not the quantity the description states: the description
    /// says `v` is affine in the CARRIER PARAMETER, and the carrier is
    /// what the residual is taken against. The collapsed expression
    /// evaluates the stated quantity. Both lose accuracy as `t0`
    /// grows, and they lose it in the same place — the carrier's own
    /// `origin + dir·t` suffers the identical cancellation — so the
    /// row bounds the DISAGREEMENT, not the error: it is at most the
    /// cancellation `t0·2⁻⁵³` propagated through the chart's v-scale,
    /// which is what the table below shows growing linearly in `t0`.
    /// An edge anchored 1e6 m from its own parameter origin has a
    /// representation problem the meter cannot fix and should not
    /// hide.
    /// **The mint's own bits**, pinned — the tripwire D2 actually
    /// asks for.
    ///
    /// The residual sweeps below measure the mint's EFFECT, and an
    /// effect can be swamped: a one-ULP change in the chart image
    /// survives a distance-to-surface only if nothing downstream
    /// rounds it away, which is how two earlier versions of this
    /// fixture read 0 ULP while the arithmetic underneath had moved.
    /// This row compares the mint's OUTPUT instead, so any
    /// re-expression of `(v0 − slope·t0) + slope·t` — however
    /// algebraically equal — changes a pinned bit pattern and fails
    /// here, whether or not it survives to the certificate.
    ///
    /// Demonstrated rather than hoped: a mutant that makes the
    /// collapsed meter reproduce the legacy `v0 + (v1 − v0)·frac`
    /// fails the anchor sweep below, and all three known
    /// re-associations of `v0 − slope·t0` fail here.
    ///
    /// **Two of those three were once recorded as "no-ops", wrongly.**
    /// They were checked against the four round anchors this row
    /// started with, agreed at all four, and were written up as
    /// bitwise-identical. They are not: one separates at ~50 % of
    /// anchors and the other at ~1.2 %. The separating anchors are
    /// pinned below for that reason, and the episode is recorded
    /// rather than quietly repaired — a false "we checked, it was a
    /// no-op" in the record is worse than an unexamined mutant,
    /// because it tells the next reader not to look.
    #[test]
    fn d2_the_mint_arithmetic_is_pinned_in_bits() {
        // **One anchor per mutant FAMILY, chosen by separation rather
        // than by roundness.** A re-association can be
        // bitwise-identical at some anchors and different at others,
        // so the anchors are not decoration — each is here because a
        // known mutant survives without it:
        //
        // - `1.7` separates `v1 − slope·t1` (the far-endpoint anchor),
        //   which is identical at every other entry;
        // - `84871.995…` separates `(v0·L − (v1−v0)·t0)/L` (the
        //   algebraic re-expression), which agrees at 0, 1.7, 1e3 and
        //   1e6 and separates at roughly HALF of random anchors;
        // - `4824.781…` separates the reciprocal-slope form
        //   `v0 − (v1−v0)·(1/L)·t0`, which separates at only ~1.2 % of
        //   anchors and so is the easiest of the three to miss.
        //
        // The last two were found by a reviewer executing mutants this
        // row's author had asserted were no-ops. They were not: they
        // coincided at the four anchors originally pinned. A tripwire
        // is only as wide as the cases it was checked against, and
        // "we tried it and nothing moved" is exactly what a too-narrow
        // one reports.
        const PINNED: [(f64, u64, u64); 6] = [
            (0.0, 0x3fc0_a3d7_0a3d_70a4, 0x3fd5_5555_5555_5556),
            (1.7, 0xbfdb_f258_bf25_8bf4, 0x3fd5_5555_5555_5556),
            (1.0e3, 0xc074_d340_da74_0d68, 0x3fd5_5555_5555_5514),
            (1.0e6, 0xc114_5854_d037_9507, 0x3fd5_5555_5556_5965),
            (
                84_871.995_158_921_64,
                0xc0db_a0a2_3e4e_7fe9,
                0x3fd5_5555_5555_1451,
            ),
            (
                4_824.781_053_628_38,
                0xc099_2085_7ac9_91b5,
                0x3fd5_5555_5555_5145,
            ),
        ];
        let measured: Vec<(f64, u64, u64)> = PINNED
            .iter()
            .map(|&(anchor, _, _)| {
                let (chart, _) = d2_iso_chart(anchor);
                let Pcurve::IsoLine { p0, pl } = chart.pcurve else {
                    panic!("the iso arm mints an IsoLine chart image");
                };
                (anchor, p0.y.to_bits(), pl.y.to_bits())
            })
            .collect();
        assert_eq!(
            measured,
            PINNED.to_vec(),
            "the iso mint's arithmetic moved. The mint is \
             `(v0 − slope·t0) + slope·t`, written that way ON PURPOSE — \
             re-expressing it is what D2 forbids, and restating these \
             constants to match a re-expression IS the laundering"
        );
    }

    #[test]
    fn d2_iso_move_is_unbounded_in_the_anchor_offset() {
        let measured: Vec<(f64, i64)> = D2_ANCHOR_SWEEP
            .iter()
            .map(|&(t0, _)| (t0, d2_iso_delta(t0).0))
            .collect();
        let pinned: Vec<(f64, i64)> = D2_ANCHOR_SWEEP.to_vec();
        assert_eq!(
            measured, pinned,
            "the anchor-offset sweep moved: measured (t0, iso ULP) = {measured:?}, \
             pinned {pinned:?}. Re-measure and RESTATE — and read the METRE \
             values (run with D2_DEBUG=1), not the ULP counts, for the size"
        );
    }

    /// The anchor-offset sweep's pinned table (see the test's docs),
    /// measured at zero drift so the residual IS the mint arithmetic:
    ///
    /// | `t0` (m) | legacy (m) | collapsed (m) | ULP | relative |
    /// |---|---|---|---|---|
    /// | 0     | 4.4409e-16 | 4.4409e-16 | 0 | 0 |
    /// | 1.7   | 4.4409e-16 | 4.4409e-16 | 0 | 0 |
    /// | 1e3   | 5.9064e-14 | 1.7053e-13 | 6.58e15 | **+189 %** |
    /// | 1e6   | 6.0536e-11 | 5.8208e-11 | 1.80e14 | **−3.85 %** |
    ///
    /// Read the ULP column as a tripwire and the relative column as
    /// the size: a ULP distance between two numbers of different
    /// ORDER is large by construction and says little on its own.
    ///
    /// The last row is the one to look at twice. At `t0 = 1e6` the
    /// residual is 6e-11 m — six per cent of a 1e-9 band — in BOTH
    /// meters, from parameter cancellation alone. Neither expression
    /// is trustworthy there, they disagree by 3.85 %, and the sign of
    /// the disagreement is not even constant across the sweep. That
    /// is the bound the D1 argument needs: the collapse evaluates the
    /// quantity the description STATES, but at a far enough anchor
    /// both evaluations are dominated by the carrier's own
    /// representation error, and no meter can repair that.
    const D2_ANCHOR_SWEEP: [(f64, i64); 4] =
        [(0.0, D2_A0), (1.7, D2_A1), (1.0e3, D2_A2), (1.0e6, D2_A3)];
    /// Anchor 0: the two expressions are identical (`slope·t0 = 0`).
    const D2_A0: i64 = 0;
    /// Anchor 1.7 m: still identical at this fixture's numbers.
    const D2_A1: i64 = 0;
    /// Anchor 1e3 m: the collapsed meter reads 2.9× the legacy one.
    const D2_A2: i64 = 6_579_477_580_611_584;
    /// Anchor 1e6 m: they disagree by 3.85 %, the other way.
    const D2_A3: i64 = 180_148_108_263_424;

    #[test]
    fn d2_bit_diff_row_is_measured_across_drift_scales() {
        // The WHOLE table is compared at once, deliberately: a
        // per-scale assertion stops at the first move and hides the
        // shape of the rest, and the shape is the measurement.
        let measured: Vec<(f64, (i64, i64))> =
            D2_SWEEP.iter().map(|&(d, _)| (d, d2_row(d))).collect();
        let pinned: Vec<(f64, (i64, i64))> = D2_SWEEP.to_vec();
        assert_eq!(
            measured, pinned,
            "D2 drift sweep moved — measured (drift, (cylinder-seam, mapped-line)) \
             = {measured:?}, pinned {pinned:?}. Re-measure and RESTATE the row; \
             never re-associate a meter to make a number match (D2)"
        );
    }

    #[test]
    fn self_loop_circle_certifies_at_its_anchor() {
        let p = Point3::new(-2.0, 0.5, 7.0);
        let spec = EdgeCurveSpec::self_loop_circle_at(p);
        EdgeCurve::certify(
            spec.clone(),
            p,
            p,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();
        // Coincident-endpoint LINE specs, by contrast, poison and are
        // refused (typed, no panic) — the reason the circle convention
        // exists.
        let bad = EdgeCurveSpec::line_between(p, p);
        assert!(
            EdgeCurve::certify(bad, p, p, |_| None, 1.0, band(), NurbsLane::certified()).is_err()
        );
    }

    #[test]
    fn mapped_line_certifies_and_is_deterministic() {
        let p0 = Point3::new(0.25, -1.0, 2.0);
        let p1 = Point3::new(1.25, 0.5, 3.5);
        let spec = line_spec(p0, p1);
        let a = EdgeCurve::certify(
            spec.clone(),
            p0,
            p1,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();
        let b = EdgeCurve::certify(
            spec.clone(),
            p0,
            p1,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();
        // D9: byte-identical certification records across runs.
        assert_eq!(
            format!("{:?}", a.certificate()),
            format!("{:?}", b.certificate())
        );
        assert_eq!(a.certificate().samples, CERT_SAMPLES);
        assert!(a.certificate().max_residual <= eps());
        // Recertification at rest agrees.
        a.recertify(p0, p1, |_| None, 1.0, band(), NurbsLane::certified())
            .unwrap();
    }

    #[test]
    fn wrong_cache_is_rejected() {
        // A carrier offset well beyond the escalation band (100·ε ≥ K·ε
        // at every CI ε row) must be rejected: the cache does not
        // represent the description.
        let p0 = Point3::new(0.0, 0.0, 0.0);
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let mut spec = line_spec(p0, p1);
        let off = 100.0 * eps();
        spec.carrier = Curve3::Line {
            origin: Point3::new(0.0, off, 0.0),
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(
            spec.clone(),
            p0,
            p1,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::ResidualExceeded {
                    check: CertCheck::EndpointStart,
                    sample: 0,
                    ..
                }
            ),
            "{err:?}"
        );
        // An in-band offset (3·ε) escalates instead of passing.
        let mut spec = line_spec(p0, p1);
        spec.carrier = Curve3::Line {
            origin: Point3::new(0.0, 3.0 * eps(), 0.0),
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(
            spec.clone(),
            p0,
            p1,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap_err();
        assert!(matches!(err, CertifyError::Escalated { .. }), "{err:?}");
    }

    /// **`with_remapped_surfaces`** (R1 MINOR-2 for PR #325): the
    /// transplant door. Only the description's surface HANDLES may
    /// differ — carrier, interval and certificate travel verbatim —
    /// and a handle the remap cannot answer for yields `None` rather
    /// than a dangling reference.
    #[test]
    fn remapping_surface_keys_changes_the_handles_and_nothing_else() {
        let planes = || {
            vec![
                Surface::Plane {
                    origin: Point3::origin(),
                    normal: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
                Surface::Plane {
                    origin: Point3::origin(),
                    normal: Vec3::unit_y(),
                    u_ref: Vec3::unit_x(),
                },
            ]
        };
        let (src_keys, src_lookup) = table(planes());
        // A SECOND table standing for the destination body's arenas:
        // the same surface VALUES under DIFFERENT keys, which is
        // exactly what a graft produces. Two fresh slotmaps mint the
        // same slots, so the destination is offset by a filler — the
        // handles have to actually differ for this row to mean
        // anything.
        let mut dst_surfaces = vec![Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_x(),
            u_ref: Vec3::unit_y(),
        }];
        dst_surfaces.extend(planes());
        let (dst_all, dst_lookup) = table(dst_surfaces);
        let dst_keys = [dst_all[1], dst_all[2]];
        assert!(
            src_keys[0] != dst_keys[0] && src_keys[1] != dst_keys[1],
            "the two tables must mint distinct handles"
        );
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(src_keys[0], src_keys[1]),
                selector: crate::CurveSelector::Witness(Point3::new(0.5, 0.0, 0.0)),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        let certified = EdgeCurve::certify(
            spec,
            p0,
            p1,
            &src_lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();

        let bridge = |k: SurfaceKey| src_keys.iter().position(|s| *s == k).map(|i| dst_keys[i]);
        let moved = certified
            .with_remapped_surfaces(bridge)
            .expect("every named surface has a destination");

        // The handles moved.
        match *moved.description() {
            EdgeDescription::Intersection {
                pair,
                selector: CurveSelector::Witness(witness),
            } => {
                assert_eq!(pair, SurfacePair::new(dst_keys[0], dst_keys[1]));
                // ...and the witness, a POINT, did not.
                assert!((witness.x - 0.5).abs() < 1e-15);
            }
            ref other => panic!("the description class changed: {other:?}"),
        }
        // Nothing else did.
        assert_eq!(
            format!("{:?}", moved.carrier()),
            format!("{:?}", certified.carrier())
        );
        assert_eq!(moved.params(), certified.params());
        assert_eq!(
            format!("{:?}", moved.certificate()),
            format!("{:?}", certified.certificate()),
            "the certificate travels verbatim — the geometry did not change"
        );
        // And the moved copy is genuinely certified AGAINST the
        // destination's surfaces: re-certification there agrees, which
        // is the claim the graft's `RemapKeys` bridge relies on.
        moved
            .recertify(p0, p1, &dst_lookup, 1.0, band(), NurbsLane::certified())
            .unwrap();

        // A remap that cannot answer writes nothing.
        assert!(
            certified.with_remapped_surfaces(|_| None).is_none(),
            "a dangling handle is never written"
        );
        // A description with no surface keys survives any remap.
        let mapped_only = EdgeCurve::certify(
            line_spec(p0, p1),
            p0,
            p1,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();
        let same = mapped_only
            .with_remapped_surfaces(|_| None)
            .expect("a MappedCurve names no surface");
        assert_eq!(
            format!("{:?}", same.description()),
            format!("{:?}", mapped_only.description())
        );
    }

    #[test]
    fn intersection_certifies_against_both_planes() {
        // The unit-cube edge x ∈ [0,1], y = 0, z = 0 as the intersection
        // of the bottom plane (z = 0) and front plane (y = 0).
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_y(),
                u_ref: Vec3::unit_x(),
            },
        ]);
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(keys[0], keys[1]),
                selector: crate::CurveSelector::Witness(Point3::new(0.5, 0.0, 0.0)),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        EdgeCurve::certify(
            spec.clone(),
            p0,
            p1,
            &lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();

        // Both-surface teeth: a carrier lying ON keys[0] but definitely
        // off keys[1] fails keys[1]'s residual specifically.
        let mut bad = spec.clone();
        bad.carrier = Curve3::Line {
            origin: Point3::new(0.0, 0.5, 0.0), // on z = 0, off y = 0
            dir: Vec3::unit_x(),
        };
        let err = EdgeCurve::certify(
            bad,
            Point3::new(0.0, 0.5, 0.0),
            Point3::new(1.0, 0.5, 0.0),
            &lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::ResidualExceeded {
                    check: CertCheck::SurfaceResidual { surface },
                    sample: 0,
                    ..
                } if surface == keys[1]
            ),
            "{err:?}"
        );

        // A displaced witness fails the witness checks.
        let mut bad = spec.clone();
        bad.description = EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(keys[0], keys[1]),
            selector: crate::CurveSelector::Witness(Point3::new(0.5, 0.0, 0.25)),
        };
        let err = EdgeCurve::certify(bad, p0, p1, &lookup, 1.0, band(), NurbsLane::certified())
            .unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::ResidualExceeded {
                    check: CertCheck::WitnessSurfaceResidual { surface },
                    sample: NOT_A_SAMPLE,
                    ..
                } if surface == keys[0]
            ),
            "{err:?}"
        );

        // Same surface twice is structurally malformed.
        let mut bad = spec.clone();
        bad.description = EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(keys[0], keys[0]),
            selector: crate::CurveSelector::Witness(Point3::new(0.5, 0.0, 0.0)),
        };
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &lookup, 1.0, band(), NurbsLane::certified())
                .unwrap_err(),
            CertifyError::IntersectionSameSurface { key: keys[0] }
        );

        // A stale key is a typed error.
        let mut bad = spec.clone();
        bad.description = EdgeDescriptionSpec::Intersection {
            pair: SurfacePair::new(keys[0], SurfaceKey::default()),
            selector: crate::CurveSelector::Witness(Point3::new(0.5, 0.0, 0.0)),
        };
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &lookup, 1.0, band(), NurbsLane::certified())
                .unwrap_err(),
            CertifyError::UnresolvedSurface {
                key: SurfaceKey::default()
            }
        );
    }

    #[test]
    fn tangent_intersection_is_refused() {
        // Two planes meeting at the run-scaled sliver angle: the
        // transversality check escalates (never a guess).
        let theta = 3.0 * eps();
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::new(0.0, theta.sin(), theta.cos()),
                u_ref: Vec3::unit_x(),
            },
        ]);
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(keys[0], keys[1]),
                selector: crate::CurveSelector::Witness(Point3::new(0.5, 0.0, 0.0)),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        let err = EdgeCurve::certify(
            spec.clone(),
            p0,
            p1,
            &lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::Escalated {
                    check: CertCheck::Transversality,
                    ..
                }
            ),
            "{err:?}"
        );
        // Exactly coincident planes: definitely smooth ⇒ NotTransverse.
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_y(),
            },
        ]);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(keys[0], keys[1]),
                selector: crate::CurveSelector::Witness(Point3::new(0.5, 0.0, 0.0)),
            },
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        assert_eq!(
            EdgeCurve::certify(
                spec.clone(),
                p0,
                p1,
                &lookup,
                1.0,
                band(),
                NurbsLane::certified()
            )
            .unwrap_err(),
            CertifyError::NotTransverse {
                lever: None,
                sample: 1,
                verdict: Refused::Zero(Classified {
                    margin: MarginDiag::value(0.0),
                    band: band(),
                }),
            }
        );
    }

    #[test]
    fn a_wrap_edge_certifies_wherever_the_construction_cut() {
        // Cylinder about +z through the origin, u_ref at +x. A wrap
        // edge sits where its construction cut the wall (D1), so the
        // ruling at u = 0 and its antipode both certify; a ruling off
        // the surface does not.
        let r = 2.0;
        let (keys, lookup) = table(vec![Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }]);
        let p0 = Point3::new(r, 0.0, 0.0);
        let p1 = Point3::new(r, 0.0, 3.0);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::wrap(keys[0]),
            carrier: Curve3::Line {
                origin: p0,
                dir: Vec3::unit_z(),
            },
            param_start: 0.0,
            param_end: 3.0,
        };
        EdgeCurve::certify(
            spec.clone(),
            p0,
            p1,
            &lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();

        let q0 = Point3::new(-r, 0.0, 0.0);
        let q1 = Point3::new(-r, 0.0, 3.0);
        let mut antipode = spec.clone();
        antipode.carrier = Curve3::Line {
            origin: q0,
            dir: Vec3::unit_z(),
        };
        let cut = EdgeCurve::certify(
            antipode,
            q0,
            q1,
            &lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();
        let chart = cut
            .description()
            .chart()
            .expect("a wrap edge is a chart image");
        assert!(chart.wrap, "the antipodal ruling carries the wrap flag");
        assert!(
            (chart.pcurve.eval(0.0).x - core::f64::consts::PI).abs() < 1e-12,
            "its image is the iso line at its own chart u: {:?}",
            chart.pcurve.eval(0.0)
        );
        let (o0, o1) = (
            Point3::new(r + 1.0, 0.0, 0.0),
            Point3::new(r + 1.0, 0.0, 3.0),
        );
        let mut off = spec.clone();
        off.carrier = Curve3::Line {
            origin: o0,
            dir: Vec3::unit_z(),
        };
        assert!(
            EdgeCurve::certify(off, o0, o1, &lookup, 1.0, band(), NurbsLane::certified()).is_err(),
            "a ruling off the cylinder is no image of it"
        );

        // A seam on a plane is malformed.
        let (pkeys, plookup) = table(vec![Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        }]);
        let mut bad = spec.clone();
        bad.description = EdgeDescriptionSpec::wrap(pkeys[0]);
        assert_eq!(
            EdgeCurve::certify(bad, p0, p1, &plookup, 1.0, band(), NurbsLane::certified())
                .unwrap_err(),
            CertifyError::WrapOnNonPeriodic
        );
    }

    /// **`OnPair` meters incidence on both surfaces, and nothing about
    /// how they meet.** A line in the floor `z = 0` along the generator
    /// of a cylinder resting on it certifies as the pair (a tangency the
    /// certificate does not judge); lifted off the cylinder by `1e-3` it
    /// refuses on that surface's residual; one surface named twice is
    /// no pair; and a spline carrier, whose between-sample bound only the
    /// rung-3 lane states, refuses typed rather than certify on nine
    /// samples.
    #[test]
    fn on_pair_meters_incidence_on_both_surfaces() {
        let (keys, lookup) = table(vec![
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            },
            Surface::Cylinder {
                origin: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::unit_y(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
        ]);
        let pair = SurfacePair::new(keys[0], keys[1]);
        let (p0, p1) = (Point3::origin(), Point3::new(0.0, 2.0, 0.0));
        let on = |p0, p1| EdgeCurveSpec {
            description: EdgeDescriptionSpec::on_pair(pair),
            ..line_spec(p0, p1)
        };
        let certified = EdgeCurve::certify(
            on(p0, p1),
            p0,
            p1,
            &lookup,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .expect("a generator on both surfaces certifies as their pair");
        assert!(matches!(
            certified.description(),
            EdgeDescription::OnPair { pair: got } if *got == pair
        ));
        let lift = Vec3::new(0.0, 0.0, -1e-3);
        let (q0, q1) = (p0 + lift, p1 + lift);
        assert!(
            matches!(
                EdgeCurve::certify(
                    on(q0, q1),
                    q0,
                    q1,
                    &lookup,
                    1.0,
                    band(),
                    NurbsLane::certified()
                )
                .unwrap_err(),
                CertifyError::ResidualExceeded {
                    check: CertCheck::SurfaceResidual { .. },
                    ..
                }
            ),
            "a carrier off a surface of the pair refuses on its residual"
        );
        let mut twice = on(p0, p1);
        twice.description = EdgeDescriptionSpec::on_pair(SurfacePair::new(keys[0], keys[0]));
        assert_eq!(
            EdgeCurve::certify(twice, p0, p1, &lookup, 1.0, band(), NurbsLane::certified())
                .unwrap_err(),
            CertifyError::IntersectionSameSurface { key: keys[0] }
        );
        let mut spline = on(p0, p1);
        spline.carrier = Curve3::nurbs_placeholder();
        assert_eq!(
            EdgeCurve::certify(spline, p0, p1, &lookup, 1.0, band(), NurbsLane::certified())
                .unwrap_err(),
            CertifyError::Unimplemented,
            "a spline carrier on a pair refuses typed: no between-sample bound"
        );
    }

    #[test]
    fn nurbs_carrier_is_refused_and_nothing_panics_on_poison() {
        let p0 = Point3::origin();
        let p1 = Point3::new(1.0, 0.0, 0.0);
        let mut spec = line_spec(p0, p1);
        spec.carrier = Curve3::nurbs_placeholder();
        assert_eq!(
            EdgeCurve::certify(
                spec.clone(),
                p0,
                p1,
                |_| None,
                1.0,
                band(),
                NurbsLane::certified()
            )
            .unwrap_err(),
            CertifyError::Unimplemented
        );
        // Poisoned endpoints: typed escalation, no panic (totality).
        let spec = line_spec(p0, p1);
        let nan = Point3::new(f64::NAN, 0.0, 0.0);
        let err = EdgeCurve::certify(
            spec.clone(),
            nan,
            p1,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap_err();
        assert!(matches!(err, CertifyError::Escalated { .. }));
    }

    /// A full-period circle carrier (the scaffolding self-loop case):
    /// coincident endpoints, params (0, τ) — certifiable because the
    /// interval is a stored certified cache (module docs).
    #[test]
    fn full_period_circle_certifies() {
        use core::f64::consts::TAU;
        let center = Point3::new(1.0, 2.0, 3.0);
        let p = Point3::new(2.0, 2.0, 3.0); // center + u_ref·r
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Scaffold(MappedCurve::whole(
                MappedSource::RevolvedPoint {
                    point: Point2::new(2.0, 2.0),
                    place: Affine3::translation(Vec3::new(0.0, 0.0, 3.0)),
                    axis_origin: center,
                    axis_dir: Vec3::unit_z(),
                    angle: TAU,
                },
            )),
            carrier: Curve3::Circle {
                center,
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: TAU,
        };
        EdgeCurve::certify(
            spec.clone(),
            p,
            p,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();
    }

    /// A placed-arc mapped curve against a circle carrier: the
    /// counterclockwise quarter of the unit circle, certified against
    /// its Circle3.
    #[test]
    fn placed_arc_certifies_against_circle_carrier() {
        use core::f64::consts::FRAC_PI_2;
        let place = Affine3::translation(Vec3::new(0.0, 0.0, 1.0));
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Scaffold(MappedCurve::whole(
                MappedSource::PlacedSegment {
                    segment: SketchSegment::Arc {
                        a: Point2::new(1.0, 0.0),
                        b: Point2::new(0.0, 1.0),
                        arc: Arc2 {
                            centre: Point2::new(0.0, 0.0),
                            radius: 1.0,
                            sweep: FRAC_PI_2,
                        },
                    },
                    place,
                },
            )),
            carrier: Curve3::Circle {
                center: Point3::new(0.0, 0.0, 1.0),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            param_start: 0.0,
            param_end: FRAC_PI_2,
        };
        let p0 = Point3::new(1.0, 0.0, 1.0);
        let p1 = Point3::new(0.0, 1.0, 1.0);
        EdgeCurve::certify(
            spec.clone(),
            p0,
            p1,
            |_| None,
            1.0,
            band(),
            NurbsLane::certified(),
        )
        .unwrap();
    }

    /// D4 ¶1 (i)/(iv), at every reading: the band-decided arms of one
    /// sized decision — its undecided margin and its Zero verdict — end
    /// in one ending, each quoting the tolerance its own margin gives;
    /// its sign-certain arm shares the lever at a build and ends in the
    /// file's defect ending over stored geometry, at rest.
    /// The pairs: `NotTransverse` with the transversality check's
    /// escalation, the lane's `NotTransverse` with its per-sample
    /// escalation, `NotSecondOrderSeparated` with the second-order and
    /// tube checks'.
    #[test]
    fn each_two_tolerance_pair_ends_in_one_recourse() {
        use crate::edge_nurbs::PlaneNurbsRefusal as P;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let cause = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band,
            predicate: Some("a_probe"),
            terminal_sliver: false,
        };
        let zero = Refused::Zero(Classified {
            margin: MarginDiag::value(5e-10),
            band,
        });
        let cross = "Recourse: move the geometry so the surfaces cross at a clearer angle";
        let curve = "Recourse: move the geometry so the faces curve apart more clearly where \
                     they touch";
        let escalated = |check| CertifyError::Escalated {
            check,
            sample: 1,
            cause,
        };
        for reading in [Reading::Build, Reading::AtRest] {
            let end = |e: CertifyError| e.ending(reading).unwrap();
            let lane = |p: P| p.ending(reading).unwrap();
            let pairs = [
                (
                    CertCheck::Transversality,
                    end(CertifyError::NotTransverse {
                        lever: None,
                        sample: 1,
                        verdict: zero,
                    }),
                    end(escalated(CertCheck::Transversality)),
                    cross,
                    "angle",
                ),
                (
                    CertCheck::Transversality,
                    lane(P::NotTransverse {
                        sample: 1,
                        lever: crate::ssi::PointLever::Extent,
                        verdict: zero,
                    }),
                    lane(P::TransversalityEscalated { sample: 1, cause }),
                    cross,
                    "angle",
                ),
                (
                    CertCheck::TangentSecondOrder,
                    end(CertifyError::NotSecondOrderSeparated {
                        sample: 1,
                        verdict: zero,
                    }),
                    end(escalated(CertCheck::TangentSecondOrder)),
                    curve,
                    "curvature difference",
                ),
                (
                    CertCheck::TangentTube,
                    end(CertifyError::TubeNotSeparated { verdict: zero }),
                    end(escalated(CertCheck::TangentTube)),
                    curve,
                    "curvature difference",
                ),
                (
                    CertCheck::TransversalityTube,
                    lane(P::TubeStraddles {
                        verdict: zero,
                        boxes: 4,
                    }),
                    end(escalated(CertCheck::TransversalityTube)),
                    cross,
                    "angle",
                ),
            ];
            for (check, at_zero, in_band, lever, size) in pairs {
                let offer = |below: &str| {
                    format!(
                        "{lever}, or, if this {size} is intended, tighten the tolerance below \
                         {below} m"
                    )
                };
                assert_eq!(at_zero, offer("5e-11"), "{reading:?}");
                assert_eq!(in_band, offer("5e-10"), "{reading:?}");
                let definite = recourse(check, RefusedArm::SignCertain(None), reading);
                match (reading, check) {
                    // A tube's margin is a lower bound: its definite
                    // refusal is the certificate's limit, not a stored
                    // contradiction.
                    (Reading::Build, _)
                    | (_, CertCheck::TangentTube | CertCheck::TransversalityTube) => {
                        assert_eq!(definite, lever, "{reading:?} {check:?}");
                    }
                    (Reading::AtRest, _) => {
                        assert_eq!(definite, KERNEL_OR_FILE_DEFECT_ENDING, "{reading:?}");
                    }
                }
                for msg in [&at_zero, &in_band, &definite] {
                    assert!(msg.matches("Recourse:").count() <= 1, "{msg}");
                    assert!(!msg.contains("declare"), "{msg}");
                }
            }
        }
    }

    /// The tangent tube's construction site carries the verdict its
    /// margin classified, with the reporting margin: a sagitta inside the
    /// zero band (positive, and exactly zero) is `Zero`, a drift past the
    /// sampled separation is `Negative`, a clear margin passes, at `f64`
    /// and at `Interval` (a point enclosure there).
    #[test]
    fn the_tangent_tube_site_carries_its_verdict() {
        use geom_core::Interval;
        use geom_core::interval::certification::Certification;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let point = Interval::point;
        let zero = |m| Refused::Zero(Classified { margin: m, band });
        let rows = [
            (
                (1e-9, 0.0),
                Some((
                    zero(MarginDiag::value(5e-10)),
                    zero(MarginDiag::enclosure(5e-10, 5e-10)),
                )),
                "5e-10 m: in the zero band",
            ),
            (
                (1e-9, 1e-9),
                Some((
                    zero(MarginDiag::value(0.0)),
                    zero(MarginDiag::enclosure(0.0, 0.0)),
                )),
                "exactly zero",
            ),
            (
                (0.0, 1.0),
                Some((
                    Refused::Negative {
                        margin: MarginDiag::value(-0.5),
                    },
                    Refused::Negative {
                        margin: MarginDiag::enclosure(-0.5, -0.5),
                    },
                )),
                "the drift swamps κ_min",
            ),
            ((1.0, 0.0), None, "a clear margin passes"),
        ];
        // Sagitta `(κ_min − drift)·1²/2`.
        for ((kappa_min, drift), want, why) in rows {
            let refused = |verdict| Err(CertifyError::TubeNotSeparated { verdict });
            let (at_f64, at_interval) = match want {
                Some((f, i)) => (refused(f), refused(i)),
                None => (Ok(()), Ok(())),
            };
            assert_eq!(
                tube_separation(kappa_min, drift, 1.0, band),
                at_f64,
                "f64: {why}"
            );
            assert_eq!(
                tube_separation(point(kappa_min), point(drift), point(1.0), band),
                at_interval,
                "Interval: {why}"
            );
        }
    }

    /// Each arm of a split verdict ends as its own arm, at every
    /// reading. A span a smaller tolerance decides forward quotes it; a
    /// span of no length or reversed, in band or past it, and the winding
    /// bound's refusal, are sides no construction mints: built by hand,
    /// they end in the lever at a build and over stored geometry in the
    /// defect ending (D4 ¶1 (iv)); an enclosure across zero, which shows neither a size nor its
    /// absence, ends in the lever alone. The tangent tube's Zero is
    /// band-decided and its Negative the
    /// certificate's limit, the lever alone everywhere; the plane × NURBS
    /// tube's Zero quotes `m/K` where its clearance is positive, and a
    /// zero or negative transversality there ends as the transversality
    /// decision's refused side does. No Negative arm offers a tolerance.
    #[test]
    fn each_split_verdict_ends_as_its_own_arm() {
        use crate::edge_nurbs::PlaneNurbsRefusal as P;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let span = "Recourse: move the geometry so this edge runs forward over a clear length";
        let cross = "Recourse: move the geometry so the surfaces cross at a clearer angle";
        let curve = "Recourse: move the geometry so the faces curve apart more clearly where \
                     they touch";
        let tube = |verdict| CertifyError::PlaneNurbs(P::TubeStraddles { verdict, boxes: 4 });
        let separated = |verdict| CertifyError::TubeNotSeparated { verdict };
        let forward = |verdict| CertifyError::IntervalNotForward { verdict };
        let zero = |m| {
            Refused::Zero(Classified {
                margin: MarginDiag::value(m),
                band,
            })
        };
        let negative = |m| Refused::Negative {
            margin: MarginDiag::value(m),
        };
        for reading in [Reading::Build, Reading::AtRest] {
            let tightened = |lever: &str, size: &str, below: &str| {
                format!(
                    "{lever}, or, if this {size} is intended, tighten the tolerance below {below} m"
                )
            };
            let lever = |lever: &str| Some(lever.to_owned());
            // A side no construction mints, built by hand.
            let unminted = |at_build: &str| match reading {
                Reading::Build => lever(at_build),
                Reading::AtRest => Some(KERNEL_OR_FILE_DEFECT_ENDING.to_owned()),
            };
            let winding = "Recourse: move the geometry so this arc stays clearly short of a full \
                           turn";
            let rows = [
                (
                    forward(zero(5e-10)),
                    Some(tightened(span, "length", "5e-11")),
                ),
                (forward(zero(0.0)), unminted(span)),
                (forward(zero(-5e-10)), unminted(span)),
                // An enclosure across zero shows no size, nor that there
                // is none: the lever alone, its payload printing it.
                (
                    CertifyError::IntervalNotForward {
                        verdict: Refused::Zero(Classified {
                            margin: MarginDiag::enclosure(-2e-10, 3e-10),
                            band,
                        }),
                    },
                    lever(span),
                ),
                (forward(negative(-1e-3)), unminted(span)),
                (CertifyError::WindingExceeded, unminted(winding)),
                (
                    separated(zero(5e-10)),
                    Some(tightened(curve, "curvature difference", "5e-11")),
                ),
                (separated(zero(0.0)), lever(curve)),
                (separated(negative(-0.5)), lever(curve)),
                // The tube's clearance is a lower bound: `0` is a straddle,
                // which keeps the lever at every door.
                (tube(zero(5e-10)), Some(tightened(cross, "angle", "5e-11"))),
                (tube(zero(0.0)), lever(cross)),
                (tube(negative(-5e-9)), lever(cross)),
            ];
            for (error, want) in rows {
                let got = error.ending(reading).unwrap();
                assert_eq!(Some(&got), want.as_ref(), "{reading:?}: {error:?}");
                if let Some((_, RefusedArm::SignCertain(_))) = error.decision() {
                    assert!(
                        !got.contains("tighten"),
                        "a Negative arm offers a tolerance: {got}"
                    );
                }
            }
        }
    }

    /// **A tube straddle never ends as a defect.** The lane's tube
    /// clearance is a lower bound that reads `0` wherever an enclosure
    /// straddles, so its refusal ends in the lever at a build, at rest and
    /// at the import door (where a zero lies within every ε_in, so the
    /// door's sentence leads), on every route, and never in a dead end.
    #[test]
    fn a_tube_straddle_keeps_the_lever_at_every_door() {
        use crate::edge_nurbs::PlaneNurbsRefusal as P;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let straddle = Refused::Zero(Classified {
            margin: MarginDiag::value(0.0),
            band,
        });
        let lever = "Recourse: move the geometry so the surfaces cross at a clearer angle";
        for at in [
            ReadAt::Run(Reading::Build),
            ReadAt::Run(Reading::AtRest),
            ReadAt::File(FileCoincidence::new(1e-12)),
        ] {
            let lane = P::TubeStraddles {
                verdict: straddle,
                boxes: 4,
            };
            let edge = CertifyError::PlaneNurbs(lane);
            let analytic = crate::edge_nurbs::AnalyticRung3Refusal::TubeStraddles {
                verdict: straddle,
                boxes: 4,
            };
            let ssi = crate::ssi::SsiError::TubeStraddles {
                verdict: straddle,
                boxes: 4,
            };
            let mut endings = vec![lane.ending(at), edge.ending(at), analytic.ending(at)];
            if let ReadAt::Run(reading) = at {
                endings.push(Some(ssi.ending(reading)));
            }
            for got in endings {
                let got = got.unwrap();
                assert!(
                    got.ends_with(lever) && !got.contains("There is no way through"),
                    "{at:?}: {got}"
                );
            }
        }
    }

    /// **A proved-zero span ends by its unminted side**: a zero the
    /// symbolic tier proved, its rounded number outside the zero band,
    /// ends in the defect ending: at rest naming the kernel or the file,
    /// and at the import door past ε_in the file.
    #[test]
    fn a_proved_zero_span_ends_by_its_side_at_rest_and_at_the_import_door() {
        use geom_core::sym::with_session;
        use geom_core::{Margin, ParamSymbol, Sym, SymBudget};

        let band = Band::new(1e-9, 1e-8).unwrap();
        // `x·3/3 − x` at this `x` rounds to 3.7e-9 m; the symbolic tier
        // proves it zero.
        let (proved, _) = with_session(
            SymBudget {
                max_terms: 64,
                max_degree: 8,
            },
            || {
                let x = Sym::param(ParamSymbol::new(0xa8), 23_456_789.1_f64);
                let three = Sym::from_f64(3.0);
                crate::dihedral::decide_positive("pin", Margin::of(x * three / three - x), band)
                    .unwrap_err()
            },
        );
        let verdict = Refused::rejected(&proved).unwrap();
        assert!(
            matches!(verdict, Refused::Zero(Classified { margin, .. }) if margin.proven()),
            "the fixture is a proved zero: {verdict:?}"
        );
        let span = CertifyError::IntervalNotForward { verdict };
        assert_eq!(
            span.ending(Reading::AtRest).as_deref(),
            Some(KERNEL_OR_FILE_DEFECT_ENDING),
            "at rest"
        );
        assert_eq!(
            span.ending(FileCoincidence::new(1e-12)).as_deref(),
            Some(DAMAGED_FILE_ENDING),
            "at the import door past ε_in"
        );
    }

    /// **A refusal on the span checks' unminted side, built by hand,
    /// renders at every door and ends as the door's own refusal of that
    /// reading does** (`[failure]`): a public render of a value a caller
    /// built is input, so it never panics. Each of the hand-built
    /// refusals, and `recourse` on its check's arm, ends as the
    /// [`CertifyError::StatedInterval`] a public door returns for the
    /// same reading: the check's lever at a build, the kernel's or the
    /// file's defect at rest, and the file's at the import door.
    #[test]
    fn a_hand_built_unminted_refusal_renders_at_every_door() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let in_band = Indeterminate {
            margin: MarginDiag::value(-5e-9),
            band,
            predicate: None,
            terminal_sliver: false,
        };
        let reversed = Refused::Negative {
            margin: MarginDiag::value(-1e-3),
        };
        let escalated = |check| CertifyError::Escalated {
            check,
            sample: 0,
            cause: in_band,
        };
        let rows = [
            (
                CertifyError::IntervalNotForward { verdict: reversed },
                StatedSpan::Forward(ForwardReading::Decided(reversed)),
            ),
            (
                escalated(CertCheck::ParamSpan),
                StatedSpan::Forward(ForwardReading::InBand(in_band)),
            ),
            (
                CertifyError::WindingExceeded,
                StatedSpan::Winding(WindingReading::Exceeded),
            ),
            (
                escalated(CertCheck::ParamWinding),
                StatedSpan::Winding(WindingReading::InBand(in_band)),
            ),
        ];
        let doors = [
            (ReadAt::Run(Reading::Build), None),
            (
                ReadAt::Run(Reading::AtRest),
                Some(KERNEL_OR_FILE_DEFECT_ENDING),
            ),
            (
                ReadAt::File(FileCoincidence::new(1e-12)),
                Some(DAMAGED_FILE_ENDING),
            ),
        ];
        for (error, refused) in rows {
            let lever = format!("Recourse: {}", refused.decision().lever());
            let stated = CertifyError::StatedInterval { refused };
            for (at, defect) in doors {
                let want = defect.map_or_else(|| lever.clone(), str::to_owned);
                assert_eq!(error.ending(at), Some(want.clone()), "{error:?} at {at:?}");
                assert_eq!(
                    error.ending(at),
                    stated.ending(at),
                    "{error:?} at {at:?} ends as the door's refusal"
                );
                assert_eq!(
                    error.render(at),
                    format!("{error}. {want}"),
                    "{error:?} at {at:?}"
                );
                assert_eq!(
                    recourse(refused.check(), refused.arm(), at),
                    want,
                    "recourse({:?}, {:?}) at {at:?}",
                    refused.check(),
                    refused.arm()
                );
            }
        }
        for (check, decision) in [
            (CertCheck::ParamSpan, SpanDecision::Forward),
            (CertCheck::ParamWinding, SpanDecision::Winding),
        ] {
            assert_eq!(
                recourse(check, RefusedArm::SignCertain(None), Reading::Build),
                format!("Recourse: {}", decision.lever()),
                "{check:?}"
            );
        }
    }

    /// **A stored arc past a full turn at a caller's finer band ends
    /// typed** (`[failure]`): the winding decision passes a headroom its
    /// band reads as zero, so an arc certified at a coarse band can lie
    /// past a turn by up to it, and a public re-certification at a finer
    /// band, the caller's, reads it past one — past the band, or in band
    /// wholly below zero. Each refuses as the door's input, ending in the
    /// winding lever at a build and in the damaged file's ending at the
    /// import door past ε_in, and neither panics; every re-certification
    /// door, the one the tier-3 validator reads through among them,
    /// refuses it alike.
    #[test]
    fn a_stored_arc_past_a_turn_at_a_callers_finer_band_ends_typed() {
        let circle = Curve3::Circle {
            center: Point3::origin(),
            axis: geom_core::Vec3::unit_z(),
            radius: 1.0,
            u_ref: geom_core::Vec3::unit_x(),
        };
        let t1 = core::f64::consts::TAU + 5e-7;
        let spec = EdgeCurveSpec::arc_of_circle(circle, 0.0, t1).unwrap();
        let (start, end) = (spec.carrier.eval(0.0), spec.carrier.eval(t1));
        let coarse = Band::new(1e-6, 1e-5).unwrap();
        let minted = EdgeCurve::certify(
            spec,
            start,
            end,
            |_| None,
            1.0,
            coarse,
            NurbsLane::certified(),
        )
        .expect("a headroom the coarse band reads as zero passes");
        let lever = "Recourse: move the geometry so this arc stays clearly short of a full turn";
        for (finer, in_band) in [
            (Band::new(1e-9, 1e-8).unwrap(), false),
            (Band::new(1e-7, 1e-6).unwrap(), true),
        ] {
            let error = minted
                .recertify(start, end, |_| None, 1.0, finer, NurbsLane::certified())
                .unwrap_err();
            let CertifyError::StatedInterval {
                refused: StatedSpan::Winding(reading),
            } = error
            else {
                panic!("{finer:?}: {error:?}");
            };
            assert_eq!(
                matches!(reading, WindingReading::InBand(_)),
                in_band,
                "{finer:?}: {error:?}"
            );
            let built = error.render(Reading::Build);
            assert!(built.ends_with(lever), "{finer:?}: {built}");
            assert_eq!(
                error.ending(FileCoincidence::new(1e-12)).as_deref(),
                Some(DAMAGED_FILE_ENDING),
                "{finer:?} at the import door"
            );
            let kernels = minted
                .recertify_stated(
                    start,
                    end,
                    |_| None,
                    1.0,
                    finer,
                    NurbsLane::certified(),
                    None,
                )
                .unwrap_err();
            assert_eq!(
                kernels, error,
                "{finer:?}: the validator's door refuses it alike, typed"
            );
        }
    }

    /// **The minting door refuses its caller's interval, and only a
    /// reading certainly on the refused side**, through each of its three
    /// spellings: a reversed line, an empty one and an arc past a full
    /// turn refuse as the caller's. At a build each ends in the check's
    /// lever; at rest in the defect ending; at the import door the
    /// empty line, a size within ε_in, in the door's sentence and the
    /// lever, and the reversed line and the arc, past it, in the damaged
    /// file's ending.
    /// A span a smaller tolerance decides forward stays the check's own,
    /// its offer intact. The reversed line's payload names how far it runs
    /// backwards, and renders whole at a build.
    #[test]
    fn the_minting_door_refuses_a_callers_interval() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let line = |t0: f64, t1: f64| EdgeCurveSpec {
            param_start: t0,
            param_end: t1,
            ..EdgeCurveSpec::line_between(Point3::origin(), Point3::new(1.0, 0.0, 0.0))
        };
        fn ends(spec: &EdgeCurveSpec<f64>) -> (Point3<f64>, Point3<f64>) {
            (
                spec.carrier.eval(spec.param_start),
                spec.carrier.eval(spec.param_end),
            )
        }
        type Door = fn(EdgeCurveSpec<f64>, Band) -> Result<(), CertifyError>;
        let doors: [(&str, Door); 3] = [
            ("certify", |spec, band| {
                let (start, end) = ends(&spec);
                EdgeCurve::certify(
                    spec,
                    start,
                    end,
                    |_| None,
                    1.0,
                    band,
                    NurbsLane::certified(),
                )
                .map(|_| ())
            }),
            ("certify_stated", |spec, band| {
                let (start, end) = ends(&spec);
                let lane = NurbsLane::certified();
                EdgeCurve::certify_stated(spec, start, end, |_| None, 1.0, band, lane, None)
                    .map(|_| ())
            }),
            ("mint", |spec, band| {
                EdgeCurve::mint(spec, |_| None, band).map(|_| ())
            }),
        ];
        let circle = Curve3::Circle {
            center: Point3::origin(),
            axis: geom_core::Vec3::unit_z(),
            radius: 1.0,
            u_ref: geom_core::Vec3::unit_x(),
        };
        let past = EdgeCurveSpec::arc_of_circle(circle, 0.0, 3.0 * core::f64::consts::PI).unwrap();
        let file = FileCoincidence::new(1e-12);
        let span = "Recourse: move the geometry so this edge runs forward over a clear length";
        let winding = "Recourse: move the geometry so this arc stays clearly short of a full turn";
        let untold = "This length is within the file's declared coincidence distance ε_in = 1e-12 \
                      m, so the file cannot tell it from zero.";
        // (row, spec, refused forward, its ending at the import door)
        let rows = [
            (
                "reversed",
                line(1.0, 0.0),
                true,
                DAMAGED_FILE_ENDING.to_owned(),
            ),
            ("empty", line(0.5, 0.5), true, format!("{untold} {span}")),
            ("past a turn", past, false, DAMAGED_FILE_ENDING.to_owned()),
        ];
        for (door, mint) in doors {
            for (row, spec, forward, at_the_file) in rows.clone() {
                let error = mint(spec, band).unwrap_err();
                let lever = if forward { span } else { winding };
                let CertifyError::StatedInterval { refused } = error else {
                    panic!("{door}, {row}: {error:?}");
                };
                assert_eq!(
                    refused.decision() == SpanDecision::Forward,
                    forward,
                    "{door}, {row}: {refused:?}"
                );
                assert_eq!(
                    refused.decision().lever(),
                    lever.trim_start_matches("Recourse: ")
                );
                for (at, want) in [
                    (ReadAt::Run(Reading::Build), lever.to_owned()),
                    (
                        ReadAt::Run(Reading::AtRest),
                        KERNEL_OR_FILE_DEFECT_ENDING.to_owned(),
                    ),
                    (ReadAt::File(file), at_the_file),
                ] {
                    assert_eq!(error.ending(at), Some(want), "{door}, {row} at {at:?}");
                }
            }
            let short = mint(line(0.5, 0.5 + 5e-9), band).unwrap_err();
            assert!(
                matches!(
                    short,
                    CertifyError::Escalated {
                        check: CertCheck::ParamSpan,
                        ..
                    }
                ),
                "{door}: {short:?}"
            );
            assert!(
                short
                    .render(Reading::Build)
                    .contains("tighten the tolerance below"),
                "{door}: {short:?}"
            );
        }
        let reversed = doors[0].1(line(1.0, 0.0), band).unwrap_err();
        assert!(
            matches!(
                reversed,
                CertifyError::StatedInterval {
                    refused: StatedSpan::Forward(ForwardReading::Decided(Refused::Negative { .. })),
                }
            ),
            "{reversed:?}"
        );
        assert_eq!(
            reversed.render(Reading::Build),
            format!("this edge's parameter interval runs backwards by 1 m. {span}"),
        );
    }

    /// **No public door panics on a span it refuses**: each minting door
    /// handed a reversed line, one reversed by less than the band, a chord
    /// between coincident points, an arc past a full turn, or one past a
    /// turn by less than a coarse band but more than its own finer one,
    /// and each re-certification door
    /// handed that last arc minted at the coarse band and read at a finer
    /// band (past it, or in band), refuses it as its input
    /// ([`CertifyError::StatedInterval`]), and the refusal renders at a
    /// build, at rest and at the import door without panicking.
    #[test]
    fn no_public_door_panics_on_a_span_it_refuses() {
        let fine = Band::new(1e-9, 1e-8).unwrap();
        let coarse = Band::new(1e-6, 1e-5).unwrap();
        let circle = Curve3::Circle {
            center: Point3::origin(),
            axis: geom_core::Vec3::unit_z(),
            radius: 1.0,
            u_ref: geom_core::Vec3::unit_x(),
        };
        let arc = |t1: f64| EdgeCurveSpec::arc_of_circle(circle.clone(), 0.0, t1).unwrap();
        let p = Point3::new(1.0, 2.0, 3.0);
        let slightly_past = core::f64::consts::TAU + 5e-7;
        let specs = [
            (
                "reversed",
                EdgeCurveSpec {
                    param_start: 1.0,
                    param_end: 0.0,
                    ..EdgeCurveSpec::line_between(Point3::origin(), Point3::new(1.0, 0.0, 0.0))
                },
            ),
            (
                "reversed within the band",
                EdgeCurveSpec {
                    param_start: 0.5,
                    param_end: 0.5 - 5e-9,
                    ..EdgeCurveSpec::line_between(Point3::origin(), Point3::new(1.0, 0.0, 0.0))
                },
            ),
            ("coincident", EdgeCurveSpec::line_between(p, p)),
            ("past a turn", arc(3.0 * core::f64::consts::PI)),
            ("past a turn within a coarse band", arc(slightly_past)),
        ];
        let certify = |spec: EdgeCurveSpec<f64>, band| {
            let (a, b) = (
                spec.carrier.eval(spec.param_start),
                spec.carrier.eval(spec.param_end),
            );
            EdgeCurve::certify(spec, a, b, |_| None, 1.0, band, NurbsLane::certified())
        };
        type Mint = fn(EdgeCurveSpec<f64>, Band) -> Result<(), CertifyError>;
        let mints: [(&str, Mint); 3] = [
            ("certify", |spec, band| {
                let (a, b) = (
                    spec.carrier.eval(spec.param_start),
                    spec.carrier.eval(spec.param_end),
                );
                EdgeCurve::certify(spec, a, b, |_| None, 1.0, band, NurbsLane::certified())
                    .map(|_| ())
            }),
            ("certify_stated", |spec, band| {
                let (a, b) = (
                    spec.carrier.eval(spec.param_start),
                    spec.carrier.eval(spec.param_end),
                );
                let lane = NurbsLane::certified();
                EdgeCurve::certify_stated(spec, a, b, |_| None, 1.0, band, lane, None).map(|_| ())
            }),
            ("mint", |spec, band| {
                EdgeCurve::mint(spec, |_| None, band).map(|_| ())
            }),
        ];
        type Recertify = fn(&EdgeCurve<f64>, Band) -> Result<Certificate<f64>, CertifyError>;
        let recertifiers: [(&str, Recertify); 2] = [
            ("recertify", |curve, band| {
                let (a, b) = (
                    curve.carrier.eval(curve.param_start),
                    curve.carrier.eval(curve.param_end),
                );
                curve.recertify(a, b, |_| None, 1.0, band, NurbsLane::certified())
            }),
            ("recertify_stated", |curve, band| {
                let (a, b) = (
                    curve.carrier.eval(curve.param_start),
                    curve.carrier.eval(curve.param_end),
                );
                curve.recertify_stated(a, b, |_| None, 1.0, band, NurbsLane::certified(), None)
            }),
        ];
        let mut refusals = Vec::new();
        for (door, mint) in mints {
            for (row, spec) in specs.clone() {
                refusals.push((format!("{door}, {row}"), mint(spec, fine).map(|_| ())));
            }
        }
        let minted = certify(arc(slightly_past), coarse)
            .expect("a headroom the coarse band reads as zero passes");
        for (door, recertify) in recertifiers {
            for band in [fine, Band::new(1e-7, 1e-6).unwrap()] {
                refusals.push((
                    format!("{door} at {band:?}"),
                    recertify(&minted, band).map(|_| ()),
                ));
            }
        }
        assert_eq!(refusals.len(), 19, "every door and row");
        let mut forward_in_band = 0;
        for (row, refusal) in refusals {
            let error = match refusal {
                Err(error @ CertifyError::StatedInterval { .. }) => error,
                other => panic!("{row}: {other:?}"),
            };
            if let CertifyError::StatedInterval {
                refused: StatedSpan::Forward(ForwardReading::InBand(_)),
            } = error
            {
                forward_in_band += 1;
            }
            for at in [
                ReadAt::Run(Reading::Build),
                ReadAt::Run(Reading::AtRest),
                ReadAt::File(FileCoincidence::new(1e-12)),
            ] {
                assert_eq!(
                    test_utils::panic_capture::caught(|| {
                        let _ = error.render(at);
                    }),
                    None,
                    "{row}: {error:?} at {at:?}"
                );
            }
        }
        assert_eq!(
            forward_in_band, 3,
            "each minting door refuses the reversed line within the band in band"
        );
    }

    /// **A coincident-points edge and a sibling whose length the run's
    /// band holds tell one story at the import door**: the door's
    /// sentence that the file cannot tell the length from zero, and the
    /// span lever, whether the door refused the span as its caller's (no
    /// length) or the check kept it (a length a smaller tolerance
    /// decides). A sibling past the band certifies, and is not this row's.
    #[test]
    fn a_coincident_edge_and_an_in_band_sibling_tell_one_story_in_a_file() {
        let band = Band::new(1e-12, 1e-11).unwrap();
        let file = FileCoincidence::new(1e-9);
        let p = Point3::new(1.0, 2.0, 3.0);
        let refused = |q: Point3<f64>| {
            EdgeCurve::certify(
                EdgeCurveSpec::line_between(p, q),
                p,
                q,
                |_| None,
                1.0,
                band,
                NurbsLane::certified(),
            )
            .unwrap_err()
        };
        let coincident = refused(p);
        let sibling = refused(Point3::new(1.0 + 5e-12, 2.0, 3.0));
        assert!(
            matches!(coincident, CertifyError::StatedInterval { .. }),
            "{coincident:?}"
        );
        assert!(
            !matches!(sibling, CertifyError::StatedInterval { .. }),
            "{sibling:?}"
        );
        let untold = "This length is within the file's declared coincidence distance ε_in = 1e-9 \
                      m, so the file cannot tell it from zero. Recourse: move the geometry so \
                      this edge runs forward over a clear length";
        for error in [coincident, sibling] {
            let ending = error.ending(file).unwrap();
            assert!(ending.starts_with(untold), "{error:?}: {ending}");
        }
    }

    /// A span of no length and a reversed one each render whole under
    /// the one lever, which names the forward edge both lack, at the
    /// minting door that refuses them as its caller's, and in the defect
    /// ending over stored geometry.
    #[test]
    fn a_zero_and_a_reversed_span_render_whole() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let in_band = |m| CertifyError::IntervalNotForward {
            verdict: Refused::Zero(Classified {
                margin: MarginDiag::value(m),
                band,
            }),
        };
        let stated = |verdict| CertifyError::StatedInterval {
            refused: StatedSpan::Forward(ForwardReading::Decided(verdict)),
        };
        let past = |reading| CertifyError::StatedInterval {
            refused: StatedSpan::Winding(reading),
        };
        let rows = [
            (
                stated(Refused::Zero(Classified {
                    margin: MarginDiag::value(0.0),
                    band,
                })),
                Reading::Build,
                "this edge's parameter interval is not shown to run forward (length 0 m). Recourse: \
                 move the geometry so this edge runs forward over a clear length",
            ),
            (
                in_band(0.0),
                Reading::AtRest,
                "the stored parameter interval is not shown to run forward (length 0 m). There \
                 is no way through: this is a kernel defect or a damaged file; report it",
            ),
            (
                stated(Refused::Negative {
                    margin: MarginDiag::value(-1e-3),
                }),
                Reading::Build,
                "this edge's parameter interval runs backwards by 0.001 m. Recourse: move the \
                 geometry so this edge runs forward over a clear length",
            ),
            (
                in_band(-5e-10),
                Reading::AtRest,
                "the stored parameter interval is not shown to run forward (length -5e-10 m). \
                 There is no way through: this is a kernel defect or a damaged file; report it",
            ),
            (
                CertifyError::StatedInterval {
                    refused: StatedSpan::Forward(ForwardReading::InBand(Indeterminate {
                        margin: MarginDiag::value(-5e-10),
                        band,
                        predicate: Some("interval_span_forward"),
                        terminal_sliver: false,
                    })),
                },
                Reading::Build,
                "this edge's parameter interval is not shown to run forward (length -5e-10 m). \
                 Recourse: move the geometry so this edge runs forward over a clear length",
            ),
            (
                past(WindingReading::Exceeded),
                Reading::Build,
                "this edge's parameter interval spans more than one full turn of its carrier. \
                 Recourse: move the geometry so this arc stays clearly short of a full turn",
            ),
            (
                past(WindingReading::InBand(Indeterminate {
                    margin: MarginDiag::value(-5e-9),
                    band,
                    predicate: Some("interval_span_winding"),
                    terminal_sliver: false,
                })),
                Reading::Build,
                "this edge's parameter interval is not shown to stay within one full turn of its \
                 carrier (headroom -5e-9 m). Recourse: move the geometry so this arc stays clearly \
                 short of a full turn",
            ),
        ];
        let wrong: Vec<String> = rows
            .iter()
            .filter_map(|(error, reading, want)| {
                let got = error.render(*reading);
                (got != *want).then(|| format!("{reading:?} {error:?}:\n got  {got}\n want {want}"))
            })
            .collect();
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }

    /// The residual families pair their definite and undecided arms per
    /// reading: an exact construction's (an unavailable chart image and
    /// the chart pick's escalation) end in the defect ending everywhere;
    /// an approximation's (the lane's limb and the certificate's
    /// escalation) share the last resort at a build, and part at rest,
    /// where a definite stored contradiction is a defect no loosening
    /// repairs.
    #[test]
    fn each_residual_pair_ends_as_its_reading_gives() {
        use crate::edge_nurbs::PlaneNurbsRefusal as P;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let cause = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band,
            predicate: Some("a_probe"),
            terminal_sliver: false,
        };
        let limb = P::Limb {
            limb: crate::ssi::SsiLimb::OnLocus,
            margin: MarginDiag::value(2e-8),
        };
        let unavailable = CertifyError::ChartImageUnavailable {
            chart: geom::SurfaceKind::Cone,
            carrier: geom::CurveKind::Ellipse,
        };
        let chart = CertifyError::Escalated {
            check: CertCheck::ChartImage,
            sample: 1,
            cause,
        };
        let rows = [
            (
                Reading::Build,
                KERNEL_LIMIT_RECOURSE,
                KERNEL_LIMIT_RECOURSE,
                KERNEL_DEFECT_ENDING,
            ),
            (
                Reading::AtRest,
                KERNEL_OR_FILE_DEFECT_ENDING,
                KERNEL_LIMIT_RECOURSE,
                KERNEL_OR_FILE_DEFECT_ENDING,
            ),
        ];
        for (reading, definite, undecided, exact) in rows {
            assert_eq!(limb.ending(reading).unwrap(), definite, "{reading:?}");
            assert_eq!(
                P::Escalated {
                    limb: crate::ssi::SsiLimb::OnLocus,
                    cause
                }
                .ending(reading)
                .unwrap(),
                undecided,
                "{reading:?}"
            );
            assert_eq!(unavailable.ending(reading).unwrap(), exact, "{reading:?}");
            assert_eq!(chart.ending(reading).unwrap(), exact, "{reading:?}");
        }
    }

    /// **A certificate escalation ends by its limb's decision.** On the
    /// lane's refusal and through its map into certification alike:
    /// limbs 1 and 2 are residuals (the last resort at a build), limb
    /// 3's margin in band is the transversality's (its lever and the
    /// tolerance below `m/K`), and the poisoned reported transversality
    /// ends in the kernel-defect ending. None of them was decided at a
    /// schedule point, so certification renders none.
    #[test]
    fn a_certificate_escalation_ends_by_its_limbs_decision() {
        use crate::edge_nurbs::PlaneNurbsRefusal as P;
        use crate::ssi::SsiLimb;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let cause = |margin| Indeterminate {
            margin,
            band,
            predicate: Some("a_probe"),
            terminal_sliver: false,
        };
        let tube = "Recourse: move the geometry so the surfaces cross at a clearer angle, or, if \
                    this angle is intended, tighten the tolerance below 5e-10 m";
        for (refusal, check, want) in [
            (
                P::Escalated {
                    limb: SsiLimb::OnLocus,
                    cause: cause(MarginDiag::value(5e-9)),
                },
                CertCheck::PlaneNurbsOnLocus,
                KERNEL_LIMIT_RECOURSE,
            ),
            (
                P::Escalated {
                    limb: SsiLimb::HullSup,
                    cause: cause(MarginDiag::value(5e-9)),
                },
                CertCheck::PlaneNurbsHull,
                KERNEL_LIMIT_RECOURSE,
            ),
            (
                P::Escalated {
                    limb: SsiLimb::Tube,
                    cause: cause(MarginDiag::value(5e-9)),
                },
                CertCheck::TransversalityTube,
                tube,
            ),
            (
                P::ReportedTransversalityPoisoned(cause(MarginDiag::INVALID)),
                CertCheck::PlaneNurbsReportedTransversality,
                KERNEL_DEFECT_ENDING,
            ),
        ] {
            let (got_check, _) = refusal.decision().unwrap();
            assert_eq!(got_check, check, "{refusal:?}");
            assert_eq!(refusal.ending(Reading::Build).unwrap(), want, "{refusal:?}");
            // Through certification: the same check, and no schedule
            // point the limb never visited.
            let certified = from_plane_nurbs(refusal);
            assert_eq!(
                certified.decision().map(|(c, _)| c),
                Some(check),
                "{certified:?}"
            );
            let shown = certified.to_string();
            assert!(
                shown.contains("(not a sampled check)") && !shown.contains("at sample"),
                "a non-sampled limb renders no schedule point: {shown}"
            );
        }
        let exceeded = CertifyError::ResidualExceeded {
            check: CertCheck::TangentHull,
            sample: NOT_A_SAMPLE,
            margin: MarginDiag::value(3e-8),
        }
        .to_string();
        assert!(
            exceeded.contains("(not a sampled check)") && !exceeded.contains("at sample"),
            "{exceeded}"
        );
    }

    /// Each decision family ends every refused arm the one way its
    /// decision gives, pinned whole:
    /// - a sized decision (the span, the winding headroom) names its own
    ///   lever, and the tolerance below `m/K` only where the margin lies
    ///   on the side the decision accepts; a margin at zero or on the
    ///   refused side shows no size and adds the decision's note where it
    ///   has one, whichever arm routed it; an enclosure touching or
    ///   straddling zero names the lever alone; a margin that could not be read keeps the lever
    ///   and says what it may mean;
    /// - an exact residual (the endpoints, the chart image) ends in the
    ///   kernel-defect ending, the file's too at rest, and its definite
    ///   refusal ends the same;
    /// - an approximation (a fitted carrier on its surface, the
    ///   plane × NURBS residual limbs) ends in the last resort, and the
    ///   plane × NURBS lane's poisoned reported transversality, which
    ///   no geometry reaches, in the kernel-defect ending.
    #[test]
    fn each_decision_family_ends_in_its_routed_sentence() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let undecided_at = |check, margin, reading| {
            let cause = Indeterminate {
                margin,
                band,
                predicate: Some("a_probe"),
                terminal_sliver: false,
            };
            recourse(check, RefusedArm::Undecided(&cause), reading)
        };
        let undecided = |check, margin| undecided_at(check, margin, Reading::Build);
        // A span reading on a side no construction mints, built by hand,
        // ends in its lever at a build, and is the kernel's or the file's
        // at rest.
        let unminted = [
            (CertCheck::ParamSpan, MarginDiag::value(-5e-9)),
            (CertCheck::ParamSpan, MarginDiag::value(0.0)),
            (CertCheck::ParamSpan, MarginDiag::value(-0.0)),
            (
                CertCheck::ParamWinding,
                MarginDiag::enclosure(-4.0e-9, -2.0e-9),
            ),
        ];
        for (check, margin) in unminted {
            let lever = match check {
                CertCheck::ParamSpan => {
                    "Recourse: move the geometry so this edge runs forward over a clear length"
                }
                _ => "Recourse: move the geometry so this arc stays clearly short of a full turn",
            };
            assert_eq!(
                undecided(check, margin),
                lever,
                "{check:?} {margin} at a build"
            );
            assert_eq!(
                undecided_at(check, margin, Reading::AtRest),
                KERNEL_OR_FILE_DEFECT_ENDING,
                "{check:?} {margin} at rest"
            );
        }
        let straddle = MarginDiag::enclosure(-2.0e-9, 4.0e-9);
        let inside = MarginDiag::enclosure(2.0e-9, 4.0e-9);
        let rows = [
            (
                undecided(CertCheck::ParamSpan, MarginDiag::value(5e-9)),
                "Recourse: move the geometry so this edge runs forward over a clear length, or, if \
                 this length is intended, tighten the tolerance below 5e-10 m",
            ),
            (
                undecided(CertCheck::ParamWinding, inside),
                "Recourse: move the geometry so this arc stays clearly short of a full turn, or, \
                 if this arc is intended, tighten the tolerance below 2e-10 m",
            ),
            (
                undecided(CertCheck::Transversality, straddle),
                "Recourse: move the geometry so the surfaces cross at a clearer angle",
            ),
            (
                undecided(CertCheck::Transversality, MarginDiag::INVALID),
                "Recourse: move the geometry so the surfaces cross at a clearer angle; an \
                 unreadable margin may indicate a kernel bug worth reporting",
            ),
            (
                undecided(CertCheck::EndpointStart, MarginDiag::value(5e-9)),
                KERNEL_DEFECT_ENDING,
            ),
            (
                undecided(CertCheck::ChartImage, straddle),
                KERNEL_DEFECT_ENDING,
            ),
            (
                undecided(
                    CertCheck::SurfaceResidual {
                        surface: SurfaceKey::default(),
                    },
                    MarginDiag::value(5e-9),
                ),
                KERNEL_LIMIT_RECOURSE,
            ),
            (
                undecided(CertCheck::PlaneNurbsHull, MarginDiag::value(5e-9)),
                KERNEL_LIMIT_RECOURSE,
            ),
            (
                undecided(
                    CertCheck::PlaneNurbsReportedTransversality,
                    MarginDiag::INVALID,
                ),
                KERNEL_DEFECT_ENDING,
            ),
            (
                // Touching zero, its sign unresolved: the lever alone.
                undecided(CertCheck::ParamWinding, MarginDiag::enclosure(0.0, 4.0e-9)),
                "Recourse: move the geometry so this arc stays clearly short of a full turn",
            ),
            (
                recourse(
                    CertCheck::EndpointStart,
                    RefusedArm::SignCertain(None),
                    Reading::AtRest,
                ),
                KERNEL_OR_FILE_DEFECT_ENDING,
            ),
            // At rest: a sized decision's sign-certain arm is a stored
            // contradiction (defect); its zero arm is band-decided
            // (lever and conditional); an approximation's definite
            // residual is a stored contradiction no loosening repairs.
            (
                recourse(
                    CertCheck::ParamSpan,
                    RefusedArm::SignCertain(None),
                    Reading::AtRest,
                ),
                KERNEL_OR_FILE_DEFECT_ENDING,
            ),
            // A span of no length has no size to tighten below; no
            // construction mints one, so over stored geometry it is the
            // kernel's or the file's.
            (
                CertifyError::IntervalNotForward {
                    verdict: Refused::Zero(Classified {
                        margin: MarginDiag::value(0.0),
                        band,
                    }),
                }
                .ending(Reading::AtRest)
                .unwrap(),
                KERNEL_OR_FILE_DEFECT_ENDING,
            ),
            (
                CertifyError::ResidualExceeded {
                    check: CertCheck::SurfaceResidual {
                        surface: SurfaceKey::default(),
                    },
                    sample: 0,
                    margin: MarginDiag::value(3e-8),
                }
                .ending(Reading::AtRest)
                .unwrap(),
                KERNEL_OR_FILE_DEFECT_ENDING,
            ),
        ];
        for (got, want) in rows {
            assert_eq!(got, want);
        }
        // A residual's definite refusal ends as its in-band arm does.
        let definite = CertifyError::ResidualExceeded {
            check: CertCheck::EndpointStart,
            sample: 0,
            margin: MarginDiag::value(3e-8),
        }
        .render(Reading::Build);
        assert!(definite.ends_with(KERNEL_DEFECT_ENDING), "{definite}");
        let escalated = CertifyError::Escalated {
            check: CertCheck::EndpointStart,
            sample: 0,
            cause: Indeterminate {
                margin: MarginDiag::value(5e-9),
                band,
                predicate: Some("a_probe"),
                terminal_sliver: false,
            },
        };
        let payload = "the start-endpoint residual at sample 0 escalated: margin 5e-9 lies \
                       inside the ambiguity band (1e-9, 1e-8)";
        // `Display` is the payload; the door's reading supplies the end.
        assert_eq!(escalated.to_string(), payload);
        assert_eq!(
            escalated.render(Reading::Build),
            format!("{payload}. There is no way through: this is a kernel defect; report it")
        );
    }

    /// **The floor's sentence is said once on the undecided route**: an
    /// escalation of the span whose positive size only a tolerance at or
    /// below `EPS_MIN` decides — a gate's rejected Zero, and an in-band
    /// reading at a run near the floor — renders its payload and its
    /// ending with the floor's sentence once, and neither the span's
    /// no-length note nor an offer, at a build and at rest. Red if the
    /// payload word says it again, or the note or an offer returns.
    #[test]
    fn the_floor_sentence_is_said_once_on_the_undecided_route() {
        use geom_core::k_stats::decide_positive;
        use geom_core::tolerance::EPS_MIN;
        use geom_core::{Margin, floor_note};

        let floor = floor_note("length");
        let band = Band::new(1e-9, 1e-8).unwrap();
        let rejected =
            decide_positive("interval_span_forward", Margin::of(2e-12), band).unwrap_err();
        let in_band = Indeterminate {
            margin: MarginDiag::value(3.0 * EPS_MIN),
            band: Band::new(2.0 * EPS_MIN, 20.0 * EPS_MIN).unwrap(),
            predicate: Some("interval_span_forward"),
            terminal_sliver: false,
        };
        assert_eq!(
            CertifyError::Escalated {
                check: CertCheck::ParamSpan,
                sample: 0,
                cause: rejected,
            }
            .render(Reading::Build),
            "the stored parameter interval's length at sample 0 escalated: margin 2e-12 lies \
             within the zero band (±1e-9). Recourse: move the geometry so this edge runs \
             forward over a clear length; no tolerance a run admits decides this length",
            "the whole sentence, so a second statement of the floor goes red"
        );
        for cause in [rejected, in_band] {
            let error = CertifyError::Escalated {
                check: CertCheck::ParamSpan,
                sample: 0,
                cause,
            };
            for reading in [Reading::Build, Reading::AtRest] {
                let text = error.render(reading);
                assert_eq!(text.matches(&floor).count(), 1, "{reading:?}: {text}");
                assert!(
                    !text.contains("no kernel construction mints") && !text.contains("tighten"),
                    "{reading:?}: {text}"
                );
            }
        }
    }

    /// **The import door reads certification as at rest, with its ε_in
    /// words** (D4 ¶1), over every check and every arm: the door's ending
    /// is the at-rest one, except that a band-decided arm of a sized
    /// decision whose margin's nearer end lies within ε_in ends in one
    /// sentence, whichever arm the run's band placed it on: the file
    /// cannot tell the size from zero, and keeping it takes declaring the
    /// file's uncertainty below it and tightening, together, or else the
    /// lever alone, with no decision's note, an enclosure across zero's
    /// included — and a residual's miss within ε_in but beyond ε, or a
    /// certified bound on it wholly within ε_in, names setting ε to ε_in
    /// as a stopgap. An ε_in below every margin leaves every ending as at
    /// rest but a reading at zero's, except that a defect ending names
    /// the file where rest names the kernel or the file. No ending at the
    /// door blames the kernel, or carries a second recourse.
    #[test]
    fn the_import_door_reads_at_rest_with_its_eps_in_words() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let tol = geom_core::Tol::witness();
        let run = Band::linear(tol).unwrap();
        let wide = FileCoincidence::new(run.escalate().max(1e-6));
        let narrow = FileCoincidence::new((run.escalate() * 0.5).min(1e-10));
        // Each undecided margin, and whether it is a miss within the wide
        // ε_in and beyond the band's zero threshold.
        let causes = [
            (MarginDiag::value(5e-9), true),
            (MarginDiag::value(-5e-9), true),
            (MarginDiag::enclosure(2.0e-9, 4.0e-9), true),
            (MarginDiag::enclosure(-2.0e-9, 4.0e-9), false),
            (MarginDiag::INVALID, false),
        ]
        .map(|(margin, miss)| {
            let cause = Indeterminate {
                margin,
                band,
                predicate: Some("a_probe"),
                terminal_sliver: false,
            };
            (cause, miss)
        });
        for check in all_checks() {
            // An incidence ends by its read, which no refused arm carries
            // ([`Ending::Incidence`]). At the import door a definite miss,
            // with or without the pose height it may quote, is the damaged
            // file, with no ε_in offer; a read in band is the geometry's
            // lever, as at rest, never a defect.
            if matches!(check.ending(), Ending::Incidence) {
                let cause = &causes[0].0;
                for read in [
                    crate::IncidenceRead::Off(None),
                    crate::IncidenceRead::Off(Some(MarginDiag::value(5e-9))),
                    crate::IncidenceRead::AtPose(MarginDiag::value(5e-9)),
                    crate::IncidenceRead::Undecided(*cause),
                ] {
                    for file in [wide, narrow] {
                        let door = read.ending(file);
                        assert!(!door.contains("ε_in"), "{read:?}: {door}");
                        if matches!(read, crate::IncidenceRead::Off(_)) {
                            assert_eq!(door, DAMAGED_FILE_ENDING, "{read:?}");
                        } else {
                            assert_eq!(door, read.ending(Reading::AtRest), "{read:?}");
                            assert!(!door.contains("defect"), "{read:?}: {door}");
                        }
                    }
                }
                continue;
            }
            let mut arms: Vec<(RefusedArm<'_>, bool)> = causes
                .iter()
                .map(|(cause, miss)| (RefusedArm::Undecided(cause), *miss))
                .collect();
            // A sign-certain arm with no reading compares nothing with
            // ε_in: it ends at rest.
            arms.push((RefusedArm::SignCertain(None), false));
            arms.push((RefusedArm::SignCertain(Some(MarginDiag::value(5e-9))), true));
            // The Interval lane's zero verdicts too: one across zero, as
            // a sub-ε_in span's enclosure reads.
            for margin in [
                MarginDiag::value(5e-10),
                MarginDiag::value(0.0),
                MarginDiag::value(-5e-10),
                MarginDiag::enclosure(-2e-12, 6e-12),
            ] {
                arms.push((RefusedArm::Zero(Classified { margin, band }), false));
            }
            for (arm, miss) in arms {
                let at_rest = recourse(check, arm, Reading::AtRest);
                // Past ε_in the door reads as rest does, but names the
                // file where rest names the kernel or the file: in the
                // defect ending, and in an unreadable margin's note.
                let past_eps_in = if at_rest == KERNEL_OR_FILE_DEFECT_ENDING {
                    DAMAGED_FILE_ENDING.to_owned()
                } else {
                    at_rest.replace(
                        geom_core::UNREADABLE_STORED_MARGIN_NOTE,
                        geom_core::UNREADABLE_FILE_MARGIN_NOTE,
                    )
                };
                let banded = match arm {
                    RefusedArm::Undecided(cause) => Some(cause.margin),
                    RefusedArm::Zero(Classified { margin, .. }) => Some(margin),
                    RefusedArm::SignCertain(_) | RefusedArm::Straddle => None,
                };
                // A reading at or across zero has its nearer end within
                // every ε_in, so even the narrow one reads it in the door's
                // sentence: "is within" where the whole reading is, "may lie
                // within" where only that end is, then the lever alone. The
                // door cannot tell a zero the file stores from a rounded
                // one, so no decision's note rides there.
                let reaches_zero = match banded.map(MarginDiag::diagnostic_f64_for_error_text) {
                    Some(geom_core::ErrorTextReading::Value(0.0)) => Some(0.0),
                    Some(geom_core::ErrorTextReading::Enclosure { lo, hi })
                        if lo <= 0.0 && hi >= 0.0 =>
                    {
                        Some(lo.abs().max(hi.abs()))
                    }
                    _ => None,
                };
                let narrow_door = recourse(check, arm, narrow);
                match (check.ending(), reaches_zero) {
                    (Ending::Sized(sized), Some(far))
                        if !(sized.passes.passes_zero() && matches!(arm, RefusedArm::Zero(_))) =>
                    {
                        let (lies, tells) = if far <= narrow.eps_in() {
                            ("is within", "cannot tell")
                        } else {
                            ("may lie within", "might not tell")
                        };
                        assert_eq!(
                            narrow_door,
                            format!(
                                "This {} {lies} the file's declared coincidence distance ε_in = \
                                 {:e} m, so the file {tells} it from zero. Recourse: {}",
                                sized.size,
                                narrow.eps_in(),
                                sized.lever
                            ),
                            "{check:?} {arm:?}: a reading at zero lies below every ε_in"
                        );
                    }
                    _ => assert_eq!(
                        narrow_door, past_eps_in,
                        "{check:?} {arm:?}: an ε_in below every margin picks no other words"
                    ),
                }
                let door = recourse(check, arm, wide);
                assert_ne!(door, KERNEL_DEFECT_ENDING, "{check:?} {arm:?}");
                assert_eq!(
                    door.matches("Recourse:").count() + door.matches("There is no way").count(),
                    1,
                    "{check:?} {arm:?}: {door}"
                );
                match (check.ending(), banded) {
                    (Ending::Sized(sized), Some(margin))
                        if margin != MarginDiag::INVALID
                            && !(sized.passes.passes_zero()
                                && matches!(arm, RefusedArm::Zero(_))) =>
                    {
                        // One sentence for every band-decided arm, the
                        // run's placement of it picking no other words.
                        assert!(
                            door.starts_with(&format!("This {} is within the file's", sized.size))
                                && door
                                    .contains("so the file cannot tell it from zero. Recourse: "),
                            "{check:?} {arm:?}: {door}"
                        );
                        if at_rest.contains("tighten") {
                            assert!(
                                door.contains(
                                    " is intended, re-export the file with its uncertainty \
                                     declared below"
                                ) && door.contains(" m and tighten the tolerance below "),
                                "{check:?} {arm:?}: {door}"
                            );
                        } else {
                            assert!(
                                door.ends_with(&format!("Recourse: {}", sized.lever)),
                                "{check:?} {arm:?}: {door}"
                            );
                        }
                    }
                    // A bound wholly within ε_in names itself, on its
                    // undecided and sign-certain arms alike: every
                    // readable margin here lies within the wide ε_in.
                    (Ending::Residual(Unsized::Bound), _) => {
                        let named = match arm {
                            RefusedArm::Undecided(cause) => !cause.margin.is_invalid(),
                            RefusedArm::SignCertain(margin) => margin.is_some(),
                            RefusedArm::Zero(_) | RefusedArm::Straddle => false,
                        };
                        if named {
                            assert!(
                                door.starts_with(
                                    "The certificate's bound on this miss lies within"
                                ) && door.contains("as a stopgap, set the tolerance to ε_in"),
                                "{check:?} {arm:?}: {door}"
                            );
                        } else {
                            assert_eq!(door, past_eps_in, "{check:?} {arm:?}");
                        }
                    }
                    (Ending::Residual(_), _) if miss => assert!(
                        door.contains("as a stopgap, set the tolerance to ε_in"),
                        "{check:?} {arm:?}: {door}"
                    ),
                    _ => assert_eq!(door, past_eps_in, "{check:?} {arm:?}"),
                }
            }
        }
    }

    /// **A definite residual miss is read by its own value at the import
    /// door** (D4 ¶1): within ε_in it names the stopgap, past ε_in the
    /// file, and a miss the kernel's fit may have made keeps the
    /// kernel-bug note rather than vouching for the file's data.
    #[test]
    fn a_definite_miss_at_the_import_door_reads_its_own_value() {
        let file = FileCoincidence::new(1e-6);
        let exceeded = |check, m| CertifyError::ResidualExceeded {
            check,
            sample: 0,
            margin: MarginDiag::value(m),
        };
        let stopgap = "Recourse: re-export the file more precisely, or, as a stopgap, set the \
                       tolerance to ε_in = 1e-6 m";
        let exact = exceeded(CertCheck::EndpointStart, 5e-7)
            .ending(file)
            .unwrap();
        assert_eq!(
            exact,
            format!(
                "This miss lies beyond the tolerance and within the file's declared \
                 coincidence distance ε_in = 1e-6 m, to which alone the file's data claims to \
                 agree. {stopgap}"
            )
        );
        let fit = exceeded(
            CertCheck::SurfaceResidual {
                surface: SurfaceKey::default(),
            },
            5e-7,
        )
        .ending(file)
        .unwrap();
        assert_eq!(
            fit,
            "This miss lies beyond the tolerance and within the file's declared coincidence \
             distance ε_in = 1e-6 m, and may be the kernel's own approximation. Recourse: \
             re-export the file more precisely, or, as a stopgap, set the tolerance to ε_in = \
             1e-6 m; this refusal may indicate a kernel bug worth reporting",
            "a fit's miss keeps the kernel-bug note and vouches for no file data"
        );
        // An enclosure across ε_in may lie within it, so the stopgap is
        // named with the defect it leaves if the miss persists.
        let across = CertifyError::ResidualExceeded {
            check: CertCheck::EndpointStart,
            sample: 0,
            margin: MarginDiag::enclosure(5e-7, 2e-6),
        }
        .ending(file)
        .unwrap();
        assert!(
            across.contains("and may lie within the file's declared")
                && across.contains(stopgap)
                && across.ends_with(
                    "a miss that persists there means the file is damaged, worth reporting to \
                     the program that wrote it"
                ),
            "{across}"
        );
        let surface = SurfaceKey::default();
        for check in [
            CertCheck::EndpointStart,
            CertCheck::EndpointEnd,
            CertCheck::WitnessSurfaceResidual { surface },
            CertCheck::WitnessMidpoint,
            CertCheck::SurfaceResidual { surface },
        ] {
            let past = exceeded(check, 2e-6);
            assert_eq!(
                past.ending(file).as_deref(),
                Some(DAMAGED_FILE_ENDING),
                "{check:?}: past ε_in"
            );
        }
    }

    /// **A definite contradiction's culprit follows the door**
    /// ([`ReadAt::culprit`]): one row per class, read at a build, at
    /// rest, and at the import door with the reading within ε_in and past
    /// it. A build names the kernel, rest the kernel or the file, and the
    /// door past ε_in the file, sized or not, whether adoption derived
    /// the comparand or the file stored it; a fit keeps its last resort
    /// at a build, and a bound everywhere.
    #[test]
    fn a_defects_culprit_follows_the_door() {
        let file = FileCoincidence::new(1e-6);
        let stopgap = "Recourse: re-export the file more precisely, or, as a stopgap, set the \
                       tolerance to ε_in = 1e-6 m";
        let file_within = format!(
            "This miss lies beyond the tolerance and within the file's declared coincidence \
             distance ε_in = 1e-6 m, to which alone the file's data claims to agree. {stopgap}"
        );
        let fit_within = format!(
            "This miss lies beyond the tolerance and within the file's declared coincidence \
             distance ε_in = 1e-6 m, and may be the kernel's own approximation. {stopgap}; this \
             refusal may indicate a kernel bug worth reporting"
        );
        let bound_within = format!(
            "The certificate's bound on this miss lies within the file's declared coincidence \
             distance ε_in = 1e-6 m. {stopgap}; this refusal may indicate a kernel bug worth \
             reporting"
        );
        let (k, kf, f) = (
            KERNEL_DEFECT_ENDING,
            KERNEL_OR_FILE_DEFECT_ENDING,
            DAMAGED_FILE_ENDING,
        );
        let lever = "Recourse: move the geometry so the faces curve apart more clearly where they \
                     touch";
        let sized_within = format!(
            "This curvature difference is within the file's declared coincidence distance ε_in = \
             1e-6 m, so the file cannot tell it from zero. {lever}"
        );
        let surface = SurfaceKey::default();
        // A sized decision's reading is band-decided on its refused side,
        // which the door compares with ε_in; a residual's miss is carried
        // on its sign-certain arm.
        let refused: fn(f64) -> RefusedArm<'static> = |m| {
            RefusedArm::Zero(Classified {
                margin: MarginDiag::value(-m),
                band: Band::new(1e-9, 1e-8).unwrap(),
            })
        };
        let missed: fn(f64) -> RefusedArm<'static> =
            |m| RefusedArm::SignCertain(Some(MarginDiag::value(m)));
        // (check, its class, its arm, build, at rest, within ε_in, past
        // ε_in)
        let rows = [
            (
                CertCheck::TangentSecondOrder,
                "a sized decision's stored contradiction",
                refused,
                lever,
                kf,
                sized_within.as_str(),
                f,
            ),
            (
                CertCheck::EndpointStart,
                "a residual over parameters adoption derives",
                missed,
                k,
                kf,
                &file_within,
                f,
            ),
            (
                CertCheck::WitnessMidpoint,
                "a residual over the witness adoption derives",
                missed,
                k,
                kf,
                &file_within,
                f,
            ),
            (
                CertCheck::ChartImage,
                "an unsized check over an image the kernel derives",
                missed,
                k,
                kf,
                f,
                f,
            ),
            (
                CertCheck::SurfaceResidual { surface },
                "a fit",
                missed,
                KERNEL_LIMIT_RECOURSE,
                kf,
                &fit_within,
                f,
            ),
            (
                CertCheck::TangentHull,
                "a bound",
                missed,
                KERNEL_LIMIT_RECOURSE,
                KERNEL_LIMIT_RECOURSE,
                &bound_within,
                KERNEL_LIMIT_RECOURSE,
            ),
        ];
        for (check, class, arm, build, at_rest, within, past) in rows {
            for (door, got, want) in [
                ("a build", recourse(check, arm(2e-6), Reading::Build), build),
                (
                    "at rest",
                    recourse(check, arm(2e-6), Reading::AtRest),
                    at_rest,
                ),
                ("within ε_in", recourse(check, arm(5e-7), file), within),
                ("past ε_in", recourse(check, arm(2e-6), file), past),
            ] {
                assert_eq!(got, want, "{check:?}, {class}, at {door}");
            }
        }
    }

    /// **Every sized decision ends past ε_in at the import door by its
    /// own refused side** (D4 ¶1 (iv)): a reading in band wholly on the
    /// refused side, its size past the file's ε_in, ends in the lever
    /// where the lever reaches that side, and in the damaged file's
    /// ending where it does not. The side is the class table's
    /// ([`class_table`]); each lever is written out here. A door's
    /// refusal of its stated interval ends as its check does. Red if the
    /// door names the kernel, or if a lever-side check is added without
    /// its lever.
    #[test]
    fn each_sized_decision_ends_past_eps_in_by_its_refused_side() {
        let file = FileCoincidence::new(1e-6);
        let band = Band::new(1e-9, 1e-8).unwrap();
        let in_band = Indeterminate {
            margin: MarginDiag::value(-2e-6),
            band,
            predicate: None,
            terminal_sliver: false,
        };
        let past = RefusedArm::Undecided(&in_band);
        let f = DAMAGED_FILE_ENDING;
        let levers = [
            (
                CertCheck::ParamSpanMeter,
                "Recourse: move the geometry so this spline edge runs steadily forward, never \
                 stalling or turning back; if it does neither, the speed floor has reached its \
                 limit, which is worth reporting",
            ),
            (
                CertCheck::TransversalityTube,
                "Recourse: move the geometry so the surfaces cross at a clearer angle",
            ),
            (
                CertCheck::TransversalityArm,
                "Recourse: move the geometry so that edge's faces clearly run farther and none \
                 curves tightly there; faces of no extent or a cone's point leave no angle to \
                 measure",
            ),
            (
                CertCheck::TangentTube,
                "Recourse: move the geometry so the faces curve apart more clearly where they \
                 touch",
            ),
            (
                CertCheck::PlaneNurbsChartSpeed,
                "Recourse: move the geometry so the spline face varies along both of its \
                 parameter directions",
            ),
            (
                CertCheck::PlaneNurbsChartSpeedBound,
                "Recourse: move the geometry to a scale where the spline face's control points \
                 stay well inside the range of finite numbers",
            ),
        ];
        let (mut on_lever, mut on_defect) = (0, 0);
        for (check, class) in class_table() {
            let Class::Sized(_, side) = class else {
                continue;
            };
            let want = match side {
                RefusedSide::Lever => {
                    on_lever += 1;
                    levers
                        .iter()
                        .find(|(c, _)| *c == check)
                        .unwrap_or_else(|| panic!("{check:?}: its lever is written out here"))
                        .1
                }
                RefusedSide::StoredContradiction | RefusedSide::Unminted => {
                    on_defect += 1;
                    f
                }
            };
            assert_eq!(recourse(check, past, file), want, "{check:?} past ε_in");
        }
        assert_eq!(
            on_lever,
            levers.len(),
            "every lever written out is a lever side's"
        );
        assert!(on_defect > 0, "the table has a defect side to read");
        for refused in [
            StatedSpan::Forward(ForwardReading::InBand(in_band)),
            StatedSpan::Winding(WindingReading::InBand(in_band)),
        ] {
            let stated = CertifyError::StatedInterval { refused };
            assert_eq!(
                stated.ending(file).as_deref(),
                Some(f),
                "{refused:?}: the stated interval past ε_in"
            );
        }
    }

    /// **A refusal on a certified bound is the certificate's limit**
    /// (D4 ¶1 (i)): a loose bound contradicts nothing stored, so every
    /// arm ends in the last resort at every reading, and at the import
    /// door a bound within ε_in names itself and the stopgap. A bound
    /// past ε_in, or only partly within it, ends in the last resort: it
    /// says nothing about where the miss lies.
    #[test]
    fn a_certified_bound_refusal_is_the_certificates_limit() {
        use crate::edge_nurbs::PlaneNurbsRefusal as P;
        use crate::ssi::SsiLimb;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let file = FileCoincidence::new(1e-6);
        let named = "The certificate's bound on this miss lies within the file's declared \
                     coincidence distance ε_in = 1e-6 m. Recourse: \
                     re-export the file more precisely, or, as a stopgap, set the tolerance to \
                     ε_in = 1e-6 m; this refusal may indicate a kernel bug worth reporting";
        let undecided = |margin| Indeterminate {
            margin,
            band,
            predicate: Some("a_probe"),
            terminal_sliver: false,
        };
        let within = undecided(MarginDiag::value(5e-9));
        let poisoned = undecided(MarginDiag::INVALID);
        for check in [CertCheck::TangentHull, CertCheck::PlaneNurbsHull] {
            let exceeded = |margin| CertifyError::ResidualExceeded {
                check,
                sample: NOT_A_SAMPLE,
                margin,
            };
            let escalated = |cause| CertifyError::Escalated {
                check,
                sample: NOT_A_SAMPLE,
                cause,
            };
            let inside = exceeded(MarginDiag::value(5e-7));
            for reading in [Reading::Build, Reading::AtRest] {
                assert_eq!(
                    inside.ending(reading).unwrap(),
                    KERNEL_LIMIT_RECOURSE,
                    "{check:?} {reading:?}"
                );
                assert_eq!(
                    escalated(within).ending(reading).unwrap(),
                    KERNEL_LIMIT_RECOURSE,
                    "{check:?} {reading:?}"
                );
            }
            assert_eq!(
                escalated(poisoned).ending(Reading::AtRest).unwrap(),
                KERNEL_OR_FILE_DEFECT_ENDING,
                "{check:?}: an unreadable margin is a defect"
            );
            assert_eq!(inside.ending(file).unwrap(), named, "{check:?}");
            assert_eq!(
                escalated(within).ending(file).unwrap(),
                named,
                "{check:?}: the in-band route names the bound too"
            );
            for past in [
                exceeded(MarginDiag::value(2e-6)),
                exceeded(MarginDiag::enclosure(5e-7, 2e-6)),
            ] {
                assert_eq!(
                    past.ending(file).unwrap(),
                    KERNEL_LIMIT_RECOURSE,
                    "{check:?} {past:?}"
                );
            }
        }
        let limb = CertifyError::PlaneNurbs(P::Limb {
            limb: SsiLimb::HullSup,
            margin: MarginDiag::value(5e-7),
        });
        assert_eq!(limb.ending(Reading::AtRest).unwrap(), KERNEL_LIMIT_RECOURSE);
        assert_eq!(limb.ending(file).unwrap(), named);
        assert_eq!(
            limb.ending(FileCoincidence::new(1e-7)).unwrap(),
            KERNEL_LIMIT_RECOURSE
        );
    }

    /// **The analytic rung-3 lane's one-arc refusal ends at the import
    /// door as the plane x NURBS lane's does** (D4 ¶1): one stored edge's
    /// one-arc proof, whichever lane certified it, with the file's ε_in
    /// words on its band-decided arm. Red where the door finds no
    /// decision for the analytic lane's refusal and renders its payload
    /// with no ending.
    #[test]
    fn the_analytic_rung3_one_arc_refusal_ends_at_the_import_door() {
        use crate::edge_nurbs::{AnalyticRung3Refusal, PlaneNurbsRefusal};
        use crate::ssi::OneArcRefusal;
        let file = FileCoincidence::new(1e-6);
        let undecided = OneArcRefusal::Undecided(Indeterminate {
            margin: MarginDiag::value(5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("ssi_tube_one_arc"),
            terminal_sliver: false,
        });
        for cause in [
            OneArcRefusal::Count { solutions: 0 },
            OneArcRefusal::Unlinked,
            OneArcRefusal::Short,
            undecided,
            OneArcRefusal::AlongSide { chart: 0 },
            OneArcRefusal::EndOnSides { sides: 2 },
        ] {
            let analytic = CertifyError::AnalyticRung3(AnalyticRung3Refusal::TubeNotOneArc {
                rungs: 3,
                cause,
            });
            let plane =
                CertifyError::PlaneNurbs(PlaneNurbsRefusal::TubeNotOneArc { rungs: 3, cause });
            let door = analytic.ending(file);
            assert!(door.is_some(), "{cause:?}: the door gives no ending");
            assert_eq!(door, plane.ending(file), "{cause:?}");
            assert_eq!(
                analytic.render(file),
                format!("{analytic}. {}", door.unwrap()),
                "{cause:?}"
            );
        }
    }

    /// **The collapsed-arm gates' definite refusals read the import door
    /// like every sized decision's** (D4 ¶1): a collapsed transversality
    /// arm and a collapsed span meter end through [`recourse`] at the door, so
    /// a zero within the file's ε_in is a size the file cannot tell from
    /// zero, not the at-rest offer to tighten alone.
    #[test]
    fn the_collapsed_arm_gates_read_the_import_door() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let file = FileCoincidence::new(1e-6);
        let verdict = Refused::Zero(Classified {
            margin: MarginDiag::value(5e-10),
            band,
        });
        for (refusal, check) in [
            (
                CertifyError::ArmCollapsed { sample: 4, verdict },
                CertCheck::TransversalityArm,
            ),
            (
                CertifyError::SpanMeterCollapsed { verdict },
                CertCheck::ParamSpanMeter,
            ),
        ] {
            let door = refusal.ending(file).unwrap();
            assert_eq!(
                door,
                recourse(check, verdict.arm(), file),
                "{check:?}: the door's ending is the decision's"
            );
            assert!(
                door.contains("so the file cannot tell it from zero. Recourse: ")
                    && door.contains(" m and tighten the tolerance below 5e-11 m"),
                "{check:?}: {door}"
            );
            assert_ne!(
                Some(door),
                refusal.ending(Reading::AtRest),
                "{check:?}: the door reads ε_in"
            );
        }
    }

    /// **An intersection edge through a cone's apex refuses on the
    /// transversality's arm, definitely, not on its wedge**: the plane
    /// `x = 0` cuts the cone along a generator, and the edge runs along
    /// it through the apex, where the cone's radius of curvature, and
    /// with it the arm the wedge is metered over, is zero. The middle
    /// sample lands on the apex, so certification refuses there as
    /// [`CertifyError::ArmCollapsed`] (the samples before it are
    /// transverse): the arm is decided zero, which is a verdict of its
    /// own, read as a length that is not there and ending in the arm's
    /// lever, never as "escalated" and never as the wedge's angle.
    #[test]
    fn an_intersection_through_a_cone_apex_refuses_on_the_arm() {
        let half = std::f64::consts::FRAC_PI_6;
        let (keys, lookup) = table(vec![
            Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: half,
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: Point3::origin(),
                normal: Vec3::unit_x(),
                u_ref: Vec3::unit_y(),
            },
        ]);
        let dir = Vec3::new(0.0, half.sin(), half.cos());
        let reach = 1.0 / half.cos();
        let p0 = Point3::origin() - dir * reach;
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(keys[0], keys[1]),
                selector: crate::CurveSelector::Witness(p0 + dir * (reach / 2.0)),
            },
            carrier: Curve3::Line { origin: p0, dir },
            param_start: 0.0,
            param_end: 2.0 * reach,
        };
        let p1 = spec.carrier.eval(2.0 * reach);
        let err = EdgeCurve::certify(spec, p0, p1, &lookup, 1.0, band(), NurbsLane::certified())
            .unwrap_err();
        assert!(
            matches!(
                err,
                CertifyError::ArmCollapsed {
                    sample: 4,
                    verdict: Refused::Zero(_),
                }
            ),
            "{err:?}"
        );
        assert_eq!(
            err.decision().map(|(check, _)| check),
            Some(CertCheck::TransversalityArm)
        );
        let text = err.render(Reading::Build);
        assert_eq!(
            text,
            "at sample 4 the edge is not shown between faces wide enough, for how they curve, to \
             measure their angle (margin 0 m). Recourse: move the geometry so that \
             edge's faces clearly run farther and none curves tightly there; faces of no \
             extent or a cone's point leave no angle to measure",
        );
    }

    /// **A zero verdict's payload prints its reading and claims no size**:
    /// whether the reading shows none is the ending's to say, from the
    /// reading's class, so a positive span and an enclosure across zero
    /// render no such claim anywhere, and a reading that does show none
    /// says so once, in the ending. Each zero payload says "not shown" of
    /// its reading, which holds of every class a zero verdict carries — a
    /// symbolic-tier proved zero whose rounded number lies outside the
    /// zero band included — and prints it as `(margin … m)`, or as the
    /// `(length … m)` its door names. A negative payload says what its
    /// verdict decided, with the same number. No payload says a word
    /// twice.
    #[test]
    fn each_zero_arm_payload_prints_its_reading_and_claims_no_size() {
        use geom_core::sym::with_session;
        use geom_core::{Margin, ParamSymbol, Sym, SymBudget};

        let band = Band::new(1e-9, 1e-8).unwrap();
        let zero = |margin| Refused::Zero(Classified { margin, band });
        let (sized, across, none) = (
            MarginDiag::value(5e-10),
            MarginDiag::enclosure(-2e-10, 3e-10),
            MarginDiag::value(0.0),
        );
        // `x·3/3 − x` at this `x` rounds to 3.7e-9 m, past the zero
        // band's 1e-9 m; the symbolic tier proves it zero.
        let (proved, _) = with_session(
            SymBudget {
                max_terms: 64,
                max_degree: 8,
            },
            || {
                let x = Sym::param(ParamSymbol::new(0xa8), 23_456_789.1_f64);
                let three = Sym::from_f64(3.0);
                crate::dihedral::decide_positive("pin", Margin::of(x * three / three - x), band)
                    .unwrap_err()
            },
        );
        let proved = Refused::rejected(&proved).unwrap();
        assert!(
            matches!(proved, Refused::Zero(Classified { margin, .. }) if margin.proven()),
            "the fixture is a proved zero: {proved:?}"
        );
        let forward = |verdict| CertifyError::IntervalNotForward { verdict };
        let meter = |verdict| CertifyError::SpanMeterCollapsed { verdict };
        let arm = |verdict| CertifyError::ArmCollapsed { sample: 4, verdict };
        let crossing = |lever, verdict| CertifyError::NotTransverse {
            sample: 4,
            lever,
            verdict,
        };
        let second = |verdict| CertifyError::NotSecondOrderSeparated { sample: 4, verdict };
        let tube = |verdict| CertifyError::TubeNotSeparated { verdict };
        let below = Refused::Negative {
            margin: MarginDiag::value(-5e-9),
        };
        let extent = Some(crate::ssi::PointLever::Extent);
        let rows = [
            (
                forward(zero(sized)),
                "the stored parameter interval is not shown to run forward (length 5e-10 m)",
            ),
            (
                forward(zero(across)),
                "the stored parameter interval is not shown to run forward (length [-2e-10, \
                 3e-10] m)",
            ),
            (
                forward(proved),
                "the stored parameter interval is not shown to run forward (length \
                 3.725290298461914e-9 m)",
            ),
            (
                forward(zero(none)),
                "the stored parameter interval is not shown to run forward (length 0 m)",
            ),
            (
                forward(below),
                "the stored parameter interval runs backwards by 5e-9 m",
            ),
            (
                CertifyError::WindingExceeded,
                "the stored parameter interval spans more than one full turn of its carrier",
            ),
            (
                meter(zero(sized)),
                "the lower bound on the spline carrier's length from its metered knot domain is \
                 not shown above zero (margin 5e-10 m), so the stored interval has no proven \
                 metre scale",
            ),
            (
                meter(below),
                "the lower bound on the spline carrier's length from its metered knot domain is \
                 below zero (margin -5e-9 m), so the stored interval has no proven metre scale",
            ),
            (
                arm(zero(across)),
                "at sample 4 the edge is not shown between faces wide enough, for how they curve, \
                 to measure their angle (margin [-2e-10, 3e-10] m)",
            ),
            (
                arm(zero(none)),
                "at sample 4 the edge is not shown between faces wide enough, for how they curve, \
                 to measure their angle (margin 0 m)",
            ),
            (
                arm(below),
                "at sample 4 the edge is not between faces wide enough, for how they curve, to \
                 measure their angle (margin -5e-9 m)",
            ),
            (
                crossing(None, zero(sized)),
                "at sample 4 the faces are not shown to cross (margin 5e-10 m of the crossing \
                 angle, levered by the arm it is read over), though the edge's description says \
                 they do",
            ),
            (
                crossing(extent, zero(sized)),
                "at sample 4 the faces are not shown to cross (margin 5e-10 m of the crossing \
                 angle, levered by the feature extent), though the edge's description says they \
                 do",
            ),
            (
                crossing(None, below),
                "at sample 4 the faces do not cross (margin -5e-9 m of the crossing angle, levered \
                 by the arm it is read over), though the edge's description says they do",
            ),
            (
                crossing(extent, below),
                "at sample 4 the faces do not cross (margin -5e-9 m of the crossing angle, levered \
                 by the feature extent), though the edge's description says they do",
            ),
            (
                second(zero(sized)),
                "at sample 4 the faces are not shown to curve apart at second order (margin \
                 5e-10 m of the curvature difference, levered by the arm it is read over), so they \
                 might not fix where the edge runs, though its description says they do",
            ),
            (
                second(below),
                "at sample 4 the faces do not curve apart at second order (margin -5e-9 m of the \
                 curvature difference, levered by the arm it is read over), so they cannot fix \
                 where the edge runs, though its description says they do",
            ),
            (
                tube(zero(sized)),
                "along the whole edge the second-order tube's lower bound on the curvature \
                 difference is not shown above zero (margin 5e-10 m, levered by the arm it is read \
                 over), so the faces are not proved to fix where it runs, though its description \
                 says they do",
            ),
            (
                tube(below),
                "along the whole edge the second-order tube's lower bound on the curvature \
                 difference is below zero (margin -5e-9 m, levered by the arm it is read over), so \
                 the faces are not proved to fix where it runs, though its description says they \
                 do",
            ),
        ];
        // No payload claims no size or names a cause its verdict did not
        // decide, and a definite arm hedges nothing. A zero arm's whole
        // render, ending included, claims no size either.
        let claims = [
            "no length",
            "not long enough",
            "not above zero",
            "spans no",
            "is zero",
            "within the tolerance",
        ];
        for (error, payload) in rows {
            let text = error.to_string();
            assert_eq!(text, payload, "{error:?}");
            for claim in claims {
                assert!(
                    !text.contains(claim),
                    "the payload claims {claim:?}: {text}"
                );
            }
            assert_eq!(
                crate::recourse::repeated_words(&text),
                Vec::<String>::new(),
                "a word said twice: {text}"
            );
            let shown = match error.decision().map(|(_, arm)| arm) {
                Some(RefusedArm::SignCertain(_)) => {
                    assert!(
                        !text.contains("not shown"),
                        "a decided verdict hedged: {text}"
                    );
                    continue;
                }
                Some(RefusedArm::Zero(Classified { margin, .. })) => margin,
                arm => panic!("every row is a refused verdict: {arm:?}"),
            };
            // An exact or proved zero shows no size, so its ending says so.
            if shown == none || shown.proven() {
                continue;
            }
            for reading in [Reading::Build, Reading::AtRest] {
                let text = error.render(reading);
                for claim in claims {
                    assert!(
                        !text.contains(claim),
                        "{reading:?} claims {claim:?}: {text}"
                    );
                }
            }
        }
        for (error, note) in [
            (meter(zero(none)), "the speed floor has reached its limit"),
            (arm(zero(none)), "faces of no extent"),
        ] {
            let text = error.render(Reading::Build);
            assert_eq!(
                text.matches(note).count(),
                1,
                "a reading of no size says so once, in its ending: {text}"
            );
            assert!(!error.to_string().contains(note), "{text}");
        }
    }

    /// **An undecided arm stays undecided when the wedge it quotes reads
    /// zero**: a plane crossing a cylinder of in-band radius along a
    /// ruling, at an angle whose wedge over that radius is inside the
    /// zero band but which reads no class at the radius's own tolerance.
    /// The arm gate cannot decide the radius, and its escalation quotes
    /// the wedge's decided zero, a tagged rejection of its own, so the
    /// edge refuses as the arm's undecided escalation, never as
    /// [`CertifyError::ArmCollapsed`]: the collapse is the gate's
    /// verdict, never read off the quoted margin.
    #[test]
    fn an_in_band_arm_quoting_a_zero_wedge_stays_undecided() {
        let b = band();
        let radius = (b.zero() + b.escalate()) / 2.0;
        let sin = (b.zero() / b.escalate() + b.zero() / radius) / 2.0;
        let normal = Vec3::new((1.0 - sin * sin).sqrt(), sin, 0.0);
        let on = Point3::new(radius, 0.0, 0.0);
        let (keys, lookup) = table(vec![
            Surface::Cylinder {
                origin: Point3::origin(),
                axis: Vec3::unit_z(),
                radius,
                u_ref: Vec3::unit_x(),
            },
            Surface::Plane {
                origin: on,
                normal,
                u_ref: Vec3::unit_z(),
            },
        ]);
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection {
                pair: SurfacePair::new(keys[0], keys[1]),
                selector: crate::CurveSelector::Witness(on + Vec3::unit_z() * 0.5),
            },
            carrier: Curve3::Line {
                origin: on,
                dir: Vec3::unit_z(),
            },
            param_start: 0.0,
            param_end: 1.0,
        };
        let end = on + Vec3::unit_z();
        let err =
            EdgeCurve::certify(spec, on, end, &lookup, 1.0, b, NurbsLane::certified()).unwrap_err();
        let CertifyError::Escalated {
            check: CertCheck::TransversalityArm,
            cause,
            ..
        } = err
        else {
            panic!("the arm escalates undecided: {err:?}");
        };
        assert_eq!(
            (cause.predicate, cause.margin.rejected_sign()),
            (Some("dihedral_arm_wedge"), Some(Sign::Zero)),
            "the escalation quotes the wedge's decided zero: {cause:?}"
        );
    }

    /// **The transversality margin's lever arm ends as a width, not an
    /// angle**: an escalation of the arm the wedge is metered over names
    /// that edge's faces' extent and their bend, and offers the tolerance
    /// the arm gives, in band and in the zero band alike; the wedge's own
    /// escalation keeps the angle. (The review of PR 3513's fix pass
    /// rendered the arm under `Transversality`: "if this angle is
    /// intended, tighten the tolerance below 5e-11 m".)
    #[test]
    fn the_transversality_arm_ends_as_a_width_not_an_angle() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        for (margin, below) in [(5e-9, "5e-10"), (5e-10, "5e-11")] {
            let cause = Indeterminate {
                margin: MarginDiag::value(margin),
                band,
                predicate: Some("dihedral_arm"),
                terminal_sliver: false,
            };
            let render = |check| {
                CertifyError::Escalated {
                    check,
                    sample: 1,
                    cause,
                }
                .render(Reading::Build)
            };
            let arm = render(CertCheck::TransversalityArm);
            assert!(
                arm.starts_with(
                    "the width the faces' angle is measured across at sample 1 escalated: "
                ) && arm.ends_with(&format!(
                    "Recourse: move the geometry so that edge's faces clearly run farther and \
                         none curves tightly there, or, if this width or the gap its faces \
                         open is intended, tighten the tolerance below {below} m"
                )),
                "{arm}"
            );
            assert!(
                render(CertCheck::Transversality).contains("if this angle is intended"),
                "the wedge keeps its angle"
            );
        }
    }

    /// **A carrier off its chart ends by the door that reads it, and one
    /// read in band at a pose in its lever** (D3, D4 ¶1 (iv)): the edge
    /// certificate's `CarrierOffChart`, decided off (with no margin or
    /// past the pose) or read within the band at the pose. A definite miss
    /// is the kernel's defect at a build, the stored culprit at rest and
    /// the damaged file at import; the read in band is the geometry's
    /// lever at every door, never a defect, and only a definite miss says
    /// the curve does not lie on its chart.
    #[test]
    fn a_carrier_off_its_chart_ends_by_the_door_and_at_a_pose_in_its_lever() {
        let refusal = |margin| CertifyError::CarrierOffChart {
            chart: geom::SurfaceKind::Cone,
            carrier: geom::CurveKind::Line,
            why: "WHY",
            margin,
        };
        let m = geom_core::MarginDiag::value(4e-10);
        let file = FileCoincidence::new(1e-6);
        for (margin, off) in [
            (None, true),
            (Some(crate::OffMargin::Past(m)), true),
            (Some(crate::OffMargin::InBand(m)), false),
        ] {
            let e = refusal(margin);
            assert_eq!(e.to_string().contains("does not lie on"), off, "{e}");
            assert_eq!(
                e.decision().map(|(check, _)| check),
                Some(CertCheck::ChartIncidence),
                "{e:?}"
            );
            let at = |at: ReadAt| e.ending(at).unwrap_or_default();
            if off {
                assert_eq!(at(Reading::Build.into()), KERNEL_DEFECT_ENDING, "{e:?}");
                assert_eq!(
                    at(Reading::AtRest.into()),
                    geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
                    "{e:?}"
                );
                assert_eq!(at(file.into()), DAMAGED_FILE_ENDING, "{e:?}");
            } else {
                for door in [Reading::Build.into(), Reading::AtRest.into(), file.into()] {
                    let ending = at(door);
                    assert!(
                        ending.starts_with("Recourse: move the geometry so the edge sits clear")
                            && !ending.contains("defect"),
                        "{e:?} at {door:?}: {ending}"
                    );
                }
            }
        }
    }

    /// A decision's class, as the hand-written table states it
    /// ([`class_table`]).
    #[derive(Debug, PartialEq, Eq)]
    enum Class {
        Sized(SizedPass, RefusedSide),
        NoSize(Unsized),
        Miss(Unsized),
        Undefined,
        /// A lane choice's lever-only "not yet" ([`Ending::NotYet`]).
        NotYet,
        /// Ended by its read ([`Ending::Incidence`]): a definite miss by
        /// the door, a read in band by its lever.
        Incidence,
    }

    /// **Every decision's class, written out by hand** in taxonomy
    /// order: the one place a check's refused side is spelled for the
    /// tests, so they read it here rather than restate it.
    fn class_table() -> Vec<(CertCheck, Class)> {
        use Class::{Miss, NoSize, NotYet, Undefined};
        use RefusedSide::{Lever, StoredContradiction, Unminted};
        use SizedPass::{NonNegative, Positive};
        vec![
            (CertCheck::EndpointStart, Miss(Unsized::Defect)),
            (CertCheck::EndpointEnd, Miss(Unsized::Defect)),
            (CertCheck::ParamSpan, Class::Sized(Positive, Unminted)),
            (CertCheck::ParamSpanMeter, Class::Sized(Positive, Lever)),
            (CertCheck::ParamWinding, Class::Sized(NonNegative, Unminted)),
            (
                CertCheck::ImageSpan,
                Class::Sized(Positive, StoredContradiction),
            ),
            (
                CertCheck::SurfaceResidual {
                    surface: SurfaceKey::default(),
                },
                Miss(Unsized::Fit),
            ),
            (
                CertCheck::WitnessSurfaceResidual {
                    surface: SurfaceKey::default(),
                },
                Miss(Unsized::Defect),
            ),
            (CertCheck::WitnessMidpoint, Miss(Unsized::Defect)),
            (
                CertCheck::Transversality,
                Class::Sized(Positive, StoredContradiction),
            ),
            (CertCheck::TransversalityTube, Class::Sized(Positive, Lever)),
            (CertCheck::TransversalityArm, Class::Sized(Positive, Lever)),
            (CertCheck::TangentPlanes, Undefined),
            (
                CertCheck::TangentSecondOrder,
                Class::Sized(Positive, StoredContradiction),
            ),
            (CertCheck::TangentParallel, Miss(Unsized::Defect)),
            (CertCheck::TangentGap, Miss(Unsized::Defect)),
            (CertCheck::TangentHull, Miss(Unsized::Bound)),
            (CertCheck::TangentTube, Class::Sized(Positive, Lever)),
            (CertCheck::MappedSource, Miss(Unsized::Defect)),
            (CertCheck::ChartResidual, Miss(Unsized::Defect)),
            (CertCheck::ChartImage, NoSize(Unsized::Defect)),
            (CertCheck::ChartClass, NotYet),
            (CertCheck::GeneralLane, NotYet),
            (CertCheck::ChartIncidence, Class::Incidence),
            (CertCheck::PlaneNurbsOnLocus, Miss(Unsized::Fit)),
            (CertCheck::PlaneNurbsHull, Miss(Unsized::Bound)),
            (CertCheck::PlaneNurbsHullValue, Miss(Unsized::Bound)),
            (CertCheck::AnalyticBreakResidual, Miss(Unsized::Fit)),
            (CertCheck::AnalyticHull, Miss(Unsized::Bound)),
            (
                CertCheck::PlaneNurbsReportedTransversality,
                NoSize(Unsized::Defect),
            ),
            (
                CertCheck::PlaneNurbsChartSpeed,
                Class::Sized(Positive, Lever),
            ),
            (
                CertCheck::PlaneNurbsChartSpeedBound,
                Class::Sized(Positive, Lever),
            ),
        ]
    }

    /// Every decision's class, pinned against [`class_table`] (D4 ¶1
    /// (i)): a decision that passes on a nonzero sign is sized, with its
    /// pass set and its refused side; an exact construction ends as a
    /// defect, an approximation as a fit and a certified bound on the
    /// miss as a bound; a residual, whose refused margin is a miss, is
    /// marked so as any of them. The table is the independent side, so a
    /// decision `ending()` misfiles fails here.
    #[test]
    fn each_decision_is_classified_as_the_table_says() {
        use Class::{Miss, NoSize, NotYet, Undefined};
        let table = class_table();
        assert_eq!(table.len(), all_checks().len());
        for check in all_checks() {
            let (_, want) = table
                .iter()
                .find(|(c, _)| *c == check)
                .unwrap_or_else(|| panic!("{check:?} is missing from the table"));
            let got = match check.ending() {
                Ending::Sized(sized) => Class::Sized(sized.passes, sized.side),
                Ending::Unsized(no_size) => NoSize(no_size),
                Ending::Residual(residual) => Miss(residual),
                Ending::Undefined(_) => Undefined,
                Ending::NotYet => NotYet,
                Ending::Incidence => Class::Incidence,
            };
            assert_eq!(&got, want, "{check:?}");
            // A lever is named exactly where the decision is sized. An
            // incidence ends by its read, never from the check.
            let at_build = match check.ending() {
                Ending::Incidence => crate::IncidenceRead::Off(None).ending(Reading::Build),
                _ => recourse(check, RefusedArm::SignCertain(None), Reading::Build),
            };
            let named = at_build.starts_with("Recourse: move the geometry");
            assert_eq!(named, matches!(want, Class::Sized(..)), "{check:?}");
        }
    }
}

//! **The plane × NURBS edge lane** (M7-8): the declare-and-check
//! certification of an `Intersection` edge whose two adjacent surfaces
//! are one PLANE and one described NURBS patch.
//!
//! # Declare and check
//!
//! An imported file STATES a carrier for such an edge; nothing in the
//! kernel constructed it, and no closed form exists on the NURBS side
//! to compare it against. So the carrier is **evidence**: adopted as
//! stated, then certified against both surfaces, and refused typed
//! with the measured bound when it does not hold up. That is the
//! ratified shape (Ev, PR #264) and it is what this module
//! implements — never a fit, never a widened gate.
//!
//! # What the lane proves
//!
//! Everything here is the rung-3 SSI certificate (`ssi::certify`'s
//! `certify_branch`, which [`crate::ssi`]'s `certify_rung3` opens for a
//! marched spline) applied to a **declared** carrier of any kind
//! instead of a marched one. That door already answers exactly the three
//! questions this edge class raises, so the lane supplies the one
//! ingredient a declared carrier is missing — the chart image — and
//! delegates:
//!
//! * **on-PLANE residual**: closed form, the analytic operand's
//!   linearized implicit residual at the fixed schedule plus its
//!   certified composite hull sup over the whole span.
//! * **on-NURBS residual**: `|C(t) − S(u*, v*)|` at a **certified foot
//!   point** ([`geom::NurbsSurface::project`], D9-fixed), and the
//!   between-samples obligation discharged by the tensor-product
//!   Bernstein composite `sup_t |S(P(t)) − C(t)|` — a whole-curve
//!   bound, not a sampled max.
//! * **transversality**: the per-sample sine of the angle between the
//!   plane's exact normal and the wall's normal from `ders`, reported
//!   for the caller to meter at the edge's honest lever arm, plus the
//!   uniqueness tube's own certified margin over the box chain.
//!
//! # The chart image is derived, and therefore checked
//!
//! The pcurve `P(t)` is not in the file. This lane derives it — foot
//! points at the fixed schedule, interpolated on the carrier's own
//! parameter (the OQ4 identity: `P(t)` and `C(t)` are the same `t` by
//! construction) — which is a **structure** selection in C6's `f64`
//! lane, lifted to the caller's scalar. A derived structure certifies
//! nothing by itself; what makes it sound is that limb 2 bounds
//! `sup_t |S(P(t)) − C(t)|` over the whole span, so a chart image that
//! wanders off the true foot path enlarges the bound it must pass.
//! A wrong `P` cannot launder a wrong carrier: it can only refuse.
//!
//! # Which scalars may derive this certificate
//!
//! Limbs 2 and 3 are C9 certification bounds and the foot point is a bracket
//! read, so the honest signature is
//! `T: Decide + Bounds + CertifiedEnclosure`
//! (`geom_core::Bounds`'s compound-allowlist note, M6-2), and
//! [`plane_nurbs_limbs`] carries it. That bound is the whole split: it
//! admits `f64`, the telemetry probe and the interval scalar, and it
//! does not admit `geom_core::Dual`, which may not certify (D1,
//! 2026-08-19 — it carries the value channel's bracket, and that is
//! not the right to mint a C9 certification bound). A dual does not receive a
//! refusal here; it cannot write the call. `Bounds` stays off `topo`'s
//! default signatures because the capability is a sealed value
//! ([`crate::certify::NurbsLane`], the shape [`crate::FittedLane`] has)
//! that the shared machinery takes as an argument and `topo` reads off
//! the scalar's policy, rather than a bound raised into that machinery.
//!
//! **The symbolic tier rides the same bound and needs no arm of its
//! own** (`geom_core::sym`): `Sym<T>` implements
//! [`geom_core::Bounds`], `geom_core::CertifiedEnclosure` and
//! `geom_core::Decide` exactly when its base scalar does, so
//! `Sym<T>` satisfies this signature for every certifying `T` — the
//! limbs are the base scalar's, run at `Sym<T>`. The tier changes how
//! one class of margin decides and nothing about the certificate, and
//! a bound says that without an impl to write.

use crate::pcurve_cache::Pcurve;
use core::num::NonZeroUsize;
use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::spline::{KnotVector, SpanLocate, SplineError};
use geom_core::{
    Band, Bounds, Decide, Decided, Indeterminate, MarginDiag, Point2, Point3, Readable, Real, Sign,
    Vec3,
};

use crate::certify::{CERT_SAMPLES, CertCheck, recourse, schedule_fraction};
use crate::recourse::{ReadAt, Refused, RefusedArm};
use crate::ssi::enclose::CellGrid;
use crate::ssi::{
    ChartSpeedRefusal, OneArcRefusal, PointLever, SsiError, SsiLimb, SsiOperand, SsiTube,
};

/// What the plane × NURBS lane proved, in metres unless noted.
#[derive(Clone, Copy, Debug)]
pub struct PlaneNurbsLimbs<T: Real> {
    /// Limb 1: the largest on-locus residual over the schedule, over
    /// **both** operands (the sampled max — it steers, it does not
    /// certify).
    pub on_locus_max: T,
    /// Limb 2: the certified sup-norm bound over the whole span, over
    /// both operands. This is the number that certifies.
    pub hull_sup: T,
    /// Limb 3: the region the uniqueness tube proved — the chart tube's
    /// per-axis pad in chart units, and the ladder rung it was tried at.
    pub tube: SsiTube<T>,
    /// Limb 3: the smallest certified transversality margin over the
    /// box chain, in meters.
    pub tube_transversality: T,
    /// Limb 3: how many boxes the chain has.
    pub tube_boxes: u32,
    /// The smallest sampled **sine of the normal angle** over the
    /// interior schedule samples — dimensionless, already classified
    /// definitely-transverse at the edge's lever arm (D4 ¶1), exactly
    /// as the analytic `Intersection` arm classifies its dihedral.
    pub min_sin_theta: T,
}

/// The lane's typed refusal — actionable, closed, and always carrying
/// the measured number when one exists.
#[derive(Clone, Copy, Debug, PartialEq)]
// The variant roster `topo`'s sample-coverage row reads (this
// crate's `test-support` feature, test builds only).
#[cfg_attr(
    feature = "test-support",
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(PlaneNurbsRefusalKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum PlaneNurbsRefusal {
    /// The foot-point projection did not converge at a schedule
    /// sample. Never a best-effort foot.
    FootPointInconclusive {
        /// The schedule sample index.
        sample: u32,
        /// `|S − P|` at the last iterate (NaN when poisoned).
        last_distance: f64,
    },
    /// The tangent planes coincide at an interior schedule sample: the
    /// `Intersection` transversality precondition fails, so the locus
    /// is not an intersection at this ε (D2's taxonomy sends a
    /// tangential contact to `TangentIntersection`).
    NotTransverse {
        /// The interior sample index.
        sample: u32,
        /// Which length levered the angle.
        lever: PointLever,
        /// The verdict on the levered angle, with its reporting margin.
        verdict: Refused,
    },
    /// The interpolation through the schedule's foot points refused
    /// ([`PCURVE_FIT_REFUSAL`]). The parameters are the schedule's own,
    /// fixed and strictly ascending, so nothing a caller sets reaches it.
    PcurveFit,
    /// The interpolated image could not be re-expressed on the
    /// carrier's own parameter domain: the domain door's refusal,
    /// carried whole. Told apart from [`PcurveFit`](Self::PcurveFit)
    /// because its recourse is the carrier's parameterization, not a
    /// defect report.
    CarrierDomain(CarrierDomainRefusal),
    /// A certificate limb exceeded ε, with its number. **The
    /// declare-and-check refusal**: limb 1 measured the carrier's foot
    /// distance from a surface; limb 2 bounds the carrier's distance from
    /// an analytic surface ([`SsiLimb::HullSup`], [`SsiLimb::HullValue`]),
    /// or reads it through the lane's own image of it on a spline one,
    /// a bound on the carrier's miss and the image's together
    /// ([`SsiLimb::HullSupChart`], [`SsiLimb::HullValueChart`]); at a
    /// break where the carrier's own foot decides Zero, the miss is the
    /// image's ([`SsiLimb::ImageMiss`]).
    Limb {
        /// Which limb refused.
        limb: SsiLimb,
        /// What the classifier saw of the limb's reading, in metres,
        /// for error reporting only
        /// ([`MarginDiag`]): it rides
        /// [`PlaneNurbsRefusal::decision`]'s sign-certain arm to the import
        /// door's words on it.
        margin: MarginDiag,
    },
    /// The uniqueness tube's lower bound on the crossing angle is not
    /// clear of the zero band — a genuine sliver of the operand pair
    /// along the locus at this tolerance (F6: escalate, never guess).
    TubeStraddles {
        /// The verdict on the transversality enclosure's **certified
        /// clearance from zero**, levered — NOT a measurement of how far
        /// the sliver straddles. An enclosure that contains zero has
        /// certified clearance exactly `0.0` by construction (rung 3's
        /// `zero_free_lower_bound`), and containing zero is what this
        /// refusal reports, so the margin is `0.0` on every straddling
        /// refusal and a small positive number only on the levered
        /// rungs that failed the band instead. Read it as the bound the
        /// certificate could prove, never as the geometry's own extent;
        /// the informative companion is `boxes`.
        verdict: Refused,
        /// How many boxes of the tube's chain the clearance above was
        /// certified over — the resolution the verdict was reached at.
        boxes: u32,
    },
    /// The uniqueness tube was a graph at some rung, but at none was its
    /// chain proved to hold one arc spanning the carrier and nothing
    /// else: the declared carrier may join two arcs of the intersection,
    /// or overrun its arc's end.
    TubeNotOneArc {
        /// How many rungs were graphs but not proved one arc.
        rungs: u32,
        /// What the narrowest of them found.
        cause: OneArcRefusal,
    },
    /// The crossing angle at an interior sample escalated: the same
    /// decision as [`NotTransverse`](Self::NotTransverse), undecided, and
    /// told in certification's sentence for it.
    TransversalityEscalated {
        /// The interior sample index.
        sample: u32,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// A limb's margin escalated inside the rung-3 certificate.
    Escalated {
        /// Which limb.
        limb: SsiLimb,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// An arc rim's exact image on a chart did not certify it
    /// ([`ArcRimRefusal`]), on the plane lane's wall or either chart of
    /// the pair lane.
    ArcRim(ArcRimRefusal),
    /// The reported transversality — the minimum sine over interior
    /// samples that each decided transverse — came out poisoned. No
    /// geometry and no tolerance reaches it, so it is a kernel defect.
    ReportedTransversalityPoisoned(Indeterminate),
    /// The spline face's chart speed along a parameter direction is
    /// zero (the face is constant along it) or has no finite bound (its
    /// net is too large), so no length in metres crosses into its
    /// parameters. A fact of the face, refused at the lane's door.
    ChartSpeed(ChartSpeedRefusal),
    /// Limb 2 has no certified net of the declared carrier over a span
    /// of its interval: the carrier-enclosure door's refusal, naming
    /// the carrier's kind and the span.
    CarrierEnclosure(crate::CarrierEnclosureRefusal),
    /// The (carrier, operand) shape is outside the lane's certified
    /// inventory, named exactly. A routing boundary (C12.1), never a
    /// runtime fallback.
    Unsupported {
        /// The refused class, named.
        what: &'static str,
    },
}

/// **Why an arc rim's exact image on a spline chart did not certify it**,
/// on either lane that reads one ([`plane_nurbs_incidence`],
/// [`nurbs_pair_limbs`]), by the one policy both follow:
///
/// - an exact image that escalates, at the pick or in its certificate,
///   refuses with that escalation, and the fitted image does not make it
///   definite (D4 ¶3, the pcurve mint's rule too);
/// - one that refuses definitely, or a rim no candidate holds, reads the
///   chart's fitted image, and where that image's certificate refuses at
///   a limb with its bound, both causes are carried. That bound is an
///   upper bound on the miss, which the fitted image's bent foot path
///   inflates on an arc, never the miss itself. A fitted path that
///   refused before bounding refuses as itself.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(
    feature = "test-support",
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(ArcRimRefusalKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum ArcRimRefusal {
    /// The exact image refused definitely and the fitted image's
    /// certificate refused at a limb.
    Refused {
        /// The check of the exact image's certificate that refused, `None`
        /// where no image was in hand (no candidate held the carrier at
        /// the pick's probe: a rim off the chart, or a partial one whose
        /// full-row images part from it there) or the refusal was no
        /// check's.
        iso: Option<crate::pcurve_cache::PcurveCheck>,
        /// The fitted path's refusing limb.
        limb: SsiLimb,
        /// Its bound.
        bound: MarginDiag,
    },
    /// The exact image escalated, at the pick ([`pick_arc_rim`], its map
    /// residual at the probe) or in its certificate.
    Escalated {
        /// The exact image's check that escalated.
        check: crate::pcurve_cache::PcurveCheck,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
}

impl ArcRimRefusal {
    /// The decision this refusal is a refused arm of. A definite refusal
    /// reads as the exact image's check where that names an incidence
    /// limb ([`crate::pcurve_cache::PcurveCheck::incidence_limb`]), its
    /// certificate carrying no margin past its verdict, and as the fitted
    /// limb with its bound otherwise; an escalation reads as the exact
    /// image's check, undecided.
    #[must_use]
    pub fn decision(&self) -> (CertCheck, RefusedArm<'_>) {
        match self {
            Self::Refused { iso, limb, bound } => {
                match iso.and_then(crate::pcurve_cache::PcurveCheck::incidence_limb) {
                    Some(exact) => (exact.check(), RefusedArm::SignCertain(None)),
                    None => (limb.check(), RefusedArm::SignCertain(Some(*bound))),
                }
            }
            Self::Escalated { check, cause } => (
                crate::certify::arc_rim_check(*check),
                RefusedArm::Undecided(cause),
            ),
        }
    }
}

impl core::fmt::Display for ArcRimRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Refused { iso, limb, bound } => write!(
                f,
                "an arc rim's incidence on its spline wall did not certify: {}, and the check \
                 through its fitted image refused too — {}",
                exact_arc_rim_words(*iso),
                PlaneNurbsRefusal::Limb {
                    limb: *limb,
                    margin: *bound
                }
            ),
            Self::Escalated { check, cause } => write!(
                f,
                "an arc rim's exact image along a boundary row of its wall escalated at its \
                 {check:?} check: {}",
                cause.payload()
            ),
        }
    }
}

/// What an arc rim's exact image said, in [`ArcRimRefusal::Refused`]'s
/// words.
fn exact_arc_rim_words(iso: Option<crate::pcurve_cache::PcurveCheck>) -> &'static str {
    use crate::pcurve_cache::PcurveCheck as C;
    match iso {
        None => "no image along a boundary row of its wall holds it",
        Some(C::MapResidual) => {
            "its exact image along a boundary row of its wall misses it by more than the \
             tolerance at a sample"
        }
        Some(C::Envelope | C::EnvelopeTerm(_)) => {
            "the bound on its exact boundary-row image's miss between samples exceeds the \
             tolerance"
        }
        Some(C::ParamSpan) => concat!(
            "its exact image along a boundary row of its wall does not run forward over a clear ",
            crate::certify::length_noun!()
        ),
        Some(
            C::ChartWinding
            | C::ChartClass
            | C::GeneralLane
            | C::ImageIdentity
            | C::AzimuthPeriod
            | C::TubePeriod
            | C::ChartIncidence
            | C::Sector,
        ) => "its exact boundary-row image's certificate refused it",
    }
}

/// A chart's fitted-image refusal `fitted`, after its exact image refused
/// definitely where `exact` is `Some` ([`ArcRimRefusal`]'s policy): a
/// limb's refusal carries both causes; any other stands as itself.
fn after_exact(
    fitted: PlaneNurbsRefusal,
    exact: Option<Option<crate::pcurve_cache::PcurveCheck>>,
) -> PlaneNurbsRefusal {
    match (fitted, exact) {
        (PlaneNurbsRefusal::Limb { limb, margin }, Some(iso)) => {
            PlaneNurbsRefusal::ArcRim(ArcRimRefusal::Refused {
                iso,
                limb,
                bound: margin,
            })
        }
        (fitted, _) => fitted,
    }
}

/// What [`PlaneNurbsRefusal::PcurveFit`] states, and the pcurve mint's
/// refusal for the same failure of the same producer
/// ([`crate::FittedLane::general_image`]).
pub(crate) const PCURVE_FIT_REFUSAL: &str = "the chart image could not be interpolated through the schedule's foot points: on the \
     schedule's fixed, strictly ascending parameters the interpolation refuses only a foot it \
     cannot take (a non-finite one) or a collocation system it cannot solve";

/// The one recourse for a carrier whose parameter domain its chart
/// image cannot be expressed on ([`CarrierDomainRefusal`]), read by the
/// lane's refusal and by `topo`'s at-rest classifier alike.
pub const CARRIER_DOMAIN_RECOURSE: &str = "Recourse: give the curve a parameter range of moderate \
     width near zero, such as 0 to 1; an affine reparameterization moves no point of it";

/// The chart image could not be re-expressed on the carrier's own
/// parameter domain `[lo, hi]`: the domain door
/// (`NurbsCurve2::on_domain`) refused, and its typed reason rides here.
///
/// The image lives on the carrier's parameter by construction (the OQ4
/// identity, module docs), so no image of this carrier can be expressed
/// on a domain the door refuses. An affine reparameterization of the
/// carrier moves no point of it and gives the door a domain it accepts,
/// which is why that is the recourse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CarrierDomainRefusal {
    /// The carrier's domain start.
    pub lo: f64,
    /// The carrier's domain end.
    pub hi: f64,
    /// Which of the door's two refusals.
    pub fault: CarrierDomainFault,
}

/// The domain door's refusal of a carrier domain, in the two shapes it
/// takes on a validated image: `SplineError`'s other arms are about
/// weights and counts, which the door carries over verbatim.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CarrierDomainFault {
    /// [`SplineError::DomainInvalid`]: the interval is not a finite
    /// increasing interval of finite width — for a validated carrier,
    /// a width that overflows.
    Interval,
    /// [`SplineError::KnotRunsNotIncreasing`]: rounding put two of the
    /// image's breakpoints on one value on a domain too narrow for the
    /// magnitude of its ends, and the held knots refuse to merge them
    /// into one run nobody wrote.
    Merge,
}

impl CarrierDomainRefusal {
    /// The domain door's refusal on `[lo, hi]`, typed.
    ///
    /// # Panics
    ///
    /// On any other arm of [`SplineError`], which the door never returns
    /// on an `f64` image (the door's one caller): it carries the
    /// validated net and weights over verbatim, its re-expressed
    /// breakpoints are `f64` points, and `Knots::new` refuses any two
    /// that collide before a clamp clause could.
    fn of(lo: f64, hi: f64, e: &SplineError) -> Self {
        let fault = match e {
            SplineError::DomainInvalid { .. } => CarrierDomainFault::Interval,
            SplineError::KnotVectorInvalid { .. }
            | SplineError::NonPositiveWeight { .. }
            | SplineError::NonFiniteWeight { .. }
            | SplineError::ControlCountMismatch { .. }
            | SplineError::WeightCountMismatch { .. }
            | SplineError::CarrierWeightNotAPoint { .. }
            | SplineError::WeightSignUndecided { .. }
            | SplineError::KnotIsPoison { .. }
            | SplineError::KnotRunsUndecided { .. }
            | SplineError::CarrierKnotNotAPoint { .. } => unreachable!(
                "the domain door re-expresses a validated f64 image's knots, so it cannot \
                 refuse a weight, a count, a poison or undecided breakpoint or a clamp \
                 clause: {e:?}"
            ),
            // Two breakpoints the re-expression rounds onto one value:
            // the held knots refuse to merge them (the `f64` vector's
            // door merged them into a higher multiplicity).
            SplineError::KnotRunsNotIncreasing { .. } => CarrierDomainFault::Merge,
        };
        Self { lo, hi, fault }
    }
}

impl core::fmt::Display for CarrierDomainRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (lo, hi) = (Readable(self.lo), Readable(self.hi));
        match self.fault {
            CarrierDomainFault::Interval => write!(
                f,
                "the carrier's parameter domain [{lo}, {hi}] is not a finite increasing interval \
                 of finite width"
            )?,
            CarrierDomainFault::Merge => write!(
                f,
                "f64 rounding collapsed two of the chart image's knots onto one value on the \
                 carrier's parameter domain [{lo}, {hi}], which is too narrow for the magnitude \
                 of its ends"
            )?,
        }
        write!(
            f,
            ", so the chart image, which lives on the carrier's own parameter, cannot be \
             expressed on it. {CARRIER_DOMAIN_RECOURSE}"
        )
    }
}

impl PlaneNurbsRefusal {
    /// The ending this refusal's decision gives it, read at `at`
    /// ([`recourse`]), or `None` for a refusal that is no decision's
    /// refused arm, but for the pcurve fit's, which ends in the door's
    /// defect ending ([`ReadAt::culprit`]). `Display` renders the payload
    /// alone, as [`crate::CertifyError`]'s does, and the door appends
    /// this.
    ///
    /// The per-sample transversality and the uniqueness tube are the
    /// `Transversality` decision, whose band-decided arms end alike (D4
    /// ¶1 (iv)); a certificate limb's refusal, definite or escalated,
    /// ends by its limb's decision ([`SsiLimb::check`]).
    #[must_use]
    pub fn ending(&self, at: impl Into<ReadAt>) -> Option<String> {
        match self {
            Self::TubeNotOneArc { cause, .. } => {
                return Some(cause.ending(crate::ssi::OneArcDoor::AtRest, at));
            }
            Self::Escalated { limb, cause } => {
                return Some(crate::ssi::SsiDecision::Limb(*limb).ending(cause, at));
            }
            // No decision's refused arm: the producer refused, so it ends
            // in the defect ending naming the door's culprit.
            Self::PcurveFit => return Some(crate::recourse::defect_ending(at).to_owned()),
            _ => {}
        }
        self.decision().map(|(check, arm)| recourse(check, arm, at))
    }

    /// The decision this refusal is a refused arm of, and which arm
    /// ([`crate::CertifyError::decision`]'s structure).
    #[must_use]
    pub fn decision(&self) -> Option<(CertCheck, RefusedArm<'_>)> {
        Some(match self {
            Self::NotTransverse { verdict, .. } => (CertCheck::Transversality, verdict.arm()),
            Self::TransversalityEscalated { cause, .. } => {
                (CertCheck::Transversality, RefusedArm::Undecided(cause))
            }
            Self::Limb { limb, margin, .. } => {
                (limb.check(), RefusedArm::SignCertain(Some(*margin)))
            }
            // The tube's margin is a lower bound on the lane's
            // transversality over the chain (`ssi_tube_transversality`),
            // and this refusal is its decided verdict.
            Self::TubeStraddles { verdict, .. } => (CertCheck::TransversalityTube, verdict.arm()),
            Self::Escalated { limb, cause } => (limb.check(), RefusedArm::Undecided(cause)),
            Self::ArcRim(refusal) => return Some(refusal.decision()),
            Self::ReportedTransversalityPoisoned(cause) => (
                CertCheck::PlaneNurbsReportedTransversality,
                RefusedArm::Undecided(cause),
            ),
            Self::ChartSpeed(r) => (r.check(), RefusedArm::SignCertain(None)),
            // The one-arc proof is the SSI door's own decision, and
            // `ending` reads it there.
            Self::FootPointInconclusive { .. }
            | Self::TubeNotOneArc { .. }
            | Self::PcurveFit
            | Self::CarrierDomain(_)
            | Self::CarrierEnclosure(_)
            | Self::Unsupported { .. } => {
                return None;
            }
        })
    }
}

impl core::fmt::Display for PlaneNurbsRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FootPointInconclusive {
                sample,
                last_distance,
            } => write!(
                f,
                "the foot-point projection did not converge at schedule sample {sample} \
                 (last distance {last_distance:e} m)"
            ),
            // One decision, one sentence: certification's, which this
            // refusal lifts into ([`crate::certify::CertifyError::NotTransverse`]).
            Self::NotTransverse {
                sample,
                lever,
                verdict,
            } => core::fmt::Display::fmt(
                &crate::certify::CertifyError::NotTransverse {
                    sample: *sample,
                    lever: Some(*lever),
                    verdict: *verdict,
                },
                f,
            ),
            Self::TransversalityEscalated { sample, cause } => core::fmt::Display::fmt(
                &crate::certify::CertifyError::Escalated {
                    check: CertCheck::Transversality,
                    sample: *sample,
                    cause: *cause,
                },
                f,
            ),
            Self::PcurveFit => f.write_str(PCURVE_FIT_REFUSAL),
            Self::CarrierDomain(refusal) => write!(f, "{refusal}"),
            Self::Limb { limb, margin } => match limb {
                SsiLimb::HullSup => write!(
                    f,
                    "{} bounds the declared carrier's distance from an analytic surface by \
                     {margin:e} m, past the run tolerance — the certificate cannot show the \
                     carrier stays on that surface between the schedule's samples",
                    limb.name()
                ),
                SsiLimb::HullSupChart => write!(
                    f,
                    "{} bounds the declared carrier's distance from the spline surface, read \
                     through the certificate's own image of the carrier on it, by {margin:e} \
                     m, past the run tolerance — limb 1 found the carrier on that surface at \
                     every schedule sample, and the certificate cannot show it stays on it \
                     between them",
                    limb.name()
                ),
                SsiLimb::HullValueChart => write!(
                    f,
                    "{} reads {margin:e} m at a break, past the run tolerance: the declared \
                     carrier against the surface point the certificate's own image of it \
                     names there, which bounds the carrier's miss and is not the miss — the \
                     certificate cannot show the carrier is on that surface",
                    limb.name()
                ),
                SsiLimb::ImageMiss => write!(
                    f,
                    "{} reads {margin:e} m at a break, past the run tolerance, where the \
                     declared carrier is on the surface — the certificate's own image of the \
                     carrier misses the carrier's foot path there, and the carrier is not \
                     shown off its surfaces",
                    limb.name()
                ),
                SsiLimb::OnLocus | SsiLimb::HullValue => write!(
                    f,
                    "{} measured the declared carrier {margin:e} m from a surface, past the run \
                     tolerance — the carrier is not on both surfaces",
                    limb.name()
                ),
                // No producer writes a `Limb` of limb 3: the tube refuses
                // as `TubeStraddles` or `TubeNotOneArc`, or escalates.
                SsiLimb::Tube => write!(
                    f,
                    "{} measured {margin:e} m against the run tolerance — the certificate \
                     cannot show the carrier is its surfaces' one crossing",
                    limb.name()
                ),
            },
            Self::TubeStraddles { verdict, boxes } => write!(
                f,
                "{} is not clear of the zero band over {boxes} boxes of the chain — a sliver of \
                 the surface pair along the locus at this tolerance; the bound reads {:e} m, \
                 which is what the certificate could prove and not the sliver's own extent",
                crate::certify::TUBE_CROSSING_BOUND,
                verdict.margin()
            ),
            Self::Escalated { limb, cause } => {
                write!(f, "{} escalated: {}", limb.name(), cause.payload())
            }
            Self::ArcRim(refusal) => write!(f, "{refusal}"),
            Self::TubeNotOneArc { rungs, cause } => write!(
                f,
                "the tube was not proved to hold one arc spanning the carrier at {rungs} rungs; \
                 at the narrowest, {cause}"
            ),
            Self::ReportedTransversalityPoisoned(cause) => write!(
                f,
                "every interior sample decided the plane and the NURBS wall cross, yet \
                 their reported minimum crossing angle is unreadable: {}",
                cause.payload()
            ),
            Self::ChartSpeed(r) => f.write_str(r.what()),
            // The door is read only by limb 2's composite (against the
            // plane and against the wall alike), before any bound
            // exists, so the refusal names the limb and no bound.
            Self::CarrierEnclosure(r) => write!(
                f,
                "limb 2 could not enclose the declared carrier between samples: {r}"
            ),
            Self::Unsupported { what } => {
                write!(f, "outside the spline Intersection lane: {what}")
            }
        }
    }
}

/// **The certified plane × NURBS derivation** — the whole of what a
/// scalar with certification rights can prove about this edge class,
/// and the function the doors inject ([`crate::certify::NurbsLane`]).
///
/// The operand ORDER is load-bearing exactly as it is in the fitted
/// pcurve lane: the NURBS wall is operand **b**, because
/// `certify_branch` reads the chart image of `b`, and the image this
/// lane derives is the carrier's foot path on the wall.
///
/// The image is re-derived here on a fixed schedule of foot points, not
/// read from the pcurve the trace fitted, so on a carrier of many spans
/// its limb-2 bound is this lane's own: it need not reproduce the SSI
/// door's certificate bit for bit. The door that re-certifies the
/// trace's own triple is [`crate::ssi::certify_rung3`].
///
/// # Errors
///
/// [`PlaneNurbsRefusal`], carrying the measured bound whenever one
/// exists. Every refusal here is about the GEOMETRY: a scalar that may
/// not certify does not receive one, because it cannot name this
/// function. The bound is the split, and a dual fails it on
/// [`geom_core::CertifiedEnclosure`] — it has carried
/// [`geom_core::Bounds`] since D1 (2026-08-19) and that is not the
/// right to mint a C9 certification bound:
///
/// ```compile_fail,E0277
/// use geom_core::{Band, Dual64};
/// use geom::{Curve3, NurbsSurface, Surface};
/// fn certified(
///     carrier: &Curve3<Dual64>,
///     params: (Dual64, Dual64),
///     plane: &Surface<Dual64>,
///     wall: &NurbsSurface<Dual64>,
///     extent: Dual64,
///     band: Band,
/// ) {
///     let _ = geom_brep::plane_nurbs_limbs(carrier, params, plane, wall, extent, band);
/// }
/// ```
///
/// The row above fails on the BOUND and not on a path or a spelling,
/// and the row below is what says so: it differs in the scalar alone,
/// every name resolving the same way.
///
/// ```
/// use geom_core::Band;
/// use geom::{Curve3, NurbsSurface, Surface};
/// fn certified(
///     carrier: &Curve3<f64>,
///     params: (f64, f64),
///     plane: &Surface<f64>,
///     wall: &NurbsSurface<f64>,
///     extent: f64,
///     band: Band,
/// ) {
///     let _ = geom_brep::plane_nurbs_limbs(carrier, params, plane, wall, extent, band);
/// }
/// ```
///
/// The symbolic tier is admitted by the same bound and needs no arm of
/// its own, which is the whole of what a per-scalar impl would have
/// said here:
///
/// ```
/// use geom_core::{Band, Sym};
/// use geom::{Curve3, NurbsSurface, Surface};
/// fn symbolic(
///     carrier: &Curve3<Sym<f64>>,
///     params: (Sym<f64>, Sym<f64>),
///     plane: &Surface<Sym<f64>>,
///     wall: &NurbsSurface<Sym<f64>>,
///     extent: Sym<f64>,
///     band: Band,
/// ) {
///     let _ = geom_brep::plane_nurbs_limbs(carrier, params, plane, wall, extent, band);
/// }
/// ```
pub fn plane_nurbs_limbs<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    plane: &Surface<T>,
    wall: &NurbsSurface<T>,
    extent: T,
    band: Band,
) -> Result<PlaneNurbsLimbs<T>, PlaneNurbsRefusal> {
    let normal = plane_lane_door(plane, wall)?;
    // The wall the tube localizes on, with its chart speeds minted at the
    // door: a wall constant along an axis, or with no finite speed bound
    // along one, has no chart a length in metres can cross into, and the
    // schedule below would only meet that as a foot point or a sine that
    // cannot be stated.
    let grid = wall_grid(wall);
    let wall_op = match &grid {
        Some(g) => SsiOperand::nurbs_on(wall, g),
        None => SsiOperand::nurbs(wall),
    }
    .map_err(refusal)?;

    // ---- The fixed schedule: foot points, and the normal angle. ----
    // The feet and the image are `chart_image`'s, which is also the
    // pcurve mint's producer: ONE derivation of this image exists in
    // the tree, and this lane certifies the same bits the mint stores.
    // The transversality sweep rides the same walk as a per-sample
    // hook, so the ORDER in which the two refusals can fire is exactly
    // what it was when the two loops were one.
    let mut min_sin = T::from_f64(f64::MAX);
    let eval = |t| carrier.eval(t);
    let (domain, ends) = crate::carrier_net::carrier_span(carrier, params);
    let pcurve = chart_image(Schedule::new(&eval, domain, ends), wall, |i, foot| {
        // Transversality at the INTERIOR samples only: an endpoint is
        // shared with the neighbouring edges, where a vanishing angle
        // is a vertex fact rather than this edge's — the analytic
        // `Intersection` arm's own convention, kept verbatim.
        if i == 0 || i == PXN_FIT_SAMPLES - 1 {
            return Ok(());
        }
        let (u, v) = measured_foot(foot);
        let jet = wall.ders(u, v);
        let sin_theta = normal_angle_sine(normal, jet.du.cross(jet.dv));
        // The point arm, the plane's curvature being zero.
        let kappa = crate::shape_operator::max_principal_curvature(&jet);
        let (arm, reach) = crate::ssi::point_arm(kappa, extent);
        let margin = geom_core::Margin::levered(sin_theta, arm);
        match crate::dihedral::decide_reported("plane_nurbs_transversality", margin, band) {
            Ok(decided) => {
                if let Some(verdict) = Refused::of(decided, band) {
                    return Err(PlaneNurbsRefusal::NotTransverse {
                        sample: i,
                        lever: PointLever::of(Bounds::lo(reach)),
                        verdict,
                    });
                }
            }
            Err(cause) => {
                return Err(PlaneNurbsRefusal::TransversalityEscalated { sample: i, cause });
            }
        }
        // `Real::min` PROPAGATES poison (unlike `f64::min`, which
        // returns the non-NaN operand), so a poisoned sine cannot
        // be dropped out of this fold — it reaches the guard below.
        min_sin = min_sin.min(sin_theta);
        Ok(())
    })?;
    // FAIL LOUD on a poisoned aggregate. A NaN sine cannot reach here
    // today — the per-sample `decide` above escalates on its levered
    // margin first — but the reported transversality must not depend on
    // that shield holding across future edits to the gate. A poison
    // that ever survives the fold refuses TYPED, carrying the
    // `Invalid` diagnostic, instead of riding out as a reported number
    // no caller can tell from a measurement.
    let min_sin =
        geom_core::k_stats::gate_measured("plane_nurbs_transversality_reported", min_sin, band)
            .map_err(PlaneNurbsRefusal::ReportedTransversalityPoisoned)?;

    // ---- The rung-3 door: all three limbs, both operands. ----
    let cert = crate::ssi::certify::certify_branch(
        carrier,
        params,
        crate::ssi::certify::Lane::AtRest {
            a: &SsiOperand::Analytic(plane),
            b: &wall_op,
            pcurve_a: None,
            pcurve_b: Some(&pcurve),
        },
        extent,
        band,
        crate::ssi::certify::Limbs::All,
        &mut crate::ssi::certify::Refused::default(),
    )
    .map_err(refusal)?;
    Ok(PlaneNurbsLimbs {
        on_locus_max: cert.on_locus_max,
        hull_sup: cert.hull_sup,
        tube: cert.tube,
        tube_transversality: cert.tube_transversality,
        tube_boxes: cert.tube_boxes,
        min_sin_theta: min_sin,
    })
}

/// The plane × NURBS lanes' door: the analytic operand a plane, whose
/// normal it returns, and the wall a described surface.
fn plane_lane_door<T: Real>(
    plane: &Surface<T>,
    wall: &NurbsSurface<T>,
) -> Result<Vec3<T>, PlaneNurbsRefusal> {
    let Surface::Plane { normal, .. } = *plane else {
        return Err(PlaneNurbsRefusal::Unsupported {
            what: "the analytic operand of this lane is a PLANE; another kind needs its own \
                   chart uniqueness form",
        });
    };
    if wall.is_placeholder() {
        return Err(PlaneNurbsRefusal::Unsupported {
            what: geom::PLACEHOLDER_SURFACE,
        });
    }
    Ok(normal)
}

/// What [`plane_nurbs_incidence`] certified: limbs 1 and 2 over both
/// operands, as [`PlaneNurbsLimbs`] reports them.
#[derive(Clone, Copy, Debug)]
pub struct PlaneNurbsIncidence<T: Real> {
    /// Limb 1: the largest on-locus residual over the schedule, over
    /// both operands (the sampled max: it steers, it does not certify).
    pub on_locus_max: T,
    /// Limb 2: the certified sup-norm distance over the whole span, over
    /// both operands. This is the number that certifies.
    pub hull_sup: T,
}

/// The arc-rim images a circle carrier may have on `chart`: each of its
/// two boundary rows, in each `u` direction, as [`Pcurve::IsoArc`] from
/// the carrier's `t0` through `span`, its sub-arcs one per chart span
/// pair. The CLOSED-FORM step of [`pick_arc_rim`]: each image is the
/// edge's whole interval onto the chart's whole `u` domain, which is
/// what a rim the kernel built is. [`arc_rim_measured`] is the step for
/// a rim that covers part of the row.
///
/// The chart's sub-arcs are read off its interior `u` knot COUNT, two
/// per sub-arc joint (a rational-quadratic circle's double knots), not
/// off their multiplicities: a chart of another structure gets
/// candidates whose sub-arcs disagree with its row's, which the
/// arc-rim certificate's own knot check refuses. Empty only where the
/// breaks will not form a knot vector, which ascending breaks strictly
/// inside `(0, 1)` always do; an empty list holds nothing, and reads as
/// no candidate.
#[must_use]
pub fn arc_rim_candidates<T: Real>(chart: &NurbsSurface<T>, t0: T, span: T) -> Vec<Pcurve<T>> {
    arc_rim_images(chart, |_, _| Some((t0, span)))
}

/// **The arc-rim images whose arc the row itself measures**: for each
/// boundary row and `u` direction, the WHOLE row's image, its arc read
/// off the row and the carrier circle rather than off the edge's
/// interval. The image is the row's map `t ↦ u` over the row's own arc
/// `[at0, at0 + angle]`, so an edge covering part of the row (a child
/// of a split rim, either half or any sub-interval) evaluates on its
/// own span inside that arc, and a whole rim's arc is its interval.
///
/// - `angle` is the turn the row's interior weight states for its `m`
///   sub-arcs (`arc_rim_turn`, the inverse of the certificate's
///   `w = cos(h/2)`); a row whose weight is not decided below 1 (a
///   polynomial row) states no turn and offers no image;
/// - `at0` is the carrier's angle at the image's start corner (the
///   row's first control point, or its last for the image against `u`),
///   an [`Azimuth`](crate::pcurve_cache::Azimuth) read in the carrier's
///   frame and moved by whole turns onto the branch nearest the middle
///   of the arcs that could hold the edge, `t0 + (span − angle)/2`,
///   the turns decided ([`Period::decide`](crate::pcurve_cache::Period::decide)).
///   The arcs that hold `[t0, t0 + span]` are less than a period wide,
///   so the branch named is one of them where any is, and the marks the
///   decision reads, half a period from that middle, are never near a
///   holding arc. A gap the decision cannot name offers no image.
///
/// A SELECTION, checked: the arc-rim certificate re-derives the
/// circle's form from `at0` and `angle` against the row's control net
/// and decides that the edge's span lies in the arc. Empty for a carrier
/// that is not a circle.
#[must_use]
pub fn arc_rim_measured<T: Decide>(
    carrier: &Curve3<T>,
    chart: &NurbsSurface<T>,
    (t0, span): (T, T),
    band: Band,
) -> Vec<Pcurve<T>> {
    let Curve3::Circle {
        center,
        axis,
        u_ref,
        ..
    } = *carrier
    else {
        return Vec::new();
    };
    let vref = axis.cross(u_ref);
    let spans = arc_rim_spans(chart);
    let turn = crate::pcurve_cache::Period::new(T::tau());
    arc_rim_images(chart, |end, forward| {
        let row = crate::nurbs_iso::boundary_iso_v(chart, end).ok()?;
        let angle = crate::pcurve_cache::arc_rim_turn(&row, spans, band)?;
        let corner = if forward {
            row.control().first()
        } else {
            row.control().last()
        };
        let d = *corner? - center;
        let read = crate::pcurve_cache::Azimuth::of(d.dot(vref), d.dot(u_ref)).principal(band);
        let middle = t0 + (span - angle) * T::from_f64(0.5);
        let k = turn
            .decide(
                "pcurve_iso_arc_branch",
                middle - read,
                geom_core::Margin::of,
                band,
            )
            .ok()?;
        Some((read + turn.times(k), angle))
    })
}

/// The chart's sub-arc count: one per pair of interior `u` knots, plus
/// one ([`arc_rim_candidates`]' note).
fn arc_rim_spans<T: Real>(chart: &NurbsSurface<T>) -> usize {
    let ku = chart.knots_u_f64_interim();
    let (d0, d1) = ku.domain();
    ku.knots().iter().filter(|k| **k > d0 && **k < d1).count() / 2 + 1
}

/// The four arc-rim images, `v₀` first and with the chart's `u` first
/// in each row, each over the arc `arc(end, forward)` names: the row
/// at `v₁` where `end`, running with `u` where `forward`. An arc that
/// is `None` offers no image.
fn arc_rim_images<T: Real>(
    chart: &NurbsSurface<T>,
    arc: impl Fn(bool, bool) -> Option<(T, T)>,
) -> Vec<Pcurve<T>> {
    let Some(breaks) = uniform_breaks(arc_rim_spans(chart)) else {
        return Vec::new();
    };
    let ((cu0, cu1), (cv0, cv1)) = (
        chart.knot_values_u().domain(),
        chart.knot_values_v().domain(),
    );
    [(false, cv0), (true, cv1)]
        .into_iter()
        .flat_map(|(end, v)| {
            [(true, cu0, cu1 - cu0), (false, cu1, cu0 - cu1)].map(|(forward, p0x, sign)| {
                arc(end, forward).map(|(t0, angle)| Pcurve::IsoArc {
                    p0: Point2::new(p0x, v),
                    pd: geom_core::Vec2::new(sign, T::zero()),
                    t0,
                    angle,
                    breaks: breaks.clone(),
                })
            })
        })
        .flatten()
        .collect()
}

/// **The `k`-th of `m` uniform breakpoints on `[0, 1]`**, `k/m` as one
/// correctly rounded `f64` division: where a circle's sub-arcs join.
///
/// One spelling for every reader of a joint, because the mesher's
/// crease line is only exact if they agree to the bit:
/// `sweep::skin::segment_curve` places its joint knots here, the arc
/// rim's sub-arc locator ([`Pcurve::IsoArc`]'s `breaks`) is built
/// here, and the mesher's chord schedule spaces every edge's points
/// here. A chord schedule of `q·m` points then lands on each joint
/// exactly, since `(q·k)/(q·m)` and `k/m` are the same real, rounded
/// once.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn sub_arc_joint(k: usize, m: usize) -> f64 {
    k as f64 / m as f64
}

/// The uniform clamped degree-1 knot vector on `[0, 1]` with `spans ≥ 1`
/// spans: an [`Pcurve::IsoArc`]'s sub-arc locator (pure `f64`
/// structure, which keeps the variant `T`-generic). `None` only where
/// [`KnotVector::clamped`] refuses, which its ascending breaks never
/// give it.
fn uniform_breaks(spans: usize) -> Option<KnotVector> {
    let mut knots = vec![0.0, 0.0];
    for k in 1..spans {
        knots.push(sub_arc_joint(k, spans));
    }
    knots.extend([1.0, 1.0]);
    KnotVector::clamped(knots, 1).ok()
}

/// **The arc rim's image, picked**: the first of [`arc_rim_candidates`],
/// then of [`arc_rim_measured`], that holds the carrier at the interior
/// probe ([`probe_pick`]). The ONE selection among them, the pcurve
/// mint's (`topo::pcurves`) and the incidence lane's
/// ([`plane_nurbs_incidence`]) alike. The same schedule as the mint's
/// boundary-iso one: the closed form a built rim has, then the map the
/// geometry measures, for a rim that covers part of its row.
///
/// **The row and the `u` direction are one pick.** A rim the kernel
/// built runs with the chart's increasing `u`; one it imported need not
/// (#327: a promoted rim circle's `axis` comes from the file's own
/// winding, and a cylinder wall's two rims routinely wind oppositely).
/// A rim against `u` starts at the `u₁` corner, so neither the start
/// nor a fixed corner can name its row, and on a FULL-PERIOD rim the
/// two `u` boundaries are one 3-D point. So the four images are judged
/// together by the probe, whose point a quarter of the arc in is where
/// the directions are a half-period apart. A SELECTION, checked: the arc-rim certification
/// follows.
///
/// `Ok(None)` where `wall` has no spline chart, or no candidate holds
/// at the probe.
///
/// # Errors
///
/// The first in-band candidate's [`Indeterminate`] where none holds: an
/// undecided pick is the rim's to report, never a definite miss. A
/// closed-form candidate that escalates outranks the measured step,
/// which is then not taken.
pub fn pick_arc_rim<T: Decide>(
    carrier: &Curve3<T>,
    (t0, span): (T, T),
    wall: &Surface<T>,
    band: Band,
) -> Result<Option<Pcurve<T>>, Indeterminate> {
    let Some(chart) = wall.spline_chart() else {
        return Ok(None);
    };
    let pick = |candidates| {
        probe_pick(
            "pcurve_iso_arc_row_direction",
            carrier,
            (t0, span),
            wall,
            band,
            candidates,
        )
    };
    match pick(arc_rim_candidates(chart, t0, span))? {
        Some(image) => Ok(Some(image)),
        None => pick(arc_rim_measured(carrier, chart, (t0, span), band)),
    }
}

/// **The probe**: the first candidate chart image that holds the carrier
/// at both ends of the edge's span and a quarter of it in, metered in
/// METRES against the chart at the image's own points, the largest of
/// the three misses deciding (D4 ¶1). The pcurve mint's iso arms and
/// [`pick_arc_rim`] alike: one judge for every closed-form and measured
/// candidate.
///
/// **Why three points.** The ends alone cannot separate the candidates:
/// on a whole rim every closed-form image passes through the chart's
/// corners, and on a full-period one the two `u` directions share both
/// ends; a quarter span in is where the rows and directions part. The
/// quarter point alone cannot either: a closed-form image maps the edge's
/// interval onto the chart's whole domain, and for each sub-span there is
/// a start at which that wrong map crosses the carrier exactly a quarter
/// in. The ends rule that out: a closed-form image puts its ends on
/// domain ends, so it holds an edge's ends only where they ARE domain
/// ends, the whole-domain edge it is the image of. Among images on one
/// locus whose moving map is affine in the parameter (every iso line),
/// two ends fix the map. A SELECTION, checked: the full certification
/// follows every derivation.
///
/// Escalations are DEFERRED per candidate, so an indeterminate one does
/// not rob the rest of their turn; the first is answered only when no
/// candidate holds.
pub fn probe_pick<T: Decide>(
    what: &'static str,
    carrier: &Curve3<T>,
    (t0, span): (T, T),
    surface: &Surface<T>,
    band: Band,
    candidates: impl IntoIterator<Item = Pcurve<T>>,
) -> Result<Option<Pcurve<T>>, Indeterminate> {
    let at = [t0, t0 + span * T::from_f64(0.25), t0 + span].map(|t| (t, carrier.eval(t)));
    first_within(what, candidates, band, |cand| {
        let miss = |&(t, point): &(T, Point3<T>)| {
            let uv = cand.eval(t);
            point.distance(surface.eval(uv.x, uv.y))
        };
        let [start, quarter, end] = at.each_ref().map(miss);
        geom_core::Margin::of(quarter.max(start).max(end))
    })
}

/// **The first candidate whose margin decides Zero**, in order. A
/// candidate whose margin is in band does not stop the search: its
/// escalation is deferred, and the first deferred one is answered only
/// where no candidate decides Zero. `Ok(None)` where every margin is
/// decided off.
///
/// # Errors
///
/// The first in-band candidate's [`Indeterminate`], where none decides
/// Zero.
pub fn first_within<C, T: Decide>(
    name: &'static str,
    candidates: impl IntoIterator<Item = C>,
    band: Band,
    margin: impl Fn(&C) -> geom_core::Margin<T>,
) -> Result<Option<C>, Indeterminate> {
    let mut deferred = None;
    for cand in candidates {
        match geom_core::k_stats::decide(name, margin(&cand), band) {
            Ok(Sign::Zero) => return Ok(Some(cand)),
            Ok(Sign::Positive | Sign::Negative) => {}
            Err(cause) => {
                deferred.get_or_insert(cause);
            }
        }
    }
    deferred.map_or(Ok(None), Err)
}

/// **Incidence alone on a plane × NURBS pair**: [`plane_nurbs_limbs`]'
/// limbs 1 and 2 and nothing about how the two surfaces meet — no
/// normal-angle sweep and no uniqueness tube. It is `OnPair`'s
/// certificate on that pair, which states the carrier lies on both
/// surfaces and leaves the meeting to tier 3.
///
/// The plane side is limbs 1 and 2 against its implicit form. The wall
/// side is metered against the wall's spline chart (`wall` is a
/// described NURBS wall, or an `Approx` one whose chart is its FIT):
///
/// - **A circle carrier** — an arc rim — first through its exact
///   [`Pcurve::IsoArc`] image's certificate ([`PcurveCache::certify`](crate::pcurve_cache::PcurveCache::certify),
///   `MapResidualIsoHull`): limb 1 its sampled residual, limb 2 its
///   envelope. The image is `stated` where the caller holds one (the
///   pcurve the mint already stored for this rim), certified as it
///   stands; otherwise it is [`pick_arc_rim`]'s, the mint's own pick.
///   The certificate compares the wall's whole boundary row with the
///   circle in the row's own spline space, over the image's arc, which
///   it decides holds the edge's span; a rim covering part of the row
///   takes the row over the arc the row measures
///   ([`arc_rim_measured`]). Its one limit is **an `f64` read**: the
///   row's weight pattern is read on the `f64` value channel.
///
///   Where it does not certify, [`ArcRimRefusal`]'s policy decides: an
///   escalation refuses, and a definite refusal reads the fitted path
///   below, a sound certificate of its own.
/// - **Any other carrier**: the carrier's fitted chart image
///   ([`chart_image`], from the same fixed schedule of foot points as
///   the full lane's) and limbs 1 and 2 over it.
///
/// Incidence on an `Approx` wall's description is this bound plus the
/// fit's own recorded distance from it, which the surface carries
/// ([`geom::ApproxSurface`]); nothing here adds the two
/// (`work/tangtail/an-approx-walls-edge-residual-is-recorded-against-its-fit.md`).
///
/// # Errors
///
/// [`PlaneNurbsRefusal`]: the analytic side not a plane, a placeholder
/// or non-spline wall, a foot that will not converge, an image that will
/// not fit, and either limb's refusal with its measured bound;
/// [`PlaneNurbsRefusal::ArcRim`] for a circle carrier whose exact image
/// did not certify, by [`ArcRimRefusal`]'s policy.
pub fn plane_nurbs_incidence<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    plane: &Surface<T>,
    wall: &Surface<T>,
    stated: Option<&Pcurve<T>>,
    band: Band,
) -> Result<PlaneNurbsIncidence<T>, PlaneNurbsRefusal> {
    use crate::ssi::certify::IncidenceOperand;
    let Some(chart) = wall.spline_chart() else {
        return Err(PlaneNurbsRefusal::Unsupported {
            what: "the wall of this lane is a spline chart",
        });
    };
    plane_lane_door(plane, chart)?;
    let exact = match carrier {
        Curve3::Circle { .. } => Some(
            exact_arc_rim(carrier, params, wall, stated, band)
                .map_err(PlaneNurbsRefusal::ArcRim)?,
        ),
        _ => None,
    };
    if let Some(Ok(cache)) = &exact {
        let c = cache.certificate();
        let (on_locus, hull) = crate::ssi::certify::incidence_limbs(
            carrier,
            params,
            &[IncidenceOperand::Analytic(plane)],
            band,
            &mut crate::ssi::certify::Refused::default(),
        )
        .map_err(refusal)?;
        return Ok(PlaneNurbsIncidence {
            on_locus_max: on_locus.max(c.max_residual),
            hull_sup: hull.max(c.envelope),
        });
    }
    // The carrier's fitted chart image alone: the wall's grid and chart
    // speeds are the tube's, which incidence does not ask.
    let eval = |t| carrier.eval(t);
    let (domain, ends) = crate::carrier_net::carrier_span(carrier, params);
    let fitted =
        chart_image(Schedule::new(&eval, domain, ends), chart, |_, _| Ok(())).and_then(|pcurve| {
            crate::ssi::certify::incidence_limbs(
                carrier,
                params,
                &[
                    IncidenceOperand::Analytic(plane),
                    IncidenceOperand::Chart(chart, &pcurve),
                ],
                band,
                &mut crate::ssi::certify::Refused::default(),
            )
            .map_err(refusal)
        });
    let (on_locus_max, hull_sup) =
        fitted.map_err(|e| after_exact(e, exact.and_then(Result::err)))?;
    Ok(PlaneNurbsIncidence {
        on_locus_max,
        hull_sup,
    })
}

/// **An arc rim's exact image on a chart, and its certificate**: `stated`
/// as it stands where the caller holds one, otherwise
/// [`pick_arc_rim`]'s. The one home of that reading for every lane
/// that meters a circle carrier against a spline chart
/// ([`plane_nurbs_incidence`], [`nurbs_pair_limbs`]). `Ok(Err)` is a
/// definite refusal: the certificate's refusing check (`None` where the
/// refusal was no check's), or the pick's finding no candidate (`None`).
///
/// # Errors
///
/// [`ArcRimRefusal::Escalated`] where the pick (its map residual at the
/// probe) or the certificate escalated.
#[allow(clippy::type_complexity)]
fn exact_arc_rim<T: Decide>(
    carrier: &Curve3<T>,
    (t0, t1): (T, T),
    wall: &Surface<T>,
    stated: Option<&Pcurve<T>>,
    band: Band,
) -> Result<
    Result<crate::pcurve_cache::PcurveCache<T>, Option<crate::pcurve_cache::PcurveCheck>>,
    ArcRimRefusal,
> {
    use crate::pcurve_cache::{PcurveCache, PcurveCertifyError as E, PcurveCheck};
    let image = match stated {
        Some(image) => image.clone(),
        None => match pick_arc_rim(carrier, (t0, t1 - t0), wall, band) {
            Ok(Some(image)) => image,
            Ok(None) => return Ok(Err(None)),
            Err(cause) => {
                return Err(ArcRimRefusal::Escalated {
                    check: PcurveCheck::MapResidual,
                    cause,
                });
            }
        },
    };
    match PcurveCache::certify(image, t0, t1, carrier, wall, band) {
        Ok(cache) => Ok(Ok(cache)),
        Err(E::Escalated { check, cause, .. }) => Err(ArcRimRefusal::Escalated { check, cause }),
        // No `IsoArc` check runs an SSI decision; were one to, it
        // escalates the envelope it bounds.
        Err(E::FittedEscalated { cause, .. }) => Err(ArcRimRefusal::Escalated {
            check: PcurveCheck::Envelope,
            cause,
        }),
        Err(refused) => Ok(Err(refused.refused_check())),
    }
}

/// **The certified NURBS × NURBS derivation** — [`plane_nurbs_limbs`]
/// for an edge between two spline charts, a described wall or a fit on
/// either side: the carrier's chart image on EACH chart, from the same
/// fixed schedule of foot points ([`chart_image`], the one producer of
/// that image), the normal angle between the two charts at each
/// interior sample's feet decided definitely transverse, and C2's
/// certificate against both, its limb 3 the two-chart tube
/// ([`crate::ssi::SsiTube::TwoChart`]).
///
/// The images are re-derived here, as the plane lane's is, so the
/// limb-2 bound is this lane's own and need not reproduce the section's
/// certificate bit for bit.
///
/// **A circle carrier** — a rim — reads limbs 1 and 2 on each chart
/// through that chart's exact [`Pcurve::IsoArc`] image where one
/// certifies ([`exact_arc_rim`], [`plane_nurbs_incidence`]'s reading,
/// the image picked from the chart's own boundary rows by
/// [`pick_arc_rim`]): its certificate's sampled residual and envelope.
/// Where it does not certify, [`ArcRimRefusal`]'s policy decides, as on
/// the plane lane: an escalation refuses, and a definite refusal reads
/// the chart's fitted image's limbs. Every reading of
/// limbs 1 and 2 bounds the carrier's distance from the chart from
/// above, so an exact image's certified bound and a fitted image's
/// refusal never disagree on which side of the band the carrier lies:
/// the fitted refusal says only that its image cannot show it, and the
/// certified bound is the one used. Limb 3's tube is built about the
/// image each chart's limb 2 read: on an exact chart, the polyline
/// through the exact image at the fitted image's knots
/// ([`exact_chart_image`]), whose windows hold the exact image.
///
/// # Errors
///
/// [`PlaneNurbsRefusal`], whose vocabulary this lane shares: a foot that
/// will not converge, an image that will not fit, a sample whose two
/// charts are not definitely transverse, an exact image that escalated
/// ([`PlaneNurbsRefusal::ArcRim`]), and a limb's refusal with its
/// measured bound.
pub fn nurbs_pair_limbs<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    a: &NurbsSurface<T>,
    b: &NurbsSurface<T>,
    extent: T,
    band: Band,
) -> Result<PlaneNurbsLimbs<T>, PlaneNurbsRefusal> {
    if a.is_placeholder() || b.is_placeholder() {
        return Err(PlaneNurbsRefusal::Unsupported {
            what: geom::PLACEHOLDER_SURFACE,
        });
    }
    // Each chart localized on its own cell grid, as the plane lane's wall.
    let (grid_a, grid_b) = (wall_grid(a), wall_grid(b));
    let op_a = match &grid_a {
        Some(g) => SsiOperand::nurbs_on(a, g),
        None => SsiOperand::nurbs(a),
    }
    .map_err(refusal)?;
    let op_b = match &grid_b {
        Some(g) => SsiOperand::nurbs_on(b, g),
        None => SsiOperand::nurbs(b),
    }
    .map_err(refusal)?;
    let mut feet_a: Vec<Point2<f64>> = Vec::with_capacity(PXN_FIT_SAMPLES as usize);
    let eval = |t| carrier.eval(t);
    let (domain, ends) = crate::carrier_net::carrier_span(carrier, params);
    let pcurve_a = chart_image(Schedule::new(&eval, domain, ends), a, |_, foot| {
        feet_a.push(foot);
        Ok(())
    })?;
    let mut min_sin = T::from_f64(f64::MAX);
    let pcurve_b = chart_image(Schedule::new(&eval, domain, ends), b, |i, foot| {
        // Interior samples only, as the plane lane reads them.
        if i == 0 || i == PXN_FIT_SAMPLES - 1 {
            return Ok(());
        }
        let Some(fa) = feet_a.get(i as usize) else {
            return Err(PlaneNurbsRefusal::PcurveFit);
        };
        let sin_theta = feet_transversality(
            (a, measured_foot(*fa)),
            (b, measured_foot(foot)),
            extent,
            i,
            band,
        )
        .map_err(FeetRefusal::lane)?;
        min_sin = min_sin.min(sin_theta);
        Ok(())
    })?;
    let min_sin =
        geom_core::k_stats::gate_measured("nurbs_pair_transversality_reported", min_sin, band)
            .map_err(PlaneNurbsRefusal::ReportedTransversalityPoisoned)?;
    let (exact_a, exact_b) = (
        exact_chart_image(carrier, params, (a, &pcurve_a), band)?,
        exact_chart_image(carrier, params, (b, &pcurve_b), band)?,
    );
    // Limbs 1 and 2 on each chart: an exact image's certificate where
    // one certified, read as a shared row's certificate reads it, the
    // fitted image's limbs otherwise.
    let (mut on_locus_max, mut hull_sup) = (T::zero(), T::zero());
    for (chart, fitted, exact) in [(a, &pcurve_a, &exact_a), (b, &pcurve_b, &exact_b)] {
        let (on_locus, hull) = match exact {
            Some(Ok((cache, _))) => cache.certificate().limbs(),
            _ => crate::ssi::certify::incidence_limbs(
                carrier,
                params,
                &[crate::ssi::certify::IncidenceOperand::Chart(chart, fitted)],
                band,
                &mut crate::ssi::certify::Refused::default(),
            )
            .map_err(|e| {
                let iso = match exact {
                    Some(Err(iso)) => Some(*iso),
                    _ => None,
                };
                after_exact(refusal(e), iso)
            })?,
        };
        on_locus_max = on_locus_max.max(on_locus);
        hull_sup = hull_sup.max(hull);
    }
    // Limb 3 alone, its tube about the image each chart's limb 2 read.
    // In this witness lane the choice is unguardable: the exact image's
    // polyline and the fitted feet agree to the foot's precision, and no
    // witness through an exact chart reaches `Ok`, since a carrier on a
    // chart's boundary row refuses here
    // (`work/splice/a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary.md`).
    // A rim on both walls' boundary rows is a shared row, certified
    // without this tube.
    #[allow(clippy::type_complexity)]
    fn tube_image<'a, C, I, T: Real>(
        exact: &'a Option<Result<(C, NurbsCurve2<T>), I>>,
        fitted: &'a NurbsCurve2<T>,
    ) -> &'a NurbsCurve2<T> {
        match exact {
            Some(Ok((_, polyline))) => polyline,
            _ => fitted,
        }
    }
    let cert = crate::ssi::certify::certify_branch(
        carrier,
        params,
        crate::ssi::certify::Lane::AtRest {
            a: &op_a,
            b: &op_b,
            pcurve_a: Some(tube_image(&exact_a, &pcurve_a)),
            pcurve_b: Some(tube_image(&exact_b, &pcurve_b)),
        },
        extent,
        band,
        crate::ssi::certify::Limbs::Tube,
        &mut crate::ssi::certify::Refused::default(),
    )
    .map_err(refusal)?;
    Ok(PlaneNurbsLimbs {
        on_locus_max,
        hull_sup,
        tube: cert.tube,
        tube_transversality: cert.tube_transversality,
        tube_boxes: cert.tube_boxes,
        min_sin_theta: min_sin,
    })
}

/// A spline pair's crossing angle refused at a sample, or its reported
/// minimum poisoned: [`PlaneNurbsRefusal`]'s three transversality arms,
/// holding the lever's reach as the scalar it was read at.
pub(crate) enum FeetRefusal<T> {
    /// [`PlaneNurbsRefusal::NotTransverse`].
    NotTransverse {
        sample: u32,
        reach: T,
        verdict: Refused,
    },
    /// [`PlaneNurbsRefusal::TransversalityEscalated`].
    Escalated { sample: u32, cause: Indeterminate },
    /// [`PlaneNurbsRefusal::ReportedTransversalityPoisoned`].
    Poisoned(Indeterminate),
}

impl<T: Bounds> FeetRefusal<T> {
    /// The lane's refusal, its lever named by the reach's bracket.
    pub(crate) fn lane(self) -> PlaneNurbsRefusal {
        match self {
            FeetRefusal::NotTransverse {
                sample,
                reach,
                verdict,
            } => PlaneNurbsRefusal::NotTransverse {
                sample,
                lever: PointLever::of(Bounds::lo(reach)),
                verdict,
            },
            FeetRefusal::Escalated { sample, cause } => {
                PlaneNurbsRefusal::TransversalityEscalated { sample, cause }
            }
            FeetRefusal::Poisoned(cause) => {
                PlaneNurbsRefusal::ReportedTransversalityPoisoned(cause)
            }
        }
    }
}

/// **Two spline charts definitely transverse at sample `i`'s feet**:
/// the sine of the angle between their normals at `(ua, va)` on `a` and
/// `(ub, vb)` on `b`, levered over the larger principal curvature at an
/// edge of `extent`, decided. The sine where it is definite.
fn feet_transversality<T: Decide>(
    (a, (ua, va)): (&NurbsSurface<T>, (T, T)),
    (b, (ub, vb)): (&NurbsSurface<T>, (T, T)),
    extent: T,
    i: u32,
    band: Band,
) -> Result<T, FeetRefusal<T>> {
    let (ja, jb) = (a.ders(ua, va), b.ders(ub, vb));
    let sin_theta = normal_angle_sine(ja.du.cross(ja.dv), jb.du.cross(jb.dv));
    let kappa = crate::shape_operator::max_principal_curvature(&ja)
        .max(crate::shape_operator::max_principal_curvature(&jb));
    let (arm, reach) = crate::ssi::point_arm(kappa, extent);
    let margin = geom_core::Margin::levered(sin_theta, arm);
    match crate::dihedral::decide_reported("nurbs_pair_transversality", margin, band) {
        Ok(decided) => match Refused::of(decided, band) {
            Some(verdict) => Err(FeetRefusal::NotTransverse {
                sample: i,
                reach,
                verdict,
            }),
            None => Ok(sin_theta),
        },
        Err(cause) => Err(FeetRefusal::Escalated { sample: i, cause }),
    }
}

/// **A shared row's two walls, definitely transverse along it**: the
/// reading [`nurbs_pair_limbs`] makes at its interior samples, here at
/// the certification schedule's interior samples and at each wall's own
/// row image (`image_a` on `a`, `image_b` on `b`), the feet the row
/// states rather than ones a projection found. The smallest sine, gated
/// as that lane gates its own.
pub(crate) fn row_transversality<T: Decide>(
    params: (T, T),
    (a, image_a): (&NurbsSurface<T>, &Pcurve<T>),
    (b, image_b): (&NurbsSurface<T>, &Pcurve<T>),
    extent: T,
    band: Band,
) -> Result<T, FeetRefusal<T>> {
    let (t0, t1) = params;
    let mut min_sin = T::from_f64(f64::MAX);
    for i in 1..crate::certify::CERT_SAMPLES - 1 {
        let t = crate::certify::sample_param(t0, t1, i);
        let (fa, fb) = (image_a.eval(t), image_b.eval(t));
        let sin_theta = feet_transversality((a, (fa.x, fa.y)), (b, (fb.x, fb.y)), extent, i, band)?;
        min_sin = min_sin.min(sin_theta);
    }
    geom_core::k_stats::gate_measured("nurbs_pair_transversality_reported", min_sin, band)
        .map_err(FeetRefusal::Poisoned)
}

/// **A circle carrier's exact image on a spline chart** ([`exact_arc_rim`],
/// the chart taken as its wall): `Ok(cache, polyline)` where it
/// certifies, with the polyline limb 3's tube is built about, and the
/// refusing check where it refuses definitely ([`ArcRimRefusal`]'s
/// policy sends the chart to its fitted image). `None` for any other
/// carrier.
///
/// The polyline runs through the image's points at the fitted image's
/// knots, `Q(tₖ) = E(tₖ)`, each read at `T`. The image is the segment
/// `p₀ → p₀ + p_d` traversed monotonically ([`Pcurve::chart_box`]), so
/// over a knot span it lies in the hull of its two ends: inside the
/// span's window, which is what the tube padded about `Q` proves over.
/// The polyline shares the fitted image's knots, so the two charts'
/// windows pair span for span. That reading needs the fitted image to be
/// degree 1 with a knot per control point ([`PXN_IMAGE_DEGREE`]), which is
/// checked.
///
/// The image lies on the chart's own knot rectangle: the pick reads its
/// candidates off the rectangle's boundary rows, and the certificate
/// decides that the edge's span lies in the image's arc. A rim across a
/// closed chart's seam lies on no single arc of a row, so it has no
/// exact image here and reads its fitted one.
///
/// The chart is cloned into a [`Surface`] for the pick and the
/// certificate, which read a surface; the lane holds the chart alone.
///
/// # Errors
///
/// [`ArcRimRefusal::Escalated`] where the pick or the certificate
/// escalated; [`PlaneNurbsRefusal::Unsupported`] where the fitted image
/// is not the degree-1 polyline the tube's reading needs.
#[allow(clippy::type_complexity)]
fn exact_chart_image<T: Decide>(
    carrier: &Curve3<T>,
    params: (T, T),
    (chart, fitted): (&NurbsSurface<T>, &NurbsCurve2<T>),
    band: Band,
) -> Result<
    Option<
        Result<
            (crate::pcurve_cache::PcurveCache<T>, NurbsCurve2<T>),
            Option<crate::pcurve_cache::PcurveCheck>,
        >,
    >,
    PlaneNurbsRefusal,
> {
    if !matches!(carrier, Curve3::Circle { .. }) {
        return Ok(None);
    }
    let wall = Surface::Nurbs(std::sync::Arc::new(chart.clone()));
    let cache = match exact_arc_rim(carrier, params, &wall, None, band) {
        Ok(Ok(cache)) => cache,
        Ok(Err(iso)) => return Ok(Some(Err(iso))),
        Err(e) => return Err(PlaneNurbsRefusal::ArcRim(e)),
    };
    let knots = fitted.knot_values();
    if knots.degree() != 1 || knots.runs().count() != fitted.control().len() {
        return Err(PlaneNurbsRefusal::Unsupported {
            what: EXACT_TUBE_NEEDS_A_POLYLINE,
        });
    }
    let control = knots
        .runs()
        .map(|(t, _)| cache.pcurve().eval(t))
        .collect::<Vec<_>>();
    let weights = vec![T::one(); control.len()];
    let polyline = NurbsCurve2::new(knots.clone(), control, weights).map_err(|_| {
        PlaneNurbsRefusal::Unsupported {
            what: EXACT_TUBE_NEEDS_A_POLYLINE,
        }
    })?;
    Ok(Some(Ok((cache, polyline))))
}

/// [`exact_chart_image`]'s refusal where the fitted image is no degree-1
/// polyline with a knot per control point.
const EXACT_TUBE_NEEDS_A_POLYLINE: &str = "the tube about an exact arc-rim image is the polyline \
     through it at the fitted image's knots, which needs that image to be degree 1 with a knot per \
     control point";

/// Whether `surface` is an analytic operand: one with an implicit form
/// and a chart frame, which the analytic rung-3 certificate reads
/// ([`analytic_rung3`]). A spline operand's pair is the plane × NURBS
/// lane's ([`plane_nurbs_limbs`]).
#[must_use]
pub fn is_analytic<T: Real>(surface: &Surface<T>) -> bool {
    !matches!(surface, Surface::Nurbs(_) | Surface::Approx(_))
}

/// **The between-samples certificate of a rung-3 carrier between two
/// ANALYTIC surfaces** — C2's limbs, the edge certificate's own (C2:
/// "the proof is the same at every door, a search's and an edge's at
/// rest"), stated here for every analytic operand whatever faces store
/// pcurve rows:
///
/// - **limb 2, per operand**: the carrier's distance from the operand,
///   certified over the edge's interval `params`
///   ([`crate::pcurve_cache::limb2_offset_sup`]: the operand's
///   canonical implicit form composed along the carrier, converted to
///   metres per part, the parts halved under limb 2's budget) and
///   decided zero against the band, or, where the halving stops on a
///   break whose certified distance lies past the band, that measured
///   miss decided first ([`Rung3Limb::HullValue`]). Limb 1, the same
///   distance at the schedule's samples, is implied by it;
/// - **limb 3**, owed unless the pair's section states, for the pose,
///   the crossing angle's floor over the carrier's stretch and the
///   clearance of the tube that floor buys from the rest of the two
///   surfaces' ε-overlap ([`crate::intersect::tube_bounds`]): over a
///   chain of boxes around the carrier the enclosure of
///   `(∇f₁ × ∇f₂)·e` excludes zero, so the solution set in the chain is
///   one arc and it spans the carrier over `params`. An operand outside
///   the tube's enclosure roster (`ssi::enclose::tube_encloses`: a kind
///   without an arm, or a torus not shown inside its ring convention)
///   refuses here, by name, before any box is probed.
///
/// Both read the carrier cut to `params` (its bracket's outer ends, so
/// the piece covers the interval at every scalar): the net past the
/// edge's ends is not the edge's (C4), and a split edge keeps its
/// parent's carrier.
///
/// # Errors
///
/// [`AnalyticRung3Refusal`]: a limb's measured refusal or escalation,
/// the tube's verdict, an operand whose distance bound has no
/// certificate, an operand the tube has no enclosure of, a spline operand, or an interval the carrier cannot be
/// cut to.
pub fn analytic_rung3<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    params: (T, T),
    s1: &Surface<T>,
    s2: &Surface<T>,
    band: Band,
) -> Result<(), AnalyticRung3Refusal> {
    if !(is_analytic(s1) && is_analytic(s2)) {
        return Err(AnalyticRung3Refusal::Unsupported {
            what: "the analytic rung-3 certificate reads two analytic operands",
        });
    }
    let (lo, hi) = crate::carrier_net::outer_ends(params);
    let piece = edge_piece(carrier, lo, hi)?;
    let curve = Curve3::Nurbs(std::sync::Arc::new(carrier.clone()));
    for operand in [s1, s2] {
        limb2(
            &curve,
            params,
            operand,
            crate::pcurve_cache::limb2_offset_sup(&curve, params, operand, band),
            None,
            band,
        )?;
    }
    // The tube is owed unless the pair's section states, for this pose,
    // the crossing angle's floor and the tube's clearance (C2).
    if crate::intersect::tube_bounds(carrier, params, s1, s2, band).is_ok() {
        return Ok(());
    }
    if let Some(operand) = [s1, s2]
        .into_iter()
        .find(|s| !crate::ssi::enclose::tube_encloses(s, band))
    {
        return Err(AnalyticRung3Refusal::NoTubeEnclosure {
            operand: operand.kind(),
        });
    }
    let carrier = Curve3::Nurbs(std::sync::Arc::new(piece.clone()));
    let (p0, p1) = piece.knot_values().domain();
    crate::ssi::certify::certify_branch(
        &carrier,
        (p0, p1),
        crate::ssi::certify::Lane::AtRest {
            a: &SsiOperand::Analytic(s1),
            b: &SsiOperand::Analytic(s2),
            pcurve_a: None,
            pcurve_b: None,
        },
        // The tube's extent, from the object being certified, as limb 3
        // reads it everywhere: from below, since its transversality is
        // the definite verdict it acts on.
        crate::edge_extent_inf(&carrier, p0, p1, piece.eval(p0).distance(piece.eval(p1))),
        band,
        crate::ssi::certify::Limbs::Tube,
        &mut crate::ssi::certify::Refused::default(),
    )
    .map(|_| ())
    .map_err(AnalyticRung3Refusal::of_tube)
}

/// **C2's limbs of an `Intersection` carrier of any kind between two
/// ANALYTIC surfaces**, over the edge's interval `params`: limb 2 against
/// each operand for every carrier, and the uniqueness tube for a spline
/// one ([`analytic_rung3`], which is this door for a `Curve3::Nurbs`).
///
/// A line, circle, ellipse or spiric is read through its certified
/// enclosing net ([`crate::pcurve_cache::limb2_offset_sup`]): the
/// operand's canonical implicit form composed along it, converted to
/// metres per part as for a spline, its parts halved under limb 2's
/// budget, and decided as [`analytic_rung3`] decides its limb 2. Limb 1,
/// the same distance at the schedule's samples, is implied by it.
///
/// # Errors
///
/// [`analytic_rung3`]'s for a spline; for any other kind, a limb's
/// measured refusal or escalation, an operand whose distance bound has
/// no certificate, a carrier with no certified enclosing net over a span
/// of the interval ([`AnalyticRung3Refusal::CarrierEnclosure`]), or a
/// spline operand.
pub fn analytic_edge_limbs<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    s1: &Surface<T>,
    s2: &Surface<T>,
    band: Band,
) -> Result<(), AnalyticRung3Refusal> {
    if let Curve3::Nurbs(net) = carrier {
        return analytic_rung3(net, params, s1, s2, band);
    }
    if !(is_analytic(s1) && is_analytic(s2)) {
        return Err(AnalyticRung3Refusal::Unsupported {
            what: "the analytic edge certificate reads two analytic operands",
        });
    }
    for operand in [s1, s2] {
        limb2(
            carrier,
            params,
            operand,
            crate::pcurve_cache::limb2_offset_sup(carrier, params, operand, band),
            line_plane_ends(carrier, params, operand),
            band,
        )?;
    }
    Ok(())
}

/// Limb 2 against `operand`, decided on what the composite read of the
/// carrier's distance from it.
///
/// **At an enclosure scalar** ([`geom_core::Witness::Exact`]) the
/// reading is the enclosure `[floor, upper]` of that distance — `floor`
/// the certified value at the break the rounds stopped on, `0` where
/// they stopped on the bound — decided as one margin
/// ([`limb2_enclosure`]): `upper` within the band passes, a floor past
/// it is the measured miss ([`AnalyticRung3Refusal::Limb`]), and
/// anything else is undecided at this input: a bare upper bound refuses
/// [`AnalyticRung3Refusal::Stalled`], claiming no cause and no
/// terminality, and a floor inside the band escalates on the measured
/// value ([`AnalyticRung3Refusal::Escalated`] on
/// [`Rung3Limb::HullValue`], the classifier's terminal bit kept). At a
/// parameter-family scalar the margin carries the free-parameter
/// residual of the carrier against the operand as its symbolic identity
/// ([`free_residual`]), so a carrier that lies on the operand for every
/// member of the box decides `Zero` there in one leaf.
///
/// **At a point scalar** the reading is an upper bound: a measured miss
/// at a break first ([`Rung3Limb::HullValue`]), then the bound, which
/// clears the band or is the kernel's approximation limit
/// ([`AnalyticRung3Refusal::KernelLimit`]) and never a miss.
///
/// `ends` are a line's distances from a plane at its ends
/// ([`line_plane_ends`]): there the sup is attained at an end, so a
/// reading that neither clears the band nor shows a measured miss
/// defers to those measured values ([`measured_ends`], in certification
/// arithmetic at every scalar): both ends in the band pass, an end past
/// it is a miss, and an end whose enclosure excludes zero but lies in
/// the ambiguity band or straddles its edge escalates on the measured
/// value ([`AnalyticRung3Refusal::Escalated`] on [`Rung3Limb::HullValue`]).
/// An end whose enclosure reaches zero and passes ε is only an upper
/// bound, which says nothing about the input, so the composite's stall
/// stands.
fn limb2<T: Decide>(
    carrier: &Curve3<T>,
    params: (T, T),
    operand: &Surface<T>,
    read: Result<crate::pcurve_cache::projected::OffsetRead<T>, crate::PcurveCertifyError>,
    ends: Option<[(geom_core::Interval, geom_core::Interval); 2]>,
    band: Band,
) -> Result<(), AnalyticRung3Refusal> {
    use crate::pcurve_cache::projected::{Offset, OffsetRead};
    let kind = operand.kind();
    let refused = |e| AnalyticRung3Refusal::of_offset(kind, e);
    let OffsetRead {
        offset,
        size,
        upper,
        floor,
    } = read.map_err(refused)?;
    let decided = match (T::WITNESS, offset) {
        (geom_core::Witness::Exact, Offset::Bound(_) | Offset::Miss { bound: Ok(_), .. }) => {
            limb2_enclosure(kind, floor, upper, band, |free| {
                free_residual(carrier, params, operand, free)
            })
        }
        (_, Offset::Bound(bound)) => upper_bound(kind, bound, upper, size, band),
        (_, Offset::Miss { floor, bound }) => measured_miss(kind, floor, band)
            .and_then(|()| upper_bound(kind, bound.map_err(refused)?, upper, size, band)),
    };
    match (decided, ends) {
        (Err(AnalyticRung3Refusal::Limb { .. }), _) | (_, None) | (Ok(()), _) => decided,
        // The sup is at an end, so the ends decide where they measure:
        // both in the band certify, one past it is a miss, and one whose
        // enclosure excludes zero but lies in the ambiguity band (or
        // straddles its edge) is the measured in-band escalation. An end
        // whose enclosure reaches zero is only an upper bound, which says
        // nothing about the input: it adds nothing to the composite's
        // stall, which stands — the kernel's limit at a point scalar, the
        // undecided enclosure at an enclosure one.
        (Err(stall), Some(ends)) => measured_ends(kind, ends, band).unwrap_or(Err(stall)),
    }
}

/// **Limb 2 at an enclosure scalar** ([`limb2`]): the carrier's
/// distance from the operand enclosed by `[floor, upper]` in the
/// scalar's own backend (C9) and decided as one margin, its symbolic
/// identity the residual `identity` builds over a free parameter
/// ([`Decide::free_identity`]). The classifier's verdict is the limb's:
/// within the band it passes; a floor past the band is a measured miss
/// ([`AnalyticRung3Refusal::Limb`]); anything else is undecided at this
/// input. A bare upper bound (`floor` 0) refuses
/// [`AnalyticRung3Refusal::Stalled`] with the classifier's diagnostic,
/// whose terminal bit is false (the enclosure reaches down to zero), and
/// claims no cause: over a parameter box a smaller box is the remedy
/// (E6), and the drive bisects it on the escalation the funnel logged. A
/// floor inside the band escalates on the measured value
/// ([`AnalyticRung3Refusal::Escalated`] on [`Rung3Limb::HullValue`]),
/// the classifier's terminal bit kept: a distance measured in the
/// sliver is one no subdivision decides.
fn limb2_enclosure<T: Decide>(
    operand: geom::SurfaceKind,
    floor: f64,
    upper: f64,
    band: Band,
    identity: impl FnOnce(&dyn Fn(T) -> T) -> T,
) -> Result<(), AnalyticRung3Refusal> {
    let (predicate, limb) = if floor > 0.0 {
        ("ssi_hull_value", Rung3Limb::HullValue)
    } else {
        ("ssi_hull_sup", Rung3Limb::HullSup)
    };
    let enclosure = T::from_computed(floor)
        .enclosure_hull(T::from_computed(upper))
        .free_identity(identity);
    match crate::dihedral::decide_reported(predicate, geom_core::Margin::of(enclosure), band) {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok(()),
        Ok(Decided { margin, .. }) => Err(AnalyticRung3Refusal::Limb { operand, margin }),
        Err(cause) if floor > 0.0 => Err(AnalyticRung3Refusal::Escalated {
            operand: Some(operand),
            limb,
            cause,
        }),
        Err(cause) => Err(AnalyticRung3Refusal::Stalled { operand, cause }),
    }
}

/// **Limb 2's symbolic identity** ([`limb2_enclosure`]): the residual of
/// the carrier's own description against `operand`, with the carrier's
/// parameter free (`free` mints it). The operand is read as the
/// composite reads it, its frame made orthonormal
/// ([`crate::pcurve_cache::orthonormal_chart`]), so the identity is a
/// residual of the surface the margin bounds the distance from; its
/// implicit form ([`crate::implicit::implicit_residual`]) vanishes
/// exactly on that surface.
///
/// A closed form is one description over its whole interval, so its
/// residual is one. A spline is a polynomial per knot span, so it is
/// read per span — every span of the carrier, which claims no less than
/// the edge's interval needs — the free parameter minted at each span's
/// middle so the span's own polynomial is the one read, and the spans'
/// residuals are joined as a sum of squares, which is the zero form
/// only where each of them is.
fn free_residual<T: Decide>(
    carrier: &Curve3<T>,
    params: (T, T),
    operand: &Surface<T>,
    free: &dyn Fn(T) -> T,
) -> T {
    let twin = crate::pcurve_cache::orthonormal_chart(operand);
    let at = |t: T| crate::implicit::implicit_residual(&twin, carrier.eval(free(t)));
    let Curve3::Nurbs(net) = carrier else {
        return at(params.0);
    };
    let breaks: Vec<T> = net.knot_values().runs().map(|(v, _)| v).collect();
    let spans: Vec<T> = breaks
        .windows(2)
        .map(|w| at((w[0] + w[1]) / (T::one() + T::one())))
        .collect();
    match spans.as_slice() {
        [] => at(params.0),
        [one] => *one,
        all => all.iter().fold(T::zero(), |sum, &r| sum + r.powi(2)),
    }
}

/// **Limb 2 at a line's ends against a plane**, where the composite
/// bound did not clear the band: the carrier's distance from the plane
/// is affine in the parameter, so its magnitude is convex and its sup
/// over the interval is attained at an end. Each end's distance is
/// measured in certification arithmetic (C9) — `|(p − o)·n|` over `|n|`,
/// the line's and the plane's data and the end's parameter each lifted
/// through its certified bracket ([`line_plane_ends`]), at every scalar
/// — and read by whether its enclosure excludes zero:
///
/// - **an end whose enclosure's floor is above zero** is a measured,
///   certified non-zero distance, decided as one: within the band it
///   passes, a floor past `Kε` is a miss ([`AnalyticRung3Refusal::Limb`]),
///   and otherwise — in the ambiguity band or straddling its edge — it
///   escalates on that value ([`Rung3Limb::HullValue`]), the input
///   ambiguous at this tolerance;
/// - **an end whose enclosure reaches zero** (it cannot exclude the
///   carrier lying on the plane) passes if its upper end is within the
///   band, and otherwise is only an upper bound, which says nothing
///   about the input.
///
/// Both ends passing certify the edge (`Some(Ok)`); a miss, then an
/// escalation, refuses (`Some(Err)`); an end that is only an upper bound
/// past the band, with neither, decides nothing (`None`), and [`limb2`]
/// lets the composite's own stall stand. No ratio over the net's width
/// enters, and no round-to-nearest product: at `f64` a dot product at
/// kilometre scale rounds past a 1e-12 band, so a point reading could
/// pass a line that is off its plane.
fn measured_ends(
    operand: geom::SurfaceKind,
    ends: [(geom_core::Interval, geom_core::Interval); 2],
    band: Band,
) -> Option<Result<(), AnalyticRung3Refusal>> {
    let (mut miss, mut escalation, mut bare) = (None, None, false);
    for (measure, lever) in ends {
        let distance = measure.abs();
        let end = geom_core::Margin::over_lever(distance, lever);
        match crate::dihedral::decide_reported("ssi_hull_value", end, band) {
            Ok(Decided {
                sign: Sign::Zero, ..
            }) => {}
            Ok(Decided { margin, .. }) => {
                miss.get_or_insert(AnalyticRung3Refusal::Limb { operand, margin });
            }
            Err(cause) if geom_core::Bounds::lo(distance) > 0.0 => {
                escalation.get_or_insert(AnalyticRung3Refusal::Escalated {
                    operand: Some(operand),
                    limb: Rung3Limb::HullValue,
                    cause,
                });
            }
            Err(_) => bare = true,
        }
    }
    match (miss.or(escalation), bare) {
        (Some(refusal), _) => Some(Err(refusal)),
        (None, true) => None,
        (None, false) => Some(Ok(())),
    }
}

/// A line's distance from a plane at the ends of `params`, as the
/// `(measure, lever)` pair `(p − o)·n`, `|n|` per end in certification
/// arithmetic ([`measured_ends`]): the line `p = q + d·t` and the plane's
/// origin and normal are lifted through their certified brackets
/// (`Interval::from_certified`), and so is each end's parameter, so each
/// pair encloses the end's true distance whatever the scalar; `None` for
/// any other pair of kinds.
fn line_plane_ends<T: geom_core::CertifiedBounds>(
    carrier: &Curve3<T>,
    params: (T, T),
    operand: &Surface<T>,
) -> Option<[(geom_core::Interval, geom_core::Interval); 2]> {
    let lift = geom_core::Interval::from_certified;
    let vec = |v: geom_core::Vec3<T>| geom_core::Vec3::new(lift(v.x), lift(v.y), lift(v.z));
    match (carrier, operand) {
        (Curve3::Line { origin: q, dir }, Surface::Plane { origin, normal, .. }) => {
            let (q, d) = (vec(*q - geom_core::Point3::origin()), vec(*dir));
            let (o, n) = (vec(*origin - geom_core::Point3::origin()), vec(*normal));
            let lever = n.norm();
            Some([params.0, params.1].map(|t| ((q + d * lift(t) - o).dot(n), lever)))
        }
        _ => None,
    }
}

/// Limb 2's measured miss at a break: a certified floor on the
/// carrier's distance from the operand, decided zero against the band.
fn measured_miss<T: Decide>(
    operand: geom::SurfaceKind,
    floor: T,
    band: Band,
) -> Result<(), AnalyticRung3Refusal> {
    match crate::dihedral::decide_reported("ssi_hull_value", geom_core::Margin::of(floor), band) {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok(()),
        Ok(Decided { margin, .. }) => Err(AnalyticRung3Refusal::Limb { operand, margin }),
        Err(cause) => Err(AnalyticRung3Refusal::Escalated {
            operand: Some(operand),
            limb: Rung3Limb::HullValue,
            cause,
        }),
    }
}

/// Limb 2's upper bound over the edge's interval: it clears the band, or
/// it is the kernel's approximation limit at this tolerance — an upper
/// bound shows no miss, so it is neither a definite refusal nor an
/// ambiguity of the input.
fn upper_bound<T: Decide>(
    operand: geom::SurfaceKind,
    bound: T,
    upper: f64,
    size: crate::pcurve_cache::projected::NetSize,
    band: Band,
) -> Result<(), AnalyticRung3Refusal> {
    let limit = |margin| AnalyticRung3Refusal::KernelLimit {
        operand,
        bound: upper,
        margin,
        width: size.width,
        extent: size.extent,
    };
    match crate::dihedral::decide_reported("ssi_hull_sup", geom_core::Margin::of(bound), band) {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok(()),
        Ok(Decided { margin, .. }) => Err(limit(margin)),
        Err(cause) => Err(limit(cause.margin)),
    }
}

/// The piece of the carrier the tube reads: the carrier over `[lo, hi]`
/// — the edge's interval from its lower end's infimum to its upper
/// end's supremum — clamped to the knot domain (a poison end reads as
/// the domain's, the wider piece).
///
/// **What holds at each scalar.** A carrier whose weights are all one
/// value `c` is polynomial, and so is its piece: the piece's weights are
/// `c` exactly, and its controls — the blossoms of each knot span's
/// stretch, joined at full multiplicity — are formed with ratios in the
/// scalar ([`crate::pcurve_cache::projected::piece_controls`]), so at an
/// enclosure scalar each encloses the true piece's control. A carrier
/// with unequal weights has no piece whose weights are the carrier's
/// own at an enclosure scalar (a refinement there is held only as
/// homogeneous enclosures, C6), so the tube reads the whole carrier: sound,
/// since the chain around the piece is a stretch of the whole carrier's
/// chain, and conservative, since geometry past the edge's ends can
/// refuse it (`work/pcert/the-tube-reads-a-rational-carriers-whole-net.md`).
fn edge_piece<T: SpanLocate>(
    carrier: &NurbsCurve3<T>,
    lo: f64,
    hi: f64,
) -> Result<NurbsCurve3<T>, AnalyticRung3Refusal> {
    let refuse = |what| AnalyticRung3Refusal::Unsupported { what };
    let (d0, d1) = carrier.domain();
    let (a, b) = (lo.max(d0), hi.min(d1));
    if a.partial_cmp(&b) != Some(core::cmp::Ordering::Less) {
        return Err(refuse(
            "the edge's interval is empty on the carrier's domain",
        ));
    }
    if (a <= d0 && b >= d1) || carrier.constant_weight().is_none() {
        return Ok(carrier.clone());
    }
    let kv = carrier.knots_f64_interim();
    let p = kv.degree();
    let malformed = || refuse("the carrier's piece over the edge's interval is malformed");
    let stretches = geom_core::spline::ParamRange::new(a, b)
        .and_then(|range| crate::pcurve_cache::projected::span_stretches(kv, range))
        .ok_or_else(malformed)?;
    let mut knots = vec![a; p + 1];
    let mut control: Vec<Point3<T>> = Vec::new();
    for (i, stretch) in stretches.iter().enumerate() {
        let segment = crate::pcurve_cache::projected::piece_controls(carrier, *stretch);
        // Neighbouring segments share their joining control; each
        // segment's own encloses it, and the earlier one is kept.
        let skip = usize::from(i > 0);
        control.extend(
            segment
                .iter()
                .skip(skip)
                .map(|(v, _)| Point3::origin() + *v),
        );
        let last = i + 1 == stretches.len();
        knots.extend(std::iter::repeat_n(
            stretch.end(),
            if last { p + 1 } else { p },
        ));
    }
    let n = control.len();
    // The piece's interior breaks are the carrier's breakpoints and keep
    // their held values; its ends are the edge's parameters, entered as
    // the constants they are.
    KnotVector::clamped(knots, p)
        .ok()
        .and_then(|kv| {
            let held = carrier.knot_values().restructure(kv, T::from_f64);
            NurbsCurve3::new(held, control, vec![carrier.weights()[0]; n]).ok()
        })
        .ok_or_else(malformed)
}

/// The limbs the analytic rung-3 certificate decides, so the ones its
/// refusals can name: limb 2 on [`analytic_rung3`]'s own bound and on
/// its value at a break, and limb 3, which it asks the SSI certificate
/// for alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rung3Limb {
    /// Limb 2: the carrier's certified distance bound from an operand.
    HullSup,
    /// Limb 2 at a break of its composite: a certified lower bound on
    /// the carrier's distance from an operand there, past the band's
    /// zero, so no bound over a part holding the break can clear.
    HullValue,
    /// Limb 3: the uniqueness tube.
    Tube,
}

impl Rung3Limb {
    /// The limb's name, as [`SsiLimb::name`] gives the SSI limb this
    /// one is (its check is not that limb's: [`Rung3Limb::check`]).
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::HullSup => SsiLimb::HullSup,
            Self::HullValue => SsiLimb::HullValue,
            Self::Tube => SsiLimb::Tube,
        }
        .name()
    }

    /// The certification check a refusal of this limb is a refused arm
    /// of: limb 2 here is the carrier's certified distance bound from an
    /// analytic operand, not the plane × NURBS lane's, and its value at
    /// a break is that distance itself, measured.
    #[must_use]
    pub fn check(self) -> CertCheck {
        match self {
            Self::HullSup => CertCheck::AnalyticHull,
            Self::HullValue => CertCheck::AnalyticBreakResidual,
            Self::Tube => SsiLimb::Tube.check(),
        }
    }
}

/// [`analytic_rung3`]'s typed refusal — the analytic pair's own
/// vocabulary, closed, and carrying the measured number where one
/// exists.
#[derive(Clone, Copy, Debug, PartialEq)]
// The variant roster `topo`'s sample-coverage row reads (this
// crate's `test-support` feature, test builds only).
#[cfg_attr(
    feature = "test-support",
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(AnalyticRung3RefusalKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum AnalyticRung3Refusal {
    /// Limb 2's measured miss: the carrier's certified distance from
    /// the operand at a break lies past the band
    /// ([`Rung3Limb::HullValue`]), so the carrier is off that surface
    /// there. The one definite refusal limb 2 makes.
    Limb {
        /// The operand the reading is from.
        operand: geom::SurfaceKind,
        /// What the classifier saw of the reading, in metres, for error
        /// reporting only ([`MarginDiag`]): it rides
        /// [`AnalyticRung3Refusal::decision`]'s sign-certain arm to the
        /// import door's words on the limb's check.
        margin: MarginDiag,
    },
    /// **The kernel's approximation limit** ([tolerance.recourse]):
    /// limb 2's upper bound on the carrier's distance from the operand
    /// ends past the band's zero with no measured miss past it. An upper
    /// bound shows no miss, so this says nothing of the input; it says
    /// the certificate cannot get under the tolerance here. The bound is
    /// read against the carrier's own enclosure width in the operand's
    /// frame: near it, the input's extent is the limit at this
    /// tolerance; far above it, the composite's conditioning
    /// ([`AnalyticRung3Refusal::ending`]).
    KernelLimit {
        /// The operand the bound is from.
        operand: geom::SurfaceKind,
        /// The bound's upper end, in metres.
        bound: f64,
        /// The bound as the classifier saw it.
        margin: MarginDiag,
        /// The carrier's own enclosure width in the operand's frame, in
        /// metres ([`crate::carrier_net::CarrierNet::width`]).
        width: f64,
        /// The extent the bound was read over: the diagonal of the
        /// carrier's net's control box, in metres.
        extent: f64,
    },
    /// **Limb 2 undecided at this input** — at an enclosure scalar, the
    /// composite's enclosure `[0, upper]` of the carrier's distance from
    /// the operand neither clears the band nor shows a measured miss
    /// ([`limb2_enclosure`]). It claims no cause and no terminality: over
    /// a parameter box a smaller box is the remedy and the drive bisects
    /// it (E6), its escalation on the node's log. A floor measured inside
    /// the band is not this: it escalates on the measured value
    /// ([`Self::Escalated`] on [`Rung3Limb::HullValue`]).
    Stalled {
        /// The operand the bound is from.
        operand: geom::SurfaceKind,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// A limb's margin escalated.
    Escalated {
        /// The operand whose distance escalated; `None` for the tube.
        operand: Option<geom::SurfaceKind>,
        /// Which limb.
        limb: Rung3Limb,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// No certified distance bound exists for the carrier against this
    /// operand: the composite or its metre conversion refused (on a
    /// cone, a part reaching the other nappe, or a window whose chart
    /// sheet the composite cannot place).
    NoOffsetBound {
        /// The operand.
        operand: geom::SurfaceKind,
        /// Why, in a clause.
        why: &'static str,
    },
    /// The carrier has no certified enclosing net over a span of the
    /// edge's interval (a spiric whose root's argument reaches zero
    /// there, or a carrier whose data are not finite), so limb 2 has no
    /// composite to bound ([`PlaneNurbsRefusal::CarrierEnclosure`] reads
    /// the same fact).
    CarrierEnclosure(crate::CarrierEnclosureRefusal),
    /// The uniqueness tube has no enclosure of this operand's implicit
    /// gradient, so no box of any rung certifies, whatever the carrier's
    /// accuracy: raised once limb 2 holds against both operands and the
    /// pair's section states no bounds for the pose
    /// ([`crate::intersect::tube_bounds`]).
    NoTubeEnclosure {
        /// The operand.
        operand: geom::SurfaceKind,
    },
    /// The uniqueness tube's lower bound on the crossing angle is not
    /// clear of the zero band — a sliver of the pair along the carrier
    /// at this tolerance ([`PlaneNurbsRefusal::TubeStraddles`] reads the
    /// same verdict).
    TubeStraddles {
        /// The verdict on the transversality's certified clearance.
        verdict: Refused,
        /// How many boxes of the chain it was certified over.
        boxes: u32,
    },
    /// The tube was a graph at some rung, but at none was its chain
    /// proved to hold one arc spanning the carrier and nothing else.
    TubeNotOneArc {
        /// How many rungs were graphs but not proved one arc.
        rungs: u32,
        /// What the narrowest of them found.
        cause: OneArcRefusal,
    },
    /// The pair is outside this certificate's inventory, named.
    Unsupported {
        /// The refused class.
        what: &'static str,
    },
}

impl AnalyticRung3Refusal {
    fn of_offset(operand: geom::SurfaceKind, e: crate::PcurveCertifyError) -> Self {
        match e {
            crate::PcurveCertifyError::Escalated { cause, .. } => Self::Escalated {
                operand: Some(operand),
                limb: Rung3Limb::HullSup,
                cause,
            },
            crate::PcurveCertifyError::CarrierEnclosure(refusal) => Self::CarrierEnclosure(refusal),
            crate::PcurveCertifyError::SectorRefused { channel, .. } => Self::NoOffsetBound {
                operand,
                why: channel.describe(),
            },
            _ => Self::NoOffsetBound {
                operand,
                why: "the operand's canonical implicit form composed along the carrier has no \
                      certified bound",
            },
        }
    }

    fn of_tube(e: SsiError) -> Self {
        match e {
            SsiError::TubeStraddles { verdict, boxes } => Self::TubeStraddles { verdict, boxes },
            SsiError::TubeNotOneArc { rungs, cause } => Self::TubeNotOneArc { rungs, cause },
            SsiError::CertificateEscalated {
                limb: SsiLimb::Tube,
                cause,
            } => Self::Escalated {
                operand: None,
                limb: Rung3Limb::Tube,
                cause,
            },
            // `Limbs::Tube` asks no operand for limbs 1 and 2, so the
            // only escalation `certify_branch` can return here is limb
            // 3's own (`tube_transversality`).
            SsiError::CertificateEscalated { limb, .. } => unreachable!(
                "the tube-only certificate escalated on limb {}, which Limbs::Tube never asks",
                limb.name()
            ),
            SsiError::UnsupportedCertificate { what } => Self::Unsupported { what },
            SsiError::TubeProbeSilent { refused, .. } if refused > 0 => Self::Unsupported {
                what: "no rung of the uniqueness tube measured a margin, and a transversality \
                       enclosure was refused",
            },
            SsiError::TubeProbeSilent { .. } => Self::Unsupported {
                what: "no rung of the uniqueness tube built a chain around the carrier",
            },
            _ => Self::Unsupported {
                what: "the uniqueness tube refused for a reason outside this certificate's \
                       vocabulary",
            },
        }
    }

    /// The ending this refusal's decision gives it, read at `at`, or
    /// `None` for a refusal that is no decision's refused arm
    /// ([`PlaneNurbsRefusal::ending`]'s structure).
    #[must_use]
    pub fn ending(&self, at: impl Into<ReadAt>) -> Option<String> {
        if let Self::TubeNotOneArc { cause, .. } = *self {
            return Some(cause.ending(crate::ssi::OneArcDoor::AtRest, at));
        }
        if let Self::KernelLimit {
            bound,
            width,
            extent,
            ..
        } = *self
        {
            return Some(kernel_limit_ending(bound, width, extent));
        }
        self.decision().map(|(check, arm)| recourse(check, arm, at))
    }

    /// The decision this refusal is a refused arm of, and which arm.
    #[must_use]
    pub fn decision(&self) -> Option<(CertCheck, RefusedArm<'_>)> {
        Some(match self {
            Self::Limb { margin, .. } => (
                Rung3Limb::HullValue.check(),
                RefusedArm::SignCertain(Some(*margin)),
            ),
            Self::Escalated { limb, cause, .. } => (limb.check(), RefusedArm::Undecided(cause)),
            Self::Stalled { cause, .. } => {
                (Rung3Limb::HullSup.check(), RefusedArm::Undecided(cause))
            }
            Self::TubeStraddles { verdict, .. } => (CertCheck::TransversalityTube, verdict.arm()),
            // The one-arc proof is the SSI door's own decision, and
            // `ending` reads it there.
            // The kernel's limit is no decision's refused arm about the
            // input, and `ending` gives its own story.
            Self::NoOffsetBound { .. }
            | Self::KernelLimit { .. }
            | Self::CarrierEnclosure(_)
            | Self::NoTubeEnclosure { .. }
            | Self::TubeNotOneArc { .. }
            | Self::Unsupported { .. } => {
                return None;
            }
        })
    }
}

impl core::fmt::Display for AnalyticRung3Refusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Limb { operand, margin } => write!(
                f,
                "{} puts the carrier at least {margin:e} m from the {}, past the run \
                 tolerance — the carrier leaves that surface between the schedule's samples",
                Rung3Limb::HullValue.name(),
                operand.name()
            ),
            Self::KernelLimit {
                operand,
                bound,
                width,
                extent,
                ..
            } => write!(
                f,
                "{} bounds the carrier's distance from the {} by {bound:e} m over {extent:e} m \
                 of it, which does not clear the run tolerance; the carrier's own enclosure \
                 width there is {width:e} m, and an upper bound shows no miss, so the \
                 certificate cannot show the carrier stays on that surface at this tolerance",
                Rung3Limb::HullSup.name(),
                operand.name()
            ),
            Self::Stalled { operand, cause } => write!(
                f,
                "{} against the {} is undecided over this input: {}",
                Rung3Limb::HullSup.name(),
                operand.name(),
                cause.payload()
            ),
            Self::Escalated {
                operand,
                limb,
                cause,
            } => {
                write!(f, "{}", limb.name())?;
                if let Some(kind) = operand {
                    write!(f, " against the {}", kind.name())?;
                }
                write!(f, " escalated: {}", cause.payload())
            }
            Self::CarrierEnclosure(r) => write!(
                f,
                "limb 2 could not enclose the carrier between samples: {r}"
            ),
            Self::NoOffsetBound { operand, why } => write!(
                f,
                "the carrier's distance from the {} has no certified bound: {why}",
                operand.name()
            ),
            Self::NoTubeEnclosure { operand } => write!(
                f,
                "the carrier lies within the tolerance of both surfaces, but the uniqueness tube \
                 has no enclosure of the {}'s gradient, so it cannot prove the carrier is their \
                 one crossing",
                operand.name()
            ),
            Self::TubeStraddles { verdict, boxes } => write!(
                f,
                "{} is not clear of the zero band over {boxes} boxes of the chain — {}'s \
                 enclosure straddles zero, from a near-tangency of the two surfaces along the \
                 carrier or from the enclosure's slack; the bound reads {:e} m, which is what \
                 the certificate could prove",
                crate::certify::TUBE_CROSSING_BOUND,
                crate::certify::CROSSING_ANGLE,
                verdict.margin()
            ),
            Self::TubeNotOneArc { rungs, cause } => write!(
                f,
                "the tube was not proved to hold one arc spanning the carrier at {rungs} rungs; \
                 at the narrowest, {cause}"
            ),
            Self::Unsupported { what } => {
                write!(f, "outside the analytic rung-3 certificate: {what}")
            }
        }
    }
}

/// How far above the carrier's own enclosure width limb 2's bound may lie
/// and still read as that width ([`AnalyticRung3Refusal::KernelLimit`]'s
/// ending). An order of magnitude: the composite's arithmetic adds a few
/// roundings at the net's scale to the net's own.
pub const KERNEL_LIMIT_ROUNDING_FACTOR: f64 = 16.0;

/// How far above `extent · 2⁻⁵²` the carrier's own enclosure width may
/// lie and still be its extent's rounding
/// ([`AnalyticRung3Refusal::KernelLimit`]'s ending): past it the width
/// is set by something other than the edge's length — its distance from
/// the operand's origin, or the width of its inputs' own enclosures —
/// and the story does not say which.
pub const KERNEL_LIMIT_EXTENT_FACTOR: f64 = 64.0;

/// **The one story of limb 2's kernel limit** ([tolerance.recourse] (i),
/// (iv)), its recourse read off the data: a bound within
/// [`KERNEL_LIMIT_ROUNDING_FACTOR`] of the carrier's own rounding `width`
/// is that width; where that width is the edge's own extent's rounding
/// (within [`KERNEL_LIMIT_EXTENT_FACTOR`] of `extent · 2⁻⁵²`), the extent
/// is the limit at this tolerance and shorter edges are the lever. A
/// wider enclosure, or a bound far above it, has a cause the story
/// cannot know from these numbers, so it claims none: it names
/// loosening as the last resort and the kernel bug it may be.
fn kernel_limit_ending(bound: f64, width: f64, extent: f64) -> String {
    let rounding = bound <= KERNEL_LIMIT_ROUNDING_FACTOR * width;
    if rounding && width <= KERNEL_LIMIT_EXTENT_FACTOR * extent * f64::EPSILON {
        "The bound is the curve's own rounding at its length, too long to certify at this \
         tolerance. Recourse: split the edge into shorter edges"
            .to_owned()
    } else if rounding {
        format!(
            "The bound is the curve's own enclosure width, wider than its length's rounding. {}",
            geom_core::KERNEL_LIMIT_RECOURSE
        )
    } else {
        format!(
            "The bound lies far above the curve's own enclosure width. {}",
            geom_core::KERNEL_LIMIT_RECOURSE
        )
    }
}

/// What [`chart_image`] reads of a carrier: its evaluator, the `f64`
/// domain the schedule spans, and that domain's ends as the carrier
/// holds them. A spline's domain is its own knot domain
/// (`Schedule::new(&eval, net.domain(), net.knot_values().domain())`);
/// a closed-form carrier's is the one its caller states.
pub(crate) struct Schedule<'a, T: geom_core::Real> {
    eval: &'a dyn Fn(T) -> geom_core::Point3<T>,
    domain: (f64, f64),
    ends: (T, T),
}

impl<'a, T: geom_core::Real> Schedule<'a, T> {
    /// The schedule of `eval` over `domain`, held at `ends`.
    pub(crate) fn new(
        eval: &'a dyn Fn(T) -> geom_core::Point3<T>,
        domain: (f64, f64),
        ends: (T, T),
    ) -> Self {
        Self { eval, domain, ends }
    }

    /// The `f64` domain the schedule spans.
    pub(crate) fn domain(&self) -> (f64, f64) {
        self.domain
    }
}

/// **The chart image of a declared carrier on a NURBS wall** — the one
/// derivation of this object in the tree.
///
/// The foot points of the D9-fixed [`PXN_FIT_SAMPLES`] schedule,
/// interpolated (not approximated, so the image is exact at every foot)
/// at [`PXN_IMAGE_DEGREE`] on the carrier's own parameter: the OQ4
/// identity, `P(t)` and `C(t)` the same `t` by construction. The
/// interpolation runs in C6's `f64` structure lane and is lifted to
/// `T`; nothing here is certified, and nothing here needs to be — the
/// image is EVIDENCE, and what makes it sound is that its consumer
/// bounds `sup_t |S(P(t)) − C(t)|` over the whole span (module docs).
///
/// **Two consumers, one producer.** [`plane_nurbs_limbs`] certifies the image as
/// part of the plane × NURBS edge certificate at ADOPT time;
/// [`crate::FittedLane::general_image`] hands the same image to
/// the pcurve mint, where it becomes a stored
/// [`crate::Pcurve::General`] cache certified through
/// [`crate::PcurveCache::certify_general`]. They must be the same bits
/// — an edge whose certificate was derived from one image and whose
/// cache stores another would be certifying a curve the body does not
/// carry — so there is one producer and both call it.
///
/// `per_sample` is a hook run at every schedule sample in order, with
/// the sample index and its foot; it is where [`plane_nurbs_limbs`] puts its
/// transversality sweep, so that adding this second consumer did not
/// move the order in which two refusals of the same run can fire. The
/// mint passes a hook that does nothing.
///
/// # Errors
///
/// [`PlaneNurbsRefusal::CarrierDomain`] before any sample when the
/// carrier's domain is not an interval the door accepts, or after the
/// fit when the image's knots collapse on it;
/// [`PlaneNurbsRefusal::FootPointInconclusive`] at the first sample
/// whose projection does not converge (never a best-effort foot);
/// [`PlaneNurbsRefusal::PcurveFit`] when the interpolation refuses; or
/// whatever `per_sample` returns.
///
/// The carrier enters as its [`Schedule`]: `net_schedule` for a
/// spline, its own knot domain and held ends.
pub(crate) fn chart_image<T, F>(
    schedule: Schedule<'_, T>,
    wall: &NurbsSurface<T>,
    mut per_sample: F,
) -> Result<NurbsCurve2<T>, PlaneNurbsRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
    F: FnMut(u32, Point2<f64>) -> Result<(), PlaneNurbsRefusal>,
{
    let Schedule {
        eval,
        domain: (t0, t1),
        ends: (h0, h1),
    } = schedule;
    if wall.is_placeholder() {
        // The same refusal `plane_nurbs_limbs` states before it gets here, kept at
        // the producer too: the placeholder has no description
        // yet, and projecting onto it would return
        // feet of a surface that does not exist. `plane_nurbs_limbs`
        // still checks first, so its own refusal ORDER is unchanged.
        return Err(PlaneNurbsRefusal::Unsupported {
            what: geom::PLACEHOLDER_SURFACE,
        });
    }
    let domain_refusal = |e| PlaneNurbsRefusal::CarrierDomain(CarrierDomainRefusal::of(t0, t1, &e));
    // The domain door, asked first on a vector with no interior knots
    // (so only the interval itself can refuse): a domain whose width
    // overflows would otherwise reach the schedule as NaN parameters and
    // refuse as a foot that did not converge, hiding the carrier's own
    // fault behind the projection's.
    KnotVector::unit_segment(NonZeroUsize::MIN)
        .on_domain(t0, t1)
        .map_err(domain_refusal)?;
    // The schedule is a SUPERSET of the certificate's own
    // ([`CERT_SAMPLES`] divides it) and both are `schedule_param`, so
    // limb 1 re-projects at parameters the image passes through
    // exactly, its assigned ends included: the image is re-expressed
    // on `[t0, t1]` exactly (`on_carrier_domain`), so the foot its last
    // knot carries is the projection at `t1` itself.
    // The fit's parameters are the schedule's `f64` fractions (the
    // image's knots are the fit's); the carrier is read at the same
    // station, its ends held (`held_station`).
    let mut uv = Vec::with_capacity(PXN_FIT_SAMPLES as usize);
    let mut params = Vec::with_capacity(PXN_FIT_SAMPLES as usize);
    for i in 0..PXN_FIT_SAMPLES {
        let frac = schedule_fraction(i, PXN_FIT_SAMPLES);
        let p = eval(crate::certify::held_station(
            (t0, t1),
            (h0, h1),
            i,
            PXN_FIT_SAMPLES,
        ));
        let proj = wall
            .project(p)
            .map_err(|e| PlaneNurbsRefusal::FootPointInconclusive {
                sample: i,
                last_distance: e.last_distance,
            })?;
        let foot = Point2::new(proj.u, proj.v);
        uv.push(foot);
        params.push(frac);
        per_sample(i, foot)?;
    }
    let image = NurbsCurve2::<f64>::interpolate_with_params(&uv, PXN_IMAGE_DEGREE, &params)
        .map_err(|_| PlaneNurbsRefusal::PcurveFit)?;
    on_carrier_domain(&image, t0, t1).map_err(domain_refusal)
}

/// **The certified foot of ONE point** on a NURBS wall, in the wall's
/// own chart coordinates — [`chart_image`]'s single-sample sibling,
/// with the same producer (`NurbsSurface::project`, D9-fixed) and the
/// same `f64` structure lane.
///
/// Its consumer is the pcurve mint's rim arms, which need to know WHERE
/// on the chart an edge's endpoint actually lands rather than assuming
/// it lands on a knot-domain end. One foot rather than the image's [`PXN_FIT_SAMPLES`],
/// because those arms already know the SHAPE of their image (a chart
/// row or column) and are only missing its position.
///
/// Certifies nothing by itself, exactly as [`chart_image`] does not:
/// the caller offers the result to its own metre-valued check and the
/// full iso certification follows.
///
/// # Errors
///
/// [`PlaneNurbsRefusal::FootPointInconclusive`] when the projection
/// will not converge (never a best-effort foot), or
/// [`PlaneNurbsRefusal::Unsupported`] on the placeholder (`geom::PLACEHOLDER_SURFACE`).
pub(crate) fn chart_foot<T>(
    point: Point3<T>,
    wall: &NurbsSurface<T>,
) -> Result<Point2<f64>, PlaneNurbsRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
{
    if wall.is_placeholder() {
        return Err(PlaneNurbsRefusal::Unsupported {
            what: geom::PLACEHOLDER_SURFACE,
        });
    }
    let proj = wall
        .project(point)
        .map_err(|e| PlaneNurbsRefusal::FootPointInconclusive {
            sample: 0,
            last_distance: e.last_distance,
        })?;
    Ok(Point2::new(proj.u, proj.v))
}

/// How many knot spans per direction the tube reads the wall's cells
/// at ([`wall_grid`]). A **structure** choice (never a decision), the
/// surface-side twin of `ssi::certify`'s `SSI_CERT_SPANS` for the
/// carrier.
pub const PXN_WALL_SPANS: usize = 16;

/// The grid points [`wall_grid`] inserts along one of the wall's
/// directions: the domain's [`PXN_WALL_SPANS`]ths
/// (`domain_grid_points`), none for a direction already holding
/// `PXN_WALL_SPANS + degree` control points.
fn wall_breaks(kv: &KnotVector) -> Vec<f64> {
    if kv.control_count() >= PXN_WALL_SPANS + kv.degree() {
        return Vec::new();
    }
    geom_core::spline::algebra::domain_grid_points(kv, PXN_WALL_SPANS)
}

/// The cells the tube reads the wall at: its net refined to about
/// [`PXN_WALL_SPANS`] spans per direction, in certification arithmetic
/// ([`CellGrid`]).
///
/// The tube's chart readings (the minted chart speeds its pads divide
/// the rung by, the transversality margin, the one-arc boxes) are hulls
/// over span cells, and a wall with few spans reads them loose: on a
/// rational wall whose weights run over decades, the whole-domain speed
/// sup over its own cells reads up to a thousand times its sampled
/// speed, and over sixteenths under sixteen times it
/// (`tests/review_4553_pxn_probes.rs`). Knot insertion is exact in ℝ,
/// so the grid describes the same wall; refined in the ring, it
/// encloses that wall's refined net rather than a rounded neighbour's
/// (C6).
///
/// `None`, and the wall's own cells, when neither direction takes a
/// point or the refinement refuses: a coarser enclosure can only make
/// the certificate harder to pass, never unsound. Limb 2's hull and the
/// evaluations read the wall itself.
fn wall_grid<T: geom_core::CertifiedBounds>(wall: &NurbsSurface<T>) -> Option<CellGrid> {
    let (bu, bv) = (
        wall_breaks(wall.knots_u_f64_interim()),
        wall_breaks(wall.knots_v_f64_interim()),
    );
    if bu.is_empty() && bv.is_empty() {
        return None;
    }
    CellGrid::refined(wall, &bu, &bv)
}

/// The chart image's own D9-fixed schedule: how many foot points the
/// image is interpolated through.
///
/// A **superset** of the certificate's [`CERT_SAMPLES`] schedule
/// (`CERT_SAMPLES − 1` divides `PXN_FIT_SAMPLES − 1`), so limb 1
/// re-projects at parameters the image was built to pass through
/// exactly, and the transversality sweep below sees every certificate
/// sample and more. Denser is strictly stricter: it can only tighten
/// the image and add transversality samples.
pub const PXN_FIT_SAMPLES: u32 = 33;

/// The chart image's degree: **1**, the piecewise-linear interpolant
/// through the foot schedule.
///
/// Measured, not assumed. Limb 2 bounds `sup_t |S(P(t)) − C(t)|` by
/// hulling the composite per span, and that bound has two error
/// sources — the image's own deviation from the true foot path
/// (`O(h²·κ_uv)` at degree 1, `O(h⁴·κ_uv)` at degree 3) and the hull's
/// per-span widening, which GROWS with the composite's degree. On the
/// certifying fixture the cubic image measures `9.8e-11 m` and the
/// piecewise-linear one `1.1e-13 m`: at the residual scales this lane
/// exists for, the hull widening dominates and the low degree wins by
/// ~10³. A higher-degree rung would only pay off in a residual window
/// (`1e-10`…`1e-5`) that lies entirely above its own widening floor,
/// so there is no second rung — one structure, fixed.
///
/// The consequence is a stated class boundary, not a hidden one: the
/// edges this lane certifies are those whose foot path is straight in
/// the wall's chart to within `ε` over a schedule step — planar
/// sections along iso-structured walls, which is the seam class. A
/// genuinely curved plane × NURBS locus refuses with its MEASURED
/// bound in the payload (never a widened gate), and tightening that is
/// the algebraic route already banked with #264's envelope findings.
pub const PXN_IMAGE_DEGREE: usize = 1;

/// A chart foot the projection measured, at the caller's scalar: two
/// computed values, so at `Sym` two unknowns.
fn measured_foot<T: Real>(foot: Point2<f64>) -> (T, T) {
    (T::from_computed(foot.x), T::from_computed(foot.y))
}

/// `|n̂ × m̂|` — the sine of the angle between the plane's exact unit
/// normal and the wall's normal `S_u × S_v`.
///
/// Poison propagates: a degenerate chart (`S_u ∥ S_v`) yields a
/// zero-norm cross product and a NaN sine, which loses every
/// comparison and refuses downstream rather than reading as
/// "transverse".
fn normal_angle_sine<T: Real>(plane_normal: Vec3<T>, wall_normal: Vec3<T>) -> T {
    let m = wall_normal.norm();
    let n = plane_normal.norm();
    plane_normal.cross(wall_normal).norm() / (n * m)
}

/// The `f64`-structure chart image re-expressed on the carrier's own
/// parameter domain and lifted to the caller's scalar — two
/// operations, in that order: the interpolation's clamped `0 → 1`
/// knots onto `[t0, t1]` through the curve's own domain door (ends
/// exact, interior affine, at `f64`), then the control net and the
/// knots through `T::from_computed`: they are a fit to measured feet,
/// so at `Sym` each is its own unknown, never a constant. The weights
/// are the interpolation's unit weights, a constant of its form, and
/// enter through `from_f64`. The rescale is the only step that can
/// refuse (the lift carries a validated curve's net verbatim), so the
/// `Result` is the knot door's alone. The image's end knots are the
/// carrier's `f64` domain ends and enter with the rest of the knots as
/// unknowns: sound, but not the carrier's held ends, so an identity
/// between the image's end and the carrier's does not discharge
/// (`work/intent/an-edges-parameter-ends-enter-a-carrier-piece-as-constants.md`).
///
/// The knot map's own rounding is not a soundness question — the image
/// is evidence that limb 2 bounds, not a certified quantity (module
/// docs).
fn on_carrier_domain<T: SpanLocate>(
    image: &NurbsCurve2<f64>,
    t0: f64,
    t1: f64,
) -> Result<NurbsCurve2<T>, SplineError> {
    let image = image.on_domain(t0, t1)?;
    NurbsCurve2::new(
        image.knot_values().lift(T::from_computed),
        image
            .control()
            .iter()
            .map(|p| p.map(T::from_computed))
            .collect(),
        image.weights().iter().map(|w| T::from_f64(*w)).collect(),
    )
}

/// The SSI refusal, in this lane's vocabulary.
fn refusal(e: SsiError) -> PlaneNurbsRefusal {
    match e {
        SsiError::CertificateLimb { limb, margin } => PlaneNurbsRefusal::Limb { limb, margin },
        SsiError::TubeStraddles { verdict, boxes } => {
            PlaneNurbsRefusal::TubeStraddles { verdict, boxes }
        }
        SsiError::CertificateEscalated { limb, cause } => {
            PlaneNurbsRefusal::Escalated { limb, cause }
        }
        SsiError::TubeNotOneArc { rungs, cause } => {
            PlaneNurbsRefusal::TubeNotOneArc { rungs, cause }
        }
        SsiError::FootPointInconclusive { t, last_distance } => {
            // The limb re-projects warm-started from the image; a
            // divergence there is the same class as the schedule's own,
            // reported at the sample the parameter names.
            let _ = t;
            PlaneNurbsRefusal::FootPointInconclusive {
                sample: CERT_SAMPLES,
                last_distance,
            }
        }
        SsiError::UnsupportedCertificate { what } => PlaneNurbsRefusal::Unsupported { what },
        SsiError::ChartSpeed(r) => PlaneNurbsRefusal::ChartSpeed(r),
        SsiError::CarrierEnclosure(r) => PlaneNurbsRefusal::CarrierEnclosure(r),
        _ => PlaneNurbsRefusal::Unsupported {
            what: "the rung-3 certificate refused for a reason outside this lane's vocabulary",
        },
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::FileCoincidence;
    use geom_core::spline::KnotVector;

    use crate::exact_q::Q;
    use crate::recourse::Reading;

    use super::*;

    /// **A refused bound stalls limb 2.** At an enclosure scalar the
    /// reading's upper end is decided as `[floor, upper]`: a certified
    /// bound at zero passes, and the same ends under a refused
    /// decoration stall, where a read of its ordinary ends would pass.
    #[test]
    fn a_refused_offset_bound_stalls_limb_two() {
        use crate::pcurve_cache::projected::Offset;
        use geom_core::Interval;
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let decide = |b: Interval| {
            let upper = Offset::Bound(b).upper();
            limb2_enclosure::<Interval>(geom::SurfaceKind::Plane, 0.0, upper, band, |_| b)
        };
        assert!(
            decide(Interval::from_bounds(0.0, 0.0)).is_ok(),
            "a certified zero passes"
        );
        let refused = decide(geom_core::test_support::refused_point(0.0));
        assert!(
            matches!(refused, Err(AnalyticRung3Refusal::Stalled { .. })),
            "a refused zero stalls: {refused:?}"
        );
    }

    /// **A silent tube says why it is silent.** Where a transversality
    /// enclosure was refused at some rung the edge says so; where no rung
    /// built a chain it says that. Neither reads as the generic
    /// out-of-vocabulary refusal.
    #[test]
    fn a_silent_tube_names_its_silence() {
        use crate::ssi::SsiError;
        let words =
            |rungs, refused| match AnalyticRung3Refusal::of_tube(SsiError::TubeProbeSilent {
                rungs,
                refused,
            }) {
                AnalyticRung3Refusal::Unsupported { what } => what,
                other => panic!("not the silent tube's refusal: {other:?}"),
            };
        assert!(
            words(20, 20).contains("enclosure was refused"),
            "{}",
            words(20, 20)
        );
        assert!(
            words(20, 3).contains("enclosure was refused"),
            "{}",
            words(20, 3)
        );
        assert!(words(20, 0).contains("built a chain"), "{}", words(20, 0));
    }

    /// A unit-domain image with the structure a lift must carry
    /// verbatim: degree 2, a double interior knot, non-unit weights.
    fn image() -> NurbsCurve2<f64> {
        let knots =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.25, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let control = vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.3, 1.1),
            Point2::new(1.2, 0.9),
            Point2::new(1.7, -0.4),
            Point2::new(2.5, 0.2),
            Point2::new(3.0, 1.0),
        ];
        let weights = vec![1.0, 0.7, 1.3, 2.0, 0.9, 1.0];
        NurbsCurve2::new(knots, control, weights).unwrap()
    }

    /// **Limb 3's at-rest refusal is held to the concision standard**,
    /// every cause at every reading: its payload then its ending, at most
    /// 75 words, with one recourse that never speaks of a search. Red
    /// under a lever that names the searched region at rest, and under
    /// the payload and ending growing past the budget.
    #[test]
    fn the_tube_not_one_arc_refusal_renders_within_the_standard() {
        use crate::ssi::OneArcRefusal;
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let undecided = geom_core::k_stats::decide_positive(
            "ssi_tube_one_arc",
            geom_core::Margin::of(f64::NAN),
            band,
        )
        .unwrap_err();
        for cause in [
            OneArcRefusal::Count { solutions: 0 },
            OneArcRefusal::Count { solutions: 4 },
            OneArcRefusal::Unlinked,
            OneArcRefusal::Short,
            OneArcRefusal::Undecided(undecided),
            OneArcRefusal::AlongSide { chart: 1 },
            OneArcRefusal::EndOnSides { sides: 4 },
        ] {
            let refusal = PlaneNurbsRefusal::TubeNotOneArc { rungs: 20, cause };
            for reading in [Reading::Build, Reading::AtRest] {
                let ending = refusal.ending(reading).unwrap();
                let rendered = format!("{refusal} {ending}");
                let words = rendered.split_whitespace().count();
                assert!(words <= 75, "{words} words: {rendered}");
                assert_eq!(rendered.matches("Recourse").count(), 1, "{rendered}");
                assert!(!rendered.contains("searched region"), "{rendered}");
            }
        }
    }

    /// **Limb 3's at-rest refusal reads the file's ε_in at the import
    /// door** (D4 ¶1): an undecided one-arc clearance at or below ε_in is
    /// not offered a tolerance alone, and one past it is offered the
    /// at-rest value. Red where the door's `ending` reads the one-arc
    /// decision at rest, which offers "tighten below 5e-10 m" for a
    /// 5e-9 m clearance within the file's 1e-6 m.
    #[test]
    fn the_tube_not_one_arc_refusal_reads_eps_in_at_the_import_door() {
        use crate::ssi::OneArcRefusal;
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let refusal = |m| PlaneNurbsRefusal::TubeNotOneArc {
            rungs: 20,
            cause: OneArcRefusal::Undecided(Indeterminate {
                margin: geom_core::MarginDiag::value(m),
                band,
                predicate: Some("ssi_tube_one_arc"),
                terminal_sliver: false,
            }),
        };
        let file = |eps_in| FileCoincidence::new(eps_in);
        let within = refusal(5e-9).ending(file(1e-6)).unwrap();
        assert!(
            within.starts_with(
                "This clearance is within the file's declared coincidence distance ε_in = 1e-6 \
                 m, so the file cannot tell it from zero. Recourse: store the edge's curve"
            ) && within.ends_with(
                ", or, if this clearance is intended, re-export the file with its uncertainty \
                 declared below 5e-9 m and tighten the tolerance below 5e-10 m"
            ),
            "{within}"
        );
        let past = refusal(5e-9);
        assert_eq!(
            past.ending(file(1e-9)),
            past.ending(Reading::AtRest),
            "a clearance past ε_in reads as at rest"
        );
    }

    /// The carrier interval the rows use: `0.3 + (0.9 − 0.3)` is an
    /// ulp above `0.9`, so a computed end would miss it.
    const CARRIER: (f64, f64) = (0.3, 0.9);

    /// The fit's computed values enter as unknowns at `Sym`: a measured
    /// foot's two coordinates, and every coordinate of the fitted
    /// image's control net. A constant would re-value to its own bits.
    #[test]
    fn the_fit_enters_its_feet_and_its_net_as_computed_values() {
        use geom_core::sym::revalue::revalue;
        use geom_core::sym::with_session;
        use geom_core::{Sym, SymBudget};
        type S = Sym<f64>;
        let budget = SymBudget {
            max_terms: 64,
            max_degree: 8,
        };
        let none = |_| None;
        let ((foot, net), _) = with_session(budget, || {
            let (u, v) = measured_foot::<S>(Point2::new(0.25, 0.75));
            let (t0, t1) = CARRIER;
            let lifted = on_carrier_domain::<S>(&image(), t0, t1).unwrap();
            let net: Vec<_> = lifted
                .control()
                .iter()
                .flat_map(|p| [revalue(p.x.node(), &none), revalue(p.y.node(), &none)])
                .collect();
            ((revalue(u.node(), &none), revalue(v.node(), &none)), net)
        });
        assert_eq!(foot, (None, None), "a measured foot's u and v are unknowns");
        assert!(
            net.iter().all(Option::is_none),
            "the fitted net's coordinates are unknowns: {net:?}"
        );
    }

    /// The rescale is the rescale and the lift is the lift, at `f64`:
    /// the carrier-domain image's domain is `(t0, t1)` bit for bit,
    /// its knots are the domain door's and its net and weights the
    /// source's verbatim, it is the SOURCE reparametrized — equal to
    /// the source at the pulled-back parameter, bit for bit at the
    /// ends and to the evaluator's rounding between them — and the
    /// only refusal is the knot door's, named as the domain's.
    #[test]
    fn on_carrier_domain_at_f64_pins_the_domain_carries_the_structure_and_reparametrizes() {
        let image = image();
        let (t0, t1) = CARRIER;
        let lifted = on_carrier_domain::<f64>(&image, t0, t1).unwrap();
        let (lo, hi) = lifted.domain();
        assert_eq!((lo.to_bits(), hi.to_bits()), (t0.to_bits(), t1.to_bits()));
        assert_eq!(
            lifted.knots_f64_interim(),
            &image.knots_f64_interim().on_domain(t0, t1).unwrap()
        );
        let net = |c: &[Point2<f64>]| {
            c.iter()
                .map(|p| (p.x.to_bits(), p.y.to_bits()))
                .collect::<Vec<_>>()
        };
        assert_eq!(net(lifted.control()), net(image.control()));
        assert_eq!(lifted.weights(), image.weights());

        // The ends, bit for bit: exact end multiplicity makes every de
        // Boor weight 0 or 1 there on BOTH sides, so the two
        // evaluations run the same arithmetic over the same net — which
        // is exactly what a domain an ulp off `t1` would break.
        let bits = |p: Point2<f64>| (p.x.to_bits(), p.y.to_bits());
        assert_eq!(bits(lifted.eval(t0)), bits(image.eval(0.0)));
        assert_eq!(bits(lifted.eval(t1)), bits(image.eval(1.0)));
        // Between them, at the dyadic samples whose pull-back
        // `(t − t0)/(t1 − t0)` is exact, so both sides evaluate the same
        // mathematical parameter and only the evaluator's rounding
        // (the mapped knots' included) separates them.
        let mut compared = 0_u32;
        for i in 1..16_u32 {
            let s = f64::from(i) / 16.0;
            let t = t0 + (t1 - t0) * s;
            if (t - t0) / (t1 - t0) != s {
                continue;
            }
            compared += 1;
            let (p, q) = (lifted.eval(t), image.eval(s));
            assert!(
                (p.x - q.x).abs() < 1e-14 && (p.y - q.y).abs() < 1e-14,
                "s = {s}: {p:?} vs {q:?}"
            );
        }
        assert_eq!(
            compared, 5,
            "the exact pull-backs among the 15 interior samples"
        );

        assert!(matches!(
            on_carrier_domain::<f64>(&image, t0, t0),
            Err(SplineError::DomainInvalid { .. })
        ));
    }

    mod interval {
        use geom_core::{Bounds, Interval};

        use super::*;

        /// The sampled parameters of the carrier interval, ends
        /// included (the last an ulp past `t1`, which both sides
        /// evaluate as the closed last span).
        fn samples() -> impl Iterator<Item = f64> {
            let (t0, t1) = CARRIER;
            (0..=16_u32).map(move |i| t0 + (t1 - t0) * (f64::from(i) / 16.0))
        }

        /// The interval half of the lift row: the lifted enclosure
        /// brackets the `f64` domain-door curve at every sample, and
        /// tightly — the lift adds no width (every control bracket is
        /// a point), so what remains is the evaluator's own rounding.
        #[test]
        fn on_carrier_domain_at_interval_brackets_the_f64_curve() {
            let image = image();
            let (t0, t1) = CARRIER;
            let lifted = on_carrier_domain::<Interval>(&image, t0, t1).unwrap();
            let want = image.on_domain(t0, t1).unwrap();
            assert_eq!(lifted.knots_f64_interim(), want.knots_f64_interim());
            for t in samples() {
                let p = lifted.eval(Interval::from_f64(t));
                let q = want.eval(t);
                for (name, enclosure, source) in [("x", p.x, q.x), ("y", p.y, q.y)] {
                    assert!(
                        enclosure.lo() <= source && source <= enclosure.hi(),
                        "t = {t}: lifted {name} = [{}, {}] must contain {source}",
                        enclosure.lo(),
                        enclosure.hi()
                    );
                    assert!(
                        enclosure.hi() - enclosure.lo() < 1e-12,
                        "t = {t}: {name} too wide"
                    );
                }
            }
        }
    }

    /// A degree-2 knot vector on `[0, 1]` with the given interior knots.
    fn deg2(interior: &[f64]) -> KnotVector {
        let mut knots = vec![0.0, 0.0, 0.0];
        knots.extend_from_slice(interior);
        knots.extend([1.0, 1.0, 1.0]);
        KnotVector::clamped(knots, 2).unwrap()
    }

    fn grid_wall(ku: KnotVector, kv: KnotVector) -> NurbsSurface<f64> {
        let (nu, nv) = (ku.control_count(), kv.control_count());
        #[allow(clippy::cast_precision_loss)]
        let control = (0..nu * nv)
            .map(|i| Point3::new((i / nv) as f64, (i % nv) as f64, 0.0))
            .collect();
        NurbsSurface::new(ku, kv, control, vec![1.0; nu * nv]).unwrap()
    }

    /// Odd 64ths: none on the sixteenths grid.
    fn odd64(n: i32) -> Vec<f64> {
        (0..n).map(|j| f64::from(2 * j + 1) / 64.0).collect()
    }

    /// `wall_grid` inserts each direction's DOMAIN sixteenths, a grid
    /// point skipped up to and including `GRID_CLEARANCE` of the
    /// spacing (`2⁻¹²`) from a knot (`0.5`, and `1/16` beside a knot
    /// exactly `2⁻¹²` above it; a knot one ulp further than that above
    /// `3/8` leaves `3/8` standing), and leaves a direction with
    /// `PXN_WALL_SPANS + degree` control points alone while one with a
    /// control point fewer takes the grid. A wall neither direction of
    /// which takes a point reads its own cells.
    #[test]
    fn wall_grid_skips_a_grid_point_up_to_the_clearance_per_direction_with_its_cut_off() {
        let c = geom_core::spline::algebra::grid_clearance(0.0, 1.0, PXN_WALL_SPANS);
        let near = 0.0625 + c;
        let clear = (0.375 + c).next_up();
        let at = deg2(&odd64(15));
        assert_eq!(at.control_count(), PXN_WALL_SPANS + 2);
        let out = wall_grid(&grid_wall(deg2(&[near, clear, 0.5]), at.clone())).unwrap();
        assert_eq!(
            out.knots_u().knots(),
            [
                0.0, 0.0, 0.0, near, 0.125, 0.1875, 0.25, 0.3125, 0.375, clear, 0.4375, 0.5,
                0.5625, 0.625, 0.6875, 0.75, 0.8125, 0.875, 0.9375, 1.0, 1.0, 1.0
            ]
        );
        assert_eq!(out.knots_v().knots(), at.knots());
        let below = deg2(&odd64(14));
        assert_eq!(below.control_count(), 17);
        let out = wall_grid(&grid_wall(deg2(&[0.5]), below)).unwrap();
        assert_eq!(out.knots_v().control_count(), 17 + 15);
        assert!(wall_grid(&grid_wall(at.clone(), at)).is_none());
    }

    /// `wall_grid`'s net has no cliff at any distance of a stated knot
    /// from a grid point: a degree-1 ruled wall bent at every offset of
    /// [`crate::grid_offsets::knot_offsets`] from the first point of the
    /// [`PXN_WALL_SPANS`] grid keeps every span's `u` difference
    /// quotient, as an enclosure, within `1e-9` of the slope of the leg
    /// it lies on. That quotient is the derivative net each cell of the
    /// tube's chart readings is built from. It is read directly because
    /// `NurbsBoxes::speed_sup` takes the largest cell and so hides a
    /// hairline whose rounding happened to land low. A grid point
    /// inserted at a gap `g` beside the bend divides the inserted
    /// point's width by `g`.
    #[test]
    fn the_wall_grid_derivative_net_has_no_cliff_at_any_knot_offset() {
        let (_, offsets) = crate::grid_offsets::knot_offsets(PXN_WALL_SPANS, 1);
        let rows: Vec<(String, f64)> = offsets
            .into_iter()
            .map(|(label, k)| {
                let ku = KnotVector::clamped(vec![0.0, 0.0, k, 1.0, 1.0], 1).unwrap();
                let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
                let control = [0.0, 0.9, 1.0]
                    .into_iter()
                    .flat_map(|x| [Point3::new(x, 0.0, 0.0), Point3::new(x, 1.0, 0.0)])
                    .collect();
                let wall = NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap();
                let fine = wall_grid(&wall).unwrap();
                let (knots, nv) = (fine.knots_u().knots(), fine.knots_v().control_count());
                let x: Vec<geom_core::Interval> =
                    fine.points().iter().step_by(nv).map(|p| p[0]).collect();
                // Degree 1: control `i` sits at knot `i + 1`.
                let worst = (0..x.len() - 1)
                    .map(|i| {
                        let (a, b) = (knots[i + 1], knots[i + 2]);
                        let slope = if b <= k { 0.9 / k } else { 0.1 / (1.0 - k) };
                        let q = (x[i + 1] - x[i])
                            / (geom_core::Interval::from_certified(b)
                                - geom_core::Interval::from_certified(a));
                        (q.hi() - slope).abs().max((q.lo() - slope).abs())
                    })
                    .fold(0.0, |a: f64, e| if e.is_nan() || e > a { e } else { a });
                (label, worst)
            })
            .collect();
        let table: String = rows
            .iter()
            .map(|(label, worst)| format!("\n  {label:>26}: slope error {worst:.4e}"))
            .collect();
        for (label, worst) in &rows {
            assert!(
                *worst < 1e-9,
                "knot {label}: a span left its leg's slope{table}"
            );
        }
    }

    /// **The wall grid encloses the exact refined net** of the described
    /// wall. A rational wall on non-dyadic knots, controls and weights,
    /// refined to the grid: every grid weight contains the exact refined
    /// weight, and every grid point the exact quotient of the refined
    /// homogeneous channels, both by Boehm's insertion in exact
    /// rationals (`crate::exact_q`). A net refined at `f64` and lifted
    /// into the ring holds thin rounded values, which miss the exact
    /// ones wherever an insertion rounds: the row's contrast arm builds
    /// that net (`refine_knots_u/v` at `f64`) and requires it to miss.
    #[test]
    fn the_wall_grid_encloses_the_exact_refined_net() {
        use crate::exact_q::{refine_u, transposed};
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.3, 0.7, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let section = [(0.1, 0.2), (0.9, 1.3), (1.7, -0.6), (2.3, 0.7), (3.1, 0.1)];
        let row_w = [1.0, 0.3, 2.7, 0.6, 1.1];
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for (i, &(x, y)) in section.iter().enumerate() {
            for (z, s) in [(0.0, 1.0), (1.3, 0.7)] {
                control.push(Point3::new(x, y, z));
                weights.push(row_w[i] * s);
            }
        }
        let wall =
            NurbsSurface::new(ku.clone(), kv.clone(), control.clone(), weights.clone()).unwrap();
        let grid = wall_grid(&wall).expect("both directions take the grid");
        // The exact homogeneous channels `(w·x, w·y, w·z, w)`, row-major,
        // refined along `u`, then along `v` through the transpose.
        let net: Vec<[Q; 4]> = control
            .iter()
            .zip(&weights)
            .map(|(p, &w)| {
                let w = Q::of(w);
                let [x, y, z] = [p.x, p.y, p.z].map(|c| w.mul(&Q::of(c)));
                [x, y, z, w]
            })
            .collect();
        let new = |old: &KnotVector, fine: &KnotVector| -> Vec<f64> {
            let mut out = fine.knots().to_vec();
            for k in old.knots() {
                out.remove(out.iter().position(|x| x == k).unwrap());
            }
            out
        };
        let (bu, bv) = (new(&ku, grid.knots_u()), new(&kv, grid.knots_v()));
        let (nu, nv) = (ku.control_count(), kv.control_count());
        let (net, knots_u) = refine_u(&net, (nu, nv), (ku.knots(), 2), &bu);
        let (nu, net) = (nu + bu.len(), transposed(&net, (nu + bu.len(), nv)));
        let (net, knots_v) = refine_u(&net, (nv, nu), (kv.knots(), 1), &bv);
        let net = transposed(&net, (nv + bv.len(), nu));
        assert_eq!(knots_u, grid.knots_u().knots());
        assert_eq!(knots_v, grid.knots_v().knots());
        assert_eq!(net.len(), grid.points().len());
        let misses = |pts: &[[geom_core::Interval; 3]], wts: &[geom_core::Interval]| {
            let mut missed = Vec::new();
            for (k, ((h, p), w)) in net.iter().zip(pts).zip(wts).enumerate() {
                if !h[3].within(w.lo(), w.hi()) {
                    missed.push(format!("weight {k}"));
                }
                for c in 0..3 {
                    if !h[c].div(&h[3]).within(p[c].lo(), p[c].hi()) {
                        missed.push(format!("point {k}.{c}"));
                    }
                }
            }
            missed
        };
        let missed = misses(&grid.points(), &grid.weights());
        assert!(
            missed.is_empty(),
            "the grid missed the exact net at {missed:?}"
        );
        // Contrast: the same refinement at `f64`, lifted thin.
        let lifted = wall
            .refine_knots_u(&bu)
            .unwrap()
            .refine_knots_v(&bv)
            .unwrap();
        let thin = |x: f64| geom_core::Interval::from_certified(x);
        let pts: Vec<_> = lifted
            .control()
            .iter()
            .map(|p| [p.x, p.y, p.z].map(thin))
            .collect();
        let wts: Vec<_> = lifted.weights().iter().map(|&w| thin(w)).collect();
        assert!(
            !misses(&pts, &wts).is_empty(),
            "the f64-refined net must miss the exact one somewhere, or this row cannot tell \
             the two apart"
        );
    }

    /// The exact controls of a polynomial B-spline's restriction to
    /// `[a, b]` inside one knot span: the blossoms `B(a^{p−m}, b^m)` by
    /// de Boor's recursion in exact rationals.
    fn exact_piece(knots: &[f64], p: usize, ctl: &[[f64; 3]], a: f64, b: f64) -> Vec<[Q; 3]> {
        let mid = 0.5 * (a + b);
        let j = (p..knots.len() - p - 1)
            .rfind(|&j| knots[j] <= mid)
            .unwrap();
        (0..=p)
            .map(|m| {
                let mut d: Vec<[Q; 3]> = (0..=p).map(|i| ctl[j - p + i].map(Q::of)).collect();
                for r in 1..=p {
                    let u = Q::of(if r <= p - m { a } else { b });
                    for i in (r..=p).rev() {
                        let lo = Q::of(knots[j - p + i]);
                        let hi = Q::of(knots[j + 1 + i - r]);
                        let alpha = u.sub(&lo).div(&hi.sub(&lo));
                        let beta = Q::of(1.0).sub(&alpha);
                        let prev = d[i - 1].clone();
                        d[i] = [0, 1, 2].map(|k| prev[k].mul(&beta).add(&d[i][k].mul(&alpha)));
                    }
                }
                d[p].clone()
            })
            .collect()
    }

    /// **The tube's piece encloses the true piece at the enclosure
    /// scalar** (`edge_piece`). A polynomial carrier with non-dyadic knots,
    /// controls and a non-unit constant weight, cut to a non-dyadic
    /// interval: at `Interval` every control of the piece contains the
    /// exact rational control of the true restriction, and the piece's
    /// weights are the carrier's weight exactly.
    #[test]
    fn the_tubes_piece_encloses_the_exact_restriction_at_interval() {
        use geom_core::{Bounds, Interval};
        let p = 2;
        let knots = vec![0.0, 0.0, 0.0, 0.3, 0.7, 1.0, 1.0, 1.0];
        let ctl = [
            [0.1, 0.2, 0.3],
            [1.1, 0.7, -0.3],
            [1.9, -0.6, 0.4],
            [2.7, 0.3, 1.3],
            [3.1, 1.7, 0.1],
        ];
        let w = 1.3;
        let carrier = NurbsCurve3::new(
            KnotVector::clamped(knots.clone(), p).unwrap(),
            ctl.iter().map(|c| Point3::new(c[0], c[1], c[2])).collect(),
            vec![w; ctl.len()],
        )
        .unwrap()
        .map_scalar(Interval::from_f64);
        let (a, b) = (0.1, 0.55);
        let piece = edge_piece(&carrier, a, b).unwrap();
        assert_eq!(piece.domain(), (a, b));
        assert!(
            piece
                .weights_f64_interim()
                .iter()
                .all(|&x| x.to_bits() == w.to_bits())
        );
        let exact: Vec<[Q; 3]> = exact_piece(&knots, p, &ctl, a, 0.3)
            .into_iter()
            .chain(exact_piece(&knots, p, &ctl, 0.3, b).into_iter().skip(1))
            .collect();
        let encloses = |c: &[Point3<Interval>]| {
            c.len() == exact.len()
                && c.iter().zip(&exact).all(|(got, want)| {
                    [got.x, got.y, got.z]
                        .iter()
                        .zip(want)
                        .all(|(g, q)| q.within(g.lo(), g.hi()))
                })
        };
        assert!(encloses(piece.control()), "{:?}", piece.control());
    }

    /// **A line against a plane is decided on its measured end value**
    /// (`measured_ends`): a line on one plane whose far end strays `d`
    /// from a second plane certifies at `d = 0`, escalates on the
    /// measured value with `d` inside the ambiguity band
    /// ([`AnalyticRung3Refusal::Escalated`] on [`Rung3Limb::HullValue`]),
    /// and is a measured miss ([`AnalyticRung3Refusal::Limb`]) with `d`
    /// just past `Kε`. Red under a rule that reads the bound as the
    /// kernel's limit or the stall where an end is measured, or that
    /// decides on a ratio of the bound over the net's width.
    #[test]
    fn a_line_against_a_plane_is_decided_on_its_measured_end_value() {
        use geom::{Curve3, Surface};
        use geom_core::Interval;
        use geom_core::{Point3, Vec3};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let lift = Interval::from_f64;
        let floor = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
        .map_scalar(lift);
        let wall = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
        .map_scalar(lift);
        let edge = |d: f64| {
            let line = Curve3::Line {
                origin: Point3::new(0.0, 0.0, 0.0),
                dir: Vec3::new(1.0, d, 0.0),
            }
            .map_scalar(lift);
            analytic_edge_limbs(&line, (lift(0.0), lift(1.0)), &floor, &wall, band)
        };
        assert_eq!(edge(0.0), Ok(()));
        assert!(
            matches!(
                edge(5e-9),
                Err(AnalyticRung3Refusal::Escalated {
                    operand: Some(geom::SurfaceKind::Plane),
                    limb: Rung3Limb::HullValue,
                    ..
                })
            ),
            "{:?}",
            edge(5e-9)
        );
        assert!(
            matches!(
                edge(1.0001e-8),
                Err(AnalyticRung3Refusal::Limb {
                    operand: geom::SurfaceKind::Plane,
                    ..
                })
            ),
            "{:?}",
            edge(1.0001e-8)
        );
    }

    /// **A line just past ε off its plane does not pass at `f64`**
    /// (the dual review's MAJOR on the line-against-a-plane rule). The
    /// plane through `(300.25, −700.5, 0)` with normal `(0.6, 0.8, 0)`,
    /// and a line in it, at both ends, `1.0736e-12` m off it in exact
    /// rational arithmetic on the stored `f64`s — past ε = 1e-12. Read
    /// in round-to-nearest `f64`, `(p − o)·n/|n|` at those ends comes to
    /// `9.663e-13`, inside the band, so an ends rule read at the scalar
    /// passed the edge; read through the data's certified brackets it
    /// cannot. The composite does not clear either (its bound encloses
    /// the true distance), so the ends rule is what decides. Red under an
    /// ends rule read in the scalar's own arithmetic at `f64`.
    #[test]
    fn a_line_just_past_epsilon_off_its_plane_does_not_pass_at_f64() {
        use geom::{Curve3, Surface};
        use geom_core::{Point3, Vec3};
        let band = geom_core::Band::new(1e-12, 1e-11).unwrap();
        let tilted = Surface::Plane {
            origin: Point3::new(300.25, -700.5, 0.0),
            normal: Vec3::new(0.6, 0.8, 0.0),
            u_ref: Vec3::new(0.0, 0.0, 1.0),
        };
        let floor = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let line = Curve3::Line {
            origin: Point3::new(-402.5061327433079, -173.4329004425178, 0.0),
            dir: Vec3::new(-8.0, 6.0, 0.0),
        };
        // The f64 reading the old rule took sits inside the band.
        for t in [0.0, 1.0] {
            let p = line.eval(t);
            let Surface::Plane { origin, normal, .. } = tilted else {
                unreachable!()
            };
            let read = (p - origin).dot(normal).abs() / normal.norm();
            assert!(
                read <= band.zero(),
                "the f64 reading at t = {t} is {read:e}"
            );
        }
        let at_f64 = analytic_edge_limbs(&line, (0.0, 1.0), &tilted, &floor, band);
        assert!(at_f64.is_err(), "f64: {at_f64:?}");
        let lift = geom_core::Interval::from_f64;
        let at_iv = analytic_edge_limbs(
            &line.map_scalar(lift),
            (lift(0.0), lift(1.0)),
            &tilted.map_scalar(lift),
            &floor.map_scalar(lift),
            band,
        );
        assert!(at_iv.is_err(), "Interval: {at_iv:?}");
    }

    /// **An end whose enclosure's floor is just above ε escalates, and is
    /// never the kernel's limit** (`measured_ends`; the band-edge class of
    /// `near_tangent_pierce_poses`' `EDGE_ON_FACES`). A unit line along
    /// `x` leaving the plane `y = 0` at its origin, `y₀ = ε + 2e-16` off
    /// it at its far end: that end's distance is `y₀` exactly in
    /// certification arithmetic, inside the ambiguity band `(ε, Kε]`,
    /// while the composite shows no floor past the zero band and its
    /// bound (`1.0000002e-9`) does not clear — the shape that read as
    /// the kernel's limit, with a net width of zero. The input is ambiguous at this
    /// tolerance, so limb 2 escalates on the measured value
    /// ([`Rung3Limb::HullValue`]) at `f64` and at `Interval`. Red under a
    /// rule that lets the composite's stall stand beside ends that
    /// measure past the zero band.
    #[test]
    fn an_end_measured_a_few_ulps_past_epsilon_escalates_and_is_never_the_limit() {
        use geom::{Curve3, Surface};
        use geom_core::{Point3, Vec3};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let y0 = 1e-9 + 2e-16;
        assert!(y0 > band.zero());
        let wall = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let floor = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let line = Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir: Vec3::new(1.0, y0, 0.0),
        };
        let escalates = |got: &Result<(), AnalyticRung3Refusal>| {
            matches!(
                got,
                Err(AnalyticRung3Refusal::Escalated {
                    operand: Some(geom::SurfaceKind::Plane),
                    limb: Rung3Limb::HullValue,
                    ..
                })
            )
        };
        let at_f64 = analytic_edge_limbs(&line, (0.0, 1.0), &wall, &floor, band);
        assert!(escalates(&at_f64), "f64: {at_f64:?}");
        let lift = geom_core::Interval::from_f64;
        let at_iv = analytic_edge_limbs(
            &line.map_scalar(lift),
            (lift(0.0), lift(1.0)),
            &wall.map_scalar(lift),
            &floor.map_scalar(lift),
            band,
        );
        assert!(escalates(&at_iv), "Interval: {at_iv:?}");
    }

    /// **An end whose enclosure reaches zero is only an upper bound, and
    /// the stall stands** (`measured_ends`). A unit line along `x` whose
    /// height off the plane `y = 0` is held only to `[−2ε, 2ε]`: each
    /// end's distance encloses `[0, 2ε]`, which cannot exclude the line
    /// lying on the plane, so past the band it says nothing about the
    /// input and adds nothing to the composite's stall: limb 2 is
    /// undecided at this input ([`AnalyticRung3Refusal::Stalled`]), not
    /// the measured escalation. Red under an ends rule that escalates on
    /// an enclosure reaching zero.
    #[test]
    fn an_end_enclosure_reaching_zero_past_epsilon_reads_the_stall() {
        use geom::{Curve3, Surface};
        use geom_core::{Interval, Point3, Vec3};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let lift = Interval::from_f64;
        let wall = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
        .map_scalar(lift);
        let floor = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        }
        .map_scalar(lift);
        let line = Curve3::Line {
            origin: Point3::new(lift(0.0), Interval::from_bounds(-2e-9, 2e-9), lift(0.0)),
            dir: Vec3::new(lift(1.0), lift(0.0), lift(0.0)),
        };
        let got = analytic_edge_limbs(&line, (lift(0.0), lift(1.0)), &wall, &floor, band);
        assert!(
            matches!(
                got,
                Err(AnalyticRung3Refusal::Stalled {
                    operand: geom::SurfaceKind::Plane,
                    ..
                })
            ),
            "{got:?}"
        );
    }

    /// **Limb 2 over a parameter box, on its identity** (C2; fork row
    /// 204). Each row reads [`analytic_edge_limbs`] at `Sym<Interval>`
    /// over the box `a ∈ [0.5, 1.5]`, the parameter bracketed in the
    /// session as a drive mints it, and returns the limb's result with
    /// limb 2's decisions (`ssi_hull_sup` and `ssi_hull_value`) as the
    /// recorder saw them.
    fn over_the_box(
        edge: impl FnOnce(SymI) -> Result<(), AnalyticRung3Refusal>,
    ) -> (
        Result<(), AnalyticRung3Refusal>,
        Vec<geom_core::sym::revalue::Decision>,
    ) {
        use geom_core::sym::revalue::record;
        use geom_core::sym::with_session;
        use geom_core::{Interval, ParamSymbol, Sym, SymBudget};
        let budget = SymBudget {
            max_terms: 4096,
            max_degree: 128,
        };
        let ((result, decisions), _) = with_session(budget, || {
            let a = Sym::param_over(
                ParamSymbol::new(1),
                Interval::from_bounds(0.5, 1.5),
                0.5,
                1.5,
            );
            record(|| edge(a))
        });
        let limb2 = decisions
            .into_iter()
            .filter(|d| matches!(d.predicate, "ssi_hull_sup" | "ssi_hull_value"))
            .collect();
        (result, limb2)
    }

    type SymI = geom_core::Sym<geom_core::Interval>;

    fn lit(x: f64) -> SymI {
        <SymI as geom_core::Real>::from_f64(x)
    }

    fn point(x: f64, y: f64, z: SymI) -> geom_core::Point3<SymI> {
        geom_core::Point3::new(lit(x), lit(y), z)
    }

    fn direction(x: f64, y: f64, z: f64) -> geom_core::Vec3<SymI> {
        geom_core::Vec3::new(lit(x), lit(y), lit(z))
    }

    /// The wall `y = 0`, the second operand of every row: each carrier
    /// lies on it.
    fn wall() -> geom::Surface<SymI> {
        geom::Surface::Plane {
            origin: point(0.0, 0.0, lit(0.0)),
            normal: direction(0.0, 1.0, 0.0),
            u_ref: direction(1.0, 0.0, 0.0),
        }
    }

    /// **A line derived from the parameter, on a plane derived from the
    /// same parameter, certifies the whole macroscopic box in one leaf**:
    /// the carrier and the floor both sit at height `a` for `a` over a
    /// metre, and the composite, reading each over the box, puts the
    /// line about a metre off its floor — the enclosure does not clear,
    /// so the numeric channel cannot decide it. The free-parameter
    /// residual `(p(t) − o)·n = a − a` is the zero form, so limb 2
    /// decides `Zero` against each operand on its first decision, the
    /// symbolic tier's, and nothing else is asked of it. Red where the
    /// limb lifts the carrier out of the scalar before deciding, or
    /// reads the stall as a kernel limit.
    #[test]
    fn a_parameter_derived_line_on_its_plane_certifies_the_box_in_one_leaf() {
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let (result, limb2) = over_the_box(|a| {
            let floor = geom::Surface::Plane {
                origin: point(0.0, 0.0, a),
                normal: direction(0.0, 0.0, 1.0),
                u_ref: direction(1.0, 0.0, 0.0),
            };
            let line = geom::Curve3::Line {
                origin: point(0.0, 0.0, a),
                dir: direction(1.0, 0.0, 0.0),
            };
            analytic_edge_limbs(&line, (lit(0.0), lit(1.0)), &floor, &wall(), band)
        });
        assert_eq!(result, Ok(()));
        assert_eq!(
            limb2
                .iter()
                .map(|d| (d.predicate, d.sign, d.proven))
                .collect::<Vec<_>>(),
            vec![
                ("ssi_hull_sup", Some(Sign::Zero), true),
                ("ssi_hull_sup", Some(Sign::Zero), true),
            ],
            "one decision per operand, each a theorem"
        );
    }

    /// The same over a curved carrier, where no end rule applies: a
    /// circle of radius `a` about the axis of a cylinder of radius `a`
    /// certifies the box on the residual `(ρ² − a²)/2a`, which is
    /// `a²(cos² t + sin² t) − a²` over `2a` — the zero form.
    #[test]
    fn a_parameter_derived_circle_on_its_cylinder_certifies_the_box_in_one_leaf() {
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let (result, limb2) = over_the_box(|a| {
            let wall = geom::Surface::Cylinder {
                origin: point(0.0, 0.0, lit(0.0)),
                axis: direction(0.0, 0.0, 1.0),
                radius: a,
                u_ref: direction(1.0, 0.0, 0.0),
            };
            let floor = geom::Surface::Plane {
                origin: point(0.0, 0.0, lit(0.0)),
                normal: direction(0.0, 0.0, 1.0),
                u_ref: direction(1.0, 0.0, 0.0),
            };
            let rim = geom::Curve3::Circle {
                center: point(0.0, 0.0, lit(0.0)),
                axis: direction(0.0, 0.0, 1.0),
                radius: a,
                u_ref: direction(1.0, 0.0, 0.0),
            };
            analytic_edge_limbs(&rim, (lit(0.0), lit(1.0)), &wall, &floor, band)
        });
        assert_eq!(result, Ok(()));
        assert!(
            limb2
                .iter()
                .any(|d| d.predicate == "ssi_hull_sup" && d.proven),
            "the cylinder's decision is a theorem: {limb2:?}"
        );
    }

    /// **The control: a coincidence at the nominal is not an identity.**
    /// The line sits at height `a` and its floor at height `1`, so the
    /// two meet only at the box's nominal `a = 1`. The residual is
    /// `a − 1`, not the zero form, so nothing is proved; the enclosure
    /// over the box does not clear, and the ends, measured over the box,
    /// enclose `[0, 0.5]` — reaching zero, only an upper bound — so they
    /// add nothing and limb 2 is undecided over the box
    /// ([`AnalyticRung3Refusal::Stalled`]), the classifier's terminal bit
    /// false — a smaller box is the remedy — and claims no kernel limit.
    #[test]
    fn a_coincidence_at_the_nominal_only_does_not_certify_the_box() {
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let (result, limb2) = over_the_box(|a| {
            let floor = geom::Surface::Plane {
                origin: point(0.0, 0.0, lit(1.0)),
                normal: direction(0.0, 0.0, 1.0),
                u_ref: direction(1.0, 0.0, 0.0),
            };
            let line = geom::Curve3::Line {
                origin: point(0.0, 0.0, a),
                dir: direction(1.0, 0.0, 0.0),
            };
            analytic_edge_limbs(&line, (lit(0.0), lit(1.0)), &floor, &wall(), band)
        });
        let Err(AnalyticRung3Refusal::Stalled {
            operand: geom::SurfaceKind::Plane,
            cause,
        }) = &result
        else {
            panic!("a coincidence at the nominal is undecided over the box: {result:?}");
        };
        assert!(!cause.terminal_sliver, "{cause:?}");
        assert!(
            limb2.iter().all(|d| !d.proven),
            "nothing is proved: {limb2:?}"
        );
    }

    /// **The stall, where nothing is measured** (C2's enclosure arm): a
    /// unit circle at height `a` against the floor at height `1`, which
    /// it meets only at the box's nominal. No ends rule reads a circle,
    /// and the composite's floor over the box reaches zero, so nothing
    /// measured lies outside the zero band: limb 2 is undecided over the
    /// box ([`AnalyticRung3Refusal::Stalled`]), the classifier's terminal
    /// bit false, and claims no kernel limit.
    #[test]
    fn a_circle_meeting_its_plane_at_the_nominal_only_stalls_over_the_box() {
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let (result, limb2) = over_the_box(|a| {
            let floor = geom::Surface::Plane {
                origin: point(0.0, 0.0, lit(1.0)),
                normal: direction(0.0, 0.0, 1.0),
                u_ref: direction(1.0, 0.0, 0.0),
            };
            let rim = geom::Curve3::Circle {
                center: point(0.0, 0.0, a),
                axis: direction(0.0, 0.0, 1.0),
                radius: lit(1.0),
                u_ref: direction(1.0, 0.0, 0.0),
            };
            analytic_edge_limbs(&rim, (lit(0.0), lit(1.0)), &floor, &wall(), band)
        });
        let Err(AnalyticRung3Refusal::Stalled {
            operand: geom::SurfaceKind::Plane,
            cause,
        }) = &result
        else {
            panic!("a circle meeting its plane at the nominal stalls over the box: {result:?}");
        };
        assert!(!cause.terminal_sliver, "{cause:?}");
        assert!(
            limb2.iter().all(|d| !d.proven),
            "nothing is proved: {limb2:?}"
        );
    }
}

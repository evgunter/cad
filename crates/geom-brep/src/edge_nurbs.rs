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
//! Everything here is the rung-3 SSI certificate ([`crate::ssi`]'s
//! `certify_rung3` door) applied to a **declared** carrier instead of
//! a marched one. That door already answers exactly the three
//! questions this edge class raises, so the lane supplies the one
//! ingredient a declared carrier is missing — the chart image — and
//! delegates:
//!
//! * **on-PLANE residual**: closed form, the analytic operand's
//!   linearized implicit residual at the fixed schedule plus its
//!   certified composite hull sup over the whole span.
//! * **on-NURBS residual**: `|C(t) − S(u*, v*)|` at a **certified foot
//!   point** ([`geom::NurbsSurface::project`], D9-fixed),
//!   the foot's own orthogonality residuals banded alongside, and the
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
//! default signatures because the capability is injected at a separate
//! door ([`crate::certify::NurbsLane`]) rather than raised into the
//! shared machinery.
//!
//! **The symbolic tier rides the same bound and needs no arm of its
//! own** (`geom_core::sym`): `Sym<T>` implements
//! [`geom_core::Bounds`], `geom_core::CertifiedEnclosure` and
//! `geom_core::Decide` exactly when its base scalar does, so
//! `Sym<T>` satisfies this signature for every certifying `T` — the
//! limbs are the base scalar's, run at `Sym<T>`. The tier changes how
//! one class of margin decides and nothing about the certificate, and
//! a bound says that without an impl to write.

use core::num::NonZeroUsize;
use geom::{NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::predicate::KERNEL_OR_FILE_DEFECT_ENDING;
use geom_core::spline::algebra::{GridSkip, domain_grid_points};
use geom_core::spline::{KnotVector, KnotVectorIssue, SplineError};
use geom_core::{Band, Bounds, Decide, Indeterminate, Point2, Point3, Readable, Real, Vec3};

use crate::certify::{CERT_SAMPLES, CertCheck, recourse, schedule_fraction, schedule_param};
use crate::recourse::{Reading, Refused, RefusedArm};
use crate::ssi::{SsiError, SsiLimb, SsiOperand, SsiTube, TubeScale, certify_rung3};

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
    /// A certificate limb exceeded ε, with the measured bound. **The
    /// declare-and-check refusal**: the file's carrier is not on both
    /// surfaces to the run's tolerance, and this is by how much.
    Limb {
        /// Which limb refused.
        limb: SsiLimb,
        /// The measured bound, in meters.
        value: f64,
    },
    /// The uniqueness tube's transversality is not certified clear of
    /// the zero band — a genuine sliver of the operand pair along the
    /// locus at this tolerance (F6: escalate, never guess).
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
    /// The per-sample transversality margin escalated: the same
    /// decision as [`NotTransverse`](Self::NotTransverse), undecided.
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
    /// The reported transversality — the minimum sine over interior
    /// samples that each decided transverse — came out poisoned. No
    /// geometry and no tolerance reaches it, so it is a kernel defect.
    ReportedTransversalityPoisoned(Indeterminate),
    /// The (carrier, operand) shape is outside the lane's certified
    /// inventory, named exactly. A routing boundary (C12.1), never a
    /// runtime fallback.
    Unsupported {
        /// The refused class, named.
        what: &'static str,
    },
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
    /// [`SplineError::KnotVectorInvalid`]: `f64` rounding collapsed the
    /// image's knots onto a domain too narrow for the magnitude of its
    /// ends, and the clamp contract refused the result by this clause.
    Collapse(KnotVectorIssue),
}

impl CarrierDomainRefusal {
    /// The domain door's refusal on `[lo, hi]`, typed.
    ///
    /// # Panics
    ///
    /// On a weight or count arm of [`SplineError`], which the door
    /// never returns: it re-expresses the knots alone and carries the
    /// validated net and weights over verbatim.
    fn of(lo: f64, hi: f64, e: &SplineError) -> Self {
        let fault = match e {
            SplineError::DomainInvalid { .. } => CarrierDomainFault::Interval,
            SplineError::KnotVectorInvalid { reason } => CarrierDomainFault::Collapse(*reason),
            SplineError::NonPositiveWeight { .. }
            | SplineError::NonFiniteWeight { .. }
            | SplineError::ControlCountMismatch { .. }
            | SplineError::WeightCountMismatch { .. } => unreachable!(
                "the domain door re-expresses knots only, so it cannot refuse a weight or a \
                 count: {e:?}"
            ),
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
            CarrierDomainFault::Collapse(_) => write!(
                f,
                "f64 rounding collapsed the chart image's knots onto the carrier's parameter \
                 domain [{lo}, {hi}], which is too narrow for the magnitude of its ends"
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
    /// The ending this refusal's decision gives it, read at `reading`
    /// ([`recourse`]), or `None` for a refusal that is no decision's
    /// refused arm. `Display` renders the payload alone, as
    /// [`crate::CertifyError`]'s does, and the door appends this.
    ///
    /// The per-sample transversality and the uniqueness tube are the
    /// `Transversality` decision, whose band-decided arms end alike (D4
    /// ¶1 (iv)); a certificate limb's refusal, definite or escalated,
    /// ends by its limb's decision ([`SsiLimb::check`]).
    #[must_use]
    pub fn ending(&self, reading: Reading) -> Option<String> {
        self.decision()
            .map(|(check, arm)| recourse(check, arm, reading))
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
            Self::Limb { limb, .. } => (limb.check(), RefusedArm::SignCertain),
            // The tube's margin is the lane's transversality over the
            // chain (`ssi_tube_transversality`), and this refusal is its
            // decided verdict.
            Self::TubeStraddles { verdict, .. } => (CertCheck::Transversality, verdict.arm()),
            Self::Escalated { limb, cause } => (limb.check(), RefusedArm::Undecided(cause)),
            Self::ReportedTransversalityPoisoned(cause) => (
                CertCheck::PlaneNurbsReportedTransversality,
                RefusedArm::Undecided(cause),
            ),
            Self::FootPointInconclusive { .. }
            | Self::PcurveFit
            | Self::CarrierDomain(_)
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
            Self::NotTransverse { sample, .. } => write!(
                f,
                "the plane and the NURBS wall have coincident tangent planes at interior \
                 sample {sample}, where the edge's description says they cross"
            ),
            Self::TransversalityEscalated { sample, cause } => write!(
                f,
                "whether the plane and the NURBS wall cross at interior sample {sample} is too \
                 close to call: {}",
                cause.payload()
            ),
            Self::PcurveFit => write!(f, "{PCURVE_FIT_REFUSAL}. {KERNEL_OR_FILE_DEFECT_ENDING}"),
            Self::CarrierDomain(refusal) => write!(f, "{refusal}"),
            Self::Limb { limb, value } => write!(
                f,
                "{} measured {value:e} m against the run tolerance — the declared carrier \
                 is not on both surfaces",
                limb.name()
            ),
            Self::TubeStraddles { verdict, boxes } => write!(
                f,
                "the uniqueness tube's transversality is not certified clear of the zero band \
                 over {boxes} boxes of the chain — a sliver of the plane/NURBS pair along the \
                 locus at this tolerance; the certificate's proven clearance from zero is {:e} \
                 m, which is the bound it could prove and not the sliver's own extent",
                verdict.margin()
            ),
            Self::Escalated { limb, cause } => {
                write!(f, "{} escalated: {}", limb.name(), cause.payload())
            }
            Self::ReportedTransversalityPoisoned(cause) => write!(
                f,
                "every interior sample decided the plane and the NURBS wall cross, yet \
                 their reported minimum crossing angle is unreadable: {}",
                cause.payload()
            ),
            Self::Unsupported { what } => write!(f, "outside the plane × NURBS lane: {what}"),
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
/// use geom::{NurbsCurve3, NurbsSurface, Surface};
/// fn certified(
///     carrier: &NurbsCurve3<Dual64>,
///     plane: &Surface<Dual64>,
///     wall: &NurbsSurface<Dual64>,
///     extent: Dual64,
///     band: Band,
/// ) {
///     let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
/// }
/// ```
///
/// The row above fails on the BOUND and not on a path or a spelling,
/// and the row below is what says so: it differs in the scalar alone,
/// every name resolving the same way.
///
/// ```
/// use geom_core::Band;
/// use geom::{NurbsCurve3, NurbsSurface, Surface};
/// fn certified(
///     carrier: &NurbsCurve3<f64>,
///     plane: &Surface<f64>,
///     wall: &NurbsSurface<f64>,
///     extent: f64,
///     band: Band,
/// ) {
///     let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
/// }
/// ```
///
/// The symbolic tier is admitted by the same bound and needs no arm of
/// its own, which is the whole of what a per-scalar impl would have
/// said here:
///
/// ```
/// use geom_core::{Band, Sym};
/// use geom::{NurbsCurve3, NurbsSurface, Surface};
/// fn symbolic(
///     carrier: &NurbsCurve3<Sym<f64>>,
///     plane: &Surface<Sym<f64>>,
///     wall: &NurbsSurface<Sym<f64>>,
///     extent: Sym<f64>,
///     band: Band,
/// ) {
///     let _ = geom_brep::plane_nurbs_limbs(carrier, plane, wall, extent, band);
/// }
/// ```
pub fn plane_nurbs_limbs<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    plane: &Surface<T>,
    wall: &NurbsSurface<T>,
    extent: T,
    band: Band,
) -> Result<PlaneNurbsLimbs<T>, PlaneNurbsRefusal> {
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
    // The wall the tube localizes on, with its chart speeds minted at the
    // door: a wall constant along an axis, or with no finite speed bound
    // along one, has no chart a length in metres can cross into, and the
    // schedule below would only meet that as a foot point or a sine that
    // cannot be stated.
    let localized = localized(wall);
    let wall_op = SsiOperand::nurbs(&localized).map_err(refusal)?;

    // ---- The fixed schedule: foot points, and the normal angle. ----
    // The feet and the image are `chart_image`'s, which is also the
    // pcurve mint's producer: ONE derivation of this image exists in
    // the tree, and this lane certifies the same bits the mint stores.
    // The transversality sweep rides the same walk as a per-sample
    // hook, so the ORDER in which the two refusals can fire is exactly
    // what it was when the two loops were one.
    let mut min_sin = T::from_f64(f64::MAX);
    let pcurve = chart_image(carrier, wall, |i, foot| {
        // Transversality at the INTERIOR samples only: an endpoint is
        // shared with the neighbouring edges, where a vanishing angle
        // is a vertex fact rather than this edge's — the analytic
        // `Intersection` arm's own convention, kept verbatim.
        if i == 0 || i == PXN_FIT_SAMPLES - 1 {
            return Ok(());
        }
        let jet = wall.ders(T::from_f64(foot.x), T::from_f64(foot.y));
        let sin_theta = normal_angle_sine(normal, jet.du.cross(jet.dv));
        // Metered at the analytic side's lever arm: a plane's own
        // curvature arm is infinite, so the honest arm is the
        // edge's spatial extent — the same meter the analytic
        // `Intersection` arm hands `classify_dihedral`.
        let margin = geom_core::Margin::levered(sin_theta, extent);
        match crate::dihedral::decide_reported("plane_nurbs_transversality", margin, band) {
            Ok(decided) => {
                if let Some(verdict) = Refused::of(decided, band) {
                    return Err(PlaneNurbsRefusal::NotTransverse { sample: i, verdict });
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
    let cert = certify_rung3(
        carrier,
        Some(&pcurve),
        &SsiOperand::Analytic(plane),
        &wall_op,
        TubeScale::uniform(extent),
        band,
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
pub(crate) fn chart_image<T, F>(
    carrier: &NurbsCurve3<T>,
    wall: &NurbsSurface<T>,
    mut per_sample: F,
) -> Result<NurbsCurve2<T>, PlaneNurbsRefusal>
where
    T: Decide + Bounds + geom_core::CertifiedEnclosure,
    F: FnMut(u32, Point2<f64>) -> Result<(), PlaneNurbsRefusal>,
{
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
    let (t0, t1) = carrier.domain();
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
    let mut uv = Vec::with_capacity(PXN_FIT_SAMPLES as usize);
    let mut params = Vec::with_capacity(PXN_FIT_SAMPLES as usize);
    for i in 0..PXN_FIT_SAMPLES {
        let frac = schedule_fraction(i, PXN_FIT_SAMPLES);
        let t = schedule_param(t0, t1, i, PXN_FIT_SAMPLES);
        let p = carrier.eval(T::from_f64(t));
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

/// How many knot spans per direction the wall is refined to before the
/// hull and tube limbs run. A **structure** choice in C6's `f64` lane
/// (never a decision), the surface-side twin of `ssi::certify`'s
/// `SSI_CERT_SPANS` for the carrier.
pub const PXN_WALL_SPANS: usize = 16;

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

/// The wall refined so the uniqueness tube can localize.
///
/// The tube's chart enclosures read `NurbsBoxes` derivative boxes,
/// which are **cell-granular**: a box narrower than a knot span still
/// reports that whole span's derivative variation. A one-span quarter
/// cylinder therefore reports the derivative swinging through 90° no
/// matter how far the tube ladder halves its radius, and the enclosure
/// straddles zero forever — a resolution artifact of the operand's
/// knot structure, not a sliver of the pair. Knot refinement is exact
/// in ℝ (the surface's locus and parameterization are unchanged), so
/// spending it here buys localization for free.
///
/// Already-fine patches are returned unchanged, and a refusing knot
/// algebra falls back to the original — a coarser enclosure can only
/// make the certificate harder to pass, never unsound.
fn localized<T: Real>(wall: &NurbsSurface<T>) -> NurbsSurface<T> {
    fn breaks(kv: &geom_core::spline::KnotVector) -> Vec<f64> {
        if kv.control_count() >= PXN_WALL_SPANS + kv.degree() {
            return Vec::new();
        }
        domain_grid_points(kv, PXN_WALL_SPANS, GridSkip::BitEqual)
    }
    let add_u = breaks(wall.knots_u());
    let add_v = breaks(wall.knots_v());
    let refined = wall.refine_knots_u(&add_u).unwrap_or_else(|_| wall.clone());
    refined
        .refine_knots_v(&add_v)
        .unwrap_or_else(|_| refined.clone())
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
/// exact, interior affine, at `f64`), then the control net through
/// `T::from_f64` as a structural lift. The rescale is the only step
/// that can refuse — the lift carries a validated curve's net verbatim
/// — so the `Result` is the knot door's alone.
///
/// The knot map's own rounding is not a soundness question — the image
/// is evidence that limb 2 bounds, not a certified quantity (module
/// docs).
fn on_carrier_domain<T: Real>(
    image: &NurbsCurve2<f64>,
    t0: f64,
    t1: f64,
) -> Result<NurbsCurve2<T>, SplineError> {
    Ok(image.on_domain(t0, t1)?.map_scalar(T::from_f64))
}

/// The SSI refusal, in this lane's vocabulary.
fn refusal(e: SsiError) -> PlaneNurbsRefusal {
    match e {
        SsiError::CertificateLimb { limb, value } => PlaneNurbsRefusal::Limb { limb, value },
        SsiError::TubeStraddles { verdict, boxes } => {
            PlaneNurbsRefusal::TubeStraddles { verdict, boxes }
        }
        SsiError::CertificateEscalated { limb, cause } => {
            PlaneNurbsRefusal::Escalated { limb, cause }
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
        SsiError::ChartSpeed(r) => PlaneNurbsRefusal::Unsupported { what: r.what() },
        _ => PlaneNurbsRefusal::Unsupported {
            what: "the rung-3 certificate refused for a reason outside this lane's vocabulary",
        },
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use geom_core::spline::KnotVector;

    use super::*;

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

    /// The carrier interval the rows use: `0.3 + (0.9 − 0.3)` is an
    /// ulp above `0.9`, so a computed end would miss it.
    const CARRIER: (f64, f64) = (0.3, 0.9);

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
        assert_eq!(lifted.knots(), &image.knots().on_domain(t0, t1).unwrap());
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
            assert_eq!(lifted.knots(), want.knots());
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

    fn wall(ku: KnotVector, kv: KnotVector) -> NurbsSurface<f64> {
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

    /// `localized` inserts each direction's DOMAIN sixteenths, skipping
    /// a grid point only where a knot sits on it bit for bit (`0.5`;
    /// a knot one ulp above `1/16` does NOT suppress `1/16`), and
    /// leaves a direction with `PXN_WALL_SPANS + degree` control points
    /// alone while one with a control point fewer takes the grid.
    #[test]
    fn localized_inserts_the_domain_grid_per_direction_with_its_cut_off() {
        let near = f64::from_bits(0.0625f64.to_bits() + 1);
        let at = deg2(&odd64(15));
        assert_eq!(at.control_count(), PXN_WALL_SPANS + 2);
        let out = localized(&wall(deg2(&[near, 0.5]), at.clone()));
        assert_eq!(
            out.knots_u().knots(),
            [
                0.0, 0.0, 0.0, 0.0625, near, 0.125, 0.1875, 0.25, 0.3125, 0.375, 0.4375, 0.5,
                0.5625, 0.625, 0.6875, 0.75, 0.8125, 0.875, 0.9375, 1.0, 1.0, 1.0
            ]
        );
        assert_eq!(out.knots_v().knots(), at.knots());
        let below = deg2(&odd64(14));
        assert_eq!(below.control_count(), 17);
        let out = localized(&wall(deg2(&[0.5]), below));
        assert_eq!(out.knots_v().control_count(), 17 + 15);
    }
}

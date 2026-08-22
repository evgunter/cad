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
//! ratified shape (Evan, PR #264) and it is what this module
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
//! # The static lane split
//!
//! Limbs 2 and 3 are C9-ring bounds and the foot point is a bracket
//! read, so the honest signature is `T: Decide + Bounds`
//! (`geom_core::Bounds`'s compound-allowlist note, M6-2). The split is
//! expressed as [`EdgeNurbsLane`] in the ratified
//! [`crate::PcurveFittedLane`] / `topo::props::PropsQuadLane` shape:
//! certified impls for `f64`, the telemetry probe and the interval
//! scalar, a **refusing** impl for `geom_core::Dual` (which may not
//! certify — D1, 2026-08-19; it carries the value channel's bracket
//! and that is not the right to mint a C9-ring bound), and `Bounds`
//! kept out of `topo`'s signatures.

use geom::{NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::{Band, Bounds, Decide, Indeterminate, Point2, Real, Vec3};

use crate::certify::CERT_SAMPLES;
use crate::ssi::{SsiError, SsiLimb, SsiOperand, TubeScale, certify_rung3};

/// What the plane × NURBS lane proved, in meters unless noted.
#[derive(Clone, Copy, Debug)]
pub struct PlaneNurbsLimbs<T: Real> {
    /// Limb 1: the largest on-locus residual over the schedule, over
    /// **both** operands (the sampled max — it steers, it does not
    /// certify).
    pub on_locus_max: T,
    /// Limb 2: the certified sup-norm bound over the whole span, over
    /// both operands. This is the number that certifies.
    pub hull_sup: T,
    /// Limb 3: the certified uniqueness-tube radius.
    pub tube_radius: T,
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
pub enum PlaneNurbsRefusal {
    /// This scalar has no certified lane ([`EdgeNurbsLane`]'s refusing
    /// side): it may not certify, so the C9 ring the hull bounds live
    /// in is not reachable from it (D1, 2026-08-19 — a dual carries a
    /// bracket but not the right to certify with it).
    LaneUnsupported {
        /// The scalar lane, named.
        scalar: &'static str,
    },
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
    },
    /// The chart image could not be interpolated through the schedule's
    /// foot points (a degenerate parameterization — coincident feet, a
    /// singular collocation system).
    PcurveFit,
    /// A certificate limb exceeded ε, with the measured bound. **The
    /// declare-and-check refusal**: the file's carrier is not on both
    /// surfaces to the run's tolerance, and this is by how much.
    Limb {
        /// Which limb refused.
        limb: SsiLimb,
        /// The measured bound, in meters.
        value: f64,
    },
    /// The uniqueness tube's transversality straddles zero — a genuine
    /// sliver of the operand pair along the locus (F6: escalate, never
    /// guess).
    TubeStraddles {
        /// The transversality enclosure's **certified clearance from
        /// zero**, levered — NOT a measurement of how far the sliver
        /// straddles. An enclosure that contains zero has certified
        /// clearance exactly `0.0` by construction (rung 3's
        /// `zero_free_lower_bound`), and containing zero is what this
        /// refusal reports, so this field is `0.0` on every straddling
        /// refusal and a small positive number only on the levered
        /// rungs that failed the band instead. Read it as the bound
        /// the certificate could prove, never as the geometry's own
        /// extent; the informative companion is `boxes`.
        certified_clearance: f64,
        /// How many boxes of the tube's chain the clearance above was
        /// certified over — the resolution the verdict was reached at.
        boxes: u32,
    },
    /// A margin escalated inside the certificate.
    Escalated(Indeterminate),
    /// The (carrier, operand) shape is outside the lane's certified
    /// inventory, named exactly. A routing boundary (C12.1), never a
    /// runtime fallback.
    Unsupported {
        /// The refused class, named.
        what: &'static str,
    },
}

impl core::fmt::Display for PlaneNurbsRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::LaneUnsupported { scalar } => write!(
                f,
                "the plane × NURBS edge lane has no certified derivation at the {scalar} \
                 scalar (it may not certify, so the exact-arithmetic ring the hull bounds live in \
                 is out of reach — replay the body at f64, the telemetry probe, or the interval \
                 scalar)"
            ),
            Self::FootPointInconclusive {
                sample,
                last_distance,
            } => write!(
                f,
                "the foot-point projection did not converge at schedule sample {sample} \
                 (last distance {last_distance:e} m)"
            ),
            Self::NotTransverse { sample } => write!(
                f,
                "the plane and the NURBS wall have coincident tangent planes at interior \
                 sample {sample} — the Intersection transversality precondition fails (D2); {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::PcurveFit => write!(
                f,
                "the chart image could not be interpolated through the schedule's foot \
                 points (a degenerate parameterization)"
            ),
            Self::Limb { limb, value } => write!(
                f,
                "{} measured {value:e} m against the run tolerance — the declared carrier \
                 is not on both surfaces",
                limb.name()
            ),
            Self::TubeStraddles {
                certified_clearance,
                boxes,
            } => write!(
                f,
                "the uniqueness tube's transversality enclosure contains zero over {boxes} \
                 boxes of the chain — a genuine sliver of the plane/NURBS pair along the \
                 locus; the certificate's proven clearance from zero is {certified_clearance:e} \
                 m, which is the bound it could prove and not the sliver's own extent"
            ),
            Self::Escalated(diag) => write!(f, "a plane × NURBS limb margin escalated: {diag}"),
            Self::Unsupported { what } => write!(f, "outside the plane × NURBS lane: {what}"),
        }
    }
}

/// **Which scalars can derive the plane × NURBS edge certificate** —
/// the static lane split (module docs).
pub trait EdgeNurbsLane: Decide {
    /// Certifies the declared `carrier` as the intersection locus of
    /// `plane` and `wall` (module docs for the three limbs).
    ///
    /// `extent` is the edge's honest spatial extent — the lever arm the
    /// uniqueness tube's margin is metered at, and the scale its radius
    /// ladder starts from.
    ///
    /// # Errors
    ///
    /// [`PlaneNurbsRefusal`], carrying the measured bound whenever one
    /// exists. The refusing lane returns
    /// [`PlaneNurbsRefusal::LaneUnsupported`] and instantiates none of
    /// the certified machinery.
    fn plane_nurbs_limbs(
        carrier: &NurbsCurve3<Self>,
        plane: &Surface<Self>,
        wall: &NurbsSurface<Self>,
        extent: Self,
        band: Band,
    ) -> Result<PlaneNurbsLimbs<Self>, PlaneNurbsRefusal>;

    /// The lane's name, for the typed refusal's text.
    fn lane_name() -> &'static str;
}

/// The certified lane's body, shared by every bracket-carrying scalar.
///
/// The operand ORDER is load-bearing exactly as it is in the fitted
/// pcurve lane: the NURBS wall is operand **b**, because
/// `certify_branch` reads the chart image of `b`, and the image this
/// lane derives is the carrier's foot path on the wall.
fn lane<T: Decide + Bounds + geom_core::CertifiedEnclosure>(
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
            what: "the mvfs placeholder is a mid-surgery 'no description yet' fact, never a \
                   surface to certify against",
        });
    }
    let (t0, t1) = carrier.domain();

    // ---- The fixed schedule: foot points, and the normal angle. ----
    // D9-fixed and a SUPERSET of the certificate's own schedule
    // ([`PXN_FIT_SAMPLES`]), so limb 1 re-projects at parameters the
    // chart image passes through exactly.
    let mut uv = Vec::with_capacity(PXN_FIT_SAMPLES as usize);
    let mut params = Vec::with_capacity(PXN_FIT_SAMPLES as usize);
    let mut min_sin = T::from_f64(f64::MAX);
    for i in 0..PXN_FIT_SAMPLES {
        let frac = f64::from(i) / f64::from(PXN_FIT_SAMPLES - 1);
        let t = t0 + (t1 - t0) * frac;
        let p = carrier.eval(T::from_f64(t));
        let proj = wall
            .project(p)
            .map_err(|e| PlaneNurbsRefusal::FootPointInconclusive {
                sample: i,
                last_distance: e.last_distance,
            })?;
        uv.push(Point2::new(proj.u, proj.v));
        params.push(frac);
        // Transversality at the INTERIOR samples only: an endpoint is
        // shared with the neighbouring edges, where a vanishing angle
        // is a vertex fact rather than this edge's — the analytic
        // `Intersection` arm's own convention, kept verbatim.
        if i > 0 && i < PXN_FIT_SAMPLES - 1 {
            let jet = wall.ders(T::from_f64(proj.u), T::from_f64(proj.v));
            let sin_theta = normal_angle_sine(normal, jet.du.cross(jet.dv));
            // Metered at the analytic side's lever arm: a plane's own
            // curvature arm is infinite, so the honest arm is the
            // edge's spatial extent — the same meter the analytic
            // `Intersection` arm hands `classify_dihedral`.
            let margin = geom_core::Margin::levered(sin_theta, extent);
            match crate::dihedral::decide("plane_nurbs_transversality", margin, band) {
                Ok(geom_core::Sign::Positive) => {}
                Ok(geom_core::Sign::Zero | geom_core::Sign::Negative) => {
                    return Err(PlaneNurbsRefusal::NotTransverse { sample: i });
                }
                Err(cause) => return Err(PlaneNurbsRefusal::Escalated(cause)),
            }
            // `Real::min` PROPAGATES poison (unlike `f64::min`, which
            // returns the non-NaN operand), so a poisoned sine cannot
            // be dropped out of this fold — it reaches the guard below.
            min_sin = min_sin.min(sin_theta);
        }
    }
    // FAIL LOUD on a poisoned aggregate. A NaN sine cannot reach here
    // today — the per-sample `decide` above escalates on its levered
    // margin first — but the reported transversality must not depend on
    // that shield holding across future edits to the gate. A poison
    // that ever survives the fold refuses TYPED, carrying the
    // `Invalid` diagnostic, instead of riding out as a reported number
    // no caller can tell from a measurement.
    if min_sin.is_poison() {
        return Err(PlaneNurbsRefusal::Escalated(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("plane_nurbs_transversality_reported"),
        }));
    }

    // ---- The derived chart image, on the carrier's own parameter. ----
    // Interpolation (not approximation) so the image is exact at every
    // foot; the interpolation runs in C6's `f64` structure lane and is
    // lifted, and limb 2 is what certifies the result (module docs).
    let image = NurbsCurve2::<f64>::interpolate_with_params(&uv, PXN_IMAGE_DEGREE, &params)
        .map_err(|_| PlaneNurbsRefusal::PcurveFit)?;
    let pcurve = on_carrier_domain(&image, t0, t1).ok_or(PlaneNurbsRefusal::PcurveFit)?;

    // ---- The rung-3 door: all three limbs, both operands. ----
    let localized = localized(wall);
    let cert = certify_rung3(
        carrier,
        Some(&pcurve),
        &SsiOperand::Analytic(plane),
        &SsiOperand::Nurbs(&localized),
        TubeScale::uniform(extent),
        band,
    )
    .map_err(refusal)?;
    Ok(PlaneNurbsLimbs {
        on_locus_max: cert.on_locus_max,
        hull_sup: cert.hull_sup,
        tube_radius: cert.tube_radius,
        tube_transversality: cert.tube_transversality,
        tube_boxes: cert.tube_boxes,
        min_sin_theta: min_sin,
    })
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
/// (`O(h²·κ_uv)` at degree 1, `O(h⁴·κ_uv)` at degree 3) and the ring's
/// per-span widening, which GROWS with the composite's degree. On the
/// certifying fixture the cubic image measures `9.8e-11 m` and the
/// piecewise-linear one `1.1e-13 m`: at the residual scales this lane
/// exists for, the ring widening dominates and the low degree wins by
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
        let (lo, hi) = kv.domain();
        (1..PXN_WALL_SPANS)
            .filter_map(|i| {
                #[allow(clippy::cast_precision_loss)]
                let t = lo + (hi - lo) * (i as f64 / PXN_WALL_SPANS as f64);
                kv.multiplicity_of(t).is_none().then_some(t)
            })
            .collect()
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
/// parameter domain: the interpolation's clamped `0 → 1` knots mapped
/// affinely onto `[t0, t1]`, control points lifted to the caller's
/// scalar.
///
/// The map's own rounding is not a soundness question — the image is
/// evidence that limb 2 bounds, not a certified quantity (module
/// docs).
fn on_carrier_domain<T: Real>(
    image: &NurbsCurve2<f64>,
    t0: f64,
    t1: f64,
) -> Option<NurbsCurve2<T>> {
    let span = t1 - t0;
    let knots: Vec<f64> = image
        .knots()
        .knots()
        .iter()
        .map(|k| t0 + span * k)
        .collect();
    let knots = geom_core::spline::KnotVector::clamped(knots, image.degree()).ok()?;
    let control = image
        .control()
        .iter()
        .map(|p| Point2::new(T::from_f64(p.x), T::from_f64(p.y)))
        .collect();
    NurbsCurve2::new(knots, control, image.weights().to_vec()).ok()
}

/// The SSI refusal, in this lane's vocabulary.
fn refusal(e: SsiError) -> PlaneNurbsRefusal {
    match e {
        SsiError::CertificateLimb { limb, value } => PlaneNurbsRefusal::Limb { limb, value },
        // Rung 3's `margin` is a CERTIFIED CLEARANCE, not a measured
        // extent — it is exactly zero whenever the enclosure contains
        // zero — so it is carried under a name that says so, with the
        // chain's box count as the informative companion.
        SsiError::TubeStraddles { margin, boxes } => PlaneNurbsRefusal::TubeStraddles {
            certified_clearance: margin,
            boxes,
        },
        SsiError::Escalated(diag) => PlaneNurbsRefusal::Escalated(diag),
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
        _ => PlaneNurbsRefusal::Unsupported {
            what: "the rung-3 certificate refused for a reason outside this lane's vocabulary",
        },
    }
}

impl EdgeNurbsLane for f64 {
    fn plane_nurbs_limbs(
        carrier: &NurbsCurve3<Self>,
        plane: &Surface<Self>,
        wall: &NurbsSurface<Self>,
        extent: Self,
        band: Band,
    ) -> Result<PlaneNurbsLimbs<Self>, PlaneNurbsRefusal> {
        lane(carrier, plane, wall, extent, band)
    }

    fn lane_name() -> &'static str {
        "f64"
    }
}

#[cfg(feature = "probe")]
impl EdgeNurbsLane for geom_core::Probe {
    fn plane_nurbs_limbs(
        carrier: &NurbsCurve3<Self>,
        plane: &Surface<Self>,
        wall: &NurbsSurface<Self>,
        extent: Self,
        band: Band,
    ) -> Result<PlaneNurbsLimbs<Self>, PlaneNurbsRefusal> {
        lane(carrier, plane, wall, extent, band)
    }

    fn lane_name() -> &'static str {
        "telemetry probe"
    }
}

#[cfg(feature = "interval")]
impl EdgeNurbsLane for geom_core::interval::Interval {
    fn plane_nurbs_limbs(
        carrier: &NurbsCurve3<Self>,
        plane: &Surface<Self>,
        wall: &NurbsSurface<Self>,
        extent: Self,
        band: Band,
    ) -> Result<PlaneNurbsLimbs<Self>, PlaneNurbsRefusal> {
        lane(carrier, plane, wall, extent, band)
    }

    fn lane_name() -> &'static str {
        "interval"
    }
}

/// The dual lane: STATICALLY no plane × NURBS certificate — this impl
/// instantiates none of the certified machinery (module docs). A dual
/// body simply never adopts this edge class, because the bound that
/// would certify it cannot be built there.
impl<T> EdgeNurbsLane for geom_core::Dual<T>
where
    geom_core::Dual<T>: Decide,
{
    fn plane_nurbs_limbs(
        _carrier: &NurbsCurve3<Self>,
        _plane: &Surface<Self>,
        _wall: &NurbsSurface<Self>,
        _extent: Self,
        _band: Band,
    ) -> Result<PlaneNurbsLimbs<Self>, PlaneNurbsRefusal> {
        Err(PlaneNurbsRefusal::LaneUnsupported {
            scalar: Self::lane_name(),
        })
    }

    fn lane_name() -> &'static str {
        "dual"
    }
}

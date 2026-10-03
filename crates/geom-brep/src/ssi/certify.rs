//! **The C2 certificate for a rung-3 fitted cache** — all three limbs,
//! always (OQ2: no staging, hull bounds are an entry requirement).
//!
//! A marched-and-fitted carrier has no closed form on either side of
//! the comparison, so "certified residual ≤ ε" has to answer three
//! separate questions. This module answers all of them before a cache
//! can reach an at-rest body, and refuses typed for each separately —
//! the acceptance suite plants a corruption that trips each limb alone.
//!
//! # Limb 1 — on-locus residual (nearness)
//!
//! At the fixed [`CERT_SAMPLES`] schedule over the carrier's parameter
//! span:
//!
//! - **analytic operand**: `|f(C(t))|` in meters, the existing
//!   linearized implicit residual, through `ssi_on_locus`;
//! - **NURBS operand**: no implicit form exists, so the residual is
//!   `|C(t) − S(u*, v*)|` at a **certified foot point** from
//!   `geom::surfaces::projection` — and the projection's own
//!   orthogonality residual is banded too (`ssi_foot_orthogonality`,
//!   normalized by the chart speed so the margin is in meters). That
//!   second band is what stops a bad projection laundering a bad cache
//!   (C2.1 verbatim): a foot on the far sheet has vanishing
//!   orthogonality and a large distance; a clamped domain-edge foot has
//!   a small distance and a large orthogonality. Both are visible.
//!
//! # Limb 2 — sup-norm honesty (between the samples)
//!
//! A sampled max is not a bound, and a marched-and-fitted curve lies
//! about the locus *precisely between samples*. Two mechanisms, one per
//! operand kind, both pure convexity facts about control coefficients
//! (C9's certification arithmetic: no evaluation, no root, no transcendental):
//!
//! - **analytic**: `geom_core::spline::compose` composes the surface's
//!   polynomial implicit form with the carrier and returns a certified
//!   per-span hull of the composite. Its units are the composite's, so
//!   this module converts to meters **exactly** — `÷ 2R` for the sphere
//!   and cylinder, identity for the plane. Cone and torus are *not*
//!   converted: their meters forms carry a root, which certification
//!   arithmetic does not take (C9), so an
//!   arm wanting them must land that conversion first
//!   ([`super::enclose`] carries the same boundary, same reason).
//! - **NURBS**: `sup_t |S(P(t)) − C(t)|` bounded by the
//!   **tensor-product Bernstein composition**
//!   (`geom_core::spline::compose::tensor`, M5 PR 7b): the difference
//!   is enclosed as ONE composite whose ring coefficients are hulled
//!   per span, so the cancellation that is the whole content of
//!   `S(P(t)) = C(t)` survives into the bound. Every coefficient is a
//!   certification enclosure; nothing is sampled.
//!
//!   This replaced PR 7's per-span first-order enclosure
//!   (`rad_C + |C(m) − S(P(m))| + rad_S`), which was sound but scaled
//!   like the *span width*: the two variation radii each bounded a
//!   real motion of the curve across its span and their near-perfect
//!   cancellation was thrown away by enclosing them separately — on
//!   the M5 wall fixture it reported ~1e-2 m where the true residual
//!   is ~1e-10 m, and reaching ε would have needed tens of thousands
//!   of spans. The composite tracks the residual's own scale (the
//!   tensor module's rustdoc carries the derivation note for why
//!   composition-then-hull keeps what hull-then-difference loses),
//!   which is what retired the plane×NURBS arm — see the C5 table's
//!   `(Plane, Nurbs)` note for the retirement record.
//!
//! # Limb 3 — the uniqueness tube (component selection, made real)
//!
//! D2's "the connected component selected by the witness" is only
//! checkable if branch-uniqueness near the cache is **proved**. Over a
//! chain of boxes covering the carrier with certified radius ρ — in the
//! chart form, the pcurve's per-span windows padded along each axis by
//! ρ over that axis's chart speed, the pad [`SsiTube::Chart`] records —
//! this module proves what the enclosures actually support — stated exactly,
//! because a slightly-too-strong claim here is the one that would
//! matter:
//!
//! > On each box `B`, enclose `(∇f₁ × ∇f₂)·e` (analytic pair) or the
//! > chart form `∇φ·e⊥ / ‖chart stretch‖` (plane×NURBS). Suppose the
//! > enclosure excludes zero. The enclosure is valid at **every** point
//! > of `B`, so on any slice `{e·x = c} ∩ B` the two constraint
//! > gradients restricted to that slice stay linearly independent
//! > throughout. Take two solutions in one slice: the mean value
//! > theorem applied along the segment joining them (which lies in `B`,
//! > a convex box) forces `f₁` and `f₂` to have a common critical
//! > direction somewhere on it — which the enclosure has just excluded.
//! > So each slice holds **at most one** solution, and the solution set
//! > in `B` is a graph over the `e` axis: one arc, no branch point, no
//! > loop, no second sheet **at the same `e`-level**.
//!
//! It is a convexity/mean-value argument over the enclosure, not a bare
//! appeal to the implicit function theorem (which is local and would
//! not, by itself, cover the whole box). And it says nothing about a
//! **disjoint component threading the padded chain at `e`-levels the
//! carrier never occupies**: a second arc beside the first along `e`,
//! leaving through the box's sides, passes the slice argument. A search
//! banks every cell inside a tube as accounted ([`super::exhaust`]), so
//! where one does, the probe proves that component absent too: a piece
//! of the solution set in a box ends on its boundary at two points, so a
//! boundary holding exactly two simple solutions holds one piece, and
//! consecutive boxes sharing a solution share it ([`one_arc`],
//! [`one_arc_r3`]). Components outside the chain are the accounting
//! pass's to exclude or refuse; uniqueness here and completeness there
//! are two theorems, and neither is doing the other's work.
//!
//! An enclosure that **straddles** zero at every rung escalates, typed,
//! never retried: `ssi_tube_transversality` lands in `Sign::Zero` and
//! refuses toward C7. Two branches passing within the band of each
//! other is a genuine sliver of the operand pair, where F6's ladder says
//! escalate; the enclosure's remaining slack can also straddle, and
//! escalates the same way.
//!
//! # The witness is unchanged
//!
//! `witness = carrier(mid)` (`WitnessMidpoint`), minted by the
//! constructing op from the cache this schedule sees. S2 stays
//! discharged; nothing about the witness contract moves at rung 3.

use geom::{NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::spline::algebra::{
    GridSkip, SLIVER_CLEARANCE_ULPS, domain_grid_points, range_grid_points,
};
use geom_core::spline::compose::{self, CurveCertData, ImplicitSurface, tensor};
use geom_core::{
    Band, Bounds, CertifiedBounds, CertifiedEnclosure, Decide, Interval, Margin, Point3, Real,
    Sign, Vec3,
};

use crate::certify::CertCheck;
use crate::certify::{CERT_SAMPLES, sample_param};
use crate::dihedral::{decide, decide_reported};
use crate::recourse::Refused;

use super::enclose::{
    Box3, NurbsBoxes, chart_transverse_margin, graph_margin, implicit_enclosure,
    implicit_gradient_enclosure, zero_free_lower_bound,
};
use super::exhaust::UvRect;
use super::{SsiError, SsiOperand, TubeScale};

/// The **largest** tube radius tried, as a fraction of the caller's
/// named extent. The ladder halves from here.
pub const SSI_TUBE_RADIUS_MAX: f64 = 1.0 / 8.0;

/// How many halvings the ladder tries before giving up. The last rung
/// is `SSI_TUBE_RADIUS_MAX · 2^−(SSI_TUBE_RUNGS−1)` of the extent.
pub const SSI_TUBE_RUNGS: usize = 20;

/// The absolute floor on the tube radius, as a multiple of ε: below
/// this the tube could not contain the carrier's own certified
/// residual, so there is nothing left to prove.
pub const SSI_TUBE_RADIUS: f64 = 8.0;

/// The certified tube radius is **searched**, not fixed, and the reason
/// is load-bearing rather than an optimization.
///
/// A tube proves one-arc-ness over a *neighborhood*; how wide a
/// neighborhood is a property of the operand pair, not a constant. A
/// tube pinned at a few ε would be technically valid and practically
/// useless: the exhaustiveness accounting counts a cell as "accounted"
/// only when the cell lies **inside** a tube box, so an ε-wide tube
/// forces the subdivision to refine every cell along the locus down to
/// ε — a number of cells linear in `locus length / ε`, which at
/// ε = 1e-9 is not a computation, it is a hang. Searching for the
/// widest radius the graph criterion certifies makes the accounting
/// terminate at the geometry's own scale, and it reports a *stronger*
/// theorem (uniqueness over a wider region) for free.
///
/// The ladder is a fixed geometric sequence tried in a fixed order
/// (D9): no value branch chooses it, the first rung that certifies
/// wins, and if none does the operation refuses typed rather than
/// shipping a carrier whose component-selection claim is unproved.
pub(crate) fn tube_ladder(extent: f64, band: Band) -> impl Iterator<Item = f64> {
    let floor = SSI_TUBE_RADIUS * band.zero();
    (0..SSI_TUBE_RUNGS).filter_map(move |k| {
        #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
        let r = SSI_TUBE_RADIUS_MAX * extent / (2.0f64).powi(k as i32);
        if r >= floor { Some(r) } else { None }
    })
}

/// How many spans the carrier is refined to before the hull limbs run.
/// More spans ⇒ tighter hulls and a tighter tube, at linear cost; this
/// is a **structure** choice (C6's f64 lane), not a decision.
pub const SSI_CERT_SPANS: usize = 32;

/// The region limb 3 proved one-arc-ness over, by the kind of tube
/// that proved it.
///
/// **Ladder structure** (C6's `f64` selection lane), lifted to the
/// certificate's scalar for uniformity. No trilean reads it, but it is
/// what the exhaustiveness accounting banks: a cell inside the recorded
/// region is accounted, so the region recorded must be the one proved.
#[derive(Clone, Copy, Debug)]
pub enum SsiTube<T> {
    /// The ℝ³ arm: a box chain around the carrier padded by `radius`
    /// metres, which is the proved region.
    Spatial {
        /// The pad, in metres.
        radius: T,
    },
    /// The chart arm (plane × NURBS): a per-span window chain around
    /// the wall's pcurve, padded per axis in chart units. The rung is
    /// the ladder radius in metres that was tried, a label and not a
    /// metre bound on the region; the pads are that radius divided by
    /// each axis's chart speed, and they are the proved region.
    Chart {
        /// The ladder rung tried, in metres.
        rung: T,
        /// The pad along `u`, in chart units.
        pad_u: T,
        /// The pad along `v`, in chart units.
        pad_v: T,
    },
}

impl<T> SsiTube<T> {
    /// The tube at another scalar, field by field.
    pub(crate) fn map<U>(self, f: impl Fn(T) -> U) -> SsiTube<U> {
        match self {
            Self::Spatial { radius } => SsiTube::Spatial { radius: f(radius) },
            Self::Chart { rung, pad_u, pad_v } => SsiTube::Chart {
                rung: f(rung),
                pad_u: f(pad_u),
                pad_v: f(pad_v),
            },
        }
    }
}

/// The three-limb certificate of a rung-3 fitted carrier. Every field
/// is a bound the corresponding limb proved, in the unit its own doc
/// names: metres, except the chart tube's pads ([`SsiTube::Chart`]).
///
/// **Generic since M6-2.** The certificate is carried at the scalar it
/// was derived at, so an interval-lane body's cache holds enclosures of
/// its own bounds rather than an `f64` shadow of somebody else's run.
/// Which fields are genuinely scalar-typed and which are lifted ring or
/// ladder structure is stated per field — the distinction is the whole
/// C6/C9 boundary and folding it away would be dishonest.
#[derive(Clone, Copy, Debug)]
pub struct SsiCertificate<T: Real> {
    /// The fixed sample count of limb 1 ([`CERT_SAMPLES`]).
    pub samples: u32,
    /// Limb 1: the largest on-locus residual over the schedule, in
    /// meters (the sampled max — it steers, it does not certify).
    /// **Scalar-typed**: evaluated at `T`, so it is an enclosure of the
    /// residual on the interval lane.
    pub on_locus_max: T,
    /// Limb 2: the certified **sup-norm** bound over the whole span, in
    /// meters. This is the number that certifies. **Certification-derived**: the
    /// certification produces an `f64` upper bound (that is what a hull bound
    /// IS), lifted here so consumers band one scalar; at the interval
    /// scalar it is a thin enclosure of that bound, and the widening of
    /// a lifted operand has already been paid inside interval arithmetic.
    pub hull_sup: T,
    /// Limb 3: the region the tube proved, by kind ([`SsiTube`]).
    pub tube: SsiTube<T>,
    /// Limb 3: the smallest certified transversality margin over the
    /// box chain, in meters — the headroom of the one-arc proof. The
    /// ring's zero-free lower bound times the caller's lever arm, so it
    /// carries the arm's scalar.
    pub tube_transversality: T,
    /// Limb 3: how many boxes the chain has.
    pub tube_boxes: u32,
}

/// Which limb a certificate refusal came from — so a consumer (and the
/// acceptance suite) can tell them apart.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsiLimb {
    /// Limb 1 — on-locus residual (including the foot-point
    /// orthogonality check).
    OnLocus,
    /// Limb 2 — the control-hull sup-norm bound.
    HullSup,
    /// Limb 3 — the uniqueness tube.
    Tube,
}

impl SsiLimb {
    /// The limb's display name.
    pub fn name(self) -> &'static str {
        match self {
            Self::OnLocus => "limb 1 (on-locus residual)",
            Self::HullSup => "limb 2 (control-hull sup-norm bound)",
            Self::Tube => "limb 3 (uniqueness tube)",
        }
    }

    /// The certification check a refusal of this limb is a refused arm
    /// of, whose ending ([`crate::certify::recourse`]) every door that
    /// reports the limb reads. Limbs 1 and 2 are the fitted carrier's
    /// on-locus residual and sup-norm bound; limb 3's margin is the
    /// operands' transversality over the box chain.
    #[must_use]
    pub fn check(self) -> CertCheck {
        match self {
            Self::OnLocus => CertCheck::PlaneNurbsOnLocus,
            Self::HullSup => CertCheck::PlaneNurbsHull,
            Self::Tube => CertCheck::Transversality,
        }
    }
}

/// Refine the carrier so the hull limbs have small spans to work with
/// (knot refinement is exact in ℝ; the curve is unchanged).
fn refined<T: Real>(curve: &NurbsCurve3<T>) -> NurbsCurve3<T> {
    let kv = curve.knots();
    // Already fine enough: refining a carrier that the marcher's step
    // rule already gave hundreds of spans buys nothing and costs an
    // O(n²) knot insertion per call.
    if kv.control_count() >= SSI_CERT_SPANS + kv.degree() {
        return curve.clone();
    }
    // Parameters already present as knots are skipped (refinement would
    // raise multiplicity, which is not what this is for).
    let add = domain_grid_points(kv, SSI_CERT_SPANS, GridSkip::BitEqual);
    curve.refine_knots(&add).unwrap_or_else(|_| curve.clone())
}

/// The exact `f64` a structural surface parameter stands for, or `None`
/// when the scalar's bracket is not a point.
///
/// **Why this gate exists (M6-2).** `geom_core`'s [`ImplicitSurface`]
/// is `f64` STRUCTURE — a polynomial form with `f64` coefficients — and
/// the composite it drives is only a bound on the surface it actually
/// describes. Lifting `certify_branch` off `f64` therefore does NOT
/// make the analytic composite accept a *widened* operand: a cylinder
/// whose radius is an enclosure is a FAMILY of cylinders, and picking
/// any representative would certify the carrier against a surface the
/// body does not have. A thin bracket is exactly the case where the
/// representative is the surface, so that case is admitted and every
/// other one refuses typed (`UnsupportedCertificate`, below) — never a
/// midpoint, never a guess. Retiring the gate means giving the
/// composite enclosure-valued coefficients, which is a `geom_core`
/// change with its own unit.
fn exact<T: Bounds>(v: T) -> Option<f64> {
    let (lo, hi) = (v.lo(), v.hi());
    (lo == hi && !lo.is_nan()).then_some(lo)
}

/// Three coordinates of an exact point, or `None` if any is widened.
fn exact3<T: Bounds>(p: [T; 3]) -> Option<[f64; 3]> {
    Some([exact(p[0])?, exact(p[1])?, exact(p[2])?])
}

/// The `compose` implicit form of an analytic surface, and the exact
/// factor converting its composite's units to meters.
///
/// `Err` names the reason, which the caller turns into an
/// [`SsiError::UnsupportedCertificate`] verbatim: the kinds whose
/// meters form carries a root certification arithmetic does not take (cone, torus), NURBS
/// (no implicit form), and — since M6-2 — an operand whose structural
/// parameters are not exact at the caller's scalar (see [`exact`]).
fn composite_form<T: Bounds>(s: &Surface<T>) -> Result<(ImplicitSurface, f64), &'static str> {
    const WIDENED: &str = "the analytic operand's structural parameters are not exact at this \
                           scalar — the ring composite's implicit form is f64 structure, and a \
                           widened operand is a family of surfaces, not the body's surface \
                           — refused rather than represented by a midpoint";
    match *s {
        Surface::Plane { origin, normal, .. } => {
            let (Some(point), Some(normal)) =
                (exact3(origin.to_array()), exact3(normal.to_array()))
            else {
                return Err(WIDENED);
            };
            Ok((
                ImplicitSurface::Plane { point, normal },
                // n·(P − p₀) is already meters for a unit normal.
                1.0,
            ))
        }
        Surface::Sphere { center, radius, .. } => {
            let (Some(center), Some(radius)) = (exact3(center.to_array()), exact(radius)) else {
                return Err(WIDENED);
            };
            Ok((
                ImplicitSurface::Sphere { center, radius },
                // |P−c|² − R² = 2R · the linearized meters residual,
                // exactly.
                1.0 / (2.0 * radius),
            ))
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (Some(point), Some(axis), Some(radius)) = (
                exact3(origin.to_array()),
                exact3(axis.to_array()),
                exact(radius),
            ) else {
                return Err(WIDENED);
            };
            Ok((
                ImplicitSurface::Cylinder {
                    point,
                    axis,
                    radius,
                },
                // |w|² − R² = 2R · the linearized meters residual,
                // exactly.
                1.0 / (2.0 * radius),
            ))
        }
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            Err("no ring-computable meters composite for this surface kind \
             (cone/torus need a certified root, which certification arithmetic does not take; \
             a spline stand-in and its offset description have no implicit form \
             to build one from)")
        }
    }
}

/// Limb 1 + limb 2 against one **analytic** operand.
fn analytic_limbs<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<(T, T), SsiError> {
    // ---- limb 1: the fixed schedule ----
    let (t0, t1) = carrier.domain();
    let mut worst = T::zero();
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        let r = crate::implicit::implicit_residual(surface, carrier.eval(T::from_f64(t))).abs();
        // `max`, not a `>` branch: the running worst is a scalar-typed
        // quantity now, and generic evaluation code does not compare.
        worst = worst.max(r);
        match decide("ssi_on_locus", Margin::of(r), band) {
            // Zero is the affirmative: the residual is zero to
            // tolerance (the `dihedral_wedge` convention).
            Ok(Sign::Zero) => {}
            Ok(Sign::Positive | Sign::Negative) => {
                return Err(SsiError::CertificateLimb {
                    limb: SsiLimb::OnLocus,
                    value: r.hi(),
                });
            }
            Err(cause) => {
                return Err(SsiError::CertificateEscalated {
                    limb: SsiLimb::OnLocus,
                    cause,
                });
            }
        }
    }

    // ---- limb 2: the certified hull bound ----
    let (form, to_meters) =
        composite_form(surface).map_err(|what| SsiError::UnsupportedCertificate { what })?;
    let fine = refined(carrier);
    let coords = fine.certified_coords();
    let data = CurveCertData::new(fine.knots(), fine.weights(), &coords).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the fitted carrier's enclosure data is malformed",
        }
    })?;
    let composite = compose::implicit_composite(&data, &form).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the implicit composite refused the fitted carrier",
        }
    })?;
    // Interval arithmetic answers with an `f64` upper bound — that is what a hull
    // bound is — and it is lifted here so the limb is banded at the
    // caller's scalar like every other residual (field docs).
    let sup = T::from_f64(composite.sup_bound() * to_meters);
    match decide("ssi_hull_sup", Margin::of(sup), band) {
        Ok(Sign::Zero) => Ok((worst, sup)),
        Ok(Sign::Positive | Sign::Negative) => Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value: sup.hi(),
        }),
        Err(cause) => Err(SsiError::CertificateEscalated {
            limb: SsiLimb::HullSup,
            cause,
        }),
    }
}

/// Limb 1 + limb 2 against a **NURBS** operand, using the traced
/// pcurve as the parameter map (module docs).
fn nurbs_limbs<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    pcurve: &NurbsCurve2<T>,
    surface: &NurbsSurface<T>,
    band: Band,
) -> Result<(T, T), SsiError> {
    // ---- limb 1: the fixed schedule, through certified foot points --
    let (t0, t1) = carrier.domain();
    let mut worst = T::zero();
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        let c = carrier.eval(T::from_f64(t));
        // Warm-start from the trace's own pcurve: the projection is a
        // *check*, and starting it where the trace says the foot is
        // makes a disagreement visible rather than hidden by a global
        // seeding sweep landing somewhere else. The seed is a
        // parameter — `f64` structure on both sides of the M6-2 lift.
        let p = pcurve.eval(T::from_f64(t));
        let proj = surface
            .project_from_seed(c, p.x.lo(), p.y.lo())
            .map_err(|e| SsiError::FootPointInconclusive {
                t,
                last_distance: e.last_distance,
            })?;
        worst = worst.max(proj.distance);
        match decide("ssi_on_locus_foot", Margin::of(proj.distance), band) {
            Ok(Sign::Zero) => {}
            Ok(Sign::Positive | Sign::Negative) => {
                return Err(SsiError::CertificateLimb {
                    limb: SsiLimb::OnLocus,
                    value: proj.distance.hi(),
                });
            }
            Err(cause) => {
                return Err(SsiError::CertificateEscalated {
                    limb: SsiLimb::OnLocus,
                    cause,
                });
            }
        }
        // The orthogonality residuals, normalized by the chart speeds
        // so the margin is a length: |S_d·r|/|S_d| is the component of
        // the offset along that parameter line, in meters.
        let jet = surface.ders(T::from_f64(proj.u), T::from_f64(proj.v));
        for (res, speed) in [
            (proj.orthogonality_u, jet.du.norm()),
            (proj.orthogonality_v, jet.dv.norm()),
        ] {
            let margin = Margin::levered_inv(res, speed);
            match decide("ssi_foot_orthogonality", margin, band) {
                Ok(Sign::Zero) => {}
                Ok(Sign::Positive | Sign::Negative) => {
                    return Err(SsiError::CertificateLimb {
                        limb: SsiLimb::OnLocus,
                        value: margin.value().hi(),
                    });
                }
                Err(cause) => {
                    return Err(SsiError::CertificateEscalated {
                        limb: SsiLimb::OnLocus,
                        cause,
                    });
                }
            }
        }
    }

    // ---- limb 2: |S(P(t)) − C(t)| as ONE composite (M5 PR 7b) ----
    // The tensor-product Bernstein composition encloses the difference
    // at the coefficient level, so the cancellation that IS the content
    // of S(P(t)) = C(t) survives into the bound (PR 7's first-order
    // enclosure added the two variation radii instead and scaled with
    // the span width — ~1e-2 m where the truth is ~1e-10 m). Data in,
    // bounds out: nothing here samples anything (C2.2).
    //
    // The OQ4-aligned fit (carrier and pcurve on one knot vector) stays
    // the cache contract, and the composite serves the unaligned case
    // over the same bound: both curves are decomposed onto the MERGED
    // break list by exact knot insertion, so alignment is recovered
    // structurally rather than approximated by a whole-domain radius.
    // The `SSI_CERT_SPANS` uniform breaks are injected for hull
    // tightness — the same structure choice `refined` makes for the box
    // chain (C6's f64 lane), expressed as breaks instead of a refit.
    let coords = carrier.certified_coords();
    let cdata = CurveCertData::new(carrier.knots(), carrier.weights(), &coords).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the fitted carrier's enclosure data is malformed",
        }
    })?;
    let pcoords = pcurve.certified_coords();
    let pdata = CurveCertData::new(pcurve.knots(), pcurve.weights(), &pcoords).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the traced pcurve's enclosure data is malformed",
        }
    })?;
    let scoords = surface.certified_coords();
    let sdata = tensor::SurfaceCertData::new(
        surface.knots_u(),
        surface.knots_v(),
        surface.weights(),
        &scoords,
    )
    .map_err(|_| SsiError::UnsupportedCertificate {
        what: "the NURBS operand's enclosure data is malformed",
    })?;
    let extra = chart_breaks(carrier.knots(), pcurve.knots());
    let sup = tensor::surface_curve_residual(&sdata, &pdata, &cdata, &extra)
        .map_err(|_| SsiError::UnsupportedCertificate {
            what: "the tensor composite refused the carrier/pcurve pair (mismatched \
                   channel counts or knot domains — the shared-parameter identity is the entry \
                   requirement)",
        })?
        .sup_bound();
    // Unlike the analytic arm, the NURBS arm needs NO exactness gate:
    // every coefficient of every operand entered interval arithmetic through its
    // own bracket (`certified_coords`), so a widened control net widens the
    // composite and the bound stays honest.
    let sup = T::from_f64(sup);
    match decide("ssi_hull_sup_chart", Margin::of(sup), band) {
        Ok(Sign::Zero) => Ok((worst, sup)),
        Ok(Sign::Positive | Sign::Negative) => Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value: sup.hi(),
        }),
        Err(cause) => Err(SsiError::CertificateEscalated {
            limb: SsiLimb::HullSup,
            cause,
        }),
    }
}

/// The uniform breaks limb 2's composite is cut at: the carrier
/// domain's `SSI_CERT_SPANS` grid, minus every point within
/// [`SLIVER_CLEARANCE_ULPS`] of an interior knot of EITHER curve. The
/// composite merges both curves' knots into its break list, so a grid
/// point a few ulps off either one's knot would open a hairline span
/// beside it.
fn chart_breaks(carrier: &KnotVector, pcurve: &KnotVector) -> Vec<f64> {
    let (lo, hi) = carrier.domain();
    let knots: Vec<f64> = carrier
        .interior_knots()
        .chain(pcurve.interior_knots())
        .map(|(k, _)| k)
        .collect();
    range_grid_points(
        lo,
        hi,
        SSI_CERT_SPANS,
        GridSkip::WithinUlps(SLIVER_CLEARANCE_ULPS),
        &knots,
    )
}

/// The box chain covering a carrier: one padded box per span of the
/// refined curve, from the span's control hull (exact containment for a
/// non-rational curve — the convex-hull property).
fn box_chain<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
) -> Vec<(Box3, Vec3<T>)> {
    let fine = refined(carrier);
    let coords = fine.certified_coords();
    let kv = fine.knots();
    let mut out = Vec::new();
    // One pair per coordinate channel, minted once outside the span
    // walk. The coordinates and the knots are both read from `fine` —
    // the SAME refined curve — so the count relation is
    // `NurbsCurve3::new`'s fact and the refusal arm is unreachable by
    // construction; it returns an EMPTY chain, which limb 3 reads as a
    // definite refusal (no box, so nothing banked).
    let (Some(cx), Some(cy), Some(cz)) = (
        kv.with_coeffs(&coords[0]),
        kv.with_coeffs(&coords[1]),
        kv.with_coeffs(&coords[2]),
    ) else {
        return out;
    };
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and window construction are one step; the
        // three channels share the vector, so they refuse alike.
        let (Some(wx), Some(wy), Some(wz)) = (cx.span(index), cy.span(index), cz.span(index))
        else {
            continue;
        };
        let (a, b) = (kv.knots()[index], kv.knots()[index + 1]);
        let hx = wx.hull();
        let hy = wy.hull();
        let hz = wz.hull();
        let bx = Box3 {
            x: hx,
            y: hy,
            z: hz,
        };
        // The box's axis: the carrier's own tangent at the span
        // midpoint, normalized. Any direction transverse to the
        // solution set works; this is the natural one.
        //
        // The `n > 0.0` guard the f64 form carried is GONE rather than
        // lifted: it is a value branch, which generic evaluation code
        // may not take (Q1), and it bought nothing. A degenerate span
        // (zero-length tangent) divided by zero poisons `e`, the graph
        // margin's certification enclosure is refused with it, and
        // `zero_free_lower_bound` reports 0 — the same typed refusal
        // the guarded zero vector produced, reached without a branch.
        let t = fine.deriv(T::from_f64(0.5 * (a + b)));
        let n = t.norm();
        out.push((bx, t / n));
    }
    out
}

/// Limb 3's **enclosure probe** for an analytic pair: the graph
/// criterion in ℝ³ over a box chain of the given radius.
///
/// Pure enclosure arithmetic — **no trilean here**. Choosing the tube's
/// radius is cache *structure* (C6's f64 selection lane), and mixing
/// the ladder's rejected rungs into the K-funnel would both pollute the
/// telemetry and blur the one decision that matters. The single named
/// decision runs once, upstairs, on the chosen radius.
///
/// Returns the chain's smallest zero-free margin (dimensionless, the
/// `sin θ` scale), the box count, and, where the margin is zero-free
/// and a search banks the tube, whether the chain's solution set in the
/// banked region is one arc ([`one_arc_r3`]); `None` when the chain is broken or an enclosure
/// refused, which is a definite structural refusal.
fn probe_tube_analytic<T: Decide + Bounds + CertifiedEnclosure>(
    chain: &[(Box3, Vec3<T>)],
    s1: &Surface<T>,
    s2: &Surface<T>,
    radius: f64,
    banked: Banked,
) -> Option<(f64, u32, bool)> {
    if chain.is_empty() {
        return None;
    }
    let mut worst = f64::INFINITY;
    let mut prev: Option<Box3> = None;
    let mut padded = Vec::with_capacity(chain.len());
    for (raw, e) in chain.iter() {
        // The chain is built once (an O(n²) knot refinement) and padded
        // per ladder rung, which is a pure interval widening.
        let bx = raw.pad(radius);
        // The chain must actually be a chain: consecutive boxes share
        // the span endpoint the carrier passes through, so a definite
        // separation means the cover is not connected and the
        // concatenation argument does not hold.
        if prev.is_some_and(|p| p.definitely_disjoint(bx)) {
            return None;
        }
        prev = Some(bx);
        padded.push(bx);
        let m = zero_free_lower_bound(graph_margin(s1, s2, bx, *e));
        if m < worst {
            worst = m;
        }
    }
    let single = worst > 0.0
        && match banked {
            Banked::No => true,
            Banked::Wall => one_arc_r3(s1, s2, &padded, None),
            Banked::Within(b) => one_arc_r3(s1, s2, &padded, Some(b)),
        };
    Some((worst, chain.len() as u32, single))
}

/// The point interval at `x`.
fn pt(x: f64) -> Interval {
    Interval::from_bounds(x, x)
}

/// A box's side along axis `i` (0, 1, 2 for x, y, z).
fn side(b: Box3, i: usize) -> Interval {
    [b.x, b.y, b.z][i]
}

/// `b` with its side along axis `i` replaced.
fn with_side(b: Box3, i: usize, s: Interval) -> Box3 {
    let mut v = [b.x, b.y, b.z];
    v[i] = s;
    Box3 {
        x: v[0],
        y: v[1],
        z: v[2],
    }
}

/// The intersection of two boxes, `None` when it has no interior or a
/// side is refused.
fn clip(a: Box3, b: Box3) -> Option<Box3> {
    let mut out = a;
    for i in 0..3 {
        let (p, q) = (side(a, i), side(b, i));
        if !(p.is_certified() && q.is_certified()) {
            return None;
        }
        let (lo, hi) = (p.lo().max(q.lo()), p.hi().min(q.hi()));
        if lo.partial_cmp(&hi) != Some(core::cmp::Ordering::Less) {
            return None;
        }
        out = with_side(out, i, Interval::from_bounds(lo, hi));
    }
    Some(out)
}

/// The solutions of `f₁ = f₂ = 0` on the boundary of `r`, each in a box
/// that holds exactly one: a face piece is dropped where `f₁` or `f₂` is
/// zero-free over it, kept where the Krawczyk operator of the pair
/// restricted to the face maps it into its own interior (one solution,
/// enclosed by the image), dropped where the image misses it, and
/// bisected otherwise. `None` when a piece resolves no way within
/// [`EXIT_DEPTH`] bisections, or the walk exceeds [`EXIT_PIECES`].
fn face_roots<T: CertifiedBounds>(s1: &Surface<T>, s2: &Surface<T>, r: Box3) -> Option<Vec<Box3>> {
    let mut roots = Vec::new();
    let mut pieces = 0u32;
    for k in 0..3 {
        let (i, j) = ((k + 1) % 3, (k + 2) % 3);
        for c in [side(r, k).lo(), side(r, k).hi()] {
            let mut stack = vec![(with_side(r, k, pt(c)), 0u32)];
            while let Some((x, depth)) = stack.pop() {
                pieces += 1;
                if pieces > EXIT_PIECES {
                    return None;
                }
                if sign_of(implicit_enclosure(s1, x)).is_some()
                    || sign_of(implicit_enclosure(s2, x)).is_some()
                {
                    continue;
                }
                match krawczyk(s1, s2, x, (i, j)) {
                    Some(Krawczyk::One(k)) => {
                        roots.push(k);
                        continue;
                    }
                    Some(Krawczyk::None) => continue,
                    _ => {}
                }
                if depth >= EXIT_DEPTH {
                    return None;
                }
                let a = if side(x, i).hi() - side(x, i).lo() >= side(x, j).hi() - side(x, j).lo() {
                    i
                } else {
                    j
                };
                let s = side(x, a);
                let m = s.lo() + EXIT_CUT * (s.hi() - s.lo());
                stack.push((with_side(x, a, Interval::from_bounds(m, s.hi())), depth + 1));
                stack.push((with_side(x, a, Interval::from_bounds(s.lo(), m)), depth + 1));
            }
        }
    }
    Some(roots)
}

/// What the Krawczyk test proved over a face piece.
enum Krawczyk {
    /// Exactly one solution, inside this box.
    One(Box3),
    /// No solution.
    None,
}

/// The Krawczyk operator of `(f₁, f₂)` over the face piece `x`, in its
/// two free axes `(i, j)`: `K = m − Y·F(m) + (I − Y·J(x))·(x − m)`,
/// `Y` the inverse of `J`'s midpoint. `K` inside the interior of `x`
/// proves exactly one solution in `x`, and in `K`; `K` missing `x`
/// proves none. `None` when it proves neither.
fn krawczyk<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    x: Box3,
    (i, j): (usize, usize),
) -> Option<Krawczyk> {
    let (xi, xj) = (side(x, i), side(x, j));
    let (mi, mj) = (0.5 * (xi.lo() + xi.hi()), 0.5 * (xj.lo() + xj.hi()));
    let m = with_side(with_side(x, i, pt(mi)), j, pt(mj));
    let (f1, f2) = (implicit_enclosure(s1, m), implicit_enclosure(s2, m));
    let (g1, g2) = (
        implicit_gradient_enclosure(s1, x),
        implicit_gradient_enclosure(s2, x),
    );
    let jac = [[g1[i], g1[j]], [g2[i], g2[j]]];
    let mid = |v: Interval| 0.5 * (v.lo() + v.hi());
    let (a, b, c, d) = (
        mid(jac[0][0]),
        mid(jac[0][1]),
        mid(jac[1][0]),
        mid(jac[1][1]),
    );
    let det = a * d - b * c;
    if !det.is_finite() || det == 0.0 {
        return None;
    }
    let y = [[d / det, -b / det], [-c / det, a / det]].map(|row| row.map(pt));
    let one = pt(1.0);
    let zero = pt(0.0);
    let (di, dj) = (xi - pt(mi), xj - pt(mj));
    let row = |r: usize, m: f64| {
        let yf = y[r][0] * f1 + y[r][1] * f2;
        let ci = (if r == 0 { one } else { zero }) - (y[r][0] * jac[0][0] + y[r][1] * jac[1][0]);
        let cj = (if r == 1 { one } else { zero }) - (y[r][0] * jac[0][1] + y[r][1] * jac[1][1]);
        pt(m) - yf + ci * di + cj * dj
    };
    let (ki, kj) = (row(0, mi), row(1, mj));
    if !(ki.is_certified() && kj.is_certified()) {
        return None;
    }
    let inside = |k: Interval, s: Interval| k.lo() > s.lo() && k.hi() < s.hi();
    let misses = |k: Interval, s: Interval| k.hi() < s.lo() || k.lo() > s.hi();
    if inside(ki, xi) && inside(kj, xj) {
        Some(Krawczyk::One(with_side(with_side(x, i, ki), j, kj)))
    } else if misses(ki, xi) || misses(kj, xj) {
        Some(Krawczyk::None)
    } else {
        None
    }
}

/// Whether `b` lies in `r`, closed.
fn within(b: Box3, r: Box3) -> bool {
    (0..3).all(|i| side(b, i).lo() >= side(r, i).lo() && side(b, i).hi() <= side(r, i).hi())
}

/// **The ℝ³ chain's solution set is one arc**, given a zero-free
/// `(∇f₁ × ∇f₂)·e` over every box, each cut to `clip` where the caller
/// bounds the search. The argument is [`one_arc`]'s: the solution set
/// in a box `R` is a graph over its `e` axis, so each of its connected
/// pieces ends on `∂R` at two distinct points, and a boundary holding
/// exactly two simple solutions ([`face_roots`]) holds one piece.
/// Consecutive boxes share it when a boundary solution of either lies
/// in the other, which then holds it too.
fn one_arc_r3<T: CertifiedBounds>(
    s1: &Surface<T>,
    s2: &Surface<T>,
    boxes: &[Box3],
    bound: Option<Box3>,
) -> bool {
    let Some(cut) = boxes
        .iter()
        .map(|b| bound.map_or(Some(*b), |s| clip(*b, s)))
        .collect::<Option<Vec<Box3>>>()
    else {
        return false;
    };
    let Some(roots) = cut
        .iter()
        .map(|r| face_roots(s1, s2, *r).filter(|v| v.len() == 2))
        .collect::<Option<Vec<Vec<Box3>>>>()
    else {
        return false;
    };
    (1..cut.len()).all(|k| {
        roots[k - 1].iter().any(|q| within(*q, cut[k]))
            || roots[k].iter().any(|q| within(*q, cut[k - 1]))
    })
}

/// One span of a pcurve's chart tube: the span's hull padded per axis,
/// and the span's parameter midpoint, where the probe reads the
/// pcurve's tangent.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartWindow {
    /// The padded window, in chart units.
    pub(crate) rect: UvRect,
    /// The span's parameter midpoint.
    pub(crate) mid: f64,
    /// The span's parameter ends.
    pub(crate) ends: (f64, f64),
}

/// The chart tube's windows: **one padded rectangle per nonempty span**
/// of the pcurve, the span's hull widened by `pad = (pad_u, pad_v)`.
/// Limb 3's probe proves over exactly these, and the accounting pass
/// banks exactly these, so the two read one region.
///
/// Per span, deliberately — not one rectangle around the whole pcurve:
/// a bounding box of a curve that wanders across its domain contains
/// regions the curve never enters, and the accounting pass would
/// account for cells nobody proved anything about.
///
/// `None` when any span's hull is refused: a refused hull is NaI, and
/// its NaN endpoints are no window. No span of such a pcurve is
/// bounded, so the probe has nothing to prove over and the accounting
/// pass banks nothing — the stricter answer for both. The window hull
/// maps any refusal to NaI, so `!is_certified()` and `Real::is_poison`
/// agree on it today; the check asks `is_certified` because that is the
/// question that stays right if the hull ever hands a refusal on with
/// real endpoints.
pub(crate) fn chart_tube_windows<T: geom_core::CertifiedBounds>(
    pcurve: &NurbsCurve2<T>,
    pad: (f64, f64),
) -> Option<Vec<ChartWindow>> {
    let kv = pcurve.knots();
    let coords = pcurve.certified_coords();
    // One pair per chart channel, minted once: the coordinates and the
    // knots are both the pcurve's, so the count is `NurbsCurve2::new`'s
    // fact and the refusal arm is unreachable by construction.
    let (cu, cv) = kv.with_coeffs(&coords[0]).zip(kv.with_coeffs(&coords[1]))?;
    let mut out = Vec::new();
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and window construction are one step; both
        // channels share the vector, so both refuse the same indices.
        let (Some(wu), Some(wv)) = (cu.span(index), cv.span(index)) else {
            continue;
        };
        let (hu, hv) = (wu.hull(), wv.hull());
        if !hu.is_certified() || !hv.is_certified() {
            return None;
        }
        let (a, b) = (kv.knots()[index], kv.knots()[index + 1]);
        out.push(ChartWindow {
            rect: UvRect {
                u: (hu.lo() - pad.0, hu.hi() + pad.0),
                v: (hv.lo() - pad.1, hv.hi() + pad.1),
            },
            mid: 0.5 * (a + b),
            ends: (a, b),
        });
    }
    Some(out)
}

/// The deepest bisection the boundary walk takes along one window edge.
const EXIT_DEPTH: u32 = 40;

/// Where a boundary walk cuts a piece, as a fraction of it from its low
/// end: off the middle, because a window is centred on the carrier and
/// the arc crosses its boundary at the middle of a side as often as not,
/// and a solution on a cut is one no piece can isolate.
const EXIT_CUT: f64 = 0.5 - 1.0 / 128.0;

/// The most edge pieces one window's boundary walk examines.
const EXIT_PIECES: u32 = 4096;

/// What one rung of the chart probe proved.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartProbe {
    /// The smallest zero-free transverse margin over the windows,
    /// dimensionless.
    pub(crate) margin: f64,
    /// How many windows were probed.
    pub(crate) windows: u32,
    /// Whether the chain's solution set is certified to be one arc
    /// ([`one_arc`]); asked only where `margin > 0`.
    pub(crate) one_arc: bool,
}

/// The plane distance `φ = n·(S − p₀)` over a parameter rectangle.
fn phi_over<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    (n, p0): ([Interval; 3], [Interval; 3]),
    (u0, u1, v0, v1): (f64, f64, f64, f64),
) -> Interval {
    let b = boxes.rect_box(u0, u1, v0, v1);
    n[0] * (b.x - p0[0]) + n[1] * (b.y - p0[1]) + n[2] * (b.z - p0[2])
}

/// The certified sign of an enclosure: `None` when it straddles zero or
/// is refused.
fn sign_of(i: Interval) -> Option<bool> {
    if !i.is_certified() {
        None
    } else if i.lo() > 0.0 {
        Some(true)
    } else if i.hi() < 0.0 {
        Some(false)
    } else {
        None
    }
}

/// The intersection of two rectangles, `None` when it has no interior.
fn meet(a: UvRect, b: UvRect) -> Option<UvRect> {
    let u = (a.u.0.max(b.u.0), a.u.1.min(b.u.1));
    let v = (a.v.0.max(b.v.0), a.v.1.min(b.v.1));
    (u.0 < u.1 && v.0 < v.1).then_some(UvRect { u, v })
}

/// One resolved piece of a window's boundary, in walk order: `φ` of one
/// certified sign over it, or monotone along it with the certified
/// signs of its two ends where they have one.
#[derive(Clone, Copy, Debug)]
enum Run {
    /// Zero-free, of this sign.
    Constant(bool),
    /// Monotone along the piece: at most one zero.
    Monotone(Option<bool>, Option<bool>),
}

impl Run {
    /// The signs at the run's two ends, in walk order.
    fn ends(self) -> (Option<bool>, Option<bool>) {
        match self {
            Self::Constant(s) => (Some(s), Some(s)),
            Self::Monotone(a, b) => (a, b),
        }
    }

    /// The run walked the other way.
    fn reversed(self) -> Self {
        match self {
            Self::Constant(s) => Self::Constant(s),
            Self::Monotone(a, b) => Self::Monotone(b, a),
        }
    }
}

/// One edge of a window's boundary resolved into [`Run`]s, in the
/// order of its running coordinate: each piece is cut until `φ` is
/// zero-free over it (by the mean-value form, which keeps the
/// cancellation of `n·S` along the piece that a box of `S` per
/// coordinate loses, or by that box), or its derivative along the edge
/// is. `None` when a piece resolves neither way within [`EXIT_DEPTH`]
/// cuts, or `pieces` passes [`EXIT_PIECES`].
fn edge_runs<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    (along_u, c, (a, b)): (bool, f64, (f64, f64)),
    pieces: &mut u32,
) -> Option<Vec<Run>> {
    let n = plane.0;
    let piece = |s: f64, t: f64| if along_u { (s, t, c, c) } else { (c, c, s, t) };
    let sign_at = |s: f64| sign_of(phi_over(boxes, plane, piece(s, s)));
    let mut out = Vec::new();
    let mut stack = vec![(a, b, 0u32)];
    while let Some((s, t, depth)) = stack.pop() {
        *pieces += 1;
        if *pieces > EXIT_PIECES {
            return None;
        }
        // The derivative over a strip as wide across the edge as the
        // piece is long: a net is cut only to a rectangle of positive
        // width, so a strip of none would read the whole span cell's.
        let w = t - s;
        let (r0, r1, r2, r3) = if along_u {
            (s, t, c - w, c + w)
        } else {
            (c - w, c + w, s, t)
        };
        let d = boxes.deriv_box(r0, r1, r2, r3, along_u);
        let slope = n[0] * d.x + n[1] * d.y + n[2] * d.z;
        let (m, h) = (0.5 * (s + t), 0.5 * (t - s));
        let mean = phi_over(boxes, plane, piece(m, m)) + slope * Interval::from_bounds(-h, h);
        if let Some(sign) = sign_of(mean).or_else(|| sign_of(phi_over(boxes, plane, piece(s, t)))) {
            out.push(Run::Constant(sign));
            continue;
        }
        if sign_of(slope).is_some() {
            out.push(Run::Monotone(sign_at(s), sign_at(t)));
            continue;
        }
        if depth >= EXIT_DEPTH {
            return None;
        }
        let cut = s + EXIT_CUT * (t - s);
        // Popped low half first, so `out` runs in the edge's order.
        stack.push((cut, t, depth + 1));
        stack.push((s, cut, depth + 1));
    }
    Some(out)
}

/// How many zeros `φ` has on the boundary of `rect`, each certified a
/// simple crossing. The boundary is walked once around as [`Run`]s.
/// Between two points of certified sign, one monotone run holds one
/// zero where the signs differ and none where they agree; two monotone
/// runs meeting at a point of no certified sign (a zero on a cut or a
/// corner) hold exactly one where the outer signs differ. Anything else
/// — two such points in a row, or outer signs that agree around one,
/// where the count could be zero or two — is `None`, as is an edge
/// [`edge_runs`] does not resolve.
fn boundary_zeros<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    rect: UvRect,
) -> Option<u32> {
    let ((u0, u1), (v0, v1)) = (rect.u, rect.v);
    let mut pieces = 0u32;
    // Counter-clockwise: the top and left edges are walked backwards.
    let mut runs = Vec::new();
    for (edge, back) in [
        ((true, v0, (u0, u1)), false),
        ((false, u1, (v0, v1)), false),
        ((true, v1, (u0, u1)), true),
        ((false, u0, (v0, v1)), true),
    ] {
        let mut r = edge_runs(boxes, plane, edge, &mut pieces)?;
        if back {
            r = r.into_iter().rev().map(Run::reversed).collect();
        }
        runs.extend(r);
    }
    // The sign at each joint (after run `j`): either neighbour's word.
    let k = runs.len();
    let joints: Vec<Option<bool>> = (0..k)
        .map(|j| runs[j].ends().1.or(runs[(j + 1) % k].ends().0))
        .collect();
    let start = joints.iter().position(Option::is_some)?;
    let mut zeros = 0u32;
    let mut from = joints[start]?;
    let mut unknown = 0u32;
    for step in 1..=k {
        let j = (start + step) % k;
        let Some(to) = joints[j] else {
            unknown += 1;
            if unknown > 1 {
                return None;
            }
            continue;
        };
        match (unknown, from == to) {
            (_, false) => zeros += 1,
            (0, true) => {}
            (_, true) => return None,
        }
        from = to;
        unknown = 0;
    }
    Some(zeros)
}

/// Whether `φ` certifiably changes sign between the two ends of the
/// line through `p` along `e` cut to `rect`: then `rect`, convex, holds
/// a zero on the segment between them.
fn holds_zero<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    p: (f64, f64),
    e: (f64, f64),
    rect: UvRect,
) -> bool {
    let (mut lo, mut hi) = (f64::NEG_INFINITY, f64::INFINITY);
    for (pc, ec, (r0, r1)) in [(p.0, e.0, rect.u), (p.1, e.1, rect.v)] {
        if ec == 0.0 {
            if !(r0..=r1).contains(&pc) {
                return false;
            }
            continue;
        }
        let (a, b) = ((r0 - pc) / ec, (r1 - pc) / ec);
        lo = lo.max(a.min(b));
        hi = hi.min(a.max(b));
    }
    if lo.partial_cmp(&hi) != Some(core::cmp::Ordering::Less) {
        return false;
    }
    // Clamped into the rectangle, so the segment between them is in it.
    let at = |t: f64| {
        let (u, v) = (
            (p.0 + t * e.0).clamp(rect.u.0, rect.u.1),
            (p.1 + t * e.1).clamp(rect.v.0, rect.v.1),
        );
        sign_of(phi_over(boxes, plane, (u, u, v, v)))
    };
    matches!((at(lo), at(hi)), (Some(x), Some(y)) if x != y)
}

/// **The chain's solution set is the one arc**, given a zero-free
/// `∂φ/∂e⊥` over every window, each clipped to the wall's `domain` as
/// every enclosure over it is.
///
/// On a clipped window `R`, convex, φ is strictly monotone along each
/// line of direction `e⊥`, so its zero set is a graph over the lines it
/// meets, and each connected piece of it ends on `∂R` at two distinct
/// points: a piece cannot end inside `R`, where the zero set continues,
/// nor touch `∂R` without crossing it, which the walk's monotone pieces
/// exclude. A boundary holding exactly two simple zeros
/// ([`boundary_zeros`]) holds the ends of one piece. Consecutive
/// windows share a zero inside their overlap, on the line along the
/// first one's `e⊥` through the knot they share, so their pieces are
/// one connected arc; a lone window holds a zero on that line through
/// its span's middle. Every zero in the chain lies on that arc.
fn one_arc<T: CertifiedBounds>(
    boxes: &NurbsBoxes<'_, T>,
    plane: ([Interval; 3], [Interval; 3]),
    domain: UvRect,
    pcurve: &NurbsCurve2<T>,
    windows: &[(ChartWindow, (f64, f64))],
) -> bool {
    let at = |t: f64| {
        let p = pcurve.span_at(t).eval_in_span(T::from_f64(t));
        (0.5 * (p.x.lo() + p.x.hi()), 0.5 * (p.y.lo() + p.y.hi()))
    };
    let Some(clipped) = windows
        .iter()
        .map(|(w, _)| meet(w.rect, domain))
        .collect::<Option<Vec<UvRect>>>()
    else {
        return false;
    };
    if !clipped
        .iter()
        .all(|r| boundary_zeros(boxes, plane, *r) == Some(2))
    {
        return false;
    }
    match windows {
        [(w, e)] => holds_zero(boxes, plane, at(w.mid), *e, clipped[0]),
        _ => windows.windows(2).zip(clipped.windows(2)).all(|(w, r)| {
            meet(r[0], r[1])
                .is_some_and(|overlap| holds_zero(boxes, plane, at(w[0].0.ends.1), w[0].1, overlap))
        }),
    }
}

/// Limb 3's enclosure probe for the **plane × NURBS** arm: the same
/// criterion in the NURBS chart, where the locus is
/// `φ(u,v) = n·(S(u,v) − p₀) = 0` and `∇φ = (n·S_u, n·S_v)`. A
/// zero-free enclosure of the component of `∇φ` transverse to the
/// pcurve's own tangent makes the zero set a graph over each window;
/// where it does and a search banks the tube, [`one_arc`] decides
/// whether the chain holds the traced arc and nothing else. The region is [`chart_tube_windows`] at
/// `pad`.
///
/// `Ok(None)` when no window can be probed at this pad; a smaller rung
/// may still answer.
///
/// # Errors
///
/// [`SsiError::TubeDegenerate`] when a window's transverse stretch is
/// zero, or the pcurve's tangent at a span midpoint is zero or not
/// finite: neither depends on the pad, so no smaller rung cures it.
fn probe_tube_chart<T: Decide + Bounds + CertifiedEnclosure>(
    pcurve: &NurbsCurve2<T>,
    surface: &NurbsSurface<T>,
    (origin, normal): (Point3<T>, Vec3<T>),
    pad: (f64, f64),
    banked: bool,
) -> Result<Option<ChartProbe>, SsiError> {
    let boxes = NurbsBoxes::new(surface);
    let Some(windows) = chart_tube_windows(pcurve, pad) else {
        return Ok(None);
    };
    // The plane equation is what the whole limb certifies, so the
    // plane crosses through the CERTIFIED door: a component whose
    // computation left its domain is refused and the transversality
    // margin collapses to zero, rather than a zero-free enclosure of
    // an equation nobody evaluated.
    let n = [normal.x, normal.y, normal.z].map(Interval::from_certified);
    let mut worst = f64::INFINITY;
    let mut probed = Vec::with_capacity(windows.len());
    for w in windows {
        let ((u0, u1), (v0, v1)) = (w.rect.u, w.rect.v);
        // The transverse chart direction is a DIRECTION — structure —
        // so it is selected through the bracket, exactly as the tube
        // ladder's radius is. `powi(2)`, never `t.x * t.x`.
        let t = pcurve.deriv(T::from_f64(w.mid));
        let tn = (t.x.powi(2) + t.y.powi(2)).sqrt().hi();
        let (tx, ty) = (t.x.hi(), t.y.hi());
        // A positive finite norm and a nonzero direction: a lane whose
        // root pads an exact `0` outward reads a zero tangent as a
        // positive norm, so the zero is asked of `(tx, ty)` too.
        // The tangent is the pcurve's alone, so an unusable one refuses
        // at this rung rather than sending the ladder down rungs that
        // read the same tangent.
        if !tn.is_finite() || tn <= 0.0 || tx.abs().max(ty.abs()) <= 0.0 {
            return Err(SsiError::TubeDegenerate(
                super::TubeDegeneracy::PcurveTangentUnusable,
            ));
        }
        let Some(margin) = chart_transverse_margin(&boxes, n, (u0, u1, v0, v1), (tx, ty, tn))?
        else {
            return Ok(None);
        };
        if margin < worst {
            worst = margin;
        }
        probed.push((w, (-ty / tn, tx / tn)));
    }
    if probed.is_empty() {
        return Ok(None);
    }
    let single = worst > 0.0
        && (!banked || {
            let p0 = [origin.x, origin.y, origin.z].map(Interval::from_certified);
            let domain = UvRect {
                u: surface.knots_u().domain(),
                v: surface.knots_v().domain(),
            };
            one_arc(&boxes, (n, p0), domain, pcurve, &probed)
        });
    #[allow(clippy::cast_possible_truncation)]
    Ok(Some(ChartProbe {
        margin: worst,
        windows: probed.len() as u32,
        one_arc: single,
    }))
}

/// [`certify_branch`]'s routing boundary for a NURBS operand with no
/// traced pcurve — the first operand's, which never has one.
pub(crate) const NURBS_LIMBS_NEED_PCURVE: &str = "a NURBS operand's limbs need the traced pcurve \
     (the ℝ⁴ trace supplies it; the ℝ³ trace has none)";

/// [`certify_branch`]'s routing boundary for a NURBS operand whose
/// analytic partner is not a plane.
pub(crate) const CHART_TUBE_NEEDS_PLANE: &str = "the chart uniqueness tube is written for a PLANE \
     against a NURBS surface; another analytic kind needs its own chart form";

/// The uniqueness tube of a certified carrier, as boxes — what the
/// exhaustiveness accounting pass consumes to prove cells "accounted"
/// (spec §4's second state). Same chain limb 3 proved one-arc-ness on,
/// so a cell inside one of these boxes is inside a region where the
/// solution set is exactly the branch already found.
pub(crate) fn tube_boxes<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    radius: f64,
) -> Vec<Box3> {
    box_chain(carrier)
        .into_iter()
        .map(|(b, _)| b.pad(radius))
        .collect()
}

/// Where a search banks a certified branch's tube as accounted
/// (`super::exhaust`), and so where limb 3 must prove the tube holds the
/// traced arc and nothing else.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Banked {
    /// Nothing banks the tube: a carrier certified at rest. Limb 3
    /// proves the locus a graph over each box.
    No,
    /// The plane × NURBS search, over the wall's own knot rectangle.
    Wall,
    /// An ℝ³ search confined to this box.
    Within(Box3),
}

/// Certify a fitted rung-3 carrier against its operand pair — all three
/// limbs, in order, refusing typed at the first failure.
///
/// # Errors
///
/// [`SsiError::CertificateLimb`] naming the limb,
/// [`SsiError::TubeStraddles`] for the sliver case,
/// [`SsiError::ChartSpeed`] when the chart is constant across the locus
/// over a tube window,
/// [`SsiError::FootPointInconclusive`] when a NURBS foot will not
/// converge, [`SsiError::CertificateEscalated`] naming the limb whose trilean
/// escalated.
///
/// `scale` carries the two lengths the certificate is stated over: the
/// folded curvature/extent lever arm the transversality margin is
/// levered by, and the caller's named feature extent, which sets the
/// tube ladder's widest rung.
///
/// `banked` says whether a search banks the tube, and over what region
/// ([`Banked`]); where one does, limb 3 also proves the tube's chain
/// holds one arc there ([`one_arc`], [`one_arc_r3`]).
///
/// The tolerance is `band`'s and only `band`'s. A linear band's
/// `zero()` **is** the run's ε, and every threshold this function
/// applies — the three limbs' trileans and the tube ladder's floor —
/// reads it from there, so there is no second number a caller could
/// certify at.
pub(crate) fn certify_branch<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    pcurve_b: Option<&NurbsCurve2<T>>,
    a: &SsiOperand<'_, T>,
    b: &SsiOperand<'_, T>,
    scale: TubeScale<T>,
    banked: Banked,
    band: Band,
) -> Result<SsiCertificate<T>, SsiError> {
    let TubeScale { arm, extent } = scale;
    let mut on_locus = T::zero();
    let mut hull_sup = T::zero();
    for (op, pc) in [(a, None), (b, pcurve_b)] {
        let (l1, l2) = match op {
            SsiOperand::Analytic(s) => analytic_limbs(carrier, s, band)?,
            SsiOperand::Nurbs(s) => {
                let Some(p) = pc else {
                    return Err(SsiError::UnsupportedCertificate {
                        what: NURBS_LIMBS_NEED_PCURVE,
                    });
                };
                nurbs_limbs(carrier, p, s.surface(), band)?
            }
        };
        on_locus = on_locus.max(l1);
        hull_sup = hull_sup.max(l2);
    }
    // ---- limb 3: pick the widest certifiable tube, then decide ONCE.
    let mut chosen: Option<(SsiTube<f64>, f64, u32)> = None; // (tube, margin, boxes)
    // Rungs whose chain was a graph but not certified to hold one arc.
    let mut not_one_arc = 0u32;
    let chain = box_chain(carrier);
    // The ladder is materialised so its EMPTINESS is a distinguishable
    // outcome. An empty ladder means every rung fell below the floor —
    // a structural fact about extent against ε, decided before any box
    // is probed — and it must refuse as itself rather than fall through
    // to the no-rung-answered path below with a manufactured margin.
    let ladder: Vec<f64> = tube_ladder(extent, band).collect();
    if ladder.is_empty() {
        return Err(SsiError::TubeLadderEmpty {
            extent,
            floor: SSI_TUBE_RADIUS * band.zero(),
        });
    }
    for radius in ladder.iter().copied() {
        let probe = match (a, b) {
            (SsiOperand::Analytic(s1), SsiOperand::Analytic(s2)) => {
                probe_tube_analytic(&chain, s1, s2, radius, banked)
                    .map(|(m, n, one)| (SsiTube::Spatial { radius }, m, n, one))
            }
            (SsiOperand::Analytic(plane), SsiOperand::Nurbs(n))
            | (SsiOperand::Nurbs(n), SsiOperand::Analytic(plane)) => {
                let Surface::Plane { origin, normal, .. } = **plane else {
                    return Err(SsiError::UnsupportedCertificate {
                        what: CHART_TUBE_NEEDS_PLANE,
                    });
                };
                let Some(p) = pcurve_b else {
                    return Err(SsiError::UnsupportedCertificate {
                        what: "the chart uniqueness tube needs the traced pcurve",
                    });
                };
                // The pad per axis: the rung ÷ the operand's minted chart
                // speed along that axis. The padded windows are the
                // proved region and the certificate records them; the
                // rung is the ladder's label, not a metre bound on that
                // region (a window's corner can sit farther than the
                // rung from the carrier).
                let (pad_u, pad_v) = n.speeds().pad(radius);
                let banked = !matches!(banked, Banked::No);
                probe_tube_chart(p, n.surface(), (origin, normal), (pad_u, pad_v), banked)?.map(
                    |c| {
                        let tube = SsiTube::Chart {
                            rung: radius,
                            pad_u,
                            pad_v,
                        };
                        (tube, c.margin, c.windows, c.one_arc)
                    },
                )
            }
            (SsiOperand::Nurbs(_), SsiOperand::Nurbs(_)) => {
                return Err(SsiError::UnsupportedCertificate {
                    what: "NURBS × NURBS routes to the general rung but its uniqueness \
                           tube is not implemented in this build (arms retire one at \
                           a time, each with its proof)",
                });
            }
        };
        let Some((tube, margin, boxes, one_arc)) = probe else {
            continue;
        };
        // A graph over the chain that holds more than the traced arc
        // proves nothing about the arc; a narrower rung may hold it
        // alone.
        if margin > 0.0 && !one_arc {
            not_one_arc += 1;
            continue;
        }
        // Structure selection (C6's f64 lane): the widest rung whose
        // enclosure is zero-free and whose chain holds the one arc wins;
        // the LAST rung is kept even when it fails, so the refusal below
        // carries a real number rather than a vacuum.
        chosen = Some((tube, margin, boxes));
        if margin > 0.0 {
            break;
        }
    }
    if not_one_arc > 0 && !chosen.is_some_and(|(_, m, _)| m > 0.0) {
        return Err(SsiError::TubeNotOneArc { rungs: not_one_arc });
    }
    let Some((tube, margin, boxes)) = chosen else {
        // Rungs were offered and none answered. Structural, and it
        // carries no margin: nothing was ever measured, so there is no
        // honest number to report.
        #[allow(clippy::cast_possible_truncation)]
        return Err(SsiError::TubeProbeSilent {
            rungs: ladder.len() as u32,
        });
    };
    let transversality = tube_transversality(margin, arm, boxes, band)?;
    Ok(SsiCertificate {
        samples: CERT_SAMPLES,
        on_locus_max: on_locus,
        hull_sup,
        tube: tube.map(T::from_f64),
        tube_transversality: transversality,
        tube_boxes: boxes,
    })
}

/// Limb 3's verdict on the chosen rung. `clearance` is interval
/// arithmetic's zero-free lower bound (`f64`, C9) and `arm` the caller's
/// scalar, so the product — the number the trilean classifies — is
/// scalar-typed. The refusal carries the verdict and the reporting
/// margin the classifier saw. Its `Negative` arm is unreachable while
/// `arm` is positive: `zero_free_lower_bound` never returns a negative
/// clearance.
fn tube_transversality<T: Decide>(
    clearance: f64,
    arm: T,
    boxes: u32,
    band: Band,
) -> Result<T, SsiError> {
    let transversality = Margin::levered(T::from_f64(clearance), arm);
    let decided =
        decide_reported("ssi_tube_transversality", transversality, band).map_err(|cause| {
            SsiError::CertificateEscalated {
                limb: SsiLimb::Tube,
                cause,
            }
        })?;
    match Refused::of(decided, band) {
        Some(verdict) => Err(SsiError::TubeStraddles { verdict, boxes }),
        None => Ok(transversality.value()),
    }
}

#[cfg(test)]
mod tests {
    /// **Limb 3's construction site carries the verdict and the margin
    /// it classified.** A clearance inside the zero band refuses `Zero`
    /// with the reporting margin the classifier saw — the levered
    /// clearance, positive or exactly zero: a point at `f64`, a point
    /// enclosure at `Interval` — and a clear one passes. `Negative` has
    /// no row: the clearance is never negative (`zero_free_lower_bound`)
    /// and the arm is positive.
    #[test]
    #[allow(clippy::unwrap_used, clippy::panic)]
    fn the_tube_site_carries_its_verdict_and_margin() {
        use super::{SsiError, tube_transversality};
        use crate::recourse::{Classified, Refused};
        use geom_core::interval::certification::Certification;
        use geom_core::{Band, Interval, MarginDiag};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let verdict = |got: Result<(), SsiError>| match got {
            Err(SsiError::TubeStraddles { verdict, boxes: 4 }) => Some(verdict),
            Ok(()) => None,
            Err(other) => panic!("not the tube's refusal: {other}"),
        };
        let zero = |margin| Some(Refused::Zero(Classified { margin, band }));
        for (clearance, arm, levered) in [
            (5e-10, 1.0, Some(5e-10)),
            (2.5e-10, 2.0, Some(5e-10)),
            (0.0, 1.0, Some(0.0)),
            (1e-6, 1.0, None),
        ] {
            let at_f64 = tube_transversality(clearance, arm, 4, band);
            assert_eq!(
                verdict(at_f64.map(|_| ())),
                levered.and_then(|m| zero(MarginDiag::value(m))),
                "f64: {clearance:e} at {arm}"
            );
            let at_interval = tube_transversality(clearance, Interval::point(arm), 4, band);
            assert_eq!(
                verdict(at_interval.map(|_| ())),
                levered.and_then(|m| zero(MarginDiag::enclosure(m, m))),
                "Interval: {clearance:e} at {arm}"
            );
        }
    }

    /// **The mignitude refuses a refusal that carries real endpoints.**
    /// `zero_free_lower_bound` lives in `ssi::enclose`, a certification
    /// file with no `Real` in scope, and refuses by `!is_certified()`;
    /// this quotient (`Trv`, a strictly positive lower end) is the value
    /// an `is_poison` check would read as a sound bracket, handing its
    /// lower end back as a certified margin. The row pins that the
    /// transversality margin this file reads is `0.0` on it.
    #[test]
    fn the_mignitude_refuses_a_refusal_with_real_endpoints() {
        use super::zero_free_lower_bound;
        use geom_core::{Bounds, Interval};
        let q = Interval::from_bounds(1.0, 2.0) / Interval::from_bounds(0.0, 1.0);
        assert!(!q.is_certified(), "the fixture is a refusal: {q:?}");
        assert!(Bounds::lo(q) > 0.0, "with a positive lower end: {q:?}");
        assert_eq!(zero_free_lower_bound(q), 0.0, "{q:?}");
    }

    /// **The tube ladder's floor is the run band's own ε, exactly.**
    ///
    /// A degradation row, not a violation row. Every other statement
    /// about a uniqueness tube — that it has positive headroom, that it
    /// covers ≥ 1 box, that the accounting terminated — gets *easier*
    /// as the floor drops, so none of them can see the failure this one
    /// is for: a ladder floored at a tolerance finer than the run's
    /// certifies a THINNER tube on a pair where the honest answer is
    /// [`SsiError::CertificateLimb`] on limb 3. The uniqueness theorem
    /// shipped under the same `SsiCertificate` type is then weaker than
    /// the band it is stated in, and nothing else goes red.
    #[allow(clippy::unwrap_used, clippy::panic)]
    #[test]
    fn the_tube_ladders_floor_is_the_run_bands_own_epsilon() {
        use super::{SSI_TUBE_RADIUS, SSI_TUBE_RADIUS_MAX, tube_ladder};
        use geom_core::Band;

        for zero in [1.0e-3_f64, 1.0e-6, 1.0e-9, 1.0e-12] {
            // The escalate edge is arbitrary here: `tube_ladder` reads
            // `band.zero()` and nothing else, so this row is a statement
            // about the coincidence threshold alone. Any value above
            // `zero` satisfies `Band::new`'s `zero < escalate`; it is
            // deliberately NOT the run's K·zero, which would read as a
            // second quantity the ladder consults.
            let band = Band::new(zero, 2.0 * zero).unwrap();
            let floor = SSI_TUBE_RADIUS * zero;
            // An extent chosen so the floor BINDS: the ladder's last
            // possible rung is below it, so the row is about where the
            // ladder stops rather than about it running out of rungs.
            let extent = 1.0e6 * zero;
            let rungs: Vec<f64> = tube_ladder(extent, band).collect();
            assert!(
                !rungs.is_empty(),
                "the ladder must offer a rung at {zero:e}"
            );
            assert!(
                rungs.iter().all(|r| *r >= floor),
                "a rung below the run band's floor {floor:e}: {rungs:?}"
            );
            let last = *rungs.last().unwrap();
            assert!(
                last * 0.5 < floor,
                "the ladder stopped at {last:e} m with room above the floor {floor:e} m \
                 — it is floored at some other tolerance"
            );
            // An extent whose WIDEST rung is already under the floor
            // offers nothing at all: limb 3 refuses rather than proving
            // one-arc-ness over a tube thinner than the carrier's own
            // certified residual.
            assert!(
                tube_ladder(floor, band).next().is_none(),
                "a tube at the floor's own scale must not be certifiable"
            );
            assert!(
                (rungs[0] - SSI_TUBE_RADIUS_MAX * extent).abs() <= f64::EPSILON * extent,
                "the widest rung is the named fraction of the extent"
            );
        }
    }

    /// The chart tube's plane-normal crossing, at the `Interval` scalar.
    ///
    /// The three components of the plane normal enter certification arithmetic through
    /// their own brackets, and at `Interval` a bracket can be sound and
    /// still inadmissible: `sqrt([−1, 4]) + 1` is `[1, 3]` with decoration
    /// `Trv`. The crossing caps that decoration at `Trv`, so a normal
    /// that cannot certify is refused at the crossing — otherwise the
    /// transversality margin is a positive number computed from a plane
    /// equation that was never evaluated where it was asked for.
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    mod normal_crossing_tests {
        use geom::NurbsCurve2;
        use geom::NurbsSurface;
        use geom_core::spline::KnotVector;
        use geom_core::{Interval, Point2, Point3, Real, Vec3};

        use super::super::probe_tube_chart;
        use crate::ssi::{SsiError, TubeDegeneracy};

        fn iv(x: f64) -> Interval {
            Interval::from_f64(x)
        }

        /// A point of the plane through [`u_line`]'s image on the unit
        /// patch, so a normal along `y` cuts the patch in that line.
        fn on_line() -> Point3<Interval> {
            Point3::new(iv(0.0), iv(0.5), iv(0.0))
        }

        /// A domain violation whose bracket is finite AND strictly positive,
        /// so the margin it produces is zero-free: the laundered answer is a
        /// *usable* certificate, not an obvious refusal.
        fn trv_pos() -> Interval {
            Interval::from_bounds(-1.0, 4.0).sqrt() + iv(1.0)
        }

        fn linear_kv() -> KnotVector {
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).expect("valid knots")
        }

        /// `S(u, v) = (u, v, 0)` — a bilinear patch with `S_u = (1, 0, 0)`
        /// and `S_v = (0, 1, 0)`, so `∇φ = (n·S_u, n·S_v) = (n.x, n.y)`.
        fn unit_patch() -> NurbsSurface<Interval> {
            NurbsSurface::new(
                linear_kv(),
                linear_kv(),
                vec![
                    Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                    Point3::new(iv(0.0), iv(1.0), iv(0.0)),
                    Point3::new(iv(1.0), iv(0.0), iv(0.0)),
                    Point3::new(iv(1.0), iv(1.0), iv(0.0)),
                ],
                vec![1.0; 4],
            )
            .expect("valid patch")
        }

        /// A `u`-aligned pcurve, so the transverse chart direction is `e_v`
        /// and the margin reads `n.y` alone.
        fn u_line() -> NurbsCurve2<Interval> {
            NurbsCurve2::new(
                linear_kv(),
                vec![Point2::new(iv(0.1), iv(0.5)), Point2::new(iv(0.9), iv(0.5))],
                vec![1.0, 1.0],
            )
            .expect("valid pcurve")
        }

        /// The control: a certified normal produces the margin the geometry
        /// implies, so the refusal row below is pinning a decoration read
        /// and not a probe that fails on everything.
        #[test]
        fn a_certified_normal_produces_its_margin() {
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let probe = probe_tube_chart(
                &u_line(),
                &unit_patch(),
                (on_line(), normal),
                (0.01, 0.01),
                true,
            )
            .expect("the unit patch's chart is not degenerate")
            .expect("the probe must reach a verdict");
            assert!(
                probe.margin > 0.0,
                "certified normal gave margin {}",
                probe.margin
            );
            assert!(probe.windows > 0, "no span was probed: the row is vacuous");
            assert!(probe.one_arc, "the window holds the line and nothing else");
        }

        /// The row S41 is about: the same geometry with the *same endpoints*
        /// on `n.y`, differing only in the decoration, must not certify.
        #[test]
        fn a_violated_normal_cannot_certify() {
            let n = trv_pos();
            assert_eq!(
                (geom_core::Bounds::lo(n), geom_core::Bounds::hi(n)),
                (1.0, 3.0),
                "fixture drifted"
            );
            assert!(
                geom_core::CertifiedEnclosure::certified_bracket(n).is_none(),
                "fixture drifted: it certifies"
            );
            let normal = Vec3::new(iv(0.0), n, iv(0.0));
            let verdict = probe_tube_chart(
                &u_line(),
                &unit_patch(),
                (on_line(), normal),
                (0.01, 0.01),
                true,
            )
            .expect("the unit patch's chart is not degenerate");
            let margin = verdict.map_or(0.0, |c| c.margin);
            assert_eq!(
                margin, 0.0,
                "a domain-violated plane normal produced transversality \
                 margin {margin} — the crossing read the BRACKET door, so \
                 the chart tube certifies uniqueness from a plane equation \
                 that was clamped out of its own domain"
            );
        }

        /// The same principle on the PCURVE: a control coordinate that left
        /// its domain carries real endpoints (`sqrt([−1, 0.01]) + 0.1` is
        /// about `[0.1, 0.2]` at `Trv`), the span window's hull refuses it as NaI,
        /// and NaI's NaN endpoints must not become a chart window — a NaN
        /// window end lands on the first span in `span_range`, and the
        /// derivative boxes of an arbitrary cell would then certify.
        #[test]
        fn a_violated_pcurve_coordinate_cannot_certify() {
            let bad = Interval::from_bounds(-1.0, 0.01).sqrt() + iv(0.1);
            let (lo, hi) = (geom_core::Bounds::lo(bad), geom_core::Bounds::hi(bad));
            assert!(
                (0.09..0.21).contains(&lo) && (0.09..0.21).contains(&hi),
                "fixture drifted: [{lo}, {hi}] is not a real bracket near [0.1, 0.2]"
            );
            assert!(!bad.is_certified(), "fixture drifted: it certifies");
            let pcurve = NurbsCurve2::new(
                linear_kv(),
                vec![Point2::new(bad, iv(0.5)), Point2::new(iv(0.9), iv(0.5))],
                vec![1.0, 1.0],
            )
            .expect("valid pcurve");
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let verdict = probe_tube_chart(
                &pcurve,
                &unit_patch(),
                (on_line(), normal),
                (0.01, 0.01),
                true,
            );
            assert!(
                matches!(verdict, Ok(None)),
                "a pcurve whose control coordinate left its domain produced the \
                 transversality verdict {verdict:?} — the span window's refused \
                 hull was read for its endpoints"
            );
        }

        /// **A chart constant across the locus refuses by name, at the
        /// first rung.** On a patch whose net is one point, `S_u` and
        /// `S_v` are exactly zero, so the stretch along e⊥ is zero over
        /// every window; a smaller pad's window lies inside this one, so
        /// answering "no enclosure, try the next rung" would only run the
        /// ladder down to `TubeProbeSilent`, the wrong diagnosis.
        #[test]
        fn a_chart_constant_across_the_locus_refuses_rather_than_trying_a_smaller_rung() {
            let p = Point3::new(iv(0.3), iv(0.3), iv(0.0));
            let point_patch = NurbsSurface::new(linear_kv(), linear_kv(), vec![p; 4], vec![1.0; 4])
                .expect("a patch a caller can build");
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let verdict = probe_tube_chart(
                &u_line(),
                &point_patch,
                (on_line(), normal),
                (0.01, 0.01),
                true,
            );
            assert!(
                matches!(
                    verdict,
                    Err(SsiError::TubeDegenerate(
                        TubeDegeneracy::WallConstantAcrossLocus
                    ))
                ),
                "a chart constant across the locus answered {verdict:?} instead of \
                 refusing by name"
            );
        }

        /// **A chart constant across the locus refuses by name when its
        /// net is not one point.** Rows `a, a` and `b, b`: `S_v` is exactly
        /// zero and `S_u` is not. The tube windows are cut below the span,
        /// and `a − c`, `b − c` do not round exactly, so the cut box alone
        /// reads `S_v` as a few ulps around zero rather than zero; met with
        /// the whole cell's exact zero, the stretch along e⊥ = `e_v` is
        /// zero again and the probe names the degeneracy rather than
        /// reporting a margin of 0.
        #[test]
        fn a_chart_constant_across_the_locus_refuses_on_a_net_that_is_not_one_point() {
            let a = Point3::new(iv(0.1), iv(0.1), iv(0.1));
            let b = Point3::new(iv(0.7), iv(0.7), iv(0.7));
            let ridge = NurbsSurface::new(linear_kv(), linear_kv(), vec![a, a, b, b], vec![1.0; 4])
                .expect("a patch a caller can build");
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let verdict =
                probe_tube_chart(&u_line(), &ridge, (on_line(), normal), (0.01, 0.01), true);
            assert!(
                matches!(
                    verdict,
                    Err(SsiError::TubeDegenerate(
                        TubeDegeneracy::WallConstantAcrossLocus
                    ))
                ),
                "a chart constant along v answered {verdict:?} instead of refusing by name"
            );
        }

        /// **A pcurve with no tangent refuses by name, at the first
        /// rung.** A linear pcurve whose two control points coincide has
        /// a zero tangent everywhere; the tangent is the pcurve's whatever
        /// the pad, so trying the next rung reads the same zero.
        #[test]
        fn a_pcurve_with_no_tangent_refuses_rather_than_trying_a_smaller_rung() {
            let still = NurbsCurve2::new(
                linear_kv(),
                vec![Point2::new(iv(0.5), iv(0.5)), Point2::new(iv(0.5), iv(0.5))],
                vec![1.0, 1.0],
            )
            .expect("a pcurve a caller can build");
            let normal = Vec3::new(iv(0.0), iv(2.0), iv(0.0));
            let verdict = probe_tube_chart(
                &still,
                &unit_patch(),
                (on_line(), normal),
                (0.01, 0.01),
                true,
            );
            assert!(
                matches!(
                    verdict,
                    Err(SsiError::TubeDegenerate(
                        TubeDegeneracy::PcurveTangentUnusable
                    ))
                ),
                "a pcurve with no tangent answered {verdict:?} instead of refusing by name"
            );
        }
    }

    /// A degree-2 carrier on `[0, 1]` with the given interior knots.
    #[allow(clippy::unwrap_used)]
    fn carrier(interior: &[f64]) -> geom::NurbsCurve3<f64> {
        use geom_core::Point3;
        use geom_core::spline::KnotVector;
        let mut knots = vec![0.0, 0.0, 0.0];
        knots.extend_from_slice(interior);
        knots.extend([1.0, 1.0, 1.0]);
        let kv = KnotVector::clamped(knots, 2).unwrap();
        let n = kv.control_count();
        #[allow(clippy::cast_precision_loss)]
        let control = (0..n).map(|i| Point3::new(i as f64, 0.0, 0.0)).collect();
        geom::NurbsCurve3::new(kv, control, vec![1.0; n]).unwrap()
    }

    /// `refined` inserts the DOMAIN's 32nds, skipping a grid point only
    /// where a knot sits on it bit for bit: `0.5` is skipped, while a
    /// knot one ulp above `2/32` does NOT suppress `2/32`.
    #[test]
    fn refined_inserts_the_domain_grid_skipping_bit_equal_knots() {
        let near = f64::from_bits(0.0625f64.to_bits() + 1);
        let fine = super::refined(&carrier(&[near, 0.5]));
        let mut want = vec![0.0, 0.0, 0.0, near];
        want.extend((1..32).map(|k| f64::from(k) / 32.0));
        want.extend([1.0, 1.0, 1.0]);
        want.sort_by(f64::total_cmp);
        assert_eq!(fine.knots().knots(), want);
    }

    /// `chart_breaks` skips a grid point beside a knot of either curve:
    /// a carrier knot one ulp above `2/32` drops `2/32`, a pcurve knot
    /// one ulp below `12/32` drops `12/32`, and every other 32nd stays.
    #[test]
    fn chart_breaks_skip_a_grid_point_beside_either_curves_knot() {
        let above = f64::from_bits(0.0625f64.to_bits() + 1);
        let below = f64::from_bits(0.375f64.to_bits() - 1);
        let breaks = super::chart_breaks(carrier(&[above]).knots(), carrier(&[below]).knots());
        let want: Vec<f64> = (1..32)
            .filter(|&k| k != 2 && k != 12)
            .map(|k| f64::from(k) / 32.0)
            .collect();
        assert_eq!(breaks, want);
    }

    /// `refined`'s cut-off: a carrier with `SSI_CERT_SPANS + degree`
    /// control points is returned as it came, one with a control point
    /// fewer still takes the whole grid. The interior knots are odd
    /// 128ths, none on the 32nds grid.
    #[test]
    fn refined_leaves_a_carrier_at_the_cut_off_alone() {
        let odd = |n: i32| -> Vec<f64> { (0..n).map(|j| f64::from(2 * j + 1) / 128.0).collect() };
        let at = carrier(&odd(31));
        assert_eq!(at.knots().control_count(), super::SSI_CERT_SPANS + 2);
        assert_eq!(super::refined(&at).knots().knots(), at.knots().knots());
        let below = carrier(&odd(30));
        assert_eq!(below.knots().control_count(), 33);
        assert_eq!(super::refined(&below).knots().control_count(), 33 + 31);
    }

    /// **Each pad widens its own axis.** A one-span linear pcurve whose
    /// hull is `[0.2, 0.6] × [0.4, 0.5]`, padded by different amounts
    /// along `u` and `v`: the window is the hull widened by `pad_u` along
    /// `u` and by `pad_v` along `v`, bit for bit, and its midpoint is the
    /// span's.
    #[test]
    #[allow(clippy::unwrap_used, clippy::expect_used)]
    fn each_pad_widens_its_own_axis_of_the_window() {
        use geom::NurbsCurve2;
        use geom_core::Point2;
        use geom_core::spline::KnotVector;

        use super::chart_tube_windows;

        let pc = NurbsCurve2::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point2::new(0.2, 0.4), Point2::new(0.6, 0.5)],
            vec![1.0, 1.0],
        )
        .unwrap();
        let (pad_u, pad_v) = (0.125, 0.375);
        let windows = chart_tube_windows(&pc, (pad_u, pad_v)).expect("certified hulls");
        assert_eq!(windows.len(), 1, "one span, one window");
        let w = windows[0];
        let got = [w.rect.u.0, w.rect.u.1, w.rect.v.0, w.rect.v.1, w.mid].map(f64::to_bits);
        let want = [0.2 - pad_u, 0.6 + pad_u, 0.4 - pad_v, 0.5 + pad_v, 0.5].map(f64::to_bits);
        assert_eq!(
            got, want,
            "window {:?} mid {}: u must widen by pad_u and v by pad_v",
            w.rect, w.mid
        );
    }

    /// **The chart tube is recorded per axis, as proved, and accounting
    /// banks the probe's windows.** One plane × NURBS run on an extruded
    /// wall whose section is slower than its 3 m extrusion, refined along
    /// `u` so a tube window narrower than the domain reads narrower
    /// derivative boxes (on a one-span wall the boxes are the span's
    /// whatever the pad, and along an extrusion `S_v` is constant, so
    /// only the `u` pad can move the margin). Per certified branch:
    ///
    /// - the recorded pad is the rung over each axis's OWN speed, bit for
    ///   bit, and the probe re-run at that pad returns the margin and box
    ///   count the certificate holds — while the probe at the max-folded
    ///   pad returns another margin, so the agreement is about the pad;
    /// - the slower axis's pad is wider than the max-folded pad that
    ///   accounting used to bank on both axes;
    /// - the windows accounting banks are [`chart_tube_windows`] at the
    ///   recorded pad, bit for bit.
    ///
    /// ε-invariant: the band is fixed, so the env row does not move it.
    #[test]
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    fn the_chart_tube_records_its_per_axis_pad_and_accounting_banks_its_windows() {
        use geom::{NurbsSurface, Surface};
        use geom_core::spline::KnotVector;
        use geom_core::{Band, Margin, Point3, Vec3};

        use super::{SsiTube, chart_tube_windows, probe_tube_chart};
        use crate::ssi::{ChartedNurbs, SsiDomain, branch_chart_tubes, plane_nurbs_ssi};

        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let mut control = Vec::with_capacity(8);
        for (x, y) in [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)] {
            control.push(Point3::new(x, y, 0.0));
            control.push(Point3::new(x, y, 3.0));
        }
        let wall = NurbsSurface::new(ku, kv, control, vec![1.0; 8])
            .unwrap()
            .refine_knots_u(&(1..16).map(|k| f64::from(k) / 16.0).collect::<Vec<_>>())
            .unwrap();
        let n = Vec3::new(0.0, 0.25, 1.0);
        let n = n / n.norm();
        let u = Vec3::new(1.0, 0.0, 0.0);
        let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
        let origin = Point3::new(0.0, 0.0, 0.4);
        let plane = Surface::Plane {
            origin,
            normal: n,
            u_ref: u,
        };
        let domain = SsiDomain {
            center: Point3::new(0.5, 0.0, 0.4),
            half_extent: 2.0,
            extent: 1.5,
            floor_scale: 1.0,
        };
        let band = Band::new(1e-9, 1e-8).unwrap();

        let speeds = ChartedNurbs::mint(&wall).unwrap().speeds();
        let (su, sv) = (speeds.u.get(), speeds.v.get());
        assert!(
            su < sv,
            "FIXTURE: the section is shorter than the 3 m extrusion, so u is the \
             slower axis (su {su:e}, sv {sv:e})"
        );
        let out = plane_nurbs_ssi(&plane, &wall, domain, band).expect("the wall certifies");
        assert!(!out.branches.is_empty(), "FIXTURE: the plane cuts the wall");
        for (i, b) in out.branches.iter().enumerate() {
            let SsiTube::Chart { rung, pad_u, pad_v } = b.certificate.tube else {
                panic!(
                    "branch {i}: the plane × NURBS arm proved {:?}",
                    b.certificate.tube
                );
            };
            let pc = b
                .pcurve_b
                .as_ref()
                .expect("the ℝ⁴ arm fits the wall's pcurve");

            // The recorded pad is the rung over each axis's own speed.
            let want = (rung / su, rung / sv);
            assert_eq!(
                (pad_u.to_bits(), pad_v.to_bits()),
                (want.0.to_bits(), want.1.to_bits()),
                "branch {i}: recorded pad ({pad_u:e}, {pad_v:e}), rung {rung:e} over the \
                 axis speeds gives ({:e}, {:e})",
                want.0,
                want.1
            );
            // ... and it is the pad the probe proved over.
            let probe = probe_tube_chart(pc, &wall, (origin, n), (pad_u, pad_v), true)
                .unwrap()
                .expect("the recorded pad probes");
            assert!(
                probe.one_arc,
                "branch {i}: the recorded pad's chain is not one arc"
            );
            let (margin, boxes) = (probe.margin, probe.windows);
            let levered = Margin::levered(margin, domain.extent).value();
            assert_eq!(
                (levered.to_bits(), boxes),
                (
                    b.certificate.tube_transversality.to_bits(),
                    b.certificate.tube_boxes
                ),
                "branch {i}: the probe at the recorded pad is not the certificate's"
            );
            let folded = rung / su.max(sv);
            let at_fold = probe_tube_chart(pc, &wall, (origin, n), (folded, folded), true).unwrap();
            assert!(
                at_fold.is_none_or(|c| c.margin.to_bits() != margin.to_bits()),
                "FIXTURE: branch {i}'s margin does not depend on the pad, so the \
                 agreement above says nothing about which pad was probed"
            );

            // The slower axis is padded wider than the max-fold pad.
            assert!(
                pad_u > folded && pad_v.to_bits() == folded.to_bits(),
                "branch {i}: per-axis pad ({pad_u:e}, {pad_v:e}) against the max-fold \
                 pad {folded:e}: the slower axis (u) must be wider"
            );

            // Accounting banks the probe's windows, bit for bit.
            let probed: Vec<_> = chart_tube_windows(pc, (pad_u, pad_v))
                .expect("a certified pcurve's hulls are certified")
                .into_iter()
                .map(|w| w.rect)
                .collect();
            let banked = branch_chart_tubes(b);
            let bits = |r: &[crate::ssi::exhaust::UvRect]| -> Vec<[u64; 4]> {
                r.iter()
                    .map(|w| [w.u.0, w.u.1, w.v.0, w.v.1].map(f64::to_bits))
                    .collect()
            };
            assert!(
                !banked.is_empty(),
                "branch {i}: accounting banked no window"
            );
            assert_eq!(
                bits(&banked),
                bits(&probed),
                "branch {i}: accounting banked other windows than limb 3 proved over"
            );
        }
    }

    /// **A window that is a graph over its lines can hold two arcs.**
    /// The fold `z = c·x + a·x² + 4β·y(L − y)/L²` against `z = 0` (β = ε,
    /// c = 80ε, a = 0.28·c²/β, w = β/c, L = 1.2w, `x ∈ [−1.5w, 1.8w]`) has
    /// `∂φ/∂x > 0` everywhere, and its locus is two arcs, each from a `v`
    /// side to the low `u` side. A pcurve straight across the gap between
    /// them, padded to a window over the whole wall, is a graph there —
    /// the margin is zero-free — but the window's boundary holds four
    /// zeros, so the chain is not one arc; read at rest, where nothing
    /// banks the tube, the graph is all limb 3 proves and the probe says
    /// so.
    #[test]
    #[allow(clippy::unwrap_used, clippy::expect_used)]
    fn a_graph_window_holding_two_arcs_is_not_one_arc() {
        use geom::{NurbsCurve2, NurbsSurface};
        use geom_core::spline::KnotVector;
        use geom_core::{Point2, Point3, Vec3};

        use super::probe_tube_chart;

        let beta = 1e-9;
        let c = 80.0 * beta;
        let a = 0.28 * c * c / beta;
        let w = beta / c;
        let l = 1.2 * w;
        let (x0, x1) = (-1.5 * w, 1.8 * w);
        let g = |x: f64| c * x + a * x * x;
        let h = |y: f64| 4.0 * beta * y * (l - y) / (l * l);
        let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * (c + 2.0 * a * x0), g(x1)];
        let hb = [0.0, 2.0 * beta, 0.0];
        let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [0.0, 0.5 * l, l]);
        let k = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = (0..9)
            .map(|i| Point3::new(xs[i / 3], ys[i % 3], gb[i / 3] + hb[i % 3]))
            .collect();
        let wall = NurbsSurface::new(k(), k(), control, vec![1.0; 9]).unwrap();
        assert!(
            (0..=8).all(|i| {
                let y = l * f64::from(i) / 8.0;
                (wall.eval(0.5, f64::from(i) / 8.0).z - g(0.5 * (x0 + x1)) - h(y)).abs() < 1e-18
            }),
            "FIXTURE: the net is the graph"
        );
        let u0 = -x0 / (x1 - x0);
        let across = NurbsCurve2::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point2::new(u0, 0.0), Point2::new(u0, 1.0)],
            vec![1.0, 1.0],
        )
        .unwrap();
        let plane = (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let pad = (3.0, 8.0);
        let banked = probe_tube_chart(&across, &wall, plane, pad, true)
            .expect("a usable chart")
            .expect("the window probes");
        assert!(
            banked.margin > 0.0,
            "FIXTURE: the window is a graph over its lines (margin {})",
            banked.margin
        );
        assert!(!banked.one_arc, "a window holding two arcs proved one arc");
        let at_rest = probe_tube_chart(&across, &wall, plane, pad, false)
            .expect("a usable chart")
            .expect("the window probes");
        assert!(
            at_rest.one_arc && at_rest.margin.to_bits() == banked.margin.to_bits(),
            "at rest the probe proves the graph and asks no more"
        );
    }
}

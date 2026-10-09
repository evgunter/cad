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
//! > in `B` is a graph over the `e` axis: no branch point, no loop, no
//! > second sheet **at the same `e`-level**.
//!
//! It is a convexity/mean-value argument over the enclosure, not a bare
//! appeal to the implicit function theorem (which is local and would
//! not, by itself, cover the whole box). A graph is not yet one arc: a
//! second arc beside the first along `e`, leaving through the box's
//! sides, passes the slice argument. The solution set in the chain is
//! one arc spanning the carrier, and limb 3 proves it at every door
//! ([`super::one_arc`]): each box holds one piece, consecutive boxes
//! share it, and it reaches both ends of the carrier. A search banks
//! every cell of a tube as accounted ([`super::exhaust`]); what lies
//! outside every tube is that pass's to exclude or refuse.
//!
//! Which rung speaks when none certifies is [`limb_three`]'s: the
//! narrowest probed, as the most specific reading of the carrier.
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
    Band, Bounds, CertifiedEnclosure, Decide, Decided, Indeterminate, Interval, Margin, Point3,
    Real, Sign, Vec3,
};

use crate::certify::CertCheck;
use crate::certify::{CERT_SAMPLES, sample_param};
use crate::dihedral::{decide_positive, decide_reported};

use super::enclose::{
    Box3, NurbsBoxes, chart_transverse_margin, graph_margin, zero_free_lower_bound,
};
use super::exhaust::UvRect;
use super::one_arc::{Shortfall, dominant_axis, one_arc, one_arc_r3};
use super::section::{BandVerdict, band_verdict};
use super::{SsiError, SsiOperand};

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
    /// ring's zero-free lower bound times the caller's feature extent,
    /// so it carries the extent's scalar.
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
    at: &mut Vec<RefusedSpan>,
) -> Result<(T, T), SsiError> {
    // Limb 2's hull: the implicit form composed with the refined
    // carrier, in metres.
    let hull = || -> Result<Hull, SsiError> {
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
        Ok(Hull {
            sup: composite.sup_bound() * to_meters,
            breaks: composite.num.breaks().to_vec(),
            spans: composite
                .span_sup_bounds()
                .into_iter()
                .map(|b| b * to_meters)
                .collect(),
        })
    };
    // ---- limb 1: the fixed schedule ----
    let (t0, t1) = carrier.domain();
    let mut worst = T::zero();
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        let r = crate::implicit::implicit_residual(surface, carrier.eval(T::from_f64(t))).abs();
        // `max`, not a `>` branch: the running worst is a scalar-typed
        // quantity now, and generic evaluation code does not compare.
        worst = worst.max(r);
        let decided = decide_reported("ssi_on_locus", Margin::of(r), band);
        if locatable(&decided) {
            at.push(RefusedSpan { lo: t, hi: t });
            if let Ok(hull) = hull() {
                hull.uncleared(band, at);
            }
        }
        match decided {
            // Zero is the affirmative: the residual is zero to
            // tolerance (the `dihedral_wedge` convention).
            Ok(Decided {
                sign: Sign::Zero, ..
            }) => {}
            Ok(Decided { margin, .. }) => {
                return Err(SsiError::CertificateLimb {
                    limb: SsiLimb::OnLocus,
                    value: r.hi(),
                    margin,
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
    let hull = hull()?;
    // Interval arithmetic answers with an `f64` upper bound — that is what a hull
    // bound is — and it is lifted here so the limb is banded at the
    // caller's scalar like every other residual (field docs).
    let sup = T::from_f64(hull.sup);
    let decided = decide_reported("ssi_hull_sup", Margin::of(sup), band);
    if locatable(&decided) {
        hull.uncleared(band, at);
    }
    match decided {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok((worst, sup)),
        Ok(Decided { margin, .. }) => Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value: sup.hi(),
            margin,
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
    at: &mut Vec<RefusedSpan>,
) -> Result<(T, T), SsiError> {
    // Limb 2's hull: `S(P(t)) − C(t)` enclosed as one composite, in
    // metres.
    let hull = || -> Result<Hull, SsiError> {
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
        let cdata =
            CurveCertData::new(carrier.knots(), carrier.weights(), &coords).map_err(|_| {
                SsiError::UnsupportedCertificate {
                    what: "the fitted carrier's enclosure data is malformed",
                }
            })?;
        let pcoords = pcurve.certified_coords();
        let pdata =
            CurveCertData::new(pcurve.knots(), pcurve.weights(), &pcoords).map_err(|_| {
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
        let residual =
            tensor::surface_curve_residual(&sdata, &pdata, &cdata, &extra).map_err(|_| {
                SsiError::UnsupportedCertificate {
                    what: "the tensor composite refused the carrier/pcurve pair (mismatched \
                       channel counts or knot domains — the shared-parameter identity is the entry \
                       requirement)",
                }
            })?;
        Ok(Hull {
            sup: residual.sup_bound(),
            breaks: residual.breaks().to_vec(),
            spans: residual.span_bounds().to_vec(),
        })
    };
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
        let decided = decide_reported("ssi_on_locus_foot", Margin::of(proj.distance), band);
        if locatable(&decided) {
            at.push(RefusedSpan { lo: t, hi: t });
            if let Ok(hull) = hull() {
                hull.uncleared(band, at);
            }
        }
        match decided {
            Ok(Decided {
                sign: Sign::Zero, ..
            }) => {}
            Ok(Decided { margin, .. }) => {
                return Err(SsiError::CertificateLimb {
                    limb: SsiLimb::OnLocus,
                    value: proj.distance.hi(),
                    margin,
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
            match decide_reported("ssi_foot_orthogonality", margin, band) {
                Ok(Decided {
                    sign: Sign::Zero, ..
                }) => {}
                Ok(Decided {
                    margin: reading, ..
                }) => {
                    return Err(SsiError::CertificateLimb {
                        limb: SsiLimb::OnLocus,
                        value: margin.value().hi(),
                        margin: reading,
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
    let hull = hull()?;
    let sup = hull.sup;
    // Unlike the analytic arm, the NURBS arm needs NO exactness gate:
    // every coefficient of every operand entered interval arithmetic through its
    // own bracket (`certified_coords`), so a widened control net widens the
    // composite and the bound stays honest.
    let sup = T::from_f64(sup);
    let decided = decide_reported("ssi_hull_sup_chart", Margin::of(sup), band);
    if locatable(&decided) {
        hull.uncleared(band, at);
    }
    match decided {
        Ok(Decided {
            sign: Sign::Zero, ..
        }) => Ok((worst, sup)),
        Ok(Decided { margin, .. }) => Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value: sup.hi(),
            margin,
        }),
        Err(cause) => Err(SsiError::CertificateEscalated {
            limb: SsiLimb::HullSup,
            cause,
        }),
    }
}

/// Whether a limb's verdict is a refusal refinement can locate: a
/// definite one, or one undecided on a margin that is a number. A
/// margin that is no number is no residual a denser carrier answers, so
/// that refusal stands at once.
fn locatable(decided: &Result<Decided, geom_core::Indeterminate>) -> bool {
    match decided {
        Ok(Decided { sign, .. }) => *sign != Sign::Zero,
        Err(cause) => !cause.margin.is_invalid(),
    }
}

/// Limb 2's composite, span by span: a certified upper bound in metres
/// on the carrier's residual over each span, span `j` covering
/// `[breaks[j], breaks[j+1]]`.
struct Hull {
    /// The bound over the whole carrier, the composite's own.
    sup: f64,
    breaks: Vec<f64>,
    spans: Vec<f64>,
}

impl Hull {
    /// Pushes onto `at` every span whose bound does not clear the band's
    /// zero. A selection of where to look (C6's f64 lane), never a
    /// decision: the refusal it locates is decided by the limb, and a
    /// refused (`NaN`) span is selected.
    fn uncleared(&self, band: Band, at: &mut Vec<RefusedSpan>) {
        for (w, bound) in self.breaks.windows(2).zip(&self.spans) {
            if !matches!(
                bound.partial_cmp(&band.zero()),
                Some(core::cmp::Ordering::Less | core::cmp::Ordering::Equal)
            ) {
                at.push(RefusedSpan { lo: w[0], hi: w[1] });
            }
        }
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

/// One rung of either arm's probe: the tube it records, the chain's
/// smallest zero-free margin (dimensionless, the `sin θ` scale), its box
/// count, and, where the margin is zero-free, what the one-arc proof
/// found. `one_arc` is `None` where the chain is no graph, so the proof
/// had no hypothesis; it never reads as proved where nothing was.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Rung {
    pub(crate) tube: SsiTube<f64>,
    pub(crate) margin: f64,
    pub(crate) boxes: u32,
    pub(crate) one_arc: Option<Result<(), Shortfall>>,
}

/// Limb 3's **enclosure probe** for an analytic pair: the graph
/// criterion in ℝ³ over a box chain of the given radius, and, where the
/// margin is zero-free, the one-arc proof over the chain cut to `slab`
/// where a search clips to one ([`one_arc_r3`]), spanning the carrier
/// between its `ends`.
///
/// Pure enclosure arithmetic — **no trilean here**. Choosing the tube's
/// radius is cache *structure* (C6's f64 selection lane), and mixing
/// the ladder's rejected rungs into the K-funnel would both pollute the
/// telemetry and blur the one decision that matters. The single named
/// decision runs once, upstairs, on the chosen radius.
///
/// `None` when the chain is broken or an enclosure refused, which is a
/// definite structural refusal.
fn probe_tube_analytic<T: Decide + Bounds + CertifiedEnclosure>(
    chain: &[(Box3, Vec3<T>)],
    s1: &Surface<T>,
    s2: &Surface<T>,
    radius: f64,
    (slab, ends, eps): (Option<Box3>, [Box3; 2], f64),
) -> Option<Rung> {
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
    let axis = |e: &Vec3<T>| dominant_axis([e.x, e.y, e.z].map(|c| 0.5 * (c.lo() + c.hi())));
    let (Some((_, e0)), Some((_, e1))) = (chain.first(), chain.last()) else {
        return None;
    };
    let one_arc = (worst > 0.0).then(|| {
        one_arc_r3(
            s1,
            s2,
            (&padded, slab),
            [(ends[0], axis(e0)), (ends[1], axis(e1))],
            eps,
        )
    });
    #[allow(clippy::cast_possible_truncation)]
    Some(Rung {
        tube: SsiTube::Spatial { radius },
        margin: worst,
        boxes: chain.len() as u32,
        one_arc,
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

/// Limb 3's enclosure probe for the **plane × NURBS** arm: the same
/// criterion in the NURBS chart, where the locus is
/// `φ(u,v) = n·(S(u,v) − p₀) = 0` and `∇φ = (n·S_u, n·S_v)`.
///
/// - A zero-free enclosure of the component of `∇φ` transverse to the
///   pcurve's own tangent makes the zero set a graph over each window.
/// - Where it does, [`one_arc`] decides whether the chain holds one arc
///   spanning the carrier between its `ends`, and nothing else.
///
/// The region is [`chart_tube_windows`] at `pad`, the ladder's `rung`
/// over each axis's chart speed.
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
    (rung, pad): (f64, (f64, f64)),
    (ends, band): ([Box3; 2], Band),
) -> Result<Option<Rung>, SsiError> {
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
    let one = (worst > 0.0).then(|| {
        let p0 = [origin.x, origin.y, origin.z].map(Interval::from_certified);
        let domain = UvRect {
            u: surface.knots_u().domain(),
            v: surface.knots_v().domain(),
        };
        one_arc(&boxes, (n, p0), (domain, band), pcurve, &probed, ends)
    });
    #[allow(clippy::cast_possible_truncation)]
    Ok(Some(Rung {
        tube: SsiTube::Chart {
            rung,
            pad_u: pad.0,
            pad_v: pad.1,
        },
        margin: worst,
        boxes: probed.len() as u32,
        one_arc: one,
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

/// Why limb 3 did not prove a tube's chain one arc, read at the
/// narrowest rung whose chain was a graph but not proved one arc.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OneArcRefusal {
    /// A box of the chain, cut to the wall's domain (and to a search's
    /// slab where it clips to one), holds this many simple solutions on
    /// its boundary, certified, where one arc through it gives two. None:
    /// no arc runs through it. More: another arc of the locus lies in it,
    /// or the locus leaves the box's region and comes back inside it.
    Count {
        /// The certified count.
        solutions: u32,
    },
    /// Two consecutive boxes each hold one piece of the locus, and the
    /// reading that would show the pieces meet certifies no shared
    /// solution there.
    Unlinked,
    /// The chain's pieces stop short of an end of the carrier, certified:
    /// an arc's box holds no solution on the slice through that end and
    /// none within ε of it, or a side's stretch does not reach the end,
    /// or the boundary pass reads the side clear there. Read also where a
    /// box resolved no piece, when the end is certified unreached by its
    /// own box: no chain reaches it.
    Short,
    /// A walk of a box's boundary resolved no count, or a linking
    /// reading no sign. A solution on the boundary may be tangential to
    /// it, sit on a corner, or lie below the walk's resolution. The
    /// cause carries the escalation as the funnel recorded it.
    Undecided(Indeterminate),
}

impl core::fmt::Display for OneArcRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Count { solutions: 0 } => {
                "a box of the chain holds no boundary solution: no arc of the locus runs \
                 through it"
            }
            Self::Count { solutions } => {
                return write!(
                    f,
                    "a box of the chain holds {solutions} boundary solutions, not two: a \
                     second arc, or the locus leaving and re-entering the box's region"
                );
            }
            Self::Unlinked => {
                "two consecutive boxes each hold one piece, and no shared solution joins them"
            }
            Self::Short => {
                "the pieces stop short of an end of the carrier: no solution lies on the \
                 slice through that end or within the tolerance of it"
            }
            Self::Undecided(_) => {
                "a box's boundary walk resolved no count: a solution tangent to it, on a \
                 corner, or below the walk's resolution"
            }
        })
    }
}

/// Limb 3's ladder: probe each rung, widest first, and take the first
/// whose chain is a graph (zero-free margin) and, where one arc was
/// asked, proved one arc. That rung's margin is decided once
/// ([`tube_transversality`]).
///
/// With no rung taken, the refusal is the narrowest rung's:
/// - a rung that straddled refuses as the band decides its margin;
/// - a rung that was a graph but not one arc refuses
///   [`SsiError::TubeNotOneArc`] with what that rung found.
///
/// A narrower rung is the more specific reading of the same carrier, so
/// it speaks over every wider one.
fn limb_three<T: Decide>(
    (widest, extent): (f64, T),
    band: Band,
    mut probe: impl FnMut(f64) -> Result<Option<Rung>, SsiError>,
) -> Result<(Rung, T), SsiError> {
    // The ladder is materialised so its EMPTINESS is a distinguishable
    // outcome. An empty ladder means every rung fell below the floor —
    // a structural fact about extent against ε, decided before any box
    // is probed — and it must refuse as itself rather than fall through
    // to the no-rung-answered path below with a manufactured margin.
    let ladder: Vec<f64> = tube_ladder(widest, band).collect();
    if ladder.is_empty() {
        return Err(SsiError::TubeLadderEmpty {
            extent: widest,
            floor: SSI_TUBE_RADIUS * band.zero(),
        });
    }
    let mut not_one_arc = 0u32;
    let mut narrowest: Option<Rung> = None;
    for radius in ladder.iter().copied() {
        let Some(rung) = probe(radius)? else {
            continue;
        };
        match (rung.margin > 0.0, rung.one_arc) {
            (true, Some(Ok(()))) => {
                let t = tube_transversality(rung.margin, extent, rung.boxes, band)?;
                return Ok((rung, t));
            }
            // A graph whose proof did not run never reads as proved.
            (true, _) => not_one_arc += 1,
            (false, _) => {}
        }
        narrowest = Some(rung);
    }
    let Some(rung) = narrowest else {
        // Rungs were offered and none answered. Structural, and it
        // carries no margin: nothing was ever measured, so there is no
        // honest number to report.
        #[allow(clippy::cast_possible_truncation)]
        return Err(SsiError::TubeProbeSilent {
            rungs: ladder.len() as u32,
        });
    };
    let shortfall = match (rung.margin > 0.0, rung.one_arc) {
        (true, Some(Err(shortfall))) => shortfall,
        (true, _) => Shortfall::Undecided,
        (false, _) => {
            let t = tube_transversality(rung.margin, extent, rung.boxes, band)?;
            return Ok((rung, t));
        }
    };
    let cause = match shortfall {
        Shortfall::Count(solutions) => OneArcRefusal::Count { solutions },
        Shortfall::Unlinked => OneArcRefusal::Unlinked,
        Shortfall::Short => OneArcRefusal::Short,
        // The walk read no margin it could classify, and the funnel
        // records that as it records any unreadable one; an unreadable
        // margin never classifies positive.
        Shortfall::Undecided => {
            match decide_positive("ssi_tube_one_arc", Margin::of(T::from_f64(f64::NAN)), band) {
                Err(cause) => OneArcRefusal::Undecided(cause),
                Ok(()) => {
                    return Err(SsiError::TubeProbeSilent { rungs: not_one_arc });
                }
            }
        }
    };
    Err(SsiError::TubeNotOneArc {
        rungs: not_one_arc,
        cause,
    })
}

/// The certificate the three limbs proved.
fn certificate<T: Real>(
    (on_locus, hull_sup): (T, T),
    (rung, transversality): (Rung, T),
) -> SsiCertificate<T> {
    SsiCertificate {
        samples: CERT_SAMPLES,
        on_locus_max: on_locus,
        hull_sup,
        tube: rung.tube.map(T::from_f64),
        tube_transversality: transversality,
        tube_boxes: rung.boxes,
    }
}

/// Which limbs a certificate asks: all three in order, or limb 3 alone,
/// which refinement asks once of a carrier whose refused margin stopped
/// falling ([`super::refine::refine_by_certificate`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Limbs {
    /// Limbs 1, 2 and 3, refusing at the first that refuses.
    All,
    /// Limb 3 alone: whether the carrier's tube holds one arc.
    Tube,
}

/// What a carrier is certified against. Each arm carries its own
/// lane's inputs, so no lane reads another's; limb 3 proves the same
/// theorem on each, over the region it is cut to.
pub(crate) enum Lane<'a, T: Real> {
    /// At rest, any operand pair: an ℝ³ chain is cut to nothing, a chart
    /// chain to the wall's knot rectangle.
    AtRest {
        /// The first operand.
        a: &'a SsiOperand<'a, T>,
        /// The second operand.
        b: &'a SsiOperand<'a, T>,
        /// The second operand's pcurve, where it is a NURBS wall.
        pcurve_b: Option<&'a NurbsCurve2<T>>,
    },
    /// A plane × NURBS search's branch, its chain cut to the wall's knot
    /// rectangle, which the search banks.
    Chart {
        /// The plane.
        plane: &'a Surface<T>,
        /// The wall, its chart speeds minted.
        wall: super::ChartedNurbs<'a, T>,
        /// The wall's pcurve.
        pcurve: &'a NurbsCurve2<T>,
    },
    /// An ℝ³ search's branch, its chain cut to the box the search is
    /// confined to and banks the tube over.
    Spatial {
        /// The two analytic surfaces.
        pair: (&'a Surface<T>, &'a Surface<T>),
        /// The search's box.
        slab: Box3,
    },
}

/// Where limb 1 or 2 refused a carrier: the parameter interval `[lo,
/// hi]`, a point for a limb-1 sample.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RefusedSpan {
    /// The interval's start.
    pub(crate) lo: f64,
    /// The interval's end.
    pub(crate) hi: f64,
}

/// A certificate's refusal, with where on the carrier limbs 1 and 2
/// refused it.
#[derive(Debug)]
pub(crate) struct Located {
    /// The refusal, as [`certify_branch`] reports it.
    pub(crate) error: SsiError,
    /// Where it lies and what its limb read, for a refusal of limb 1 or
    /// 2 on a margin that is a number; `None` for every other refusal,
    /// which no density of samples answers.
    pub(crate) at: Option<Box<Spans>>,
}

/// A located refusal: its limb, what the limb read, and the parameter
/// intervals whose residual did not clear the band's zero, a limb-1
/// sample as a point interval.
#[derive(Debug)]
pub(crate) struct Spans {
    /// The limb that refused.
    pub(crate) limb: SsiLimb,
    /// What it read.
    pub(crate) margin: super::RoundMargin,
    /// The carrier's parameter intervals the refusal lies in.
    pub(crate) spans: Vec<RefusedSpan>,
}

/// A refusal the certificate does not locate.
impl From<SsiError> for Located {
    fn from(error: SsiError) -> Self {
        Self { error, at: None }
    }
}

/// [`certify_branch`] at `f64`, locating a limb-1 or limb-2 refusal on
/// the carrier. A limb-3 refusal is not located: no density of samples
/// answers it.
///
/// # Errors
///
/// As [`certify_branch`], with the refused intervals.
pub(crate) fn certify_located(
    carrier: &NurbsCurve3<f64>,
    lane: Lane<'_, f64>,
    extent: f64,
    band: Band,
    limbs: Limbs,
) -> Result<SsiCertificate<f64>, Located> {
    let mut spans = Vec::new();
    certify_branch(carrier, lane, extent, band, limbs, &mut spans).map_err(|error| {
        let at = match super::refine::limb_reading(&error) {
            Some((limb, margin)) if !spans.is_empty() => Some(Box::new(Spans {
                limb,
                margin,
                spans,
            })),
            _ => None,
        };
        Located { error, at }
    })
}

/// Certify a fitted rung-3 carrier on its [`Lane`] — all three limbs, in
/// order, refusing typed at the first failure.
///
/// # Errors
///
/// [`SsiError::CertificateLimb`] naming the limb,
/// [`SsiError::TubeStraddles`] for the sliver case,
/// [`SsiError::TubeNotOneArc`] where a search's tube was a graph but not
/// proved one arc, [`SsiError::ChartSpeed`] when the chart is constant
/// across the locus over a tube window,
/// [`SsiError::FootPointInconclusive`] when a NURBS foot will not
/// converge, [`SsiError::CertificateEscalated`] naming the limb whose trilean
/// escalated.
///
/// `extent` is the caller's named feature extent, which levers limb 3's
/// transversality clearance and sets the tube ladder's widest rung.
///
/// The tolerance is `band`'s and only `band`'s. A linear band's
/// `zero()` **is** the run's ε, and every threshold this function
/// applies — the three limbs' trileans and the tube ladder's floor —
/// reads it from there, so there is no second number a caller could
/// certify at.
///
/// A limb-1 or limb-2 refusal is located on the carrier in `at`
/// ([`RefusedSpan`]), for [`super::refine::refine_by_certificate`].
pub(crate) fn certify_branch<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    lane: Lane<'_, T>,
    extent: T,
    band: Band,
    limbs: Limbs,
    at: &mut Vec<RefusedSpan>,
) -> Result<SsiCertificate<T>, SsiError> {
    // The pair the first two limbs read, the pcurve beside the second.
    let (first, second);
    let (a, b, pcurve_b) = match lane {
        Lane::AtRest { a, b, pcurve_b } => (a, b, pcurve_b),
        Lane::Chart {
            plane,
            wall,
            pcurve,
        } => {
            (first, second) = (SsiOperand::Analytic(plane), SsiOperand::Nurbs(wall));
            (&first, &second, Some(pcurve))
        }
        Lane::Spatial { pair, .. } => {
            (first, second) = (SsiOperand::Analytic(pair.0), SsiOperand::Analytic(pair.1));
            (&first, &second, None)
        }
    };
    let mut on_locus = T::zero();
    let mut hull_sup = T::zero();
    let asked: &[_] = match limbs {
        Limbs::All => &[(a, None), (b, pcurve_b)],
        Limbs::Tube => &[],
    };
    for &(op, pc) in asked {
        let (l1, l2) = match op {
            SsiOperand::Analytic(s) => analytic_limbs(carrier, s, band, at)?,
            SsiOperand::Nurbs(s) => {
                let Some(p) = pc else {
                    return Err(SsiError::UnsupportedCertificate {
                        what: NURBS_LIMBS_NEED_PCURVE,
                    });
                };
                nurbs_limbs(carrier, p, s.surface(), band, at)?
            }
        };
        on_locus = on_locus.max(l1);
        hull_sup = hull_sup.max(l2);
    }
    // ---- limb 3: one theorem at every door, over the region the
    // search is cut to (an ℝ³ search's slab; the wall's knot rectangle,
    // which every chart enclosure is clipped to).
    let slab = match lane {
        Lane::Spatial { slab, .. } => Some(slab),
        Lane::AtRest { .. } | Lane::Chart { .. } => None,
    };
    // The carrier's two ends, as the carrier's scalar evaluates them: on
    // the f64 lane a point, not an enclosure of the exact end, which the
    // end checks then read within ε of
    // (`work/ssiarith/limb3-carrier-ends-read-at-f64-points.md`).
    let (t0, t1) = carrier.domain();
    let ends = [t0, t1].map(|t| {
        let p = carrier.eval(T::from_f64(t));
        Box3 {
            x: Interval::from_certified(p.x),
            y: Interval::from_certified(p.y),
            z: Interval::from_certified(p.z),
        }
    });
    // The extent levers the clearance; its upper end is the ladder's
    // widest rung.
    let widest = Bounds::hi(extent);
    let three = match (a, b) {
        (SsiOperand::Analytic(s1), SsiOperand::Analytic(s2)) => {
            let chain = box_chain(carrier);
            limb_three((widest, extent), band, |radius| {
                Ok(probe_tube_analytic(
                    &chain,
                    s1,
                    s2,
                    radius,
                    (slab, ends, band.zero()),
                ))
            })?
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
            limb_three((widest, extent), band, |radius| {
                // The pad per axis: the rung ÷ the operand's minted chart
                // speed along that axis. The padded windows are the
                // proved region and the certificate records them; the
                // rung is the ladder's label, not a metre bound on that
                // region (a window's corner can sit farther than the rung
                // from the carrier).
                let pad = n.speeds().pad(radius);
                probe_tube_chart(
                    p,
                    n.surface(),
                    (origin, normal),
                    (radius, pad),
                    (ends, band),
                )
            })?
        }
        (SsiOperand::Nurbs(_), SsiOperand::Nurbs(_)) => {
            return Err(SsiError::UnsupportedCertificate {
                what: "NURBS × NURBS routes to the general rung but its uniqueness \
                       tube is not implemented in this build (arms retire one at \
                       a time, each with its proof)",
            });
        }
    };
    Ok(certificate((on_locus, hull_sup), three))
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
    match band_verdict("ssi_tube_transversality", transversality, band) {
        None => Ok(transversality.value()),
        Some(BandVerdict::Refused(verdict)) => Err(SsiError::TubeStraddles { verdict, boxes }),
        Some(BandVerdict::Undecided(cause)) => Err(SsiError::CertificateEscalated {
            limb: SsiLimb::Tube,
            cause,
        }),
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
        use crate::ssi::enclose::Box3;
        use crate::ssi::{SsiError, TubeDegeneracy};

        fn iv(x: f64) -> Interval {
            Interval::from_f64(x)
        }

        /// [`u_line`]'s carrier ends on [`unit_patch`], and a band.
        fn at_ends() -> ([Box3; 2], geom_core::Band) {
            let at = |u: f64| Box3 {
                x: Interval::from_bounds(u, u),
                y: Interval::from_bounds(0.5, 0.5),
                z: Interval::from_bounds(0.0, 0.0),
            };
            (
                [at(0.1), at(0.9)],
                geom_core::Band::new(1e-9, 1e-8).expect("a valid band"),
            )
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
                (0.0, (0.01, 0.01)),
                at_ends(),
            )
            .expect("the unit patch's chart is not degenerate")
            .expect("the probe must reach a verdict");
            assert!(
                probe.margin > 0.0,
                "certified normal gave margin {}",
                probe.margin
            );
            assert!(probe.boxes > 0, "no span was probed: the row is vacuous");
            assert_eq!(
                probe.one_arc,
                Some(Ok(())),
                "the window holds the line and nothing else"
            );
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
                (0.0, (0.01, 0.01)),
                at_ends(),
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
        /// end mints no `ParamRange`, where a window landed on the first
        /// span would certify the derivative boxes of an arbitrary cell.
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
                (0.0, (0.01, 0.01)),
                at_ends(),
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
                (0.0, (0.01, 0.01)),
                at_ends(),
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
            let verdict = probe_tube_chart(
                &u_line(),
                &ridge,
                (on_line(), normal),
                (0.0, (0.01, 0.01)),
                at_ends(),
            );
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
                (0.0, (0.01, 0.01)),
                at_ends(),
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

        use super::{Box3, Interval, SsiTube, chart_tube_windows, probe_tube_chart};
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
            let geom::Curve3::Nurbs(carrier) = &b.carrier else {
                panic!("branch {i}: a fitted carrier is a NURBS curve");
            };
            let (t0, t1) = carrier.domain();
            let ends = [t0, t1].map(|t| {
                let p = carrier.eval(t);
                let i = |c: f64| Interval::from_bounds(c, c);
                Box3 {
                    x: i(p.x),
                    y: i(p.y),
                    z: i(p.z),
                }
            });
            let ends = (ends, band);
            let probe = probe_tube_chart(pc, &wall, (origin, n), (rung, (pad_u, pad_v)), ends)
                .unwrap()
                .expect("the recorded pad probes");
            assert_eq!(
                probe.one_arc,
                Some(Ok(())),
                "branch {i}: the recorded pad's chain is not one arc"
            );
            let (margin, boxes) = (probe.margin, probe.boxes);
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
            let at_fold =
                probe_tube_chart(pc, &wall, (origin, n), (rung, (folded, folded)), ends).unwrap();
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

    /// A ladder over `extent` 1 at ε = 1e-9 whose first (widest) rung
    /// reads `first` and every narrower one `rest`.
    #[allow(clippy::unwrap_used)]
    fn ladder_of(
        first: (f64, Option<Result<(), super::Shortfall>>),
        rest: (f64, Option<Result<(), super::Shortfall>>),
    ) -> Result<(super::Rung, f64), crate::ssi::SsiError> {
        use super::{Rung, SsiTube, limb_three};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let mut widest = true;
        limb_three((1.0, 1.0), band, |radius| {
            let (margin, one_arc) = if widest { first } else { rest };
            widest = false;
            Ok(Some(Rung {
                tube: SsiTube::Spatial { radius },
                margin,
                boxes: 4,
                one_arc,
            }))
        })
    }

    /// **A graph whose one-arc proof did not run is not proved.** Every
    /// rung zero-free with no proof recorded refuses as undecided, never
    /// certifies. Red under the ladder taking a zero-free rung with no
    /// proof as proved.
    #[test]
    fn a_graph_with_no_proof_is_not_proved_one_arc() {
        let got = ladder_of((0.5, None), (0.5, None));
        assert!(
            matches!(
                got,
                Err(crate::ssi::SsiError::TubeNotOneArc {
                    cause: crate::ssi::OneArcRefusal::Undecided(_),
                    ..
                })
            ),
            "{got:?}"
        );
    }

    /// **The narrowest rung speaks.** A wide rung that is a graph but
    /// not one arc, and narrower rungs that all straddle: the straddle is
    /// the more specific reading of the same carrier, and its band
    /// verdict is the refusal.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn a_narrower_straddle_speaks_over_a_wider_rung_not_one_arc() {
        use super::Shortfall;
        let got = ladder_of((0.5, Some(Err(Shortfall::Count(4)))), (0.0, None));
        assert!(
            matches!(got, Err(crate::ssi::SsiError::TubeStraddles { .. })),
            "{got:?}"
        );
    }

    /// The mirror: a wide rung that straddles, and narrower ones that are
    /// graphs but not one arc. The refusal is `TubeNotOneArc`, counting
    /// those rungs, with the narrowest's cause.
    #[test]
    #[allow(clippy::unwrap_used)]
    fn a_narrower_rung_not_one_arc_speaks_over_a_wider_straddle() {
        use super::{OneArcRefusal, Shortfall, tube_ladder};
        let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
        let rungs = tube_ladder(1.0, band).count() as u32;
        let got = ladder_of((0.0, None), (0.5, Some(Err(Shortfall::Unlinked))));
        assert!(
            matches!(
                got,
                Err(crate::ssi::SsiError::TubeNotOneArc {
                    rungs: r,
                    cause: OneArcRefusal::Unlinked,
                }) if r == rungs - 1
            ),
            "{got:?}"
        );
    }

    /// A walk that resolved nothing escalates through the funnel, with
    /// the unreadable margin it read.
    #[test]
    #[allow(clippy::unwrap_used, clippy::panic)]
    fn an_undecided_walk_escalates_through_the_funnel() {
        use super::{OneArcRefusal, Shortfall};
        let got = ladder_of(
            (0.5, Some(Err(Shortfall::Undecided))),
            (0.5, Some(Err(Shortfall::Undecided))),
        );
        let Err(crate::ssi::SsiError::TubeNotOneArc {
            cause: OneArcRefusal::Undecided(cause),
            ..
        }) = got
        else {
            panic!("{got:?}");
        };
        assert!(cause.margin.is_invalid(), "{cause:?}");
    }

    /// **A limb-3 refusal is not located, so refinement leaves it
    /// unwrapped.** The fold at slope `c = 800ε`, gap height β = ½ε:
    /// the straight carrier across its gap passes limbs 1 and 2, and on
    /// the chart search's lane its tube is a graph but not one arc. The
    /// refusal is `TubeNotOneArc` with no located interval, which
    /// [`super::super::refine::refine_by_certificate`] returns as it is
    /// rather than as `RefinementExhausted`.
    #[test]
    #[allow(clippy::unwrap_used, clippy::panic)]
    fn a_limb_three_refusal_on_the_chart_lane_is_not_located() {
        use geom::{NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
        use geom_core::spline::KnotVector;
        use geom_core::{Band, Point2, Point3, Vec3};

        use super::{Lane, certify_located};
        use crate::ssi::{ChartedNurbs, SsiError};

        let beta = 0.5e-9;
        let c = 1600.0 * beta;
        let a = 0.28 * c * c / beta;
        let w = beta / c;
        let l = 1.2 * w;
        let (x0, x1) = (-1.5 * w, 1.8 * w);
        let g = |x: f64| c * x + a * x * x;
        let gb = [g(x0), g(x0) + 0.5 * (x1 - x0) * (c + 2.0 * a * x0), g(x1)];
        let k2 = || KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let (xs, ys) = ([x0, 0.5 * (x0 + x1), x1], [0.0, 0.5 * l, l]);
        let control = (0..9)
            .map(|i| {
                Point3::new(
                    xs[i / 3],
                    ys[i % 3],
                    gb[i / 3] + [0.0, 2.0 * beta, 0.0][i % 3],
                )
            })
            .collect();
        let wall = NurbsSurface::new(k2(), k2(), control, vec![1.0; 9]).unwrap();
        let k1 = || KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = NurbsCurve3::new(
            k1(),
            vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, l, 0.0)],
            vec![1.0, 1.0],
        )
        .unwrap();
        let u0 = -x0 / (x1 - x0);
        let pcurve = NurbsCurve2::new(
            k1(),
            vec![Point2::new(u0, 0.0), Point2::new(u0, 1.0)],
            vec![1.0, 1.0],
        )
        .unwrap();
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let lane = Lane::Chart {
            plane: &plane,
            wall: ChartedNurbs::mint(&wall).unwrap(),
            pcurve: &pcurve,
        };
        let band = Band::new(1e-9, 1e-8).unwrap();
        let Err(refused) = certify_located(&carrier, lane, 1.0, band, super::Limbs::All) else {
            panic!("the carrier across the gap certified on the search's lane");
        };
        assert!(
            matches!(refused.error, SsiError::TubeNotOneArc { .. }),
            "{:?}",
            refused.error
        );
        assert!(refused.at.is_none(), "limb 3 was located: {:?}", refused.at);
    }
}

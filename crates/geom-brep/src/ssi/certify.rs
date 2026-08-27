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
//! (C9's ring, no evaluation, no interval feature):
//!
//! - **analytic**: `geom_core::spline::compose` composes the surface's
//!   polynomial implicit form with the carrier and returns a certified
//!   per-span hull of the composite. Its units are the composite's, so
//!   this module converts to meters **exactly** — `÷ 2R` for the sphere
//!   and cylinder, identity for the plane. Cone and torus are *not*
//!   converted: their meters forms carry a root the ring lacks, so an
//!   arm wanting them must land that conversion first
//!   ([`super::enclose`] carries the same boundary, same reason).
//! - **NURBS**: `sup_t |S(P(t)) − C(t)|` bounded by the
//!   **tensor-product Bernstein composition**
//!   (`geom_core::spline::compose::tensor`, M5 PR 7b): the difference
//!   is enclosed as ONE composite whose ring coefficients are hulled
//!   per span, so the cancellation that is the whole content of
//!   `S(P(t)) = C(t)` survives into the bound. Every coefficient is a
//!   ring enclosure; nothing is sampled.
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
//! chain of boxes covering the carrier with certified radius ρ, this
//! module proves what the enclosures actually support — stated exactly,
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
//! Note what that does *not* say. It is a convexity/mean-value
//! argument over the enclosure, not a bare appeal to the implicit
//! function theorem (which is local and would not, by itself, cover the
//! whole box). And it says nothing about a **disjoint component
//! threading the padded chain at `e`-levels the carrier never
//! occupies**: the slice argument is silent there. What excludes that
//! is the other obligation entirely — [`super::exhaust`]'s accounting
//! pass, which requires every cell of the bounded domain to be
//! excluded by enclosure or *contained* in a tube, and refuses typed at
//! the floor otherwise. Uniqueness in this module and completeness
//! there are two theorems, and neither is doing the other's work.
//!
//! An enclosure that **straddles** zero at the chain's box size is not
//! a resolution failure to retry: two branches passing within the band
//! of each other is a genuine sliver of the operand pair, and F6's
//! ladder says escalate. That is `ssi_tube_transversality` landing in
//! `Sign::Zero`, and it refuses toward C7.
//!
//! # The witness is unchanged
//!
//! `witness = carrier(mid)` (`WitnessMidpoint`), minted by the
//! constructing op from the cache this schedule sees. S2 stays
//! discharged; nothing about the witness contract moves at rung 3.

use geom::{NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::spline::compose::{self, CurveRingData, ImplicitSurface, tensor};
use geom_core::spline::hull;
use geom_core::{
    Band, Bounds, CertifiedEnclosure, Decide, Margin, Point3, Real, RingInterval, Sign, Vec3,
};

use crate::certify::CERT_SAMPLES;
use crate::dihedral::decide;

use super::enclose::{Box3, NurbsBoxes, graph_margin};
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
fn tube_ladder(extent: f64, band: Band) -> impl Iterator<Item = f64> {
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

/// The three-limb certificate of a rung-3 fitted carrier. Every field
/// is a *bound*, in meters, that the corresponding limb proved.
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
    /// meters. This is the number that certifies. **Ring-derived**: the
    /// C9 ring produces an `f64` upper bound (that is what a hull bound
    /// IS), lifted here so consumers band one scalar; at the interval
    /// scalar it is a thin enclosure of that bound, and the widening of
    /// a lifted operand has already been paid inside the ring.
    pub hull_sup: T,
    /// Limb 3: the tube's radius, in meters. **Ladder structure**
    /// (C6's `f64` selection lane), lifted for uniformity — no decision
    /// reads it.
    pub tube_radius: T,
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
}

/// Refine the carrier so the hull limbs have small spans to work with
/// (knot refinement is exact in ℝ; the curve is unchanged).
fn refined<T: Real>(curve: &NurbsCurve3<T>) -> NurbsCurve3<T> {
    let (lo, hi) = curve.domain();
    let kv = curve.knots();
    // Already fine enough: refining a carrier that the marcher's step
    // rule already gave hundreds of spans buys nothing and costs an
    // O(n²) knot insertion per call.
    if kv.control_count() >= SSI_CERT_SPANS + kv.degree() {
        return curve.clone();
    }
    let mut add = Vec::new();
    for i in 1..SSI_CERT_SPANS {
        #[allow(clippy::cast_precision_loss)]
        let t = lo + (hi - lo) * (i as f64 / SSI_CERT_SPANS as f64);
        // Skip parameters already present as knots (refinement would
        // raise multiplicity, which is not what this is for).
        if kv.multiplicity_of(t).is_none() {
            add.push(t);
        }
    }
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
/// meters form carries a root the ring cannot take (cone, torus), NURBS
/// (no implicit form), and — since M6-2 — an operand whose structural
/// parameters are not exact at the caller's scalar (see [`exact`]).
fn composite_form<T: Bounds>(s: &Surface<T>) -> Result<(ImplicitSurface, f64), &'static str> {
    const WIDENED: &str = "the analytic operand's structural parameters are not exact at this \
                           scalar — the ring composite's implicit form is f64 structure, and a \
                           widened operand is a family of surfaces, not the body's surface \
                           — refused rather than represented by a midpoint";
    match *s {
        Surface::Plane { origin, normal, .. } => {
            let (Some(point), Some(normal)) = (
                exact3([origin.x, origin.y, origin.z]),
                exact3([normal.x, normal.y, normal.z]),
            ) else {
                return Err(WIDENED);
            };
            Ok((
                ImplicitSurface::Plane { point, normal },
                // n·(P − p₀) is already meters for a unit normal.
                1.0,
            ))
        }
        Surface::Sphere { center, radius, .. } => {
            let (Some(center), Some(radius)) =
                (exact3([center.x, center.y, center.z]), exact(radius))
            else {
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
                exact3([origin.x, origin.y, origin.z]),
                exact3([axis.x, axis.y, axis.z]),
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
             (cone/torus need a certified root the exact-arithmetic ring lacks; \
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
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(CERT_SAMPLES - 1));
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
            Err(diag) => return Err(SsiError::Escalated(diag)),
        }
    }

    // ---- limb 2: the certified hull bound ----
    let (form, to_meters) =
        composite_form(surface).map_err(|what| SsiError::UnsupportedCertificate { what })?;
    let fine = refined(carrier);
    let coords = fine.ring_coords();
    let data = CurveRingData::new(fine.knots(), fine.weights(), &coords).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the fitted carrier's ring data is malformed",
        }
    })?;
    let composite = compose::implicit_composite(&data, &form).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the implicit composite refused the fitted carrier",
        }
    })?;
    // The ring answers with an `f64` upper bound — that is what a hull
    // bound is — and it is lifted here so the limb is banded at the
    // caller's scalar like every other residual (field docs).
    let sup = T::from_f64(composite.sup_bound() * to_meters);
    match decide("ssi_hull_sup", Margin::of(sup), band) {
        Ok(Sign::Zero) => Ok((worst, sup)),
        Ok(Sign::Positive | Sign::Negative) => Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value: sup.hi(),
        }),
        Err(diag) => Err(SsiError::Escalated(diag)),
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
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(CERT_SAMPLES - 1));
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
            Err(diag) => return Err(SsiError::Escalated(diag)),
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
                Err(diag) => return Err(SsiError::Escalated(diag)),
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
    let coords = carrier.ring_coords();
    let cdata = CurveRingData::new(carrier.knots(), carrier.weights(), &coords).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the fitted carrier's ring data is malformed",
        }
    })?;
    let pcoords = pcurve.ring_coords();
    let pdata = CurveRingData::new(pcurve.knots(), pcurve.weights(), &pcoords).map_err(|_| {
        SsiError::UnsupportedCertificate {
            what: "the traced pcurve's ring data is malformed",
        }
    })?;
    let scoords = surface.ring_coords();
    let sdata = tensor::SurfaceRingData::new(
        surface.knots_u(),
        surface.knots_v(),
        surface.weights(),
        &scoords,
    )
    .map_err(|_| SsiError::UnsupportedCertificate {
        what: "the NURBS operand's ring data is malformed",
    })?;
    let (t0c, t1c) = carrier.domain();
    #[allow(clippy::cast_precision_loss)]
    let extra: Vec<f64> = (1..SSI_CERT_SPANS)
        .map(|i| t0c + (t1c - t0c) * (i as f64 / SSI_CERT_SPANS as f64))
        .collect();
    let sup = tensor::surface_curve_residual(&sdata, &pdata, &cdata, &extra)
        .map_err(|_| SsiError::UnsupportedCertificate {
            what: "the tensor composite refused the carrier/pcurve pair (mismatched \
                   channel counts or knot domains — the shared-parameter identity is the entry \
                   requirement)",
        })?
        .sup_bound();
    // Unlike the analytic arm, the NURBS arm needs NO exactness gate:
    // every coefficient of every operand entered the ring through its
    // own bracket (`ring_coords`), so a widened control net widens the
    // composite and the bound stays honest.
    let sup = T::from_f64(sup);
    match decide("ssi_hull_sup_chart", Margin::of(sup), band) {
        Ok(Sign::Zero) => Ok((worst, sup)),
        Ok(Sign::Positive | Sign::Negative) => Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value: sup.hi(),
        }),
        Err(diag) => Err(SsiError::Escalated(diag)),
    }
}

/// The box chain covering a carrier: one padded box per span of the
/// refined curve, from the span's control hull (exact containment for a
/// non-rational curve — the convex-hull property).
fn box_chain<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
) -> Vec<(Box3, Vec3<T>)> {
    let fine = refined(carrier);
    let coords = fine.ring_coords();
    let kv = fine.knots();
    let mut out = Vec::new();
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and span validation are one step.
        let Some(span) = kv.span(index) else { continue };
        let (a, b) = (kv.knots()[index], kv.knots()[index + 1]);
        let hx = hull::span_hull(kv, &coords[0], span);
        let hy = hull::span_hull(kv, &coords[1], span);
        let hz = hull::span_hull(kv, &coords[2], span);
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
        // margin's ring enclosure poisons with it, and
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
/// `sin θ` scale) and the box count; `None` when the chain is broken or
/// an enclosure poisoned, which is a definite structural refusal.
fn probe_tube_analytic<T: Decide + Bounds + CertifiedEnclosure>(
    chain: &[(Box3, Vec3<T>)],
    s1: &Surface<T>,
    s2: &Surface<T>,
    radius: f64,
) -> Option<(f64, u32)> {
    if chain.is_empty() {
        return None;
    }
    let mut worst = f64::INFINITY;
    let mut prev: Option<Box3> = None;
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
        let m = zero_free_lower_bound(graph_margin(s1, s2, bx, *e));
        if m < worst {
            worst = m;
        }
    }
    Some((worst, chain.len() as u32))
}

/// Limb 3's enclosure probe for the **plane × NURBS** arm: the same
/// criterion in the NURBS chart, where the locus is
/// `φ(u,v) = n·(S(u,v) − p₀) = 0` and `∇φ = (n·S_u, n·S_v)`. A
/// zero-free enclosure of the component of `∇φ` transverse to the
/// pcurve's own tangent proves the same thing the ℝ³ form proves: a
/// graph, hence one arc.
fn probe_tube_chart<T: Decide + Bounds + CertifiedEnclosure>(
    pcurve: &NurbsCurve2<T>,
    surface: &NurbsSurface<T>,
    normal: Vec3<T>,
    radius_uv: (f64, f64),
) -> Option<(f64, u32)> {
    let kv = pcurve.knots();
    let boxes = NurbsBoxes::new(surface);
    let coords = pcurve.ring_coords();
    let mut worst = f64::INFINITY;
    let mut count = 0u32;
    for index in kv.first_span()..=kv.last_span() {
        // Emptiness check and span validation are one step.
        let Some(span) = kv.span(index) else { continue };
        let (a, b) = (kv.knots()[index], kv.knots()[index + 1]);
        let m = 0.5 * (a + b);
        let hu = hull::span_hull(kv, &coords[0], span);
        let hv = hull::span_hull(kv, &coords[1], span);
        let (u0, u1) = (hu.lo() - radius_uv.0, hu.hi() + radius_uv.0);
        let (v0, v1) = (hv.lo() - radius_uv.1, hv.hi() + radius_uv.1);
        let du = boxes.deriv_box(u0, u1, v0, v1, true);
        let dv = boxes.deriv_box(u0, u1, v0, v1, false);
        // The plane equation is what the whole limb certifies, so the
        // normal crosses through the CERTIFIED door: a component whose
        // computation left its domain becomes poison and the
        // transversality margin collapses to zero, rather than a
        // zero-free enclosure of an equation nobody evaluated.
        let n = [
            RingInterval::from_certified(normal.x),
            RingInterval::from_certified(normal.y),
            RingInterval::from_certified(normal.z),
        ];
        let phi_u = n[0] * du.x + n[1] * du.y + n[2] * du.z;
        let phi_v = n[0] * dv.x + n[1] * dv.y + n[2] * dv.z;
        // The transverse chart direction is a DIRECTION — structure —
        // so it is selected through the bracket, exactly as the tube
        // ladder's radius is. `powi(2)`, never `t.x * t.x`.
        let t = pcurve.deriv(T::from_f64(m));
        let tn = (t.x.powi(2) + t.y.powi(2)).sqrt().hi();
        if tn.is_nan() || tn <= 0.0 {
            return None;
        }
        let (tx, ty) = (t.x.hi(), t.y.hi());
        // e⊥ = (−t.y, t.x)/‖t‖; the transverse derivative of φ.
        let ex = RingInterval::point(-ty / tn);
        let ey = RingInterval::point(tx / tn);
        // ∇φ·e⊥ is metres of plane-distance per CHART unit, so it is
        // not yet a margin: multiplying it by a lever arm in metres
        // would give metres² per chart unit (D4 ¶1 forbids exactly
        // that). Dividing by the chart's own stretch along e⊥ —
        // ‖S_u·ex + S_v·ey‖, metres per chart unit — cancels the chart
        // units and leaves the dimensionless sine-like quantity the ℝ³
        // lane's `(∇f₁×∇f₂)·e` already is. An UPPER bound on the
        // stretch is used, which can only shrink the margin: the safe
        // direction.
        let vt = Box3 {
            x: du.x * ex + dv.x * ey,
            y: du.y * ex + dv.y * ey,
            z: du.z * ex + dv.z * ey,
        };
        let stretch =
            (vt.x.mag() * vt.x.mag() + vt.y.mag() * vt.y.mag() + vt.z.mag() * vt.z.mag()).sqrt();
        if stretch.is_nan() || stretch <= 0.0 {
            return None;
        }
        let margin = zero_free_lower_bound(phi_u * ex + phi_v * ey) / stretch;
        if margin < worst {
            worst = margin;
        }
        count += 1;
    }
    if count == 0 {
        None
    } else {
        Some((worst, count))
    }
}

/// The certified distance of an enclosure from zero: `0` when it
/// straddles (or is poison), which is exactly what makes the trilean
/// land in the sliver band.
/// (The mignitude. `offset_meters::mig` is the same arithmetic read
/// as a coefficient-hull assembly term rather than a decision; noted
/// at both sites.)
fn zero_free_lower_bound(i: RingInterval) -> f64 {
    if i.is_poison() {
        return 0.0;
    }
    if i.lo() > 0.0 {
        i.lo()
    } else if i.hi() < 0.0 {
        -i.hi()
    } else {
        0.0
    }
}

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

/// Certify a fitted rung-3 carrier against its operand pair — all three
/// limbs, in order, refusing typed at the first failure.
///
/// # Errors
///
/// [`SsiError::CertificateLimb`] naming the limb,
/// [`SsiError::TubeStraddles`] for the sliver case,
/// [`SsiError::FootPointInconclusive`] when a NURBS foot will not
/// converge, [`SsiError::Escalated`] for any in-band trilean.
///
/// `scale` carries the two lengths the certificate is stated over: the
/// folded curvature/extent lever arm the transversality margin is
/// levered by, and the caller's named feature extent, which sets the
/// tube ladder's widest rung.
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
                        what: "a NURBS operand's limbs need the traced pcurve \
                               (the ℝ⁴ trace supplies it; the ℝ³ trace has none)",
                    });
                };
                nurbs_limbs(carrier, p, s, band)?
            }
        };
        on_locus = on_locus.max(l1);
        hull_sup = hull_sup.max(l2);
    }
    // ---- limb 3: pick the widest certifiable tube, then decide ONCE.
    let mut chosen: Option<(f64, f64, u32)> = None; // (radius, margin, boxes)
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
                probe_tube_analytic(&chain, s1, s2, radius)
            }
            (SsiOperand::Analytic(plane), SsiOperand::Nurbs(n))
            | (SsiOperand::Nurbs(n), SsiOperand::Analytic(plane)) => {
                let Surface::Plane { normal, .. } = **plane else {
                    return Err(SsiError::UnsupportedCertificate {
                        what: "the chart uniqueness tube is written for a PLANE against \
                               a NURBS surface; another analytic kind needs its own \
                               chart form",
                    });
                };
                let Some(p) = pcurve_b else {
                    return Err(SsiError::UnsupportedCertificate {
                        what: "the chart uniqueness tube needs the traced pcurve",
                    });
                };
                // The radius in chart units: metres ÷ a certified chart
                // speed, taken over the whole domain so the pad is
                // conservative in the safe direction (a wider uv pad
                // gives a wider enclosure and a HARDER test).
                let (ud, vd) = (n.knots_u().domain(), n.knots_v().domain());
                let nb = NurbsBoxes::new(n);
                let speed = |bx: Box3| {
                    let m = (bx.x.mag() * bx.x.mag()
                        + bx.y.mag() * bx.y.mag()
                        + bx.z.mag() * bx.z.mag())
                    .sqrt();
                    if m > 0.0 { m } else { f64::NAN }
                };
                let su = speed(nb.deriv_box(ud.0, ud.1, vd.0, vd.1, true));
                let sv = speed(nb.deriv_box(ud.0, ud.1, vd.0, vd.1, false));
                probe_tube_chart(p, n, normal, (radius / su, radius / sv))
            }
            (SsiOperand::Nurbs(_), SsiOperand::Nurbs(_)) => {
                return Err(SsiError::UnsupportedCertificate {
                    what: "NURBS × NURBS routes to the general rung but its uniqueness \
                           tube is not implemented in this build (arms retire one at \
                           a time, each with its proof)",
                });
            }
        };
        let Some((margin, boxes)) = probe else {
            continue;
        };
        // Structure selection (C6's f64 lane): the widest rung whose
        // enclosure is zero-free wins; the LAST rung is kept even when
        // it fails, so the refusal below carries a real number rather
        // than a vacuum.
        if margin > 0.0 {
            chosen = Some((radius, margin, boxes));
            break;
        }
        chosen = Some((radius, margin, boxes));
    }
    let Some((radius, margin, boxes)) = chosen else {
        // Rungs were offered and none answered. Structural, and it
        // carries no margin: nothing was ever measured, so there is no
        // honest number to report.
        #[allow(clippy::cast_possible_truncation)]
        return Err(SsiError::TubeProbeSilent {
            rungs: ladder.len() as u32,
        });
    };
    // The margin is the ring's zero-free lower bound (`f64`, C9); the
    // lever arm is the caller's scalar, so the product — the number the
    // trilean classifies — is scalar-typed.
    let transversality = Margin::levered(T::from_f64(margin), arm);
    match decide("ssi_tube_transversality", transversality, band) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => {
            return Err(SsiError::TubeStraddles {
                margin: transversality.value().lo(),
                boxes,
            });
        }
        Err(diag) => return Err(SsiError::Escalated(diag)),
    }
    Ok(SsiCertificate {
        samples: CERT_SAMPLES,
        on_locus_max: on_locus,
        hull_sup,
        tube_radius: T::from_f64(radius),
        tube_transversality: transversality.value(),
        tube_boxes: boxes,
    })
}

/// The witness of a rung-3 carrier: `carrier(mid)`, unchanged from M2
/// (`WitnessMidpoint`; S2 stays discharged).
pub(crate) fn witness<T: Decide + Bounds + CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
) -> Point3<T> {
    let (t0, t1) = carrier.domain();
    carrier.eval(T::from_f64(0.5 * (t0 + t1)))
}

#[cfg(test)]
mod tests {
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
            let band = Band::new(zero, 10.0 * zero).unwrap();
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
    /// The three components of the plane normal enter the C9 ring through
    /// their own brackets, and at `Interval` a bracket can be sound and
    /// still inadmissible: `sqrt([−1, 4]) + 1` is `[1, 3]` with decoration
    /// `Trv`. `RingInterval` has no decoration channel, so a normal that
    /// cannot certify has to be refused at the crossing — otherwise the
    /// transversality margin is a positive number computed from a plane
    /// equation that was never evaluated where it was asked for.
    #[cfg(feature = "interval")]
    #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    mod normal_crossing_tests {
        use geom::NurbsCurve2;
        use geom::NurbsSurface;
        use geom_core::spline::KnotVector;
        use geom_core::{Interval, Point2, Point3, Real, Vec3};

        use super::super::probe_tube_chart;

        fn iv(x: f64) -> Interval {
            Interval::from_f64(x)
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
            let (margin, boxes) = probe_tube_chart(&u_line(), &unit_patch(), normal, (0.01, 0.01))
                .expect("the probe must reach a verdict");
            assert!(margin > 0.0, "certified normal gave margin {margin}");
            assert!(boxes > 0, "no span was probed: the row is vacuous");
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
            let verdict = probe_tube_chart(&u_line(), &unit_patch(), normal, (0.01, 0.01));
            let margin = verdict.map_or(0.0, |(m, _)| m);
            assert_eq!(
                margin, 0.0,
                "a domain-violated plane normal produced transversality \
                 margin {margin} — the crossing read the BRACKET door, so \
                 the chart tube certifies uniqueness from a plane equation \
                 that was clamped out of its own domain"
            );
        }
    }
}

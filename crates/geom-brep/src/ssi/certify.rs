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
//!   `geom_surfaces::projection` — and the projection's own
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
//! - **NURBS**: `sup_t |C(t) − S(P(t))|` over each span of the
//!   (refined) carrier, bounded by
//!   `rad_C + |C(m) − S(P(m))| + rad_S` — the curve's own derivative
//!   hull over the span, the exact residual at the span midpoint, and
//!   the surface's certified derivative box over the parameter window
//!   the pcurve can reach in that span. Every term is a hull; nothing
//!   is sampled.
//!
//!   **This bound is sound but not tight, and the gap is structural.**
//!   `rad_C` and `rad_S` are first-order: each is `‖derivative hull‖ ×
//!   half-span`, so the bound scales like the *span width*, not like
//!   the residual. On the M5 wall fixture the midpoint residual is
//!   ~1e-10 m and the bound is ~1e-2 m — the two variation terms are
//!   each bounding a real motion of the curve across its own span, and
//!   they very nearly cancel (the curve and the surface point move
//!   together, which is the whole content of `S(P(t)) = C(t)`), but
//!   enclosing them separately throws that cancellation away. Reaching
//!   ε would need spans around 1e-5 wide, i.e. tens of thousands of
//!   them.
//!
//!   Capturing the cancellation needs the difference to be a **single
//!   composite** whose control coefficients are hulled — which is
//!   exactly what `compose` does for an analytic surface and exactly
//!   what does not exist for a surface: `geom_core::spline::compose`
//!   is curve-only by design, and `φ∘P` for a tensor-product surface is
//!   a Bernstein *composition*, not a product. That machinery is the
//!   entry requirement for retiring the plane×NURBS arm and is **banked
//!   as M5 PR 7b**. See the C5 table's `(Plane, Nurbs)` note, which
//!   says the same thing where a caller will read it.
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

use geom_core::spline::compose::{self, CurveRingData, ImplicitSurface};
use geom_core::spline::hull;
use geom_core::{Band, Point3, RingInterval, Sign, Vec3};
use geom_curves::{NurbsCurve2, NurbsCurve3};
use geom_surfaces::{NurbsSurface, Surface};

use crate::certify::CERT_SAMPLES;
use crate::dihedral::decide;

use super::enclose::{Box3, NurbsBoxes, graph_margin};
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
fn tube_ladder(extent: f64, eps: f64) -> impl Iterator<Item = f64> {
    let floor = SSI_TUBE_RADIUS * eps;
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
#[derive(Clone, Copy, Debug)]
pub struct SsiCertificate {
    /// The fixed sample count of limb 1 ([`CERT_SAMPLES`]).
    pub samples: u32,
    /// Limb 1: the largest on-locus residual over the schedule, in
    /// meters (the sampled max — it steers, it does not certify).
    pub on_locus_max: f64,
    /// Limb 2: the certified **sup-norm** bound over the whole span, in
    /// meters. This is the number that certifies.
    pub hull_sup: f64,
    /// Limb 3: the tube's radius, in meters.
    pub tube_radius: f64,
    /// Limb 3: the smallest certified transversality margin over the
    /// box chain, in meters — the headroom of the one-arc proof.
    pub tube_transversality: f64,
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
fn refined(curve: &NurbsCurve3<f64>) -> NurbsCurve3<f64> {
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

/// The `compose` implicit form of an analytic surface, and the exact
/// factor converting its composite's units to meters.
///
/// `None` for the kinds whose meters form carries a root the ring
/// cannot take (cone, torus) and for NURBS (no implicit form) — the
/// caller then routes to the NURBS mechanism or refuses, and never
/// invents a conversion.
fn composite_form(s: &Surface<f64>) -> Option<(ImplicitSurface, f64)> {
    match *s {
        Surface::Plane { origin, normal, .. } => Some((
            ImplicitSurface::Plane {
                point: [origin.x, origin.y, origin.z],
                normal: [normal.x, normal.y, normal.z],
            },
            // n·(P − p₀) is already meters for a unit normal.
            1.0,
        )),
        Surface::Sphere { center, radius, .. } => Some((
            ImplicitSurface::Sphere {
                center: [center.x, center.y, center.z],
                radius,
            },
            // |P−c|² − R² = 2R · the linearized meters residual, exactly.
            1.0 / (2.0 * radius),
        )),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => Some((
            ImplicitSurface::Cylinder {
                point: [origin.x, origin.y, origin.z],
                axis: [axis.x, axis.y, axis.z],
                radius,
            },
            // |w|² − R² = 2R · the linearized meters residual, exactly.
            1.0 / (2.0 * radius),
        )),
        Surface::Cone { .. } | Surface::Torus { .. } | Surface::Nurbs(_) => None,
    }
}

/// Limb 1 + limb 2 against one **analytic** operand.
fn analytic_limbs(
    carrier: &NurbsCurve3<f64>,
    surface: &Surface<f64>,
    band: Band,
) -> Result<(f64, f64), SsiError> {
    // ---- limb 1: the fixed schedule ----
    let (t0, t1) = carrier.domain();
    let mut worst = 0.0f64;
    for i in 0..CERT_SAMPLES {
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(CERT_SAMPLES - 1));
        let r = crate::implicit::implicit_residual(surface, carrier.eval(t)).abs();
        if r > worst {
            worst = r;
        }
        match decide("ssi_on_locus", r, band) {
            // Zero is the affirmative: the residual is zero to
            // tolerance (the `dihedral_wedge` convention).
            Ok(Sign::Zero) => {}
            Ok(Sign::Positive | Sign::Negative) => {
                return Err(SsiError::CertificateLimb {
                    limb: SsiLimb::OnLocus,
                    value: r,
                });
            }
            Err(diag) => return Err(SsiError::Escalated(diag)),
        }
    }

    // ---- limb 2: the certified hull bound ----
    let Some((form, to_meters)) = composite_form(surface) else {
        return Err(SsiError::UnsupportedCertificate {
            what: "no ring-computable meters composite for this surface kind \
                   (cone/torus need a certified root the C9 ring lacks)",
        });
    };
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
    let sup = composite.sup_bound() * to_meters;
    match decide("ssi_hull_sup", sup, band) {
        Ok(Sign::Zero) => Ok((worst, sup)),
        Ok(Sign::Positive | Sign::Negative) => Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value: sup,
        }),
        Err(diag) => Err(SsiError::Escalated(diag)),
    }
}

/// A whole-domain bound on `‖C′‖` for a **non-rational** fitted curve,
/// from the derivative control-coefficient hulls (one per channel,
/// folded Euclidean — conservative, which is the safe direction).
fn speed_bound3(curve: &NurbsCurve3<f64>) -> f64 {
    let coords = curve.ring_coords();
    let mut acc = 0.0f64;
    for ch in coords.iter() {
        let h = hull::derivative_domain_hull(curve.knots(), ch);
        let m = h.mag();
        acc += m * m;
    }
    acc.sqrt()
}

/// The same for a 2-D pcurve, per channel (`[u, v]`).
fn speed_bound2(curve: &NurbsCurve2<f64>) -> [f64; 2] {
    let coords = curve.ring_coords();
    let mut out = [f64::NAN; 2];
    for (i, ch) in coords.iter().enumerate().take(2) {
        out[i] = hull::derivative_domain_hull(curve.knots(), ch).mag();
    }
    out
}

/// Limb 1 + limb 2 against a **NURBS** operand, using the traced
/// pcurve as the parameter map (module docs).
fn nurbs_limbs(
    carrier: &NurbsCurve3<f64>,
    pcurve: &NurbsCurve2<f64>,
    surface: &NurbsSurface<f64>,
    band: Band,
) -> Result<(f64, f64), SsiError> {
    // ---- limb 1: the fixed schedule, through certified foot points --
    let (t0, t1) = carrier.domain();
    let mut worst = 0.0f64;
    for i in 0..CERT_SAMPLES {
        #[allow(clippy::cast_precision_loss)]
        let t = t0 + (t1 - t0) * (f64::from(i) / f64::from(CERT_SAMPLES - 1));
        let c = carrier.eval(t);
        // Warm-start from the trace's own pcurve: the projection is a
        // *check*, and starting it where the trace says the foot is
        // makes a disagreement visible rather than hidden by a global
        // seeding sweep landing somewhere else.
        let p = pcurve.eval(t);
        let proj = surface.project_from_seed(c, p.x, p.y).map_err(|e| {
            SsiError::FootPointInconclusive {
                t,
                last_distance: e.last_distance,
            }
        })?;
        if proj.distance > worst {
            worst = proj.distance;
        }
        match decide("ssi_on_locus_foot", proj.distance, band) {
            Ok(Sign::Zero) => {}
            Ok(Sign::Positive | Sign::Negative) => {
                return Err(SsiError::CertificateLimb {
                    limb: SsiLimb::OnLocus,
                    value: proj.distance,
                });
            }
            Err(diag) => return Err(SsiError::Escalated(diag)),
        }
        // The orthogonality residuals, normalized by the chart speeds
        // so the margin is a length: |S_d·r|/|S_d| is the component of
        // the offset along that parameter line, in meters.
        let jet = surface.ders(proj.u, proj.v);
        for (res, speed) in [
            (proj.orthogonality_u, jet.du.norm()),
            (proj.orthogonality_v, jet.dv.norm()),
        ] {
            let margin = res / speed;
            match decide("ssi_foot_orthogonality", margin, band) {
                Ok(Sign::Zero) => {}
                Ok(Sign::Positive | Sign::Negative) => {
                    return Err(SsiError::CertificateLimb {
                        limb: SsiLimb::OnLocus,
                        value: margin,
                    });
                }
                Err(diag) => return Err(SsiError::Escalated(diag)),
            }
        }
    }

    // ---- limb 2: per-span hull enclosure of |C(t) − S(P(t))| ----
    let fine = refined(carrier);
    let coords = fine.ring_coords();
    let kv = fine.knots();
    let dc_domain = speed_bound3(&fine);
    let dp_domain = speed_bound2(pcurve);
    let pcoords = pcurve.ring_coords();
    let pkv = pcurve.knots();
    // The carrier and its pcurves are fitted on ONE parameterization
    // (the OQ4 contract), so when neither has been refined further they
    // carry the same knot vector and a span index means the same thing
    // on both. When it does not, the whole-domain bound stands in —
    // looser, never unsound.
    let aligned = pkv.control_count() == kv.control_count() && pkv.degree() == kv.degree();
    let boxes = NurbsBoxes::new(surface);
    let mut sup = 0.0f64;
    for span in kv.first_span()..=kv.last_span() {
        if !kv.span_is_nonempty(span) {
            continue;
        }
        let (a, b) = (kv.knots()[span], kv.knots()[span + 1]);
        let half = 0.5 * (b - a);
        let m = 0.5 * (a + b);
        // Curve variation over the span, from its own derivative hull.
        let mut dc2 = 0.0f64;
        for ch in coords.iter() {
            let h = hull::derivative_span_hull(kv, ch, span).mag();
            dc2 += h * h;
        }
        let rad_c = dc2.sqrt().min(dc_domain) * half;
        // The parameter window the pcurve can reach inside the span.
        let pm = pcurve.eval(m);
        let dp = if aligned {
            [
                hull::derivative_span_hull(pkv, &pcoords[0], span).mag(),
                hull::derivative_span_hull(pkv, &pcoords[1], span).mag(),
            ]
        } else {
            dp_domain
        };
        let (ru, rv) = (dp[0] * half, dp[1] * half);
        let (u0, u1) = (pm.x - ru, pm.x + ru);
        let (v0, v1) = (pm.y - rv, pm.y + rv);
        // Surface variation over that window, from its derivative boxes.
        let du = boxes.deriv_box(u0, u1, v0, v1, true);
        let dv = boxes.deriv_box(u0, u1, v0, v1, false);
        let mag = |b: Box3| {
            (b.x.mag() * b.x.mag() + b.y.mag() * b.y.mag() + b.z.mag() * b.z.mag()).sqrt()
        };
        let rad_s = mag(du) * ru + mag(dv) * rv;
        let mid = (fine.eval(m) - surface.eval(pm.x, pm.y)).norm();
        let bound = rad_c + mid + rad_s;
        // NaN-catching: a poisoned term must become the reported sup,
        // not be skipped by a comparison it silently fails.
        if bound.is_nan() || bound > sup {
            sup = bound;
        }
    }
    match decide("ssi_hull_sup_chart", sup, band) {
        Ok(Sign::Zero) => Ok((worst, sup)),
        Ok(Sign::Positive | Sign::Negative) => Err(SsiError::CertificateLimb {
            limb: SsiLimb::HullSup,
            value: sup,
        }),
        Err(diag) => Err(SsiError::Escalated(diag)),
    }
}

/// The box chain covering a carrier: one padded box per span of the
/// refined curve, from the span's control hull (exact containment for a
/// non-rational curve — the convex-hull property).
fn box_chain(carrier: &NurbsCurve3<f64>) -> Vec<(Box3, Vec3<f64>)> {
    let fine = refined(carrier);
    let coords = fine.ring_coords();
    let kv = fine.knots();
    let mut out = Vec::new();
    for span in kv.first_span()..=kv.last_span() {
        if !kv.span_is_nonempty(span) {
            continue;
        }
        let (a, b) = (kv.knots()[span], kv.knots()[span + 1]);
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
        let t = fine.deriv(0.5 * (a + b));
        let n = t.norm();
        out.push((bx, if n > 0.0 { t / n } else { t }));
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
fn probe_tube_analytic(
    chain: &[(Box3, Vec3<f64>)],
    s1: &Surface<f64>,
    s2: &Surface<f64>,
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
fn probe_tube_chart(
    pcurve: &NurbsCurve2<f64>,
    surface: &NurbsSurface<f64>,
    normal: Vec3<f64>,
    radius_uv: (f64, f64),
) -> Option<(f64, u32)> {
    let kv = pcurve.knots();
    let boxes = NurbsBoxes::new(surface);
    let coords = pcurve.ring_coords();
    let mut worst = f64::INFINITY;
    let mut count = 0u32;
    for span in kv.first_span()..=kv.last_span() {
        if !kv.span_is_nonempty(span) {
            continue;
        }
        let (a, b) = (kv.knots()[span], kv.knots()[span + 1]);
        let m = 0.5 * (a + b);
        let hu = hull::span_hull(kv, &coords[0], span);
        let hv = hull::span_hull(kv, &coords[1], span);
        let (u0, u1) = (hu.lo() - radius_uv.0, hu.hi() + radius_uv.0);
        let (v0, v1) = (hv.lo() - radius_uv.1, hv.hi() + radius_uv.1);
        let du = boxes.deriv_box(u0, u1, v0, v1, true);
        let dv = boxes.deriv_box(u0, u1, v0, v1, false);
        let n = [
            RingInterval::point(normal.x),
            RingInterval::point(normal.y),
            RingInterval::point(normal.z),
        ];
        let phi_u = n[0] * du.x + n[1] * du.y + n[2] * du.z;
        let phi_v = n[0] * dv.x + n[1] * dv.y + n[2] * dv.z;
        let t = pcurve.deriv(m);
        let tn = (t.x * t.x + t.y * t.y).sqrt();
        if tn.is_nan() || tn <= 0.0 {
            return None;
        }
        // e⊥ = (−t.y, t.x)/‖t‖; the transverse derivative of φ.
        let ex = RingInterval::point(-t.y / tn);
        let ey = RingInterval::point(t.x / tn);
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
pub(crate) fn tube_boxes(carrier: &NurbsCurve3<f64>, radius: f64) -> Vec<Box3> {
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
/// `arm` is the folded curvature/extent lever arm the transversality
/// margin is stated over (metres); `extent` is the caller's named
/// feature extent, which sets the tube ladder's widest rung.
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
pub(crate) fn certify_branch(
    carrier: &NurbsCurve3<f64>,
    pcurve_b: Option<&NurbsCurve2<f64>>,
    a: &SsiOperand<'_>,
    b: &SsiOperand<'_>,
    arm: f64,
    extent: f64,
    eps: f64,
    band: Band,
) -> Result<SsiCertificate, SsiError> {
    let mut on_locus = 0.0f64;
    let mut hull_sup = 0.0f64;
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
    for radius in tube_ladder(extent, eps) {
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
                           tube is not implemented in this build (per-arm retirement, \
                           C12.1)",
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
        return Err(SsiError::CertificateLimb {
            limb: SsiLimb::Tube,
            value: f64::NAN,
        });
    };
    let transversality = margin * arm;
    match decide("ssi_tube_transversality", transversality, band) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => {
            return Err(SsiError::TubeStraddles {
                margin: transversality,
                boxes,
            });
        }
        Err(diag) => return Err(SsiError::Escalated(diag)),
    }
    Ok(SsiCertificate {
        samples: CERT_SAMPLES,
        on_locus_max: on_locus,
        hull_sup,
        tube_radius: radius,
        tube_transversality: transversality,
        tube_boxes: boxes,
    })
}

/// The witness of a rung-3 carrier: `carrier(mid)`, unchanged from M2
/// (`WitnessMidpoint`; S2 stays discharged).
pub(crate) fn witness(carrier: &NurbsCurve3<f64>) -> Point3<f64> {
    let (t0, t1) = carrier.domain();
    carrier.eval(0.5 * (t0 + t1))
}

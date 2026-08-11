//! D7 stage-1 NURBS **curve** recognition (#327): every imported NURBS
//! curve carrier is tested for promotion to an analytic kind, and
//! promotion fires iff the per-kind residual CERTIFIES at ε_in —
//! verified, never trusted. The curve analogue of [`crate::recognize`]
//! (surfaces, #256/M7-6), sharing its posture verbatim:
//!
//! * **the file's form flag is NEVER consulted.** dm1's edge #685
//!   carries `.CIRCULAR_ARC.` on its `B_SPLINE_CURVE`; b123d writes
//!   `.UNSPECIFIED.` for geometry of the same class. Two witnesses,
//!   one conclusion — the flag is a hint, the certificate is the
//!   authority;
//! * **the estimator only PROPOSES; the certificate decides.** A wrong
//!   estimate fails certification and the carrier stays NURBS, so an
//!   estimator can never make promotion incorrect, only incomplete;
//! * **conditioning-gated typed ambiguity** ([`CurveRecognition::IllConditioned`]);
//! * **`flush_zero` on every DERIVED frame** (center/axis/u_ref/origin/
//!   dir): the reader's numeric path states `+0.0` as the one
//!   representative, so a promoted carrier's derived frame must state
//!   it too or the promoted one-cycle re-export fixed point misses by
//!   exactly those sign bits;
//! * **a carrier that certifies nowhere stays NURBS silently** — the
//!   pre-#327 state; recognition failing must never refuse an edge
//!   that imports today.
//!
//! # Kind scope (stage 1)
//!
//! [`PromotedCurveKind::Circle`], restricted to the **closed
//! full-period** class. Line-as-degree-1, ellipse, helix, and partial
//! (open) circular arcs are NAMED EXCLUSIONS with filed follow-ups —
//! no dead per-kind code here.
//!
//! **The line kind was ATTEMPTED and stopped, measured.** A degree-1
//! carrier certifies trivially (the zero-radius cylinder composite is
//! `dist²` to the chord line, and the control-projection excursion
//! bounds the SEGMENT residual by convexity), and dm1's 37 polyline
//! carriers all certify. What it costs is downstream: a promoted
//! `Curve3::Line` changes the edge's adopted DESCRIPTION from
//! `IsoCurve` — whose exact `Pcurve::IsoLine` chart image the mint
//! already knows — to `MappedCurve::ExtrudedPoint` (the
//! `adopt::mapped_self_description` arm for a line on a
//! non-`nurbs_rim` edge), for which `topo::pcurves::nurbs_iso_derive`
//! has no derivation at all. Measured: dm1 refuses at the FIRST such
//! edge with `IsoUnsupported`, strictly earlier than it refuses
//! today. Closing that is a pcurve-lane rung, not a recognition
//! question, so it is its own unit and its own issue.
//!
//! # The certificate (D9-clean by construction: no sampling, no
//! schedule)
//!
//! Unlike the surface recognizer's curved-kind track — which needs a
//! sampled grid plus a first-order envelope, and pays for it in slack
//! — the curve case has an EXACT whole-domain certificate available:
//! [`geom_core::spline::compose`]'s ring-coefficient implicit
//! composites. `implicit_composite(curve, f)` builds `f ∘ C` in
//! rational Bernstein form with interval coefficients, and
//! [`compose::CompositeForm::sup_bound`] reads a certified sup off its
//! coefficient hulls. Data in, bounds out: nothing is evaluated, no
//! schedule exists to be data-dependent, and rational carriers are
//! first-class (the weight channel is part of the form). Poison
//! anywhere yields `NaN`, which fails every `≤ ε` comparison (D4 ¶2).
//!
//! ## The dimension conversion, as an invariant
//!
//! `compose`'s implicit surfaces are not all metered in meters (its
//! module docs state the convention): the plane composite is
//! `n·(P − p₀)` — meters for unit `n` — while sphere and cylinder are
//! `|P − c|² − r²` and `|Q|² − (Q·â)² − r²` — **meters²**. A residual
//! is only comparable to ε_in in meters, so each conversion is stated
//! here as an invariant, each one an inequality that holds for every
//! point (never an approximation, never a linearization):
//!
//! * **INV-C1 (sphere → distance).** If `| |P−c|² − r² | ≤ S` then
//!   `| |P−c| − r | = S′/(|P−c| + r) ≤ S/r`, because `|P−c| ≥ 0` makes
//!   the divisor at least `r`. Conversion: `δ_s = S / r`.
//! * **INV-C2 (plane ∧ sphere → distance to the CIRCLE).** Write
//!   `P − c = h·n̂ + q` with `q ⊥ n̂`. The plane certificate gives
//!   `|h| ≤ δ_p`; INV-C1 gives `| |P−c| − r | ≤ δ_s`, and
//!   `|q|² = |P−c|² − h²`. Hence `|q| ≤ r + δ_s` above and
//!   `|q| ≥ sqrt((r−δ_s)² − δ_p²)` below, so
//!   `| |q| − r | ≤ m := max(δ_s, r − sqrt(max(0, (r−δ_s)² − δ_p²)))`.
//!   The distance from `P` to the circle `plane ∩ sphere` is exactly
//!   `hypot(|q| − r, h)`, so `residual ≤ hypot(m, δ_p)`. Every step is
//!   an inequality between certified quantities; no small-angle or
//!   first-order step appears.
//!
//! # Estimators (D9-clean: closed form, fixed evaluation order)
//!
//! * **Circle**: three samples of the carrier at the FIXED domain
//!   fractions 0, ¼, ½ (fixed schedule, data-independent — D9); the
//!   plane through them oriented by `(s₁−s₀) × (s₂−s₀)`, which is the
//!   INCREASING-PARAMETER winding, so the promoted circle's `axis`
//!   means what the carrier's direction meant; the exact circumcenter
//!   solve for `center` and `radius` (the surface recognizer's, in the
//!   samples' own plane); `u_ref` toward `s₀`, so the promoted seam
//!   sits at the carrier's start point and the derived parameter
//!   interval starts at `θ = 0`.
//!
//! # Conditioning (D7's typed ambiguity)
//!
//! The circle estimator carries the surface cylinder estimator's
//! margin trilean: three samples collinear within ε_in determine no
//! finite radius, which is [`CurveRecognition::IllConditioned`]. No
//! call site ESCALATES it today — unlike the surface case there is no
//! gate that needs a curve promotion to import at all — so an
//! ill-conditioned carrier stays NURBS exactly like a refuted one.
//! The tri-state is kept because the distinction is real and the
//! escalation site is the thing that is missing, not the class.
//!
//! # The full-period coverage witness (stated as what it is)
//!
//! A locus certificate says the carrier lies ON the circle; it does
//! not say the carrier COVERS it. For the closed class the adopted
//! edge is the full period ([`crate::geometry::endpoint_params`]'s
//! self-loop arm), so a carrier that doubled back over a sub-arc would
//! adopt as a strictly larger locus. The gate against that is a
//! **turning witness**: five samples at the fixed fractions 0, ¼, ½,
//! ¾, 1, their azimuths in the estimated frame, and the four wrapped
//! increments required STRICTLY POSITIVE. Since the carrier is closed,
//! the increments sum to a multiple of 2π; all-positive then forces
//! that multiple to be at least one full turn. This is a fixed-
//! schedule NECESSARY condition on the traversal, not a proof of
//! monotone azimuth — the proof wants a derivative composite
//! (`(Q × Q′)·â` in ring coefficients), which `compose` does not
//! expose for curves today and which is a named follow-up. It is
//! recorded here as the one non-certified limb of the promotion, and
//! it is conservative: failing it refuses, never promotes.

use geom_core::spline::compose::{self, CurveRingData, ImplicitSurface};
use geom_core::{Point3, Vec3};
use geom_curves::{Curve3, NurbsCurve3};

pub(crate) use crate::PromotedCurveKind;

/// The outcome of testing one NURBS curve carrier for promotion —
/// the tri-state of [`crate::recognize::Recognition`], for curves.
#[derive(Debug)]
pub(crate) enum CurveRecognition {
    /// A kind certified at ε_in.
    Promoted {
        /// The analytic carrier. A circle's `axis` winds with the
        /// carrier's increasing parameter and its `u_ref` sits at the
        /// carrier's start point; a line's `dir` runs start → end, so
        /// the derived parameter interval keeps its orientation
        /// through the promotion.
        curve: Curve3<f64>,
        /// The certified residual sup (METERS — module docs INV-C1..4).
        residual: f64,
        /// Which kind certified.
        kind: PromotedCurveKind,
    },
    /// No implemented kind certifies — the carrier stays NURBS
    /// (the pre-#327 state, silent).
    StaysNurbs,
    /// An estimator could not answer at ε_in (its own margin trilean).
    IllConditioned {
        /// The kind whose estimator declined.
        kind: PromotedCurveKind,
        /// The margin that fell inside ε_in (meters).
        margin: f64,
    },
}

/// Tests `curve` for promotion at the interpretation budget `eps_in`
/// (module docs: kind, estimator, certificate).
///
/// One kind, so no preference order exists to state. When a second
/// kind lands, the surface recognizer's rule applies verbatim: a fixed
/// order, and the note that a carrier certifying as two kinds is
/// canonicalization rather than ambiguity (both analytic curves agree
/// with the carrier, hence with each other, within 2·ε_in everywhere
/// on it).
pub(crate) fn recognize(curve: &NurbsCurve3<f64>, eps_in: f64) -> CurveRecognition {
    match try_circle(curve, eps_in) {
        Ok(Some((circle, residual))) => CurveRecognition::Promoted {
            curve: circle,
            residual,
            kind: PromotedCurveKind::Circle,
        },
        Ok(None) => CurveRecognition::StaysNurbs,
        Err(margin) => CurveRecognition::IllConditioned {
            kind: PromotedCurveKind::Circle,
            margin,
        },
    }
}

/// Negative zeros flushed to `+0.0`, componentwise — verbatim
/// [`crate::recognize`]'s, for the same re-export fixed-point reason
/// (its docs carry the argument).
fn flush_zero(v: Vec3<f64>) -> Vec3<f64> {
    Vec3::new(v.x + 0.0, v.y + 0.0, v.z + 0.0)
}

/// [`flush_zero`] for a point.
fn flush_zero_point(p: Point3<f64>) -> Point3<f64> {
    Point3::new(p.x + 0.0, p.y + 0.0, p.z + 0.0)
}

/// The certified sup of `|f ∘ C|` over the whole domain, in the
/// composite's OWN units (module docs' scaling conventions). `NaN` on
/// every refusal and every poison path, which certifies nothing.
fn composite_sup(curve: &NurbsCurve3<f64>, surface: &ImplicitSurface) -> f64 {
    let coords = curve.ring_coords();
    let Ok(data) = CurveRingData::new(curve.knots(), curve.weights(), &coords) else {
        return f64::NAN;
    };
    match compose::implicit_composite(&data, surface) {
        Ok(form) => form.sup_bound(),
        Err(_) => f64::NAN,
    }
}

/// The circle candidate (module docs; certificate INV-C1 + INV-C3,
/// plus the full-period turning witness). `Err(margin)` is the
/// estimator's conditioning refusal; `Ok(None)` a refuted certificate
/// or an out-of-scope (open-arc) carrier.
fn try_circle(curve: &NurbsCurve3<f64>, eps_in: f64) -> Result<Option<(Curve3<f64>, f64)>, f64> {
    let control = curve.control();
    let (Some(first), Some(last)) = (control.first(), control.last()) else {
        return Ok(None);
    };
    if control.len() < 3 {
        return Ok(None);
    }
    // **Stage-1 scope: the CLOSED full-period class.** An open arc
    // needs an arc-COVERAGE certificate the promotion does not have
    // (module docs) — refused here, filed as a follow-up, silent.
    if (*last - *first).norm() > eps_in {
        return Ok(None);
    }
    let (a, b) = curve.domain();
    if !(a.is_finite() && b.is_finite() && b > a) {
        return Ok(None);
    }
    let sample = |f: f64| curve.eval(a + (b - a) * f);
    let s0 = sample(0.0);
    let s1 = sample(0.25);
    let s2 = sample(0.5);
    // Conditioning: the middle sample's sagitta over the chord —
    // collinear-at-ε_in samples determine no finite radius (the
    // surface recognizer's near-flat class, verbatim).
    let chord = s2 - s0;
    let chord_norm = chord.norm();
    if !(chord_norm.is_finite() && chord_norm > 0.0) {
        return Ok(None);
    }
    let chord_dir = chord / chord_norm;
    let off = (s1 - s0) - chord_dir * (s1 - s0).dot(chord_dir);
    let sagitta = off.norm();
    if !sagitta.is_finite() {
        return Ok(None);
    }
    if sagitta <= eps_in {
        return Err(sagitta);
    }
    // The plane through the three samples, oriented by the
    // INCREASING-PARAMETER winding.
    let normal = (s1 - s0).cross(s2 - s0);
    let normal_norm = normal.norm();
    if !(normal_norm.is_finite() && normal_norm > 0.0) {
        return Ok(None);
    }
    let axis = normal / normal_norm;
    // The exact circumcenter of three coplanar points (the surface
    // recognizer's closed form).
    let u = s1 - s0;
    let v = s2 - s0;
    let uxv = u.cross(v);
    let denom = 2.0 * uxv.norm_squared();
    let center = s0 + (uxv.cross(u) * v.norm_squared() + v.cross(uxv) * u.norm_squared()) / denom;
    let radial = s0 - center;
    let radius = radial.norm();
    if !(radius.is_finite() && radius > eps_in) {
        return Ok(None);
    }
    let u_ref = radial / radius;
    let v_ref = axis.cross(u_ref);
    // **The turning witness** (module docs): five fixed samples, four
    // wrapped azimuth increments, all required strictly positive.
    let azimuth = |p: Point3<f64>| -> f64 {
        let w = p - center;
        w.dot(v_ref).atan2(w.dot(u_ref))
    };
    let tau = core::f64::consts::TAU;
    let mut previous = azimuth(s0);
    for k in 1..=4 {
        let theta = azimuth(sample(f64::from(k) / 4.0));
        if !theta.is_finite() {
            return Ok(None);
        }
        let mut step = theta - previous;
        while step <= 0.0 {
            step += tau;
        }
        while step > tau {
            step -= tau;
        }
        // Each increment must be a strictly positive advance of less
        // than a full turn; a sample that does not advance (or that
        // advances by a whole period between two consecutive fixed
        // fractions) is not a witness of one clean turn.
        if !(step > 0.0 && step < tau) {
            return Ok(None);
        }
        previous = theta;
    }
    // The certificate. INV-C1 + INV-C3, both in meters at the end.
    let plane_sup = composite_sup(
        curve,
        &ImplicitSurface::Plane {
            point: [center.x, center.y, center.z],
            normal: [axis.x, axis.y, axis.z],
        },
    );
    let sphere_sup = composite_sup(
        curve,
        &ImplicitSurface::Sphere {
            center: [center.x, center.y, center.z],
            radius,
        },
    );
    let delta_p = plane_sup.abs();
    // INV-C1: meters² → meters, divided by the radius (the smallest
    // the divisor `|P−c| + r` can be).
    let delta_s = sphere_sup.abs() / radius;
    // INV-C2: the two into one distance-to-the-circle bound.
    let inner = (radius - delta_s).powi(2) - delta_p * delta_p;
    let lower = if inner > 0.0 { inner.sqrt() } else { 0.0 };
    let m = delta_s.max(radius - lower);
    let residual = m.hypot(delta_p);
    if !residual.is_finite() {
        return Ok(None);
    }
    let circle = Curve3::Circle {
        center: flush_zero_point(center),
        axis: flush_zero(axis),
        radius,
        u_ref: flush_zero(u_ref),
    };
    Ok((residual <= eps_in).then_some((circle, residual)))
}

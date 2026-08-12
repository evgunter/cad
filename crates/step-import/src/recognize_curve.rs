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
//!   `hypot(|q| − r, h)`, and `hypot` is monotone in both arguments,
//!   so `residual ≤ hypot(m, δ_p)`. Every step is an inequality
//!   between certified quantities; no small-angle or first-order step
//!   appears.
//!
//!   **Hypothesis, stated rather than assumed**: the lower bound needs
//!   `r − δ_s ≥ 0`. Where it fails the expression under the root is
//!   negative, the code takes `lower = 0`, and `m ≥ r > ε_in` makes
//!   the promotion refuse — so the corner is safe, but it is safe
//!   BECAUSE of that arm and not by the derivation, and the code says
//!   so at the site.
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
//! # Coverage — the OTHER half of the promotion, certified
//!
//! A locus certificate says the carrier lies ON the circle; it does
//! not say the carrier COVERS it. For the closed class the adopted
//! edge is the full period ([`crate::geometry::endpoint_params`]'s
//! self-loop arm), so a carrier that doubled back over a sub-arc would
//! adopt as a strictly LARGER locus — a silent false promotion, the
//! exact class #256's posture exists to prevent.
//!
//! [`covers_one_full_turn`] is that gate, and it is a CERTIFICATE, not
//! a witness. The distinction matters and was learned the hard way: an
//! earlier draft sampled five fixed domain fractions and required the
//! wrapped azimuth increments positive. Wrapping a difference into a
//! positive interval MANUFACTURES the positivity, so that check
//! refused only exactly-equal consecutive azimuths — near-vacuous, and
//! two independently constructed on-locus carriers (a 120° and a 300°
//! out-and-back, both exactly on the circle) promoted through it. Both
//! are pinned red here now.
//!
//! No amount of sampling could have fixed it: any three points of a
//! circle lie in a common arc shorter than 2π, so a finite sample set
//! can never prove coverage. What works is a STRUCTURAL bound that
//! makes each wrapped increment equal the true one — the per-knot-span
//! half-plane containment of [`covers_one_full_turn`], whose derivation
//! is on that function. The schedule is the carrier's own span
//! structure (knots, not data), so it is D9-clean.
//!
//! The gate refuses whenever it cannot tell, so it can make promotion
//! incomplete but never incorrect. What remains banked in the
//! surjectivity follow-up is the *tightness*: a carrier whose spans
//! individually turn by π or more is refused rather than analysed.

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
        /// The certified residual sup (METRES — module docs INV-C1/C2).
        residual: f64,
        /// Which kind certified.
        kind: PromotedCurveKind,
    },
    /// No implemented kind certifies — the carrier stays NURBS
    /// (the pre-#327 state, silent).
    StaysNurbs,
    /// An estimator could not answer at ε_in (its own margin trilean).
    ///
    /// **Its payload has no consumer yet, deliberately** (module docs:
    /// conditioning): unlike the surface case there is no gate that
    /// needs a curve promotion in order to import at all, so nothing
    /// escalates this and the carrier stays NURBS exactly like a
    /// refuted one. The payload is kept because the distinction is
    /// real and the ESCALATION SITE is what is missing, not the class
    /// — a refusal that could say "recognition declined this carrier
    /// at margin m" is the follow-up. Read by this module's own pins;
    /// the allow is that statement, not a shrug.
    #[allow(dead_code)]
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

/// The circle candidate (module docs; the locus certificate INV-C1 +
/// INV-C2, then the coverage certificate). `Err(margin)` is the
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
    // The certificate. INV-C1 + INV-C2, both in metres at the end.
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
    // INV-C2: the two into one distance-to-the-circle bound. The
    // lower-bound branch is valid only for `r − δ_s ≥ 0` (invariant
    // docs); outside it `inner` is not a bound and `lower = 0` makes
    // `m ≥ r`, which is `> eps_in` for every radius this estimator
    // admits — refusal, not a silent wrong number.
    let inner = (radius - delta_s).powi(2) - delta_p.powi(2);
    let lower = if inner > 0.0 { inner.sqrt() } else { 0.0 };
    let m = delta_s.max(radius - lower);
    let residual = m.hypot(delta_p);
    if !residual.is_finite() {
        return Ok(None);
    }
    if residual > eps_in {
        return Ok(None);
    }
    // **The COVERAGE certificate** (module docs). The locus half is
    // now established; this is the other half, and it is a
    // certificate rather than a witness. A carrier that lies on the
    // circle but does not COVER it must not become the circle.
    if !covers_one_full_turn(curve, center, axis, u_ref) {
        return Ok(None);
    }
    let circle = Curve3::Circle {
        center: flush_zero_point(center),
        axis: flush_zero(axis),
        radius,
        u_ref: flush_zero(u_ref),
    };
    Ok(Some((circle, residual)))
}

/// **The coverage certificate**: does `curve` wind exactly once around
/// the estimated circle? (module docs, "Coverage"). `true` only when
/// that is PROVEN; every path that cannot tell answers `false`, so the
/// gate can make promotion incomplete but never incorrect.
///
/// # Why the schedule is the carrier's own spans
///
/// The obstruction to reading a turning number off samples is that a
/// sampled azimuth increment is only known modulo 2π: wrapping it into
/// any interval MANUFACTURES a value, and summing manufactured values
/// proves nothing. The fix is not more samples — no finite sample set
/// can prove coverage, since any three points of a circle lie in a
/// common arc shorter than 2π — but a structural bound that makes the
/// wrapped increment EQUAL the true one.
///
/// That bound is per KNOT SPAN, and it is exact:
///
/// 1. On one knot span the curve is a convex combination of that
///    span's `degree + 1` local control points (rational, strictly
///    positive weights — the hull property `compose` also relies on).
///    Project those into the circle's plane. If they all lie strictly
///    on ONE side of some line through the centre — tested against
///    their own normalized sum, so the direction is data-derived but
///    the test is a fixed-order scan — then the whole span's image
///    lies in that open half-plane.
/// 2. Every point of an open half-plane bounded by a line through the
///    centre has azimuth within an open interval of length π. A
///    CONTINUOUS azimuth lift over the span therefore cannot leave one
///    such interval, so the span's true azimuth increment is strictly
///    between −π and π. (It also proves the curve never meets the
///    centre, which is what makes the lift exist at all.)
/// 3. An increment known to lie in `(−π, π)` is recovered EXACTLY by
///    wrapping the sampled difference into `(−π, π]`: the two are
///    congruent mod 2π and both in that interval, hence equal. No
///    value is manufactured.
/// 4. Summing the recovered increments over the spans in ascending
///    order gives the carrier's TRUE total turning, `θ(b) − θ(a)`.
///
/// # What the total is allowed to be
///
/// The carrier is closed at ε_in (checked before the estimator runs),
/// so the total is `2π·w` up to that closure gap, `w` the winding
/// number. Exactly ONE positive turn is required — the total in
/// `(π, 3π)`. Then `θ` is continuous with total change ≥ 2π minus the
/// closure gap, so by the intermediate value theorem its image is an
/// interval of that length: the carrier covers the circle except
/// possibly a gap of at most `ε_in / r` in angle, which is precisely
/// the ε_in-closure the file's own self-loop vertex already asserts.
///
/// `w = 0` (any doubling-back) and `|w| ≥ 2` (a multiply-wound
/// carrier, whose locus is equal but whose traversal the adopted
/// single period does not describe) both refuse. `w = −1` refuses too:
/// the estimator's axis fixes the positive sense, and a carrier that
/// disagrees with it is one the estimator read wrongly.
// `!(a < b)` and `!(x > 0.0)` are deliberate, NaN-catching negations:
// a poisoned coordinate must REFUSE, and the positive form would
// silently accept it. Same convention, same reason, as
// `geom_core::spline::compose`'s weight checks.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn covers_one_full_turn(
    curve: &NurbsCurve3<f64>,
    center: Point3<f64>,
    axis: Vec3<f64>,
    u_ref: Vec3<f64>,
) -> bool {
    let v_ref = axis.cross(u_ref);
    let control = curve.control();
    let kv = curve.knots();
    let flat = kv.knots();
    let degree = kv.degree();
    let count = control.len();
    // The clamped-B-spline structure this reasoning assumes, checked
    // rather than trusted (a degree-0 carrier has no hull argument).
    if degree == 0 || flat.len() != count + degree + 1 {
        return false;
    }
    // A point's coordinates in the estimated circle's own plane.
    let plane = |p: Point3<f64>| -> (f64, f64) {
        let w = p - center;
        (w.dot(u_ref), w.dot(v_ref))
    };
    let pi = core::f64::consts::PI;
    let tau = core::f64::consts::TAU;
    let mut total = 0.0f64;
    let mut previous: Option<f64> = None;
    for j in degree..count {
        // Empty spans carry no turning and no obligation.
        if !(flat[j] < flat[j + 1]) {
            continue;
        }
        // Step 1: the span's local control points, all strictly on one
        // side of a line through the centre.
        let (mut sx, mut sy) = (0.0f64, 0.0f64);
        for p in &control[j - degree..=j] {
            let (x, y) = plane(*p);
            sx += x;
            sy += y;
        }
        let norm = sx.hypot(sy);
        if !(norm.is_finite() && norm > 0.0) {
            return false;
        }
        let (dx, dy) = (sx / norm, sy / norm);
        for p in &control[j - degree..=j] {
            let (x, y) = plane(*p);
            // `!(… > 0.0)` is deliberate: a NaN must refuse.
            if !(x * dx + y * dy > 0.0) {
                return false;
            }
        }
        // Steps 2-4: the span's TRUE increment, by exact recovery.
        let at = |t: f64| -> f64 {
            let (x, y) = plane(curve.eval(t));
            y.atan2(x)
        };
        let start = match previous {
            Some(a) => a,
            None => at(flat[j]),
        };
        let end = at(flat[j + 1]);
        let mut step = end - start;
        while step > pi {
            step -= tau;
        }
        while step <= -pi {
            step += tau;
        }
        if !step.is_finite() {
            return false;
        }
        total += step;
        previous = Some(end);
    }
    total > pi && total < 3.0 * pi
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::spline::KnotVector;

    const EPS_IN: f64 = 1e-9;

    /// The 3×120° rational-quadratic full circle — dm1's edge #685
    /// form, parameterized as the file states it (knots at multiples
    /// of `√3`, weights `1, ½, …`, the tangent-intersection corners at
    /// `r/cos(60°) = 2r`).
    fn rational_circle(radius: f64, reversed: bool) -> NurbsCurve3<f64> {
        let tau = core::f64::consts::TAU;
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for k in 0..7 {
            let on = k % 2 == 0;
            let theta = tau * (k as f64) / 6.0 * if reversed { -1.0 } else { 1.0 };
            // On-circle points at 0°, 120°, 240°; corners between them
            // at twice the radius (the tangent intersection).
            let r = if on { radius } else { radius * 2.0 };
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), 0.25));
            weights.push(if on { 1.0 } else { 0.5 });
        }
        let s = 3.0f64.sqrt();
        let knots = KnotVector::clamped(
            vec![
                0.0,
                0.0,
                0.0,
                s,
                s,
                2.0 * s,
                2.0 * s,
                3.0 * s,
                3.0 * s,
                3.0 * s,
            ],
            2,
        )
        .unwrap();
        NurbsCurve3::new(knots, control, weights).unwrap()
    }

    /// C1: the unit — an exact rational-quadratic circle certifies, and
    /// the promoted frame is the geometry (not the file's opinion).
    #[test]
    fn c1_the_rational_quadratic_circle_certifies_exactly() {
        let curve = rational_circle(0.005, false);
        let CurveRecognition::Promoted {
            curve:
                Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                },
            residual,
            kind,
        } = recognize(&curve, EPS_IN)
        else {
            panic!("dm1's carrier class must certify");
        };
        assert_eq!(kind, PromotedCurveKind::Circle);
        assert!(residual < 1e-15, "an exact circle's residual: {residual:e}");
        assert!((radius - 0.005).abs() < 1e-15, "radius {radius:e}");
        assert!(center.distance(Point3::new(0.0, 0.0, 0.25)) < 1e-15);
        // Increasing parameter winds +z here, so the axis is +z and
        // the seam sits at the carrier's own start point.
        assert!((axis.z - 1.0).abs() < 1e-12, "axis {axis:?}");
        assert!((u_ref.x - 1.0).abs() < 1e-12, "u_ref {u_ref:?}");
    }

    /// C2: ORIENTATION survives the promotion — the same locus wound
    /// the other way promotes to the opposite axis, which is what
    /// keeps a wall's two rims traversing its chart in opposite
    /// directions after both are promoted.
    #[test]
    fn c2_winding_survives_the_promotion() {
        let CurveRecognition::Promoted {
            curve: Curve3::Circle { axis, .. },
            ..
        } = recognize(&rational_circle(0.005, true), EPS_IN)
        else {
            panic!("the reversed circle must certify too");
        };
        assert!((axis.z + 1.0).abs() < 1e-12, "reversed axis {axis:?}");
    }

    /// C3: the NEGATIVE control — a genuine freeform closed spline
    /// (TAIL_TURBINE's class) stays NURBS. Recognition that promoted
    /// this would be worse than recognition that does nothing.
    #[test]
    fn c3_a_freeform_closed_spline_stays_nurbs() {
        let mut curve = rational_circle(0.005, false);
        // One control point pushed a millimetre off — three decades
        // past ε_in, and nowhere near any circle.
        let mut control = curve.control().to_vec();
        control[2] = control[2] + Vec3::new(0.001, 0.0, 0.0);
        curve = NurbsCurve3::new(curve.knots().clone(), control, curve.weights().to_vec()).unwrap();
        assert!(
            matches!(recognize(&curve, EPS_IN), CurveRecognition::StaysNurbs),
            "a dented circle is not a circle"
        );
    }

    /// C4: the NAMED EXCLUSION, executable — an OPEN arc stays NURBS.
    /// Stage-1 scope is the closed full-period class, because an open
    /// arc's adopted interval needs an arc-COVERAGE certificate the
    /// promotion does not have (module docs).
    #[test]
    fn c4_an_open_arc_stays_nurbs() {
        let full = rational_circle(0.005, false);
        // The first two spans only: a 240° arc, ends distinct.
        let control = full.control()[..5].to_vec();
        let weights = full.weights()[..5].to_vec();
        let s = 3.0f64.sqrt();
        let knots =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, s, s, 2.0 * s, 2.0 * s, 2.0 * s], 2).unwrap();
        let arc = NurbsCurve3::new(knots, control, weights).unwrap();
        assert!(matches!(
            recognize(&arc, EPS_IN),
            CurveRecognition::StaysNurbs
        ));
    }

    /// C5: the CONDITIONING trilean is reachable — a closed carrier
    /// whose three estimator samples are collinear at ε_in determines
    /// no finite radius, and that is `IllConditioned`, not a silent
    /// refutation. (Both outcomes stay NURBS; the distinction is the
    /// one D7 asks recognition to be able to make.)
    #[test]
    fn c5_a_collinear_closed_carrier_is_ill_conditioned() {
        // Out along +x and back: closed, non-degenerate net, and every
        // sample on one line.
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.01, 0.0, 0.0),
            Point3::new(0.02, 0.0, 0.0),
            Point3::new(0.01, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ];
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 3.0, 3.0, 3.0], 2).unwrap();
        let curve = NurbsCurve3::new(knots, control, vec![1.0; 5]).unwrap();
        let CurveRecognition::IllConditioned { kind, margin } = recognize(&curve, EPS_IN) else {
            panic!("collinear samples determine no radius");
        };
        assert_eq!(kind, PromotedCurveKind::Circle);
        assert!(margin <= EPS_IN, "the margin fell inside the budget");
    }

    /// An exact rational-quadratic arc chain on the circle of `radius`
    /// centred at the origin in `z = 0`, through the given azimuths —
    /// every span a TRUE arc of that circle (corner at the tangent
    /// intersection `r/cos(h/2)`, weight `cos(h/2)`), so the chain
    /// lies on the circle to the bit. Uniform integer knots.
    fn arc_chain(radius: f64, angles: &[f64]) -> NurbsCurve3<f64> {
        let spans = angles.len() - 1;
        let on = |t: f64| Point3::new(radius * t.cos(), radius * t.sin(), 0.0);
        let mut control = vec![on(angles[0])];
        let mut weights = vec![1.0];
        for k in 0..spans {
            let (a, b) = (angles[k], angles[k + 1]);
            let c = ((b - a) / 2.0).cos();
            let mid = (a + b) / 2.0;
            control.push(Point3::new(
                radius / c * mid.cos(),
                radius / c * mid.sin(),
                0.0,
            ));
            weights.push(c);
            control.push(on(b));
            weights.push(1.0);
        }
        let mut kn = vec![0.0, 0.0, 0.0];
        for k in 1..spans {
            kn.push(k as f64);
            kn.push(k as f64);
        }
        kn.extend([spans as f64; 3]);
        NurbsCurve3::new(KnotVector::clamped(kn, 2).unwrap(), control, weights).unwrap()
    }

    /// C6: the COVERAGE CERTIFICATE, tested AT THE MECHANISM.
    ///
    /// **This pin used to be green for the wrong reason**, and both
    /// reviews found it independently: its carrier placed a 120°
    /// corner (`2r`, weight ½) over a 60° chord, so the span sat
    /// 1.22 mm off a 5 mm circle and the pin died at the LOCUS
    /// certificate, never reaching the gate it is named for. It now
    /// builds its carrier with [`arc_chain`] — exactly on the circle,
    /// asserted densely here so the locus arm cannot silently take
    /// over again — and asserts the gate DIRECTLY as well as through
    /// `recognize`.
    #[test]
    fn c6_the_coverage_certificate_refuses_a_doubled_back_carrier() {
        let radius = 0.005f64;
        let center = Point3::new(0.0, 0.0, 0.0);
        let axis = Vec3::new(0.0, 0.0, 1.0);
        let u_ref = Vec3::new(1.0, 0.0, 0.0);
        let d = f64::to_radians;
        // Out to 90° and back: on the circle, covering a quarter.
        let back = arc_chain(radius, &[d(0.0), d(45.0), d(90.0), d(45.0), d(0.0)]);
        let (a, b) = back.domain();
        for k in 0..=400 {
            let p = back.eval(a + (b - a) * f64::from(k) / 400.0);
            let off = (p.x.hypot(p.y) - radius).abs().max(p.z.abs());
            assert!(off < 1e-14, "the carrier must lie ON the circle: {off:e}");
        }
        assert!(
            !covers_one_full_turn(&back, center, axis, u_ref),
            "a quarter-circle out-and-back does not wind once"
        );
        // And the gate is not vacuously false — the genuine full
        // circle, same construction, passes it.
        let full = arc_chain(radius, &[d(0.0), d(120.0), d(240.0), d(360.0)]);
        assert!(
            covers_one_full_turn(&full, center, axis, u_ref),
            "the full circle must wind once"
        );
        assert!(
            !matches!(recognize(&back, EPS_IN), CurveRecognition::Promoted { .. }),
            "a carrier covering a quarter of the circle must not become the circle"
        );
    }

    /// C7: a MULTIPLY-WOUND carrier refuses. Its locus IS the circle,
    /// so the locus certificate has nothing to object to; what refuses
    /// is the coverage certificate's "exactly one turn" arm, because
    /// the adopted single period does not describe a traversal that
    /// goes round twice.
    #[test]
    fn c7_a_double_wound_carrier_refuses() {
        let d = f64::to_radians;
        let angles: Vec<f64> = (0..=6).map(|k| d(120.0 * f64::from(k))).collect();
        let twice = arc_chain(0.005, &angles);
        assert!(
            !covers_one_full_turn(
                &twice,
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0)
            ),
            "two turns is not one turn"
        );
        assert!(!matches!(
            recognize(&twice, EPS_IN),
            CurveRecognition::Promoted { .. }
        ));
    }

    /// C8: **the certificate's SCALE, pinned from both sides.** The
    /// residual must be a true upper bound on the densely measured
    /// deviation (soundness) AND within a stated factor of it
    /// (tightness). Together these pin the metres² → metres
    /// conversions: inverting INV-C1 (`·r` for `/r`) or weakening it
    /// by any factor makes the residual smaller than the truth and
    /// breaks soundness; inflating it 4× breaks tightness. Before this
    /// pin both mutations left every test green — which is exactly
    /// what both reviews measured.
    #[test]
    fn c8_the_certified_residual_brackets_the_true_deviation() {
        let radius = 0.005f64;
        let mut tight_checks = 0;
        for delta in [0.0f64, 1e-12, 1e-11, 5e-11, 1e-10] {
            let base = rational_circle(radius, false);
            let mut control = base.control().to_vec();
            for (k, p) in control.iter_mut().enumerate() {
                p.y *= 1.0 + delta / radius;
                if k % 2 == 1 {
                    p.z += delta;
                }
            }
            let curve =
                NurbsCurve3::new(base.knots().clone(), control, base.weights().to_vec()).unwrap();
            let CurveRecognition::Promoted {
                curve:
                    Curve3::Circle {
                        center,
                        axis,
                        radius: r,
                        ..
                    },
                residual,
                ..
            } = recognize(&curve, EPS_IN)
            else {
                panic!("delta {delta:e} is inside the budget and must certify");
            };
            let (a, b) = curve.domain();
            let mut worst = 0.0f64;
            for k in 0..=8192 {
                let p = curve.eval(a + (b - a) * f64::from(k) / 8192.0);
                let w = p - center;
                let h = w.dot(axis);
                worst = worst.max(((w - axis * h).norm() - r).hypot(h));
            }
            assert!(
                worst <= residual,
                "UNSOUND at delta {delta:e}: certified {residual:e} < true {worst:e}"
            );
            // TIGHTNESS: a bound, not a shrug. The measured ratio on
            // this family is a stable 3.27; 10x leaves real headroom
            // for legitimate arithmetic drift and still catches the 4x
            // inflation of INV-C1 that a review mutated in.
            if delta > 0.0 {
                assert!(
                    residual <= worst * 10.0,
                    "LOOSE at delta {delta:e}: certified {residual:e} vs true {worst:e}"
                );
                tight_checks += 1;
            }
        }
        assert!(tight_checks >= 3, "the tightness arm must actually run");
    }

    /// C9: **the budget decides AT the residual**, pinned either side
    /// — a carrier certifying at `residual·1.5` must stay NURBS at
    /// `residual/1.5`, so no arm may quietly widen or narrow the
    /// `residual <= eps_in` decision.
    #[test]
    fn c9_the_budget_decides_at_the_residual() {
        let radius = 0.005f64;
        let base = rational_circle(radius, false);
        let mut control = base.control().to_vec();
        for p in &mut control {
            p.y *= 1.0 + 1e-10 / radius;
        }
        let curve =
            NurbsCurve3::new(base.knots().clone(), control, base.weights().to_vec()).unwrap();
        let CurveRecognition::Promoted { residual, .. } = recognize(&curve, EPS_IN) else {
            panic!("must certify at the module budget");
        };
        assert!(
            matches!(
                recognize(&curve, residual * 1.5),
                CurveRecognition::Promoted { .. }
            ),
            "must certify just above its own residual"
        );
        assert!(
            !matches!(
                recognize(&curve, residual / 1.5),
                CurveRecognition::Promoted { .. }
            ),
            "must NOT certify just below its own residual"
        );
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::float_cmp,
    clippy::panic,
    clippy::print_stdout
)]
mod r2_probes {
    use super::*;
    use geom_core::spline::KnotVector;

    const EPS_IN: f64 = 1e-9;

    /// A closed rational-quadratic polyline-of-arcs on the circle of
    /// `radius` in the z=0 plane through the given angle sequence
    /// (radians), each consecutive pair one rational-quadratic span,
    /// with the given knot breaks (domain [0,1]).
    fn arc_chain(radius: f64, angles: &[f64], breaks: &[f64]) -> NurbsCurve3<f64> {
        let n = angles.len() - 1;
        assert_eq!(breaks.len(), n - 1);
        let mut control = Vec::new();
        let mut weights = Vec::new();
        let on = |t: f64| Point3::new(radius * t.cos(), radius * t.sin(), 0.0);
        control.push(on(angles[0]));
        weights.push(1.0);
        for k in 0..n {
            let (a, b) = (angles[k], angles[k + 1]);
            let half = (b - a) / 2.0;
            let c = half.cos();
            let mid = (a + b) / 2.0;
            control.push(Point3::new(
                radius / c * mid.cos(),
                radius / c * mid.sin(),
                0.0,
            ));
            weights.push(c);
            control.push(on(b));
            weights.push(1.0);
        }
        let mut kn = vec![0.0, 0.0, 0.0];
        for b in breaks {
            kn.push(*b);
            kn.push(*b);
        }
        kn.extend([1.0, 1.0, 1.0]);
        let knots = KnotVector::clamped(kn, 2).unwrap();
        NurbsCurve3::new(knots, control, weights).unwrap()
    }

    fn azimuths(curve: &NurbsCurve3<f64>) -> Vec<f64> {
        let (a, b) = curve.domain();
        (0..=4)
            .map(|k| {
                let p = curve.eval(a + (b - a) * f64::from(k) / 4.0);
                p.y.atan2(p.x).to_degrees()
            })
            .collect()
    }

    /// R2-A: THE TURNING-WITNESS ATTACK. A closed carrier lying
    /// EXACTLY on the circle that covers only 300°, whose five fixed
    /// samples land at 0°, 100°, 200°, 300°, 0° — four strictly
    /// positive wrapped increments (100, 100, 100, 60). If this
    /// promotes, the adopted full period is a strictly larger locus
    /// than the carrier, with a ~1e-18 "certified" residual.
    #[test]
    fn r2a_a_300_degree_carrier_that_passes_the_turning_witness() {
        let d = |x: f64| x.to_radians();
        // forward 0→100→200→300 over [0, .75], back 300→200→100→0
        // over [.75, 1].
        let curve = arc_chain(
            0.005,
            &[
                d(0.0),
                d(100.0),
                d(200.0),
                d(300.0),
                d(200.0),
                d(100.0),
                d(0.0),
            ],
            &[0.25, 0.5, 0.75, 0.75 + 1.0 / 12.0, 0.75 + 2.0 / 12.0],
        );
        println!("R2-A azimuths: {:?}", azimuths(&curve));
        let outcome = recognize(&curve, EPS_IN);
        println!("R2-A outcome: {outcome:?}");
        assert!(
            !matches!(outcome, CurveRecognition::Promoted { .. }),
            "a 300-degree carrier must not become the full circle"
        );
    }

    /// R2-B2: C6's own curve, measured. Its `third` is `TAU/6` = 60
    /// degrees, but its corner is placed at `2r` with weight 1/2 — the
    /// 120-degree construction. So the span is NOT on the circle and
    /// C6 refuses on the CERTIFICATE, never reaching the turning
    /// witness it is named for.
    #[test]
    fn r2b2_c6s_curve_is_not_on_its_circle() {
        let radius = 0.005f64;
        let on = |t: f64| Point3::new(radius * t.cos(), radius * t.sin(), 0.0);
        let corner = |t: f64| Point3::new(2.0 * radius * t.cos(), 2.0 * radius * t.sin(), 0.0);
        let third = core::f64::consts::TAU / 6.0;
        println!("C6 `third` = {} deg", third.to_degrees());
        let control = vec![
            on(0.0),
            corner(third / 2.0),
            on(third),
            corner(third / 2.0),
            on(0.0),
        ];
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
        let curve = NurbsCurve3::new(knots, control, vec![1.0, 0.5, 1.0, 0.5, 1.0]).unwrap();
        println!("C6 azimuths: {:?}", azimuths(&curve));
        // How far off the origin-centred circle the midpoint sits.
        let m = curve.eval(0.5);
        println!(
            "C6 mid |P| = {} vs r = {radius}",
            (m.x * m.x + m.y * m.y).sqrt()
        );
        println!("C6 outcome: {:?}", recognize(&curve, EPS_IN));
    }

    /// R2-B: why C6 refuses — diagnostic, to see whether the witness
    /// or something else does the work on the symmetric out-and-back.
    #[test]
    fn r2b_c6_diagnostic() {
        let d = |x: f64| x.to_radians();
        let curve = arc_chain(0.005, &[d(0.0), d(120.0), d(0.0)], &[0.5]);
        println!("R2-B azimuths: {:?}", azimuths(&curve));
        println!("R2-B outcome: {:?}", recognize(&curve, EPS_IN));
    }

    /// R2-C: CERTIFICATE SOUNDNESS. For a family of perturbed
    /// carriers, the certified residual must be an upper bound on the
    /// densely sampled true distance to the promoted circle.
    #[test]
    fn r2c_the_certified_residual_bounds_the_true_deviation() {
        let radius = 0.005f64;
        for (i, delta) in [0.0, 1e-12, 1e-11, 1e-10, 5e-10, 9e-10]
            .into_iter()
            .enumerate()
        {
            // Ellipse-ish: scale y, and wobble z, both at `delta`.
            let base = super::tests_support::rational_circle(radius);
            let mut control = base.control().to_vec();
            for (k, p) in control.iter_mut().enumerate() {
                p.y *= 1.0 + delta / radius;
                if k % 2 == 1 {
                    p.z += delta;
                }
            }
            let curve =
                NurbsCurve3::new(base.knots().clone(), control, base.weights().to_vec()).unwrap();
            let outcome = recognize(&curve, EPS_IN);
            let CurveRecognition::Promoted {
                curve:
                    Curve3::Circle {
                        center,
                        axis,
                        radius: r,
                        ..
                    },
                residual,
                ..
            } = outcome
            else {
                println!("R2-C[{i}] delta={delta:e}: stays NURBS");
                continue;
            };
            // Dense true deviation.
            let (a, b) = curve.domain();
            let mut worst = 0.0f64;
            for k in 0..=20_000 {
                let p = curve.eval(a + (b - a) * f64::from(k) / 20_000.0);
                let w = p - center;
                let h = w.dot(axis);
                let q = (w - axis * h).norm();
                worst = worst.max((q - r).hypot(h));
            }
            println!(
                "R2-C[{i}] delta={delta:e} certified={residual:e} true={worst:e} ratio={:.3}",
                worst / residual
            );
            assert!(
                worst <= residual * (1.0 + 1e-9),
                "UNSOUND: certified {residual:e} < true {worst:e}"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
pub(crate) mod tests_support {
    use super::*;
    use geom_core::spline::KnotVector;
    /// The 3x120 rational-quadratic full circle, centred at the origin.
    pub(crate) fn rational_circle(radius: f64) -> NurbsCurve3<f64> {
        let tau = core::f64::consts::TAU;
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for k in 0..7 {
            let on = k % 2 == 0;
            let theta = tau * (k as f64) / 6.0;
            let r = if on { radius } else { radius * 2.0 };
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), 0.0));
            weights.push(if on { 1.0 } else { 0.5 });
        }
        let s = 3.0f64.sqrt();
        let knots = KnotVector::clamped(
            vec![
                0.0,
                0.0,
                0.0,
                s,
                s,
                2.0 * s,
                2.0 * s,
                3.0 * s,
                3.0 * s,
                3.0 * s,
            ],
            2,
        )
        .unwrap();
        NurbsCurve3::new(knots, control, weights).unwrap()
    }
}

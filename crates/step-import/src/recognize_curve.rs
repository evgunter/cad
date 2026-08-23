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
//! * **[`crate::signed_zero`] on every DERIVED frame**
//!   (center/axis/u_ref/origin/dir), for the reason that module states;
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

use geom::{Curve3, NurbsCurve3};
use geom_core::spline::compose::{self, CurveRingData, ImplicitSurface};
use geom_core::{Point3, Vec3};

use crate::signed_zero::{plus_zero, plus_zero_point};

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
        center: plus_zero_point(center),
        axis: plus_zero(axis),
        radius,
        u_ref: plus_zero(u_ref),
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
///    Project those into the circle's plane and test them against one
///    direction, their own normalized sum.
///
///    **Why a data-derived direction is not a data-dependent schedule
///    (D9).** The direction is only ever a WITNESS: whatever it is,
///    the scan proves membership of the one region it names, and a
///    wrong guess cannot prove a false membership — it can only fail
///    to find a true one. Nothing downstream reads it. What D9
///    forbids is letting data choose how much work is done or which
///    obligations are checked, and both are fixed here: one scan per
///    knot span, every local control point, ascending.
///
///    **The incompleteness this buys, recorded.** A span hull
///    dominated by one far control point is refused even where some
///    other separating direction would have worked. Harmless on arc
///    control nets, which are near-symmetric — but it is a reason a
///    legitimate carrier can stay NURBS, and it belongs with the
///    `|Δ| ≥ π − δ` limb in the surjectivity follow-up's tightness
///    scope.
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
///
///    **In ℝ. The f64 clause** (a review executed the gap): that
///    identity has NO margin at the branch cut, and step 1 with a
///    bare open half-plane hands it none — two nearly antipodal
///    control points clear any bisector by `sin(η/2) > 0`, so a span
///    turning within an ulp of π is admitted, its computed difference
///    can land on `±π`, and the wrap moves it by a whole turn. A
///    carrier with true turning 0 then totals ~2π. The rational
///    quadratic self-limits (such a span needs corner weight
///    `sin(η/2) → 0`, which the locus certificate catches) but the
///    CUBIC form does not: degree-elevating keeps the middle weights
///    near 1/3, so a span a hair under π is perfectly conditioned. A
///    2160-case sweep forged 162 full turns out of carriers covering
///    exactly 180°.
///
///    Step 1 therefore admits a CONE of half-angle `(π − δ)/2` rather
///    than a half-plane, which bounds the true increment by `π − δ`
///    and puts the recovery a whole [`TURN_MARGIN`] away from the cut.
///    The sweep is pinned (`r2_delta::d7_…`) and goes to zero.
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
/// **The recovery margin `δ`** (radians) that makes
/// [`covers_one_full_turn`]'s step 3 true in `f64` and not only in ℝ.
///
/// A span is admitted only once its TRUE turning is bounded by
/// `π − δ`, so the sampled azimuth difference sits a whole `δ` away
/// from the `±π` branch cut where wrapping would move it by 2π.
///
/// `δ = π/6` (30°). The margin has to clear the rounding of an `atan2`
/// pair and one subtraction near `π`, which is a few ulps — the f64
/// spacing at `π` is 4.4·10⁻¹⁶ rad — so 0.52 rad is roughly fifteen
/// orders of magnitude of room. It is bounded ABOVE by the carriers
/// that must still certify: a span's admitted half-width is
/// `(π − δ)/2 = 75°`, and the widest genuine form in the corpus or in
/// these pins is the 3×120° rational quadratic, whose control azimuths
/// spread ±60° about the span bisector. So the interval of workable
/// margins is wide, `δ` sits near its middle, and nothing about the
/// choice is delicate.
const TURN_MARGIN: f64 = core::f64::consts::FRAC_PI_6;

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
        // Step 1: the span's local control points, all inside a CONE
        // of half-angle `(π − δ)/2` about one direction through the
        // centre. Membership of the open half-plane alone (half-angle
        // exactly π/2) is what the exact-arithmetic argument needs and
        // what f64 cannot cash: it admits a span turning within an ulp
        // of π, whose azimuth difference lands on the branch cut. The
        // cone is the same test with `δ` of clearance, and it is still
        // a convex set containing no direction and its opposite, so
        // every step below reads the same.
        let clearance = (TURN_MARGIN / 2.0).sin();
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
            let radial = x.hypot(y);
            // A control point AT the centre has no azimuth and would
            // put the centre in the hull; `!(… > 0.0)` also refuses a
            // NaN, here and below.
            if !(radial > 0.0) {
                return false;
            }
            // `cos((π − δ)/2) = sin(δ/2)`: the cone, in the form that
            // needs no inverse trigonometry.
            if !(x * dx + y * dy >= radial * clearance) {
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
        // Step 1 PROVES `|step| ≤ π − δ`; this asserts it. The bound is
        // relaxed by `δ/2` so legitimate rounding cannot trip it, which
        // still leaves the recovered value 15° clear of the branch cut.
        // Redundant by the derivation — and the derivation is exactly
        // the thing a review found stated in ℝ and executed in f64, so
        // it is checked rather than trusted.
        if !(step.abs() <= pi - TURN_MARGIN / 2.0) {
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
    /// **This pin can go green for the wrong reason**: its carrier
    /// once placed a 120°
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

/// R2 DELTA re-verify probes for the coverage certificate (head e5fd98eb).
#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::float_cmp,
    clippy::panic,
    clippy::print_stdout
)]
mod r2_delta {
    use super::*;
    use geom_core::spline::KnotVector;

    const EPS_IN: f64 = 1e-9;
    const R: f64 = 0.005;

    /// A closed on-circle carrier of rational-quadratic spans through
    /// `angles`, each span's corner at `r/cos(half)` with weight
    /// `cos(half)` — valid (strictly positive weight) for any span
    /// strictly under a half turn, INCLUDING one a hair under π.
    fn chain(angles: &[f64]) -> NurbsCurve3<f64> {
        let n = angles.len() - 1;
        let on = |t: f64| Point3::new(R * t.cos(), R * t.sin(), 0.0);
        let mut control = vec![on(angles[0])];
        let mut weights = vec![1.0];
        for k in 0..n {
            let (a, b) = (angles[k], angles[k + 1]);
            let half = (b - a) / 2.0;
            let c = half.cos();
            let mid = (a + b) / 2.0;
            control.push(Point3::new(R / c * mid.cos(), R / c * mid.sin(), 0.0));
            weights.push(c);
            control.push(on(b));
            weights.push(1.0);
        }
        let mut kn = vec![0.0, 0.0, 0.0];
        for k in 1..n {
            kn.push(k as f64);
            kn.push(k as f64);
        }
        kn.extend([n as f64; 3]);
        NurbsCurve3::new(KnotVector::clamped(kn, 2).unwrap(), control, weights).unwrap()
    }

    fn frame() -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
        (
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        )
    }

    /// **D1 — the ulp-scale attack on step 3.** The recovery
    /// "wrapped == true" is a REAL-arithmetic identity applied to f64.
    /// A span turning by `π − η` passes the half-plane test for every
    /// `η > 0` (its three control points span exactly `π − η`, and the
    /// corner weight `cos((π−η)/2) = sin(η/2)` stays positive), so as
    /// `η` falls below the f64 spacing at π (~4.4e-16) the computed
    /// difference can cross ±π and wrap by a whole 2π. Out-and-back
    /// over such an arc has TRUE turning 0 and covers half the circle;
    /// a single mis-wrap would put the total at ±2π and PROMOTE it.
    #[test]
    fn d1_near_pi_spans_out_and_back_must_all_refuse() {
        let (c, ax, u) = frame();
        let pi = core::f64::consts::PI;
        let mut promoted = Vec::new();
        for k in 0..64 {
            let eta = 1e-3 * 0.5f64.powi(k / 2) * if k % 2 == 0 { 1.0 } else { 0.37 };
            let top = pi - eta;
            if !(top.cos().is_finite() && (top / 2.0).cos() > 0.0) {
                continue;
            }
            let out_back = chain(&[0.0, top, 0.0]);
            let gate = covers_one_full_turn(&out_back, c, ax, u);
            let rec = matches!(
                recognize(&out_back, EPS_IN),
                CurveRecognition::Promoted { .. }
            );
            if gate || rec {
                promoted.push((eta, gate, rec));
            }
        }
        assert!(
            promoted.is_empty(),
            "an out-and-back over a near-half turn must never wind once: {promoted:?}"
        );
    }

    /// **D2 — the same ulp attack, asymmetric.** Forward `π − η`, back
    /// `−(π − η')` with a DIFFERENT `η'`, so the two spans round
    /// independently and a single mis-wrap is not cancelled by its
    /// twin. Still closed, still true turning ~0.
    #[test]
    fn d2_asymmetric_near_pi_out_and_back_must_refuse() {
        let (c, ax, u) = frame();
        let pi = core::f64::consts::PI;
        let mut bad = Vec::new();
        for a in 0..40 {
            for b in 0..8 {
                let eta = 4.0e-16 * 0.5f64.powi(a / 2) * (1.0 + 0.11 * f64::from(b));
                let top = pi - eta;
                // Down to a point a hair off the antipode, then back.
                let mid = top - 2.0e-16 * (1.0 + f64::from(b));
                if !((top / 2.0).cos() > 0.0 && (mid / 2.0).cos() > 0.0) {
                    continue;
                }
                let carrier = chain(&[0.0, top, top - mid]);
                if covers_one_full_turn(&carrier, c, ax, u) {
                    bad.push((eta, mid));
                }
            }
        }
        assert!(
            bad.is_empty(),
            "asymmetric near-π out-and-back wound once: {bad:?}"
        );
    }

    /// **D2b — is D2 EXPLOITABLE end to end?** For each carrier whose
    /// gate mis-recovers, measure the locus arm too: the true max
    /// deviation from the circle, the certified residual, and what
    /// `recognize` actually answers.
    #[test]
    fn d2b_is_the_mis_recovery_exploitable() {
        let (c, ax, u) = frame();
        let pi = core::f64::consts::PI;
        let mut fired = 0;
        for a in 0..40 {
            for b in 0..8 {
                let eta = 4.0e-16 * 0.5f64.powi(a / 2) * (1.0 + 0.11 * f64::from(b));
                let top = pi - eta;
                let mid = top - 2.0e-16 * (1.0 + f64::from(b));
                if !((top / 2.0).cos() > 0.0 && (mid / 2.0).cos() > 0.0) {
                    continue;
                }
                let carrier = chain(&[0.0, top, top - mid]);
                if !covers_one_full_turn(&carrier, c, ax, u) {
                    continue;
                }
                fired += 1;
                if fired > 3 {
                    continue;
                }
                let (d0, d1) = carrier.domain();
                let mut worst = 0.0f64;
                for k in 0..=4000 {
                    let p = carrier.eval(d0 + (d1 - d0) * f64::from(k) / 4000.0);
                    worst = worst.max((p.x.hypot(p.y) - R).abs().max(p.z.abs()));
                }
                let corner_w = (mid / 2.0).cos();
                println!(
                    "GATE-TRUE eta={eta:e} corner_weight={corner_w:e} true_dev={worst:e} rec={:?}",
                    recognize(&carrier, EPS_IN)
                );
            }
        }
        println!("D2b: {fired} carriers mis-recovered by the gate");
    }

    /// A **cubic** rational Bezier arc `a -> b` on the circle, as the
    /// degree-elevation of the rational quadratic. Its middle weights
    /// are `(1 + 2cos((b-a)/2))/3`, which stay near 1/3 even as the
    /// span approaches a HALF TURN — unlike the quadratic form, whose
    /// weight collapses to 0 there and blows the locus bound up.
    /// Returns the four (point, weight) pairs.
    fn cubic_arc(a: f64, b: f64) -> Vec<(Point3<f64>, f64)> {
        let al = (b - a) / 2.0;
        let mid = (a + b) / 2.0;
        let c = al.cos();
        // Homogeneous quadratic control in the arc's own frame.
        let h = [
            (R * al.cos(), -R * al.sin(), 1.0),
            (R, 0.0, c),
            (R * al.cos(), R * al.sin(), 1.0),
        ];
        let lerp = |p: (f64, f64, f64), q: (f64, f64, f64), t: f64| {
            (
                p.0 * (1.0 - t) + q.0 * t,
                p.1 * (1.0 - t) + q.1 * t,
                p.2 * (1.0 - t) + q.2 * t,
            )
        };
        let g = [
            h[0],
            lerp(h[0], h[1], 2.0 / 3.0),
            lerp(h[1], h[2], 1.0 / 3.0),
            h[2],
        ];
        g.iter()
            .map(|(x, y, w)| {
                let (cx, sx) = (mid.cos(), mid.sin());
                let (rx, ry) = (x * cx - y * sx, x * sx + y * cx);
                (Point3::new(rx / w, ry / w, 0.0), *w)
            })
            .collect()
    }

    /// A degree-3 clamped B-spline chaining cubic Bezier arcs.
    fn cubic_chain(angles: &[f64]) -> NurbsCurve3<f64> {
        let n = angles.len() - 1;
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for k in 0..n {
            let seg = cubic_arc(angles[k], angles[k + 1]);
            let skip = usize::from(k > 0);
            for (p, w) in seg.into_iter().skip(skip) {
                control.push(p);
                weights.push(w);
            }
        }
        let mut kn = vec![0.0; 4];
        for k in 1..n {
            kn.extend([k as f64; 3]);
        }
        kn.extend([n as f64; 4]);
        NurbsCurve3::new(KnotVector::clamped(kn, 3).unwrap(), control, weights).unwrap()
    }

    /// **D6 — the ulp mis-recovery, made EXPLOITABLE.** One span runs
    /// BACKWARD by `pi - eta` (its computed azimuth difference reaches
    /// exactly `-pi` and wraps UP by a whole turn, contributing `+pi`
    /// where the truth is `-pi`); the return leg is split into two
    /// safe half-spans contributing `+pi - eta` honestly. Total
    /// recovered `~2pi`, so the gate certifies "one turn" — while the
    /// TRUE turning is 0 and the carrier covers HALF the circle. The
    /// cubic form keeps every weight near 1/3, so the locus arm has
    /// nothing to object to.
    #[test]
    fn d6_cubic_near_pi_span_forges_a_full_turn() {
        let (c, ax, u) = frame();
        let pi = core::f64::consts::PI;
        let mut hits = Vec::new();
        for k in 0..70 {
            let eta = 1e-14 * 0.5f64.powi(k);
            let top = pi - eta;
            let carrier = cubic_chain(&[0.0, -top, -top / 2.0, 0.0]);
            let gate = covers_one_full_turn(&carrier, c, ax, u);
            let rec = recognize(&carrier, EPS_IN);
            let (d0, d1) = carrier.domain();
            let mut worst = 0.0f64;
            let mut span = (f64::MAX, f64::MIN);
            for i in 0..=4000 {
                let p = carrier.eval(d0 + (d1 - d0) * f64::from(i) / 4000.0);
                worst = worst.max((p.x.hypot(p.y) - R).abs().max(p.z.abs()));
                let az = p.y.atan2(p.x);
                span = (span.0.min(az), span.1.max(az));
            }
            if gate {
                hits.push((eta, worst, span, format!("{rec:?}")));
            }
        }
        for (eta, worst, span, rec) in hits.iter().take(4) {
            println!(
                "FORGED eta={eta:e} on_circle_dev={worst:e} azimuth_span=[{:.4},{:.4}] rad ({:.1} deg) -> {rec}",
                span.0,
                span.1,
                (span.1 - span.0).to_degrees()
            );
        }
        println!("D6: {} forged full turns", hits.len());
        assert!(
            hits.is_empty(),
            "a carrier covering half the circle certified as a full turn"
        );
    }

    /// **D6b — D6 with the near-pi span placed LAST**, so the
    /// estimator's own three samples (fractions 0, 1/4, 1/2) all fall
    /// on the forward legs and it derives the SAME +z frame the gate
    /// is then asked about. Prints every stage.
    #[test]
    fn d6b_forged_turn_in_the_estimators_own_frame() {
        let pi = core::f64::consts::PI;
        for k in [40, 44, 48, 52] {
            let eta = 1e-14 * 0.5f64.powi(k);
            let top = pi - eta;
            // forward top/2, forward top/2, then BACKWARD top.
            let carrier = cubic_chain(&[0.0, top / 2.0, top, 0.0]);
            let (a, b) = carrier.domain();
            let sample = |f: f64| carrier.eval(a + (b - a) * f);
            let (s0, s1, s2) = (sample(0.0), sample(0.25), sample(0.5));
            let normal = (s1 - s0).cross(s2 - s0);
            let axis = normal / normal.norm();
            let mut worst = 0.0f64;
            let (mut lo, mut hi) = (f64::MAX, f64::MIN);
            for i in 0..=4000 {
                let p = carrier.eval(a + (b - a) * f64::from(i) / 4000.0);
                worst = worst.max((p.x.hypot(p.y) - R).abs().max(p.z.abs()));
                let az = p.y.atan2(p.x);
                lo = lo.min(az);
                hi = hi.max(az);
            }
            let u = Vec3::new(1.0, 0.0, 0.0);
            let gate_est = covers_one_full_turn(&carrier, Point3::new(0.0, 0.0, 0.0), axis, u);
            println!(
                "eta={eta:e} est_axis=({:.2},{:.2},{:.2}) dev={worst:e} covers={:.1}deg gate={gate_est} rec={:?}",
                axis.x,
                axis.y,
                axis.z,
                (hi - lo).to_degrees(),
                recognize(&carrier, EPS_IN)
            );
        }
    }

    /// **D6c — which arm actually refuses D6b?**
    #[test]
    fn d6c_which_arm_refuses() {
        let pi = core::f64::consts::PI;
        let eta = 1e-14 * 0.5f64.powi(48);
        for (tag, angles) in [
            ("forged-half", vec![0.0, (pi - eta) / 2.0, pi - eta, 0.0]),
            (
                "honest-full",
                vec![0.0, pi / 2.0, pi, 3.0 * pi / 2.0, 2.0 * pi],
            ),
        ] {
            let carrier = cubic_chain(&angles);
            let (a, b) = carrier.domain();
            let sample = |f: f64| carrier.eval(a + (b - a) * f);
            let (s0, s1, s2) = (sample(0.0), sample(0.25), sample(0.5));
            let normal = (s1 - s0).cross(s2 - s0);
            let axis = normal / normal.norm();
            let uu = s1 - s0;
            let vv = s2 - s0;
            let uxv = uu.cross(vv);
            let center = s0
                + (uxv.cross(uu) * vv.norm_squared() + vv.cross(uxv) * uu.norm_squared())
                    / (2.0 * uxv.norm_squared());
            let radius = (s0 - center).norm();
            let u_ref = (s0 - center) / radius;
            let ps = composite_sup(
                &carrier,
                &ImplicitSurface::Plane {
                    point: [center.x, center.y, center.z],
                    normal: [axis.x, axis.y, axis.z],
                },
            );
            let ss = composite_sup(
                &carrier,
                &ImplicitSurface::Sphere {
                    center: [center.x, center.y, center.z],
                    radius,
                },
            );
            let dp = ps.abs();
            let ds = ss.abs() / radius;
            let inner = (radius - ds).powi(2) - dp.powi(2);
            let lower = if inner > 0.0 { inner.sqrt() } else { 0.0 };
            let residual = ds.max(radius - lower).hypot(dp);
            println!(
                "{tag}: r={radius:e} plane_sup={ps:e} sphere_sup={ss:e} residual={residual:e} \
                 locus_ok={} gate={}",
                residual <= EPS_IN,
                covers_one_full_turn(&carrier, center, axis, u_ref)
            );
        }
    }

    /// **D7 — the decisive sweep.** The forged construction, rotated
    /// and re-scaled over thousands of cases, run through the WHOLE
    /// pipeline. Any `Promoted` here is a half-covering carrier that
    /// became a full circle.
    #[test]
    fn d7_forged_half_turn_sweep_through_recognize() {
        let pi = core::f64::consts::PI;
        let mut promoted = 0usize;
        let mut gate_true = 0usize;
        let mut cases = 0usize;
        for ki in 0..90 {
            let eta = 1e-13 * 0.5f64.powi(ki);
            let top = pi - eta;
            for ri in 0..24 {
                let phi = core::f64::consts::TAU * f64::from(ri) / 24.0 + 1e-3 * f64::from(ri);
                let angles = [phi, phi + top / 2.0, phi + top, phi];
                let carrier = cubic_chain(&angles);
                cases += 1;
                let (a, b) = carrier.domain();
                let sample = |f: f64| carrier.eval(a + (b - a) * f);
                let (s0, s1, s2) = (sample(0.0), sample(0.25), sample(0.5));
                let normal = (s1 - s0).cross(s2 - s0);
                if normal.norm() == 0.0 {
                    continue;
                }
                let axis = normal / normal.norm();
                let uu = s1 - s0;
                let vv = s2 - s0;
                let uxv = uu.cross(vv);
                let center = s0
                    + (uxv.cross(uu) * vv.norm_squared() + vv.cross(uxv) * uu.norm_squared())
                        / (2.0 * uxv.norm_squared());
                let radius = (s0 - center).norm();
                if covers_one_full_turn(&carrier, center, axis, (s0 - center) / radius) {
                    gate_true += 1;
                }
                if let CurveRecognition::Promoted { residual, .. } = recognize(&carrier, EPS_IN) {
                    promoted += 1;
                    if promoted <= 3 {
                        println!("FALSE PROMOTION eta={eta:e} phi={phi:.4} residual={residual:e}");
                    }
                }
            }
        }
        println!(
            "D7: {cases} half-covering carriers; gate said one-turn on {gate_true}; \
             recognize promoted {promoted}"
        );
        assert_eq!(
            promoted, 0,
            "a half-covering carrier promoted to a full circle"
        );
    }

    /// **D3 — the hull straddling the centre must refuse**, and the
    /// refusal must be the half-plane arm rather than an accident:
    /// the exact circle read against a centre displaced INTO the
    /// carrier's own annulus makes span hulls contain the origin.
    #[test]
    fn d3_a_span_hull_straddling_the_centre_refuses() {
        let (_, ax, u) = frame();
        let d = f64::to_radians;
        let full = chain(&[d(0.0), d(120.0), d(240.0), d(360.0)]);
        assert!(covers_one_full_turn(
            &full,
            Point3::new(0.0, 0.0, 0.0),
            ax,
            u
        ));
        for shift in [R * 0.5, R, R * 1.5, R * 2.0, R * 4.0] {
            let off = Point3::new(shift, 0.0, 0.0);
            assert!(
                !covers_one_full_turn(&full, off, ax, u),
                "a centre displaced by {shift:e} leaves span hulls straddling it"
            );
        }
    }

    /// **D4 — triple winding**, the k ≥ 2 arm beyond C7's k = 2, plus
    /// the reversed single turn (w = −1) read against a +z axis.
    #[test]
    fn d4_triple_wound_and_reversed_refuse() {
        let (c, ax, u) = frame();
        let d = f64::to_radians;
        let thrice: Vec<f64> = (0..=9).map(|k| d(120.0 * f64::from(k))).collect();
        assert!(
            !covers_one_full_turn(&chain(&thrice), c, ax, u),
            "three turns"
        );
        let back: Vec<f64> = (0..=3).map(|k| d(-120.0 * f64::from(k))).collect();
        assert!(
            !covers_one_full_turn(&chain(&back), c, ax, u),
            "a carrier winding against the read axis"
        );
    }

    /// **D5 — the gate is not merely restrictive**: the shipped class
    /// (the 3×120° form, and finer uniform subdivisions of one turn)
    /// still certifies, so "refuses everything" is not how D1–D4 pass.
    #[test]
    fn d5_genuine_full_turns_still_certify() {
        let (c, ax, u) = frame();
        for spans in [3usize, 4, 5, 6, 8, 12, 30] {
            let angles: Vec<f64> = (0..=spans)
                .map(|k| core::f64::consts::TAU * (k as f64) / (spans as f64))
                .collect();
            let full = chain(&angles);
            assert!(
                covers_one_full_turn(&full, c, ax, u),
                "{spans} uniform spans is one turn"
            );
            assert!(
                matches!(recognize(&full, EPS_IN), CurveRecognition::Promoted { .. }),
                "{spans}-span circle must promote"
            );
        }
    }
}

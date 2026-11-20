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
//! # Kind scope
//!
//! [`PromotedCurveKind::Circle`] — the **full circle** and the
//! **circular arc**, both stated as rational quadratics, one span or
//! many joined at double knots — and [`PromotedCurveKind::Line`], the
//! **chord segment** class. A carrier whose ends meet within ε_in is
//! closed: the circle limb's full-period class, and a carrier the line
//! limb refuses (a chord inside the budget determines no direction).
//! An open carrier may reach both limbs; [`recognize`] states the
//! order. Ellipse and helix are NAMED EXCLUSIONS with a filed
//! follow-up — no dead per-kind code here.
//!
//! **A promoted carrier has no knots**, so the C⁰-knot split at the
//! import door (`docs/DESIGN.md` `[topology]`) has nothing to cut on it:
//! recognition runs on the WHOLE carrier, at its one construction
//! site, before any normalization reads it.
//!
//! # The line's certificate (D9-clean by construction: no sampling,
//! no schedule)
//!
//! [`geom_core::spline::compose`]'s ring-coefficient implicit
//! composites give the line an EXACT whole-domain locus certificate:
//! `implicit_composite(curve, f)` builds `f ∘ C` in rational Bernstein
//! form with interval coefficients, and
//! [`compose::CompositeForm::sup_bound`] reads a certified sup off its
//! coefficient hulls. Data in, bounds out: nothing is evaluated, no
//! schedule exists to be data-dependent. A refusal anywhere yields
//! `NaN`, which fails every `≤ ε` comparison (D4 ¶2).
//!
//! * **INV-C3 (zero-radius cylinder → distance to the LINE).** The
//!   cylinder composite is `(|Q|² − (Q·â)² − r²)` with `Q = P − p₀`
//!   and `â` the unit axis; at `r = 0` (an exact certification value —
//!   `compose` forms `r²` as `Interval::point(0).sqr()`, the zero
//!   interval, and refuses no radius) it is exactly `dist(P, line)²`,
//!   meters². Conversion: `δ_line = √S`, rooted outward.
//! * **INV-C4 (coverage — the locus is the WHOLE segment).** The
//!   projection `t(P) = (P − p₀)·d̂` is continuous from
//!   `t(start) = 0` to `t(end) = ℓ`, so by the intermediate value
//!   theorem its image contains `[0, ℓ]` and the carrier's locus is
//!   the whole segment (thickened by at most the residual), never a
//!   proper sub-segment. The segment obligation carries NO separate
//!   excursion fold: INV-C5's hull (below) meters an overshooting
//!   net, and it dominates — on clamped knots every Greville
//!   abscissa lies in the domain, so `ℓ(ξᵢ)` is on the chord and a
//!   control `o` past either end is at least `o` from it.
//! * **INV-C5 (the line's MAP obligation).** Promotion replaces the
//!   carrier as a parameterized MAP, not only as a locus: every
//!   downstream door — the chart-image certify schedules above all —
//!   compares `C(t)` against its description pointwise, so a promotion
//!   that kept the locus but re-timed the interior would strand the
//!   edge at adoption (measured: a nonuniform loft's straight seam,
//!   whose affine `Line` no iso image can reproduce). With one
//!   constant weight the polynomial basis reproduces affine functions
//!   of the parameter on the Greville abscissae `ξᵢ` (linear
//!   precision), so with `ℓ` the affine map taking the knot domain
//!   onto the chord,
//!   `|C(t) − ℓ(t)| = |Σ Nᵢ(t)(cᵢ − ℓ(ξᵢ))| ≤ maxᵢ |cᵢ − ℓ(ξᵢ)|` —
//!   the seam-class chart limb's own hull, transposed to recognition.
//!   A rational carrier has no linear-precision fact and refuses the
//!   certificate (stays NURBS silently). The hull also carries the
//!   segment obligation (INV-C4 above), so the reported residual is
//!   `max(δ_line, hull)` and the budget decides at whichever of the
//!   locus and map obligations binds.
//!
//! # The circle's certificate: the MAP (INV-C6)
//!
//! **What a promoted circle promises.** The promoted carrier is a
//! [`Curve3::Circle`] parameterized by its angle `θ` from `u_ref`, and
//! every consumer reads it there. Promotion is sound only if the
//! carrier, as a MAP, is that circle under a reparameterization the
//! carrier itself determines: a continuous, strictly increasing `θ(t)`
//! with `|C(t) − circle(θ(t))| ≤ residual` for every `t`. A locus
//! certificate cannot say this — a carrier on the circle that doubles
//! back, winds twice, or runs ahead of its own sub-arcs has the
//! circle's locus and not its traversal.
//!
//! **The law `θ(t)`** is the one the carrier's own structure states:
//! the rational-quadratic arc law of its spans. Span `j` is a rational
//! quadratic Bézier `(c₀, c₁, c₂)` at weights `(w₀, w₁, w₂)` on a local
//! parameter `s` affine in `t` ([`conic_spans`] checks that
//! structure). Write `e_a`, `e_b` for its joints' unit directions from
//! the centre in the circle's plane, `h` the turn between them and
//! `α = h/2`. The **canonical sub-arc** `Ĉ_c` has control points
//! `ĉ₀ = o + R·e_a`, `ĉ₁ = o + R·(e_a + e_b)/(2cos²α)` (the tangent
//! intersection) and `ĉ₂ = o + R·e_b`, at weights
//! `(w₀, cos α·√(w₀w₂), w₂)`: it lies on the circle, and its angle is
//! `φ = 2·arctan((2σ − 1)·tan(h/4))` from the sub-arc's middle, `σ`
//! the Möbius change of `s` its end weights state — continuous and
//! strictly increasing from `e_a` to `e_b` ([`geom_brep::Pcurve::IsoArc`]
//! derives the same law, and the arc rim is the consumer that reads it
//! off a chart's row). Consecutive sub-arcs share their joint, so the
//! law is continuous across spans, and it is strictly increasing on the
//! whole domain.
//!
//! **The certificate**, two inequalities per span, each exact:
//!
//! 1. `Ĉ_w` — the same control points at the CARRIER's weights. The
//!    two curves share weights, so `C − Ĉ_w = Σ Rᵢ(s)·(cᵢ − ĉᵢ)` with
//!    `Rᵢ ≥ 0` summing to one (positive weights), and
//!    `|C(s) − Ĉ_w(s)| ≤ maxᵢ |cᵢ − ĉᵢ|` — the control-difference hull,
//!    the arc-rim certificate's middle term transposed to recognition
//!    as INV-C5's Greville hull is the seam limb's.
//! 2. `|Ĉ_w(s) − Ĉ_c(s)|` is what a weight off `cos α` costs: the two
//!    differ only in the normalized interior weight, and
//!    [`geom_brep::pcurve_cache::arc_rim_weight_miss`] bounds that miss
//!    in closed form (the arc rim's third term).
//!
//! So `|C(t) − circle(θ(t))| ≤ max over spans of (hull + miss)`, which
//! is the reported residual. Everything a promotion needs follows from
//! that one statement:
//!
//! * **locus** — every carrier point is within the residual of the
//!   circle;
//! * **orientation and single traversal** — `θ` is strictly
//!   increasing, so each span is required to turn FORWARD about the
//!   estimator's axis (which winds with the increasing parameter),
//!   by an arc the file reads apart from zero;
//! * **coverage** — `θ` runs continuously from `0` (the promoted
//!   `u_ref` is the first joint's direction) to the turns' sum. A
//!   closed carrier's canonical form is closed exactly (its last joint
//!   IS its first, so the closure gap is metered by the hull at the
//!   last control point), its sum is a whole number of turns, and it
//!   must be ONE: the full period the self-loop adopts. An arc's sum
//!   must stay under one turn; its ends are the canonical form's,
//!   within the residual of the carrier's end points, and the adopted
//!   edge trims the circle at its vertices
//!   ([`crate::geometry::endpoint_params`]'s increasing-parameter arc),
//!   which the adoption gates pin to the carrier's ends.
//!
//! A doubled-back carrier has a span turning backward; a twice-wound
//! one sums to two turns; a near-half-turn span exceeds
//! [`SPAN_MARGIN`]; a re-timed one — its interior weights off its
//! sub-arcs, so that it hugs the circle while running ahead of the law
//! — pays the miss. Each refuses, and stays NURBS silently.
//!
//! **What is computed, and what is treated as exact.** The estimator's
//! frame and the canonical control points are closed forms in `f64`,
//! posed well-conditioned (the tangent point's divisor is at least
//! `sin(δ/2)`, [`SPAN_MARGIN`]) and then treated as exact
//! (`docs/DESIGN.md` `[simple.conditioned]`), as the arc-rim certificate
//! treats its own `Ĉ`. The comparison against them is CERTIFIED: each
//! distance, the miss, and their sum are read at `Interval` and rounded
//! outward, so the residual is never an ulp on the unsafe side.
//! Rational carriers are first-class (the weights are part of the
//! form), and nothing is sampled.
//!
//! **Its incompleteness, recorded.** A circle stated at another degree
//! (a degree-elevated cubic), or with single interior knots, has no
//! canonical form here and stays NURBS, as does a span turning more
//! than 150°.
//!
//! # Estimators (D9-clean: closed form, fixed evaluation order)
//!
//! * **Line**: the CHORD from the first control point to the last —
//!   no samples at all (a clamped carrier interpolates both), `dir`
//!   running start → end so the derived parameter interval keeps the
//!   carrier's orientation. A wrong chord fails the certificate and
//!   the carrier stays NURBS, like every estimator here.
//! * **Circle**: three samples of the carrier at the FIXED domain
//!   fractions 0, ¼, ½ (fixed schedule, data-independent — D9); the
//!   plane through them oriented by `(s₁−s₀) × (s₂−s₀)`, which is the
//!   INCREASING-PARAMETER winding, so the promoted circle's `axis`
//!   means what the carrier's direction meant; the exact circumcenter
//!   solve for `center` and `radius` (the surface recognizer's, in the
//!   samples' own plane). The certificate sets `u_ref` to the first
//!   joint's direction, so the promoted seam sits at the carrier's
//!   start point and the derived parameter interval starts at `θ = 0`.
//!
//! # Conditioning (D7's typed ambiguity)
//!
//! The circle estimator carries the surface cylinder estimator's
//! margin trilean: three samples collinear within ε_in determine no
//! finite radius, which is [`CurveRecognition::IllConditioned`] when
//! the line limb does not take the carrier either. No call site
//! ESCALATES it today — unlike the surface case there is no gate that
//! needs a curve promotion to import at all — so an ill-conditioned
//! carrier stays NURBS exactly like a refuted one. The tri-state is
//! kept because the distinction is real and the escalation site is the
//! thing that is missing, not the class.

use geom::{Curve3, NurbsCurve3};
use geom_core::interval::certification::Certification;
use geom_core::spline::compose::{self, CurveCertData, ImplicitSurface};
use geom_core::{Interval, Point3, Vec3};

use crate::interpret::{
    FileBand, Read, STEP_RECOGNIZED_CLOSURE, STEP_RECOGNIZED_CONDITIONING, STEP_RECOGNIZED_RESIDUAL,
};
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
        /// The certified residual sup (METRES — module docs INV-C3/C5 for
        /// a line, INV-C6 for a circle).
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

/// Tests `curve` for promotion at the file's band (a lane choice: Zero
/// promotes)
/// (module docs: kind, estimator, certificate).
///
/// The circle is tried first and the line second, a fixed order. The
/// closed class is the circle's alone (the line limb refuses a chord
/// inside the budget); an open carrier may reach both, and a carrier
/// certifying as both is canonicalization rather than ambiguity — both
/// analytic curves agree with the carrier, hence with each other,
/// within 2·ε_in everywhere on it (the surface recognizer's rule).
pub(crate) fn recognize(curve: &NurbsCurve3<f64>, file: FileBand) -> CurveRecognition {
    let line = || {
        try_line(curve, file).map(|(line, residual)| CurveRecognition::Promoted {
            curve: line,
            residual,
            kind: PromotedCurveKind::Line,
        })
    };
    match try_circle(curve, file) {
        Ok(Some((circle, residual))) => CurveRecognition::Promoted {
            curve: circle,
            residual,
            kind: PromotedCurveKind::Circle,
        },
        Ok(None) => line().unwrap_or(CurveRecognition::StaysNurbs),
        // Collinear estimator samples on an open carrier are the line's
        // territory, so the line limb answers first; only a carrier it
        // refuses too is the circle estimator's typed ambiguity.
        Err(margin) => line().unwrap_or(CurveRecognition::IllConditioned {
            kind: PromotedCurveKind::Circle,
            margin,
        }),
    }
}

/// The certified sup of `|f ∘ C|` over the whole domain, in the
/// composite's OWN units (module docs' scaling conventions). `NaN` on
/// every structural refusal and every refused hull, which certifies nothing.
fn composite_sup(curve: &NurbsCurve3<f64>, surface: &ImplicitSurface) -> f64 {
    let coords = curve.certified_coords();
    let Ok(data) = CurveCertData::new(curve.knots_f64_interim(), curve.weights(), &coords) else {
        return f64::NAN;
    };
    match compose::implicit_composite(&data, surface) {
        Ok(form) => form.sup_bound(),
        Err(_) => f64::NAN,
    }
}

// `!(a < b)` forms below are deliberate, NaN-catching negations: a
// NaN quantity must REFUSE, and the positive form would silently
// accept it (the file's standing convention).
#[allow(clippy::neg_cmp_op_on_partial_ord)]
/// The line candidate (module docs; the locus certificate INV-C3,
/// coverage INV-C4, and the map obligation INV-C5, whose hull also
/// carries the segment fold — all read off ring coefficient hulls
/// and control values, no sampling anywhere). `None` is a refuted
/// certificate or an out-of-scope (closed-at-ε, or rational —
/// INV-C5) carrier; there is no conditioning trilean here because
/// there is no estimate to condition — the chord IS the candidate,
/// and a chord inside the budget is the closed class, which is the
/// circle limb's.
pub(crate) fn try_line(curve: &NurbsCurve3<f64>, file: FileBand) -> Option<(Curve3<f64>, f64)> {
    let control = curve.control();
    let (first, last) = (control.first()?, control.last()?);
    let chord = *last - *first;
    // The complement of the circle limb's closure gate (module docs,
    // "Kind scope"): ends that read Zero at the file's band determine
    // no direction, and that carrier is the closed class
    // anyway, so it can never promote to a line.
    let (dir, _) = chord
        .checked_direction()
        .ok()
        .filter(|&(_, n)| file.read(STEP_RECOGNIZED_CONDITIONING, n) != Read::Zero)?;
    // INV-C3: the zero-radius cylinder composite is `dist(P, line)²`
    // over the whole domain — meters², exact, whole-domain, no
    // schedule. NaN (a refused hull, or a structural refusal) fails the
    // budget comparison below (D4 ¶2).
    let sup = composite_sup(
        curve,
        &ImplicitSurface::Cylinder {
            point: first.to_array(),
            axis: chord.to_array(),
            radius: 0.0,
        },
    );
    // Rooted outward: `√` to nearest can land below the true bound.
    let delta_line = Interval::point(sup.abs()).sqrt().mag();
    // Coverage of the whole segment is INV-C4's intermediate-value
    // argument (module docs); the segment obligation itself rides
    // INV-C5's hull below, which dominates any control excursion on
    // clamped knots.
    // INV-C5: promotion replaces the carrier as a MAP, not only as a
    // locus — downstream doors compare `C(t)` against chart images
    // pointwise, so the affine chord map must be within budget of the
    // carrier's own parameterization or the promoted edge is
    // unadoptable. With one constant weight, which cancels, the
    // polynomial basis reproduces affine maps on the Greville abscissae
    // (linear precision), so the control-vs-affine deviations bound the
    // map deviation over the whole domain (module docs); a rational
    // carrier has no linear-precision fact and refuses the certificate
    // silently. Non-finite refuses.
    let degree = curve.knots_f64_interim().degree();
    if degree == 0 || curve.constant_weight().is_none() {
        return None;
    }
    let (d0, d1) = curve.domain();
    let span = d1 - d0;
    if !(span > 0.0) || !span.is_finite() {
        return None;
    }
    let flat = curve.knots_f64_interim().knots();
    let mut hull = 0.0f64;
    for (i, p) in control.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let xi = flat[i + 1..=i + degree].iter().sum::<f64>() / degree as f64;
        let dev = distance_sup(*p, *first + chord * ((xi - d0) / span));
        if !dev.is_finite() {
            return None;
        }
        hull = hull.max(dev);
    }
    // `f64::max` drops a NaN, so a refused composite refuses here,
    // before the fold could read it as a zero.
    if delta_line.is_nan() {
        return None;
    }
    let residual = delta_line.max(hull);
    if file.read(STEP_RECOGNIZED_RESIDUAL, residual) != Read::Zero {
        return None;
    }
    let line = Curve3::line(plus_zero_point(*first), plus_zero(dir));
    Some((line, residual))
}

/// The circle candidate (module docs: the estimator, then the map
/// certificate INV-C6, which carries locus, coverage and orientation
/// with it). `Err(margin)` is the estimator's conditioning refusal;
/// `Ok(None)` a carrier outside the rational-quadratic arc structure or
/// a refuted certificate.
pub(crate) fn try_circle(
    curve: &NurbsCurve3<f64>,
    file: FileBand,
) -> Result<Option<(Curve3<f64>, f64)>, f64> {
    let Some(spans) = conic_spans(curve) else {
        return Ok(None);
    };
    let control = curve.control();
    let (Some(first), Some(last)) = (control.first(), control.last()) else {
        return Ok(None);
    };
    // The closure gate: a carrier whose ends read Zero is the full
    // period, any other an arc trimmed by its own end points.
    let closed = file.read(STEP_RECOGNIZED_CLOSURE, (*last - *first).norm()) == Read::Zero;
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
    let Some((chord_dir, _)) = chord.checked_direction().ok() else {
        return Ok(None);
    };
    let off = geom::Line {
        origin: s0,
        dir: chord_dir,
    }
    .offset(s1);
    let sagitta = off.norm();
    if !sagitta.is_finite() {
        return Ok(None);
    }
    if file.read(STEP_RECOGNIZED_CONDITIONING, sagitta) == Read::Zero {
        return Err(sagitta);
    }
    // The plane through the three samples, oriented by the
    // INCREASING-PARAMETER winding.
    let normal = (s1 - s0).cross(s2 - s0);
    let Some((axis, _)) = normal.checked_direction().ok() else {
        return Ok(None);
    };
    // The exact circumcenter of three coplanar points (the surface
    // recognizer's closed form).
    let u = s1 - s0;
    let v = s2 - s0;
    let uxv = u.cross(v);
    let denom = 2.0 * uxv.norm_squared();
    let center = s0 + (uxv.cross(u) * v.norm_squared() + v.cross(uxv) * u.norm_squared()) / denom;
    let radius = (s0 - center).norm();
    if !(radius.is_finite() && file.read(STEP_RECOGNIZED_CONDITIONING, radius) != Read::Zero) {
        return Ok(None);
    }
    let Some((u_ref, residual)) = map_certificate(curve, spans, center, axis, radius, closed, file)
    else {
        return Ok(None);
    };
    if file.read(STEP_RECOGNIZED_RESIDUAL, residual) != Read::Zero {
        return Ok(None);
    }
    let circle = Curve3::circle(
        plus_zero_point(center),
        plus_zero(axis),
        radius,
        plus_zero(u_ref),
    );
    Ok(Some((circle, residual)))
}

/// **The rational-quadratic arc structure** INV-C6 reads, checked as
/// exact `f64` structure: degree 2, clamped, every interior knot
/// DOUBLE with a nonempty span either side, so span `j` is the rational
/// quadratic Bézier on control points `2j, 2j + 1, 2j + 2` and its
/// local parameter is affine in the carrier's. The span count, or
/// `None` for any other structure (a single interior knot included:
/// its spans' Bézier nets are not the carrier's own control points).
fn conic_spans(curve: &NurbsCurve3<f64>) -> Option<usize> {
    let kv = curve.knots_f64_interim();
    let flat = kv.knots();
    let count = curve.control().len();
    if kv.degree() != 2 || count < 3 || count.is_multiple_of(2) || flat.len() != count + 3 {
        return None;
    }
    let spans = (count - 1) / 2;
    // The breaks: `flat[2]`, each interior pair, `flat[count]`.
    let clamped = flat[0] == flat[2] && flat[count] == flat[count + 2];
    let paired = (0..spans - 1).all(|k| flat[3 + 2 * k] == flat[4 + 2 * k]);
    let ascending = (0..spans).all(|k| flat[2 + 2 * k] < flat[3 + 2 * k]);
    (clamped && paired && ascending).then_some(spans)
}

/// **The sub-arc conditioning margin `δ`** (radians): INV-C6 admits a
/// span only where its canonical sub-arc turns by at most `π − δ`, so
/// the tangent point `R/cos(h/2)` out from the centre is read through a
/// divisor of at least `sin(δ/2)` and the construction is posed
/// well-conditioned where it is decided.
///
/// `δ = π/6`: a span's admitted turn is at most 150°. The widest
/// genuine form in the corpus or in these pins is the 3×120° full
/// circle, whose divisor is `cos 60° = ½`; at 150° it is `0.26`, and a
/// span near a half turn — whose weight collapses towards zero and
/// whose tangent point runs off to infinity — is refused rather than
/// read through a vanishing divisor.
const SPAN_MARGIN: f64 = core::f64::consts::FRAC_PI_6;

/// **INV-C6, the circle's map certificate** (module docs): the
/// carrier against the circle re-expressed in its own spline space,
/// span by span. `Some((u_ref, residual))` — the promoted circle's
/// reference direction (the carrier's start) and the certified sup of
/// `|C(t) − circle(θ(t))|` over the whole domain, in metres, rounded
/// outward — where every span turns forward and the turns sum to one
/// full turn (`closed`) or to less than one (an arc). `None` on every
/// path that cannot tell.
// `!(x > 0.0)` and `!(a >= b)` are deliberate, NaN-catching negations:
// a poisoned coordinate must REFUSE, and the positive form would
// silently accept it.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
fn map_certificate(
    curve: &NurbsCurve3<f64>,
    spans: usize,
    center: Point3<f64>,
    axis: Vec3<f64>,
    radius: f64,
    closed: bool,
    file: FileBand,
) -> Option<(Vec3<f64>, f64)> {
    let control = curve.control();
    let weights = curve.weights();
    // The joints' directions in the circle's plane: each sub-arc of the
    // canonical form starts where the carrier's own joint projects.
    let mut joints = (0..=spans)
        .map(|k| {
            let q = control[2 * k] - center;
            (q - axis * q.dot(axis))
                .checked_direction()
                .ok()
                .map(|(e, _)| e)
        })
        .collect::<Option<Vec<Vec3<f64>>>>()?;
    // A closed carrier's canonical form closes exactly: its last joint
    // IS its first, so the turns sum to whole turns and the closure gap
    // is metered by the hull at the last control point.
    if closed {
        joints[spans] = joints[0];
    }
    let clearance = (SPAN_MARGIN / 2.0).sin();
    let mut residual = 0.0f64;
    let mut turn = 0.0f64;
    for j in 0..spans {
        let (ea, eb) = (joints[j], joints[j + 1]);
        let (sin_h, cos_h) = (ea.cross(eb).dot(axis), ea.dot(eb));
        // Forward about the axis, by an arc the file reads apart from
        // zero: a span that runs backward, or stands still, has no
        // canonical sub-arc.
        if !(sin_h > 0.0) || file.read(STEP_RECOGNIZED_CONDITIONING, radius * sin_h) != Read::Apart
        {
            return None;
        }
        let sum = ea + eb;
        let cos_half = sum.norm() / 2.0;
        let sin_half = (eb - ea).norm() / 2.0;
        if !(cos_half >= clearance) {
            return None;
        }
        // The canonical sub-arc's control points: its two ends on the
        // circle, and the tangent intersection `R/cos(h/2)` out along
        // the bisector, `(ea + eb)·R/(2cos²(h/2))` since
        // `|ea + eb| = 2cos(h/2)`.
        let canonical = [
            center + ea * radius,
            center + sum * (radius / (2.0 * cos_half.powi(2))),
            center + eb * radius,
        ];
        let hull = canonical
            .iter()
            .zip(&control[2 * j..=2 * j + 2])
            .map(|(c, p)| distance_sup(*p, *c))
            .fold(0.0f64, |m, d| if d > m || d.is_nan() { d } else { m });
        let w = geom_brep::pcurve_cache::arc_rim_normal_weight(
            Interval::point(weights[2 * j]),
            Interval::point(weights[2 * j + 1]),
            Interval::point(weights[2 * j + 2]),
        );
        let miss = geom_brep::pcurve_cache::arc_rim_weight_miss(
            Interval::point(radius),
            Interval::point(sin_half),
            Interval::point(cos_half),
            w,
        );
        let bound = (Interval::point(hull) + miss).mag();
        if !bound.is_finite() {
            return None;
        }
        residual = residual.max(bound);
        turn += sin_h.atan2(cos_h);
    }
    let pi = core::f64::consts::PI;
    // Every span turns by less than π, so the sum is the canonical
    // law's true total; a closed form's is a whole number of turns and
    // must be ONE, an arc's must stay short of one.
    let once = if closed {
        turn > pi && turn < 3.0 * pi
    } else {
        turn < 2.0 * pi
    };
    once.then_some((joints[0], residual))
}

/// An upper bound on `|p − q|`, rounded outward: the certified sups'
/// one metre reading.
fn distance_sup(p: Point3<f64>, q: Point3<f64>) -> f64 {
    let d = |a: f64, b: f64| (Interval::point(a) - Interval::point(b)).sqr();
    (d(p.x, q.x) + d(p.y, q.y) + d(p.z, q.z)).sqrt().mag()
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
                Curve3::Circle(geom::Circle {
                    frame:
                        geom::Frame {
                            origin: center,
                            axis,
                            u_ref,
                        },
                    radius,
                }),
            residual,
            kind,
        } = recognize(&curve, crate::interpret::FileBand::test(EPS_IN))
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
            curve:
                Curve3::Circle(geom::Circle {
                    frame: geom::Frame { axis, .. },
                    ..
                }),
            ..
        } = recognize(
            &rational_circle(0.005, true),
            crate::interpret::FileBand::test(EPS_IN),
        )
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
        curve = NurbsCurve3::new(
            curve.knots_f64_interim().clone(),
            control,
            curve.weights().to_vec(),
        )
        .unwrap();
        assert!(
            matches!(
                recognize(&curve, crate::interpret::FileBand::test(EPS_IN)),
                CurveRecognition::StaysNurbs
            ),
            "a dented circle is not a circle"
        );
    }

    /// C4: an OPEN arc promotes to its circle, and the adopted edge is
    /// the arc: the circle's seam sits at the carrier's start, so
    /// [`crate::geometry::endpoint_params`] trims it to the 240° the
    /// carrier covers, in the carrier's direction.
    #[test]
    fn c4_an_open_arc_promotes_and_trims_to_its_arc() {
        let full = rational_circle(0.005, false);
        // The first two spans only: a 240° arc, ends distinct.
        let control = full.control()[..5].to_vec();
        let weights = full.weights()[..5].to_vec();
        let s = 3.0f64.sqrt();
        let knots =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, s, s, 2.0 * s, 2.0 * s, 2.0 * s], 2).unwrap();
        let arc = NurbsCurve3::new(knots, control.clone(), weights).unwrap();
        let CurveRecognition::Promoted {
            curve: circle @ Curve3::Circle(geom::Circle { radius, .. }),
            residual,
            kind: PromotedCurveKind::Circle,
        } = recognize(&arc, crate::interpret::FileBand::test(EPS_IN))
        else {
            panic!("an exact rational-quadratic arc must promote");
        };
        assert!(residual < 1e-15, "an exact arc's residual: {residual:e}");
        assert!((radius - 0.005).abs() < 1e-15, "radius {radius:e}");
        let (t0, t1) =
            crate::geometry::endpoint_params(1, &circle, control[0], control[4], false).unwrap();
        let third = core::f64::consts::TAU / 3.0;
        assert!(t0.abs() < 1e-12, "the arc starts at the seam: {t0:e}");
        assert!(
            (t1 - t0 - 2.0 * third).abs() < 1e-12,
            "the arc spans 240°: {:e}",
            t1 - t0
        );
    }

    /// C4b: an open arc that DOUBLES BACK stays NURBS — its locus is a
    /// quarter of the circle, but its second span turns backward, so no
    /// increasing angle law traverses it.
    #[test]
    fn c4b_an_open_arc_that_doubles_back_stays_nurbs() {
        let d = f64::to_radians;
        let back = arc_chain(0.005, &[d(0.0), d(90.0), d(45.0)]);
        assert!(matches!(
            recognize(&back, crate::interpret::FileBand::test(EPS_IN)),
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
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
        let curve = NurbsCurve3::new(knots, control, vec![1.0; 5]).unwrap();
        let CurveRecognition::IllConditioned { kind, margin } =
            recognize(&curve, crate::interpret::FileBand::test(EPS_IN))
        else {
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

    /// C6: a doubled-back CLOSED carrier stays NURBS. It lies exactly
    /// on the circle — asserted densely here, so a locus refusal
    /// cannot stand in for the one this pin is about — and covers a
    /// quarter of it; its third and fourth spans turn backward.
    #[test]
    fn c6_a_doubled_back_closed_carrier_stays_nurbs() {
        let radius = 0.005f64;
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
            !matches!(
                recognize(&back, crate::interpret::FileBand::test(EPS_IN)),
                CurveRecognition::Promoted { .. }
            ),
            "a carrier covering a quarter of the circle must not become the circle"
        );
        // The refusal is not vacuous: the same construction, once round.
        let full = arc_chain(radius, &[d(0.0), d(120.0), d(240.0), d(360.0)]);
        assert!(matches!(
            recognize(&full, crate::interpret::FileBand::test(EPS_IN)),
            CurveRecognition::Promoted { .. }
        ));
    }

    /// C7: a MULTIPLY-WOUND carrier refuses. Its locus IS the circle
    /// and every span turns forward; what refuses is the turns' sum,
    /// two where the adopted single period is one.
    #[test]
    fn c7_a_double_wound_carrier_refuses() {
        let d = f64::to_radians;
        let angles: Vec<f64> = (0..=6).map(|k| d(120.0 * f64::from(k))).collect();
        let twice = arc_chain(0.005, &angles);
        assert!(!matches!(
            recognize(&twice, crate::interpret::FileBand::test(EPS_IN)),
            CurveRecognition::Promoted { .. }
        ));
    }

    /// C8: **the certificate's SCALE, pinned from both sides.** The
    /// residual bounds the carrier's distance from the circle at the
    /// law's angle, so it must dominate the densely measured distance
    /// from the circle (soundness), and on a family whose error is
    /// radial it must stay within a stated factor of it (tightness):
    /// a hull or miss term inflated by a mutation breaks the second,
    /// any term dropped the first.
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
            let curve = NurbsCurve3::new(
                base.knots_f64_interim().clone(),
                control,
                base.weights().to_vec(),
            )
            .unwrap();
            let CurveRecognition::Promoted {
                curve:
                    Curve3::Circle(geom::Circle {
                        frame:
                            geom::Frame {
                                origin: center,
                                axis,
                                ..
                            },
                        radius: r,
                    }),
                residual,
                ..
            } = recognize(&curve, crate::interpret::FileBand::test(EPS_IN))
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
            if delta > 0.0 {
                assert!(
                    residual <= worst * TIGHTNESS,
                    "LOOSE at delta {delta:e}: certified {residual:e} vs true {worst:e}"
                );
                tight_checks += 1;
            }
        }
        assert!(tight_checks >= 3, "the tightness arm must actually run");
    }

    /// C8's tightness factor: the residual over the densely measured
    /// distance is a stable 3.28 on its family (the control-difference
    /// hull reads the corners' whole displacement, which the curve
    /// feels only in part); 4 leaves headroom for arithmetic drift and still
    /// catches a term doubled.
    const TIGHTNESS: f64 = 4.0;

    /// C10: **the MAP obligation's own row (INV-C6)** — a closed
    /// carrier RE-TIMED around its circle. Every interior weight sits
    /// `Δ` above its sub-arc's `cos(h/2)`: the conic hugs the circle
    /// (its densely measured distance from it is inside the budget, so
    /// a locus certificate has nothing to object to) while its
    /// parameter runs ahead of its own sub-arcs' law by several times
    /// the budget — measured here against the canonical form at the
    /// same parameter. It must stay NURBS; dropping the weight-miss
    /// term from the certificate promotes it.
    #[test]
    fn c10_a_re_timed_closed_carrier_refuses() {
        let radius = 0.005f64;
        let spans = 120usize;
        let angles: Vec<f64> = (0..=spans)
            .map(|k| core::f64::consts::TAU * (k as f64) / (spans as f64))
            .collect();
        let canonical = arc_chain(radius, &angles);
        let delta = 1.6e-4;
        let weights: Vec<f64> = canonical
            .weights()
            .iter()
            .enumerate()
            .map(|(i, &w)| if i % 2 == 1 { w + delta } else { w })
            .collect();
        let retimed = NurbsCurve3::new(
            canonical.knots_f64_interim().clone(),
            canonical.control().to_vec(),
            weights,
        )
        .unwrap();
        let (a, b) = retimed.domain();
        let (mut radial, mut map) = (0.0f64, 0.0f64);
        for k in 0..=24_000 {
            let t = a + (b - a) * f64::from(k) / 24_000.0;
            let p = retimed.eval(t);
            radial = radial.max((p.x.hypot(p.y) - radius).abs().max(p.z.abs()));
            map = map.max(p.distance(canonical.eval(t)));
        }
        assert!(
            radial < EPS_IN / 2.0,
            "the re-timed carrier lies on the circle within the budget: {radial:e}"
        );
        assert!(
            map > 3.0 * EPS_IN,
            "and runs off its sub-arcs' law by past the budget: {map:e}"
        );
        assert!(
            matches!(
                recognize(&retimed, crate::interpret::FileBand::test(EPS_IN)),
                CurveRecognition::StaysNurbs
            ),
            "a carrier on the circle but off its law must not promote"
        );
        assert!(
            matches!(
                recognize(&canonical, crate::interpret::FileBand::test(EPS_IN)),
                CurveRecognition::Promoted { .. }
            ),
            "the same net at its sub-arcs' own weights promotes"
        );
    }

    /// C11: **the law is the CARRIER's own**: knots spaced unevenly and
    /// joint weights that are not one value re-time the parameter
    /// exactly as the rational-quadratic arc law states (each span's
    /// local parameter affine in the carrier's, a Möbius change of it
    /// per span), so the carrier is still that law's circle and
    /// promotes, at a rounding-level residual.
    #[test]
    fn c11_uneven_knots_and_joint_weights_promote() {
        let d = f64::to_radians;
        let base = arc_chain(0.005, &[d(0.0), d(100.0), d(230.0), d(360.0)]);
        let uneven =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 3.0, 3.0, 3.25, 3.25, 3.25], 2)
                .unwrap();
        let joints = [1.0f64, 2.5, 0.4, 1.7];
        let weights: Vec<f64> = base
            .weights()
            .iter()
            .enumerate()
            .map(|(i, &w)| {
                if i % 2 == 0 {
                    joints[i / 2]
                } else {
                    w * (joints[i / 2] * joints[i / 2 + 1]).sqrt()
                }
            })
            .collect();
        for (what, knots, weights) in [
            ("uneven knots", uneven.clone(), base.weights().to_vec()),
            (
                "joint weights",
                base.knots_f64_interim().clone(),
                weights.clone(),
            ),
            ("both", uneven, weights),
        ] {
            let curve = NurbsCurve3::new(knots, base.control().to_vec(), weights).unwrap();
            let CurveRecognition::Promoted { residual, .. } =
                recognize(&curve, crate::interpret::FileBand::test(EPS_IN))
            else {
                panic!("{what}: the carrier's own law must promote");
            };
            assert!(residual < 1e-15, "{what}: residual {residual:e}");
        }
    }

    /// C12: **the incompleteness, recorded**: a circle stated at another
    /// degree — the 4×90° rational quadratic, degree-elevated to cubics —
    /// has no canonical form in INV-C6 and stays NURBS.
    #[test]
    fn c12_a_degree_elevated_circle_stays_nurbs() {
        let d = f64::to_radians;
        let quadratic = arc_chain(0.005, &[d(0.0), d(90.0), d(180.0), d(270.0), d(360.0)]);
        // Each span's homogeneous net, elevated: (P0, (P0 + 2P1)/3,
        // (2P1 + P2)/3, P2).
        let (q, w) = (quadratic.control(), quadratic.weights());
        let mut control = vec![q[0]];
        let mut weights = vec![w[0]];
        for j in 0..4 {
            let h = |i: usize| ((q[i] - Point3::origin()) * w[i], w[i]);
            let ((p0, w0), (p1, w1), (p2, w2)) = (h(2 * j), h(2 * j + 1), h(2 * j + 2));
            for (p, wt) in [
                ((p0 + p1 * 2.0) / 3.0, (w0 + 2.0 * w1) / 3.0),
                ((p1 * 2.0 + p2) / 3.0, (2.0 * w1 + w2) / 3.0),
                (p2, w2),
            ] {
                control.push(Point3::origin() + p / wt);
                weights.push(wt);
            }
        }
        let mut kn = vec![0.0; 4];
        for k in 1..4 {
            kn.extend([f64::from(k); 3]);
        }
        kn.extend([4.0; 4]);
        let cubic =
            NurbsCurve3::new(KnotVector::clamped(kn, 3).unwrap(), control, weights).unwrap();
        let m = cubic.eval(0.5);
        assert!(
            (m.x.hypot(m.y) - 0.005).abs() < 1e-15,
            "the elevated form is the same circle"
        );
        assert!(matches!(
            recognize(&cubic, crate::interpret::FileBand::test(EPS_IN)),
            CurveRecognition::StaysNurbs
        ));
    }

    /// The line pins' budget. The composite's certified sup carries
    /// the certification arithmetic's rounding slack, and the `√sup` metre
    /// conversion (INV-C3) turns a few-ulp slack on a centimetre-scale
    /// `dist²` into a ~2.5e-9 m FLOOR — an exact chord's certified
    /// residual is that floor, not zero (the dm1 census's measured
    /// range, 1.9e-10..6.2e-9 m over 3–100 mm chords). So the line
    /// pins run at a realistic import band (dm1's own declared ε_in is
    /// 1e-5 m) and L1 pins the floor's ORDER rather than pretending to
    /// zero; the circle pins keep their 1e-9 because INV-C1's `/r`
    /// conversion has no square root to amplify the slack.
    const EPS_LINE: f64 = 1e-6;

    /// L1: the unit — a two-point degree-1 chord certifies as a Line,
    /// the frame is the carrier's own (origin at its start, `dir`
    /// start → end), and the reversed carrier promotes to the opposite
    /// direction — orientation survives the promotion, which is what
    /// keeps `endpoint_params`' forward-projection rule meaning what
    /// the file meant.
    #[test]
    fn l1_a_two_point_chord_certifies_as_a_line() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let (a, b) = (Point3::new(0.01, 0.02, 0.03), Point3::new(0.05, 0.02, 0.03));
        let chord = NurbsCurve3::new(knots.clone(), vec![a, b], vec![1.0, 1.0]).unwrap();
        let CurveRecognition::Promoted {
            curve: Curve3::Line(geom::Line { origin, dir }),
            residual,
            kind,
        } = recognize(&chord, crate::interpret::FileBand::test(EPS_LINE))
        else {
            panic!("a chord must certify as a line");
        };
        assert_eq!(kind, PromotedCurveKind::Line);
        assert!(
            residual < 1e-8,
            "an exact chord's residual is the ring-slack floor: {residual:e}"
        );
        assert!(origin.distance(a) < 1e-15);
        assert!((dir.x - 1.0).abs() < 1e-15, "dir {dir:?}");
        let reversed = NurbsCurve3::new(knots, vec![b, a], vec![1.0, 1.0]).unwrap();
        let CurveRecognition::Promoted {
            curve: Curve3::Line(geom::Line { dir: rdir, .. }),
            ..
        } = recognize(&reversed, crate::interpret::FileBand::test(EPS_LINE))
        else {
            panic!("the reversed chord certifies too");
        };
        assert!((rdir.x + 1.0).abs() < 1e-15, "reversed dir {rdir:?}");
    }

    /// L2: **the segment obligation rides the MAP hull (INV-C4 via
    /// INV-C5), pinned at its value.** A carrier lying EXACTLY on the
    /// line whose control net overshoots the chord (the locus
    /// certificate has nothing to refuse) is metered by the hull
    /// alone: its middle control sits 0.03 from the affine chord's
    /// Greville position (its overshoot past the 0.04 chord is 0.01
    /// — dominated, as INV-C4 argues), and the budget decides AT the
    /// hull — certifying just above 0.03 and refusing just below,
    /// including at 0.025, a budget an excursion-only reading would
    /// have cleared. Dropping the hull channel (residual → ~δ_line ≈
    /// 0) reds every row here.
    #[test]
    fn l2_an_overshooting_net_decides_at_the_greville_hull() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let curve = NurbsCurve3::new(
            knots,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.05, 0.0, 0.0),
                Point3::new(0.04, 0.0, 0.0),
            ],
            vec![1.0; 3],
        )
        .unwrap();
        assert!(
            matches!(
                recognize(&curve, crate::interpret::FileBand::test(EPS_IN)),
                CurveRecognition::StaysNurbs | CurveRecognition::IllConditioned { .. }
            ),
            "an overshooting net must not certify at the module budget"
        );
        let CurveRecognition::Promoted { residual, kind, .. } =
            recognize(&curve, crate::interpret::FileBand::test(0.035))
        else {
            panic!("the map fold is 0.03, inside a 0.035 budget");
        };
        assert_eq!(kind, PromotedCurveKind::Line);
        assert!(
            (residual - 0.03).abs() < 1e-12,
            "the residual IS the Greville map fold: {residual:e}"
        );
        assert!(
            matches!(
                recognize(&curve, crate::interpret::FileBand::test(0.025)),
                CurveRecognition::StaysNurbs | CurveRecognition::IllConditioned { .. }
            ),
            "a budget below the hull refuses, even one an excursion-only reading clears"
        );
    }

    /// The rational gate is load-bearing (INV-C5's hypothesis), and it
    /// asks constancy: straight degree-2 carriers whose weights VARY
    /// must stay NURBS at every tested budget, however generous — a
    /// re-timed interior ([1, 2, 1]) and a re-timing the other way
    /// ([1, ½, 1]). The gate is the hypothesis of the Greville hull,
    /// not a residual: without it the hull reads 0 for both (their
    /// CONTROLS sit at the affine positions where the rational CURVE
    /// does not) and they would promote unsoundly.
    ///
    /// A CONSTANT weight is the exact affine chord
    /// (`NurbsCurve3::constant_weight`), so it promotes to the unit
    /// carrier's line. At `2` the residual is the unit carrier's to the
    /// bit (a power of two scales the composite exactly). At `3` and
    /// `0.1` the line is the same to the bit, but the residual is not:
    /// it is the square root of the zero-radius composite's rounding
    /// width, which the weight's bits enter (`3.44e-8` at `1`, `1.16e-7`
    /// at `0.1`), so it is pinned inside the budget only. At `1e-300`
    /// the composite's weight powers underflow and it refuses, and the
    /// carrier stays NURBS rather than reading the refusal as a zero.
    #[test]
    fn a_varying_weight_straight_carrier_refuses_and_a_constant_one_promotes() {
        let carrier = |w1: f64, w: f64| {
            let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            NurbsCurve3::new(
                knots,
                vec![
                    Point3::new(0.0, 0.0, 0.0),
                    Point3::new(1.5, 0.0, 0.0),
                    Point3::new(3.0, 0.0, 0.0),
                ],
                vec![w, w1, w],
            )
            .unwrap()
        };
        for eps in [1e-6, 1e-3, 1.0] {
            for (w1, w) in [(2.0, 1.0), (0.5, 1.0)] {
                assert!(
                    matches!(
                        recognize(&carrier(w1, w), crate::interpret::FileBand::test(eps)),
                        CurveRecognition::StaysNurbs | CurveRecognition::IllConditioned { .. }
                    ),
                    "weights [{w}, {w1}, {w}] must refuse the map certificate at ε = {eps:e}"
                );
            }
            let line =
                |w: f64| match recognize(&carrier(w, w), crate::interpret::FileBand::test(eps)) {
                    CurveRecognition::Promoted {
                        curve: Curve3::Line(geom::Line { origin, dir }),
                        residual,
                        ..
                    } => (
                        [origin.x, origin.y, origin.z, dir.x, dir.y, dir.z].map(f64::to_bits),
                        residual,
                    ),
                    other => panic!("weights [{w}; 3] at ε = {eps:e} must promote: {other:?}"),
                };
            let unit = line(1.0);
            let twos = line(2.0);
            assert_eq!(
                (twos.0, twos.1.to_bits()),
                (unit.0, unit.1.to_bits()),
                "weights [2; 3] at ε = {eps:e}"
            );
            for c in [3.0, 0.1] {
                let (got, residual) = line(c);
                assert_eq!(got, unit.0, "weights [{c}; 3] at ε = {eps:e}: the line");
                assert!(
                    residual <= eps,
                    "weights [{c}; 3] at ε = {eps:e}: residual {residual:e}"
                );
            }
            assert!(
                matches!(
                    recognize(
                        &carrier(1e-300, 1e-300),
                        crate::interpret::FileBand::test(eps)
                    ),
                    CurveRecognition::StaysNurbs | CurveRecognition::IllConditioned { .. }
                ),
                "weights [1e-300; 3] at ε = {eps:e}: a refused composite is no zero"
            );
        }
    }

    /// L3: **the MAP obligation's own row (INV-C5).** A doubling-back
    /// carrier that stays WITHIN the chord keeps the segment locus
    /// (INV-C4's coverage argument still holds — the projection is
    /// continuous from 0 to ℓ), but it is NOT the affine chord as a
    /// map: promotion would hand every downstream door a `Line` whose
    /// interior timing the file's carrier contradicts, and the edge
    /// would strand at adoption (no iso chart image reproduces a
    /// doubling-back map). It stays NURBS at the module band, and the
    /// budget decides at the measured re-timing: controls at x =
    /// {0, 0.03, 0.01, 0.04} against Greville-affine positions
    /// {0, 0.04/3, 0.08/3, 0.04} give hull 0.05/3 ≈ 0.0167.
    #[test]
    fn l3_a_within_chord_doubling_back_is_not_the_affine_map() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 2.0, 3.0, 3.0], 1).unwrap();
        let curve = NurbsCurve3::new(
            knots,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.03, 0.0, 0.0),
                Point3::new(0.01, 0.0, 0.0),
                Point3::new(0.04, 0.0, 0.0),
            ],
            vec![1.0; 4],
        )
        .unwrap();
        assert!(
            matches!(
                recognize(&curve, crate::interpret::FileBand::test(EPS_LINE)),
                CurveRecognition::StaysNurbs
            ),
            "a re-timed interior must not certify at the module band"
        );
        let CurveRecognition::Promoted {
            curve: Curve3::Line(geom::Line { dir, .. }),
            residual,
            ..
        } = recognize(&curve, crate::interpret::FileBand::test(0.02))
        else {
            panic!("the re-timing is 0.05/3, inside a 0.02 budget");
        };
        assert!((dir.x - 1.0).abs() < 1e-15);
        assert!(
            (residual - 0.05 / 3.0).abs() < 1e-12,
            "the residual IS the re-timing hull: {residual:e}"
        );
    }

    /// L4: the NEGATIVE control at every tested ε — a genuinely curved
    /// open carrier (a centimetre chord with a millimetre bulge) never
    /// promotes to Line.
    #[test]
    fn l4_a_curved_open_carrier_never_promotes_at_any_tested_eps() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let curve = NurbsCurve3::new(
            knots,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.02, 0.01, 0.0),
                Point3::new(0.04, 0.0, 0.0),
            ],
            vec![1.0; 3],
        )
        .unwrap();
        for eps in [1e-6, 1e-9, 1e-12] {
            assert!(
                matches!(
                    recognize(&curve, crate::interpret::FileBand::test(eps)),
                    CurveRecognition::StaysNurbs
                ),
                "a bulged quadratic is not a line at eps {eps:e}"
            );
        }
    }

    /// L5: **the line certificate's SCALE, pinned from both sides**
    /// (C8's shape for the line kind): the certified residual is a
    /// true upper bound on the densely measured distance to the
    /// SEGMENT (soundness), and within a stated factor of it
    /// (tightness) — so neither channel of the fold can be quietly
    /// weakened or inflated.
    #[test]
    fn l5_the_certified_residual_brackets_the_true_segment_deviation() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let (a, b) = (Point3::new(0.0, 0.0, 0.0), Point3::new(0.04, 0.0, 0.0));
        let mut tight_checks = 0;
        // Deltas sit ABOVE the interval-slack floor (EPS_LINE's doc), so
        // the tightness half measures the certificate and not the
        // slack; below the floor only the soundness half is meaningful.
        for delta in [1e-8f64, 1e-7, 5e-7] {
            let curve = NurbsCurve3::new(
                knots.clone(),
                vec![a, Point3::new(0.02, delta, 0.0), b],
                vec![1.0; 3],
            )
            .unwrap();
            let CurveRecognition::Promoted {
                curve: Curve3::Line(geom::Line { origin, dir }),
                residual,
                ..
            } = recognize(&curve, crate::interpret::FileBand::test(EPS_LINE))
            else {
                panic!("delta {delta:e} is inside the budget");
            };
            let len = (b - a).norm();
            let (d0, d1) = curve.domain();
            let mut worst = 0.0f64;
            for k in 0..=4096 {
                let p = curve.eval(d0 + (d1 - d0) * f64::from(k) / 4096.0);
                let t = (p - origin).dot(dir).clamp(0.0, len);
                worst = worst.max(p.distance(origin + dir * t));
            }
            assert!(
                worst <= residual,
                "UNSOUND at delta {delta:e}: certified {residual:e} < true {worst:e}"
            );
            assert!(
                residual <= worst * 10.0,
                "LOOSE at delta {delta:e}: certified {residual:e} vs true {worst:e}"
            );
            tight_checks += 1;
        }
        assert!(tight_checks == 3);
    }

    /// L6: the closure gate keeps the closed class the circle's — the
    /// closed rational circle promotes as the circle it is, and C5's
    /// collinear CLOSED carrier keeps its `IllConditioned` answer
    /// rather than being claimed by the line kind (its chord is inside
    /// ε_in, which determines no direction).
    #[test]
    fn l6_the_closure_gate_keeps_the_closed_class_the_circles() {
        assert!(matches!(
            recognize(
                &rational_circle(0.005, false),
                crate::interpret::FileBand::test(EPS_IN)
            ),
            CurveRecognition::Promoted {
                kind: PromotedCurveKind::Circle,
                ..
            }
        ));
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.01, 0.0, 0.0),
            Point3::new(0.02, 0.0, 0.0),
            Point3::new(0.01, 0.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ];
        let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
        let closed = NurbsCurve3::new(knots.clone(), control, vec![1.0; 5]).unwrap();
        assert!(matches!(
            recognize(&closed, crate::interpret::FileBand::test(EPS_IN)),
            CurveRecognition::IllConditioned { .. }
        ));
        // The OPEN collinear carrier in the same structure reaches the
        // circle estimator first, which declines it, and the line limb
        // takes it.
        let straight: Vec<_> = (0..5)
            .map(|k| Point3::new(0.005 * f64::from(k), 0.0, 0.0))
            .collect();
        let open = NurbsCurve3::new(knots, straight, vec![1.0; 5]).unwrap();
        assert!(matches!(
            recognize(&open, crate::interpret::FileBand::test(EPS_LINE)),
            CurveRecognition::Promoted {
                kind: PromotedCurveKind::Line,
                ..
            }
        ));
    }

    /// L7: **a refused locus composite refuses the line**: a degree-30
    /// Bézier on 31 collinear control points puts the cylinder
    /// composite past `compose`'s exact binomial row, so INV-C3 is NaN;
    /// the Greville hull alone (zero on this net) must not promote it.
    #[test]
    fn l7_a_refused_locus_composite_refuses_the_line() {
        let degree = 30usize;
        let mut kn = vec![0.0; degree + 1];
        kn.extend(vec![1.0; degree + 1]);
        let control: Vec<_> = (0..=degree)
            .map(|k| Point3::new(0.001 * k as f64, 0.0, 0.0))
            .collect();
        let curve = NurbsCurve3::new(
            KnotVector::clamped(kn, degree).unwrap(),
            control,
            vec![1.0; degree + 1],
        )
        .unwrap();
        assert!(matches!(
            recognize(&curve, crate::interpret::FileBand::test(EPS_LINE)),
            CurveRecognition::StaysNurbs
        ));
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
        let curve = NurbsCurve3::new(
            base.knots_f64_interim().clone(),
            control,
            base.weights().to_vec(),
        )
        .unwrap();
        let CurveRecognition::Promoted { residual, .. } =
            recognize(&curve, crate::interpret::FileBand::test(EPS_IN))
        else {
            panic!("must certify at the module budget");
        };
        assert!(
            matches!(
                recognize(&curve, crate::interpret::FileBand::test(residual * 1.5)),
                CurveRecognition::Promoted { .. }
            ),
            "must certify just above its own residual"
        );
        assert!(
            !matches!(
                recognize(&curve, crate::interpret::FileBand::test(residual / 1.5)),
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
        let outcome = recognize(&curve, crate::interpret::FileBand::test(EPS_IN));
        assert!(
            !matches!(outcome, CurveRecognition::Promoted { .. }),
            "a 300-degree carrier must not become the full circle"
        );
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
            let curve = NurbsCurve3::new(
                base.knots_f64_interim().clone(),
                control,
                base.weights().to_vec(),
            )
            .unwrap();
            let outcome = recognize(&curve, crate::interpret::FileBand::test(EPS_IN));
            let CurveRecognition::Promoted {
                curve:
                    Curve3::Circle(geom::Circle {
                        frame:
                            geom::Frame {
                                origin: center,
                                axis,
                                ..
                            },
                        radius: r,
                    }),
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

/// Near-half-turn and winding attacks on INV-C6, through the whole
/// recognizer.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp, clippy::panic)]
mod r2_delta {
    use super::*;
    use geom_core::spline::KnotVector;

    const EPS_IN: f64 = 1e-9;
    const R: f64 = 0.005;

    /// A carrier of rational-quadratic spans through `angles`, each
    /// span's corner at `r/cos(half)` with weight `cos(half)` — on the
    /// circle (strictly positive weight) for any span strictly under a
    /// half turn, INCLUDING one a hair under π.
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

    fn promotes(curve: &NurbsCurve3<f64>) -> bool {
        matches!(
            recognize(curve, crate::interpret::FileBand::test(EPS_IN)),
            CurveRecognition::Promoted { .. }
        )
    }

    /// **D1 — out and back over a near-half turn.** A span turning by
    /// `π − η` is on the circle for every `η > 0`; out and back over it
    /// has true turning 0 and covers half the circle. None may promote,
    /// down to `η` below the f64 spacing at π.
    #[test]
    fn d1_near_pi_spans_out_and_back_must_all_refuse() {
        let pi = core::f64::consts::PI;
        let mut promoted = Vec::new();
        for k in 0..64 {
            let eta = 1e-3 * 0.5f64.powi(k / 2) * if k % 2 == 0 { 1.0 } else { 0.37 };
            let top = pi - eta;
            if !(top.cos().is_finite() && (top / 2.0).cos() > 0.0) {
                continue;
            }
            if promotes(&chain(&[0.0, top, 0.0])) {
                promoted.push(eta);
            }
        }
        assert!(
            promoted.is_empty(),
            "an out-and-back over a near-half turn must never wind once: {promoted:?}"
        );
    }

    /// **D2 — the same, asymmetric.** Forward `π − η`, back by a
    /// different near-half turn, so the two spans round independently.
    /// Still closed, still true turning ~0.
    #[test]
    fn d2_asymmetric_near_pi_out_and_back_must_refuse() {
        let pi = core::f64::consts::PI;
        let mut bad = Vec::new();
        for a in 0..40 {
            for b in 0..8 {
                let eta = 4.0e-16 * 0.5f64.powi(a / 2) * (1.0 + 0.11 * f64::from(b));
                let top = pi - eta;
                let mid = top - 2.0e-16 * (1.0 + f64::from(b));
                if !((top / 2.0).cos() > 0.0 && (mid / 2.0).cos() > 0.0) {
                    continue;
                }
                if promotes(&chain(&[0.0, top, top - mid])) {
                    bad.push((eta, mid));
                }
            }
        }
        assert!(
            bad.is_empty(),
            "asymmetric near-π out-and-back wound once: {bad:?}"
        );
    }

    /// **D3 — the span margin, at its edge.** One forward turn in three
    /// spans whose widest turns 148° promotes; the same turn with a
    /// 152° span refuses (INV-C6 admits a span to `π − δ` = 150°).
    #[test]
    fn d3_the_span_margin_admits_150_degrees() {
        let d = f64::to_radians;
        assert!(promotes(&chain(&[d(0.0), d(148.0), d(254.0), d(360.0)])));
        assert!(!promotes(&chain(&[d(0.0), d(152.0), d(256.0), d(360.0)])));
    }

    /// **D4 — triple winding**, beyond C7's two turns, plus the single
    /// turn wound against the estimator's axis — which the estimator
    /// reads as winding the other way, so it promotes with the axis
    /// reversed rather than refusing.
    #[test]
    fn d4_triple_wound_refuses_and_reversed_reads_its_own_axis() {
        let d = f64::to_radians;
        let thrice: Vec<f64> = (0..=9).map(|k| d(120.0 * f64::from(k))).collect();
        assert!(!promotes(&chain(&thrice)), "three turns");
        let back: Vec<f64> = (0..=3).map(|k| d(-120.0 * f64::from(k))).collect();
        let CurveRecognition::Promoted {
            curve:
                Curve3::Circle(geom::Circle {
                    frame: geom::Frame { axis, .. },
                    ..
                }),
            ..
        } = recognize(&chain(&back), crate::interpret::FileBand::test(EPS_IN))
        else {
            panic!("a reversed single turn is a circle");
        };
        assert!((axis.z + 1.0).abs() < 1e-12, "reversed axis {axis:?}");
    }

    /// **D5 — the certificate is not merely restrictive**: the shipped
    /// class (the 3×120° form, and finer uniform subdivisions of one
    /// turn) certifies, so "refuses everything" is not how D1–D4 pass.
    #[test]
    fn d5_genuine_full_turns_still_certify() {
        for spans in [3usize, 4, 5, 6, 8, 12, 30] {
            let angles: Vec<f64> = (0..=spans)
                .map(|k| core::f64::consts::TAU * (k as f64) / (spans as f64))
                .collect();
            assert!(
                promotes(&chain(&angles)),
                "{spans}-span circle must promote"
            );
        }
    }
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::float_cmp,
    clippy::panic,
    clippy::cast_precision_loss,
    clippy::many_single_char_names,
    clippy::too_many_lines
)]
mod rev2_probe {
    use super::*;
    use geom_core::spline::KnotVector;

    struct Rng(u64);
    impl Rng {
        fn f(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
        }
        fn r(&mut self, a: f64, b: f64) -> f64 {
            a + (b - a) * self.f()
        }
        fn unit(&mut self) -> Vec3<f64> {
            loop {
                let v = Vec3::new(self.r(-1.0, 1.0), self.r(-1.0, 1.0), self.r(-1.0, 1.0));
                let n = v.norm();
                if n > 0.1 && n < 1.0 {
                    return v / n;
                }
            }
        }
    }

    struct Case {
        curve: NurbsCurve3<f64>,
        center: Point3<f64>,
        axis: Vec3<f64>,
        radius: f64,
        closed: bool,
    }

    fn build(rng: &mut Rng, delta_scale: f64, pert: f64, max_span_deg: f64) -> Option<Case> {
        let closed = rng.f() < 0.5;
        let d = f64::to_radians;
        let mut incs: Vec<f64> = Vec::new();
        if closed {
            let n = 3 + (rng.f() * 4.0) as usize;
            let raw: Vec<f64> = (0..n).map(|_| rng.r(0.3, 1.0)).collect();
            let s: f64 = raw.iter().sum();
            incs = raw.iter().map(|x| x / s * core::f64::consts::TAU).collect();
            if incs.iter().any(|&h| h > d(max_span_deg)) {
                return None;
            }
        } else {
            let n = 1 + (rng.f() * 4.0) as usize;
            for _ in 0..n {
                incs.push(d(rng.r(3.0, max_span_deg)));
            }
            if incs.iter().sum::<f64>() >= d(359.0) {
                return None;
            }
        }
        let radius = 10f64.powf(rng.r(-3.0, 1.0));
        let axis = rng.unit();
        let u0 = axis.cross(rng.unit());
        let u = u0 / u0.norm();
        let v = axis.cross(u);
        let center = Point3::new(rng.r(-1.0, 1.0), rng.r(-1.0, 1.0), rng.r(-1.0, 1.0));
        let at = |t: f64, r: f64| center + u * (r * t.cos()) + v * (r * t.sin());
        let start = rng.r(0.0, 6.28);
        let jw: Vec<f64> = (0..=incs.len())
            .map(|_| if rng.f() < 0.5 { 1.0 } else { rng.r(0.3, 3.0) })
            .collect();
        let mut control = vec![at(start, radius)];
        let mut weights = vec![jw[0]];
        let mut th = start;
        for (k, &h) in incs.iter().enumerate() {
            let c = (h / 2.0).cos();
            control.push(at(th + h / 2.0, radius / c));
            let dl = if rng.f() < 0.5 {
                0.0
            } else {
                rng.r(-1.0, 1.0) * delta_scale
            };
            weights.push(c * (jw[k] * jw[k + 1]).sqrt() * (1.0 + dl).max(1e-3));
            th += h;
            control.push(at(th, radius));
            weights.push(jw[k + 1]);
        }
        if closed {
            let f = control[0];
            *control.last_mut().unwrap() = f;
        }
        for p in &mut control {
            *p = *p + rng.unit() * (pert * rng.f());
        }
        let mut kn = vec![0.0; 3];
        let mut x = 0.0;
        for _ in 1..incs.len() {
            x += rng.r(0.1, 5.0);
            kn.push(x);
            kn.push(x);
        }
        x += rng.r(0.1, 5.0);
        kn.extend([x; 3]);
        let curve = NurbsCurve3::new(KnotVector::clamped(kn, 2).ok()?, control, weights).ok()?;
        Some(Case {
            curve,
            center,
            axis,
            radius,
            closed,
        })
    }

    /// Dense |C(t) − Ĉ_c(t)| against the promoted circle's own frame.
    fn dense_map(
        curve: &NurbsCurve3<f64>,
        circle: &geom::Circle<f64>,
        closed: bool,
    ) -> (f64, f64, bool) {
        let o = circle.frame.origin;
        let ax = circle.frame.axis;
        let u = circle.frame.u_ref;
        let v = ax.cross(u);
        let r = circle.radius;
        let ang = |p: Point3<f64>| {
            let q = p - o;
            q.dot(v).atan2(q.dot(u))
        };
        let ctl = curve.control();
        let w = curve.weights();
        let flat = curve.knots_f64_interim().knots().to_vec();
        let spans = (ctl.len() - 1) / 2;
        let mut th: Vec<f64> = (0..=spans).map(|k| ang(ctl[2 * k])).collect();
        for k in 1..=spans {
            while th[k] <= th[k - 1] {
                th[k] += core::f64::consts::TAU;
            }
        }
        if closed {
            th[spans] = th[0] + core::f64::consts::TAU;
        }
        let on = |t: f64, rr: f64| o + u * (rr * t.cos()) + v * (rr * t.sin());
        let (mut map, mut locus, mut mono) = (0.0f64, 0.0f64, true);
        let mut prev = f64::NEG_INFINITY;
        for j in 0..spans {
            let (ta, tb) = (th[j], th[j + 1]);
            let h = tb - ta;
            let c = (h / 2.0).cos();
            let pts = [on(ta, r), on((ta + tb) / 2.0, r / c), on(tb, r)];
            let ws = [w[2 * j], c * (w[2 * j] * w[2 * j + 2]).sqrt(), w[2 * j + 2]];
            let (k0, k1) = (flat[2 + 2 * j], flat[3 + 2 * j]);
            let n = 4000;
            for i in 0..=n {
                let s = i as f64 / n as f64;
                let t = k0 + (k1 - k0) * s;
                let p = curve.eval(t);
                let b = [(1.0 - s) * (1.0 - s), 2.0 * s * (1.0 - s), s * s];
                let den: f64 = (0..3).map(|i| b[i] * ws[i]).sum();
                let mut acc = Vec3::new(0.0, 0.0, 0.0);
                for i in 0..3 {
                    acc = acc + (pts[i] - Point3::new(0.0, 0.0, 0.0)) * (b[i] * ws[i] / den);
                }
                let cc = Point3::new(0.0, 0.0, 0.0) + acc;
                map = map.max(p.distance(cc));
                let q = p - o;
                let hh = q.dot(ax);
                locus = locus.max(((q - ax * hh).norm() - r).hypot(hh));
                let mut a = ang(p);
                while a < prev - 1e-9 {
                    a += core::f64::consts::TAU;
                }
                if a < prev - 1e-12 {
                    mono = false;
                }
                prev = a;
            }
        }
        let _ = map;
        (map, locus, mono)
    }

    #[test]
    fn rev2_fuzz_inv_c6_soundness() {
        let mut rng = Rng(0x5eed_1234);
        let (mut promoted, mut worst_ratio, mut worst_locus_ratio) = (0usize, 0.0f64, 0.0f64);
        let mut unsound = Vec::new();
        for trial in 0..6000 {
            let (ds, pt, eps) = match trial % 4 {
                0 => (0.0, 0.0, 1e-9),
                1 => (1e-7, 1e-8, 1e-6),
                2 => (1e-4, 1e-6, 1e-5),
                _ => (1e-3, 1e-5, 1e-4),
            };
            let sc = case_scale(&mut rng);
            let Some(case) = build(&mut rng, ds, pt * sc, 150.0) else {
                continue;
            };
            let file = crate::interpret::FileBand::test(eps);
            if let CurveRecognition::Promoted {
                curve: Curve3::Circle(circle),
                residual,
                ..
            } = recognize(&case.curve, file)
            {
                promoted += 1;
                let (map, locus, mono) = dense_map(&case.curve, &circle, case.closed);
                let slack = 2e-14 * (circle.radius + 2.0);
                if map > residual + slack || locus > residual + slack || !mono {
                    unsound.push(format!("trial {trial}: map {map:e} locus {locus:e} residual {residual:e} mono {mono} closed {} R {:e}", case.closed, case.radius));
                }
                if residual > 1e-11 {
                    worst_ratio = worst_ratio.max(map / residual);
                    worst_locus_ratio = worst_locus_ratio.max(locus / residual);
                }
            }
        }
        println!(
            "promoted {promoted}; worst map/residual {worst_ratio}; worst locus/residual {worst_locus_ratio}"
        );
        for u in unsound.iter().take(20) {
            println!("{u}");
        }
        assert!(unsound.is_empty(), "{} unsound", unsound.len());
    }

    fn case_scale(rng: &mut Rng) -> f64 {
        rng.r(0.0, 1.0)
    }

    /// The bound directly, at huge weight error, spans near 150°, uneven knots:
    /// map_certificate's residual vs dense |C − Ĉ_c|, frame known.
    #[test]
    fn rev2_map_certificate_bound_far_from_cos() {
        let mut rng = Rng(77);
        let mut worst = 0.0f64;
        let mut n = 0;
        let mut bad = Vec::new();
        for trial in 0..4000 {
            let ds = [0.5, 2.0, 5.0, 0.99][trial % 4];
            let Some(case) = build(&mut rng, ds, 0.0, 150.0) else {
                continue;
            };
            let file = crate::interpret::FileBand::test(1e-9);
            let spans = conic_spans(&case.curve).unwrap();
            let closed = case.closed;
            let Some((u_ref, residual)) = map_certificate(
                &case.curve,
                spans,
                case.center,
                case.axis,
                case.radius,
                closed,
                file,
            ) else {
                continue;
            };
            n += 1;
            let circle = geom::Circle {
                frame: geom::Frame {
                    origin: case.center,
                    axis: case.axis,
                    u_ref,
                },
                radius: case.radius,
            };
            let (map, _, _) = dense_map(&case.curve, &circle, closed);
            if residual > 1e-11 {
                worst = worst.max(map / residual);
            }
            if map > residual * (1.0 + 1e-12) + 2e-14 * (case.radius + 2.0) {
                bad.push(format!(
                    "trial {trial}: map {map:e} > residual {residual:e}"
                ));
            }
        }
        println!("certified {n}; worst map/residual {worst}");
        for b in bad.iter().take(10) {
            println!("{b}");
        }
        assert!(bad.is_empty());
    }
}

//! **Pcurves as per-half-edge certified caches** (M5 PR 6; C4): the
//! chart-image cache
//! of an edge's certified 3-D carrier, certified **in meters through
//! the map**.
//!
//! This module owns the cache *value* and its certification gate; the
//! per-half-edge storage, the minting pass, and the face-level branch
//! walk live in `topo` (`topo::pcurves`), which is where half-edges,
//! faces, and loops exist.
//!
//! # What a pcurve is here (and what it is not)
//!
//! D2/OQ4 stand carrier-primary: the intensional description is
//! authoritative, the 3-D carrier is the authoritative *machinery*, and
//! a pcurve is a peer **cache** — never a peer of the description. A
//! [`PcurveCache`] is therefore constructible only through
//! [`PcurveCache::certify`] (its fields are private), exactly as
//! [`crate::EdgeCurve`] is: an uncertified pcurve is unrepresentable.
//!
//! # The parameter contract (spec §2, D1 verbatim)
//!
//! **The pcurve's parameter IS the edge's carrier parameter** — the
//! `he_plus`-forward `t ∈ [t₀, t₁]` of [`crate::EdgeCurve::params`].
//! There is no re-parameterization and no per-face parameter flip in
//! storage: the traversal sense per face is *derived* from the
//! half-edge, never stored. Both half-edges of an edge therefore carry
//! pcurves over the same `[t₀, t₁]` in the same direction; what differs
//! between them is the *chart* (different faces, possibly different
//! surfaces) and, on a periodic chart, the **branch** (the seam case —
//! see `topo::pcurves`).
//!
//! This contract is why the stored form is [`Pcurve::Harmonic`] and not
//! either of PR 5's `pcurve` constructors. Both of those are
//! **locus**-exact but **parameter**-non-affine: the rational-quadratic
//! chain's parameter is the conic's rational one (its θ correspondence
//! is pinned only at 0, ½, 1 of each segment), and the fitted cylinder
//! graph's parameter is the PR 4 loop's chord-length parameter. Fed
//! through the map at the certification schedule, both land millimetres
//! from `C(tᵢ)` on ordinary metre-scale geometry — orders above any ε
//! this kernel runs at. `tests/pcurve_parameter_finding.rs` pins that
//! measurement rather than asserting it. See the report/PR description
//! for the deviation this justifies.
//!
//! # The certified statement (spec §3, C4 verbatim)
//!
//! `|S(P(t)) − C(t)| ≤ ε` — a **3-D displacement in metres** between
//! the surface-composed pcurve and the carrier cache, on the shared
//! [`crate::CERT_SAMPLES`] schedule, plus a **between-samples envelope**
//! whose own statement the certificate NAMES
//! ([`PcurveCertificate::statement`]), because the lanes do not
//! bound the same thing. For the closed-form lane, and for a fitted
//! image on a NURBS chart, the envelope bounds *that same displacement*
//! over the whole span. For a fitted image on a periodic ANALYTIC chart
//! it cannot: `S ∘ P` is transcendental in the pcurve's azimuth channel
//! and the C9 ring has no transcendentals by construction, so the
//! between-samples statement there is the carrier's incidence with the
//! chart's own surface (`sup |f_S(C(t))|`) together with limb 3's
//! uniqueness tube — [`EnvelopeStatement::OnLocusHull`] carries the
//! argument, and the displacement itself stays certified at the
//! schedule.
//! **No UV-space tolerance appears in any certified statement or in any
//! message this module emits**: chart steps are implementation dials,
//! the map's local stretch is the lever arm, and a certification
//! quoting chart units would be dimensionally dishonest (C4; D4 ¶1
//! transposed).
//!
//! ## The between-samples limb, and why it is not a hull bound
//!
//! C2.2 requires a *sup-norm* statement, not a sampled max: between
//! samples is exactly where a cache lies. For the fitted rung that limb
//! is a control-coefficient hull bound (`geom_core::spline::compose`).
//! For the forms this PR mints it is **stronger and closed-form**: for
//! every (chart, carrier) pair in the certified lane, both `S ∘ P` and
//! `C` lie in the four-dimensional function space
//! `span{1, cos t, sin t, t}` with **exactly computable coefficients**,
//! so the residual does too, and
//!
//! ```text
//! sup |D(t)| ≤ |Δ₀| + |Δ_a| + |Δ_b| + |Δ_l|·max(|t₀|, |t₁|)
//! ```
//!
//! is a certified sup bound over the *whole* span with nothing sampled
//! and nothing hulled — the envelope IS the between-samples limb, and
//! it is tighter in kind than a hull over an unknown polynomial (a hull
//! bound exists to bound what sampling cannot see; here nothing is
//! sampled). It is evaluated at `T`, so it is scalar-generic exactly
//! like every other residual in [`crate::certify`]: `f64` on the value
//! lane, a rigorous enclosure on the interval lane (C6/C9).
//!
//! A corollary worth stating, because it is the [`Pcurve`] type doing
//! the work rather than a check: within the certified family the
//! residual has four coefficients, and the nine-sample schedule
//! determines them — so a corruption that hides *between* samples is
//! **unrepresentable** here, not merely caught. The envelope catches
//! the rest (this module's `a_corrupted_pcurve_fails_typed` and
//! `the_envelope_dominates_a_dense_resampling` rows).
//!
//! One boundary on that corollary, because the trilean has a band:
//! certification admits an **ε-shell around the family** (the winding
//! decision classifies `|pl.x − β|·r ≤ ε` as Zero), and a pcurve in
//! that shell is not exactly of the four-coefficient shape. The
//! envelope carries the discarded drift explicitly — the *snap slack*
//! of [`PcurveCache::certify`] step 4 — so the stored bound dominates
//! the true sup for every input the gate admits, not merely for the
//! exact-in-family caches the minting lane produces.
//!
//! # Domain validity (spec §3)
//!
//! Two limbs, both part of the certificate:
//!
//! - **One branch, pinned at the start.** On a periodic chart the
//!   azimuth channel of a [`Pcurve::Harmonic`] is `α + β·t` with a
//!   *single* stored `α` and a winding `β ∈ {−1, 0, +1}`. There is no
//!   per-sample branch choice to get wrong: the M2 PR 5 meridian
//!   finding ("nearest-previous per-sample unwrapping is a bug")
//!   is generalized here by making the wrong unwrap
//!   **unrepresentable** — a τ jump cannot be expressed. Which branch
//!   (`α + kτ`) a given half-edge takes is chosen once per face by the
//!   loop walk in `topo::pcurves` and *certified* by loop continuity
//!   there.
//! - **Trim containment.** The certificate records exactly one thing:
//!   that the pcurve's chart-box enclosure lies inside the face's chart
//!   window ([`ChartWindow`], supplied by the caller — the face's own
//!   one-branch hull); escaping it is the typed
//!   [`PcurveCertifyError::TrimEscape`]. It is a **precondition this
//!   door requires of its caller**, and what it buys is that it is the
//!   cache's only BRANCH constraint: on a periodic chart a τ-shifted
//!   pcurve certifies every other check identically, so this is the
//!   one check that can tell the two apart. Whether any given caller
//!   can trip it is that caller's property — `topo::pcurves` records
//!   that neither of its own can. Two honesty notes, both binding:
//!   - The window is a conservative *over-approximation* of the trim
//!     region (a box, not the region bounded by the loop).
//!     Point-in-trim-region is the tessellation trim-loop consumer's,
//!     arriving with PR 11.
//!   - **No bound on the window's own azimuth width is certified
//!     here.** The one-period statement that IS certified is the
//!     per-pcurve azimuth extent ([`PcurveCheck::AzimuthPeriod`]) plus,
//!     at body level, the loop-closure check in `topo::pcurves` (a
//!     loop's total azimuth advance is 0 or exactly ±τ). A naive
//!     "window width ≤ τ" check would be WRONG as stated: the window
//!     is a hull of *conservative* chart boxes, and a rim pcurve's box
//!     spans ±(reach) in azimuth, so a legitimately minted seam-closed
//!     wall's window is already ~2τ wide. Tightening the window (exact
//!     ranges instead of conservative boxes) is a separate unit.
//!
//! # Consumers
//!
//! Wired here: the tier-3 validator's pcurve pass (`topo::validate`).
//! **Not** wired here, and named so nobody wonders: tessellation trim
//! loops and curved-face quadrature (M5 PR 11), SSI on trimmed faces
//! and native ℝ⁴ pcurve production (M5 PR 7), the census extension
//! (PR 12). Those are the hot-path consumers C4 cites as the reason to
//! store at all; until they land, a stored pcurve is exercised by its
//! certification and by nothing else.

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom::{NurbsSurface, Surface};
use geom_core::k_stats::decide;
use geom_core::predicate::{Band, BandError};
use geom_core::spline::{KnotVector, SpanLocate};
use geom_core::{Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, Vec2, Vec3};

use crate::certify::CERT_SAMPLES;
use crate::ssi::{SsiCertificate, SsiLimb, SsiOperand};

/// A pcurve: the 2-D chart image of an edge's carrier, parameterized by
/// **the carrier's own parameter** (module docs).
///
/// Four variants; the closed enum is the D3 shape.
///
/// # What separates them
///
/// [`Pcurve::Harmonic`] is the closed-form image of the C5 table's
/// (chart, carrier) pairs: exact in the four-dimensional space
/// `span{1, cos t, sin t, t}`, with an envelope that is a *closed-form
/// sup bound* over the whole span and nothing sampled.
///
/// [`Pcurve::Fitted`] is the general rung, and the one rung no kernel
/// constructor mints — see [`PcurveCache::certify_fitted`], its sole
/// door, for what that frontier is waiting on. It is the 2-D NURBS
/// chart image an SSI trace produces **on the carrier's own parameter**, by
/// construction rather than by coincidence (the ℝ⁴ trace yields the
/// 3-D curve and both pcurves as projections of one parameterized
/// object, and `geom::NurbsCurve2::interpolate_with_params` fits
/// them on the carrier's chord parameters — the OQ4 identity). It has
/// no closed form on either side of the comparison, so its
/// between-samples obligation is discharged by the **C2.2 control-hull
/// machinery** in `geom_brep::ssi::certify` instead: see
/// [`PcurveCertificate::statement`] for exactly which sup-norm each
/// lane bounds, and [`PcurveFittedLane`] for which scalars can derive
/// it at all.
///
/// [`Pcurve::IsoLine`] (M6-3) and [`Pcurve::IsoArc`] (M8-3) are the two
/// NURBS-chart boundary rungs: both images are exactly straight in UV,
/// and they are separate variants because their MOVING CHANNEL differs
/// — `IsoLine`'s is the carrier's own parameter, `IsoArc`'s is the
/// chart's rational-quadratic Bézier parameter, related to the arc
/// angle by a transcendental piecewise map. Each variant's own docs
/// carry the derivation.
///
/// **Not `Copy`.** Two variants carry heap payloads — `Fitted` an
/// `Arc<NurbsCurve2>`, `IsoArc` a `Vec`-backed
/// [`geom_core::spline::KnotVector`] — so [`Pcurve`] and
/// [`PcurveCache`] are `Clone` only, and either variant alone is
/// enough for that. Removing one does not make the enum `Copy`.
#[derive(Clone, Debug)]
pub enum Pcurve<T: Real> {
    /// The closed-form chart image
    /// `P(t) = p0 + pa·cos t + pb·sin t + pl·t`.
    ///
    /// This one form is exact for every (chart, carrier) pair the C5
    /// table mints: a rim circle on its cylinder chart (`pl = (β, 0)`,
    /// the `v = const` line), a seam/meridian line (`pl = (0, dv)`, the
    /// `u = const` line), a tilted-section conic on its cylinder chart
    /// (the sinusoid graph `pa = pb = (0, ·)`, `pl = (β, 0)`), and any
    /// conic or line in a plane chart (the chart map is affine, so the
    /// carrier's own `{1, cos, sin, t}` form maps through coefficient
    /// by coefficient).
    Harmonic {
        /// The constant term (chart coordinates).
        p0: Point2<T>,
        /// The `cos t` coefficient.
        pa: Vec2<T>,
        /// The `sin t` coefficient.
        pb: Vec2<T>,
        /// The linear-in-`t` coefficient.
        pl: Vec2<T>,
    },
    /// The **fitted** chart image: a 2-D NURBS curve whose parameter is
    /// the carrier's own (type docs; the OQ4 identity is the entry
    /// requirement, not a hope). This is the rung-3 form — an SSI
    /// trace's chart projection — and it is certified through the
    /// control-hull machinery, never through the harmonic algebra.
    Fitted(Arc<NurbsCurve2<T>>),
    /// The **general curve-in-UV** (U2's `General` arm): a 2-D NURBS
    /// chart image on the carrier's own parameter that does NOT carry
    /// [`Pcurve::Fitted`]'s construction provenance.
    ///
    /// `Fitted` is the SSI trace's projection, and its entry
    /// requirement is the OQ4 identity — the 3-D curve and both
    /// pcurves are projections of one parameterized ℝ⁴ object, so the
    /// shared parameter is a construction fact. `General` is the same
    /// SHAPE with that fact absent: a curve in UV whose agreement with
    /// the carrier is a certified measurement rather than a
    /// construction identity (#498's interior/diagonal
    /// `Intersection` loci, and any chart image a future lane fits
    /// directly).
    ///
    /// It certifies at exactly the Fitted GRADE — the same C2
    /// certificate, hull sup-norm and uniqueness tube, through
    /// [`PcurveCache::certify_general`] — because the grade is a
    /// statement about what was MEASURED, and the two arms measure
    /// the same thing. What differs is what may be assumed without
    /// measuring, which is why the variants are distinct and no arm
    /// is ever a catch-all for the other.
    General(Arc<NurbsCurve2<T>>),
    /// The **exact straight line in UV**: `P(t) = p0 + pl·t` — the
    /// iso-parameter lane of a NURBS chart (M6-3; M5 PR 10 §3's
    /// "Line-in-UV pcurve variant", landed where its first minting
    /// construction lands).
    ///
    /// Why a dedicated variant instead of a [`Pcurve::Harmonic`] with
    /// zero trigonometric channels: the iso lane's certification
    /// hinges on the image being exactly line-shaped, and scalar
    /// equality is deliberately not a thing this kernel does outside
    /// `f64` structure (C6) — the VARIANT makes the shape structural,
    /// so no zero-test on `T` ever has to run. Every loft/sweep wall
    /// boundary stores this form: wall–wall seams as `u = const`
    /// lines, cap–wall rims as `v = const` lines (the definitional
    /// payoff — no fit anywhere).
    IsoLine {
        /// The chart point at `t = 0`.
        p0: Point2<T>,
        /// The velocity in chart coordinates (constant).
        pl: Vec2<T>,
    },
    /// The **circular-ARC rim** on a NURBS chart (M8-3): the chart
    /// image is again an exact straight line in UV — the boundary
    /// column `v = 0`/`v = 1` — but the moving channel is the chart's
    /// own **rational-quadratic Bézier parameter**, not the arc angle
    /// the carrier is parameterized by. That mismatch is the whole
    /// reason this is a variant and not an [`Pcurve::IsoLine`], and it
    /// is exactly the arm M6-3 banked.
    ///
    /// # The map, derived
    ///
    /// One sub-arc of angle `h` is the rational quadratic with weights
    /// `(1, cos(h/2), 1)` whose middle control point is the tangent
    /// intersection. Writing `φ` for the angle measured from the
    /// sub-arc's MID-angle and `s ∈ [0, 1]` for its Bézier parameter,
    /// the standard identity is
    ///
    /// ```text
    /// φ = 2·arctan( (2s − 1)·tan(h/4) )   ⟺   s = ½ + tan(φ/2) / (2·tan(h/4))
    /// ```
    ///
    /// (check: `s = 1` gives `φ = h/2`). With `m` uniform sub-arcs the
    /// chart parameter is `g = (k + s)/m`, `k` the sub-arc index — so
    /// the map is **transcendental and piecewise**, representable by
    /// none of the other three variants. The turn's sign cancels
    /// (`tan(σx/2)/tan(σh/4) = tan(x/2)/tan(h/4)`), which is why
    /// nothing here carries one.
    ///
    /// # Why this stays `T`-generic
    ///
    /// The sub-arcs are uniform *by construction*, so their
    /// breakpoints on the NORMALIZED carrier parameter
    /// `τ = (t − t0)/angle` are pure `f64` structure — `breaks` below.
    /// Locating `k` is then [`geom_core::spline::SpanLocate`] plus
    /// `enclosure_hull`, the same mechanism [`Pcurve::Fitted`] already
    /// uses to stay sound at interval scalars.
    IsoArc {
        /// The chart point at the arc's start (`g = 0`).
        p0: Point2<T>,
        /// The chart displacement over the WHOLE arc (`g: 0 → 1`).
        pd: Vec2<T>,
        /// The carrier parameter at `g = 0`.
        t0: T,
        /// The arc's total turn, `t1 − t0`. Always POSITIVE on a
        /// certified cache — check 2 (`pcurve_interval_forward`)
        /// refuses a non-forward interval before the class is built —
        /// so the map's own sign cancellation (variant docs) never has
        /// a sign to carry.
        angle: T,
        /// Sub-arc breakpoints on `τ ∈ [0, 1]`: a uniform clamped
        /// degree-1 knot vector with one span per sub-arc.
        breaks: KnotVector,
    },
}

/// The chart parameter `g(t) ∈ [0, 1]` of an [`Pcurve::IsoArc`]
/// (variant docs for the derivation). Total: a malformed `breaks`
/// answers poison rather than panicking (D4).
fn iso_arc_g<T: SpanLocate>(t: T, t0: T, angle: T, breaks: &KnotVector) -> T {
    let spans = breaks.control_count().saturating_sub(1);
    if spans == 0 {
        return T::from_f64(f64::NAN);
    }
    #[allow(clippy::cast_precision_loss)]
    let m = T::from_f64(spans as f64);
    let h = angle / m;
    // tan(h/4) through sin/cos — no transcendental beyond the ring's
    // own `sin_cos` (the same door every harmonic pcurve uses).
    let (s_q, c_q) = (h * T::from_f64(0.25)).sin_cos();
    let tan_q = s_q / c_q;
    let set = ((t - t0) / angle).locate_spans(breaks);
    let degree = breaks.degree();
    let mut acc: Option<T> = None;
    for span in set.first.index()..=set.last.index() {
        #[allow(clippy::cast_precision_loss)]
        let kf = T::from_f64(span.saturating_sub(degree) as f64);
        let phi = (t - t0) - (kf + T::from_f64(0.5)) * h;
        let (s_h, c_h) = (phi * T::from_f64(0.5)).sin_cos();
        let s = T::from_f64(0.5) + (s_h / c_h) / (T::from_f64(2.0) * tan_q);
        let g = (kf + s) / m;
        acc = Some(match acc {
            None => g,
            Some(a) => a.enclosure_hull(g),
        });
    }
    // **`g` is a chart parameter in `[0, 1]` BY DEFINITION** (variant
    // docs), and the identity above delivers exactly `0` and `1` at
    // the arc's two ends only when `m·(angle/m)` reproduces `angle` to
    // the bit — true for the angles the kernel BUILDS, false in
    // general for the ones it reads off a file. Left alone, the
    // overshoot is ~1 ulp of `g`, which lands the arc rim's chart
    // corner ~1 ulp off the chart's own boundary and makes the
    // rectangle certificate in `topo::props` — an EXACT f64 identity,
    // deliberately — fail on a rim that is otherwise perfect. Clamping
    // states the definition rather than approximating it: `locate_spans`
    // already saturates at the end spans, so no value outside `[0, 1]`
    // was ever a different POINT, only a different rounding of the
    // same one.
    //
    // **Scope, honestly**: that argument is ON-DOMAIN. For `t` outside
    // `[t0, t0 + angle]` the map is a genuine extrapolation and `g` is
    // legitimately outside `[0, 1]`; the clamp saturates it. Every
    // caller evaluates on the trimmed interval (the schedule, the loop
    // walk's entry/exit, `trim_containment`), and `locate_spans`
    // already saturated the span choice off-domain, so no caller sees
    // a changed answer — but a future off-domain one would, and that
    // is a property of this function, not an accident of its callers.
    acc.map(|g| g.max(T::zero()).min(T::one()))
        .unwrap_or_else(|| T::from_f64(f64::NAN))
}

impl<T: SpanLocate> Pcurve<T> {
    /// The chart point at the **carrier parameter** `t` (module docs:
    /// the parameter is not re-mapped). Fixed evaluation order (D9);
    /// total.
    pub fn eval(&self, t: T) -> Point2<T> {
        match self {
            Pcurve::Harmonic { p0, pa, pb, pl } => {
                let (s, c) = t.sin_cos();
                Point2::new(
                    p0.x + pa.x * c + pb.x * s + pl.x * t,
                    p0.y + pa.y * c + pb.y * s + pl.y * t,
                )
            }
            Pcurve::Fitted(image) | Pcurve::General(image) => image.eval(t),
            Pcurve::IsoLine { p0, pl } => Point2::new(p0.x + pl.x * t, p0.y + pl.y * t),
            Pcurve::IsoArc {
                p0,
                pd,
                t0,
                angle,
                breaks,
            } => {
                let g = iso_arc_g(t, *t0, *angle, breaks);
                Point2::new(p0.x + pd.x * g, p0.y + pd.y * g)
            }
        }
    }

    /// A **conservative** chart-box enclosure of the pcurve over
    /// `[t₀, t₁]`: the constant term widened by the trigonometric
    /// amplitudes and the linear term's reach. Deliberately coarse
    /// (module docs: the trim limb is a box over-approximation at M5)
    /// and always sound in the containment direction — it can only make
    /// a containment claim harder to satisfy, never falsely satisfied.
    pub fn chart_box(&self, t0: T, t1: T) -> ChartWindow<T> {
        match self {
            Pcurve::Harmonic { p0, pa, pb, pl } => {
                let reach = t0.abs().max(t1.abs());
                let du = pa.x.abs() + pb.x.abs() + pl.x.abs() * reach;
                let dv = pa.y.abs() + pb.y.abs() + pl.y.abs() * reach;
                ChartWindow {
                    u_min: p0.x - du,
                    u_max: p0.x + du,
                    v_min: p0.y - dv,
                    v_max: p0.y + dv,
                }
            }
            // The **convex-hull property**: a NURBS curve with positive
            // weights lies in the hull of its control polygon, so the
            // control net's own axis-aligned box contains the image
            // over the WHOLE domain — conservative in the containment
            // direction, exactly like the harmonic arm, and a
            // convexity fact rather than an evaluation.
            Pcurve::Fitted(image) | Pcurve::General(image) => {
                let mut ctl = image.control().iter();
                let Some(first) = ctl.next() else {
                    // An empty net is unrepresentable through
                    // `NurbsCurve2::new`; a degenerate window here
                    // would be a claim, so answer with the inverted
                    // box, which contains nothing and escapes every
                    // window loudly.
                    let (lo, hi) = (T::one(), -T::one());
                    return ChartWindow {
                        u_min: lo,
                        u_max: hi,
                        v_min: lo,
                        v_max: hi,
                    };
                };
                let mut w = ChartWindow {
                    u_min: first.x,
                    u_max: first.x,
                    v_min: first.y,
                    v_max: first.y,
                };
                for p in ctl {
                    w.u_min = w.u_min.min(p.x);
                    w.u_max = w.u_max.max(p.x);
                    w.v_min = w.v_min.min(p.y);
                    w.v_max = w.v_max.max(p.y);
                }
                w
            }
            // A straight line's extremes over an interval are at its
            // endpoints — the one arm whose box is TIGHT, not merely
            // conservative (still sound in the containment direction).
            Pcurve::IsoLine { p0, pl } => {
                let a = Point2::new(p0.x + pl.x * t0, p0.y + pl.y * t0);
                let b = Point2::new(p0.x + pl.x * t1, p0.y + pl.y * t1);
                ChartWindow {
                    u_min: a.x.min(b.x),
                    u_max: a.x.max(b.x),
                    v_min: a.y.min(b.y),
                    v_max: a.y.max(b.y),
                }
            }
            // The arc rim's chart image is the SEGMENT `p0 → p0 + pd`
            // (`g` is monotone in `t`: `tan` is monotone on
            // `(−π/2, π/2)` and every sub-arc's `φ/2` stays inside it).
            //
            // This is the WHOLE-SEGMENT box, and it deliberately
            // ignores `t0`/`t1`: sound at every window (a sub-window's
            // image is a sub-segment) and TIGHT at the full span, which
            // is the only span any mint asks for. A sub-window would
            // get a conservative box — never a wrong one.
            Pcurve::IsoArc { p0, pd, .. } => {
                let b = Point2::new(p0.x + pd.x, p0.y + pd.y);
                ChartWindow {
                    u_min: p0.x.min(b.x),
                    u_max: p0.x.max(b.x),
                    v_min: p0.y.min(b.y),
                    v_max: p0.y.max(b.y),
                }
            }
        }
    }

    /// The azimuth channel shifted by `k` whole periods — the only
    /// branch freedom a pcurve on a periodic chart has (module docs:
    /// the branch is chosen once, by the face's loop walk, and never
    /// per sample).
    /// `None` only if a fitted image's control net could not be
    /// re-wrapped with its own knots and weights — structurally
    /// impossible, and therefore reported rather than swallowed (see
    /// the arm's note).
    pub fn shift_branch(&self, k: T, period: T) -> Option<Self> {
        Some(match self {
            Pcurve::Harmonic { p0, pa, pb, pl } => Pcurve::Harmonic {
                p0: Point2::new(p0.x + k * period, p0.y),
                pa: *pa,
                pb: *pb,
                pl: *pl,
            },
            // A whole-period shift of a NURBS chart image is a
            // translation of its control net in the azimuth channel:
            // exact, structure-preserving, and it moves the branch
            // WITHOUT touching the parameter (the same one-branch
            // contract the harmonic arm keeps).
            //
            // The rebuild takes the ORIGINAL knots and weights and a
            // control net of the original length, so `NurbsCurve2::new`
            // re-validates exactly what it validated once already and
            // cannot fail. It is still not swallowed: the failing arm
            // answers `None`, because silently returning the UNSHIFTED
            // image would hand the loop walk a branch it did not ask
            // for, and "the branch is chosen once and never guessed" is
            // this method's whole point. The kernel never panics
            // (D4 ¶2), so the impossible case is a typed absence the
            // caller must handle, not an abort.
            Pcurve::Fitted(image) | Pcurve::General(image) => {
                let shifted: Vec<Point2<T>> = image
                    .control()
                    .iter()
                    .map(|p| Point2::new(p.x + k * period, p.y))
                    .collect();
                let rebuilt =
                    NurbsCurve2::new(image.knots().clone(), shifted, image.weights().to_vec())
                        .ok()?;
                // The arm reconstructs the ORIGINAL variant: a shift
                // moves the branch, never the provenance.
                if matches!(self, Pcurve::Fitted(_)) {
                    Pcurve::Fitted(Arc::new(rebuilt))
                } else {
                    Pcurve::General(Arc::new(rebuilt))
                }
            }
            // The iso lane lives on NURBS charts, which have no
            // periodic azimuth — no minted iso line is ever shifted.
            // Kept total (the same first-channel translation) rather
            // than special-cased: a shift by k periods of a
            // non-periodic chart is meaningless but harmless, and the
            // loop walk never computes a nonzero k there.
            Pcurve::IsoLine { p0, pl } => Pcurve::IsoLine {
                p0: Point2::new(p0.x + k * period, p0.y),
                pl: *pl,
            },
            // Same reasoning as the iso-line arm: NURBS charts have no
            // periodic azimuth, so no minted arc rim is ever shifted.
            Pcurve::IsoArc {
                p0,
                pd,
                t0,
                angle,
                breaks,
            } => Pcurve::IsoArc {
                p0: Point2::new(p0.x + k * period, p0.y),
                pd: *pd,
                t0: *t0,
                angle: *angle,
                breaks: breaks.clone(),
            },
        })
    }
}

/// A chart-space axis-aligned window: the conservative
/// over-approximation of a face's trim region the domain-validity limb
/// certifies against (module docs).
#[derive(Clone, Copy, Debug)]
pub struct ChartWindow<T: Real> {
    /// Lower azimuth/first-parameter bound.
    pub u_min: T,
    /// Upper azimuth/first-parameter bound.
    pub u_max: T,
    /// Lower second-parameter bound.
    pub v_min: T,
    /// Upper second-parameter bound.
    pub v_max: T,
}

impl<T: Real> ChartWindow<T> {
    /// The hull of two windows (fixed order, D9).
    pub fn hull(self, other: Self) -> Self {
        Self {
            u_min: self.u_min.min(other.u_min),
            u_max: self.u_max.max(other.u_max),
            v_min: self.v_min.min(other.v_min),
            v_max: self.v_max.max(other.v_max),
        }
    }
}

/// Which certified statement a [`PcurveCertifyError`] names — one
/// variant per documented check (the [`crate::certify::CertCheck`]
/// idiom).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcurveCheck {
    /// The carrier-parameter interval's forward-span check.
    ParamSpan,
    /// `|S(P(tᵢ)) − C(tᵢ)|` at a schedule sample — metres through the
    /// map (module docs).
    MapResidual,
    /// The closed-form between-samples envelope — metres.
    Envelope,
    /// The chart winding `β` selection on a periodic chart.
    ChartWinding,
    /// The pcurve's azimuth extent against one period.
    AzimuthPeriod,
    /// The pcurve's chart box against the face's window.
    TrimContainment,
}
/// The number a fitted-lane refusal carries, named for what it IS.
///
/// The SSI door's definite refusals each measured something different,
/// and each had already projected it out of an enclosure before the
/// error was minted. Flattening the three onto one anonymous `f64` —
/// or worse, onto [`geom_core::MarginDiag::Value`], which additionally
/// claims the classifier judged it and found it in the band — loses
/// the only thing a reader needs: what the number means. Naming each
/// follows `edge_nurbs`' `certified_clearance` precedent, where the
/// same SSI errors are translated into that lane's vocabulary.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FittedMagnitude {
    /// A certificate limb exceeded ε: the limb's own residual bound in
    /// metres, as projected from its enclosure when the limb refused.
    /// A definite refusal's quantity — not a classified margin.
    LimbResidual(f64),
    /// Limb 3's uniqueness tube straddled zero. The number is a
    /// **certified clearance**, not a measured extent: it is exactly
    /// zero whenever the enclosure contains zero, so `0` here reads
    /// "not certifiably zero-free", never "measured zero". The box
    /// count is the informative companion (the `edge_nurbs` precedent
    /// carries the same pair).
    CertifiedClearance {
        /// The certified zero-free clearance in metres (0 = none).
        certified_clearance: f64,
        /// Boxes in the tube chain.
        boxes: u32,
    },
    /// A certified foot point would not converge: the last distance the
    /// projection saw, in metres, at the schedule parameter it gave up
    /// on.
    LastFootDistance {
        /// The schedule parameter.
        t: f64,
        /// The last distance seen.
        last_distance: f64,
    },
}

/// Typed pcurve-certification failure (D4 ¶3): actionable, closed enum.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PcurveCertifyError {
    /// The face's chart is outside the certified lane. Plane and
    /// cylinder charts have exact closed-form images for every carrier
    /// the C5 table mints; cone/sphere/torus charts and `Nurbs`
    /// surfaces keep derive-on-demand status until their consumers land
    /// (M5 PR 7 for SSI-produced pcurves, PR 11 for tessellation trim).
    /// This is a *routing decision*, permanent until a PR moves it —
    /// never a runtime fallback (C5).
    UnsupportedChart {
        /// The surface kind, named.
        chart: &'static str,
    },
    /// The carrier kind is outside the **closed-form** lane: a
    /// [`Pcurve::Harmonic`] image is being certified against a carrier
    /// with no `{1, cos, sin, t}` form (a `Curve3::Nurbs`).
    ///
    /// **Retired for the rung-3 class at M6-2** (the S9 flip). This
    /// variant used to be the answer for *any* fitted/marched carrier,
    /// because the storage variant that could hold its chart image did
    /// not exist; a rung-3 carrier now certifies through
    /// [`Pcurve::Fitted`] and this refusal is what remains for the
    /// genuine mismatch — a harmonic image claimed for a spline
    /// carrier, which no constructor mints.
    UnsupportedCarrier,
    /// A [`Pcurve::Fitted`] cache was offered to a scalar with **no
    /// certified fitted lane** — [`PcurveFittedLane`]'s refusing side.
    /// A dual scalar may not certify (D1, 2026-08-19), so the C9 ring
    /// is not reachable from it and the C2.2 hull bound does not exist
    /// there; the refusal is typed and static rather than a silent
    /// success.
    FittedLaneUnsupported {
        /// The scalar lane, named.
        scalar: &'static str,
    },
    /// A [`Pcurve::Fitted`] cache was certified without the **mate
    /// operand**: the fitted lane's certificate is the SSI one, whose
    /// uniqueness tube is a statement about the operand PAIR whose
    /// intersection minted the carrier. The mate is re-read from the
    /// body at rest (never stored with the cache — a stored operand
    /// could drift from the body's own), so a caller that has one must
    /// supply it.
    FittedMateMissing,
    /// An iso image was offered outside the iso lane's certified
    /// inventory, with the exact boundary named. The refused set is:
    /// a chart that is still the mvfs placeholder; an INTERIOR
    /// (non-boundary) iso; a DIAGONAL line in UV; a degenerate iso
    /// (neither chart channel moves); an image that leaves the chart's
    /// parameter domain, where the hull bound does not hold; a carrier
    /// whose spline structure is not the chart's own boundary row, or
    /// whose weights are not positive (the convex-hull hypothesis
    /// itself); a seam-class image over a non-spline carrier; a LINE
    /// cap rim on a RATIONAL column (the Greville hull is a
    /// linear-precision fact the rational basis does not have). A
    /// rational CHART and an ARC-parameterized cap rim are not in that
    /// set — both certify, through the seam and arc-rim classes. Nor is
    /// an intersection locus that is not a boundary column: since
    /// PCURVE P-2 (#498) the mint derives its image and stores it as
    /// [`Pcurve::General`] instead of refusing. Typed and permanent until a unit moves it — never a
    /// runtime fallback (C5).
    IsoUnsupported {
        /// The refused class, named.
        what: &'static str,
    },
    /// The fitted lane's SSI certificate refused, **flattened to its
    /// three actionable parts** rather than nested whole.
    ///
    /// The triple IS the actionable content — which limb, why, and the
    /// offending margin — which is what a consumer can act on and what
    /// this module's other refusals carry. Nesting `SsiError` whole
    /// would additionally have cost this enum its `Copy` (that error
    /// nests fit and spline refusals which are not `Copy`), rippling
    /// through `topo::pcurves::PcurveMintError` and its containers for
    /// no gain in what a caller can do. That is an avoided ripple, not
    /// a demonstrated dependency: no `src` site is known to exercise
    /// this enum's `Copy` today — every flow here moves through
    /// `map_err` — so the honest statement is "the flattened form is
    /// the right shape and keeps the existing error stack unchanged",
    /// not "`Copy` is required".
    FittedCertificate {
        /// The SSI limb that refused, when the refusal names one.
        limb: Option<SsiLimb>,
        /// The refusal's own reason.
        what: &'static str,
        /// The number this refusal measured, named for what it IS —
        /// `None` when the refusal is structural and measured nothing.
        ///
        /// Deliberately NOT a classified margin. Every value reaching
        /// here is a definite refusal's own quantity, already projected
        /// out of an enclosure when the SSI error was minted, so
        /// dressing it as [`geom_core::MarginDiag::Value`] would assert
        /// two false things at once: that it is a margin the classifier
        /// judged, and that it landed inside the band. Escalations —
        /// the only refusals that DO carry a classified margin — are a
        /// separate variant ([`PcurveCertifyError::FittedEscalated`]),
        /// which carries the classifier's diagnostic whole.
        magnitude: Option<FittedMagnitude>,
    },
    /// A fitted-lane classification ESCALATED — D4 ¶3's
    /// escalate-never-guess, at the SSI door rather than at one of this
    /// module's own schedule checks.
    ///
    /// The classifier's [`Indeterminate`] is carried WHOLE, so the
    /// margin, the band it was judged against and the predicate's name
    /// travel together. They are not separable without lying: a margin
    /// without its band cannot be read (is 1.8e-12 inside the band or
    /// three decades clear of it?), and the projection this error used
    /// to perform — margin onto one `f64` — reported `NaN` for every
    /// interval-lane escalation, which is also what genuine poison
    /// reports.
    FittedEscalated {
        /// The classifier's diagnostic, whole.
        cause: Indeterminate,
    },
    /// The stored carrier-parameter interval is not forward — the same
    /// `he_plus` contract [`crate::certify::CertifyError::IntervalNotForward`]
    /// enforces on the carrier itself.
    IntervalNotForward,
    /// The pcurve's azimuth channel is not `α + β·t` with
    /// `β ∈ {−1, 0, +1}` on a periodic chart: a chart image this lane
    /// cannot certify in closed form (a helix-like or
    /// multiply-wound azimuth). Typed, never approximated.
    ChartWindingUnsupported,
    /// The pcurve's azimuth extent definitely exceeds one full period —
    /// the chart-side counterpart of
    /// [`crate::certify::CertifyError::WindingExceeded`].
    AzimuthPeriodExceeded,
    /// A certified residual definitely exceeded the tolerance band: the
    /// pcurve does not represent the carrier through the map (D4 ¶2).
    ResidualExceeded {
        /// The check that failed.
        check: PcurveCheck,
        /// The schedule sample index (0 for span-wide checks).
        sample: u32,
    },
    /// The pcurve leaves the face's chart window — the trim-containment
    /// limb of domain validity (module docs: a box over-approximation
    /// at M5).
    TrimEscape,
    /// A classification escalated (sliver band or poison) — D4 ¶3's
    /// escalate-never-guess.
    Escalated {
        /// The check that escalated.
        check: PcurveCheck,
        /// The schedule sample index.
        sample: u32,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// The linear band could not be built from the run's tolerance.
    Band(BandError),
}

impl core::fmt::Display for PcurveCertifyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsupportedChart { chart } => write!(
                f,
                "pcurve certification: no {chart}-chart lane covers this pcurve — every \
                 analytic chart certifies its closed-form (Harmonic) classes, a NURBS \
                 chart routes through its description-driven \
                 iso/fitted lanes instead of this door, and an image outside the chart's \
                 harmonic family belongs to the fitted lane where one exists"
            ),
            Self::UnsupportedCarrier => write!(
                f,
                "pcurve certification: a closed-form (Harmonic) chart image was offered for a \
                 carrier with no {{1, cos, sin, t}} form. The general fitted/marched rung \
                 certifies this class through the control-hull lane, but no kernel \
                 constructor mints one — reaching it means offering the chart image to \
                 PcurveCache::certify_fitted yourself"
            ),
            Self::FittedLaneUnsupported { scalar } => write!(
                f,
                "pcurve certification: a fitted (rung-3) chart image has no certified lane at \
                 the {scalar} scalar — its between-samples bound is an exact-arithmetic-ring hull, \
                 and this scalar may not certify one. Replay the body at f64, the \
                 telemetry probe, or the interval scalar to certify it"
            ),
            Self::FittedMateMissing => write!(
                f,
                "pcurve certification: a fitted (rung-3) chart image needs the MATE operand — \
                 its certificate is the SSI one, whose uniqueness tube is a statement about \
                 the surface PAIR whose intersection minted the carrier. Supply the mate \
                 face's surface (re-read from the body; never stored with the cache)"
            ),
            Self::IsoUnsupported { what } => write!(
                f,
                "pcurve certification: the iso-line lane refuses this class — {what}"
            ),
            Self::FittedCertificate {
                limb,
                what,
                magnitude,
            } => write!(
                f,
                "pcurve certification: the fitted lane's certificate refused{} — {what}{}",
                match limb {
                    Some(l) => format!(" at {}", l.name()),
                    None => String::new(),
                },
                match magnitude {
                    Some(FittedMagnitude::LimbResidual(v)) => format!(" (limb residual {v:e} m)"),
                    Some(FittedMagnitude::CertifiedClearance {
                        certified_clearance,
                        boxes,
                    }) => format!(
                        " (certified clearance {certified_clearance:e} m over {boxes} tube \
                         boxes — zero means no clearance was certified, not a measured zero)"
                    ),
                    Some(FittedMagnitude::LastFootDistance { t, last_distance }) => format!(
                        " (the projection's last distance was {last_distance:e} m at t = {t:e})"
                    ),
                    None => String::new(),
                }
            ),
            // The classifier's own payload renderer, plus this module's
            // site context and the shared recourse tail — the
            // composition `IndeterminatePayload` exists for.
            Self::FittedEscalated { cause } => write!(
                f,
                "pcurve certification: the fitted lane's certificate escalated — {} ({})",
                cause.payload(),
                geom_core::predicate::COINCIDENCE_RECOURSE
            ),
            Self::IntervalNotForward => write!(
                f,
                "pcurve certification: the carrier-parameter interval is not forward — a \
                 pcurve shares the edge's he_plus-forward parameter (D1)"
            ),
            Self::ChartWindingUnsupported => write!(
                f,
                "pcurve certification: the chart azimuth is not α + β·t with β in \
                 {{−1, 0, +1}} — this lane certifies closed-form chart images only"
            ),
            Self::AzimuthPeriodExceeded => write!(
                f,
                "pcurve certification: the pcurve winds more than one full period around \
                 the chart — split the edge first (the winding gate, chart side)"
            ),
            Self::ResidualExceeded { check, sample } => write!(
                f,
                "pcurve certification: {check:?} at sample {sample} definitely exceeds the \
                 tolerance band — the pcurve does not represent the carrier through the \
                 map (D4 ¶2; the residual is a 3-D displacement in metres)"
            ),
            Self::TrimEscape => write!(
                f,
                "pcurve certification: the pcurve leaves its face's chart window — domain \
                 validity is part of the certificate"
            ),
            Self::Escalated {
                check,
                sample,
                cause,
            } => write!(
                f,
                "pcurve certification: {check:?} at sample {sample} escalated: {cause}"
            ),
            Self::Band(e) => write!(f, "pcurve certification: {e}"),
        }
    }
}

impl std::error::Error for PcurveCertifyError {}

/// **Which sup-norm a certificate's envelope bounds.** The pcurve lanes
/// discharge C2.2 by different mechanisms over different quantities,
/// and a certificate that did not say which would be a number without a
/// statement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvelopeStatement {
    /// `sup |S(P(t)) − C(t)|`, by the closed-form harmonic algebra —
    /// the [`Pcurve::Harmonic`] lane. Nothing sampled, nothing hulled.
    MapResidualClosedForm,
    /// `sup |S(P(t)) − C(t)|`, by the **tensor Bernstein composite**
    /// (`geom_core::spline::compose::tensor`) — the [`Pcurve::Fitted`]
    /// lane on a NURBS chart, where both sides of the comparison are
    /// splines and the difference is enclosed as ONE composite, so the
    /// cancellation survives into the bound.
    MapResidualComposite,
    /// `sup |f_S(C(t))|` — the on-locus control-hull bound, in metres,
    /// for a [`Pcurve::Fitted`] cache on an **analytic** chart.
    ///
    /// Read this one carefully, because it is a different sup from the
    /// other two and the difference is the honest content of the fitted
    /// analytic lane. `S ∘ P` on a periodic analytic chart is
    /// transcendental in the pcurve's own azimuth channel, so the C9
    /// ring — which has no transcendentals by construction — cannot
    /// enclose `S(P(t)) − C(t)` between samples at all. What it CAN
    /// enclose, exactly and tightly, is the carrier's incidence with
    /// the chart's own surface: `f_S ∘ C` is a polynomial composite.
    /// So the fitted analytic certificate proves, between the samples,
    /// that **the carrier never leaves the surface** — and pairs that
    /// with limb 3's uniqueness tube, which proves the locus near the
    /// carrier is a single arc, so there is no second branch for the
    /// chart image to have drifted onto. The map residual itself is
    /// certified at the [`CERT_SAMPLES`] schedule, as
    /// [`PcurveCertificate::max_residual`] records.
    OnLocusHull,
    /// `sup |S(P(t)) − C(t)|` for the two NURBS-chart boundary rungs —
    /// [`Pcurve::IsoLine`] (M6-3) and [`Pcurve::IsoArc`] (M8-3, whose
    /// chart column is rational by construction) — by the boundary-row
    /// **control-difference hull**: the traversed iso is a boundary
    /// row of the chart's own control net (a copy, no arithmetic —
    /// `crate::nurbs_iso`), so `S ∘ P` and the carrier-side comparison
    /// live in the same spline space and the partition-of-unity hull
    /// `sup |Σ Nᵢ·Δcᵢ| ≤ max |Δcᵢ|` bounds their difference over the
    /// whole domain — nothing sampled. The hull needs the two curves to
    /// share a spline space and (when rational) strictly positive
    /// weights, not a non-rational chart; M8-3 moved that hypothesis
    /// from a blanket chart-level gate to the class arms that use it.
    /// The banded axis/side/domain
    /// snap slacks (the trilean-admitted ε-shell around the exact
    /// axis-aligned family, metered through the chart's
    /// derivative-net stretch bounds) are folded in explicitly,
    /// exactly as the cylinder lane's winding snap slack is; every
    /// slack is exactly zero on the minted path.
    MapResidualIsoHull,
}

/// The certification record stored with a certified pcurve: the
/// schedule that ran, the worst sampled displacement, the
/// between-samples envelope and **which sup it bounds** — all metres
/// (module docs). Byte-identical across replays of the same
/// construction (D9).
#[derive(Clone, Copy, Debug)]
pub struct PcurveCertificate<T: Real> {
    /// The sample count of the schedule that ran ([`CERT_SAMPLES`]).
    pub samples: u32,
    /// The maximum `|S(P(tᵢ)) − C(tᵢ)|` over the schedule (metres) —
    /// **the sampled max only**. The between-samples statement is
    /// [`Self::envelope`], deliberately a separate field: folding them
    /// into one number would let a reader mistake a sup bound for a
    /// measurement or the reverse.
    pub max_residual: T,
    /// The certified sup bound on `|S(P(t)) − C(t)|` over the whole
    /// span (metres) — the C2.2 between-samples limb (module docs).
    /// Always ≥ the true sup, hence ≥ [`Self::max_residual`] for an
    /// exact-in-family pcurve; it additionally carries the winding
    /// snap's slack (see [`PcurveCache::certify`] step 4), so it is the
    /// number to quote for "how far can this cache be from its
    /// carrier".
    ///
    /// For a [`Pcurve::Fitted`] cache the *quantity* changes with the
    /// chart — read [`Self::statement`] before quoting this number.
    pub envelope: T,
    /// Which sup-norm [`Self::envelope`] bounds.
    pub statement: EnvelopeStatement,
    /// The **full C2 certificate** of a [`Pcurve::Fitted`] cache: hull
    /// sup-norm and uniqueness tube, re-derived (never trusted) by
    /// [`PcurveCache::recertify`] through `geom_brep::ssi::certify`.
    /// `None` for the closed-form lane, which discharges C2.2 by
    /// algebra and has no locus tube to prove — its one arc is the
    /// carrier itself.
    pub ssi: Option<SsiCertificate<T>>,
}

/// **Which scalars can derive a fitted pcurve's certificate** — the
/// static lane split, in the `topo::props::PropsQuadLane` shape (M5
/// PR 11's ratified pattern; `topo/src/props.rs`).
///
/// A [`Pcurve::Fitted`] cache's between-samples obligation is a C9-ring
/// hull bound, and building it is **certification**. `f64`, the
/// telemetry probe and the interval scalar may certify;
/// [`geom_core::Dual`] may not — Evan's D1 ruling, 2026-08-19: a dual
/// carries a bracket (the value channel's) and may still not certify,
/// which is why `geom_core::CertifiedEnclosure` has no dual impl and
/// `geom_core::Bounds` now does. So the fitted lane exists
/// for the first three and is **statically absent** for the fourth,
/// which is stated here as a refusing impl rather than discovered as a
/// mysterious failure at run time.
///
/// The trait is also what keeps `Bounds` out of `topo`'s signatures:
/// consumers write `T: PcurveFittedLane` and get the lane, exactly as
/// they write `T: PropsQuadLane` for the quadrature one.
pub trait PcurveFittedLane: Decide {
    /// The full C2 certificate of a fitted chart image against its
    /// operand pair, or `None` when this scalar has no certified lane.
    ///
    /// The carrier arrives as the edge's own [`Curve3`] (M6-3): a
    /// rung-3 `Curve3::Nurbs` feeds the SSI door directly; an exact
    /// `Curve3::Circle` (the sphere chart's GENERAL-circle class,
    /// walk row 4) is converted to its locus-exact rational-quadratic
    /// chain for the certificate limbs — every limb consulted is a
    /// statement about the LOCUS (on-locus hull, uniqueness tube), so
    /// the chain's own parameter never enters the certified claim;
    /// `t0`/`t1` name the traversed angular arc.
    ///
    /// # Errors
    ///
    /// [`PcurveCertifyError::FittedCertificate`] when the SSI
    /// certificate itself refuses, or for a (Circle carrier, NURBS
    /// operand) pairing — the NURBS limbs are parameter-coupled to a
    /// traced pcurve a synthetic arc chain does not have. Never from
    /// the "no lane" arm.
    fn fitted_certificate(
        carrier: &Curve3<Self>,
        t0: Self,
        t1: Self,
        image: &NurbsCurve2<Self>,
        surface: &Surface<Self>,
        mate: &Surface<Self>,
        band: Band,
    ) -> Result<Option<SsiCertificate<Self>>, PcurveCertifyError>;

    /// **The chart image of a spline carrier on a NURBS wall**, or
    /// `None` when this scalar has no certified lane.
    ///
    /// The producer is `edge_nurbs`'s — the one derivation of this
    /// object in the tree (`edge_nurbs::chart_image`): foot points at
    /// the D9-fixed schedule, interpolated on the carrier's own
    /// parameter. It is EVIDENCE and certifies nothing by itself; the
    /// caller's next move is [`PcurveCache::certify_general`], which
    /// bounds `sup_t |S(P(t)) − C(t)|` over the whole span against the
    /// operand pair.
    ///
    /// It sits on THIS trait rather than beside its producer because
    /// the derivation and the certificate are the same static split —
    /// both need the C9 ring, both are absent at [`geom_core::Dual`] —
    /// and a mint that had to name two lane traits for one image would
    /// carry the split twice. `edge_nurbs::EdgeNurbsLane` keeps its own
    /// door for the ADOPT path, which certifies the same image with the
    /// plane operand's limbs beside it.
    ///
    /// # Errors
    ///
    /// [`PcurveCertifyError::FittedCertificate`] when a foot point of
    /// the schedule will not converge or the interpolation is
    /// degenerate. Never from the "no lane" arm, which returns
    /// `Ok(None)`.
    fn general_image(
        carrier: &NurbsCurve3<Self>,
        wall: &NurbsSurface<Self>,
    ) -> Result<Option<NurbsCurve2<Self>>, PcurveCertifyError>;
    /// **The chart foot of one point** on a NURBS wall, or `None` when
    /// this scalar has no certified lane.
    ///
    /// [`Self::general_image`]'s single-sample sibling, same producer
    /// (`edge_nurbs::chart_foot`). Its consumer is the pcurve mint's
    /// rim arms: they know the SHAPE of their image and are only
    /// missing its position, which on a chart wider than the face it
    /// trims is not a knot-domain end. Evidence, not a certificate —
    /// the caller offers it to its own metre-valued check.
    ///
    /// # Errors
    ///
    /// [`PcurveCertifyError::FittedCertificate`] when the projection
    /// will not converge. Never from the "no lane" arm, which returns
    /// `Ok(None)`.
    fn chart_foot(
        point: Point3<Self>,
        wall: &NurbsSurface<Self>,
    ) -> Result<Option<Point2<f64>>, PcurveCertifyError>;

    /// The lane's name, for the typed refusal's text.
    fn lane_name() -> &'static str;
}

/// The image producer's body, shared by every bracket-carrying scalar
/// ([`PcurveFittedLane::general_image`]).
///
/// The `edge_nurbs` schedule with no per-sample hook: the transversality
/// sweep the adopt path runs there is a statement about the PLANE
/// operand, which the mint does not have in hand and does not need —
/// the mint's next step re-derives the whole C2 certificate against the
/// operand pair anyway.
fn general_image_lane<T: Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    carrier: &NurbsCurve3<T>,
    wall: &NurbsSurface<T>,
) -> Result<Option<NurbsCurve2<T>>, PcurveCertifyError> {
    let (t0, t1) = carrier.domain();
    match crate::edge_nurbs::chart_image(carrier, wall, |_, _| Ok(())) {
        Ok(image) => Ok(Some(image)),
        Err(crate::edge_nurbs::PlaneNurbsRefusal::FootPointInconclusive {
            sample,
            last_distance,
        }) => Err(PcurveCertifyError::FittedCertificate {
            limb: Some(SsiLimb::OnLocus),
            what: "a foot point of the chart-image schedule would not converge, so this \
                   locus has no derived image to certify",
            // The schedule's own parameter at that sample, computed the
            // way the schedule computes it — not the sample index dressed
            // up as one.
            magnitude: Some(FittedMagnitude::LastFootDistance {
                t: t0
                    + (t1 - t0) * f64::from(sample)
                        / f64::from(crate::edge_nurbs::PXN_FIT_SAMPLES - 1),
                last_distance,
            }),
        }),
        Err(_) => Err(PcurveCertifyError::FittedCertificate {
            limb: None,
            what: "the chart image could not be interpolated through the schedule's foot \
                   points (a degenerate parameterization)",
            magnitude: None,
        }),
    }
}

/// The foot producer's body, shared by every bracket-carrying scalar
/// ([`PcurveFittedLane::chart_foot`]).
fn chart_foot_lane<T: Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    point: Point3<T>,
    wall: &NurbsSurface<T>,
) -> Result<Option<Point2<f64>>, PcurveCertifyError> {
    match crate::edge_nurbs::chart_foot(point, wall) {
        Ok(foot) => Ok(Some(foot)),
        Err(crate::edge_nurbs::PlaneNurbsRefusal::FootPointInconclusive {
            last_distance, ..
        }) => Err(PcurveCertifyError::FittedCertificate {
            limb: Some(SsiLimb::OnLocus),
            what: "an edge endpoint has no certified foot on this chart, so where its \
                   image sits cannot be measured",
            magnitude: Some(FittedMagnitude::LastFootDistance {
                t: f64::NAN,
                last_distance,
            }),
        }),
        Err(_) => Err(PcurveCertifyError::UnsupportedChart {
            chart: "the mvfs placeholder is not a surface to derive a chart image on",
        }),
    }
}

/// The certified lane's body, shared by every bracket-carrying scalar.
///
/// The operand ORDER is load-bearing: the face's own surface is
/// operand **b**, because `certify_branch` reads the traced pcurve of
/// `b` — and the cache's image is exactly that pcurve, on the
/// carrier's own parameter (the OQ4 identity). A NURBS *mate* has no
/// stored image to offer, so that pairing refuses typed inside the SSI
/// door rather than being invented here.
fn fitted_lane<T: Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    image: &NurbsCurve2<T>,
    surface: &Surface<T>,
    mate: &Surface<T>,
    band: Band,
) -> Result<Option<SsiCertificate<T>>, PcurveCertifyError> {
    fn operand<T: Real>(s: &Surface<T>) -> SsiOperand<'_, T> {
        // The catch-all is SPLIT: an approximating surface's chart is
        // its fit's, so the spline operand is the one that describes
        // its geometry — routing it to `Analytic` would hand the SSI
        // limbs an implicit form that does not exist.
        match s {
            Surface::Nurbs(n) => SsiOperand::Nurbs(n),
            Surface::Approx(a) => SsiOperand::Nurbs(a.fit()),
            other @ (Surface::Plane { .. }
            | Surface::Cylinder { .. }
            | Surface::Cone { .. }
            | Surface::Sphere { .. }
            | Surface::Torus { .. }) => SsiOperand::Analytic(other),
        }
    }
    // The certificate's carrier spline: a rung-3 carrier IS one; an
    // exact circle converts to its locus-exact rational-quadratic
    // chain (trait docs — the limbs are locus statements, so the
    // chain's rational parameter never enters the claim). The chain
    // conversion is only honest against ANALYTIC operands: the NURBS
    // limbs warm-start foot points from the traced pcurve at the SAME
    // parameter, which a synthetic chain cannot offer.
    let chain;
    let spline: &NurbsCurve3<T> = match carrier {
        Curve3::Nurbs(spline) => spline,
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => {
            // `Approx` is included, and it has to be: `operand` three
            // lines up routes it to `SsiOperand::Nurbs(a.fit())`, so
            // the very limbs this guard's premise is about — the
            // parameter-coupled NURBS limbs — are the ones an `Approx`
            // operand would run. The guard reads the SAME roster its
            // premise names.
            let spline_operand =
                |s: &Surface<T>| matches!(s, Surface::Nurbs(_) | Surface::Approx(_));
            if spline_operand(surface) || spline_operand(mate) {
                return Err(PcurveCertifyError::FittedCertificate {
                    limb: None,
                    what: "a Circle carrier's rational-chain certificate is written for \
                           analytic operand pairs only (the spline limbs — a Nurbs \
                           payload's or an approximating surface's fit — are \
                           parameter-coupled to a traced pcurve)",
                    magnitude: None,
                });
            }
            chain = rational_arc_chain(*center, *axis, *radius, *u_ref, t0, t1).ok_or(
                PcurveCertifyError::FittedCertificate {
                    limb: None,
                    what: "the circle arc's rational-quadratic chain refused to build \
                           (degenerate span or malformed structure)",
                    magnitude: None,
                },
            )?;
            &chain
        }
        Curve3::Line { .. } | Curve3::Ellipse { .. } => {
            return Err(PcurveCertifyError::UnsupportedCarrier);
        }
    };
    let carrier = spline;
    // The lever arm and the tube ladder's widest rung, both from the
    // OBJECT BEING CERTIFIED rather than passed down a call chain that
    // has no better number: the carrier's own control-net diameter is
    // D4 ¶1's lever arm of last resort, and it is exactly the scale a
    // uniqueness tube around this carrier can hope to reach.
    let arm = carrier_diameter(carrier);
    crate::ssi::certify_rung3(
        carrier,
        Some(image),
        &operand(mate),
        &operand(surface),
        crate::ssi::TubeScale::uniform(arm),
        band,
    )
    .map(Some)
    .map_err(ssi_refusal)
}

/// The **locus-exact rational-quadratic chain** of a circle arc (Book
/// §7.3): ≤ 90° Bézier segments, middle weight `cos(θ/2)`, middle
/// point the tangent intersection `center + radial(m)·r/cos(θ/2)`.
///
/// The chain's LOCUS is the circle arc exactly (positive weights, the
/// classic construction); its rational parameter is NOT the angle, so
/// callers may consult it for locus statements only (the fitted
/// certificate's on-locus hull and uniqueness tube — trait docs).
/// Knot structure is `f64` (C6), read from the angular span's bracket
/// midpoints; control points are exact at `T`. `None` for a
/// degenerate (non-forward) span — the certificate's own forward-span
/// check refuses those before this door is consulted.
fn rational_arc_chain<T: Decide + geom_core::Bounds>(
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
    u_ref: Vec3<T>,
    t0: T,
    t1: T,
) -> Option<NurbsCurve3<T>> {
    let mid = |x: T| 0.5 * (x.lo() + x.hi());
    let (f0, f1) = (mid(t0), mid(t1));
    let span = f1 - f0;
    // NaN-catching by design: only a definitely-forward finite span
    // builds a chain.
    if !(span > 0.0 && span.is_finite()) {
        return None;
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (span / core::f64::consts::FRAC_PI_2).ceil().max(1.0) as usize;
    let cv = axis.cross(u_ref);
    let at = |t: f64| -> Point3<T> {
        let (s, c) = T::from_f64(t).sin_cos();
        center + (u_ref * c + cv * s) * radius
    };
    let seg = span / n as f64;
    let w_mid = (seg / 2.0).cos();
    let mut control: Vec<Point3<T>> = Vec::with_capacity(2 * n + 1);
    let mut weights: Vec<f64> = Vec::with_capacity(2 * n + 1);
    let mut knots: Vec<f64> = Vec::with_capacity(2 * n + 4);
    knots.extend([f0, f0, f0]);
    control.push(at(f0));
    weights.push(1.0);
    for i in 0..n {
        let a = f0 + seg * i as f64;
        let b = if i + 1 == n {
            f1
        } else {
            f0 + seg * (i + 1) as f64
        };
        let m = 0.5 * (a + b);
        let (s, c) = T::from_f64(m).sin_cos();
        control.push(center + (u_ref * c + cv * s) * (radius / T::from_f64(w_mid)));
        weights.push(w_mid);
        control.push(at(b));
        weights.push(1.0);
        if i + 1 == n {
            knots.extend([b, b, b]);
        } else {
            knots.extend([b, b]);
        }
    }
    let kv = geom_core::spline::KnotVector::clamped(knots, 2).ok()?;
    NurbsCurve3::new(kv, control, weights).ok()
}

/// The control-net diameter of a carrier, in metres — a convexity fact
/// (the hull property), not an evaluation.
fn carrier_diameter<T: Real>(carrier: &NurbsCurve3<T>) -> T {
    let mut ctl = carrier.control().iter();
    let Some(first) = ctl.next() else {
        return T::zero();
    };
    let (mut lo, mut hi) = (*first, *first);
    for p in ctl {
        lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
        hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
    }
    (hi - lo).norm()
}

/// The SSI refusal, reduced to the three parts this module's closed
/// enum carries — the limb, the reason (including the escalating
/// predicate's own name) and the offending margin. That triple is the
/// actionable content; see the
/// [`PcurveCertifyError::FittedCertificate`] docs for what the
/// flattening buys and, honestly, what it does not.
///
/// **Escalations leave by a different door.** They are the only
/// refusals carrying a margin the classifier judged, and that margin is
/// unreadable without the band it was judged against, so the whole
/// `Indeterminate` travels together in
/// [`PcurveCertifyError::FittedEscalated`]. What remains here is
/// definite and structural refusals, whose numbers are each named for
/// what they are ([`FittedMagnitude`]) and are `None` when the refusal
/// measured nothing.
fn ssi_refusal(e: crate::ssi::SsiError) -> PcurveCertifyError {
    use crate::ssi::SsiError as E;
    let (limb, what, magnitude) = match e {
        // An escalation is the ONE refusal that carries a classified
        // margin, and it leaves through its own door with the
        // classifier's diagnostic whole.
        E::Escalated(cause) => return PcurveCertifyError::FittedEscalated { cause },
        E::CertificateLimb { limb, value } => (
            Some(limb),
            "a certificate limb exceeded ε",
            Some(FittedMagnitude::LimbResidual(value)),
        ),
        E::TubeStraddles { margin, boxes } => (
            Some(SsiLimb::Tube),
            "the uniqueness tube's transversality straddles zero (a genuine sliver of the \
             operand pair — escalate, never desingularize)",
            Some(FittedMagnitude::CertifiedClearance {
                certified_clearance: margin,
                boxes,
            }),
        ),
        E::FootPointInconclusive { t, last_distance } => (
            Some(SsiLimb::OnLocus),
            "a certified foot point would not converge",
            Some(FittedMagnitude::LastFootDistance { t, last_distance }),
        ),
        E::TubeLadderEmpty { .. } => (
            Some(SsiLimb::Tube),
            "the uniqueness tube's radius ladder is empty — the carrier's extent is too \
             small against the run's tolerance for any rung to clear the ladder floor, so \
             limb 3 never ran and measured nothing",
            None,
        ),
        E::TubeProbeSilent { .. } => (
            Some(SsiLimb::Tube),
            "no rung of the uniqueness tube's radius ladder produced an enclosure, so \
             limb 3 has nothing to decide",
            None,
        ),
        E::UnsupportedCertificate { what } => (None, what, None),
        // Exhaustive BY VARIANT rather than by catch-all: a new
        // `SsiError` must be dispositioned here deliberately, and the
        // compiler is what enforces that. These are the structural
        // refusals whose full text lives at the SSI door; none of them
        // measured a quantity this lane can name.
        E::TransversalityBand { .. }
        | E::ExhaustivenessInconclusive { .. }
        | E::CellBudget { .. }
        | E::StepBudget { .. }
        | E::StepCollapsed { .. }
        | E::SeedRefinementFailed { .. }
        | E::SelfCrossingLocus { .. }
        | E::Fit(_)
        | E::FitSampleBudget { .. }
        | E::WrongLane { .. }
        | E::Band(_)
        | E::InvalidMarchTol { .. }
        | E::MarchTolMismatch { .. } => (
            None,
            "the rung-3 certificate refused structurally (see geom_brep::SsiError for the \
             full text at the SSI door)",
            None,
        ),
    };
    PcurveCertifyError::FittedCertificate {
        limb,
        what,
        magnitude,
    }
}

impl PcurveFittedLane for f64 {
    fn fitted_certificate(
        carrier: &Curve3<Self>,
        t0: Self,
        t1: Self,
        image: &NurbsCurve2<Self>,
        surface: &Surface<Self>,
        mate: &Surface<Self>,
        band: Band,
    ) -> Result<Option<SsiCertificate<Self>>, PcurveCertifyError> {
        fitted_lane(carrier, t0, t1, image, surface, mate, band)
    }

    fn general_image(
        carrier: &NurbsCurve3<Self>,
        wall: &NurbsSurface<Self>,
    ) -> Result<Option<NurbsCurve2<Self>>, PcurveCertifyError> {
        general_image_lane(carrier, wall)
    }

    fn chart_foot(
        point: Point3<Self>,
        wall: &NurbsSurface<Self>,
    ) -> Result<Option<Point2<f64>>, PcurveCertifyError> {
        chart_foot_lane(point, wall)
    }

    fn lane_name() -> &'static str {
        "f64"
    }
}

#[cfg(feature = "probe")]
impl PcurveFittedLane for geom_core::Probe {
    fn fitted_certificate(
        carrier: &Curve3<Self>,
        t0: Self,
        t1: Self,
        image: &NurbsCurve2<Self>,
        surface: &Surface<Self>,
        mate: &Surface<Self>,
        band: Band,
    ) -> Result<Option<SsiCertificate<Self>>, PcurveCertifyError> {
        fitted_lane(carrier, t0, t1, image, surface, mate, band)
    }

    fn general_image(
        carrier: &NurbsCurve3<Self>,
        wall: &NurbsSurface<Self>,
    ) -> Result<Option<NurbsCurve2<Self>>, PcurveCertifyError> {
        general_image_lane(carrier, wall)
    }

    fn chart_foot(
        point: Point3<Self>,
        wall: &NurbsSurface<Self>,
    ) -> Result<Option<Point2<f64>>, PcurveCertifyError> {
        chart_foot_lane(point, wall)
    }

    fn lane_name() -> &'static str {
        "telemetry probe"
    }
}

#[cfg(feature = "interval")]
impl PcurveFittedLane for geom_core::interval::Interval {
    fn fitted_certificate(
        carrier: &Curve3<Self>,
        t0: Self,
        t1: Self,
        image: &NurbsCurve2<Self>,
        surface: &Surface<Self>,
        mate: &Surface<Self>,
        band: Band,
    ) -> Result<Option<SsiCertificate<Self>>, PcurveCertifyError> {
        fitted_lane(carrier, t0, t1, image, surface, mate, band)
    }

    fn general_image(
        carrier: &NurbsCurve3<Self>,
        wall: &NurbsSurface<Self>,
    ) -> Result<Option<NurbsCurve2<Self>>, PcurveCertifyError> {
        general_image_lane(carrier, wall)
    }

    fn chart_foot(
        point: Point3<Self>,
        wall: &NurbsSurface<Self>,
    ) -> Result<Option<Point2<f64>>, PcurveCertifyError> {
        chart_foot_lane(point, wall)
    }

    fn lane_name() -> &'static str {
        "interval"
    }
}

/// The dual lane: STATICALLY no fitted certificate — this impl
/// instantiates none of the certified machinery (trait docs). The
/// caller turns the `None` into
/// [`PcurveCertifyError::FittedLaneUnsupported`]; a dual body simply
/// never carries a fitted cache, because one cannot be built there.
impl<T> PcurveFittedLane for geom_core::Dual<T>
where
    geom_core::Dual<T>: Decide,
{
    fn fitted_certificate(
        _carrier: &Curve3<Self>,
        _t0: Self,
        _t1: Self,
        _image: &NurbsCurve2<Self>,
        _surface: &Surface<Self>,
        _mate: &Surface<Self>,
        _band: Band,
    ) -> Result<Option<SsiCertificate<Self>>, PcurveCertifyError> {
        Ok(None)
    }

    fn general_image(
        _carrier: &NurbsCurve3<Self>,
        _wall: &NurbsSurface<Self>,
    ) -> Result<Option<NurbsCurve2<Self>>, PcurveCertifyError> {
        Ok(None)
    }

    fn chart_foot(
        _point: Point3<Self>,
        _wall: &NurbsSurface<Self>,
    ) -> Result<Option<Point2<f64>>, PcurveCertifyError> {
        Ok(None)
    }

    fn lane_name() -> &'static str {
        "dual"
    }
}

/// A certified pcurve cache: the chart image, the carrier-parameter
/// interval it is certified over, and the [`PcurveCertificate`] of the
/// run. Constructible only through [`PcurveCache::certify`] — the
/// fields are private, so an uncertified pcurve is unrepresentable
/// (D4 ¶2 made structural, exactly as for [`crate::EdgeCurve`]).
///
/// `Clone`, not `Copy`, for the reason [`Pcurve`] is not.
#[derive(Clone, Debug)]
pub struct PcurveCache<T: Real> {
    pcurve: Pcurve<T>,
    param_start: T,
    param_end: T,
    certificate: PcurveCertificate<T>,
}

impl<T: Real> PcurveCache<T> {
    /// The cached chart image.
    pub fn pcurve(&self) -> &Pcurve<T> {
        &self.pcurve
    }

    /// The carrier-parameter interval `(t₀, t₁)` this pcurve is
    /// certified over — the edge's own interval (spec §2).
    pub fn params(&self) -> (T, T) {
        (self.param_start, self.param_end)
    }

    /// The certification record of the run that admitted this cache.
    pub fn certificate(&self) -> &PcurveCertificate<T> {
        &self.certificate
    }
}

impl<T: Decide> PcurveCache<T> {
    /// Certifies a **closed-form** [`Pcurve::Harmonic`] image of
    /// `carrier` on `surface` over `[t0, t1]`, inside the face's chart
    /// `window` — the minting lane's door, at every `Decide` scalar.
    ///
    /// A [`Pcurve::Fitted`] image refuses here, naming
    /// [`PcurveCache::certify_fitted`]: the fitted lane's certificate
    /// is the SSI one, which needs the mate operand and a CERTIFYING
    /// scalar (`Decide + Bounds + CertifiedEnclosure`), and hiding those
    /// behind this signature
    /// would put the whole minting pipeline behind a bound it does not
    /// need (a dual body mints closed-form pcurves perfectly well).
    ///
    /// The check sequence (fixed order, D9; every margin in metres):
    ///
    /// 1. **Lane**: the chart and carrier kinds are ones this module
    ///    certifies in closed form; the azimuth channel of a periodic
    ///    chart is `α + β·t` with `β ∈ {−1, 0, +1}` (a named trilean
    ///    selection over a finite structural set, not a guess).
    /// 2. **Interval**: `t₁ − t₀` is definitely forward, metered
    ///    through the carrier's own rate; the pcurve's azimuth extent
    ///    does not definitely exceed one period.
    /// 3. **Schedule**: `|S(P(tᵢ)) − C(tᵢ)| ≤ ε` at the
    ///    [`CERT_SAMPLES`] schedule — evaluated through
    ///    `Surface::eval` and `Curve3::eval` directly, so the harmonic
    ///    decomposition step 4 uses is *verified*, never trusted.
    /// 4. **Envelope**: the between-samples sup bound over the whole
    ///    span ≤ ε, by the lane the variant selects.
    ///    - [`Pcurve::Harmonic`]: the closed-form bound (module docs),
    ///      **plus the winding snap's slack** — step 1 admits an
    ///      ε-shell around the exact harmonic family, and the stored
    ///      envelope must bound the pcurve that was actually certified,
    ///      not the snapped one the closed form describes. Zero on
    ///      every minted cache (they are exact in family); the term
    ///      exists so the certificate is honest for every input
    ///      `certify` admits, including attach-path ones.
    /// 5. **Trim containment**: the pcurve's chart box lies inside
    ///    `window`.
    ///
    /// # Errors
    ///
    /// The first failing check, as a typed [`PcurveCertifyError`];
    /// [`PcurveCertifyError::UnsupportedCarrier`] for a fitted image
    /// offered to this door.
    pub fn certify(
        pcurve: Pcurve<T>,
        t0: T,
        t1: T,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        window: ChartWindow<T>,
        band: Band,
    ) -> Result<Self, PcurveCertifyError> {
        let certificate = match &pcurve {
            Pcurve::Fitted(_) | Pcurve::General(_) => {
                return Err(PcurveCertifyError::UnsupportedCarrier);
            }
            // The iso lane (M6-3): closed-form like the harmonic one —
            // no mate operand, no bracket obligation — so it shares
            // this `Decide`-scalar door.
            Pcurve::IsoLine { p0, pl } => {
                run_iso_checks(*p0, *pl, t0, t1, carrier, surface, window, band)?
            }
            Pcurve::IsoArc {
                p0,
                pd,
                t0: at0,
                angle,
                breaks,
            } => run_iso_arc_checks(
                *p0, *pd, *at0, *angle, breaks, t0, t1, carrier, surface, window, band,
            )?,
            // EXHAUSTIVE by variant, never a catch-all (D3): a
            // catch-all here would route a NEW closed-form variant
            // into the harmonic checker silently, and the build would
            // not say a word. Adding a variant must be a
            // compiler-guided edit at every dispatch site.
            harmonic @ Pcurve::Harmonic { .. } => {
                run_harmonic_checks(harmonic, t0, t1, carrier, surface, window, band)?
            }
        };
        Ok(Self {
            pcurve,
            param_start: t0,
            param_end: t1,
            certificate,
        })
    }
}

impl<T: PcurveFittedLane> PcurveCache<T> {
    /// Certifies a **fitted** (rung-3) chart image.
    ///
    /// **This door has no `src` caller** — the certified route exists,
    /// and no kernel constructor mints a `Fitted` cache into a body.
    /// It is nonetheless the lane's only callerless ITEM: the rest is
    /// reached through [`PcurveCache::recertify`], whose `Fitted` arm
    /// the tier-3 validator dispatches per half-edge, which is why
    /// `topo::validate_pcurves` carries the [`PcurveFittedLane`] bound
    /// at all. That arm cannot execute on a body this workspace
    /// builds, since this door is the variant's sole origin; it is
    /// live for a caller who attaches a `Fitted` cache through
    /// `topo::Body::attach_pcurve`.
    ///
    /// Three consumers are waiting on it, in decreasing firmness:
    ///
    /// 1. **Mint-side wiring of the general-circle route** — the
    ///    oblique-trihedron octant faces whose boundary circles are
    ///    GENERAL sphere circles stay legally uncached. The BOUND is no
    ///    longer what blocks it: `topo::mint_pcurves` carries
    ///    [`PcurveFittedLane`] since PCURVE P-2 (#498), which wired
    ///    [`PcurveCache::certify_general`] through it. What is left is
    ///    this door's own wiring for a Circle carrier, which no mint
    ///    site reaches. Named as an open frontier in `docs/DESIGN.md`,
    ///    and in **no** milestone plan and no carried-items register.
    /// 2. The cyl×sphere germ-chord lane, banked with the join-lane
    ///    analog.
    ///
    /// The `General` curve-in-UV arm of the ratified pcurve unification
    /// (`docs/PCURVE-UNIFY-DESIGN.md` U2) is no longer among them: it
    /// certifies through [`PcurveCache::certify_general`] beside this
    /// door, and `topo::mint_pcurves` mints it.
    ///
    /// Same five checks in the same fixed order as
    /// [`PcurveCache::certify`], with two differences that are the
    /// whole content of the lane: check 1 admits a `Curve3::Nurbs`
    /// carrier (the closed-form lane's `UnsupportedCarrier` retires for
    /// this class), and check 4 is the **full C2 certificate** — hull
    /// sup-norm AND uniqueness tube — derived through
    /// `geom_brep::ssi::certify` against the operand pair
    /// (`surface`, `mate`). [`PcurveCertificate::statement`] records
    /// which sup the resulting envelope bounds.
    ///
    /// `mate` is the other operand of the pair whose intersection
    /// minted the carrier: the uniqueness tube is a statement about the
    /// PAIR, so a single surface cannot produce one. It is a parameter
    /// rather than stored data precisely so that re-certification
    /// re-reads the body's own geometry.
    ///
    /// # Errors
    ///
    /// The first failing check, as a typed [`PcurveCertifyError`].
    #[allow(clippy::too_many_arguments)] // one parameter per named quantity
    pub fn certify_fitted(
        image: Arc<NurbsCurve2<T>>,
        t0: T,
        t1: T,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        mate: Option<&Surface<T>>,
        window: ChartWindow<T>,
        band: Band,
    ) -> Result<Self, PcurveCertifyError> {
        let certificate = run_fitted_checks(&image, t0, t1, carrier, surface, mate, window, band)?;
        Ok(Self {
            pcurve: Pcurve::Fitted(image),
            param_start: t0,
            param_end: t1,
            certificate,
        })
    }

    /// Certifies a **general curve-in-UV** ([`Pcurve::General`], U2's
    /// arm) at the FITTED GRADE: the identical five checks in the
    /// identical order as [`PcurveCache::certify_fitted`], against the
    /// identical `(surface, mate)` operand pair, producing the
    /// identical C2 certificate.
    ///
    /// The two doors are separate because their ENTRY requirements
    /// differ, not their statements: `certify_fitted` is entered by a
    /// caller who can assert the OQ4 construction identity, this one
    /// by a caller who cannot. Nothing here is weaker as a
    /// consequence — the certificate is measured either way — so the
    /// three outcomes are the fitted lane's verbatim:
    ///
    /// - **certify**: every sampled map residual is coincident with
    ///   zero, the hull sup bound is within ε, and the uniqueness
    ///   tube is definitely positive;
    /// - **refuse**: [`PcurveCertifyError::FittedCertificate`] (a
    ///   definite limb failure) or [`PcurveCertifyError::
    ///   IntervalNotForward`];
    /// - **escalate**: [`PcurveCertifyError::Escalated`] /
    ///   [`PcurveCertifyError::FittedEscalated`] (a sliver-band
    ///   verdict), [`PcurveCertifyError::FittedMateMissing`] (no
    ///   operand pair to state a tube about), or
    ///   [`PcurveCertifyError::FittedLaneUnsupported`] (a scalar with
    ///   no exact-ring hull).
    ///
    /// # Errors
    ///
    /// The first failing check, as a typed [`PcurveCertifyError`].
    #[allow(clippy::too_many_arguments)] // one parameter per named quantity
    pub fn certify_general(
        image: Arc<NurbsCurve2<T>>,
        t0: T,
        t1: T,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        mate: Option<&Surface<T>>,
        window: ChartWindow<T>,
        band: Band,
    ) -> Result<Self, PcurveCertifyError> {
        let certificate = run_fitted_checks(&image, t0, t1, carrier, surface, mate, window, band)?;
        Ok(Self {
            pcurve: Pcurve::General(image),
            param_start: t0,
            param_end: t1,
            certificate,
        })
    }

    /// Re-runs the full certification at rest — the tier-3 validator's
    /// per-half-edge pass, for EITHER lane. Same checks, same schedule,
    /// same errors; the stored certificate is not consulted
    /// (re-certification re-derives, it does not trust — and for a
    /// fitted cache that means re-deriving the whole C2 certificate,
    /// hull bound and uniqueness tube included).
    ///
    /// # Errors
    ///
    /// As [`PcurveCache::certify`] / [`PcurveCache::certify_fitted`].
    pub fn recertify(
        &self,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        mate: Option<&Surface<T>>,
        window: ChartWindow<T>,
        band: Band,
    ) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
        match &self.pcurve {
            Pcurve::Fitted(image) | Pcurve::General(image) => run_fitted_checks(
                image,
                self.param_start,
                self.param_end,
                carrier,
                surface,
                mate,
                window,
                band,
            ),
            Pcurve::IsoLine { p0, pl } => run_iso_checks(
                *p0,
                *pl,
                self.param_start,
                self.param_end,
                carrier,
                surface,
                window,
                band,
            ),
            Pcurve::IsoArc {
                p0,
                pd,
                t0,
                angle,
                breaks,
            } => run_iso_arc_checks(
                *p0,
                *pd,
                *t0,
                *angle,
                breaks,
                self.param_start,
                self.param_end,
                carrier,
                surface,
                window,
                band,
            ),
            // Exhaustive by variant (see `certify`).
            harmonic @ Pcurve::Harmonic { .. } => run_harmonic_checks(
                harmonic,
                self.param_start,
                self.param_end,
                carrier,
                surface,
                window,
                band,
            ),
        }
    }
}

/// The carrier parameter at schedule sample `i` — bitwise the schedule
/// [`crate::EdgeCurve::sample_param`] uses (D9: one schedule, shared).
fn sample_param<T: Real>(t0: T, t1: T, i: u32) -> T {
    let frac = T::from_f64(f64::from(i) / f64::from(CERT_SAMPLES - 1));
    t0 + (t1 - t0) * frac
}

/// A 3-D curve in the certified basis: `c + a·cos t + b·sin t + l·t`.
/// Both `S ∘ P` and `C` land here for every pair in the lane (module
/// docs), which is what makes the envelope closed-form.
#[derive(Clone, Copy, Debug)]
struct Harmonic3<T: Real> {
    c: Point3<T>,
    a: Vec3<T>,
    b: Vec3<T>,
    l: Vec3<T>,
}

/// The carrier in the certified basis. Total for the analytic kinds;
/// `Nurbs` carriers are refused at the lane check.
fn carrier_harmonic<T: Real>(carrier: &Curve3<T>) -> Option<Harmonic3<T>> {
    match *carrier {
        Curve3::Line { origin, dir } => Some(Harmonic3 {
            c: origin,
            a: Vec3::zero(),
            b: Vec3::zero(),
            l: dir,
        }),
        Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => Some(Harmonic3 {
            c: center,
            a: u_ref * radius,
            b: axis.cross(u_ref) * radius,
            l: Vec3::zero(),
        }),
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => Some(Harmonic3 {
            c: center,
            a: u_ref * major,
            b: axis.cross(u_ref) * minor,
            l: Vec3::zero(),
        }),
        Curve3::Nurbs(_) => None,
    }
}

/// The periodic chart's azimuth **winding**: the finite structural set
/// `{−1, 0, +1}` a certified chart image may traverse per unit carrier
/// parameter. Structure, not a scalar — so the branch on it below is a
/// match on a named decision, never a comparison on `T`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Winding {
    /// `β = −1`: the chart azimuth runs against the chart frame.
    Neg,
    /// `β = 0`: a constant azimuth — a meridian.
    Zero,
    /// `β = +1`: the chart azimuth runs with the chart frame.
    Pos,
}

impl Winding {
    fn value<T: Real>(self) -> T {
        match self {
            Winding::Neg => T::zero() - T::one(),
            Winding::Zero => T::zero(),
            Winding::Pos => T::one(),
        }
    }

    const ALL: [Winding; 3] = [Winding::Neg, Winding::Zero, Winding::Pos];
}

/// The chart image `S ∘ P` in the certified basis.
///
/// - **Plane chart**: the map is affine, so the pcurve's coefficients
///   map through one by one.
/// - **Cylinder chart**: the azimuth channel is `α + β·t` with
///   `β ∈ {−1, 0, +1}` (named by [`chart_windings`]). For `β = 0` the
///   radial vector is constant and folds into the constant term; for
///   `β = ±1` the angle-sum identity
///   `cos(α + βt) = cos α·cos t − β·sin α·sin t` puts the radial
///   channel exactly in the basis.
fn chart_image_harmonic<T: Real>(
    pcurve: &Pcurve<T>,
    surface: &Surface<T>,
    windings: ChartWindings,
) -> Option<Harmonic3<T>> {
    // The closed-form image is the closed-form lane's business; a
    // fitted image has no harmonic decomposition by construction.
    let Pcurve::Harmonic { p0, pa, pb, pl } = *pcurve else {
        return None;
    };
    let winding = windings.u;
    match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let v_ref = normal.cross(u_ref);
            Some(Harmonic3 {
                c: origin + (u_ref * p0.x + v_ref * p0.y),
                a: u_ref * pa.x + v_ref * pa.y,
                b: u_ref * pb.x + v_ref * pb.y,
                l: u_ref * pl.x + v_ref * pl.y,
            })
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let cv = axis.cross(u_ref);
            let (sa, ca) = p0.x.sin_cos();
            // radial(α) and tangential(α) in the chart frame.
            let rad = u_ref * ca + cv * sa;
            let tang = u_ref * (T::zero() - sa) + cv * ca;
            match winding {
                Winding::Zero => Some(Harmonic3 {
                    c: origin + rad * radius + axis * p0.y,
                    a: axis * pa.y,
                    b: axis * pb.y,
                    l: axis * pl.y,
                }),
                Winding::Pos | Winding::Neg => Some(Harmonic3 {
                    c: origin + axis * p0.y,
                    a: rad * radius + axis * pa.y,
                    b: tang * (radius * winding.value()) + axis * pb.y,
                    l: axis * pl.y,
                }),
            }
        }
        // The M6-3 completion (walk row 4): the cone/sphere/torus
        // closed-form tables. Every arm below is the SNAPPED image —
        // affine angular channels with winding-valued slopes — and the
        // drift the snap discarded is carried back into the envelope
        // by check 4's snap slacks, exactly the cylinder discipline.
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let (s_ha, c_ha) = half_angle.sin_cos();
            let cv = axis.cross(u_ref);
            let (sa, ca) = p0.x.sin_cos();
            let rad = u_ref * ca + cv * sa;
            match winding {
                // Fixed azimuth: S is AFFINE in v along the ruling
                // direction, so any harmonic v channel maps through
                // exactly, coefficient by coefficient (the slant-line
                // meridian class, `v` a length).
                Winding::Zero => {
                    let d = axis * c_ha + rad * s_ha;
                    Some(Harmonic3 {
                        c: apex + d * p0.y,
                        a: d * pa.y,
                        b: d * pb.y,
                        l: d * pl.y,
                    })
                }
                // Moving azimuth: the rim-circle class at constant
                // slant v₀ — radius v₀·sin α about the axis point at
                // height v₀·cos α.
                Winding::Pos | Winding::Neg => {
                    let tang = u_ref * (T::zero() - sa) + cv * ca;
                    let rho = p0.y * s_ha;
                    Some(Harmonic3 {
                        c: apex + axis * (p0.y * c_ha),
                        a: rad * rho,
                        b: tang * (rho * winding.value()),
                        l: Vec3::zero(),
                    })
                }
            }
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        }
        | Surface::Torus {
            center,
            axis,
            minor_radius: radius,
            u_ref,
            ..
        } => {
            // One body for both charts: the torus IS the sphere's
            // algebra with the spine offset `rad·R` added (R = 0
            // recovers the sphere — the sphere arm passes 0).
            let major = match *surface {
                Surface::Torus { major_radius, .. } => major_radius,
                _ => T::zero(),
            };
            let cv = axis.cross(u_ref);
            let (sa, ca) = p0.x.sin_cos();
            let rad = u_ref * ca + cv * sa;
            let (sd, cd) = p0.y.sin_cos();
            match (winding, windings.v.unwrap_or(Winding::Zero)) {
                // Moving azimuth, constant polar v₀: the PARALLEL /
                // polar-circle class — a circle of radius
                // R + r·cos v₀ (sphere: r·cos v₀) at height r·sin v₀.
                (Winding::Pos | Winding::Neg, Winding::Zero) => {
                    let tang = u_ref * (T::zero() - sa) + cv * ca;
                    let rho = major + radius * cd;
                    Some(Harmonic3 {
                        c: center + axis * (radius * sd),
                        a: rad * rho,
                        b: tang * (rho * winding.value()),
                        l: Vec3::zero(),
                    })
                }
                // Constant azimuth, moving polar (σ = ±1): the
                // MERIDIAN class — cos(δ+σt)/sin(δ+σt) expanded by the
                // angle-sum identity, exactly as the cylinder's
                // azimuth arm.
                (Winding::Zero, sigma @ (Winding::Pos | Winding::Neg)) => {
                    let s = sigma.value::<T>();
                    Some(Harmonic3 {
                        c: center + rad * major,
                        a: rad * (radius * cd) + axis * (radius * sd),
                        b: (axis * cd - rad * sd) * (radius * s),
                        l: Vec3::zero(),
                    })
                }
                // Both channels constant: the image is one point; a
                // genuine carrier never certifies against it (the
                // schedule residuals refuse), so the honest form is
                // returned rather than a refusal invented here.
                (Winding::Zero, Winding::Zero) => Some(Harmonic3 {
                    c: center + rad * (major + radius * cd) + axis * (radius * sd),
                    a: Vec3::zero(),
                    b: Vec3::zero(),
                    l: Vec3::zero(),
                }),
                // Both channels moving: azimuth-NON-harmonic (the
                // sphere's general circles, the torus Villarceau
                // class) — no closed form exists; the fitted lane or a
                // typed refusal owns it.
                (Winding::Pos | Winding::Neg, Winding::Pos | Winding::Neg) => None,
            }
        }
        // A spline chart's image has no harmonic decomposition —
        // neither the payload's nor an approximating surface's fit.
        Surface::Nurbs(_) | Surface::Approx(_) => None,
    }
}

/// Both angular channels' windings for a chart (M6-3, walk row 4): the
/// azimuth channel's `β` always, plus the POLAR/MERIDIONAL channel's
/// `σ` where the second chart parameter is itself an angle (sphere `v`,
/// torus `v`). `None` = the second channel is a length (cylinder/cone
/// `v`, plane both) and its slope is unconstrained.
#[derive(Clone, Copy, Debug)]
struct ChartWindings {
    u: Winding,
    v: Option<Winding>,
}

impl ChartWindings {
    const NONE: Self = Self {
        u: Winding::Zero,
        v: None,
    };
}

/// The azimuth lever arm of a chart, in metres per radian — the safe
/// OVER-statement per kind (module docs' sphere note): cylinder `r`,
/// sphere `r` (true arm `r·cos v ≤ r`), torus `R + r` (the outer
/// equator's), cone `v_sup·sin α` — the cone has no surface-level
/// constant, so the caller supplies the pcurve's/window's own `|v|`
/// sup, which dominates the local arm `v·sin α` everywhere the object
/// being metered lives.
fn azimuth_lever<T: Real>(surface: &Surface<T>, v_sup: T) -> T {
    match *surface {
        Surface::Cylinder { radius, .. } | Surface::Sphere { radius, .. } => radius,
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => major_radius + minor_radius,
        Surface::Cone { half_angle, .. } => v_sup * half_angle.sin(),
        // Non-periodic charts have no azimuth: plane, spline payload,
        // and an approximating surface's fitted chart alike.
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => T::one(),
    }
}

/// Names the chart's angular windings for the pcurve's linear channels:
/// selection over the finite structural set [`Winding::ALL`] by named
/// trileans metered at the chart's own lever arms (metres — an angular
/// slope is dimensionless, so it is metered through the lever arm,
/// D4 ¶1; no UV-space tolerance is ever compared against ε). `reach`
/// is `max(|t₀|, |t₁|)`, the cone's azimuth lever needing the
/// pcurve's own `v` reach.
fn chart_windings<T: Decide>(
    pcurve: &Pcurve<T>,
    surface: &Surface<T>,
    reach: T,
    band: Band,
) -> Result<ChartWindings, PcurveCertifyError> {
    let Pcurve::Harmonic { p0, pa, pb, pl } = *pcurve else {
        return Err(PcurveCertifyError::UnsupportedCarrier);
    };
    // Which arms, per chart kind: the azimuth arm always exists on a
    // periodic chart; the v arm exists exactly where v is an angle.
    let (u_arm, v_arm) = match *surface {
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            // Non-periodic charts have no winding; the value is unused.
            return Ok(ChartWindings::NONE);
        }
        Surface::Cylinder { radius, .. } => (radius, None),
        Surface::Sphere { radius, .. } => (radius, Some(radius)),
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => (major_radius + minor_radius, Some(minor_radius)),
        Surface::Cone { .. } => {
            let v_sup = p0.y.abs() + pa.y.abs() + pb.y.abs() + pl.y.abs() * reach;
            (azimuth_lever(surface, v_sup), None)
        }
    };
    let esc = |cause: Indeterminate| PcurveCertifyError::Escalated {
        check: PcurveCheck::ChartWinding,
        sample: 0,
        cause,
    };
    let classify = |trig: [T; 2],
                    slope: T,
                    arm: T,
                    affine_name: &'static str,
                    winding_name: &'static str|
     -> Result<Winding, PcurveCertifyError> {
        // The angular channel carries no trigonometric part in this
        // lane.
        for coeff in trig {
            match decide(affine_name, Margin::levered(coeff, arm), band).map_err(esc)? {
                Sign::Zero => {}
                Sign::Positive | Sign::Negative => {
                    return Err(PcurveCertifyError::ChartWindingUnsupported);
                }
            }
        }
        for candidate in Winding::ALL {
            match decide(
                winding_name,
                Margin::levered(slope - candidate.value(), arm),
                band,
            ) {
                Ok(Sign::Zero) => return Ok(candidate),
                Ok(Sign::Positive | Sign::Negative) => {}
                Err(cause) => return Err(esc(cause)),
            }
        }
        Err(PcurveCertifyError::ChartWindingUnsupported)
    };
    let u = classify(
        [pa.x, pb.x],
        pl.x,
        u_arm,
        "pcurve_chart_azimuth_affine",
        "pcurve_chart_winding",
    )?;
    let v = match v_arm {
        None => None,
        Some(arm) => Some(classify(
            [pa.y, pb.y],
            pl.y,
            arm,
            "pcurve_chart_polar_affine",
            "pcurve_chart_polar_winding",
        )?),
    };
    Ok(ChartWindings { u, v })
}

/// The rate that meters this carrier's parameter into metres — the
/// lever arm the forward-span and period checks use (the
/// [`crate::certify`] convention: radians × the conservative radius).
/// A NURBS net answers its certified speed lower bound, the same
/// meter [`crate::certify`]'s span check uses, so a parameter span
/// crosses to model space through the kind's real metric everywhere.
/// The bound is POISON on a degenerate net, so the two lanes a NURBS
/// carrier can reach — fitted and iso — gate it through
/// [`param_rate_gate`] before metering anything through it. The
/// harmonic and ARC-RIM lanes take the rate bare because neither can
/// receive one: `carrier_harmonic` answers `None` for a net, and the
/// ARC-RIM class refuses any carrier that is not a `Curve3::Circle`.
/// A new caller that CAN see a net must take the gate.
fn param_rate<T: Real>(carrier: &Curve3<T>) -> T {
    match *carrier {
        Curve3::Line { .. } => T::one(),
        Curve3::Nurbs(ref n) => n.speed_lower_bound(),
        Curve3::Circle { radius, .. } => radius,
        Curve3::Ellipse { minor, .. } => minor,
    }
}

/// The collapsed-arm gate for [`param_rate`] (the
/// [`crate::certify`] idiom, `enters`' shape): the rate is metres per
/// parameter unit, so what a classifier may see is the LENGTH it
/// subtends over the carrier's own parameter extent — a NURBS net's
/// knot-domain length, and the unit parameter the closed-form kinds
/// state their rate over. A collapsed (zero/negative) or poison rate
/// cannot convert a span to metres and no forward verdict may be
/// fabricated from one; refusing here keeps that failure distinct
/// from a backwards span, which the metered span check names.
///
/// # Errors
///
/// [`Indeterminate`] carrying [`geom_core::MarginDiag::Invalid`] when
/// the subtended length is not definitely positive; the classifier's
/// own escalation otherwise.
fn param_rate_gate<T: Decide>(carrier: &Curve3<T>, band: Band) -> Result<T, Indeterminate> {
    let rate = param_rate(carrier);
    let extent = match *carrier {
        Curve3::Nurbs(ref n) => {
            let (d0, d1) = n.domain();
            T::from_f64(d1 - d0)
        }
        _ => T::one(),
    };
    match decide("pcurve_interval_meter", Margin::metered(extent, rate), band)? {
        Sign::Positive => Ok(rate),
        Sign::Zero | Sign::Negative => Err(Indeterminate {
            margin: geom_core::MarginDiag::Invalid,
            band,
            predicate: Some("pcurve_interval_meter"),
        }),
    }
}

/// Folds a residual into the running max and classifies it against the
/// band (the `certify::check_residual` idiom, one module over).
fn check_residual<T: Decide>(
    name: &'static str,
    check: PcurveCheck,
    sample: u32,
    residual: Margin<T>,
    band: Band,
    max_residual: &mut T,
) -> Result<(), PcurveCertifyError> {
    *max_residual = max_residual.max(residual.value().abs());
    match decide(name, residual, band) {
        Ok(Sign::Zero) => Ok(()),
        Ok(Sign::Positive | Sign::Negative) => {
            Err(PcurveCertifyError::ResidualExceeded { check, sample })
        }
        Err(cause) => Err(PcurveCertifyError::Escalated {
            check,
            sample,
            cause,
        }),
    }
}

/// The shared pcurve-certification engine (sequence documented on
/// [`PcurveCache::certify`]).
fn run_harmonic_checks<T: Decide>(
    pcurve: &Pcurve<T>,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    window: ChartWindow<T>,
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    // ---- Check 1: the certified lane. ----
    let chart = chart_name(surface);
    // The closed-form lane is the analytic charts'; a spline chart —
    // the payload's or an approximating surface's fit — goes through
    // the fitted lane instead.
    if matches!(surface, Surface::Nurbs(_) | Surface::Approx(_)) {
        return Err(PcurveCertifyError::UnsupportedChart { chart });
    }
    let Some(carrier_form) = carrier_harmonic(carrier) else {
        return Err(PcurveCertifyError::UnsupportedCarrier);
    };
    let reach = t0.abs().max(t1.abs());
    let windings = chart_windings(pcurve, surface, reach, band)?;
    let winding = windings.u;
    let Some(image_form) = chart_image_harmonic(pcurve, surface, windings) else {
        return Err(PcurveCertifyError::UnsupportedChart { chart });
    };

    let mut max_residual = T::zero();

    // ---- Check 2: the parameter interval, metered into metres. ----
    let rate = param_rate(carrier);
    let span = t1 - t0;
    let span_escalated = |cause: Indeterminate| PcurveCertifyError::Escalated {
        check: PcurveCheck::ParamSpan,
        sample: 0,
        cause,
    };
    match decide("pcurve_interval_forward", Margin::metered(span, rate), band)
        .map_err(span_escalated)?
    {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(PcurveCertifyError::IntervalNotForward),
    }
    // The chart-side winding gates: the angular extent of a periodic
    // chart's pcurve, metered at the chart's lever arm — the azimuth
    // channel on every periodic chart, and the polar/meridional
    // channel too where it is itself an angle (sphere/torus).
    if !matches!(
        surface,
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_)
    ) {
        let Pcurve::Harmonic { p0, pa, pb, pl } = *pcurve else {
            return Err(PcurveCertifyError::UnsupportedCarrier);
        };
        let v_sup = p0.y.abs() + pa.y.abs() + pb.y.abs() + pl.y.abs() * reach;
        let mut gates = vec![(pl.x, azimuth_lever(surface, v_sup))];
        match *surface {
            Surface::Sphere { radius, .. } => gates.push((pl.y, radius)),
            Surface::Torus { minor_radius, .. } => gates.push((pl.y, minor_radius)),
            _ => {}
        }
        for (slope, arm) in gates {
            let extent = (slope * span).abs();
            let headroom = Margin::levered(T::tau() - extent, arm);
            match decide("pcurve_azimuth_period", headroom, band).map_err(|cause| {
                PcurveCertifyError::Escalated {
                    check: PcurveCheck::AzimuthPeriod,
                    sample: 0,
                    cause,
                }
            })? {
                Sign::Positive | Sign::Zero => {}
                Sign::Negative => return Err(PcurveCertifyError::AzimuthPeriodExceeded),
            }
        }
    }

    // ---- Check 3: the schedule, in metres through the map. ----
    schedule_residuals(pcurve, t0, t1, carrier, surface, band, &mut max_residual)?;

    // ---- Check 4: the closed-form between-samples envelope. ----
    let d_c = image_form.c - carrier_form.c;
    let d_a = image_form.a - carrier_form.a;
    let d_b = image_form.b - carrier_form.b;
    let d_l = image_form.l - carrier_form.l;
    // **The snap slack.** Check 1's winding trilean classifies
    // `|pa.x|·r`, `|pb.x|·r` and `|pl.x − β|·r` as Zero anywhere inside
    // the band, so `certify` admits pcurves in an ε-shell OUTSIDE the
    // exact harmonic family — and `image_form` above is built from the
    // SNAPPED azimuth channel `α + β·t`. The envelope of the snapped
    // image would therefore under-report the true sup of the pcurve
    // actually being certified, by exactly the drift the snap discarded
    // (measured at 7 orders on an attach-path `pl.x = 1 + 0.6e-9` — the
    // reviewer's probe, now `envelope_dominates_a_winding_snapped_pcurve`).
    // Add it back: the discarded channel is
    // `δu(t) = pa.x·cos t + pb.x·sin t + (pl.x − β)·t`, and moving the
    // azimuth by `δu` moves the mapped point by `2r·|sin(δu/2)| ≤ r·|δu|`
    // — so `r·(|pa.x| + |pb.x| + |pl.x − β|·reach)` bounds it. Minted
    // caches are exact in family (`pa.x = pb.x = 0`, `pl.x ∈ {−1,0,1}`
    // bitwise), so this term is exactly zero on the ship path.
    // Per angular channel: the discarded drift metered at that
    // channel's lever arm (azimuth: the chart's safe over-arm; polar:
    // r / minor r; the cone's snapped-constant v: the unit ruling
    // arm). Every term is exactly zero on the minted path.
    let snap_slack = match (surface, pcurve) {
        (_, Pcurve::Harmonic { p0, pa, pb, pl })
            if !matches!(
                surface,
                Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_)
            ) =>
        {
            let v_sup = p0.y.abs() + pa.y.abs() + pb.y.abs() + pl.y.abs() * reach;
            let u_arm = azimuth_lever(surface, v_sup);
            let u_slack =
                (pa.x.abs() + pb.x.abs() + (pl.x - winding.value::<T>()).abs() * reach) * u_arm;
            let v_slack = match *surface {
                // The v channel maps exactly on these charts wherever
                // the image exists: cylinder always; cone in the
                // fixed-azimuth class. The cone's MOVING-azimuth class
                // snapped v to the constant p0.y — |∂S/∂v| = 1.
                Surface::Cylinder { .. } => T::zero(),
                Surface::Cone { .. } => match winding {
                    Winding::Zero => T::zero(),
                    Winding::Pos | Winding::Neg => pa.y.abs() + pb.y.abs() + pl.y.abs() * reach,
                },
                Surface::Sphere { radius, .. } => {
                    let sigma = windings.v.unwrap_or(Winding::Zero);
                    (pa.y.abs() + pb.y.abs() + (pl.y - sigma.value::<T>()).abs() * reach) * radius
                }
                Surface::Torus { minor_radius, .. } => {
                    let sigma = windings.v.unwrap_or(Winding::Zero);
                    (pa.y.abs() + pb.y.abs() + (pl.y - sigma.value::<T>()).abs() * reach)
                        * minor_radius
                }
                // Non-periodic charts snap nothing in v.
                Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => T::zero(),
            };
            u_slack + v_slack
        }
        _ => T::zero(),
    };
    let envelope = d_c.norm() + d_a.norm() + d_b.norm() + d_l.norm() * reach + snap_slack;
    // The envelope is classified against the band like every other
    // residual, but it is NOT folded into `max_residual`: that field is
    // the sampled max, and the two statements stay separate (the
    // certificate's field docs).
    let mut envelope_margin = T::zero();
    check_residual(
        "pcurve_envelope",
        PcurveCheck::Envelope,
        0,
        Margin::of(envelope),
        band,
        &mut envelope_margin,
    )?;

    // ---- Check 5: trim containment (the chart-box limb). ----
    trim_containment(pcurve, t0, t1, surface, window, band)?;

    Ok(PcurveCertificate {
        samples: CERT_SAMPLES,
        max_residual,
        envelope,
        statement: EnvelopeStatement::MapResidualClosedForm,
        ssi: None,
    })
}

/// The chart kind, named — shared by both lanes' refusal texts.
pub(crate) fn chart_name<T: Real>(surface: &Surface<T>) -> &'static str {
    match surface {
        Surface::Plane { .. } => "plane",
        Surface::Cylinder { .. } => "cylinder",
        Surface::Cone { .. } => "cone",
        Surface::Sphere { .. } => "sphere",
        Surface::Torus { .. } => "torus",
        Surface::Nurbs(_) => "Nurbs",
        Surface::Approx(_) => "Approx",
    }
}

/// **`(sup |S_u|, sup |S_v|)`** — the chart's UPPER stretch bounds:
/// the lever arms that turn a chart-space overshoot into metres, one
/// per chart parameter, valid over the WHOLE chart.
///
/// # What this bounds, and what it must not be used for
///
/// Each component **dominates** the true local stretch everywhere on
/// the chart: `|S_u(u, v)| ≤ arm_u` and `|S_v(u, v)| ≤ arm_v` at
/// every `(u, v)` of the chart's domain. It is therefore
/// **sup-side by construction and is NOT a lower bound**. Quoting it
/// where an `inf` is wanted is unsound, and the two uses split like
/// this:
///
/// - **ESCAPE metering (sound).** A claim of the form *"this
///   chart-space displacement does not move the point out of the
///   band"* — trim containment, loop continuity, an azimuth-period
///   headroom. Over-stating the arm inflates the metred displacement,
///   which can only make the in-band verdict HARDER to obtain: the
///   error direction refuses, it never falsely certifies.
/// - **POSITIVE-extent claims (UNSOUND).** A claim of the form *"this
///   chart-space region has definitely-positive model-space extent"*.
///   An over-stated arm inflates the extent and would certify a
///   model-space sliver as definitely positive. That direction needs
///   certified LOWER bounds — [`chart_stretch_inf`] — which are a
///   different derivation behind a different name; nothing here may
///   be read as one.
///
/// A sphere's true azimuth arm is `r·cos v ≤ r`, so quoting `r`
/// **over**-states the escape and can only make containment harder —
/// the safe direction, and the same posture the cylinder arm takes
/// exactly. A plane chart's parameters are already metres, so its
/// arms are exactly `(1, 1)` by construction rather than by default.
pub fn chart_stretch_sup<T: Real>(surface: &Surface<T>) -> (T, T) {
    match *surface {
        Surface::Cylinder { radius, .. } => (radius, T::one()),
        // The sphere/torus second parameter IS an angle (M6-3): its
        // arm is the polar / meridional radius. The cone keeps unit
        // arms HERE — its true azimuth arm needs a `v` reach no
        // surface-level constant dominates; the containment check
        // supplies it through [`chart_arms_at`].
        Surface::Sphere { radius, .. } => (radius, radius),
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => (major_radius + minor_radius, minor_radius),
        // A described NURBS chart's honest arms are its derivative-net
        // stretch bounds (`sup |S_u|`, `sup |S_v|`) — over-statements
        // of the local stretch, the safe direction exactly as the
        // sphere's.
        //
        // **RATIONAL charts take the same arms since M8-3**, and they
        // must: an arm under-states only in the UNSAFE direction here
        // (`trim_containment` meters an escape, so a smaller arm makes
        // an escape easier to admit), and before M8-3 the unit arms
        // were harmless only because the iso lane's own rational gate
        // refused before any rational chart reached this function.
        // That gate is gone, so the arm has to be real —
        // `nurbs_stretch_bounds` carries the Floater weight-ratio
        // factor for exactly this. The placeholder keeps unit arms: it
        // has no net to bound.
        // The catch-all is SPLIT: an approximating surface's arms are
        // its FIT's derivative-net bounds — the same statement about
        // the same chart. Unit arms would under-state in the unsafe
        // direction here (see the rational note above).
        Surface::Nurbs(ref payload) => {
            if payload.is_placeholder() {
                (T::one(), T::one())
            } else {
                nurbs_stretch_bounds(payload)
            }
        }
        Surface::Approx(ref a) => nurbs_stretch_bounds(a.fit()),
        // A plane chart's parameters are already metres; the cone's
        // arms are the caller's to supply (see the sphere note above).
        Surface::Plane { .. } | Surface::Cone { .. } => (T::one(), T::one()),
    }
}

/// [`chart_stretch_sup`] with the containment check's own boxes in
/// hand: the cone's azimuth arm becomes `v_sup·sin α`, with `v_sup`
/// the larger `|v|` reach of the pcurve's box and the window (dominating the
/// local arm everywhere either object lives — the safe direction);
/// every other kind answers as [`chart_stretch_sup`].
fn chart_arms_at<T: Real>(
    surface: &Surface<T>,
    boxed: &ChartWindow<T>,
    window: &ChartWindow<T>,
) -> (T, T) {
    match *surface {
        Surface::Cone { .. } => {
            let v_sup = boxed
                .v_min
                .abs()
                .max(boxed.v_max.abs())
                .max(window.v_min.abs())
                .max(window.v_max.abs());
            (azimuth_lever(surface, v_sup), T::one())
        }
        _ => chart_stretch_sup(surface),
    }
}

/// `(sup |S_u|, sup |S_v|)` bounds for a **non-rational** NURBS chart,
/// from the derivative control net (the B-spline derivative formula
/// `Qᵢ = p·(Pᵢ₊₁ − Pᵢ)/(kᵢ₊ₚ₊₁ − kᵢ₊₁)` per row/column, then the
/// partition-of-unity hull `max |Qᵢⱼ|`). Knot arithmetic is `f64`
/// structure; a zero divisor (a fully collapsed support) contributes
/// nothing, per the standard convention. Callers gate rationality —
/// with weights ≠ 1 this formula does not bound the true derivative.
fn nurbs_stretch_bounds<T: Real>(s: &geom::NurbsSurface<T>) -> (T, T) {
    let (nu, nv) = s.control_counts();
    // **The rational factor** (M8-3). The control-difference bounds
    // below are POLYNOMIAL convexity facts. For a rational patch the
    // standard extension (Floater 1992, derivatives of rational
    // Bézier/B-spline forms) multiplies them by the weight ratio;
    // squaring it is the conservative reading, and conservative is the
    // SAFE direction for both consumers — a larger arm makes
    // `side_of`'s boundary snap harder to admit and every slack term
    // larger, never the reverse. Exactly 1 for a weight-1 net, so no
    // integral-lane number moves.
    let ratio = weight_ratio_factor::<T>(s.weights());
    let ctl = s.control();
    let mut sup_u = T::zero();
    let (pu, ku) = (s.knots_u().degree(), s.knots_u().knots());
    #[allow(clippy::cast_precision_loss)]
    for i in 0..nu.saturating_sub(1) {
        let denom = ku[i + pu + 1] - ku[i + 1];
        if denom == 0.0 {
            continue;
        }
        let factor = T::from_f64(pu as f64 / denom);
        for j in 0..nv {
            sup_u = sup_u.max((ctl[(i + 1) * nv + j] - ctl[i * nv + j]).norm() * factor);
        }
    }
    let mut sup_v = T::zero();
    let (pv, kv) = (s.knots_v().degree(), s.knots_v().knots());
    #[allow(clippy::cast_precision_loss)]
    for j in 0..nv.saturating_sub(1) {
        let denom = kv[j + pv + 1] - kv[j + 1];
        if denom == 0.0 {
            continue;
        }
        let factor = T::from_f64(pv as f64 / denom);
        for i in 0..nu {
            sup_v = sup_v.max((ctl[i * nv + j + 1] - ctl[i * nv + j]).norm() * factor);
        }
    }
    (sup_u * ratio, sup_v * ratio)
}

/// The INF-side reading of a spline chart's derivative nets — the
/// certified quantities a positive-extent claim needs, where
/// [`chart_stretch_sup`] is the escape side's single sup pair.
///
/// See [`chart_stretch_inf`] for what each field bounds and for the
/// assembly a caller owes before using them as lever arms.
#[derive(Clone, Copy, Debug)]
pub struct ChartStretchInf<T> {
    /// A certified lower bound on `|S_u|` over the WHOLE chart, or
    /// exactly zero when none is certified (the derivative net
    /// crosses zero — a fold, a collapsed row, a stationary column).
    pub inf_u: T,
    /// The same for `|S_v|`.
    pub inf_v: T,
    /// `sup |S_u|` — [`chart_stretch_sup`]'s first component, carried
    /// here so the assembly reads one consistent pair of brackets.
    pub sup_u: T,
    /// The same for `|S_v|`.
    pub sup_v: T,
    /// A certified lower bound on the AREA element `|S_u × S_v|`, or
    /// exactly zero when none is certified. This is the term that
    /// knows about skew: two nearly-parallel derivative columns have
    /// large norms and a vanishing cross product.
    pub area_inf: T,
}

/// **The certified LOWER stretch reading of a chart** — the direction
/// [`chart_stretch_sup`] is explicitly not, for the claims it is
/// explicitly unsafe for.
///
/// # The nets, and the inf read along a direction
///
/// The polynomial derivative nets are the same ones
/// [`nurbs_stretch_bounds`] takes the max over: per direction,
/// `Qᵢⱼ = p·(Pᵢ₊₁ⱼ − Pᵢⱼ)/(kᵢ₊ₚ₊₁ − kᵢ₊₁)`, and `S_u(u, v)` is a
/// **convex combination** of them (partition of unity over the
/// degree-reduced basis; a zero divisor is a support the basis
/// function vanishes on identically, so dropping it is exact on both
/// sides rather than safe on one).
///
/// A convex combination's norm is NOT bounded below by the smallest
/// `|Qᵢⱼ|` — a net whose columns cancel has combinations of every
/// norm down to zero — so the inf is read **along a direction**: for
/// the net's own summed direction `d`, every combination projects to
/// at least `minᵢⱼ (Qᵢⱼ·d)` ([`net_inf`]). A **zero-crossing net** —
/// a wall with a fold — has some entry on the far side of `d`, or no
/// direction at all, and gets exactly zero. That zero is the honest
/// answer and the caller must refuse on it: no positive bound is
/// invented for a net whose derivative genuinely vanishes.
///
/// `area_inf` is the same read on the **pairwise cross products**:
/// `S_u × S_v = Σₐ_b λₐ μ_b (Qᵘₐ × Qᵛ_b)` is a convex combination of
/// them (a product of two convex combinations), so [`net_inf`] over
/// that set lower-bounds the area element. A chart whose two
/// derivative directions can align somewhere gets zero here even
/// though both per-axis infs are healthy — which is exactly the case
/// the per-axis pair alone cannot see.
///
/// **The rational factor, taken OPPOSITE to the sup side.** The
/// rational derivative identity (Floater 1992) writes `S_u` as
/// `Σᵢ γᵢ ΔPᵢ` with non-negative `γ` whose sum lies in
/// `[1/ratio, ratio]` for `ratio = (w_max/w_min)²` — the same factor
/// [`weight_ratio_factor`] computes and [`nurbs_stretch_bounds`]
/// MULTIPLIES by. The two ends of that one bracket are the two sides:
/// the sup multiplies, the inf **divides**, and the area element — a
/// product of two such factors — divides by `ratio²`. A rational
/// chart therefore earns a weaker inf reading than a unit-weight one.
/// That is the conservative direction, and on a unit-weight net every
/// factor is exactly 1, so nothing here moves for the polynomial
/// charts.
///
/// # The assembly a caller owes: per-axis infs are not lever arms
///
/// A chart is not required to be orthogonal, and on a skew chart the
/// two per-axis infs do **not** lower-bound the metric: `S_u du` and
/// `S_v dv` can partially cancel, so a chart displacement can be
/// shorter in metres than either axis' inf suggests. What lower-bounds
/// the metric is the Jacobian's **smallest singular value**, and these
/// five numbers bound it. Normalize the chart by the per-axis infs
/// (`ũ = u·inf_u`, `ṽ = v·inf_v`) so the question is scale-free, and
/// for the normalized Gram matrix write
///
/// ```text
/// T = (sup_u/inf_u)² + (sup_v/inf_v)²      ≥ trace
/// D = (area_inf/(inf_u·inf_v))²            ≤ det
/// λ_min ≥ 2D / (T + √(T² − 4D))            (the stable form of
///                                           (T − √(T²−4D))/2)
/// ```
///
/// — valid because `λ_min` of a 2×2 SPD matrix decreases in the trace
/// and increases in the determinant, so a sup trace with an inf
/// determinant is the conservative corner. The lever arms are then
/// `(inf_u·ρ, inf_v·ρ)` with `ρ = √λ_min ≤ 1`, and on an ORTHOGONAL
/// chart of constant stretch `ρ` is exactly 1, so the arms are the
/// per-axis infs verbatim — anisotropy included.
///
/// The assembly is left to the caller deliberately: its divisions are
/// only well-conditioned once `inf_u` and `inf_v` are **definitely**
/// positive, which is a band question this function has no band to
/// ask.
///
/// Analytic charts do not come through here: every one of them is
/// orthogonal by construction with a closed-form inf, and their arms
/// are window-dependent besides. The tree's other inf-side surface
/// bound, `offset_meters`' `‖S_u × S_v‖` floor, is `f64`-only and
/// reads `patch_bound`'s per-cell hulls, so it is a sharper answer to
/// a narrower question and not reusable here.
pub fn chart_stretch_inf<T: Real>(surface: &Surface<T>) -> ChartStretchInf<T> {
    let zero = ChartStretchInf {
        inf_u: T::zero(),
        inf_v: T::zero(),
        sup_u: T::zero(),
        sup_v: T::zero(),
        area_inf: T::zero(),
    };
    match *surface {
        Surface::Nurbs(ref payload) => {
            if payload.is_placeholder() {
                // No net to bound: the placeholder certifies nothing.
                zero
            } else {
                nurbs_stretch_inf(payload)
            }
        }
        Surface::Approx(ref a) => nurbs_stretch_inf(a.fit()),
        // The analytic charts' infs are closed-form and window-
        // dependent; this door answers about derivative NETS only, and
        // an all-zero reading is the refusing answer, never a claim.
        Surface::Plane { .. }
        | Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Torus { .. } => zero,
    }
}

/// A certified lower bound on `|Σ λₐ Qₐ|` over every convex
/// combination of a derivative net, and the direction it is read
/// along.
///
/// The net's own **sum** `c = Σ Qₐ` names the direction: for the unit
/// `d = c/|c|`, every convex combination satisfies
/// `|Σ λₐ Qₐ| ≥ (Σ λₐ Qₐ)·d ≥ minₐ (Qₐ·d)`, which is
/// `minₐ (Qₐ·c) / |c|`. A net that CROSSES ZERO fails this at one of
/// two places and gets exactly zero either way: a net straddling the
/// origin has some `Qₐ·d` negative, and a net whose entries cancel
/// outright has `|c| = 0`, whose quotient is poison. The answer is
/// capped at the net's own `sup`, which is finite by construction, so
/// an ill-conditioned quotient can never leak an inflated arm.
fn net_inf<T: Real>(q: &[Vec3<T>], sup: T) -> T {
    if q.is_empty() {
        return T::zero();
    }
    let mut c = Vec3::zero();
    for &v in q {
        c = c + v;
    }
    let mut min_dot = q[0].dot(c);
    for &v in &q[1..] {
        min_dot = min_dot.min(v.dot(c));
    }
    let raw = min_dot / c.norm();
    if raw.is_poison() {
        T::zero()
    } else {
        raw.max(T::zero()).min(sup)
    }
}

/// The `u`-direction derivative control net of a spline patch:
/// `Qᵢⱼ = p·(Pᵢ₊₁ⱼ − Pᵢⱼ)/(kᵢ₊ₚ₊₁ − kᵢ₊₁)`, degenerate supports
/// dropped (their basis function is identically zero, so the drop is
/// exact on both sides rather than safe on one). Pass `along_v` to
/// read the `v` direction through the same code.
fn derivative_net<T: Real>(s: &geom::NurbsSurface<T>, along_v: bool) -> Vec<Vec3<T>> {
    let (nu, nv) = s.control_counts();
    let ctl = s.control();
    let (n_out, n_in) = if along_v { (nv, nu) } else { (nu, nv) };
    let knots = if along_v { s.knots_v() } else { s.knots_u() };
    let (p, k) = (knots.degree(), knots.knots());
    let at = |i: usize, j: usize| -> geom_core::Point3<T> {
        if along_v {
            ctl[j * nv + i]
        } else {
            ctl[i * nv + j]
        }
    };
    let mut out = Vec::with_capacity(n_out.saturating_sub(1) * n_in);
    #[allow(clippy::cast_precision_loss)]
    for i in 0..n_out.saturating_sub(1) {
        let denom = k[i + p + 1] - k[i + 1];
        if denom == 0.0 {
            continue;
        }
        let factor = T::from_f64(p as f64 / denom);
        for j in 0..n_in {
            out.push((at(i + 1, j) - at(i, j)) * factor);
        }
    }
    out
}

/// [`chart_stretch_inf`]'s spline arm — the derivation lives there.
fn nurbs_stretch_inf<T: Real>(s: &geom::NurbsSurface<T>) -> ChartStretchInf<T> {
    let ratio = weight_ratio_factor::<T>(s.weights());
    let (q_u, q_v) = (derivative_net(s, false), derivative_net(s, true));
    let sup_of = |q: &[Vec3<T>]| q.iter().fold(T::zero(), |m, v| m.max(v.norm()));
    let (poly_sup_u, poly_sup_v) = (sup_of(&q_u), sup_of(&q_v));
    let mut crosses = Vec::with_capacity(q_u.len() * q_v.len());
    for &a in &q_u {
        for &b in &q_v {
            crosses.push(a.cross(b));
        }
    }
    let cross_sup = sup_of(&crosses);
    // Each bracket taken at the end the direction needs: the sups
    // multiply (as `nurbs_stretch_bounds` does), the infs divide, and
    // the area element carries one factor per net.
    ChartStretchInf {
        inf_u: net_inf(&q_u, poly_sup_u) / ratio,
        inf_v: net_inf(&q_v, poly_sup_v) / ratio,
        sup_u: poly_sup_u * ratio,
        sup_v: poly_sup_v * ratio,
        area_inf: net_inf(&crosses, cross_sup) / ratio.powi(2),
    }
}

/// `sup |C′|` bound for a **non-rational** spline curve — the curve
/// twin of [`nurbs_stretch_bounds`], used by the iso lane's
/// parameter-map slack. Callers gate rationality.
fn curve_rate_bound<T: Real>(c: &NurbsCurve3<T>) -> T {
    let ctl = c.control();
    let (p, k) = (c.knots().degree(), c.knots().knots());
    let mut sup = T::zero();
    #[allow(clippy::cast_precision_loss)]
    for i in 0..ctl.len().saturating_sub(1) {
        let denom = k[i + p + 1] - k[i + 1];
        if denom == 0.0 {
            continue;
        }
        sup = sup.max((ctl[i + 1] - ctl[i]).norm() * T::from_f64(p as f64 / denom));
    }
    // The rational factor (M8-3), the curve-side twin of
    // `nurbs_stretch_bounds`': the control-difference bound above is a
    // POLYNOMIAL fact, and the standard rational extension (Floater
    // 1992) multiplies it by the weight ratio; squaring it is the
    // conservative reading. Exactly 1 on a unit-weight net, so no
    // integral-lane number moves.
    sup * weight_ratio_factor::<T>(c.weights())
}

/// `(w_max/w_min)²` for a positive weight list, `1` when the list is
/// unit, empty or non-positive (a non-positive weight fails its own
/// gate elsewhere; answering 1 here never widens a bound that the
/// caller then trusts).
fn weight_ratio_factor<T: Real>(weights: &[f64]) -> T {
    let (mut lo, mut hi) = (f64::INFINITY, 0.0f64);
    for w in weights {
        lo = lo.min(*w);
        hi = hi.max(*w);
    }
    if lo > 0.0 && hi.is_finite() {
        T::from_f64((hi / lo).powi(2))
    } else {
        T::one()
    }
}

/// Check 3 for either lane: `|S(P(tᵢ)) − C(tᵢ)|` at the shared
/// schedule, in metres through the map.
fn schedule_residuals<T: Decide>(
    pcurve: &Pcurve<T>,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
    max_residual: &mut T,
) -> Result<(), PcurveCertifyError> {
    for i in 0..CERT_SAMPLES {
        let t = sample_param(t0, t1, i);
        let chart_point = pcurve.eval(t);
        let mapped = surface.eval(chart_point.x, chart_point.y);
        let on_carrier = carrier.eval(t);
        check_residual(
            "pcurve_map_residual",
            PcurveCheck::MapResidual,
            i,
            Margin::of(mapped.distance(on_carrier)),
            band,
            max_residual,
        )?;
    }
    Ok(())
}

/// Check 5 for either lane: the pcurve's chart box inside the face's
/// window, metered through the map (no UV tolerance is ever compared
/// against ε — C4).
fn trim_containment<T: Decide>(
    pcurve: &Pcurve<T>,
    t0: T,
    t1: T,
    surface: &Surface<T>,
    window: ChartWindow<T>,
    band: Band,
) -> Result<(), PcurveCertifyError> {
    let boxed = pcurve.chart_box(t0, t1);
    let (u_arm, v_arm) = chart_arms_at(surface, &boxed, &window);
    let escapes = [
        Margin::metered(window.u_min - boxed.u_min, u_arm),
        Margin::metered(boxed.u_max - window.u_max, u_arm),
        Margin::metered(window.v_min - boxed.v_min, v_arm),
        Margin::metered(boxed.v_max - window.v_max, v_arm),
    ];
    for over in escapes {
        match decide("pcurve_trim_containment", over, band) {
            Ok(Sign::Negative | Sign::Zero) => {}
            Ok(Sign::Positive) => return Err(PcurveCertifyError::TrimEscape),
            Err(cause) => {
                return Err(PcurveCertifyError::Escalated {
                    check: PcurveCheck::TrimContainment,
                    sample: 0,
                    cause,
                });
            }
        }
    }
    Ok(())
}

/// **The fitted lane's five checks** (M6-2), in the same fixed order as
/// the closed-form lane's — what differs is check 1's admission rule
/// and check 4's mechanism.
///
/// 1. **Lane**: the carrier is a rung-3 (`Curve3::Nurbs`) one, which is
///    what a fitted chart image is the image OF. The chart kind is not
///    restricted here: whether a certificate exists for it is decided
///    by the SSI machinery in check 4, which refuses typed per kind
///    (cone/torus have no ring-computable meters composite) rather than
///    being pre-judged by a list that would drift out of date.
/// 2. **Interval**: the same forward-span check, plus the periodic
///    chart's azimuth gate — taken over the CONTROL-NET box (the hull
///    property) instead of a closed-form extent.
/// 3. **Schedule**: identical, and shared code.
/// 4. **Envelope**: the full C2 certificate from
///    `geom_brep::ssi::certify` — hull sup-norm AND uniqueness tube —
///    re-derived here at rest, never trusted from storage. The stored
///    envelope is that certificate's `hull_sup`, and
///    [`PcurveCertificate::statement`] records which sup it bounds.
/// 5. **Trim containment**: identical, and shared code.
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
fn run_fitted_checks<T: PcurveFittedLane>(
    image: &Arc<NurbsCurve2<T>>,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    mate: Option<&Surface<T>>,
    window: ChartWindow<T>,
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    // ---- Check 1: the lane. ----
    // Rung-3 NURBS carriers feed the SSI door directly; exact CIRCLE
    // carriers are the sphere chart's general-circle class (M6-3,
    // walk row 4) and enter through their locus-exact rational chain
    // inside the lane (trait docs). Lines/ellipses have no fitted
    // class anywhere — every line and every conic-on-its-own-chart is
    // a closed-form citizen or a named refusal.
    if !matches!(carrier, Curve3::Nurbs(_) | Curve3::Circle { .. }) {
        return Err(PcurveCertifyError::UnsupportedCarrier);
    }
    let Some(mate) = mate else {
        return Err(PcurveCertifyError::FittedMateMissing);
    };
    let pcurve = Pcurve::Fitted(Arc::clone(image));

    // ---- Check 2: the parameter interval, metered into metres. ----
    let span = t1 - t0;
    // The span crosses to model space through the carrier kind's own
    // metric rate, gated definitely-positive first so a collapsed or
    // poison meter can never fabricate a forward verdict.
    let span_escalated = |cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::ParamSpan,
        sample: 0,
        cause,
    };
    let rate = param_rate_gate(carrier, band).map_err(span_escalated)?;
    match decide("pcurve_interval_forward", Margin::metered(span, rate), band)
        .map_err(span_escalated)?
    {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(PcurveCertifyError::IntervalNotForward),
    }
    let boxed = pcurve.chart_box(t0, t1);
    if !matches!(
        surface,
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_)
    ) {
        // The azimuth headroom is an ANGLE, so it reaches the band
        // through the chart's own lever arm — the cone's taken at the
        // `v` reach that dominates both the pcurve's box and the
        // window, which is the local lever's supremum everywhere
        // either object lives (`chart_arms_at`'s safe direction).
        let (u_arm, _) = chart_arms_at(surface, &boxed, &window);
        let headroom = decide(
            "pcurve_azimuth_period",
            Margin::levered(T::tau() - (boxed.u_max - boxed.u_min), u_arm),
            band,
        );
        match headroom.map_err(|cause| PcurveCertifyError::Escalated {
            check: PcurveCheck::AzimuthPeriod,
            sample: 0,
            cause,
        })? {
            Sign::Positive | Sign::Zero => {}
            Sign::Negative => return Err(PcurveCertifyError::AzimuthPeriodExceeded),
        }
    }

    // ---- Check 3: the schedule, in metres through the map. ----
    let mut max_residual = T::zero();
    schedule_residuals(&pcurve, t0, t1, carrier, surface, band, &mut max_residual)?;

    // ---- Check 4: the full C2 certificate, RE-DERIVED. ----
    let Some(ssi) = T::fitted_certificate(carrier, t0, t1, image, surface, mate, band)? else {
        return Err(PcurveCertifyError::FittedLaneUnsupported {
            scalar: T::lane_name(),
        });
    };
    let envelope = ssi.hull_sup;
    // The catch-all is SPLIT: an approximating surface's limbs are the
    // spline composite's, exactly as a `Nurbs` chart's, because the
    // limbs run against its fit.
    let statement = match surface {
        Surface::Nurbs(_) | Surface::Approx(_) => EnvelopeStatement::MapResidualComposite,
        Surface::Plane { .. }
        | Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Torus { .. } => EnvelopeStatement::OnLocusHull,
    };
    // The envelope is banded exactly as the closed-form lane's is, and
    // for the same reason: a certificate whose own bound exceeds ε is
    // not a certificate. It is NOT folded into `max_residual` (the
    // sampled max and the sup bound stay separate statements).
    let mut envelope_margin = T::zero();
    check_residual(
        "pcurve_envelope",
        PcurveCheck::Envelope,
        0,
        Margin::of(envelope),
        band,
        &mut envelope_margin,
    )?;

    // ---- Check 5: trim containment (the chart-box limb). ----
    trim_containment(&pcurve, t0, t1, surface, window, band)?;

    Ok(PcurveCertificate {
        samples: CERT_SAMPLES,
        max_residual,
        envelope,
        statement,
        ssi: Some(ssi),
    })
}

/// **The ARC-RIM iso class** (M8-3) — certification of a
/// [`Pcurve::IsoArc`], to the same bar as every other minted pcurve.
///
/// The five checks in the same fixed order as [`run_iso_checks`]; only
/// check 4 differs, and it is the whole content of the class.
///
/// # The envelope chain
///
/// With `P(t) = (g(t), v_side)` and `B` the chart's own boundary
/// COLUMN at the snapped side,
///
/// ```text
/// sup |S(P(t)) − C(t)|
///   ≤ |S(g, v) − B(g)|          (the v-snap slack, exactly `side_of`'s)
///   + |B(g)    − Ĉ(g)|          (the RATIONAL control-difference hull)
///   + |Ĉ(g(t)) − C(t)|          (= 0: the variant's own algebraic identity)
/// ```
///
/// `Ĉ` is the carrier circle re-expressed in **B's own spline space**
/// — B's knots and B's weights, control points from the closed form
/// (`on(θ)` and the tangent intersection `on(θ)/cos(h/2)`). Sharing
/// the weights is what makes the middle term a convex combination:
/// `B − Ĉ = Σ Rᵢ(u)·(bᵢ − ĉᵢ)` with `Rᵢ ≥ 0` summing to 1 (the
/// rational hull property, positive weights), hence
/// `sup|B − Ĉ| ≤ maxᵢ|bᵢ − ĉᵢ|`. Sampling could not do this job: a
/// second-order between-samples bound at nine samples is `O(10⁻²) m`,
/// six decades past useful.
///
/// The structure `Ĉ` assumes — degree 2, uniform clamped knots with
/// one span per sub-arc on `[0, 1]`, weights `1, w, 1, w, …, 1` — is
/// read as EXACT `f64` structure (C6) and refused typed when it does
/// not hold, so an imported rational chart that is not this
/// construction cannot slip through. That `w` really is `cos(h/2)` is
/// the one DECIDED margin, metered into metres by the radius.
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
#[allow(clippy::too_many_lines)] // one class, kept whole like its siblings
fn run_iso_arc_checks<T: Decide>(
    p0: Point2<T>,
    pd: Vec2<T>,
    at0: T,
    angle: T,
    breaks: &KnotVector,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    window: ChartWindow<T>,
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    // ---- Check 1: the certified lane. ----
    // The iso lane is the SPLINE chart's — an approximating surface's
    // fit is one, on the same `(u, v)`, so it enters here rather than
    // being refused as an unimplemented chart.
    let Some(payload) = surface.spline_chart() else {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: chart_name(surface),
        });
    };
    if payload.is_placeholder() {
        return Err(PcurveCertifyError::IsoUnsupported {
            what: "the chart is the mvfs placeholder (no description yet) — a mid-surgery \
                   fact, not a certifiable chart",
        });
    }
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = carrier
    else {
        return Err(PcurveCertifyError::IsoUnsupported {
            what: "an arc-rim iso over a non-Circle carrier — the class IS the circle's \
                   rational-quadratic reparameterization",
        });
    };

    // ---- Check 2: the parameter interval, metered into metres. ----
    //
    // This class's carrier is a `Curve3::Circle` by construction
    // (refused just above otherwise), so `param_rate` is the RADIUS
    // and `span·rate` is arc length — a genuine metre, and a rate that
    // cannot be poison, which is why no meter gate stands here.
    let span = t1 - t0;
    match decide(
        "pcurve_interval_forward",
        Margin::metered(span, param_rate(carrier)),
        band,
    )
    .map_err(|cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::ParamSpan,
        sample: 0,
        cause,
    })? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(PcurveCertifyError::IntervalNotForward),
    }

    // ---- Check 3: the schedule, in metres through the map. ----
    let pcurve = Pcurve::IsoArc {
        p0,
        pd,
        t0: at0,
        angle,
        breaks: breaks.clone(),
    };
    let mut max_residual = T::zero();
    schedule_residuals(&pcurve, t0, t1, carrier, surface, band, &mut max_residual)?;

    // ---- Check 4: the rational control-difference hull. ----
    let esc = |cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::Envelope,
        sample: 0,
        cause,
    };
    let bad = |what: &'static str| PcurveCertifyError::IsoUnsupported { what };
    let (stretch_u, stretch_v) = nurbs_stretch_bounds(payload);
    // The chart's OWN domain (`side_of`'s note, #327).
    let (cu0f, cu1f) = payload.knots_u().domain();
    let (cv0f, cv1f) = payload.knots_v().domain();
    let (cu0, cu1) = (T::from_f64(cu0f), T::from_f64(cu1f));
    let (cv0, cv1) = (T::from_f64(cv0f), T::from_f64(cv1f));
    // The moving channel is u and the fixed one v (the cap class'
    // geometry).
    let (end, slack_v) = side_of(
        p0.y,
        cv0,
        cv1,
        stretch_v,
        pd.y.abs() * stretch_v,
        band,
        &esc,
    )?;
    let b = crate::nurbs_iso::boundary_iso_v(payload, end)
        .map_err(|_| bad("the chart's boundary column failed to re-wrap as a curve"))?;
    // --- The construction's EXACT structure (C6). ---
    let spans = breaks.control_count().saturating_sub(1);
    if spans == 0 || b.knots().degree() != 2 {
        return Err(bad(
            "an arc rim whose chart column is not a quadratic — the rational-quadratic \
             arc construction is the only one this class certifies",
        ));
    }
    let kn = b.knots().knots();
    #[allow(clippy::cast_precision_loss)]
    let expected: Vec<f64> = {
        let m = spans as f64;
        let mut v = vec![cu0f, cu0f, cu0f];
        for k in 1..spans {
            let t = cu0f + (cu1f - cu0f) * (k as f64) / m;
            v.push(t);
            v.push(t);
        }
        v.extend([cu1f, cu1f, cu1f]);
        v
    };
    if kn.len() != expected.len() {
        return Err(bad(
            "an arc rim whose chart column knots are not the uniform sub-arc structure \
             (degree 2, one double knot per sub-arc over the chart's own u domain)",
        ));
    }
    // **The knot values, METERED rather than compared bitwise (#327).**
    // The kernel's own arc walls carry knots it computed, so `k/m`
    // holds to the bit and this margin is identically zero. An
    // IMPORTED wall carries the file's printed knots — dm1's are
    // `√3, 2√3, 3√3` each rounded on its own, so the interior breaks
    // miss exact thirds of the domain by ~2·10⁻¹⁴ — and demanding
    // bitwise uniformity there refuses the construction for its
    // PRINTING rather than for its geometry. What the uniform breaks
    // are load-bearing for is the map `g ↦ u`: within a span both the
    // assumed and the true parameterization are affine, so a knot off
    // by `Δ` moves the chart point by at most `Δ` in `u`, hence by at
    // most `Δ·stretch_u` in metres. That quantity is DECIDED here and
    // then PAID into the envelope below — never assumed away.
    //
    // **A posture change, called out.** This check used to be an exact
    // `f64` comparison, whose only outcomes
    // were "structure holds" and a definite typed refusal. Routing it
    // through `decide` adds a third — a knot deviation inside the
    // ambiguity band ESCALATES rather than refusing definitely. That
    // is the correct posture for a quantity metered into metres, and
    // the one every other margin in this class already has, but it is
    // a new outcome on this path.
    let mut knot_dev = T::zero();
    for (a, b) in kn.iter().zip(&expected) {
        knot_dev = knot_dev.max(T::from_f64(a - b).abs());
    }
    match decide(
        "pcurve_iso_boundary",
        Margin::metered(knot_dev, stretch_u),
        band,
    )
    .map_err(&esc)?
    {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            return Err(bad(
                "an arc rim whose chart column knots are not the uniform sub-arc \
                 structure (degree 2, one double knot per sub-arc over the chart's own \
                 u domain)",
            ));
        }
    }
    let slack_knots = knot_dev * stretch_u;
    let bw = b.weights();
    if bw.len() != 2 * spans + 1 || bw.first() != Some(&1.0) || bw.last() != Some(&1.0) {
        return Err(bad(
            "an arc rim whose chart column weights are not the arc pattern",
        ));
    }
    let half_w = bw.get(1).copied().unwrap_or(f64::NAN);
    for (i, w) in bw.iter().enumerate() {
        let want = if i % 2 == 0 { 1.0 } else { half_w };
        if *w != want {
            return Err(bad(
                "an arc rim whose chart column weights are not `1, w, 1, …` with ONE \
                 interior weight — the uniform sub-arc construction",
            ));
        }
    }
    // --- The one decided margin: `w = cos(h/2)`. ---
    #[allow(clippy::cast_precision_loss)]
    let m_t = T::from_f64(spans as f64);
    let h = angle / m_t;
    let (_, cos_half) = (h * T::from_f64(0.5)).sin_cos();
    match decide(
        "pcurve_iso_boundary",
        Margin::metered(T::from_f64(half_w) - cos_half, *radius),
        band,
    )
    .map_err(&esc)?
    {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            return Err(bad(
                "an arc rim whose chart column weight is not `cos(h/2)` for its sub-arc \
                 angle — the column is not this circle's rational-quadratic form",
            ));
        }
    }
    // --- `Ĉ` in B's own space, and the hull. ---
    //
    // Angles are measured from the RIM'S OWN start, `at0` — the
    // carrier's angle at `g = 0` — not from the carrier frame's zero.
    // A rim that begins at a nonzero phase is the same construction
    // rotated, and the class certifies it as such; keying the control
    // points to absolute zero would have refused it with a message
    // blaming the weight pattern for what is really a phase.
    let vref = axis.cross(*u_ref);
    let on = |a: T| {
        let (s, c) = (at0 + a).sin_cos();
        *center + *u_ref * (*radius * c) + vref * (*radius * s)
    };
    let tangent = |a: T| {
        let (s, c) = (at0 + a).sin_cos();
        let r = *radius / cos_half;
        *center + *u_ref * (r * c) + vref * (r * s)
    };
    let mut chat: Vec<Point3<T>> = Vec::with_capacity(2 * spans + 1);
    chat.push(on(T::zero()));
    for k in 0..spans {
        #[allow(clippy::cast_precision_loss)]
        let base = h * T::from_f64(k as f64);
        chat.push(tangent(base + h * T::from_f64(0.5)));
        chat.push(on(base + h));
    }
    // **The u-DIRECTION (#327).** `Ĉ` is built in `B`'s control order,
    // which runs along the chart's INCREASING u. An imported rim whose
    // carrier winds against the chart traverses `u: cu1 → cu0` (the
    // mint's reversed `IsoArc`), and its `g = 0` sits at `B`'s LAST
    // control point — so the comparison list is the same one, read
    // backwards. Which case this is, is read off the placement, and
    // the `slack_affine` below is what PAYS for the reading: a
    // placement that is neither traversal pays its whole distance.
    // Which traversal this is, is the SAME two-way boundary question
    // `side_of` answers for the fixed channel — asked of the moving
    // one's start, and refused typed when the answer is neither.
    let (reversed, slack_start) = side_of(p0.x, cu0, cu1, stretch_u, T::zero(), band, &esc)?;
    let forward = !reversed;
    if !forward {
        chat.reverse();
    }
    if chat.len() != b.control().len() {
        return Err(bad(
            "an arc rim whose chart column control count is not the sub-arc construction's",
        ));
    }
    let mut hull = T::zero();
    for (pb, pc) in b.control().iter().zip(&chat) {
        hull = hull.max((*pb - *pc).norm());
    }
    // No domain-overshoot term: `g ∈ [0, 1]` by construction and the
    // column's domain is exactly `[0, 1]` (checked as structure just
    // above), so the iso-line class's `over·stretch` slack is
    // identically zero here rather than merely small.
    //
    // **The admitted-input slack** (R1 MINOR-1). The chain above
    // compares `B(g)` against `Ĉ(g)`, i.e. it reads the pcurve's u
    // channel AS `g`. Every mint satisfies that bitwise (`p0 = (0, v)`,
    // `pd = (1, 0)`), but `certify` is a public door and admits any
    // placement, so the difference has to be PAID rather than assumed:
    // `u(t) − g(t) = p0.x + (pd.x − 1)·g` is affine in `g ∈ [0, 1]`, so
    // its sup is at an endpoint — `max(|p0.x|, |p0.x + pd.x − 1|)` —
    // and `stretch_u` meters it into metres. Identically zero on the
    // minted path, exactly as the iso-line class's `slack_param` and
    // the harmonic class's winding snap are.
    let far = if forward {
        (p0.x + pd.x - cu1).abs()
    } else {
        (p0.x + pd.x - cu0).abs()
    };
    let slack_affine = slack_start.max(far * stretch_u);
    let envelope = hull + slack_v + slack_affine + slack_knots;
    let mut envelope_margin = T::zero();
    check_residual(
        "pcurve_envelope",
        PcurveCheck::Envelope,
        0,
        Margin::of(envelope),
        band,
        &mut envelope_margin,
    )?;

    // ---- Check 5: trim containment (the chart-box limb). ----
    trim_containment(&pcurve, t0, t1, surface, window, band)?;

    Ok(PcurveCertificate {
        samples: CERT_SAMPLES,
        max_residual,
        envelope,
        statement: EnvelopeStatement::MapResidualIsoHull,
        ssi: None,
    })
}

/// Which boundary a banded-constant chart channel sits on, plus the
/// slack the admission costs: `w` is the channel value at `t0`,
/// `(lo, hi)` the channel's own DOMAIN ends, `drift` its whole-span
/// motion bound, `arm` the stretch that meters both into metres.
/// `Zero` at `lo` → the start row, at `hi` → the end row; anything
/// else is an interior iso, refused typed. Shared by the iso-line and
/// iso-arc classes.
///
/// **The domain, not the unit square (#327).** A chart the kernel
/// BUILT is normalized to `[0, 1]²` and `(lo, hi) = (0, 1)` reads
/// exactly as before; a chart the kernel IMPORTED carries the file's
/// own parameterization (dm1's cylinder wall is `u ∈ [0, 3√3]`),
/// where testing against a literal `1` asks about an interior column.
fn side_of<T: Decide>(
    w: T,
    lo: T,
    hi: T,
    arm: T,
    drift: T,
    band: Band,
    esc: &impl Fn(Indeterminate) -> PcurveCertifyError,
) -> Result<(bool, T), PcurveCertifyError> {
    if let Sign::Zero =
        decide("pcurve_iso_boundary", Margin::metered(w - lo, arm), band).map_err(esc)?
    {
        return Ok((false, (w - lo).abs() * arm + drift));
    }
    if let Sign::Zero =
        decide("pcurve_iso_boundary", Margin::metered(w - hi, arm), band).map_err(esc)?
    {
        return Ok((true, (w - hi).abs() * arm + drift));
    }
    Err(PcurveCertifyError::IsoUnsupported {
        what: "an INTERIOR iso (the fixed channel sits on neither chart boundary): \
               boundary rows are control-net copies, an interior iso needs the de Boor \
               collapse extractor — which arrives with the construction that first \
               mints one",
    })
}

/// **The iso lane's five checks** (M6-3), same fixed order as the
/// closed-form lane's. What differs: check 1 admits any described-NURBS
/// chart — the blanket non-rational gate it used to carry came off in
/// M8-3, which moved the rationality hypothesis into the class arms of
/// check 4, where it is load-bearing per class (the seam class needs
/// strictly positive weights and one shared spline space; the rim class
/// still needs weights of exactly 1, for linear precision). Check 4's
/// sup bound is the boundary-row control-difference hull
/// ([`EnvelopeStatement::MapResidualIsoHull`]) with the banded
/// axis/side/domain snap slacks folded in — the cylinder lane's
/// winding-snap idiom transposed. Every slack is exactly zero on the
/// minted path (the builder mints exact `0`/`1` chart values).
#[allow(clippy::too_many_lines)] // one check sequence, kept whole like its two siblings
#[allow(clippy::too_many_arguments)] // one parameter per named quantity (the siblings' shape)
fn run_iso_checks<T: Decide>(
    p0: Point2<T>,
    pl: Vec2<T>,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    window: ChartWindow<T>,
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    // ---- Check 1: the certified lane. ----
    // The iso lane is the SPLINE chart's — an approximating surface's
    // fit is one, on the same `(u, v)`, so it enters here rather than
    // being refused as an unimplemented chart.
    let Some(payload) = surface.spline_chart() else {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: chart_name(surface),
        });
    };
    if payload.is_placeholder() {
        return Err(PcurveCertifyError::IsoUnsupported {
            what: "the chart is the mvfs placeholder (no description yet) — a mid-surgery \
                   fact, not a certifiable chart",
        });
    }

    // ---- Check 2: the parameter interval, metered into metres. ----
    let span = t1 - t0;
    // The metered span, gated on its meter first — the fitted lane's
    // shape exactly.
    let span_escalated = |cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::ParamSpan,
        sample: 0,
        cause,
    };
    let rate = param_rate_gate(carrier, band).map_err(span_escalated)?;
    match decide("pcurve_interval_forward", Margin::metered(span, rate), band)
        .map_err(span_escalated)?
    {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(PcurveCertifyError::IntervalNotForward),
    }

    // ---- Check 3: the schedule, in metres through the map. ----
    let pcurve = Pcurve::IsoLine { p0, pl };
    let mut max_residual = T::zero();
    schedule_residuals(&pcurve, t0, t1, carrier, surface, band, &mut max_residual)?;

    // ---- Check 4: the control-difference hull envelope, by class. ----
    let esc = |cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::Envelope,
        sample: 0,
        cause,
    };
    let (stretch_u, stretch_v) = nurbs_stretch_bounds(payload);
    let du_extent = Margin::metered(pl.x.abs() * span, stretch_u);
    let dv_extent = Margin::metered(pl.y.abs() * span, stretch_v);
    let u_moves = !matches!(
        decide("pcurve_iso_axis_u", du_extent, band).map_err(esc)?,
        Sign::Zero
    );
    let v_moves = !matches!(
        decide("pcurve_iso_axis_v", dv_extent, band).map_err(esc)?,
        Sign::Zero
    );
    let envelope = match (u_moves, v_moves) {
        // The SEAM class: u banded-constant on a boundary, v traverses
        // the carrier's own parameter. sup |S(P(t)) − C(t)| ≤
        //   |S(u(t), v(t)) − S(side, v(t))|   (u snap slack)
        // + |S(side, v(t)) − B(v(t))|         (exactly 0: B IS S(side, ·))
        // + |B(v(t)) − C(v(t))|               (control hull, same basis)
        // + |C(v(t)) − C(t)|                  (parameter-map slack).
        (false, true) => {
            let Curve3::Nurbs(c) = carrier else {
                return Err(PcurveCertifyError::IsoUnsupported {
                    what: "a seam-class iso line over a non-spline carrier — no \
                           construction mints one",
                });
            };
            // **The re-derivation** (M8-3) of the chart-level rational
            // gate this class used to carry. The control-difference
            // hull below needs `B − C = Σ Rᵢ·(bᵢ − cᵢ)` with the `Rᵢ`
            // non-negative and summing to 1. For a POLYNOMIAL pair
            // that is the B-spline partition of unity; for a RATIONAL
            // pair it is the rational basis `Rᵢ = NᵢwᵢΣ⁻¹`, which is
            // the same partition of unity **provided the two curves
            // share knots AND weights** — checked immediately below,
            // structurally and exactly (C6) — **and the weights are
            // strictly positive**, which is the convex-hull hypothesis
            // itself and is checked here. So the hull is valid for
            // rational seams too, and the blanket chart-level gate
            // this class used to carry was over-broad: what is
            // load-bearing is the shared spline space, not the
            // weights being 1.
            if c.weights().iter().any(|w| !w.is_finite() || *w <= 0.0) {
                return Err(PcurveCertifyError::IsoUnsupported {
                    what: "a seam carrier with a non-positive or non-finite weight — the \
                           rational convex-hull property is exactly the hypothesis that \
                           fails there",
                });
            }
            let u_start = p0.x + pl.x * t0;
            let (cu0, cu1) = payload.knots_u().domain();
            let (end, slack_u) = side_of(
                u_start,
                T::from_f64(cu0),
                T::from_f64(cu1),
                stretch_u,
                du_extent.value(),
                band,
                &esc,
            )?;
            let b = crate::nurbs_iso::boundary_iso_u(payload, end).map_err(|_| {
                PcurveCertifyError::IsoUnsupported {
                    what: "the chart's boundary row failed to re-wrap as a curve \
                           (corrupt chart structure)",
                }
            })?;
            if b.knots().knots() != c.knots().knots()
                || b.knots().degree() != c.knots().degree()
                || b.weights() != c.weights()
            {
                return Err(PcurveCertifyError::IsoUnsupported {
                    what: "the seam carrier is not the chart's own boundary row (its \
                           knot/weight structure differs) — the hull comparison needs \
                           one spline space",
                });
            }
            let mut hull = T::zero();
            for (pb, pc) in b.control().iter().zip(c.control()) {
                hull = hull.max((*pb - *pc).norm());
            }
            // Parameter map v(t) = p0.y + pl.y·t vs the identity: the
            // difference is affine, so its extremes are at the
            // endpoints; metered through the carrier's own rate bound.
            let v_at_0 = p0.y + pl.y * t0;
            let v_at_1 = p0.y + pl.y * t1;
            let slack_param = (v_at_0 - t0).abs().max((v_at_1 - t1).abs()) * curve_rate_bound(c);
            // Domain containment: the hull and rate bounds hold on the
            // carrier's knot domain only.
            let (d0, d1) = c.domain();
            let lo = t0.min(v_at_0).min(v_at_1);
            let hi = t1.max(v_at_0).max(v_at_1);
            let over = (T::from_f64(d0) - lo)
                .max(hi - T::from_f64(d1))
                .max(T::zero());
            match decide("pcurve_iso_domain", Margin::metered(over, stretch_v), band)
                .map_err(esc)?
            {
                Sign::Zero => {}
                Sign::Positive | Sign::Negative => {
                    return Err(PcurveCertifyError::IsoUnsupported {
                        what: "the iso line leaves the chart's parameter domain — the \
                               hull bound holds on the domain only",
                    });
                }
            }
            hull + slack_u + slack_param + over * stretch_v
        }
        // The CAP class: v banded-constant on a boundary, u affine in
        // the carrier parameter, carrier a straight line. The affine
        // composite Line(t(u)) reproduces exactly on the Greville
        // abscissae, so the same control hull applies.
        (true, false) => {
            let Curve3::Line { origin, dir } = carrier else {
                return Err(PcurveCertifyError::IsoUnsupported {
                    what: "a cap-class iso LINE over a non-Line carrier — an arc rim is \
                           minted as `Pcurve::IsoArc`, whose chart parameter is the \
                           segment's rational-quadratic one",
                });
            };
            let v_start = p0.y + pl.y * t0;
            let (cv0, cv1) = payload.knots_v().domain();
            let (end, slack_v) = side_of(
                v_start,
                T::from_f64(cv0),
                T::from_f64(cv1),
                stretch_v,
                dv_extent.value(),
                band,
                &esc,
            )?;
            let b = crate::nurbs_iso::boundary_iso_v(payload, end).map_err(|_| {
                PcurveCertifyError::IsoUnsupported {
                    what: "the chart's boundary column failed to re-wrap as a curve \
                           (corrupt chart structure)",
                }
            })?;
            // **This class keeps a rational gate, and it is the real
            // one** (M8-3). The hull below compares the column's
            // control points against the LINE sampled at the Greville
            // abscissae, which is sound because a B-spline basis
            // reproduces affine functions exactly there — LINEAR
            // PRECISION. The rational basis has no such property, so
            // on a rational column the Greville sample is not the
            // line's representation in the column's own space and the
            // control-difference hull would bound nothing. (The seam
            // class needs no gate: there both curves are given in ONE
            // shared space. The arc-rim class builds `Ĉ` in the
            // column's space explicitly, which is the same fix by
            // construction.)
            if b.weights().iter().any(|w| *w != 1.0) {
                return Err(PcurveCertifyError::IsoUnsupported {
                    what: "a LINE cap rim on a RATIONAL chart column: the Greville hull is \
                           a linear-precision fact and the rational basis has none — a line \
                           rim whose column is rational needs its line re-expressed in that \
                           column's own space, the arc-rim class's construction",
                });
            }
            let (p, kn) = (b.knots().degree(), b.knots().knots());
            let mut hull = T::zero();
            for (i, cp) in b.control().iter().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                let xi = kn[i + 1..=i + p].iter().sum::<f64>() / p as f64;
                let t_at = (T::from_f64(xi) - p0.x) / pl.x;
                let d = *origin + *dir * t_at;
                hull = hull.max((*cp - d).norm());
            }
            let u_at_0 = p0.x + pl.x * t0;
            let u_at_1 = p0.x + pl.x * t1;
            let (d0, d1) = b.knots().domain();
            let over = (T::from_f64(d0) - u_at_0.min(u_at_1))
                .max(u_at_0.max(u_at_1) - T::from_f64(d1))
                .max(T::zero());
            match decide("pcurve_iso_domain", Margin::metered(over, stretch_u), band)
                .map_err(esc)?
            {
                Sign::Zero => {}
                Sign::Positive | Sign::Negative => {
                    return Err(PcurveCertifyError::IsoUnsupported {
                        what: "the iso line leaves the chart's parameter domain — the \
                               hull bound holds on the domain only",
                    });
                }
            }
            hull + slack_v + over * stretch_u
        }
        (false, false) => {
            return Err(PcurveCertifyError::IsoUnsupported {
                what: "a DEGENERATE iso line (neither chart channel definitely moves \
                       over the span)",
            });
        }
        (true, true) => {
            return Err(PcurveCertifyError::IsoUnsupported {
                what: "a DIAGONAL line in UV (both channels move): only axis-aligned \
                       isos have the boundary-row closed form",
            });
        }
    };
    let mut envelope_margin = T::zero();
    check_residual(
        "pcurve_envelope",
        PcurveCheck::Envelope,
        0,
        Margin::of(envelope),
        band,
        &mut envelope_margin,
    )?;

    // ---- Check 5: trim containment (the chart-box limb). ----
    trim_containment(&pcurve, t0, t1, surface, window, band)?;

    Ok(PcurveCertificate {
        samples: CERT_SAMPLES,
        max_residual,
        envelope,
        statement: EnvelopeStatement::MapResidualIsoHull,
        ssi: None,
    })
}

/// Branch-stabilized azimuth (M5 S13, shared by every chart's
/// derivation since M6-3): atan2's cut sits on the negative-x axis,
/// and an interval y touching zero there (a seam meridian's angle-π
/// copy, or a rim whose start sits ON the seam) explodes the
/// enclosure to a full period even though every consumer reads
/// azimuth mod τ. On a definitely-negative-x frame the same angle is
/// `atan2(−y, −x) + π`. The frame trilean chooses between two
/// identical formulas; degenerate and in-band arms keep the direct
/// one (tie-break, D9).
fn stable_azimuth<T: Decide>(y: T, x: T, band: Band) -> T {
    match decide("pcurve_chart_azimuth_frame", Margin::of(x), band) {
        Ok(Sign::Negative) => (T::zero() - y).atan2(T::zero() - x) + T::pi(),
        Ok(Sign::Positive | Sign::Zero) | Err(_) => y.atan2(x),
    }
}

/// Derives the **exact closed-form chart image** of `carrier` on
/// `surface` — the constructor every minted cache goes through, and the
/// derive-on-demand answer for the faces that store nothing (planar
/// faces keep M2's status, C4 verbatim).
///
/// Total arithmetic with one named trilean (the periodic chart's
/// orientation): degenerate or incoherent inputs produce a pcurve whose
/// certification fails loudly rather than a guess (the established
/// posture — nothing here decides what certification can check).
///
/// The azimuth branch is the **principal** one; a face that needs
/// another branch shifts it by whole periods exactly once, through
/// [`Pcurve::shift_branch`] (the loop walk in `topo::pcurves`). No
/// per-sample unwrapping exists anywhere in this lane.
///
/// # Errors
///
/// [`PcurveCertifyError::UnsupportedChart`] / `UnsupportedCarrier` for
/// kinds outside the certified lane; `Escalated` when the orientation
/// trilean lands in the sliver band.
pub fn chart_pcurve<T: Decide>(
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<Pcurve<T>, PcurveCertifyError> {
    let Some(form) = carrier_harmonic(carrier) else {
        return Err(PcurveCertifyError::UnsupportedCarrier);
    };
    match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            // Affine chart: the coefficients map through one by one.
            let v_ref = normal.cross(u_ref);
            let chart = |v: Vec3<T>| Vec2::new(v.dot(u_ref), v.dot(v_ref));
            let w = form.c - origin;
            Ok(Pcurve::Harmonic {
                p0: Point2::new(w.dot(u_ref), w.dot(v_ref)),
                pa: chart(form.a),
                pb: chart(form.b),
                pl: chart(form.l),
            })
        }
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let cv = axis.cross(u_ref);
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let w = form.c - origin;
            // The axial channel is exact for every carrier kind (`v` is
            // a linear functional of the point); only the azimuth
            // channel needs a case.
            let a_r = radial(form.a);
            let b_r = radial(form.b);
            let l_r = radial(form.l);
            // The azimuth channel. A carrier whose radial part is
            // constant (`a_r = b_r = l_r = 0`) is a meridian: β = 0,
            // α from the constant radial part. Otherwise the radial
            // part must be the chart circle traversed once — the
            // section/rim case — and β is its orientation against the
            // chart frame.
            let w_r = radial(w);
            // The amplitude of the radial oscillation IS metres (the
            // harmonic coefficients are displacement vectors) — it is
            // compared against the band directly. Weighting it by the
            // chart radius again (the pre-M6-3 form) squared the
            // scale and misread a genuinely-moving rim on a
            // near-band-radius cylinder as a meridian, which then
            // failed the residual schedule loudly (the 100ε washer).
            let moving = a_r.norm() + b_r.norm() + l_r.norm();
            let alpha_const = stable_azimuth(w_r.dot(cv), w_r.dot(u_ref), band);
            match decide("pcurve_chart_radial_moving", Margin::of(moving), band) {
                // Zero — AND the in-band arm (Err): a sub-escalation
                // radial amplitude takes the meridian form as a D9
                // tie-break (the `stable_azimuth` posture): this is
                // STRUCTURE selection, not a topological decision —
                // check 4's envelope is built from the true difference
                // of forms, so the discarded drift lands in the
                // certified sup bound and an amplitude that matters
                // refuses there in metres. (The executed case: a wild
                // NIST import whose near-meridian line carries a
                // few-nanometre radial tilt.)
                Ok(Sign::Zero) | Err(_) => Ok(Pcurve::Harmonic {
                    p0: Point2::new(alpha_const, w.dot(axis)),
                    pa: Vec2::new(T::zero(), form.a.dot(axis)),
                    pb: Vec2::new(T::zero(), form.b.dot(axis)),
                    pl: Vec2::new(T::zero(), form.l.dot(axis)),
                }),
                Ok(Sign::Positive | Sign::Negative) => {
                    // The moving case: radial(t) = a_r·cos t + b_r·sin t
                    // (a linear radial part would not close a chart
                    // circle; certification refuses it through the
                    // residual). α is the azimuth of `a_r`; β is the
                    // orientation of (a_r, b_r) against the chart's own
                    // frame, a named trilean metered at the radius.
                    let alpha = stable_azimuth(a_r.dot(cv), a_r.dot(u_ref), band);
                    let orient = a_r.cross(b_r).dot(axis);
                    let beta = match decide(
                        "pcurve_chart_orientation",
                        Margin::over_lever(orient, radius),
                        band,
                    ) {
                        Ok(Sign::Positive) => T::one(),
                        Ok(Sign::Negative) => T::zero() - T::one(),
                        Ok(Sign::Zero) => T::zero(),
                        Err(cause) => {
                            return Err(PcurveCertifyError::Escalated {
                                check: PcurveCheck::ChartWinding,
                                sample: 0,
                                cause,
                            });
                        }
                    };
                    Ok(Pcurve::Harmonic {
                        p0: Point2::new(alpha, w.dot(axis)),
                        pa: Vec2::new(T::zero(), form.a.dot(axis)),
                        pb: Vec2::new(T::zero(), form.b.dot(axis)),
                        pl: Vec2::new(beta, form.l.dot(axis)),
                    })
                }
            }
        }
        // The cone chart (M6-3, walk row 4): closed forms for the two
        // classes a cone carries at rest — RIM circles (⊥ axis,
        // centred on it: azimuth `α + β·t`, slant constant) and RULING
        // lines (azimuth constant, slant affine — `v` is a length, so
        // its slope is unconstrained). A general conic on a cone chart
        // (a tilted plane×cone section ellipse) is azimuth-NON-harmonic
        // and refuses typed. Derivations are structure selection; the
        // full residual certification follows every derivation and is
        // what makes a wrong pick fail loudly.
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let c_ha = half_angle.cos();
            let cv = axis.cross(u_ref);
            let esc = |cause| PcurveCertifyError::Escalated {
                check: PcurveCheck::ChartWinding,
                sample: 0,
                cause,
            };
            match *carrier {
                Curve3::Line { origin, dir } => {
                    // The RULING class. The `v` channel is exact and
                    // NAPPE-FREE from the axial data alone: chart
                    // height is v·cos α, so
                    // `v(t) = ((L(t) − apex)·axis) / cos α` is affine
                    // with no branch to pick (cos α > 0, the kernel's
                    // half-angle range). The azimuth branch IS the
                    // nappe: a point at v < 0 sits at spatial azimuth
                    // u + π (the mirror nappe — the Seam docs' case),
                    // so `u = azimuth(radial · sign v)`, the sign read
                    // from whichever height datum is definite (the
                    // anchor's, else the slope's; a line with neither
                    // is height-constant — no ruling of any cone).
                    let w = origin - apex;
                    let radial = |v: Vec3<T>| v - axis * v.dot(axis);
                    let (h0, hs) = (w.dot(axis), dir.dot(axis));
                    let (r_ref, h_sign) =
                        match decide("pcurve_cone_chart_nappe", Margin::of(h0), band)
                            .map_err(esc)?
                        {
                            Sign::Positive => (radial(w), T::one()),
                            Sign::Negative => (radial(w), T::zero() - T::one()),
                            Sign::Zero => {
                                match geom_core::k_stats::decide_flagged(
                                    "pcurve_cone_chart_nappe",
                                    hs,
                                    band,
                                    "F13",
                                )
                                .map_err(esc)?
                                {
                                    Sign::Positive => (radial(dir), T::one()),
                                    Sign::Negative => (radial(dir), T::zero() - T::one()),
                                    Sign::Zero => {
                                        return Err(PcurveCertifyError::UnsupportedCarrier);
                                    }
                                }
                            }
                        };
                    let r_dir = r_ref * h_sign;
                    let alpha = stable_azimuth(r_dir.dot(cv), r_dir.dot(u_ref), band);
                    Ok(Pcurve::Harmonic {
                        p0: Point2::new(alpha, h0 / c_ha),
                        pa: Vec2::new(T::zero(), T::zero()),
                        pb: Vec2::new(T::zero(), T::zero()),
                        pl: Vec2::new(T::zero(), hs / c_ha),
                    })
                }
                Curve3::Circle { center, .. } => {
                    let Some(form) = carrier_harmonic(carrier) else {
                        return Err(PcurveCertifyError::UnsupportedCarrier);
                    };
                    // Rim class: carrier plane ⊥ axis (a, b axial parts
                    // zero — already metres) and centred on the axis.
                    let (aa, ba) = (form.a.dot(axis), form.b.dot(axis));
                    match decide(
                        "pcurve_cone_chart_axial",
                        Margin::of(aa.abs() + ba.abs()),
                        band,
                    )
                    .map_err(esc)?
                    {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(PcurveCertifyError::UnsupportedCarrier);
                        }
                    }
                    let radial = |v: Vec3<T>| v - axis * v.dot(axis);
                    let w_r = radial(center - apex);
                    match decide("pcurve_cone_chart_centered", Margin::norm3(w_r), band)
                        .map_err(esc)?
                    {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(PcurveCertifyError::UnsupportedCarrier);
                        }
                    }
                    let a_r = radial(form.a);
                    // Slant from the axial height: v·cos α = h. A
                    // mirror-nappe rim has v < 0 (Seam docs), and its
                    // chart azimuth is the SPATIAL azimuth + π
                    // (radial(u)·v flips sign) — the same nappe rule
                    // as the ruling arm, decided on the height.
                    let h = (center - apex).dot(axis);
                    let v0 = h / c_ha;
                    let n_sign = match decide("pcurve_cone_chart_nappe", Margin::of(h), band)
                        .map_err(esc)?
                    {
                        Sign::Positive => T::one(),
                        Sign::Negative => T::zero() - T::one(),
                        // An apex-level "rim" is the apex point itself;
                        // no circle lies there.
                        Sign::Zero => return Err(PcurveCertifyError::UnsupportedCarrier),
                    };
                    let a_dir = a_r * n_sign;
                    let alpha = stable_azimuth(a_dir.dot(cv), a_dir.dot(u_ref), band);
                    let orient = a_r.cross(radial(form.b)).dot(axis);
                    // β metered at the rim's own radius — the honest
                    // local lever. The spatial traversal rate equals
                    // the chart azimuth rate on either nappe (the +π
                    // offset is constant), so β needs no nappe sign.
                    let rho = a_r.norm();
                    let beta = match decide(
                        "pcurve_chart_orientation",
                        Margin::over_lever(orient, rho),
                        band,
                    )
                    .map_err(esc)?
                    {
                        Sign::Positive => T::one(),
                        Sign::Negative => T::zero() - T::one(),
                        Sign::Zero => T::zero(),
                    };
                    Ok(Pcurve::Harmonic {
                        p0: Point2::new(alpha, v0),
                        pa: Vec2::new(T::zero(), T::zero()),
                        pb: Vec2::new(T::zero(), T::zero()),
                        pl: Vec2::new(beta, T::zero()),
                    })
                }
                Curve3::Ellipse { .. } | Curve3::Nurbs(_) => {
                    // The tilted-section class: azimuth-non-harmonic
                    // on a cone chart (the section's angle is not the
                    // chart azimuth), and no ring-computable meters
                    // composite exists for the cone (ssi/certify docs)
                    // — neither route is honest, so the class refuses.
                    Err(PcurveCertifyError::UnsupportedCarrier)
                }
            }
        }
        // The sphere chart (M5 S13, certified since M6-3): closed
        // forms for the two azimuth-affine circle classes — POLAR
        // circles (carrier plane ⊥ the polar axis: azimuth `α + β·t`,
        // polar constant) and MERIDIAN-class great circles (carrier
        // plane contains the polar axis: azimuth constant, polar
        // `δ + σ·t`). The GENERAL circle (neither class) is
        // azimuth-non-harmonic: it refuses HERE, and its chart image
        // lives in the fitted lane (`certify_fitted`'s Circle-carrier
        // arm, `EnvelopeStatement::OnLocusHull`) — walk row 4's
        // remaining route.
        //
        // Since M6-3 this arm is CERTIFIED (run_harmonic_checks admits
        // the sphere chart) and sphere faces mint stored caches. The
        // meridian arm's DERIVED anchor is the principal branch
        // (δ = atan2(aa, ‖a_r‖) ∈ [−π/2, π/2]), but the traversed arc
        // is NOT confined to it: a POLE-CROSSING meridian arc
        // CERTIFIES — `S(u, v)` extends smoothly past |v| = π/2 (the
        // chart formula covers the far meridian at the same u), so
        // the harmonic image is exact over the whole span (executed:
        // `review_m6_3_chart_probes::probe_pole_crossing_meridian_arc_
        // certifies`, envelope < 1e-12). What the far side changes is
        // which REPRESENTATION a loop walk needs: past the pole the
        // same points also carry the involution twin `(u+π, π−v)`,
        // and the walk selects between the two by certified
        // continuity (`topo::pcurves::sphere_twin`) — never snapped.
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            // Structural carrier gate: only circles lie on a sphere.
            if !matches!(carrier, Curve3::Circle { .. }) {
                return Err(PcurveCertifyError::UnsupportedCarrier);
            }
            let cv = axis.cross(u_ref);
            let w = form.c - center;
            let (aa, ba, wa) = (form.a.dot(axis), form.b.dot(axis), w.dot(axis));
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let (a_r, b_r, w_r) = (radial(form.a), radial(form.b), radial(w));
            let esc = |cause| PcurveCertifyError::Escalated {
                check: PcurveCheck::ChartWinding,
                sample: 0,
                cause,
            };
            // Branch-stabilized azimuth (M5 S13): atan2's cut sits on
            // the negative-x axis, and an interval y touching zero
            // there (a seam meridian's angle-π copy) explodes the
            // enclosure to a full period even though every consumer
            // reads azimuth mod τ. On a definitely-negative-x frame
            // the same angle is atan2(−y, −x) + π. The frame trilean
            // chooses between two identical formulas; degenerate and
            // in-band arms keep the direct one (tie-break, D9).
            let stable_az = |y: T, x: T| -> T { stable_azimuth(y, x, band) };
            // Which class: does the carrier plane contain the polar
            // axis' direction? (Metered in meters at the chart radius.)
            match decide(
                "pcurve_sphere_chart_axial",
                Margin::of(aa.abs() + ba.abs()),
                band,
            )
            .map_err(esc)?
            {
                Sign::Zero => {
                    // POLAR-circle class: a,b ⊥ axis. On the sphere the
                    // center then sits on the axis (its radial part is
                    // zero) — checked, not assumed.
                    match decide("pcurve_sphere_chart_centered", Margin::norm3(w_r), band)
                        .map_err(esc)?
                    {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(PcurveCertifyError::UnsupportedCarrier);
                        }
                    }
                    let alpha = stable_az(a_r.dot(cv), a_r.dot(u_ref));
                    let orient = a_r.cross(b_r).dot(axis);
                    let beta = match decide(
                        "pcurve_chart_orientation",
                        Margin::over_lever(orient, radius),
                        band,
                    )
                    .map_err(esc)?
                    {
                        Sign::Positive => T::one(),
                        Sign::Negative => T::zero() - T::one(),
                        Sign::Zero => T::zero(),
                    };
                    let polar = (wa / radius).asin();
                    Ok(Pcurve::Harmonic {
                        p0: Point2::new(alpha, polar),
                        pa: Vec2::new(T::zero(), T::zero()),
                        pb: Vec2::new(T::zero(), T::zero()),
                        pl: Vec2::new(beta, T::zero()),
                    })
                }
                Sign::Positive | Sign::Negative => {
                    // MERIDIAN class: the carrier plane must contain the
                    // axis (its own axis ⊥ polar) and be centered.
                    let coax = Margin::over_lever(form.a.cross(form.b).dot(axis), radius);
                    match decide("pcurve_sphere_chart_meridian", coax, band).map_err(esc)? {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(PcurveCertifyError::UnsupportedCarrier);
                        }
                    }
                    match decide("pcurve_sphere_chart_centered", Margin::norm3(w), band)
                        .map_err(esc)?
                    {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(PcurveCertifyError::UnsupportedCarrier);
                        }
                    }
                    // v(t) = σ·t + δ with sin δ = aa/r, cos δ = ‖a_r‖/r
                    // (principal branch), and the constant azimuth
                    // direction d̂ read off whichever radial part is
                    // structurally nonzero.
                    let delta = aa.atan2(a_r.norm());
                    let use_a =
                        match decide("pcurve_sphere_chart_pole_frame", Margin::norm3(a_r), band)
                            .map_err(esc)?
                        {
                            Sign::Positive | Sign::Negative => true,
                            Sign::Zero => false,
                        };
                    let d_hat = if use_a {
                        a_r / a_r.norm()
                    } else {
                        b_r / b_r.norm()
                    };
                    // σ: the polar rate at t = 0 is v′(0) = σ, and
                    // e′(0)·axis = ba, with e·axis = r·sin v ⇒
                    // ba = r·cos δ·σ. At a pole start (cos δ = 0) the
                    // radial consistency b_r = −σ·sin δ·r·d̂ decides
                    // instead. Both margins metered in meters.
                    let sigma_margin = if use_a {
                        ba
                    } else {
                        T::zero() - b_r.dot(d_hat) * aa / radius
                    };
                    let sigma = match decide(
                        "pcurve_sphere_chart_polar_rate",
                        Margin::of(sigma_margin),
                        band,
                    )
                    .map_err(esc)?
                    {
                        Sign::Positive => T::one(),
                        Sign::Negative => T::zero() - T::one(),
                        Sign::Zero => return Err(PcurveCertifyError::UnsupportedCarrier),
                    };
                    let alpha = stable_az(d_hat.dot(cv), d_hat.dot(u_ref));
                    Ok(Pcurve::Harmonic {
                        p0: Point2::new(alpha, delta),
                        pa: Vec2::new(T::zero(), T::zero()),
                        pb: Vec2::new(T::zero(), T::zero()),
                        pl: Vec2::new(T::zero(), sigma),
                    })
                }
            }
        }
        // The torus chart (M6-3, walk row 4): closed forms for the two
        // circle families the kernel mints — PARALLELS (⊥ axis,
        // centred on it: azimuth `α + β·t`, meridional constant) and
        // MERIDIANS (plane containing the axis, centre on the spine:
        // azimuth constant, meridional `δ + σ·t`). The Villarceau
        // class (and any other oblique circle) is azimuth-NON-harmonic
        // AND the cone/torus have no ring-computable meters composite
        // for a fitted certificate (ssi/certify docs) — neither route
        // is honest, so that class refuses typed.
        Surface::Torus {
            center: t_center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let cv = axis.cross(u_ref);
            let esc = |cause| PcurveCertifyError::Escalated {
                check: PcurveCheck::ChartWinding,
                sample: 0,
                cause,
            };
            if !matches!(carrier, Curve3::Circle { .. }) {
                return Err(PcurveCertifyError::UnsupportedCarrier);
            }
            let w = form.c - t_center;
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let (aa, ba) = (form.a.dot(axis), form.b.dot(axis));
            let (a_r, b_r, w_r) = (radial(form.a), radial(form.b), radial(w));
            // Which family: carrier plane ⊥ the axis? (Metres — the
            // axial parts of a and b are displacements.)
            match decide(
                "pcurve_torus_chart_axial",
                Margin::of(aa.abs() + ba.abs()),
                band,
            )
            .map_err(esc)?
            {
                Sign::Zero => {
                    // PARALLEL: centred on the axis, checked.
                    match decide("pcurve_torus_chart_centered", Margin::norm3(w_r), band)
                        .map_err(esc)?
                    {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(PcurveCertifyError::UnsupportedCarrier);
                        }
                    }
                    let alpha = stable_azimuth(a_r.dot(cv), a_r.dot(u_ref), band);
                    let rho = a_r.norm();
                    let orient = a_r.cross(b_r).dot(axis);
                    let beta = match decide(
                        "pcurve_chart_orientation",
                        Margin::over_lever(orient, rho),
                        band,
                    )
                    .map_err(esc)?
                    {
                        Sign::Positive => T::one(),
                        Sign::Negative => T::zero() - T::one(),
                        Sign::Zero => T::zero(),
                    };
                    // v₀ from the height/radius pair: R + r·cos v =
                    // ρ, r·sin v = h — atan2 of the two residual-
                    // certified coordinates (inner equator lands at
                    // v = π exactly as the chart states it).
                    let v0 = w.dot(axis).atan2(rho - major_radius);
                    Ok(Pcurve::Harmonic {
                        p0: Point2::new(alpha, v0),
                        pa: Vec2::new(T::zero(), T::zero()),
                        pb: Vec2::new(T::zero(), T::zero()),
                        pl: Vec2::new(beta, T::zero()),
                    })
                }
                Sign::Positive | Sign::Negative => {
                    // MERIDIAN: the carrier plane must contain the
                    // axis direction and the centre must sit on the
                    // spine (radius R from the axis) — the second is
                    // certified by the residual schedule; the first is
                    // the class gate.
                    let coax = Margin::over_lever(form.a.cross(form.b).dot(axis), minor_radius);
                    match decide("pcurve_torus_chart_meridian", coax, band).map_err(esc)? {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(PcurveCertifyError::UnsupportedCarrier);
                        }
                    }
                    let alpha = stable_azimuth(w_r.dot(cv), w_r.dot(u_ref), band);
                    let (sa, ca) = alpha.sin_cos();
                    let rad = u_ref * ca + cv * sa;
                    // δ from the t = 0 point's meridional components;
                    // σ from the t-derivative's (b's) — the relations
                    // a·axis = r·sin δ·?… spelled from
                    // e(t) = spine + a·cos t + b·sin t:
                    //   r·cos v = (e − spine)·rad, r·sin v = (e − spine)·axis
                    // with v = δ + σ·t ⇒ δ = atan2(a·axis, a·rad) and
                    // σ·r = b·axis·cos δ − b·rad·sin δ.
                    let delta = aa.atan2(a_r.dot(rad));
                    let (sd, cd) = delta.sin_cos();
                    let sigma_margin = ba * cd - b_r.dot(rad) * sd;
                    let sigma = match decide(
                        "pcurve_torus_chart_meridional_rate",
                        Margin::of(sigma_margin),
                        band,
                    )
                    .map_err(esc)?
                    {
                        Sign::Positive => T::one(),
                        Sign::Negative => T::zero() - T::one(),
                        Sign::Zero => return Err(PcurveCertifyError::UnsupportedCarrier),
                    };
                    Ok(Pcurve::Harmonic {
                        p0: Point2::new(alpha, delta),
                        pa: Vec2::new(T::zero(), T::zero()),
                        pb: Vec2::new(T::zero(), T::zero()),
                        pl: Vec2::new(T::zero(), sigma),
                    })
                }
            }
        }
        Surface::Nurbs(_) => Err(PcurveCertifyError::UnsupportedChart {
            chart: "Nurbs (representable-unimplemented)",
        }),
        // The closed-form pcurve mint is the analytic charts'. An
        // approximating surface's chart is a spline's, so it has no
        // harmonic image to mint — the fitted lane owns it.
        Surface::Approx(_) => Err(PcurveCertifyError::UnsupportedChart {
            chart: "Approx (fitted chart — no closed-form image)",
        }),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::{FRAC_PI_2, PI, TAU};
    use geom_core::Tol;

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// A unit-frame cylinder of radius `r` about `+z`, seam at `+x`.
    fn cylinder(r: f64) -> Surface<f64> {
        Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }
    }

    /// The tilted plane×cylinder section ellipse of `cylinder(r)` cut
    /// at height `h` by a plane tilted `tilt` off the axis (the corpus
    /// shape (i) configuration, built by hand here).
    fn tilted_section(r: f64, h: f64, tilt: f64) -> Curve3<f64> {
        // The section plane's normal is (sin tilt, 0, cos tilt); the
        // ellipse's minor axis is +y (in the cross-section plane), its
        // major axis is the tilt direction (cos tilt, 0, -sin tilt)
        // ... with semi-axis r/cos(tilt).
        Curve3::Ellipse {
            center: Point3::new(0.0, 0.0, h),
            // The ellipse's own axis is the section plane's normal.
            axis: Vec3::new(tilt.sin(), 0.0, tilt.cos()),
            major: r / tilt.cos(),
            minor: r,
            u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
        }
    }

    /// The chart image of a tilted section on its cylinder chart is the
    /// exact sinusoid graph `(t, h + r·tan(tilt)·cos t)` — derived, not
    /// fitted, and certified in metres through the map.
    #[test]
    fn tilted_section_on_cylinder_is_the_exact_sinusoid_graph() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let p = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let Pcurve::Harmonic { p0, pa, pb, pl } = p else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15, "azimuth anchored at the major axis");
        assert!((p0.y - h).abs() < 1e-15);
        assert!((pl.x - 1.0).abs() < 1e-15, "one azimuth turn per period");
        assert!(pl.y.abs() < 1e-15);
        // v(t) = h − (major·sin tilt)·cos t = h − r·tan(tilt)·cos t
        // (the ellipse's major direction tilts DOWN the axis by
        // construction, so the graph's amplitude is signed).
        assert!((pa.y + r * tilt.tan()).abs() < 1e-15);
        assert!(pb.y.abs() < 1e-15);
        assert!(pa.x.abs() < 1e-15 && pb.x.abs() < 1e-15);
        // And it certifies over the half-arc the corpus cuts.
        let cache =
            PcurveCache::certify(p, 0.0, PI, &carrier, &cyl, wide_window(), band()).unwrap();
        assert!(cache.certificate().max_residual < 1e-14);
        assert!(cache.certificate().envelope < 1e-14);
    }

    fn wide_window() -> ChartWindow<f64> {
        ChartWindow {
            u_min: -100.0,
            u_max: 100.0,
            v_min: -100.0,
            v_max: 100.0,
        }
    }

    /// A rim circle on its cylinder chart is the `v = const` line —
    /// closed form, kept exact (spec §4).
    #[test]
    fn rim_circle_on_cylinder_is_the_v_const_line() {
        let (r, h) = (0.5, 1.0);
        let cyl = cylinder(r);
        let carrier = Curve3::Circle {
            center: Point3::new(0.0, 0.0, h),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let p = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let Pcurve::Harmonic { p0, pa, pb, pl } = p else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15 && (p0.y - h).abs() < 1e-15);
        assert!(pa.y.abs() < 1e-15 && pb.y.abs() < 1e-15 && pl.y.abs() < 1e-15);
        assert!((pl.x - 1.0).abs() < 1e-15);
        let cache =
            PcurveCache::certify(p, 0.0, FRAC_PI_2, &carrier, &cyl, wide_window(), band()).unwrap();
        assert!(cache.certificate().envelope < 1e-15);
    }

    /// A rim traversed the other way (the split lane's axis-flipped
    /// frame) is the `β = −1` winding — named, not approximated.
    #[test]
    fn reversed_rim_takes_the_negative_winding() {
        let (r, h) = (0.5, 1.0);
        let cyl = cylinder(r);
        let carrier = Curve3::Circle {
            center: Point3::new(0.0, 0.0, h),
            axis: -Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let p = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let Pcurve::Harmonic { pl, .. } = p else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!((pl.x + 1.0).abs() < 1e-15);
        PcurveCache::certify(p, 0.0, FRAC_PI_2, &carrier, &cyl, wide_window(), band()).unwrap();
    }

    /// A meridian (seam) line on a cylinder chart is the `u = const`
    /// line: the azimuth channel is a single stored `α`, so there is no
    /// branch to choose per sample — the wrong unwrap is
    /// unrepresentable (module docs).
    #[test]
    fn meridian_line_on_cylinder_is_the_u_const_line() {
        let r = 0.5;
        let cyl = cylinder(r);
        let carrier = Curve3::Line {
            origin: Point3::new(r, 0.0, 0.0),
            dir: Vec3::unit_z(),
        };
        let p = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let Pcurve::Harmonic { p0, pa, pb, pl } = p else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15 && p0.y.abs() < 1e-15);
        assert!(pa.x.abs() < 1e-15 && pb.x.abs() < 1e-15 && pl.x.abs() < 1e-15);
        assert!((pl.y - 1.0).abs() < 1e-15);
        PcurveCache::certify(p, 0.0, 1.0, &carrier, &cyl, wide_window(), band()).unwrap();
    }

    /// The SAME seam edge, on the SAME surface, carries two DIFFERENT
    /// pcurves — the `u = 0` and `u = 2π` branches. This is the
    /// under-keying counterexample in miniature (the body-level row
    /// lives in `topo`): a per-edge-per-face key cannot hold both.
    #[test]
    fn a_seam_edge_carries_two_different_branches_of_one_surface() {
        let r = 0.5;
        let cyl = cylinder(r);
        let carrier = Curve3::Line {
            origin: Point3::new(r, 0.0, 0.0),
            dir: Vec3::unit_z(),
        };
        let base = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let wrapped = base
            .shift_branch(1.0, TAU)
            .expect("a harmonic image always shifts");
        let wrapped_for_shift = wrapped.clone();
        let Pcurve::Harmonic { p0: a, .. } = base else {
            panic!("the closed-form lane stores harmonic images")
        };
        let Pcurve::Harmonic { p0: b, .. } = wrapped else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!((b.x - a.x - TAU).abs() < 1e-15, "different chart curves");
        // Both certify against the same carrier and the same surface —
        // the chart is periodic, so both branches map to the same locus.
        let w = ChartWindow {
            u_min: -1.0,
            u_max: 1.0,
            v_min: -1.0,
            v_max: 2.0,
        };
        PcurveCache::certify(base, 0.0, 1.0, &carrier, &cyl, w, band()).unwrap();
        let w2 = ChartWindow {
            u_min: TAU - 1.0,
            u_max: TAU + 1.0,
            v_min: -1.0,
            v_max: 2.0,
        };
        PcurveCache::certify(wrapped_for_shift, 0.0, 1.0, &carrier, &cyl, w2, band()).unwrap();
        // And each escapes the OTHER face's window — typed, not silent.
        assert!(matches!(
            PcurveCache::certify(wrapped, 0.0, 1.0, &carrier, &cyl, w, band()),
            Err(PcurveCertifyError::TrimEscape)
        ));
    }

    /// A conic in a plane chart maps coefficient by coefficient (the
    /// chart map is affine) and certifies to rounding.
    #[test]
    fn conic_in_a_plane_chart_is_exact() {
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.0, 0.5),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let carrier = Curve3::Ellipse {
            center: Point3::new(0.1, -0.2, 0.5),
            axis: Vec3::unit_z(),
            major: 0.7,
            minor: 0.3,
            u_ref: Vec3::unit_x(),
        };
        let p = chart_pcurve(&carrier, &plane, band()).unwrap();
        let cache =
            PcurveCache::certify(p, 0.2, 2.0, &carrier, &plane, wide_window(), band()).unwrap();
        assert!(cache.certificate().max_residual < 1e-15);
        assert!(cache.certificate().envelope < 1e-15);
    }

    /// A perturbed stored pcurve FAILS certification. Every coefficient
    /// of the certified family is visible to the nine-sample schedule
    /// (four coefficients, nine samples), and the closed-form envelope
    /// bounds the whole span — so there is no "between the samples"
    /// hiding place to construct (module docs).
    #[test]
    fn a_corrupted_pcurve_fails_typed() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let good = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let Pcurve::Harmonic { p0, pa, pb, pl } = good else {
            panic!("the closed-form lane stores harmonic images")
        };
        let nudge = 1e-3;
        let corruptions = [
            Pcurve::Harmonic {
                p0: Point2::new(p0.x + nudge, p0.y),
                pa,
                pb,
                pl,
            },
            Pcurve::Harmonic {
                p0: Point2::new(p0.x, p0.y + nudge),
                pa,
                pb,
                pl,
            },
            Pcurve::Harmonic {
                p0,
                pa: Vec2::new(pa.x, pa.y + nudge),
                pb,
                pl,
            },
            Pcurve::Harmonic {
                p0,
                pa,
                pb: Vec2::new(pb.x, pb.y + nudge),
                pl,
            },
        ];
        for (i, bad) in corruptions.into_iter().enumerate() {
            let out = PcurveCache::certify(bad, 0.0, PI, &carrier, &cyl, wide_window(), band());
            assert!(
                matches!(out, Err(PcurveCertifyError::ResidualExceeded { .. })),
                "corruption {i} certified: {out:?}"
            );
        }
    }

    /// The envelope is a genuine sup bound over the whole span, not a
    /// sampled max: it dominates a dense resampling of the residual.
    #[test]
    fn the_envelope_dominates_a_dense_resampling() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let Pcurve::Harmonic { p0, pa, pb, pl } = chart_pcurve(&carrier, &cyl, band()).unwrap()
        else {
            panic!("the closed-form lane stores harmonic images")
        };
        // A deliberately imperfect pcurve, so the envelope is not a
        // degenerate zero.
        let bad = Pcurve::Harmonic {
            p0,
            pa: Vec2::new(pa.x, pa.y * 1.01),
            pb,
            pl,
        };
        let Some(image) = chart_image_harmonic(
            &bad,
            &cyl,
            ChartWindings {
                u: Winding::Pos,
                v: None,
            },
        ) else {
            panic!("lane")
        };
        let cform = carrier_harmonic(&carrier).unwrap();
        let envelope = (image.c - cform.c).norm()
            + (image.a - cform.a).norm()
            + (image.b - cform.b).norm()
            + (image.l - cform.l).norm() * PI;
        for k in 0..=2048 {
            let t = PI * (f64::from(k) / 2048.0);
            let q = bad.eval(t);
            let d = cyl.eval(q.x, q.y).distance(carrier.eval(t));
            assert!(d <= envelope, "sample {d:e} exceeds envelope {envelope:e}");
        }
    }

    /// The dense-resampling sup of a pcurve against its carrier — an
    /// independent oracle for the envelope rows (no closed form on
    /// either side of the comparison).
    fn true_sup(p: &Pcurve<f64>, s: &Surface<f64>, c: &Curve3<f64>, t0: f64, t1: f64) -> f64 {
        let mut sup: f64 = 0.0;
        for k in 0..=8192 {
            let t = t0 + (t1 - t0) * (f64::from(k) / 8192.0);
            let q = p.eval(t);
            sup = sup.max(s.eval(q.x, q.y).distance(c.eval(t)));
        }
        sup
    }

    /// **The snap-slack row** (adopted from the adversarial review's
    /// envelope probe, assertion flipped to the fixed behaviour).
    ///
    /// The winding trilean admits `pl.x = β + δ` for any `δ·r ≤ ε`, so
    /// `certify` accepts a pcurve an ε-shell outside the exact harmonic
    /// family — and the closed-form envelope is computed from the
    /// SNAPPED image. Before the fix the stored envelope read 5.6e-17
    /// against a true sup of 9.4e-10 (false by seven orders). The
    /// stored envelope must dominate the true sup for **every** input
    /// certification admits, not only for the exact-in-family caches
    /// the minting lane produces.
    #[test]
    fn envelope_dominates_a_winding_snapped_pcurve() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let Pcurve::Harmonic { p0, pa, pb, pl } = chart_pcurve(&carrier, &cyl, band()).unwrap()
        else {
            panic!("the closed-form lane stores harmonic images")
        };
        // δ·r just inside the Zero band at the default ε = 1e-9; the
        // drift residual r·δ·t peaks at ~0.94e-9 at the last schedule
        // sample, so the 9-sample limb passes as well.
        let delta = 0.3e-9 / r;
        let drifted = Pcurve::Harmonic {
            p0,
            pa,
            pb,
            pl: Vec2::new(pl.x + delta, pl.y),
        };
        let Ok(cache) = PcurveCache::certify(
            drifted.clone(),
            0.0,
            PI,
            &carrier,
            &cyl,
            wide_window(),
            band(),
        ) else {
            // At a tighter ε row the snap does not admit it at all —
            // also honest, and nothing left to check.
            return;
        };
        let stored = cache.certificate().envelope;
        let sup = true_sup(&drifted, &cyl, &carrier, 0.0, PI);
        assert!(
            stored >= sup,
            "stored envelope {stored:e} under-reports the true sup {sup:e}"
        );
        // And it stays O(ε): the slack is the discarded drift, not a
        // blanket pad that would make the certificate useless.
        assert!(
            stored < 4.0 * sup + 1e-15,
            "slack {stored:e} vs sup {sup:e}"
        );
        // The sampled max stays the SAMPLED max — the two statements do
        // not get folded into one number.
        assert!(cache.certificate().max_residual <= sup * (1.0 + 1e-12));
    }

    /// An exact-in-family (minted-shape) cache pays no slack: the
    /// snap term is bitwise zero, so the fix costs the ship path
    /// nothing.
    #[test]
    fn a_minted_shape_cache_pays_no_snap_slack() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let p = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let Pcurve::Harmonic { pa, pb, pl, .. } = p else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert_eq!((pa.x, pb.x), (0.0, 0.0));
        assert_eq!(pl.x, 1.0);
        let cache =
            PcurveCache::certify(p, 0.0, PI, &carrier, &cyl, wide_window(), band()).unwrap();
        assert!(cache.certificate().envelope < 1e-14);
    }

    /// The review probe's second attack, kept: tuned near-cancelling
    /// harmonic + linear corruptions cannot hide under the envelope
    /// (the residual lives in `span{1, cos, sin, t}` and the envelope
    /// dominates it termwise), and certification refuses them.
    #[test]
    fn cancellation_cannot_beat_the_envelope() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let Pcurve::Harmonic { p0, pa, pb, pl } = chart_pcurve(&carrier, &cyl, band()).unwrap()
        else {
            panic!("the closed-form lane stores harmonic images")
        };
        let d = 1e-4;
        let combos = [
            (d, -d, 0.0, 0.0),
            (d, -d, 2.0 * d / PI, -d),
            (-d, d, -2.0 * d / PI, d),
            (d, d, -2.0 * d / PI, 0.0),
        ];
        for (ca, cb, cl, c0) in combos {
            let bad = Pcurve::Harmonic {
                p0: Point2::new(p0.x, p0.y + c0),
                pa: Vec2::new(pa.x, pa.y + ca),
                pb: Vec2::new(pb.x, pb.y + cb),
                pl: Vec2::new(pl.x, pl.y + cl),
            };
            let env = c0.abs() + ca.abs() + cb.abs() + cl.abs() * PI;
            let sup = true_sup(&bad, &cyl, &carrier, 0.0, PI);
            assert!(
                sup <= env * (1.0 + 1e-12),
                "sup {sup:e} beats envelope {env:e} for ({ca},{cb},{cl},{c0})"
            );
            assert!(
                PcurveCache::certify(bad, 0.0, PI, &carrier, &cyl, wide_window(), band()).is_err(),
                "an off-by-1e-4 pcurve certified"
            );
        }
    }

    /// The chart-derivation pins across the frontier's two flips
    /// (S9 lineage: `frontier_charts_refuse_typed` until M6-3): the
    /// sphere and cone closed-form classes DERIVE, and the surviving
    /// refusals are class-named carriers outside every honest route —
    /// never a silent fallback (C5).
    #[test]
    fn chart_closed_forms_derive_and_offlane_classes_refuse_typed() {
        // CONSTRUCTION arm, flipped from the sphere-chart refusal pin
        // (M5 S13; the S9 pattern): the equator on its own sphere
        // chart is the closed-form azimuth-linear image u = t, v = 0.
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let carrier = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let Pcurve::Harmonic { p0, pa, pb, pl } = chart_pcurve(&carrier, &sphere, band()).unwrap()
        else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15 && p0.y.abs() < 1e-15);
        assert!((pl.x - 1.0).abs() < 1e-15 && pl.y.abs() < 1e-15);
        assert!(pa.x.abs() < 1e-15 && pa.y.abs() < 1e-15);
        assert!(pb.x.abs() < 1e-15 && pb.y.abs() < 1e-15);
        // What stays refused on the sphere chart, TYPED: a tilted
        // small circle's azimuth is non-harmonic — never fitted (C5).
        let tilt = 0.6_f64;
        let tilted = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(tilt.sin(), 0.0, tilt.cos()),
            radius: 1.0,
            u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
        };
        let err = chart_pcurve(&tilted, &sphere, band()).unwrap_err();
        assert!(matches!(err, PcurveCertifyError::UnsupportedCarrier));
        // SECOND flip (M6-3, the S9 pattern again): the cone chart's
        // frontier refusal is retired — a genuine rim circle now
        // derives its closed form (azimuth `α + β·t`, slant constant
        // v₀ = h / cos α).
        let ha = 0.5_f64;
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: ha,
            u_ref: Vec3::unit_x(),
        };
        let h = 2.0_f64;
        let rim = Curve3::Circle {
            center: Point3::new(0.0, 0.0, h),
            axis: Vec3::unit_z(),
            radius: h * ha.tan(),
            u_ref: Vec3::unit_x(),
        };
        let Pcurve::Harmonic { p0, pl, .. } = chart_pcurve(&rim, &cone, band()).unwrap() else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15 && (p0.y - h / ha.cos()).abs() < 1e-12);
        assert!((pl.x - 1.0).abs() < 1e-15 && pl.y.abs() < 1e-15);
        // What stays refused on the cone chart, TYPED and class-named:
        // a tilted-section ELLIPSE — azimuth-non-harmonic, and the
        // cone has no ring-computable meters composite for a fitted
        // certificate either (ssi/certify docs), so neither route is
        // honest.
        let section = Curve3::Ellipse {
            center: Point3::new(0.2, 0.0, 2.0),
            axis: Vec3::new(0.3_f64.sin(), 0.0, 0.3_f64.cos()),
            major: 1.2,
            minor: 1.0,
            u_ref: Vec3::new(0.3_f64.cos(), 0.0, -0.3_f64.sin()),
        };
        let err = chart_pcurve(&section, &cone, band()).unwrap_err();
        assert!(matches!(err, PcurveCertifyError::UnsupportedCarrier));
    }

    /// A pcurve that winds more than one full period around the chart
    /// refuses typed — the chart side of the winding gate.
    #[test]
    fn azimuth_beyond_one_period_refuses_typed() {
        let r = 0.5;
        let cyl = cylinder(r);
        let carrier = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let p = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let out = PcurveCache::certify(p, 0.0, TAU + 0.5, &carrier, &cyl, wide_window(), band());
        assert!(matches!(
            out,
            Err(PcurveCertifyError::AzimuthPeriodExceeded)
        ));
    }

    /// The cone's azimuth lever DOMINATES the local one everywhere
    /// either object lives. `azimuth_lever` at a single `v` is the
    /// parallel radius there (`|v|·sin alpha`); the arm the checks take
    /// is that lever at the `|v|` supremum of the pcurve's box and the
    /// face window together, so it is an upper bound at every `v` in
    /// either — the direction that cannot under-state an escape or a
    /// winding. Both signs of `v` (both nappes) are swept.
    #[test]
    fn the_cone_azimuth_lever_dominates_the_local_lever_over_box_and_window() {
        let half_angle = 0.5_f64.atan();
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle,
            u_ref: Vec3::unit_x(),
        };
        let boxed = ChartWindow {
            u_min: 0.0,
            u_max: FRAC_PI_2,
            v_min: -3.0,
            v_max: 1.5,
        };
        let window = ChartWindow {
            u_min: -0.5,
            u_max: PI,
            v_min: 0.25,
            v_max: 2.25,
        };
        let (arm, v_arm) = chart_arms_at(&cone, &boxed, &window);
        assert!(
            (v_arm - 1.0).abs() < 1e-15,
            "the cone's v IS a slant length"
        );
        for w in [&boxed, &window] {
            for i in 0..=64 {
                let t = f64::from(i) / 64.0;
                let v = w.v_min + (w.v_max - w.v_min) * t;
                let local = azimuth_lever(&cone, v.abs());
                assert!(
                    local <= arm + 1e-15,
                    "the arm {arm:e} must dominate the local lever {local:e} at v = {v:e}"
                );
            }
        }
        // And it is the lever AT the supremum, not something larger:
        // an arm bigger than the geometry demands would refuse honest
        // work. |v| tops out at 3 (the box's lower edge).
        assert!((arm - azimuth_lever(&cone, 3.0)).abs() < 1e-15);
    }

    /// A carrier whose meter collapses refuses AT THE METER — no
    /// forward verdict is fabricated from a rate that cannot convert a
    /// span to metres, and the refusal is `Invalid`, distinct from the
    /// backwards-span verdict the metered check below it names.
    #[test]
    fn a_collapsed_carrier_meter_refuses_rather_than_metering_a_span() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let point = Point3::new(1.0, 2.0, 3.0);
        let net = NurbsCurve3::new(knots, vec![point, point], vec![1.0, 1.0]).unwrap();
        let carrier = Curve3::Nurbs(Arc::new(net));
        assert!(
            param_rate(&carrier).is_nan(),
            "a net that never moves states no speed bound at all"
        );
        let cause = param_rate_gate(&carrier, band())
            .expect_err("a poison meter cannot license a metered span");
        assert!(matches!(cause.margin, geom_core::MarginDiag::Invalid));
        assert_eq!(cause.predicate, Some("pcurve_interval_meter"));
        // A HEALTHY net of the same shape licenses its span, so the
        // refusal above is the meter's, not the lane's.
        let ok_knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let ok = NurbsCurve3::new(
            ok_knots,
            vec![point, Point3::new(1.0, 2.0, 4.0)],
            vec![1.0, 1.0],
        )
        .unwrap();
        let rate = param_rate_gate(&Curve3::Nurbs(Arc::new(ok)), band()).unwrap();
        assert!((rate - 1.0).abs() < 1e-15, "the unit-chord net meters at 1");
    }
}

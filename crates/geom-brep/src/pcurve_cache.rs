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
//! [`PcurveCache`] is therefore constructible only through its
//! certifying doors ([`PcurveCache::certify`],
//! [`PcurveCache::certify_fitted`], [`PcurveCache::certify_general`],
//! [`PcurveCache::certify_projected`])
//! or from a cache one of them built (its fields are private), exactly as
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
//! # The certified statement (C4)
//!
//! `|S(P(t)) − C(t)| ≤ ε` — a **3-D displacement in metres** between
//! the surface-composed pcurve and the carrier cache over the whole
//! edge, bounded by an envelope whose own statement the certificate
//! NAMES ([`PcurveCertificate::statement`]), because the lanes do not
//! bound the same thing.
//!
//! - **Where `S ∘ P` has a closed form, the envelope alone is the
//!   certified statement.** For a [`Pcurve::Harmonic`] image it is the
//!   carrier's incidence with the chart plus the stored image's
//!   fidelity to the image re-derived from the carrier, both on the
//!   chart's orthonormalised frame, plus that frame's distance from the
//!   stored one ([`EnvelopeStatement::MapResidualClosedForm`]), so a minted row's
//!   identity is a theorem rather than a trig round trip. The shared
//!   [`crate::CERT_SAMPLES`] schedule is the closed-form tables'
//!   cross-check: it runs on the witness lane (a scalar whose
//!   [`geom_core::Witness`] is `Inexact`: `f64`, `Sym<f64>`, the
//!   driver's point witness) and as the property test
//!   `envelope_lemma_fuzz`, and not at a certifying scalar (`Interval`,
//!   `Sym<Interval>`), over a box or at a point alike.
//! - **On an analytic chart, a carrier with no closed-form image** (a
//!   spline carrier, a sphere's general circle) stores its projected
//!   image, the chart's own inverse applied to the carrier, and its
//!   envelope is again the whole certified statement
//!   ([`EnvelopeStatement::MapResidualProjected`], [`projected`]'s
//!   docs): exact, not a fit.
//! - **On a NURBS chart, where no closed form exists**, a fitted image's
//!   certificate is the displacement at the shared schedule plus a
//!   between-samples envelope over the same displacement, C2's limb-2
//!   composite hull against the chart alone
//!   ([`EnvelopeStatement::MapResidualComposite`]).
//!
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
//! the harmonic (chart, carrier) pairs of the certified lane, both
//! `S ∘ P` and `C` lie in the four-dimensional function space
//! `span{1, cos t, sin t, t}` with **exactly computable coefficients**,
//! so the residual does too, and
//!
//! ```text
//! sup |D(t)| ≤ |Δ₀| + |Δ_a| + |Δ_b| + |Δ_l|·max(|t₀|, |t₁|)
//! ```
//!
//! is a certified sup bound over the *whole* span with nothing sampled.
//! On a periodic chart the `Δ`s are not formed from the image's angle:
//! the bound is the carrier's INCIDENCE with the chart, read off its
//! own coefficients, plus the image's FIDELITY to the one re-derived
//! from the carrier ([`EnvelopeStatement::MapResidualClosedForm`]
//! carries the lemma per chart arm). The same SHAPE of argument, in a different function space or
//! as an outright identity, is what every other closed-form lane here
//! states: the iso rungs' control-difference hull, the spiric cap's
//! `span{1, f, sin t}`, and the spiric wall's algebraic identity
//! ([`EnvelopeStatement`] names which sup each one bounds)
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
//! One boundary on that corollary: certification admits an **ε-shell
//! around the family** in each image's real coefficients (a phase, a
//! free channel's four), so a pcurve in that shell is not exactly the
//! derivation's. The envelope's fidelity term carries that drift
//! explicitly — every coefficient's offset from the re-derived image,
//! metered at the chart's arm — so the stored bound dominates the true
//! sup for every input the gate admits, not merely for the
//! exact-in-family caches the minting lane produces. An angle's
//! windings are no part of the shell: they are integers, compared
//! exactly ([`PcurveCertifyError::WindingMismatch`]).
//!
//! # Domain validity (spec §3)
//!
//! A row certifies against its carrier and its chart alone, on **one
//! branch**: on a periodic chart the azimuth channel of a
//! [`Pcurve::Harmonic`] is a [`Channel::Angle`], a *single* stored phase
//! and an integer count of windings, so `phase + w·t` with `w ∈ {−1, 0,
//! 1}` on every closed-form table. There is no per-sample branch choice
//! to get wrong: the M2 PR 5 meridian finding ("nearest-previous
//! per-sample unwrapping is a bug") is generalized here by making the
//! wrong unwrap **unrepresentable** — a τ jump cannot be expressed. The
//! row's azimuth spans at most one period because its carrier's
//! interval does (the edge certificate's `interval_span_winding`,
//! [`crate::CertifyError::WindingExceeded`]) and its windings are at
//! most one per carrier period.
//!
//! No caller's window enters the certificate. A row is its placed
//! image (C4), and the same row a whole period over is a lifted copy,
//! which certification refuses as a row: check 4 reads the period as
//! its offset. Which lift (`α + kτ`) a half-edge's walk reads is the
//! loop's fact, decided in `topo::pcurves`: at each joint the walk
//! decides the deck element as an integer ([`Period::decide`]), with
//! half the step to the next orbit point as room (half a period, or a
//! quarter on a sphere, whose twin sits half a period over), and each
//! loop winds at most one period per channel. A joint's
//! 3-D coincidence is not a chart fact either: it follows from the two
//! rows' envelopes (check 4) and the edge certificate's pinning of each
//! carrier's ends to its vertices.
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
use geom_core::spline::{KnotVector, SpanLocate, SplineError, constant_weight};
use geom_core::{
    Decide, Decided, Indeterminate, InfSpeed, Margin, Point2, Point3, Real, Sign, SupSpeed, Vec2,
    Vec3,
};

pub(crate) use geom::Window;
pub use geom::{ANGLE_WINDOW_START, Azimuth, LATITUDE_WINDOW_START};

use crate::certify::{CERT_SAMPLES, sample_param};
use crate::harmonic::{harmonic_sup, harmonic_sup_scalar};
use crate::ladder::{IdentityLadder, read_identity};
use crate::recourse::{
    ReadAt, Reading, RefusedArm, RefusedSide::StoredContradiction, SizedDecision, SpanDecision,
    Unsized,
};
use crate::ssi::{SsiDecision, SsiLimb};

/// **One chart coordinate of a harmonic image** ([`Pcurve::Harmonic`]).
///
/// An angle of a periodic chart and a coordinate with no period are
/// different kinds of thing, so they are different variants: the
/// angle's slope is not a number the row holds but a whole number of
/// windings, structure a certificate compares exactly (C4, "windings
/// are integers counted in the channel's period"). The variant a
/// chart's coordinate takes is the chart's: `u` is an angle on every
/// periodic chart, `v` on a sphere and a torus, and neither on a plane
/// (certification refuses a row whose channels are not its chart's,
/// [`PcurveCertifyError::ChannelKindMismatch`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Channel<T> {
    /// An angle of a periodic chart, `phase + w·t`: the image advances
    /// `w` whole periods of the chart's channel per period of its
    /// carrier ([`angle_rate`]).
    Angle {
        /// The angle at `t = 0`.
        phase: T,
        /// The windings `w`, whole periods per carrier period.
        winding: i32,
    },
    /// A coordinate with no period, `c + a·cos t + b·sin t + l·t`.
    Free {
        /// The constant term.
        c: T,
        /// The `cos t` coefficient.
        a: T,
        /// The `sin t` coefficient.
        b: T,
        /// The linear coefficient.
        l: T,
    },
}

impl<T: Real> Channel<T> {
    /// An angle with no windings: a constant azimuth or polar angle.
    pub fn fixed_angle(phase: T) -> Self {
        Channel::Angle { phase, winding: 0 }
    }

    /// A constant coordinate with no period.
    pub fn constant(c: T) -> Self {
        Channel::Free {
            c,
            a: T::zero(),
            b: T::zero(),
            l: T::zero(),
        }
    }

    /// The channel's four coefficients `(c, a, b, l)` of
    /// `c + a·cos t + b·sin t + l·t`: an angle's are its phase, two
    /// structural zeros and its windings' rate ([`angle_rate`]).
    pub fn coefficients(self) -> [T; 4] {
        match self {
            Channel::Angle { phase, winding } => [phase, T::zero(), T::zero(), angle_rate(winding)],
            Channel::Free { c, a, b, l } => [c, a, b, l],
        }
    }

    /// The channel translated by `s`: an angle's phase, a free
    /// channel's constant. Total: a translation touches no winding.
    #[must_use]
    pub fn shifted(self, s: T) -> Self {
        match self {
            Channel::Angle { phase, winding } => Channel::Angle {
                phase: phase + s,
                winding,
            },
            Channel::Free { c, a, b, l } => Channel::Free { c: c + s, a, b, l },
        }
    }

    /// The windings of an angle channel; `None` on a free one.
    pub fn winding(self) -> Option<i32> {
        match self {
            Channel::Angle { winding, .. } => Some(winding),
            Channel::Free { .. } => None,
        }
    }

    /// The channel under `x ↦ ±x (+ shift)` ([`AxisMap`]): an angle's
    /// windings flip with a reflection, and its phase and a free
    /// channel's constant take the shift. The one place a reflection
    /// touches a winding.
    ///
    /// `None` where a reflection meets a count with no negation in
    /// `i32` (`i32::MIN`): no count is invented for it.
    pub fn mapped(self, map: AxisMap<T>) -> Option<Self> {
        let lin = |x: T| if map.flip { -x } else { x };
        Some(match self {
            Channel::Angle { phase, winding } => Channel::Angle {
                phase: map.point(phase),
                winding: if map.flip {
                    winding.checked_neg()?
                } else {
                    winding
                },
            },
            Channel::Free { c, a, b, l } => Channel::Free {
                c: map.point(c),
                a: lin(a),
                b: lin(b),
                l: lin(l),
            },
        })
    }
}

/// **A winding as a slope**: `winding` whole periods of the chart's
/// angle per period of its carrier, read through the two periods it is
/// counted in ([`Period::rate`]). Both periods are `τ` here, by
/// assumption rather than read: every analytic chart's angle has period
/// `τ` (the spline charts carry no harmonic image), and so does every
/// carrier with a harmonic form (a circle, an ellipse; a line winds 0).
/// A channel holds no chart, so there is no chart `Period` to read at
/// this site; a chart whose angle had another period would have to pass
/// its own.
pub fn angle_rate<T: Real>(winding: i32) -> T {
    let tau = Period::new(T::tau());
    tau.rate(winding, tau)
}

/// **One chart axis's map**, `x ↦ ±x (+ shift)`: the maps a chart's
/// rows take, one axis at a time — a whole-period branch translation,
/// a reflection, and the sphere involution's half-turn and mirror. An
/// axis map keeps an angle an angle, which a general affine map of the
/// chart would not.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisMap<T> {
    /// Whether the axis is reflected.
    pub flip: bool,
    /// The translation after the reflection, if any: `x + s`, or
    /// `s − x` when reflected.
    pub shift: Option<T>,
}

impl<T: Real> AxisMap<T> {
    /// The identity.
    pub fn identity() -> Self {
        Self {
            flip: false,
            shift: None,
        }
    }

    /// `x ↦ x + s`.
    pub fn shift(s: T) -> Self {
        Self {
            flip: false,
            shift: Some(s),
        }
    }

    /// `x ↦ −x`.
    pub fn reflect() -> Self {
        Self {
            flip: true,
            shift: None,
        }
    }

    /// `x ↦ s − x`.
    pub fn reflect_about(s: T) -> Self {
        Self {
            flip: true,
            shift: Some(s),
        }
    }

    /// The map on a coordinate.
    pub fn point(self, x: T) -> T {
        match (self.flip, self.shift) {
            (false, None) => x,
            (false, Some(s)) => x + s,
            (true, None) => -x,
            (true, Some(s)) => s - x,
        }
    }

    /// The map's linear part on a displacement.
    pub fn vector(self, x: T) -> T {
        if self.flip { -x } else { x }
    }
}

/// **A map of the chart, axis by axis** ([`Pcurve::map_affine`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartMap<T> {
    /// The first axis.
    pub u: AxisMap<T>,
    /// The second axis.
    pub v: AxisMap<T>,
}

impl<T: Real> ChartMap<T> {
    /// The sphere chart's involution `(u, v) ↦ (u + π, π − v)`:
    /// `S(u + π, π − v) = S(u, v)` on a sphere, so every sphere image
    /// has this second name, its twin.
    pub fn sphere_involution() -> Self {
        Self {
            u: AxisMap::shift(T::pi()),
            v: AxisMap::reflect_about(T::pi()),
        }
    }

    /// Whether the map reflects either axis.
    pub fn reflects(self) -> bool {
        self.u.flip || self.v.flip
    }

    /// The map on a chart point.
    pub fn point(self, p: Point2<T>) -> Point2<T> {
        Point2::new(self.u.point(p.x), self.v.point(p.y))
    }

    /// The map's linear part on a chart displacement.
    pub fn vector(self, d: Vec2<T>) -> Vec2<T> {
        Vec2::new(self.u.vector(d.x), self.v.vector(d.y))
    }
}

/// Why [`Pcurve::map_affine`] has no image of a row under a map.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartMapRefusal {
    /// A reflection met a spiric wall image, which rides one sense on
    /// both channels.
    WallReflected,
    /// A reflection met an angle channel whose count has no negation in
    /// `i32`.
    WindingUnnegatable,
}

/// A pcurve: the 2-D chart image of an edge's carrier, parameterized by
/// **the carrier's own parameter** (module docs).
///
/// The closed enum is the D3 shape: a new variant is a
/// compiler-guided edit at every dispatch site, and the count is
/// deliberately not written here — a number in prose is a second,
/// unchecked copy of something the compiler already knows.
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
/// lane bounds, and [`crate::FittedLane`] for which scalars can derive
/// it at all.
///
/// [`Pcurve::IsoLine`] (M6-3) and [`Pcurve::IsoArc`] (M8-3) are the two
/// NURBS-chart iso rungs: both images are exactly straight in UV,
/// and they are separate variants because their MOVING CHANNEL differs
/// — `IsoLine`'s is the carrier's own parameter, `IsoArc`'s is the
/// chart's rational-quadratic Bézier parameter, related to the arc
/// angle by a transcendental piecewise map. Each variant's own docs
/// carry the derivation.
///
/// **Not `Copy`.** Three variants carry heap payloads — `Fitted` and
/// `General` an `Arc<NurbsCurve2>` each, `IsoArc` a `Vec`-backed
/// [`geom_core::spline::KnotVector`] — so [`Pcurve`] and
/// [`PcurveCache`] are `Clone` only, and any one of them alone is
/// enough for that. Removing one does not make the enum `Copy`.
#[derive(Clone, Debug)]
pub enum Pcurve<T: Real> {
    /// The closed-form chart image, one [`Channel`] per chart
    /// coordinate: `P(t) = (u(t), v(t))`.
    ///
    /// A coordinate that is an angle of a periodic chart (the azimuth
    /// on every periodic chart, and the second angle on a sphere or a
    /// torus) is a [`Channel::Angle`]: a phase and a whole number of
    /// windings, structure the row stores and certification compares
    /// exactly (C4). Every other coordinate is a [`Channel::Free`]
    /// `c + a·cos t + b·sin t + l·t`. That covers every (chart,
    /// carrier) pair the C5 table mints: a rim circle on its cylinder
    /// chart (`u` winding `β`, `v` constant), a seam or meridian line
    /// (`u` winding 0, `v` linear), a tilted-section conic on its
    /// cylinder chart (`u` winding `β`, `v` the sinusoid), the cone,
    /// sphere and torus rims and meridians, and any conic or line in a
    /// plane chart (both channels free: the chart map is affine, so the
    /// carrier's own `{1, cos, sin, t}` form maps through coefficient by
    /// coefficient). [`Pcurve::harmonic_form`] reads the image as its
    /// four coefficient vectors.
    Harmonic {
        /// The first chart coordinate.
        u: Channel<T>,
        /// The second chart coordinate.
        v: Channel<T>,
    },
    /// The **fitted** chart image: a 2-D NURBS curve whose parameter is
    /// the carrier's own (type docs; the OQ4 identity is the entry
    /// requirement, not a hope): an SSI trace's chart projection over a
    /// rung-3 carrier on a SPLINE chart, certified by the C2
    /// certificate's control hulls. An analytic chart holds no fitted
    /// image: its image of a carrier with no closed form is the
    /// projected one ([`Pcurve::Projected`]).
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
    /// It certifies at exactly the Fitted GRADE — the same four checks,
    /// check 4 the hull bound against its chart, through
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
    /// payoff — no fit anywhere). A `u = const` line may sit on an
    /// INTERIOR column of its chart (a chart wider than the face it
    /// trims): the certificate's hull then rests on the de Boor
    /// collapse rather than a boundary row, and the variant is the same
    /// one — nothing reads "boundary" off it.
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
        /// The arc's total turn: the image's arc is
        /// `[t0, t0 + angle]`, the whole boundary row, and its edge's
        /// span lies in it (a built rim's span is the arc; an edge
        /// covering part of the row, such as a split rim's child, keeps
        /// the row's arc). Always POSITIVE on a certified cache — check
        /// 2 refuses a non-forward interval and a span outside the arc
        /// (`pcurve_iso_arc_span`) — so the map's own sign cancellation
        /// (variant docs) never has a sign to carry.
        angle: T,
        /// Sub-arc breakpoints on `τ ∈ [0, 1]`: a uniform clamped
        /// degree-1 knot vector with one span per sub-arc.
        breaks: KnotVector,
    },
    /// The **exact chart image of a [`geom::Curve3::Spiric`]** on the
    /// charts a spiric lies on — the plane of its own cutting section,
    /// and the two tori that hold the section: the one it sections and
    /// that torus's mirror through the cutting plane. Data-free: `major`, `minor`
    /// and `offset` are the CARRIER's own three scalars, copied so the
    /// image evaluates from `t` alone like every other variant, and
    /// there is no fitted net anywhere.
    ///
    /// The one function both images are built from is the carrier's
    /// own radial half-width
    /// `f(t) = √((R + r·cos t)² − d²)` ([`geom::spiric_radial`], the
    /// single spelling of the radicand): on the CAP the chart image is
    /// the affine chart's coordinates of `c + n·d + m·f(t) + a·r·sin t`
    /// with `n·d` folded into the constant term, and on the WALL the
    /// chart azimuth is `u₀ + σμ·atan2(f(t), d)` while the minor angle
    /// IS the parameter (`σ` the axis sign, `μ` the [`WallTorus`]'s). `f > 0` everywhere in the two-oval regime
    /// `|d| < R − r`, so `atan2(f, d) ∈ (0, π)` is smooth in `t`, never
    /// crosses `atan2`'s cut and never winds — which is why the
    /// azimuth channel needs no branch datum beyond its constant.
    ///
    /// **Why a variant and not a [`Pcurve::Fitted`] net.** Both images
    /// are closed forms of the carrier's own parameter, and the
    /// certified statement they support is an ALGEBRAIC IDENTITY
    /// ([`EnvelopeStatement::SpiricIdentity`] for the wall, the
    /// `span{1, f, sin}` closed form for the cap) rather than a bound
    /// on a fitting error: a fitted image certifies only to its fit,
    /// while these images are exact.
    Spiric {
        /// The carrier's major radius `R` (metres).
        major: T,
        /// The carrier's minor radius `r` (metres).
        minor: T,
        /// The carrier's signed plane stand-off `d` (metres).
        offset: T,
        /// Which chart kind this image is on (and, on a torus, which
        /// of the two tori), with its own chart-space coefficients.
        image: SpiricImage<T>,
    },
    /// The **exact chart image of a focal section**: a conic on a
    /// cone or torus chart whose projection onto the plane normal to
    /// the chart's axis is an ellipse with the AXIS AT A FOCUS. Two
    /// carriers have one: a plane×cone section ellipse on its cone, and
    /// a Villarceau circle on its torus.
    ///
    /// ```text
    /// u(t) = u0 + sense·(E + 2·atan2(β·sin E, 1 − β·cos E)),  E = t − t0
    /// v(t) = v0 + va·cos t + vb·sin t + vl·t
    /// ```
    ///
    /// on the carrier's own parameter `t` (the conic's eccentric
    /// anomaly, measured from the vertex at `t0`).
    ///
    /// # The azimuth
    ///
    /// The projection's focus is on the axis, so the chart azimuth is
    /// the projected ellipse's true anomaly while `E` is its eccentric
    /// anomaly. Kepler's relation between the two, written cut-free, is
    /// `ν = E + 2·atan2(β·sin E, 1 − β·cos E)` with
    /// `β = e / (1 + √(1 − e²))` for the projection's eccentricity `e`,
    /// SIGNED: positive when the vertex at `E = 0` is the one nearer
    /// the axis. `|β| < 1`, so `1 − β·cos E > 0` and the `atan2` never
    /// reaches its cut; `ν` is strictly increasing, which is why the
    /// azimuth needs no branch datum beyond `u0`. At `β = 0` this is
    /// a rim's or parallel's `u0 + sense·E`.
    ///
    /// # The two instances
    ///
    /// - **Cone section** (`t0 = 0`, `vl = 0`): the slant is the axial
    ///   height over `cos α`, a linear functional of the point, so `v`
    ///   is harmonic. On the cone the distance from the axis is
    ///   `tan α` times the height, affine on the cutting plane — the
    ///   focus–directrix property that puts the axis at a focus.
    /// - **Villarceau circle** (`va = vb = 0`, `vl = ±1`): the circle
    ///   of radius `R` centred `r` from the torus centre along `d̂ ⊥`
    ///   the axis, in the plane through the torus centre that holds `d̂`
    ///   and is tilted `asin(r/R)` off the equator. Its projection has
    ///   semi-major `R` and focal distance `r`, so its distance from
    ///   the axis is `R + r·cos E` measured from the vertex `t0` farthest
    ///   from the axis, `β = −r/(R + √(R² − r²))`, and the tube angle is
    ///   `v = vl·(t − t0)` exactly: `R + r·cos v` is that distance and
    ///   `r·sin v` the height.
    ///
    /// Every field is read off the carrier and the chart: no fit, and
    /// the image evaluates at every scalar.
    ///
    /// **Why a variant and not a [`Pcurve::Fitted`] net**: the image is
    /// a closed form of the carrier's own parameter, and its check-4
    /// statement is closed form too (`run_focal_section_checks`) —
    /// whereas a fitted image on a cone or torus chart has no
    /// certificate, since the chart's meters composite needs a root.
    FocalSection(FocalImage<T>),
    /// The **projected image** of a carrier with no closed-form image on
    /// an analytic chart — a spline carrier, or a circle on a sphere
    /// that is neither a parallel nor a meridian: the chart's own
    /// inverse applied to the carrier, `P(t) = ψ(C(t))`, evaluated from
    /// the carrier written in the chart's frame on one `atan2` branch
    /// per piece. Exact, not a fit; its envelope bounds the
    /// displacement over the whole span as the carrier's distance from
    /// the chart plus the stored data's fidelity to the carrier
    /// ([`EnvelopeStatement::MapResidualProjected`];
    /// `pcurve_cache::projected`'s docs carry the channels and the
    /// lemma). Boxed: it is the largest image by far, and every
    /// stored row would otherwise pay for it.
    Projected(Box<ProjectedImage<T>>),
}

/// The azimuth channel of a [`Pcurve::FocalSection`] image (variant
/// docs), at the eccentric anomaly `e = t − t0`.
fn focal_section_azimuth<T: Real>(u0: T, beta: T, sense: T, e: T) -> T {
    let (s, c) = e.sin_cos();
    let lead = (beta * s).atan2(T::one() - beta * c);
    u0 + sense * (e + lead + lead)
}

/// The chart box of a [`Pcurve::FocalSection`] image over `[t0, t1]`:
/// the azimuth's endpoint hull (it is monotone) and the second
/// channel's harmonic box.
fn focal_section_span_box<T: Real>(image: &FocalImage<T>, t0: T, t1: T) -> ChartWindow<T> {
    let zero = T::zero();
    let second = harmonic_span_box(
        Point2::new(zero, image.v0),
        Vec2::new(zero, image.va),
        Vec2::new(zero, image.vb),
        Vec2::new(zero, image.vl),
        t0,
        t1,
    );
    let (ua, ub) = (image.azimuth(t0), image.azimuth(t1));
    ChartWindow {
        u_min: ua.min(ub),
        u_max: ua.max(ub),
        v_min: second.v_min,
        v_max: second.v_max,
    }
}

/// The data of a [`Pcurve::FocalSection`] image: its eight numbers,
/// read as the variant's docs state.
#[derive(Clone, Copy, Debug)]
pub struct FocalImage<T: Real> {
    /// The chart azimuth at the vertex, `E = 0`.
    pub u0: T,
    /// The carrier parameter at the vertex.
    pub t0: T,
    /// The second channel's constant term.
    pub v0: T,
    /// The second channel's `cos t` coefficient.
    pub va: T,
    /// The second channel's `sin t` coefficient.
    pub vb: T,
    /// The second channel's `t` coefficient.
    pub vl: T,
    /// The signed Kepler parameter, `|β| < 1`.
    pub beta: T,
    /// `+1` when the azimuth runs with the chart frame, `−1` against
    /// it; exactly `±1` on every minted image.
    pub sense: T,
}

impl<T: Real> FocalImage<T> {
    fn azimuth(&self, t: T) -> T {
        focal_section_azimuth(self.u0, self.beta, self.sense, t - self.t0)
    }

    fn second(&self, t: T) -> T {
        let (s, c) = t.sin_cos();
        self.v0 + self.va * c + self.vb * s + self.vl * t
    }
}

/// Which chart a [`Pcurve::Spiric`] image lives on, and its chart-space
/// coefficients — the cutting plane and the two tori that hold a
/// `Curve3::Spiric`, and no others (a spiric lies on no cylinder, cone
/// or sphere, and [`PcurveCache::certify`] admits the variant on
/// `Plane` and `Torus` charts only).
#[derive(Clone, Copy, Debug)]
pub enum SpiricImage<T: Real> {
    /// The **plane cap**: `P(t) = p0 + pm·f(t) + pa·sin t`, with
    /// `f(t) = √((R + r·cos t)² − d²)`.
    ///
    /// Derived coefficient by coefficient through the plane's affine
    /// chart from the carrier's own form
    /// `C(t) = c + n·d + m·f(t) + a·(r·sin t)`:
    /// `p0 = chart(c + n·d − origin)`, `pm = chart(m)`,
    /// `pa = chart(a·r)`. Affine charts commute with the carrier's
    /// linear combination, so the image is exact and its residual
    /// against the carrier is a constant-coefficient combination of
    /// `{1, f, sin}` — the closed form check 4 bounds.
    Cap {
        /// The constant term in chart coordinates.
        p0: Point2<T>,
        /// The `f(t)` coefficient.
        pm: Vec2<T>,
        /// The `sin t` coefficient.
        pa: Vec2<T>,
    },
    /// The **torus wall**: `u(t) = u0 + μ·sense·atan2(f(t), d)`,
    /// `v(t) = v0 + sense·t`, with `μ` = [`WallTorus::mu`] of `torus`.
    ///
    /// Derived from the chart formula
    /// `S(u, v) = c′ + e(u)·(R + r cos v) + A·(r sin v)`. Two tori hold
    /// the oval (`spiric_off_own_chart`'s docs): the carrier's own,
    /// `c′ = c` ([`WallTorus::Own`], `μ = +1`), and its MIRROR through
    /// the cutting plane, `c′ = c + 2d·n` ([`WallTorus::Mirror`],
    /// `μ = −1`). About either, the carrier's point
    /// `c + n·d + m·f + a·(r sin t)` has radial part `μ·n·d + m·f` of
    /// length `R + r cos t`, and height `a·(r sin t)`. With `A = σ·a`
    /// (`σ = sense = ±1`, decided at the mint by
    /// `pcurve_spiric_chart_axis`) the height gives `v = σ·t`, and the
    /// +90° direction from `μ·n` is `A × μn = σμ·m`, so the radial part
    /// has azimuth `φ_{μn} + σμ·atan2(f, d)`. So ONE sign `σ` rides
    /// both channels, the azimuth's opposed on the mirror, and the only
    /// derived numbers in the whole image are `u0` (one `atan2` of
    /// `μ·n` in the chart frame) and `v0` (`0` at the mint, `k·τ` after
    /// the loop walk's branch shift). That is what makes the wall's
    /// between-samples statement an identity rather than a bound:
    /// [`EnvelopeStatement::SpiricIdentity`].
    Wall {
        /// The chart azimuth at `f = d`'s angle zero — the azimuth of
        /// `μ·n` in the chart frame.
        u0: T,
        /// The minor-angle constant: `0` at the mint, `k·τ` after a
        /// branch shift.
        v0: T,
        /// `σ`: `+1` when the chart axis agrees with the carrier's,
        /// `−1` when it opposes; exactly `±1` on every minted image.
        sense: T,
        /// Which of the two tori that hold the oval the image maps
        /// through.
        torus: WallTorus,
    },
}

/// Which of the two tori that hold a spiric oval a
/// [`SpiricImage::Wall`] maps through: a tag, so a wall image names
/// exactly one of them and its azimuth sign is `σ·μ` by construction
/// rather than a second stored number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WallTorus {
    /// The carrier's own torus, centred at the carrier's centre `c`.
    Own,
    /// The carrier's torus mirrored through the cutting plane, centred
    /// at `c + 2d·n`.
    Mirror,
}

impl WallTorus {
    /// `μ`: `+1` on the own torus, `−1` on the mirror — the azimuth
    /// channel's sign relative to the minor-angle channel's.
    #[must_use]
    pub fn mu<T: Real>(self) -> T {
        match self {
            Self::Own => T::one(),
            Self::Mirror => T::zero() - T::one(),
        }
    }

    /// The torus's centre for a spiric of centre `c`, plane normal `n`
    /// and offset `d`: `c` or `c + n·(d + d)`. The ONE spelling both
    /// the mint and check 1 read, so check 1's centre comparand is
    /// bit-zero on a chart the mint was handed with this centre.
    #[must_use]
    pub fn center<T: Real>(self, c: Point3<T>, n: Vec3<T>, d: T) -> Point3<T> {
        match self {
            Self::Own => c,
            Self::Mirror => c + n * (d + d),
        }
    }
}

/// The chart parameter `g(t) ∈ [0, 1]` of an [`Pcurve::IsoArc`]
/// (variant docs for the derivation). Total: a malformed `breaks`
/// answers poison rather than panicking (D4).
fn iso_arc_g<T: SpanLocate>(t: T, t0: T, angle: T, breaks: &KnotVector) -> T {
    let spans = iso_arc_spans(breaks);
    if spans == 0 {
        return T::from_f64(f64::NAN);
    }
    #[allow(clippy::cast_precision_loss)]
    let m = T::from_f64(spans as f64);
    let h = angle / m;
    // tan(h/4) through sin/cos — no transcendental beyond the scalar's
    // own `sin_cos` (the same door every harmonic pcurve uses).
    let (s_q, c_q) = (h * T::from_f64(0.25)).sin_cos();
    let tan_q = s_q / c_q;
    let x = (t - t0) / angle;
    // A poison parameter locates no span; `g` is poison in every
    // channel `x` carries.
    let Some(set) = x.locate_spans(breaks) else {
        return geom_core::spline::poison_from(x);
    };
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
    // walk's entry/exit, `chart_box`), and `locate_spans`
    // already saturated the span choice off-domain, so no caller sees
    // a changed answer — but a future off-domain one would, and that
    // is a property of this function, not an accident of its callers.
    acc.map(|g| g.max(T::zero()).min(T::one()))
        .unwrap_or_else(|| T::from_f64(f64::NAN))
}

/// **The harmonic image's span box.** Per channel
/// `P(t) = c + l·t + T(t)` with `T(t) = a·cos t + b·sin t`; the box is
/// the SUM of the linear part's box and the trigonometric part's, each
/// true on its own:
///
/// - **linear**: `hull(c + l·t₀, c + l·t₁)` — exact, the part is monotone.
/// - **trigonometric**: with `M = hypot(a, b)` and `h = |t₁ − t₀|`, the
///   MEET of the chord enclosure `hull(T(t₀), T(t₁)) ± M·h²/8` (`|T″| =
///   |T| ≤ M`, and a C² function leaves its chord over a width-`h`
///   interval by at most `sup|T″|·h²/8`) and the ball `[−M, M]`, which
///   is the tighter once the span passes a few radians (a full turn's
///   chord charge is `M·π²/2`).
///
/// A channel with only one of the two parts — every image a
/// constructor mints — gets the meet of the whole channel's chord and
/// ball. **Restriction-monotone**: a sub-span's box lies inside the
/// span's. The linear
/// box is monotone exactly; the trigonometric box is pinned by
/// `tests/chart_box_span.rs`'s sub-span fuzz. Summing the parts rather
/// than taking one chord of the whole channel is what buys it: a
/// mixed channel's whole-channel chord is not monotone, and costs
/// `|T(t₁) − T(t₀)|` of tightness at most.
///
/// **At every scalar.** At `f64` each end is the round-to-nearest value
/// of a true bound, so it can miss the exact image by rounding error in
/// the chart coordinates, a rounding-scale miss that check 2's
/// headroom meters through the band. At `Interval` every operation
/// is outward-rounded and [`Real::min`]/[`Real::max`] are the envelope
/// extremes, so each end encloses the true bound for every `t₀`, `t₁`
/// in their enclosures.
fn harmonic_span_box<T: Real>(
    p0: Point2<T>,
    pa: Vec2<T>,
    pb: Vec2<T>,
    pl: Vec2<T>,
    t0: T,
    t1: T,
) -> ChartWindow<T> {
    let (s0, c0) = t0.sin_cos();
    let (s1, c1) = t1.sin_cos();
    let dip = (t1 - t0).powi(2) * T::from_f64(0.125);
    let channel = |c: T, a: T, b: T, l: T| {
        let (l0, l1) = (c + l * t0, c + l * t1);
        let amp = (a.powi(2) + b.powi(2)).sqrt();
        let (e0, e1) = (a * c0 + b * s0, a * c1 + b * s1);
        let lo = (e0.min(e1) - amp * dip).max(T::zero() - amp);
        let hi = (e0.max(e1) + amp * dip).min(amp);
        (l0.min(l1) + lo, l0.max(l1) + hi)
    };
    let (u_min, u_max) = channel(p0.x, pa.x, pb.x, pl.x);
    let (v_min, v_max) = channel(p0.y, pa.y, pb.y, pl.y);
    ChartWindow {
        u_min,
        u_max,
        v_min,
        v_max,
    }
}

/// A harmonic image's four coefficient vectors `(p0, pa, pb, pl)`, of
/// `p0 + pa·cos t + pb·sin t + pl·t` ([`Pcurve::harmonic_form`]).
pub type HarmonicForm<T> = (Point2<T>, Vec2<T>, Vec2<T>, Vec2<T>);

/// A harmonic image's two channels read as its four coefficient
/// vectors ([`HarmonicForm`]), channel by channel
/// ([`Channel::coefficients`]): total, for a caller that already holds
/// the channels of a [`Pcurve::Harmonic`].
pub fn harmonic_form_of<T: Real>(u: Channel<T>, v: Channel<T>) -> HarmonicForm<T> {
    let ([cu, au, bu, lu], [cv, av, bv, lv]) = (u.coefficients(), v.coefficients());
    (
        Point2::new(cu, cv),
        Vec2::new(au, av),
        Vec2::new(bu, bv),
        Vec2::new(lu, lv),
    )
}

impl<T: Real> Pcurve<T> {
    /// A harmonic image read as its four coefficient vectors
    /// `(p0, pa, pb, pl)` of `p0 + pa·cos t + pb·sin t + pl·t`, channel
    /// by channel ([`Channel::coefficients`]); `None` for every other
    /// variant.
    pub fn harmonic_form(&self) -> Option<HarmonicForm<T>> {
        let Pcurve::Harmonic { u, v } = *self else {
            return None;
        };
        Some(harmonic_form_of(u, v))
    }

    /// A harmonic image whose two channels are free: an image on a
    /// chart with no period (a plane), from its four coefficient
    /// vectors.
    pub fn free_harmonic(p0: Point2<T>, pa: Vec2<T>, pb: Vec2<T>, pl: Vec2<T>) -> Self {
        Pcurve::Harmonic {
            u: Channel::Free {
                c: p0.x,
                a: pa.x,
                b: pb.x,
                l: pl.x,
            },
            v: Channel::Free {
                c: p0.y,
                a: pa.y,
                b: pb.y,
                l: pl.y,
            },
        }
    }

    /// **Whether a derivation places this kind of image** over its edge's
    /// span (C4): a closed-form or projected image is derived again over
    /// any sub-span and named in its chart's window there, while a fitted,
    /// general or iso image has no angular placement and restricts. One
    /// fact each kind answers, so a new kind cannot fall to restriction
    /// silently: the match has no wildcard.
    #[must_use]
    pub fn is_placed(&self) -> bool {
        match self {
            Pcurve::Harmonic { .. }
            | Pcurve::FocalSection(_)
            | Pcurve::Spiric { .. }
            | Pcurve::Projected(_) => true,
            Pcurve::Fitted(_)
            | Pcurve::General(_)
            | Pcurve::IsoLine { .. }
            | Pcurve::IsoArc { .. } => false,
        }
    }

    /// [`Pcurve::chart_box`] for the closed-form images, whose box needs
    /// no span location — [`Pcurve::Harmonic`] and
    /// [`Pcurve::FocalSection`] — at every [`Real`] scalar: the door for
    /// a caller with none, which `chart_box` needs for its NURBS arms.
    /// `None` for every other variant. The construction, its enclosure
    /// argument and its restriction monotonicity are
    /// `harmonic_span_box`'s and `focal_section_span_box`'s.
    pub fn closed_form_span_box(&self, t0: T, t1: T) -> Option<ChartWindow<T>> {
        match *self {
            Pcurve::Harmonic { .. } => self
                .harmonic_form()
                .map(|(p0, pa, pb, pl)| harmonic_span_box(p0, pa, pb, pl, t0, t1)),
            Pcurve::FocalSection(ref image) => Some(focal_section_span_box(image, t0, t1)),
            _ => None,
        }
    }

    /// The same image under a **map of the chart, axis by axis**
    /// ([`ChartMap`], `x ↦ ±x (+ shift)` on each axis): the constant
    /// term of a harmonic image, an iso line's or arc's start, and a
    /// NURBS image's control net take the map; the `cos`/`sin`/linear
    /// coefficients, an iso line's velocity and an iso arc's
    /// displacement take its linear part; a harmonic image's angle
    /// channel takes the shift on its phase and the reflection on its
    /// windings ([`Channel::mapped`]). Knots, weights, breaks and the
    /// carrier parameter are untouched, and every variant keeps its
    /// variant — a map of the chart moves an image, never its
    /// provenance. A NEW variant is a compile error here, not a
    /// fall-through.
    ///
    /// Exact: every variant is linear in its chart-space coefficients,
    /// so the coefficient-wise image IS the pointwise image of the
    /// curve, and a NURBS net goes through [`NurbsCurve2::map_points`]
    /// — the door a structural map takes instead of
    /// `NurbsCurve2::new`, since a pointwise map of a validated net has
    /// nothing left to validate.
    ///
    /// The map is axis by axis because those are the maps a chart's
    /// rows take — [`Pcurve::mirror_v`]'s reflection,
    /// [`Pcurve::shift_branch`]'s whole-period translation and the
    /// sphere's involution `(u + π, π − v)` — and because an axis map
    /// keeps an angle an angle, which a map mixing the axes would not.
    ///
    /// # Errors
    ///
    /// - [`ChartMapRefusal::WallReflected`]: [`SpiricImage::Wall`] rides
    ///   ONE `sense` on two channels (the azimuth's opposed on the mirror
    ///   torus, by its [`WallTorus`] tag rather than by a stored number),
    ///   so a map that reflects an axis has no wall image to land on: the
    ///   reflected channel would follow and the other would not. No torus
    ///   row takes a reflection (`(u, v) ↦ (u, −v)` reverses a torus
    ///   chart's orientation, and the torus whose frame realises it maps
    ///   the image to `(−u, −v)`), and `topo::revert` mirrors the rows of
    ///   plane faces only.
    /// - [`ChartMapRefusal::WindingUnnegatable`]: a reflection meets an
    ///   angle channel whose count has no negation in `i32`
    ///   ([`Channel::mapped`]).
    ///
    /// A map that reflects nothing refuses neither: [`Pcurve::translate`]
    /// is that map's total door.
    pub fn map_affine(&self, map: ChartMap<T>) -> Result<Self, ChartMapRefusal> {
        self.map_chart(
            map,
            |c, axis| c.mapped(axis).ok_or(ChartMapRefusal::WindingUnnegatable),
            || {
                if map.reflects() {
                    Err(ChartMapRefusal::WallReflected)
                } else {
                    Ok(())
                }
            },
        )
    }

    /// The same image translated by `(du, dv)` in the chart: a whole-
    /// period branch shift of either channel ([`Pcurve::shift_branch`])
    /// or a re-parameterising offset. [`Pcurve::map_affine`] with no
    /// reflection, and total: a translation's linear part is the
    /// identity, so every variant, a spiric wall's included, takes it
    /// exactly and no winding moves.
    #[must_use]
    pub fn translate(&self, du: T, dv: T) -> Self {
        let map = ChartMap {
            u: AxisMap::shift(du),
            v: AxisMap::shift(dv),
        };
        let shifted = self.map_chart(
            map,
            |c, axis| Ok::<_, core::convert::Infallible>(c.shifted(axis.point(T::zero()))),
            || Ok(()),
        );
        match shifted {
            Ok(image) => image,
            Err(never) => match never {},
        }
    }

    /// The one body of [`Pcurve::map_affine`] and
    /// [`Pcurve::translate`]: `channel` maps a harmonic channel, and
    /// `wall` says whether a spiric wall takes the map.
    fn map_chart<E>(
        &self,
        map: ChartMap<T>,
        channel: impl Fn(Channel<T>, AxisMap<T>) -> Result<Channel<T>, E>,
        wall: impl Fn() -> Result<(), E>,
    ) -> Result<Self, E> {
        let point = |p: Point2<T>| map.point(p);
        let vector = |d: Vec2<T>| map.vector(d);
        Ok(match self {
            Pcurve::Harmonic { u, v } => Pcurve::Harmonic {
                u: channel(*u, map.u)?,
                v: channel(*v, map.v)?,
            },
            Pcurve::Fitted(image) => Pcurve::Fitted(Arc::new(image.map_points(point))),
            Pcurve::General(image) => Pcurve::General(Arc::new(image.map_points(point))),
            Pcurve::IsoLine { p0, pl } => Pcurve::IsoLine {
                p0: point(*p0),
                pl: vector(*pl),
            },
            Pcurve::IsoArc {
                p0,
                pd,
                t0,
                angle,
                breaks,
            } => Pcurve::IsoArc {
                p0: point(*p0),
                pd: vector(*pd),
                t0: *t0,
                angle: *angle,
                breaks: breaks.clone(),
            },
            // The CAP image is linear in its chart-space coefficients
            // exactly as the harmonic one is — `p0` through `point`,
            // the `f` and `sin` coefficients through `vector` — and a
            // plane chart is where the reflection this map exists for
            // actually happens.
            //
            // The WALL image is not a general affine object and this
            // arm does not pretend otherwise. Its two channels ride
            // ONE `sense` (variant docs; the `WallTorus` tag fixes the
            // azimuth's sign relative to it), so a linear part that negates
            // the second channel alone has no wall image to land on:
            // the `v` channel would follow and the `u` channel would
            // not. What a torus chart's rows DO take is the two
            // whole-period branch translations — the azimuth shift
            // ([`Pcurve::shift_branch`]) and the minor-angle shift —
            // and both are exact here, since a translation's linear
            // part is the identity. Nothing hands this arm a
            // reflection: `topo::revert` mirrors the rows of PLANE
            // faces only and puts a non-plane chart's reversal on
            // `face.sense` instead, which is the same restriction the
            // iso arms record for a periodic shift of a non-periodic
            // chart.
            Pcurve::Spiric {
                major,
                minor,
                offset,
                image,
            } => Pcurve::Spiric {
                major: *major,
                minor: *minor,
                offset: *offset,
                image: match image {
                    SpiricImage::Cap { p0, pm, pa } => SpiricImage::Cap {
                        p0: point(*p0),
                        pm: vector(*pm),
                        pa: vector(*pa),
                    },
                    SpiricImage::Wall {
                        u0,
                        v0,
                        sense,
                        torus,
                    } => {
                        wall()?;
                        let p = point(Point2::new(*u0, *v0));
                        SpiricImage::Wall {
                            u0: p.x,
                            v0: p.y,
                            sense: *sense,
                            torus: *torus,
                        }
                    }
                },
            },
            // The two channels are separate functions of `t`, so a map
            // whose linear part is `diag(1, ±1)` — every map a cone or
            // torus chart's rows take: the branch translations of
            // either channel and the `v` reflection — lands exactly:
            // the constant pair through `point`, the second channel's
            // amplitudes through `vector`. A linear part that mixes the
            // channels has no image of this form, and nothing hands one
            // to such a row.
            Pcurve::FocalSection(image) => {
                let p = point(Point2::new(image.u0, image.v0));
                let second = |x: T| vector(Vec2::new(T::zero(), x)).y;
                Pcurve::FocalSection(FocalImage {
                    u0: p.x,
                    v0: p.y,
                    va: second(image.va),
                    vb: second(image.vb),
                    vl: second(image.vl),
                    ..*image
                })
            }
            // Every map a chart row takes has linear part `diag(1, ±1)`,
            // and the image's deck map absorbs it exactly.
            Pcurve::Projected(image) => {
                Pcurve::Projected(Box::new(image.map_affine(point, vector)))
            }
        })
    }

    /// The same image under the chart reflection `(u, v) ↦ (u, −v)`,
    /// for every variant that HAS one (the section below names the one
    /// that does not) —
    /// the map a chart's OWN frame undergoes when its second axis is
    /// negated (a `Plane` whose stored `normal` is negated with
    /// `u_ref` fixed has `v_ref = normal × u_ref` negated with it), so
    /// the mirrored image on the mirrored chart names the same locus,
    /// point for point. [`Pcurve::map_affine`] with the second channel
    /// negated on points and vectors alike: a sign flip on every `v`
    /// coefficient, knots, weights, breaks and the carrier parameter
    /// untouched. An IEEE sign flip is a bitwise involution, so
    /// `mirror_v ∘ mirror_v` is the identity bit for bit.
    ///
    /// **Why a certificate of the source is a certificate of the
    /// result** — stated once, here; the doors that carry one across
    /// ([`crate::PcurveCache::mirrored_v`],
    /// [`crate::EdgeCurve::with_chart_v_mirrored`]) point back rather
    /// than restate. A certificate is a record of metred NORMS —
    /// sampled residuals `|S(P(tᵢ)) − C(tᵢ)|`, an envelope over the
    /// span, on the fitted lane a hull sup-norm — and stores nothing in
    /// a chart's coordinates, so the one caller (`topo::revert`, which
    /// reflects plane charts only) carries it across unchanged. On the mirrored chart
    /// the mirrored image evaluates to the same 3-D point at every
    /// parameter: `S'(u, −v) = origin + u_ref·u + (−v_ref)·(−v)`, and
    /// `(−a)·(−b)` is `a·b` exactly in IEEE arithmetic, so every
    /// coordinate is bit-identical up to the sign of a zero, which a
    /// distance squares away (a squared zero is `+0`). So every number the schedule would produce again is
    /// the number it produced, and the certificate travels verbatim —
    /// the same reading [`crate::EdgeCurve::with_remapped_surfaces`]
    /// lands under: a re-statement under which no metred value can
    /// move is not a geometry change. Where the signed zero does land
    /// is the IMAGE (a coefficient that was `+0` reads `−0` after the
    /// flip), which a `Debug` comparison sees and no certificate can.
    /// # The variant that has no reflection
    ///
    /// [`SpiricImage::Wall`] is the one image whose reflected locus is
    /// not representable: `u = u₀ + σμ·atan2(f, d)`, `v = v₀ + σ·t`
    /// ride ONE sign `σ`, and the reflection wants `u` fixed while `v`
    /// negates. Nor is the reflected locus a wall image on any torus:
    /// `(u, v) ↦ (u, −v)` reverses the chart's orientation, the torus
    /// whose frame realises a reversal (axis negated) takes the image
    /// to `(−u, −v)`, and re-tagging [`WallTorus`] would mint a
    /// certified-shaped row naming the OTHER torus while holding on
    /// neither. So this door answers `None` for it rather than a wrong
    /// locus — measured before it was written: the arm that silently
    /// kept `sense` returned an image whose `v` was wrong by the entire
    /// span while `mirror_v ∘ mirror_v` still held, so an involution
    /// row could not see it.
    ///
    /// `None` is unreachable through every door in the tree:
    /// `topo::revert` mirrors the rows of PLANE faces only and puts a
    /// non-plane chart's reversal on `face.sense` instead, and a wall
    /// image lives on a torus chart by construction (check 1 admits it
    /// nowhere else). A caller that reaches it is holding a row on a
    /// face it does not belong to.
    ///
    /// # Errors
    ///
    /// `None` for a [`SpiricImage::Wall`] image, per the paragraph
    /// above, and for a harmonic image whose angle channel stores a
    /// count with no negation in `i32` ([`Pcurve::map_affine`]'s two
    /// refusals). Every other image answers `Some`, exactly.
    #[must_use]
    pub fn mirror_v(&self) -> Option<Self> {
        self.map_affine(ChartMap {
            u: AxisMap::identity(),
            v: AxisMap::reflect(),
        })
        .ok()
    }
}

impl<T: SpanLocate> Pcurve<T> {
    /// The chart point at the **carrier parameter** `t` (module docs:
    /// the parameter is not re-mapped). Fixed evaluation order (D9);
    /// total.
    pub fn eval(&self, t: T) -> Point2<T> {
        match self {
            Pcurve::Harmonic { u, v } => {
                let (p0, pa, pb, pl) = harmonic_form_of(*u, *v);
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
            // One `sin_cos`, one `sqrt` (inside `spiric_radial`), one
            // `atan2` on the wall — the carrier's own evaluation order
            // (D9), read from the parameter alone.
            Pcurve::Spiric {
                major,
                minor,
                offset,
                image,
            } => {
                let (s, c) = t.sin_cos();
                let (_, f) = geom::spiric_radial(*major, *minor, *offset, c);
                match image {
                    SpiricImage::Cap { p0, pm, pa } => {
                        Point2::new(p0.x + pm.x * f + pa.x * s, p0.y + pm.y * f + pa.y * s)
                    }
                    SpiricImage::Wall {
                        u0,
                        v0,
                        sense,
                        torus,
                    } => {
                        let u_sense = *sense * torus.mu::<T>();
                        Point2::new(*u0 + u_sense * f.atan2(*offset), *v0 + *sense * t)
                    }
                }
            }
            Pcurve::FocalSection(image) => Point2::new(image.azimuth(t), image.second(t)),
            Pcurve::Projected(image) => image.eval(t),
        }
    }

    /// A **conservative** chart-box enclosure of the pcurve over
    /// `[t₀, t₁]`: every point of the image over the span lies in it,
    /// so an extent read off it (check 2's azimuth headroom, the cone's
    /// `|v|` reach) can only be over-stated.
    /// A harmonic or iso-line image's box is `harmonic_span_box`'s, and
    /// a focal-section image's [`Pcurve::closed_form_span_box`]'s.
    ///
    /// **Restriction-monotone on every arm**: a sub-span's box lies
    /// inside the span's (the net and arc-segment arms ignore the span;
    /// the harmonic arm by its construction's docs).
    pub fn chart_box(&self, t0: T, t1: T) -> ChartWindow<T> {
        match self {
            Pcurve::Harmonic { u, v } => {
                let (p0, pa, pb, pl) = harmonic_form_of(*u, *v);
                harmonic_span_box(p0, pa, pb, pl, t0, t1)
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
            // A straight line is a harmonic image with no
            // trigonometric part, and its box is its endpoint hull.
            Pcurve::IsoLine { p0, pl } => {
                let zero = Vec2::new(T::zero(), T::zero());
                harmonic_span_box(*p0, zero, zero, *pl, t0, t1)
            }
            // The arc rim's chart image is the SEGMENT `p0 → p0 + pd`
            // (`g` is monotone in `t`: `tan` is monotone on
            // `(−π/2, π/2)` and every sub-arc's `φ/2` stays inside it).
            //
            // This is the WHOLE-SEGMENT box, and it deliberately
            // ignores `t0`/`t1`: sound at every window (a sub-window's
            // image is a sub-segment) and TIGHT at the full span. A
            // sub-span — what a restriction of a stored row to one
            // child of a parameter split asks for — gets a
            // conservative box, never a wrong one.
            Pcurve::IsoArc { p0, pd, .. } => {
                let b = Point2::new(p0.x + pd.x, p0.y + pd.y);
                ChartWindow {
                    u_min: p0.x.min(b.x),
                    u_max: p0.x.max(b.x),
                    v_min: p0.y.min(b.y),
                    v_max: p0.y.max(b.y),
                }
            }
            // Both images are built from `f(t)`, whose range over the
            // WHOLE period is `[f_min, f_max]` at `ρ = R ∓ r`
            // (`geom::spiric_f_range`), and from one bounded
            // trigonometric channel. So:
            //
            // - *Cap*: `p0 + pm·[f_min, f_max] + pa·[−1, 1]` — the
            //   amplitude box, the harmonic arm's own shape with `f`'s
            //   range in place of a unit cosine's.
            // - *Wall*: `atan2(f, d)` is MONOTONE in `f` at fixed `d`
            //   (increasing for `d > 0`, decreasing for `d < 0`,
            //   constant at `d = 0`), so the hull of its two ENDPOINT
            //   values over `[f_min, f_max]` is its exact range; the
            //   `v` channel is affine in `t`, so its endpoints are
            //   exact too.
            //
            // Whole-period in `f` like the carrier's own box, so a
            // sub-span gets a conservative answer, never a wrong one —
            // the enclosure every arm here keeps.
            Pcurve::Spiric {
                major,
                minor,
                offset,
                image,
            } => {
                let (f_min, f_max) = geom::spiric_f_range(*major, *minor, *offset);
                match image {
                    SpiricImage::Cap { p0, pm, pa } => {
                        let span = |k: T, a: T| {
                            let (lo, hi) = ((k * f_min).min(k * f_max), (k * f_min).max(k * f_max));
                            (lo - a.abs(), hi + a.abs())
                        };
                        let (du_lo, du_hi) = span(pm.x, pa.x);
                        let (dv_lo, dv_hi) = span(pm.y, pa.y);
                        ChartWindow {
                            u_min: p0.x + du_lo,
                            u_max: p0.x + du_hi,
                            v_min: p0.y + dv_lo,
                            v_max: p0.y + dv_hi,
                        }
                    }
                    SpiricImage::Wall {
                        u0,
                        v0,
                        sense,
                        torus,
                    } => {
                        let u_sense = *sense * torus.mu::<T>();
                        let (a_lo, a_hi) = (f_min.atan2(*offset), f_max.atan2(*offset));
                        let (u_a, u_b) = (*u0 + u_sense * a_lo, *u0 + u_sense * a_hi);
                        let (v_a, v_b) = (*v0 + *sense * t0, *v0 + *sense * t1);
                        ChartWindow {
                            u_min: u_a.min(u_b),
                            u_max: u_a.max(u_b),
                            v_min: v_a.min(v_b),
                            v_max: v_a.max(v_b),
                        }
                    }
                }
            }
            // The azimuth is strictly monotone in `t` (variant docs), so
            // its two endpoint values are its exact range over the span;
            // the second channel is a harmonic one and takes the
            // harmonic arm's box. Both are restriction-monotone.
            Pcurve::FocalSection(image) => focal_section_span_box(image, t0, t1),
            // Per overlapped piece, the channels' ranges over the piece's
            // sector hull (`projected`'s docs).
            Pcurve::Projected(image) => image.chart_box(t0, t1),
        }
    }

    /// The azimuth channel shifted by `k` whole periods, as a length
    /// only through [`Period::times`]: a lift of the row, which is what a
    /// joint's deck element applies (`topo`'s `Deck::apply`). A lift is
    /// the row with that element, never a row of its own (C4: a row is
    /// its placed image), so certification refuses a lifted image as a
    /// row. [`Pcurve::map_affine`] with the first channel of
    /// every POINT translated by `k·period` and every vector left as
    /// it is: a whole-period shift is a translation of the chart, so
    /// a harmonic image's constant term, an iso image's start and a
    /// NURBS image's control net move, while the `cos`/`sin`/linear
    /// coefficients, an iso velocity or displacement, knots, weights
    /// and the parameter do not. Exact and structure-preserving, and
    /// it moves the lift WITHOUT touching the parameter, which is why the
    /// shift is not a re-derivation: the row stays the placed one, and
    /// only the walk's deck element names a lift.
    ///
    /// The iso lane lives on NURBS charts, which have no periodic
    /// azimuth, so no minted iso line or arc rim is ever shifted; the
    /// map is kept total over them rather than special-cased — a
    /// shift by `k` periods of a non-periodic chart is meaningless
    /// but harmless, and the loop walk never computes a nonzero `k`
    /// there.
    #[must_use]
    pub fn shift_branch(&self, k: i32, period: Period<T>) -> Self {
        self.translate(period.times(k), T::zero())
    }
}

/// A chart-space axis-aligned box: the conservative enclosure of a
/// row's image over a span ([`Pcurve::chart_box`]).
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

    /// `sup |v|` over the window: the larger magnitude of its two `v`
    /// ends, which bounds `|v|` everywhere in between.
    fn v_reach(&self) -> T {
        self.v_min.abs().max(self.v_max.abs())
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
    /// The between-samples envelope — metres: the closed-form lanes'
    /// sum of terms, or the fitted lane's hull sup.
    Envelope,
    /// The closed-form envelope on a periodic chart, refused with this
    /// one term over the band on its own ([`EnvelopeTerm`]). A refusal
    /// no single term carries names [`PcurveCheck::Envelope`].
    EnvelopeTerm(EnvelopeTerm),
    /// The placement integer: the whole turns the image constant is
    /// placed by in its chart's window ([`Placement::turns`]).
    ChartWinding,
    /// A class or form selection at derivation (D3's lane choice): which
    /// closed form images the carrier on the chart — a rim, ruling,
    /// parallel or meridian, a section's nappe or sense, the winding `β`,
    /// the Villarceau ladder. Zero takes the closed form; in band and
    /// definite take the general lane, so a refusal under this check is
    /// one where no general lane is written yet for a carrier the chart's
    /// incidence test does not show off it (`incidence::class_escalated`
    /// takes that test's witness), and it ends lever-only
    /// "not yet" ([`crate::recourse::not_yet`]), never as a defect or a
    /// tolerance offer.
    ChartClass,
    /// The general lane's own reading at the mint, for a carrier the
    /// chart's incidence test does not show off it: a circle D3's lane
    /// choice sent there, or a spline, whose only image it is (the
    /// projected lane's sector and nappe readings, relabelled in one place,
    /// `general_lane_refusal`): the lane cannot image this pose yet, a
    /// capability gap, so it ends lever-only "not yet" like
    /// [`PcurveCheck::ChartClass`]. Check 4's reading of a STORED image
    /// stays [`PcurveCheck::Sector`].
    GeneralLane,
    /// An identity check on a stored image's copied data at check 4: the
    /// spiric image's scalars, sense and chart premise, the focal
    /// section's sense, `β` and tube rate. The kernel copies them
    /// exactly, so a miss is a defect.
    ImageIdentity,
    /// The pcurve's azimuth extent against one period.
    AzimuthPeriod,
    /// A projected image's tube-angle extent against one period on a
    /// torus chart (the projected lane's own gate).
    TubePeriod,
    /// A curved chart's incidence test (`chart_incidence`,
    /// `incidence_over_span`, the cone conic's apex-height read): whether
    /// the carrier lies on the chart. Only a genuine incidence reading
    /// raises it.
    ChartIncidence,
    /// A projected image's sector condition: each piece in one `atan2`
    /// sector, clear of the chart's singular set.
    Sector,
}

/// One term of check 4's envelope — a slot of the bound the lemmas on
/// [`EnvelopeStatement::MapResidualClosedForm`] and
/// [`EnvelopeStatement::MapResidualProjected`] state. The closed-form
/// envelope reads `Frame`, four **incidence** terms (`Centre` to
/// `Drift`: the carrier against the chart, read off the carrier's own
/// coefficients and the chart's data, with no image involved) and two
/// **fidelity** terms (`FidelityU`, `FidelityV`: the stored image
/// against the one `certify` re-derives from the carrier). The projected
/// envelope reads `Frame`, `Incidence`, `Fidelity` and the two channel
/// fidelities. Both keep one array and name a refusal in one order; the
/// certified bound adds every slot except that the closed-form lane on
/// a cylinder, sphere or torus reads the two channel fidelities as the
/// legs of one displacement (`EnvelopeTerms::total`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvelopeTerm {
    /// The carrier's centre is off the chart's axis — off the sphere's
    /// centre for a sphere meridian, off the spine for a torus meridian
    /// (its radial and axial parts as one vector). The torus parallel's
    /// rides in `Radius`, read with its pair.
    Centre,
    /// The carrier is not the class's circle or radius:
    /// - a turning class's `cos t`/`sin t` pair against the image's —
    ///   its radius, its winding and its plane read as one pair — on
    ///   the cylinder, the cone rim, the sphere's parallel and meridian
    ///   (a pole start's included) and the torus's meridian;
    /// - the torus parallel's whole residual: its constant `−w_r +
    ///   e·h·κ` read with its pair (`constant_and_pair`);
    /// - a meridian's constant radial against the cylinder's radius,
    ///   and a cone ruling's anchor off its generator;
    /// - a still class's (winding decided Zero) image constant at its
    ///   full length: `ρ`, or the torus's `R + r` plus the tube gap.
    Radius,
    /// A linear term the class has no room for: a cylinder line's
    /// radial direction, a cone ruling's direction off its generator,
    /// any circle class's `l`.
    Line,
    /// The motion a class admitted by its own decision: a cylinder
    /// meridian's radial pair `σ(a_r, b_r)` (its radial amplitude
    /// decided Zero), and a still class's whole pair `σ(a, b)`, axial
    /// part included (its winding decided Zero).
    Drift,
    /// The stored image's first channel against the placed one check 4
    /// re-derives, metered at the chart's arm for it: on the harmonic
    /// lane its constant's offset read as it is; on the focal-section,
    /// spiric and projected lanes, whose envelopes are 3-D, the offset
    /// past half the channel's period (`placement_terms`), which a
    /// lifted copy reads and a drift does not. Either way a whole period
    /// off is a lifted copy, no row. The projected lane adds its own deck
    /// map's term here too: its `u_off`'s distance from a whole number of
    /// periods (`off_period`), times the arm.
    FidelityU,
    /// The stored image's second channel, as [`EnvelopeTerm::FidelityU`]
    /// reads the first; a free (length) channel carries no placement
    /// term.
    FidelityV,
    /// The chart's frame is not orthonormal: its distance from its
    /// Gram–Schmidt twin over the stored image, levered by the radius
    /// and by the image's actual axial reach (the lemma's premise,
    /// metered rather than assumed).
    Frame,
    /// A projected image's carrier against the chart: the chart's
    /// implicit form composed along the carrier, in metres.
    Incidence,
    /// A projected image's stored carrier against the one re-derived
    /// from the edge's, through the projection's Lipschitz bound.
    Fidelity,
}

impl EnvelopeTerm {
    /// Every term, in the order a refusal is attributed (and the slots
    /// are added).
    const ALL: [EnvelopeTerm; 9] = [
        EnvelopeTerm::Frame,
        EnvelopeTerm::Centre,
        EnvelopeTerm::Radius,
        EnvelopeTerm::Line,
        EnvelopeTerm::Drift,
        EnvelopeTerm::FidelityU,
        EnvelopeTerm::FidelityV,
        EnvelopeTerm::Incidence,
        EnvelopeTerm::Fidelity,
    ];

    /// The term's slot in [`EnvelopeTerm::ALL`]. An exhaustive match,
    /// so a variant added to the enum does not compile until it has a
    /// slot, and the assertion below fails the build until `ALL` lists
    /// it there.
    const fn slot(self) -> usize {
        match self {
            EnvelopeTerm::Frame => 0,
            EnvelopeTerm::Centre => 1,
            EnvelopeTerm::Radius => 2,
            EnvelopeTerm::Line => 3,
            EnvelopeTerm::Drift => 4,
            EnvelopeTerm::FidelityU => 5,
            EnvelopeTerm::FidelityV => 6,
            EnvelopeTerm::Incidence => 7,
            EnvelopeTerm::Fidelity => 8,
        }
    }
}

const _: () = {
    let mut i = 0;
    while i < EnvelopeTerm::ALL.len() {
        assert!(
            EnvelopeTerm::ALL[i].slot() == i,
            "EnvelopeTerm::ALL is the slot order"
        );
        i += 1;
    }
};
/// The number a fitted-lane refusal carries, named for what it IS.
///
/// The fitted lane's definite refusals each measured something
/// different, so each rides its own named arm: flattening them onto one
/// anonymous number loses the only thing a reader needs, what the
/// number means. Both are quantities the image producer projected out
/// of an enclosure when it refused. A decided limb's own reading rides
/// [`PcurveCertifyError::FittedLimb`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FittedMagnitude {
    /// A certified foot point would not converge: the last distance the
    /// projection saw, in metres, at the schedule parameter it gave up
    /// on.
    LastFootDistance {
        /// The schedule parameter.
        t: f64,
        /// The last distance seen.
        last_distance: f64,
    },
    /// A certified foot point of an edge ENDPOINT would not converge:
    /// the last distance the projection saw, in metres. There is no
    /// schedule parameter — the endpoint is not a schedule position —
    /// so this arm carries none rather than a placeholder.
    EndpointFootDistance {
        /// The last distance seen.
        last_distance: f64,
    },
}

/// The closed kind tag of a [`Pcurve`] variant (D3), for a refusal
/// that names the image it was handed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PcurveKind {
    /// [`Pcurve::Harmonic`].
    Harmonic,
    /// [`Pcurve::IsoLine`].
    IsoLine,
    /// [`Pcurve::IsoArc`].
    IsoArc,
    /// [`Pcurve::Spiric`].
    Spiric,
    /// [`Pcurve::FocalSection`].
    FocalSection,
    /// [`Pcurve::Fitted`].
    Fitted,
    /// [`Pcurve::General`].
    General,
    /// [`Pcurve::Projected`].
    Projected,
}

impl PcurveKind {
    /// The kind of an image.
    pub fn of<T: Real>(p: &Pcurve<T>) -> Self {
        match p {
            Pcurve::Harmonic { .. } => Self::Harmonic,
            Pcurve::IsoLine { .. } => Self::IsoLine,
            Pcurve::IsoArc { .. } => Self::IsoArc,
            Pcurve::Spiric { .. } => Self::Spiric,
            Pcurve::FocalSection(_) => Self::FocalSection,
            Pcurve::Fitted(_) => Self::Fitted,
            Pcurve::General(_) => Self::General,
            Pcurve::Projected(_) => Self::Projected,
        }
    }

    /// The kind's display name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Harmonic => "Harmonic",
            Self::IsoLine => "IsoLine",
            Self::IsoArc => "IsoArc",
            Self::Spiric => "Spiric",
            Self::FocalSection => "FocalSection",
            Self::Fitted => "Fitted",
            Self::General => "General",
            Self::Projected => "Projected",
        }
    }
}

/// Typed pcurve-certification failure (D4 ¶3): actionable, closed enum.
#[derive(Clone, Debug, PartialEq)]
// The variant roster `topo`'s sample-coverage row reads (this
// crate's `test-support` feature, test builds only).
#[cfg_attr(
    feature = "test-support",
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(PcurveCertifyErrorKind), derive(strum::EnumIter), doc(hidden))
)]
pub enum PcurveCertifyError {
    /// The face's chart is outside the certified lane. Plane and
    /// cylinder charts have exact closed-form images for every carrier
    /// the C5 table mints; cone/sphere/torus charts and `Nurbs`
    /// surfaces keep derive-on-demand status until their consumers land
    /// (M5 PR 7 for SSI-produced pcurves, PR 11 for tessellation trim).
    /// This is a *routing decision*, permanent until a PR moves it —
    /// never a runtime fallback (C5).
    UnsupportedChart {
        /// The surface kind.
        chart: geom::SurfaceKind,
    },
    /// A carrier the chart's incidence test does not show off it went to
    /// the general lane (D3's lane choice, or a spline's only image), and
    /// no general image is there for it ([`LaneGap`] says why: none
    /// written for the pair, or the lane does not image this pose). A capability gap, never a defect: it
    /// ends lever-only "not yet" ([`PcurveCheck::ChartClass`] for an
    /// unwritten pair, [`PcurveCheck::GeneralLane`] for an uncovered pose).
    NoGeneralLane {
        /// The chart kind.
        chart: geom::SurfaceKind,
        /// The carrier kind.
        carrier: geom::CurveKind,
        /// Why there is no image.
        gap: LaneGap,
    },
    /// The carrier cannot lie on the chart, or is degenerate there: the
    /// edge is not on its face, a body defect no lane will ever image.
    CarrierOffChart {
        /// The chart kind.
        chart: geom::SurfaceKind,
        /// The carrier kind.
        carrier: geom::CurveKind,
        /// Why the pair has no common locus, named.
        why: &'static str,
        /// The margin the decision that refused read, where the refusal
        /// reads a degenerate pose (the cone's apex plane): decided past it,
        /// or a band-decided Zero not shown away from it (D4 ¶1 (i)).
        margin: Option<OffMargin>,
    },
    /// The image offered is not the carrier's image, or not this
    /// door's: a fitted or general image at the closed-form door, a
    /// harmonic image for a carrier with no `{1, cos, sin, t}` form, a
    /// spiric image whose stored scalars are not the carrier's. No
    /// constructor mints one, so it is a wiring defect or a hand-built
    /// image.
    ImageMismatch {
        /// The image kind.
        image: PcurveKind,
        /// What disagrees, named.
        why: &'static str,
    },
    /// A [`Pcurve::Fitted`] or [`Pcurve::General`] image needed the
    /// fitted door with **none in hand** — at check 4 of the fitted
    /// lane, where [`crate::FittedLane`] derives its hull bound, or
    /// in the mint, before any check, where it derives a general image.
    /// The door is absent either because the scalar may not certify (a
    /// dual, D1 2026-08-19: certification arithmetic, C9, is not
    /// reachable from it) or because a caller at a certifying scalar
    /// withheld it; this crate cannot tell which, and the refusal says
    /// only what is true of both.
    FittedLaneUnsupported {
        /// The scalar the check ran at ([`geom_core::Real::NAME`]).
        scalar: &'static str,
    },
    /// A projected image's piece is not in one `atan2` sector: its hull
    /// reaches, or comes within the band of, the chart's singular set —
    /// a sphere's pole, the cone's axis or apex, a torus's tube core —
    /// or a stored piece leaves the half-plane its branch centre names.
    /// The derivation refines pieces up to a cap before refusing; a
    /// carrier through the singular set has no one-branch image.
    SectorRefused {
        /// The piece (or, for [`projected::SectorChannel::Lever`], the
        /// carrier's knot span) that refused.
        piece: u32,
        /// Which channel's sector refused.
        channel: projected::SectorChannel,
    },
    /// An iso image was offered outside the iso lane's certified
    /// inventory, with the exact boundary named. The refused set is:
    /// a chart that is still the mvfs placeholder; a non-boundary ROW
    /// under the arc-rim class (the seam class certifies an interior
    /// COLUMN by the de Boor collapse, `crate::nurbs_iso`, the cap class
    /// an interior row the same way on the transposed chart, and both
    /// refuse one outside the domain); an
    /// interior column or row of a chart whose weight net varies along both
    /// parameters (the collapsed row's weights are computed, so no
    /// stored carrier shares its space); a DIAGONAL line in UV; a
    /// degenerate iso (neither chart channel moves); an image that
    /// leaves the chart's parameter domain, where the hull bound does
    /// not hold; a carrier whose spline structure is not the traversed
    /// column's, or whose weights are not positive (the convex-hull
    /// hypothesis itself); a seam-class image over a non-spline
    /// carrier; a LINE carrier on a RATIONAL column or row (the Greville hull
    /// is a linear-precision fact the rational basis does not have). A
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
    /// The row or column read from the chart would not re-wrap as a
    /// curve —
    /// unreachable for a chart that already validated, and surfaced
    /// with the spline layer's own refusal rather than swallowed
    /// (D4 ¶2).
    ChartRow {
        /// The spline layer's typed refusal.
        source: SplineError,
    },
    /// A certificate limb of the fitted lane refused definitely: which
    /// limb, and what the classifier saw of its value
    /// ([`geom_core::MarginDiag`], for the message only), as
    /// `edge_nurbs`' `Limb` carries it: a point at `f64`, and at
    /// `Interval` the enclosure, rendered `[lo, hi] m`. What that value
    /// measures here is this door's to say
    /// ([`PcurveCertifyError::ending`]).
    FittedLimb {
        /// The limb that refused.
        limb: SsiLimb,
        /// What the classifier saw of its value.
        margin: geom_core::MarginDiag,
    },
    /// The fitted lane's SSI certificate refused, **flattened to its
    /// three actionable parts** rather than nested whole.
    ///
    /// The triple IS the actionable content — which limb, why, and the
    /// number measured — which is what a consumer can act on and what
    /// this module's other refusals carry; nesting `SsiError` whole
    /// would add nothing a caller can act on. A limb's decided refusal
    /// is [`PcurveCertifyError::FittedLimb`].
    FittedCertificate {
        /// The SSI limb that refused, when the refusal names one.
        limb: Option<SsiLimb>,
        /// The refusal's own reason.
        what: &'static str,
        /// The number this refusal measured, named for what it IS —
        /// `None` when the refusal is structural and measured nothing.
        ///
        /// Never an escalation's diagnostic: every value reaching here
        /// belongs to a definite refusal. Escalations are a separate
        /// variant ([`PcurveCertifyError::FittedEscalated`]), which
        /// carries the classifier's diagnostic whole.
        magnitude: Option<FittedMagnitude>,
    },
    /// The general image's producer could not re-express its image on
    /// the carrier's own parameter domain: the domain door's refusal,
    /// carried whole, as the plane × NURBS lane carries it from the same
    /// producer ([`crate::PlaneNurbsRefusal::CarrierDomain`]).
    CarrierDomain(crate::edge_nurbs::CarrierDomainRefusal),
    /// The fitted lane's limb 2 has no certified net of the carrier
    /// over a span of the image's domain: the carrier-enclosure door's
    /// refusal, naming the carrier's kind and the span, carried whole as
    /// the edge certificate carries it
    /// ([`crate::PlaneNurbsRefusal::CarrierEnclosure`]).
    CarrierEnclosure(crate::CarrierEnclosureRefusal),
    /// A fitted-lane classification ESCALATED — D4 ¶3's
    /// escalate-never-guess, at the SSI door rather than at one of this
    /// module's own schedule checks. It ends by the SSI decision it
    /// names, as it does at the SSI door ([`SsiDecision::ending`]).
    ///
    /// The classifier's [`Indeterminate`] is carried WHOLE, so the
    /// margin, the band it was judged against and the predicate's name
    /// travel together: a margin without its band cannot be read.
    FittedEscalated {
        /// The SSI decision that escalated.
        decision: SsiDecision,
        /// The classifier's diagnostic, whole.
        cause: Indeterminate,
    },
    /// The stored carrier-parameter interval is not forward — the same
    /// `he_plus` contract [`crate::certify::CertifyError::IntervalNotForward`]
    /// enforces on the carrier itself.
    IntervalNotForward,
    /// The image stores windings no closed-form table holds — more than
    /// one turn per carrier period ([`Channel::Angle`]): a helix-like
    /// or multiply-wound image this lane cannot certify in closed form.
    /// Typed, never approximated.
    ChartWindingUnsupported,
    /// A harmonic image's channels are not its chart's: an angle where
    /// the chart's coordinate has no period, or a free channel where it
    /// is an angle ([`Channel`]). Structure, compared with no decision.
    ChannelKindMismatch {
        /// The surface kind.
        chart: geom::SurfaceKind,
    },
    /// A harmonic row's stored windings are not the windings its
    /// carrier's derivation names on its chart (C4: windings are
    /// structure a certificate compares exactly). `[u, v]`, `None` for a
    /// free channel.
    WindingMismatch {
        /// The row's windings.
        stored: [Option<i32>; 2],
        /// The derivation's.
        derived: [Option<i32>; 2],
    },
    /// The chart is the placeholder ([`Surface::is_placeholder_chart`]):
    /// it has no locus, so no lane can image a pcurve on it or meter a
    /// margin through it. Every lane refuses it with this, at check 1.
    PlaceholderChart,
    /// The pcurve's azimuth extent definitely exceeds one full period —
    /// the chart-side counterpart of
    /// [`crate::certify::CertifyError::WindingExceeded`].
    AzimuthPeriodExceeded,
    /// The edge's span sits on a mark of its chart's window more whole
    /// turns away than an `i32` counts (`Placement::turns`): its image
    /// names no copy the kernel can store.
    PlacementTurnsExceeded,
    /// A projected image's tube angle definitely sweeps more than one
    /// period on a torus chart. Only the projected lane emits it: the
    /// focal-section and spiric-wall lanes leave a span past a period to
    /// the edge certificate's `interval_span_winding`.
    TubePeriodExceeded,
    /// A certified residual definitely exceeded the tolerance band: the
    /// pcurve does not represent the carrier through the map (D4 ¶2).
    ResidualExceeded {
        /// The check that failed.
        check: PcurveCheck,
        /// The schedule sample index (0 for span-wide checks).
        sample: u32,
    },
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

impl PcurveCertifyError {
    /// Every arm's text but an escalation's, whose ending reads the door
    /// ([`PcurveCertifyError::render`]).
    fn write_payload(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsupportedChart { chart } => write!(
                f,
                "pcurve certification: no {}-chart lane covers this pcurve — every \
                 analytic chart certifies its closed-form (Harmonic) classes, a NURBS \
                 chart routes through its description-driven \
                 iso/fitted lanes instead of this door, and an image outside the chart's \
                 harmonic family belongs to the fitted lane where one exists",
                chart.name()
            ),
            Self::NoGeneralLane {
                chart,
                carrier,
                gap: LaneGap::Unwritten,
            } => write!(
                f,
                "pcurve certification: the {} carrier is on the {} chart but is none of its own \
                 curves, so no closed form images it, and its general image is not written yet \
                 for this pair",
                carrier.name(),
                chart.name()
            ),
            Self::NoGeneralLane {
                chart,
                carrier,
                gap: LaneGap::Pose { piece, channel },
            } => write!(
                f,
                "pcurve certification: the {} carrier is on the {} chart but is none of its own \
                 curves, and its general (projected) image does not yet cover this pose: piece \
                 {piece}: {}. Recourse: {POSE_LEVER}",
                carrier.name(),
                chart.name(),
                channel.describe()
            ),
            // The payload says what was read; the ending says what to do,
            // and it is the read's at the door ([`IncidenceRead::ending`]),
            // so no recourse is written here.
            Self::CarrierOffChart {
                chart,
                carrier,
                why,
                margin,
            } => match IncidenceRead::of_margin(*margin) {
                IncidenceRead::Off(margin) => write!(
                    f,
                    "pcurve certification: the {} carrier does not lie on the {} chart — \
                     {why}{}. The edge is not on its face",
                    carrier.name(),
                    chart.name(),
                    margin
                        .map(|m| format!(" (margin {m:e} m)"))
                        .unwrap_or_default()
                ),
                // Not shown off the chart: the carrier is read AT a pose
                // where no carrier of its kind lies on it, which says
                // nothing about whether the edge is on its face.
                IncidenceRead::AtPose(m) => write!(
                    f,
                    "pcurve certification: the {} carrier is no curve of the {} chart at this \
                     tolerance — {why} (margin {m:e} m, within the band)",
                    carrier.name(),
                    chart.name(),
                ),
                IncidenceRead::Undecided(_) => {
                    unreachable!("a decided incidence refusal carries no undecided read")
                }
            },
            Self::ImageMismatch { image, why } => write!(
                f,
                "pcurve certification: the {} image offered is not its carrier's image at \
                 this door — {why}. Recourse: re-mint the image from the carrier \
                 (chart_pcurve, or the body's pcurve mint) rather than building it by hand, \
                 and hand a Fitted or General image to certify_fitted or certify_general",
                image.name()
            ),
            Self::FittedLaneUnsupported { scalar } => write!(
                f,
                "pcurve certification: a fitted chart image needs the fitted door at the \
                 {scalar} scalar, which derives such an image and certifies its \
                 between-samples bound, and none was in hand. Only a scalar with certification rights holds the door, {holders} \
                 (the {sym} one over a certifying base): replay the body at one, or, if the run \
                 is at one already, pass the call its door",
                holders = crate::ScalarList(crate::FITTED_DOOR_HOLDERS),
                sym = <geom_core::Sym<f64> as geom_core::Real>::NAME,
            ),
            Self::IsoUnsupported { what } => write!(
                f,
                "pcurve certification: the iso-line lane refuses this class — {what}"
            ),
            Self::ChartRow { source } => write!(
                f,
                "pcurve certification: the chart's control net, or the row or column read \
                 from it, is not valid spline structure — {source}"
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
                    Some(FittedMagnitude::LastFootDistance { t, last_distance }) => format!(
                        " (the projection's last distance was {last_distance:e} m at t = {t:e})"
                    ),
                    Some(FittedMagnitude::EndpointFootDistance { last_distance }) => format!(
                        " (the projection's last distance at the endpoint was {last_distance:e} m)"
                    ),
                    None => String::new(),
                }
            ),
            Self::FittedLimb { limb, margin } => write!(
                f,
                "pcurve certification: the fitted lane's certificate refused at {} — a \
                 certificate limb exceeded ε (limb residual {margin:e} m)",
                limb.name()
            ),
            Self::CarrierDomain(refusal) => write!(f, "pcurve certification: {refusal}"),
            Self::CarrierEnclosure(refusal) => write!(
                f,
                "pcurve certification: the fitted lane's certificate refused at limb 2 — \
                 {refusal}"
            ),
            Self::FittedEscalated { .. } | Self::Escalated { .. } => {
                unreachable!("`render` writes an escalation")
            }
            Self::IntervalNotForward => write!(
                f,
                "pcurve certification: the carrier-parameter interval is not forward — a \
                 pcurve shares the edge's he_plus-forward parameter (D1)"
            ),
            Self::ChartWindingUnsupported => write!(
                f,
                "pcurve certification: the image winds more than once per turn of its \
                 carrier — this lane certifies closed-form chart images only"
            ),
            Self::ChannelKindMismatch { chart } => write!(
                f,
                "pcurve certification: the image's channels are not the {chart:?} chart's \
                 — an angle is stored where the chart's coordinate has no period, or the \
                 other way round"
            ),
            Self::WindingMismatch { stored, derived } => write!(
                f,
                "pcurve certification: the row winds {stored:?} times, and its carrier \
                 winds {derived:?} times on this chart"
            ),
            Self::PlaceholderChart => write!(
                f,
                "pcurve certification: {}, so nothing can be imaged on its chart",
                geom::PLACEHOLDER_SURFACE
            ),
            Self::SectorRefused { piece, channel } => write!(
                f,
                "pcurve certification: piece {piece} of the projected image is not in one \
                 atan2 sector — {}. Recourse: a carrier through a pole, the cone's apex or \
                 the torus's core has no one-branch image there; split the edge at it, or \
                 re-aim the chart's axis away from it",
                channel.describe()
            ),
            Self::PlacementTurnsExceeded => write!(
                f,
                "pcurve certification: the edge's parameter span sits more than 2³¹ turns \
                 from its chart's window, and its image names no copy the kernel stores — \
                 restate the carrier's parameter near zero"
            ),
            Self::AzimuthPeriodExceeded => write!(
                f,
                "pcurve certification: the pcurve winds more than one full period around \
                 the chart — split the edge first (the winding gate, chart side)"
            ),
            Self::TubePeriodExceeded => write!(
                f,
                "pcurve certification: the projected image's tube angle sweeps more than \
                 one full period of the torus — split the edge first (the winding gate, tube \
                 side)"
            ),
            Self::ResidualExceeded { check, sample } => {
                let claim = if check.bounds_a_miss() {
                    "the certificate cannot show the pcurve represents the carrier through the map"
                } else {
                    "the pcurve does not represent the carrier through the map"
                };
                write!(
                    f,
                    "pcurve certification: {check:?} at sample {sample} definitely exceeds the \
                     tolerance band — {claim} (D4 ¶2; the residual is a 3-D displacement in \
                     metres)"
                )
            }
            Self::Band(e) => write!(f, "pcurve certification: {e}"),
        }
    }
}

/// Read at a build: the mint or certification a body's own operation
/// runs.
impl core::fmt::Display for PcurveCertifyError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.render(Reading::Build))
    }
}

impl std::error::Error for PcurveCertifyError {}

impl PcurveCertifyError {
    /// The check this refusal restates, or `None` for a refusal no check
    /// of this door owns ([`Self::decided`]'s first half).
    #[must_use]
    pub fn refused_check(&self) -> Option<PcurveCheck> {
        self.decided().map(|(check, _)| check)
    }

    /// The check this refusal restates and the arm it refused on, or
    /// `None` for a refusal no check of this door owns: an unsupported
    /// class, a missing operand, a band the tolerance cannot form, a
    /// fitted certificate's structural refusal, or an escalation at the
    /// SSI door (which ends by that door's decision).
    fn decided(&self) -> Option<(PcurveCheck, RefusedArm<'_>)> {
        Some(match self {
            Self::Escalated { check, cause, .. } => (*check, RefusedArm::Undecided(cause)),
            Self::ResidualExceeded { check, .. } => (*check, RefusedArm::SignCertain(None)),
            Self::FittedLimb { limb, margin } => (
                PcurveCheck::of_fitted_limb(*limb)?,
                RefusedArm::SignCertain(Some(*margin)),
            ),
            Self::IntervalNotForward => (PcurveCheck::ParamSpan, RefusedArm::SignCertain(None)),
            Self::AzimuthPeriodExceeded => {
                (PcurveCheck::AzimuthPeriod, RefusedArm::SignCertain(None))
            }
            Self::PlacementTurnsExceeded => {
                (PcurveCheck::ChartWinding, RefusedArm::SignCertain(None))
            }
            Self::TubePeriodExceeded => (PcurveCheck::TubePeriod, RefusedArm::SignCertain(None)),
            // The check and the arm its read carries; its ending is
            // the read's ([`Self::ending`], [`IncidenceRead::ending`]).
            Self::CarrierOffChart { margin, .. } => (
                PcurveCheck::ChartIncidence,
                IncidenceRead::of_margin(*margin).arm(),
            ),
            Self::NoGeneralLane {
                gap: LaneGap::Unwritten,
                ..
            } => (PcurveCheck::ChartClass, RefusedArm::SignCertain(None)),
            Self::NoGeneralLane {
                gap: LaneGap::Pose { .. },
                ..
            } => (PcurveCheck::GeneralLane, RefusedArm::SignCertain(None)),
            Self::FittedEscalated { .. }
            | Self::UnsupportedChart { .. }
            | Self::ImageMismatch { .. }
            | Self::FittedLaneUnsupported { .. }
            | Self::SectorRefused { .. }
            | Self::IsoUnsupported { .. }
            | Self::ChartRow { .. }
            | Self::FittedCertificate { .. }
            | Self::CarrierDomain(_)
            | Self::CarrierEnclosure(_)
            | Self::ChartWindingUnsupported
            | Self::ChannelKindMismatch { .. }
            | Self::WindingMismatch { .. }
            | Self::PlaceholderChart
            | Self::Band(_) => return None,
        })
    }

    /// The ending this refusal's decision gives it, read at `at`
    /// ([`PcurveCheck::recourse`], or the SSI decision an escalation
    /// there names), or `None` for a refusal that is no decision's
    /// refused arm (an unsupported class, a missing operand, a band the
    /// tolerance cannot form, a fitted certificate's structural refusal).
    #[must_use]
    pub fn ending(&self, at: impl Into<ReadAt>) -> Option<String> {
        let at = at.into();
        if let Self::FittedEscalated { decision, cause } = self {
            return Some(decision.ending(cause, at));
        }
        if let Some(read) = IncidenceRead::of(self) {
            return Some(read.ending(at));
        }
        self.decided().map(|(check, arm)| check.recourse(arm, at))
    }

    /// This refusal's text read at `at`: an incidence refusal
    /// ([`IncidenceRead::of`]) ends through the one map
    /// ([`IncidenceRead::ending`]); an escalation ends as its decision
    /// does there; every other decided refusal is its payload alone, its
    /// ending left to the reader ([`Self::ending`]).
    #[must_use]
    pub fn render(&self, at: impl Into<ReadAt>) -> String {
        let at = at.into();
        if let Some(read) = IncidenceRead::of(self) {
            return self.with_ending(&read.ending(at));
        }
        match self {
            Self::FittedEscalated { decision, cause } => format!(
                "pcurve certification: the fitted lane's certificate escalated — {}",
                cause.undecided(decision, decision.ending(cause, at))
            ),
            Self::Escalated { check, cause, .. } => {
                self.with_ending(&check.recourse(RefusedArm::Undecided(cause), at))
            }
            _ => core::fmt::from_fn(|f| self.write_payload(f)).to_string(),
        }
    }

    /// **This refusal's text ended by its own ending at `at`**
    /// ([`Self::ending`]), the decided refusal's included, which
    /// [`Self::render`] leaves to its reader; a payload that carries its
    /// own recourse (a hand-built image, a scalar without the fitted
    /// door, a piece outside one sector) is that payload, ended once.
    /// `None` for a refusal no decision owns and whose payload carries no
    /// recourse, which a door holding it ends itself.
    #[must_use]
    pub fn render_ended(&self, at: impl Into<ReadAt>) -> Option<String> {
        let at = at.into();
        if matches!(
            self,
            Self::ImageMismatch { .. }
                | Self::FittedLaneUnsupported { .. }
                | Self::SectorRefused { .. }
        ) {
            return Some(self.render(at));
        }
        let ending = self.ending(at)?;
        Some(match self {
            Self::FittedEscalated { .. } => self.render(at),
            _ => self.with_ending(&ending),
        })
    }

    /// The refusal's text followed by `ending`: the one spelling of an
    /// escalation's text and of a payload with its ending.
    fn with_ending(&self, ending: &str) -> String {
        match self {
            Self::Escalated {
                check,
                sample,
                cause,
            } => format!(
                "pcurve certification: {check:?} at sample {sample} escalated: {}. {ending}",
                cause.payload()
            ),
            _ => format!(
                "{}. {ending}",
                core::fmt::from_fn(|f| self.write_payload(f))
            ),
        }
    }

    /// Whether this is a definite refusal on a certified upper bound on
    /// the miss ([`PcurveCheck::bounds_a_miss`]): it says only that the
    /// certificate cannot show the image represents the carrier.
    #[must_use]
    pub fn refuses_on_a_bound(&self) -> bool {
        self.decided().is_some_and(|(check, arm)| {
            matches!(arm, RefusedArm::SignCertain(_)) && check.bounds_a_miss()
        })
    }
}

/// How a refusal of a [`PcurveCheck`] ends: as a span decision read on
/// the image where the check restates one ([`SpanDecision`]), and
/// otherwise as a decision with no size.
#[derive(Clone, Copy)]
enum Routing {
    Sized(SizedDecision),
    Unsized(Unsized),
    /// Lever-only "not yet", never a defect or a tolerance: a lane choice
    /// with no general lane written ([`PcurveCheck::ChartClass`]), and a
    /// written general lane that cannot image this pose yet
    /// ([`PcurveCheck::GeneralLane`]).
    NotYet,
    /// A carrier's incidence on its chart ([`PcurveCheck::ChartIncidence`]):
    /// ended by its read ([`IncidenceRead::ending`]). A refusal that
    /// carries its read ends by it ([`PcurveCertifyError::ending`]); the
    /// check alone reads it back off the arm ([`IncidenceRead::arm_ending`]).
    Incidence,
}

impl PcurveCheck {
    fn routing(self) -> Routing {
        match self {
            // The image's span is the edge's image-span check.
            Self::ParamSpan => match crate::CertCheck::ImageSpan.span() {
                Some((decision, side)) => Routing::Sized(decision.on(side)),
                None => unreachable!("ImageSpan is a span check"),
            },
            // The winding on a spline image, which can sweep past a period
            // no edge check counts (the projected lane's gate): a stored
            // contradiction.
            Self::AzimuthPeriod | Self::TubePeriod => {
                Routing::Sized(SpanDecision::Winding.on(StoredContradiction))
            }
            // A lane choice whose general lane is not written yet (D3), and
            // a written general lane's capability gap on this pose:
            // lever-only "not yet", never a defect, never a tolerance.
            Self::ChartClass | Self::GeneralLane => Routing::NotYet,
            // The placement integer and the stored copies the kernel
            // writes exactly.
            Self::ChartWinding | Self::ImageIdentity | Self::Sector => {
                Routing::Unsized(Unsized::Defect)
            }
            Self::ChartIncidence => Routing::Incidence,
            // A sample's residual is the miss at that sample, on a
            // fitted image as well as an exact one.
            Self::MapResidual => Routing::Unsized(Unsized::Fit),
            // The envelope and each of its terms bound a part of the
            // miss from above, and the parts share `t`, so they can
            // partly cancel: a loose bound contradicts nothing stored.
            Self::Envelope | Self::EnvelopeTerm(_) => {
                #[cfg(test)]
                if mutants::envelope_fit() {
                    return Routing::Unsized(Unsized::Fit);
                }
                Routing::Unsized(Unsized::Bound)
            }
        }
    }

    /// The one ending a refusal of this check carries on `arm`, read at
    /// `at` (D4 ¶1 (i)). An incidence refusal ends by its read instead
    /// ([`PcurveCertifyError::ending`], [`IncidenceRead::ending`]).
    #[must_use]
    pub(crate) fn recourse(self, arm: RefusedArm<'_>, at: impl Into<ReadAt>) -> String {
        let at = at.into();
        match self.routing() {
            Routing::Sized(decision) => decision.recourse(arm, at),
            Routing::Unsized(decision) => decision.recourse(arm, at),
            Routing::NotYet => crate::recourse::not_yet(arm, at),
            Routing::Incidence => IncidenceRead::arm_ending(arm, at),
        }
    }

    /// Whether this check refuses on a certified upper bound on a miss
    /// rather than on the miss: its refusal says only that the
    /// certificate cannot show the image represents the carrier.
    #[must_use]
    pub fn bounds_a_miss(self) -> bool {
        matches!(self.routing(), Routing::Unsized(Unsized::Bound))
    }

    /// What a decided SSI limb measures at this door, as the check it
    /// restates. The door decides it, not the limb: here the image `P`
    /// is the STORED one, so the hull's value at a break,
    /// `|S(P(t)) − C(t)|`, is the stored image's own miss there, where
    /// at the SSI door `P` is a witness and the same value over-reads
    /// the carrier's distance from the surface ([`SsiLimb::check`]).
    /// The hull's sup is the envelope over the window. A limb this door
    /// never decides is `None`: among them [`SsiLimb::ImageMiss`], the
    /// SSI door's tell, which this door does not run, since its image
    /// is the one under judgement and a miss at a break is that image's
    /// whichever side the carrier lies.
    fn of_fitted_limb(limb: SsiLimb) -> Option<Self> {
        match limb {
            SsiLimb::HullSupChart => Some(Self::Envelope),
            SsiLimb::HullValueChart => Some(Self::MapResidual),
            SsiLimb::OnLocus
            | SsiLimb::HullSup
            | SsiLimb::HullValue
            | SsiLimb::ImageMiss
            | SsiLimb::Tube => None,
        }
    }

    /// The plane × NURBS incidence limb this check of an exact image's
    /// certificate restates, where `OnPair`'s lane reads its wall side
    /// through that image ([`crate::plane_nurbs_incidence`]):
    /// [`Self::of_fitted_limb`] read back. The map residual is
    /// `|S(P(t)) − C(t)|` at a point, [`SsiLimb::HullValueChart`]'s
    /// quantity, and the envelope (or one of its terms) bounds it over
    /// the span, [`SsiLimb::HullSupChart`]'s. `None` for a check that
    /// restates no limb.
    #[must_use]
    pub fn incidence_limb(self) -> Option<SsiLimb> {
        match self {
            Self::MapResidual => Some(SsiLimb::HullValueChart),
            Self::Envelope | Self::EnvelopeTerm(_) => Some(SsiLimb::HullSupChart),
            Self::ParamSpan
            | Self::ChartWinding
            | Self::ChartClass
            | Self::GeneralLane
            | Self::ImageIdentity
            | Self::AzimuthPeriod
            | Self::TubePeriod
            | Self::ChartIncidence
            | Self::Sector => None,
        }
    }
}

/// **Which sup-norm a certificate's envelope bounds.** The pcurve lanes
/// discharge C2.2 by different mechanisms over different quantities,
/// and a certificate that did not say which would be a number without a
/// statement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnvelopeStatement {
    /// `sup |S(P(t)) − C(t)|`, by the closed-form harmonic algebra —
    /// the [`Pcurve::Harmonic`] lane, or `span{1, f, sin}` for the
    /// spiric CAP ([`SpiricImage::Cap`]), where the difference is a
    /// constant-coefficient combination of the same three functions
    /// on both sides and its sup is `|k₀| + |k₁|·f_max + |k₂|`, or the
    /// [`Pcurve::FocalSection`] lane, whose mapped image is a harmonic
    /// plus a unit vector times a bounded scalar
    /// (`focal_section_envelope`).
    /// Nothing sampled, nothing hulled.
    ///
    /// On a PLANE chart the map is affine, so `S ∘ P` has the
    /// coefficients the image maps to, and the bound is the norms of
    /// their differences from the carrier's: `‖Δc‖ + σ(Δa, Δb) +
    /// ‖Δl‖·T`, with `T = max(|t₀|, |t₁|)` and
    /// `σ(x, y) = sup_t ‖x·cos t + y·sin t‖ = √(m + √(q² + (x·y)²))`,
    /// `m = (‖x‖² + ‖y‖²)/2`, `q = (‖x‖² − ‖y‖²)/2` — `√(x² + y²)` for
    /// scalars.
    ///
    /// # On a periodic chart: incidence plus fidelity
    ///
    /// There the image's azimuth is an angle, and pushing it back
    /// through `cos`/`sin` routes a fact about the carrier through a
    /// trigonometric identity no arithmetic sees through. So check 4
    /// re-derives the image `P_d` from the carrier inside `certify`
    /// (the derivation [`chart_pcurve`] runs, with the structure it
    /// selects: the class, `β`, `σ`, the cone's nappe) and splits
    ///
    /// ```text
    /// |S(P(t)) − C(t)| ≤ |S(P(t)) − S(P_d(t))| + |S(P_d(t)) − C(t)|
    ///                         fidelity                incidence
    /// ```
    ///
    /// (on the chart's orthonormal twin, plus **frame** for the stored
    /// chart's distance from it — the frame paragraph below).
    ///
    /// **Fidelity** meters the stored image against `P_d` through the
    /// chart's sup stretch along the chart segment between them, each
    /// channel's sup bounded by its coefficients as above, an angular
    /// channel's constant read as it is, with no whole period taken off
    /// (a row is its placed image, C4; and a sphere's image against
    /// `P_d`'s involution twin `(u + π, π − v)` where its structure names
    /// the twin: on a meridian its windings, on a parallel the sign of
    /// its azimuth offset's cosine). On the cylinder, the sphere and the
    /// torus `S_u ⊥ S_v` everywhere, so the segment's image is no longer
    /// than `√((arm_u·sup|Δu|)² + (arm_v·sup|Δv|)²)`. The cone's
    /// azimuth arm is read along a chord at the nearer slant, which is
    /// not orthogonal to the ruling it is joined to, so there the two
    /// channels add. It is zero, node for node, on the image the mint
    /// stores.
    ///
    /// **Incidence** bounds `S(P_d(t)) − C(t)` in the carrier's own
    /// coefficients (`C = c + a·cos t + b·sin t + l·t`) and the chart's
    /// data, and no image angle enters it: wherever the derivation took
    /// an azimuth `α = atan2(q·v_ref, q·u_ref)` of a radial vector `q`,
    /// `rad(α)` is `q/‖q‖` exactly, and every polar angle's cosine and
    /// sine are the quotients it was the `atan2` or `asin` of. Each
    /// distance from a radius is spelled `|p − ‖q‖| = |p² − q·q|/(p +
    /// ‖q‖)`, and each radial of length `p` along `q` is `q·(1 + k)`
    /// with `k = (p² − q·q)/((p + ‖q‖)·‖q‖)`, a polynomial numerator
    /// both. With `e` the axis, `x_r = x − e·(x·e)`, `w = c − (the
    /// chart's origin, apex or centre)`, `h = w·e`, and `T` as above,
    /// the lemma per arm — `S(P_d(t)) − C(t)` written out, then bounded
    /// by one triangle inequality per coefficient, a `cos t`/`sin t`
    /// pair read together by its `σ`. A turning class's pair is the
    /// image's coefficients less the carrier's, read whole, so its
    /// radius, its winding and its plane are one `σ`:
    ///
    /// - **Cylinder, moving** (`β = ±1`; the image's radial is
    ///   `R·(â_r·cos t + β·e×â_r·sin t)` and `R·â_r = a_r·(1 + k)`):
    ///   `−w_r + k·a_r·cos t + (β·e×a_r·(1 + k) − b_r)·sin t − l_r·t` ⇒
    ///   **centre** `‖w_r‖` + **radius** `σ(k·a_r, β·e×a_r·(1 + k) −
    ///   b_r)` + **line** `‖l_r‖·T`. The axial channel is the carrier's
    ///   own and cancels.
    /// - **Cylinder, meridian** (radial motion decided Zero — or
    ///   undecided, the D9 tie-break): `(R·ŵ_r − w_r) − a_r·cos t −
    ///   b_r·sin t − l_r·t` ⇒ **radius** `|R − ‖w_r‖|` + **drift**
    ///   `σ(a_r, b_r)` + **line** `‖l_r‖·T`.
    /// - **Cone, rim** (slant `v = h·cos α + n·‖a_r‖·sin α`, the rim's
    ///   projection on its generators, `n` the nappe; `rad(u)·v` flips
    ///   with the nappe, so the image's radial is `ρ·â_r` on either,
    ///   `ρ = n·v·sin α`, at height `v·cos α`): `e·λ − w_r + (a_r·(1 +
    ///   k) − a)·cos t + (β·e×a_r·(1 + k) − b)·sin t − l·t`, the lift `λ
    ///   = v·cos α − h = sin α·(n·‖a_r‖·cos α − h·sin α)` ⇒ **centre**
    ///   `‖w_r‖` + **radius** the lift and the pair at
    ///   [`constant_and_pair`]'s sup — `√(λ² + σ²)` for a carrier in a
    ///   plane normal to the axis, where the pair is radial — + **line**
    ///   `‖l‖·T`.
    /// - **Cone, ruling** (a line; `q` the radial of whichever datum
    ///   the nappe was read off — the anchor `w` or the direction `l` —
    ///   signed by it, and `ĝ = cos α·e + sin α·q̂` the generator at
    ///   `rad(α_d) = q̂`): the image is `apex + ((w + l·t)·ĝ)·ĝ`, so
    ///   `−(w − (w·ĝ)·ĝ) − (l − (l·ĝ)·ĝ)·t` ⇒ **radius** `‖w − (w·ĝ)·ĝ‖`,
    ///   the anchor off the generator, + **line** `‖l − (l·ĝ)·ĝ‖·T`. A
    ///   datum `h·e + p·q̂ + m·e×q̂` is `√((p·cos α − h·sin α)² + m²)`
    ///   off it; the reference datum's is `||h|·sin α − ‖q‖·cos α|`.
    /// - **Sphere, parallel** (`ρ = √(r² − h²)` is `r·cos asin(h/r)`):
    ///   the cone rim's form with `ρ`, `k` spelled over `r² − h² −
    ///   a_r·a_r` ⇒ **centre** + **radius** + **line**.
    /// - **Sphere, meridian** (`δ = atan2(a·e, ‖a_r‖)` and `d̂` the
    ///   azimuth direction read, so `r·(cos δ·d̂ + sin δ·e) = a′·(1 + k)`
    ///   and `r·(cos δ·e − sin δ·d̂) = b′·(1 + k)` with `a′ = ‖a_r‖·d̂ +
    ///   (a·e)·e`, `b′ = ‖a_r‖·e − (a·e)·d̂`, `‖a′‖ = ‖b′‖ = ‖a‖`; `d̂ =
    ///   â_r` gives `a′ = a` and `b′ = Ja`, the quarter turn of `a` in its
    ///   meridian plane, and at a pole start (`a_r` decided Zero) `d̂ =
    ///   b̂_r`): `−w + (a′·(1 + k) − a)·cos t + (σ·b′·(1 + k) − b)·sin t
    ///   − l·t` ⇒ **centre** `‖w‖` + **radius** `σ(a′·(1 + k) − a,
    ///   σ·b′·(1 + k) − b)` + **line** — the whole residual, the pole
    ///   start's too.
    /// - **Torus, parallel** (`m = √(h² + (‖a_r‖ − R)²)`, the distance to
    ///   the tube's core; the image's radius and height are `R +
    ///   r·(‖a_r‖ − R)/m = ‖a_r‖ + (‖a_r‖ − R)·κ` and `h·(1 + κ)`, `κ =
    ///   r/m − 1 = (r² − m²)/((r + m)·m)`; `f = 1 + (‖a_r‖ − R)·κ/‖a_r‖`):
    ///   `C + A·cos t + B·sin t − l·t` with the constant `C = e·h·κ − w_r`
    ///   and `(A, B) = (a_r·f − a, β·e×a_r·f − b)` ⇒ **radius** `√(‖C‖² +
    ///   σ(A, B)² + 2·σ(C·A, C·B))` + **line**: `‖C + H(t)‖² = ‖C‖² +
    ///   ‖H(t)‖² + 2·C·H(t)` and `C·H(t)` is the scalar pair `(C·A, C·B)`,
    ///   so the bound holds at any tilt of the carrier's plane, and it is
    ///   `√(‖C‖² + σ²)` where `C ⊥ A, B` (an untilted parallel: `C` axial
    ///   up to the centre's offset, the pair radial).
    /// - **Torus, meridian** (`α` the azimuth of `w_r`; `x = a·ŵ_r`,
    ///   `n = √(x² + (a·e)²)`, `a* = x·ŵ_r + (a·e)·e`, `b* = σ·(x·e −
    ///   (a·e)·ŵ_r)`, `a* ⊥ b*`, both of norm `n`, so the image turns
    ///   `f·a*` and `f·b*` with `f = r/n`): `(R·ŵ_r − w) + (f·a* −
    ///   a)·cos t + (f·b* − b)·sin t − l·t` ⇒ **centre** `σ(R − ‖w_r‖,
    ///   h)`, the constant's radial and axial parts, + **radius** `σ(f·a*
    ///   − a, f·b* − b)` + **line**.
    ///
    /// A winding decided Zero on a moving class (a carrier whose radial
    /// part does not turn — no carrier of the chart) takes the image's
    /// constant radial `ρ·rad(α)` at its full length: **radius** `ρ`
    /// (the torus's `R + r` plus the tube) and **drift** `σ(a, b)`, the
    /// carrier's pair whole (its radial part alone on the cylinder,
    /// whose axial channel cancels),
    /// so even that class is bounded rather than refused
    /// unmeasured.
    ///
    /// **The frame.** Every arm above is a lemma about an orthonormal
    /// frame (`axis` and `u_ref` unit, `u_ref ⊥ axis`), as every meter
    /// in this lane is ([`chart_stretch_sup`]'s arms, `azimuth_lever`).
    /// Nothing at rest makes the stored chart's frame one: the cone
    /// carries no frame margins, and where
    /// `Surface::representability_margins` does carry them it meters
    /// them at the radius, while an axis's defect moves a point by the
    /// defect times its HEIGHT. So check 4 applies the lemma, and
    /// derives `P_d`, on the chart's Gram–Schmidt twin `Ŝ` (`n̂ =
    /// axis/‖axis‖`, `ê₁` the normalised part of `u_ref` normal to it,
    /// `ê₂ = n̂ × ê₁`), and adds **frame**: `sup |S − Ŝ|` over the
    /// stored image's chart box. The two maps share every scalar, so
    /// the radial direction differs by at most `‖u_ref − ê₁‖ + ‖v_ref −
    /// ê₂‖` and the axis by `‖axis − n̂‖`, each levered by what
    /// multiplies it in the map: the radius, the torus's `R + r` and
    /// `r`, and on a cylinder's axis and both of a cone's channels the
    /// image's own `sup |v|`. On the sphere and the torus's tube the
    /// radial and axial defects ride `cos v` and `sin v`, a unit pair,
    /// so they are read at their norm. Then `|S(P) − C| ≤ |S(P) − Ŝ(P)| + |Ŝ(P) −
    /// C|`, the second bounded by the arms above on `Ŝ`. On a literal
    /// unit frame the twin is the frame itself and the term is the zero
    /// form.
    ///
    /// The terms combine as `EnvelopeTerms::total` (every slot added,
    /// the two channel fidelities as legs where the chart is
    /// orthogonal) and are decided once as `pcurve_envelope`; a
    /// refusal names the first term that is over the band on its own
    /// ([`EnvelopeTerm`]). `envelope_lemma_fuzz` sweeps every arm: the
    /// total dominates the sampled residual of carriers and images moved
    /// off in one respect at a time, and on an exact carrier the
    /// closed-form tables compose back to the carrier.
    ///
    /// **On a harmonic row this envelope is the whole certified
    /// statement** (C4): the lemma above is proven once and pinned by
    /// that sweep, and the schedule that used to re-check it on every
    /// body runs only on the witness lane (`Witness::Inexact`), as a cross-check
    /// whose verdicts are not part of the certificate
    /// ([`PcurveCache::certify`] step 3).
    MapResidualClosedForm,
    /// `sup |S(P(t)) − C(t)|`, by the **tensor Bernstein composite**
    /// (`geom_core::spline::compose::tensor`) — the [`Pcurve::Fitted`]
    /// lane on a NURBS chart, where both sides of the comparison are
    /// splines and the difference is enclosed as ONE composite, so the
    /// cancellation survives into the bound.
    MapResidualComposite,
    /// `sup |S(P(t)) − C(t)|` for the two NURBS-chart iso rungs —
    /// [`Pcurve::IsoLine`] (M6-3) and [`Pcurve::IsoArc`] (M8-3, whose
    /// chart column is rational by construction) — by the traversed
    /// row's **control-difference hull**. The traversed iso is a curve
    /// in the chart's own spline space with an explicit control
    /// polygon — a BOUNDARY row is a copy of the control net (no
    /// arithmetic), an INTERIOR column is the de Boor collapse of the
    /// net at the fixed parameter (`crate::nurbs_iso`; exact in ℝ, and
    /// at the interval scalar each collapsed control point encloses the
    /// exact one, so the hull's upper end still bounds the sup) — so
    /// `S ∘ P` and the carrier-side comparison live in one spline space
    /// and the partition-of-unity hull `sup |Σ Nᵢ·Δcᵢ| ≤ max |Δcᵢ|`
    /// bounds their difference over the whole domain — nothing sampled.
    /// The hull needs the two curves to share a spline space and (when
    /// rational) strictly positive weights, not a non-rational chart;
    /// M8-3 moved that hypothesis from a blanket chart-level gate to
    /// the class arms that use it. The banded axis/side/domain snap
    /// slacks (the trilean-admitted ε-shell around the exact
    /// axis-aligned family, metered through the chart's derivative-net
    /// stretch bounds) are folded in explicitly, exactly as the
    /// harmonic lane's fidelity term carries its coefficients' drift; a
    /// boundary row pays its
    /// snap to the domain end, a collapsed row is taken at the stored
    /// parameter itself and pays the channel's drift alone. Every slack
    /// is exactly zero on the minted path.
    MapResidualIsoHull,
    /// **Zero, by an algebraic identity** — the [`SpiricImage::Wall`]
    /// lane, and the only statement in this enum that is not a bound
    /// on a computed difference.
    ///
    /// **The premises, and how each is established.** The wall image
    /// stores `major`, `minor`, `offset` and `sense`; check 1 compares
    /// the first three against the CARRIER's own fields and `sense`
    /// against `±1`, AND compares the CHART's centre, axis, `R` and
    /// `r` against the torus its [`WallTorus`] tag names — the
    /// carrier's own or its mirror through the cutting plane — because
    /// the identity is a statement about the map, so a chart that is
    /// not that torus is not an input this statement covers. All eight
    /// comparands are
    /// **banded**, not bit-equal: each is a metre margin whose
    /// `Sign::Zero` window is `|Δ| ≤ ε` (the C6 bit-equal read needs
    /// `T: Bounds`, and this is a `T: Decide` door). Whatever the band
    /// admits is carried into the envelope below rather than
    /// discarded, so the statement is true for every input the door
    /// accepts and not only for the ones a mint produces.
    ///
    /// Under those equalities `S(P(t)) − C(t)` is identically zero in
    /// exact arithmetic for every `t`: the chart formula
    /// `S(u, v) = c′ + e(u)·(R + r cos v) + A·(r sin v)` evaluated at
    /// `u = u₀ + σμ·atan2(f, d)`, `v = v₀ + σ·t` reproduces
    /// `c + n·d + m·f + a·(r sin t)` term for term, because
    /// `e(φ_{μn} + σμ·atan2(f, d))·ρ = μ·n·d + m·f` is the polar
    /// decomposition of that radial part about `c′ = c + (1 − μ)·d·n`
    /// and `A·sin(v₀ + σt) = a·sin t` for `v₀ ∈ τℤ` and `A = σ·a`.
    ///
    /// So the only numbers the identity does NOT fix are the two
    /// derived constants `u₀` and `v₀`, and neither can hide between
    /// samples: an error `δ` in `u₀` displaces every point by
    /// `ρ·2|sin(δ/2)| ≥ (R − r)·2|sin(δ/2)|`, and an error in `v₀`
    /// that is not a multiple of `τ` moves every point too — both are
    /// therefore visible at EVERY sample of the schedule, which is
    /// what certifies them. An error of a whole number of periods in
    /// either moves no point: that row is a lifted copy of the placed
    /// one, which the identity cannot see, and the placement terms
    /// added to this envelope refuse it (C4). The stored envelope is a statement about
    /// the algebra, not a measurement:
    /// [`PcurveCertificate::max_residual`] carries the schedule's own
    /// worst displacement as always.
    ///
    /// **What the envelope actually holds, per scalar.** At a POINT
    /// scalar it is exactly `0` on every minted image whose chart
    /// centre is spelled as check 1 spells it ([`WallTorus::center`]:
    /// `c` itself on the own torus, `c + n·(d + d)` on the mirror),
    /// because check 1's admitted drift is computed from differences
    /// that are then bit-zero. A producer that spells the mirror's
    /// centre another way (`c + 2d·n`, say) pays that spelling's
    /// rounding here, inside the band, as admitted drift. At a
    /// BRACKETED scalar it is the bracket's own width metered at the
    /// chart's arms, and not because anything moved: interval
    /// arithmetic cannot cancel a variable against itself, so
    /// `x − x` over a bracket of width `w` is `[−w, w]` rather than
    /// `[0, 0]`, and the drift term carries that through. The answer
    /// is still a true sup bound — the conservative direction — and it
    /// is the honest price of a banded structural gate at a scalar
    /// whose values are sets.
    SpiricIdentity,
    /// `sup |S(P(t)) − C(t)|` for a [`Pcurve::Projected`] image over the
    /// whole span: the carrier's distance from the chart (its implicit
    /// form composed along it, in metres through a per-span lever), plus
    /// the stored carrier's fidelity to the edge's through the
    /// projection's Lipschitz bound, plus the frame's defect and the deck
    /// map's distance from a deck transformation of the chart, each piece
    /// held in one `atan2` sector. The same quantity as
    /// [`Self::MapResidualClosedForm`]; the lemma is
    /// `pcurve_cache::projected`'s module docs.
    MapResidualProjected,
}

/// The certification record stored with a certified pcurve: the
/// schedule that ran, the worst sampled displacement, the
/// between-samples envelope and **which sup it bounds** — all metres
/// (module docs). Byte-identical across replays of the same
/// construction (D9).
#[derive(Clone, Copy, Debug)]
pub struct PcurveCertificate<T: Real> {
    /// The sample count of the schedule that ran ([`CERT_SAMPLES`]), or
    /// `0` where none ran: a [`Pcurve::Harmonic`] row certified at a
    /// scalar whose [`geom_core::Witness`] is `Exact` (`Interval`,
    /// `Sym<Interval>`), at a point or over a box alike. Its envelope
    /// is its whole certified statement; the schedule cross-checks it on
    /// the witness lane only.
    pub samples: u32,
    /// The maximum `|S(P(tᵢ)) − C(tᵢ)|` over the schedule (metres), `0`
    /// where none ran — **the sampled max only**. The between-samples statement is
    /// [`Self::envelope`], deliberately a separate field: folding them
    /// into one number would let a reader mistake a sup bound for a
    /// measurement or the reverse.
    pub max_residual: T,
    /// The certified sup bound on `|S(P(t)) − C(t)|` over the whole
    /// span (metres) — the C2.2 between-samples limb (module docs).
    /// Always ≥ the true sup, hence ≥ [`Self::max_residual`] for an
    /// exact-in-family pcurve; it additionally carries the drift the
    /// winding gate admits (see [`PcurveCache::certify`] step 4), so it
    /// is the number to quote for "how far can this cache be from its
    /// carrier".
    ///
    /// For a [`Pcurve::Fitted`] cache the *quantity* changes with the
    /// chart — read [`Self::statement`] before quoting this number.
    pub envelope: T,
    /// Which sup-norm [`Self::envelope`] bounds.
    pub statement: EnvelopeStatement,
}

impl<T: Real> PcurveCertificate<T> {
    /// The edge's limbs 1 and 2 on this chart, as an edge certificate
    /// reads them off a row image's certificate: the residual at the
    /// samples, and the envelope between them.
    pub fn limbs(&self) -> (T, T) {
        (self.max_residual, self.envelope)
    }

    /// A certificate whose envelope is its own statement's bound.
    pub(crate) fn closed(
        samples: u32,
        max_residual: T,
        envelope: T,
        statement: EnvelopeStatement,
    ) -> Self {
        Self {
            samples,
            max_residual,
            envelope,
            statement,
        }
    }
}

/// The image producer's body, shared by every certifying scalar
/// ([`crate::FittedLane::general_image`]).
///
/// The `edge_nurbs` schedule with no per-sample hook: the transversality
/// sweep the adopt path runs there is a statement about the PLANE
/// operand, which is the edge certificate's and not the row's.
pub(crate) fn general_image_lane<T: Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<T>,
    params: (T, T),
    wall: &NurbsSurface<T>,
) -> Result<NurbsCurve2<T>, PcurveCertifyError> {
    use crate::edge_nurbs::PlaneNurbsRefusal as P;
    // The image spans the carrier's certificate span, the edge
    // certificate's rule (`carrier_span`).
    let eval = |t| carrier.eval(t);
    let (domain, ends) = crate::carrier_net::carrier_span(carrier, params);
    let schedule = crate::edge_nurbs::Schedule::new(&eval, domain, ends);
    let (t0, t1) = schedule.domain();
    let certificate = |what| PcurveCertifyError::FittedCertificate {
        limb: None,
        what,
        magnitude: None,
    };
    crate::edge_nurbs::chart_image(schedule, wall, |_, _| Ok(())).map_err(|e| match e {
        P::FootPointInconclusive {
            sample,
            last_distance,
        } => PcurveCertifyError::FittedCertificate {
            limb: Some(SsiLimb::OnLocus),
            what: "a foot point of the chart-image schedule would not converge, so this \
                   locus has no derived image to certify",
            // The parameter the schedule projected at, not the sample
            // index dressed up as one.
            magnitude: Some(FittedMagnitude::LastFootDistance {
                t: crate::certify::schedule_param(
                    t0,
                    t1,
                    sample,
                    crate::edge_nurbs::PXN_FIT_SAMPLES,
                ),
                last_distance,
            }),
        },
        P::PcurveFit => certificate(crate::edge_nurbs::PCURVE_FIT_REFUSAL),
        P::CarrierDomain(refusal) => PcurveCertifyError::CarrierDomain(refusal),
        P::Unsupported { what } => certificate(what),
        other @ (P::NotTransverse { .. }
        | P::TransversalityEscalated { .. }
        | P::Limb { .. }
        | P::TubeStraddles { .. }
        | P::TubeNotOneArc { .. }
        | P::Escalated { .. }
        | P::ArcRim(_)
        | P::ReportedTransversalityPoisoned(_)
        | P::ChartSpeed(_)
        | P::CarrierEnclosure(_)) => unreachable!(
            "chart_image returns these only from its per-sample hook or the certificate, and \
             the mint passes a no-op hook and runs no certificate: {other:?}"
        ),
    })
}

/// The foot producer's body, shared by every certifying scalar
/// ([`crate::FittedLane::chart_foot`]).
pub(crate) fn chart_foot_lane<T: Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    point: Point3<T>,
    wall: &NurbsSurface<T>,
) -> Result<Point2<f64>, PcurveCertifyError> {
    match crate::edge_nurbs::chart_foot(point, wall) {
        Ok(foot) => Ok(foot),
        Err(crate::edge_nurbs::PlaneNurbsRefusal::FootPointInconclusive {
            last_distance, ..
        }) => Err(PcurveCertifyError::FittedCertificate {
            limb: Some(SsiLimb::OnLocus),
            what: "an edge endpoint has no certified foot on this chart, so where its \
                   image sits cannot be measured",
            magnitude: Some(FittedMagnitude::EndpointFootDistance { last_distance }),
        }),
        Err(_) => Err(PcurveCertifyError::PlaceholderChart),
    }
}

/// The fitted door's body for a net's projected row
/// ([`crate::FittedLane`]): the stored image's piece hulls (its sector
/// and lever data), and the chart's canonical implicit form composed
/// along `twin_net`, the carrier written in the frame of `twin`, the
/// chart's orthonormal twin, over parts cut at `cuts` besides the
/// uniform ones. Every value crosses into certification arithmetic
/// through [`geom_core::Interval::from_certified`].
///
/// # Errors
///
/// [`PcurveCertifyError::ImageMismatch`] for a circle's image (it takes
/// no door) or a net the composite refuses.
pub(crate) fn projected_hull_lane<T: Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    image: &ProjectedImage<T>,
    twin_net: &NurbsCurve3<T>,
    twin: &Surface<T>,
    cuts: &[f64],
) -> Result<projected::ProjectedHull, PcurveCertifyError> {
    use projected::{PieceHull, ProjectedHull};
    let cross = geom_core::Interval::from_certified;
    let refuse = |why| PcurveCertifyError::ImageMismatch {
        image: PcurveKind::Projected,
        why,
    };
    let FramedCarrier::Net(net) = &image.carrier else {
        return Err(refuse("a circle's projected row takes no door"));
    };
    // The stored net at the certification scalar: control points, held
    // knots and weights, each through the certified enclosure of its
    // value (a lift: every point is kept).
    let lift = |n: &NurbsCurve3<T>| n.map_scalar(cross);
    let chart = match image.chart {
        ProjectedChart::Plane => ProjectedChart::Plane,
        ProjectedChart::Cylinder => ProjectedChart::Cylinder,
        ProjectedChart::Sphere => ProjectedChart::Sphere,
        ProjectedChart::Cone { sin, cos, nappe } => ProjectedChart::Cone {
            sin: cross(sin),
            cos: cross(cos),
            nappe,
        },
        ProjectedChart::Torus { major } => ProjectedChart::Torus {
            major: cross(major),
        },
    };
    let lifted_net = Arc::new(lift(net));
    let lifted = ProjectedImage {
        chart,
        carrier: FramedCarrier::Net(Arc::clone(&lifted_net)),
        breaks: image.breaks.clone(),
        azimuth: image.azimuth.clone(),
        tube: image.tube.clone(),
        u_off: cross(image.u_off),
        v_off: cross(image.v_off),
        v_sign: cross(image.v_sign),
    };
    let (lower, uncertified) = (certified_lower, uncertified_over);
    let knots = image.breaks.knots();
    let pieces = (0..image.pieces())
        .map(|k| {
            let (a, b) = (knots[k + 1], knots[k + 2]);
            let fb = lifted.frame_box(
                k,
                geom_core::Interval::from_bounds(a, a),
                geom_core::Interval::from_bounds(b, b),
            );
            let ok = !uncertified(&lifted_net, (a, b));
            let tube_lo = match lifted.chart {
                ProjectedChart::Torus { major } => lower(lifted.tube_box(k, &fb, major).0.0, ok),
                _ => f64::INFINITY,
            };
            PieceHull {
                x_lo: lower(fb.x.0, ok),
                tube_lo,
            }
        })
        .collect();
    let spans = twin_span_hulls(
        &lift(twin_net),
        twin,
        &image.chart,
        projected::INCIDENCE_CUTS,
        cuts,
    )?;
    Ok(ProjectedHull { pieces, spans })
}

/// An endpoint of a bracket that does not certify is an ordinary
/// number, not a bound: every read here asks first, and reads NaN for
/// one that does not. A piece or span the edge's interval meets refuses
/// on it (`projected::net_incidence`, the sector check); one outside the
/// interval is not the edge's.
fn certified_lower(v: geom_core::Interval, ok: bool) -> f64 {
    use geom_core::Bounds;
    if ok && v.is_certified() {
        v.lo()
    } else {
        f64::NAN
    }
}

/// [`certified_lower`]'s upper end.
fn certified_upper(v: geom_core::Interval, ok: bool) -> f64 {
    use geom_core::Bounds;
    if ok && v.is_certified() {
        v.hi()
    } else {
        f64::NAN
    }
}

/// Whether a control of `n` whose support meets `(a, b)` does not
/// certify. A net with such a control is corrupt, and it refuses
/// wherever that control's support reaches: a piece or part reads NaN
/// for every bound, whichever channel the control's refusal sits in —
/// not only where a box the bound is read off carries it.
fn uncertified_over(n: &NurbsCurve3<geom_core::Interval>, (a, b): (f64, f64)) -> bool {
    let (knots, p) = (
        n.knots_f64_interim().knots(),
        n.knots_f64_interim().degree(),
    );
    n.control().iter().enumerate().any(|(i, c)| {
        knots[i] < b
            && knots[i + p + 1] > a
            && !(c.x.is_certified() && c.y.is_certified() && c.z.is_certified())
    })
}

/// The parts of a carrier's twin net in certification arithmetic: the
/// chart's canonical implicit form composed along `twin_net` (the
/// carrier written in the frame of `twin`, the chart's orthonormal
/// twin), each knot span cut into `per_span` equal parts and at `cuts`,
/// with each part's bound of `|f|`, the floors its metre conversion
/// reads, and its metre readings.
///
/// # Errors
///
/// [`PcurveCertifyError::ImageMismatch`] for a net the composite
/// refuses; [`PcurveCertifyError::UnsupportedChart`] off an analytic
/// chart.
fn twin_span_hulls<T: geom_core::CertifiedBounds>(
    twin_net: &NurbsCurve3<geom_core::Interval>,
    twin: &Surface<T>,
    chart: &ProjectedChart<T>,
    per_span: usize,
    cuts: &[f64],
) -> Result<Vec<projected::SpanHull>, PcurveCertifyError> {
    use geom_core::spline::compose::CanonicalSurface;
    let cross = geom_core::Interval::from_certified;
    let form = match *twin {
        Surface::Plane { .. } => CanonicalSurface::Plane,
        Surface::Cylinder { radius, .. } => CanonicalSurface::Cylinder {
            radius: cross(radius),
        },
        Surface::Sphere { radius, .. } => CanonicalSurface::Sphere {
            radius: cross(radius),
        },
        Surface::Cone { half_angle, .. } => {
            let (s, c) = half_angle.sin_cos();
            let (s, c) = (cross(s), cross(c));
            CanonicalSurface::Cone {
                cos2: c.powi(2),
                sin2: s.powi(2),
            }
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => CanonicalSurface::Torus {
            major: cross(major_radius),
            minor: cross(minor_radius),
        },
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(PcurveCertifyError::UnsupportedChart { chart: twin.kind() });
        }
    };
    let Some(meters) = projected::Meters::of(twin, chart, cross) else {
        unreachable!("the form above is analytic")
    };
    span_memo::read(twin_net, &form, meters, per_span, cuts, || {
        spans_of(twin_net, &form, meters, per_span, cuts)
    })
}

/// [`twin_span_hulls`]' parts, from the twin's canonical form and metre
/// conversion in certification arithmetic.
fn spans_of(
    twin_net: &NurbsCurve3<geom_core::Interval>,
    form: &geom_core::spline::compose::CanonicalSurface,
    meters: projected::Meters<geom_core::Interval>,
    per_span: usize,
    cuts: &[f64],
) -> Result<Vec<projected::SpanHull>, PcurveCertifyError> {
    use geom_core::spline::compose::{CurveCertData, canonical_composite};
    use projected::SpanHull;
    let refuse = |why| PcurveCertifyError::ImageMismatch {
        image: PcurveKind::Projected,
        why,
    };
    let (lower, upper, uncertified) = (certified_lower, certified_upper, uncertified_over);
    // The composite of the net, and the same composite with each knot
    // span cut into equal parts, and at `cuts`: the Bernstein hull over a
    // part is tighter than over the whole span (its excess falls as the
    // part's square). The parts are cut in the composite's own ring
    // (`CurveCertData::with_breaks`), not by refining the net through an
    // `f64` plan, which would re-round its controls and weights into a
    // neighbouring curve's. Each part reads the smaller of its own bound
    // and its span's, both bounds on `|f|` over the part; its floors are
    // read off its own controls (`projected::piece_controls`, whose
    // ratios are formed in the scalar).
    let all_certified = twin_net
        .control()
        .iter()
        .all(|c| c.x.is_certified() && c.y.is_certified() && c.z.is_certified());
    let coords = twin_net.certified_coords();
    let data = CurveCertData::new(
        twin_net.knots_f64_interim(),
        twin_net.weights_f64_interim(),
        &coords,
    )
    .map_err(|_| refuse("the re-derived net's certification data is malformed"))?;
    let composite_of = |data: &CurveCertData<'_>| {
        canonical_composite(data, form)
            .map_err(|_| refuse("the canonical composite refused the re-derived net"))
    };
    let whole = composite_of(&data)?;
    let (whole_breaks, whole_sup) = (whole.num.breaks().to_vec(), whole.span_sup_bounds());
    let runs: Vec<f64> = twin_net
        .knots_f64_interim()
        .knot_runs()
        .map(|(k, _)| k)
        .collect();
    let mut all_cuts: Vec<f64> = runs
        .windows(2)
        .flat_map(|w| {
            (1..per_span).map(move |j| {
                #[allow(clippy::cast_precision_loss)]
                let f = j as f64 / per_span as f64;
                w[0] + (w[1] - w[0]) * f
            })
        })
        .chain(cuts.iter().copied())
        .collect();
    all_cuts.sort_by(f64::total_cmp);
    all_cuts.dedup();
    let cut = if all_cuts.is_empty() {
        None
    } else {
        Some(composite_of(&data.clone().with_breaks(&all_cuts))?)
    };
    let parts = cut.as_ref().unwrap_or(&whole);
    let (breaks, part_sup, values) = (
        parts.num.breaks().to_vec(),
        parts.span_sup_bounds(),
        parts.break_values(),
    );
    // A bound the composite could not certify is NaN, which `f64::min`
    // passes over while the other is certified, and which its consumer
    // refuses (`projected::net_incidence`) when neither is.
    let spans = part_sup
        .into_iter()
        .enumerate()
        .map(|(j, part)| {
            let range = (breaks[j], breaks[j + 1]);
            let whole = whole_breaks
                .windows(2)
                .position(|w| w[0] <= range.0 && range.1 <= w[1])
                .map_or(f64::NAN, |i| whole_sup[i]);
            // The breaks are a knot vector's, so a part's ends are
            // ordered numbers; a pair that is not names no part, and its
            // hull is the uncertified one.
            let Some(piece) = geom_core::spline::ParamRange::new(range.0, range.1) else {
                return SpanHull {
                    range,
                    f_sup: f64::NAN,
                    rho_lo: f64::NAN,
                    z: (f64::NAN, f64::NAN),
                    meters: f64::NAN,
                    floors: (0.0, 0.0),
                };
            };
            let controls = projected::piece_controls(twin_net, piece);
            let (boxed, support, z) = projected::part_floors(&controls);
            // The support's divisor may not certify (a chord through the
            // axis): the box's floor stands alone then.
            let rho = if support.is_certified() {
                boxed.max(support)
            } else {
                boxed
            };
            let ok = all_certified || !uncertified(twin_net, range);
            let f_sup = if ok { part.min(whole) } else { f64::NAN };
            let (rho_lo, z) = (lower(rho, ok), (lower(z.0, ok), upper(z.1, ok)));
            let point = |x: f64| geom_core::Interval::from_bounds(x, x);
            let meters_sup =
                if f_sup.is_finite() && rho_lo.is_finite() && z.0.is_finite() && z.1.is_finite() {
                    meters
                        .over(point(f_sup), point(rho_lo), z, |lever| {
                            if lower(lever, true) > 0.0 {
                                Ok(lever)
                            } else {
                                Err(())
                            }
                        })
                        .map_or_else(
                            |()| {
                                meters
                                    .apex_root(point(f_sup))
                                    .map_or(f64::INFINITY, |m| upper(m, true))
                            },
                            |m| upper(m, true),
                        )
                } else {
                    f64::NAN
                };
            // A Bézier part interpolates its end controls, so they are
            // its points at its ends.
            let floors = match (ok, controls.first(), controls.last()) {
                (true, Some(&(first, _)), Some(&(last, _))) => (
                    projected::distance_floor(meters, values[j], first),
                    projected::distance_floor(meters, values[j + 1], last),
                ),
                _ => (0.0, 0.0),
            };
            SpanHull {
                range,
                f_sup,
                rho_lo,
                z,
                meters: meters_sup,
                floors,
            }
        })
        .collect();
    Ok(spans)
}

/// **The span hulls' memo**: [`spans_of`] keyed by exactly what it
/// reads, for the projected rows' door and limb 2's rounds alike.
///
/// [`spans_of`] is certification arithmetic over the twin net, the
/// twin's canonical form and its metre conversion, all in `Interval`,
/// and the cuts. It takes no decision, so a hit skips no verdict, no
/// escalation and no decision count; the twin's own scalars are crossed
/// into the form and the conversion on every call, before the memo is
/// read ([`twin_span_hulls`]). Each `Interval` enters the key as its
/// certified bracket's two ends (NaN where it does not certify, which is
/// all the parts read of it: a part an uncertified control reaches
/// reads NaN), each `f64` as its bits, and the key is compared whole: no
/// hash stands for it, so a hit returns exactly what a miss computes
/// (D9). It is a
/// bounded thread-local memo ([`crate::memo`]), so an entry left from
/// another operation is still that key's value, never stale.
mod span_memo {
    use super::{PcurveCertifyError, projected};
    use geom::NurbsCurve3;
    use geom_core::Interval;
    use geom_core::spline::compose::CanonicalSurface;

    type Spans = Result<Vec<projected::SpanHull>, PcurveCertifyError>;

    std::thread_local! {
        static MEMO: core::cell::RefCell<crate::memo::Memo<String, Spans>> =
            core::cell::RefCell::new(crate::memo::Memo::new());
    }

    /// The parts at this input: the memo's, or `compute`'s, which is
    /// then kept.
    pub(super) fn read(
        twin_net: &NurbsCurve3<Interval>,
        form: &CanonicalSurface,
        meters: projected::Meters<Interval>,
        per_span: usize,
        cuts: &[f64],
        compute: impl FnOnce() -> Spans,
    ) -> Spans {
        crate::memo::read(&MEMO, key(twin_net, form, meters, per_span, cuts), compute)
    }

    /// The input rendered whole, each scalar as its certified bracket's
    /// two ends (NaN where it does not certify), as limb 2's round memo
    /// renders its own.
    fn key(
        twin_net: &NurbsCurve3<Interval>,
        form: &CanonicalSurface,
        meters: projected::Meters<Interval>,
        per_span: usize,
        cuts: &[f64],
    ) -> String {
        use geom_core::CertifiedEnclosure as _;
        use projected::Meters;
        let end = |x: Interval, hi: bool| {
            x.certified_bracket()
                .map_or(f64::NAN, |(lo, up)| if hi { up } else { lo })
        };
        let ends = |xs: &[Interval]| -> Vec<u64> {
            xs.iter()
                .flat_map(|&x| [end(x, false).to_bits(), end(x, true).to_bits()])
                .collect()
        };
        let form = match *form {
            CanonicalSurface::Plane => (0, ends(&[])),
            CanonicalSurface::Cylinder { radius } => (1, ends(&[radius])),
            CanonicalSurface::Sphere { radius } => (2, ends(&[radius])),
            CanonicalSurface::Cone { cos2, sin2 } => (3, ends(&[cos2, sin2])),
            CanonicalSurface::Torus { major, minor } => (4, ends(&[major, minor])),
        };
        let meters = match meters {
            Meters::Plane => (0, ends(&[])),
            Meters::Cylinder { radius } => (1, ends(&[radius])),
            Meters::Sphere { radius } => (2, ends(&[radius])),
            Meters::Cone { sin, cos, nappe } => (
                if nappe == crate::offset::Nappe::Opening {
                    3
                } else {
                    5
                },
                ends(&[sin, cos]),
            ),
            Meters::Torus { major, minor } => (4, ends(&[major, minor])),
        };
        format!(
            "{:?}|{:?}|{form:x?}|{meters:x?}|{per_span}|{:x?}",
            twin_net.map_scalar(|x| end(x, false)),
            twin_net.map_scalar(|x| end(x, true)),
            cuts.iter().map(|c| c.to_bits()).collect::<Vec<_>>(),
        )
    }

    /// This thread's memo hits and misses so far.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn counts() -> (u64, u64) {
        crate::memo::counts(&MEMO)
    }
}

/// **Limb 2's reader: an `Intersection` carrier's distance from an
/// analytic surface**, for a carrier of every kind — limb 2 of C2 for an
/// analytic operand ([`crate::analytic_edge_limbs`]). The carrier is read
/// through its net ([`crate::carrier_net::framed_carrier_net`]: a spline
/// is its own net, a line, circle, ellipse or spiric its certified
/// enclosing net), written in the frame of the surface's orthonormal
/// twin in certification arithmetic, the frame applied to the carrier's
/// own data before anything is combined, so each control rounds at its
/// distance from the operand's origin and never at its distance from the
/// world's. The twin's canonical composite along the net is converted to
/// metres per part (the cone's and the torus's lever per part, the cone's
/// apex root where its lever is not positive) and halved under limb 2's
/// budget ([`projected::offset_rounds`]): a spline's rounds insert their
/// breaks into its own net exactly, and a closed form's re-enclose the
/// spans they cut, over [`closed_form_grid`]'s first breaks. No round
/// reads a closed form's net past the window's outer ends; a spline is
/// read whole, its parts the window overlaps deciding. A curved closed
/// form is read first on its dyadic spans (`limb2_quick`), which may
/// reach past the window's ends (a bound over a span holding a piece of
/// the window bounds that piece); that reading passes only on cleared
/// parts that cover the window's outer ends with no gap, and hands
/// anything else to the rounds, so every refusal is the rounds'.
///
/// The reading carries the size of the net it stopped on
/// ([`projected::NetSize`]): the carrier's own enclosure width there,
/// below which no composite of it can bound anything, and its extent.
///
/// # Errors
///
/// [`PcurveCertifyError::CarrierEnclosure`] for a span the carrier has
/// no certified net over; [`PcurveCertifyError::UnsupportedChart`] off
/// an analytic chart; as [`projected::offset_rounds`] otherwise.
pub(crate) fn limb2_offset_sup<T: Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<T>,
    window: (T, T),
    surface: &Surface<T>,
    band: Band,
) -> Result<projected::OffsetRead<T>, PcurveCertifyError> {
    let Some((origin, e)) = projected::chart_frame(surface) else {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: surface.kind(),
        });
    };
    let chart = projected::chart_at(
        surface,
        || (carrier.mid_point(window.0, window.1) - origin).dot(e[2]),
        band,
    )?;
    let twin = orthonormal_chart(surface);
    let Some((o, te)) = projected::chart_frame(&twin) else {
        unreachable!("an analytic chart has a frame")
    };
    let frame = crate::carrier_net::Frame::of(o, te);
    let (domain, grid, per_span) = match carrier {
        Curve3::Nurbs(net) => (net.domain(), Vec::new(), projected::INCIDENCE_CUTS),
        _ => {
            let (lo, hi) = crate::carrier_net::outer_ends(window);
            ((lo, hi), closed_form_grid(carrier, lo, hi), 1)
        }
    };
    let round_over =
        |grid: &[f64],
         cuts: &[f64]|
         -> Result<(projected::ProjectedHull, projected::NetSize), PcurveCertifyError> {
            let mut breaks: Vec<f64> = grid.iter().chain(cuts).copied().collect();
            breaks.sort_by(f64::total_cmp);
            breaks.dedup();
            let net = crate::carrier_net::framed_carrier_net(carrier, domain, &breaks, &frame)
                .map_err(PcurveCertifyError::CarrierEnclosure)?;
            let curve = net.curve().ok_or(PcurveCertifyError::ImageMismatch {
                image: PcurveKind::Projected,
                why: "the carrier's net forms no curve",
            })?;
            let size = projected::NetSize {
                width: net.width(),
                rounding: net.rounding(),
                extent: net.extent(),
            };
            let hull = projected::ProjectedHull {
                pieces: Vec::new(),
                spans: twin_span_hulls(&curve, &twin, &chart, per_span, net.cuts())?,
            };
            Ok((hull, size))
        };
    let round_hull = |cuts: &[f64]| round_over(&grid, cuts);
    // A curved closed form is read first on its dyadic spans, each
    // enclosed alone (`limb2_quick`); a reading that clears is the
    // limb's, and anything else goes to the rounds below. Its parts
    // all clear, so `offset_cleared`'s decisions (the window's ends
    // located, a cone's or torus's lever per part) are taken only on a
    // reading about to pass; where a lever then refuses, the rounds
    // take those decisions again, so the trail shows them twice.
    let quick = if limb2_quick::enabled() {
        limb2_quick::read(carrier, domain, surface, &frame, &twin, &chart, band).and_then(
            |(hull, size)| {
                projected::offset_cleared(hull, size, window, &chart, &twin, band)
                    .ok()
                    .flatten()
            },
        )
    } else {
        None
    };
    let mut read = match quick {
        Some(read) => read,
        None => {
            // The rounds read through limb 2's memo (`limb2_memo`): a
            // round is a pure function of the key's inputs and its cuts.
            let key = limb2_memo::Key::of(carrier, window, surface, band);
            let hull = |cuts: &[f64]| limb2_memo::read(&key, cuts, || round_hull(cuts));
            projected::offset_rounds(hull, window, &chart, &twin, band)?
        }
    };
    read.upper = read.offset.upper();
    Ok(read)
}

/// **Limb 2's round memo**: one round of the composite
/// ([`limb2_offset_sup`]'s `round_hull`: the carrier's framed net cut at
/// the round's breaks, and the twin's composite along it, converted to
/// metres per part) keyed by exactly what it reads.
///
/// **What the key holds, and why that is all.** The round reads the
/// carrier, the operand surface (its orthonormal twin, the frame and the
/// chart are derived from it, the chart also from the carrier's middle
/// over the window and from the band), the window, the band and the
/// cuts, and nothing else: no decision is taken in a round (the
/// composite is certification arithmetic; the chart's decisions are
/// taken outside the memo, every call), so a hit skips no verdict, no
/// escalation and no decision count. Each scalar of the carrier, the
/// surface and the window enters the key as its certified bracket's two
/// ends' bits (NaN where it does not certify), beside the scalar's
/// witness kind: the twin is formed in the scalar, and an exact scalar
/// (`Interval`, and `Sym<Interval>`, whose value channel is
/// `Interval`'s bit for bit) rounds it outward where an inexact one
/// (`f64`, `Probe`) rounds it to nearest. The round's output is `f64`
/// data with no symbolic node in it, so sharing one between `Interval`
/// and `Sym<Interval>` hands each the bits it would have computed. The
/// key is the whole rendering, compared whole: no hash stands for it,
/// so no collision can return another input's round (D9: a hit returns
/// exactly what a miss computes).
///
/// **Where it lives**: a bounded thread-local memo ([`crate::memo`]),
/// so it needs no owner to thread through every door that attaches an
/// edge. A round is a pure function of its key, so an entry left from
/// another body or document is still that key's round, never stale.
mod limb2_memo {
    use super::{PcurveCertifyError, projected};
    use geom::{Curve3, Surface};
    use geom_core::CertifiedBounds;
    use geom_core::predicate::Band;

    pub(super) type Round =
        Result<(projected::ProjectedHull, projected::NetSize), PcurveCertifyError>;

    std::thread_local! {
        static MEMO: core::cell::RefCell<crate::memo::Memo<(String, Vec<u64>), Round>> =
            core::cell::RefCell::new(crate::memo::Memo::new());
    }

    /// The inputs a round reads, rendered whole.
    pub(super) struct Key(String);

    impl Key {
        pub(super) fn of<T: CertifiedBounds>(
            carrier: &Curve3<T>,
            window: (T, T),
            surface: &Surface<T>,
            band: Band,
        ) -> Self {
            let (lo, hi) = (|x: T| end(x, false), |x: T| end(x, true));
            let bits = |x: f64| x.to_bits();
            Self(format!(
                "{}|{:x?}|{:x},{:x}",
                rendered(carrier, surface),
                [lo(window.0), hi(window.0), lo(window.1), hi(window.1)].map(bits),
                band.zero().to_bits(),
                band.escalate().to_bits(),
            ))
        }
    }

    /// A scalar's certified bracket's lower (`hi` false) or upper end,
    /// NaN where it does not certify.
    fn end<T: CertifiedBounds>(x: T, hi: bool) -> f64 {
        x.certified_bracket()
            .map_or(f64::NAN, |(lo, up)| if hi { up } else { lo })
    }

    /// The carrier and the surface rendered whole, each scalar as its
    /// certified bracket's two ends beside the scalar's witness kind.
    pub(super) fn rendered<T: CertifiedBounds>(
        carrier: &Curve3<T>,
        surface: &Surface<T>,
    ) -> String {
        let (lo, hi) = (|x: T| end(x, false), |x: T| end(x, true));
        format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}",
            T::WITNESS,
            carrier.map_scalar(lo),
            carrier.map_scalar(hi),
            surface.map_scalar(lo),
            surface.map_scalar(hi),
        )
    }

    /// The round at `key` cut at `cuts`: the memo's, or `compute`'s,
    /// which is then kept.
    pub(super) fn read(key: &Key, cuts: &[f64], compute: impl FnOnce() -> Round) -> Round {
        let k = (
            key.0.clone(),
            cuts.iter().map(|c| c.to_bits()).collect::<Vec<_>>(),
        );
        crate::memo::read(&MEMO, k, compute)
    }

    /// This thread's memo hits and misses so far.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn counts() -> (u64, u64) {
        crate::memo::counts(&MEMO)
    }

    /// Empties this thread's memo, so the next read of every key
    /// computes.
    #[cfg(test)]
    pub(crate) fn clear() {
        crate::memo::clear(&MEMO);
    }
}

/// **Limb 2's first reading of a curved closed form**: the carrier cut
/// at the dyadic points of its angle parameter (multiples of `τ/2ᵈ`),
/// each span enclosed alone as a degree-[`limb2_quick::ORDER`] Bézier
/// segment ([`crate::carrier_net::framed_span_net`]) in the operand's
/// twin frame and composed with the twin's canonical form as the rounds
/// compose each part ([`twin_span_hulls`]). A span whose bound does not
/// clear the band is replaced by its dyadic halves that meet the edge's
/// outer ends, and only those are recomposed.
///
/// **What it hands on.** Parts that clear the band and **cover** the
/// outer ends: sorted, each starting exactly where the one before ends,
/// the first starting at or before the lower end and the last ending at
/// or past the upper one ([`limb2_quick::covering`]). Anything else is
/// `None`, so the rounds decide: a span with no certified net or
/// composite, a halving that no longer halves the bound, a span at
/// [`limb2_quick::DEPTH`], more than [`limb2_quick::WORK`] spans, or a
/// covering with a gap. No span is ever dropped from the reading: a span
/// that cannot be halved further ends it with no pass.
///
/// **Past the window.** A span may reach past the outer ends: a bound
/// over a span holding a piece of the window bounds that piece. A span
/// past the ends whose halving no longer halves its bound (the carrier
/// past the edge's ends is not the edge's: a spiric past its oval has
/// no net there) is read again clipped to the ends, and halved clipped
/// from then on; a span past the ends that still halves keeps halving
/// unclipped, its halves that leave the ends dropped as not the edge's.
///
/// **Why the spans are dyadic and alone.** A span's reading is then a
/// function of the carrier, the operand, the chart (a cone's nappe,
/// which the chart takes from the window's middle, is in the key) and
/// the span's ends, so every edge that shares a carrier, an operand and
/// a nappe shares its spans inside both windows (a split edge's pieces,
/// an edge read at its attach and at rest, an edge re-attached over a
/// moved window), and they are read once ([`limb2_quick`]'s memo). The
/// degree is the Taylor remainder's lever: the remainder falls as the
/// span's `ORDER + 1`th power, so a higher degree reaches the band on a
/// few spans where [`crate::carrier_net::ORDER`] needs tens.
mod limb2_quick {
    use super::twin_span_hulls;
    use crate::pcurve_cache::projected::{NetSize, ProjectedChart, ProjectedHull, SpanHull};
    use geom::{Curve3, Surface};
    use geom_core::CertifiedBounds;
    use geom_core::predicate::Band;

    /// The Taylor degree of a span's net. The remainder of a span of
    /// length `h` is about `h^(ORDER+1)/(ORDER+1)!` times the carrier's
    /// size: at 12, a quarter turn of a unit circle carries
    /// `(π/2)¹³/13! ≈ 5.7e-8` and an eighth `≈ 7e-12`, so the finest
    /// band is reached within a halving or two, while the conversion's
    /// cost grows as the degree's square and its rounding with the
    /// binomials it divides by.
    pub(super) const ORDER: usize = 12;

    /// The first spans' level: quarter turns. A coarser span's remainder
    /// at [`ORDER`] is above every band on a unit carrier (a half turn
    /// carries `π¹³/13! ≈ 0.04`), so its composite would only be thrown
    /// away; a finer start composes spans a quarter turn would have
    /// cleared.
    const START: u32 = 2;

    /// The most spans one reading composes, memo hits included: a few
    /// rounds' worth of the rounds' own work (a round composes tens of
    /// spans), so a carrier the first reading cannot clear costs about
    /// that before the rounds decide.
    pub(super) const WORK: usize = 256;

    /// The deepest level a span is halved to. [`dyadic`]'s step is
    /// `τ/2ᵈ` with `2ᵈ` a `u32`, so a level-30 span's halves are still
    /// representable; a span of `τ/2³⁰ ≈ 5.9e-9` rad that has not
    /// cleared is at its net's rounding, not its remainder, and a span
    /// at this level that does not clear ends the reading.
    pub(super) const DEPTH: u32 = 30;

    /// One span's reading: its part, and its net's widths and box.
    #[derive(Clone, Copy)]
    pub(super) struct Part {
        pub(super) hull: SpanHull,
        width: f64,
        rounding: f64,
        boxed: Option<[(f64, f64); 3]>,
    }

    type Key = (std::rc::Rc<str>, u64, u64);

    std::thread_local! {
        static MEMO: core::cell::RefCell<crate::memo::Memo<Key, Option<Part>>> =
            core::cell::RefCell::new(crate::memo::Memo::new());
    }

    #[cfg(test)]
    std::thread_local! {
        static OFF: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    }

    /// Whether the first reading is taken: always, but under a test's
    /// `without`.
    pub(super) fn enabled() -> bool {
        #[cfg(test)]
        {
            !OFF.with(core::cell::Cell::get)
        }
        #[cfg(not(test))]
        {
            true
        }
    }

    /// `f` with the first reading not taken on this thread, so limb 2
    /// is its rounds alone.
    #[cfg(test)]
    pub(crate) fn without<R>(f: impl FnOnce() -> R) -> R {
        let was = OFF.with(|o| o.replace(true));
        let r = f();
        OFF.with(|o| o.set(was));
        r
    }

    /// This thread's span hits and misses so far.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn counts() -> (u64, u64) {
        crate::memo::counts(&MEMO)
    }

    /// A cleared part over `range`, for rows of [`covering`].
    #[cfg(test)]
    pub(crate) fn part_over(range: (f64, f64)) -> Part {
        Part {
            hull: SpanHull {
                range,
                f_sup: 0.0,
                rho_lo: 1.0,
                z: (0.0, 0.0),
                meters: 0.0,
                floors: (0.0, 0.0),
            },
            width: 0.0,
            rounding: 0.0,
            boxed: None,
        }
    }

    /// Empties this thread's memo, so the next read of every span
    /// computes.
    #[cfg(test)]
    pub(crate) fn clear() {
        crate::memo::clear(&MEMO);
    }

    /// Dyadic span `k` at level `d`, `[k·τ/2ᵈ, (k + 1)·τ/2ᵈ]`. Two
    /// neighbours' shared end is the same bits at every level: `τ/2ᵈ⁺¹`
    /// is `τ/2ᵈ` halved exactly, so `2j·τ/2ᵈ⁺¹` and `j·τ/2ᵈ` round the
    /// same real.
    fn dyadic(d: u32, k: i64) -> (f64, f64) {
        let h = core::f64::consts::TAU / f64::from(1u32 << d);
        #[allow(clippy::cast_precision_loss)]
        ((k as f64) * h, ((k + 1) as f64) * h)
    }

    /// [`dyadic`] clipped to `[lo, hi]`; `None` where the clip is empty.
    fn clip(d: u32, k: i64, (lo, hi): (f64, f64)) -> Option<(f64, f64)> {
        let (a, b) = dyadic(d, k);
        let (a, b) = (a.max(lo), b.min(hi));
        (a < b).then_some((a, b))
    }

    /// The level-`d` spans meeting `[lo, hi]`; `None` past [`WORK`].
    fn cover(d: u32, (lo, hi): (f64, f64)) -> Option<Vec<(u32, i64)>> {
        let h = core::f64::consts::TAU / f64::from(1u32 << d);
        let (first, last) = ((lo / h).floor(), (hi / h).ceil());
        #[allow(clippy::cast_precision_loss)]
        if !(first.is_finite() && last.is_finite()) || last - first > WORK as f64 {
            return None;
        }
        #[allow(clippy::cast_possible_truncation)]
        let (first, last) = (first as i64 - 1, last as i64 + 1);
        Some(
            (first..=last)
                .filter(|&k| clip(d, k, (lo, hi)).is_some())
                .map(|k| (d, k))
                .collect(),
        )
    }

    /// `(d, k)`'s halves that meet `[lo, hi]`, each descended (where
    /// read `clipped`) past any level where the clip leaves it whole, so
    /// every half read is a shorter span than its parent's. `None` where
    /// a half would be past [`DEPTH`], or none meets `[lo, hi]`: the
    /// span cannot be halved, and the reading ends.
    fn halves(d: u32, k: i64, domain: (f64, f64), clipped: bool) -> Option<Vec<(u32, i64)>> {
        let whole = if clipped { clip(d, k, domain) } else { None };
        let (mut d, mut k) = (d, k);
        loop {
            if d >= DEPTH {
                return None;
            }
            let kids: Vec<(u32, i64)> = [(d + 1, 2 * k), (d + 1, 2 * k + 1)]
                .into_iter()
                .filter(|&(e, j)| clip(e, j, domain).is_some())
                .collect();
            match kids.as_slice() {
                [] => return None,
                [(e, j)] if clip(*e, *j, domain) == whole => (d, k) = (*e, *j),
                _ => return Some(kids),
            }
        }
    }

    /// **The cleared parts as a covering of `[lo, hi]`**, sorted: the
    /// first starts at or before `lo`, each next one starts exactly where
    /// the one before ends, and the last ends at or past `hi`. `None`
    /// otherwise: a gap is a piece of the edge no part has read, which
    /// [`super::projected::window_of`] would read as the part before
    /// it, so a reading with one does not pass.
    pub(super) fn covering(mut parts: Vec<Part>, (lo, hi): (f64, f64)) -> Option<Vec<Part>> {
        parts.sort_by(|x, y| x.hull.range.0.total_cmp(&y.hull.range.0));
        let first = parts.first()?;
        let reaches_lo = first.hull.range.0 <= lo;
        if !reaches_lo {
            return None;
        }
        let mut reach = first.hull.range.1;
        for p in &parts[1..] {
            if p.hull.range.0 != reach {
                return None;
            }
            reach = p.hull.range.1;
        }
        (reach >= hi).then_some(parts)
    }

    /// [`super::limb2_offset_sup`]'s first reading (module docs): the
    /// parts covering `domain`, every one clearing the band, and the
    /// size of their nets; `None` otherwise.
    pub(super) fn read<T: CertifiedBounds>(
        carrier: &Curve3<T>,
        domain: (f64, f64),
        surface: &Surface<T>,
        frame: &crate::carrier_net::Frame,
        twin: &Surface<T>,
        chart: &ProjectedChart<T>,
        band: Band,
    ) -> Option<(ProjectedHull, NetSize)> {
        if !crate::carrier_net::has_span_nets(carrier) {
            return None;
        }
        let nappe = match chart {
            ProjectedChart::Cone { nappe, .. } => format!("{nappe:?}"),
            _ => String::new(),
        };
        let base: std::rc::Rc<str> = format!(
            "{}|{nappe}|{ORDER}",
            super::limb2_memo::rendered(carrier, surface)
        )
        .into();
        let compute = |range: (f64, f64)| -> Option<Part> {
            let net = crate::carrier_net::framed_span_net(carrier, range, frame, ORDER).ok()?;
            let curve = net.curve()?;
            let hull = match twin_span_hulls(&curve, twin, chart, 1, &[])
                .ok()?
                .as_slice()
            {
                [one] => *one,
                _ => return None,
            };
            Some(Part {
                hull,
                width: net.width(),
                rounding: net.rounding(),
                boxed: net.control_box(),
            })
        };
        let clears =
            |p: &Option<Part>| p.is_some_and(|p| crate::ssi::certify::clears(p.hull.meters, band));
        // Each pending span: its dyadic index, its parent's bound, and
        // whether it is read clipped to the window's outer ends.
        let mut pending: Vec<((u32, i64), f64, bool)> = cover(START, domain)?
            .into_iter()
            .map(|dk| (dk, f64::INFINITY, false))
            .collect();
        let (mut cleared, mut work) = (Vec::new(), 0usize);
        while let Some(((d, k), parent, clipped)) = pending.pop() {
            work += 1;
            if work > WORK {
                return None;
            }
            let range = if clipped {
                clip(d, k, domain)?
            } else {
                dyadic(d, k)
            };
            let p = crate::memo::read(
                &MEMO,
                (base.clone(), range.0.to_bits(), range.1.to_bits()),
                || compute(range),
            );
            if clears(&p) {
                cleared.extend(p);
                continue;
            }
            // A halving that no longer halves the bound is at the net's
            // rounding (or the bound is NaN, or the span has no net):
            // no deeper span clears. A span reaching past the window's
            // ends is then read again within them, since the carrier
            // past the edge's ends is not the edge's.
            let bound = p.map_or(f64::NAN, |p| p.hull.meters);
            let halved = bound <= 0.5 * parent;
            if !halved {
                if clipped || clip(d, k, domain) == Some(range) {
                    return None;
                }
                pending.push(((d, k), f64::INFINITY, true));
                continue;
            }
            pending.extend(
                halves(d, k, domain, clipped)?
                    .into_iter()
                    .map(|dk| (dk, bound, clipped)),
            );
        }
        let cleared = covering(cleared, domain)?;
        let widest = crate::carrier_net::widest;
        let boxed = cleared
            .iter()
            .try_fold([(f64::INFINITY, f64::NEG_INFINITY); 3], |acc, p| {
                let b = p.boxed?;
                Some(core::array::from_fn(|i| {
                    (acc[i].0.min(b[i].0), acc[i].1.max(b[i].1))
                }))
            });
        let size = NetSize {
            width: cleared.iter().map(|p| p.width).fold(0.0, widest),
            rounding: cleared.iter().map(|p| p.rounding).fold(0.0, widest),
            extent: boxed.map_or(f64::NAN, |b| crate::carrier_net::diagonal(&b)),
        };
        let hull = ProjectedHull {
            pieces: Vec::new(),
            spans: cleared.iter().map(|p| p.hull).collect(),
        };
        Some((hull, size))
    }
}

/// The breaks a closed-form carrier's first net is cut at inside
/// `[lo, hi]`: none for a line, whose degree-1 net is exact; for a
/// curved kind, equal spans of at most a [`crate::ssi::SSI_CERT_SPANS`]th
/// of a turn of its angle parameter, so a span's Taylor remainder is
/// bounded by the turn's fraction whatever the arc's length.
fn closed_form_grid<T: Real>(carrier: &Curve3<T>, lo: f64, hi: f64) -> Vec<f64> {
    if matches!(carrier, Curve3::Line { .. }) {
        return Vec::new();
    }
    #[allow(clippy::cast_precision_loss)]
    let turn = core::f64::consts::TAU / crate::ssi::SSI_CERT_SPANS as f64;
    let reach = ((hi - lo) / turn).ceil();
    if !reach.is_finite() || reach < 2.0 {
        return Vec::new();
    }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let n = (reach as usize).min(crate::ssi::SSI_CERT_SPANS);
    #[allow(clippy::cast_precision_loss)]
    (1..n)
        .map(|k| lo + (hi - lo) * (k as f64 / n as f64))
        .collect()
}

/// The fitted door's body for check 4 ([`crate::FittedLane`]): limb 2
/// of C2 against this chart alone, a certified bound on
/// `sup |S(P(t)) − C(t)|` over the image's domain, its rounds halved as
/// `ssi::certify`'s limb 2 halves them.
/// The image must cover the edge's interval `params`: its domain is
/// read as `f64` structure against the interval's outer ends (C6).
///
/// # Errors
///
/// [`PcurveCertifyError::FittedCertificate`] when the bound refuses or
/// the image stops short of the edge;
/// [`PcurveCertifyError::CarrierEnclosure`] when the carrier has no
/// certified net over a span of the image's domain.
pub(crate) fn fitted_lane<T: Decide + geom_core::Bounds + geom_core::CertifiedEnclosure>(
    carrier: &Curve3<T>,
    image: &NurbsCurve2<T>,
    surface: &Surface<T>,
    (t0, t1): (T, T),
    band: Band,
) -> Result<T, PcurveCertifyError> {
    let chart = match surface {
        Surface::Nurbs(n) => n,
        Surface::Approx(a) => a.fit(),
        Surface::Plane { .. }
        | Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Sphere { .. }
        | Surface::Torus { .. } => {
            unreachable!("fitted_lane: `run_fitted_checks` admits only a spline chart at check 1")
        }
    };
    // The bound is over the image's domain, so the edge's interval must
    // lie inside it, for every carrier kind. Nothing upstream says so of
    // a spline: its evaluation clamps at its knot domain, so an edge's
    // end check reads `C(1)` at `t = 1.5` and passes. The domain ends
    // and the interval's outer ends are `f64` structure here (C6): the
    // knot domain is stored `f64`, and the outer ends are the edge
    // interval's evaluation bracket (`Bounds`), so the comparison decides
    // containment of every point the scalar admits and rounds nothing. A
    // bracket whose decoration refuses never reaches here: check 2 of
    // `run_fitted_checks`, this gate's only caller, has already decided
    // `t1 − t0` through `decide`, which refuses an uncertified value.
    let (d0, d1) = image.knots_f64_interim().domain();
    let (lo, hi) = crate::carrier_net::outer_ends((t0, t1));
    if !(d0 <= lo && hi <= d1) {
        return Err(PcurveCertifyError::FittedCertificate {
            limb: Some(SsiLimb::HullSupChart),
            what: "the image's domain stops short of the edge's interval, so its hull \
                   bound says nothing about the edge's ends",
            magnitude: None,
        });
    }
    use crate::ssi::certify::{CHART_HULL, Refused, carrier_chart_round, chart_grid, hull_limb};
    // Each round's composite against this chart alone, the carrier read
    // through the carrier-enclosure door over the image's domain, cut at
    // `cuts` besides the round's grid: a halving shrinks a closed form's
    // remainder too.
    let (pcoords, scoords) = (image.certified_coords(), chart.certified_coords());
    let pimage = (
        image.knots_f64_interim(),
        image.weights_f64_interim(),
        &pcoords[..],
    );
    let schart = (
        chart.knots_u_f64_interim(),
        chart.knots_v_f64_interim(),
        chart.weights_f64_interim(),
        &scoords[..],
    );
    let grid = chart_grid(carrier, image.knots_f64_interim());
    let stopped = crate::ssi::certify::subdivide(
        |cuts: &[f64]| carrier_chart_round(carrier, pimage, schart, &grid, cuts),
        band,
    );
    let stopped = stopped.map_err(ssi_refusal)?;
    hull_limb(&CHART_HULL, stopped, band, &mut Refused::default()).map_err(ssi_refusal)
}

/// The hull limb's refusal, reduced to the three parts this module's
/// closed enum carries — the limb, the reason (including the
/// escalating predicate's own name) and the offending margin. That
/// triple is the actionable content; see the
/// [`PcurveCertifyError::FittedCertificate`] docs for what the
/// flattening buys and, honestly, what it does not.
///
/// Its one producer is [`fitted_lane`], which runs limb 2's hull
/// against the row's chart and nothing else: the composite refuses
/// [`SsiError::UnsupportedCertificate`](crate::ssi::SsiError), and the
/// limb's verdict a definite [`SsiError::CertificateLimb`](crate::ssi::SsiError::CertificateLimb) or an
/// escalated `CertificateEscalated`. Those are the arms written out.
///
/// **Escalations leave by a different door.** They are the only
/// refusals carrying a margin the classifier judged, and that margin is
/// unreadable without the band it was judged against, so the
/// escalation travels whole, its limb with its `Indeterminate`, in
/// [`PcurveCertifyError::FittedEscalated`].
fn ssi_refusal(e: crate::ssi::SsiError) -> PcurveCertifyError {
    use crate::ssi::SsiError as E;
    let (limb, what, magnitude) = match e {
        E::CertificateEscalated { limb, cause } => {
            return PcurveCertifyError::FittedEscalated {
                decision: SsiDecision::Limb(limb),
                cause,
            };
        }
        E::CertificateLimb { limb, margin } => {
            return PcurveCertifyError::FittedLimb { limb, margin };
        }
        E::UnsupportedCertificate { what } => (None, what, None),
        E::CarrierEnclosure(refusal) => return PcurveCertifyError::CarrierEnclosure(refusal),
        // Exhaustive BY VARIANT rather than by catch-all: a new
        // `SsiError` must be dispositioned here deliberately, and the
        // compiler is what enforces that. The hull limb raises none of
        // these (the trace's, the tube's, the foot projection's and the
        // marching doors' refusals); were one to reach here it is a
        // structural refusal whose full text lives at the SSI door.
        E::Escalated { .. }
        | E::ChartRow { .. }
        | E::RefinementExhausted { .. }
        | E::TubeStraddles { .. }
        | E::FootPointInconclusive { .. }
        | E::TubeLadderEmpty { .. }
        | E::TubeProbeSilent { .. }
        | E::ChartSpeed(_)
        | E::TubeDegenerate(_)
        | E::TubeNotOneArc { .. }
        | E::TransversalityBand { .. }
        | E::PairTangent { .. }
        | E::RadiusNotPositive { .. }
        | E::DegenerateTorus(_)
        | E::CoincidentTori
        | E::OperandNotFinite { .. }
        | E::ExhaustivenessInconclusive(_)
        | E::CellBudget { .. }
        | E::StepBudget { .. }
        | E::FloorUnresolvable(_)
        | E::SettlingUnresolvable(_)
        | E::StepCollapsed { .. }
        | E::StepUnusable { .. }
        | E::SeedRefinementFailed { .. }
        | E::SeedOffDomain { .. }
        | E::StepRefinementFailed { .. }
        | E::SelfCrossingLocus { .. }
        | E::Fit(_)
        | E::TraceUnresolved { .. }
        | E::BoundaryGraze { .. }
        | E::BoundaryTangent { .. }
        | E::SideSignBudget { .. }
        | E::EndNotOnLocus { .. }
        | E::CrossingUnmatched { .. }
        | E::PairBoundaryUnresolved { .. }
        | E::SeedOutsideTube { .. }
        | E::ShortBranchUncertified { .. }
        | E::MarchStepInBand { .. }
        | E::MarchShortOfFit { .. }
        | E::WindowShortOfWall { .. }
        | E::DomainUnusable { .. }
        | E::WrongLane { .. }
        | E::InvalidMarchTol { .. }
        | E::MarchTolMismatch { .. } => (
            None,
            "the hull limb refused structurally (see geom_brep::SsiError for the full text \
             at the SSI door)",
            None,
        ),
    };
    PcurveCertifyError::FittedCertificate {
        limb,
        what,
        magnitude,
    }
}

/// A certified pcurve cache: the chart image, the carrier-parameter
/// interval it is certified over, and the [`PcurveCertificate`] of the
/// run. Constructible only through its certifying doors
/// ([`PcurveCache::certify`], [`PcurveCache::certify_fitted`],
/// [`PcurveCache::certify_general`]) or from a cache one of them built —
/// the fields are private, so an uncertified pcurve is unrepresentable
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

    /// The same certified cache with its image mirrored in `v`
    /// ([`Pcurve::mirror_v`]), for a chart whose second frame axis was
    /// negated — the interval and the certificate travel verbatim.
    ///
    /// Why the certificate is still the certificate is stated once,
    /// on [`Pcurve::mirror_v`]: a certificate stores norms and nothing
    /// in chart coordinates ([`PcurveCertificate`]'s fields are
    /// `samples`, `max_residual`, `envelope`, `statement` and the
    /// fitted lane's `ssi`), and every value the schedule metres is
    /// bit-identical on the mirrored chart. So this is the same kind
    /// of door as [`crate::EdgeCurve::with_remapped_surfaces`]: it
    /// cannot express a geometry change, only the one re-statement
    /// under which no metred value can move, which is why it hands
    /// out a certificate without a run.
    ///
    /// # Errors
    ///
    /// `None` exactly when [`Pcurve::mirror_v`] answers `None` — a
    /// [`SpiricImage::Wall`] image, which has no reflected locus; that
    /// door's own docs say why, and why no door in the tree reaches it.
    #[must_use]
    pub fn mirrored_v(&self) -> Option<Self> {
        Some(Self {
            pcurve: self.pcurve.mirror_v()?,
            param_start: self.param_start,
            param_end: self.param_end,
            certificate: self.certificate,
        })
    }
}

impl<T: Decide> PcurveCache<T> {
    /// Certifies a **closed-form** image — [`Pcurve::Harmonic`],
    /// [`Pcurve::IsoLine`], [`Pcurve::IsoArc`], [`Pcurve::Spiric`] or
    /// [`Pcurve::FocalSection`] — of `carrier` on `surface` over
    /// `[t0, t1]`: the minting lane's door, at every `Decide` scalar.
    /// Which of the five is being certified selects the check-4
    /// statement and nothing else; the other three checks are one
    /// sequence.
    ///
    /// A [`Pcurve::Fitted`] image refuses here, naming
    /// [`PcurveCache::certify_fitted`]: the fitted lane's certificate
    /// is a hull bound, which needs a CERTIFYING scalar
    /// (`Decide + Bounds + CertifiedEnclosure`), and hiding that
    /// behind this signature
    /// would put the whole minting pipeline behind a bound it does not
    /// need (a dual body mints closed-form pcurves perfectly well).
    ///
    /// The check sequence (fixed order, D9; every margin in metres):
    ///
    /// 1. **Lane**: the chart and carrier kinds are ones this module
    ///    certifies in closed form. On a [`Pcurve::Harmonic`] image, the
    ///    **kind check**: each channel is its chart coordinate's kind, a
    ///    [`Channel::Angle`] where the coordinate is an angle and a
    ///    [`Channel::Free`] where it is not
    ///    ([`PcurveCertifyError::ChannelKindMismatch`]); and the stored
    ///    windings are counts a closed-form table holds, at most one
    ///    turn per carrier period
    ///    ([`PcurveCertifyError::ChartWindingUnsupported`]). Structure,
    ///    compared with no decision.
    /// 2. **Interval**: `t₁ − t₀` is definitely forward, metered
    ///    through the carrier's own rate. The image's angles span at
    ///    most one period because the carrier's interval does (the edge
    ///    certificate's `interval_span_winding`); a spline net's
    ///    projected image, whose parameter is no angle, has its own
    ///    period gate here.
    /// 3. **Schedule**: `|S(P(tᵢ)) − C(tᵢ)| ≤ ε` at the
    ///    [`CERT_SAMPLES`] schedule — evaluated through
    ///    `Surface::eval` and `Curve3::eval` directly, so the closed
    ///    form step 4 states is *verified* at the samples, never
    ///    trusted. On a [`Pcurve::Harmonic`] or [`Pcurve::FocalSection`]
    ///    image step 4 is the whole
    ///    certified statement and this step is its cross-check: it runs
    ///    on the witness lane ([`geom_core::Witness::Inexact`]: `f64`,
    ///    `Sym<f64>`), records no verdict, and is not run at an
    ///    exact-witness scalar (`Interval`, `Sym<Interval>`), at a point
    ///    or over a box.
    /// 4. **Envelope**: the between-samples sup bound over the whole
    ///    span ≤ ε, by the lane the variant selects.
    ///    - [`Pcurve::Harmonic`]: the closed-form bound
    ///      ([`EnvelopeStatement::MapResidualClosedForm`]) — on a
    ///      periodic chart the carrier's incidence with the chart plus
    ///      the stored image's fidelity to the image re-derived from the
    ///      carrier. The **winding compare** comes first: the stored
    ///      windings equal the derived name's exactly, or the row refuses
    ///      as [`PcurveCertifyError::WindingMismatch`]. Fidelity then
    ///      carries every drift the real coefficients admit (an ε-shell
    ///      around the exact harmonic family), so the stored envelope
    ///      bounds the pcurve actually certified; it is zero on every
    ///      minted cache, and exists so the certificate is honest for
    ///      every input `certify` admits, attach-path ones included. A
    ///      refusal names its term ([`PcurveCheck::EnvelopeTerm`]).
    ///    - [`Pcurve::IsoLine`] / [`Pcurve::IsoArc`]: the traversed
    ///      row's control-difference hull
    ///      ([`EnvelopeStatement::MapResidualIsoHull`]).
    ///    - [`Pcurve::Spiric`]: `span{1, f, sin t}` in closed form for
    ///      a cap image, and an algebraic IDENTITY for a wall image
    ///      ([`EnvelopeStatement::SpiricIdentity`]) — each plus the
    ///      drift its own check 1 admitted, and each zero on every
    ///      minted image, for the same reason fidelity is.
    ///    - [`Pcurve::FocalSection`]: the harmonic closed form of the
    ///      image's Kepler decomposition plus its remainder terms
    ///      (`focal_section_envelope`).
    ///    - [`Pcurve::Projected`]: the projected bound
    ///      ([`EnvelopeStatement::MapResidualProjected`]).
    ///
    ///    The spiric, focal-section and projected statements are 3-D and
    ///    cannot see a whole period of an angle channel, so each also
    ///    carries its **placement terms** (`placement_terms`, as
    ///    `FidelityU`/`FidelityV`): a lifted copy refuses there as on the
    ///    harmonic lane. Every lane decides its verdict once
    ///    (`envelope_verdict`).
    ///
    /// # Errors
    ///
    /// The first failing check, as a typed [`PcurveCertifyError`];
    /// [`PcurveCertifyError::ImageMismatch`] for a fitted or general
    /// image offered to this door.
    pub fn certify(
        pcurve: Pcurve<T>,
        t0: T,
        t1: T,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
    ) -> Result<Self, PcurveCertifyError> {
        let certificate = match &pcurve {
            Pcurve::Fitted(_) | Pcurve::General(_) => {
                return Err(PcurveCertifyError::ImageMismatch {
                    image: PcurveKind::of(&pcurve),
                    why: "a fitted-grade image certifies at certify_fitted or certify_general, \
                          never at the closed-form door",
                });
            }
            // The iso lane (M6-3): closed-form like the harmonic one —
            // no bracket obligation — so it shares this `Decide`-scalar
            // door.
            Pcurve::IsoLine { p0, pl } => run_iso_checks(*p0, *pl, t0, t1, carrier, surface, band)?,
            Pcurve::IsoArc {
                p0,
                pd,
                t0: at0,
                angle,
                breaks,
            } => run_iso_arc_checks(
                *p0, *pd, *at0, *angle, breaks, t0, t1, carrier, surface, band,
            )?,
            // EXHAUSTIVE by variant, never a catch-all (D3): a
            // catch-all here would route a NEW closed-form variant
            // into the harmonic checker silently, and the build would
            // not say a word. Adding a variant must be a
            // compiler-guided edit at every dispatch site.
            Pcurve::Harmonic { u, v } => {
                run_harmonic_checks((*u, *v), t0, t1, carrier, surface, band)?
            }
            // The spiric lane: closed-form like the harmonic and iso
            // ones — no bracket obligation — so it shares this
            // `Decide`-scalar door.
            spiric @ Pcurve::Spiric { .. } => {
                run_spiric_checks(spiric, t0, t1, carrier, surface, band)?
            }
            section @ Pcurve::FocalSection(_) => {
                run_focal_section_checks(section, t0, t1, carrier, surface, band)?
            }
            // A circle's projected row certifies here at every scalar; a
            // net's needs the fitted door, which this door does not
            // hold ([`PcurveCache::certify_projected`]).
            projected @ Pcurve::Projected(_) => {
                projected::run_projected_checks(projected, t0, t1, carrier, surface, band, None)?
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

impl<T: Decide> PcurveCache<T> {
    /// Certifies a **fitted** chart image: an SSI trace's chart
    /// projection over a rung-3 carrier on a SPLINE chart. No mint site
    /// derives one today; a caller that holds one (an attach path, the
    /// cut-loft unit's future producer) certifies it here, and the tier-3
    /// validator re-derives the certificate per half-edge through
    /// [`PcurveCache::recertify`].
    ///
    /// `lane` is the door itself, not an `Option`: only a scalar that
    /// may certify can hold one ([`crate::FittedLane`]), so this door
    /// cannot be called at a scalar that may not.
    ///
    /// The `General` curve-in-UV arm of the ratified pcurve unification
    /// (`docs/PCURVE-UNIFY-DESIGN.md` U2) certifies through
    /// [`PcurveCache::certify_general`] beside this door.
    ///
    /// Same four checks in the same fixed order as
    /// [`PcurveCache::certify`], with two differences that are the
    /// whole content of the lane: check 1 admits any carrier on a spline
    /// chart, and check 4 is the hull bound on `|S(P(t)) − C(t)|` over
    /// the whole span against this chart alone
    /// ([`EnvelopeStatement::MapResidualComposite`]). The row reads no
    /// mate surface: an intersection carrier's uniqueness tube is the
    /// edge's certificate (C2).
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
        band: Band,
        lane: crate::FittedLane<T>,
    ) -> Result<Self, PcurveCertifyError> {
        let certificate = run_fitted_checks(&image, t0, t1, carrier, surface, band, Some(lane))?;
        Ok(Self {
            pcurve: Pcurve::Fitted(image),
            param_start: t0,
            param_end: t1,
            certificate,
        })
    }

    /// Certifies a **general curve-in-UV** ([`Pcurve::General`], U2's
    /// arm) at the FITTED GRADE: the identical four checks in the
    /// identical order as [`PcurveCache::certify_fitted`], against the
    /// identical chart, producing the identical certificate.
    ///
    /// The two doors are separate because their ENTRY requirements
    /// differ, not their statements: `certify_fitted` is entered by a
    /// caller who can assert the OQ4 construction identity, this one
    /// by a caller who cannot. Nothing here is weaker as a
    /// consequence — the certificate is measured either way — so the
    /// outcomes are the fitted lane's verbatim:
    ///
    /// - **certify**: every sampled map residual is coincident with
    ///   zero and the envelope, the hull sup bound, is within ε;
    /// - **refuse**: [`PcurveCertifyError::FittedCertificate`] (a
    ///   definite limb failure) or [`PcurveCertifyError::
    ///   IntervalNotForward`];
    /// - **escalate**: [`PcurveCertifyError::Escalated`] /
    ///   [`PcurveCertifyError::FittedEscalated`] (a sliver-band
    ///   verdict);
    /// - **no door**: [`PcurveCertifyError::FittedLaneUnsupported`], at
    ///   check 4, when `lane` is `None` — the scalar may not certify,
    ///   or the caller withheld its door.
    ///
    /// `lane` is the scalar's fitted door (`topo::AtRestPolicy::fitted_lane`),
    /// or `None` where the scalar may not certify or the caller
    /// withholds it; the refusal names the scalar by
    /// [`geom_core::Real::NAME`]. This door takes the `Option` because
    /// its mint caller reaches it at every scalar: `topo::mint_pcurves`
    /// hands it a construction's STATED `General` image with no
    /// derivation in front of it. An absent door is check 4's refusal,
    /// in check 4's place, so an image that fails checks 1–3 draws the
    /// same verdict at every scalar, and no `General` cache is built
    /// without the door, since check 4 derives its certificate.
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
        band: Band,
        lane: Option<crate::FittedLane<T>>,
    ) -> Result<Self, PcurveCertifyError> {
        let certificate = run_fitted_checks(&image, t0, t1, carrier, surface, band, lane)?;
        Ok(Self {
            pcurve: Pcurve::General(image),
            param_start: t0,
            param_end: t1,
            certificate,
        })
    }

    /// Certifies a **projected** image ([`Pcurve::Projected`]) with the
    /// fitted door in hand: a spline carrier's row reads its hull terms
    /// through `lane` at check 4 (certification arithmetic, C9), and a
    /// circle's reads nothing from it. `None` is check 4's refusal for a
    /// net's row, [`PcurveCertifyError::FittedLaneUnsupported`], after
    /// checks 1–3 have run.
    ///
    /// # Errors
    ///
    /// The first failing check, as a typed [`PcurveCertifyError`];
    /// [`PcurveCertifyError::ImageMismatch`] for an image of another
    /// variant.
    pub fn certify_projected(
        image: ProjectedImage<T>,
        t0: T,
        t1: T,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
        lane: Option<crate::FittedLane<T>>,
    ) -> Result<Self, PcurveCertifyError> {
        let pcurve = Pcurve::Projected(Box::new(image));
        let certificate =
            projected::run_projected_checks(&pcurve, t0, t1, carrier, surface, band, lane)?;
        Ok(Self {
            pcurve,
            param_start: t0,
            param_end: t1,
            certificate,
        })
    }

    /// Re-runs the full certification at rest — the tier-3 validator's
    /// per-half-edge pass, for EITHER lane. Same checks, same schedule,
    /// same errors; the stored certificate is not consulted
    /// (re-certification re-derives, it does not trust — for a fitted
    /// cache, its hull bound).
    ///
    /// `lane` is the pass's fitted door (`topo::AtRestPolicy::fitted_lane`),
    /// or `None` where the scalar may not certify or the caller
    /// withholds it. Only the `Fitted | General` arm reads it, where
    /// [`PcurveCache::certify_general`] does: `None` is check 4's
    /// refusal, [`PcurveCertifyError::FittedLaneUnsupported`] naming
    /// the scalar by [`geom_core::Real::NAME`], after checks 1–3 have
    /// run.
    ///
    /// # Errors
    ///
    /// As [`PcurveCache::certify`] / [`PcurveCache::certify_fitted`],
    /// and [`PcurveCertifyError::FittedLaneUnsupported`] for a fitted
    /// cache with no door.
    pub fn recertify(
        &self,
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        band: Band,
        lane: Option<crate::FittedLane<T>>,
    ) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
        match &self.pcurve {
            Pcurve::Fitted(image) | Pcurve::General(image) => run_fitted_checks(
                image,
                self.param_start,
                self.param_end,
                carrier,
                surface,
                band,
                lane,
            ),
            Pcurve::IsoLine { p0, pl } => run_iso_checks(
                *p0,
                *pl,
                self.param_start,
                self.param_end,
                carrier,
                surface,
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
                band,
            ),
            spiric @ Pcurve::Spiric { .. } => run_spiric_checks(
                spiric,
                self.param_start,
                self.param_end,
                carrier,
                surface,
                band,
            ),
            section @ Pcurve::FocalSection(_) => run_focal_section_checks(
                section,
                self.param_start,
                self.param_end,
                carrier,
                surface,
                band,
            ),
            projected @ Pcurve::Projected(_) => projected::run_projected_checks(
                projected,
                self.param_start,
                self.param_end,
                carrier,
                surface,
                band,
                lane,
            ),
            // Exhaustive by variant (see `certify`).
            Pcurve::Harmonic { u, v } => run_harmonic_checks(
                (*u, *v),
                self.param_start,
                self.param_end,
                carrier,
                surface,
                band,
            ),
        }
    }
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

/// The amplitude of `form`'s motion off the cylinder axis `axis`, in
/// metres (its harmonic coefficients are displacements): zero for a
/// ruling, the radial circle's size for a carrier moving around it —
/// its `cos t`/`sin t` pair at its exact sup. A carrier this reads
/// under the escalation band but over the zero band takes the meridian;
/// one that turns on the chart has `‖a_r‖ ≈ R`, and its winding margin
/// `‖a_r × b_r‖/R ≤ R` is in the band too, so no carrier the moving
/// class could certify is read there.
fn radial_amplitude<T: Real>(form: &Harmonic3<T>, axis: Vec3<T>) -> T {
    let radial = |v: Vec3<T>| v - axis * v.dot(axis);
    let (a_r, b_r, l_r) = (radial(form.a), radial(form.b), radial(form.l));
    let amplitude = harmonic_sup(a_r, b_r) + l_r.norm();
    #[cfg(test)]
    let amplitude = mutants::radial_amplitude(a_r, b_r, l_r).unwrap_or(amplitude);
    amplitude
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
            semi_u,
            semi_v,
            u_ref,
        } => Some(Harmonic3 {
            c: center,
            a: u_ref * semi_u,
            b: axis.cross(u_ref) * semi_v,
            l: Vec3::zero(),
        }),
        // A spiric has no `{1, cos, sin, t}` form (its `m` channel is
        // `√((R + r cos v)² − d²)`), so every chart arm of
        // [`chart_pcurve`] refuses it here, typed: its exact chart
        // images are the data-free `Pcurve` variant of the spiric
        // unit's second PR, which precedes this gate when it lands.
        Curve3::Spiric { .. } | Curve3::Nurbs(_) => None,
    }
}

/// The side of a cone's apex a ruling or rim lies on, as a cone
/// derivation decides it on the height. A sign, not a winding.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Nappe {
    /// Above the apex, along the axis.
    Pos,
    /// Below it.
    Neg,
}

impl Nappe {
    /// The side as the unit sign it multiplies a direction by.
    fn value<T: Real>(self) -> T {
        match self {
            Nappe::Pos => T::one(),
            Nappe::Neg => T::zero() - T::one(),
        }
    }
}

/// **The windings a closed-form table derives**: `−1`, `0` or `1` whole
/// periods of the chart's angle per carrier period. Its one home: a
/// decided orientation names a count in it ([`winding_of`]), and a
/// stored count is read against it ([`angle_windings`]).
const WINDING_TABLE: core::ops::RangeInclusive<i32> = -1..=1;

/// The winding a decided orientation names, sign for sign, in
/// [`WINDING_TABLE`]: the one place a sign becomes a count. A count
/// reversed is the reversed sign's (`Sign::flip`), or, on a stored
/// channel, [`Channel::mapped`]'s.
fn winding_of(sign: Sign) -> i32 {
    match sign {
        Sign::Negative => -1,
        Sign::Zero => 0,
        Sign::Positive => 1,
    }
}

/// The windings a harmonic image on a periodic chart stores, read off
/// its channels by type: the azimuth's always, the second angle's where
/// the chart has one. `None` where the first channel is not an angle
/// (the image is no periodic chart's: a plane's channels are free and
/// carry no windings), or where a count winds more than once per
/// carrier period, which no closed-form table holds. Structure, read
/// with no decision.
fn angle_windings<T: Real>(u: Channel<T>, v: Channel<T>) -> Option<ChartWindings> {
    let in_table = |w: i32| WINDING_TABLE.contains(&w).then_some(w);
    Some(ChartWindings {
        u: in_table(u.winding()?)?,
        v: match v.winding() {
            None => None,
            Some(w) => Some(in_table(w)?),
        },
    })
}

/// A plane chart's image of a harmonic form: the map is affine, so the
/// coefficients map through one by one. Total.
fn plane_image<T: Real>(
    (p0, pa, pb, pl): HarmonicForm<T>,
    origin: Point3<T>,
    normal: Vec3<T>,
    u_ref: Vec3<T>,
) -> Harmonic3<T> {
    let v_ref = normal.cross(u_ref);
    Harmonic3 {
        c: origin + (u_ref * p0.x + v_ref * p0.y),
        a: u_ref * pa.x + v_ref * pa.y,
        b: u_ref * pb.x + v_ref * pb.y,
        l: u_ref * pl.x + v_ref * pl.y,
    }
}

/// The chart image `S ∘ P` in the certified basis. The rows' own
/// reference (certification reads the plane's arm alone,
/// [`plane_image`], and every periodic chart's through check 4's lemma).
///
/// - **Plane chart**: the map is affine, so the pcurve's coefficients
///   map through one by one.
/// - **Cylinder chart**: the azimuth channel is `α + β·t` with
///   `β ∈ {−1, 0, +1}` (the stored windings, [`Channel::Angle`],
///   [`angle_windings`]). For `β = 0` the
///   radial vector is constant and folds into the constant term; for
///   `β = ±1` the angle-sum identity
///   `cos(α + βt) = cos α·cos t − β·sin α·sin t` puts the radial
///   channel exactly in the basis.
#[cfg(test)]
fn chart_image_harmonic<T: Real>(pcurve: &Pcurve<T>, surface: &Surface<T>) -> Option<Harmonic3<T>> {
    // The closed-form image is the closed-form lane's business; a
    // fitted image has no harmonic decomposition by construction.
    let Pcurve::Harmonic { u, v } = *pcurve else {
        return None;
    };
    let (p0, pa, pb, pl) = harmonic_form_of(u, v);
    // A periodic chart reads the image's windings by type; a plane's
    // channels carry none.
    let windings = || angle_windings(u, v);
    match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => Some(plane_image((p0, pa, pb, pl), origin, normal, u_ref)),
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
            match windings()?.u {
                0 => Some(Harmonic3 {
                    c: origin + rad * radius + axis * p0.y,
                    a: axis * pa.y,
                    b: axis * pb.y,
                    l: axis * pl.y,
                }),
                winding => Some(Harmonic3 {
                    c: origin + axis * p0.y,
                    a: rad * radius + axis * pa.y,
                    b: tang * (radius * angle_rate::<T>(winding)) + axis * pb.y,
                    l: axis * pl.y,
                }),
            }
        }
        // The M6-3 completion (walk row 4): the cone/sphere/torus
        // closed-form tables. Every arm below is the SNAPPED image —
        // affine angular channels with winding-valued slopes.
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
            match windings()?.u {
                // Fixed azimuth: S is AFFINE in v along the ruling
                // direction, so any harmonic v channel maps through
                // exactly, coefficient by coefficient (the slant-line
                // meridian class, `v` a length).
                0 => {
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
                winding => {
                    let tang = u_ref * (T::zero() - sa) + cv * ca;
                    let rho = p0.y * s_ha;
                    Some(Harmonic3 {
                        c: apex + axis * (p0.y * c_ha),
                        a: rad * rho,
                        b: tang * (rho * angle_rate::<T>(winding)),
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
            let windings = windings()?;
            match (windings.u, windings.v.unwrap_or(0)) {
                // Moving azimuth, constant polar v₀: the PARALLEL /
                // polar-circle class — a circle of radius
                // R + r·cos v₀ (sphere: r·cos v₀) at height r·sin v₀.
                (winding @ (-1 | 1), 0) => {
                    let tang = u_ref * (T::zero() - sa) + cv * ca;
                    let rho = major + radius * cd;
                    Some(Harmonic3 {
                        c: center + axis * (radius * sd),
                        a: rad * rho,
                        b: tang * (rho * angle_rate::<T>(winding)),
                        l: Vec3::zero(),
                    })
                }
                // Constant azimuth, moving polar (σ = ±1): the
                // MERIDIAN class — cos(δ+σt)/sin(δ+σt) expanded by the
                // angle-sum identity, exactly as the cylinder's
                // azimuth arm.
                (0, sigma @ (-1 | 1)) => {
                    let s = angle_rate::<T>(sigma);
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
                (0, 0) => Some(Harmonic3 {
                    c: center + rad * (major + radius * cd) + axis * (radius * sd),
                    a: Vec3::zero(),
                    b: Vec3::zero(),
                    l: Vec3::zero(),
                }),
                // Both channels moving: azimuth-NON-harmonic (the
                // sphere's general circles, the torus Villarceau
                // class) — no closed form exists; the fitted lane or a
                // typed refusal owns it.
                // (`angle_windings` holds every count in `{−1, 0, 1}`.)
                _ => None,
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
    u: i32,
    v: Option<i32>,
}

/// The azimuth lever arm of a chart, in metres per radian — the safe
/// OVER-statement per kind (module docs' sphere note), and the one home
/// of [`chart_stretch_sup`]'s azimuth column: cylinder `r`, sphere `r`
/// (true arm `r·cos v ≤ r`), torus `R + r` (the outer equator's), cone
/// `v_sup·sin α` — the cone has no surface-level constant, so the
/// caller supplies the pcurve box's own `|v|` sup, which dominates the
/// local arm `v·sin α` everywhere the object being metered lives.
/// `v_sup` is read on the cone only.
fn azimuth_lever<T: Real>(surface: &Surface<T>, v_sup: T) -> T {
    match *surface {
        Surface::Cylinder { radius, .. } | Surface::Sphere { radius, .. } => radius,
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => major_radius + minor_radius,
        Surface::Cone { half_angle, .. } => v_sup * half_angle.sin(),
        // Non-periodic charts have no azimuth, so there is no lever to
        // answer — not even 1. Every caller asks on a periodic chart
        // only: `chart_sup_arms_at` on the cone arm, and the harmonic lane's
        // fidelity term and the focal and spiric lanes behind their own
        // non-periodic exclusion.
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => unreachable!(
            "azimuth_lever: a {} chart has no azimuth, and every caller asks on a \
             periodic chart only",
            surface.kind().name()
        ),
    }
}

/// The inf of [`azimuth_lever`] over a chart window whose second
/// parameter spans `[v_lo, v_hi]`, the arm's under-statement per kind:
/// cylinder `r` (exact); sphere `r·cos v` at the window's largest `|v|`
/// (`v` is a latitude, so `cos` decreases in `|v|` up to the pole, where
/// it reads zero); torus `R − r` (`R + r·cos v ≥ R − r` at every `v`);
/// cone `v_inf·sin α` with `v_inf` the window's least `|v|`, zero where
/// the window straddles the apex. Never negative: a spindle torus, or a
/// window on the pole, reads zero.
fn azimuth_lever_inf<T: Real>(surface: &Surface<T>, v_lo: T, v_hi: T) -> InfSpeed<T> {
    let arm = match *surface {
        Surface::Cylinder { radius, .. } => radius,
        Surface::Sphere { radius, .. } => {
            let half_turn = T::pi() * T::from_f64(0.5);
            radius * v_lo.abs().max(v_hi.abs()).min(half_turn).cos()
        }
        Surface::Torus {
            major_radius,
            minor_radius,
            ..
        } => major_radius - minor_radius,
        Surface::Cone { half_angle, .. } => {
            v_lo.max(T::zero() - v_hi).max(T::zero()) * half_angle.sin()
        }
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => unreachable!(
            "azimuth_lever_inf: a {} chart has no azimuth, and every caller asks on a \
             periodic chart only",
            surface.kind().name()
        ),
    };
    InfSpeed::new(arm.max(T::zero()))
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
fn param_rate<T: SpanLocate>(carrier: &Curve3<T>) -> InfSpeed<T> {
    match *carrier {
        Curve3::Line { .. } => InfSpeed::new(T::one()),
        Curve3::Nurbs(ref n) => n.speed_lower_bound(),
        Curve3::Circle { radius, .. } => InfSpeed::new(radius),
        // The smaller semi-axis MAGNITUDE: the semi-axes carry no order
        // and no sign (`Conic`); `minor` for the ordinary order.
        Curve3::Ellipse { semi_u, semi_v, .. } => {
            InfSpeed::new(geom::Curve3::semi_axes_by_size(semi_u, semi_v).1)
        }
        Curve3::Spiric { minor_radius, .. } => InfSpeed::new(minor_radius),
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
/// [`Indeterminate`] under `pcurve_interval_meter`: carrying the
/// decided margin when the subtended length is decided non-positive,
/// and the classifier's own escalation when it is in band or poison.
fn param_rate_gate<T: Decide>(
    carrier: &Curve3<T>,
    band: Band,
) -> Result<InfSpeed<T>, Indeterminate> {
    let rate = param_rate(carrier);
    let extent = match *carrier {
        Curve3::Nurbs(ref n) => {
            let (d0, d1) = n.knot_values().domain();
            d1 - d0
        }
        _ => T::one(),
    };
    geom_core::k_stats::decide_positive(
        "pcurve_interval_meter",
        Margin::metered(extent, rate),
        band,
    )?;
    Ok(rate)
}

/// How a residual's decision is recorded: as a verdict of the
/// certified statement ([`decide`]), or as check 4's cross-check on a
/// harmonic row, which refuses as a verdict would and records none
/// (`k_stats::check_unlogged`, audit row F20: a point build would
/// otherwise log rows a box build cannot, and the driver compares the
/// two row for row).
#[derive(Clone, Copy)]
enum Record {
    Verdict,
    CrossCheck,
}

/// Folds a residual into the running max and classifies it against the
/// band (the `certify::check_residual` idiom, one module over), recorded
/// as `record` says.
fn check_residual_as<T: Decide>(
    record: Record,
    name: &'static str,
    check: PcurveCheck,
    sample: u32,
    residual: Margin<T>,
    band: Band,
    max_residual: &mut T,
) -> Result<(), PcurveCertifyError> {
    *max_residual = max_residual.max(residual.value().abs());
    let decided = match record {
        Record::Verdict => decide(name, residual, band),
        Record::CrossCheck => {
            geom_core::k_stats::check_unlogged(name, residual.value(), band, "F20")
        }
    };
    match decided {
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
    (u, v): (Channel<T>, Channel<T>),
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    let pcurve = &Pcurve::Harmonic { u, v };
    // ---- Check 1: the certified lane. ----
    let chart = surface.kind();
    // The closed-form lane is the analytic charts'; a spline chart —
    // the payload's or an approximating surface's fit — goes through
    // the fitted lane instead.
    if !crate::edge_nurbs::is_analytic(surface) {
        return Err(PcurveCertifyError::UnsupportedChart { chart });
    }
    // The channels are the chart's: an angle where the chart's
    // coordinate is one, a free channel where it is not. Structure,
    // compared with no decision.
    let angle = |c: Channel<T>| c.winding().is_some();
    let Some((period_u, period_v)) = angle_periods(surface) else {
        // A spline chart has no closed-form lane (refused above too).
        return Err(PcurveCertifyError::UnsupportedChart { chart });
    };
    if (angle(u), angle(v)) != (period_u.is_some(), period_v.is_some()) {
        return Err(PcurveCertifyError::ChannelKindMismatch { chart });
    }
    // `chart_pcurve` mints a harmonic image only from a carrier's own
    // harmonic form, so a carrier without one (a spline, a spiric) was
    // handed an image it never had.
    let Some(carrier_form) = carrier_harmonic(carrier) else {
        return Err(PcurveCertifyError::ImageMismatch {
            image: PcurveKind::Harmonic,
            why: "the carrier has no {1, cos, sin, t} form, so no harmonic image is its own",
        });
    };
    let reach = t0.abs().max(t1.abs());
    // The windings the row stores, on a periodic chart (a plane's
    // channels carry none); a count no closed-form table holds (more
    // than one turn per carrier period) has no image here.
    if period_u.is_some() {
        let windings = angle_windings(u, v).ok_or(PcurveCertifyError::ChartWindingUnsupported)?;
        // Both angular channels moving is the sphere's general circle
        // and the torus's Villarceau class: azimuth-non-harmonic, so no
        // harmonic image exists; the projected image owns the first and
        // the focal section the second.
        if windings.u != 0 && windings.v.is_some_and(|w| w != 0) {
            return Err(PcurveCertifyError::UnsupportedChart { chart });
        }
    }

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
    // ---- Check 3: the schedule, the closed form's cross-check. ----
    // Check 4's envelope is the whole certified statement of a harmonic
    // row (C4); the schedule cross-checks its tables on the witness lane
    // (`Witness::Inexact`), and is not run at an exact-witness scalar.
    let samples = match T::WITNESS {
        geom_core::Witness::Inexact => {
            schedule_residuals(
                Record::CrossCheck,
                pcurve,
                t0,
                t1,
                carrier,
                surface,
                band,
                &mut max_residual,
            )?;
            CERT_SAMPLES
        }
        geom_core::Witness::Exact => 0,
    };

    // ---- Check 4: the closed-form between-samples envelope. ----
    // A plane chart is affine, so the image's coefficients map through
    // and their differences from the carrier's are the bound. A
    // periodic chart's is frame plus incidence plus fidelity (the lemma
    // on `EnvelopeStatement::MapResidualClosedForm`).
    let (envelope, terms) = match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let image_form = plane_image(harmonic_form_of(u, v), origin, normal, u_ref);
            let d_c = image_form.c - carrier_form.c;
            let d_a = image_form.a - carrier_form.a;
            let d_b = image_form.b - carrier_form.b;
            let d_l = image_form.l - carrier_form.l;
            (
                d_c.norm() + harmonic_sup(d_a, d_b) + d_l.norm() * reach,
                None,
            )
        }
        _ => {
            let terms = periodic_envelope(
                (u, v),
                carrier,
                carrier_form,
                surface,
                (t0, t1),
                reach,
                band,
            )?;
            (terms.total(), Some(terms))
        }
    };
    // Not folded into `max_residual`: that field is the sampled max,
    // and the two statements stay separate (the certificate's field
    // docs).
    envelope_verdict(envelope, band, || {
        terms
            .and_then(|terms| terms.first_over(band))
            .map_or(PcurveCheck::Envelope, PcurveCheck::EnvelopeTerm)
    })?;

    Ok(PcurveCertificate::closed(
        samples,
        max_residual,
        envelope,
        EnvelopeStatement::MapResidualClosedForm,
    ))
}

/// What [`derive_harmonic`] selected beside the image it built: the
/// class arm and its decided windings. Check 4 states incidence for
/// this structure, so the structure is the derivation's own and is
/// never decided a second time.
#[derive(Clone, Copy, Debug)]
enum Derivation {
    Plane,
    CylinderMoving {
        beta: i32,
    },
    CylinderMeridian,
    /// `anchored`: the nappe was read off the anchor's height, not the
    /// direction's. `nappe` is the side of the apex it lies on.
    ConeRuling {
        anchored: bool,
        nappe: Nappe,
    },
    ConeRim {
        nappe: Nappe,
        beta: i32,
    },
    SphereParallel {
        beta: i32,
    },
    /// `pole`: the azimuth was read off `b`'s radial part, `a`'s having
    /// decided Zero. `far`: the edge lies on the far half of its great
    /// circle from that direction, so the image reads the azimuth off
    /// its negation (the span's midpoint decides the half). `sigma`: the
    /// image's polar rate.
    SphereMeridian {
        pole: bool,
        sigma: i32,
        far: bool,
    },
    TorusParallel {
        beta: i32,
    },
    TorusMeridian {
        sigma: i32,
    },
    /// An ellipse on a cone or a Villarceau circle on a torus: the
    /// image is a [`Pcurve::FocalSection`], which its own lane
    /// certifies, never a harmonic one.
    FocalSection,
}

/// How [`EnvelopeTerms::total`] reads the two channel fidelities.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FidelityLegs {
    /// `FidelityU` and `FidelityV` are `arm·sup|Δ|` of one chart
    /// segment's two channels on a chart whose `S_u ⊥ S_v` everywhere
    /// (the cylinder, the sphere, the torus): the segment's image is no
    /// longer than `√(arm_u²·Δu² + arm_v²·Δv²)`, so they are read as the
    /// two legs of one displacement.
    Orthogonal,
    /// They add: the cone's two legs are not orthogonal (`fidelity`),
    /// and the projected envelope's `FidelityV` carries more than its
    /// channel.
    Summed,
}

impl FidelityLegs {
    /// The legs of the closed-form lane's fidelity on `surface`, by its
    /// kind alone.
    fn of<T: Real>(surface: &Surface<T>) -> Self {
        let legs = match surface {
            Surface::Cylinder { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => {
                Self::Orthogonal
            }
            _ => Self::Summed,
        };
        #[cfg(test)]
        let legs = mutants::fidelity_legs(surface, legs);
        legs
    }
}

/// Check 4's envelope on a periodic chart, one running sum per
/// [`EnvelopeTerm`] and how the two channel fidelities combine; the
/// certified bound is [`EnvelopeTerms::total`].
#[derive(Clone, Copy)]
struct EnvelopeTerms<T: Real>([T; EnvelopeTerm::ALL.len()], FidelityLegs);

/// Which name of the placed image a stored row is: the derivation's own,
/// or on a sphere its involution twin `(u + π, π − v)`. The projected
/// lane decides it once, as structure (`pcurve_projected_v_sign`), and
/// returns it beside its terms; [`placement_terms`] takes it from there.
/// The focal-section and spiric lanes live on no sphere and name the
/// derivation's own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ImageName {
    /// The derivation's own name.
    Derived,
    /// The sphere's involution twin of it.
    Twin,
}

impl<T: Decide> EnvelopeTerms<T> {
    fn new() -> Self {
        Self([T::zero(); EnvelopeTerm::ALL.len()], FidelityLegs::Summed)
    }

    fn add(&mut self, term: EnvelopeTerm, value: T) {
        let slot = &mut self.0[term.slot()];
        *slot = *slot + value;
    }

    /// The certified bound: every slot added, except that
    /// [`FidelityLegs::Orthogonal`] reads the two channel fidelities as
    /// the legs of one displacement, `√(FidelityU² + FidelityV²)`.
    fn total(&self) -> T {
        let (u, v) = (
            EnvelopeTerm::FidelityU.slot(),
            EnvelopeTerm::FidelityV.slot(),
        );
        let legs = self.1 == FidelityLegs::Orthogonal;
        let rest = self
            .0
            .iter()
            .enumerate()
            .filter(|&(slot, _)| !legs || (slot != u && slot != v))
            .fold(T::zero(), |sum, (_, &term)| sum + term);
        let total = if legs {
            rest + harmonic_sup_scalar(self.0[u], self.0[v])
        } else {
            rest
        };
        #[cfg(test)]
        let total = mutants::fidelity(&self.0).unwrap_or(total);
        total
    }

    /// The first term, in [`EnvelopeTerm::ALL`]'s order, that does not
    /// decide Zero on its own — what a refused envelope names. `None`
    /// when every term fits the band alone and only their sum does not.
    fn first_over(&self, band: Band) -> Option<EnvelopeTerm> {
        EnvelopeTerm::ALL.into_iter().find(|&term| {
            !matches!(
                decide(
                    "pcurve_envelope_term",
                    Margin::of(self.0[term.slot()]),
                    band
                ),
                Ok(Sign::Zero)
            )
        })
    }
}

/// Check 4 on a periodic chart: the carrier's incidence under the
/// structure [`derive_harmonic`] selects for it, plus the stored
/// image's fidelity to the image it derives.
fn periodic_envelope<T: Decide>(
    stored: (Channel<T>, Channel<T>),
    carrier: &Curve3<T>,
    carrier_form: Harmonic3<T>,
    surface: &Surface<T>,
    span: (T, T),
    reach: T,
    band: Band,
) -> Result<EnvelopeTerms<T>, PcurveCertifyError> {
    // The lemma's premise is an orthonormal frame. The stored chart's
    // need not be one over a box (or at all), so the lemma is applied
    // on the chart's orthonormalised twin and the two maps' distance
    // over the stored image is its own term (`Frame`).
    let ideal = orthonormal_chart(surface);
    let (derived, derivation) = derive_harmonic(
        carrier,
        &ideal,
        Placement::over(span, carrier, &ideal),
        band,
    )?;
    // The derivation's image is harmonic on every arm but the focal
    // section's (`Derivation::FocalSection`), which images as its own
    // variant.
    let Pcurve::Harmonic { u: du, v: dv } = derived else {
        return Err(PcurveCertifyError::ImageMismatch {
            image: PcurveKind::Harmonic,
            why: "an ellipse on a cone or a Villarceau circle on a torus images as a focal \
                  section, not a harmonic",
        });
    };
    let mut terms = EnvelopeTerms::new();
    let (p0, pa, pb, pl) = harmonic_form_of(stored.0, stored.1);
    let v_reach = harmonic_span_box(p0, pa, pb, pl, span.0, span.1).v_reach();
    terms.add(EnvelopeTerm::Frame, frame_defect(surface, v_reach));
    incidence(carrier_form, &ideal, derivation, reach, &mut terms);
    fidelity(stored, (du, dv), &ideal, span, reach, band, &mut terms)?;
    Ok(terms)
}

/// The chart with its frame made orthonormal ([`geom::square_frame`]):
/// `n̂ = axis/‖axis‖`, and `ê₁` the part of `u_ref` normal to `n̂`,
/// normalised (the map's `v_ref` is then `n̂ × ê₁`). In exact
/// arithmetic it is the frame the lemma on
/// [`EnvelopeStatement::MapResidualClosedForm`] assumes; a literal
/// unit frame is its own twin, and its twin's forms reduce to its own.
pub(crate) fn orthonormal_chart<T: Real>(surface: &Surface<T>) -> Surface<T> {
    let frame = |axis: Vec3<T>, u_ref: Vec3<T>| {
        let f = geom::square_frame(axis, u_ref);
        (f.axis, f.u_ref)
    };
    match *surface {
        Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => {
            let (axis, u_ref) = frame(axis, u_ref);
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            }
        }
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let (axis, u_ref) = frame(axis, u_ref);
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            }
        }
        Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => {
            let (axis, u_ref) = frame(axis, u_ref);
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            }
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let (axis, u_ref) = frame(axis, u_ref);
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            }
        }
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let (normal, u_ref) = frame(normal, u_ref);
            Surface::Plane {
                origin,
                normal,
                u_ref,
            }
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            unreachable!("orthonormal_chart: a spline chart has no frame to make orthonormal")
        }
    }
}

/// `sup |S(u, v) − Ŝ(u, v)|` over an image whose `|v|` stays within
/// `v_reach`, `Ŝ` the chart's orthonormal twin ([`orthonormal_chart`]).
/// Both maps share the chart's scalars and differ only in their frame:
/// the radial direction moves by at most `‖u_ref − ê₁‖ + ‖v_ref − ê₂‖`
/// at any azimuth (`ρ(u) − ρ̂(u)` is that pair's difference under
/// `cos u`, `sin u`), and the axis by `‖axis − n̂‖`. Each is levered by
/// what multiplies it in the chart's own map: the radius, or the
/// slant `v` on a cylinder's axis and a cone's two channels — the
/// actual axial reach, not the radius.
///
/// The distances are bounded in the frame's own invariants, never by
/// differencing the normalised vectors (whose enclosures widen by the
/// radius times a few ulps at the interval scalar — the wide-arc
/// widening check 4 was restated to remove). With `A = axis·axis`,
/// `U = u_ref·u_ref` and `θ = |axis·u_ref|/√A = |n̂·u_ref|`, and since
/// `|x − 1| ≤ |x² − 1|` for `x ≥ 0`:
///
/// - `‖axis − n̂‖ = |√A − 1| ≤ |A − 1|`;
/// - `‖u_ref − ê₁‖ ≤ ‖u_ref − u⊥‖ + ‖u⊥ − ê₁‖ ≤ θ + |U − θ² − 1| ≤
///   θ + |U − 1| + θ²`, `u⊥ = u_ref − n̂·θ` its part normal to `n̂`;
/// - `‖axis × u_ref − n̂ × ê₁‖ ≤ ‖axis − n̂‖·√U + ‖u_ref − ê₁‖`.
///
/// On a literal unit frame every one is the zero form.
fn frame_defect<T: Real>(surface: &Surface<T>, v_reach: T) -> T {
    let defect = |axis: Vec3<T>, u_ref: Vec3<T>| {
        let (a2, u2) = (axis.dot(axis), u_ref.dot(u_ref));
        let one = T::one();
        let d_axis = (a2 - one).abs();
        let tilt = axis.dot(u_ref).abs() / a2.sqrt();
        let d_u = tilt + (u2 - one).abs() + tilt.powi(2);
        (d_axis, d_u + d_u + d_axis * u2.sqrt())
    };
    match *surface {
        Surface::Cylinder {
            axis,
            radius,
            u_ref,
            ..
        } => {
            let (d_axis, d_radial) = defect(axis, u_ref);
            d_radial * radius + d_axis * v_reach
        }
        Surface::Cone {
            axis,
            half_angle,
            u_ref,
            ..
        } => {
            let (s_a, c_a) = half_angle.sin_cos();
            let (d_axis, d_radial) = defect(axis, u_ref);
            (d_axis * c_a.abs() + d_radial * s_a.abs()) * v_reach
        }
        // `r·(cos v·Δrad + sin v·Δaxis)`: `(cos v, sin v)` is a unit
        // pair, so the two defects read at their norm (Cauchy–Schwarz).
        Surface::Sphere {
            radius,
            axis,
            u_ref,
            ..
        } => {
            let (d_axis, d_radial) = defect(axis, u_ref);
            let defect = harmonic_sup_scalar(d_radial, d_axis);
            #[cfg(test)]
            let defect = mutants::frame(d_radial, d_axis).unwrap_or(defect);
            defect * radius
        }
        Surface::Torus {
            axis,
            major_radius,
            minor_radius,
            u_ref,
            ..
        } => {
            // `(R + r·cos v)·Δrad + r·sin v·Δaxis`: the tube's share is
            // the sphere's, `r` levering the two defects at their norm.
            let (d_axis, d_radial) = defect(axis, u_ref);
            let tube = harmonic_sup_scalar(d_radial, d_axis);
            #[cfg(test)]
            let tube = mutants::frame(d_radial, d_axis).unwrap_or(tube);
            d_radial * major_radius + tube * minor_radius
        }
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => {
            unreachable!("frame_defect: check 4's periodic arm reads an analytic periodic chart")
        }
    }
}

/// The unit generator `cos α·e + sin α·q̂` of a cone of half-angle
/// `α` about `e`, at the azimuth of the radial `q`: the chart's
/// `S(u, v) = apex + v·ĝ` along the ruling at `rad(u) = q̂`.
fn ruling_generator<T: Real>(axis: Vec3<T>, half_angle: T, q: Vec3<T>) -> Vec3<T> {
    let (s_ha, c_ha) = half_angle.sin_cos();
    axis * c_ha + q * (s_ha / q.norm())
}

/// A rim's slant: the projection `h·cos α + n·ρ·sin α` of a point at
/// height `h` and radius `ρ` on the generator through it, `n` the
/// nappe. On the cone it is the point's `v`; off it, the image misses
/// the rim by its distance from the cone.
fn rim_slant<T: Real>(h: T, radius: T, half_angle: T, nappe: Nappe) -> T {
    let (s_ha, c_ha) = half_angle.sin_cos();
    h * c_ha + radius * s_ha * nappe.value()
}

/// `|p − ‖q‖|`, spelled `|p² − q·q| / (p + ‖q‖)` so the numerator is a
/// polynomial in the carrier's and the chart's data and `√(p²) = p` is
/// never asked of the arithmetic. `p_sq` is `p²` in the caller's own
/// spelling; `p ≥ 0`.
fn norm_gap<T: Real>(p: T, p_sq: T, q: Vec3<T>) -> T {
    (p_sq - q.dot(q)).abs() / (p + q.norm())
}

/// `p/‖q‖ − 1`, the factor that stretches `q` to length `p` (`q·(1 + k)`
/// has norm `p`), its numerator [`norm_gap`]'s polynomial. `p ≥ 0`,
/// `q ≠ 0`.
fn stretch<T: Real>(p: T, p_sq: T, q: Vec3<T>) -> T {
    let n = q.norm();
    (p_sq - q.dot(q)) / ((p + n) * n)
}

/// `sup_t ‖c + a·cos t + b·sin t‖`, bounded with no orthogonality
/// assumed: `‖c + H(t)‖² = ‖c‖² + ‖H(t)‖² + 2·c·H(t)`, and `c·H(t)` is
/// the scalar pair `(c·a, c·b)`, so the sup is at most `√(‖c‖² + σ(a,
/// b)² + 2·σ(c·a, c·b))`. That is `√(‖c‖² + σ²)` where `c ⊥ a, b` and
/// never over `‖c‖ + σ`, its value at the worst alignment.
fn constant_and_pair<T: Real>(c: Vec3<T>, a: Vec3<T>, b: Vec3<T>) -> T {
    let two = T::one() + T::one();
    let cross = harmonic_sup_scalar(c.dot(a), c.dot(b));
    (c.norm_squared() + harmonic_sup(a, b).powi(2) + two * cross).sqrt()
}

/// A rim's or a parallel's **class circle** against the carrier's
/// `(a, b)`: the image turns `ρ·â_r` at the decided winding, so its
/// `cos t` and `sin t` coefficients are `a_r·(1 + k)` and
/// `β·e×a_r·(1 + k)` with `k` the [`stretch`] to `ρ`, and the pair's
/// difference from the carrier's is read whole — radius, winding and
/// the plane's tilt at one exact sup. Each coefficient's difference is
/// spelled as the radius defect `q·k` plus the unstretched defect `q −
/// (the carrier's)`, so neither repeats the carrier's coefficient and a
/// bracketed scalar pays each part's own width. `axial` is false where
/// the chart's axial channel is the carrier's own and cancels (the
/// cylinder, whose `(a, b)` arrive radial). `lift`, where the image
/// stands off the carrier's plane by a constant `axis·lift`, is read
/// with the pair at [`constant_and_pair`]'s sup: the two are legs of one
/// displacement where the pair is radial. A winding decided Zero reads
/// the image's constant radial at its full length and the carrier's
/// pair as drift, the lift beside them.
fn class_circle<T: Decide>(
    terms: &mut EnvelopeTerms<T>,
    beta: i32,
    (rho, rho_sq): (T, T),
    (a, b): (Vec3<T>, Vec3<T>),
    axis: Vec3<T>,
    axial: bool,
    lift: Option<T>,
) {
    let (a_r, b_r, aa, bb) = if axial {
        let (aa, bb) = (a.dot(axis), b.dot(axis));
        (a - axis * aa, b - axis * bb, aa, bb)
    } else {
        (a, b, T::zero(), T::zero())
    };
    match beta {
        0 => {
            terms.add(EnvelopeTerm::Radius, rho + lift.map_or(T::zero(), T::abs));
            let drift = harmonic_sup(a, b);
            #[cfg(test)]
            let drift = mutants::zero_winding(a_r, b_r, (aa, bb)).unwrap_or(drift);
            terms.add(EnvelopeTerm::Drift, drift);
        }
        _ => {
            let k = stretch(rho, rho_sq, a_r);
            let turn = axis.cross(a_r) * angle_rate::<T>(beta);
            let (cos_part, sin_part) = (a_r * k - axis * aa, turn * k + (turn - b_r) - axis * bb);
            let pair = harmonic_sup(cos_part, sin_part);
            #[cfg(test)]
            let pair =
                mutants::class_circle((rho, rho_sq), a_r, b_r, turn, (aa, bb)).unwrap_or(pair);
            let pair = match lift {
                Some(lift) => {
                    let c = axis * lift;
                    let two = T::one() + T::one();
                    let cross = harmonic_sup_scalar(c.dot(cos_part), c.dot(sin_part));
                    (c.norm_squared() + pair.powi(2) + two * cross).sqrt()
                }
                None => pair,
            };
            terms.add(EnvelopeTerm::Radius, pair);
        }
    }
}

/// Check 4's **incidence** terms: a bound on `sup |S(P_d(t)) − C(t)|`
/// for the image `P_d` that [`derive_harmonic`] builds from `form`
/// under `derivation`, in the carrier's own coefficients and the
/// chart's data alone — no image angle enters it. The lemma per arm
/// is on [`EnvelopeStatement::MapResidualClosedForm`].
fn incidence<T: Decide>(
    form: Harmonic3<T>,
    surface: &Surface<T>,
    derivation: Derivation,
    reach: T,
    terms: &mut EnvelopeTerms<T>,
) {
    use EnvelopeTerm::{Centre, Drift, Line, Radius};
    let Harmonic3 { c, a, b, l } = form;
    match (surface, derivation) {
        (
            &Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            },
            Derivation::CylinderMoving { .. } | Derivation::CylinderMeridian,
        ) => {
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let (w_r, a_r, b_r) = (radial(c - origin), radial(a), radial(b));
            terms.add(Line, radial(l).norm() * reach);
            match derivation {
                Derivation::CylinderMoving { beta } => {
                    terms.add(Centre, w_r.norm());
                    class_circle(
                        terms,
                        beta,
                        (radius, radius.powi(2)),
                        (a_r, b_r),
                        axis,
                        false,
                        None,
                    );
                }
                _ => {
                    terms.add(Radius, norm_gap(radius, radius.powi(2), w_r));
                    terms.add(Drift, harmonic_sup(a_r, b_r));
                }
            }
        }
        (
            &Surface::Cone {
                apex,
                axis,
                half_angle,
                ..
            },
            Derivation::ConeRuling { .. } | Derivation::ConeRim { .. },
        ) => {
            let (s_ha, c_ha) = half_angle.sin_cos();
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let w = c - apex;
            let (h, w_r) = (w.dot(axis), radial(w));
            match derivation {
                Derivation::ConeRuling { anchored, nappe } => {
                    // The image is `apex + ((w + l·t)·ĝ)·ĝ`, so its miss
                    // is the anchor's and the direction's parts off the
                    // generator, each read in the frame `(ĝ, N̂, e × q̂)`
                    // with `N̂ = cos α·q̂ − sin α·e`: a datum `h·e + p·q̂
                    // + m·(e × q̂)` is `√((p·cos α − h·sin α)² + m²)` off
                    // it, its parts radial-sized rather than its whole
                    // length. The reference datum has `m = 0` and `p =
                    // n·‖q‖`, `n·h = |h|`, so its own is the polynomial
                    // gap `||h|·sin α − ‖q‖·cos α|`.
                    let datum = |x: Vec3<T>| (x.dot(axis), radial(x));
                    let (anchor, direction) = (datum(w), datum(l));
                    let ((h_ref, r_ref), other) = if anchored {
                        (anchor, direction)
                    } else {
                        (direction, anchor)
                    };
                    let rise = h_ref * nappe.value() * s_ha;
                    let own = norm_gap(rise, rise.powi(2), r_ref * c_ha);
                    let q = r_ref * nappe.value();
                    let q_len = q.norm();
                    let (h_o, r_o) = other;
                    let off = harmonic_sup_scalar(
                        r_o.dot(q) / q_len * c_ha - h_o * s_ha,
                        r_o.cross(q).norm() / q_len,
                    );
                    let (w_off, l_off) = if anchored { (own, off) } else { (off, own) };
                    terms.add(Radius, w_off);
                    terms.add(Line, l_off * reach);
                }
                Derivation::ConeRim { nappe, beta } => {
                    let r = radial(a).norm();
                    let v0 = rim_slant(h, r, half_angle, nappe);
                    let rho = v0 * nappe.value() * s_ha;
                    // The image's height `v0·cos α` less the carrier's,
                    // `sin α·(n·r·cos α − h·sin α)` with no `h`-sized
                    // difference; it stands along the axis, normal to
                    // the radial pair.
                    let lift = s_ha * (r * c_ha * nappe.value() - h * s_ha);
                    terms.add(Centre, w_r.norm());
                    terms.add(Line, l.norm() * reach);
                    class_circle(
                        terms,
                        beta,
                        (rho, rho.powi(2)),
                        (a, b),
                        axis,
                        true,
                        Some(lift),
                    );
                }
                _ => unreachable!("incidence: the cone arm matched its own derivations"),
            }
        }
        (
            &Surface::Sphere {
                center,
                radius,
                axis,
                ..
            },
            Derivation::SphereParallel { .. } | Derivation::SphereMeridian { .. },
        ) => {
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let w = c - center;
            let (h, w_r) = (w.dot(axis), radial(w));
            let (a_r, b_r) = (radial(a), radial(b));
            let aa = a.dot(axis);
            terms.add(Line, l.norm() * reach);
            match derivation {
                Derivation::SphereParallel { beta } => {
                    let rho_sq = radius.powi(2) - h.powi(2);
                    let rho = rho_sq.sqrt();
                    terms.add(Centre, w_r.norm());
                    class_circle(terms, beta, (rho, rho_sq), (a, b), axis, true, None);
                }
                Derivation::SphereMeridian { pole, sigma, far } => {
                    // The image turns `(1 + k)·(a′·cos t + σ·b′·sin t)` with
                    // `a′ = ‖a_r‖·d̂ + (a·e)·e` and `b′ = ‖a_r‖·e − (a·e)·d̂`
                    // (`‖a′‖ = ‖b′‖ = ‖a‖`), `d̂` the azimuth direction the
                    // derivation read: `â_r`, where `a′ = a` and `b′ = J·a`, or
                    // `b̂_r` at a pole start. A pole start with `b_r` zero too
                    // is no circle; its `b̂_r` is NaN or an entire enclosure,
                    // which refuses at the envelope's decision.
                    // On the far half the derivation read `d̂` negated, and
                    // `‖a_r‖` with it: the same `a′`, `b′`.
                    let flip = if far { T::zero() - T::one() } else { T::one() };
                    let s_r = a_r.norm() * flip;
                    let d = if pole {
                        b_r / b_r.norm()
                    } else {
                        a_r / a_r.norm()
                    } * flip;
                    let jb = (axis * s_r - d * aa) * angle_rate::<T>(sigma);
                    let k = stretch(radius, radius.powi(2), a);
                    terms.add(Centre, w.norm());
                    let cos_part = if pole {
                        let ja = d * s_r + axis * aa;
                        let cos_part = ja * k + (ja - a);
                        #[cfg(test)]
                        let cos_part = mutants::pole_start_under(a, k).unwrap_or(cos_part);
                        cos_part
                    } else {
                        a * k
                    };
                    let pair = harmonic_sup(cos_part, jb * k + (jb - b));
                    #[cfg(test)]
                    // The derivation's own σ, before the far half reversed
                    // it: reversed back through `Channel::mapped`, the one
                    // place a reflection touches a stored count.
                    let read = if far {
                        Channel::<T>::Angle {
                            phase: T::zero(),
                            winding: sigma,
                        }
                        .mapped(AxisMap::reflect())
                        .and_then(Channel::winding)
                        // A table winding always has a negation.
                        .unwrap_or(sigma)
                    } else {
                        sigma
                    };
                    #[cfg(test)]
                    let pair = mutants::sphere_meridian(
                        pole,
                        radius,
                        (a, b),
                        (a_r, b_r),
                        (aa, b.dot(axis)),
                        read,
                        jb,
                    )
                    .unwrap_or(pair);
                    terms.add(Radius, pair);
                }
                _ => unreachable!("incidence: the sphere arm matched its own derivations"),
            }
        }
        (
            &Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                ..
            },
            Derivation::TorusParallel { .. } | Derivation::TorusMeridian { .. },
        ) => {
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let w = c - center;
            let (h, w_r) = (w.dot(axis), radial(w));
            let a_r = radial(a);
            let aa = a.dot(axis);
            let minor_sq = minor_radius.powi(2);
            terms.add(Line, l.norm() * reach);
            match derivation {
                Derivation::TorusParallel { beta } => {
                    let rho = a_r.norm();
                    let core = rho - major_radius;
                    let m_sq = h.powi(2) + core.powi(2);
                    let m = m_sq.sqrt();
                    let tube = (minor_sq - m_sq).abs() / (minor_radius + m);
                    let (b_r, bb) = (radial(b), b.dot(axis));
                    match beta {
                        0 => {
                            terms.add(Centre, w_r.norm());
                            terms.add(Radius, tube + major_radius + minor_radius);
                            let drift = harmonic_sup(a, b);
                            #[cfg(test)]
                            let drift = mutants::zero_winding(a_r, b_r, (aa, bb)).unwrap_or(drift);
                            terms.add(Drift, drift);
                        }
                        _ => {
                            // `κ = r/m − 1`: the image's radius is `ρ + core·κ`
                            // and its height `h·(1 + κ)`, so its constant
                            // stands `−w_r + e·h·κ` off the carrier's, one
                            // vector read with the pair (`constant_and_pair`).
                            // A carrier on the tube's core (`m = 0`) has no
                            // image; `κ` is then NaN or an entire enclosure,
                            // which refuses at the envelope's decision.
                            let kappa = (minor_sq - m_sq) / ((minor_radius + m) * m);
                            let k = core * kappa / rho;
                            let turn = axis.cross(a_r) * angle_rate::<T>(beta);
                            let (constant, cos_part, sin_part) = (
                                axis * (h * kappa) - w_r,
                                a_r * k - axis * aa,
                                turn * k + (turn - b_r) - axis * bb,
                            );
                            let whole = constant_and_pair(constant, cos_part, sin_part);
                            #[cfg(test)]
                            let whole = mutants::torus_parallel(w_r, tube, b_r, turn, (aa, bb))
                                .or_else(|| {
                                    mutants::torus_parallel_constant(constant, cos_part, sin_part)
                                })
                                .unwrap_or(whole);
                            terms.add(Radius, whole);
                        }
                    }
                }
                Derivation::TorusMeridian { sigma } => {
                    let wr_sq = w_r.dot(w_r);
                    let aw = a.dot(w_r);
                    let n_sq = aw.powi(2) / wr_sq + aa.powi(2);
                    // `R·ŵ_r − w`: its radial and axial parts, one vector.
                    let spine = norm_gap(major_radius, major_radius.powi(2), w_r);
                    let centre = harmonic_sup_scalar(spine, h);
                    #[cfg(test)]
                    let centre = mutants::torus_meridian_centre(spine, h).unwrap_or(centre);
                    terms.add(Centre, centre);
                    // `a*` and `b*`, the image's `cos t` and `sin t`
                    // coefficients up to the factor `r/n = 1 + k`. An `a`
                    // with no part in the meridian plane (`n = 0`) has no
                    // image; `k` is then NaN or an entire enclosure, which
                    // refuses at the envelope's decision.
                    let n = n_sq.sqrt();
                    let k = (minor_sq - n_sq) / ((minor_radius + n) * n);
                    let a_star = w_r * (aw / wr_sq) + axis * aa;
                    let b_star = (axis * aw - w_r * aa) / wr_sq.sqrt() * angle_rate::<T>(sigma);
                    let pair = harmonic_sup(a_star * k + (a_star - a), b_star * k + (b_star - b));
                    #[cfg(test)]
                    let pair = mutants::torus_meridian(
                        (minor_radius, minor_sq),
                        (n, n_sq),
                        a_r - w_r * (aw / wr_sq),
                        b_star - b,
                    )
                    .unwrap_or(pair);
                    terms.add(Radius, pair);
                }
                _ => unreachable!("incidence: the torus arm matched its own derivations"),
            }
        }
        _ => unreachable!(
            "incidence: derive_harmonic answers a {} chart with that chart's own derivations, \
             and the plane's envelope is not incidence",
            surface.kind().name()
        ),
    }
}

/// **The principal windows** a periodic chart's angles are placed in
/// ([`Window`]): an azimuth or a torus's tube angle in `(−1, τ − 1]`
/// ([`ANGLE_WINDOW_START`], whose doc has the census that chose it),
/// about the centre `π − 1`; a sphere's latitude in `(1 − π, 1 + π]`
/// ([`LATITUDE_WINDOW_START`]), about `1`, which holds every latitude
/// the chart's points have, so each of those is named as itself. A
/// row's angle at its span's midpoint is taken into its window by
/// deciding the turns off it through a [`Period`]'s marks
/// ([`Placement::turns`]). A net's channels keep their knot domain.
pub use geom_core::period::Period;
pub use geom_core::period::{BranchMiss, WalkStop};

/// The `(u, v)` periods of an analytic chart's channels: `None` where the
/// channel is a length ([`angle_periods`]).
type AnglePeriods<T> = (Option<Period<T>>, Option<Period<T>>);

/// **The analytic chart's angle channels, each with its `Period`**: `τ`
/// where the chart's coordinate is an angle, none where it is a length
/// (a plane's two, a cylinder's or cone's `v`). The one home of which
/// analytic channel is an angle: the harmonic lane's kind check and
/// [`placement_terms`] read it. `None` on a spline chart, whose periods
/// are its knot domains' and are decided closed by a band (`topo`).
fn angle_periods<T: Real>(surface: &Surface<T>) -> Option<AnglePeriods<T>> {
    let angle = || Some(Period::new(T::tau()));
    match *surface {
        Surface::Plane { .. } => Some((None, None)),
        Surface::Cylinder { .. } | Surface::Cone { .. } => Some((angle(), None)),
        Surface::Sphere { .. } | Surface::Torus { .. } => Some((angle(), angle())),
        Surface::Nurbs(_) | Surface::Approx(_) => None,
    }
}

/// Check 4's **placement** terms on the lanes whose envelope is a 3-D
/// statement — the focal-section, spiric and projected ones. Their
/// envelopes compare `S(P(t))` with `C(t)` in space, and `S` is periodic
/// in each angular channel, so a row stored a whole period off its placed
/// image (a lifted copy, no row: C4) meets the carrier exactly and those
/// envelopes cannot see it. So the stored row is compared in the chart
/// with the image the mint places over the same span
/// ([`chart_pcurve_over`], `Placement::over`): the two chart points at
/// `t₀`, each **angle** channel's offset read past half its own period
/// ([`angle_periods`]), `(|Δ| − p/2)⁺`, metered at the chart's sup
/// stretch, as `(FidelityU, FidelityV)` in metres. Below half a period a
/// stored row's drift moves its point in space, which the lane's own
/// envelope already reads, so this term reads none of it and no drift is
/// counted twice. At a whole period or more the point does not move, the
/// lane's envelope reads nothing, and this term reads at least half the
/// period times the channel's arm. No whole-period integer is read: the
/// term is a continuous function of the offset. A **free** channel (a
/// length: a cylinder's or cone's `v`) has no period and no lifted copy;
/// every offset in it moves the point, which the lane's envelope reads,
/// so it carries no placement term.
///
/// On a sphere a point has two names, `P_d` and its involution twin
/// `(u + π, π − v)`, half a period apart in `u`; the row is read against
/// the `name` its lane decided as structure (the projected lane's
/// `pcurve_projected_v_sign`: the twin negates the second channel's
/// sign), never the nearer by offset, which would let a copy a whole
/// period off read as the twin half a period away. On a
/// plane no channel is an angle and there is no lifted copy to read:
/// both terms are zero, and nothing is derived.
fn placement_terms<T: Decide>(
    stored: &Pcurve<T>,
    name: ImageName,
    (t0, t1): (T, T),
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<(T, T), PcurveCertifyError> {
    let Some((period_u, period_v)) = angle_periods(surface) else {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: surface.kind(),
        });
    };
    if period_u.is_none() && period_v.is_none() {
        return Ok((T::zero(), T::zero()));
    }
    let (arm_u, arm_v) = chart_sup_arms_at(surface, &stored.chart_box(t0, t1))?;
    let derived = chart_pcurve_over(carrier, t0, t1, surface, band)?;
    let at = stored.eval(t0);
    let past_half = |d: T, period: Option<Period<T>>| match period {
        Some(p) => (d.abs() - p.half()).max(T::zero()),
        None => T::zero(),
    };
    let offsets = |image: &Pcurve<T>| {
        let p = image.eval(t0);
        (
            past_half(at.x - p.x, period_u),
            past_half(at.y - p.y, period_v),
        )
    };
    // The row is read against the name its lane decided it is; only the
    // projected lane on a sphere names a twin (`pcurve_projected_v_sign`).
    let named = match name {
        ImageName::Derived => derived,
        ImageName::Twin => {
            let Ok(twin) = derived.map_affine(ChartMap::sphere_involution()) else {
                unreachable!(
                    "only a projected image is named a twin, and its deck map takes every chart map"
                )
            };
            twin
        }
    };
    let (du, dv) = offsets(&named);
    Ok((arm_u.to_meters(du), arm_v.to_meters(dv)))
}

/// **Check 4's verdict**, the one spelling every lane decides its
/// envelope through (`pcurve_envelope`): `Zero` certifies; anything else
/// refuses, or escalates, at the check `named` names — the envelope term
/// over the band alone where the lane keeps its terms apart (the harmonic
/// and projected lanes, [`EnvelopeTerms::first_over`]), its placement
/// term where the lane's own statement is one 3-D number (the
/// focal-section and spiric lanes, [`placement_check`]), and the whole
/// [`PcurveCheck::Envelope`] where it is one hull bound with no terms to
/// name (the iso, iso-arc and fitted lanes).
fn envelope_verdict<T: Decide>(
    envelope: T,
    band: Band,
    named: impl FnOnce() -> PcurveCheck,
) -> Result<(), PcurveCertifyError> {
    match decide("pcurve_envelope", Margin::of(envelope), band) {
        Ok(Sign::Zero) => Ok(()),
        refused => {
            let check = named();
            Err(match refused {
                Err(cause) => PcurveCertifyError::Escalated {
                    check,
                    sample: 0,
                    cause,
                },
                Ok(_) => PcurveCertifyError::ResidualExceeded { check, sample: 0 },
            })
        }
    }
}

/// The check a lane's refused envelope names when it carries
/// [`placement_terms`]: the placement term over the band alone, the `u`
/// channel's first, else the lane's own `fallback`.
fn placement_check<T: Decide>((fu, fv): (T, T), band: Band, fallback: PcurveCheck) -> PcurveCheck {
    let over = |x: T| {
        !matches!(
            decide("pcurve_envelope_term", Margin::of(x), band),
            Ok(Sign::Zero)
        )
    };
    if over(fu) {
        PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityU)
    } else if over(fv) {
        PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityV)
    } else {
        fallback
    }
}

/// Check 4's **fidelity** terms: the stored image `P` against the
/// re-derived `P_d`, channel by channel, metered at the chart's sup
/// stretch — `|S(P(t)) − S(P_d(t))| ≤ arm_u·sup|Δu| + arm_v·sup|Δv|`
/// along the chart segment between the two. Each offset is read as it
/// is: a row is its placed image (C4), so a row a whole period off is a
/// lifted copy and its term reads the period. A sphere point has two
/// harmonic names, `P_d` and its
/// involution twin `(u + π, π − v)`, and the stored image is measured
/// against the name its structure says it is: on a meridian its
/// windings, on a parallel the sign of its azimuth offset's cosine
/// (`pcurve_fidelity_twin`).
fn fidelity<T: Decide>(
    (su, sv): (Channel<T>, Channel<T>),
    (du, dv): (Channel<T>, Channel<T>),
    surface: &Surface<T>,
    (t0, t1): (T, T),
    reach: T,
    band: Band,
    terms: &mut EnvelopeTerms<T>,
) -> Result<(), PcurveCertifyError> {
    let (p0, pa, pb, pl) = harmonic_form_of(su, sv);
    let (q0, qa, qb, ql) = harmonic_form_of(du, dv);
    let stored_windings = [su.winding(), sv.winding()];
    // The cone's two legs below are not orthogonal, so its channels add
    // (`FidelityLegs::of`): with `v₁` the nearer slant and `θ` the
    // azimuth gap, the chord `v₁·(g(u₂) − g(u₁))` and the ruling
    // `(v₂ − v₁)·g(u₂)` meet with cross term `2·v₁·(v₂ − v₁)·sin²α·(1 −
    // cos θ)`, which is `≥ 0` when the two slants share a nappe (`v₁`
    // the nearer, so `v₁·(v₂ − v₁) ≥ 0`).
    terms.1 = FidelityLegs::of(surface);
    // On a cone, `S(u, v) = apex + v·g(u)` with `|g(u₁) − g(u₂)| = sin α·
    // chord(u₁ − u₂)`, so `|S(u₁, v₁) − S(u₂, v₂)| ≤ |v₁ − v₂| +
    // min(|v₁|, |v₂|)·sin α·|u₁ − u₂|`: either image's slant levers the
    // azimuth, and the nearer one's is enough.
    let (arm_u, arm_v) = chart_sup_arms_over(surface, || {
        harmonic_span_box(p0, pa, pb, pl, t0, t1)
            .v_reach()
            .min(harmonic_span_box(q0, qa, qb, ql, t0, t1).v_reach())
    })?;
    // The stored row is placed as its derivation is (C4: the mint and
    // check 4 place one image over one span, `Placement`), so its
    // constants ARE the derived ones, or on a sphere those of the
    // derivation's involution twin `(u + π, π − v)`, the same placed
    // image's other name: no whole period is decided, the offsets are read
    // as they are, and a row stored a whole period off is a lifted copy,
    // no row (it refuses as its channel's term). Where the polar channel
    // winds (a meridian), the windings say which name the row is, as
    // structure: the twin's polar winding is the derivation's negated.
    // Where it does not (a parallel), which name is nearer is one sign,
    // the cosine of the azimuth offset `Δ` (`pcurve_fidelity_twin`):
    // positive, the row is `P_d`; negative, its twin. The cosine is
    // levered at the azimuth's arm (metres per radian), and its unit holds
    // there: `|cos Δ|` lies between `(2/π)·d` and `d`, `d` the angular
    // distance from `Δ` to the nearest half-way mark `π/2 + kπ`, so the
    // levered cosine is that distance in metres to within `2/π`, and with
    // a positive arm its decided sign is the cosine's own.
    // - Decided `Zero`, the row sits on a half-way mark, a quarter turn
    //   off both names: it is read as `P_d` (D9's tie-break), and the
    //   quarter turn refuses as its azimuth's term at either name, so the
    //   tie-break names no row.
    // - Undecided, the twin read escalates typed, at the azimuth's term:
    //   no name is guessed, so the refusal names its own cause.
    let sphere = matches!(surface, Surface::Sphere { .. });
    let involution = ChartMap::sphere_involution();
    let named_twin = match (sv.winding(), dv.winding()) {
        (Some(stored), Some(derived)) if sphere && derived != 0 => {
            Some(Some(stored) == dv.mapped(involution.v).and_then(Channel::winding))
        }
        _ => None,
    };
    let twin = match named_twin {
        Some(twin) => twin,
        None if sphere => match decide(
            "pcurve_fidelity_twin",
            Margin::levered((p0.x - q0.x).cos(), arm_u.get()),
            band,
        ) {
            Ok(Sign::Negative) => true,
            Ok(Sign::Positive | Sign::Zero) => false,
            Err(cause) => {
                return Err(PcurveCertifyError::Escalated {
                    check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityU),
                    sample: 0,
                    cause,
                });
            }
        },
        None => false,
    };
    // The twin is the derived row under the sphere's involution
    // (`ChartMap::sphere_involution`, the map `topo`'s walk names a twin
    // by, channel by channel as `Pcurve::map_affine` maps it), so
    // `Channel::mapped` is the one place its reflection touches a
    // winding.
    // The derived counts are `winding_of`'s, in `WINDING_TABLE`, so the
    // reflection always has a negation; a count without one would be a
    // count no closed-form table holds, which is what
    // `ChartWindingUnsupported` names.
    let (nu, nv) = if twin {
        du.mapped(involution.u)
            .zip(dv.mapped(involution.v))
            .ok_or(PcurveCertifyError::ChartWindingUnsupported)?
    } else {
        (du, dv)
    };
    let (q0, qa, qb, ql) = harmonic_form_of(nu, nv);
    // The windings are structure (C4): the row's are the windings of
    // the name it is measured against, the derivation's or, on a
    // sphere, its twin's.
    let derived_windings = [nu.winding(), nv.winding()];
    if stored_windings != derived_windings {
        return Err(PcurveCertifyError::WindingMismatch {
            stored: stored_windings,
            derived: derived_windings,
        });
    }
    let du = (p0.x - q0.x).abs()
        + harmonic_sup_scalar(pa.x - qa.x, pb.x - qb.x)
        + (pl.x - ql.x).abs() * reach;
    let dv = (p0.y - q0.y).abs()
        + harmonic_sup_scalar(pa.y - qa.y, pb.y - qb.y)
        + (pl.y - ql.y).abs() * reach;
    terms.add(EnvelopeTerm::FidelityU, arm_u.to_meters(du));
    terms.add(EnvelopeTerm::FidelityV, arm_v.to_meters(dv));
    Ok(())
}

/// **The focal-section lane's four checks** — the harmonic lane's
/// order and meters. As on a harmonic row (C4), check 4's envelope is
/// the whole certified statement and check 3 its cross-check.
///
/// 1. **Lane**: the chart is a cone or a torus and the carrier the conic
///    that instance images — an [`geom::Curve3::Ellipse`] on a cone, a
///    [`geom::Curve3::Circle`] on a torus; `sense` is a unit sign
///    (`pcurve_focal_section_sense`, levered at the chart's azimuth
///    arm), `β` is definitely inside `(−1, 1)`
///    (`pcurve_focal_section_beta`) — the premise of the cut-free
///    azimuth — and on a torus `vl` is a unit sign
///    (`pcurve_focal_section_tube_rate`, levered at `r`).
/// 2. **Interval**: `t₁ − t₀` definitely forward, metered through the
///    carrier's rate. No period gate: the carrier (an ellipse, a
///    circle) has its interval held to one period by the edge
///    certificate (`interval_span_winding`), and the image's azimuth
///    advances monotonically one turn per carrier period (the true
///    anomaly's), its tube angle at the unit rate check 1 decides, so
///    neither spans more than one period where the carrier does not.
/// 3. **Schedule**: the shared [`CERT_SAMPLES`] residuals, the closed
///    form's cross-check on the witness lane
///    ([`geom_core::Witness::Inexact`]), recording no verdict.
/// 4. **Envelope**: [`focal_section_envelope`]'s lemma —
///    [`EnvelopeStatement::MapResidualClosedForm`].
fn run_focal_section_checks<T: Decide>(
    pcurve: &Pcurve<T>,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    let esc = |check| {
        move |cause| PcurveCertifyError::Escalated {
            check,
            sample: 0,
            cause,
        }
    };
    let mismatch = |why| PcurveCertifyError::ImageMismatch {
        image: PcurveKind::FocalSection,
        why,
    };
    // ---- Check 1: the lane. ----
    let Pcurve::FocalSection(image) = *pcurve else {
        unreachable!(
            "run_focal_section_checks: both callers match `Pcurve::FocalSection` to reach it"
        )
    };
    let tube = match (surface, carrier) {
        (Surface::Cone { .. }, Curve3::Ellipse { .. }) => None,
        (Surface::Torus { minor_radius, .. }, Curve3::Circle { .. }) => Some(*minor_radius),
        (Surface::Cone { .. } | Surface::Torus { .. }, _) => {
            return Err(mismatch(
                "a focal-section image is a cone's ellipse or a torus's circle, and the \
                 carrier is neither on this chart",
            ));
        }
        _ => {
            return Err(PcurveCertifyError::UnsupportedChart {
                chart: surface.kind(),
            });
        }
    };
    let Some(carrier_form) = carrier_harmonic(carrier) else {
        unreachable!("carrier_harmonic: an ellipse or circle has its harmonic form")
    };
    let boxed = pcurve.chart_box(t0, t1);
    let v_sup = boxed.v_reach();
    let arm = azimuth_lever(surface, v_sup);
    match decide(
        "pcurve_focal_section_sense",
        Margin::levered(image.sense.abs() - T::one(), arm),
        band,
    )
    .map_err(esc(PcurveCheck::ImageIdentity))?
    {
        Sign::Zero => {}
        Sign::Positive | Sign::Negative => {
            return Err(mismatch(
                "the stored sense is not ±1, so the image winds no section",
            ));
        }
    }
    match decide(
        "pcurve_focal_section_beta",
        Margin::levered(T::one() - image.beta.abs(), arm),
        band,
    )
    .map_err(esc(PcurveCheck::ImageIdentity))?
    {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => {
            return Err(mismatch(
                "the stored |β| is not below 1, so the true-anomaly map is not cut-free",
            ));
        }
    }
    if let Some(r) = tube {
        match decide(
            "pcurve_focal_section_tube_rate",
            Margin::levered(image.vl.abs() - T::one(), r),
            band,
        )
        .map_err(esc(PcurveCheck::ImageIdentity))?
        {
            Sign::Zero => {}
            Sign::Positive | Sign::Negative => {
                return Err(mismatch(
                    "the stored tube rate is not ±1, so the image is no Villarceau circle's",
                ));
            }
        }
    }

    let mut max_residual = T::zero();

    // ---- Check 2: the parameter interval, metered into metres. ----
    let rate = param_rate(carrier);
    let span = t1 - t0;
    match decide("pcurve_interval_forward", Margin::metered(span, rate), band)
        .map_err(esc(PcurveCheck::ParamSpan))?
    {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(PcurveCertifyError::IntervalNotForward),
    }

    // ---- Check 3: the schedule, the closed form's cross-check. ----
    let samples = match T::WITNESS {
        geom_core::Witness::Inexact => {
            schedule_residuals(
                Record::CrossCheck,
                pcurve,
                t0,
                t1,
                carrier,
                surface,
                band,
                &mut max_residual,
            )?;
            CERT_SAMPLES
        }
        geom_core::Witness::Exact => 0,
    };

    // ---- Check 4: the closed-form envelope over the whole span. ----
    // The 3-D envelope cannot see a whole period of either channel, so
    // the row's placement is its own pair of terms (`placement_terms`).
    let placement = placement_terms(pcurve, ImageName::Derived, (t0, t1), carrier, surface, band)?;
    let envelope =
        focal_section_envelope(&image, carrier_form, surface, v_sup, t0.abs().max(t1.abs()))
            + placement.0
            + placement.1;
    envelope_verdict(envelope, band, || {
        placement_check(placement, band, PcurveCheck::Envelope)
    })?;

    Ok(PcurveCertificate::closed(
        samples,
        max_residual,
        envelope,
        EnvelopeStatement::MapResidualClosedForm,
    ))
}

/// **The focal-section envelope lemma**: a bound on
/// `sup |S(P(t)) − C(t)|` over a span with `|t| ≤ reach`, for the
/// stored image `P` on a cone or torus chart `S` whose second channel
/// stays within `|v| ≤ v_sup` there, in the image's fields, the chart's
/// data and the carrier's harmonic form `c + a·cos t + b·sin t + l·t`.
///
/// Term by term, with `σ` the sign of `sense`, `E = t − t0`,
/// `E_max = reach + |t0|`, `e = 2β/(1 + β²)` and
/// `q = √(1 − e²) = (1 − β²)/(1 + β²)`:
///
/// - **Frame.** The chart's orthonormal twin `Ŝ`
///   ([`orthonormal_chart`]) is within [`frame_defect`] of `S` over the
///   image; everything below is on `Ŝ`, frame `(n̂, ê₁, ê₂ = n̂ × ê₁)`.
/// - **Sense drift.** Reading `σ` for `sense` moves the azimuth by
///   `||sense| − 1|·|ν|`, `|ν| ≤ E_max + π` (`ν − E` is an `atan2`
///   off its cut), and `d1` below by the same factor of its
///   coefficient; both are levered by the azimuth arm `L`
///   (`sin α·max(v_sup, |v0|)` on a cone, `R + r` on a torus):
///   `L·||sense| − 1|·(E_max + π + 1)`. Two parts of it are required by
///   the split and covered by slack elsewhere, so no row isolates them:
///   the `+ 1` is `d1`'s share, and `|ν − E| + q·|sin E| < π` over every
///   `β` and `E` (its sup tends to `π` as `|β| → 1`), so the azimuth
///   term's `π` already holds it; and the cone's `|v0|` arm bounds only that share
///   (`d1`'s coefficient is `sin α·v0·q`), while the point itself moves
///   by `sin α·|v(t)| ≤ sin α·v_sup` per radian.
/// - **The focal decomposition.** With `d0 = ρ̂(u0)` and
///   `d1 = sense·(n̂ × d0)`, `ρ̂(u0 + σν) = cos ν·d0 + sin ν·d1`, and
///   Kepler's identities `cos ν = (cos E − e)/(1 − e·cos E)`,
///   `sin ν = q·sin E/(1 − e·cos E)` give, for any `A`,
///   `A·(1 − e·cos E)·ρ̂(u(t)) = A·((cos E − e)·d0 + q·sin E·d1)`:
///   harmonic in `E`, so in `t`. Each instance splits its radial factor
///   as `A·(1 − e·cos E)` plus a remainder times the unit `ρ̂(u(t))`:
///   - *Cone*, `Ŝ(u, v) = apex + n̂·cos α·v + ρ̂(u)·sin α·v`, `A = sin α·v0`:
///     `Ŝ(P(t)) = H(t) + sin α·(v(t) − v0·(1 − e·cos E))·ρ̂(u(t))` with
///     `H = apex + n̂·cos α·v(t) + sin α·v0·((cos E − e)·d0 + q·sin E·d1)`,
///     and `v(t) − v0·(1 − e·cos E) = (va + e·v0·cos t0)·cos t +
///     (vb + e·v0·sin t0)·sin t + vl·t`, so the **remainder** is
///     `sin α·(√((va + e·v0·cos t0)² + (vb + e·v0·sin t0)²) + |vl|·reach)`.
///   - *Torus*, `Ŝ(u, v) = O + (R + r·cos v)·ρ̂(u) + r·sin v·n̂`,
///     `A = R`, with `v_r = v0 + vl·t0` the tube angle at the vertex.
///     **Meridional drift**: `v(t) = v_r + vl·E + va·cos t + vb·sin t`,
///     and reading `v* = v_r + s·E` (`s` the sign of `vl`) instead
///     moves the point by `r·(√(va² + vb²) + ||vl| − 1|·E_max)`
///     (`|∂Ŝ/∂v| = r`). Then `R + r·cos v* = R·(1 − e·cos E) +
///     (r·cos v_r + R·e)·cos E − s·r·sin v_r·sin E` and
///     `r·sin v* = r·sin v_r·cos E + s·r·cos v_r·sin E`, so
///     `Ŝ(P*(t)) = H(t) + (…)·ρ̂(u(t))` with
///     `H = O + R·((cos E − e)·d0 + q·sin E·d1) + r·(sin v_r·cos E + s·cos v_r·sin E)·n̂`,
///     whose `s` is read as `vl` at `r·||vl| − 1|` more, and the
///     **meridional-phase remainder** `√((r·cos v_r + R·e)² + r²·sin² v_r)`
///     — zero in ℝ exactly when the vertex is on the outer equator and
///     `e = −r/R`, the Villarceau circle's.
/// - **Coefficients.** `H` is `h_c + h_a·cos t + h_b·sin t + h_l·t`
///   (rotating `cos E`, `sin E` by `t0`), so
///   `sup |H − C| ≤ |h_c − c| + σ(h_a − a, h_b − b) + |h_l − l|·reach`,
///   `σ` the exact sup of a `cos t`/`sin t` pair
///   ([`EnvelopeStatement::MapResidualClosedForm`]).
///
/// Every term is rounding-scale on a minted image, so the envelope is
/// the exact image's distance from the carrier plus the drift every
/// banded check-1 gate admitted.
///
/// The coefficient terms are full-period sups by construction: a short
/// arc pays the far side's coefficient error. That is the contract; on
/// the suites' certificates it costs at most about 2.2× against the
/// span's own miss (`focal-section-envelope-bounds-the-whole-ellipse-not-the-span`).
fn focal_section_envelope<T: Real>(
    image: &FocalImage<T>,
    carrier: Harmonic3<T>,
    surface: &Surface<T>,
    v_sup: T,
    reach: T,
) -> T {
    let one = T::one();
    let ideal = orthonormal_chart(surface);
    let (axis, u_ref) = match ideal {
        Surface::Cone { axis, u_ref, .. } | Surface::Torus { axis, u_ref, .. } => (axis, u_ref),
        _ => unreachable!("focal_section_envelope: check 1 admits cone and torus charts only"),
    };
    let FocalImage {
        u0,
        t0,
        v0,
        va,
        vb,
        vl,
        beta,
        sense,
    } = *image;
    let (su, cu) = u0.sin_cos();
    let cv = axis.cross(u_ref);
    let d0 = u_ref * cu + cv * su;
    let d1 = (cv * cu - u_ref * su) * sense;
    let bb = beta.powi(2);
    let ecc = (beta + beta) / (one + bb);
    let root = (one - bb) / (one + bb);
    let (st, ct) = t0.sin_cos();
    let e_max = reach + t0.abs();
    let sense_drift = |lever: T| lever * (sense.abs() - one).abs() * (e_max + T::pi() + one);
    // `H = k + x·cos E + y·sin E + (a·cos t + b·sin t + l·t)`, and the
    // terms outside `H`.
    let (k, x, y, a, b, l, outside) = match ideal {
        Surface::Cone {
            apex, half_angle, ..
        } => {
            let (sin_a, cos_a) = half_angle.sin_cos();
            let lever = sin_a * v0;
            let remainder = sin_a
                * (harmonic_sup_scalar(va + ecc * v0 * ct, vb + ecc * v0 * st) + vl.abs() * reach);
            (
                apex + axis * (cos_a * v0) - d0 * (lever * ecc),
                d0 * lever,
                d1 * (lever * root),
                axis * (cos_a * va),
                axis * (cos_a * vb),
                axis * (cos_a * vl),
                remainder + sense_drift(sin_a * v_sup.max(v0.abs())),
            )
        }
        Surface::Torus {
            center,
            major_radius: big,
            minor_radius: r,
            ..
        } => {
            let v_r = v0 + vl * t0;
            let (sv, cv_r) = v_r.sin_cos();
            let tube_drift = (vl.abs() - one).abs();
            let meridional = r * (harmonic_sup_scalar(va, vb) + tube_drift * (e_max + one));
            let phase = harmonic_sup_scalar(r * cv_r + big * ecc, r * sv);
            let zero = Vec3::new(T::zero(), T::zero(), T::zero());
            (
                center - d0 * (big * ecc),
                d0 * big + axis * (r * sv),
                d1 * (big * root) + axis * (r * vl * cv_r),
                zero,
                zero,
                zero,
                meridional + phase + sense_drift(big + r),
            )
        }
        _ => unreachable!("focal_section_envelope: check 1 admits cone and torus charts only"),
    };
    // cos E = cos t·cos t0 + sin t·sin t0, sin E = sin t·cos t0 − cos t·sin t0.
    let h_a = a + x * ct - y * st;
    let h_b = b + x * st + y * ct;
    (k - carrier.c).norm()
        + harmonic_sup(h_a - carrier.a, h_b - carrier.b)
        + (l - carrier.l).norm() * reach
        + outside
        + frame_defect(surface, v_sup)
}

/// **The spiric lane's four checks** — the harmonic lane's order, the
/// harmonic lane's meters, and two different check-4 statements, one
/// per [`SpiricImage`].
///
/// 1. **Lane**: the carrier is a [`geom::Curve3::Spiric`]; the chart is
///    the one its image kind lives on ([`SpiricImage::Cap`] on a plane,
///    [`SpiricImage::Wall`] on a torus, and nothing else — a spiric
///    lies on no cylinder, cone or sphere); the image's three stored
///    scalars are the CARRIER's (`pcurve_spiric_major`,
///    `pcurve_spiric_minor`, `pcurve_spiric_offset`, each a metre
///    comparand required Zero), a wall's `sense` is a unit sign
///    (`pcurve_spiric_sense`, levered at the chart's azimuth arm),
///    and — for a wall — the CHART is the torus its [`WallTorus`] tag
///    names, the carrier's own or its mirror through the cutting plane
///    (`pcurve_spiric_chart_center`, `_chart_major`, `_chart_minor`,
///    `_chart_tilt`; a chart that is the OTHER of the two tori is
///    read by `pcurve_spiric_chart_other` and refused as a mismatched
///    image), which is the premise the identity maps through.
///    Every one of those gates is BANDED, not bit-equal: `Sign::Zero`
///    admits `|Δ| ≤ ε` and escalates above it. Whatever they admit
///    inside the band is measured, not discarded — it is carried into
///    check 4 as the envelope's own term, and it is exactly zero on
///    every minted image whose scalars are bit-for-bit the carrier's
///    and whose chart centre is spelled as [`WallTorus::center`]
///    spells it.
/// 2. **Interval**: `t₁ − t₀` definitely forward, metered through the
///    carrier's rate (the minor radius — a spiric's speed floor, its
///    variant docs). On the torus the two angular channels are gated
///    against one period as the harmonic lane gates them: the minor
///    angle at the chart's `minor_radius` and the azimuth at
///    [`azimuth_lever`]. The azimuth extent is `≤ π` by STRUCTURE
///    (`atan2`'s range on `f > 0`) and is metered anyway rather than
///    asserted.
/// 3. **Schedule**: the shared [`CERT_SAMPLES`] residuals, evaluated
///    through `Surface::eval` and `Curve3::eval` — so nothing below is
///    trusted, it is verified.
/// 4. **Envelope**: per image.
///    - *Cap*: `S(P(t)) − C(t) = k₀ + k₁·f(t) + k₂·sin t` with
///      CONSTANT vector coefficients (the plane chart is affine and
///      both sides are the same three functions of `t`), so
///      `sup ≤ |k₀| + |k₁|·f_max + |k₂|` in closed form —
///      [`EnvelopeStatement::MapResidualClosedForm`], the harmonic
///      lane's own statement with `f` in place of a unit cosine.
///    - *Wall*: the map is the algebraic identity the variant docs
///      derive, whose only derived numbers are `u₀` and `v₀` and
///      whose residues are visible at every sample —
///      [`EnvelopeStatement::SpiricIdentity`], envelope `0` plus the
///      drift check 1 admitted.
fn run_spiric_checks<T: Decide>(
    pcurve: &Pcurve<T>,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    let &Pcurve::Spiric {
        major,
        minor,
        offset,
        ref image,
    } = pcurve
    else {
        unreachable!("run_spiric_checks: both callers match `Pcurve::Spiric` to reach it")
    };
    // ---- Check 1: the certified lane. ----
    let Curve3::Spiric {
        center: c_c,
        axis: a_c,
        u_ref: n_c,
        major_radius: r_major,
        minor_radius: r_minor,
        offset: c_offset,
    } = *carrier
    else {
        return Err(PcurveCertifyError::ImageMismatch {
            image: PcurveKind::Spiric,
            why: "the carrier is not a spiric, so no spiric image is its own",
        });
    };
    let chart = surface.kind();
    let esc = |check: PcurveCheck| {
        move |cause| PcurveCertifyError::Escalated {
            check,
            sample: 0,
            cause,
        }
    };
    // The image kind IS the chart kind, exhaustively: a spiric lies on
    // its own cutting plane and on the two tori that hold it (its own
    // and that torus's mirror through the plane; which one is check 1's
    // gate below), and on no other analytic chart at all.
    match (image, surface) {
        (SpiricImage::Cap { .. }, Surface::Plane { .. })
        | (SpiricImage::Wall { .. }, Surface::Torus { .. }) => {}
        _ => return Err(PcurveCertifyError::UnsupportedChart { chart }),
    }
    // The three scalars the image carries are the carrier's own —
    // copies, so the image can evaluate from `t` alone. A difference
    // is a metre comparand and must be Zero; what the band admits is
    // measured below.
    let (d_major, d_minor, d_offset) = (major - r_major, minor - r_minor, offset - c_offset);
    for (name, diff) in [
        ("pcurve_spiric_major", d_major),
        ("pcurve_spiric_minor", d_minor),
        ("pcurve_spiric_offset", d_offset),
    ] {
        match decide(name, Margin::of(diff), band).map_err(esc(PcurveCheck::ImageIdentity))? {
            Sign::Zero => {}
            Sign::Positive | Sign::Negative => {
                return Err(PcurveCertifyError::ImageMismatch {
                    image: PcurveKind::Spiric,
                    why: "the image's major radius, minor radius or offset is not the carrier's",
                });
            }
        }
    }
    let reach = t0.abs().max(t1.abs());
    // The wall's sense is a unit sign: a DIMENSIONLESS residue, so it
    // is metered by MULTIPLYING the chart's own lever arm
    // (`Margin::levered` — the door whose docs name "a sine or cosine
    // of unit vectors" — not `over_lever`, which divides a measure by
    // a lever and is for an area over a radius). Spec §1's table says
    // levered, and PR-1a's sibling of the identical shape levers too.
    // The azimuth channel's sign is `μ·sense` with `μ` the exact
    // `±1` of the image's `WallTorus` tag, so this one residue is the
    // residue of both channels.
    let sense_drift = match *image {
        SpiricImage::Wall { sense, .. } => {
            let residue = sense.abs() - T::one();
            match decide(
                "pcurve_spiric_sense",
                Margin::levered(residue, azimuth_lever(surface, reach)),
                band,
            )
            .map_err(esc(PcurveCheck::ImageIdentity))?
            {
                Sign::Zero => residue.abs(),
                Sign::Positive | Sign::Negative => {
                    return Err(PcurveCertifyError::ImageMismatch {
                        image: PcurveKind::Spiric,
                        why: "the wall image's sense is not a unit sign",
                    });
                }
            }
        }
        SpiricImage::Cap { .. } => T::zero(),
    };
    // **The wall's own premise: the chart IS the torus its tag
    // names** — the carrier's own or its mirror through the cutting
    // plane (centre moved `2·offset` along `n`; `spiric_off_own_chart`'s
    // docs), each centre spelled by `WallTorus::center`. The image's scalars
    // being the carrier's says nothing about the SURFACE the identity
    // maps through, and the identity is a statement about that map —
    // so a chart whose centre, axis or radii have drifted from that
    // torus would otherwise store "zero, by an algebraic identity"
    // over a real displacement. Four banded comparands, the harmonic
    // torus arm's posture one lane over, each an audit row; the
    // residues ride into check 4 exactly as the image's do, and each
    // is bit-zero when the chart is the named torus.
    //
    // The CAP arm needs none of this: its check-4 coefficients `k₀`,
    // `k₁`, `k₂` are built FROM the chart, so a drifted plane shows up
    // in the envelope as a number rather than as an unstated premise.
    let chart_drift = match (*image, surface) {
        (
            SpiricImage::Wall { torus, .. },
            &Surface::Torus {
                center: s_center,
                axis: s_axis,
                major_radius: s_major,
                minor_radius: s_minor,
                ..
            },
        ) => {
            let arm = s_major + s_minor;
            let torus_center = |t: WallTorus| t.center(c_c, n_c, c_offset);
            // The axis is gated for PARALLELISM only — a sine of unit
            // vectors, levered at the chart's own outer reach. Which
            // of the two directions it takes is `sense`'s decision at
            // the mint, and a wrong sign is a whole-span displacement
            // the schedule sees at every sample.
            let sin_tilt = s_axis.cross(a_c).norm();
            let d_center = s_center - torus_center(torus);
            let gates = [
                ("pcurve_spiric_chart_center", Margin::norm3(d_center)),
                ("pcurve_spiric_chart_major", Margin::of(s_major - r_major)),
                ("pcurve_spiric_chart_minor", Margin::of(s_minor - r_minor)),
                ("pcurve_spiric_chart_tilt", Margin::levered(sin_tilt, arm)),
            ];
            // Read as an identity ladder (`crate::ladder`): a gate
            // decided apart answers past an earlier in-band one.
            let reader = |name, margin| geom_core::k_stats::decide_reported(name, margin, band);
            match read_identity(gates, reader) {
                IdentityLadder::Zero(_) => {}
                IdentityLadder::Undecided(kept) => {
                    return Err(identity_escalated(kept.refusing_cause()));
                }
                IdentityLadder::Apart { at, .. } => {
                    // Not the named torus. Only a CENTRE apart can leave
                    // the chart the OTHER torus that holds the oval: a
                    // radius or tilt apart is apart from both, so the
                    // other torus's ladder is read only on the centre.
                    let neither = spiric_off_own_chart(
                        "the chart torus is neither the carrier's own nor its mirror \
                         through the cutting plane, and a spiric oval lies on no other \
                         torus",
                    )
                    .refusal(surface, carrier);
                    if at != 0 {
                        return Err(neither);
                    }
                    let d_other = s_center
                        - torus_center(match torus {
                            WallTorus::Own => WallTorus::Mirror,
                            WallTorus::Mirror => WallTorus::Own,
                        });
                    let data = gates
                        .into_iter()
                        .skip(1)
                        .chain([("pcurve_spiric_chart_other", Margin::norm3(d_other))]);
                    return Err(match read_identity(data, reader) {
                        IdentityLadder::Zero(_) => PcurveCertifyError::ImageMismatch {
                            image: PcurveKind::Spiric,
                            why: "the wall image names one of the two tori that hold the \
                                  oval, and the chart is the other",
                        },
                        IdentityLadder::Apart { .. } => neither,
                        IdentityLadder::Undecided(kept) => {
                            identity_escalated(kept.refusing_cause())
                        }
                    });
                }
            }
            // Each residue at its own arm, in metres: a moved centre
            // moves every point by its own length; a wrong `R` moves
            // the radial channel by `|ΔR|`; a wrong `r` moves BOTH the
            // radial and the axial channel, so it pays twice; a frame
            // rotated by `θ` moves a point at radius `≤ R + r` by
            // `(R + r)·θ`, and `θ ≤ (π/2)·sin θ` on `[0, π/2]`.
            d_center.norm()
                + (s_major - r_major).abs()
                + (s_minor - r_minor).abs() * T::from_f64(2.0)
                + arm * sin_tilt * T::pi() * T::from_f64(0.5)
        }
        _ => T::zero(),
    };

    let mut max_residual = T::zero();

    // ---- Check 2: the parameter interval, metered into metres. ----
    let rate = param_rate(carrier);
    let span = t1 - t0;
    match decide("pcurve_interval_forward", Margin::metered(span, rate), band).map_err(|cause| {
        PcurveCertifyError::Escalated {
            check: PcurveCheck::ParamSpan,
            sample: 0,
            cause,
        }
    })? {
        Sign::Positive => {}
        Sign::Zero | Sign::Negative => return Err(PcurveCertifyError::IntervalNotForward),
    }
    // No period gate on a wall image: its carrier (a spiric) has its
    // interval held to one period by the edge certificate
    // (`interval_span_winding`); the minor angle moves at the unit
    // `sense` check 1 decides, so it spans what the carrier does, and
    // the azimuth `u0 + μ·sense·atan2(f, d)` swings under `π` over the
    // whole period (`f > 0` over the two-oval regime).

    // ---- Check 3: the schedule, in metres through the map. ----
    schedule_residuals(
        Record::Verdict,
        pcurve,
        t0,
        t1,
        carrier,
        surface,
        band,
        &mut max_residual,
    )?;

    // ---- Check 4: the between-samples statement, one per image. ----
    //
    // Both arms carry the SAME drift term for the three scalars check 1
    // admitted: the image evaluates `f′` from its stored triple and the
    // carrier evaluates `f` from its own, so a residue in the triple is
    // a residue in `f`, bounded over the whole period by
    //   `|f′ − f| ≤ (|ρ′² − ρ²| + |d′² − d²|) / (f′_min + f_min)`
    // — the `|√A − √B| = |A − B| / (√A + √B)` identity, with
    //   `|ρ′² − ρ²| ≤ (ρ′_max + ρ_max)·(|ΔR| + |Δr|)`  and
    //   `|d′² − d²| ≤ (|d′| + |d|)·|Δd|`.
    // Every factor is exactly zero on a minted image, where the triple
    // is the carrier's own bits, so both statements below are the exact
    // ones their docs claim there and the term is the honest price of a
    // banded gate rather than a bit-equal one.
    let (f_min_i, _) = geom::spiric_f_range(major, minor, offset);
    let (f_min_c, f_max_c) = geom::spiric_f_range(r_major, r_minor, c_offset);
    let rho_sum = (major + minor) + (r_major + r_minor);
    let f_drift = (rho_sum * (d_major.abs() + d_minor.abs())
        + (offset.abs() + c_offset.abs()) * d_offset.abs())
        / (f_min_i + f_min_c);
    let envelope = match *image {
        SpiricImage::Cap { p0, pm, pa } => {
            let Surface::Plane {
                origin,
                normal,
                u_ref,
            } = *surface
            else {
                return Err(PcurveCertifyError::UnsupportedChart { chart });
            };
            let v_ref = normal.cross(u_ref);
            let lift = |p: Vec2<T>| u_ref * p.x + v_ref * p.y;
            let m_c = a_c.cross(n_c);
            // `S(P(t)) − C(t) = k₀ + (k₁ − m)·f + k₁·(f′ − f) + k₂·sin t`
            // with `k₀`, `k₁`, `k₂` constant vectors — the affine
            // chart carries the carrier's own combination through
            // coefficient by coefficient (variant docs).
            let k0 = (origin + (u_ref * p0.x + v_ref * p0.y)) - (c_c + n_c * c_offset);
            let k1 = lift(pm);
            let k2 = lift(pa) - a_c * r_minor;
            k0.norm() + (k1 - m_c).norm() * f_max_c + k1.norm() * f_drift + k2.norm()
        }
        SpiricImage::Wall { .. } => {
            let Surface::Torus {
                major_radius: chart_major,
                minor_radius: chart_minor,
                ..
            } = *surface
            else {
                return Err(PcurveCertifyError::UnsupportedChart { chart });
            };
            // The identity's own residue is zero; what is left is the
            // drift check 1 admitted, carried through the two channels
            // it reaches.
            //
            // *Azimuth*: the image's azimuth is `atan2(f′, d′)` and the
            // carrier's radial part has azimuth `atan2(f, d)`; two
            // planar vectors `(f′, d′)`, `(f, d)` subtend an angle at
            // most `(π/2)·|Δ| / min(|·|)` (the `2sin(φ/2) ≥ (2/π)φ`
            // bound on `[0, π]`), and moving a chart point by `δu`
            // moves it by at most `ρ_max·|δu|`.
            //
            // *`sense`, BOTH channels.* A residue `η` in `|sense|`
            // moves the minor angle by at most `η·reach` at
            // `|∂S/∂v| = r`, AND it moves the azimuth: the channel is
            // `u₀ + μ·sense·atan2(f, d)` (`μ = ±1` exactly, the tag's)
            // and `atan2(f, d) ∈ (0, π)` over the whole two-oval
            // regime, so `|δu| ≤ η·π` at the chart's outer arm. The
            // `u` half is what the over-strict `over_lever` gate used
            // to mask — with the door metering the way its dimension
            // asks, the band admits `η` up to `ε/(R + r)` and this term
            // is what keeps the certificate honest over that window.
            //
            // *The chart's own drift* rides in from check 1, already
            // in metres.
            let rho_min = (f_min_i.powi(2) + offset.powi(2))
                .sqrt()
                .min((f_min_c.powi(2) + c_offset.powi(2)).sqrt());
            let d_theta = T::pi() * T::from_f64(0.5) * (f_drift + d_offset.abs()) / rho_min;
            let arm = chart_major + chart_minor;
            arm * d_theta
                + chart_minor * sense_drift * reach
                + arm * sense_drift * T::pi()
                + chart_drift
        }
    };
    // The wall's identity cannot see a whole period of either channel,
    // so the row's placement is its own pair of terms
    // (`placement_terms`); the cap's plane has none to read.
    let placement = placement_terms(pcurve, ImageName::Derived, (t0, t1), carrier, surface, band)?;
    let envelope = envelope + placement.0 + placement.1;
    envelope_verdict(envelope, band, || {
        placement_check(placement, band, PcurveCheck::Envelope)
    })?;

    Ok(PcurveCertificate::closed(
        CERT_SAMPLES,
        max_residual,
        envelope,
        match image {
            SpiricImage::Cap { .. } => EnvelopeStatement::MapResidualClosedForm,
            SpiricImage::Wall { .. } => EnvelopeStatement::SpiricIdentity,
        },
    ))
}

/// Why a chart kind has no surface-level sup pair — the refusal
/// [`chart_stretch_sup`] answers instead of minting a [`SupSpeed`] it
/// cannot honour.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoChartSup {
    /// A cone's azimuth stretch is `|v·sin α|`, which grows without
    /// bound down the ruling: no surface-level constant dominates it,
    /// so no `u` component exists to mint. The arm is a function of
    /// the caller's own `v` reach, which is what [`chart_sup_arms_at`]
    /// supplies (`v_sup·sin α`). The `v` channel still has an exact
    /// answer and [`chart_stretch_sup_v`] is the door for it.
    ConeAzimuthGrowsWithV,
    /// The chart is the placeholder ([`Surface::is_placeholder_chart`]):
    /// its control net is all-poison, so every evaluation of it is
    /// poison and there is no locus for either arm to be an arm of.
    /// Both channels refuse, [`chart_stretch_sup_v`] included — the
    /// sup-side twin of [`chart_stretch_inf`]'s all-zero answer.
    Placeholder,
}

/// **`(sup |S_u|, sup |S_v|)`** — the chart's UPPER stretch bounds:
/// the lever arms that turn a chart-space overshoot into metres, one
/// per chart parameter, valid over the WHOLE chart.
///
/// # What this bounds, and what it must not be used for
///
/// Each component **dominates** the true local stretch everywhere on
/// the chart: `|S_u(u, v)| ≤ arm_u` and `|S_v(u, v)| ≤ arm_v` at
/// every `(u, v)` of the chart's domain. That is what a [`SupSpeed`]
/// asserts, so the kind that cannot honour it **refuses**: a cone has
/// no surface-level azimuth arm and this door answers
/// [`NoChartSup::ConeAzimuthGrowsWithV`] rather than a number, and the
/// placeholder has no locus at all and answers
/// [`NoChartSup::Placeholder`]. A cone
/// caller goes through [`chart_sup_arms_at`], which supplies
/// `v_sup·sin α` from the row's own box, or through
/// [`chart_stretch_sup_v`] for the `v` channel alone, which is exact
/// on every kind.
///
/// Subject to that, the pair is **sup-side by construction and is NOT
/// a lower bound**. Quoting it where an `inf` is wanted is unsound,
/// and the two uses split like this:
///
/// - **ESCAPE metering (sound).** A claim of the form *"this
///   chart-space displacement does not move the point out of the
///   band"* — a spline chart's loop continuity, an azimuth-period
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
/// **over**-states the escape and can only make an in-band verdict harder —
/// the safe direction, and the same posture the cylinder arm takes
/// exactly. A plane chart's parameters are already metres, so its
/// arms are exactly `(1, 1)` by construction rather than by default.
pub fn chart_stretch_sup<T: Real>(
    surface: &Surface<T>,
) -> Result<(SupSpeed<T>, SupSpeed<T>), NoChartSup> {
    if surface.is_placeholder_chart() {
        return Err(NoChartSup::Placeholder);
    }
    // [`azimuth_lever`] reads no `v` reach off a cylinder, sphere or
    // torus, the kinds that ask it here.
    let azimuth = || SupSpeed::new(azimuth_lever(surface, T::zero()));
    match *surface {
        // The cone's azimuth arm is the caller's to supply; its `v`
        // channel is answered by [`chart_stretch_sup_v`].
        Surface::Cone { .. } => Err(NoChartSup::ConeAzimuthGrowsWithV),
        Surface::Cylinder { .. } => Ok((azimuth(), SupSpeed::new(T::one()))),
        // The sphere/torus second parameter IS an angle (M6-3): its
        // arm is the polar / meridional radius.
        Surface::Sphere { radius, .. } => Ok((azimuth(), SupSpeed::new(radius))),
        Surface::Torus { minor_radius, .. } => Ok((azimuth(), SupSpeed::new(minor_radius))),
        // A described NURBS chart's honest arms are its derivative-net
        // stretch bounds (`sup |S_u|`, `sup |S_v|`) — over-statements
        // of the local stretch, the safe direction exactly as the
        // sphere's.
        //
        // **RATIONAL charts take the same arms since M8-3**, and they
        // must: an arm under-states only in the UNSAFE direction here
        // (its consumers meter escapes: a spline chart's joint gap,
        // `topo::pcurves`' `spline_gap_closes`, and the iso lane's
        // knot-deviation and overshoot terms; a smaller arm makes an
        // escape easier to admit), and before M8-3 the unit arms
        // were harmless only because the iso lane's own rational gate
        // refused before any rational chart reached this function.
        // That gate is gone, so the arm has to be real —
        // `nurbs_stretch_bounds` reads the rational quotient bound
        // (`NurbsSurface::stretch_sup`) for exactly this.
        //
        // The catch-all is SPLIT: an approximating surface's arms are
        // its FIT's derivative-net bounds — the same statement about
        // the same chart. Unit arms would under-state in the unsafe
        // direction here (see the rational note above).
        Surface::Nurbs(ref payload) => Ok(nurbs_stretch_bounds(payload)),
        Surface::Approx(ref a) => Ok(nurbs_stretch_bounds(a.fit())),
        // A plane chart's parameters are already metres.
        Surface::Plane { .. } => Ok((SupSpeed::new(T::one()), SupSpeed::new(T::one()))),
    }
}

/// The SECOND component of [`chart_stretch_sup`] alone — `sup |S_v|`,
/// which every described chart kind has, the cone included.
///
/// [`chart_stretch_sup`] refuses the cone over the `u` channel only: a
/// cone's `v` is a SLANT LENGTH along the ruling, so `|S_v| = 1`
/// exactly, everywhere. The tag is minted here because here it is
/// true. Every other kind answers the pair's second component, which
/// is where that answer lives — and the placeholder, which has no
/// second channel either, refuses here exactly as it does there.
///
/// The direction argument is [`chart_stretch_sup`]'s in every
/// respect: this is the escape side, unsafe for a positive-extent
/// claim.
///
/// # Errors
///
/// [`NoChartSup::Placeholder`] for the placeholder chart, and nothing
/// else.
pub fn chart_stretch_sup_v<T: Real>(surface: &Surface<T>) -> Result<SupSpeed<T>, NoChartSup> {
    match chart_stretch_sup(surface) {
        Ok((_, v)) => Ok(v),
        Err(NoChartSup::ConeAzimuthGrowsWithV) => Ok(SupSpeed::new(T::one())),
        Err(NoChartSup::Placeholder) => Err(NoChartSup::Placeholder),
    }
}

/// [`chart_stretch_sup`] over a chart window — the door that can answer
/// the refusal the surface-level one cannot: the cone's azimuth arm
/// becomes `v_sup·sin α`, with `v_sup` the window's `|v|` reach
/// (dominating the local arm everywhere in the window — the safe
/// direction); every other kind answers as [`chart_stretch_sup`], a
/// sup over the whole chart and so over any window of it.
///
/// # Errors
///
/// [`PcurveCertifyError::PlaceholderChart`] for the placeholder chart,
/// which has arms nowhere, and nothing else.
pub fn chart_sup_arms_at<T: Real>(
    surface: &Surface<T>,
    window: &ChartWindow<T>,
) -> Result<(SupSpeed<T>, SupSpeed<T>), PcurveCertifyError> {
    chart_sup_arms_over(surface, || window.v_reach())
}

/// [`chart_sup_arms_at`] for a caller whose `|v|` sup is not one
/// window's reach; `v_sup` is read on a cone only.
fn chart_sup_arms_over<T: Real>(
    surface: &Surface<T>,
    v_sup: impl FnOnce() -> T,
) -> Result<(SupSpeed<T>, SupSpeed<T>), PcurveCertifyError> {
    match chart_stretch_sup(surface) {
        Ok(pair) => Ok(pair),
        Err(NoChartSup::ConeAzimuthGrowsWithV) => Ok((
            SupSpeed::new(azimuth_lever(surface, v_sup())),
            SupSpeed::new(T::one()),
        )),
        Err(NoChartSup::Placeholder) => Err(PcurveCertifyError::PlaceholderChart),
    }
}

/// `(sup |S_u|, sup |S_v|)` bounds for a NURBS chart — **rational
/// nets included** — read off the chart's own statement,
/// [`geom::NurbsSurface::stretch_sup`] (the quotient bound on the
/// dyadic pieces of each nonempty cell). One door for the whole file: [`chart_stretch_inf`]'s sup
/// components read the same call, so `chart_stretch_sup(s).0` and
/// `chart_stretch_inf(s).sup_u` are one computation by construction
/// (`chart_stretch_inf`'s spline arm calls this function).
fn nurbs_stretch_bounds<T: Real>(s: &geom::NurbsSurface<T>) -> (SupSpeed<T>, SupSpeed<T>) {
    s.stretch_sup()
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
/// # The floors, read per piece
///
/// A derivative's norm is not bounded below by its smallest control
/// difference (a net whose columns cancel has combinations of every
/// norm down to zero), so the inf is read **along a direction, per
/// piece**: on each exact dyadic piece of each nonempty cell, `S_u` is
/// a convex combination of its coefficient ratios `a_k/b_k`
/// (`N_u/W²` raised to one degree), so for the unit `d` of the piece's
/// `Σ a_k`, `‖S_u‖ ≥ S_u·d ≥ min_k (a_k·d)/b_k`
/// ([`geom::NurbsSurface::stretch_floor`]; the proof is
/// `geom_core::spline::derivative`'s). Each piece picks its own `d`, so
/// a chart that turns — a rational half-cylinder — reads positive where
/// one direction for the whole chart would read zero. A **zero-crossing
/// net** — a wall with a fold — has some ratio on the far side of its
/// piece's `d`, or no direction at all, and gets exactly zero. That zero
/// is the honest answer and the caller must refuse on it: no positive
/// bound is invented for a net whose derivative genuinely vanishes.
///
/// `area_inf` is the same read of the area element
/// `S_u × S_v = M̃/W³` ([`geom::NurbsSurface::area_floor`]). A chart
/// whose two derivative directions can align somewhere gets zero here
/// even though both per-axis infs are healthy — which is exactly the
/// case the per-axis pair alone cannot see.
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
/// # Which scalar makes this rigorous
///
/// Every step above is floating point, and at `f64` every step rounds
/// to **nearest**: the answer is a lower bound *up to a few ulps*,
/// not a below-rounded one, so a value that is a bound in exact
/// arithmetic can come back a fraction of an ulp high. That is
/// immaterial against a band whose tightest setting is 1e-12 against
/// arms of order 1, and it is not an argument. Under the **interval
/// scalar** every step rounds outward and the returned bracket's
/// floor is a genuine certified lower bound — which is the lane a
/// caller leaning on these numbers should ask for, and the reason
/// the caller reads the FLOOR rather than the bracket.
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
    if surface.is_placeholder_chart() {
        // No net to bound: the placeholder certifies nothing.
        return zero;
    }
    match *surface {
        Surface::Nurbs(ref payload) => nurbs_stretch_inf(payload),
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

/// A signed floor of a norm, clamped to `[0, cap]`: a negative floor (a
/// piece straddling its own direction) clamps to zero, and a poison one
/// — a piece whose numerator cancels outright, or a refused statement —
/// is exactly zero. The cap is a sup of the same quantity, so a reading
/// can never leak an inflated arm.
fn clamped_floor<T: Real>(floor: T, cap: T) -> T {
    if floor.is_poison() {
        T::zero()
    } else {
        floor.max(T::zero()).min(cap)
    }
}

/// [`chart_stretch_inf`]'s spline arm — the derivation lives there.
fn nurbs_stretch_inf<T: Real>(s: &geom::NurbsSurface<T>) -> ChartStretchInf<T> {
    let (sup_u, sup_v) = nurbs_stretch_bounds(s);
    let (sup_u, sup_v) = (sup_u.get(), sup_v.get());
    ChartStretchInf {
        inf_u: clamped_floor(s.stretch_floor(true), sup_u),
        inf_v: clamped_floor(s.stretch_floor(false), sup_v),
        sup_u,
        sup_v,
        area_inf: clamped_floor(s.area_floor(), sup_u * sup_v),
    }
}

/// Check 3 for every lane: `|S(P(tᵢ)) − C(tᵢ)|` at the shared
/// schedule, in metres through the map, recorded as `record` says: a
/// verdict on every lane whose statement it is part of, and the
/// cross-check of check 4's closed form on a harmonic row.
#[allow(clippy::too_many_arguments)] // one parameter per named quantity
fn schedule_residuals<T: Decide>(
    record: Record,
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
        check_residual_as(
            record,
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

/// **An escape's positive part**: `max(gap, 0)`, the one home of the
/// one-sided containment gates in this file (the iso rows'
/// `pcurve_iso_domain`).
/// Containment is one-sided: a box or a parameter inside its bound by
/// any amount is contained, and the clearance between two conservative
/// boxes is nothing built, so only the escape is decided against the
/// band. NaN propagates ([`Real::max`]).
///
/// Metered through an infinite sup arm (a sup whose quotient
/// overflows, or a weight a refinement rounded to zero), a contained
/// gap is `0·∞ = NaN` and refuses, fail-loud
/// (`escape_tests::a_contained_gap_through_an_infinite_arm_refuses`).
fn escape<T: Real>(gap: T) -> T {
    gap.max(T::zero())
}

/// **The fitted lane's four checks** (M6-2), in the same fixed order as
/// the closed-form lane's — what differs is check 1's admission rule
/// and check 4's mechanism.
///
/// 1. **Lane**: a SPLINE chart. An analytic chart holds no fitted
///    image: its image of a carrier with no closed form is the
///    projected one. The carrier's kind decides nothing here.
/// 2. **Interval**: the same forward-span check.
/// 3. **Schedule**: identical, and shared code.
/// 4. **Envelope**: the hull bound on `|S(P(t)) − C(t)|` over the
///    image's domain, by the tensor composite against this chart alone
///    ([`EnvelopeStatement::MapResidualComposite`]), re-derived here at
///    rest, never trusted from storage. A spline carrier enters as its
///    own net, a closed-form one as its enclosing net
///    (`crate::carrier_net`). The row reads no mate surface: an
///    intersection carrier's uniqueness tube is the edge's certificate
///    (C2). `lane` is the fitted door that derives it; `None` refuses
///    HERE ([`PcurveCertifyError::FittedLaneUnsupported`]) and nowhere
///    earlier: checks 1–3 read no door, so their verdicts are the same
///    at every scalar.
fn run_fitted_checks<T: Decide>(
    image: &Arc<NurbsCurve2<T>>,
    t0: T,
    t1: T,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
    lane: Option<crate::FittedLane<T>>,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    // ---- Check 1: the lane. ----
    if surface.is_placeholder_chart() {
        return Err(PcurveCertifyError::PlaceholderChart);
    }
    if crate::edge_nurbs::is_analytic(surface) {
        return Err(PcurveCertifyError::ImageMismatch {
            image: PcurveKind::Fitted,
            why: "an analytic chart holds no fitted-grade image: its image of a carrier \
                  with no closed form is the projected one",
        });
    }
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

    // ---- Check 3: the schedule, in metres through the map. ----
    let mut max_residual = T::zero();
    schedule_residuals(
        Record::Verdict,
        &pcurve,
        t0,
        t1,
        carrier,
        surface,
        band,
        &mut max_residual,
    )?;

    // ---- Check 4: the envelope, RE-DERIVED. ----
    let lane = lane.ok_or(PcurveCertifyError::FittedLaneUnsupported { scalar: T::NAME })?;
    let hull_sup = lane.fitted_certificate(carrier, image, surface, (t0, t1), band)?;
    // The envelope is banded exactly as the closed-form lane's is, and
    // for the same reason: a certificate whose own bound exceeds ε is
    // not a certificate. It is NOT folded into `max_residual` (the
    // sampled max and the sup bound stay separate statements).
    envelope_verdict(hull_sup, band, || PcurveCheck::Envelope)?;

    Ok(PcurveCertificate::closed(
        CERT_SAMPLES,
        max_residual,
        hull_sup,
        EnvelopeStatement::MapResidualComposite,
    ))
}

/// The arc-rim class' sub-arc in the scalar, and its one decided
/// margin: the ONE spelling of `h = angle / spans` and `cos(h/2)`, which
/// both the margin and `Ĉ`'s tangent points read.
struct ArcRimWeight<T> {
    /// The sub-arc angle `h`.
    h: T,
    /// `cos(h/2)`, the rational-quadratic arc's interior weight.
    cos_half: T,
    /// The chart column's held interior weight `w` (its second, the
    /// first sub-arc's; the pattern check has read every other as `1`
    /// or this one on the value channel) less `cos(h/2)`. A column
    /// built over the same angle as the circle, in the scalar, is
    /// `w − w`: at `Sym` a symbolic Zero, which the weight held as an
    /// `f64` constant (`from_f64` of its value) is not. Poison for a
    /// column with fewer than two weights.
    margin: T,
}

/// **The arc pattern's interior weight at unit end weight**: `w₁/w₀`
/// on the column's held weights. A rational curve is invariant under
/// one positive scale of all its weights, so a column `c, c·w, c, …`
/// is the arc `1, w, 1, …`, and the ratio is the `w` the pattern
/// states. `None` for a column with fewer than two weights.
fn arc_rim_ratio<T: Real>(column: &NurbsCurve3<T>) -> Option<T> {
    let w = column.weights();
    Some(*w.get(1)? / *w.first()?)
}

fn arc_rim_weight<T: SpanLocate>(
    column: &NurbsCurve3<T>,
    angle: T,
    spans: usize,
) -> ArcRimWeight<T> {
    #[allow(clippy::cast_precision_loss)]
    let h = angle / T::from_f64(spans as f64);
    let (_, cos_half) = (h * T::from_f64(0.5)).sin_cos();
    let margin = match arc_rim_ratio(column) {
        Some(w) => w - cos_half,
        None => T::from_f64(f64::NAN),
    };
    ArcRimWeight {
        h,
        cos_half,
        margin,
    }
}

/// **The turn a chart column's arc weight states**: `m` sub-arcs of
/// `h = 2·acos(w)` each, `w` the column's interior weight. The inverse
/// of [`arc_rim_weight`]'s `w = cos(h/2)`, read by the arc rim's measured
/// image (`edge_nurbs::arc_rim_measured`). `None` for a column with fewer
/// than two weights, or whose weight is not decided below 1: a
/// polynomial column (`w = 1`) states no turn.
pub(crate) fn arc_rim_turn<T: Decide>(
    column: &NurbsCurve3<T>,
    spans: usize,
    band: Band,
) -> Option<T> {
    let w = arc_rim_ratio(column)?;
    match decide("pcurve_iso_arc_weight", Margin::of(T::one() - w), band) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) | Err(_) => return None,
    }
    #[allow(clippy::cast_precision_loss)]
    let m = T::from_f64(spans as f64);
    Some(T::from_f64(2.0) * w.acos() * m)
}

/// **An arc rim's sub-arc count**: one per span of its `breaks`.
#[must_use]
pub fn iso_arc_spans(breaks: &KnotVector) -> usize {
    breaks.control_count().saturating_sub(1)
}

/// **The ARC-RIM iso class** (M8-3) — certification of a
/// [`Pcurve::IsoArc`], to the same bar as every other minted pcurve.
///
/// The four checks in the same fixed order as [`run_iso_checks`]; only
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
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    // ---- Check 1: the certified lane. ----
    // The iso lane is the SPLINE chart's — an approximating surface's
    // fit is one, on the same `(u, v)`, so it enters here rather than
    // being refused as an unimplemented chart.
    let Some(payload) = surface.spline_chart() else {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: surface.kind(),
        });
    };
    if surface.is_placeholder_chart() {
        return Err(PcurveCertifyError::PlaceholderChart);
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
    // **The edge's span lies in the image's arc**, `at0 ≤ t0` and
    // `t1 ≤ at0 + angle`, each decided in metres along the arc. The
    // envelope below bounds the miss over the image's own arc, where
    // `Ĉ(g(t)) = C(t)`; off it the map clamps and the envelope holds
    // nothing. A span inside the band of an arc end reads as on it.
    for room in [t0 - at0, at0 + angle - t1] {
        match decide(
            "pcurve_iso_arc_span",
            Margin::metered(room, param_rate(carrier)),
            band,
        )
        .map_err(|cause| PcurveCertifyError::Escalated {
            check: PcurveCheck::ParamSpan,
            sample: 0,
            cause,
        })? {
            Sign::Positive | Sign::Zero => {}
            Sign::Negative => {
                return Err(PcurveCertifyError::IsoUnsupported {
                    what: "an arc rim whose edge runs past its image's arc — the arc-rim \
                           class bounds the miss over the image's own arc, and its map \
                           holds nothing off it",
                });
            }
        }
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
    schedule_residuals(
        Record::Verdict,
        &pcurve,
        t0,
        t1,
        carrier,
        surface,
        band,
        &mut max_residual,
    )?;

    // ---- Check 4: the rational control-difference hull. ----
    let esc = |cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::Envelope,
        sample: 0,
        cause,
    };
    let bad = |what: &'static str| PcurveCertifyError::IsoUnsupported { what };
    let (stretch_u, stretch_v) = nurbs_stretch_bounds(payload);
    // The chart's OWN domain (`side_of`'s note, #327), as the chart
    // holds it.
    let (cu0, cu1) = payload.knot_values_u().domain();
    let (cv0, cv1) = payload.knot_values_v().domain();
    // The moving channel is u and the fixed one v (the cap class'
    // geometry).
    let Some((end, slack_v)) = side_of(
        p0.y,
        cv0,
        cv1,
        stretch_v,
        stretch_v.to_meters(pd.y.abs()),
        band,
        &esc,
    )?
    else {
        return Err(bad(
            "an arc rim on a row that is not a chart boundary — the arc-rim class compares \
             the chart's boundary column against the circle's rational-quadratic form, and \
             no construction mints a rim off a boundary",
        ));
    };
    let b = crate::nurbs_iso::boundary_iso_v(payload, end)
        .map_err(|source| PcurveCertifyError::ChartRow { source })?;
    // --- The construction's EXACT structure (C6). ---
    let spans = iso_arc_spans(breaks);
    if spans == 0 || b.knots_f64_interim().degree() != 2 {
        return Err(bad(
            "an arc rim whose chart column is not a quadratic — the rational-quadratic \
             arc construction is the only one this class certifies",
        ));
    }
    // The column's knots as the chart holds them, against the uniform
    // sub-arc breaks over the chart's own held domain, both in `T`.
    let kn = b.knot_values().values();
    #[allow(clippy::cast_precision_loss)]
    let expected: Vec<T> = {
        let m = T::from_f64(spans as f64);
        let mut v = vec![cu0, cu0, cu0];
        for k in 1..spans {
            #[allow(clippy::cast_precision_loss)]
            let t = cu0 + (cu1 - cu0) * T::from_f64(k as f64) / m;
            v.push(t);
            v.push(t);
        }
        v.extend([cu1, cu1, cu1]);
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
        knot_dev = knot_dev.max((*a - *b).abs());
    }
    match decide(
        "pcurve_iso_boundary",
        Margin::metered_sup(knot_dev, stretch_u),
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
    let slack_knots = stretch_u.to_meters(knot_dev);
    let bw = b.weights_f64_interim();
    if bw.len() != 2 * spans + 1 {
        return Err(bad(
            "an arc rim whose chart column weights are not the arc pattern",
        ));
    }
    // The pattern is `c, c·w, c, …, c` for one positive scale `c`: one
    // scale leaves the rational basis unchanged, so the column is the
    // arc `1, w, 1, …`, and `Ĉ` below, built at unit end weight, shares
    // its basis. Each half of the pattern is one constant, compared on
    // the bits as every constancy is (`constant_weight`). It reads the
    // weights' VALUE channel, not their plain constants: a held arc's
    // interior weight is `cos(h/2)` in the scalar, no plain constant at
    // `Sym`, and the pattern's equalities are value identities decided
    // at the witness only
    // (`work/intent/held-value-equalities-decided-on-the-value-channel.md`);
    // the margin below reads the column's held `w₁/w₀`.
    let ends: Vec<f64> = bw.iter().step_by(2).copied().collect();
    let interior: Vec<f64> = bw.iter().skip(1).step_by(2).copied().collect();
    if constant_weight(&ends).is_none() || constant_weight(&interior).is_none() {
        return Err(bad(
            "an arc rim whose chart column weights are not `c, c·w, c, …` with ONE \
             interior ratio — the uniform sub-arc construction",
        ));
    }
    // --- The one decided margin: `w = cos(h/2)`. ---
    let ArcRimWeight {
        h,
        cos_half,
        margin,
    } = arc_rim_weight(&b, angle, spans);
    match decide(
        "pcurve_iso_boundary",
        Margin::metered(margin, InfSpeed::new(*radius)),
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
    let Some((reversed, slack_start)) = side_of(p0.x, cu0, cu1, stretch_u, T::zero(), band, &esc)?
    else {
        return Err(bad(
            "an arc rim whose u-start is on neither domain end — the class is a \
             FULL-DOMAIN traversal of the chart's column, forward or reversed, and a \
             start that is not a domain end is neither",
        ));
    };
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
    // the harmonic class's fidelity term are.
    let far = if forward {
        (p0.x + pd.x - cu1).abs()
    } else {
        (p0.x + pd.x - cu0).abs()
    };
    let slack_affine = slack_start.max(stretch_u.to_meters(far));
    let envelope = hull + slack_v + slack_affine + slack_knots;
    envelope_verdict(envelope, band, || PcurveCheck::Envelope)?;

    Ok(PcurveCertificate::closed(
        CERT_SAMPLES,
        max_residual,
        envelope,
        EnvelopeStatement::MapResidualIsoHull,
    ))
}

/// Which boundary a banded-constant chart channel sits on, if either,
/// plus the slack the admission costs: `w` is the channel value at
/// `t0`, `(lo, hi)` the channel's own DOMAIN ends, `drift` its
/// whole-span motion bound, `arm` the stretch that meters both into
/// metres. `Zero` at `lo` → `Some((false, slack))`, the start row; at
/// `hi` → `Some((true, slack))`, the end row; two definite non-zero
/// verdicts → `None`, NOT a boundary — a value strictly inside the
/// domain or outside it, which this decider does not distinguish. A
/// decider, not a refusal: what a non-boundary value means differs
/// per class (the seam class meters it against the domain and
/// collapses the row inside; the cap and arc classes have no
/// construction off a boundary), so each call site owns its own text.
/// Escalations escalate. Shared by the iso-line and iso-arc classes.
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
    arm: SupSpeed<T>,
    drift: T,
    band: Band,
    esc: &impl Fn(Indeterminate) -> PcurveCertifyError,
) -> Result<Option<(bool, T)>, PcurveCertifyError> {
    if let Sign::Zero = decide(
        "pcurve_iso_boundary",
        Margin::metered_sup(w - lo, arm),
        band,
    )
    .map_err(esc)?
    {
        return Ok(Some((false, arm.to_meters((w - lo).abs()) + drift)));
    }
    if let Sign::Zero = decide(
        "pcurve_iso_boundary",
        Margin::metered_sup(w - hi, arm),
        band,
    )
    .map_err(esc)?
    {
        return Ok(Some((true, arm.to_meters((w - hi).abs()) + drift)));
    }
    Ok(None)
}

/// **The iso lane's four checks** (M6-3), same fixed order as the
/// closed-form lane's. What differs: check 1 admits any described-NURBS
/// chart — the blanket non-rational gate it used to carry came off in
/// M8-3, which moved the rationality hypothesis into the class arms of
/// check 4, where it is load-bearing per class (the seam class needs
/// strictly positive weights and one shared spline space; its LINE limb
/// needs one constant weight, for linear precision). Check 4's
/// sup bound is the traversed row's control-difference hull
/// ([`EnvelopeStatement::MapResidualIsoHull`]) with the banded
/// axis/side/domain snap slacks folded in — the harmonic lane's
/// fidelity idiom transposed. Every slack is exactly zero on the
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
    band: Band,
) -> Result<PcurveCertificate<T>, PcurveCertifyError> {
    // ---- Check 1: the certified lane. ----
    // The iso lane is the SPLINE chart's — an approximating surface's
    // fit is one, on the same `(u, v)`, so it enters here rather than
    // being refused as an unimplemented chart.
    let Some(payload) = surface.spline_chart() else {
        return Err(PcurveCertifyError::UnsupportedChart {
            chart: surface.kind(),
        });
    };
    if surface.is_placeholder_chart() {
        return Err(PcurveCertifyError::PlaceholderChart);
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
    schedule_residuals(
        Record::Verdict,
        &pcurve,
        t0,
        t1,
        carrier,
        surface,
        band,
        &mut max_residual,
    )?;

    // ---- Check 4: the control-difference hull envelope, by class. ----
    let esc = |cause| PcurveCertifyError::Escalated {
        check: PcurveCheck::Envelope,
        sample: 0,
        cause,
    };
    let (stretch_u, stretch_v) = nurbs_stretch_bounds(payload);
    let du_extent = Margin::metered_sup(pl.x.abs() * span, stretch_u);
    let dv_extent = Margin::metered_sup(pl.y.abs() * span, stretch_v);
    let u_moves = !matches!(
        decide("pcurve_iso_axis_u", du_extent, band).map_err(esc)?,
        Sign::Zero
    );
    let v_moves = !matches!(
        decide("pcurve_iso_axis_v", dv_extent, band).map_err(esc)?,
        Sign::Zero
    );
    let envelope = match (u_moves, v_moves) {
        // The SEAM class: u banded-constant, v traverses the carrier's
        // own parameter. With `u*` the row's parameter (a domain end
        // for a boundary row, `u_start` itself for a collapsed one):
        // sup |S(P(t)) − C(t)| ≤
        //   |S(u(t), v(t)) − S(u*, v(t))|     (u snap + drift slack)
        // + |S(u*, v(t)) − B(v(t))|           (exactly 0: B IS S(u*, ·))
        // + |B(v(t)) − C(v(t))|               (control hull, same basis)
        // + |C(v(t)) − C(t)|                  (parameter-map slack).
        (false, true) => seam_envelope(
            payload,
            (p0, pl),
            (t0, t1),
            carrier,
            (stretch_u, stretch_v),
            du_extent.value(),
            band,
            &esc,
        )?,
        // The CAP class: `v` banded-constant, `u` traversing. A row of
        // the chart is a column of its transpose, so this is the seam
        // class run there: a line rim or a spline row, on a boundary
        // row (a control-net copy) or an interior one (the collapse).
        (true, false) => seam_envelope(
            &payload.transposed(),
            (Point2::new(p0.y, p0.x), Vec2::new(pl.y, pl.x)),
            (t0, t1),
            carrier,
            (stretch_v, stretch_u),
            dv_extent.value(),
            band,
            &esc,
        )?,
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
    envelope_verdict(envelope, band, || PcurveCheck::Envelope)?;

    Ok(PcurveCertificate::closed(
        CERT_SAMPLES,
        max_residual,
        envelope,
        EnvelopeStatement::MapResidualIsoHull,
    ))
}

/// The column `S(u_start, ·)` a seam-class image traverses, and the
/// slack its fixed channel pays: a boundary column is a control-net
/// COPY and pays the boundary snap `|u_start − side|·stretch`; an
/// interior column is the de Boor COLLAPSE at `u_start` itself, so
/// there is nothing to snap and the slack is the channel's drift alone.
fn traversed_column<T: Decide>(
    payload: &geom::NurbsSurface<T>,
    u_start: T,
    stretch_u: SupSpeed<T>,
    du_drift: T,
    band: Band,
    esc: &impl Fn(Indeterminate) -> PcurveCertifyError,
) -> Result<(NurbsCurve3<T>, T), PcurveCertifyError> {
    let (cu0, cu1) = payload.knot_values_u().domain();
    Ok(
        match side_of(u_start, cu0, cu1, stretch_u, du_drift, band, esc)? {
            Some((end, slack_u)) => (
                crate::nurbs_iso::boundary_iso_u(payload, end)
                    .map_err(|source| PcurveCertifyError::ChartRow { source })?,
                slack_u,
            ),
            None => {
                // Not a boundary: inside the domain it is a column
                // the collapse bounds; outside it the collapse is a
                // span's polynomial EXTENSION, which the chart does
                // not have — metered through the same stretch the
                // boundary decide used, refused typed.
                let outside = escape((cu0 - u_start).max(u_start - cu1));
                match decide(
                    "pcurve_iso_domain",
                    Margin::metered_sup(outside, stretch_u),
                    band,
                )
                .map_err(esc)?
                {
                    Sign::Zero => {}
                    Sign::Positive | Sign::Negative => {
                        return Err(PcurveCertifyError::IsoUnsupported {
                            what: "the iso line's fixed channel sits outside the chart's \
                                   u domain — not a boundary column, and the collapsed row \
                                   bounds the chart on its domain only",
                        });
                    }
                }
                let row =
                    crate::nurbs_iso::interior_iso_u(payload, u_start).map_err(
                        |error| match error {
                            crate::nurbs_iso::IsoRowError::WeightsNotSeparable { .. } => {
                                PcurveCertifyError::IsoUnsupported {
                                    what: "an interior column of a chart whose weight net \
                                       varies in both directions — the collapsed row's \
                                       weights are computed, not structure, so no \
                                       carrier shares its rational space; the composite \
                                       route is banked",
                                }
                            }
                            crate::nurbs_iso::IsoRowError::Structure { source } => {
                                PcurveCertifyError::ChartRow { source }
                            }
                            crate::nurbs_iso::IsoRowError::Interior { .. }
                            | crate::nurbs_iso::IsoRowError::Escalated { .. } => unreachable!(
                                "interior_iso_u decides nothing and refuses no parameter"
                            ),
                        },
                    )?;
                (row, du_drift)
            }
        },
    )
}

/// **The SEAM class's envelope**: `u` banded-constant, `v` traversing
/// the carrier's own parameter, on `payload`'s chart (run on the
/// transposed chart, it is the cap class's: a row is a column of the
/// transpose). With `u*` the column's parameter (a domain end for a
/// boundary column, `u_start` itself for a collapsed one):
/// sup |S(P(t)) − C(t)| ≤
///   |S(u(t), v(t)) − S(u*, v(t))|     (u snap + drift slack)
/// + |S(u*, v(t)) − B(v(t))|           (exactly 0: B IS S(u*, ·))
/// + |B(v(t)) − C(v(t))|               (control hull, same basis)
/// + |C(v(t)) − C(t)|                  (parameter-map slack).
#[allow(clippy::too_many_arguments)]
fn seam_envelope<T: Decide>(
    payload: &geom::NurbsSurface<T>,
    (p0, pl): (Point2<T>, Vec2<T>),
    (t0, t1): (T, T),
    carrier: &Curve3<T>,
    (stretch_u, stretch_v): (SupSpeed<T>, SupSpeed<T>),
    du_drift: T,
    band: Band,
    esc: &impl Fn(Indeterminate) -> PcurveCertifyError,
) -> Result<T, PcurveCertifyError> {
    let span = t1 - t0;
    // **The LINE-carrier limb** (#388): a straight ruling
    // edge whose carrier was promoted to `Curve3::Line`
    // upstream (D7 curve recognition) claims the same
    // boundary column a spline seam claims, and the bound is
    // a linear-precision hull. With `B` the traversed
    // column (constant weights, gated below), `t(v)` the
    // affine inverse of the image's `v(t)`, and
    // `M(v) := C(t(v))` — affine in `v`, a line composed
    // with an affine map — the B-spline basis reproduces `M`
    // exactly on the Greville abscissae, so on the column's
    // own knot domain
    //   |B(v) − M(v)| = |Σ Nᵢ(v)·(bᵢ − M(ξᵢ))|
    //                 ≤ maxᵢ |bᵢ − M(ξᵢ)|   (the hull),
    // and sup |S(P(t)) − C(t)| ≤ slack_u + hull + the banded
    // domain-excursion fold — each term an inequality, no
    // sampling. A RATIONAL column (`constant_weight` answers `None`)
    // refuses typed: linear precision is a polynomial-basis fact.
    if let Curve3::Line { origin, dir } = carrier {
        let u_start = p0.x + pl.x * t0;
        let (b, slack_u) = traversed_column(payload, u_start, stretch_u, du_drift, band, esc)?;
        if b.constant_weight().is_none() {
            return Err(PcurveCertifyError::IsoUnsupported {
                what: "a LINE seam on a RATIONAL chart column: the Greville hull \
                           is a linear-precision fact and the rational basis has none \
                           — a line ruling whose column is rational needs its line \
                           re-expressed in that column's own space, the arc-rim \
                           class's construction",
            });
        }
        let (p, kn) = (b.knots_f64_interim().degree(), b.knot_values().values());
        let mut hull = T::zero();
        // `pl.y ≠ 0` is this class's own classification: the
        // `(false, true)` arm means the v channel MOVES, so
        // the `t_at` division is over a nonzero slope by
        // construction. The hull runs over the FULL column —
        // for an image trimmed to part of the domain that is
        // a superset bound: sound and conservative, and no
        // measured caller exercises the partial case.
        for (i, cp) in b.control().iter().enumerate() {
            // The Greville abscissa of the held knots, summed in
            // order from the first (`f64`'s sum from `-0.0` is that).
            let window = &kn[i + 1..=i + p];
            #[allow(clippy::cast_precision_loss)]
            let xi = window[1..].iter().fold(window[0], |a, k| a + *k) / T::from_f64(p as f64);
            let t_at = (xi - p0.y) / pl.y;
            hull = hull.max((*cp - (*origin + *dir * t_at)).norm());
        }
        // Domain containment, the siblings' gate verbatim on
        // this class's moving channel: the hull bound holds on
        // the column's own knot domain only.
        let v_at_0 = p0.y + pl.y * t0;
        let v_at_1 = p0.y + pl.y * t1;
        let (d0, d1) = b.knot_values().domain();
        let over = escape((d0 - v_at_0.min(v_at_1)).max(v_at_0.max(v_at_1) - d1));
        match decide(
            "pcurve_iso_domain",
            Margin::metered_sup(over, stretch_v),
            band,
        )
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
        return Ok(hull + slack_u + stretch_v.to_meters(over));
    }
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
    if c.weights_f64_interim()
        .iter()
        .any(|w| !w.is_finite() || *w <= 0.0)
    {
        return Err(PcurveCertifyError::IsoUnsupported {
            what: "a seam carrier with a non-positive or non-finite weight — the \
                       rational convex-hull property is exactly the hypothesis that \
                       fails there",
        });
    }
    let u_start = p0.x + pl.x * t0;
    // Which way the image runs the column: a wrap edge a
    // construction laid against the column's own direction (D1)
    // is carried by the column run back — on any exact reflection
    // of its knots — and is compared against the traversed row
    // read backwards.
    let backward = match decide(
        "pcurve_iso_seam_sense",
        Margin::metered_sup(pl.y * span, stretch_v),
        band,
    )
    .map_err(esc)?
    {
        Sign::Positive => false,
        Sign::Negative => true,
        Sign::Zero => {
            return Err(PcurveCertifyError::IsoUnsupported {
                what: "a DEGENERATE iso line (neither chart channel definitely moves \
                           over the span)",
            });
        }
    };
    // The traversed column: a boundary row is a control-net
    // COPY and pays the boundary snap `|u_start − side|·stretch`;
    // an interior column is the de Boor COLLAPSE at `u_start`
    // itself, so there is nothing to snap and the slack is the
    // channel's drift alone. Both rows are `S(u_start, ·)` in
    // the chart's own `v` space, which is all the hull below
    // ever used.
    let (b, slack_u) = traversed_column(payload, u_start, stretch_u, du_drift, band, esc)?;
    // One spline space: the knots bitwise, or — run back — the
    // carrier's knots the row's EXACT reflection (a reflection
    // rebuilt in `f64` is not that space); and the weights, read
    // the same way, either bitwise (a boundary row's, or a collapsed
    // row's with the net's weights constant along `u`) or both
    // constant (`constant_weight`: each is its own polynomial).
    let row_weights: Vec<f64> = if backward {
        b.weights_f64_interim().iter().rev().copied().collect()
    } else {
        b.weights_f64_interim().to_vec()
    };
    let shared_weights = row_weights == c.weights_f64_interim()
        || (b.constant_weight().is_some() && c.constant_weight().is_some());
    let shared_knots = if backward {
        b.knots_f64_interim()
            .is_reflection_of(c.knots_f64_interim())
    } else {
        b.knots_f64_interim().knots() == c.knots_f64_interim().knots()
            && b.knots_f64_interim().degree() == c.knots_f64_interim().degree()
    };
    if !shared_knots || !shared_weights {
        return Err(PcurveCertifyError::IsoUnsupported {
            what: "the seam carrier is not the chart's own column (its knot/weight \
                       structure differs from the traversed row's) — the hull \
                       comparison needs one spline space",
        });
    }
    // One spline space means one control count, so the backward
    // index `n − 1 − i` is in range for every `i`.
    let n = b.control().len();
    let mut hull = T::zero();
    for (i, pc) in c.control().iter().enumerate() {
        let pb = b.control()[if backward { n - 1 - i } else { i }];
        hull = hull.max((pb - *pc).norm());
    }
    // Parameter map v(t) = p0.y + pl.y·t vs the identity, or
    // vs `S − t` on a column run back (the reflection's sum, the
    // row's first knot plus the carrier's last): the difference is
    // affine, so its extremes are at the endpoints; metered through
    // the carrier's own rate bound.
    let v_at_0 = p0.y + pl.y * t0;
    let v_at_1 = p0.y + pl.y * t1;
    let along = |t: T| {
        if backward {
            b.knot_values().domain().0 + c.knot_values().domain().1 - t
        } else {
            t
        }
    };
    let slack_param = c
        .speed_upper_bound()
        .to_meters((v_at_0 - along(t0)).abs().max((v_at_1 - along(t1)).abs()));
    // Domain containment: the hull and rate bounds hold on the knot
    // domains only — the image's `v` on the row's, the span on the
    // carrier's (one domain, unless the carrier is reflected).
    let (r0, r1) = b.knot_values().domain();
    let (d0, d1) = c.knot_values().domain();
    let over = escape(
        (r0 - v_at_0.min(v_at_1))
            .max(v_at_0.max(v_at_1) - r1)
            .max(d0 - t0)
            .max(t1 - d1),
    );
    match decide(
        "pcurve_iso_domain",
        Margin::metered_sup(over, stretch_v),
        band,
    )
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
    Ok(hull + slack_u + slack_param + stretch_v.to_meters(over))
}

/// **Where a derivation places its angular phases** (C4): over the
/// edge's span, in its channel's principal window ([`Window`]: from
/// [`ANGLE_WINDOW_START`] for an angle, [`LATITUDE_WINDOW_START`] for a
/// sphere's latitude). It
/// carries the span's midpoint; a phase is the angle its channel reads
/// at the image's own origin, placed so that the channel lies in the
/// window at that midpoint ([`Placement::phase`]). Where the carrier's
/// parameter starts then moves no stored image: a carrier stated whole
/// periods off reads the same placed image.
///
/// **What holds across scalars.** Each scalar names the copy its own
/// reading of the marks decides, and those can differ just outside a
/// mark's band, where one scalar's margin is decided and another's
/// enclosure still straddles the mark and takes the lower branch: an
/// `f64` row and its `Interval` twin may then sit a whole turn apart.
/// Either is a placement of the same curve, whole turns from the other,
/// and certified as itself; what does not hold is that two scalars'
/// rows are the same numbers there, and no reader compares rows across
/// scalars.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Placement<T> {
    mid: T,
    levers: Levers<T>,
    /// The span the placement reads, which an open carrier's incidence
    /// test samples ([`incidence::incident`]).
    span: (T, T),
}

/// **Which angle a placement names**, and so the window it is named in
/// and the lever its marks are read at ([`Placement`]'s one lever rule).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Along {
    /// The chart's azimuth, about its axis.
    Azimuth,
    /// A torus's tube angle, about its core circle.
    Tube,
    /// A sphere's latitude.
    Latitude,
}

/// **The levers a placement's marks are read at**, one per angle, each
/// a metre length where the angle moves the geometry (D4), with one home
/// ([`Placement::over`]): the azimuth at the farthest the carrier reaches
/// from the chart's axis at the span's ends and midpoint (so an edge
/// through the axis at its midpoint, a ruling across a cone's apex, still
/// has the length its ends give it), and, for a span wholly on the axis (a
/// pole or apex vertex read over no length), at the farthest its carrier
/// reaches over its own interval; only a carrier wholly on the axis has
/// none.
///
/// **What the azimuth lever is.** A point's azimuth rate is its distance
/// from the axis, so over the span the rate runs from its inf (zero, on
/// an edge through the axis) to its sup, the span's largest reach. Three
/// samples of the reach are a floor on that sup, not on the rate
/// everywhere: the lever is the least the span's farthest point moves per
/// radian. That is the side a placement needs and no more, since every
/// outcome of its marks names a placement of the same curve: either side
/// of a decided mark, or the tie-break on one in band, are whole turns
/// apart ([`Placement`], *What holds across scalars*). The lever only
/// sets how wide the band about each mark reads, and a floor never
/// decides a mark apart that the span's largest movement would leave in
/// band. The tube angle at the torus's minor radius; the latitude at the
/// sphere's radius. A chart without one of those angles carries `1` in
/// its slot, which no arm reads (no arm places a tube angle off a torus,
/// or a latitude off a sphere).
#[derive(Clone, Copy, Debug)]
struct Levers<T> {
    azimuth: T,
    tube: T,
    latitude: T,
}

impl Along {
    /// The window the angle is read and named in.
    fn window(self) -> Window {
        match self {
            Along::Azimuth | Along::Tube => Window::Angle,
            Along::Latitude => Window::Latitude,
        }
    }
}

impl<T: Real> Placement<T> {
    /// The span's midpoint.
    pub(crate) fn mid(self) -> T {
        self.mid
    }
}

impl<T: Decide> Placement<T> {
    /// Placement of `carrier`'s image on `surface` over the edge's span
    /// `[t0, t1]`, its levers read there ([`Levers`]).
    pub(crate) fn over((t0, t1): (T, T), carrier: &Curve3<T>, surface: &Surface<T>) -> Self {
        let mid = (t0 + t1) * T::from_f64(0.5);
        let azimuth = projected::chart_frame(surface).map_or_else(T::one, |(origin, e)| {
            let reach = |t: T| {
                let q = carrier.eval(t) - origin;
                (q - e[2] * q.dot(e[2])).norm()
            };
            let at = |(a, b): (T, T)| {
                reach(a)
                    .max(reach((a + b) * T::from_f64(0.5)))
                    .max(reach(b))
            };
            let over_span = at((t0, t1));
            // A span on the axis (a vertex at a pole or the apex, read over
            // no length) still has its carrier's direction: the reach of
            // the carrier over its own interval. Only a carrier wholly on
            // the axis has none.
            if (T::one() / over_span).is_bounded() {
                over_span
            } else {
                at(own_interval(carrier))
            }
        });
        let (tube, latitude) = match *surface {
            Surface::Torus { minor_radius, .. } => (minor_radius, T::one()),
            Surface::Sphere { radius, .. } => (T::one(), radius),
            _ => (T::one(), T::one()),
        };
        Self {
            mid,
            levers: Levers {
                azimuth,
                tube,
                latitude,
            },
            span: (t0, t1),
        }
    }

    /// Placement over the carrier's own interval ([`own_interval`]), for
    /// a caller holding no edge.
    pub(crate) fn own(carrier: &Curve3<T>, surface: &Surface<T>) -> Self {
        Self::over(own_interval(carrier), carrier, surface)
    }

    /// The phase of the `along` channel whose direction at the image's
    /// origin is `dir` and which moves by `offset(t)` from there by
    /// parameter `t`: `dir`'s angle read in the channel's window
    /// (`Azimuth::principal_in`), less the whole turns that put
    /// `phase + offset(mid)` in its window
    /// ([`Placement::turns`]); none leaves the principal angle as it was
    /// read.
    pub(crate) fn phase(
        self,
        along: Along,
        dir: Azimuth<T>,
        offset: impl FnOnce(T) -> T,
        band: Band,
    ) -> Result<T, PcurveCertifyError> {
        // The read's one jump is the cut of the window it is named in.
        let read = dir.principal_in(along.window(), band);
        let turns = self.turns(along, read + offset(self.mid), band)?;
        Ok(if turns == 0 {
            read
        } else {
            read - Period::new(T::tau()).times(turns)
        })
    }

    /// The whole turns to take off the `along` angle `at`, read at the
    /// span's midpoint, to name it in its window (the angular one, or a
    /// sphere's latitude's, [`Window`]); for an image whose channels are
    /// read piece by piece (the projected one), the turns its deck origin
    /// takes. The turns are decided through the half-period marks about
    /// the window's centre (C4: a whole-period choice a reader tells apart
    /// goes through the marks), metered at the angle's lever ([`Levers`]).
    /// A lever of zero (an edge wholly on the axis, a pole's zero-radius
    /// parallel; at `Interval`, an enclosure reaching zero) moves nothing,
    /// so every copy is the same curve and the angle as read is kept, at
    /// every scalar alike; a lever in the band reads its marks in band,
    /// where the copy is the tie-break's.
    /// On a mark (the window's cut) both copies are the image, and the
    /// lower is taken, which puts the midpoint at the window's closed end:
    /// a tie-break (D9).
    pub(crate) fn turns(self, along: Along, at: T, band: Band) -> Result<i32, PcurveCertifyError> {
        let refused = |cause| PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartWinding,
            sample: 0,
            cause,
        };
        let window = along.window();
        let lever = match along {
            Along::Azimuth => self.levers.azimuth,
            Along::Tube => self.levers.tube,
            Along::Latitude => self.levers.latitude,
        };
        // A zero lever (or an enclosure reaching zero: no bounded
        // reciprocal, at every scalar alike) is an angle that moves
        // nothing, a pole's zero-radius parallel: every copy names the same
        // points, and the angle as read is the copy (D9's tie-break).
        if !(T::one() / lever).is_bounded() {
            return Ok(0);
        }
        let centre = window.centre::<T>();
        match Period::new(T::tau()).decide(
            "pcurve_chart_placement",
            at - centre,
            |g| Margin::metered(g, InfSpeed::new(lever)),
            band,
        ) {
            Ok(k) => Ok(k),
            Err(WalkStop {
                astride: Some([lower, _]),
                ..
            }) => Ok(lower),
            // A midpoint that names no mark (its gap no finite number), or
            // one on a mark past every turn an `i32` counts: refused, never
            // read as the angle it was (fail-loud).
            Err(WalkStop {
                miss: BranchMiss::Undecided(cause),
                astride: None,
            }) => Err(refused(cause)),
            Err(WalkStop {
                miss: BranchMiss::OnMark,
                astride: None,
            }) => Err(PcurveCertifyError::PlacementTurnsExceeded),
        }
    }
}

/// A carrier's own parameter interval, for a derivation no edge spans:
/// one turn `[0, τ]` of a periodic carrier, a spline's knot domain, and
/// a unit `[0, 1]` of a line, whose image has no moving angular channel
/// but whose placement reads its lever at the interval's points
/// ([`Levers`]).
pub(crate) fn own_interval<T: Real>(carrier: &Curve3<T>) -> (T, T) {
    match carrier {
        Curve3::Line { .. } => (T::zero(), T::one()),
        Curve3::Circle { .. } | Curve3::Ellipse { .. } | Curve3::Spiric { .. } => {
            (T::zero(), T::tau())
        }
        Curve3::Nurbs(net) => {
            let (a, b) = net.domain();
            (T::from_f64(a), T::from_f64(b))
        }
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
/// The image is **placed** over the edge's span in the chart's
/// principal window (C4). A face's walk that reads it another whole
/// period over reads the row with a deck element (`topo::pcurves`),
/// never a second row. No per-sample unwrapping exists anywhere in
/// this lane.
///
/// # Errors
///
/// [`PcurveCertifyError::UnsupportedChart`] for a spline chart;
/// [`PcurveCertifyError::CarrierOffChart`] for a carrier the chart's
/// incidence test shows off it, and `Escalated`
/// ([`PcurveCheck::ChartIncidence`]) where that test lands in the sliver
/// band. A carrier the test does not show off the chart whose class
/// reading shows none of the chart's own classes takes the general lane
/// (D3's lane choice) where one is written ([`projected::has_lane`]: a
/// circle on a sphere, a spline on any analytic chart), and otherwise
/// refuses lever-only "not yet": `Escalated` ([`PcurveCheck::ChartClass`])
/// with the class reading's cause where it was in band, and
/// [`PcurveCertifyError::NoGeneralLane`] ([`LaneGap::Unwritten`]) where it
/// was definite. A circle on a cone, torus or cylinder, a line and an
/// ellipse have no general lane yet (units E2 and E4, `work/pcert/plan.md`).
/// Where the projected lane is taken, a circle's lane choice or a
/// spline's only image, the lane's own capability gap also ends "not yet":
/// `Escalated` ([`PcurveCheck::GeneralLane`]) where its sector or nappe
/// reading is in band, and [`PcurveCertifyError::NoGeneralLane`]
/// ([`LaneGap::Pose`]) where its partition finds no sector
/// (`general_lane_refusal`).
pub fn chart_pcurve<T: Decide>(
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<Pcurve<T>, PcurveCertifyError> {
    chart_image(carrier, None, surface, band)
}

/// [`chart_pcurve`] for an edge over `[t0, t1]`: the same image for
/// every class but a sphere's general circle, whose projected image is
/// partitioned over the edge's own interval rather than one turn from
/// `0`. The door a caller holding the edge's interval takes (the mint,
/// the site mint).
///
/// # Errors
///
/// As [`chart_pcurve`].
pub fn chart_pcurve_over<T: Decide>(
    carrier: &Curve3<T>,
    t0: T,
    t1: T,
    surface: &Surface<T>,
    band: Band,
) -> Result<Pcurve<T>, PcurveCertifyError> {
    chart_image(carrier, Some((t0, t1)), surface, band)
}

fn chart_image<T: Decide>(
    carrier: &Curve3<T>,
    span: Option<(T, T)>,
    surface: &Surface<T>,
    band: Band,
) -> Result<Pcurve<T>, PcurveCertifyError> {
    let place = span.map_or_else(
        || Placement::own(carrier, surface),
        |span| Placement::over(span, carrier, surface),
    );
    // The spiric's two exact images come FIRST, because the kind has no
    // `{1, cos, sin, t}` form at all: `carrier_harmonic` answers `None`
    // for it and would turn the one carrier with a closed image on two
    // charts into a blanket refusal.
    if matches!(carrier, Curve3::Spiric { .. }) {
        return spiric_chart_pcurve(carrier, surface, place, band);
    }
    // A spline carrier has no closed form on any chart: its image is the
    // projected one, after the incidence test shows it not off the chart.
    // That image is its only lane rather than a lane choice, but a sector
    // the lane cannot partition is the same capability gap as a circle's,
    // so it ends the same way ([`general_lane_refusal`]).
    if let Curve3::Nurbs(net) = carrier {
        if !projected::has_lane(carrier, surface) {
            return Err(PcurveCertifyError::UnsupportedChart {
                chart: surface.kind(),
            });
        }
        if let Incidence::Off = net_incidence(net, span, surface, band)? {
            return Err(
                OffChart("the spline carrier does not lie on the chart").refusal(surface, carrier)
            );
        }
        return projected::project(carrier, span, surface, band)
            .map(|image| Pcurve::Projected(Box::new(image)))
            .map_err(|refused| general_lane_refusal(refused, carrier, surface));
    }
    match derive_class(carrier, surface, place, band) {
        Ok((image, _)) => Ok(image),
        Err(Underived::Refused(e)) => Err(e),
        // D3's lane choice: a class not shown, on a carrier the incidence
        // test does not show off the chart, takes the general lane where
        // one is written ([`projected::has_lane`]); every other pair
        // refuses lever-only "not yet" ([`PcurveCheck::ChartClass`]):
        // with the class reading's cause where it was in band.
        Err(Underived::General { shown, undecided }) => match undecided {
            _ if projected::has_lane(carrier, surface) => {
                projected::project(carrier, span, surface, band)
                    .map(|image| Pcurve::Projected(Box::new(image)))
                    .map_err(|refused| general_lane_refusal(refused, carrier, surface))
            }
            Some(cause) => Err(incidence::class_escalated(shown, cause)),
            None => Err(PcurveCertifyError::NoGeneralLane {
                chart: surface.kind(),
                carrier: carrier.kind(),
                gap: LaneGap::Unwritten,
            }),
        },
    }
}

/// The lever of a [`LaneGap::Pose`]: the one sentence both
/// [`PcurveCertifyError::NoGeneralLane`]'s and
/// [`crate::CertifyError::NoGeneralLane`]'s Display give it.
pub(crate) const POSE_LEVER: &str = "a carrier through a pole, the cone's apex or the torus's \
     core has no one-branch image there; split the edge at it, or re-aim the chart's axis away \
     from it";

/// **Why D3's lane choice found no general image** for a carrier the
/// chart's incidence test does not show off it
/// ([`PcurveCertifyError::NoGeneralLane`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaneGap {
    /// No general lane is written for this (chart, carrier) pair
    /// ([`projected::has_lane`]).
    Unwritten,
    /// The lane is written for the pair but does not image this pose: its
    /// partition found no piece sector ([`projected::SectorChannel`])
    /// within its refinement cap.
    Pose {
        /// The piece whose sector failed.
        piece: u32,
        /// The channel that failed.
        channel: projected::SectorChannel,
    },
}

/// **The projected lane's refusal at the mint**: the one place a
/// general-lane refusal is relabelled, read from both of `chart_image`'s
/// calls of the lane (a circle's lane choice and a spline's only image),
/// and nothing in `projected` learns of either. The lane's own capability
/// gap on a carrier the incidence test does not show off the chart, its
/// partition's sector reading (and, for a cone net or circle, the nappe
/// `chart_of` reads at its midpoint), ends "not yet"
/// ([`PcurveCheck::GeneralLane`]): in band as an escalation of that
/// check, decided as [`LaneGap::Pose`]. Every other refusal `project`
/// returns stays as it is: the deck's placement (`Placement::turns`)
/// refuses only a poisoned midpoint or one past every turn an `i32`
/// counts, and the kind and chart refusals are fenced off by
/// [`projected::has_lane`] before the call.
fn general_lane_refusal<T: Real>(
    refused: PcurveCertifyError,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
) -> PcurveCertifyError {
    match refused {
        PcurveCertifyError::Escalated {
            check: PcurveCheck::Sector,
            sample,
            cause,
        } => PcurveCertifyError::Escalated {
            check: PcurveCheck::GeneralLane,
            sample,
            cause,
        },
        PcurveCertifyError::SectorRefused { piece, channel } => PcurveCertifyError::NoGeneralLane {
            chart: surface.kind(),
            carrier: carrier.kind(),
            gap: LaneGap::Pose { piece, channel },
        },
        other => other,
    }
}

/// The largest exact distance from `surface` ([`chart_distance`]) of
/// `at(t)` at [`INCIDENCE_SAMPLES`] + 1 evenly spaced parameters of
/// `[t0, t1]`, its ends included: an open carrier's sampled incidence. A
/// poisoned distance is carried to the decision rather than dropped by
/// `max`.
fn farthest_over_span<T: Real>(
    surface: &Surface<T>,
    (t0, t1): (T, T),
    at: impl Fn(T) -> Point3<T>,
) -> T {
    let n = f64::from(INCIDENCE_SAMPLES);
    (0..=INCIDENCE_SAMPLES).fold(T::zero(), |far, i| {
        let t = t0 + (t1 - t0) * T::from_f64(f64::from(i) / n);
        let d = chart_distance(surface, at(t));
        if far.is_poison() || d.is_poison() {
            d + far
        } else {
            far.max(d)
        }
    })
}

/// The incidence test of a spline carrier: [`chart_incidence`]'s, at
/// [`INCIDENCE_SAMPLES`] points of the edge's interval `span`, or of
/// the knot domain where none is given: the net past the edge's ends is
/// not the edge's.
fn net_incidence<T: Decide>(
    net: &NurbsCurve3<T>,
    span: Option<(T, T)>,
    surface: &Surface<T>,
    band: Band,
) -> Result<Incidence, PcurveCertifyError> {
    let span = span.unwrap_or_else(|| net.knot_values().domain());
    incidence_over_span(
        "pcurve_spline_chart_incident",
        surface,
        span,
        |t| net.eval(t),
        band,
    )
}

/// The two iso families of an analytic chart: `U` the `u = const`
/// lines (a cylinder's or cone's rulings, a sphere's or torus's
/// meridians), `V` the `v = const` lines (rims and parallels).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsoFamily {
    /// `u = const`.
    U,
    /// `v = const`.
    V,
}

/// Why [`chart_iso_family`] names no family.
#[derive(Clone, Debug, PartialEq)]
pub enum IsoFamilyRefusal {
    /// The chart image's derivation refused ([`chart_pcurve`]'s).
    Image(PcurveCertifyError),
    /// The cylinder arm read the carrier's radial amplitude in the band
    /// (`pcurve_chart_radial_moving`): a ruling or a moving carrier,
    /// undecided. The mint takes the ruling's form there as a structure
    /// tie-break that its envelope keeps honest; a family cannot be
    /// read off a tie-break. Its sentence is the question
    /// ([`ISO_FAMILY_CLAUSE`]) and the payload, with no ending; the join,
    /// which holds it, reads the escalation and writes its own sentence
    /// on the same clause.
    Undecided(Indeterminate),
}

/// What the chart-class reading of [`chart_iso_family`] asks of an
/// edge, after "whether": the one spelling of that question.
pub const ISO_FAMILY_CLAUSE: &str = "runs along a cylinder's ruling or around it";

impl core::fmt::Display for IsoFamilyRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Image(e) => write!(f, "{e}"),
            Self::Undecided(diag) => write!(
                f,
                concat!("whether the edge {}", geom_core::undecided!(), ": {}"),
                ISO_FAMILY_CLAUSE,
                diag.payload()
            ),
        }
    }
}

/// The iso family `carrier`'s image on `surface`'s chart lies on, read
/// off the class arm [`chart_pcurve`]'s derivation selects for it, so
/// the family is the mint's own structure. One reading is taken again:
/// where the derivation took the cylinder's ruling form, its class
/// verdict (`pcurve_chart_radial_moving`, the same margin) is re-read,
/// because the mint takes that form for an in-band verdict too, as a
/// tie-break. `None` for a planar chart, a spiric carrier, and an image
/// on no family (a cylinder's oblique section, a cone's tilted one, a
/// torus's Villarceau circle).
///
/// # Errors
///
/// [`IsoFamilyRefusal::Image`] with [`chart_pcurve`]'s refusal, and
/// [`IsoFamilyRefusal::Undecided`] where the cylinder arm's class
/// reading is in the band, which the mint settles by a tie-break.
pub fn chart_iso_family<T: Decide>(
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<Option<IsoFamily>, IsoFamilyRefusal> {
    if matches!(carrier, Curve3::Spiric { .. }) {
        return Ok(None);
    }
    let (_, derivation) = derive_harmonic(carrier, surface, Placement::own(carrier, surface), band)
        .map_err(IsoFamilyRefusal::Image)?;
    if let (Derivation::CylinderMeridian, Surface::Cylinder { axis, .. }, Some(form)) =
        (derivation, surface, carrier_harmonic(carrier))
    {
        decide(
            "pcurve_chart_radial_moving",
            Margin::of(radial_amplitude(&form, *axis)),
            band,
        )
        .map_err(IsoFamilyRefusal::Undecided)?;
    }
    Ok(match derivation {
        Derivation::CylinderMeridian
        | Derivation::ConeRuling { .. }
        | Derivation::SphereMeridian { .. }
        | Derivation::TorusMeridian { .. } => Some(IsoFamily::U),
        // A circle on a cylinder is a cross-section; any other moving
        // carrier there is an oblique section's sinusoid.
        Derivation::CylinderMoving { .. } => {
            matches!(carrier, Curve3::Circle { .. }).then_some(IsoFamily::V)
        }
        Derivation::ConeRim { .. }
        | Derivation::SphereParallel { .. }
        | Derivation::TorusParallel { .. } => Some(IsoFamily::V),
        Derivation::Plane | Derivation::FocalSection => None,
    })
}

/// [`chart_pcurve`]'s harmonic arms, answering the image together with
/// the structure selections that produced it ([`Derivation`]) — the
/// one body both the mint and check 4's re-derivation run.
fn derive_class<T: Decide>(
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    place: Placement<T>,
    band: Band,
) -> Result<(Pcurve<T>, Derivation), Underived> {
    let off_chart = |why| Underived::Refused(OffChart(why).refusal(surface, carrier));
    // A class rate read Zero: the carrier is not that class, and the
    // reading says nothing of whether it lies on the chart (a circle in
    // the plane through a torus's tube centre ⊥ the meridian reads its
    // meridional rate 0 off the torus; a great circle tilted in band at
    // `K < √2` reads its polar rate in band on the sphere). So the
    // incidence test decides, through D3's one door to the general lane:
    // off, a definite miss; not shown off, the general lane or "not yet".
    let not_this_class = || incidence::general_lane(carrier, surface, place, band, None);
    let Some(form) = carrier_harmonic(carrier) else {
        return Err(Underived::Refused(PcurveCertifyError::ImageMismatch {
            image: PcurveKind::Harmonic,
            why: "a spline carrier has no {1, cos, sin, t} form; its image is the projected one",
        }));
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
            Ok((
                Pcurve::free_harmonic(
                    Point2::new(w.dot(u_ref), w.dot(v_ref)),
                    chart(form.a),
                    chart(form.b),
                    chart(form.l),
                ),
                Derivation::Plane,
            ))
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
            let moving = radial_amplitude(&form, axis);
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
                // The meridian's constant azimuth is read here only: a
                // moving carrier's centre may sit on the axis, where its
                // radial part names no direction to place.
                Ok(Sign::Zero) | Err(_) => Ok((
                    Pcurve::Harmonic {
                        u: Channel::fixed_angle(place.phase(
                            Along::Azimuth,
                            Azimuth::of(w_r.dot(cv), w_r.dot(u_ref)),
                            |_| T::zero(),
                            band,
                        )?),
                        v: Channel::Free {
                            c: w.dot(axis),
                            a: form.a.dot(axis),
                            b: form.b.dot(axis),
                            l: form.l.dot(axis),
                        },
                    },
                    Derivation::CylinderMeridian,
                )),
                Ok(Sign::Positive | Sign::Negative) => {
                    // The moving case: radial(t) = a_r·cos t + b_r·sin t
                    // (a linear radial part would not close a chart
                    // circle; certification refuses it through the
                    // residual). α is the azimuth of `a_r`; β is the
                    // orientation of (a_r, b_r) against the chart's own
                    // frame, a named trilean metered at the radius.
                    let orient = a_r.cross(b_r).dot(axis);
                    let beta = match decide(
                        "pcurve_chart_orientation",
                        Margin::over_lever(orient, radius),
                        band,
                    ) {
                        Ok(sign) => winding_of(sign),
                        Err(cause) => {
                            return Err(incidence::general_lane(
                                carrier,
                                surface,
                                place,
                                band,
                                Some(cause),
                            ));
                        }
                    };
                    let alpha = place.phase(
                        Along::Azimuth,
                        Azimuth::of(a_r.dot(cv), a_r.dot(u_ref)),
                        |m| angle_rate::<T>(beta) * m,
                        band,
                    )?;
                    let (a, b) = cylinder_rim_axial(form.a.dot(axis), form.b.dot(axis), band);
                    Ok((
                        Pcurve::Harmonic {
                            u: Channel::Angle {
                                phase: alpha,
                                winding: beta,
                            },
                            v: Channel::Free {
                                c: w.dot(axis),
                                a,
                                b,
                                l: form.l.dot(axis),
                            },
                        },
                        Derivation::CylinderMoving { beta },
                    ))
                }
            }
        }
        // The cone chart (M6-3, walk row 4): closed forms for the three
        // classes a cone carries at rest — RIM circles (⊥ axis,
        // centred on it: azimuth `α + β·t`, slant constant), RULING
        // lines (azimuth constant, slant affine — `v` is a length, so
        // its slope is unconstrained) and tilted SECTION ellipses
        // (azimuth-non-harmonic: the [`Pcurve::FocalSection`] image).
        // Derivations are structure selection; the full residual
        // certification follows every derivation and is what makes a
        // wrong pick fail loudly.
        Surface::Cone {
            apex,
            axis,
            half_angle,
            u_ref,
        } => {
            let c_ha = half_angle.cos();
            let cv = axis.cross(u_ref);
            match *carrier {
                Curve3::Line { origin, dir } => {
                    // Incidence first: a line the cone does not hold
                    // refuses as off it wherever its parameter starts,
                    // and none reaches the nappe readings below, whose
                    // in-band arm is a lane choice.
                    incidence::incident(carrier, surface, place, band)?;
                    // The RULING class. The azimuth branch IS the
                    // nappe: a point at v < 0 sits at spatial azimuth
                    // u + π (the mirror nappe — the Seam docs' case),
                    // so `u = azimuth(radial · sign v)`, the sign read
                    // from whichever height datum is definite (the
                    // anchor's, else the slope's; a line with neither
                    // is height-constant — no ruling of any cone).
                    // `v(t)` is the line's projection on that azimuth's
                    // generator `ĝ`, affine in `t`, so the image misses
                    // the line by its distance from the generator.
                    let w = origin - apex;
                    let radial = |v: Vec3<T>| v - axis * v.dot(axis);
                    let (h0, hs) = (w.dot(axis), dir.dot(axis));
                    // Read off the origin, the nappe names the same image
                    // from any origin on the line: `radial · sign h` is one
                    // direction at every point of a ruling through the
                    // apex, so where the parameter starts moves nothing.
                    let (r_ref, h_sign, anchored) = match decide(
                        "pcurve_cone_chart_nappe",
                        Margin::of(h0),
                        band,
                    )
                    .map_err(|cause| {
                        incidence::general_lane(carrier, surface, place, band, Some(cause))
                    })? {
                        Sign::Positive => (radial(w), Nappe::Pos, true),
                        Sign::Negative => (radial(w), Nappe::Neg, true),
                        Sign::Zero => {
                            match geom_core::k_stats::decide_flagged_reported(
                                "pcurve_cone_chart_nappe",
                                hs,
                                band,
                                "F13",
                            )
                            .map_err(|cause| {
                                incidence::general_lane(carrier, surface, place, band, Some(cause))
                            })? {
                                Decided {
                                    sign: Sign::Positive,
                                    ..
                                } => (radial(dir), Nappe::Pos, false),
                                Decided {
                                    sign: Sign::Negative,
                                    ..
                                } => (radial(dir), Nappe::Neg, false),
                                // The apex plane meets the cone at
                                // its radius `h·tan α = 0`: the apex
                                // alone.
                                Decided {
                                    sign: Sign::Zero,
                                    margin,
                                } => {
                                    return Err(Underived::Refused(
                                        OffChart(
                                            "the line is not shown off the plane through the \
                                             apex ⊥ the axis, which meets the cone only at the \
                                             apex",
                                        )
                                        .with_margin(
                                            OffMargin::InBand(margin),
                                            surface,
                                            carrier,
                                        ),
                                    ));
                                }
                            }
                        }
                    };
                    let r_dir = r_ref * h_sign.value();
                    let alpha = place.phase(
                        Along::Azimuth,
                        Azimuth::of(r_dir.dot(cv), r_dir.dot(u_ref)),
                        |_| T::zero(),
                        band,
                    )?;
                    let g = ruling_generator(axis, half_angle, r_dir);
                    Ok((
                        Pcurve::Harmonic {
                            u: Channel::fixed_angle(alpha),
                            v: Channel::Free {
                                c: w.dot(g),
                                a: T::zero(),
                                b: T::zero(),
                                l: dir.dot(g),
                            },
                        },
                        Derivation::ConeRuling {
                            anchored,
                            nappe: h_sign,
                        },
                    ))
                }
                Curve3::Circle { center, .. } => {
                    // Rim class: carrier plane ⊥ axis (a, b axial parts
                    // zero — already metres) and centred on the axis.
                    // Those are the only circles on a right circular
                    // cone: a plane section at angle β to the axis has
                    // eccentricity cos β / cos α, zero only at β = 90°,
                    // and the section ⊥ the axis at height `h` is the
                    // circle of radius `|h|·tan α` centred on it.
                    //
                    // The two gates read first-order quantities — the
                    // tilt at the radius, the centre's offset — while a
                    // circle tilted off a rim leaves the cone only to
                    // second order (`ρθ²`), and on a wide cone a rim's
                    // in-band tilt moves its centre by `tan α` times
                    // the axial margin. So a failure is not a verdict:
                    // the incidence test decides it.
                    let (aa, ba) = (form.a.dot(axis), form.b.dot(axis));
                    let radial = |v: Vec3<T>| v - axis * v.dot(axis);
                    let w_r = radial(center - apex);
                    let rim = match decide(
                        "pcurve_cone_chart_axial",
                        Margin::of(harmonic_sup_scalar(aa, ba)),
                        band,
                    )
                    .map_err(|cause| {
                        incidence::general_lane(carrier, surface, place, band, Some(cause))
                    })? {
                        Sign::Zero => decide(
                            "pcurve_cone_chart_centered",
                            Margin::norm3(w_r),
                            band,
                        )
                        .map_err(|cause| {
                            incidence::general_lane(carrier, surface, place, band, Some(cause))
                        })?,
                        tilted @ (Sign::Positive | Sign::Negative) => tilted,
                    };
                    match rim {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(incidence::general_lane(
                                carrier, surface, place, band, None,
                            ));
                        }
                    }
                    let a_r = radial(form.a);
                    // Slant from the axial height: v·cos α = h. A
                    // mirror-nappe rim has v < 0 (Seam docs), and its
                    // chart azimuth is the SPATIAL azimuth + π
                    // (radial(u)·v flips sign) — the same nappe rule
                    // as the ruling arm, decided on the height.
                    let h = (center - apex).dot(axis);
                    let nappe = match geom_core::k_stats::decide_reported(
                        "pcurve_cone_chart_nappe",
                        Margin::of(h),
                        band,
                    )
                    .map_err(|cause| {
                        incidence::general_lane(carrier, surface, place, band, Some(cause))
                    })? {
                        Decided {
                            sign: Sign::Positive,
                            ..
                        } => Nappe::Pos,
                        Decided {
                            sign: Sign::Negative,
                            ..
                        } => Nappe::Neg,
                        // A rim whose height is not shown off the apex's
                        // is not shown to be a circle at all: the apex
                        // plane meets the cone only at the apex.
                        Decided {
                            sign: Sign::Zero,
                            margin,
                        } => {
                            return Err(Underived::Refused(
                                OffChart(
                                    "the rim's height is not shown away from the apex's, where \
                                     the apex plane meets the cone only at the apex",
                                )
                                .with_margin(
                                    OffMargin::InBand(margin),
                                    surface,
                                    carrier,
                                ),
                            ));
                        }
                    };
                    let a_dir = a_r * nappe.value();
                    let orient = a_r.cross(radial(form.b)).dot(axis);
                    // β metered at the rim's own radius — the honest
                    // local lever. The spatial traversal rate equals
                    // the chart azimuth rate on either nappe (the +π
                    // offset is constant), so β needs no nappe sign.
                    let rho = a_r.norm();
                    let v0 = rim_slant(h, rho, half_angle, nappe);
                    let beta = winding_of(
                        decide(
                            "pcurve_chart_orientation",
                            Margin::over_lever(orient, rho),
                            band,
                        )
                        .map_err(|cause| {
                            incidence::general_lane(carrier, surface, place, band, Some(cause))
                        })?,
                    );
                    let alpha = place.phase(
                        Along::Azimuth,
                        Azimuth::of(a_dir.dot(cv), a_dir.dot(u_ref)),
                        |m| angle_rate::<T>(beta) * m,
                        band,
                    )?;
                    Ok((
                        Pcurve::Harmonic {
                            u: Channel::Angle {
                                phase: alpha,
                                winding: beta,
                            },
                            v: Channel::constant(v0),
                        },
                        Derivation::ConeRim { nappe, beta },
                    ))
                }
                Curve3::Ellipse {
                    center,
                    axis: e_axis,
                    semi_u,
                    semi_v,
                    u_ref: seam,
                } => {
                    // The tilted-section class: the exact
                    // [`Pcurve::FocalSection`] image (variant docs for
                    // the derivation). The slant is the axial height
                    // over cos α; the azimuth is the true anomaly of
                    // the radial projection `r0 + ra·cos t + rb·sin t`,
                    // whose focus is the axis.
                    //
                    // The incidence test decides first: an ellipse off
                    // this cone is no section of it, and its refusal
                    // names that rather than an image's residual.
                    incidence::incident(carrier, surface, place, band)?;
                    // The image is read about the major vertex, where
                    // the focus is on the `t = t0` side: a carrier seamed
                    // at a minor vertex is read a quarter turn on. The
                    // major direction is the principal one the cone's
                    // axis has a component along (a section's minor axis
                    // is perpendicular to it), which a near-circle's
                    // semi-axes, rounding past each other, cannot say.
                    let Some(frame) = carrier.section_frame(axis) else {
                        unreachable!("an Ellipse carrier has a major frame")
                    };
                    let (e_u, major, minor) = (frame.major_dir, frame.major, frame.minor);
                    let e_v = e_axis.cross(e_u);
                    let radial = |v: Vec3<T>| v - axis * v.dot(axis);
                    let w = center - apex;
                    let h = w.dot(axis);
                    // The incidence test above decided the ellipse's
                    // lowest point past the band off the apex's height
                    // (`z_min = |h| − |(a_z, b_z)|`), so its centre's
                    // height `h` is definite: an ellipse at the apex
                    // height refuses there, as reaching past it.
                    let ns = match decide("pcurve_cone_chart_nappe", Margin::of(h), band) {
                        Ok(Sign::Positive) => T::one(),
                        Ok(Sign::Negative) => T::zero() - T::one(),
                        Ok(Sign::Zero) | Err(_) => unreachable!(
                            "cone_conic_incidence decided the ellipse's lowest point past the \
                             band off the apex's height, so its centre's height is definite"
                        ),
                    };
                    let (r0, ra, rb) = (radial(w), radial(e_u * major), radial(e_v * minor));
                    // Signed eccentricity: positive when the axis (the
                    // focus, at `−r0` from the projection's centre) is
                    // on the `t = 0` side. Total arithmetic: a carrier
                    // that is no section of this cone yields a `β`
                    // the schedule and envelope refuse in metres.
                    let ecc = (T::zero() - r0.dot(ra)) / ra.dot(ra);
                    // On a section `√(1 − ecc²)` is the projection's axis
                    // ratio `|rb|/|ra|` (`ra ⟂ rb`: the major axis lies in
                    // the plane of the cone's axis, `e_v` normal to it),
                    // read so without the root's `1/k` gain on `ecc`'s
                    // rounding near a parabola. Off the cone the two part
                    // at first order (a centre `δ` along `e_u` parts them
                    // by `δ/(A·k)`); either way the envelope measures the
                    // image against the carrier.
                    let beta = ecc / (T::one() + rb.norm() / ra.norm());
                    let orient = ra.cross(rb).dot(axis);
                    let sense = match geom_core::k_stats::decide_reported(
                        "pcurve_chart_orientation",
                        Margin::over_lever(orient, ra.norm()),
                        band,
                    )
                    .map_err(|cause| {
                        incidence::general_lane(carrier, surface, place, band, Some(cause))
                    })? {
                        Decided {
                            sign: Sign::Positive,
                            ..
                        } => T::one(),
                        Decided {
                            sign: Sign::Negative,
                            ..
                        } => T::zero() - T::one(),
                        // The class rate read Zero: not this class, and no
                        // reading of incidence (`not_this_class`).
                        Decided {
                            sign: Sign::Zero, ..
                        } => return Err(not_this_class()),
                    };
                    // The vertex at `t = 0` lies along `ra` from the axis:
                    // `r0 = −e·ra` (the focus), so `r0 + ra = (1 − e)·ra`,
                    // and `1 − e > 0` because this arm takes only a
                    // `Curve3::Ellipse` (`|e| < 1`). Its azimuth is read
                    // off `ra` alone, because the sum cancels to `A·k²/2`
                    // near a parabola and would turn any off-plane
                    // rounding in the centre by `1/k²`.
                    let start = ra * ns;
                    let image = |u0| FocalImage {
                        u0,
                        t0: frame.phase,
                        v0: h / c_ha,
                        va: seam.dot(axis) * semi_u / c_ha,
                        vb: e_axis.cross(seam).dot(axis) * semi_v / c_ha,
                        vl: T::zero(),
                        beta,
                        sense,
                    };
                    // The azimuth moves by the true anomaly from `t0`: the
                    // image read from `u0 = 0` is that motion.
                    let u0 = place.phase(
                        Along::Azimuth,
                        Azimuth::of(start.dot(cv), start.dot(u_ref)),
                        |m| Pcurve::FocalSection(image(T::zero())).eval(m).x,
                        band,
                    )?;
                    Ok((Pcurve::FocalSection(image(u0)), Derivation::FocalSection))
                }
                Curve3::Nurbs(_) | Curve3::Spiric { .. } => unreachable!(
                    "chart_pcurve: spirics route to spiric_chart_pcurve and splines refuse \
                     at carrier_harmonic, both before the chart match"
                ),
            }
        }
        // The sphere chart (M5 S13, certified since M6-3): closed
        // forms for the two azimuth-affine circle classes — POLAR
        // circles (carrier plane ⊥ the polar axis: azimuth `α + β·t`,
        // polar constant) and MERIDIAN-class great circles (carrier
        // plane contains the polar axis: azimuth constant, polar
        // `δ + σ·t`). The GENERAL circle (neither class) is
        // azimuth-non-harmonic: this arm hands it on as such, and its
        // chart image is the projected one (`chart_image`).
        //
        // Since M6-3 this arm is CERTIFIED (run_harmonic_checks admits
        // the sphere chart) and sphere faces mint stored caches. The
        // meridian arm's DERIVED anchor is read in the latitude window
        // (`Azimuth::of(aa, reach)`, `reach` negative on the far half, so
        // the READ ranges over the whole circle; the PLACED anchor lies in
        // [−π/2, π/2]), but the traversed arc
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
            // Structural carrier gate: only circles lie on a sphere — a
            // bounded algebraic surface holds no line, and its plane
            // sections are circles. An `Ellipse` is refused by kind,
            // before any incidence read: its roundness is no reading of
            // whether it lies on the sphere, so no margin is quoted.
            // Reading incidence first, as the cone's ellipse arm does, is
            // E1c's (`work/pcert/plane-and-cylinder-mints-meter-incidence-as-a-residual.md`).
            if !matches!(carrier, Curve3::Circle { .. }) {
                return Err(off_chart(
                    "a sphere holds no line, and its plane sections are circles",
                ));
            }
            let cv = axis.cross(u_ref);
            let w = form.c - center;
            let (aa, ba, wa) = (form.a.dot(axis), form.b.dot(axis), w.dot(axis));
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let (a_r, b_r, w_r) = (radial(form.a), radial(form.b), radial(w));
            // Which class: does the carrier plane contain the polar
            // axis' direction? (Metered in meters at the chart radius.)
            match decide(
                "pcurve_sphere_chart_axial",
                Margin::of(harmonic_sup_scalar(aa, ba)),
                band,
            )
            .map_err(|cause| incidence::general_lane(carrier, surface, place, band, Some(cause)))?
            {
                Sign::Zero => {
                    // POLAR-circle class: a,b ⊥ axis, and the centre on
                    // the axis — checked, not assumed. Failing that, the
                    // circle is a general one, and the incidence test
                    // decides: the axial gate reads the tilt at the
                    // circle's radius `ρ` and this one at its height
                    // `h`, so near a pole an on-sphere circle tilted
                    // inside the first band fails the second.
                    match decide("pcurve_sphere_chart_centered", Margin::norm3(w_r), band).map_err(
                        |cause| incidence::general_lane(carrier, surface, place, band, Some(cause)),
                    )? {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(incidence::general_lane(
                                carrier, surface, place, band, None,
                            ));
                        }
                    }
                    let orient = a_r.cross(b_r).dot(axis);
                    let beta = winding_of(
                        decide(
                            "pcurve_chart_orientation",
                            Margin::over_lever(orient, radius),
                            band,
                        )
                        .map_err(|cause| {
                            incidence::general_lane(carrier, surface, place, band, Some(cause))
                        })?,
                    );
                    let alpha = place.phase(
                        Along::Azimuth,
                        Azimuth::of(a_r.dot(cv), a_r.dot(u_ref)),
                        |m| angle_rate::<T>(beta) * m,
                        band,
                    )?;
                    let polar = (wa / radius).asin();
                    Ok((
                        Pcurve::Harmonic {
                            u: Channel::Angle {
                                phase: alpha,
                                winding: beta,
                            },
                            v: Channel::fixed_angle(polar),
                        },
                        Derivation::SphereParallel { beta },
                    ))
                }
                Sign::Positive | Sign::Negative => {
                    // MERIDIAN class: the carrier plane must contain the
                    // axis (its own axis ⊥ polar) and be centered.
                    // Failing either, the circle is a GENERAL one —
                    // on the sphere or off it, which the incidence
                    // test decides.
                    let coax = Margin::over_lever(form.a.cross(form.b).dot(axis), radius);
                    let meridian =
                        decide("pcurve_sphere_chart_meridian", coax, band).map_err(|cause| {
                            incidence::general_lane(carrier, surface, place, band, Some(cause))
                        })?;
                    let centred = match meridian {
                        Sign::Zero => decide(
                            "pcurve_sphere_chart_centered",
                            Margin::norm3(w),
                            band,
                        )
                        .map_err(|cause| {
                            incidence::general_lane(carrier, surface, place, band, Some(cause))
                        })?,
                        Sign::Positive | Sign::Negative => meridian,
                    };
                    match centred {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return Err(incidence::general_lane(
                                carrier, surface, place, band, None,
                            ));
                        }
                    }
                    // v(t) = σ·t + δ with sin δ = aa/r, cos δ = ‖a_r‖/r
                    // (placed below, once σ is known), and the constant
                    // azimuth direction d̂ read off whichever radial part
                    // is structurally nonzero.
                    let use_a =
                        match decide("pcurve_sphere_chart_pole_frame", Margin::norm3(a_r), band)
                            .map_err(|cause| {
                                incidence::general_lane(carrier, surface, place, band, Some(cause))
                            })? {
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
                    let sigma_sign = match geom_core::k_stats::decide_reported(
                        "pcurve_sphere_chart_polar_rate",
                        Margin::of(sigma_margin),
                        band,
                    )
                    .map_err(|cause| {
                        incidence::general_lane(carrier, surface, place, band, Some(cause))
                    })? {
                        Decided {
                            sign: sign @ (Sign::Positive | Sign::Negative),
                            ..
                        } => sign,
                        // The class rate read Zero: not this class, and no
                        // reading of incidence (`not_this_class`).
                        Decided {
                            sign: Sign::Zero, ..
                        } => return Err(not_this_class()),
                    };
                    let sigma = winding_of(sigma_sign);
                    // The half of the great circle the edge lies on is
                    // the one its span's midpoint is on, never the one its
                    // carrier's `t = 0` point happens to be on (C4: where
                    // the parameter starts moves no stored image). The
                    // half is read against `d̂` turned first to the chart's
                    // own side (toward `u_ref`, or toward `axis × u_ref`
                    // where that is in band; `d̂` is a unit horizontal, so
                    // not both), so every start of the carrier reads the
                    // same number. On the other half the same meridian
                    // reads with `d̂` negated, `σ` negated and `δ` at
                    // `π − δ`. A midpoint on the pole, or within the band
                    // of it, is on neither half decidedly, and either half
                    // names the edge there (the twin joint reads it): it
                    // takes the chart's side, a D9 tie-break.
                    let (s_m, c_m) = place.mid().sin_cos();
                    let at_mid = radial(form.a * c_m + form.b * s_m);
                    let toward = |along: Vec3<T>| match decide(
                        "pcurve_sphere_chart_meridian_frame",
                        Margin::levered(d_hat.dot(along), radius),
                        band,
                    ) {
                        Ok(Sign::Negative) => Some(true),
                        Ok(Sign::Positive) => Some(false),
                        Ok(Sign::Zero) | Err(_) => None,
                    };
                    let turned = toward(u_ref).or_else(|| toward(cv)).unwrap_or(false);
                    let chart_side = if turned {
                        d_hat * (T::zero() - T::one())
                    } else {
                        d_hat
                    };
                    let other = matches!(
                        decide(
                            "pcurve_sphere_chart_meridian_side",
                            Margin::of(at_mid.dot(chart_side)),
                            band,
                        ),
                        Ok(Sign::Negative)
                    );
                    let far = turned != other;
                    let (d_hat, sigma, reach) = if far {
                        let flipped = winding_of(sigma_sign.flip());
                        (
                            d_hat * (T::zero() - T::one()),
                            flipped,
                            T::zero() - a_r.norm(),
                        )
                    } else {
                        (d_hat, sigma, a_r.norm())
                    };
                    let alpha = place.phase(
                        Along::Azimuth,
                        Azimuth::of(d_hat.dot(cv), d_hat.dot(u_ref)),
                        |_| T::zero(),
                        band,
                    )?;
                    // The polar angle moves along the meridian: read and
                    // placed in the latitude window at the span's midpoint.
                    // On the far half the read is `π − δ`, anywhere on
                    // `(π/2, 3π/2)`, and the window's cut, at `1 ± π`, is
                    // generic there; a midpoint on the chart's points is
                    // named as itself ([`Window::Latitude`]).
                    let delta = place.phase(
                        Along::Latitude,
                        Azimuth::of(aa, reach),
                        |m| angle_rate::<T>(sigma) * m,
                        band,
                    )?;
                    Ok((
                        Pcurve::Harmonic {
                            u: Channel::fixed_angle(alpha),
                            v: Channel::Angle {
                                phase: delta,
                                winding: sigma,
                            },
                        },
                        Derivation::SphereMeridian {
                            pole: !use_a,
                            sigma,
                            far,
                        },
                    ))
                }
            }
        }
        // The torus chart (M6-3, walk row 4): closed forms for the
        // three circle families a ring torus holds — PARALLELS (⊥ axis,
        // centred on it: azimuth `α + β·t`, meridional constant),
        // MERIDIANS (plane containing the axis, centre on the spine:
        // azimuth constant, meridional `δ + σ·t`) and VILLARCEAU
        // circles, azimuth-non-harmonic, whose image is the
        // [`Pcurve::FocalSection`] (`torus_oblique_circle`).
        Surface::Torus {
            center: t_center,
            axis,
            major_radius,
            minor_radius,
            u_ref,
        } => {
            let cv = axis.cross(u_ref);
            // A bounded surface holds no line, and a torus's plane
            // sections are bicircular quartics, whose conic components
            // are circles. An `Ellipse` is refused by kind, as on the
            // sphere arm, before any incidence read (E1c's).
            if !matches!(carrier, Curve3::Circle { .. }) {
                return Err(off_chart(
                    "a torus holds no line, and the conics among its plane sections are circles",
                ));
            }
            let w = form.c - t_center;
            let radial = |v: Vec3<T>| v - axis * v.dot(axis);
            let (aa, ba) = (form.a.dot(axis), form.b.dot(axis));
            let (a_r, b_r, w_r) = (radial(form.a), radial(form.b), radial(w));
            // Which family: carrier plane ⊥ the axis? (Metres — the
            // axial parts of a and b are displacements.)
            match decide(
                "pcurve_torus_chart_axial",
                Margin::of(harmonic_sup_scalar(aa, ba)),
                band,
            )
            .map_err(|cause| incidence::general_lane(carrier, surface, place, band, Some(cause)))?
            {
                Sign::Zero => {
                    // PARALLEL: centred on the axis, checked. Failing it,
                    // the circle is not a parallel, but it is not shown
                    // off the torus either: near the tube's top a
                    // centre offset `δ` leaves the torus only by
                    // `δ²/2r`, so a definite offset can sit on the
                    // chart within the band. The incidence test decides.
                    match decide("pcurve_torus_chart_centered", Margin::norm3(w_r), band).map_err(
                        |cause| incidence::general_lane(carrier, surface, place, band, Some(cause)),
                    )? {
                        Sign::Zero => {}
                        Sign::Positive | Sign::Negative => {
                            return torus_oblique_circle(carrier, surface, place, band);
                        }
                    }
                    let rho = a_r.norm();
                    let orient = a_r.cross(b_r).dot(axis);
                    let beta = winding_of(
                        decide(
                            "pcurve_chart_orientation",
                            Margin::over_lever(orient, rho),
                            band,
                        )
                        .map_err(|cause| {
                            incidence::general_lane(carrier, surface, place, band, Some(cause))
                        })?,
                    );
                    let alpha = place.phase(
                        Along::Azimuth,
                        Azimuth::of(a_r.dot(cv), a_r.dot(u_ref)),
                        |m| angle_rate::<T>(beta) * m,
                        band,
                    )?;
                    // v₀ from the height/radius pair: R + r·cos v =
                    // ρ, r·sin v = h — the tube angle of the two
                    // residual-certified coordinates, placed in the
                    // window like the azimuth (a parallel holds it).
                    let v0 = place.phase(
                        Along::Tube,
                        Azimuth::of(w.dot(axis), rho - major_radius),
                        |_| T::zero(),
                        band,
                    )?;
                    Ok((
                        Pcurve::Harmonic {
                            u: Channel::Angle {
                                phase: alpha,
                                winding: beta,
                            },
                            v: Channel::fixed_angle(v0),
                        },
                        Derivation::TorusParallel { beta },
                    ))
                }
                Sign::Positive | Sign::Negative => {
                    // MERIDIAN: the carrier plane must contain the
                    // axis direction and the centre must sit on the
                    // spine (radius R from the axis) — the second is
                    // certified by the residual schedule; the first is
                    // the class gate.
                    let coax = Margin::over_lever(form.a.cross(form.b).dot(axis), minor_radius);
                    match decide("pcurve_torus_chart_meridian", coax, band).map_err(|cause| {
                        incidence::general_lane(carrier, surface, place, band, Some(cause))
                    })? {
                        Sign::Zero => {}
                        // Oblique circles DO lie on a torus (the
                        // Villarceau class), and the incidence test
                        // separates them from circles off it.
                        Sign::Positive | Sign::Negative => {
                            return torus_oblique_circle(carrier, surface, place, band);
                        }
                    }
                    let alpha = place.phase(
                        Along::Azimuth,
                        Azimuth::of(w_r.dot(cv), w_r.dot(u_ref)),
                        |_| T::zero(),
                        band,
                    )?;
                    let (sa, ca) = alpha.sin_cos();
                    let rad = u_ref * ca + cv * sa;
                    // δ from the t = 0 point's meridional components;
                    // σ from the t-derivative's (b's) — the relations
                    // a·axis = r·sin δ·?… spelled from
                    // e(t) = spine + a·cos t + b·sin t:
                    //   r·cos v = (e − spine)·rad, r·sin v = (e − spine)·axis
                    // with v = δ + σ·t ⇒ δ = atan2(a·axis, a·rad) and
                    // σ·r = b·axis·cos δ − b·rad·sin δ. With `ℓ` the
                    // length of `(a·axis, a·rad)`, `ℓ·cos δ = a·rad` and
                    // `ℓ·sin δ = a·axis`, so σ's margin is read off that
                    // one direction, `(b·axis·a·rad − b·rad·a·axis)/ℓ`,
                    // and `δ` is computed once, placed, below.
                    let a_rad = a_r.dot(rad);
                    let tube = Azimuth::of(aa, a_rad);
                    let sigma = match geom_core::k_stats::decide_reported(
                        "pcurve_torus_chart_meridional_rate",
                        Margin::over_lever(ba * a_rad - b_r.dot(rad) * aa, tube.length()),
                        band,
                    )
                    .map_err(|cause| {
                        incidence::general_lane(carrier, surface, place, band, Some(cause))
                    })? {
                        Decided {
                            sign: sign @ (Sign::Positive | Sign::Negative),
                            ..
                        } => winding_of(sign),
                        // The class rate read Zero: not this class, and no
                        // reading of incidence (`not_this_class`).
                        Decided {
                            sign: Sign::Zero, ..
                        } => return Err(not_this_class()),
                    };
                    // The tube angle placed in the window at the span's
                    // midpoint; σ, read off the direction above, is the
                    // same on either copy.
                    let delta =
                        place.phase(Along::Tube, tube, |m| angle_rate::<T>(sigma) * m, band)?;
                    Ok((
                        Pcurve::Harmonic {
                            u: Channel::fixed_angle(alpha),
                            v: Channel::Angle {
                                phase: delta,
                                winding: sigma,
                            },
                        },
                        Derivation::TorusMeridian { sigma },
                    ))
                }
            }
        }
        Surface::Nurbs(_) => Err(Underived::Refused(PcurveCertifyError::UnsupportedChart {
            chart: geom::SurfaceKind::Nurbs,
        })),
        // The closed-form pcurve mint is the analytic charts'. An
        // approximating surface's chart is a spline's, so it has no
        // harmonic image to mint — the fitted lane owns it.
        Surface::Approx(_) => Err(Underived::Refused(PcurveCertifyError::UnsupportedChart {
            chart: geom::SurfaceKind::Approx,
        })),
    }
}

/// Why [`derive_class`] built no closed-form image: a refusal, or a
/// carrier the chart's incidence test does not show off the chart whose
/// class reading shows none of the chart's own classes, for D3's lane
/// choice to send to the general lane ([`chart_image`]).
enum Underived {
    Refused(PcurveCertifyError),
    /// No closed form images the carrier, and the chart's incidence test
    /// does not show it off the chart (`shown`, the test's witness): a
    /// class reading answered in band (`undecided`, its cause) or
    /// definitely not one of the chart's own classes (`None`). D3's lane
    /// choice takes the general lane for both where one is written, and
    /// refuses "not yet" where none is ([`chart_image`]).
    General {
        shown: incidence::Incident,
        undecided: Option<Indeterminate>,
    },
}

impl From<PcurveCertifyError> for Underived {
    fn from(e: PcurveCertifyError) -> Self {
        Self::Refused(e)
    }
}

/// **A moving carrier's axial `cos t` and `sin t` terms on a cylinder's
/// chart**, `(a, b)`: zero for a rim, its plane ⊥ the axis, where both
/// are enclosures holding zero and their sup is decided `Zero` at
/// `pcurve_cylinder_chart_axial` — the cone arm's
/// `pcurve_cone_chart_axial` read on the cylinder. Otherwise as given.
///
/// An interval scalar encloses a rim's terms, dot products of its frame
/// with the axis, as intervals about zero that an exact test does not
/// read as zero. At `f64` a value holds zero only when it is zero, so
/// nothing there is decided or moved. The dropped drift is within the
/// band, and the certificate's envelope bounds what it moves.
fn cylinder_rim_axial<T: Decide>(a: T, b: T, band: Band) -> (T, T) {
    let holds_zero = |x: T| {
        x.structure_bracket()
            .is_some_and(|(lo, hi)| lo <= 0.0 && 0.0 <= hi)
    };
    if (geom_core::is_exact_zero(a) && geom_core::is_exact_zero(b))
        || !(holds_zero(a) && holds_zero(b))
    {
        return (a, b);
    }
    match decide(
        "pcurve_cylinder_chart_axial",
        Margin::of(harmonic_sup_scalar(a, b)),
        band,
    ) {
        Ok(Sign::Zero) => (T::zero(), T::zero()),
        _ => (a, b),
    }
}

/// [`derive_class`] for a caller that wants a closed-form image or a
/// refusal: the sphere's general circle has none.
fn derive_harmonic<T: Decide>(
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    place: Placement<T>,
    band: Band,
) -> Result<(Pcurve<T>, Derivation), PcurveCertifyError> {
    derive_class(carrier, surface, place, band).map_err(|u| match u {
        Underived::Refused(e) => e,
        // The class reading is in band: not shown either way, so a
        // closed form re-read here is undecided, never a contradiction.
        Underived::General {
            shown,
            undecided: Some(cause),
        } => incidence::class_escalated(shown, cause),
        Underived::General {
            undecided: None, ..
        } => PcurveCertifyError::ImageMismatch {
            image: PcurveKind::Harmonic,
            why: "the carrier is none of the chart's own classes, so it has no closed-form \
                  image; its image is the general one",
        },
    })
}

/// The margin a [`PcurveCertifyError::CarrierOffChart`] read at a
/// degenerate pose (the cone's apex plane, which meets the cone only at
/// the apex), and which side of the band it fell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OffMargin {
    /// Decided past the pose: the carrier is off the chart.
    Past(geom_core::MarginDiag),
    /// A band-decided Zero: the carrier is not shown away from the pose,
    /// so at this tolerance it is read as there, and no carrier of its
    /// kind lies on the chart there. It is not shown off the chart either.
    InBand(geom_core::MarginDiag),
}

impl OffMargin {
    /// The margin the decision read, on either side of the band.
    #[must_use]
    pub fn diag(self) -> geom_core::MarginDiag {
        match self {
            Self::Past(m) | Self::InBand(m) => m,
        }
    }
}

/// **How a carrier's incidence on its chart was read**: the one
/// classifier every door's incidence ending matches on
/// ([`IncidenceRead::of`], [`IncidenceRead::ending`]).
#[derive(Clone, Copy, Debug)]
pub enum IncidenceRead<'a> {
    /// Decided off the chart: past the band, or with no margin to quote.
    Off(Option<geom_core::MarginDiag>),
    /// A band-decided Zero at a degenerate pose ([`OffMargin::InBand`]):
    /// at this tolerance the carrier sits where the chart holds no curve
    /// of its kind; it is not shown off the chart.
    AtPose(geom_core::MarginDiag),
    /// The incidence test landed in the band
    /// ([`PcurveCheck::ChartIncidence`] escalated).
    Undecided(&'a Indeterminate),
}

impl<'a> IncidenceRead<'a> {
    /// **The one incidence classifier**: the read a refusal carries, or
    /// `None` for a refusal of anything else. Every reader of an incidence
    /// refusal reads it here (the edge certificate's through
    /// [`crate::CertifyError::incidence_read`], which agrees arm for arm),
    /// and decides definite or in band by [`Self::is_definite`].
    #[must_use]
    pub fn of(error: &'a PcurveCertifyError) -> Option<Self> {
        match error {
            PcurveCertifyError::CarrierOffChart { margin, .. } => Some(Self::of_margin(*margin)),
            PcurveCertifyError::Escalated {
                check: PcurveCheck::ChartIncidence,
                cause,
                ..
            } => Some(Self::Undecided(cause)),
            // The test's residual decided past the band: a definite miss
            // with no margin carried.
            PcurveCertifyError::ResidualExceeded {
                check: PcurveCheck::ChartIncidence,
                ..
            } => Some(Self::Off(None)),
            _ => None,
        }
    }

    /// The read a decided incidence refusal's margin names.
    #[must_use]
    pub fn of_margin(margin: Option<OffMargin>) -> Self {
        match margin {
            Some(OffMargin::InBand(m)) => Self::AtPose(m),
            None => Self::Off(None),
            Some(OffMargin::Past(m)) => Self::Off(Some(m)),
        }
    }

    /// The refused arm the read carries.
    #[must_use]
    pub fn arm(self) -> RefusedArm<'a> {
        match self {
            Self::Off(margin) => RefusedArm::SignCertain(margin),
            Self::AtPose(m) => RefusedArm::SignCertain(Some(m)),
            Self::Undecided(cause) => RefusedArm::Undecided(cause),
        }
    }

    /// **An incidence decision's ending read off its refused arm alone**,
    /// for a refusal that does not carry its read: a sign-certain arm is a
    /// definite miss, a band-decided Zero a pose, an undecided arm
    /// undecided, each through [`Self::ending`]; a straddle, two readable
    /// bounds neither showing the carrier off nor on, the undecided read's
    /// lever. A refusal that carries its read ([`Self::of`]) ends by it
    /// instead: off and at a pose both refuse sign-certain, so only the
    /// read tells them apart.
    #[must_use]
    pub fn arm_ending(arm: RefusedArm<'_>, at: impl Into<ReadAt>) -> String {
        match arm {
            RefusedArm::SignCertain(margin) => IncidenceRead::Off(margin).ending(at),
            RefusedArm::Zero(crate::recourse::Classified { margin, .. }) => {
                IncidenceRead::AtPose(margin).ending(at)
            }
            RefusedArm::Undecided(cause) => IncidenceRead::Undecided(cause).ending(at),
            RefusedArm::Straddle => crate::recourse::INCIDENCE_UNSHOWN.recourse(arm, at),
        }
    }

    /// **The one ending of an incidence refusal**, read at `at` (D3, D4
    /// ¶1 (iv)): a definite miss is a contradiction between what the edge
    /// states and its geometry, ended by the door that reads it (the
    /// kernel's defect at a build, the stored culprit at rest, the
    /// damaged file at the import door, with no ε_in offer: the margin it
    /// may carry is a pose's height, not the miss); a read in band, at a
    /// pose or undecided, is a reading of the geometry, ending in its
    /// lever at every door, never a defect, unless its margin could not
    /// be read: that reads nothing about the geometry and ends in the
    /// defect ending. No ending reads who placed the carrier.
    #[must_use]
    pub fn ending(self, at: impl Into<ReadAt>) -> String {
        let arm = self.arm();
        match self {
            Self::AtPose(_) if !self.is_definite() => {
                crate::recourse::INCIDENCE_AT_POSE.recourse(arm, at)
            }
            Self::Undecided(_) if !self.is_definite() => {
                crate::recourse::INCIDENCE_UNSHOWN.recourse(arm, at)
            }
            // A definite miss, or a margin that could not be read.
            Self::Off(_) | Self::AtPose(_) | Self::Undecided(_) => {
                crate::recourse::Unsized::Defect.recourse(arm, at)
            }
        }
    }

    /// **Whether this read is definite**: a miss decided off the chart,
    /// or a read whose margin could not be read, which says nothing of
    /// the geometry's pose and which no lever makes readable (as every
    /// unsized decision's unreadable arm, [`crate::recourse::Unsized::recourse`]).
    /// A definite read ends in the door's defect ending
    /// ([`Self::ending`]) and is a torn row to a reader that walks one
    /// (topo's boolean loop walk); a read in band is neither. The one
    /// policy every reader of an incidence refusal uses.
    #[must_use]
    pub fn is_definite(self) -> bool {
        match self {
            Self::Off(_) => true,
            Self::AtPose(m) => m.is_invalid(),
            Self::Undecided(cause) => cause.margin.is_invalid(),
        }
    }
}

/// A closed-form arm's conclusion that the carrier cannot lie on the
/// chart, with why ([`PcurveCertifyError::CarrierOffChart`]).
#[derive(Clone, Copy, Debug)]
struct OffChart(&'static str);

impl OffChart {
    fn refusal<T: Real>(self, surface: &Surface<T>, carrier: &Curve3<T>) -> PcurveCertifyError {
        PcurveCertifyError::CarrierOffChart {
            chart: surface.kind(),
            carrier: carrier.kind(),
            why: self.0,
            margin: None,
        }
    }

    /// The refusal quoting the margin the decision read (D4 ¶1 (i)): a
    /// reading not shown away from a degenerate pose
    /// ([`OffMargin::InBand`]), or one decided past it
    /// ([`OffMargin::Past`]).
    fn with_margin<T: Real>(
        self,
        margin: OffMargin,
        surface: &Surface<T>,
        carrier: &Curve3<T>,
    ) -> PcurveCertifyError {
        PcurveCertifyError::CarrierOffChart {
            chart: surface.kind(),
            carrier: carrier.kind(),
            why: self.0,
            margin: Some(margin),
        }
    }
}

/// What a chart's incidence test decided about a carrier
/// ([`chart_incidence`]): not shown off the chart, or definitely off it.
#[derive(Clone, Copy, Debug)]
enum Incidence {
    On,
    Off,
}

/// The escalation an identity check on a stored image's copied data
/// raises at check 4 ([`PcurveCheck::ImageIdentity`]).
fn identity_escalated(cause: Indeterminate) -> PcurveCertifyError {
    PcurveCertifyError::Escalated {
        check: PcurveCheck::ImageIdentity,
        sample: 0,
        cause,
    }
}

/// A circle on a torus chart that is neither a parallel nor a meridian:
/// a Villarceau circle, imaged as a [`Pcurve::FocalSection`], or no
/// circle of the torus.
///
/// **Incidence** ([`chart_incidence`], `pcurve_torus_chart_incident`):
/// a sampled point definitely off the torus refuses the circle as off
/// it; otherwise it is not shown off, and the gates decide its class.
///
/// **The Villarceau gates**, each a banded metre margin required Zero:
/// the centre `r` from the torus centre (`_offset`) in the equatorial
/// plane (`_equator`), the carrier plane through the torus centre
/// (`_plane`) and tilted `asin(r/R)` off the equator (`_tilt`, levered
/// at `R`), and the radius `R` (`_radius`). Together they pin the
/// Villarceau circle: the centre lies on the line where the plane meets
/// the equator. Not shown off the torus but past any gate is a circle no
/// closed form images — a circle ⊥ the axis centred `δ` off it leaves the
/// tube's crest by only `δ²/2r` — so D3's lane choice sends it to the
/// general lane, which is not written for a circle on a torus: it refuses
/// lever-only "not yet" (unit E4). How far off the torus a gate's failure
/// puts a circle depends on the gate and the torus (the gate site's
/// comment): about the gate's own margin for each gate on a ring torus,
/// so the incidence test reads such a circle first, but much less for the
/// plane gate near a horn torus, where a circle past that gate reaches
/// the gates. The gates are what tell the class from a circle that only
/// meets the torus within the band.
///
/// **The image** (variant docs): the vertex is the point
/// `centre + R·d̂` farthest from the axis (`d̂` the unit radial of the
/// centre), on the outer equator, so `t0` is its carrier parameter,
/// `u0` its chart azimuth and `v0 = −vl·t0` puts the tube angle `0`
/// there. `vl` and `sense` are the signs of the carrier's tangent there,
/// `R·(n × d̂)`, along the axis (`pcurve_torus_villarceau_tube_rate`,
/// `±r`) and along the azimuth (`pcurve_chart_orientation`,
/// `±√(R² − r²)`), each levered at `R`; each magnitude is first decided
/// positive, which fails only where the tube `r`, or `√(R² − r²)`, is at
/// the band's scale (the comment at `unit_sign`).
///
/// # Errors
///
/// [`PcurveCertifyError::CarrierOffChart`] off the torus;
/// [`Underived::General`] not shown off it and no Villarceau circle
/// shown (in band: with the ladder's cause), through the incidence door
/// ([`incidence::general_lane`]); [`PcurveCertifyError::Escalated`] when
/// the incidence test lands in the sliver band
/// ([`PcurveCheck::ChartIncidence`]), or a tangent's sign is not decided:
/// [`PcurveCheck::ChartClass`], the class not shown (the comment at
/// `unit_sign`).
fn torus_oblique_circle<T: Decide>(
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    place: Placement<T>,
    band: Band,
) -> Result<(Pcurve<T>, Derivation), Underived> {
    let (
        &Curve3::Circle {
            center: c,
            axis: n,
            radius,
            ..
        },
        &Surface::Torus {
            center,
            axis,
            major_radius: big,
            minor_radius: r,
            u_ref,
        },
    ) = (carrier, surface)
    else {
        unreachable!("torus_oblique_circle: the torus arm reads a circle carrier")
    };
    let shown = incidence::incident(carrier, surface, place, band)?;
    let w = c - center;
    let w_r = w - axis * w.dot(axis);
    let gates = [
        ("pcurve_torus_villarceau_offset", Margin::of(w_r.norm() - r)),
        ("pcurve_torus_villarceau_equator", Margin::of(w.dot(axis))),
        ("pcurve_torus_villarceau_plane", Margin::of(n.dot(w))),
        (
            "pcurve_torus_villarceau_tilt",
            Margin::levered(n.cross(axis).norm() - r / big, big),
        ),
        ("pcurve_torus_villarceau_radius", Margin::of(radius - big)),
    ];
    // Read as an identity ladder (`crate::ladder`), the spiric chart's
    // rule: a gate decided apart answers past an earlier in-band one.
    // What reaches these gates through `chart_pcurve`, as measured:
    // - On `R = 2`, `r = 0.7`, a circle's first-order departure from the
    //   torus at the 64 incidence samples has a one-dimensional null space
    //   (a turn about the axis, which moves no gate), and the least
    //   departure that puts any one gate at `m` is `0.995·m` to `1.04·m`
    //   at its farthest sample: a gate in band or past it is read about as
    //   far off the torus by the incidence test, which decides first.
    // - Near a horn torus the plane gate is cheaper. On `R = 2`,
    //   `R − r = 1e-6`, band `(1e-9, 1e-8)`, the Villarceau circle moved
    //   along its own normal by `3e-9` to `1e-8` escalates here on
    //   `_plane` (`pcurve_villarceau::a_plane_gate_in_band_near_the_horn_…`),
    //   and by `3e-8` is not shown off the torus, no Villarceau circle,
    //   and refused "not yet" (`NoGeneralLane`).
    // - The masking order needs a gate read before `_plane` (`_offset`,
    //   `_equator`) in band with `_plane` past it. On that near-horn pose,
    //   with `_plane` at `3e-8` or `5e-8`, `2e-9` or `5e-9` of offset or
    //   of equator departs about its own margin (`2.6e-9` for `2e-9` of
    //   offset, `2.03e-9` for `2e-9` of equator),
    //   and the incidence test escalates on it before the gates read
    //   anything.
    // `ladder`'s rows hold the rule itself.
    match read_identity(gates, |name, margin| {
        geom_core::k_stats::decide_reported(name, margin, band)
    }) {
        IdentityLadder::Zero(_) => {}
        // Not a Villarceau circle, and not shown off the torus (the
        // incidence test above): no closed form images it, so D3's lane
        // choice sends it to the general lane, which a circle on a torus
        // does not have yet ("not yet", unit E4).
        IdentityLadder::Apart { .. } => {
            return Err(incidence::general_lane(carrier, surface, place, band, None));
        }
        IdentityLadder::Undecided(kept) => {
            return Err(incidence::general_lane(
                carrier,
                surface,
                place,
                band,
                Some(kept.refusing_cause()),
            ));
        }
    }
    let d = w_r / w_r.norm();
    let tangent = n.cross(d);
    // The tangent's two parts, levered at `R`, are `R·sin θ` (`r` on a
    // Villarceau circle) and `R·cos θ` (`√(R² − r²)`), `θ` the plane's
    // tilt. The router reaches here by two routes:
    // - Both class gates apart. `pcurve_torus_chart_axial` read the
    //   tube-rate part's own number, `R·sin θ`, apart.
    //   `pcurve_torus_chart_meridian` read `(a × b)·e / r = (R/r)·R·cos θ`
    //   apart. `R·cos θ` at the band's scale forces `R − r ≤ (K·ε)²/2R`,
    //   where `R/r` is one to below rounding, so the readings agree.
    // - The parallel arm: `_axial` Zero (`R·sin θ ≤ ε`) and
    //   `pcurve_torus_chart_centered` apart (`‖w_r‖ ≥ K·ε`). Passing the
    //   gates above then needs `‖w_r‖ ≤ r + ε` (`_offset`) and
    //   `r ≤ R·sin θ + ε` (`_tilt`), so `‖w_r‖ ≤ 3·ε`: this route reaches
    //   here only at `K ≤ 3`, up to rounding, and there the tube-rate part
    //   decides Zero (measured at `R = 2`, `r = 1.8e-9`, band
    //   `(1e-9, 2e-9)`).
    // Neither part is an incidence reading: the test above did not show
    // the circle off the torus. Each route, read by its own gates:
    // - the parallel route (`_axial` Zero, `_centered` apart) reaches here
    //   only at `K ≤ 3`, and there the tube-rate part is a class reading of
    //   a circle at the band's scale: a Zero or in-band reading is the
    //   class not shown, a lane choice, "not yet" (`ChartClass`, Ev on
    //   PR 5113; `pcurve_villarceau::a_tube_rate_gates_rejected_zero_is_a_class_reading`);
    // - the both-apart route reads two numbers its gates already decided
    //   apart, so its parts decide too, up to rounding: nothing in band
    //   reaches the refusal from it, and a rounding-level reading would
    //   be the same class not shown.
    // So both end as the class reading; no check of its own is owed
    // (check 4's re-read of a class in band ends `ChartClass` too,
    // `check_4_class_rows`).
    let reread = |cause| incidence::class_escalated(shown, cause);
    let unit_sign = |name, along: Vec3<T>| -> Result<T, PcurveCertifyError> {
        let part = tangent.dot(along);
        geom_core::k_stats::decide_positive(name, Margin::levered(part.abs(), big), band)
            .map_err(reread)?;
        match decide(name, Margin::levered(part, big), band).map_err(reread)? {
            Sign::Positive => Ok(T::one()),
            Sign::Negative => Ok(T::zero() - T::one()),
            Sign::Zero => unreachable!("a part whose magnitude decided positive decides a sign"),
        }
    };
    let vl = unit_sign("pcurve_torus_villarceau_tube_rate", axis)?;
    let sense = unit_sign("pcurve_chart_orientation", axis.cross(d))?;
    let Some(form) = carrier_harmonic(carrier) else {
        unreachable!("carrier_harmonic: a circle has its harmonic form")
    };
    // Read through `Azimuth::principal`, so a direction of no length (an
    // undefined `t0`) reads `0` here as everywhere.
    let t0 = Azimuth::of(d.dot(form.b), d.dot(form.a)).principal(band);
    let cv = axis.cross(u_ref);
    // The tube angle, `vl·(t − t0)` from the carrier's own parameter, is
    // an angle of the chart like the azimuth: placed in the window at the
    // span's midpoint, levered at the tube's radius.
    let v_read = T::zero() - vl * t0;
    let v0 = match place.turns(Along::Tube, v_read + vl * place.mid(), band)? {
        0 => v_read,
        k => v_read - Period::new(T::tau()).times(k),
    };
    let image = |u0| FocalImage {
        u0,
        t0,
        v0,
        va: T::zero(),
        vb: T::zero(),
        vl,
        beta: T::zero() - r / (big + (big.powi(2) - r.powi(2)).sqrt()),
        sense,
    };
    // The azimuth placed in the window at the span's midpoint, levered at
    // the major radius: `d` is a unit direction, and the circle's points
    // sit about the core circle `R` from the axis.
    let u0 = place.phase(
        Along::Azimuth,
        Azimuth::of(d.dot(cv), d.dot(u_ref)),
        |m| Pcurve::FocalSection(image(T::zero())).eval(m).x,
        band,
    )?;
    Ok((Pcurve::FocalSection(image(u0)), Derivation::FocalSection))
}

/// Why a circle off a torus chart is off it: one sample of it is past the
/// band from the torus.
const TORUS_OFF: &str = "the circle's points are not on the torus (its distance from the torus \
                         at a sample is past the band)";

/// The carrier parameters [`chart_incidence`] samples over one turn.
const INCIDENCE_SAMPLES: u32 = 64;

/// The exact distance of `p` from an analytic curved chart: the sphere's
/// `|‖p − c‖ − R|`, the double cone's `|elevation|`
/// ([`crate::implicit::cone_elevation`], exact on the double cone), the
/// torus's `|√((ρ − R)² + h²) − r|`.
fn chart_distance<T: Real>(surface: &Surface<T>, p: Point3<T>) -> T {
    match *surface {
        Surface::Sphere { center, radius, .. } => (p.distance(center) - radius).abs(),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => crate::implicit::cone_elevation(apex, axis, half_angle, None, p).abs(),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (h, w) = crate::implicit::axial_radial(p, center, axis);
            ((w.norm() - major_radius).powi(2) + h.powi(2)).sqrt() - minor_radius
        }
        .abs(),
        Surface::Plane { origin, normal, .. } => ((p - origin).dot(normal) / normal.norm()).abs(),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (_, w) = crate::implicit::axial_radial(p, origin, axis);
            (w.norm() - radius).abs()
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            unreachable!("chart_distance: the incidence test reads an analytic chart")
        }
    }
}

/// **The incidence test of a closed conic on a curved chart**: the
/// sampled test ([`incidence_over_span`]) over one turn `[0, τ]`, whose
/// end repeats its start.
///
/// # Errors
///
/// As [`incidence_over_span`].
fn chart_incidence<T: Decide>(
    name: &'static str,
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<Incidence, PcurveCertifyError> {
    incidence_over_span(
        name,
        surface,
        (T::zero(), T::tau()),
        |t| carrier.eval(t),
        band,
    )
}

/// **A carrier's sampled incidence test**, decided one-sidedly: `Off`
/// only when the carrier is definitely off the chart somewhere. The
/// largest exact distance from the chart of its points at the
/// [`farthest_over_span`] samples of `span` bounds its distance from
/// below, so a `Positive` verdict is certified; `Zero` says only that the
/// carrier is not shown off, and is `On` here — the image the caller
/// derives is then measured by its certificate, and a carrier that is no
/// class of the chart takes D3's lane choice ([`incidence::general_lane`]).
/// The one decider of a closed conic's, a line's and a spline's tests.
///
/// # Errors
///
/// [`PcurveCertifyError::Escalated`] ([`PcurveCheck::ChartIncidence`])
/// when the sampled distance lands in the sliver band, or is poisoned.
fn incidence_over_span<T: Decide>(
    name: &'static str,
    surface: &Surface<T>,
    span: (T, T),
    at: impl Fn(T) -> Point3<T>,
    band: Band,
) -> Result<Incidence, PcurveCertifyError> {
    let farthest = farthest_over_span(surface, span, at);
    match decide(name, Margin::of(farthest), band).map_err(incidence_escalated)? {
        Sign::Positive => Ok(Incidence::Off),
        Sign::Zero | Sign::Negative => Ok(Incidence::On),
    }
}

/// **The one door to the general lane** (D3): a lane choice picks between
/// images of a carrier ON the chart, so before a class reading that did
/// not show one of the chart's own classes may take the general lane, or
/// refuse "not yet" ([`PcurveCheck::ChartClass`]), the chart's incidence
/// test must not have shown the carrier off the chart. The witness of
/// that ([`Incident`](incidence::Incident)) is built here and nowhere
/// else, so an arm that skips the test cannot build
/// [`Underived::General`] or a class escalation.
mod incidence {
    use super::{
        Curve3, Decide, Incidence, Indeterminate, OffChart, PcurveCertifyError, PcurveCheck,
        Placement, Surface, Underived, chart_incidence, cone_conic_incidence,
    };
    use geom_core::Band;

    /// The carrier's incidence test on its chart did not show it off the
    /// chart. Built only by [`incident`].
    #[derive(Clone, Copy, Debug)]
    pub(super) struct Incident {
        _tested: (),
    }

    /// **The chart's incidence test of `carrier`**: a carrier shown off
    /// the chart refuses `CarrierOffChart`, one whose test lands in band
    /// escalates [`PcurveCheck::ChartIncidence`], and only one not shown
    /// off it is [`Incident`]. A closed conic is sampled over a turn
    /// ([`chart_incidence`]); on a cone it first decides its lowest point
    /// off the apex's height ([`cone_conic_incidence`]); a line is
    /// sampled over the placement's span, the only part of it the image
    /// states.
    ///
    /// # Errors
    ///
    /// As above.
    pub(super) fn incident<T: Decide>(
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        place: Placement<T>,
        band: Band,
    ) -> Result<Incident, PcurveCertifyError> {
        let tested = Incident { _tested: () };
        #[cfg(test)]
        if super::mutants::skip_incidence() {
            return Ok(tested);
        }
        let (name, off_why) = match (carrier, surface) {
            (Curve3::Circle { .. } | Curve3::Ellipse { .. }, Surface::Cone { .. }) => {
                return cone_conic_incidence(carrier, surface, band).map(|()| tested);
            }
            (Curve3::Line { .. }, _) => {
                return line_incidence(carrier, surface, place.span, band).map(|()| tested);
            }
            (_, Surface::Sphere { .. }) => (
                "pcurve_sphere_chart_incident",
                "the circle is not at the sphere's radius from its centre",
            ),
            (_, Surface::Torus { .. }) => ("pcurve_torus_chart_incident", super::TORUS_OFF),
            (_, Surface::Cylinder { .. }) => (
                "pcurve_cylinder_chart_incident",
                "the carrier's points are not at the cylinder's radius from its axis",
            ),
            _ => unreachable!(
                "incident: only the sphere, cone, torus and cylinder mints read incidence (a \
                 plane's mint reads none until E1c; a spline chart has no mint), and a spline or \
                 spiric carrier takes its own test"
            ),
        };
        match chart_incidence(name, carrier, surface, band)? {
            Incidence::On => Ok(tested),
            Incidence::Off => Err(OffChart(off_why).refusal(surface, carrier)),
        }
    }

    /// A line's incidence test: the sampled test
    /// ([`super::incidence_over_span`]) over the placement's `span`.
    fn line_incidence<T: Decide>(
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        span: (T, T),
        band: Band,
    ) -> Result<(), PcurveCertifyError> {
        match super::incidence_over_span(
            "pcurve_line_chart_incident",
            surface,
            span,
            |t| carrier.eval(t),
            band,
        )? {
            Incidence::Off => Err(OffChart(
                "the line's points are not on the chart (its distance from the chart at a \
                 sample of its span is past the band)",
            )
            .refusal(surface, carrier)),
            Incidence::On => Ok(()),
        }
    }

    /// **The general lane, through the incidence test** (D3's lane
    /// choice): [`Underived::General`] for a carrier the test did not show
    /// off the chart, carrying the class reading's cause where it was in
    /// band; the test's own refusal otherwise.
    pub(super) fn general_lane<T: Decide>(
        carrier: &Curve3<T>,
        surface: &Surface<T>,
        place: Placement<T>,
        band: Band,
        undecided: Option<Indeterminate>,
    ) -> Underived {
        match incident(carrier, surface, place, band) {
            Ok(shown) => Underived::General { shown, undecided },
            Err(refused) => Underived::Refused(refused),
        }
    }

    /// The escalation of a class reading that did not show one of the
    /// chart's own classes, on a carrier [`incident`] did not show off
    /// the chart, where the pair has no general lane written
    /// ([`PcurveCheck::ChartClass`]): lever-only "not yet".
    pub(super) fn class_escalated(_shown: Incident, cause: Indeterminate) -> PcurveCertifyError {
        PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartClass,
            sample: 0,
            cause,
        }
    }
}

/// The escalation of a genuine incidence reading landing in the sliver
/// band: the sampled test (`incidence_over_span`) and the spiric wall's
/// centre read. No class gate raises it (a class reading in band is
/// [`PcurveCheck::ChartClass`]'s, through the incidence door).
fn incidence_escalated(cause: Indeterminate) -> PcurveCertifyError {
    PcurveCertifyError::Escalated {
        check: PcurveCheck::ChartIncidence,
        sample: 0,
        cause,
    }
}

/// The exact chart image of a [`geom::Curve3::Spiric`] — on its cutting
/// plane and on the two tori that hold it, and a typed refusal for every
/// other chart.
///
/// **Plane cap.** The chart is affine, so the carrier's own
/// combination `C(t) = (c + n·d) + m·f(t) + (a·r)·sin t` maps through
/// coefficient by coefficient: the constant point through the chart's
/// point map, `m` and `a·r` through its linear part
/// ([`SpiricImage::Cap`]'s docs).
///
/// **Torus wall.** Two decisions ([`SpiricImage::Wall`]'s derivation).
/// `sense = σ`, the cosine of the chart axis against the carrier's
/// levered at the chart's major radius (`pcurve_spiric_chart_axis`):
/// `+1` when they agree, `−1` when they oppose, and Zero refuses — a
/// torus whose axis is perpendicular to the carrier's holds no oval of
/// it ([`spiric_off_own_chart`]). `torus`, which of the two tori
/// holding the oval the chart is: [`WallTorus::Own`] when its centre is
/// the carrier's (`pcurve_spiric_mint_center`), else
/// [`WallTorus::Mirror`] — check 1 then gates the centre against
/// [`WallTorus::center`] of the tag, so a chart that is neither torus
/// refuses there. `u₀` is one `atan2` of `μ·n` in the chart frame;
/// `v₀` is zero at the mint and gains `k·τ` only from the loop walk's
/// branch shift.
///
/// Nothing here is a class decision beyond the sign and the tag: the image is
/// built, and [`PcurveCache::certify`] then measures it against the
/// chart and the carrier. A spiric handed a torus that is neither of
/// its two mints an image whose check 1 refuses it as off the chart,
/// which is the module's standing posture.
///
/// # Errors
///
/// [`spiric_off_own_chart`]'s refusal for a cylinder, cone or sphere
/// chart and for a torus whose axis is perpendicular to the carrier's;
/// [`PcurveCertifyError::UnsupportedChart`] for a spline or
/// approximating chart, whose images are description-driven.
fn spiric_chart_pcurve<T: Decide>(
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    place: Placement<T>,
    band: Band,
) -> Result<Pcurve<T>, PcurveCertifyError> {
    let Curve3::Spiric {
        center,
        axis,
        u_ref: n,
        major_radius,
        minor_radius,
        offset,
    } = *carrier
    else {
        unreachable!("spiric_chart_pcurve: the one caller, `chart_pcurve`, routes spirics only")
    };
    let spiric = |image| Pcurve::Spiric {
        major: major_radius,
        minor: minor_radius,
        offset,
        image,
    };
    match *surface {
        Surface::Plane {
            origin,
            normal,
            u_ref,
        } => {
            let v_ref = normal.cross(u_ref);
            let chart = |v: Vec3<T>| Vec2::new(v.dot(u_ref), v.dot(v_ref));
            let w = (center + n * offset) - origin;
            let m = axis.cross(n);
            Ok(spiric(SpiricImage::Cap {
                p0: Point2::new(w.dot(u_ref), w.dot(v_ref)),
                pm: chart(m),
                pa: chart(axis * minor_radius),
            }))
        }
        Surface::Torus {
            center: chart_center,
            axis: chart_axis,
            major_radius: chart_major,
            u_ref: chart_u,
            ..
        } => {
            let cv = chart_axis.cross(chart_u);
            // Both readings place the oval on one of the tori that hold
            // it, so their far or Zero arm is the chart's incidence, never
            // a lane choice. The axis reading's sign is all that can
            // place it: a reading Zero or in the sliver puts the axes
            // within `K·ε/R` of perpendicular, far from the parallel ones
            // every torus holding the oval has, so it is definitely off
            // (no margin to quote). A poisoned read says only that. The centre's reading in the sliver is
            // the chart's distance from the own torus, an incidence
            // reading ([`PcurveCheck::ChartIncidence`]).
            let perpendicular = || {
                spiric_off_own_chart(
                    "the chart torus's axis is perpendicular to the carrier's, and a spiric \
                     oval lies only on tori parallel to its own",
                )
                .refusal(surface, carrier)
            };
            let sigma = match decide(
                "pcurve_spiric_chart_axis",
                // A COSINE of unit vectors: dimensionless, so it is
                // metered by multiplying the chart's own lever arm
                // (`Margin::levered`, whose docs name this exact case)
                // — not `over_lever`, which divides a measure by a
                // lever and belongs to an oriented area over a radius.
                Margin::levered(chart_axis.dot(axis), chart_major),
                band,
            )
            .map_err(|cause| {
                if cause.margin.is_invalid() {
                    // A poisoned read shows nothing about the axes: no
                    // claim that they are perpendicular, and no lever
                    // makes it readable (the defect ending).
                    spiric_off_own_chart(
                        "the chart torus's axis could not be read against the carrier's (its \
                         reading was poisoned)",
                    )
                    .refusal(surface, carrier)
                } else {
                    perpendicular()
                }
            })? {
                Sign::Positive => T::one(),
                Sign::Negative => T::zero() - T::one(),
                Sign::Zero => return Err(perpendicular()),
            };
            let torus = match decide(
                "pcurve_spiric_mint_center",
                Margin::norm3(chart_center - WallTorus::Own.center(center, n, offset)),
                band,
            )
            .map_err(incidence_escalated)?
            {
                Sign::Zero => WallTorus::Own,
                Sign::Positive | Sign::Negative => WallTorus::Mirror,
            };
            let toward = n * torus.mu::<T>();
            // The tube angle, `σ·t` from the carrier's own parameter, is an
            // angle of the chart: placed in the window at the span's
            // midpoint, levered at the tube's radius.
            let v0 = match place.turns(Along::Tube, sigma * place.mid(), band)? {
                0 => T::zero(),
                k => T::zero() - Period::new(T::tau()).times(k),
            };
            let wall = |u0| SpiricImage::Wall {
                u0,
                v0,
                sense: sigma,
                torus,
            };
            // The azimuth placed in the window at the span's midpoint, the
            // image read from `u0 = 0` being its motion there, levered at
            // the major radius (`n·μ` is a unit direction).
            let u0 = place.phase(
                Along::Azimuth,
                Azimuth::of(toward.dot(cv), toward.dot(chart_u)),
                |m| spiric(wall(T::zero())).eval(m).x,
                band,
            )?;
            Ok(spiric(wall(u0)))
        }
        Surface::Cylinder { .. } | Surface::Cone { .. } | Surface::Sphere { .. } => {
            let verdict = spiric_off_own_chart(
                "a spiric oval is an irreducible quartic, and every planar curve on a \
                 cylinder, cone or sphere is a conic",
            );
            Err(verdict.refusal(surface, carrier))
        }
        Surface::Nurbs(_) => Err(PcurveCertifyError::UnsupportedChart {
            chart: geom::SurfaceKind::Nurbs,
        }),
        Surface::Approx(_) => Err(PcurveCertifyError::UnsupportedChart {
            chart: geom::SurfaceKind::Approx,
        }),
    }
}

/// A conic carrier on a cone chart that is not a rim: off the cone, or
/// not shown off it ([`chart_incidence`]).
///
/// Its lowest point's height from the apex, `z_min = |w_z| − |(a_z, b_z)|`,
/// is decided first (`pcurve_cone_chart_nappe`): a conic that reaches the
/// apex's height leaves its nappe, and the apex plane meets the cone only
/// at the apex, so no circle or ellipse on the cone does.
///
/// # Errors
///
/// [`PcurveCertifyError::Escalated`] when `z_min` or the sampled
/// distance lands in the sliver band.
fn cone_conic_incidence<T: Decide>(
    carrier: &Curve3<T>,
    surface: &Surface<T>,
    band: Band,
) -> Result<(), PcurveCertifyError> {
    let (Some(form), &Surface::Cone { apex, axis, .. }) = (carrier_harmonic(carrier), surface)
    else {
        unreachable!("cone_conic_incidence: the cone arm reads a conic carrier")
    };
    let (wz, az, bz) = (
        (form.c - apex).dot(axis),
        form.a.dot(axis),
        form.b.dot(axis),
    );
    let z_min = wz.abs() - (az.powi(2) + bz.powi(2)).sqrt();
    // A pose reading: in the sliver the conic is not shown away from the
    // apex's height, the pose the in-band Zero below names, so it ends in
    // the pose lever too (`OffMargin::InBand`), never "not shown on the
    // face" of a conic that may lie on it.
    let near_apex = OffChart(
        "the conic's lowest point is not shown away from the apex's height, where the apex \
         plane meets the cone only at the apex",
    );
    match geom_core::k_stats::decide_reported("pcurve_cone_chart_nappe", Margin::of(z_min), band)
        .map_err(|cause| near_apex.with_margin(OffMargin::InBand(cause.margin), surface, carrier))?
    {
        Decided {
            sign: Sign::Positive,
            ..
        } => {}
        Decided {
            sign: Sign::Negative,
            margin,
        } => {
            return Err(OffChart(
                "the conic reaches past the apex's height, and the apex plane meets the cone \
                 only at the apex",
            )
            .with_margin(OffMargin::Past(margin), surface, carrier));
        }
        Decided {
            sign: Sign::Zero,
            margin,
        } => {
            return Err(near_apex.with_margin(OffMargin::InBand(margin), surface, carrier));
        }
    }
    match chart_incidence("pcurve_cone_chart_incident", carrier, surface, band)? {
        Incidence::On => Ok(()),
        Incidence::Off => Err(OffChart(
            "the conic's points are not on the cone (its distance from the cone at a sample is \
             past the band)",
        )
        .refusal(surface, carrier)),
    }
}

/// A spiric met with a chart that holds no oval of it: off the chart.
///
/// A spiric's offset is nonzero ([`geom::Curve3::spiric`] refuses a
/// plane through the axis, whose ovals are `Circle`s), so its oval is
/// an irreducible bicircular quartic (genus 1, the variant docs), and a
/// curve containing one oval of it contains all of it (Bézout). A
/// cylinder, cone or sphere meets the cutting plane in a conic, which
/// holds no such quartic. Another torus meets it in a bicircular
/// quartic too, and matching the two equations coefficient by
/// coefficient in the plane's frame forces a parallel axis, the same
/// `R` and `r`, and a centre either the carrier's or its MIRROR through
/// the plane (`center + 2·offset·n`). So the oval lies on its cutting
/// plane and on those two tori, each of which [`SpiricImage`] images,
/// and on no other analytic chart.
fn spiric_off_own_chart(why: &'static str) -> OffChart {
    OffChart(why)
}

// Check 4's per-arm lemma, swept, in a module of its own so the per-file
// test gate can skip the sweep without skipping the deterministic rows.
pub mod projected;
pub use projected::{FramedCarrier, ProjectedChart, ProjectedImage, SectorChannel};

#[cfg(test)]
mod envelope_lemma_fuzz;

/// The rows' door to the channel-taking checks: a harmonic image's two
/// channels, or a panic naming the row that handed something else.
#[cfg(test)]
#[allow(clippy::panic)]
fn channels_of<T: Real>(image: &Pcurve<T>) -> (Channel<T>, Channel<T>) {
    match *image {
        Pcurve::Harmonic { u, v } => (u, v),
        _ => panic!("the row hands a harmonic image"),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod harmonic_sup_rows;
#[cfg(test)]
mod mutants;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod placement_rows;
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod radial_pair_rows;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::{FRAC_PI_2, PI, TAU};
    use geom_core::Tol;

    use super::*;

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **A pcurve escalation renders at the door it is read at**
    /// ([`PcurveCertifyError::render`]): its defect ending names the
    /// kernel at a build, as its `Display` does, the kernel or the file at
    /// rest, and the file at the import door. Red if a door's render
    /// reads a build.
    #[test]
    fn a_pcurve_escalation_renders_at_its_door() {
        use geom_core::{
            DAMAGED_FILE_ENDING, FileCoincidence, KERNEL_DEFECT_ENDING,
            KERNEL_OR_FILE_DEFECT_ENDING, MarginDiag,
        };

        let error = PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartWinding,
            sample: 0,
            cause: Indeterminate {
                margin: MarginDiag::value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: None,
                terminal_sliver: false,
            },
        };
        assert!(error.to_string().ends_with(KERNEL_DEFECT_ENDING), "{error}");
        for (at, want) in [
            (ReadAt::Run(Reading::Build), KERNEL_DEFECT_ENDING),
            (ReadAt::Run(Reading::AtRest), KERNEL_OR_FILE_DEFECT_ENDING),
            (
                ReadAt::File(FileCoincidence::new(1e-6)),
                DAMAGED_FILE_ENDING,
            ),
        ] {
            let text = error.render(at);
            assert!(text.ends_with(want), "{at:?}: {text}");
        }
    }

    /// **Each pcurve check's sized decision names its refused side**
    /// (D4 ¶1 (iv)): the image's span and its period gates are each a
    /// stored contradiction, and past ε_in at the import door a reading
    /// in band wholly on it names the file. The table and the roster are
    /// exhaustive matches, so a check or envelope term added without a
    /// row does not compile; red if a side is flipped or the door names
    /// the kernel.
    #[test]
    fn each_pcurve_check_names_its_refused_side() {
        use crate::recourse::RefusedSide;
        use geom_core::{DAMAGED_FILE_ENDING, FileCoincidence, MarginDiag};

        let stored = Some(RefusedSide::StoredContradiction);
        let want = |check: PcurveCheck| match check {
            PcurveCheck::ParamSpan | PcurveCheck::AzimuthPeriod | PcurveCheck::TubePeriod => stored,
            PcurveCheck::MapResidual
            | PcurveCheck::Envelope
            | PcurveCheck::EnvelopeTerm(_)
            | PcurveCheck::ChartWinding
            | PcurveCheck::ChartClass
            | PcurveCheck::GeneralLane
            | PcurveCheck::ImageIdentity
            | PcurveCheck::ChartIncidence
            | PcurveCheck::Sector => None,
        };
        let in_band = Indeterminate {
            margin: MarginDiag::value(-2e-6),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: None,
            terminal_sliver: false,
        };
        let past = RefusedArm::Undecided(&in_band);
        let file = FileCoincidence::new(1e-6);
        // One list spells both the roster iterated and an exhaustive
        // match over it.
        macro_rules! every_check {
            ($($check:ident),+; $($term:ident),+) => {{
                let _ = |check: PcurveCheck| match check {
                    $(PcurveCheck::$check)|+ | PcurveCheck::EnvelopeTerm(_) => (),
                };
                let _ = |term: EnvelopeTerm| match term {
                    $(EnvelopeTerm::$term)|+ => (),
                };
                [$(PcurveCheck::$check),+]
                    .into_iter()
                    .chain([$(EnvelopeTerm::$term),+].map(PcurveCheck::EnvelopeTerm))
            }};
        }
        let checks = every_check!(
            ParamSpan, MapResidual, Envelope, ChartWinding, ChartClass, GeneralLane,
            ImageIdentity, AzimuthPeriod, TubePeriod, ChartIncidence, Sector;
            Frame, Centre, Radius, Line, Drift, FidelityU, FidelityV, Incidence, Fidelity
        );
        for check in checks {
            let side = match check.routing() {
                Routing::Sized(decision) => {
                    assert_eq!(
                        decision.recourse(past, file),
                        DAMAGED_FILE_ENDING,
                        "{check:?} past ε_in at the import door"
                    );
                    Some(decision.side)
                }
                Routing::Unsized(_) | Routing::NotYet | Routing::Incidence => None,
            };
            assert_eq!(side, want(check), "{check:?}");
        }
    }

    /// **A sphere parallel's twin read is one sign, and undecided it
    /// escalates typed.** A parallel of radius 0.5 on a sphere, its
    /// stored azimuth moved `Δ` off the placed image's, at `f64` and at
    /// `Interval`:
    /// - `Δ = π/2 + 3·zero/r`: the levered cosine sits in the band, so the
    ///   twin read escalates at the azimuth's term, naming
    ///   `pcurve_fidelity_twin`, rather than guess a name;
    /// - `Δ = π/2`: decided `Zero` on the half-way mark, read as `P_d`
    ///   (D9), and the quarter turn refuses as the azimuth's term;
    /// - `Δ = π`: the twin's azimuth, `v` left as it is, so the row is the
    ///   twin's quarter of a latitude off and refuses at the latitude's
    ///   term.
    #[test]
    fn a_sphere_parallels_twin_read_escalates_typed_in_band() {
        use geom_core::Interval;
        let (r, lat) = (0.5, 0.4_f64);
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: r,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let parallel = Curve3::Circle {
            center: Point3::new(0.0, 0.0, r * lat.sin()),
            axis: Vec3::unit_z(),
            radius: r * lat.cos(),
            u_ref: Vec3::unit_x(),
        };
        let span = (0.2, 2.9);
        let f = <Interval as Real>::from_f64;
        let at = |du: f64| {
            let row = chart_pcurve_over(&parallel, span.0, span.1, &sphere, band()).unwrap();
            let a = PcurveCache::certify(
                row.translate(du, 0.0),
                span.0,
                span.1,
                &parallel,
                &sphere,
                band(),
            )
            .map(|_| ());
            let (ci, si) = (parallel.map_scalar(f), sphere.map_scalar(f));
            let row = chart_pcurve_over(&ci, f(span.0), f(span.1), &si, band()).unwrap();
            let b = PcurveCache::certify(
                row.translate(f(du), f(0.0)),
                f(span.0),
                f(span.1),
                &ci,
                &si,
                band(),
            )
            .map(|_| ());
            [a, b]
        };
        let azimuth = PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityU);
        let map_residual = |got: &Result<(), PcurveCertifyError>| {
            matches!(
                got,
                Err(PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::MapResidual,
                    ..
                })
            )
        };
        // At `f64` the schedule (check 3) samples the quarter turn first.
        let [f64_row, interval] = at(FRAC_PI_2 + 3.0 * band().zero() / r);
        assert!(map_residual(&f64_row), "in band, f64: {f64_row:?}");
        assert!(
            matches!(
                interval,
                Err(PcurveCertifyError::Escalated { check, ref cause, .. })
                    if check == azimuth && cause.predicate == Some("pcurve_fidelity_twin")
            ),
            "in band, Interval: {interval:?}"
        );
        let [f64_row, interval] = at(FRAC_PI_2);
        assert!(map_residual(&f64_row), "on the mark, f64: {f64_row:?}");
        assert!(
            matches!(interval, Err(PcurveCertifyError::ResidualExceeded { check, .. }) if check == azimuth),
            "on the mark, Interval: {interval:?}"
        );
        let [f64_row, interval] = at(PI);
        assert!(
            map_residual(&f64_row),
            "the twin's azimuth, f64: {f64_row:?}"
        );
        assert!(
            matches!(
                interval,
                Err(PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityV),
                    ..
                })
            ),
            "the twin's azimuth, Interval: {interval:?}"
        );
    }

    /// `(p0, pa, pb, pl, t0, t1)` — harmonic images over spans that
    /// cover every shape the span box has an argument for: a rim edge
    /// starting at `t = 0`, one not starting there, a negative-direction
    /// span, spans past π and past a whole turn (the ball arm), a short
    /// span (the chord arm), and a negative-`t` mixed image.
    type HarmonicRow = ([f64; 2], [f64; 2], [f64; 2], [f64; 2], f64, f64);
    #[rustfmt::skip]
    const SPAN_BOX_ROWS: [HarmonicRow; 9] = [
        ([0.0, 0.3], [0.0; 2], [0.0; 2], [1.0, 0.0], 0.0, 0.383_972_435_438_752_5),
        ([0.0, 0.3], [0.0; 2], [0.0; 2], [1.0, 0.0], 2.0, 2.5),
        ([0.0, 0.3], [0.0; 2], [0.0; 2], [-1.0, 0.0], 2.5, 2.0),
        ([0.1, 0.2], [0.0, 0.5], [0.0, -0.3], [1.0, 0.0], 0.3, 0.3 + TAU),
        ([1.0, -2.0], [2.0, 0.5], [-0.7, 1.5], [0.0; 2], -1.0, 3.0),
        ([0.4, 0.0], [0.2, -0.1], [0.05, 0.3], [-0.5, 2.0], -3.0, -2.2),
        ([0.0; 2], [1.0, 0.0], [0.0, 1.0], [0.0; 2], 0.0, 3.0 * TAU),
        ([0.0; 2], [1.0, 0.0], [0.0, 1.0], [0.0; 2], 5.0, -1.0),
        ([0.5, 0.5], [0.3, -0.2], [0.1, 0.4], [0.7, 0.0], 1.0, 1.001),
    ];

    fn harmonic_row<T: Real>(row: &HarmonicRow) -> (Pcurve<T>, T, T) {
        let (p0, pa, pb, pl, t0, t1) = *row;
        let f = T::from_f64;
        let v = |c: [f64; 2]| Vec2::new(f(c[0]), f(c[1]));
        (
            Pcurve::free_harmonic(Point2::new(f(p0[0]), f(p0[1])), v(pa), v(pb), v(pl)),
            f(t0),
            f(t1),
        )
    }

    /// The parameters a row is sampled at: both ends and 4000 even
    /// steps between them, in the span's own direction.
    fn span_samples(t0: f64, t1: f64) -> impl Iterator<Item = f64> {
        const N: u32 = 4000;
        (0..=N).map(move |i| {
            if i == N {
                t1
            } else {
                t0 + (t1 - t0) * f64::from(i) / f64::from(N)
            }
        })
    }

    /// **The span box ENCLOSES the image at `f64`**, densely sampled
    /// on every row, and is at most its own charge looser than the
    /// sampled range: each end lies within `min(M·h²/8 + s, 2M)` of the
    /// sampled extreme (`M = hypot(a, b)` per channel) — the chord
    /// arm's dip and the ball arm's full swing, plus, on a channel
    /// mixing a linear and a trigonometric part, the trigonometric
    /// part's endpoint swing `s = |T(t₁) − T(t₀)|` the sum of the two
    /// parts' boxes gives up. A linear channel has `M = 0`, so its box
    /// is exactly its endpoint hull.
    #[test]
    fn the_harmonic_span_box_encloses_its_image_and_is_no_looser_than_its_charge() {
        for row in &SPAN_BOX_ROWS {
            let (pcurve, t0, t1) = harmonic_row::<f64>(row);
            let b = pcurve.chart_box(t0, t1);
            let (mut u, mut v) = ((f64::MAX, f64::MIN), (f64::MAX, f64::MIN));
            for t in span_samples(t0, t1) {
                let p = pcurve.eval(t);
                u = (u.0.min(p.x), u.1.max(p.x));
                v = (v.0.min(p.y), v.1.max(p.y));
            }
            let h = (t1 - t0).abs();
            let (_, pa, pb, pl, ..) = *row;
            let charge = |a: [f64; 2], b: [f64; 2], i: usize| {
                let m = a[i].hypot(b[i]);
                let trig = |t: f64| a[i] * t.cos() + b[i] * t.sin();
                let swing = if pl[i] == 0.0 {
                    0.0
                } else {
                    (trig(t1) - trig(t0)).abs()
                };
                (m * h * h / 8.0 + swing).min(2.0 * m)
            };
            let rounding = 1e-12;
            for (lo, hi, sampled, charge, what) in [
                (b.u_min, b.u_max, u, charge(pa, pb, 0), "u"),
                (b.v_min, b.v_max, v, charge(pa, pb, 1), "v"),
            ] {
                assert!(
                    lo <= sampled.0 + rounding && hi >= sampled.1 - rounding,
                    "row {row:?}: the {what} box [{lo}, {hi}] misses the sampled image {sampled:?}"
                );
                assert!(
                    sampled.0 - lo <= charge + rounding && hi - sampled.1 <= charge + rounding,
                    "row {row:?}: the {what} box [{lo}, {hi}] is looser than its charge {charge} \
                     over the sampled image {sampled:?}"
                );
            }
        }
    }

    /// **The span box ENCLOSES the image at `Interval`**: on every row
    /// no sample's image enclosure lies certainly outside the box. The
    /// true point is in both, so a box end's bracket past the far end
    /// of the image's is a miss; the near ends may cross by rounding.
    #[test]
    fn the_harmonic_span_box_encloses_its_image_at_interval() {
        use geom_core::{Bounds, Interval};
        for row in &SPAN_BOX_ROWS {
            let (pcurve, t0, t1) = harmonic_row::<Interval>(row);
            let b = pcurve.chart_box(t0, t1);
            for t in span_samples(row.4, row.5) {
                let p = pcurve.eval(Interval::from_f64(t));
                for (lo, hi, at, what) in
                    [(b.u_min, b.u_max, p.x, "u"), (b.v_min, b.v_max, p.y, "v")]
                {
                    assert!(
                        lo.lo() <= at.hi() && hi.hi() >= at.lo(),
                        "row {row:?}, t = {t}: the {what} box [{}, {}] misses the image's \
                         enclosure [{}, {}]",
                        lo.lo(),
                        hi.hi(),
                        at.lo(),
                        at.hi()
                    );
                }
            }
        }
    }

    /// The chart-side winding gate ends in the edge certifier's winding
    /// lever on both arms: a definite excess in the lever alone at a build
    /// and as `CertifyError::WindingExceeded` does at rest, where both are
    /// a contradiction of what is stored, and an in-band headroom as
    /// `CertCheck::ParamWinding`'s undecided arm does (D4 ¶1 (iv)). Its
    /// side is a stored contradiction, not unminted: a spline image can
    /// sweep past a period from input no door refuses.
    #[test]
    fn the_azimuth_gate_ends_in_the_edge_winding_lever() {
        use crate::CertCheck;
        use crate::certify::{CertifyError, recourse};
        let cause = Indeterminate {
            margin: geom_core::MarginDiag::value(5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("pcurve_azimuth_period"),
            terminal_sliver: false,
        };
        assert_eq!(
            PcurveCertifyError::AzimuthPeriodExceeded.ending(Reading::Build),
            Some(format!("Recourse: {}", SpanDecision::Winding.lever())),
            "definite, at a build"
        );
        assert_eq!(
            PcurveCertifyError::AzimuthPeriodExceeded.ending(Reading::AtRest),
            CertifyError::WindingExceeded.ending(Reading::AtRest),
            "definite, at rest"
        );
        for reading in [Reading::Build, Reading::AtRest] {
            let escalated = PcurveCertifyError::Escalated {
                check: PcurveCheck::AzimuthPeriod,
                sample: 0,
                cause,
            };
            assert_eq!(
                escalated.ending(reading),
                Some(recourse(
                    CertCheck::ParamWinding,
                    RefusedArm::Undecided(&cause),
                    reading
                )),
                "in band, {reading:?}"
            );
        }
    }

    /// **Both escalation arms tell a gate's rejected sign.** Each renders
    /// its cause's payload before its decision's ending, and the payload
    /// carries the word of a reading a verdict was reached on: a gate's
    /// rejection past the band reads "a decided sign this decision cannot
    /// use" there, whatever ending follows.
    #[test]
    fn both_escalation_arms_tell_a_gates_rejected_sign() {
        use crate::ssi::{SsiDecision, TraceDecision};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let cause = geom_core::k_stats::gate_rejection(
            "pcurve_test_gate",
            geom_core::Decided {
                sign: geom_core::Sign::Negative,
                margin: geom_core::MarginDiag::value(-5e-8),
            },
            band,
            geom_core::SizedPass::Positive,
        );
        let told = "margin -5e-8 lies past the ambiguity band (1e-9, 1e-8) — a decided sign this \
                    decision cannot use. ";
        let escalated = PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartIncidence,
            sample: 0,
            cause,
        }
        .to_string();
        assert!(
            escalated.contains(&format!("escalated: {told}Recourse:")),
            "`Escalated`: {escalated}"
        );
        let fitted = PcurveCertifyError::FittedEscalated {
            decision: SsiDecision::Trace(TraceDecision::ClosureReturn),
            cause,
        }
        .to_string();
        assert!(
            fitted.contains(&format!(" is undecided: {told}")),
            "`FittedEscalated`: {fitted}"
        );
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
            semi_u: r / tilt.cos(),
            semi_v: r,
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
        let Some((p0, pa, pb, pl)) = p.harmonic_form() else {
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
        let cache = PcurveCache::certify(p, 0.0, PI, &carrier, &cyl, band()).unwrap();
        assert!(cache.certificate().max_residual < 1e-14);
        assert!(cache.certificate().envelope < 1e-14);
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
        let Some((p0, pa, pb, pl)) = p.harmonic_form() else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15 && (p0.y - h).abs() < 1e-15);
        assert!(pa.y.abs() < 1e-15 && pb.y.abs() < 1e-15 && pl.y.abs() < 1e-15);
        assert!((pl.x - 1.0).abs() < 1e-15);
        let cache = PcurveCache::certify(p, 0.0, FRAC_PI_2, &carrier, &cyl, band()).unwrap();
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
        let p = chart_pcurve_over(&carrier, 0.0, FRAC_PI_2, &cyl, band()).unwrap();
        let Some((_, _, _, pl)) = p.harmonic_form() else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!((pl.x + 1.0).abs() < 1e-15);
        PcurveCache::certify(p, 0.0, FRAC_PI_2, &carrier, &cyl, band()).unwrap();
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
        let Some((p0, pa, pb, pl)) = p.harmonic_form() else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15 && p0.y.abs() < 1e-15);
        assert!(pa.x.abs() < 1e-15 && pb.x.abs() < 1e-15 && pl.x.abs() < 1e-15);
        assert!((pl.y - 1.0).abs() < 1e-15);
        PcurveCache::certify(p, 0.0, 1.0, &carrier, &cyl, band()).unwrap();
    }

    /// The SAME seam edge, on the SAME surface, carries two DIFFERENT
    /// chart curves — the `u = 0` and `u = 2π` branches. This is the
    /// under-keying counterexample in miniature (the body-level row
    /// lives in `topo`): a per-edge-per-face key cannot hold both. Only
    /// the placed one is a row (C4: a row is its placed image); the other
    /// is its lifted copy, the row with a deck element, and certification
    /// refuses it as a row, the whole period read as its offset.
    #[test]
    fn a_seam_edge_carries_two_different_branches_of_one_surface() {
        let r = 0.5;
        let cyl = cylinder(r);
        let carrier = Curve3::Line {
            origin: Point3::new(r, 0.0, 0.0),
            dir: Vec3::unit_z(),
        };
        let base = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let wrapped = base.shift_branch(1, Period::new(TAU));
        let wrapped_for_shift = wrapped.clone();
        let Some((a, _, _, _)) = base.harmonic_form() else {
            panic!("the closed-form lane stores harmonic images")
        };
        let Some((b, _, _, _)) = wrapped.harmonic_form() else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!((b.x - a.x - TAU).abs() < 1e-15, "different chart curves");
        // The placed branch certifies; the lifted copy does not, as a
        // row: which lift a half-edge's loop walks is its joints' deck
        // elements (`topo::pcurves`), never a second row.
        PcurveCache::certify(base, 0.0, 1.0, &carrier, &cyl, band()).unwrap();
        let lifted = PcurveCache::certify(wrapped_for_shift, 0.0, 1.0, &carrier, &cyl, band());
        assert!(
            matches!(
                lifted,
                Err(PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityU),
                    ..
                })
            ),
            "a whole-period shift is no row: {lifted:?}"
        );
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
            semi_u: 0.7,
            semi_v: 0.3,
            u_ref: Vec3::unit_x(),
        };
        let p = chart_pcurve(&carrier, &plane, band()).unwrap();
        let cache = PcurveCache::certify(p, 0.2, 2.0, &carrier, &plane, band()).unwrap();
        assert!(cache.certificate().max_residual < 1e-15);
        assert!(cache.certificate().envelope < 1e-15);
    }

    /// `p`'s harmonic channels with `du` added to the first's constant
    /// and `dv` to the second's four coefficients `(c, a, b, l)`: a
    /// corruption a row makes of a derived image. An angle channel takes
    /// a constant only; its slope is its windings, which no corruption
    /// of a number reaches.
    fn bumped(p: &Pcurve<f64>, du: f64, dv: [f64; 4]) -> Pcurve<f64> {
        let Pcurve::Harmonic { u, v } = *p else {
            panic!("a harmonic image")
        };
        let bump = |c: Channel<f64>, d: [f64; 4]| match c {
            Channel::Angle { phase, winding } => {
                assert_eq!(
                    d[1..],
                    [0.0; 3],
                    "an angle takes no trigonometric or linear part"
                );
                Channel::Angle {
                    phase: phase + d[0],
                    winding,
                }
            }
            Channel::Free { c, a, b, l } => Channel::Free {
                c: c + d[0],
                a: a + d[1],
                b: b + d[2],
                l: l + d[3],
            },
        };
        Pcurve::Harmonic {
            u: bump(u, [du, 0.0, 0.0, 0.0]),
            v: bump(v, dv),
        }
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
        let nudge = 1e-3;
        let corruptions = [
            bumped(&good, nudge, [0.0; 4]),
            bumped(&good, 0.0, [nudge, 0.0, 0.0, 0.0]),
            bumped(&good, 0.0, [0.0, nudge, 0.0, 0.0]),
            bumped(&good, 0.0, [0.0, 0.0, nudge, 0.0]),
        ];
        for (i, bad) in corruptions.into_iter().enumerate() {
            let out = PcurveCache::certify(bad, 0.0, PI, &carrier, &cyl, band());
            assert!(
                matches!(out, Err(PcurveCertifyError::ResidualExceeded { .. })),
                "corruption {i} certified: {out:?}"
            );
        }
    }

    /// **Windings are structure, compared exactly** (C4; `[ev]` PR 5113's
    /// first decision). On a rim (derived winding 1) and a seam line
    /// (derived 0) of a cylinder, the row whose azimuth stores the derived
    /// count certifies, at `f64` and at `Interval`; every count one off it
    /// refuses at both lanes. At `Interval` check 3's schedule is not run,
    /// so the refusal is the compare's own: `WindingMismatch`, naming
    /// both counts, reached in check 4's fidelity term. At `f64` check
    /// 3's cross-check reads the one-off image first, and refuses it as
    /// its residual (`ResidualExceeded` at `MapResidual`). A count no
    /// closed-form table holds (the rim's 2) refuses at check 1 as
    /// `ChartWindingUnsupported`, at both lanes.
    #[test]
    fn a_rows_windings_are_compared_exactly() {
        use geom_core::Interval;
        let f = Interval::from_f64;
        let rim = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let seam = Curve3::Line {
            origin: Point3::new(0.5, 0.0, 0.0),
            dir: Vec3::unit_z(),
        };
        for (name, carrier, derived) in [("rim", rim, 1), ("seam", seam, 0)] {
            let Pcurve::Harmonic {
                u: Channel::Angle { phase, winding },
                v,
            } = chart_pcurve(&carrier, &cylinder(0.5), band()).unwrap()
            else {
                panic!("{name}: a cylinder row's azimuth is an angle")
            };
            assert_eq!(winding, derived, "{name}: the derived count");
            let at = |w: i32| Pcurve::Harmonic {
                u: Channel::Angle { phase, winding: w },
                v,
            };
            let lift_channel = |c: Channel<f64>| match c {
                Channel::Angle { phase, winding } => Channel::Angle {
                    phase: f(phase),
                    winding,
                },
                Channel::Free { c, a, b, l } => Channel::Free {
                    c: f(c),
                    a: f(a),
                    b: f(b),
                    l: f(l),
                },
            };
            let lift = |p: Pcurve<f64>| {
                let (u, v) = channels_of(&p);
                Pcurve::Harmonic {
                    u: lift_channel(u),
                    v: lift_channel(v),
                }
            };
            let (cyl, carrier_iv) = (cylinder(0.5).map_scalar(f), carrier.map_scalar(f));
            for stored in [derived - 1, derived, derived + 1] {
                let at_f64 =
                    PcurveCache::certify(at(stored), 0.0, PI, &carrier, &cylinder(0.5), band());
                let at_iv = PcurveCache::certify(
                    lift(at(stored)),
                    f(0.0),
                    f(PI),
                    &carrier_iv,
                    &cyl,
                    band(),
                );
                if stored == derived {
                    assert!(
                        at_f64.is_ok() && at_iv.is_ok(),
                        "{name}: the derived count certifies: {at_f64:?} / {at_iv:?}"
                    );
                    continue;
                }
                // A count no closed-form table holds refuses at check 1, at
                // both lanes, before anything is read.
                if stored.abs() > 1 {
                    assert!(
                        matches!(at_f64, Err(PcurveCertifyError::ChartWindingUnsupported))
                            && matches!(at_iv, Err(PcurveCertifyError::ChartWindingUnsupported)),
                        "{name}, stored {stored}: check 1 refuses: {at_f64:?} / {at_iv:?}"
                    );
                    continue;
                }
                assert!(
                    matches!(
                        at_f64,
                        Err(PcurveCertifyError::ResidualExceeded {
                            check: PcurveCheck::MapResidual,
                            ..
                        })
                    ),
                    "{name}, stored {stored}: f64 refuses at check 3: {at_f64:?}"
                );
                assert!(
                    matches!(
                        at_iv,
                        Err(PcurveCertifyError::WindingMismatch {
                            stored: [Some(s), None],
                            derived: [Some(d), None],
                        }) if s == stored && d == derived
                    ),
                    "{name}, stored {stored}: Interval refuses at the compare: {at_iv:?}"
                );
            }
        }
    }

    /// **A row's channels are its chart's.** The same rim's image with
    /// its azimuth stored as a free channel, and a plane's circle with
    /// its first coordinate stored as an angle, each refuse as the
    /// channels' mismatch at check 1, before any number is read.
    #[test]
    fn a_rows_channels_are_its_charts() {
        let cyl = cylinder(0.5);
        let rim = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let Some((p0, pa, pb, pl)) = chart_pcurve(&rim, &cyl, band()).unwrap().harmonic_form()
        else {
            panic!("a rim images harmonic")
        };
        let out = PcurveCache::certify(
            Pcurve::free_harmonic(p0, pa, pb, pl),
            0.0,
            PI,
            &rim,
            &cyl,
            band(),
        );
        assert!(
            matches!(
                out,
                Err(PcurveCertifyError::ChannelKindMismatch {
                    chart: geom::SurfaceKind::Cylinder
                })
            ),
            "{out:?}"
        );
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let Pcurve::Harmonic { u, v } = chart_pcurve(&rim, &plane, band()).unwrap() else {
            panic!("a circle in a plane images harmonic")
        };
        let [c, ..] = u.coefficients();
        let angled = Pcurve::Harmonic {
            u: Channel::fixed_angle(c),
            v,
        };
        let out = PcurveCache::certify(angled, 0.0, PI, &rim, &plane, band());
        assert!(
            matches!(
                out,
                Err(PcurveCertifyError::ChannelKindMismatch {
                    chart: geom::SurfaceKind::Plane
                })
            ),
            "{out:?}"
        );
    }

    /// The envelope is a genuine sup bound over the whole span, not a
    /// sampled max: it dominates a dense resampling of the residual.
    #[test]
    fn the_envelope_dominates_a_dense_resampling() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let good = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let Some((_, pa, _, _)) = good.harmonic_form() else {
            panic!("the closed-form lane stores harmonic images")
        };
        // A deliberately imperfect pcurve, so the envelope is not a
        // degenerate zero.
        let bad = bumped(&good, 0.0, [0.0, pa.y * 0.01, 0.0, 0.0]);
        let Some(image) = chart_image_harmonic(&bad, &cyl) else {
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

    /// A minted image's angle channel has no trigonometric part and
    /// its windings' rate, and its envelope pays nothing for them.
    #[test]
    fn a_minted_shape_cache_pays_no_snap_slack() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let p = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let Some((_, pa, pb, pl)) = p.harmonic_form() else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert_eq!((pa.x, pb.x), (0.0, 0.0));
        assert_eq!(pl.x, 1.0);
        let cache = PcurveCache::certify(p, 0.0, PI, &carrier, &cyl, band()).unwrap();
        assert!(cache.certificate().envelope < 1e-14);
    }

    /// The review probe's second attack, kept: tuned near-cancelling
    /// harmonic + linear corruptions cannot hide under the envelope
    /// (the residual lives in `span{1, cos, sin, t}`, and the envelope
    /// reads its constant and linear parts at their size and its
    /// `cos`/`sin` pair at its exact sup, so the oracle's termwise sum
    /// here is over it), and certification refuses them.
    #[test]
    fn cancellation_cannot_beat_the_envelope() {
        let (r, h, tilt) = (0.5, 0.5, 0.3);
        let cyl = cylinder(r);
        let carrier = tilted_section(r, h, tilt);
        let good = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let d = 1e-4;
        let combos = [
            (d, -d, 0.0, 0.0),
            (d, -d, 2.0 * d / PI, -d),
            (-d, d, -2.0 * d / PI, d),
            (d, d, -2.0 * d / PI, 0.0),
        ];
        for (ca, cb, cl, c0) in combos {
            let bad = bumped(&good, 0.0, [c0, ca, cb, cl]);
            let env = c0.abs() + ca.abs() + cb.abs() + cl.abs() * PI;
            let sup = true_sup(&bad, &cyl, &carrier, 0.0, PI);
            assert!(
                sup <= env * (1.0 + 1e-12),
                "sup {sup:e} beats envelope {env:e} for ({ca},{cb},{cl},{c0})"
            );
            assert!(
                PcurveCache::certify(bad, 0.0, PI, &carrier, &cyl, band()).is_err(),
                "an off-by-1e-4 pcurve certified"
            );
        }
    }

    /// The term [`EnvelopeTerms::first_over`] names for a carrier or an
    /// image moved by `100·ε` in one respect — what a refused check 4
    /// carries on a periodic chart.
    fn named_term(stored: &Pcurve<f64>, carrier: &Curve3<f64>, s: &Surface<f64>) -> EnvelopeTerm {
        let form = carrier_harmonic(carrier).unwrap();
        let terms = periodic_envelope(channels_of(stored), carrier, form, s, (0.0, PI), PI, band())
            .unwrap();
        terms
            .first_over(band())
            .unwrap_or_else(|| panic!("no term over the band alone: {:?}", terms.0))
    }

    /// **A refused envelope is a bound, read at rest as one.** A rim
    /// whose stored image drifts axially by `l·(t − t_mid)` over a short
    /// window far from `t = 0` misses by at most `|l|·w/2`, under the
    /// band, yet its fidelity term levers the drift by the window's
    /// reach from the origin, and refuses definitely. At rest that
    /// refusal ends in the kernel-limit recourse and claims only that
    /// the certificate cannot show the image; a rim off the axis by
    /// `100·ε`, whose sample residual is the miss itself, ends in the
    /// defect ending.
    #[test]
    fn a_refused_envelope_reads_as_a_bound_at_rest() {
        use crate::recourse::{Reading, defect_ending};
        let r = 0.5;
        let cyl = cylinder(r);
        let rim = |center: Point3<f64>| Curve3::Circle {
            center,
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let carrier = rim(Point3::origin());
        // The drift's true miss is half the zero band; the term levers
        // it by about `2·t0/w` past that.
        let (t0, w) = (100.0, 0.1);
        let good = chart_pcurve_over(&carrier, t0, t0 + w, &cyl, band()).unwrap();
        let l = band().zero() / w;
        let mid = t0 + w / 2.0;
        let drifting = bumped(&good, 0.0, [-(l * mid), 0.0, 0.0, l]);
        assert!(
            l * w / 2.0 < band().zero(),
            "the drift's true miss is inside the zero band"
        );
        let loose = PcurveCache::certify(drifting, t0, t0 + w, &carrier, &cyl, band())
            .map(|_| ())
            .expect_err("the drift's fidelity term refuses");
        assert!(
            matches!(
                loose,
                PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityV),
                    ..
                }
            ),
            "a definite refusal on the axial fidelity: {loose:?}"
        );
        assert_eq!(
            loose.ending(Reading::AtRest).as_deref(),
            Some(geom_core::KERNEL_LIMIT_RECOURSE),
            "a loose envelope contradicts nothing stored"
        );
        assert!(
            loose.to_string().contains("the certificate cannot show"),
            "{loose}"
        );
        let off = rim(Point3::new(100.0 * Tol::witness().eps(), 0.0, 0.0));
        let image = chart_pcurve(&off, &cyl, band()).unwrap();
        let miss = PcurveCache::certify(image, 0.0, PI, &off, &cyl, band())
            .map(|_| ())
            .expect_err("a rim off the axis by 100·ε refuses");
        assert!(
            matches!(
                miss,
                PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::MapResidual,
                    ..
                }
            ),
            "its samples read the miss: {miss:?}"
        );
        assert_eq!(
            miss.ending(Reading::AtRest).as_deref(),
            Some(defect_ending(Reading::AtRest)),
            "a sample's residual is the miss"
        );
    }

    /// **A refused envelope names the incidence that failed.** Each row
    /// moves the carrier off the cylinder, or the stored image off the
    /// one derived from it, in exactly one respect, and the term the
    /// refusal carries is that respect's.
    #[test]
    fn a_refused_envelope_names_the_incidence_that_failed() {
        let r = 0.5;
        let cyl = cylinder(r);
        let d = 100.0 * Tol::witness().eps();
        let rim = |center: Point3<f64>, axis: Vec3<f64>, radius: f64| Curve3::Circle {
            center,
            axis,
            radius,
            u_ref: Vec3::unit_x(),
        };
        let own = |carrier: &Curve3<f64>| chart_pcurve(carrier, &cyl, band()).unwrap();
        // `b` short of `a`'s quarter turn by `d`, `a` on the chart: the
        // turning pair off the class circle, the radius's own term.
        let squashed = Curve3::Ellipse {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            semi_u: r,
            semi_v: r - d,
            u_ref: Vec3::unit_x(),
        };
        let rows = [
            (
                rim(Point3::new(d, 0.0, 0.0), Vec3::unit_z(), r),
                EnvelopeTerm::Centre,
            ),
            (
                rim(Point3::origin(), Vec3::unit_z(), r + d),
                EnvelopeTerm::Radius,
            ),
            (squashed, EnvelopeTerm::Radius),
            // A radial slope over the band reads as a moving carrier;
            // one inside the escalation band takes the meridian (the
            // D9 tie-break) and lands in the line's own term.
            (
                Curve3::Line {
                    origin: Point3::new(r, 0.0, 0.0),
                    dir: Vec3::new(2.0 * Tol::witness().eps(), 0.0, 1.0),
                },
                EnvelopeTerm::Line,
            ),
        ];
        for (carrier, term) in rows {
            assert_eq!(
                named_term(&own(&carrier), &carrier, &cyl),
                term,
                "{carrier:?}"
            );
        }
        let carrier = rim(Point3::origin(), Vec3::unit_z(), r);
        let good = own(&carrier);
        for (stored, term) in [
            (bumped(&good, d, [0.0; 4]), EnvelopeTerm::FidelityU),
            (
                bumped(&good, 0.0, [d, 0.0, 0.0, 0.0]),
                EnvelopeTerm::FidelityV,
            ),
        ] {
            assert_eq!(named_term(&stored, &carrier, &cyl), term);
        }
    }

    /// **Check 4 refusing on its own carries its term.** The stored
    /// height is `0.6·ε` below the carrier's and rises `0.6·ε/π` per
    /// radian fast, so the two drifts cancel at the end of the span and
    /// no schedule sample sees more than `0.6·ε`; the envelope adds
    /// them, `1.2·ε`, and refuses naming the axial channel's fidelity.
    /// (The azimuth's slope is its windings, which no drift reaches.)
    #[test]
    fn an_envelope_over_the_band_between_samples_names_its_term() {
        let r = 0.5;
        let cyl = cylinder(r);
        let carrier = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let off = 0.6 * Tol::witness().eps();
        let stored = bumped(
            &chart_pcurve(&carrier, &cyl, band()).unwrap(),
            0.0,
            [-off, 0.0, 0.0, off / PI],
        );
        let out = PcurveCache::certify(stored, 0.0, PI, &carrier, &cyl, band());
        assert!(
            matches!(
                out,
                Err(PcurveCertifyError::Escalated {
                    check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityV),
                    ..
                } | PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::EnvelopeTerm(EnvelopeTerm::FidelityV),
                    ..
                })
            ),
            "{out:?}"
        );
    }

    /// A circle in a plane holding the cylinder's axis is no carrier of
    /// the cylinder, and its radial parts are parallel, so the
    /// derivation decides it MOVING with a zero winding — the one
    /// moving class whose image does not turn. The envelope still
    /// bounds the residual of the image it derives, and refuses.
    #[test]
    fn a_moving_carrier_with_no_winding_is_still_bounded() {
        let r = 0.5;
        let cyl = cylinder(r);
        let carrier = Curve3::Circle {
            center: Point3::new(r, 0.0, 0.0),
            axis: Vec3::unit_y(),
            radius: 0.1,
            u_ref: Vec3::unit_x(),
        };
        let (image, derivation) = derive_harmonic(
            &carrier,
            &cyl,
            super::Placement::own(&carrier, &cyl),
            band(),
        )
        .unwrap();
        assert!(
            matches!(derivation, Derivation::CylinderMoving { beta: 0 }),
            "{derivation:?}"
        );
        let form = carrier_harmonic(&carrier).unwrap();
        let envelope = periodic_envelope(
            channels_of(&image),
            &carrier,
            form,
            &cyl,
            (0.0, PI),
            PI,
            band(),
        )
        .unwrap()
        .total();
        assert!(envelope >= true_sup(&image, &cyl, &carrier, 0.0, PI));
        assert!(PcurveCache::certify(image, 0.0, PI, &carrier, &cyl, band()).is_err());
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
        let Some((p0, pa, pb, pl)) = chart_pcurve(&carrier, &sphere, band())
            .unwrap()
            .harmonic_form()
        else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15 && p0.y.abs() < 1e-15);
        assert!((pl.x - 1.0).abs() < 1e-15 && pl.y.abs() < 1e-15);
        assert!(pa.x.abs() < 1e-15 && pa.y.abs() < 1e-15);
        assert!(pb.x.abs() < 1e-15 && pb.y.abs() < 1e-15);
        // A tilted circle's azimuth is non-harmonic: its image is the
        // projected one, never a fit.
        let tilt = 0.6_f64;
        let tilted = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(tilt.sin(), 0.0, tilt.cos()),
            radius: 1.0,
            u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
        };
        let got = chart_pcurve(&tilted, &sphere, band());
        assert!(
            matches!(got, Ok(Pcurve::Projected(_))),
            "a tilted great circle lies on the sphere: {got:?}"
        );
        // The incidence test splits the two gates' failures: a
        // vertical SMALL circle (plane x = 0.6, radius 0.8) lies on
        // the unit sphere and fails only the centring gate; the same
        // tilted plane's circle at radius 0.5 lies off it.
        let vertical = Curve3::Circle {
            center: Point3::new(0.6, 0.0, 0.0),
            axis: Vec3::unit_x(),
            radius: 0.8,
            u_ref: Vec3::unit_z(),
        };
        let got = chart_pcurve(&vertical, &sphere, band());
        assert!(
            matches!(got, Ok(Pcurve::Projected(_))),
            "a vertical small circle lies on the sphere: {got:?}"
        );
        for (what, off) in [
            (
                "tilted, wrong radius",
                Curve3::Circle {
                    center: Point3::new(0.0, 0.0, 0.0),
                    axis: Vec3::new(tilt.sin(), 0.0, tilt.cos()),
                    radius: 0.5,
                    u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
                },
            ),
            (
                "vertical, wrong radius",
                Curve3::Circle {
                    center: Point3::new(0.6, 0.0, 0.0),
                    axis: Vec3::unit_x(),
                    radius: 0.7,
                    u_ref: Vec3::unit_z(),
                },
            ),
        ] {
            let err = chart_pcurve(&off, &sphere, band()).unwrap_err();
            assert!(
                matches!(
                    err,
                    PcurveCertifyError::CarrierOffChart { why, .. }
                        if why.contains("the sphere's radius")
                ),
                "{what}: the incidence test answers: {err:?}"
            );
        }
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
        let Some((p0, _, _, pl)) = chart_pcurve(&rim, &cone, band()).unwrap().harmonic_form()
        else {
            panic!("the closed-form lane stores harmonic images")
        };
        assert!(p0.x.abs() < 1e-15 && (p0.y - h / ha.cos()).abs() < 1e-12);
        assert!((pl.x - 1.0).abs() < 1e-15 && pl.y.abs() < 1e-15);
        // What stays refused on the cone chart, TYPED: an ellipse is
        // not a rim, and the incidence test says whether it is a
        // section (uncovered — `tests/chart_incidence.rs` builds one)
        // or off the cone, as this hand-placed one is.
        let ellipse = Curve3::Ellipse {
            center: Point3::new(0.2, 0.0, 2.0),
            axis: Vec3::new(0.3_f64.sin(), 0.0, 0.3_f64.cos()),
            semi_u: 1.2,
            semi_v: 1.0,
            u_ref: Vec3::new(0.3_f64.cos(), 0.0, -0.3_f64.sin()),
        };
        let err = chart_pcurve(&ellipse, &cone, band()).unwrap_err();
        assert!(
            matches!(
                err,
                PcurveCertifyError::CarrierOffChart {
                    chart: geom::SurfaceKind::Cone,
                    carrier: geom::CurveKind::Ellipse,
                    why,
                    ..
                } if why.contains("not on the cone")
            ),
            "{err:?}"
        );
    }

    /// **A line in the cone's apex plane over a sub-band span is not
    /// called off its face.** Over a fifth of the zero band of its length
    /// about the apex (`2e-10` at the default ε),
    /// the incidence test does not show the line off the cone, and the
    /// apex-plane reading lands its band-decided Zero: at this tolerance
    /// the line sits where the apex plane meets the cone only at the apex.
    /// The refusal carries that in-band margin ([`OffMargin::InBand`]), and
    /// its Display does not say the edge is off its face. At `f64` and at
    /// `Interval`, at every ε.
    #[test]
    fn an_apex_plane_line_on_a_sub_band_span_is_not_called_off_its_face() {
        fn read<T: Decide>(at: impl Fn(f64) -> T) -> PcurveCertifyError {
            let cone = Surface::Cone {
                apex: Point3::origin(),
                axis: Vec3::unit_z(),
                half_angle: 0.5,
                u_ref: Vec3::unit_x(),
            }
            .map_scalar(&at);
            let line = Curve3::Line {
                origin: Point3::origin(),
                dir: Vec3::unit_x(),
            }
            .map_scalar(&at);
            let half = 0.1 * band().zero();
            chart_pcurve_over(&line, at(-half), at(half), &cone, band())
                .expect_err("the apex plane meets the cone only at the apex")
        }
        for (lane, err) in [
            ("f64", read(|x| x)),
            (
                "Interval",
                read(<geom_core::Interval as geom_core::Real>::from_f64),
            ),
        ] {
            assert!(
                matches!(
                    err,
                    PcurveCertifyError::CarrierOffChart {
                        why,
                        margin: Some(OffMargin::InBand(_)),
                        ..
                    } if why.contains("the plane through the apex")
                ),
                "{lane}: {err:?}"
            );
            let shown = err.to_string();
            assert!(
                !shown.contains("not on its face") && shown.contains("within the band"),
                "{lane}: {shown}"
            );
        }
    }

    /// **The carrier refusals split by meaning**: a carrier that cannot
    /// lie on the chart is `CarrierOffChart`, a pair it can hold that a
    /// closed form images certifies. Each
    /// row builds its case, and the off-chart rows' geometric claims
    /// are argued at their sites in `chart_pcurve`.
    #[test]
    fn uncovered_and_off_chart_carriers_refuse_apart() {
        let cone = Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: 0.5,
            u_ref: Vec3::unit_x(),
        };
        let sphere = Surface::Sphere {
            center: Point3::origin(),
            radius: 1.0,
            axis: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let (major, minor) = (2.0_f64, 1.0_f64);
        let torus = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: major,
            minor_radius: minor,
            u_ref: Vec3::unit_x(),
        };
        let circle =
            |center: Point3<f64>, axis: Vec3<f64>, radius: f64, u_ref: Vec3<f64>| Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            };
        // Each row names its site by a word only that site's `why`
        // carries, so a row answered by a different site goes red.
        let off: [(&str, &str, Curve3<f64>, &Surface<f64>); 6] = [
            (
                "a line in the cone's apex plane",
                "the line's points are not on the chart",
                Curve3::Line {
                    origin: Point3::new(1.0, 0.0, 0.0),
                    dir: Vec3::unit_y(),
                },
                &cone,
            ),
            (
                "a tilted circle on a cone",
                "not on the cone",
                circle(
                    Point3::new(0.0, 0.0, 2.0),
                    Vec3::new(0.3_f64.sin(), 0.0, 0.3_f64.cos()),
                    1.0,
                    Vec3::unit_y(),
                ),
                &cone,
            ),
            (
                "a cone rim centred off the axis",
                "not on the cone",
                circle(
                    Point3::new(0.1, 0.0, 2.0),
                    Vec3::unit_z(),
                    1.0,
                    Vec3::unit_x(),
                ),
                &cone,
            ),
            (
                "a line on a sphere",
                "a sphere holds no line",
                Curve3::Line {
                    origin: Point3::origin(),
                    dir: Vec3::unit_x(),
                },
                &sphere,
            ),
            (
                "a polar circle centred off the sphere's axis",
                "the sphere's radius",
                circle(
                    Point3::new(0.1, 0.0, 0.5),
                    Vec3::unit_z(),
                    0.5,
                    Vec3::unit_x(),
                ),
                &sphere,
            ),
            (
                "an ellipse on a torus",
                "a torus holds no line",
                Curve3::Ellipse {
                    center: Point3::origin(),
                    axis: Vec3::unit_z(),
                    semi_u: 2.5,
                    semi_v: 2.0,
                    u_ref: Vec3::unit_x(),
                },
                &torus,
            ),
        ];
        for (what, site, carrier, surface) in off {
            let err = chart_pcurve(&carrier, surface, band()).unwrap_err();
            assert!(
                matches!(err, PcurveCertifyError::CarrierOffChart { why, .. } if why.contains(site)),
                "{what}: expected the site saying {site:?}, got {err:?}"
            );
        }

        // A Villarceau circle: the bitangent plane through the centre,
        // tilted by asin(r/R), cuts the torus in two circles of radius
        // R centred ±r off the axis. It lies on the torus — measured
        // here — and is neither a parallel nor a meridian.
        let tilt = (minor / major).asin();
        let villarceau = circle(
            Point3::new(minor, 0.0, 0.0),
            Vec3::new(0.0, -tilt.sin(), tilt.cos()),
            major,
            Vec3::unit_x(),
        );
        for k in 0..16 {
            let p = villarceau.eval(f64::from(k) * TAU / 16.0);
            let rho = p.x.hypot(p.y);
            let implicit = (rho - major).powi(2) + p.z.powi(2) - minor * minor;
            assert!(implicit.abs() < 1e-12, "on the torus: {implicit:e}");
        }
        let nurbs = Curve3::Nurbs(Arc::new(
            NurbsCurve3::new(
                KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
                vec![Point3::origin(), Point3::new(1.0, 0.0, 0.0)],
                vec![1.0, 1.0],
            )
            .unwrap(),
        ));
        let plane = Surface::Plane {
            origin: Point3::origin(),
            normal: Vec3::unit_z(),
            u_ref: Vec3::unit_x(),
        };
        let got = chart_pcurve(&villarceau, &torus, band());
        assert!(
            matches!(got, Ok(Pcurve::FocalSection(_))),
            "a Villarceau circle images as a focal section: {got:?}"
        );
        let got = chart_pcurve(&nurbs, &plane, band());
        assert!(
            matches!(got, Ok(Pcurve::Projected(_))),
            "a spline on a plane images as its mapped net: {got:?}"
        );
        // Near the tube's top an off-axis centre `δ` leaves the torus
        // only by `δ²/2r`, so a circle ⊥ the axis centred off it — past
        // the centring band — is not shown off the chart, and no closed
        // form images it. Its general lane is not written yet on a torus
        // (D3: lever-only "not yet").
        let delta = 2.0 * Tol::witness().k() * Tol::witness().eps();
        let crest = circle(
            Point3::new(delta, 0.0, minor),
            Vec3::unit_z(),
            major,
            Vec3::unit_x(),
        );
        let got = chart_pcurve(&crest, &torus, band());
        assert!(
            matches!(
                got,
                Err(PcurveCertifyError::NoGeneralLane {
                    gap: LaneGap::Unwritten,
                    ..
                })
            ),
            "a crest parallel centred off the axis: {got:?}"
        );
    }

    /// Each split arm names its condition, the pair it was raised on,
    /// and a recourse — a floor on the vocabulary, as the sibling
    /// `every_*_arm_names_a_recourse` rows are. An incidence refusal's
    /// recourse is its ending, the read's at the door: none in its payload.
    #[test]
    fn every_carrier_refusal_arm_names_its_pair_and_a_recourse() {
        use geom::{CurveKind, SurfaceKind};
        let rows: Vec<(PcurveCertifyError, Vec<&str>)> = vec![
            (
                PcurveCertifyError::CarrierOffChart {
                    chart: SurfaceKind::Sphere,
                    carrier: CurveKind::Line,
                    why: "WHY",
                    margin: None,
                },
                vec!["sphere", "line", "WHY"],
            ),
            (
                PcurveCertifyError::CarrierOffChart {
                    chart: SurfaceKind::Cone,
                    carrier: CurveKind::Circle,
                    why: "WHY",
                    margin: Some(OffMargin::Past(geom_core::MarginDiag::value(4e-10))),
                },
                vec![
                    "cone",
                    "circle",
                    "WHY",
                    "(margin 4e-10 m)",
                    "not on its face",
                ],
            ),
            (
                PcurveCertifyError::CarrierOffChart {
                    chart: SurfaceKind::Cone,
                    carrier: CurveKind::Line,
                    why: "WHY",
                    margin: Some(OffMargin::InBand(geom_core::MarginDiag::value(4e-10))),
                },
                vec!["cone", "line", "WHY", "(margin 4e-10 m, within the band)"],
            ),
            (
                PcurveCertifyError::ImageMismatch {
                    image: PcurveKind::Spiric,
                    why: "WHY",
                },
                vec!["Spiric", "WHY"],
            ),
        ];
        for (err, payload) in rows {
            let msg = err.to_string();
            for word in payload {
                assert!(msg.contains(word), "{word} missing from: {msg}");
            }
            assert!(
                test_utils::refusal::recourse_markers(&msg) >= 1,
                "no recourse in: {msg}"
            );
            let Some(read) = IncidenceRead::of(&err) else {
                assert_eq!(err.ending(Reading::AtRest), None, "{msg}");
                continue;
            };
            // An incidence refusal's payload carries no recourse: its
            // ending is the door's, through the one map.
            let text = core::fmt::from_fn(|f| err.write_payload(f)).to_string();
            assert!(!text.contains("Recourse"), "{text}");
            assert_eq!(
                err.ending(Reading::Build),
                Some(read.ending(Reading::Build))
            );
            assert!(!msg.contains("repair the body"), "{msg}");
        }
    }

    /// **An incidence refusal ends by the door that reads it, and a read
    /// in band in its lever** (D3, D4 ¶1 (iv)), through the one map: a
    /// definite miss (with no margin, or past a pose) is the kernel's
    /// defect at a build, the stored culprit at rest and the damaged file
    /// at import, with no ε_in offer; a read at a pose or undecided is
    /// the geometry's lever at every door, never a defect, unless its
    /// margin was poisoned: an unreadable margin reads nothing about the
    /// pose and ends in the defect ending, as every unsized decision's
    /// does. `render` ends each the same way. A kernel build's miss read
    /// as a lever, a read in band read as a defect, or a poisoned read
    /// offered a lever, reds a row here.
    #[test]
    fn an_incidence_refusal_ends_by_the_door_and_in_band_by_its_lever() {
        use geom::{CurveKind, SurfaceKind};
        let off = |margin| PcurveCertifyError::CarrierOffChart {
            chart: SurfaceKind::Torus,
            carrier: CurveKind::Line,
            why: "WHY",
            margin,
        };
        let m = geom_core::MarginDiag::value(4e-9);
        let poisoned = geom_core::MarginDiag::INVALID;
        let undecided = |margin| PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartIncidence,
            sample: 2,
            cause: Indeterminate {
                margin,
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("pcurve_torus_chart_incident"),
                terminal_sliver: false,
            },
        };
        let file = geom_core::FileCoincidence::new(1e-6);
        // `definite`: ends in the door's defect ending.
        for (err, definite) in [
            (off(None), true),
            (off(Some(OffMargin::Past(m))), true),
            (off(Some(OffMargin::InBand(m))), false),
            (undecided(geom_core::MarginDiag::value(5e-9)), false),
            (off(Some(OffMargin::InBand(poisoned))), true),
            (undecided(poisoned), true),
        ] {
            let read = IncidenceRead::of(&err).expect("an incidence refusal");
            for (door, culprit) in [
                (
                    ReadAt::from(Reading::Build),
                    geom_core::KERNEL_DEFECT_ENDING,
                ),
                (
                    ReadAt::from(Reading::AtRest),
                    geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
                ),
                (ReadAt::from(file), geom_core::DAMAGED_FILE_ENDING),
            ] {
                let ending = read.ending(door);
                assert_eq!(
                    err.ending(door).as_deref(),
                    Some(ending.as_str()),
                    "{err:?}"
                );
                assert!(err.render(door).ends_with(&ending), "{err:?} at {door:?}");
                assert!(!ending.contains("ε_in"), "{err:?} at {door:?}: {ending}");
                if definite {
                    assert_eq!(ending, culprit, "{err:?} at {door:?}");
                } else {
                    assert!(
                        ending.starts_with("Recourse:") && !ending.contains("defect"),
                        "{err:?} at {door:?}: {ending}"
                    );
                }
            }
        }
    }

    /// **No public refusal of the incidence check panics for want of its
    /// read** (review A's executed case on E1d): a pub-constructible
    /// `ResidualExceeded` of [`PcurveCheck::ChartIncidence`], here and as
    /// the edge certificate's [`crate::CertCheck::ChartIncidence`], is a
    /// definite miss ending by the door; and the check alone ends each
    /// refused arm through the one map ([`IncidenceRead::arm_ending`]).
    #[test]
    fn no_incidence_refusal_panics_for_want_of_its_read() {
        use crate::recourse::Classified;
        use geom_core::{KERNEL_DEFECT_ENDING, KERNEL_OR_FILE_DEFECT_ENDING, MarginDiag};
        let pcurve = PcurveCertifyError::ResidualExceeded {
            check: PcurveCheck::ChartIncidence,
            sample: 3,
        };
        assert_eq!(
            pcurve.ending(Reading::Build).as_deref(),
            Some(KERNEL_DEFECT_ENDING)
        );
        assert!(
            pcurve
                .render(Reading::AtRest)
                .ends_with(KERNEL_OR_FILE_DEFECT_ENDING)
        );
        let edge = crate::CertifyError::ResidualExceeded {
            check: crate::CertCheck::ChartIncidence,
            sample: 3,
            margin: MarginDiag::value(4e-6),
        };
        assert_eq!(
            edge.ending(Reading::Build).as_deref(),
            Some(KERNEL_DEFECT_ENDING)
        );
        assert_eq!(
            edge.ending(Reading::AtRest).as_deref(),
            Some(KERNEL_OR_FILE_DEFECT_ENDING)
        );
        let band = Band::new(1e-9, 1e-8).unwrap();
        let cause = Indeterminate {
            margin: MarginDiag::value(5e-9),
            band,
            predicate: Some("pcurve_cone_chart_incident"),
            terminal_sliver: false,
        };
        let m = MarginDiag::value(4e-10);
        for (arm, want) in [
            (
                RefusedArm::SignCertain(None),
                IncidenceRead::Off(None).ending(Reading::Build),
            ),
            (
                RefusedArm::Zero(Classified { margin: m, band }),
                IncidenceRead::AtPose(m).ending(Reading::Build),
            ),
            (
                RefusedArm::Undecided(&cause),
                IncidenceRead::Undecided(&cause).ending(Reading::Build),
            ),
            (
                RefusedArm::Straddle,
                crate::recourse::INCIDENCE_UNSHOWN.recourse(RefusedArm::Straddle, Reading::Build),
            ),
        ] {
            assert_eq!(
                PcurveCheck::ChartIncidence.recourse(arm, Reading::Build),
                want,
                "{arm:?}"
            );
            assert_eq!(
                crate::certify::recourse(crate::CertCheck::ChartIncidence, arm, Reading::Build),
                want,
                "{arm:?}"
            );
        }
    }

    /// A rim's image over more than one period is still its carrier's
    /// image: its windings are structure, one per carrier period, so the
    /// row certifies, and the edge that long is the carrier's own
    /// winding gate's to refuse ([`crate::certify::CertifyError::WindingExceeded`]).
    #[test]
    fn azimuth_beyond_one_period_is_its_carriers_image() {
        let r = 0.5;
        let cyl = cylinder(r);
        let carrier = Curve3::Circle {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let p = chart_pcurve(&carrier, &cyl, band()).unwrap();
        let out = PcurveCache::certify(p, 0.0, TAU + 0.5, &carrier, &cyl, band());
        assert!(out.is_ok(), "{out:?}");
    }

    /// The cone's azimuth lever DOMINATES the local one everywhere the
    /// row lives. `azimuth_lever` at a single `v` is the parallel
    /// radius there (`|v|·sin alpha`); the arm the checks take is that
    /// lever at the `|v|` supremum of the pcurve's box, so it is an
    /// upper bound at every `v` in it — the direction that cannot
    /// under-state a winding or a headroom. Both signs of `v` (both
    /// nappes) are swept.
    #[test]
    fn the_cone_azimuth_lever_dominates_the_local_lever_over_the_box() {
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
        let (arm, v_arm) = chart_sup_arms_at(&cone, &boxed).unwrap();
        let (arm, v_arm) = (arm.get(), v_arm.get());
        assert!(
            (v_arm - 1.0).abs() < 1e-15,
            "the cone's v IS a slant length"
        );
        for i in 0..=64 {
            let t = f64::from(i) / 64.0;
            let v = boxed.v_min + (boxed.v_max - boxed.v_min) * t;
            let local = azimuth_lever(&cone, v.abs());
            assert!(
                local <= arm + 1e-15,
                "the arm {arm:e} must dominate the local lever {local:e} at v = {v:e}"
            );
        }
        // And it is the lever AT the supremum, not something larger:
        // an arm bigger than the geometry demands would refuse honest
        // work. |v| tops out at 3 (the box's lower edge).
        assert!((arm - azimuth_lever(&cone, 3.0)).abs() < 1e-15);
    }

    /// A carrier whose meter collapses refuses AT THE METER — no
    /// forward verdict is fabricated from a rate that cannot convert a
    /// span to metres, and the refusal of a poison rate is `Invalid`,
    /// distinct from the backwards-span verdict the metered check below
    /// it names.
    #[test]
    fn a_collapsed_carrier_meter_refuses_rather_than_metering_a_span() {
        let knots = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let point = Point3::new(1.0, 2.0, 3.0);
        let net = NurbsCurve3::new(knots, vec![point, point], vec![1.0, 1.0]).unwrap();
        let carrier = Curve3::Nurbs(Arc::new(net));
        assert!(
            param_rate(&carrier).get().is_nan(),
            "a net that never moves states no speed bound at all"
        );
        let cause = param_rate_gate(&carrier, band())
            .expect_err("a poison meter cannot license a metered span");
        assert!(cause.margin.is_invalid());
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
        let rate = param_rate_gate(&Curve3::Nurbs(Arc::new(ok)), band())
            .unwrap()
            .get();
        assert!((rate - 1.0).abs() < 1e-15, "the unit-chord net meters at 1");
    }

    /// A schedule check's escalation renders its payload and then its
    /// check's own labelled ending, read at a build: the azimuth headroom
    /// ends as the edge winding gate does.
    #[test]
    fn a_schedule_escalation_renders_its_checks_own_ending() {
        let escalated = PcurveCertifyError::Escalated {
            check: PcurveCheck::AzimuthPeriod,
            sample: 3,
            cause: Indeterminate {
                margin: geom_core::MarginDiag::value(5e-9),
                band: Band::new(1e-9, 1e-8).unwrap(),
                predicate: Some("pcurve_azimuth_period"),
                terminal_sliver: false,
            },
        };
        assert_eq!(
            escalated.to_string(),
            "pcurve certification: AzimuthPeriod at sample 3 escalated: margin 5e-9 lies inside \
             the ambiguity band (1e-9, 1e-8). Recourse: move the geometry so this arc stays \
             clearly short of a full turn, or, if this arc is intended, tighten the tolerance \
             below 5e-10 m"
        );
    }

    /// A hull-limb escalation, the one escalation the fitted lane
    /// raises, ends by its limb's own decision at the pcurve door as at
    /// the SSI door, at rest, poisoned margin or not.
    #[test]
    fn a_hull_escalation_ends_the_same_at_the_ssi_and_pcurve_doors() {
        let cause = Indeterminate {
            margin: geom_core::MarginDiag::value(5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("ssi_hull_sup_chart"),
            terminal_sliver: false,
        };
        for margin in [cause.margin, geom_core::MarginDiag::INVALID] {
            let ssi = crate::ssi::SsiError::CertificateEscalated {
                limb: SsiLimb::HullSupChart,
                cause: Indeterminate { margin, ..cause },
            };
            let want = ssi.ending(Reading::AtRest);
            let pcurve = ssi_refusal(ssi);
            assert!(
                matches!(
                    pcurve,
                    PcurveCertifyError::FittedEscalated {
                        decision: SsiDecision::Limb(SsiLimb::HullSupChart),
                        ..
                    }
                ),
                "{pcurve:?}"
            );
            assert_eq!(
                pcurve.ending(Reading::AtRest).as_deref(),
                Some(want.as_str()),
                "the pcurve door ends as the SSI door: {pcurve}"
            );
        }
    }

    // ---- The seam class's LINE-carrier limb (#388). ----

    /// The limb's wall: a rational quadratic quarter arc in `u`
    /// (weights 1, 1, √2/2 by column pair), ruled straight one unit up
    /// in `v` — so the `u = 0` column is a straight degree-1
    /// unit-weight ruling from `(1,0,0)` to `(1,0,1)`, the shape a
    /// promoted degree-1 STEP slit/seam carrier claims (dm1's class).
    /// `weights` overrides the six weights row-major in `u`.
    fn ruled_wall_with(weights: [f64; 6]) -> Surface<f64> {
        let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let control = vec![
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 1.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 1.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(0.0, 1.0, 1.0),
        ];
        Surface::Nurbs(Arc::new(
            geom::NurbsSurface::new(ku, kv, control, weights.to_vec()).unwrap(),
        ))
    }

    fn ruled_wall() -> Surface<f64> {
        let w = core::f64::consts::FRAC_1_SQRT_2;
        ruled_wall_with([1.0, 1.0, w, w, 1.0, 1.0])
    }

    /// A `Curve3::Line` from `origin` along `dir` (already unit here).
    fn ruling_line(origin: Point3<f64>, dir: Vec3<f64>) -> Curve3<f64> {
        Curve3::Line { origin, dir }
    }

    /// The positive row: a promoted LINE carrier on the wall's `u = 0`
    /// column certifies through the seam class, and the envelope is the
    /// Greville hull's — essentially zero for the exact ruling.
    #[test]
    fn a_line_ruling_on_its_column_certifies_through_the_seam_class() {
        let cache = PcurveCache::certify(
            Pcurve::IsoLine {
                p0: Point2::new(0.0, 0.0),
                pl: Vec2::new(0.0, 1.0),
            },
            0.0,
            1.0,
            &ruling_line(Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            &ruled_wall(),
            band(),
        )
        .expect("the exact ruling certifies");
        let cert = cache.certificate();
        assert!(
            cert.max_residual < 1e-14,
            "sampled: {:e}",
            cert.max_residual
        );
        assert!(cert.envelope < 1e-14, "envelope: {:e}", cert.envelope);
    }

    /// The REVERSED row (`#389`'s shape): the same column traversed
    /// against the chart's increasing `v` — negative image slope —
    /// certifies identically; direction is data, not an assumption.
    #[test]
    fn the_reversed_column_certifies_too() {
        let cache = PcurveCache::certify(
            Pcurve::IsoLine {
                p0: Point2::new(0.0, 1.0),
                pl: Vec2::new(0.0, -1.0),
            },
            0.0,
            1.0,
            &ruling_line(Point3::new(1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, -1.0)),
            &ruled_wall(),
            band(),
        )
        .expect("the reversed ruling certifies");
        assert!(cache.certificate().envelope < 1e-14);
    }

    /// The SIDE mutant is red: the same line, with an image claiming
    /// the chart's OTHER column (`u = 1`), refuses — that column is a
    /// different ruling a quarter turn away, and the residual schedule
    /// meters the gap in metres.
    #[test]
    fn the_wrong_column_side_refuses() {
        let got = PcurveCache::certify(
            Pcurve::IsoLine {
                p0: Point2::new(1.0, 0.0),
                pl: Vec2::new(0.0, 1.0),
            },
            0.0,
            1.0,
            &ruling_line(Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            &ruled_wall(),
            band(),
        );
        assert!(got.is_err(), "the other column is not this line: {got:?}");
    }

    /// The hull is load-bearing, pinned from both sides: a line
    /// displaced off the column INSIDE the band certifies with an
    /// envelope at least the displacement (deleting or weakening the
    /// Greville hull would report ~0 here), and a displacement beyond
    /// the band refuses. Both displacements SCALE with the ambient
    /// band — "in-band" is a band-relative fact, and a fixed metre
    /// value flips the row's meaning across the ε matrix (3e-10 is
    /// in-band at 1e-9 and three decades OUT at 1e-12).
    #[test]
    fn the_hull_meters_an_off_column_line_from_both_sides() {
        let displaced =
            |d: f64| ruling_line(Point3::new(1.0 + d, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0));
        let image = || Pcurve::IsoLine {
            p0: Point2::new(0.0, 0.0),
            pl: Vec2::new(0.0, 1.0),
        };
        let d = 0.3 * Tol::witness().get().eps;
        let cache = PcurveCache::certify(image(), 0.0, 1.0, &displaced(d), &ruled_wall(), band())
            .expect("an in-band displacement certifies");
        let envelope = cache.certificate().envelope;
        assert!(
            envelope >= d * 0.99,
            "the hull must see the displacement: {envelope:e}"
        );
        assert!(
            PcurveCache::certify(
                image(),
                0.0,
                1.0,
                &displaced(1.0e3 * Tol::witness().get().eps),
                &ruled_wall(),
                band(),
            )
            .is_err(),
            "a displacement three decades past the band must refuse"
        );
    }

    /// The rational gate asks constancy, typed: a column whose two
    /// weights are one constant `2` is the polynomial ruling and
    /// certifies with
    /// the unit column's envelope, to the bit; a column whose weights
    /// differ by one ulp is outside the Greville hull's linear-precision
    /// hypothesis and refuses naming that. The ulp keeps the column's
    /// locus and parameterization the exact ruling to the schedule, so
    /// the weight gate is the deciding check, not a residual.
    #[test]
    fn a_constant_weight_column_certifies_and_a_rational_one_refuses_typed() {
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let certify = |weights: [f64; 6]| {
            PcurveCache::certify(
                Pcurve::IsoLine {
                    p0: Point2::new(0.0, 0.0),
                    pl: Vec2::new(0.0, 1.0),
                },
                0.0,
                1.0,
                &ruling_line(Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
                &ruled_wall_with(weights),
                band(),
            )
        };
        let unit = certify([1.0, 1.0, w, w, 1.0, 1.0]).expect("the unit column certifies");
        let twos = certify([2.0, 2.0, w, w, 1.0, 1.0]).expect("a constant column certifies");
        assert_eq!(
            twos.certificate().envelope.to_bits(),
            unit.certificate().envelope.to_bits(),
            "the constant column's envelope is the unit column's"
        );
        match certify([1.0, 1.0f64.next_up(), w, w, 1.0, 1.0]) {
            Err(PcurveCertifyError::IsoUnsupported { what }) => {
                assert!(what.contains("RATIONAL chart column"), "{what}");
            }
            other => panic!("a rational column must refuse typed: {other:?}"),
        }
    }

    /// The domain gate holds: an image whose moving channel leaves the
    /// column's knot domain refuses by name — the hull bound holds on
    /// the domain only, even though the straight ruling EXTRAPOLATES
    /// exactly (the pointwise schedule alone would pass this).
    #[test]
    fn an_image_leaving_the_column_domain_refuses() {
        let got = PcurveCache::certify(
            Pcurve::IsoLine {
                p0: Point2::new(0.0, 0.0),
                pl: Vec2::new(0.0, 1.0),
            },
            0.0,
            1.5,
            &ruling_line(Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            &ruled_wall(),
            band(),
        );
        match got {
            Err(PcurveCertifyError::IsoUnsupported { what }) => {
                assert!(
                    what.contains("leaves the chart's parameter domain"),
                    "{what}"
                );
            }
            other => panic!("the domain gate must refuse by name: {other:?}"),
        }
    }

    /// **The NURBS weights-in-`T` pin, through the carrier**
    /// (`work/intent/nurbs-weights-and-knots-are-the-scalar.md`): a
    /// rational arc built in `T` over a variable sweep `θ`
    /// (`geom::test_support::held_arc`), held by a `NurbsCurve2` at
    /// `Sym`, gives the arc-rim class' margin `w − cos(h/2)` over the
    /// same `θ` as a symbolic Zero — a theorem, not a reading. The same
    /// carrier with its weights held as `f64` constants (the stored
    /// weight before this item) decides the margin numerically: Zero
    /// at the witness, but no theorem.
    #[test]
    fn the_arc_rim_weight_margin_over_a_held_arc_is_a_symbolic_zero() {
        use core::num::NonZeroUsize;
        use geom::test_support::held_arc::HeldArc;
        use geom_core::Point3;
        use geom_core::sym::with_session;
        use geom_core::{ParamSymbol, Sym, SymBudget};
        type S = Sym<f64>;
        let budget = SymBudget {
            max_terms: 4096,
            max_degree: 128,
        };
        let spans = 3;
        let (theorems, _) = with_session(budget, || {
            let theta = S::param(ParamSymbol::new(0x7e7a), 2.1);
            let arc =
                HeldArc::new(S::from_f64(1.5), theta, NonZeroUsize::new(spans).unwrap()).unwrap();
            // The column a chart holds for the arc, in its plane.
            let control: Vec<Point3<S>> = arc
                .control
                .iter()
                .map(|p| Point3::new(p.x, p.y, S::zero()))
                .collect();
            let held =
                NurbsCurve3::new(arc.knots.clone(), control.clone(), arc.weights.clone()).unwrap();
            let frozen = NurbsCurve3::new(
                arc.knots.clone(),
                control,
                held.weights_f64_interim()
                    .iter()
                    .map(|w| S::from_f64(*w))
                    .collect(),
            )
            .unwrap();
            let theorem = |c: &NurbsCurve3<S>| {
                let margin = arc_rim_weight(c, theta, spans).margin;
                let d = geom_core::k_stats::decide_reported(
                    "pcurve_iso_boundary",
                    Margin::metered(margin, InfSpeed::new(S::from_f64(1.5))),
                    band(),
                )
                .unwrap();
                (d.sign, d.margin.proven())
            };
            (theorem(&held), theorem(&frozen))
        });
        assert_eq!(theorems.0, (Sign::Zero, true), "held");
        assert_eq!(theorems.1.0, Sign::Zero, "frozen: Zero at the witness");
        assert!(!theorems.1.1, "frozen: no proof over a frozen weight");
    }
}

/// **A cone has no surface-level azimuth sup, and the door says so.**
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::float_cmp)]
mod cone_azimuth_sup {
    use super::{NoChartSup, SupSpeed, chart_stretch_sup, chart_stretch_sup_v};
    use geom::Surface;
    use geom_core::{Point3, Vec3};

    /// A cone whose azimuth stretch beats any unit arm well inside the
    /// geometry a model would carry: `α = π/6`, so `|S_u| = v/2`, and
    /// at `v = 4` m down the ruling that is `2` m per radian.
    fn cone() -> (Surface<f64>, f64) {
        let half_angle = core::f64::consts::FRAC_PI_6;
        (
            Surface::Cone {
                apex: Point3::new(0.0, 0.0, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                half_angle,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            half_angle,
        )
    }

    /// The refusal, and the witness that makes it the only honest
    /// answer: two chart points `Δu = 0.5` apart at `v = 4` are
    /// `0.989…` m apart in the model, which a rate of `1` meters as
    /// `0.5` m — an UNDER-statement, the direction a
    /// [`SupSpeed`](geom_core::SupSpeed) exists to forbid, and the
    /// direction that admits an escape the model can see.
    ///
    /// Before this door refused, it answered `SupSpeed(1)` for exactly
    /// this surface: `chart_stretch_sup(cone)` = `(1, 1)`, against a
    /// true `|S_u|` of `2` at that `v`. That is the number this row
    /// exists to make unreachable.
    #[test]
    fn the_door_refuses_a_cone_rather_than_minting_a_false_sup() {
        let (cone, alpha) = cone();
        assert!(
            matches!(
                chart_stretch_sup(&cone),
                Err(NoChartSup::ConeAzimuthGrowsWithV)
            ),
            "the door must refuse rather than answer a number"
        );
        let (v, du) = (4.0_f64, 0.5_f64);
        let chord = cone.eval(0.0, v).distance(cone.eval(du, v));
        assert!(
            chord > SupSpeed::new(1.0).to_meters(du),
            "the unit arm meters {du} rad at v={v} as {} m, against a true \
             chord of {chord} m and a true |S_u| of {}",
            SupSpeed::new(1.0).to_meters(du),
            v * alpha.sin()
        );
    }

    /// The `v` channel is the half that IS true, and it stays minted:
    /// a cone's `v` is a slant length, so `|S_v| = 1` exactly and a
    /// `Δv` step moves the point `Δv` metres.
    #[test]
    fn a_cones_second_channel_is_exactly_unit_and_stays_minted() {
        let (cone, _) = cone();
        let rate = chart_stretch_sup_v(&cone).unwrap();
        assert_eq!(rate.get(), 1.0);
        let (v, dv) = (4.0_f64, 0.25_f64);
        let chord = cone.eval(0.0, v).distance(cone.eval(0.0, v + dv));
        assert!(
            (chord - rate.to_meters(dv)).abs() <= 4.0 * f64::EPSILON * dv,
            "a slant step of {dv} moves {chord} m, metred as {} m",
            rate.to_meters(dv)
        );
    }
}

/// **The placeholder chart has no sup arms, and both doors say so.**
#[cfg(test)]
mod placeholder_sup {
    use super::{
        ChartWindow, NoChartSup, PcurveCertifyError, chart_stretch_inf, chart_stretch_sup,
        chart_stretch_sup_v, chart_sup_arms_at,
    };
    use geom::{NurbsSurface, Surface};
    use std::sync::Arc;

    /// The placeholder refuses on both sup doors and on the boxed one,
    /// where it used to answer unit arms; the inf door's all-zero
    /// answer is the same statement on the other side.
    #[test]
    fn the_placeholder_refuses_every_sup_door() {
        let ph: Surface<f64> = Surface::Nurbs(Arc::new(NurbsSurface::placeholder()));
        assert!(
            matches!(chart_stretch_sup(&ph), Err(NoChartSup::Placeholder)),
            "pair"
        );
        assert!(
            matches!(chart_stretch_sup_v(&ph), Err(NoChartSup::Placeholder)),
            "v only"
        );
        let unit = ChartWindow {
            u_min: 0.0,
            u_max: 1.0,
            v_min: 0.0,
            v_max: 1.0,
        };
        assert_eq!(
            chart_sup_arms_at(&ph, &unit).err(),
            Some(PcurveCertifyError::PlaceholderChart),
            "the boxed door, which the azimuth headroom reads"
        );
        let inf = chart_stretch_inf(&ph);
        assert_eq!(
            (inf.inf_u, inf.inf_v, inf.sup_u, inf.sup_v, inf.area_inf),
            (0.0, 0.0, 0.0, 0.0, 0.0),
            "the inf door certifies nothing for it"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod fitted_lane_tests {
    use std::sync::Arc;

    use geom::{Curve3, NurbsCurve2, NurbsSurface, Surface};
    use geom_core::spline::KnotVector;
    use geom_core::{Band, Point2, Point3, Vec3};

    use super::{PcurveCertifyError, fitted_lane};

    /// The bilinear wall `z = 0` over `[0, 1]²`, its chart the identity.
    fn flat_wall() -> Surface<f64> {
        let k = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let ctl = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ];
        Surface::Nurbs(Arc::new(
            NurbsSurface::new(k.clone(), k, ctl, vec![1.0; 4]).unwrap(),
        ))
    }

    /// `P(t) = (t, 0.5)` over `[0, 1]`.
    fn image() -> NurbsCurve2<f64> {
        NurbsCurve2::new(
            KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(),
            vec![Point2::new(0.0, 0.5), Point2::new(1.0, 0.5)],
            vec![1.0, 1.0],
        )
        .unwrap()
    }

    fn line(lift: f64) -> Curve3<f64> {
        Curve3::Line {
            origin: Point3::new(0.0, 0.5, lift),
            dir: Vec3::new(1.0, 0.0, 0.0),
        }
    }

    /// **A line carrier enters as its exact net, against the chart
    /// alone.** On its image the bound is the composite's rounding; a
    /// carrier lifted off the chart refuses at the hull, and an image
    /// whose domain stops short of the edge refuses before it.
    #[test]
    fn a_line_carrier_is_bounded_against_its_chart_alone() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (wall, image) = (flat_wall(), image());
        let sup = fitted_lane(&line(0.0), &image, &wall, (0.0, 1.0), band)
            .unwrap_or_else(|e| panic!("the line on its image certifies: {e}"));
        assert!(sup <= 1e-12, "the bound is the composite's rounding: {sup}");
        assert!(
            matches!(
                fitted_lane(&line(1e-3), &image, &wall, (0.0, 1.0), band),
                Err(PcurveCertifyError::FittedLimb { .. })
            ),
            "a millimetre off the chart refuses at the hull"
        );
        match fitted_lane(&line(0.0), &image, &wall, (0.0, 1.5), band) {
            Err(PcurveCertifyError::FittedCertificate { what, .. }) => {
                assert!(what.contains("stops short"), "{what}");
            }
            other => panic!("an image short of the edge refuses: {other:?}"),
        }
    }
}

/// **An escape is decided on its positive part, and an infinite arm
/// refuses a contained gap.**
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod escape_tests {
    use super::escape;
    use geom_core::k_stats::decide;
    use geom_core::predicate::{Band, Margin, Sign, SupSpeed};

    fn band() -> Band {
        Band::new(1e-6, 1e-5).unwrap()
    }

    /// A clearance anywhere inside, the 1e-6 row's band included,
    /// decides as contained; an escape keeps its band.
    #[test]
    fn only_the_positive_part_of_a_gap_is_decided() {
        for gap in [-3.0645639348403364e-6, -1.0, 0.0] {
            let m = Margin::metered_sup(escape(gap), SupSpeed::new(1.0));
            assert_eq!(decide("escape", m, band()), Ok(Sign::Zero), "gap {gap}");
        }
        let m = Margin::metered_sup(escape(3e-6), SupSpeed::new(1.0));
        assert!(
            decide("escape", m, band()).is_err(),
            "an in-band escape escalates"
        );
        let m = Margin::metered_sup(escape(1.0), SupSpeed::new(1.0));
        assert_eq!(decide("escape", m, band()), Ok(Sign::Positive));
        assert!(escape(f64::NAN).is_nan(), "poison propagates");
    }

    /// A sup arm can read `+inf` (an overflowing quotient, or a zero
    /// least weight after a refinement rounded one away). Through that
    /// arm a contained gap meters as `0·∞ = NaN` and
    /// refuses (fail-loud); an escape meters as `+inf` and is definite.
    #[test]
    fn a_contained_gap_through_an_infinite_arm_refuses() {
        let arm = SupSpeed::new(f64::INFINITY);
        for gap in [-1.0, 0.0] {
            let m = Margin::metered_sup(escape(gap), arm);
            assert!(m.value().is_nan(), "gap {gap}");
            assert!(decide("escape", m, band()).is_err(), "gap {gap}");
        }
        let m = Margin::metered_sup(escape(1.0), arm);
        assert_eq!(decide("escape", m, band()), Ok(Sign::Positive));
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod iso_family {
    //! **[`chart_iso_family`]'s cylinder arm**: a ruling is `U`, a rim
    //! `V`, an oblique line no family, and a line whose tilt off the
    //! axis is in the band refuses — the mint's tie-break takes the
    //! ruling's form there, which no family may be read off.
    use super::{IsoFamily, IsoFamilyRefusal, chart_iso_family};
    use geom::{Curve3, Surface};
    use geom_core::{Band, Point3, Vec3};

    #[test]
    fn a_cylinder_line_tilted_in_the_band_names_no_family() {
        let cylinder = Surface::Cylinder {
            origin: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let band = Band::new(1e-9, 1e-6).expect("a band");
        let line = |tilt: f64| Curve3::Line {
            origin: Point3::new(1.0, 0.0, 0.0),
            dir: Vec3::new(0.0, tilt, 1.0).normalize(),
        };
        let rim = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.5),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        assert_eq!(
            chart_iso_family(&line(0.0), &cylinder, band),
            Ok(Some(IsoFamily::U))
        );
        assert_eq!(
            chart_iso_family(&rim, &cylinder, band),
            Ok(Some(IsoFamily::V))
        );
        assert_eq!(chart_iso_family(&line(1e-2), &cylinder, band), Ok(None));
        assert!(matches!(
            chart_iso_family(&line(1e-7), &cylinder, band),
            Err(IsoFamilyRefusal::Undecided(_))
        ));
    }

    /// The undecided family's sentence is its question and payload: the
    /// door holding it writes the ending.
    #[test]
    fn an_undecided_family_says_its_payload_and_no_ending() {
        let refusal = IsoFamilyRefusal::Undecided(geom_core::Indeterminate {
            margin: geom_core::MarginDiag::value(5e-8),
            band: Band::new(1e-9, 1e-6).expect("a band"),
            predicate: Some("pcurve_chart_radial_moving"),
            terminal_sliver: false,
        });
        assert_eq!(
            refusal.to_string(),
            "whether the edge runs along a cylinder's ruling or around it is undecided: margin \
             5e-8 lies inside the ambiguity band (1e-9, 1e-6)"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod iso_arc_poison {
    use super::*;
    use geom_core::Dual64;

    /// A poison parameter locates no span, so the iso arc's chart
    /// parameter is poison in every channel it carries — at `Dual` the
    /// derivative channel too, where a bare `from_f64(NaN)` would be a
    /// dual constant whose derivative reads as a zero.
    #[test]
    fn a_poison_parameter_is_poison_in_every_dual_channel() {
        let breaks = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let g = iso_arc_g(
            Dual64::variable(f64::NAN),
            Dual64::constant(0.0),
            Dual64::constant(1.0),
            &breaks,
        );
        assert!(g.value.is_nan() && g.deriv.is_nan(), "{g:?}");
    }
}

/// Check 4's re-derivation of a closed form's class (`derive_harmonic`,
/// the body the mint and check 4 share), where the class reading is in
/// the band.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod off_chart_class_rows {
    use super::{PcurveCertifyError, chart_pcurve};
    use geom::{Curve3, Surface};
    use geom_core::{Band, Interval, Point3, Real, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// The right circular cone of half-angle `alpha` about `+z`, apex at
    /// the origin.
    fn cone(alpha: f64) -> Surface<f64> {
        Surface::Cone {
            apex: Point3::origin(),
            axis: Vec3::unit_z(),
            half_angle: alpha,
            u_ref: Vec3::unit_x(),
        }
    }

    /// **A carrier off the chart never reaches a class reading's "not yet"**
    /// (D3; the PCERT E1 review's P1): a lane choice picks between images of
    /// a carrier ON the chart, so a carrier the chart's incidence test shows
    /// off it refuses as off the chart (`CarrierOffChart`), or escalates the
    /// incidence check where that test is in band, and never ends
    /// `PcurveCheck::ChartClass`. Three carriers whose class reading sits in
    /// the band, each well off its chart, read at `f64` and at `Interval`:
    ///
    /// - a line 1 m off a cone, with its origin at a mid-band height (the
    ///   nappe reading in band), and the same line with its origin moved
    ///   1 m along it, which must end the same way wherever the parameter
    ///   starts;
    /// - a unit circle on a plane tilted mid-band off the axis of a unit
    ///   cylinder, centred 2 m out (the orientation reading in band);
    /// - a spiric met with a torus whose axis is mid-band off perpendicular
    ///   to its own (`pcurve_spiric_chart_axis` in band), an incidence
    ///   reading.
    #[test]
    fn a_carrier_off_its_chart_never_ends_as_a_class_reading() {
        fn ends<T: geom_core::Decide>(carrier: &Curve3<T>, surface: &Surface<T>) -> String {
            match chart_pcurve(carrier, surface, band()) {
                Err(PcurveCertifyError::CarrierOffChart { .. }) => "off".into(),
                Err(PcurveCertifyError::Escalated {
                    check: super::PcurveCheck::ChartIncidence,
                    ..
                }) => "incidence".into(),
                other => format!("{other:?}"),
            }
        }
        let mid = 0.5 * (band().zero() + band().escalate());
        let dir = Vec3::new(0.0, 1.0, 0.3).normalize();
        let line = |origin: Point3<f64>| Curve3::Line { origin, dir };
        let at_mid = Point3::new(1.0, 0.0, mid);
        let (u, v) = (Vec3::unit_y(), Vec3::new(mid, 0.0, 1.0).normalize());
        let tilted_circle = Curve3::Circle {
            center: Point3::new(2.0, 0.0, 0.0),
            axis: u.cross(v).normalize(),
            radius: 1.0,
            u_ref: u,
        };
        let cylinder = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        };
        let (major, minor, offset) = (2.0, 0.75, 0.5);
        let spiric = Curve3::Spiric {
            center: Point3::origin(),
            axis: Vec3::unit_y(),
            u_ref: Vec3::unit_x(),
            major_radius: major,
            minor_radius: minor,
            offset,
        };
        let crossed_torus = Surface::Torus {
            center: Point3::origin(),
            axis: (Vec3::unit_x() + Vec3::unit_y() * (mid / major)).normalize(),
            major_radius: major,
            minor_radius: minor,
            u_ref: Vec3::unit_z(),
        };
        let cases: [(&str, Curve3<f64>, Surface<f64>); 4] = [
            (
                "a line off the cone, origin mid-band",
                line(at_mid),
                cone(0.5),
            ),
            (
                "the same line, origin moved along it",
                line(at_mid + dir * 1.0),
                cone(0.5),
            ),
            ("a circle off the cylinder", tilted_circle, cylinder),
            ("a spiric on a crossed torus", spiric, crossed_torus),
        ];
        let mut wrong = Vec::new();
        for (what, carrier, surface) in cases {
            let at_f64 = ends(&carrier, &surface);
            let at_iv = ends(
                &carrier.map_scalar(Interval::from_f64),
                &surface.map_scalar(Interval::from_f64),
            );
            for (lane, got) in [("f64", at_f64), ("Interval", at_iv)] {
                if got != "off" && got != "incidence" {
                    wrong.push(format!("{what} at {lane}: {got}"));
                }
            }
        }
        assert!(wrong.is_empty(), "{wrong:#?}");
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod check_4_class_rows {
    use super::{PcurveCertifyError, PcurveCheck, Placement, derive_harmonic};
    use geom::{Curve3, Surface};
    use geom_core::{Band, Decide, Interval, Point3, Real, Tol, Vec3};

    fn band() -> Band {
        Band::linear(Tol::witness()).unwrap()
    }

    /// **Check 4's re-derivation of an in-band class is undecided** (D3,
    /// the PCERT E1 ruling): a closed form minted at Zero whose class
    /// reading has moved into the band re-derives to neither the closed
    /// form (a pass would decide the class on an in-band margin) nor a
    /// contradiction of it (only a definite one is a defect), but to the
    /// class check's typed escalation, at `f64` and at `Interval`. The
    /// carrier is a sphere parallel tilted on the sphere so its axial
    /// reading sits mid-band.
    #[test]
    fn check_4s_re_derivation_of_an_in_band_class_is_undecided() {
        fn read<T: Decide>(at: impl Fn(f64) -> T) -> Result<(), PcurveCertifyError> {
            let (h, rho) = (0.6, 0.8);
            let mid = 0.5 * (band().zero() + band().escalate());
            let theta = (mid / rho).asin();
            let (s, c) = theta.sin_cos();
            let n = Vec3::new(0.0, -s, c);
            let tilted = Curve3::Circle {
                center: Point3::origin() + n * h,
                axis: n,
                radius: rho,
                u_ref: Vec3::unit_x(),
            }
            .map_scalar(&at);
            let sphere = Surface::Sphere {
                center: Point3::origin(),
                radius: 1.0,
                axis: Vec3::unit_z(),
                u_ref: Vec3::unit_x(),
            }
            .map_scalar(&at);
            derive_harmonic(&tilted, &sphere, Placement::own(&tilted, &sphere), band()).map(|_| ())
        }
        for (lane, got) in [
            ("f64", read(|x: f64| x)),
            ("Interval", read(<Interval as Real>::from_f64)),
        ] {
            assert!(
                matches!(
                    got,
                    Err(PcurveCertifyError::Escalated {
                        check: PcurveCheck::ChartClass,
                        ..
                    })
                ),
                "{lane}: {got:?}"
            );
        }
    }

    /// **A lane choice with no general lane yet ends lever-only "not
    /// yet"** (D3, `PcurveCheck::ChartClass` and `GeneralLane`): its
    /// undecided reading and its definite refusal (`NoGeneralLane`, for an
    /// unwritten pair and an uncovered pose) all end in the
    /// not-yet ending at every reading, never in a defect ending and
    /// never in a tolerance offer. An identity check on a stored image's
    /// copies (`ImageIdentity`) keeps the defect ending.
    #[test]
    fn a_lane_choice_with_no_lane_ends_not_yet_never_a_defect() {
        use crate::recourse::Reading;
        let cause = geom_core::Indeterminate {
            margin: geom_core::MarginDiag::value(5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("pcurve_torus_chart_axial"),
            terminal_sliver: false,
        };
        let undecided = PcurveCertifyError::Escalated {
            check: PcurveCheck::ChartClass,
            sample: 0,
            cause,
        };
        let unwritten = PcurveCertifyError::NoGeneralLane {
            chart: geom::SurfaceKind::Torus,
            carrier: geom::CurveKind::Circle,
            gap: super::LaneGap::Unwritten,
        };
        let lane_undecided = PcurveCertifyError::Escalated {
            check: PcurveCheck::GeneralLane,
            sample: 0,
            cause,
        };
        let uncovered = PcurveCertifyError::NoGeneralLane {
            chart: geom::SurfaceKind::Sphere,
            carrier: geom::CurveKind::Circle,
            gap: super::LaneGap::Pose {
                piece: 0,
                channel: super::projected::SectorChannel::Azimuth,
            },
        };
        for reading in [Reading::Build, Reading::AtRest] {
            for e in [&undecided, &unwritten, &lane_undecided, &uncovered] {
                let ending = e.ending(reading).unwrap();
                assert!(
                    ending.starts_with(geom_core::NOT_YET_ENDING),
                    "{reading:?}: {e:?} ends {ending}"
                );
                for not in ["defect", "tolerance", "Recourse:"] {
                    assert!(!ending.contains(not), "{reading:?}: {ending}");
                }
            }
        }
        let identity = PcurveCertifyError::Escalated {
            check: PcurveCheck::ImageIdentity,
            sample: 0,
            cause,
        };
        let ending = identity.ending(Reading::AtRest).unwrap();
        assert!(
            !ending.starts_with(geom_core::NOT_YET_ENDING),
            "an identity check ends as a defect: {ending}"
        );
    }
}

/// Limb 2's round memo (`limb2_memo`): a hit is the miss's bits, and a
/// changed input misses.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod limb2_memo_rows {
    use super::{PcurveCertifyError, limb2_memo, limb2_offset_sup, span_memo};
    use geom::{Curve3, Surface};
    use geom_core::{Band, Point3, Real, Vec3};

    /// The wall a micron inside the rim: limb 2's first reading does
    /// not clear it, so its rounds, which the memo serves, decide.
    const WALL: f64 = 1.25 - 1e-6;

    fn rim(radius: f64) -> (Curve3<f64>, Surface<f64>) {
        let rim = Curve3::Circle {
            center: Point3::new(0.3, -0.2, 0.7),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let wall = Surface::Cylinder {
            origin: Point3::new(0.3, -0.2, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: WALL,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        (rim, wall)
    }

    /// **A memo hit is the fresh compute, bit for bit, and every input
    /// the key holds misses when it changes.** The same rim against the
    /// same wall read cold (every round a miss), warm (every round a hit)
    /// and cold again after the memo is emptied render identically, `f64`
    /// bits included. Each key input then changed alone, after a warm
    /// read of the base, must miss: the carrier (a ulp wider), the
    /// surface (a ulp wider), the window, the band, the cuts (asked of
    /// the memo directly), and the decoration (an `Interval` radius with
    /// the base's ends that does not certify). And the one sharing the
    /// key allows: `Sym<Interval>` reads `Interval`'s rounds as hits, and
    /// its reading's `f64` fields are `Interval`'s. Red under a key that
    /// leaves out an input the round reads, or a memo that returns a
    /// neighbour's round.
    /// **The span memo serves a repeated read with the fresh parts, and
    /// every input its key holds misses when it changes.** Limb 2's round
    /// read again after its own memo is emptied takes its parts from the
    /// span memo alone, the same bits. Read directly, a twin net's parts
    /// hit on the same input and miss on a wall a ulp wider, another
    /// count of parts per span, another cut, a control a ulp away, and a
    /// control with the same ends that does not certify. Red under a key
    /// that leaves out an input the parts read, or a memo that returns a
    /// neighbour's parts.
    #[test]
    fn the_span_memo_serves_a_repeated_read_and_a_changed_input_misses() {
        use super::twin_span_hulls;
        use crate::pcurve_cache::projected::ProjectedChart;
        use geom::NurbsCurve3;
        use geom_core::Interval;
        use geom_core::spline::{KnotVector, SpanLocate as _};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (c, s) = rim(1.25);
        let round = || format!("{:?}", limb2_offset_sup(&c, (0.2, 2.9), &s, band));
        limb2_memo::clear();
        let cold = round();
        limb2_memo::clear();
        let (h0, m0) = span_memo::counts();
        let warm = round();
        let (h1, m1) = span_memo::counts();
        assert!(
            h1 > h0 && m1 == m0,
            "a repeated round's parts hit: {:?}",
            (h1 - h0, m1 - m0)
        );
        assert_eq!(warm, cold, "a hit is the miss's bits");
        let uncertified = |x: f64| {
            let y = Interval::from_bounds(-1.0, x * x).sqrt().clip_to(x, x);
            assert!(!y.is_certified());
            y
        };
        let net = |x: f64, lift: &dyn Fn(f64) -> Interval| {
            let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            let control = vec![
                Point3::new(1.2, 0.0, 0.1),
                Point3::new(x, 1.3, 0.2),
                Point3::new(-0.1, 1.2, 0.3),
            ];
            NurbsCurve3::new(knots, control, vec![1.0, 1.0, 1.0])
                .unwrap()
                .map_scalar(|v| {
                    if v == x {
                        lift(v)
                    } else {
                        Interval::from_f64(v)
                    }
                })
        };
        let parts = |net: &NurbsCurve3<Interval>, radius: f64, per_span: usize, cuts: &[f64]| {
            let (_, mut wall) = rim(1.25);
            if let Surface::Cylinder {
                radius: ref mut r, ..
            } = wall
            {
                *r = radius;
            }
            format!(
                "{:?}",
                twin_span_hulls(net, &wall, &ProjectedChart::Cylinder, per_span, cuts)
            )
        };
        let base = net(1.3, &Interval::from_f64);
        let first = parts(&base, 1.25, 4, &[0.5]);
        let (h2, m2) = span_memo::counts();
        assert_eq!(
            parts(&base, 1.25, 4, &[0.5]),
            first,
            "a hit is the miss's bits"
        );
        let (h3, m3) = span_memo::counts();
        assert_eq!((h3 - h2, m3 - m2), (1, 0), "the same input hits");
        let misses = |what: &str, read: &dyn Fn() -> String| {
            let (_, before) = span_memo::counts();
            let _ = read();
            let (_, after) = span_memo::counts();
            assert_eq!(after, before + 1, "a changed {what} misses");
        };
        misses("wall", &|| parts(&base, 1.25f64.next_up(), 4, &[0.5]));
        misses("count per span", &|| parts(&base, 1.25, 3, &[0.5]));
        misses("cut", &|| parts(&base, 1.25, 4, &[0.6]));
        misses("control", &|| {
            parts(&net(1.3f64.next_up(), &Interval::from_f64), 1.25, 4, &[0.5])
        });
        misses("decoration", &|| {
            parts(&net(1.3, &uncertified), 1.25, 4, &[0.5])
        });
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn a_hit_is_the_fresh_compute_and_a_changed_input_misses() {
        use geom_core::spline::SpanLocate as _;
        use geom_core::{Interval, Sym};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let read = |radius: f64, wall: f64, window: (f64, f64), band: Band| {
            let (c, mut s) = rim(radius);
            if let Surface::Cylinder { ref mut radius, .. } = s {
                *radius = wall;
            }
            format!("{:?}", limb2_offset_sup(&c, window, &s, band))
        };
        let base = || read(1.25, WALL, (0.2, 2.9), band);
        limb2_memo::clear();
        let (h0, m0) = limb2_memo::counts();
        let cold = base();
        let (h1, m1) = limb2_memo::counts();
        assert!(
            m1 > m0 && h1 == h0,
            "a cold read misses: {:?}",
            (h1 - h0, m1 - m0)
        );
        let warm = base();
        let (h2, m2) = limb2_memo::counts();
        assert!(
            h2 > h1 && m2 == m1,
            "a warm read hits: {:?}",
            (h2 - h1, m2 - m1)
        );
        assert_eq!(warm, cold, "a hit is the miss's bits");
        limb2_memo::clear();
        assert_eq!(base(), cold, "a recompute is the same bits");
        let misses = |what: &str, f: &dyn Fn()| {
            let _ = base();
            let (_, before) = limb2_memo::counts();
            f();
            let (_, after) = limb2_memo::counts();
            assert!(after > before, "a changed {what} misses");
        };
        misses("carrier", &|| {
            let _ = read(1.25f64.next_up(), WALL, (0.2, 2.9), band);
        });
        misses("surface", &|| {
            let _ = read(1.25, WALL.next_up(), (0.2, 2.9), band);
        });
        misses("window", &|| {
            let _ = read(1.25, WALL, (0.2, 2.8), band);
        });
        misses("band", &|| {
            let _ = read(1.25, WALL, (0.2, 2.9), Band::new(1e-10, 1e-9).unwrap());
        });
        let (c, s) = rim(1.25);
        let key = limb2_memo::Key::of(&c, (0.2, 2.9), &s, band);
        let round = || -> super::limb2_memo::Round {
            Err(PcurveCertifyError::UnsupportedChart { chart: s.kind() })
        };
        let _ = limb2_memo::read(&key, &[0.5], round);
        let (_, before) = limb2_memo::counts();
        let _ = limb2_memo::read(&key, &[0.5], round);
        let _ = limb2_memo::read(&key, &[0.6], round);
        let (_, after) = limb2_memo::counts();
        assert_eq!(after, before + 1, "the same cuts hit and changed cuts miss");
        // The decoration: the same ends, certified and not.
        let lift = Interval::from_f64;
        let iv = |radius: Interval| {
            let (c, s) = rim(1.25);
            let c = match c.map_scalar(lift) {
                Curve3::Circle {
                    center,
                    axis,
                    u_ref,
                    ..
                } => Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                },
                _ => unreachable!(),
            };
            format!(
                "{:?}",
                limb2_offset_sup(&c, (lift(0.2), lift(2.9)), &s.map_scalar(lift), band)
            )
        };
        let certified = lift(1.25);
        let uncertified = Interval::from_bounds(-1.0, 1.5625)
            .sqrt()
            .clip_to(1.25, 1.25);
        assert!(!uncertified.is_certified());
        let _ = iv(certified);
        let (_, before) = limb2_memo::counts();
        let _ = iv(uncertified);
        let (_, after) = limb2_memo::counts();
        assert!(after > before, "a changed decoration misses");
        // `Sym<Interval>` shares `Interval`'s rounds, and reads the same.
        let at_iv = iv(certified);
        let (hits, misses_before) = limb2_memo::counts();
        let sym = geom_core::sym::with_session(
            geom_core::SymBudget {
                max_terms: 64,
                max_degree: 8,
            },
            || {
                let lift = |x: f64| <Sym<Interval> as Real>::from_f64(x);
                let (c, s) = rim(1.25);
                let read = limb2_offset_sup(
                    &c.map_scalar(lift),
                    (lift(0.2), lift(2.9)),
                    &s.map_scalar(lift),
                    band,
                )
                .unwrap();
                (
                    read.upper.to_bits(),
                    read.size.width.to_bits(),
                    read.size.rounding.to_bits(),
                )
            },
        )
        .0;
        let (hits_after, misses_after) = limb2_memo::counts();
        assert!(
            hits_after > hits && misses_after == misses_before,
            "Sym reads Interval's rounds"
        );
        let r = limb2_offset_sup(
            &rim(1.25).0.map_scalar(lift),
            (lift(0.2), lift(2.9)),
            &rim(1.25).1.map_scalar(lift),
            band,
        )
        .unwrap();
        assert_eq!(
            sym,
            (
                r.upper.to_bits(),
                r.size.width.to_bits(),
                r.size.rounding.to_bits()
            ),
            "{at_iv}"
        );
    }
}

/// Limb 2's first reading (`limb2_quick`): a span hit is the miss's
/// bits, every input its key holds misses when it changes, the window
/// is not one of them, a span past the window's ends is read within
/// them where the carrier has no net past them, and a reading that does
/// not clear leaves the rounds' verdict as it was.
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod limb2_quick_rows {
    use super::{limb2_memo, limb2_offset_sup, limb2_quick, projected};
    use geom::{Curve3, Surface};
    use geom_core::{Band, Point3, Real, Vec3};

    fn rim(radius: f64, wall: f64) -> (Curve3<f64>, Surface<f64>) {
        (
            Curve3::Circle {
                center: Point3::new(0.3, -0.2, 0.7),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
            Surface::Cylinder {
                origin: Point3::new(0.3, -0.2, 0.0),
                axis: Vec3::new(0.0, 0.0, 1.0),
                radius: wall,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            },
        )
    }

    /// The reading's bits, `f64` fields included.
    fn bits<T: core::fmt::Debug>(
        read: &Result<projected::OffsetRead<T>, super::PcurveCertifyError>,
    ) -> String {
        match read {
            Ok(r) => format!(
                "{:?} {:x} {:x} {:x} {:x}",
                r.offset,
                r.upper.to_bits(),
                r.size.width.to_bits(),
                r.size.rounding.to_bits(),
                r.size.extent.to_bits()
            ),
            Err(e) => format!("{e:?}"),
        }
    }

    /// **A span hit is the fresh compute, and only the window is shared.**
    /// A rim against its wall read cold, warm and cold again after the
    /// memo is emptied reads the same bits, and passes without a round.
    /// A second window over the same rim reads every span as a hit (the
    /// spans are dyadic, so a window is not in their key), while a
    /// carrier or a surface a ulp off, or a radius with the same ends
    /// that does not certify, misses. `Sym<Interval>` reads
    /// `Interval`'s spans and the same `f64` fields. Red under a key
    /// that leaves out an input a span reads, or one that holds the
    /// window.
    #[test]
    fn a_span_hit_is_the_fresh_compute_and_a_window_shares_its_spans() {
        use geom_core::spline::SpanLocate as _;
        use geom_core::{Interval, Sym};
        let band = Band::new(1e-9, 1e-8).unwrap();
        let read = |radius: f64, wall: f64, window: (f64, f64)| {
            let (c, s) = rim(radius, wall);
            bits(&limb2_offset_sup(&c, window, &s, band))
        };
        limb2_quick::clear();
        let rounds_before = limb2_memo::counts();
        let (h0, m0) = limb2_quick::counts();
        let cold = read(1.25, 1.25, (0.2, 2.9));
        let (h1, m1) = limb2_quick::counts();
        assert!(m1 > m0 && h1 == h0, "a cold read computes its spans");
        assert!(cold.starts_with("Bound("), "the rim clears: {cold}");
        assert_eq!(
            limb2_memo::counts(),
            rounds_before,
            "a reading that clears asks no round"
        );
        let warm = read(1.25, 1.25, (0.2, 2.9));
        let (h2, m2) = limb2_quick::counts();
        assert!(h2 > h1 && m2 == m1, "a warm read hits");
        assert_eq!(warm, cold, "a hit is the miss's bits");
        limb2_quick::clear();
        assert_eq!(
            read(1.25, 1.25, (0.2, 2.9)),
            cold,
            "a recompute is the same bits"
        );
        let (_, before) = limb2_quick::counts();
        let _ = read(1.25, 1.25, (0.3, 2.7));
        assert_eq!(
            limb2_quick::counts().1,
            before,
            "another window over the same rim reads the same spans"
        );
        let misses = |what: &str, f: &dyn Fn()| {
            let (_, before) = limb2_quick::counts();
            f();
            assert!(limb2_quick::counts().1 > before, "a changed {what} misses");
        };
        misses("carrier", &|| {
            let _ = read(1.25f64.next_up(), 1.25, (0.2, 2.9));
        });
        misses("surface", &|| {
            let _ = read(1.25, 1.25f64.next_up(), (0.2, 2.9));
        });
        let lift = Interval::from_f64;
        let iv = |radius: Interval| {
            let (c, s) = rim(1.25, 1.25);
            let Curve3::Circle {
                center,
                axis,
                u_ref,
                ..
            } = c.map_scalar(lift)
            else {
                unreachable!()
            };
            let c = Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            };
            bits(&limb2_offset_sup(
                &c,
                (lift(0.2), lift(2.9)),
                &s.map_scalar(lift),
                band,
            ))
        };
        let uncertified = Interval::from_bounds(-1.0, 1.5625)
            .sqrt()
            .clip_to(1.25, 1.25);
        assert!(!uncertified.is_certified());
        let at_iv = iv(lift(1.25));
        misses("decoration", &|| {
            let _ = iv(uncertified);
        });
        let (_, before) = limb2_quick::counts();
        let sym = geom_core::sym::with_session(
            geom_core::SymBudget {
                max_terms: 64,
                max_degree: 8,
            },
            || {
                let lift = |x: f64| <Sym<Interval> as Real>::from_f64(x);
                let (c, s) = rim(1.25, 1.25);
                let r = limb2_offset_sup(
                    &c.map_scalar(lift),
                    (lift(0.2), lift(2.9)),
                    &s.map_scalar(lift),
                    band,
                )
                .unwrap();
                (
                    r.upper.to_bits(),
                    r.size.width.to_bits(),
                    r.size.rounding.to_bits(),
                )
            },
        )
        .0;
        assert_eq!(
            limb2_quick::counts().1,
            before,
            "Sym reads Interval's spans"
        );
        let r = limb2_offset_sup(
            &rim(1.25, 1.25).0.map_scalar(lift),
            (lift(0.2), lift(2.9)),
            &rim(1.25, 1.25).1.map_scalar(lift),
            band,
        )
        .unwrap();
        assert_eq!(
            sym,
            (
                r.upper.to_bits(),
                r.size.width.to_bits(),
                r.size.rounding.to_bits()
            ),
            "{at_iv}"
        );
    }

    /// **A span past the window's ends is read within them where the
    /// carrier has none past them.** A spiric section of a torus whose
    /// oval ends at `cos t = −0.4` (`t ≈ ±1.98`), over `[0.1, 1.9]`: the
    /// quarter-turn span `[π/2, π]` reaches past the oval's end, where
    /// the square root's argument is negative and the span has no net,
    /// so it is read again as `[π/2, 1.9]`. The first reading clears
    /// against both operands, and no round is asked. Red under a first
    /// reading that gives up on a span past the window, or one that
    /// passes on a net it could not certify.
    #[test]
    fn a_span_past_the_window_is_read_within_it_where_the_carrier_ends() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (axis, u_ref) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let center = Point3::new(0.4, -0.3, 0.2);
        let spiric = Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius: 1.0,
            minor_radius: 0.5,
            offset: 0.8,
        };
        let torus = Surface::Torus {
            center,
            axis,
            major_radius: 1.0,
            minor_radius: 0.5,
            u_ref,
        };
        let plane = Surface::Plane {
            origin: center + u_ref * 0.8,
            normal: u_ref,
            u_ref: axis.cross(u_ref),
        };
        assert!(
            crate::carrier_net::carrier_net(&spiric, (0.0, 3.0), &[]).is_err(),
            "the spiric has no net past its oval's end"
        );
        limb2_quick::clear();
        let rounds = limb2_memo::counts();
        for operand in [&torus, &plane] {
            let read = limb2_offset_sup(&spiric, (0.1, 1.9), operand, band).unwrap();
            assert!(
                matches!(read.offset, projected::Offset::Bound(_)) && read.upper <= band.zero(),
                "{:?}: the spiric clears: {:?}",
                operand.kind(),
                read.offset
            );
        }
        assert_eq!(limb2_memo::counts(), rounds, "the first reading decided it");
    }

    /// **A first reading that does not clear leaves the rounds'
    /// verdict, bit for bit.** A rim a micron off its wall, at a
    /// nanometre band: limb 2's reading asks the rounds, is a measured
    /// miss, and is the reading limb 2 gives with the first reading not
    /// taken, `f64` fields included (the rounds' memo emptied between,
    /// so each computes). Red under a first reading that passes or
    /// refuses on its own, or leaves any trace in the reading.
    #[test]
    fn a_reading_that_does_not_clear_is_the_rounds() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (c, s) = rim(1.25 + 1e-6, 1.25);
        limb2_memo::clear();
        let (_, misses) = limb2_memo::counts();
        let read = limb2_offset_sup(&c, (0.2, 2.9), &s, band);
        assert!(limb2_memo::counts().1 > misses, "the rounds are asked");
        assert!(
            matches!(
                read,
                Ok(projected::OffsetRead {
                    offset: projected::Offset::Miss { .. },
                    ..
                })
            ),
            "a micron off is a measured miss: {}",
            bits(&read)
        );
        limb2_memo::clear();
        let alone = limb2_quick::without(|| limb2_offset_sup(&c, (0.2, 2.9), &s, band));
        assert_eq!(bits(&read), bits(&alone), "the rounds' reading");
    }

    /// **A covering has no gap.** Parts over `[lo, hi]` cover it when,
    /// sorted, the first starts at or before `lo`, each next starts at
    /// the bits the one before ends at, and the last ends at or past
    /// `hi`: any order of a partition passes, reaching past either end
    /// passes, and a gap, an overlap, a part short of either end, or no
    /// part at all does not. Red under a covering that reads a gap as
    /// the part before it.
    #[test]
    fn a_covering_has_no_gap() {
        let parts = |r: &[(f64, f64)]| r.iter().map(|&x| limb2_quick::part_over(x)).collect();
        let covers = |r: &[(f64, f64)], d| limb2_quick::covering(parts(r), d).is_some();
        let d = (0.1, 0.9);
        assert!(covers(&[(0.5, 1.0), (0.0, 0.25), (0.25, 0.5)], d));
        assert!(covers(&[(0.1, 0.9)], d));
        assert!(covers(&[(-2.0, 0.5), (0.5, 3.0)], d));
        assert!(!covers(&[(0.0, 0.25), (0.5, 1.0)], d), "a gap");
        assert!(
            !covers(&[(0.0, 0.25), (0.25f64.next_up(), 1.0)], d),
            "a gap of a ulp"
        );
        assert!(!covers(&[(0.0, 0.6), (0.5, 1.0)], d), "an overlap");
        assert!(!covers(&[(0.2, 1.0)], d), "short of the lower end");
        assert!(!covers(&[(0.0, 0.8)], d), "short of the upper end");
        assert!(!covers(&[], d), "no part");
    }

    /// **Every pass bounds the true deviation.** A unit circle about `ẑ`
    /// against the plane `y = 1` it touches at `t = π/2`, where its
    /// deviation is `1 − cos s = 2·sin²(s/2)` at `s = t − π/2`. Over
    /// windows `[π/2 + s₀, π/2 + s₁]`, symmetric and lopsided either way,
    /// some under the band's `1e-9` throughout and some reaching past it
    /// at one end only, every `Bound` limb 2 reads is at least the
    /// window's true greatest deviation, and the symmetric windows under
    /// the band pass in the first reading. Red under a first reading that
    /// passes without reading a piece of the window: keeping one half of
    /// each halving, say, passes the lopsided windows on the half that
    /// clears.
    #[test]
    fn every_pass_bounds_the_true_deviation() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let circle = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 1.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let deviation = |s: f64| 2.0 * (s / 2.0).sin().powi(2);
        let quarter = core::f64::consts::FRAC_PI_2;
        let ends = [1e-5, 2e-5, 3e-5, 6e-5, 1e-4];
        let (mut passes, mut first) = (0, 0);
        for &lo in &ends {
            for &hi in &ends {
                let window = (quarter - lo, quarter + hi);
                let truth = deviation(lo).max(deviation(hi));
                let (_, misses) = limb2_memo::counts();
                let read = limb2_offset_sup(&circle, window, &plane, band).unwrap();
                if let projected::Offset::Bound(b) = read.offset {
                    passes += 1;
                    if limb2_memo::counts().1 == misses {
                        first += 1;
                    }
                    assert!(
                        b >= truth * (1.0 - 1e-9),
                        "[π/2 − {lo}, π/2 + {hi}]: a pass at {} under the true {truth}",
                        b
                    );
                }
                if lo == hi && truth < 0.5e-9 {
                    assert!(
                        matches!(read.offset, projected::Offset::Bound(_)),
                        "[π/2 ± {lo}]: under the band, a pass"
                    );
                }
            }
        }
        assert!(
            first > 0,
            "the first reading passes some window ({passes} passes)"
        );
    }

    /// **No piece of the window goes unread** (dual review of PR 5885,
    /// R1). A circle of radius 0.2 about `ẑ` against the plane
    /// `y = 0.5e-9`, at the band `(1e-9, 1e-8)`, over the window
    /// `[−5e-9, 1e-10]`: the carrier's deviation reaches `1.5e-9` at the
    /// window's lower end, so no bound below the band holds there. The
    /// clipped piece `[−5e-9, 0]` stays whole down to the deepest level,
    /// where it cannot be halved: the first reading ends there rather
    /// than passing on `[0, 1e-10]` alone. At `f64` and at `Interval`
    /// limb 2's reading is the rounds' alone, bit for bit, and a
    /// measured miss. Red under a first reading that drops a span it
    /// cannot halve, or passes on parts with a gap between them.
    #[test]
    fn a_window_piece_no_span_reads_ends_the_first_reading() {
        use geom_core::Interval;
        let band = Band::new(1e-9, 1e-8).unwrap();
        let circle = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 0.2,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let plane = Surface::Plane {
            origin: Point3::new(0.0, 0.5e-9, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let window = (-5e-9, 1e-10);
        let miss = |what: &str, read: &str| {
            assert!(read.starts_with("Miss"), "{what}: a measured miss: {read}");
        };
        limb2_memo::clear();
        let read = bits(&limb2_offset_sup(&circle, window, &plane, band));
        limb2_memo::clear();
        let alone = bits(&limb2_quick::without(|| {
            limb2_offset_sup(&circle, window, &plane, band)
        }));
        assert_eq!(read, alone, "f64: the rounds' reading");
        miss("f64", &read);
        let lift = Interval::from_f64;
        let (c, p) = (circle.map_scalar(lift), plane.map_scalar(lift));
        let w = (lift(window.0), lift(window.1));
        limb2_memo::clear();
        let read = bits(&limb2_offset_sup(&c, w, &p, band));
        limb2_memo::clear();
        let alone = bits(&limb2_quick::without(|| limb2_offset_sup(&c, w, &p, band)));
        assert_eq!(read, alone, "Interval: the rounds' reading");
        miss("Interval", &read);
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod cylinder_rim_rows {
    use super::cylinder_rim_axial;
    use geom_core::interval::Interval;
    use geom_core::predicate::Band;
    use geom_core::{Bounds, Tol};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness tolerance has a band")
    }

    fn iv(lo: f64, hi: f64) -> Interval {
        Interval::from_bounds(lo, hi)
    }

    fn bits(x: Interval) -> (f64, f64) {
        (x.lo(), x.hi())
    }

    /// **A rim's axial terms are zero only where both hold zero and their
    /// sup is decided `Zero`** ([`cylinder_rim_axial`]). Red if the
    /// decision is lax (an amplitude past the band zeroed), if `b`'s
    /// holding-zero gate is dropped (an in-band `b` beside an exact
    /// zero `a` zeroed, which at `f64` moves a pcurve), or if the rim is
    /// not decided (an enclosure of zero kept).
    #[test]
    fn a_rims_axial_terms_are_zeroed_only_where_both_hold_zero_within_the_band() {
        let rounding = iv(-1e-17, 1e-17);
        let zero = iv(0.0, 0.0);
        let (a, b) = cylinder_rim_axial(rounding, zero, band());
        assert_eq!(
            (bits(a), bits(b)),
            ((0.0, 0.0), (0.0, 0.0)),
            "a rim's rounding"
        );
        let wide = iv(-1e-3, 1e-3);
        let (a, _) = cylinder_rim_axial(wide, zero, band());
        assert_eq!(bits(a), bits(wide), "an amplitude past the band is kept");
        let off = iv(1e-17, 2e-17);
        let (_, b) = cylinder_rim_axial(rounding, off, band());
        assert_eq!(bits(b), bits(off), "a term that excludes zero is kept");
        assert_eq!(
            cylinder_rim_axial(0.0_f64, 1e-17, band()),
            (0.0, 1e-17),
            "at f64 a nonzero term is kept, however small"
        );
        assert_eq!(
            cylinder_rim_axial(1e-17_f64, 0.0, band()),
            (1e-17, 0.0),
            "at f64 a nonzero term is kept, however small"
        );
    }
}

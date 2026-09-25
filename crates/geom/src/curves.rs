//! Analytic 3-D curves: the [`Curve3`] closed enum and its evaluators.
//!
//! Curve kinds form a **closed enum** per D3 (`docs/DESIGN.md`):
//! intersection and classification need pairwise dispatch, and a closed
//! enum makes every dispatch site exhaustively checked at compile time.
//! The [`Curve3::Nurbs`] variant is the universal fallback — it carries
//! a validated [`NurbsCurve3`] payload (see [`nurbs`]) and its evaluator
//! arms are real; the "no description yet" state is
//! [`Curve3::nurbs_placeholder`].
//!
//! # Curve conventions (normative; the curve half)
//!
//! The crate docs carry the conventions curves and surfaces share —
//! units, complete loci, the no-range-reduction rule and its
//! bit-identity policy, conventional-and-unchecked frame fields,
//! totality and poison, and the evaluation-code discipline. These are
//! the curve-specific ones:
//!
//! - **What the parameter means, per kind.** A line's parameter is arc
//!   length in meters (unit `dir`); a circle's is the angle in radians;
//!   an ellipse's is the eccentric anomaly.
//! - **Bounds come from the topology, never from the curve.** An edge
//!   bounds its carrier by its **vertices**: the parameter interval is
//!   derived from the vertex positions, not stored (the
//!   representation-consistency lesson of D2 applied to bounds; also why
//!   profile carriers are split so no edge spans a full period).
//! - **The `he_plus` forward contract (D1, ratified at M1):** an edge's
//!   intrinsic direction is its plus half-edge, and the curve geometry
//!   MUST agree — **increasing curve parameter runs from
//!   `start(he_plus)` to `end(he_plus)`**. Per-face traversal senses and
//!   pcurves are *derived* from that one orientation, never stored as
//!   peers.
//! - **The azimuthal frame is one body, and it is not here.** The
//!   circle and ellipse arms read `v_ref = axis × u_ref` and the
//!   radial/tangential pair at angle `u` from the interior
//!   `crate::azimuth` module — the same one home the surface half
//!   reads, so the two halves cannot drift a convention apart.
//! - **Periodicity:** a circle is 2π-periodic in θ: as a locus,
//!   `P(θ) = P(θ + 2πk)` exactly, in the reals. What that does and does
//!   not promise in floating point is the crate docs' bit-identity
//!   paragraph.
//! - **The conventional fields** here are `dir`, `axis` and `u_ref`
//!   (unit; `u_ref ⊥ axis`), unchecked per the crate docs' rule.

pub mod boxes;
pub mod compose;
pub mod fit;
pub mod nurbs;
pub mod projection;

use std::sync::Arc;

pub use compose::{ComposeError, SeamSide, compose_chain};
pub use fit::{FIT_REMOVAL_BUDGET, FitError, FitOutcome, RefitSkip};
use geom_core::spline::SpanLocate;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::azimuth;
pub use nurbs::{CurveWindow2, CurveWindow3, NurbsCurve2, NurbsCurve3};
pub use projection::{Projection2, Projection3, ProjectionInconclusive};

/// An analytic 3-D curve — a **complete locus**. Units, the
/// no-range-reduction rule and its bit-identity policy, and the
/// conventional-and-unchecked field rule are the crate docs'; what the
/// parameter means per kind, where an edge's bounds come from, and the
/// `he_plus` forward contract are this module's.
///
/// Fields are public data (D2: conventions are carried by data);
/// construction is by struct-literal variant syntax.
///
/// **`Clone`, not `Copy` (M5 PR 3, accepted and binding):** the
/// [`Curve3::Nurbs`] payload is an [`Arc`]-shared [`NurbsCurve3`], so
/// the enum is cheap to clone (one refcount) but no longer `Copy`. The
/// payload is immutable after validated construction — sharing is
/// D9-clean (no address-dependent behavior, no interior mutability).
#[derive(Clone, Debug)]
pub enum Curve3<T: Real> {
    /// The infinite straight line `P(t) = origin + dir·t`.
    ///
    /// - `dir` is **unit** (conventional, unchecked), so `t` is arc
    ///   length in meters; domain t ∈ ℝ, not periodic.
    /// - `origin` is the `t = 0` point — conventional data (any point of
    ///   the line would do; the choice fixes the parameterization).
    Line {
        /// The point at parameter `t = 0`.
        origin: Point3<T>,
        /// The unit tangent direction (conventional, unchecked);
        /// increasing `t` runs along it.
        dir: Vec3<T>,
    },

    /// The full circle
    /// `P(θ) = center + (u_ref·cos θ + v_ref·sin θ)·radius`, where
    /// `v_ref = axis × u_ref` (computed, never stored — the frame is
    /// right-handed by construction).
    ///
    /// - `axis` is the unit circle normal, `u_ref` the unit reference
    ///   direction with `u_ref ⊥ axis` (both conventional, unchecked);
    ///   `u_ref` carries the seam — `θ = 0` lives at
    ///   `center + u_ref·radius` (D2: seam placement is conventional
    ///   data).
    /// - θ in radians, domain ℝ, period 2π; increasing θ winds
    ///   **counterclockwise viewed from the tip of `axis`** (right-hand
    ///   rule about `axis`).
    /// - `radius > 0` in meters (conventional; a zero/negative radius is
    ///   degenerate data, rejected upstream by construction/validation,
    ///   evaluated as-is here).
    Circle {
        /// The circle's center.
        center: Point3<T>,
        /// The unit normal of the circle's plane (right-hand winding
        /// rule; conventional, unchecked).
        axis: Vec3<T>,
        /// The radius in meters (positive by convention).
        radius: T,
        /// The unit reference direction ⊥ `axis` where θ = 0 lives —
        /// the seam, carried as conventional data per D2.
        u_ref: Vec3<T>,
    },

    /// The full ellipse
    /// `P(θ) = center + u_ref·(major·cos θ) + v_ref·(minor·sin θ)`,
    /// where `v_ref = axis × u_ref` (computed, never stored — the frame
    /// is right-handed by construction). C1 rung 2 (M5 PR 5): the exact
    /// conic carrier of the common curved-boolean cuts (tilted
    /// plane×cylinder, equal-radius cylinder×cylinder).
    ///
    /// - `axis` is the unit ellipse-plane normal, `u_ref` the unit
    ///   **semi-major** direction with `u_ref ⊥ axis` (both
    ///   conventional, unchecked); `u_ref` carries the seam — `θ = 0`
    ///   lives at `center + u_ref·major`.
    /// - θ in radians, domain ℝ, period 2π; increasing θ winds
    ///   counterclockwise viewed from the tip of `axis` (right-hand
    ///   rule), exactly the circle convention. θ is **not** arc length
    ///   and not the polar angle of the point — it is the conic's
    ///   eccentric anomaly; `|dP/dθ|` varies in `[minor, major]`.
    /// - `major > minor > 0` **strictly** (meters). Equal semi-axes are
    ///   a `Circle` — one kind per configuration (D3's closed-enum
    ///   discipline) — and are refused by [`Curve3::ellipse`], the one
    ///   deciding constructor. Like every conventional invariant the
    ///   ordering is *data* here: evaluators consume the fields as
    ///   given, tier-3 certification owns the invariant at rest, and a
    ///   struct-literal that bypasses the constructor owns the
    ///   consequences (well-defined garbage, not poison).
    Ellipse {
        /// The ellipse's center.
        center: Point3<T>,
        /// The unit normal of the ellipse's plane (right-hand winding
        /// rule; conventional, unchecked).
        axis: Vec3<T>,
        /// The semi-major axis length in meters (`major > minor` by
        /// the constructor's refusal).
        major: T,
        /// The semi-minor axis length in meters (positive by the
        /// constructor's refusal).
        minor: T,
        /// The unit semi-major direction ⊥ `axis` where θ = 0 lives —
        /// the seam, carried as conventional data per D2.
        u_ref: Vec3<T>,
    },

    /// The **spiric of Perseus** — the section of a ring torus by a
    /// plane parallel to its axis — restricted to ONE of its two ovals:
    ///
    /// ```text
    /// P(v) = center + u_ref·offset + m·√((R + r·cos v)² − offset²) + axis·(r·sin v)
    /// m = axis × u_ref,  R = major_radius,  r = minor_radius,  v ∈ ℝ, period 2π
    /// ```
    ///
    /// The torus is `T(R, r)` about `(center, axis)`; the cutting plane
    /// is `{ x : (x − center)·u_ref = offset }`, parallel to the axis at
    /// the SIGNED stand-off `offset`; the carried oval is the one on the
    /// `+m` side of the plane's trace. The parameter `v` is the torus's
    /// own minor angle, exactly as a circle's is its angle and an
    /// ellipse's its eccentric anomaly: `|dP/dv|² = r²cos²v +
    /// r²ρ²sin²v/(ρ² − offset²)`, `ρ = R + r·cos v`, so
    /// `|dP/dv| ∈ [r, r(R − r)/√((R − r)² − offset²)]` — bounded away
    /// from zero, a regular parameter on the oval.
    ///
    /// Conventions (D2: carried as data, unchecked by the evaluators,
    /// decided at the mint):
    /// - `axis`, `u_ref` unit and orthogonal; `R > r > 0` (the ring
    ///   torus) and `|offset| < R − r` (the TWO-oval regime, where
    ///   `ρ² − offset² > 0` for every `v`). Off-regime data yields a
    ///   negative radicand, which is poison by [`Real::sqrt`]'s
    ///   totality policy — never a panic.
    /// - `v = 0` is the seam at the outer-equator point `ρ = R + r`;
    ///   increasing `v` winds counterclockwise viewed from the tip of
    ///   `u_ref` (`dP/dv(0) = axis·r`, `dP/dv(π/2) ∝ −m`, and
    ///   `m × axis = u_ref`).
    /// - The two spellings `(axis, u_ref, offset)` and
    ///   `(−axis, −u_ref, −offset)` describe the SAME oval in opposite
    ///   senses, exactly as a circle's `axis` sign does — the sense is
    ///   frame data, there is no `bool`. Under `u_ref ↦ −u_ref,
    ///   offset ↦ −offset` alone the OTHER oval is named.
    ///
    /// [`Curve3::spiric`] is the one deciding door into this variant:
    /// it decides the ring, the two-oval regime and the frame's
    /// orthogonality through named predicates, exactly as
    /// [`Curve3::ellipse`] decides its axis ordering. A struct literal
    /// that bypasses it owns the consequences (well-defined garbage or
    /// poison, never a panic).
    ///
    /// The locus is a bicircular quartic of genus 1 — no rational
    /// parameterization exists, so no `Nurbs` is its exact locus.
    Spiric {
        /// The torus's centre.
        center: Point3<T>,
        /// The torus's unit axis (conventional, unchecked); the winding
        /// sense of `v` is fixed with `u_ref` as described above.
        axis: Vec3<T>,
        /// The unit normal of the cutting plane, ⊥ `axis` (conventional,
        /// unchecked); the plane stands `offset` along it from `center`.
        u_ref: Vec3<T>,
        /// The torus's major radius `R` in meters.
        major_radius: T,
        /// The torus's minor radius `r` in meters (`R > r > 0`).
        minor_radius: T,
        /// The plane's signed stand-off from the torus centre along
        /// `u_ref`, in meters (`|offset| < R − r` by convention).
        offset: T,
    },

    /// The NURBS fallback (D3: representable from day one; evaluators
    /// implemented at M5 PR 3). The payload is a validated
    /// [`NurbsCurve3`] behind an [`Arc`] (immutable, cheap to clone —
    /// see the enum docs on the `Copy` loss). The "no description yet"
    /// state that the former unit variant carried is now
    /// [`Curve3::nurbs_placeholder`] — a poison-valued payload with the
    /// same all-poison evaluation behavior.
    Nurbs(Arc<NurbsCurve3<T>>),
}

/// Typed refusal of [`Curve3::ellipse`] — the one place that decides an
/// ellipse's axis ordering (spec M5-PR5 §1: one kind per configuration;
/// the constructor is the only decision point).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EllipseInvalid {
    /// The semi-axes coincide (|major − minor| ≤ ε): this configuration
    /// is a `Circle`, and D3's one-kind-per-configuration discipline
    /// refuses to mint it as a degenerate `Ellipse`.
    CircularAxes,
    /// `major` is definitely smaller than `minor`: the caller swapped
    /// the axes (the frame convention is major-first; swap `u_ref` to
    /// the true major direction and reorder).
    AxesSwapped,
    /// The minor semi-axis is not definitely positive (zero or
    /// negative: a degenerate segment, not an ellipse).
    MinorNotPositive,
    /// A constructor predicate landed in the ambiguity band or was
    /// poisoned (`ellipse_axes_distinct` / `ellipse_minor_positive`):
    /// the configuration is too close to the circular (or degenerate)
    /// coincidence to name a kind soundly.
    Escalated(Indeterminate),
}

impl core::fmt::Display for EllipseInvalid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CircularAxes => write!(
                f,
                "ellipse construction: the semi-axes coincide — this configuration is a \
                 Circle, one kind per configuration (D3); construct the Circle carrier, or {}",
                geom_core::COINCIDENCE_RECOURSE
            ),
            Self::AxesSwapped => write!(
                f,
                "ellipse construction: major < minor — the frame convention is major-first \
                 (u_ref is the semi-major direction); swap the axes"
            ),
            Self::MinorNotPositive => write!(
                f,
                "ellipse construction: the minor semi-axis is not positive (a degenerate \
                 segment, not an ellipse)"
            ),
            Self::Escalated(diag) => write!(
                f,
                "ellipse construction escalated: {} — the configuration sits too close to \
                 the circular coincidence to name a kind; construct the Circle carrier, \
                 or {} (D4)",
                diag.payload(),
                geom_core::COINCIDENCE_RECOURSE
            ),
        }
    }
}

impl std::error::Error for EllipseInvalid {}

/// Typed refusal of [`Curve3::spiric`] — the one place that decides a
/// spiric's regime (one kind per configuration, D3; the constructor is
/// the only decision point).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpiricInvalid {
    /// The minor radius is not definitely positive (a degenerate tube,
    /// not a torus).
    MinorNotPositive,
    /// `R ≤ r`: not a ring torus (a horn or spindle torus, whose
    /// axis-parallel sections are not two ovals).
    NotARing,
    /// `|offset| ≥ R − r`: the cutting plane reaches the inner
    /// equator, so the section is a node, one oval with folds, or
    /// empty — not the two-oval regime this kind carries.
    NotTwoOvals,
    /// `axis · u_ref` is definitely nonzero (levered at `R + r`): the
    /// cutting plane is not parallel to the torus axis.
    FrameNotOrthogonal,
    /// A constructor predicate landed in the ambiguity band or was
    /// poisoned (`spiric_minor_positive`, `spiric_ring`,
    /// `spiric_two_ovals`, `spiric_frame_orthogonal`).
    Escalated(Indeterminate),
}

impl core::fmt::Display for SpiricInvalid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MinorNotPositive => write!(
                f,
                "spiric construction: the minor radius is not positive (a degenerate tube, \
                 not a torus)"
            ),
            Self::NotARing => write!(
                f,
                "spiric construction: major ≤ minor — not a ring torus, whose axis-parallel \
                 sections are the two ovals this kind carries"
            ),
            Self::NotTwoOvals => write!(
                f,
                "spiric construction: |offset| ≥ major − minor — the cutting plane reaches \
                 the inner equator, so the section is not two ovals"
            ),
            Self::FrameNotOrthogonal => write!(
                f,
                "spiric construction: axis · u_ref is not zero — the cutting plane is not \
                 parallel to the torus axis"
            ),
            Self::Escalated(diag) => write!(
                f,
                "spiric construction escalated: {} — the configuration sits too close to a \
                 regime boundary to name the kind; {} (D4)",
                diag.payload(),
                geom_core::COINCIDENCE_RECOURSE
            ),
        }
    }
}

impl std::error::Error for SpiricInvalid {}

/// A stored datum of an analytic [`Curve3`] — the FIELD, named apart
/// from the variant that carries it (a circle's and an ellipse's
/// `center` are both [`CurveDatum::Center`]). The curve half of
/// [`crate::SurfaceDatum`]; a consumer naming a datum names the curve
/// kind beside it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CurveDatum {
    /// A line's `origin`.
    Origin,
    /// A line's `dir`.
    Dir,
    /// A circle's, ellipse's or spiric's `center`.
    Center,
    /// The `axis` of a circle, ellipse or spiric.
    Axis,
    /// The seam direction `u_ref` of a circle, ellipse or spiric.
    URef,
    /// A circle's `radius`.
    Radius,
    /// An ellipse's semi-major `major`.
    Major,
    /// An ellipse's semi-minor `minor`.
    Minor,
    /// A spiric's torus `major_radius`.
    MajorRadius,
    /// A spiric's torus `minor_radius`.
    MinorRadius,
    /// A spiric's plane stand-off `offset`.
    Offset,
}

impl CurveDatum {
    /// The datum's field name, as the variant spells it.
    ///
    /// **Hand-kept against the variants' field names**, and nothing
    /// derives it: a renamed field leaves this string stale with
    /// nothing red. The exhaustive match only guarantees every datum
    /// HAS a name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Origin => "origin",
            Self::Dir => "dir",
            Self::Center => "center",
            Self::Axis => "axis",
            Self::URef => "u_ref",
            Self::Radius => "radius",
            Self::Major => "major",
            Self::Minor => "minor",
            Self::MajorRadius => "major_radius",
            Self::MinorRadius => "minor_radius",
            Self::Offset => "offset",
        }
    }
}

impl<T: Real> Curve3<T> {
    /// **The representability margins of this carrier's datum
    /// conventions** — the curve half of
    /// [`crate::Surface::representability_margins`], with that door's
    /// contract: each quantity a variant's docs require to be strictly
    /// positive for its stored datum to describe the curve the variant
    /// names at all, named by the datum it constrains and the END of the
    /// convention it measures:
    ///
    /// - `Circle`: `radius`, lower end (`radius > 0`: at zero the circle
    ///   is a point);
    /// - `Ellipse`: `major` and `minor`, each at its lower end (a zero
    ///   semi-axis is a segment or a point). The ordering
    ///   `major > minor` relates two datums rather than bounding one,
    ///   and is not a margin here — a swapped pair still describes an
    ///   ellipse, and [`Curve3::ellipse`] is where the ordering is
    ///   decided;
    /// - `Spiric`: `minor_radius`, lower end (the `r > 0` half of the
    ///   ring convention). `R > r` and `|offset| < R − r` relate datums,
    ///   and [`Curve3::spiric`] decides them;
    /// - `Line`, `Nurbs`: none — a line's datums carry no scalar
    ///   convention, and a spline's datum is its net.
    ///
    /// **Nothing is decided here**, exactly as on the surface door: the
    /// quantities are computed at `T` and returned, a margin of a
    /// poisoned datum is poison, and the variants are destructured
    /// without `..` so a field a variant gains is a compile error here.
    pub fn representability_margins(
        &self,
    ) -> [Option<crate::RepresentabilityMargin<T, CurveDatum>>; 2] {
        let lower = |datum, margin| {
            Some(crate::RepresentabilityMargin {
                datum,
                end: crate::ConventionEnd::Lower,
                margin,
            })
        };
        match self {
            Curve3::Circle {
                center: _,
                axis: _,
                radius,
                u_ref: _,
            } => [lower(CurveDatum::Radius, *radius), None],
            Curve3::Ellipse {
                center: _,
                axis: _,
                major,
                minor,
                u_ref: _,
            } => [
                lower(CurveDatum::Major, *major),
                lower(CurveDatum::Minor, *minor),
            ],
            Curve3::Spiric {
                center: _,
                axis: _,
                u_ref: _,
                major_radius: _,
                minor_radius,
                offset: _,
            } => [lower(CurveDatum::MinorRadius, *minor_radius), None],
            Curve3::Line { origin: _, dir: _ } | Curve3::Nurbs(_) => [None, None],
        }
    }

    /// The "no description yet" NURBS state (the former unit
    /// placeholder variant, as data): a structurally valid payload
    /// whose control points are all-poison, so evaluation yields the
    /// all-poison point and every downstream certification fails
    /// loudly (D4 ¶2) — representable ≠ described.
    pub fn nurbs_placeholder() -> Self {
        Curve3::Nurbs(Arc::new(NurbsCurve3::placeholder()))
    }
}

impl<T: Decide> Curve3<T> {
    /// The one deciding door into [`Curve3::Ellipse`] (M5 PR 5 spec §1):
    /// refuses `major = minor` (that configuration is a `Circle` — one
    /// kind per configuration, D3), swapped axes, and a non-positive
    /// minor, each through a named Q1 trilean:
    ///
    /// - `ellipse_axes_distinct` — margin `major − minor` (meters):
    ///   Positive ⇒ a genuine ellipse; Zero ⇒
    ///   [`EllipseInvalid::CircularAxes`]; Negative ⇒
    ///   [`EllipseInvalid::AxesSwapped`]; in-band/poison ⇒
    ///   [`EllipseInvalid::Escalated`].
    /// - `ellipse_minor_positive` — margin `minor` (meters): Positive
    ///   required; Zero/Negative ⇒ [`EllipseInvalid::MinorNotPositive`];
    ///   in-band/poison ⇒ escalated.
    ///
    /// Frame fields stay conventional data exactly as for `Circle`
    /// (unit, orthogonal, unchecked here — tier-3 certification owns
    /// them at rest).
    ///
    /// # Errors
    ///
    /// [`EllipseInvalid`] — see each variant.
    pub fn ellipse(
        center: Point3<T>,
        axis: Vec3<T>,
        major: T,
        minor: T,
        u_ref: Vec3<T>,
        band: Band,
    ) -> Result<Self, EllipseInvalid> {
        match geom_core::k_stats::decide("ellipse_minor_positive", Margin::of(minor), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => return Err(EllipseInvalid::MinorNotPositive),
            Err(diag) => return Err(EllipseInvalid::Escalated(diag)),
        }
        match geom_core::k_stats::decide("ellipse_axes_distinct", Margin::of(major - minor), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero) => return Err(EllipseInvalid::CircularAxes),
            Ok(Sign::Negative) => return Err(EllipseInvalid::AxesSwapped),
            Err(diag) => return Err(EllipseInvalid::Escalated(diag)),
        }
        Ok(Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        })
    }

    /// The one deciding door into [`Curve3::Spiric`]: refuses a
    /// non-positive minor radius, a non-ring torus, a stand-off at or
    /// past the inner equator, and a cutting plane not parallel to the
    /// axis, each through a named trilean:
    ///
    /// - `spiric_minor_positive` — margin `minor_radius` (m): Positive
    ///   required; else [`SpiricInvalid::MinorNotPositive`].
    /// - `spiric_ring` — margin `major_radius − minor_radius` (m):
    ///   Positive required; else [`SpiricInvalid::NotARing`].
    /// - `spiric_two_ovals` — margin `(major_radius − minor_radius) −
    ///   |offset|` (m), the length the two-oval regime closes by,
    ///   decided BEFORE any root is taken: Positive required; else
    ///   [`SpiricInvalid::NotTwoOvals`].
    /// - `spiric_frame_orthogonal` — margin `axis · u_ref` levered at
    ///   `major_radius + minor_radius` (m, the farthest point the
    ///   frame places): Zero required; else
    ///   [`SpiricInvalid::FrameNotOrthogonal`].
    ///
    /// In-band or poison on any of them ⇒ [`SpiricInvalid::Escalated`].
    /// The frame's unit lengths stay conventional data exactly as for
    /// `Circle` (unchecked here — tier-3 certification owns them at
    /// rest).
    ///
    /// # Errors
    ///
    /// [`SpiricInvalid`] — see each variant.
    pub fn spiric(
        center: Point3<T>,
        axis: Vec3<T>,
        u_ref: Vec3<T>,
        major_radius: T,
        minor_radius: T,
        offset: T,
        band: Band,
    ) -> Result<Self, SpiricInvalid> {
        use geom_core::k_stats::decide;
        match decide("spiric_minor_positive", Margin::of(minor_radius), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => return Err(SpiricInvalid::MinorNotPositive),
            Err(diag) => return Err(SpiricInvalid::Escalated(diag)),
        }
        let ring = major_radius - minor_radius;
        match decide("spiric_ring", Margin::of(ring), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => return Err(SpiricInvalid::NotARing),
            Err(diag) => return Err(SpiricInvalid::Escalated(diag)),
        }
        match decide("spiric_two_ovals", Margin::of(ring - offset.abs()), band) {
            Ok(Sign::Positive) => {}
            Ok(Sign::Zero | Sign::Negative) => return Err(SpiricInvalid::NotTwoOvals),
            Err(diag) => return Err(SpiricInvalid::Escalated(diag)),
        }
        match decide(
            "spiric_frame_orthogonal",
            Margin::levered(axis.dot(u_ref), major_radius + minor_radius),
            band,
        ) {
            Ok(Sign::Zero) => {}
            Ok(Sign::Positive | Sign::Negative) => return Err(SpiricInvalid::FrameNotOrthogonal),
            Err(diag) => return Err(SpiricInvalid::Escalated(diag)),
        }
        Ok(Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        })
    }
}

impl<T: Real> Curve3<T> {
    /// **A circle carrier's point at parameter `t`**, as
    /// [`Curve3::eval`] builds it — `(s, c) = t.sin_cos()`,
    /// `radial = u_ref·c + v_ref·s` with `v_ref = axis × u_ref`, result
    /// `center + radial·radius`, exactly as parenthesized (D9).
    ///
    /// It is a door rather than a copy: the point expression lives in
    /// [`Self::circle_point`] alone, this is that expression on a frame
    /// built here, `eval`'s `Circle` arm CALLS this, and `ders1`'s
    /// `Circle` arm calls `circle_point` on the one frame it shares
    /// with its tangent half — so a caller that builds a point here
    /// builds the very node either door would. That is what
    /// `sweep::swept::register_span_identity` rests on — node ids are
    /// content hashes, so "the constructor states the identity about
    /// the node the certifier will ask about" is a fact of this
    /// delegation and not a transcription anyone has to keep in step.
    ///
    /// Its own bound is [`Real`] alone, and that is the point:
    /// `eval` carries [`SpanLocate`] for its `Nurbs` arm's sealed span
    /// selection, and evaluation-code discipline forbids a generic
    /// caller from carrying a second bound beside `Real` to reach it.
    pub fn circle_at(
        center: Point3<T>,
        axis: Vec3<T>,
        radius: T,
        u_ref: Vec3<T>,
        t: T,
    ) -> Point3<T> {
        Self::circle_point(center, &azimuth::frame(axis, u_ref, t), radius)
    }

    /// The circle's point from an azimuthal frame already built:
    /// `center + radial·radius`, exactly as parenthesized (D9). The one
    /// spelling of that expression — [`Self::circle_at`] builds the
    /// frame and calls this; `ders1`'s `Circle` arm calls this on the
    /// frame its tangent half reads too, which is how the jet pays one
    /// frame and still produces `eval`'s bits.
    fn circle_point(center: Point3<T>, frame: &azimuth::AzimuthFrame<T>, radius: T) -> Point3<T> {
        center + frame.radial.0 * radius
    }
}

/// The range of a spiric's `f = √(ρ² − offset²)` over one period, as
/// `(f_min, f_max)` at `ρ = R ∓ r` — `f` is monotone in `ρ` and
/// `ρ ∈ [R − r, R + r]`. The one spelling the amplitude boxes read
/// (`topo`'s census reach, `mesh`'s chord sizing); `curves::boxes`
/// spells the same two numbers in its outward-bracket arithmetic, which
/// no plain-scalar helper can supply. Off-regime data (`ρ_min² <
/// offset²`) yields poison in `f_min`, as every evaluator does.
pub fn spiric_f_range<T: Real>(major: T, minor: T, offset: T) -> (T, T) {
    let d2 = offset.powi(2);
    (
        ((major - minor).powi(2) - d2).sqrt(),
        ((major + minor).powi(2) - d2).sqrt(),
    )
}

/// The spiric's radial pair from `c = cos v`: `ρ = R + r·c` and
/// `f = √(ρ² − offset²)` — one `sqrt`, fixed order (D9). Shared by the
/// three evaluators so the radicand is spelled once; each evaluator
/// takes its one `sin_cos` itself.
fn spiric_radial<T: Real>(major: T, minor: T, offset: T, c: T) -> (T, T) {
    let rho = major + minor * c;
    (rho, (rho.powi(2) - offset.powi(2)).sqrt())
}

impl<T: SpanLocate> Curve3<T> {
    /// The point at parameter `t` (see the variant docs for each
    /// parameterization; the crate docs for units and periodicity).
    ///
    /// Evaluation orders (fixed, D9):
    /// - Line: `origin + dir·t` — one componentwise scale, one add.
    /// - Circle: `(s, c) = θ.sin_cos()`; `radial = u_ref·c + v_ref·s`
    ///   with `v_ref = axis × u_ref` (the cross's own fixed order);
    ///   result `center + radial·radius` — exactly as parenthesized.
    /// - Ellipse: `(s, c) = θ.sin_cos()`;
    ///   `center + (u_ref·(major·c) + v_ref·(minor·s))` with
    ///   `v_ref = axis × u_ref` — exactly as parenthesized (the per-axis
    ///   scales multiply the trig values first, then scale the frame
    ///   vectors).
    /// - Spiric: `(s, c) = v.sin_cos()`; `ρ = R + r·c`;
    ///   `f = (ρ² − offset²).sqrt()`; `m = axis × u_ref`;
    ///   `center + u_ref·offset + m·f + axis·(r·s)` — exactly as
    ///   parenthesized, one `sin_cos`, one `sqrt`, no branch.
    /// - Nurbs: the payload's [`NurbsCurve3::eval`] (span selection via
    ///   the sealed seam; all-poison for the placeholder state).
    pub fn eval(&self, t: T) -> Point3<T> {
        match self {
            Curve3::Line { origin, dir } => *origin + *dir * t,
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Self::circle_at(*center, *axis, *radius, *u_ref, t),
            Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => {
                let ((s, c), v_ref) = azimuth::basis(*axis, *u_ref, t);
                *center + (*u_ref * (*major * c) + v_ref * (*minor * s))
            }
            Curve3::Spiric {
                center,
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
            } => {
                let ((s, c), m) = azimuth::basis(*axis, *u_ref, t);
                let (_, f) = spiric_radial(*major_radius, *minor_radius, *offset, c);
                *center + *u_ref * *offset + m * f + *axis * (*minor_radius * s)
            }
            Curve3::Nurbs(n) => n.eval(t),
        }
    }

    /// The first derivative `dP/dt` at parameter `t`.
    ///
    /// - Line: `dir`, constant (arc-length parameterization for unit
    ///   `dir`).
    /// - Circle: the tangent `(u_ref·(−s) + v_ref·c)·radius`, evaluated
    ///   exactly as written from one `sin_cos` call (fixed order;
    ///   `|dP/dθ| = radius`, the radians-to-meters rate).
    /// - Ellipse: `u_ref·(−(major·s)) + v_ref·(minor·c)` — fixed order;
    ///   `|dP/dθ|` varies in `[minor, major]` (θ is the eccentric
    ///   anomaly, not arc length).
    /// - Spiric: `m·f′ + axis·(r·c)` with `f′ = −r·ρ·s/f` — fixed
    ///   order; `|dP/dv| ≥ r` (the variant docs).
    /// - Nurbs: the payload’s derivative (all-poison for the placeholder).
    ///
    /// The order-1 jet is [`Self::ders1`]: a caller wanting the point
    /// and this at one `t` asks once. There is no order-2 jet on the
    /// enum: a caller wanting `deriv` and [`Self::deriv2`] at one `t`
    /// pays two frames on the conic arms (its one consumer, the
    /// splitting orbit's conic arm, never reaches `Nurbs`, where the
    /// payload's [`NurbsCurve3::ders`] would answer all three from one
    /// pass).
    pub fn deriv(&self, t: T) -> Vec3<T> {
        match self {
            Curve3::Line { dir, .. } => *dir,
            Curve3::Circle {
                axis,
                radius,
                u_ref,
                ..
            } => {
                let tangential = azimuth::frame(*axis, *u_ref, t).tangential.0;
                tangential * *radius
            }
            Curve3::Ellipse {
                axis,
                major,
                minor,
                u_ref,
                ..
            } => {
                let ((s, c), v_ref) = azimuth::basis(*axis, *u_ref, t);
                *u_ref * (-(*major * s)) + v_ref * (*minor * c)
            }
            Curve3::Spiric {
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
                ..
            } => {
                let ((s, c), m) = azimuth::basis(*axis, *u_ref, t);
                let (rho, f) = spiric_radial(*major_radius, *minor_radius, *offset, c);
                let f1 = -(*minor_radius * rho * s) / f;
                m * f1 + *axis * (*minor_radius * c)
            }
            Curve3::Nurbs(n) => n.deriv(t),
        }
    }

    /// The point and the first derivative at parameter `t` from ONE
    /// pass — the order-1 jet for a caller wanting both, who would
    /// otherwise run [`Self::eval`] and [`Self::deriv`] (on `Nurbs`, two
    /// span selections and two basis passes for what one answers).
    ///
    /// Each half is its own evaluator's answer, bit for bit:
    /// - Line: `(origin + dir·t, dir)`.
    /// - Circle: one azimuthal frame, both its fields —
    ///   `(center + radial·radius, tangential·radius)`, the frame's own
    ///   formulas.
    /// - Ellipse: one `sin_cos`, then the two combinations exactly as
    ///   [`Self::eval`] and [`Self::deriv`] parenthesize them.
    /// - Spiric: one `sin_cos`, one `sqrt` (`ρ` and `f` shared), then
    ///   the two combinations exactly as [`Self::eval`] and
    ///   [`Self::deriv`] parenthesize them.
    /// - Nurbs: the payload's [`NurbsCurve3::ders1`].
    ///
    /// On the `Nurbs` arm the tangent half is `deriv`'s by
    /// construction and the point half is `eval`'s because the order-0
    /// row of the derivative basis recursion is the evaluation
    /// recursion, pinned by rows — [`NurbsCurve3::ders1`] says which is
    /// which. At `Dual` each half carries its own derivative channel;
    /// at `Interval` the point box and the tangent box are each their
    /// own evaluator's enclosure, hulled independently across the
    /// spans an interval parameter overlaps, not a coupled jet.
    ///
    /// The return is the tuple the NURBS jets return; a consumer
    /// destructures it on the spot. `eval` and `deriv` keep their own
    /// passes and are not projections of this one.
    pub fn ders1(&self, t: T) -> (Point3<T>, Vec3<T>) {
        match self {
            Curve3::Line { origin, dir } => (*origin + *dir * t, *dir),
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => {
                // One frame for both halves: that is the whole saving
                // on this arm, and no bit row can see it (two frames
                // give the same bits), so the `ders1_meter` row's
                // analytic table is its only guard.
                let f = azimuth::frame(*axis, *u_ref, t);
                (
                    Self::circle_point(*center, &f, *radius),
                    f.tangential.0 * *radius,
                )
            }
            Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => {
                let ((s, c), v_ref) = azimuth::basis(*axis, *u_ref, t);
                (
                    *center + (*u_ref * (*major * c) + v_ref * (*minor * s)),
                    *u_ref * (-(*major * s)) + v_ref * (*minor * c),
                )
            }
            Curve3::Spiric {
                center,
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
            } => {
                let ((s, c), m) = azimuth::basis(*axis, *u_ref, t);
                let (rho, f) = spiric_radial(*major_radius, *minor_radius, *offset, c);
                let f1 = -(*minor_radius * rho * s) / f;
                (
                    *center + *u_ref * *offset + m * f + *axis * (*minor_radius * s),
                    m * f1 + *axis * (*minor_radius * c),
                )
            }
            Curve3::Nurbs(n) => n.ders1(t),
        }
    }

    /// The second derivative `d²P/dt²` at parameter `t` — cheap for both
    /// analytic kinds and needed later for curvature (M2-PLAN PR 1).
    ///
    /// - Line: the zero vector, exactly.
    /// - Circle: `(u_ref·(−c) + v_ref·(−s))·radius` (the inward radial,
    ///   scaled; fixed order as written).
    /// - Ellipse: `u_ref·(−(major·c)) + v_ref·(−(minor·s))` — the
    ///   negated radial offset from the center (`P + P″ = center`
    ///   exactly in ℝ), fixed order as written.
    /// - Spiric: `m·f″ − axis·(r·s)` with
    ///   `f″ = −r·((ρ·c − r·s²)/f + r·ρ²·s²/f³)` — fixed order as
    ///   written.
    /// - Nurbs: the payload’s derivative (all-poison for the placeholder).
    pub fn deriv2(&self, t: T) -> Vec3<T> {
        match self {
            Curve3::Line { .. } => Vec3::zero(),
            Curve3::Circle {
                axis,
                radius,
                u_ref,
                ..
            } => {
                let ((s, c), v_ref) = azimuth::basis(*axis, *u_ref, t);
                (*u_ref * (-c) + v_ref * (-s)) * *radius
            }
            Curve3::Ellipse {
                axis,
                major,
                minor,
                u_ref,
                ..
            } => {
                let ((s, c), v_ref) = azimuth::basis(*axis, *u_ref, t);
                *u_ref * (-(*major * c)) + v_ref * (-(*minor * s))
            }
            Curve3::Spiric {
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
                ..
            } => {
                let ((s, c), m) = azimuth::basis(*axis, *u_ref, t);
                let r = *minor_radius;
                let (rho, f) = spiric_radial(*major_radius, r, *offset, c);
                let f2 = -(r
                    * ((rho * c - r * s.powi(2)) / f + r * rho.powi(2) * s.powi(2) / f.powi(3)));
                m * f2 - *axis * (r * s)
            }
            Curve3::Nurbs(n) => n.deriv2(t),
        }
    }

    /// The parameter of a point **on** this carrier, on the branch
    /// nearest `near` — the one body for point-on-carrier parameter
    /// recovery, and pure carrier arithmetic. `None` for the kinds
    /// whose inversion is a solve rather than a closed form.
    ///
    /// - **`Line`**: the projection `t = (p − origin)·dir`. A line's
    ///   parameterization is injective, so there is no branch to pick
    ///   and `near` is unused — the argument belongs to the periodic
    ///   kind and costs this arm nothing.
    /// - **`Circle`**: `near + δ` with `δ = atan2(w·τ̂, w·r̂)`,
    ///   `w = p − center`, and the frame at `near` read from the public
    ///   evaluators (`r̂·radius = eval(near) − center`,
    ///   `τ̂·radius = deriv(near)`). Both `atan2` arguments carry the
    ///   factor `radius`, so no division enters and no frame is
    ///   re-derived here. **The factor is not assumed positive.** A
    ///   `Curve3::Circle` with a representable NEGATIVE `radius` is a
    ///   circle traversed through the antipode of the `u_ref` seam, and
    ///   there `atan2` does not quotient the factor away — it flips the
    ///   angle by π, which is precisely the parameter that reproduces
    ///   the point. So this arm inverts `eval` for either sign, which
    ///   the retired seam spelling did NOT: reading the stored `u_ref`
    ///   and `v_ref` directly, it answers about the point's angle in
    ///   the frame rather than about its parameter, and at `radius =
    ///   −1`, `near = 0`, `t = 0` it returns `π` where this arm returns
    ///   `0` and `eval(0)` is the point. Not a claimed feature of the
    ///   consolidation — a measured consequence of reading the frame
    ///   from the evaluators, recorded so the sign is not later
    ///   "simplified" back out.
    /// - **`Spiric`**: the circle arm's anchored-difference form in the
    ///   torus's meridian half-plane: `h = (p − center)·axis`,
    ///   `ρ = |(p − center) − axis·h|`, `w = (ρ − R, h)`,
    ///   `r̂ = (cos near, sin near)`, `τ̂ = (−sin near, cos near)`,
    ///   `near + atan2(w·τ̂, w·r̂)`. Both `atan2` arguments are lengths,
    ///   the quotient is scale-free, and the branch is the one within a
    ///   half-turn of `near` — the circle arm's tie and midpoint-anchor
    ///   preconditions verbatim. It answers the point's minor angle on
    ///   EITHER oval: which oval the carrier names is the mint's
    ///   decision, not this arithmetic's.
    /// - **`Ellipse`, `Nurbs`**: `None`. The eccentric anomaly is not
    ///   the polar angle of the point, and a spline's inversion is
    ///   Newton on the foot-point condition (`project`) — a different
    ///   machine with a different refusal, not a branch policy.
    ///
    /// **Anchoring at `near` is what removes the branch cut.** `atan2`
    /// returns its principal value in `(−π, π]`, so `near + δ` is by
    /// construction the unique branch within half a turn of `near`:
    /// there is no `k·2π` to select, hence no ordering decision and no
    /// lane fork. A SEAM anchor would need that selection, which on a
    /// bare `Real` costs either an ordering (not available) or a
    /// `floor` whose interval answer widens across the integer. Here
    /// the interval scalar's `atan2` encloses the same value, and a
    /// `near` whose half-turn window straddles the cut widens the
    /// enclosure rather than mis-selecting a branch — degradation the
    /// consumer's own gate can see, never a silent turn.
    ///
    /// **Two preconditions, neither checked here**, because neither is
    /// this arithmetic's to decide:
    ///
    /// - `p` must be ON the carrier. Off it, the circle arm answers
    ///   about `p`'s radial projection and the line arm about its
    ///   axial one. The degenerate violation `p == center` has no
    ///   radial projection at all: `w` is the zero vector, both
    ///   `atan2` arguments are zero, and the total `atan2(0, 0) = 0`
    ///   makes the answer `near` itself. (The retired seam spellings
    ///   answered the nearest multiple of `τ` to `near` instead. Both
    ///   are arbitrary; this one is at least the anchor the caller
    ///   already had.) The spiric arm reads `w = (ρ − R, h)` in the
    ///   meridian half-plane, so `p == center` gives `w = (−R, 0)` and
    ///   the answer `near + π` — equally arbitrary, and equally the
    ///   caller's violation.
    /// - The branch the caller wants must be the one nearest `near`.
    ///   A caller recovering a parameter INSIDE a stored span
    ///   `[t₀, t₁]` can get that by passing the span's MIDPOINT, and
    ///   only while the span is at most one period: then `|t − mid|` is
    ///   at most half a period for every `t` in the span, so the
    ///   nearest branch to the midpoint IS the in-span one. Past a
    ///   period the answer aliases by `2π` and nothing downstream can
    ///   see it, so a caller with a span that long owes a period guard.
    ///
    /// **A THIRD PRECONDITION BELONGS TO SOME CALLERS AND NOT OTHERS,
    /// and it is what decides the anchor**: whether the answer may
    /// depend on the anchor the caller passed. It does, at the ulp
    /// scale, and unavoidably — `near` enters both `atan2` arguments
    /// through `eval(near)` and `deriv(near)`. So a caller whose anchor
    /// is derived from a STORED SPAN gets an answer that moves when the
    /// stored span moves, and a stored span is not a stable thing:
    /// splitting an edge rewrites it. A caller that needs the recovered
    /// parameter to be a function of the POINT and the CARRIER alone —
    /// because two orderings of the same operations have to agree
    /// bitwise — must anchor at something the CARRIER owns, and the
    /// circle's own such anchor is its SEAM, `near = 0`.
    /// `sweep::blend::surgery::seam_split_param` is that caller and
    /// carries the measurement that made it one; its period guard is
    /// what makes the principal branch the in-window one.
    ///
    /// # `|δ| = π` — the tie, and what it means at each posture
    ///
    /// At exactly half a turn the point has TWO parameters within half
    /// a turn of the anchor, `near ± π`, and this body returns one of
    /// them. Which one is not derivable: it is `atan2`'s cut, so the
    /// SIGN BIT of `w·τ̂` decides it — `Real::atan2(0.0, −1.0) = π`
    /// against `atan2(−0.0, −1.0) = −π`. Both answers are correct
    /// parameters of `p`; no answer is "the" one.
    ///
    /// **Midpoint-anchored callers are unaffected.** `|δ| = π` from a
    /// midpoint means the two ends of a full-period span, so whichever
    /// the tie names is an ENDPOINT, and a consumer whose interiority
    /// gate refuses a split at either end is unaffected by which.
    ///
    /// **ENDPOINT-anchored callers are NOT covered by that argument,
    /// and this is the harder half.** An endpoint anchor has no span to
    /// make the tie harmless: the two answers `t_old ± π` describe the
    /// same point but produce stored spans differing by a full turn,
    /// and a gate that checks only `eval(t_new) ≈ p` — which is the
    /// natural gate, and the one
    /// `topo::replace_face::plan_reanchors` writes — passes both,
    /// because both ARE parameters of the point. Such a caller is
    /// relying on never reaching `|δ| = π`: an endpoint that MOVES
    /// along its carrier does not jump half a turn, so the pose is
    /// sound, but it is a precondition on the caller's motion and not
    /// a property of this arithmetic. A caller that cannot argue that
    /// owes a `|δ| < π` refusal of its own; this body cannot make the
    /// choice for it, because at `|δ| = π` there is nothing to choose
    /// between.
    ///
    /// The size of the residue is measured rather than asserted:
    /// `geom`'s `curves/param_near.rs` row
    /// `at_the_half_turn_boundary_the_two_forms_disagree_by_a_turn_and_
    /// both_are_right` puts 9 of 30 boundary cases a full `2π` from
    /// what the retired seam-anchored longhand picks — close to a coin
    /// flip, and decided by two unrelated last bits.
    pub fn param_near(&self, p: Point3<T>, near: T) -> Option<T> {
        match self {
            Curve3::Line { origin, dir } => Some((p - *origin).dot(*dir)),
            Curve3::Circle { center, .. } => {
                let w = p - *center;
                let (p_near, tau_near) = self.ders1(near);
                let r_near = p_near - *center;
                Some(near + w.dot(tau_near).atan2(w.dot(r_near)))
            }
            Curve3::Spiric {
                center,
                axis,
                major_radius,
                ..
            } => {
                let q = p - *center;
                let h = q.dot(*axis);
                let rho = (q - *axis * h).norm();
                let (w_rho, w_h) = (rho - *major_radius, h);
                let (s, c) = near.sin_cos();
                let along = w_rho * c + w_h * s;
                let across = w_rho * (-s) + w_h * c;
                Some(near + across.atan2(along))
            }
            Curve3::Ellipse { .. } | Curve3::Nurbs(_) => None,
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use core::f64::consts::{FRAC_PI_2, PI, TAU};

    use geom_core::{Dual, Dual64};
    use proptest::prelude::*;

    use super::*;

    /// A unit-ish circle fixture in a tilted frame: axis +z rotated is
    /// avoided on purpose — the frame is exactly representable so the
    /// closed-form checks stay exact where possible.
    fn xy_circle(r: f64) -> Curve3<f64> {
        Curve3::Circle {
            center: Point3::new(1.0, 2.0, 3.0),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        }
    }

    /// A general (non-axis-aligned but exactly orthonormal) frame:
    /// axis ∝ (2, 2, 1)/3, u_ref ∝ (1, −2, 2)/3 — an exact integer
    /// orthogonal triple scaled by exact 1/3, so the frame is unit and
    /// orthogonal to rounding-free precision (components are exact
    /// ratios with power-of-two-free denominators — 2/3, 1/3 round, but
    /// dot products still cancel to ~1 ulp).
    fn tilted_circle() -> Curve3<f64> {
        Curve3::Circle {
            center: Point3::new(-0.5, 4.0, 1.25),
            axis: Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0),
            radius: 2.5,
            u_ref: Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0),
        }
    }

    fn assert_point_close(p: Point3<f64>, q: Point3<f64>, tol: f64) {
        assert!((p.x - q.x).abs() <= tol, "x: {} vs {}", p.x, q.x);
        assert!((p.y - q.y).abs() <= tol, "y: {} vs {}", p.y, q.y);
        assert!((p.z - q.z).abs() <= tol, "z: {} vs {}", p.z, q.z);
    }

    // ------------------------------------------------------------------
    // Closed-form loci
    // ------------------------------------------------------------------

    #[test]
    fn line_evaluates_exactly_on_dyadic_data() {
        let line = Curve3::Line {
            origin: Point3::new(1.0, -2.0, 0.5),
            dir: Vec3::new(0.0, 0.0, 1.0),
        };
        let p = line.eval(3.25);
        assert_eq!((p.x, p.y, p.z), (1.0, -2.0, 3.75));
        let d = line.deriv(3.25);
        assert_eq!((d.x, d.y, d.z), (0.0, 0.0, 1.0));
        let d2 = line.deriv2(3.25);
        assert_eq!((d2.x, d2.y, d2.z), (0.0, 0.0, 0.0));
        // t is a length: eval(t) is exactly t meters from origin for
        // unit dir (exact here: dyadic data).
        assert_eq!(line.eval(0.0).distance(line.eval(3.25)), 3.25);
    }

    #[test]
    fn circle_cardinal_points() {
        let c = xy_circle(2.0);
        // θ = 0: center + u_ref·r — exact (sin_cos(0) = (0, 1) exactly).
        let p0 = c.eval(0.0);
        assert_eq!((p0.x, p0.y, p0.z), (3.0, 2.0, 3.0));
        // θ = π/2: center + v_ref·r to rounding (fl(π/2) ≠ π/2).
        assert_point_close(c.eval(FRAC_PI_2), Point3::new(1.0, 4.0, 3.0), 1e-15);
        // θ = π: center − u_ref·r.
        assert_point_close(c.eval(PI), Point3::new(-1.0, 2.0, 3.0), 1e-15);
        // Winding is counterclockwise viewed from +axis (right-hand
        // rule): at θ = 0 the tangent points along +v_ref = +y.
        let t0 = c.deriv(0.0);
        assert_eq!((t0.x, t0.y, t0.z), (0.0, 2.0, 0.0));
        // Second derivative at θ = 0 is the inward radial, −u_ref·r.
        let a0 = c.deriv2(0.0);
        assert_eq!((a0.x, a0.y, a0.z), (-2.0, 0.0, 0.0));
    }

    proptest! {
        /// The circle's defining residuals at arbitrary θ and frames:
        /// distance to center = r, and the point lies in the circle's
        /// plane — a few roundings of magnitudes ≤ ~|center| + r.
        #[test]
        fn circle_point_lies_on_locus(theta in -50.0..50.0f64) {
            let c = tilted_circle();
            let (center, axis, r) = match c {
                Curve3::Circle { center, axis, radius, .. } => (center, axis, radius),
                _ => panic!("fixture is a circle"),
            };
            let p = c.eval(theta);
            prop_assert!((p.distance(center) - r).abs() <= 1e-13);
            prop_assert!((p - center).dot(axis).abs() <= 1e-13);
        }

        /// Tangent orthogonality and speed: dP/dθ ⊥ (P − center),
        /// dP/dθ ⊥ axis, |dP/dθ| = r (the radians-to-meters rate); the
        /// second derivative is the inward radial: P + deriv2/1 = center
        /// in exact arithmetic… stated as deriv2 = −(P − center).
        #[test]
        fn circle_derivative_geometry(theta in -50.0..50.0f64) {
            let c = tilted_circle();
            let (center, axis, r) = match c {
                Curve3::Circle { center, axis, radius, .. } => (center, axis, radius),
                _ => panic!("fixture is a circle"),
            };
            let p = c.eval(theta);
            let d = c.deriv(theta);
            let d2 = c.deriv2(theta);
            prop_assert!(d.dot(p - center).abs() <= 1e-12);
            prop_assert!(d.dot(axis).abs() <= 1e-13);
            prop_assert!((d.norm() - r).abs() <= 1e-13);
            let radial = p - center;
            prop_assert!((d2.x + radial.x).abs() <= 1e-13);
            prop_assert!((d2.y + radial.y).abs() <= 1e-13);
            prop_assert!((d2.z + radial.z).abs() <= 1e-13);
        }

        /// Derivative-vs-Dual consistency, the M2 test axis: seeding θ
        /// as a dual variable and evaluating `eval` must produce the
        /// closed-form `deriv` in the tangent channel — algebraically
        /// identical expressions differing only in rounding order, so
        /// the agreement bound is tight (both are a handful of libm ops
        /// on O(r) magnitudes).
        #[test]
        fn circle_deriv_matches_dual_of_eval(
            theta in -50.0..50.0f64,
            seed in prop_oneof![-100.0..-0.01f64, 0.01..100.0f64],
        ) {
            let c = tilted_circle();
            let cd: Curve3<Dual64> = c.map_scalar(Dual::constant);
            let p = cd.eval(Dual::new(theta, seed));
            let d = c.deriv(theta);
            // Value channel: bit-identical to the f64 evaluation.
            let pf = c.eval(theta);
            prop_assert_eq!(p.x.value.to_bits(), pf.x.to_bits());
            prop_assert_eq!(p.y.value.to_bits(), pf.y.to_bits());
            prop_assert_eq!(p.z.value.to_bits(), pf.z.to_bits());
            // Tangent channel: the closed-form derivative scaled by the
            // seed (the chain rule), within rounding.
            prop_assert!((p.x.deriv - seed * d.x).abs() <= 1e-11 * (1.0 + seed.abs()));
            prop_assert!((p.y.deriv - seed * d.y).abs() <= 1e-11 * (1.0 + seed.abs()));
            prop_assert!((p.z.deriv - seed * d.z).abs() <= 1e-11 * (1.0 + seed.abs()));
        }

        /// Same axis one order up: dual of `deriv` matches `deriv2`.
        #[test]
        fn circle_deriv2_matches_dual_of_deriv(theta in -50.0..50.0f64) {
            let c = tilted_circle();
            let cd: Curve3<Dual64> = c.map_scalar(Dual::constant);
            let d = cd.deriv(Dual::variable(theta));
            let d2 = c.deriv2(theta);
            prop_assert!((d.x.deriv - d2.x).abs() <= 1e-12);
            prop_assert!((d.y.deriv - d2.y).abs() <= 1e-12);
            prop_assert!((d.z.deriv - d2.z).abs() <= 1e-12);
        }

        /// Line: dual-of-eval reproduces `deriv` (here exactly — the
        /// chain rule multiplies dir by the seed, both single products).
        #[test]
        fn line_deriv_matches_dual_of_eval(t in -1.0e3..1.0e3f64) {
            let line = Curve3::Line {
                origin: Point3::new(1.0, -2.0, 0.5),
                dir: Vec3::new(3.0 / 13.0, 4.0 / 13.0, 12.0 / 13.0),
            };
            let ld: Curve3<Dual64> = line.map_scalar(Dual::constant);
            let p = ld.eval(Dual::variable(t));
            let d = line.deriv(t);
            prop_assert_eq!(p.x.deriv.to_bits(), d.x.to_bits());
            prop_assert_eq!(p.y.deriv.to_bits(), d.y.to_bits());
            prop_assert_eq!(p.z.deriv.to_bits(), d.z.to_bits());
        }

        /// Periodicity, the honest value-level statement: eval(θ) and
        /// eval(θ + k·fl(τ)) agree to rounding scaled by k — never
        /// asserted bitwise (fl(τ) ≠ τ, so the parameters differ as
        /// reals by k·(τ − fl(τ)) ≈ k·2.4e-16, times |dP/dθ| = r).
        #[test]
        fn circle_periodicity_value_level(
            theta in -10.0..10.0f64,
            k in -100i32..100,
        ) {
            let c = tilted_circle();
            let p = c.eval(theta);
            let q = c.eval(theta + f64::from(k) * TAU);
            let slack = 1e-15 + 5e-15 * f64::from(k).abs();
            assert_point_close(p, q, slack);
        }
    }

    // ------------------------------------------------------------------
    // Ellipse (M5 PR 5): constructor trileans, locus, derivatives
    // ------------------------------------------------------------------

    /// A pure band (the geom-core test discipline: never `Band::linear`
    /// in a lib test — the global `Tolerance` stays untouched).
    fn band() -> geom_core::Band {
        geom_core::Band::new(1e-9, 1e-8).unwrap()
    }

    /// The tilted-frame ellipse fixture (same exact orthonormal frame
    /// as [`tilted_circle`]).
    fn tilted_ellipse() -> Curve3<f64> {
        Curve3::ellipse(
            Point3::new(-0.5, 4.0, 1.25),
            Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0),
            2.5,
            1.0,
            Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0),
            band(),
        )
        .unwrap()
    }

    #[test]
    fn ellipse_cardinal_points() {
        let e = Curve3::ellipse(
            Point3::new(1.0, 2.0, 3.0),
            Vec3::unit_z(),
            2.0,
            0.5,
            Vec3::unit_x(),
            band(),
        )
        .unwrap();
        // θ = 0: center + u_ref·major — exact.
        let p0 = e.eval(0.0);
        assert_eq!((p0.x, p0.y, p0.z), (3.0, 2.0, 3.0));
        // θ = π/2: center + v_ref·minor to rounding.
        assert_point_close(e.eval(FRAC_PI_2), Point3::new(1.0, 2.5, 3.0), 1e-15);
        // θ = π: center − u_ref·major.
        assert_point_close(e.eval(PI), Point3::new(-1.0, 2.0, 3.0), 1e-15);
        // Winding: CCW viewed from +axis — tangent at θ = 0 is +v̂·minor.
        let t0 = e.deriv(0.0);
        assert_eq!((t0.x, t0.y, t0.z), (0.0, 0.5, 0.0));
        // deriv2 at θ = 0 is the negated radial offset: −u_ref·major.
        let a0 = e.deriv2(0.0);
        assert_eq!((a0.x, a0.y, a0.z), (-2.0, 0.0, 0.0));
    }

    /// The constructor trilean trios (M5 PR 5 acceptance: each named
    /// predicate gets exactly-degenerate, definitely-generic, and
    /// in-band rows).
    #[test]
    fn ellipse_constructor_trios() {
        let b = band();
        let mk = |major: f64, minor: f64| {
            Curve3::ellipse(
                Point3::origin(),
                Vec3::unit_z(),
                major,
                minor,
                Vec3::unit_x(),
                b,
            )
        };
        // ellipse_axes_distinct: definitely-generic passes …
        assert!(mk(2.0, 1.0).is_ok());
        // … exactly-degenerate (major = minor, margin 0) refuses as the
        // circular coincidence …
        assert_eq!(mk(1.0, 1.0).unwrap_err(), EllipseInvalid::CircularAxes);
        // … a sub-ε separation (dyadic 2⁻³¹ ≈ 4.7e-10, exact under
        // subtraction) still refuses as the coincidence …
        assert_eq!(
            mk(1.0 + 2.0f64.powi(-31), 1.0).unwrap_err(),
            EllipseInvalid::CircularAxes
        );
        // … in-band escalates typed …
        let err = mk(1.0 + 5e-9, 1.0).unwrap_err();
        assert!(matches!(err, EllipseInvalid::Escalated(_)), "{err:?}");
        // … and definitely-swapped refuses with the swap story.
        assert_eq!(mk(1.0, 2.0).unwrap_err(), EllipseInvalid::AxesSwapped);

        // ellipse_minor_positive: zero and negative refuse, in-band
        // escalates, poison escalates (total, never a panic).
        assert_eq!(mk(2.0, 0.0).unwrap_err(), EllipseInvalid::MinorNotPositive);
        assert_eq!(mk(2.0, -1.0).unwrap_err(), EllipseInvalid::MinorNotPositive);
        let err = mk(2.0, 5e-9).unwrap_err();
        assert!(matches!(err, EllipseInvalid::Escalated(_)), "{err:?}");
        let err = mk(2.0, f64::NAN).unwrap_err();
        assert!(matches!(err, EllipseInvalid::Escalated(_)), "{err:?}");

        // The circular-coincidence refusals compose the shared
        // two-tolerance recourse (S6 / D4 ¶1 addendum), exactly once.
        for e in [
            mk(1.0, 1.0).unwrap_err().to_string(),
            mk(1.0 + 5e-9, 1.0).unwrap_err().to_string(),
        ] {
            assert_eq!(e.matches(geom_core::COINCIDENCE_RECOURSE).count(), 1, "{e}");
        }
    }

    proptest! {
        /// The ellipse's defining residuals at arbitrary θ on the tilted
        /// frame: the frame-coordinate quadratic (x/a)² + (y/b)² − 1
        /// vanishes and the point lies in the ellipse's plane.
        #[test]
        fn ellipse_point_lies_on_locus(theta in -50.0..50.0f64) {
            let e = tilted_ellipse();
            let Curve3::Ellipse { center, axis, major, minor, u_ref } = e else {
                panic!("fixture is an ellipse");
            };
            let v_ref = axis.cross(u_ref);
            let p = e.eval(theta);
            let d = p - center;
            let (x, y) = (d.dot(u_ref), d.dot(v_ref));
            let quad = (x / major) * (x / major) + (y / minor) * (y / minor) - 1.0;
            prop_assert!(quad.abs() <= 1e-12, "quadratic residual {quad:e}");
            prop_assert!(d.dot(axis).abs() <= 1e-13, "planarity");
        }

        /// Derivative geometry: P + P″ = center (the eccentric-anomaly
        /// identity), P′ ⊥ axis, and the speed interpolates the axes:
        /// |P′|² = major²·sin²θ + minor²·cos²θ.
        #[test]
        fn ellipse_derivative_geometry(theta in -50.0..50.0f64) {
            let e = tilted_ellipse();
            let Curve3::Ellipse { center, axis, major, minor, .. } = e else {
                panic!("fixture is an ellipse");
            };
            let p = e.eval(theta);
            let d1 = e.deriv(theta);
            let d2 = e.deriv2(theta);
            prop_assert!((p.x + d2.x - center.x).abs() <= 1e-12);
            prop_assert!((p.y + d2.y - center.y).abs() <= 1e-12);
            prop_assert!((p.z + d2.z - center.z).abs() <= 1e-12);
            prop_assert!(d1.dot(axis).abs() <= 1e-12);
            let (s, c) = theta.sin_cos();
            let speed2 = major.powi(2) * s.powi(2) + minor.powi(2) * c.powi(2);
            prop_assert!((d1.norm_squared() - speed2).abs() <= 1e-11);
        }

        /// Derivative-vs-Dual consistency (the M2 test axis) for the
        /// new variant: dual-of-eval reproduces `deriv`, value channel
        /// bit-identical; one order up for `deriv2`.
        #[test]
        fn ellipse_derivs_match_duals(theta in -50.0..50.0f64) {
            let e = tilted_ellipse();
            let ed: Curve3<Dual64> = e.map_scalar(Dual::constant);
            let p = ed.eval(Dual::variable(theta));
            let pf = e.eval(theta);
            prop_assert_eq!(p.x.value.to_bits(), pf.x.to_bits());
            prop_assert_eq!(p.y.value.to_bits(), pf.y.to_bits());
            prop_assert_eq!(p.z.value.to_bits(), pf.z.to_bits());
            let d = e.deriv(theta);
            prop_assert!((p.x.deriv - d.x).abs() <= 1e-11);
            prop_assert!((p.y.deriv - d.y).abs() <= 1e-11);
            prop_assert!((p.z.deriv - d.z).abs() <= 1e-11);
            let dd = ed.deriv(Dual::variable(theta));
            let d2 = e.deriv2(theta);
            prop_assert!((dd.x.deriv - d2.x).abs() <= 1e-11);
            prop_assert!((dd.y.deriv - d2.y).abs() <= 1e-11);
            prop_assert!((dd.z.deriv - d2.z).abs() <= 1e-11);
        }

        /// Periodicity at the value level (the honest statement — the
        /// crate-doc policy; never bitwise).
        #[test]
        fn ellipse_periodicity_value_level(
            theta in -10.0..10.0f64,
            k in -100i32..100,
        ) {
            let e = tilted_ellipse();
            let p = e.eval(theta);
            let q = e.eval(theta + f64::from(k) * TAU);
            let slack = 1e-15 + 5e-15 * f64::from(k).abs();
            assert_point_close(p, q, slack);
        }
    }

    // ------------------------------------------------------------------
    // The spiric — the elbow's numbers in `tilted_circle`'s frame,
    // orthonormal to rounding (`|axis|² − 1 = −1.1e-16`: thirds do not
    // round exactly), so the closed forms are read against the plane
    // and speed identities rather than a coordinate-aligned special
    // case.
    // ------------------------------------------------------------------

    /// `T(R = 1.2, r = 0.225)` cut at stand-off `d = 0.05`: the klein
    /// elbow's moved rim (`torax_axial`'s `R`, `r − t`, `t`).
    fn tilted_spiric() -> Curve3<f64> {
        Curve3::spiric(
            Point3::new(-0.5, 4.0, 1.25),
            Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0),
            Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0),
            1.2,
            0.225,
            0.05,
            band(),
        )
        .expect("the elbow's numbers are a ring torus cut short of its inner equator")
    }

    /// **The constructor's four refusals, each on the number that
    /// breaks its predicate**, and the acceptance the fixture relies
    /// on. The ellipse constructor's row shape.
    #[test]
    fn spiric_constructor_decides_its_regime() {
        let mk = |big_r: f64, r: f64, d: f64, u_ref: Vec3<f64>| {
            Curve3::spiric(Point3::origin(), Vec3::unit_z(), u_ref, big_r, r, d, band())
        };
        assert!(matches!(
            mk(1.2, 0.225, 0.05, Vec3::unit_x()),
            Ok(Curve3::Spiric { .. })
        ));
        assert_eq!(
            mk(1.2, 0.0, 0.05, Vec3::unit_x()).err(),
            Some(SpiricInvalid::MinorNotPositive)
        );
        assert_eq!(
            mk(0.2, 0.225, 0.05, Vec3::unit_x()).err(),
            Some(SpiricInvalid::NotARing)
        );
        // Exactly at the inner equator: the node, refused as the
        // regime boundary it is.
        assert_eq!(
            mk(1.2, 0.225, 0.975, Vec3::unit_x()).err(),
            Some(SpiricInvalid::NotTwoOvals)
        );
        assert_eq!(
            mk(1.2, 0.225, -1.0, Vec3::unit_x()).err(),
            Some(SpiricInvalid::NotTwoOvals)
        );
        // A cutting plane tilted 30° off the axis.
        let tilted = Vec3::new(0.75_f64.sqrt(), 0.0, 0.5);
        assert_eq!(
            mk(1.2, 0.225, 0.05, tilted).err(),
            Some(SpiricInvalid::FrameNotOrthogonal)
        );
    }

    fn spiric_fields(c: &Curve3<f64>) -> (Point3<f64>, Vec3<f64>, Vec3<f64>, f64, f64, f64) {
        let Curve3::Spiric {
            center,
            axis,
            u_ref,
            major_radius,
            minor_radius,
            offset,
        } = *c
        else {
            panic!("fixture is a spiric");
        };
        (center, axis, u_ref, major_radius, minor_radius, offset)
    }

    /// Every sample lies in the cutting plane and on the torus (the
    /// torus read in its own `(ρ, h)` half-plane), at rounding. The
    /// mutant `offset ↦ −offset` in `eval` puts the plane residual at
    /// `2|d| = 0.1`; the other oval (`u_ref, offset` both negated)
    /// passes this row by construction and is the mint's decision.
    #[test]
    fn spiric_lies_on_its_plane_and_its_torus() {
        let c = tilted_spiric();
        let (center, axis, u_ref, big_r, r, d) = spiric_fields(&c);
        let mut worst_plane = 0.0_f64;
        let mut worst_torus = 0.0_f64;
        for i in 0..10_000 {
            let v = f64::from(i) * TAU / 10_000.0 - 3.0;
            let p = c.eval(v);
            let w = p - center;
            worst_plane = worst_plane.max((w.dot(u_ref) - d).abs());
            let h = w.dot(axis);
            let rho = (w - axis * h).norm();
            worst_torus = worst_torus.max(((rho - big_r).hypot(h) - r).abs());
        }
        println!("[spiric] plane residual {worst_plane:e}, torus residual {worst_torus:e}");
        assert!(worst_plane <= 2e-15, "plane residual {worst_plane}");
        assert!(
            worst_torus <= 2e-15,
            "torus meridian residual {worst_torus}"
        );
    }

    /// `deriv` and `deriv2` against central differences of the closed
    /// form; kills a dropped `ρ/f` factor in `f′` (the speed would read
    /// `r` everywhere) or a dropped `f³` term in `f″`.
    #[test]
    fn spiric_derivatives_match_central_differences() {
        let c = tilted_spiric();
        for i in 0..2_000 {
            let v = f64::from(i) * TAU / 2_000.0 + 0.123;
            let h = 1e-5;
            let fd = (c.eval(v + h) - c.eval(v - h)) / (2.0 * h);
            let d1 = c.deriv(v);
            assert!((fd - d1).norm() <= 1e-8, "deriv at {v}: {d1:?} vs {fd:?}");
            let fd2 = (c.deriv(v + h) - c.deriv(v - h)) / (2.0 * h);
            let d2 = c.deriv2(v);
            assert!(
                (fd2 - d2).norm() <= 1e-7,
                "deriv2 at {v}: {d2:?} vs {fd2:?}"
            );
        }
    }

    /// `|dP/dv|` within `[r, r(R − r)/√((R − r)² − d²)]` at every sample;
    /// the floor is attained (at `v = 0` and `v = π`, where `sin v = 0`)
    /// and the sampled maximum reaches at least the `v = π/2` value
    /// `r·R/√(R² − d²)` (the `ρ²/(ρ² − d²)` factor — the mutant that
    /// drops it reads `r` everywhere). The ceiling is a bound, not
    /// attained: the factor peaks at `ρ = R − r`, where `sin v`
    /// vanishes.
    #[test]
    fn spiric_speed_within_its_bounds() {
        let c = tilted_spiric();
        let (_, _, _, big_r, r, d) = spiric_fields(&c);
        let ceiling = r * (big_r - r) / ((big_r - r).powi(2) - d * d).sqrt();
        let (mut lo, mut hi) = (f64::INFINITY, 0.0_f64);
        for i in 0..10_000 {
            let v = f64::from(i) * TAU / 10_000.0;
            let speed = c.deriv(v).norm();
            assert!(
                speed >= r - 1e-15,
                "speed {speed} below the floor {r} at {v}"
            );
            assert!(
                speed <= ceiling + 1e-15,
                "speed {speed} above the ceiling {ceiling} at {v}"
            );
            lo = lo.min(speed);
            hi = hi.max(speed);
        }
        assert!(
            (lo - r).abs() <= 1e-15,
            "the floor is attained: {lo} vs {r}"
        );
        // The rise above the floor, stated from `(d, R, r)`: at
        // `v = π/2` (a sample, `i = 2500`) `ρ = R` and
        // `|dP/dv| = r·R/√(R² − d²)`, which is what a dropped
        // `ρ²/(ρ² − d²)` factor flattens back to `r`.
        let rise = r * big_r / (big_r.powi(2) - d * d).sqrt();
        assert!(
            hi >= rise - 1e-15,
            "the sampled maximum {hi} is below the v = π/2 value {rise}"
        );
    }

    /// `param_near` inverts `eval` on the branch nearest the anchor, on
    /// either oval — the arithmetic reads the minor angle, not the
    /// side.
    #[test]
    fn spiric_param_near_inverts_eval_on_either_oval() {
        let c = tilted_spiric();
        let (center, axis, u_ref, big_r, r, d) = spiric_fields(&c);
        let other = Curve3::Spiric {
            center,
            axis,
            u_ref: -u_ref,
            major_radius: big_r,
            minor_radius: r,
            offset: -d,
        };
        for i in 0..1_000 {
            let v = f64::from(i) * TAU / 1_000.0 - 2.0;
            for anchor in [v + 0.3, v - 1.2, v + TAU] {
                let t = c.param_near(c.eval(v), anchor).expect("a spiric inverts");
                let want = if anchor > v + 4.0 { v + TAU } else { v };
                assert!((t - want).abs() <= 1e-12, "param_near({v}, {anchor}) = {t}");
                // The other oval's point has the SAME minor angle.
                let t2 = c
                    .param_near(other.eval(v), anchor)
                    .expect("a spiric inverts");
                assert!((t2 - want).abs() <= 1e-12, "other oval: {t2} vs {want}");
            }
        }
    }

    /// The whole-period box contains every sample, on every axis
    /// (kills a dropped `axis·e` term), and is the amplitude
    /// construction and nothing wider: per axis `e` the box's span is
    /// `|m·e|·(f_max − f_min) + 2r·|axis·e|` to rounding, since the
    /// endpoint hull lies inside that amplitude box (kills a box padded
    /// or doubled along any channel).
    #[test]
    fn spiric_arc_aabb_contains_every_sample() {
        let c = tilted_spiric();
        let (_, axis, u_ref, big_r, r, d) = spiric_fields(&c);
        let (t0, t1) = (0.4, 2.9);
        let b = boxes::conic_arc_aabb(&c, t0, t1, c.eval(t0), c.eval(t1)).expect("a spiric boxes");
        let inside = |x: f64, lo: f64, hi: f64| lo <= x && x <= hi;
        for i in 0..10_000 {
            let v = f64::from(i) * TAU / 10_000.0;
            let p = c.eval(v);
            assert!(
                inside(p.x, b.min_x, b.max_x)
                    && inside(p.y, b.min_y, b.max_y)
                    && inside(p.z, b.min_z, b.max_z),
                "sample {p:?} escapes {b:?}"
            );
        }
        // The ceiling: each axis's span is exactly the amplitude
        // channel sum, to the bracket arithmetic's few ulps.
        let (f_min, f_max) = super::spiric_f_range(big_r, r, d);
        let m = axis.cross(u_ref);
        for (name, lo, hi, me, ae) in [
            ("x", b.min_x, b.max_x, m.x, axis.x),
            ("y", b.min_y, b.max_y, m.y, axis.y),
            ("z", b.min_z, b.max_z, m.z, axis.z),
        ] {
            let want = me.abs() * (f_max - f_min) + 2.0 * r * ae.abs();
            assert!(
                (hi - lo - want).abs() <= 1e-12,
                "{name}: box span {} vs the amplitude construction {want}",
                hi - lo
            );
        }
    }

    mod spiric_interval {
        use geom_core::{Bounds, Interval};

        use super::*;

        /// `eval` at a bracketed `v` encloses the f64 point, and the
        /// plane and torus residual enclosures straddle zero.
        #[test]
        fn spiric_eval_encloses_the_f64_point_and_its_residuals_straddle_zero() {
            let c = tilted_spiric();
            let (center, axis, u_ref, big_r, r, d) = spiric_fields(&c);
            let ci = c.map_scalar(Interval::from_f64);
            let (ci_center, ci_axis, ci_u_ref) = (
                center.map(Interval::from_f64),
                axis.map(Interval::from_f64),
                u_ref.map(Interval::from_f64),
            );
            for v in [0.0, 0.7, 2.9, -14.6, 300.0, core::f64::consts::PI] {
                let p = c.eval(v);
                let pi = ci.eval(Interval::from_f64(v));
                for (x, xi) in [(p.x, pi.x), (p.y, pi.y), (p.z, pi.z)] {
                    assert!(xi.lo() <= x && x <= xi.hi(), "enclosure {xi:?} misses {x}");
                }
                let w = pi - ci_center;
                let plane = w.dot(ci_u_ref) - Interval::from_f64(d);
                assert!(plane.lo() <= 0.0 && 0.0 <= plane.hi(), "plane {plane:?}");
                let h = w.dot(ci_axis);
                let rho = (w - ci_axis * h).norm();
                let torus = ((rho - Interval::from_f64(big_r)) * (rho - Interval::from_f64(big_r))
                    + h * h)
                    .sqrt()
                    - Interval::from_f64(r);
                assert!(torus.lo() <= 0.0 && 0.0 <= torus.hi(), "torus {torus:?}");
            }
        }

        /// The box door at the interval scalar, on POINT brackets: the
        /// lifted fixture's brackets are single f64 values, so this
        /// box is bit-for-bit the f64 twin's (`Brk` reads `lo()`/`hi()`
        /// and both are the point) — what this row checks is that the
        /// `Interval` scalar passes the door and the result contains
        /// every f64 sample. It cannot see an inward-turned bracket;
        /// the wide-bracket row below is that reader.
        #[test]
        fn spiric_arc_aabb_contains_every_sample_at_interval() {
            let c = tilted_spiric();
            let ci = c.map_scalar(Interval::from_f64);
            let (t0, t1) = (Interval::from_f64(0.4), Interval::from_f64(2.9));
            let b = boxes::conic_arc_aabb(&ci, t0, t1, ci.eval(t0), ci.eval(t1)).expect("boxes");
            for i in 0..10_000 {
                let v = f64::from(i) * TAU / 10_000.0;
                let p = c.eval(v);
                assert!(
                    b.min_x <= p.x
                        && p.x <= b.max_x
                        && b.min_y <= p.y
                        && p.y <= b.max_y
                        && b.min_z <= p.z
                        && p.z <= b.max_z,
                    "sample {p:?} escapes {b:?}"
                );
            }
        }

        /// **A WIDE bracket must box every member.** A carrier whose
        /// `offset` is the bracket `[0.05, 0.06]` describes a family of
        /// spirics; the f64 twins at the bracket's two ends sample the
        /// `m`-faces of the box, so a `Brk` turned inward
        /// (`[f_min.hi, f_max.lo]`) lets them escape by
        /// `≈ 5.65e-4` — the reader the point-bracket row above cannot
        /// be. Adopted from the v6 dual's second reviewer lane
        /// (`sp1a_wide_bracket_box_contains_every_member_twin`),
        /// authorship preserved.
        #[test]
        fn spiric_wide_bracket_box_contains_every_member_twin() {
            let iv = Interval::from_f64;
            let ci = Curve3::Spiric {
                center: Point3::new(iv(0.0), iv(0.0), iv(0.0)),
                axis: Vec3::new(iv(0.0), iv(0.0), iv(1.0)),
                u_ref: Vec3::new(iv(1.0), iv(0.0), iv(0.0)),
                major_radius: iv(1.2),
                minor_radius: iv(0.225),
                offset: Interval::from_bounds(0.05, 0.06),
            };
            let (t0, t1) = (iv(0.0), iv(TAU));
            let b = boxes::conic_arc_aabb(&ci, t0, t1, ci.eval(t0), ci.eval(t1)).expect("boxes");
            let mut worst = f64::NEG_INFINITY;
            for d in [0.05, 0.06] {
                let twin = Curve3::Spiric {
                    center: Point3::new(0.0, 0.0, 0.0),
                    axis: Vec3::new(0.0, 0.0, 1.0),
                    u_ref: Vec3::new(1.0, 0.0, 0.0),
                    major_radius: 1.2,
                    minor_radius: 0.225,
                    offset: d,
                };
                for i in 0..10_000 {
                    let v = f64::from(i) * TAU / 10_000.0;
                    let p = twin.eval(v);
                    for (x, lo, hi) in [
                        (p.x, b.min_x, b.max_x),
                        (p.y, b.min_y, b.max_y),
                        (p.z, b.min_z, b.max_z),
                    ] {
                        worst = worst.max(lo - x).max(x - hi);
                    }
                }
            }
            assert!(
                worst <= 0.0,
                "a member twin escapes the wide-bracket box by {worst:e}"
            );
        }
    }

    mod ellipse_interval {
        use geom_core::{Bounds, Interval};

        use super::*;

        /// Truth containment through identities at the interval scalar:
        /// the frame quadratic and planarity residuals enclose zero.
        #[test]
        fn ellipse_residuals_enclose_zero() {
            let e = super::tilted_ellipse();
            let ei = e.map_scalar(geom_core::Interval::from_f64);
            let Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } = ei
            else {
                panic!("fixture is an ellipse");
            };
            let v_ref = axis.cross(u_ref);
            for theta in [0.0, 0.7, 2.9, -14.6, 300.0] {
                let p = ei.eval(Interval::from_f64(theta));
                let d = p - center;
                let (x, y) = (d.dot(u_ref), d.dot(v_ref));
                let quad = (x / major) * (x / major) + (y / minor) * (y / minor) - Interval::one();
                assert!(
                    quad.lo() <= 0.0 && 0.0 <= quad.hi(),
                    "θ = {theta}: quadratic [{}, {}]",
                    quad.lo(),
                    quad.hi()
                );
                assert!(quad.hi() - quad.lo() < 1e-12);
                let plane_res = d.dot(axis);
                assert!(plane_res.lo() <= 0.0 && 0.0 <= plane_res.hi());
            }
        }
    }

    // ------------------------------------------------------------------
    // Totality and poison
    // ------------------------------------------------------------------

    #[test]
    fn nurbs_placeholder_evaluates_to_poison() {
        let n: Curve3<f64> = Curve3::nurbs_placeholder();
        let p = n.eval(0.5);
        assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan());
        let d = n.deriv(0.5);
        assert!(d.x.is_nan() && d.y.is_nan() && d.z.is_nan());
        let d2 = n.deriv2(0.5);
        assert!(d2.x.is_nan() && d2.y.is_nan() && d2.z.is_nan());
        let (jp, jd) = n.ders1(0.5);
        assert!(jp.x.is_nan() && jp.y.is_nan() && jp.z.is_nan());
        assert!(jd.x.is_nan() && jd.y.is_nan() && jd.z.is_nan());
    }

    /// A described NURBS fixture: the rational quadratic quarter circle
    /// of radius 2 about the origin in the xy-plane (weights
    /// `[1, √2/2, 1]`), so the payload is rational and non-trivial.
    fn quarter_circle_nurbs() -> Curve3<f64> {
        let knots =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let control = vec![
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(2.0, 2.0, 0.0),
            Point3::new(0.0, 2.0, 0.0),
        ];
        let weights = vec![1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0];
        Curve3::Nurbs(Arc::new(NurbsCurve3::new(knots, control, weights).unwrap()))
    }

    /// A described NURBS lifts as its PAYLOAD, never as the placeholder:
    /// the lifted curve's value channel is the source's evaluation bit
    /// for bit, and its tangent channel is the source's closed-form
    /// derivative to rounding.
    #[test]
    fn described_nurbs_lifts_as_its_payload_at_dual() {
        let c = quarter_circle_nurbs();
        let cd: Curve3<Dual64> = c.map_scalar(Dual::constant);
        for t in [0.0, 0.3, 0.5, 0.75, 1.0] {
            let p = cd.eval(Dual::variable(t));
            let q = c.eval(t);
            let d = c.deriv(t);
            for (name, lifted, source, tangent) in [
                ("x", p.x, q.x, d.x),
                ("y", p.y, q.y, d.y),
                ("z", p.z, q.z, d.z),
            ] {
                assert_eq!(
                    lifted.value.to_bits(),
                    source.to_bits(),
                    "t = {t}: lifted {name} = {} vs source {source}",
                    lifted.value
                );
                assert!(
                    (lifted.deriv - tangent).abs() <= 1e-12 * (1.0 + tangent.abs()),
                    "t = {t}: lifted d{name} = {} vs source {tangent}",
                    lifted.deriv
                );
            }
        }
        assert!(
            matches!(&cd, Curve3::Nurbs(n) if !n.is_placeholder()),
            "a described NURBS lifted to the placeholder"
        );
    }

    /// A described NURBS with the structure the quarter circle lacks:
    /// degree 4, interior knots at multiplicities 1, 2 and 3 (= p − 1),
    /// weights spanning twelve orders of magnitude.
    fn knotted_curve() -> Curve3<f64> {
        let knots = geom_core::spline::KnotVector::clamped(
            vec![
                0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.5, 0.5, 0.8, 0.8, 0.8, 1.0, 1.0, 1.0, 1.0, 1.0,
            ],
            4,
        )
        .unwrap();
        let control: Vec<Point3<f64>> = (0..11)
            .map(|i| {
                let x = i as f64;
                Point3::new(x * 0.5 - 2.0, (x * 0.8).sin() * 1.5, (x * 0.3).cos())
            })
            .collect();
        let weights = vec![1.0, 1e6, 1e-6, 3.0, 2e5, 5e-5, 1.0, 1e4, 1e-4, 7.0, 1.0];
        Curve3::Nurbs(Arc::new(NurbsCurve3::new(knots, control, weights).unwrap()))
    }

    /// Every knot value (so every span boundary) and every nonempty
    /// span's midpoint of a NURBS curve.
    fn knot_and_span_params(c: &Curve3<f64>) -> Vec<f64> {
        let Curve3::Nurbs(n) = c else {
            panic!("fixture is a NURBS");
        };
        let knots = n.knots().knots();
        let mut ps: Vec<f64> = knots.to_vec();
        ps.extend(
            knots
                .windows(2)
                .filter(|w| w[1] > w[0])
                .map(|w| 0.5 * (w[0] + w[1])),
        );
        ps.sort_by(f64::total_cmp);
        ps.dedup();
        ps
    }

    /// The lift carries the STRUCTURE verbatim — every knot, the degree,
    /// every weight — and the lifted curve evaluates to the source at
    /// every knot value, span boundary and span midpoint. A lift that
    /// perturbs an interior knot or a weight is red here and nowhere in
    /// the placeholder-vs-payload rows above.
    #[test]
    fn knotted_nurbs_lift_carries_structure_verbatim_at_dual() {
        let c = knotted_curve();
        let Curve3::Nurbs(source) = &c else {
            panic!("fixture is a NURBS");
        };
        let cd: Curve3<Dual64> = c.map_scalar(Dual::constant);
        let Curve3::Nurbs(lifted) = &cd else {
            panic!("a described NURBS lifted to another variant");
        };
        assert_eq!(
            lifted.knots(),
            source.knots(),
            "knots must be carried verbatim"
        );
        assert_eq!(
            lifted.weights(),
            source.weights(),
            "weights must be carried verbatim"
        );
        for t in knot_and_span_params(&c) {
            let p = cd.eval(Dual::variable(t));
            let q = c.eval(t);
            let d = c.deriv(t);
            for (name, lifted, source, tangent) in [
                ("x", p.x, q.x, d.x),
                ("y", p.y, q.y, d.y),
                ("z", p.z, q.z, d.z),
            ] {
                assert_eq!(
                    lifted.value.to_bits(),
                    source.to_bits(),
                    "t = {t}: lifted {name} = {} vs source {source}",
                    lifted.value
                );
                assert!(
                    (lifted.deriv - tangent).abs() <= 1e-9 * (1.0 + tangent.abs()),
                    "t = {t}: lifted d{name} = {} vs source {tangent}",
                    lifted.deriv
                );
            }
        }
    }

    /// `ders1_in_span` is `eval_in_span` and `deriv_in_span` bit for
    /// bit — one order-1 pass answering both — on the knotted fixture
    /// at every knot value, span boundary and span midpoint.
    #[test]
    fn ders1_in_span_is_eval_and_deriv_bit_for_bit() {
        let c = knotted_curve();
        let Curve3::Nurbs(n) = &c else {
            panic!("fixture is a NURBS");
        };
        for t in knot_and_span_params(&c) {
            let span = n.span_at(t);
            let (p, d) = span.ders1_in_span(t);
            let q = span.eval_in_span(t);
            let e = span.deriv_in_span(t);
            for (name, a, b) in [
                ("x", p.x, q.x),
                ("y", p.y, q.y),
                ("z", p.z, q.z),
                ("dx", d.x, e.x),
                ("dy", d.y, e.y),
                ("dz", d.z, e.z),
            ] {
                assert_eq!(a.to_bits(), b.to_bits(), "t = {t}: {name} {a} vs {b}");
            }
        }
    }

    /// `ders1` is `eval` and `deriv` bit for bit at the whole-curve
    /// level — one span selection and one order-1 pass answering both
    /// — on the knotted fixture at every knot value, span boundary and
    /// span midpoint. A differential, not a digest: a one-ulp move in
    /// either half against its own evaluator reds it by name.
    #[test]
    fn ders1_is_eval_and_deriv_bit_for_bit() {
        let c = knotted_curve();
        for t in knot_and_span_params(&c) {
            let (p, d) = c.ders1(t);
            let q = c.eval(t);
            let e = c.deriv(t);
            for (name, a, b) in [
                ("x", p.x, q.x),
                ("y", p.y, q.y),
                ("z", p.z, q.z),
                ("dx", d.x, e.x),
                ("dy", d.y, e.y),
                ("dz", d.z, e.z),
            ] {
                assert_eq!(a.to_bits(), b.to_bits(), "t = {t}: {name} {a} vs {b}");
            }
        }
    }

    /// The analytic arms' `ders1` is their `eval` and `deriv` bit for
    /// bit: the line's closed form, the circle's one azimuthal frame
    /// against the two frames the pair builds, the ellipse's one
    /// `sin_cos` against the pair's two, the spiric's one `sin_cos` and
    /// one `sqrt` against the pair's two of each — on the tilted
    /// fixtures, at parameters that are not special to any of them.
    #[test]
    fn analytic_ders1_is_eval_and_deriv_bit_for_bit() {
        let line = Curve3::Line {
            origin: Point3::new(1.0, -2.0, 0.5),
            dir: Vec3::new(0.3, -0.4, 1.2),
        };
        for (kind, c) in [
            ("line", line),
            ("circle", tilted_circle()),
            ("ellipse", tilted_ellipse()),
            ("spiric", tilted_spiric()),
        ] {
            for t in [-7.3, -1.0, 0.0, 0.37, 1.0, FRAC_PI_2, 2.9, TAU, 41.5] {
                let (p, d) = c.ders1(t);
                let q = c.eval(t);
                let e = c.deriv(t);
                for (name, a, b) in [
                    ("x", p.x, q.x),
                    ("y", p.y, q.y),
                    ("z", p.z, q.z),
                    ("dx", d.x, e.x),
                    ("dy", d.y, e.y),
                    ("dz", d.z, e.z),
                ] {
                    assert_eq!(
                        a.to_bits(),
                        b.to_bits(),
                        "{kind} at t = {t}: {name} {a} vs {b}"
                    );
                }
            }
        }
    }

    /// `ders1_in_span` is `ders_in_span`'s first two components bit for
    /// bit — the order-1 pass and the order-2 pass agree on everything
    /// the order-2 pass does not need its third row for — on the
    /// knotted fixture at every knot value, span boundary and span
    /// midpoint.
    ///
    /// This is what licenses a consumer that binds `C″` to `_` to drop
    /// to the order-1 door. It does NOT follow from
    /// [`ders1_in_span_is_eval_and_deriv_bit_for_bit`]: that row pins
    /// the order-1 door against the two single-value doors, and the
    /// order-2 door's own doc claims only its arithmetic, not agreement
    /// with a shorter pass.
    #[test]
    fn ders1_in_span_is_ders_in_spans_first_two_bit_for_bit() {
        let c = knotted_curve();
        let Curve3::Nurbs(n) = &c else {
            panic!("fixture is a NURBS");
        };
        for t in knot_and_span_params(&c) {
            let span = n.span_at(t);
            let (p, d) = span.ders1_in_span(t);
            let (q, e, _) = span.ders_in_span(t);
            for (name, a, b) in [
                ("x", p.x, q.x),
                ("y", p.y, q.y),
                ("z", p.z, q.z),
                ("dx", d.x, e.x),
                ("dy", d.y, e.y),
                ("dz", d.z, e.z),
            ] {
                assert_eq!(a.to_bits(), b.to_bits(), "t = {t}: {name} {a} vs {b}");
            }
        }
    }

    #[test]
    fn poison_parameter_poisons_the_point() {
        let c = xy_circle(2.0);
        let p = c.eval(f64::NAN);
        assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan());
        let d = c.deriv(f64::NAN);
        assert!(d.x.is_nan() && d.y.is_nan() && d.z.is_nan());
        let (jp, jd) = c.ders1(f64::NAN);
        assert!(jp.x.is_nan() && jp.y.is_nan() && jp.z.is_nan());
        assert!(jd.x.is_nan() && jd.y.is_nan() && jd.z.is_nan());
        let line = Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::unit_x(),
        };
        let lp = line.eval(f64::NAN);
        assert!(lp.x.is_nan() && lp.y.is_nan() && lp.z.is_nan());
        // The line's deriv is parameter-independent — NaN t does not
        // poison it (there is nothing to poison: the tangent is data).
        assert_eq!(line.deriv(f64::NAN).x, 1.0);
        assert_eq!(line.ders1(f64::NAN).1.x, 1.0);
    }

    #[test]
    fn extreme_parameters_do_not_panic() {
        let c = xy_circle(2.0);
        for t in [f64::INFINITY, f64::NEG_INFINITY, 1e300, -1e300, f64::MAX] {
            // sin_cos(±∞) is NaN (poison), huge finite values evaluate;
            // either way: total, no panic.
            let _ = c.eval(t);
            let _ = c.deriv(t);
            let _ = c.deriv2(t);
            let _ = c.ders1(t);
        }
        // ±∞ specifically poisons through sin_cos — every channel of
        // the point, not the first one.
        let p = c.eval(f64::INFINITY);
        assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan());
        let (jp, jd) = c.ders1(f64::INFINITY);
        assert!(jp.x.is_nan() && jp.y.is_nan() && jp.z.is_nan());
        assert!(jd.x.is_nan() && jd.y.is_nan() && jd.z.is_nan());
    }

    // ------------------------------------------------------------------
    // Interval instantiation
    // ------------------------------------------------------------------

    mod interval {
        use geom_core::{Bounds, Interval};

        use super::*;

        fn contains(enclosure: Interval, x: f64) -> bool {
            enclosure.lo() <= x && x <= enclosure.hi()
        }

        /// Truth containment via residuals (the module rule in
        /// geom-core's interval.rs: transcendental results are tested
        /// through identities, not f64-value containment): at interval
        /// type, |P − center|² − r² and (P − center)·axis both enclose 0.
        #[test]
        fn circle_residuals_enclose_zero() {
            let c = super::tilted_circle();
            let ci = c.map_scalar(Interval::from_f64);
            let (center, axis, r) = match ci {
                Curve3::Circle {
                    center,
                    axis,
                    radius,
                    ..
                } => (center, axis, radius),
                _ => panic!("fixture is a circle"),
            };
            for theta in [0.0, 0.7, 2.9, -14.6, 300.0] {
                let p = ci.eval(Interval::from_f64(theta));
                let radial = p - center;
                let dist_res = radial.norm_squared() - r.powi(2);
                assert!(
                    contains(dist_res, 0.0),
                    "θ = {theta}: |P − c|² − r² = [{}, {}]",
                    dist_res.lo(),
                    dist_res.hi()
                );
                assert!(dist_res.hi() - dist_res.lo() < 1e-12);
                let plane_res = radial.dot(axis);
                assert!(contains(plane_res, 0.0), "θ = {theta}: planarity");
                assert!(plane_res.hi() - plane_res.lo() < 1e-13);
            }
        }

        /// The line evaluator is exact-ops only (+, ·), so the f64
        /// evaluation IS contained in the interval evaluation — the
        /// assertable form of enclosure containment for this variant.
        #[test]
        fn line_encloses_f64_evaluation() {
            let line = Curve3::Line {
                origin: Point3::new(1.0, -2.0, 0.5),
                dir: Vec3::new(3.0 / 13.0, 4.0 / 13.0, 12.0 / 13.0),
            };
            let li = line.map_scalar(Interval::from_f64);
            for t in [0.0, 1.75, -3.5e2, 1234.5678] {
                let p = line.eval(t);
                let pi = li.eval(Interval::from_f64(t));
                assert!(contains(pi.x, p.x) && contains(pi.y, p.y) && contains(pi.z, p.z));
            }
        }

        /// The containment form of periodicity: evaluating over
        /// θ + k·tau() (the τ *enclosure*) yields an enclosure
        /// containing the true point, which equals the true point at θ —
        /// so the θ-evaluation and the shifted evaluation must overlap.
        #[test]
        fn circle_periodicity_containment_form() {
            let ci = super::tilted_circle().map_scalar(Interval::from_f64);
            let theta = Interval::from_f64(0.7);
            let k = Interval::from_f64(3.0);
            let p = ci.eval(theta);
            let q = ci.eval(theta + Interval::tau() * k);
            for (a, b) in [(p.x, q.x), (p.y, q.y), (p.z, q.z)] {
                assert!(
                    a.lo() <= b.hi() && b.lo() <= a.hi(),
                    "enclosures [{}, {}] and [{}, {}] must intersect \
                     (both contain the same true point)",
                    a.lo(),
                    a.hi(),
                    b.lo(),
                    b.hi()
                );
            }
        }

        /// NaI in → NaI out (surfaced as NaN brackets through Bounds),
        /// and the Nurbs placeholder poisons at interval type too.
        #[test]
        fn poison_propagates_at_interval() {
            let ci = super::xy_circle(2.0).map_scalar(Interval::from_f64);
            let p = ci.eval(Interval::from_f64(f64::NAN));
            assert!(p.x.lo().is_nan() && p.y.lo().is_nan() && p.z.lo().is_nan());
            let n: Curve3<Interval> = Curve3::nurbs_placeholder();
            // All-poison, not first-channel-poison.
            let q = n.eval(Interval::zero());
            assert!(q.x.is_poison() && q.y.is_poison() && q.z.is_poison());
        }

        /// The interval half of the payload-lift row: a described NURBS
        /// lifts as its payload, and the lifted enclosure brackets the
        /// source's f64 evaluation at every sampled parameter.
        #[test]
        fn described_nurbs_lifts_as_its_payload_at_interval() {
            let c = super::quarter_circle_nurbs();
            let ci = c.map_scalar(Interval::from_f64);
            for t in [0.0, 0.3, 0.5, 0.75, 1.0] {
                let p = ci.eval(Interval::from_f64(t));
                let q = c.eval(t);
                for (name, enclosure, source) in [("x", p.x, q.x), ("y", p.y, q.y), ("z", p.z, q.z)]
                {
                    assert!(
                        contains(enclosure, source),
                        "t = {t}: lifted {name} = [{}, {}] must contain source {source}",
                        enclosure.lo(),
                        enclosure.hi()
                    );
                    assert!(
                        enclosure.hi() - enclosure.lo() < 1e-12,
                        "t = {t}: {name} too wide"
                    );
                }
            }
            assert!(
                matches!(&ci, Curve3::Nurbs(n) if !n.is_placeholder()),
                "a described NURBS lifted to the placeholder"
            );
        }

        /// The interval half of the structure row: knots and weights
        /// verbatim, and the lifted enclosure brackets the source at
        /// every knot value, span boundary and span midpoint. The width
        /// bound is a FIXTURE PIN (this net, these weights), not a
        /// degradation guard: what it measures is the evaluator's
        /// cancellation under extreme weights, not any width the lift
        /// adds (the lift adds none — every bracket is a point).
        #[test]
        fn knotted_nurbs_lift_carries_structure_verbatim_at_interval() {
            let c = super::knotted_curve();
            let Curve3::Nurbs(source) = &c else {
                panic!("fixture is a NURBS");
            };
            let ci = c.map_scalar(Interval::from_f64);
            let Curve3::Nurbs(lifted) = &ci else {
                panic!("a described NURBS lifted to another variant");
            };
            assert_eq!(
                lifted.knots(),
                source.knots(),
                "knots must be carried verbatim"
            );
            assert_eq!(
                lifted.weights(),
                source.weights(),
                "weights must be carried verbatim"
            );
            for t in super::knot_and_span_params(&c) {
                let p = ci.eval(Interval::from_f64(t));
                let q = c.eval(t);
                for (name, enclosure, source) in [("x", p.x, q.x), ("y", p.y, q.y), ("z", p.z, q.z)]
                {
                    assert!(
                        contains(enclosure, source),
                        "t = {t}: lifted {name} = [{}, {}] must contain source {source}",
                        enclosure.lo(),
                        enclosure.hi()
                    );
                    assert!(
                        enclosure.hi() - enclosure.lo() <= 1e-8 * (1.0 + source.abs()),
                        "t = {t}: {name} width {} (fixture pin)",
                        enclosure.hi() - enclosure.lo()
                    );
                }
            }
        }

        /// `Dual<Interval>` instantiates cleanly and its derivative
        /// enclosure intersects the closed-form derivative enclosure
        /// (both bracket the true tangent).
        #[test]
        fn dual_interval_instantiates() {
            use geom_core::DualInterval;
            let c = super::tilted_circle();
            // Lift f64 → Interval → Dual<Interval>, constants throughout
            // except the evaluation parameter.
            let cd: Curve3<DualInterval> =
                c.map_scalar(Interval::from_f64).map_scalar(Dual::constant);
            let p = cd.eval(Dual::variable(Interval::from_f64(0.7)));
            let ci = c.map_scalar(Interval::from_f64);
            let d = ci.deriv(Interval::from_f64(0.7));
            for (dual_ch, closed) in [(p.x.deriv, d.x), (p.y.deriv, d.y), (p.z.deriv, d.z)] {
                assert!(
                    dual_ch.lo() <= closed.hi() && closed.lo() <= dual_ch.hi(),
                    "derivative enclosures [{}, {}] and [{}, {}] must intersect",
                    dual_ch.lo(),
                    dual_ch.hi(),
                    closed.lo(),
                    closed.hi()
                );
            }
        }
    }
}

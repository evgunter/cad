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
pub use nurbs::{NurbsCurve2, NurbsCurve3};
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

impl<T: Real> Curve3<T> {
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
            } => {
                let (radial, _) = azimuth::frame(*axis, *u_ref, t);
                *center + radial * *radius
            }
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
    /// - Nurbs: the payload’s derivative (all-poison for the placeholder).
    pub fn deriv(&self, t: T) -> Vec3<T> {
        match self {
            Curve3::Line { dir, .. } => *dir,
            Curve3::Circle {
                axis,
                radius,
                u_ref,
                ..
            } => {
                let (_, tangential) = azimuth::frame(*axis, *u_ref, t);
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
            Curve3::Nurbs(n) => n.deriv(t),
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
    ///   already had.)
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
    /// `sweep::fillet::surgery::seam_split_param` is that caller and
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
                let r_near = self.eval(near) - *center;
                let tau_near = self.deriv(near);
                Some(near + w.dot(tau_near).atan2(w.dot(r_near)))
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

    #[cfg(feature = "interval")]
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
            let span = n.knots().span_at(t);
            let (p, d) = n.ders1_in_span(span, t);
            let q = n.eval_in_span(span, t);
            let e = n.deriv_in_span(span, t);
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
        let line = Curve3::Line {
            origin: Point3::origin(),
            dir: Vec3::unit_x(),
        };
        let lp = line.eval(f64::NAN);
        assert!(lp.x.is_nan() && lp.y.is_nan() && lp.z.is_nan());
        // The line's deriv is parameter-independent — NaN t does not
        // poison it (there is nothing to poison: the tangent is data).
        assert_eq!(line.deriv(f64::NAN).x, 1.0);
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
        }
        // ±∞ specifically poisons through sin_cos — every channel of
        // the point, not the first one.
        let p = c.eval(f64::INFINITY);
        assert!(p.x.is_nan() && p.y.is_nan() && p.z.is_nan());
    }

    // ------------------------------------------------------------------
    // Interval instantiation (feature-gated)
    // ------------------------------------------------------------------

    #[cfg(feature = "interval")]
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

//! Analytic 3-D curves: the [`Curve3`] closed enum and its evaluators
//! (M2 PR 1).
//!
//! Curve kinds form a **closed enum** per D3 (`docs/DESIGN.md`):
//! intersection and classification need pairwise dispatch, and a closed
//! enum makes every dispatch site exhaustively checked at compile time.
//! The [`Curve3::Nurbs`] variant is the universal-fallback placeholder —
//! representable now, implemented at M5.
//!
//! # Curve conventions (normative, stated once)
//!
//! These are the parameterization conventions every consumer — edges,
//! pcurves, tessellation — derives from. They are conventions in D2's
//! sense: data like `u_ref` *carries* a convention (where θ = 0 lives)
//! and is therefore never recomputable from the locus alone.
//!
//! - **Units (D6):** lengths in meters, angles in radians. A line's
//!   parameter is arc length in meters (unit `dir`); a circle's
//!   parameter is the angle in radians.
//! - **Curve entities are complete loci.** A [`Curve3`] is the *whole*
//!   infinite line or full circle — never a segment or arc. An edge
//!   bounds a carrier curve by its **vertices**: the parameter interval
//!   is derived from the vertex positions, not stored (the
//!   representation-consistency lesson of D2 applied to bounds; also why
//!   profile carriers are split so no edge spans a full period —
//!   M2-PLAN PR 2).
//! - **The `he_plus` forward contract (D1, ratified at M1):** an edge's
//!   intrinsic direction is its plus half-edge, and the curve geometry
//!   MUST agree — **increasing curve parameter runs from
//!   `start(he_plus)` to `end(he_plus)`**. Per-face traversal senses and
//!   pcurves are *derived* from that one orientation, never stored as
//!   peers.
//! - **Periodicity:** a circle is 2π-periodic in θ: as a locus,
//!   `P(θ) = P(θ + 2πk)` exactly, in the reals. Evaluators do **not**
//!   range-reduce — `sin_cos` is total on ℝ and evaluation is the same
//!   fixed formula at every θ (no comparison, no seam special-case).
//!   Consumers that need a canonical representative reduce explicitly
//!   with [`geom_core::Real::reduce_periodic`] (θ mod 2π, seam blur
//!   documented there).
//! - **The bit-identity policy for periodicity, stated honestly:**
//!   floating-point evaluation promises **no** bit-level periodicity.
//!   2π is not representable, so `θ + k·fl(τ)` is a *different real
//!   parameter* than `θ + 2πk`; evaluations at the two f64 parameters
//!   agree to rounding (ulps scaled by `k` and by `r`), never bitwise —
//!   and range reduction itself rounds, so reduced evaluation is also
//!   value-close, not bit-identical. What IS promised: evaluation is a
//!   pure function (same input bits → same output bits, D9), and at the
//!   interval scalar the enclosure of an evaluation contains the true
//!   image of every parameter in the input enclosure — the containment
//!   form of periodicity (evaluate over `θ + k·Real::tau()` at interval
//!   type and the true periodic image is enclosed).
//! - **Unit-vector fields are conventional data, unchecked here.**
//!   `dir`, `axis`, `u_ref` are unit by convention, `u_ref ⊥ axis` by
//!   convention. Constructors do not renormalize (a hidden normalize
//!   would silently reparameterize; D6's meters-per-parameter contract
//!   is the caller's to establish) and evaluators consume the fields as
//!   given. Tier-3 geometric validation (M2 PR 3+) certifies the
//!   invariants at rest; violating them yields well-defined garbage
//!   (a non-arc-length parameterization, an elliptical "circle"), not
//!   poison and not a panic.
//!
//! # Totality and poison (geom-core's policy, inherited)
//!
//! Every evaluator is **total**: no panic, no `Result`. Out-of-domain
//! and unimplemented cases produce the scalar's poison value (NaN at
//! `f64`, NaI/empty at the interval scalar) which flows through values
//! and is caught at the predicate/certification layer — in particular,
//! evaluating the [`Curve3::Nurbs`] placeholder yields an all-poison
//! point (representable ≠ implemented; the poison fails every
//! downstream certification loudly, per D4 ¶2).
//!
//! # Evaluation-code discipline
//!
//! All evaluation code here is generic over [`Real`] with no extra
//! bounds, comparison-free, uses `sin_cos` as the trig primitive, no
//! fused operations, and fixed documented association orders (D9). It
//! instantiates at `f64`, `Dual<f64>`, `Interval`, and `Dual<Interval>`
//! — the derivative-vs-dual consistency and enclosure-containment test
//! axes below rely on exactly that.

use geom_core::{Point3, Real, Vec3};

/// An analytic 3-D curve — a **complete locus** (see the crate docs for
/// the conventions: units, periodicity, the `he_plus` forward contract,
/// and the conventional unit-field invariants).
///
/// Fields are public data (D2: conventions are carried by data);
/// construction is by struct-literal variant syntax.
#[derive(Clone, Copy, Debug)]
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

    /// Placeholder for the NURBS fallback (D3: representable from day
    /// one so the closed enum's dispatch sites are complete; implemented
    /// at M5). Evaluating it yields poison — see the crate docs'
    /// totality policy.
    Nurbs,
}

/// The all-poison point (every coordinate the scalar's poison value) —
/// the totality outcome for the unimplemented [`Curve3::Nurbs`]
/// placeholder. `from_f64(NaN)` maps to each scalar's own poison: NaN at
/// `f64`, NaI at the interval scalar, a poisoned value channel at duals.
fn poison_point<T: Real>() -> Point3<T> {
    let nan = T::from_f64(f64::NAN);
    Point3::new(nan, nan, nan)
}

/// The all-poison vector — [`poison_point`]'s tangent-side counterpart.
fn poison_vec<T: Real>() -> Vec3<T> {
    let nan = T::from_f64(f64::NAN);
    Vec3::new(nan, nan, nan)
}

impl<T: Real> Curve3<T> {
    /// The point at parameter `t` (see the variant docs for each
    /// parameterization; the crate docs for units and periodicity).
    ///
    /// Evaluation orders (fixed, D9):
    /// - Line: `origin + dir·t` — one componentwise scale, one add.
    /// - Circle: `(s, c) = θ.sin_cos()`; `radial = u_ref·c + v_ref·s`
    ///   with `v_ref = axis × u_ref` (the cross's own fixed order);
    ///   result `center + radial·radius` — exactly as parenthesized.
    /// - Nurbs: the all-poison point (unimplemented placeholder).
    pub fn eval(&self, t: T) -> Point3<T> {
        match *self {
            Curve3::Line { origin, dir } => origin + dir * t,
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => {
                let (s, c) = t.sin_cos();
                let v_ref = axis.cross(u_ref);
                center + (u_ref * c + v_ref * s) * radius
            }
            Curve3::Nurbs => poison_point(),
        }
    }

    /// The first derivative `dP/dt` at parameter `t`.
    ///
    /// - Line: `dir`, constant (arc-length parameterization for unit
    ///   `dir`).
    /// - Circle: the tangent `(u_ref·(−s) + v_ref·c)·radius`, evaluated
    ///   exactly as written from one `sin_cos` call (fixed order;
    ///   `|dP/dθ| = radius`, the radians-to-meters rate).
    /// - Nurbs: all-poison.
    pub fn deriv(&self, t: T) -> Vec3<T> {
        match *self {
            Curve3::Line { dir, .. } => dir,
            Curve3::Circle {
                axis,
                radius,
                u_ref,
                ..
            } => {
                let (s, c) = t.sin_cos();
                let v_ref = axis.cross(u_ref);
                (u_ref * (-s) + v_ref * c) * radius
            }
            Curve3::Nurbs => poison_vec(),
        }
    }

    /// The second derivative `d²P/dt²` at parameter `t` — cheap for both
    /// analytic kinds and needed later for curvature (M2-PLAN PR 1).
    ///
    /// - Line: the zero vector, exactly.
    /// - Circle: `(u_ref·(−c) + v_ref·(−s))·radius` (the inward radial,
    ///   scaled; fixed order as written).
    /// - Nurbs: all-poison.
    pub fn deriv2(&self, t: T) -> Vec3<T> {
        match *self {
            Curve3::Line { .. } => Vec3::zero(),
            Curve3::Circle {
                axis,
                radius,
                u_ref,
                ..
            } => {
                let (s, c) = t.sin_cos();
                let v_ref = axis.cross(u_ref);
                (u_ref * (-c) + v_ref * (-s)) * radius
            }
            Curve3::Nurbs => poison_vec(),
        }
    }
}

//! Analytic 3-D curves: the [`Curve3`] closed enum and its evaluators
//! (M2 PR 1).
//!
//! Curve kinds form a **closed enum** per D3 (`docs/DESIGN.md`):
//! intersection and classification need pairwise dispatch, and a closed
//! enum makes every dispatch site exhaustively checked at compile time.
//! The [`Curve3::Nurbs`] variant is the universal fallback — since M5
//! PR 3 it carries a validated [`NurbsCurve3`] payload (see [`nurbs`])
//! and its evaluator arms are real; the "no description yet" state is
//! [`Curve3::nurbs_placeholder`].
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
//! and undescribed cases produce the scalar's poison value (NaN at
//! `f64`, NaI/empty at the interval scalar) which flows through values
//! and is caught at the predicate/certification layer — in particular,
//! evaluating the [`Curve3::nurbs_placeholder`] "no description yet"
//! state yields an all-poison point (representable ≠ described; the
//! poison fails every downstream certification loudly, per D4 ¶2).
//!
//! # Evaluation-code discipline
//!
//! All evaluation *arithmetic* here is comparison-free ring/trig code:
//! `sin_cos` is the trig primitive, no fused operations, fixed
//! documented association orders (D9). Since M5 PR 3 the enum
//! evaluators are bounded by [`geom_core::spline::SpanLocate`] (as the
//! **sole bound** — a sealed `Real` subtrait, the same style rule as
//! `Bounds`): NURBS evaluation needs per-instantiation knot-span
//! *selection*, a structure decision the seam localizes per scalar
//! (its module docs carry each instantiation's semantics and the
//! `Dual` kink convention). The bound adds no comparison surface to
//! generic code. Everything instantiates at `f64`, `Probe`,
//! `Dual<f64>`, `Interval`, and `Dual<Interval>` — the
//! derivative-vs-dual consistency and enclosure-containment test axes
//! below rely on exactly that.

pub mod boxes;
pub mod fit;
pub mod nurbs;
pub mod projection;

use std::sync::Arc;

pub use fit::{FIT_REMOVAL_BUDGET, FitError, FitOutcome, RefitSkip};
use geom_core::spline::SpanLocate;
use geom_core::{Point3, Real, Vec3};
pub use nurbs::{NurbsCurve2, NurbsCurve3};
pub use projection::{Projection2, Projection3, ProjectionInconclusive};

/// An analytic 3-D curve — a **complete locus** (see the crate docs for
/// the conventions: units, periodicity, the `he_plus` forward contract,
/// and the conventional unit-field invariants).
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

    /// The NURBS fallback (D3: representable from day one; evaluators
    /// implemented at M5 PR 3). The payload is a validated
    /// [`NurbsCurve3`] behind an [`Arc`] (immutable, cheap to clone —
    /// see the enum docs on the `Copy` loss). The "no description yet"
    /// state that the former unit variant carried is now
    /// [`Curve3::nurbs_placeholder`] — a poison-valued payload with the
    /// same all-poison evaluation behavior.
    Nurbs(Arc<NurbsCurve3<T>>),
}

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

impl<T: SpanLocate> Curve3<T> {
    /// The point at parameter `t` (see the variant docs for each
    /// parameterization; the crate docs for units and periodicity).
    ///
    /// Evaluation orders (fixed, D9):
    /// - Line: `origin + dir·t` — one componentwise scale, one add.
    /// - Circle: `(s, c) = θ.sin_cos()`; `radial = u_ref·c + v_ref·s`
    ///   with `v_ref = axis × u_ref` (the cross's own fixed order);
    ///   result `center + radial·radius` — exactly as parenthesized.
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
                let (s, c) = t.sin_cos();
                let v_ref = axis.cross(*u_ref);
                *center + (*u_ref * c + v_ref * s) * *radius
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
                let (s, c) = t.sin_cos();
                let v_ref = axis.cross(*u_ref);
                (*u_ref * (-s) + v_ref * c) * *radius
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
                let (s, c) = t.sin_cos();
                let v_ref = axis.cross(*u_ref);
                (*u_ref * (-c) + v_ref * (-s)) * *radius
            }
            Curve3::Nurbs(n) => n.deriv2(t),
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
            let cd: Curve3<Dual64> = lift_to_dual(&c);
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
            let cd: Curve3<Dual64> = lift_to_dual(&c);
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
            let ld: Curve3<Dual64> = lift_to_dual(&line);
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

    /// Lifts an f64 curve to `Curve3<Dual64>` with constant (∂/∂θ = 0)
    /// geometry — only the evaluation parameter is the variable.
    fn lift_to_dual(c: &Curve3<f64>) -> Curve3<Dual64> {
        fn cp(p: Point3<f64>) -> Point3<Dual64> {
            Point3::new(
                Dual::constant(p.x),
                Dual::constant(p.y),
                Dual::constant(p.z),
            )
        }
        fn cv(v: Vec3<f64>) -> Vec3<Dual64> {
            Vec3::new(
                Dual::constant(v.x),
                Dual::constant(v.y),
                Dual::constant(v.z),
            )
        }
        match *c {
            Curve3::Line { origin, dir } => Curve3::Line {
                origin: cp(origin),
                dir: cv(dir),
            },
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => Curve3::Circle {
                center: cp(center),
                axis: cv(axis),
                radius: Dual::constant(radius),
                u_ref: cv(u_ref),
            },
            Curve3::Nurbs(_) => Curve3::nurbs_placeholder(),
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
        assert!(line.eval(f64::NAN).x.is_nan());
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
        // ±∞ specifically poisons through sin_cos.
        assert!(c.eval(f64::INFINITY).x.is_nan());
    }

    // ------------------------------------------------------------------
    // Interval instantiation (feature-gated)
    // ------------------------------------------------------------------

    #[cfg(feature = "interval")]
    mod interval {
        use geom_core::{Bounds, Interval};

        use super::*;

        fn ipoint(p: Point3<f64>) -> Point3<Interval> {
            Point3::new(
                Interval::from_f64(p.x),
                Interval::from_f64(p.y),
                Interval::from_f64(p.z),
            )
        }

        fn ivec(v: Vec3<f64>) -> Vec3<Interval> {
            Vec3::new(
                Interval::from_f64(v.x),
                Interval::from_f64(v.y),
                Interval::from_f64(v.z),
            )
        }

        fn lift(c: &Curve3<f64>) -> Curve3<Interval> {
            match *c {
                Curve3::Line { origin, dir } => Curve3::Line {
                    origin: ipoint(origin),
                    dir: ivec(dir),
                },
                Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                } => Curve3::Circle {
                    center: ipoint(center),
                    axis: ivec(axis),
                    radius: Interval::from_f64(radius),
                    u_ref: ivec(u_ref),
                },
                Curve3::Nurbs(_) => Curve3::nurbs_placeholder(),
            }
        }

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
            let ci = lift(&c);
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
                let dist_res = radial.norm_squared() - r * r;
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
            let li = lift(&line);
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
            let ci = lift(&super::tilted_circle());
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
            let ci = lift(&super::xy_circle(2.0));
            let p = ci.eval(Interval::from_f64(f64::NAN));
            assert!(p.x.lo().is_nan() && p.y.lo().is_nan() && p.z.lo().is_nan());
            let n: Curve3<Interval> = Curve3::nurbs_placeholder();
            assert!(n.eval(Interval::zero()).x.lo().is_nan());
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
            let cd: Curve3<DualInterval> = match lift(&c) {
                Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                } => Curve3::Circle {
                    center: Point3::new(
                        Dual::constant(center.x),
                        Dual::constant(center.y),
                        Dual::constant(center.z),
                    ),
                    axis: Vec3::new(
                        Dual::constant(axis.x),
                        Dual::constant(axis.y),
                        Dual::constant(axis.z),
                    ),
                    radius: Dual::constant(radius),
                    u_ref: Vec3::new(
                        Dual::constant(u_ref.x),
                        Dual::constant(u_ref.y),
                        Dual::constant(u_ref.z),
                    ),
                },
                _ => panic!("fixture is a circle"),
            };
            let p = cd.eval(Dual::variable(Interval::from_f64(0.7)));
            let ci = lift(&c);
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

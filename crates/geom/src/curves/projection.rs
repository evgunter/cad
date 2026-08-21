//! Point projection / inversion onto NURBS curves (The NURBS Book
//! §6.1, pp. 229–234) — the C2.1 foot-point machinery: Newton on the
//! orthogonality condition `g(t) = C′(t)·(C(t) − P)`, with the
//! [`Projection3`]/[`Projection2`] **carrying their own certified
//! orthogonality residual**
//! so a bad projection cannot launder a bad cache — the consumer
//! re-checks the carried values through its own Decide/band machinery;
//! this module pins their presence and honesty.
//!
//! # This half's reading of the shared policy
//!
//! `crate::projection` carries the policy itself — the C6 `f64`
//! lane, the `f64`-structure + `T`-payload lift, the honesty contract,
//! and the four constants named below, declared once for both halves.
//! What follows is what that policy means in one parameter.
//!
//! - **Seeding rule**: for every nonempty span, in ascending span
//!   order, evaluate the squared distance to `P` at
//!   [`PROJECT_SEEDS_PER_SPAN`] parameters placed uniformly across the
//!   span **including both ends**; the seed is the first strict
//!   minimum (ascending scan, `<`) — ties keep the earlier parameter,
//!   NaN distances never win. Fixed count, fixed placement, fixed
//!   scan order: same curve and point ⇒ same seed, bitwise.
//! - **Newton**: at most [`PROJECT_MAX_ITERS`] steps of
//!   `t ← t − g(t)/g′(t)`, `g′ = C″·(C − P) + |C′|²`, each step
//!   clamped to the knot domain (clamped-v1 curves; periodic
//!   wraparound is a designed absence until the periodic form exists).
//! - **Acceptance** (the Book's two zero conditions plus its
//!   parameter-stagnation condition, as named constants):
//!   *point coincidence* `|C(t) − P| ≤` [`PROJECT_EPS_POINT`];
//!   *cosine* `|g| ≤` [`PROJECT_EPS_COSINE`]`·|C′|·|C − P|` — note
//!   that at a degenerate parameterization point (`|C′| = 0`, e.g. a
//!   cusp or a stationary control layout) this criterion is met with a
//!   trivially-zero orthogonality residual, so a consumer must band
//!   the **pair**: the carried `distance` stays honest there and is
//!   the value that refuses a bad foot (C2.1's both-residuals point);
//!   *stagnation* `|Δt|·|C′| ≤` [`PROJECT_EPS_POINT`] (this is how a
//!   domain-end foot converges: the clamp pins `t`, the step dies, and
//!   the projection reports the **honest, possibly large**
//!   orthogonality residual of the boundary point — the consumer's
//!   band decides, never this module).
//! - **Non-convergence** is the typed [`ProjectionInconclusive`]
//!   refusal — never a best-effort answer.
//!
//! # Honesty, in one parameter
//!
//! The shared contract is `crate::projection`'s. Here the residual
//! set is two: on a closed curve a deliberately bad seed can converge
//! to the far branch with a tiny `orthogonality` and a large
//! `distance`, and at `|C′| = 0` the cosine test is met with a
//! trivially-zero `orthogonality` — so a consumer bands the **pair**.
//! The planted-fixture rows in `tests/curves/projection.rs` pin both,
//! via [`NurbsCurve3::project_from_seed`], the raw-Newton entry that
//! exists for exactly those fixtures and for warm-started consumers.

use geom_core::{Bounds, Point2, Point3, Real};

use crate::curves::{NurbsCurve2, NurbsCurve3};
use crate::projection::{
    PROJECT_EPS_COSINE, PROJECT_EPS_POINT, PROJECT_MAX_ITERS, PROJECT_SEEDS_PER_SPAN, mid,
};

/// The typed non-convergence refusal: Newton spent its fixed budget
/// without meeting any acceptance condition (or hit a degenerate
/// `g′`/poisoned arithmetic). Carries the last state honestly — a
/// diagnosing consumer sees where the iteration died, and nothing
/// here can be mistaken for a foot point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectionInconclusive {
    /// Newton steps consumed.
    pub iterations: usize,
    /// The last parameter iterate.
    pub last_t: f64,
    /// `|C′·(C − P)|` at `last_t` (NaN when arithmetic poisoned).
    pub last_orthogonality: f64,
    /// `|C(last_t) − P|` (NaN when arithmetic poisoned).
    pub last_distance: f64,
}

impl core::fmt::Display for ProjectionInconclusive {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "projection inconclusive after {} iterations at t = {} \
             (orthogonality {:e}, distance {:e})",
            self.iterations, self.last_t, self.last_orthogonality, self.last_distance
        )
    }
}

impl core::error::Error for ProjectionInconclusive {}

macro_rules! nurbs_project {
    ($Curve:ident, $Point:ident, $Projection:ident) => {
        /// A converged foot point WITH its certified residuals (C2.1:
        /// the projection carries `|C′(t*)·(C(t*) − P)|` and the
        /// distance, so a bad projection cannot launder a bad cache —
        /// the consumer re-checks both through its own band machinery;
        /// see the module docs' honesty section).
        ///
        /// **At `T = Dual`, two of these fields carry a partial
        /// derivative and not a total one — issue #874.** `t` is
        /// selected as `f64` and frozen, so `foot` and `orthogonality`
        /// are differentiated at fixed `t*` and are short by the
        /// `C′(t*)·dt*/dp` term. `distance` is unaffected: at a
        /// converged foot the orthogonality condition IS the vanishing
        /// of that term's coefficient, and at a clamped domain-end foot
        /// `dt*/dp` is itself zero. Every VALUE channel is the plain-`T`
        /// run's bit-identically (D9); only the tangents are at stake.
        #[derive(Clone, Copy, Debug)]
        pub struct $Projection<T: Real> {
            /// The foot parameter `t*` (inside the knot domain) —
            /// **`f64` structure**: a selected parameter, not a
            /// certified quantity.
            pub t: f64,
            /// The curve point `C(t*)`, evaluated at the consumer's
            /// scalar.
            pub foot: $Point<T>,
            /// `|C(t*) − P|` in meters — the nearness residual, at `T`.
            pub distance: T,
            /// `|C′(t*)·(C(t*) − P)|` — the orthogonality residual
            /// (meters² per parameter unit; honest and possibly large
            /// for a domain-end foot), at `T`.
            pub orthogonality: T,
            /// Newton steps consumed (diagnostic structure).
            pub iterations: usize,
        }

        impl<T: Bounds> $Curve<T> {
            /// Projects `p` onto the curve: the fixed seeding sweep
            /// (module docs) followed by [`Self::project_from_seed`].
            /// Deterministic bit-for-bit per D9.
            ///
            /// # Errors
            ///
            /// [`ProjectionInconclusive`] when Newton's fixed budget
            /// expires without meeting an acceptance condition (a NaN
            /// input point lands here too — poison converges nowhere).
            pub fn project(&self, p: $Point<T>) -> Result<$Projection<T>, ProjectionInconclusive> {
                self.project_from_seed(p, self.project_seed(p))
            }

            /// The fixed-count seeding sweep (module docs: the seeding
            /// rule). Public for warm-start consumers and the planted
            /// wrong-branch fixtures; `project` = this + Newton.
            pub fn project_seed(&self, p: $Point<T>) -> f64 {
                let kv = self.knots();
                let mut best_t = kv.domain().0;
                let mut best_d2 = f64::INFINITY;
                for index in kv.first_span()..=kv.last_span() {
                    // Emptiness check and span validation are one step.
                    let Some(span) = kv.span(index) else { continue };
                    let (u0, u1) = (kv.knots()[index], kv.knots()[index + 1]);
                    for j in 0..PROJECT_SEEDS_PER_SPAN {
                        #[allow(clippy::cast_precision_loss)]
                        let frac = j as f64 / (PROJECT_SEEDS_PER_SPAN - 1) as f64;
                        let t = u0 + (u1 - u0) * frac;
                        let d = self.eval_in_span(span, T::from_f64(t)) - p;
                        let d2 = mid(d.dot(d));
                        // Strict `<`: first minimum wins, NaN never does.
                        if d2 < best_d2 {
                            best_d2 = d2;
                            best_t = t;
                        }
                    }
                }
                best_t
            }

            /// Newton on the orthogonality condition from an explicit
            /// seed — the raw entry behind [`Self::project`] (module
            /// docs: iteration policy, acceptance conditions, clamping,
            /// honesty). A bad seed converges to whatever stationary
            /// point it converges to; the carried residuals stay
            /// honest, which is the point.
            ///
            /// # Errors
            ///
            /// [`ProjectionInconclusive`] on budget expiry or
            /// degenerate/poisoned Newton arithmetic.
            pub fn project_from_seed(
                &self,
                p: $Point<T>,
                seed: f64,
            ) -> Result<$Projection<T>, ProjectionInconclusive> {
                let (lo, hi) = self.domain();
                let mut t = seed.clamp(lo, hi);
                let mut iterations = 0usize;
                let mut last_g = f64::NAN;
                let mut last_dist = f64::NAN;
                while iterations < PROJECT_MAX_ITERS {
                    let span = self.knots().span_at(t);
                    let (c, c1, c2) = self.ders_in_span(span, T::from_f64(t));
                    let d = c - p;
                    // The iteration reads structure through the
                    // brackets; the T-valued jet above is what the
                    // accepted payload is built from (module docs:
                    // f64 structure, T payload).
                    let dist = mid(d.norm());
                    let g = mid(c1.dot(d));
                    last_g = g;
                    last_dist = dist;
                    let speed = mid(c1.norm());
                    // Acceptance: coincidence, then cosine.
                    if dist <= PROJECT_EPS_POINT || g.abs() <= PROJECT_EPS_COSINE * speed * dist {
                        return Ok($Projection {
                            t,
                            foot: c,
                            distance: d.norm(),
                            orthogonality: c1.dot(d).abs(),
                            iterations,
                        });
                    }
                    let gp = mid(c2.dot(d)) + mid(c1.dot(c1));
                    let step = g / gp;
                    if !step.is_finite() {
                        break;
                    }
                    let tn = (t - step).clamp(lo, hi);
                    iterations += 1;
                    // Acceptance: parameter stagnation (domain-end
                    // feet land here — module docs).
                    if ((tn - t) * speed).abs() <= PROJECT_EPS_POINT {
                        let span = self.knots().span_at(tn);
                        let (c, c1, _) = self.ders_in_span(span, T::from_f64(tn));
                        let d = c - p;
                        let dist = d.norm();
                        if !mid(dist).is_nan() {
                            return Ok($Projection {
                                t: tn,
                                foot: c,
                                distance: dist,
                                orthogonality: c1.dot(d).abs(),
                                iterations,
                            });
                        }
                        break;
                    }
                    t = tn;
                }
                Err(ProjectionInconclusive {
                    iterations,
                    last_t: t,
                    last_orthogonality: last_g.abs(),
                    last_distance: last_dist,
                })
            }
        }
    };
}

nurbs_project!(NurbsCurve2, Point2, Projection2);
nurbs_project!(NurbsCurve3, Point3, Projection3);

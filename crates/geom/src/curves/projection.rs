//! Point projection / inversion onto NURBS curves (The NURBS Book
//! §6.1, pp. 229–234) — the C2.1 foot-point machinery: Newton on the
//! orthogonality condition `g(t) = C′(t)·(C(t) − P)`, with the
//! [`Projection3`]/[`Projection2`] **carrying their own certified
//! orthogonality residual**
//! so a bad projection cannot launder a bad cache — the consumer
//! re-checks the carried values through its own Decide/band machinery;
//! this module pins their presence and honesty.
//!
//! # This is C6's f64 lane — structure machinery
//!
//! Projection *selects* a parameter (structure); it decides no
//! topology. Everything here is `f64` with raw comparisons under C6's
//! pinning rule, deterministic per D9: fixed constants, fixed seeding,
//! fixed iteration policy, no data-dependent iteration order. The
//! certification of whatever a consumer builds from the foot point is
//! the consumer's, at its own scalar (C2).
//!
//! # The D9-fixed iteration policy (binding, named constants)
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
//! # Honesty (C2.1 verbatim)
//!
//! Newton converges to *stationary points* of the distance: on a
//! closed curve a deliberately bad seed can converge to the far branch
//! with a tiny orthogonality residual but a large distance. Both
//! values ride the projection, so the consumer's band check
//! catches every such case: wrong branch ⇒ `distance` fails the band;
//! boundary clamp ⇒ `orthogonality` fails it. The planted-fixture
//! tests (`tests/projection.rs`) pin both, via
//! [`NurbsCurve3::project_from_seed`] — the raw-Newton entry that
//! exists for exactly those fixtures and for warm-started consumers
//! (the PR 7 marcher).

use geom_core::{Point2, Point3};

use crate::curves::{NurbsCurve2, NurbsCurve3};

/// Fixed Newton iteration cap (D9 — never data-dependent).
pub const PROJECT_MAX_ITERS: usize = 32;

/// Fixed per-span seed count (module docs: the seeding rule).
pub const PROJECT_SEEDS_PER_SPAN: usize = 8;

/// Point-coincidence and parameter-stagnation threshold, in meters
/// (the Book's ε₁ role).
pub const PROJECT_EPS_POINT: f64 = 1e-13;

/// Cosine-zero threshold, dimensionless (the Book's ε₂ role):
/// `|g| ≤ ε₂·|C′|·|C − P|` is `|cos ∠(C′, C − P)| ≤ ε₂`.
pub const PROJECT_EPS_COSINE: f64 = 1e-12;

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
        #[derive(Clone, Copy, Debug)]
        pub struct $Projection {
            /// The foot parameter `t*` (inside the knot domain).
            pub t: f64,
            /// The curve point `C(t*)`.
            pub foot: $Point<f64>,
            /// `|C(t*) − P|` in meters — the nearness residual.
            pub distance: f64,
            /// `|C′(t*)·(C(t*) − P)|` — the orthogonality residual
            /// (meters² per parameter unit; honest and possibly large
            /// for a domain-end foot).
            pub orthogonality: f64,
            /// Newton steps consumed (diagnostic structure).
            pub iterations: usize,
        }

        impl $Curve<f64> {
            /// Projects `p` onto the curve: the fixed seeding sweep
            /// (module docs) followed by [`Self::project_from_seed`].
            /// Deterministic bit-for-bit per D9.
            ///
            /// # Errors
            ///
            /// [`ProjectionInconclusive`] when Newton's fixed budget
            /// expires without meeting an acceptance condition (a NaN
            /// input point lands here too — poison converges nowhere).
            pub fn project(&self, p: $Point<f64>) -> Result<$Projection, ProjectionInconclusive> {
                self.project_from_seed(p, self.project_seed(p))
            }

            /// The fixed-count seeding sweep (module docs: the seeding
            /// rule). Public for warm-start consumers and the planted
            /// wrong-branch fixtures; `project` = this + Newton.
            pub fn project_seed(&self, p: $Point<f64>) -> f64 {
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
                        let d = self.eval_in_span(span, t) - p;
                        let d2 = d.dot(d);
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
                p: $Point<f64>,
                seed: f64,
            ) -> Result<$Projection, ProjectionInconclusive> {
                let (lo, hi) = self.domain();
                let mut t = seed.clamp(lo, hi);
                let mut iterations = 0usize;
                let mut last_g = f64::NAN;
                let mut last_dist = f64::NAN;
                while iterations < PROJECT_MAX_ITERS {
                    let span = self.knots().span_at(t);
                    let (c, c1, c2) = self.ders_in_span(span, t);
                    let d = c - p;
                    let dist = d.norm();
                    let g = c1.dot(d);
                    last_g = g;
                    last_dist = dist;
                    let speed = c1.norm();
                    // Acceptance: coincidence, then cosine.
                    if dist <= PROJECT_EPS_POINT || g.abs() <= PROJECT_EPS_COSINE * speed * dist {
                        return Ok($Projection {
                            t,
                            foot: c,
                            distance: dist,
                            orthogonality: g.abs(),
                            iterations,
                        });
                    }
                    let gp = c2.dot(d) + c1.dot(c1);
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
                        let (c, c1, _) = self.ders_in_span(span, tn);
                        let d = c - p;
                        let dist = d.norm();
                        let g = c1.dot(d);
                        if !dist.is_nan() {
                            return Ok($Projection {
                                t: tn,
                                foot: c,
                                distance: dist,
                                orthogonality: g.abs(),
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

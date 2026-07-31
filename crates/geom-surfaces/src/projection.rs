//! Point projection / inversion onto NURBS **surfaces** (The NURBS Book
//! §6.1, pp. 229–234, the surface half) — the C2.1 foot-point machinery
//! for the limb-1 residual of a rung-3 cache whose operand has no
//! implicit form.
//!
//! This is the exact surface analogue of `geom_curves::projection`, and
//! it is deliberately written to the same contract, constant for
//! constant. M5 PR 4 landed the curve half; a fitted SSI carrier lying
//! against a NURBS **surface** needs this half, so PR 7 lands it with
//! its first consumer (the no-speculative-abstraction rule).
//!
//! # This is C6's f64 lane — structure machinery
//!
//! Projection *selects* a parameter pair (structure); it decides no
//! topology. Everything here is `f64` with raw comparisons under C6's
//! pinning rule, deterministic per D9: fixed constants, fixed seeding,
//! fixed iteration policy, no data-dependent iteration order. The
//! certification of whatever a consumer builds from the foot point is
//! the consumer's, at its own scalar (C2).
//!
//! # The D9-fixed iteration policy (binding, named constants)
//!
//! - **Seeding rule**: over every nonempty span **cell** (u-span ×
//!   v-span), in ascending `(span_u, span_v)` order, evaluate the
//!   squared distance to `P` on a
//!   [`SURFACE_PROJECT_SEEDS_PER_SPAN`]² grid placed uniformly across
//!   the cell **including all four edges**; the seed is the first
//!   strict minimum (ascending scan, `<`) — ties keep the earlier
//!   parameter pair, NaN distances never win.
//! - **Newton** on the two orthogonality conditions
//!   `f = S_u·r = 0`, `g = S_v·r = 0` with `r = S(u,v) − P`
//!   (the Book's Eqs. 6.5–6.6), Jacobian
//!
//!   ```text
//!   J = [ |S_u|² + S_uu·r    S_u·S_v + S_uv·r ]
//!       [ S_u·S_v + S_uv·r   |S_v|² + S_vv·r  ]
//!   ```
//!
//!   solved by the explicit 2×2 inverse (fixed order, no pivoting), at
//!   most [`SURFACE_PROJECT_MAX_ITERS`] steps, each clamped to the knot
//!   domain rectangle (clamped-v1 surfaces; periodic wraparound is a
//!   designed absence until the periodic form exists).
//! - **Acceptance**, exactly the Book's three conditions as named
//!   constants: *point coincidence* `|r| ≤`
//!   [`SURFACE_PROJECT_EPS_POINT`]; *cosine*, **both** directions
//!   `|S_u·r| ≤ ε₂·|S_u|·|r|` **and** `|S_v·r| ≤ ε₂·|S_v|·|r|` with
//!   ε₂ = [`SURFACE_PROJECT_EPS_COSINE`]; *stagnation*
//!   `|Δu·S_u + Δv·S_v| ≤` [`SURFACE_PROJECT_EPS_POINT`] (this is how a
//!   domain-edge foot converges — the clamp pins the parameters, the
//!   step dies, and the projection reports the **honest, possibly
//!   large** orthogonality residuals of the boundary point).
//! - **Non-convergence** is the typed [`SurfaceProjectionInconclusive`]
//!   refusal — never a best-effort answer.
//!
//! # Honesty (C2.1 verbatim: a bad projection cannot launder a bad
//! cache)
//!
//! Newton converges to *stationary points* of the distance: a
//! deliberately bad seed can converge to a far sheet with tiny
//! orthogonality residuals and a large distance, and a degenerate chart
//! point (`|S_u| = 0`, e.g. a collapsed row of control points) meets
//! the cosine test with a trivially-zero residual. **All three
//! residuals ride the [`SurfaceProjection`]** — `distance`,
//! `orthogonality_u`, `orthogonality_v` — so the consumer must band
//! them together: wrong sheet ⇒ `distance` fails the band; domain-edge
//! clamp ⇒ an orthogonality residual fails it. This module decides
//! nothing; it reports.

use geom_core::Point3;

use crate::NurbsSurface;

/// Fixed Newton iteration cap (D9 — never data-dependent). Matches
/// `geom_curves::projection::PROJECT_MAX_ITERS`; the two halves of §6.1
/// share one policy.
pub const SURFACE_PROJECT_MAX_ITERS: usize = 32;

/// Fixed per-span-per-direction seed count (module docs: the seeding
/// rule) — the grid over a cell is this squared.
pub const SURFACE_PROJECT_SEEDS_PER_SPAN: usize = 8;

/// Point-coincidence and parameter-stagnation threshold, in meters (the
/// Book's ε₁ role).
pub const SURFACE_PROJECT_EPS_POINT: f64 = 1e-13;

/// Cosine-zero threshold, dimensionless (the Book's ε₂ role).
pub const SURFACE_PROJECT_EPS_COSINE: f64 = 1e-12;

/// A converged surface foot point WITH its certified residuals (C2.1;
/// see the module docs' honesty section — the consumer bands all three
/// together, and this type exists so it *can*).
#[derive(Clone, Copy, Debug)]
pub struct SurfaceProjection {
    /// The foot parameter `u*` (inside the u knot domain).
    pub u: f64,
    /// The foot parameter `v*` (inside the v knot domain).
    pub v: f64,
    /// The surface point `S(u*, v*)`.
    pub foot: Point3<f64>,
    /// `|S(u*,v*) − P|` in meters — the nearness residual.
    pub distance: f64,
    /// `|S_u·(S − P)|` — the u-orthogonality residual (meters² per
    /// parameter unit; honest and possibly large for a domain-edge
    /// foot).
    pub orthogonality_u: f64,
    /// `|S_v·(S − P)|` — the v-orthogonality residual.
    pub orthogonality_v: f64,
    /// Newton steps consumed (diagnostic structure).
    pub iterations: usize,
}

/// The typed non-convergence refusal: Newton spent its fixed budget
/// without meeting any acceptance condition (or hit a singular
/// Jacobian / poisoned arithmetic). Carries the last state honestly —
/// nothing here can be mistaken for a foot point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SurfaceProjectionInconclusive {
    /// Newton steps consumed.
    pub iterations: usize,
    /// The last `u` iterate.
    pub last_u: f64,
    /// The last `v` iterate.
    pub last_v: f64,
    /// `|S_u·r|` there (NaN when arithmetic poisoned).
    pub last_orthogonality_u: f64,
    /// `|S_v·r|` there (NaN when arithmetic poisoned).
    pub last_orthogonality_v: f64,
    /// `|S − P|` there (NaN when arithmetic poisoned).
    pub last_distance: f64,
}

impl core::fmt::Display for SurfaceProjectionInconclusive {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "surface projection inconclusive after {} iterations at \
             (u, v) = ({}, {}) (orthogonality {:e}/{:e}, distance {:e})",
            self.iterations,
            self.last_u,
            self.last_v,
            self.last_orthogonality_u,
            self.last_orthogonality_v,
            self.last_distance
        )
    }
}

impl core::error::Error for SurfaceProjectionInconclusive {}

impl NurbsSurface<f64> {
    /// Projects `p` onto the surface: the fixed seeding sweep (module
    /// docs) followed by [`Self::project_from_seed`]. Deterministic
    /// bit-for-bit per D9.
    ///
    /// # Errors
    ///
    /// [`SurfaceProjectionInconclusive`] when Newton's fixed budget
    /// expires without meeting an acceptance condition (a NaN input
    /// point lands here too — poison converges nowhere).
    pub fn project(
        &self,
        p: Point3<f64>,
    ) -> Result<SurfaceProjection, SurfaceProjectionInconclusive> {
        let (u, v) = self.project_seed(p);
        self.project_from_seed(p, u, v)
    }

    /// The fixed-count seeding sweep (module docs: the seeding rule).
    /// Public for warm-start consumers (the PR 7 marcher and its
    /// subdivision seeds) and for the planted wrong-sheet fixtures.
    pub fn project_seed(&self, p: Point3<f64>) -> (f64, f64) {
        let (ku, kv) = (self.knots_u(), self.knots_v());
        let mut best = (ku.domain().0, kv.domain().0);
        let mut best_d2 = f64::INFINITY;
        for span_u in ku.first_span()..=ku.last_span() {
            if !ku.span_is_nonempty(span_u) {
                continue;
            }
            let (a0, a1) = (ku.knots()[span_u], ku.knots()[span_u + 1]);
            for span_v in kv.first_span()..=kv.last_span() {
                if !kv.span_is_nonempty(span_v) {
                    continue;
                }
                let (b0, b1) = (kv.knots()[span_v], kv.knots()[span_v + 1]);
                for i in 0..SURFACE_PROJECT_SEEDS_PER_SPAN {
                    #[allow(clippy::cast_precision_loss)]
                    let fu = i as f64 / (SURFACE_PROJECT_SEEDS_PER_SPAN - 1) as f64;
                    let u = a0 + (a1 - a0) * fu;
                    for j in 0..SURFACE_PROJECT_SEEDS_PER_SPAN {
                        #[allow(clippy::cast_precision_loss)]
                        let fv = j as f64 / (SURFACE_PROJECT_SEEDS_PER_SPAN - 1) as f64;
                        let v = b0 + (b1 - b0) * fv;
                        let d = self.eval_in_span(span_u, span_v, u, v) - p;
                        let d2 = d.dot(d);
                        // Strict `<`: first minimum wins, NaN never does.
                        if d2 < best_d2 {
                            best_d2 = d2;
                            best = (u, v);
                        }
                    }
                }
            }
        }
        best
    }

    /// Newton on the two orthogonality conditions from an explicit seed
    /// — the raw entry behind [`Self::project`] (module docs: iteration
    /// policy, acceptance conditions, clamping, honesty). A bad seed
    /// converges to whatever stationary point it converges to; the
    /// carried residuals stay honest, which is the point.
    ///
    /// # Errors
    ///
    /// [`SurfaceProjectionInconclusive`] on budget expiry or a
    /// degenerate/poisoned Newton system.
    pub fn project_from_seed(
        &self,
        p: Point3<f64>,
        seed_u: f64,
        seed_v: f64,
    ) -> Result<SurfaceProjection, SurfaceProjectionInconclusive> {
        let (ulo, uhi) = self.knots_u().domain();
        let (vlo, vhi) = self.knots_v().domain();
        let mut u = seed_u.clamp(ulo, uhi);
        let mut v = seed_v.clamp(vlo, vhi);
        let mut iterations = 0usize;
        let mut last_fu = f64::NAN;
        let mut last_fv = f64::NAN;
        let mut last_dist = f64::NAN;
        while iterations < SURFACE_PROJECT_MAX_ITERS {
            let su = self.knots_u().find_span(u);
            let sv = self.knots_v().find_span(v);
            let j = self.ders_in_span(su, sv, u, v);
            let r = j.point - p;
            let dist = r.norm();
            let fu = j.du.dot(r);
            let fv = j.dv.dot(r);
            last_fu = fu;
            last_fv = fv;
            last_dist = dist;
            let speed_u = j.du.norm();
            let speed_v = j.dv.norm();
            // Acceptance: coincidence, then the two cosines together.
            if dist <= SURFACE_PROJECT_EPS_POINT
                || (fu.abs() <= SURFACE_PROJECT_EPS_COSINE * speed_u * dist
                    && fv.abs() <= SURFACE_PROJECT_EPS_COSINE * speed_v * dist)
            {
                return Ok(SurfaceProjection {
                    u,
                    v,
                    foot: j.point,
                    distance: dist,
                    orthogonality_u: fu.abs(),
                    orthogonality_v: fv.abs(),
                    iterations,
                });
            }
            // The 2×2 Newton system, explicit inverse, fixed order.
            let a = j.du.dot(j.du) + j.duu.dot(r);
            let b = j.du.dot(j.dv) + j.duv.dot(r);
            let d = j.dv.dot(j.dv) + j.dvv.dot(r);
            let det = a * d - b * b;
            let step_u = -(d * fu - b * fv) / det;
            let step_v = -(a * fv - b * fu) / det;
            if !step_u.is_finite() || !step_v.is_finite() {
                break;
            }
            let un = (u + step_u).clamp(ulo, uhi);
            let vn = (v + step_v).clamp(vlo, vhi);
            iterations += 1;
            // Acceptance: parameter stagnation, measured in meters
            // through the chart (domain-edge feet land here).
            let moved = j.du * (un - u) + j.dv * (vn - v);
            if moved.norm() <= SURFACE_PROJECT_EPS_POINT {
                let su = self.knots_u().find_span(un);
                let sv = self.knots_v().find_span(vn);
                let jn = self.ders_in_span(su, sv, un, vn);
                let r = jn.point - p;
                let dist = r.norm();
                if !dist.is_nan() {
                    return Ok(SurfaceProjection {
                        u: un,
                        v: vn,
                        foot: jn.point,
                        distance: dist,
                        orthogonality_u: jn.du.dot(r).abs(),
                        orthogonality_v: jn.dv.dot(r).abs(),
                        iterations,
                    });
                }
                break;
            }
            u = un;
            v = vn;
        }
        Err(SurfaceProjectionInconclusive {
            iterations,
            last_u: u,
            last_v: v,
            last_orthogonality_u: last_fu.abs(),
            last_orthogonality_v: last_fv.abs(),
            last_distance: last_dist,
        })
    }
}

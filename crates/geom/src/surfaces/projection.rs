//! Point projection / inversion onto NURBS **surfaces** (The NURBS Book
//! §6.1, pp. 229–234, the surface half) — the C2.1 foot-point machinery
//! for the limb-1 residual of a rung-3 cache whose operand has no
//! implicit form.
//!
//! This is the surface analogue of [`crate::curves::projection`] and
//! implements the same contract: both halves take their iteration
//! budget, seed schedule and acceptance thresholds from
//! `crate::projection`, which is the one place that policy is
//! declared. What differs is dimension — two orthogonality conditions
//! instead of one, solved through a 2×2 Jacobian instead of a scalar
//! derivative — and that is why the iteration is written here rather
//! than shared.
//!
//! # This half's reading of the shared policy
//!
//! `crate::projection` carries the policy itself — the C6 `f64`
//! lane, the `f64`-structure + `T`-payload lift, the honesty contract,
//! and the four constants named below, declared once for both halves.
//! What follows is what that policy means in two parameters.
//!
//! - **Seeding rule**: over every nonempty span **cell** (u-span ×
//!   v-span), in ascending `(span_u, span_v)` order, evaluate the
//!   squared distance to `P` on a
//!   [`PROJECT_SEEDS_PER_SPAN`]² grid placed uniformly across
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
//!   most [`PROJECT_MAX_ITERS`] steps, each clamped to the knot
//!   domain rectangle (clamped-v1 surfaces; periodic wraparound is a
//!   designed absence until the periodic form exists).
//! - **Acceptance**, exactly the Book's three conditions as named
//!   constants: *point coincidence* `|r| ≤`
//!   [`PROJECT_EPS_POINT`]; *cosine*, **both** directions
//!   `|S_u·r| ≤ ε₂·|S_u|·|r|` **and** `|S_v·r| ≤ ε₂·|S_v|·|r|` with
//!   ε₂ = [`PROJECT_EPS_COSINE`]; *stagnation*
//!   `|Δu·S_u + Δv·S_v| ≤` [`PROJECT_EPS_POINT`] (this is how a
//!   domain-edge foot converges — the clamp pins the parameters, the
//!   step dies, and the projection reports the **honest, possibly
//!   large** orthogonality residuals of the boundary point).
//! - **Non-convergence** is the typed [`SurfaceProjectionInconclusive`]
//!   refusal — never a best-effort answer.
//!
//! # Honesty, in two parameters
//!
//! The shared contract is `crate::projection`'s. Here the residual
//! set is three — `distance`, `orthogonality_u`, `orthogonality_v` —
//! and all three ride the [`SurfaceProjection`] so the consumer can
//! band them together: a far sheet fails on `distance`, a domain-edge
//! clamp fails on an orthogonality residual, and a degenerate chart
//! point (`|S_u| = 0`, e.g. a collapsed row of control points) meets
//! the cosine test with a trivially-zero residual that `distance`
//! refuses.

use geom_core::{Bounds, Point3, Real};

use crate::projection::{
    PROJECT_EPS_COSINE, PROJECT_EPS_POINT, PROJECT_MAX_ITERS, PROJECT_SEEDS_PER_SPAN, mid,
};
use crate::surfaces::NurbsSurface;

/// A converged surface foot point WITH its certified residuals (C2.1;
/// see the module docs' honesty section — the consumer bands all three
/// together, and this type exists so it *can*).
///
/// **At `T = Dual` every `T`-valued field here carries a partial
/// derivative rather than a total one — issue #874.** `u` and `v` are
/// selected as `f64` and frozen (`crate::projection::mid`), so each field
/// is differentiated at fixed `(u*, v*)` and is short by the two
/// `∂/∂u × du*/dp` and `∂/∂v × dv*/dp` terms a frozen parameter cannot
/// produce — `S_u`, `S_v` for `foot`; `S_uu·r + |S_u|²` and
/// `S_uv·r + S_u·S_v` for `orthogonality_u`, and their transposes for
/// `orthogonality_v`; `S_u·r/|r|` and `S_v·r/|r|` for `distance`.
///
/// **`distance` is the only one the iteration bounds, and it bounds it on
/// ONE of the three exits** — the *cosine* one, where its coefficients
/// are the acceptance quantities in both directions and the dropped terms
/// are at most `ε₂·|S_u|·|du*/dp|` and `ε₂·|S_v|·|dv*/dp|`. The
/// *coincidence* exit holds no orthogonality condition, and *stagnation*
/// fires at any foot whose step dies rather than only at a domain edge.
/// Every VALUE channel is the plain-`T` run's bit-identically (D9); only
/// the tangents are at stake; `geom/tests/dual_foot_tangent.rs` pins it.
/// The curve half's type carries the same analysis in one parameter.
#[derive(Clone, Copy, Debug)]
pub struct SurfaceProjection<T: Real> {
    /// The foot parameter `u*` (inside the u knot domain) — **`f64`
    /// structure**: a selected parameter, not a certified quantity.
    pub u: f64,
    /// The foot parameter `v*` (inside the v knot domain) — `f64`
    /// structure, same reason.
    pub v: f64,
    /// The surface point `S(u*, v*)`, evaluated at the consumer's
    /// scalar.
    pub foot: Point3<T>,
    /// `|S(u*,v*) − P|` in meters — the nearness residual, at `T`.
    pub distance: T,
    /// `|S_u·(S − P)|` — the u-orthogonality residual (meters² per
    /// parameter unit; honest and possibly large for a domain-edge
    /// foot), at `T`.
    pub orthogonality_u: T,
    /// `|S_v·(S − P)|` — the v-orthogonality residual, at `T`.
    pub orthogonality_v: T,
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

impl<T: Bounds> NurbsSurface<T> {
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
        p: Point3<T>,
    ) -> Result<SurfaceProjection<T>, SurfaceProjectionInconclusive> {
        let (u, v) = self.project_seed(p);
        self.project_from_seed(p, u, v)
    }

    /// The fixed-count seeding sweep (module docs: the seeding rule).
    /// Public for warm-start consumers (the PR 7 marcher and its
    /// subdivision seeds) and for the planted wrong-sheet fixtures.
    pub fn project_seed(&self, p: Point3<T>) -> (f64, f64) {
        let (ku, kv) = (self.knots_u(), self.knots_v());
        let mut best = (ku.domain().0, kv.domain().0);
        let mut best_d2 = f64::INFINITY;
        for iu in ku.first_span()..=ku.last_span() {
            let (a0, a1) = (ku.knots()[iu], ku.knots()[iu + 1]);
            for iv in kv.first_span()..=kv.last_span() {
                // Emptiness check, span validation and window
                // construction are one step in both directions; the
                // window is then reused for all `SEEDS_PER_SPAN²`
                // evaluations below.
                let Some(win) = self.window(iu, iv) else {
                    continue;
                };
                let (b0, b1) = (kv.knots()[iv], kv.knots()[iv + 1]);
                for i in 0..PROJECT_SEEDS_PER_SPAN {
                    #[allow(clippy::cast_precision_loss)]
                    let fu = i as f64 / (PROJECT_SEEDS_PER_SPAN - 1) as f64;
                    let u = a0 + (a1 - a0) * fu;
                    for j in 0..PROJECT_SEEDS_PER_SPAN {
                        #[allow(clippy::cast_precision_loss)]
                        let fv = j as f64 / (PROJECT_SEEDS_PER_SPAN - 1) as f64;
                        let v = b0 + (b1 - b0) * fv;
                        let d = self.eval_in_span(win, T::from_f64(u), T::from_f64(v)) - p;
                        let d2 = mid(d.dot(d));
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
        p: Point3<T>,
        seed_u: f64,
        seed_v: f64,
    ) -> Result<SurfaceProjection<T>, SurfaceProjectionInconclusive> {
        let (ulo, uhi) = self.knots_u().domain();
        let (vlo, vhi) = self.knots_v().domain();
        let mut u = seed_u.clamp(ulo, uhi);
        let mut v = seed_v.clamp(vlo, vhi);
        let mut iterations = 0usize;
        let mut last_fu = f64::NAN;
        let mut last_fv = f64::NAN;
        let mut last_dist = f64::NAN;
        while iterations < PROJECT_MAX_ITERS {
            let j = self.ders_in_span(self.window_at(u, v), T::from_f64(u), T::from_f64(v));
            let r = j.point - p;
            // The iteration reads structure through the brackets; the
            // T-valued jet above is what the accepted payload is built
            // from (module docs: f64 structure, T payload).
            let dist = mid(r.norm());
            let fu = mid(j.du.dot(r));
            let fv = mid(j.dv.dot(r));
            last_fu = fu;
            last_fv = fv;
            last_dist = dist;
            let speed_u = mid(j.du.norm());
            let speed_v = mid(j.dv.norm());
            // Acceptance: coincidence, then the two cosines together.
            if dist <= PROJECT_EPS_POINT
                || (fu.abs() <= PROJECT_EPS_COSINE * speed_u * dist
                    && fv.abs() <= PROJECT_EPS_COSINE * speed_v * dist)
            {
                return Ok(SurfaceProjection {
                    u,
                    v,
                    foot: j.point,
                    distance: r.norm(),
                    orthogonality_u: j.du.dot(r).abs(),
                    orthogonality_v: j.dv.dot(r).abs(),
                    iterations,
                });
            }
            // The 2×2 Newton system, explicit inverse, fixed order.
            let a = mid(j.du.dot(j.du)) + mid(j.duu.dot(r));
            let b = mid(j.du.dot(j.dv)) + mid(j.duv.dot(r));
            let d = mid(j.dv.dot(j.dv)) + mid(j.dvv.dot(r));
            let det = a * d - b.powi(2);
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
            let moved = j.du * T::from_f64(un - u) + j.dv * T::from_f64(vn - v);
            if mid(moved.norm()) <= PROJECT_EPS_POINT {
                let jn =
                    self.ders_in_span(self.window_at(un, vn), T::from_f64(un), T::from_f64(vn));
                let r = jn.point - p;
                let dist = r.norm();
                if !mid(dist).is_nan() {
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

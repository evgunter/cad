//! **A NURBS patch's certified distance from an analytic surface** — the
//! patch-side twin of the curve × face composite arm
//! ([`crate::curve_face::composite`]): one number, an upper bound on how
//! far any point of the WHOLE patch lies from the surface's locus, read
//! off coefficients with no evaluation and no sampling.
//!
//! # What it bounds, and why that is whole-patch
//!
//! The patch's control net is written in the surface's chart frame in
//! certification arithmetic (the frame orthonormalized from the stored
//! axis and reference direction, [`crate::curve_face::composite`]'s
//! `framed_form`), and the surface's canonical form `f` (plane `z`,
//! cylinder `x² + y² − R²`, sphere `|ξ|² − r²`, double cone
//! `cos²α·(x² + y²) − sin²α·z²`, torus quartic) is composed with the
//! patch as a tensor-product rational Bernstein form `f ∘ S = N/D`, `D` a
//! power of the weight function
//! ([`geom_core::spline::compose::patch::canonical_patch_composite`]).
//! The patch's knot-span cells cover its domain, and on each cell:
//!
//! - `|f|` is at most the magnitude of `hull(N)/hull(D)`: a Bernstein
//!   polynomial lies in the hull of its coefficients, and `D`'s hull is
//!   certified positive or the quotient refuses;
//! - the cell's points lie in the convex hull of its rational Bézier
//!   control points `(X_k, Y_k, Z_k)/W_k` (positive weights make the
//!   rational basis a nonnegative partition of unity), so the box and
//!   the chord support of those points bound the cell's axial distance
//!   `ρ` from below and above and its height `z`;
//! - the chart's conversion from `|f|` to metres
//!   ([`crate::pcurve_cache::projected`]'s `Meters`, the one statement of
//!   it, read here as the curve arms read it) turns those three into an
//!   upper bound on the distance of every point of the cell from the
//!   locus: `|f|/(ρ + R)` on a cylinder and its sphere twin, `|f|` over
//!   the generator lever `ρ cos α + z sin α` on a cone (`√|f|` where that
//!   lever does not decide positive), and `|f|` over the core-circle and
//!   `(ρ + R)² − r²` levers on a torus. Each is an inequality at every
//!   point of the cell, never a linearization.
//!
//! The largest cell bound is the patch's. Every operation is interval
//! arithmetic with outward rounding, and the final bound is the upper end
//! of an enclosure, so it is rounded up: a bound read here holds for the
//! real patch the `f64` net names, against the real surface the stored
//! fields name.
//!
//! The cone's locus is the DOUBLE cone ([`geom::Surface::Cone`]'s
//! complete locus), and its conversion is read on the opening nappe's
//! lever: a patch on the other nappe reads its weaker apex bound.
//!
//! # Refusal
//!
//! `NaN` — which fails every `≤ ε` comparison (D4 ¶2) — off an analytic
//! chart, on a spindle sheet, for a net or a frame that does not
//! certify, and for a cell whose bound does not (a weight coefficient not
//! certified positive, a lever not certified positive on a torus).

use geom::{NurbsSurface, Surface};
use geom_core::interval::max_bound;
use geom_core::spline::compose::patch::{PatchSpans, canonical_patch_composite};
use geom_core::spline::compose::tensor::SurfaceCertData;
use geom_core::{Bounds, Interval, Real};

use crate::curve_face::composite::framed_form;
use crate::pcurve_cache::projected::part_floors;

/// A certified upper bound in metres on the distance of every point of
/// `patch` from `surface`'s locus (module docs); `NaN` where none is
/// certified.
#[must_use]
pub fn patch_distance_sup(patch: &NurbsSurface<f64>, surface: &Surface<f64>) -> f64 {
    let point = |x: f64| Interval::from_bounds(x, x);
    let Ok((coords, form, meters)) = framed_form(surface, patch.control(), &point) else {
        return f64::NAN;
    };
    let Ok(data) = SurfaceCertData::new(
        patch.knots_u_f64_interim(),
        patch.knots_v_f64_interim(),
        patch.weights_f64_interim(),
        &coords,
    ) else {
        return f64::NAN;
    };
    let (points, weight) = data.bernstein_channels();
    let composite = canonical_patch_composite(&points, &weight, &form);
    let (nu, nv) = composite.num.cell_counts();
    if nu == 0 || nv == 0 {
        return f64::NAN;
    }
    let upper = |x: Interval| if x.is_certified() { x.hi() } else { f64::NAN };
    let lower = |x: Interval| if x.is_certified() { x.lo() } else { f64::NAN };
    let mut worst = 0.0_f64;
    // Fixed ascending cell order (D9).
    for su in 0..nu {
        for sv in 0..nv {
            let f = composite.cell_bound(su, sv);
            let Some(controls) = PatchSpans::rational_cell_controls(
                [&points[0], &points[1], &points[2]],
                &weight,
                su,
                sv,
            ) else {
                return f64::NAN;
            };
            if !f.is_certified() || controls.is_empty() {
                return f64::NAN;
            }
            let f_sup = f.lo().abs().max(f.hi().abs());
            let (boxed, support, z, far) = part_floors(&controls);
            // The support's divisor may not certify (a chord through the
            // axis): the box's floor stands alone then.
            let rho = if support.is_certified() {
                boxed.max(support)
            } else {
                boxed
            };
            let (rho_lo, rho_hi, z) = (lower(rho).max(0.0), upper(far), (lower(z.0), upper(z.1)));
            if !(rho_lo.is_finite() && rho_hi.is_finite() && z.0.is_finite() && z.1.is_finite()) {
                return f64::NAN;
            }
            let bound = meters
                .certified(point(f_sup), (point(rho_lo), point(rho_hi)), z, |lever| {
                    lever.is_certified() && lever.lo() > 0.0
                })
                .map_or(f64::NAN, upper);
            worst = max_bound(worst, bound);
            if worst.is_nan() {
                return worst;
            }
        }
    }
    worst
}

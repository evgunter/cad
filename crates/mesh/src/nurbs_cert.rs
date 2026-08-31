//! **The trimmed-NURBS deviation certificate** (M7, the montage
//! skin-scenes unit): a certified per-face Hessian sup bound from the
//! control net, and the per-triangle interpolation bound it feeds —
//! the torus certificate's derivation generalized from a closed-form
//! Hessian to a **hull-derived** one.
//!
//! # The Hessian bound is a convexity fact, not an estimate
//!
//! For a NON-RATIONAL tensor-product B-spline surface
//! `S(u,v) = Σᵢⱼ Nᵢ(u)·Nⱼ(v)·Pᵢⱼ`, each second partial is itself a
//! tensor-product B-spline whose coefficient net comes from knot
//! differencing (The NURBS Book eq. 3.24, applied per direction —
//! exactly [`geom_core::spline::hull::derivative_coeffs`], iterated):
//! `S_uu` differences twice along `u`, `S_vv` twice along `v`, `S_uv`
//! once along each. The B-spline bases in both directions are
//! nonnegative partitions of unity (degree ≥ 0), so every value of
//! `S_uu` (etc.) is a convex combination of its derived coefficients
//! and lies in their hull — the C2.2 mechanism `chords` already uses
//! for edge carriers, lifted to the surface. Componentwise hulls give
//! `sup‖S_uu‖ ≤ √(Σ_c sup²)` = [`NurbsFaceBound::muu`], and likewise
//! `muv`, `mvv`. Rounding: interval (ring) arithmetic end to end, with
//! a final `next_up` on the square root.
//!
//! # The per-triangle certificate
//!
//! For a mesh triangle with UV corners spanning an axis-aligned box of
//! extents `(a_u, a_v)`, expand `S` to first order at the box center
//! `w₀` (the surface is defined on the whole chart rectangle, so the
//! box is in-domain). Every triangle point `w` has
//! `|w_u − w₀_u| ≤ a_u/2`, `|w_v − w₀_v| ≤ a_v/2`, and the integral
//! Taylor remainder (valid for C¹ surfaces with an a.e. second
//! derivative bound — hence the C¹ gate below) gives
//!
//! ```text
//! |S(w) − T₁(w)| ≤ ½·(muu·(a_u/2)² + 2·muv·(a_u/2)(a_v/2) + mvv·(a_v/2)²)
//!               = Q/8,   Q := muu·a_u² + 2·muv·a_u·a_v + mvv·a_v²
//! ```
//!
//! The affine interpolant Π agrees with `S` at the three vertices, so
//! `|Π − T₁| ≤ Q/8` at the vertices; `Π − T₁` is affine over the
//! triangle, hence `≤ Q/8` everywhere on it. Total: `‖S − Π‖ ≤ Q/4` —
//! [`NurbsFaceBound::cert`]. (Sanity: with `muu = R+r`, `muv = r`,
//! `mvv = r` and `a_u, a_v ≤ L` this is at most `(R+2r)·L²/2`,
//! strictly inside the torus certificate's `(3/4)(R+2r)·L²` — same
//! derivation, anisotropic accounting.)
//!
//! As everywhere in this crate the two documented additive slacks (≤ ε
//! boundary-carrier residual, f64 evaluation rounding) sit OUTSIDE the
//! bound: the honest promise is δ + ε (crate docs).
//!
//! # The rational arm (M8-5)
//!
//! A RATIONAL face is `S = A/w` with `A = ΣΣ Nᵢ Nⱼ wᵢⱼ Pᵢⱼ` and
//! `w = ΣΣ Nᵢ Nⱼ wᵢⱼ` — both POLYNOMIAL tensor-product B-splines, so
//! every ingredient below is the same control-hull convexity fact as
//! the integral arm, taken on the homogeneous nets. The quotient rule
//! (exactly `NurbsSurface::ders_in_span`'s corrections) gives, for any
//! constant `c` (write `Ã = A − c·w`, so `S − c = Ã/w` and the
//! derivatives of `S` are unchanged):
//!
//! ```text
//! S_u  = (Ã_u  − (S − c)·w_u) / w                        (v symmetric)
//! S_uu = (Ã_uu − 2·S_u·w_u − (S − c)·w_uu) / w           (v symmetric)
//! S_uv = (Ã_uv − S_u·w_v − S_v·w_u − (S − c)·w_uv) / w
//! ```
//!
//! Taking sups componentwise over one knot-span cell:
//!
//! - `sup|Ã_kl|` — hull of the recentred homogeneous derivative
//!   coefficients active on the cell
//!   ([`geom_core::spline::hull::derivative_coeffs`]
//!   iterated, exactly as the integral arm; recentring commutes with
//!   knot differencing, `d(A − c·w) = dA − c·dw`);
//! - `sup|S^c − c^c| ≤ max_active |P^c − c^c|` — the rational value
//!   hull: strictly positive weights make the rational basis a
//!   nonnegative partition of unity over the active net (the licence,
//!   checked; refused typed otherwise);
//! - `sup|S_u|` — the same recurrence one order down;
//! - `sup|w_kl|` — the weight spline's own derivative hulls.
//!
//! **The divisor is the cell's weight hull, argued not assumed.** On
//! the cell, `w ∈ [w_min, w_max]` of the active weights (convex
//! combination), and the recurrence is evaluated SIGNED in the ring —
//! the true minus signs, divided by the whole hull, which is where the
//! quotient rule's cancellations survive. The interval division
//! poisons if positivity was never proven.
//!
//! **Recentring keeps the cross terms cell-sized**: with the cell's
//! control centroid as `c`, `sup|S − c|` is a cell-of-control-net
//! fact, not a whole-patch one, so `(S − c)·w_dd` and `S_d·w_d` do not
//! inflate with the patch's distance from the origin (the M8-2
//! template's trick, lifted to two parameters). The whole-domain bound
//! is the max over cells of the per-cell sups, after the FIXED
//! [`patch_bound::RATIONAL_CERT_SPLITS`] refinement (schedule docs there).
//!
//! A degree-1 direction's `Ã_dd`, `w_dd` are exactly zero, but its
//! CROSS terms survive — a rational degree-1 direction genuinely
//! curves in parameter (a Möbius-reparameterized ruling), so its
//! second partials are NOT the integral arm's exact zero. The
//! recurrences above carry that automatically.
//!
//! The certificate consumer is unchanged: the Q/4 Taylor bound only
//! needs C¹ with an a.e. second-derivative bound, and the C¹ gate
//! (interior multiplicities ≤ p − 1, on the homogeneous nets) plus
//! `w > 0` gives exactly that for `S = A/w`.
//!
//! **Conservatism** (the speed meter's posture): the answer is a
//! bound, not an estimate. Ordinary rational walls measure within a
//! small factor of the true sup (the falsifier rows print the
//! ratios), but extreme weight ratios (1e-2..1e2 and beyond) can
//! leave the bound orders above the truth — the product terms lose
//! the sign correlation a steep ramp lives in. The cost is only grid
//! density, and a δ fine enough to overflow the 2²⁴ cap refuses
//! typed ([`crate::sizing::ceil_count`]) — never a wrong mesh.
//!
//! # Grid sizing (heuristic; the certificate is the guarantee)
//!
//! Budgeting a triangle's box at two grid cells per axis
//! (`a_u ≤ 2·h_u`, `a_v ≤ 2·h_v`), a step pair `(h_u, h_v)` is legal
//! exactly when it lies inside the certified ellipse
//!
//! ```text
//! muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s .
//! ```
//!
//! **Since TESS-SPLIT the shipped point selection is the
//! cell-minimizing point of that region subject to the ratified 3-D
//! aspect cap** ([`NurbsFaceBound::split_steps`], [`ASPECT_CAP`]) —
//! the retired AM-GM decoupling `2·a_u·a_v ≤ a_u² + a_v²` landed on a
//! particular interior point and over-gridded every ruled wall across
//! its flat direction (the #547 measurement's dominant ~4x). A fully
//! unconstrained direction (affine patch) keeps step ∞ — one cell.
//!
//! **Since TESS-SPAN (the #320 span promotion) the shipped grid is
//! sized PER KNOT-SPAN CELL in `v`**: the trimmed lane consumes
//! [`nurbs_cell_grid`] — the same certified assembly reported cell by
//! cell — and builds a TENSOR grid whose v-rows apply the step rule
//! above per v-band with that band's own bounds
//! ([`NurbsCellGrid::row_bound`]), rows landing on the band
//! boundaries so a grid triangle's certificate is the certificate of
//! the band containing it ([`NurbsCellGrid::cert`]). Malign bands and
//! their neighbours snap their u-columns to the whole-patch schedule:
//! they stay phase-aligned with the chord pass's boundary points
//! (sized from the same steps), which is what keeps anisotropic
//! boundary slivers certified (`crate::trimmed` module docs tell the
//! measured story); that forfeit is metered.
//!
//! The whole-patch steps still bound the BOUNDARY chord schedule of
//! every adjacent edge (`chords`: the adjacent-torus tightening
//! pattern) — the reported TESS-SPAN D-2 choice: an edge's chords are
//! shared with the neighbouring face, so they keep the conservative
//! whole-patch schedule and forfeit the span gain along boundaries
//! (quantified by `crate::budget`'s meter), rather than teaching the
//! chord pass which cells a pcurve image crosses.
//!
//! # Covered vs refused (partial coverage, stated)
//!
//! Covered: described, per direction either degree ≥ 2 with interior
//! multiplicities ≤ p − 1 (C¹) or degree 1 single-span — integral
//! faces through the direct hull arm, rational faces (any weight not
//! bitwise 1, all strictly positive) through the quotient-rule arm
//! above (M8-5; the loft/sweep wall class plus the arc-walled bodies
//! M8-2 made buildable). Refused typed
//! ([`TessellateError::UnsupportedNurbsFace`]): illegal rational
//! descriptions (a non-positive/non-finite weight voids the
//! convex-combination licence), C⁰ creases (the Taylor remainder
//! needs C¹ — for the standard multi-arc rational quadratic that
//! means split at the double knots), degree-0 directions, and
//! poisoned/non-finite hulls. The placeholder refuses
//! [`TessellateError::UnsupportedSurface`] upstream in `trimmed`.

use geom::NurbsSurface;
use geom_brep::patch_bound::{self, PatchBoundError};
use geom_core::ring_interval::RingInterval;
use topo::FaceKey;

use crate::types::TessellateError;

/// A patch-bound refusal, reported in this crate's error type. The
/// prose is the assembly's own ([`PatchBoundError::note`]) — one
/// spelling of each refusal, wherever it surfaces.
fn face_err(fk: FaceKey, e: PatchBoundError) -> TessellateError {
    TessellateError::UnsupportedNurbsFace {
        face: fk,
        note: e.note(),
    }
}

/// **The whole-patch bound as a fold over the per-cell enclosures**
/// ([`patch_bound::PatchCell`]) — the one arm, both patch classes.
///
/// Per-cell-then-union is tighter or equal, never wider: a cell's
/// window is a SUBSET of the whole net's, so no cell can report more
/// than the whole net. It reports LESS wherever the three channels'
/// extremes live in different cells — the whole-net reading adds three
/// per-channel maxima that no single point of the patch attains
/// together, while the fold takes the largest sum any one cell
/// realises.
///
/// The accumulation is hull-then-`m`, at the ring level and in cell
/// order, because that is what makes a poisoned cell reach the shared
/// finite check: `m` maps poison to NaN, and an f64 `max` over NaN
/// would drop it.
fn folded_face_bound(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    let cells = patch_bound::patch_cells(n).map_err(|e| face_err(fk, e))?;
    let mut acc: [Option<RingInterval>; 5] = [None; 5];
    for c in &cells {
        for (slot, v) in acc.iter_mut().zip(cell_readings(c)) {
            *slot = Some(match *slot {
                None => v,
                Some(h) => RingInterval::hull(h, v),
            });
        }
    }
    let m = |sq: Option<RingInterval>| sq.map_or(f64::NAN, cell_component);
    Ok(NurbsFaceBound {
        muu: m(acc[0]),
        muv: m(acc[1]),
        mvv: m(acc[2]),
        mu1: m(acc[3]),
        mv1: m(acc[4]),
    })
}

/// One cell's five squared-sum readings, in [`NurbsFaceBound`]'s field
/// order (`uu, uv, vv, u, v`) — the single place this crate names which
/// enclosure feeds which bound.
///
/// The cell reports SIGNED componentwise enclosures and this is where
/// a vector magnitude is read off them: `Σ_c sup²`, whose `√hi`
/// ([`cell_component`]) bounds the partial's norm. There is no second
/// reading to choose between — the magnitude assembly the rational arm
/// used to carry alongside the signed one applied the triangle
/// inequality to the quotient rule and could not see its cancellations
/// (issue 1006).
fn cell_readings(c: &patch_bound::PatchCell) -> [RingInterval; 5] {
    [
        patch_bound::sq_norm(c.s_uu),
        patch_bound::sq_norm(c.s_uv),
        patch_bound::sq_norm(c.s_vv),
        patch_bound::sq_norm(c.s_u),
        patch_bound::sq_norm(c.s_v),
    ]
}

/// Certified sup bounds on the three second partials of one described
/// non-rational NURBS face, over its whole chart rectangle (module
/// docs). Computed once per face; consumed by the trimmed lane's grid
/// sizing, its per-triangle certificate, and the chord pass's
/// adjacent-face tightening.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NurbsFaceBound {
    /// `sup ‖S_uu‖` — EXACTLY `0.0` when the assembled enclosure is
    /// the exact zero (a degree-1 single-span integral direction, or a
    /// control net whose differences vanish in f64), so the split
    /// selection's degenerate-direction predicates are decided on the
    /// value rather than on a threshold ([`cell_component`]).
    pub muu: f64,
    /// `sup ‖S_uv‖`.
    pub muv: f64,
    /// `sup ‖S_vv‖` (exact `0.0` for the v-direction analogue).
    pub mvv: f64,
    /// `sup ‖S_u‖` — the first-fundamental-form sample the split
    /// selection's 3-D aspect cap reads ([`ASPECT_CAP`]): the same
    /// control-hull convexity fact as the Hessian sups, one derivative
    /// order down, taken over the same window the Hessian bound is
    /// taken over. Exact `0.0` only for a 3-D-degenerate direction
    /// (the surface does not move with `u`).
    pub mu1: f64,
    /// `sup ‖S_v‖` (the v-direction analogue of [`Self::mu1`]).
    pub mv1: f64,
}

impl NurbsFaceBound {
    /// The per-triangle deviation certificate `Q/4` (module docs) for
    /// a triangle whose UV corners are `uv`.
    pub(crate) fn cert(&self, uv: [[f64; 2]; 3]) -> f64 {
        let au = max3(uv[0][0], uv[1][0], uv[2][0]) - min3(uv[0][0], uv[1][0], uv[2][0]);
        let av = max3(uv[0][1], uv[1][1], uv[2][1]) - min3(uv[0][1], uv[1][1], uv[2][1]);
        0.25 * (self.muu * au.powi(2) + 2.0 * self.muv * au * av + self.mvv * av.powi(2))
    }

    /// The `(h_u, h_v)` UV grid steps for sizing target `delta_s` —
    /// [`Self::split_steps`] without the constraint-activity flag.
    /// `f64::INFINITY` for a direction nothing constrains
    /// ([`crate::sizing::ceil_count`] turns that into one cell).
    pub(crate) fn grid_steps(&self, delta_s: f64) -> (f64, f64) {
        let s = self.split_steps(delta_s);
        (s.hu, s.hv)
    }

    /// **The split selection** (TESS-SPLIT, the ratified aspect policy
    /// — `docs/TESS-BUDGET.md`, PR #568): the cell-minimizing point on
    /// the certified ellipse
    /// `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`, subject to the 3-D
    /// aspect cap [`ASPECT_CAP`].
    ///
    /// # The closed form
    ///
    /// Parametrize the ellipse boundary by the parameter aspect
    /// `t = h_u/h_v` (NOTE the convention: `tess-meter`'s
    /// counterfactual scan parametrizes the same ellipse by the
    /// RECIPROCAL, `t = h_v/h_u` — the two are separate derivations
    /// in separate cargo roots by design, and their `t`s must never
    /// be compared directly); then `h_v = √(δ_s/q(t))` with
    /// `q(t) = muu·t² + 2·muv·t + mvv`, and the cell count over a box
    /// is proportional to `1/(h_u·h_v) = q(t)/(t·δ_s)`. Minimizing
    /// `g(t) = q(t)/t = muu·t + 2·muv + mvv/t` (convex for
    /// `muu, mvv ≥ 0`) gives the interior optimum `t* = √(mvv/muu)`
    /// where both exist; the cross term `muv` shifts no optimum (it is
    /// constant in `g`), it only scales the budget. The optimum always
    /// SATURATES the ellipse — growing both steps at fixed `t` only
    /// helps — so every chosen point is inside the same certificate
    /// region as the retired AM-GM point.
    ///
    /// # The aspect cap, through the first fundamental form
    ///
    /// The cell's 3-D edge-length ratio is measured through the first
    /// fundamental form as `(h_u·mu1)/(h_v·mv1)`, and the cap demands
    /// it lie in `[1/A, A]` — a window `t ∈ [ρ/A, ρ·A]`,
    /// `ρ = mv1/mu1`. **Sampling choice, and its conservatism,
    /// stated** (spec D-1): `mu1`/`mv1` are the certified SUPS of the
    /// two speeds over the same window the Hessian sups are taken over
    /// — no new sample sites. The capped quantity is therefore the
    /// ratio of sup-mapped edge lengths, which brackets the true
    /// pointwise ratio only up to each speed's variation across the
    /// cell: a cell whose `‖S_u‖` varies by a factor k can realize a
    /// pointwise aspect up to k beyond the cap. On the ruled walls the
    /// cap exists for, per-cell speeds are near-uniform and the factor
    /// is small; the certificate is untouched either way — the cap is
    /// mesh-quality policy, not a deviation bound.
    ///
    /// Under `ceil` the step-space cap is exact for the emitted cell:
    /// a trim box narrow enough that ONE cell already satisfies the
    /// cap has extent ≤ the capped step, so no division is added.
    ///
    /// # Degenerate directions — exact arms, decided predicates
    ///
    /// The predicates are `== 0.0` against the assembled enclosures'
    /// exact zeros ([`cell_component`] preserves them), never a
    /// threshold. Each degenerate case gets its own arm rather than a
    /// limit of the generic formula (spec D-1; the test pins the arm
    /// against the limit):
    ///
    /// * `muu = mvv = muv = 0` (affine patch): nothing constrains
    ///   either step — `(∞, ∞)`, one cell, deviation exactly zero.
    /// * `muu = 0 < mvv` (the ruled wall): `g` is strictly decreasing,
    ///   the unconstrained optimum is the degenerate strip `t → ∞`,
    ///   and the cap is what binds: `t = ρ·A` exactly. Mirror for
    ///   `mvv = 0 < muu`.
    /// * `muu = mvv = 0 < muv` (twisted ruling): `g` is constant —
    ///   every ellipse point costs the same — so the selection takes
    ///   the aspect-1 point `t = ρ`, deterministically.
    /// * A 3-D-degenerate direction (`mu1 = 0` or `mv1 = 0`: the
    ///   surface has no extent to aspect) leaves the cap without a
    ///   window. The generic interior optimum needs no window; the
    ///   boundary-seeking cases above then have no attained optimum
    ///   and take the balanced point `t = 1` — a decided fallback for
    ///   a face that is degenerate as geometry, chosen over refusing
    ///   because the certificate still holds at any chosen point.
    pub(crate) fn split_steps(&self, delta_s: f64) -> SplitSteps {
        let (muu, muv, mvv) = (self.muu, self.muv, self.mvv);
        if muu == 0.0 && mvv == 0.0 && muv == 0.0 {
            return SplitSteps {
                hu: f64::INFINITY,
                hv: f64::INFINITY,
                cap: false,
            };
        }
        // The speed ratio ρ and the aspect window in t = h_u/h_v,
        // when the face has 3-D extent in both directions to measure
        // an aspect against. The finite/positive filter on ρ ITSELF
        // covers the overflow/underflow corner (two finite positive
        // sups whose ratio leaves the finite line — a face degenerate
        // beyond any real geometry): the window then does not exist
        // and the arms below take their windowless fallbacks, so the
        // answer stays finite arithmetic rather than a NaN step.
        let rho =
            (self.mu1 > 0.0 && self.mv1 > 0.0 && self.mu1.is_finite() && self.mv1.is_finite())
                .then(|| self.mv1 / self.mu1)
                .filter(|r| r.is_finite() && *r > 0.0);
        let window = rho.map(|r| (r / ASPECT_CAP, r * ASPECT_CAP));
        // The chosen parameter aspect and whether the cap chose it.
        let (t, cap) = if muu > 0.0 && mvv > 0.0 {
            let t_star = (mvv / muu).sqrt();
            match window {
                Some((tlo, thi)) => (t_star.clamp(tlo, thi), t_star < tlo || t_star > thi),
                None => (t_star, false),
            }
        } else if muu == 0.0 && mvv > 0.0 {
            // Ruled wall: the strip t → ∞ is optimal; the cap binds.
            match window {
                Some((_, thi)) => (thi, true),
                None => (1.0, false),
            }
        } else if mvv == 0.0 && muu > 0.0 {
            match window {
                Some((tlo, _)) => (tlo, true),
                None => (1.0, false),
            }
        } else {
            // muu = mvv = 0 < muv: cost is aspect-invariant.
            (rho.unwrap_or(1.0), false)
        };
        // Saturate the ellipse at the chosen aspect. q(t) > 0 here:
        // at least one of muu·t², 2·muv·t, mvv is a positive product
        // of finite positives (t is positive and finite by
        // construction of every arm above).
        let q = muv.mul_add(2.0 * t, muu.mul_add(t.powi(2), mvv));
        let hv = (delta_s / q).sqrt();
        SplitSteps {
            hu: t * hv,
            hv,
            cap,
        }
    }

    /// The largest `h_v` this bound's ellipse admits at a FIXED `h_u`
    /// — the alignment projection [`NurbsCellGrid::band_schedule`]
    /// uses to put a band on the patch column schedule: solve
    /// `mvv·h_v² + 2·muv·h_u·h_v + muu·h_u² = δ_s` for the positive
    /// root. Exact arms on the decided zero predicates, as
    /// [`Self::split_steps`].
    ///
    /// The caller guarantees `muu·h_u² ≤ δ_s` (the projection is only
    /// ever taken at `h_u ≤` the whole-patch selection's own `h_u`,
    /// whose saturated budget already carries the `muu` term); the
    /// remainder is clamped at zero so floating-point at that boundary
    /// answers a zero step — which [`crate::sizing::ceil_count`]
    /// refuses typed — rather than a NaN certificate.
    pub(crate) fn step_v_at(&self, hu: f64, delta_s: f64) -> f64 {
        let rem = (-self.muu).mul_add(hu.powi(2), delta_s).max(0.0);
        if self.mvv == 0.0 {
            if self.muv == 0.0 {
                f64::INFINITY
            } else {
                rem / (2.0 * self.muv * hu)
            }
        } else {
            let b = self.muv * hu;
            (b.mul_add(b, self.mvv * rem).sqrt() - b) / self.mvv
        }
    }
}

/// One chosen point of the split selection
/// ([`NurbsFaceBound::split_steps`]): the two UV steps, plus whether
/// the 3-D aspect cap is what bound the choice — the meter's
/// constraint-activity indicator (spec D-3), reported rather than
/// re-derived because the selection rule lives only here.
#[derive(Clone, Copy, Debug)]
pub(crate) struct SplitSteps {
    /// The `u` step.
    pub hu: f64,
    /// The `v` step.
    pub hv: f64,
    /// The [`ASPECT_CAP`] clamped the optimum (constraint-active).
    pub cap: bool,
}

/// **The 3-D aspect cap `A = 16`** on the split selection's grid cell,
/// measured through the first fundamental form
/// ([`NurbsFaceBound::split_steps`]).
///
/// RATIFIED at 16 (docs/TESS-BUDGET.md "The split schedule's aspect
/// policy", PR #568): one octave beyond the 4–8 range typical mesh
/// quality bounds tolerate, which captures most of the measured ~4.1x
/// split slack on ruled walls — where the honest optimum is mildly
/// anisotropic — while refusing the degenerate strip the unconstrained
/// optimum degenerates to (leaf_a f2: 70×328 → 1×4905, parameter
/// aspect ~5·10³). Mesh quality is a consumer contract; nothing
/// downstream was polled on strips. The dial is re-tunable by ordinary
/// measurement + baseline re-cut, and it is NOT the only bound on
/// anisotropy: the realized-lattice sliver line
/// ([`SAFE_ASPECT`], a DIFFERENT quantity — post-`ceil` parameter
/// spacing, not 3-D shape) stays in force over this selection and on
/// ruled walls generally binds first, through the snap alignment
/// [`NurbsCellGrid::band_schedule`] argues.
///
/// **What is guarded, and what is deliberately not** (the
/// measured-claim rule): the cap is a bound on the SELECTION — the
/// chosen `(h_u, h_v)`'s sup-mapped edge ratio — and THAT claim has a
/// mechanical guard, the fuzz row
/// `split_steps_stay_on_the_ellipse_and_inside_the_cap` (red on any
/// point outside the window, replayable seed). The EMITTED lattice's
/// FFF aspect carries no guard and no register on purpose: the snap
/// projection may legitimately move a band off the chosen point in
/// either direction when the sliver bound binds first (the
/// `snapped`/`snap_bands` indicator says where), so an emitted-side
/// `≤ A` assertion would be asserting a claim the design does not
/// make. Emitted-side measurements (the TESS-SPLIT PR's per-band
/// numbers) are evidence about a head, not an invariant; the
/// re-measured register for the emitted lattice is the budget CSV's
/// `realized_aspect` column (parameter aspect, the sliver line's own
/// quantity).
pub(crate) const ASPECT_CAP: f64 = 16.0;

fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}

/// One knot-span cell's own certified Hessian bound (the
/// [`nurbs_face_bound`] assembly restricted to the cell's active
/// coefficient window) with the UV rectangle it is valid on.
#[derive(Clone, Copy, Debug)]
pub(crate) struct CellBound {
    /// The cell's `u` extent, `[lo, hi]`.
    pub u: (f64, f64),
    /// The cell's `v` extent, `[lo, hi]`.
    pub v: (f64, f64),
    /// The cell-local bound — same units, same meaning, same consumer
    /// (`grid_steps`, `cert`) as the whole-patch one.
    pub bound: NurbsFaceBound,
}

/// The shared `Σ sup² → sup` collapse: `√hi`, rounded out. Poison
/// answers NaN, which every consumer treats as "unbounded/poisoned".
///
/// **An exactly-zero enclosure collapses to exactly `0.0`** — sound
/// (the sup of the zero enclosure IS zero; `next_up` exists to cover
/// `sqrt` rounding, and `√0` does not round) and load-bearing: the
/// split selection's degenerate-direction predicates
/// ([`NurbsFaceBound::split_steps`]) are decided on `== 0.0`, so the
/// structurally-exact zero of a degree-1 direction must not leave here
/// as subnormal dust.
fn cell_component(sq: RingInterval) -> f64 {
    let hi = sq.hi();
    if hi == 0.0 { 0.0 } else { hi.sqrt().next_up() }
}

/// **The per-cell bounds** (TESS-SPAN, promoted from the #320 sizing
/// diagnostic): the same certified assembly as [`nurbs_face_bound`],
/// reported per knot-span cell instead of maxed over the patch.
///
/// Since TESS-SPAN this is the SHIPPED lane's sizing input (through
/// [`nurbs_cell_grid`]); `crate::budget`'s meter reads it too, for the
/// span-sizing prediction it checks the lane against. Nothing here can
/// loosen a certificate: every cell's bound is the same hull assembly
/// over a SUBSET of the whole-patch coefficient window.
///
/// Granularity differs by arm, because each arm reports at the
/// granularity its own certified assembly already works in: the
/// integral arm's cells are the raw knot spans, the rational arm's are
/// the cells of the fixed [`patch_bound::RATIONAL_CERT_SPLITS`] refinement. The
/// budget row carries the cell count, so a reader is never guessing
/// which.
///
/// The max over the returned cells IS the face bound
/// ([`folded_face_bound`] folds exactly these cells); this module's own
/// `no_cell_exceeds_the_whole_patch_bound` test asserts the inequality
/// componentwise and `cert10_whole_face_bound_is_the_per_cell_fold`
/// asserts the equality.
///
/// # Errors
///
/// As [`nurbs_face_bound`] — same gates, same arms, same refusals.
pub(crate) fn nurbs_cell_bounds(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<Vec<CellBound>, TessellateError> {
    Ok(patch_bound::patch_cells(n)
        .map_err(|e| face_err(fk, e))?
        .into_iter()
        .map(|c| CellBound {
            u: c.u,
            v: c.v,
            bound: {
                let r = cell_readings(&c);
                NurbsFaceBound {
                    muu: cell_component(r[0]),
                    muv: cell_component(r[1]),
                    mvv: cell_component(r[2]),
                    mu1: cell_component(r[3]),
                    mv1: cell_component(r[4]),
                }
            },
        })
        .collect())
}

/// One tessellation's memo of certified whole-patch NURBS bounds, one
/// entry per described NURBS face.
///
/// It lives HERE, beside the assembly it remembers, rather than in
/// either pass that reads it: [`crate::chords`]' adjacent-face
/// tightening and [`crate::trimmed`]'s band schedule both need the
/// same per-face fact, and a cache hosted inside one of its two
/// consumers is the shape that drifts.
pub(crate) type FaceBounds = std::collections::HashMap<FaceKey, NurbsFaceBound>;

/// A described NURBS face's certified whole-patch bound, assembled on
/// first ask and remembered for the rest of the tessellation.
///
/// The assembly is the most expensive thing either pass does and its
/// answer is a per-face fact, so one memo threaded from
/// [`crate::tessellate()`] through both passes makes it one assembly
/// per face per tessellation instead of one per pass.
///
/// # Errors
///
/// As [`nurbs_face_bound`] — a face outside the certified inventory
/// refuses here exactly as it would there, on the first ask and (from
/// the memo's absence) on every later one.
pub(crate) fn face_bound(
    memo: &mut FaceBounds,
    payload: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    match memo.get(&fk) {
        Some(&b) => Ok(b),
        None => {
            let b = nurbs_face_bound(payload, fk)?;
            memo.insert(fk, b);
            Ok(b)
        }
    }
}

/// The realized-anisotropy line beyond which a band snaps to the
/// patch column count ([`NurbsCellGrid::band_schedule`] derives the
/// sliver certificate `(aspect² + 1)/8 · δ_s` and what happens at the
/// line).
///
/// **5.0 is a MEASURED constant, not a derived one** (dual review of
/// PR #594, MAJ-3): the sliver formula's own "certifies under δ" line
/// is `aspect ≤ √15 ≈ 3.87` — in the gap `(3.87, 5]` a worst-case
/// off-lattice sliver can certify up to ~1.6·δ, and what holds the
/// lane there is measured margin (worst tour face certificate
/// 0.60·δ) plus the typed [`TessellateError::CertificateExceeded`]
/// refusal as the backstop — a bad mesh is unrepresentable either
/// way; the constant only trades snap cost against refusal risk.
/// Measured at 5.0: the tour's certificates hold with the refinement
/// arm cold, and the #316 leaf's 12:1 band and the split-shadow
/// interfaces (the classes the snap exists for) sit above the line —
/// dropping to the derived 3.87 was measured to cost a large share of
/// the span gain for margin the tour does not need. Malignity is
/// judged on REALIZED spacings (`s_u/s_v`, post-`ceil`, up to ~2x the
/// ideal-step aspect near one-step bands), so the line is applied to
/// the lattice that actually exists; moving the test from the ideal
/// step to the realized one cost +3.0% of the tour's cells (leaf_a
/// 3.35x → 3.10x, still over the acceptance line) for correctly
/// snapping the near-one-step bands the ideal-step test missed.
///
/// **What goes red, named — and scoped, because none of it is as wide as
/// it first reads** (issue #667's Q6). Three things, and they bracket the
/// constant only when taken together:
///
/// * UPWARD, mechanically: `band_schedule_snaps_on_realized_aspect` below
///   is malign at realized aspect 9.09, so raising this constant to 9.09
///   or above stops the fixture snapping and fails the row. It is
///   **one-sided** — lowering the constant leaves every assertion in that
///   row passing;
/// * DOWNWARD, and only as cost: lowering it snaps more bands, which grows
///   the mesh, which is what the same job's `tessellation-budget sweep` +
///   `tessellation-budget lint (gate)` catch — every tour scene re-measured
///   per face against `docs/tess-budget-data/tess-budget-baseline.csv`, a
///   grown budget failing the row. A scheduled register, not an assert;
/// * SOUNDNESS, on a fixed corpus: `ci.yml`'s `k-lint (gate)` also runs
///   `mesh budget meter + certificate falsifier (feature = budget)`, i.e.
///   `probe_review::z1_per_triangle_certificate_falsification`, which
///   resamples every emitted triangle against its own certificate — but
///   over **four NURBS fixtures at two δ**, not the tour. It falsifies
///   under-certification where it looks; it is not tour-wide coverage.
///
/// And what has NO guard, stated so the three above are not over-read: the
/// *worst tour face certificate 0.60·δ* above. One-shot, and nothing
/// computes with it — the refusal named beside it is what holds the gap.
pub(crate) const SAFE_ASPECT: f64 = 5.0;

/// One v-band of the shipped schedule ([`NurbsCellGrid::band_schedule`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct BandCounts {
    /// The band's clipped v-extent, low end.
    pub va: f64,
    /// The band's clipped v-extent, high end.
    pub vb: f64,
    /// Column count (`u` divisions of the band).
    pub nuc: usize,
    /// Row count (`v` divisions of the band).
    pub nvc: usize,
    /// The [`ASPECT_CAP`] clamped this band's step selection
    /// ([`NurbsFaceBound::split_steps`]) — the budget meter's
    /// constraint-activity indicator, A-cap kind.
    pub cap: bool,
    /// The malign-band snap PROJECTED this band onto the patch
    /// column schedule and its counts CHANGED (either direction:
    /// columns added, or columns traded for rows) — the indicator's
    /// sliver/snap kind. `false` for a near-malign band whose own
    /// counts already sat on the patch schedule.
    pub snapped: bool,
}

/// **The shipped per-cell certificate table** (TESS-SPAN): one face's
/// [`nurbs_cell_bounds`] assembled into a tensor lookup — sorted cell
/// boundary values per direction, and each cell's own certified
/// Hessian bound. The trimmed lane sizes its grid from it (per-cell
/// steps, grid lines on the cell boundaries) and certifies each
/// triangle from it ([`Self::cert`]).
#[derive(Clone, Debug)]
pub(crate) struct NurbsCellGrid {
    /// Sorted cell boundary values in `u` (`cols + 1` entries); cell
    /// column `ci` covers `[u_cuts[ci], u_cuts[ci + 1])`, half-open —
    /// the same convention the certificate lookup argues from.
    u_cuts: Vec<f64>,
    /// Sorted cell boundary values in `v` (`rows + 1` entries).
    v_cuts: Vec<f64>,
    /// Per-cell bounds, u-major: `bounds[ci * rows + ri]`.
    bounds: Vec<NurbsFaceBound>,
}

impl NurbsCellGrid {
    /// **The whole-patch bound, folded off the cells already
    /// assembled** — componentwise max over this grid's own cells.
    ///
    /// Identical to [`nurbs_face_bound`] on the same face, and the
    /// identity is exact rather than approximate: that function hulls
    /// the per-cell squared-sum ENCLOSURES and collapses once, this one
    /// collapses per cell and takes the f64 max, and
    /// [`cell_component`] is monotone in the enclosure's `hi` — so
    /// `max_c sqrt(hi_c)` and `sqrt(max_c hi_c)` are the same f64. The
    /// NaN asymmetry that makes the ring-level accumulation
    /// load-bearing there does not arise here: [`nurbs_cell_grid`] has
    /// already refused a face with any non-finite cell.
    ///
    /// It exists so a caller that needs BOTH readings of a face pays
    /// for one assembly. The fold made the whole-patch bound a reading
    /// of the cells rather than a second pass over the net, and this is
    /// where that shows up as work not done.
    pub(crate) fn patch(&self) -> NurbsFaceBound {
        let mut m = NurbsFaceBound {
            muu: 0.0,
            muv: 0.0,
            mvv: 0.0,
            mu1: 0.0,
            mv1: 0.0,
        };
        for b in &self.bounds {
            m.muu = m.muu.max(b.muu);
            m.muv = m.muv.max(b.muv);
            m.mvv = m.mvv.max(b.mvv);
            m.mu1 = m.mu1.max(b.mu1);
            m.mv1 = m.mv1.max(b.mv1);
        }
        m
    }
}

/// The shipped lane's entry to per-cell sizing: [`nurbs_cell_bounds`]
/// finite-checked cell by cell (the [`nurbs_face_bound`] refusal,
/// applied where the shipped consumer now reads) and assembled into a
/// [`NurbsCellGrid`].
///
/// # Errors
///
/// As [`nurbs_cell_bounds`], plus the unbounded/poisoned refusal when
/// any single cell's bound fails the finite check.
pub(crate) fn nurbs_cell_grid(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsCellGrid, TessellateError> {
    let cells = nurbs_cell_bounds(n, fk)?;
    for c in &cells {
        let b = c.bound;
        if !(b.muu.is_finite()
            && b.muv.is_finite()
            && b.mvv.is_finite()
            && b.mu1.is_finite()
            && b.mv1.is_finite())
        {
            return Err(TessellateError::UnsupportedNurbsFace {
                face: fk,
                note: "NURBS face second-derivative hull is unbounded/poisoned — \
                       outside the certified inventory",
            });
        }
    }
    Ok(NurbsCellGrid::from_cells(&cells))
}

impl NurbsCellGrid {
    /// Assembles the tensor lookup. Both arms of [`nurbs_cell_bounds`]
    /// emit one cell per (nonempty u-span × nonempty v-span) of their
    /// own knot structure, so the cell rectangles ARE a tensor grid;
    /// the asserts are the fail-loud statement of that invariant, not
    /// a recovery path.
    fn from_cells(cells: &[CellBound]) -> Self {
        let mut u_cuts: Vec<f64> = cells.iter().flat_map(|c| [c.u.0, c.u.1]).collect();
        let mut v_cuts: Vec<f64> = cells.iter().flat_map(|c| [c.v.0, c.v.1]).collect();
        for cuts in [&mut u_cuts, &mut v_cuts] {
            cuts.sort_unstable_by(f64::total_cmp);
            cuts.dedup();
        }
        let cols = u_cuts.len().saturating_sub(1);
        let rows = v_cuts.len().saturating_sub(1);
        assert_eq!(
            cells.len(),
            cols * rows,
            "nurbs_cell_bounds emitted a non-tensor cell layout — kernel bug"
        );
        let nan = NurbsFaceBound {
            muu: f64::NAN,
            muv: f64::NAN,
            mvv: f64::NAN,
            mu1: f64::NAN,
            mv1: f64::NAN,
        };
        let mut bounds = vec![nan; cols * rows];
        let mut filled = vec![false; cols * rows];
        for c in cells {
            // The cell's own corner is a member of the cut set by
            // construction, so partition_point lands exactly on it.
            let ci = u_cuts.partition_point(|x| x.total_cmp(&c.u.0).is_lt());
            let ri = v_cuts.partition_point(|x| x.total_cmp(&c.v.0).is_lt());
            assert!(
                u_cuts.get(ci).copied() == Some(c.u.0)
                    && u_cuts.get(ci + 1).copied() == Some(c.u.1)
                    && v_cuts.get(ri).copied() == Some(c.v.0)
                    && v_cuts.get(ri + 1).copied() == Some(c.v.1),
                "nurbs_cell_bounds emitted overlapping cells — kernel bug"
            );
            let idx = ci * rows + ri;
            assert!(
                !filled[idx],
                "duplicate cell in nurbs_cell_bounds — kernel bug"
            );
            filled[idx] = true;
            bounds[idx] = c.bound;
        }
        // Count + no-duplicates + in-range ⇒ every slot filled; stated
        // anyway so a violation names itself.
        assert!(
            filled.iter().all(|f| *f),
            "nurbs_cell_bounds left a tensor slot empty — kernel bug"
        );
        Self {
            u_cuts,
            v_cuts,
            bounds,
        }
    }

    /// Cell boundary values in `u`. The schedule consumers go
    /// through [`Self::band_schedule`], so only the tests read this;
    /// the certificate lookup uses the field directly.
    #[cfg(test)]
    fn u_cuts(&self) -> &[f64] {
        &self.u_cuts
    }

    /// Cell boundary values in `v`. As [`Self::u_cuts`]: the schedule
    /// consumers go through [`Self::band_schedule`] now, so only the
    /// tests read this.
    #[cfg(test)]
    fn v_cuts(&self) -> &[f64] {
        &self.v_cuts
    }

    /// The certified bound of cell `(ci, ri)`.
    pub(crate) fn bound(&self, ci: usize, ri: usize) -> NurbsFaceBound {
        self.bounds[ci * (self.v_cuts.len() - 1) + ri]
    }

    /// Every cell of the table, in the u-major order
    /// [`nurbs_cell_bounds`] emits — the assembly read back out, so a
    /// consumer that needs the per-cell bounds does not run the
    /// assembly a second time to get them.
    ///
    /// The budget meter is that consumer and it is opt-in, so in a
    /// default build nothing calls this.
    #[cfg_attr(not(feature = "budget"), allow(dead_code))]
    pub(crate) fn cells(&self) -> impl Iterator<Item = CellBound> + '_ {
        let rows = self.v_cuts.len() - 1;
        let cols = self.u_cuts.len() - 1;
        (0..cols).flat_map(move |ci| {
            (0..rows).map(move |ri| CellBound {
                u: (self.u_cuts[ci], self.u_cuts[ci + 1]),
                v: (self.v_cuts[ri], self.v_cuts[ri + 1]),
                bound: self.bound(ci, ri),
            })
        })
    }

    /// The SIZING bound of the v-band `ri` (the ROW schedule's
    /// input): the componentwise max over the band's cells across all
    /// of `u`. A row line runs the full trim box, so its spacing
    /// answers to the band's WORST cell — and to nothing more: rows
    /// land exactly on the band cuts, so a grid triangle never
    /// crosses a band (a ±1-band dilation was measured to cost ~a
    /// third of the span gain for insurance the refinement ladder
    /// already provides). It is a HEURISTIC, exactly as the whole
    /// schedule is (module docs: the certificate is the guarantee) —
    /// the per-triangle certificate, taken from the raw per-cell
    /// bounds of every covered cell, still refuses loudly if a
    /// triangle reaches further.
    fn row_bound(&self, ri: usize) -> NurbsFaceBound {
        let cols = self.u_cuts.len() - 1;
        let mut m = NurbsFaceBound {
            muu: 0.0,
            muv: 0.0,
            mvv: 0.0,
            mu1: 0.0,
            mv1: 0.0,
        };
        for c in 0..cols {
            let b = self.bound(c, ri);
            m.muu = m.muu.max(b.muu);
            m.muv = m.muv.max(b.muv);
            m.mvv = m.mvv.max(b.mvv);
            m.mu1 = m.mu1.max(b.mu1);
            m.mv1 = m.mv1.max(b.mv1);
        }
        m
    }

    /// The v-band index containing `v` (clamped as [`Self::cell_lo`]).
    fn row_of(&self, v: f64) -> usize {
        Self::cell_lo(&self.v_cuts, v)
    }

    /// **The shipped band schedule** (TESS-SPAN): the trim box cut at
    /// interior band boundaries, each band's `(nuc, nvc)` counts —
    /// one derivation consumed by BOTH the trimmed lane's candidate
    /// generation and the budget meter's prediction, so the two
    /// cannot drift.
    ///
    /// Per band: `nvc` from the band bound's own `h_v`; `nuc` from
    /// the band bound's own `h_u` — EXCEPT that a MALIGN band
    /// (realized aspect `s_u/s_v` beyond [`SAFE_ASPECT`]) and its
    /// immediate neighbours are PROJECTED onto the patch column
    /// schedule: `nuc := patch_nuc` EXACTLY (equality with the chord
    /// pass's count is what alignment means — a mere max reintroduces
    /// the misaligned-interface sliver, executed on the #320 s_duct;
    /// the snap site carries the derivation), and `nvc` re-derived
    /// from the band's own ellipse at that column spacing
    /// ([`NurbsFaceBound::step_v_at`]). The projection can therefore
    /// move a band's counts in EITHER direction — more columns than
    /// its own optimum wanted, or fewer columns paid for with rows —
    /// and every emitted cell remains a point of the band's own
    /// certified region either way. The steps come from the split
    /// selection [`NurbsFaceBound::split_steps`] (TESS-SPLIT: the
    /// aspect-capped cell minimizer; `patch_nuc` derives from the
    /// whole-patch bound through the SAME selection, one derivation
    /// with the chord pass).
    /// The two aspect bounds are DIFFERENT quantities and both bind:
    /// the selection caps the chosen cell's 3-D shape at
    /// [`ASPECT_CAP`]; this snap judges the emitted lattice's
    /// post-`ceil` parameter spacing against [`SAFE_ASPECT`], and on
    /// ruled walls — where the capped optimum is still far more
    /// anisotropic than the sliver line tolerates — it is the snap's
    /// alignment, not spacing, that keeps off-lattice slivers out.
    ///
    /// **Why (measured, three times over)**: any point of an
    /// anisotropic lattice strip that is not ON a column admits an
    /// empty circumcircle reaching up to a full column spacing past
    /// it — a Delaunay-legal sliver whose certificate is
    /// ~`(aspect² + 1)/8 · δ_s`, malign beyond ~aspect 4. Band
    /// interfaces deliver exactly such points (the neighbour band's
    /// columns sit on the shared cut line), and no local insertion
    /// cures it (anchors and centroids each re-admit the circle
    /// beside themselves — both measured on the #316 leaf). Snapping
    /// a malign band AND its neighbours to the patch count makes
    /// those interfaces coincide column-for-column — the phase
    /// alignment the uniform schedule had everywhere, restored
    /// exactly where it is load-bearing — and the patch count is also
    /// the chord pass's schedule, so a full-width iso rim on a malign
    /// band lands its chord points ON the columns as before
    /// (`chords::nurbs_tighten`, the D-2 whole-patch arm). Benign
    /// interfaces keep their own counts: below the derived
    /// `√15 ≈ 3.87` line a foreign point's sliver certifies under δ
    /// outright; in the measured `(3.87, SAFE_ASPECT]` gap the tour's
    /// margin holds it (SAFE_ASPECT docs) and the certificate refusal
    /// plus the refinement ladder backstop the rest.
    ///
    /// # Errors
    ///
    /// [`TessellateError::ResolutionOverflow`] via
    /// [`crate::sizing::ceil_count`], as the uniform schedule.
    pub(crate) fn band_schedule(
        &self,
        patch: NurbsFaceBound,
        u: (f64, f64),
        v: (f64, f64),
        delta_s: f64,
    ) -> Result<Vec<BandCounts>, TessellateError> {
        let du = u.1 - u.0;
        let mut edges = vec![v.0];
        edges.extend(self.v_cuts.iter().copied().filter(|c| *c > v.0 && *c < v.1));
        edges.push(v.1);
        let (phu, _) = patch.grid_steps(delta_s);
        let patch_nuc = crate::sizing::ceil_count(du, phu)?;
        let mut bands = Vec::with_capacity(edges.len().saturating_sub(1));
        let mut row_bounds = Vec::with_capacity(edges.len().saturating_sub(1));
        for w in edges.windows(2) {
            let (va, vb) = (w[0], w[1]);
            // The band is found by the slab midpoint — strictly inside
            // one band, since no interior cut crosses a slab.
            let bound = self.row_bound(self.row_of(0.5 * (va + vb)));
            let steps = bound.split_steps(delta_s);
            let nuc = crate::sizing::ceil_count(du, steps.hu)?;
            let nvc = crate::sizing::ceil_count(vb - va, steps.hv)?;
            bands.push(BandCounts {
                va,
                vb,
                nuc,
                nvc,
                cap: steps.cap,
                snapped: false,
            });
            row_bounds.push(bound);
        }
        // Malignity is judged on the REALIZED spacings `s_u/s_v`, not
        // the pre-`ceil` ideal steps: the lattice a sliver lives in
        // has rows every `s_v = (vb−va)/nvc ≤ h_v`, so testing against
        // `h_v` under-estimates the aspect by up to ~2x whenever a
        // band's extent barely exceeds one step (R2's review fixture,
        // pinned in the tests below).
        //
        // EVERY band is judged, `nuc = 1` included (TESS-SPLIT): a
        // band that does not subdivide `u` still meets its neighbours'
        // columns on the shared cut lines, and beside a full-width
        // `s_u = du` lattice those foreign points admit the same
        // `(aspect²+1)/8·δ_s` sliver — executed on the #320 leaf under
        // the aspect-capped selection, whose `nuc = 1` bands beside
        // snapped `nuc = 3` bands certified a 41·δ sliver and refused
        // the face. The retired selection never realized an
        // anisotropic `nuc = 1` band, which is the only reason a
        // `nuc >= 2` exemption survived here.
        #[allow(clippy::cast_precision_loss)]
        let is_malign = |b: &BandCounts| {
            let su = du / b.nuc as f64;
            let sv = (b.vb - b.va) / b.nvc as f64;
            sv.is_finite() && sv > 0.0 && su > SAFE_ASPECT * sv
        };
        // The snap: every near-malign band takes the patch column
        // count EXACTLY — `patch_nuc` is the chord pass's schedule
        // (one derivation), so equality is what puts every band of the
        // group AND the boundary chord points on one column family.
        // Under the retired selection every band's own count was `<=
        // patch_nuc` by componentwise bound dominance and a
        // max-with-patch was the same thing; the aspect-capped
        // selection loses that monotonicity (each band optimizes
        // inside its OWN first-fundamental-form window, so a band can
        // honestly want MORE columns than the patch point — executed
        // on the #320 s_duct: a band's own `nuc = 4` snapped-by-max
        // beside rim chords at `patch_nuc = 3` certified a 25·δ rim
        // sliver and refused the face). A band whose columns are
        // WIDENED to the patch spacing pays rows for it: its `h_v` is
        // re-projected onto its own ellipse at `h_u = du/patch_nuc`
        // ([`NurbsFaceBound::step_v_at`]), so every emitted cell is
        // still a point of that band's certified region.
        //
        // Membership runs to a FIXPOINT: projection changes realized
        // spacings, so malignity is re-judged and the ±1-neighbour
        // dilation re-applied until the group is closed — at exit,
        // every neighbour of a malign band is in the group, so every
        // high-aspect interface is column-aligned and every interface
        // with a band outside the group is benign on both sides
        // (foreign points beside a benign lattice certify under δ —
        // the SAFE_ASPECT argument). TERMINATION: membership only
        // ever grows and projection is idempotent (a member is never
        // re-projected), so the loop exits within `bands.len()`
        // passes — each pass either admits a new member or breaks.
        let n = bands.len();
        let mut member = vec![false; n];
        loop {
            let malign: Vec<bool> = bands.iter().map(is_malign).collect();
            let mut grew = false;
            for i in 0..n {
                if member[i] {
                    continue;
                }
                if malign[i]
                    || (i > 0 && malign[i - 1])
                    || malign.get(i + 1).copied().unwrap_or(false)
                {
                    member[i] = true;
                    grew = true;
                    #[allow(clippy::cast_precision_loss)]
                    let s_u = du / patch_nuc as f64;
                    let hv = row_bounds[i].step_v_at(s_u, delta_s);
                    let nvc = crate::sizing::ceil_count(bands[i].vb - bands[i].va, hv)?;
                    let b = &mut bands[i];
                    b.snapped = b.nuc != patch_nuc || b.nvc != nvc;
                    b.nuc = patch_nuc;
                    b.nvc = nvc;
                }
            }
            if !grew {
                break;
            }
        }
        Ok(bands)
    }

    /// The index of the half-open cell `[cut_i, cut_{i+1})` containing
    /// `x`, clamped to the covered range (a trim polygon evaluated
    /// through pcurve arithmetic can stray an ulp outside the chart
    /// rectangle; the edge cell's bound is the honest answer there —
    /// the surface is not defined beyond it).
    fn cell_lo(cuts: &[f64], x: f64) -> usize {
        cuts.partition_point(|c| *c <= x)
            .saturating_sub(1)
            .min(cuts.len().saturating_sub(2))
    }

    /// As [`Self::cell_lo`] for a box's UPPER end: an end exactly on a
    /// cut belongs to the cell BELOW it — the box only touches the cut
    /// itself, a measure-zero set the Taylor remainder never charges
    /// (see [`Self::cert`]).
    fn cell_hi(cuts: &[f64], x: f64) -> usize {
        cuts.partition_point(|c| *c < x)
            .saturating_sub(1)
            .min(cuts.len().saturating_sub(2))
    }

    /// The per-triangle deviation certificate `Q/4`
    /// ([`NurbsFaceBound::cert`]) with the componentwise sup of the
    /// second partials over the triangle's UV box, read from the cells
    /// the box covers.
    ///
    /// **Why per-cell bounds certify across half-open cell
    /// boundaries**: the `Q/4` derivation is the integral Taylor
    /// remainder (module docs), which needs only C¹ plus an a.e.
    /// second-derivative bound along the segments it integrates. A
    /// knot is exactly where a C¹ surface's second derivative jumps,
    /// but the cell boundaries are measure-zero, and each cell's hull
    /// bounds its polynomial piece on the cell's CLOSURE — so the
    /// componentwise max over the covered cells dominates the second
    /// partials a.e. on the whole box. This is the same fact the
    /// whole-patch assembly has always rested on at its own interior
    /// knots. (The componentwise max is the CONSERVATIVE choice of two
    /// sound bounds: the max of the per-cell certificates also bounds
    /// the remainder — pointwise, the integrand is the local cell's
    /// form at the fixed direction, so the max of the per-cell forms
    /// dominates it along every segment, with the same constant — and
    /// is strictly tighter on straddling boxes (dual review of PR
    /// #594, settled by derivation and dense numeric sup). The shipped
    /// semantics is the componentwise sup, pinned bit-for-bit by
    /// `cert_is_pinned_to_the_componentwise_sup`; adopting the tighter
    /// bound is possible future work, worth little — straddlers are
    /// boundary fans only.)
    ///
    /// For the common case — the trimmed lane places its grid lines on
    /// the cell boundaries, so a grid triangle's box lies inside ONE
    /// cell — this is exactly that cell's own certificate.
    pub(crate) fn cert(&self, uv: [[f64; 2]; 3]) -> f64 {
        let u_lo = min3(uv[0][0], uv[1][0], uv[2][0]);
        let u_hi = max3(uv[0][0], uv[1][0], uv[2][0]);
        let v_lo = min3(uv[0][1], uv[1][1], uv[2][1]);
        let v_hi = max3(uv[0][1], uv[1][1], uv[2][1]);
        let ci0 = Self::cell_lo(&self.u_cuts, u_lo);
        let ci1 = Self::cell_hi(&self.u_cuts, u_hi).max(ci0);
        let ri0 = Self::cell_lo(&self.v_cuts, v_lo);
        let ri1 = Self::cell_hi(&self.v_cuts, v_hi).max(ri0);
        let mut m = NurbsFaceBound {
            muu: 0.0,
            muv: 0.0,
            mvv: 0.0,
            mu1: 0.0,
            mv1: 0.0,
        };
        for ci in ci0..=ci1 {
            for ri in ri0..=ri1 {
                let b = self.bound(ci, ri);
                m.muu = m.muu.max(b.muu);
                m.muv = m.muv.max(b.muv);
                m.mvv = m.mvv.max(b.mvv);
            }
        }
        m.cert(uv)
    }
}

/// The certified Hessian sup bounds of a described NURBS face, or the
/// typed refusal naming its class (module docs).
///
/// ONE arm for both patch classes: the fold over
/// [`patch_bound::patch_cells`]' per-cell enclosures
/// ([`folded_face_bound`]). Which assembly produced those cells is the
/// patch's own business — the plain hull assembly for an integral net,
/// the quotient rule over the homogeneous nets for a rational one
/// (module docs, "The rational arm").
///
/// # Errors
///
/// [`TessellateError::UnsupportedNurbsFace`] — C⁰-creased, degree-0,
/// an illegal rational description (non-positive/non-finite weight),
/// or a poisoned/unbounded hull. The PLACEHOLDER is the caller's check
/// (it refuses `UnsupportedSurface`, the mvfs state's historical
/// variant).
pub(crate) fn nurbs_face_bound(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    patch_bound::check_direction(n.knots_u()).map_err(|e| face_err(fk, e))?;
    patch_bound::check_direction(n.knots_v()).map_err(|e| face_err(fk, e))?;
    let bound = folded_face_bound(n, fk)?;
    if !(bound.muu.is_finite()
        && bound.muv.is_finite()
        && bound.mvv.is_finite()
        && bound.mu1.is_finite()
        && bound.mv1.is_finite())
    {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "NURBS face second-derivative hull is unbounded/poisoned — \
                   outside the certified inventory",
        });
    }
    Ok(bound)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Point3;
    use geom_core::Tol;
    use geom_core::spline::KnotVector;
    use profile::RawLoop;
    use test_utils::fuzz;

    /// A wavy degree-2×3 integral net on [0,1]² (nothing symmetric, so
    /// every second partial is genuinely nonzero).
    fn wavy() -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let (x, y) = (i as f64 * 0.7, j as f64 * 0.5);
                control.push(Point3::new(x, y, (1.3 * x + 0.9 * y).sin() + 0.3 * x * y));
            }
        }
        let w = vec![1.0; control.len()];
        NurbsSurface::new(kv_u, kv_v, control, w).unwrap()
    }

    /// The hull bound DOMINATES every sampled second partial — the
    /// convexity claim, measured (never the other way round).
    #[test]
    fn hessian_hull_dominates_sampled_second_partials() {
        let s = wavy();
        let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered class");
        assert!(b.muu > 0.0 && b.muv > 0.0 && b.mvv > 0.0);
        let n = 41;
        let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
        for i in 0..=n {
            for j in 0..=n {
                let jet = s.ders(f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                wuu = wuu.max(jet.duu.norm());
                wuv = wuv.max(jet.duv.norm());
                wvv = wvv.max(jet.dvv.norm());
            }
        }
        assert!(
            wuu > 0.0 && wuu <= b.muu,
            "sup|S_uu| {wuu} vs hull {}",
            b.muu
        );
        assert!(
            wuv > 0.0 && wuv <= b.muv,
            "sup|S_uv| {wuv} vs hull {}",
            b.muv
        );
        assert!(
            wvv > 0.0 && wvv <= b.mvv,
            "sup|S_vv| {wvv} vs hull {}",
            b.mvv
        );
    }

    /// The certificate arithmetic: Q/4 with the box extents.
    ///
    /// DEMOTED to a formula FREEZE (MIN-1, the #218 review): this
    /// mirrors the implementation and can only catch accidental
    /// edits, tautologically — the review's planted 0.25 → 0.05 cert
    /// weakening sailed past the aggregate δ+ε pin and died only
    /// here. The GUARD against under-certification is the empirical
    /// per-triangle falsifier (`probe_review::z1`, armed through
    /// `budget::arm`), which kills the same plant on measured
    /// deviations. Kept as a cheap freeze; never cite it as evidence
    /// the bound is honest.
    #[test]
    fn cert_is_the_documented_quarter_q() {
        let b = NurbsFaceBound {
            muu: 2.0,
            muv: 3.0,
            mvv: 5.0,
            mu1: 1.0,
            mv1: 1.0,
        };
        let uv = [[0.0, 0.0], [0.1, 0.0], [0.0, 0.2]];
        let q = 2.0 * 0.01 + 2.0 * 3.0 * 0.1 * 0.2 + 5.0 * 0.04;
        assert!((b.cert(uv) - 0.25 * q).abs() < 1e-15);
    }

    /// A planar bilinear quad is flat: every second-derivative
    /// enclosure assembles to the EXACT zero (the differences of these
    /// coordinates are exact in f64), which [`cell_component`]
    /// preserves as `0.0` — the decided degenerate-direction predicate
    /// — and both grid steps come out unconstrained. A net whose
    /// arithmetic does round assembles to dust instead, which is the
    /// conservative side.
    #[test]
    fn planar_bilinear_bounds_collapse() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(b.muu < 1e-100 && b.muv < 1e-12 && b.mvv < 1e-100);
        let (hu, hv) = b.grid_steps(1e-3);
        // Effectively unconstrained: one cell across any real chart
        // rectangle (spans are O(1)).
        assert!(hu > 1e3 && hv > 1e3);
    }

    /// A TWISTED bilinear quad has S_uv = ΔΔP exactly; the hull pins
    /// that constant to outward rounding (degree-1 directions
    /// contribute exact zeros to uu/vv).
    #[test]
    fn twisted_bilinear_mixed_term_is_tight() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.5),
        ];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(b.muu < 1e-100 && b.mvv < 1e-100);
        // S_uv = P11 − P10 − P01 + P00 = (0, 0, 0.5); the hull only
        // ever widens.
        assert!(b.muv >= 0.5 && b.muv < 0.5 + 1e-12);
    }

    /// REVIEW Z2: degree-0 direction refuses typed (if constructible).
    #[test]
    fn probe_degree_zero_refuses_typed() {
        let Ok(kv_u) = KnotVector::clamped(vec![0.0, 1.0], 0) else {
            // Degree 0 cannot even be described — the gate is upstream.
            return;
        };
        let kv_v = KnotVector::unit_segment(1);
        let n = kv_u.control_count() * kv_v.control_count();
        let control = vec![Point3::new(0.0, 0.0, 0.0); n];
        let s = NurbsSurface::new(kv_u, kv_v, control, vec![1.0; n]).unwrap();
        match nurbs_face_bound(&s, FaceKey::default()) {
            Err(TessellateError::UnsupportedNurbsFace { note, .. }) => {
                assert!(note.contains("degree-0"));
            }
            other => panic!("expected typed refusal, got {other:?}"),
        }
    }

    /// REVIEW Z2 boundary: interior multiplicity EXACTLY p-1 is C^1 —
    /// the covered side of the gate — and the hull still dominates a
    /// dense sample straddling the knot (S_uu jumps there).
    #[test]
    fn probe_multiplicity_p_minus_one_is_covered_and_dominated() {
        for (p_deg, mult) in [(2usize, 1usize), (3, 2)] {
            let mut knots = vec![0.0; p_deg + 1];
            knots.extend(std::iter::repeat_n(0.5, mult));
            knots.extend(vec![1.0; p_deg + 1]);
            let kv_u = KnotVector::clamped(knots, p_deg).unwrap();
            let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
            let mut control = Vec::new();
            for i in 0..nu {
                for j in 0..nv {
                    let (x, y) = (i as f64 * 0.6, j as f64 * 0.8);
                    control.push(Point3::new(x, y, (2.1 * x - 1.3 * y).cos() + 0.4 * x * y));
                }
            }
            let s = NurbsSurface::new(kv_u, kv_v, control, vec![1.0; nu * nv]).unwrap();
            let b = nurbs_face_bound(&s, FaceKey::default())
                .expect("multiplicity p-1 is the covered side");
            let n = 160;
            let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
            for i in 0..=n {
                for j in 0..=n {
                    let jet = s.ders(f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                    wuu = wuu.max(jet.duu.norm());
                    wuv = wuv.max(jet.duv.norm());
                    wvv = wvv.max(jet.dvv.norm());
                }
            }
            assert!(
                wuu <= b.muu && wuv <= b.muv && wvv <= b.mvv,
                "deg {p_deg} mult {mult}: sampled ({wuu},{wuv},{wvv}) vs hull ({},{},{})",
                b.muu,
                b.muv,
                b.mvv
            );
        }
    }

    /// CONSCIOUS FLIP (M8-5): `rational_face_refuses_typed` re-derived
    /// as the positive row. The refusal's premise — "a rational second
    /// derivative is not a control-hull convexity fact" — was answered
    /// by the quotient-rule assembly over the HOMOGENEOUS nets (module
    /// docs), so the pin is now the bound's honesty: on adversarial
    /// rational patches, dense-sampled true second partials are
    /// DOMINATED by the certified sups, and the sups are real (> 0),
    /// never a fabricated zero.
    #[test]
    fn rational_face_bound_dominates_sampled_hessian() {
        // (a) The exact quarter cylinder: rational quadratic arc × line
        // (the arc-walled loft class M8-2 made buildable) — muu real,
        // mvv parameter-flat.
        let arc = quarter_cylinder();
        // (b) A wavy rational 2×3 patch with steep alternating weights
        // (nothing symmetric; every second partial nonzero, and the
        // weight ramps are what the cross terms have to survive).
        let wild = wavy_rational();
        for (name, s, all_positive) in [("quarter_cylinder", arc, false), ("wavy", wild, true)] {
            let b = nurbs_face_bound(&s, FaceKey::default()).expect("rational is covered now");
            let n = 80;
            let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
            for i in 0..=n {
                for j in 0..=n {
                    let jet = s.ders(f64::from(i) / f64::from(n), f64::from(j) / f64::from(n));
                    wuu = wuu.max(jet.duu.norm());
                    wuv = wuv.max(jet.duv.norm());
                    wvv = wvv.max(jet.dvv.norm());
                }
            }
            assert!(
                wuu <= b.muu && wuv <= b.muv && wvv <= b.mvv,
                "{name}: sampled ({wuu},{wuv},{wvv}) escapes the certified \
                 ({},{},{})",
                b.muu,
                b.muv,
                b.mvv
            );
            assert!(
                b.muu > 0.0,
                "{name}: muu must be real, not a fabricated zero"
            );
            if all_positive {
                assert!(b.muv > 0.0 && b.mvv > 0.0, "{name}: all partials real");
            }
        }
    }

    /// **The per-cell bounds are honest, cell by cell** — since
    /// TESS-SPAN the claim the SHIPPED grid schedule and per-triangle
    /// certificate rest on (and still the budget meter's span
    /// prediction), and the one that would be easy to get subtly wrong
    /// (an off-by-one in a derived window silently reports a cell as
    /// flatter than it is). Runs in the default lane for exactly that
    /// reason — the falsifier guards a shipped bound now, not a
    /// diagnostic.
    ///
    /// Falsified the same way the whole-patch bound is: dense-sample
    /// the TRUE second partials, but inside each cell's own rectangle,
    /// and require that cell's own sups to dominate them.
    #[test]
    fn every_cell_bound_dominates_its_own_sampled_hessian() {
        for (name, s) in [
            ("wavy", wavy()),
            ("quarter_cylinder", quarter_cylinder()),
            ("wavy_rational", wavy_rational()),
        ] {
            let cells = nurbs_cell_bounds(&s, FaceKey::default()).expect("covered");
            assert!(!cells.is_empty(), "{name}: no analysis cells");
            let n = 12;
            for (k, c) in cells.iter().enumerate() {
                let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
                // The cell is HALF-OPEN: a knot is where the second
                // derivative jumps (the surface is C¹, not C²), so the
                // closing corner belongs to the NEXT cell's window and
                // sampling it would falsify the wrong bound.
                let inside = |lo: f64, hi: f64, k: u32| {
                    lo + (hi - lo) * f64::from(k) / f64::from(n) * (1.0 - 1e-9)
                };
                for i in 0..=n {
                    for j in 0..=n {
                        let u = inside(c.u.0, c.u.1, i);
                        let v = inside(c.v.0, c.v.1, j);
                        let jet = s.ders(u, v);
                        wuu = wuu.max(jet.duu.norm());
                        wuv = wuv.max(jet.duv.norm());
                        wvv = wvv.max(jet.dvv.norm());
                    }
                }
                let b = c.bound;
                assert!(
                    wuu <= b.muu && wuv <= b.muv && wvv <= b.mvv,
                    "{name} cell {k} u{:?} v{:?}: sampled ({wuu:e},{wuv:e},{wvv:e}) escapes \
                     its own certified ({:e},{:e},{:e})",
                    c.u,
                    c.v,
                    b.muu,
                    b.muv,
                    b.mvv
                );
            }
        }
    }

    /// REVIEW PROBE (r1, adoptable): the first-derivative sups the
    /// aspect cap reads (`mu1`/`mv1`, TESS-SPLIT) get the SAME
    /// falsification rows their Hessian siblings have — sampled
    /// dominance per cell and whole-patch, and cell ≤ patch
    /// componentwise. Without this, a subtle `first_derivative_hull` /
    /// `s1u` window bug would misplace the aspect window with nothing
    /// going red (the cap is policy, so no certificate catches it).
    #[test]
    fn first_derivative_sups_dominate_samples_and_refine_upward() {
        for (name, s) in [
            ("wavy", wavy()),
            ("quarter_cylinder", quarter_cylinder()),
            ("wavy_rational", wavy_rational()),
        ] {
            let whole = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
            let cells = nurbs_cell_bounds(&s, FaceKey::default()).expect("covered");
            let n = 12;
            for (k, c) in cells.iter().enumerate() {
                assert!(
                    c.bound.mu1 <= whole.mu1 && c.bound.mv1 <= whole.mv1,
                    "{name} cell {k}: first-derivative sup exceeds the patch's"
                );
                let inside = |lo: f64, hi: f64, k: u32| {
                    lo + (hi - lo) * f64::from(k) / f64::from(n) * (1.0 - 1e-9)
                };
                let (mut wu, mut wv) = (0.0f64, 0.0f64);
                for i in 0..=n {
                    for j in 0..=n {
                        let jet = s.ders(inside(c.u.0, c.u.1, i), inside(c.v.0, c.v.1, j));
                        wu = wu.max(jet.du.norm());
                        wv = wv.max(jet.dv.norm());
                    }
                }
                assert!(
                    wu <= c.bound.mu1 && wv <= c.bound.mv1,
                    "{name} cell {k} u{:?} v{:?}: sampled speeds ({wu:e},{wv:e}) escape \
                     the certified ({:e},{:e})",
                    c.u,
                    c.v,
                    c.bound.mu1,
                    c.bound.mv1
                );
            }
        }
    }

    /// The per-cell bounds refine the whole-patch one, never exceed it
    /// — the whole-patch hull is taken over a superset of every cell's
    /// window, so a cell reporting MORE than the patch would mean the
    /// two assemblies disagree.
    #[test]
    fn no_cell_exceeds_the_whole_patch_bound() {
        for (name, s) in [
            ("wavy", wavy()),
            ("quarter_cylinder", quarter_cylinder()),
            ("wavy_rational", wavy_rational()),
        ] {
            let whole = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
            for c in nurbs_cell_bounds(&s, FaceKey::default()).expect("covered") {
                assert!(
                    c.bound.muu <= whole.muu
                        && c.bound.muv <= whole.muv
                        && c.bound.mvv <= whole.mvv,
                    "{name}: cell u{:?} v{:?} bound ({:e},{:e},{:e}) exceeds the patch's \
                     ({:e},{:e},{:e})",
                    c.u,
                    c.v,
                    c.bound.muu,
                    c.bound.muv,
                    c.bound.mvv,
                    whole.muu,
                    whole.muv,
                    whole.mvv
                );
            }
        }
    }

    /// **The shipped `cert` semantics, pinned bit-for-bit** (R1 MIN-2 /
    /// R2 MAJ-1 of the PR #594 dual review): the certificate of a
    /// straddling box IS the componentwise sup over covered cells fed
    /// through [`NurbsFaceBound::cert`] — asserted with exact values on
    /// an asymmetric fixture (one band `muu`-dominated, one
    /// `mvv`-dominated) where the max-of-per-cell-certificates
    /// alternative is strictly smaller, so substituting it goes RED
    /// here. The review settled by derivation and by dense numeric sup
    /// that max-of-cells is ALSO sound (per-segment form bound, same
    /// constant) — the shipped choice is the more conservative of two
    /// sound bounds, and THIS test pins which one ships.
    #[test]
    fn cert_is_pinned_to_the_componentwise_sup() {
        let mk = |muu: f64, mvv: f64| NurbsFaceBound {
            muu,
            muv: 0.0,
            mvv,
            mu1: 1.0,
            mv1: 1.0,
        };
        let cells = [
            CellBound {
                u: (0.0, 1.0),
                v: (0.0, 0.3),
                bound: mk(8.0, 0.0),
            },
            CellBound {
                u: (0.0, 1.0),
                v: (0.3, 0.7),
                bound: mk(0.5, 0.5),
            },
            CellBound {
                u: (0.0, 1.0),
                v: (0.7, 1.0),
                bound: mk(0.0, 8.0),
            },
        ];
        let grid = NurbsCellGrid::from_cells(&cells);
        // Straddles all three bands.
        let uv = [[0.1, 0.05], [0.9, 0.05], [0.5, 0.95]];
        let expected = mk(8.0, 8.0).cert(uv);
        assert_eq!(
            grid.cert(uv).to_bits(),
            expected.to_bits(),
            "shipped cert must BE the componentwise-sup bound, exactly"
        );
        let max_of_cells = cells
            .iter()
            .map(|c| c.bound.cert(uv))
            .fold(0.0f64, f64::max);
        assert!(
            expected > 1.5 * max_of_cells,
            "fixture must separate the semantics (componentwise {expected:e} vs \
             max-of-cells {max_of_cells:e}) or this test cannot catch a substitution"
        );
    }

    /// **`band_schedule` judges malignity on REALIZED spacings** (R2
    /// MAJ-2's executed fixture, adopted): a band whose extent barely
    /// exceeds one ideal step has `s_v = extent/nvc` down to ~half of
    /// `h_v`, so testing `s_u` against `SAFE_ASPECT·h_v` under-detects
    /// by up to ~2x. This fixture measures 4.79 against the ideal step
    /// (benign — the pre-fix test left `nuc = 11`) but 9.09 realized
    /// (malign): the band and its neighbour must be projected onto
    /// the patch column schedule (columns here; band 0 also trades
    /// its rows down through the ellipse re-projection).
    #[test]
    fn band_schedule_snaps_on_realized_aspect() {
        let cells = [
            CellBound {
                u: (0.0, 1.0),
                v: (0.0, 0.02),
                bound: NurbsFaceBound {
                    muu: 0.1108,
                    muv: 0.0,
                    mvv: 2.77,
                    mu1: 1.0,
                    mv1: 1.0,
                },
            },
            CellBound {
                u: (0.0, 1.0),
                v: (0.02, 1.0),
                bound: NurbsFaceBound {
                    muu: 0.1,
                    muv: 0.0,
                    mvv: 0.1,
                    mu1: 1.0,
                    mv1: 1.0,
                },
            },
        ];
        let grid = NurbsCellGrid::from_cells(&cells);
        // With every muv = 0 and unit speeds, the split selection's
        // interior optimum t* = √(mvv/muu) sits inside the aspect
        // window everywhere here (t* = 5.0, 1.0, ~1.18 vs [1/16, 16]),
        // and the chosen steps coincide with the retired grouping's —
        // the fixture pins the SNAP, not the selection.
        let patch = NurbsFaceBound {
            muu: 2.0,
            muv: 0.0,
            mvv: 2.77,
            mu1: 1.0,
            mv1: 1.0,
        };
        let delta_s = 2e-3;
        let bands = grid
            .band_schedule(patch, (0.0, 1.0), (0.0, 1.0), delta_s)
            .expect("schedules");
        assert_eq!(bands.len(), 2);
        // Band 0's own schedule: h_u ≈ 0.0950 → nuc 11; h_v ≈ 0.0190
        // with extent 0.02 → nvc 2, s_v = 0.01, s_u = 1/11 ≈ 0.0909:
        // realized aspect 9.09 > SAFE_ASPECT while 5·h_v = 0.095 >
        // s_u would have read it benign. Patch h_u ≈ 0.02236 → 45.
        assert_eq!(
            bands[0].nuc, 45,
            "the malign band snaps to the patch column count: {bands:?}"
        );
        assert_eq!(
            bands[1].nuc, 45,
            "the malign band's neighbour snaps too: {bands:?}"
        );
        // A snapped band's rows are RE-PROJECTED onto its own ellipse
        // at the patch column spacing (band 0's h_v relaxes from
        // ~0.0190 to ~0.0265 at s_u = 1/45, so its nvc falls to 1) —
        // and the post-snap lattice is benign, which is the fixpoint's
        // exit condition.
        assert_eq!(
            bands[0].nvc, 1,
            "rows re-projected at the snapped columns: {bands:?}"
        );
        assert!(bands[1].nvc >= 1);
        // The constraint-activity flags: the snap projected both
        // bands onto the patch schedule (sliver/snap kind), and the A
        // cap clamped neither (the interior optima above sit inside
        // the aspect window).
        assert!(
            bands[0].snapped && bands[1].snapped,
            "the snap must report itself: {bands:?}"
        );
        assert!(
            !bands[0].cap && !bands[1].cap,
            "no cap activity in this fixture: {bands:?}"
        );
    }

    /// **The split selection stays inside the certified ellipse and
    /// the aspect window** — the TESS-SPLIT counterpart of the meter's
    /// optimizer test, asserted on the ANSWER over random bounds,
    /// degenerate corners included (exact-zero directions, degenerate
    /// 3-D speeds).
    #[test]
    fn split_steps_stay_on_the_ellipse_and_inside_the_cap() {
        let mut rng = fuzz::start("nurbs_cert::split_steps_constraints");
        fn mag(r: &mut fuzz::Rng) -> f64 {
            if r.unit() < 0.2 {
                0.0
            } else {
                10.0f64.powf(r.range(-6.0, 4.0))
            }
        }
        for _ in 0..fuzz::scaled(500) {
            let delta_s = 10.0f64.powf(rng.range(-6.0, -1.0));
            let b = NurbsFaceBound {
                muu: mag(&mut rng),
                muv: mag(&mut rng),
                mvv: mag(&mut rng),
                mu1: mag(&mut rng),
                mv1: mag(&mut rng),
            };
            let s = b.split_steps(delta_s);
            // Ellipse membership, checked at a finite box (an
            // unconstrained ∞ step realizes as the box extent).
            let ext = 10.0f64.powf(rng.range(-2.0, 2.0));
            let (hu, hv) = (s.hu.min(ext), s.hv.min(ext));
            let q = b.muu * hu.powi(2) + 2.0 * b.muv * hu * hv + b.mvv * hv.powi(2);
            assert!(
                q <= delta_s * (1.0 + 1e-9),
                "chosen point violates the certificate: q={q:e} > {delta_s:e} for {b:?} — {}",
                fuzz::replay()
            );
            // Aspect-cap membership whenever the window exists and the
            // chosen steps are finite (the affine ∞ arm has no cell
            // shape to cap).
            if b.mu1 > 0.0 && b.mv1 > 0.0 && s.hu.is_finite() && s.hv.is_finite() {
                let aspect = (s.hu * b.mu1) / (s.hv * b.mv1);
                assert!(
                    ((1.0 - 1e-9) / ASPECT_CAP..=ASPECT_CAP * (1.0 + 1e-9)).contains(&aspect),
                    "3-D aspect {aspect:e} escapes the cap for {b:?} — {}",
                    fuzz::replay()
                );
            }
        }
    }

    /// The speed-ratio overflow/underflow corner: two finite positive
    /// first-derivative sups whose RATIO leaves the finite line (a
    /// face degenerate beyond any real geometry). The window filter
    /// answers "no window" there, so the selection falls back to its
    /// windowless arms and every returned step is non-NaN — total
    /// arithmetic, with `ceil_count`'s typed refusals as the only
    /// exit for an absurd count.
    #[test]
    fn rho_off_the_finite_line_drops_the_window_not_the_answer() {
        for (mu1, mv1) in [(5e-324, 1e300), (1e300, 5e-324)] {
            for (muu, mvv) in [(2.0, 51.3), (0.0, 51.3), (51.3, 0.0)] {
                let b = NurbsFaceBound {
                    muu,
                    muv: 2.4,
                    mvv,
                    mu1,
                    mv1,
                };
                let s = b.split_steps(1e-3);
                assert!(
                    !s.hu.is_nan() && !s.hv.is_nan(),
                    "NaN step at rho overflow: {b:?} -> ({:e}, {:e})",
                    s.hu,
                    s.hv
                );
                assert!(!s.cap, "no window means no cap activity: {b:?}");
            }
        }
    }

    /// **Row 4: the ruled wall's flat direction gets its exact arm,
    /// pinned against the generic formula's limit.** The exact arm's
    /// answer is asserted BITWISE against its own closed form, and the
    /// generic arm evaluated at muu = dust (the value the collapse
    /// used to leak) must agree to a stated 1e-6 relative bound — the
    /// arm is the limit's value, reached without the division by zero.
    #[test]
    fn ruled_wall_degenerate_arm_is_exact() {
        let delta_s = 1e-3;
        let (muv, mvv) = (2.4, 51.3);
        let (mu1, mv1) = (0.9, 7.3);
        let ruled = NurbsFaceBound {
            muu: 0.0,
            muv,
            mvv,
            mu1,
            mv1,
        };
        let s = ruled.split_steps(delta_s);
        assert!(s.cap, "the cap is what binds a ruled wall");
        // Bitwise: t = ρ·A on the ellipse, hv = √(δ_s/q(t)), hu = t·hv.
        let t = (mv1 / mu1) * ASPECT_CAP;
        let q = muv.mul_add(2.0 * t, mvv);
        let hv = (delta_s / q).sqrt();
        assert_eq!(s.hv.to_bits(), hv.to_bits(), "exact arm hv");
        assert_eq!(s.hu.to_bits(), (t * hv).to_bits(), "exact arm hu");
        // The generic arm at muu = subnormal dust lands on the same
        // point through the clamp (t* = √(mvv/dust) is far beyond the
        // window): agreement to 1e-6 relative is the stated bound.
        let dusty = NurbsFaceBound {
            muu: 3.8e-162,
            muv,
            mvv,
            mu1,
            mv1,
        };
        let d = dusty.split_steps(delta_s);
        assert!(d.cap);
        assert!(
            (d.hu - s.hu).abs() <= 1e-6 * s.hu && (d.hv - s.hv).abs() <= 1e-6 * s.hv,
            "generic limit ({:e},{:e}) vs exact arm ({:e},{:e})",
            d.hu,
            d.hv,
            s.hu,
            s.hv
        );
        // Mirror: mvv = 0 exact takes t = ρ/A.
        let mirror = NurbsFaceBound {
            muu: mvv,
            muv,
            mvv: 0.0,
            mu1: mv1,
            mv1: mu1,
        };
        let m = mirror.split_steps(delta_s);
        assert!(m.cap);
        let tm = (mu1 / mv1) / ASPECT_CAP;
        // q spelled exactly as the selection spells it (mul_add order),
        // so the pin is bitwise: mirror.muu = 51.3, mirror.mvv = 0.
        let qm = muv.mul_add(2.0 * tm, mvv.mul_add(tm.powi(2), 0.0));
        let hvm = (delta_s / qm).sqrt();
        assert_eq!(m.hv.to_bits(), hvm.to_bits(), "mirror arm hv");
        assert_eq!(m.hu.to_bits(), (tm * hvm).to_bits(), "mirror arm hu");
    }

    /// The ruled-wall recovery the unit exists for: against the
    /// retired AM-GM grouping the capped optimum spends several-fold
    /// fewer cells, and its 3-D aspect sits exactly on the cap rather
    /// than beyond it (the strip the unconstrained optimum would be).
    #[test]
    fn the_cap_binds_and_the_ruled_wall_gets_cheaper() {
        let delta_s = 1e-3;
        let b = NurbsFaceBound {
            muu: 0.0,
            muv: 2.4,
            mvv: 51.3,
            mu1: 1.1,
            mv1: 2.9,
        };
        let s = b.split_steps(delta_s);
        assert!(s.cap);
        let aspect = (s.hu * b.mu1) / (s.hv * b.mv1);
        assert!(
            (aspect - ASPECT_CAP).abs() < 1e-9 * ASPECT_CAP,
            "the chosen cell sits ON the cap: {aspect}"
        );
        // The retired grouping's point, spelled here as the
        // counterfactual it now is.
        let amgm = |group: f64| (delta_s / (2.0 * group)).sqrt();
        let (ou, ov) = (amgm(b.muu + b.muv), amgm(b.mvv + b.muv));
        let cells = |hu: f64, hv: f64| (1.0 / hu).ceil().max(1.0) * (1.0 / hv).ceil().max(1.0);
        let (new_cells, old_cells) = (cells(s.hu, s.hv), cells(ou, ov));
        assert!(
            new_cells * 3.0 <= old_cells,
            "expected a several-fold ruled-wall recovery: {new_cells} vs {old_cells}"
        );
    }

    /// The shipped cell-grid lookup ([`NurbsCellGrid::cert`]): a box
    /// inside one cell certifies at exactly that cell's own bound; a
    /// box crossing a knot line certifies at the componentwise sup
    /// over the cells it covers (the SHIPPED semantics — pinned
    /// exactly, against the tighter max-of-cells alternative, by
    /// `cert_is_pinned_to_the_componentwise_sup` above), and never
    /// more than the whole-patch certificate.
    #[test]
    fn cell_grid_cert_is_the_covered_cells_componentwise_sup() {
        for (name, s) in [("wavy", wavy()), ("wavy_rational", wavy_rational())] {
            let grid = nurbs_cell_grid(&s, FaceKey::default()).expect("covered");
            let cells = nurbs_cell_bounds(&s, FaceKey::default()).expect("covered");
            let whole = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
            assert!(
                grid.u_cuts().len() >= 2 && grid.v_cuts().len() >= 2,
                "{name}"
            );
            // A triangle strictly inside each cell: its certificate is
            // the cell's own.
            for c in &cells {
                let (u0, u1) = (
                    c.u.0 + 0.25 * (c.u.1 - c.u.0),
                    c.u.0 + 0.75 * (c.u.1 - c.u.0),
                );
                let (v0, v1) = (
                    c.v.0 + 0.25 * (c.v.1 - c.v.0),
                    c.v.0 + 0.75 * (c.v.1 - c.v.0),
                );
                let uv = [[u0, v0], [u1, v0], [u0, v1]];
                let got = grid.cert(uv);
                let own = c.bound.cert(uv);
                assert!(
                    (got - own).abs() <= f64::EPSILON * own.abs(),
                    "{name}: in-cell cert {got:e} is not the cell's own {own:e}"
                );
                assert!(
                    got <= whole.cert(uv),
                    "{name}: per-cell cert exceeds the whole-patch one"
                );
            }
            // A triangle spanning the whole chart: componentwise sup
            // over ALL cells — dominated by the whole-patch bound, and
            // dominating every cell's own certificate of the same box.
            let (ul, uh) = (
                *grid.u_cuts().first().unwrap(),
                *grid.u_cuts().last().unwrap(),
            );
            let (vl, vh) = (
                *grid.v_cuts().first().unwrap(),
                *grid.v_cuts().last().unwrap(),
            );
            let uv = [[ul, vl], [uh, vl], [ul, vh]];
            let got = grid.cert(uv);
            assert!(got <= whole.cert(uv), "{name}: spanning cert exceeds patch");
            for c in &cells {
                assert!(
                    got >= c.bound.cert(uv) - f64::EPSILON,
                    "{name}: spanning cert below a covered cell's — the componentwise \
                     sup is missing a cell"
                );
            }
        }
    }

    /// The exact unit quarter cylinder: rational quadratic arc
    /// (weights `[1, √2/2, 1]`) × unit line in z.
    fn quarter_cylinder() -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let arc = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut control = Vec::new();
        for (x, y) in arc {
            for z in [0.0, 1.0] {
                control.push(Point3::new(x, y, z));
            }
        }
        let weights = vec![1.0, 1.0, w, w, 1.0, 1.0];
        NurbsSurface::new(kv_u, kv_v, control, weights).unwrap()
    }

    /// The `wavy` net with steep alternating weights.
    fn wavy_rational() -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let (x, y) = (i as f64 * 0.7, j as f64 * 0.5);
                control.push(Point3::new(x, y, (1.3 * x + 0.9 * y).sin() + 0.3 * x * y));
                weights.push(match (i + 2 * j) % 4 {
                    0 => 0.4,
                    1 => 2.5,
                    2 => 1.0,
                    _ => 3.0,
                });
            }
        }
        NurbsSurface::new(kv_u, kv_v, control, weights).unwrap()
    }

    /// The rational wall of the REAL construction: the M8-5 pie loft
    /// (single-span bulge-0.4 arc profile, straight loft — the class
    /// M8-2's rational span meter made buildable), extracted from the
    /// assembled body.
    fn pie_wall() -> NurbsSurface<f64> {
        use geom_core::{Affine3, Point2, Vec3};
        let v = |x: f64, y: f64, bulge: f64| sweep::ProfileVertex::new(Point2::new(x, y), bulge);
        let lp =
            sweep::ProfileLoop::new(vec![v(1.0, 0.0, 0.4), v(0.0, 1.0, 0.0), v(0.0, 0.0, 0.0)]);
        let sections = vec![vec![lp.clone()], vec![lp]];
        let places: Vec<Affine3<f64>> = [0.0, 1.0]
            .iter()
            .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
            .collect();
        let body = sweep::loft_body::<f64>(&sections, &places, 1, Tol::witness())
            .expect("the rational pie lofts")
            .body;
        for (_, face) in body.faces() {
            if let Some(geom::Surface::Nurbs(p)) = body.get_surface(face.surface)
                && p.weights().iter().any(|w| *w != 1.0)
            {
                return (**p).clone();
            }
        }
        panic!("the pie loft minted no rational wall — the fixture stopped exercising M8-5");
    }

    /// The #218 per-triangle falsifier, pointed at the RATIONAL arm
    /// (M8-5): the z1 driver's 12-deep barycentric lattice (91
    /// samples/triangle), run on the real pie wall and the two
    /// adversarial rational patches, over the certificate's own
    /// `grid_steps` triangulation at two deltas. Every triangle's
    /// sampled `|S − Π|` must be dominated by `cert(uv)` (plus f64
    /// evaluation dust — the crate's documented ε-side slack, taken
    /// here as 1e-12 on O(1) fixtures), and the worst ratio is
    /// printed as the PR's evidence. Full-body z1 coverage of this
    /// lane awaits the topo pcurve frontier
    /// (`z1r_rational_wall_full_body_frontier_pin`).
    #[test]
    fn rational_z1_lattice_falsification() {
        // Per-fixture deltas: the steep-weight `wavy_rational`'s bound
        // is conservative (documented — a bound, not an estimate), so
        // its fine-δ grids run to millions of triangles; the lattice's
        // falsification power is PER TRIANGLE, not δ-dependent, so it
        // takes coarser deltas and the two real-construction fixtures
        // take the z1 driver's.
        for (name, s, deltas) in [
            ("pie_wall", pie_wall(), [3e-2, 6e-3]),
            ("quarter_cylinder", quarter_cylinder(), [3e-2, 6e-3]),
            ("wavy_rational", wavy_rational(), [1.0, 2e-1]),
        ] {
            let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
            let (u0, u1) = s.knots_u().domain();
            let (v0, v1) = s.knots_v().domain();
            for delta in deltas {
                let delta_s = delta / 2.0;
                let (hu, hv) = b.grid_steps(delta_s);
                let cells = |span: f64, h: f64| -> usize {
                    let raw = (span / h).ceil();
                    if raw.is_finite() && raw >= 1.0 {
                        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                        {
                            raw as usize
                        }
                    } else {
                        1
                    }
                };
                let (nu, nv) = (cells(u1 - u0, hu), cells(v1 - v0, hv));
                let at = |i: usize, j: usize| -> [f64; 2] {
                    #[allow(clippy::cast_precision_loss)]
                    [
                        u0 + (u1 - u0) * (i as f64 / nu as f64),
                        v0 + (v1 - v0) * (j as f64 / nv as f64),
                    ]
                };
                let (mut worst_ratio, mut tris) = (0.0f64, 0usize);
                for i in 0..nu {
                    for j in 0..nv {
                        let (a, bb, c, d) =
                            (at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1));
                        for uv in [[a, bb, c], [a, c, d]] {
                            tris += 1;
                            let cert = b.cert(uv);
                            let p: Vec<geom_core::Point3<f64>> =
                                uv.iter().map(|w| s.eval(w[0], w[1])).collect();
                            let m = 12usize;
                            for ba in 0..=m {
                                for bbn in 0..=(m - ba) {
                                    #[allow(clippy::cast_precision_loss)]
                                    let (b0, b1) = (ba as f64 / m as f64, bbn as f64 / m as f64);
                                    let b2 = 1.0 - b0 - b1;
                                    let (u, v) = (
                                        b0 * uv[0][0] + b1 * uv[1][0] + b2 * uv[2][0],
                                        b0 * uv[0][1] + b1 * uv[1][1] + b2 * uv[2][1],
                                    );
                                    let sv = s.eval(u, v);
                                    let pi = Point3::new(
                                        b0 * p[0].x + b1 * p[1].x + b2 * p[2].x,
                                        b0 * p[0].y + b1 * p[1].y + b2 * p[2].y,
                                        b0 * p[0].z + b1 * p[1].z + b2 * p[2].z,
                                    );
                                    let dev = ((sv.x - pi.x).powi(2)
                                        + (sv.y - pi.y).powi(2)
                                        + (sv.z - pi.z).powi(2))
                                    .sqrt();
                                    assert!(
                                        dev <= cert + 1e-12,
                                        "{name}: per-triangle violation |S-Pi| {dev} > cert \
                                         {cert} at uv=({u},{v}) tri {uv:?}"
                                    );
                                    if cert > 0.0 {
                                        worst_ratio = worst_ratio.max(dev / cert);
                                    }
                                }
                            }
                        }
                    }
                }
                println!(
                    "{name} delta={delta:.0e}: grid {nu}x{nv} tris={tris} max d/cert={worst_ratio:.4}"
                );
                // Monotone the easy way — `worst_ratio` only shrinks
                // as the certificate grows, so a LOOSE bound passes
                // this by a wider margin than a tight one. One of the
                // class's three open instances; S237.
                assert!(
                    worst_ratio <= 1.0,
                    "{name}: a triangle's samples exceeded its certificate"
                );
            }
        }
    }

    /// The POISON row the flip keeps: an ILLEGAL rational (non-positive
    /// or non-finite weight) cannot even be described —
    /// `NurbsSurface::new` refuses at the door, which is why
    /// `rational_face_bound`'s own licence check is a defensive
    /// backstop rather than a reachable lane.
    #[test]
    fn illegal_rational_weight_refuses_at_the_door() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![Point3::new(0.0, 0.0, 0.0); 4];
        for bad in [0.0, -1.0, f64::NAN] {
            assert!(
                NurbsSurface::new(
                    kv.clone(),
                    kv.clone(),
                    control.clone(),
                    vec![1.0, bad, 1.0, 1.0]
                )
                .is_err(),
                "weight {bad} must refuse at construction"
            );
        }
    }

    /// The rational dust re-derivation (the
    /// `planar_bilinear_bounds_collapse` class, rational arm): a flat
    /// bilinear patch with UNIFORM weight `1/2` is bitwise rational but
    /// geometrically the same flat quad (constant weights cancel in
    /// ℝ), so every true second partial is zero. What the rational
    /// assembly answers is DUST scaled by the divisor `w_min = 1/2` —
    /// re-derived, not blind-reused from the integral thresholds:
    ///
    /// - `muu`/`mvv`: the degree-1 `Ã_dd`/`w_dd` terms are exact
    ///   zeros and the `w_d` hulls of a CONSTANT weight column stay
    ///   outward-rounded zeros through refinement (convex combinations
    ///   of `0.5` are exact), so the cross terms are subnormal dust
    ///   over `w_min` — the integral 1e-100 class survives (measured
    ///   at the deep-subnormal ~1e-16x scale, run-dependent in the
    ///   last decades: 5e-166 here, 5e-162 by the R1 review; the
    ///   1e-100 pin is the claim).
    /// - `muv`: the refined homogeneous net carries knot-INSERTION
    ///   rounding (~ulp of O(1) coefficients), and the mixed
    ///   differencing scales it by `p/Δu` at the 16-fold-refined span
    ///   width — measured ~2e-13, so the integral arm's 1e-12 pin
    ///   would be blind reuse; pinned an order above at 1e-11.
    #[test]
    fn rational_uniform_weight_bilinear_dust() {
        let kv = KnotVector::unit_segment(1);
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![0.5; 4]).unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(
            b.muu < 1e-100 && b.muv < 1e-11 && b.mvv < 1e-100,
            "rational dust escaped its derivation: ({}, {}, {})",
            b.muu,
            b.muv,
            b.mvv
        );
        let (hu, hv) = b.grid_steps(1e-3);
        assert!(hu > 1e3 && hv > 1e3, "flat stays effectively unconstrained");
    }

    /// A C⁰ crease (interior multiplicity = degree) refuses typed —
    /// the Taylor remainder needs C¹.
    #[test]
    fn c0_crease_refuses_typed() {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        let n = kv_u.control_count() * kv_v.control_count();
        let control = vec![Point3::new(0.0, 0.0, 0.0); n];
        let s = NurbsSurface::new(kv_u, kv_v, control, vec![1.0; n]).unwrap();
        match nurbs_face_bound(&s, FaceKey::default()) {
            Err(TessellateError::UnsupportedNurbsFace { note, .. }) => {
                assert!(note.contains("crease"));
            }
            other => panic!("expected UnsupportedNurbsFace, got {other:?}"),
        }
    }

    /// A degree-1 direction with interior knots is a crease too.
    #[test]
    fn degree_one_interior_knot_refuses_typed() {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
        let kv_v = KnotVector::unit_segment(1);
        let n = kv_u.control_count() * kv_v.control_count();
        let control = vec![Point3::new(0.0, 0.0, 0.0); n];
        let s = NurbsSurface::new(kv_u, kv_v, control, vec![1.0; n]).unwrap();
        match nurbs_face_bound(&s, FaceKey::default()) {
            Err(TessellateError::UnsupportedNurbsFace { note, .. }) => {
                assert!(note.contains("crease"));
            }
            other => panic!("expected UnsupportedNurbsFace, got {other:?}"),
        }
    }

    // ------------------------------------------------------------------
    // R1 REVIEW PROBES (M8-5, PR #322): independent adversarial
    // fixtures beyond the PR's own — extreme weight ratios, the C¹
    // multiplicity edge on the RATIONAL arm, tiny/huge/offset patches
    // (the recentring claim), and a Möbius-reparameterized ruling
    // (degree-1 cross-term path). Each dense-samples the true second
    // partials via the independently dual-validated `ders` oracle and
    // demands domination, with a finite-difference spot cross-check.
    // ------------------------------------------------------------------

    fn sample_worst(s: &NurbsSurface<f64>, n: u32) -> (f64, f64, f64) {
        let (u0, u1) = s.knots_u().domain();
        let (v0, v1) = s.knots_v().domain();
        let (mut wuu, mut wuv, mut wvv) = (0.0f64, 0.0f64, 0.0f64);
        for i in 0..=n {
            for j in 0..=n {
                let u = u0 + (u1 - u0) * f64::from(i) / f64::from(n);
                let v = v0 + (v1 - v0) * f64::from(j) / f64::from(n);
                let jet = s.ders(u, v);
                wuu = wuu.max(jet.duu.norm());
                wuv = wuv.max(jet.duv.norm());
                wvv = wvv.max(jet.dvv.norm());
            }
        }
        (wuu, wuv, wvv)
    }

    fn assert_dominates(
        name: &str,
        s: &NurbsSurface<f64>,
        n: u32,
    ) -> (f64, f64, f64, NurbsFaceBound) {
        let b = nurbs_face_bound(s, FaceKey::default()).expect("covered");
        let (wuu, wuv, wvv) = sample_worst(s, n);
        assert!(
            wuu <= b.muu && wuv <= b.muv && wvv <= b.mvv,
            "{name}: sampled ({wuu:.6e},{wuv:.6e},{wvv:.6e}) escapes certified \
             ({:.6e},{:.6e},{:.6e})",
            b.muu,
            b.muv,
            b.mvv
        );
        println!(
            "{name}: truth/bound uu={:.4} uv={:.4} vv={:.4}",
            wuu / b.muu,
            wuv / b.muv,
            wvv / b.mvv
        );
        (wuu, wuv, wvv, b)
    }

    /// Mixed extreme weights (1e-3 .. 1e3, interior) on a wavy
    /// quadratic×cubic multi-span net.
    fn extreme_weight_patch(offset: [f64; 3], scale: f64) -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v =
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        let wtab = [1e-3, 1.0, 1e3, 0.04, 25.0];
        for i in 0..nu {
            for j in 0..nv {
                let (x, y) = (i as f64 * 0.5, j as f64 * 0.45);
                control.push(Point3::new(
                    offset[0] + scale * x,
                    offset[1] + scale * y,
                    offset[2] + scale * ((1.7 * x - 1.1 * y).sin() + 0.25 * x * y),
                ));
                weights.push(wtab[(3 * i + j) % 5]);
            }
        }
        NurbsSurface::new(kv_u, kv_v, control, weights).unwrap()
    }

    #[test]
    fn r1_extreme_weights_dominated() {
        let s = extreme_weight_patch([0.0; 3], 1.0);
        assert_dominates("extreme_weights", &s, 400);
    }

    /// Interior multiplicity EXACTLY p−1 on the RATIONAL arm — the C¹
    /// gate's covered edge composed with the quotient rule; S_uu jumps
    /// at the knot and the sampling straddles it.
    #[test]
    fn r1_rational_multiplicity_p_minus_one_dominated() {
        for (p_deg, mult) in [(2usize, 1usize), (3, 2)] {
            let mut knots = vec![0.0; p_deg + 1];
            knots.extend(std::iter::repeat_n(0.5, mult));
            knots.extend(vec![1.0; p_deg + 1]);
            let kv_u = KnotVector::clamped(knots, p_deg).unwrap();
            let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
            let mut control = Vec::new();
            let mut weights = Vec::new();
            for i in 0..nu {
                for j in 0..nv {
                    let (x, y) = (i as f64 * 0.6, j as f64 * 0.8);
                    control.push(Point3::new(x, y, (2.1 * x - 1.3 * y).cos() + 0.4 * x * y));
                    weights.push(match (i + j) % 3 {
                        0 => 0.3,
                        1 => 2.0,
                        _ => 0.9,
                    });
                }
            }
            let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
            assert_dominates(&format!("rational_mult_p1_deg{p_deg}"), &s, 320);
        }
    }

    /// Tiny (1e-6-scale) and huge far-from-origin (1e6 offset) copies
    /// of the extreme-weight patch: domination must hold at both, and
    /// the OFFSET copy's bound must stay commensurate with the centred
    /// one (the recentring claim — no distance-to-origin inflation).
    #[test]
    fn r1_tiny_and_offset_patches_dominated() {
        let tiny = extreme_weight_patch([0.0; 3], 1e-6);
        assert_dominates("tiny_1e-6", &tiny, 250);
        let centred = extreme_weight_patch([0.0; 3], 1.0);
        let bc = nurbs_face_bound(&centred, FaceKey::default()).unwrap();
        let far = extreme_weight_patch([1e6, -3e6, 2e6], 1.0);
        let (wuu, wuv, wvv, bf) = assert_dominates("offset_1e6", &far, 250);
        let _ = (wuu, wuv, wvv);
        assert!(
            bf.muu < 16.0 * bc.muu && bf.muv < 16.0 * bc.muv && bf.mvv < 16.0 * bc.mvv,
            "recentring failed: offset bound ({},{},{}) vs centred ({},{},{})",
            bf.muu,
            bf.muv,
            bf.mvv,
            bc.muu,
            bc.muv,
            bc.mvv
        );
    }

    /// A Möbius-reparameterized ruling: degree-1 u with UNEQUAL weights
    /// along u — S is ruled but S_uu ≠ 0 in parameter. The d20-None
    /// path must still bound it through the surviving cross terms.
    #[test]
    fn r1_moebius_ruling_degree1_cross_terms_dominated() {
        let kv_u = KnotVector::unit_segment(1);
        let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let (x, y) = (i as f64 * 2.0, j as f64 * 0.7);
                control.push(Point3::new(x, y, 0.5 * y.powi(2) + 0.3 * x * y));
                weights.push(if i == 0 { 1.0 } else { 5.0 });
            }
        }
        let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
        let (wuu, _, _, b) = assert_dominates("moebius_ruling", &s, 400);
        assert!(
            wuu > 1e-3,
            "the ruling must genuinely curve in u (fixture check)"
        );
        assert!(b.muu > 0.0, "muu must be real, not the integral arm's zero");
    }

    /// Near-zero-touching (but legal) weight 1e-6 amid O(1): the true
    /// derivatives spike; the bound must still dominate.
    #[test]
    fn r1_near_zero_weight_dominated() {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                let (x, y) = (i as f64, j as f64);
                control.push(Point3::new(x, y, 0.6 * (x + y).sin()));
                weights.push(if (i, j) == (1, 1) { 1e-6 } else { 1.0 });
            }
        }
        let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
        assert_dominates("near_zero_weight", &s, 600);
    }

    /// FD spot cross-check of the `ders` oracle itself on the
    /// extreme-weight patch (independent of the dual validation).
    #[test]
    fn r1_ders_oracle_fd_crosscheck() {
        let s = extreme_weight_patch([0.0; 3], 1.0);
        let h = 1e-5;
        for (u, v) in [(0.3, 0.4), (0.61, 0.27), (0.45, 0.55)] {
            let jet = s.ders(u, v);
            let fd_uu = {
                let a = s.eval(u - h, v);
                let b = s.eval(u, v);
                let c = s.eval(u + h, v);
                Point3::new(
                    (a.x - 2.0 * b.x + c.x) / h.powi(2),
                    (a.y - 2.0 * b.y + c.y) / h.powi(2),
                    (a.z - 2.0 * b.z + c.z) / h.powi(2),
                )
            };
            let n_fd = (fd_uu.x.powi(2) + fd_uu.y.powi(2) + fd_uu.z.powi(2)).sqrt();
            let rel = (n_fd - jet.duu.norm()).abs() / jet.duu.norm().max(1.0);
            assert!(rel < 1e-4, "ders/FD disagree at ({u},{v}): {rel:.3e}");
        }
    }

    /// R1 claim-2 probe: my own adversarial rational fixture through
    /// the SAME z1 lattice (12-deep barycentric, per-triangle d/cert ≤
    /// 1) over the certificate's own grid.
    #[test]
    fn r1_extreme_weight_z1_lattice() {
        let s = extreme_weight_patch([0.0; 3], 1.0);
        let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
        let (u0, u1) = s.knots_u().domain();
        let (v0, v1) = s.knots_v().domain();
        for delta in [1.0, 2e-1] {
            let delta_s = delta / 2.0;
            let (hu, hv) = b.grid_steps(delta_s);
            // Grid capped at 96 cells/direction: the extreme-weight
            // bound is very conservative, so the budget grid would run
            // to millions of cells; the per-triangle claim d ≤ cert(uv)
            // is grid-independent, so a coarser grid still falsifies.
            let cells = |span: f64, h: f64| -> usize {
                let raw = (span / h).ceil();
                if raw.is_finite() && raw >= 1.0 {
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    {
                        (raw as usize).min(96)
                    }
                } else {
                    1
                }
            };
            let (nu, nv) = (cells(u1 - u0, hu), cells(v1 - v0, hv));
            let at = |i: usize, j: usize| -> [f64; 2] {
                #[allow(clippy::cast_precision_loss)]
                [
                    u0 + (u1 - u0) * (i as f64 / nu as f64),
                    v0 + (v1 - v0) * (j as f64 / nv as f64),
                ]
            };
            let (mut worst_ratio, mut tris) = (0.0f64, 0usize);
            for i in 0..nu {
                for j in 0..nv {
                    let (a, bb, c, d) = (at(i, j), at(i + 1, j), at(i + 1, j + 1), at(i, j + 1));
                    for uv in [[a, bb, c], [a, c, d]] {
                        tris += 1;
                        let cert = b.cert(uv);
                        let p: Vec<geom_core::Point3<f64>> =
                            uv.iter().map(|w| s.eval(w[0], w[1])).collect();
                        let m = 12usize;
                        for ba in 0..=m {
                            for bbn in 0..=(m - ba) {
                                #[allow(clippy::cast_precision_loss)]
                                let (b0, b1) = (ba as f64 / m as f64, bbn as f64 / m as f64);
                                let b2 = 1.0 - b0 - b1;
                                let (u, v) = (
                                    b0 * uv[0][0] + b1 * uv[1][0] + b2 * uv[2][0],
                                    b0 * uv[0][1] + b1 * uv[1][1] + b2 * uv[2][1],
                                );
                                let sv = s.eval(u, v);
                                let pi = Point3::new(
                                    b0 * p[0].x + b1 * p[1].x + b2 * p[2].x,
                                    b0 * p[0].y + b1 * p[1].y + b2 * p[2].y,
                                    b0 * p[0].z + b1 * p[1].z + b2 * p[2].z,
                                );
                                let dev = ((sv.x - pi.x).powi(2)
                                    + (sv.y - pi.y).powi(2)
                                    + (sv.z - pi.z).powi(2))
                                .sqrt();
                                assert!(dev <= cert + 1e-12, "violation d={dev} cert={cert}");
                                if cert > 0.0 {
                                    worst_ratio = worst_ratio.max(dev / cert);
                                }
                            }
                        }
                    }
                }
            }
            println!("r1_extreme delta={delta:.0e}: tris={tris} max d/cert={worst_ratio:.4}");
            // As above: one-sided, and a loose bound passes it more
            // easily than a tight one (S237).
            assert!(
                worst_ratio <= 1.0,
                "r1_extreme: a sample exceeded its own certificate ({worst_ratio})"
            );
        }
    }

    /// R1 randomized soundness sweep: random rational patches (degrees
    /// 1-3, 1-3 spans, log-uniform weights 1e-2..1e2), dense-sampled true
    /// second partials vs the certified sups. This sweep is the row that
    /// KILLS the one mutation the rest of the suite missed (dropping the
    /// `v0*w11` term from `suv` — the recentred-value x
    /// mixed-weight-derivative cross term).
    ///
    /// The trial count rides `CAD_FUZZ_EFFORT` and the seed varies per
    /// run. The 61x61 `sample_worst` grid does NOT: it is the domination
    /// check itself, not a sweep dimension.
    #[test]
    fn r1_random_rational_soundness_sweep() {
        let mut rng = fuzz::start("nurbs_cert::r1_random_rational_soundness");
        fn mk(r: &mut fuzz::Rng, p: usize) -> KnotVector {
            let spans = 1 + r.below(2);
            let mut k = vec![0.0; p + 1];
            for i in 1..spans {
                #[allow(clippy::cast_precision_loss)]
                k.push(i as f64 / spans as f64);
            }
            k.extend(vec![1.0; p + 1]);
            KnotVector::clamped(k, p).unwrap()
        }
        let mut worst = 0.0f64;
        // TRIALS are breadth; the 61x61 `sample_worst` grid below is the
        // per-trial falsification power and is deliberately NOT reduced —
        // it IS the domination check. With a varying seed, breadth is
        // what successive runs supply for free, so the trial count is the
        // honest lever here and the grid is not.
        for trial in 0..fuzz::scaled(60) {
            let pu = 1 + rng.below(3);
            let pv = 1 + rng.below(3);
            let kv_u = mk(&mut rng, pu);
            let kv_v = mk(&mut rng, pv);
            let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
            let mut control = Vec::new();
            let mut weights = Vec::new();
            for _ in 0..nu * nv {
                control.push(Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                ));
                weights.push(10f64.powf(rng.range(-2.0, 2.0)));
            }
            let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
            let Ok(b) = nurbs_face_bound(&s, FaceKey::default()) else {
                continue;
            };
            let (wuu, wuv, wvv) = sample_worst(&s, 60);
            let r = (wuu / b.muu).max(wuv / b.muv).max(wvv / b.mvv);
            worst = worst.max(r);
            assert!(
                wuv <= b.muv && wuu <= b.muu && wvv <= b.mvv,
                "UNSOUND at trial {trial}: ({wuu:.3e},{wuv:.3e},{wvv:.3e}) vs \
                 ({:.3e},{:.3e},{:.3e}) — {}",
                b.muu,
                b.muv,
                b.mvv,
                fuzz::replay()
            );
        }
        println!("random sweep: worst truth/bound {worst:.6}");
        // COVERAGE FLOOR: the sweep must keep producing cases where the
        // bound is genuinely tight, otherwise a slack bound would pass by
        // never being challenged. Verified to hold at the shipped trial
        // count; if a run ever trips it, RAISE the count rather than
        // lowering the threshold.
        assert!(
            worst > 0.5,
            "the sweep must stay adversarial (tight cases exist): worst \
             {worst:.6} — {}",
            fuzz::replay()
        );
    }

    // ---------------------------------------------------------------
    // CERT-10 (issue 1006): the fold-cost measurement, taken BEFORE
    // the whole-face arm's shape was chosen.
    // ---------------------------------------------------------------

    /// A high-knot-count integral face: degree 3 x 3 with `k` interior
    /// knots per direction, so the whole-net hull covers a net the
    /// per-cell fold reads in `(k+1)^2` windows of `4 x 4`.
    fn many_knot_face(k: usize) -> NurbsSurface<f64> {
        #[allow(clippy::cast_precision_loss)]
        let interior: Vec<f64> = (1..=k).map(|i| i as f64 / (k + 1) as f64).collect();
        let mk = || {
            let mut ks = vec![0.0; 4];
            ks.extend(interior.iter().copied());
            ks.extend([1.0; 4]);
            KnotVector::clamped(ks, 3).unwrap()
        };
        let (kv_u, kv_v) = (mk(), mk());
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64 * 0.31, j as f64 * 0.27);
                control.push(Point3::new(x, y, (1.7 * x).sin() * (1.1 * y).cos()));
            }
        }
        let w = vec![1.0; control.len()];
        NurbsSurface::new(kv_u, kv_v, control, w).unwrap()
    }

    /// **The fixture the tighter-or-equal claim has teeth on.** A
    /// per-channel `mag` hull is the max of `|coefficient|` over the
    /// window, and the cell windows COVER the net, so for any single
    /// channel `max over cells == whole net` exactly. The whole-net
    /// spelling is strictly wider only where the `sum over channels of
    /// sup squared` mixes channels whose extremes live in DIFFERENT
    /// cells: the whole-net number adds three maxima that no single
    /// point of the patch attains together. This net staggers them —
    /// `x` bends hard near `u = 0`, `z` near `u = 1`.
    fn staggered_channels() -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        // Per u index: a big x-kink at the low end, a big z-kink at the
        // high end, and a y that ramps uniformly.
        let xs = [0.0, 3.0, 0.0, 0.05];
        let zs = [0.0, 0.02, 0.0, 4.0];
        let mut control = Vec::new();
        for i in 0..nu {
            for j in 0..nv {
                #[allow(clippy::cast_precision_loss)]
                let y = j as f64 * 0.5;
                control.push(Point3::new(xs[i % 4], y, zs[i % 4]));
            }
        }
        let w = vec![1.0; control.len()];
        NurbsSurface::new(kv_u, kv_v, control, w).unwrap()
    }

    /// **The whole-net hull, as the fold's counterfactual.** This is
    /// not a third spelling of the differencing: it assembles the very
    /// same derivative nets through the shared home
    /// ([`geom_core::spline::net::TensorNet`]) and reads them with ONE
    /// window instead of one per cell — which is exactly what the
    /// retired whole-face arm did. Kept in the test module because the
    /// only remaining consumer is the measurement that chose the
    /// fold's shape.
    ///
    /// INTEGRAL faces only: the spelling reads the control net with no
    /// weights at all, so on a rational description it answers a
    /// different surface's bound. The rational arm never had a
    /// whole-net counterpart — it has always been a fold.
    fn whole_net_bound(s: &NurbsSurface<f64>) -> Option<NurbsFaceBound> {
        use geom_core::spline::net::TensorNet;
        assert!(
            !patch_bound::is_rational(s),
            "the whole-net spelling is the integral arm's; a rational face has no such arm"
        );
        let (kv_u, kv_v) = (s.knots_u(), s.knots_v());
        patch_bound::check_direction(kv_u).ok()?;
        patch_bound::check_direction(kv_v).ok()?;
        let kv_u1 = (kv_u.degree() >= 2)
            .then(|| patch_bound::derived_knots(kv_u))
            .transpose()
            .ok()?;
        let kv_v1 = (kv_v.degree() >= 2)
            .then(|| patch_bound::derived_knots(kv_v))
            .transpose()
            .ok()?;
        let (nu, nv) = s.control_counts();
        let zero = RingInterval::zero();
        let mut sq = [zero; 5];
        for c in 0..3 {
            let base = TensorNet::from_fn(nu, nv, |i, j| {
                let p = s.control()[i * nv + j];
                RingInterval::point(match c {
                    0 => p.x,
                    1 => p.y,
                    _ => p.z,
                })
            });
            let d10 = base.diff_u_knots(kv_u);
            let d01 = base.diff_v_knots(kv_v);
            let d11 = d10.diff_v_knots(kv_v);
            let g20 = kv_u1.as_ref().map_or(zero, |k| d10.diff_u_knots(k).hull());
            let g02 = kv_v1.as_ref().map_or(zero, |k| d01.diff_v_knots(k).hull());
            for (slot, h) in sq
                .iter_mut()
                .zip([g20, d11.hull(), g02, d10.hull(), d01.hull()])
            {
                *slot = *slot + h.sqr();
            }
        }
        Some(NurbsFaceBound {
            muu: cell_component(sq[0]),
            muv: cell_component(sq[1]),
            mvv: cell_component(sq[2]),
            mu1: cell_component(sq[3]),
            mv1: cell_component(sq[4]),
        })
    }

    /// The per-cell fold: the componentwise max over `patch_cells`'
    /// per-cell bounds.
    fn fold_bound(s: &NurbsSurface<f64>) -> Option<NurbsFaceBound> {
        let cells = nurbs_cell_bounds(s, FaceKey::default()).ok()?;
        let mut m = NurbsFaceBound {
            muu: 0.0,
            muv: 0.0,
            mvv: 0.0,
            mu1: 0.0,
            mv1: 0.0,
        };
        for c in &cells {
            m.muu = m.muu.max(c.bound.muu);
            m.muv = m.muv.max(c.bound.muv);
            m.mvv = m.mvv.max(c.bound.mvv);
            m.mu1 = m.mu1.max(c.bound.mu1);
            m.mv1 = m.mv1.max(c.bound.mv1);
        }
        Some(m)
    }

    /// Mean microseconds per call of `f`, over `reps` repetitions.
    fn micros<T>(reps: u32, mut f: impl FnMut() -> T) -> f64 {
        let t = std::time::Instant::now();
        for _ in 0..reps {
            core::hint::black_box(f());
        }
        t.elapsed().as_secs_f64() * 1e6 / f64::from(reps)
    }

    /// **The fold-cost table** (issue 1006's ruling: "fold cost
    /// measured against the whole-net hull BEFORE the shape is
    /// chosen"). Reports, per INTEGRAL fixture, the wall time and the
    /// bound width of both spellings, plus the rational fixtures'
    /// per-cell cost for scale (the rational arm has always been a
    /// fold, so it has no whole-net counterpart to compare against).
    ///
    /// `#[ignore]`d: it is a MEASUREMENT, not an assertion — timings
    /// are a reading on one box and gate nothing. Run it with
    /// `cargo test -p mesh --lib -- --ignored --nocapture cert10_fold_cost`.
    #[test]
    #[ignore = "measurement harness: prints the fold-cost table, asserts nothing about timings"]
    fn cert10_fold_cost_table() {
        let integral: Vec<(&str, NurbsSurface<f64>)> = vec![
            ("wavy (2x3, 1+1 interior)", wavy()),
            ("staggered_channels (2x2)", staggered_channels()),
            ("many_knot k=8 (3x3)", many_knot_face(8)),
            ("many_knot k=24 (3x3)", many_knot_face(24)),
        ];
        let reps = 40;
        println!(
            "\nINTEGRAL: whole-net hull vs per-cell fold\n{:<30} {:>6} {:>9} {:>9} {:>6} \
             {:>13} {:>13} {:>9}",
            "fixture",
            "cells",
            "whole us",
            "fold us",
            "cost x",
            "whole muu",
            "fold muu",
            "fold/whole"
        );
        for (name, s) in &integral {
            let cells = patch_bound::patch_cells(s).map_or(0, |c| c.len());
            let tw = micros(reps, || whole_net_bound(s));
            let tf = micros(reps, || fold_bound(s));
            let (w, f) = (
                whole_net_bound(s).expect("covered"),
                fold_bound(s).expect("covered"),
            );
            println!(
                "{name:<30} {cells:>6} {tw:>9.1} {tf:>9.1} {:>6.2} {:>13.6e} {:>13.6e} {:>9.4}",
                tf / tw,
                w.muu,
                f.muu,
                f.muu / w.muu
            );
            println!(
                "{:<30} {:>6} {:>9} {:>9} {:>6} muv {:>9.6e} {:>13.6e} {:>9.4}",
                "",
                "",
                "",
                "",
                "",
                w.muv,
                f.muv,
                f.muv / w.muv
            );
            println!(
                "{:<30} {:>6} {:>9} {:>9} {:>6} mu1 {:>9.6e} {:>13.6e} {:>9.4}",
                "",
                "",
                "",
                "",
                "",
                w.mu1,
                f.mu1,
                f.mu1 / w.mu1
            );
        }
        let rational: Vec<(&str, NurbsSurface<f64>)> = vec![
            ("quarter_cylinder (2x1)", quarter_cylinder()),
            ("wavy_rational (2x3)", wavy_rational()),
            ("pie_wall (real loft)", pie_wall()),
        ];
        println!(
            "\nRATIONAL (already a fold; no whole-net arm exists)\n{:<30} {:>6} {:>9} {:>13}",
            "fixture", "cells", "fold us", "fold muu"
        );
        for (name, s) in &rational {
            let cells = patch_bound::patch_cells(s).map_or(0, |c| c.len());
            let tf = micros(reps, || fold_bound(s));
            let f = fold_bound(s).expect("covered");
            println!("{name:<30} {cells:>6} {tf:>9.1} {:>13.6e}", f.muu);
        }
    }

    /// R2 probe: full-precision digits of the whole-net counterfactual,
    /// for comparison against the pre-collapse arm's own output.
    #[test]
    fn r2_probe_whole_net_digits() {
        for (name, s) in [
            ("wavy", wavy()),
            ("staggered", staggered_channels()),
            ("many8", many_knot_face(8)),
        ] {
            let b = whole_net_bound(&s).unwrap();
            println!(
                "R2WHOLE {name} muu={:.17e} muv={:.17e} mvv={:.17e} mu1={:.17e} mv1={:.17e}",
                b.muu, b.muv, b.mvv, b.mu1, b.mv1
            );
        }
    }

    /// **The tighter-or-equal claim, randomized.** The two pinned
    /// fixtures show the gap exists and that a smooth net closes it;
    /// this row asserts the INEQUALITY over a sweep of random integral
    /// nets, which is the form the claim is actually made in. Integral
    /// only, because the rational arm has always been a fold and has
    /// no whole-net counterpart to be tighter than.
    #[test]
    fn cert10_the_fold_never_exceeds_the_whole_net_hull() {
        let mut rng = fuzz::start("nurbs_cert::cert10_fold_vs_whole_net");
        fn mk(r: &mut fuzz::Rng, p: usize) -> KnotVector {
            let spans = 1 + r.below(4);
            let mut k = vec![0.0; p + 1];
            for i in 1..spans {
                #[allow(clippy::cast_precision_loss)]
                k.push(i as f64 / spans as f64);
            }
            k.extend(vec![1.0; p + 1]);
            KnotVector::clamped(k, p).unwrap()
        }
        let mut strict = 0usize;
        let trials = fuzz::scaled(60);
        for _ in 0..trials {
            let (pu, pv) = (1 + rng.below(3), 1 + rng.below(3));
            let kv_u = mk(&mut rng, pu);
            let kv_v = mk(&mut rng, pv);
            let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
            let control: Vec<Point3<f64>> = (0..nu * nv)
                .map(|_| {
                    Point3::new(
                        rng.range(-4.0, 4.0),
                        rng.range(-4.0, 4.0),
                        rng.range(-4.0, 4.0),
                    )
                })
                .collect();
            let w = vec![1.0; control.len()];
            let Ok(s) = NurbsSurface::new(kv_u, kv_v, control, w) else {
                continue;
            };
            let (Some(whole), Some(fold)) = (whole_net_bound(&s), fold_bound(&s)) else {
                continue;
            };
            for (what, f, w) in [
                ("muu", fold.muu, whole.muu),
                ("muv", fold.muv, whole.muv),
                ("mvv", fold.mvv, whole.mvv),
                ("mu1", fold.mu1, whole.mu1),
                ("mv1", fold.mv1, whole.mv1),
            ] {
                assert!(
                    f <= w,
                    "the fold EXCEEDED the whole-net hull on {what}: {f:.17e} > \
                     {w:.17e} — {}",
                    fuzz::replay()
                );
                if f < w {
                    strict += 1;
                }
            }
        }
        // COVERAGE FLOOR: an inequality nothing ever makes strict is a
        // tautology. If a run trips this, RAISE the trial count.
        assert!(
            strict > trials,
            "the sweep must keep producing STRICT gaps: {strict} strict of {} \
             comparisons — {}",
            trials * 5,
            fuzz::replay()
        );
        println!(
            "cert10 fold-vs-whole-net: {strict} strict of {} comparisons",
            trials * 5
        );
    }

    /// **CERT-10 red row (issue 1006, the Q2 ruling): the whole-face
    /// bound IS the per-cell fold.** Per-cell-then-union is tighter or
    /// equal — every cell's window is a SUBSET of the whole net's, so
    /// no cell can report more, and the fold can report less wherever
    /// two channels' extremes live in different cells. This row pins
    /// the shipped face bound to that fold, componentwise, on every
    /// covered fixture class.
    #[test]
    fn cert10_whole_face_bound_is_the_per_cell_fold() {
        for (name, s) in [
            ("wavy", wavy()),
            ("staggered_channels", staggered_channels()),
            ("many_knot k=8", many_knot_face(8)),
            ("quarter_cylinder", quarter_cylinder()),
            ("wavy_rational", wavy_rational()),
            ("pie_wall", pie_wall()),
        ] {
            let shipped = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
            let fold = fold_bound(&s).expect("covered");
            for (what, a, b) in [
                ("muu", shipped.muu, fold.muu),
                ("muv", shipped.muv, fold.muv),
                ("mvv", shipped.mvv, fold.mvv),
                ("mu1", shipped.mu1, fold.mu1),
                ("mv1", shipped.mv1, fold.mv1),
            ] {
                assert!(
                    a == b,
                    "{name}: shipped {what} {a:.17e} is not the per-cell fold {b:.17e}"
                );
            }
        }
    }

    /// **CERT-10 red row: the tightening is real, not merely
    /// non-negative.** The whole-net figures below were MEASURED on
    /// this tree before the collapse (`cert10_fold_cost_table`), and
    /// are pinned as the record of what the whole-net spelling
    /// answered. The mechanism, so the gap is not read as noise: a
    /// per-channel hull's `mag` is `max |coefficient|` over the
    /// window, and the cell windows COVER the net, so for one channel
    /// the fold and the whole net agree exactly. The gap lives in the
    /// `sum over channels of sup squared` — the whole-net number adds
    /// three per-channel maxima no single point of the patch attains
    /// together, and `staggered_channels` puts the `x` and `z` maxima
    /// in different cells on purpose.
    #[test]
    fn cert10_fold_is_strictly_tighter_than_the_whole_net_hull() {
        let s = staggered_channels();
        let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
        // Measured whole-net figures (pre-collapse), full precision.
        let whole_muu = 4.821_956_449_409_315_6e1;
        let whole_mu1 = 2.000_000_000_000_002_5e1;
        assert!(
            b.muu < whole_muu * 0.8,
            "staggered muu {:.17e} is not strictly tighter than the whole-net {whole_muu:.17e}",
            b.muu
        );
        assert!(
            b.mu1 < whole_mu1 * 0.9,
            "staggered mu1 {:.17e} is not strictly tighter than the whole-net {whole_mu1:.17e}",
            b.mu1
        );
        // And a smooth fixture keeps the "or equal" half honest: the
        // three second-partial channels peak in one cell there, so the
        // fold reproduces the whole-net number exactly, while the
        // first-partial `mv1` still gains.
        let w = wavy();
        let bw = nurbs_face_bound(&w, FaceKey::default()).expect("covered");
        assert!(
            bw.muu == 1.039_409_499_704_883_5e1,
            "wavy muu moved: {:.17e}",
            bw.muu
        );
        assert!(
            bw.mv1 < 4.335_017_346_749_23,
            "wavy mv1 {:.17e} did not gain on the whole-net figure",
            bw.mv1
        );
    }

    /// **CERT-10 red row: the rational per-cell reading is the SIGNED
    /// one.** The magnitude reading applied the triangle inequality to
    /// the quotient rule (all `+`, divide by the smallest weight); the
    /// signed one evaluates the quotient rule itself in the ring, with
    /// the true minus signs and the whole weight hull as the divisor.
    /// The signed reading is strictly tighter and both are sound, so
    /// the shipped grid sizing reads the signed one.
    #[test]
    fn cert10_rational_cell_reading_is_the_signed_hull() {
        for (name, s) in [
            ("quarter_cylinder", quarter_cylinder()),
            ("wavy_rational", wavy_rational()),
            ("pie_wall", pie_wall()),
        ] {
            let cells = patch_bound::patch_cells(&s).expect("covered");
            let shipped = nurbs_cell_bounds(&s, FaceKey::default()).expect("covered");
            assert_eq!(cells.len(), shipped.len());
            for (c, b) in cells.iter().zip(&shipped) {
                for (what, got, want) in [
                    (
                        "muu",
                        b.bound.muu,
                        cell_component(patch_bound::sq_norm(c.s_uu)),
                    ),
                    (
                        "muv",
                        b.bound.muv,
                        cell_component(patch_bound::sq_norm(c.s_uv)),
                    ),
                    (
                        "mvv",
                        b.bound.mvv,
                        cell_component(patch_bound::sq_norm(c.s_vv)),
                    ),
                    (
                        "mu1",
                        b.bound.mu1,
                        cell_component(patch_bound::sq_norm(c.s_u)),
                    ),
                    (
                        "mv1",
                        b.bound.mv1,
                        cell_component(patch_bound::sq_norm(c.s_v)),
                    ),
                ] {
                    assert!(
                        got == want,
                        "{name}: cell u{:?} v{:?} shipped {what} {got:.17e} is not the \
                         signed reading {want:.17e}",
                        c.u,
                        c.v
                    );
                }
            }
        }
    }

    /// **CERT-10 red row: the signed-vs-magnitude width gap on a
    /// rational face.** The magnitude figures below were MEASURED on
    /// this tree before the retirement and are pinned as the record of
    /// what the retired reading answered. `muv` is where the triangle
    /// inequality costs most: on the quarter cylinder the quotient
    /// rule's minus signs cancel almost entirely, and the magnitude
    /// spelling — which cannot see a sign — reports eleven times the
    /// signed one.
    #[test]
    fn cert10_signed_reading_is_strictly_tighter_on_a_rational_face() {
        let s = quarter_cylinder();
        let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
        let mag_muu = 3.942_263_838_556_179_7;
        let mag_muv = 1.266_375_820_315_083_4;
        assert!(
            b.muu < mag_muu * 0.78,
            "quarter cylinder muu {:.17e} did not tighten on the magnitude {mag_muu:.17e}",
            b.muu
        );
        assert!(
            b.muv < mag_muv * 0.1,
            "quarter cylinder muv {:.17e} did not tighten on the magnitude {mag_muv:.17e}",
            b.muv
        );
    }

    /// **The rational-face grid re-sizing, in digits** (issue 1006's
    /// ruling: the retirement "coarsens/re-sizes rational-face grids",
    /// and the figures are owed). Reports the split selection's steps
    /// and whole-chart division counts for the retired MAGNITUDE
    /// reading (pinned from the measurement taken before it was
    /// removed) beside the shipped SIGNED one, at the tour's own
    /// delta_s decade.
    ///
    /// `#[ignore]`d: a measurement, not an assertion.
    #[test]
    #[ignore = "measurement harness: prints the rational-face grid re-sizing"]
    fn cert10_rational_grid_resizing() {
        // Measured on this tree before the retirement, full precision.
        let retired = [
            (
                "quarter_cylinder",
                NurbsFaceBound {
                    muu: 3.942_263_838_556_179_7,
                    muv: 1.266_375_820_315_083_4,
                    mvv: 4.250_461_439_678_581e-15,
                    mu1: 1.758_098_729_671_621_3,
                    mv1: 1.064_513_033_689_903,
                },
                quarter_cylinder(),
            ),
            (
                "pie_wall",
                NurbsFaceBound {
                    muu: 3.757_781_184_602_715_4,
                    muv: 1.187_199_279_294_388_2,
                    mvv: 4.202_650_729_863_987_4e-15,
                    mu1: 1.736_856_620_662_045_7,
                    mv1: 1.060_465_116_279_076_4,
                },
                pie_wall(),
            ),
            (
                "wavy_rational",
                NurbsFaceBound {
                    muu: 2.102_738_222_528_367_3e3,
                    muv: 7.072_400_295_746_917e2,
                    mvv: 3.944_546_314_591_795_5e2,
                    mu1: 3.783_331_582_921_541_5e1,
                    mv1: 1.429_748_990_854_184_7e1,
                },
                wavy_rational(),
            ),
        ];
        for delta_s in [1e-2, 4e-3, 1e-3] {
            println!("\ndelta_s = {delta_s:.0e}");
            println!(
                "{:<18} {:>6} {:>13} {:>13} {:>9} {:>9} {:>7}",
                "face", "read", "h_u", "h_v", "div_u", "div_v", "cells"
            );
            for (name, old, s) in &retired {
                let new = nurbs_face_bound(s, FaceKey::default()).expect("covered");
                for (tag, b) in [("mag", *old), ("signed", new)] {
                    let (hu, hv) = b.grid_steps(delta_s);
                    // The chart is the unit square in both directions
                    // for every fixture here.
                    let du = if hu.is_finite() && hu > 0.0 {
                        (1.0f64 / hu).ceil().max(1.0)
                    } else {
                        1.0
                    };
                    let dv = if hv.is_finite() && hv > 0.0 {
                        (1.0f64 / hv).ceil().max(1.0)
                    } else {
                        1.0
                    };
                    println!(
                        "{name:<18} {tag:>6} {hu:>13.6e} {hv:>13.6e} {du:>9.0} {dv:>9.0} \
                         {:>7.0}",
                        du * dv
                    );
                }
            }
        }
    }
}

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
//! exactly [`geom_core::spline::SplineCoeffs::derivative_coeffs`], iterated):
//! `S_uu` differences twice along `u`, `S_vv` twice along `v`, `S_uv`
//! once along each. The B-spline bases in both directions are
//! nonnegative partitions of unity (degree ≥ 0), so every value of
//! `S_uu` (etc.) is a convex combination of its derived coefficients
//! and lies in their hull — the C2.2 mechanism `chords` already uses
//! for edge carriers, lifted to the surface. Componentwise hulls give
//! `sup‖S_uu‖ ≤ √(Σ_c sup²)` = [`NurbsFaceBound::muu`], and likewise
//! `muv`, `mvv`. Rounding: interval (ring) arithmetic end to end —
//! including the rational arm's pieces, which are cut from
//! certification enclosures of the described net rather than in `f64` —
//! with a final `next_up` on the square root.
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
//! `mvv = r` this is twice the torus certificate's
//! `(A·Δu² + 2B·Δu·Δv + C·Δv²)/8` at the same sups — the same
//! anisotropic accounting, spent here through an affine detour at the
//! cell centre (`Q/8` twice) where `crate::sizing::torus_grid_steps`
//! bounds the interpolant at each point directly and pays `Q/8` once.)
//!
//! As everywhere in this crate the two documented additive slacks (≤ ε
//! boundary-carrier residual, f64 evaluation rounding) sit OUTSIDE the
//! bound: the honest promise is δ + ε (crate docs).
//!
//! # The rational arm (M8-5)
//!
//! A RATIONAL face's second partials are quotients `M/W³` with
//! polynomial numerators, and the sups are read from
//! [`patch_bound`]'s rational arm (its module docs, "The rational
//! arm"): on each dyadic piece of a span cell (halved from the cell's
//! quotient, formed once over it), numerator and
//! denominator raised to one degree, the partial is a convex
//! combination of the coefficient ratios `m_k/b_k`, so its signed
//! enclosure is their coordinate range. The pieces are cut from the
//! DESCRIBED face in certification arithmetic, so the sups bound it
//! and not a rounded neighbour. A degree-1 direction's second partial
//! is not the integral arm's exact zero — a rational degree-1
//! direction genuinely curves in parameter (a Möbius-reparameterized
//! ruling) — and the quotient's numerator carries that: its doubled
//! cross term alone survives.
//!
//! The certificate consumer is unchanged: the Q/4 Taylor bound only
//! needs C¹ with an a.e. second-derivative bound, and the C¹ gate
//! (interior multiplicities ≤ p − 1, on the homogeneous nets) plus
//! `w > 0` gives exactly that for `S = A/w`.
//!
//! **Conservatism** (the speed meter's posture): the answer is a
//! bound, not an estimate. Its slack is the spread of the coefficient
//! ratios over a piece, which extreme weight ratios widen (the
//! falsifier rows print the ratios). The cost is only grid density,
//! and a δ fine enough to overflow the 2²⁴ cap refuses typed
//! ([`crate::sizing::ceil_count`]) — never a wrong mesh.
//!
//! # Grid sizing (heuristic; the certificate is the guarantee)
//!
//! Budgeting a triangle's box at two grid cells per axis
//! (`a_u ≤ 2·h_u`, `a_v ≤ 2·h_v`), a step pair `(h_u, h_v)` is legal
//! exactly when it lies inside the certified ellipse
//!
//! ```text
//! muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ b .
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
//! the band containing it ([`NurbsCellGrid::cert`]). Each band keeps
//! its own column count; the trimmed lane triangulates over the face's
//! [`lattice_metric`], which is what keeps anisotropic boundary slivers
//! out ([`NurbsCellGrid::band_schedule`]).
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
//! bitwise 1, all strictly positive) through the quotient arm
//! above (M8-5; the loft/sweep wall class plus the arc-walled bodies
//! M8-2 made buildable). Refused typed
//! ([`TessellateError::UnsupportedNurbsFace`]): illegal rational
//! descriptions (a non-positive/non-finite weight voids the
//! convex-combination licence), C⁰ creases (the Taylor remainder
//! needs C¹ — for the standard multi-arc rational quadratic that
//! means split at the double knots), degree-0 directions, and
//! refused/non-finite hulls. The placeholder refuses
//! [`TessellateError::UnsupportedSurface`] upstream in `trimmed`.

use geom::NurbsSurface;
use geom_brep::patch_bound::{self, PatchBoundError};
use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, norm_sq};
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
fn cell_readings(c: &patch_bound::PatchCell) -> [Interval; 5] {
    [
        norm_sq(&c.s_uu),
        norm_sq(&c.s_uv),
        norm_sq(&c.s_vv),
        norm_sq(&c.s_u),
        norm_sq(&c.s_v),
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
    ///
    /// **On the rational arm the exact zero is the quotient's.** A
    /// degree-1 direction's numerator `M = A″W² − AW″W − 2A′WW′ + 2AW′²`
    /// keeps its doubled cross term (module docs), which vanishes where
    /// the weights are constant along the direction and the points
    /// affine in it: the direction is then affine in ℝ, every
    /// coefficient of `M` comes out an exact zero, and `muu` (or `mvv`)
    /// is the proved `0.0`, so the degenerate arms are taken on a
    /// direction the bound proved. The quarter cylinder, whose weights
    /// are constant along `v`, certifies `mvv = 0.0`; a degree-1
    /// direction whose weights vary along it (a Möbius ruling) reports
    /// its genuine curvature in parameter
    /// (`rational_degree_one_direction_is_not_taken_as_degenerate`).
    pub muu: f64,
    /// `sup ‖S_uv‖`.
    pub muv: f64,
    /// `sup ‖S_vv‖` (exact `0.0` for the v-direction analogue, on the
    /// same terms as [`Self::muu`]).
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

    /// The `(h_u, h_v)` UV grid steps for sizing target `budget` —
    /// [`Self::split_steps`] without the constraint-activity flag.
    /// `f64::INFINITY` for a direction nothing constrains
    /// ([`crate::sizing::ceil_count`] turns that into one cell).
    pub(crate) fn grid_steps(&self, budget: f64) -> (f64, f64) {
        let s = self.split_steps(budget);
        (s.hu, s.hv)
    }

    /// **The split selection** (TESS-SPLIT, the ratified aspect policy
    /// — `docs/TESS-BUDGET.md`, PR #568): the cell-minimizing point on
    /// the certified ellipse
    /// `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ b`, subject to the 3-D
    /// aspect cap [`ASPECT_CAP`].
    ///
    /// # The closed form
    ///
    /// Parametrize the ellipse boundary by the parameter aspect
    /// `t = h_u/h_v` (NOTE the convention: `tess-meter`'s
    /// counterfactual scan parametrizes the same ellipse by the
    /// RECIPROCAL, `t = h_v/h_u` — the two are separate derivations
    /// in separate cargo roots by design, and their `t`s must never
    /// be compared directly); then `h_v = √(b/q(t))` with
    /// `q(t) = muu·t² + 2·muv·t + mvv`, and the cell count over a box
    /// is proportional to `1/(h_u·h_v) = q(t)/(t·b)`. Minimizing
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
    /// exact zeros ([`cell_component`] preserves them, from a ring that
    /// pads only inexact operations), never a threshold — so an arm is
    /// taken only where the assembly proved the zero: an integral
    /// degree-1 direction, or a rational face with a constant weight
    /// net, which is the integral face it equals. Any other rational
    /// degree-1 direction's sup is positive dust ([`Self::muu`]) and
    /// takes the generic formula, which is what its surviving cross
    /// terms deserve. Each degenerate
    /// case gets its own arm rather than a limit of the generic formula
    /// (spec D-1; the test pins the arm against the limit):
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
    pub(crate) fn split_steps(&self, budget: f64) -> SplitSteps {
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
        let hv = (budget / q).sqrt();
        SplitSteps {
            hu: t * hv,
            hv,
            cap,
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
/// ([`NurbsFaceBound::split_steps`]). **Quality**: a heuristic that
/// keeps the split from choosing a long thin strip, not a promise
/// about triangle shape (`crates/mesh/README.md`).
///
/// RATIFIED at 16 (docs/TESS-BUDGET.md "The split schedule's aspect
/// policy", PR #568): one octave beyond the 4–8 range typical mesh
/// quality bounds tolerate, which captures most of the measured ~4.1x
/// split slack on ruled walls — where the honest optimum is mildly
/// anisotropic — while refusing the degenerate strip the unconstrained
/// optimum degenerates to (leaf_a f2: 70×328 → 1×4905, parameter
/// aspect ~5·10³). Mesh quality is a consumer contract; nothing
/// downstream was polled on strips. The dial is re-tunable by ordinary
/// measurement + baseline re-cut. It bounds 3-D cell shape only: the
/// PARAMETER anisotropy that slivers a Delaunay triangulation is the
/// trimmed lane's to cure, and it does so with the face's
/// [`lattice_metric`] ([`NurbsCellGrid::band_schedule`]).
///
/// **What is guarded, and what is deliberately not** (the
/// measured-claim rule): the cap is a bound on the SELECTION — the
/// chosen `(h_u, h_v)`'s sup-mapped edge ratio — and THAT claim has a
/// mechanical guard, the fuzz row
/// `split_steps_stay_on_the_ellipse_and_inside_the_cap` (red on any
/// point outside the window, replayable seed). The EMITTED lattice's
/// FFF aspect carries no guard and no register on purpose: `ceil`
/// moves each band off the chosen point by up to a step, so an
/// emitted-side `≤ A` assertion would be asserting a claim the design
/// does not make. Emitted-side measurements (the TESS-SPLIT PR's
/// per-band numbers) are evidence about a head, not an invariant; the
/// re-measured register for the emitted lattice is the budget CSV's
/// `realized_aspect` column (raw-UV parameter aspect).
pub(crate) const ASPECT_CAP: f64 = 16.0;

fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}

/// One knot-span cell's own certified Hessian bound (the
/// [`nurbs_cell_grid`] assembly restricted to the cell's active
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

/// The shared `Σ sup² → sup` collapse: `√hi`, rounded out. A refused
/// enclosure answers NaN, which every consumer treats as "unbounded/refused".
///
/// **An exactly-zero enclosure collapses to exactly `0.0`** — sound
/// (the sup of the zero enclosure IS zero; the root steps outward only
/// where it rounds, and `√0` does not round) and load-bearing: the
/// split selection's degenerate-direction predicates
/// ([`NurbsFaceBound::split_steps`]) are decided on `== 0.0`, so the
/// structurally-exact zero of a degree-1 direction must not leave here
/// as subnormal dust.
///
/// **The zero reaches here exact because interval arithmetic's arithmetic keeps
/// it.** [`norm_sq`] sums the three dependent squares, and the
/// backend pads only where an
/// operation was inexact: `0 · 0` is exact by the zero-factor corner
/// convention and `0 + 0` by the TwoSum witness
/// (`interval_transcendentals`' `mul_lo`/`mul_hi`, `add_lo`/`add_hi`),
/// so a direction whose derivative net is the exact zero assembles to
/// `[0, 0]`, whose root is exactly `[0, 0]`. An enclosure that is merely
/// NARROW does not, and must not: it is an enclosure of something the
/// assembly could not prove zero.
fn cell_component(sq: Interval) -> f64 {
    sq.sqrt().mag()
}

/// **The per-cell bounds** (TESS-SPAN, promoted from the #320 sizing
/// diagnostic): the same certified assembly as [`face_bound`],
/// reported per knot-span cell instead of maxed over the patch.
///
/// Since TESS-SPAN this is the SHIPPED lane's sizing input (through
/// [`nurbs_cell_grid`]); `crate::budget`'s meter reads it too, for the
/// span-sizing prediction it checks the lane against. Nothing here can
/// loosen a certificate: every cell's bound is the same statement
/// [`face_bound`] folds, read over a subset of the patch.
///
/// Granularity differs by arm, because each arm reports at the
/// granularity its own certified assembly already works in: the
/// integral arm's cells are the raw knot spans, the rational arm's are
/// the [`patch_bound::RATIONAL_CERT_SPLITS`] pieces of each span cell. The
/// budget row carries the cell count, so a reader is never guessing
/// which.
///
/// The max over the returned cells IS the face bound
/// ([`NurbsCellGrid::patch`] folds exactly these cells); this module's
/// own `no_cell_exceeds_the_whole_patch_bound` test asserts the
/// inequality componentwise and
/// `cert10_whole_face_bound_is_the_per_cell_fold` asserts the
/// equality.
///
/// # Errors
///
/// As [`patch_bound::patch_cells_across_creases`] — the rational arm
/// admits a C⁰ crease, which [`NurbsCellGrid::cert`] then keeps every
/// certified box off; otherwise the same gates, arms and refusals as
/// [`patch_bound::patch_cells`], reported in this crate's error type.
pub(crate) fn nurbs_cell_bounds(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<Vec<CellBound>, TessellateError> {
    Ok(patch_bound::patch_cells_across_creases(n)
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

/// One tessellation's memo of certified NURBS cell tables, **one entry
/// per described NURBS face and nothing per edge**.
///
/// It lives HERE, beside the assembly it remembers, rather than in
/// either pass that reads it: [`crate::chords`]' adjacent-face
/// tightening and [`crate::trimmed`]'s band schedule both need the
/// same per-face fact, and a cache hosted inside one of its two
/// consumers is the shape that drifts.
///
/// **Who writes it, and when.** Its two consumers read it in two
/// different phases of [`crate::tessellate()`], and the phases are what
/// make one `HashMap` safe under the per-face parallel map:
///
/// * the CHORD PASS walks the edge arena and fills the entry of every
///   described NURBS face adjacent to an edge ([`face_bound`], through
///   `chords::nurbs_tighten`). It is a per-edge walk writing per-FACE
///   entries, and it is serial and complete before any face's lane
///   runs;
/// * a FACE's LANE then reads its OWN face's entry, once, through
///   [`face_cells`], which takes `&FaceBounds`. No lane reads another
///   face's entry and no lane writes.
///
/// So the map needs no lock and no per-face split of this type: the
/// only mutation is the chord pass's, and it has finished.
pub(crate) type FaceBounds = std::collections::HashMap<FaceKey, NurbsCellGrid>;

/// The certified cell table of a described NURBS face, **as the chord
/// pass left it** — the lane's door onto [`FaceBounds`], and a plain
/// lookup.
///
/// **Why the lanes get a different door from the chord pass, and why
/// this one does not assemble.** [`face_bound`] takes `&mut` and is the
/// FILL door: `chords::nurbs_tighten` calls it for every described
/// NURBS face adjacent to an edge, which in a body
/// whose faces have boundaries is every described NURBS face there is.
/// That pass runs to completion before the per-face parallel map, so by
/// the time a lane asks, the answer is already there and every lane can
/// hold `&FaceBounds`. Assembling here as a fallback would be a second
/// way to reach `nurbs_cell_grid` that no body takes, and a silent one:
/// it would hide a face the chord pass failed to fill instead of
/// refusing.
///
/// # Errors
///
/// [`TessellateError::MissingEntity`] when the entry is absent, which
/// for a body the chord pass accepted means a face that owns no
/// half-edge — a malformed body, and the same class as this crate's
/// other dangling-key refusals. The certificate refusals
/// ([`nurbs_cell_grid`]'s) cannot arrive here: they happen in the chord
/// pass, which refuses the whole tessellation before any lane runs.
pub(crate) fn face_cells(
    bounds: &FaceBounds,
    fk: FaceKey,
) -> Result<&NurbsCellGrid, TessellateError> {
    bounds.get(&fk).ok_or(TessellateError::MissingEntity {
        what: "face NURBS cell table",
    })
}

/// The whole-patch bound of a described NURBS face — the chord pass's
/// door, and **the only thing that writes [`FaceBounds`]**: the cell
/// table is assembled on first ask and remembered for the rest of the
/// tessellation, and the bound returned is a reading of it
/// ([`NurbsCellGrid::patch`]).
///
/// **The memo holds the CELL GRID, not the whole-patch bound, and that
/// is the whole point of it.** A memo of the coarser fact would make
/// the finer one unreachable and force the second consumer to
/// reassemble. It was measured doing exactly that: with a
/// `NurbsFaceBound` memo, a swept elbow ran `patch_cells` twice per
/// described NURBS face — once for the chord pass, once for the trimmed
/// lane — because [`crate::tessellate()`] runs
/// [`crate::chords::compute_chords`] BEFORE the per-face dispatch, so
/// the trimmed lane's ask was always a memo hit on a value it could not
/// refine. `crates/mesh/tests/cert10r1_assembly_accounting.rs` pins the
/// count.
///
/// # Errors
///
/// As [`nurbs_cell_grid`] — a face outside the certified inventory
/// refuses here exactly as it would there, on the first ask and (from
/// the memo's absence) on every later one. The refusal CLASS is
/// unchanged from a whole-patch memo: the per-cell finite check and the
/// whole-patch one refuse the same faces with the same prose, because
/// the fold's hull carries a refused or infinite cell straight into
/// the whole-patch number. A refusal here refuses the whole
/// tessellation, in the chord pass, before any lane runs — which is why
/// [`face_cells`] never has to answer for one.
pub(crate) fn face_bound(
    memo: &mut FaceBounds,
    payload: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    // The Entry API, not `contains_key` + `insert` (clippy::map_entry),
    // and the shape matters beyond the lint: the assembly's `?` sits
    // INSIDE the vacant arm, so a face that refuses leaves the memo
    // untouched and refuses identically on every later ask — the
    // "refuses on the first ask and (from the memo's absence) on every
    // later one" contract above, made structural.
    let grid = match memo.entry(fk) {
        std::collections::hash_map::Entry::Occupied(e) => e.into_mut(),
        std::collections::hash_map::Entry::Vacant(e) => e.insert(nurbs_cell_grid(payload, fk)?),
    };
    Ok(grid.patch())
}

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
}

impl BandCounts {
    /// The band's realized parameter aspect `s_u/s_v` over a trim box
    /// `du` wide: post-`ceil` spacings, the lattice that exists.
    #[allow(clippy::cast_precision_loss)]
    pub(crate) fn aspect(&self, du: f64) -> f64 {
        (du / self.nuc as f64) / ((self.vb - self.va) / self.nvc as f64)
    }
}

/// **The NURBS lane's worst-triangle theorem, and the split of δ it
/// is spent at.** The lane's lattice is sized on the budget
/// `b = LATTICE_SHARE·δ` ([`lattice_budget`]): a band sized by
/// [`NurbsFaceBound::split_steps`] spends
/// `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ b` as `x + z + y`
/// (`x = muu·h_u² = s_u·b`, `z = 2·muv·h_u·h_v`, `y = mvv·h_v²`).
///
/// **The worst admissible triangle is the off-row sliver.** Over the
/// lattice metric a band of residual aspect `1/a` (`a ≥ 1`) has rows
/// `a` columns tall. A point between two of its rows has an empty
/// circle about one row across, so a Delaunay triangle on it spans at
/// most a box `a·s_u × s_v` (`s ≤ h`). That holds for two off-row
/// vertices too: adjacent chord points of a slanted trim edge can span
/// the circle's diameter. Its certificate ([`NurbsFaceBound::cert`]) is
/// then
///
/// `¼(muu·a²·s_u² + 2·muv·a·s_u·s_v + mvv·s_v²) ≤ ¼(a²·x + a·z + y) ≤ a²/4 · b`,
///
/// since `a² ≥ a ≥ 1`. The worst split puts the budget in `u`. That is
/// the aspect cap's split on a ruled wall (`mvv = 0`, `s_u ≤ 1`): on the
/// uneven-station cylinder every band is cap-bound, with `s_u` 0.90–0.999,
/// short of 1 only by the ceil on the counts. A lattice cell-half
/// (`a = 1`) certifies at `b/4`.
///
/// So the sliver lands at δ exactly when `a²·LATTICE_SHARE/4 = 1`, and
/// [`MAX_RESIDUAL_SQ`] is that solved for the aspect: a band at the
/// residual line puts its worst triangle at δ at any share, and one
/// under it lands below δ. The share chooses between a coarser lattice
/// and fewer raised rows; it is the lane's sizing split, typed rather
/// than measured
/// (`work/tess/the-nurbs-lattice-share-is-not-chosen-by-measurement.md`). The only slack is the ceil (`s_u < 1`). The
/// centroid retries of `trimmed::MAX_GRID_RETRIES` are a backstop,
/// not part of the bound.
///
/// A single off-row vertex has margin: the empty circle keeps its
/// triangles within `(a + 1)/2` columns, which at this share would
/// allow `a ≤ 4√2 − 1 ≈ 4.66`. The two-vertex case does not, so it
/// cannot carry the line.
///
/// `nurbs_cert::tests::a_strip_lattice_off_row_sliver_stays_inside_the_box_bound`
/// tests the bound on a real CDT, and
/// `nurbs_cert::tests::the_residual_line_lands_the_strip_sliver_at_delta_over_every_split`
/// pins the arithmetic.
pub(crate) const LATTICE_SHARE: f64 = 0.5;

/// The NURBS lane's lattice budget for chordal tolerance `delta`
/// ([`LATTICE_SHARE`]): what [`NurbsFaceBound::grid_steps`] is sized
/// on, in the lane and in the chord pass's tightening alike.
pub(crate) fn lattice_budget(delta: f64) -> f64 {
    delta * LATTICE_SHARE
}

/// The square of the residual aspect past which a band's off-row
/// points sliver over the lattice metric ([`NurbsCellGrid::band_schedule`]
/// raises such a band's rows to it): the sliver theorem at
/// [`LATTICE_SHARE`] solved for the aspect, `a² = 4/LATTICE_SHARE`.
pub(crate) const MAX_RESIDUAL_SQ: f64 = 4.0 / LATTICE_SHARE;

/// The shipped band schedule ([`NurbsCellGrid::band_schedule`]): the
/// bands, and the lattice metric the trimmed lane triangulates in.
#[derive(Clone, Debug)]
pub(crate) struct BandSchedule {
    /// The v-bands, in order.
    pub bands: Vec<BandCounts>,
    /// The face's lattice metric: a power of two `k`, and the trimmed
    /// lane's CDT runs on `(u/k, v)` ([`lattice_metric`]).
    pub metric: f64,
}

/// **The lattice metric** a face's trimmed CDT runs in: the power of
/// two nearest (in `log₂`) the geometric mean of the extreme band
/// aspects, `√(min a_b · max a_b)`, `a_b = s_u/s_v` realized by each
/// band's own schedule.
///
/// Delaunay is not stretch-invariant. In raw UV, a band whose columns
/// are far narrower than its rows (a ruled wall's strip: `a_b ≈ 0.03`
/// on a lofted circle) gives any point off its lattice an empty circle
/// many columns wide, and a sliver across them; in `(u/k, v)` the same
/// lattice is near square and the circle a column or two.
///
/// A POWER OF TWO so the map is exact in f64: `u/k` rounds nothing, so
/// two points coincide in the scaled plane exactly when they coincide
/// in UV, and every exact-coincidence rule the lane relies on (the
/// grid-on-constraint drop, crease snapping, [`crate::planar::Pinches`],
/// the handle merge) behaves as before. `1` when no band has a finite
/// positive aspect.
pub(crate) fn lattice_metric(aspects: impl Iterator<Item = f64>) -> f64 {
    let (mut lo, mut hi) = (f64::INFINITY, 0.0f64);
    for a in aspects.filter(|a| a.is_finite() && *a > 0.0) {
        lo = lo.min(a);
        hi = hi.max(a);
    }
    if hi == 0.0 {
        return 1.0;
    }
    // `(lo·hi)^½` in logs: no product to overflow, and the exponent is
    // the rounded mean of the two exponents.
    let e = (0.5 * (lo.log2() + hi.log2())).round();
    if !e.is_finite() {
        return 1.0;
    }
    // A finite positive aspect's `log₂` lies within ±1075, so `e` is
    // an exact small integer and `2^e` is exact for |e| ≤ 1022.
    #[allow(clippy::cast_possible_truncation)]
    2f64.powi(e.clamp(-1022.0, 1023.0) as i32)
}

/// **The shipped per-cell certificate table** (TESS-SPAN): one face's
/// [`nurbs_cell_bounds`] assembled into a tensor lookup — sorted cell
/// boundary values per direction, and each cell's own certified
/// Hessian bound. The trimmed lane sizes its grid from it (per-cell
/// steps; rows on the `v` cell boundaries, columns uniform over the
/// trim box and so across the `u` ones) and certifies each triangle
/// from the cells its box covers ([`Self::cert`]), one or several.
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
    /// The face's C⁰ creases in `u`: interior knots of multiplicity at
    /// least the degree, where the surface's first derivative may jump.
    /// A triangle whose box strictly crosses one has no certificate
    /// ([`Self::cert`]).
    creases_u: Vec<f64>,
    /// The face's C⁰ creases in `v`, as [`Self::creases_u`].
    creases_v: Vec<f64>,
}

impl NurbsCellGrid {
    /// **The whole-patch bound, folded off the cells already
    /// assembled** — componentwise max over this grid's own cells.
    ///
    /// Identical to the fold over [`nurbs_cell_bounds`] on the same
    /// face, and the
    /// identity is exact rather than approximate: that function hulls
    /// the per-cell squared-sum ENCLOSURES and collapses once, this one
    /// collapses per cell and takes the f64 max, and
    /// [`cell_component`] is monotone in the enclosure's `hi` — so
    /// `max_c sqrt(hi_c)` and `sqrt(max_c hi_c)` are the same f64. The
    /// NaN asymmetry that makes interval-level accumulation
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
/// finite-checked cell by cell (the whole-patch refusal,
/// applied where the shipped consumer now reads) and assembled into a
/// [`NurbsCellGrid`].
///
/// # Errors
///
/// As [`nurbs_cell_bounds`], plus the unbounded/refused refusal when
/// any single cell's bound fails the finite check.
pub(crate) fn nurbs_cell_grid(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsCellGrid, TessellateError> {
    crate::budget::note_assembly();
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
                note: "NURBS face second-derivative hull is unbounded/refused — \
                       outside the certified inventory",
            });
        }
    }
    Ok(NurbsCellGrid::from_cells(
        &cells,
        (creases(n.knot_values_u()), creases(n.knot_values_v())),
    ))
}

/// The C⁰ creases of one direction, as held knot values: the runs
/// [`geom_core::spline::KnotStructure::crease_runs`] names, the same
/// statement the C¹ gate (`KnotVector::c1`) refuses on.
fn creases(knots: &geom_core::spline::Knots<f64>) -> Vec<f64> {
    knots.crease_values()
}

impl NurbsCellGrid {
    /// Assembles the tensor lookup. Both arms of [`nurbs_cell_bounds`]
    /// emit one cell per (nonempty u-span × nonempty v-span) of their
    /// own knot structure, so the cell rectangles ARE a tensor grid;
    /// the asserts are the fail-loud statement of that invariant, not
    /// a recovery path.
    ///
    /// `(creases_u, creases_v)` are the face's C⁰ creases ([`creases`]),
    /// an argument rather than a later step so no grid is built without
    /// them: [`Self::cert`] refuses a triangle across one.
    fn from_cells(cells: &[CellBound], (creases_u, creases_v): (Vec<f64>, Vec<f64>)) -> Self {
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
            creases_u,
            creases_v,
        }
    }

    /// The face's C⁰ creases, `(u, v)`: the lines the trimmed lane
    /// cuts its triangulation along, so that no triangle crosses one.
    pub(crate) fn creases(&self) -> (&[f64], &[f64]) {
        (&self.creases_u, &self.creases_v)
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
    /// interior band boundaries, each band's `(nuc, nvc)` counts, and
    /// the face's [`lattice_metric`] — one derivation consumed by BOTH
    /// the trimmed lane's candidate generation and the budget meter's
    /// prediction, so the two cannot drift.
    ///
    /// Per band: `nuc` and `nvc` from the band bound's own steps, the
    /// split selection [`NurbsFaceBound::split_steps`] (TESS-SPLIT: the
    /// aspect-capped cell minimizer). Every emitted cell is a point of
    /// its band's own certified region.
    ///
    /// **No column snap.** A strip lattice admits a Delaunay-legal
    /// sliver of certificate up to `aspect²/4 · b` beside any point
    /// off its columns (a neighbour band's columns on the shared cut
    /// line, a rim chord point, a refinement centroid), and no local
    /// insertion cures it. The trimmed lane triangulates over the
    /// metric returned here, where each band's lattice is near square,
    /// so bands are not aligned to a shared column count. Aligning them
    /// keeps no measured face sound
    /// (`work/tess/the-lower-malign-arm-is-unwitnessed` carries the
    /// measurement); on uneven-station lofts it saves triangles
    /// (`work/budget/uneven-station-lofts-cost-triangles-without-the-column-snap`).
    ///
    /// **Rows for narrow-column bands.** One face-wide metric cannot
    /// square bands whose aspects spread far apart (an uneven-station
    /// loft's thin first knot span beside thick ones: 166:1). A band
    /// left below `1/√MAX_RESIDUAL_SQ` over the metric has columns far
    /// narrower than its rows, and a point between two of its rows (a
    /// trim chord point) fans a sliver across them. Its rows are raised
    /// until its residual reaches the line; its columns, and every other
    /// band, are untouched. The opposite arm, columns too wide, gets no
    /// remedy: no measured face slivers there. On the ruled walls
    /// measured, `mvv ≈ 0`, so a wide band's `v` extent costs nothing. A
    /// doubly curved wide-column band is unwitnessed.
    ///
    /// A sliver the rows do not cure still fails the face's certificate,
    /// and the face then refuses typed
    /// ([`TessellateError::CertificateExceeded`]): loud, never a wrong
    /// mesh. `trimmed::tests::a_face_left_slivered_refuses_typed_through_tessellate`
    /// pins that refusal with the rows switched off;
    /// `lofted_circle::an_uneven_station_cylinder_meshes_at_every_review_delta`
    /// pins that with them the face meshes.
    ///
    /// # Errors
    ///
    /// [`TessellateError::ResolutionOverflow`] via
    /// [`crate::sizing::ceil_count`], as the uniform schedule.
    pub(crate) fn band_schedule(
        &self,
        u: (f64, f64),
        v: (f64, f64),
        budget: f64,
    ) -> Result<BandSchedule, TessellateError> {
        let du = u.1 - u.0;
        let mut edges = vec![v.0];
        edges.extend(self.v_cuts.iter().copied().filter(|c| *c > v.0 && *c < v.1));
        edges.push(v.1);
        let mut bands = Vec::with_capacity(edges.len().saturating_sub(1));
        for w in edges.windows(2) {
            let (va, vb) = (w[0], w[1]);
            // The band is found by the slab midpoint — strictly inside
            // one band, since no interior cut crosses a slab.
            let bound = self.row_bound(self.row_of(0.5 * (va + vb)));
            let steps = bound.split_steps(budget);
            let nuc = crate::sizing::ceil_count(du, steps.hu)?;
            let nvc = crate::sizing::ceil_count(vb - va, steps.hv)?;
            bands.push(BandCounts {
                va,
                vb,
                nuc,
                nvc,
                cap: steps.cap,
            });
        }
        let metric = lattice_metric(bands.iter().map(|b| b.aspect(du)));
        // Rows for a band whose columns are too narrow for its rows over
        // the metric (doc: **Rows for narrow-column bands**). The metric
        // stays the one taken from the bands' own counts.
        #[cfg(test)]
        let raise_rows = tests::ROWS_RAISED.with(std::cell::Cell::get);
        #[cfg(not(test))]
        let raise_rows = true;
        for b in bands.iter_mut().filter(|_| raise_rows) {
            let residual = b.aspect(du) / metric;
            if residual.is_finite() && residual > 0.0 && residual.powi(2) * MAX_RESIDUAL_SQ < 1.0 {
                // `ceil(nvc / (residual·√MAX_RESIDUAL_SQ))`, through the
                // crate's one count door: a row count past `MAX_COUNT`
                // refuses typed (`ResolutionOverflow`). The cap is per
                // count; the band's `nuc·nvc` is not capped here.
                #[allow(clippy::cast_precision_loss)]
                let want =
                    crate::sizing::ceil_count(b.nvc as f64, residual * MAX_RESIDUAL_SQ.sqrt())?;
                b.nvc = b.nvc.max(want);
            }
        }
        Ok(BandSchedule { bands, metric })
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
    /// boundaries, and where they do not**: the `Q/4` derivation is the
    /// integral Taylor remainder (module docs), which needs the surface
    /// C¹ along every segment it integrates, plus an a.e.
    /// second-derivative bound there. A knot of multiplicity below the
    /// degree is exactly where a C¹ surface's second derivative jumps,
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
    /// `cert_is_pinned_to_the_componentwise_sup`.)
    ///
    /// **Both arguments need C¹ across the cut, so a C⁰ crease voids
    /// them.** At an interior knot of multiplicity at least the degree
    /// the first derivative jumps, and a triangle across it deviates by
    /// the kink no second-derivative bound sees. A box that strictly
    /// crosses a crease therefore answers `+∞`, which the trimmed lane
    /// reads as a refused certificate. Straddling boxes are not only
    /// boundary fans: the trimmed lane's `u` columns are uniform over the
    /// trim box, not placed on the `u` cuts, so interior grid triangles
    /// straddle `u` cuts too. That lane cuts its triangulation along
    /// every crease ([`Self::creases`]), so on its meshes this answer
    /// is a backstop.
    ///
    /// For a box inside ONE cell this is exactly that cell's own
    /// certificate.
    pub(crate) fn cert(&self, uv: [[f64; 2]; 3]) -> f64 {
        let u_lo = min3(uv[0][0], uv[1][0], uv[2][0]);
        let u_hi = max3(uv[0][0], uv[1][0], uv[2][0]);
        let v_lo = min3(uv[0][1], uv[1][1], uv[2][1]);
        let v_hi = max3(uv[0][1], uv[1][1], uv[2][1]);
        let crosses = |creases: &[f64], lo: f64, hi: f64| creases.iter().any(|&c| lo < c && c < hi);
        if crosses(&self.creases_u, u_lo, u_hi) || crosses(&self.creases_v, v_lo, v_hi) {
            return f64::INFINITY;
        }
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

// `pub(crate)`, and only under `cfg(test)`, for two kinds of caller.
// `nurbs_cert_fuzz`'s rows take the bound doors and the sampler: that
// sibling module exists so the per-file test gate can skip the randomized
// rows without skipping the deterministic pins here, and re-declaring the
// helpers there would be a second copy of a bound this file owns. And
// every in-crate test module that states a domination takes `Domination`
// — `chords`' tests as well as the fuzz rows — so its message has one
// spelling.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
pub(crate) mod tests {
    use super::*;
    use geom_core::interval::certification::Certification;

    thread_local! {
        /// Test-only: whether [`super::NurbsCellGrid::band_schedule`] raises a
        /// narrow-column band's rows. It exists for one row,
        /// `trimmed::tests::a_face_left_slivered_refuses_typed_through_tessellate`:
        /// once rows are raised no natural body refuses
        /// [`TessellateError::CertificateExceeded`] any more, and that row
        /// turns the raise off to reach the refusal through the real call
        /// path. `cfg(test)` only, so no shipped build and no other crate
        /// can read it.
        pub(crate) static ROWS_RAISED: std::cell::Cell<bool> = const { std::cell::Cell::new(true) };
    }

    /// The whole-patch bound of a single face, with no memo — the door
    /// the rows below take. The shipped lane never has this shape (it
    /// always carries a `FaceBounds`), which is why it lives here: an
    /// un-memoized production entry is exactly how a second assembly
    /// per face gets reintroduced, and `cert10r1_assembly_accounting`
    /// is the row that would catch it.
    pub(crate) fn nurbs_face_bound(
        n: &NurbsSurface<f64>,
        fk: FaceKey,
    ) -> Result<NurbsFaceBound, TessellateError> {
        Ok(nurbs_cell_grid(n, fk)?.patch())
    }

    use geom_core::Point3;
    use geom_core::Tol;
    use geom_core::spline::KnotVector;
    use profile::test_support::bulge_loop;

    /// A componentwise domination claim — every `lesser <= greater` — and
    /// the failure message of the rows that state one through it: the
    /// sampled-second-partial, sampled-speed, cell-under-patch and
    /// curve `sup|C''|` rows of this crate's unit tests. Rows that bound
    /// a single deviation by its certificate, or a ratio by one, spell
    /// their own messages.
    ///
    /// A red on such a row is read by someone who did not write it, and
    /// is decided by the last bits as readily as by a percent. So the
    /// message says which side each number is on, names every component
    /// that escaped together with its excess, and prints each `f64` at
    /// `{:.17e}`, which no two distinct doubles share.
    ///
    /// The comparison is `<=`, and a NaN on either side fails it.
    ///
    /// **One allowance, and only where the lesser side is a SAMPLE.**
    /// A certified sup encloses the true value of an expression; a
    /// dense `f64` evaluation of the same expression is not that
    /// value — it carries the rounding of its own tens of operations,
    /// which the certificate never claimed to cover. While certification arithmetic
    /// padded one representable step outward per operation the bound
    /// absorbed that rounding by accident, and a sampled `uu` sat
    /// under a certified one it exceeds in the reals by nothing at
    /// all. It does not absorb it now: the backend pads only where the
    /// operation was inexact. So a sampled comparison allows the
    /// SAMPLER its own relative error, [`SAMPLER_ULPS`] ulps of the
    /// certified figure, and every other comparison — certificate
    /// against certificate — keeps the bare `<=`.
    pub(crate) struct Domination<'a> {
        lesser: &'a str,
        greater: &'a str,
        components: Vec<(&'a str, f64, f64)>,
        /// Relative allowance on the LESSER side, as a multiple of
        /// `f64::EPSILON`; zero for every comparison whose lesser side
        /// is itself certified.
        sampler_ulps: f64,
        /// Absolute allowance on each component's LESSER side, in the
        /// compared quantity's units, parallel to `components`; zero
        /// unless a row states the scale the sampler's rounding is
        /// relative to ([`Self::second_partials`]).
        sampler_floors: Vec<f64>,
    }

    /// The sampler's allowance: 64 ulps of the certified figure,
    /// relative. A dense-sampled `‖∂²S‖` is a rational surface
    /// evaluation, three derivative folds and a norm — tens of
    /// operations at one rounding each — so a bound on its own error
    /// is tens of ulps and this is comfortably above that without
    /// being a band the certificate could hide a real escape in: the
    /// escapes this row is for are relative 1e-3 and up, and the
    /// largest observed here is 1.7e-16.
    ///
    /// **Two other sites owe the sampler the same thing and spell it
    /// their own way** — `geom/tests/curves/hull_circle_rehearsal.rs`
    /// (absolute, derived from its one measured escape) and
    /// `review_m5_pr2_e2e.rs` (absolute, a house scale). The crate all
    /// three can reach is `geom-core`:
    /// `work/props/the-samplers-own-error-has-three-spellings-and-no-home`.
    pub(crate) const SAMPLER_ULPS: f64 = 64.0;

    impl<'a> Domination<'a> {
        /// `components` are `(name, lesser, greater)`; `lesser` and
        /// `greater` name the two SIDES, and label every number printed.
        ///
        /// # Panics
        ///
        /// If `components` is empty: a claim over no components holds
        /// whatever the code under test does.
        pub(crate) fn new(
            lesser: &'a str,
            greater: &'a str,
            components: &[(&'a str, f64, f64)],
        ) -> Self {
            assert!(
                !components.is_empty(),
                "a domination of {lesser} by {greater} over NO components holds vacuously"
            );
            Self {
                lesser,
                greater,
                components: components.to_vec(),
                sampler_ulps: 0.0,
                sampler_floors: vec![0.0; components.len()],
            }
        }

        /// Dense-sampled truth under a certified sup, component by
        /// component.
        pub(crate) fn sampled_under_certified(components: &[(&'a str, f64, f64)]) -> Self {
            Self {
                sampler_ulps: SAMPLER_ULPS,
                ..Self::new("sampled", "certified", components)
            }
        }

        /// The greater side as this comparison reads it: the certified
        /// figure, widened by the SAMPLER's own error where the lesser
        /// side is a sample and by nothing at all otherwise.
        fn allowed(&self, component: usize, greater: f64) -> f64 {
            greater
                + self.sampler_ulps * f64::EPSILON * greater.abs()
                + self.sampler_floors[component]
        }

        /// The sampled second-partial norms `(uu, uv, vv)` under `b`'s.
        ///
        /// The sampler's rounding is relative to the jet it evaluates,
        /// not to the certified second partial, which is an exact zero
        /// wherever the assembly proves one — a degree-1 direction
        /// on the integral arm or one with a constant weight net. So
        /// besides [`SAMPLER_ULPS`] of the certified figure, a sample
        /// is allowed [`SAMPLER_ULPS`] of the matching first-derivative
        /// sup — `mu1` for `uu`, `mv1` for `vv`, the larger for `uv` —
        /// which is that jet's scale on the unit-width domains these
        /// rows sample.
        ///
        /// **Where the floor is above the certified figure, the
        /// component asserts only that the sample is under the floor.**
        /// That is every exact zero, and it is also the near-unit
        /// rational pattern `[1 + ε, 1, 1, 1]` of
        /// `the_bilinear_rational_stratum_is_dominated_at_every_weight_decade`,
        /// whose `muu` and `mvv` certify as dust under the floor
        /// ([`Self::floor_only`] names such components).
        pub(crate) fn second_partials(sampled: (f64, f64, f64), b: &NurbsFaceBound) -> Self {
            let floor = |scale: f64| SAMPLER_ULPS * f64::EPSILON * scale;
            Self {
                sampler_floors: vec![floor(b.mu1), floor(b.mu1.max(b.mv1)), floor(b.mv1)],
                ..Self::sampled_under_certified(&[
                    ("uu", sampled.0, b.muu),
                    ("uv", sampled.1, b.muv),
                    ("vv", sampled.2, b.mvv),
                ])
            }
        }

        /// The components whose absolute floor is above the certified
        /// figure: there the comparison asserts only the floor.
        pub(crate) fn floor_only(&self) -> Vec<&'a str> {
            self.components
                .iter()
                .zip(&self.sampler_floors)
                .filter(|&(&(_, _, hi), &floor)| floor > hi)
                .map(|(&(name, _, _), _)| name)
                .collect()
        }

        pub(crate) fn holds(&self) -> bool {
            self.components
                .iter()
                .enumerate()
                .all(|(i, &(_, lo, hi))| lo <= self.allowed(i, hi))
        }
    }

    impl core::fmt::Display for Domination<'_> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let (lesser, greater) = (self.lesser, self.greater);
            for (i, &(name, lo, hi)) in self.components.iter().enumerate() {
                // Compared against the allowance, PRINTED as the
                // certified figure: a reader needs the number the
                // certificate carries, and the excess measured from it.
                if lo > self.allowed(i, hi) {
                    write!(
                        f,
                        "`{name}` ESCAPES: {lesser} {lo:.17e} exceeds {greater} {hi:.17e} by {:.17e} ",
                        lo - hi
                    )?;
                    // An excess relative to a `greater` of zero is `inf`,
                    // and relative to a negative one it has the wrong sign.
                    if hi > 0.0 {
                        write!(f, "({:.3e} of the {greater}); ", (lo - hi) / hi)?;
                    } else {
                        write!(
                            f,
                            "(the {greater} is not positive, so the excess has no relative figure); "
                        )?;
                    }
                } else if lo.partial_cmp(&hi).is_none() {
                    write!(
                        f,
                        "`{name}` IS UNORDERED: {lesser} {lo:.17e} against {greater} {hi:.17e}; "
                    )?;
                }
            }
            let names: Vec<&str> = self.components.iter().map(|c| c.0).collect();
            let side = |pick: fn(&(&str, f64, f64)) -> f64| -> String {
                let v: Vec<String> = self
                    .components
                    .iter()
                    .map(|c| format!("{:.17e}", pick(c)))
                    .collect();
                v.join(", ")
            };
            write!(
                f,
                "all components ({}): {lesser} ({}) against {greater} ({})",
                names.join(", "),
                side(|c| c.1),
                side(|c| c.2)
            )
        }
    }

    /// The message is the deliverable, so it is pinned on the case that
    /// needs it most: the SMALLEST escape this row can report, one just
    /// outside the sampler's allowance, in one component of three. At
    /// `{:.3e}` all three pairs of this fixture print equal; `uv` and
    /// `vv` hold with room at the 12th digit, and `uu` differs only at
    /// the 14th — which is where the allowance leaves off
    /// ([`SAMPLER_ULPS`] ulps is 1.774e-14 of this `muu`, and 1.42e-15
    /// of this face's `mu1`). `mv1` is a hundred times `mu1`, so a `uu`
    /// floor taken from `mv1` (1.42e-13) would swallow the escape.
    #[test]
    fn a_failed_domination_names_the_component_the_side_and_the_excess() {
        let sampled = (
            1.248_592_341_233_754_3_f64,
            6.659_454_728_150_955,
            4.294_173_537_974_707,
        );
        let b = NurbsFaceBound {
            muu: 1.248_592_341_233_724_3,
            muv: 6.659_454_728_151_211_5,
            mvv: 4.294_173_537_974_748,
            mu1: 0.1,
            mv1: 10.0,
        };
        assert!(
            sampled.0
                > b.muu * (1.0 + SAMPLER_ULPS * f64::EPSILON) + SAMPLER_ULPS * f64::EPSILON * b.mu1,
            "the fixture is a real escape in uu, outside the sampler's own allowance"
        );
        let d = Domination::second_partials(sampled, &b);
        assert!(!d.holds());
        let msg = d.to_string();
        assert!(
            msg.starts_with(
                "`uu` ESCAPES: sampled 1.24859234123375429e0 exceeds certified \
                 1.24859234123372431e0 by 2.99760216648792266e-14"
            ),
            "{msg}"
        );
        assert!(
            !msg.contains("`uv` ESCAPES") && !msg.contains("`vv` ESCAPES"),
            "{msg}"
        );
        assert!(
            msg.contains("sampled (1.24859234123375429e0, 6.65945472815095485e0, ")
                && msg.contains("certified (1.24859234123372431e0, 6.65945472815121153e0, "),
            "both sides are printed in full, labelled: {msg}"
        );
    }

    /// `holds` is the bare `<=`, per component: strictly under holds,
    /// equal holds, over does not, and a NaN on either side does not.
    #[test]
    fn a_domination_holds_exactly_when_every_component_is_le() {
        let one = |lo: f64, hi: f64| Domination::sampled_under_certified(&[("k", lo, hi)]);
        assert!(one(1.0, 2.0).holds(), "strictly dominated");
        assert!(one(1.0, 1.0).holds(), "equal");
        assert!(!one(2.0, 1.0).holds(), "escaped");
        assert!(!one(f64::NAN, 1.0).holds() && !one(1.0, f64::NAN).holds());
        let mixed = Domination::sampled_under_certified(&[("a", 1.0, 2.0), ("b", 2.0, 1.0)]);
        assert!(!mixed.holds(), "one escaping component fails the claim");
    }

    /// The excess is relative to the GREATER side, only the escaping
    /// components are called out, and an unordered pair is reported with
    /// the same side labels.
    #[test]
    fn a_failed_domination_reports_the_excess_relative_to_the_greater_side() {
        let d = Domination::sampled_under_certified(&[("a", 1.0, 1.0), ("b", 3.0, 2.0)]);
        let msg = d.to_string();
        assert!(
            msg.starts_with(
                "`b` ESCAPES: sampled 3.00000000000000000e0 exceeds certified \
                 2.00000000000000000e0 by 1.00000000000000000e0 (5.000e-1 of the certified); "
            ),
            "{msg}"
        );
        assert!(
            !msg.contains("`a`"),
            "an equal component is not an escape: {msg}"
        );

        let zero = Domination::sampled_under_certified(&[("uu", 1e-300, 0.0)]).to_string();
        assert!(!zero.contains("inf") && !zero.contains("NaN"), "{zero}");
        assert!(
            zero.contains("(the certified is not positive, so the excess has no relative figure)"),
            "{zero}"
        );

        let nan = Domination::new("cell", "patch", &[("k", f64::NAN, 1.0)]).to_string();
        assert!(
            nan.starts_with("`k` IS UNORDERED: cell NaN against patch 1.00000000000000000e0; "),
            "{nan}"
        );
    }

    #[test]
    #[should_panic(expected = "over NO components holds vacuously")]
    fn a_domination_over_no_components_is_refused() {
        let _ = Domination::new("sampled", "certified", &[]);
    }

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
        let d = Domination::sampled_under_certified(&[("uu", wuu, b.muu)]);
        assert!(
            wuu > 0.0 && d.holds(),
            "sampled sup|S_uu| must be positive and under the certified hull: {d}"
        );
        let d = Domination::sampled_under_certified(&[("uv", wuv, b.muv)]);
        assert!(
            wuv > 0.0 && d.holds(),
            "sampled sup|S_uv| must be positive and under the certified hull: {d}"
        );
        let d = Domination::sampled_under_certified(&[("vv", wvv, b.mvv)]);
        assert!(
            wvv > 0.0 && d.holds(),
            "sampled sup|S_vv| must be positive and under the certified hull: {d}"
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
    /// — and both grid steps come out unconstrained.
    ///
    /// **`muu` and `mvv` are pinned at `== 0.0`, not under a ceiling.**
    /// They are the structural zeros of two degree-1 directions, and a
    /// ceiling cannot tell the zero from dust: this row reds if the
    /// ring ever pads `0 + 0`, `0²` or `√0` again, or if `norm_sq` stops
    /// summing exact zeros exactly, either of which kills the
    /// `hi == 0.0` arm the split selection is decided on. `muv` keeps a
    /// ceiling because its zero is not structural — `S_uv = ΔΔP`
    /// vanishes here only because these coordinates' differences are
    /// exact in f64, and a net whose arithmetic rounds is entitled to
    /// dust there.
    ///
    /// **The second half pins the ARM, not the answer.** Infinite steps
    /// with no cap are produced by the `muu = mvv = muv = 0` arm and by
    /// nothing else: any dust at all makes `q` positive and `h_v`
    /// finite. So a regression that revives the dust cannot pass this
    /// row by landing on numerically similar steps.
    #[test]
    fn planar_bilinear_bounds_collapse() {
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
        ];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(
            b.muu == 0.0 && b.mvv == 0.0 && b.muv < 1e-12,
            "a degree-1 direction's certified sup is not the exact zero: (uu, uv, vv) \
             ({:.17e}, {:.17e}, {:.17e}) against (exactly 0, < 1e-12, exactly 0)",
            b.muu,
            b.muv,
            b.mvv
        );
        // The affine arm, pinned by the only observable it alone
        // produces: unconstrained steps with the aspect cap inactive.
        let st = b.split_steps(1e-3);
        assert!(
            st.hu == f64::INFINITY && st.hv == f64::INFINITY && !st.cap,
            "the affine degenerate arm was not taken: (hu, hv, cap) \
             ({:.17e}, {:.17e}, {}) against (inf, inf, false)",
            st.hu,
            st.hv,
            st.cap
        );
    }

    /// A TWISTED bilinear quad has S_uv = ΔΔP exactly; the hull pins
    /// that constant to outward rounding (degree-1 directions
    /// contribute exact zeros to uu/vv).
    #[test]
    fn twisted_bilinear_mixed_term_is_tight() {
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let control = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.0, 1.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.5),
        ];
        let s = NurbsSurface::new(kv.clone(), kv, control, vec![1.0; 4]).unwrap();
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(
            b.muu == 0.0 && b.mvv == 0.0,
            "a bilinear quad's certified (uu, vv) is not the exact zero of its two \
             degree-1 directions: ({:.17e}, {:.17e})",
            b.muu,
            b.mvv
        );
        // S_uv = P11 − P10 − P01 + P00 = (0, 0, 0.5); the hull only
        // ever widens.
        assert!(
            b.muv >= 0.5 && b.muv < 0.5 + 1e-12,
            "certified uv {:.17e} against the exact |S_uv| = 5.00000000000000000e-1: it \
             must dominate it, and by less than 1e-12",
            b.muv
        );
    }

    /// **The ONE-SIDED degenerate arm, on a face where only `u` is
    /// degenerate**: degree 1 in `u`, degree 2 with a real bend in `v`,
    /// integral. `muu` is the structural zero and `mvv` is not, so
    /// [`NurbsFaceBound::split_steps`] must take its `muu == 0.0 &&
    /// mvv > 0.0` ruled-wall arm rather than the generic interior
    /// optimum.
    ///
    /// **Which arm ran is observable on the WINDOWLESS variant only,
    /// and that is why the variant is here.** With a 3-D aspect window
    /// the ruled-wall arm answers `t = ρ·ASPECT_CAP` with the cap
    /// active — and so does the generic arm at a dusty `muu`, because
    /// `t* = √(mvv/muu)` is then enormous and clamps to the same
    /// ceiling. The two arms are numerically indistinguishable there,
    /// so the exact-zero comparison is the only pin the windowed case
    /// admits — which is what
    /// [`ruled_wall_degenerate_arm_is_exact`] measures, on hand-built
    /// bounds, one level down from the assembly. Strip the `u` direction's 3-D extent (`mu1 = 0`, no
    /// window) and they separate sharply: the ruled-wall arm's
    /// windowless fallback is the balanced point `t = 1` — equal steps
    /// — while a dusty `muu` would take the generic arm to
    /// `t* = √(2.8/dust)`, steps apart by eighty decades.
    ///
    /// Neither half samples the surface, so the ~1e-16 `duu` noise
    /// `NurbsSurface::ders` returns on an integral degree-1 direction
    /// never meets the zero bound. A `Domination` cannot state this
    /// claim at all: its allowance is RELATIVE
    /// ([`SAMPLER_ULPS`]), hence zero against a zero certified
    /// side, so a sampled 1e-16 under a certified `0.0` reds. The claim
    /// here is about the arm the selection took, not about a sample.
    #[test]
    fn one_sided_degenerate_arm_on_a_degree_one_direction() {
        let bent = |extent_in_u: f64| {
            let kv_u = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
            let kv_v = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
            let mut control = Vec::new();
            for i in 0..2 {
                for j in 0..3 {
                    let y = f64::from(j) * 0.5;
                    let z = if j == 1 { 0.7 } else { 0.0 };
                    control.push(Point3::new(f64::from(i) * extent_in_u, y, z));
                }
            }
            NurbsSurface::new(kv_u, kv_v, control, vec![1.0; 6]).unwrap()
        };
        for (name, extent) in [("with a 3-D window", 1.0), ("windowless", 0.0)] {
            let s = bent(extent);
            let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
            assert!(
                b.muu == 0.0 && b.muv == 0.0 && b.mvv > 1.0,
                "{name}: the degree-1 u direction is not the exact zero over a bending v: \
                 (uu, uv, vv) ({:.17e}, {:.17e}, {:.17e})",
                b.muu,
                b.muv,
                b.mvv
            );
            let st = b.split_steps(1e-3);
            assert!(
                st.hu.is_finite() && st.hv > 0.0,
                "{name}: v constrains the mesh, so both steps are real: \
                 ({:.17e}, {:.17e})",
                st.hu,
                st.hv
            );
            if extent == 0.0 {
                // The arm: `t = 1` exactly. A dusty `muu` would reach
                // the generic interior optimum instead and drive the
                // steps eighty decades apart.
                assert!(
                    st.hu == st.hv && !st.cap,
                    "{name}: the ruled-wall arm's windowless fallback was not taken: \
                     (hu, hv, cap) ({:.17e}, {:.17e}, {})",
                    st.hu,
                    st.hv,
                    st.cap
                );
            } else {
                // The cap is what binds a ruled wall that HAS a window.
                let rho = b.mv1 / b.mu1;
                assert!(
                    st.cap && st.hu / st.hv == rho * ASPECT_CAP,
                    "{name}: the ruled wall did not saturate the aspect cap: \
                     hu/hv {:.17e} against ρ·cap {:.17e}, cap {}",
                    st.hu / st.hv,
                    rho * ASPECT_CAP,
                    st.cap
                );
            }
        }
    }

    /// **A RATIONAL degree-1 direction is degenerate only where it is
    /// proved so.** The arm predicate IS the observable: `split_steps`'
    /// v-degenerate arms are guarded by `mvv == 0.0`.
    ///
    /// - A degree-1 `v` whose weights VARY along `v` is a
    ///   Möbius-reparameterized ruling and genuinely curves in
    ///   parameter: its `mvv` must be real (> 0), and its `v` steps
    ///   real, or the split selection would treat it as unconstrained.
    /// - The quarter cylinder's weights are constant along `v`, so it
    ///   is affine in `v` in ℝ, and the quotient statement proves it:
    ///   every coefficient of the `S_vv` numerator is an exact zero, so
    ///   `mvv` is the certified `0.0` and the degenerate arm is entered
    ///   on a direction it proved, not one it assumed.
    #[test]
    fn rational_degree_one_direction_is_not_taken_as_degenerate() {
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let arc = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let mut control = Vec::new();
        for (x, y) in arc {
            for z in [0.0, 1.0] {
                control.push(Point3::new(x, y, z));
            }
        }
        // `w_ij = a_i·b_j` with `b = (1, 2.5)`: `z` is the Möbius map
        // `2.5v / (1 + 1.5v)` of `v`, so `S_vv ≠ 0` while the image stays
        // the cylinder.
        let weights = vec![1.0, 2.5, w, 2.5 * w, 1.0, 2.5];
        let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
        let (_, _, wvv) = sample_worst(&s, 64);
        assert!(
            wvv > 1.0,
            "the ruling must genuinely curve in v (fixture check)"
        );
        let b = nurbs_face_bound(&s, FaceKey::default()).unwrap();
        assert!(
            b.mvv >= wvv,
            "the Möbius ruling's v sup {:.17e} is under its sampled {wvv:.17e}",
            b.mvv
        );
        let st = b.split_steps(1e-3);
        assert!(
            st.hv.is_finite(),
            "a rational ruling's v step must stay real: hv {:.17e}",
            st.hv
        );

        let cylinder = nurbs_face_bound(&quarter_cylinder(), FaceKey::default()).unwrap();
        assert!(
            cylinder.mvv == 0.0,
            "the quarter cylinder is affine in v and the quotient's S_vv coefficients are \
             exact zeros, so mvv is the proved 0.0, not {:.17e}",
            cylinder.mvv
        );
    }

    /// REVIEW Z2: degree-0 direction refuses typed (if constructible).
    #[test]
    fn probe_degree_zero_refuses_typed() {
        let Ok(kv_u) = KnotVector::clamped(vec![0.0, 1.0], 0) else {
            // Degree 0 cannot even be described — the gate is upstream.
            return;
        };
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
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
            let d = Domination::second_partials((wuu, wuv, wvv), &b);
            assert!(d.holds(), "deg {p_deg} mult {mult}: {d}");
        }
    }

    /// CONSCIOUS FLIP (M8-5): `rational_face_refuses_typed` re-derived
    /// as the positive row. The refusal's premise — "a rational second
    /// derivative is not a control-hull convexity fact" — was answered
    /// by the quotient statement over the HOMOGENEOUS nets (module
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
            let d = Domination::second_partials((wuu, wuv, wvv), &b);
            assert!(d.holds(), "{name}: {d}");
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
                let d = Domination::second_partials((wuu, wuv, wvv), &b);
                assert!(
                    d.holds(),
                    "{name} cell {k} u{:?} v{:?}, against its own bound: {d}",
                    c.u,
                    c.v
                );
            }
        }
    }

    /// REVIEW PROBE (r1, adoptable): the first-derivative sups the
    /// aspect cap reads (`mu1`/`mv1`, TESS-SPLIT) get the SAME
    /// falsification rows their Hessian siblings have — sampled
    /// dominance per cell and whole-patch, and cell ≤ patch
    /// componentwise. Without this, a subtle first-derivative window
    /// or `s1u` recurrence bug would misplace the aspect window with nothing
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
                let refines = Domination::new(
                    "cell",
                    "patch",
                    &[
                        ("u1", c.bound.mu1, whole.mu1),
                        ("v1", c.bound.mv1, whole.mv1),
                    ],
                );
                assert!(
                    refines.holds(),
                    "{name} cell {k}: first-derivative sup exceeds the patch's: {refines}"
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
                let d = Domination::sampled_under_certified(&[
                    ("u1", wu, c.bound.mu1),
                    ("v1", wv, c.bound.mv1),
                ]);
                assert!(
                    d.holds(),
                    "{name} cell {k} u{:?} v{:?}, first-derivative speeds: {d}",
                    c.u,
                    c.v
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
                let refines = Domination::new(
                    "cell",
                    "patch",
                    &[
                        ("uu", c.bound.muu, whole.muu),
                        ("uv", c.bound.muv, whole.muv),
                        ("vv", c.bound.mvv, whole.mvv),
                    ],
                );
                assert!(
                    refines.holds(),
                    "{name}: cell u{:?} v{:?} bound exceeds the patch's: {refines}",
                    c.u,
                    c.v
                );
            }
        }
    }

    /// **A triangle across a C⁰ crease has no certificate.** A tent
    /// face, degree 2 in `u` with a double knot at the ridge `u = ½`,
    /// each half a straight strip: every second partial is zero on both
    /// cells, so a box crossing the ridge would take the componentwise
    /// max `0` while the triangle cuts under the ridge by `0.2`. The
    /// row measures each triangle's true deviation (the surface against
    /// the linear interpolant of its corners, densely sampled) against
    /// its certificate; the straddler must answer `+∞`, and the
    /// one-sided triangle its cell's own `0`.
    #[test]
    fn a_triangle_across_a_crease_has_no_certificate() {
        let ridge = |u: f64| if u <= 0.5 { 2.0 * u } else { 2.0 - 2.0 * u };
        let half = |lo: f64, hi: f64| {
            let kv_u = KnotVector::clamped(vec![lo, lo, lo, hi, hi, hi], 2).unwrap();
            let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
            let mut control = Vec::new();
            for i in 0..3 {
                let u = lo + (hi - lo) * f64::from(i) / 2.0;
                for j in 0..2 {
                    control.push(Point3::new(u, f64::from(j), ridge(u)));
                }
            }
            NurbsSurface::new(kv_u, kv_v, control, vec![1.0; 6]).unwrap()
        };
        let face = FaceKey::default();
        let (left, right) = (half(0.0, 0.5), half(0.5, 1.0));
        let mut cells = nurbs_cell_bounds(&left, face).unwrap();
        cells.extend(nurbs_cell_bounds(&right, face).unwrap());
        assert!(
            cells
                .iter()
                .all(|c| c.bound.muu == 0.0 && c.bound.mvv == 0.0)
        );
        // The whole tent's own knots name the ridge as its one crease.
        let tent = {
            let kv_u =
                KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
            let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
            let mut control = Vec::new();
            for i in 0..5 {
                let u = f64::from(i) / 4.0;
                for j in 0..2 {
                    control.push(Point3::new(u, f64::from(j), ridge(u)));
                }
            }
            NurbsSurface::new(kv_u, kv_v, control, vec![1.0; 10]).unwrap()
        };
        assert_eq!(creases(tent.knot_values_u()), vec![0.5]);
        assert!(creases(tent.knot_values_v()).is_empty());
        let grid = NurbsCellGrid::from_cells(&cells, (vec![0.5], Vec::new()));
        // The tent itself, read on either side of the ridge.
        let surface = |u: f64, v: f64| Point3::new(u, v, ridge(u));
        let deviation = |uv: [[f64; 2]; 3]| {
            let p = uv.map(|[u, v]| surface(u, v));
            let mut worst = 0.0f64;
            for i in 0..=40 {
                for j in 0..=(40 - i) {
                    let (a, b) = (f64::from(i) / 40.0, f64::from(j) / 40.0);
                    let c = 1.0 - a - b;
                    let u = a * uv[0][0] + b * uv[1][0] + c * uv[2][0];
                    let v = a * uv[0][1] + b * uv[1][1] + c * uv[2][1];
                    let lin = Point3::new(
                        a * p[0].x + b * p[1].x + c * p[2].x,
                        a * p[0].y + b * p[1].y + c * p[2].y,
                        a * p[0].z + b * p[1].z + c * p[2].z,
                    );
                    worst = worst.max((surface(u, v) - lin).norm());
                }
            }
            worst
        };
        let across = [[0.4, 0.2], [0.6, 0.2], [0.5, 0.8]];
        let beside = [[0.1, 0.2], [0.4, 0.2], [0.25, 0.8]];
        for (name, uv) in [("across the ridge", across), ("beside it", beside)] {
            let (cert, dev) = (grid.cert(uv), deviation(uv));
            assert!(cert >= dev, "{name}: certified {cert:e}, deviates {dev:e}");
        }
        assert!(deviation(across) > 0.19);
        assert!(grid.cert(across) == f64::INFINITY);
        assert!(grid.cert(beside) == 0.0);
    }

    /// **The lofted circle's straddler, read for its true deviation.**
    /// The wall of `sweep::test_support::lofted_cylinder` carries the
    /// crease `u = ½` (two quarter arcs at a double knot), and a
    /// triangle across it has no certificate. Its true deviation (the
    /// wall against the linear interpolant of its corners, densely
    /// sampled) is read beside two certificates: the one `cert` answers
    /// (`+∞`), and the componentwise max of the cells on both sides,
    /// which the crease voids in general (the tent row above).
    ///
    /// On THIS wall the max still dominates, because the sub-arcs are
    /// equal: each is the same rational quadratic on an equal knot
    /// span, so the one-sided derivatives at the joint agree and the
    /// wall is C¹ there in fact. The trimmed lane cuts along it anyway;
    /// here that costs only the cut.
    ///
    /// The second assertion is a tripwire for unequal sub-arcs, and only
    /// a narrow straddler is a sharp one. #5499's review built a half
    /// cylinder from unequal sub-arcs on equal knot spans: a straddler
    /// over `u ∈ [0.49, 0.51]`, the span of the first one here,
    /// deviated past the cells' max already at an 80°/100° split, while
    /// the `[0.45, 0.55]` straddler stayed under it down to a 45° split
    /// and the `[0.3, 0.7]` one at every split tried. So the row reads
    /// a split as near equal as 80°/100°, through its first straddler;
    /// it does not claim every inequality.
    #[test]
    fn a_lofted_circle_straddler_reads_its_true_deviation() {
        let body = sweep::test_support::lofted_cylinder(1.0, Tol::witness());
        let mut walls = 0;
        for (fk, face) in body.faces() {
            let Some(geom::Surface::Nurbs(wall)) = body.get_surface(face.surface) else {
                continue;
            };
            walls += 1;
            let grid = nurbs_cell_grid(wall, fk).expect("the creased wall is covered");
            assert_eq!(grid.creases(), (&[0.5][..], &[][..]));
            let uncreased = NurbsCellGrid::from_cells(
                &nurbs_cell_bounds(wall, fk).unwrap(),
                (Vec::new(), Vec::new()),
            );
            let deviation = |uv: [[f64; 2]; 3]| {
                let p = uv.map(|[u, v]| wall.eval(u, v));
                let mut worst = 0.0f64;
                for i in 0..=40 {
                    for j in 0..=(40 - i) {
                        let (a, b) = (f64::from(i) / 40.0, f64::from(j) / 40.0);
                        let c = 1.0 - a - b;
                        let u = a * uv[0][0] + b * uv[1][0] + c * uv[2][0];
                        let v = a * uv[0][1] + b * uv[1][1] + c * uv[2][1];
                        let lin = Point3::new(
                            a * p[0].x + b * p[1].x + c * p[2].x,
                            a * p[0].y + b * p[1].y + c * p[2].y,
                            a * p[0].z + b * p[1].z + c * p[2].z,
                        );
                        worst = worst.max((wall.eval(u, v) - lin).norm());
                    }
                }
                worst
            };
            for across in [
                [[0.49, 0.2], [0.51, 0.2], [0.5, 0.8]],
                [[0.45, 0.2], [0.55, 0.2], [0.5, 0.8]],
                [[0.3, 0.1], [0.7, 0.1], [0.5, 0.9]],
            ] {
                let dev = deviation(across);
                assert!(dev > 0.0);
                assert!(grid.cert(across) == f64::INFINITY, "{across:?}");
                let max = uncreased.cert(across);
                assert!(
                    max >= dev,
                    "{across:?}: the cells' max {max:e}, deviates {dev:e}"
                );
            }
        }
        assert_eq!(walls, 2);
    }

    /// Two degree-2 halves meeting at `u = ½`, `v` linear: the left a
    /// flat strip, the right bowed (`z = (u − ½)(1 − u)·4`), so their
    /// cells' bounds differ. With `transpose` the same patches with `u`
    /// and `v` swapped. Answers the grid built from both halves' cells
    /// with the crease at `½` in the creased direction.
    fn two_sided_grid(transpose: bool) -> NurbsCellGrid {
        let half = |lo: f64, hi: f64, bowed: bool| {
            let curved = KnotVector::clamped(vec![lo, lo, lo, hi, hi, hi], 2).unwrap();
            let straight = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
            // Control row-major in the surface's own `u`: along the
            // curved direction first, or along the straight one when
            // transposed.
            let at = |i: usize, j: usize| {
                let u = lo + (hi - lo) * (i as f64) / 2.0;
                // The bowed half's middle control point lifts the
                // parabola through its ends.
                let z = if bowed && i == 1 { 0.25 } else { 0.0 };
                if transpose {
                    Point3::new(j as f64, u, z)
                } else {
                    Point3::new(u, j as f64, z)
                }
            };
            if transpose {
                let control = (0..2).flat_map(|j| (0..3).map(move |i| at(i, j))).collect();
                NurbsSurface::new(straight, curved, control, vec![1.0; 6]).unwrap()
            } else {
                let control = (0..3).flat_map(|i| (0..2).map(move |j| at(i, j))).collect();
                NurbsSurface::new(curved, straight, control, vec![1.0; 6]).unwrap()
            }
        };
        let face = FaceKey::default();
        let mut cells = nurbs_cell_bounds(&half(0.0, 0.5, false), face).unwrap();
        cells.extend(nurbs_cell_bounds(&half(0.5, 1.0, true), face).unwrap());
        let creases = if transpose {
            (Vec::new(), vec![0.5])
        } else {
            (vec![0.5], Vec::new())
        };
        NurbsCellGrid::from_cells(&cells, creases)
    }

    /// **A crease in `v`, crossed, has no certificate** — the `v` half of
    /// `cert`'s crossing check, which the tent row (a `u` crease) cannot
    /// see. Red with the `v` check dropped.
    #[test]
    fn a_triangle_across_a_v_crease_has_no_certificate() {
        let grid = two_sided_grid(true);
        let swap = |uv: [[f64; 2]; 3]| uv.map(|[a, b]| [b, a]);
        assert!(grid.cert(swap([[0.4, 0.2], [0.6, 0.2], [0.5, 0.8]])) == f64::INFINITY);
        assert!(grid.cert(swap([[0.1, 0.2], [0.4, 0.2], [0.25, 0.8]])) == 0.0);
    }

    /// **A triangle touching a crease is certified by its own side's
    /// cells.** The crossing check is strict: a box whose edge lies ON
    /// the crease is no straddler, and the half-open lookup reads the
    /// side it lies on — the flat left's `0`, the bowed right's
    /// positive bound — in either direction. The trimmed lane's crease
    /// lines put triangle edges exactly there, so a non-strict check
    /// (`<=`) would refuse every one of them: red then.
    #[test]
    fn a_triangle_touching_a_crease_reads_its_own_side() {
        for transpose in [false, true] {
            let grid = two_sided_grid(transpose);
            let orient = |uv: [[f64; 2]; 3]| {
                if transpose {
                    uv.map(|[a, b]| [b, a])
                } else {
                    uv
                }
            };
            let left = grid.cert(orient([[0.3, 0.2], [0.5, 0.2], [0.5, 0.8]]));
            let right = grid.cert(orient([[0.5, 0.2], [0.7, 0.2], [0.5, 0.8]]));
            assert!(left == 0.0, "transpose {transpose}: left {left:e}");
            assert!(
                right > 0.0 && right.is_finite(),
                "transpose {transpose}: right {right:e}"
            );
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
        let grid = NurbsCellGrid::from_cells(&cells, (Vec::new(), Vec::new()));
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

    /// **The residual line lands the strip sliver at δ over every split**
    /// ([`MAX_RESIDUAL_SQ`]). A band sized to `b` with a share `s` of
    /// the budget in `u` (`muu·h_u² = s·b`, `mvv·h_v² = (1 − s)·b`)
    /// admits a sliver spanning `a` columns and one row. Just under
    /// `a = √MAX_RESIDUAL_SQ` its certificate is within δ at every share;
    /// at the line, at the all-`u` share (the cap-bound ruled wall), it
    /// is δ; just past it, it exceeds δ.
    #[test]
    fn the_residual_line_lands_the_strip_sliver_at_delta_over_every_split() {
        let delta = 1e-3;
        let budget = lattice_budget(delta);
        let line = MAX_RESIDUAL_SQ.sqrt();
        let strip = |s: f64, a: f64| {
            let b = NurbsFaceBound {
                muu: 2.0,
                muv: 0.0,
                mvv: 3.0,
                mu1: 1.0,
                mv1: 1.0,
            };
            let hu = (s * budget / b.muu).sqrt();
            let hv = ((1.0 - s) * budget / b.mvv).sqrt();
            b.cert([[0.0, 0.0], [a * hu, 0.0], [0.5 * a * hu, hv]])
        };
        for i in 0..=100 {
            let s = f64::from(i) / 100.0;
            let c = strip(s, line * (1.0 - 1e-9));
            assert!(c <= delta, "share {s}: {c:e} > δ {delta:e}");
        }
        let at = strip(1.0, line);
        assert!(
            ((at - delta) / delta).abs() < 1e-12,
            "all-u share at the line: {at:e}, not δ {delta:e}"
        );
        let past = strip(1.0, line * (1.0 + 1e-9));
        assert!(
            past > delta,
            "all-u share past the line: {past:e} <= δ {delta:e}"
        );
    }

    /// **A real off-row sliver stays inside the box bound** ([`MAX_RESIDUAL_SQ`]).
    /// A strip lattice at residual `1/a`, with `a = √MAX_RESIDUAL_SQ`, goes
    /// into the scaled CDT (rows one apart, columns `1/a` apart). Off-row
    /// points are added between two rows: one point, or two adjacent chord
    /// points joined by a constraint, as a slanted trim edge leaves them.
    /// Over every placement, the worst triangle's certificate stays within
    /// `¼(a²·s_u + (1 − s_u))·b` and within δ. This holds for the
    /// cap-bound split (`s_u = 1`, `mvv = 0`) and for a split that keeps a
    /// `v` share.
    #[test]
    fn a_strip_lattice_off_row_sliver_stays_inside_the_box_bound() {
        use spade::{ConstrainedDelaunayTriangulation, Point2, Triangulation};
        let delta = 1e-3;
        let budget = lattice_budget(delta);
        let a = MAX_RESIDUAL_SQ.sqrt();
        let mut ratio = 0.0f64;
        for (s_u, mvv) in [(1.0, 0.0), (0.9, 3.0)] {
            let b = NurbsFaceBound {
                muu: 2.0,
                muv: 0.0,
                mvv,
                mu1: 1.0,
                mv1: 1.0,
            };
            let hu = (s_u * budget / b.muu).sqrt();
            let hv = if mvv > 0.0 {
                ((1.0 - s_u) * budget / mvv).sqrt()
            } else {
                1.0
            };
            let bound = 0.25 * (a.powi(2) * s_u + (1.0 - s_u)) * budget;
            // CDT `(X, Y)` is UV `(X·a·h_u, Y·h_v)`.
            let uv = |p: Point2<f64>| [p.x * a * hu, p.y * hv];
            let mut worst = 0.0f64;
            for i in 0..4 {
                for j in 1..8 {
                    let p0 = (f64::from(i) / (4.0 * a), 1.0 + f64::from(j) / 8.0);
                    let mut placements = vec![vec![p0]];
                    for dx in [0.3, 0.6, 0.9, 0.99] {
                        for dy in [-0.1, 0.0, 0.1] {
                            placements.push(vec![p0, (p0.0 + dx, p0.1 + dy)]);
                        }
                    }
                    for off in placements {
                        let mut cdt = ConstrainedDelaunayTriangulation::<Point2<f64>>::new();
                        for c in -12..=15 {
                            for r in 0..=3 {
                                cdt.insert(Point2::new(f64::from(c) / a, f64::from(r)))
                                    .expect("lattice point");
                            }
                        }
                        let hs: Vec<_> = off
                            .iter()
                            .map(|&(x, y)| cdt.insert(Point2::new(x, y)).expect("off-row point"))
                            .collect();
                        if let [h0, h1] = hs[..] {
                            cdt.add_constraint(h0, h1);
                        }
                        for f in cdt.inner_faces() {
                            let [p, q, r] = f.vertices().map(|v| v.position());
                            worst = worst.max(b.cert([uv(p), uv(q), uv(r)]));
                        }
                    }
                }
            }
            assert!(
                worst <= bound * (1.0 + 1e-12),
                "s_u {s_u}: {worst:e} > box {bound:e}"
            );
            assert!(worst <= delta, "s_u {s_u}: {worst:e} > δ {delta:e}");
            ratio = ratio.max(worst / delta);
        }
        eprintln!("worst sliver / δ = {ratio:.4}");
    }

    /// **Every band keeps its own columns, and a narrow-column band takes
    /// rows** ([`NurbsCellGrid::band_schedule`]). Three bands whose
    /// realized aspects (9.09, ≈ 1.04, ≈ 0.113) put the metric at 1,
    /// bands 0 and 2 far from it on either side. Each takes its own split
    /// steps, none the patch's 45 columns. Band 2, below `1/√8` over the
    /// metric, has its rows raised to the line ([`MAX_RESIDUAL_SQ`]).
    #[test]
    fn every_band_keeps_its_columns_and_a_narrow_column_band_takes_rows() {
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
                v: (0.02, 0.5),
                bound: NurbsFaceBound {
                    muu: 0.1,
                    muv: 0.0,
                    mvv: 0.1,
                    mu1: 1.0,
                    mv1: 1.0,
                },
            },
            CellBound {
                u: (0.0, 1.0),
                v: (0.5, 1.0),
                bound: NurbsFaceBound {
                    muu: 2.77,
                    muv: 0.0,
                    mvv: 0.0335,
                    mu1: 1.0,
                    mv1: 1.0,
                },
            },
        ];
        let grid = NurbsCellGrid::from_cells(&cells, (Vec::new(), Vec::new()));
        // With every muv = 0 and unit speeds, the split selection's
        // interior optimum t* = √(mvv/muu) sits inside the aspect
        // window everywhere here (t* = 5.0, 1.0, ~0.11 and the patch's
        // ~1.18 vs [1/16, 16]),
        // and the chosen steps coincide with the retired grouping's —
        // the fixture pins the schedule, not the selection.
        let budget = 2e-3;
        let schedule = grid
            .band_schedule((0.0, 1.0), (0.0, 1.0), budget)
            .expect("schedules");
        assert_eq!(schedule.metric, 1.0, "{schedule:?}");
        let bands = schedule.bands;
        assert_eq!(bands.len(), 3);
        // Band 0: h_u ≈ 0.0950 → nuc 11; h_v ≈ 0.0190 over 0.02 → nvc 2
        // (realized 9.09). Band 2: h_u ≈ 0.0190 → nuc 53; h_v ≈ 0.173
        // over 0.5 → nvc 3 (realized ≈ 0.113).
        assert_eq!((bands[0].nuc, bands[0].nvc), (11, 2), "{bands:?}");
        // Band 2 then takes rows: ceil(3 / (0.113·√8)) = 10.
        assert_eq!((bands[2].nuc, bands[2].nvc), (53, 10), "{bands:?}");
        assert_ne!(bands[1].nuc, 45, "band 1 keeps its own columns: {bands:?}");
        // The A cap clamped none (the interior optima above sit inside
        // the aspect window).
        assert!(
            bands.iter().all(|b| !b.cap),
            "no cap activity in this fixture: {bands:?}"
        );
    }

    /// **The lattice metric** is the power of two nearest the geometric
    /// mean of the extreme band aspects: a lofted wall's strip (`≈ 0.03`)
    /// takes 2⁻⁵, where it is near square, and a face with no finite
    /// positive aspect takes 1.
    #[test]
    fn the_lattice_metric_is_the_rounded_mean_of_the_extremes() {
        assert_eq!(lattice_metric([0.03, 0.033].into_iter()), 1.0 / 32.0);
        assert_eq!(lattice_metric([9.09, 1.04, 0.113].into_iter()), 1.0);
        assert_eq!(lattice_metric([f64::NAN, 0.0].into_iter()), 1.0);
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
    /// generic arm evaluated at muu = a dust value must agree to a
    /// stated 1e-6 relative bound — the arm is the limit's value,
    /// reached without the division by zero. That agreement is also why
    /// this row cannot tell which arm ran:
    /// [`one_sided_degenerate_arm_on_a_degree_one_direction`] is the
    /// row that can, and it does it on a described face.
    #[test]
    fn ruled_wall_degenerate_arm_is_exact() {
        let budget = 1e-3;
        let (muv, mvv) = (2.4, 51.3);
        let (mu1, mv1) = (0.9, 7.3);
        let ruled = NurbsFaceBound {
            muu: 0.0,
            muv,
            mvv,
            mu1,
            mv1,
        };
        let s = ruled.split_steps(budget);
        assert!(s.cap, "the cap is what binds a ruled wall");
        // Bitwise: t = ρ·A on the ellipse, hv = √(b/q(t)), hu = t·hv.
        let t = (mv1 / mu1) * ASPECT_CAP;
        let q = muv.mul_add(2.0 * t, mvv);
        let hv = (budget / q).sqrt();
        assert_eq!(s.hv.to_bits(), hv.to_bits(), "exact arm hv");
        assert_eq!(s.hu.to_bits(), (t * hv).to_bits(), "exact arm hu");
        // The generic arm at a subnormal muu lands on the same point
        // through the clamp (t* = √(mvv/dust) is far beyond the
        // window): agreement to 1e-6 relative is the stated bound.
        let dusty = NurbsFaceBound {
            muu: 3.8e-162,
            muv,
            mvv,
            mu1,
            mv1,
        };
        let d = dusty.split_steps(budget);
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
        let m = mirror.split_steps(budget);
        assert!(m.cap);
        let tm = (mu1 / mv1) / ASPECT_CAP;
        // q spelled exactly as the selection spells it (mul_add order),
        // so the pin is bitwise: mirror.muu = 51.3, mirror.mvv = 0.
        let qm = muv.mul_add(2.0 * tm, mvv.mul_add(tm.powi(2), 0.0));
        let hvm = (budget / qm).sqrt();
        assert_eq!(m.hv.to_bits(), hvm.to_bits(), "mirror arm hv");
        assert_eq!(m.hu.to_bits(), (tm * hvm).to_bits(), "mirror arm hu");
    }

    /// The ruled-wall recovery the unit exists for: against the
    /// retired AM-GM grouping the capped optimum spends several-fold
    /// fewer cells, and its 3-D aspect sits exactly on the cap rather
    /// than beyond it (the strip the unconstrained optimum would be).
    #[test]
    fn the_cap_binds_and_the_ruled_wall_gets_cheaper() {
        let budget = 1e-3;
        let b = NurbsFaceBound {
            muu: 0.0,
            muv: 2.4,
            mvv: 51.3,
            mu1: 1.1,
            mv1: 2.9,
        };
        let s = b.split_steps(budget);
        assert!(s.cap);
        let aspect = (s.hu * b.mu1) / (s.hv * b.mv1);
        assert!(
            (aspect - ASPECT_CAP).abs() < 1e-9 * ASPECT_CAP,
            "the chosen cell sits ON the cap: {aspect}"
        );
        // The retired grouping's point, spelled here as the
        // counterfactual it now is.
        let amgm = |group: f64| (budget / (2.0 * group)).sqrt();
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
                let under_patch =
                    Domination::new("per-cell", "whole-patch", &[("cert", got, whole.cert(uv))]);
                assert!(
                    under_patch.holds(),
                    "{name}: per-cell cert exceeds the whole-patch one: {under_patch}"
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
            let under_patch =
                Domination::new("spanning", "whole-patch", &[("cert", got, whole.cert(uv))]);
            assert!(
                under_patch.holds(),
                "{name}: spanning cert exceeds patch: {under_patch}"
            );
            for c in &cells {
                let own = c.bound.cert(uv);
                assert!(
                    got >= own - f64::EPSILON,
                    "{name}: spanning cert {got:.17e} is below the {own:.17e} of the covered \
                     cell u{:?} v{:?} — the componentwise sup is missing a cell",
                    c.u,
                    c.v
                );
            }
        }
    }

    /// The exact unit quarter cylinder: rational quadratic arc
    /// (weights `[1, √2/2, 1]`) × unit line in z.
    fn quarter_cylinder() -> NurbsSurface<f64> {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
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
        use geom_core::{Point2, Point3};
        let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
        let lp = bulge_loop(vec![v(1.0, 0.0, 0.4), v(0.0, 1.0, 0.0), v(0.0, 0.0, 0.0)]);
        let sections = vec![vec![lp.clone()], vec![lp]];
        let places: Vec<profile::SketchPlane<f64>> = [0.0, 1.0]
            .iter()
            .map(|z| profile::SketchPlane::xy_at(Point3::new(0.0, 0.0, *z)))
            .collect();
        let body = sweep::loft_body::<f64>(&sections, &places, 1, Tol::witness())
            .expect("the rational pie lofts")
            .body;
        for (_, face) in body.faces() {
            if let Some(geom::Surface::Nurbs(p)) = body.get_surface(face.surface)
                && p.constant_weight().is_none()
            {
                return (**p).clone();
            }
        }
        panic!("the pie loft minted no rational wall — the fixture stopped exercising M8-5");
    }

    /// The z1 lattice rows' closing claim: over every triangle sampled at
    /// this `delta`, the largest sampled-deviation / certificate ratio is
    /// at most one.
    ///
    /// Monotone the easy way — the ratio only shrinks as the certificate
    /// grows, so a LOOSE bound passes this by a wider margin than a tight
    /// one. **A ceiling with no floor**, which is a known and unfixed gap
    /// at every row of this shape.
    #[track_caller]
    fn assert_no_sample_exceeded_its_certificate(name: &str, delta: f64, worst_ratio: f64) {
        assert!(
            worst_ratio <= 1.0,
            "{name} delta={delta:e}: a triangle's samples exceeded its certificate — worst \
             sampled-deviation / certificate = {worst_ratio:.17e}"
        );
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
            let (u0, u1) = s.knots_u_f64_interim().domain();
            let (v0, v1) = s.knots_v_f64_interim().domain();
            for delta in deltas {
                let budget = lattice_budget(delta);
                let (hu, hv) = b.grid_steps(budget);
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
                assert_no_sample_exceeded_its_certificate(name, delta, worst_ratio);
            }
        }
    }

    /// The row the CONSCIOUS FLIP (M8-5) above keeps: an ILLEGAL
    /// rational (non-positive or non-finite weight) cannot even be
    /// described — `NurbsSurface::new` returns `Err` at construction,
    /// which is why `rational_face_bound`'s own licence check is a
    /// defensive backstop rather than a reachable lane.
    #[test]
    fn illegal_rational_weight_refuses_at_the_door() {
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
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

    /// **A uniform weight is the integral arm's** (the
    /// `planar_bilinear_bounds_collapse` class): a bilinear patch with
    /// UNIFORM weight (`1/2`, `3`, `0.1`) is the polynomial bilinear on
    /// its net (`NurbsSurface::constant_weight`), so it takes
    /// the integral arm and its bound is the unit-weight patch's to the
    /// bit. On the flat net with zero twist every second-partial sup is
    /// the exact zero; on the twisted net (`z` lifted at one corner) the
    /// pair agreeing is what goes red where a uniform weight is read as
    /// rational, whose arm bounds the same patch through the quotient.
    #[test]
    fn a_uniform_weight_bilinear_bounds_as_its_unit_twin() {
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let net = |z: f64| {
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(0.0, 1.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, z),
            ]
        };
        let bound = |z: f64, w: f64| {
            let s = NurbsSurface::new(kv.clone(), kv.clone(), net(z), vec![w; 4]).unwrap();
            nurbs_face_bound(&s, FaceKey::default()).unwrap()
        };
        let bits = |b: &NurbsFaceBound| [b.muu, b.muv, b.mvv, b.mu1, b.mv1].map(f64::to_bits);
        for z in [0.0, 0.7] {
            let unit = bound(z, 1.0);
            for c in [0.5, 3.0, 0.1] {
                let b = bound(z, c);
                assert_eq!(
                    bits(&b),
                    bits(&unit),
                    "z = {z}: weight {c} ({b:?}) is not the unit-weight patch's bound ({unit:?})"
                );
            }
        }
        let b = bound(0.0, 0.5);
        assert!(
            b.muu == 0.0 && b.muv == 0.0 && b.mvv == 0.0,
            "a flat bilinear's second partials are the exact zero: {b:?}"
        );
        let (hu, hv) = b.grid_steps(1e-3);
        assert!(hu > 1e3 && hv > 1e3, "flat stays effectively unconstrained");
    }

    /// A described bilinear rational patch, from the exact bits of its
    /// description. Hex bit patterns rather than decimal literals: the
    /// referee that measured these patches reads the same bits, and a
    /// decimal spelling would make the two descriptions agree only as
    /// far as two parsers do.
    fn bilinear_rational(weights: [u64; 4], control: [u64; 12]) -> NurbsSurface<f64> {
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let p = |i: usize| {
            Point3::new(
                f64::from_bits(control[3 * i]),
                f64::from_bits(control[3 * i + 1]),
                f64::from_bits(control[3 * i + 2]),
            )
        };
        NurbsSurface::new(
            kv.clone(),
            kv,
            (0..4).map(p).collect(),
            weights.map(f64::from_bits).to_vec(),
        )
        .unwrap()
    }

    /// **The certificate's own claim, refereed exactly.** `muu`, `muv`
    /// and `mvv` are sup bounds on the DESCRIBED patch's second
    /// partials, so the claim is falsified by one real value above one
    /// of them — and on these patches the real values are known, not
    /// sampled.
    ///
    /// Each literal is the largest true `‖S_kl‖` over the four domain
    /// corners, computed in exact rational arithmetic (every `f64` of a
    /// NURBS description is a rational, so the whole quotient-rule jet
    /// is exact) and then **rounded DOWN to an `f64`** — so it is a real
    /// number the patch genuinely attains, or below one. Corners are
    /// where a bilinear patch's second partials take their extremes and
    /// where the certificate is tight; the referee chooses among them,
    /// not this row.
    ///
    /// Two consequences of the literals being TRUTHS and not samples.
    /// The comparison is [`Domination::new`] — the bare `<=`, allowance
    /// zero — because [`SAMPLER_ULPS`] pays for a sampler's rounding and
    /// there is no sampler here. And **this is the row that goes red on
    /// the defect**: the escapes it is for are 6–8 ulps, which the
    /// sampler's allowance absorbs, so a sampled comparison cannot see
    /// them at all. Against `patch_bound.rs` reverted to the `f64`
    /// refinement, A and B escape in `uu` by 1.836e-15 and 1.601e-15
    /// relative, and the four enumeration fixtures do NOT — on A's own
    /// net, at every weight decade. That is worth stating rather than
    /// hiding: the escape is a knife-edge in the last bits of a
    /// particular weight pattern, which is exactly why a seed-varying
    /// sweep needed thousands of runs to find two. So these two draws are
    /// kept as fixtures because they cannot be re-derived from a decade,
    /// and the decades are kept because coverage of the claim is what
    /// stops the next one needing thousands of runs again.
    ///
    /// **Regenerate the table** — never hand-edit it — with
    ///
    /// ```text
    /// python3 crates/mesh/tests/nurbs_exact_referee.py --rust
    /// ```
    ///
    /// whose fixtures are these, by the same bits. The first two are the
    /// surfaces two hosted runs drew and the diagnostic measured; the
    /// rest are one patch per weight decade of the bilinear stratum
    /// enumeration below, on that row's tightest net, so the
    /// deterministic coverage there has an exact referent too.
    ///
    /// A structurally-zero component's literal is `0.0` (fixtures C and
    /// D are polynomial in disguise — equal weights cancel in ℝ — so
    /// their true `S_uu` and `S_vv` vanish). Those two comparisons check
    /// only that the certified figure is a non-negative real, which is
    /// worth little; what carries the claim for that pair is that they
    /// certify IDENTICALLY, asserted below, since a uniform weight scale
    /// is the same surface. The zero's CONTAINMENT is a different claim
    /// and lives in `geom-brep`'s
    /// `cert10r1_the_rational_arm_contains_the_structural_zero_of_s_vv`.
    #[test]
    fn the_described_bilinear_rationals_true_second_partials_are_under_the_certified_sups() {
        // Fixture A, weights ≈ 0.0103–0.0135 (hosted run 33904520538).
        const NET_A: [u64; 12] = [
            0xbfd4_5c5e_a128_e388,
            0xbfe3_53dc_5381_e28c,
            0x3fee_e4c7_4f58_da14,
            0x3ffa_dea5_3eea_524a,
            0xbffe_1bfe_e0da_6176,
            0xbfb5_7ac6_093c_8d60,
            0x3ff2_4217_85e0_9da0,
            0xbff7_ec6a_dc24_3a5c,
            0xbffd_007c_723f_c4a6,
            0xbfd9_ac7d_34a0_77d8,
            0x3ff2_2092_5d90_b010,
            0xbfc5_0c20_054b_4c20,
        ];
        // Fixture B, weights ≈ 3.3–5.2 (hosted run 35050942262).
        const NET_B: [u64; 12] = [
            0xbff5_4a0c_eafe_35ba,
            0x3f95_e6ac_362b_df80,
            0xbfd0_260d_ed55_9cd0,
            0x3fea_bff6_7292_74d0,
            0xbfe5_f005_b8eb_add4,
            0x3fe4_64f4_1381_1098,
            0xbff5_221f_c005_d982,
            0xbff8_6db2_84fd_9286,
            0xbfe9_576c_7148_3e30,
            0x3fe4_da54_6f12_0828,
            0x3ff4_7c37_162b_9746,
            0xbfcb_0fc4_c9a4_7fd0,
        ];
        // The enumeration's shared net is A's own: that is the geometry
        // the certificate was measured to escape on, so varying only the
        // weight decade over it makes the decades comparable AND lets
        // them red on the defect rather than only the two hosted draws.
        const NET_ENUM: [u64; 12] = NET_A;
        const W_A: [u64; 4] = [
            0x3f85_1d67_0625_33ec,
            0x3f85_e951_aad1_06d4,
            0x3f8b_ae9f_b921_3c25,
            0x3f89_570f_cf39_bbfe,
        ];
        const W_B: [u64; 4] = [
            0x4014_b6e6_9928_91ad,
            0x400a_864c_9997_93db,
            0x4010_ca99_b259_0ee1,
            0x4010_6199_a0a8_9a72,
        ];
        const W_LOW: [u64; 4] = [0x3f84_7ae1_47ae_147b; 4];
        const W_HIGH: [u64; 4] = [0x4059_0000_0000_0000; 4];
        const W_SPREAD: [u64; 4] = [
            0x3f84_7ae1_47ae_147b,
            0x3ff0_0000_0000_0000,
            0x4059_0000_0000_0000,
            0x3ff0_0000_0000_0000,
        ];
        const W_NEAR_LOW: [u64; 4] = [
            0x3f85_182a_9930_be0e,
            0x3f85_e9e1_b089_a027,
            0x3f8b_a5e3_53f7_ced9,
            0x3f89_652b_d3c3_6113,
        ];
        const W_NEAR_HIGH: [u64; 4] = [
            0x4059_c000_0000_0000,
            0x405a_c000_0000_0000,
            0x4060_e000_0000_0000,
            0x405f_0000_0000_0000,
        ];
        // Generated by `python3 crates/mesh/tests/nurbs_exact_referee.py
        // --rust`; the names and the order are that script's.
        let truths: [(&str, [f64; 3]); 7] = [
            (
                "A",
                [
                    2.660_331_998_073_639_5,
                    7.953_091_356_095_674,
                    0.699_588_379_285_154,
                ],
            ),
            (
                "B",
                [
                    1.248_592_341_233_724_5,
                    6.659_454_728_150_954,
                    4.294_173_537_974_706,
                ],
            ),
            ("C (all 1e-2)", [0.0, 5.921_457_034_209_361, 0.0]),
            ("D (all 1e2)", [0.0, 5.921_457_034_209_361, 0.0]),
            (
                "E (1e-2, 1, 1e2, 1)",
                [
                    652_355_242.259_523_7,
                    5_594_901.142_172_855,
                    68_596.229_218_882_04,
                ],
            ),
            (
                "G (near-equal 1e2)",
                [
                    2.656_665_559_628_96,
                    7.961_192_352_458_233,
                    0.669_187_980_152_985_6,
                ],
            ),
            (
                "F (near-equal 1e-2)",
                [
                    2.656_665_559_628_96,
                    7.961_192_352_458_232,
                    0.669_187_980_152_985_9,
                ],
            ),
        ];
        let descriptions: [(&str, [u64; 12], [u64; 4]); 7] = [
            ("A", NET_A, W_A),
            ("B", NET_B, W_B),
            ("C (all 1e-2)", NET_ENUM, W_LOW),
            ("D (all 1e2)", NET_ENUM, W_HIGH),
            ("E (1e-2, 1, 1e2, 1)", NET_ENUM, W_SPREAD),
            ("G (near-equal 1e2)", NET_ENUM, W_NEAR_HIGH),
            ("F (near-equal 1e-2)", NET_ENUM, W_NEAR_LOW),
        ];
        // Every fixture is reported before the row fails: which of them
        // escape, and by how much, is the measurement — a row that
        // stopped at the first would hide the rest.
        let mut escaped = Vec::new();
        let mut certified: Vec<(&str, NurbsFaceBound)> = Vec::new();
        for ((name, net, weights), (truth_name, truth)) in descriptions.into_iter().zip(truths) {
            assert_eq!(
                name, truth_name,
                "the description table and the referee's table disagree on fixture order"
            );
            let surface = bilinear_rational(weights, net);
            let bound = nurbs_face_bound(&surface, FaceKey::default()).expect("covered");
            let d = Domination::new(
                "true (exact rational referee)",
                "certified",
                &[
                    ("uu", truth[0], bound.muu),
                    ("uv", truth[1], bound.muv),
                    ("vv", truth[2], bound.mvv),
                ],
            );
            println!(
                "bilinear {name}: true ({:.17e}, {:.17e}, {:.17e}) vs certified \
                 ({:.17e}, {:.17e}, {:.17e})",
                truth[0], truth[1], truth[2], bound.muu, bound.muv, bound.mvv
            );
            if !d.holds() {
                escaped.push(format!("fixture {name}: {d}"));
            }
            certified.push((name, bound));
        }
        assert!(
            escaped.is_empty(),
            "the certificate does not bound the DESCRIBED patch: {}",
            escaped.join(" | ")
        );
        // C and D describe ONE surface at two weight scales four decades
        // apart, so what they may differ by is the arithmetic's own dust
        // and nothing else. They do differ: this assembly is not
        // scale-invariant — interval arithmetic's rounding rides the weight
        // magnitude through the quotient rule — so the claim is a bound
        // on the disagreement, which is what a scale-dependent blow-up
        // would break. Measured at ~2.6e-13 relative on the nonzero
        // component; pinned at 1e-11, and the structurally-zero pair is
        // covered by the dust row's own band instead.
        let pick = |want: &str| {
            certified
                .iter()
                .find(|(n, _)| n.starts_with(want))
                .map(|(_, b)| *b)
                .expect("fixture present")
        };
        let (c, d) = (pick("C "), pick("D "));
        let gap = (c.muv - d.muv).abs() / c.muv;
        assert!(
            gap < 1e-11,
            "one surface at two weight scales certified `muv` {:.17e} against {:.17e} \
             — {gap:.3e} apart, past the dust this assembly's scale dependence explains",
            c.muv,
            d.muv
        );
    }

    /// **The tight stratum, enumerated.** Every escape this certificate
    /// has ever been caught in — two hosted, both measured in exact
    /// arithmetic — was a BILINEAR rational patch at a domain corner,
    /// where the quotient rule's factors all take their extremes at the
    /// same point and the cell hull has no slack left to absorb a
    /// rounding. `r1_random_rational_soundness_sweep` reaches that
    /// stratum on one draw in nine, which is why two hits took
    /// thousands of hosted runs.
    ///
    /// This row reaches it on every run, and it is an ENUMERATION
    /// rather than a sweep (implementer-discipline §8's shape
    /// question): its content is a product of boundary cases — four
    /// weights over a three-decade ladder, across a handful of nets
    /// chosen for the corner geometries that matter — so it is written
    /// out rather than drawn, and it carries no seed because there is
    /// nothing random in it to replay.
    ///
    /// The sampling grid is deliberately COARSE (9x9). It includes all
    /// four corners exactly, which is the whole subject; density in the
    /// interior is what the random sweep buys and this row does not owe
    /// it twice.
    ///
    /// **This row cannot go red on the refinement defect, and says so
    /// rather than implying otherwise.** It compares a SAMPLE, so it
    /// compares through [`SAMPLER_ULPS`], and the defect's escapes are
    /// 6–8 ulps. What it is for is COVERAGE of the stratum on every run
    /// — the random sweep reaches bilinear on one draw in nine — plus
    /// the worst excess printed in ulps, which is visible in a log even
    /// when the allowance absorbs it. The row that reds on the defect is
    /// `the_described_bilinear_rationals_true_second_partials_are_under_the_certified_sups`,
    /// which compares exact truths bare.
    ///
    /// **Where it asserts only the sampler's floor**: on a component
    /// whose certified figure sits under the floor
    /// [`Domination::second_partials`] allows, the near-unit pattern's
    /// `uu` and `vv` among them. The log names every such component.
    #[test]
    fn the_bilinear_rational_stratum_is_dominated_at_every_weight_decade() {
        // Four control nets whose corner geometry differs: a unit quad,
        // a twisted one (the mixed term's tight case), a skewed one, one
        // far from the origin (the recentring claim), and a tiny one.
        let nets: [[[f64; 3]; 4]; 5] = [
            [
                [0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 1.0, 0.7],
            ],
            [
                [0.0, 0.0, 0.0],
                [1.0, 0.0, 1.0],
                [0.0, 1.0, -1.0],
                [1.0, 1.0, 0.0],
            ],
            [
                [-0.3, -0.6, 0.9],
                [1.6, -1.8, -0.08],
                [1.1, -1.4, -1.8],
                [-0.4, 1.1, -0.16],
            ],
            [
                [1.0e3, 2.0e3, -5.0e2],
                [1.0e3, 2.0e3 + 1.0, -5.0e2],
                [1.0e3 + 1.0, 2.0e3, -5.0e2 + 0.5],
                [1.0e3 + 1.0, 2.0e3 + 1.0, -5.0e2],
            ],
            [
                [0.0, 0.0, 0.0],
                [0.0, 1.0e-6, 0.0],
                [1.0e-6, 0.0, 0.0],
                [1.0e-6, 1.0e-6, 3.0e-7],
            ],
        ];
        // Weight PATTERNS rather than the full product of a decade
        // ladder. The escapes that were measured exactly sat at
        // near-equal weights, once at 1e-2 and once at ~4, so what the
        // stratum needs is each decade taken with the corners
        // coinciding, plus the spreads that break that coincidence.
        // The full 3^4 product costs 81 patches per net and buys
        // nothing the six below do not: at ~0.19 s per patch in a debug
        // build the product is a 75-second row, which is not what this
        // claim is worth.
        //
        // **Every pattern is RATIONAL, and that is load-bearing.** A
        // constant pattern, `1.0` or any other, takes the INTEGRAL arm
        // (`NurbsSurface::constant_weight`) — which never refines at
        // `patch_cells` and is not this stratum at all. One sat here and
        // it was the pattern whose ratio this row printed as its worst,
        // so the printed number was about the wrong arm. So each
        // decade's coinciding corners are taken one ulp apart. The integral arm's own domination rows are
        // `planar_bilinear_bounds_collapse` and
        // `twisted_bilinear_mixed_term_is_tight`; the near-unit pattern
        // below is what reaches the rational arm nearest to unit
        // weights.
        const WEIGHTS: [[f64; 4]; 6] = [
            [1.0e-2f64.next_up(), 1.0e-2, 1.0e-2, 1.0e-2],
            [1.0 + f64::EPSILON, 1.0, 1.0, 1.0],
            [1.0e2f64.next_up(), 1.0e2, 1.0e2, 1.0e2],
            [1.0e-2, 1.0, 1.0e2, 1.0],
            [1.0e2, 1.0e-2, 1.0e-2, 1.0e2],
            [1.03e-2, 1.07e-2, 1.35e-2, 1.24e-2],
        ];
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let mut escaped = Vec::new();
        let mut worst_ratio = 0.0f64;
        let mut worst_excess_ulps = f64::NEG_INFINITY;
        let mut worst_where = String::new();
        let mut floor_only = Vec::new();
        let mut patches = 0usize;
        for net in &nets {
            for w in WEIGHTS {
                let s = NurbsSurface::new(
                    kv.clone(),
                    kv.clone(),
                    net.iter().copied().map(Point3::from_array).collect(),
                    w.to_vec(),
                )
                .unwrap();
                let b = nurbs_face_bound(&s, FaceKey::default()).expect("covered");
                patches += 1;
                let (wuu, wuv, wvv) = sample_worst(&s, 8);
                let d = Domination::second_partials((wuu, wuv, wvv), &b);
                if !d.holds() {
                    escaped.push(format!("weights {w:?}: {d}"));
                }
                for name in d.floor_only() {
                    floor_only.push(format!("{name} on {w:?}"));
                }
                // The sampled/certified ratio, and how far the sample
                // sits ABOVE the certified figure in ulps of it. The
                // second number is what the shipped allowance absorbs,
                // so it is printed rather than left to the comparison:
                // an escape inside the allowance is invisible in
                // `holds` and visible here.
                for (name, sampled, certified) in
                    [("uu", wuu, b.muu), ("uv", wuv, b.muv), ("vv", wvv, b.mvv)]
                {
                    if certified <= 0.0 || !certified.is_finite() {
                        continue;
                    }
                    worst_ratio = worst_ratio.max(sampled / certified);
                    let ulps = (sampled - certified) / (certified.abs() * f64::EPSILON);
                    if ulps > worst_excess_ulps {
                        worst_excess_ulps = ulps;
                        worst_where = format!(
                            "{name} on weights {w:?}: sampled {sampled:.17e} \
                             certified {certified:.17e}"
                        );
                    }
                }
            }
        }
        println!(
            "bilinear stratum: {patches} patches, worst sampled/certified {worst_ratio:.17e}, \
             worst excess {worst_excess_ulps:.3} ulps of the certified figure ({worst_where}); \
             asserting only the sampler floor: {}",
            floor_only.join(", ")
        );
        assert!(
            escaped.is_empty(),
            "the certificate is escaped on the bilinear stratum: {}",
            escaped.join(" | ")
        );
        // ANTI-VACUITY, and it is a static witness rather than a floor
        // over a sample: this enumeration contains patches whose bound
        // is tight at a corner, so a bound that had become loose enough
        // to pass everything would fail here. The figure is the
        // stratum's, not a tolerance — lowering it would be the repair
        // this row exists to refuse.
        assert!(
            worst_ratio > 0.9,
            "the bilinear stratum stopped being tight: worst sampled/certified \
             {worst_ratio:.17e} is not above 0.9, so this row no longer challenges \
             the bound it asserts"
        );
    }

    /// **The measurement apparatus for the sampler's own error**, which
    /// is a number two work rows cite as the figure a future lane sizes
    /// [`SAMPLER_ULPS`] from
    /// (`work/props/the-samplers-own-error-has-three-spellings-and-no-home`,
    /// `work/chord/soundness-sweep-allowance-is-fifty-times-the-measured-sampler-error`).
    ///
    /// It is committed because a cited number whose apparatus was thrown
    /// away is a number nobody can re-take. One command reproduces the
    /// whole table:
    ///
    /// ```text
    /// cargo test --release -p mesh --lib sampler_error_dump -- --ignored --nocapture \
    ///   | python3 crates/mesh/tests/sampler_error.py
    /// ```
    ///
    /// 400 patches at `CAD_FUZZ_EFFORT=1`, and the dial scales it.
    ///
    /// What it dumps, per trial: the described bilinear patch's bits, the
    /// certified triple, and the 61x61 sampler's per-component ARGMAX —
    /// which is the part that matters, because the referee has to evaluate
    /// the exact truth at the same point the sampler found its worst, not
    /// at a point of its own choosing.
    ///
    /// `#[ignore]`d, and that is the honest shape for it: it asserts
    /// nothing, so gating on it would be a green over a print. What it
    /// costs to run is minutes; what a gate would buy is nothing.
    #[test]
    #[ignore = "measurement apparatus: prints a dump for crates/mesh/tests/sampler_error.py"]
    fn sampler_error_dump() {
        // Breadth rides the shared EFFORT dial rather than an env var of
        // its own: `mesh` is a kernel crate and may not read the
        // environment at all (the `no ambient environment in the kernel`
        // gate), and `CAD_FUZZ_EFFORT` is the lever the repo already has.
        let trials = test_utils::fuzz::scaled(400);
        // A varying seed, logged: this is a counterexample-shaped
        // measurement (the DISTRIBUTION over draws is the answer), so
        // successive runs are supposed to sample different patches, and
        // the log line is how a cited figure is traced to its draw.
        let mut rng = test_utils::fuzz::start("nurbs_cert::sampler_error");
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let hex = |x: f64| format!("{:016x}", x.to_bits());
        for trial in 0..trials {
            let mut control = Vec::new();
            let mut weights = Vec::new();
            for _ in 0..4 {
                control.push(Point3::new(
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                    rng.range(-2.0, 2.0),
                ));
                weights.push(10f64.powf(rng.range(-2.0, 2.0)));
            }
            let s = NurbsSurface::new(kv.clone(), kv.clone(), control.clone(), weights.clone())
                .expect("positive finite weights");
            let Ok(b) = nurbs_face_bound(&s, FaceKey::default()) else {
                continue;
            };
            // `sample_worst`'s grid and order, with the argmax recorded.
            let grid = 60u32;
            let mut best = [(0.0f64, 0.0f64, 0.0f64); 3];
            for i in 0..=grid {
                for j in 0..=grid {
                    let u = f64::from(i) / f64::from(grid);
                    let v = f64::from(j) / f64::from(grid);
                    let jet = s.ders(u, v);
                    for (k, value) in [jet.duu.norm(), jet.duv.norm(), jet.dvv.norm()]
                        .into_iter()
                        .enumerate()
                    {
                        if value > best[k].0 {
                            best[k] = (value, u, v);
                        }
                    }
                }
            }
            let list = |v: &[f64]| v.iter().map(|x| hex(*x)).collect::<Vec<_>>().join(",");
            let points: Vec<f64> = control.iter().flat_map(|p| p.to_array()).collect();
            println!(
                "DUMP {trial} W {} C {} B {} ARG {}",
                list(&weights),
                list(&points),
                list(&[b.muu, b.muv, b.mvv]),
                best.iter()
                    .map(|(n, u, v)| format!("{}:{}:{}", hex(*n), hex(*u), hex(*v)))
                    .collect::<Vec<_>>()
                    .join(",")
            );
        }
    }

    /// A C⁰ crease (interior multiplicity = degree) refuses typed —
    /// the Taylor remainder needs C¹.
    #[test]
    fn c0_crease_refuses_typed() {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
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
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
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

    pub(crate) fn sample_worst(s: &NurbsSurface<f64>, n: u32) -> (f64, f64, f64) {
        let (u0, u1) = s.knots_u_f64_interim().domain();
        let (v0, v1) = s.knots_v_f64_interim().domain();
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
        let d = Domination::second_partials((wuu, wuv, wvv), &b);
        assert!(d.holds(), "{name}: {d}");
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
        let kv_u = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
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
        let (u0, u1) = s.knots_u_f64_interim().domain();
        let (v0, v1) = s.knots_v_f64_interim().domain();
        for delta in [1.0, 2e-1] {
            let budget = lattice_budget(delta);
            let (hu, hv) = b.grid_steps(budget);
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
            assert_no_sample_exceeded_its_certificate("r1_extreme", delta, worst_ratio);
        }
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
    pub(crate) fn whole_net_bound(s: &NurbsSurface<f64>) -> Option<NurbsFaceBound> {
        use geom_core::spline::{Derived, TensorCoeffs, TensorNet};
        assert!(
            s.constant_weight().is_some(),
            "the whole-net spelling is the integral arm's; a rational face has no such arm"
        );
        let (kv_u, kv_v) = (s.knots_u_f64_interim(), s.knots_v_f64_interim());
        patch_bound::check_direction(kv_u).ok()?;
        patch_bound::check_direction(kv_v).ok()?;
        let nv = kv_v.control_count();
        let zero = Interval::zero();
        let mut sq = [zero; 5];
        // Past `check_direction`, a first partial is a spline or one
        // span's constant, whose second partial is zero; a
        // discontinuous one is a broken premise, said loudly.
        fn second(
            derived: Derived<TensorCoeffs<'_>>,
            diff: impl FnOnce(&TensorCoeffs<'_>) -> TensorNet,
        ) -> Interval {
            match derived {
                Derived::Vector(t) => diff(&t).hull(),
                Derived::SpanConstant(_) => Interval::zero(),
                Derived::Discontinuous(_) => {
                    panic!("a discontinuous derivative past check_direction")
                }
            }
        }
        for c in 0..3 {
            let base = TensorCoeffs::from_fn(kv_u, kv_v, |i, j| {
                let p = s.control()[i * nv + j];
                Interval::point(match c {
                    0 => p.x,
                    1 => p.y,
                    _ => p.z,
                })
            });
            let (d10, d01, d11) = (base.diff_u(), base.diff_v(), base.diff_uv());
            let g20 = second(base.derived_u(), |t| t.diff_u());
            let g02 = second(base.derived_v(), |t| t.diff_v());
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

    /// **The counterfactual's digits are pinned**: re-expressing the
    /// retired whole-net arm through the shared home
    /// ([`whole_net_bound`]) answers these literals exactly. They began as
    /// the deleted `integral_face_bound`'s digits and have since moved
    /// only tighter, each move noted at its row.
    ///
    /// The fold-cost table and the tighter-or-equal claim are both
    /// measured AGAINST this function, so a counterfactual that drifted
    /// unnoticed would silently restate the comparison they rest on.
    #[test]
    fn cert10_the_whole_net_counterfactual_digits_are_pinned() {
        for (name, s, want) in [
            (
                // The wavy row's `muu` moved TIGHTER with interval arithmetic's
                // arithmetic (was 10.394_094_997_048_835); the other
                // four components are unmoved.
                "wavy",
                wavy(),
                [
                    10.394_094_997_048_823,
                    9.245_962_289_850_127,
                    13.743_674_653_694_725,
                    5.275_689_895_607_06,
                    4.335_017_346_749_228,
                ],
            ),
            (
                // Every component moved TIGHTER, and `muv` all the
                // way to EXACTLY zero: the channel is identically zero
                // in the reals and the retired arithmetic's
                // unconditional pad was the whole of its
                // 1.08e-13 (was 48.219_564_494_093_156,
                // 1.084_596_414_278_405_7e-13, 2.000_000_000_000_007,
                // 20.000_000_000_000_025, 2.000_000_000_000_002_7).
                // The last three sum exact squares (4, 400, 4), and
                // their roots are exact: 2, 20 and 2.
                "staggered_channels",
                staggered_channels(),
                [48.219_564_494_093_07, 0.0, 2.0, 20.0, 2.0],
            ),
        ] {
            let b = whole_net_bound(&s).expect("covered");
            let got = [b.muu, b.muv, b.mvv, b.mu1, b.mv1];
            // EVERY component that moved, not the first: a re-pin
            // reads the whole row at once, and a first-mismatch
            // report costs one rebuild per component.
            let moved: Vec<String> = got
                .iter()
                .zip(&want)
                .enumerate()
                .filter(|(_, (g, w))| g != w)
                .map(|(k, (g, w))| format!("component {k} is {g:.17e}, was {w:.17e}"))
                .collect();
            assert!(
                moved.is_empty(),
                "{name}: the counterfactual moved — {}",
                moved.join("; ")
            );
        }
    }

    /// The per-cell fold: the componentwise max over `patch_cells`'
    /// per-cell bounds.
    pub(crate) fn fold_bound(s: &NurbsSurface<f64>) -> Option<NurbsFaceBound> {
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
        // **Re-pinned when certification arithmetic became a newtype over the
        // backend**: interval arithmetic padded one representable step outward on
        // every operation and the backend pads only where the
        // operation was inexact, so the fold's `muu` came in tighter
        // (was 1.0394094997048835e1). The claim this row makes — the
        // smooth fixture's three second-partial channels peak in one
        // cell, so the fold reproduces the whole-net number EXACTLY —
        // is unmoved: the counterfactual below moved with it.
        assert!(
            bw.muu == 1.039_409_499_704_882_3e1,
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
    /// signed one is each partial's own quotient, `M/W³`, whose
    /// coordinates lie between their coefficient ratios on a piece
    /// (`patch_bound`, "The rational arm"). The signed reading is
    /// strictly tighter and both are sound, so the shipped grid sizing
    /// reads the signed one.
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
                    ("muu", b.bound.muu, cell_component(norm_sq(&c.s_uu))),
                    ("muv", b.bound.muv, cell_component(norm_sq(&c.s_uv))),
                    ("mvv", b.bound.mvv, cell_component(norm_sq(&c.s_vv))),
                    ("mu1", b.bound.mu1, cell_component(norm_sq(&c.s_u))),
                    ("mv1", b.bound.mv1, cell_component(norm_sq(&c.s_v))),
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
    /// inequality costs most: on the quarter cylinder `S_uv` is zero in
    /// ℝ, and the quotient's coefficients read it as rounding dust, while the
    /// magnitude spelling — which cannot see a sign — reports a figure
    /// of order one.
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
    /// budget decade.
    ///
    /// `#[ignore]`d: a measurement, not an assertion.
    ///
    /// **The `mvv` entries are NOT pinned digits.** On these ruled
    /// (degree-1 in `v`) faces the true `S_vv` is identically zero.
    /// The entries below carry the retired reading's ORDER of magnitude
    /// and their provenance: that reading enclosed a refined net and
    /// reported its insertion rounding, where the quotient reading now
    /// proves the zero on a face whose weights are constant along `v`
    /// (`NurbsFaceBound::muu`'s docs). Nothing in this harness is
    /// sensitive to them (a `mvv` of 0, 1e-15 or 5e-15 sizes the same
    /// grid).
    #[test]
    #[ignore = "measurement harness: prints the rational-face grid re-sizing"]
    fn cert10_rational_grid_resizing() {
        // Measured on this tree before the retirement. The two first
        // partials and `muu`/`muv` are digits; `mvv` is dust (above).
        let dust = 4.3e-15;
        let retired = [
            (
                "quarter_cylinder",
                NurbsFaceBound {
                    muu: 3.942_263_838_556_179_7,
                    muv: 1.266_375_820_315_083_4,
                    mvv: dust,
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
                    mvv: dust,
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
        for budget in [1e-2, 4e-3, 1e-3] {
            println!("\nbudget = {budget:.0e}");
            println!(
                "{:<18} {:>6} {:>13} {:>13} {:>9} {:>9} {:>7}",
                "face", "read", "h_u", "h_v", "div_u", "div_v", "cells"
            );
            for (name, old, s) in &retired {
                let new = nurbs_face_bound(s, FaceKey::default()).expect("covered");
                for (tag, b) in [("mag", *old), ("signed", new)] {
                    let (hu, hv) = b.grid_steps(budget);
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

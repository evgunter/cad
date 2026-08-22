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
//! **The divisor is `w_min`, argued not assumed.** On the cell,
//! `w ∈ [w_min, w_max]` of the active weights (convex combination).
//! Every numerator bound above is a NONNEGATIVE magnitude sup, and for
//! a nonnegative numerator the conservative (sup-side) division is by
//! the SMALLEST denominator: `|X|/w ≤ sup|X|/w_min`. This is the
//! mirror image of the speed meter's lower-bound choice
//! (`geom::rational_speed_lower_bound`: a nonnegative numerator
//! divides by `w_max` for an INF bound) — same lattice, opposite side.
//! The interval division by the cell's weight hull `[w_lo, w_hi]`
//! computes exactly `sup/w_lo`, outward-rounded, and poisons if
//! positivity was never proven.
//!
//! **Recentring keeps the cross terms cell-sized**: with the cell's
//! control centroid as `c`, `sup|S − c|` is a cell-of-control-net
//! fact, not a whole-patch one, so `(S − c)·w_dd` and `S_d·w_d` do not
//! inflate with the patch's distance from the origin (the M8-2
//! template's trick, lifted to two parameters). The whole-domain bound
//! is the max over cells of the per-cell sups, after the FIXED
//! [`RATIONAL_CERT_SPLITS`] refinement (schedule docs there).
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
//! (`a_u ≤ 2·h_u`, `a_v ≤ 2·h_v`) and splitting δ_s across the `u`/`v`
//! groups via `2·a_u·a_v ≤ a_u² + a_v²`:
//!
//! ```text
//! cert ≤ (muu + muv)·h_u² + (mvv + muv)·h_v²  ⇒
//! h_u = √(δ_s / (2(muu + muv))),   h_v = √(δ_s / (2(mvv + muv)))
//! ```
//!
//! A zero group (e.g. a degree-1 direction of a ruled wall with
//! `muv = 0`) leaves that direction unconstrained — step ∞, one cell.
//!
//! **Since TESS-SPAN (the #320 span promotion) the shipped grid is
//! sized PER KNOT-SPAN CELL in `v`**: the trimmed lane consumes
//! [`nurbs_cell_grid`] — the same certified assembly reported cell by
//! cell — and builds a TENSOR grid whose v-rows apply the step rule
//! above per v-band with that band's own bounds
//! ([`NurbsCellGrid::row_bound`]), rows landing on the band
//! boundaries so a grid triangle's certificate is the certificate of
//! the band containing it ([`NurbsCellGrid::cert`]). The u-columns
//! deliberately KEEP the whole-patch schedule: they stay
//! phase-aligned with the chord pass's boundary points (sized from
//! the same steps), which is what keeps anisotropic boundary slivers
//! certified (`crate::trimmed` module docs tell the measured story);
//! the u-direction's per-cell share of the span slack is forfeited
//! and metered. The point-selection rule (the
//! `2·a_u·a_v ≤ a_u² + a_v²` grouping) is deliberately UNCHANGED —
//! decoupling it is the split unit's open aspect-policy question, out
//! of scope here.
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

use core::ops::RangeInclusive;

use geom::NurbsSurface;
use geom_core::ring_interval::RingInterval;
use geom_core::spline::KnotVector;
use geom_core::spline::hull::derivative_coeffs;
use topo::FaceKey;

use crate::types::TessellateError;

/// Certified sup bounds on the three second partials of one described
/// non-rational NURBS face, over its whole chart rectangle (module
/// docs). Computed once per face; consumed by the trimmed lane's grid
/// sizing, its per-triangle certificate, and the chord pass's
/// adjacent-face tightening.
#[derive(Clone, Copy, Debug)]
pub(crate) struct NurbsFaceBound {
    /// `sup ‖S_uu‖` (outward-rounding dust for a degree-1 single-span
    /// u direction — the exact-zero term still passes through the
    /// ring's conservative arithmetic).
    pub muu: f64,
    /// `sup ‖S_uv‖`.
    pub muv: f64,
    /// `sup ‖S_vv‖` (dust for a degree-1 single-span v direction).
    pub mvv: f64,
}

impl NurbsFaceBound {
    /// The per-triangle deviation certificate `Q/4` (module docs) for
    /// a triangle whose UV corners are `uv`.
    pub(crate) fn cert(&self, uv: [[f64; 2]; 3]) -> f64 {
        let au = max3(uv[0][0], uv[1][0], uv[2][0]) - min3(uv[0][0], uv[1][0], uv[2][0]);
        let av = max3(uv[0][1], uv[1][1], uv[2][1]) - min3(uv[0][1], uv[1][1], uv[2][1]);
        0.25 * (self.muu * au.powi(2) + 2.0 * self.muv * au * av + self.mvv * av.powi(2))
    }

    /// The `(h_u, h_v)` UV grid steps for sizing target `delta_s`
    /// (module docs) — `f64::INFINITY` for an unconstrained direction
    /// ([`crate::sizing::ceil_count`] turns that into one cell).
    pub(crate) fn grid_steps(&self, delta_s: f64) -> (f64, f64) {
        let step = |group: f64| {
            if group > 0.0 {
                (delta_s / (2.0 * group)).sqrt()
            } else {
                f64::INFINITY
            }
        };
        (step(self.muu + self.muv), step(self.mvv + self.muv))
    }
}

fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}

fn min3(a: f64, b: f64, c: f64) -> f64 {
    a.min(b).min(c)
}

/// One analysis cell before its components are collapsed to f64: the
/// cell's UV extent and the three per-component squared sums the bound
/// is assembled from. Kept as ring enclosures so poison flows exactly
/// as the whole-patch assembly's does (`rational_face_bound`).
pub(crate) struct CellRaw {
    /// The cell's `u` extent, `[lo, hi]`.
    pub u: (f64, f64),
    /// The cell's `v` extent, `[lo, hi]`.
    pub v: (f64, f64),
    /// `Σ_c sup²(S_uu^c)` on the cell.
    pub sq_uu: RingInterval,
    /// `Σ_c sup²(S_uv^c)` on the cell.
    pub sq_uv: RingInterval,
    /// `Σ_c sup²(S_vv^c)` on the cell.
    pub sq_vv: RingInterval,
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
fn cell_component(sq: RingInterval) -> f64 {
    sq.hi().sqrt().next_up()
}

/// A span's `[knot, next knot]` extent (the caller has already
/// established the span is nonempty, so both knots exist).
fn span_extent(kv: &KnotVector, span: usize) -> (f64, f64) {
    let k = kv.knots();
    (
        k.get(span).copied().unwrap_or(f64::NAN),
        k.get(span + 1).copied().unwrap_or(f64::NAN),
    )
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
/// the cells of the fixed [`RATIONAL_CERT_SPLITS`] refinement. The
/// budget row carries the cell count, so a reader is never guessing
/// which.
///
/// The max over the returned cells is `≤` the face bound in every arm
/// (the whole-patch hull is over a superset of every cell's window);
/// `crate::budget`'s test asserts exactly that.
///
/// # Errors
///
/// As [`nurbs_face_bound`] — same gates, same arms, same refusals.
pub(crate) fn nurbs_cell_bounds(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<Vec<CellBound>, TessellateError> {
    check_direction(n.knots_u(), fk)?;
    check_direction(n.knots_v(), fk)?;
    let raw = if n.weights().iter().any(|w| *w != 1.0) {
        rational_cell_bounds(n, fk)?
    } else {
        integral_cell_bounds(n, fk)?
    };
    Ok(raw
        .into_iter()
        .map(|c| CellBound {
            u: c.u,
            v: c.v,
            bound: NurbsFaceBound {
                muu: cell_component(c.sq_uu),
                muv: cell_component(c.sq_uv),
                mvv: cell_component(c.sq_vv),
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
        if !(b.muu.is_finite() && b.muv.is_finite() && b.mvv.is_finite()) {
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

    /// Cell boundary values in `u`. Since the shipped columns went
    /// back to the whole-patch schedule (module docs), only the tests
    /// read this; the certificate lookup uses the field directly.
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
        };
        for c in 0..cols {
            let b = self.bound(c, ri);
            m.muu = m.muu.max(b.muu);
            m.muv = m.muv.max(b.muv);
            m.mvv = m.mvv.max(b.mvv);
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
    /// (subdividing in `u`, realized aspect `s_u/h_v` beyond
    /// [`SAFE_ASPECT`]) and its immediate neighbours take the
    /// whole-patch column count instead. The step derivation
    /// [`NurbsFaceBound::grid_steps`] is untouched either way
    /// (TESS-SPAN's binding constraint); only WHICH bound feeds it
    /// changes, and the patch count only ever adds columns.
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
        let mut malign = Vec::with_capacity(edges.len().saturating_sub(1));
        for w in edges.windows(2) {
            let (va, vb) = (w[0], w[1]);
            // The band is found by the slab midpoint — strictly inside
            // one band, since no interior cut crosses a slab.
            let (hu, hv) = self
                .row_bound(self.row_of(0.5 * (va + vb)))
                .grid_steps(delta_s);
            let nuc = crate::sizing::ceil_count(du, hu)?;
            let nvc = crate::sizing::ceil_count(vb - va, hv)?;
            // Malignity is judged on the REALIZED spacings `s_u/s_v`,
            // not the pre-`ceil` ideal steps: the lattice a sliver
            // lives in has rows every `s_v = (vb−va)/nvc ≤ h_v`, so
            // testing against `h_v` under-estimates the aspect by up
            // to ~2x whenever a band's extent barely exceeds one step
            // (R2's review fixture, pinned in the tests below).
            #[allow(clippy::cast_precision_loss)]
            let su = du / nuc as f64;
            #[allow(clippy::cast_precision_loss)]
            let sv = (vb - va) / nvc as f64;
            malign.push(nuc >= 2 && sv.is_finite() && sv > 0.0 && su > SAFE_ASPECT * sv);
            bands.push(BandCounts { va, vb, nuc, nvc });
        }
        for (i, b) in bands.iter_mut().enumerate() {
            let near_malign = malign[i]
                || (i > 0 && malign[i - 1])
                || malign.get(i + 1).copied().unwrap_or(false);
            if near_malign {
                b.nuc = b.nuc.max(patch_nuc);
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

/// The integral arm of [`nurbs_cell_bounds`]: the same knot-differenced
/// coefficient nets [`integral_face_bound`] hulls whole, hulled instead
/// over each span's own active window (`Span::derived_window`, the
/// rational arm's windowing verbatim). A degree-1 direction's
/// second-derivative net does not exist, and its term is the exact zero
/// [`second_derivative_hull`] answers for the same case.
fn integral_cell_bounds(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<Vec<CellRaw>, TessellateError> {
    let (kv_u, kv_v) = (n.knots_u(), n.knots_v());
    let (nu, nv) = n.control_counts();
    let kv_u1 = (kv_u.degree() >= 2)
        .then(|| derived_knots(kv_u, fk))
        .transpose()?;
    let kv_v1 = (kv_v.degree() >= 2)
        .then(|| derived_knots(kv_v, fk))
        .transpose()?;
    // Per component: the u-major net and its three derivative nets.
    struct Comp {
        d20: Option<Net>,
        d02: Option<Net>,
        d11: Net,
    }
    let comps: Vec<Comp> = (0..3)
        .map(|c| {
            // Row-major layout: control[iu·nv + iv] (NurbsSurface docs).
            let base: Net = (0..nu)
                .map(|i| {
                    (0..nv)
                        .map(|j| {
                            let p = n.control()[i * nv + j];
                            RingInterval::point(match c {
                                0 => p.x,
                                1 => p.y,
                                _ => p.z,
                            })
                        })
                        .collect()
                })
                .collect();
            let d10 = net_d_u(kv_u, &base);
            let d01 = net_d_v(kv_v, &base);
            Comp {
                d11: net_d_v(kv_v, &d10),
                d20: kv_u1.as_ref().map(|k1| net_d_u(k1, &d10)),
                d02: kv_v1.as_ref().map(|k1| net_d_v(k1, &d01)),
            }
        })
        .collect();
    let zero = RingInterval::zero();
    let mut cells = Vec::new();
    for su in kv_u.first_span()..=kv_u.last_span() {
        let Some(span_u) = kv_u.span(su) else {
            continue;
        };
        for sv in kv_v.first_span()..=kv_v.last_span() {
            let Some(span_v) = kv_v.span(sv) else {
                continue;
            };
            let (wu_val, wv_val) = (span_u.window(), span_v.window());
            let (wu_d1, wv_d1) = (span_u.first_derived_window(), span_v.first_derived_window());
            let (wu_d2, wv_d2) = (span_u.derived_window(2), span_v.derived_window(2));
            let (mut cuu, mut cuv, mut cvv) = (zero, zero, zero);
            for comp in &comps {
                let s20 = comp
                    .d20
                    .as_ref()
                    .zip(wu_d2.as_ref())
                    .map_or(zero, |(d, wu2)| window_hull(d, wu2, &wv_val));
                let s02 = comp
                    .d02
                    .as_ref()
                    .zip(wv_d2.as_ref())
                    .map_or(zero, |(d, wv2)| window_hull(d, &wu_val, wv2));
                let s11 = window_hull(&comp.d11, &wu_d1, &wv_d1);
                cuu = cuu + s20.sqr();
                cvv = cvv + s02.sqr();
                cuv = cuv + s11.sqr();
            }
            cells.push(CellRaw {
                u: span_extent(kv_u, su),
                v: span_extent(kv_v, sv),
                sq_uu: cuu,
                sq_uv: cuv,
                sq_vv: cvv,
            });
        }
    }
    Ok(cells)
}

/// The certified Hessian sup bounds of a described NURBS face, or the
/// typed refusal naming its class (module docs).
///
/// Two arms share the gates and the finite check: the INTEGRAL arm
/// (all weights bitwise `1.0` — the kernel's definition of
/// non-rational) is the original hull assembly, bit-identical; the
/// RATIONAL arm (M8-5) is the quotient-rule assembly over the
/// homogeneous nets (module docs, "The rational arm").
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
    check_direction(n.knots_u(), fk)?;
    check_direction(n.knots_v(), fk)?;
    let bound = if n.weights().iter().any(|w| *w != 1.0) {
        rational_face_bound(n, fk)?
    } else {
        integral_face_bound(n, fk)?
    };
    if !(bound.muu.is_finite() && bound.muv.is_finite() && bound.mvv.is_finite()) {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "NURBS face second-derivative hull is unbounded/poisoned — \
                   outside the certified inventory",
        });
    }
    Ok(bound)
}

/// The integral (all-unit-weight) arm of [`nurbs_face_bound`]: the
/// direct control-hull convexity assembly (module docs). Bit-identical
/// to the pre-M8-5 path.
fn integral_face_bound(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    let (nu, nv) = n.control_counts();
    let comp = |c: usize| -> Vec<RingInterval> {
        n.control()
            .iter()
            .map(|p| {
                RingInterval::point(match c {
                    0 => p.x,
                    1 => p.y,
                    _ => p.z,
                })
            })
            .collect()
    };
    let mut sq_uu = RingInterval::zero();
    let mut sq_uv = RingInterval::zero();
    let mut sq_vv = RingInterval::zero();
    for c in 0..3 {
        // Row-major layout: control[iu·nv + iv] (NurbsSurface docs).
        let grid = comp(c);
        let u_rows: Vec<Vec<RingInterval>> = (0..nv)
            .map(|j| (0..nu).map(|i| grid[i * nv + j]).collect())
            .collect();
        let v_rows: Vec<Vec<RingInterval>> = (0..nu)
            .map(|i| (0..nv).map(|j| grid[i * nv + j]).collect())
            .collect();
        sq_uu = sq_uu + second_derivative_hull(n.knots_u(), &u_rows, fk)?.sqr();
        sq_vv = sq_vv + second_derivative_hull(n.knots_v(), &v_rows, fk)?.sqr();
        sq_uv = sq_uv + mixed_derivative_hull(n.knots_u(), n.knots_v(), &u_rows)?.sqr();
    }
    let m = |sq: RingInterval| sq.hi().sqrt().next_up();
    Ok(NurbsFaceBound {
        muu: m(sq_uu),
        muv: m(sq_uv),
        mvv: m(sq_vv),
    })
}

/// The fixed refinement schedule of the RATIONAL arms (this face bound
/// and `chords`' rational carrier bound): every nonempty span of every
/// direction splits into this many equal pieces before the per-cell
/// assembly. A CONSTANT (D9: structure, never a data-dependent
/// iteration) — the `RATIONAL_METER_SPLITS = 16` precedent of
/// `geom::curves`' rational speed meter, mirrored. Knot insertion is
/// evaluation-invariant in ℝ, so it changes no geometry; it only
/// shrinks every hull the bound is assembled from, which is what keeps
/// the `sup‖S − c‖·sup|w_dd|` cross terms cell-sized. (The f64
/// insertion arithmetic rounds; that residue sits inside the crate's
/// documented f64-evaluation slack — the ε side of δ + ε — exactly
/// where every `surface.eval` already spends it.)
pub(crate) const RATIONAL_CERT_SPLITS: usize = 16;

/// The interior split points of the fixed rational refinement schedule
/// for one knot vector (module-constant docs): `RATIONAL_CERT_SPLITS`
/// equal pieces per nonempty span, skipping any split point floating
/// point collapses onto a span end — refinement is a tightening, never
/// a correctness condition (the speed meter's rule, verbatim).
pub(crate) fn rational_split_points(kv: &KnotVector) -> Vec<f64> {
    let mut add = Vec::new();
    for span in kv.first_span()..=kv.last_span() {
        if !kv.span_is_nonempty(span) {
            continue;
        }
        let (Some(&lo), Some(&hi)) = (kv.knots().get(span), kv.knots().get(span + 1)) else {
            continue;
        };
        for k in 1..RATIONAL_CERT_SPLITS {
            #[allow(clippy::cast_precision_loss)]
            let f = k as f64 / RATIONAL_CERT_SPLITS as f64;
            let u = lo + (hi - lo) * f;
            if u > lo && u < hi {
                add.push(u);
            }
        }
    }
    add
}

/// A coefficient net as ring enclosures, u-major: `net[i][j]` is the
/// coefficient at u-index `i`, v-index `j`.
type Net = Vec<Vec<RingInterval>>;

/// Differences a net once along u against `kv_u` (per fixed v index):
/// `(nu − 1) × nv` from `nu × nv`.
fn net_d_u(kv_u: &KnotVector, net: &Net) -> Net {
    let nu = net.len();
    let nv = net.first().map_or(0, Vec::len);
    let cols: Vec<Vec<RingInterval>> = (0..nv)
        .map(|j| {
            let col: Vec<RingInterval> = (0..nu).map(|i| net[i][j]).collect();
            derivative_coeffs(kv_u, &col)
        })
        .collect();
    let nu1 = nu.saturating_sub(1);
    (0..nu1)
        .map(|i| {
            (0..nv)
                .map(|j| cols[j].get(i).copied().unwrap_or_else(RingInterval::poison))
                .collect()
        })
        .collect()
}

/// Differences a net once along v against `kv_v` (per fixed u index):
/// `nu × (nv − 1)`.
fn net_d_v(kv_v: &KnotVector, net: &Net) -> Net {
    net.iter().map(|row| derivative_coeffs(kv_v, row)).collect()
}

/// The signed hull of `a[i][j] − c·w[i][j]` over the window
/// `wu × wv` — the recentred homogeneous net `Ã = A − c·w`
/// read through the linearity of knot differencing (`d(A − c·w) =
/// dA − c·dw`, entrywise, same knots). Out-of-range indices poison.
fn window_tilde_hull(
    a: &Net,
    w: &Net,
    c: RingInterval,
    wu: &RangeInclusive<usize>,
    wv: &RangeInclusive<usize>,
) -> RingInterval {
    let mut acc: Option<RingInterval> = None;
    for i in wu.clone() {
        for j in wv.clone() {
            let (av, wv) = match (
                a.get(i).and_then(|r| r.get(j)),
                w.get(i).and_then(|r| r.get(j)),
            ) {
                (Some(&av), Some(&wv)) => (av, wv),
                _ => (RingInterval::poison(), RingInterval::poison()),
            };
            let e = av - c * wv;
            acc = Some(match acc {
                None => e,
                Some(h) => RingInterval::hull(h, e),
            });
        }
    }
    acc.unwrap_or_else(RingInterval::poison)
}

/// The signed hull of `net[i][j]` over the window `wu × wv`.
/// Out-of-range indices poison.
///
/// Distinct from [`window_tilde_hull`] with a zero centre on purpose:
/// that spelling computes `a − 0·w`, and the ring's outward rounding
/// makes the subtraction widen the answer by an ulp — enough to put a
/// CELL's bound above the whole-patch hull it is a subset of, which is
/// exactly the invariant `crate::budget` checks.
fn window_hull(net: &Net, wu: &RangeInclusive<usize>, wv: &RangeInclusive<usize>) -> RingInterval {
    let mut acc: Option<RingInterval> = None;
    for i in wu.clone() {
        for j in wv.clone() {
            let e = net
                .get(i)
                .and_then(|r| r.get(j))
                .copied()
                .unwrap_or_else(RingInterval::poison);
            acc = Some(match acc {
                None => e,
                Some(h) => RingInterval::hull(h, e),
            });
        }
    }
    acc.unwrap_or_else(RingInterval::poison)
}

/// The `[0, sup]` magnitude enclosure of a signed hull (poison flows).
fn mag_iv(h: RingInterval) -> RingInterval {
    RingInterval::from_bounds(0.0, h.mag())
}

/// The rational (non-unit-weight) arm of [`nurbs_face_bound`]: the
/// whole-patch max over [`rational_cell_bounds`]' per-cell enclosures.
///
/// The accumulation is hull-then-`m`, at the ring level and in cell
/// order, because that is what makes a poisoned cell reach the shared
/// finite check: `m` maps poison to NaN, and an f64 `max` over NaN
/// would drop it.
fn rational_face_bound(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<NurbsFaceBound, TessellateError> {
    let cells = rational_cell_bounds(n, fk)?;
    let (mut sq_uu, mut sq_uv, mut sq_vv) = (None, None, None);
    let acc = |slot: &mut Option<RingInterval>, v: RingInterval| {
        *slot = Some(match *slot {
            None => v,
            Some(h) => RingInterval::hull(h, v),
        });
    };
    for c in &cells {
        acc(&mut sq_uu, c.sq_uu);
        acc(&mut sq_uv, c.sq_uv);
        acc(&mut sq_vv, c.sq_vv);
    }
    let m = |sq: Option<RingInterval>| sq.map_or(f64::NAN, cell_component);
    Ok(NurbsFaceBound {
        muu: m(sq_uu),
        muv: m(sq_uv),
        mvv: m(sq_vv),
    })
}

/// The per-cell quotient-rule Hessian assembly the rational arm is the
/// max of (module docs, "The rational arm"): one [`CellRaw`] per
/// nonempty cell of the FIXED [`RATIONAL_CERT_SPLITS`] refinement. The
/// C¹ gates have already run (homogeneous C¹ plus `w > 0` gives
/// `S = A/w` C¹, which is what the Taylor certificate needs).
fn rational_cell_bounds(
    n: &NurbsSurface<f64>,
    fk: FaceKey,
) -> Result<Vec<CellRaw>, TessellateError> {
    // The convex-combination licence, on f64 STRUCTURE: every hull
    // fact below (the rational value hull, the weight-range divisor)
    // requires strictly positive finite weights. `!(w > 0.0)` catches
    // NaN. (`NurbsSurface::new` refuses these at the door; re-checked
    // here so THIS bound never divides by an unproven denominator.)
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if n.weights().iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "rational NURBS face with a non-positive or non-finite weight — an \
                   illegal rational description: the convex-combination licence every \
                   hull fact rests on requires strictly positive weights",
        });
    }
    // Fixed refinement (RATIONAL_CERT_SPLITS docs), both directions.
    let refined = n
        .refine_knots_u(&rational_split_points(n.knots_u()))
        .and_then(|r| r.refine_knots_v(&rational_split_points(r.knots_v())))
        .map_err(|_| TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "rational NURBS face whose refinement fails to materialise — \
                   outside the certified inventory",
        })?;
    let r = &refined;
    // Positivity survives insertion in ℝ (convex combinations); this
    // code may not assume floating point did (the speed meter's rule).
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    if r.weights().iter().any(|w| !(*w > 0.0) || !w.is_finite()) {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "rational NURBS face whose refined weights lost positivity — \
                   outside the certified inventory",
        });
    }
    let (kv_u, kv_v) = (r.knots_u(), r.knots_v());
    let (pu, pv) = (kv_u.degree(), kv_v.degree());
    let (nu, nv) = r.control_counts();
    // Homogeneous nets: w and A^c = w·P^c per component, as ring
    // products of the exact f64 inputs.
    let w_grid: Net = (0..nu)
        .map(|i| {
            (0..nv)
                .map(|j| RingInterval::point(r.weights()[i * nv + j]))
                .collect()
        })
        .collect();
    let comp_grid = |c: usize| -> Net {
        (0..nu)
            .map(|i| {
                (0..nv)
                    .map(|j| {
                        let p = r.control()[i * nv + j];
                        RingInterval::point(r.weights()[i * nv + j])
                            * RingInterval::point(match c {
                                0 => p.x,
                                1 => p.y,
                                _ => p.z,
                            })
                    })
                    .collect()
            })
            .collect()
    };
    // Derivative nets. Second derivatives along a degree-1 direction
    // are EXACTLY zero in ℝ for the polynomial nets A and w (the
    // direction is a single linear span pre-refinement — the C¹ gate —
    // and refinement's inserted knots are removable), so those nets
    // are `None` and their terms exact zeros; the CROSS terms
    // (`S_d·w_d`, and `w_dd` of the other direction) stay, which is
    // where a rational degree-1 direction genuinely curves in
    // parameter.
    let kv_u1 = (pu >= 2).then(|| derived_knots(kv_u, fk)).transpose()?;
    let kv_v1 = (pv >= 2).then(|| derived_knots(kv_v, fk)).transpose()?;
    struct DNets {
        d10: Net,
        d01: Net,
        d11: Net,
        d20: Option<Net>,
        d02: Option<Net>,
    }
    let build = |base: &Net| -> DNets {
        let d10 = net_d_u(kv_u, base);
        let d01 = net_d_v(kv_v, base);
        let d11 = net_d_v(kv_v, &d10);
        let d20 = kv_u1.as_ref().map(|k1| net_d_u(k1, &d10));
        let d02 = kv_v1.as_ref().map(|k1| net_d_v(k1, &d01));
        DNets {
            d10,
            d01,
            d11,
            d20,
            d02,
        }
    };
    let w_nets = build(&w_grid);
    let a_nets: Vec<(Net, DNets)> = (0..3)
        .map(|c| {
            let g = comp_grid(c);
            let d = build(&g);
            (g, d)
        })
        .collect();
    // Per-cell assembly. The caller either maxes these (the shipped
    // bound — `rational_face_bound`) or keeps them apart (the sizing
    // diagnostic — `nurbs_cell_bounds`).
    let mut cells: Vec<CellRaw> = Vec::new();
    let two = RingInterval::point(2.0);
    for su in kv_u.first_span()..=kv_u.last_span() {
        for sv in kv_v.first_span()..=kv_v.last_span() {
            // Emptiness skip, span validation and window construction
            // in one operation, both directions. The `checked_sub`
            // pair this replaces — and its
            // `MissingEntity { what: "NURBS span below its degree" }`
            // return — are unrepresentable now: `iu0`/`jv0` ARE
            // `first_control()`, subtracted once inside `Span`'s
            // invariant.
            let Some(win) = r.window(su, sv) else {
                continue;
            };
            let (span_u, span_v) = (win.span_u(), win.span_v());
            // Active windows on span (su, sv): value indices
            // [su−p, su]; each u/v differencing drops the top index —
            // which is `derived_window`, so `su − 1` and `su − 2` are
            // no longer subtractions at the use site either. The
            // order-2 windows are `None` exactly when their derived
            // NETS are (degree < 2), so the two `Option`s are zipped
            // rather than independently discharged.
            let wu_val = span_u.window();
            let wv_val = span_v.window();
            let wu_d1 = span_u.first_derived_window();
            let wv_d1 = span_v.first_derived_window();
            let wu_d2 = span_u.derived_window(2);
            let wv_d2 = span_v.derived_window(2);
            // The cell centroid — a translation CHOICE (any finite c
            // is sound), computed on f64 structure, fixed order.
            let mut csum = [0.0f64; 3];
            let mut count = 0.0f64;
            for i in 0..=span_u.degree() {
                let row = win.row(i);
                for j in 0..=span_v.degree() {
                    let p = r.control()[row + j];
                    csum[0] += p.x;
                    csum[1] += p.y;
                    csum[2] += p.z;
                    count += 1.0;
                }
            }
            let c = [csum[0] / count, csum[1] / count, csum[2] / count];
            // The cell's weight range — the divisor (module docs: for
            // a SUP bound the divisor is w_min; the interval division
            // by [w_lo, w_hi] takes exactly num_sup/w_lo for the
            // nonnegative numerators below, outward-rounded, and
            // poisons if positivity was never proven).
            let mut w_cell: Option<RingInterval> = None;
            for row in &w_grid[wu_val.clone()] {
                for wv in &row[wv_val.clone()] {
                    w_cell = Some(match w_cell {
                        None => *wv,
                        Some(h) => RingInterval::hull(h, *wv),
                    });
                }
            }
            let w_cell = w_cell.unwrap_or_else(RingInterval::poison);
            let zero = RingInterval::zero();
            // Weight-net magnitude sups on the cell.
            let w10 = mag_iv(window_tilde_hull(
                &w_nets.d10,
                &w_nets.d10,
                zero,
                &wu_d1,
                &wv_val,
            ));
            let w01 = mag_iv(window_tilde_hull(
                &w_nets.d01,
                &w_nets.d01,
                zero,
                &wu_val,
                &wv_d1,
            ));
            let w11 = mag_iv(window_tilde_hull(
                &w_nets.d11,
                &w_nets.d11,
                zero,
                &wu_d1,
                &wv_d1,
            ));
            let w20 = w_nets
                .d20
                .as_ref()
                .zip(wu_d2.as_ref())
                .map_or_else(RingInterval::zero, |(d, wu2)| {
                    mag_iv(window_tilde_hull(d, d, zero, wu2, &wv_val))
                });
            let w02 = w_nets
                .d02
                .as_ref()
                .zip(wv_d2.as_ref())
                .map_or_else(RingInterval::zero, |(d, wv2)| {
                    mag_iv(window_tilde_hull(d, d, zero, &wu_val, wv2))
                });
            let (mut cuu, mut cuv, mut cvv) = (
                RingInterval::zero(),
                RingInterval::zero(),
                RingInterval::zero(),
            );
            for (comp, (_, a)) in a_nets.iter().enumerate() {
                let cc = RingInterval::point(c[comp]);
                // sup|S^c − c^c| on the cell: the rational value hull
                // (positive weights ⇒ nonnegative partition of unity
                // over the ACTIVE control points).
                let mut v0h: Option<RingInterval> = None;
                for i in 0..=span_u.degree() {
                    let row = win.row(i);
                    for j in 0..=span_v.degree() {
                        let p = r.control()[row + j];
                        let e = RingInterval::point(match comp {
                            0 => p.x,
                            1 => p.y,
                            _ => p.z,
                        }) - cc;
                        v0h = Some(match v0h {
                            None => e,
                            Some(h) => RingInterval::hull(h, e),
                        });
                    }
                }
                let v0 = mag_iv(v0h.unwrap_or_else(RingInterval::poison));
                // Recentred homogeneous derivative sups Ã_kl = A_kl −
                // c·w_kl on the cell.
                let a10 = mag_iv(window_tilde_hull(&a.d10, &w_nets.d10, cc, &wu_d1, &wv_val));
                let a01 = mag_iv(window_tilde_hull(&a.d01, &w_nets.d01, cc, &wu_val, &wv_d1));
                let a11 = mag_iv(window_tilde_hull(&a.d11, &w_nets.d11, cc, &wu_d1, &wv_d1));
                let a20 = match (a.d20.as_ref(), w_nets.d20.as_ref(), wu_d2.as_ref()) {
                    (Some(ad), Some(wd), Some(wu2)) => {
                        mag_iv(window_tilde_hull(ad, wd, cc, wu2, &wv_val))
                    }
                    _ => RingInterval::zero(),
                };
                let a02 = match (a.d02.as_ref(), w_nets.d02.as_ref(), wv_d2.as_ref()) {
                    (Some(ad), Some(wd), Some(wv2)) => {
                        mag_iv(window_tilde_hull(ad, wd, cc, &wu_val, wv2))
                    }
                    _ => RingInterval::zero(),
                };
                // The quotient-rule recurrences (module docs), each
                // division by the cell weight range.
                let s1u = (a10 + v0 * w10) / w_cell;
                let s1v = (a01 + v0 * w01) / w_cell;
                let suu = (a20 + two * s1u * w10 + v0 * w20) / w_cell;
                let svv = (a02 + two * s1v * w01 + v0 * w02) / w_cell;
                let suv = (a11 + s1u * w01 + s1v * w10 + v0 * w11) / w_cell;
                cuu = cuu + suu.sqr();
                cuv = cuv + suv.sqr();
                cvv = cvv + svv.sqr();
            }
            cells.push(CellRaw {
                u: span_extent(kv_u, su),
                v: span_extent(kv_v, sv),
                sq_uu: cuu,
                sq_uv: cuv,
                sq_vv: cvv,
            });
        }
    }
    Ok(cells)
}

/// The C¹ gate per direction (module docs): degree 0 refuses; degree 1
/// must be single-span (an interior knot is a C⁰ crease); degree ≥ 2
/// needs interior multiplicities ≤ p − 1 — the chord pass's carrier
/// gate, verbatim on the surface's knot vectors.
fn check_direction(kv: &KnotVector, fk: FaceKey) -> Result<(), TessellateError> {
    let p = kv.degree();
    if p == 0 {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "degree-0 NURBS direction (a degenerate face description)",
        });
    }
    if p == 1 {
        if kv.interior_knots().next().is_none() {
            return Ok(());
        }
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "degree-1 NURBS direction with interior knots (a C⁰ crease) — \
                   the interpolation Taylor bound needs C¹; split the face at \
                   the crease",
        });
    }
    // C¹ needs every interior multiplicity ≤ p − 1 (p ≥ 2 here).
    if kv.interior_knots().any(|(_, m)| m > p - 1) {
        return Err(TessellateError::UnsupportedNurbsFace {
            face: fk,
            note: "NURBS direction with a C⁰ crease (interior multiplicity = \
                   degree) — the interpolation Taylor bound needs C¹; split \
                   the face at the crease",
        });
    }
    Ok(())
}

/// Hull of ALL second-derivative coefficients along one direction:
/// each `rows[k]` is the coefficient row of one fixed cross-direction
/// index, differenced twice against `kv` (module docs). A degree-1
/// single-span direction answers the exact zero.
fn second_derivative_hull(
    kv: &KnotVector,
    rows: &[Vec<RingInterval>],
    fk: FaceKey,
) -> Result<RingInterval, TessellateError> {
    if kv.degree() < 2 {
        return Ok(RingInterval::zero());
    }
    let kv1 = derived_knots(kv, fk)?;
    let mut acc: Option<RingInterval> = None;
    for row in rows {
        let q1 = derivative_coeffs(kv, row);
        let q2 = derivative_coeffs(&kv1, &q1);
        for q in q2 {
            acc = Some(match acc {
                None => q,
                Some(a) => RingInterval::hull(a, q),
            });
        }
    }
    acc.ok_or(TessellateError::MissingEntity {
        what: "empty NURBS control net",
    })
}

/// Hull of ALL mixed-derivative coefficients: difference once along
/// `kv_u` (per `u_rows` row, i.e. per fixed v index), then once along
/// `kv_v` across the resulting net (module docs — one application per
/// direction, so only degree ≥ 1 is needed on each, which
/// [`check_direction`] guarantees).
fn mixed_derivative_hull(
    kv_u: &KnotVector,
    kv_v: &KnotVector,
    u_rows: &[Vec<RingInterval>],
) -> Result<RingInterval, TessellateError> {
    // q_u[j][i'] : derivative-in-u coefficients at v index j.
    let q_u: Vec<Vec<RingInterval>> = u_rows
        .iter()
        .map(|row| derivative_coeffs(kv_u, row))
        .collect();
    let nu1 = q_u.first().map_or(0, Vec::len);
    let nv = q_u.len();
    if nu1 == 0 || nv == 0 {
        return Err(TessellateError::MissingEntity {
            what: "empty NURBS control net",
        });
    }
    let mut acc: Option<RingInterval> = None;
    // Walked column-wise on purpose (transposing the row-major
    // intermediate net), so `i` indexes across the OUTER Vec.
    #[allow(clippy::needless_range_loop)]
    for i in 0..nu1 {
        let v_row: Vec<RingInterval> = (0..nv).map(|j| q_u[j][i]).collect();
        for q in derivative_coeffs(kv_v, &v_row) {
            acc = Some(match acc {
                None => q,
                Some(a) => RingInterval::hull(a, q),
            });
        }
    }
    acc.ok_or(TessellateError::MissingEntity {
        what: "empty NURBS control net",
    })
}

/// The once-differenced knot vector (drop the outer knot pair, degree
/// − 1) — the chord pass's construction, shared refusal shape.
fn derived_knots(kv: &KnotVector, fk: FaceKey) -> Result<KnotVector, TessellateError> {
    let inner = kv.knots()[1..kv.knots().len() - 1].to_vec();
    KnotVector::clamped(inner, kv.degree() - 1).map_err(|_| TessellateError::UnsupportedNurbsFace {
        face: fk,
        note: "NURBS direction whose derivative knot vector fails to materialise — \
               outside the certified inventory",
    })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Point3;
    use geom_core::Tol;
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
        };
        let uv = [[0.0, 0.0], [0.1, 0.0], [0.0, 0.2]];
        let q = 2.0 * 0.01 + 2.0 * 3.0 * 0.1 * 0.2 + 5.0 * 0.04;
        assert!((b.cert(uv) - 0.25 * q).abs() < 1e-15);
    }

    /// A planar bilinear quad is flat: every bound collapses to the
    /// ring's outward-rounding dust (subnormal / few-ulp scale, never
    /// claimed as exact zero — conservative is the promised
    /// direction), and both grid steps come out effectively
    /// unconstrained.
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
    /// contribute only rounding dust to uu/vv).
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
        let mk = |muu: f64, mvv: f64| NurbsFaceBound { muu, muv: 0.0, mvv };
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
    /// (malign): the band and its neighbour must snap to the patch
    /// column count, and the snap only ever adds columns.
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
                },
            },
            CellBound {
                u: (0.0, 1.0),
                v: (0.02, 1.0),
                bound: NurbsFaceBound {
                    muu: 0.1,
                    muv: 0.0,
                    mvv: 0.1,
                },
            },
        ];
        let grid = NurbsCellGrid::from_cells(&cells);
        let patch = NurbsFaceBound {
            muu: 2.0,
            muv: 0.0,
            mvv: 2.77,
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
        assert_eq!(bands[0].nvc, 2, "the fixture's nvc must be 2: {bands:?}");
        assert_eq!(
            bands[0].nuc, 45,
            "the malign band snaps to the patch column count: {bands:?}"
        );
        assert_eq!(
            bands[1].nuc, 45,
            "the malign band's neighbour snaps too: {bands:?}"
        );
        // The snap only ever ADDS columns: both own counts were below.
        assert!(bands[1].nvc >= 1);
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
    #[cfg_attr(
        not(nightly_suite),
        ignore = "nightly-only: 3.2 s of lattice falsification over rational carriers; never red in the \
     repository's entire life, and its sibling nurbs_cert rows cover the same certificate \
     on every PR at a fraction of the cost."
    )]
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
}

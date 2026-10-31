//! **The rational patch lane** ([`rational_patch_face`]): the same
//! rectangle as [`super::patch`], through the quotient statement
//! `S = A/w`, with its `w`-uniform-in-v exact arm and its composite arm.

use geom_core::Bounds;
use geom_core::interval::certification::Certification;
use geom_core::interval::{Interval, cross, dot};
use geom_core::spline::KnotVector;
use geom_core::{Band, Decide};

use super::area::{
    AREA_CELLS_PER_SPAN, AreaCell, PERIMETER_CHORDS_PER_SPAN, area_midpoint_taylor,
    boundary_chord_perimeter_lo, debug_assert_area_gauge,
};
use super::cells::{
    HULL_BLOCKS_PER_SPAN, MeasureCells, RefinedNet, advance_block, cell_count, cell_midpoint,
    knot_aligned_cuts, remainder_cell_u, remainder_cell_v, round_cuts, span_cells,
};
use super::net::{
    Collapse, GridSlice, PatchGrid, PatchNet, PatchRect, RVec3, clipped_spans, grid_slice,
    grid_slice_at, newton_cotes_weights, widen,
};
use super::run::{Accept, FaceCutBounds, LaneRun, RoundWindow, Run, face_extent};
use crate::props::PropsError;
use crate::props::bracket::{lo_or_refuse, pt};

/// Weight of one cell of the rational lane's composite arm: the unit's ten.
const W_RATIONAL_COMPOSITE: usize = 10;
/// Weight of one cell of the rational lane's exact-v arm (a u cell
/// against one v span, integrated exactly).
const W_RATIONAL_EXACT_V: usize = 58;

/// The derivative ladder of one polynomial (B-spline) net, built once.
///
/// The ladder bottoms out in `None` exactly where the polynomial
/// degree runs out, which [`grid_vec`] reads as the identically-zero
/// derivative. Two instantiations: the homogeneous position net
/// `A = w·P` (all three channels) and the weight net `w` (channel 0,
/// the other two structural zeros).
struct Ladder {
    a: PatchGrid,
    au: Option<PatchGrid>,
    av: Option<PatchGrid>,
}

impl Ladder {
    fn build(a: PatchGrid) -> Self {
        let au = a.deriv_u();
        let av = a.deriv_v();
        Self { a, au, av }
    }

    /// The surface point `S = A/w` at an exact parameter pair, read
    /// off the homogeneous net and its weight ladder.
    fn point(&self, w: &Self, u: f64, v: f64) -> RVec3 {
        let (uc, vc) = (Collapse::At(u), Collapse::At(v));
        let p = self.a.vec(uc, vc);
        let wq = w.chan(uc, vc);
        [p[0] / wq, p[1] / wq, p[2] / wq]
    }

    /// The scalar (weight-net) channel reads.
    fn chan(&self, u: Collapse<'_>, v: Collapse<'_>) -> Interval {
        self.a.channel(0, u, v)
    }
}

/// One ladder's three grids (`A`, `A_u`, `A_v`) collapsed in u at one
/// column ([`PatchGrid::slice_u`] for why), `None` where the ladder has
/// run out of degree.
type LadderSlice = [GridSlice; 3];

/// **One composite column**: the homogeneous ladder's slice and the
/// weight ladder's, the one u collapse every cell of the column reads,
/// flux and area alike.
struct ColumnSlice {
    a: LadderSlice,
    w: LadderSlice,
}

impl Ladder {
    /// The three grids' slices at one u collapse.
    fn slice(&self, u: Collapse<'_>) -> LadderSlice {
        self.grids().map(|g| grid_slice(g, u))
    }

    /// The base grid's slice alone, its derivatives left unread.
    fn slice_base(&self, u: Collapse<'_>) -> LadderSlice {
        [grid_slice(Some(&self.a), u), None, None]
    }

    /// The three grids at one v collapse of `sl`, the identically-zero
    /// derivative where the ladder has run out.
    fn slice_at(&self, sl: &LadderSlice, v: Collapse<'_>) -> [RVec3; 3] {
        let grids = self.grids();
        core::array::from_fn(|k| grid_slice_at(grids[k], sl[k].as_ref(), v))
    }

    /// The ladder's three grids, `None` where its degree ran out.
    fn grids(&self) -> [Option<&PatchGrid>; 3] {
        [Some(&self.a), self.au.as_ref(), self.av.as_ref()]
    }

    /// The base grid alone at one v collapse of `sl`.
    fn base_at(&self, sl: &LadderSlice, v: Collapse<'_>) -> RVec3 {
        match sl[0].as_ref() {
            Some(s) => self.a.at_v(s, v),
            None => unreachable!("a ladder's base grid is always sliced"),
        }
    }

    /// This ladder's and the weight ladder's slices at one u collapse:
    /// the area column, which reads the weight's derivatives.
    fn column(&self, w: &Self, u: Collapse<'_>) -> ColumnSlice {
        ColumnSlice {
            a: self.slice(u),
            w: w.slice(u),
        }
    }

    /// The flux column: the weight ladder's base grid alone, the only
    /// one the flux reads.
    fn flux_column(&self, w: &Self, u: Collapse<'_>) -> ColumnSlice {
        ColumnSlice {
            a: self.slice(u),
            w: w.slice_base(u),
        }
    }

    /// The numerator of `S_u×S_v`,
    /// `w·(A_u×A_v) − w_v·(A_u×A) − w_u·(A×A_v)`, which sits over `w³`
    /// ([`rational_patch_face`] docs), at one v collapse of a column.
    fn cross_num_at(&self, w: &Self, sl: &ColumnSlice, v: Collapse<'_>) -> RVec3 {
        let [a, au, av] = self.slice_at(&sl.a, v);
        let [w0, wu, wv] = w.slice_at(&sl.w, v).map(|g| g[0]);
        let base = cross(&au, &av);
        let ta = cross(&au, &a);
        let tb = cross(&a, &av);
        [
            w0 * base[0] - wv * ta[0] - wu * tb[0],
            w0 * base[1] - wv * ta[1] - wu * tb[1],
            w0 * base[2] - wv * ta[2] - wu * tb[2],
        ]
    }

    /// `N/w³` at one v collapse of a column — the composite rule's
    /// per-cell integrand.
    fn integrand_at(&self, w: &Self, sl: &ColumnSlice, v: Collapse<'_>) -> Interval {
        self.numerator_at(sl, v) / w.base_at(&sl.w, v)[0].powi(3)
    }

    /// `A·(A_u×A_v)` at one v collapse of a column — the flux numerator
    /// alone, for the arm that divides by `w³` once per column instead
    /// of once per node.
    fn numerator_at(&self, sl: &ColumnSlice, v: Collapse<'_>) -> Interval {
        let [a, au, av] = self.slice_at(&sl.a, v);
        dot(&a, &cross(&au, &av))
    }

    /// **The EXACT v integral** `∫ N dv` over one v-cell, from a
    /// u-slice: closed Newton–Cotes per KNOT SPAN of the cell, which
    /// is exact because `N` is one polynomial of v-degree ≤ 3q−1 on
    /// each.
    ///
    /// `N = A·(A_u×A_v)` has v-degree `q + q + (q−1)`, so the
    /// order-`3q` rule integrates it with no truncation error at all;
    /// what the returned enclosure carries is the nodes' and weights'
    /// outward rounding and the de Boor recurrence's own widening.
    ///
    /// **The subdivision is per span and not per cell**, and that is
    /// load-bearing rather than tidy. One `Collapse::AtSpan` read
    /// evaluates a span's polynomial, and it stays exact in ℝ when the
    /// node sits outside that span — but only in ℝ. The interval de
    /// Boor widens with the distance extrapolated, and it widens
    /// without bound when the refinement has left a hairline span
    /// nearby, because the recurrence divides by knot differences.
    /// Integrating span by span never extrapolates: every node lies in
    /// the span it is read through.
    fn num_v_exact(
        &self,
        sl: &ColumnSlice,
        kv: &KnotVector,
        vlo: f64,
        vhi: f64,
        nc: &[Interval],
    ) -> Interval {
        let m = nc.len() - 1;
        let mut total = Interval::zero();
        for (a, b, mid) in clipped_spans(kv, vlo, vhi) {
            let scale = pt(b) - pt(a);
            let mut acc = Interval::zero();
            for (j, wj) in nc.iter().enumerate() {
                #[allow(clippy::cast_precision_loss)]
                let t = pt(a) + scale * (pt(j as f64) / pt(m as f64));
                acc = acc + *wj * self.numerator_at(sl, Collapse::AtSpan { mid, t: &t });
            }
            total = total + scale * acc;
        }
        total
    }
}

/// The work of the lane's round `round` on `rect`: its cells
/// ([`round_cuts`]) at the weight of the arm `exact_v` picks.
pub(super) fn round_cost(
    rect: PatchRect,
    knots: (&[f64], &[f64]),
    exact_v: bool,
    round: usize,
) -> usize {
    let weight = if exact_v {
        W_RATIONAL_EXACT_V
    } else {
        W_RATIONAL_COMPOSITE
    };
    weight * cell_count(&round_cuts(rect, knots, exact_v, round))
}

/// **Certified RATIONAL patch contributions** (M8-3) — the same two
/// numbers [`nurbs_patch_face`](super::patch::nurbs_patch_face) certifies, for a patch whose weights
/// are not all 1.
///
/// # The enclosure, derived
///
/// Write the patch as the quotient of its homogeneous nets,
/// `S = A/w` with `A = Σ N_i N_j w_ij P_ij` and `w = Σ N_i N_j w_ij`
/// — **both polynomial**, so both carry the control-hull convexity
/// facts the integral lane already runs on. The quotient rule gives
///
/// ```text
/// S_u×S_v = [w·(A_u×A_v) − w_v·(A_u×A) − w_u·(A×A_v)] / w³
/// ```
///
/// (the `A×A` term vanishes), and dotting with `S = A/w` kills the two
/// triple products that carry `A` twice, leaving the **flux integrand
/// as one polynomial over one polynomial cube**:
///
/// ```text
/// f = S·(S_u×S_v) = A·(A_u×A_v) / w³ = N / w³
/// ```
///
/// `N` is the integral lane's integrand on the homogeneous net. Weights
/// are `f64` structure and strictly positive (checked exactly, C6), so
/// `w`'s control hull excludes zero on every cell and interval
/// arithmetic division is defined; a hull that does not is refused,
/// and the refusal is typed rather than a wide answer.
///
/// # The rule, the remainder, and why there is no exact lane
///
/// A rational integrand has **no finite exact quadrature rule** — the
/// integral-lane's per-span Newton–Cotes shortcut has no rational
/// counterpart and none is claimed. The honest deliverable is the
/// composite midpoint enclosure over a per-span cut schedule (pieces
/// per knot span double per round from
/// [`SPAN_INIT_PIECES`](super::cells::SPAN_INIT_PIECES); which round the
/// loop stops at is decided by the caller's acceptance and by the lane's
/// floor and work wall ([`Run`]), functions of the face and the caller
/// alone), with the two 1-D Taylor remainders per cell:
///
/// ```text
/// ∫∫_cell f ∈ A_cell·f(m) + hull(f_uu)·h_u³h_v/24 + hull(f_vv)·h_u h_v³/24
/// ```
///
/// The `f_dd` hulls are the quotient statement's
/// ([`geom_core::spline::cell_measure_quotients`]): `f_dd` is one
/// polynomial over `w⁵`, read per hull block as a convex combination of
/// its coefficient ratios on the block ([`MeasureCells`]); the area
/// pad's `∂_d(S_u×S_v)` is one polynomial over `w⁴`, read the same way
/// per area cell.
///
/// **The cells are cut ON the interior knots**
/// ([`knot_aligned_cuts`]), which is what makes that ONE rule the
/// whole rule: the Taylor remainder needs `f` to be twice
/// differentiable across the cell, and at an interior knot it need not
/// even be continuous. A uniform cut has to fall back to the
/// smoothness-free `A_cell·hull(f)` wherever a knot lands inside a
/// cell, and that hull is a control-net fact — span-granular, so it
/// stops shrinking once cells are finer than a span. Each straddling
/// knot line then contributes Θ(1/pieces): it halves per round while
/// the cell count quadruples, so the enclosure has a FLOOR and the
/// fixed schedule cannot reach a target below it. Aligning the cuts
/// removes the case instead of bounding it — a knot on a cell boundary
/// is in no cell's interior — and the enclosure is O(h²) throughout.
/// (The refinement to [`REFINED_PIECES_PER_SPAN`](super::cells::REFINED_PIECES_PER_SPAN) does not do this on its
/// own: inserted knots do not change smoothness, so the ORIGINAL knot
/// vectors are what the cuts are taken from.) Whether a round is enough
/// is the caller's question ([`Accept`]), and the face-extent gate
/// `props_quad_face_extent` is asked of the round it accepts. A call the
/// caller does not accept ends at the lane's floor or its work wall and
/// says which ([`LaneEnd`](super::run::LaneEnd)); a wide answer is never returned silently.
///
/// # The practical envelope (measured; every row re-measured)
///
/// The rule is O(h²) and the meter is a **LENGTH**, so the achievable
/// width scales roughly linearly with part size while the target does
/// not: the frontier is a part-SIZE frontier. The lane's stops do not
/// move with ε, so each carrier stops at the same displacement on every
/// ε row and only the `1024·ε` target moves. The width here is the one
/// the work wall stops the lane at ([`FACE_WORK_WALL`](super::run::FACE_WORK_WALL)): on a one-span
/// face, the composite arm's round 7 and the exact-v arm's round 11. It is the Taylor
/// remainder's, which quarters per round (exactly where every cell
/// splits evenly; more slowly where a cell abuts an off-grid knot); the
/// midpoint sum's own width is a small fraction of it on these carriers,
/// so none of them reaches its floor first. Measured
/// (`crates/geom-brep/tests/review_r1_rational_probes.rs`, which pins
/// each one that stops short of a row's target), against the DEFAULT
/// target of 1.024e-6 m:
///
/// ```text
/// carrier                                      width (m)   arm        @ ε = 1e-9
/// 1 µm quarter cylinder                         3.326e-17  exact-v    certifies*
/// 1 m × 2 m quarter cylinder, single span       3.404e-11  exact-v    certifies
/// unit sphere octant (degenerate pole row)      5.794e-8   composite  certifies (18×)
/// quarter torus, R = 2 m, r = 0.5 m             9.301e-8   composite  certifies (11×)
/// 1 m × 2 m half cylinder, TWO spans            1.118e-10  exact-v    certifies
/// 5 m C0-kink extruded wall                     exact      exact-v    certifies
/// 1 km quarter cylinder                         3.298e-8   exact-v    certifies (31×)
/// warped bilinear, weights 1e-1/1e1             9.882e-6   composite  STOPS SHORT
/// Möbius quarter cylinder, weight ratio 100     1.452e-8   exact-v    certifies (71×)
/// bilinear square, weights 1e-3/1e3             6.358e+5   composite  STOPS SHORT
/// ```
///
/// \* Convergence is never the small end's problem — its width scales
/// with the part. The FACE-EXTENT gate is: at ε = 1e-6 the 1 µm
/// cylinder's 0.22 µm mean width is under the run's own tolerance and
/// the same face is refused [`PropsError::DegenerateFace`].
///
/// Read that honestly: metre-scale parts converge at the default ε with
/// an order or more to spare, a kilometre-scale one only by its exact-v
/// arm's extra rounds, and a weight spread of 10² across a warped patch
/// is enough to stop a metre-scale one short. Callers on large or
/// extreme-weight parts should expect a bracket rather than a number.
///
/// The last row is the extreme of the weight spread: at 1e-3/1e3 the
/// area element itself ranges over ten orders across the square, so
/// its hull per block and the flux's `f_dd` hulls are genuinely that
/// wide. What keeps the area from straddling zero — and so from
/// refusing a plain unit square as having no extent at all — is that
/// the rule intersects the pad with the block's hull of `g`, whose
/// lower end is a magnitude and cannot be negative. The face therefore
/// certifies a positive extent and stops at the wall with a width. The
/// width is useless; saying so is the point.
///
/// A carrier that stops short says where ([`LaneEnd`](super::run::LaneEnd)) and hands back
/// its narrowest enclosure. The next levers, in order, are a
/// higher-order rule (Simpson needs `A` to fifth derivatives) and a
/// finer hull-block grid: with the cells knot-aligned the enclosure
/// quarters cleanly per round, so a carrier under a factor of four over
/// target is one round short of a wall that admits no further round.
///
/// The `w`-uniform-in-v fast path is no longer a lever — it is taken,
/// below — but what it buys is worth stating, because it is not what
/// the note proposing it expected. It makes the v integral EXACT and
/// the v cell count O(1), so it is a large COST win and it removes
/// the v direction from the error entirely; on the carriers that
/// satisfy it (cylinders, extruded and skinned walls) the curvature
/// is all in u and the v remainder was already near zero, so the
/// WIDTH barely moves. Its hypothesis is also narrower than "loft
/// walls satisfy it": geometrically they do, but a skin fit of degree
/// ≥ 2 SOLVES for its weights and the solve returns them equal only
/// to rounding. The test is exact `f64` structure (C6) — the arm's
/// exactness is a soundness hypothesis, not a tolerance — so those
/// walls take the composite arm.
///
/// # Errors
///
/// [`PropsError`] — a non-positive weight (a non-finite one never
/// becomes a [`PatchNet`]), a weight hull that does not exclude zero,
/// an escalated funnel decision, a degenerate face, or `accept`'s own
/// refusal.
#[allow(clippy::too_many_lines)] // one engine, kept whole like the integral lane
pub(super) fn rational_patch_face<T: Decide>(
    net: &PatchNet<'_>,
    rect: PatchRect,
    perimeter: f64,
    boundary_defect: f64,
    band: Band,
    window: RoundWindow,
    accept: &mut Accept<'_>,
) -> Result<LaneRun, PropsError> {
    let (u0, u1, v0, v1) = rect.corners();
    let (kv_u, kv_v) = (net.knots_u(), net.knots_v());
    // Exact `f64` structure (C6): the rational hull property IS the
    // positive-weight hypothesis, so a non-positive weight is not a
    // tolerance question but a refusal.
    if net.with_weights(|ws| ws.iter().any(|w| w.lo() <= 0.0)) {
        return Err(PropsError::QuadratureUnsupported {
            what: "a rational patch with a non-positive weight: the convex-hull \
                   property (and with it every enclosure here) needs strictly \
                   positive weights",
        });
    }
    // The homogeneous position net `A = w·P` and the weight net, each
    // refined FIRST ([`RefinedNet`]): the weight hull and the
    // position bound below are control-net facts.
    let refuse_refine = PropsError::QuadratureUnsupported {
        what: "a rational patch whose bracketed net would not refine (malformed knot \
               structure) — its weight hull and position bound have no other lever",
    };
    let a_net = RefinedNet::of(&net.map_points(|c, w| {
        let rw = pt(w);
        [rw * c[0], rw * c[1], rw * c[2]]
    }))
    .ok_or(refuse_refine.clone())?;
    let w_net = RefinedNet::of(&net.map_points(|_, w| [pt(w), Interval::zero(), Interval::zero()]))
        .ok_or(refuse_refine)?;
    let (a_ch, w_ch) = (a_net.channels(), w_net.channels());
    // The refined `v` vector, whose spans the exact-in-v arm walks.
    let r_v = w_ch.knots_v();
    let a = Ladder::build(PatchGrid::base(&a_ch));
    let w = Ladder::build(PatchGrid::base(&w_ch));

    // The CUT lists come from the ORIGINAL knot vectors: refinement's
    // inserted knots are artificial (the locus and its smoothness are
    // unchanged), so cutting on them would only cost cells. What the
    // cells must land on is where the smoothness actually breaks.
    let (knots_u, knots_v) = (kv_u.knots(), kv_v.knots());
    let measure = MeasureCells::build(net, rect)?;

    let over_all = (Collapse::Over(rect.u), Collapse::Over(rect.v));
    let g_w = w.chan(over_all.0, over_all.1);
    let g_w_lo = lo_or_refuse(g_w);
    if g_w_lo <= 0.0 || !g_w_lo.is_finite() {
        return Err(PropsError::QuadratureUnsupported {
            what: "a rational patch whose weight-function hull does not exclude zero \
                   over the trim rectangle — the quotient's enclosures are undefined",
        });
    }
    // Position magnitude bound over the rectangle: `S = A/w`, so the
    // homogeneous hull over the weight hull bounds it (1-norm
    // over-bound of the 2-norm — only an upper bound is consumed).
    let a_hull = a.a.vec(over_all.0, over_all.1);
    let p_bound = (a_hull[0] / g_w).mag() + (a_hull[1] / g_w).mag() + (a_hull[2] / g_w).mag();

    // The Taylor remainders' `f_dd` hulls, per coarse block
    // ([`HULL_BLOCKS_PER_SPAN`]) rather than one whole-rectangle hull: sound because a
    // hull over a superset contains every sub-cell's own, and tighter
    // because the statement's coefficient ratios converge on the
    // quotient's range as the region shrinks.
    //
    // The block edges are KNOT-ALIGNED for the same reason the cells
    // are ([`knot_aligned_cuts`]), and it is the same defect: `f_dd`
    // is where the integrand's smoothness actually lives, so a block
    // spanning an interior knot hulls a function that jumps inside it
    // and hands every cell in that block the jump's width. Aligning
    // the blocks keeps each hull inside one smooth piece. Built once,
    // before the rounds; the cut list at every refinement level
    // contains these edges, so each block still owns whole cells.
    let edges_u = knot_aligned_cuts(u0, u1, HULL_BLOCKS_PER_SPAN, knots_u);
    let edges_v = knot_aligned_cuts(v0, v1, HULL_BLOCKS_PER_SPAN, knots_v);
    let nbv = edges_v.len() - 1;
    let (cells_u, cells_v) = (span_cells(kv_u, &edges_u)?, span_cells(kv_v, &edges_v)?);
    let blocks = measure.block_hulls(&cells_u, &cells_v)?;

    // AREA: the shared [`area_midpoint_taylor`] rule with this lane's
    // quotient integrand `g = |S_u×S_v|`, its hull the area element's
    // norm range and its pad
    //
    //     G_d ⊇ sup |∂_d g| ≤ sup |∂_d(S_u×S_v)|
    //
    // (`| ∂_d |c| | ≤ |∂_d c|` — the magnitude is 1-Lipschitz), both
    // read off the statement on the hull block holding the cell: a bound
    // over a superset holds on every sub-cell, and one reading per block
    // rather than per cell keeps the statement's restrictions off the
    // area grid's cell count.
    let area_blocks = cells_u
        .iter()
        .flat_map(|u| cells_v.iter().map(|v| measure.area_bounds(u, v)))
        .collect::<Result<Vec<_>, _>>()?;
    // One u collapse per area column: the pass walks `u` outermost, and
    // each column's cells `v` upward, so the blocks advance in step.
    let mut column: Option<(f64, ColumnSlice)> = None;
    let (mut bu, mut bv) = (0usize, 0usize);
    let area = area_midpoint_taylor(
        rect,
        AREA_CELLS_PER_SPAN,
        boundary_defect,
        (knots_u, knots_v),
        |b| {
            if column.as_ref().is_none_or(|(u, _)| *u != b.umid) {
                column = Some((b.umid, a.column(&w, Collapse::At(b.umid))));
                bv = 0;
            }
            let Some((_, sl)) = column.as_ref() else {
                unreachable!("the column was just sliced")
            };
            let vm = Collapse::At(b.vmid);
            let cm = a.cross_num_at(&w, sl, vm);
            let wm = w.base_at(&sl.w, vm)[0];
            let g_mid = (cm[0].sqr() + cm[1].sqr() + cm[2].sqr()).sqrt() / wm.powi(3);
            advance_block(&mut bu, b.u.start(), &edges_u);
            advance_block(&mut bv, b.v.start(), &edges_v);
            let (g_u, g_v, g_hull) = area_blocks[bu * nbv + bv];
            Ok(AreaCell {
                g_mid,
                g_u,
                g_v,
                g_hull,
            })
        },
    )?;

    // **The `w`-uniform-in-v arm.** With the weights constant along v
    // the quotient's denominator leaves the v integral entirely —
    // `f = N(u,v)/w(u)³` — and `N` is a polynomial there, so the v
    // integral is EXACT per knot span: the true analogue of
    // [`patch_flux_exact`], available to the rational lane on the
    // patches that satisfy the hypothesis (loft and sweep walls, whose
    // weights come from the profile direction only, and the rational
    // cylinder walls a STEP file states the same way).
    //
    // Two things follow, and the second is the surprise. The v Taylor
    // remainder is gone, which is the tightening. And subdividing in v
    // no longer buys anything: the surviving remainder is
    // `hull(f_uu)·h_u³·h_v/24` with `hull(f_uu)` a per-BLOCK quantity,
    // so splitting a v-cell inside its block leaves the sum over that
    // block unchanged, and the exact v integrals over the pieces sum
    // to the exact integral over the whole. The v cuts are therefore
    // the block edges and the interior knots ONLY, and the cost drops
    // from `pieces²` cells to `pieces × (blocks + knots)`.
    //
    // The hypothesis is exact `f64` structure (C6), read off the
    // caller's weight net: nothing here is a tolerance question.
    let nv_in = kv_v.control_count();
    let w_uniform_in_v = nv_in > 0
        && net.with_weights(|ws| {
            ws.chunks_exact(nv_in)
                .all(|row| row.iter().all(|x| x.lo() == row[0].lo()))
        });
    let nc_v = if w_uniform_in_v {
        newton_cotes_weights(3 * r_v.degree())
    } else {
        None
    };

    let perimeter_lo = || {
        boundary_chord_perimeter_lo(
            rect.corners(),
            PERIMETER_CHORDS_PER_SPAN,
            (knots_u, knots_v),
            |cu, cv| a.point(&w, cu, cv),
        )
    };

    let exact_v = nc_v.is_some();
    // The round's cut lists, and its cost: one home for the loop and
    // the work it spends.
    let cuts = |round: usize| round_cuts(rect, (knots_u, knots_v), exact_v, round);
    let cost = |round: usize| round_cost(rect, (knots_u, knots_v), exact_v, round);
    // The hulls a cell in block `(bu, bv)` reads: its `f_uu` hull, and
    // its `f_vv` hull unless the exact-v arm carries no v remainder.
    let block_hulls = |bu: usize, bv: usize| -> (Interval, Option<Interval>) {
        let (b_uu, b_vv) = blocks[bu * nbv + bv];
        (b_uu, if exact_v { None } else { Some(b_vv) })
    };
    let mut run = Run::enter(window, &cost);
    let mut round = run.first();
    loop {
        let mut flux = Interval::zero();
        let mut removable = 0.0_f64;
        let (cuts_u, cuts_v) = cuts(round);
        let mut bu = 0usize;
        for iu in 0..cuts_u.len() - 1 {
            let (c_ulo, c_uhi) = (cuts_u[iu], cuts_u[iu + 1]);
            advance_block(&mut bu, c_ulo, &edges_u);
            // The width as an ENCLOSURE of the true cell width, not a
            // rounded float: the cells tile the rectangle exactly
            // (consecutive cells share a cut point), and the outward
            // subtraction is what keeps the tiling's arithmetic
            // enclosing too.
            let hu = pt(c_uhi) - pt(c_ulo);
            let um = cell_midpoint(c_ulo, c_uhi);
            let slice = a.flux_column(
                &w,
                Collapse::AtSpan {
                    mid: um.0,
                    t: &um.1,
                },
            );
            // `w` does not depend on v under the exact arm, so its
            // cube is a per-COLUMN quantity there.
            let w3 = nc_v
                .as_ref()
                .map(|_| w.base_at(&slice.w, Collapse::At(v0.midpoint(v1)))[0].powi(3));
            let mut bv = 0usize;
            for iv in 0..cuts_v.len() - 1 {
                let (c_vlo, c_vhi) = (cuts_v[iv], cuts_v[iv + 1]);
                advance_block(&mut bv, c_vlo, &edges_v);
                let hv = pt(c_vhi) - pt(c_vlo);
                let (b_uu, b_vv) = block_hulls(bu, bv);
                let r_uu = b_uu * remainder_cell_u(hu, hv);
                match (nc_v.as_ref(), w3, b_vv) {
                    // The exact arm: `h_u·g(u_m) + hull(g'')·h_u³/24`
                    // with `g(u) = ∫_cell f(u,·)` taken exactly and
                    // `g'' = ∫_cell f_uu ⊆ h_v·hull(f_uu)`, which is
                    // the SAME `r_uu` the midpoint arm carries.
                    (Some(nc), Some(cube), None) => {
                        removable += r_uu.width();
                        flux = flux
                            + hu * (a.num_v_exact(&slice, r_v, c_vlo, c_vhi, nc) / cube)
                            + r_uu;
                    }
                    (None, None, Some(b_vv)) => {
                        let vm = cell_midpoint(c_vlo, c_vhi);
                        let fm = a.integrand_at(
                            &w,
                            &slice,
                            Collapse::AtSpan {
                                mid: vm.0,
                                t: &vm.1,
                            },
                        );
                        let r_vv = b_vv * remainder_cell_v(hu, hv);
                        removable += r_uu.width() + r_vv.width();
                        flux = flux + hu * hv * fm + r_uu + r_vv;
                    }
                    // The exact-v arm's three witnesses — the
                    // Newton–Cotes weights, the per-column `w³`, the
                    // dropped v hull — are one decision (`nc_v`), so a
                    // mixed tuple is a kernel bug.
                    _ => unreachable!("the exact-v arm's witnesses disagree"),
                }
            }
        }
        let flux = widen(flux, boundary_defect * p_bound);
        if !flux.is_certified() || !area.is_certified() {
            return Err(PropsError::QuadratureUnsupported {
                what: "a rational patch enclosure refused (a weight hull straddling \
                       zero, or a non-finite net) — refusing rather than answering wide",
            });
        }
        let bounds = FaceCutBounds { flux, area };
        let (stop, accepted) = run.ask(round, bounds, removable, accept, || {
            face_extent::<T>(area, perimeter, band)?;
            debug_assert_area_gauge(area, perimeter, &perimeter_lo);
            Ok(())
        })?;
        if let Some(out) = run.exit(round, stop, accepted, bounds) {
            return Ok(out);
        }
        round += 1;
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Bounds;

    use super::super::cells::REFINED_PIECES_PER_SPAN;
    use super::super::net::knots_inside;
    use super::super::patch::{nurbs_patch_face, nurbs_patch_face_rounds};
    use super::super::run::{LaneEnd, mean_boundary_displacement};
    use crate::props::bracket::mid;
    use crate::props::quad::test_target::{self, bounds_of, encloses, patch, to_target};
    use geom_core::Tol;

    /// The two HONEST outcomes of an ε-coupled convergence row.
    ///
    /// The target is `REPORTING_RESOLUTION_FACTOR·ε` while the lane's floor
    /// and work wall do not scale with ε, so the same patch that
    /// certifies at ε = 1e-6 genuinely cannot at ε = 1e-12: the target
    /// is a million times tighter. A row asserting `Ok` unconditionally
    /// is ε-BLIND, not strict — it fails on the hosted ε matrix for a
    /// reason that is the lane working correctly.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum EpsPosture {
        /// The enclosure met the target — it must CONTAIN the truth.
        Certified,
        /// The lane stopped (its floor or its work wall) with an
        /// enclosure DEFINITELY wider than the run's target.
        Stopped,
    }

    /// Assert the ε-row posture of one patch outcome and return it.
    ///
    /// Every arm PINS its live signature so a row can never be green
    /// for the wrong reason:
    ///
    /// - certified ⇒ both enclosures contain their truths (this is the
    ///   arm that catches a tight-but-WRONG bracket, the M8-3 MAJOR);
    /// - stopped ⇒ the enclosure contains its truths too, and its width
    ///   really did exceed `1024·ε` for this run.
    ///
    /// Any error is an unsound outcome and panics. Nothing here widens
    /// a target, loosens a schedule, or special-cases ε into
    /// certifying.
    fn eps_posture(
        row: &str,
        out: &Result<LaneRun, PropsError>,
        truth_flux: f64,
        truth_area: f64,
        slack: f64,
    ) -> EpsPosture {
        let target = test_target::REPORTING_RESOLUTION_FACTOR * Tol::witness().get().eps;
        let run = match out {
            Ok(run) => run,
            Err(e) => panic!("{row}: unsound outcome {e:?}"),
        };
        let b = run.bounds;
        for (what, encl, truth) in [("flux", b.flux, truth_flux), ("area", b.area, truth_area)] {
            assert!(
                encl.lo() - slack <= truth && truth <= encl.hi() + slack,
                "{row}: {what} {encl:?} EXCLUDES the truth {truth}"
            );
        }
        match run.end {
            LaneEnd::Accepted(_) => EpsPosture::Certified,
            LaneEnd::Floor | LaneEnd::Wall => {
                let width_len = mean_boundary_displacement(b.flux, b.area).unwrap();
                assert!(
                    width_len > target,
                    "{row}: a stopped lane must carry a width that really missed: \
                     {width_len:e} vs {target:e}"
                );
                EpsPosture::Stopped
            }
            LaneEnd::Window => panic!("{row}: a whole-schedule call ended at a window: {run:?}"),
        }
    }

    /// **The M8-3 flip** of `rational_patch_refuses_typed`: the same
    /// patch — the unit square in the `z = 1` plane, reparameterized
    /// by a non-unit corner weight — now certifies an ENCLOSURE, and
    /// the re-derivation is that the rational bilinear map is a
    /// diffeomorphism of the square onto itself (corners fixed, edges
    /// to edges, positive Jacobian), so the reparameterization cannot
    /// move either number: flux `= ∫∫ 1·J = |Ω| = 1` and area `= 1`,
    /// exactly what the weight-1 patch gives.
    #[test]
    fn rational_patch_encloses_the_reparameterized_square() {
        let band = Band::linear(Tol::witness()).unwrap();
        let kv = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        let flat = [
            p(0.0, 0.0, 1.0),
            p(0.0, 1.0, 1.0),
            p(1.0, 0.0, 1.0),
            p(1.0, 1.0, 1.0),
        ];
        let out = nurbs_patch_face::<f64>(
            &patch(&kv, &kv, &flat, &[1.0, 2.0, 1.0, 1.0]),
            (0.0, 1.0, 0.0, 1.0),
            4.0,
            0.0,
            band,
            &mut to_target(Tol::witness().get().eps),
        );
        let posture = eps_posture("reparameterized square", &out, 1.0, 1.0, 0.0);
        eprintln!(
            "EPS-ROW reparameterized square @ eps={:e}: {posture:?}",
            Tol::witness().get().eps
        );
    }

    /// The SAME quarter cylinder built from TWO 45° sub-arcs (an
    /// interior double knot): the multi-span structure an arc wider
    /// than the skin's `MAX_SUB_ARC` has. A quarter arc itself is one
    /// span there (C6's η).
    #[test]
    fn rational_two_span_quarter_cylinder() {
        let band = Band::linear(Tol::witness()).unwrap();
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        let h = 2.0;
        let d = core::f64::consts::FRAC_PI_4;
        let hw = (d / 2.0).cos();
        let on = |a: f64| (a.cos(), a.sin());
        let tg = |a: f64| (a.cos() / hw, a.sin() / hw);
        let mut net = Vec::new();
        let mut weights = Vec::new();
        let pts: Vec<((f64, f64), f64)> = vec![
            (on(0.0), 1.0),
            (tg(d / 2.0), hw),
            (on(d), 1.0),
            (tg(1.5 * d), hw),
            (on(2.0 * d), 1.0),
        ];
        for ((x, y), w) in pts {
            net.push(p(x, y, 0.0));
            net.push(p(x, y, h));
            weights.push(w);
            weights.push(w);
        }
        let out = nurbs_patch_face::<f64>(
            &patch(&kv_u, &kv_v, &net, &weights),
            (0.0, 1.0, 0.0, 1.0),
            2.0f64.mul_add(2.0, core::f64::consts::PI),
            0.0,
            band,
            &mut to_target(Tol::witness().get().eps),
        );
        let truth = core::f64::consts::PI;
        let posture = eps_posture("two-span quarter cylinder", &out, truth, truth, 0.0);
        eprintln!(
            "EPS-ROW two-span quarter cylinder @ eps={:e}: {posture:?}",
            Tol::witness().get().eps
        );
    }

    /// **The multiplicity ladder** (#313, the MAJOR's regression row).
    /// One degree-3 wall, one interior knot at `u = 0.5`, walked over
    /// every multiplicity the clamped invariant admits — `1..degree+1`,
    /// i.e. C² down to the C⁰ kink — through BOTH lanes (non-unit
    /// weights → rational, the unit-weight twin → integral).
    ///
    /// `KnotVector::derivative` cannot represent the derivative knot vector from
    /// multiplicity `degree` upward; before the fix that fell through
    /// to a per-span CONSTANT first derivative, and the lane returned a
    /// thin enclosure that EXCLUDED the truth. The oracle here is
    /// independent — `geom_core`'s Book-A2.3 basis ladder and the
    /// quotient rule, not `net.rs`'s de Boor — and the standing claim
    /// is the one the MAJOR broke: **an answer always contains the
    /// truth; the alternative is a typed refusal, never a wrong
    /// bracket.**
    #[test]
    #[allow(clippy::too_many_lines)] // the ladder plus its own oracle
    fn interior_multiplicity_ladder_never_certifies_a_wrong_enclosure() {
        use geom_core::spline::basis::ders_basis_funs;
        let band = Band::linear(Tol::witness()).unwrap();
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        // `pv` is gone: the oracle's v-window base is the `Span`'s own
        // `first_control()`, not a re-derived `span − degree`.
        let (pu, nv, height) = (3usize, 2usize, 2.0f64);
        let mut postures: Vec<(String, EpsPosture)> = Vec::new();
        for mult in 1..=pu {
            let mut knots = vec![0.0; pu + 1];
            knots.extend(core::iter::repeat_n(0.5, mult));
            knots.extend(core::iter::repeat_n(1.0, pu + 1));
            let kv_u = KnotVector::clamped(knots, pu).unwrap();
            let count = kv_u.control_count();
            // A curved wall: a flaring quarter-turn profile extruded.
            #[allow(clippy::cast_precision_loss)]
            let profile: Vec<(f64, f64)> = (0..count)
                .map(|i| {
                    let s = i as f64 / (count - 1) as f64;
                    let (a, r) = (s * core::f64::consts::FRAC_PI_2, 0.2f64.mul_add(s, 1.0));
                    (r * a.cos(), r * a.sin())
                })
                .collect();
            #[allow(clippy::cast_precision_loss)]
            let profile_w: Vec<f64> = (0..count)
                .map(|i| 0.06f64.mul_add((i % 3) as f64, 1.0))
                .collect();
            for rational in [false, true] {
                let mut net_f: Vec<[f64; 3]> = Vec::with_capacity(count * nv);
                let mut ws: Vec<f64> = Vec::with_capacity(count * nv);
                for (i, (x, y)) in profile.iter().enumerate() {
                    for z in [0.0, height] {
                        net_f.push([*x, *y, z]);
                        ws.push(if rational { profile_w[i] } else { 1.0 });
                    }
                }
                let net: Vec<RVec3> = net_f
                    .iter()
                    .map(|q| [pt(q[0]), pt(q[1]), pt(q[2])])
                    .collect();
                // The INDEPENDENT oracle: S = A/W through geom-core's
                // basis ladder, S_d by the quotient rule.
                let at = |u: f64, v: f64| -> ([f64; 3], [f64; 3], [f64; 3]) {
                    let (su, sv) = (kv_u.span_at(u).unwrap(), kv_v.span_at(v).unwrap());
                    let bu = ders_basis_funs::<f64>(su, u, 1);
                    let bv = ders_basis_funs::<f64>(sv, v, 1);
                    // The `iu * nv + iv` stride stays written out on
                    // purpose: this oracle shares NO derivation with
                    // the code under test, so it does not borrow the
                    // surface window type.
                    let (mut a, mut w) = ([[0.0f64; 3]; 3], [0.0f64; 3]);
                    for (r, (nu0, nu1)) in bu[0].iter().zip(&bu[1]).enumerate() {
                        for (s, (nv0, nv1)) in bv[0].iter().zip(&bv[1]).enumerate() {
                            let idx = (su.first_control() + r) * nv + (sv.first_control() + s);
                            let (ww, q) = (ws[idx], net_f[idx]);
                            let b = [nu0 * nv0, nu1 * nv0, nu0 * nv1];
                            for k in 0..3 {
                                w[k] += b[k] * ww;
                                for (c, aq) in a[k].iter_mut().enumerate() {
                                    *aq += b[k] * ww * q[c];
                                }
                            }
                        }
                    }
                    let quot = |k: usize| -> [f64; 3] {
                        core::array::from_fn(|c| (a[k][c] * w[0] - a[0][c] * w[k]) / (w[0] * w[0]))
                    };
                    (core::array::from_fn(|c| a[0][c] / w[0]), quot(1), quot(2))
                };
                // WHY THE V-GRID IS 4 AND THE U-GRID IS 2048. Not a
                // budget judgement: on THIS net both integrands below
                // are EXACTLY independent of v, so the v-sum adds N
                // copies of one value and divides by N. Two premises,
                // both visible in the construction above — check them
                // before touching the net:
                //   (i)  `kv_v` is `unit_segment(core::num::NonZeroUsize::MIN)`: degree 1, two
                //        control points, no interior knot;
                //   (ii) the weight pushed for BOTH v-rows of profile
                //        point `i` is the same `ws[i]` — the weight is
                //        indexed by the PROFILE index only, so the two
                //        v-rows carry EQUAL weights.
                // Through `at()` those give
                //   W = Σ_r N_r(u)·w_r·Σ_s N_s(v) = Σ_r N_r(u)·w_r
                //       (the v basis is a partition of unity), so `w[0]`
                //       is v-free and `w[2] = Σ_r N_r w_r · Σ_s N'_s`
                //       = 0 — the denominator has NO v-derivative;
                //   S   = (X(u), Y(u), height·v) — x,y repeat down each
                //       v-row, and in z the shared W cancels against
                //       Σ_s N_s(v)·z_s = height·v;
                //   S_u = (X_u, Y_u, 0) — the z component is
                //       (w[1]·h·v·w[0] − w[0]·h·v·w[1])/w[0]² ≡ 0;
                //   S_v = (0, 0, height) — w[2] = 0 leaves a[2]/w[0],
                //       and a[2] vanishes in x,y.
                // So cross = S_u × S_v = (Y_u·h, −X_u·h, 0), and
                //   S·cross = h·(X·Y_u − Y·X_u)  — S's v-dependent z
                //             multiplies cross_z = 0 and drops out;
                //   |cross| = h·√(X_u² + Y_u²).
                // Both are functions of u alone. MEASURED: 64 → 4 moves
                // the oracle by at most 2.3e-13 absolute on these O(3.5)
                // values (pure summation-order rounding) against the
                // 1e-5 slack below — 4e7x of headroom — while cutting
                // this row's runtime from 78 s to 53 s.
                //
                // The 2048 u-samples are the real content and STAY: u is
                // the O(h²) direction the slack is actually sized for.
                let (nu_s, nv_s) = (2048usize, 4usize);
                #[allow(clippy::cast_precision_loss)]
                let (hu, hv) = (1.0 / nu_s as f64, 1.0 / nv_s as f64);
                let (mut o_flux, mut o_area) = (0.0f64, 0.0f64);
                for i in 0..nu_s {
                    for j in 0..nv_s {
                        #[allow(clippy::cast_precision_loss)]
                        let (u, v) = ((i as f64 + 0.5) * hu, (j as f64 + 0.5) * hv);
                        let (s, s_u, s_v) = at(u, v);
                        let cr = [
                            s_u[1] * s_v[2] - s_u[2] * s_v[1],
                            s_u[2] * s_v[0] - s_u[0] * s_v[2],
                            s_u[0] * s_v[1] - s_u[1] * s_v[0],
                        ];
                        o_flux += (s[0] * cr[0] + s[1] * cr[1] + s[2] * cr[2]) * hu * hv;
                        o_area += (cr[0].powi(2) + cr[1].powi(2) + cr[2].powi(2)).sqrt() * hu * hv;
                    }
                }
                let lane = if rational { "rational" } else { "integral" };
                let row = format!("mult {mult} / {lane}");
                let out = nurbs_patch_face::<f64>(
                    &patch(&kv_u, &kv_v, &net, &ws),
                    (0.0, 1.0, 0.0, 1.0),
                    10.0,
                    0.0,
                    band,
                    &mut to_target(Tol::witness().get().eps),
                );
                // The slack is the ORACLE's discretization error, not
                // the lane's: on the polynomial rows the exact
                // Newton–Cotes lane returns the truth to rounding and
                // the dense midpoint sum lands 2.6e-7 off it (O(h²) at
                // 2048 u-samples, measured). 1e-5 is 40x that headroom
                // and still ~900x tighter than the exclusion the MAJOR
                // produced.
                postures.push((row.clone(), eps_posture(&row, &out, o_flux, o_area, 1e-5)));
            }
        }
        eprintln!(
            "EPS-ROW multiplicity ladder @ eps={:e}: {postures:?}",
            Tol::witness().get().eps
        );
        // Anti-vacuity. The INTEGRAL rows ride the exact per-span
        // Newton–Cotes lane, whose width is outward rounding only —
        // ε-independent. The RATIONAL rows ride the exact-v arm's O(h²)
        // composite in u against an ε-coupled target, and every one
        // certifies down to ε = 1e-12, the C⁰ kink's at its last
        // round under the work wall. A posture that moves is a finding.
        let certified: Vec<&str> = postures
            .iter()
            .filter(|(_, p)| *p == EpsPosture::Certified)
            .map(|(r, _)| r.as_str())
            .collect();
        let expected: Vec<String> = (1..=pu)
            .flat_map(|mult| {
                [
                    format!("mult {mult} / integral"),
                    format!("mult {mult} / rational"),
                ]
            })
            .collect();
        assert_eq!(
            certified,
            expected,
            "the ladder's ε posture MOVED at eps={:e}: all {postures:?}",
            Tol::witness().get().eps
        );
    }

    /// A **genuinely curved** rational patch against a closed-form
    /// oracle: the quarter cylinder `r = 1`, height 2, as the standard
    /// rational-quadratic arc (weights `1, √2/2, 1`) crossed with a
    /// linear height. On it `S·(S_u×S_v) = (dθ/du)(dz/dv)`, so the
    /// flux is `∫dθ∫dz = (π/2)·2 = π` and the area is
    /// `r·(π/2)·2 = π` — both independent of the parameterization.
    #[test]
    fn rational_quarter_cylinder_brackets_the_closed_form() {
        let band = Band::linear(Tol::witness()).unwrap();
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        let h = 2.0;
        let net = [
            p(1.0, 0.0, 0.0),
            p(1.0, 0.0, h),
            p(1.0, 1.0, 0.0),
            p(1.0, 1.0, h),
            p(0.0, 1.0, 0.0),
            p(0.0, 1.0, h),
        ];
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let weights = [1.0, 1.0, w, w, 1.0, 1.0];
        let out = nurbs_patch_face::<f64>(
            &patch(&kv_u, &kv_v, &net, &weights),
            (0.0, 1.0, 0.0, 1.0),
            2.0f64.mul_add(2.0, core::f64::consts::PI),
            0.0,
            band,
            &mut to_target(Tol::witness().get().eps),
        );
        let truth = core::f64::consts::PI;
        let posture = eps_posture("quarter cylinder", &out, truth, truth, 0.0);
        eprintln!(
            "EPS-ROW quarter cylinder @ eps={:e}: {posture:?}",
            Tol::witness().get().eps
        );
    }

    /// A rational carrier: `(name, kv_u, kv_v, net, weights)`.
    type Carrier = (&'static str, KnotVector, KnotVector, Vec<RVec3>, Vec<f64>);

    /// The rational lane's carriers:
    /// the quarter cylinder in one span and in two, the quarter torus,
    /// and the cubic × linear wall at the origin and 100 m out.
    fn rational_carriers() -> Vec<Carrier> {
        let p = |x: f64, y: f64, z: f64| [pt(x), pt(y), pt(z)];
        let w2 = core::f64::consts::FRAC_1_SQRT_2;
        let linear = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let quadratic = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
        let mut out = Vec::new();
        let ring = [(1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        out.push((
            "quarter cylinder",
            quadratic.clone(),
            linear.clone(),
            ring.iter()
                .flat_map(|(x, y)| [p(*x, *y, 0.0), p(*x, *y, 2.0)])
                .collect(),
            vec![1.0, 1.0, w2, w2, 1.0, 1.0],
        ));
        let d = core::f64::consts::FRAC_PI_4;
        let hw = (d / 2.0).cos();
        let (on, tg) = (
            |a: f64| (a.cos(), a.sin(), 1.0),
            |a: f64| (a.cos() / hw, a.sin() / hw, hw),
        );
        let two = [on(0.0), tg(d / 2.0), on(d), tg(1.5 * d), on(2.0 * d)];
        out.push((
            "two-span quarter cylinder",
            KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap(),
            linear.clone(),
            two.iter()
                .flat_map(|(x, y, _)| [p(*x, *y, 0.0), p(*x, *y, 2.0)])
                .collect(),
            two.iter().flat_map(|(_, _, w)| [*w, *w]).collect(),
        ));
        let (rr, r) = (2.0, 0.5);
        let prof = [(rr + r, 0.0), (rr + r, r), (rr, r)];
        let (mut net, mut weights) = (Vec::new(), Vec::new());
        for (k, (x, z)) in prof.iter().enumerate() {
            for (pp, wj) in [
                (p(*x, 0.0, *z), 1.0),
                (p(*x, *x, *z), w2),
                (p(0.0, *x, *z), 1.0),
            ] {
                net.push(pp);
                weights.push([1.0, w2, 1.0][k] * wj);
            }
        }
        out.push(("quarter torus", quadratic.clone(), quadratic, net, weights));
        let cubic = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        for (name, t) in [("rational wall", 0.0), ("rational wall at 100 m", 100.0)] {
            let net = [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)]
                .iter()
                .flat_map(|(x, y)| [p(x + t, y + t, t), p(x + t, y + t, 0.8 + t)])
                .collect();
            let weights = vec![1.0, 1.0, 1.8, 1.8, 0.7, 0.7, 1.0, 1.0];
            out.push((name, cubic.clone(), linear.clone(), net, weights));
        }
        out
    }

    /// **The statement's `f_dd` hulls and area bounds hold the sampled
    /// integrand on every rational carrier** — red if either ever
    /// under-encloses. The truth is the lane's own point evaluation on
    /// the caller's net (`N/w³` and `|S_u×S_v|` through [`Ladder`]),
    /// differenced: a central second difference against each hull block's
    /// `f_uu` and `f_vv`, a central first difference of `g` against each
    /// block's area pads, and `g` itself against its area hull, on a
    /// 3 × 3 sample grid per block. Each check allows 1e-4 of its
    /// bound's magnitude for the difference's truncation, and the
    /// difference's rounding (`1e-13·|f|/h²`, `1e-10·|g|`) beside it.
    /// It guards containment only: a bound that widens and still holds
    /// is the floors' pins' to catch (`review_r1_rational_probes`).
    #[test]
    fn the_statements_hulls_and_pads_hold_the_sampled_integrand_on_every_rational_carrier() {
        for (name, ku, kv, control, weights) in rational_carriers() {
            let rect = PatchRect::new((0.0, 1.0, 0.0, 1.0)).unwrap();
            let net = patch(&ku, &kv, &control, &weights);
            let measure = MeasureCells::build(&net, rect).unwrap();
            let inner =
                |k: &KnotVector| -> Vec<f64> { knots_inside(k.knots(), 0.0, 1.0).collect() };
            let (knots_u, knots_v) = (inner(&ku), inner(&kv));
            let a = Ladder::build(PatchGrid::base(&net.map_points(|c, w| {
                let rw = pt(w);
                [rw * c[0], rw * c[1], rw * c[2]]
            })));
            let w = Ladder::build(PatchGrid::base(
                &net.map_points(|_, w| [pt(w), Interval::zero(), Interval::zero()]),
            ));
            let f = |u: f64, v: f64| {
                mid(a.integrand_at(&w, &a.column(&w, Collapse::At(u)), Collapse::At(v)))
            };
            let g = |u: f64, v: f64| {
                let (cu, cv) = (Collapse::At(u), Collapse::At(v));
                let c = a.cross_num_at(&w, &a.column(&w, cu), cv).map(mid);
                (c[0] * c[0] + c[1] * c[1] + c[2] * c[2]).sqrt() / mid(w.chan(cu, cv)).powi(3)
            };
            let inside = |x: f64, hull: Interval, noise: f64| {
                let slack = 1e-4 * hull.mag() + noise;
                hull.lo() - slack <= x && x <= hull.hi() + slack
            };
            let samples =
                |lo: f64, hi: f64| (0..3).map(move |i| lo + (hi - lo) * (f64::from(i) + 0.5) / 3.0);
            let cells = |kv: &KnotVector, knots: &[f64]| {
                span_cells(
                    kv,
                    &knot_aligned_cuts(0.0, 1.0, HULL_BLOCKS_PER_SPAN, knots),
                )
                .unwrap()
            };
            let (cells_u, cells_v) = (cells(&ku, &knots_u), cells(&kv, &knots_v));
            let hulls = measure.block_hulls(&cells_u, &cells_v).unwrap();
            let mut k = 0;
            for bu in &cells_u {
                for bv in &cells_v {
                    let (h_uu, h_vv) = hulls[k];
                    k += 1;
                    let (hu, hv) = (1e-3 * (bu.hi - bu.lo), 1e-3 * (bv.hi - bv.lo));
                    for u in samples(bu.lo, bu.hi) {
                        for v in samples(bv.lo, bv.hi) {
                            let fm = f(u, v);
                            let f_uu = (f(u + hu, v) - 2.0 * fm + f(u - hu, v)) / (hu * hu);
                            let f_vv = (f(u, v + hv) - 2.0 * fm + f(u, v - hv)) / (hv * hv);
                            assert!(
                                inside(f_uu, h_uu, 1e-13 * fm.abs() / (hu * hu))
                                    && inside(f_vv, h_vv, 1e-13 * fm.abs() / (hv * hv)),
                                "{name}: at ({u}, {v}) the sampled (f_uu, f_vv) = ({f_uu}, {f_vv}) \
                                 escape the block hulls {h_uu:?}, {h_vv:?}"
                            );
                        }
                    }
                }
            }
            for cu in &cells_u {
                for cv in &cells_v {
                    let (g_u, g_v, g_hull) = measure.area_bounds(cu, cv).unwrap();
                    let (hu, hv) = (1e-4 * (cu.hi - cu.lo), 1e-4 * (cv.hi - cv.lo));
                    for u in samples(cu.lo, cu.hi) {
                        for v in samples(cv.lo, cv.hi) {
                            let gm = g(u, v);
                            let noise = 1e-10 * gm;
                            let d_u = ((g(u + hu, v) - g(u - hu, v)) / (2.0 * hu)).abs();
                            let d_v = ((g(u, v + hv) - g(u, v - hv)) / (2.0 * hv)).abs();
                            assert!(
                                inside(gm, g_hull, noise)
                                    && d_u <= g_u * (1.0 + 1e-4) + noise
                                    && d_v <= g_v * (1.0 + 1e-4) + noise,
                                "{name}: at ({u}, {v}) the sampled g = {gm}, |g_u| = {d_u}, \
                                 |g_v| = {d_v} escape the cell's hull {g_hull:?} or pads \
                                 ({g_u}, {g_v})"
                            );
                        }
                    }
                }
            }
        }
    }

    /// The rational lane's round-0 enclosure of the quarter cylinder
    /// `x² + y² = 1`, `0 ≤ z ≤ 2`, with its `u` arc split at `k` (the
    /// Bézier arc with one knot inserted, so the locus and the truth
    /// `flux = area = π` do not depend on `k`).
    fn quarter_cylinder_split_at(k: f64) -> FaceCutBounds {
        let kv_u = KnotVector::clamped(vec![0.0, 0.0, 0.0, k, 1.0, 1.0, 1.0], 2).unwrap();
        let kv_v = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        let w = core::f64::consts::FRAC_1_SQRT_2;
        let bezier = [[1.0, 0.0, 1.0], [w, w, w], [0.0, 1.0, 1.0]];
        let lerp = |a: [f64; 3], b: [f64; 3]| -> [f64; 3] {
            core::array::from_fn(|i| (1.0 - k).mul_add(a[i], k * b[i]))
        };
        let hom = [
            bezier[0],
            lerp(bezier[0], bezier[1]),
            lerp(bezier[1], bezier[2]),
            bezier[2],
        ];
        let mut net = Vec::new();
        let mut weights = Vec::new();
        for [x, y, h] in hom {
            for z in [0.0, 2.0] {
                net.push([pt(x / h), pt(y / h), pt(z)]);
                weights.push(h);
            }
        }
        let out = nurbs_patch_face_rounds::<f64>(
            &patch(&kv_u, &kv_v, &net, &weights),
            (0.0, 1.0, 0.0, 1.0),
            2.0f64.mul_add(2.0, core::f64::consts::PI),
            0.0,
            Band::linear(Tol::witness()).unwrap(),
            RoundWindow::at(0),
            &mut to_target(Tol::witness().get().eps),
        )
        .unwrap_or_else(|e| panic!("the quarter cylinder split at {k}: {e:?}"));
        bounds_of(&out)
    }

    /// `RefinedNet`'s width has no cliff at any distance of a stated
    /// knot from the point the refinement would take without it: with
    /// the quarter cylinder split at every offset of
    /// [`crate::grid_offsets::knot_offsets`] from the midpoint of its
    /// span (the [`REFINED_PIECES_PER_SPAN`] split), the round-0 flux and
    /// area widths stay within `1.25×` of the split ON it. The
    /// refinement splits each span equally, so a knot beside that point
    /// moves the split points with it rather than leaving a hairline
    /// span whose derivative hulls carry an inserted point's rounding.
    #[test]
    fn the_refine_grid_enclosure_has_no_cliff_at_any_knot_offset() {
        let truth = core::f64::consts::PI;
        let (g, offsets) = crate::grid_offsets::knot_offsets(REFINED_PIECES_PER_SPAN, 1);
        let on = quarter_cylinder_split_at(g);
        let rows: Vec<(String, FaceCutBounds)> = offsets
            .into_iter()
            .map(|(label, k)| (label, quarter_cylinder_split_at(k)))
            .collect();
        let table: String = rows
            .iter()
            .map(|(label, b)| {
                format!(
                    "\n  {label:>26}: flux {:.4e}  area {:.4e}",
                    b.flux.width(),
                    b.area.width()
                )
            })
            .collect();
        for (label, b) in &rows {
            encloses(b.flux, truth, &format!("flux, knot {label}"));
            encloses(b.area, truth, &format!("area, knot {label}"));
            assert!(
                b.flux.width() <= 1.25 * on.flux.width()
                    && b.area.width() <= 1.25 * on.area.width(),
                "knot {label}: the width left 1.25x of on-grid{table}"
            );
        }
    }
}

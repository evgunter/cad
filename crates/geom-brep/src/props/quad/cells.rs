//! **The composite's cells**: the knot-aligned cut schedule the patch
//! lanes refine on ([`knot_aligned_cuts`]) and the per-cell Taylor
//! remainders, and the rational lane's hull blocks — its
//! certified-refined net ([`RefinedNet`]) and the quotient statement's
//! per-block reads ([`MeasureCells`]).

use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;
use geom_core::spline::algebra;
use geom_core::spline::net::TensorNet;
use geom_core::spline::{
    AxisCut, CellAxis, CellBlock, CellMeasureQuotients, KnotVector, QuotientPiece, RingRefinement,
    TensorChannels, cell_measure_quotients,
};

use super::net::{PatchNet, PatchRect, range_spans};
use crate::patch_bound::{certified_inf, certified_sup};
use crate::props::PropsError;
use crate::props::bracket::pt;

/// Pieces per KNOT SPAN, per axis, of the rectangle lanes' 2-D
/// composite rule at round 0, doubling per round (cells = p² per span
/// cell). Per span, not per face, so a patch and its per-span tiles
/// are read on the same cells ([`knot_aligned_cuts`]).
pub(super) const SPAN_INIT_PIECES: usize = 8;
/// Blocks per knot span, per axis, for both rectangle lanes'
/// Taylor-remainder hulls and the rational lane's area pads. Its edges
/// are cut points of every cell list that reads a block
/// ([`block_aligned_cuts`]), so every block owns whole cells at every
/// refinement level.
pub(super) const HULL_BLOCKS_PER_SPAN: usize = 4;
/// Pieces per knot span the RATIONAL lane refines its bracketed net to
/// before hulling it — see [`RefinedNet`].
pub(super) const REFINED_PIECES_PER_SPAN: usize = 2;

/// **A bracketed tensor net, certified-refined** in both directions,
/// every knot span cut into [`REFINED_PIECES_PER_SPAN`] equal pieces
/// ([`algebra::equal_split_points`]), held with the two insertion
/// chains that refined it, so the refined vectors are the chains' own.
///
/// Why the rational lane takes it: a range hull read off a control net
/// is a convexity fact, and those are **span-granular** — on a
/// single-span patch the hull over a sub-cell IS the hull over the
/// whole patch. The rational lane's Taylor remainders and area pads are
/// the quotient statement's, which cuts its own spans
/// ([`MeasureCells`]); what it still reads off the net is the weight
/// hull its zero test takes and the position bound its boundary pad
/// takes. Knot insertion is exact in ℝ (Book §5.2/§5.3 — it changes the
/// representation, never the locus) and the control polygon converges
/// to the patch at O(h²), so splitting every span tightens both.
pub(super) struct RefinedNet {
    u: RingRefinement,
    v: RingRefinement,
    nets: [TensorNet; 3],
}

impl RefinedNet {
    /// `net` refined, its chains built from its own vectors. `None` if
    /// the knot algebra refuses — the caller then refuses typed rather
    /// than proceeding on an unrefined net.
    pub(super) fn of(net: &TensorChannels<'_, 3>) -> Option<Self> {
        let chain = |kv: &KnotVector| {
            RingRefinement::new(
                kv,
                &algebra::equal_split_points(kv, REFINED_PIECES_PER_SPAN),
            )
            .ok()
        };
        let (u, v) = (chain(net.knots_u())?, chain(net.knots_v())?);
        let nets = net.channels().map(|c| c.net().refine_u(&u).refine_v(&v));
        Some(Self { u, v, nets })
    }

    /// The refined channels over the refined vectors.
    pub(super) fn channels(&self) -> TensorChannels<'_, 3> {
        TensorChannels::from_fn(self.u.knots(), self.v.knots(), |i, j| {
            self.nets.each_ref().map(|n| n.get(i, j))
        })
    }
}

/// A cut per axis, `None` along an axis kept whole.
type CellCuts = (Option<AxisCut>, Option<AxisCut>);

/// **The rational lane's measure quotients**
/// ([`cell_measure_quotients`]): one per span cell of the ORIGINAL net
/// that the trim rectangle meets, formed in the frame of the cell
/// window's first point, and read on a quadrature cell or hull block by
/// restricting it there. Every cell and block the lane reads lies
/// inside one span cell, because its cut lists hold every interior knot
/// ([`knot_aligned_cuts`]), so each is read through its [`SpanCell`].
pub(super) struct MeasureCells {
    cells: Vec<((usize, usize), CellMeasureQuotients<Interval>)>,
}

impl MeasureCells {
    /// A restriction or a ratio that does not certify, or is not finite.
    const REFUSED_CUT: PropsError = PropsError::QuadratureUnsupported {
        what: "a rational patch cell whose quotient-statement reading does not certify \
               or overflows (a weight power's coefficient reaching zero, or a span too \
               short for its derivatives' scale) — its Taylor hulls and area pads have \
               no finite bound there",
    };

    pub(super) fn build<'n>(net: &'n PatchNet<'_>, rect: PatchRect) -> Result<Self, PropsError> {
        let (kv_u, kv_v) = (net.knots_u(), net.knots_v());
        let (u0, u1, v0, v1) = rect.corners();
        let held =
            |kv: &KnotVector| -> Vec<Interval> { kv.knots().iter().map(|k| pt(*k)).collect() };
        let (held_u, held_v) = (held(kv_u), held(kv_v));
        // A span's lower end is a cut of the range (`lo` or a knot), so
        // it locates the span exactly.
        let meeting = |kv: &'n KnotVector, lo: f64, hi: f64| {
            range_spans(kv.knots(), lo, hi)
                .into_iter()
                .filter_map(move |(a, _)| kv.span_at(a))
        };
        let mut cells = Vec::new();
        for su in meeting(kv_u, u0, u1) {
            for sv in meeting(kv_v, v0, v1) {
                let mut points = Vec::new();
                let mut ws = Vec::new();
                for iu in su.window() {
                    for iv in sv.window() {
                        let (c, w) = net.at(iu, iv);
                        points.push(c);
                        ws.push(pt(w));
                    }
                }
                let q = cell_measure_quotients(
                    (CellAxis::Span(su, &held_u), CellAxis::Span(sv, &held_v)),
                    CellBlock::Points {
                        points: &points,
                        weights: &ws,
                        origin: points[0].map(|c| pt(c.lo())),
                    },
                )
                .ok_or(PropsError::QuadratureUnsupported {
                    what: "a rational patch past the quotient statement's degree ceiling \
                           (5·max(p, q) above QUOTIENT_MAX_DEGREE) — its Taylor hulls and \
                           area pads have no reading",
                })?;
                cells.push(((su.index(), sv.index()), q));
            }
        }
        Ok(Self { cells })
    }

    /// The span cell holding the cell `u × v`, and the cut that keeps
    /// the cell's part of it: `None` along an axis the cell covers whole.
    fn on(
        &self,
        u: &SpanCell,
        v: &SpanCell,
    ) -> Result<(&CellMeasureQuotients<Interval>, CellCuts), PropsError> {
        // Built in ascending `(u, v)` span order, so a lookup is a
        // search, not a walk: the reads stay linear in the span cells.
        self.cells
            .binary_search_by_key(&(u.span, v.span), |(k, _)| *k)
            .map(|i| (&self.cells[i].1, (u.cut(), v.cut())))
            .map_err(|_| Self::REFUSED_CUT)
    }

    /// The `(f_uu, f_vv)` hull of every block `u × v`, row-major in `u`
    /// then `v`.
    pub(super) fn block_hulls(
        &self,
        cells_u: &[SpanCell],
        cells_v: &[SpanCell],
    ) -> Result<Vec<(Interval, Interval)>, PropsError> {
        let mut out = Vec::with_capacity(cells_u.len() * cells_v.len());
        for u in cells_u {
            for v in cells_v {
                let (q, cuts) = self.on(u, v)?;
                let (uu, vv) = q.flux_second_on(cuts).ok_or(Self::REFUSED_CUT)?;
                out.push((ratio_hull(&uu)?, ratio_hull(&vv)?));
            }
        }
        Ok(out)
    }

    /// A cell's `(G_u, G_v, hull(g))`: upper bounds on
    /// `|∂_d(S_u×S_v)|` and the range of `|S_u×S_v|` over `u × v`.
    pub(super) fn area_bounds(
        &self,
        u: &SpanCell,
        v: &SpanCell,
    ) -> Result<(f64, f64, Interval), PropsError> {
        let (q, cuts) = self.on(u, v)?;
        let on = q.area_on(cuts).ok_or(Self::REFUSED_CUT)?;
        let (g_u, g_v, g_hi) = (
            certified_sup(&on.u),
            certified_sup(&on.v),
            certified_sup(&on.area),
        );
        if g_u.is_finite() && g_v.is_finite() && g_hi.is_finite() {
            Ok((
                g_u,
                g_v,
                Interval::from_bounds(certified_inf(&on.area), g_hi),
            ))
        } else {
            Err(Self::REFUSED_CUT)
        }
    }
}

/// One cell of a cut list together with the span of the knot vector
/// it lies in. Minted only by [`span_cells`], which walks the cuts
/// against the knots: a reader never re-locates a cell from a rounded
/// point inside it, which on a cell an ulp wide can name the next span.
#[derive(Clone, Copy, Debug)]
pub(super) struct SpanCell {
    pub(super) lo: f64,
    pub(super) hi: f64,
    pub(super) span: usize,
    pub(super) ends: (f64, f64),
}

impl SpanCell {
    /// The restriction that keeps this cell's part of its span, `None`
    /// when the cell is the whole span.
    fn cut(&self) -> Option<AxisCut> {
        (self.lo > self.ends.0 || self.hi < self.ends.1).then_some((self.ends, (self.lo, self.hi)))
    }
}

/// The cells of the sorted cut list `cuts`, each with its span of `kv`.
/// The first cut is located once (an exact cut, never a midpoint); each
/// later cell takes the first span whose end lies past its lower cut,
/// by exact comparison. A cell whose upper cut lies past that span's
/// end holds a knot in its interior, which a cut list holding every
/// interior knot ([`knot_aligned_cuts`]) never mints: refused typed.
pub(super) fn span_cells(kv: &KnotVector, cuts: &[f64]) -> Result<Vec<SpanCell>, PropsError> {
    const STRADDLE: PropsError = PropsError::QuadratureUnsupported {
        what: "a quadrature cell holding a knot in its interior — the cut list was \
               not cut on the knot vector it is read against",
    };
    let k = kv.knots();
    let first = *cuts.first().ok_or(STRADDLE)?;
    let mut s = kv.span_at(first).ok_or(STRADDLE)?.index();
    cuts.windows(2)
        .map(|w| {
            let (lo, hi) = (w[0], w[1]);
            while s + 2 < k.len() && k[s + 1] <= lo {
                s += 1;
            }
            let ends = (k[s], k[s + 1]);
            if ends.0 <= lo && hi <= ends.1 {
                Ok(SpanCell {
                    lo,
                    hi,
                    span: s,
                    ends,
                })
            } else {
                Err(STRADDLE)
            }
        })
        .collect()
}

/// The midpoint of the cell `[lo, hi]` as the composite flux rules read
/// it: a locator in `[lo, hi)`, which names the cell's span, and the
/// enclosure of the exact midpoint, at which the span is evaluated.
///
/// The rule's remainder `hull(f_dd)·h³/24` holds about the exact
/// midpoint only; at a rounded one the first-order term `f_d·δ·h`
/// stays outside it, and on a cell beside a knot 1e-9 from the
/// rectangle's edge, whose integrand's slope is near 1e18, that term
/// outweighs the remainder. The exact midpoint lies in the cell, so the
/// enclosure is clamped to it: the outward sum steps past an end on
/// about a third of adjacent-float pairs. The locator is the rounded
/// midpoint unless that rounds up to `hi`, which on a cell an ulp wide
/// is the next span's first knot; `lo` then.
pub(super) fn cell_midpoint(lo: f64, hi: f64) -> (f64, Interval) {
    let m = lo.midpoint(hi);
    (
        if m < hi { m } else { lo },
        ((pt(lo) + pt(hi)) * pt(0.5)).clamped_to(lo, hi),
    )
}

/// The hull of a scalar quotient over its piece, from its coefficient
/// ratios; [`MeasureCells::REFUSED_CUT`] when they do not certify.
fn ratio_hull(q: &QuotientPiece<Interval, 1>) -> Result<Interval, PropsError> {
    let [(lo, hi)] = q.coordinate_range();
    if lo.is_certified() && hi.is_certified() && lo.lo().is_finite() && hi.hi().is_finite() {
        Ok(Interval::from_bounds(lo.lo(), hi.hi()))
    } else {
        Err(MeasureCells::REFUSED_CUT)
    }
}

/// **A cut list in one direction**: the range's ends, the interior
/// knots inside it, and every knot span between two of them
/// ([`range_spans`]) cut into `pieces` equal pieces
/// ([`algebra::split_points`]).
///
/// The knots are the point. An integrand that is only C⁰ at a knot may
/// genuinely JUMP there, and no derivative hull bounds a jump — so a
/// cell holding a knot in its OPEN INTERIOR has no rule better than
/// the smoothness-free `A_cell·hull(f)`, whose width is a control-net
/// fact and therefore span-granular: it stops shrinking once cells are
/// finer than a span. Cutting ON the knots removes the case rather than
/// bounding it: a knot on a cell BOUNDARY is not in any cell's
/// interior, every cell is a smoothness island, and the
/// midpoint-plus-Taylor rule applies throughout at its full O(h²).
///
/// **The grid is per knot span**, so the cells a span gets are a
/// function of that span's ends (clipped to the range) and `pieces`
/// alone, never of how many other spans share the range: a patch read
/// whole and the same patch split into one face per span
/// (`docs/DESIGN.md` [topology]'s C⁰-knot split) are read on the same
/// cells. A knot is a span end by construction, so no grid point needs
/// a clearance from one.
///
/// The result is sorted, deduplicated, and spans exactly `[lo, hi]`,
/// so consecutive cells share a cut point and the cells tile the
/// range with no gap.
///
/// `knots` is the raw slice the caller cuts on. A derivative's interior
/// knots are its parent's, so every caller cuts on the carrier's own.
pub(super) fn knot_aligned_cuts(lo: f64, hi: f64, pieces: usize, knots: &[f64]) -> Vec<f64> {
    let mut cuts = vec![lo];
    for (a, b) in range_spans(knots, lo, hi) {
        cuts.extend(algebra::split_points(a, b, pieces));
        cuts.push(b);
    }
    // Ascending by construction; on a span a few ulps wide two split
    // points can round together, which would mint a zero-width cell.
    cuts.dedup();
    cuts
}

/// **A cut list for cells that read a hull block**: [`knot_aligned_cuts`]
/// at `pieces`, merged with the hull blocks' own edges (the same rule at
/// [`HULL_BLOCKS_PER_SPAN`]). The block edges are in the list for a
/// soundness reason, not a tightness one: each cell reads its hulls from
/// ONE block, which is only valid if the block contains it. While the
/// cell counts ([`SPAN_INIT_PIECES`] doubled, the area pass's) are
/// multiples of the block count the merge adds nothing, since
/// `split_points` at `k/n` and `2k/2n` round alike; it stays so the
/// containment does not rest on that arithmetic, and a count that
/// stopped dividing would cut a cell across a block edge, where the
/// cell's hull read from one block would not bound it, and nothing
/// downstream would say so.
pub(super) fn block_aligned_cuts(lo: f64, hi: f64, pieces: usize, knots: &[f64]) -> Vec<f64> {
    let mut cuts = knot_aligned_cuts(lo, hi, pieces, knots);
    cuts.extend(knot_aligned_cuts(lo, hi, HULL_BLOCKS_PER_SPAN, knots));
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();
    cuts
}

/// The hull block a cell reads its `f_dd` hulls from: the one that
/// CONTAINS it (a hull over a superset contains every sub-cell's
/// own). The block edges are cut points at every refinement level
/// ([`block_aligned_cuts`]), so advancing while the cell starts past
/// the current block's top lands on the block that does.
pub(super) fn advance_block(b: &mut usize, cell_lo: f64, edges: &[f64]) {
    while *b + 1 < edges.len() - 1 && cell_lo >= edges[*b + 1] {
        *b += 1;
    }
}

/// Cuts per axis in v under the rational lane's exact-v arm — one,
/// the whole v extent being integrated exactly per column — and the
/// round's `pieces` otherwise. One home for the loop's cut list and
/// its cost.
fn v_pieces(exact_v: bool, pieces: usize) -> usize {
    if exact_v { 1 } else { pieces }
}

/// **A rectangle lane's cut lists at `round`**, per axis: the round's
/// `pieces` in `u`, and in `v` the same or the exact-v arm's one
/// ([`v_pieces`]), each knot-aligned ([`knot_aligned_cuts`]). The loop
/// cuts on them and the round's cost counts their cells.
pub(super) fn round_cuts(
    rect: PatchRect,
    knots: (&[f64], &[f64]),
    exact_v: bool,
    round: usize,
) -> (Vec<f64>, Vec<f64>) {
    let (u0, u1, v0, v1) = rect.corners();
    let pieces = SPAN_INIT_PIECES << round;
    (
        block_aligned_cuts(u0, u1, pieces, knots.0),
        block_aligned_cuts(v0, v1, v_pieces(exact_v, pieces), knots.1),
    )
}

/// The cells a pair of cut lists makes.
pub(super) fn cell_count((cu, cv): &(Vec<f64>, Vec<f64>)) -> usize {
    (cu.len() - 1) * (cv.len() - 1)
}

/// The u Taylor remainder's cell factor `h_u³·h_v/24` (module docs),
/// one spelling for both patch loops.
pub(super) fn remainder_cell_u(hu: Interval, hv: Interval) -> Interval {
    hu * hu.sqr() * hv / pt(24.0)
}

/// The v Taylor remainder's cell factor `h_u·h_v³/24`
/// ([`remainder_cell_u`]'s twin).
pub(super) fn remainder_cell_v(hu: Interval, hv: Interval) -> Interval {
    hu * hv * hv.sqr() / pt(24.0)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use geom_core::Bounds;

    /// Adjacent-float pairs from bases of both mantissa parities, near
    /// knots the tests use, across binades: 2 000 pairs from each.
    fn adjacent_pairs() -> impl Iterator<Item = (f64, f64)> {
        [
            0.5,
            0.5f64.next_up(),
            0.3,
            1e-9,
            1.0 - 1e-9,
            0.75,
            1e-300,
            7.0,
        ]
        .into_iter()
        .flat_map(|base| {
            core::iter::successors(Some(base), |x: &f64| Some(x.next_up()))
                .take(2000)
                .map(|lo| (lo, lo.next_up()))
        })
    }

    /// **A cell's midpoint enclosure is the cell itself on every
    /// adjacent-float pair, and its locator lies in `[lo, hi)`.** The
    /// exact midpoint of two adjacent floats lies strictly between
    /// them, so its tightest enclosure is `[lo, hi]`: red if the clamp
    /// goes (the outward sum steps past an end on about a third of the
    /// pairs) or if the enclosure reverts to the rounded midpoint (a
    /// point, which excludes the exact one), and red if the locator is
    /// the rounded midpoint where that ties up to `hi`.
    #[test]
    fn a_cells_midpoint_encloses_the_exact_one_on_every_adjacent_pair() {
        for (lo, hi) in adjacent_pairs() {
            let (loc, enc) = cell_midpoint(lo, hi);
            assert!(
                enc.lo() == lo && enc.hi() == hi,
                "[{lo:e}, {hi:e}]: the exact midpoint's enclosure is {enc:?}, not the cell"
            );
            assert!(
                lo <= loc && loc < hi,
                "[{lo:e}, {hi:e}]: the locator {loc:e} is outside [lo, hi)"
            );
        }
        // Where the exact midpoint is a float, it is inside the enclosure.
        let (lo, hi) = (0.5, 0.5f64.next_up().next_up());
        assert!(cell_midpoint(lo, hi).1.contains(0.5f64.next_up()));
    }

    /// **Every cell of a cut list is read in the span it lies in**, on a
    /// knot vector with a one-ulp span at each pair of
    /// [`adjacent_pairs`]' first 100 per base: [`span_cells`] names the
    /// span whose ends hold the cell, and the round loop's locator
    /// ([`cell_midpoint`]) locates that same span. Red at
    /// `k0 = 0.5⁺`, whose rounded midpoint with `k0⁺` ties up into the
    /// next span, if either reads the cell off its rounded midpoint.
    #[test]
    fn every_cell_is_read_in_its_own_span_beside_a_one_ulp_span() {
        for (k0, k1) in adjacent_pairs().step_by(20) {
            if !(k0 > 0.0 && k1 < 1.0) {
                continue;
            }
            let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, k0, k1, 1.0, 1.0, 1.0], 2).unwrap();
            let cuts = knot_aligned_cuts(0.0, 1.0, HULL_BLOCKS_PER_SPAN, &[k0, k1]);
            let cells = span_cells(&kv, &cuts).unwrap();
            assert!(
                cells.iter().any(|c| (c.lo, c.hi) == (k0, k1)),
                "the one-ulp span [{k0:e}, {k1:e}] is a cell"
            );
            for c in &cells {
                assert!(
                    c.ends.0 <= c.lo && c.hi <= c.ends.1,
                    "[{:e}, {:e}] read in the span {:?}",
                    c.lo,
                    c.hi,
                    c.ends
                );
                let loc = cell_midpoint(c.lo, c.hi).0;
                assert_eq!(
                    kv.span_at(loc).map(|s| s.index()),
                    Some(c.span),
                    "[{:e}, {:e}]: the locator {loc:e} names another span",
                    c.lo,
                    c.hi
                );
            }
        }
    }

    /// `RefinedNet` cuts every span of the vector into
    /// [`REFINED_PIECES_PER_SPAN`] equal pieces, however close two knots
    /// stand: a knot an ulp past a sixteenth still gets its spans
    /// halved on both sides. The expected vector is written out by hand.
    #[test]
    fn refined_net_splits_every_span() {
        assert_eq!(REFINED_PIECES_PER_SPAN, 2, "the vector below is the halves");
        let near = 0.0625f64.next_up();
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, near, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let (rkv, count) = refined_u(&kv);
        let half = |a: f64, b: f64| a + (b - a) * 0.5;
        assert_eq!(
            rkv.knots(),
            [
                0.0,
                0.0,
                0.0,
                half(0.0, near),
                near,
                half(near, 0.5),
                0.5,
                0.75,
                1.0,
                1.0,
                1.0
            ]
        );
        assert_eq!(count, 8);
    }

    /// [`RefinedNet`] of a net over `kv` by a linear segment: the
    /// refined `u` vector, and the `u` count its nets were refined to.
    fn refined_u(kv: &KnotVector) -> (KnotVector, usize) {
        let lin = KnotVector::unit_segment(core::num::NonZeroUsize::MIN);
        #[allow(clippy::cast_precision_loss)]
        let net = TensorChannels::from_fn(kv, &lin, |i, _| [pt(i as f64); 3]);
        let refined = RefinedNet::of(&net).unwrap();
        let rkv = refined.channels().knots_u().clone();
        let count = refined.nets[0].nu();
        assert_eq!(
            count,
            rkv.control_count(),
            "the refined nets' count is the refined vector's"
        );
        (rkv, count)
    }

    /// **A cut list holds no repeat**, on a three-ulp span where
    /// `split_points` rounds two of its points together: a repeat would
    /// be a zero-width cell, or a zero-width hull block. Red while the
    /// split points were extended unchecked.
    #[test]
    fn knot_aligned_cuts_hold_no_repeat_on_a_three_ulp_span() {
        let lo = 1.0_f64;
        let hi = f64::from_bits(lo.to_bits() + 3);
        let cuts = knot_aligned_cuts(lo, hi, 4, &[]);
        assert!(
            cuts.windows(2).all(|w| w[0] < w[1]),
            "the cuts strictly ascend: {cuts:?}"
        );
        assert_eq!(
            cuts.len(),
            4,
            "the ends and the two distinct points: {cuts:?}"
        );
    }

    /// `knot_aligned_cuts` pinned bit for bit, the values written out by
    /// `lo + (hi − lo)·(k/n)` per span:
    ///
    /// * `[0.1, 0.7]`, `pieces = 3`, a double knot at `0.5`: the thirds
    ///   of `[0.1, 0.5]` and of `[0.5, 0.7]`, the knot one cut.
    /// * `pieces = 4`, knots outside or ON the range ends add nothing.
    /// * `[-0.3, 0.2]`, `pieces = 3`, no knots: a range straddling zero.
    ///
    /// [`block_aligned_cuts`] adds the quarters of each span to the
    /// first.
    #[test]
    fn knot_aligned_cuts_are_pinned_bit_for_bit() {
        assert_eq!(
            knot_aligned_cuts(0.1, 0.7, 3, &[0.5, 0.5]),
            [
                0.1,
                0.233_333_333_333_333_34,
                0.366_666_666_666_666_7,
                0.5,
                0.566_666_666_666_666_7,
                0.633_333_333_333_333_3,
                0.7
            ]
        );
        assert_eq!(
            knot_aligned_cuts(0.1, 0.7, 4, &[-0.5, 0.1, 0.3, 0.7, 0.9]),
            [
                0.1,
                0.15,
                0.2,
                0.25,
                0.3,
                0.399_999_999_999_999_97,
                0.5,
                0.6,
                0.7
            ]
        );
        assert_eq!(
            knot_aligned_cuts(-0.3, 0.2, 3, &[]),
            [
                -0.3,
                -0.133_333_333_333_333_33,
                0.033_333_333_333_333_326,
                0.2
            ]
        );
        assert_eq!(HULL_BLOCKS_PER_SPAN, 4, "the list below holds the quarters");
        assert_eq!(
            block_aligned_cuts(0.1, 0.7, 3, &[0.5, 0.5]),
            [
                0.1,
                0.2,
                0.233_333_333_333_333_34,
                0.300_000_000_000_000_04,
                0.366_666_666_666_666_7,
                0.4,
                0.5,
                0.55,
                0.566_666_666_666_666_7,
                0.6,
                0.633_333_333_333_333_3,
                0.649_999_999_999_999_9,
                0.7
            ]
        );
    }
}

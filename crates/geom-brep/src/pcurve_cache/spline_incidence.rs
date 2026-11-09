//! **A spline chart's incidence test** (D3): whether a carrier lies on a
//! NURBS chart (a [`geom::Surface::Nurbs`] payload, or an
//! [`geom::Surface::Approx`]'s fit, which is its geometry), read at the
//! samples [`super::incidence_over_span`] reads an analytic chart at,
//! and decided one-sidedly as that test is: `Off` only on a certified
//! LOWER bound on a sample's distance from the chart past the band.
//!
//! # The bound
//!
//! The chart is cut into its Bézier cells (every interior knot raised to
//! full multiplicity by Boehm's insertion, exact in ℝ). With every weight
//! positive, a rational Bézier cell lies in the convex hull of its
//! control points out of homogeneous form, so in any convex set holding
//! them. Two such sets are read, and the larger distance to them is the
//! cell's **floor**:
//!
//! - the control points' axis-aligned box;
//! - the slab between the two planes, normal to the cross product of the
//!   net's corner diagonals, that hold every control point. A nearly
//!   flat cell has a slab nearly as thin as itself, whatever its tilt,
//!   where its box is as thick as the cell is wide.
//!
//! A cell whose floor is decided past the band holds no point near the
//! sample. Any other cell is halved by de Casteljau across its longer way
//! (its control net's longer polyline), and the floor of
//! the chart is the least floor over the cells that cleared. The chart's
//! point at a kept cell's parameter centre bounds the sample's distance
//! from above: decided within the zero band, the sample is on the
//! chart. The walk ends when every cell has cleared (the sample is off),
//! at such a point (on), or at its depth or visit cap with a cell kept,
//! unsettled: the first decision it made in band is the read's cause, or,
//! where every decision was definite, the straddle of the kept cells'
//! least floor (decided zero) and the points' least distance (decided
//! past the band), [`geom_core::k_stats::straddle`].
//!
//! **A weight not shown positive** leaves the cell's hull unproven, so
//! its floor is the poison: its decision escalates with an unreadable
//! margin, and the cell is kept as a leaf: halving does not cure the
//! poison, nor a poisoned net or sample. A chart built through
//! [`geom::NurbsSurface::new`] holds positive weights on their value
//! channel; only an enclosure reaching zero meets this.
//!
//! # Rounding
//!
//! The net is cut about the middle of its box, `c`, so its coordinates
//! are of the patch's size, not its placement. At an enclosure scalar
//! ([`geom_core::Witness::Exact`]) the move and the cut are in the
//! enclosure's own arithmetic, so each control point's box holds the
//! exact one, and the floors are decided as they are.
//!
//! At `f64` each homogeneous control point carries a first-order bound
//! on each channel's rounding, set where the point is made and carried
//! through every blend `x + (y − x)·t` of the insertions and the
//! halvings ([`blend`]): the inputs' bounds blend as the values do,
//! and each blend adds `u·|y − x|·t` for the difference, `u·|result|`
//! for the sum, and, at an insertion's ratio of knots, `u·|y − x|·t`
//! more for the product by the ratio and `|y − x|·δt` for the ratio's
//! own rounding (`δt` three units of roundoff of `t`). A halving's
//! product by the exact half is exact.
//! Out of homogeneous form a point is off by at most
//! `(e_xyz + |P|·e_w)/(w − e_w) + u·|P|` per coordinate, and the floor is
//! charged the root sum of the worst per channel ([`cut_rounding`]). Every
//! bound is read at the point it belongs to, so a weight far from the
//! cell's own charges nothing there. The floor is also charged
//! [`geom_core::rounding_charge`] of twice the reach of the points, the
//! sample and `c`, which covers the moves and the read of a box or a
//! slab, as [`super::incidence_at`] charges its read. It is a
//! first-order rounding estimate, the `f64` lane's contract, as
//! [`geom_core::running`]'s is.
//!
//! The upper bound is the chart's own evaluation at `f64`, so its
//! rounding errs either way. Toward on, it can read a sample on the
//! chart where its exact distance is a few roundings of a far patch's
//! placement past the zero band. That ends the read as not shown off, a
//! lane gap and never a false `Off`.

use geom::NurbsSurface;
use geom_core::k_stats::{decide, decide_reported, straddle};
use geom_core::spline::{Knots, SpanLocate};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, Point3, Real, Sign, UNIT_ROUNDOFF, Vec3,
    rounding_charge,
};

use super::{INCIDENCE_SAMPLES, Incidence, PcurveCertifyError, incidence_escalated};

#[cfg(test)]
mod rows;

/// A cell's floor against the sample: whether the cell clears it.
pub(super) const HULL: &str = "pcurve_spline_chart_hull";
/// The chart's point at a kept cell's centre against the sample:
/// whether the sample is on the chart.
pub(super) const FOOT: &str = "pcurve_spline_chart_foot";
/// A sample left with a kept cell whose floor decided zero and whose
/// points decided past the band.
pub(super) const STRADDLE: &str = "pcurve_spline_chart_straddle";

/// How many times a kept cell is halved at most. `topo`'s spline face
/// placement walks the same cells with its own pair (`CELL_DEPTH` 48
/// quarterings, `CELL_VISITS` 8192): one home for both is
/// `work/splice/the-spline-chart-cell-walk-is-written-twice.md`.
const CELL_DEPTH: usize = 96;
/// The levels past the one where the chart's own diagonal, halved at
/// each, is below half the band's zero: a cell's geometric size halves
/// only on average with its parameters'.
const DEPTH_HEADROOM: f64 = 8.0;
/// The kept cells one sample's walk may visit.
const CELL_VISITS: usize = 1 << 12;

/// **The spline chart's incidence test of the carrier `at` over
/// `[t0, t1]`** (module docs), at [`INCIDENCE_SAMPLES`] + 1 evenly
/// spaced parameters, its ends included. A chart patch is bounded, so
/// the read is always the edge's span, never a closed conic's turn.
///
/// # Errors
///
/// [`PcurveCertifyError::Escalated`] ([`super::PcurveCheck::ChartIncidence`])
/// where no sample is shown off and some sample is unsettled: in band,
/// at a straddle, or with an unreadable floor (the cell structure does
/// not decompose, or a weight not shown positive).
pub(super) fn spline_incidence<T: Decide>(
    name: &'static str,
    chart: &NurbsSurface<T>,
    (t0, t1): (T, T),
    at: impl Fn(T) -> Point3<T>,
    band: Band,
) -> Result<Incidence, PcurveCertifyError> {
    let Some(net) = Net::of(chart, band) else {
        return Err(incidence_escalated(unreadable(band)));
    };
    let n = f64::from(INCIDENCE_SAMPLES);
    let mut pending: Option<Indeterminate> = None;
    for i in 0..=INCIDENCE_SAMPLES {
        let t = t0 + (t1 - t0) * T::from_f64(f64::from(i) / n);
        match net.read(at(t), band) {
            Read::On => {}
            Read::Off(floor) => match decide_reported(name, Margin::of(floor), band) {
                Ok(Decided {
                    sign: Sign::Positive,
                    margin,
                }) => return Ok(Incidence::Off(margin)),
                Ok(_) => unreachable!(
                    "spline_incidence: the least of floors each decided past the band is past it"
                ),
                Err(cause) => keep(&mut pending, cause),
            },
            Read::Unsettled(cause) => keep(&mut pending, cause),
        }
    }
    match pending {
        None => Ok(Incidence::On(None)),
        Some(cause) => Err(incidence_escalated(cause)),
    }
}

/// The first unsettled decision is the one reported.
fn keep(pending: &mut Option<Indeterminate>, cause: Indeterminate) {
    if pending.is_none() {
        *pending = Some(cause);
    }
}

/// The escalation of a floor that could not be read at all: the poison,
/// decided.
fn unreadable(band: Band) -> Indeterminate {
    match decide(HULL, Margin::of(f64::NAN), band) {
        Err(cause) => cause,
        Ok(_) => unreachable!("unreadable: the poison decides no sign"),
    }
}

/// What one sample's walk found.
enum Read<T> {
    /// The chart's point at a kept cell's centre was decided within the
    /// zero band of it.
    On,
    /// Every cell cleared it: the least of their floors.
    Off(T),
    /// Neither, with the cause.
    Unsettled(Indeterminate),
}

/// **The chart's Bézier cells** and what the walk reads them at.
struct Net<'a, T: Real> {
    chart: &'a NurbsSurface<T>,
    /// The middle of the net's box, the origin the cells are held about.
    centre: Vec3<T>,
    cells: Vec<Cell<T>>,
    pu: usize,
    pv: usize,
    /// The halvings a walk may make ([`depth_cap`]).
    depth: usize,
}

/// A homogeneous control point, `[w·x, w·y, w·z, w]` about the centre,
/// with a first-order bound on each channel's accumulated rounding at
/// `f64` (module docs, *Rounding*); zero at an enclosure scalar, whose
/// arithmetic holds the exact value already.
type Control<T> = ([T; 4], [f64; 4]);

/// One Bézier cell: its parameter rectangle `[u0, u1] × [v0, v1]`, its
/// control net row-major in `u`, and how many times it was halved.
#[derive(Clone)]
struct Cell<T> {
    rect: [f64; 4],
    net: Vec<Control<T>>,
    depth: usize,
}

/// A cell in the walk's queue, the nearest box (read on the brackets'
/// middles, which order the walk and decide nothing) first, then the
/// nearest corner (among boxes that all hold the sample, a point of the
/// chart near it), then the deepest, then the order it was queued in
/// (D9).
struct Queued<T> {
    key: (f64, f64),
    seq: usize,
    cell: Cell<T>,
}

impl<T> PartialEq for Queued<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == core::cmp::Ordering::Equal
    }
}

impl<T> Eq for Queued<T> {}

impl<T> PartialOrd for Queued<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Queued<T> {
    /// Reversed, so the max-heap pops the nearest first.
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        other
            .key
            .0
            .total_cmp(&self.key.0)
            .then_with(|| other.key.1.total_cmp(&self.key.1))
            .then_with(|| self.cell.depth.cmp(&other.cell.depth))
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

/// **One blend `x + (y − x)·t`** of two homogeneous control points, each
/// channel by [`SpanLocate::homogeneous_step`], with its rounding bound
/// at `f64`. The step rounds three times, with no fused multiply-add:
/// the difference, its product by `t`, and the sum. To first order the
/// bound is the inputs' bounds blended as the values are, plus
/// `u·|y − x|·t` for the difference, `u·|y − x|·t` for the product,
/// `u·|result|` for the sum, and `|y − x|·dt` for `t`'s own rounding.
/// A halving's `t` is the exact half, so its product is exact and
/// `dt` is zero, and the product term is dropped. An insertion's ratio
/// of knots is rounded by three units of `t` (`dt`: two differences and
/// a quotient), and its product rounds as well.
fn blend<T: Decide>(x: Control<T>, y: Control<T>, t: T, ratio: bool, track: bool) -> Control<T> {
    let h: [T; 4] = core::array::from_fn(|c| T::homogeneous_step(x.0[c], y.0[c], t));
    if !track {
        return (h, [0.0; 4]);
    }
    let tf = middle(t);
    let (dt, roundings) = if ratio {
        (3.0 * UNIT_ROUNDOFF * tf.abs(), 2.0)
    } else {
        (0.0, 1.0)
    };
    #[cfg(test)]
    let roundings = if super::mutants::spline_product_uncharged() {
        1.0
    } else {
        roundings
    };
    let e = core::array::from_fn(|c| {
        let span = (middle(y.0[c]) - middle(x.0[c])).abs();
        (1.0 - tf) * x.1[c]
            + tf * y.1[c]
            + UNIT_ROUNDOFF * (roundings * span * tf.abs() + middle(h[c]).abs())
            + span * dt
    });
    (h, e)
}

impl<'a, T: Decide> Net<'a, T> {
    fn of(chart: &'a NurbsSurface<T>, band: Band) -> Option<Self> {
        let (ku, kv) = (chart.knot_values_u(), chart.knot_values_v());
        let (pu, pv) = (ku.degree(), kv.degree());
        let (nu, nv) = chart.control_counts();
        if pu == 0 || pv == 0 || nu == 0 || nv == 0 {
            return None;
        }
        let track = T::WITNESS == geom_core::Witness::Inexact;
        let (control, weights) = (chart.control(), chart.weights());
        let (lo, hi) = shadow_box(control);
        let centre = Vec3::new(
            T::from_computed(lo[0] + (hi[0] - lo[0]) * 0.5),
            T::from_computed(lo[1] + (hi[1] - lo[1]) * 0.5),
            T::from_computed(lo[2] + (hi[2] - lo[2]) * 0.5),
        );
        // Moved about the centre and out to homogeneous form: the
        // difference and the product round each coordinate by `u` of it.
        let homogeneous = |k: usize| -> Control<T> {
            let (p, w) = (control[k] - centre, weights[k]);
            let h = [p.x * w, p.y * w, p.z * w, w];
            let e = if track {
                core::array::from_fn(|c| {
                    if c == 3 {
                        0.0
                    } else {
                        2.0 * UNIT_ROUNDOFF * middle(h[c]).abs()
                    }
                })
            } else {
                [0.0; 4]
            };
            (h, e)
        };
        // Along `v` first, one row per `u` index; then along `u`, one
        // row per refined `v` index.
        let rows: Vec<Vec<Control<T>>> = (0..nu)
            .map(|i| {
                let row = (0..nv).map(|j| homogeneous(i * nv + j)).collect();
                refined(kv, row, track)
            })
            .collect::<Option<_>>()?;
        let nv = rows.first()?.len();
        let columns: Vec<Vec<Control<T>>> = (0..nv)
            .map(|j| refined(ku, rows.iter().map(|r| r[j]).collect(), track))
            .collect::<Option<_>>()?;
        let nu = columns.first()?.len();
        let (bu, bv) = (breaks(ku)?, breaks(kv)?);
        let (cu, cv) = (bu.len() - 1, bv.len() - 1);
        if nu != cu * pu + 1 || nv != cv * pv + 1 {
            return None;
        }
        let mut cells = Vec::with_capacity(cu * cv);
        for i in 0..cu {
            for j in 0..cv {
                let mut net = Vec::with_capacity((pu + 1) * (pv + 1));
                for a in 0..=pu {
                    for b in 0..=pv {
                        net.push(columns[j * pv + b][i * pu + a]);
                    }
                }
                cells.push(Cell {
                    rect: [bu[i], bu[i + 1], bv[j], bv[j + 1]],
                    net,
                    depth: 0,
                });
            }
        }
        Some(Self {
            chart,
            centre,
            cells,
            pu,
            pv,
            depth: depth_cap(lo, hi, band),
        })
    }

    /// **One sample's walk** (module docs), nearest box first, so a
    /// sample on the chart dives to its cell rather than walking every
    /// cell beside it to the cap.
    ///
    /// A point of the chart read in band of the sample shows it within
    /// the band, so not off it. Past that, only a cell whose floor reads
    /// within the zero band can hold a point within the zero band of the
    /// sample, a floor bounding every point of its cell and so of every
    /// cell cut from it: a cell whose floor reads past the zero band is
    /// dropped unread. A sample in band of the chart is so left with no
    /// cell once its cells are small enough that their floors pass the
    /// zero band.
    fn read(&self, q: Point3<T>, band: Band) -> Read<T> {
        let moved = q - self.centre;
        let key_q = [moved.x, moved.y, moved.z].map(middle);
        // Best first, not depth first: a queue keyed on each cell's
        // nearness to the sample, so this walk is not
        // `geom_core::bisect`'s, whose order is depth first.
        let mut seq = 0usize;
        let mut queue = std::collections::BinaryHeap::new();
        let mut push = |queue: &mut std::collections::BinaryHeap<Queued<T>>, cell: Cell<T>| {
            seq += 1;
            let key = box_key(&cell, key_q, self.pu, self.pv);
            queue.push(Queued { key, seq, cell });
        };
        for cell in &self.cells {
            push(&mut queue, cell.clone());
        }
        // The least floor over the cells that cleared and over the kept
        // ones, and the least distance of a point past the zero band.
        let (mut floor, mut kept, mut foot) = (None, None, None);
        let mut pending: Option<Indeterminate> = None;
        let mut near = false;
        let mut unsettled = false;
        let mut visits = 0usize;
        #[cfg(test)]
        let mut cut_short = 0usize;
        let least = |acc: Option<T>, x: T| Some(acc.map_or(x, |a| a.min(x)));
        while let Some(Queued { cell, .. }) = queue.pop() {
            let gap = self.floor(&cell, moved);
            match decide(HULL, Margin::of(gap), band) {
                Ok(Sign::Positive) => {
                    floor = least(floor, gap);
                    continue;
                }
                Ok(Sign::Zero) => kept = least(kept, gap),
                Ok(Sign::Negative) => unreachable!("read: a floor is a distance"),
                Err(cause) => {
                    keep(&mut pending, cause);
                    if near && past_zero(gap, band) {
                        unsettled = true;
                        continue;
                    }
                }
            }
            visits += 1;
            #[cfg(test)]
            if near && super::mutants::spline_on_cut_short() {
                cut_short += 1;
                if cut_short > 4 {
                    unsettled = true;
                    break;
                }
            }
            if visits > CELL_VISITS {
                unsettled = true;
                break;
            }
            let [u0, u1, v0, v1] = cell.rect;
            let centre = self.chart.eval(
                T::from_computed(u0 + (u1 - u0) * 0.5),
                T::from_computed(v0 + (v1 - v0) * 0.5),
            );
            let distance = (centre - q).norm();

            match decide(FOOT, Margin::of(distance), band) {
                Ok(Sign::Zero) => return Read::On,
                Ok(_) => foot = least(foot, distance),
                Err(cause) => {
                    keep(&mut pending, cause);
                    near = true;
                }
            }
            // A poisoned floor or point is not cured by halving, since
            // the poison is in the net or the sample: the cell is a leaf.
            if cell.depth >= self.depth || gap.is_poison() || distance.is_poison() {
                unsettled = true;
                continue;
            }
            let (a, b) = cell.split(self.pu, self.pv, cell.longer_along_u(self.pu, self.pv));
            push(&mut queue, a);
            push(&mut queue, b);
        }
        if !unsettled {
            return match floor {
                Some(floor) => Read::Off(floor),
                None => Read::Unsettled(unreadable(band)),
            };
        }
        Read::Unsettled(pending.unwrap_or_else(|| match (kept, foot) {
            (Some(kept), Some(foot)) => match (
                decide_reported(HULL, Margin::of(kept), band),
                decide_reported(FOOT, Margin::of(foot), band),
            ) {
                (Ok(lower), Ok(upper)) => straddle(STRADDLE, lower, upper, band),
                (Err(cause), _) | (_, Err(cause)) => cause,
            },
            _ => unreadable(band),
        }))
    }

    /// **The cell's floor against the moved sample `q`** (module docs,
    /// *The bound*): the larger of its distances to the control points'
    /// box and slab, less, at `f64`, the cut's rounding and the read's.
    /// The poison where a weight is not shown positive.
    fn floor(&self, cell: &Cell<T>, q: Point3<T>) -> T {
        if !cell.net.iter().all(|(h, _)| h[3].structure_positive()) {
            return T::from_f64(f64::NAN);
        }
        let points: Vec<Point3<T>> = cell.net.iter().map(|&(h, _)| point(h)).collect();
        #[cfg(test)]
        if super::mutants::spline_floor_net() {
            return points
                .iter()
                .map(|p| (*p - q).norm())
                .reduce(T::min)
                .unwrap_or_else(|| T::from_f64(f64::NAN));
        }
        let gap = box_gap(&points, q);
        let gap = match slab_gap(&points, self.pu, self.pv, q) {
            Some(slab) => gap.max(slab),
            None => gap,
        };
        if T::WITNESS == geom_core::Witness::Exact {
            return gap;
        }
        #[cfg(test)]
        if super::mutants::spline_charge_dropped() {
            return gap;
        }
        let reach = points
            .iter()
            .map(|p| (*p - Point3::origin()).norm())
            .fold(T::zero(), T::max);
        let read = rounding_charge(
            T::from_f64(2.0) * (reach + (q - Point3::origin()).norm() + self.centre.norm()),
        );
        (gap - T::from_computed(cut_rounding(cell)) - read).max(T::zero())
    }
}

/// Whether an in-band floor is shown past the band's zero, so its cell
/// holds no point within the zero band of the sample: an exact order
/// ([`geom_core::k_stats::prove`]), since an enclosure straddling the
/// zero is in band too and may hold such a point. Not a reading of the
/// gap against the band: it is asked only where that reading
/// ([`decide`] at `band`) refused, and the band would refuse it again.
fn past_zero<T: Decide>(gap: T, band: Band) -> bool {
    matches!(
        geom_core::k_stats::prove(HULL, Margin::of(gap - T::from_f64(band.zero()))),
        Ok(Sign::Positive)
    )
}

/// **The cut's rounding of a cell's points** at `f64` (module docs,
/// *Rounding*): how far the box or the slab of the computed points can
/// lie from the exact ones', `√(Σ_c ε_c²)` over the channels, `ε_c` the
/// largest over the points of `(e_c + |P_c|·e_w)/(w − e_w) + u·|P_c|`
/// (out of homogeneous form, the quotient's own rounding included). Any
/// direction holds the hull in its slab, so a slab read on the computed
/// points' direction moves with the points alone. Infinite where a
/// weight's bound reaches the weight, which leaves the cell no floor.
fn cut_rounding<T: SpanLocate>(cell: &Cell<T>) -> f64 {
    #[cfg(test)]
    if super::mutants::spline_cut_uncharged() {
        return 0.0;
    }
    let mut worst = [0.0f64; 3];
    for control in &cell.net {
        let bound = point_rounding(control);
        for c in 0..3 {
            worst[c] = worst[c].max(bound[c]);
        }
    }
    worst.iter().map(|x| x.powi(2)).sum::<f64>().sqrt()
}

/// One control point's rounding out of homogeneous form, per coordinate
/// ([`cut_rounding`]): `(e_c + |P_c|·e_w)/(w − e_w) + u·|P_c|`, infinite
/// where the weight's bound reaches the weight.
fn point_rounding<T: SpanLocate>((h, e): &Control<T>) -> [f64; 3] {
    let w = middle(h[3]);
    let lead = w - e[3];
    if lead.partial_cmp(&0.0) != Some(core::cmp::Ordering::Greater) {
        return [f64::INFINITY; 3];
    }
    #[cfg(test)]
    let scale = if super::mutants::spline_bound_undercut() {
        0.25
    } else {
        1.0
    };
    #[cfg(not(test))]
    let scale = 1.0;
    core::array::from_fn(|c| {
        let p = (middle(h[c]) / w).abs();
        scale * ((e[c] + p * e[3]) / lead + UNIT_ROUNDOFF * p)
    })
}

impl<T: Decide> Cell<T> {
    /// Whether the cell is longer along `u` than along `v`: each way, the
    /// longest polyline of control points or chord between corners (points
    /// of the chart, which a rational row of equal points but other
    /// weights moves), on the brackets' middles. A cell is halved across
    /// its longer way, so a way along which it does not move (a degree-one
    /// span between equal rows of equal weights) is never cut.
    fn longer_along_u(&self, pu: usize, pv: usize) -> bool {
        let at = |i: usize, j: usize| {
            let h = self.net[i * (pv + 1) + j].0;
            let w = middle(h[3]);
            [0, 1, 2].map(|k| middle(h[k]) / w)
        };
        let gap =
            |a: [f64; 3], b: [f64; 3]| (0..3).map(|k| (a[k] - b[k]).powi(2)).sum::<f64>().sqrt();
        let along_u = (0..=pv)
            .map(|j| (0..pu).map(|i| gap(at(i, j), at(i + 1, j))).sum::<f64>())
            .fold(0.0, f64::max)
            .max(gap(at(0, 0), at(pu, 0)).max(gap(at(0, pv), at(pu, pv))));
        let along_v = (0..=pu)
            .map(|i| (0..pv).map(|j| gap(at(i, j), at(i, j + 1))).sum::<f64>())
            .fold(0.0, f64::max)
            .max(gap(at(0, 0), at(0, pv)).max(gap(at(pu, 0), at(pu, pv))));
        along_v.partial_cmp(&along_u) != Some(core::cmp::Ordering::Greater)
    }

    /// The cell split at the middle of its `u` range (`along_u`) or its
    /// `v` range, by de Casteljau on each row.
    fn split(&self, pu: usize, pv: usize, along_u: bool) -> (Self, Self) {
        let (n, m) = if along_u { (pu, pv) } else { (pv, pu) };
        let at = |i: usize, k: usize| {
            if along_u {
                i * (pv + 1) + k
            } else {
                k * (pv + 1) + i
            }
        };
        let track = T::WITNESS == geom_core::Witness::Inexact;
        let half = T::from_f64(0.5);
        let mut left = self.net.clone();
        let mut right = self.net.clone();
        for k in 0..=m {
            let mut row: Vec<Control<T>> = (0..=n).map(|i| self.net[at(i, k)]).collect();
            for level in 0..=n {
                left[at(level, k)] = row[0];
                right[at(n - level, k)] = row[n - level];
                for i in 0..n - level {
                    row[i] = blend(row[i], row[i + 1], half, false, track);
                }
            }
        }
        let [u0, u1, v0, v1] = self.rect;
        let (rect_left, rect_right) = if along_u {
            let mid = u0 + (u1 - u0) * 0.5;
            ([u0, mid, v0, v1], [mid, u1, v0, v1])
        } else {
            let mid = v0 + (v1 - v0) * 0.5;
            ([u0, u1, v0, mid], [u0, u1, mid, v1])
        };
        let depth = self.depth + 1;
        (
            Self {
                rect: rect_left,
                net: left,
                depth,
            },
            Self {
                rect: rect_right,
                net: right,
                depth,
            },
        )
    }
}

/// The box of the control points, on their brackets' middles.
fn shadow_box<T: SpanLocate>(control: &[Point3<T>]) -> ([f64; 3], [f64; 3]) {
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for p in control {
        for (k, x) in [p.x, p.y, p.z].into_iter().enumerate() {
            let x = middle(x);
            lo[k] = lo[k].min(x);
            hi[k] = hi[k].max(x);
        }
    }
    (lo, hi)
}

/// The break values of `knots`, one per run, on the value channel.
fn breaks<T: Decide>(knots: &Knots<T>) -> Option<Vec<f64>> {
    let out: Vec<f64> = knots.runs().map(|(v, _)| v.value_channel()).collect();
    out.iter().all(|x| x.is_finite()).then_some(out)
}

/// The middle of a value's bracket: the walk's order, and the value an
/// `f64` rounding bound is read at. NaN for the poison.
fn middle<T: SpanLocate>(x: T) -> f64 {
    x.structure_bracket()
        .map_or(f64::NAN, |(lo, hi)| lo + (hi - lo) * 0.5)
}

/// The squared gap from the sample to the cell's box and its squared
/// distance from the nearest of the cell's four corners, points of the
/// chart, on the brackets' middles: the walk's order. NaN orders last.
fn box_key<T: SpanLocate>(cell: &Cell<T>, q: [f64; 3], pu: usize, pv: usize) -> (f64, f64) {
    let at = |(h, _): &Control<T>| {
        let w = middle(h[3]);
        [0, 1, 2].map(|k| middle(h[k]) / w)
    };
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for control in &cell.net {
        let p = at(control);
        for k in 0..3 {
            lo[k] = lo[k].min(p[k]);
            hi[k] = hi[k].max(p[k]);
        }
    }
    let gap = (0..3)
        .map(|k| (lo[k] - q[k]).max(q[k] - hi[k]).max(0.0).powi(2))
        .sum::<f64>();
    let corner = [(0, 0), (0, pv), (pu, 0), (pu, pv)]
        .map(|(a, b)| {
            let p = at(&cell.net[a * (pv + 1) + b]);
            (0..3).map(|k| (p[k] - q[k]).powi(2)).sum::<f64>()
        })
        .into_iter()
        .fold(f64::INFINITY, f64::min);
    let last = |d: f64| if d.is_nan() { f64::INFINITY } else { d };
    (last(gap), last(corner))
}

/// **A homogeneous row refined to a Bézier cut along `knots`**: every
/// interior knot raised to the degree by Boehm's insertion, through the
/// kernel's insertion plans ([`geom_core::spline::plan_inputs::refine_plan`])
/// applied in the scalar ([`geom_core::spline::CurvePlan::apply_homogeneous`]):
/// each ratio formed from the held knots, each step a [`blend`], so an
/// enclosure holds the exact refined row and the `f64` bound follows the
/// arithmetic. Not [`geom_core::spline::RingRefinement`]: it refines one
/// `Interval` channel ([`geom_core::spline::CurvePlan::apply_certified`]),
/// and this cut runs at every scalar, carrying the `f64` bound beside each
/// point. `None` where a knot has no value or a plan refuses.
fn refined<T: Decide>(
    knots: &Knots<T>,
    row: Vec<Control<T>>,
    track: bool,
) -> Option<Vec<Control<T>>> {
    let p = knots.degree();
    let runs: Vec<(T, usize)> = knots.runs().collect();
    let mut add = Vec::new();
    for &(value, m) in runs.iter().skip(1).take(runs.len().saturating_sub(2)) {
        let x = value.value_channel();
        if m > p || x.is_nan() {
            return None;
        }
        add.extend(core::iter::repeat_n(x, p - m));
    }
    if add.is_empty() {
        return Some(row);
    }
    // Every knot an insertion meets is one of the held breakpoints.
    let held = |v: f64| {
        runs.iter()
            .find(|(value, _)| value.value_channel() == v)
            .map_or_else(|| T::from_f64(f64::NAN), |(value, _)| *value)
    };
    let unit = geom_core::spline::Weights::new(vec![T::one(); row.len()]).ok()?;
    let plans = geom_core::spline::plan_inputs::refine_plan(knots, &unit, &add).ok()?;
    let mut values: Vec<T> = knots.values().to_vec();
    let poison: Control<T> = ([T::from_f64(f64::NAN); 4], [f64::NAN; 4]);
    let mut row = row;
    for plan in &plans {
        let pairs: Vec<(Control<T>, T)> = row.iter().map(|c| (*c, c.0[3])).collect();
        row = plan
            .apply_homogeneous(
                &values,
                &pairs,
                ((poison, poison.0[3]), &held),
                |x, y, t| blend(x, y, t, true, track),
            )
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        values = plan.knots().knots().iter().map(|&v| held(v)).collect();
    }
    (row.iter().all(|(h, _)| !h[3].is_poison())).then_some(row)
}

/// A homogeneous control point out of homogeneous form.
fn point<T: Real>(h: [T; 4]) -> Point3<T> {
    Point3::new(h[0] / h[3], h[1] / h[3], h[2] / h[3])
}

/// The distance from `q` to the box of `points`, which holds their
/// hull.
fn box_gap<T: Real>(points: &[Point3<T>], q: Point3<T>) -> T {
    let Some((&first, rest)) = points.split_first() else {
        return T::from_f64(f64::NAN);
    };
    let (mut lo, mut hi) = (first, first);
    for p in rest {
        lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
        hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
    }
    let axis = |lo: T, hi: T, x: T| (lo - x).max(x - hi).max(T::zero());
    (axis(lo.x, hi.x, q.x).powi(2) + axis(lo.y, hi.y, q.y).powi(2) + axis(lo.z, hi.z, q.z).powi(2))
        .sqrt()
}

/// The distance from `q` to the slab of `points` normal to the cross
/// product of the net's corner diagonals, which holds their hull for any
/// direction; `None` where that direction has no length (a cell whose
/// diagonals are parallel or a point).
fn slab_gap<T: Real>(points: &[Point3<T>], pu: usize, pv: usize, q: Point3<T>) -> Option<T> {
    let corner = |a: usize, b: usize| points[a * (pv + 1) + b];
    let n = (corner(pu, pv) - corner(0, 0)).cross(corner(0, pv) - corner(pu, 0));
    let height = |p: Point3<T>| (p - Point3::origin()).dot(n);
    let mut heights = points.iter().map(|&p| height(p));
    let first = heights.next()?;
    let (lo, hi) = heights.fold((first, first), |(lo, hi), h| (lo.min(h), hi.max(h)));
    let at = height(q);
    let gap = (lo - at).max(at - hi).max(T::zero()) / n.norm();
    gap.is_bounded().then_some(gap)
}

/// The halvings a walk may make: [`CELL_DEPTH`], or fewer where the
/// net's diagonal, halved at every second halving, falls below half the
/// band's zero [`DEPTH_HEADROOM`] levels sooner. Read off the shadow's box, a
/// bound on the work alone: a poisoned point caps nothing.
fn depth_cap(lo: [f64; 3], hi: [f64; 3], band: Band) -> usize {
    let diagonal = (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>().sqrt();
    let levels = (2.0 * diagonal / band.zero()).log2() + DEPTH_HEADROOM;
    if levels.is_finite() && levels >= 0.0 {
        (2 * levels.ceil() as usize).min(CELL_DEPTH)
    } else {
        CELL_DEPTH
    }
}

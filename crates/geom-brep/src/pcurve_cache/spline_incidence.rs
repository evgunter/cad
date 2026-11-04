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
//! sample. Any other cell is quartered by de Casteljau, and the floor of
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
//! margin, and the cell is kept as a leaf: quartering does not cure the
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
//! exact one, and the floors are decided as they are. At `f64` the cut
//! rounds: each blend `x + (y − x)·s` of the insertions and the
//! quarterings is of values within the net's largest homogeneous
//! coordinate, a few half-ulps of it, and a blend carries its inputs'
//! errors as a convex combination does. A cell `d` quarterings deep has
//! been through `k + d·(p + q)` levels (`k` the insertions, `p` and `q`
//! the degrees), each charged [`geom_core::rounding_charge`] of the
//! largest coordinate and of the weight, out of homogeneous form over
//! the least weight. The moves of the net and the sample by `c`, and the
//! sample's reading of a box or a slab, are charged the same rounding of
//! their reach, as [`super::incidence_over_span`] charges its read. A
//! first-order rounding estimate, the `f64` lane's contract, as
//! [`geom_core::running`]'s is. The upper bound is the chart's own
//! evaluation, whose rounding errs only toward reading the sample on.

use geom::NurbsSurface;
use geom_core::k_stats::{decide, decide_reported, straddle};
use geom_core::spline::{Knots, SpanLocate};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, Point3, Real, Sign, Vec3, rounding_charge,
};

use super::{INCIDENCE_SAMPLES, Incidence, PcurveCertifyError, incidence_escalated};

#[cfg(test)]
mod rows;
#[cfg(test)]
mod probe;

/// A cell's floor against the sample: whether the cell clears it.
pub(super) const HULL: &str = "pcurve_spline_chart_hull";
/// The chart's point at a kept cell's centre against the sample:
/// whether the sample is on the chart.
pub(super) const FOOT: &str = "pcurve_spline_chart_foot";
/// A sample left with a kept cell whose floor decided zero and whose
/// points decided past the band.
pub(super) const STRADDLE: &str = "pcurve_spline_chart_straddle";

/// How many times a kept cell is quartered at most.
const CELL_DEPTH: usize = 48;
/// The levels past the one where the chart's own diagonal, halved at
/// each, is below half the band's zero: a cell's geometric size halves
/// only on average with its parameters'.
const DEPTH_HEADROOM: f64 = 8.0;
/// The kept cells one sample's walk may visit.
const CELL_VISITS: usize = 1 << 12;
/// The cells a walk visits past its first point read in band.
const ON_VISITS: usize = 1 << 5;

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
        None => Ok(Incidence::On),
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
    /// The insertions the cut made, both directions.
    inserted: usize,
    /// The quarterings a walk may make ([`depth_cap`]).
    depth: usize,
    /// At `f64` ([`Rounding`]).
    rounding: Option<Rounding<T>>,
}

/// The `f64` cut's rounding (module docs, *Rounding*).
#[derive(Clone, Copy)]
struct Rounding<T> {
    /// What one blend level charges a point.
    per_level: T,
    /// The cells' points' largest reach from the centre, and the
    /// centre's from the origin.
    reach: T,
}

/// One Bézier cell: its parameter rectangle `[u0, u1] × [v0, v1]`, its
/// control net about the centre in homogeneous form, row-major in `u`,
/// the same net's `f64` shadow (the middle of each coordinate's bracket,
/// cut alongside it), and how many times it was quartered. The shadow
/// orders the walk and decides nothing.
#[derive(Clone)]
struct Cell<T> {
    rect: [f64; 4],
    net: Vec<[T; 4]>,
    shadow: Vec<[f64; 4]>,
    depth: usize,
}

/// A cell in the walk's queue, nearest shadow box first, then the
/// deepest, then the order it was queued in (D9).
struct Queued<T> {
    key: f64,
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
            .total_cmp(&self.key)
            .then_with(|| self.cell.depth.cmp(&other.cell.depth))
            .then_with(|| other.seq.cmp(&self.seq))
    }
}

impl<'a, T: Decide> Net<'a, T> {
    fn of(chart: &'a NurbsSurface<T>, band: Band) -> Option<Self> {
        let (ku, kv) = (chart.knot_values_u(), chart.knot_values_v());
        let (pu, pv) = (ku.degree(), kv.degree());
        let (nu, nv) = chart.control_counts();
        if pu == 0 || pv == 0 || nu == 0 || nv == 0 {
            return None;
        }
        let (control, weights) = (chart.control(), chart.weights());
        let (lo, hi) = shadow_box(control);
        let centre = Vec3::new(
            T::from_computed(lo[0] + (hi[0] - lo[0]) * 0.5),
            T::from_computed(lo[1] + (hi[1] - lo[1]) * 0.5),
            T::from_computed(lo[2] + (hi[2] - lo[2]) * 0.5),
        );
        let homogeneous = |k: usize| {
            let (p, w) = (control[k] - centre, weights[k]);
            [p.x * w, p.y * w, p.z * w, w]
        };
        // Along `v` first, one row per `u` index; then along `u`, one
        // row per refined `v` index.
        let (ku_full, add_u) = (expanded(ku), insertions(ku)?);
        let (kv_full, add_v) = (expanded(kv), insertions(kv)?);
        let rows: Vec<Vec<[T; 4]>> = (0..nu)
            .map(|i| {
                let row = (0..nv).map(|j| homogeneous(i * nv + j)).collect();
                inserted(&kv_full, pv, row, &add_v)
            })
            .collect::<Option<_>>()?;
        let nv = rows.first()?.len();
        let columns: Vec<Vec<[T; 4]>> = (0..nv)
            .map(|j| inserted(&ku_full, pu, rows.iter().map(|r| r[j]).collect(), &add_u))
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
                let shadow = net.iter().map(|h| h.map(middle)).collect();
                cells.push(Cell {
                    rect: [bu[i], bu[i + 1], bv[j], bv[j + 1]],
                    net,
                    shadow,
                    depth: 0,
                });
            }
        }
        let rounding = (T::WITNESS == geom_core::Witness::Inexact).then(|| {
            let (per_level, reach) = rounding(&cells);
            Rounding {
                per_level,
                reach: reach + centre.norm(),
            }
        });
        Some(Self {
            chart,
            centre,
            cells,
            pu,
            pv,
            inserted: add_u.len() + add_v.len(),
            depth: depth_cap(lo, hi, band),
            rounding,
        })
    }

    /// **One sample's walk** (module docs), nearest shadow box first,
    /// so a sample on the chart dives to its cell rather than walking
    /// every cell beside it to the cap. A point of the chart read in band
    /// of the sample shows it within the band, so not off; the walk then
    /// looks only [`ON_VISITS`] cells further for one within the zero
    /// band.
    fn read(&self, q: Point3<T>, band: Band) -> Read<T> {
        let moved = q - self.centre;
        let shadow_q = [moved.x, moved.y, moved.z].map(middle);
        let mut seq = 0usize;
        let mut queue = std::collections::BinaryHeap::new();
        let mut push = |queue: &mut std::collections::BinaryHeap<Queued<T>>, cell: Cell<T>| {
            seq += 1;
            let key = shadow_gap(&cell, shadow_q);
            queue.push(Queued { key, seq, cell });
        };
        for cell in &self.cells {
            push(&mut queue, cell.clone());
        }
        // The least floor over the cells that cleared and over the kept
        // ones, and the least distance of a point past the zero band.
        let (mut floor, mut kept, mut foot) = (None, None, None);
        let mut pending: Option<Indeterminate> = None;
        let mut near: Option<usize> = None;
        let mut unsettled = false;
        let mut visits = 0usize;
        let least = |acc: Option<T>, x: T| Some(acc.map_or(x, |a| a.min(x)));
        while let Some(Queued { cell, .. }) = queue.pop() {
            let gap = self.floor(&cell, moved);
            match decide(HULL, Margin::of(gap), band) {
                Ok(Sign::Positive) => {
                    floor = least(floor, gap);
                    continue;
                }
                Ok(_) => kept = least(kept, gap),
                Err(cause) => keep(&mut pending, cause),
            }
            visits += 1;
            if visits > CELL_VISITS || near.is_some_and(|at| visits > at + ON_VISITS) {
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
                    near.get_or_insert(visits);
                }
            }
            // A poisoned floor or point is not cured by quartering, since
            // the poison is in the net or the sample: the cell is a leaf.
            if cell.depth >= self.depth || gap.is_poison() || distance.is_poison() {
                unsettled = true;
                continue;
            }
            let (a, b) = cell.split(self.pu, self.pv, true);
            for half in [a, b] {
                let (c, d) = half.split(self.pu, self.pv, false);
                push(&mut queue, c);
                push(&mut queue, d);
            }
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
    /// box and slab, less the cut's rounding at `f64`. The poison where a
    /// weight is not shown positive.
    fn floor(&self, cell: &Cell<T>, q: Point3<T>) -> T {
        if !cell.net.iter().all(|h| h[3].structure_positive()) {
            return T::from_f64(f64::NAN);
        }
        let points: Vec<Point3<T>> = cell.net.iter().map(|&h| point(h)).collect();
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
        match self.rounding {
            None => gap,
            Some(Rounding { per_level, reach }) => {
                let levels = self.inserted + cell.depth * (self.pu + self.pv);
                let charge = per_level * T::from_f64(levels as f64)
                    + rounding_charge(T::from_f64(2.0) * (reach + (q - Point3::origin()).norm()));
                (gap - charge).max(T::zero())
            }
        }
    }
}

impl<T: Decide> Cell<T> {
    /// The cell split at the middle of its `u` range (`along_u`) or its
    /// `v` range, by de Casteljau on each row, its shadow alike.
    fn split(&self, pu: usize, pv: usize, along_u: bool) -> (Self, Self) {
        let (n, m) = if along_u { (pu, pv) } else { (pv, pu) };
        let at = |i: usize, k: usize| {
            if along_u {
                i * (pv + 1) + k
            } else {
                k * (pv + 1) + i
            }
        };
        let half = T::from_f64(0.5);
        let mut left = self.net.clone();
        let mut right = self.net.clone();
        for k in 0..=m {
            let mut row: Vec<[T; 4]> = (0..=n).map(|i| self.net[at(i, k)]).collect();
            for level in 0..=n {
                left[at(level, k)] = row[0];
                right[at(n - level, k)] = row[n - level];
                for i in 0..n - level {
                    let (a, b) = (row[i], row[i + 1]);
                    row[i] = core::array::from_fn(|c| T::homogeneous_step(a[c], b[c], half));
                }
            }
        }
        let (shadow_left, shadow_right) = halves_f64(&self.shadow, n, m, &at);
        let [u0, u1, v0, v1] = self.rect;
        let (rect_left, rect_right) = if along_u {
            let mid = u0 + (u1 - u0) * 0.5;
            ([u0, mid, v0, v1], [mid, u1, v0, v1])
        } else {
            let mid = v0 + (v1 - v0) * 0.5;
            ([u0, u1, v0, mid], [u0, u1, mid, v1])
        };
        let depth = self.depth + usize::from(!along_u);
        (
            Self {
                rect: rect_left,
                net: left,
                shadow: shadow_left,
                depth,
            },
            Self {
                rect: rect_right,
                net: right,
                shadow: shadow_right,
                depth,
            },
        )
    }
}

/// The box of the control points' shadows.
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

/// The rounding one blend level charges a point at `f64`, and the
/// points' largest reach from the centre (module docs, *Rounding*).
fn rounding<T: Real>(cells: &[Cell<T>]) -> (T, T) {
    let (mut m_xyz, mut m_w, mut w_min, mut reach) = (T::zero(), T::zero(), None, T::zero());
    for h in cells.iter().flat_map(|c| c.net.iter()) {
        m_xyz = m_xyz.max(h[0].abs()).max(h[1].abs()).max(h[2].abs());
        m_w = m_w.max(h[3].abs());
        w_min = Some(w_min.map_or(h[3], |w: T| w.min(h[3])));
        reach = reach.max((point(*h) - Point3::origin()).norm());
    }
    let w_min = w_min.unwrap_or_else(T::zero);
    let per_level = T::from_f64(3.0_f64.sqrt())
        * (rounding_charge(m_xyz) + reach * rounding_charge(m_w))
        / w_min;
    (per_level, reach)
}

/// The break values of `knots`, one per run, on the value channel.
fn breaks<T: Decide>(knots: &Knots<T>) -> Option<Vec<f64>> {
    let out: Vec<f64> = knots.runs().map(|(v, _)| v.value_channel()).collect();
    out.iter().all(|x| x.is_finite()).then_some(out)
}

/// [`Cell::split`]'s de Casteljau on the shadow, in plain `f64`.
fn halves_f64(
    net: &[[f64; 4]],
    n: usize,
    m: usize,
    at: &dyn Fn(usize, usize) -> usize,
) -> (Vec<[f64; 4]>, Vec<[f64; 4]>) {
    let mut left = net.to_vec();
    let mut right = net.to_vec();
    for k in 0..=m {
        let mut row: Vec<[f64; 4]> = (0..=n).map(|i| net[at(i, k)]).collect();
        for level in 0..=n {
            left[at(level, k)] = row[0];
            right[at(n - level, k)] = row[n - level];
            for i in 0..n - level {
                let (a, b) = (row[i], row[i + 1]);
                row[i] = core::array::from_fn(|c| a[c] + (b[c] - a[c]) * 0.5);
            }
        }
    }
    (left, right)
}

/// The middle of a value's bracket, the shadow's coordinate: NaN for
/// the poison, which orders last.
fn middle<T: SpanLocate>(x: T) -> f64 {
    x.structure_bracket()
        .map_or(f64::NAN, |(lo, hi)| lo + (hi - lo) * 0.5)
}

/// The gap from the shadow sample to the shadow's box: the walk's
/// order. NaN orders last.
fn shadow_gap<T>(cell: &Cell<T>, q: [f64; 3]) -> f64 {
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for h in &cell.shadow {
        for k in 0..3 {
            let x = h[k] / h[3];
            lo[k] = lo[k].min(x);
            hi[k] = hi[k].max(x);
        }
    }
    let d = (0..3)
        .map(|k| (lo[k] - q[k]).max(q[k] - hi[k]).max(0.0).powi(2))
        .sum::<f64>();
    if d.is_nan() { f64::INFINITY } else { d }
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
/// direction; `None` where that direction has no unit (a cell whose
/// diagonals are parallel or a point).
fn slab_gap<T: Real>(points: &[Point3<T>], pu: usize, pv: usize, q: Point3<T>) -> Option<T> {
    let corner = |a: usize, b: usize| points[a * (pv + 1) + b];
    let n = (corner(pu, pv) - corner(0, 0)).cross(corner(0, pv) - corner(pu, 0));
    let n = n / n.norm();
    let height = |p: Point3<T>| (p - Point3::origin()).dot(n);
    let mut heights = points.iter().map(|&p| height(p));
    let first = heights.next()?;
    let (lo, hi) = heights.fold((first, first), |(lo, hi), h| (lo.min(h), hi.max(h)));
    let at = height(q);
    let gap = (lo - at).max(at - hi).max(T::zero()) / n.norm();
    gap.is_bounded().then_some(gap)
}

/// The quarterings a walk may make: [`CELL_DEPTH`], or fewer where the
/// net's diagonal, halved at each level, falls below half the band's
/// zero [`DEPTH_HEADROOM`] levels sooner. Read off the shadow's box, a
/// bound on the work alone: a poisoned point caps nothing.
fn depth_cap(lo: [f64; 3], hi: [f64; 3], band: Band) -> usize {
    let diagonal = (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>().sqrt();
    let levels = (2.0 * diagonal / band.zero()).log2() + DEPTH_HEADROOM;
    if levels.is_finite() && levels >= 0.0 {
        (levels.ceil() as usize).min(CELL_DEPTH)
    } else {
        CELL_DEPTH
    }
}

/// The knot vector's every value, each run repeated by its
/// multiplicity, in the scalar.
fn expanded<T: Decide>(knots: &Knots<T>) -> Vec<T> {
    knots
        .runs()
        .flat_map(|(value, m)| core::iter::repeat_n(value, m))
        .collect()
}

/// The interior knots to insert so each reaches the degree, the breaks
/// of a Bézier cut, in the scalar; `None` past the degree or at a knot
/// with no value.
fn insertions<T: Decide>(knots: &Knots<T>) -> Option<Vec<T>> {
    let p = knots.degree();
    let runs: Vec<(T, usize)> = knots.runs().collect();
    let mut add = Vec::new();
    for &(value, m) in runs.iter().skip(1).take(runs.len().saturating_sub(2)) {
        if m > p || value.value_channel().is_nan() {
            return None;
        }
        add.extend(core::iter::repeat_n(value, p - m));
    }
    Some(add)
}

/// **A homogeneous row with `add` inserted into its knots** (Boehm's
/// insertion, one knot at a time, The NURBS Book A5.1 at `r = 1`): each
/// new point the blend `P[i−1] + (P[i] − P[i−1])·α` of its neighbours,
/// `α = (x − U[i]) / (U[i+p] − U[i])` in the scalar, so an enclosure
/// holds the exact refined row. The span a knot lands in is located on
/// the value channel, a structure read of point knots. `None` where a
/// knot has no value or a span is empty.
fn inserted<T: Decide>(
    knots: &[T],
    p: usize,
    mut row: Vec<[T; 4]>,
    add: &[T],
) -> Option<Vec<[T; 4]>> {
    let mut knots = knots.to_vec();
    for &x in add {
        let at = x.value_channel();
        // The last knot at or below `x`, and how many knots equal it.
        let k = knots.iter().rposition(|u| u.value_channel() <= at)?;
        let s = knots[..=k]
            .iter()
            .rev()
            .take_while(|u| u.value_channel() == at)
            .count();
        if k < p || s > p {
            return None;
        }
        let mut next = Vec::with_capacity(row.len() + 1);
        next.extend_from_slice(&row[..=k - p]);
        for i in k - p + 1..=k - s {
            let alpha = (x - knots[i]) / (knots[i + p] - knots[i]);
            let (a, b) = (row[i - 1], row[i]);
            next.push(core::array::from_fn(|c| {
                T::homogeneous_step(a[c], b[c], alpha)
            }));
        }
        next.extend_from_slice(&row[k - s..]);
        knots.insert(k + 1, x);
        row = next;
    }
    Some(row)
}

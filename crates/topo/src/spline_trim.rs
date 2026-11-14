//! **A face's trim in its spline chart, and a point placed against it**:
//! every loop's chain of rational Bézier pieces, each ring moved into its
//! outer loop's cover by the ring rule, a query read against it by ray
//! parity ([`crate::chain_parity`]), and the chart's Bézier cells that
//! meter the reading. Two doors read it: the boolean's spline arm
//! (`boolean::spline_region`), which places its feet and cells by
//! [`Trim::place`] and walks the cells for the carrier, and the ring
//! door ([`crate::Body::face_lift`]), which reads a spline face's ring
//! decks through [`ring_decks`].
//!
//! # The trim, in the chart
//!
//! The trim is read in the face's chart from C4's derived pcurves, lifted
//! along each loop past its placeholders
//! ([`crate::pcurves::Lift::read_past_placeholders`]: mid-boolean, the
//! sweep's null edges at the crossing vertices), each half-edge's image
//! taken over its edge's span and in its walk's direction, and the
//! chart-space gap at a joint closed by a segment. A loop that winds a
//! closed channel bounds no chart polygon and is not read. Each loop's
//! lift starts on one of its own rows, so on a closed channel a ring is
//! first moved the one whole number of periods that places it in its
//! outer loop's region, decided, or the trim is not read. A query (a
//! foot, or a cell's parameter box) is placed by the ray parity of its
//! centre along `+u`, read off the pieces' control boxes, a piece
//! subdivided until its box is definitely clear of the query. "Clear" is
//! decided in metres, never in chart units: a channel's gap between two
//! boxes crosses the metric door metered by that channel's inf-side arm
//! over a window round the query, and every way out of the window is
//! decided past the band first, since a piece the gap does not hold
//! inside the window is at least that far. So a query within the band of
//! the trim's boundary, along the chart, escalates rather than reading a
//! side. On a chart closed in a channel, the query's copies a period
//! apart are each read in the chart's cover: in where one is in, out
//! where every one is out.
//!
//! # Rounding
//!
//! The chart's cells and the trim's pieces are cut by blends
//! `x + (y − x)·t` (Boehm's insertions, through the kernel's plans, and
//! de Casteljau), and the trim is read in `f64` at every scalar. A box
//! of rounded points can exclude a point the exact box holds, so every
//! box that decides is widened by a first-order bound on its points'
//! rounding: each homogeneous point carries one per channel, set where
//! it is made and carried through every blend ([`blend`]), and is off by
//! at most `(e_c + |P_c|·e_w)/(w − e_w) + u·|P_c|` out of homogeneous
//! form ([`point_rounding`]). The net is held about the middle of the
//! chart's control box, so the cut is in the patch's own size. A cell's
//! floor is less the root sum of its worst per channel and
//! [`geom_core::rounding_charge`] of twice the reach of its points, the
//! moved point and the centre; a weight not shown positive, or a bound that is not finite,
//! leaves it the poison. A piece's box is wider by its worst per channel
//! (no box where one is not finite), a piece taken out of an enclosure by
//! the enclosure's half-width as well, and a run's ends by the image's
//! reach over its span's brackets. So the reading stays one-way: a point
//! is off the carrier, and a query clear of the trim, only past the band
//! and the charge. It is a rounding estimate, the `f64` lane's contract,
//! as [`geom_core::running`]'s is; at an enclosure scalar the cells'
//! arithmetic holds the exact points already and charges nothing.
//! `geom_brep`'s spline incidence test cuts its cells the same way
//! (`work/splice/the-spline-chart-cell-walk-is-written-twice.md`); its
//! floor adds a slab.

use geom::{NurbsCurve2, NurbsSurface, Surface};
use geom_brep::Pcurve;
use geom_core::spline::Knots;
use geom_core::{
    Band, Decide, Indeterminate, InfSpeed, Margin, Point3, Sign, UNIT_ROUNDOFF, Vec3,
    rounding_charge,
};
use std::num::NonZeroUsize;
use std::sync::Arc;

use crate::body::Body;
use crate::chain_parity::{ChainPiece, PieceBox, RayQuery, gap, ray_parity_as};
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopKey};
use crate::joint::Deck;
use crate::joint::Winding;
use crate::live::proven;
use crate::pcurves::{Lift, RingMiss, RingPlacementError, Shift, Side, SplineTrimGap};
use crate::validate::decide;

/// A chart window's inf-side lever arm, gated as a length.
pub(crate) const SPLINE_TRIM_ARM: &str = "spline_trim_arm";
/// A query's clearance from the face's trim along the chart.
pub(crate) const SPLINE_TRIM_CLEAR: &str = "spline_trim_clear";

// The trim's work bounds are the ring rule's, at the crate root
// (`crate::pcurves`): one home for both readers of a point against a
// loop, and the one count for a ring's shifts and a query's copies.
use crate::pcurves::{HALVINGS, PIECE_VISITS, SHIFTS as COPIES, WINDOW_SHARE};

/// A rectangle of a chart's parameters, `u` then `v`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Rect {
    pub(crate) u: (f64, f64),
    pub(crate) v: (f64, f64),
}

impl Rect {
    pub(crate) fn point(u: f64, v: f64) -> Self {
        Self {
            u: (u, u),
            v: (v, v),
        }
    }

    pub(crate) fn centre(self) -> (f64, f64) {
        (geom::bracket_mid(self.u), geom::bracket_mid(self.v))
    }

    fn size(self) -> (f64, f64) {
        (self.u.1 - self.u.0, self.v.1 - self.v.0)
    }

    /// The overlap with `other`, where it has area.
    fn overlap(self, other: Self) -> Option<Self> {
        let u = (self.u.0.max(other.u.0), self.u.1.min(other.u.1));
        let v = (self.v.0.max(other.v.0), self.v.1.min(other.v.1));
        (u.0 < u.1 && v.0 < v.1).then_some(Self { u, v })
    }

    fn shifted(self, du: f64, dv: f64) -> Self {
        Self {
            u: (self.u.0 + du, self.u.1 + du),
            v: (self.v.0 + dv, self.v.1 + dv),
        }
    }
}

/// **One Bézier cell of a chart**: its parameter rectangle and its
/// control net in homogeneous form about `centre`, row-major in `u`,
/// each point with its rounding bound (*Rounding*, module docs).
#[derive(Clone, Debug)]
pub(crate) struct Patch<T: geom_core::Real> {
    pub(crate) rect: Rect,
    pub(crate) pu: usize,
    pub(crate) pv: usize,
    pub(crate) net: Vec<Control<T, 4>>,
    /// The middle of the chart's control box, the origin the net is held
    /// about.
    pub(crate) centre: Vec3<T>,
}

impl<T: Decide> Patch<T> {
    fn at(&self, i: usize, j: usize) -> Control<T, 4> {
        self.net[i * (self.pv + 1) + j]
    }

    /// The control points about the centre, out of homogeneous form.
    fn points(&self) -> impl Iterator<Item = Point3<T>> + '_ {
        self.net
            .iter()
            .map(|(h, _)| Point3::new(h[0] / h[3], h[1] / h[3], h[2] / h[3]))
    }

    /// **A lower bound on the distance from `q` to the cell**: its
    /// distance to the control net's box, which holds the cell (the
    /// convex-hull property, every weight positive), less at `f64` the
    /// cut's rounding and the read's (*Rounding*, module docs).
    pub(crate) fn floor(&self, q: Point3<T>) -> T {
        let poison = T::from_f64(f64::NAN);
        // A weight not shown positive leaves the hull unproven.
        if !self.net.iter().all(|(h, _)| h[3].structure_positive()) {
            return poison;
        }
        let moved = q - self.centre;
        let mut points = self.points();
        let Some(first) = points.next() else {
            return poison;
        };
        let (mut lo, mut hi) = (first, first);
        let mut reach = (first - Point3::origin()).norm();
        for p in points {
            lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
            reach = reach.max((p - Point3::origin()).norm());
        }
        let axis = |lo: T, hi: T, x: T| (lo - x).max(x - hi).max(T::zero());
        let (gx, gy, gz) = (
            axis(lo.x, hi.x, moved.x),
            axis(lo.y, hi.y, moved.y),
            axis(lo.z, hi.z, moved.z),
        );
        let gap = (gx.powi(2) + gy.powi(2) + gz.powi(2)).sqrt();
        if !tracks::<T>() || mutant("spline-trim-charge-dropped") {
            return gap;
        }
        // The worst per channel, folded so a NaN bound stays NaN.
        let mut worst = [0.0f64; 3];
        for control in &self.net {
            let e = point_rounding(control);
            for c in 0..3 {
                if e[c].is_nan() || e[c] > worst[c] {
                    worst[c] = e[c];
                }
            }
        }
        let cut = worst.iter().map(|e| e.powi(2)).sum::<f64>().sqrt();
        if !cut.is_finite() {
            return poison;
        }
        let cut = if mutant("spline-trim-cut-dropped") {
            0.0
        } else {
            cut
        };
        // The read's own rounding, as `geom_brep`'s spline incidence
        // charges it: twice the reach of the points, the moved point and
        // the centre. No fixture makes this term decide alone: near the
        // box the move and the differences are exact or round by `u` of
        // coordinates the cut's charge already bounds by `u·|P_c|` at
        // least; nor the cut's alone, which this covers wherever `q`
        // meets a box face.
        let read = if mutant("spline-trim-read-dropped") {
            T::zero()
        } else {
            rounding_charge(
                T::from_f64(2.0) * (reach + (moved - Point3::origin()).norm() + self.centre.norm()),
            )
        };
        (gap - T::from_computed(cut) - read).max(T::zero())
    }

    /// The diagonal of the control net's box, read on its value channel:
    /// a cell narrower than half the band's zero is not quartered again,
    /// since its centre already stands for every point of it.
    pub(crate) fn extent(&self) -> f64 {
        let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
        for p in self.points() {
            for (k, x) in [p.x, p.y, p.z].into_iter().enumerate() {
                let x = x.value_channel();
                lo[k] = lo[k].min(x);
                hi[k] = hi[k].max(x);
            }
        }
        (0..3).map(|k| (hi[k] - lo[k]).powi(2)).sum::<f64>().sqrt()
    }

    /// The cell split at the fraction `s` of its `u` range (`along_u`)
    /// or its `v` range, by de Casteljau on each row.
    pub(crate) fn split(&self, along_u: bool, s: f64) -> (Self, Self) {
        let (n, m) = if along_u {
            (self.pu, self.pv)
        } else {
            (self.pv, self.pu)
        };
        let mut left = self.clone();
        let mut right = self.clone();
        let st = T::from_computed(s);
        // The exact half is a halving's: its product rounds nothing.
        #[allow(clippy::float_cmp)]
        let ratio = s != 0.5;
        let track = tracks::<T>();
        for k in 0..=m {
            let mut row: Vec<Control<T, 4>> = (0..=n)
                .map(|i| {
                    if along_u {
                        self.at(i, k)
                    } else {
                        self.at(k, i)
                    }
                })
                .collect();
            let mut lo = Vec::with_capacity(n + 1);
            let mut hi = Vec::with_capacity(n + 1);
            for level in 0..=n {
                lo.push(row[0]);
                hi.push(row[n - level]);
                for i in 0..n - level {
                    row[i] = blend(row[i], row[i + 1], st, ratio, track);
                }
            }
            hi.reverse();
            for i in 0..=n {
                let idx = if along_u {
                    i * (self.pv + 1) + k
                } else {
                    k * (self.pv + 1) + i
                };
                left.net[idx] = lo[i];
                right.net[idx] = hi[i];
            }
        }
        let (r0, r1) = if along_u { self.rect.u } else { self.rect.v };
        let cut = r0 + (r1 - r0) * s;
        if along_u {
            left.rect.u = (r0, cut);
            right.rect.u = (cut, r1);
        } else {
            left.rect.v = (r0, cut);
            right.rect.v = (cut, r1);
        }
        (left, right)
    }

    /// The four quarters, `((u-lo v-lo, u-lo v-hi), (u-hi v-lo, u-hi v-hi))`.
    pub(crate) fn quartered(&self) -> ((Self, Self), (Self, Self)) {
        let (a, b) = self.split(true, 0.5);
        (a.split(false, 0.5), b.split(false, 0.5))
    }

    /// The cell restricted to `rect`, a rectangle inside its own.
    pub(crate) fn restricted(&self, rect: Rect) -> Self {
        let mut cell = self.clone();
        for along_u in [true, false] {
            let ((r0, r1), (w0, w1)) = if along_u {
                (cell.rect.u, rect.u)
            } else {
                (cell.rect.v, rect.v)
            };
            if w0 > r0 {
                cell = cell.split(along_u, (w0 - r0) / (r1 - r0)).1;
            }
            let (r0, r1) = if along_u { cell.rect.u } else { cell.rect.v };
            if w1 < r1 {
                cell = cell.split(along_u, (w1 - r0) / (r1 - r0)).0;
            }
        }
        cell
    }

    /// The cell as a surface of its own, on its own parameter rectangle,
    /// held about the centre: a translate of the chart's cell, so its
    /// speeds are the chart's.
    fn surface(&self) -> Option<Surface<T>> {
        let knots = |(a, b): (f64, f64), p: usize| {
            let ends = NonZeroUsize::new(p + 1)?;
            Knots::new(
                vec![(T::from_computed(a), ends), (T::from_computed(b), ends)],
                p,
            )
            .ok()
        };
        let surface = NurbsSurface::new(
            knots(self.rect.u, self.pu)?,
            knots(self.rect.v, self.pv)?,
            self.points().collect(),
            self.net.iter().map(|(h, _)| h[3]).collect(),
        )
        .ok()?;
        Some(Surface::Nurbs(Arc::new(surface)))
    }
}

/// A homogeneous control point and, at `f64`, a first-order bound on
/// each channel's accumulated rounding (*Rounding*, module docs); the
/// weight is the last channel.
pub(crate) type Control<T, const N: usize> = ([T; N], [f64; N]);

/// Whether the scalar's arithmetic rounds uncharged, so a cut carries
/// rounding bounds: `f64`, not an enclosure.
fn tracks<T: Decide>() -> bool {
    T::WITNESS == geom_core::Witness::Inexact
}

/// **One blend `x + (y − x)·t`** of two homogeneous control points, each
/// channel by [`geom_core::spline::SpanLocate::homogeneous_step`], with its rounding bound
/// when `track`ed: the inputs' bounds blended as the values are, plus
/// `u·|y − x|·t` for the difference, `u·|result|` for the sum, and, at a
/// ratio that is not the exact half (`ratio`), `u·|y − x|·t` more for the
/// product and `|y − x|·δt` for `t`'s own rounding (three units of it).
fn blend<T: Decide, const N: usize>(
    x: Control<T, N>,
    y: Control<T, N>,
    t: T,
    ratio: bool,
    track: bool,
) -> Control<T, N> {
    let h: [T; N] = core::array::from_fn(|c| T::homogeneous_step(x.0[c], y.0[c], t));
    if !track {
        return (h, [0.0; N]);
    }
    let tf = middle(t);
    let (dt, roundings) = if ratio && !mutant("spline-trim-ratio-uncharged") {
        (3.0 * UNIT_ROUNDOFF * tf.abs(), 2.0)
    } else {
        (0.0, 1.0)
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

/// One control point's rounding out of homogeneous form, per coordinate:
/// `(e_c + |P_c|·e_w)/(w − e_w) + u·|P_c|`, infinite where the weight's
/// bound reaches the weight; the weight's own channel reads zero. A
/// first-order bound: measured against double-double cuts, the worst
/// drift is within 5% of it (`spline_region_rows`). At an enclosure scalar
/// the bounds carried are zero, and no floor reads this.
pub(crate) fn point_rounding<T: Decide, const N: usize>((h, e): &Control<T, N>) -> [f64; N] {
    let w = middle(h[N - 1]);
    let lead = w - e[N - 1];
    if lead.partial_cmp(&0.0) != Some(core::cmp::Ordering::Greater) {
        return [f64::INFINITY; N];
    }
    let scale = if mutant("spline-trim-bound-halved") {
        0.5
    } else {
        1.0
    };
    core::array::from_fn(|c| {
        if c == N - 1 {
            return 0.0;
        }
        let p = (middle(h[c]) / w).abs();
        scale * ((e[c] + p * e[N - 1]) / lead + UNIT_ROUNDOFF * p)
    })
}

/// The middle of a value's bracket, the value an `f64` rounding bound is
/// read at; NaN for the poison.
fn middle<T: Decide>(x: T) -> f64 {
    x.structure_bracket().map_or(f64::NAN, geom::bracket_mid)
}

/// **A homogeneous row refined along `knots`** by inserting `add`,
/// through the kernel's insertion plans
/// ([`geom_core::spline::plan_inputs::refine_plan`]) applied in the
/// scalar ([`geom_core::spline::CurvePlan::apply_homogeneous`]), each
/// step a [`blend`], so an enclosure holds the exact refined row and the
/// `f64` bound follows the arithmetic. Insertion keeps the knots' breaks.
/// `None` where a knot has no value or a plan refuses.
fn refined<T: Decide, const N: usize>(
    knots: &Knots<T>,
    row: Vec<Control<T, N>>,
    add: &[f64],
) -> Option<Vec<Control<T, N>>> {
    if add.is_empty() {
        return Some(row);
    }
    let track = tracks::<T>();
    let runs: Vec<(T, usize)> = knots.runs().collect();
    // Every knot an insertion meets is a held break or one of `add`.
    let held = |v: f64| {
        runs.iter()
            .find(|(value, _)| value.value_channel() == v)
            .map_or_else(|| T::from_computed(v), |(value, _)| *value)
    };
    let unit = geom_core::spline::Weights::new(vec![T::one(); row.len()]).ok()?;
    let plans = geom_core::spline::plan_inputs::refine_plan(knots, &unit, add).ok()?;
    let mut values: Vec<T> = knots.values().to_vec();
    let poison: Control<T, N> = ([T::from_f64(f64::NAN); N], [f64::NAN; N]);
    let mut row = row;
    for plan in &plans {
        let pairs: Vec<(Control<T, N>, T)> = row.iter().map(|c| (*c, c.0[N - 1])).collect();
        row = plan
            .apply_homogeneous(
                &values,
                &pairs,
                ((poison, poison.0[N - 1]), &held),
                |x, y, t| blend(x, y, t, true, track),
            )
            .into_iter()
            .map(|(c, _)| c)
            .collect();
        values = plan.knots().knots().iter().map(|&v| held(v)).collect();
    }
    (row.iter().all(|(h, _)| !h[N - 1].is_poison())).then_some(row)
}

/// **The chart's Bézier cells**, ascending in `u` then `v`; `None` where
/// the decomposition does not run. Each is the chart's own net, moved
/// about the middle of its control box and refined to full multiplicity
/// at every break ([`refined`]: along `v`, one row per `u` index, then
/// along `u`, one row per refined `v` index), held at the door's scalar:
/// `geom_brep::patch_bound::patch_cells` reads an `f64` chart and
/// `PatchSpans::decompose` one interval channel, and this door is
/// generic over [`Decide`]. `geom_brep`'s spline incidence test cuts the
/// same way with the same charge
/// (`work/splice/the-spline-chart-cell-walk-is-written-twice.md`).
pub(crate) fn bezier_cells<T: Decide>(chart: &NurbsSurface<T>) -> Option<Vec<Patch<T>>> {
    let (ku, kv) = (chart.knot_values_u(), chart.knot_values_v());
    let (pu, pv) = (ku.degree(), kv.degree());
    let (nu, nv) = chart.control_counts();
    if pu == 0 || pv == 0 || nu == 0 || nv == 0 {
        return None;
    }
    let track = tracks::<T>();
    let (control, weights) = (chart.control(), chart.weights());
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    for p in control {
        for (k, x) in [p.x, p.y, p.z].into_iter().enumerate() {
            lo[k] = lo[k].min(middle(x));
            hi[k] = hi[k].max(middle(x));
        }
    }
    let centre = Vec3::new(
        T::from_computed(geom::bracket_mid((lo[0], hi[0]))),
        T::from_computed(geom::bracket_mid((lo[1], hi[1]))),
        T::from_computed(geom::bracket_mid((lo[2], hi[2]))),
    );
    // Moved and out to homogeneous form: the difference and the product
    // round each coordinate by `u` of it.
    let homogeneous = |k: usize| -> Control<T, 4> {
        let (p, w) = (control[k] - centre, weights[k]);
        let h = [p.x * w, p.y * w, p.z * w, w];
        let e = core::array::from_fn(|c| {
            if track && c < 3 {
                2.0 * UNIT_ROUNDOFF * middle(h[c]).abs()
            } else {
                0.0
            }
        });
        (h, e)
    };
    let rows: Vec<Vec<Control<T, 4>>> = (0..nu)
        .map(|i| {
            refined(
                kv,
                (0..nv).map(|j| homogeneous(i * nv + j)).collect(),
                &bezier_insertions(kv)?,
            )
        })
        .collect::<Option<_>>()?;
    let nv = rows.first()?.len();
    let add_u = bezier_insertions(ku)?;
    let columns: Vec<Vec<Control<T, 4>>> = (0..nv)
        .map(|j| refined(ku, rows.iter().map(|r| r[j]).collect(), &add_u))
        .collect::<Option<_>>()?;
    let nu = columns.first()?.len();
    // The hull needs every weight shown positive: a net cut through
    // weights not shown so (an enclosure reaching zero) is not read.
    if !columns
        .iter()
        .flatten()
        .all(|(h, _)| h[3].structure_positive())
    {
        return None;
    }
    let (bu, bv) = (ku.breakpoint_values()?, kv.breakpoint_values()?);
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
            cells.push(Patch {
                rect: Rect {
                    u: (bu[i], bu[i + 1]),
                    v: (bv[j], bv[j + 1]),
                },
                pu,
                pv,
                net,
                centre,
            });
        }
    }
    Some(cells)
}

/// Whether the mutant `site` is named at compile time
/// (`SPLINE_TRIM_MUTANT`, `option_env!` because
/// `scripts/gates/no-ambient-env.sh` forbids a runtime read): the
/// charges read wrong one at a time, each refused by a row in
/// `spline_region_rows`.
fn mutant(site: &str) -> bool {
    #[cfg(test)]
    {
        debug_assert!(MUTANTS.contains(&site), "an unlisted mutant site: {site}");
        option_env!("SPLINE_TRIM_MUTANT") == Some(site)
    }
    #[cfg(not(test))]
    {
        let _ = site;
        false
    }
}

/// Every site [`mutant`] answers for.
#[cfg(test)]
const MUTANTS: [&str; 10] = [
    "spline-trim-charge-dropped",
    "spline-trim-cut-dropped",
    "spline-trim-read-dropped",
    "spline-trim-ratio-uncharged",
    "spline-trim-bound-halved",
    "spline-trim-trim-uncharged",
    "spline-trim-u-unwidened",
    "spline-trim-halfwidth-dropped",
    "spline-trim-shift-untracked",
    "spline-trim-iso-end-uncharged",
];

/// The interior knots of `knots` to insert so each reaches multiplicity
/// `p`, the breaks of a Bézier decomposition; `None` past `p` or at a
/// knot with no value.
fn bezier_insertions<T: Decide>(knots: &Knots<T>) -> Option<Vec<f64>> {
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
    Some(add)
}

/// **One piece of the trim's chain**: a rational Bézier curve in the
/// chart, its control points in homogeneous form, each with its rounding
/// bound (*Rounding*, module docs).
#[derive(Clone, Debug)]
pub(crate) struct Piece {
    pub(crate) net: Vec<Control<f64, 3>>,
}

impl Piece {
    pub(crate) fn segment(a: Control<f64, 3>, b: Control<f64, 3>) -> Self {
        Self { net: vec![a, b] }
    }

    /// The chart point `p`, each coordinate off by at most `e`.
    pub(crate) fn vertex(p: [f64; 2], e: [f64; 2]) -> Control<f64, 3> {
        ([p[0], p[1], 1.0], [e[0], e[1], 0.0])
    }

    fn point((h, _): Control<f64, 3>) -> [f64; 2] {
        [h[0] / h[2], h[1] / h[2]]
    }

    pub(crate) fn first(&self) -> Control<f64, 3> {
        self.net[0]
    }

    pub(crate) fn last(&self) -> Control<f64, 3> {
        self.net[self.net.len() - 1]
    }

    fn ends(&self) -> ([f64; 2], [f64; 2]) {
        (Self::point(self.first()), Self::point(self.last()))
    }

    /// The control points' box, which holds the piece, widened on each
    /// side by the worst rounding of a point; `None` for a point with no
    /// value.
    pub(crate) fn bbox(&self) -> Option<Rect> {
        let mut r = Rect::point(f64::INFINITY, f64::INFINITY);
        r.u.1 = f64::NEG_INFINITY;
        r.v.1 = f64::NEG_INFINITY;
        let mut worst = [0.0f64; 2];
        for control in &self.net {
            let [x, y] = Self::point(*control);
            if !x.is_finite() || !y.is_finite() {
                return None;
            }
            r.u = (r.u.0.min(x), r.u.1.max(x));
            r.v = (r.v.0.min(y), r.v.1.max(y));
            let e = point_rounding(control);
            if !e.iter().all(|x| x.is_finite()) {
                return None;
            }
            worst = [worst[0].max(e[0]), worst[1].max(e[1])];
        }
        if mutant("spline-trim-trim-uncharged") {
            return Some(r);
        }
        if mutant("spline-trim-u-unwidened") {
            worst[0] = 0.0;
        }
        Some(Rect {
            u: (r.u.0 - worst[0], r.u.1 + worst[0]),
            v: (r.v.0 - worst[1], r.v.1 + worst[1]),
        })
    }

    /// The two halves of the piece's parameter, by de Casteljau.
    pub(crate) fn halves(&self) -> (Self, Self) {
        let n = self.net.len() - 1;
        let mut row = self.net.clone();
        let (mut lo, mut hi) = (Vec::with_capacity(n + 1), Vec::with_capacity(n + 1));
        for level in 0..=n {
            lo.push(row[0]);
            hi.push(row[n - level]);
            for i in 0..n - level {
                row[i] = blend(row[i], row[i + 1], 0.5, false, true);
            }
        }
        hi.reverse();
        (Self { net: lo }, Self { net: hi })
    }

    fn reversed(mut self) -> Self {
        self.net.reverse();
        self
    }

    /// The piece moved by `(du, dv)`: `h_c + d_c·w`, a product and a sum.
    pub(crate) fn shifted(&self, du: f64, dv: f64) -> Self {
        let step = |x: f64, ex: f64, d: f64, w: f64, ew: f64| {
            let r = x + d * w;
            if mutant("spline-trim-shift-untracked") {
                return (r, ex);
            }
            (
                r,
                ex + d.abs() * ew + UNIT_ROUNDOFF * ((d * w).abs() + r.abs()),
            )
        };
        Self {
            net: self
                .net
                .iter()
                .map(|&(h, e)| {
                    let (x, ex) = step(h[0], e[0], du, h[2], e[2]);
                    let (y, ey) = step(h[1], e[1], dv, h[2], e[2]);
                    ([x, y, h[2]], [ex, ey, e[2]])
                })
                .collect(),
        }
    }
}

/// **A face's trim in its spline chart** (module docs): every loop's
/// chain, each ring lifted into the outer loop's cover.
pub(crate) struct Trim {
    pieces: Vec<Piece>,
    /// The box of every piece.
    reach: Rect,
    /// The chart's knot domain.
    domain: Rect,
    /// The chart's period in each channel it is decided closed in.
    periods: [Option<f64>; 2],
    /// The piece visits one query's reading may make.
    piece_visits: usize,
}

/// **The chart a trim is read in**: its periods, its knot domain and
/// the scale its images' ends are charged at.
struct TrimChart {
    /// The chart's period in each channel it is decided closed in.
    periods: [Option<f64>; 2],
    /// The chart's knot domain.
    domain: Rect,
    /// The chart coordinates' magnitude an image's evaluation rounds at:
    /// the domain's, and a period past it.
    scale: f64,
}

impl TrimChart {
    /// The chart of `surface`, a spline `chart`; [`SplineTrimGap::Periods`]
    /// where a channel the chart is decided closed in states no period or
    /// another one.
    fn of<T: Decide>(
        surface: &Surface<T>,
        chart: &NurbsSurface<T>,
        band: Band,
    ) -> Result<Self, SplineTrimGap> {
        // The lift moves a row by the periods the chart STATES
        // ([`crate::pcurves::Lift::read`]), and a copy of a query is read a
        // period apart only on a channel the chart is DECIDED closed in.
        // The two agree wherever an element carries a period, since only
        // a net decided closed carries one; a channel where they part is
        // refused below, row by row.
        let (stated, owned) = (
            crate::pcurves::ChartPeriods::stated(surface),
            crate::pcurves::ChartPeriods::of(surface, band),
        );
        let mut periods = [None, None];
        for (k, (s, o)) in [(stated.u(), owned.u()), (stated.v(), owned.v())]
            .into_iter()
            .enumerate()
        {
            if let Some(o) = o {
                let s = s.ok_or(SplineTrimGap::Periods)?.length().value_channel();
                #[allow(clippy::float_cmp)] // one knot domain's length, read twice
                let agree = s == o.length().value_channel();
                if !agree {
                    return Err(SplineTrimGap::Periods);
                }
                periods[k] = Some(s);
            }
        }
        let (u0, u1) = chart.knot_values_u().domain();
        let (v0, v1) = chart.knot_values_v().domain();
        let domain = Rect {
            u: (u0.value_channel(), u1.value_channel()),
            v: (v0.value_channel(), v1.value_channel()),
        };
        let scale = [domain.u.0, domain.u.1, domain.v.0, domain.v.1]
            .into_iter()
            .chain(periods.iter().flatten().copied())
            .map(f64::abs)
            .fold(0.0, f64::max);
        Ok(Self {
            periods,
            domain,
            scale,
        })
    }

    /// **A loop's chain in this chart**: each row of `lift` as its image's
    /// pieces over its edge's span, in its walk's direction, and the gap
    /// at each joint closed by a segment.
    ///
    /// # Errors
    ///
    /// [`SplineTrimGap`]: the lift is empty, lacks a joint element or
    /// winds; a row is moved by a period on a channel the chart is not
    /// decided closed in, or onto the twin sheet; or an image has no
    /// piece reading.
    fn chain<T: Decide>(
        &self,
        body: &Body<T>,
        lift: &Lift<T>,
    ) -> Result<Vec<Piece>, SplineTrimGap> {
        if lift.is_empty() {
            return Err(SplineTrimGap::NoClosedLift);
        }
        // A skipped placeholder's joints are the identity
        // (`Lift::read_past_placeholders`), so the members' elements
        // wind as the whole loop's.
        let elements: Option<Vec<_>> = lift.iter().map(|r| body.joint(r.half_edge)).collect();
        if !Winding::of(elements.ok_or(SplineTrimGap::NoClosedLift)?).is_zero() {
            return Err(SplineTrimGap::NoClosedLift);
        }
        let mut chain: Vec<Piece> = Vec::new();
        for row in lift {
            if (row.lift.u != 0 && self.periods[0].is_none())
                || (row.lift.v != 0 && self.periods[1].is_none())
                || row.lift.twin
            {
                return Err(SplineTrimGap::Moved);
            }
            let (t0, t1) = row.params;
            let mut run =
                image_pieces(&row.pcurve, t0, t1, self.scale).ok_or(SplineTrimGap::NoPieces)?;
            if !walks_forward(body, row.half_edge) {
                run = run.into_iter().rev().map(Piece::reversed).collect();
            }
            close_gap(
                &mut chain,
                run.first().ok_or(SplineTrimGap::NoPieces)?.first(),
            );
            chain.extend(run);
        }
        let start = chain.first().ok_or(SplineTrimGap::NoPieces)?.first();
        close_gap(&mut chain, start);
        Ok(chain)
    }
}

/// **The ring door's spline arm** ([`crate::Body::face_lift`]): the deck
/// that moves each of `rings` (each ring and its lift as read) into the
/// region `outer`, the lift of `face`'s outer loop, bounds, decided by
/// the ring rule against the outer loop's trim alone
/// ([`Trim::ring_shift`]), the one the trim places its own rings by.
///
/// The door's lifts are [`Lift::read`]'s, the trim's own
/// [`Lift::read_past_placeholders`]'s. At rest the two are one lift, so
/// a ring's deck here is the one [`Trim::with_rings`] moves it by. A loop
/// through a placeholder has no `Lift::read`, so on such a face the door
/// answers [`RingPlacementError::Gap`] before this arm is reached.
///
/// # Errors
///
/// [`RingPlacementError::SplineTrimUnread`] where the chart, the outer
/// loop's trim or a ring's chain is not read, with its cause; the rule's miss for the first ring it
/// places nowhere, or more than once.
pub(crate) fn ring_decks<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    outer: &Lift<T>,
    rings: &[(LoopKey, &Lift<T>)],
    band: Band,
) -> Result<Vec<Deck>, RingPlacementError> {
    let surface = body.proven_face(face).surface();
    let Some(chart) = surface.spline_chart() else {
        unreachable!("the spline arm is reached on a spline chart")
    };
    let unread =
        |ring: Option<LoopKey>| move |cause| RingPlacementError::SplineTrimUnread { ring, cause };
    if surface.is_placeholder_chart() {
        return Err(unread(None)(SplineTrimGap::Placeholder));
    }
    let cells = bezier_cells(chart).ok_or_else(|| unread(None)(SplineTrimGap::NoCells))?;
    let sheet = TrimChart::of(surface, chart, band).map_err(unread(None))?;
    let trim = Trim::from_pieces(
        sheet.chain(body, outer).map_err(unread(None))?,
        sheet.domain,
        sheet.periods,
        PIECE_VISITS,
    )
    .ok_or_else(|| unread(None)(SplineTrimGap::NoBox))?;
    rings
        .iter()
        .map(|&(ring, lift)| {
            let chain = sheet.chain(body, lift).map_err(unread(Some(ring)))?;
            let Some(first) = chain.first() else {
                unreachable!("a chain read is not empty")
            };
            trim.ring_shift(&cells, first.ends().0, band)
                .map(|shift| shift.deck)
                .map_err(|miss| miss.of(ring))
        })
        .collect()
}

impl Trim {
    /// The trim of `face` on `chart` (`cells` its Bézier decomposition);
    /// `None` where a loop's chain is not read past its placeholders
    /// ([`TrimChart::chain`]), the chart states no period or another one
    /// on a channel it is decided closed in, or a ring's place in the
    /// outer loop's cover is not decided.
    pub(crate) fn of<T: Decide>(
        body: &Body<T>,
        face: FaceKey,
        chart: &NurbsSurface<T>,
        cells: &[Patch<T>],
        piece_visits: usize,
        band: Band,
    ) -> Option<Self> {
        let view = body.proven_face(face);
        let sheet = TrimChart::of(view.surface(), chart, band).ok()?;
        let mut chains = Vec::new();
        for (lk, _, _) in view.loops() {
            let lift = Lift::read_past_placeholders(body, lk).ok()?;
            chains.push(sheet.chain(body, &lift).ok()?);
        }
        let mut chains = chains.into_iter();
        let outer = Self::from_pieces(chains.next()?, sheet.domain, sheet.periods, piece_visits)?;
        let rings: Vec<Vec<Piece>> = chains.collect();
        outer.with_rings(cells, rings, band)
    }

    /// The trim the chain `pieces` bounds, with no ring.
    pub(crate) fn from_pieces(
        pieces: Vec<Piece>,
        domain: Rect,
        periods: [Option<f64>; 2],
        piece_visits: usize,
    ) -> Option<Self> {
        let mut reach = pieces.first()?.bbox()?;
        for p in &pieces {
            let b = p.bbox()?;
            reach = Rect {
                u: (reach.u.0.min(b.u.0), reach.u.1.max(b.u.1)),
                v: (reach.v.0.min(b.v.0), reach.v.1.max(b.v.1)),
            };
        }
        Some(Self {
            pieces,
            reach,
            domain,
            periods,
            piece_visits,
        })
    }

    /// **This outer trim with `rings` in its cover**, each moved by the
    /// ring rule ([`crate::pcurves::place_ring`]) with this trim's own
    /// reading of a point in the cover ([`Self::place_in_cover`]); `None`
    /// where a ring has no place.
    pub(crate) fn with_rings<T: Decide>(
        self,
        cells: &[Patch<T>],
        rings: Vec<Vec<Piece>>,
        band: Band,
    ) -> Option<Self> {
        let mut pieces = self.pieces.clone();
        for ring in rings {
            let shift = self.ring_shift(cells, ring.first()?.ends().0, band).ok()?;
            let [du, dv] = shift.offset;
            pieces.extend(ring.iter().map(|p| p.shifted(du, dv)));
        }
        Self::from_pieces(pieces, self.domain, self.periods, self.piece_visits)
    }

    /// **The ring rule over this trim** ([`crate::pcurves::place_ring`]):
    /// the one shift that puts a ring whose chain starts at `at` in this
    /// trim's region, each candidate read in the cover by
    /// [`Self::place_in_cover`].
    pub(crate) fn ring_shift<T: Decide>(
        &self,
        cells: &[Patch<T>],
        [u, v]: [f64; 2],
        band: Band,
    ) -> Result<Shift, RingMiss> {
        crate::pcurves::place_ring(
            self.periods,
            [self.reach.u, self.reach.v],
            [u, v],
            |shift| {
                let [du, dv] = shift.offset;
                self.place_in_cover(cells, Rect::point(u + du, v + dv), band)
            },
        )
    }

    /// **Where the query `q` lies against the trim** (module docs): `In`
    /// or `Out` only where every piece is decided clear of it, along the
    /// chart, past the band, in every copy of `q` a period apart on each
    /// closed channel. `cells` is the chart's Bézier decomposition, which
    /// the window's arms are read off.
    pub(crate) fn place<T: Decide>(&self, cells: &[Patch<T>], q: Rect, band: Band) -> Side {
        self.read_window(cells, q, band, true)
    }

    /// [`Self::place`] of `q` alone, in the cover, with no copy.
    fn place_in_cover<T: Decide>(&self, cells: &[Patch<T>], q: Rect, band: Band) -> Side {
        self.read_window(cells, q, band, false)
    }

    /// The window round `q` its reading is metered over: its arms, then
    /// its way out, then the copies (`copies`) or `q` alone.
    fn read_window<T: Decide>(
        &self,
        cells: &[Patch<T>],
        at: Rect,
        band: Band,
        copies: bool,
    ) -> Side {
        // The window is the chart's, read at the query's copy in the
        // domain; the parity is the cover's, read at the query itself.
        let q = self.principal(at);
        let (du, dv) = self.domain.size();
        let (su, sv) = q.size();
        let reach = (
            if su > 0.0 { su } else { du / WINDOW_SHARE },
            if sv > 0.0 { sv } else { dv / WINDOW_SHARE },
        );
        let mut pending = None;
        for halving in 0..HALVINGS {
            let scale = f64::from(1u32 << halving);
            let (ru, rv) = (reach.0 / scale, reach.1 / scale);
            let window = Rect {
                u: (q.u.0 - ru, q.u.1 + ru),
                v: (q.v.0 - rv, q.v.1 + rv),
            };
            let Some(window) = window.overlap(self.domain) else {
                return Side::Near(None);
            };
            let arms = match window_arms(cells, window, band) {
                Ok(arms) => arms,
                Err(diag) => {
                    pending = pending.or(diag);
                    continue;
                }
            };
            // A path from `q` out of the window is at least the window's
            // way out long, so every piece's clearance is at most that:
            // it is decided past the band once, here. A narrower window
            // only shortens it, so past here a halving is for an arm
            // alone.
            if let Err(diag) = self.exits_clear(q, window, arms, band) {
                return Side::Near(diag.or(pending));
            }
            return if copies {
                self.copies(at, arms, band)
            } else {
                self.copy(at, arms, band)
            };
        }
        Side::Near(pending)
    }

    /// `q`'s copy whose centre lies in the domain, a whole number of
    /// periods along each closed channel.
    fn principal(&self, q: Rect) -> Rect {
        let (cu, cv) = q.centre();
        let shift = |k: usize, c: f64, (d0, _): (f64, f64)| {
            self.periods[k]
                .filter(|p| *p > 0.0)
                .map_or(0.0, |p| ((c - d0) / p).floor() * p)
        };
        q.shifted(-shift(0, cu, self.domain.u), -shift(1, cv, self.domain.v))
    }

    /// Whether every way from `q` out of `window` is decided longer than
    /// the band along the chart: each side of the window inside the
    /// domain, or on a closed channel's domain side, metered by its
    /// channel's arm. `Err` with the in-band side, if any.
    fn exits_clear<T: Decide>(
        &self,
        q: Rect,
        window: Rect,
        (au, av): (InfSpeed<T>, InfSpeed<T>),
        band: Band,
    ) -> Result<(), Option<Indeterminate>> {
        let sides = [
            (window.u.0, self.domain.u.0, q.u.0 - window.u.0, au, 0),
            (window.u.1, self.domain.u.1, window.u.1 - q.u.1, au, 0),
            (window.v.0, self.domain.v.0, q.v.0 - window.v.0, av, 1),
            (window.v.1, self.domain.v.1, window.v.1 - q.v.1, av, 1),
        ];
        for (side, end, room, arm, channel) in sides {
            #[allow(clippy::float_cmp)] // the clip left the window on the domain's own side
            let on_end = side == end;
            if on_end && self.periods[channel].is_none() {
                continue;
            }
            match decide(
                SPLINE_TRIM_CLEAR,
                Margin::metered(T::from_computed(room.max(0.0)), arm),
                band,
            ) {
                Ok(Sign::Positive) => {}
                Ok(_) => return Err(None),
                Err(diag) => return Err(Some(diag)),
            }
        }
        Ok(())
    }

    /// [`Self::place`] over the query's copies a period apart on each
    /// closed channel: in where one is in, out where every one is out.
    fn copies<T: Decide>(&self, q: Rect, arms: (InfSpeed<T>, InfSpeed<T>), band: Band) -> Side {
        let shifts = |channel: usize, (q0, q1): (f64, f64), (r0, r1): (f64, f64)| -> Vec<f64> {
            let Some(p) = self.periods[channel].filter(|p| *p > 0.0) else {
                return vec![0.0];
            };
            let lo = ((r0 - q1) / p).floor() - 1.0;
            let hi = ((r1 - q0) / p).ceil() + 1.0;
            let mut k = lo;
            let mut out = Vec::new();
            while k <= hi && out.len() <= COPIES {
                out.push(k * p);
                k += 1.0;
            }
            out
        };
        let (us, vs) = (shifts(0, q.u, self.reach.u), shifts(1, q.v, self.reach.v));
        if us.len() * vs.len() > COPIES {
            return Side::Near(None);
        }
        let mut out = true;
        let mut near = None;
        for &du in &us {
            for &dv in &vs {
                match self.copy(q.shifted(du, dv), arms, band) {
                    Side::In => return Side::In,
                    Side::Out => {}
                    Side::Near(diag) => {
                        out = false;
                        near = near.or(diag);
                    }
                }
            }
        }
        if out { Side::Out } else { Side::Near(near) }
    }

    /// One copy of the query in the cover: the ray parity of its centre
    /// along `+u` ([`ray_parity_as`]), every piece decided clear of it
    /// first.
    fn copy<T: Decide>(&self, q: Rect, arms: (InfSpeed<T>, InfSpeed<T>), band: Band) -> Side {
        ray_parity_as(
            SPLINE_TRIM_CLEAR,
            &self.pieces,
            q,
            arms,
            self.piece_visits,
            band,
        )
    }
}

impl<T: Decide> ChainPiece<T> for Piece {
    type Query = Rect;

    /// A foot's point, or a cell's parameter box, its ray leaving from
    /// the box's centre.
    fn ray(q: Rect) -> RayQuery {
        let (cu, cv) = q.centre();
        RayQuery {
            centre: [cu, cv],
            rect: [q.u, q.v],
        }
    }

    fn boxed(&self, q: Rect) -> Option<PieceBox<T>> {
        let b = self.bbox()?;
        Some(PieceBox {
            gap: [
                T::from_computed(gap(q.u, b.u)),
                T::from_computed(gap(q.v, b.v)),
            ],
            extent: [b.u, b.v],
        })
    }

    /// A shared end is bit-identical: the Bézier pieces of one image
    /// share the control point of the one refined row between them, the
    /// halves the one de Casteljau midpoint, a joint's segment copies the
    /// ends either side ([`close_gap`]), and a ring's shift moves each
    /// copy by the one arithmetic.
    fn ends(&self) -> ([f64; 2], [f64; 2]) {
        Piece::ends(self)
    }

    fn halves(&self) -> (Self, Self) {
        Piece::halves(self)
    }
}

/// Close the chain's gap to `to`: a segment from its last end, where
/// that end is not `to` itself in homogeneous form, so the computed chain
/// is continuous (a query clear of every widened box reads the parity of
/// the exact chain, *Rounding*).
fn close_gap(chain: &mut Vec<Piece>, to: Control<f64, 3>) {
    if let Some(last) = chain.last() {
        let from = last.last();
        #[allow(clippy::float_cmp)] // one control point, read twice
        if from.0 != to.0 {
            chain.push(Piece::segment(from, to));
        }
    }
}

/// Whether the loop walks `he`'s edge forward in the carrier parameter.
fn walks_forward<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> bool {
    let edge = proven(&body.half_edges, he, EntityId::HalfEdge).edge;
    proven(&body.edges, edge, EntityId::Edge).he_plus == he
}

/// **An image's pieces over its edge's span `[t0, t1]`**, forward: an
/// iso line or an iso arc is the segment between its two ends (its image
/// is that segment, whatever its parameterization; its ends as
/// [`Pcurve::span_ends`] reads them), a fitted or general
/// image its Bézier pieces over the span. `None` for any other kind, or
/// where the decomposition does not run. An end is held at its
/// bracket's middle, off by the bracket's half-width and, at `f64`, by a
/// short chain's rounding at `scale` past it (`p0 + d·g`).
pub(crate) fn image_pieces<T: Decide>(
    pcurve: &Pcurve<T>,
    t0: T,
    t1: T,
    scale: f64,
) -> Option<Vec<Piece>> {
    let end = |p: geom_core::Point2<T>| {
        let [(x, ex), (y, ey)] = [p.x, p.y].map(|c| {
            let m = middle(c);
            let mut e = half_width(c);
            if tracks::<T>() && !mutant("spline-trim-iso-end-uncharged") {
                e += rounding_charge(m.abs() + scale);
            }
            (m, e)
        });
        Piece::vertex([x, y], [ex, ey])
    };
    match pcurve {
        Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. } => {
            let [a, b] = pcurve.span_ends((t0, t1));
            Some(vec![Piece::segment(end(a), end(b))])
        }
        Pcurve::Fitted(image) | Pcurve::General(image) => {
            let mut run = bezier_pieces(image, t0.value_channel(), t1.value_channel())?;
            if !tracks::<T>() {
                // The span's ends are brackets, cut at their value: each
                // end of the run is also off by the image's reach over its
                // bracket, its enclosure's middle and half-width.
                let reach = |(h, e): &mut Control<f64, 3>, at: geom_core::Point2<T>| {
                    for (c, x) in [at.x, at.y].into_iter().enumerate() {
                        let off = (middle(x) - h[c] / h[2]).abs() + half_width(x);
                        e[c] += h[2].abs() * off;
                    }
                };
                reach(run.first_mut()?.net.first_mut()?, image.eval(t0));
                reach(run.last_mut()?.net.last_mut()?, image.eval(t1));
            }
            Some(run)
        }
        _ => None,
    }
}

/// Half the width of a value's bracket: how far its middle can lie from
/// the value it stands for; NaN for the poison.
fn half_width<T: Decide>(x: T) -> f64 {
    if mutant("spline-trim-halfwidth-dropped") {
        return 0.0;
    }
    x.structure_bracket()
        .map_or(f64::NAN, |(lo, hi)| (hi - lo) * 0.5)
}

/// The Bézier pieces of `image` over `[a, b]`, ascending.
pub(crate) fn bezier_pieces<T: Decide>(
    image: &NurbsCurve2<T>,
    a: f64,
    b: f64,
) -> Option<Vec<Piece>> {
    let p = image.degree();
    let (lo, hi) = image.domain();
    if p == 0 || !(lo <= a && a < b && b <= hi) {
        return None;
    }
    let runs: Vec<(f64, usize)> = image
        .knot_values()
        .runs()
        .map(|(v, m)| (v.value_channel(), m))
        .collect();
    let have = |x: f64| runs.iter().find(|(v, _)| *v == x).map_or(0, |&(_, m)| m);
    let mut at: Vec<f64> = runs
        .iter()
        .map(|&(v, _)| v)
        .chain([a, b])
        .filter(|&v| v > lo && v < hi)
        .collect();
    at.sort_by(f64::total_cmp);
    at.dedup();
    let mut add = Vec::new();
    for x in at {
        let m = have(x);
        if m > p {
            return None;
        }
        add.extend(core::iter::repeat_n(x, p - m));
    }
    // Insertion keeps every break and adds `a` and `b`.
    let mut bk: Vec<f64> = runs.iter().map(|&(v, _)| v).chain([a, b]).collect();
    bk.sort_by(f64::total_cmp);
    bk.dedup();
    let track = tracks::<T>();
    let row = image
        .control()
        .iter()
        .zip(image.weights())
        .map(|(c, &w)| {
            let h = [c.x * w, c.y * w, w];
            let e = core::array::from_fn(|k| {
                if track && k < 2 {
                    UNIT_ROUNDOFF * middle(h[k]).abs()
                } else {
                    0.0
                }
            });
            (h, e)
        })
        .collect();
    let control = refined(image.knot_values(), row, &add)?;
    if control.len() != (bk.len() - 1) * p + 1 {
        return None;
    }
    // Out of the scalar: each channel's bracket middle, off by its
    // half-width as well.
    let out_of_scalar = |(h, e): Control<T, 3>| -> Control<f64, 3> {
        (
            h.map(middle),
            core::array::from_fn(|k| e[k] + half_width(h[k])),
        )
    };
    let mut out = Vec::new();
    for k in 0..bk.len() - 1 {
        if bk[k] < a || bk[k + 1] > b {
            continue;
        }
        out.push(Piece {
            net: (k * p..=k * p + p)
                .map(|i| out_of_scalar(control[i]))
                .collect(),
        });
    }
    (!out.is_empty()).then_some(out)
}

/// **The chart's inf-side lever arms over `window`**: the least, over the
/// Bézier cells the window overlaps, of each cell's arms restricted to
/// the window ([`crate::chart_region::spline_chart_arms`], gated as the
/// chart region gates its own). `Err` carries the in-band arm, if any.
fn window_arms<T: Decide>(
    cells: &[Patch<T>],
    window: Rect,
    band: Band,
) -> Result<(InfSpeed<T>, InfSpeed<T>), Option<Indeterminate>> {
    let mut least: Option<(T, T)> = None;
    for cell in cells {
        let Some(part) = cell.rect.overlap(window) else {
            continue;
        };
        let surface = cell.restricted(part).surface().ok_or(None)?;
        let floor = |x: T| x.structure_bracket().map_or(f64::NAN, |(lo, _)| lo);
        let (au, av) =
            crate::chart_region::spline_chart_arms(&surface, SPLINE_TRIM_ARM, floor, band)
                .map_err(|gate| match gate {
                    crate::chart_region::ArmGate::Unbounded => None,
                    crate::chart_region::ArmGate::Escalated(diag) => Some(diag),
                })?;
        let (au, av) = (au.get(), av.get());
        least = Some(least.map_or((au, av), |(lu, lv)| (lu.min(au), lv.min(av))));
    }
    least
        .map(|(u, v)| (InfSpeed::new(u), InfSpeed::new(v)))
        .ok_or(None)
}

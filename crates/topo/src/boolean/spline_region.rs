//! **Where a point lies against a spline face**: the NURBS and
//! approximating-surface arm of [`super::contain::curved_face_placement`],
//! reached once the boundary walk has placed the point off every edge.
//!
//! # The carrier: a foot, or a lower bound past the band
//!
//! A spline face has no implicit form and is given no signed residual
//! (C5, the curve × face clause). The point is decided ON the carrier by
//! a foot, a point of the chart decided within the band of it, and OFF
//! the carrier by a lower bound on its distance past the band. Both come
//! from one walk over the chart's Bézier cells, subdivided: a cell whose
//! control hull lies definitely past the band of the point holds no point
//! of the carrier near it and is dropped, and the centre of a cell that
//! is kept is its foot candidate. Nothing reads a derivative of the chart
//! at a point, so a crease (a C⁰ knot run) is a cell boundary like any
//! other, and a fold or a stationary point of the chart is no foot unless
//! its distance decides.
//!
//! # The foot's sheet
//!
//! A carrier can pass within the band of the point on a sheet the face
//! does not use (one folded back over the face, or a span that comes
//! round), so no one foot is the face's. Every kept cell is placed in the
//! face's trim, and the reading is:
//!
//! - **In** where some foot decided within the band lies in the trim: the
//!   point is within the band of a point of the face;
//! - **Out** where some foot decided within the band exists and every
//!   kept cell lies wholly outside the trim: no point of the face is
//!   within the band, and the point is on the carrier;
//! - **off the carrier** where no cell is kept;
//! - otherwise unsettled. An in-band reading escalates and never prunes.
//!
//! # The trim, in the chart
//!
//! The trim is read in the face's chart from C4's derived pcurves, lifted
//! along each loop past its placeholders
//! ([`crate::pcurves::loop_lift_past_placeholders`]: mid-boolean, the
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
//! [`geom_core::rounding_charge`] of the read (the moved point's reach and
//! the gap); a weight not shown positive, or a bound that is not finite,
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
//! floor adds a slab and charges its read at its points' reach.

use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::Arc;

use geom::{NurbsCurve2, NurbsSurface, Surface};
use geom_brep::Pcurve;
use geom_core::spline::Knots;
use geom_core::{
    Band, Decide, Indeterminate, InfSpeed, Margin, Point3, Sign, UNIT_ROUNDOFF, Vec3,
    rounding_charge,
};

use super::contain::{ContainDecision, ContainError, CurvedPlacement, FaceContainment};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey};
use crate::joint::Winding;
use crate::live::proven;
use crate::splitting::Escalation;
use crate::validate::decide;

/// A cell's control hull, or a foot candidate, against the point: how
/// far the point lies off the spline face's carrier.
pub(crate) const CARRIER: &str = "bool_spline_region_carrier";
/// A chart window's inf-side lever arm, gated as a length.
pub(crate) const ARM: &str = "bool_spline_region_arm";
/// A query's clearance from the face's trim along the chart.
pub(crate) const TRIM: &str = "bool_spline_region_trim";

/// How many times a kept cell is quartered at most, a backstop behind
/// the band's own floor ([`Patch::extent`]).
const CELL_DEPTH: usize = 48;
/// The kept cells one reading may visit ([`Caps`]).
const CELL_VISITS: usize = 1 << 13;
/// How many times a trim piece is halved before a query it is not clear
/// of reads near the trim.
const PIECE_DEPTH: usize = 52;
/// The piece visits one query's trim reading may make ([`Caps`]).
const PIECE_VISITS: usize = 1 << 14;
/// The copies of a query a closed chart's periods may ask for.
const COPIES: usize = 9;

/// **The spline arm of [`super::contain::curved_face_placement`]** (module
/// docs): the point `q`, already off every boundary edge, against `face`,
/// a face whose carrier is a spline chart.
///
/// # Errors
///
/// [`ContainError::Escalated`] where the reading stopped on an in-band
/// decision; an unsettled reading with no such decision is
/// `Trim(None)`, the curved door's remainder.
///
/// An `In` placement comes with the chart point of the in-trim foot it
/// was decided at.
pub(super) fn spline_face_placement<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    q: Point3<T>,
    band: Band,
) -> Result<(CurvedPlacement, Option<(f64, f64)>), ContainError> {
    let view = body.proven_face(face);
    let Some(chart) = view.surface().spline_chart() else {
        unreachable!("the spline arm is reached on a spline chart")
    };
    Ok(match read(body, face, chart, q, band) {
        Reading::OffCarrier => (CurvedPlacement::OffCarrier, None),
        Reading::In(uv) => (CurvedPlacement::Trim(Some(FaceContainment::In)), Some(uv)),
        Reading::Out => (CurvedPlacement::Trim(Some(FaceContainment::Out)), None),
        Reading::Unsettled(None) => (CurvedPlacement::Trim(None), None),
        Reading::Unsettled(Some(diag)) => {
            // The carrier rows are the carrier decision's; the trim's and
            // the arm's have no decision of the door's to name.
            let decision = (diag.predicate == Some(CARRIER)).then_some(ContainDecision::Carrier);
            return Err(ContainError::Escalated {
                decision,
                escalation: Escalation::Margin,
                diag,
            });
        }
    })
}

/// **A point of a spline face's chart, against the face's trim**: the
/// chart point `(u, v)` a meeting was found at, placed by the trim alone
/// ([`Trim::place`]), so it is read on the sheet it lies on. A point
/// read in space, as [`spline_face_placement`] reads one, is in where any
/// sheet within the band of it is, and a meeting's own sheet may be
/// outside the trim where another passing within the band is inside it.
///
/// `None` where the trim is not read, or the point lies within the band
/// of it along the chart with no decision to name.
///
/// # Errors
///
/// [`ContainError::Escalated`] where the trim's reading stopped on an
/// in-band decision.
pub(super) fn spline_chart_placement<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    (u, v): (f64, f64),
    band: Band,
) -> Result<Option<FaceContainment>, ContainError> {
    let view = body.proven_face(face);
    let linked = view.surface();
    let Some(chart) = linked.spline_chart() else {
        unreachable!("the spline arm is reached on a spline chart")
    };
    if linked.is_placeholder_chart() {
        return Ok(None);
    }
    let Some(cells) = bezier_cells(chart) else {
        return Ok(None);
    };
    let Some(trim) = Trim::of(body, face, chart, &cells, Caps::READ, band) else {
        return Ok(None);
    };
    match trim.place(&cells, Rect::point(u, v), band) {
        Place::In => Ok(Some(FaceContainment::In)),
        Place::Out => Ok(Some(FaceContainment::Out)),
        Place::Near(None) => Ok(None),
        Place::Near(Some(diag)) => Err(ContainError::Escalated {
            decision: None,
            escalation: Escalation::Margin,
            diag,
        }),
    }
}

/// What [`read`] found (module docs).
#[derive(Clone, Debug)]
pub(crate) enum Reading {
    /// No point of the carrier is within the band of the point.
    OffCarrier,
    /// A foot within the band lies in the trim: its chart point.
    In((f64, f64)),
    /// A foot lies within the band, and every point of the carrier near
    /// the point lies outside the trim.
    Out,
    /// Neither, with the in-band decision the walk stopped on, if any.
    Unsettled(Option<Indeterminate>),
}

/// The work one reading may do before it stands unsettled: bounds on
/// the work, not on the geometry.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Caps {
    /// The kept cells the carrier walk may visit.
    pub(crate) cells: usize,
    /// The pieces one query's trim reading may visit.
    pub(crate) pieces: usize,
}

impl Caps {
    /// The caps every caller reads at.
    pub(crate) const READ: Self = Self {
        cells: CELL_VISITS,
        pieces: PIECE_VISITS,
    };
}

/// **The reading of `q` against `face` on its spline `chart`** (module
/// docs).
pub(crate) fn read<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    chart: &NurbsSurface<T>,
    q: Point3<T>,
    band: Band,
) -> Reading {
    read_capped(body, face, chart, q, band, Caps::READ)
}

/// The first in-band decision a walk stops on is the one it reports.
fn keep(pending: &mut Option<Indeterminate>, diag: Option<Indeterminate>) {
    if pending.is_none() {
        *pending = diag;
    }
}

/// [`read`] under `caps`.
pub(crate) fn read_capped<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    chart: &NurbsSurface<T>,
    q: Point3<T>,
    band: Band,
    caps: Caps,
) -> Reading {
    let view = body.proven_face(face);
    if view.surface().is_placeholder_chart() {
        return Reading::Unsettled(None);
    }
    let Some(cells) = bezier_cells(chart) else {
        return Reading::Unsettled(None);
    };
    let Some(trim) = Trim::of(body, face, chart, &cells, caps, band) else {
        return Reading::Unsettled(None);
    };
    carrier_walk(chart, &cells, q, band, caps, |rect| {
        trim.place(&cells, rect, band)
    })
}

/// **The carrier walk of [`read_capped`]** over the chart's `cells`,
/// each kept cell and foot placed in the trim by `place`: the reading,
/// every way it ends. [`read_capped`] reads exactly this with its
/// [`Trim::place`]; a row may hand it another placement.
fn carrier_walk<T: Decide>(
    chart: &NurbsSurface<T>,
    cells: &[Patch<T>],
    q: Point3<T>,
    band: Band,
    caps: Caps,
    place: impl Fn(Rect) -> Place,
) -> Reading {
    // Breadth first, so the shallowest foot is found before any cell
    // beside it is walked to the floor.
    let mut queue: VecDeque<(Patch<T>, Option<bool>, usize)> =
        cells.iter().map(|c| (c.clone(), None, 0)).collect();
    let (mut leaf, mut foot, mut open) = (false, false, false);
    let mut visits = 0usize;
    let mut pending: Option<Indeterminate> = None;
    while let Some((cell, known, depth)) = queue.pop_front() {
        match decide(CARRIER, Margin::of(cell.floor(q)), band) {
            Ok(Sign::Positive) => continue,
            Ok(_) => {}
            Err(diag) => keep(&mut pending, Some(diag)),
        }
        visits += 1;
        if visits > caps.cells {
            return Reading::Unsettled(pending);
        }
        let extent = cell.extent();
        // A cell no wider than the band is not placed, a bound on the
        // work: its window's way out is no longer than the cell, so its
        // placement could read nothing but near.
        let placeable = extent > band.escalate();
        let inside = known.or_else(|| {
            if !placeable {
                return None;
            }
            match place(cell.rect) {
                Place::In => Some(true),
                Place::Out => Some(false),
                Place::Near(diag) => {
                    keep(&mut pending, diag);
                    None
                }
            }
        });
        if inside == Some(false) && foot {
            leaf = true;
            continue;
        }
        let (cu, cv) = cell.rect.centre();
        let at = chart.eval(T::from_computed(cu), T::from_computed(cv));
        match decide(CARRIER, Margin::of((at - q).norm()), band) {
            Ok(Sign::Zero) => {
                foot = true;
                let placed = inside.or_else(|| match place(Rect::point(cu, cv)) {
                    Place::In => Some(true),
                    Place::Out => None,
                    Place::Near(diag) => {
                        keep(&mut pending, diag);
                        None
                    }
                });
                if placed == Some(true) {
                    return Reading::In((cu, cv));
                }
                if inside == Some(false) {
                    leaf = true;
                    continue;
                }
            }
            Ok(_) => {}
            Err(diag) => keep(&mut pending, Some(diag)),
        }
        if depth == CELL_DEPTH || extent.is_nan() || extent <= band.zero() * 0.5 {
            leaf = true;
            open |= inside != Some(false);
            continue;
        }
        let ((a, b), (c, d)) = cell.quartered();
        queue.extend([a, b, c, d].map(|child| (child, inside, depth + 1)));
    }
    match (foot, leaf, open) {
        (false, false, _) => Reading::OffCarrier,
        (true, _, false) => Reading::Out,
        _ => Reading::Unsettled(pending),
    }
}

/// A rectangle of a chart's parameters, `u` then `v`.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Rect {
    u: (f64, f64),
    v: (f64, f64),
}

impl Rect {
    fn point(u: f64, v: f64) -> Self {
        Self {
            u: (u, u),
            v: (v, v),
        }
    }

    fn centre(self) -> (f64, f64) {
        (mid(self.u), mid(self.v))
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

fn mid((a, b): (f64, f64)) -> f64 {
    a + (b - a) * 0.5
}

/// The gap between two closed intervals, zero where they meet.
fn gap((a0, a1): (f64, f64), (b0, b1): (f64, f64)) -> f64 {
    (b0 - a1).max(a0 - b1).max(0.0)
}

/// **One Bézier cell of a chart**: its parameter rectangle and its
/// control net in homogeneous form about `centre`, row-major in `u`,
/// each point with its rounding bound (*Rounding*, module docs).
#[derive(Clone, Debug)]
struct Patch<T: geom_core::Real> {
    rect: Rect,
    pu: usize,
    pv: usize,
    net: Vec<Control<T, 4>>,
    /// The middle of the chart's control box, the origin the net is held
    /// about.
    centre: Vec3<T>,
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
    fn floor(&self, q: Point3<T>) -> T {
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
        for p in points {
            lo = Point3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Point3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
        let axis = |lo: T, hi: T, x: T| (lo - x).max(x - hi).max(T::zero());
        let (gx, gy, gz) = (
            axis(lo.x, hi.x, moved.x),
            axis(lo.y, hi.y, moved.y),
            axis(lo.z, hi.z, moved.z),
        );
        let gap = (gx.powi(2) + gy.powi(2) + gz.powi(2)).sqrt();
        if !tracks::<T>() || mutant("spline-region-charge-dropped") {
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
        let cut = if mutant("spline-region-cut-dropped") {
            0.0
        } else {
            cut
        };
        // The read's own rounding: the move `q − c` rounds each coordinate
        // by `u·|q_c − c_c|`, each axis difference by `u` of itself, and
        // the squares, sum and root by a few `u` of the gap; charged twice
        // over. The points' quotients are the cut's (`point_rounding`).
        // No fixture makes this term decide alone: near the box the move
        // and the differences are exact or round by `u` of coordinates
        // the cut's charge already bounds by `u·|P_c|` at least.
        let read = if mutant("spline-region-read-dropped") {
            T::zero()
        } else {
            rounding_charge((moved - Point3::origin()).norm() + gap)
        };
        (gap - T::from_computed(cut) - read).max(T::zero())
    }

    /// The diagonal of the control net's box, read on its value channel:
    /// a cell narrower than half the band's zero is not quartered again,
    /// since its centre already stands for every point of it.
    fn extent(&self) -> f64 {
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
    fn split(&self, along_u: bool, s: f64) -> (Self, Self) {
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
    fn quartered(&self) -> ((Self, Self), (Self, Self)) {
        let (a, b) = self.split(true, 0.5);
        (a.split(false, 0.5), b.split(false, 0.5))
    }

    /// The cell restricted to `rect`, a rectangle inside its own.
    fn restricted(&self, rect: Rect) -> Self {
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
type Control<T, const N: usize> = ([T; N], [f64; N]);

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
    let (dt, roundings) = if ratio && !mutant("spline-region-ratio-uncharged") {
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
fn point_rounding<T: Decide, const N: usize>((h, e): &Control<T, N>) -> [f64; N] {
    let w = middle(h[N - 1]);
    let lead = w - e[N - 1];
    if lead.partial_cmp(&0.0) != Some(core::cmp::Ordering::Greater) {
        return [f64::INFINITY; N];
    }
    let scale = if mutant("spline-region-bound-halved") {
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
    x.structure_bracket()
        .map_or(f64::NAN, |(lo, hi)| lo + (hi - lo) * 0.5)
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
fn bezier_cells<T: Decide>(chart: &NurbsSurface<T>) -> Option<Vec<Patch<T>>> {
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
        T::from_computed(mid((lo[0], hi[0]))),
        T::from_computed(mid((lo[1], hi[1]))),
        T::from_computed(mid((lo[2], hi[2]))),
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
/// (`SPLICE_SPLINE_REGION_MUTANT`, `option_env!` because
/// `scripts/gates/no-ambient-env.sh` forbids a runtime read): the
/// charges read wrong one at a time, each refused by a row in
/// `spline_region_rows`.
fn mutant(site: &str) -> bool {
    #[cfg(test)]
    {
        debug_assert!(MUTANTS.contains(&site), "an unlisted mutant site: {site}");
        option_env!("SPLICE_SPLINE_REGION_MUTANT") == Some(site)
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
    "spline-region-charge-dropped",
    "spline-region-cut-dropped",
    "spline-region-read-dropped",
    "spline-region-ratio-uncharged",
    "spline-region-bound-halved",
    "spline-region-trim-uncharged",
    "spline-region-u-unwidened",
    "spline-region-halfwidth-dropped",
    "spline-region-shift-untracked",
    "spline-region-iso-end-uncharged",
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

/// The break values of `knots`, one per run.
fn breaks<T: Decide>(knots: &Knots<T>) -> Option<Vec<f64>> {
    let out: Vec<f64> = knots.runs().map(|(v, _)| v.value_channel()).collect();
    out.iter().all(|x| x.is_finite()).then_some(out)
}

/// **One piece of the trim's chain**: a rational Bézier curve in the
/// chart, its control points in homogeneous form, each with its rounding
/// bound (*Rounding*, module docs).
#[derive(Clone, Debug)]
struct Piece {
    net: Vec<Control<f64, 3>>,
}

impl Piece {
    fn segment(a: Control<f64, 3>, b: Control<f64, 3>) -> Self {
        Self { net: vec![a, b] }
    }

    /// The chart point `p`, each coordinate off by at most `e`.
    fn vertex(p: [f64; 2], e: [f64; 2]) -> Control<f64, 3> {
        ([p[0], p[1], 1.0], [e[0], e[1], 0.0])
    }

    fn point((h, _): Control<f64, 3>) -> [f64; 2] {
        [h[0] / h[2], h[1] / h[2]]
    }

    fn first(&self) -> Control<f64, 3> {
        self.net[0]
    }

    fn last(&self) -> Control<f64, 3> {
        self.net[self.net.len() - 1]
    }

    fn ends(&self) -> ([f64; 2], [f64; 2]) {
        (Self::point(self.first()), Self::point(self.last()))
    }

    /// The control points' box, which holds the piece, widened on each
    /// side by the worst rounding of a point; `None` for a point with no
    /// value.
    fn bbox(&self) -> Option<Rect> {
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
        if mutant("spline-region-trim-uncharged") {
            return Some(r);
        }
        if mutant("spline-region-u-unwidened") {
            worst[0] = 0.0;
        }
        Some(Rect {
            u: (r.u.0 - worst[0], r.u.1 + worst[0]),
            v: (r.v.0 - worst[1], r.v.1 + worst[1]),
        })
    }

    /// The two halves of the piece's parameter, by de Casteljau.
    fn halves(&self) -> (Self, Self) {
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
    fn shifted(&self, du: f64, dv: f64) -> Self {
        let step = |x: f64, ex: f64, d: f64, w: f64, ew: f64| {
            let r = x + d * w;
            if mutant("spline-region-shift-untracked") {
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

/// Where [`Trim::place`] put a query.
#[derive(Clone, Debug)]
enum Place {
    In,
    Out,
    /// Not clear of the trim, with the in-band decision that kept it so.
    Near(Option<Indeterminate>),
}

/// **A face's trim in its spline chart** (module docs): every loop's
/// chain, each ring lifted into the outer loop's cover.
struct Trim {
    pieces: Vec<Piece>,
    /// The box of every piece.
    reach: Rect,
    /// The chart's knot domain.
    domain: Rect,
    /// The chart's period in each channel it is decided closed in.
    periods: [Option<f64>; 2],
    /// The piece visits one query's reading may make ([`Caps`]).
    piece_visits: usize,
}

impl Trim {
    /// The trim of `face` on `chart` (`cells` its Bézier decomposition);
    /// `None` where a loop has no lift past its placeholders or winds, an image is of a kind
    /// with no piece reading here, the lift applies a period the chart
    /// is not decided closed in, or a ring's place in the outer loop's
    /// cover is not decided.
    fn of<T: Decide>(
        body: &Body<T>,
        face: FaceKey,
        chart: &NurbsSurface<T>,
        cells: &[Patch<T>],
        caps: Caps,
        band: Band,
    ) -> Option<Self> {
        let view = body.proven_face(face);
        let surface = view.surface();
        // The lift moves a row by the periods the chart STATES
        // ([`crate::pcurves::loop_lift`]), and a copy of a query is read a
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
                let s = s?.length().value_channel();
                #[allow(clippy::float_cmp)] // one knot domain's length, read twice
                let agree = s == o.length().value_channel();
                if !agree {
                    return None;
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
        // The chart coordinates' magnitude an image's evaluation rounds
        // at: the domain's, and a period past it.
        let scale = [domain.u.0, domain.u.1, domain.v.0, domain.v.1]
            .into_iter()
            .chain(periods.iter().flatten().copied())
            .map(f64::abs)
            .fold(0.0, f64::max);
        let mut chains = Vec::new();
        for (lk, _, _) in view.loops() {
            let lift = crate::pcurves::loop_lift_past_placeholders(body, lk).ok()?;
            if lift.is_empty() {
                return None;
            }
            // A skipped placeholder's joints are the identity
            // (`loop_lift_past_placeholders`), so the members' elements
            // wind as the whole loop's.
            let elements: Option<Vec<_>> = lift.iter().map(|r| body.joint(r.half_edge)).collect();
            if !Winding::of(elements?).is_zero() {
                return None;
            }
            let mut chain: Vec<Piece> = Vec::new();
            for row in &lift {
                if (row.lift.u != 0 && periods[0].is_none())
                    || (row.lift.v != 0 && periods[1].is_none())
                    || row.lift.twin
                {
                    return None;
                }
                let (t0, t1) = row.row.params();
                let mut run = image_pieces(&row.pcurve, t0, t1, scale)?;
                if !walks_forward(body, row.half_edge) {
                    run = run.into_iter().rev().map(Piece::reversed).collect();
                }
                close_gap(&mut chain, run.first()?.first());
                chain.extend(run);
            }
            let start = chain.first()?.first();
            close_gap(&mut chain, start);
            chains.push(chain);
        }
        let mut chains = chains.into_iter();
        let outer = Self::from_pieces(chains.next()?, domain, periods, caps)?;
        let rings: Vec<Vec<Piece>> = chains.collect();
        outer.with_rings(cells, rings, band)
    }

    /// The trim the chain `pieces` bounds, with no ring.
    fn from_pieces(
        pieces: Vec<Piece>,
        domain: Rect,
        periods: [Option<f64>; 2],
        caps: Caps,
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
            piece_visits: caps.pieces,
        })
    }

    /// **This outer trim with `rings` in its cover.** Each loop's lift
    /// starts on one of its own rows, so on a closed channel a ring can
    /// sit whole periods from the region its outer loop's lift bounds.
    /// A ring is moved by the one whole number of periods per closed
    /// channel that places its first point in that region, decided
    /// against the outer loop alone; `None` where no such shift, or more
    /// than one, is decided.
    fn with_rings<T: Decide>(
        self,
        cells: &[Patch<T>],
        rings: Vec<Vec<Piece>>,
        band: Band,
    ) -> Option<Self> {
        let mut pieces = self.pieces.clone();
        for ring in rings {
            let [u, v] = ring.first()?.ends().0;
            let shifts = |k: usize, x: f64, (r0, r1): (f64, f64)| -> Vec<f64> {
                match self.periods[k].filter(|p| *p > 0.0) {
                    None => vec![0.0],
                    Some(p) => {
                        let (lo, hi) = (((r0 - x) / p).floor(), ((r1 - x) / p).ceil());
                        let n = (hi - lo).max(0.0);
                        if n > COPIES as f64 {
                            return Vec::new();
                        }
                        (0..=n as usize).map(|i| (lo + i as f64) * p).collect()
                    }
                }
            };
            let mut found = None;
            for du in shifts(0, u, self.reach.u) {
                for dv in shifts(1, v, self.reach.v) {
                    match self.place_in_cover(cells, Rect::point(u + du, v + dv), band) {
                        Place::In if found.is_none() => found = Some((du, dv)),
                        Place::Out => {}
                        Place::In | Place::Near(_) => return None,
                    }
                }
            }
            let (du, dv) = found?;
            pieces.extend(ring.iter().map(|p| p.shifted(du, dv)));
        }
        Self::from_pieces(
            pieces,
            self.domain,
            self.periods,
            Caps {
                cells: 0,
                pieces: self.piece_visits,
            },
        )
    }

    /// **Where the query `q` lies against the trim** (module docs): `In`
    /// or `Out` only where every piece is decided clear of it, along the
    /// chart, past the band, in every copy of `q` a period apart on each
    /// closed channel. `cells` is the chart's Bézier decomposition, which
    /// the window's arms are read off.
    fn place<T: Decide>(&self, cells: &[Patch<T>], q: Rect, band: Band) -> Place {
        self.read_window(cells, q, band, true)
    }

    /// [`Self::place`] of `q` alone, in the cover, with no copy.
    fn place_in_cover<T: Decide>(&self, cells: &[Patch<T>], q: Rect, band: Band) -> Place {
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
    ) -> Place {
        // The window is the chart's, read at the query's copy in the
        // domain; the parity is the cover's, read at the query itself.
        let q = self.principal(at);
        let (du, dv) = self.domain.size();
        let (su, sv) = q.size();
        let reach = (
            if su > 0.0 { su } else { du / 16.0 },
            if sv > 0.0 { sv } else { dv / 16.0 },
        );
        let mut pending = None;
        for halving in 0..6 {
            let scale = f64::from(1u32 << halving);
            let (ru, rv) = (reach.0 / scale, reach.1 / scale);
            let window = Rect {
                u: (q.u.0 - ru, q.u.1 + ru),
                v: (q.v.0 - rv, q.v.1 + rv),
            };
            let Some(window) = window.overlap(self.domain) else {
                return Place::Near(None);
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
                return Place::Near(diag.or(pending));
            }
            return if copies {
                self.copies(at, arms, band)
            } else {
                self.copy(at, arms, band)
            };
        }
        Place::Near(pending)
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
                TRIM,
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
    fn copies<T: Decide>(&self, q: Rect, arms: (InfSpeed<T>, InfSpeed<T>), band: Band) -> Place {
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
            return Place::Near(None);
        }
        let mut out = true;
        let mut near = None;
        for &du in &us {
            for &dv in &vs {
                match self.copy(q.shifted(du, dv), arms, band) {
                    Place::In => return Place::In,
                    Place::Out => {}
                    Place::Near(diag) => {
                        out = false;
                        near = near.or(diag);
                    }
                }
            }
        }
        if out { Place::Out } else { Place::Near(near) }
    }

    /// One copy of the query in the cover: the ray parity of its centre
    /// along `+u`, every piece decided clear of it first.
    ///
    /// `ray_walk`, the chart region's parity, reads a polygon's straight
    /// segments vertex by vertex and sets a grazing ray aside; these
    /// pieces are curved arcs read off their control boxes, so the
    /// crossing count is read off each clear piece's two ends under the
    /// one half-open rule `v ≥ v_c`, whose shared ends are bit-identical
    /// across the chain.
    fn copy<T: Decide>(&self, q: Rect, (au, av): (InfSpeed<T>, InfSpeed<T>), band: Band) -> Place {
        let (cu, cv) = q.centre();
        let (su, sv) = q.size();
        let mut stack: Vec<(Piece, usize)> =
            self.pieces.iter().rev().map(|p| (p.clone(), 0)).collect();
        let (mut parity, mut visits) = (false, 0usize);
        while let Some((piece, depth)) = stack.pop() {
            visits += 1;
            let Some(b) = piece.bbox() else {
                return Place::Near(None);
            };
            if visits > self.piece_visits {
                return Place::Near(None);
            }
            let (gu, gv) = (gap(q.u, b.u), gap(q.v, b.v));
            // Clear where either channel's gap alone is decided past the
            // band: each lower-bounds the way along the chart to the
            // piece inside the window, and the way out of it was decided
            // past the band already.
            let mut diag = None;
            let mut clear = false;
            for (g, arm) in [(gu, au), (gv, av)] {
                match decide(TRIM, Margin::metered(T::from_computed(g), arm), band) {
                    Ok(Sign::Positive) => clear = true,
                    Ok(_) => {}
                    Err(d) => diag = diag.or(Some(d)),
                }
            }
            if clear {
                if b.v.0 > cv || b.v.1 < cv || b.u.1 < cu {
                    continue;
                }
                if b.u.0 > cu {
                    let (s, e) = piece.ends();
                    parity ^= (s[1] >= cv) != (e[1] >= cv);
                    continue;
                }
            }
            let (bu, bv) = b.size();
            let meets = gu == 0.0 && gv == 0.0;
            if depth == PIECE_DEPTH || (meets && (su > 0.0 || sv > 0.0) && bu <= su && bv <= sv) {
                return Place::Near(diag);
            }
            let (l, r) = piece.halves();
            stack.push((r, depth + 1));
            stack.push((l, depth + 1));
        }
        if parity { Place::In } else { Place::Out }
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
/// is that segment, whatever its parameterization), a fitted or general
/// image its Bézier pieces over the span. `None` for any other kind, or
/// where the decomposition does not run. An end is held at its
/// bracket's middle, off by the bracket's half-width and, at `f64`, by a
/// short chain's rounding at `scale` past it (`p0 + d·g`).
fn image_pieces<T: Decide>(pcurve: &Pcurve<T>, t0: T, t1: T, scale: f64) -> Option<Vec<Piece>> {
    let end = |p: geom_core::Point2<T>| {
        let [(x, ex), (y, ey)] = [p.x, p.y].map(|c| {
            let m = middle(c);
            let mut e = half_width(c);
            if tracks::<T>() && !mutant("spline-region-iso-end-uncharged") {
                e += rounding_charge(m.abs() + scale);
            }
            (m, e)
        });
        Piece::vertex([x, y], [ex, ey])
    };
    match pcurve {
        Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. } => Some(vec![Piece::segment(
            end(pcurve.eval(t0)),
            end(pcurve.eval(t1)),
        )]),
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
    if mutant("spline-region-halfwidth-dropped") {
        return 0.0;
    }
    x.structure_bracket()
        .map_or(f64::NAN, |(lo, hi)| (hi - lo) * 0.5)
}

/// The Bézier pieces of `image` over `[a, b]`, ascending.
fn bezier_pieces<T: Decide>(image: &NurbsCurve2<T>, a: f64, b: f64) -> Option<Vec<Piece>> {
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
            crate::chart_region::spline_chart_arms(&surface, ARM, floor, band).map_err(|gate| {
                match gate {
                    crate::chart_region::ArmGate::Unbounded => None,
                    crate::chart_region::ArmGate::Escalated(diag) => Some(diag),
                }
            })?;
        let (au, av) = (au.get(), av.get());
        least = Some(least.map_or((au, av), |(lu, lv)| (lu.min(au), lv.min(av))));
    }
    least
        .map(|(u, v)| (InfSpeed::new(u), InfSpeed::new(v)))
        .ok_or(None)
}

#[cfg(test)]
#[path = "spline_region_rows.rs"]
pub(crate) mod rows;

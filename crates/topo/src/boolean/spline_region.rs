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
//! # The trim
//!
//! The face's trim in its chart, the query's placement in it and the
//! chart's Bézier cells that meter both are the crate root's
//! ([`crate::spline_trim`], whose docs say how a query is read and how
//! its rounding is charged); this arm walks the cells and places each
//! kept cell and foot through it.

use std::collections::VecDeque;

use geom::NurbsSurface;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Sign};

use super::contain::{ContainDecision, ContainError, CurvedPlacement, FaceContainment};
use crate::body::Body;
use crate::entity::FaceKey;
use crate::pcurves::PIECE_VISITS;
use crate::pcurves::Side;
use crate::spline_trim::{Patch, Rect, Trim, bezier_cells};
use crate::splitting::Escalation;
use crate::validate::decide;
// What this arm's rows read of the trim beside what the arm itself does.
#[cfg(test)]
use crate::spline_trim::{Piece, SPLINE_TRIM_CLEAR, bezier_pieces, image_pieces, point_rounding};

/// A cell's control hull, or a foot candidate, against the point: how
/// far the point lies off the spline face's carrier.
pub(crate) const SPLINE_REGION_CARRIER: &str = "bool_spline_region_carrier";

/// How many times a kept cell is quartered at most, a backstop behind
/// the band's own floor ([`Patch::extent`]).
const CELL_DEPTH: usize = 48;

/// The kept cells one reading may visit ([`Caps`]).
const CELL_VISITS: usize = 1 << 13;

/// The work one reading may do before it stands unsettled: bounds on
/// the work, not on the geometry.
#[derive(Clone, Copy, Debug)]
struct Caps {
    /// The kept cells the carrier walk may visit.
    cells: usize,
    /// The pieces one query's trim reading may visit.
    pieces: usize,
}

impl Caps {
    /// The caps every caller reads at.
    const READ: Self = Self {
        cells: CELL_VISITS,
        pieces: PIECE_VISITS,
    };
}

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
            let decision =
                (diag.predicate == Some(SPLINE_REGION_CARRIER)).then_some(ContainDecision::Carrier);
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
    let Some(trim) = Trim::of(body, face, chart, &cells, PIECE_VISITS, band) else {
        return Ok(None);
    };
    match trim.place(&cells, Rect::point(u, v), band) {
        Side::In => Ok(Some(FaceContainment::In)),
        Side::Out => Ok(Some(FaceContainment::Out)),
        Side::Near(None) => Ok(None),
        Side::Near(Some(diag)) => Err(ContainError::Escalated {
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
fn read_capped<T: Decide>(
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
    let Some(trim) = Trim::of(body, face, chart, &cells, caps.pieces, band) else {
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
    place: impl Fn(Rect) -> Side,
) -> Reading {
    // Breadth first, so the shallowest foot is found before any cell
    // beside it is walked to the floor.
    let mut queue: VecDeque<(Patch<T>, Option<bool>, usize)> =
        cells.iter().map(|c| (c.clone(), None, 0)).collect();
    let (mut leaf, mut foot, mut open) = (false, false, false);
    let mut visits = 0usize;
    let mut pending: Option<Indeterminate> = None;
    while let Some((cell, known, depth)) = queue.pop_front() {
        match decide(SPLINE_REGION_CARRIER, Margin::of(cell.floor(q)), band) {
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
                Side::In => Some(true),
                Side::Out => Some(false),
                Side::Near(diag) => {
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
        match decide(SPLINE_REGION_CARRIER, Margin::of((at - q).norm()), band) {
            Ok(Sign::Zero) => {
                foot = true;
                let placed = inside.or_else(|| match place(Rect::point(cu, cv)) {
                    Side::In => Some(true),
                    Side::Out => None,
                    Side::Near(diag) => {
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

#[cfg(test)]
#[path = "spline_region_rows.rs"]
pub(crate) mod rows;

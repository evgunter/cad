//! **The lift and its readers**: a face's stored rows and a loop's
//! [`Lift`], decided by the walk or read off what the mint stored.

use geom::Surface;
use geom_brep::{ChartAxis, ChartImage, ChartLine, HeldLine, Pcurve, PcurveCertifyError};
use geom_core::predicate::Band;
use geom_core::{Decide, Point2, Real};

use super::{
    ChartPeriods, DerivedRow, DescribedChart, PinMiss, WalkFail, WalkItem, edge_record, entry_exit,
    entry_vertex, half_edge_curve, is_plus, stored_edge, stored_reach, walk_cycle, walk_item,
};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopKey};
use crate::joint::{Deck, JointElement, Winding};
use crate::live::proven;
use crate::null::Scaffolding;

/// A half-edge charted for a derived lift: its carrier and its window
/// `(t0, t1)`.
type Charted<T> = (HalfEdgeKey, geom::Curve3<T>, T, T);

/// One face's boundary as this module's passes walk it: each loop's
/// half-edge cycle, **whether that boundary already STORES a row**, and
/// where the row set has a gap.
///
/// Read by [`validate_pcurves`], which re-certifies the stored rows and
/// reports the gaps, and by the site mint ([`site_rows_from`]), which
/// asks [`StoredRows::remints`] whether its door re-mints the face. So
/// "which rows does this face store" and "is the set complete" each
/// have one answer.
///
/// [`site_rows_from`]: super::site_rows_from
/// [`validate_pcurves`]: super::validate_pcurves
pub(crate) struct StoredRows {
    /// The face's loops in walk order, outer first, each with its
    /// half-edge cycle. A loop whose boundary is not a `Cycle` is
    /// ABSENT from this list: there is nothing to walk and nothing
    /// wrong.
    pub(crate) loops: Vec<(LoopKey, Vec<HalfEdgeKey>)>,
    /// Whether those cycles store any row: `false` when the face's
    /// boundary stores none at all, so the face has not been minted.
    pub(crate) stores: bool,
    /// The gaps in the row set, in walk order: each half-edge of a
    /// loop with no row — no image, or no element on the joint into it
    /// from a predecessor that stores one.
    pub(crate) gaps: Vec<RowGap>,
}

/// One gap in a face's row set ([`StoredRows::gaps`]): a half-edge
/// that stores no image, or no element on the joint into it from a
/// predecessor that stores one.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RowGap {
    /// The half-edge.
    pub(crate) half_edge: HalfEdgeKey,
    /// The loop it is on.
    pub(crate) r#loop: LoopKey,
}

impl StoredRows {
    /// **Whether the face's row set is COMPLETE**: it stores a row, and
    /// every half-edge of every loop carries a row. A face storing no
    /// row is unminted, not complete; an `Empty` loop holds no
    /// half-edge and so misses no row.
    pub(crate) fn complete(&self) -> bool {
        self.stores && self.gaps.is_empty()
    }

    /// **Whether a site mint re-mints the face** ([`site_rows`]): it
    /// stores a row, and every row it misses is on a
    /// loop in `open` — the loops a null edge holds open
    /// ([`StoredRows::open_loops`]) — unless the door `released` it,
    /// taking the last null edge off it, whatever else it misses.
    ///
    /// **"Minted" is read as "stores a row"** (`stores`). No door writes
    /// a row onto a face no mint has run over: the site mint leaves a
    /// face storing no row as found, and `mev_null` derives none. So a
    /// face storing any row was minted, and a half missing one arrived
    /// after that mint; a face storing none — never minted, or emptied
    /// by a door — is the producer's closing mint's, and stays rowless
    /// until it runs.
    ///
    /// **A loop a null edge holds open cannot be walked**: the edge's
    /// halves have no carrier. So no door before the edge leaves the
    /// loop can mint its rows — the edge's own two, or those of the
    /// halves an operator adds to the loop meanwhile — and those gaps
    /// do not keep the face from its site mint, which mints every loop
    /// its door leaves running through no null edge ([`site_rows`]).
    ///
    /// **The door that takes a face's last null edge off it re-mints
    /// it whatever it misses**, as the minting pass would once the
    /// scaffolding is gone: a null edge's first description, which
    /// walks every loop of its faces, an operator that rewires a loop
    /// out from under the edge, as the pipelines' joins do, and the band
    /// kill of the edge ([`crate::Body::plan_released_rows`]). An
    /// operator or a kill walks only the loops it rewires, so a gap on a
    /// loop it keeps stays. **Any other gap is not the site mint's to fill**: a door
    /// left the face half-minted ([`PcurveMintError::MissingCache`]
    /// lists them) and the site mint leaves it as found, for the
    /// producer's final pass.
    ///
    /// [`PcurveMintError::MissingCache`]: super::PcurveMintError::MissingCache
    /// [`site_rows`]: super::site_rows
    pub(crate) fn remints(&self, open: &[LoopKey], released: bool) -> bool {
        self.stores && (released || self.gaps.iter().all(|gap| open.contains(&gap.r#loop)))
    }
}

impl StoredRows {
    /// The loops of the face a null edge holds open ([`held_open`]), in
    /// walk order. Empty, with nothing read, on a face that misses no
    /// row: a null half never stores one, since every row derives from
    /// its half's carrier and a null edge has none.
    #[track_caller]
    pub(crate) fn open_loops<T: Decide>(&self, body: &Body<T>) -> Vec<LoopKey> {
        if self.gaps.is_empty() {
            return Vec::new();
        }
        self.loops
            .iter()
            .filter(|(_, cycle)| held_open(body, cycle.iter().copied()))
            .map(|&(lk, _)| lk)
            .collect()
    }
}

/// **Whether a null edge holds a loop open**: one of `halves`, the
/// loop's half-edges, is a half of a null edge
/// ([`crate::null::CurveGeom::NullScaffold`]), scaffolding with no carrier, so the
/// loop cannot be walked. The site mint's one reading of it
/// ([`StoredRows::open_loops`], [`site_rows`]).
///
/// # Panics
///
/// Where a half's edge or that edge's curve does not resolve: each is a
/// link of the record before it, and `halves` are half-edges the caller
/// read out of the body's records.
///
/// [`site_rows`]: super::site_rows
#[track_caller]
pub(super) fn held_open<T: Decide>(
    body: &Body<T>,
    halves: impl IntoIterator<Item = HalfEdgeKey>,
) -> bool {
    let mut open = false;
    for he in halves {
        open |= edge_record(body, he).curve().null_scaffold().is_some();
    }
    open
}

/// **The one per-loop rows walk**: the half-edges of `r#loop` a pcurve
/// row can be keyed on, in cycle order from its `first` — empty for a
/// loop whose boundary is a lone vertex ([`crate::LoopBoundary::Empty`]),
/// which holds no half-edge and so no row. These are the keys the map
/// is keyed on for this loop, and the only ones.
///
/// A row is keyed on a half-edge and belongs to the face that
/// half-edge's loop is on, so every question this module asks about
/// one loop's rows — which rows a face STORES ([`stored_rows`], and
/// through it [`validate_pcurves`] and [`split_cache`]), and which
/// rows a door that re-parents the loop must dispose of
/// ([`crate::Body::drop_rows_on_chart_change`]) — is a walk of this
/// one cycle. Two spellings of it would be two answers to "which rows
/// does this loop have", and a door and the validator disagreeing
/// about that is exactly the defect neither could see.
///
/// The walk is [`crate::Body::site_cycle_from`]: a member that does not
/// claim `r#loop` panics, so a torn `next` cannot hand another loop's
/// rows to this one's reader. A walk that closes short of a member it
/// should reach is not caught here; a door that moves or re-charts the
/// whole loop proves that too ([`crate::Body::whole_cycle`]).
///
/// # Panics
///
/// Where `r#loop` — a loop a door resolved, or one a face record names —
/// does not resolve, or its walk does not close on the half-edges that
/// claim it ([`crate::body::CYCLES_ARE_CLAIMANTS`]).
///
/// [`split_cache`]: super::split_cache
/// [`validate_pcurves`]: super::validate_pcurves
#[track_caller]
pub(crate) fn loop_rows<T: Decide>(body: &Body<T>, r#loop: LoopKey) -> Vec<HalfEdgeKey> {
    match proven(&body.loops, r#loop, EntityId::Loop).boundary {
        crate::entity::LoopBoundary::Cycle { first } => body.site_cycle_from(first, r#loop),
        crate::entity::LoopBoundary::Empty { .. } => Vec::new(),
    }
}

/// One half-edge of a [`Lift`].
#[derive(Clone, Debug)]
pub struct LiftedRow<T: Real> {
    /// The half-edge.
    pub half_edge: HalfEdgeKey,
    /// The deck transformation the lift moves the half-edge's image by:
    /// the joint elements summed along the chain ([`lift_decks`]).
    pub lift: Deck,
    /// The image moved by `lift`: the chart curve the chain draws on
    /// this half-edge.
    pub pcurve: Pcurve<T>,
    /// The carrier interval `(t0, t1)` the image is over, in the edge's
    /// forward order: the edge's span. Private, with `stored`: a row is
    /// made by a read of a body's stored row or by a suite's statement,
    /// so no caller pairs an image with another image's span.
    params: (T, T),
    /// The line the half-edge's STORED image holds, before `lift` moved
    /// it; `None` for an image that is no iso image. What a corner on
    /// the stored branch is read from ([`Lift::stored_corners`]): the
    /// stored value is the one a mint placed, where the moved one is a
    /// sum for every move but one step off a domain's end.
    stored: Option<ChartLine<T>>,
    /// Whether the chain traverses the image forward.
    pub plus: bool,
}

impl<T: Real> LiftedRow<T> {
    /// A row a suite states: `pcurve` over `(t0, t1)`, traversed forward
    /// where `plus`. What it lets a suite state that no body can: a row on
    /// no half-edge of any body (`HalfEdgeKey::default()`), unmoved by any
    /// joint element, whose image no certificate has read against a
    /// carrier, so its ends sit wherever the suite puts them.
    #[cfg(test)]
    pub(crate) fn stated_for_tests(pcurve: Pcurve<T>, (t0, t1): (T, T), plus: bool) -> Self {
        Self {
            half_edge: HalfEdgeKey::default(),
            lift: Deck::IDENTITY,
            stored: pcurve.held_line().map(|held| held.line),
            pcurve,
            params: (t0, t1),
            plus,
        }
    }

    /// This row as the lift `deck` moved it, its image unmoved: a row a
    /// suite states on a closed chart's lift (the image itself is the
    /// suite's to state where the move would put it; the stored line
    /// stays the one the row was stated with).
    #[cfg(test)]
    #[must_use]
    pub(crate) fn moved_for_tests(self, deck: Deck) -> Self {
        Self { lift: deck, ..self }
    }

    /// This row's image over the span `(t0, t1)`: a split's piece, which
    /// holds its parent's image over its own part of the parent's span.
    /// What it lets a suite state that no body can: a span no edge was
    /// certified over.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn piece_for_tests(self, (t0, t1): (T, T)) -> Self {
        Self {
            params: (t0, t1),
            ..self
        }
    }

    /// This row traversed the other way by its chain.
    #[cfg(test)]
    #[must_use]
    pub(crate) fn reversed_for_tests(self) -> Self {
        Self {
            plus: !self.plus,
            ..self
        }
    }

    /// The carrier interval `(t0, t1)` the image is over, in the edge's
    /// forward order: the edge's span.
    #[must_use]
    pub fn params(&self) -> (T, T) {
        self.params
    }

    /// The carrier parameters the chain enters and leaves the image at.
    #[must_use]
    pub fn entry_exit(&self) -> (T, T) {
        let (t0, t1) = self.params;
        entry_exit(self.plus, t0, t1)
    }

    /// **The chart line the image lies on, as the chain runs it**: the
    /// image's own held line ([`Pcurve::held_line`]), its rate negated
    /// where the chain traverses the image backward, so the rate's sign is
    /// the way the chain runs along the line. `None` where the image is
    /// no iso image.
    pub fn held_line(&self) -> Option<HeldLine<T>> {
        let line = self.pcurve.held_line()?;
        Some(HeldLine {
            rate: if self.plus { line.rate } else { -line.rate },
            ..line
        })
    }
}

impl<T: Decide> LiftedRow<T> {
    /// The chart points the chain enters and leaves the image at, read
    /// through its map ([`Pcurve::eval`]).
    #[must_use]
    pub fn entry_exit_points(&self) -> (Point2<T>, Point2<T>) {
        let (entry, exit) = self.entry_exit();
        (self.pcurve.eval(entry), self.pcurve.eval(exit))
    }
}

/// **A maximal run of the chain's images on one held axis**
/// ([`Lift::runs`]): consecutive rows whose lines all hold `axis`, each
/// with the line it was read on ([`LiftedRow::held_line`]: the rate in
/// chain order), in chain order. Two consecutive images holding one axis
/// meet only on one line, so in a loop whose joints close a run is one
/// side; whether its pieces state one value is the reader's to check.
#[derive(Clone, Debug)]
pub struct HeldRun<T> {
    /// The channel the run's lines hold fixed.
    pub axis: ChartAxis,
    /// The run's rows, each its index in [`Lift::rows`] and its line, in
    /// chain order. Never empty.
    pub members: Vec<(usize, HeldLine<T>)>,
}

/// Why the chain has no runs ([`Lift::runs`]): the first row, in chain
/// order, whose image is no iso image and so lies on no held line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NotHeld {
    /// The row's index in [`Lift::rows`].
    pub row: usize,
}

/// Why a loop has no lift to read ([`Lift::read`]): the first
/// half-edge, in cycle order from the loop's `first`, that stores no
/// image or no element on the joint into it, or whose image its lift
/// does not apply to ([`Deck::apply`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LiftGap {
    /// The half-edge.
    pub half_edge: HalfEdgeKey,
}

/// **A loop's position in its face's chart**: the chain of its
/// half-edges' chart images, each moved by the joint elements summed
/// along it ([`lift_decks`]). Consecutive curves of the chain meet at
/// every joint off the chart's singular set; at a joint on it (a pole,
/// an apex) the chain runs along the singular latitude through the
/// face's turn there (`crate::joint`), so the chain is the face's
/// boundary in the chart.
///
/// Where the chain starts is gauge: the chain from another start is the
/// same chain moved by one deck transformation, and every reader reads
/// the chain, not its offset.
///
/// **Two constructors, and the caller names which.** [`Lift::read`]
/// takes the rows a finished body stores: no band, and a hole is loud
/// ([`LiftGap`]). [`Lift::decided`] runs the mint's own walk
/// ([`walk_cycle`], [`decide_joint`]) at the caller's band, over a loop
/// or an open run of one, and stores nothing: for a reader
/// mid-operation, whose rows are not all written. The mint is "decide,
/// then store", so the two agree wherever both answer; neither falls
/// back to the other.
///
/// [`decide_joint`]: super::decide_joint
#[derive(Clone, Debug)]
pub struct Lift<T: Real> {
    rows: Vec<LiftedRow<T>>,
    /// The chain's joints composed once around, for a whole loop; `None`
    /// for an open run, which has no closure joint.
    winding: Option<Winding>,
}

impl<T: Real> Lift<T> {
    /// A lift a suite states row by row: a whole loop winding as
    /// `winding` says, an open run where it is `None`. What it lets a
    /// suite state that no body can: a winding no joint of the chain
    /// decided (the rows' own lifts stay the identity, whatever the
    /// winding), and a chain whose consecutive images need not meet.
    #[cfg(test)]
    pub(crate) fn stated_for_tests(rows: Vec<LiftedRow<T>>, winding: Option<Winding>) -> Self {
        Self { rows, winding }
    }

    /// The chain's rows, in order.
    #[must_use]
    pub fn rows(&self) -> &[LiftedRow<T>] {
        &self.rows
    }

    /// The chain's rows, in order.
    pub fn iter(&self) -> core::slice::Iter<'_, LiftedRow<T>> {
        self.rows.iter()
    }

    /// Whether the chain has no row.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// **How far the loop winds**: its joints composed once around.
    /// `None` for an open run.
    #[must_use]
    pub fn winding(&self) -> Option<Winding> {
        self.winding
    }

    /// **Whether the loop closes in the chart**: its chain ends where it
    /// starts, an integer reading ([`Winding::is_zero`]). `false` for an
    /// open run, and for a loop that winds a period of either channel.
    #[must_use]
    pub fn closes_in_chart(&self) -> bool {
        self.winding.is_some_and(Winding::is_zero)
    }

    /// **The chain's window in the chart**: the hull of each row's
    /// closed-form span box ([`Pcurve::closed_form_span_box`]), as
    /// `((u_min, u_max), (v_min, v_max))`. `None` for an empty chain, or
    /// one with a row that has no closed-form box (a projected, spiric
    /// or fitted image).
    #[must_use]
    pub fn window(&self) -> Option<((T, T), (T, T))> {
        self.rows
            .iter()
            .map(|row| {
                let (t0, t1) = row.params;
                row.pcurve
                    .closed_form_span_box(t0, t1)
                    .map(|b| ((b.u_min, b.u_max), (b.v_min, b.v_max)))
            })
            .reduce(|a, b| {
                let ((a_u, a_v), (b_u, b_v)) = (a?, b?);
                Some((
                    (a_u.0.min(b_u.0), a_u.1.max(b_u.1)),
                    (a_v.0.min(b_v.0), a_v.1.max(b_v.1)),
                ))
            })
            .flatten()
    }

    /// The first channel of [`Lift::window`]: the chain's azimuth hull.
    #[must_use]
    pub fn azimuth_window(&self) -> Option<(T, T)> {
        self.window().map(|(u, _)| u)
    }
}

impl<T: Decide> Lift<T> {
    /// **The chain grouped into its held runs** ([`HeldRun`]): maximal
    /// runs of consecutive rows whose images hold one chart axis, in
    /// chain order. A whole loop's chain is a cycle, so its last run and
    /// its first are one where they hold the same axis; an open run's
    /// are not joined. Consecutive runs hold different axes, by
    /// construction.
    ///
    /// # Errors
    ///
    /// [`NotHeld`] for the first row whose image lies on no held line.
    pub fn runs(&self) -> Result<Vec<HeldRun<T>>, NotHeld> {
        let mut runs: Vec<HeldRun<T>> = Vec::new();
        for (row, lifted) in self.rows.iter().enumerate() {
            let line = lifted.held_line().ok_or(NotHeld { row })?;
            match runs.last_mut() {
                Some(run) if run.axis == line.line.axis => run.members.push((row, line)),
                _ => runs.push(HeldRun {
                    axis: line.line.axis,
                    members: vec![(row, line)],
                }),
            }
        }
        if self.winding.is_some() && runs.len() > 1 && runs[0].axis == runs[runs.len() - 1].axis {
            let last = runs.pop().map(|run| run.members).unwrap_or_default();
            runs[0].members.splice(0..0, last);
        }
        Ok(runs)
    }

    /// **The chart point of the joint into row `i`**, from the row before
    /// it (the last row, for a whole loop's first). The convention, stated
    /// once for every reader of a joint's chart point:
    ///
    /// - **where a row of the chart meets a column**, the MEET of their two
    ///   held values, `(column's value, row's value)`, exact wherever both
    ///   are, and no end of either image is read;
    /// - **everywhere else**, the point the chain enters row `i` at, through
    ///   its map: between two images on one line (a split's cut), where
    ///   either image holds no line, at an open run's first row, and at
    ///   the closure joint of a loop that winds a period. At a cut that
    ///   point is an enclosure, and a reader that measures the arriving
    ///   image's exit against it measures that enclosure's own width.
    ///
    /// An image stores no ends, so this is where every reader of a corner
    /// reads it: the rectangle certificate (`props::quad_lane`), the loop
    /// polygon (`chart_region`), the spline trim's chain (`spline_trim`),
    /// the corners two spline walls share (`boolean::spline_known`) and
    /// an offset's extracted row (`replace_face`). The meet has one
    /// spelling ([`Lift::corner`]), which this, [`Lift::exit_point`] and
    /// [`Lift::stored_corners`] each read.
    ///
    /// # Panics
    ///
    /// Where `i` is not a row of the chain.
    #[must_use]
    pub fn joint_point(&self, i: usize) -> Point2<T> {
        match self.corner(i, End::Entry) {
            Some((meet, _)) => meet,
            None => self.rows[i].entry_exit_points().0,
        }
    }

    /// **The chart point row `i` leaves at**: the meet with the row after
    /// it where one holds a row of the chart and the other a column
    /// ([`Lift::joint_point`] of that joint), the point the chain leaves
    /// row `i` at through its own map everywhere else. With
    /// [`Lift::joint_point`]`(i)`, the two chart points an image runs
    /// between, for a reader of one row's extent.
    ///
    /// # Panics
    ///
    /// Where `i` is not a row of the chain.
    #[must_use]
    pub fn exit_point(&self, i: usize) -> Point2<T> {
        match self.corner(i, End::Exit) {
            Some((meet, _)) => meet,
            None => self.rows[i].entry_exit_points().1,
        }
    }

    /// **The corners at row `i`'s two ends, on the branch its image is
    /// STORED on**: at its edge's start and at its end (the edge's forward
    /// order), the coordinate along the row's line of the corner with the
    /// neighbouring row there ([`Lift::corner`]). The neighbour holds
    /// that coordinate; its STORED value is moved by the periods the lift
    /// moved the neighbour by, less the periods it moved row `i` by
    /// ([`Period::moved`]). Where the two were moved alike that is the
    /// neighbour's stored value itself, and one step off a domain's end
    /// is the other end itself, so a corner at a domain's end reads as
    /// that end at both ends of the row, however far the lift moved it.
    ///
    /// `None` at an end:
    /// - where there is no corner: the neighbour there holds no line
    ///   across the row's (a cut between two images on one line, an image
    ///   that is no iso image), or there is no neighbour (an open run's
    ///   end, the closure joint of a loop that winds a period);
    /// - where either row sits on the twin sheet (`lift.twin`): the twin
    ///   map is no whole-period move, and a stored branch across it is
    ///   not read here.
    ///
    /// `[None, None]` for a row whose own image holds no line. A reader
    /// takes its own image's value at a `None` end
    /// (`replace_face::iso_image_row`).
    ///
    /// For a reader of one edge's own image (its description), whose
    /// parameter along the line is the stored branch's.
    ///
    /// # Panics
    ///
    /// Where the lift moved the two rows by different periods of a
    /// channel that states no period: no deck element moves along an
    /// open channel, so such a lift is corrupt.
    ///
    /// [`Period::moved`]: geom_brep::Period::moved
    pub(crate) fn stored_corners(&self, i: usize, periods: ChartPeriods<T>) -> [Option<T>; 2] {
        let row = &self.rows[i];
        let Some(own) = row.held_line() else {
            return [None, None];
        };
        // The corner's coordinate along row `i`'s line is the channel the
        // neighbour holds, which is the channel row `i` moves along.
        let (moved_by, period): (fn(Deck) -> i32, _) = match own.line.axis {
            ChartAxis::U => (|deck| deck.v, periods.v()),
            ChartAxis::V => (|deck| deck.u, periods.u()),
        };
        let at = |end: End| {
            let (_, neighbour) = self.corner(i, end)?;
            let neighbour = &self.rows[neighbour];
            if row.lift.twin || neighbour.lift.twin {
                return None;
            }
            let stored = neighbour.stored?.value;
            Some(match moved_by(neighbour.lift) - moved_by(row.lift) {
                0 => stored,
                k => match period {
                    Some(period) => period.moved(stored, k),
                    None => unreachable!(
                        "stored_corners: rows {i} and its neighbour are {k} periods apart \
                         along a channel that states no period"
                    ),
                },
            })
        };
        let (entry, exit) = (at(End::Entry), at(End::Exit));
        if row.plus {
            [entry, exit]
        } else {
            [exit, entry]
        }
    }

    /// **The corner at one end of row `i`**: the meet of its held line
    /// with its neighbour's there (the row before it at its entry, the
    /// row after it at its exit), where one holds a row of the chart and
    /// the other a column, with the neighbour's index. The one spelling
    /// of [`Lift::joint_point`]'s meet. `None` where there is no
    /// neighbour, or the two do not hold a row and a column.
    fn corner(&self, i: usize, end: End) -> Option<(Point2<T>, usize)> {
        let neighbour = match end {
            End::Entry => self.before(i)?,
            End::Exit => self.after(i)?,
        };
        let (a, b) = (
            self.rows[i].held_line()?.line,
            self.rows[neighbour].held_line()?.line,
        );
        match (a.axis, b.axis) {
            (ChartAxis::U, ChartAxis::V) => Some((Point2::new(a.value, b.value), neighbour)),
            (ChartAxis::V, ChartAxis::U) => Some((Point2::new(b.value, a.value), neighbour)),
            _ => None,
        }
    }

    /// The row before row `i` in the chain: the last, for a first row of
    /// a loop that closes in its chart; none at an open run's first row.
    fn before(&self, i: usize) -> Option<usize> {
        match i.checked_sub(1) {
            Some(b) => Some(b),
            None if self.closes_in_chart() => self.rows.len().checked_sub(1),
            None => None,
        }
    }

    /// The row after row `i` in the chain: the first, for the last row of
    /// a loop that closes in its chart; none at an open run's last row.
    fn after(&self, i: usize) -> Option<usize> {
        if i + 1 < self.rows.len() {
            Some(i + 1)
        } else if self.closes_in_chart() {
            Some(0)
        } else {
            None
        }
    }
}

/// An end of a row, as the chain runs it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum End {
    /// Where the chain enters the row.
    Entry,
    /// Where the chain leaves it.
    Exit,
}

impl<T: Real> IntoIterator for Lift<T> {
    type Item = LiftedRow<T>;
    type IntoIter = std::vec::IntoIter<LiftedRow<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.into_iter()
    }
}

impl<'a, T: Real> IntoIterator for &'a Lift<T> {
    type Item = &'a LiftedRow<T>;
    type IntoIter = core::slice::Iter<'a, LiftedRow<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.rows.iter()
    }
}

impl<T: Decide> Lift<T> {
    /// **The lift a finished body stores**: each half-edge of `r#loop`
    /// in cycle order from its `first` ([`loop_rows`]), its stored image
    /// moved by the deck transformation the stored elements sum to.
    ///
    /// A rowless half is a gap, whatever its edge: a finished body holds
    /// no null edge, and mid-operation a reader that steps over one asks
    /// [`Lift::read_past_placeholders`].
    ///
    /// # Errors
    ///
    /// [`LiftGap`] naming the first half-edge, in cycle order, that
    /// stores no image or no element on the joint into it, or whose
    /// image its lift does not apply to.
    ///
    /// # Panics
    ///
    /// Where `r#loop`, its face or a member does not resolve, or its
    /// walk does not close on the half-edges that claim it
    /// ([`loop_rows`]).
    #[track_caller]
    pub fn read(body: &Body<T>, r#loop: LoopKey) -> Result<Self, LiftGap> {
        Self::read_with(body, r#loop, Scaffolding::Refused, Deck::IDENTITY)
    }

    /// [`Lift::read`] in another gauge: the chain moved by `start`, each
    /// row's lift `start` composed after its own, and its image moved by
    /// that sum from the stored row — rounded afresh, so not [`Lift::read`]'s
    /// image moved by `start` bit for bit.
    ///
    /// # Errors
    ///
    /// [`Lift::read`]'s, and a row `start` does not apply to.
    ///
    /// # Panics
    ///
    /// [`Lift::read`]'s.
    #[track_caller]
    pub(crate) fn read_from(body: &Body<T>, r#loop: LoopKey, start: Deck) -> Result<Self, LiftGap> {
        Self::read_with(body, r#loop, Scaffolding::Refused, start)
    }

    /// **[`Lift::read`] past the loop's placeholders**
    /// ([`crate::null::is_placeholder`]): a placeholder's rowless halves
    /// are left out of the chain, as the analytic region reader leaves
    /// the edge out of its boundary ([`Scaffolding::Skipped`]). The skip
    /// is exact. The edge is one point, so the halves either side of it
    /// meet there, and the mint gives the joints into its halves
    /// [`JointElement::IDENTITY`](crate::joint::JointElement::IDENTITY)
    /// while the joint out of it keeps its successor's element
    /// (`Body::mev_fan_execute`), so every other member's lift is the
    /// one it has with the placeholder gone. A null edge whose ends have
    /// parted is no placeholder, and any other rowless half is a gap.
    ///
    /// # Errors
    ///
    /// [`Lift::read`]'s, but for a placeholder's half.
    ///
    /// # Panics
    ///
    /// [`Lift::read`]'s, and [`crate::null::is_placeholder`]'s.
    #[track_caller]
    pub(crate) fn read_past_placeholders(body: &Body<T>, r#loop: LoopKey) -> Result<Self, LiftGap> {
        Self::read_with(body, r#loop, Scaffolding::Skipped, Deck::IDENTITY)
    }

    /// The one read [`Lift::read`], [`Lift::read_from`] and
    /// [`Lift::read_past_placeholders`] make, a placeholder's half as
    /// `scaffolding` says, in the gauge `start`.
    #[track_caller]
    fn read_with(
        body: &Body<T>,
        r#loop: LoopKey,
        scaffolding: Scaffolding,
        start: Deck,
    ) -> Result<Self, LiftGap> {
        let cycle = loop_rows(body, r#loop);
        let face = proven(&body.loops, r#loop, EntityId::Loop).face;
        // A row on a pending chart lifts nowhere: it is a gap.
        let surface = body.proven_face(face).surface().ok();
        // No band is at hand, so the periods are the chart's as stated:
        // on every element a door wrote they are the decided ones
        // ([`ChartPeriods::stated`]).
        let periods = surface.map(ChartPeriods::stated);
        let mut stored = Vec::with_capacity(cycle.len());
        let mut elements = Vec::with_capacity(cycle.len());
        for &half_edge in &cycle {
            let Some(element) = body.joint(half_edge) else {
                return Err(LiftGap { half_edge });
            };
            let row = body.pcurve(half_edge);
            if row.is_none()
                && !(scaffolding == Scaffolding::Skipped
                    && crate::null::is_placeholder(body, edge_record(body, half_edge).key()))
            {
                return Err(LiftGap { half_edge });
            }
            stored.push(row);
            elements.push(element);
        }
        let rows = cycle
            .into_iter()
            .zip(stored)
            .zip(lift_decks(&elements))
            .filter_map(|((half_edge, row), lift)| Some((half_edge, row?, start.compose(lift))))
            .map(|(half_edge, row, lift)| {
                let pcurve = surface
                    .zip(periods)
                    .and_then(|(surface, periods)| lift.apply(row.pcurve(), surface, periods))
                    .ok_or(LiftGap { half_edge })?;
                Ok(LiftedRow {
                    half_edge,
                    lift,
                    stored: row.pcurve().held_line().map(|held| held.line),
                    pcurve,
                    params: row.params(),
                    plus: is_plus(body, half_edge),
                })
            })
            .collect::<Result<Vec<_>, LiftGap>>()?;
        Ok(Self {
            rows,
            winding: Some(Winding::of(elements)),
        })
    }

    /// **The lift decided at `band`, storing nothing**: `halves` —
    /// consecutive members of one loop of `face`, in loop order — walked
    /// in `surface`'s chart by the mint's own walk ([`walk_cycle`]):
    /// each certified edge's image derived in closed form on the chart's
    /// principal branch ([`chart_pcurve_over`](geom_brep::chart_pcurve_over)) and every joint between
    /// two of them decided ([`decide_joint`]) on the face's side, its
    /// stored sense. The whole loop where `closes`, with its closure
    /// joint and its winding; otherwise an open run.
    ///
    /// A half of an edge with no certified curve (null scaffolding) is a
    /// point and is left out of the chain.
    ///
    /// # Errors
    ///
    /// [`LiftRefusal`]: an image's derivation refused, or a joint
    /// decided no element.
    ///
    /// # Panics
    ///
    /// Where `face` or a member of `halves` does not resolve.
    ///
    /// [`decide_joint`]: super::decide_joint
    #[track_caller]
    pub(crate) fn decided(
        body: &Body<T>,
        face: FaceKey,
        surface: &Surface<T>,
        halves: &[HalfEdgeKey],
        closes: bool,
        band: Band,
    ) -> Result<Self, LiftRefusal> {
        let chart = DescribedChart::of(surface);
        let sense = proven(&body.faces, face, EntityId::Face).sense;
        // Each half's carrier and its window, from one read of the edge's
        // certified curve.
        let charted: Vec<Charted<T>> = halves
            .iter()
            .filter_map(|&he| {
                let curve = half_edge_curve(body, he).ok()?;
                let (t0, t1) = curve.params();
                Some((he, curve.carrier().clone(), t0, t1))
            })
            .collect();
        let item = |i: usize| -> Result<WalkItem<T, DerivedRow<T>>, LiftRefusal> {
            let (he, carrier, t0, t1) = &charted[i];
            let ends = super::decision::singular_ends(body, *he, carrier, surface, band);
            let pcurve = geom_brep::chart_pcurve_over_ends(carrier, *t0, *t1, ends, surface, band)
                .map_err(LiftRefusal::Image)?;
            Ok(walk_item(
                (
                    carrier.clone(),
                    ChartImage::Image {
                        pcurve,
                        t0: *t0,
                        t1: *t1,
                    },
                ),
                is_plus(body, *he),
                entry_vertex(body, *he),
                stored_edge(body, *he),
            ))
        };
        let wedges = crate::wedge_reach::WedgeReach::new(body);
        let reach = |i: usize| stored_reach(&wedges, body, charted[i].0);
        let walked = walk_cycle(
            chart,
            sense,
            charted.len(),
            item,
            |_| None,
            &reach,
            band,
            closes,
        )
        .map_err(|fail| match fail {
            WalkFail::Item(refusal) => refusal,
            WalkFail::Miss { miss, .. } => LiftRefusal::Joint(miss),
            WalkFail::NotClosed => LiftRefusal::NotClosed,
        })?;
        let periods = ChartPeriods::of(surface, band);
        // An open run's first member joins nothing, and the sum does not
        // cross the joint into the first member.
        let elements: Vec<JointElement> = walked
            .iter()
            .map(|pinned| pinned.3.unwrap_or(JointElement::IDENTITY))
            .collect();
        let winding = closes.then(|| Winding::of(walked.iter().filter_map(|pinned| pinned.3)));
        let mut rows = Vec::with_capacity(walked.len());
        for (i, (((_, image), _, _, _), lift)) in
            walked.into_iter().zip(lift_decks(&elements)).enumerate()
        {
            let (half_edge, params) = (charted[i].0, (charted[i].2, charted[i].3));
            let pcurve = lift
                .apply(image.pcurve(), surface, periods)
                .ok_or(LiftRefusal::Joint(PinMiss::Discontinuity))?;
            rows.push(LiftedRow {
                half_edge,
                lift,
                stored: image.pcurve().held_line().map(|held| held.line),
                pcurve,
                params,
                plus: is_plus(body, half_edge),
            });
        }
        Ok(Self { rows, winding })
    }
}

/// Why [`Lift::decided`] decided no lift.
#[derive(Debug)]
pub(crate) enum LiftRefusal {
    /// A half-edge's closed-form image refused.
    Image(PcurveCertifyError),
    /// A joint of the chain decided no element.
    Joint(PinMiss),
    /// The loop's joints compose to a winding no loop has.
    NotClosed,
}

/// **Each member's lift, from the elements of a cycle's joints** —
/// `elements[i]` the element into member `i`, in cycle order. The lift
/// starts on the first member's own image, at the identity, and crosses
/// every joint after it once round the cycle
/// ([`JointElement::follow`]); the joint into the first member is the
/// one it does not cross, where the loop's winding shows ([`Winding`]).
pub(crate) fn lift_decks(elements: &[JointElement]) -> Vec<Deck> {
    let mut lift = Deck::IDENTITY;
    elements
        .iter()
        .enumerate()
        .map(|(i, element)| {
            if i > 0 {
                lift = element.follow(lift);
            }
            lift
        })
        .collect()
}

/// **The lift of the loop `halves` are members of, with each half's row
/// in it** ([`Lift::read`]): for a reader that walks `halves` in its own
/// order and reads the lift's rows and joints ([`Lift::joint_point`],
/// [`Lift::exit_point`]). A half with no row in that lift reads `None`
/// (a half of another loop). `None` for no halves, and where the loop
/// has no lift to read (a member stores no image, or no element on the
/// joint into it): a reader then has no row for any half, and each
/// refuses or derives its own.
///
/// # Panics
///
/// [`Lift::read`]'s, and where the first half does not resolve.
#[track_caller]
pub(crate) fn lift_of_halves<T: Decide>(
    body: &Body<T>,
    halves: &[HalfEdgeKey],
) -> Option<(Lift<T>, Vec<Option<usize>>)> {
    let r#loop = proven(&body.half_edges, *halves.first()?, EntityId::HalfEdge).parent_loop;
    // No lift to read is this reader's `None`, not a fallback to another
    // reading: each caller refuses or derives per half, by name.
    let lift = Lift::read(body, r#loop).ok()?;
    let rows = halves
        .iter()
        .map(|he| lift.rows.iter().position(|row| row.half_edge == *he))
        .collect();
    Some((lift, rows))
}

/// Each of `halves`' images as its loop's lift places it
/// ([`lift_of_halves`]'s rows), `None` for a half with no row and for
/// every half where the loop has no lift to read.
///
/// # Panics
///
/// [`lift_of_halves`]'s.
#[track_caller]
pub(crate) fn lifted_images<T: Decide>(
    body: &Body<T>,
    halves: &[HalfEdgeKey],
) -> Vec<Option<Pcurve<T>>> {
    match lift_of_halves(body, halves) {
        Some((lift, rows)) => rows
            .into_iter()
            .map(|row| row.map(|row| lift.rows[row].pcurve.clone()))
            .collect(),
        None => vec![None; halves.len()],
    }
}

/// **Whether `he` misses a row** ([`StoredRows::gaps`]): it stores no
/// image, or the joint into it from `before`, its predecessor, has no
/// element (`element` false) where `before` stores an image — a joint
/// between two stored images owes its element.
pub(super) fn misses_a_row<T: Real>(
    body: &Body<T>,
    before: HalfEdgeKey,
    he: HalfEdgeKey,
    element: bool,
) -> bool {
    body.pcurve(he).is_none() || (body.pcurve(before).is_some() && !element)
}

/// [`StoredRows`] for `face`, a face of the body: its loops walked
/// once.
///
/// # Panics
///
/// [`crate::live::FaceView::cycles`]'.
#[track_caller]
pub(crate) fn stored_rows<T: Decide>(body: &Body<T>, face: FaceKey) -> StoredRows {
    let loops: Vec<_> = body.proven_face(face).cycles().collect();
    let mut gaps = Vec::new();
    let mut stores = false;
    for (lk, cycle) in &loops {
        for (i, &he) in cycle.iter().enumerate() {
            let before = cycle[(i + cycle.len() - 1) % cycle.len()];
            stores |= body.pcurve(he).is_some();
            if misses_a_row(body, before, he, body.joint(he).is_some()) {
                gaps.push(RowGap {
                    half_edge: he,
                    r#loop: *lk,
                });
            }
        }
    }
    StoredRows {
        loops,
        stores,
        gaps,
    }
}

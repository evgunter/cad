//! **The lift and its readers**: a face's stored rows and a loop's
//! [`Lift`], decided by the walk or read off what the mint stored.

use geom::Surface;
use geom_brep::{ChartImage, Pcurve, PcurveCertifyError, chart_pcurve_over};
use geom_core::predicate::Band;
use geom_core::{Decide, Real};

use super::{
    ChartPeriods, DerivedRow, DescribedChart, PinMiss, WalkFail, WalkItem, edge_record, entry_exit,
    entry_vertex, half_edge_carrier, is_plus, stored_edge, stored_reach, walk_cycle, walk_item,
};
use crate::body::Body;
use crate::entity::{EntityId, FaceKey, HalfEdgeKey, LoopKey};
use crate::joint::{Deck, JointElement, Winding};
use crate::live::proven;
use crate::null::Scaffolding;

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
        let (edge, data) = edge_record(body, he);
        open |= body.edge_curve_linked(edge, data).null_scaffold().is_some();
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
    /// The carrier interval the image is over, in the edge's forward
    /// order.
    pub params: (T, T),
    /// Whether the chain traverses the image forward.
    pub plus: bool,
}

impl<T: Real> LiftedRow<T> {
    /// The carrier parameters the chain enters and leaves the image at.
    #[must_use]
    pub fn entry_exit(&self) -> (T, T) {
        entry_exit(self.plus, self.params.0, self.params.1)
    }
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
        let surface = body.proven_face(face).surface();
        // No band is at hand, so the periods are the chart's as stated:
        // on every element a door wrote they are the decided ones
        // ([`ChartPeriods::stated`]).
        let periods = ChartPeriods::stated(surface);
        let mut stored = Vec::with_capacity(cycle.len());
        let mut elements = Vec::with_capacity(cycle.len());
        for &half_edge in &cycle {
            let Some(element) = body.joint(half_edge) else {
                return Err(LiftGap { half_edge });
            };
            let row = body.pcurve(half_edge);
            if row.is_none()
                && !(scaffolding == Scaffolding::Skipped
                    && crate::null::is_placeholder(body, edge_record(body, half_edge).0))
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
                let pcurve = lift
                    .apply(row.pcurve(), surface, periods)
                    .ok_or(LiftGap { half_edge })?;
                Ok(LiftedRow {
                    half_edge,
                    lift,
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
    /// principal branch ([`chart_pcurve_over`]) and every joint between
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
        let Some(chart) = DescribedChart::of(surface) else {
            return Err(LiftRefusal::Placeholder);
        };
        let sense = proven(&body.faces, face, EntityId::Face).sense;
        let charted: Vec<(HalfEdgeKey, geom::Curve3<T>, T, T)> = halves
            .iter()
            .filter_map(|&he| {
                half_edge_carrier(body, he)
                    .ok()
                    .map(|(carrier, t0, t1)| (he, carrier, t0, t1))
            })
            .collect();
        let item = |i: usize| -> Result<WalkItem<T, DerivedRow<T>>, LiftRefusal> {
            let (he, carrier, t0, t1) = &charted[i];
            let pcurve =
                chart_pcurve_over(carrier, *t0, *t1, surface, band).map_err(LiftRefusal::Image)?;
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
        for (i, (((_, image), t0, t1, _), lift)) in
            walked.into_iter().zip(lift_decks(&elements)).enumerate()
        {
            let half_edge = charted[i].0;
            let pcurve = lift
                .apply(image.pcurve(), surface, periods)
                .ok_or(LiftRefusal::Joint(PinMiss::Discontinuity))?;
            rows.push(LiftedRow {
                half_edge,
                lift,
                pcurve,
                params: (t0, t1),
                plus: is_plus(body, half_edge),
            });
        }
        Ok(Self { rows, winding })
    }
}

/// Why [`Lift::decided`] decided no lift.
#[derive(Debug)]
pub(crate) enum LiftRefusal {
    /// The chart is the placeholder: it has no lever to decide a joint
    /// by.
    Placeholder,
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

/// [`Lift::read`] for a reader that walks `halves` — members of one
/// loop, in the reader's own order: each one's lifted image, or `None`
/// for every one where that loop has no lift to read (a member stores
/// no image, or no element on the joint into it). Empty for no halves.
///
/// # Panics
///
/// [`Lift::read`]'s, and where the first half does not resolve.
#[track_caller]
pub(crate) fn lifted_images<T: Decide>(
    body: &Body<T>,
    halves: &[HalfEdgeKey],
) -> Vec<Option<Pcurve<T>>> {
    let Some(&first) = halves.first() else {
        return Vec::new();
    };
    let r#loop = proven(&body.half_edges, first, EntityId::HalfEdge).parent_loop;
    let Ok(lift) = Lift::read(body, r#loop) else {
        return vec![None; halves.len()];
    };
    let mut by_half: slotmap::SecondaryMap<HalfEdgeKey, Pcurve<T>> = slotmap::SecondaryMap::new();
    for row in lift {
        by_half.insert(row.half_edge, row.pcurve);
    }
    halves.iter().map(|&he| by_half.get(he).cloned()).collect()
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

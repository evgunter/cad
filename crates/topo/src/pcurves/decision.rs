//! **The walk and the one joint decision**: [`walk_cycle`] visits a
//! cycle's joints and [`decide_joint`] decides each one's element.

use geom::Surface;
use geom_brep::{BranchMiss, ChartImage, Pcurve, PcurveCache, Period, WalkStop, first_within};
use geom_core::k_stats::decide;
use geom_core::predicate::Band;
use geom_core::{Decide, Indeterminate, Margin, Real, Sign, SupSpeed};

use super::{
    ChartPeriods, DescribedChart, NetChannel, VPeriod, chart_u_arm, half_edge_record, net_reach,
    shift_v_branch, sphere_twin, v_meter,
};
use crate::body::Body;
use crate::entity::{EdgeKey, HalfEdgeKey};
use crate::joint::{Deck, JointElement, Winding};

/// One half-edge's minted chart curve, before certification: the
/// image, its carrier and the carrier interval, and the element of the
/// joint into it, keyed on whatever the walk names its half-edges by — a
/// [`HalfEdgeKey`] in the pass, a [`SiteHalf`] in an Euler operator's
/// plan.
///
/// [`SiteHalf`]: super::SiteHalf
pub(crate) struct Walked<T: Real, K = HalfEdgeKey> {
    pub(super) key: K,
    pub(super) carrier: geom::Curve3<T>,
    /// The image and its interval, with its certificate where the
    /// derivation's lane choice already read one.
    pub(super) image: ChartImage<T>,
    pub(super) element: Option<JointElement>,
}

impl<T: Real, K> Walked<T, K> {
    /// The chart image.
    pub(super) fn pcurve(&self) -> &Pcurve<T> {
        self.image.pcurve()
    }

    /// The certified row, where the derivation's lane choice already
    /// read the certificate; the walked row back otherwise.
    pub(super) fn into_certified(
        self,
    ) -> Result<(K, PcurveCache<T>, Option<JointElement>), Box<Self>> {
        match self.image {
            ChartImage::Certified(cache) => Ok((self.key, cache, self.element)),
            image @ (ChartImage::Image { .. } | ChartImage::Stated { .. }) => {
                Err(Box::new(Self { image, ..self }))
            }
        }
    }

    /// The carrier interval the image is over.
    pub(super) fn window(&self) -> (T, T) {
        self.image.window()
    }
}

/// A half-edge's derived image with the carrier it was derived on: the
/// image a walk hands [`walk_cycle`] and gets back, the carrier riding
/// with it.
pub(super) type DerivedRow<T> = (geom::Curve3<T>, ChartImage<T>);

/// **What [`walk_cycle`] reads an item's image through.** It reads the
/// image's pcurve and hands the item's image back as it came, so the row
/// a walk pins is the row it derived, by type: no copy of it is made.
pub(crate) trait WalkImage<T: Real> {
    /// The image's pcurve.
    fn pcurve(&self) -> &Pcurve<T>;
    /// The carrier the image is of.
    fn carrier(&self) -> &geom::Curve3<T>;
}

impl<T: Real> WalkImage<T> for DerivedRow<T> {
    fn pcurve(&self) -> &Pcurve<T> {
        self.1.pcurve()
    }

    fn carrier(&self) -> &geom::Curve3<T> {
        &self.0
    }
}

/// A site mint's image of one half: the row it keeps, borrowed as it
/// stands, or a row it derives.
pub(crate) enum SiteImage<'a, T: Real> {
    /// A stored row the door leaves as found, with its edge's carrier.
    Kept(&'a Pcurve<T>, geom::Curve3<T>),
    /// A row derived for the site.
    Derived(DerivedRow<T>),
}

impl<T: Real> WalkImage<T> for SiteImage<'_, T> {
    fn pcurve(&self) -> &Pcurve<T> {
        match self {
            Self::Kept(pcurve, _) => pcurve,
            Self::Derived(row) => row.pcurve(),
        }
    }

    fn carrier(&self) -> &geom::Curve3<T> {
        match self {
            Self::Kept(_, carrier) => carrier,
            Self::Derived(row) => row.carrier(),
        }
    }
}

impl<T: Real, K> Walked<T, K> {
    /// The walked row of `key`: its derivation, and its joint's element.
    pub(super) fn pinned(
        key: K,
        (carrier, image): DerivedRow<T>,
        element: Option<JointElement>,
    ) -> Self {
        Self {
            key,
            carrier,
            image,
            element,
        }
    }
}

/// The [`WalkItem`] [`walk_cycle`] reads for a derived row.
pub(super) fn walk_item<T: Real>(
    row: DerivedRow<T>,
    plus: bool,
    vertex: geom_core::Point3<T>,
    edge: WalkEdge,
) -> WalkItem<T, DerivedRow<T>> {
    let (t0, t1) = row.1.window();
    WalkItem {
        image: row,
        t0,
        t1,
        plus,
        vertex,
        edge,
    }
}

/// The [`WalkEdge`] of a half-edge the body holds.
pub(super) fn stored_edge<T: Real>(body: &Body<T>, half_edge: HalfEdgeKey) -> WalkEdge {
    WalkEdge::Stored(half_edge_record(body, half_edge).edge)
}

/// One half-edge of a cycle as [`walk_cycle`] reads it: its chart
/// image on the chart's principal branch — derived, or borrowed from a
/// row that stands — its carrier interval, whether the loop traverses
/// it forward, and the 3-D point of the vertex it enters at — the joint
/// [`decide_joint`] reads the chart's singular set and the azimuth's
/// lever on.
pub(crate) struct WalkItem<T: Real, I> {
    pub(super) image: I,
    pub(super) t0: T,
    pub(super) t1: T,
    pub(super) plus: bool,
    pub(super) vertex: geom_core::Point3<T>,
    pub(super) edge: WalkEdge,
}

/// Which edge a walked half is a half of, as far as [`walk_cycle`]
/// reads it: whether two consecutive halves are one edge's two.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WalkEdge {
    /// An edge the body holds.
    Stored(EdgeKey),
    /// The edge an Euler operator is minting ([`SiteHalf::NewPlus`],
    /// [`SiteHalf::NewMinus`]).
    ///
    /// [`SiteHalf::NewMinus`]: super::SiteHalf::NewMinus
    /// [`SiteHalf::NewPlus`]: super::SiteHalf::NewPlus
    New,
}

/// One edge at a joint, as the singular-set rule reads it
/// ([`singular_turn`]): its carrier, the carrier parameter at the
/// joint's vertex, and the parameter at its other end.
#[derive(Clone, Copy)]
pub(crate) struct EdgeEnd<'a, T: Real> {
    pub(crate) carrier: &'a geom::Curve3<T>,
    pub(crate) at: T,
    pub(crate) far: T,
}

/// One half-edge either side of a joint, as [`JointSides::of_halves`]
/// reads it: its edge's carrier, the carrier parameters the loop enters
/// and leaves it at, whether the loop traverses it forward, which edge
/// it is a half of, and its edge's reach ([`edge_reach`]).
#[derive(Clone, Copy)]
pub(crate) struct JointHalf<'a, T: Real> {
    pub(crate) carrier: &'a geom::Curve3<T>,
    pub(crate) entry: T,
    pub(crate) exit: T,
    pub(crate) plus: bool,
    pub(crate) edge: WalkEdge,
    /// The extent of the faces the edge bounds, asked only at a joint
    /// on the singular set; `None` where it is not read.
    pub(crate) reach: &'a dyn Fn() -> Option<geom_core::Lever<T>>,
}

/// **The face's side of a joint**: what [`decide_joint`] reads of a
/// joint beyond its two images, where the vertex is on the chart's
/// singular set ([`singular_turn`]). Built by [`JointSides::of_halves`]
/// alone, so what the two halves are to each other is read one way.
#[derive(Clone, Copy)]
pub(crate) struct JointSides<'a, T: Real> {
    /// The sense of the face the loop bounds.
    sense: bool,
    /// Whether the walk leaves the vertex along the edge it arrived on:
    /// the two halves are one edge's two.
    mates: bool,
    /// The edge the walk arrives on.
    arrived: EdgeEnd<'a, T>,
    /// The edge the walk leaves on.
    leaving: EdgeEnd<'a, T>,
    /// The two edges' reaches, the arriving edge's first.
    reaches: [&'a dyn Fn() -> Option<geom_core::Lever<T>>; 2],
    /// **The rest of the loop**: its other joints' elements composed,
    /// where the loop is closed and every one of them is decided. Read
    /// only where two distinct edges leave the point along one
    /// direction ([`singular_turn`], the slit).
    rest: Option<Winding>,
}

impl<'a, T: Real> JointSides<'a, T> {
    /// The joint from `arrived` into `leaving`, two consecutive halves
    /// of a loop of a face of sense `sense`, with the rest of that loop
    /// where it is known (`rest`).
    pub(crate) fn of_halves(
        sense: bool,
        arrived: JointHalf<'a, T>,
        leaving: JointHalf<'a, T>,
        rest: Option<Winding>,
    ) -> Self {
        Self {
            sense,
            mates: arrived.edge == leaving.edge && arrived.plus != leaving.plus,
            arrived: EdgeEnd {
                carrier: arrived.carrier,
                at: arrived.exit,
                far: arrived.entry,
            },
            leaving: EdgeEnd {
                carrier: leaving.carrier,
                at: leaving.entry,
                far: leaving.exit,
            },
            reaches: [arrived.reach, leaving.reach],
            rest,
        }
    }

    /// **The lever of the turn between the two edges**: the extent of
    /// the faces they bound, the shorter of the two edges' reaches
    /// ([`edge_reach`]). `None` where either is not read.
    fn lever(&self) -> Option<geom_core::Lever<T>> {
        let [arrived, leaving] = self.reaches;
        Some(arrived()?.min(leaving()?))
    }
}

/// **An edge's reach, for the turn at a joint on the singular set**: the
/// extent of the faces it bounds, how far their boundary vertices and
/// the edge's own ends stand from the middle of its span
/// ([`mod@crate::wedge_reach`]). An angle between two edges on a face is
/// levered at that extent, not at how long either edge is.
///
/// **One reading, at rest and mid-door.** Tier 3 audits a stored element
/// with the reach of the finished body's edge, over both its faces. A
/// door decides before its faces are as it leaves them, so it reads the
/// same measure over points that are among the finished faces':
/// - the mint's walk, the decided lift, a kill's turn and tier 3 read
///   the edge the body holds ([`stored_reach`]);
/// - a split reads each child over its own span and its edge's faces,
///   which is the child's reach once the split lands (the faces gain
///   only the split vertex, a child's own end), and a neighbour as the
///   body holds it, whose faces only gain that vertex;
/// - a site mint reads the face it mints as the door leaves it, alone:
///   the vertices of its loops ([`site_rows`]).
///
/// So a door's reach is the finished edge's or shorter, and a turn a
/// door decides is one tier 3 decides the same way: a longer lever
/// only moves a margin farther from its mark.
///
/// [`site_rows`]: super::site_rows
pub(super) fn edge_reach<T: Decide>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    points: impl IntoIterator<Item = geom_core::Point3<T>>,
) -> geom_core::Lever<T> {
    geom_brep::points_lever_from(
        carrier.mid_point(t0, t1),
        [carrier.eval(t0), carrier.eval(t1)]
            .into_iter()
            .chain(points),
    )
}

/// [`edge_reach`] of the edge of `half_edge` as the body holds it, over
/// its two faces. `None` where the edge has no certified curve.
pub(super) fn stored_reach<T: Decide>(
    wedges: &crate::wedge_reach::WedgeReach<'_, T>,
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Option<geom_core::Lever<T>> {
    wedges.edge(half_edge_record(body, half_edge).edge)
}

/// Why the singular-set rule decided no turn ([`singular_turn`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SingularMiss {
    /// Two distinct edges leave the singular point along one direction,
    /// and the rest of their loop does not say the face wraps the point:
    /// an overlap or a cusp, a sector of no decided width.
    EdgesCoincide,
    /// The two edges leave the point on different sheets of the chart
    /// that no element joins: a cone's two nappes. The face crosses the
    /// point, and no one side of it is the face's.
    TwoNappes,
    /// An edge of the joint does not leave a cone's apex past the band,
    /// so the nappe it lies on is not read.
    NoNappe,
    /// An edge of the joint has no certified curve, so the faces' extent
    /// along it, the lever the angle at the point is metered at, is not
    /// read.
    NoLever,
}

impl SingularMiss {
    /// What to do about it.
    #[must_use]
    pub fn recourse(self) -> &'static str {
        match self {
            Self::EdgesCoincide => {
                "Recourse: open the face's corner at that point, so its two edges leave it \
                 along different directions, or merge the two edges into one"
            }
            Self::TwoNappes => {
                "Recourse: divide the face at that point, so each piece lies on one nappe"
            }
            Self::NoNappe => {
                "Recourse: lengthen that edge past the tolerance, or merge its two ends"
            }
            Self::NoLever => {
                "Recourse: describe that edge's curve before the face's boundary is minted"
            }
        }
    }
}

impl core::fmt::Display for SingularMiss {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::EdgesCoincide => {
                "its two edges leave the point along one direction (an overlap or a cusp)"
            }
            Self::TwoNappes => "its two edges leave the point on different nappes",
            Self::NoNappe => "one of its edges does not leave the apex past the tolerance",
            Self::NoLever => {
                "one of its edges has no certified curve, and its angle at the point has no \
                 lever"
            }
        })
    }
}

/// One image [`walk_cycle`] placed: the image with its carrier
/// interval, and the element of the joint into it — `None` only for the
/// first item of an open run, which joins nothing.
type Pinned<T, I> = (I, T, T, Option<JointElement>);

/// Why [`decide_joint`] decided no element.
#[derive(Debug)]
pub(crate) enum PinMiss {
    /// The image meets the predecessor's exit on no lift: its gap sits
    /// on a mark, or a spline chart's gap is definite.
    Discontinuity,
    /// A decision escalated.
    Escalated(Indeterminate),
    /// The joint is on a spline net closed in `v` whose whole `v` sweep
    /// moves a point within the band (`pcurve_loop_v_stretch` undecided,
    /// [`net_reach`]), or whose `u` sweep does on a net closed in `u`
    /// (`pcurve_loop_pole_joint` undecided): that channel's periods are
    /// not decided, and the joint refuses under that name rather than
    /// writing an element.
    NetStretchUndecided(Indeterminate),
    /// The joint is on the chart's singular set and its turn on the
    /// face's side is decided by nothing ([`singular_turn`]).
    Singular(SingularMiss),
}

/// Why [`walk_cycle`] refused, with the index of the item it refused
/// at where there is one.
pub(crate) enum WalkFail<E> {
    /// The caller's item producer refused (a derivation that refused,
    /// or a half with no carrier).
    Item(E),
    /// No element joins item `index` to the item before it.
    Miss {
        /// The cycle position of the item.
        index: usize,
        /// Why.
        miss: PinMiss,
    },
    /// The loop does not close: its elements compose to a winding no
    /// loop has ([`Winding::closes`]).
    NotClosed,
}

/// **The one-branch walk, stated under `Decide`.** Walks `len` items in
/// cycle order, asking `item` for each in turn (so an item's refusal
/// is reported in cycle order, before any later item is read), decides
/// the element of each joint between consecutive images
/// ([`decide_joint`]) that `kept` does not hand it, and — when `closes`
/// — the closure joint into the first item and the loop's winding
/// ([`Winding::closes`]), kept elements among it; an open run of a loop
/// ([`walk_runs`]) has no closure. Returns each item's image with its
/// interval and the element into it.
///
/// Every joint is decided between two IMAGES, each a function of its
/// edge and the chart, so no element depends on where the walk starts:
/// the closure joint is decided exactly as every other, and the winding
/// is read off conjugation invariants.
///
/// This is the whole of the minting walk except the derivation, and
/// none of it needs the fitted lane: its callers — [`walk_loop`] and
/// [`walk_runs`], the pass's, and [`site_rows`], the Euler operators'
/// — hand it their own derivation through `item`. The pass keeps no
/// joint; the site mint keeps the element of every joint its door
/// leaves as found, between two images that stand.
///
/// [`site_rows`]: super::site_rows
/// [`walk_loop`]: super::walk_loop
/// [`walk_runs`]: super::walk_runs
#[allow(clippy::too_many_arguments)]
pub(super) fn walk_cycle<T: Decide, I: WalkImage<T>, E>(
    chart: DescribedChart<'_, T>,
    sense: bool,
    len: usize,
    mut item: impl FnMut(usize) -> Result<WalkItem<T, I>, E>,
    kept: impl Fn(usize) -> Option<JointElement>,
    reach: &dyn Fn(usize) -> Option<geom_core::Lever<T>>,
    band: Band,
    closes: bool,
) -> Result<Vec<Pinned<T, I>>, WalkFail<E>> {
    /// One placed item's traversal: its entry and exit parameters, its
    /// direction, its edge and its entry vertex.
    struct Traversed<T: Real> {
        entry: T,
        exit: T,
        plus: bool,
        edge: WalkEdge,
        vertex: geom_core::Point3<T>,
    }
    // The chart's own periods (`ChartPeriods::of`): in `u`, `τ` on an
    // analytic azimuth chart, the knot-domain length on a NURBS chart
    // closed in u, and NO shift at all on a chart that does not wrap;
    // in `v`, likewise for the second channel.
    let periods = ChartPeriods::of(chart.surface(), band);
    // The joint from the placed item `before` into `image`, traversed
    // as `this`, with the rest of the loop where it is known.
    let joint = |before: (&I, &Traversed<T>, usize),
                 image: &I,
                 this: &Traversed<T>,
                 index: usize,
                 rest: Option<Winding>| {
        let (arrived, prev, arrived_index) = before;
        let arrived_reach = || reach(arrived_index);
        let leaving_reach = || reach(index);
        decide_joint(
            chart,
            image.pcurve(),
            this.entry,
            arrived.pcurve().eval(prev.exit),
            this.vertex,
            periods,
            band,
            JointSides::of_halves(
                sense,
                JointHalf {
                    carrier: arrived.carrier(),
                    entry: prev.entry,
                    exit: prev.exit,
                    plus: prev.plus,
                    edge: prev.edge,
                    reach: &arrived_reach,
                },
                JointHalf {
                    carrier: image.carrier(),
                    entry: this.entry,
                    exit: this.exit,
                    plus: this.plus,
                    edge: this.edge,
                    reach: &leaving_reach,
                },
                rest,
            ),
        )
    };
    let mut ends: Vec<Traversed<T>> = Vec::with_capacity(len);
    let mut out: Vec<Pinned<T, I>> = Vec::with_capacity(len);
    // The one joint of a closed loop where two distinct edges leave a
    // singular point along one direction: its decision reads the rest
    // of the loop ([`JointSides::rest`]), so it is asked again once
    // every other joint is decided.
    let mut slit: Option<usize> = None;
    for index in 0..len {
        let WalkItem {
            image,
            t0,
            t1,
            plus,
            vertex,
            edge,
        } = item(index).map_err(WalkFail::Item)?;
        let (entry, exit) = entry_exit(plus, t0, t1);
        let this = Traversed {
            entry,
            exit,
            plus,
            edge,
            vertex,
        };
        let element = match (out.last().zip(ends.last()), kept(index)) {
            (None, _) => None,
            (Some(_), Some(element)) => Some(element),
            (Some((placed, prev)), None) => {
                match joint((&placed.0, prev, index - 1), &image, &this, index, None) {
                    Ok(element) => Some(element),
                    Err(PinMiss::Singular(SingularMiss::EdgesCoincide))
                        if closes && slit.is_none() =>
                    {
                        slit = Some(index);
                        None
                    }
                    Err(miss) => return Err(WalkFail::Miss { index, miss }),
                }
            }
        };
        ends.push(this);
        out.push((image, t0, t1, element));
    }
    if closes && let (Some(placed), Some(prev)) = (out.last(), ends.last()) {
        // The closure joint: the first image's entry carried onto the
        // last image's exit, decided exactly as every other joint.
        let closure = match kept(0) {
            Some(element) => Some(element),
            None => match joint((&placed.0, prev, len - 1), &out[0].0, &ends[0], 0, None) {
                Ok(element) => Some(element),
                Err(PinMiss::Singular(SingularMiss::EdgesCoincide)) if slit.is_none() => {
                    slit = Some(0);
                    None
                }
                Err(miss) => return Err(WalkFail::Miss { index: 0, miss }),
            },
        };
        out[0].3 = closure;
        if let Some(index) = slit {
            // The other joints, in cycle order from the one after it.
            let rest = Winding::of(
                (1..len)
                    .map(|k| (index + k) % len)
                    .filter_map(|other| out[other].3),
            );
            let before = (index + len - 1) % len;
            let element = joint(
                (&out[before].0, &ends[before], before),
                &out[index].0,
                &ends[index],
                index,
                Some(rest),
            )
            .map_err(|miss| WalkFail::Miss { index, miss })?;
            out[index].3 = Some(element);
        }
        if !Winding::of(out.iter().filter_map(|pinned| pinned.3)).closes() {
            return Err(WalkFail::NotClosed);
        }
    }
    Ok(out)
}

/// Where a joint's vertex sits against the chart's **singular set**:
/// the points where the first channel stops naming a point, so the
/// images either side of a joint there meet at no azimuth and the
/// element is the turn on the face's side ([`singular_turn`]). Read by
/// [`singular_at`].
#[derive(Clone, Copy, Debug)]
pub(super) enum Singular {
    /// Definitely off the set, or the chart has none.
    Off,
    /// On this singular point of an analytic chart.
    On(geom_brep::chart_singularity::Singularity),
    /// A spline chart whose whole `u` knot domain moves a point under
    /// the band: every `u` lift names the same point.
    Collapsed,
    /// Undecided at this width.
    Undecided(Indeterminate),
}

/// **Whether a joint's vertex lies on the chart's singular set**,
/// decided as 3-D incidence of the vertex on each singular point — a
/// sphere's poles `c ± r·axis`, a cone's apex — by the one reading of
/// that incidence ([`geom_brep::chart_singularity::vertex_at_singular_point`],
/// which places a singular point at a face's vertex at rest too), so the
/// distance decided is polynomial in the vertex and the chart's data,
/// never a chart coordinate read back through a lever. A cylinder, a
/// plane and a ring torus have no singular set, and decide nothing.
///
/// A spline chart keeps the net-level form: the metres its whole `u`
/// knot domain moves a point at most ([`net_reach`]: the domain's length
/// times `sup |S_u|`, [`chart_u_arm`]), under the band, reads as
/// collapsed everywhere. That is a property of the net, not of the
/// vertex, and the chart-space gap that spline joints still state
/// ([`spline_gap_closes`]) is what keeps its joints sound.
pub(super) fn singular_at<T: Decide>(
    chart: DescribedChart<'_, T>,
    vertex: geom_core::Point3<T>,
    band: Band,
) -> Singular {
    match *chart.surface() {
        // The singular points, from their one home, each read by the one
        // incidence reading: on the set where on one of them, off where
        // off every one, and otherwise the first escalation.
        Surface::Sphere { .. } | Surface::Cone { .. } | Surface::Spindle { .. } => {
            use geom_brep::chart_singularity::{singular_points, vertex_at_singular_point};
            let Ok(inverse) = geom::ChartInverse::of(chart.surface()) else {
                return Singular::Off;
            };
            let mut undecided = None;
            for (which, point) in singular_points(&inverse) {
                match vertex_at_singular_point(vertex, point, band) {
                    Ok(true) => return Singular::On(which),
                    Ok(false) => {}
                    Err(cause) => {
                        undecided.get_or_insert(cause);
                    }
                }
            }
            undecided.map_or(Singular::Off, Singular::Undecided)
        }
        Surface::Cylinder { .. } | Surface::Torus { .. } | Surface::Plane { .. } => Singular::Off,
        Surface::Nurbs(_) | Surface::Approx(_) => {
            let Some(payload) = chart.surface().spline_chart() else {
                unreachable!("a spline kind answers its spline chart")
            };
            match decide(
                "pcurve_loop_pole_joint",
                net_reach(chart, payload, NetChannel::U),
                band,
            ) {
                Ok(Sign::Zero) => Singular::Collapsed,
                Ok(Sign::Positive | Sign::Negative) => Singular::Off,
                Err(cause) => Singular::Undecided(cause),
            }
        }
    }
}

/// Whether `surface`'s chart has a singular point at all, asked of the
/// chart's own list ([`geom_brep::chart_singularity::singular_points`])
/// rather than of a kind roster: a tip off every point of a non-empty
/// set is off it, and one on a chart with none has no set to be on.
pub(crate) fn has_singular_set<T: geom_core::Real>(surface: &Surface<T>) -> bool {
    geom::ChartInverse::of(surface)
        .is_ok_and(|chart| !geom_brep::chart_singularity::singular_points(&chart).is_empty())
}

/// The first channel's lever **at a joint**, read off the joint's
/// vertex: on an azimuth chart, the vertex's distance from the chart's
/// axis ([`axis_distance`]), exact at the vertex and so a sup there; on
/// a plane or a spline chart, [`chart_u_arm`]'s rate.
pub(crate) fn joint_arm<T: Real>(
    chart: DescribedChart<'_, T>,
    vertex: geom_core::Point3<T>,
) -> SupSpeed<T> {
    axis_distance(chart, vertex).map_or_else(|| chart_u_arm(chart), SupSpeed::new)
}

/// A vertex's distance from an azimuth chart's axis — cylinder `r`,
/// sphere `r·|cos v|`, torus `R + r·cos v`, cone `|v·sin α|`, each at
/// the vertex, but computed from the vertex point rather than from a
/// stored latitude. `None` on a plane or a spline chart, which has no
/// axis.
fn axis_distance<T: Real>(chart: DescribedChart<'_, T>, vertex: geom_core::Point3<T>) -> Option<T> {
    let off_axis = |origin: geom_core::Point3<T>, axis: geom_core::Vec3<T>| {
        geom::Line { origin, dir: axis }.distance(vertex)
    };
    match *chart.surface() {
        Surface::Cylinder { radius, .. } => Some(radius),
        Surface::Sphere { center, axis, .. }
        | Surface::Torus { center, axis, .. }
        | Surface::Spindle { center, axis, .. } => Some(off_axis(center, axis)),
        Surface::Cone { apex, axis, .. } => Some(off_axis(apex, axis)),
        Surface::Plane { .. } | Surface::Nurbs(_) | Surface::Approx(_) => None,
    }
}

/// How [`decide_joint`] reads a joint's first channel: the chart × the
/// vertex's [`Singular`] reading, as one value.
#[derive(Clone, Copy, Debug)]
enum AzimuthRule {
    /// No azimuth integer: the chart has no period, or it is a spline
    /// chart whose `u` knot domain moves a point under the band
    /// ([`Singular::Collapsed`]), where every `u` lift names the same
    /// point and the joint's chart-space gap decides
    /// ([`spline_gap_closes`]).
    Aperiodic,
    /// The vertex is decided ON this singular point of an analytic
    /// chart: the element is the turn on the face's side
    /// ([`singular_turn`]).
    OnSingularSet(geom_brep::chart_singularity::Singularity),
    /// A spline chart whose net-level `u` gate is UNDECIDED: its `u`
    /// periods are not decided, and the joint refuses
    /// ([`PinMiss::NetStretchUndecided`]), as its `v` gate's does.
    SplineUndecided(Indeterminate),
    /// The integer by the marks, and where they miss, an orbit point
    /// whose gap is decided within ε at the vertex's lever
    /// ([`lever_periods`]): a vertex decided off the singular set can
    /// still have a band-sized lever (near a narrow cone's apex, where
    /// the incidence reads `|p − apex|` and the lever is that times
    /// `sin α`). Where neither closes, the marks' own miss, so a gap on
    /// a mark is a decided discontinuity, except that the read's
    /// escalation answers where one of its offsets is in band.
    Marks,
    /// The same read as [`AzimuthRule::Marks`], with one difference: a
    /// miss is the incidence's escalation, never the marks' own. The
    /// vertex may be ON the set, where the whole latitude is one point,
    /// so no miss there is a decided discontinuity or a decided reach. The
    /// two differ in that error alone, and its readers read it: a
    /// discontinuity refuses as decided ([`turn_element`]'s `Err(None)`,
    /// [`PcurveMintError::LoopDiscontinuity`]), an escalation as
    /// undecided.
    ///
    /// [`PcurveMintError::LoopDiscontinuity`]: super::PcurveMintError::LoopDiscontinuity
    /// [`turn_element`]: super::turn_element
    MarksNearPole(Indeterminate),
}

impl AzimuthRule {
    fn of(periodic: bool, singular: Singular, spline: bool) -> Self {
        match (periodic, singular) {
            (false, _) | (true, Singular::Collapsed) => Self::Aperiodic,
            (true, Singular::On(which)) => Self::OnSingularSet(which),
            (true, Singular::Undecided(cause)) if spline => Self::SplineUndecided(cause),
            (true, Singular::Off) => Self::Marks,
            (true, Singular::Undecided(cause)) => Self::MarksNearPole(cause),
        }
    }
}

/// **The turn at a joint on the singular set, on the face's side**: the
/// element's first-channel periods and its twin bit, which together put
/// the leaving image's entry the face's turn past the arriving image's
/// exit.
///
/// At a singular point the whole latitude is the one point, so the two
/// images' ends there are apart in the chart by the angle between the
/// edges, and the element is read off the two EDGES: the
/// angle about the chart's axis from the arriving edge's tangent at the
/// point to the leaving edge's, swept on the face's side.
///
/// **The directions are the carriers'.** Each edge's direction is read
/// off its certified curve where it leaves the vertex. An image's first
/// coordinate at the point is that direction's azimuth or half a turn
/// off it ([`half_turn_off`]): half a turn off
/// on a sheet whose points lie against their azimuth (a cone's negative
/// nappe), and on a sphere where the image runs on the chart's twin
/// sheet, past the pole. Two edges that differ in it are joined through
/// the twin where the chart has one, so the leaving image is read on
/// the sheet the arriving one is on; where it has none they lie on
/// different nappes ([`SingularMiss::TwoNappes`]).
///
/// **The face's side.** Walking a loop with its face on the left of the
/// chart normal turned by `sense`, the azimuth ADVANCES along a singular
/// latitude the face lies above in `v` (a sphere's south pole, a cone's
/// apex under its positive nappe, a sheet's lower apex), so the entry is
/// the one branch in `(exit, exit + τ)`, and RETURNS along one it lies
/// below (`(exit − τ, exit)`); the opposite sense swaps the two.
///
/// The interval is open, and its ends are where the two edges leave the
/// point along one direction. That is structure, never a number:
/// - the walk leaves along the edge it arrived on (`mates`), so the
///   face wraps the point: one whole turn, a literal at every scalar;
/// - two distinct edges: [`geom_brep::Period::decide`] of `gap ± τ/2`,
///   its marks at whole periods, levered at the faces' extent
///   ([`JointSides::lever`]); one in band escalates;
/// - **a slit**: a mark decided Zero is two distinct edges leaving along
///   one direction, a turn of none or of a whole one, which no value at
///   the joint tells apart. The rest of the loop does
///   ([`JointSides::rest`]): where its other joints wind the chart once
///   against the face's own whole turn, the face wraps the point, the
///   two edges are a slit in it, and the turn is the whole one, so the
///   chain closes on its start as the face's boundary through the
///   point. So a slit's element depends on its loop's winding, and on
///   nothing else beyond its two edges and the face's sense. Anywhere
///   else the sector has no decided width
///   ([`SingularMiss::EdgesCoincide`]).
///
/// A cone's apex is on the face's side of one nappe only: which is read
/// off each edge ([`apex_nappe`]).
pub(super) fn singular_turn<T: Decide>(
    chart: DescribedChart<'_, T>,
    which: geom_brep::chart_singularity::Singularity,
    image: &Pcurve<T>,
    entry_t: T,
    prev: geom_core::Point2<T>,
    sides: JointSides<'_, T>,
    band: Band,
) -> Result<(i32, bool), PinMiss> {
    use geom_brep::chart_singularity::Singularity;
    let above = match which {
        Singularity::South | Singularity::SheetApexDown => true,
        Singularity::North | Singularity::SheetApexUp => false,
        Singularity::Apex => {
            let arrived = apex_nappe(chart, sides.arrived, band)?;
            if arrived != apex_nappe(chart, sides.leaving, band)? {
                return Err(PinMiss::Singular(SingularMiss::TwoNappes));
            }
            arrived
        }
    };
    let step = if above == sides.sense { 1 } else { -1 };
    if sides.mates {
        return Ok((step, false));
    }
    let Some(lever) = sides.lever() else {
        return Err(PinMiss::Singular(SingularMiss::NoLever));
    };
    let entry = image.eval(entry_t);
    let twin = half_turn_off(chart, sides.arrived, prev.x, lever, band)?
        != half_turn_off(chart, sides.leaving, entry.x, lever, band)?;
    let entry = if twin {
        let Some(sheet) = sphere_twin(chart.surface(), image) else {
            return Err(PinMiss::Singular(SingularMiss::TwoNappes));
        };
        sheet.eval(entry_t)
    } else {
        entry
    };
    let gap = prev.x - entry.x;
    let half = T::tau() * T::from_f64(0.5);
    let shifted = if step > 0 { gap + half } else { gap - half };
    match geom_brep::Period::angle().decide(
        "pcurve_singular_turn",
        shifted,
        |g| Margin::levered(g, lever),
        band,
    ) {
        Ok(k) => Ok((k, twin)),
        Err(stop) => match stop.miss {
            BranchMiss::Undecided(cause) => Err(PinMiss::Escalated(cause)),
            BranchMiss::OnMark => {
                let wraps = Deck {
                    u: -step,
                    v: 0,
                    twin: false,
                };
                match sides.rest {
                    Some(rest) if !twin && rest.deck == wraps => Ok((step, false)),
                    _ => Err(PinMiss::Singular(SingularMiss::EdgesCoincide)),
                }
            }
        },
    }
}

/// **Whether an image's first coordinate at a singular point is half a
/// turn off its edge's direction there**: `u`, the image's azimuth at
/// the point, against the direction `edge`'s carrier leaves the point
/// along, read about the chart's axis (`pcurve_singular_sheet`). The two
/// agree or oppose, so the reading is the side of a half turn, levered
/// as the turn is.
///
/// The direction is the carrier's chord from the point to an eighth of
/// the edge's span along. On a line it is the tangent; on a circle it
/// is within a sixteenth of the turn the edge sweeps of it, far inside
/// the quarter turn either side that the reading has; and it stands at
/// a cusp, where the tangent itself lies along the axis and has no
/// direction about it (a spindle's horn).
fn half_turn_off<T: Decide>(
    chart: DescribedChart<'_, T>,
    edge: EdgeEnd<'_, T>,
    u: T,
    lever: geom_core::Lever<T>,
    band: Band,
) -> Result<bool, PinMiss> {
    let Ok(inverse) = geom::ChartInverse::of(chart.surface()) else {
        unreachable!("a chart with a singular set has an azimuth frame")
    };
    let frame = inverse.frame();
    // The chord from the point to the carrier an eighth of the span
    // along, less its part along the axis.
    let near = edge.at + (edge.far - edge.at) * T::from_f64(0.125);
    let chord = edge.carrier.eval(near) - edge.carrier.eval(edge.at);
    let across = chord - frame.axis * chord.dot(frame.axis);
    let (sin, cos) = u.sin_cos();
    let along = frame.u_ref * cos + frame.v_ref() * sin;
    match decide(
        "pcurve_singular_sheet",
        Margin::levered(across.dot(along) / across.norm(), lever),
        band,
    ) {
        Ok(Sign::Positive) => Ok(false),
        Ok(Sign::Negative) => Ok(true),
        Ok(Sign::Zero) => Err(PinMiss::Singular(SingularMiss::NoLever)),
        Err(cause) => Err(PinMiss::Escalated(cause)),
    }
}

/// A chart's axis, as a line: the axis its azimuth turns about. `None`
/// on a chart with no azimuth.
fn chart_axis<T: Real>(chart: DescribedChart<'_, T>) -> Option<geom::Line<T>> {
    geom::ChartInverse::of(chart.surface())
        .ok()
        .map(|inverse| inverse.frame().axis_line())
}

/// **Which nappe of a cone an edge leaves its apex on**: `true` for
/// the positive one (`v > 0`, along the axis), read as the side of the
/// apex's normal plane the edge lies on (`pcurve_singular_nappe`), at
/// its far end or the middle of its span, whichever stands farther off
/// that plane. A point of the cone at distance `ρ` from the axis stands
/// `ρ / tan α` off it, so the reading is zero only for an edge that
/// does not leave the apex.
fn apex_nappe<T: Decide>(
    chart: DescribedChart<'_, T>,
    edge: EdgeEnd<'_, T>,
    band: Band,
) -> Result<bool, PinMiss> {
    let Some(axis) = chart_axis(chart) else {
        unreachable!("a cone's chart has an axis")
    };
    let far = axis.height(edge.carrier.eval(edge.far));
    let mid = axis.height(edge.carrier.mid_point(edge.at, edge.far));
    let height = (mid.abs() - far.abs()).select_le_zero(far, mid);
    match decide("pcurve_singular_nappe", Margin::of(height), band) {
        Ok(Sign::Positive) => Ok(true),
        Ok(Sign::Negative) => Ok(false),
        Ok(Sign::Zero) => Err(PinMiss::Singular(SingularMiss::NoNappe)),
        Err(cause) => Err(PinMiss::Escalated(cause)),
    }
}

/// **The ONE decision of one joint**: the element ([`JointElement`])
/// that carries `image`'s entry (at `entry_t`) onto `prev`, the exit of
/// the image before it, at the joint's vertex `vertex`. Two chart
/// points with one 3-D image differ by a deck element of the chart: a
/// whole number of periods per periodic channel and, on a sphere,
/// possibly the involution `(u, v) ↦ (u + π, π − v)` ([`sphere_twin`]).
/// This decides which, as integers ([`geom_brep::Period::decide`]),
/// the azimuth's marks metered at the vertex's own lever
/// ([`joint_arm`]).
///
/// Both points are images' — a function of their edges and the chart —
/// and the vertex is the joint's, so the element is too, and a joint
/// reads the same from every walk of its loop.
///
/// **The margin of each decision.** On a cylinder, cone or torus the
/// orbit of a chart point steps a whole period in azimuth, so the
/// integer has half a period of room either side. On a sphere the orbit
/// steps half a period: the image and its twin sit at `kτ` and
/// `π + kτ` in azimuth. So the azimuth is decided once, as a whole
/// number `m` of HALF periods (marks at `±π/2`), and `m` names both the
/// sheet (`m` odd: the twin) and the period (`⌊m/2⌋`). Each orbit point
/// then has a quarter period of room, `(π/2)·d` in metres at the
/// vertex's distance `d` from the axis: half the separation of the two
/// nearest orbit points in the chart metric.
///
/// The integer counts the orbit's steps, so on a sphere it counts half
/// periods. The marks are read outward from zero with no cap, however
/// many steps out the gap lies ([`geom_brep::Period::decide`]).
///
/// **What that room buys, and where it is not enough.** Where it exceeds
/// the joint bound below (the chart ends `≤ 4ε` apart in metres), the
/// decided integer is the joint's true deck element. It does not exceed
/// it everywhere [`singular_at`] reads `Off`: `Off` only puts the vertex
/// `K·ε` from a pole, so near a pole at a small `K` the lever `d` is
/// about `K·ε`, and near a narrow cone's apex it is `p·sin α` at slant
/// `p`, small even at `K = 10`. There a mark may decide an orbit point
/// other than the true one, but only one equally near the joint: every
/// orbit point is a lift of the SAME 3-D point, and two that are both
/// within the joint bound are within it of each other, so the lift
/// stays continuous in metres to that bound. Where a mark misses there
/// (the gap on it, or in its band), the orbit point the gap meets within
/// ε at the vertex's lever is taken ([`lever_periods`]): at a sub-band
/// lever every mark reads Zero, and a continuous joint whose gap is
/// within ε of an offset the read tries decides a lift of its own
/// point. Where an offset the read tries is in band, the chart points
/// it would join may be one point, and the joint escalates; a gap on a
/// mark refuses only where the read decides its own nearest orbit
/// points off ([`lever_periods`]).
/// No branch is ever decided that names a different point. What such a
/// pick can move is the loop's winding by one orbit step, so a reader of
/// the winding as a region checks each joint's room first:
/// [`loop_windings`] refuses a joint whose room is not past the joint
/// bound ([`joint_has_room`]).
///
/// **The joint's 3-D coincidence is not decided again here**, and needs
/// no chart margin: it follows from two certified bounds. Each row's
/// envelope bounds `|S(P(t)) − C(t)| ≤ ε` over its whole span (check 4,
/// `geom_brep::PcurveCache`), and the edge certificate pins each
/// carrier's ends to its vertices within ε (`carrier_endpoint_start` /
/// `_end`), so the two chart ends of a joint map within `4ε` of each
/// other. On the lift decided here (the image or its twin, `|Δu| ≤ π`),
/// `sin(x/2) ≥ x/π` turns that into `r·|Δu| ≤ 2π·ε` and `|Δv| ≤ 4ε` on
/// a cylinder of radius `r`; the sphere, cone and torus give the same
/// shape with the vertex's own lever away from their singular set. That
/// is a bound the joint's room is read against ([`joint_has_room`]),
/// not a decision.
///
/// **The first channel's rule** is [`AzimuthRule`]'s. On the singular
/// set the whole latitude is one point, so the two images' ends are
/// apart in the chart by the angle between their edges: the element is
/// the turn on the face's side ([`singular_turn`]), read off `sides`,
/// its twin bit set where the two images run on different sheets of a
/// sphere's chart. Two distinct edges leaving the point along one
/// direction are the one case that reads past the joint: their element
/// depends on the winding of the rest of their loop
/// ([`JointSides::rest`]). Where the incidence is UNDECIDED on an analytic chart the
/// integer is decided as at any other joint, which is sound either way
/// (a deck transformation moves no point), with the same near-pole read
/// where its marks miss ([`lever_periods`]), and any miss left
/// escalates with the incidence's cause. A spline chart has no
/// injectivity lemma (a net can fold, so a 3-D coincidence does not
/// name the sheet), and each of its joints states its chart-space gap
/// as well ([`spline_gap_closes`]); a net whose `u` gate is undecided
/// refuses, as one whose `v` gate is.
///
/// **Which writers decide, and which compose or copy.** The writers that
/// decide a joint call this and nothing else: the minting walk (the
/// closure joint read as every other), the site mint's new and missing
/// joints ([`site_rows`]), a split's joint between its children
/// ([`split_cache`]), a kill's turn across a closed carrier
/// ([`turn_element`]), and tier 3 ([`validate_pcurves`]). The rest write
/// elements by algebra or copy, as R's design has them: a kill's bridged
/// joint is the composition of the elements either side
/// ([`JointElement::then`]), a reversed loop's is the inverse
/// ([`JointElement::inverse`], `Body::revert`), a null edge's is the
/// identity (a point), and the site mint's kept elements, a split's
/// carried rows and the boolean graft copy elements between images that
/// stand unchanged on a face whose sense stands. Each is the element
/// this decision gives the same geometry, where its integer has room:
/// the composed path and the bridged joint carry the same image onto
/// the same point, and on the singular set a killed strut splits the
/// face's sector into the two turns summed. A door that moves a loop to
/// a face of the opposite sense on a chart with a singular set treats
/// the loop's rows as rows about another chart, and its band door
/// decides them again (`Body::sides_agree`). And tier 3
/// re-decides every stored element here, so a written element this
/// decision does not give reads as a [`PcurveMintError::LoopDiscontinuity`]
/// at rest — loud, never silent. Where the room is short (a pole at a
/// small `K`, a narrow cone's apex), a composition may name another lift
/// of the same point than a fresh decision does; that too is loud at
/// rest (`work/topo/a-kill-bridging-joints-that-straddle-the-pole-lever-band-writes-a-reset-the-pass-decides-a-shift`).
///
/// **Every element records a decision.** A joint that decides none
/// refuses: [`PinMiss::Singular`] on the singular set, and
/// [`PinMiss::NetStretchUndecided`] on a net whose gate in either
/// channel is undecided.
///
/// [`PcurveMintError::LoopDiscontinuity`]: super::PcurveMintError::LoopDiscontinuity
/// [`loop_windings`]: super::loop_windings
/// [`site_rows`]: super::site_rows
/// [`split_cache`]: super::split_cache
/// [`turn_element`]: super::turn_element
/// [`validate_pcurves`]: super::validate_pcurves
#[allow(clippy::too_many_arguments)]
pub(super) fn decide_joint<T: Decide>(
    chart: DescribedChart<'_, T>,
    image: &Pcurve<T>,
    entry_t: T,
    prev: geom_core::Point2<T>,
    vertex: geom_core::Point3<T>,
    periods: ChartPeriods<T>,
    band: Band,
    sides: JointSides<'_, T>,
) -> Result<JointElement, PinMiss> {
    let surface = chart.surface();
    let u_period = periods.u();
    let spline = surface.spline_chart().is_some();
    let arm = joint_arm(chart, vertex);
    let twin = sphere_twin(surface, image);
    let orbit = Orbit::of(surface, image);
    let gap = prev.x - image.eval(entry_t).x;
    let marks = |p: Period<T>| lever_periods("pcurve_loop_branch", arm, gap, p, orbit, band);
    let rule = AzimuthRule::of(u_period.is_some(), singular_at(chart, vertex, band), spline);
    let m = match (rule, u_period) {
        (AzimuthRule::Marks, Some(p)) => marks(p).map_err(PinMiss::from)?,
        (AzimuthRule::MarksNearPole(cause), Some(p)) => {
            marks(p).map_err(|_| PinMiss::Escalated(cause))?
        }
        // The turn's periods, and its twin bit as an odd step.
        (AzimuthRule::OnSingularSet(which), _) => {
            let (periods, twin) = singular_turn(chart, which, image, entry_t, prev, sides, band)?;
            periods * orbit.steps() + i32::from(twin)
        }
        (AzimuthRule::SplineUndecided(cause), _) => {
            return Err(PinMiss::NetStretchUndecided(cause));
        }
        _ => 0,
    };
    // An odd step of a two-step orbit lands on the twin; the whole
    // periods are the steps over the orbit's count.
    let (is_twin, ku) = (
        m.rem_euclid(orbit.steps()) == 1,
        m.div_euclid(orbit.steps()),
    );
    let sheet = match twin {
        Some(t) if is_twin => t,
        _ => image.clone(),
    };
    // The second channel's periods, on the chosen sheet, in the period
    // the chart owns for it (C4): `τ` metered by the polar radius where
    // `v` is an angle, the knot length metred by the net's `sup |S_v|`
    // ([`v_meter`]) on a net closed in `v`. A shift of the first channel
    // moves no second-channel value.
    let gap_v = prev.y - sheet.eval(entry_t).y;
    let kv = match periods.v {
        // A mark decides the sign of `gap − mark`, which any positive
        // rate keeps, so the rate's direction only sizes the band about
        // the mark: the exact polar radius serves as both members, and
        // the net's sup as the escape side.
        Some(VPeriod::Angle { period, polar }) => period
            .decide(
                "pcurve_loop_branch",
                gap_v,
                |g| Margin::metered_sup(g, polar),
                band,
            )
            .map_err(|stop| PinMiss::from(stop.miss))?,
        // A net's `v` periods are read by the marks only where its `v`
        // knot domain moves a point past the band ([`net_reach`]), as its
        // `u` periods are ([`singular_at`]). Decided under it, no `v`
        // displacement on the chart reaches ε, every lift names the same
        // point, and the joint is an ordinary one with `kv = 0`, its
        // chart-space gap deciding ([`spline_gap_closes`]). Undecided, the
        // joint refuses ([`PinMiss::NetStretchUndecided`]): a degenerate
        // net, one whose whole `v` sweep is within the band's sliver.
        Some(VPeriod::Net(period)) => {
            let Some(payload) = surface.spline_chart() else {
                unreachable!("a net's period is a spline chart's")
            };
            match decide(
                "pcurve_loop_v_stretch",
                net_reach(chart, payload, NetChannel::V),
                band,
            ) {
                Ok(Sign::Positive) => period
                    .decide(
                        "pcurve_loop_branch",
                        gap_v,
                        |g| Margin::metered_sup(g, v_meter(chart)),
                        band,
                    )
                    .map_err(|stop| PinMiss::from(stop.miss))?,
                Ok(Sign::Zero | Sign::Negative) => 0,
                Err(cause) => return Err(PinMiss::NetStretchUndecided(cause)),
            }
        }
        None => 0,
    };
    // A spline chart has no angular channel, so its lift is the sheet
    // moved by the periods of each channel it closes in, and the
    // joint's chart-space gap is read on that lift.
    if spline {
        let mut lifted = sheet.clone();
        if let Some(p) = u_period {
            lifted = lifted.shift_branch(ku, p);
        }
        if let Some(p) = periods.v() {
            lifted = shift_v_branch(&lifted, kv, p);
        }
        spline_gap_closes(chart, lifted.eval(entry_t), prev, band)?;
    }
    Ok(JointElement(Deck {
        u: ku,
        v: kv,
        twin: is_twin,
    }))
}

/// [`decide_joint`] for a row that states a joint by its two images
/// alone, at sense `true`: the two edges are two lines through the
/// vertex, each leaving it along its image's azimuth there (the
/// principal sheet), or along `x` on a chart with no azimuth frame.
#[cfg(test)]
pub(super) fn decide_plain_joint<T: Decide>(
    chart: DescribedChart<'_, T>,
    image: &Pcurve<T>,
    entry_t: T,
    prev: geom_core::Point2<T>,
    vertex: geom_core::Point3<T>,
    periods: ChartPeriods<T>,
    band: Band,
) -> Result<JointElement, PinMiss> {
    let along = |u: T| match geom::ChartInverse::of(chart.surface()) {
        Ok(inverse) => {
            let frame = inverse.frame();
            let (sin, cos) = u.sin_cos();
            frame.u_ref * cos + frame.v_ref() * sin
        }
        Err(_) => geom_core::Vec3::unit_x(),
    };
    let arrived = geom::Curve3::line(vertex, along(prev.x));
    let leaving = geom::Curve3::line(vertex, along(image.eval(entry_t).x));
    let reach = || Some(geom_core::Lever::curvature_radius(T::one()));
    decide_joint(
        chart,
        image,
        entry_t,
        prev,
        vertex,
        periods,
        band,
        JointSides::of_halves(
            true,
            JointHalf {
                carrier: &arrived,
                entry: T::one(),
                exit: T::zero(),
                plus: true,
                edge: WalkEdge::New,
                reach: &reach,
            },
            JointHalf {
                carrier: &leaving,
                entry: T::zero(),
                exit: T::one(),
                plus: true,
                edge: WalkEdge::New,
                reach: &reach,
            },
            None,
        ),
    )
}

/// A half-edge's `(entry, exit)` carrier parameters in its loop's
/// direction, from its row's forward interval `(t0, t1)`: its own order
/// on a plus half, reversed on a minus one.
pub(super) fn entry_exit<T: Copy>(plus: bool, t0: T, t1: T) -> (T, T) {
    if plus { (t0, t1) } else { (t1, t0) }
}

impl From<BranchMiss> for PinMiss {
    fn from(miss: BranchMiss) -> Self {
        match miss {
            BranchMiss::Undecided(cause) => Self::Escalated(cause),
            BranchMiss::OnMark => Self::Discontinuity,
        }
    }
}

/// **A joint's orbit**: the step its marks count in, and the offsets
/// about the image the near-pole read tries first
/// ([`near_pole_gap_closes`]), in that step; the read then tries the
/// two branches astride the mark the walk stopped on. Both are read off
/// one fact, the steps in a period ([`Orbit::steps`]), so an offset is
/// never read in the other orbit's step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Orbit {
    /// No twin: whole periods. The image itself, then the image one
    /// period either way round.
    Periods,
    /// A sphere twin half a period over: half periods. The image, the
    /// twin either way round, then the image one period either way
    /// round.
    HalfPeriodsWithTwin,
}

impl Orbit {
    /// The orbit of a joint into `image` on `surface`: half periods
    /// where the image has a sphere twin ([`sphere_twin`]), whole periods
    /// elsewhere. The one fact the walk ([`decide_joint`]) and the room
    /// fence ([`joint_has_room`]) both read.
    pub(crate) fn of<T: Real>(surface: &Surface<T>, image: &Pcurve<T>) -> Self {
        if sphere_twin(surface, image).is_some() {
            Self::HalfPeriodsWithTwin
        } else {
            Self::Periods
        }
    }

    /// The steps in one period: 1, or 2 where the twin sits half a
    /// period over.
    pub(crate) fn steps(self) -> i32 {
        match self {
            Self::Periods => 1,
            Self::HalfPeriodsWithTwin => 2,
        }
    }

    /// The offsets tried, first to last, in [`Orbit::step`]s: the image,
    /// then each step out to one period, either way round.
    pub(crate) fn offsets(self) -> impl Iterator<Item = i32> + Clone {
        core::iter::once(0).chain((1..=self.steps()).flat_map(|m| [m, -m]))
    }

    /// The step of `period` this orbit counts in, as the period its
    /// marks are read in.
    pub(crate) fn step<T: Real>(self, period: Period<T>) -> Period<T> {
        match self.steps() {
            1 => period,
            n => Period::new(period.length() * T::from_f64(1.0 / f64::from(n))),
        }
    }
}

/// What the near-pole read decided of a joint's orbit
/// ([`near_pole_gap_closes`]).
#[derive(Clone, Debug)]
pub(super) enum NearRead {
    /// The gap meets the offset `m` within ε at the vertex's lever.
    Closes(i32),
    /// Every offset tried is decided farther than ε off.
    Off,
    /// No offset closes, and one is in band (its cause): the chart
    /// points it would join may be within ε in metres, so whether the
    /// joint is continuous is undecided.
    InBand(Indeterminate),
}

/// The branch of a joint whose orbit's marks missed at a band-sized
/// lever `arm` ([`lever_periods`]): the vertex sits within the band of
/// a pole or an apex (its incidence undecided), or off a narrow cone's
/// apex by more than the band while its lever `|p − apex|·sin α` is not,
/// so no mark reads definitely. The offsets tried are `orbit`'s
/// ([`Orbit::offsets`]), then the branches `astride` the mark the
/// marks' walk stopped on, in `orbit`'s steps of `period`; the first
/// whose gap, metered at that lever, is decided within ε
/// (`pcurve_loop_pole_gap`) is taken ([`first_within`]).
/// The two chart points then agree in metres at the vertex, and no
/// branch is asserted that a margin did not see. Every offset tried is
/// a lift of the one point: a whole period is a deck transformation of
/// the chart, and a half step lands on the twin, which names the point
/// once the caller's second-channel count on it decides. Where none
/// closes, [`NearRead::InBand`] with the first in-band offset's cause,
/// else [`NearRead::Off`].
///
/// A vertex ON the axis but off the surface (within the band of a pole)
/// has lever 0, so any gap decides Zero there and the image itself is
/// taken. That is sound in 3-D: every chart end of the joint then lies
/// within a few ε of the pole, where no azimuth names a point; and
/// [`loop_windings`] reports such a joint's loop singular and reads no
/// winding of it.
///
/// [`loop_windings`]: super::loop_windings
pub(super) fn near_pole_gap_closes<T: Decide>(
    arm: SupSpeed<T>,
    gap: T,
    period: Period<T>,
    orbit: Orbit,
    astride: Option<[i32; 2]>,
    band: Band,
) -> NearRead {
    let step = orbit.step(period);
    let bracket = astride
        .into_iter()
        .flatten()
        .filter(|m| !orbit.offsets().any(|o| o == *m));
    let tried = orbit.offsets().chain(bracket);
    match first_within("pcurve_loop_pole_gap", tried, band, |&m| {
        Margin::metered_sup(gap - step.times(m), arm)
    }) {
        Ok(Some(m)) => NearRead::Closes(m),
        Ok(None) => NearRead::Off,
        Err(cause) => NearRead::InBand(cause),
    }
}

/// **A joint's whole orbit steps by the marks, with the near-pole
/// read**: [`geom_brep::Period::decide`] of `gap` in `orbit`'s
/// steps of `period`, metered at the vertex's lever `arm`. Where a mark
/// misses (undecided, or the gap on it), an orbit point whose gap is
/// decided within ε at that lever is taken ([`near_pole_gap_closes`],
/// trying `orbit`'s offsets in order). A band-sized lever reads every
/// mark as Zero: a vertex near a pole, and a vertex near a narrow
/// cone's apex, whose incidence decides it OFF the apex ([`singular_at`]
/// reads `|p − apex|`) while its lever `|p − apex|·sin α` is under the
/// band. The two chart points then agree in metres at the vertex. A
/// whole-period offset is a lift of the joint's own point; an odd
/// offset of [`Orbit::HalfPeriodsWithTwin`] is a half step onto the
/// twin, which names the joint's point only once the caller's
/// second-channel count on the twin decides ([`decide_joint`]).
///
/// **Where no offset closes, the read's own verdict stands over the
/// marks'.** A mark decides Zero at a band-sized lever because every
/// azimuth there is within the band in metres, so the marks' `OnMark`
/// says only that the gap is half a step off an orbit point, not that
/// the two chart points are apart. Where an offset is in band, they may
/// be one point, and the answer is an escalation.
///
/// **A decided miss stands only where no orbit point is within the
/// band.** Besides the orbit's offsets about the image, the read tries
/// the two branches astride the mark the walk stopped on
/// ([`geom_brep::WalkStop`]). Where the marks miss as `OnMark`, that
/// mark decided Zero, so the gap is within the band's zero of it; where
/// the read then decides both branches astride it off, the gap lies
/// between them (past either, that branch would be within ε), so
/// `π·lever ≥ escalate`, and every other orbit point is at least
/// `3π·lever − ε` off, past the band too, at any `K > 1`.
///
/// At a lever with room nothing moves: the read is asked only where a
/// mark missed, so where the gap is within the band of a mark, and a
/// gap within ε of an orbit point is that near a mark only where half a
/// step at the lever is itself band-sized, whichever orbit point it is.
/// Likewise an offset in band needs the gap within the band of it.
///
/// # Errors
///
/// The marks' [`BranchMiss`] where the read does not close, except
/// that a decided `OnMark` is the read's [`BranchMiss::Undecided`]
/// where an offset is in band. An undecided mark keeps its own cause.
pub(crate) fn lever_periods<T: Decide>(
    name: &'static str,
    arm: SupSpeed<T>,
    gap: T,
    period: Period<T>,
    orbit: Orbit,
    band: Band,
) -> Result<i32, BranchMiss> {
    // A mark decides the sign of `gap − mark`, which any positive rate
    // keeps; the exact vertex lever serves as both members, the spline
    // chart's sup as the escape side.
    match orbit
        .step(period)
        .decide(name, gap, |g| Margin::metered_sup(g, arm), band)
    {
        Ok(k) => Ok(k),
        Err(WalkStop {
            miss: miss @ (BranchMiss::Undecided(_) | BranchMiss::OnMark),
            astride,
        }) => match near_pole_gap_closes(arm, gap, period, orbit, astride, band) {
            NearRead::Closes(m) => Ok(m),
            NearRead::InBand(cause) => match miss {
                BranchMiss::OnMark => Err(BranchMiss::Undecided(cause)),
                undecided => Err(undecided),
            },
            NearRead::Off => Err(miss),
        },
    }
}

/// **The two branches astride a half-period mark**: where `gap` sits on
/// or near the mark between `j` and `j + 1` periods, `[j, j + 1]`, with
/// `j` decided by the marks of `gap` less half a period, which then sits
/// near `j` periods, half a period from either of its own marks. A caller
/// whose marks missed asks its question on both and keeps an answer only
/// where the two agree.
///
/// # Errors
///
/// [`BranchMiss`] where `j`'s own marks miss.
pub(crate) fn branches_astride_as<T: Decide>(
    name: &'static str,
    gap: T,
    period: T,
    meter: impl Fn(T) -> Margin<T>,
    band: Band,
) -> Result<[i32; 2], BranchMiss> {
    let j = Period::new(period)
        .decide(name, gap - period * T::from_f64(0.5), meter, band)
        .map_err(|stop| stop.miss)?;
    Ok([j, j + 1])
}

/// **A spline chart's joint gap**: the entry `entry` and the
/// predecessor's exit `prev` agree within ε through the chart's sup
/// stretch on each channel ([`chart_u_arm`], [`v_meter`]). A net can
/// fold, so on a spline chart a 3-D coincidence does not name the
/// sheet, and the joint states its chart-space gap ([`decide_joint`]).
///
/// # Errors
///
/// [`PinMiss::Discontinuity`] for a definite gap,
/// [`PinMiss::Escalated`] for an undecided one.
pub(super) fn spline_gap_closes<T: Decide>(
    chart: DescribedChart<'_, T>,
    entry: geom_core::Point2<T>,
    prev: geom_core::Point2<T>,
    band: Band,
) -> Result<(), PinMiss> {
    let arm = chart_u_arm(chart);
    for margin in [
        Margin::metered_sup(entry.x - prev.x, arm),
        Margin::metered_sup(entry.y - prev.y, v_meter(chart)),
    ] {
        match decide("pcurve_loop_continuity", margin, band) {
            Ok(Sign::Zero) => {}
            Ok(Sign::Positive | Sign::Negative) => return Err(PinMiss::Discontinuity),
            Err(cause) => return Err(PinMiss::Escalated(cause)),
        }
    }
    Ok(())
}

/// **Whether a joint's azimuth integer was decided with room** past the
/// joint bound ([`loop_windings`]' room fence). The walk decides the
/// integer against marks half the step to the next orbit point away
/// ([`decide_joint`]): half a period on a cylinder, cone or torus, a
/// quarter on a sphere. A mark can name another orbit point than the
/// joint's own only where the joint's two chart ends lie at least that
/// half step apart in azimuth. Each end is within `2ε` of the vertex in
/// 3-D (its row's envelope and the edge certificate's pin), so its
/// distance from the axis is at least `d − 2ε` at the vertex's own
/// distance `d`, and two such points half a step apart are at least the
/// chord `2·(d − 2ε)·sin(step/4)` apart. Where that chord exceeds the
/// joint bound `4ε` no such pair exists, and the integer is the
/// joint's own. So the fence decides the chord in the same quantity
/// as the bound: a quarter of it, `(d − 2ε)·sin(step/4)/2`, must
/// decide definitely past the band's escalation (`loop_windings_joint_room`),
/// which puts the chord past `4·K·ε > 4ε` at every admitted `K > 1`.
/// Anything else refuses — the conservative side. On a sphere that
/// admits `d > 2ε + 2√2·K·ε`; on a cylinder, cone or torus,
/// `d > 2ε + 2·K·ε`.
///
/// A chart with no period decides no integer. A spline chart is exempt:
/// each of its joints already decided its lifted chart-space gap within
/// the band through the net's sup stretch ([`spline_gap_closes`]), which
/// pins the lift itself rather than the room of its integer.
///
/// [`loop_windings`]: super::loop_windings
pub(super) fn joint_has_room<T: Decide>(
    chart: DescribedChart<'_, T>,
    vertex: geom_core::Point3<T>,
    u_period: Option<Period<T>>,
    orbit: Orbit,
    band: Band,
) -> bool {
    let Some(period) = u_period else {
        return true;
    };
    // A periodic chart with no axis is a spline chart, exempt above.
    let Some(distance) = axis_distance(chart, vertex) else {
        return true;
    };
    // The step the walk decided this joint's integer in.
    let step = orbit.step(period).length();
    // The ends' least distance from the axis, and a quarter of the
    // chord between two of them half a step apart.
    let lever = distance - T::from_f64(2.0 * band.zero());
    let quarter_chord = (step * T::from_f64(0.25)).sin() * T::from_f64(0.5);
    matches!(
        decide(
            "loop_windings_joint_room",
            Margin::levered(
                quarter_chord,
                // Unsourced lever: the joint ends' least distance from the axis, `d − 2ε`, a
                // band-deflated radius of revolution; no extent read or curvature arm states it.
                geom_core::Lever::unsourced(lever),
            ),
            band,
        ),
        Ok(Sign::Positive)
    )
}

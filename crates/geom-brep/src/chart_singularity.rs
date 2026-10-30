//! **Where a chart singularity lies against a face**: a sphere's pole or
//! a cone's apex read against the face's region, at one of its
//! vertices, inside one of its edges, in its interior, or outside its
//! closure ([`locate`]).
//!
//! Every chart singularity in a face's closure is a vertex of it (DESIGN
//! `[topology.tiers]`, tier 3). Two doors read that statement through
//! [`locate`]: `step-import`'s normalization, which makes each such point
//! a vertex of the face it lies in, and tier 3, which refuses a face
//! where one is not. The reading is form-neutral: it never asks whether
//! a loop has a meridian, since a tilted cap holds a pole with none and
//! a meridian spur that stops short of the pole has one and still holds
//! it.
//!
//! **The singular points** are [`singular_points`]: a sphere's two poles
//! ([`sphere_poles`]) and a cone's apex, from the chart's own frame.
//!
//! **On the boundary.** A point is at a vertex of the face where its
//! distance to the vertex's stored point decides zero, and inside an
//! edge where its distance to the edge's nearest interior point does,
//! both at the caller's band and each decided once
//! ([`on_boundary`], `chart_singularity_on_boundary`); a distance the
//! band cannot call is undecided and carries its margin
//! ([`Unread::Boundary`]). Lines and circles are read in closed form
//! ([`foot`]); any other carrier by samples refined against its
//! certified speed bound, whose every span is cleared of the point past
//! the band or read to the sample nearest it ([`Unread::Carrier`] where
//! neither settles within the refinement's budget, or no speed bound is
//! stated).
//!
//! **Inside the face.** Off the boundary, the point is in the face where
//! it lies on the face's side of every loop. A loop's side is read from
//! a quantity whose error is bounded, and a reading that does not clear
//! its bound is undecided ([`SideUnread`]), never folded to a side:
//!
//! - **A sphere loop of circle arcs** is read in closed form, by the
//!   turning of its stereographic image from the pole
//!   (`crate::props`' pole reading, the one props reads a sphere face's
//!   side by): a whole turn, its sign the side.
//! - **Any other loop** is walked through the chart ([`walk_loop`]):
//!   its winding about the surface axis, and the signed metric area of
//!   its chart image with a certified bound on the walk's error, from
//!   each carrier's speed bound. A loop that winds separates the
//!   sphere's poles, the north on its left when it winds positively,
//!   and rings the cone's apex, which it holds on its left exactly when
//!   it winds positively on the negative nappe; one that does not wind
//!   leaves the singular points on the side its chart image does not
//!   enclose, which is its left exactly when that image runs clockwise
//!   (`chart_singularity_loop_side`, the area against its bound).
//!
//! The face's side of each loop is its left under the outward normal,
//! the chart normal turned by the face's sense.

use geom::{ChartArm, ChartInverse, Curve3, Surface};
use geom_core::k_stats::{NonzeroSign, decide, decide_nonzero};
use geom_core::predicate::Band;
use geom_core::{Decide, Indeterminate, Margin, Point2, Point3, Real, Sign, SizedPass, Vec3};

use crate::props::{LoopEdge, PoleLeft, sphere_pole_left};
use crate::recourse::{ReadAt, RefusedArm, RefusedSide, SizedDecision};

/// **Whether a chart singularity lies on a face's boundary**, a sized
/// decision: the distance from the point to a vertex or an edge of the
/// face. Zero (on it) and a clear distance (off it) both pass; only a
/// distance the band cannot call refuses, and a smaller tolerance calls
/// any nonzero one. The one ending of every refusal of it, at import
/// and at rest.
pub const ON_BOUNDARY: SizedDecision = SizedDecision {
    lever: "move the face's boundary, or its vertex, onto that point or clear of it",
    size: "distance",
    passes: SizedPass::AnySign,
    side: RefusedSide::Lever,
    no_size: None,
};

/// **Which side of a face's loop a chart singularity lies on**, a
/// decision whose reading is the loop's own geometry rather than a
/// distance a tolerance is held against: it refuses where the reading
/// does not clear its own error bound ([`SideUnread::Straddle`], no
/// tolerance decides it) or a band decision of it is in band
/// ([`SideUnread::Banded`]). The one ending of every refusal of it, at
/// import and at rest.
pub const LOOP_SIDE: SizedDecision = SizedDecision {
    lever: "move the face's boundary clear of that point, or widen the face where its \
            boundary passes it",
    size: "distance",
    passes: SizedPass::NonZero,
    side: RefusedSide::Lever,
    no_size: None,
};

/// Which singular point of its chart a point is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Singularity {
    /// A sphere's pole along its axis.
    North,
    /// A sphere's pole against its axis.
    South,
    /// A cone's apex.
    Apex,
}

/// **A sphere's two poles**, north first: its centre plus and minus its
/// unit `axis` times its radius. The one spelling of them, which
/// [`singular_points`] states for a sphere's chart.
pub fn sphere_poles<T: Real>(
    center: Point3<T>,
    axis: Vec3<T>,
    radius: T,
) -> [(Singularity, Point3<T>); 2] {
    [
        (Singularity::North, center + axis * radius),
        (Singularity::South, center - axis * radius),
    ]
}

/// **A chart's singular points**: a sphere's two poles, north first
/// ([`sphere_poles`]), and a cone's apex, from the chart's own frame;
/// none for any other kind.
pub fn singular_points<T: Real>(chart: &ChartInverse<T>) -> Vec<(Singularity, Point3<T>)> {
    let (anchor, axis) = (chart.anchor(), chart.axis());
    match chart.arm() {
        ChartArm::Sphere { radius } => sphere_poles(anchor, axis, radius).to_vec(),
        ChartArm::Cone { .. } => vec![(Singularity::Apex, anchor)],
        ChartArm::Plane | ChartArm::Cylinder { .. } | ChartArm::Torus { .. } => Vec::new(),
    }
}

/// **One loop of a face**, as [`locate`] reads it: its edges in
/// traversal order, and the stored point of the vertex each one starts
/// at, edge for edge. "At a vertex" reads the stored point, which is
/// what the vertex is, not where a carrier's evaluation ends.
#[derive(Clone, Copy, Debug)]
pub struct FaceLoop<'a, T: Real> {
    /// The loop's edges.
    pub edges: &'a [LoopEdge<T>],
    /// `corners[k]` is the stored point of the vertex `edges[k]`'s
    /// traversal starts at.
    pub corners: &'a [Point3<T>],
}

/// **The corners of a loop built by hand**, with no stored vertices:
/// each edge's traversal start, for a [`FaceLoop`] whose vertices are
/// where its carriers end.
#[must_use]
pub fn traversal_corners<T: Decide>(edges: &[LoopEdge<T>]) -> Vec<Point3<T>> {
    edges.iter().map(|e| e.traversal_ends().0).collect()
}

/// Where a singular point lies against a face ([`locate`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lies<T> {
    /// At the vertex `loops[r#loop][edge]`'s traversal starts at, which
    /// the rule admits.
    AtVertex {
        /// The loop, in the order given.
        r#loop: usize,
        /// The edge whose traversal start it is.
        edge: usize,
    },
    /// Inside `loops[r#loop][edge]`, at its carrier parameter `t`.
    InsideEdge {
        /// The loop, in the order given.
        r#loop: usize,
        /// The edge.
        edge: usize,
        /// The carrier parameter of the edge's point nearest it.
        t: T,
    },
    /// In the face's interior.
    Interior,
    /// Outside the face's closure.
    Outside,
}

/// A loop side [`locate`] could not read ([`LOOP_SIDE`]).
///
/// **Not on a curated `pncad` list, argued** (with [`Undecided`]):
/// `pncad` re-exports `topo` and `step_import` whole, so a Rust caller
/// matches it already, and a Python caller reads the finding's word
/// (`chart_singularity_undecided`, `singularity_side_undecided`) and its
/// sentence, which ends in the one recourse each arm has. **The
/// falsifier is a caller who must act differently on a side that
/// straddles its bound and one a band decision left in band** without
/// reading the sentence; then the prelude carries this type and Python a
/// word beside the variant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SideUnread {
    /// The reading does not clear its own error bound: a winding-free
    /// loop's chart area within the bound on its walk's error, or a
    /// sphere loop's closed-form turning or an arc's circulation within
    /// its rounding. No single margin states it, and no tolerance
    /// decides it.
    Straddle,
    /// A band decision of the reading is in band: an arc's span within
    /// the band of a whole turn, read by the closed form.
    Banded(Indeterminate),
}

/// A reading that does not place the point ([`locate`]).
#[derive(Clone, Debug)]
pub enum Unread<T> {
    /// Its distance to a vertex or an edge is in band
    /// ([`on_boundary`]).
    Boundary {
        /// The escalation, carrying the margin.
        cause: Indeterminate,
        /// The distance.
        distance: T,
    },
    /// The side of `loops[r#loop]` it lies on is undecided.
    Side {
        /// The loop.
        r#loop: usize,
        /// Why.
        why: SideUnread,
    },
    /// `loops[r#loop][edge]` rides a carrier read by samples that
    /// neither clear the point past the band nor bring it within reach of
    /// one of them inside the refinement's budget, or a carrier with no
    /// stated speed bound to refine against. No other edge places the
    /// point on the boundary, and none is undecided.
    Carrier {
        /// The loop.
        r#loop: usize,
        /// The edge.
        edge: usize,
    },
    /// `loops[r#loop]`'s chart image does not state a winding about the
    /// surface axis: a sample reads no chart point away from every
    /// singular point, a step of its walk cannot be certified to turn
    /// less than an eighth of a turn about the axis within the
    /// refinement's budget, or the loop winds more than once.
    Loop {
        /// The loop.
        r#loop: usize,
    },
}

/// **A reading in band**, owned: what tier 3 and import refuse a point
/// [`locate`] could not place on its boundary or a side of it, and the
/// one ending each refusal carries ([`Undecided::recourse`]). The
/// exempt readings ([`Unread::Carrier`], [`Unread::Loop`]) are not one.
/// Not on a curated `pncad` list, argued at [`SideUnread`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Undecided {
    /// Its distance to a vertex or an edge is in band ([`ON_BOUNDARY`]).
    Boundary(Indeterminate),
    /// The side of one of its loops is undecided ([`LOOP_SIDE`]).
    Side(SideUnread),
}

impl Undecided {
    /// The undecided reading `why` is, or `None` for an exempt one.
    #[must_use]
    pub fn of<T>(why: &Unread<T>) -> Option<Self> {
        match why {
            Unread::Boundary { cause, .. } => Some(Self::Boundary(*cause)),
            Unread::Side { why, .. } => Some(Self::Side(*why)),
            Unread::Carrier { .. } | Unread::Loop { .. } => None,
        }
    }

    /// The escalation it carries, where a band decision escalated; none
    /// for a side straddling its own bound.
    #[must_use]
    pub fn cause(&self) -> Option<&Indeterminate> {
        match self {
            Self::Boundary(cause) | Self::Side(SideUnread::Banded(cause)) => Some(cause),
            Self::Side(SideUnread::Straddle) => None,
        }
    }

    /// **The one ending of its refusal**, read at `at`: a distance in
    /// band as [`ON_BOUNDARY`] ends it, a side as [`LOOP_SIDE`] does —
    /// through the margin's class where a band decision escalated, and
    /// in its lever alone where the reading straddles its bound.
    #[must_use]
    pub fn recourse(&self, at: impl Into<ReadAt>) -> String {
        match self {
            Self::Boundary(cause) => ON_BOUNDARY.recourse(RefusedArm::Undecided(cause), at),
            Self::Side(SideUnread::Banded(cause)) => {
                LOOP_SIDE.recourse(RefusedArm::Undecided(cause), at)
            }
            Self::Side(SideUnread::Straddle) => LOOP_SIDE.recourse(RefusedArm::Straddle, at),
        }
    }
}

/// One singular point of the face's surface, located ([`locate`]).
#[derive(Clone, Debug)]
pub struct Located<T: Real> {
    /// Which singular point.
    pub singularity: Singularity,
    /// The point.
    pub point: Point3<T>,
    /// Where it lies.
    pub lies: Result<Lies<T>, Unread<T>>,
}

/// **Whether a distance `d` from a singular point to a vertex or edge
/// puts it on that feature**, at `band` (`chart_singularity_on_boundary`,
/// [`ON_BOUNDARY`]): on it where `d` decides zero, off it where it
/// decides clear. The one site every on-or-off decision of [`locate`]
/// is taken at, once per distance.
///
/// # Errors
///
/// The escalation of a distance in the band.
pub fn on_boundary<T: Decide>(d: T, band: Band) -> Result<bool, Indeterminate> {
    decide("chart_singularity_on_boundary", Margin::of(d), band).map(|s| s == Sign::Zero)
}

/// **Every singular point of `surface` against the face** whose loops
/// are `loops` (any order; every loop closed, each edge with its stored
/// interval and traversal, and each vertex's stored point) and whose
/// sense is `sense` (its outward normal is the chart normal where
/// `true`), on/off and side decided at `band` (module docs). Empty for a
/// surface with no singular point.
///
/// Each point is read in turn: at its nearest vertex, else inside an
/// edge (the loops and edges in order, the first that decides on), else
/// inside the face or outside it. A distance in band, or a sampled edge
/// that does not settle, is that point's answer only where no other
/// vertex or edge places it on the boundary; a distance in band is
/// answered before an unsettled edge. A reading that cannot place a
/// point ([`Unread`]) is that point's answer; the others are read
/// regardless.
pub fn locate<T: Decide>(
    surface: &Surface<T>,
    loops: &[FaceLoop<'_, T>],
    sense: bool,
    band: Band,
) -> Vec<Located<T>> {
    let Ok(chart) = ChartInverse::of(surface) else {
        return Vec::new();
    };
    let points = singular_points(&chart);
    if points.is_empty() {
        return Vec::new();
    }
    let edges: Vec<&[LoopEdge<T>]> = loops.iter().map(|l| l.edges).collect();
    let mut sides = Sides::new(surface, &chart, &points, &edges, band);
    points
        .iter()
        .map(|&(singularity, x)| Located {
            singularity,
            point: x,
            lies: locate_one(loops, sense, singularity, x, band, &mut sides),
        })
        .collect()
}

fn locate_one<T: Decide>(
    loops: &[FaceLoop<'_, T>],
    sense: bool,
    singularity: Singularity,
    x: Point3<T>,
    band: Band,
    sides: &mut Sides<'_, T>,
) -> Result<Lies<T>, Unread<T>> {
    // At a vertex: the nearest stored corner decides, once; a farther
    // one cannot decide zero where the nearest does not. A vertex in band
    // answers at once: the edges at it pass within the band too, so
    // whether the point is at the vertex or inside one of them is what
    // is undecided.
    let nearest = loops
        .iter()
        .enumerate()
        .flat_map(|(l, lp)| {
            lp.corners
                .iter()
                .enumerate()
                .map(move |(k, &c)| (l, k, (c - x).norm()))
        })
        .reduce(|a, b| if below(b.2, a.2) { b } else { a });
    if let Some((l, k, distance)) = nearest {
        match on_boundary(distance, band) {
            Ok(true) => return Ok(Lies::AtVertex { r#loop: l, edge: k }),
            Ok(false) => {}
            Err(cause) => return Err(Unread::Boundary { cause, distance }),
        }
    }
    // Inside an edge: the first that decides on.
    let mut in_band: Option<Unread<T>> = None;
    let mut unsettled: Option<Unread<T>> = None;
    for (l, lp) in loops.iter().enumerate() {
        for (k, e) in lp.edges.iter().enumerate() {
            match nearest_on_edge(e, x, band) {
                Near::Off => {}
                Near::On { t } => {
                    return Ok(Lies::InsideEdge {
                        r#loop: l,
                        edge: k,
                        t,
                    });
                }
                Near::InBand { cause, distance } => {
                    in_band.get_or_insert(Unread::Boundary { cause, distance });
                }
                Near::Unsettled => {
                    unsettled.get_or_insert(Unread::Carrier { r#loop: l, edge: k });
                }
            }
        }
    }
    if let Some(why) = in_band.or(unsettled) {
        return Err(why);
    }
    // Inside the face.
    Ok(if sides.holds(singularity, x, sense)? {
        Lies::Interior
    } else {
        Lies::Outside
    })
}

/// Where a point lies against one edge's carrier.
enum Near<T> {
    /// No interior point of the edge is within the band's escalation.
    Off,
    /// On the edge's interior point at carrier parameter `t`.
    On { t: T },
    /// The distance to its nearest interior point is in band.
    InBand { cause: Indeterminate, distance: T },
    /// A sampled carrier whose refinement settles neither.
    Unsettled,
}

/// **The carrier parameter of the point of `carrier` nearest `x`**, in
/// closed form: a line's projection, or a circle's angle about its
/// centre in its own frame brought to the turn at or past `from` and
/// short of a turn beyond it. `None` for any other carrier. The one
/// spelling of the foot, which import's meridian landing cites.
pub fn foot<T: Real>(carrier: &Curve3<T>, x: Point3<T>, from: T) -> Option<T> {
    match *carrier {
        Curve3::Line { origin, dir } => Some((x - origin).dot(dir) / dir.dot(dir)),
        Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        } => {
            let w = x - center;
            let t = w.dot(axis.cross(u_ref)).atan2(w.dot(u_ref));
            Some(from + (t - from).reduce_periodic(T::tau()))
        }
        _ => None,
    }
}

/// **The point of `e` nearest `x`**, decided once at `band`
/// ([`on_boundary`]): a line's or circle's closed-form [`foot`], kept
/// where it lies strictly inside the edge's interval (its ends are
/// vertices, which [`locate`] reads first); any other carrier by
/// [`sampled_near`].
fn nearest_on_edge<T: Decide>(e: &LoopEdge<T>, x: Point3<T>, band: Band) -> Near<T> {
    let (t0, t1) = (e.t0, e.t1);
    let speed = match e.carrier {
        Curve3::Line { dir, .. } => dir.norm(),
        Curve3::Circle { radius, .. } => radius,
        _ => return sampled_near(e, x, band),
    };
    let Some(t) = foot(&e.carrier, x, t0) else {
        unreachable!("a line or a circle has a closed-form foot")
    };
    if !(strictly_inside(t - t0, speed) && strictly_inside(t1 - t, speed)) {
        return Near::Off;
    }
    decide_near(t, (e.carrier.eval(t) - x).norm(), band)
}

/// A parameter step `m` at `speed` is strictly positive, an exact order
/// (`chart_singularity_in_span`): an edge's interval ends at its
/// vertices, read before it.
fn strictly_inside<T: Decide>(m: T, speed: T) -> bool {
    matches!(
        decide(
            "chart_singularity_in_span",
            Margin::of(m * speed),
            Band::least()
        ),
        Ok(Sign::Positive)
    )
}

/// The edge point at `t`, `distance` from the point, decided once.
fn decide_near<T: Decide>(t: T, distance: T, band: Band) -> Near<T> {
    match on_boundary(distance, band) {
        Ok(true) => Near::On { t },
        Ok(false) => Near::Off,
        Err(cause) => Near::InBand { cause, distance },
    }
}

/// Whether `a` is definitely below `b`, an exact order: a heuristic
/// choice of which sample or vertex to decide, never a decision itself.
fn below<T: Decide>(a: T, b: T) -> bool {
    matches!(
        decide(
            "chart_singularity_nearer",
            Margin::sign_of(a - b),
            Band::enclosure_sign()
        ),
        Ok(Sign::Negative)
    )
}

/// The spans a sampled carrier starts from, and the refinement's
/// budget of bisections.
const SAMPLED_SPANS: u32 = 64;
const SAMPLED_BUDGET: usize = 1 << 14;
/// The depth past which a span is read at its samples: its parameter
/// step is then below every carrier's resolution.
const SAMPLED_DEPTH: u32 = 60;

/// **A sampled carrier's point nearest `x`**, certified by the
/// carrier's speed bound `L` ([`speed_bound`]): every
/// point of a span `[a, b]` lies within `L·|b − a|/2` of the midpoint of
/// its end samples, so the span is clear of `x` where that midpoint's
/// distance less the reach decides past the band
/// (`chart_singularity_sampled_clear`). A span that is not clear is
/// bisected, down to a depth where its samples are its points.
///
/// On at the first interior sample whose distance decides zero. Off
/// where every span clears. A span whose reach is below half the band's
/// zero is resolved at its samples, which lie within that of its every
/// point; then the interior sample nearest `x` is decided
/// ([`on_boundary`]): in band, or — where it decides clear while a span
/// stays unclear — unsettled, as is a carrier with no finite speed bound
/// or a refinement past its budget.
fn sampled_near<T: Decide>(e: &LoopEdge<T>, x: Point3<T>, band: Band) -> Near<T> {
    let (t0, t1) = (e.t0, e.t1);
    let Some(speed) = speed_bound(&e.carrier) else {
        return Near::Unsettled;
    };
    let n = T::from_f64(f64::from(SAMPLED_SPANS));
    // Each sample with whether it is interior to the edge (its ends are
    // vertices, read before).
    let at = |i: u32| {
        let t = t0 + (t1 - t0) * T::from_f64(f64::from(i)) / n;
        (t, e.carrier.eval(t), i != 0 && i != SAMPLED_SPANS)
    };
    type Sample<T> = (T, Point3<T>, bool);
    let mut pending: Vec<(Sample<T>, Sample<T>, u32)> =
        (0..SAMPLED_SPANS).map(|i| (at(i), at(i + 1), 0)).collect();
    let mut best: Option<(T, T)> = None;
    let mut budget = SAMPLED_BUDGET;
    let mut unclear = false;
    let mut over_budget = false;
    let resolved = T::from_f64(band.zero() * 0.5);
    while let Some(((a, pa, ia), (b, pb, ib), depth)) = pending.pop() {
        for (t, p, interior) in [(a, pa, ia), (b, pb, ib)] {
            let d = (p - x).norm();
            if interior && best.is_none_or(|(_, bd)| below(d, bd)) {
                if let Ok(true) = on_boundary(d, band) {
                    return Near::On { t };
                }
                best = Some((t, d));
            }
        }
        let mid = Point3::new(
            (pa.x + pb.x) * T::from_f64(0.5),
            (pa.y + pb.y) * T::from_f64(0.5),
            (pa.z + pb.z) * T::from_f64(0.5),
        );
        let reach = speed * (b - a).abs() * T::from_f64(0.5);
        if matches!(
            decide(
                "chart_singularity_sampled_clear",
                Margin::of((x - mid).norm() - reach),
                band
            ),
            Ok(Sign::Positive)
        ) {
            continue;
        }
        if depth >= SAMPLED_DEPTH || below(reach, resolved) {
            unclear = true;
            continue;
        }
        if budget == 0 {
            over_budget = true;
            continue;
        }
        budget -= 1;
        let tm = (a + b) * T::from_f64(0.5);
        let m = (tm, e.carrier.eval(tm), true);
        pending.push(((a, pa, ia), m, depth + 1));
        pending.push((m, (b, pb, ib), depth + 1));
    }
    if over_budget {
        return Near::Unsettled;
    }
    if !unclear {
        return Near::Off;
    }
    match best {
        Some((t, d)) => match decide_near(t, d, band) {
            Near::Off => Near::Unsettled,
            near => near,
        },
        None => Near::Unsettled,
    }
}

/// The sides of every loop of one face, read once per loop and point.
struct Sides<'a, T: Decide> {
    surface: &'a Surface<T>,
    chart: &'a ChartInverse<T>,
    points: &'a [(Singularity, Point3<T>)],
    loops: &'a [&'a [LoopEdge<T>]],
    band: Band,
    walks: Vec<Option<Option<LoopWalk<T>>>>,
    /// Each loop with its spurs taken out ([`without_spurs`]).
    trimmed: Vec<Vec<LoopEdge<T>>>,
}

impl<'a, T: Decide> Sides<'a, T> {
    fn new(
        surface: &'a Surface<T>,
        chart: &'a ChartInverse<T>,
        points: &'a [(Singularity, Point3<T>)],
        loops: &'a [&'a [LoopEdge<T>]],
        band: Band,
    ) -> Self {
        Self {
            surface,
            chart,
            points,
            loops,
            band,
            walks: vec![None; loops.len()],
            trimmed: loops.iter().map(|edges| without_spurs(edges)).collect(),
        }
    }

    /// Whether `loops[l]`'s left side, under the chart normal, holds the
    /// singular point `x` (module docs).
    fn left_holds(
        &mut self,
        l: usize,
        singularity: Singularity,
        x: Point3<T>,
    ) -> Result<bool, Unread<T>> {
        let side = |why| Unread::Side { r#loop: l, why };
        if let Surface::Sphere { center, radius, .. } = *self.surface {
            match sphere_pole_left(center, radius, x, &self.trimmed[l], self.band) {
                PoleLeft::Read(left) => return Ok(left),
                PoleLeft::Straddle => return Err(side(SideUnread::Straddle)),
                PoleLeft::Banded(cause) => return Err(side(SideUnread::Banded(cause))),
                PoleLeft::NoClosedForm => {}
            }
        }
        let (chart, points, edges, band) = (self.chart, self.points, &self.trimmed[l], self.band);
        let walk = (*self.walks[l].get_or_insert_with(|| walk_loop(chart, points, edges, band)))
            .ok_or(Unread::Loop { r#loop: l })?;
        loop_holds_on_left(&walk, singularity).map_err(side)
    }

    /// **Whether the singular point lies on the face's side of every
    /// loop**; a loop that places it on its other side settles it
    /// outside, whatever any other loop reads.
    fn holds(
        &mut self,
        singularity: Singularity,
        x: Point3<T>,
        sense: bool,
    ) -> Result<bool, Unread<T>> {
        let mut unread: Option<Unread<T>> = None;
        for l in 0..self.loops.len() {
            match self.left_holds(l, singularity, x) {
                Ok(left) if left != sense => return Ok(false),
                Ok(_) => {}
                Err(why) => {
                    unread.get_or_insert(why);
                }
            }
        }
        unread.map_or(Ok(true), Err)
    }
}

/// **A loop with its spurs taken out**: every pair of consecutive edges
/// (cyclically) where the second retraces the first ([`retraces`]),
/// removed until none is left. A spur bounds no area, so it moves no
/// point off it from one side of the loop to the other, and a point on
/// it is on the boundary, read before any side; its tip is a cusp, whose
/// turn of half a turn either way no reading of the turning decides.
fn without_spurs<T: Decide>(edges: &[LoopEdge<T>]) -> Vec<LoopEdge<T>> {
    let mut kept: Vec<LoopEdge<T>> = Vec::with_capacity(edges.len());
    for e in edges {
        if kept.last().is_some_and(|k| retraces(k, e)) {
            kept.pop();
        } else {
            kept.push(e.clone());
        }
    }
    // The pairs that close across the loop's start.
    while kept.len() >= 2 && retraces(&kept[kept.len() - 1], &kept[0]) {
        kept.pop();
        kept.remove(0);
    }
    kept
}

/// Whether `f` retraces `e` backwards: their vertex tags reversed, the
/// same stored interval traversed the other way, the same edge where
/// both carry one's identity, and the same point at the interval's
/// middle within the shared rounding charge (`geom_core::rounding_charge`).
fn retraces<T: Decide>(e: &LoopEdge<T>, f: &LoopEdge<T>) -> bool {
    if e.forward == f.forward || e.start != f.end || e.end != f.start {
        return false;
    }
    if let (Some(a), Some(b)) = (e.carrier_id, f.carrier_id)
        && a != b
    {
        return false;
    }
    let same = |a: T, b: T| !below(a, b) && !below(b, a);
    if !(same(e.t0, f.t0) && same(e.t1, f.t1)) {
        return false;
    }
    let mid = |x: &LoopEdge<T>| x.carrier.eval((x.t0 + x.t1) * T::from_f64(0.5));
    let (a, b) = (mid(e), mid(f));
    let origin = Point3::new(T::zero(), T::zero(), T::zero());
    let rounding = geom_core::rounding_charge((a - origin).norm() + (b - origin).norm());
    !below(rounding, (a - b).norm())
}

/// **One loop walked through its surface's chart**, the reading a side
/// of a loop no closed form reads is decided from ([`walk_loop`]): only
/// the walk constructs one.
#[derive(Clone, Copy, Debug)]
pub struct LoopWalk<T> {
    winding: i8,
    area: T,
    error: T,
    nappe_v: T,
}

impl<T: Real> LoopWalk<T> {
    /// Its winding about the surface axis, in turns: `−1`, `0` or `1`.
    #[must_use]
    pub fn winding(&self) -> i8 {
        self.winding
    }

    /// The signed metric area of its chart image, positive where the
    /// image runs counter-clockwise, as the walk reads it; within
    /// [`Self::error`] of the loop's own. Read only for a loop that does
    /// not wind, whose image closes in the chart.
    #[must_use]
    pub fn area(&self) -> T {
        self.area
    }

    /// The certified bound on [`Self::area`]'s error.
    #[must_use]
    pub fn error(&self) -> T {
        self.error
    }
}

/// The samples one edge starts from, the bound on a refined walk's
/// samples, and the depth past which a step is not bisected.
const WALK_SAMPLES: u32 = 16;
const WALK_BUDGET: usize = 1 << 15;
const WALK_DEPTH: u32 = 50;

/// One sample of a loop's walk: its carrier parameter, its point, its
/// chart point (`None` on a singular point, which has none), the edge
/// it lies on, and its bisection depth.
#[derive(Clone, Copy)]
struct WalkSample<T: Real> {
    t: T,
    p: Point3<T>,
    uv: Option<Point2<T>>,
    edge: usize,
    depth: u32,
    /// The edge's traversal end: the step after it is a junction with
    /// the next edge's start (or, on a one-edge loop, its own).
    last: bool,
}

/// **The walk of one loop through `chart`**, the reading its sides are
/// decided from where no closed form reads them ([`LoopWalk`]).
///
/// Each edge is sampled along its traversal and read through the chart
/// inverse at `band`. Every step between consecutive samples is held to
/// a certified bound from its carrier's speed `L` (the step's points lie
/// within `r = L·Δt/2` of its samples' midpoint, `m`; a step between two
/// edges is the straight gap between their stored ends):
///
/// - it stays off the surface axis where `ρ_min = ρ(m) − r` is positive,
///   and then turns about it through at most `2r/ρ_min` — a line's
///   exactly its principal step;
/// - its chart `v` moves by at most `2r/R` on a sphere and `2r` along a
///   cone's slant, and a circle's by its height range about the axis
///   over `ρ_min`;
/// - so its share of the metric area `−∮ g(v) du` is within
///   `1.5·sup|g′|·Δv·Δu` of the trapezoid the walk sums (`∂g/∂v` is the
///   chart's area element: `R² cos v`, and `sin α · |v|` on a cone).
///
/// A step is bisected until it is certified to turn by less than a
/// sixteenth of a turn, so its principal step is its turn and the steps
/// of a closed loop sum to whole turns; then, for a loop that does not
/// wind, the steps carrying the most error are bisected until the summed
/// bound, rounding included, lies below the area's magnitude, or the
/// budget is spent. A sample on a singular point (one of `points`,
/// decided not clear of it at `band`) has no chart preimage and is
/// skipped: the step across it is read as the nearest representative,
/// which stands for a detour round that point on one side or the other,
/// and so leaves the side of every OTHER singular point as it is; its
/// turn is bounded by half a turn. So is a step within reach of a
/// singular point that lies on the loop (at its band, at a vertex or
/// inside an edge), which no refinement certifies off the axis.
///
/// `None` where a sample reads no chart point away from every singular
/// point, fewer than three samples read, a step cannot be certified to
/// turn by less than a sixteenth of a turn, an edge has no speed bound
/// ([`speed_bound`]: a rational spline), or the loop winds more than
/// once.
#[must_use]
pub fn walk_loop<T: Decide>(
    chart: &ChartInverse<T>,
    points: &[(Singularity, Point3<T>)],
    edges: &[LoopEdge<T>],
    band: Band,
) -> Option<LoopWalk<T>> {
    let shapes: Vec<EdgeShape<T>> = edges
        .iter()
        .map(|e| EdgeShape::of(&e.carrier, chart.axis()))
        .collect::<Option<_>>()?;
    let read = |e: usize, t: T, depth: u32, last: bool| -> Option<WalkSample<T>> {
        let p = edges[e].carrier.eval(t);
        let chart_read = chart.point(p, band);
        let uv = if chart_read.singular {
            let on_point = points.iter().any(|&(_, s)| {
                !matches!(
                    decide("chart_singularity_sample", Margin::of((s - p).norm()), band),
                    Ok(Sign::Positive)
                )
            });
            if !on_point {
                return None;
            }
            None
        } else {
            Some(chart_read.uv)
        };
        Some(WalkSample {
            t,
            p,
            uv,
            edge: e,
            depth,
            last,
        })
    };
    let mut samples: Vec<WalkSample<T>> = Vec::new();
    for (k, e) in edges.iter().enumerate() {
        let (a, b) = if e.forward {
            (e.t0, e.t1)
        } else {
            (e.t1, e.t0)
        };
        let n = T::from_f64(f64::from(WALK_SAMPLES));
        for i in 0..=WALK_SAMPLES {
            let t = a + (b - a) * T::from_f64(f64::from(i)) / n;
            samples.push(read(k, t, 0, i == WALK_SAMPLES)?);
        }
    }
    // The singular points on this loop: a step within reach of one is
    // read as the detour round it, as a sample on one is.
    let on_loop: Vec<Point3<T>> = points
        .iter()
        .map(|&(_, x)| x)
        .filter(|&x| {
            edges.iter().any(|e| {
                !matches!(
                    on_boundary((e.traversal_ends().0 - x).norm(), band),
                    Ok(false)
                ) || !matches!(nearest_on_edge(e, x, band), Near::Off)
            })
        })
        .collect();
    let shape = Shape::of(chart, on_loop, T::from_f64(band.escalate()));
    // Certify every step's turn, bisecting where it is not.
    loop {
        let mut wide = Vec::new();
        for i in 0..samples.len() {
            let (s, q) = (samples[i], samples[(i + 1) % samples.len()]);
            if !s.last && step_bound(&shape, &shapes, s, q).is_none() {
                wide.push(i);
            }
        }
        if wide.is_empty() {
            break;
        }
        if !bisect(&mut samples, &wide, &read) {
            return None;
        }
    }
    let mut reading = sum_walk(&shape, &shapes, &samples)?;
    if reading.winding != 0 {
        return Some(reading.walk());
    }
    // Bisect the steps carrying the most error until the area clears
    // its bound.
    loop {
        if area_clears(&reading) {
            break;
        }
        let total = reading.error;
        let share = total / T::from_f64(2.0 * samples.len() as f64);
        let heavy: Vec<usize> = reading
            .errors
            .iter()
            .enumerate()
            .filter(|&(i, &e)| !samples[i].last && !below(e, share))
            .map(|(i, _)| i)
            .collect();
        if heavy.is_empty() || !bisect(&mut samples, &heavy, &read) {
            break;
        }
        reading = sum_walk(&shape, &shapes, &samples)?;
    }
    Some(reading.walk())
}

/// Whether a winding-free walk's area clears its error bound, its
/// reading wholly on one side of zero (`chart_singularity_loop_side`).
fn area_clears<T: Decide>(reading: &WalkSum<T>) -> bool {
    matches!(
        decide(
            "chart_singularity_loop_side",
            Margin::sign_of(reading.area.abs() - reading.error),
            Band::enclosure_sign()
        ),
        Ok(Sign::Positive)
    )
}

/// Bisects the steps after the samples at `at` (each within one edge),
/// within the walk's depth and budget. Whether any was bisected.
fn bisect<T: Decide>(
    samples: &mut Vec<WalkSample<T>>,
    at: &[usize],
    read: &impl Fn(usize, T, u32, bool) -> Option<WalkSample<T>>,
) -> bool {
    let mut inserted = 0;
    for &i in at {
        let i = i + inserted;
        let (s, q) = (samples[i], samples[(i + 1) % samples.len()]);
        let depth = s.depth.max(q.depth);
        if depth >= WALK_DEPTH || samples.len() >= WALK_BUDGET {
            continue;
        }
        let Some(m) = read(s.edge, (s.t + q.t) * T::from_f64(0.5), depth + 1, false) else {
            continue;
        };
        samples.insert(i + 1, m);
        inserted += 1;
    }
    inserted > 0
}

/// The chart a walk runs on, as its error bound reads it.
struct Shape<T: Real> {
    anchor: Point3<T>,
    axis: Vec3<T>,
    kind: ShapeKind<T>,
    /// The singular points on the loop walked.
    on_loop: Vec<Point3<T>>,
    /// The band's escalation, the reach of "on the loop".
    escalate: T,
}

#[derive(Clone, Copy)]
enum ShapeKind<T> {
    /// A sphere of radius `R`.
    Sphere(T),
    /// A cone of half-angle `α`: `sin α`, `cos α`.
    Cone(T, T),
    /// No singular point: no walk is read.
    Other,
}

impl<T: Decide> Shape<T> {
    fn of(chart: &ChartInverse<T>, on_loop: Vec<Point3<T>>, escalate: T) -> Self {
        let kind = match chart.arm() {
            ChartArm::Sphere { radius } => ShapeKind::Sphere(radius),
            ChartArm::Cone { half_angle } => ShapeKind::Cone(half_angle.sin(), half_angle.cos()),
            ChartArm::Plane | ChartArm::Cylinder { .. } | ChartArm::Torus { .. } => {
                ShapeKind::Other
            }
        };
        Self {
            anchor: chart.anchor(),
            axis: chart.axis(),
            kind,
            on_loop,
            escalate,
        }
    }

    /// `g(v)`, whose `v`-derivative is the chart's area element over
    /// `du dv`.
    fn g(&self, v: T) -> T {
        match self.kind {
            ShapeKind::Sphere(r) => r.powi(2) * v.sin(),
            ShapeKind::Cone(sin, _) => sin * v.abs() * v * T::from_f64(0.5),
            ShapeKind::Other => T::zero(),
        }
    }

    /// A point's distance from the axis.
    fn rho(&self, p: Point3<T>) -> T {
        let w = p - self.anchor;
        (w - self.axis * w.dot(self.axis)).norm()
    }
}

/// **A carrier's speed bound, where it is one at every scalar**: the sup
/// of [`geom::carrier_speed::speed_range`], positive and readable. `None`
/// for a rational spline, whose bound `speed_range` certifies only at
/// `Interval`: at `f64` its expressions run in round-to-nearest and are
/// no bound at extreme weights, and a reading generic over the scalar
/// cannot tell the two apart. A sampled arm with no bound settles
/// nothing ([`Unread::Carrier`], and a walk over it [`Unread::Loop`]):
/// both exempt, never decided on a bound that may not hold.
fn speed_bound<T: Decide>(carrier: &Curve3<T>) -> Option<T> {
    if let Curve3::Nurbs(spline) = carrier
        && crate::props::is_rational(spline)
    {
        return None;
    }
    let speed = geom::carrier_speed::speed_range(carrier).1.get();
    (!speed.is_poison()
        && matches!(
            decide(
                "chart_singularity_sampled_speed",
                Margin::sign_of(speed),
                Band::enclosure_sign()
            ),
            Ok(Sign::Positive)
        ))
    .then_some(speed)
}

/// What a walk's error bound reads of one edge's carrier: its speed
/// bound ([`speed_bound`]), whether it is a line
/// (whose azimuth is monotone, so a step turns by its principal step),
/// and a circle's height range about the chart axis over its whole turn,
/// `2r·|â × n̂|` (zero for a parallel).
#[derive(Clone, Copy)]
struct EdgeShape<T> {
    speed: T,
    line: bool,
    height: Option<T>,
}

impl<T: Decide> EdgeShape<T> {
    fn of(carrier: &Curve3<T>, axis: Vec3<T>) -> Option<Self> {
        let speed = speed_bound(carrier)?;
        Some(match *carrier {
            Curve3::Line { .. } => Self {
                speed,
                line: true,
                height: None,
            },
            Curve3::Circle {
                axis: n, radius, ..
            } => Self {
                speed,
                line: false,
                height: Some(radius.abs() * T::from_f64(2.0) * axis.cross(n / n.norm()).norm()),
            },
            _ => Self {
                speed,
                line: false,
                height: None,
            },
        })
    }
}

/// The certified error of one step's share of the area ([`walk_loop`]),
/// where the step is certified to turn by less than a sixteenth of a
/// turn, or crosses a skipped sample on a singular point; `None` where
/// it is not.
fn step_bound<T: Decide>(
    shape: &Shape<T>,
    edges: &[EdgeShape<T>],
    s: WalkSample<T>,
    q: WalkSample<T>,
) -> Option<T> {
    let half = T::from_f64(0.5);
    let along = !s.last;
    let reach = if along {
        edges[s.edge].speed * (q.t - s.t).abs() * half
    } else {
        (q.p - s.p).norm() * half
    };
    let positive = |m: T| {
        matches!(
            decide(
                "chart_singularity_walk_step",
                Margin::sign_of(m),
                Band::enclosure_sign()
            ),
            Ok(Sign::Positive)
        )
    };
    let span = reach * T::from_f64(2.0);
    let mid = Point3::new(
        (s.p.x + q.p.x) * half,
        (s.p.y + q.p.y) * half,
        (s.p.z + q.p.z) * half,
    );
    // Within reach of a singular point on the loop: the detour round it.
    let detour = shape
        .on_loop
        .iter()
        .any(|&x| !positive((x - mid).norm() - reach - shape.escalate));
    let (turn, dv) = match (s.uv, q.uv) {
        (Some(a), Some(b)) if !detour => {
            let rho_min = shape.rho(mid) - reach;
            if !positive(rho_min) {
                return None;
            }
            let turn = if along && edges[s.edge].line {
                principal(b.x - a.x).abs()
            } else {
                span / rho_min
            };
            if !positive(T::pi() / T::from_f64(8.0) - turn) {
                return None;
            }
            let dv = match shape.kind {
                ShapeKind::Sphere(r) => span / r,
                ShapeKind::Cone(..) | ShapeKind::Other => span,
            };
            let dv = match (along, edges[s.edge].height, shape.kind) {
                (true, Some(height), ShapeKind::Sphere(_)) => dv.min(height / rho_min),
                (true, Some(height), ShapeKind::Cone(_, cos)) => dv.min(height / cos),
                _ => dv,
            };
            (turn, dv)
        }
        // Across a skipped sample on a singular point, or within reach
        // of one on the loop: its principal step, at most half a turn.
        _ => (
            T::pi(),
            match shape.kind {
                ShapeKind::Sphere(r) => span / r,
                ShapeKind::Cone(..) | ShapeKind::Other => span,
            },
        ),
    };
    let slope = match shape.kind {
        ShapeKind::Sphere(r) => r.powi(2),
        ShapeKind::Cone(sin, _) => {
            let v = |w: WalkSample<T>| w.uv.map_or(T::zero(), |uv| uv.y.abs());
            sin * (v(s).max(v(q)) + span)
        }
        ShapeKind::Other => T::zero(),
    };
    Some(T::from_f64(1.5) * slope * dv * turn)
}

/// The principal value of an angle step, in `(−π, π]`.
fn principal<T: Real>(d: T) -> T {
    let (s, c) = d.sin_cos();
    s.atan2(c)
}

/// A walk's sums: its winding, its area, its bound, and each step's
/// share of the bound.
struct WalkSum<T> {
    winding: i8,
    area: T,
    error: T,
    errors: Vec<T>,
    nappe_v: T,
}

impl<T: Real> WalkSum<T> {
    fn walk(&self) -> LoopWalk<T> {
        LoopWalk {
            winding: self.winding,
            area: self.area,
            error: self.error,
            nappe_v: self.nappe_v,
        }
    }
}

/// Sums a certified walk ([`walk_loop`]). `None` where fewer than three
/// samples read a chart point or the loop winds more than once.
fn sum_walk<T: Decide>(
    shape: &Shape<T>,
    edges: &[EdgeShape<T>],
    samples: &[WalkSample<T>],
) -> Option<WalkSum<T>> {
    let charted: Vec<(usize, Point2<T>)> = samples
        .iter()
        .enumerate()
        .filter_map(|(i, s)| s.uv.map(|uv| (i, uv)))
        .collect();
    let m = charted.len();
    if m < 3 {
        return None;
    }
    let n = samples.len();
    let mut errors = vec![T::zero(); n];
    let mut total = T::zero();
    let mut magnitude = T::zero();
    let mut area = T::zero();
    let (mut v_lo, mut v_hi) = (charted[0].1.y, charted[0].1.y);
    for (j, &(i, uv)) in charted.iter().enumerate() {
        (v_lo, v_hi) = (v_lo.min(uv.y), v_hi.max(uv.y));
        let (k, next) = charted[(j + 1) % m];
        let step = principal(next.x - uv.x);
        total = total + step;
        let term = -(shape.g(uv.y) + shape.g(next.y)) * T::from_f64(0.5) * step;
        area = area + term;
        magnitude = magnitude + term.abs();
        // The steps between the two charted samples, a skipped one among
        // them where `k` is not the next.
        let mut i0 = i;
        loop {
            let i1 = (i0 + 1) % n;
            errors[i0] = step_bound(shape, edges, samples[i0], samples[i1])?;
            if i1 == k {
                break;
            }
            i0 = i1;
        }
    }
    // Whole turns by construction: read at the half turns, exactly.
    let pi = T::pi();
    let past = |c: f64| {
        decide(
            "chart_singularity_winding",
            Margin::sign_of(total - pi * T::from_f64(c)),
            Band::enclosure_sign(),
        )
    };
    let winding = match (past(1.0).ok()?, past(-1.0).ok()?) {
        (Sign::Positive, _) => match past(3.0).ok()? {
            Sign::Negative => 1,
            _ => return None,
        },
        (_, Sign::Negative) => match past(-3.0).ok()? {
            Sign::Positive => -1,
            _ => return None,
        },
        _ => 0,
    };
    // The sum's own rounding: not a short chain but `m` terms, each a
    // few roundings of a quantity its magnitude bounds, summed with one
    // rounding per addition — Higham's `γ_n·Σ|term|`, `n = m` additions
    // and a term's own four, at `2u` each with room (`u` the unit
    // roundoff, `geom_core::UNIT_ROUNDOFF`).
    let rounding = T::from_f64(2.0 * (m as f64 + 8.0) * geom_core::UNIT_ROUNDOFF) * magnitude;
    let error = errors.iter().fold(rounding, |a, &e| a + e);
    Some(WalkSum {
        winding,
        area,
        error,
        errors,
        nappe_v: v_lo + v_hi,
    })
}

/// **Whether a walked loop's left side holds the singular point**, under
/// the chart normal (module docs): a winding loop by its winding and,
/// on a cone, its nappe; one that does not wind by its area's sign,
/// where the area clears its error bound (`chart_singularity_loop_side`).
///
/// # Errors
///
/// [`SideUnread::Straddle`] where the area, or a cone loop's nappe,
/// does not clear its bound.
pub fn loop_holds_on_left<T: Decide>(
    walk: &LoopWalk<T>,
    singularity: Singularity,
) -> Result<bool, SideUnread> {
    match walk.winding {
        0 => {
            let reading = WalkSum {
                winding: 0,
                area: walk.area,
                error: walk.error,
                errors: Vec::new(),
                nappe_v: walk.nappe_v,
            };
            if !area_clears(&reading) {
                return Err(SideUnread::Straddle);
            }
            Ok(matches!(
                decide(
                    "chart_singularity_loop_side",
                    Margin::sign_of(walk.area),
                    Band::enclosure_sign()
                ),
                Ok(Sign::Negative)
            ))
        }
        w => match singularity {
            Singularity::North => Ok(w == 1),
            Singularity::South => Ok(w == -1),
            Singularity::Apex => Ok((w == 1) == (nappe(walk)? == NonzeroSign::Negative)),
        },
    }
}

/// **The nappe a cone loop off the apex runs on**: the sign of its
/// samples' slant (`chart_singularity_nappe`). Every step of the walk is
/// certified off the axis, so the loop stays on one nappe and every
/// sample's slant has its sign: an exact order.
///
/// # Errors
///
/// [`SideUnread::Straddle`] for a slant range that reads no sign.
pub fn nappe<T: Decide>(walk: &LoopWalk<T>) -> Result<NonzeroSign, SideUnread> {
    decide_nonzero(
        "chart_singularity_nappe",
        Margin::sign_of(walk.nappe_v),
        Band::enclosure_sign(),
    )
    .map_err(|_| SideUnread::Straddle)
}

/// **Whether the singular point lies on the face's side of every
/// loop** — in the face, for a point off its boundary (module docs),
/// read as [`locate`] reads it. A loop that places it on its other side
/// settles it outside, whatever any other loop reads. It answers a point
/// `locate` places at a vertex too, the side its loops leave it on
/// (import's strut tip reads that).
///
/// # Errors
///
/// [`Unread::Loop`] for a loop whose walk states no winding, and
/// [`Unread::Side`] for a side undecided, where no loop settles the point
/// outside.
pub fn holds<T: Decide>(
    surface: &Surface<T>,
    loops: &[&[LoopEdge<T>]],
    sense: bool,
    singularity: Singularity,
    band: Band,
) -> Result<bool, Unread<T>> {
    let Ok(chart) = ChartInverse::of(surface) else {
        return Ok(false);
    };
    let points = singular_points(&chart);
    let Some(&(_, x)) = points.iter().find(|(s, _)| *s == singularity) else {
        return Ok(false);
    };
    Sides::new(surface, &chart, &points, loops, band).holds(singularity, x, sense)
}

//! **The cone of material at a point** — a solid's LINK there: every
//! face of the solid through the point, as its sector ([`Link`]).
//!
//! A link is read three ways:
//! - [`Link::reads`]: where a direction from the point lies against the
//!   material — inside, outside, or on a sector — by crossing parity, no
//!   convexity assumed, so a saddle reads like any other corner;
//! - [`Link::offset`]: a point's signed distance from the cone's
//!   boundary, negative inside, as a closed form over the sectors, and
//!   [`Link::lowest_on`] the lowest of it along a segment;
//! - `rest::judge`: the one test of two links at one point, which the
//!   census's touch analysis reads.
//!
//! A link is built off a live body at a vertex ([`Link::at_point`]),
//! inside a line edge ([`Link::on_edge`]) or inside a planar face
//! ([`Link::in_face`]); the census builds its own off its snapshot,
//! gathered across the keys a touch ties to its point.
//!
//! **Every sign decided here is a point's signed distance from a
//! plane**, in metres, and it comes through ONE door: [`metric`], whose
//! [`metric::Distance`] is built only from a point and a plane (a point
//! on it and its unit normal). Directions — face normals, ray
//! directions, their bisectors — only PROPOSE planes. One other decision
//! stays, named where it is made: [`pairing`], a sign of unit normals of
//! magnitude about 1. `link::tests::
//! the_link_decides_only_through_its_doors` holds that by reading this
//! file's source, and the census's touch analysis beside it.
//!
//! Each face is read through its PIECE at the point `p`: the part of the
//! face visible from `p` ([`face_piece`]). The piece is star-shaped from
//! `p`, so every point of it lies on a segment from `p` to its boundary,
//! and it spans the face's whole sector at `p`. Nothing is extrapolated,
//! and a face that is not convex is read where it is near `p`, not where
//! it reaches back far from it.
//!
//! A link holds planar faces bounded by line edges only: a curved face
//! or edge at the point refuses ([`TouchUnread::Unreadable`]).

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};
use crate::validate::decide;

/// K name: a point's signed distance from the plane of a link's face
/// through the touch point `p`, in metres — which side of the face's
/// plane a piece's end, a probe or a parity arc's end lies on.
pub(crate) const TOUCH_PLANE: &str = "census_touch_plane";
/// K name: a point's signed distance from the plane through a ray from
/// `p` (a sector's ray, or a face vertex's while its piece is built),
/// normal to its face, in metres — which side of the ray the point lies
/// on, in the face's plane, read no further out than the ray's point.
const TOUCH_WEDGE: &str = "census_touch_wedge";
/// K name: a point on a ray's line through `p` read along the ray, in
/// metres — ahead of `p` or behind it, read no further out than the
/// ray's point.
const TOUCH_AHEAD: &str = "census_touch_ahead";
/// K name: a parity arc's two ends read against the plane through `p`
/// normal to their bisector, in metres — the arc under half a turn.
const TOUCH_ARC: &str = "census_touch_arc";
/// K name, building a face's piece: one edge's crossing of a probe ray
/// read against another's along the ray, in metres — which edge the ray
/// meets first.
const TOUCH_PIECE_REACH: &str = "census_touch_piece_reach";
/// K name, building a face's piece: where a ray through a gap's bound
/// meets the gap's edge, read against that edge's ends along it, in
/// metres — on the edge, or past an end.
const TOUCH_PIECE_MEET: &str = "census_touch_piece_meet";
/// K name: two coplanar faces' outward normals, one of each link,
/// aligned or opposed ([`geom_brep::classify_material_pairing_as`]).
const TOUCH_NORMAL: &str = "census_touch_normal";

/// **The one door every verdict sign of the touch analysis comes
/// through.** A [`Distance`] has a private field and one constructor,
/// from a point and a plane, so a lever, a product or any other
/// quantity cannot be decided here as a verdict: it has no way to
/// become a `Distance`.
///
/// What the type cannot hold: that `n` is a unit normal. Every caller
/// passes a normalized direction or a face's outward normal, and a
/// normal scaled by a length would be a lever spelled as a plane; the
/// source row states that blind spot.
pub(crate) mod metric {
    use super::{Band, Decide, Margin, Point3, Real, Sign, Vec3, decide};

    /// A point's signed distance from a plane, in metres.
    pub(crate) struct Distance<T>(T);

    impl<T: Real> Distance<T> {
        /// `n·(q − o)`: `q`'s signed distance from the plane through
        /// `o` with unit normal `n`.
        pub(crate) fn of(q: Point3<T>, o: Point3<T>, n: Vec3<T>) -> Self {
            Self(n.dot(q - o))
        }
    }

    /// How a link reading decides: the run band, and a slack — the
    /// farthest a link's sector stands from the point it is read about
    /// (a corner on another key of the point, decided one point with
    /// it). A definite sign is taken only where it holds over every
    /// point within the slack of the one read, so no sector's offset
    /// can flip it; a Zero stays the band's coincidence, the slack being
    /// within it. `None` where the reading is about the sector's own
    /// apex ([`face_piece`](super::face_piece)), so nothing is moved and the one read is
    /// the reading.
    #[derive(Clone, Copy)]
    pub(crate) struct Reading<T> {
        pub(crate) band: Band,
        pub(crate) slack: Option<T>,
    }

    impl<T: Real> Reading<T> {
        /// The slack as a distance: zero where there is none.
        pub(crate) fn slack(&self) -> T {
            self.slack.unwrap_or(T::zero())
        }
    }

    /// The distance's sign under the run band, as the K row `name`;
    /// `None` in band.
    pub(crate) fn sign<T: Decide>(name: &'static str, d: Distance<T>, band: Band) -> Option<Sign> {
        read(name, d.0, band)
    }

    /// The distance's sign under `r` ([`Reading`]): Zero as the band
    /// reads it, a definite sign only where the distance moved by the
    /// slack toward zero reads the same; `None` otherwise.
    pub(crate) fn sign_over<T: Decide>(
        name: &'static str,
        d: Distance<T>,
        r: Reading<T>,
    ) -> Option<Sign> {
        let s = read(name, d.0, r.band)?;
        let Some(slack) = r.slack else {
            return Some(s);
        };
        match s {
            Sign::Zero => Some(Sign::Zero),
            Sign::Positive => {
                (read(name, d.0 - slack, r.band)? == Sign::Positive).then_some(Sign::Positive)
            }
            Sign::Negative => {
                (read(name, d.0 + slack, r.band)? == Sign::Negative).then_some(Sign::Negative)
            }
        }
    }

    fn read<T: Decide>(name: &'static str, v: T, band: Band) -> Option<Sign> {
        decide(name, Margin::of(v), band).ok()
    }
}

use metric::{Distance, Reading};

/// What the local analysis of one touch between two solids decided.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TouchVerdict {
    /// Every piece of both links read, on decided readings, as no
    /// crossing: the materials' interiors are disjoint near the touch.
    /// The certificate is minted only by [`rest::judge`], which runs
    /// the readings.
    Rest(rest::Rest),
    /// The two materials' interiors meet at the touch, on decided
    /// readings: the touch is the degenerate shape of a crossing.
    Crossing,
    /// No side read, for the reason named.
    Unread(TouchUnread),
}

/// **Why tier 3′ read no side** at a touch: the one home of its
/// refusal reasons, which a cross-solid touch renders as the census's
/// `Undecided`, an edge lying in a face of its own solid as
/// [`ValidationError::FaceThroughEdgeWedgeUndecided`](crate::validate::ValidationError::FaceThroughEdgeWedgeUndecided), two coincident
/// edges on one line as
/// [`ValidationError::CoincidentWedgesOverlapUndecided`](crate::validate::ValidationError::CoincidentWedgesOverlapUndecided), and `Gap`'s
/// cell arms as a typed refusal.
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names this
/// rung; the disposition table cites this paragraph as its home). A Rust
/// caller can already name and match this type — `pncad` re-exports
/// `topo` whole — and the Python door already says which check refused
/// (`face_through_edge_wedge_undecided`) and carries the reason's
/// sentence. Publishing the name owes the CUR3 property row
/// `carried_refusal_payloads_are_matchable_through_the_prelude` and a
/// Python word per arm, both the façade crate's to write.
///
/// **The falsifier is a Python caller who must tell a curved edge face
/// (no way through yet) from a reading in band (move the parts)**
/// without reading the sentence. When the façade carries the reason,
/// delete this paragraph and the disposition row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "test-support"), derive(strum::EnumIter))]
pub enum TouchUnread {
    /// A sign the analysis needed is in band.
    InBand,
    /// A face's piece at the touch did not build: a choice in building
    /// it was in band, or its readings contradicted each other within
    /// ε. The smallest piece of all, which refuses.
    PieceInBand,
    /// A curved face or edge takes part.
    Unreadable,
    /// A face meets the touch point both at a corner and along an edge,
    /// or along two of its edges; its piece there does not close on
    /// decided readings (a gap between neighbouring turns of half a turn
    /// or more, or no edge met across one, [`face_piece`]); or a link's
    /// sectors do not pair up at their bounds ([`Link::closes`]).
    Unanalysed,
    /// Two faces of one solid lie on each other at the touch (a slit,
    /// or a face doubled).
    Degenerate,
    /// A link's extent is too long or too short to be a number.
    ChordScale,
    /// The topology around the touch did not read. The census runs only
    /// on bodies tier 1 admits (loops closed, every key resolving), so
    /// this is a kernel defect.
    Corrupt,
}

impl From<TouchUnread> for TouchVerdict {
    fn from(why: TouchUnread) -> Self {
        Self::Unread(why)
    }
}

/// Where a face holds the touch point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Hold {
    /// At its corner, this vertex.
    Corner(VertexKey),
    /// Inside its edge, the edge between these two vertices.
    Edge(VertexKey, VertexKey),
    /// Inside the face.
    Inside,
}

/// **One face's sector at the touch point `p`**: the face's piece there
/// ([`face_piece`]) as a fan of wedges about `p`, each a triangle
/// `(p, a, b)` of real points of the face under half a turn, in turn
/// counterclockwise about the outward normal, consecutive wedges on one
/// ray. The fan spans the face's whole sector at `p`.
pub(crate) struct Sector<T: Real> {
    pub(crate) surface: geom::Surface<T>,
    /// `Face::sense`, the bit [`geom_brep::classify_material_pairing_as`]
    /// takes.
    pub(crate) sense: bool,
    /// The outward unit normal.
    pub(crate) outward: Vec3<T>,
    /// The point the fan is built about: the link's point, or the
    /// corner of this face on one of its keys.
    pub(crate) apex: Point3<T>,
    pub(crate) wedges: Vec<Segment<T>>,
    /// The fan closes all the way round: `p` is inside the face.
    /// Otherwise its first ray and its last are the face's edges through
    /// `p`, the sector's bounds.
    pub(crate) closed: bool,
}

/// Where a point on a sector's plane lies against the sector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Place {
    /// Strictly inside a wedge, or on a ray two wedges share.
    Inside,
    Outside,
    /// On one of the sector's bounds.
    Bound,
}

/// **One solid's link at a touch point `p`**: every face of the solid
/// through `p`, as its sector there. Its material near `p` is the set of
/// directions an odd number of its sectors separate from outside
/// ([`Link::reads`]).
pub struct Link<T: Real> {
    pub(crate) p: Point3<T>,
    pub(crate) sectors: Vec<Sector<T>>,
    /// The farthest wedge point from `p`.
    pub(crate) reach: T,
    /// The radius of the ball about `p` the link's wedges fill: the
    /// least distance from `p` to a wedge's far side. Inside it each of
    /// the link's faces covers its whole sector; a face of the solid
    /// that reaches into the ball without being tied to the point is
    /// not in the link, and is the backstop's to answer for (its
    /// material probe and findings).
    pub(crate) ball: T,
    /// The farthest a sector's apex stands from `p`: zero but where a
    /// corner on another key of the point was decided one point with it.
    pub(crate) tie: T,
}

/// Where a direction from `p` lies against a link's material.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Material {
    /// Inside the material.
    In,
    /// Outside it.
    Out,
    /// On the link's sector with this index.
    On(usize),
}

/// Two outward normals against each other, aligned or opposed, through
/// the one classifier of that question under the K row `name`: two
/// coplanar faces', one of each link ([`TOUCH_NORMAL`]). The magnitude
/// is about 1 wherever it is asked (the faces lie on one plane), and
/// only its sign is read.
/// `None` is every refusal — in band, decided zero, or poisoned — and
/// each reads as in band, ending as the touch's one in-band refusal
/// (`TouchInBand`), which carries no margin.
fn pairing<T: Decide>(
    name: &'static str,
    s: (&geom::Surface<T>, bool),
    t: (&geom::Surface<T>, bool),
    p: Point3<T>,
    reach: T,
    band: Band,
) -> Option<geom_brep::MaterialPairing> {
    let arm = geom_brep::folded_lever_arm(s.0, t.0, p, reach);
    geom_brep::classify_material_pairing_as(name, s.0, s.1, t.0, t.1, p, arm, band).ok()
}

/// A chord from the touch point to `q`, as its length: a number, or a
/// [`TouchUnread::ChordScale`] refusal for an edge too long or too
/// short to measure. The one home of that question in the analysis.
fn chord<T: Real>(p: Point3<T>, q: Point3<T>) -> Result<T, TouchUnread> {
    let v = q - p;
    let len = v.norm();
    if !geom_core::is_finite_length(len) || geom_core::is_underflowed_length(len, v.norm_witness())
    {
        return Err(TouchUnread::ChordScale);
    }
    Ok(len)
}

/// A boundary edge of a face, as its two end points.
type Segment<T> = (Point3<T>, Point3<T>);

/// Where `p` sits on the face whose piece is built.
#[derive(Clone, Copy, Debug)]
pub(crate) enum PieceAt {
    /// At the face's corner: loop positions `(ring, k)` of `p`.
    Corner(usize, usize),
    /// Inside the face's edge from `ring[k]` to `ring[k + 1]`.
    Edge(usize, usize),
    /// Inside the face.
    Inside,
}

/// Where a direction from `p` lies, turning counterclockwise about the
/// face's outward normal from a reference ray: on it, strictly left of
/// it, straight back, or strictly right.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Turn {
    Ahead,
    Left,
    Back,
    Right,
}

/// **A face's piece at `p`: the part of the face visible from `p`**
/// (the design's S2′), as its vertices — face vertices and points where
/// a ray from `p` through a face vertex meets the face's boundary
/// beyond it, all real points of the face.
///
/// `rings` are the face's loops as the snapshot walks them: each with
/// the face's interior on its LEFT about the outward normal `m` (the
/// outer loop counterclockwise, each hole clockwise). The sector the
/// piece spans at a corner or an edge is read off that orientation.
///
/// `also` names the face's other corners at `p`, where the face meets
/// `p` again: their edges bound other sectors of the face and are not
/// read as blocking here, and the sector ends at the first of their
/// incoming rays it reaches.
///
/// Holes' edges block and holes' vertices cast windows. At a corner the
/// piece is bounded by `p`'s two edges and holds the corner's whole
/// sector; inside an edge it is bounded by that edge; inside the face
/// it holds a disc about `p`. It is built as a fan about `p`:
/// 1. the face's vertices are sorted by their turn about `p`;
/// 2. each gap between two neighbouring turns is checked EMPTY — no
///    vertex strictly inside it, on decided readings — so no edge ends
///    inside a gap. That is what the sort's comparisons cannot promise
///    alone: two readings within ε are not transitive, and a sort over
///    them can misplace a vertex; the check refuses where it did;
/// 3. in each gap, the edge a ray from `p` meets first is read on the
///    bisecting ray — no edge ends inside the gap, so the order of the
///    edges along every ray through it is the same — and cut where the
///    rays through the gap's two bounds meet it, each meeting checked
///    to lie on that edge.
///
/// Every choice is decided in metres, through [`metric`]. A choice in
/// band, or readings that contradict each other within ε, build no
/// piece ([`TouchUnread::PieceInBand`]): the smallest piece of all,
/// which costs a refusal and never a rest. A turn read Zero is a
/// direction shared within ε, and an edge read as blocking within ε
/// blocks: both only make the piece smaller. An edge whose both ends lie
/// decidedly behind `p` along a probe ray never meets it, whatever else
/// is undecided about it.
pub(crate) fn face_piece<T: Decide>(
    p: Point3<T>,
    m: Vec3<T>,
    rings: &[Vec<Point3<T>>],
    at: PieceAt,
    also: &[(usize, usize)],
    band: Band,
) -> Result<Vec<Point3<T>>, TouchUnread> {
    use core::cmp::Ordering::{Equal, Greater, Less};
    use metric::sign;
    // The fan is built about its own apex: no slack.
    let rd = Reading { band, slack: None };
    // Which side of the ray from `p` through the point `r` a point is
    // on ([`wedge_side`], read no further out than `r`).
    let side = |r: Point3<T>, q: Point3<T>| wedge_side(p, m, r, q, rd);
    let unit = |q: Point3<T>| (q - p).normalize();
    // Every face vertex but `p`, and every boundary edge that does not
    // hold `p`.
    let mut points = Vec::new();
    let mut edges: Vec<Segment<T>> = Vec::new();
    for (r, ring) in rings.iter().enumerate() {
        let n = ring.len();
        for k in 0..n {
            let corner = |(rr, kk): (usize, usize)| rr == r && (k == kk || (k + 1) % n == kk);
            let holds_p = match at {
                PieceAt::Corner(rr, kk) => corner((rr, kk)),
                PieceAt::Edge(rr, kk) => rr == r && k == kk,
                PieceAt::Inside => false,
            } || also.iter().any(|&c| corner(c));
            if !holds_p {
                edges.push((ring[k], ring[(k + 1) % n]));
            }
            let at_p = matches!(at, PieceAt::Corner(rr, kk) if rr == r && kk == k)
                || also.contains(&(r, k));
            if !at_p {
                points.push(ring[k]);
            }
        }
    }
    // The reference ray the turns count from, and the last turn the
    // piece reaches (`None`: all the way round).
    let (start, end) = match at {
        PieceAt::Corner(r, k) => {
            let n = rings[r].len();
            (rings[r][(k + 1) % n], Some(rings[r][(k + n - 1) % n]))
        }
        PieceAt::Edge(r, k) => {
            let n = rings[r].len();
            (rings[r][(k + 1) % n], Some(rings[r][k]))
        }
        PieceAt::Inside => (*points.first().ok_or(TouchUnread::Corrupt)?, None),
    };
    let turn = |q: Point3<T>| -> Result<Turn, TouchUnread> {
        Ok(match side(start, q) {
            Some(Sign::Positive) => Turn::Left,
            Some(Sign::Negative) => Turn::Right,
            Some(Sign::Zero) => match ahead(p, start, q, rd) {
                Some(Sign::Positive) => Turn::Ahead,
                Some(Sign::Negative) => Turn::Back,
                _ => return Err(TouchUnread::PieceInBand),
            },
            None => return Err(TouchUnread::PieceInBand),
        })
    };
    // The order of two points' turns about `p`.
    let order =
        |a: (Turn, Point3<T>), b: (Turn, Point3<T>)| -> Result<core::cmp::Ordering, TouchUnread> {
            if a.0 != b.0 {
                return Ok(a.0.cmp(&b.0));
            }
            if matches!(a.0, Turn::Ahead | Turn::Back) {
                return Ok(Equal);
            }
            Ok(match side(a.1, b.1) {
                Some(Sign::Positive) => Less,
                Some(Sign::Negative) => Greater,
                Some(Sign::Zero) => Equal,
                None => return Err(TouchUnread::PieceInBand),
            })
        };
    let mut sorted: Vec<(Turn, Point3<T>)> = Vec::with_capacity(points.len());
    for &q in &points {
        let item = (turn(q)?, q);
        let mut slot = sorted.len();
        while slot > 0 && order(sorted[slot - 1], item)? == Greater {
            slot -= 1;
        }
        sorted.insert(slot, item);
    }
    // Where the face meets `p` more than once, a corner's sector runs
    // from its outgoing ray to the first incoming ray of ANY pass
    // counterclockwise from it: the face's interior lies left of the
    // outgoing edge until the loop next reaches `p`. (A loop through two
    // vertices on one point, a pinch, returns to `p` on another lobe's
    // edge, so its own pass's incoming edge is not where it ends.)
    let end = match (at, end) {
        (PieceAt::Corner(..), Some(own)) if !also.is_empty() => {
            let mut best = (turn(own)?, own);
            for &(rr, kk) in also {
                let n = rings[rr].len();
                let q = rings[rr][(kk + n - 1) % n];
                let item = (turn(q)?, q);
                if order(item, best)? == Less {
                    best = item;
                }
            }
            Some(best.1)
        }
        _ => end,
    };
    // Only the turns inside the piece's sector.
    if let Some(end) = end {
        let last = (turn(end)?, end);
        let mut kept = Vec::with_capacity(sorted.len());
        for item in sorted {
            if order(item, last)? != Greater {
                kept.push(item);
            }
        }
        sorted = kept;
    }
    // One direction per distinct turn.
    let mut rays: Vec<(Turn, Point3<T>)> = Vec::new();
    for item in sorted {
        match rays.last() {
            Some(&last) if order(last, item)? == Equal => {}
            _ => rays.push(item),
        }
    }
    let mut gaps: Vec<(Point3<T>, Point3<T>)> = rays.windows(2).map(|w| (w[0].1, w[1].1)).collect();
    if end.is_none() {
        match (rays.first(), rays.last()) {
            (Some(&first), Some(&last)) if rays.len() > 1 => gaps.push((last.1, first.1)),
            _ => return Err(TouchUnread::Unanalysed),
        }
    }
    if gaps.is_empty() {
        return Err(TouchUnread::Unanalysed);
    }
    let mut piece = Vec::with_capacity(2 * gaps.len());
    for (qa, qb) in gaps {
        // Under half a turn apart, so the bisector lies between them.
        match side(qa, qb) {
            Some(Sign::Positive) => {}
            Some(_) => return Err(TouchUnread::Unanalysed),
            None => return Err(TouchUnread::PieceInBand),
        }
        // Empty: no vertex strictly left of `ua` and right of `ub`.
        for &q in &points {
            match (side(qa, q), side(qb, q)) {
                (Some(Sign::Negative | Sign::Zero), _) | (_, Some(Sign::Positive | Sign::Zero)) => {
                }
                _ => return Err(TouchUnread::PieceInBand),
            }
        }
        // The bisecting ray, as a point at the nearer bound's reach along
        // it: the scale its two bounds fix it at.
        let probe = (unit(qa) + unit(qb)).normalize();
        let pr = p + probe * ((qa - p).norm().min((qb - p).norm()) * unit(qa).dot(probe));
        let front = |q: Point3<T>| ahead(p, pr, q, rd);
        // The edge the probe meets first, with the point it meets it at.
        let mut first: Option<(Segment<T>, Point3<T>)> = None;
        for &(x, y) in &edges {
            let (sx, sy) = (side(pr, x), side(pr, y));
            if matches!(
                (sx, sy),
                (Some(Sign::Positive), Some(Sign::Positive))
                    | (Some(Sign::Negative), Some(Sign::Negative))
            ) {
                continue;
            }
            // Wholly behind `p` along the probe: it never meets the ray.
            if front(x) == Some(Sign::Negative) && front(y) == Some(Sign::Negative) {
                continue;
            }
            let cross = match (sx, sy) {
                (None, _) | (_, None) | (Some(Sign::Zero), Some(Sign::Zero)) => {
                    return Err(TouchUnread::PieceInBand);
                }
                (Some(Sign::Zero), _) => x,
                (_, Some(Sign::Zero)) => y,
                _ => ray_meets(p, m, pr, (x, y)),
            };
            match front(cross) {
                Some(Sign::Positive) => {}
                Some(Sign::Negative) => continue,
                _ => return Err(TouchUnread::PieceInBand),
            }
            first = match first {
                None => Some(((x, y), cross)),
                Some((e, c)) => {
                    match sign(TOUCH_PIECE_REACH, Distance::of(cross, c, probe), band) {
                        Some(Sign::Negative) => Some(((x, y), cross)),
                        Some(_) => Some((e, c)),
                        None => return Err(TouchUnread::PieceInBand),
                    }
                }
            };
        }
        let ((x, y), _) = first.ok_or(TouchUnread::Unanalysed)?;
        // Where the ray through `u` meets that edge: on it, each end
        // read along the edge, or the piece does not build.
        let along = (y - x).normalize();
        let meet = |r: Point3<T>| -> Result<Point3<T>, TouchUnread> {
            let q = ray_meets(p, m, r, (x, y));
            let past_x = sign(TOUCH_PIECE_MEET, Distance::of(q, x, along), band);
            let past_y = sign(TOUCH_PIECE_MEET, Distance::of(q, y, -along), band);
            match (past_x, past_y) {
                (Some(Sign::Positive | Sign::Zero), Some(Sign::Positive | Sign::Zero)) => Ok(q),
                _ => Err(TouchUnread::PieceInBand),
            }
        };
        piece.push(meet(qa)?);
        piece.push(meet(qb)?);
    }
    Ok(piece)
}

/// Which side of the ray from `p` through `r` the point `x` lies on, in
/// the plane of the face whose outward normal is `m`: Positive to the
/// ray's left about `m`. A ray is known by its real point `r`, so `x` is
/// read no further out than `r` ([`within`]): past it the ray would be
/// carried beyond what fixes it.
fn wedge_side<T: Decide>(
    p: Point3<T>,
    m: Vec3<T>,
    r: Point3<T>,
    x: Point3<T>,
    rd: Reading<T>,
) -> Option<Sign> {
    let x = within(p, x, (r - p).norm());
    metric::sign_over(
        TOUCH_WEDGE,
        Distance::of(x, p, m.cross((r - p).normalize())),
        rd,
    )
}

/// Whether `x` lies ahead of `p` along the ray through `r` or behind it,
/// read no further out than `r` (as [`wedge_side`]).
fn ahead<T: Decide>(p: Point3<T>, r: Point3<T>, x: Point3<T>, rd: Reading<T>) -> Option<Sign> {
    let x = within(p, x, (r - p).norm());
    metric::sign_over(TOUCH_AHEAD, Distance::of(x, p, (r - p).normalize()), rd)
}

/// `q` drawn in toward `p` along its ray to lie within `r` of it — on a
/// wedge's ray or inside it, a point of the same face.
fn within<T: Real>(p: Point3<T>, q: Point3<T>, r: T) -> Point3<T> {
    p + (q - p) * (r / (q - p).norm()).min(T::from_f64(1.0))
}

/// The point of the segment `x`–`y` where a quantity affine along it,
/// `ox` at `x` and `oy` at `y`, vanishes — a construction, read only
/// where the quantity is decided to change sign between them, or to
/// bound where it could.
fn on_plane<T: Real>(x: Point3<T>, y: Point3<T>, ox: T, oy: T) -> Point3<T> {
    x + (y - x) * (ox / (ox - oy))
}

/// Where the line from `p` along the ray through `r`, in the face of
/// outward normal `m`, meets the segment `a`–`b` of that face.
fn ray_meets<T: Real>(p: Point3<T>, m: Vec3<T>, r: Point3<T>, (a, b): Segment<T>) -> Point3<T> {
    let n = m.cross((r - p).normalize());
    on_plane(a, b, n.dot(a - p), n.dot(b - p))
}

/// Whether `x` lies on the ray from `c` through `q`, in the face of
/// outward normal `m`, on decided readings: within zero of the face's
/// plane and of the ray's line in it, and ahead of `c`, each read no
/// further out than `q` ([`Link::closes`]'s reading of a bound on a
/// bound).
fn on_ray<T: Decide>(c: Point3<T>, m: Vec3<T>, q: Point3<T>, x: Point3<T>, rd: Reading<T>) -> bool {
    let near = within(c, x, (q - c).norm());
    metric::sign_over(TOUCH_PLANE, Distance::of(near, c, m), rd) == Some(Sign::Zero)
        && wedge_side(c, m, q, x, rd) == Some(Sign::Zero)
        && ahead(c, q, x, rd) == Some(Sign::Positive)
}

impl<T: Decide> Sector<T> {
    /// One planar, line-bounded face as its sectors at `p`, held there
    /// as `holds` say: inside it or inside one edge of it (one sector,
    /// about `p`), or at its corners — every place its loops pass
    /// through a vertex the holds name, each its own sector about that
    /// vertex, whether the loops pass one vertex twice or two vertices
    /// on one point (a pinch, one vertex per cone). A face held inside
    /// two of its edges, or both at a corner and inside an edge, touches
    /// itself there in a way no sector reads, and is not read. A corner
    /// hold beside an inside hold reads the corner, and an edge hold
    /// beside one reads the edge: the face's interior holds no point of
    /// its boundary.
    pub(crate) fn of_face(
        face: &Planar<'_, T>,
        p: Point3<T>,
        holds: &[Hold],
        band: Band,
    ) -> Result<Vec<Self>, TouchUnread> {
        let corner = |v: VertexKey| holds.contains(&Hold::Corner(v));
        // Every chord from `p` a number (the point's own corners aside).
        for (&v, &q) in face.loops.iter().flatten().zip(face.rings.iter().flatten()) {
            if !corner(v) {
                chord(p, q)?;
            }
        }
        // Where the loops hold the point: each a loop position, with the
        // point it is read about.
        let mut corners = Vec::new();
        let mut edges = Vec::new();
        for (r, ring) in face.loops.iter().enumerate() {
            let n = ring.len();
            for k in 0..n {
                if corner(ring[k]) {
                    corners.push(((r, k), face.rings[r][k]));
                }
                for &h in holds {
                    if let Hold::Edge(a, b) = h
                        && ((ring[k] == a && ring[(k + 1) % n] == b)
                            || (ring[k] == b && ring[(k + 1) % n] == a))
                    {
                        edges.push((r, k));
                    }
                }
            }
        }
        let inside = holds.contains(&Hold::Inside);
        let sector = |at: Point3<T>, place: PieceAt, also: &[(usize, usize)]| {
            let piece = face_piece(at, face.outward, &face.rings, place, also, band)?;
            Ok(Self {
                surface: face.surface.clone(),
                sense: face.sense,
                outward: face.outward,
                apex: at,
                wedges: piece.chunks_exact(2).map(|w| (w[0], w[1])).collect(),
                closed: matches!(place, PieceAt::Inside),
            })
        };
        let named_corners = holds
            .iter()
            .filter(|h| matches!(h, Hold::Corner(_)))
            .count();
        match (&corners[..], &edges[..], inside) {
            // A corner tie that is a face tie too names the touch at the
            // corner: the face's interior holds no point of its boundary.
            ([_, ..], [], _) => corners
                .iter()
                .map(|&(at, q)| {
                    let also: Vec<_> = corners.iter().map(|c| c.0).filter(|&c| c != at).collect();
                    sector(q, PieceAt::Corner(at.0, at.1), &also)
                })
                .collect(),
            ([], &[(r, k)], _) => Ok(vec![sector(p, PieceAt::Edge(r, k), &[])?]),
            ([], [], true) if named_corners == 0 => Ok(vec![sector(p, PieceAt::Inside, &[])?]),
            ([], [], _) => Err(TouchUnread::Corrupt),
            _ => Err(TouchUnread::Unanalysed),
        }
    }

    /// The sector's bounds, where it has them: real points on its first
    /// ray and its last.
    fn bounds(&self) -> Option<[Point3<T>; 2]> {
        match (self.closed, self.wedges.first(), self.wedges.last()) {
            (false, Some(first), Some(last)) => Some([first.0, last.1]),
            _ => None,
        }
    }

    /// **Where `x`, on the sector's plane, lies against the sector**, in
    /// its plane: each wedge `(p, a, b)` read by `x`'s side of its two
    /// rays (a wedge is under half a turn, so strictly left of `a` and
    /// right of `b` is strictly inside). `None` where a reading the
    /// answer needs is in band.
    fn place(&self, p: Point3<T>, x: Point3<T>, rd: Reading<T>) -> Option<Place> {
        let m = self.outward;
        let last = self.wedges.len().saturating_sub(1);
        let (mut in_band, mut bound) = (false, false);
        for (k, &(a, b)) in self.wedges.iter().enumerate() {
            // On a ray: a bound of the sector, or a ray two wedges share.
            let on = |outer: bool, ahead: Option<Sign>| match ahead {
                Some(Sign::Positive) if outer => Some(Place::Bound),
                Some(Sign::Positive) => Some(Place::Inside),
                _ => None,
            };
            let got = match (wedge_side(p, m, a, x, rd), wedge_side(p, m, b, x, rd)) {
                (Some(Sign::Negative), _) | (_, Some(Sign::Positive)) => continue,
                (Some(Sign::Positive), Some(Sign::Negative)) => Some(Place::Inside),
                (Some(Sign::Zero), Some(Sign::Negative)) => {
                    on(k == 0 && !self.closed, ahead(p, a, x, rd))
                }
                (Some(Sign::Positive), Some(Sign::Zero)) => {
                    on(k == last && !self.closed, ahead(p, b, x, rd))
                }
                _ => None,
            };
            match got {
                Some(Place::Inside) => return Some(Place::Inside),
                Some(Place::Bound) => bound = true,
                Some(Place::Outside) => {}
                None => in_band = true,
            }
        }
        match (in_band, bound) {
            (true, _) => None,
            (false, true) => Some(Place::Bound),
            (false, false) => Some(Place::Outside),
        }
    }
}

impl<T: Decide> Sector<T> {
    /// Whether the segment `x0`–`x1` (seen from `p`, an arc) decidedly
    /// misses the sector: for every wedge both ends lie strictly beyond
    /// one of its rays, a half-space through `p` that then holds the
    /// whole arc.
    fn misses(&self, p: Point3<T>, (x0, x1): Segment<T>, rd: Reading<T>) -> bool {
        let m = self.outward;
        let beyond = |r: Point3<T>, sign: Sign| {
            [x0, x1]
                .iter()
                .all(|&x| wedge_side(p, m, r, x, rd) == Some(sign))
        };
        self.wedges
            .iter()
            .all(|&(a, b)| beyond(a, Sign::Negative) || beyond(b, Sign::Positive))
    }
}

impl<T: Decide> Link<T> {
    /// A link from its sectors.
    pub(crate) fn of(p: Point3<T>, sectors: Vec<Sector<T>>) -> Result<Self, TouchUnread> {
        if sectors.is_empty() {
            return Err(TouchUnread::Corrupt);
        }
        let wedges = || sectors.iter().flat_map(|s| &s.wedges);
        let reach = wedges()
            .flat_map(|&(a, b)| [a, b])
            .map(|q| (q - p).norm())
            .fold(T::zero(), |r, l| r.max(l));
        // The nearest point of each far side: its foot from `p`, held to
        // the segment.
        let ball = wedges()
            .map(|&(a, b)| {
                let ab = b - a;
                let t = ((p - a).dot(ab) / ab.dot(ab))
                    .max(T::zero())
                    .min(T::from_f64(1.0));
                (a + ab * t - p).norm()
            })
            .fold(reach, |r, l| r.min(l));
        let tie = sectors
            .iter()
            .map(|s| (s.apex - p).norm())
            .fold(T::zero(), |r, l| r.max(l));
        Ok(Self {
            p,
            sectors,
            reach,
            ball,
            tie,
        })
    }

    /// `q`'s side of sector `s`'s plane through `p`.
    fn side(&self, s: &Sector<T>, q: Point3<T>, rd: Reading<T>) -> Option<Sign> {
        metric::sign_over(TOUCH_PLANE, Distance::of(q, self.p, s.outward), rd)
    }

    /// **Whether the link closes**: every sector bound is the bound of an
    /// even number of the link's sectors, counted with its own, so the
    /// sectors bound a region of directions and crossing-parity reads
    /// it ([`Link::reads`]). Gathering makes it so — every edge at a
    /// corner is two faces' at that corner, an edge tie brings both its
    /// faces, a face tie is a closed disc — and this holds that a body
    /// whose faces at the point do not pair up is refused, not read.
    /// `None` where a bound's reading is in band.
    fn closes(&self, rd: Reading<T>) -> Option<bool> {
        let p = self.p;
        let bounds: Vec<(Vec3<T>, Point3<T>)> = self
            .sectors
            .iter()
            .filter_map(|s| Some(s.bounds()?.map(|q| (s.outward, q))))
            .flatten()
            .collect();
        for &(m, q) in &bounds {
            let mut count = 0usize;
            for &(_, x) in &bounds {
                // On the ray: within zero of its plane and of its line in
                // that plane, and ahead. Any one reading decided otherwise
                // puts `x` off it, whatever the others read. All three
                // ask one thing about the ray, which its real point `q`
                // fixes, so `x` is drawn in to `q`'s reach for each, the
                // plane's too ([`within`]). [`Link::side`] reads a point
                // against a face's plane, a surface, at its own radius.
                let readings = [
                    metric::sign_over(
                        TOUCH_PLANE,
                        Distance::of(within(p, x, (q - p).norm()), p, m),
                        rd,
                    )
                    .map(|s| s == Sign::Zero),
                    wedge_side(p, m, q, x, rd).map(|s| s == Sign::Zero),
                    ahead(p, q, x, rd).map(|s| s == Sign::Positive),
                ];
                if readings.contains(&Some(false)) {
                    continue;
                }
                if readings.contains(&None) {
                    return None;
                }
                count += 1;
            }
            if count % 2 == 1 {
                return Some(false);
            }
        }
        Some(true)
    }

    /// The link's rays: every sector's bounds.
    fn rays(&self) -> Vec<Point3<T>> {
        self.sectors
            .iter()
            .filter_map(Sector::bounds)
            .flatten()
            .collect()
    }

    /// **Sector `s`'s pieces**: each wedge cut at every ray in `rays`
    /// lying strictly inside it, each piece as the two real points of the
    /// face its bounding rays meet the wedge's far side at. A ray on the
    /// plane is decided inside the wedge, on one of its rays, or outside;
    /// one in band of the plane, outside the wedge or the piece refuses.
    fn pieces(
        &self,
        s: &Sector<T>,
        rays: &[Point3<T>],
        rd: Reading<T>,
    ) -> Result<Vec<Segment<T>>, TouchUnread> {
        use Sign::{Negative, Positive, Zero};
        let (p, m) = (self.p, s.outward);
        let mut out = Vec::new();
        for &(a, b) in &s.wedges {
            let mut cuts: Vec<Point3<T>> = Vec::new();
            for &r in rays {
                let sides = (wedge_side(p, m, a, r, rd), wedge_side(p, m, b, r, rd));
                let outside = matches!(sides, (Some(Negative), _) | (_, Some(Positive)));
                let inside = match self.side(s, r, rd) {
                    Some(Zero) => match sides {
                        (Some(Positive), Some(Negative)) => true,
                        (Some(Zero), Some(Negative)) | (Some(Positive), Some(Zero)) => false,
                        _ if outside => false,
                        _ => return Err(TouchUnread::InBand),
                    },
                    Some(_) => false,
                    None if outside => false,
                    None => return Err(TouchUnread::InBand),
                };
                if !inside {
                    continue;
                }
                let c = ray_meets(p, m, r, (a, b));
                // In turn from `a`: after every cut it is left of.
                let mut slot = cuts.len();
                while slot > 0 {
                    match wedge_side(p, m, cuts[slot - 1], c, rd) {
                        Some(Positive) => break,
                        Some(Negative) => slot -= 1,
                        Some(Zero) => {
                            slot = usize::MAX;
                            break;
                        }
                        None => return Err(TouchUnread::InBand),
                    }
                }
                if slot != usize::MAX {
                    cuts.insert(slot, c);
                }
            }
            let mut ends = vec![a];
            ends.extend(cuts);
            ends.push(b);
            out.extend(ends.windows(2).map(|w| (w[0], w[1])));
        }
        Ok(out)
    }

    /// **Where the direction to `q` lies against the link's material**,
    /// by arc crossing-parity, no convexity assumed. `q` on a sector's
    /// plane inside it is [`Material::On`] that sector. Otherwise `q` is
    /// read along the minor arc to a reference `w`, a wedge's centroid on
    /// one sector `T` carried out to `q`'s distance (planes are read at
    /// one scale; rays at their own, [`wedge_side`]), where the link's
    /// material is the side behind `T`:
    /// the material holds `q` iff `q` lies behind `T` and the arc crosses
    /// the other sectors an even number of times, or in front and odd.
    /// The arc is the segment `q`–`w` seen from `p`, under half a turn
    /// (both ends ahead of the plane normal to their bisector), so it
    /// meets each plane through `p` at most once and `T`'s only at `w`.
    /// Each other sector is read from its plane's sides at `q` and `w`:
    /// on one side, not crossed; on opposite sides, crossed iff the
    /// segment's point on the plane lies inside the sector; one end on
    /// the plane (within `zero`), not crossed iff the stretch of the arc
    /// where it could then meet the plane misses the sector
    /// ([`Sector::misses`]). A reference
    /// any reading of which is in band, on a sector's bound, or on a
    /// second sector at `w` is passed over for the next; `None` where
    /// none reads, or where `q` itself is in band of a plane or on a
    /// bound.
    pub(crate) fn reads_over(&self, q: Point3<T>, rd: Reading<T>) -> Option<Material> {
        let p = self.p;
        // `q`'s strict side of each plane; `None` for a plane `q` lies
        // on outside its sector, which no arc from `q` crosses inside it.
        let mut sides = Vec::with_capacity(self.sectors.len());
        for (k, s) in self.sectors.iter().enumerate() {
            sides.push(match self.side(s, q, rd)? {
                Sign::Zero => match s.place(p, q, rd)? {
                    Place::Inside => return Some(Material::On(k)),
                    Place::Bound => return None,
                    Place::Outside => None,
                },
                sign => Some(sign),
            });
        }
        for (t, s0) in self.sectors.iter().enumerate() {
            let Some(base) = sides[t] else { continue };
            'reference: for &(a, b) in &s0.wedges {
                // The wedge's centroid, carried along its ray to `q`'s
                // distance: a plane is a surface, read at one scale for
                // both ends of the arc.
                let c = (a - p) + (b - p);
                let w = p + c * ((q - p).norm() / c.norm());
                let half = ((q - p).normalize() + (w - p).normalize()).normalize();
                for x in [q, w] {
                    if metric::sign_over(TOUCH_ARC, Distance::of(x, p, half), rd)
                        != Some(Sign::Positive)
                    {
                        continue 'reference;
                    }
                }
                let mut held = base == Sign::Negative;
                for (k, s) in self.sectors.iter().enumerate() {
                    if k == t {
                        continue;
                    }
                    let Some(at_w) = self.side(s, w, rd) else {
                        continue 'reference;
                    };
                    let (oq, ow) = (s.outward.dot(q - p), s.outward.dot(w - p));
                    // A Zero reads within the band's zero, the slack added.
                    let zero = T::from_computed(rd.band.zero()) + rd.slack();
                    let at = |t: T| q + (w - q) * t;
                    // An end read Zero lies within `zero` of the plane on
                    // either side, so a crossing, if any, lies on the
                    // stretch of the arc where the plane can then be met:
                    // read as not crossed only where that whole stretch
                    // misses the sector.
                    let crossed = match (sides[k], at_w) {
                        (None, Sign::Zero) => continue 'reference,
                        (None, sw) => {
                            let ow = if sw == Sign::Positive { ow } else { -ow };
                            if !s.misses(p, (q, at(zero / (zero + ow))), rd) {
                                continue 'reference;
                            }
                            false
                        }
                        (Some(sq), Sign::Zero) => {
                            let z = if sq == Sign::Positive { zero } else { -zero };
                            if !s.misses(p, (at(oq / (oq + z)), w), rd) {
                                continue 'reference;
                            }
                            false
                        }
                        (Some(at_q), at_w) if at_q == at_w => false,
                        (Some(_), _) => match s.place(p, on_plane(q, w, oq, ow), rd) {
                            Some(Place::Inside) => true,
                            Some(Place::Outside) => false,
                            _ => continue 'reference,
                        },
                    };
                    held ^= crossed;
                }
                return Some(if held { Material::In } else { Material::Out });
            }
        }
        None
    }
}

/// **One planar, line-bounded face as [`Sector::of_face`] reads it**:
/// its surface and sense, its outward unit normal, and its loops as
/// vertex keys and as points, each walked with the face's interior on
/// its LEFT about the outward normal (the outer loop counterclockwise,
/// each hole clockwise).
pub(crate) struct Planar<'a, T: Real> {
    pub(crate) surface: geom::Surface<T>,
    /// `Face::sense`.
    pub(crate) sense: bool,
    pub(crate) outward: Vec3<T>,
    pub(crate) loops: &'a [Vec<VertexKey>],
    pub(crate) rings: Vec<Vec<Point3<T>>>,
}

/// Vertex `v`'s position in `body`.
fn position<T: Real>(body: &Body<T>, v: VertexKey) -> Result<Point3<T>, TouchUnread> {
    let point = body.vertices.get(v).ok_or(TouchUnread::Corrupt)?.point;
    body.points.get(point).copied().ok_or(TouchUnread::Corrupt)
}

impl<T: Decide> Sector<T> {
    /// Face `f` of a live `body` as its sectors at `p`, held there as
    /// `holds` say ([`Sector::of_face`]): a face off a plane, or one with
    /// an edge that is not a line, is [`TouchUnread::Unreadable`].
    fn of_body_face(
        body: &Body<T>,
        f: FaceKey,
        p: Point3<T>,
        holds: &[Hold],
        band: Band,
    ) -> Result<Vec<Self>, TouchUnread> {
        let face = body.get_face(f).ok_or(TouchUnread::Corrupt)?;
        let surface = body
            .get_surface(face.surface)
            .cloned()
            .ok_or(TouchUnread::Corrupt)?;
        if !matches!(surface, geom::Surface::Plane { .. }) {
            return Err(TouchUnread::Unreadable);
        }
        let outward = crate::face_normal::face_outward_normal(body, f)
            .ok_or(TouchUnread::Unreadable)?
            .vec();
        let mut loops = Vec::new();
        let mut rings = Vec::new();
        for cycle in crate::census::face_cycles(body, face) {
            let cycle = cycle.map_err(|_| TouchUnread::Corrupt)?;
            let mut keys = Vec::with_capacity(cycle.len());
            let mut ring = Vec::with_capacity(cycle.len());
            for he in cycle {
                let he = body.half_edges.get(he).ok_or(TouchUnread::Corrupt)?;
                if !crate::census::edge_is_line(body, he.edge) {
                    return Err(TouchUnread::Unreadable);
                }
                keys.push(he.start);
                ring.push(position(body, he.start)?);
            }
            loops.push(keys);
            rings.push(ring);
        }
        let face = Planar {
            surface,
            sense: face.sense,
            outward,
            loops: &loops,
            rings,
        };
        Self::of_face(&face, p, holds, band)
    }
}

impl<T: Decide> Link<T> {
    /// **The link at vertex `v` of `body`**: every face around the
    /// vertex, a sector at each place its loops pass through `v`.
    ///
    /// # Errors
    ///
    /// The [`TouchUnread`] a sector's build refused with: a curved face
    /// or edge at the vertex is [`TouchUnread::Unreadable`].
    pub fn at_point(body: &Body<T>, v: VertexKey, band: Band) -> Result<Self, TouchUnread> {
        let p = position(body, v)?;
        let mut sectors = Vec::new();
        for f in body.faces_of_vertex(v).ok_or(TouchUnread::Corrupt)? {
            sectors.extend(Sector::of_body_face(body, f, p, &[Hold::Corner(v)], band)?);
        }
        Self::of(p, sectors)
    }

    /// **The link at a point `p` inside the line edge `e` of `body`**:
    /// the edge's two faces, each a half-plane about the edge's line.
    /// `p` is the caller's to place strictly inside the edge.
    ///
    /// # Errors
    ///
    /// As [`Link::at_point`]; an edge that is not a line is
    /// [`TouchUnread::Unreadable`].
    pub fn on_edge(
        body: &Body<T>,
        e: EdgeKey,
        p: Point3<T>,
        band: Band,
    ) -> Result<Self, TouchUnread> {
        if !crate::census::edge_is_line(body, e) {
            return Err(TouchUnread::Unreadable);
        }
        let he = body.edges.get(e).ok_or(TouchUnread::Corrupt)?.he_plus;
        let v0 = body.half_edges.get(he).ok_or(TouchUnread::Corrupt)?.start;
        let v1 = body.half_edge_end(he).ok_or(TouchUnread::Corrupt)?;
        let (f_plus, f_minus) = crate::readback::edge_sides(body, e)
            .map_err(|_| TouchUnread::Corrupt)?
            .faces();
        let mut keys = vec![f_plus];
        if f_minus != f_plus {
            keys.push(f_minus);
        }
        let mut sectors = Vec::new();
        for f in keys {
            sectors.extend(Sector::of_body_face(
                body,
                f,
                p,
                &[Hold::Edge(v0, v1)],
                band,
            )?);
        }
        Self::of(p, sectors)
    }

    /// **The link at a point `p` inside the planar face `f` of `body`**:
    /// that face, a closed disc about `p`. `p` is the caller's to place
    /// strictly inside the face.
    ///
    /// # Errors
    ///
    /// As [`Link::at_point`].
    pub fn in_face(
        body: &Body<T>,
        f: FaceKey,
        p: Point3<T>,
        band: Band,
    ) -> Result<Self, TouchUnread> {
        Self::of(p, Sector::of_body_face(body, f, p, &[Hold::Inside], band)?)
    }

    /// The point the link is about.
    pub fn point(&self) -> Point3<T> {
        self.p
    }

    /// **Where the direction to `q` lies against the link's material**
    /// ([`Link::reads_over`] on `band` alone); `None` where a reading it
    /// needs is in band.
    pub fn reads(&self, q: Point3<T>, band: Band) -> Option<Material> {
        self.reads_over(q, Reading { band, slack: None })
    }

    /// **`q`'s signed distance from the cone's boundary**, in metres:
    /// negative where [`Link::reads`] says In, the distance itself where
    /// it says Out or On. The cone is the link's sectors carried out
    /// from their apexes without end, so this is a statement about the
    /// material near the point, not about the faces' far extent.
    ///
    /// The distance is a closed form over the sectors
    /// ([`Link::distance`]), and the sign is the parity reading's. Where
    /// that reading is in band but `q` lies on a sector within the band
    /// ([`Link::touches`]), membership does not matter at the band's
    /// resolution and the distance is the answer.
    ///
    /// # Errors
    ///
    /// [`TouchUnread::InBand`] where the parity reading is in band and
    /// `q` stands off every sector: its magnitude is definite and its
    /// sign is not.
    pub fn offset(&self, q: Point3<T>, band: Band) -> Result<T, TouchUnread> {
        let rd = Reading { band, slack: None };
        let d = self.distance(q, rd).ok_or(TouchUnread::Corrupt)?;
        match self.reads_over(q, rd) {
            Some(Material::In) => Ok(-d),
            Some(Material::Out | Material::On(_)) => Ok(d),
            None if self.touches(q, rd) => Ok(d),
            None => Err(TouchUnread::InBand),
        }
    }

    /// **The lowest [`Link::offset`] along the segment `x0`–`x1`**, over
    /// a finite closed-form candidate set, with no sampling.
    ///
    /// Along the segment, `x(t) = x0 + t·(x1 − x0)`, each piece of the
    /// cone's boundary (a sector's plane, an apex, a ray's line) is at a
    /// squared distance quadratic in `t`, and the distance to the
    /// boundary is the least of those over the pieces that apply. Off
    /// the material the offset is that distance, lowest at an end, at
    /// some piece's nearest point (a plane's nearest is its crossing),
    /// or where the boundary is met; in it the offset is minus the
    /// distance, lowest at an end or where two pieces stand equally far.
    /// So the candidates are the ends, each piece's nearest point and
    /// every root in `[0, 1]` of two pieces' squared distances equated,
    /// each root in both of its forms so that one of them is stable. A
    /// ray two sectors share is one piece ([`on_ray`]), and the apex is
    /// the link's point.
    /// A candidate is a real point of the segment, read in full by
    /// [`Link::offset`]; one that does not apply only reads higher.
    ///
    /// # Errors
    ///
    /// The first refusal of [`Link::offset`] at a candidate.
    pub fn lowest_on(
        &self,
        (x0, x1): (Point3<T>, Point3<T>),
        band: Band,
    ) -> Result<T, TouchUnread> {
        let e = x1 - x0;
        // Each piece's squared distance from x(t), as [t², t, 1]
        // coefficients.
        let mut forms: Vec<[T; 3]> = Vec::new();
        let two = T::from_f64(2.0);
        let rd = Reading { band, slack: None };
        // Each ray once: an edge's ray is a bound of both its faces'
        // sectors, and two near copies of one form would only add roots
        // of their rounding.
        let mut kept: Vec<(Point3<T>, Vec3<T>, Point3<T>)> = Vec::new();
        // The apex once, at the link's point: every sector stands on it,
        // or within the link's tie of it.
        let w = x0 - self.p;
        forms.push([e.norm_squared(), two * w.dot(e), w.norm_squared()]);
        for s in &self.sectors {
            let w = x0 - s.apex;
            let (o0, o1) = (s.outward.dot(w), s.outward.dot(e));
            forms.push([o1.powi(2), two * o0 * o1, o0.powi(2)]);
            let last = s.wedges.len().saturating_sub(1);
            for (k, &(a, b)) in s.wedges.iter().enumerate() {
                let rays = if k == last { vec![a, b] } else { vec![a] };
                for r in rays {
                    if kept.iter().any(|&(c, m, q)| on_ray(c, m, q, r, rd)) {
                        continue;
                    }
                    kept.push((s.apex, s.outward, r));
                    let u = (r - s.apex).normalize();
                    let (wp, ep) = (w - u * u.dot(w), e - u * u.dot(e));
                    forms.push([ep.norm_squared(), two * wp.dot(ep), wp.norm_squared()]);
                }
            }
        }
        let mut ts = vec![T::zero(), T::one()];
        for f in &forms {
            ts.push(-f[1] / (two * f[0]));
        }
        for (i, f) in forms.iter().enumerate() {
            for g in &forms[i + 1..] {
                let (a, b, c) = (f[0] - g[0], f[1] - g[1], f[2] - g[2]);
                let r = (b.powi(2) - T::from_f64(4.0) * a * c).max(T::zero()).sqrt();
                ts.extend([
                    (-b + r) / (two * a),
                    (-b - r) / (two * a),
                    two * c / (-b + r),
                    two * c / (-b - r),
                ]);
            }
        }
        let mut lowest: Option<T> = None;
        for t in ts {
            if t.is_poison() || !t.is_bounded() {
                continue;
            }
            let t = t.max(T::zero()).min(T::one());
            let at = self.offset(x0 + e * t, band)?;
            lowest = Some(lowest.map_or(at, |l| l.min(at)));
        }
        lowest.ok_or(TouchUnread::Corrupt)
    }

    /// **`q`'s distance from the cone's boundary**: the least, over every
    /// wedge `(c, a, b)` of every sector, of `q`'s distance from the
    /// wedge's two rays from `c` and, where `q` stands over the wedge
    /// (not decidedly beyond either ray, [`wedge_side`]), from its plane.
    /// A wedge is convex, so off its rays its nearest point is the foot
    /// on its plane exactly where `q` stands over it; a reading in band
    /// takes the plane too, which is the nearer by at most the band.
    /// `None` for a link with no wedge.
    fn distance(&self, q: Point3<T>, rd: Reading<T>) -> Option<T> {
        let mut least: Option<T> = None;
        for s in &self.sectors {
            let (c, m) = (s.apex, s.outward);
            let ray = |r: Point3<T>| {
                let (u, v) = ((r - c).normalize(), q - c);
                (v - u * u.dot(v).max(T::zero())).norm()
            };
            for &(a, b) in &s.wedges {
                let mut d = ray(a).min(ray(b));
                let beyond = wedge_side(c, m, a, q, rd) == Some(Sign::Negative)
                    || wedge_side(c, m, b, q, rd) == Some(Sign::Positive);
                if !beyond {
                    let o = m.dot(q - c);
                    d = d.min(o.max(-o));
                }
                least = Some(least.map_or(d, |l| l.min(d)));
            }
        }
        least
    }

    /// **Whether `q` lies on a sector within the band**: on its plane
    /// ([`Link::side`] Zero) and not decidedly outside it
    /// ([`Sector::place`]).
    fn touches(&self, q: Point3<T>, rd: Reading<T>) -> bool {
        self.sectors.iter().any(|s| {
            self.side(s, q, rd) == Some(Sign::Zero)
                && s.place(self.p, q, rd) != Some(Place::Outside)
        })
    }
}

/// [`Link::reads`] over a bare polygon cone at `p`, each face its
/// outward normal and one wedge's two real points, read on `band`
/// alone: how the boolean's sector fuzz runs the census's reader on its
/// own cones beside `cone_side` (`boolean::sectors::cone_fuzz`).
#[cfg(test)]
pub(crate) fn bare_cone_reads(
    p: Point3<f64>,
    faces: &[(Vec3<f64>, Point3<f64>, Point3<f64>)],
    q: Point3<f64>,
    band: Band,
) -> Option<Material> {
    let sectors = faces
        .iter()
        .map(|&(m, a, b)| Sector {
            surface: geom::Surface::Plane {
                origin: p,
                normal: m,
                u_ref: (a - p).normalize(),
            },
            sense: true,
            outward: m,
            apex: p,
            wedges: vec![(a, b)],
            closed: false,
        })
        .collect();
    let rd = Reading { band, slack: None };
    Link::of(p, sectors).ok()?.reads_over(q, rd)
}

/// **The one door a rest comes through.** [`Rest`](rest::Rest) has a
/// private field and is neither `Copy` nor `Clone`, so nothing outside
/// this module can build one or replay one: a rest exists only where
/// [`judge`](rest::judge) ran the readings over two links and they
/// held, once.
pub(crate) mod rest {
    use super::{
        Band, Decide, Link, Material, Place, Point3, Reading, Segment, Sign, TouchUnread,
        TouchVerdict,
    };
    use super::{TOUCH_NORMAL, ahead, on_plane, pairing, wedge_side, within};

    /// The certificate of a rest: every piece of both links read, on
    /// decided readings, as no crossing.
    #[derive(Debug, PartialEq, Eq)]
    pub(crate) struct Rest {
        _sealed: (),
    }

    /// What the readings noted on the way: the reasons the test could
    /// not certify other than a decided crossing.
    #[derive(Default)]
    struct Notes {
        in_band: bool,
        degenerate: bool,
    }

    /// **The one test of two polygonal cones** at a touch point `p`,
    /// each a solid's link ([`Link`]). Every face sector of each link is
    /// cut into PIECES at every ray of either link lying on it
    /// ([`Link::pieces`]). Then:
    ///
    /// 1. **Crossing** if a piece of one link crosses a face of the other
    ///    along a ray inside both — its two ends decidedly on opposite
    ///    sides of the face's plane, the point between them on that plane
    ///    inside the face's sector and on no ray of either link lying on
    ///    the face ([`crosses`]) — or if a piece's
    ///    probe (its middle) lies on a face of the other link inside it
    ///    with the two outward normals aligned: coplanar sectors facing
    ///    the same way, overlapping in their plane.
    /// 2. Otherwise **Crossing** if any piece's probe reads inside the
    ///    other link's material ([`Link::reads`]).
    /// 3. Otherwise **Rest**.
    ///
    /// Both links are first held to close ([`Link::closes`]), and every
    /// reading is taken over the two links' tie ([`Reading`]).
    ///
    /// **Sound.** (1): along a ray inside a piece of `S` and inside `T`'s
    /// sector off every ray lying on `T`, each material is the half-space
    /// behind its face (no other face of either link reaches the ray: on
    /// `S`'s side the piece is cut at every ray, on `T`'s the point is off
    /// them), and two half-spaces through one line overlap in a wedge; aligned coplanar faces put both
    /// materials behind one plane. (2): the probe lies inside a piece,
    /// off every face of the other link, so the direction just off it
    /// on its own material side reads as the probe does, and lies in
    /// both materials.
    ///
    /// **Complete.** Let `U` be the directions inside both materials,
    /// open on the sphere about `p`. Were it not empty, its frontier
    /// holds an arc of one link's face, `S`, say, inside some piece `P`
    /// of `S` and off the other link's faces: a frontier point on both
    /// links' faces at once lies on two faces crossing (1) or two
    /// coplanar ones, aligned (1) or opposed (then the materials lie on
    /// opposite sides there, and the point is not on the frontier). `U`
    /// lies on `S`'s material side of that arc, inside the other
    /// material. Nothing of either link meets `P`'s interior — the
    /// pieces are cut at every ray lying on them, and (1) found no face
    /// crossing them — so the directions just off all of `P` on its
    /// material side lie on one side of one face, and in one cell of the
    /// two links' arrangement: `U`'s. `P`'s probe then reads inside the
    /// other material (2).
    ///
    /// **Each verdict at its own scale.** The readings run at the
    /// pieces' real points, each link's as far out as its faces reach.
    /// A rest needs every one decided there: a Zero read at a far point
    /// is a Zero at every nearer one, so that is where the fewest
    /// coincidences are taken. A crossing is confirmed inside the BALL
    /// where both links are their solids — out to the nearest far side
    /// of any wedge of either ([`Link::ball`]) — with the piece's points
    /// drawn in along their rays ([`within`]), still real points of its
    /// face: an overlap read there is one of the two bodies, not of one
    /// link's planes carried past the other's faces. A crossing read
    /// only further out refuses in band.
    ///
    /// A crossing is claimed on decided readings only; any other
    /// reading the test needed in band, or a sector whose pieces did not
    /// cut, leaves the touch [`TouchUnread::InBand`] (or the cut's own
    /// refusal), and a probe lying on another face of its own link (a
    /// slit, or a face doubled) leaves it [`TouchUnread::Degenerate`] —
    /// never a crossing and never a rest.
    pub(crate) fn judge<T: Decide>(a: &Link<T>, b: &Link<T>, band: Band) -> TouchVerdict {
        // Every reading over the pair is taken about `p`, while each
        // sector stands on its own apex, off `p` by at most its link's
        // tie. A plane through the apex lies within the tie of the same
        // plane through `p`. A ray's line through the apex and the same
        // ray's line through `p` meet at the ray's real point `r`, and a
        // point read against them is drawn in within `|r − p|` of `p`
        // ([`within`]), so it lies at most twice that from `r`: the two
        // lines part there by at most twice the tie, and an ahead
        // reading, whose line turns with the ray, by as much. One link's
        // point read against the other's planes and rays is moved by
        // both, so twice their sum covers every reading.
        let rd = Reading {
            band,
            slack: Some((a.tie + b.tie) * T::from_f64(2.0)),
        };
        for link in [a, b] {
            match link.closes(rd) {
                Some(true) => {}
                Some(false) => return TouchVerdict::Unread(TouchUnread::Unanalysed),
                None => return TouchVerdict::Unread(TouchUnread::InBand),
            }
        }
        let mut notes = Notes::default();
        let mut refused = None;
        let ball = a.ball.min(b.ball);
        let rays: Vec<_> = a.rays().into_iter().chain(b.rays()).collect();
        let mut all: [Vec<(usize, Vec<Segment<T>>)>; 2] = [Vec::new(), Vec::new()];
        for (side, link) in [a, b].into_iter().enumerate() {
            for (k, s) in link.sectors.iter().enumerate() {
                match link.pieces(s, &rays, rd) {
                    Ok(pieces) => all[side].push((k, pieces)),
                    Err(v) => {
                        refused.get_or_insert(v);
                    }
                }
            }
        }
        for (side, (own, other)) in [(a, b), (b, a)].into_iter().enumerate() {
            let near = |q| within(own.p, q, ball);
            let cut: Vec<usize> = all[1 - side].iter().map(|(k, _)| *k).collect();
            for (k, pieces) in &all[side] {
                let s = &own.sectors[*k];
                for &(c0, c1) in pieces {
                    if crosses(other, &cut, &rays, (c0, c1), rd, &mut notes) {
                        let mut scratch = Notes::default();
                        if crosses(other, &cut, &rays, (near(c0), near(c1)), rd, &mut scratch) {
                            return TouchVerdict::Crossing;
                        }
                        notes.in_band = true;
                    }
                    let probe = c0 + (c1 - c0) * T::from_f64(0.5);
                    // On another face of its own link: a slit or a face
                    // doubled, no boundary the probe can stand on.
                    for (j, t) in own.sectors.iter().enumerate() {
                        if j == *k || own.side(t, probe, rd) != Some(Sign::Zero) {
                            continue;
                        }
                        match t.place(own.p, probe, rd) {
                            Some(Place::Outside) => {}
                            Some(Place::Inside | Place::Bound) => notes.degenerate = true,
                            None => notes.in_band = true,
                        }
                    }
                    // Every point strictly inside the piece reads its one
                    // cell, so any one that decides is the piece's
                    // reading. The far side is tried at its middle, then
                    // its quarter and sixteenth points from each end: a
                    // point stands in band of a plane only on a stretch
                    // of the side (the distance is affine along it), and
                    // these sit at five distinct places, nearer each end
                    // in turn, where a far end lifted off a plane reads
                    // most of its lift. The ladder chooses only how often
                    // a piece refuses, never what one that reads says.
                    let fractions = [
                        T::from_f64(0.5),
                        T::from_f64(0.25),
                        T::from_f64(0.75),
                        T::from_f64(0.0625),
                        T::from_f64(0.9375),
                    ];
                    let read = fractions.into_iter().find_map(|t| {
                        let q = c0 + (c1 - c0) * t;
                        other.reads_over(q, rd).map(|m| (q, m))
                    });
                    let full = read.map(|(_, m)| m);
                    let overlaps = match full {
                        Some(Material::In) => true,
                        Some(Material::Out) => false,
                        Some(Material::On(j)) => {
                            let t = &other.sectors[j];
                            match pairing(
                                TOUCH_NORMAL,
                                (&s.surface, s.sense),
                                (&t.surface, t.sense),
                                own.p,
                                own.reach.max(other.reach),
                                rd.band,
                            ) {
                                Some(geom_brep::MaterialPairing::Aligned) => true,
                                Some(geom_brep::MaterialPairing::Opposed) => false,
                                None => {
                                    notes.in_band = true;
                                    false
                                }
                            }
                        }
                        None => {
                            notes.in_band = true;
                            false
                        }
                    };
                    // Confirmed inside the ball: the same reading there.
                    if let (true, Some((q, _))) = (overlaps, read) {
                        if other.reads_over(near(q), rd) == full {
                            return TouchVerdict::Crossing;
                        }
                        notes.in_band = true;
                    }
                }
            }
        }
        if let Some(v) = refused {
            TouchVerdict::Unread(v)
        } else if notes.degenerate {
            TouchVerdict::Unread(TouchUnread::Degenerate)
        } else if notes.in_band {
            TouchVerdict::Unread(TouchUnread::InBand)
        } else {
            TouchVerdict::Rest(Rest { _sealed: () })
        }
    }

    /// Whether the piece `c0`–`c1` crosses a face of `other` along a ray
    /// inside both: its ends decidedly on opposite sides of the face's
    /// plane, and the point between them on the plane inside the face's
    /// sector ([`Sector::place`](super::Sector::place): strictly inside a wedge, or on a ray two
    /// of its wedges share) and on none of the `rays` of either link that
    /// lie on the face — a cut, where another face of one link reaches
    /// the ray and neither material is a half-space there. Only the
    /// sectors in `cut` are read (their pieces built). A reading in band
    /// notes it, unless the point is decidedly outside the face.
    fn crosses<T: Decide>(
        other: &Link<T>,
        cut: &[usize],
        rays: &[Point3<T>],
        (c0, c1): Segment<T>,
        rd: Reading<T>,
        notes: &mut Notes,
    ) -> bool {
        use Sign::{Negative, Positive, Zero};
        let p = other.p;
        for &k in cut {
            let t = &other.sectors[k];
            let sides = (other.side(t, c0, rd), other.side(t, c1, rd));
            let decided = match sides {
                (Some(Positive), Some(Negative)) | (Some(Negative), Some(Positive)) => true,
                (Some(_), Some(_)) => continue,
                _ => false,
            };
            let x = on_plane(c0, c1, t.outward.dot(c0 - p), t.outward.dot(c1 - p));
            let place = t.place(p, x, rd);
            if decided && place == Some(Place::Inside) {
                // On a cut: a ray lying on the face, the point on it.
                let mut on_cut = Some(false);
                for &r in rays {
                    let on = match other.side(t, r, rd) {
                        Some(Zero) => match wedge_side(p, t.outward, r, x, rd) {
                            Some(Zero) => ahead(p, r, x, rd).map(|a| a == Positive),
                            Some(_) => Some(false),
                            None => None,
                        },
                        Some(_) => Some(false),
                        None => None,
                    };
                    on_cut = match (on_cut, on) {
                        (Some(false), Some(false)) => Some(false),
                        (_, Some(true)) | (Some(true), _) => Some(true),
                        _ => None,
                    };
                }
                if on_cut == Some(false) {
                    return true;
                }
            }
            if place != Some(Place::Outside) {
                notes.in_band = true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic)]

    use super::*;

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).expect("the witness tolerance is a band")
    }

    fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
        Vec3::new(x, y, z)
    }

    fn close(got: f64, want: f64, what: &str) {
        assert!((got - want).abs() < 1e-12, "{what}: {got}, not {want}");
    }

    const O: Point3<f64> = Point3::new(0.0, 0.0, 0.0);

    /// **A link from bare fans about `p`**: each face its outward normal and
    /// the real points of its sector in turn counterclockwise about it, one
    /// wedge between each two consecutive points, each under half a turn —
    /// a sector bounded by its first ray and its last. For rows that state a
    /// cone by hand rather than build a body.
    fn of_fans(p: Point3<f64>, fans: &[(Vec3<f64>, Vec<Point3<f64>>)]) -> Link<f64> {
        let sectors = fans
            .iter()
            .map(|(m, pts)| {
                assert!(pts.len() >= 2, "a fan holds a wedge");
                Sector {
                    surface: geom::Surface::Plane {
                        origin: p,
                        normal: *m,
                        u_ref: (pts[0] - p).normalize(),
                    },
                    sense: true,
                    outward: *m,
                    apex: p,
                    wedges: pts.windows(2).map(|w| (w[0], w[1])).collect(),
                    closed: false,
                }
            })
            .collect();
        Link::of(p, sectors).unwrap_or_else(|why| panic!("the fans make a link: {why:?}"))
    }

    /// A block's corner at the origin, its material where `x, y, z ≤ 0`,
    /// or (`hole`) the corner of an octant hole cut into material, its
    /// material wherever any coordinate is negative.
    fn corner(hole: bool) -> Link<f64> {
        let s = if hole { 1.0 } else { -1.0 };
        let (x, y, z) = (p(s, 0.0, 0.0), p(0.0, s, 0.0), p(0.0, 0.0, s));
        of_fans(
            O,
            &[
                (v(1.0, 0.0, 0.0), vec![y, z]),
                (v(0.0, 1.0, 0.0), vec![z, x]),
                (v(0.0, 0.0, 1.0), vec![x, y]),
            ],
        )
    }

    /// The reflex top corner of an L-bracket at the origin: its material
    /// below `z = 0` and outside the notch `x, y > 0`. Its two rim edges
    /// are convex and the strut down from it concave: a saddle.
    fn saddle() -> Link<f64> {
        let (x, y, z) = (p(1.0, 0.0, 0.0), p(0.0, 1.0, 0.0), p(0.0, 0.0, -1.0));
        let (nx, ny) = (p(-1.0, 0.0, 0.0), p(0.0, -1.0, 0.0));
        of_fans(
            O,
            &[
                (v(0.0, 0.0, 1.0), vec![y, nx, ny, x]),
                (v(0.0, 1.0, 0.0), vec![x, z]),
                (v(1.0, 0.0, 0.0), vec![z, y]),
            ],
        )
    }

    /// The convex edge along `y` through the origin between the top face
    /// (`+z`) and the side face (`+x`): material where `x, z ≤ 0`.
    fn ridge() -> Link<f64> {
        let (y, ny) = (p(0.0, 1.0, 0.0), p(0.0, -1.0, 0.0));
        of_fans(
            O,
            &[
                (v(0.0, 0.0, 1.0), vec![y, p(-1.0, 0.0, 0.0), ny]),
                (v(1.0, 0.0, 0.0), vec![ny, p(0.0, 0.0, -1.0), y]),
            ],
        )
    }

    fn offset(link: &Link<f64>, q: Point3<f64>) -> f64 {
        link.offset(q, band())
            .unwrap_or_else(|why| panic!("{q:?} reads: {why:?}"))
    }

    fn lowest(link: &Link<f64>, a: Point3<f64>, b: Point3<f64>) -> f64 {
        link.lowest_on((a, b), band())
            .unwrap_or_else(|why| panic!("{a:?}–{b:?} reads: {why:?}"))
    }

    /// **A point's offset is its true distance from the cone, negative
    /// inside**, at a block's corner, a hole's and a saddle's. Red if a
    /// hole's corner reads its empty octant as material, if an inner
    /// point reads its distance from a face it does not stand over, or if
    /// the saddle refuses or reads a single face's half-space.
    #[test]
    fn an_offset_is_the_distance_from_the_cone_negative_inside() {
        let (block, hole) = (corner(false), corner(true));
        close(offset(&block, O), 0.0, "the apex");
        close(offset(&block, p(0.01, -0.02, -0.03)), 0.01, "off the block");
        close(
            offset(&block, p(-0.01, -0.02, -0.03)),
            -0.01,
            "in the block",
        );
        close(
            offset(&block, p(0.03, 0.04, -1.0)),
            0.05,
            "beside the block's x–y edge, off both faces",
        );
        close(offset(&hole, p(0.01, 0.02, 0.03)), 0.01, "in the hole");
        close(
            offset(&hole, p(0.01, -0.02, 0.03)),
            -0.02,
            "in the hole's wall",
        );
        let l = saddle();
        close(
            offset(&l, p(0.02, -0.01, -0.5)),
            -0.01,
            "in the saddle's leg",
        );
        close(
            offset(&l, p(0.02, 0.01, -0.5)),
            0.01,
            "in the saddle's notch",
        );
        close(offset(&l, p(-0.3, -0.4, 0.2)), 0.2, "over the saddle's top");
    }

    /// **The lowest offset along a segment** over a ridge: a segment
    /// crossing it in the plane `x + z = h√2` reads its distance `h` from
    /// the ridge line outside and `h/√2` inside, where the two faces stand
    /// equally deep; a segment along the ridge reads its depth; one
    /// resting on a face reads zero however near the ridge. Red if a
    /// piece's nearest point or the faces' crossing is not a candidate.
    #[test]
    fn the_lowest_offset_along_a_segment_over_a_ridge() {
        let ridge = ridge();
        let n = v(1.0, 0.0, 1.0) / 2.0_f64.sqrt();
        for (h, cross, along) in [
            (0.0, 0.0, 0.0),
            (0.01, 0.01, 0.01 * 2.0_f64.sqrt()),
            (-0.01, -0.01 / 2.0_f64.sqrt(), -0.01),
        ] {
            let at = |q: Point3<f64>| q + n * h;
            close(
                lowest(&ridge, at(p(-1.0, 0.3, 1.0)), at(p(1.0, 0.3, -1.0))),
                cross,
                &format!("a segment crossing the ridge, h {h}"),
            );
            let lift = n * (h * 2.0_f64.sqrt());
            close(
                lowest(&ridge, p(0.0, -0.5, 0.0) + lift, p(0.0, 0.5, 0.0) + lift),
                along,
                &format!("a segment along the ridge, h {h}"),
            );
        }
        let eps = geom_core::Tol::witness().eps();
        for s in [eps, 0.1] {
            close(
                lowest(&ridge, p(-s, -0.5, 0.0), p(-s, 0.5, 0.0)),
                0.0,
                &format!("a segment resting {s} in from the ridge"),
            );
        }
    }

    /// **Inside the material the lowest offset is where two faces stand
    /// equally deep**, which no end and no piece's nearest point marks: a
    /// segment through the ridge's material from `(−1, ·, −0.2)` to
    /// `(−0.2, ·, −0.6)` is deepest at `t = 2/3`, `7/15` from both faces,
    /// while its ends stand `0.2` deep and its nearest point to the ridge
    /// line `0.28`. Red if the pairwise crossings are not candidates.
    #[test]
    fn the_deepest_point_of_a_segment_is_where_two_faces_tie() {
        close(
            lowest(&ridge(), p(-1.0, 0.3, -0.2), p(-0.2, 0.3, -0.6)),
            -7.0 / 15.0,
            "the tie's depth",
        );
    }

    /// **`reads` over-refuses a point in band of a sector's plane that
    /// lies decidedly outside the sector** (pinned, not wanted): over a
    /// block's corner, a point half a metre off both side faces and in the
    /// band's sliver of the top face's plane is far outside the material,
    /// but the top face's side of it reads in band before its place in the
    /// face is asked, so the parity reading refuses, and with it `offset`.
    /// Red when it is sharpened: then this row says what it reads.
    #[test]
    fn reads_refuses_a_point_in_band_of_a_plane_outside_its_sector() {
        let b = band();
        let z = 0.5 * (b.zero() + b.escalate());
        let q = p(0.5, 0.5, z);
        let block = corner(false);
        assert_eq!(block.reads(q, b), None, "the parity reading refuses");
        assert_eq!(
            block.offset(q, b),
            Err(TouchUnread::InBand),
            "and so does the offset, its magnitude definite"
        );
        assert_eq!(
            block.reads(p(0.5, 0.5, 2.0 * b.escalate()), b),
            Some(Material::Out),
            "off the band it reads"
        );
    }

    /// **The link decides only through its doors** (S4 of `CONTACT-7`'s
    /// design). A levered reading of directions decides a verdict right
    /// wherever it is definite and wrong where its Zero is taken as one:
    /// the class five review rounds found one site at a time. So every
    /// sign a link decides comes through [`metric::sign`], whose
    /// [`metric::Distance`] is built only from a point and a plane. This
    /// reads this module's source (all but its tests) and the census's
    /// touch analysis (`census.rs`, from its banner to the backstop's
    /// header), and holds:
    /// - every `Margin` door and every `decide` call lies inside `mod
    ///   metric` (one `Margin::of`);
    /// - the classifier of unit normals is called only from `pairing`;
    /// - nothing reads a bare sign: no `sign_within`, no comparison of a
    ///   real against zero (`< T::zero()`, `> T::zero()`), no
    ///   `is_sign_negative`, `is_sign_positive`, `signum`, `copysign`,
    ///   `abs(`, `partial_cmp` or `total_cmp`, and no `classify_dihedral` (a levered
    ///   classifier of its own). No construction code needs one today, so
    ///   the list has no exemption; one added for code whose outcome only
    ///   shrinks a piece must be named here;
    /// - `Distance` is built in exactly one place: its field is private,
    ///   `mod metric` holds exactly one `Self(` — inside `Distance::of` —
    ///   and no `Distance(`, and `of` is the module's only function
    ///   returning a `Distance`. So a product or a lever has no way to
    ///   become one, not even through a second constructor beside `of`.
    ///
    /// **What it cannot see.** `Distance::of` takes the plane's normal
    /// on trust: a normal scaled by a length is a lever spelled as a
    /// plane, and a point built as `p + (q − p)·k` is a product spelled
    /// as a point. Every caller passes a face's outward normal or a
    /// normalized direction, and a real face point, a point interpolated
    /// on a face's edge or a point of a segment `offset` is asked about;
    /// that is checked by reading, not by this row. Nor does it see a
    /// caller outside topo: `Gap`'s cell arms only call [`Link::offset`]
    /// and [`Link::lowest_on`].
    #[test]
    fn the_link_decides_only_through_its_doors() {
        use test_utils::source::{balanced_end, code_only};
        let own = include_str!("link.rs");
        let own = code_only(own);
        let module = &own[..own
            .find("\n#[cfg(test)]\nmod tests {")
            .expect("the module's tests")];
        let census = include_str!("census.rs");
        let census_code = code_only(census);
        let start = census
            .find("// ---- The touch analysis: arm 2's clear, condition 3.")
            .expect("the touch analysis's banner");
        let end = start
            + census[start..]
                .find("\n/// **The conservative loudness backstop**")
                .expect("the backstop's header after it");
        let touch = &census_code[start..end];
        let body = |anchor: &str| -> core::ops::Range<usize> {
            let at = module
                .find(anchor)
                .unwrap_or_else(|| panic!("the module holds `{anchor}`"));
            let open = at + module[at..].find('{').expect("its body");
            open..balanced_end(module, open).expect("its closing brace")
        };
        let metric = body("\npub(crate) mod metric");
        let pairing = body("\nfn pairing");
        let outside = |section: &str, needle: &str, allowed: &[&core::ops::Range<usize>]| {
            section
                .match_indices(needle)
                .map(|(i, _)| i)
                .filter(|i| !allowed.iter().any(|r| r.contains(i)))
                .map(|i| {
                    section[i.saturating_sub(60)..(i + 40).min(section.len())]
                        .trim()
                        .to_owned()
                })
                .collect::<Vec<String>>()
        };
        let mut stray = Vec::new();
        for needle in ["Margin::", "decide(", "classify_material_pairing"] {
            stray.extend(outside(touch, needle, &[]));
        }
        for needle in ["Margin::", "decide("] {
            stray.extend(outside(module, needle, &[&metric]));
        }
        stray.extend(outside(module, "classify_material_pairing", &[&pairing]));
        for needle in [
            "sign_within",
            "classify_dihedral",
            "< T::zero()",
            "> T::zero()",
            "is_sign_negative",
            "is_sign_positive",
            "signum",
            "partial_cmp",
            "total_cmp",
            "copysign",
            "abs(",
        ] {
            stray.extend(outside(module, needle, &[]));
            stray.extend(outside(touch, needle, &[]));
        }
        assert!(
            stray.is_empty(),
            "a decision in the link outside its doors: {stray:#?}"
        );
        let count =
            |needle: &str, r: &core::ops::Range<usize>| module[r.clone()].matches(needle).count();
        assert_eq!(
            count("Margin::", &metric),
            1,
            "the metric door's one `Margin::of`"
        );
        assert_eq!(
            count("Margin::of(", &metric),
            1,
            "the metric door's one `Margin::of`"
        );
        let door = &module[metric.clone()];
        assert!(
            door.contains("struct Distance<T>(T);"),
            "`Distance`'s field is private"
        );
        let of = body("\n        pub(crate) fn of(");
        assert!(
            metric.contains(&of.start),
            "`Distance::of` is in `mod metric`"
        );
        let builds: Vec<usize> = module
            .match_indices("Self(")
            .chain(module.match_indices("Distance("))
            .map(|(i, _)| i)
            .filter(|i| metric.contains(i))
            .collect();
        assert!(
            builds.len() == 1 && of.contains(&builds[0]),
            "`Distance` is built once, inside `Distance::of`: {} builds",
            builds.len()
        );
        assert_eq!(
            door.matches("-> Self").count() + door.matches("-> Distance").count(),
            1,
            "`Distance::of` is `mod metric`'s only function returning a `Distance`"
        );
    }
}

//! **The cone of material at a point** — a solid's LINK there: every
//! face of the solid through the point, as its sector ([`Link`]).
//!
//! A cone of directions at a point is a [`Cone`]: its sectors, each a
//! plane and a fan of rays. It is read by crossing parity: where a
//! direction lies against the material — inside, outside, or on a
//! sector — no convexity assumed, so a saddle reads like any other
//! corner ([`Link::reads`]). A link is a cone whose wedges are its faces'
//! real points, and it is read at their scale too:
//! - [`Link::offset`]: a point's signed distance from the cone's
//!   boundary, negative inside, as a closed form over the sectors, and
//!   [`Link::lowest_on`] the lowest of it along a segment;
//! - `rest::judge`: the one test of two links at one point, which the
//!   census's touch analysis reads.
//!
//! A sphere or cylinder face is read by its carrier ([`Round`]): its
//! offset and [`Round::lowest_on`], the least along a segment, in closed
//! form. `rest::by_round` is the test of a link against such a face
//! holding the point inside it, and `rest::meet` the one entry the
//! census reads either through.
//!
//! A link is built off a live body at a vertex ([`Link::at_point`]),
//! inside a line edge ([`Link::on_edge`]) or inside a planar face
//! ([`Link::in_face`]); the census builds its own off its snapshot,
//! gathered across the keys a touch ties to its point.
//!
//! **Every sign decided here is a point's signed distance from a
//! plane or from a sphere's or cylinder's carrier**, in metres, and it
//! comes through one door: [`metric`], whose [`metric::Distance`] is
//! built only from a point and a plane (a point on it and its unit
//! normal), or from a segment and a round carrier, as its lowest point's
//! ([`Round`]). Directions — face normals, ray
//! directions, their bisectors — only PROPOSE planes. One other sign
//! stays, named where it is made: [`pairing`], of unit normals of
//! magnitude about 1. Beside the signs, [`Link::lowest_on`] and
//! [`Round::lowest_on`] branch on structure alone: a candidate that is
//! not a bounded number is skipped (or, for a round carrier's enclosure,
//! the whole segment stands in for it). `Round::reach_from_axis` reads a
//! point exactly on the axis as structure, by whether the scalar moves it.
//! `link::tests::the_link_decides_only_through_its_doors` holds the signs
//! to the doors by reading this file's source, and the census's touch
//! analysis beside it, and says what such a reading cannot see.
//!
//! Each face is read through its PIECE at the point `p`: the part of the
//! face visible from `p` ([`face_piece`]). The piece is star-shaped from
//! `p`, so every point of it lies on a segment from `p` to its boundary,
//! and it spans the face's whole sector at `p`. Nothing is extrapolated,
//! and a face that is not convex is read where it is near `p`, not where
//! it reaches back far from it.
//!
//! A link holds planar faces only. One with an edge that is not a line
//! is not read through a piece: held inside it or inside a line edge, it
//! is read as the disc about `p` its other edges clear, the face's own
//! out to that radius, which bounds the link's ball, and carried past
//! the face's reach so that its readings are taken where the face's
//! would be ([`FaceSector::cleared`]). A curved face, or such a face held at
//! its corner, refuses ([`TouchUnread::Unreadable`]); a sphere or
//! cylinder face is read by its carrier instead, as above.

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

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
/// K name: a read point of a segment against the ball a refused
/// point's distance from the boundary clears, in metres — its side of
/// the ball's tangent plane toward it ([`Link::lowest_on`]).
const LINK_BALL: &str = "link_offset_ball";
/// K name: a point of a segment against the piece of a link's boundary
/// it is nearest, along the line to it, and two such points against each
/// other along the segment, in metres ([`Link::lowest_on`]'s ties).
const LINK_TIE: &str = "link_offset_tie";
/// K name: a point of a link's boundary, or the lowest of a segment of
/// it, against a sphere's or a cylinder's carrier, in metres — on the
/// side of it the face's material is on ([`rest::by_round`]).
const LINK_ROUND: &str = "link_round_offset";
/// K name: two coplanar faces' outward normals, one of each link,
/// aligned or opposed ([`geom_brep::classify_material_pairing_as`]).
const TOUCH_NORMAL: &str = "census_touch_normal";

/// **The one door every verdict sign of the touch analysis comes
/// through.** A [`Distance`] has a private field and two constructors,
/// from a point and a plane and from a segment and a sphere's or a
/// cylinder's carrier, so a lever, a product or any other quantity
/// cannot be decided here as a verdict: it has no way to become a
/// `Distance`.
///
/// What the type cannot hold: that `n` is a unit normal. Every caller
/// passes a normalized direction or a face's outward normal, and a
/// normal scaled by a length would be a lever spelled as a plane; the
/// source row states that blind spot.
pub(crate) mod metric {
    use super::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3, decide};

    /// A point's signed distance from a plane, in metres.
    pub(crate) struct Distance<T>(T);

    impl<T: Real> Distance<T> {
        /// `n·(q − o)`: `q`'s signed distance from the plane through
        /// `o` with unit normal `n`.
        pub(crate) fn of(q: Point3<T>, o: Point3<T>, n: Vec3<T>) -> Self {
            Self(n.dot(q - o))
        }
    }

    impl<T: Decide> Distance<T> {
        /// The segment `x0`–`x1`'s lowest signed distance from a sphere's
        /// or a cylinder's carrier, positive away from its material
        /// ([`Round::lowest_on`](super::Round::lowest_on)); a point is
        /// the segment from it to itself. `None` where it has no value
        /// ([`AtTheAxis`](super::AtTheAxis)).
        pub(crate) fn off(round: &super::Round<T>, seg: (Point3<T>, Point3<T>)) -> Option<Self> {
            match round.lowest_on(seg) {
                Ok(d) => Some(Self(d)),
                Err(_) => None,
            }
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
    /// the reading. `own`: a point is read against a ray at its own
    /// distance, not drawn in to the ray's real point first — the cone's
    /// own reading, for a point the cone is carried out to
    /// ([`Link::offset`](super::Link::offset)); the census reads drawn in.
    #[derive(Clone, Copy)]
    pub(crate) struct Reading<T> {
        pub(crate) band: Band,
        pub(crate) slack: Option<T>,
        pub(crate) own: bool,
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

    /// The distance's escalation under the run band, as the K row
    /// `name`, where it reads in band: what a refusal carries.
    pub(crate) fn why<T: Decide>(
        name: &'static str,
        d: Distance<T>,
        band: Band,
    ) -> Option<Indeterminate> {
        decided(name, d.0, band).err()
    }

    fn read<T: Decide>(name: &'static str, v: T, band: Band) -> Option<Sign> {
        decided(name, v, band).ok()
    }

    fn decided<T: Decide>(name: &'static str, v: T, band: Band) -> Result<Sign, Indeterminate> {
        decide(name, Margin::of(v), band)
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
/// [`ValidationError::CoincidentWedgesOverlapUndecided`](crate::validate::ValidationError::CoincidentWedgesOverlapUndecided), a solid
/// touching itself at a point as
/// [`ValidationError::BoundaryCrossesItselfUndecided`](crate::validate::ValidationError::BoundaryCrossesItselfUndecided), and `Gap`'s
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
    /// A curved face or edge takes part: a face off a plane, or a planar
    /// face with an edge that is not a line where [`FaceSector::cleared`]
    /// does not read it — held at a corner, or bounded by a curve no
    /// clearance reads. (A tied edge whose key does not
    /// resolve reads the same; tier 1, which the census runs behind,
    /// admits none. A face whose surface or loops do not resolve reads
    /// [`Self::Corrupt`].)
    Unreadable,
    /// A face's piece at the touch point does not close on decided
    /// readings (a gap between neighbouring turns of half a turn or
    /// more, or no edge met across one, [`face_piece`]); or a link's
    /// sectors do not pair up at their bounds ([`Cone::closes`]).
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

/// **One sector of a cone at its point `p`**: a plane through `apex` of
/// outward unit normal `outward`, and a fan of wedges in it about the
/// apex, each a triangle `(apex, a, b)` under half a turn, in turn
/// counterclockwise about the outward normal, consecutive wedges on one
/// ray. Each ray is fixed by its point `a` or `b`, at the ray's reach.
pub(crate) struct Sector<T: Real> {
    /// The outward unit normal.
    pub(crate) outward: Vec3<T>,
    /// The point the fan is built about: the cone's point, or the
    /// corner of this face on one of its keys.
    pub(crate) apex: Point3<T>,
    pub(crate) wedges: Vec<Segment<T>>,
    /// The fan closes all the way round: `p` is inside the face.
    /// Otherwise its first ray and its last are the sector's bounds.
    pub(crate) closed: bool,
}

/// **One face's sector at the touch point `p`**, spanning the face's
/// whole sector at `p`: either the face's piece there ([`face_piece`]),
/// whose wedges are real points of the face, or the disc its edges clear
/// ([`FaceSector::cleared`]), the face's own inside `clear` and carried
/// past the face beyond it.
pub(crate) struct FaceSector<T: Real> {
    pub(crate) sector: Sector<T>,
    pub(crate) face: LinkFace<T>,
}

/// **What a link holds of a sector's face** beyond the cone: its surface
/// and sense, which a coplanar pair of faces is read by ([`pairing`]),
/// and how far its wedges are the face's own.
pub(crate) struct LinkFace<T: Real> {
    pub(crate) surface: geom::Surface<T>,
    /// `Face::sense`, the bit [`geom_brep::classify_material_pairing_as`]
    /// takes.
    pub(crate) sense: bool,
    /// Where the wedges run past the face (a cleared disc): the radius
    /// about `p` inside which they are the face's own. `None` where every
    /// wedge is real points of the face.
    pub(crate) clear: Option<T>,
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

/// **A cone of directions at a point `p`**: its sectors, each a plane
/// and a fan of rays. Its material near `p` is the set of directions an
/// odd number of its sectors separate from outside ([`Cone::reads_over`]),
/// where its sectors pair up at their bounds ([`Cone::closes`]). A cone
/// makes no claim about how far its sectors are a body's faces, so
/// nothing at a scale is read off it: that is a [`Link`]'s.
pub(crate) struct Cone<T: Real> {
    pub(crate) p: Point3<T>,
    pub(crate) sectors: Vec<Sector<T>>,
    /// The farthest a sector's apex stands from `p`: zero but where a
    /// corner on another key of the point was decided one point with it.
    pub(crate) tie: T,
}

/// **One solid's link at a touch point `p`**: every face of the solid
/// through `p`, as its sector there — a [`Cone`] whose wedges are the
/// faces' real points, or a cleared disc's ([`FaceSector::cleared`]),
/// read at their scale: the ball they fill, a point's offset from the
/// cone, and the touch analysis ([`rest`]).
pub struct Link<T: Real> {
    pub(crate) cone: Cone<T>,
    /// Each sector's face, by the sector's index in the cone.
    pub(crate) faces: Vec<LinkFace<T>>,
    /// The farthest wedge point from `p`.
    pub(crate) reach: T,
    /// The radius of the ball about `p` the link's wedges fill: the
    /// least distance from `p` to a wedge's far side, or to a cleared
    /// sector's edge of the face ([`LinkFace::clear`]). Inside it each
    /// of the link's faces covers its whole sector; a face of the solid
    /// that reaches into the ball without being tied to the point is
    /// not in the link, and is the backstop's to answer for (its
    /// material probe and findings).
    pub(crate) ball: T,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
/// `also` names the face's other passes through `p`, where the face
/// meets `p` again — corners, and edges with `p` inside them: their
/// edges bound other sectors of the face and are not read as blocking
/// here, and the sector ends at the first incoming ray, of any pass, it
/// reaches.
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
    also: &[PieceAt],
    band: Band,
) -> Result<Vec<Point3<T>>, TouchUnread> {
    use core::cmp::Ordering::{Equal, Greater, Less};
    use metric::sign;
    // The fan is built about its own apex: no slack.
    let rd = Reading {
        band,
        slack: None,
        own: false,
    };
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
            let holds = |pass: PieceAt| match pass {
                PieceAt::Corner(rr, kk) => rr == r && (k == kk || (k + 1) % n == kk),
                PieceAt::Edge(rr, kk) => rr == r && k == kk,
                PieceAt::Inside => false,
            };
            if !(holds(at) || also.iter().any(|&pass| holds(pass))) {
                edges.push((ring[k], ring[(k + 1) % n]));
            }
            let corner =
                |pass: PieceAt| matches!(pass, PieceAt::Corner(rr, kk) if rr == r && kk == k);
            let at_p = corner(at) || also.iter().any(|&pass| corner(pass));
            if !at_p {
                points.push(ring[k]);
            }
        }
    }
    // The reference ray the turns count from: the outgoing edge, or a
    // vertex of the face inside it.
    let start = match at {
        PieceAt::Corner(r, k) | PieceAt::Edge(r, k) => rings[r][(k + 1) % rings[r].len()],
        PieceAt::Inside => *points.first().ok_or(TouchUnread::Corrupt)?,
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
    // A pass's sector runs from its outgoing ray to the first incoming
    // ray of ANY pass counterclockwise from it, its own included: the
    // face's interior lies left of the outgoing edge until the loop next
    // reaches `p`. (A loop through two vertices on one point, a pinch,
    // returns to `p` on another lobe's edge, so its own pass's incoming
    // edge is not where it ends.) A corner's incoming ray runs to its
    // loop's previous vertex; an edge through `p` comes in from its
    // start. An incoming ray along the outgoing one (a slit's two edges,
    // a crack's tip) closes the turn: the interior lies left of the
    // outgoing edge just off it, so the sector ends there a full turn
    // round, never at its start; where every incoming ray does, the
    // sector is that full turn.
    let end = match at {
        PieceAt::Corner(..) | PieceAt::Edge(..) => {
            let mut best: Option<(Turn, Point3<T>)> = None;
            for &pass in core::iter::once(&at).chain(also) {
                let q = match pass {
                    PieceAt::Corner(rr, kk) => {
                        let n = rings[rr].len();
                        rings[rr][(kk + n - 1) % n]
                    }
                    PieceAt::Edge(rr, kk) => rings[rr][kk],
                    PieceAt::Inside => continue,
                };
                let item = (turn(q)?, q);
                if item.0 == Turn::Ahead {
                    continue;
                }
                best = match best {
                    Some(b) if order(item, b)? != Less => Some(b),
                    _ => Some(item),
                };
            }
            best.map(|b| b.1)
        }
        PieceAt::Inside => None,
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
/// carried beyond what fixes it. An `own` reading reads `x` where it is:
/// the cone carried out ([`Link::offset`]).
fn wedge_side<T: Decide>(
    p: Point3<T>,
    m: Vec3<T>,
    r: Point3<T>,
    x: Point3<T>,
    rd: Reading<T>,
) -> Option<Sign> {
    let x = if rd.own {
        x
    } else {
        within(p, x, (r - p).norm())
    };
    metric::sign_over(
        TOUCH_WEDGE,
        Distance::of(x, p, m.cross((r - p).normalize())),
        rd,
    )
}

/// Whether `x` lies ahead of `p` along the ray through `r` or behind it,
/// read no further out than `r` (as [`wedge_side`]).
fn ahead<T: Decide>(p: Point3<T>, r: Point3<T>, x: Point3<T>, rd: Reading<T>) -> Option<Sign> {
    let x = if rd.own {
        x
    } else {
        within(p, x, (r - p).norm())
    };
    metric::sign_over(TOUCH_AHEAD, Distance::of(x, p, (r - p).normalize()), rd)
}

/// `q` drawn in toward `p` along its ray to lie within `r` of it — on a
/// wedge's ray or inside it, a point of the same face.
fn within<T: Real>(p: Point3<T>, q: Point3<T>, r: T) -> Point3<T> {
    p + (q - p) * (r / (q - p).norm()).min(T::from_f64(1.0))
}

/// `p`'s distance from the segment `a`–`b`: from its foot on the
/// segment's line, held to the segment.
pub(crate) fn to_segment<T: Real>(p: Point3<T>, (a, b): Segment<T>) -> T {
    let ab = b - a;
    let t = ((p - a).dot(ab) / ab.dot(ab))
        .max(T::zero())
        .min(T::from_f64(1.0));
    (a + ab * t - p).norm()
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
/// further out than `q` ([`Cone::closes`]'s reading of a bound on a
/// bound).
fn on_ray<T: Decide>(c: Point3<T>, m: Vec3<T>, q: Point3<T>, x: Point3<T>, rd: Reading<T>) -> bool {
    let near = within(c, x, (q - c).norm());
    metric::sign_over(TOUCH_PLANE, Distance::of(near, c, m), rd) == Some(Sign::Zero)
        && wedge_side(c, m, q, x, rd) == Some(Sign::Zero)
        && ahead(c, q, x, rd) == Some(Sign::Positive)
}

impl<T: Real> FaceSector<T> {
    /// A face sector on the plane of `sector` through its apex, its
    /// sense `true`, every wedge real points of the face: for rows that
    /// state a cone by hand rather than build a body.
    #[cfg(test)]
    pub(crate) fn planar(sector: Sector<T>) -> Self {
        let surface = geom::Surface::Plane {
            origin: sector.apex,
            normal: sector.outward,
            u_ref: (sector.wedges[0].0 - sector.apex).normalize(),
        };
        Self {
            sector,
            face: LinkFace {
                surface,
                sense: true,
                clear: None,
            },
        }
    }
}

impl<T: Decide> FaceSector<T> {
    /// One planar, line-bounded face as its sectors at `p`, held there
    /// as `holds` say: inside it (one sector about `p`), or at each pass
    /// of its loops through the point — every corner the holds name,
    /// about its vertex, whether the loops pass one vertex twice or two
    /// vertices on one point (a pinch, one vertex per cone), and every
    /// edge a hold holds the point inside, about `p` — each pass ending
    /// at the first incoming ray of any pass ([`face_piece`]). A corner
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
        // An edge two holds name is one pass.
        edges.sort_unstable();
        edges.dedup();
        let inside = holds.contains(&Hold::Inside);
        let sector = |at: Point3<T>, place: PieceAt, also: &[PieceAt]| {
            let piece = face_piece(at, face.outward, &face.rings, place, also, band)?;
            Ok(Self {
                sector: Sector {
                    outward: face.outward,
                    apex: at,
                    wedges: piece.chunks_exact(2).map(|w| (w[0], w[1])).collect(),
                    closed: matches!(place, PieceAt::Inside),
                },
                face: LinkFace {
                    surface: face.surface.clone(),
                    sense: face.sense,
                    clear: None,
                },
            })
        };
        let named_corners = holds
            .iter()
            .filter(|h| matches!(h, Hold::Corner(_)))
            .count();
        // Every pass of the loops through the point, a corner about its
        // vertex and an edge about `p`. A corner or edge hold beside an
        // inside hold names the touch there: the face's interior holds
        // no point of its boundary.
        let passes: Vec<(PieceAt, Point3<T>)> = corners
            .iter()
            .map(|&((r, k), q)| (PieceAt::Corner(r, k), q))
            .chain(edges.iter().map(|&(r, k)| (PieceAt::Edge(r, k), p)))
            .collect();
        match (&passes[..], inside) {
            ([], true) if named_corners == 0 => Ok(vec![sector(p, PieceAt::Inside, &[])?]),
            ([], _) => Err(TouchUnread::Corrupt),
            _ => passes
                .iter()
                .map(|&(at, q)| {
                    let also: Vec<PieceAt> = passes
                        .iter()
                        .map(|pass| pass.0)
                        .filter(|&pass| pass != at)
                        .collect();
                    sector(q, at, &also)
                })
                .collect(),
        }
    }
}

impl<T: Decide> Sector<T> {
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

impl<T: Decide> Cone<T> {
    /// A cone from its sectors.
    pub(crate) fn of(p: Point3<T>, sectors: Vec<Sector<T>>) -> Result<Self, TouchUnread> {
        if sectors.is_empty() {
            return Err(TouchUnread::Corrupt);
        }
        let tie = sectors
            .iter()
            .map(|s| (s.apex - p).norm())
            .fold(T::zero(), |r, l| r.max(l));
        Ok(Self { p, sectors, tie })
    }

    /// The cone where it closes ([`Cone::closes`]): its sectors bound a
    /// region of directions, so the parity reading reads it.
    /// `Unanalysed` where they do not pair up at their bounds, `InBand`
    /// where a bound's reading is in band.
    pub(crate) fn held_closed(&self, rd: Reading<T>) -> Result<(), TouchUnread> {
        match self.closes(rd) {
            Some(true) => Ok(()),
            Some(false) => Err(TouchUnread::Unanalysed),
            None => Err(TouchUnread::InBand),
        }
    }

    /// `q`'s side of sector `s`'s plane through `p`.
    fn side(&self, s: &Sector<T>, q: Point3<T>, rd: Reading<T>) -> Option<Sign> {
        metric::sign_over(TOUCH_PLANE, Distance::of(q, self.p, s.outward), rd)
    }

    /// **Whether the cone closes**: every sector bound is the bound of an
    /// even number of the cone's sectors, counted with its own, so the
    /// sectors bound a region of directions and crossing-parity reads
    /// it ([`Cone::reads_over`]). Gathering makes it so — every edge at a
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
                // plane's too ([`within`]). [`Cone::side`] reads a point
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

    /// **Where the direction to `q` lies against the cone's material**,
    /// by arc crossing-parity, no convexity assumed. `q` on a sector's
    /// plane inside it, or on its bound, is [`Material::On`] that sector:
    /// in a cone that closes, a bound is one an even number of sectors
    /// share, and a direction along it lies on the material's boundary. A
    /// plane `q` lies in band of but decidedly outside its sector is one
    /// no arc from `q` crosses inside that sector, its crossing stretch
    /// widened from the band's zero to `q`'s own distance from the plane
    /// ([`Cone::crossing`]). Otherwise `q` is
    /// read along the minor arc to a reference `w`, a wedge's centroid on
    /// one sector `T` carried out to `q`'s distance (planes are read at
    /// one scale; rays at their own, [`wedge_side`]), where the link's
    /// material is the side behind `T`:
    /// the material holds `q` iff `q` lies behind `T` and the arc crosses
    /// the other sectors an even number of times, or in front and odd.
    /// The arc is the segment `q`–`w` seen from `p`, under half a turn
    /// (both ends ahead of the plane normal to their bisector), so it
    /// meets each plane through `p` at most once and `T`'s only at `w`,
    /// and each other sector is read as [`Cone::crossing`] reads it.
    /// Parity is the material only where the cone's winding takes two
    /// values ([`rest::one_material`]). A reference any reading of which
    /// is in band, on a sector's bound, or on a second sector at `w` is
    /// passed over for the next; `None` where none reads, or where `q`
    /// itself is in band of a plane it is not decidedly outside the
    /// sector of.
    pub(crate) fn reads_over(&self, q: Point3<T>, rd: Reading<T>) -> Option<Material> {
        let p = self.p;
        // `q`'s strict side of each plane; `None` for a plane `q` lies
        // on outside its sector, which no arc from `q` crosses inside it.
        let mut sides = Vec::with_capacity(self.sectors.len());
        // Where such a plane was read in band, how far `q` may stand
        // off it.
        let mut reach: Vec<Option<T>> = vec![None; self.sectors.len()];
        for (k, s) in self.sectors.iter().enumerate() {
            let side = match self.side(s, q, rd) {
                Some(side) => side,
                None if s.place(p, q, rd) == Some(Place::Outside) => {
                    reach[k] = Some(s.outward.dot(q - p).abs() + rd.slack());
                    sides.push(None);
                    continue;
                }
                None => return None,
            };
            sides.push(match side {
                Sign::Zero => match s.place(p, q, rd)? {
                    Place::Inside | Place::Bound => return Some(Material::On(k)),
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
                    let Some(crossed) = self.crossing(s, sides[k], q, w, reach[k], rd) else {
                        continue 'reference;
                    };
                    held ^= crossed != 0;
                }
                return Some(if held { Material::In } else { Material::Out });
            }
        }
        None
    }

    /// **The arc from `w` to `q` against sector `s`**, as the winding it
    /// adds: `1` where it crosses `s` into the material behind it, `-1`
    /// where it crosses out, `0` where it does not cross. `sq` is `q`'s
    /// strict side of `s`'s plane, `None` for `q` on the plane outside
    /// the sector. On one side, not crossed; on opposite sides, crossed
    /// iff the arc's point on the plane lies inside the sector. An end
    /// read Zero lies within `zero` of the plane on either side (the
    /// band's zero, the slack added, or `reach` where [`Cone::reads_over`]
    /// read `q` in band of the plane outside the sector), so a crossing, if any, lies on the
    /// stretch of the arc where the plane can then be met: not crossed
    /// only where that whole stretch misses the sector
    /// ([`Sector::misses`]). `None` where that does not decide.
    fn crossing(
        &self,
        s: &Sector<T>,
        sq: Option<Sign>,
        q: Point3<T>,
        w: Point3<T>,
        reach: Option<T>,
        rd: Reading<T>,
    ) -> Option<i32> {
        let p = self.p;
        let at_w = self.side(s, w, rd)?;
        let (oq, ow) = (s.outward.dot(q - p), s.outward.dot(w - p));
        let zero = reach.unwrap_or(T::from_computed(rd.band.zero()) + rd.slack());
        let at = |t: T| q + (w - q) * t;
        match (sq, at_w) {
            (None, Sign::Zero) => None,
            (None, sw) => {
                let ow = if sw == Sign::Positive { ow } else { -ow };
                s.misses(p, (q, at(zero / (zero + ow))), rd).then_some(0)
            }
            (Some(sq), Sign::Zero) => {
                let z = if sq == Sign::Positive { zero } else { -zero };
                s.misses(p, (at(oq / (oq + z)), w), rd).then_some(0)
            }
            (Some(at_q), at_w) if at_q == at_w => Some(0),
            (Some(at_q), _) => match s.place(p, on_plane(q, w, oq, ow), rd) {
                Some(Place::Inside) if at_q == Sign::Negative => Some(1),
                Some(Place::Inside) => Some(-1),
                Some(Place::Outside) => Some(0),
                _ => None,
            },
        }
    }

    /// **The winding just in front of sector `on` at `q`, less the
    /// winding just in front of sector `from` at `w`**, each point
    /// strictly inside its own sector: the signed sum of
    /// [`Cone::crossing`] along the minor arc from `w` to `q`, `w`
    /// carried along its ray to `q`'s distance as [`Cone::reads_over`]
    /// carries its reference. The arc leaves `from` on `q`'s side and
    /// reaches `on` from `w`'s, and the material behind a sector winds
    /// one more than the side in front of it. `None` where a reading is
    /// in band, the arc is not under half a turn, an end lies on the
    /// other's plane, or `q` lies on a third sector, or on its bound.
    fn winds(
        &self,
        (on, q): (usize, Point3<T>),
        (from, w): (usize, Point3<T>),
        rd: Reading<T>,
    ) -> Option<i32> {
        if on == from {
            return None;
        }
        let p = self.p;
        let w = p + (w - p) * ((q - p).norm() / (w - p).norm());
        let half = ((q - p).normalize() + (w - p).normalize()).normalize();
        for x in [q, w] {
            if metric::sign_over(TOUCH_ARC, Distance::of(x, p, half), rd) != Some(Sign::Positive) {
                return None;
            }
        }
        let behind = |sign: Sign| match sign {
            Sign::Negative => Some(1),
            Sign::Positive => Some(0),
            Sign::Zero => None,
        };
        let mut n = behind(self.side(&self.sectors[from], q, rd)?)?
            - behind(self.side(&self.sectors[on], w, rd)?)?;
        for (k, s) in self.sectors.iter().enumerate() {
            if k == on || k == from {
                continue;
            }
            let sq = match self.side(s, q, rd)? {
                Sign::Zero => match s.place(p, q, rd)? {
                    Place::Outside => None,
                    Place::Inside | Place::Bound => return None,
                },
                sign => Some(sign),
            };
            n += self.crossing(s, sq, q, w, None, rd)?;
        }
        Some(n)
    }
}

impl<T: Decide> Link<T> {
    /// A link from its faces' sectors.
    pub(crate) fn of(p: Point3<T>, sectors: Vec<FaceSector<T>>) -> Result<Self, TouchUnread> {
        let wedges = || sectors.iter().flat_map(|s| &s.sector.wedges);
        let reach = wedges()
            .flat_map(|&(a, b)| [a, b])
            .map(|q| (q - p).norm())
            .fold(T::zero(), |r, l| r.max(l));
        let ball = wedges()
            .map(|&(a, b)| to_segment(p, (a, b)))
            .chain(sectors.iter().filter_map(|s| s.face.clear))
            .fold(reach, |r, l| r.min(l));
        let (sectors, faces) = sectors.into_iter().map(|s| (s.sector, s.face)).unzip();
        Ok(Self {
            cone: Cone::of(p, sectors)?,
            faces,
            reach,
            ball,
        })
    }

    /// The link's faces' sectors, as [`Link::of`] takes them.
    #[cfg(test)]
    pub(crate) fn into_faces(self) -> Vec<FaceSector<T>> {
        self.cone
            .sectors
            .into_iter()
            .zip(self.faces)
            .map(|(sector, face)| FaceSector { sector, face })
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
        let (p, m) = (self.cone.p, s.outward);
        let mut out = Vec::new();
        for &(a, b) in &s.wedges {
            let mut cuts: Vec<Point3<T>> = Vec::new();
            for &r in rays {
                let sides = (wedge_side(p, m, a, r, rd), wedge_side(p, m, b, r, rd));
                let outside = matches!(sides, (Some(Negative), _) | (_, Some(Positive)));
                let inside = match self.cone.side(s, r, rd) {
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
}

/// **One planar, line-bounded face as [`FaceSector::of_face`] reads it**:
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

impl<T: Decide> FaceSector<T> {
    /// Face `f` of a live `body` as its sectors at `p`, held there as
    /// `holds` say ([`FaceSector::of_face`]), or as the disc its edges clear
    /// where one of them is not a line ([`FaceSector::cleared`]): a face off
    /// a plane is [`TouchUnread::Unreadable`].
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
                    return Self::cleared(body, f, outward, p, holds);
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

    /// **A planar face with an edge that is not a line, as its sectors
    /// at `p`**, held inside the face or inside one of its line edges:
    /// the disc about `p` that every other edge of the face clears
    /// ([`clearance`]) — whole for a point inside the face, and inside an
    /// edge, the half of it left of each pass of the face's loops along
    /// the edge about the outward normal (the face's side, [`face_piece`]'s
    /// orientation) — carried out past the face's reach.
    ///
    /// Inside the disc the sector is the face's own: every edge but the
    /// held one stands at least the radius off `p`, and so do the held
    /// edge's ends, so inside the disc the face's boundary is the held
    /// edge alone, a diameter, or nothing. The open disc, or its open
    /// half on the face's side, is connected, meets no boundary, and
    /// holds points of the face as near `p` as any, so it lies inside the
    /// face, and the sector spans the face's whole sector at `p`. The
    /// radius is the sector's [`LinkFace::clear`], which bounds the link's
    /// ball.
    ///
    /// The fan itself runs further, past the face ([`fan`]): its every
    /// chord stands past the farthest the face's boundary stands from `p`
    /// ([`face_reach_from`](crate::splitting::rules::face_reach_from)),
    /// so every point of the rim lies past every point of the face, and a
    /// coincidence read there holds over the whole face.
    ///
    /// A corner hold, a curved edge held, two edges held at once, a face
    /// off a plane, or an edge no clearance reads is
    /// [`TouchUnread::Unreadable`]; a radius or a reach that is not a
    /// positive number, [`TouchUnread::ChordScale`].
    pub(crate) fn cleared(
        body: &Body<T>,
        f: FaceKey,
        outward: Vec3<T>,
        p: Point3<T>,
        holds: &[Hold],
    ) -> Result<Vec<Self>, TouchUnread> {
        let face = body.get_face(f).ok_or(TouchUnread::Corrupt)?;
        let surface = body
            .get_surface(face.surface)
            .cloned()
            .ok_or(TouchUnread::Corrupt)?;
        let geom::Surface::Plane { u_ref, .. } = surface else {
            return Err(TouchUnread::Unreadable);
        };
        if holds.iter().any(|h| matches!(h, Hold::Corner(_))) {
            return Err(TouchUnread::Unreadable);
        }
        let mut held = holds.iter().filter_map(|h| match *h {
            Hold::Edge(a, b) => Some((a, b)),
            Hold::Corner(_) | Hold::Inside => None,
        });
        let edge = held.next();
        if held.next().is_some() {
            return Err(TouchUnread::Unreadable);
        }
        // The held edge's ends bound the radius too: a spur's tip is the
        // end of no other edge.
        let mut radius: Option<T> = match edge {
            Some((a, b)) => {
                let (a, b) = (position(body, a)?, position(body, b)?);
                Some((a - p).norm().min((b - p).norm()))
            }
            None => None,
        };
        let mut passes = Vec::new();
        for cycle in crate::census::face_cycles(body, face) {
            for he in cycle.map_err(|_| TouchUnread::Corrupt)? {
                let half = body.half_edges.get(he).ok_or(TouchUnread::Corrupt)?;
                let end = body.half_edge_end(he).ok_or(TouchUnread::Corrupt)?;
                if let Some((a, b)) = edge
                    && ((half.start, end) == (a, b) || (half.start, end) == (b, a))
                    && crate::census::edge_is_line(body, half.edge)
                {
                    passes.push((position(body, end)? - position(body, half.start)?).normalize());
                    continue;
                }
                let c = clearance(body, he, p)?;
                radius = Some(radius.map_or(c, |r| r.min(c)));
            }
        }
        let radius = radius.ok_or(TouchUnread::Corrupt)?;
        let far = crate::splitting::rules::face_reach_from(body, f, p)
            .map_err(|_| TouchUnread::Corrupt)?;
        for length in [radius, far] {
            if !geom_core::is_finite_length(length) || geom_core::is_zero_length(length, length) {
                return Err(TouchUnread::ChordScale);
            }
        }
        let along: Vec<(Vec3<T>, bool)> = match edge {
            None => vec![((u_ref - outward * outward.dot(u_ref)).normalize(), false)],
            // No line edge between the held ends: a curved edge held.
            Some(_) if passes.is_empty() => return Err(TouchUnread::Unreadable),
            Some(_) => passes.into_iter().map(|d| (d, true)).collect(),
        };
        Ok(along
            .into_iter()
            .map(|(a, half)| Self {
                sector: Sector {
                    outward,
                    apex: p,
                    wedges: fan(p, outward, a, far, half),
                    closed: !half,
                },
                face: LinkFace {
                    surface: surface.clone(),
                    sense: face.sense,
                    clear: Some(radius),
                },
            })
            .collect())
    }
}

/// The fan about `p` normal to `outward` whose every chord stands past
/// `reach`: eight wedges of an eighth of a turn counterclockwise about it
/// from the in-plane unit `a`, or, for a `half`, the four left of `a`.
/// A chord of a wedge of half-angle `θ` stands `cos θ` of the rim's
/// radius off `p`, so the rim is `reach / cos θ`, with a hundredth to
/// spare for the rounding of `reach` and of the turns.
fn fan<T: Real>(
    p: Point3<T>,
    outward: Vec3<T>,
    a: Vec3<T>,
    reach: T,
    half: bool,
) -> Vec<Segment<T>> {
    let b = outward.cross(a);
    let s = T::from_f64(core::f64::consts::FRAC_1_SQRT_2);
    let back = T::from_f64(-1.0);
    // Eight turns, so each wedge's half-angle is π/8: the array's type
    // and the rim's factor change together.
    let turns: [Vec3<T>; 8] = [
        a,
        (a + b) * s,
        b,
        (b - a) * s,
        a * back,
        (a + b) * (s * back),
        b * back,
        (a - b) * s,
    ];
    let radius = reach * T::from_f64(1.01) / T::from_f64(core::f64::consts::FRAC_PI_8).cos();
    let rim: Vec<Point3<T>> = turns
        .iter()
        .take(if half {
            turns.len() / 2 + 1
        } else {
            turns.len()
        })
        .map(|&t| p + t * radius)
        .collect();
    let mut wedges: Vec<Segment<T>> = rim.windows(2).map(|w| (w[0], w[1])).collect();
    if !half {
        wedges.push((rim[rim.len() - 1], rim[0]));
    }
    wedges
}

/// **How far `p` stands from the edge of half-edge `he`, at least**, in
/// metres: a line's distance from its segment, another curve's from its
/// whole carrier ([`geom::Curve3::clearance`]). [`TouchUnread::Unreadable`]
/// for a spline, which has no closed-form bound, or an edge with no
/// certified curve; [`TouchUnread::Corrupt`] where it does not read.
pub(crate) fn clearance<T: Decide>(
    body: &Body<T>,
    he: crate::entity::HalfEdgeKey,
    p: Point3<T>,
) -> Result<T, TouchUnread> {
    let half = body.half_edges.get(he).ok_or(TouchUnread::Corrupt)?;
    let curve = body
        .edges
        .get(half.edge)
        .and_then(|d| body.curves.get(d.curve))
        .and_then(crate::null::CurveGeom::certified)
        .ok_or(TouchUnread::Unreadable)?;
    if let geom::Curve3::Line { .. } = curve.carrier() {
        let end = body.half_edge_end(he).ok_or(TouchUnread::Corrupt)?;
        return Ok(to_segment(
            p,
            (position(body, half.start)?, position(body, end)?),
        ));
    }
    curve.carrier().clearance(p).ok_or(TouchUnread::Unreadable)
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
            sectors.extend(FaceSector::of_body_face(
                body,
                f,
                p,
                &[Hold::Corner(v)],
                band,
            )?);
        }
        Self::of(p, sectors)?.closed(band)
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
            sectors.extend(FaceSector::of_body_face(
                body,
                f,
                p,
                &[Hold::Edge(v0, v1)],
                band,
            )?);
        }
        Self::of(p, sectors)?.closed(band)
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
        Self::of(
            p,
            FaceSector::of_body_face(body, f, p, &[Hold::Inside], band)?,
        )?
        .closed(band)
    }

    /// The link where its cone closes ([`Cone::held_closed`]).
    fn closed(self, band: Band) -> Result<Self, TouchUnread> {
        self.cone.held_closed(Reading {
            band,
            slack: None,
            own: false,
        })?;
        Ok(self)
    }

    /// The point the link is about.
    pub fn point(&self) -> Point3<T> {
        self.cone.p
    }

    /// **Where the direction to `q` lies against the link's material**
    /// ([`Link::reads_over`] on `band` alone); `None` where a reading it
    /// needs is in band.
    pub fn reads(&self, q: Point3<T>, band: Band) -> Option<Material> {
        self.cone.reads_over(
            q,
            Reading {
                band,
                slack: None,
                own: false,
            },
        )
    }

    /// **`q`'s signed distance from the cone's boundary**, in metres:
    /// negative where the parity reading says In, the distance itself
    /// where it says Out or On. The cone is the link's sectors carried
    /// out from their apexes without end, so this is a statement about
    /// the material near the point carried out, not about the faces' far
    /// extent.
    ///
    /// The distance is a closed form over the sectors
    /// ([`Link::distance`]), and the sign is the parity reading's, with a
    /// plane read in band outside its sector read as off it
    /// ([`Cone::reads_over`]). Where the parity reading is in band but `q`
    /// lies on a sector within the band ([`Link::touches`]), membership
    /// does not matter at the band's resolution: a point scalar reads the
    /// distance, and an enclosure reads both signs of it.
    ///
    /// # Errors
    ///
    /// Where the parity reading is in band and `q` stands off every
    /// sector, so its magnitude is definite and its sign is not:
    /// [`LinkRefusal::Undecided`] with the reading that was in band, or
    /// [`LinkRefusal::Unread`] where none of `q`'s own readings was.
    pub fn offset(&self, q: Point3<T>, band: Band) -> Result<T, LinkRefusal> {
        let rd = Reading {
            band,
            slack: None,
            own: true,
        };
        match self.reading(q, rd)? {
            (d, Some(true)) => Ok(-d),
            (d, _) => Ok(d),
        }
    }

    /// `q`'s distance from the boundary and whether the material holds
    /// it: `Some(true)` In, `Some(false)` Out or On, `None` where `q`
    /// lies on a sector within the band (the distance then the reading
    /// at the band's resolution, [`Link::offset`]).
    fn reading(&self, q: Point3<T>, rd: Reading<T>) -> Result<(T, Option<bool>), LinkRefusal> {
        let d = self
            .distance(q, rd)
            .ok_or(LinkRefusal::Unread(TouchUnread::Corrupt))?;
        match self.cone.reads_over(q, rd) {
            Some(Material::In) => Ok((d, Some(true))),
            Some(Material::Out | Material::On(_)) => Ok((d, Some(false))),
            None if self.touches(q, rd) => Ok((either(d, -d), None)),
            None => Err(self.refusal(q, rd)),
        }
    }

    /// Why `q`'s parity reading refused: the first of its plane readings,
    /// then of its sector-bound readings, that is in band.
    fn refusal(&self, q: Point3<T>, rd: Reading<T>) -> LinkRefusal {
        let planes = self
            .cone
            .sectors
            .iter()
            .map(|s| Distance::of(q, self.cone.p, s.outward));
        let bounds = self.cone.sectors.iter().flat_map(|s| {
            s.bounds()
                .into_iter()
                .flatten()
                .map(move |r| Distance::of(q, s.apex, s.outward.cross((r - s.apex).normalize())))
        });
        planes
            .map(|d| metric::why(TOUCH_PLANE, d, rd.band))
            .chain(bounds.map(|d| metric::why(TOUCH_WEDGE, d, rd.band)))
            .find_map(|why| why)
            .map_or(
                LinkRefusal::Unread(TouchUnread::InBand),
                LinkRefusal::Undecided,
            )
    }

    /// **The lowest [`Link::offset`] along the segment `x0`–`x1`**, over
    /// a finite closed-form candidate set, with no sampling.
    ///
    /// Along the segment, `x(t) = x0 + t·(x1 − x0)`, the distance from
    /// the boundary is the least of the distances from the cone's
    /// wedges, each convex. Each wedge's distance is in turn the least
    /// of its pieces' (its plane where `x` stands over it, its two rays'
    /// lines ahead of the apex, the apex behind them), each squared a
    /// quadratic in `t`, and it is C1, so where two pieces of ONE wedge
    /// meet nothing turns. Off the material the offset is the distance,
    /// lowest at an end, at a piece's nearest point (a plane's nearest
    /// is its crossing) or where the boundary is met; in it the offset
    /// is minus the distance, lowest at an end or where two wedges stand
    /// equally far. So the candidates are:
    /// - the ends, and each piece's nearest point;
    /// - where two planes stand equally far, the two roots of their
    ///   offsets equal and opposed;
    /// - where a ray and a plane it does not lie in, or two rays, stand
    ///   equally far, every root in `[0, 1]` of their squared distances
    ///   equated, each in both of its forms so that one is stable.
    ///
    /// Two pieces of one wedge are never paired: the apex with anything
    /// (it is a point of every wedge, and nearer than none of them), and
    /// a plane with a ray lying in it. Their squared distances differ by
    /// a square, so the pair's root is double, and only rounding would
    /// read it ([`on_ray`] and the plane reading decide what lies in
    /// what; a ray two sectors share is one piece).
    ///
    /// Every candidate is a real point of the segment, read in full by
    /// [`Link::reading`], so the least value read is a value the segment
    /// takes, and no candidate the minimum is not at can pull it down.
    /// A candidate whose parity refuses takes the membership of a read
    /// candidate inside the ball its distance clears (nothing of the
    /// boundary is in it), read through the door as the read point's
    /// side of the ball's tangent plane toward it; it refuses the
    /// segment only where no read candidate is that near.
    ///
    /// Its cost is quadratic in the link's pieces, the ties being pairs:
    /// tens of microseconds over two sectors, about half a millisecond
    /// over seven (release, random segments). No row holds that, since a
    /// timing row flakes; a link is read at one cell, whose faces are few.
    ///
    /// # Errors
    ///
    /// A candidate's refusal of [`Link::offset`] that no nearer reading
    /// settles, or a link with no wedge.
    pub fn lowest_on(
        &self,
        (x0, x1): (Point3<T>, Point3<T>),
        band: Band,
    ) -> Result<T, LinkRefusal> {
        let e = x1 - x0;
        let two = T::from_f64(2.0);
        let rd = Reading {
            band,
            slack: None,
            own: true,
        };
        // A ray's real points read against one another drawn in.
        let drawn = Reading { own: false, ..rd };
        // Whether a reading joins two points: decided within the band's
        // zero, or, for a segment of points, in band (rounding cannot part
        // what the band does not). An enclosure read in band stands for
        // points the band does not join, so it joins nothing.
        let point = [x0.x, x0.y, x0.z, x1.x, x1.y, x1.z]
            .iter()
            .all(|v| v.point_value().is_some());
        let joined = |d: Distance<T>| match metric::sign(LINK_TIE, d, band) {
            Some(Sign::Zero) => true,
            None => point,
            Some(_) => false,
        };
        // Each piece's squared distance from x(t), as [t², t, 1]
        // coefficients.
        let w = x0 - self.cone.p;
        let apex = [e.norm_squared(), two * w.dot(e), w.norm_squared()];
        // Each plane's offset from x(t), as [t, 1], and where the
        // segment's line meets it within the band.
        let mut planes: Vec<([T; 2], Option<Point3<T>>)> = Vec::new();
        // Each ray's line, the planes it lies in, and where the segment's
        // line meets it within the band.
        let mut rays: Vec<RayPiece<T>> = Vec::new();
        // The point of the segment at `t`, where it meets the piece whose
        // nearest point to it `foot` gives within the band; `None` where
        // it stands decidedly off.
        let meets = |t: T, foot: &dyn Fn(Point3<T>) -> Point3<T>| {
            let x = x0 + e * t.max(T::zero()).min(T::one());
            let f = foot(x);
            joined(Distance::of(x, f, (x - f).normalize())).then_some(x)
        };
        // Each ray once: an edge's ray is a bound of both its faces'
        // sectors.
        let mut kept: Vec<(Point3<T>, Vec3<T>, Point3<T>)> = Vec::new();
        for s in &self.cone.sectors {
            let w = x0 - s.apex;
            let (o1, o0) = (s.outward.dot(e), s.outward.dot(w));
            let foot = |x: Point3<T>| x - s.outward * s.outward.dot(x - s.apex);
            planes.push(([o1, o0], meets(-o0 / o1, &foot)));
            let last = s.wedges.len().saturating_sub(1);
            for (k, &(a, b)) in s.wedges.iter().enumerate() {
                let ends = if k == last { vec![a, b] } else { vec![a] };
                for r in ends {
                    if kept.iter().any(|&(c, m, q)| on_ray(c, m, q, r, drawn)) {
                        continue;
                    }
                    kept.push((s.apex, s.outward, r));
                    let lies_in = self
                        .cone
                        .sectors
                        .iter()
                        .enumerate()
                        .filter(|(_, t)| {
                            metric::sign_over(TOUCH_PLANE, Distance::of(r, t.apex, t.outward), rd)
                                == Some(Sign::Zero)
                        })
                        .map(|(j, _)| j)
                        .collect();
                    let u = (r - s.apex).normalize();
                    let (wp, ep) = (w - u * u.dot(w), e - u * u.dot(e));
                    let f = [ep.norm_squared(), two * wp.dot(ep), wp.norm_squared()];
                    let foot = |x: Point3<T>| s.apex + u * u.dot(x - s.apex);
                    rays.push((f, lies_in, meets(-f[1] / (two * f[0]), &foot)));
                }
            }
        }
        let squared = |[o1, o0]: [T; 2]| [o1.powi(2), two * o0 * o1, o0.powi(2)];
        let along = e.normalize();
        // Two pieces the segment's line meets at one point, within the
        // band's zero, tie only there: a double root already a candidate
        // (where each meets it), which rounding would scatter about it.
        // Only what `joined` joins is pruned.
        let apart = |x: Option<Point3<T>>, y: Option<Point3<T>>| match (x, y) {
            (Some(x), Some(y)) => !joined(Distance::of(x, y, along)),
            _ => true,
        };
        let mut ts = vec![T::zero(), T::one(), -apex[1] / (two * apex[0])];
        for &([o1, o0], _) in &planes {
            ts.push(-o0 / o1);
        }
        for (f, _, _) in &rays {
            ts.push(-f[1] / (two * f[0]));
        }
        for (i, &([a1, a0], _)) in planes.iter().enumerate() {
            for &([b1, b0], _) in &planes[i + 1..] {
                ts.extend([-(a0 - b0) / (a1 - b1), -(a0 + b0) / (a1 + b1)]);
            }
        }
        let mut quadratics: Vec<([T; 3], [T; 3])> = Vec::new();
        for (j, &(plane, at)) in planes.iter().enumerate() {
            for (f, lies_in, meets) in &rays {
                if !lies_in.contains(&j) && apart(at, *meets) {
                    quadratics.push((squared(plane), *f));
                }
            }
        }
        for (i, (f, _, x)) in rays.iter().enumerate() {
            for (g, _, y) in &rays[i + 1..] {
                if apart(*x, *y) {
                    quadratics.push((*f, *g));
                }
            }
        }
        for (f, g) in quadratics {
            let (a, b, c) = (f[0] - g[0], f[1] - g[1], f[2] - g[2]);
            let r = (b.powi(2) - T::from_f64(4.0) * a * c).max(T::zero()).sqrt();
            let roots = [(-b + r) / (two * a), (-b - r) / (two * a)];
            // Two roots within the band's zero of each other along the
            // segment, decided, are one double root, taken in closed form:
            // its square root is all rounding, and so is its derivative.
            let at = |t: T| x0 + e * t.max(T::zero()).min(T::one());
            let twin = roots.iter().all(|t| !t.is_poison() && t.is_bounded())
                && joined(Distance::of(at(roots[0]), at(roots[1]), along));
            if twin {
                ts.push(-b / (two * a));
            } else {
                ts.extend(roots);
                ts.extend([two * c / (-b + r), two * c / (-b - r)]);
            }
        }
        let mut read: Vec<(Point3<T>, T, Option<bool>)> = Vec::new();
        let mut refused: Vec<(Point3<T>, T, LinkRefusal)> = Vec::new();
        for t in ts {
            if t.is_poison() || !t.is_bounded() {
                continue;
            }
            // Where the least is, is structure: a point scalar reads the
            // candidate as the point it is, so a derivative of the least
            // is the offset's at that point (the envelope's), not the
            // root formula's, which a near-double root or a
            // near-degenerate denominator blows up. An enclosure keeps
            // every point it stands for.
            let t = t.point_value().map_or(t, T::from_computed);
            let x = x0 + e * t.max(T::zero()).min(T::one());
            match self.reading(x, rd) {
                Ok((d, Some(true))) => read.push((x, -d, Some(true))),
                Ok((d, held)) => read.push((x, d, held)),
                Err(why) => refused.push((
                    x,
                    self.distance(x, rd)
                        .ok_or(LinkRefusal::Unread(TouchUnread::Corrupt))?,
                    why,
                )),
            }
        }
        for (x, d, why) in refused {
            let held = read.iter().find_map(|&(y, _, held)| {
                let n = (y - x).normalize();
                let inside = metric::sign(LINK_BALL, Distance::of(y, x + n * d, n), band)
                    == Some(Sign::Negative);
                if inside { held } else { None }
            });
            match held {
                Some(true) => read.push((x, -d, held)),
                Some(false) => read.push((x, d, held)),
                None => return Err(why),
            }
        }
        read.into_iter()
            .map(|(_, v, _)| v)
            .reduce(T::min)
            .ok_or(LinkRefusal::Unread(TouchUnread::Corrupt))
    }

    /// **`q`'s distance from the cone's boundary**: the least, over every
    /// wedge of every sector, of `q`'s distance from the wedge
    /// ([`Link::wedge`]). `None` for a link with no wedge.
    fn distance(&self, q: Point3<T>, rd: Reading<T>) -> Option<T> {
        self.cone
            .sectors
            .iter()
            .flat_map(|s| s.wedges.iter().map(move |&w| self.wedge(s, w, q, rd).0))
            .reduce(T::min)
    }

    /// **`q`'s distance from the wedge `(c, a, b)` of sector `s`**, and
    /// which of its pieces is nearest: the plane (`Some(true)`), its two
    /// closed rays (`Some(false)`), or either within the band (`None`).
    ///
    /// A wedge is convex: where `q` stands over it (left of its ray `a`,
    /// right of its ray `b`, about its outward normal) its nearest point is
    /// the foot on its plane, and elsewhere a point of its two closed rays.
    /// Which side of a ray `q` stands on is read at `q`'s own distance
    /// through the door (`TOUCH_WEDGE`). Read in band, `q` stands within
    /// the band of the ray's line; where it stands decidedly behind the
    /// apex along the ray (`TOUCH_AHEAD`) it is beyond the wedge, which lies
    /// under half a turn ahead, and otherwise it stands within the band of
    /// the ray itself, where the two branches differ by at most the band: a
    /// point scalar takes the plane, and an enclosure takes both
    /// ([`either`]). The plane is taken outright in every tie, so a
    /// derivative at contact is a face's one-sided one.
    fn wedge(
        &self,
        s: &Sector<T>,
        (a, b): Segment<T>,
        q: Point3<T>,
        rd: Reading<T>,
    ) -> (T, Option<bool>) {
        let (c, m) = (s.apex, s.outward);
        let ray = |r: Point3<T>| {
            let (u, v) = ((r - c).normalize(), q - c);
            (v - u * u.dot(v).max(T::zero())).norm()
        };
        let side = |r: Point3<T>, inner: Sign| match wedge_side(c, m, r, q, rd) {
            Some(Sign::Zero) | None => match ahead(c, r, q, rd) {
                Some(Sign::Negative) => Some(false),
                _ => None,
            },
            Some(sign) => Some(sign == inner),
        };
        let rays = ray(a).min(ray(b));
        let plane = m.dot(q - c).abs();
        match (side(a, Sign::Positive), side(b, Sign::Negative)) {
            (Some(false), _) | (_, Some(false)) => (rays, Some(false)),
            (Some(true), Some(true)) => (plane, Some(true)),
            _ => (either(plane, rays), None),
        }
    }

    /// **Whether `q` stands within the band of the cone's boundary**: of
    /// a wedge's plane where it stands over the wedge or within the band
    /// of its ray (`TOUCH_PLANE`), or of one of its closed rays, read along
    /// the line from the ray's nearest point to `q` (`LINK_TIE`; at that
    /// point itself the reading is in band). A distance, not a place: a
    /// wedge thinner than the band leaves `q`'s place in its face
    /// undecided however far off it `q` stands.
    fn touches(&self, q: Point3<T>, rd: Reading<T>) -> bool {
        let near = |d: Distance<T>, name| {
            !matches!(
                metric::sign(name, d, rd.band),
                Some(Sign::Positive | Sign::Negative)
            )
        };
        self.cone.sectors.iter().any(|s| {
            s.wedges.iter().any(|&(a, b)| {
                let plane = self.wedge(s, (a, b), q, rd).1 != Some(false)
                    && near(Distance::of(q, s.apex, s.outward), TOUCH_PLANE);
                plane
                    || [a, b].into_iter().any(|r| {
                        let u = (r - s.apex).normalize();
                        let foot = s.apex + u * u.dot(q - s.apex).max(T::zero());
                        near(Distance::of(q, foot, (q - foot).normalize()), LINK_TIE)
                    })
            })
        })
    }
}

/// A ray's line as [`Link::lowest_on`] reads it along a segment: its
/// squared distance's `[t², t, 1]` coefficients, the sectors whose planes
/// hold it, and where the segment's line meets it within the band.
type RayPiece<T> = ([T; 3], Vec<usize>, Option<Point3<T>>);

/// **`lo` and `hi` taken together**: their hull where the scalar holds
/// an enclosure, and `lo` where it holds a point, the caller passing as
/// `lo` the branch a point reads (one within the band of the other).
fn either<T: Decide>(lo: T, hi: T) -> T {
    let both = lo.enclosure_hull(hi);
    if both.is_poison() && !hi.is_poison() {
        lo
    } else {
        both
    }
}

/// **Why a link's reading of a point refused**.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LinkRefusal {
    /// The link refused for a reason it names.
    Unread(TouchUnread),
    /// A reading of the point was in band: the escalation, unaltered.
    Undecided(Indeterminate),
}

impl From<TouchUnread> for LinkRefusal {
    fn from(why: TouchUnread) -> Self {
        Self::Unread(why)
    }
}

/// **A sphere or cylinder face's material, read by its carrier**: the
/// side of the carrier the material is on. A point's offset from it is
/// its signed distance from the carrier, positive away from the
/// material ([`Round::offset`]), which is a face's reading of a vertex
/// or an edge in `Gap` and of another solid's link at a touch inside the
/// face in the census ([`rest::by_round`]).
#[derive(Clone, Copy, Debug)]
pub struct Round<T: Real> {
    at: Axis<T>,
    radius: T,
    /// The material lies on the centre's side: a ball or a pin, not a
    /// socket or a bore.
    convex: bool,
}

/// **One solid's material at a touch point**, as the census reads it:
/// its link there, or, where the point lies strictly inside one sphere or
/// cylinder face of the solid and nothing else of it, that face's
/// material and the face's reach about the point ([`rest::meet`]).
pub(crate) enum Germ<T: Real> {
    Link(Link<T>),
    Round {
        round: Round<T>,
        /// A lower bound on the face's reach about the point: how far its
        /// carrier is its material.
        reach: T,
    },
}

/// **A point exactly on a round carrier's axis, or at its centre, that
/// the scalar moves**: the distance from the axis has a corner there and
/// no derivative, so no value with one is minted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtTheAxis;

impl<T: Real> Germ<T> {
    /// The touch point a link is about; `None` for a round face, which
    /// holds no point of its own.
    pub(crate) fn point(&self) -> Option<Point3<T>> {
        match self {
            Self::Link(link) => Some(link.cone.p),
            Self::Round { .. } => None,
        }
    }
}

/// What a [`Round`]'s carrier is round about.
#[derive(Clone, Copy, Debug)]
enum Axis<T: Real> {
    Centre(Point3<T>),
    Line(Point3<T>, Vec3<T>),
}

impl<T: Decide> Round<T> {
    /// The material of a face on `surface` with `Face::sense` `sense`;
    /// `None` off a sphere or a cylinder.
    pub fn of(surface: &geom::Surface<T>, sense: bool) -> Option<Self> {
        match *surface {
            geom::Surface::Sphere { center, radius, .. } => {
                Some(Self::sphere(center, radius, sense))
            }
            geom::Surface::Cylinder {
                origin,
                axis,
                radius,
                ..
            } => Some(Self::cylinder(origin, axis, radius, sense)),
            _ => None,
        }
    }

    /// A sphere's material, `convex` a ball's.
    pub fn sphere(center: Point3<T>, radius: T, convex: bool) -> Self {
        Self {
            at: Axis::Centre(center),
            radius,
            convex,
        }
    }

    /// A cylinder's material about the line through `origin` along
    /// `axis`, normalised here, `convex` a pin's.
    pub fn cylinder(origin: Point3<T>, axis: Vec3<T>, radius: T, convex: bool) -> Self {
        Self {
            at: Axis::Line(origin, axis.normalize()),
            radius,
            convex,
        }
    }

    /// The carrier's radius.
    pub fn radius(&self) -> T {
        self.radius
    }

    /// Whether the material lies on the centre's side: a ball's or a
    /// pin's, not a socket's or a bore's.
    pub fn convex(&self) -> bool {
        self.convex
    }

    /// `v`'s part off the axis: all of it about a centre, its part
    /// normal to the axis about a line.
    fn across(&self, v: Vec3<T>) -> Vec3<T> {
        match self.at {
            Axis::Centre(_) => v,
            Axis::Line(_, axis) => v - axis * axis.dot(v),
        }
    }

    /// `q` less the centre, or less the axis's origin.
    fn off_centre(&self, q: Point3<T>) -> Vec3<T> {
        match self.at {
            Axis::Centre(c) | Axis::Line(c, _) => q - c,
        }
    }

    /// **`q`'s signed distance from the carrier**, in metres, positive
    /// away from the material: out of a ball or a pin, into a socket's
    /// or a bore's hollow.
    ///
    /// # Errors
    ///
    /// [`AtTheAxis`] where `q` is exactly on the axis or at the centre
    /// and the scalar moves it ([`Round::reach_from_axis`]).
    pub fn offset(&self, q: Point3<T>) -> Result<T, AtTheAxis> {
        let d = self.reach_from_axis(self.across(self.off_centre(q)))? - self.radius;
        Ok(if self.convex { d } else { -d })
    }

    /// **`v`'s length, `v` a point's part off the axis.** Where `v` is
    /// exactly zero the length has a corner, so its derivative is read
    /// as structure: zero where every component is a plain constant
    /// (the point does not move, and the radius alone may), and
    /// [`AtTheAxis`] where a component carries a derivative or a symbol
    /// (the point moves across the axis, where the length has none).
    /// Elsewhere it is the norm.
    fn reach_from_axis(&self, v: Vec3<T>) -> Result<T, AtTheAxis> {
        let parts = [v.x, v.y, v.z];
        if !parts.iter().all(|c| c.point_value() == Some(0.0)) {
            return Ok(v.norm());
        }
        if parts.iter().all(|c| c.plain_constant() == Some(0.0)) {
            Ok(T::zero())
        } else {
            Err(AtTheAxis)
        }
    }

    /// The outward unit normal at `p`, a point of the carrier.
    fn outward(&self, p: Point3<T>) -> Vec3<T> {
        let n = self.across(self.off_centre(p)).normalize();
        if self.convex { n } else { -n }
    }

    /// **The lowest [`Round::offset`] along the segment `x0`–`x1`**, in
    /// closed form. Along the segment the distance from the axis is the
    /// square root of a convex quadratic, so the offset is convex for a
    /// ball or a pin and concave for a socket or a bore. The candidates
    /// are the two ends, which hold a concave least, and the quadratic's
    /// least held to the segment, which holds a convex one; every
    /// candidate is a real point of the segment, so the least read is a
    /// value the segment takes. A segment along a cylinder's axis has no
    /// least: its offset is constant, and an enclosure reads it over the
    /// whole segment.
    ///
    /// # Errors
    ///
    /// [`AtTheAxis`] where a candidate is ([`Round::offset`]).
    pub fn lowest_on(&self, (x0, x1): (Point3<T>, Point3<T>)) -> Result<T, AtTheAxis> {
        let e = x1 - x0;
        let (w, u) = (self.across(self.off_centre(x0)), self.across(e));
        let ends = self.offset(x0)?.min(self.offset(x1)?);
        let t = -w.dot(u) / u.dot(u);
        // Where the least is, is structure, as in [`Link::lowest_on`]: a
        // point scalar reads it as the point it is, and an enclosure
        // with no bounded least reads every point of the segment.
        let t = if t.is_poison() || !t.is_bounded() {
            T::zero().enclosure_hull(T::one())
        } else {
            t.point_value().map_or(t, T::from_computed)
        };
        if t.is_poison() {
            return Ok(ends);
        }
        Ok(ends.min(self.offset(x0 + e * t.max(T::zero()).min(T::one()))?))
    }
}

/// [`Cone::reads_over`] over a bare polygon cone at `p`, each face its
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
            outward: m,
            apex: p,
            wedges: vec![(a, b)],
            closed: false,
        })
        .collect();
    let rd = Reading {
        band,
        slack: None,
        own: false,
    };
    Cone::of(p, sectors).ok()?.reads_over(q, rd)
}

/// **The one door a rest comes through.** [`Rest`](rest::Rest) has a
/// private field and is neither `Copy` nor `Clone`, so nothing outside
/// this module can build one or replay one: a rest exists only where
/// [`judge`](rest::judge) ran the readings over two links, or
/// [`one_material`](rest::one_material) over one, and they held, once.
pub(crate) mod rest {
    use super::{
        Band, Cone, Decide, Link, Material, Place, Point3, Reading, Segment, Sign, TouchUnread,
        TouchVerdict,
    };
    use super::{Distance, Germ, LINK_ROUND, Round, TOUCH_PLANE, metric};
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
    ///    other link's material ([`Cone::reads_over`]).
    /// 3. Otherwise **Rest**.
    ///
    /// Both links are first held to close ([`Cone::closes`]), and every
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
    /// pieces' real points, each link's as far out as its faces reach,
    /// or, on a cleared sector, past every point of its face
    /// ([`super::FaceSector::cleared`]). A rest needs every one decided there: a
    /// Zero read at a far point is a Zero at every nearer one, so that is
    /// where the fewest coincidences are taken. A crossing is confirmed
    /// inside the BALL where both links are their solids — out to the
    /// nearest far side of any wedge of either, or a cleared sector's
    /// radius ([`Link::ball`]) — with the piece's points
    /// drawn in along their rays ([`within`]), still real points of its
    /// face: an overlap read there is one of the two bodies, not of one
    /// link's planes carried past the other's faces. A crossing read
    /// only further out refuses in band.
    ///
    /// A crossing is claimed on decided readings only; any other
    /// reading the test needed in band, or a sector whose pieces did not
    /// cut, leaves the touch [`TouchUnread::InBand`] (or the cut's own
    /// refusal). A piece is SKIPPED, read for no crossing and no
    /// overlap, where its probe reads on the plane of another sector of
    /// its own link (`side == Some(Zero)`) and that sector's `place` of
    /// it is `Inside` or `Bound` (a slit, or a face doubled: a fin, with
    /// no material behind it), which notes the touch
    /// [`TouchUnread::Degenerate`], or `None`, which notes it
    /// [`TouchUnread::InBand`]. A skipped piece costs completeness, not
    /// soundness: the argument for (1) and (2) holds of every piece read,
    /// and a touch with a skipped piece never reads as a rest, so a
    /// crossing that only a skipped piece would show refuses rather than
    /// clears.
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
        let rd = reading(a, b, band);
        if let Some(open) = held_closed(&[a, b], rd) {
            return open;
        }
        let mut notes = Notes::default();
        let mut refused = None;
        let ball = a.ball.min(b.ball);
        let rays: Vec<_> = a.cone.rays().into_iter().chain(b.cone.rays()).collect();
        let mut all: [Vec<(usize, Vec<Segment<T>>)>; 2] = [Vec::new(), Vec::new()];
        for (side, link) in [a, b].into_iter().enumerate() {
            for (k, s) in link.cone.sectors.iter().enumerate() {
                match link.pieces(s, &rays, rd) {
                    Ok(pieces) => all[side].push((k, pieces)),
                    Err(v) => {
                        refused.get_or_insert(v);
                    }
                }
            }
        }
        for (side, (own, other)) in [(a, b), (b, a)].into_iter().enumerate() {
            let near = |q| within(own.cone.p, q, ball);
            let cut: Vec<usize> = all[1 - side].iter().map(|(k, _)| *k).collect();
            for (k, pieces) in &all[side] {
                for &(c0, c1) in pieces {
                    let probe = c0 + (c1 - c0) * T::from_f64(0.5);
                    // On another face of its own link (a slit or a face
                    // doubled, no boundary the probe can stand on and no
                    // material behind it), or in band of one: the piece
                    // is skipped, and the touch never reads as a rest.
                    let mut on_own = false;
                    for (j, t) in own.cone.sectors.iter().enumerate() {
                        if j == *k || own.cone.side(t, probe, rd) != Some(Sign::Zero) {
                            continue;
                        }
                        match t.place(own.cone.p, probe, rd) {
                            Some(Place::Outside) => {}
                            Some(Place::Inside | Place::Bound) => {
                                notes.degenerate = true;
                                on_own = true;
                            }
                            None => {
                                notes.in_band = true;
                                on_own = true;
                            }
                        }
                    }
                    if on_own {
                        continue;
                    }
                    if crosses(&other.cone, &cut, &rays, (c0, c1), rd, &mut notes) {
                        let mut scratch = Notes::default();
                        if crosses(
                            &other.cone,
                            &cut,
                            &rays,
                            (near(c0), near(c1)),
                            rd,
                            &mut scratch,
                        ) {
                            return TouchVerdict::Crossing;
                        }
                        notes.in_band = true;
                    }
                    // Every point strictly inside the piece reads its one
                    // cell, so any one that decides is the piece's
                    // reading, tried along the far side ([`ladder`]).
                    let read = ladder::<T>().into_iter().find_map(|t| {
                        let q = c0 + (c1 - c0) * t;
                        other.cone.reads_over(q, rd).map(|m| (q, m))
                    });
                    let full = read.map(|(_, m)| m);
                    let overlaps = match full {
                        Some(Material::In) => true,
                        Some(Material::Out) => false,
                        Some(Material::On(j)) => {
                            let (s, t) = (&own.faces[*k], &other.faces[j]);
                            match pairing(
                                TOUCH_NORMAL,
                                (&s.surface, s.sense),
                                (&t.surface, t.sense),
                                own.cone.p,
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
                        if other.cone.reads_over(near(q), rd) == full {
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

    /// **A polygonal cone against a round face**, at a touch point `p`
    /// strictly inside a sphere or cylinder face of the other solid: the
    /// link `a` of one solid, and the face's material `round` of the
    /// other, which is its carrier's side within `reach` of `p` (the
    /// face's own reach about `p`, as [`Link::ball`] is a link's). `near`
    /// is the lesser of the two. With `n` the face's outward normal at
    /// `p`, each ray of `a` — every wedge's two ends, read as directions
    /// of the cone, its points beyond `near` real points of a face or
    /// not (a cleared sector's rim runs past its face) — reads:
    ///
    /// - against a ball or a pin: Zero or above against the tangent plane
    ///   through `p`, so its wedges keep out; below it, a crossing where
    ///   its segment from the apex, drawn in to `near`, reads below the
    ///   carrier ([`Round::lowest_on`]);
    /// - against a socket or a bore: its point drawn in to `near`, Zero or
    ///   above against the carrier, so its wedges keep out; below it, a
    ///   crossing.
    ///
    /// Then the probe `p − n·s`, `s` the lesser of `near` and the
    /// radius, inside the face's material: **Crossing** where `a` reads
    /// it In, **Rest** where every ray kept out and it reads Out, and
    /// [`TouchUnread::InBand`] otherwise.
    ///
    /// **Sound.** Against a ball or a pin, the carrier's signed distance
    /// is at least the tangent plane's (the material is convex and the
    /// plane supports it), and a wedge's plane offset is affine over its
    /// triangle, so a wedge whose rays read Zero or above stands no
    /// deeper than the band anywhere it reaches. Against a socket or a
    /// bore the hollow is convex, so a wedge whose rays' points drawn in
    /// to `near` lie in it (within the band) holds its triangle with
    /// them, and that triangle covers the wedge out to `near·cos(θ/2)`
    /// from the apex, `θ` the wedge's angle. A ray that enters the
    /// hollow at first order can still meet the wall again across it,
    /// within `near`; that point reads below the carrier, and is a
    /// crossing. So `a`'s boundary keeps out of the face's material
    /// within that radius of `p`, positive, where the face's material is
    /// its carrier's side. That material is connected there, so it lies
    /// wholly inside `a`'s or wholly outside, and the probe reads which.
    /// A crossing is a point of `a`'s boundary decidedly inside the
    /// face's material within `near`, or the probe decidedly inside both
    /// materials.
    ///
    /// A reading the carrier has no value for ([`AtTheAxis`](super::AtTheAxis))
    /// reads in band. Every reading is taken over twice `a`'s tie, as
    /// [`judge`] takes its own.
    pub(crate) fn by_round<T: Decide>(
        a: &Link<T>,
        round: &Round<T>,
        reach: T,
        band: Band,
    ) -> TouchVerdict {
        let rd = Reading {
            band,
            slack: Some(a.cone.tie * T::from_f64(2.0)),
            own: false,
        };
        if let Some(open) = held_closed(&[a], rd) {
            return open;
        }
        let p = a.cone.p;
        let n = round.outward(p);
        let near = a.ball.min(reach);
        let below =
            |seg| Distance::off(round, seg).and_then(|d| metric::sign_over(LINK_ROUND, d, rd));
        let mut in_band = false;
        for s in &a.cone.sectors {
            for r in s.wedges.iter().flat_map(|&(x, y)| [x, y]) {
                let drawn = within(s.apex, r, near);
                let enters = if round.convex {
                    match metric::sign_over(TOUCH_PLANE, Distance::of(r, p, n), rd) {
                        Some(Sign::Positive | Sign::Zero) => continue,
                        Some(Sign::Negative) => below((s.apex, drawn)),
                        None => None,
                    }
                } else {
                    match below((drawn, drawn)) {
                        Some(Sign::Positive | Sign::Zero) => continue,
                        other => other,
                    }
                };
                if enters == Some(Sign::Negative) {
                    return TouchVerdict::Crossing;
                }
                in_band = true;
            }
        }
        let probe = p - n * near.min(round.radius());
        match a.cone.reads_over(probe, rd) {
            Some(Material::In) => TouchVerdict::Crossing,
            Some(Material::Out) if !in_band => TouchVerdict::Rest(Rest { _sealed: () }),
            _ => TouchVerdict::Unread(TouchUnread::InBand),
        }
    }

    /// **The one test of two solids' materials at a touch point**: two
    /// links by [`judge`], a link and a round face by [`by_round`]. Two
    /// round faces are a face pair, which the touch analysis does not
    /// read ([`TouchUnread::Unreadable`]).
    pub(crate) fn meet<T: Decide>(a: &Germ<T>, b: &Germ<T>, band: Band) -> TouchVerdict {
        match (a, b) {
            (Germ::Link(x), Germ::Link(y)) => judge(x, y, band),
            (Germ::Link(x), Germ::Round { round, reach })
            | (Germ::Round { round, reach }, Germ::Link(x)) => by_round(x, round, *reach, band),
            (Germ::Round { .. }, Germ::Round { .. }) => {
                TouchVerdict::Unread(TouchUnread::Unreadable)
            }
        }
    }

    /// **Whether one solid's link at a point where it touches itself
    /// bounds one material** (D1 tier 3, `crates/topo/README.md`): no two
    /// of its sheets cross, and its winding over the sphere of directions
    /// takes two consecutive values. Each sector is cut into pieces at
    /// every ray of the link, as [`judge`] cuts them. Then:
    ///
    /// 1. **Crossing** if a piece crosses another sector of the link off
    ///    the link's rays ([`crosses`], the link read against itself less
    ///    the piece's own sector), confirmed inside the ball.
    /// 2. Otherwise **Crossing** if a ray of the link — any ray a wedge of
    ///    any sector ends on, cuts included — lies strictly inside two
    ///    sectors, confirmed inside the ball: two sheets run on across it.
    /// 3. Otherwise **Crossing** if two pieces read different windings
    ///    just in front of them ([`Cone::winds`], probed along the
    ///    [`ladder`]), the same difference read again with both probes
    ///    drawn into the ball.
    /// 4. Otherwise **Rest** where every probe is tied to the first by
    ///    readings of difference zero, or the link is one sector (one
    ///    region in front of it and one behind).
    ///
    /// **Sound.** No sheet crosses a piece off the rays it is cut at (1),
    /// nor along one (2), so each piece has one region in front of it and
    /// one behind, the one behind winding one more. Every region borders a
    /// piece, so a common winding `m` in front of every piece leaves every
    /// region `m` or `m + 1`. **Complete.** Two sheets crossing meet along
    /// a line inside both. Off the link's rays, a piece of one straddles
    /// the other there, which (1) reads. On one, that ray lies strictly
    /// inside both sectors, which (2) reads: the rays are every wedge's
    /// ends, so every cut and every ray a fan turns on is one of them. Two
    /// values in front of two pieces are three values between them, which
    /// (3) reads on any reading that decides. Sheets that end on one ray
    /// from either side (the faces of coincident edges) do not cross
    /// there by this reading; how their wedges pair is the line rule's.
    ///
    /// A probe on another sector of the link (a slit, or a sheet doubled)
    /// refuses [`TouchUnread::Degenerate`]; a reading in band, or a probe
    /// no decided reading ties to the others, [`TouchUnread::InBand`].
    /// The link is first held to close ([`Cone::closes`]).
    pub(crate) fn one_material<T: Decide>(link: &Link<T>, band: Band) -> TouchVerdict {
        // The link is read against its own planes and rays: the reading
        // over the pair it makes with itself.
        let rd = reading(link, link, band);
        if let Some(open) = held_closed(&[link], rd) {
            return open;
        }
        let mut notes = Notes::default();
        let near = |q| within(link.cone.p, q, link.ball);
        let rays = link.cone.rays();
        let mut probes: Vec<(usize, Segment<T>)> = Vec::new();
        for (k, s) in link.cone.sectors.iter().enumerate() {
            let pieces = match link.pieces(s, &rays, rd) {
                Ok(pieces) => pieces,
                Err(v) => return TouchVerdict::Unread(v),
            };
            let others: Vec<usize> = (0..link.cone.sectors.len()).filter(|&j| j != k).collect();
            for (c0, c1) in pieces {
                if crosses(&link.cone, &others, &rays, (c0, c1), rd, &mut notes) {
                    let mut scratch = Notes::default();
                    if crosses(
                        &link.cone,
                        &others,
                        &rays,
                        (near(c0), near(c1)),
                        rd,
                        &mut scratch,
                    ) {
                        return TouchVerdict::Crossing;
                    }
                    notes.in_band = true;
                }
                let probe = c0 + (c1 - c0) * T::from_f64(0.5);
                for &j in &others {
                    let t = &link.cone.sectors[j];
                    if link.cone.side(t, probe, rd) != Some(Sign::Zero) {
                        continue;
                    }
                    match t.place(link.cone.p, probe, rd) {
                        Some(Place::Outside) => {}
                        Some(Place::Inside | Place::Bound) => notes.degenerate = true,
                        None => notes.in_band = true,
                    }
                }
                probes.push((k, (c0, c1)));
            }
        }
        if notes.degenerate {
            return TouchVerdict::Unread(TouchUnread::Degenerate);
        }
        // Two sheets crossing along a ray of the link: every ray a wedge
        // of any sector ends on (a sector's bound, so a cut, or a ray two
        // of its wedges share) is read against every sector, and two
        // sectors running on across it, the ray strictly inside both,
        // cross there. No piece crosses a sheet along such a ray (each
        // piece ends on it, and a cut is exempt), so this is the one place
        // a crossing on a ray is read. A sheet that ends at the ray has it
        // for a bound and only meets the others there.
        for r in link
            .cone
            .sectors
            .iter()
            .flat_map(|s| s.wedges.iter().flat_map(|&(a, b)| [a, b]))
        {
            let mut through = 0usize;
            let mut confirmed = 0usize;
            for t in &link.cone.sectors {
                if link.cone.side(t, r, rd) != Some(Sign::Zero) {
                    continue;
                }
                match (
                    t.place(link.cone.p, r, rd),
                    t.place(link.cone.p, near(r), rd),
                ) {
                    (Some(Place::Inside), near_place) => {
                        through += 1;
                        if near_place == Some(Place::Inside) {
                            confirmed += 1;
                        }
                    }
                    (None, _) => notes.in_band = true,
                    _ => {}
                }
            }
            if confirmed >= 2 {
                return TouchVerdict::Crossing;
            }
            if through >= 2 {
                notes.in_band = true;
            }
        }
        // One sheet alone (a crack's tip, its sector the full turn) has
        // one region in front of it and one behind: no arc between its
        // pieces leaves its plane, and none need be read.
        if link.cone.sectors.len() == 1 && !notes.in_band {
            return TouchVerdict::Rest(Rest { _sealed: () });
        }
        // Tie every probe to the first, each through any probe already
        // tied: the windings in front of them all equal, or two that
        // differ. A pair is read along both pieces ([`ladder`]): where it
        // reads, it reads the pieces' one cell each.
        let ladder = ladder::<T>();
        let at = |(k, (c0, c1)): (usize, Segment<T>), t: T| (k, c0 + (c1 - c0) * t);
        let mut tied = vec![false; probes.len()];
        if let Some(first) = tied.first_mut() {
            *first = true;
        }
        let mut grew = true;
        while grew {
            grew = false;
            for j in 0..probes.len() {
                if tied[j] {
                    continue;
                }
                for i in 0..probes.len() {
                    if !tied[i] {
                        continue;
                    }
                    let read = ladder.iter().find_map(|&tj| {
                        ladder.iter().find_map(|&ti| {
                            let (q, w) = (at(probes[j], tj), at(probes[i], ti));
                            link.cone.winds(q, w, rd).map(|d| (d, q, w))
                        })
                    });
                    match read {
                        Some((0, ..)) => {
                            tied[j] = true;
                            grew = true;
                            break;
                        }
                        Some((d, (on, q), (from, w))) => {
                            if link.cone.winds((on, near(q)), (from, near(w)), rd) == Some(d) {
                                return TouchVerdict::Crossing;
                            }
                            notes.in_band = true;
                        }
                        None => {}
                    }
                }
            }
        }
        if notes.in_band || tied.contains(&false) {
            TouchVerdict::Unread(TouchUnread::InBand)
        } else {
            TouchVerdict::Rest(Rest { _sealed: () })
        }
    }

    /// The reading over two links, `a` and `b`: one link's point read
    /// against the other's planes and rays is moved by both their ties
    /// ([`judge`]'s first paragraph), so twice their sum covers it.
    fn reading<T: Decide>(a: &Link<T>, b: &Link<T>, band: Band) -> Reading<T> {
        Reading {
            band,
            slack: Some((a.cone.tie + b.cone.tie) * T::from_f64(2.0)),
            own: false,
        }
    }

    /// The refusal of the first of `links` that does not close
    /// ([`Cone::closes`]), or `None` where every one does.
    fn held_closed<T: Decide>(links: &[&Link<T>], rd: Reading<T>) -> Option<TouchVerdict> {
        links
            .iter()
            .find_map(|link| link.cone.held_closed(rd).err().map(TouchVerdict::Unread))
    }

    /// Where along a piece a reading is taken: its middle, then its
    /// quarter and sixteenth points from each end. A point stands in band
    /// of a plane only on a stretch of the piece (the distance is affine
    /// along it), and these sit at five distinct places, nearer each end
    /// in turn. The ladder chooses only how often a piece refuses, never
    /// what one that reads says.
    fn ladder<T: Decide>() -> [T; 5] {
        [
            T::from_f64(0.5),
            T::from_f64(0.25),
            T::from_f64(0.75),
            T::from_f64(0.0625),
            T::from_f64(0.9375),
        ]
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
        other: &Cone<T>,
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
mod cone_oracle;
#[cfg(test)]
mod oracle_rows;
#[cfg(test)]
mod round_rows;

#[cfg(test)]
mod cleared_rows;

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
                FaceSector::planar(Sector {
                    outward: *m,
                    apex: p,
                    wedges: pts.windows(2).map(|w| (w[0], w[1])).collect(),
                    closed: false,
                })
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

    /// **A link built off a live body reads its cell**: a unit cube's
    /// corner at the origin (material `x, y, z ≥ 0`), inside its edge
    /// along `z` at `(0, 0, 0.5)`, and inside its top face at
    /// `(0.5, 0.5, 1)`, each reading the offset of the cube's own
    /// material. Red if a builder gathers a wrong face, a face's sense
    /// flips the material, or a builder refuses a cube.
    #[test]
    fn a_link_off_a_body_reads_its_cell() {
        let cube = crate::test_support_fixtures::geometric_cube::<f64>(geom_core::Tol::witness());
        let body = &cube.body;
        let at = |q: Point3<f64>| {
            body.vertices
                .keys()
                .find(|&v| position(body, v).is_ok_and(|x| (x - q).norm() < 1e-12))
                .expect("a vertex there")
        };
        let corner = Link::at_point(body, at(O), band()).expect("the corner's link");
        close(
            offset(&corner, p(-0.01, 0.02, 0.03)),
            0.01,
            "off the corner",
        );
        close(offset(&corner, p(0.01, 0.02, 0.03)), -0.01, "in the cube");
        let (v0, v1) = (at(O), at(p(0.0, 0.0, 1.0)));
        let strut = body
            .edges
            .iter()
            .find(|(_, e)| {
                let ends = [
                    body.half_edges[e.he_plus].start,
                    body.half_edges[e.he_minus].start,
                ];
                ends.contains(&v0) && ends.contains(&v1)
            })
            .map(|(k, _)| k)
            .expect("the strut along z");
        let edge = Link::on_edge(body, strut, p(0.0, 0.0, 0.5), band()).expect("the strut's link");
        close(offset(&edge, p(-0.01, 0.2, 0.9)), 0.01, "off the strut");
        close(
            offset(&edge, p(0.03, 0.02, 0.9)),
            -0.02,
            "beside the strut, in the cube",
        );
        let top = body
            .faces()
            .map(|(k, _)| k)
            .find(|&f| {
                crate::face_normal::face_outward_normal(body, f)
                    .is_some_and(|n| (n.vec() - v(0.0, 0.0, 1.0)).norm() < 1e-12)
            })
            .expect("the top face");
        let face = Link::in_face(body, top, p(0.5, 0.5, 1.0), band()).expect("the top's link");
        close(offset(&face, p(0.9, 0.1, 1.02)), 0.02, "over the top");
        close(offset(&face, p(0.9, 0.1, 0.97)), -0.03, "under it");
    }

    /// **A point in band of a sector's plane that lies decidedly outside
    /// the sector reads as off that plane**, for the census's reading and
    /// `offset`'s alike: over a block's corner, a point half a metre off
    /// both side faces and in the band's sliver of the top face's plane is
    /// far outside the material ([`Cone::reads_over`]). Red if either
    /// refuses it, or reads it anything but outside.
    #[test]
    fn a_point_in_a_planes_sliver_outside_its_sector_reads_out() {
        let b = band();
        let z = 0.5 * (b.zero() + b.escalate());
        let q = p(0.5, 0.5, z);
        let block = corner(false);
        assert_eq!(
            block.reads(q, b),
            Some(Material::Out),
            "the census's reading"
        );
        close(
            offset(&block, q),
            0.5_f64.hypot(0.5).hypot(z),
            "the offset reads the distance from the apex, outside",
        );
    }

    /// **A direction along a bound two sectors share reads `On`**, one
    /// of the sectors whose plane holds it: along each edge of a block's
    /// corner, a hole's and a ridge. Red if a direction on the material's
    /// boundary refuses, reads In or Out, or names a sector it is off.
    #[test]
    fn a_direction_along_a_shared_bound_reads_on() {
        let b = band();
        let rows = [
            (
                "block",
                corner(false),
                vec![p(-1.0, 0.0, 0.0), p(0.0, -2.0, 0.0), p(0.0, 0.0, -0.5)],
            ),
            (
                "hole",
                corner(true),
                vec![p(1.0, 0.0, 0.0), p(0.0, 2.0, 0.0), p(0.0, 0.0, 0.5)],
            ),
            ("ridge", ridge(), vec![p(0.0, 1.0, 0.0), p(0.0, -3.0, 0.0)]),
        ];
        for (what, link, dirs) in rows {
            for q in dirs {
                match link.reads(q, b) {
                    Some(Material::On(k)) => close(
                        link.cone.sectors[k].outward.dot(q - O),
                        0.0,
                        &format!("{what} at {q:?}: the sector it is on holds it"),
                    ),
                    got => panic!("{what} at {q:?}: reads {got:?}, not On"),
                }
            }
        }
    }

    /// **The link decides only through its doors** (S4 of `CONTACT-7`'s
    /// design). A levered reading of directions decides a verdict right
    /// wherever it is definite and wrong where its Zero is taken as one:
    /// the class five review rounds found one site at a time. So every
    /// sign a link decides comes through [`metric::sign`], whose
    /// [`metric::Distance`] is built only from a point and a plane, or a
    /// segment and a round carrier. This
    /// reads this module's source (all but its tests) and the census's
    /// touch analysis (`census.rs`, from its banner to the backstop's
    /// header), and holds:
    /// - every `Margin` door and every `decide` call lies inside `mod
    ///   metric` (one `Margin::of`);
    /// - the classifier of unit normals is called only from `pairing`;
    /// - nothing reads a bare sign: no `sign_within`, no
    ///   `definitely_positive(` (`validate`'s unnamed door), no
    ///   `enclosure_probe(` (an instrument nothing may branch on), no
    ///   `is_sign_negative`, `is_sign_positive`, `signum`, `copysign`,
    ///   `select_le_zero`, `partial_cmp` or `total_cmp`, and no
    ///   `classify_dihedral` (a levered classifier of its own). No code
    ///   needs one today, so the list has no exemption;
    /// - `Distance` is built in exactly two places: its field is private,
    ///   `mod metric` holds exactly two `Self(` — one inside
    ///   `Distance::of`, and one inside `Distance::off`, which is
    ///   `Round::lowest_on` and nothing else — and no `Distance(`, and
    ///   those are the module's only functions returning a `Distance`. So
    ///   a product or a lever has no way to become one, not even through
    ///   a third constructor.
    ///
    /// - structure (`is_poison(`, `is_bounded(`, `point_value(`,
    ///   `plain_constant(`, `enclosure_hull(`) is read only at twelve
    ///   named call sites, each
    ///   occurring once: four in `Link::lowest_on` (a candidate that is no
    ///   bounded number is skipped; a candidate's location read as a
    ///   point; a band joining two points of a point segment), four in
    ///   `Round::lowest_on` (the same skip and location, the whole segment
    ///   standing in where an enclosure has no bounded least, and a point
    ///   scalar then reading its ends), two in `Round::reach_from_axis`
    ///   (a point exactly on the axis, and whether the scalar moves it),
    ///   and two in `either` (a hull where the scalar holds one);
    /// - no text is made of a number: no `format!(`, `format_args!`,
    ///   `write!`, `writeln!`, `write_fmt` or `to_string(`.
    ///
    /// **What it cannot see.**
    /// - A sign decided through structure at a named site. The sites are
    ///   pinned by their text, so a new `(-x).sqrt().is_poison()` or
    ///   `x.point_value().is_some_and(|v| v < 0.0)` anywhere goes red; but
    ///   what a site is handed is not: `t` at the skip could be built from
    ///   a sign upstream, and only reading tells a skip from a decision.
    /// - A sign spelled through a needle-free route: a `Debug` string
    ///   reached through a helper outside this module and the census's
    ///   touch analysis, or a trait method under another name that reads
    ///   a sign (`Into<f64>` and a comparison, say).
    /// - A comparison of two reals: `Real` has no order, so `<` on a `T`
    ///   does not compile and needs no needle, but a value read out to
    ///   `f64` and compared would.
    /// - What `Distance::of` is handed. It takes the plane's normal on
    ///   trust: a normal scaled by a length is a lever spelled as a plane,
    ///   and a point built as `p + (q − p)·k` is a product spelled as a
    ///   point. Every caller passes a face's outward normal or a
    ///   normalized direction, and a real face point, a point
    ///   interpolated on a face's edge, a point of a segment `offset` is
    ///   asked about, or the point a refused candidate's ball reaches
    ///   toward a read one (`lowest_on`); that is checked by reading.
    /// - A caller outside topo: `Gap`'s cell arms only call
    ///   [`Link::offset`] and [`Link::lowest_on`].
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
        let lowest_on = body("\n    pub fn lowest_on(\n");
        let round_lowest = body(
            "\n    pub fn lowest_on(&self, (x0, x1): (Point3<T>, Point3<T>)) -> Result<T, AtTheAxis>",
        );
        let either = body("\nfn either<");
        let reach_from_axis = body("\n    fn reach_from_axis(");
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
            "definitely_positive(",
            "enclosure_probe(",
            "classify_dihedral",
            "is_sign_negative",
            "is_sign_positive",
            "signum",
            "partial_cmp",
            "total_cmp",
            "copysign",
            "select_le_zero",
            "format!(",
            "format_args!",
            "write!",
            "writeln!",
            "write_fmt",
            "to_string(",
        ] {
            stray.extend(outside(module, needle, &[]));
            stray.extend(outside(touch, needle, &[]));
        }
        // Structure, read only at these call sites, each where a candidate
        // is skipped or a branch hulled, never where a sign is decided.
        // Each must occur once in its body, so a second call goes red.
        let sites: Vec<core::ops::Range<usize>> = [
            (&lowest_on, ".all(|v| v.point_value().is_some());"),
            (
                &lowest_on,
                "roots.iter().all(|t| !t.is_poison() && t.is_bounded())",
            ),
            (&lowest_on, "if t.is_poison() || !t.is_bounded() {"),
            (
                &lowest_on,
                "let t = t.point_value().map_or(t, T::from_computed);",
            ),
            (
                &round_lowest,
                "let t = if t.is_poison() || !t.is_bounded() {",
            ),
            (&round_lowest, "T::zero().enclosure_hull(T::one())"),
            (&round_lowest, "t.point_value().map_or(t, T::from_computed)"),
            (&round_lowest, "if t.is_poison() {"),
            (
                &reach_from_axis,
                "if !parts.iter().all(|c| c.point_value() == Some(0.0)) {",
            ),
            (
                &reach_from_axis,
                "if parts.iter().all(|c| c.plain_constant() == Some(0.0)) {",
            ),
            (&either, "let both = lo.enclosure_hull(hi);"),
            (&either, "if both.is_poison() && !hi.is_poison() {"),
        ]
        .into_iter()
        .map(|(r, site)| {
            let found: Vec<usize> = module[r.clone()]
                .match_indices(site)
                .map(|(i, _)| r.start + i)
                .collect();
            assert_eq!(found.len(), 1, "the structure site `{site}`, once");
            found[0]..found[0] + site.len()
        })
        .collect();
        let sites: Vec<&core::ops::Range<usize>> = sites.iter().collect();
        for needle in [
            "is_poison(",
            "is_bounded(",
            "point_value(",
            "plain_constant(",
            "enclosure_hull(",
        ] {
            stray.extend(outside(module, needle, &sites));
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
        let off = body("\n        pub(crate) fn off(");
        assert!(
            metric.contains(&of.start) && metric.contains(&off.start),
            "`Distance::of` and `Distance::off` are in `mod metric`"
        );
        let builds: Vec<usize> = module
            .match_indices("Self(")
            .chain(module.match_indices("Distance("))
            .map(|(i, _)| i)
            .filter(|i| metric.contains(i))
            .collect();
        assert!(
            builds.len() == 2 && of.contains(&builds[0]) && off.contains(&builds[1]),
            "`Distance` is built twice, once inside `Distance::of` from a plane and once \
             inside `Distance::off` from a round carrier: {} builds",
            builds.len()
        );
        assert!(
            door[off.start - metric.start..off.end - metric.start]
                .contains("match round.lowest_on(seg) {\n                Ok(d) => Some(Self(d)),"),
            "`Distance::off` is the round carrier's closed form, and nothing else"
        );
        assert_eq!(
            door.matches("-> Self").count()
                + door.matches("-> Option<Self>").count()
                + door.matches("-> Distance").count(),
            2,
            "`Distance::of` and `Distance::off` are `mod metric`'s only functions returning a \
             `Distance`"
        );
    }
}

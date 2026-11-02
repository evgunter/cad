//! **A coincident face is divided where its region's fate changes**
//! (`[topology.records]`' lineage clause: one fate per cell).
//!
//! Where two operands' faces coincide and the Eq. 15.3 lump keeps one
//! copy of their shared region, the region of the other copy, the
//! dropped one, has one of two fates: *substituted* (the result holds
//! its interior through the kept copy) or *removed*. The classifier's
//! sites read both: a vertex–vertex site where an edge bounding the
//! kept copy's sector runs into the dropped copy's sector
//! (`recl::recl_sectors`), and a vertex-on-face site where a lumped
//! sector of the kept copy lies on the pierced dropped face
//! (`vtxfac::classify_vertex_on_face`). Each records the kept edge
//! ([`FateEdge`]): on its one side the dropped face is substituted, on
//! its other side not.
//!
//! After the join, or once a containment fallback has built its result
//! (`ops::fallback_fates`), [`cut`] divides the dropped face along each
//! such edge with a one-sided chord, between the vertices the dropped
//! operand holds at the edge's two ends: the partner of a vertex–vertex
//! site; the ring vertex of a pierce; where no record names a partner of
//! the end in the face, a vertex of the face's division at its point
//! (the ring vertex of another kept vertex's pierce there, say), read by
//! where the point lies; or, where the face holds no vertex there (a
//! vertex-on-face site the classification did no surgery at, or a
//! partner of the dropped operand's own resting on the face
//! unimprinted), a lone vertex minted there as the ring door mints one.
//! No null edge is paired with the chord: it lies in a face the
//! selection drops, so no section runs along it. Where
//! the dropped face already holds an edge along the kept edge (a
//! section's chord), it is divided there already and nothing is cut.
//!
//! A site whose dropped sector lies wholly on the kept copy
//! ([`FateCorner`]) needs no cut: its corner's fragment is substituted.
//!
//! The product ([`FateCuts`]) is each substituted fragment with the
//! kept face holding it, and each chord with the kept edge it runs
//! along, which the carve reads for the removed fragment's border
//! (`finish::discarded`).

use geom_brep::LeverBounds;
use geom_core::{Band, Decide, Margin, Point3, Sign, Tol, Vec3};

use super::sectors::reading_lever;
use super::{BooleanError, BooleanReduction, NullCopies, Operand};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, VertexKey};
use crate::euler::{FaceSurface, MefSite};
use crate::euler_ring::MekrSite;
use crate::lineage::FaceDivisions;

/// An edge of a kept coincident copy that bounds its region inside the
/// dropped copy (module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct FateEdge {
    /// The operand of the kept copy, whose clone holds `edge`.
    pub holder: Operand,
    /// The edge, in `holder`'s clone keys.
    pub edge: EdgeKey,
    /// The kept copy's face beside it, in `holder`'s clone keys.
    pub kept: FaceKey,
    /// The dropped copy, in the other operand's clone keys as the
    /// classification read them.
    pub face: FaceKey,
    /// Whether the two copies' outward normals agree.
    pub aligned: bool,
}

/// A sector the classification read at a vertex, kept as it read it: the
/// join divides its face and re-links its vertex after, so neither the
/// half-edge's face then nor the face's own key names the piece.
#[derive(Debug, Clone, Copy)]
pub(crate) struct SectorAt<T: geom_core::Real> {
    /// The orbit half-edge it follows, which starts at its vertex.
    pub he: HalfEdgeKey,
    /// Its face, as the classification read it.
    pub face: FaceKey,
    /// A unit direction inside it.
    pub dir: Vec3<T>,
}

/// Where a kept copy holds a dropped sector's region.
#[derive(Debug, Clone, Copy)]
pub(crate) enum KeptAt<T: geom_core::Real> {
    /// At the kept copy's own sector, in the holder's clone, its
    /// direction the dropped sector's, which the kept one holds: the
    /// kept sector's own direction may lie in a piece of its face the
    /// join divides off.
    Sector(SectorAt<T>),
    /// On the pierced face, in the holder's clone keys, at the dropped
    /// operand's piercing vertex, the dropped sector's interior along
    /// the unit direction.
    Pierced(FaceKey, VertexKey, Vec3<T>),
}

impl<T: geom_core::Real> KeptAt<T> {
    /// The kept face, in the holder's clone keys, as the classification
    /// read it.
    pub(crate) fn face(&self) -> FaceKey {
        match *self {
            Self::Sector(s) => s.face,
            Self::Pierced(f, ..) => f,
        }
    }
}

/// A dropped sector the kept copy holds wholly (module docs).
#[derive(Debug, Clone, Copy)]
pub(crate) struct FateCorner<T: geom_core::Real> {
    /// The operand of the kept copy.
    pub holder: Operand,
    /// The dropped sector, in the other operand's clone: a wide corner
    /// is read as two sectors that follow one half-edge, so the
    /// half-edge alone does not say which; its direction does.
    pub dropped: SectorAt<T>,
    /// Where the kept copy holds it.
    pub kept: KeptAt<T>,
}

/// What the classification owes the dropped copies.
#[derive(Debug, Clone)]
pub(crate) struct Fates<T: geom_core::Real> {
    /// The kept edges along which a dropped face's fate changes.
    pub edges: Vec<FateEdge>,
    /// The dropped sectors held wholly.
    pub corners: Vec<FateCorner<T>>,
    /// Pairs of fate edges of one holder that may lie across a slit's
    /// line from each other ([`border_fates`]), as `(holder, dropped
    /// face, the two edges)`.
    pub across: Vec<(Operand, FaceKey, [EdgeKey; 2])>,
}

impl<T: geom_core::Real> Default for Fates<T> {
    fn default() -> Self {
        Self {
            edges: Vec::new(),
            corners: Vec::new(),
            across: Vec::new(),
        }
    }
}

/// **A face a boolean holds through a coincident face**
/// (`BooleanNaming::substituted`): where two operands' faces coincide
/// and the Eq. 15.3 lump keeps one copy of their shared region, the
/// fragment of the dropped copy that region covers, with the kept face
/// holding its interior. The dropped face is cut where the region ends
/// ([`super::fate`]), so the fragment is substituted whole, never in
/// part. Clone keys, as [`super::DiscardRow::face`]: chase
/// `face_fragments_a`/`face_fragments_b` for the operand faces. `by` is
/// a face the selection keeps, never a discard; the output stage may
/// merge it into a neighbour after, so it need not be live in the
/// result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubstitutedFace {
    /// The dropped copy's operand.
    pub operand: Operand,
    /// The fragment, in its operand's clone keys.
    pub face: FaceKey,
    /// The kept face holding it, in the other operand's clone keys.
    pub by: FaceKey,
}

/// **A dropped coincident face a containment fallback could not divide**
/// (`BooleanNaming::uncut`): a kept face holds part of it, and the cut
/// that would divide it where the held region ends met one of the cut's
/// placement limits (`fate::Limit`). No row names any part of
/// it, neither a [`SubstitutedFace`] nor a discard: the naming says
/// nothing of which part the kept face holds, and a reader meets an
/// absence. The fallback keeps or drops whole shells, so the face's
/// refusal moves no body and no other face's cut. Clone keys, as
/// [`SubstitutedFace`]'s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UncutFace {
    /// The dropped copy's operand.
    pub operand: Operand,
    /// The dropped face, whole.
    pub face: FaceKey,
    /// The kept face holding part of it, in the other operand's clone
    /// keys.
    pub by: FaceKey,
    /// The cut's refusal.
    pub kind: super::BooleanErrorKind,
    /// Its words.
    pub what: String,
}

/// A one-sided chord and the kept edge it runs along.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Chord {
    /// The dropped operand, whose clone holds `edge`.
    pub operand: Operand,
    /// The chord.
    pub edge: EdgeKey,
    /// The kept edge, in the other operand's clone keys.
    pub along: EdgeKey,
    /// The chord's two ends, each at the kept edge's end in `kept_ends`.
    pub ends: [VertexKey; 2],
    /// The kept edge's two ends, in the other operand's clone keys.
    pub kept_ends: [VertexKey; 2],
}

/// [`cut`]'s product (module docs).
#[derive(Debug, Clone, Default)]
pub(crate) struct FateCuts {
    /// Every substituted fragment, sorted and deduplicated.
    pub substituted: Vec<SubstitutedFace>,
    /// Every chord, in mint order.
    pub chords: Vec<Chord>,
}

impl FateCuts {
    /// The chord `edge` of `operand`'s clone, if it is one.
    pub(crate) fn chord(&self, operand: Operand, edge: EdgeKey) -> Option<&Chord> {
        self.chords
            .iter()
            .find(|c| c.operand == operand && c.edge == edge)
    }

    /// Whether `face` of `operand`'s clone is substituted.
    pub(crate) fn is_substituted(&self, operand: Operand, face: FaceKey) -> bool {
        self.substituted
            .iter()
            .any(|s| s.operand == operand && s.face == face)
    }
}

/// The fate edges that bound a dropped region from outside. A null
/// edge is the split's scaffolding, not an edge of the face, and an
/// edge whose two faces both cover the dropped face (a seam between two
/// coplanar faces of the kept copy, or a face meeting itself) lies
/// inside the held region; both are dropped. Sorted and deduplicated.
///
/// Two fate edges of one holder that overlap along a length
/// ([`crate::census::collinear_overlaps`]) over one dropped face, each
/// with the one kept face on its side as the classification read it,
/// are pieces of a slit's or ring's two edges: [`Fates::across`] lists
/// them for [`cut`], which reads after the join whether the kept face
/// still lies on both sides of the line.
///
/// # Errors
///
/// [`BooleanError::ClassificationInvariant`] for a fate edge that no
/// longer resolves; [`BooleanError::Escalated`] where whether two of the
/// holder's edges overlap reads in band.
pub(super) fn border_fates<T: geom_core::Real>(
    mut fates: Fates<T>,
    covered: &[(FaceKey, FaceKey)],
    null_edges: &[super::BoolNullEdgeRecord<T>],
    (a, b): (&Body<T>, &Body<T>),
    overlaps: impl Fn(&Body<T>) -> Result<Vec<[EdgeKey; 2]>, BooleanError>,
) -> Result<Fates<T>, BooleanError> {
    fates.edges.sort();
    fates.edges.dedup();
    let mut edges = Vec::with_capacity(fates.edges.len());
    for h in fates.edges {
        if null_edges
            .iter()
            .any(|r| r.operand == h.holder && r.edge == h.edge)
        {
            continue;
        }
        let body = match h.holder {
            Operand::A => a,
            Operand::B => b,
        };
        let edge = body
            .get_edge(h.edge)
            .ok_or(BooleanError::ClassificationInvariant {
                what: "a fate edge no longer resolves",
            })?;
        let sides = [edge.he_plus, edge.he_minus].map(|he| body.face_of_half_edge(he));
        let covers = |f: Option<FaceKey>| {
            f.is_some_and(|f| {
                covered.contains(&match h.holder {
                    Operand::A => (f, h.face),
                    Operand::B => (h.face, f),
                })
            })
        };
        if sides[0] == sides[1] || sides.iter().all(|&f| covers(f)) {
            continue;
        }
        edges.push(h);
    }
    let mut across = Vec::new();
    for (holder, body) in [(Operand::A, a), (Operand::B, b)] {
        let mut sides: Vec<(FaceKey, FaceKey)> = edges
            .iter()
            .filter(|h| h.holder == holder)
            .map(|h| (h.face, h.kept))
            .collect();
        let count = sides.len();
        sides.sort();
        sides.dedup();
        if sides.len() == count {
            continue;
        }
        for [e0, e1] in overlaps(body)? {
            let fate = |e: EdgeKey| {
                edges
                    .iter()
                    .filter(move |h| h.holder == holder && h.edge == e)
            };
            for h in fate(e0) {
                if fate(e1).any(|g| (g.face, g.kept) == (h.face, h.kept)) {
                    across.push((holder, h.face, [e0, e1]));
                }
            }
        }
    }
    across.sort();
    across.dedup();
    fates.edges = edges;
    fates.across = across;
    Ok(fates)
}

/// **The cut's placement limits**: the poses where the rules here
/// cannot place a chord or a ring in the dropped face as the face stands.
/// Each is a pose the cut has no rule for, not a defect: a key that does
/// not resolve is [`BooleanError::JoinDesync`], a reading in band is
/// [`BooleanError::Escalated`], and an Euler operator's refusal is its
/// own; none of those is one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Limit {
    /// A chord's end reads in no corner of the dropped face.
    NoCorner,
    /// A chord's end reads in several corners at once.
    SeveralCorners,
    /// A dropped corner pierced inside an edge of the kept face's
    /// division, at no vertex of it, whose direction no one piece beside
    /// that edge holds: the edge curved, or the direction decided along
    /// it.
    BesideNoPiece,
    /// A ring of a divided face lies on the new piece's boundary at
    /// every vertex.
    RingUnplaced,
    /// A closed kept edge's one vertex leaves and returns through two
    /// corners no rule here joins.
    ClosedAtOneVertex,
    /// A ring the chord closes, on a face with no one normal to wind it
    /// by.
    RingOnCurved,
    /// A circular edge, on a face with no one normal to wind it by.
    CircleOnCurved,
}

impl Limit {
    /// Its words, as the section path refuses with them.
    const fn what(self) -> &'static str {
        match self {
            Self::NoCorner => "a fate cut's end lies in no corner of the dropped face",
            Self::SeveralCorners => "a fate cut's end lies in several corners of the dropped face",
            Self::BesideNoPiece => {
                "a fate cut's end on an edge of its face's division is held by no one piece beside it"
            }
            Self::RingUnplaced => {
                "a ring of a face a fate cut divides is placed at none of its vertices"
            }
            Self::ClosedAtOneVertex => {
                "a fate cut along a closed kept edge has both ends at one vertex of the dropped face"
            }
            Self::RingOnCurved => "a fate cut closes a ring on a curved face",
            Self::CircleOnCurved => "a fate cut closes a circular edge on a curved face",
        }
    }
}

/// A refusal of [`cut`]: one of its placement limits, or any other
/// refusal, as it is.
#[derive(Debug)]
pub(super) enum CutError {
    /// A placement limit ([`Limit`]).
    Limit(Limit),
    /// Any other refusal.
    Refused(BooleanError),
}

impl From<Limit> for CutError {
    fn from(limit: Limit) -> Self {
        Self::Limit(limit)
    }
}

impl From<BooleanError> for CutError {
    fn from(e: BooleanError) -> Self {
        Self::Refused(e)
    }
}

impl From<crate::lineage::RowRefusal> for CutError {
    fn from(e: crate::lineage::RowRefusal) -> Self {
        Self::Refused(e.into())
    }
}

impl From<crate::euler::EulerOpError> for CutError {
    fn from(e: crate::euler::EulerOpError) -> Self {
        Self::Refused(e.into())
    }
}

/// A limit refuses the boolean that cannot place its cut as the broken
/// invariant it would be there (the section path).
impl From<CutError> for BooleanError {
    fn from(e: CutError) -> Self {
        match e {
            CutError::Limit(limit) => Self::ClassificationInvariant { what: limit.what() },
            CutError::Refused(e) => e,
        }
    }
}

/// Where a chord meets the dropped face at one of its ends.
#[derive(Debug, Clone, Copy)]
enum Place {
    /// An edge of the face already runs this way from the end.
    Along(HalfEdgeKey),
    /// In the corner before this half-edge, at its start.
    Corner(HalfEdgeKey),
    /// At a lone vertex, in its empty loop.
    Lone(LoopKey, VertexKey),
}

/// **A direction leaving a vertex, as a corner reads it**
/// ([`corner_holds`]): an edge's ([`leaving`]), its reach `R` known, or
/// as a [`Ray`] one with no edge behind it (a sector's bisector), whose
/// reach is `None`.
#[derive(Debug, Clone, Copy)]
struct Leaving<T: geom_core::Real, R = LeverBounds<T>> {
    /// The unit direction: the edge's tangent, or its secant.
    dir: Vec3<T>,
    /// How far the edge reaches from the vertex
    /// ([`geom_brep::edge_reach_bounds`]): a corner's reading acts on
    /// every sign, so its lever is the region's own measure from where
    /// it is read, and where that is known only from both sides (a curve
    /// other than a line or circle, whose bound from above can pass it
    /// many times over) the corner reads at both and acts only on a sign
    /// they share ([`sign_at`]).
    reach: R,
    /// The edge's curvature vector where `dir` is its tangent: a line's
    /// zero, a circle's or an ellipse's. `None` for a secant, for a
    /// spiric or a spline edge, whose bend carries no bound
    /// ([`geom_brep::BendBounds::Unbounded`]), and for a direction with
    /// no edge behind it. What tells a direction apart from a bound it
    /// leaves along at first order ([`corner_holds`]).
    bend: Option<Vec3<T>>,
}

/// A direction read against a corner ([`corner_holds`]): an edge's
/// [`Leaving`], or one with no edge behind it, which has no reach.
type Ray<T> = Leaving<T, Option<LeverBounds<T>>>;

impl<T: geom_core::Real> Leaving<T> {
    /// As a direction read against a corner, its reach its own.
    fn ray(self) -> Ray<T> {
        Leaving {
            dir: self.dir,
            reach: Some(self.reach),
            bend: self.bend,
        }
    }
}

impl<T: geom_core::Real> Ray<T> {
    /// A direction with no edge behind it.
    fn bisector(d: Vec3<T>) -> Self {
        Leaving {
            dir: d,
            reach: None,
            bend: None,
        }
    }
}

/// How `he` leaves its start along its tangent ([`leaving`]); `None`
/// for null scaffolding.
fn departure<T: Decide>(body: &Body<T>, he: HalfEdgeKey) -> Option<Leaving<T>> {
    leaving(body, he, false)
}

/// How `he` leaves its start: along its tangent with its bend, or with
/// `secant` along the unit chord from `he`'s start to its edge's
/// midpoint, which tells apart two curves leaving one point tangent to
/// each other, with none. `None` for null scaffolding.
fn leaving<T: Decide>(body: &Body<T>, he: HalfEdgeKey, secant: bool) -> Option<Leaving<T>> {
    let h = body.get_half_edge(he)?;
    let edge = body.get_edge(h.edge)?;
    let curve = body.get_curve_geom(edge.curve)?.certified()?;
    let base = body.half_edge_start_point(he)?;
    let end = body.half_edge_start_point(body.mate(he)?)?;
    let chord = (end - base).norm();
    let (t0, t1) = curve.params();
    let reach = match curve.carrier() {
        geom::Curve3::Line { .. } => LeverBounds::Exact(chord),
        carrier => geom_brep::edge_reach_bounds(carrier, t0, t1, base),
    };
    let (dir, bend) = match curve.carrier() {
        _ if secant => (curve.mid_point() - base, None),
        geom::Curve3::Line { .. } => (end - base, Some(Vec3::new(T::zero(), T::zero(), T::zero()))),
        geom::Curve3::Circle { .. }
        | geom::Curve3::Ellipse { .. }
        | geom::Curve3::Spiric { .. }
        | geom::Curve3::Nurbs(_) => {
            let departure = geom_brep::ConicDeparture::of(curve, he == edge.he_plus, chord);
            let bend = match departure.bend.bounds {
                geom_brep::BendBounds::Planar { .. } => Some(departure.bend.curvature),
                geom_brep::BendBounds::Unbounded => None,
            };
            (departure.dir, bend)
        }
    };
    Some(Leaving {
        dir: dir / dir.norm(),
        reach,
        bend,
    })
}

/// The sign of `x`, a dimensionless reading levered at `arm`.
fn sign<T: Decide>(x: T, arm: T, band: Band) -> Result<Sign, BooleanError> {
    crate::validate::decide("bool_fate_corner", Margin::levered(x, arm), band)
        .map_err(corner_refused)
}

/// The sign of `x`, a dimensionless reading levered at `arm`, known
/// exactly or from both sides: read once where exact, else at both
/// bounds, a verdict only where the two agree and their straddle of the
/// band (`bool_fate_corner_straddle`) refused as an in-band sign is
/// ([`crate::validate::decide_levered`]).
fn sign_at<T: Decide>(x: T, arm: LeverBounds<T>, band: Band) -> Result<Sign, BooleanError> {
    crate::validate::decide_levered(
        ("bool_fate_corner", "bool_fate_corner_straddle"),
        x,
        arm,
        band,
    )
    .map_err(corner_refused)
}

/// A corner's sign that did not decide: the sectors' coincidence, the
/// refusal an in-band corner reading has always been.
fn corner_refused(diag: geom_core::Indeterminate) -> BooleanError {
    BooleanError::coincidence(super::Coincide::Sectors, super::DeclarationRead::Moot, diag)
}

/// A unit direction strictly inside `s`: its bounds' bisector. The
/// sector reader splits every corner a half-turn wide or wider
/// (`crate::sector_shape`), so `s` is narrower than one and its bounds'
/// sum points inside it; a sum that does not read strictly positive is a
/// sector left a half-turn wide, whose bounds name no inside, and
/// refuses.
pub(super) fn bisector<T: Decide>(
    s: &super::sectors::BoolSector<T>,
    band: Band,
) -> Result<Vec3<T>, BooleanError> {
    let mid = s.start + s.end;
    let len = mid.norm();
    match sign(len, T::one(), band)? {
        Sign::Positive => Ok(mid / len),
        Sign::Zero | Sign::Negative => Err(BooleanError::ClassificationInvariant {
            what: "a sector a half-turn wide reached the fate reading, whose bounds name no inside",
        }),
    }
}

/// A containment read refused at a fate cut: typed where it read in
/// band, else the cut's own refusal `what`, which names the read.
fn contained(e: super::ContainError, what: &'static str) -> BooleanError {
    match e {
        super::ContainError::Escalated {
            decision,
            escalation,
            diag,
        } => BooleanError::Escalated {
            decision: super::BooleanDecision::Containment {
                decision,
                escalation,
            },
            diag,
        },
        _ => BooleanError::ClassificationInvariant { what },
    }
}

/// Where [`corner_holds`] reads a direction against one corner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Holds {
    /// Strictly inside it.
    In,
    /// Not strictly inside it: outside its turn, or along one of its
    /// bounds at first order and bending away from the corner one order
    /// down.
    Out,
    /// Along its first bound at first order, with no bend on one side
    /// to read it by one order down.
    Along,
    /// Along one of its bounds at both orders, the two bends reading
    /// equal; `along_out` where that bound is the first, the face's own
    /// edge leaving with the direction. Two arcs of one circle, or two
    /// lines, coincide and never part; a curve of equal curvature but
    /// another shape (an ellipse against its osculating circle) parts
    /// only at third order, which is not read.
    Tied {
        /// Whether the bound is the corner's first.
        along_out: bool,
    },
}

/// Where unit `d.dir` lies against the corner swept counterclockwise
/// about `n` from `out` to `back`. Where the two bounds leave along one
/// ray, `wraps` says whether the face wraps round it, the corner then
/// the whole turn but the ray itself, or the cusp between them.
///
/// Every sign acts on its verdict, so each is levered at the shorter of
/// the two directions' own reaches ([`reading_lever`]): `d`'s side of
/// each bound at that bound's reach and `d`'s own (`None` for a
/// bisector, which has none), and the bounds' turn against each other
/// at the lesser of theirs. Where a reach is known only from both sides
/// its signs are read at both and acted on only where they agree
/// ([`sign_at`]).
///
/// Where `d` leaves along a bound at first order, its side of that bound
/// is read one order down off the two bends
/// ([`geom_brep::parting`], with `û` the unit in-plane
/// direction square to the bound into the corner), at the lever that
/// bound's first-order sign is read at: from both sides where a reach is
/// known only so, a verdict only where the two agree and their straddle
/// (`bool_fate_corner_bend_straddle`) refused, so a margin read short at
/// one end refuses rather than flips.
///
/// **Where the rule stops.** Along a bound with no bend to read on
/// either side (a spline or spiric edge, a bisector, a secant), `d` is
/// [`Holds::Along`] at `out`, and at `back` leaves `s2` Zero: the corner
/// then holds it only as its turn holds a direction on that bound, a
/// convex corner not and a reflex one by `s1` alone. Along either bound
/// with bends that read equal, `d` is [`Holds::Tied`], which [`place`]
/// refuses rather than read by its secant. Both answer before `wraps` is
/// asked: a direction along a dangling edge runs along it, and where it
/// runs on to the cut's far end the face's own edge leaving with it is
/// the place.
fn corner_holds<T: Decide>(
    d: Ray<T>,
    (out, back): (Leaving<T>, Leaving<T>),
    wraps: impl FnOnce() -> Result<bool, BooleanError>,
    n: Vec3<T>,
    band: Band,
) -> Result<Holds, BooleanError> {
    // `d`'s side of bound `b` it leaves along, read one order down at
    // `lever`, `u` square to the bound into the corner.
    let past = |b: Leaving<T>, lever: LeverBounds<T>, u: Vec3<T>| {
        let (Some(k), Some(kb)) = (d.bend, b.bend) else {
            return Ok(None);
        };
        crate::validate::decide_at_lever(
            ("bool_fate_corner_bend", "bool_fate_corner_bend_straddle"),
            lever,
            |l| geom_brep::parting(k - kb, u, l),
            band,
        )
        .map(Some)
        .map_err(corner_refused)
    };
    let at_out = reading_lever(out.reach, d.reach);
    let mut s1 = sign_at(n.dot(out.dir.cross(d.dir)), at_out, band)?;
    if s1 == Sign::Zero && sign_at(out.dir.dot(d.dir), at_out, band)? == Sign::Positive {
        match past(out, at_out, n.cross(out.dir))? {
            None => return Ok(Holds::Along),
            Some(Sign::Zero) => return Ok(Holds::Tied { along_out: true }),
            Some(s) => s1 = s,
        }
    }
    let at_back = reading_lever(back.reach, d.reach);
    let mut s2 = sign_at(n.dot(d.dir.cross(back.dir)), at_back, band)?;
    if s2 == Sign::Zero && sign_at(back.dir.dot(d.dir), at_back, band)? == Sign::Positive {
        match past(back, at_back, back.dir.cross(n))? {
            None => {}
            Some(Sign::Zero) => return Ok(Holds::Tied { along_out: false }),
            Some(s) => s2 = s,
        }
    }
    let either = s1 == Sign::Positive || s2 == Sign::Positive;
    let both = s1 == Sign::Positive && s2 == Sign::Positive;
    let arm = reading_lever(back.reach, Some(out.reach));
    let holds = match sign_at(n.dot(out.dir.cross(back.dir)), arm, band)? {
        Sign::Positive => both,
        Sign::Negative => either,
        Sign::Zero if sign_at(out.dir.dot(back.dir), arm, band)? == Sign::Positive => {
            wraps()? || both
        }
        Sign::Zero => either,
    };
    Ok(if holds { Holds::In } else { Holds::Out })
}

/// Every place, among the faces of `body` divided from `root`, a chord
/// leaving `at` (or a null-edge copy of it) along `d` may start, with
/// whether `d` reads [`Holds::Tied`] to a bound it leaves along; `None`
/// where none of those faces holds `at`. A corner whose two bounds
/// leave along one ray wraps the face round it where they are one
/// dangling edge, or two straight edges at a tip ([`tip_wraps`]); two
/// other edges leaving tangent bound a cusp, which holds a direction
/// along them only one order down ([`corner_holds`]). With `secant`,
/// `d` is a secant and the corners' bounds are read by theirs.
fn places<T: Decide>(
    (body, operand): (&Body<T>, Operand),
    at: &[VertexKey],
    root: FaceKey,
    divisions: &FaceDivisions,
    (d, secant): (Ray<T>, bool),
    band: Band,
) -> Result<Option<(Vec<Place>, bool)>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut found = Vec::new();
    let (mut held, mut tied) = (false, false);
    for &w in at {
        let vertex = body
            .get_vertex(w)
            .ok_or_else(|| desync("a fate cut's end vertex no longer resolves"))?;
        let Some(_) = vertex.emanating else {
            for (f, face) in body.faces() {
                if divisions.root(f) != root {
                    continue;
                }
                for &l in &face.rings {
                    if let Some(lp) = body.get_loop(l)
                        && lp.boundary == (LoopBoundary::Empty { vertex: w })
                    {
                        held = true;
                        found.push(Place::Lone(l, w));
                    }
                }
            }
            continue;
        };
        for h in body.vertex_orbit_linked(w) {
            let f = body
                .face_of_half_edge(h)
                .ok_or_else(|| desync("a fate cut's corner has no face"))?;
            if divisions.root(f) != root || body.null_face_pair(f).is_some() {
                continue;
            }
            held = true;
            let Some(out) = leaving(body, h, secant) else {
                continue;
            };
            // The corner's other bound is the first edge before `h` that
            // is not null scaffolding.
            let mut p = h;
            let back = loop {
                p = body
                    .get_half_edge(p)
                    .ok_or_else(|| desync("a fate cut's corner does not walk"))?
                    .prev;
                let mate = body
                    .mate(p)
                    .ok_or_else(|| desync("a fate cut's corner edge has no mate"))?;
                if let Some(back) = leaving(body, mate, secant) {
                    break back;
                }
                if p == h {
                    return Err(BooleanError::ClassificationInvariant {
                        what: "a fate cut's corner is bounded by null edges only",
                    });
                }
            };
            let at_point = body
                .half_edge_start_point(h)
                .ok_or_else(|| desync("a fate cut's corner has no point"))?;
            let normal = normal_at(
                (body, f, at_point),
                "a fate cut's corner face has no normal at its vertex",
                band,
            )?;
            let wraps = || {
                let bounds = [p, h].map(|x| body.get_half_edge(x).map(|x| x.edge));
                if bounds[0] == bounds[1] {
                    return Ok(true);
                }
                if !bounds
                    .iter()
                    .all(|e| e.is_some_and(|e| crate::census::edge_is_line(body, e)))
                {
                    return Ok(false);
                }
                tip_wraps((body, operand), (f, h), band)
            };
            match corner_holds(d, (out, back), wraps, normal, band)? {
                Holds::Along => found.push(Place::Along(h)),
                Holds::Tied { along_out } => {
                    tied = true;
                    if along_out {
                        found.push(Place::Along(h));
                    }
                }
                Holds::In => found.push(Place::Corner(h)),
                Holds::Out => {}
            }
        }
    }
    Ok(held.then_some((found, tied)))
}

/// Face `f`'s outward normal at `p`, where a corner is read: an
/// escalation where it reads in band, else `what`.
fn normal_at<T: Decide>(
    (body, f, p): (&Body<T>, FaceKey, Point3<T>),
    what: &'static str,
    band: Band,
) -> Result<Vec3<T>, BooleanError> {
    match crate::face_normal::face_outward_normal_at(body, f, p, band) {
        Ok(Some(n)) => Ok(n.vec()),
        Err(crate::face_normal::NormalAtError::Escalated { decision, diag }) => {
            Err(BooleanError::Escalated {
                decision: super::BooleanDecision::of_normal(decision),
                diag,
            })
        }
        Ok(None) | Err(_) => Err(BooleanError::ClassificationInvariant { what }),
    }
}

/// **Whether a corner whose two straight bounds leave along one ray is
/// a tip the face wraps round**, a slit's or zero-area ring's, rather
/// than a spur's, which holds nothing: `operand`'s face `f`, the corner
/// at `h`'s start. The face lies left of each of its loops, so on a
/// plane face the turns of a loop's corners sum to a whole turn,
/// counterclockwise about the outward normal for the outline and
/// clockwise for a ring. A corner whose bounds leave along one ray turns
/// a half-turn: clockwise round a tip the face wraps, counterclockwise
/// round a spur. The loop's other corners fix the sum of those
/// half-turns, which says which they are where all turn one way: a
/// sliver's outline (`slit::sliver_halves`) turns at two spurs, a
/// zero-area ring at two tips, an outline with a slit at the tip.
/// This reads the body as the join left it, not the operand at rest
/// ([`crate::sector_shape::SlitTips`]): the join cuts a slit's two edges into the
/// other operand's flush face too.
///
/// # Errors
///
/// [`BooleanError::SlitFaceGlued`] where the turns are not read: a
/// curved face, a curved edge on the loop, or half-turns both ways; an
/// escalation where a turn reads in band.
fn tip_wraps<T: Decide>(
    (body, operand): (&Body<T>, Operand),
    (f, h): (FaceKey, HalfEdgeKey),
    band: Band,
) -> Result<bool, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let unread = BooleanError::SlitFaceGlued {
        operand,
        face: f,
        gap: super::SlitFaceGap::TipUnread,
    };
    let Some(normal) = crate::face_normal::face_outward_normal(body, f) else {
        return Err(unread);
    };
    let n = normal.vec();
    let outer = body.get_face(f).map(|x| x.outer) == Some(loop_of(body, h)?);
    let cycle = body
        .loop_cycle(h)
        .ok_or_else(|| desync("a tip's loop does not walk"))?;
    // The loop's straight runs, its null scaffolding skipped.
    let mut runs = Vec::with_capacity(cycle.len());
    for he in cycle {
        let Some(run) = departure(body, he) else {
            continue;
        };
        let edge = body
            .get_half_edge(he)
            .ok_or_else(|| desync("a tip's loop does not walk"))?
            .edge;
        if !crate::census::edge_is_line(body, edge) {
            return Err(unread);
        }
        runs.push(run);
    }
    let (mut turned, mut halves, mut any) = (T::zero(), T::zero(), false);
    for (k, b) in runs.iter().enumerate() {
        let a = runs[(k + runs.len() - 1) % runs.len()];
        let (across, ahead) = (n.dot(a.dir.cross(b.dir)), a.dir.dot(b.dir));
        let arm = reading_lever(b.reach, Some(a.reach));
        if sign_at(across, arm, band)? == Sign::Zero && sign_at(ahead, arm, band)? == Sign::Negative
        {
            halves = halves + T::one();
            any = true;
        } else {
            turned = turned + across.atan2(ahead);
        }
    }
    let whole = if outer {
        T::tau()
    } else {
        T::zero() - T::tau()
    };
    let half_turns = (whole - turned) / T::pi();
    match (
        sign(half_turns + halves, T::one(), band)?,
        sign(half_turns - halves, T::one(), band)?,
    ) {
        _ if !any => Err(unread),
        (Sign::Zero, _) => Ok(true),
        (_, Sign::Zero) => Ok(false),
        _ => Err(unread),
    }
}

/// The place where a chord leaving `at` starts (`places`): an edge
/// the face already holds from `at` to one of `beyond`, the chord's
/// other end, along its tangent, or the one corner its tangent lies in
/// (one order down where it leaves along a bound), else the one its
/// secant does, the corners' bounds then read by their own secants too.
/// `readings` are the direction's tangent, then its secant. `None` where
/// no face divided from `root` holds `at`: the kept edge leaves the
/// dropped face where the reduction put no vertex in it.
///
/// A tangent [`Holds::Tied`] to a bound that it does not run along to
/// `beyond` places in no corner, and the secant is not read: at a tie
/// the two secants part by how far each edge runs (two arcs of one
/// circle, by the difference of their arc lengths), not by which side
/// one lies on, so a secant there names a corner the edge need not lie
/// in.
fn place<T: Decide>(
    (body, operand): (&Body<T>, Operand),
    (at, beyond): (&[VertexKey], &[VertexKey]),
    root: FaceKey,
    divisions: &FaceDivisions,
    readings: [Ray<T>; 2],
    band: Band,
) -> Result<Option<Place>, CutError> {
    let mut corners = Vec::new();
    for (k, d) in readings.into_iter().enumerate() {
        let read = (d, k == 1);
        let Some((found, tied)) = places((body, operand), at, root, divisions, read, band)? else {
            return Ok(None);
        };
        if k == 0
            && let Some(&along) = found.iter().find(|p| match p {
                Place::Along(h) => body.half_edge_end(*h).is_some_and(|v| beyond.contains(&v)),
                Place::Corner(_) | Place::Lone(..) => false,
            })
        {
            return Ok(Some(along));
        }
        corners = found
            .into_iter()
            .filter(|p| !matches!(p, Place::Along(_)))
            .collect();
        if let [one] = corners.as_slice() {
            return Ok(Some(*one));
        }
        if tied {
            break;
        }
    }
    Err(if corners.is_empty() {
        Limit::NoCorner
    } else {
        Limit::SeveralCorners
    }
    .into())
}

/// Whether a face divided from `root` holds one of `at`, a lone vertex
/// in it among them.
fn holds<T: geom_core::Real>(
    body: &Body<T>,
    at: &[VertexKey],
    root: FaceKey,
    divisions: &FaceDivisions,
) -> bool {
    let ours = |f: FaceKey| divisions.root(f) == root && body.null_face_pair(f).is_none();
    at.iter()
        .any(|&w| match body.get_vertex(w).and_then(|v| v.emanating) {
            Some(_) => body
                .vertex_orbit_linked(w)
                .into_iter()
                .any(|h| body.face_of_half_edge(h).is_some_and(ours)),
            None => body.faces().any(|(f, face)| {
                ours(f)
                    && face.rings.iter().any(|&l| {
                        body.get_loop(l)
                            .is_some_and(|lp| lp.boundary == (LoopBoundary::Empty { vertex: w }))
                    })
            }),
        })
}

/// Where the dropped operand meets kept vertex `x`.
enum End {
    /// At these vertices: a vertex–vertex partner or a pierce's ring
    /// vertex, each with its null-edge copies.
    At(Vec<VertexKey>),
    /// At this vertex of the kept operand on the dropped face, where no
    /// record names a partner of it in the face. The face may still hold
    /// a vertex at its point, which [`lies`] reads.
    Touch(VertexKey),
}

/// Where the dropped operand meets kept vertex `x`, which lies in the
/// face divided from `root`.
fn end_at<T: geom_core::Real>(
    red: &BooleanReduction<T>,
    holder: Operand,
    x: VertexKey,
    root: FaceKey,
    divisions: &FaceDivisions,
) -> Result<End, BooleanError> {
    let dropped = holder.other();
    let xs = NullCopies::of_operand(&red.null_edges, holder).of(x);
    let theirs = NullCopies::of_operand(&red.null_edges, dropped);
    // A pinch is several vertices on one point, one per cone, each a
    // partner: every one is offered, and the face's own is placed.
    let mut at = Vec::new();
    for c in &red.contacts.vv {
        let (k, d) = match holder {
            Operand::A => (c.a, c.b),
            Operand::B => (c.b, c.a),
        };
        if xs.contains(&k) {
            at.extend(theirs.of(d));
        }
    }
    for ring in &red.pierce_rings {
        if ring.operand == dropped
            && divisions.root(ring.face) == root
            && ring.pierces.iter().any(|v| xs.contains(v))
        {
            at.extend(theirs.of(ring.ring_vertex));
        }
    }
    if !at.is_empty() {
        at.sort();
        at.dedup();
        // Partners on other faces only (the dropped operand's own vertex
        // resting on the face, unimprinted): `x` touches the face.
        let body = match dropped {
            Operand::A => &red.a,
            Operand::B => &red.b,
        };
        return Ok(if holds(body, &at, root, divisions) {
            End::At(at)
        } else {
            End::Touch(x)
        });
    }
    let on = match holder {
        Operand::A => &red.contacts.a_on_b,
        Operand::B => &red.contacts.b_on_a,
    };
    on.iter()
        .find(|c| xs.contains(&c.vertex) && divisions.root(c.face) == root)
        .map(|c| End::Touch(c.vertex))
        .ok_or(BooleanError::ClassificationInvariant {
            what: "a fate cut's end is no contact of the dropped face",
        })
}

/// The one face divided from `root`, null faces aside, where it is one
/// piece: that piece holds whatever the face held.
fn one_piece<T: geom_core::Real>(
    body: &Body<T>,
    root: FaceKey,
    divisions: &FaceDivisions,
) -> Option<FaceKey> {
    match pieces(body, root, divisions).as_slice() {
        [one] => Some(*one),
        _ => None,
    }
}

/// The faces divided from `root`, null faces aside.
fn pieces<T: geom_core::Real>(
    body: &Body<T>,
    root: FaceKey,
    divisions: &FaceDivisions,
) -> Vec<FaceKey> {
    body.faces()
        .map(|(f, _)| f)
        .filter(|&f| divisions.root(f) == root && body.null_face_pair(f).is_none())
        .collect()
}

/// Where a point lies among the faces divided from one root ([`lies`]).
enum Lies {
    /// Inside this one piece.
    Inside(FaceKey),
    /// Inside none, at these vertices of the pieces' loops, each once: a
    /// vertex of the face's division.
    AtCorners(Vec<VertexKey>),
    /// Inside none and at none of their vertices, on an edge of the
    /// face's division: each piece with the edge of its loops it lies on.
    OnEdge(Vec<(FaceKey, EdgeKey)>),
    /// Placed by no piece ([`Unplaced`]).
    Nowhere(Unplaced),
}

/// Why no one piece of a face, and no vertex or edge of their loops,
/// places a point ([`lies`]).
#[derive(Clone, Copy)]
enum Unplaced {
    /// Inside several pieces.
    InSeveral,
    /// On no piece.
    Off,
}

impl Unplaced {
    /// A touching end on an edge of its face's division
    /// ([`Lies::OnEdge`]), which the touch placement does not place.
    const ON_EDGE: &'static str = "a fate cut's end lies on an edge of its face's division";

    /// A fate cut's end placed so.
    const fn refused(self) -> BooleanError {
        BooleanError::ClassificationInvariant {
            what: match self {
                Self::InSeveral => "a fate cut's end lies inside several pieces of its face",
                Self::Off => "a fate cut's end lies on no piece of its face",
            },
        }
    }
}

/// Where `p` lies among the faces divided from `root`: inside one, at
/// vertices of their loops, or on edges of them.
fn lies<T: Decide>(
    body: &Body<T>,
    root: FaceKey,
    divisions: &FaceDivisions,
    p: Point3<T>,
    band: Band,
) -> Result<Lies, BooleanError> {
    if let Some(one) = one_piece(body, root, divisions) {
        return Ok(Lies::Inside(one));
    }
    let unplaced = |e| contained(e, "a fate cut's end is not placed in its face");
    let (mut inside, mut corners, mut on_edge) = (Vec::new(), Vec::new(), Vec::new());
    for f in pieces(body, root, divisions) {
        let at = match crate::face_normal::face_outward_normal(body, f) {
            Some(normal) => super::contfp(body, f, normal.vec(), p, band).map_err(unplaced)?,
            None => super::curved_face_containment(body, f, p, band)
                .map_err(unplaced)?
                .ok_or(BooleanError::ClassificationInvariant {
                    what: "a fate cut's end lies in a curved face its chart cannot place it in",
                })?,
        };
        match at {
            super::FaceContainment::In => inside.push(f),
            super::FaceContainment::OnVertex(v) => corners.push(v),
            super::FaceContainment::OnEdge(e) => on_edge.push((f, e)),
            super::FaceContainment::Out => {}
        }
    }
    corners.sort();
    corners.dedup();
    Ok(match (inside.as_slice(), corners.is_empty()) {
        ([one], _) => Lies::Inside(*one),
        ([], false) => Lies::AtCorners(corners),
        ([], true) if !on_edge.is_empty() => Lies::OnEdge(on_edge),
        ([], true) => Lies::Nowhere(Unplaced::Off),
        _ => Lies::Nowhere(Unplaced::InSeveral),
    })
}

/// What the vertices a dropped sector is held at are ([`piece_holding`]).
#[derive(Clone, Copy)]
enum HeldAt {
    /// A pierce's ring vertex, with its null-edge copies.
    RingVertex,
    /// A vertex of the kept face's division.
    DivisionVertex,
}

/// **The piece of the kept face divided from `root` that holds a dropped
/// sector at one of `at`**, its interior along unit `d`: the piece whose
/// corner there holds `d`, or whose empty loop is the lone vertex
/// ([`place`]).
fn piece_holding<T: Decide>(
    (body, operand): (&Body<T>, Operand),
    (at, held): (&[VertexKey], HeldAt),
    root: FaceKey,
    divisions: &FaceDivisions,
    d: Vec3<T>,
    band: Band,
) -> Result<FaceKey, CutError> {
    let refused = |ring, division| {
        CutError::from(BooleanError::ClassificationInvariant {
            what: match held {
                HeldAt::RingVertex => ring,
                HeldAt::DivisionVertex => division,
            },
        })
    };
    match place(
        (body, operand),
        (at, &[]),
        root,
        divisions,
        [Ray::bisector(d); 2],
        band,
    )? {
        Some(Place::Corner(h)) => Ok(face_of_he(body, h)?),
        Some(Place::Lone(l, _)) => body.get_loop(l).map(|l| l.face).ok_or_else(|| {
            BooleanError::JoinDesync {
                what: "a ring vertex's loop no longer resolves",
            }
            .into()
        }),
        None => Err(refused(
            "a pierce's ring vertex lies off its kept face",
            "a vertex of a kept face's division lies off its pieces",
        )),
        Some(Place::Along(_)) => Err(refused(
            "a dropped sector held at a pierce runs along a kept edge",
            "a dropped sector held at a vertex of its kept face's division runs along a kept edge",
        )),
    }
}

/// **The piece of the kept face beside an edge of its division that
/// holds a dropped sector**, its interior along unit `d`, its vertex at
/// `p` inside that edge: the one of `sides` (a piece, and the edge of
/// its loops `p` lies on) whose corner at `p` holds `d`, its edge's
/// half-edge in it read as a straight corner there ([`corner_holds`]),
/// each bound reaching to the end of the edge it runs to.
///
/// # Errors
///
/// [`Limit::BesideNoPiece`] where no one piece holds `d`: an edge other
/// than a line, whose direction at `p` is not read here, or `d` decided
/// along the edge. A side of the edge that reads in band escalates as
/// the corner reading does ([`corner_holds`]: the sectors'
/// coincidence), and a piece with no normal at `p` refuses as
/// [`normal_at`] does.
fn piece_beside<T: Decide>(
    body: &Body<T>,
    sides: &[(FaceKey, EdgeKey)],
    p: Point3<T>,
    d: Vec3<T>,
    band: Band,
) -> Result<FaceKey, CutError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let mut holding = Vec::new();
    for &(f, e) in sides {
        if !crate::census::edge_is_line(body, e) {
            return Err(Limit::BesideNoPiece.into());
        }
        let plus = body
            .get_edge(e)
            .ok_or_else(|| desync("a division edge no longer resolves"))?
            .he_plus;
        let normal = normal_at(
            (body, f, p),
            "a piece of a fate cut's kept face has no normal on its division's edge",
            band,
        )?;
        for h in [Some(plus), body.mate(plus)].into_iter().flatten() {
            if body.face_of_half_edge(h) != Some(f) {
                continue;
            }
            let ends =
                [Some(h), body.mate(h)].map(|x| x.and_then(|x| body.half_edge_start_point(x)));
            let (Some(run), [Some(start), Some(end)]) = (departure(body, h), ends) else {
                return Err(desync("a division edge's half-edge does not walk").into());
            };
            let bound = |dir, reach| Leaving {
                dir,
                reach: LeverBounds::Exact(reach),
                bend: run.bend,
            };
            // A straight corner's bounds leave apart: it never wraps.
            let straight = (
                bound(run.dir, (end - p).norm()),
                bound(-run.dir, (p - start).norm()),
            );
            if corner_holds(Ray::bisector(d), straight, || Ok(false), normal, band)? == Holds::In {
                holding.push(f);
            }
        }
    }
    holding.sort();
    holding.dedup();
    match holding.as_slice() {
        [one] => Ok(*one),
        _ => Err(Limit::BesideNoPiece.into()),
    }
}

/// The signed area `cycle`'s points enclose about `n`, over its
/// length: positive where it winds counterclockwise.
fn winding<T: Decide>(points: &[Point3<T>], n: Vec3<T>) -> (T, T) {
    let mut area = T::zero();
    let mut len = T::zero();
    for (k, &p) in points.iter().enumerate() {
        let q = points[(k + 1) % points.len()];
        area = area + n.dot((p - points[0]).cross(q - points[0]));
        len = len + (q - p).norm();
    }
    (area, len)
}

/// The loop half-edge `he` belongs to.
fn loop_of<T: geom_core::Real>(body: &Body<T>, he: HalfEdgeKey) -> Result<LoopKey, BooleanError> {
    body.get_half_edge(he)
        .map(|h| h.parent_loop)
        .ok_or(BooleanError::JoinDesync {
            what: "a fate cut's half-edge no longer resolves",
        })
}

/// Moves each ring of `old` that the division's new face `new` holds
/// into it (Mäntylä's `laringmv`): a lone vertex minted for a later cut,
/// or a cut's dangling run; the division's own `remainder`, where it
/// split a ring, stays. A ring is read at its first vertex `new`
/// decides strictly inside or outside; one no vertex of which is
/// decided refuses, as the reading's escalation where one read in band.
fn rehome<T: Decide>(
    body: &mut Body<T>,
    (old, remainder): (FaceKey, LoopKey),
    new: FaceKey,
    band: Band,
) -> Result<(), CutError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let rings = body
        .get_face(old)
        .ok_or_else(|| desync("a divided face no longer resolves"))?
        .rings
        .clone();
    let normal = crate::face_normal::face_outward_normal(body, new);
    // Every ring is read before any moves: the reading cannot walk a
    // lone vertex's loop in `new`.
    let mut moved = Vec::new();
    for ring in rings.into_iter().filter(|&r| r != remainder) {
        let vertices: Vec<VertexKey> = match body
            .get_loop(ring)
            .ok_or_else(|| desync("a ring no longer resolves"))?
            .boundary
        {
            LoopBoundary::Empty { vertex } => vec![vertex],
            LoopBoundary::Cycle { first } => body
                .loop_cycle(first)
                .ok_or_else(|| desync("a ring does not walk"))?
                .into_iter()
                .filter_map(|he| body.get_half_edge(he).map(|h| h.start))
                .collect(),
        };
        let (mut side, mut in_band) = (None, None);
        for v in vertices {
            let p = body
                .get_vertex(v)
                .and_then(|v| body.get_point(v.point))
                .copied()
                .ok_or_else(|| desync("a ring's vertex has no point"))?;
            // A vertex the reading cannot place (on the boundary, or in
            // band) says nothing; the next is asked.
            let at = match normal {
                Some(n) => super::contfp(body, new, n.vec(), p, band).map(Some),
                None => super::curved_face_containment(body, new, p, band),
            };
            match at {
                Ok(Some(super::FaceContainment::In)) => side = Some(true),
                Ok(Some(super::FaceContainment::Out)) => side = Some(false),
                Err(e @ super::ContainError::Escalated { .. }) => {
                    in_band.get_or_insert(contained(e, ""));
                    continue;
                }
                _ => continue,
            }
            break;
        }
        match (side, in_band) {
            (Some(true), _) => moved.push(ring),
            (Some(false), _) => {}
            (None, Some(escalated)) => return Err(escalated.into()),
            (None, None) => {
                return Err(Limit::RingUnplaced.into());
            }
        }
    }
    for ring in moved {
        body.ring_move(ring, new)?;
    }
    Ok(())
}

/// The kept edge's `curve` as a chord's spec, run in its own sense
/// (`along`) or back: a line or a circle as the scaffold the join mints
/// its chords on, any other carrier, or a closed one (a whole-turn
/// scaffold circle reads as a null edge's), as an image in `chart`, the
/// dropped face's surface, which the kept one coincides with.
fn along_spec<T: Decide>(
    curve: &geom_brep::EdgeCurve<T>,
    chart: crate::geometry::SurfaceKey,
    (along, closed): (bool, bool),
) -> Result<geom_brep::EdgeCurveSpec<T>, BooleanError> {
    let refuse = || BooleanError::ClassificationInvariant {
        what: "a fate cut's kept edge does not restate as a chord on the dropped face's surface",
    };
    let own = curve.restated_in_chart(chart);
    let (carrier, t0, t1) = if along {
        (own.carrier, own.param_start, own.param_end)
    } else {
        (
            own.carrier.reversed().ok_or_else(refuse)?,
            T::zero() - own.param_end,
            T::zero() - own.param_start,
        )
    };
    match carrier {
        geom::Curve3::Line { .. } => geom_brep::EdgeCurveSpec::segment_of_line(carrier, t0, t1),
        geom::Curve3::Circle { .. } if !closed => {
            geom_brep::EdgeCurveSpec::arc_of_circle(carrier, t0, t1)
        }
        _ => Some(geom_brep::EdgeCurveSpec {
            description: own.description,
            carrier,
            param_start: t0,
            param_end: t1,
        }),
    }
    .ok_or_else(refuse)
}

/// Joins `from` to `to` in their face along the kept edge's `curve`
/// (`forward` where `from` lies at its plus half's start), returning the half
/// running from `from`'s vertex to `to`'s; a division it makes is
/// recorded in `divisions`.
fn chord<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    (from, to): (Place, Place),
    (curve, forward): (&geom_brep::EdgeCurve<T>, bool),
    divisions: &mut FaceDivisions,
    band: Band,
    tol: Tol,
) -> Result<HalfEdgeKey, CutError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let start_of = |body: &Body<T>, p: Place| -> Result<VertexKey, BooleanError> {
        match p {
            Place::Corner(h) | Place::Along(h) => body
                .get_half_edge(h)
                .map(|h| h.start)
                .ok_or_else(|| desync("a fate cut's corner no longer resolves")),
            Place::Lone(_, w) => Ok(w),
        }
    };
    let face_of = |body: &Body<T>, p: Place| -> Result<FaceKey, BooleanError> {
        let l = match p {
            Place::Corner(h) | Place::Along(h) => loop_of(body, h)?,
            Place::Lone(l, _) => l,
        };
        body.get_loop(l)
            .map(|l| l.face)
            .ok_or_else(|| desync("a fate cut's loop no longer resolves"))
    };
    let (u, w) = (start_of(body, from)?, start_of(body, to)?);
    let face = face_of(body, from)?;
    if face != face_of(body, to)? {
        return Err(BooleanError::ClassificationInvariant {
            what: "a fate cut's two ends lie in different pieces of the dropped face",
        }
        .into());
    }
    let point = |body: &Body<T>, v: VertexKey| {
        body.get_vertex(v)
            .and_then(|v| body.get_point(v.point))
            .copied()
            .ok_or_else(|| desync("a fate cut's end has no point"))
    };
    let pw = point(body, w)?;
    // The kept edge's curve, run from `v`, one of the chord's ends.
    let chart = body
        .get_face(face)
        .map(|f| f.surface)
        .ok_or_else(|| desync("a fate cut's face no longer resolves"))?;
    let run = |v: VertexKey| along_spec(curve, chart, (forward == (v == u), false));
    let outer = body
        .get_face(face)
        .map(|f| f.outer)
        .ok_or_else(|| desync("a fate cut's face no longer resolves"))?;
    if u == w {
        // A closed kept edge, both of whose ends are one vertex of the
        // face: a circular edge there (Fig. 9.8b), in one corner of it or
        // lone, or, leaving and returning through two corners of the
        // outer loop, a chord splitting that loop.
        let (site, along) = match (from, to) {
            (Place::Lone(l, _), Place::Lone(m, _)) if l == m => (
                MefSite::Lone { r#loop: l },
                circular(body, face, curve, band)?,
            ),
            (Place::Corner(h0), Place::Corner(h1)) if h0 == h1 => (
                MefSite::Chords { he1: h0, he2: h0 },
                circular(body, face, curve, band)?,
            ),
            (Place::Corner(h0), Place::Corner(h1))
                if loop_of(body, h0)? == outer && loop_of(body, h1)? == outer =>
            {
                (MefSite::Chords { he1: h0, he2: h1 }, forward)
            }
            _ => {
                return Err(Limit::ClosedAtOneVertex.into());
            }
        };
        let made = body.mef(
            site,
            along_spec(curve, chart, (along, true))?,
            FaceSurface::Inherit,
            tol,
        )?;
        divisions.push((made.face, face))?;
        let remainder = loop_of(body, made.he_plus)?;
        rehome(body, (face, remainder), made.face, band)?;
        // The half run in the kept edge's own sense along `forward`.
        return Ok(if along == forward {
            made.he_plus
        } else {
            made.he_minus
        });
    }
    let made = match (from, to) {
        (Place::Corner(h0), Place::Corner(h1)) if loop_of(body, h0)? == loop_of(body, h1)? => {
            // A ring the chord closes splits into the new face's outer
            // loop, which must wind counterclockwise, and the rest.
            let (he1, he2) = if loop_of(body, h0)? == outer {
                (h0, h1)
            } else {
                let normal = crate::face_normal::face_outward_normal(body, face)
                    .ok_or(Limit::RingOnCurved)?;
                let mut points = Vec::new();
                let mut at = h0;
                while at != h1 {
                    points.push(point(body, start_of(body, Place::Corner(at))?)?);
                    at = body
                        .get_half_edge(at)
                        .ok_or_else(|| desync("a fate cut's ring does not walk"))?
                        .next;
                }
                // Back from `w` to `u` along the kept edge, which runs
                // from `u` in its own sense where `forward`.
                let mut back: Vec<Point3<T>> = (1..8)
                    .map(|i| curve.carrier().eval(curve.sample_param(i)))
                    .collect();
                if forward {
                    back.reverse();
                }
                points.push(pw);
                points.extend(back);
                match ring_winding(&points, normal.vec(), band)? {
                    Sign::Positive => (h0, h1),
                    Sign::Negative => (h1, h0),
                    Sign::Zero => {
                        return Err(
                            BooleanError::ClassificationInvariant { what: RING_NO_AREA }.into()
                        );
                    }
                }
            };
            let made = body.mef(
                MefSite::Chords { he1, he2 },
                run(start_of(body, Place::Corner(he1))?)?,
                FaceSurface::Inherit,
                tol,
            )?;
            divisions.push((made.face, face))?;
            let remainder = loop_of(body, made.he_plus)?;
            rehome(body, (face, remainder), made.face, band)?;
            [made.he_plus, made.he_minus]
        }
        (Place::Corner(h0), Place::Corner(h1)) => {
            let (target, ring) = if loop_of(body, h1)? == outer {
                (h1, h0)
            } else {
                (h0, h1)
            };
            let made = body.mekr(
                MekrSite::Cycles { target, ring },
                run(start_of(body, Place::Corner(target))?)?,
                tol,
            )?;
            [made.he_plus, made.he_minus]
        }
        (Place::Corner(target), Place::Lone(ring, _))
        | (Place::Lone(ring, _), Place::Corner(target)) => {
            let made = body.mekr(
                MekrSite::EmptyRing { target, ring },
                run(start_of(body, Place::Corner(target))?)?,
                tol,
            )?;
            [made.he_plus, made.he_minus]
        }
        (Place::Lone(target, _), Place::Lone(ring, _)) => {
            let made = body.mekr(MekrSite::BothEmpty { target, ring }, run(u)?, tol)?;
            [made.he_plus, made.he_minus]
        }
        (Place::Along(_), _) | (_, Place::Along(_)) => {
            return Err(BooleanError::ClassificationInvariant {
                what: "a fate cut runs along an edge at one end only",
            }
            .into());
        }
    };
    Ok(made
        .into_iter()
        .find(|&he| body.get_half_edge(he).is_some_and(|h| h.start == u))
        .ok_or_else(|| desync("a fate cut's chord does not start at its end"))?)
}

/// The sense, the kept edge's own (`true`) or back, in which a circular
/// edge along closed `curve` runs in plane `face`: its minus half bounds
/// the new face's outer loop, which winds counterclockwise, so the plus
/// half runs the way the curve winds clockwise.
fn circular<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    curve: &geom_brep::EdgeCurve<T>,
    band: Band,
) -> Result<bool, CutError> {
    let normal =
        crate::face_normal::face_outward_normal(body, face).ok_or(Limit::CircleOnCurved)?;
    let points: Vec<Point3<T>> = (0..8)
        .map(|i| curve.carrier().eval(curve.sample_param(i)))
        .collect();
    match ring_winding(&points, normal.vec(), band)? {
        Sign::Negative => Ok(true),
        Sign::Positive => Ok(false),
        Sign::Zero => Err(BooleanError::ClassificationInvariant { what: RING_NO_AREA }.into()),
    }
}

/// A ring the cut closes whose signed area decides zero.
const RING_NO_AREA: &str = "a fate cut closes a ring of no area";

/// Which way the ring through `points` winds about `n`: its signed area
/// over its length, positive counterclockwise.
///
/// # Errors
///
/// [`BooleanError::Escalated`] ([`super::BooleanDecision::CutRingWinding`])
/// where it reads in band.
fn ring_winding<T: Decide>(
    points: &[Point3<T>],
    n: Vec3<T>,
    band: Band,
) -> Result<Sign, BooleanError> {
    let (area, len) = winding(points, n);
    crate::validate::decide("bool_fate_ring", Margin::of(area / len), band).map_err(|diag| {
        BooleanError::Escalated {
            decision: super::BooleanDecision::CutRingWinding,
            diag,
        }
    })
}

/// The piece of the sector's face, as divided since, that holds the
/// sector's direction at its vertex ([`place`]); the face itself where
/// it is one piece.
///
/// # Errors
///
/// [`place`]'s, the reading in band among them; a vertex no piece of
/// the face holds, or whose direction runs along an edge of it, refuses
/// [`BooleanError::ClassificationInvariant`].
fn sector_piece<T: Decide>(
    (body, operand): (&Body<T>, Operand),
    s: SectorAt<T>,
    (divisions, copies): (&FaceDivisions, &NullCopies),
    band: Band,
) -> Result<FaceKey, CutError> {
    let (he, face, d) = (s.he, s.face, s.dir);
    if let Some(one) = one_piece(body, divisions.root(face), divisions) {
        return Ok(one);
    }
    let v = body
        .get_half_edge(he)
        .ok_or(BooleanError::JoinDesync {
            what: "a sector's half-edge no longer resolves",
        })?
        .start;
    let placed = place(
        (body, operand),
        (&copies.of(v), &[]),
        divisions.root(face),
        divisions,
        [Ray::bisector(d); 2],
        band,
    );
    let piece: Result<FaceKey, BooleanError> = match placed? {
        Some(Place::Corner(h)) => face_of_he(body, h),
        Some(Place::Lone(l, _)) => {
            body.get_loop(l)
                .map(|l| l.face)
                .ok_or(BooleanError::JoinDesync {
                    what: "a lone vertex's loop no longer resolves",
                })
        }
        None => Err(BooleanError::ClassificationInvariant {
            what: "a sector's vertex lies on no piece of its divided face",
        }),
        Some(Place::Along(_)) => Err(BooleanError::ClassificationInvariant {
            what: "a sector's bisector runs along an edge of its divided face",
        }),
    };
    Ok(piece?)
}

/// The face holding `he`, in `body`.
fn face_of_he<T: geom_core::Real>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> Result<FaceKey, BooleanError> {
    body.face_of_half_edge(he).ok_or(BooleanError::JoinDesync {
        what: "a fate cut's half-edge has no face",
    })
}

/// **Divides each dropped coincident face where its fate changes**
/// (module docs), after the join: `divisions` are each operand's face
/// division rows, A's then B's, which each chord that divides a face
/// extends.
///
/// # Errors
///
/// The Euler operators' refusals; a cut that cannot be placed (an end in
/// no one piece or corner of the dropped face) or would leave a slit;
/// [`BooleanError::Escalated`] where a placement reads in band (a ring
/// of a divided face no vertex of which is clear of the chord); and any
/// refusal past the cuts (a corner's kept face, a key that no longer
/// resolves).
pub(super) fn cut<T: Decide + crate::props::AtRestPolicy>(
    red: &mut BooleanReduction<T>,
    divisions: [&mut FaceDivisions; 2],
    band: Band,
    tol: Tol,
) -> Result<FateCuts, CutError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let [div_a, div_b] = divisions;
    let mut touched = std::collections::BTreeMap::new();
    // Each half whose face is substituted, with the kept half whose face
    // holds it; each chord, with its other half.
    let mut halves: Vec<(Operand, HalfEdgeKey, HalfEdgeKey)> = Vec::new();
    let mut out = FateCuts::default();
    let mut slits = Vec::new();
    let mut done = std::collections::BTreeSet::new();
    // Every touch is placed before any lone vertex is minted: the
    // containment reading cannot walk an empty loop.
    // A kept vertex's point key with its touch, per dropped face: two
    // kept vertices on one point (a slit's open end, one per wedge) touch
    // the dropped face at one place.
    let mut touches: Vec<(Operand, FaceKey, VertexKey, Point3<T>)> = Vec::new();
    let mut touch_points: Vec<(Operand, FaceKey, VertexKey, crate::geometry::PointKey)> =
        Vec::new();
    // Where one kept face holds the dropped one on both sides of a
    // slit's line, a chord along it divides nothing, so none is minted;
    // its touching ends are not placed, so it is passed over before they
    // are read.
    let across = |red: &BooleanReduction<T>, fe: &FateEdge| {
        let (div_k, kept_body) = match fe.holder {
            Operand::A => (&*div_a, &red.a),
            Operand::B => (&*div_b, &red.b),
        };
        let root = div_k.root(fe.kept);
        let kept = |e: EdgeKey| {
            kept_body.get_edge(e).and_then(|edge| {
                [edge.he_plus, edge.he_minus]
                    .into_iter()
                    .filter_map(|he| kept_body.face_of_half_edge(he))
                    .find(|&f| div_k.root(f) == root)
            })
        };
        red.fates.across.iter().any(|&(holder, face, pair)| {
            holder == fe.holder
                && face == fe.face
                && pair.contains(&fe.edge)
                && kept(pair[0]).is_some()
                && kept(pair[0]) == kept(pair[1])
        })
    };
    let skipped: Vec<bool> = red.fates.edges.iter().map(|fe| across(red, fe)).collect();
    for fe in red
        .fates
        .edges
        .iter()
        .zip(&skipped)
        .filter_map(|(fe, &skip)| (!skip).then_some(fe))
    {
        let (div_d, kept_body) = match fe.holder {
            Operand::A => (&*div_b, &red.a),
            Operand::B => (&*div_a, &red.b),
        };
        let root = div_d.root(fe.face);
        let edge = kept_body
            .get_edge(fe.edge)
            .ok_or_else(|| desync("a fate edge no longer resolves"))?;
        for he in [edge.he_plus, edge.he_minus] {
            let x = kept_body
                .get_half_edge(he)
                .ok_or_else(|| desync("a fate edge's half-edge no longer resolves"))?
                .start;
            match end_at(red, fe.holder, x, root, div_d)? {
                End::At(_) => {}
                End::Touch(t) => {
                    if !touch_points
                        .iter()
                        .any(|&(h, r, k, _)| (h, r, k) == (fe.holder, root, t))
                    {
                        let key = kept_body
                            .get_vertex(t)
                            .map(|v| v.point)
                            .ok_or_else(|| desync("a fate cut's touching vertex has no point"))?;
                        let p = kept_body
                            .get_point(key)
                            .copied()
                            .ok_or_else(|| desync("a fate cut's touching vertex has no point"))?;
                        if !touch_points
                            .iter()
                            .any(|&(h, r, _, k)| (h, r, k) == (fe.holder, root, key))
                        {
                            touches.push((fe.holder, root, t, p));
                        }
                        touch_points.push((fe.holder, root, t, key));
                    }
                }
            }
        }
    }
    let touches = touches
        .into_iter()
        .map(|(h, root, t, p)| {
            let (div_d, body) = match h {
                Operand::A => (&*div_b, &red.b),
                Operand::B => (&*div_a, &red.a),
            };
            Ok((h, root, t, lies(body, root, div_d, p, band)?, p))
        })
        .collect::<Result<Vec<_>, BooleanError>>()?;
    for (holder, root, t, reading, p) in touches {
        let body = match holder {
            Operand::A => &mut red.b,
            Operand::B => &mut red.a,
        };
        // The point is read, not the records: at a vertex of the face's
        // division (such as another kept vertex's ring vertex there, which
        // no record pairs with this one) the chord ends there, its corner
        // read by its direction (`place`); inside one piece it ends at a
        // lone vertex.
        let ws = match reading {
            Lies::Inside(face) => vec![super::vtxfac::ring_vertex_in(body, face, p, tol)?.0],
            Lies::AtCorners(corners) => corners,
            Lies::OnEdge(_) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: Unplaced::ON_EDGE,
                }
                .into());
            }
            Lies::Nowhere(unplaced) => return Err(unplaced.refused().into()),
        };
        let key = touch_points
            .iter()
            .find(|&&(h, r, k, _)| (h, r, k) == (holder, root, t))
            .map(|&(.., key)| key);
        for &(h, r, k, at) in &touch_points {
            if (h, r) == (holder, root) && Some(at) == key {
                touched.insert((holder, root, k), ws.clone());
            }
        }
    }
    for (fe, across) in red.fates.edges.clone().into_iter().zip(skipped) {
        let dropped = fe.holder.other();
        let (div_k, div_d) = match fe.holder {
            Operand::A => (&*div_a, &mut *div_b),
            Operand::B => (&*div_b, &mut *div_a),
        };
        let root = div_d.root(fe.face);
        if !done.insert((fe.holder, fe.edge, root)) {
            continue;
        }
        let kept_body = match fe.holder {
            Operand::A => &red.a,
            Operand::B => &red.b,
        };
        let edge = kept_body
            .get_edge(fe.edge)
            .ok_or_else(|| desync("a fate edge no longer resolves"))?;
        let kept_root = div_k.root(fe.kept);
        let kept_he = [edge.he_plus, edge.he_minus]
            .into_iter()
            .find(|&he| {
                kept_body
                    .face_of_half_edge(he)
                    .is_some_and(|f| div_k.root(f) == kept_root)
            })
            .ok_or_else(|| desync("a fate edge does not bound its kept face"))?;
        let curve = kept_body
            .get_curve_geom(edge.curve)
            .and_then(|c| c.certified())
            .cloned()
            .ok_or_else(|| desync("a fate edge has no certified curve"))?;
        let forward = kept_he == edge.he_plus;
        let kept_mate = kept_body
            .mate(kept_he)
            .ok_or_else(|| desync("a fate edge has no mate"))?;
        // Each end's tangent, then its chord to the kept edge's midpoint.
        let ends = [kept_he, kept_mate].map(|he| {
            let x = kept_body.get_half_edge(he)?.start;
            let read = |secant| leaving(kept_body, he, secant).map(Leaving::ray);
            Some((x, [read(false)?, read(true)?]))
        });
        let [Some((x0, r0)), Some((x1, r1))] = ends else {
            return Err(desync("a fate edge has no direction").into());
        };
        let at = |x| match end_at(red, fe.holder, x, root, div_d)? {
            End::At(ws) => Ok(ws),
            End::Touch(t) => {
                touched
                    .get(&(fe.holder, root, t))
                    .cloned()
                    .ok_or(BooleanError::JoinDesync {
                        what: "a touching fate cut end is not placed",
                    })
            }
        };
        if across
            && [x0, x1].into_iter().try_fold(false, |touch, x| {
                Ok::<_, BooleanError>(
                    touch || matches!(end_at(red, fe.holder, x, root, div_d)?, End::Touch(_)),
                )
            })?
        {
            continue;
        }
        let (w0, w1) = (at(x0)?, at(x1)?);
        let body = match dropped {
            Operand::A => &mut red.a,
            Operand::B => &mut red.b,
        };
        let (Some(p0), Some(p1)) = (
            place((body, dropped), (&w0, &w1), root, div_d, r0, band)?,
            place((body, dropped), (&w1, &w0), root, div_d, r1, band)?,
        ) else {
            return Err(BooleanError::ClassificationInvariant {
                what: "a fate cut's end left the dropped face as it was cut",
            }
            .into());
        };
        let run = match (p0, p1) {
            (Place::Along(h), _) => h,
            (_, Place::Along(h)) => body
                .mate(h)
                .ok_or_else(|| desync("an edge along a fate edge has no mate"))?,
            _ if across => continue,
            _ => {
                let c = chord(body, (p0, p1), (&curve, forward), div_d, band, tol)?;
                let h = body
                    .get_half_edge(c)
                    .cloned()
                    .ok_or_else(|| desync("a fate cut's chord no longer resolves"))?;
                let end = body
                    .half_edge_end(c)
                    .ok_or_else(|| desync("a fate cut's chord has no end"))?;
                out.chords.push(Chord {
                    operand: dropped,
                    edge: h.edge,
                    along: fe.edge,
                    ends: [h.start, end],
                    kept_ends: [x0, x1],
                });
                slits.push((dropped, c));
                c
            }
        };
        let mate = body
            .mate(run)
            .ok_or_else(|| desync("a fate cut's run has no mate"))?;
        halves.push((dropped, if fe.aligned { run } else { mate }, kept_he));
    }
    for (operand, c) in slits {
        let body = match operand {
            Operand::A => &red.a,
            Operand::B => &red.b,
        };
        let mate = body
            .mate(c)
            .ok_or_else(|| desync("a fate cut's chord has no mate"))?;
        if face_of_he(body, c)? == face_of_he(body, mate)? {
            return Err(BooleanError::ClassificationInvariant {
                what: "a fate cut leaves a slit in the dropped face",
            }
            .into());
        }
    }
    let mut corners: Vec<(Operand, FaceKey, FaceKey)> = Vec::new();
    for &(operand, half, kept_he) in &halves {
        let (body, kept_body) = match operand {
            Operand::A => (&red.a, &red.b),
            Operand::B => (&red.b, &red.a),
        };
        corners.push((
            operand,
            face_of_he(body, half)?,
            face_of_he(kept_body, kept_he)?,
        ));
    }
    for fc in red.fates.corners.clone() {
        let dropped = fc.holder.other();
        let (body, kept_body, div_k) = match fc.holder {
            Operand::A => (&red.b, &red.a, &*div_a),
            Operand::B => (&red.a, &red.b, &*div_b),
        };
        let div_d = match fc.holder {
            Operand::A => &*div_b,
            Operand::B => &*div_a,
        };
        let copies = [dropped, fc.holder].map(|o| NullCopies::of_operand(&red.null_edges, o));
        let face = sector_piece((body, dropped), fc.dropped, (div_d, &copies[0]), band)?;
        let by = match fc.kept {
            KeptAt::Sector(s) => {
                sector_piece((kept_body, fc.holder), s, (div_k, &copies[1]), band)?
            }
            KeptAt::Pierced(f, at, d) => {
                let root = div_k.root(f);
                let ring = red.pierce_rings.iter().find(|r| {
                    r.operand == fc.holder && div_k.root(r.face) == root && r.pierces.contains(&at)
                });
                match (one_piece(kept_body, root, div_k), ring) {
                    (Some(one), _) => one,
                    (None, Some(ring)) => {
                        let copies =
                            NullCopies::of_operand(&red.null_edges, fc.holder).of(ring.ring_vertex);
                        let held = (kept_body, fc.holder);
                        piece_holding(held, (&copies, HeldAt::RingVertex), root, div_k, d, band)?
                    }
                    // No ring serves the pierce where the point lies on the
                    // kept face's division: the piece is the one whose
                    // corner there holds the sector, at a vertex of the
                    // division or inside an edge of it.
                    (None, None) => {
                        let p = body
                            .get_vertex(at)
                            .and_then(|v| body.get_point(v.point))
                            .copied()
                            .ok_or_else(|| desync("a piercing vertex has no point"))?;
                        match lies(kept_body, root, div_k, p, band)? {
                            Lies::Inside(one) => one,
                            Lies::AtCorners(corners) => piece_holding(
                                (kept_body, fc.holder),
                                (&corners, HeldAt::DivisionVertex),
                                root,
                                div_k,
                                d,
                                band,
                            )?,
                            Lies::OnEdge(sides) => piece_beside(kept_body, &sides, p, d, band)?,
                            Lies::Nowhere(unplaced) => return Err(unplaced.refused().into()),
                        }
                    }
                }
            }
        };
        corners.push((dropped, face, by));
    }
    let mut substituted: Vec<SubstitutedFace> = corners
        .into_iter()
        .map(|(operand, face, by)| SubstitutedFace { operand, face, by })
        .collect();
    substituted.sort();
    substituted.dedup();
    out.substituted = substituted;
    Ok(out)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Band, Point3, Sign, Tol, Vec3};

    use super::ring_winding;
    use crate::boolean::{BooleanDecision, BooleanError};

    /// **A ring whose winding reads in band escalates as its own
    /// decision**: the triangle (0, 0), (1, 0), (0.5, h) in z = 0 winds
    /// counterclockwise about +z, and its reading (twice its area over its
    /// length) is h / √5. At h = ±1 that is clear; at h = 5.5 √5 ε it is
    /// 5.5 ε, mid-band (ε to 10 ε), and refuses
    /// [`BooleanDecision::CutRingWinding`], not a broken invariant.
    #[test]
    fn a_ring_winding_in_band_escalates_as_its_own_decision() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let ring = |h: f64| {
            [
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 0.0, 0.0),
                Point3::new(0.5, h, 0.0),
            ]
        };
        let up = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(ring_winding(&ring(1.0), up, band).unwrap(), Sign::Positive);
        assert_eq!(ring_winding(&ring(-1.0), up, band).unwrap(), Sign::Negative);
        let err =
            ring_winding(&ring(5.5 * 5f64.sqrt() * tol.eps()), up, band).expect_err("in band");
        assert!(
            matches!(
                err,
                BooleanError::Escalated {
                    decision: BooleanDecision::CutRingWinding,
                    ..
                }
            ),
            "{err:?}"
        );
    }

    /// **A kept edge leaving along a corner's bound is read one order
    /// down**: in `z = 0` about `+z`, the cusp a crescent narrows to,
    /// from the outer rim `r = 0.5` leaving `+y` round to the inner rim
    /// `r = 0.3` arriving along it, both bending toward `−x`. An arc
    /// leaving `+y` with the curvature `1/r` lies inside the cusp for
    /// `0.3 < r < 0.5`, and outside it bending harder or straight; the
    /// outer rim's own curvature ties with it. At a half-plane corner, an
    /// arc leaving along the bound it arrives by lies in the corner iff it
    /// bends into it.
    #[test]
    fn a_kept_edge_tangent_to_a_bound_reads_its_side_one_order_down() {
        use super::{Holds, Leaving, corner_holds};
        use geom_brep::LeverBounds;
        let band = Band::linear(Tol::witness()).unwrap();
        let (up, y) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.0, 1.0, 0.0));
        let edge = |dir, k: f64| Leaving {
            dir,
            reach: LeverBounds::Exact(0.6),
            bend: Some(Vec3::new(-k, 0.0, 0.0)),
        };
        let at = |k: f64, ((out, back), (ko, kb)): ((Vec3<f64>, Vec3<f64>), (f64, f64))| {
            corner_holds(
                edge(y, k).ray(),
                (edge(out, ko), edge(back, kb)),
                || Ok(false),
                up,
                band,
            )
            .unwrap()
        };
        let cusp = ((y, y), (2.0, 1.0 / 0.3));
        assert_eq!(at(1.0 / 0.4, cusp), Holds::In, "r = 0.4 runs into the cusp");
        assert_eq!(
            at(5.0, cusp),
            Holds::Out,
            "r = 0.2 bends past the inner rim"
        );
        assert_eq!(
            at(0.0, cusp),
            Holds::Out,
            "a line runs outside the outer rim"
        );
        assert_eq!(
            at(2.0, cusp),
            Holds::Tied { along_out: true },
            "the outer rim's own curvature ties with it"
        );
        let half = ((-y, y), (0.0, 0.0));
        assert_eq!(
            at(-1.0, half),
            Holds::In,
            "bending toward +x, into the corner"
        );
        assert_eq!(at(1.0, half), Holds::Out, "bending toward -x, out of it");
        assert_eq!(
            at(0.0, half),
            Holds::Tied { along_out: false },
            "straight along the bound it arrives by"
        );
    }
}

/// **A fate corner whose arm is known only from both sides refuses
/// where its two readings disagree**: the arm a spline edge's reach from
/// its start gives, on a window where its parameter crawls (clustered
/// knots, weights 1e±3), read from below by the samples and from above
/// through the whole carrier's speed bound about 90 times over. A cut
/// `θ = 1e-6` off the corner's first bound reads along it at the bound
/// from below and strictly inside at the bound from above, so the corner
/// places it neither way: it refuses as the sectors' coincidence, its
/// diagnosis the straddle, as an in-band corner reading refuses.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod corner_bounds_rows {
    use geom_brep::LeverBounds;
    use geom_core::{Band, Point3, Vec3};

    use super::{Holds, Leaving, Ray, corner_holds};
    use crate::boolean::{BooleanDecision, BooleanError};

    /// A straight bound leaving along `dir`, reaching `reach`.
    fn bound(dir: Vec3<f64>, reach: LeverBounds<f64>) -> Leaving<f64> {
        Leaving {
            dir,
            reach,
            bend: Some(Vec3::new(0.0, 0.0, 0.0)),
        }
    }

    /// The straddle a corner refuses with, under `name`.
    fn straddles(got: Result<Holds, BooleanError>, name: &str) {
        match got {
            Err(BooleanError::Escalated {
                decision: BooleanDecision::Coincidence(..),
                diag,
            }) => assert_eq!(diag.predicate, Some(name)),
            other => panic!("the corner refuses with {name}, got {other:?}"),
        }
    }

    #[test]
    fn a_corner_whose_arm_is_known_from_both_sides_refuses_where_they_disagree() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let p = Point3::new;
        let wild = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                geom_core::spline::KnotVector::clamped(
                    vec![
                        0.0,
                        0.0,
                        0.0,
                        0.0,
                        0.5,
                        0.500_000_1,
                        0.500_000_2,
                        1.0,
                        1.0,
                        1.0,
                        1.0,
                    ],
                    3,
                )
                .unwrap(),
                vec![
                    p(0.0, 0.0, 0.0),
                    p(1.0, 2.0, 0.0),
                    p(2.0, -1.0, 1.0),
                    p(3.0, 0.5, 0.0),
                    p(4.0, 1.0, 0.0),
                    p(5.0, 0.0, 2.0),
                    p(6.0, 0.0, 0.0),
                ],
                vec![1.0, 1e3, 1e-3, 1e3, 1e-3, 1e3, 1.0],
            )
            .unwrap(),
        ));
        let (t0, t1) = (0.2, 0.2001);
        let arm = geom_brep::edge_reach_bounds(&wild, t0, t1, wild.eval(t0));
        let LeverBounds::Between { below, above } = arm else {
            panic!("a spline's reach is known from both sides, got {arm:?}");
        };
        let theta = 1e-6_f64;
        let d = Vec3::new(theta.cos(), theta.sin(), 0.0);
        let bounds = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let n = Vec3::new(0.0, 0.0, 1.0);
        let at = |arm| {
            let bounds = (bound(bounds.0, arm), bound(bounds.1, arm));
            corner_holds(Ray::bisector(d), bounds, || Ok(false), n, band)
        };
        // Each bound alone places the cut, and they differ.
        assert!(matches!(at(LeverBounds::Exact(above)), Ok(Holds::In)));
        assert!(matches!(at(LeverBounds::Exact(below)), Ok(Holds::Along)));
        // Read at both, the corner refuses with the straddle.
        straddles(at(arm), "bool_fate_corner_straddle");
    }

    /// **A kept edge tangent to a bound is read one order down at both
    /// ends of a reach known from both sides**: a quarter corner from
    /// `x̂` to `ŷ` about `+z`, the edge leaving along `x̂`, the straight
    /// bound with it, bending into the corner by `κ = 1.2e-9`. Its
    /// parting `κ·l²/2` reads Zero at the reach from below, `l = 1`, and
    /// past the band at the reach from above, `l = 5`: from below alone
    /// the edge ties with the bound, from above alone it lies in the
    /// corner, and read at both the corner refuses with the straddle.
    #[test]
    fn a_tangent_edge_read_one_order_down_refuses_where_its_reach_straddles() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (x, y, n) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let at = |reach| {
            let edge = Leaving {
                dir: x,
                reach,
                bend: Some(Vec3::new(0.0, 1.2e-9, 0.0)),
            };
            let bounds = (bound(x, reach), bound(y, LeverBounds::Exact(1.0)));
            corner_holds(edge.ray(), bounds, || Ok(false), n, band)
        };
        assert!(matches!(
            at(LeverBounds::Exact(1.0)),
            Ok(Holds::Tied { along_out: true })
        ));
        assert!(matches!(at(LeverBounds::Exact(5.0)), Ok(Holds::In)));
        straddles(
            at(LeverBounds::Between {
                below: 1.0,
                above: 5.0,
            }),
            "bool_fate_corner_bend_straddle",
        );
    }

    /// **A cut `θ` inside a corner beside its long bound**, the short
    /// bound setting the corner's arm: `θ` times the short bound reads
    /// Zero, `θ` times the long one definite. Read at the long bound's
    /// reach, a cut with no reach of its own or a long one lies in the
    /// corner, where the arm would read it along the bound; a cut no
    /// longer than the short bound lies along it.
    #[test]
    fn a_cut_beside_a_long_bound_is_read_at_the_shorter_of_their_reaches() {
        let band = Band::new(1e-9, 1e-8).unwrap();
        let (long, short) = (LeverBounds::Exact(1.0), LeverBounds::Exact(1e-6));
        let theta = 1e-4_f64;
        let d = Vec3::new(theta.cos(), theta.sin(), 0.0);
        let bounds = (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        let n = Vec3::new(0.0, 0.0, 1.0);
        let at = |reach, (out, back)| {
            let d = Ray {
                dir: d,
                reach,
                bend: None,
            };
            let bounds = (bound(bounds.0, out), bound(bounds.1, back));
            corner_holds(d, bounds, || Ok(false), n, band)
        };
        assert!(
            matches!(at(None, (long, short)), Ok(Holds::In)),
            "no reach of its own"
        );
        assert!(
            matches!(at(Some(long), (long, short)), Ok(Holds::In)),
            "a long cut"
        );
        assert!(
            matches!(at(Some(short), (long, short)), Ok(Holds::Along)),
            "a short cut"
        );
        assert!(
            matches!(at(None, (short, short)), Ok(Holds::Along)),
            "at the arm"
        );
    }
}

/// **A tangent tied with a corner's bound places no corner**: the top
/// face of the unit brick at its corner `(0, 0, 1)`, bounded there by
/// its edges along `+x` and `+y`, one leaving the vertex and one
/// arriving. A straight cut leaving along either edge ties with it, both
/// bending by zero, and its secant, pointed into the face, lies in the
/// face's corner: `place` refuses rather than read that secant, at the
/// bound the face's edge leaves by and at the one it arrives by. The
/// same cut bending into the face places in the corner.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tie_rows {
    use geom_brep::LeverBounds;
    use geom_core::{Band, Point3, Tol, Vec3};

    use super::{CutError, Limit, Place, Ray, place};
    use crate::boolean::Operand;
    use crate::lineage::{FaceDivisions, LineageRows};

    #[test]
    fn a_tangent_tied_with_either_bound_places_no_corner() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let body =
            crate::test_support_fixtures::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let top = body
            .faces()
            .map(|(f, _)| f)
            .find(|&f| {
                crate::face_normal::face_outward_normal(&body, f).is_some_and(|n| n.vec().z > 0.5)
            })
            .expect("the top face");
        let corner = body
            .vertices()
            .find(|(_, v)| {
                body.get_point(v.point)
                    .is_some_and(|p| (*p - Point3::new(0.0, 0.0, 1.0)).norm() < 1e-12)
            })
            .map(|(k, _)| k)
            .expect("the corner");
        let divisions = FaceDivisions::new(LineageRows::FaceDivisions);
        let diagonal = Vec3::new(1.0, 1.0, 0.0).normalize();
        let at = |dir: Vec3<f64>, bend: Vec3<f64>| {
            let reach = Some(LeverBounds::Exact(0.5));
            let tangent = Ray {
                dir,
                reach,
                bend: Some(bend),
            };
            let secant = Ray {
                dir: diagonal,
                reach,
                bend: None,
            };
            place(
                (&body, Operand::A),
                (&[corner], &[]),
                top,
                &divisions,
                [tangent, secant],
                band,
            )
        };
        let straight = Vec3::new(0.0, 0.0, 0.0);
        for (dir, into) in [
            (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            (Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0)),
        ] {
            assert!(
                matches!(at(dir, straight), Err(CutError::Limit(Limit::NoCorner))),
                "tied along {dir:?}: {:?}",
                at(dir, straight)
            );
            assert!(
                matches!(at(dir, into), Ok(Some(Place::Corner(_)))),
                "bending into the face along {dir:?}: {:?}",
                at(dir, into)
            );
        }
    }
}

/// **A spline leaves a vertex along its tangent, not its chord**: the
/// sweep rows' Bézier sheet, whose arc leaves `(0, 0, 0)` along
/// `(1, 2, 0)` while its chord runs along `x̂`
/// (`sectors::spline_departure_rows`, the sector walk's twin).
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod spline_departure_rows {
    use crate::boolean::reduce::planar_lane_carrier_rows::arc_sheet;
    use geom_core::Vec3;

    #[test]
    fn a_fate_cut_reads_a_splines_tangent() {
        let (body, edge) = arc_sheet(1.0, 1.0);
        let he = body.get_edge(edge).unwrap().he_plus;
        let super::Leaving {
            dir, reach: len, ..
        } = super::departure(&body, he).unwrap();
        let tangent = Vec3::new(1.0, 2.0, 0.0).normalize();
        assert!(
            (dir - tangent).norm() < 1e-12,
            "the arc leaves along its tangent (1, 2, 0)/√5, got {dir:?}"
        );
        // The arc's farthest point from its start is its end, 2 off: the
        // samples read it from below (the end is one), and the reach from
        // above passes it by its samples' Lipschitz slack, short of the
        // control net's reach √5.
        let geom_brep::LeverBounds::Between { below, above } = len else {
            panic!("a spline's reach is known from both sides, got {len:?}");
        };
        assert!(
            (below - 2.0).abs() < 1e-12 && (2.0..2.2).contains(&above),
            "with its reach from the start between 2 and the bound's slack: {below}, {above}"
        );
    }
}

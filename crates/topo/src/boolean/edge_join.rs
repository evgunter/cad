//! **The join**: a vertex of valence 2 whose two edges lie on one
//! structural carrier between the same two faces is no corner, so the
//! two edges are one edge cut for no reason (`docs/DESIGN.md`, maximal
//! edges). The join kills the vertex and makes the two edges one.
//!
//! **The join door** ([`Body::join_edges`](crate::construction::Construct::join_edges)) is the one public spelling:
//! every door that finishes a body ends with it (`docs/DESIGN.md`, the
//! merge stage), and it returns each join it made ([`EdgeJoin`]) for a
//! door that carries records or names keyed by the body's cells. The
//! door is whole: the kills run on a staged clone, so on any refusal
//! the body is as found (a reading in the band refuses before any kill,
//! and a reading or kill that refuses later refuses on the clone), and
//! a body that carried pcurve rows has them re-derived, so no row is
//! left keyed by a dead cell. A door that is not the boolean carries
//! its refusal typed ([`JoinRefusal`]); the split's halves reach it
//! through the merge door. The offset doors, the shell and the blend
//! join on their own staging (`Body::join_edges_within`), the offset
//! doors and the blend over their own scope alone.
//!
//! The boolean's output stages run it after the merge ([`join_stage`])
//! and write, per join, the substitution rows that carry every contact
//! record naming the cells it replaces onto the edge it makes, through
//! the op's one substitution door ([`super::ops::carry`]),
//! and hand back the class Zero of each join made on an iso family
//! ([`ClassZero`]), which the op records as a coincidence
//! (`[coincidence.door]`).
//!
//! Joinable is one predicate ([`joinable`]), shared by the join and by
//! [`joinable_vertices`]. The two edges' face pairs are one pair of
//! distinct faces, and the edges lie on one carrier by structure:
//! - **planar**: both faces on `Plane` carriers of distinct keys, both
//!   edges certified lines described as the `Intersection` of those
//!   keys. Two planes that cross meet in one line; two that do not
//!   certify no intersection, so coplanar faces sharing a bent boundary
//!   are never joined.
//! - **a locus of the surface pair**: both edges `Intersection`, or
//!   both `TangentIntersection`, citing the faces' two surface keys, and
//!   the locus one regular curve at the vertex: the surfaces definitely
//!   transverse there, or, for the tangent arm, definitely determined
//!   one order up (`tangent_second_order`).
//! - **an iso family of one chart**: both faces on one surface key,
//!   both edges `Chart` images in it on one iso family, and the chart
//!   regular at the vertex.
//! - **an iso family of one of the pair's charts**: both edges `OnPair`
//!   citing the faces' two surface keys. The pair under-determines the
//!   locus, so it does not decide the branch; the first of the two
//!   charts (the plus face's, then the minus face's) on which both
//!   carriers lie on one iso family does, regular at the vertex as
//!   above. A pair on no family of either chart is not joined, nor is a
//!   plane × plane pair, which the planar path takes and joins only as
//!   an `Intersection`. The joined edge is asked the must-carry rule
//!   again over its own span.
//!
//! The joined edge runs on the kept edge's carrier through the vertex,
//! so the kept edge is one whose carrier has no crease there
//! ([`crease_at`]: a spline carrier at its clamped end, or at an
//! interior knot of multiplicity at least its degree). A spline that
//! ends at the vertex beside a line or a circle on the same locus is
//! the edge the join kills. Where both carriers crease there, as two
//! spline pieces meeting at a knot of full multiplicity do, the vertex
//! is never joined, whatever arm their faces take: every face chart and
//! edge carrier is C¹ by its knot structure strictly inside its cell
//! (Ev's PR 5913 ruling), so it is the vertex the knot structure puts
//! there. Whether an end lies on a crease is two parameters compared
//! exactly ([`crease_read`]). It is decided as a crease or as none, which
//! picks the kept edge ([`kept_edge`]: the edge with none, the second
//! where neither has one, no join where both have one). An enclosure of
//! the parameter that straddles a crease is undecided. The join is then
//! neither taken nor left: it refuses typed, as every reading in the
//! band does ([`JoinUndecided`], [`JoinReading::Crease`]), at the join
//! and at tier 3's check 11 alike ([`joinable_at_rest`]).
//!
//! Where the joined edge's far parameter comes from, rung by rung, is
//! [`joined_spec`]'s.
//!
//! Distinct components of one locus never share a point, and one iso
//! family has one member through a regular point, so the vertex decides
//! the branch and nothing is compared between the two edges. A surface
//! singularity (a cone's apex) or a chart's (a sphere's pole) is never
//! joinable; a reading in the margin band refuses typed
//! ([`JoinUndecided`]).
//!
//! A vertex that shares its point key with another vertex is not
//! joinable either: it is one cone of a pinch (Ev, PR 4057), or a
//! corner of one lump touching another where a boolean decided them one
//! point (`ops::record_classes`), and the joined edge would run through
//! the other vertex on that point, a vertex-on-edge contact no record
//! names. The boolean's own join frees a vertex its operands' carried
//! records name, where no other vertex of its shell sits on its key
//! ([`free_vertices`]): those records follow the join, so the contact
//! stays named.
//!
//! **The closed join.** Two edges that share both ends join into one
//! closed edge: the kill leaves the kept edge a self-loop at the far
//! vertex. Where no other edge ends there, that vertex is the edge's
//! *conventional vertex* (DESIGN.md, maximal edges): a vertex whose only
//! edge is one closed edge, at both its ends ([`is_conventional_vertex`]).
//! It has no identity of its own, so the join substitutes it, as well as
//! the killed vertex, onto the kept edge.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign, Tol};

use super::ops::{Descendants, describe_minted_edges};
use super::{BooleanError, Cell};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, HalfEdgeKey, VertexKey};
use crate::geometry::PointKey;
use crate::live::{linked, proven};
use crate::readback::edge_sides_of;

/// A joinable vertex's two edges: `gone` dies with the vertex, `kept`
/// becomes the joined edge; `he` is `gone`'s half-edge that ends at the
/// vertex, the one `kev` kills it through. `far` is `gone`'s other end;
/// where it is `kept`'s other end too the join is closed.
struct Join {
    he: HalfEdgeKey,
    gone: EdgeKey,
    kept: EdgeKey,
    far: VertexKey,
    closed: bool,
    planar: bool,
    /// Where the two edges are one on an iso family their carriers were
    /// read on, the Zero that reading stands on: the most either carrier
    /// misses the family by ([`geom_brep::IsoReading::zero`],
    /// [`geom_core::MarginDiag::worst`]).
    class: Option<geom_core::MarginDiag>,
}

/// **The class Zero a join acted on**: `gone` and `kept` (keys as the
/// join found them) were made one edge because each carrier was read on
/// one iso family of one chart, and `margin` is the most either carrier
/// misses that family by: the larger of both readings' Zeros
/// ([`geom_brep::IsoReading::zero`]). The boolean records
/// it as a coincidence (`[coincidence.door]`).
#[derive(Clone, Copy, Debug)]
pub(super) struct ClassZero {
    pub(super) gone: EdgeKey,
    pub(super) kept: EdgeKey,
    pub(super) margin: geom_core::MarginDiag,
}

/// **What each edge a run of joins made covers**, read off the joins'
/// records alone, in the order made: each live `kept` edge → every
/// edge it holds, itself first, then each `gone` in the order the joins
/// took them, a later join's `gone` that was an earlier one's `kept`
/// bringing its own cover with it. The one chase every door that
/// carries records or names over a join reads (the boolean's
/// [`crate::BooleanNaming::joined_edge`], and the split's, the shell's
/// and the blend's emitters); an edge no join touched has no row.
#[must_use]
pub fn join_covers(joins: &[EdgeJoin]) -> BTreeMap<EdgeKey, Vec<EdgeKey>> {
    let mut covers: BTreeMap<EdgeKey, Vec<EdgeKey>> = BTreeMap::new();
    for j in joins {
        let gone = covers.remove(&j.gone).unwrap_or_else(|| vec![j.gone]);
        covers
            .entry(j.kept)
            .or_insert_with(|| vec![j.kept])
            .extend(gone);
    }
    covers
}

/// The live edge `edge` is part of after `joins`: itself, or the edge a
/// join killed it into, followed through every later join.
#[must_use]
pub fn joined_edge(joins: &[EdgeJoin], edge: EdgeKey) -> EdgeKey {
    joins
        .iter()
        .fold(edge, |at, j| if j.gone == at { j.kept } else { at })
}

/// The live cell `cell` is after `joins`: a joined-away vertex or edge is
/// the edge that holds its interior now ([`joined_edge`]), and every
/// other cell is itself.
#[must_use]
pub fn joined_cell(joins: &[EdgeJoin], cell: super::Cell) -> super::Cell {
    use super::Cell;
    match cell {
        Cell::Vertex(v) => joins
            .iter()
            .find(|j| j.vertex == v)
            .map_or(cell, |j| Cell::Edge(joined_edge(joins, j.kept))),
        Cell::Edge(e) => Cell::Edge(joined_edge(joins, e)),
        Cell::Face(_) => cell,
    }
}

/// One join an output stage made ([`join_stage`]): `vertex` and `gone`
/// are dead, and `kept` holds both their interiors and its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeJoin {
    /// The joined-away vertex.
    pub vertex: VertexKey,
    /// The edge killed with it.
    pub gone: EdgeKey,
    /// The edge that is the two edges now.
    pub kept: EdgeKey,
    /// Where the join closed `kept` into a self-loop that is the
    /// surviving vertex's only edge: that vertex, now conventional.
    pub conventional: Option<VertexKey>,
}

/// **A joinable reading the band cannot decide**: whether `vertex` is
/// a regular point of the carrier its two edges lie on, or which iso
/// family an edge's chart image is on, read inside the margin band, so
/// the vertex is neither joined nor left (D4 ¶3). Its own Display is
/// the Boolean's refusal ([`BooleanError::JoinUndecided`]); the join
/// door ([`JoinRefusal`]) and the at-rest check
/// (`ValidationError::JoinUndecidedAtRest`) render it from
/// [`JoinUndecided::diag`] themselves.
#[derive(Clone, Debug, PartialEq)]
pub struct JoinUndecided {
    /// The valence-2 vertex.
    pub vertex: VertexKey,
    /// The reading that escalated.
    pub reading: JoinReading,
}

impl JoinUndecided {
    /// The escalation this refusal carries, where its reading is a
    /// margin the band could not decide: what a reader over stored
    /// geometry ends at rest itself ([`JOIN_SUBJECT`], [`JOIN_LEVER`]).
    #[must_use]
    pub fn diag(&self) -> Option<&Indeterminate> {
        match &self.reading {
            JoinReading::Regularity(diag)
            | JoinReading::Crease(diag)
            | JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Undecided(diag)) => Some(diag),
            JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(_)) => None,
        }
    }
}

/// The question an undecided join asks ([`JoinRefusal::Undecided`]).
pub const JOIN_SUBJECT: &str = "whether two edges meeting at a vertex on one curve are one edge";

/// The undecided join's geometry lever.
pub const JOIN_LEVER: &str = "move the vertex clear of the pole, apex or tangency it sits near";

/// Which reading of [`JoinUndecided`] escalated.
#[derive(Clone, Debug, PartialEq)]
pub enum JoinReading {
    /// A regularity margin at the vertex: its distance from a surface
    /// or chart singularity, the pair's wedge, or the tangent arm's
    /// second order.
    Regularity(Indeterminate),
    /// The class arm of an edge's chart image.
    ChartClass(geom_brep::IsoFamilyRefusal),
    /// Whether an edge's end parameter lies on one of its spline
    /// carrier's creases ([`crease_at`]): an enclosure of the parameter
    /// straddles a crease.
    Crease(Indeterminate),
}

/// The whole refusal: a regularity reading asks [`JOIN_SUBJECT`], a
/// sized margin, and ends in [`Indeterminate::ending`] under
/// [`geom_core::COINCIDENCE_RECOURSE`]; a chart-class reading asks its
/// own question ([`geom_brep::ISO_FAMILY_CLAUSE`]), a form selection on
/// no size the user chose, and ends in that lever alone
/// ([`geom_brep::recourse::LeverOnly`]).
///
/// `at` ends the image's own refusal at the door that reads it; `None`
/// writes it as its payload, as `Display` does.
impl JoinUndecided {
    fn write(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        at: Option<geom_brep::recourse::ReadAt>,
    ) -> core::fmt::Result {
        match &self.reading {
            JoinReading::Regularity(diag) | JoinReading::Crease(diag) => write!(
                f,
                "{}",
                diag.undecided(JOIN_SUBJECT, diag.ending(geom_core::COINCIDENCE_RECOURSE))
            ),
            JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Undecided(diag)) => write!(
                f,
                "{}",
                diag.undecided(
                    format_args!(
                        "whether an edge meeting another at a vertex on one curve {}",
                        geom_brep::ISO_FAMILY_CLAUSE
                    ),
                    diag.lever_ending(geom_core::COINCIDENCE_RECOURSE)
                )
            ),
            // The image's own refusal, no class reading: an incidence read
            // among them ends by its own ending, never as undecided.
            JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(e)) => write!(
                f,
                "two edges meeting at a vertex on one surface read their family off a chart image \
                 that did not derive: {}",
                geom_brep::recourse::read_at(e, at)
            ),
        }
    }
}

geom_brep::ended_by_write!(JoinUndecided);

/// `e`'s certified curve, where it is a line: the one home of "a
/// certified line edge", read by the join and by the carried-record
/// door, whose edge-edge lineage reads an edge as its two ends.
pub(super) fn certified_line<'a, T: Real>(
    body: &'a Body<T>,
    e: EdgeKey,
    d: &crate::entity::Edge,
) -> Option<&'a geom_brep::EdgeCurve<T>> {
    body.edge_curve_linked(e, d)
        .certified()
        .filter(|c| !c.carrier().is_curved())
}

/// Whether `v` is a **conventional vertex**: its only edge is one
/// closed edge, at both its ends. Defined by structure, so the tier-2
/// check and every reader that skips such a vertex read the same set.
/// A sweep's closed rim meets its seam strut there and stays outside.
pub fn is_conventional_vertex<T: Real>(body: &Body<T>, v: VertexKey) -> bool {
    let Some(vertex) = body.get_vertex(v) else {
        return false;
    };
    let Some(first) = vertex.emanating else {
        return false;
    };
    let Some(he) = body.get_half_edge(first) else {
        return false;
    };
    body.edge_vertices(he.edge) == Some((v, v))
        && body.vertex_orbit(first).is_some_and(|orbit| {
            orbit
                .iter()
                .all(|&h| body.get_half_edge(h).is_some_and(|d| d.edge == he.edge))
        })
}

/// What one pass reads off the body before it asks any vertex: every
/// half-edge starting at each vertex, the point keys more than one
/// vertex sits on (a pinch's cones, or touching lumps), and the
/// vertices the caller frees from that rule ([`free_vertices`]).
struct Pass<'f> {
    starts: BTreeMap<VertexKey, Vec<HalfEdgeKey>>,
    shared: BTreeSet<PointKey>,
    free: &'f BTreeSet<VertexKey>,
}

impl<'f> Pass<'f> {
    fn of<T: Real>(body: &Body<T>, free: &'f BTreeSet<VertexKey>) -> Self {
        let mut starts: BTreeMap<VertexKey, Vec<HalfEdgeKey>> = BTreeMap::new();
        for (k, h) in body.half_edges() {
            starts.entry(h.start).or_default().push(k);
        }
        let mut seen = BTreeSet::new();
        let mut shared = BTreeSet::new();
        for (_, v) in body.vertices() {
            if !seen.insert(v.point) {
                shared.insert(v.point);
            }
        }
        Self {
            starts,
            shared,
            free,
        }
    }
}

/// Of `recorded`, the vertices no other vertex of their own shell
/// shares a point key with: corners of a lump touching another lump,
/// not cones of one shell's pinch. The boolean's join frees them
/// ([`join_stage`]).
pub(super) fn free_vertices<T: Real>(
    body: &Body<T>,
    recorded: &BTreeSet<VertexKey>,
) -> BTreeSet<VertexKey> {
    // A vertex no edge leaves lies in no shell, and the join takes none.
    let mut at: BTreeMap<VertexKey, (crate::entity::ShellKey, PointKey)> = BTreeMap::new();
    for (w, v) in body.vertices() {
        let shell = v
            .emanating
            .and_then(|he| body.face_of_half_edge(he))
            .and_then(|f| body.get_face(f))
            .map(|f| f.shell);
        if let Some(shell) = shell {
            at.insert(w, (shell, v.point));
        }
    }
    let mut on: BTreeMap<(crate::entity::ShellKey, PointKey), usize> = BTreeMap::new();
    for &k in at.values() {
        *on.entry(k).or_default() += 1;
    }
    recorded
        .iter()
        .copied()
        .filter(|w| at.get(w).is_some_and(|k| on[k] == 1))
        .collect()
}

/// The structural carrier a curved edge's description names on a face
/// pair: a locus of the surface pair (transverse or tangent), an iso
/// family of one surface's chart, or, for a locus the pair
/// under-determines, an iso family of one of its two charts (`false`
/// the plus face's, `true` the minus face's).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Locus {
    Transverse,
    Tangent,
    Iso(geom_brep::IsoFamily),
    PairIso(bool, geom_brep::IsoFamily),
}

/// Whether `s`'s chart is singular at its point `p`: a cone's apex, a
/// sphere's poles, a torus's points on its axis — where the `u`
/// coordinate's lever arm (the distance from the apex or the axis)
/// collapses. A pole is never joinable, whichever arm the edges are
/// on: no chart image runs one branch through it.
pub(super) fn singular_at<T: Decide>(
    s: &geom::Surface<T>,
    p: geom_core::Point3<T>,
    band: Band,
) -> Result<bool, Indeterminate> {
    let off_axis = |origin: geom_core::Point3<T>, axis: geom_core::Vec3<T>| {
        geom::Line { origin, dir: axis }.distance(p)
    };
    let arm = match *s {
        geom::Surface::Cone { apex, .. } => (p - apex).norm(),
        geom::Surface::Sphere { center, axis, .. } => off_axis(center, axis),
        geom::Surface::Torus { center, axis, .. } | geom::Surface::Spindle { center, axis, .. } => {
            off_axis(center, axis)
        }
        _ => return Ok(false),
    };
    geom_core::k_stats::decide("join_regular_point", Margin::of(arm), band)
        .map(|sign| sign != Sign::Positive)
}

/// What `e`'s carrier holds at its end `w`, decided ([`crease_at`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Crease {
    /// The end's parameter is decided equal to one of the carrier's
    /// creases.
    Decided,
    /// Decided apart from every crease, or the carrier is no spline.
    No,
}

/// Whether `e`'s carrier holds a crease run at its end `w`: a spline
/// carrier whose parameter there is one point and is its own clamped
/// end, or an interior knot of multiplicity at least its degree. A
/// carrier run on through `w` would carry a knot of that multiplicity,
/// so `w` is the vertex the knot structure puts there, not one a join
/// takes back (module docs). The parameter's exact equality with one of
/// the carrier's own knots ([`crease_read`]).
///
/// # Errors
///
/// The undecided channel's escalation, where an enclosure of the
/// parameter straddles a crease and none is decided equal.
fn crease_at<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    d: &crate::entity::Edge,
    w: VertexKey,
) -> Result<Crease, Indeterminate> {
    let Some(c) = body.edge_curve_linked(e, d).certified() else {
        return Ok(Crease::No);
    };
    let geom::Curve3::Nurbs(n) = c.carrier() else {
        return Ok(Crease::No);
    };
    let (t0, t1) = c.params();
    let starts_at_w = body.get_half_edge(d.he_plus).map(|h| h.start) == Some(w);
    let t = if starts_at_w { t0 } else { t1 };
    let knots = n.knot_values();
    let (a, b) = knots.domain();
    crease_read(t, [a, b].into_iter().chain(knots.crease_values()))
}

/// **`t` against a carrier's creases** ([`crease_at`]): equal as two
/// parameters compared exactly read it ([`geom_brep::curve_pair::param_order`]),
/// where only a zero difference is Zero and only an enclosure straddling
/// it is undecided. A decided crease answers whatever the others read.
///
/// # Errors
///
/// The first straddle's escalation, where no crease is decided equal.
fn crease_read<T: Decide>(
    t: T,
    creases: impl IntoIterator<Item = T>,
) -> Result<Crease, Indeterminate> {
    let mut undecided = None;
    for k in creases {
        match geom_brep::curve_pair::param_order("edge_join_crease_at", t, k) {
            Ok(Sign::Zero) => return Ok(Crease::Decided),
            Ok(Sign::Positive | Sign::Negative) => {}
            Err(why) => {
                undecided.get_or_insert(why);
            }
        }
    }
    undecided.map_or(Ok(Crease::No), Err)
}

/// Which of a join's two edges is kept ([`kept_edge`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kept {
    /// The first edge is kept, the second goes.
    First,
    /// The second edge is kept, the first goes.
    Second,
}

/// **Which edge a join keeps, by the two edges' decided creases at the
/// vertex** (module docs): the edge whose carrier has no crease there,
/// the second where neither has one, and `None` where both have one, the
/// vertex the knot structure puts there. The two readings are decided:
/// an undecided one never reaches here, since it is the join's undecided
/// reading ([`JoinReading::Crease`]).
fn kept_edge(first: Crease, second: Crease) -> Option<Kept> {
    match (first, second) {
        (Crease::Decided, Crease::Decided) => None,
        (Crease::No, Crease::Decided) => Some(Kept::First),
        (Crease::Decided | Crease::No, Crease::No) => Some(Kept::Second),
    }
}

/// `w`'s join, if `w` is joinable (module docs). The half-edges in
/// `pass` were read out of the arena, so every hop past them is a link.
///
/// # Errors
///
/// [`JoinUndecided`] where a regularity reading at `w` lands in the
/// margin band.
fn joinable<T: Decide>(
    body: &Body<T>,
    w: VertexKey,
    pass: &Pass<'_>,
    band: Band,
) -> Result<Option<Join>, JoinUndecided> {
    let [h1, h2] = pass.starts.get(&w).map_or(&[][..], Vec::as_slice) else {
        return Ok(None);
    };
    let w_point = proven(&body.vertices, w, EntityId::Vertex).point;
    // A pinch's cone, or a touching lump's corner: the joined edge would
    // run through another vertex on this point (module docs).
    if pass.shared.contains(&w_point) && !pass.free.contains(&w) {
        return Ok(None);
    }
    let edge = |h: HalfEdgeKey| proven(&body.half_edges, h, EntityId::HalfEdge).edge;
    let (e1, e2) = (edge(*h1), edge(*h2));
    if e1 == e2 {
        return Ok(None);
    }
    let link = |e: EdgeKey, h: HalfEdgeKey| {
        linked(
            &body.edges,
            e,
            EntityId::Edge,
            EntityId::HalfEdge(h),
            "edge",
        )
    };
    // The joined edge runs on the kept carrier through `w` (module docs,
    // [`kept_edge`]). A crease the band leaves undecided is the join's
    // undecided reading, as every other one is.
    let crease = |e, h| {
        crease_at(body, e, link(e, h), w).map_err(|diag| JoinUndecided {
            vertex: w,
            reading: JoinReading::Crease(diag),
        })
    };
    let (h1, h2, e1, e2) = match kept_edge(crease(e1, *h1)?, crease(e2, *h2)?) {
        None => return Ok(None),
        Some(Kept::First) => (h2, h1, e2, e1),
        Some(Kept::Second) => (h1, h2, e1, e2),
    };
    let (d1, d2) = (link(e1, *h1), link(e2, *h2));
    let (s1, s2) = (edge_sides_of(body, e1, d1), edge_sides_of(body, e2, d2));
    let pair = |s: crate::readback::EdgeSides| {
        let (f, g) = s.faces();
        if f <= g { (f, g) } else { (g, f) }
    };
    let (f, g) = pair(s1);
    if f == g || pair(s2) != (f, g) {
        return Ok(None);
    }
    let Some(he) = body.mate(*h1) else {
        return Ok(None);
    };
    let far_of = |h: HalfEdgeKey| {
        let m = body.mate(h)?;
        Some(proven(&body.half_edges, m, EntityId::HalfEdge).start)
    };
    let (Some(far), Some(far2)) = (far_of(*h1), far_of(*h2)) else {
        return Ok(None);
    };
    let join = |planar, class| Join {
        he,
        gone: e1,
        kept: e2,
        far,
        closed: far == far2,
        planar,
        class,
    };
    let (sf, sg) = s1.surfaces();
    let surface = |side: crate::readback::EdgeSide| {
        let face = body.proven_face(side.face);
        face.surface()
    };
    let (surf_f, surf_g) = (surface(s1.plus), surface(s1.minus));
    let planar = |s: &geom::Surface<T>| matches!(s, geom::Surface::Plane { .. });
    if planar(surf_f) && planar(surf_g) {
        // One carrier by structure: both edges certified as the
        // intersection of these two planes, which is one line (planes
        // that do not cross certify no intersection).
        let on_the_pair = |e: EdgeKey, d| {
            certified_line(body, e, d).is_some_and(|c| {
                matches!(
                    c.description(),
                    geom_brep::EdgeDescription::Intersection { pair, .. }
                        if *pair == geom_brep::SurfacePair::new(sf, sg)
                )
            })
        };
        let joins = sf != sg && on_the_pair(e1, d1) && on_the_pair(e2, d2);
        return Ok(joins.then(|| join(true, None)));
    }
    let undecided = |reading| JoinUndecided { vertex: w, reading };
    let locus_of = |e: EdgeKey, d| {
        let Some(c) = body.edge_curve_linked(e, d).certified() else {
            return Ok(None);
        };
        Ok(match c.description() {
            geom_brep::EdgeDescription::Intersection { pair, .. }
                if sf != sg && *pair == geom_brep::SurfacePair::new(sf, sg) =>
            {
                Some((Locus::Transverse, None))
            }
            geom_brep::EdgeDescription::TangentIntersection { pair, .. }
                if sf != sg && *pair == geom_brep::SurfacePair::new(sf, sg) =>
            {
                Some((Locus::Tangent, None))
            }
            geom_brep::EdgeDescription::Chart(chart) if sf == sg && chart.surface == sf => {
                geom_brep::chart_iso_family(c.carrier(), c.params(), surf_f, band)
                    .map_err(|e| undecided(JoinReading::ChartClass(e)))?
                    .map(|read| (Locus::Iso(read.family), Some(read.zero)))
            }
            geom_brep::EdgeDescription::OnPair { pair }
                if sf != sg && *pair == geom_brep::SurfacePair::new(sf, sg) =>
            {
                // WHICH, not a search a later surface may answer past
                // ([`geom_brep::ladder::Answering::First`]'s rule, several rungs may answer):
                // which surface carries the iso (`minus`) is in the locus compared with the
                // other edge's below, so a class in band on the first surface may be the true
                // carrier, and its refusal stands.
                let mut found = None;
                for (minus, s) in [(false, surf_f), (true, surf_g)] {
                    if let Some(read) =
                        geom_brep::chart_iso_family(c.carrier(), c.params(), s, band)
                            .map_err(|e| undecided(JoinReading::ChartClass(e)))?
                    {
                        found = Some((Locus::PairIso(minus, read.family), Some(read.zero)));
                        break;
                    }
                }
                found
            }
            geom_brep::EdgeDescription::Intersection { .. }
            | geom_brep::EdgeDescription::TangentIntersection { .. }
            | geom_brep::EdgeDescription::OnPair { .. }
            | geom_brep::EdgeDescription::Chart(_)
            | geom_brep::EdgeDescription::Scaffold(_) => None,
        })
    };
    // Each iso reading's Zero rides beside its locus, which alone is
    // compared.
    let Some((locus, gone_zero)) = locus_of(e1, d1)? else {
        return Ok(None);
    };
    let Some((kept_locus, kept_zero)) = locus_of(e2, d2)? else {
        return Ok(None);
    };
    if kept_locus != locus {
        return Ok(None);
    }
    let regularity = |diag| undecided(JoinReading::Regularity(diag));
    let p = body.linked_vertex_point(w, EntityId::HalfEdge(*h1), "start");
    for s in [surf_f, surf_g] {
        if singular_at(s, p, band).map_err(regularity)? {
            return Ok(None);
        }
    }
    if matches!(locus, Locus::Iso(_) | Locus::PairIso(..)) {
        let zero = kept_zero.zip(gone_zero).map(|(k, g)| k.worst(g));
        return Ok(Some(join(false, zero)));
    }
    // The pair's reading at `w`, levered at the two faces' extent as
    // every reader of their wedge is (`crate::wedge_reach`), read from
    // both edges so the lever does not depend on which edge the arena
    // lists first.
    let reach = [e1, e2]
        .into_iter()
        .filter_map(|e| crate::edge_wedge_reach(body, e))
        .reduce(geom_core::Lever::max);
    let Some(reach) = reach else {
        return Ok(None);
    };
    let class = geom_brep::classify_dihedral(surf_f, surf_g, p, reach, band)
        .map_err(|e| regularity(e.diag()))?
        .class();
    let regular = match (locus, class) {
        (Locus::Transverse, geom_brep::DihedralClass::Transverse) => true,
        (Locus::Tangent, geom_brep::DihedralClass::Smooth) => {
            let Some(tangent) = tangent_at(body, e2, d2, w) else {
                return Ok(None);
            };
            let verdict =
                geom_brep::tangent_second_order(surf_f, surf_g, p, tangent, reach, band).verdict;
            verdict.map_err(regularity)?.sign == Sign::Positive
        }
        _ => false,
    };
    Ok(regular.then(|| join(false, None)))
}

/// `e`'s carrier tangent at its end `w`.
fn tangent_at<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    d: &crate::entity::Edge,
    w: VertexKey,
) -> Option<geom_core::Vec3<T>> {
    let c = body.edge_curve_linked(e, d).certified()?;
    let (t0, t1) = c.params();
    let start = body.get_half_edge(d.he_plus)?.start;
    Some(c.carrier().ders1(if start == w { t0 } else { t1 }).1)
}

/// Every joinable vertex of `body`, in vertex-arena order: the vertices
/// an op's output must not hold (maximal edges), read through the one
/// predicate the join takes them by.
///
/// # Errors
///
/// [`JoinUndecided`] for the first vertex, in arena order, whose
/// reading lands in `band`.
pub fn joinable_vertices<T: Decide>(
    body: &Body<T>,
    band: Band,
) -> Result<Vec<VertexKey>, JoinUndecided> {
    let none = BTreeSet::new();
    let pass = Pass::of(body, &none);
    let mut out = Vec::new();
    for (w, _) in body.vertices() {
        if joinable(body, w, &pass, band)?.is_some() {
            out.push(w);
        }
    }
    Ok(out)
}

/// **Tier 3's check 11, read** (`docs/DESIGN.md`, maximal edges; Ev's
/// PR 4251 ruling): every vertex of `body`, in vertex-arena order, that
/// the join's own predicate takes (`Ok`) or cannot decide in `band`
/// (`Err`). A finished body holds neither: every finisher ends with the
/// join, and a reading in the band is a size the run's tolerance cannot
/// tell from a regular point, which a tighter tolerance can.
pub(crate) fn joinable_at_rest<T: Decide>(
    body: &Body<T>,
    band: Band,
) -> Vec<Result<VertexKey, JoinUndecided>> {
    let none = BTreeSet::new();
    let pass = Pass::of(body, &none);
    body.vertices()
        .filter_map(|(w, _)| match joinable(body, w, &pass, band) {
            Ok(Some(_)) => Some(Ok(w)),
            Ok(None) => None,
            Err(undecided) => Some(Err(undecided)),
        })
        .collect()
}

/// Joins every joinable vertex of `body` ([`Body::join_edges`](crate::construction::Construct::join_edges)) and
/// writes each join's substitution rows into `desc` (`w → kept`,
/// `gone → kept`, and where the join leaves a conventional vertex, that
/// vertex `→ kept`), so the op's one [`super::ops::carry`] takes every
/// record through its zips, its merge and its joins together. Runs
/// after the merge and its re-description, before the records are
/// carried.
///
/// Also writes into `lineage`, for each `gone` edge that held a record
/// in `split_from` (the output stage's one snapshot, taken before the
/// merge), the edge it was split from
/// ([`super::BooleanNaming::killed_lineage`]).
///
/// Returns the joins, and the class Zero of each one made on an
/// iso-family reading ([`ClassZero`]), which the op records.
///
/// # Errors
///
/// As [`Body::join_edges`](crate::construction::Construct::join_edges).
pub(super) fn join_stage<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    desc: &mut Descendants,
    recorded: &BTreeSet<VertexKey>,
    split_from: &BTreeMap<EdgeKey, EdgeKey>,
    lineage: &mut BTreeMap<EdgeKey, EdgeKey>,
    band: Band,
    tol: Tol,
) -> Result<(Vec<EdgeJoin>, Vec<ClassZero>), BooleanError> {
    // The boolean's own surgery body, re-minted whole after the carry:
    // no staging and no re-mint here.
    let free = free_vertices(body, recorded);
    let (joined, _, zeros) = join_all(body, band, tol, &|_| true, &free)?;
    for j in &joined {
        desc.substitute((Cell::Vertex(j.vertex), Cell::Edge(j.kept)))?;
        desc.substitute((Cell::Edge(j.gone), Cell::Edge(j.kept)))?;
        if let Some(v) = j.conventional {
            desc.substitute((Cell::Vertex(v), Cell::Edge(j.kept)))?;
        }
    }
    record_killed_lineage(lineage, split_from, joined.iter().map(|j| j.gone));
    Ok((joined, zeros))
}

/// Each edge of `body` with a `SplitEdge` record → the edge it was
/// split from: the records as they stand, taken before a kill (a merge's
/// or a join's) drops the killed edges' ([`record_killed_lineage`]).
pub(crate) fn split_records<T: Real>(body: &Body<T>) -> BTreeMap<EdgeKey, EdgeKey> {
    body.edges()
        .filter_map(|(e, _)| match body.edge_provenance_of(e) {
            Some(crate::provenance::Provenance::SplitEdge { edge }) => Some((e, *edge)),
            _ => None,
        })
        .collect()
}

/// Writes into `lineage` each of the `killed` edges that held a record
/// in `split_from` ([`split_records`], taken before the kills), with the
/// edge it was split from. An edge recorded twice names one parent: a
/// body holds one record per edge.
pub(crate) fn record_killed_lineage(
    lineage: &mut BTreeMap<EdgeKey, EdgeKey>,
    split_from: &BTreeMap<EdgeKey, EdgeKey>,
    killed: impl IntoIterator<Item = EdgeKey>,
) {
    for gone in killed {
        if let Some(&from) = split_from.get(&gone) {
            let earlier = lineage.insert(gone, from);
            debug_assert!(
                earlier.is_none_or(|e| e == from),
                "{gone:?} recorded with two parents ({earlier:?}, {from:?})"
            );
        }
    }
}

/// **Why the join refused**, as a door that is not the boolean carries
/// it ([`crate::MergeCoplanarError::Join`],
/// [`crate::ReplaceFaceError::Join`]): typed, keyless, and worded with
/// one recourse.
#[derive(Clone, Debug, PartialEq)]
pub enum JoinRefusal {
    /// A regularity or chart-class reading at a vertex landed in the
    /// band ([`BooleanError::JoinUndecided`]).
    Undecided(JoinUndecided),
    /// The joined edge's carrier has no period (a closed join) or no
    /// parameter inverse (a spline) to run it on
    /// ([`BooleanError::JoinCarrierUnsupported`]).
    CarrierUnsupported {
        /// The carrier's kind.
        carrier: geom::CurveKind,
        /// Whether the join closes.
        closed: bool,
    },
    /// The kill, the re-description or the re-mint refused on a body
    /// that passed tier 2: every other [`BooleanError`] the join
    /// raises ([`JoinRefusal::of`]). That mixes causes: Euler
    /// certification's limit, which user geometry reaches at a build;
    /// the pair rule's refusal over the joined span (a seam's lever
    /// rung, its undecided jet, a mixed seam); and the bug-only
    /// [`BooleanError::JoinDesync`]. A build ends in the last resort
    /// ([`geom_core::KERNEL_LIMIT_RECOURSE`]), the one ending honest
    /// across that mix; read over a body at rest or a file, it is a
    /// stored contradiction and ends in the door's defect ending.
    Kernel {
        /// The boolean refusal's kind.
        kind: super::BooleanErrorKind,
    },
}

/// A refusal kind as its variant's name.
fn kind_word(kind: super::BooleanErrorKind) -> String {
    format!("{kind:?}")
}

impl JoinRefusal {
    /// The join door's refusal, typed for a door that is not the
    /// boolean.
    #[must_use]
    pub fn of(refusal: &BooleanError) -> Self {
        match refusal {
            BooleanError::JoinUndecided(e) => Self::Undecided(e.clone()),
            BooleanError::JoinCarrierUnsupported {
                carrier, closed, ..
            } => Self::CarrierUnsupported {
                carrier: *carrier,
                closed: *closed,
            },
            other => Self::Kernel { kind: other.kind() },
        }
    }
}

impl JoinRefusal {
    /// This refusal's text read at `at`: a reading in band ends in the
    /// join's lever, a poisoned one with the door's unreadable-margin
    /// note ([`geom_brep::recourse::unreadable_margin_note`]), a refused
    /// kill or re-mint as [`Self::Kernel`] says, and a defect in the
    /// door's defect ending ([`geom_brep::recourse::defect_ending`]).
    /// `None` writes the payload alone, its `Display`.
    fn write(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        at: Option<geom_brep::recourse::ReadAt>,
    ) -> core::fmt::Result {
        use geom_brep::recourse::{defect_ending, unreadable_margin_note};
        const IMAGE: &str = "two edges meeting at a vertex on one surface read their family off a chart image \
             that did not derive";
        match self {
            Self::Undecided(JoinUndecided {
                reading: JoinReading::Regularity(diag) | JoinReading::Crease(diag),
                ..
            })
            | Self::Undecided(JoinUndecided {
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Undecided(diag)),
                ..
            }) => {
                write!(f, "{JOIN_SUBJECT} is undecided ({})", diag.payload())?;
                match at {
                    None => Ok(()),
                    Some(at) if at.is_build() => write!(f, ". {}", diag.ending(JOIN_LEVER)),
                    Some(at) => write!(
                        f,
                        ". {}",
                        diag.ending_noted(JOIN_LEVER, unreadable_margin_note(at))
                    ),
                }
            }
            Self::Undecided(JoinUndecided {
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(e)),
                ..
            }) => {
                // The image's refusal ends by its own ending at this door:
                // an incidence read by the one map (D3), a lane gap "not
                // yet", an escalation once, as its decision ends it.
                write!(f, "{IMAGE}: {}", geom_brep::recourse::read_at(e, at))
            }
            Self::CarrierUnsupported { carrier, closed } => write!(
                f,
                "two edges meet at a vertex on one {} carrier, which the join cannot run an \
                 edge on {} beside the other edge's carrier yet \
                 (work/splice/a-spline-join-beside-another-carrier-has-no-far-parameter); there \
                 is no way through this yet",
                carrier.name(),
                if *closed {
                    "over a whole period"
                } else {
                    "through the vertex"
                }
            ),
            Self::Kernel { kind } => {
                write!(
                    f,
                    "the join of two edges meeting at a vertex on one curve refused ({})",
                    kind_word(*kind)
                )?;
                match at {
                    None => Ok(()),
                    Some(at) if at.is_build() => {
                        write!(f, ". {}", geom_core::KERNEL_LIMIT_RECOURSE)
                    }
                    Some(at) => write!(f, ". {}", defect_ending(at)),
                }
            }
        }
    }
}

geom_brep::ended_by_write!(JoinRefusal);

impl<T: Decide + crate::props::AtRestPolicy> Body<T> {
    /// The door [`Construct::join_edges`](crate::construction::Construct::join_edges) forwards to;
    /// documented there.
    pub(crate) fn join_edges(
        &mut self,
        band: Band,
        tol: Tol,
    ) -> Result<Vec<EdgeJoin>, BooleanError> {
        // Nothing to join: no clone. Every reading in the band refuses
        // here, before any kill.
        if joinable_vertices(self, band)
            .map_err(BooleanError::JoinUndecided)?
            .is_empty()
        {
            return Ok(Vec::new());
        }
        // Staged: a refusal past the first kill leaves `self` as found.
        let mut work = self.clone();
        let mut staged = work.begin_surgery();
        let (joined, _, _) = join_all(&mut staged, band, tol, &|_| true, &BTreeSet::new())?;
        // A kill leaves the rows keyed by its half-edges; a body that
        // carried rows has them re-derived whole (`pcurves::mint_pcurves`
        // clears the map first), so none is left keyed by a dead cell.
        if !staged.pcurves.is_empty() || !staged.joints.is_empty() {
            crate::pcurves::mint_pcurves(&mut staged, tol)
                .map_err(|source| BooleanError::Pcurves { source })?;
        }
        staged.sweep_and_close();
        self.adopt(work);
        Ok(joined)
    }

    /// The door [`Construct::join_edges_within`](crate::construction::Construct::join_edges_within) forwards to;
    /// documented there.
    pub(crate) fn join_edges_within(
        &mut self,
        band: Band,
        tol: Tol,
        within: &dyn Fn(VertexKey) -> bool,
    ) -> Result<Vec<EdgeJoin>, BooleanError> {
        let mut body = self.begin_surgery();
        let (joined, dead, _) = join_all(&mut body, band, tol, within, &BTreeSet::new())?;
        if joined.is_empty() {
            // No kill ran: the body is as the caller handed it, so this
            // call has no postcondition to sweep.
            return Ok(joined);
        }
        if !(body.pcurves.is_empty() && body.joints.is_empty()) {
            body.drop_rows(dead);
            let mut faces = BTreeSet::new();
            for j in &joined {
                if let Some(e) = body.get_edge(j.kept) {
                    faces.extend(body.face_of_half_edge(e.he_plus));
                    faces.extend(body.face_of_half_edge(e.he_minus));
                }
            }
            let faces: Vec<_> = faces.into_iter().collect();
            crate::pcurves::mint_pcurves_of(&mut body, &faces, tol)
                .map_err(|source| BooleanError::Pcurves { source })?;
        }
        body.sweep_and_close();
        Ok(joined)
    }
}

/// What [`join_all`] made: the joins in order, the half-edges they
/// killed, and the class Zero of each join that acted on one.
type Joined = (Vec<EdgeJoin>, Vec<HalfEdgeKey>, Vec<ClassZero>);

/// [`Body::join_edges`](crate::construction::Construct::join_edges)' loop, on the staged body.
fn join_all<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    band: Band,
    tol: Tol,
    within: &dyn Fn(VertexKey) -> bool,
    free: &BTreeSet<VertexKey>,
) -> Result<Joined, BooleanError> {
    let mut joined = Vec::new();
    let mut dead = Vec::new();
    let mut zeros = Vec::new();
    loop {
        let pass = Pass::of(body, free);
        let mut next = None;
        for (w, _) in body.vertices().filter(|&(w, _)| within(w)) {
            if let Some(j) = joinable(body, w, &pass, band).map_err(BooleanError::JoinUndecided)? {
                next = Some((w, j));
                break;
            }
        }
        let Some((w, join)) = next else {
            return Ok((joined, dead, zeros));
        };
        if let Some(margin) = join.class {
            zeros.push(ClassZero {
                gone: join.gone,
                kept: join.kept,
                margin,
            });
        }
        if let Some(gone) = body.get_edge(join.gone) {
            dead.extend([gone.he_plus, gone.he_minus]);
        }
        join_one(body, w, &join, band, tol)?;
        // A closed join's survivor is conventional unless another edge
        // still ends there (a seam strut on the rim it closed).
        let conventional =
            (join.closed && is_conventional_vertex(body, join.far)).then_some(join.far);
        joined.push(EdgeJoin {
            vertex: w,
            gone: join.gone,
            kept: join.kept,
            conventional,
        });
    }
}

/// **`gone`'s far parameter, carried**: where `gone` is `kept`'s carrier
/// by content ([`geom::Curve3::same_content`]) and stands at `kept`'s
/// parameter `tw` at the vertex, its interval is one of that carrier with
/// `tw` at one end, so its far end `gx` is the joined edge's far
/// parameter. `None` otherwise, or where `gx` does not lie past `tw` away
/// from `kept`. Exact, since a split restates its parent's carrier
/// verbatim on both children; at a scalar whose values are not plain
/// constants (an interval wider than a point, a derivative, a symbol)
/// nothing carries.
fn carried_far<T: Decide>(
    carrier: &geom::Curve3<T>,
    tw: T,
    backward: bool,
    gone: &geom::Curve3<T>,
    (gw, gx): (T, T),
) -> Option<T> {
    let at_tw = matches!(
        (gw.plain_constant(), tw.plain_constant()),
        (Some(x), Some(y)) if x == y
    );
    if !(at_tw && carrier.same_content(gone)) {
        return None;
    }
    let past = if backward { tw - gx } else { gx - tw };
    let pick = geom_core::k_stats::choose("edge_join_carried_far", Margin::of(past), "F39");
    matches!(pick, geom_core::k_stats::Pick::Positive).then_some(gx)
}

/// **`gone`'s far end read off `kept`'s carrier**: the parameter of
/// `gone`'s middle near `tw`, then of `far` near that, so the reading
/// walks from the vertex along `gone`. Where `gone` lies on the very
/// carrier ([`carried_far`]), this names `gx`'s point, and the two
/// spellings agree up to rounding (ledger row F39).
fn read_far<T: Decide>(
    carrier: &geom::Curve3<T>,
    gone: &geom::Curve3<T>,
    (g0, g1): (T, T),
    far: geom_core::Point3<T>,
    tw: T,
) -> Option<T> {
    carrier
        .param_near(gone.mid_point(g0, g1), tw)
        .and_then(|tm| carrier.param_near(far, tm))
}

/// The joined edge's spec: `kept`'s carrier over its interval run on
/// through `w` to `gone`'s far end, its description restated over the
/// new interval. The far end's parameter is `gone`'s own where `gone`
/// lies on `kept`'s very carrier ([`carried_far`]); otherwise it is read
/// off the carrier: a whole period where the join closes, the point's
/// parameter ([`geom::Curve3::param_near`]) where it does not.
fn joined_spec<T: Decide>(
    body: &Body<T>,
    w: VertexKey,
    join: &Join,
    band: Band,
) -> Result<geom_brep::EdgeCurveSpec<T>, BooleanError> {
    let desync = |what| BooleanError::JoinDesync { what };
    let curve = |e: EdgeKey| {
        let d = body.get_edge(e)?;
        Some((body.edge_curve_linked(e, d).certified()?, d))
    };
    let ((kept, kd), (gone, gd)) = curve(join.kept)
        .zip(curve(join.gone))
        .ok_or(desync("a joined edge is not certified"))?;
    let carrier = kept.carrier();
    let (t0, t1) = kept.params();
    let backward = body.get_half_edge(kd.he_plus).map(|h| h.start) == Some(w);
    let (g0, g1) = gone.params();
    let far = *body
        .get_vertex(join.far)
        .and_then(|v| body.get_point(v.point))
        .ok_or(desync("a joined edge's far end does not resolve"))?;
    let tw = if backward { t0 } else { t1 };
    let gone_from_w = body.get_half_edge(gd.he_plus).map(|h| h.start) == Some(w);
    let at_w = if gone_from_w { (g0, g1) } else { (g1, g0) };
    let carried = carried_far(carrier, tw, backward, gone.carrier(), at_w);
    let tx = if let Some(tx) = carried {
        tx
    } else if join.closed {
        let period = T::from_f64(core::f64::consts::TAU);
        match carrier {
            geom::Curve3::Circle(..) | geom::Curve3::Ellipse(..) | geom::Curve3::Spiric { .. } => {
                if backward {
                    t1 - period
                } else {
                    t0 + period
                }
            }
            _ => {
                return Err(BooleanError::JoinCarrierUnsupported {
                    edge: join.kept,
                    carrier: carrier.kind(),
                    closed: true,
                });
            }
        }
    } else {
        read_far(carrier, gone.carrier(), (g0, g1), far, tw).ok_or(
            BooleanError::JoinCarrierUnsupported {
                edge: join.kept,
                carrier: carrier.kind(),
                closed: false,
            },
        )?
    };
    let (a, b) = if backward { (tx, t1) } else { (t0, tx) };
    // A declaration's parameter is affine in the carrier's, so the kept
    // edge's runs on over the joined span exactly as a split restricts
    // it; certification re-meters it.
    let declared = kept.authority().declared().map(|mc| {
        let span = t1 - t0;
        mc.restrict((a - t0) / span, (b - t0) / span)
    });
    // The join's refusal of a pair or an image the rule refuses over
    // the joined span.
    let refused = |refusal| match refusal {
        geom_brep::PairRefusal::InBand(escalation) => {
            let (reading, diag) = geom_brep::DihedralReading::of_must_carry(escalation);
            super::ops::seam_refusal(reading, diag)
        }
        geom_brep::PairRefusal::Mixed(stations) => BooleanError::MixedSeam {
            edge: join.kept,
            stations: Some(stations),
        },
    };
    let description = match kept.description() {
        geom_brep::EdgeDescription::Intersection { pair, selector } => {
            geom_brep::EdgeDescriptionSpec::Intersection {
                pair: *pair,
                selector: selector.repinned(carrier, a, b),
            }
        }
        geom_brep::EdgeDescription::TangentIntersection { pair, .. } => {
            geom_brep::EdgeDescriptionSpec::TangentIntersection {
                pair: *pair,
                witness: carrier.mid_point(a, b),
            }
        }
        // The joined edge is longer than either piece, and the
        // must-carry verdict reads its extent, so the pair is asked again
        // over the joined span.
        geom_brep::EdgeDescription::OnPair { pair } => body
            .reask_on(
                *pair,
                crate::edge_faces(body, join.kept),
                carrier,
                (a, b),
                declared,
                band,
            )
            .map_err(refused)?,
        // An image is asked the same, between the joined edge's faces,
        // as every door that keeps one is. The join takes two images only
        // where both faces are on one surface key ([`joinable`]'s iso
        // family of one chart), which the rule leaves alone, so no input
        // reaches its demand here.
        geom_brep::EdgeDescription::Chart(c) => {
            let demanded = if c.wrap {
                None
            } else {
                body.reask_chart_on(join.kept, carrier, (a, b), band)
                    .map_err(refused)?
            };
            demanded.unwrap_or_else(|| geom_brep::EdgeDescriptionSpec::Chart {
                surface: c.surface,
                image: if c.wrap { None } else { Some(c.pcurve.clone()) },
                wrap: c.wrap,
                declared,
            })
        }
        geom_brep::EdgeDescription::Scaffold(_) => {
            return Err(desync("a joined edge is scaffolding at rest"));
        }
    };
    Ok(geom_brep::EdgeCurveSpec {
        description,
        carrier: carrier.clone(),
        param_start: a,
        param_end: b,
    })
}

/// Joins at `w` (module docs): `gone` dies with `w` through `kev`, and
/// `kept`, re-based onto `gone`'s far end, is restated over both edges'
/// span: a planar join as the chord its faces then describe, a curved
/// one on its own carrier ([`joined_spec`]).
fn join_one<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    w: VertexKey,
    join: &Join,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let spec = if join.planar {
        let members = body.kev_merged_members(join.he)?;
        members
            .iter()
            .find(|m| m.edge == join.kept)
            .map(|m| crate::EdgeCurveSpec::line_between(m.start, m.end))
            .ok_or(BooleanError::JoinDesync {
                what: "a joined edge is not the kill's merged member",
            })?
    } else {
        joined_spec(body, w, join, band)?
    };
    let killed = body.kev_describing(join.he, &[(join.kept, spec)], tol)?;
    debug_assert_eq!(killed.killed_vertex, w, "the kill takes the joined vertex");
    if !join.planar {
        return Ok(());
    }
    describe_minted_edges(
        body,
        &[join.kept],
        &crate::merge_faces::MergeCoplanarOutcome::default(),
        band,
        tol,
    )
}

/// **An undecided crease is the join's undecided reading**
/// (orchestrator's revised ruling on PR 6709's review): an enclosure of
/// an edge's end parameter straddling a crease of its spline carrier
/// refuses the join through [`JoinUndecided`], the channel every
/// undecided join reading takes, so tier 3's check 11 sees it too. Only
/// a decided pair picks the kept edge. Red where a straddle is read as
/// "no crease" (the reading before PR 6709), as "a crease" (no join, a
/// silent guess either way), or where the undecided channel is dropped
/// at `joinable`.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod crease {
    use geom_core::{Band, Interval, Point2, Point3, Tol, Vec3};

    use super::{Crease, JoinReading, JoinUndecided, Kept, Pass, crease_read, joinable, kept_edge};
    use crate::body::Body;
    use crate::euler::{FaceSurface, MevSite};
    use crate::test_support_fixtures::fresh_keyed_body;

    #[test]
    fn a_crease_is_read_decided_or_its_straddle_escalates() {
        let knots = [0.0, 0.5, 1.0];
        let at = |lo: f64, hi: f64| Interval::from_bounds(lo, hi);
        let creases = || knots.map(|k| at(k, k));
        assert_eq!(crease_read(at(0.5, 0.5), creases()), Ok(Crease::Decided));
        assert_eq!(crease_read(at(0.25, 0.26), creases()), Ok(Crease::No));
        assert!(crease_read(at(0.49, 0.51), creases()).is_err());
        assert_eq!(crease_read(0.5, knots), Ok(Crease::Decided));
        assert_eq!(crease_read(0.5 + 1e-15, knots), Ok(Crease::No));
        assert_eq!(kept_edge(Crease::Decided, Crease::Decided), None);
        assert_eq!(kept_edge(Crease::No, Crease::Decided), Some(Kept::First));
        assert_eq!(kept_edge(Crease::Decided, Crease::No), Some(Kept::Second));
        assert_eq!(kept_edge(Crease::No, Crease::No), Some(Kept::Second));
    }

    /// A planar sheet's wire edge on a degree-2 spline with a crease at
    /// `0.5` (a double interior knot), split at `t`.
    fn split_spline<T: geom_core::Decide + crate::props::AtRestPolicy>(
        t: T,
    ) -> (Body<T>, crate::VertexKey) {
        let tol = Tol::witness();
        let c = T::from_f64;
        let kv =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2)
                .unwrap();
        let net = [(0.0, 0.0), (0.5, 0.6), (1.0, 0.0), (1.5, 0.6), (2.0, 0.0)];
        let knots = || geom_core::spline::Knots::from(kv.clone()).lift(c);
        let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                knots(),
                net.iter()
                    .map(|&(x, y)| Point3::new(c(x), c(y), c(0.0)))
                    .collect(),
                vec![c(1.0); 5],
            )
            .unwrap(),
        ));
        let image = geom_brep::Pcurve::Fitted(std::sync::Arc::new(
            geom::NurbsCurve2::new(
                knots(),
                net.iter().map(|&(x, y)| Point2::new(c(x), c(y))).collect(),
                vec![c(1.0); 5],
            )
            .unwrap(),
        ));
        let mut body = fresh_keyed_body::<T>();
        let p0 = Point3::new(c(0.0), c(0.0), c(0.0));
        let seed = body.mvfs(p0, true).unwrap();
        let plane = body
            .set_face_surface(
                seed.face,
                FaceSurface::New {
                    surface: geom::Surface::Plane {
                        origin: p0,
                        normal: Vec3::new(c(0.0), c(0.0), c(1.0)),
                        u_ref: Vec3::new(c(1.0), c(0.0), c(0.0)),
                    },
                    sense: true,
                },
            )
            .unwrap();
        let e = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                Point3::new(c(2.0), c(0.0), c(0.0)),
                geom_brep::EdgeCurveSpec {
                    description: geom_brep::EdgeDescriptionSpec::Chart {
                        surface: plane,
                        image: Some(image),
                        wrap: false,
                        declared: None,
                    },
                    carrier,
                    param_start: c(0.0),
                    param_end: c(1.0),
                },
                tol,
            )
            .unwrap();
        let made = body.split_edge(e.edge, t, tol).unwrap();
        (body, made.vertex)
    }

    fn reading<T: geom_core::Decide + crate::props::AtRestPolicy>(
        t: T,
    ) -> Result<bool, JoinUndecided> {
        let (body, w) = split_spline(t);
        let none = std::collections::BTreeSet::new();
        let pass = Pass::of(&body, &none);
        let band = Band::linear(Tol::witness()).unwrap();
        joinable(&body, w, &pass, band).map(|join| join.is_some())
    }

    #[test]
    fn a_split_straddling_a_crease_refuses_the_join_undecided() {
        // On the crease, decided: both pieces crease there, no join.
        assert!(!reading(0.5).unwrap(), "f64 on the crease");
        assert!(
            !reading(Interval::from_bounds(0.5, 0.5)).unwrap(),
            "a point interval on the crease"
        );
        // Straddling it: the join's undecided reading, not a guess.
        // Narrow against the band, so the split's point certifies: the
        // straddle is in the parameter, not in the point's place.
        let half = 1e-4 * Band::linear(Tol::witness()).unwrap().zero();
        let got = reading(Interval::from_bounds(0.5 - half, 0.5 + half));
        assert!(
            matches!(
                got,
                Err(JoinUndecided {
                    reading: JoinReading::Crease(_),
                    ..
                })
            ),
            "{got:?}"
        );
    }
}

#[cfg(test)]
mod carry {
    //! **The carry rung** ([`carried_far`]): on one carrier of each kind,
    //! a piece standing at the kept edge's parameter at the vertex hands
    //! over its own far parameter, on either side; a twin one ulp off in
    //! one value, a piece standing at another parameter, or a far end on
    //! the kept edge's own side carries nothing. That every stated value
    //! is read is `geom`'s row (`curves/same_content.rs`); the two rows
    //! after the first pin the rung on the values a parametrization hangs
    //! on, a net's knots and a circle's reference direction.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::sync::Arc;

    use geom::Curve3;
    use geom_core::{Point3, Vec3};

    use super::{carried_far, read_far};

    /// Each kind's carrier, and its twin with one stated value one ulp
    /// off.
    fn carriers() -> Vec<(&'static str, Curve3<f64>, Curve3<f64>)> {
        let up = f64::next_up;
        let o = Point3::new(0.25, -0.5, 1.0);
        let (z, x) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let frame = geom::Frame::new(o, z, x);
        let net = |w: f64| {
            let kv =
                geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2)
                    .unwrap();
            let pts = vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(2.0, 0.0, 0.0),
                Point3::new(3.0, 1.0, 0.0),
            ];
            Curve3::Nurbs(Arc::new(
                geom::NurbsCurve3::new(kv, pts, vec![1.0, w, 1.0, 1.0]).unwrap(),
            ))
        };
        let spiric = |offset: f64| Curve3::Spiric {
            center: o,
            axis: z,
            u_ref: x,
            major_radius: 2.0,
            minor_radius: 0.5,
            offset,
        };
        vec![
            (
                "line",
                Curve3::Line(geom::Line { origin: o, dir: x }),
                Curve3::Line(geom::Line {
                    origin: Point3::new(o.x, up(o.y), o.z),
                    dir: x,
                }),
            ),
            (
                "circle",
                Curve3::Circle(geom::Circle { frame, radius: 1.5 }),
                Curve3::Circle(geom::Circle {
                    frame,
                    radius: up(1.5),
                }),
            ),
            (
                "ellipse",
                Curve3::Ellipse(geom::Ellipse {
                    frame,
                    semi_u: 2.0,
                    semi_v: 1.0,
                }),
                Curve3::Ellipse(geom::Ellipse {
                    frame,
                    semi_u: 2.0,
                    semi_v: up(1.0),
                }),
            ),
            ("spiric", spiric(0.75), spiric(up(0.75))),
            ("nurbs", net(0.8), net(up(0.8))),
        ]
    }

    #[test]
    fn a_piece_carries_on_its_own_carrier_and_not_on_a_twin_or_past_the_vertex() {
        for (kind, kept, twin) in carriers() {
            // Kept over [0.2, 0.4], forward to w at 0.4; gone [0.4, 0.7].
            assert_eq!(
                carried_far(&kept, 0.4, false, &kept.clone(), (0.4, 0.7)),
                Some(0.7),
                "{kind}: forward, carried"
            );
            // Kept over [0.4, 0.7], w at its start; gone [0.2, 0.4].
            assert_eq!(
                carried_far(&kept, 0.4, true, &kept.clone(), (0.4, 0.2)),
                Some(0.2),
                "{kind}: backward, carried"
            );
            assert_eq!(
                carried_far(&kept, 0.4, false, &twin, (0.4, 0.7)),
                None,
                "{kind}: a carrier one value off carries nothing"
            );
            assert_eq!(
                carried_far(&kept, 0.4, false, &kept.clone(), (f64::next_up(0.4), 0.7)),
                None,
                "{kind}: a piece at another parameter at the vertex carries nothing"
            );
            assert_eq!(
                carried_far(&kept, 0.4, false, &kept.clone(), (0.4, 0.3)),
                None,
                "{kind}: a far end on the kept edge's side carries nothing"
            );
        }
    }

    /// **The carry and the reading off the carrier name one parameter**
    /// (ledger row F39, the premise `carried_far` is read unlogged on):
    /// where `gone` lies on `kept`'s very carrier, the parameter
    /// [`read_far`] reads `gone`'s far point at is `gx` up to rounding,
    /// on either side, wherever it reads one. A carrier it reads none on
    /// is listed, and the carry is then the only spelling.
    #[test]
    fn the_carried_far_parameter_is_the_one_the_carrier_reads() {
        let mut unread = Vec::new();
        for (kind, kept, _) in carriers() {
            for (tw, (g0, g1)) in [(0.4, (0.4, 0.7)), (0.4, (0.4, 0.2)), (1.1, (1.1, 2.9))] {
                let backward = g1 < g0;
                let Some(gx) = carried_far(&kept, tw, backward, &kept, (g0, g1)) else {
                    panic!("{kind}: [{g0}, {g1}] on the kept carrier carries");
                };
                let far = kept.eval(gx);
                match read_far(&kept, &kept, (g0, g1), far, tw) {
                    Some(read) => assert!(
                        (read - gx).abs() <= 1e-12 * (1.0 + gx.abs()),
                        "{kind}: from {tw} to {gx}, the carrier reads {read}"
                    ),
                    None => unread.push(kind),
                }
            }
        }
        unread.dedup();
        assert_eq!(
            unread,
            ["nurbs"],
            "the carriers no reading off the carrier serves"
        );
    }

    fn net(knot: f64) -> Curve3<f64> {
        let kv =
            geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, knot, 1.0, 1.0, 1.0], 2)
                .unwrap();
        let pts = vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(2.0, 0.0, 0.0),
            Point3::new(3.0, 1.0, 0.0),
        ];
        Curve3::Nurbs(Arc::new(
            geom::NurbsCurve3::new(kv, pts, vec![1.0, 0.8, 1.0, 1.0]).unwrap(),
        ))
    }

    #[test]
    fn a_piece_of_a_net_with_its_interior_knot_moved_carries_nothing() {
        let (kept, twin) = (net(0.5), net(f64::next_up(0.5)));
        assert_eq!(
            carried_far(&kept, 0.4, false, &kept.clone(), (0.4, 0.7)),
            Some(0.7),
            "the net itself carries"
        );
        assert_eq!(
            carried_far(&kept, 0.4, false, &twin, (0.4, 0.7)),
            None,
            "one control polygon and weight row on other knots is another curve"
        );
    }

    #[test]
    fn a_piece_of_a_circle_turned_in_its_reference_direction_carries_nothing() {
        let o = Point3::new(0.0, 0.0, 0.0);
        let z = Vec3::new(0.0, 0.0, 1.0);
        let circle = |u: Vec3<f64>| {
            Curve3::Circle(geom::Circle {
                frame: geom::Frame::new(o, z, u),
                radius: 1.0,
            })
        };
        let (kept, turned) = (
            circle(Vec3::new(1.0, 0.0, 0.0)),
            circle(Vec3::new(0.0, 1.0, 0.0)),
        );
        assert_eq!(
            carried_far(&kept, 0.4, false, &kept.clone(), (0.4, 0.7)),
            Some(0.7),
            "the circle itself carries"
        );
        assert_eq!(
            carried_far(&kept, 0.4, false, &turned, (0.4, 0.7)),
            None,
            "one locus from another reference direction is another parametrization"
        );
    }
}

#[cfg(test)]
mod conventional {
    //! **[`is_conventional_vertex`] is structural**: a vertex whose
    //! only edge is one closed edge, at both its ends. A self-loop's
    //! lone vertex is conventional; an open edge's end is not, and nor
    //! is a self-loop's vertex once another edge leaves it (a seam
    //! strut on the rim it closes, as `pi_seam`'s puck keeps).
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Point3, Tol};

    use super::is_conventional_vertex;
    use crate::Body;
    use crate::construction::{MefSite, MevSite};

    #[test]
    fn only_a_lone_self_loops_vertex_is_conventional() {
        let tol = Tol::witness();
        let mut body = Body::<f64>::new();
        let born = body.mvfs(Point3::new(0.0, 0.0, 0.0), true).unwrap();
        assert!(!is_conventional_vertex(&body, born.vertex), "a lone vertex");
        let closed = body
            .mef_chord(
                MefSite::Lone {
                    r#loop: born.r#loop,
                },
                tol,
            )
            .unwrap();
        assert!(is_conventional_vertex(&body, born.vertex), "a self-loop's");
        let he = body.get_edge(closed.edge).unwrap().he_plus;
        let strut = body
            .mev_line(
                MevSite::Fan { he1: he, he2: he },
                Point3::new(0.0, 0.0, 1.0),
                tol,
            )
            .unwrap();
        // Whichever half-edge the vertex records as its first, the
        // strut leaving it is read: the closed edge's two halves, and
        // the strut's own.
        let e = body.get_edge(closed.edge).unwrap();
        let s = body.get_edge(strut.edge).unwrap();
        let out = [s.he_plus, s.he_minus]
            .into_iter()
            .find(|&h| body.get_half_edge(h).unwrap().start == born.vertex)
            .unwrap();
        let firsts = [e.he_plus, e.he_minus, out];
        for first in firsts {
            body.vertices.get_mut(born.vertex).unwrap().emanating = Some(first);
            assert!(
                !is_conventional_vertex(&body, born.vertex),
                "a self-loop's vertex another edge leaves, first {first:?}"
            );
        }
        let (_, far) = crate::query::edge_ends(&body, strut.edge);
        assert!(!is_conventional_vertex(&body, far), "an open edge's end");
    }
}

#[cfg(test)]
mod join_door {
    //! **The join door and the doors that end with it**: a brick whose
    //! edge was split at its middle holds one joinable vertex; the join
    //! door takes it back to the brick, and the public merge door and
    //! an offset door, ending with the join, leave none either.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Band, Tol};

    use super::joinable_vertices;
    use crate::body::Body;
    use crate::split::SplitEdgeCreated;
    use crate::test_support_fixtures::brick;

    /// A unit brick with one edge split at its middle, the split's
    /// record, and the brick's own arena counts.
    fn split_brick() -> (
        Body<f64>,
        SplitEdgeCreated,
        crate::test_support::ArenaCounts,
    ) {
        let tol = Tol::witness();
        let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let whole = body.arena_counts();
        let (e, d) = body.edges().next().unwrap();
        let (t0, t1) = body
            .get_curve_geom(d.curve)
            .and_then(crate::CurveGeom::certified)
            .unwrap()
            .params();
        let made = body.split_edge(e, 0.5 * (t0 + t1), tol).unwrap();
        (body, made, whole)
    }

    #[test]
    fn the_join_door_takes_a_split_edge_back() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (mut body, made, whole) = split_brick();
        assert_eq!(joinable_vertices(&body, band).unwrap(), vec![made.vertex]);
        let joins = body.join_edges(band, tol).unwrap();
        assert_eq!(joins.len(), 1);
        assert_eq!(joins[0].vertex, made.vertex);
        assert_eq!(joins[0].conventional, None);
        assert_eq!(body.arena_counts(), whole, "the brick again");
        assert!(joinable_vertices(&body, band).unwrap().is_empty());
        crate::validate_geometric(&body, tol).unwrap();
        assert!(body.join_edges(band, tol).unwrap().is_empty(), "idempotent");
    }

    #[test]
    fn the_merge_door_ends_with_the_join_and_reports_it() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (mut body, made, whole) = split_brick();
        let outcome = body.merge_coplanar_faces(tol).unwrap();
        assert!(
            outcome.groups.is_empty(),
            "nothing to merge, and still joined"
        );
        assert_eq!(outcome.joins.len(), 1);
        assert_eq!(outcome.joins[0].vertex, made.vertex);
        assert_eq!(body.arena_counts(), whole);
        assert!(joinable_vertices(&body, band).unwrap().is_empty());
    }

    /// **An offset door joins only over what it writes.** The split
    /// vertex lies on no edge the offset of a face it does not bound
    /// rewrites, so the door's join (`replace_face::staged_join`) does
    /// not reach it: the door reports no join and the vertex stands, as
    /// the operand stated it.
    #[test]
    fn an_offset_door_joins_only_over_what_it_writes() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let (mut body, made, whole) = split_brick();
        // A face the split edge does not bound.
        let far = body
            .faces()
            .map(|(f, _)| f)
            .find(|&f| {
                body.proven_face(f)
                    .loops()
                    .all(|(_, l, _)| match l.boundary {
                        crate::LoopBoundary::Cycle { first } => body
                            .loop_cycle(first)
                            .unwrap()
                            .iter()
                            .all(|&h| body.get_half_edge(h).unwrap().start != made.vertex),
                        crate::LoopBoundary::Empty { .. } => true,
                    })
            })
            .unwrap();
        let out = crate::replace_face_offset(&mut body, far, 0.25, tol).unwrap();
        assert!(out.joins.is_empty(), "{:?}", out.joins);
        assert_eq!(joinable_vertices(&body, band).unwrap(), vec![made.vertex]);
        assert_ne!(body.arena_counts(), whole, "the split stands");
    }

    /// A unit brick with one edge cut into `k + 1` pieces, and the two
    /// faces that edge bounds.
    fn brick_with_splits(k: usize) -> (Body<f64>, [crate::FaceKey; 2]) {
        let tol = Tol::witness();
        let mut body = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let faces_of = |body: &Body<f64>, e| {
            let d = body.get_edge(e).unwrap();
            [d.he_plus, d.he_minus].map(|h| body.face_of_half_edge(h).unwrap())
        };
        let still = faces_of(&body, body.edges().next().unwrap().0);
        for _ in 0..k {
            // The longest piece between the two faces, cut at its middle.
            let (e, (t0, t1)) = body
                .edges()
                .filter(|&(e, _)| faces_of(&body, e) == still)
                .map(|(e, d)| {
                    let span = body
                        .get_curve_geom(d.curve)
                        .and_then(crate::CurveGeom::certified)
                        .unwrap()
                        .params();
                    (e, span)
                })
                .max_by(|a, b| (a.1.1 - a.1.0).total_cmp(&(b.1.1 - b.1.0)))
                .unwrap();
            body.split_edge(e, 0.5 * (t0 + t1), tol).unwrap();
        }
        (body, still)
    }

    /// **A join derives tier 1 once, however many vertices it joins.**
    /// Each door ends with a join over the `k` vertices cutting one edge
    /// of a brick; the whole-body tier-1 derivations the call runs
    /// (`validate::TIER1_DERIVATIONS`) are the same at every `k`, where
    /// a kill outside the join's scope adds a walk per vertex.
    #[test]
    #[cfg_attr(
        feature = "per-op-postcondition",
        ignore = "pinned red: work/topo/two-topo-rows-assume-the-default-build-under-the-per-op-scalpel"
    )]
    fn a_join_sweeps_tier_1_once_however_many_it_joins() {
        type Door = fn(&mut Body<f64>, [crate::FaceKey; 2]) -> usize;
        // Every face of the solid, as the simultaneous doors require:
        // the cut edge's two faces held, every other face moved in.
        fn moved(body: &Body<f64>, still: [crate::FaceKey; 2]) -> Vec<crate::ChartMove<f64>> {
            body.faces()
                .map(|(f, _)| crate::ChartMove {
                    faces: vec![f],
                    distance: if still.contains(&f) { 0.0 } else { -0.1 },
                })
                .collect()
        }
        fn band() -> Band {
            Band::linear(Tol::witness()).unwrap()
        }
        let doors: [(&str, Door); 3] = [
            ("join_edges", |body, _| {
                body.join_edges(band(), Tol::witness()).unwrap().len()
            }),
            ("replace_face_offset", |body, still| {
                crate::replace_face_offset(body, still[0], 0.25, Tol::witness())
                    .unwrap()
                    .joins
                    .len()
            }),
            ("offset_planes_together", |body, still| {
                crate::offset_planes_together(body, &moved(body, still), band(), Tol::witness())
                    .unwrap()
                    .joins
                    .len()
            }),
        ];
        for (name, door) in doors {
            let derivations: Vec<usize> = [3, 5]
                .into_iter()
                .map(|k| {
                    let (mut body, still) = brick_with_splits(k);
                    let before = crate::validate::TIER1_DERIVATIONS.with(core::cell::Cell::get);
                    let joined = door(&mut body, still);
                    let after = crate::validate::TIER1_DERIVATIONS.with(core::cell::Cell::get);
                    assert_eq!(joined, k, "{name} joins every cut vertex at k = {k}");
                    after - before
                })
                .collect();
            assert_eq!(
                derivations[0], derivations[1],
                "{name}: tier-1 derivations at k = 3 and k = 5 ({derivations:?})"
            );
        }
    }
}

#[cfg(test)]
mod undecided_text {
    //! **The join door ends an undecided join with the reading's own
    //! lever in front of its own**: an enclosure the band leaves
    //! undecided offers subdivision first.
    #![allow(clippy::unwrap_used)]

    use geom_core::{Band, Indeterminate, MarginDiag};

    use super::{JOIN_LEVER, JOIN_SUBJECT, JoinReading, JoinRefusal, JoinUndecided};

    #[test]
    fn a_straddling_join_offers_subdivision_before_its_lever() {
        let diag = Indeterminate {
            margin: MarginDiag::enclosure(-2e-9, 5e-9),
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some("dihedral_arm"),
            terminal_sliver: false,
        };
        let refusal = JoinRefusal::Undecided(JoinUndecided {
            vertex: crate::VertexKey::default(),
            reading: JoinReading::Regularity(diag),
        });
        assert_eq!(
            refusal.render(geom_brep::recourse::Reading::Build),
            format!(
                "{JOIN_SUBJECT} is undecided ({}). Recourse: subdivide the parameter box for \
                 a tighter enclosure, or {JOIN_LEVER}",
                diag.payload()
            )
        );
    }

    /// **A chart image refused on incidence ends by its read at the join**
    /// (D3): a carrier decided off its chart is the door's defect, once; one
    /// read in band at a pose ends in its lever, at a build and at rest,
    /// with no defect appended.
    #[test]
    fn a_join_reading_an_incidence_refusal_ends_by_its_read() {
        use geom_brep::recourse::Reading;
        let refusal = |margin| {
            JoinRefusal::Undecided(JoinUndecided {
                vertex: crate::VertexKey::default(),
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(
                    geom_brep::PcurveCertifyError::CarrierOffChart {
                        chart: geom::SurfaceKind::Cone,
                        carrier: geom::CurveKind::Circle,
                        why: "WHY",
                        margin,
                    },
                )),
            })
        };
        let m = MarginDiag::value(4e-10);
        for at in [Reading::Build, Reading::AtRest] {
            let off = refusal(None).render(at);
            let defect = geom_brep::recourse::defect_ending(geom_brep::recourse::ReadAt::from(at));
            assert!(off.ends_with(defect), "{off}");
            assert_eq!(off.matches("defect").count(), 1, "{off}");
            let at_pose = refusal(Some(geom_brep::OffMargin::InBand(m))).render(at);
            assert!(
                at_pose.contains("Recourse: move the geometry so the edge sits clear")
                    && !at_pose.contains("defect"),
                "{at:?}: {at_pose}"
            );
        }
    }

    /// **The join's image arm ends a non-incidence refusal by its own
    /// ending, once** (D4 ¶1 (iv)): a lane gap or a class reading
    /// lever-only "not yet", never with a defect appended, and an
    /// escalation of a defect check in the door's defect, once (review B's
    /// probe on E1d).
    #[test]
    fn the_joins_image_arm_ends_a_refusal_by_its_own_ending() {
        use geom_brep::recourse::Reading;
        for (e, defect) in [
            (
                geom_brep::PcurveCertifyError::Escalated {
                    check: geom_brep::PcurveCheck::ChartClass,
                    sample: 0,
                    cause: diag("pcurve_probe", MarginDiag::value(3e-9)),
                },
                false,
            ),
            (
                geom_brep::PcurveCertifyError::NoGeneralLane {
                    chart: geom::SurfaceKind::Sphere,
                    carrier: geom::CurveKind::Ellipse,
                    gap: geom_brep::LaneGap::Unwritten,
                },
                false,
            ),
            (
                geom_brep::PcurveCertifyError::Escalated {
                    check: geom_brep::PcurveCheck::ChartWinding,
                    sample: 0,
                    cause: diag("pcurve_probe", MarginDiag::value(3e-9)),
                },
                true,
            ),
        ] {
            let r = JoinRefusal::Undecided(JoinUndecided {
                vertex: crate::VertexKey::default(),
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(e.clone())),
            });
            for at in [Reading::Build, Reading::AtRest] {
                let text = r.render(at);
                let ending = e.ending(at);
                assert!(
                    !ending.is_empty() && text.ends_with(&ending),
                    "{e:?} at {at:?}: {text}"
                );
                assert_eq!(text.matches("There is no way through").count(), 1, "{text}");
                assert_eq!(text.contains("defect"), defect, "{e:?} at {at:?}: {text}");
            }
        }
    }

    /// **A refusal whose payload already ends gets no second ending**
    /// (review on PR 5844, H5), and every refusal ends once, by its own
    /// decision: a stored piece outside one sector in the door's defect,
    /// an unsupported chart (a lane choice) "not yet", never a defect.
    #[test]
    fn the_joins_image_arm_ends_each_refusal_once_by_its_decision() {
        use geom_brep::recourse::Reading;
        let render = |e| {
            JoinRefusal::Undecided(JoinUndecided {
                vertex: crate::VertexKey::default(),
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(e)),
            })
            .render(Reading::Build)
        };
        let sector = render(geom_brep::PcurveCertifyError::SectorRefused {
            piece: 1,
            channel: geom_brep::SectorChannel::Azimuth,
        });
        assert_eq!(sector.matches("Recourse").count(), 0, "{sector}");
        assert_eq!(
            sector.matches(geom_core::KERNEL_DEFECT_ENDING).count(),
            1,
            "{sector}"
        );
        assert!(
            sector.ends_with(geom_core::KERNEL_DEFECT_ENDING),
            "{sector}"
        );
        let unsupported = render(geom_brep::PcurveCertifyError::UnsupportedChart {
            chart: geom::SurfaceKind::Sphere,
        });
        assert_eq!(
            unsupported.matches(geom_core::NOT_YET_ENDING).count(),
            1,
            "{unsupported}"
        );
        assert!(
            unsupported.ends_with(geom_core::NOT_YET_ENDING) && !unsupported.contains("defect"),
            "{unsupported}"
        );
    }

    /// The undecided escalation at `margin`, read by `predicate`.
    fn diag(predicate: &'static str, margin: MarginDiag) -> Indeterminate {
        Indeterminate {
            margin,
            band: Band::new(1e-9, 1e-8).unwrap(),
            predicate: Some(predicate),
            terminal_sliver: false,
        }
    }

    /// The Boolean's render of an undecided join at `reading`.
    fn boolean_text(reading: JoinReading) -> String {
        JoinUndecided {
            vertex: crate::VertexKey::default(),
            reading,
        }
        .render(geom_brep::recourse::Reading::Build)
    }

    /// **The Boolean's join refusal asks the question its reading answers
    /// and ends once**: a regularity reading is sized (a value offers the
    /// tolerance, a straddle subdivision, an unreadable margin the
    /// inputs); a chart-class reading is a form selection and ends in the
    /// lever alone, with the unreadable note where its margin is poison.
    #[test]
    fn the_booleans_undecided_join_ends_once() {
        let regularity =
            |margin| boolean_text(JoinReading::Regularity(diag("dihedral_arm", margin)));
        let chart_class = |margin| {
            boolean_text(JoinReading::ChartClass(
                geom_brep::IsoFamilyRefusal::Undecided(diag("pcurve_chart_radial_moving", margin)),
            ))
        };
        let cases = [
            (
                "a chart-class value",
                chart_class(MarginDiag::value(5e-9)),
                "whether an edge meeting another at a vertex on one curve runs along a \
                 ruling of its chart or around it is undecided: margin 5e-9 lies inside the \
                 ambiguity band (1e-9, 1e-8). Recourse: move the geometry",
            ),
            (
                "a chart-class straddle",
                chart_class(MarginDiag::enclosure(-2e-9, 5e-9)),
                "whether an edge meeting another at a vertex on one curve runs along a \
                 ruling of its chart or around it is undecided: enclosure [-2e-9, 5e-9] cannot be \
                 classified against the ambiguity band (1e-9, 1e-8). Recourse: move the \
                 geometry",
            ),
            (
                "an unreadable chart-class margin",
                chart_class(MarginDiag::INVALID),
                "whether an edge meeting another at a vertex on one curve runs along a \
                 ruling of its chart or around it is undecided: margin is invalid (NaN or a \
                 refused enclosure) against the ambiguity band (1e-9, 1e-8). Recourse: move the \
                 geometry; an unreadable margin may indicate a kernel bug worth reporting",
            ),
            (
                "a regularity value",
                regularity(MarginDiag::value(5e-9)),
                "whether two edges meeting at a vertex on one curve are one edge is undecided: \
                 margin 5e-9 lies inside the ambiguity band (1e-9, 1e-8). Recourse: move the \
                 geometry, or, if this size is intended, tighten the tolerance below 5e-10 m",
            ),
            (
                "a regularity straddle",
                regularity(MarginDiag::enclosure(-2e-9, 5e-9)),
                "whether two edges meeting at a vertex on one curve are one edge is undecided: \
                 enclosure [-2e-9, 5e-9] cannot be classified against the ambiguity band (1e-9, \
                 1e-8). Recourse: subdivide the parameter box for a tighter enclosure, or move \
                 the geometry",
            ),
            (
                "an unreadable regularity margin",
                regularity(MarginDiag::INVALID),
                "whether two edges meeting at a vertex on one curve are one edge is undecided: \
                 margin is invalid (NaN or a refused enclosure) against the ambiguity band \
                 (1e-9, 1e-8). Recourse: check the operation's inputs upstream, then move the \
                 geometry; an unreadable margin may indicate a kernel bug worth reporting",
            ),
        ];
        for (what, got, want) in cases {
            assert_eq!(got, want, "{what}");
        }
    }

    /// A chart image that did not derive is no class reading: the
    /// Boolean names the image's refusal, which ends by its own decision,
    /// here a lane choice's "not yet" (an incidence read among them ends
    /// by its own ending, never as undecided).
    #[test]
    fn a_chart_image_that_did_not_derive_ends_by_its_own_decision() {
        let got = boolean_text(JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(
            geom_brep::PcurveCertifyError::UnsupportedChart {
                chart: geom::SurfaceKind::Sphere,
            },
        )));
        assert_eq!(
            got,
            "two edges meeting at a vertex on one surface read their family off a chart image \
                 that did not derive: pcurve certification: no sphere-chart lane covers this pcurve — every analytic \
             chart certifies its closed-form (Harmonic) classes, a NURBS chart routes through \
             its description-driven iso/fitted lanes instead of this door, and an image outside \
             the chart's harmonic family belongs to the fitted lane where one exists. \
             There is no way through yet"
        );
    }
}

#[cfg(test)]
mod kernel_text {
    //! **A refused kill or re-mint ends by its door**: at a build, the
    //! kernel-limit last resort, since user geometry reaches Euler
    //! certification's limit there; at rest, the kernel-or-file defect
    //! ending; at the import door, the file's.

    use geom_brep::recourse::Reading;
    use geom_core::FileCoincidence;

    use super::JoinRefusal;

    #[test]
    fn a_refused_join_ends_by_its_door() {
        let refusal = JoinRefusal::Kernel {
            kind: crate::BooleanErrorKind::Euler,
        };
        let lead = "the join of two edges meeting at a vertex on one curve refused (Euler). ";
        let rows = [
            (
                "a build",
                refusal.render(geom_brep::recourse::Reading::Build),
                "Recourse: loosen the tolerance, as a last resort; this refusal may indicate \
                 a kernel bug worth reporting",
            ),
            (
                "at rest",
                refusal.render(Reading::AtRest),
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING,
            ),
            (
                "the import door",
                refusal.render(FileCoincidence::new(1e-6)),
                geom_core::DAMAGED_FILE_ENDING,
            ),
        ];
        for (door, got, ending) in rows {
            assert_eq!(got, format!("{lead}{ending}"), "{door}");
        }
    }
}

#[cfg(test)]
mod class_zero_rows {
    //! **The class Zero a join acts on** ([`ClassZero`]): a sphere cap
    //! whose rim is two arcs through one pair of points, the upper on
    //! the parallel at `z = ½`, the lower on a circle through the same
    //! two points tilted about `x` by `tilt` (at zero tilt the same
    //! parallel, written with another reference direction, so the two
    //! carriers are not one by content). Joined, the two arcs are one
    //! closed rim, and each was read on the chart's parallels: the join
    //! hands back that reading's Zero; definite, the tilt keeps the two
    //! arcs. A cylinder wall's ruling split and its upper piece restated
    //! on its own line (`ruled`) joins back on the same reading, its Zero
    //! covering how far the piece stands off the wall. A ruling bent in
    //! band (`bent`: its corner moved round the wall, so each piece
    //! leaves the ruling within the band over its span) refuses typed,
    //! ending on its lever.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::collections::BTreeSet;

    use geom_core::{Band, Decide, Point3, Tol, Vec3};

    use super::{BooleanError, JoinReading, join_all};
    use crate::body::Body;
    use crate::construction::{FaceSurface, MefSite, MevSite};

    const Z1: f64 = 0.5;

    /// The cap of the unit sphere above its rim (module docs), the lower
    /// arc on the circle tilted by `tilt` radians; `grow` grows the upper
    /// arc's radius and the lower's by its two values (off the sphere by
    /// about that, which only the incidence test reads: the tilt and
    /// centre gates read nothing).
    fn cap<T: Decide + crate::props::AtRestPolicy>(
        tilt: f64,
        (grow_upper, grow): (f64, f64),
        tol: Tol,
    ) -> Body<T> {
        let tau = core::f64::consts::TAU;
        let c = T::from_f64;
        let p3 = |[x, y, z]: [f64; 3]| Point3::new(c(x), c(y), c(z));
        let v3 = |[x, y, z]: [f64; 3]| Vec3::new(c(x), c(y), c(z));
        let r1 = (1.0 - Z1 * Z1).sqrt();
        let (east, west) = ([r1, 0.0, Z1], [-r1, 0.0, Z1]);
        // The tilted circle: the sphere's section by the plane through
        // the chord `east`–`west` with normal `n`, centred at the foot of
        // the sphere's centre on it, its reference direction off the
        // chord so the parallel's twin is another carrier.
        let n = [0.0, -tilt.sin(), tilt.cos()];
        let h = Z1 * tilt.cos();
        let centre = n.map(|k| k * h);
        let rho = (1.0 - h * h).sqrt();
        let cross = |a: [f64; 3], b: [f64; 3]| {
            [
                a[1] * b[2] - a[2] * b[1],
                a[2] * b[0] - a[0] * b[2],
                a[0] * b[1] - a[1] * b[0],
            ]
        };
        let dot = |a: [f64; 3], b: [f64; 3]| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        // In the plane, off the chord's direction by 0.3 rad.
        let beta = 0.3_f64;
        let u_ref: [f64; 3] = core::array::from_fn(|i| {
            beta.cos() * [1.0, 0.0, 0.0][i] + beta.sin() * cross(n, [1.0, 0.0, 0.0])[i]
        });
        // The lower arc runs west → east through `y < 0`, so the carrier's
        // axis is whichever of `±n` turns it that way.
        let arc = |axis: [f64; 3]| {
            let v = cross(axis, u_ref);
            let angle = |p: [f64; 3]| {
                let d: [f64; 3] = core::array::from_fn(|i| p[i] - centre[i]);
                dot(d, v).atan2(dot(d, u_ref)).rem_euclid(tau)
            };
            let t0 = angle(west);
            let t1 = angle(east) + if angle(east) < t0 { tau } else { 0.0 };
            let mid = 0.5 * (t0 + t1);
            let y = rho * (mid.cos() * u_ref[1] + mid.sin() * v[1]);
            (axis, t0, t1, y)
        };
        let (axis, t0, t1) = match arc(n) {
            (axis, t0, t1, y) if y < 0.0 => (axis, t0, t1),
            _ => {
                let (axis, t0, t1, _) = arc(n.map(|k| -k));
                (axis, t0, t1)
            }
        };
        let lower = geom::Curve3::circle(p3(centre), v3(axis), c(rho + grow), v3(u_ref));
        let upper = geom::Curve3::circle(
            p3([0.0, 0.0, Z1]),
            Vec3::unit_z(),
            c(r1 + grow_upper),
            Vec3::unit_x(),
        );
        let mut body = Body::<T>::new();
        let seed = body.mvfs(p3(east), true).unwrap();
        body.set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Sphere {
                    center: Point3::origin(),
                    radius: c(1.0),
                    axis: Vec3::unit_z(),
                    u_ref: Vec3::unit_x(),
                },
                sense: true,
            },
        )
        .unwrap();
        let half = body
            .mev(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p3(west),
                geom_brep::EdgeCurveSpec::arc_of_circle(upper, c(0.0), c(core::f64::consts::PI))
                    .unwrap(),
                tol,
            )
            .unwrap();
        let below = body
            .mef(
                MefSite::Chords {
                    he1: half.he_minus,
                    he2: half.he_plus,
                },
                geom_brep::EdgeCurveSpec::arc_of_circle(lower, c(t0), c(t1)).unwrap(),
                FaceSurface::Inherit,
                tol,
            )
            .unwrap();
        for edge in [half.edge, below.edge] {
            body.describe_at_rest(edge, tol).unwrap();
        }
        crate::mint_pcurves(&mut body, tol).unwrap();
        body
    }

    fn joined<T: Decide + crate::props::AtRestPolicy>(
        tilt: f64,
        grow: (f64, f64),
    ) -> Result<(usize, Vec<super::ClassZero>, Band), BooleanError> {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = cap::<T>(tilt, grow, tol);
        let (joins, _, zeros) = join_all(&mut body, band, tol, &|_| true, &BTreeSet::new())?;
        Ok((joins.len(), zeros, band))
    }

    fn the_parallels_join_and_hand_back_their_zero<T: Decide + crate::props::AtRestPolicy>(
        scalar: &str,
    ) {
        // One arc off the sphere by half the band's zero, the upper, then
        // the lower: its incidence reads it, its class gates read nothing,
        // and the row carries it whichever arc the join keeps.
        let off = 0.5 * Tol::witness().eps();
        for grow in [(off, 0.0), (0.0, off)] {
            let (joins, zeros, band) = joined::<T>(0.0, grow).unwrap();
            assert_eq!(
                joins, 1,
                "{scalar}, {grow:?}: the two arcs of one parallel are one rim"
            );
            let [zero] = zeros.as_slice() else {
                panic!("{scalar}, {grow:?}: one class Zero for the one join, got {zeros:?}");
            };
            assert_ne!(
                zero.gone, zero.kept,
                "{scalar}, {grow:?}: the Zero names both arcs"
            );
            let reading = zero.margin.diagnostic_f64_for_error_text();
            let reach = match reading {
                geom_core::ErrorTextReading::Value(v) => v.abs(),
                geom_core::ErrorTextReading::Enclosure { lo, hi } => lo.abs().max(hi.abs()),
                other => panic!("{scalar}, {grow:?}: the Zero reads a margin, got {other:?}"),
            };
            assert!(
                reach <= band.zero() && reach >= 0.4 * off,
                "{scalar}, {grow:?}: the margin is a Zero covering the arc's miss ({off:e}), \
                 got {reading:?}"
            );
        }
    }

    #[test]
    fn the_parallels_join_and_hand_back_their_zero_at_f64() {
        the_parallels_join_and_hand_back_their_zero::<f64>("f64");
    }

    #[test]
    fn the_parallels_join_and_hand_back_their_zero_at_interval() {
        the_parallels_join_and_hand_back_their_zero::<geom_core::Interval>("interval");
    }

    /// Where the wall starts, off the chart's seam.
    const U0: f64 = 0.3;

    /// An open wall of the unit cylinder about `z` over `u ∈ [U0, U0 + 1]`,
    /// `v ∈ [0, 0.4]`, its ruling at `u = U0` split at `v = 0.2` and the
    /// upper piece restated on its own line through the split point,
    /// parallel to the axis and `off` out from the wall, so the two
    /// carriers are not one by content.
    fn ruled<T: Decide + crate::props::AtRestPolicy>(off: f64, tol: Tol) -> Body<T> {
        let c = T::from_f64;
        let mut body = Body::<T>::new();
        let face = crate::test_support::cyl_wall_sheet(
            &mut body,
            crate::test_support::CylFrame {
                origin: geom_core::Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            (U0, U0 + 1.0),
            (0.0, 0.4),
            tol,
        );
        let surface = body.get_face(face).unwrap().surface;
        let on_ruling = |e: &crate::entity::Edge| {
            let curve = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
            let geom::Curve3::Line(line) = curve.carrier() else {
                return false;
            };
            let f = |x: T| x.plain_constant().unwrap_or(f64::NAN);
            (f(line.origin.x) - U0.cos()).abs() < 1e-12
                && (f(line.origin.y) - U0.sin()).abs() < 1e-12
        };
        let ruling = body
            .edges()
            .find(|(_, e)| on_ruling(e))
            .map(|(k, _)| k)
            .expect("the sheet has a ruling at u = 0");
        let (t0, t1) = body
            .get_curve_geom(body.get_edge(ruling).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .params();
        let split = body.split_edge(ruling, (t0 + t1) * c(0.5), tol).unwrap();
        let end = |h: crate::entity::HalfEdgeKey| {
            let v = body.get_half_edge(h).unwrap().start;
            *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
        };
        let (from, to) = (end(split.he_plus), end(split.he_minus));
        let up = if (to.z - from.z).plain_constant().unwrap_or(f64::NAN) > 0.0 {
            1.0
        } else {
            -1.0
        };
        let (s0, c0) = U0.sin_cos();
        let dir = Vec3::new(c(0.0), c(0.0), c(up));
        let length = (to - from).norm();
        let spec = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::chart(surface),
            carrier: geom::Curve3::Line(geom::Line {
                origin: from + Vec3::new(c(c0 * off), c(s0 * off), c(0.0)),
                dir,
            }),
            param_start: c(0.0),
            param_end: length,
        };
        body.set_edge_curve(split.new_edge, spec, tol).unwrap();
        body.describe_at_rest(ruling, tol).unwrap();
        crate::mint_pcurves(&mut body, tol).unwrap();
        body
    }

    /// The wall of [`ruled`], its top corner on the ruling at `u = U0`
    /// moved round the wall by `shift` (an arc length on the unit
    /// radius), and that ruling restated as the line from its foot to the
    /// moved corner, then split at its middle: two pieces of one line,
    /// each leaving the ruling by `shift / 2` over its span. Its ends stay
    /// on their vertices, so the edge certifies, and its class reading
    /// is metered over each piece (`geom_brep::ruling_miss`).
    fn bent<T: Decide + crate::props::AtRestPolicy>(shift: f64, tol: Tol) -> Body<T> {
        let c = T::from_f64;
        let f = |x: T| x.plain_constant().unwrap_or(f64::NAN);
        let mut body = Body::<T>::new();
        let face = crate::test_support::cyl_wall_sheet(
            &mut body,
            crate::test_support::CylFrame {
                origin: geom_core::Point3::origin(),
                axis: Vec3::unit_z(),
                radius: 1.0,
                u_ref: Vec3::unit_x(),
            },
            (U0, U0 + 1.0),
            (0.0, 0.4),
            tol,
        );
        let surface = body.get_face(face).unwrap().surface;
        let ruling = body
            .edges()
            .find(|(_, e)| {
                let curve = body.get_curve_geom(e.curve).unwrap().certified().unwrap();
                matches!(curve.carrier(), geom::Curve3::Line(line)
                    if (f(line.origin.x) - U0.cos()).abs() < 1e-12
                        && (f(line.origin.y) - U0.sin()).abs() < 1e-12)
            })
            .map(|(k, _)| k)
            .expect("the sheet has a ruling at u = U0");
        let d = body.get_edge(ruling).unwrap().clone();
        let ends = [d.he_plus, d.he_minus].map(|h| {
            let v = body.get_half_edge(h).unwrap().start;
            body.get_vertex(v).unwrap().point
        });
        let z = |k: crate::geometry::PointKey| f(body.points[k].z);
        let (foot, top) = if z(ends[0]) < z(ends[1]) {
            (ends[0], ends[1])
        } else {
            (ends[1], ends[0])
        };
        let (s1, c1) = (U0 + shift).sin_cos();
        let moved = Point3::new(c(c1), c(s1), body.points[top].z);
        body.points[top] = moved;
        let from = body.points[foot];
        let length = (moved - from).norm();
        // The edge runs he_plus's start to its end: foot to top or back.
        let (origin, toward) = if ends[0] == foot {
            (from, moved)
        } else {
            (moved, from)
        };
        let spec = geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::chart(surface),
            carrier: geom::Curve3::Line(geom::Line {
                origin,
                dir: (toward - origin) / length,
            }),
            param_start: c(0.0),
            param_end: length,
        };
        body.set_edge_curve(ruling, spec, tol).unwrap();
        body.split_edge(ruling, length * c(0.5), tol).unwrap();
        crate::mint_pcurves(&mut body, tol).unwrap();
        body
    }

    fn joined_ruled<T: Decide + crate::props::AtRestPolicy>(
        off: f64,
    ) -> Result<(usize, Vec<super::ClassZero>), BooleanError> {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = ruled::<T>(off, tol);
        let (joins, _, zeros) = join_all(&mut body, band, tol, &|_| true, &BTreeSet::new())?;
        Ok((joins.len(), zeros))
    }

    fn a_ruling_on_another_line_joins_and_hands_back_its_zero<
        T: Decide + crate::props::AtRestPolicy,
    >(
        scalar: &str,
    ) {
        // The restated piece stands off the wall by half the band's
        // zero, which its incidence reads and the row must carry.
        let off = 0.5 * Tol::witness().eps();
        let (joins, zeros) = joined_ruled::<T>(off).unwrap();
        let [zero] = zeros.as_slice() else {
            panic!("{scalar}: one class Zero for the one join, got {zeros:?}");
        };
        assert_eq!(
            joins, 1,
            "{scalar}: the two pieces of the ruling are one edge"
        );
        let reach = match zero.margin.diagnostic_f64_for_error_text() {
            geom_core::ErrorTextReading::Value(v) => v.abs(),
            geom_core::ErrorTextReading::Enclosure { lo, hi } => lo.abs().max(hi.abs()),
            other => panic!("{scalar}: the Zero reads a margin, got {other:?}"),
        };
        assert!(
            reach >= 0.5 * off,
            "{scalar}: the row carries the most a carrier misses its family by ({off:e} off the \
             wall), got {reach:e}"
        );
    }

    #[test]
    fn a_ruling_on_another_line_joins_and_hands_back_its_zero_at_f64() {
        a_ruling_on_another_line_joins_and_hands_back_its_zero::<f64>("f64");
    }

    #[test]
    fn a_ruling_on_another_line_joins_and_hands_back_its_zero_at_interval() {
        a_ruling_on_another_line_joins_and_hands_back_its_zero::<geom_core::Interval>("interval");
    }

    fn a_tilt_in_band_refuses_on_its_lever<T: Decide + crate::props::AtRestPolicy>(scalar: &str) {
        let eps = Tol::witness().eps();
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let mut body = bent::<T>(6.0 * eps, tol);
        let Err(refusal) = join_all(&mut body, band, tol, &|_| true, &BTreeSet::new()) else {
            panic!("{scalar}: a ruling turned in band refuses the join");
        };
        let BooleanError::JoinUndecided(undecided) = &refusal else {
            panic!("{scalar}: the join's own refusal, got {refusal:?}");
        };
        assert!(
            matches!(
                undecided.reading,
                JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Undecided(_))
            ),
            "{scalar}: the class reading is in band, got {undecided:?}"
        );
        let build = geom_brep::recourse::ReadAt::Run(geom_brep::recourse::Reading::Build);
        // Read at a build, each door ends on a lever, never on the
        // kernel's defect.
        for (door, text) in [
            ("the boolean", refusal.render(build)),
            (
                "the join door",
                super::JoinRefusal::of(&refusal).render(build),
            ),
        ] {
            assert!(
                text.contains("Recourse: move ") && !text.contains("kernel"),
                "{scalar}, {door}: the in-band reading ends on its lever: {text}"
            );
        }
    }

    #[test]
    fn a_tilt_in_band_refuses_on_its_lever_at_f64() {
        a_tilt_in_band_refuses_on_its_lever::<f64>("f64");
    }

    #[test]
    fn a_tilt_in_band_refuses_on_its_lever_at_interval() {
        a_tilt_in_band_refuses_on_its_lever::<geom_core::Interval>("interval");
    }

    fn a_definite_tilt_keeps_both_arcs<T: Decide + crate::props::AtRestPolicy>(scalar: &str) {
        let (joins, zeros, _) = joined::<T>(0.3, (0.0, 0.0)).unwrap();
        assert_eq!(
            (joins, zeros.len()),
            (0, 0),
            "{scalar}: a tilted arc is no parallel's"
        );
    }

    #[test]
    fn a_definite_tilt_keeps_both_arcs_at_f64() {
        a_definite_tilt_keeps_both_arcs::<f64>("f64");
    }

    #[test]
    fn a_definite_tilt_keeps_both_arcs_at_interval() {
        a_definite_tilt_keeps_both_arcs::<geom_core::Interval>("interval");
    }
}

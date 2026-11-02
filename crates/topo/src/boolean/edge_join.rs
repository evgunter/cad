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
//! the op's one substitution door ([`super::ops::carry`]).
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
//! there.
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
}

/// The whole refusal: a regularity reading asks [`JOIN_SUBJECT`], a
/// sized margin, and ends in [`Indeterminate::ending`] under
/// [`geom_core::COINCIDENCE_RECOURSE`]; a chart-class reading asks its
/// own question ([`geom_brep::ISO_FAMILY_CLAUSE`]), a form selection on
/// no size the user chose, and ends in that lever alone
/// ([`geom_brep::recourse::LeverOnly`]).
impl core::fmt::Display for JoinUndecided {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.reading {
            JoinReading::Regularity(diag) => write!(
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
                 that did not derive: {e}"
            ),
        }
    }
}

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
        .filter(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
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
    let Some(edge) = body.get_edge(he.edge) else {
        return false;
    };
    let ends = [edge.he_plus, edge.he_minus].map(|h| body.get_half_edge(h).map(|h| h.start));
    ends == [Some(v), Some(v)]
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
        let w = p - origin;
        (w - axis * w.dot(axis)).norm()
    };
    let arm = match *s {
        geom::Surface::Cone { apex, .. } => (p - apex).norm(),
        geom::Surface::Sphere { center, axis, .. } => off_axis(center, axis),
        geom::Surface::Torus { center, axis, .. } => off_axis(center, axis),
        _ => return Ok(false),
    };
    geom_core::k_stats::decide("join_regular_point", Margin::of(arm), band)
        .map(|sign| sign != Sign::Positive)
}

/// Whether `e`'s carrier holds a crease run at its end `w`: a spline
/// carrier whose parameter there is one point and is its own clamped
/// end, or an interior knot of multiplicity at least its degree. A
/// carrier run on through `w` would carry a knot of that multiplicity,
/// so `w` is the vertex the knot structure puts there, not one a join
/// takes back (module docs). Structural: the parameter's exact equality
/// with one of the carrier's own knots, no margin.
fn crease_at<T: Decide>(body: &Body<T>, e: EdgeKey, d: &crate::entity::Edge, w: VertexKey) -> bool {
    let Some(c) = body.edge_curve_linked(e, d).certified() else {
        return false;
    };
    let geom::Curve3::Nurbs(n) = c.carrier() else {
        return false;
    };
    let (t0, t1) = c.params();
    let starts_at_w = body.get_half_edge(d.he_plus).map(|h| h.start) == Some(w);
    let t = if starts_at_w { t0 } else { t1 };
    let knots = n.knot_values();
    let (a, b) = knots.domain();
    // Equal as an exact comparison reads it: the least band, where only
    // a zero difference is Zero and an enclosure straddling it is no
    // answer, so no crease.
    [a, b].into_iter().chain(knots.crease_values()).any(|k| {
        (t - k)
            .sign_within(Band::least())
            .is_ok_and(|d| d.sign == Sign::Zero)
    })
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
    // The joined edge runs on the kept carrier through `w`, so `kept`
    // is an edge whose carrier has no crease there; where both have
    // one, `w` is the vertex the knot structure puts there.
    let (h1, h2, e1, e2) = match (
        crease_at(body, e1, link(e1, *h1), w),
        crease_at(body, e2, link(e2, *h2), w),
    ) {
        (true, true) => return Ok(None),
        (false, true) => (h2, h1, e2, e1),
        _ => (h1, h2, e1, e2),
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
    let join = |planar| Join {
        he,
        gone: e1,
        kept: e2,
        far,
        closed: far == far2,
        planar,
    };
    let (sf, sg) = s1.surfaces();
    let surface = |side: crate::readback::EdgeSide| {
        let face = proven(&body.faces, side.face, EntityId::Face);
        body.face_surface_linked(side.face, face)
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
        return Ok(joins.then(|| join(true)));
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
                Some(Locus::Transverse)
            }
            geom_brep::EdgeDescription::TangentIntersection { pair, .. }
                if sf != sg && *pair == geom_brep::SurfacePair::new(sf, sg) =>
            {
                Some(Locus::Tangent)
            }
            geom_brep::EdgeDescription::Chart(chart) if sf == sg && chart.surface == sf => {
                geom_brep::chart_iso_family(c.carrier(), surf_f, band)
                    .map_err(|e| undecided(JoinReading::ChartClass(e)))?
                    .map(Locus::Iso)
            }
            geom_brep::EdgeDescription::OnPair { pair }
                if sf != sg && *pair == geom_brep::SurfacePair::new(sf, sg) =>
            {
                let mut found = None;
                for (minus, s) in [(false, surf_f), (true, surf_g)] {
                    if let Some(family) = geom_brep::chart_iso_family(c.carrier(), s, band)
                        .map_err(|e| undecided(JoinReading::ChartClass(e)))?
                    {
                        found = Some(Locus::PairIso(minus, family));
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
    let Some(locus) = locus_of(e1, d1)? else {
        return Ok(None);
    };
    if locus_of(e2, d2)? != Some(locus) {
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
        return Ok(Some(join(false)));
    }
    // The pair's reading at `w`, levered at the two faces' extent as
    // every reader of their wedge is (`crate::wedge_reach`), read from
    // both edges so the lever does not depend on which edge the arena
    // lists first.
    let reach = [e1, e2]
        .into_iter()
        .filter_map(|e| crate::edge_wedge_reach(body, e))
        .reduce(T::max);
    let Some(reach) = reach else {
        return Ok(None);
    };
    let class = geom_brep::classify_dihedral(surf_f, surf_g, p, reach, band)
        .map_err(|e| regularity(e.diag()))?;
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
    Ok(regular.then(|| join(false)))
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
) -> Result<Vec<EdgeJoin>, BooleanError> {
    // The boolean's own surgery body, re-minted whole after the carry:
    // no staging and no re-mint here.
    let free = free_vertices(body, recorded);
    let (joined, _) = join_all(body, band, tol, &|_| true, &free)?;
    for j in &joined {
        desc.substitute((Cell::Vertex(j.vertex), Cell::Edge(j.kept)))?;
        desc.substitute((Cell::Edge(j.gone), Cell::Edge(j.kept)))?;
        if let Some(v) = j.conventional {
            desc.substitute((Cell::Vertex(v), Cell::Edge(j.kept)))?;
        }
    }
    record_killed_lineage(lineage, split_from, joined.iter().map(|j| j.gone));
    Ok(joined)
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
    #[must_use]
    pub fn render(&self, at: impl Into<geom_brep::recourse::ReadAt>) -> String {
        use geom_brep::recourse::{ReadAt, Reading, defect_ending, unreadable_margin_note};
        let at = at.into();
        let defect = defect_ending(at);
        let undecided = |diag: &Indeterminate| {
            let ending = match at {
                ReadAt::Run(Reading::Build) => diag.ending(JOIN_LEVER),
                _ => diag.ending_noted(JOIN_LEVER, unreadable_margin_note(at)),
            };
            format!("{JOIN_SUBJECT} is undecided ({}). {ending}", diag.payload())
        };
        match self {
            Self::Undecided(JoinUndecided {
                reading: JoinReading::Regularity(diag),
                ..
            })
            | Self::Undecided(JoinUndecided {
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Undecided(diag)),
                ..
            }) => undecided(diag),
            Self::Undecided(JoinUndecided {
                reading: JoinReading::ChartClass(geom_brep::IsoFamilyRefusal::Image(e)),
                ..
            }) => match e.render_ended(at) {
                // The image's refusal ends by its own ending at this door:
                // an incidence read by the one map (D3), a lane gap "not
                // yet", an escalation once, as its decision ends it.
                Some(ended) => format!(
                    "two edges meeting at a vertex on one surface read their family off a chart image \
                 that did not derive: {ended}"
                ),
                // A refusal no decision owns whose payload carries no
                // recourse (an unsupported class, a band the tolerance
                // cannot form), on a body the kernel built.
                None => format!(
                    "two edges meeting at a vertex on one surface read their family off a chart image \
                 that did not derive ({e}). {defect}"
                ),
            },
            Self::CarrierUnsupported { carrier, closed } => format!(
                "two edges meet at a vertex on one {} carrier, which the join cannot run an \
                 edge on {} yet (work/splice/joining-a-spline-carrier-is-unbuilt); there is no \
                 way through this yet",
                carrier.name(),
                if *closed {
                    "over a whole period"
                } else {
                    "through the vertex"
                }
            ),
            Self::Kernel { kind } => format!(
                "the join of two edges meeting at a vertex on one curve refused ({}). {}",
                kind_word(*kind),
                match at {
                    ReadAt::Run(Reading::Build) => geom_core::KERNEL_LIMIT_RECOURSE,
                    _ => defect,
                }
            ),
        }
    }
}

/// Read at a build: every operation that reports it joins the edges of
/// a body it built.
impl core::fmt::Display for JoinRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.render(geom_brep::recourse::Reading::Build))
    }
}

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
        let (joined, _) = join_all(&mut staged, band, tol, &|_| true, &BTreeSet::new())?;
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
        let (joined, dead) = join_all(&mut body, band, tol, within, &BTreeSet::new())?;
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

/// [`Body::join_edges`](crate::construction::Construct::join_edges)' loop, on the staged body.
fn join_all<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    band: Band,
    tol: Tol,
    within: &dyn Fn(VertexKey) -> bool,
    free: &BTreeSet<VertexKey>,
) -> Result<(Vec<EdgeJoin>, Vec<HalfEdgeKey>), BooleanError> {
    let mut joined = Vec::new();
    let mut dead = Vec::new();
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
            return Ok((joined, dead));
        };
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

/// The joined edge's spec: `kept`'s carrier over its interval run on
/// through `w` to `gone`'s far end, a whole period where the join
/// closes, its description restated over the new interval.
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
    let ((kept, kd), (gone, _)) = curve(join.kept)
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
    let tx = if join.closed {
        let period = T::from_f64(core::f64::consts::TAU);
        match carrier {
            geom::Curve3::Circle { .. }
            | geom::Curve3::Ellipse { .. }
            | geom::Curve3::Spiric { .. } => {
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
        carrier
            .param_near(gone.carrier().mid_point(g0, g1), tw)
            .and_then(|tm| carrier.param_near(far, tm))
            .ok_or(BooleanError::JoinCarrierUnsupported {
                edge: join.kept,
                carrier: carrier.kind(),
                closed: false,
            })?
    };
    let (a, b) = if backward { (tx, t1) } else { (t0, tx) };
    // A declaration's parameter is affine in the carrier's, so the kept
    // edge's runs on over the joined span exactly as a split restricts
    // it; certification re-meters it.
    let declared = match kept.authority() {
        geom_brep::EdgeAuthority::Declared(mc) => {
            let span = t1 - t0;
            Some(mc.restrict((a - t0) / span, (b - t0) / span))
        }
        geom_brep::EdgeAuthority::Derived => None,
    };
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
        let far = body
            .half_edge_end(body.get_edge(strut.edge).unwrap().he_plus)
            .unwrap();
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
                body.face_loops_linked(f, body.get_face(f).unwrap())
                    .all(|(_, l)| match l.boundary {
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
            refusal.to_string(),
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
                    chart: geom::SurfaceKind::Torus,
                    carrier: geom::CurveKind::Circle,
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
                let ending = e.ending(at).unwrap_or_default();
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
    /// (review on PR 5844, H5): a piece outside one sector carries its own
    /// recourse and ends there, with no defect after it; an unsupported
    /// chart carries none, and the join ends it in its defect, once.
    #[test]
    fn the_joins_image_arm_ends_a_self_ended_payload_once() {
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
        assert_eq!(sector.matches("Recourse").count(), 1, "{sector}");
        assert!(!sector.contains("defect"), "{sector}");
        let unsupported = render(geom_brep::PcurveCertifyError::UnsupportedChart {
            chart: geom::SurfaceKind::Sphere,
        });
        assert_eq!(
            unsupported.matches(geom_core::KERNEL_DEFECT_ENDING).count(),
            1,
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
        .to_string()
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
                 cylinder's ruling or around it is undecided: margin 5e-9 lies inside the \
                 ambiguity band (1e-9, 1e-8). Recourse: move the geometry",
            ),
            (
                "a chart-class straddle",
                chart_class(MarginDiag::enclosure(-2e-9, 5e-9)),
                "whether an edge meeting another at a vertex on one curve runs along a \
                 cylinder's ruling or around it is undecided: enclosure [-2e-9, 5e-9] cannot be \
                 classified against the ambiguity band (1e-9, 1e-8). Recourse: move the \
                 geometry",
            ),
            (
                "an unreadable chart-class margin",
                chart_class(MarginDiag::INVALID),
                "whether an edge meeting another at a vertex on one curve runs along a \
                 cylinder's ruling or around it is undecided: margin is invalid (NaN or a \
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
    /// Boolean names the image's refusal, which ends in whatever it
    /// carries, here none (an incidence read among them ends by its own
    /// ending, never as undecided).
    /// `work/honejoin/the-join-lever-names-a-pole-for-a-chart-class-reading.md`
    /// owns giving it one verdict, and re-baselines this text.
    #[test]
    fn a_chart_image_that_did_not_derive_ends_without_a_recourse_today() {
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
             the chart's harmonic family belongs to the fitted lane where one exists"
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
                refusal.to_string(),
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

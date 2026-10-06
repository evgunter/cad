//! **The join**: a vertex of valence 2 whose two edges lie between the
//! same two planar faces is no corner, so the two edges are one edge
//! cut for no reason (`docs/DESIGN.md`, maximal edges). The join kills
//! the vertex and makes the two edges one. Every boolean output stage
//! runs it after the merge ([`join_stage`]) and writes, per join, the
//! substitution rows that carry every contact record naming the three
//! cells it replaces onto the edge it makes, through the op's one
//! substitution door ([`super::ops::carry`]).
//!
//! Joinable is decided by structure alone, no value compared: the two
//! edges' face pairs are one pair of distinct faces, both on `Plane`
//! carriers of distinct keys, and both edges are certified as the
//! `Intersection` of those two surface keys. Two planes that cross meet
//! in one line, so the two edges lie on it; two that do not cross
//! certify no intersection, so coplanar faces sharing a bent boundary
//! are never joined. A valence-2 vertex between curved faces is outside
//! this door: whether its two edges share a carrier is a question of
//! the intersection branch, which no key answers yet.

use std::collections::BTreeMap;

use geom_core::{Band, Decide, Real, Tol};

use super::ops::{Descendants, describe_minted_edges};
use super::{BooleanError, Cell};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, HalfEdgeKey, VertexKey};
use crate::live::{linked, proven};
use crate::readback::edge_sides_of;

/// A joinable vertex's two edges: `gone` dies with the vertex, `kept`
/// becomes the joined edge; `he` is `gone`'s half-edge that ends at the
/// vertex, the one `kev` kills it through.
struct Join {
    he: HalfEdgeKey,
    gone: EdgeKey,
    kept: EdgeKey,
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

/// Every half-edge starting at each vertex.
fn starts<T: Real>(body: &Body<T>) -> BTreeMap<VertexKey, Vec<HalfEdgeKey>> {
    let mut out: BTreeMap<VertexKey, Vec<HalfEdgeKey>> = BTreeMap::new();
    for (k, h) in body.half_edges() {
        out.entry(h.start).or_default().push(k);
    }
    out
}

/// `w`'s join, if `w` is joinable (module docs). The half-edges in
/// `starts` were read out of the arena, so every hop past them is a link.
fn joinable<T: Real>(
    body: &Body<T>,
    w: VertexKey,
    starts: &BTreeMap<VertexKey, Vec<HalfEdgeKey>>,
) -> Option<Join> {
    let [h1, h2] = starts.get(&w)?.as_slice() else {
        return None;
    };
    let edge = |h: HalfEdgeKey| proven(&body.half_edges, h, EntityId::HalfEdge).edge;
    let (e1, e2) = (edge(*h1), edge(*h2));
    if e1 == e2 {
        return None;
    }
    let d1 = linked(
        &body.edges,
        e1,
        EntityId::Edge,
        EntityId::HalfEdge(*h1),
        "edge",
    );
    let d2 = linked(
        &body.edges,
        e2,
        EntityId::Edge,
        EntityId::HalfEdge(*h2),
        "edge",
    );
    let (s1, s2) = (edge_sides_of(body, e1, d1), edge_sides_of(body, e2, d2));
    let pair = |s: crate::readback::EdgeSides| {
        let (f, g) = s.faces();
        if f <= g { (f, g) } else { (g, f) }
    };
    let (f, g) = pair(s1);
    if f == g || pair(s2) != (f, g) {
        return None;
    }
    let (sf, sg) = s1.surfaces();
    let planar = |side: crate::readback::EdgeSide| {
        let face = proven(&body.faces, side.face, EntityId::Face);
        matches!(
            body.face_surface_linked(side.face, face),
            geom::Surface::Plane { .. }
        )
    };
    if sf == sg || !planar(s1.plus) || !planar(s1.minus) {
        return None;
    }
    // One carrier by structure: both edges certified as the
    // intersection of these two planes, which is one line (planes that
    // do not cross certify no intersection).
    let on_the_pair = |e: EdgeKey, d| {
        certified_line(body, e, d).is_some_and(|c| {
            matches!(
                c.description(),
                geom_brep::EdgeDescription::Intersection { s1, s2, .. }
                    if Body::<T>::cites_pair((*s1, *s2), sf, sg)
            )
        })
    };
    if !on_the_pair(e1, d1) || !on_the_pair(e2, d2) {
        return None;
    }
    Some(Join {
        he: body.mate(*h1)?,
        gone: e1,
        kept: e2,
    })
}

/// Every joinable vertex of `body`, in vertex-arena order: the vertices
/// an op's output must not hold (maximal edges). Planar only today: a
/// valence-2 vertex between curved faces is neither listed nor joined
/// (`work/fuse/curved-joinable-vertices-are-left-unjoined.md`).
pub fn joinable_vertices<T: Real>(body: &Body<T>) -> Vec<VertexKey> {
    let starts = starts(body);
    body.vertices()
        .map(|(k, _)| k)
        .filter(|&w| joinable(body, w, &starts).is_some())
        .collect()
}

/// **The output stage's join**: joins every joinable vertex
/// ([`joinable_vertices`]) of `body`, one at a time in vertex-arena
/// order until none is left, and writes each join's substitution rows
/// into `desc` (`w → kept`, `gone → kept`), so the op's one
/// [`super::ops::carry`] takes every record through its zips, its merge
/// and its joins together. Runs after the merge and its re-description,
/// before the records are carried. Returns the joins in the order made,
/// a later one's `gone` or `kept` possibly an earlier one's `kept`. A
/// join touches only planar faces, which store no pcurve rows
/// (`pcurves::chart_mints`), so it leaves nothing to re-mint; a curved
/// join (`work/fuse/curved-joinable-vertices-are-left-unjoined.md`)
/// brings what it needs.
///
/// # Errors
///
/// The kill's own refusal ([`BooleanError::Euler`]) or the
/// description's.
pub(super) fn join_stage<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    desc: &mut Descendants,
    band: Band,
    tol: Tol,
) -> Result<Vec<EdgeJoin>, BooleanError> {
    let mut joined = Vec::new();
    loop {
        let starts = starts(body);
        let Some((w, join)) = body
            .vertices()
            .map(|(k, _)| k)
            .find_map(|w| joinable(body, w, &starts).map(|j| (w, j)))
        else {
            return Ok(joined);
        };
        join_one(body, w, &join, band, tol)?;
        desc.substitute(Cell::Vertex(w), Cell::Edge(join.kept));
        desc.substitute(Cell::Edge(join.gone), Cell::Edge(join.kept));
        joined.push(EdgeJoin {
            vertex: w,
            gone: join.gone,
            kept: join.kept,
        });
    }
}

/// Joins at `w` (module docs): `gone` dies with `w` through `kev`, and
/// `kept`, re-based onto `gone`'s far end, is described from its faces.
fn join_one<T: Decide + crate::props::AtRestPolicy>(
    body: &mut Body<T>,
    w: VertexKey,
    join: &Join,
    band: Band,
    tol: Tol,
) -> Result<(), BooleanError> {
    let members = body
        .kev_merged_members(join.he)
        .map_err(BooleanError::Euler)?;
    let spec = members
        .iter()
        .find(|m| m.edge == join.kept)
        .map(|m| crate::EdgeCurveSpec::line_between(m.start, m.end))
        .ok_or(BooleanError::JoinDesync {
            what: "a joined edge is not the kill's merged member",
        })?;
    let killed = body
        .kev_describing(join.he, &[(join.kept, spec)], tol)
        .map_err(BooleanError::Euler)?;
    debug_assert_eq!(killed.killed_vertex, w, "the kill takes the joined vertex");
    describe_minted_edges(
        body,
        &[join.kept],
        &crate::merge_faces::MergeCoplanarOutcome::default(),
        band,
        tol,
    )
}

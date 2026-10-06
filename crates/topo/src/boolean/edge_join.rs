//! **The join**: a vertex of valence 2 whose two edges lie between the
//! same two planar faces is no corner, so the two edges are one edge
//! cut for no reason (`docs/DESIGN.md`, maximal edges). The join kills
//! the vertex and makes the two edges one, and carries every contact
//! record naming the three cells it replaces onto the edge it makes,
//! through the substitution door ([`super::ops::carry_in_place`]).
//!
//! Joinable is decided by topology alone: the two edges' face pairs
//! are one pair of distinct faces, both on `Plane` carriers of distinct
//! keys, so both edges lie on the one line the two planes meet in. A
//! valence-2 vertex between curved faces is outside this door: whether
//! its two edges share a carrier is a question of the intersection
//! branch, which no key answers yet.

use std::collections::BTreeMap;

use geom_core::{Band, Decide, Real, Tol};

use super::ops::{Descendants, carry_in_place, describe_minted_edges, structural_gate};
use super::{BooleanBody, BooleanError, Cell};
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

/// One join [`BooleanBody::join_edges`] made: `vertex` and `gone` are
/// dead, and `kept` holds both their interiors and its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EdgeJoin {
    /// The joined-away vertex.
    pub vertex: VertexKey,
    /// The edge killed with it.
    pub gone: EdgeKey,
    /// The edge that is the two edges now.
    pub kept: EdgeKey,
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
    let line = |e: EdgeKey, d| {
        body.edge_curve_linked(e, d)
            .certified()
            .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
    };
    if !line(e1, d1) || !line(e2, d2) {
        return None;
    }
    Some(Join {
        he: body.mate(*h1)?,
        gone: e1,
        kept: e2,
    })
}

/// Every joinable vertex of `body`, in vertex-arena order: the vertices
/// an op's output must not hold (maximal edges).
pub fn joinable_vertices<T: Real>(body: &Body<T>) -> Vec<VertexKey> {
    let starts = starts(body);
    body.vertices()
        .map(|(k, _)| k)
        .filter(|&w| joinable(body, w, &starts).is_some())
        .collect()
}

impl<T: Decide + crate::props::AtRestPolicy> BooleanBody<T> {
    /// Joins every joinable vertex ([`joinable_vertices`]), one at a
    /// time in vertex-arena order, and carries the contact records by
    /// substitution: a record naming a joined vertex or a killed edge
    /// names the joined edge. The joined edge is described from its two
    /// faces, as the boolean describes the edges it mints, the pcurve
    /// map is re-minted, and the joined body passes the at-rest gate the
    /// boolean's result gate ends in. Returns the result and the joins in the order made,
    /// a later one's `gone` or `kept` possibly an earlier one's `kept`.
    ///
    /// # Errors
    ///
    /// The kill's own refusal ([`BooleanError::Euler`]), the
    /// description's, the pcurve mint's, the door's, or the gate's.
    pub fn join_edges(self, tol: Tol) -> Result<(Self, Vec<EdgeJoin>), BooleanError> {
        let band = Band::linear(tol)?;
        let mut joined = Vec::new();
        let mut desc = Descendants::default();
        let mut finished = self.body.into_body();
        let mut body = finished.begin_surgery();
        loop {
            let starts = starts(&body);
            let Some((w, join)) = body
                .vertices()
                .map(|(k, _)| k)
                .find_map(|w| joinable(&body, w, &starts).map(|j| (w, j)))
            else {
                break;
            };
            join_one(&mut body, w, &join, band, tol)?;
            desc.substitute(Cell::Vertex(w), Cell::Edge(join.kept));
            desc.substitute(Cell::Edge(join.gone), Cell::Edge(join.kept));
            joined.push(EdgeJoin {
                vertex: w,
                gone: join.gone,
                kept: join.kept,
            });
        }
        if !joined.is_empty() {
            crate::pcurves::mint_pcurves(&mut body, tol)
                .map_err(|source| BooleanError::Pcurves { source })?;
        }
        body.sweep_and_close();
        let contacts = if joined.is_empty() {
            self.contacts
        } else {
            carry_in_place(&finished, &self.contacts, &desc)?
        };
        // The join kills no material, so the result's pieces stand; the
        // body passes the at-rest gate the boolean's own result gate ends
        // in.
        let body = T::gate_at_rest_kept(finished, tol)
            .map_err(|errors| BooleanError::ResultInvalid { errors })?;
        if body.outcome() == crate::AtRestOutcome::NotRunAtThisScalar {
            structural_gate(&body)?;
        }
        let out = Self {
            body,
            contacts,
            ..self
        };
        Ok((out, joined))
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

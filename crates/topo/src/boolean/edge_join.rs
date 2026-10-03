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

use super::ops::{Descendants, carry_in_place, describe_minted_edges};
use super::{BooleanBody, BooleanError, Cell};
use crate::body::Body;
use crate::entity::{EdgeKey, HalfEdgeKey, VertexKey};

/// A joinable vertex's two edges: `gone` dies with the vertex, `kept`
/// becomes the joined edge; `he` is `gone`'s half-edge that ends at the
/// vertex, the one `kev` kills it through.
struct Join {
    he: HalfEdgeKey,
    gone: EdgeKey,
    kept: EdgeKey,
}

/// Every half-edge starting at each vertex.
fn starts<T: Real>(body: &Body<T>) -> BTreeMap<VertexKey, Vec<HalfEdgeKey>> {
    let mut out: BTreeMap<VertexKey, Vec<HalfEdgeKey>> = BTreeMap::new();
    for (k, h) in body.half_edges() {
        out.entry(h.start).or_default().push(k);
    }
    out
}

/// `w`'s join, if `w` is joinable (module docs).
fn joinable<T: Real>(
    body: &Body<T>,
    w: VertexKey,
    starts: &BTreeMap<VertexKey, Vec<HalfEdgeKey>>,
) -> Option<Join> {
    let [h1, h2] = starts.get(&w)?.as_slice() else {
        return None;
    };
    let edge = |h: HalfEdgeKey| body.get_half_edge(h).map(|h| h.edge);
    let (e1, e2) = (edge(*h1)?, edge(*h2)?);
    if e1 == e2 {
        return None;
    }
    let sides = |e| crate::readback::edge_sides(body, e).ok();
    let (s1, s2) = (sides(e1)?, sides(e2)?);
    let pair = |s: crate::readback::EdgeSides| {
        let (f, g) = s.faces();
        if f <= g { (f, g) } else { (g, f) }
    };
    let (f, g) = pair(s1);
    if f == g || pair(s2) != (f, g) {
        return None;
    }
    let (sf, sg) = s1.surfaces();
    let planar = |k| matches!(body.get_surface(k), Some(geom::Surface::Plane { .. }));
    if sf == sg || !planar(sf) || !planar(sg) {
        return None;
    }
    let line = |e: EdgeKey| {
        body.get_edge(e)
            .and_then(|d| body.get_curve_geom(d.curve))
            .and_then(|c| c.certified())
            .is_some_and(|c| matches!(c.carrier(), geom::Curve3::Line { .. }))
    };
    if !line(e1) || !line(e2) {
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
    /// faces, as the boolean describes the edges it mints. Returns the
    /// vertices joined away.
    ///
    /// # Errors
    ///
    /// The kill's own refusal ([`BooleanError::Euler`]), the
    /// description's, or the door's.
    pub fn join_edges(&mut self, tol: Tol) -> Result<Vec<VertexKey>, BooleanError> {
        let band = Band::linear(tol)?;
        let mut joined = Vec::new();
        let mut desc = Descendants::default();
        loop {
            let starts = starts(&self.body);
            let Some((w, join)) = self
                .body
                .vertices()
                .map(|(k, _)| k)
                .find_map(|w| joinable(&self.body, w, &starts).map(|j| (w, j)))
            else {
                break;
            };
            self.join_one(w, &join, band, tol)?;
            desc.substitute(Cell::Vertex(w), Cell::Edge(join.kept));
            desc.substitute(Cell::Edge(join.gone), Cell::Edge(join.kept));
            joined.push(w);
        }
        if !joined.is_empty() {
            self.contacts = carry_in_place(&self.body, &self.contacts, &desc)?;
        }
        Ok(joined)
    }

    fn join_one(
        &mut self,
        w: VertexKey,
        join: &Join,
        band: Band,
        tol: Tol,
    ) -> Result<(), BooleanError> {
        let body = &mut self.body;
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
}

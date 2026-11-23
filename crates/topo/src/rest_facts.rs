//! **What tier 3 decided per entity, kept with its verdict**
//! (`docs/DESIGN.md` `[topology.tiers]`): the embedding question reads
//! these and decides none of them.
//!
//! - **An edge's class** ([`EdgeClass`]): what check 4's whole-edge fold
//!   read there. An edge check 4 exempts by kind (one beside a spline
//!   face) has none.
//! - **A face's run from each edge and vertex of its boundary**
//!   ([`geom_brep::BoundaryStations`]): how far the face runs from that
//!   cell, measured across it and from below. Two faces that share the
//!   cell part over the lesser of their two runs. An edge's are read by
//!   check 2, which reads every certified edge; a vertex's where more
//!   than three edges meet, the only vertices two faces can share
//!   without an edge on them.
//! - **A face's singular vertices**: each pole or apex of its chart that
//!   check 6 placed on a vertex of its boundary.
//!
//! Every fact is keyed by entity and is a fact of the shape up to
//! isometry, so a finished body moved by one keeps them as they are.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::Lever;
use slotmap::SecondaryMap;

use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, VertexKey};

/// **An edge's wedge as check 4 passed it**: the three verdicts a body
/// at rest can hold. A lamina, a mixed edge and an in-band reading
/// refuse at tier 3, so no kept class names them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EdgeClass {
    /// A corner along the whole edge, of this turn.
    Crease(geom_brep::Convexity),
    /// The wedge is π: the faces leave the edge in one tangent plane, to
    /// opposite sides.
    Seam,
    /// The wedge is 0 or 2π: the faces leave the edge in one tangent
    /// plane to one side, separating at second order.
    End(geom_brep::WedgeEnd),
}

impl EdgeClass {
    /// The class of a verdict check 4 passes; `None` for one it refuses.
    pub(crate) fn of(verdict: geom_brep::WedgeVerdict) -> Option<Self> {
        use geom_brep::WedgeVerdict as V;
        match verdict {
            V::Crease { convexity, .. } => Some(Self::Crease(convexity)),
            V::Seam { .. } => Some(Self::Seam),
            V::End { end, .. } => Some(Self::End(end)),
            V::Lamina | V::Mixed(_) | V::InBand(_) => None,
        }
    }
}

/// Tier 3's kept facts on one body (module docs), at the body's scalar:
/// a run is a length as the body states it, a value at a point scalar
/// and the family's enclosure at a box scalar.
#[derive(Clone, Debug)]
pub struct Tier3Facts<T> {
    classes: SecondaryMap<EdgeKey, EdgeClass>,
    edge_runs: SecondaryMap<FaceKey, BTreeMap<EdgeKey, Lever<T>>>,
    vertex_runs: SecondaryMap<FaceKey, BTreeMap<VertexKey, Lever<T>>>,
    singular: SecondaryMap<FaceKey, Vec<VertexKey>>,
}

impl<T> Default for Tier3Facts<T> {
    fn default() -> Self {
        Self {
            classes: SecondaryMap::new(),
            edge_runs: SecondaryMap::new(),
            vertex_runs: SecondaryMap::new(),
            singular: SecondaryMap::new(),
        }
    }
}

impl<T: Copy> Tier3Facts<T> {
    pub(crate) fn keep_class(&mut self, edge: EdgeKey, class: EdgeClass) {
        self.classes.insert(edge, class);
    }

    pub(crate) fn keep_edge_run(&mut self, face: FaceKey, edge: EdgeKey, run: Lever<T>) {
        if !self.edge_runs.contains_key(face) {
            self.edge_runs.insert(face, BTreeMap::new());
        }
        self.edge_runs[face].insert(edge, run);
    }

    pub(crate) fn keep_vertex_run(&mut self, face: FaceKey, vertex: VertexKey, run: Lever<T>) {
        if !self.vertex_runs.contains_key(face) {
            self.vertex_runs.insert(face, BTreeMap::new());
        }
        self.vertex_runs[face].insert(vertex, run);
    }

    pub(crate) fn keep_singular(&mut self, face: FaceKey, vertex: VertexKey) {
        if !self.singular.contains_key(face) {
            self.singular.insert(face, Vec::new());
        }
        let held = &mut self.singular[face];
        if !held.contains(&vertex) {
            held.push(vertex);
        }
    }

    /// `edge`'s class; `None` where check 4 read none.
    #[must_use]
    pub fn class(&self, edge: EdgeKey) -> Option<EdgeClass> {
        self.classes.get(edge).copied()
    }

    /// How far `face` runs from its `edge`; `None` for an edge off its
    /// boundary.
    #[must_use]
    pub fn edge_run(&self, face: FaceKey, edge: EdgeKey) -> Option<Lever<T>> {
        self.edge_runs.get(face)?.get(&edge).copied()
    }

    /// How far `face` runs from its `vertex`; `None` for a vertex off
    /// its boundary, or one no two faces share alone.
    #[must_use]
    pub fn vertex_run(&self, face: FaceKey, vertex: VertexKey) -> Option<Lever<T>> {
        self.vertex_runs.get(face)?.get(&vertex).copied()
    }

    /// `face`'s poles and apexes, as vertices of its boundary.
    #[must_use]
    pub fn singular(&self, face: FaceKey) -> &[VertexKey] {
        self.singular.get(face).map_or(&[], Vec::as_slice)
    }
}

/// **A body with tier 3's facts on it**: what the embedding question
/// takes ([`crate::clearance::embedded`]). It is made only by tier 3's
/// own battery over this body ([`crate::AtRestBody::geometric`] of a
/// finished body, [`crate::geometric`] of any other), so the facts are
/// never read against a body they were not decided on.
#[derive(Clone, Debug)]
pub struct Geometric<'b, T: geom_core::Real> {
    body: &'b Body<T>,
    facts: Arc<Tier3Facts<T>>,
}

impl<'b, T: geom_core::Real> Geometric<'b, T> {
    pub(crate) fn new(body: &'b Body<T>, facts: Arc<Tier3Facts<T>>) -> Self {
        Self { body, facts }
    }

    /// The body.
    #[must_use]
    pub fn body(&self) -> &'b Body<T> {
        self.body
    }

    /// Tier 3's facts on it.
    #[must_use]
    pub fn facts(&self) -> &Tier3Facts<T> {
        &self.facts
    }
}

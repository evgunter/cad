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

    /// **Keep each face's run from every vertex two faces can share
    /// with no edge on them**: those more than three face corners meet
    /// at. The corners about a vertex of a closed two-manifold form one
    /// cycle, each next to the two it shares an edge with, so two faces
    /// that share the vertex and no edge at it are not neighbours in a
    /// cycle of at least four. A vertex the rule misses keeps no run,
    /// and a pair near it alone then reads no extent and is not
    /// certified: the miss refuses, it does not permit.
    pub(crate) fn keep_fan_runs(&mut self, body: &Body<T>, wedges: &crate::WedgeReach<'_, T>)
    where
        T: geom_core::Decide,
    {
        let mut fans: SecondaryMap<VertexKey, Vec<FaceKey>> = SecondaryMap::new();
        for (he_key, he) in body.half_edges.iter() {
            if let Some(face) = body.face_of_half_edge(he_key) {
                if !fans.contains_key(he.start) {
                    fans.insert(he.start, Vec::new());
                }
                fans[he.start].push(face);
            }
        }
        for (vertex, faces) in fans.iter().filter(|(_, faces)| faces.len() > 3) {
            let Some(&at) = body
                .vertices
                .get(vertex)
                .and_then(|v| body.points.get(v.point))
            else {
                continue; // unreachable on tier-1 input
            };
            for &face in faces {
                self.keep_vertex_run(face, vertex, wedges.run_from_vertex(face, at));
            }
        }
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

    /// **A hand-built body with the runs tier 3 would keep on it, and
    /// nothing check 4 or check 6 decides**: each face's run from each
    /// edge of its boundary and from each vertex more than three edges
    /// meet at, through the doors tier 3 keeps them by
    /// ([`crate::WedgeReach`], [`Tier3Facts::keep_fan_runs`]); no class
    /// and no singular vertex. It states nothing a verdict could not. For a row that asks the question of construction state
    /// tier 3 refuses (two open sheets, a lamina) to exercise a reading
    /// on it: every separator then runs, as on an edge tier 3 read no
    /// class for.
    #[cfg(any(test, feature = "test-support"))]
    #[must_use]
    pub fn fixture(body: &'b Body<T>) -> Self
    where
        T: geom_core::Decide,
    {
        let wedges = crate::WedgeReach::new(body);
        let mut facts = Tier3Facts::default();
        for (edge, record) in body.edges() {
            let Some(curve) = body
                .curves
                .get(record.curve)
                .and_then(crate::null::CurveGeom::certified)
            else {
                continue;
            };
            for face in crate::edge_faces(body, edge) {
                facts.keep_edge_run(
                    face,
                    edge,
                    wedges.run_from_edge(face, curve.carrier(), curve.params()),
                );
            }
        }
        facts.keep_fan_runs(body, &wedges);
        Self::new(body, Arc::new(facts))
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod rows {
    use geom_core::Tol;

    /// **Tier 3 keeps what it decided on a brick.** Every edge of a
    /// `2 × 3 × 5` brick is a convex crease; each face runs from each of
    /// its edges the face's own width across it, so of the 24 face-edge
    /// pairs eight read each of 2, 3 and 5; no vertex has more than
    /// three edges, so no vertex run is kept, and no face has a singular
    /// vertex. Red if a class or run is dropped, if a run is the
    /// farther face's or read from the edge's middle, or if a fact is
    /// invented.
    #[test]
    fn tier_3_keeps_a_bricks_classes_and_runs() {
        let tol = Tol::witness();
        let brick: crate::Body<f64> =
            crate::test_support_fixtures::brick((0.0, 2.0), (0.0, 3.0), (0.0, 5.0), tol);
        let geometric = crate::geometric(&brick, tol).expect("a brick passes tier 3");
        let facts = geometric.facts();
        let mut runs = Vec::new();
        for (edge, _) in brick.edges() {
            assert_eq!(
                facts.class(edge),
                Some(crate::EdgeClass::Crease(geom_brep::Convexity::Convex)),
                "{edge:?}"
            );
            for face in crate::edge_faces(&brick, edge) {
                runs.push(facts.edge_run(face, edge).expect("a run").length());
            }
        }
        for width in [2.0, 3.0, 5.0] {
            let count = runs.iter().filter(|r| (**r - width).abs() < 1e-12).count();
            assert_eq!(count, 8, "runs of {width}: {runs:?}");
        }
        for (face, _) in brick.faces() {
            assert!(facts.singular(face).is_empty(), "{face:?}");
            for (vertex, _) in brick.vertices() {
                assert!(
                    facts.vertex_run(face, vertex).is_none(),
                    "{face:?} {vertex:?}"
                );
            }
        }
        // A run from an edge off the face is not kept.
        let (edge, _) = brick.edges().next().unwrap();
        let off = brick
            .faces()
            .map(|(f, _)| f)
            .find(|f| !crate::edge_faces(&brick, edge).contains(f))
            .unwrap();
        assert!(facts.edge_run(off, edge).is_none());
    }
}

//! Eager per-op name-table emission (N4; spec D4): mechanical, linear
//! passes over each op's output, driven ONLY by kernel birth data and
//! mint-time wiring facts (`Extruded`/`Revolved` maps, `SplitNaming`,
//! `BooleanNaming`, D5 provenance) plus combinatorial adjacency
//! between EMITTED anchors — never geometric matching. Where geometry
//! must discriminate (N2 fragment qualifiers), it enters as margined
//! predicate VERDICTS through `k_stats` ([`super::discriminate`]).
//!
//! Every emitter finishes with the TOTALITY check: every live
//! boundary entity of every output body must be named (a gap is a
//! typed [`NamingError`], surfacing kernel-emission holes loudly),
//! and injectivity is enforced by [`NameTable`] at insertion.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::Indeterminate;
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, VertexKey};

use super::role::{EntityKind, StableName};
use super::table::{DuplicateName, EntityKey, EntityRef, NameTable};
use crate::node::RecipeNodeId;

/// Typed failure of name emission (spec D4's loud assertions, as
/// in-band errors — this crate has no panic paths). Every variant is
/// an emission BUG or an honest in-band escalation, never a normal
/// modeling outcome.
#[derive(Debug)]
pub enum NamingError {
    /// A would-be duplicate name outside the tie path (the
    /// no-silent-aliasing guarantee N1 rests on), or a kind/entity
    /// collision at insertion.
    Duplicate {
        /// The colliding name.
        name: Box<StableName>,
    },
    /// A live boundary entity the emission failed to cover — a
    /// kernel-emission gap surfaced (spec D4: never guessed around).
    Unnamed {
        /// The uncovered entity's kind.
        kind: EntityKind,
        /// The output-body index it lives in.
        body: u32,
    },
    /// An upstream input's name table lacks an entity the emission
    /// needed (wiring bug: upstream tables are total by this same
    /// machinery).
    MissingUpstream {
        /// The upstream node whose table missed.
        node: RecipeNodeId,
    },
    /// A mint-time emission fact was inconsistent with the result
    /// body (kernel bug, loudly).
    Emission {
        /// What was inconsistent.
        what: &'static str,
    },
    /// An N2 discriminator margin escalated in-band (typed, never a
    /// silent pick — spec D3).
    Escalated {
        /// The named predicate.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: Indeterminate,
    },
}

impl From<DuplicateName> for NamingError {
    fn from(e: DuplicateName) -> Self {
        Self::Duplicate { name: e.name }
    }
}

/// A name at `node` with a single role segment.
pub(crate) fn name1(
    kind: EntityKind,
    node: RecipeNodeId,
    seg: super::role::RoleSeg,
) -> StableName {
    StableName {
        kind,
        node,
        path: vec![seg],
    }
}

/// An entity in output body `body`.
pub(crate) fn ent(body: u32, key: EntityKey) -> EntityRef {
    EntityRef { body, key }
}

/// The empty table (nodes with no output bodies: datum, profile,
/// declare, empty boolean).
pub(crate) fn empty() -> Arc<NameTable> {
    Arc::new(NameTable::new())
}

/// Wraps a pattern master's table per structural instance index
/// (A8/N1 `Instance(i)`): instance `i` holds the master's keys
/// verbatim (`transform_rigid` key-stability), body index `i`.
pub(crate) fn name_pattern(
    node: RecipeNodeId,
    master: &NameTable,
    n: i64,
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    for i in 0..n {
        let iu = u32::try_from(i).map_err(|_| NamingError::Emission {
            what: "pattern instance index exceeds u32",
        })?;
        for (name, entry) in master.iter() {
            let wrapped = StableName {
                kind: name.kind,
                node,
                path: vec![super::role::RoleSeg::Instance {
                    i: iu,
                    of: Box::new(name.clone()),
                }],
            };
            match entry {
                super::table::Entry::Unique(e) => {
                    t.insert(wrapped, ent(iu, e.key))?;
                }
                super::table::Entry::Tied(es) => {
                    t.insert_tied(wrapped, es.iter().map(|e| ent(iu, e.key)).collect())?;
                }
            }
        }
    }
    Ok(Arc::new(t))
}

/// Per-body topology walk products the emitters share: half-edge
/// incidence (vertex → edges, edge → faces) built in ONE deterministic
/// arena-order pass — combinatorial wiring facts, not inspection.
pub(crate) struct Incidence {
    /// Vertex → incident edges (sorted, deduplicated).
    pub vertex_edges: BTreeMap<VertexKey, Vec<EdgeKey>>,
    /// Edge → the (≤ 2) faces of its halves' loops.
    pub edge_faces: BTreeMap<EdgeKey, Vec<FaceKey>>,
}

impl Incidence {
    /// Builds the incidence maps for one body.
    pub(crate) fn of<T: geom_core::Real>(body: &Body<T>) -> Result<Self, NamingError> {
        let bug = || NamingError::Emission {
            what: "incidence walk: dangling half-edge reference",
        };
        let mut vertex_edges: BTreeMap<VertexKey, Vec<EdgeKey>> = BTreeMap::new();
        let mut edge_faces: BTreeMap<EdgeKey, Vec<FaceKey>> = BTreeMap::new();
        for (_, he) in body.half_edges() {
            vertex_edges.entry(he.start).or_default().push(he.edge);
            let face = body.get_loop(he.parent_loop).ok_or_else(bug)?.face;
            edge_faces.entry(he.edge).or_default().push(face);
        }
        for v in vertex_edges.values_mut() {
            v.sort_unstable();
            v.dedup();
        }
        for f in edge_faces.values_mut() {
            f.sort_unstable();
            f.dedup();
        }
        Ok(Self {
            vertex_edges,
            edge_faces,
        })
    }
}

/// The unique edge shared by face `f` and face `g` (cap–wall rims:
/// derived combinatorially from emitted anchors). Errors if absent or
/// ambiguous — an emission-fact inconsistency.
pub(crate) fn unique_shared_edge<T: geom_core::Real>(
    body: &Body<T>,
    f: FaceKey,
    g: FaceKey,
) -> Result<EdgeKey, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let mut found: Option<EdgeKey> = None;
    for he in face_half_edges(body, f)? {
        let mate = body
            .mate(he)
            .ok_or_else(|| bug("shared-edge walk: unmated half-edge"))?;
        let mate_he = body
            .get_half_edge(mate)
            .ok_or_else(|| bug("shared-edge walk: dangling mate"))?;
        let other = body
            .get_loop(mate_he.parent_loop)
            .ok_or_else(|| bug("shared-edge walk: dangling loop"))?
            .face;
        if other == g {
            let edge = mate_he.edge;
            if found.is_some_and(|e| e != edge) {
                return Err(bug("shared-edge walk: two shared edges where one expected"));
            }
            found = Some(edge);
        }
    }
    found.ok_or_else(|| bug("shared-edge walk: no shared edge"))
}

/// All half-edges of a face (outer loop + rings), deterministic
/// order.
pub(crate) fn face_half_edges<T: geom_core::Real>(
    body: &Body<T>,
    f: FaceKey,
) -> Result<Vec<HalfEdgeKey>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let face = body
        .get_face(f)
        .ok_or_else(|| bug("face walk: dangling face"))?;
    let mut out = Vec::new();
    for l in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        match body
            .get_loop(l)
            .ok_or_else(|| bug("face walk: dangling loop"))?
            .boundary
        {
            topo::LoopBoundary::Empty { .. } => {}
            topo::LoopBoundary::Cycle { first } => {
                out.extend(
                    body.loop_cycle(first)
                        .ok_or_else(|| bug("face walk: unwalkable loop"))?,
                );
            }
        }
    }
    Ok(out)
}

/// The two endpoint vertices of an edge.
pub(crate) fn edge_ends<T: geom_core::Real>(
    body: &Body<T>,
    e: EdgeKey,
) -> Result<(VertexKey, VertexKey), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let edge = body
        .get_edge(e)
        .ok_or_else(|| bug("edge ends: dangling edge"))?;
    let start = body
        .get_half_edge(edge.he_plus)
        .ok_or_else(|| bug("edge ends: dangling he_plus"))?
        .start;
    let end = body
        .half_edge_end(edge.he_plus)
        .ok_or_else(|| bug("edge ends: he_plus has no end"))?;
    Ok((start, end))
}

/// TOTALITY (spec D4): every live face/edge/vertex of `body` (output
/// body index `ix`) is covered by the table, plus the body row
/// itself.
pub(crate) fn check_total<T: geom_core::Real>(
    table: &NameTable,
    body: &Body<T>,
    ix: u32,
) -> Result<(), NamingError> {
    let miss = |kind| NamingError::Unnamed { kind, body: ix };
    if table.name_of(&ent(ix, EntityKey::Body)).is_none() {
        return Err(miss(EntityKind::Body));
    }
    for (f, _) in body.faces() {
        if table.name_of(&ent(ix, EntityKey::Face(f))).is_none() {
            return Err(miss(EntityKind::Face));
        }
    }
    for (e, _) in body.edges() {
        if table.name_of(&ent(ix, EntityKey::Edge(e))).is_none() {
            return Err(miss(EntityKind::Edge));
        }
    }
    for (v, _) in body.vertices() {
        if table.name_of(&ent(ix, EntityKey::Vertex(v))).is_none() {
            return Err(miss(EntityKind::Vertex));
        }
    }
    Ok(())
}

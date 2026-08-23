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

// #380: the refusal must NAME its subject. Every variant above is
// diagnosed precisely at the emitter — which name collided, which
// entity went uncovered, which upstream table missed, which
// consistency fact broke — and until this impl existed all of that
// died at the `EvalError::Naming` boundary, whose `Display` said only
// "name emission failed". Callers (Python's typed exception text
// included) had to BISECT scenes to relocate a wall the emitter had
// already located. Unlike the kernel-refusal payloads that ride the
// op variants unaltered (`NodeErrorKind`'s Display note, D2), this is
// editor-core's OWN error: rendering it IS the op's vocabulary, and
// there is no other path by which it reaches a human.
impl core::fmt::Display for NamingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Duplicate { name } => write!(
                f,
                "the name {name:?} was minted twice — names alias silently only over the \
                 kernel's dead body"
            ),
            Self::Unnamed { kind, body } => write!(
                f,
                "a live {kind:?} of output body {body} was left unnamed — a kernel-emission \
                 gap, not a naming choice"
            ),
            Self::MissingUpstream { node } => write!(
                f,
                "the name table of upstream node {} lacks an entity the emission needed",
                node.0
            ),
            Self::Emission { what } => write!(
                f,
                "a mint-time emission fact was inconsistent with the result body: {what}"
            ),
            Self::Escalated { predicate, source } => write!(
                f,
                "the discriminator {predicate} escalated (in-band indeterminacy): {source}"
            ),
        }
    }
}

impl From<DuplicateName> for NamingError {
    fn from(e: DuplicateName) -> Self {
        Self::Duplicate { name: e.name }
    }
}

/// A name at `node` with a single role segment.
pub(crate) fn name1(kind: EntityKind, node: RecipeNodeId, seg: super::role::RoleSeg) -> StableName {
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
///
/// **The wrapping is uniform** (ASM-2K D-2): `Instance(i)` wraps EVERY
/// name of the master, whatever the master's body holds. A master with
/// several SOLIDS is one such master and is admitted — its names are
/// already distinct within it (derivation paths tell its solids apart),
/// and one qualifier per instance carries that distinctness across the
/// instances; no per-solid sub-index exists, because which solid a name
/// denotes is read off the name's own derivation, never off the
/// instance qualifier.
///
/// What stays refused is narrower than that (review R7): a master with
/// MULTIPLE output BODIES. Body index here IS the instance index, so
/// admitting one would need a ratified instance×body layout — and
/// nothing can produce one, `body_operand` refusing multi-body inputs
/// upstream. Totality is checked against every instance body.
pub(crate) fn name_pattern<T: geom_core::Real>(
    node: RecipeNodeId,
    master: &NameTable,
    n: i64,
    instances: &[Arc<Body<T>>],
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
                    if e.body != 0 {
                        return Err(NamingError::Emission {
                            what: "pattern of a multi-OUTPUT-BODY master — deferred (typed); multi-SOLID masters are admitted",
                        });
                    }
                    t.insert(wrapped, ent(iu, e.key))?;
                }
                super::table::Entry::Tied(es) => {
                    if es.iter().any(|e| e.body != 0) {
                        return Err(NamingError::Emission {
                            what: "pattern of a multi-OUTPUT-BODY master — deferred (typed); multi-SOLID masters are admitted",
                        });
                    }
                    t.insert_tied(wrapped, es.iter().map(|e| ent(iu, e.key)).collect())?;
                }
            }
        }
    }
    for (i, body) in instances.iter().enumerate() {
        check_total(&t, body, u32::try_from(i).unwrap_or(u32::MAX))?;
    }
    Ok(Arc::new(t))
}

/// Names a [`crate::node::Node::PlacedUnion`]'s ONE output body
/// (GROUP-BOOLEAN-DESIGN, ratified A′).
///
/// The vocabulary does NOT grow: per-instance discrimination is
/// [`name_pattern`]'s own `Instance(i)` wrapper, segment for segment,
/// so "instance 7's cavity face" is the same one-row selector on a
/// fused group as on an unfused pattern. What differs is the TARGET —
/// a PlacedUnion emits one body, so every instance's rows land at body
/// index 0, re-keyed through that instance's graft bridge (`bridges[i]`
/// is the correspondence the graft of instance `i` established).
///
/// The BODY name is minted here rather than wrapped, for
/// [`name_in_part`]'s reason: the fused body is not the body any
/// instance-local name denotes, so it is this node's own output body,
/// named like every other body-producing op's. The prototype's own
/// body-kind rows therefore do not carry — they have nothing left to
/// point at, exactly as the product gather rules.
pub(crate) fn name_placed_union<T: geom_core::Real>(
    node: RecipeNodeId,
    master: &NameTable,
    bridges: &[topo::GraftKeys],
    fused: &Body<T>,
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, super::role::RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    for (i, keys) in bridges.iter().enumerate() {
        let iu = u32::try_from(i).map_err(|_| NamingError::Emission {
            what: "placed-union instance index exceeds u32",
        })?;
        let mapped = |key: EntityKey| -> Option<EntityKey> {
            match key {
                EntityKey::Body => None,
                EntityKey::Face(f) => keys.face(f).map(EntityKey::Face),
                EntityKey::Edge(e) => keys.edge(e).map(EntityKey::Edge),
                EntityKey::Vertex(v) => keys.vertex(v).map(EntityKey::Vertex),
            }
        };
        for (name, entry) in master.iter() {
            let wrapped = StableName {
                kind: name.kind,
                node,
                path: vec![super::role::RoleSeg::Instance {
                    i: iu,
                    of: Box::new(name.clone()),
                }],
            };
            // The prototype's table is a single-output-body table
            // (`body_operand` refuses multi-body inputs upstream), so a
            // row at any other index is a bug — surfaced, not dropped.
            let rows: Vec<EntityRef> = match entry {
                super::table::Entry::Unique(e) => vec![*e],
                super::table::Entry::Tied(es) => es.clone(),
            };
            if rows.iter().any(|e| e.body != 0) {
                return Err(NamingError::Emission {
                    what: "placed union of a multi-OUTPUT-BODY prototype — deferred (typed); multi-SOLID prototypes are admitted",
                });
            }
            let moved: Vec<EntityRef> = rows
                .into_iter()
                .filter_map(|e| mapped(e.key).map(|key| ent(0, key)))
                .collect();
            match moved.len() {
                0 => {}
                1 => t.insert(wrapped, moved[0])?,
                _ => t.insert_tied(wrapped, moved)?,
            }
        }
    }
    check_total(&t, fused, 0)?;
    Ok(Arc::new(t))
}

/// Wraps an instantiated part's product table under the instance
/// (ASM-2A D-4: the GQ4 wrapper composed with N1–N7).
///
/// Every stable name of the referenced part's product body is re-minted
/// as `InPart(part-local name)` at the INSTANTIATE node, so two
/// instances of one part have wholly distinct names (their nodes
/// differ) while a part-document edit that breaks a local name surfaces
/// through the unchanged N1–N7 diagnosis ladder — the wrapped name is
/// still in there, structurally.
///
/// Keys hold verbatim: `product_named` keyed the part table onto the
/// product body, and `transform_rigid` is key-stable, so the placed
/// copy's arena answers to exactly those keys.
///
/// The BODY name is minted here rather than wrapped: the part's product
/// is not a body any part-local name denotes (see
/// [`crate::product::product_named`]), and an instance's placed body is
/// this node's own output, named like every other body-producing op's.
pub(crate) fn name_in_part<T: geom_core::Real>(
    node: RecipeNodeId,
    part: &NameTable,
    placed: &Body<T>,
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, super::role::RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;
    for (name, entry) in part.iter() {
        let wrapped = StableName {
            kind: name.kind,
            node,
            path: vec![super::role::RoleSeg::InPart {
                of: Box::new(name.clone()),
            }],
        };
        // The part's table is the PRODUCT's: one body, index 0. A row
        // anywhere else is a gather bug, surfaced rather than dropped.
        let misplaced = || NamingError::Emission {
            what: "an instantiated part's product table names a body other than the product",
        };
        match entry {
            super::table::Entry::Unique(e) => {
                if e.body != 0 {
                    return Err(misplaced());
                }
                t.insert(wrapped, ent(0, e.key))?;
            }
            super::table::Entry::Tied(es) => {
                if es.iter().any(|e| e.body != 0) {
                    return Err(misplaced());
                }
                t.insert_tied(wrapped, es.iter().map(|e| ent(0, e.key)).collect())?;
            }
        }
    }
    check_total(&t, placed, 0)?;
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

/// **ASM-2K D-2 rows**: the uniform `Instance(i)` wrapping over a
/// master whose body holds SEVERAL SOLIDS — the shape an assembly's
/// instantiated part arrives in (`topo::graft_disjoint_all`). Kernel
/// level on purpose: no document node produces a multi-solid body yet
/// (solids are minted inside `topo` alone), so the master here is
/// built at the door that will feed one.
#[cfg(test)]
mod pattern_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::Tol;
    use std::collections::BTreeSet;
    use std::sync::Arc;

    use geom_core::{Affine3, Vec3};
    use topo::Body;

    use super::*;
    use crate::names::emit_sweep::name_extrude;
    use crate::names::role::RoleSeg;
    use crate::names::table::Entry;
    use crate::node::RecipeNodeId;
    use profile::RawLoop;

    /// A unit cube at `dx`, with its extrude's own name table.
    fn cube(node: RecipeNodeId, dx: f64) -> (Body<f64>, Arc<NameTable>) {
        let plane = profile::SketchPlane::from_frame(
            geom_core::Point3::new(dx, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| geom_core::Point2::new(x, y)),
        );
        let prof = profile::Profile::new(plane, vec![square])
            .validate(geom_core::Tol::witness())
            .unwrap();
        let built =
            sweep::extrude(&prof, sweep::Extrusion::Distance(1.0_f64), Tol::witness()).unwrap();
        let table = name_extrude(node, &built).unwrap();
        (built.body, table)
    }

    /// A master body holding TWO solids, and a table that covers it:
    /// solid 0 wears its own extrude's names (node 1), solid 1 wears
    /// node 2's, re-keyed kind-wise in arena order onto the grafted
    /// copies. Two derivations, so the master's names are distinct
    /// within the master — which is the premise the wrapping rests on.
    /// WHICH grafted face wears which of node 2's names is immaterial
    /// to these rows (they pin the wrapping); minting a faithful key
    /// bridge across a graft is the consumer's job, not this door's.
    fn two_solid_master() -> (Body<f64>, NameTable) {
        let (mut body, a) = cube(RecipeNodeId(1), 0.0);
        let (second, b) = cube(RecipeNodeId(2), 10.0);
        let was: (BTreeSet<_>, BTreeSet<_>, BTreeSet<_>) = (
            body.faces().map(|(k, _)| k).collect(),
            body.edges().map(|(k, _)| k).collect(),
            body.vertices().map(|(k, _)| k).collect(),
        );
        topo::graft_disjoint(&mut body, &second, Tol::witness()).expect("a two-solid master");
        let fresh_f: Vec<_> = body
            .faces()
            .map(|(k, _)| k)
            .filter(|k| !was.0.contains(k))
            .collect();
        let fresh_e: Vec<_> = body
            .edges()
            .map(|(k, _)| k)
            .filter(|k| !was.1.contains(k))
            .collect();
        let fresh_v: Vec<_> = body
            .vertices()
            .map(|(k, _)| k)
            .filter(|k| !was.2.contains(k))
            .collect();

        let mut t = NameTable::new();
        for (name, entry) in a.iter() {
            match entry {
                Entry::Unique(e) => t.insert(name.clone(), *e).unwrap(),
                Entry::Tied(es) => t.insert_tied(name.clone(), es.clone()).unwrap(),
            }
        }
        let (mut fi, mut ei, mut vi) = (0, 0, 0);
        for (name, entry) in b.iter() {
            let Entry::Unique(e) = entry else {
                panic!("the fixture's extrude ties nothing");
            };
            // The aggregate is ONE body: it keeps solid 0's body name.
            let key = match e.key {
                EntityKey::Body => continue,
                EntityKey::Face(_) => {
                    fi += 1;
                    EntityKey::Face(fresh_f[fi - 1])
                }
                EntityKey::Edge(_) => {
                    ei += 1;
                    EntityKey::Edge(fresh_e[ei - 1])
                }
                EntityKey::Vertex(_) => {
                    vi += 1;
                    EntityKey::Vertex(fresh_v[vi - 1])
                }
            };
            t.insert(name.clone(), ent(0, key)).unwrap();
        }
        assert_eq!(
            (fi, ei, vi),
            (fresh_f.len(), fresh_e.len(), fresh_v.len()),
            "the second solid is covered exactly"
        );
        (body, t)
    }

    /// The pattern's instance bodies for a master and a z-step —
    /// `wire_pattern`'s own shape: instance 0 verbatim, the rest
    /// through the key-stable placement door.
    fn instances(master: &Body<f64>, n: i64, step: f64) -> Vec<Arc<Body<f64>>> {
        (0..n)
            .map(|i| {
                Arc::new(if i == 0 {
                    master.clone()
                } else {
                    topo::transform_rigid(
                        master,
                        &Affine3::translation(Vec3::new(0.0, 0.0, step * i as f64)),
                        Tol::witness(),
                    )
                    .unwrap()
                })
            })
            .collect()
    }

    /// **Row 4.** A pattern of a two-solid master is admitted: N×2
    /// solids, N × the master's census, and every stable name distinct
    /// — one `Instance(i)` over the master's own name, no per-solid
    /// sub-index anywhere in the path.
    #[test]
    fn a_multi_solid_master_patterns_with_uniform_instance_wrapping() {
        let (master_body, master) = two_solid_master();
        assert_eq!(master_body.solids().count(), 2, "a two-solid master");
        let n = 3_i64;
        let bodies = instances(&master_body, n, 5.0);
        let node = RecipeNodeId(9);
        let t = name_pattern(node, &master, n, &bodies).expect("a multi-solid master is admitted");

        let times = usize::try_from(n).unwrap();
        assert_eq!(t.len(), master.len() * times, "census: N × the master's");
        assert_eq!(
            bodies.iter().map(|b| b.solids().count()).sum::<usize>(),
            2 * times,
            "N × 2 solids"
        );
        assert_eq!(
            bodies.iter().map(|b| b.faces().count()).sum::<usize>(),
            master_body.faces().count() * times
        );

        let mut seen = BTreeSet::new();
        for (name, entry) in t.iter() {
            assert!(seen.insert(name.clone()), "a name repeated: {name:?}");
            assert_eq!(name.node, node);
            assert_eq!(name.path.len(), 1, "one qualifier, not one per solid");
            let RoleSeg::Instance { i, of } = &name.path[0] else {
                panic!("not wrapped: {name:?}");
            };
            assert!(master.lookup(of).is_some(), "wraps a master name");
            if let Entry::Unique(e) = entry {
                assert_eq!(e.body, *i, "the instance index IS the body index");
            }
        }
        assert_eq!(seen.len(), master.len() * times, "all distinct");
    }

    /// **Row 5.** Names of instance `i` resolve to instance `i`'s
    /// geometry: the wrapped name looks up to `(body i, the master's
    /// key)`, the key is live in that instance (placement is
    /// key-stable), and the entity it names sits where instance `i`
    /// sits — for BOTH of the master's solids.
    #[test]
    fn instance_i_names_resolve_to_instance_i_geometry() {
        let (master_body, master) = two_solid_master();
        let (n, step) = (3_i64, 5.0);
        let bodies = instances(&master_body, n, step);
        let node = RecipeNodeId(9);
        let t = name_pattern(node, &master, n, &bodies).expect("admitted");

        let mut checked = 0;
        for i in 0..n {
            let iu = u32::try_from(i).unwrap();
            for (name, entry) in master.iter() {
                let Entry::Unique(e) = entry else { continue };
                let wrapped = StableName {
                    kind: name.kind,
                    node,
                    path: vec![RoleSeg::Instance {
                        i: iu,
                        of: Box::new(name.clone()),
                    }],
                };
                assert_eq!(t.lookup(&wrapped), Some(&Entry::Unique(ent(iu, e.key))));
                assert_eq!(t.name_of(&ent(iu, e.key)), Some(&wrapped));
                let EntityKey::Vertex(v) = e.key else {
                    continue;
                };
                let at = |b: &Body<f64>| {
                    let p = b.get_vertex(v).expect("key-stable placement").point;
                    *b.get_point(p).unwrap()
                };
                let (m, inst) = (at(&master_body), at(&bodies[usize::try_from(i).unwrap()]));
                assert_eq!(inst.x, m.x, "unmoved across the pattern direction");
                assert_eq!(inst.z, m.z + step * i as f64, "instance {i}'s geometry");
                checked += 1;
            }
        }
        assert_eq!(checked, 16 * 3, "both solids' 8 vertices, every instance");
    }

    /// The refusal that STAYS (and is not the multi-solid one): a
    /// master with several output BODIES has no ratified instance×body
    /// layout, so it refuses typed rather than conflating halves.
    #[test]
    fn a_multi_output_body_master_still_refuses_typed() {
        let (body, a) = cube(RecipeNodeId(1), 0.0);
        let mut master = NameTable::new();
        for (name, entry) in a.iter() {
            if let Entry::Unique(e) = entry {
                master.insert(name.clone(), ent(1, e.key)).unwrap();
            }
        }
        let err = name_pattern(RecipeNodeId(9), &master, 2, &[Arc::new(body)])
            .expect_err("a multi-output-body master must refuse");
        assert!(
            format!("{err:?}").contains("multi-OUTPUT-BODY"),
            "typed, and about bodies: {err:?}"
        );
    }
}

/// **#380**: the diagnostic must SURVIVE the `EvalError` boundary. The
/// bug these rows close was not a missing diagnosis — every emitter
/// already refused precisely — but a rendering that threw the
/// diagnosis away, leaving `Display` consumers (Python's typed
/// exception text among them) with "name emission failed" and a
/// bisect. Substring pins, per the [`geom_core::COINCIDENCE_RECOURSE`]
/// convention: prose may be reworded, the SUBJECT may not vanish.
#[cfg(test)]
mod display_tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Band, MarginDiag};

    use super::*;
    use crate::eval::NodeErrorKind;
    use crate::names::role::RoleSeg;

    fn escalation() -> Indeterminate {
        Indeterminate {
            margin: MarginDiag::Invalid,
            band: Band::new(1e-9, 1e-6).unwrap(),
            predicate: Some("side_of_plane"),
        }
    }

    /// Every variant names WHICH entity/node/fact it refused on.
    #[test]
    fn every_variant_names_its_subject() {
        let name = StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(7),
            path: vec![RoleSeg::Cap(super::super::role::CapEnd::Top)],
        };
        let rows: Vec<(NamingError, Vec<&str>)> = vec![
            (
                NamingError::Duplicate {
                    name: Box::new(name),
                },
                vec!["Face", "7", "Cap"],
            ),
            (
                NamingError::Unnamed {
                    kind: EntityKind::Edge,
                    body: 3,
                },
                vec!["Edge", "3"],
            ),
            (
                NamingError::MissingUpstream {
                    node: RecipeNodeId(11),
                },
                vec!["11"],
            ),
            (
                // A refusal that still EXISTS: LIB-G14 retired the
                // tied-upstream one this row used to sample (ties
                // propagate now), and a sample payload that greps to
                // nothing would outlive its own subject.
                NamingError::Emission {
                    what: "section face classified On",
                },
                vec!["section face classified On"],
            ),
            (
                NamingError::Escalated {
                    predicate: "side_of_plane",
                    source: escalation(),
                },
                vec!["side_of_plane"],
            ),
        ];
        for (err, wanted) in rows {
            let shown = err.to_string();
            for w in wanted {
                assert!(shown.contains(w), "{err:?} rendered without {w:?}: {shown}");
            }
        }
    }

    /// The boundary the issue was actually about: wrapping the refusal
    /// in `NodeErrorKind` must not swallow it.
    #[test]
    fn the_eval_boundary_carries_the_diagnostic_through() {
        let shown = NodeErrorKind::Naming(NamingError::Emission {
            what: "split naming across boolean-minted faces",
        })
        .to_string();
        assert!(
            shown.contains("split naming across boolean-minted faces"),
            "the emitter's diagnostic must reach Display consumers: {shown}"
        );
    }
}

//! Split/boolean name emission (spec D2/D3): descent-driven — every
//! result entity is chased to its operand parent through the kernels'
//! mint-time rows (`SplitNaming`, `BooleanNaming`, D5 `SplitEdge`
//! provenance), then named as pass-through, `FromA`/`FromB`,
//! fragment (with N2 qualifiers), seam, section, or merged. Nothing
//! is matched; unresolvable descent is a typed error.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom::Surface;
use geom_core::{Decide, Point3, Vec3};
use topo::splitting::{PlaneSide, SplitNaming};
use topo::{Body, EdgeKey, FaceKey, Provenance, VertexKey};

use super::discriminate::{Extent, band, order_along, side_of_face};
use super::emit::{
    Incidence, NamingError, edge_ends, ent, face_half_edges, name1, unique_shared_edge,
};
use super::role::{EntityKind, Qualifier, RoleSeg, SplitHalf, StableName};
use super::table::{EntityKey, Entry, NameTable};
use crate::node::RecipeNodeId;
use geom_core::Tol;

/// One split-side body under naming.
struct Side<'a, T: Decide> {
    body: &'a Body<T>,
    ix: u32,
    half: SplitHalf,
}

/// The operand-face plane, **oriented outward** (result carriers are
/// the N2 references).
///
/// S10 CATEGORY A: the returned normal is the face's outward normal,
/// `Face::sense_sign() · chart_normal`, not the raw chart normal.
/// Every consumer uses the direction as an *oriented reference* whose
/// sign lands in a stable name — [`side_of_face`] turns it into a
/// `Qualifier::SideOf` verdict vector, and `n_a × n_b` orients the
/// carrier [`order_along`] ranks fragments along
/// (`Qualifier::OrderAlong`). Reading the chart normal raw would let a
/// sense flip silently swap Positive↔Negative and reverse every rank,
/// renaming fragments that did not move: an N4 covariance break, since
/// a face's orientation sense is part of the geometry names are
/// covariant *with*, not a private encoding detail the naming layer
/// may ignore. The sign is exact structure (a `bool` selecting `±1`),
/// so no new numeric decision enters here, and every face this build
/// mints has `sense: true` — the multiply is `· 1` and no name moves.
fn face_plane<T: Decide>(body: &Body<T>, f: FaceKey) -> Result<(Point3<T>, Vec3<T>), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let face = body
        .get_face(f)
        .ok_or_else(|| bug("face_plane: dangling"))?;
    match body
        .get_surface(face.surface)
        .ok_or_else(|| bug("face_plane: dangling surface"))?
    {
        Surface::Plane { origin, normal, .. } => Ok((*origin, *normal * face.sense_sign())),
        _ => Err(bug("face_plane: non-planar carrier in planar pipeline")),
    }
}

/// A face's extent along `dir` (probe values stay here; only order
/// enters names).
fn face_extent<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
    dir: Vec3<T>,
) -> Result<Extent<T>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let mut min: Option<T> = None;
    let mut max: Option<T> = None;
    for he in face_half_edges(body, f)? {
        let v = body
            .get_half_edge(he)
            .ok_or_else(|| bug("face_extent: dangling half-edge"))?
            .start;
        let p = *body
            .get_vertex(v)
            .and_then(|vd| body.get_point(vd.point))
            .ok_or_else(|| bug("face_extent: vertex without point"))?;
        let t = Vec3::new(p.x, p.y, p.z).dot(dir);
        min = Some(match min {
            None => t,
            Some(m) => m.min(t),
        });
        max = Some(match max {
            None => t,
            Some(m) => m.max(t),
        });
    }
    match (min, max) {
        (Some(min), Some(max)) => Ok(Extent { min, max }),
        _ => Err(bug("face_extent: face has no vertices")),
    }
}

/// An entity's upstream name, plus whether the upstream entry is an N2
/// TIE.
///
/// B1 (ratified, #512): a tie PROPAGATES — naming a tie is fine (N2);
/// only *referencing* one is `Ambiguous`. This mirrors the three
/// emitters that already do it (`name_pattern`, `name_in_part`,
/// `graft_names`), so a tie anywhere in an operand table no longer
/// refuses the whole downstream op.
#[derive(Clone)]
struct Upstream {
    /// The operand-table name (identical for every tied candidate).
    name: StableName,
    /// True iff that name's entry is `Entry::Tied`.
    tied: bool,
}

/// The upstream name of an entity. A MISSING row is still loud (the
/// upstream tables are total by this same machinery), and so is a
/// table whose two directions disagree — after B1, that is the ONE
/// remaining condition genuinely needing a unique upstream, because no
/// candidate list exists to propagate.
///
/// It has no executable test row ON PURPOSE, and the reason is a
/// property rather than an omission: the condition is unconstructible
/// through `NameTable`'s public API — `insert`/`insert_tied` write both
/// directions together and there is no removal door, so no caller can
/// reach a state where `name_of` answers and `lookup` does not. The
/// LIB-G14 review confirmed this independently (MINOR-1) and recorded
/// the prose as the faithful reading. The arm stays because the
/// invariant is the emitter's to assert, not to assume.
fn upstream_name(
    table: &NameTable,
    node: RecipeNodeId,
    e: super::table::EntityRef,
) -> Result<Upstream, NamingError> {
    let name = table
        .name_of(&e)
        .ok_or(NamingError::MissingUpstream { node })?;
    let tied = match table.lookup(name) {
        Some(Entry::Unique(_)) => false,
        Some(Entry::Tied(_)) => true,
        None => {
            return Err(NamingError::Emission {
                what: "an operand name table's forward and reverse directions disagree",
            });
        }
    };
    Ok(Upstream {
        name: name.clone(),
        tied,
    })
}

/// Rows deferred because their name descends from an N2 tie (B1) — or,
/// for `SectionEdge`, because the op itself mints one (A2).
///
/// Upstream candidates that were equally admissible stay equally
/// admissible downstream, so their same-named descendants MERGE into
/// one entry at flush: `Tied` when ≥ 2 survive, narrowed back to
/// `Unique` when exactly one does (the `graft_names` shape). Rows that
/// do NOT descend from a tie keep going through `NameTable::insert`
/// directly, so a genuine aliasing bug is still a typed `Duplicate` —
/// and so is a tie-descended name colliding with a strict one, since
/// the flush inserts into the same table.
///
/// Narrowing means a WRAPPED name can come out `Unique` here while the
/// upstream name it wraps stays `Tied` (review NOTE-2). That is the
/// ratified `graft_names` semantics, not laundering: the op genuinely
/// separated the candidates, and the upstream table is untouched.
#[derive(Default)]
struct TieRows(BTreeMap<StableName, Vec<super::table::EntityRef>>);

impl TieRows {
    /// Defers one row.
    fn push(&mut self, name: StableName, e: super::table::EntityRef) {
        self.0.entry(name).or_default().push(e);
    }

    /// Drains the deferred rows into the table. Called at each stage
    /// boundary, because later stages read the names earlier stages
    /// wrote (the boolean vertex pass reads its incident EDGE names).
    fn flush(&mut self, t: &mut NameTable) -> Result<(), NamingError> {
        for (name, ents) in core::mem::take(&mut self.0) {
            match ents.as_slice() {
                [one] => t.insert(name, *one)?,
                _ => t.insert_tied(name, ents)?,
            }
        }
        Ok(())
    }
}

/// Inserts a downstream row: strict when its upstream name was unique,
/// deferred into the tie lane when it descends from a tie (B1).
fn put(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    name: StableName,
    e: super::table::EntityRef,
) -> Result<(), NamingError> {
    if from_tie {
        tie.push(name, e);
        Ok(())
    } else {
        Ok(t.insert(name, e)?)
    }
}

/// Chases a face key through fragment rows to its root (the key that
/// is not itself a minted fragment). Bounded by the row count.
fn chase(rows: &BTreeMap<FaceKey, FaceKey>, mut f: FaceKey) -> FaceKey {
    for _ in 0..=rows.len() {
        match rows.get(&f) {
            Some(&p) => f = p,
            None => return f,
        }
    }
    f
}

/// Chases an edge through `SplitEdge` birth records, STOPPING at the
/// first key the operand's table names: the table is the identity
/// boundary — records deeper than the operand's own entities belong
/// to earlier ops (a union's rim fragment must not chase past its
/// own union-level name into its grand-parent).
fn chase_edge_to_table<T: Decide>(
    body: &Body<T>,
    table: &NameTable,
    mut e: EdgeKey,
    limit: usize,
) -> EdgeKey {
    for _ in 0..=limit {
        if table.name_of(&ent(0, EntityKey::Edge(e))).is_some() {
            return e;
        }
        match body.edge_provenance_of(e) {
            Some(Provenance::SplitEdge { edge }) => e = *edge,
            _ => return e,
        }
    }
    e
}

/// Names both sides of a split (spec D2's split vocabulary + N2).
#[allow(clippy::too_many_arguments)]
pub(crate) fn name_split<T: Decide>(
    node: RecipeNodeId,
    above: Option<&Body<T>>,
    below: Option<&Body<T>>,
    naming: &SplitNaming,
    target_node: RecipeNodeId,
    target_table: &NameTable,
    target_body: &Body<T>,
    tool_normal: Vec3<T>,
) -> Result<Arc<NameTable>, NamingError> {
    let b = band()?;
    let mut t = NameTable::new();
    let frag_rows: BTreeMap<FaceKey, FaceKey> = naming.face_fragments.iter().copied().collect();
    let section_keys: BTreeSet<FaceKey> = naming.sections.iter().map(|&(f, _)| f).collect();
    let mut sides: Vec<Side<'_, T>> = Vec::new();
    if let Some(body) = above {
        sides.push(Side {
            body,
            ix: 0,
            half: SplitHalf::Above,
        });
    }
    if let Some(body) = below {
        sides.push(Side {
            body,
            ix: 1,
            half: SplitHalf::Below,
        });
    }
    for s in &sides {
        t.insert(
            name1(EntityKind::Body, node, RoleSeg::SplitBody(s.half)),
            ent(s.ix, EntityKey::Body),
        )?;
    }

    // ---- Section faces: per-side completion-order index. ----
    let mut per_side_ix = [0u32; 2];
    for &(f, side) in &naming.sections {
        let half = match side {
            PlaneSide::Above => SplitHalf::Above,
            PlaneSide::Below => SplitHalf::Below,
            PlaneSide::On => {
                return Err(NamingError::Emission {
                    what: "section face classified On",
                });
            }
        };
        let slot = usize::from(half == SplitHalf::Below);
        let section = per_side_ix[slot];
        per_side_ix[slot] += 1;
        // The face is live in exactly the side that kept it.
        let Some(s) = sides
            .iter()
            .find(|s| s.half == half && s.body.get_face(f).is_some())
        else {
            return Err(NamingError::Emission {
                what: "section face live in no matching side body",
            });
        };
        t.insert(
            name1(
                EntityKind::Face,
                node,
                RoleSeg::SectionFace {
                    side: half,
                    section,
                },
            ),
            ent(s.ix, EntityKey::Face(f)),
        )?;
    }

    let mut tie = TieRows::default();
    name_split_faces(
        node,
        &mut t,
        &mut tie,
        &sides,
        &frag_rows,
        &section_keys,
        target_node,
        target_table,
        target_body,
        tool_normal,
        b,
    )?;
    tie.flush(&mut t)?;
    name_split_edges_vertices(
        node,
        &mut t,
        &mut tie,
        &sides,
        &frag_rows,
        &section_keys,
        naming,
        target_node,
        target_table,
    )?;
    tie.flush(&mut t)?;

    for s in &sides {
        super::emit::check_total(&t, s.body, s.ix)?;
    }
    Ok(Arc::new(t))
}

/// Split edges + vertices: pass-through, `SectionEdge` (chords),
/// `SplitFragment` (crossing-cut operand edges), `CrossingVertex`.
#[allow(clippy::too_many_arguments)]
fn name_split_edges_vertices<T: Decide>(
    node: RecipeNodeId,
    t: &mut NameTable,
    tie: &mut TieRows,
    sides: &[Side<'_, T>],
    frag_rows: &BTreeMap<FaceKey, FaceKey>,
    section_keys: &BTreeSet<FaceKey>,
    naming: &SplitNaming,
    target_node: RecipeNodeId,
    target_table: &NameTable,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let copy_to_original: BTreeMap<VertexKey, VertexKey> =
        naming.vertex_pairs.iter().copied().collect();
    for s in sides {
        let body = s.body;
        // Chord edges: every boundary edge of this side's section
        // faces.
        let mut chord_faces: BTreeMap<EdgeKey, FaceKey> = BTreeMap::new();
        for &(sf, _) in &naming.sections {
            if body.get_face(sf).is_none() || !section_keys.contains(&sf) {
                continue;
            }
            for he in face_half_edges(body, sf)? {
                let mate = body.mate(he).ok_or_else(|| bug("chord mate missing"))?;
                let mate_he = body
                    .get_half_edge(mate)
                    .ok_or_else(|| bug("chord mate dangling"))?;
                let other = body
                    .get_loop(mate_he.parent_loop)
                    .ok_or_else(|| bug("chord loop dangling"))?
                    .face;
                chord_faces.insert(mate_he.edge, other);
            }
        }
        // Chord edges named by the operand face their section boundary
        // runs across. `SectionEdge{side, face}` carries only that
        // face's name, so a section line that re-enters ONE operand
        // face — an inner loop, or any non-convex face — would mint
        // one name twice.
        //
        // A2 (ratified, #512): those chords become an N2 TIE rather
        // than a refusal. They are equally admissible under the one
        // name the vocabulary can spell; the selector layer narrows to
        // a specific chord geometrically (`select_where`), which is
        // the same disambiguation story ties have everywhere else. No
        // ordering is invented: the chords bound one section face and
        // have no covariant order-along direction of their own.
        let mut chords_by_face: BTreeMap<FaceKey, Vec<EdgeKey>> = BTreeMap::new();
        for (&e, &other) in &chord_faces {
            let root = chase(frag_rows, other);
            chords_by_face.entry(root).or_default().push(e);
        }
        for (root, edges) in chords_by_face {
            if section_keys.contains(&root) {
                return Err(bug("section chord adjacent to a section face"));
            }
            let parent = upstream_name(target_table, target_node, ent(0, EntityKey::Face(root)))?;
            let name = name1(
                EntityKind::Edge,
                node,
                RoleSeg::SectionEdge {
                    side: s.half,
                    face: Box::new(parent.name),
                },
            );
            let shared = parent.tied || edges.len() > 1;
            for e in edges {
                put(t, tie, shared, name.clone(), ent(s.ix, EntityKey::Edge(e)))?;
            }
        }
        // Remaining edges: pass-through or crossing-cut fragments. A
        // kept-key first child looks like an intact operand edge in
        // ITS side alone, so divided parents are collected across
        // BOTH sides first.
        let mut divided_edges: BTreeSet<EdgeKey> = BTreeSet::new();
        for sb in sides {
            for (e, _) in sb.body.edges() {
                // FRESH children only: an edge the target table
                // already names is the target's own entity, not a
                // product of THIS split.
                if target_table.name_of(&ent(0, EntityKey::Edge(e))).is_none()
                    && matches!(
                        sb.body.edge_provenance_of(e),
                        Some(Provenance::SplitEdge { .. })
                    )
                {
                    divided_edges.insert(chase_edge_to_table(
                        sb.body,
                        target_table,
                        e,
                        sb.body.edges().count(),
                    ));
                }
            }
        }
        for (e, _) in body.edges() {
            if chord_faces.contains_key(&e) {
                continue;
            }
            let root = chase_edge_to_table(body, target_table, e, body.edges().count());
            if target_table.name_of(&ent(0, EntityKey::Edge(e))).is_some()
                && !divided_edges.contains(&root)
            {
                // Intact operand edge: pass-through.
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(e)))?;
                put(t, tie, up.tied, up.name, ent(s.ix, EntityKey::Edge(e)))?;
                continue;
            }
            if target_table
                .name_of(&ent(0, EntityKey::Edge(root)))
                .is_none()
            {
                return Err(bug("edge descent reached no operand edge"));
            }
            let parent = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(root)))?;
            put(
                t,
                tie,
                parent.tied,
                name1(
                    EntityKind::Edge,
                    node,
                    RoleSeg::SplitFragment {
                        side: s.half,
                        parent: Box::new(parent.name),
                    },
                ),
                ent(s.ix, EntityKey::Edge(e)),
            )?;
        }
        // Vertices. Pair membership FIRST: a vertex the tool plane
        // passed through exists as a coincident copy in BOTH halves
        // (null-pair rows), so even an operand-named original must
        // take a side-tagged role, never the bare pass-through (the
        // bare name would alias across the two halves).
        let pair_originals: BTreeSet<VertexKey> =
            naming.vertex_pairs.iter().map(|&(_, o)| o).collect();
        for (v, _) in body.vertices() {
            let is_pair_member = copy_to_original.contains_key(&v) || pair_originals.contains(&v);
            if !is_pair_member
                && target_table
                    .name_of(&ent(0, EntityKey::Vertex(v)))
                    .is_some()
            {
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Vertex(v)))?;
                put(t, tie, up.tied, up.name, ent(s.ix, EntityKey::Vertex(v)))?;
                continue;
            }
            // Resolve the birth record — directly, or through the
            // null-pair copy row (the original's record lives in
            // whichever side kept it).
            let src = copy_to_original.get(&v).copied().unwrap_or(v);
            let parent_edge = sides
                .iter()
                .find_map(|sb| match sb.body.vertex_provenance_of(src) {
                    Some(Provenance::SplitEdge { edge }) => Some(chase_edge_to_table(
                        sb.body,
                        target_table,
                        *edge,
                        sb.body.edges().count(),
                    )),
                    _ => None,
                });
            let (seg, from_tie) = if let Some(parent_edge) = parent_edge {
                // Crossing vertex: minted where the plane crossed an
                // operand edge's interior.
                let parent = upstream_name(
                    target_table,
                    target_node,
                    ent(0, EntityKey::Edge(parent_edge)),
                )?;
                (
                    RoleSeg::CrossingVertex {
                        side: s.half,
                        edge: Box::new(parent.name),
                    },
                    parent.tied,
                )
            } else if target_table
                .name_of(&ent(0, EntityKey::Vertex(src)))
                .is_some()
            {
                // The plane passed THROUGH an operand vertex (review
                // R2): side-tagged pass-through — operand identity
                // from the pair row, side from body membership (a
                // recorded verdict).
                let of = upstream_name(target_table, target_node, ent(0, EntityKey::Vertex(src)))?;
                (
                    RoleSeg::OnToolVertex {
                        side: s.half,
                        of: Box::new(of.name),
                    },
                    of.tied,
                )
            } else {
                return Err(bug(
                    "on-plane vertex with neither a SplitEdge record nor an operand identity",
                ));
            };
            put(
                t,
                tie,
                from_tie,
                name1(EntityKind::Vertex, node, seg),
                ent(s.ix, EntityKey::Vertex(v)),
            )?;
        }
    }
    Ok(())
}

/// One boolean operand under naming.
pub(crate) struct OperandCtx<'a, T: Decide> {
    /// The operand's node (error context).
    pub node: RecipeNodeId,
    /// Its name table (total over its body).
    pub table: &'a NameTable,
    /// Its body (order-along carrier geometry).
    pub body: &'a Body<T>,
}

/// Which operand a result key descends from, with its operand-space
/// key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Descent {
    A(FaceKey),
    B(FaceKey),
}

/// Names a boolean result (spec D2's boolean vocabulary; N2/N3).
pub(crate) fn name_boolean<T: Decide>(
    node: RecipeNodeId,
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
) -> Result<Arc<NameTable>, NamingError> {
    let bnd = band()?;
    let bug = |what| NamingError::Emission { what };
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;

    use topo::OperandKeys;
    let inv_faces: BTreeMap<FaceKey, FaceKey> =
        naming.graft_faces.iter().map(|&(s, d)| (d, s)).collect();
    let inv_edges: BTreeMap<EdgeKey, EdgeKey> =
        naming.graft_edges.iter().map(|&(s, d)| (d, s)).collect();
    let fwd_edges: BTreeMap<EdgeKey, EdgeKey> = naming.graft_edges.iter().copied().collect();
    let inv_vertices: BTreeMap<VertexKey, VertexKey> =
        naming.graft_vertices.iter().map(|&(s, d)| (d, s)).collect();
    let a_rows: BTreeMap<FaceKey, FaceKey> = naming.face_fragments_a.iter().copied().collect();
    let b_rows: BTreeMap<FaceKey, FaceKey> = naming.face_fragments_b.iter().copied().collect();
    let seam_set: BTreeSet<EdgeKey> = naming.seam_edges.iter().copied().collect();
    let inc = Incidence::of(body)?;

    // Result face → operand-space root (fragment rows chased in the
    // right key space).
    let descend_face = |f: FaceKey| -> Result<Descent, NamingError> {
        match (naming.a_keys, naming.b_keys) {
            (OperandKeys::Direct, OperandKeys::Grafted) => match inv_faces.get(&f) {
                Some(&fb) => Ok(Descent::B(chase(&b_rows, fb))),
                None => Ok(Descent::A(chase(&a_rows, f))),
            },
            (OperandKeys::Direct, OperandKeys::Absent) => Ok(Descent::A(chase(&a_rows, f))),
            (OperandKeys::Absent, OperandKeys::Direct) => Ok(Descent::B(chase(&b_rows, f))),
            _ => Err(NamingError::Emission {
                what: "unsupported operand-key layout",
            }),
        }
    };
    let operand_face_name = |d: Descent| -> Result<Upstream, NamingError> {
        match d {
            Descent::A(f) => upstream_name(a.table, a.node, ent(0, EntityKey::Face(f))),
            Descent::B(f) => upstream_name(b.table, b.node, ent(0, EntityKey::Face(f))),
        }
    };
    let wrap = |d: Descent, inner: StableName, kind: EntityKind| {
        let seg = match d {
            Descent::A(_) => RoleSeg::FromA(Box::new(inner)),
            Descent::B(_) => RoleSeg::FromB(Box::new(inner)),
        };
        name1(kind, node, seg)
    };

    // ---- Faces: merges first (N3), then descent groups. ----
    let mut tie = TieRows::default();
    let mut handled: BTreeSet<FaceKey> = BTreeSet::new();
    // Kept face → constituent descents (M4 PR 5: the seam-edge walk
    // reads THROUGH a merged face to its mint-time operand identity).
    let mut merged_descents: BTreeMap<FaceKey, Vec<Descent>> = BTreeMap::new();
    for (kept, absorbed) in &naming.merge_groups {
        if body.get_face(*kept).is_none() {
            return Err(bug("merge kept face not live"));
        }
        let mut constituents = Vec::new();
        let mut descents = Vec::new();
        // A merged name descends from a tie iff ANY constituent does.
        let mut from_tie = false;
        for &c in core::iter::once(kept).chain(absorbed) {
            let d = descend_face(c)?;
            descents.push(d);
            let up = operand_face_name(d)?;
            from_tie |= up.tied;
            constituents.push(wrap(d, up.name, EntityKind::Face));
        }
        merged_descents.insert(*kept, descents);
        constituents.sort_unstable();
        // Review R8, RESOLVED at M4 PR 5: dedup makes the constituent
        // SET the name — TWO merge groups with one constituent set
        // (reachable only when BOTH kept faces are fragments of one
        // operand face, glued to fragments of one partner: a
        // disjoint-patch declared contact) collide LOUDLY at insert
        // (`DuplicateName` → typed `NamingError`; pinned by
        // `merged_same_constituent_groups_collide_loudly`). No silent
        // aliasing is possible; a per-group discriminator upgrades
        // this refusal to a success if the disjoint-patch class ever
        // matters (REPORT'd, banked).
        constituents.dedup();
        put(
            &mut t,
            &mut tie,
            from_tie,
            name1(EntityKind::Face, node, RoleSeg::Merged(constituents)),
            ent(0, EntityKey::Face(*kept)),
        )?;
        handled.insert(*kept);
    }
    let mut groups: BTreeMap<Descent, Vec<FaceKey>> = BTreeMap::new();
    for (f, _) in body.faces() {
        if !handled.contains(&f) {
            groups.entry(descend_face(f)?).or_default().push(f);
        }
    }
    for (d, members) in groups {
        let root_name = operand_face_name(d)?;
        let from_tie = root_name.tied;
        let base = wrap(d, root_name.name, EntityKind::Face);
        if members.len() == 1 {
            put(
                &mut t,
                &mut tie,
                from_tie,
                base,
                ent(0, EntityKey::Face(members[0])),
            )?;
            continue;
        }
        name_fragment_group(
            &mut t,
            &mut tie,
            from_tie,
            body,
            &base,
            &members,
            &seam_set,
            &inc,
            &descend_face,
            &operand_face_name,
            bnd,
        )?;
    }
    tie.flush(&mut t)?;

    name_boolean_edges(
        node,
        &mut t,
        &mut tie,
        body,
        naming,
        a,
        b,
        &inv_edges,
        &fwd_edges,
        &seam_set,
        &inc,
        &descend_face,
        &operand_face_name,
        &merged_descents,
        bnd,
    )?;
    tie.flush(&mut t)?;
    name_boolean_vertices(
        node,
        &mut t,
        &mut tie,
        body,
        naming,
        &inv_vertices,
        a,
        b,
        &inc,
        bnd,
    )?;
    tie.flush(&mut t)?;

    super::emit::check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// Qualifies a multi-fragment descent group (N2): sign vectors of
/// `name_frag_side_of` against the seam partners' carriers; equal
/// vectors tie.
#[allow(clippy::too_many_arguments)]
fn name_fragment_group<T: Decide>(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    body: &Body<T>,
    base: &StableName,
    members: &[FaceKey],
    seam_set: &BTreeSet<EdgeKey>,
    inc: &Incidence,
    descend_face: &impl Fn(FaceKey) -> Result<Descent, NamingError>,
    operand_face_name: &impl Fn(Descent) -> Result<Upstream, NamingError>,
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };
    // Partners: operand faces across the members' seam edges.
    let mut partners: BTreeMap<StableName, FaceKey> = BTreeMap::new();
    for &m in members {
        for he in face_half_edges(body, m)? {
            let e = body
                .get_half_edge(he)
                .ok_or_else(|| bug("fragment partner walk: dangling half-edge"))?
                .edge;
            if !seam_set.contains(&e) {
                continue;
            }
            let faces = inc
                .edge_faces
                .get(&e)
                .ok_or_else(|| bug("fragment partner walk: seam edge without faces"))?;
            for &other in faces {
                if other != m {
                    let d = descend_face(other)?;
                    // The partner name is a discriminator LABEL here,
                    // so a tied partner is admissible unchanged. Its
                    // representative face is the first in BTreeMap
                    // order (review NOTE-1): arbitrary among tied
                    // candidates, never nondeterministic.
                    partners.entry(operand_face_name(d)?.name).or_insert(other);
                }
            }
        }
    }
    if partners.is_empty() {
        return Err(bug("multi-fragment group with no seam partners"));
    }
    let mut by_vector: BTreeMap<Vec<(StableName, super::role::SideVerdict)>, Vec<FaceKey>> =
        BTreeMap::new();
    for &m in members {
        let mut vector = Vec::with_capacity(partners.len());
        for (pname, &pface) in &partners {
            let (origin, normal) = face_plane(body, pface)?;
            let verdict = side_of_face(body, m, origin, normal, bnd)?;
            vector.push((pname.clone(), verdict));
        }
        by_vector.entry(vector).or_default().push(m);
    }
    for (vector, faces) in by_vector {
        let mut name = base.clone();
        name.path.push(RoleSeg::Fragment(Qualifier::SideOf(vector)));
        if faces.len() == 1 {
            put(t, tie, from_tie, name, ent(0, EntityKey::Face(faces[0])))?;
        } else {
            // The N2 tie: equally-admissible symmetric candidates.
            for &f in &faces {
                tie.push(name.clone(), ent(0, EntityKey::Face(f)));
            }
        }
    }
    Ok(())
}

/// Boolean edges: `Seam` for zip-minted edges, `FromA`/`FromB` (with
/// order-along fragment qualifiers) for operand-descended ones.
#[allow(clippy::too_many_arguments)]
fn name_boolean_edges<T: Decide>(
    node: RecipeNodeId,
    t: &mut NameTable,
    tie: &mut TieRows,
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    inv_edges: &BTreeMap<EdgeKey, EdgeKey>,
    fwd_edges: &BTreeMap<EdgeKey, EdgeKey>,
    seam_set: &BTreeSet<EdgeKey>,
    inc: &Incidence,
    descend_face: &impl Fn(FaceKey) -> Result<Descent, NamingError>,
    operand_face_name: &impl Fn(Descent) -> Result<Upstream, NamingError>,
    merged_descents: &BTreeMap<FaceKey, Vec<Descent>>,
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };
    use topo::OperandKeys;

    // ---- Seam edges (zip-listed AND derived — see below), grouped
    // by their (fA, fB) operand pair. A derived chord between two
    // SAME-operand faces (the collinear channel-cut lane re-mints a
    // sub-edge of an operand edge as a chord) descends instead to the
    // unique operand edge its two parent faces share — combinatorial
    // adjacency of emitted anchors, not matching. ----
    enum ChordKind {
        Cross(Upstream, Upstream),
        SameA(EdgeKey),
        SameB(EdgeKey),
    }
    // A face's descent for CHORD purposes: a plain face descends as
    // itself; a MERGED face (M4 PR 5, N3 live) reads through to its
    // unique constituent on the side the partner needs — the seam's
    // mint-time operand identity survives the glue. Ambiguity (both
    // faces merged, or several same-side constituents) refuses typed.
    let chord_descent =
        |f: FaceKey, want_opposite_of: Option<Descent>| -> Result<Descent, NamingError> {
            let Some(ds) = merged_descents.get(&f) else {
                return descend_face(f);
            };
            let pick = |want_a: bool| -> Result<Descent, NamingError> {
                // Constituent fragments of ONE operand face share a
                // descent — dedup before the uniqueness demand.
                let mut hits: Vec<Descent> = ds
                    .iter()
                    .filter(|d| matches!(d, Descent::A(_)) == want_a)
                    .copied()
                    .collect();
                hits.sort_unstable();
                hits.dedup();
                match hits.as_slice() {
                    [] => Err(bug("merged face lacks the needed operand-side constituent")),
                    [one] => Ok(*one),
                    _ => Err(bug(
                        "merged face has several same-side constituents at a seam edge",
                    )),
                }
            };
            match want_opposite_of {
                Some(Descent::A(_)) => pick(false),
                Some(Descent::B(_)) => pick(true),
                None => Err(bug("seam edge between two merged faces (unsupported)")),
            }
        };
    let chord_kind = |e: EdgeKey| -> Result<ChordKind, NamingError> {
        let faces = inc
            .edge_faces
            .get(&e)
            .ok_or_else(|| bug("seam edge without adjacent faces"))?;
        if faces.len() != 2 {
            return Err(bug("seam edge without exactly two adjacent faces"));
        }
        let merged0 = merged_descents.contains_key(&faces[0]);
        let merged1 = merged_descents.contains_key(&faces[1]);
        let (d0, d1) = match (merged0, merged1) {
            (false, false) => (descend_face(faces[0])?, descend_face(faces[1])?),
            (true, false) => {
                let d1 = descend_face(faces[1])?;
                (chord_descent(faces[0], Some(d1))?, d1)
            }
            (false, true) => {
                let d0 = descend_face(faces[0])?;
                (d0, chord_descent(faces[1], Some(d0))?)
            }
            (true, true) => (
                chord_descent(faces[0], None)?,
                chord_descent(faces[1], None)?,
            ),
        };
        Ok(match (d0, d1) {
            (Descent::A(_), Descent::B(_)) => {
                ChordKind::Cross(operand_face_name(d0)?, operand_face_name(d1)?)
            }
            (Descent::B(_), Descent::A(_)) => {
                ChordKind::Cross(operand_face_name(d1)?, operand_face_name(d0)?)
            }
            (Descent::A(fa0), Descent::A(fa1)) => {
                ChordKind::SameA(unique_shared_edge(a.body, fa0, fa1)?)
            }
            (Descent::B(fb0), Descent::B(fb1)) => {
                ChordKind::SameB(unique_shared_edge(b.body, fb0, fb1)?)
            }
        })
    };
    let seam_pair = |e: EdgeKey| -> Result<(Upstream, Upstream), NamingError> {
        match chord_kind(e)? {
            ChordKind::Cross(fa, fb) => Ok((fa, fb)),
            _ => Err(bug("seam edge between same-operand faces")),
        }
    };
    // Group value: (descends-from-a-tie, edges). Two tied operand
    // faces answer to ONE name, so their seam chords land in one
    // group — the widening B1 asks for, not a collision.
    let mut seam_groups: BTreeMap<(StableName, StableName), (bool, Vec<EdgeKey>)> = BTreeMap::new();
    let mut add_seam = |fa: Upstream, fb: Upstream, e: EdgeKey| {
        let from_tie = fa.tied || fb.tied;
        let slot = seam_groups
            .entry((fa.name, fb.name))
            .or_insert((false, Vec::new()));
        slot.0 |= from_tie;
        slot.1.push(e);
    };
    for &e in &naming.seam_edges {
        if body.get_edge(e).is_none() {
            continue; // consumed by the merge stage — historical row
        }
        let (fa, fb) = seam_pair(e)?;
        add_seam(fa, fb, e);
    }

    // ---- Operand-descended edges, grouped by (space, root). ----
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    enum ERoot {
        A(EdgeKey),
        B(EdgeKey),
    }
    // Best-effort B-space descent: stops at the first key B's table
    // names. A broken chain (a middle fragment of a doubly-pierced
    // edge dies PRE-graft, so its key is unnamed AND ungrafted)
    // returns the non-resolving key instead of refusing — the
    // `resolves == false` route below hands such edges to
    // `chord_kind`, the same rescue the A lane gets (review R1: lane
    // parity; the kernel completed the body, naming must too).
    let chase_b = |mut e_b: EdgeKey| -> EdgeKey {
        for _ in 0..=fwd_edges.len() {
            if b.table.name_of(&ent(0, EntityKey::Edge(e_b))).is_some() {
                return e_b;
            }
            let Some(res) = fwd_edges.get(&e_b) else {
                return e_b; // dead, ungrafted intermediate
            };
            match body.edge_provenance_of(*res) {
                Some(Provenance::SplitEdge { edge }) => e_b = *edge,
                _ => return e_b,
            }
        }
        e_b
    };
    let mut groups: BTreeMap<ERoot, Vec<EdgeKey>> = BTreeMap::new();
    for (e, _) in body.edges() {
        if seam_set.contains(&e) {
            continue;
        }
        let root = match (naming.a_keys, naming.b_keys) {
            (OperandKeys::Direct, OperandKeys::Grafted) => match inv_edges.get(&e) {
                Some(&eb) => ERoot::B(chase_b(eb)),
                None => ERoot::A(chase_edge_to_table(body, a.table, e, body.edges().count())),
            },
            (OperandKeys::Direct, OperandKeys::Absent) => {
                ERoot::A(chase_edge_to_table(body, a.table, e, body.edges().count()))
            }
            (OperandKeys::Absent, OperandKeys::Direct) => ERoot::B(chase_b(e)),
            _ => return Err(bug("unsupported operand-key layout")),
        };
        // A root that resolves in no operand table is a join-minted
        // crossing chord that survived OUTSIDE the zip's list (channel
        // cuts: chords on the operand's own faces) — a DERIVED seam
        // edge, named by its adjacent faces' descent like any seam.
        let resolves = match &root {
            ERoot::A(k) => a.table.name_of(&ent(0, EntityKey::Edge(*k))).is_some(),
            ERoot::B(k) => b.table.name_of(&ent(0, EntityKey::Edge(*k))).is_some(),
        };
        if resolves {
            groups.entry(root).or_default().push(e);
        } else {
            match chord_kind(e)? {
                ChordKind::Cross(fa, fb) => {
                    add_seam(fa, fb, e);
                }
                ChordKind::SameA(k) => {
                    groups.entry(ERoot::A(k)).or_default().push(e);
                }
                ChordKind::SameB(k) => {
                    groups.entry(ERoot::B(k)).or_default().push(e);
                }
            }
        }
    }
    for ((fa, fb), (from_tie, edges)) in seam_groups {
        let base = name1(
            EntityKind::Edge,
            node,
            RoleSeg::Seam {
                a: Box::new(fa),
                b: Box::new(fb),
            },
        );
        if edges.len() == 1 {
            put(t, tie, from_tie, base, ent(0, EntityKey::Edge(edges[0])))?;
            continue;
        }
        // Collinear chain: order along the pair's intersection line,
        // oriented n_a × n_b (the carriers' own orientations, A side
        // first — descent decides which is which, never list order).
        //
        // The SIGN of `dir` is load-bearing, not just its axis:
        // `edge_extent` projects onto it and `order_along` ranks by
        // that signed parameter, so negating `dir` reverses every
        // `OrderAlong` rank and renames the whole chain. Hence
        // `face_plane` returns OUTWARD normals (S10 category A): the
        // orientation of this line is a fact about the two faces'
        // material sides, and it must move only when they do.
        let faces = inc
            .edge_faces
            .get(&edges[0])
            .ok_or_else(|| bug("seam group lost its faces"))?;
        let (f0, f1) = (faces[0], faces[1]);
        let (fa_key, fb_key) = match descend_face(f0)? {
            Descent::A(_) => (f0, f1),
            Descent::B(_) => (f1, f0),
        };
        let (_, na) = face_plane(body, fa_key)?;
        let (_, nb) = face_plane(body, fb_key)?;
        let dir = na.cross(nb);
        let extents = edges
            .iter()
            .map(|&e| edge_extent(body, e, dir))
            .collect::<Result<Vec<_>, _>>()?;
        insert_ranked_or_tied(t, tie, from_tie, base, &edges, &extents, bnd, |&e| {
            ent(0, EntityKey::Edge(e))
        })?;
    }
    for (root, edges) in groups {
        let (inner, wrap_a, op_body, root_key) = match root {
            ERoot::A(k) => (
                upstream_name(a.table, a.node, ent(0, EntityKey::Edge(k)))?,
                true,
                a.body,
                k,
            ),
            ERoot::B(k) => (
                upstream_name(b.table, b.node, ent(0, EntityKey::Edge(k)))?,
                false,
                b.body,
                k,
            ),
        };
        let from_tie = inner.tied;
        let seg = if wrap_a {
            RoleSeg::FromA(Box::new(inner.name))
        } else {
            RoleSeg::FromB(Box::new(inner.name))
        };
        let base = name1(EntityKind::Edge, node, seg);
        if edges.len() == 1 {
            put(t, tie, from_tie, base, ent(0, EntityKey::Edge(edges[0])))?;
            continue;
        }
        // Sub-edge chain: order along the parent edge's own oriented
        // carrier (operand geometry).
        let dir = edge_dir(op_body, root_key)?;
        let extents = edges
            .iter()
            .map(|&e| edge_extent(body, e, dir))
            .collect::<Result<Vec<_>, _>>()?;
        insert_ranked_or_tied(t, tie, from_tie, base, &edges, &extents, bnd, |&e| {
            ent(0, EntityKey::Edge(e))
        })?;
    }
    Ok(())
}

/// Boolean vertices: operand pass-downs (`FromA`/`FromB`), and seam
/// (crossing/fused) vertices named `Seam{a, b}` by the operand
/// entities whose crossing minted them — derived from the already-
/// named incident edges (combinatorial wiring facts).
#[allow(clippy::too_many_arguments)]
fn name_boolean_vertices<T: Decide>(
    node: RecipeNodeId,
    t: &mut NameTable,
    tie: &mut TieRows,
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    inv_vertices: &BTreeMap<VertexKey, VertexKey>,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    inc: &Incidence,
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };
    // Zip fusions: kept key → dead partners (a fused vertex may owe
    // its operand identity to a DEAD partner's key — e.g. a B corner
    // vertex fused into an A-side crossing key on a shared plane).
    let mut fused: BTreeMap<VertexKey, Vec<VertexKey>> = BTreeMap::new();
    for &(dead, kept) in &naming.vertex_merges {
        fused.entry(kept).or_default().push(dead);
    }
    // The operand identity of one result-arena vertex key, if any:
    // A-space direct, or grafted B through the graft rows.
    let operand_identity = |k: VertexKey| -> Result<Option<(StableName, bool)>, NamingError> {
        if let Some(&vb) = inv_vertices.get(&k) {
            if b.table.name_of(&ent(0, EntityKey::Vertex(vb))).is_some() {
                let inner = upstream_name(b.table, b.node, ent(0, EntityKey::Vertex(vb)))?;
                return Ok(Some((
                    name1(
                        EntityKind::Vertex,
                        node,
                        RoleSeg::FromB(Box::new(inner.name)),
                    ),
                    inner.tied,
                )));
            }
        } else if a.table.name_of(&ent(0, EntityKey::Vertex(k))).is_some() {
            let inner = upstream_name(a.table, a.node, ent(0, EntityKey::Vertex(k)))?;
            return Ok(Some((
                name1(
                    EntityKind::Vertex,
                    node,
                    RoleSeg::FromA(Box::new(inner.name)),
                ),
                inner.tied,
            )));
        }
        Ok(None)
    };
    // Candidate seam-vertex names, grouped for multiplicity: the key
    // is the (A, B) parent pair, the value (descends-from-a-tie,
    // vertices).
    let mut groups: BTreeMap<(StableName, StableName), (bool, Vec<VertexKey>)> = BTreeMap::new();
    for (v, _) in body.vertices() {
        // Operand pass-downs: the kept key itself, then its dead
        // fusion partners (deterministic order: KEPT-KEY identity
        // wins when both operands fused here — `operand_identity`
        // checks the graft destination first, and the kept key is
        // A's exactly because `zip_seam` keeps the outer cycle's
        // vertex; review R9 — the PR 4 Vanished-diagnosis item covers
        // the retired partner).
        let mut identity = operand_identity(v)?;
        if identity.is_none() {
            for &dead in fused.get(&v).into_iter().flatten() {
                identity = operand_identity(dead)?;
                if identity.is_some() {
                    break;
                }
            }
        }
        if let Some((name, from_tie)) = identity {
            put(t, tie, from_tie, name, ent(0, EntityKey::Vertex(v)))?;
            continue;
        }
        // Seam vertex: parents from incident edges' names.
        let edges = inc
            .vertex_edges
            .get(&v)
            .ok_or_else(|| bug("seam vertex without incident edges"))?;
        let mut a_edges: Vec<StableName> = Vec::new();
        let mut b_edges: Vec<StableName> = Vec::new();
        let mut a_faces: Vec<StableName> = Vec::new();
        let mut b_faces: Vec<StableName> = Vec::new();
        let mut seam_lines: Vec<(StableName, StableName)> = Vec::new();
        // B1: a seam vertex reads its parentage off the incident EDGE
        // names, so an edge name that is itself tied makes the vertex
        // name tie-descended too.
        let mut from_tie = false;
        for &e in edges {
            let Some(ename) = t.name_of(&ent(0, EntityKey::Edge(e))) else {
                return Err(bug("seam vertex incident to an unnamed edge"));
            };
            from_tie |= t.is_tied(ename);
            match ename.path.first() {
                Some(RoleSeg::FromA(x)) => a_edges.push((**x).clone()),
                Some(RoleSeg::FromB(x)) => b_edges.push((**x).clone()),
                // Zip-listed AND derived seams both qualify (M4 PR 5:
                // declared merges reroute channel-cut chords into the
                // derived-seam lane, so a seam vertex may lean on a
                // Seam-named edge outside `naming.seam_edges`) — the
                // NAME is the evidence either way.
                Some(RoleSeg::Seam { a: fa, b: fb }) => {
                    a_faces.push((**fa).clone());
                    b_faces.push((**fb).clone());
                    seam_lines.push(((**fa).clone(), (**fb).clone()));
                }
                _ => return Err(bug("seam vertex incident to an unexpected edge role")),
            }
        }
        for list in [&mut a_edges, &mut b_edges, &mut a_faces, &mut b_faces] {
            list.sort_unstable();
            list.dedup();
        }
        // Contact-record partner: the reduction's own declared
        // contacts (mint-time, PRE-remap — `reduction_contacts`),
        // read in the right key spaces: A rows are result keys, B
        // rows are B-operand keys. A residual crossing vertex whose
        // seam structure was consumed (shared-plane overlaps) finds
        // its coincident operand partner here — recorded knowledge,
        // never re-measured.
        use topo::OperandKeys;
        let (va_key, vb_key): (Option<VertexKey>, Option<VertexKey>) =
            match (naming.a_keys, naming.b_keys) {
                (OperandKeys::Direct, OperandKeys::Grafted) => match inv_vertices.get(&v) {
                    Some(&vb) => (None, Some(vb)),
                    None => (Some(v), None),
                },
                (OperandKeys::Direct, OperandKeys::Absent) => (Some(v), None),
                (OperandKeys::Absent, OperandKeys::Direct) => (None, Some(v)),
                _ => (None, None),
            };
        let rc = &naming.reduction_contacts;
        let partner_b: Option<Upstream> = va_key
            .and_then(|k| rc.vv.iter().find(|r| r.a == k).map(|r| r.b))
            .and_then(|pb| upstream_name(b.table, b.node, ent(0, EntityKey::Vertex(pb))).ok());
        let partner_a: Option<Upstream> = vb_key
            .and_then(|k| rc.vv.iter().find(|r| r.b == k).map(|r| r.a))
            .and_then(|pa| upstream_name(a.table, a.node, ent(0, EntityKey::Vertex(pa))).ok());
        from_tie |= partner_b.as_ref().is_some_and(|u| u.tied);
        from_tie |= partner_a.as_ref().is_some_and(|u| u.tied);
        let partner_b_inner: Option<StableName> = partner_b.map(|u| u.name);
        let partner_a_inner: Option<StableName> = partner_a.map(|u| u.name);
        seam_lines.sort_unstable();
        seam_lines.dedup();
        // The A side of the pair is always an A-descended name and the
        // B side always a B-descended one: every arm draws its two
        // components from different sources, so `Seam{x, x}` — a
        // well-formed name for the wrong thing — has no arm to come
        // from. The contact-record partners are bound in the
        // scrutinee, not guarded and unwrapped, so the compiler
        // carries that.
        let pair = match (
            a_edges.as_slice(),
            b_edges.as_slice(),
            partner_a_inner.as_ref(),
            partner_b_inner.as_ref(),
        ) {
            ([ae], [be], _, _) => (ae.clone(), be.clone()),
            // A pure seam-junction vertex (M4 PR 5: declared merges
            // can consume every operand-descended edge at a crossing
            // vertex): the incident seam edges' face parents determine
            // it when they agree on ONE (A, B) pair.
            ([], [], _, _) if a_faces.len() == 1 && b_faces.len() == 1 => {
                (a_faces[0].clone(), b_faces[0].clone())
            }
            ([ae], [], _, _) if b_faces.len() == 1 => (ae.clone(), b_faces[0].clone()),
            ([], [be], _, _) if a_faces.len() == 1 => (a_faces[0].clone(), be.clone()),
            ([ae], [], _, Some(pb)) => (ae.clone(), pb.clone()),
            ([], [be], Some(pa), _) => (pa.clone(), be.clone()),
            // A seam JUNCTION (M4 PR 5: declared merges can consume
            // every operand-descended edge at a crossing): the vertex
            // where k ≥ 2 seam LINES meet. Its name is the sorted
            // path of the lines' Seam segments — deterministic, and
            // unique per line set (straight lines meet once).
            ([], [], _, _) if seam_lines.len() >= 2 => {
                let mut segs: Vec<RoleSeg> = seam_lines
                    .iter()
                    .map(|(fa, fb)| RoleSeg::Seam {
                        a: Box::new(fa.clone()),
                        b: Box::new(fb.clone()),
                    })
                    .collect();
                segs.sort_unstable();
                let name = StableName {
                    kind: EntityKind::Vertex,
                    node,
                    path: segs,
                };
                put(t, tie, from_tie, name, ent(0, EntityKey::Vertex(v)))?;
                continue;
            }
            _ => {
                return Err(bug(
                    "seam vertex parentage underdetermined from incident edges",
                ));
            }
        };
        let slot = groups.entry(pair).or_insert((false, Vec::new()));
        slot.0 |= from_tie;
        slot.1.push(v);
    }
    for ((pa, pb), (from_tie, verts)) in groups {
        let base = name1(
            EntityKind::Vertex,
            node,
            RoleSeg::Seam {
                a: Box::new(pa.clone()),
                b: Box::new(pb.clone()),
            },
        );
        if verts.len() == 1 {
            put(t, tie, from_tie, base, ent(0, EntityKey::Vertex(verts[0])))?;
            continue;
        }
        // Same pair crossing more than once: order along the edge
        // parent's own carrier (prefer the A side).
        let carrier = resolve_edge_carrier(&pa, a).or_else(|| resolve_edge_carrier(&pb, b));
        let Some(dir) = carrier else {
            for &v in &verts {
                tie.push(base.clone(), ent(0, EntityKey::Vertex(v)));
            }
            continue;
        };
        let extents = verts
            .iter()
            .map(|&v| {
                let p = body
                    .get_vertex(v)
                    .and_then(|vd| body.get_point(vd.point))
                    .copied()
                    .ok_or_else(|| bug("seam vertex without point"))?;
                let tv = Vec3::new(p.x, p.y, p.z).dot(dir);
                Ok(Extent { min: tv, max: tv })
            })
            .collect::<Result<Vec<_>, NamingError>>()?;
        insert_ranked_or_tied(t, tie, from_tie, base, &verts, &extents, bnd, |&v| {
            ent(0, EntityKey::Vertex(v))
        })?;
    }
    Ok(())
}

/// The oriented carrier of an operand-edge parent name, if the name
/// denotes an edge in that operand's table.
fn resolve_edge_carrier<T: Decide>(parent: &StableName, op: &OperandCtx<'_, T>) -> Option<Vec3<T>> {
    if parent.kind != EntityKind::Edge {
        return None;
    }
    match op.table.lookup(parent) {
        Some(Entry::Unique(e)) => match e.key {
            EntityKey::Edge(k) => edge_dir(op.body, k).ok(),
            _ => None,
        },
        _ => None,
    }
}

/// An edge's endpoint-extent along `dir`.
fn edge_extent<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    dir: Vec3<T>,
) -> Result<Extent<T>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let (v0, v1) = edge_ends(body, e)?;
    let p = |v: VertexKey| -> Result<Point3<T>, NamingError> {
        body.get_vertex(v)
            .and_then(|vd| body.get_point(vd.point))
            .copied()
            .ok_or_else(|| bug("edge_extent: vertex without point"))
    };
    let (p0, p1) = (p(v0)?, p(v1)?);
    let t0 = Vec3::new(p0.x, p0.y, p0.z).dot(dir);
    let t1 = Vec3::new(p1.x, p1.y, p1.z).dot(dir);
    Ok(Extent {
        min: t0.min(t1),
        max: t0.max(t1),
    })
}

/// The oriented direction of an operand edge (he_plus start → end).
fn edge_dir<T: Decide>(body: &Body<T>, e: EdgeKey) -> Result<Vec3<T>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let (v0, v1) = edge_ends(body, e)?;
    let p = |v: VertexKey| -> Result<Point3<T>, NamingError> {
        body.get_vertex(v)
            .and_then(|vd| body.get_point(vd.point))
            .copied()
            .ok_or_else(|| bug("edge_dir: vertex without point"))
    };
    Ok(p(v1)? - p(v0)?)
}

/// Inserts a same-name group ranked by order-along, or tied when
/// genuinely unordered.
#[allow(clippy::too_many_arguments)]
fn insert_ranked_or_tied<T: Decide, K: Copy>(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    base: StableName,
    keys: &[K],
    extents: &[Extent<T>],
    bnd: geom_core::Band,
    to_ent: impl Fn(&K) -> super::table::EntityRef,
) -> Result<(), NamingError> {
    match order_along(extents, bnd)? {
        Some(ranks) => {
            let of = u32::try_from(keys.len()).unwrap_or(u32::MAX);
            for (k, rank) in keys.iter().zip(ranks) {
                let mut name = base.clone();
                name.path
                    .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }));
                put(t, tie, from_tie, name, to_ent(k))?;
            }
        }
        None => {
            for k in keys {
                tie.push(base.clone(), to_ent(k));
            }
        }
    }
    Ok(())
}

/// Split faces: pass-through for uncut operand faces; `SplitFragment`
/// (side-discriminated, same-side multiplicity by order-along the
/// parent's section line) for cut ones.
#[allow(clippy::too_many_arguments)]
fn name_split_faces<T: Decide>(
    node: RecipeNodeId,
    t: &mut NameTable,
    tie: &mut TieRows,
    sides: &[Side<'_, T>],
    frag_rows: &BTreeMap<FaceKey, FaceKey>,
    section_keys: &BTreeSet<FaceKey>,
    target_node: RecipeNodeId,
    target_table: &NameTable,
    target_body: &Body<T>,
    tool_normal: Vec3<T>,
    b: geom_core::Band,
) -> Result<(), NamingError> {
    // Every root that was ever divided: fragments cover it.
    let divided: BTreeSet<FaceKey> = frag_rows.values().map(|&p| chase(frag_rows, p)).collect();
    // (root, side-slot) → members.
    type Members = Vec<(u32, SplitHalf, FaceKey)>;
    let mut groups: BTreeMap<(FaceKey, u32), Members> = BTreeMap::new();
    for s in sides {
        for (f, _) in s.body.faces() {
            if section_keys.contains(&f) {
                continue;
            }
            let root = chase(frag_rows, f);
            if root == f && !divided.contains(&root) {
                // Uncut operand face: pass-through (N1: the split
                // contributes no segment to survivors).
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Face(f)))?;
                put(t, tie, up.tied, up.name, ent(s.ix, EntityKey::Face(f)))?;
            } else {
                groups
                    .entry((root, s.ix))
                    .or_default()
                    .push((s.ix, s.half, f));
            }
        }
    }
    for ((root, _), members) in groups {
        let parent = upstream_name(target_table, target_node, ent(0, EntityKey::Face(root)))?;
        let from_tie = parent.tied;
        let half = members[0].1;
        let base = RoleSeg::SplitFragment {
            side: half,
            parent: Box::new(parent.name),
        };
        if members.len() == 1 {
            let (ix, _, f) = members[0];
            put(
                t,
                tie,
                from_tie,
                name1(EntityKind::Face, node, base),
                ent(ix, EntityKey::Face(f)),
            )?;
            continue;
        }
        // Same-side multiplicity: order along the parent's section
        // line, oriented n_parent × n_tool (both recipe-covariant).
        // Sign-dependent, as in the seam-chain case above: ranks
        // reverse with `dir`. `n_parent` is the parent face's OUTWARD
        // normal (S10 category A, via `face_plane`); `tool_normal` is
        // the split plane's own oriented normal, a recipe parameter
        // carrying no face sense.
        let (_, n_parent) = face_plane(target_body, root)?;
        let dir = n_parent.cross(tool_normal);
        let body = members
            .iter()
            .map(|&(ix, _, _)| sides.iter().find(|s| s.ix == ix))
            .next()
            .flatten()
            .ok_or(NamingError::Emission {
                what: "split fragment group without a side body",
            })?
            .body;
        let extents = members
            .iter()
            .map(|&(_, _, f)| face_extent(body, f, dir))
            .collect::<Result<Vec<_>, _>>()?;
        match order_along(&extents, b)? {
            Some(ranks) => {
                let of = u32::try_from(members.len()).unwrap_or(u32::MAX);
                for (m, rank) in members.iter().zip(ranks) {
                    let mut name = name1(EntityKind::Face, node, base.clone());
                    name.path
                        .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }));
                    put(t, tie, from_tie, name, ent(m.0, EntityKey::Face(m.2)))?;
                }
            }
            None => {
                // Genuine tie (N2): one name, all candidates marked.
                let name = name1(EntityKind::Face, node, base);
                for &(ix, _, f) in &members {
                    tie.push(name.clone(), ent(ix, EntityKey::Face(f)));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Kernel-level exercise of the N3 `Merged` lane (review R4; the
    //! eval-level end-to-end fixture landed with M4 PR 5's declare
    //! threading — see `m4_pr3_names_bool`'s declared-union Merged
    //! pins). The synthetic `merge_groups` rows here keep the
    //! emission-unit coverage: sorted-constituents dedup, and the R8
    //! same-set collision refusing loudly.
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::unreachable
    )]

    use super::*;
    use crate::names::emit_sweep::name_extrude;
    use crate::node::RecipeNodeId;
    use profile::RawLoop;

    #[test]
    fn merged_lane_names_kept_face_with_sorted_deduped_constituents() {
        // A unit-cube extrusion: the "result body" stand-in.
        let plane = profile::SketchPlane::from_frame(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
        );
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| geom_core::Point2::new(x, y)),
        );
        let profile = profile::Profile::new(plane, vec![square])
            .validate(geom_core::Tol::witness().get())
            .unwrap();
        let built = sweep::extrude(&profile, sweep::Extrusion::Distance(1.0_f64)).unwrap();
        let ext_node = RecipeNodeId(1);
        let a_table = name_extrude(ext_node, &built).unwrap();

        // Synthetic merge: the top cap absorbed one lateral — listed
        // TWICE to exercise the dedup.
        let lateral = *a_table
            .iter()
            .find_map(|(n, e)| match (n.path.first(), e) {
                (Some(RoleSeg::Lateral(_)), Entry::Unique(r)) => match r.key {
                    EntityKey::Face(f) => Some(e).map(|_| f),
                    _ => None,
                },
                _ => None,
            })
            .as_ref()
            .unwrap();
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            merge_groups: vec![(built.top, vec![lateral, lateral])],
            ..topo::BooleanNaming::default()
        };
        let empty = NameTable::new();
        let bool_node = RecipeNodeId(9);
        let a = OperandCtx {
            node: ext_node,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId(2),
            table: &empty,
            body: &built.body,
        };
        let t = name_boolean(bool_node, &built.body, &naming, &a, &b).unwrap();

        // Exactly one Merged name, on the kept (top) face, with the
        // TWO deduped constituents in sorted order.
        let merged: Vec<_> = t
            .iter()
            .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
            .collect();
        assert_eq!(merged.len(), 1);
        let (name, entry) = merged[0];
        let Some(RoleSeg::Merged(cs)) = name.path.first() else {
            unreachable!()
        };
        assert_eq!(cs.len(), 2, "constituents must dedup");
        assert!(cs.windows(2).all(|w| w[0] < w[1]), "constituents sorted");
        match entry {
            Entry::Unique(r) => assert_eq!(r.key, EntityKey::Face(built.top)),
            other => panic!("merged entry not unique: {other:?}"),
        }
        // The (synthetically still-live) absorbed lateral keeps its
        // own FromA row; the table stays total over the body.
        assert!(
            t.name_of(&ent(0, EntityKey::Face(lateral))).is_some(),
            "absorbed-but-live lateral must still be covered"
        );
    }

    /// Review R8 (resolved M4 PR 5): two merge groups with the SAME
    /// constituent set — kept faces that are fragments of one operand
    /// face, each absorbing a fragment of one partner — refuse
    /// LOUDLY (typed `NamingError`), never a silent alias.
    #[test]
    fn merged_same_constituent_groups_collide_loudly() {
        let plane = profile::SketchPlane::from_frame(
            geom_core::Point3::new(0.0, 0.0, 0.0),
            geom_core::Vec3::new(1.0, 0.0, 0.0),
            geom_core::Vec3::new(0.0, 1.0, 0.0),
        );
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| geom_core::Point2::new(x, y)),
        );
        let profile = profile::Profile::new(plane, vec![square])
            .validate(geom_core::Tol::witness().get())
            .unwrap();
        let built = sweep::extrude(&profile, sweep::Extrusion::Distance(1.0_f64)).unwrap();
        let ext_node = RecipeNodeId(1);
        let a_table = name_extrude(ext_node, &built).unwrap();
        let laterals: Vec<_> = a_table
            .iter()
            .filter_map(|(n, e)| match (n.path.first(), e) {
                (Some(RoleSeg::Lateral(_)), Entry::Unique(r)) => match r.key {
                    EntityKey::Face(f) => Some(f),
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert!(laterals.len() >= 2);
        // Synthetic descent: `top` reads as a fragment of `bottom`,
        // `laterals[1]` as a fragment of `laterals[0]` — the two
        // groups then share the constituent set
        // {FromA(bottom), FromA(laterals[0])}.
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            face_fragments_a: vec![(built.top, built.bottom), (laterals[1], laterals[0])],
            merge_groups: vec![
                (built.top, vec![laterals[0]]),
                (built.bottom, vec![laterals[1]]),
            ],
            ..topo::BooleanNaming::default()
        };
        let empty = NameTable::new();
        let a = OperandCtx {
            node: ext_node,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId(2),
            table: &empty,
            body: &built.body,
        };
        let err = name_boolean(RecipeNodeId(9), &built.body, &naming, &a, &b)
            .expect_err("same-constituent merge groups must refuse loudly");
        let _ = err; // typed NamingError, never a silent alias
    }
}

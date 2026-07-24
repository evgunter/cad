//! Split/boolean name emission (spec D2/D3): descent-driven — every
//! result entity is chased to its operand parent through the kernels'
//! mint-time rows (`SplitNaming`, `BooleanNaming`, D5 `SplitEdge`
//! provenance), then named as pass-through, `FromA`/`FromB`,
//! fragment (with N2 qualifiers), seam, section, or merged. Nothing
//! is matched; unresolvable descent is a typed error.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom_core::{Decide, Point3, Vec3};
use geom_surfaces::Surface;
use topo::splitting::{PlaneSide, SplitNaming};
use topo::{Body, EdgeKey, FaceKey, Provenance, VertexKey};

use super::discriminate::{Extent, band, order_along, side_of_face};
use super::emit::{
    Incidence, NamingError, edge_ends, ent, face_half_edges, name1, unique_shared_edge,
};
use super::role::{EntityKind, Qualifier, RoleSeg, SplitHalf, StableName};
use super::table::{EntityKey, Entry, NameTable};
use crate::node::RecipeNodeId;

/// One split-side body under naming.
struct Side<'a, T: Decide> {
    body: &'a Body<T>,
    ix: u32,
    half: SplitHalf,
}

/// The operand-face plane (result carriers are the N2 references).
fn face_plane<T: Decide>(body: &Body<T>, f: FaceKey) -> Result<(Point3<T>, Vec3<T>), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let face = body
        .get_face(f)
        .ok_or_else(|| bug("face_plane: dangling"))?;
    match body
        .get_surface(face.surface)
        .ok_or_else(|| bug("face_plane: dangling surface"))?
    {
        Surface::Plane { origin, normal, .. } => Ok((*origin, *normal)),
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

/// The unique upstream name of an entity, refusing tied upstream
/// entries (deferred — reported) and missing rows loudly.
fn upstream_name(
    table: &NameTable,
    node: RecipeNodeId,
    e: super::table::EntityRef,
) -> Result<StableName, NamingError> {
    let name = table
        .name_of(&e)
        .ok_or(NamingError::MissingUpstream { node })?;
    match table.lookup(name) {
        Some(Entry::Unique(_)) => Ok(name.clone()),
        _ => Err(NamingError::Emission {
            what: "tied upstream entry through a downstream op — deferred (reported)",
        }),
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

    name_split_faces(
        node,
        &mut t,
        &sides,
        &frag_rows,
        &section_keys,
        target_node,
        target_table,
        target_body,
        tool_normal,
        b,
    )?;
    name_split_edges_vertices(
        node,
        &mut t,
        &sides,
        &frag_rows,
        &section_keys,
        naming,
        target_node,
        target_table,
    )?;

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
        // Chord edges named by the operand face their section
        // boundary runs across; per-(face) multiplicity ordered by
        // completion adjacency is NOT available — multiple chords on
        // one operand face use the same carrier order as face
        // fragments. v1 corpus: one chord per (face, side); more is
        // a typed refusal (reported).
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
            if edges.len() > 1 {
                return Err(bug(
                    "multiple section chords across one operand face — deferred (reported)",
                ));
            }
            t.insert(
                name1(
                    EntityKind::Edge,
                    node,
                    RoleSeg::SectionEdge {
                        side: s.half,
                        face: Box::new(parent),
                    },
                ),
                ent(s.ix, EntityKey::Edge(edges[0])),
            )?;
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
                let name = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(e)))?;
                t.insert(name, ent(s.ix, EntityKey::Edge(e)))?;
                continue;
            }
            if target_table
                .name_of(&ent(0, EntityKey::Edge(root)))
                .is_none()
            {
                return Err(bug("edge descent reached no operand edge"));
            }
            let parent = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(root)))?;
            t.insert(
                name1(
                    EntityKind::Edge,
                    node,
                    RoleSeg::SplitFragment {
                        side: s.half,
                        parent: Box::new(parent),
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
            let is_pair_member =
                copy_to_original.contains_key(&v) || pair_originals.contains(&v);
            if !is_pair_member
                && target_table
                    .name_of(&ent(0, EntityKey::Vertex(v)))
                    .is_some()
            {
                let name = upstream_name(target_table, target_node, ent(0, EntityKey::Vertex(v)))?;
                t.insert(name, ent(s.ix, EntityKey::Vertex(v)))?;
                continue;
            }
            // Resolve the birth record — directly, or through the
            // null-pair copy row (the original's record lives in
            // whichever side kept it).
            let src = copy_to_original.get(&v).copied().unwrap_or(v);
            let parent_edge = sides.iter().find_map(|sb| {
                match sb.body.vertex_provenance_of(src) {
                    Some(Provenance::SplitEdge { edge }) => Some(chase_edge_to_table(
                        sb.body,
                        target_table,
                        *edge,
                        sb.body.edges().count(),
                    )),
                    _ => None,
                }
            });
            let seg = if let Some(parent_edge) = parent_edge {
                // Crossing vertex: minted where the plane crossed an
                // operand edge's interior.
                let parent = upstream_name(
                    target_table,
                    target_node,
                    ent(0, EntityKey::Edge(parent_edge)),
                )?;
                RoleSeg::CrossingVertex {
                    side: s.half,
                    edge: Box::new(parent),
                }
            } else if target_table
                .name_of(&ent(0, EntityKey::Vertex(src)))
                .is_some()
            {
                // The plane passed THROUGH an operand vertex (review
                // R2): side-tagged pass-through — operand identity
                // from the pair row, side from body membership (a
                // recorded verdict).
                let of =
                    upstream_name(target_table, target_node, ent(0, EntityKey::Vertex(src)))?;
                RoleSeg::OnToolVertex {
                    side: s.half,
                    of: Box::new(of),
                }
            } else {
                return Err(bug(
                    "on-plane vertex with neither a SplitEdge record nor an operand identity",
                ));
            };
            t.insert(
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
    let operand_face_name = |d: Descent| -> Result<StableName, NamingError> {
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
    let mut handled: BTreeSet<FaceKey> = BTreeSet::new();
    for (kept, absorbed) in &naming.merge_groups {
        if body.get_face(*kept).is_none() {
            return Err(bug("merge kept face not live"));
        }
        let mut constituents = Vec::new();
        for &c in core::iter::once(kept).chain(absorbed) {
            let d = descend_face(c)?;
            constituents.push(wrap(d, operand_face_name(d)?, EntityKind::Face));
        }
        constituents.sort_unstable();
        constituents.dedup();
        t.insert(
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
        let base = wrap(d, root_name, EntityKind::Face);
        if members.len() == 1 {
            t.insert(base, ent(0, EntityKey::Face(members[0])))?;
            continue;
        }
        name_fragment_group(
            &mut t,
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

    name_boolean_edges(
        node,
        &mut t,
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
        bnd,
    )?;
    name_boolean_vertices(
        node,
        &mut t,
        body,
        naming,
        &inv_vertices,
        a,
        b,
        &seam_set,
        &inc,
        bnd,
    )?;

    super::emit::check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// Qualifies a multi-fragment descent group (N2): sign vectors of
/// `name_frag_side_of` against the seam partners' carriers; equal
/// vectors tie.
#[allow(clippy::too_many_arguments)]
fn name_fragment_group<T: Decide>(
    t: &mut NameTable,
    body: &Body<T>,
    base: &StableName,
    members: &[FaceKey],
    seam_set: &BTreeSet<EdgeKey>,
    inc: &Incidence,
    descend_face: &impl Fn(FaceKey) -> Result<Descent, NamingError>,
    operand_face_name: &impl Fn(Descent) -> Result<StableName, NamingError>,
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
                    partners.entry(operand_face_name(d)?).or_insert(other);
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
            t.insert(name, ent(0, EntityKey::Face(faces[0])))?;
        } else {
            // The N2 tie: equally-admissible symmetric candidates.
            let ents = faces.iter().map(|&f| ent(0, EntityKey::Face(f))).collect();
            t.insert_tied(name, ents)?;
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
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    inv_edges: &BTreeMap<EdgeKey, EdgeKey>,
    fwd_edges: &BTreeMap<EdgeKey, EdgeKey>,
    seam_set: &BTreeSet<EdgeKey>,
    inc: &Incidence,
    descend_face: &impl Fn(FaceKey) -> Result<Descent, NamingError>,
    operand_face_name: &impl Fn(Descent) -> Result<StableName, NamingError>,
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
        Cross(StableName, StableName),
        SameA(EdgeKey),
        SameB(EdgeKey),
    }
    let chord_kind = |e: EdgeKey| -> Result<ChordKind, NamingError> {
        let faces = inc
            .edge_faces
            .get(&e)
            .ok_or_else(|| bug("seam edge without adjacent faces"))?;
        if faces.len() != 2 {
            return Err(bug("seam edge without exactly two adjacent faces"));
        }
        let (d0, d1) = (descend_face(faces[0])?, descend_face(faces[1])?);
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
    let seam_pair = |e: EdgeKey| -> Result<(StableName, StableName), NamingError> {
        match chord_kind(e)? {
            ChordKind::Cross(fa, fb) => Ok((fa, fb)),
            _ => Err(bug("seam edge between same-operand faces")),
        }
    };
    let mut seam_groups: BTreeMap<(StableName, StableName), Vec<EdgeKey>> = BTreeMap::new();
    for &e in &naming.seam_edges {
        if body.get_edge(e).is_none() {
            continue; // consumed by the merge stage — historical row
        }
        seam_groups.entry(seam_pair(e)?).or_default().push(e);
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
                None => ERoot::A(chase_edge_to_table(
                    body,
                    a.table,
                    e,
                    body.edges().count(),
                )),
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
                    seam_groups.entry((fa, fb)).or_default().push(e);
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
    for ((fa, fb), edges) in seam_groups {
        let base = name1(
            EntityKind::Edge,
            node,
            RoleSeg::Seam {
                a: Box::new(fa),
                b: Box::new(fb),
            },
        );
        if edges.len() == 1 {
            t.insert(base, ent(0, EntityKey::Edge(edges[0])))?;
            continue;
        }
        // Collinear chain: order along the pair's intersection line,
        // oriented n_a × n_b (the carriers' own orientations, A side
        // first — descent decides which is which, never list order).
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
        insert_ranked_or_tied(t, base, &edges, &extents, bnd, |&e| {
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
        let seg = if wrap_a {
            RoleSeg::FromA(Box::new(inner))
        } else {
            RoleSeg::FromB(Box::new(inner))
        };
        let base = name1(EntityKind::Edge, node, seg);
        if edges.len() == 1 {
            t.insert(base, ent(0, EntityKey::Edge(edges[0])))?;
            continue;
        }
        // Sub-edge chain: order along the parent edge's own oriented
        // carrier (operand geometry).
        let dir = edge_dir(op_body, root_key)?;
        let extents = edges
            .iter()
            .map(|&e| edge_extent(body, e, dir))
            .collect::<Result<Vec<_>, _>>()?;
        insert_ranked_or_tied(t, base, &edges, &extents, bnd, |&e| {
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
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    inv_vertices: &BTreeMap<VertexKey, VertexKey>,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    seam_set: &BTreeSet<EdgeKey>,
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
    let operand_identity = |k: VertexKey| -> Result<Option<StableName>, NamingError> {
        if let Some(&vb) = inv_vertices.get(&k) {
            if b.table.name_of(&ent(0, EntityKey::Vertex(vb))).is_some() {
                let inner = upstream_name(b.table, b.node, ent(0, EntityKey::Vertex(vb)))?;
                return Ok(Some(name1(
                    EntityKind::Vertex,
                    node,
                    RoleSeg::FromB(Box::new(inner)),
                )));
            }
        } else if a.table.name_of(&ent(0, EntityKey::Vertex(k))).is_some() {
            let inner = upstream_name(a.table, a.node, ent(0, EntityKey::Vertex(k)))?;
            return Ok(Some(name1(
                EntityKind::Vertex,
                node,
                RoleSeg::FromA(Box::new(inner)),
            )));
        }
        Ok(None)
    };
    // Candidate seam-vertex names, grouped for multiplicity.
    let mut groups: BTreeMap<(StableName, StableName), Vec<VertexKey>> = BTreeMap::new();
    for (v, _) in body.vertices() {
        // Operand pass-downs: the kept key itself, then its dead
        // fusion partners (deterministic order: A-side identity wins
        // when both operands fused here — documented choice, the
        // result arena keeps A's key space).
        let mut identity = operand_identity(v)?;
        if identity.is_none() {
            for &dead in fused.get(&v).into_iter().flatten() {
                identity = operand_identity(dead)?;
                if identity.is_some() {
                    break;
                }
            }
        }
        if let Some(name) = identity {
            t.insert(name, ent(0, EntityKey::Vertex(v)))?;
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
        for &e in edges {
            let Some(ename) = t.name_of(&ent(0, EntityKey::Edge(e))) else {
                return Err(bug("seam vertex incident to an unnamed edge"));
            };
            match ename.path.first() {
                Some(RoleSeg::FromA(x)) => a_edges.push((**x).clone()),
                Some(RoleSeg::FromB(x)) => b_edges.push((**x).clone()),
                Some(RoleSeg::Seam { a: fa, b: fb }) if seam_set.contains(&e) => {
                    a_faces.push((**fa).clone());
                    b_faces.push((**fb).clone());
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
        let partner_b_inner: Option<StableName> = va_key
            .and_then(|k| rc.vv.iter().find(|r| r.a == k).map(|r| r.b))
            .and_then(|pb| upstream_name(b.table, b.node, ent(0, EntityKey::Vertex(pb))).ok());
        let partner_a_inner: Option<StableName> = vb_key
            .and_then(|k| rc.vv.iter().find(|r| r.b == k).map(|r| r.a))
            .and_then(|pa| upstream_name(a.table, a.node, ent(0, EntityKey::Vertex(pa))).ok());
        let pair = match (a_edges.as_slice(), b_edges.as_slice()) {
            ([ae], [be]) => (ae.clone(), be.clone()),
            ([ae], []) if b_faces.len() == 1 => (ae.clone(), b_faces[0].clone()),
            ([], [be]) if a_faces.len() == 1 => (a_faces[0].clone(), be.clone()),
            ([ae], []) if partner_b_inner.is_some() => (
                ae.clone(),
                partner_b_inner.clone().unwrap_or_else(|| ae.clone()),
            ),
            ([], [be]) if partner_a_inner.is_some() => (
                partner_a_inner.clone().unwrap_or_else(|| be.clone()),
                be.clone(),
            ),
            _ => {
                return Err(bug(
                    "seam vertex parentage underdetermined from incident edges",
                ));
            }
        };
        groups.entry(pair).or_default().push(v);
    }
    for ((pa, pb), verts) in groups {
        let base = name1(
            EntityKind::Vertex,
            node,
            RoleSeg::Seam {
                a: Box::new(pa.clone()),
                b: Box::new(pb.clone()),
            },
        );
        if verts.len() == 1 {
            t.insert(base, ent(0, EntityKey::Vertex(verts[0])))?;
            continue;
        }
        // Same pair crossing more than once: order along the edge
        // parent's own carrier (prefer the A side).
        let carrier = resolve_edge_carrier(&pa, a).or_else(|| resolve_edge_carrier(&pb, b));
        let Some(dir) = carrier else {
            let ents = verts
                .iter()
                .map(|&v| ent(0, EntityKey::Vertex(v)))
                .collect();
            t.insert_tied(base, ents)?;
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
        insert_ranked_or_tied(t, base, &verts, &extents, bnd, |&v| {
            ent(0, EntityKey::Vertex(v))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Kernel-level exercise of the N3 `Merged` lane (review R4): no
    //! eval-level path can mint an F7 merge until PR 5's declare
    //! threading (glued unions refuse `UnpairedLooseEnds`), so the
    //! merge-group naming lane is driven directly with a synthetic
    //! `merge_groups` row over a real extrusion — including the
    //! sorted-constituents dedup.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::emit_sweep::name_extrude;
    use crate::node::RecipeNodeId;

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
            .validate(geom_core::Tolerance::get())
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
fn insert_ranked_or_tied<T: Decide, K: Copy>(
    t: &mut NameTable,
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
                t.insert(name, to_ent(k))?;
            }
        }
        None => {
            let ents = keys.iter().map(to_ent).collect();
            t.insert_tied(base, ents)?;
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
                let name = upstream_name(target_table, target_node, ent(0, EntityKey::Face(f)))?;
                t.insert(name, ent(s.ix, EntityKey::Face(f)))?;
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
        let half = members[0].1;
        let base = RoleSeg::SplitFragment {
            side: half,
            parent: Box::new(parent),
        };
        if members.len() == 1 {
            let (ix, _, f) = members[0];
            t.insert(
                name1(EntityKind::Face, node, base),
                ent(ix, EntityKey::Face(f)),
            )?;
            continue;
        }
        // Same-side multiplicity: order along the parent's section
        // line, oriented n_parent × n_tool (both recipe-covariant).
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
                    t.insert(name, ent(m.0, EntityKey::Face(m.2)))?;
                }
            }
            None => {
                // Genuine tie (N2): one name, all candidates marked.
                let ents = members
                    .iter()
                    .map(|&(ix, _, f)| ent(ix, EntityKey::Face(f)))
                    .collect();
                t.insert_tied(name1(EntityKind::Face, node, base), ents)?;
            }
        }
    }
    Ok(())
}

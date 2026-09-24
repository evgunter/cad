//! Split/boolean name emission (spec D2/D3): descent-driven — every
//! result entity is chased to its operand parent through the kernels'
//! mint-time rows (`SplitNaming`, `BooleanNaming`, D5 `SplitEdge`
//! provenance), then named as pass-through, `FromA`/`FromB`,
//! fragment (with N2 qualifiers), seam, section, or merged. Nothing
//! is matched; unresolvable descent is a typed error.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom::Surface;
use geom_brep::OutwardNormal;
use geom_core::k_stats::decide;
use geom_core::{Decide, Margin, Point3, Sign, Vec3};
use topo::splitting::{PlaneSide, SplitNaming};
use topo::{Body, EdgeKey, FaceKey, Provenance, VertexKey};

use super::canonical;
use super::defer::{TieRows, Upstream, put, upstream_name};
use super::discriminate::{CHORD_ON_RIM, Extent, band, order_along, side_of_face};
use super::emit::{
    Incidence, NamingError, Rim, edge_ends, ent, face_half_edges, name1, rim_between,
};
use super::merged::{self, NESTED_MERGED};
use super::role::{EntityKind, NameRef, Qualifier, RoleSeg, SplitHalf, StableName};
use super::seam_pair;
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
/// S10 CATEGORY A: the returned normal is the face's outward normal —
/// the chart normal with `Face::sense` folded in through
/// [`OutwardNormal::from_chart`], unwrapped at this door because every
/// consumer reads it as geometry — not the raw chart normal.
/// Every consumer uses the direction as an *oriented reference* whose
/// sign lands in a stable name — [`side_of_face`] turns it into a
/// `Qualifier::SideOf` verdict vector, and `n_a × n_b` orients the
/// carrier [`order_along`] ranks fragments along
/// (`Qualifier::OrderAlong`). Reading the chart normal raw would let a
/// sense flip silently swap Positive↔Negative and reverse every rank,
/// renaming fragments that did not move: an N4 covariance break, since
/// a face's orientation sense is part of the geometry names are
/// covariant *with*, not a private encoding detail the naming layer
/// may ignore. The fold is exact structure (a `bool` selecting a
/// negation), so no new numeric decision enters here, and every face
/// this build mints has `sense: true` — the fold is the identity and
/// no name moves.
pub(super) fn face_plane<T: Decide>(
    body: &Body<T>,
    f: FaceKey,
) -> Result<(Point3<T>, Vec3<T>), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let face = body
        .get_face(f)
        .ok_or_else(|| bug("face_plane: dangling"))?;
    match body
        .get_surface(face.surface)
        .ok_or_else(|| bug("face_plane: dangling surface"))?
    {
        Surface::Plane { origin, normal, .. } => Ok((
            *origin,
            OutwardNormal::from_chart(*normal, face.sense).vec(),
        )),
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

/// Chases a face key through fragment rows to its root (the key that
/// is not itself a minted fragment). Bounded by the row count.
///
/// # Errors
///
/// A spent budget means the walk revisited a key, which is a corrupt
/// mint-time record — [`NamingError::FragmentLineage`], carrying the
/// face this chase was asked about. The root becomes a GROUP key and
/// is handed to [`upstream_name`], so falling out of the loop with the
/// cursor would name faces after a stranger instead of refusing.
/// Guarded by `a_cycling_fragment_map_refuses` below.
fn chase(rows: &BTreeMap<FaceKey, FaceKey>, f: FaceKey) -> Result<FaceKey, NamingError> {
    let mut at = f;
    for _ in 0..=rows.len() {
        match rows.get(&at) {
            Some(&p) => at = p,
            None => return Ok(at),
        }
    }
    Err(NamingError::FragmentLineage { face: f })
}

/// Chases an edge through `SplitEdge` birth records
/// (`Body::split_root`), STOPPING at the first key the operand's table
/// names: the table is the identity boundary — records deeper than the
/// operand's own entities belong to earlier ops (a union's rim fragment
/// must not chase past its own union-level name into its grand-parent).
///
/// # Errors
///
/// A cycling lineage is a corrupt record, surfaced as an emission bug
/// — [`NamingError::SplitLineage`], carrying the cycling edge, which
/// is the only locator this failure has.
///
/// **Unguardable from this crate, and the reason is WRITER ACCESS,
/// not a survey of call sites.** This walk advances only on
/// `Body::edge_provenance`, which is `pub(crate)` to `topo`; the one
/// door that writes a `SplitEdge` record is `Body::split_edge`, and it
/// records the parent on the child it has just minted, so every record
/// points at a key that already existed and a chain is strictly
/// decreasing in age. Nothing outside `topo` can close it. That a
/// cycling lineage exists at all is real — a graft copies these
/// records with their SOURCE keys and they chain into strangers in the
/// destination (`work/bool/graft-copies-provenance-keys-verbatim.md`
/// records `Body::split_root`'s cycle arm firing on real assembly
/// products) — but the aliasing is `topo`-internal and this crate has
/// no door to it. `chase_b` below is NOT covered by this argument and
/// is guarded instead: it hops through a caller-supplied graft map
/// between provenance reads, which is enough to close a loop that
/// `split_edge`'s records alone cannot.
fn chase_edge_to_table<T: Decide>(
    body: &Body<T>,
    table: &NameTable,
    e: EdgeKey,
) -> Result<EdgeKey, NamingError> {
    Ok(body.split_root(e, |k| table.name_of(&ent(0, EntityKey::Edge(k))).is_some())?)
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
    tol: Tol,
) -> Result<Arc<NameTable>, NamingError> {
    let b = band(tol)?;
    let mut t = NameTable::new();
    let frag_rows: BTreeMap<FaceKey, FaceKey> = naming.face_fragments.iter().copied().collect();
    let section_keys: BTreeSet<FaceKey> = naming.sections.iter().map(|&(f, _)| f).collect();
    let mut sides: Vec<Side<'_, T>> = Vec::new();
    for (half, body) in [(SplitHalf::Above, above), (SplitHalf::Below, below)] {
        if let Some(body) = body {
            sides.push(Side {
                body,
                ix: half.output_body(),
                half,
            });
        }
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
        // The per-side counter is indexed by the half's OUTPUT-BODY
        // index — the one mapping, not a second one.
        let slot = usize::try_from(half.output_body()).map_err(|_| NamingError::Emission {
            what: "a split half's output-body index exceeds usize",
        })?;
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

/// Chord edges: every boundary edge of `body`'s live section faces,
/// with the operand face across it.
///
/// The walk is half-edge → mate → mate's loop → face, by hand, because
/// each hop refuses in its own words; `Body::face_of_half_edge` answers
/// the last two hops as one `None`. `walk_tests` in [`super::emit`]
/// drives each refusal through a body with that hop's entity removed.
pub(super) fn chord_faces<T: geom_core::Real>(
    body: &Body<T>,
    sections: &[(FaceKey, PlaneSide)],
    section_keys: &BTreeSet<FaceKey>,
) -> Result<BTreeMap<EdgeKey, FaceKey>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let mut chord_faces: BTreeMap<EdgeKey, FaceKey> = BTreeMap::new();
    for &(sf, _) in sections {
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
    Ok(chord_faces)
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
        let chord_faces = chord_faces(body, &naming.sections, section_keys)?;
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
            let root = chase(frag_rows, other)?;
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
                    face: parent.name,
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
                    divided_edges.insert(chase_edge_to_table(sb.body, target_table, e)?);
                }
            }
        }
        for (e, _) in body.edges() {
            if chord_faces.contains_key(&e) {
                continue;
            }
            let root = chase_edge_to_table(body, target_table, e)?;
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
                        parent: parent.name,
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
                    Some(Provenance::SplitEdge { edge }) => {
                        Some(chase_edge_to_table(sb.body, target_table, *edge))
                    }
                    _ => None,
                })
                .transpose()?;
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
                        edge: parent.name,
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
                        of: of.name,
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

/// Which operand a boolean result's key belongs to, carrying the key in
/// that operand's space — the one side type the face, edge and vertex
/// passes all speak. The side picks the table and body a key is read
/// against ([`OpSide::of`]); nothing downstream re-derives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum OpSide<K> {
    A(K),
    B(K),
}

impl<K: Copy> OpSide<K> {
    /// The operand this key is read against, and the key in its space.
    fn of<'o, 'a, T: Decide>(
        self,
        a: &'o OperandCtx<'a, T>,
        b: &'o OperandCtx<'a, T>,
    ) -> (&'o OperandCtx<'a, T>, K) {
        match self {
            OpSide::A(k) => (a, k),
            OpSide::B(k) => (b, k),
        }
    }

    /// The same side, carrying another key.
    fn with<J>(self, j: J) -> OpSide<J> {
        match self {
            OpSide::A(_) => OpSide::A(j),
            OpSide::B(_) => OpSide::B(j),
        }
    }

    /// The other side, carrying the same key.
    fn other(self) -> Self {
        match self {
            OpSide::A(k) => OpSide::B(k),
            OpSide::B(k) => OpSide::A(k),
        }
    }

    /// The `FromA` / `FromB` segment wrapping a name read on this side.
    fn wrap(self, inner: NameRef) -> RoleSeg {
        match self {
            OpSide::A(_) => RoleSeg::FromA(inner),
            OpSide::B(_) => RoleSeg::FromB(inner),
        }
    }
}

/// Where an operand-space key returned by [`operand_key`] lives.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum KeySpace {
    /// The operand's clone IS the result arena: the key is an arena
    /// key, and its lineage is the arena's own provenance.
    Arena,
    /// The operand was grafted in: the key is the graft's SOURCE key,
    /// read back through the graft rows.
    Graft,
}

/// **The one read of a boolean result's key layout**: which operand a
/// result-arena key belongs to, its key in that operand's space, and
/// where that key lives. A result key is an A key only where A's clone
/// is the arena, and a B key either through the graft rows (`inv`,
/// destination → source) or because B's clone is the arena itself — so
/// "not grafted" means "A" only in the grafted layout. Every pass reads
/// keys through here, and the side and its key space come out of one
/// match rather than being re-derived from the layout.
fn operand_key<K: Copy + Ord>(
    naming: &topo::BooleanNaming,
    inv: &BTreeMap<K, K>,
    k: K,
) -> Result<(OpSide<K>, KeySpace), NamingError> {
    use topo::OperandKeys;
    match (naming.a_keys, naming.b_keys) {
        (OperandKeys::Direct, OperandKeys::Grafted) => Ok(match inv.get(&k) {
            Some(&kb) => (OpSide::B(kb), KeySpace::Graft),
            None => (OpSide::A(k), KeySpace::Arena),
        }),
        (OperandKeys::Direct, OperandKeys::Absent) => Ok((OpSide::A(k), KeySpace::Arena)),
        (OperandKeys::Absent, OperandKeys::Direct) => Ok((OpSide::B(k), KeySpace::Arena)),
        _ => Err(NamingError::Emission {
            what: "unsupported operand-key layout",
        }),
    }
}

/// Names a boolean result (spec D2's boolean vocabulary; N2/N3).
pub(crate) fn name_boolean<T: Decide>(
    node: RecipeNodeId,
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    tol: Tol,
) -> Result<Arc<NameTable>, NamingError> {
    let bnd = band(tol)?;
    let bug = |what| NamingError::Emission { what };
    let mut t = NameTable::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;

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
    // right key space). The key the B arm chases — and therefore the
    // key a refusal here renders — is a B-CLONE key, not a key of the
    // result arena: the rows are `face_fragments_b`, minted before the
    // clone was grafted. A reader comparing the rendered key against
    // the result body will not find it.
    let descend_face = |f: FaceKey| -> Result<OpSide<FaceKey>, NamingError> {
        Ok(match operand_key(naming, &inv_faces, f)?.0 {
            OpSide::A(fa) => OpSide::A(chase(&a_rows, fa)?),
            OpSide::B(fb) => OpSide::B(chase(&b_rows, fb)?),
        })
    };
    let operand_face_name = |d: OpSide<FaceKey>| -> Result<Upstream, NamingError> {
        let (op, f) = d.of(a, b);
        upstream_name(op.table, op.node, ent(0, EntityKey::Face(f)))
    };

    // ---- Faces: merges first (N3), then descent groups. ----
    let mut tie = TieRows::default();
    let mut handled: BTreeSet<FaceKey> = BTreeSet::new();
    // Kept face → constituent descents (M4 PR 5: the seam-edge walk
    // reads THROUGH a merged face to its mint-time operand identity).
    let mut merged_descents: BTreeMap<FaceKey, Vec<OpSide<FaceKey>>> = BTreeMap::new();
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
            // The constituent set is FLAT (N3), decided here: an
            // operand face that is a merged face, read through its
            // descent wrappers, contributes its constituents
            // (re-wrapped by that chain, then by this side) and never
            // its `Merged` name.
            match merged::constituents_through_wrappers(&up.name) {
                Some(cs) => constituents.extend(
                    cs.into_iter()
                        .map(|inner| name1(EntityKind::Face, node, d.wrap(NameRef::new(inner)))),
                ),
                None => constituents.push(name1(EntityKind::Face, node, d.wrap(up.name))),
            }
        }
        // The kernel's guarantee that the set is flat, held here for
        // both emitters: a constituent that is itself a merged face
        // (through any wrapping) is a name this loop cannot have
        // built from a flat operand, so it is refused rather than
        // published. `emit_union`'s `collapse` reads the same rule at
        // the union's second door; the two are one rule at two doors.
        if constituents
            .iter()
            .any(|c| merged::constituents_through_wrappers(c).is_some())
        {
            return Err(bug(NESTED_MERGED));
        }
        merged_descents.insert(*kept, descents);
        // The constituent SET is the name (review R8), so the
        // canonical form sorts and deduplicates it: two merge groups
        // with one set collide LOUDLY at insert (`DuplicateName` →
        // typed `NamingError`; pinned by
        // `merged_same_constituent_groups_collide_loudly`). With the
        // set flat, two groups collide whenever they list the same
        // faces — a merge over a merged face and a merge over that
        // face's constituents are ONE set, where nesting once kept
        // them apart — and a per-group discriminator is what would
        // upgrade the refusal to a success if that class ever matters.
        put(
            &mut t,
            &mut tie,
            from_tie,
            canonical::minted(name1(EntityKind::Face, node, RoleSeg::Merged(constituents))),
            ent(0, EntityKey::Face(*kept)),
        )?;
        handled.insert(*kept);
    }
    let mut groups: BTreeMap<OpSide<FaceKey>, Vec<FaceKey>> = BTreeMap::new();
    for (f, _) in body.faces() {
        if !handled.contains(&f) {
            groups.entry(descend_face(f)?).or_default().push(f);
        }
    }
    for (d, members) in groups {
        let root_name = operand_face_name(d)?;
        let from_tie = root_name.tied;
        let base = name1(EntityKind::Face, node, d.wrap(root_name.name));
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
    descend_face: &impl Fn(FaceKey) -> Result<OpSide<FaceKey>, NamingError>,
    operand_face_name: &impl Fn(OpSide<FaceKey>) -> Result<Upstream, NamingError>,
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };
    // Partners: operand faces across the members' seam edges.
    let mut partners: BTreeMap<NameRef, FaceKey> = BTreeMap::new();
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
            vector.push(((**pname).clone(), verdict));
        }
        by_vector.entry(vector).or_default().push(m);
    }
    for (vector, faces) in by_vector {
        let mut name = base.clone();
        name.path.push(RoleSeg::Fragment(Qualifier::SideOf(vector)));
        let name = canonical::minted(name);
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
    descend_face: &impl Fn(FaceKey) -> Result<OpSide<FaceKey>, NamingError>,
    operand_face_name: &impl Fn(OpSide<FaceKey>) -> Result<Upstream, NamingError>,
    merged_descents: &BTreeMap<FaceKey, Vec<OpSide<FaceKey>>>,
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };

    // ---- Seam edges (zip-listed AND derived — see below), grouped
    // by their (fA, fB) operand pair. A derived chord between two
    // SAME-operand faces (the collinear channel-cut lane re-mints a
    // sub-edge of an operand edge as a chord) descends instead to an
    // operand edge its two parent faces share — combinatorial adjacency
    // of emitted anchors, not matching. That the pair shares EXACTLY
    // ONE is a guess, not a property: a later member splitting a merged
    // face refutes it, and `NamingError::SharedRim` is what the arm
    // below says when it does. ----
    enum ChordKind {
        Cross(Upstream, Upstream),
        SameA(EdgeKey),
        SameB(EdgeKey),
    }
    // A face's descent for CHORD purposes: a plain face descends as
    // itself; a MERGED face (M4 PR 5, N3 live) reads through to its
    // unique constituent on side `want` — the seam's mint-time operand
    // identity survives the glue. Several same-side constituents
    // refuse.
    let chord_descent = |f: FaceKey, want: OpSide<()>| -> Result<OpSide<FaceKey>, NamingError> {
        let Some(ds) = merged_descents.get(&f) else {
            return descend_face(f);
        };
        // Constituent fragments of ONE operand face share a
        // descent — dedup before the uniqueness demand.
        let mut hits: Vec<OpSide<FaceKey>> =
            ds.iter().filter(|d| d.with(()) == want).copied().collect();
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
    // **A chord between two MERGED faces.** Each face has a constituent
    // on both sides, so neither face says which side to read through
    // to. `own` is the side the chord's own key descends to, and the
    // chord is read through to THAT side's two constituents and named
    // as the rim they share — but the key is only where to look, not
    // why the answer holds: the join mints keys on both sides, and a
    // B-clone result keys every chord as B's. What makes the answer
    // right is geometric. Two planar constituents of one operand meet
    // along their shared rim's LINE, so a chord lying on both lies on
    // that line; it is that rim's piece exactly when it also lies
    // WITHIN the rim. [`chord_on_rim`] checks both before the name is
    // given, and a chord that fails it — or one with no key side, a
    // zip-listed edge — refuses as the missing rule it is
    // (`NamingError::MergedChordOffRim`, `NamingError::MergedChord`).
    let chord_kind = |e: EdgeKey, own: Option<OpSide<()>>| -> Result<ChordKind, NamingError> {
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
                (chord_descent(faces[0], d1.with(()).other())?, d1)
            }
            (false, true) => {
                let d0 = descend_face(faces[0])?;
                (d0, chord_descent(faces[1], d0.with(()).other())?)
            }
            (true, true) => {
                let Some(side) = own else {
                    return Err(NamingError::MergedChord { edge: e });
                };
                let (d0, d1) = (
                    chord_descent(faces[0], side)?,
                    chord_descent(faces[1], side)?,
                );
                let (op, f0) = d0.of(a, b);
                let (_, f1) = d1.of(a, b);
                return match rim_between(op.body, f0, f1)? {
                    Rim::One(rim) if chord_on_rim(body, e, op.body, rim, bnd)? => Ok(match side {
                        OpSide::A(()) => ChordKind::SameA(rim),
                        OpSide::B(()) => ChordKind::SameB(rim),
                    }),
                    Rim::One(rim) => Err(NamingError::MergedChordOffRim {
                        edge: e,
                        node: op.node,
                        rim,
                    }),
                    Rim::NotOne(found) => Err(NamingError::SharedRim {
                        node: op.node,
                        face: f0,
                        other: f1,
                        found,
                    }),
                };
            }
        };
        Ok(match (d0, d1) {
            (OpSide::A(_), OpSide::B(_)) => {
                ChordKind::Cross(operand_face_name(d0)?, operand_face_name(d1)?)
            }
            (OpSide::B(_), OpSide::A(_)) => {
                ChordKind::Cross(operand_face_name(d1)?, operand_face_name(d0)?)
            }
            // The premise this caller is asking under: it did not
            // build these bodies, it DESCENDED two result faces into
            // one of them and guesses the pair carries this chord's
            // rim. A pair that turns out not to have one rim refutes
            // the guess, not the body — so the answer is classified
            // here rather than at the walk.
            (OpSide::A(fa0), OpSide::A(fa1)) => match rim_between(a.body, fa0, fa1)? {
                Rim::One(e) => ChordKind::SameA(e),
                Rim::NotOne(found) => {
                    return Err(NamingError::SharedRim {
                        node: a.node,
                        face: fa0,
                        other: fa1,
                        found,
                    });
                }
            },
            (OpSide::B(fb0), OpSide::B(fb1)) => match rim_between(b.body, fb0, fb1)? {
                Rim::One(e) => ChordKind::SameB(e),
                Rim::NotOne(found) => {
                    return Err(NamingError::SharedRim {
                        node: b.node,
                        face: fb0,
                        other: fb1,
                        found,
                    });
                }
            },
        })
    };
    let seam_pair = |e: EdgeKey| -> Result<(Upstream, Upstream), NamingError> {
        // A zip-listed seam edge is the join's own, so it has no side
        // to read a both-merged pair through to.
        match chord_kind(e, None)? {
            ChordKind::Cross(fa, fb) => Ok((fa, fb)),
            _ => Err(bug("seam edge between same-operand faces")),
        }
    };
    // Group value: (descends-from-a-tie, edges). Two tied operand
    // faces answer to ONE name, so their seam chords land in one
    // group — the widening B1 asks for, not a collision.
    let mut seam_groups: BTreeMap<(NameRef, NameRef), (bool, Vec<EdgeKey>)> = BTreeMap::new();
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

    // ---- Operand-descended edges, grouped by (side, root). ----
    // Best-effort descent for a GRAFTED B side: stops at the first key B's table
    // names. A broken chain (a middle fragment of a doubly-pierced
    // edge dies PRE-graft, so its key is unnamed AND ungrafted)
    // returns the non-resolving key instead of refusing — the
    // `resolves == false` route below hands such edges to
    // `chord_kind`, the same rescue the A lane gets (review R1: lane
    // parity; the kernel completed the body, naming must too).
    //
    // This is NOT `Body::split_root`, deliberately: the result body's
    // records for grafted edges carry B-space keys verbatim (the graft
    // copies provenance without forwarding), and this lane depends on
    // that: B's table names ancestors that died in B
    // before the graft, and the verbatim key is the only way back to
    // them. B's operand body cannot be chased instead — it is not the
    // body that was grafted (a placed copy is). Forwarding `SplitEdge`
    // at the graft therefore needs a dead-ancestor bridge on the
    // `GraftMap` before this descent can become the shared chase.
    //
    // Spending the budget means the walk revisited a key — the same
    // corrupt record `chase_edge_to_table` refuses, on the lane where
    // the aliasing above can actually produce one — so it refuses with
    // the same locator rather than falling out of the loop with a root
    // it cannot justify. **Guarded**, by
    // `a_cycling_graft_map_refuses_in_the_b_lane` below: unlike
    // `chase_edge_to_table`, this walk reads `fwd_edges` — built from
    // `BooleanNaming::graft_edges`, which the caller supplies —
    // between provenance reads, so one honest `split_edge` record and
    // one synthetic graft row close the loop from outside `topo`.
    let chase_b = |e_b0: EdgeKey| -> Result<EdgeKey, NamingError> {
        let mut e_b = e_b0;
        for _ in 0..=fwd_edges.len() {
            if b.table.name_of(&ent(0, EntityKey::Edge(e_b))).is_some() {
                return Ok(e_b);
            }
            let Some(res) = fwd_edges.get(&e_b) else {
                return Ok(e_b); // dead, ungrafted intermediate
            };
            match body.edge_provenance_of(*res) {
                Some(Provenance::SplitEdge { edge }) => e_b = *edge,
                _ => return Ok(e_b),
            }
        }
        Err(topo::SplitLineageCycle { edge: e_b0 }.into())
    };
    let mut groups: BTreeMap<OpSide<EdgeKey>, Vec<EdgeKey>> = BTreeMap::new();
    for (e, _) in body.edges() {
        if seam_set.contains(&e) {
            continue;
        }
        // The split-lineage chase is decided by where the side's keys
        // live, not by which side it is: an operand whose clone IS the
        // arena has its lineage in the arena's own provenance, so its
        // edge chases there — A in an A-clone or grafted result, B in a
        // B-clone one. Only a grafted side needs `chase_b`'s bridge.
        let (side, space) = operand_key(naming, inv_edges, e)?;
        let (op, k) = side.of(a, b);
        let root_key = match space {
            KeySpace::Arena => chase_edge_to_table(body, op.table, k)?,
            KeySpace::Graft => chase_b(k)?,
        };
        let root = side.with(root_key);
        // A root that resolves in no operand table is a join-minted
        // crossing chord that survived OUTSIDE the zip's list (channel
        // cuts: chords on the operand's own faces) — a DERIVED seam
        // edge, named by its adjacent faces' descent like any seam.
        let (op, k) = root.of(a, b);
        let resolves = op.table.name_of(&ent(0, EntityKey::Edge(k))).is_some();
        if resolves {
            groups.entry(root).or_default().push(e);
        } else {
            // Where a both-merged chord reads through to, checked
            // geometrically there (`chord_kind`).
            match chord_kind(e, Some(root.with(())))? {
                ChordKind::Cross(fa, fb) => {
                    add_seam(fa, fb, e);
                }
                ChordKind::SameA(k) => {
                    groups.entry(OpSide::A(k)).or_default().push(e);
                }
                ChordKind::SameB(k) => {
                    groups.entry(OpSide::B(k)).or_default().push(e);
                }
            }
        }
    }
    for ((fa, fb), (from_tie, edges)) in seam_groups {
        let base = name1(EntityKind::Edge, node, RoleSeg::Seam { a: fa, b: fb });
        if edges.len() == 1 {
            put(t, tie, from_tie, base, ent(0, EntityKey::Edge(edges[0])))?;
            continue;
        }
        // Collinear chain: order along the pair's intersection line,
        // oriented n_a × n_b with the A-side face first — the side read
        // off the face's descent (structure, never list order). A later
        // step that knows this seam only by its name reads the same
        // orientation back through `seam_line_dir`.
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
            OpSide::A(_) => (f0, f1),
            OpSide::B(_) => (f1, f0),
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
        let (op, root_key) = root.of(a, b);
        let inner = upstream_name(op.table, op.node, ent(0, EntityKey::Edge(root_key)))?;
        let op_body = op.body;
        let from_tie = inner.tied;
        let seg = root.wrap(inner.name.clone());
        let base = name1(EntityKind::Edge, node, seg);
        if edges.len() == 1 {
            put(t, tie, from_tie, base, ent(0, EntityKey::Edge(edges[0])))?;
            continue;
        }
        // Sub-edge chain: order along the parent's line. A parent that
        // is itself a SEAM edge is ordered the way a seam chain is — along
        // its pair's `n_a × n_b`, in the pair's minted order — because
        // the same pieces are a seam chain in an order that cuts the line
        // before it is minted; one line, one orientation, whichever step
        // cut it. Any other parent is ordered along its own oriented
        // carrier (operand geometry).
        let dir = match seam_pair::seam_line_pair(&inner.name) {
            Some(pair) => seam_line_dir(op.body, op.table, op.node, root_key, pair)?,
            None => edge_dir(op_body, root_key)?,
        };
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
    // An operand vertex's upstream name, if that operand's table names
    // it. A key the table does not name (a vertex the reduction minted)
    // yields nothing; a table that names a key and then fails to resolve
    // it is corrupt, and says so.
    let named_in = |side: OpSide<VertexKey>| -> Result<Option<Upstream>, NamingError> {
        let (op, k) = side.of(a, b);
        if op.table.name_of(&ent(0, EntityKey::Vertex(k))).is_none() {
            return Ok(None);
        }
        upstream_name(op.table, op.node, ent(0, EntityKey::Vertex(k))).map(Some)
    };
    // The operand identity of one result-arena vertex key, if its
    // operand's table names it.
    let operand_identity = |k: VertexKey| -> Result<Option<(StableName, bool)>, NamingError> {
        let (side, _) = operand_key(naming, inv_vertices, k)?;
        Ok(named_in(side)?.map(|u| (name1(EntityKind::Vertex, node, side.wrap(u.name)), u.tied)))
    };
    // Candidate seam-vertex names, grouped for multiplicity: the key
    // is the (A, B) parent pair, the value (descends-from-a-tie,
    // vertices).
    let mut groups: BTreeMap<(NameRef, NameRef), (bool, Vec<VertexKey>)> = BTreeMap::new();
    for (v, _) in body.vertices() {
        // Operand pass-downs: the kept key itself, then its dead
        // fusion partners (deterministic order: KEPT-KEY identity
        // wins when both operands fused here — `operand_identity`
        // checks the graft destination first, and the kept key is
        // A's exactly because `zip_seam` keeps the outer cycle's
        // vertex).
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
        // The parentage a seam vertex is named from is read off the
        // incident EDGE names and put straight back into this vertex's
        // own name, so it travels as the operand tables' handles: no
        // copy is made and the order cache survives the trip.
        let mut a_edges: Vec<NameRef> = Vec::new();
        let mut b_edges: Vec<NameRef> = Vec::new();
        let mut a_faces: Vec<NameRef> = Vec::new();
        let mut b_faces: Vec<NameRef> = Vec::new();
        // The distinct seam LINES through this vertex: a set, because
        // several incident seam edges lie on one line. The junction's
        // name is these lines, and `names::canonical` orders them.
        let mut seam_lines: BTreeSet<(NameRef, NameRef)> = BTreeSet::new();
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
                Some(RoleSeg::FromA(x)) => a_edges.push(x.clone()),
                Some(RoleSeg::FromB(x)) => b_edges.push(x.clone()),
                // Zip-listed AND derived seams both qualify (M4 PR 5:
                // declared merges reroute channel-cut chords into the
                // derived-seam lane, so a seam vertex may lean on a
                // Seam-named edge outside `naming.seam_edges`) — the
                // NAME is the evidence either way.
                Some(RoleSeg::Seam { a: fa, b: fb }) => {
                    a_faces.push(fa.clone());
                    b_faces.push(fb.clone());
                    seam_lines.insert((fa.clone(), fb.clone()));
                }
                _ => return Err(bug("seam vertex incident to an unexpected edge role")),
            }
        }
        for list in [&mut a_edges, &mut b_edges, &mut a_faces, &mut b_faces] {
            list.sort_unstable();
            list.dedup();
        }
        // Contact-record partner: the reduction's own declared
        // contacts (mint-time, PRE-remap — `reduction_contacts`). Each
        // row speaks each operand's own CLONE keys — its A column A-clone
        // keys, its B column B-clone keys — and `operand_key` puts this
        // vertex's result key into the same space, whichever clone the
        // arena is. A vertex with no seam structure of its own finds its
        // coincident operand partner here — recorded knowledge, never
        // re-measured. Two shapes have one: a residual crossing whose
        // seam was consumed (shared-plane overlaps), and a vertex minted
        // on one operand's edge where the other's vertex touches it with
        // nothing zipped AT that vertex — in any result kind, a seamed
        // one included, since a zip elsewhere in the body does not reach
        // it. The touch comes in both orientations, so the partner is
        // read on whichever side the vertex's key belongs to.
        let rc = &naming.reduction_contacts;
        let (partner_a, partner_b): (Option<Upstream>, Option<Upstream>) =
            match operand_key(naming, inv_vertices, v)?.0 {
                OpSide::A(ka) => match rc.vv.iter().find(|r| r.a == ka) {
                    Some(r) => (None, named_in(OpSide::B(r.b))?),
                    None => (None, None),
                },
                OpSide::B(kb) => match rc.vv.iter().find(|r| r.b == kb) {
                    Some(r) => (named_in(OpSide::A(r.a))?, None),
                    None => (None, None),
                },
            };
        from_tie |= partner_b.as_ref().is_some_and(|u| u.tied);
        from_tie |= partner_a.as_ref().is_some_and(|u| u.tied);
        let partner_b_inner: Option<NameRef> = partner_b.map(|u| u.name);
        let partner_a_inner: Option<NameRef> = partner_a.map(|u| u.name);
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
            // where k ≥ 2 seam LINES meet. Its name is the path of the
            // lines' Seam segments, in the canonical form's order —
            // deterministic, and unique per line set (straight lines
            // meet once).
            ([], [], _, _) if seam_lines.len() >= 2 => {
                let name = canonical::minted(StableName {
                    kind: EntityKind::Vertex,
                    node,
                    path: seam_lines
                        .iter()
                        .map(|(fa, fb)| RoleSeg::Seam {
                            a: fa.clone(),
                            b: fb.clone(),
                        })
                        .collect(),
                });
                put(t, tie, from_tie, name, ent(0, EntityKey::Vertex(v)))?;
                continue;
            }
            // WITNESSED, and the arm is written to the witness. An
            // ordinary declared union reaches exactly this shape: one
            // operand-descended edge on the A side, none on the B side,
            // and — everything above having already run — no single B
            // face and no contact-record partner to supply the other
            // parent. The vertex is half-decided, the body is sound and
            // the recipe is legal, so what is missing is a rule.
            //
            // **Its mirror (`([], [_], None, _)`) is not here, and the
            // reason is the finish's asymmetry, scoped to ZIPPED
            // vertices.** The witness is a vertex whose B structure a
            // shared plane consumed: Eq. 15.3 lumps every B on-sector to
            // the discarded side (`topo::boolean::tables::eq15_3_lump`),
            // so on a shared plane A's copy is what survives, and the zip
            // keeps A's vertex where the two meet. The mirror needs B's
            // lone edge surviving where A's structure was consumed, which
            // that lumping does not produce. A vertex nothing zipped
            // (a touch) is not covered by this argument; its parent comes
            // from the contact-record partner on either side. Two kinds
            // of row hold that: `swapping_the_operands_swaps_the_sides_of_every_name`
            // (in `emit_boolean_vertex_keys`) holds the two sides
            // SYMMETRIC — the same geometry named alike with A and B
            // exchanged — which a consistent A/B relabel would pass; the
            // absolute rows beside it (the nested corners, the split
            // reflex edge, the assembly touch) pin WHICH side each name
            // belongs to. A shape nobody has reached is not a shape known
            // to be legal, so the mirror stays in the residue below.
            ([_], [], _, _) => return Err(NamingError::SeamVertexParentage { vertex: v }),
            // The unenumerated residue, which stays an emission bug. A
            // catch-all is the preimage of every case nobody has named
            // — `a_edges.len() >= 2 && b_edges.len() >= 2` among them —
            // and a shape nobody has reached is not a shape known to be
            // legal.
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
                a: pa.clone(),
                b: pb.clone(),
            },
        );
        if verts.len() == 1 {
            put(t, tie, from_tie, base, ent(0, EntityKey::Vertex(verts[0])))?;
            continue;
        }
        // Same pair crossing more than once: order along the edge
        // parent's own carrier (prefer the A side).
        let carrier = match resolve_edge_carrier(&pa, a)? {
            Some(dir) => Some(dir),
            None => resolve_edge_carrier(&pb, b)?,
        };
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
///
/// A parent that is not a uniquely named edge of that table has no
/// carrier (`None`); an edge the table names whose geometry does not
/// resolve is a corrupt operand body, and refuses rather than reading as
/// "no carrier" and demoting the group to a tie.
fn resolve_edge_carrier<T: Decide>(
    parent: &StableName,
    op: &OperandCtx<'_, T>,
) -> Result<Option<Vec3<T>>, NamingError> {
    if parent.kind != EntityKind::Edge {
        return Ok(None);
    }
    match op.table.lookup(parent) {
        Some(Entry::Unique(e)) => match (e.key, seam_pair::seam_line_pair(parent)) {
            // An edge on a seam line is ranked along that line, the one
            // orientation every ranker along a seam line uses.
            (EntityKey::Edge(k), Some(pair)) => {
                seam_line_dir(op.body, op.table, op.node, k, pair).map(Some)
            }
            (EntityKey::Edge(k), None) => edge_dir(op.body, k).map(Some),
            _ => Ok(None),
        },
        _ => Ok(None),
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

/// Whether result edge `chord` lies on operand edge `rim` of
/// `op_body`: both of its ends on the rim's line and between the rim's
/// ends. Three margins per end, each a length and each decided through
/// [`CHORD_ON_RIM`]: the distance off the line (must be `Zero`), and
/// the signed distances past each rim end along it (must not be
/// `Negative`). An in-band margin escalates typed.
fn chord_on_rim<T: Decide>(
    body: &Body<T>,
    chord: EdgeKey,
    op_body: &Body<T>,
    rim: EdgeKey,
    bnd: geom_core::Band,
) -> Result<bool, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let point = |b: &Body<T>, v: VertexKey| -> Result<Point3<T>, NamingError> {
        b.get_vertex(v)
            .and_then(|vd| b.get_point(vd.point))
            .copied()
            .ok_or_else(|| bug("chord_on_rim: vertex without point"))
    };
    let (r0, r1) = edge_ends(op_body, rim)?;
    let (q0, q1) = (point(op_body, r0)?, point(op_body, r1)?);
    let d = q1 - q0;
    let len = d.norm();
    let sign = |m: Margin<T>| {
        decide(CHORD_ON_RIM, m, bnd).map_err(|source| NamingError::Escalated {
            predicate: CHORD_ON_RIM,
            source,
        })
    };
    let (c0, c1) = edge_ends(body, chord)?;
    for v in [c0, c1] {
        let p = point(body, v)?;
        let off = sign(Margin::over_lever((p - q0).cross(d).norm(), len))?;
        let past0 = sign(Margin::over_lever((p - q0).dot(d), len))?;
        let past1 = sign(Margin::over_lever((q1 - p).dot(d), len))?;
        if off != Sign::Zero || past0 == Sign::Negative || past1 == Sign::Negative {
            return Ok(false);
        }
    }
    Ok(true)
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

/// The direction a chain along a seam line is ranked in: the pair's
/// `n_a × n_b`, with `a` and `b` the pair's two sides as its name
/// records them (`super::seam_pair`). They are matched by NAME to the
/// two faces of `edge` in `body`, whose names `table` holds, and the
/// outward normals are read from those faces. `node` is the node whose
/// body this is, carried by the refusal.
///
/// The rankers that know a seam only by its NAME read their direction
/// here: the descent ranker and the vertex carrier, on an operand body.
/// The seam-chain ranker knows its sides structurally and computes the
/// same `n_a × n_b` from them. So the pieces of one line are ranked one
/// way, whichever step cut it.
fn seam_line_dir<T: Decide>(
    body: &Body<T>,
    table: &NameTable,
    node: RecipeNodeId,
    edge: EdgeKey,
    (a, b): (&StableName, &StableName),
) -> Result<Vec3<T>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let e = body
        .get_edge(edge)
        .ok_or_else(|| bug("seam line edge not live in its body"))?;
    let mut faces = [None, None];
    for (slot, he) in faces
        .iter_mut()
        .zip([Some(e.he_plus), body.mate(e.he_plus)])
    {
        let he = he.ok_or_else(|| bug("seam line edge without a mate"))?;
        let face = body
            .get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .map(|l| l.face)
            .ok_or_else(|| bug("seam line edge half-edge off any face"))?;
        let name = table
            .name_of(&ent(0, EntityKey::Face(face)))
            .ok_or_else(|| bug("seam line edge face unnamed"))?;
        *slot = Some((face, name));
    }
    let [Some((f0, n0)), Some((f1, n1))] = faces else {
        return Err(bug("seam line edge without two faces"));
    };
    let (fa, fb) = match seam_pair::a_side_is_first(n0, n1, a, b) {
        Some(true) => (f0, f1),
        Some(false) => (f1, f0),
        None => return Err(NamingError::SeamLineSides { node, edge }),
    };
    let (_, na) = face_plane(body, fa)?;
    let (_, nb) = face_plane(body, fb)?;
    Ok(na.cross(nb))
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
    let divided: BTreeSet<FaceKey> = frag_rows
        .values()
        .map(|&p| chase(frag_rows, p))
        .collect::<Result<_, NamingError>>()?;
    // (root, side-slot) → members.
    type Members = Vec<(u32, SplitHalf, FaceKey)>;
    let mut groups: BTreeMap<(FaceKey, u32), Members> = BTreeMap::new();
    for s in sides {
        for (f, _) in s.body.faces() {
            if section_keys.contains(&f) {
                continue;
            }
            let root = chase(frag_rows, f)?;
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
            parent: parent.name,
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
    use geom_core::Tol;
    use profile::RawLoop;

    /// A body holding one straight edge from `p` to `q`, and that edge.
    fn segment(p: [f64; 3], q: [f64; 3]) -> (Body<f64>, EdgeKey) {
        let mut body = Body::<f64>::new();
        let born = body
            .mvfs(Point3::new(p[0], p[1], p[2]))
            .expect("mvfs births a lone vertex");
        let edge = body
            .mev_line(
                topo::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                Point3::new(q[0], q[1], q[2]),
                Tol::witness(),
            )
            .expect("mev on an empty loop grows it by one edge")
            .edge;
        (body, edge)
    }

    /// `chord_on_rim` for a chord `p`–`q` against the rim (0,0,0)–(1,0,0).
    fn on_unit_rim(p: [f64; 3], q: [f64; 3]) -> bool {
        let (rim_body, rim) = segment([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let (chord_body, chord) = segment(p, q);
        chord_on_rim(
            &chord_body,
            chord,
            &rim_body,
            rim,
            band(Tol::witness()).unwrap(),
        )
        .unwrap()
    }

    #[test]
    fn a_chord_within_its_rim_is_on_it_ends_included() {
        assert!(on_unit_rim([0.2, 0.0, 0.0], [0.6, 0.0, 0.0]));
        assert!(on_unit_rim([0.0, 0.0, 0.0], [0.5, 0.0, 0.0]));
        assert!(on_unit_rim([0.5, 0.0, 0.0], [1.0, 0.0, 0.0]));
        assert!(on_unit_rim([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]));
    }

    #[test]
    fn a_chord_off_its_rims_line_is_not_on_it() {
        // Within the rim's span along it, so only the off-line margin
        // can refuse.
        assert!(!on_unit_rim([0.2, 0.1, 0.0], [0.6, 0.1, 0.0]));
        assert!(!on_unit_rim([0.2, 0.0, 0.0], [0.6, 0.0, 0.1]));
    }

    #[test]
    fn a_chord_past_either_end_of_its_rim_is_not_on_it() {
        // On the rim's line, so only the past-an-end margins can refuse.
        assert!(!on_unit_rim([-0.2, 0.0, 0.0], [0.5, 0.0, 0.0]));
        assert!(!on_unit_rim([0.5, 0.0, 0.0], [1.3, 0.0, 0.0]));
    }

    /// **The result-body stand-in every row here descends from**: a
    /// unit-cube extrusion, whose table names a top, a bottom and four
    /// laterals. Written once — the rows below differ in the synthetic
    /// `BooleanNaming` they hand the emitter, never in the body.
    fn unit_cube() -> sweep::Extruded<f64> {
        let plane = profile::SketchPlane::from_frame(geom_core::OrthoFrame::axes_xy(
            geom_core::Point3::new(0.0, 0.0, 0.0),
        ));
        let square = profile::ProfileLoop::polygon(
            [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                .into_iter()
                .map(|(x, y)| geom_core::Point2::new(x, y)),
        );
        let profile = profile::Profile::new(plane, vec![square])
            .validate(geom_core::Tol::witness())
            .unwrap();
        sweep::extrude(
            &profile,
            sweep::Extrusion::Distance(1.0_f64),
            Tol::witness(),
        )
        .unwrap()
    }

    #[test]
    fn merged_lane_names_kept_face_with_sorted_deduped_constituents() {
        // A unit-cube extrusion: the "result body" stand-in.
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let a_table = name_extrude(ext_node, &built).unwrap();

        // Synthetic merge: the end cap absorbed one lateral — listed
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
        let t = name_boolean(bool_node, &built.body, &naming, &a, &b, Tol::witness()).unwrap();

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

    /// **A cycling fragment map refuses, and the refusal is what
    /// stands between the kernel and a face named after a stranger.**
    ///
    /// `BooleanNaming` is a public struct with public fields and the
    /// emitter takes it as data, so the corrupt mint `chase` exists to
    /// catch is writable at this door — which is why this raise is
    /// guarded rather than argued about.
    ///
    /// The row is worth more than its pass. With `chase`'s refusal
    /// removed, `name_boolean` returns `Ok` with a TOTAL table of 27
    /// rows in which the two caps carry each other's operand names —
    /// measured, not argued: `built.top` comes out
    /// `FromA(Cap(Start))` and `built.bottom` comes out
    /// `FromA(Cap(End))`, each the other's. Nothing is missing and
    /// nothing refuses; the document is simply wrong about which face
    /// is which.
    ///
    /// Written by the review lane of 2026-09-13; adopted with its
    /// argument.
    #[test]
    fn a_cycling_fragment_map_refuses() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let a_table = name_extrude(ext_node, &built).unwrap();
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            // The corrupt mint, spelled: top is a fragment of bottom
            // and bottom a fragment of top.
            face_fragments_a: vec![(built.top, built.bottom), (built.bottom, built.top)],
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
        let err = name_boolean(
            RecipeNodeId(9),
            &built.body,
            &naming,
            &a,
            &b,
            Tol::witness(),
        )
        .expect_err("a cycling fragment map must refuse");
        assert!(
            matches!(
                err,
                NamingError::FragmentLineage { face }
                    if face == built.top || face == built.bottom
            ),
            "the refusal names one of the two faces in the cycle: {err:?}"
        );
        assert!(
            err.to_string().contains("fragment lineage of face"),
            "and says which record family it caught: {err}"
        );
    }

    /// **The B-lane edge chase refuses on a cycle too, and this is the
    /// row `chase_edge_to_table` cannot have.**
    ///
    /// `chase_b` advances in two steps — a hop through `fwd_edges`,
    /// built from the caller's `BooleanNaming::graft_edges`, and then
    /// a read of `Body::edge_provenance`. The second is `topo`'s and
    /// writes only strictly-older parents; the FIRST is the caller's,
    /// and one row through it closes a loop that `split_edge`'s
    /// records alone cannot. So the refusal `chase_edge_to_table`
    /// shares is exercised here, on the lane where a real graft can
    /// alias the same way.
    ///
    /// The chain is honest: `split_edge` twice off one lateral gives
    /// `e2 → e1 → e0` as genuine birth records, and only the one graft
    /// row is synthetic.
    #[test]
    fn a_cycling_graft_map_refuses_in_the_b_lane() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let b_table = name_extrude(ext_node, &built).unwrap();
        let mut body = built.body.clone();
        let e0 = body.edges().next().map(|(k, _)| k).unwrap();
        let e1 = body
            .split_edge(e0, 0.25, Tol::witness())
            .expect("an edge splits at an interior parameter")
            .new_edge;
        let e2 = body
            .split_edge(e1, 0.5, Tol::witness())
            .expect("the second child splits again")
            .new_edge;
        assert!(
            b_table.name_of(&ent(0, EntityKey::Edge(e0))).is_some()
                && b_table.name_of(&ent(0, EntityKey::Edge(e1))).is_none(),
            "the parent is named and the children are not, or the walk stops early"
        );
        // The grafted layout — the only one `chase_b` serves. ONE
        // synthetic graft row is enough: result edge e2 reads as B's
        // e1, e1 forwards to e2, and e2's birth record points back at
        // e1. A is the same named cube, so every other key resolves on
        // the A side and the walk reaches e2.
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Grafted,
            graft_edges: vec![(e1, e2)],
            ..topo::BooleanNaming::default()
        };
        let a = OperandCtx {
            node: RecipeNodeId(2),
            table: &b_table,
            body: &body,
        };
        let b = OperandCtx {
            node: ext_node,
            table: &b_table,
            body: &body,
        };
        let err = name_boolean(RecipeNodeId(9), &body, &naming, &a, &b, Tol::witness())
            .expect_err("a graft row that forwards into its own parent must refuse");
        assert!(
            matches!(err, NamingError::SplitLineage(c) if c.edge == e1),
            "the refusal names the edge the walk was asked about: {err:?}"
        );
    }

    /// Review R8 (resolved M4 PR 5): two merge groups with the SAME
    /// constituent set — kept faces that are fragments of one operand
    /// face, each absorbing a fragment of one partner — refuse
    /// LOUDLY (typed `NamingError`), never a silent alias.
    #[test]
    fn merged_same_constituent_groups_collide_loudly() {
        let built = unit_cube();
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
        let err = name_boolean(
            RecipeNodeId(9),
            &built.body,
            &naming,
            &a,
            &b,
            Tol::witness(),
        )
        .expect_err("same-constituent merge groups must refuse loudly");
        let _ = err; // typed NamingError, never a silent alias
    }

    /// The flat mint: an operand face that is itself a merged face
    /// contributes its CONSTITUENTS to a merge over it, each wrapped
    /// by the descent side, and never its `Merged` name.
    #[test]
    fn a_merge_over_a_merged_face_lists_its_constituents_flat() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let ext_table = name_extrude(ext_node, &built).unwrap();
        // Three laterals: one the merge absorbs, two standing as the
        // constituents the operand's top cap already merged.
        let mut laterals: Vec<(StableName, FaceKey)> = ext_table
            .iter()
            .filter_map(|(n, e)| match (n.path.first(), e) {
                (Some(RoleSeg::Lateral(_)), Entry::Unique(r)) => match r.key {
                    EntityKey::Face(f) => Some((n.clone(), f)),
                    _ => None,
                },
                _ => None,
            })
            .collect();
        laterals.sort();
        let (absorbed_name, absorbed) = laterals[0].clone();
        let mut inner_set = vec![laterals[1].0.clone(), laterals[2].0.clone()];
        inner_set.sort();
        // The operand's table, its top cap named as a merged face.
        let top = ent(0, EntityKey::Face(built.top));
        let top_name = ext_table.name_of(&top).unwrap().clone();
        let mut a_table = NameTable::new();
        for (n, e) in ext_table.iter() {
            let name = if *n == top_name {
                name1(
                    EntityKind::Face,
                    ext_node,
                    RoleSeg::Merged(inner_set.clone()),
                )
            } else {
                n.clone()
            };
            match e {
                Entry::Unique(r) => a_table.insert(name, *r).unwrap(),
                Entry::Tied(es) => a_table.insert_tied(name, es.clone()).unwrap(),
            }
        }
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            merge_groups: vec![(built.top, vec![absorbed])],
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
        let t = name_boolean(bool_node, &built.body, &naming, &a, &b, Tol::witness()).unwrap();
        let wrap = |inner: &StableName| {
            name1(
                EntityKind::Face,
                bool_node,
                RoleSeg::FromA(inner.clone().into()),
            )
        };
        let mut want = vec![
            wrap(&inner_set[0]),
            wrap(&inner_set[1]),
            wrap(&absorbed_name),
        ];
        want.sort();
        let row = name1(EntityKind::Face, bool_node, RoleSeg::Merged(want));
        match t.lookup(&row) {
            Some(Entry::Unique(r)) => assert_eq!(r.key, EntityKey::Face(built.top)),
            other => panic!("the flat merged row is not published: {other:?}"),
        }
        // And no row carries the operand's merged name inside a merge.
        let nested = t.iter().any(|(n, _)| {
            n.path.iter().any(|seg| match seg {
                RoleSeg::Merged(cs) => cs.iter().any(|c| match c.path.first() {
                    Some(RoleSeg::FromA(inner)) => {
                        matches!(inner.path.first(), Some(RoleSeg::Merged(_)))
                    }
                    _ => false,
                }),
                _ => false,
            })
        });
        assert!(!nested, "a merge over a merged face nested the merged name");
    }

    /// The mint's own guarantee: an operand whose merged face lists a
    /// merged face — a table no emitter of this crate produces — is
    /// refused at the merge-group loop, not published nested. The
    /// pair boolean has no `collapse` after it, so this is the one
    /// door that holds N3 for it.
    #[test]
    fn a_merge_over_a_nested_merged_face_refuses_at_the_mint() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let ext_table = name_extrude(ext_node, &built).unwrap();
        let mut laterals: Vec<(StableName, FaceKey)> = ext_table
            .iter()
            .filter_map(|(n, e)| match (n.path.first(), e) {
                (Some(RoleSeg::Lateral(_)), Entry::Unique(r)) => match r.key {
                    EntityKey::Face(f) => Some((n.clone(), f)),
                    _ => None,
                },
                _ => None,
            })
            .collect();
        laterals.sort();
        let absorbed = laterals[0].1;
        // The top cap named as a merged face whose ONE constituent is
        // itself a merged face — the nested shape.
        let inner = name1(
            EntityKind::Face,
            ext_node,
            RoleSeg::Merged(vec![laterals[1].0.clone(), laterals[2].0.clone()]),
        );
        let nested = name1(EntityKind::Face, ext_node, RoleSeg::Merged(vec![inner]));
        let top = ent(0, EntityKey::Face(built.top));
        let top_name = ext_table.name_of(&top).unwrap().clone();
        let mut a_table = NameTable::new();
        for (n, e) in ext_table.iter() {
            let name = if *n == top_name {
                nested.clone()
            } else {
                n.clone()
            };
            match e {
                Entry::Unique(r) => a_table.insert(name, *r).unwrap(),
                Entry::Tied(es) => a_table.insert_tied(name, es.clone()).unwrap(),
            }
        }
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            merge_groups: vec![(built.top, vec![absorbed])],
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
        let err = name_boolean(
            RecipeNodeId(9),
            &built.body,
            &naming,
            &a,
            &b,
            Tol::witness(),
        )
        .expect_err("a nested merged face must refuse at the mint");
        assert!(
            matches!(err, NamingError::Emission { what } if what == NESTED_MERGED),
            "{err:?}"
        );
    }
}

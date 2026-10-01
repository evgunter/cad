//! Split/boolean name emission (spec D2/D3): descent-driven — every
//! result entity is chased to its operand parent through the kernels'
//! mint-time rows (`SplitNaming`, `BooleanNaming`, D5 `SplitEdge`
//! provenance), then named as pass-through, `FromA`/`FromB`,
//! fragment (with N2 qualifiers), seam, section, or merged. Nothing
//! is matched; unresolvable descent is a typed error.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::k_stats::decide;
use geom_core::{Decide, Margin, Point3, Sign, Vec3};
use topo::splitting::{PlaneSide, SplitNaming};
use topo::{Body, EdgeKey, FaceKey, Provenance, VertexKey};

use super::borders::Obstacles;
use super::canonical;
use super::defer::{TieRows, Upstream, mint_candidates, put, upstream_name};
use super::discriminate::{CHORD_ON_RIM, Extent, band, order_along};
use super::emit::{
    Incidence, NamingError, Rim, RimShare, edge_ends, ent, face_half_edges, name1, rim_between,
    rims_between, vertex_point,
};
use super::groups::{Emitted, GroupRecord, Parent};
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
/// decreasing in age in the arena that wrote it. A graft does not keep
/// that order — a dead-on-arrival key is minted after the grafted
/// edges whose records name it — but it forwards injectively (each
/// source key to its own result key), and an injective image of an
/// acyclic chain is acyclic. Nothing outside `topo` can close it. `chase_b` below walks the same records and reads the
/// caller's graft rows only to decide where to stop, so the argument
/// covers it too.
fn chase_edge_to_table<T: Decide>(
    body: &Body<T>,
    table: &NameTable,
    e: EdgeKey,
) -> Result<EdgeKey, NamingError> {
    Ok(body.split_root(e, |k| table.name_of(&ent(0, EntityKey::Edge(k))).is_some())?)
}

/// [`chase_edge_to_table`] over a split's two halves. The halves are
/// carved from one scratch arena, so a key resolves in whichever half
/// kept the entity and only there (`SplitNaming`), and so does its
/// `SplitEdge` record: each hop reads the record from the half holding
/// that key. An operand edge the plane crosses twice has its middle
/// piece on one side and the outer two on the other, so an outer
/// piece's lineage can run through a key its own half does not hold.
///
/// # Errors
///
/// [`NamingError::SplitLineage`] on a cycling lineage; the halves'
/// records are restrictions of one acyclic record set, so their union
/// is acyclic by the same writer-access argument.
fn chase_split_edge_to_table<T: Decide>(
    sides: &[Side<'_, T>],
    table: &NameTable,
    e: EdgeKey,
) -> Result<EdgeKey, NamingError> {
    let bound: usize = sides.iter().map(|s| s.body.edges().count()).sum();
    let mut root = e;
    for _ in 0..=bound {
        if table.name_of(&ent(0, EntityKey::Edge(root))).is_some() {
            return Ok(root);
        }
        let mut held = sides.iter().filter_map(|s| s.body.edge_provenance_of(root));
        let record = held.next();
        debug_assert!(
            held.all(|other| Some(other) == record),
            "split halves disagree on edge {root:?}'s birth record"
        );
        match record {
            Some(Provenance::SplitEdge { edge }) => root = *edge,
            _ => return Ok(root),
        }
    }
    Err(topo::SplitLineageCycle { edge: e }.into())
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
    tol: Tol,
) -> Result<Emitted, NamingError> {
    let b = band(tol)?;
    let mut t = NameTable::new();
    let mut rec = GroupRecord::new();
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
        &mut rec,
        &sides,
        &frag_rows,
        &section_keys,
        target_node,
        target_table,
        target_body,
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
        target_body,
        b,
    )?;
    tie.flush(&mut t)?;

    for s in &sides {
        super::emit::check_total(&t, s.body, s.ix)?;
    }
    Ok(Emitted::new(t, rec))
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
/// `SplitFragment` (crossing-cut operand edges), `CrossingVertex`,
/// `OnToolVertex`. The edges are grouped by their parent first, the
/// vertices named from those parents, and then several pieces of one
/// parent qualified by their ends ([`name_edge_pieces`]).
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
    target_body: &Body<T>,
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let copy_to_original: BTreeMap<VertexKey, VertexKey> =
        naming.vertex_pairs.iter().copied().collect();
    // (side, base) → (from a tie, the side's edges under it).
    let mut edge_groups: BTreeMap<(usize, StableName), (bool, Vec<EdgeKey>)> = BTreeMap::new();
    // A divided parent is collected across BOTH sides first: a
    // kept-key first child looks like an intact operand edge in ITS
    // side alone.
    let mut divided_edges: BTreeSet<EdgeKey> = BTreeSet::new();
    for sb in sides {
        for (e, _) in sb.body.edges() {
            // FRESH children only: an edge the target table already
            // names is the target's own entity, not a product of THIS
            // split.
            if target_table.name_of(&ent(0, EntityKey::Edge(e))).is_none()
                && matches!(
                    sb.body.edge_provenance_of(e),
                    Some(Provenance::SplitEdge { .. })
                )
            {
                divided_edges.insert(chase_split_edge_to_table(sides, target_table, e)?);
            }
        }
    }
    for (slot, s) in sides.iter().enumerate() {
        let body = s.body;
        let chord_faces = chord_faces(body, &naming.sections, section_keys)?;
        // Chord edges named by the operand face their section boundary
        // runs across. A section line that re-enters ONE operand face
        // (an inner loop, or any non-convex face) cuts several chords
        // `SectionEdge{side, face}` spells alike: pieces of one parent,
        // told apart by their ends like any other (N2).
        for (&e, &other) in &chord_faces {
            let root = chase(frag_rows, other)?;
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
            let group = edge_groups.entry((slot, name)).or_default();
            group.0 |= parent.tied;
            group.1.push(e);
        }
        // Remaining edges: pass-through or crossing-cut fragments.
        for (e, _) in body.edges() {
            if chord_faces.contains_key(&e) {
                continue;
            }
            let root = chase_split_edge_to_table(sides, target_table, e)?;
            if target_table.name_of(&ent(0, EntityKey::Edge(e))).is_some()
                && !divided_edges.contains(&root)
            {
                // Intact operand edge: pass-through.
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(e)))?;
                tie.carry(up, ent(s.ix, EntityKey::Edge(e)))?;
                continue;
            }
            if target_table
                .name_of(&ent(0, EntityKey::Edge(root)))
                .is_none()
            {
                return Err(bug("edge descent reached no operand edge"));
            }
            let parent = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(root)))?;
            let name = name1(
                EntityKind::Edge,
                node,
                RoleSeg::SplitFragment {
                    side: s.half,
                    parent: parent.name,
                },
            );
            let group = edge_groups.entry((slot, name)).or_default();
            group.0 |= parent.tied;
            group.1.push(e);
        }
        // Vertices. Pair membership FIRST: a vertex the tool plane
        // passed through exists as a coincident copy in BOTH halves
        // (null-pair rows), so even an operand-named original must
        // take a side-tagged role, never the bare pass-through (the
        // bare name would alias across the two halves).
        let pair_originals: BTreeSet<VertexKey> =
            naming.vertex_pairs.iter().map(|&(_, o)| o).collect();
        // Crossing vertices, by the operand edge they cross: (from a
        // tie, the vertices).
        let mut crossings: BTreeMap<EdgeKey, (Upstream, Vec<VertexKey>)> = BTreeMap::new();
        for (v, _) in body.vertices() {
            let is_pair_member = copy_to_original.contains_key(&v) || pair_originals.contains(&v);
            if !is_pair_member
                && target_table
                    .name_of(&ent(0, EntityKey::Vertex(v)))
                    .is_some()
            {
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Vertex(v)))?;
                tie.carry(up, ent(s.ix, EntityKey::Vertex(v)))?;
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
                        Some(chase_split_edge_to_table(sides, target_table, *edge))
                    }
                    _ => None,
                })
                .transpose()?;
            if let Some(parent_edge) = parent_edge {
                // Crossing vertex: minted where the plane crossed an
                // operand edge's interior.
                match crossings.get_mut(&parent_edge) {
                    Some((_, vs)) => vs.push(v),
                    None => {
                        let parent = upstream_name(
                            target_table,
                            target_node,
                            ent(0, EntityKey::Edge(parent_edge)),
                        )?;
                        crossings.insert(parent_edge, (parent, vec![v]));
                    }
                }
            } else if target_table
                .name_of(&ent(0, EntityKey::Vertex(src)))
                .is_some()
            {
                // The plane passed THROUGH an operand vertex (review
                // R2): side-tagged pass-through — operand identity
                // from the pair row, side from body membership (a
                // recorded verdict).
                let of = upstream_name(target_table, target_node, ent(0, EntityKey::Vertex(src)))?;
                put(
                    t,
                    tie,
                    of.tied,
                    name1(
                        EntityKind::Vertex,
                        node,
                        RoleSeg::OnToolVertex {
                            side: s.half,
                            of: of.name,
                        },
                    ),
                    ent(s.ix, EntityKey::Vertex(v)),
                )?;
            } else {
                return Err(bug(
                    "on-plane vertex with neither a SplitEdge record nor an operand identity",
                ));
            }
        }
        // A plane crosses a straight edge at most once, and an arc it
        // crosses twice leaves both crossings on both sides: ranked
        // along the crossed edge (N2).
        for (crossed, (parent, verts)) in crossings {
            let base = name1(
                EntityKind::Vertex,
                node,
                RoleSeg::CrossingVertex {
                    side: s.half,
                    edge: parent.name.clone(),
                },
            );
            let crossings = verts
                .iter()
                .map(|&v| Ok((ent(s.ix, EntityKey::Vertex(v)), vertex_point(body, v)?)))
                .collect::<Result<Vec<_>, NamingError>>()?;
            let edge = CrossedEdge {
                body: target_body,
                table: target_table,
                edge: crossed,
                name: &parent.name,
            };
            rank_crossings(t, tie, parent.tied, &base, &edge, &crossings, bnd)?;
        }
    }
    tie.flush(t)?;
    for ((slot, base), (from_tie, edges)) in edge_groups {
        let s = &sides[slot];
        name_edge_pieces(t, tie, from_tie, &base, s.body, s.ix, &edges)?;
    }
    Ok(())
}

/// One boolean operand under naming.
pub(crate) struct OperandCtx<'a, T: Decide> {
    /// The operand's node (error context).
    pub node: RecipeNodeId,
    /// Its name table (total over its body).
    pub table: &'a NameTable,
    /// Its body (the carriers its crossed edges are ranked along).
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

    /// The same side, its key mapped through `f`.
    fn map<J>(self, f: impl FnOnce(K) -> J) -> OpSide<J> {
        match self {
            OpSide::A(k) => OpSide::A(f(k)),
            OpSide::B(k) => OpSide::B(f(k)),
        }
    }

    /// The same side, carrying another key.
    fn with<J>(self, j: J) -> OpSide<J> {
        match self {
            OpSide::A(_) => OpSide::A(j),
            OpSide::B(_) => OpSide::B(j),
        }
    }

    /// The operand and the key.
    fn of_operand(self) -> (topo::Operand, K) {
        match self {
            OpSide::A(k) => (topo::Operand::A, k),
            OpSide::B(k) => (topo::Operand::B, k),
        }
    }

    /// Which operand this is, without the key — the kernel's own
    /// dataless side ([`topo::Operand`]).
    fn operand(self) -> topo::Operand {
        match self {
            OpSide::A(_) => topo::Operand::A,
            OpSide::B(_) => topo::Operand::B,
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

impl OpSide<EntityKey> {
    /// Where a group whose members descend from this operand entity
    /// comes from, for the group record (`names::groups`): the A
    /// operand's entity, which a union's next fold step carries by key,
    /// or the B operand's, which no later step carries.
    fn parent(self) -> Parent {
        match self {
            OpSide::A(k) => Parent::AOperand(ent(0, k)),
            OpSide::B(_) => Parent::Elsewhere,
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

/// **Which operand face each face of a boolean result descends from**,
/// by entity: the operand and the root face in that operand's space,
/// read through the graft rows and the fragment rows. A union's fold
/// reads it step by step to follow each face to the member faces it
/// descends from.
pub(super) struct FaceDescent<'n> {
    naming: &'n topo::BooleanNaming,
    inv_faces: BTreeMap<FaceKey, FaceKey>,
    a_rows: BTreeMap<FaceKey, FaceKey>,
    b_rows: BTreeMap<FaceKey, FaceKey>,
}

impl<'n> FaceDescent<'n> {
    pub(super) fn of(naming: &'n topo::BooleanNaming) -> Self {
        Self {
            naming,
            inv_faces: naming.graft_faces.iter().map(|&(s, d)| (d, s)).collect(),
            a_rows: naming.face_fragments_a.iter().copied().collect(),
            b_rows: naming.face_fragments_b.iter().copied().collect(),
        }
    }

    /// The operand face result face `f` descends from. A merged face
    /// descends from each face it absorbed as well: see
    /// [`FaceDescent::merged`].
    pub(super) fn result_face(&self, f: FaceKey) -> Result<(topo::Operand, FaceKey), NamingError> {
        let (operand, k) = operand_key(self.naming, &self.inv_faces, f)?.0.of_operand();
        Ok((operand, self.clone_face(operand, k)?))
    }

    /// The operand face a face in `operand`'s CLONE keys is a fragment
    /// of (a discarded face's key is one).
    pub(super) fn clone_face(
        &self,
        operand: topo::Operand,
        f: FaceKey,
    ) -> Result<FaceKey, NamingError> {
        match operand {
            topo::Operand::A => chase(&self.a_rows, f),
            topo::Operand::B => chase(&self.b_rows, f),
        }
    }

    /// Merged result face → every face it holds, itself first, in
    /// result keys before the merge.
    pub(super) fn merged(&self) -> BTreeMap<FaceKey, Vec<FaceKey>> {
        self.naming
            .merge_groups
            .iter()
            .map(|(kept, absorbed)| {
                (
                    *kept,
                    core::iter::once(*kept)
                        .chain(absorbed.iter().copied())
                        .collect(),
                )
            })
            .collect()
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
) -> Result<Emitted, NamingError> {
    let bnd = band(tol)?;
    let bug = |what| NamingError::Emission { what };
    let mut t = NameTable::new();
    let mut rec = GroupRecord::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;

    let inv_faces: BTreeMap<FaceKey, FaceKey> =
        naming.graft_faces.iter().map(|&(s, d)| (d, s)).collect();
    let inv_edges: BTreeMap<EdgeKey, EdgeKey> =
        naming.graft_edges.iter().map(|&(s, d)| (d, s)).collect();
    // Result → B key for every key a grafted record can name: the
    // grafted edges and the dead-on-arrival keys standing for B edges
    // that died before the graft.
    let lineage_inv: BTreeMap<EdgeKey, EdgeKey> = naming
        .graft_edges
        .iter()
        .chain(&naming.graft_dead_edges)
        .map(|&(s, d)| (d, s))
        .collect();
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
    // Operand face → the merged faces it survives in: those faces
    // descend from it too, so they are members of its group
    // (`names::groups`).
    let mut merged_into: BTreeMap<OpSide<FaceKey>, Vec<FaceKey>> = BTreeMap::new();
    // Merged face → its parent's name, the one a `Borders` wall cites.
    let mut merged_names: BTreeMap<FaceKey, StableName> = BTreeMap::new();
    // The merged parents in the order the merges first list them, each
    // keyed by the operand faces its merges list: a parent is a set of
    // entities, never a name, so tied parents spelled alike stay apart.
    let mut merged_parents: Vec<MergedParent> = Vec::new();
    let mut parent_at: BTreeMap<BTreeSet<OpSide<FaceKey>>, usize> = BTreeMap::new();
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
        let parents: BTreeSet<OpSide<FaceKey>> = descents.iter().copied().collect();
        merged_descents.insert(*kept, descents);
        for &d in &parents {
            merged_into.entry(d).or_default().push(*kept);
        }
        let merged =
            canonical::minted(name1(EntityKind::Face, node, RoleSeg::Merged(constituents)));
        merged_names.insert(*kept, merged.clone());
        // Two merges that list the same operand faces hold one parent
        // between them (N2: a parent is a set of entities), so the
        // faces it is held as are named together below.
        let at = *parent_at.entry(parents.clone()).or_insert_with(|| {
            merged_parents.push(MergedParent {
                name: merged,
                faces: Vec::new(),
                parents,
                from_tie: false,
            });
            merged_parents.len() - 1
        });
        let held = &mut merged_parents[at];
        held.faces.push(*kept);
        held.from_tie |= from_tie;
        handled.insert(*kept);
    }
    let mut groups: BTreeMap<OpSide<FaceKey>, Vec<FaceKey>> = BTreeMap::new();
    for (f, _) in body.faces() {
        if !handled.contains(&f) {
            groups.entry(descend_face(f)?).or_default().push(f);
        }
    }
    // A parent that survives only inside merged faces still has a
    // group: those faces.
    for d in merged_into.keys() {
        groups.entry(*d).or_default();
    }
    let mut obstacles = Obstacles::new();
    obstacles.record(naming, body, |operand, f| {
        Ok(BTreeSet::from([match operand {
            topo::Operand::A => OpSide::A(chase(&a_rows, f)?),
            topo::Operand::B => OpSide::B(chase(&b_rows, f)?),
        }]))
    })?;
    // A wall is cited by its parent's name: a merged face's, or the
    // name of the operand face it descends from in its operand, the
    // space the pair reads its operands in (a union's fold collapses it
    // like any other embedded name).
    let wall = |g: FaceKey| -> Result<StableName, NamingError> {
        if let Some(n) = merged_names.get(&g) {
            return Ok(n.clone());
        }
        Ok((*operand_face_name(descend_face(g)?)?.name).clone())
    };
    // Every face that holds part of operand face `d`: its unmerged
    // pieces, then the merged faces that list it.
    let held_by = |d: &OpSide<FaceKey>| -> Vec<FaceKey> {
        groups
            .get(d)
            .into_iter()
            .chain(merged_into.get(d))
            .flatten()
            .copied()
            .collect()
    };
    for held in merged_parents {
        let name = held.name;
        rec.record(
            &name,
            held.faces
                .iter()
                .map(|&f| ent(0, EntityKey::Face(f)))
                .collect(),
            Parent::Elsewhere,
        );
        if let [one] = held.faces[..] {
            put(
                &mut t,
                &mut tie,
                held.from_tie,
                name,
                ent(0, EntityKey::Face(one)),
            )?;
            continue;
        }
        let others: Vec<FaceKey> = held
            .parents
            .iter()
            .flat_map(&held_by)
            .filter(|f| !held.faces.contains(f))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        name_parent_faces(
            &mut t,
            &mut tie,
            held.from_tie,
            name,
            &held.faces,
            &others,
            (&obstacles, body, &held.parents),
            &wall,
        )?;
    }
    for (d, members) in &groups {
        let root_name = operand_face_name(*d)?;
        let from_tie = root_name.tied;
        let base = name1(EntityKind::Face, node, d.wrap(root_name.name));
        let in_merged = merged_into.get(d).map_or(&[][..], Vec::as_slice);
        rec.record(
            &base,
            held_by(d)
                .into_iter()
                .map(|f| ent(0, EntityKey::Face(f)))
                .collect(),
            d.map(EntityKey::Face).parent(),
        );
        name_parent_faces(
            &mut t,
            &mut tie,
            from_tie,
            base,
            members,
            in_merged,
            (&obstacles, body, &BTreeSet::from([*d])),
            &wall,
        )?;
    }
    tie.flush(&mut t)?;

    let edge_groups = name_boolean_edges(
        node,
        &mut rec,
        body,
        naming,
        a,
        b,
        &inv_edges,
        &lineage_inv,
        &seam_set,
        &inc,
        &descend_face,
        &operand_face_name,
        &merged_descents,
        bnd,
    )?;
    name_boolean_vertices(
        node,
        &mut t,
        &mut tie,
        &mut rec,
        body,
        naming,
        &inv_vertices,
        a,
        b,
        &inc,
        &edge_groups,
        bnd,
    )?;
    tie.flush(&mut t)?;
    for g in &edge_groups {
        name_edge_pieces(&mut t, &mut tie, g.from_tie, &g.base, body, 0, &g.edges)?;
    }
    tie.flush(&mut t)?;

    super::emit::check_total(&t, body, 0)?;
    Ok(Emitted::new(t, rec))
}

/// A merged parent in a pair boolean: its `Merged` name, the faces it
/// is held as, the operand faces its merges list, and whether any of
/// those is tied.
struct MergedParent {
    name: StableName,
    faces: Vec<FaceKey>,
    parents: BTreeSet<OpSide<FaceKey>>,
    from_tie: bool,
}

/// Names the faces one parent is held as (N2/N3): a lone face whose
/// parent no merge shares is the parent, `base`; otherwise each of
/// `pieces` is `base` + `Fragment(Borders)` over the divider walls
/// [`Obstacles::split`] finds, reading `merged` as the parent's other
/// faces — ones that hold part of its region but are named apart from
/// `pieces`, such as a merge that lists it beside other faces, or an
/// operand face's unmerged piece beside a merged parent's faces — and
/// the pieces one set does not tell apart are the tie. With any
/// `merged` the bare `base` does not name the whole parent, so no
/// piece takes it. The pair boolean and
/// the union's end pass both name their parents here.
#[allow(clippy::too_many_arguments)]
pub(super) fn name_parent_faces<T: geom_core::Real, K: Ord + Clone>(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    base: StableName,
    pieces: &[FaceKey],
    merged: &[FaceKey],
    split: (&Obstacles<K>, &Body<T>, &BTreeSet<K>),
    wall: impl FnMut(FaceKey) -> Result<StableName, NamingError>,
) -> Result<(), NamingError> {
    match (pieces, merged) {
        ([], _) => Ok(()),
        ([one], []) => Ok(put(t, tie, from_tie, base, ent(0, EntityKey::Face(*one)))?),
        _ => {
            let (obstacles, body, parent) = split;
            for (walls, faces) in obstacles.split(body, parent, pieces, merged, wall)? {
                let mut name = base.clone();
                name.path.push(RoleSeg::Fragment(Qualifier::Borders(walls)));
                let ents = faces.iter().map(|&f| ent(0, EntityKey::Face(f))).collect();
                mint_candidates(t, tie, from_tie, canonical::minted(name), ents)?;
            }
            Ok(())
        }
    }
}

/// Boolean edges, grouped by parent: `Seam` for zip-minted edges,
/// `FromA`/`FromB` for operand-descended ones. A group's pieces are
/// qualified once the vertices are named ([`EdgeGroup`]).
#[allow(clippy::too_many_arguments)]
fn name_boolean_edges<T: Decide>(
    node: RecipeNodeId,
    rec: &mut GroupRecord,
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    inv_edges: &BTreeMap<EdgeKey, EdgeKey>,
    lineage_inv: &BTreeMap<EdgeKey, EdgeKey>,
    seam_set: &BTreeSet<EdgeKey>,
    inc: &Incidence,
    descend_face: &impl Fn(FaceKey) -> Result<OpSide<FaceKey>, NamingError>,
    operand_face_name: &impl Fn(OpSide<FaceKey>) -> Result<Upstream, NamingError>,
    merged_descents: &BTreeMap<FaceKey, Vec<OpSide<FaceKey>>>,
    bnd: geom_core::Band,
) -> Result<Vec<EdgeGroup>, NamingError> {
    let bug = |what| NamingError::Emission { what };

    // ---- Seam edges (zip-listed AND derived — see below), grouped
    // by their (fA, fB) operand pair. A derived chord between two
    // SAME-operand faces (the collinear channel-cut lane re-mints a
    // sub-edge of an operand edge as a chord) descends instead to an
    // operand edge its two parent faces share — combinatorial adjacency
    // of emitted anchors, not matching, while the pair shares one edge.
    // When it shares SEVERAL — collinear pieces of one line, left by an
    // earlier union step — the chord's geometry picks the piece it lies
    // within (`rim_holding`); when nothing picks one,
    // `NamingError::SharedRim` is what the arm below says. ----
    enum ChordKind {
        Cross(Upstream, Upstream),
        SameA(EdgeKey),
        SameB(EdgeKey),
    }
    // A face's descent for CHORD purposes: a plain face descends as
    // itself; a MERGED face (M4 PR 5, N3 live) reads through to its
    // unique constituent on side `want` — the seam's mint-time operand
    // identity survives the glue. Several same-side constituents are a
    // legal body no rule reads — a declared union's merge can glue
    // faces of two members of one assembly into one face — and refuse
    // as the missing rule (`NamingError::MergedChordConstituents`).
    let chord_descent =
        |e: EdgeKey, f: FaceKey, want: topo::Operand| -> Result<OpSide<FaceKey>, NamingError> {
            let Some(ds) = merged_descents.get(&f) else {
                return descend_face(f);
            };
            // Constituent fragments of ONE operand face share a
            // descent — dedup before the uniqueness demand.
            let mut hits: Vec<OpSide<FaceKey>> =
                ds.iter().filter(|d| d.operand() == want).copied().collect();
            hits.sort_unstable();
            hits.dedup();
            match hits.as_slice() {
                [] => Err(bug("merged face lacks the needed operand-side constituent")),
                [one] => Ok(*one),
                several => Err(NamingError::MergedChordConstituents {
                    edge: e,
                    face: f,
                    several: several.len(),
                }),
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
    let chord_kind = |e: EdgeKey, own: Option<topo::Operand>| -> Result<ChordKind, NamingError> {
        // **One rule for a chord between two faces of ONE operand**,
        // merged or not. The premise this caller asks under: it did not
        // build these bodies, it DESCENDED two result faces into one of
        // them and guesses the pair carries this chord's rim. One shared
        // edge is that rim, when the chord lies within it
        // ([`chord_on_rim`]); several are the pieces of one line, and the
        // chord picks the piece it lies within ([`rim_holding`]). A pair
        // with no rim, or pieces the chord picks none of, refutes the
        // guess, not the body — so the answer is classified here rather
        // than at the walk. `off_rim` says what a chord off its one rim
        // is, which depends on why the faces were paired.
        //
        // `chord_on_rim` is a straight-segment test, so a CURVED rim
        // (an arc between a cylinder's side and a cap) is taken without
        // it. Only unmerged faces can meet along one: merged faces are
        // planar (F7), and two planes meet in a line.
        let same_side_rim = |op: &OperandCtx<'_, T>,
                             f0: FaceKey,
                             f1: FaceKey,
                             off_rim: &dyn Fn(EdgeKey) -> NamingError|
         -> Result<EdgeKey, NamingError> {
            let found = match rim_between(op.body, f0, f1)? {
                Rim::One(rim) => {
                    let straight = topo::query::edge_carrier_kind(op.body, rim)
                        == Some(topo::query::CurveKind::Line);
                    return if !straight || chord_on_rim(body, e, op.body, rim, bnd)? {
                        Ok(rim)
                    } else {
                        Err(off_rim(rim))
                    };
                }
                Rim::NotOne(RimShare::Several) => {
                    if let Some(rim) = rim_holding(body, e, op.body, f0, f1, bnd)? {
                        return Ok(rim);
                    }
                    RimShare::Several
                }
                Rim::NotOne(found) => found,
            };
            Err(NamingError::SharedRim {
                node: op.node,
                face: f0,
                other: f1,
                found,
            })
        };
        // Two UNMERGED faces of one operand whose closures meet at the
        // chord meet along their one shared rim, so a chord off it is a
        // body the emitter cannot read — a kernel fact, not a rule.
        let unmerged_off_rim = |_| NamingError::Emission {
            what: "a chord between two unmerged faces of one operand lies off the rim they share",
        };
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
                (chord_descent(e, faces[0], d1.operand().other())?, d1)
            }
            (false, true) => {
                let d0 = descend_face(faces[0])?;
                (d0, chord_descent(e, faces[1], d0.operand().other())?)
            }
            (true, true) => {
                let Some(side) = own else {
                    return Err(NamingError::MergedChord { edge: e });
                };
                let (d0, d1) = (
                    chord_descent(e, faces[0], side)?,
                    chord_descent(e, faces[1], side)?,
                );
                let (op, f0) = d0.of(a, b);
                let (_, f1) = d1.of(a, b);
                let rim = same_side_rim(op, f0, f1, &|rim| NamingError::MergedChordOffRim {
                    edge: e,
                    node: op.node,
                    rim,
                })?;
                return Ok(match side {
                    topo::Operand::A => ChordKind::SameA(rim),
                    topo::Operand::B => ChordKind::SameB(rim),
                });
            }
        };
        Ok(match (d0, d1) {
            (OpSide::A(_), OpSide::B(_)) => {
                ChordKind::Cross(operand_face_name(d0)?, operand_face_name(d1)?)
            }
            (OpSide::B(_), OpSide::A(_)) => {
                ChordKind::Cross(operand_face_name(d1)?, operand_face_name(d0)?)
            }
            // Two faces of one operand: the rim they share
            // (`same_side_rim`).
            (OpSide::A(fa0), OpSide::A(fa1)) => {
                ChordKind::SameA(same_side_rim(a, fa0, fa1, &unmerged_off_rim)?)
            }
            (OpSide::B(fb0), OpSide::B(fb1)) => {
                ChordKind::SameB(same_side_rim(b, fb0, fb1, &unmerged_off_rim)?)
            }
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
    // A GRAFTED B side chases in the result body — the graft forwards
    // every record into result keys — stopping at the first key whose B
    // preimage B's table names, and reads the root back to B. A B
    // ancestor that died before the graft is a dead result key with a
    // `graft_dead_edges` row, so the walk still reaches it; a broken
    // chain (a middle fragment of a doubly-pierced edge dies PRE-graft,
    // unnamed) returns that non-resolving B key, which the
    // `resolves == false` route below hands to `chord_kind`, the rescue
    // the A lane gets. A walk that leaves the graft rows is a record
    // the graft did not forward, and refuses.
    let chase_b = |e: EdgeKey| -> Result<EdgeKey, NamingError> {
        let named_in_b = |k: &EdgeKey| b.table.name_of(&ent(0, EntityKey::Edge(*k))).is_some();
        let root = body.split_root(e, |r| lineage_inv.get(&r).is_none_or(named_in_b))?;
        lineage_inv
            .get(&root)
            .copied()
            .ok_or(bug("a grafted edge's split lineage left the graft rows"))
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
            KeySpace::Graft => chase_b(e)?,
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
            match chord_kind(e, Some(root.operand()))? {
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
    let mut out = Vec::with_capacity(seam_groups.len() + groups.len());
    for ((fa, fb), (from_tie, edges)) in seam_groups {
        let base = name1(EntityKind::Edge, node, RoleSeg::Seam { a: fa, b: fb });
        rec.record_by_name(
            &base,
            edges.iter().map(|&e| ent(0, EntityKey::Edge(e))).collect(),
            from_tie,
        );
        out.push(EdgeGroup {
            base,
            from_tie,
            edges,
        });
    }
    for (root, edges) in groups {
        let (op, root_key) = root.of(a, b);
        let inner = upstream_name(op.table, op.node, ent(0, EntityKey::Edge(root_key)))?;
        let base = name1(EntityKind::Edge, node, root.wrap(inner.name));
        rec.record(
            &base,
            edges.iter().map(|&e| ent(0, EntityKey::Edge(e))).collect(),
            root.map(EntityKey::Edge).parent(),
        );
        out.push(EdgeGroup {
            base,
            from_tie: inner.tied,
            edges,
        });
    }
    Ok(out)
}

/// The edges of a pair boolean's result that share one parent: its
/// name, whether that descends from a tie, and the edges. Their
/// vertices are named from `base` (a vertex cites an edge by its head),
/// and the edges then by their ends ([`name_edge_pieces`]).
struct EdgeGroup {
    base: StableName,
    from_tie: bool,
    edges: Vec<EdgeKey>,
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
    rec: &mut GroupRecord,
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    inv_vertices: &BTreeMap<VertexKey, VertexKey>,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    inc: &Incidence,
    edge_groups: &[EdgeGroup],
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };
    // Each edge's parent, the head a vertex cites it by, and whether
    // that descends from a tie.
    let edge_base: BTreeMap<EdgeKey, (&StableName, bool)> = edge_groups
        .iter()
        .flat_map(|g| g.edges.iter().map(move |&e| (e, (&g.base, g.from_tie))))
        .collect();
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
    let operand_identity =
        |k: VertexKey| -> Result<Option<(StableName, bool, Parent)>, NamingError> {
            let (side, _) = operand_key(naming, inv_vertices, k)?;
            Ok(named_in(side)?.map(|u| {
                let parent = side.map(EntityKey::Vertex).parent();
                (
                    name1(EntityKind::Vertex, node, side.wrap(u.name)),
                    u.tied,
                    parent,
                )
            }))
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
        if let Some((name, from_tie, parent)) = identity {
            // A vertex carried through whole: a group of one.
            rec.record(&name, vec![ent(0, EntityKey::Vertex(v))], parent);
            put(t, tie, from_tie, name, ent(0, EntityKey::Vertex(v)))?;
            continue;
        }
        // Seam vertex: parents from incident edges' names.
        let edges = inc
            .vertex_edges
            .get(&v)
            .ok_or_else(|| bug("seam vertex without incident edges"))?;
        // The parentage a seam vertex is named from is read off the
        // incident edges' HEADS (N2: never a piece's qualifier, which is
        // named from its ends) and put straight back into this vertex's
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
        // B1: a seam vertex reads its parentage off the incident edges'
        // parents, so a parent descended from a tie makes the vertex
        // name tie-descended too.
        let mut from_tie = false;
        for &e in edges {
            let Some(&(ename, tied)) = edge_base.get(&e) else {
                return Err(bug("seam vertex incident to an unnamed edge"));
            };
            from_tie |= tied;
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
        //
        // EVERY row for this vertex is read, not the first: a vertex
        // coincident with several vertices of the other operand has a
        // row for each, and its name must not depend on which one the
        // reduction wrote first (`one_partner`).
        let rc = &naming.reduction_contacts;
        let (partner_a, partner_b): (Option<Upstream>, Option<Upstream>) =
            match operand_key(naming, inv_vertices, v)?.0 {
                OpSide::A(ka) => {
                    let named = rc
                        .vv
                        .iter()
                        .filter(|r| r.a == ka)
                        .map(|r| named_in(OpSide::B(r.b)))
                        .collect::<Result<Vec<_>, _>>()?;
                    (None, one_partner(v, named.into_iter().flatten().collect())?)
                }
                OpSide::B(kb) => {
                    let named = rc
                        .vv
                        .iter()
                        .filter(|r| r.b == kb)
                        .map(|r| named_in(OpSide::A(r.a)))
                        .collect::<Result<Vec<_>, _>>()?;
                    (one_partner(v, named.into_iter().flatten().collect())?, None)
                }
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
        rec.record_by_name(
            &base,
            verts
                .iter()
                .map(|&v| ent(0, EntityKey::Vertex(v)))
                .collect(),
            from_tie,
        );
        if verts.len() == 1 {
            put(t, tie, from_tie, base, ent(0, EntityKey::Vertex(verts[0])))?;
            continue;
        }
        // Same pair crossing more than once: ranked along the edge
        // parent (the A side's where both are edges).
        let crossed = match crossed_edge(&pa, a.table) {
            Some(k) => Some((a, k, &pa)),
            None => crossed_edge(&pb, b.table).map(|k| (b, k, &pb)),
        };
        let Some((op, k, parent)) = crossed else {
            let ents = verts
                .iter()
                .map(|&v| ent(0, EntityKey::Vertex(v)))
                .collect();
            mint_candidates(t, tie, from_tie, base, ents)?;
            continue;
        };
        let crossings = verts
            .iter()
            .map(|&v| Ok((ent(0, EntityKey::Vertex(v)), vertex_point(body, v)?)))
            .collect::<Result<Vec<_>, NamingError>>()?;
        let edge = CrossedEdge {
            body: op.body,
            table: op.table,
            edge: k,
            name: parent,
        };
        rank_crossings(t, tie, from_tie, &base, &edge, &crossings, bnd)?;
    }
    Ok(())
}

/// The one contact-record partner a seam vertex is named by, from the
/// named partners of EVERY row that pairs it: none is no partner, one
/// name — however many rows carry it — is that partner (tie-descended
/// if any row's is), and several distinct names refuse
/// ([`NamingError::SeamVertexPartners`]) rather than choose.
fn one_partner(vertex: VertexKey, named: Vec<Upstream>) -> Result<Option<Upstream>, NamingError> {
    let mut distinct: Vec<Upstream> = Vec::new();
    for u in named {
        match distinct.iter_mut().find(|d| d.name == u.name) {
            Some(d) => d.tied |= u.tied,
            None => distinct.push(u),
        }
    }
    if distinct.len() > 1 {
        let mut candidates: Vec<StableName> = distinct.iter().map(|u| (*u.name).clone()).collect();
        candidates.sort();
        return Err(NamingError::SeamVertexPartners { vertex, candidates });
    }
    Ok(distinct.pop())
}

/// The operand edge a seam vertex's parent name denotes, if it is a
/// uniquely named edge of that operand's table.
fn crossed_edge(parent: &StableName, table: &NameTable) -> Option<EdgeKey> {
    if parent.kind != EntityKind::Edge {
        return None;
    }
    match table.lookup(parent) {
        Some(Entry::Unique(e)) => match e.key {
            EntityKey::Edge(k) => Some(k),
            _ => None,
        },
        _ => None,
    }
}

/// Where a point lies against a [`Segment`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OnSegment {
    /// Off the segment's line, or on it past one of its ends.
    Off,
    /// At its start.
    AtStart,
    /// At its end.
    AtEnd,
    /// On the line, strictly between the ends.
    Inside,
}

/// **An edge's closed segment, start to end** — the one place a point's
/// position against an edge is read: [`Segment::place`] says whether it
/// lies on the segment.
pub(super) struct Segment<T: Decide> {
    q0: Point3<T>,
    q1: Point3<T>,
    d: Vec3<T>,
    len: T,
}

impl<T: Decide> Segment<T> {
    /// Edge `e` of `body`, from its `he_plus` start to its end.
    pub(super) fn of_edge(body: &Body<T>, e: EdgeKey) -> Result<Self, NamingError> {
        let (v0, v1) = edge_ends(body, e)?;
        let (q0, q1) = (vertex_point(body, v0)?, vertex_point(body, v1)?);
        let d = q1 - q0;
        Ok(Segment {
            q0,
            q1,
            d,
            len: d.norm(),
        })
    }

    /// Whether `p` lies on the segment's LINE, decided through
    /// `predicate` over its distance off it: the first verdict
    /// [`Segment::place`] takes.
    pub(super) fn on_line(
        &self,
        p: Point3<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<bool, NamingError> {
        let off = decide(
            predicate,
            Margin::over_lever((p - self.q0).cross(self.d).norm(), self.len),
            bnd,
        )
        .map_err(|source| NamingError::Escalated { predicate, source })?;
        Ok(off == Sign::Zero)
    }

    /// Where `p` lies, decided through `predicate` over lengths: first
    /// its distance off the line, and only for a point ON it the
    /// distances past each end (a `Negative` one is off the segment, a
    /// `Zero` one at that end). A point off the line costs one verdict.
    /// An in-band margin escalates typed.
    pub(super) fn place(
        &self,
        p: Point3<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<OnSegment, NamingError> {
        if !self.on_line(p, predicate, bnd)? {
            return Ok(OnSegment::Off);
        }
        let sign = |m: Margin<T>| {
            decide(predicate, m, bnd).map_err(|source| NamingError::Escalated { predicate, source })
        };
        let past0 = sign(Margin::over_lever((p - self.q0).dot(self.d), self.len))?;
        let past1 = sign(Margin::over_lever((self.q1 - p).dot(self.d), self.len))?;
        Ok(match (past0, past1) {
            (Sign::Zero, Sign::Zero | Sign::Positive) => OnSegment::AtStart,
            (Sign::Positive, Sign::Zero) => OnSegment::AtEnd,
            (Sign::Positive, Sign::Positive) => OnSegment::Inside,
            _ => OnSegment::Off,
        })
    }
}

/// Whether result edge `chord` lies on operand edge `rim` of
/// `op_body`: both of its ends on the rim's closed [`Segment`], through
/// [`CHORD_ON_RIM`].
fn chord_on_rim<T: Decide>(
    body: &Body<T>,
    chord: EdgeKey,
    op_body: &Body<T>,
    rim: EdgeKey,
    bnd: geom_core::Band,
) -> Result<bool, NamingError> {
    let seg = Segment::of_edge(op_body, rim)?;
    let (c0, c1) = edge_ends(body, chord)?;
    for v in [c0, c1] {
        if seg.place(vertex_point(body, v)?, CHORD_ON_RIM, bnd)? == OnSegment::Off {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Of the SEVERAL edges operand faces `f0` and `f1` share in
/// `op_body`, the one result edge `chord` lies within
/// ([`chord_on_rim`]) — `None` unless exactly one does.
///
/// Two faces of one operand share several edges when their common
/// line is cut into pieces: a union step before this one met the pair
/// along one rim and left it in collinear pieces, split at the
/// vertices where the members' ends meet it. "The rim they share" is
/// then not one edge, and adjacency cannot say which piece a chord
/// derived on that line belongs to; its geometry can. The pieces are
/// disjoint but for their shared ends, so a chord strictly inside one
/// lies within no other, and the answer is one piece or none. None,
/// or more than one, and the caller refuses the pair as it did
/// before: the chord does not pick a piece, and no other rule does.
fn rim_holding<T: Decide>(
    body: &Body<T>,
    chord: EdgeKey,
    op_body: &Body<T>,
    f0: FaceKey,
    f1: FaceKey,
    bnd: geom_core::Band,
) -> Result<Option<EdgeKey>, NamingError> {
    let mut holding = None;
    for rim in rims_between(op_body, f0, f1)? {
        if chord_on_rim(body, chord, op_body, rim, bnd)? {
            if holding.is_some() {
                return Ok(None);
            }
            holding = Some(rim);
        }
    }
    Ok(holding)
}

/// A ranked group's size as its names' `OrderAlong { of }`, through
/// [`super::emit::to_u32`]: a saturated `of` would be a wrong count in
/// every name of the group, and two different oversized groups would
/// spell one.
fn group_count(n: usize) -> Result<u32, NamingError> {
    super::emit::to_u32(
        n,
        "a ranked group has more members than a name's rank count holds",
    )
}

/// **Names the pieces of one parent edge** (N2): a lone piece is
/// `base`, and each of several is `base` + `Fragment(Ends)`, the sorted
/// pair of its two end vertices' names as `t` publishes them, read off
/// body `ix`. Pieces with equal pairs are N4's tie. Every end vertex is
/// named before this runs, so `t` holds its name.
///
/// # Errors
///
/// [`NamingError::Emission`] for a piece ending at a vertex `t` does
/// not name, and the insert doors' own refusals.
pub(super) fn name_edge_pieces<T: geom_core::Real>(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    base: &StableName,
    body: &Body<T>,
    ix: u32,
    edges: &[EdgeKey],
) -> Result<(), NamingError> {
    if let [one] = edges {
        return Ok(put(
            t,
            tie,
            from_tie,
            base.clone(),
            ent(ix, EntityKey::Edge(*one)),
        )?);
    }
    let mut pieces = Vec::with_capacity(edges.len());
    for &e in edges {
        let (v0, v1) = edge_ends(body, e)?;
        let mut ends = Vec::with_capacity(2);
        for v in [v0, v1] {
            ends.push(
                t.name_of(&ent(ix, EntityKey::Vertex(v)))
                    .ok_or(NamingError::Emission {
                        what: "an edge piece ends at a vertex the table does not name",
                    })?
                    .clone(),
            );
        }
        ends.sort();
        pieces.push((Qualifier::Ends(ends), ent(ix, EntityKey::Edge(e))));
    }
    mint_qualified(t, tie, from_tie, base, pieces)
}

/// Mints each piece as `base` + `Fragment(q)` over its qualifier, the
/// pieces that share one qualifier as N4's tie.
fn mint_qualified(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    base: &StableName,
    pieces: impl IntoIterator<Item = (Qualifier, super::table::EntityRef)>,
) -> Result<(), NamingError> {
    let mut by_qualifier: BTreeMap<Qualifier, Vec<super::table::EntityRef>> = BTreeMap::new();
    for (q, e) in pieces {
        by_qualifier.entry(q).or_default().push(e);
    }
    for (q, ents) in by_qualifier {
        let mut name = base.clone();
        name.path.push(RoleSeg::Fragment(q));
        mint_candidates(t, tie, from_tie, name, ents)?;
    }
    Ok(())
}

/// The way the crossings of edge `e` of `body`, named `name` in its
/// own table `table`, are ranked along it (N2): `Some(true)` along the
/// edge as `body` stores it, `Some(false)` against it, and `None` where
/// no orientation is defined, which ties the crossings.
///
/// An edge on a seam line runs as the loop of its pair's first side
/// runs along it, the side found by name among the edge's two faces
/// (`seam_pair::a_side_is_first`). A pair whose two sides carry one
/// name has no first side, nor does one whose faces the names do not
/// tell apart; both tie. Any other edge runs as stored.
pub(super) fn crossed_edge_orientation<T: geom_core::Real>(
    body: &Body<T>,
    table: &NameTable,
    e: EdgeKey,
    name: &StableName,
) -> Result<Option<bool>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let Some((a, b)) = seam_pair::seam_edge_sides(name) else {
        return Ok(Some(true));
    };
    if a == b {
        return Ok(None);
    }
    let edge = body
        .get_edge(e)
        .ok_or_else(|| bug("a crossed seam edge is not live in its body"))?;
    let mut names = Vec::with_capacity(2);
    for he in [edge.he_plus, edge.he_minus] {
        let face = body
            .get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .map(|l| l.face)
            .ok_or_else(|| bug("a crossed seam edge's half-edge lies on no face"))?;
        names.push(
            table
                .name_of(&ent(0, EntityKey::Face(face)))
                .ok_or_else(|| bug("a crossed seam edge's face is unnamed"))?,
        );
    }
    Ok(seam_pair::a_side_is_first(names[0], names[1], a, b))
}

/// Where `p`, a point on edge `e` of `body`, lies along it: the edge's
/// carrier's own parameter, increasing as the edge runs as stored.
/// `None` for a carrier with no closed-form parameter here (a NURBS
/// curve). The parameter is read near the middle of the edge's
/// certified interval, so a closed edge's crossings, which lie inside
/// it, are read without the period's cut between them.
pub(super) fn param_along<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    p: Point3<T>,
) -> Result<Option<T>, NamingError> {
    use geom::Curve3;
    let bug = |what| NamingError::Emission { what };
    let edge = body
        .get_edge(e)
        .ok_or_else(|| bug("a crossed edge is not live in its body"))?;
    let curve = body
        .get_curve_geom(edge.curve)
        .ok_or_else(|| bug("a crossed edge's carrier is dangling"))?
        .certified()
        .ok_or_else(|| bug("a crossed edge carries no certified carrier"))?;
    let (t0, t1) = curve.params();
    let near = (t0 + t1) * T::from_f64(0.5);
    Ok(match curve.carrier() {
        Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => {
            let w = p - *center;
            let (x, y) = (w.dot(*u_ref) / *major, w.dot(axis.cross(*u_ref)) / *minor);
            let (s, c) = near.sin_cos();
            Some(near + (y * c - x * s).atan2(x * c + y * s))
        }
        carrier => carrier.param_near(p, near),
    })
}

/// The edge a group of crossings lies on: edge `edge` of `body`, named
/// `name` in its own table `table`.
pub(super) struct CrossedEdge<'a, T: geom_core::Real> {
    pub(super) body: &'a Body<T>,
    pub(super) table: &'a NameTable,
    pub(super) edge: EdgeKey,
    pub(super) name: &'a StableName,
}

/// **Ranks the crossings of one edge by one face** (N2): a lone crossing
/// is `base`, and several, each an entity and the point it lies at on
/// the crossed edge, take `base` + `Fragment(OrderAlong)` by the edge's
/// carrier parameter, oriented by [`crossed_edge_orientation`]; with no
/// orientation or no parameter they tie.
pub(super) fn rank_crossings<T: Decide>(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    base: &StableName,
    crossed: &CrossedEdge<'_, T>,
    crossings: &[(super::table::EntityRef, Point3<T>)],
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let keys: Vec<super::table::EntityRef> = crossings.iter().map(|&(e, _)| e).collect();
    if let [one] = keys.as_slice() {
        return Ok(put(t, tie, from_tie, base.clone(), *one)?);
    }
    let CrossedEdge {
        body,
        table,
        edge,
        name,
    } = *crossed;
    let Some(forward) = crossed_edge_orientation(body, table, edge, name)? else {
        return Ok(mint_candidates(t, tie, from_tie, base.clone(), keys)?);
    };
    let mut extents = Vec::with_capacity(crossings.len());
    for &(_, p) in crossings {
        let Some(along) = param_along(body, edge, p)? else {
            return Ok(mint_candidates(t, tie, from_tie, base.clone(), keys)?);
        };
        let along = if forward { along } else { -along };
        extents.push(Extent {
            min: along,
            max: along,
        });
    }
    insert_ranked_or_tied(t, tie, from_tie, base, &keys, &extents, bnd, |e| *e)
}

/// Inserts a same-name group ranked by order-along, or tied when
/// genuinely unordered.
#[allow(clippy::too_many_arguments)]
pub(super) fn insert_ranked_or_tied<T: Decide, K: Copy>(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    base: &StableName,
    keys: &[K],
    extents: &[Extent<T>],
    bnd: geom_core::Band,
    to_ent: impl Fn(&K) -> super::table::EntityRef,
) -> Result<(), NamingError> {
    match order_along(extents, bnd)? {
        Some(ranks) => {
            let of = group_count(keys.len())?;
            for (k, rank) in keys.iter().zip(ranks) {
                let mut name = base.clone();
                name.path
                    .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }));
                put(t, tie, from_tie, name, to_ent(k))?;
            }
        }
        None => mint_candidates(
            t,
            tie,
            from_tie,
            base.clone(),
            keys.iter().map(to_ent).collect(),
        )?,
    }
    Ok(())
}

/// Split faces: pass-through for uncut operand faces; `SplitFragment`
/// (side-discriminated, same-side multiplicity by `Keeps`) for cut
/// ones.
#[allow(clippy::too_many_arguments)]
fn name_split_faces<T: Decide>(
    node: RecipeNodeId,
    t: &mut NameTable,
    tie: &mut TieRows,
    rec: &mut GroupRecord,
    sides: &[Side<'_, T>],
    frag_rows: &BTreeMap<FaceKey, FaceKey>,
    section_keys: &BTreeSet<FaceKey>,
    target_node: RecipeNodeId,
    target_table: &NameTable,
    target_body: &Body<T>,
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
                // contributes no segment to survivors). It is still
                // its (face, side) group's one member, spelled by its
                // upstream name rather than the group's base.
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Face(f)))?;
                rec.record(
                    &split_base(node, s.half, up.name.clone()),
                    vec![ent(s.ix, EntityKey::Face(f))],
                    Parent::Elsewhere,
                );
                tie.carry(up, ent(s.ix, EntityKey::Face(f)))?;
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
        let base_name = split_base(node, members[0].1, parent.name);
        rec.record(
            &base_name,
            members
                .iter()
                .map(|&(ix, _, f)| ent(ix, EntityKey::Face(f)))
                .collect(),
            Parent::Elsewhere,
        );
        if members.len() == 1 {
            let (ix, _, f) = members[0];
            put(t, tie, from_tie, base_name, ent(ix, EntityKey::Face(f)))?;
            continue;
        }
        // Same-side multiplicity: each piece by the parent's boundary
        // edges it holds a stretch of (N2's `Keeps`), cited by their
        // operand names. A piece's edge is one of them when it descends
        // from one: whole, or as a piece the plane cut.
        let boundary: BTreeSet<EdgeKey> = face_half_edges(target_body, root)?
            .into_iter()
            .map(|he| {
                target_body
                    .get_half_edge(he)
                    .map(|h| h.edge)
                    .ok_or(NamingError::Emission {
                        what: "a split parent's half-edge is dangling",
                    })
            })
            .collect::<Result<_, _>>()?;
        let mut pieces = Vec::with_capacity(members.len());
        for &(ix, _, f) in &members {
            let body = sides
                .iter()
                .find(|s| s.ix == ix)
                .ok_or(NamingError::Emission {
                    what: "split fragment group without a side body",
                })?
                .body;
            let mut kept: BTreeSet<StableName> = BTreeSet::new();
            for he in face_half_edges(body, f)? {
                let e = body
                    .get_half_edge(he)
                    .ok_or(NamingError::Emission {
                        what: "a split piece's half-edge is dangling",
                    })?
                    .edge;
                let root_edge = chase_split_edge_to_table(sides, target_table, e)?;
                if boundary.contains(&root_edge) {
                    let up = upstream_name(
                        target_table,
                        target_node,
                        ent(0, EntityKey::Edge(root_edge)),
                    )?;
                    kept.insert((*up.name).clone());
                }
            }
            pieces.push((
                Qualifier::Keeps(kept.into_iter().collect()),
                ent(ix, EntityKey::Face(f)),
            ));
        }
        mint_qualified(t, tie, from_tie, &base_name, pieces)?;
    }
    Ok(())
}

/// The base of a split's (face, side) group: the face `parent` on the
/// `side` half. One spelling for the group the split divides and for
/// the face it passes through whole.
fn split_base(node: RecipeNodeId, side: SplitHalf, parent: NameRef) -> StableName {
    name1(
        EntityKind::Face,
        node,
        RoleSeg::SplitFragment { side, parent },
    )
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

    fn upstream(node: u64, tied: bool) -> Upstream {
        Upstream {
            name: NameRef::new(name1(
                EntityKind::Vertex,
                RecipeNodeId(node),
                RoleSeg::CapVertex(
                    super::super::role::CapEnd::End,
                    super::super::role::ProfileVertexRef::Piece {
                        step: crate::node::StepId(0),
                        role: crate::names::PieceRole::Leg,
                    },
                ),
            )),
            tied,
            candidate: if tied {
                super::super::table::Candidate::Of(0)
            } else {
                super::super::table::Candidate::Only
            },
        }
    }

    #[test]
    fn one_partner_takes_one_name_however_many_rows_carry_it() {
        let v = VertexKey::default();
        assert!(one_partner(v, Vec::new()).unwrap().is_none());
        let got = one_partner(v, vec![upstream(3, false), upstream(3, true)])
            .unwrap()
            .expect("two rows naming one partner decide it");
        assert_eq!(got.name, upstream(3, false).name);
        assert!(got.tied, "tie-descended if any row's partner is");
    }

    #[test]
    fn one_partner_refuses_two_distinct_partners() {
        let v = VertexKey::default();
        let Err(NamingError::SeamVertexPartners { vertex, candidates }) =
            one_partner(v, vec![upstream(3, false), upstream(4, false)])
        else {
            panic!("two distinct partners must refuse as SeamVertexPartners")
        };
        assert_eq!(vertex, v);
        assert_eq!(candidates.len(), 2);
    }

    #[test]
    fn a_ranked_group_count_past_u32_refuses() {
        assert_eq!(group_count(3).unwrap(), 3);
        if let Ok(too_many) = usize::try_from(u64::from(u32::MAX) + 1) {
            assert!(matches!(
                group_count(too_many),
                Err(NamingError::Emission { .. })
            ));
        }
    }

    /// A body holding one straight edge from `p` to `q`, and that edge.
    fn segment(p: [f64; 3], q: [f64; 3]) -> (Body<f64>, EdgeKey) {
        let mut body = Body::<f64>::new();
        let born = body
            .mvfs(Point3::new(p[0], p[1], p[2]), true)
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
    /// `BooleanNaming` they hand the emitter, and a row that merges
    /// faces hands it the body [`absorbed_into`] leaves.
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

    /// `body` with `absorbed` killed into `kept` across the edge they
    /// share: topologically what a merge of the two leaves, one face
    /// where there were two, though the faces are not coplanar.
    fn absorbed_into(body: &Body<f64>, kept: FaceKey, absorbed: FaceKey) -> Body<f64> {
        let mut out = body.clone();
        let he = face_half_edges(body, absorbed)
            .unwrap()
            .into_iter()
            .find(|&he| body.mate(he).and_then(|m| body.face_of_half_edge(m)) == Some(kept))
            .unwrap();
        assert_eq!(out.kef(he).unwrap().killed_face, absorbed);
        out
    }

    #[test]
    fn merged_lane_names_kept_face_with_sorted_deduped_constituents() {
        // A unit-cube extrusion: the "result body" stand-in.
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let a_table = name_extrude(
            ext_node,
            &built,
            &crate::eval::ProfilePieces::numbered(
                &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
            ),
        )
        .unwrap();

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
        let result = absorbed_into(&built.body, built.top, lateral);
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
        let t = name_boolean(bool_node, &result, &naming, &a, &b, Tol::witness())
            .unwrap()
            .table;

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
        // Both constituents retired into the merge (N3).
        assert!(
            cs.iter().all(|c| t.lookup(c).is_none()),
            "a constituent is published: {cs:?}"
        );
    }

    /// **A face a merge absorbed that the body still holds refuses.**
    /// The merge holds part of the parent, so its live face is a piece
    /// of a face held as several, and it borders no recorded discard:
    /// `Obstacles::split` refuses rather than mint `Borders([])`.
    #[test]
    fn an_absorbed_face_still_live_beside_its_merge_refuses() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let a_table = name_extrude(
            ext_node,
            &built,
            &crate::eval::ProfilePieces::numbered(
                &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
            ),
        )
        .unwrap();
        let lateral = a_table
            .iter()
            .find_map(|(n, e)| match (n.path.first(), e) {
                (Some(RoleSeg::Lateral(_)), Entry::Unique(r)) => match r.key {
                    EntityKey::Face(f) => Some(f),
                    _ => None,
                },
                _ => None,
            })
            .unwrap();
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            merge_groups: vec![(built.top, vec![lateral])],
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
        .expect_err("a live absorbed face beside its merge must refuse");
        assert!(
            matches!(
                err,
                NamingError::Emission {
                    what: "a piece of a face held as several borders no recorded discard between them"
                }
            ),
            "{err:?}"
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
        let a_table = name_extrude(
            ext_node,
            &built,
            &crate::eval::ProfilePieces::numbered(
                &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
            ),
        )
        .unwrap();
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

    /// **A B-lane lineage that leaves the graft rows refuses.** A
    /// grafted edge's records are forwarded into result keys, each
    /// with a graft row back to B (a live edge's, or a dead ancestor's
    /// `graft_dead_edges` row), so `chase_b` never reaches a key with no
    /// B preimage unless a record was not forwarded — which it refuses
    /// rather than reading as an A key or a root.
    ///
    /// The chain is honest: `split_edge` twice off one lateral gives
    /// `e2 → e1 → e0` as genuine birth records; the one graft row makes
    /// e2 a grafted edge whose parent e1 has no row.
    #[test]
    fn a_b_lineage_leaving_the_graft_rows_refuses() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let b_table = name_extrude(
            ext_node,
            &built,
            &crate::eval::ProfilePieces::numbered(
                &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
            ),
        )
        .unwrap();
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
        // The grafted layout — the only one `chase_b` serves. Result
        // edge e2 reads as B's e1, and its birth record names result
        // edge e1, which no graft row covers. A is the same named cube,
        // so every other key resolves on the A side and the walk
        // reaches e2.
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
            .expect_err("a lineage that leaves the graft rows must refuse");
        assert!(
            matches!(
                err,
                NamingError::Emission {
                    what: "a grafted edge's split lineage left the graft rows"
                }
            ),
            "the refusal is the B lane's own: {err:?}"
        );
    }

    /// Two merge groups listing the same operand faces — kept faces
    /// that are fragments of one operand face, each absorbing a
    /// fragment of one partner — hold one parent, and with no recorded
    /// discard between them nothing tells the two apart: the Borders
    /// rule refuses, never a silent alias.
    #[test]
    fn two_holders_of_one_merged_parent_with_nothing_between_them_refuse() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let a_table = name_extrude(
            ext_node,
            &built,
            &crate::eval::ProfilePieces::numbered(
                &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
            ),
        )
        .unwrap();
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
        .expect_err("two faces of one merged parent with no divider between them refuse");
        assert!(
            matches!(
                err,
                NamingError::Emission {
                    what: "a piece of a face held as several borders no recorded discard between them"
                }
            ),
            "the Borders rule reads the two merges as one parent: {err:?}"
        );
    }

    /// **Tied parents spelled alike stay two parents.** The operand's
    /// top and bottom are one tied name, and each is its own
    /// single-face merge: the two merges are spelled alike but list
    /// different entities, so each is a parent held as one face and the
    /// row is the tie of both, candidates in merge order.
    #[test]
    fn two_merges_of_tied_faces_publish_one_tied_merged_row() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let own = name_extrude(
            ext_node,
            &built,
            &crate::eval::ProfilePieces::numbered(
                &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
            ),
        )
        .unwrap();
        let caps = [built.top, built.bottom];
        let is_cap = |e: &Entry| matches!(e, Entry::Unique(r) if matches!(r.key, EntityKey::Face(f) if caps.contains(&f)));
        let tied_name = own
            .iter()
            .find(|(_, e)| is_cap(e))
            .map(|(n, _)| n.clone())
            .unwrap();
        let mut a_table = NameTable::new();
        for (name, entry) in own.iter().filter(|(_, e)| !is_cap(e)) {
            match entry {
                Entry::Unique(e) => a_table.insert(name.clone(), *e).unwrap(),
                Entry::Tied(es) => a_table.insert_tied(name.clone(), es.clone()).unwrap(),
            }
        }
        a_table
            .insert_tied(
                tied_name,
                caps.iter().map(|&f| ent(0, EntityKey::Face(f))).collect(),
            )
            .unwrap();
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            merge_groups: vec![(built.top, vec![]), (built.bottom, vec![])],
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
        let out = name_boolean(
            RecipeNodeId(9),
            &built.body,
            &naming,
            &a,
            &b,
            Tol::witness(),
        )
        .expect("two tied single-face merges name as one tied row");
        let merged: Vec<_> = out
            .table
            .iter()
            .filter(|(n, _)| matches!(n.path.first(), Some(RoleSeg::Merged(_))))
            .collect();
        assert_eq!(merged.len(), 1, "one merged row: {merged:?}");
        let (name, entry) = merged[0];
        assert_eq!(name.path.len(), 1, "the row carries no qualifier: {name:?}");
        let Entry::Tied(es) = entry else {
            panic!("the merged row is not the tie: {entry:?}");
        };
        assert_eq!(
            es.iter().map(|e| e.key).collect::<Vec<_>>(),
            caps.iter().map(|&f| EntityKey::Face(f)).collect::<Vec<_>>(),
            "the tie's candidates are the two caps, in merge order"
        );
    }

    /// The flat mint: an operand face that is itself a merged face
    /// contributes its CONSTITUENTS to a merge over it, each wrapped
    /// by the descent side, and never its `Merged` name.
    #[test]
    fn a_merge_over_a_merged_face_lists_its_constituents_flat() {
        let built = unit_cube();
        let ext_node = RecipeNodeId(1);
        let ext_table = name_extrude(
            ext_node,
            &built,
            &crate::eval::ProfilePieces::numbered(
                &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
            ),
        )
        .unwrap();
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
        let result = absorbed_into(&built.body, built.top, absorbed);
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
        let t = name_boolean(bool_node, &result, &naming, &a, &b, Tol::witness())
            .unwrap()
            .table;
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
        let ext_table = name_extrude(
            ext_node,
            &built,
            &crate::eval::ProfilePieces::numbered(
                &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
            ),
        )
        .unwrap();
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

#[cfg(test)]
mod split_carries_candidates {
    //! **A split's intact pass-through carries each tie candidate's
    //! number** (N4, "A tie's candidates keep their identity"), whether
    //! the tie survives the split `Tied` or narrows. Over the U-cutter
    //! subtract — a block less a U whose prongs leave two cap fragments
    //! under one tied name — every row the split carries verbatim must
    //! answer, for each entity, the candidate the subtract gave that
    //! entity. Renumbering at the split would pass whenever its fresh
    //! order happened to match; the two plane normals below swap which
    //! half is output body 0, so one of the two orders disagrees with
    //! any fresh numbering.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use crate::edit::DocEdit;
    use crate::eval::{CancelToken, EvalOptions, Evaluation, evaluate};
    use crate::ident::DocumentId;
    use crate::names::table::{EntityRef, Entry, NameTable};
    use crate::node::{BooleanOp, Datum, Node, RecipeNodeId};
    use crate::program::{LoopProgram, ProfileProgram};
    use crate::test_support::{frame, len, scl};
    use crate::{ProfileDoc, RefusingReach};
    use geom_core::Tol;

    fn ins(doc: ProfileDoc, node: Node<ProfileProgram>) -> (ProfileDoc, RecipeNodeId) {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode { node },
            Tol::witness(),
            &RefusingReach,
        )
        .expect("inserts");
        (a.doc, a.record.minted.expect("a node"))
    }

    fn prism(doc: ProfileDoc, z0: f64, dz: f64, pts: &[(f64, f64)]) -> (ProfileDoc, RecipeNodeId) {
        let (doc, plane) = ins(doc, frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        let (doc, profile) = ins(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![LoopProgram::polygon(pts.iter().copied()).expect("finite")],
                ids: Vec::new(),
            }),
        );
        ins(
            doc,
            Node::Extrude {
                profile,
                distance: len(dz),
            },
        )
    }

    /// The U-cutter subtract, split by the plane y = `y` with normal
    /// (0, `ny`, 0): the subtract's id and the split's.
    fn split_u_cutter(y: f64, ny: f64) -> (Evaluation<f64>, RecipeNodeId, RecipeNodeId) {
        let doc = ProfileDoc::empty(
            DocumentId::derive("split-carries-candidates"),
            Tol::witness(),
        );
        let (doc, a) = prism(
            doc,
            0.0,
            4.0,
            &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        );
        let (doc, b) = prism(
            doc,
            1.0,
            2.0,
            &[
                (2.0, 1.0),
                (6.0, 1.0),
                (6.0, 3.0),
                (2.0, 3.0),
                (2.0, 2.5),
                (5.0, 2.5),
                (5.0, 1.5),
                (2.0, 1.5),
            ],
        );
        let (doc, sub) = ins(
            doc,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a,
                b,
                declare: None,
            },
        );
        let (doc, tool) = ins(
            doc,
            Node::Datum(Datum::Plane {
                origin: [len(0.0), len(y), len(0.0)],
                normal: [scl(0.0), scl(ny), scl(0.0)],
            }),
        );
        let (doc, split) = ins(doc, Node::Split { target: sub, tool });
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        (ev, sub, split)
    }

    fn table(ev: &Evaluation<f64>, id: RecipeNodeId) -> &NameTable {
        &ev.value(id)
            .unwrap_or_else(|| panic!("node {id:?} evaluates: {:?}", ev.nodes.get(&id)))
            .name_table
    }

    /// Every tied row of the subtract the split carries verbatim, as
    /// (entry shape in the split, whether each candidate kept its
    /// number). Keys are kept through the split, so an entity of the
    /// split names the same key in the subtract's one body.
    fn carried_ties(y: f64, ny: f64) -> Vec<(bool, Vec<bool>)> {
        let (ev, sub, split) = split_u_cutter(y, ny);
        let (sub, split) = (table(&ev, sub), table(&ev, split));
        let mut out = Vec::new();
        for (name, entry) in sub.iter_refs() {
            if !matches!(entry, Entry::Tied(_)) {
                continue;
            }
            let Some(row) = split.rows().find(|(n, _)| *n == name).map(|(_, r)| r) else {
                continue;
            };
            let kept = row
                .pairs()
                .map(|(c, e)| {
                    let upstream = EntityRef {
                        body: 0,
                        key: e.key,
                    };
                    sub.candidate_of(name, &upstream) == Some(c)
                })
                .collect();
            let tied = matches!(split.entry_of(name), Some(Entry::Tied(_)));
            out.push((tied, kept));
        }
        out
    }

    #[test]
    fn a_split_carries_each_tie_candidates_number_tied_or_narrowed() {
        // Planes between the prongs separate the candidates, one per
        // half, and the split's own table keeps the tie `Tied` across
        // its two bodies; a plane clear of both keeps both in one half.
        for (what, y, ny) in [
            ("between the prongs, +y", 2.0, 1.0),
            ("between the prongs, -y", 2.0, -1.0),
            ("clear of both prongs, +y", 3.5, 1.0),
            ("clear of both prongs, -y", 3.5, -1.0),
        ] {
            let rows = carried_ties(y, ny);
            assert!(
                rows.iter().any(|(tied, kept)| *tied && kept.len() == 2),
                "{what}: the premise, a subtract tie the split keeps Tied: {rows:?}"
            );
            assert!(
                rows.iter().all(|(_, kept)| kept.iter().all(|k| *k)),
                "{what}: a carried candidate lost its number: {rows:?}"
            );
        }
    }
}

#[cfg(test)]
mod split_edge_lineage {
    //! **A split's edge chase crosses halves.** The clipped cylinder
    //! (`test_support::clipped_cylinder`) crosses its start rim arc
    //! twice: the arc's middle piece lies Above and its outer two
    //! Below, and one Below piece's `SplitEdge` record names the middle
    //! piece's key, which only the Above half holds.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{Side, chase_edge_to_table, chase_split_edge_to_table};
    use crate::eval::{CancelToken, DatumValue, EvalOptions, ValuePayload, evaluate};
    use crate::names::role::SplitHalf;
    use crate::names::table::{EntityKey, EntityRef};
    use crate::test_support::clipped_cylinder;
    use geom_core::Tol;
    use topo::{Provenance, SplitPlane};

    #[test]
    fn a_twice_crossed_rim_arcs_pieces_chase_to_the_rim_across_halves() {
        let (doc, [ext, tool, _]) = clipped_cylinder(Tol::witness());
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let value = ev.value(ext).expect("the extrude evaluates");
        let ValuePayload::Body(body) = &value.payload else {
            panic!("the extrude is one body");
        };
        // The plane the split verb reads off the same datum.
        let Some(ValuePayload::Datum(DatumValue::Plane { origin, normal })) =
            ev.value(tool).map(|v| &v.payload)
        else {
            panic!("the tool is a plane datum");
        };
        let table = &value.name_table;
        let out = topo::split(
            body,
            &SplitPlane {
                origin: *origin,
                normal: normal.get(),
            },
            Tol::witness(),
        )
        .expect("the plane splits the cylinder");
        let sides: Vec<Side<'_, f64>> = [
            (SplitHalf::Above, out.above.body()),
            (SplitHalf::Below, out.below.body()),
        ]
        .into_iter()
        .map(|(half, body)| Side {
            body: body.expect("material on both sides"),
            ix: half.output_body(),
            half,
        })
        .collect();
        let named = |k| {
            table
                .name_of(&EntityRef {
                    body: 0,
                    key: EntityKey::Edge(k),
                })
                .is_some()
        };
        let mut fresh = 0;
        let mut lost_within_its_half = 0;
        for s in &sides {
            for (e, _) in s.body.edges() {
                if named(e)
                    || !matches!(
                        s.body.edge_provenance_of(e),
                        Some(Provenance::SplitEdge { .. })
                    )
                {
                    continue;
                }
                fresh += 1;
                let root = chase_split_edge_to_table(&sides, table, e).expect("acyclic");
                assert!(
                    named(root),
                    "{:?} edge {e:?} chases to {root:?}, which the extrude never named",
                    s.half
                );
                let within = chase_edge_to_table(s.body, table, e).expect("acyclic");
                if !named(within) {
                    lost_within_its_half += 1;
                }
            }
        }
        assert!(fresh >= 2, "the premise, fresh split pieces: {fresh}");
        assert!(
            lost_within_its_half >= 1,
            "the premise, a piece whose lineage leaves its own half"
        );
    }
}

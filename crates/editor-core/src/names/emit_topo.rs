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
use super::discriminate::{
    CHORD_ON_RIM, Extent, ON_MEMBER_EDGE, ORDER_ALONG, band, extent_before, rank_by,
};
use super::emit::{
    Incidence, NamingError, Rim, RimShare, edge_ends, ent, face_half_edges, name1, rim_between,
    rims_between, vertex_point,
};
use super::groups::{CrossingSenses, Emitted, GroupRecord, Parent};
use super::merged::{self, NESTED_MERGED};
use super::role::{
    EntityKind, NameRef, Qualifier, RoleSeg, Sense, SplitHalf, StableName, edge_line,
};
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
    topo::lineage_root(f, rows.len(), |k| rows.get(&k).copied())
        .ok_or(NamingError::FragmentLineage { face: f })
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
        // The side's pieces of each operand edge, for the crossings'
        // senses.
        let mut pieces_of: BTreeMap<EdgeKey, Vec<EdgeKey>> = BTreeMap::new();
        // Remaining edges: pass-through or crossing-cut fragments.
        for (e, _) in body.edges() {
            if chord_faces.contains_key(&e) {
                continue;
            }
            let root = chase_split_edge_to_table(sides, target_table, e)?;
            pieces_of.entry(root).or_default().push(e);
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
        // Each crossing carries its sense against this half; a plane
        // crosses a straight edge at most once, and an arc it crosses
        // twice leaves crossings of both senses on each side. Several
        // of one sense on one line are ranked along it (N2).
        type OnLine<'b, T> = (bool, Vec<Crossed<'b, T>>);
        let mut by_line: BTreeMap<(NameRef, Sense), OnLine<'_, T>> = BTreeMap::new();
        for (crossed, (parent, verts)) in &crossings {
            let forward = oriented(target_body, target_table, *crossed, &parent.name)?;
            let pieces = pieces_of.get(crossed).map_or(&[][..], Vec::as_slice);
            let along = CrossedEdge {
                body: target_body,
                table: target_table,
                edge: *crossed,
                name: &parent.name,
            };
            for &v in verts {
                let sense = split_sense(body, v, pieces)?;
                let sense = if forward { sense } else { sense.flipped() };
                let on = by_line.entry((edge_line(&parent.name), sense)).or_default();
                on.0 |= parent.tied;
                on.1.push((
                    ent(s.ix, EntityKey::Vertex(v)),
                    vertex_point(body, v)?,
                    along,
                ));
            }
        }
        for ((line, sense), (tied, on)) in by_line {
            let base = name1(
                EntityKind::Vertex,
                node,
                RoleSeg::CrossingVertex {
                    side: s.half,
                    edge: line,
                    sense,
                },
            );
            rank_crossings(t, tie, tied, &base, &on, bnd)?;
        }
    }
    tie.flush(t)?;
    let mut pieces = EdgePieces::default();
    for ((slot, base), (from_tie, edges)) in edge_groups {
        let s = &sides[slot];
        // A lone section chord is the whole of the section line across
        // its face; an operand edge's fragment is always a piece of it.
        let lone = match base.path.first() {
            Some(RoleSeg::SectionEdge { .. }) => Lone::Whole,
            _ => Lone::Piece,
        };
        name_edge_pieces(
            &mut pieces,
            t,
            from_tie,
            &base,
            (s.body, s.ix),
            &edges,
            lone,
        )?;
    }
    pieces.mint(t, tie)
}

/// **The sense of a Split's crossing at `v`** (N2) against the half
/// whose body `body` holds it, along the crossed edge as it is stored:
/// `Enters` where the one piece of the edge the half holds at `v`
/// starts there, `Leaves` where it ends there. `pieces` are the half's
/// pieces of the crossed edge.
///
/// # Errors
///
/// [`NamingError::Emission`] unless exactly one piece meets `v`: a
/// crossing the half holds no piece at, or two, is not a crossing of
/// the edge into or out of the half.
fn split_sense<T: geom_core::Real>(
    body: &Body<T>,
    v: VertexKey,
    pieces: &[EdgeKey],
) -> Result<Sense, NamingError> {
    let mut senses = Vec::with_capacity(1);
    for &e in pieces {
        let (start, end) = edge_ends(body, e)?;
        if start == v {
            senses.push(Sense::Enters);
        }
        if end == v {
            senses.push(Sense::Leaves);
        }
    }
    match senses.as_slice() {
        [one] => Ok(*one),
        _ => Err(NamingError::Emission {
            what: "a split's crossing vertex does not hold exactly one piece of the edge it crosses",
        }),
    }
}

/// Whether the crossed edge `e` of `body`, named `name` in `table`, is
/// read as stored (`true`) or against it (`false`) when its crossings'
/// senses and ranks are read along it ([`crossed_edge_orientation`]).
///
/// # Errors
///
/// [`NamingError::Emission`] for a seam edge with no first side: no
/// orientation of it is a fact of the names, so neither is a sense.
fn oriented<T: geom_core::Real>(
    body: &Body<T>,
    table: &NameTable,
    e: EdgeKey,
    name: &StableName,
) -> Result<bool, NamingError> {
    crossed_edge_orientation(body, table, e, name)?
        .ok_or(NamingError::Emission { what: UNORIENTED })
}

/// A crossed seam edge with no first side: neither way along it is a
/// fact of the names, so neither is a crossing's sense.
const UNORIENTED: &str = "a crossed seam edge has no first side to orient its crossings by";

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
        &inv_vertices,
        &lineage_inv,
        &seam_set,
        &inc,
        &descend_face,
        &operand_face_name,
        &merged_descents,
        bnd,
    )?;
    let senses = name_boolean_vertices(
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
    let mut pieces = EdgePieces::default();
    for g in &edge_groups {
        name_edge_pieces(
            &mut pieces,
            &t,
            g.from_tie,
            &g.base,
            (body, 0),
            &g.edges,
            g.lone,
        )?;
    }
    pieces.mint(&mut t, &mut tie)?;
    tie.flush(&mut t)?;

    super::emit::check_total(&t, body, 0)?;
    Ok(Emitted::new(t, rec).with_senses(senses))
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
/// `FromA`/`FromB` for operand-descended ones, and `Merged` for an edge
/// the output stage joined across several operand edges
/// ([`joined_cover`]). A group's pieces are qualified once the vertices
/// are named ([`EdgeGroup`]).
#[allow(clippy::too_many_arguments)]
fn name_boolean_edges<T: Decide>(
    node: RecipeNodeId,
    rec: &mut GroupRecord,
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    inv_edges: &BTreeMap<EdgeKey, EdgeKey>,
    inv_vertices: &BTreeMap<VertexKey, VertexKey>,
    lineage_inv: &BTreeMap<EdgeKey, EdgeKey>,
    seam_set: &BTreeSet<EdgeKey>,
    inc: &Incidence,
    descend_face: &impl Fn(FaceKey) -> Result<OpSide<FaceKey>, NamingError>,
    operand_face_name: &impl Fn(OpSide<FaceKey>) -> Result<Upstream, NamingError>,
    merged_descents: &BTreeMap<FaceKey, Vec<OpSide<FaceKey>>>,
    bnd: geom_core::Band,
) -> Result<Vec<EdgeGroup>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let fused = Fused::of(naming);

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
    // ---- Edges the output stage's joins made, named for the operand
    // edges they lie along (`joined_cover`): read off this body and the
    // operands, since a join leaves no piece of the lineage the rows
    // below chase. ----
    let mut joined_one: BTreeMap<EdgeKey, OpSide<EdgeKey>> = BTreeMap::new();
    let mut joined_faces: BTreeSet<EdgeKey> = BTreeSet::new();
    let mut joined_sets: BTreeMap<Vec<OpSide<EdgeKey>>, Vec<EdgeKey>> = BTreeMap::new();
    let joined: BTreeSet<EdgeKey> = naming
        .edge_joins
        .iter()
        .map(|j| naming.joined_edge(j.kept))
        .filter(|&e| body.get_edge(e).is_some())
        .collect();
    for &e in &joined {
        match joined_cover(e, body, a, b, inc, descend_face, merged_descents, bnd)? {
            JoinedCover::Faces => {
                joined_faces.insert(e);
            }
            JoinedCover::One(r) => {
                joined_one.insert(e, r);
            }
            JoinedCover::Set(set) => joined_sets.entry(set).or_default().push(e),
        }
    }
    let joined_named =
        |e: &EdgeKey| joined_one.contains_key(e) || joined_sets.values().any(|es| es.contains(e));

    for &e in &naming.seam_edges {
        if body.get_edge(e).is_none() || joined_named(&e) {
            continue; // consumed by the merge stage, or named above
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
        if seam_set.contains(&e) || joined_named(&e) {
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
        // edge, named by its adjacent faces' descent like any seam; so
        // is an edge a join made that its operand edges do not cover
        // (`joined_cover`).
        let (op, k) = root.of(a, b);
        let resolves =
            op.table.name_of(&ent(0, EntityKey::Edge(k))).is_some() && !joined_faces.contains(&e);
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
    for (e, r) in joined_one {
        groups.entry(r).or_default().push(e);
    }
    let in_sets: BTreeSet<OpSide<EdgeKey>> = joined_sets.keys().flatten().copied().collect();
    let mut out = Vec::with_capacity(seam_groups.len() + groups.len() + joined_sets.len());
    for (set, edges) in joined_sets {
        let (base, from_tie) = set_name(node, &set, a, b)?;
        rec.record_by_name(
            &base,
            edges.iter().map(|&e| ent(0, EntityKey::Edge(e))).collect(),
            from_tie,
        );
        out.push(EdgeGroup {
            base,
            from_tie,
            edges,
            set,
            lone: Lone::Whole,
        });
    }
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
            set: Vec::new(),
            lone: Lone::Whole,
        });
    }
    for (root, edges) in groups {
        let (op, root_key) = root.of(a, b);
        let inner = upstream_name(op.table, op.node, ent(0, EntityKey::Edge(root_key)))?;
        let base = name1(EntityKind::Edge, node, root.wrap(inner.name));
        // Undivided: the one edge runs between the operand edge's own two
        // ends, as the operand's keys read the result's vertices there.
        let whole = match edges.as_slice() {
            [one] => {
                let (r0, r1) = edge_ends(op.body, root_key)?;
                let (e0, e1) = edge_ends(body, *one)?;
                let at = |v| operand_vertex_keys(naming, inv_vertices, &fused, v);
                let (k0, k1) = (at(e0)?, at(e1)?);
                let side = root.operand();
                (k0.contains(&(side, r0)) && k1.contains(&(side, r1)))
                    || (k0.contains(&(side, r1)) && k1.contains(&(side, r0)))
            }
            _ => false,
        };
        let lone = if in_sets.contains(&root) || !whole {
            Lone::Piece
        } else {
            Lone::Whole
        };
        // The group is the line's: every edge on it the node holds from
        // an operand edge, whichever parent on the line it descends
        // from. Tied parents share one name and so one line, and stay
        // each its own group.
        let line = edge_line(&NameRef::new(base.clone()));
        let members = edges.iter().map(|&e| ent(0, EntityKey::Edge(e))).collect();
        let parent = root.map(EntityKey::Edge).parent();
        if inner.tied {
            rec.record(&line, members, parent);
        } else {
            rec.record_on_line(&line, members, parent);
        }
        out.push(EdgeGroup {
            base,
            from_tie: inner.tied,
            edges,
            set: Vec::new(),
            lone,
        });
    }
    Ok(out)
}

/// What an edge the output stage's joins made lies along.
enum JoinedCover {
    /// Along no operand edges that cover it: named from its two faces,
    /// as a seam is ([`ChordKind`]).
    Faces,
    /// Within one operand edge, a piece of it.
    One(OpSide<EdgeKey>),
    /// Along several operand edges that together cover it and none of
    /// which holds it whole: named for the set.
    Set(Vec<OpSide<EdgeKey>>),
}

/// **The operand edges joined edge `e` lies along** (`names/README.md`,
/// "Flush edges"): the edges of either operand between two faces that
/// `e`'s two faces descend from (a merged face from each of its
/// constituents), straight where `e` is and curved where `e` is, that
/// lie on `e`'s carrier and overlap it over a length, read through
/// [`ON_MEMBER_EDGE`] along `e`'s line ([`Segment::cover`]) or its
/// curved carrier ([`Track::cover`]). One that holds `e` whole makes
/// `e` a piece of it, the least such in key order with A's before B's;
/// otherwise several that overlap it and together cover it name it as a
/// set, since a set name says the edge lies on its constituents.
/// Anything else — one that runs along a seam for part of its length —
/// is named from its two faces, never from the lineage of the key the
/// join kept, which holds only part of it. A closed `e`, two pieces the
/// join made one, lies within a closed rim only and is covered over its
/// whole period, so where its vertex sits says nothing.
#[allow(clippy::too_many_arguments)]
fn joined_cover<T: Decide>(
    e: EdgeKey,
    body: &Body<T>,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    inc: &Incidence,
    descend_face: &impl Fn(FaceKey) -> Result<OpSide<FaceKey>, NamingError>,
    merged_descents: &BTreeMap<FaceKey, Vec<OpSide<FaceKey>>>,
    bnd: geom_core::Band,
) -> Result<JoinedCover, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let Some([f0, f1]) = inc.edge_faces.get(&e).map(Vec::as_slice) else {
        return Err(bug("a joined edge without exactly two adjacent faces"));
    };
    let descents = |f: FaceKey| -> Result<Vec<OpSide<FaceKey>>, NamingError> {
        Ok(match merged_descents.get(&f) {
            Some(ds) => ds.clone(),
            None => vec![descend_face(f)?],
        })
    };
    let (d0, d1) = (descents(*f0)?, descents(*f1)?);
    let straight = |body: &Body<T>, k: EdgeKey| {
        topo::query::edge_carrier_kind(body, k) == Some(topo::query::CurveKind::Line)
    };
    let line = straight(body, e);
    let mut candidates: BTreeSet<OpSide<EdgeKey>> = BTreeSet::new();
    for &g0 in &d0 {
        for &g1 in d1
            .iter()
            .filter(|g1| g1.operand() == g0.operand() && **g1 != g0)
        {
            let (op, k0) = g0.of(a, b);
            let (_, k1) = g1.of(a, b);
            for r in rims_between(op.body, k0, k1)? {
                if straight(op.body, r) == line {
                    candidates.insert(g0.with(r));
                }
            }
        }
    }
    let mut arcs = Vec::with_capacity(candidates.len());
    for r in candidates {
        let (op, k) = r.of(a, b);
        let (v0, v1) = edge_ends(op.body, k)?;
        let (p0, p1) = (vertex_point(op.body, v0)?, vertex_point(op.body, v1)?);
        // A segment is read by its ends alone.
        let mid = if line {
            p0
        } else {
            op.body
                .get_edge(k)
                .and_then(|d| op.body.get_curve_geom(d.curve))
                .and_then(topo::CurveGeom::certified)
                .map(|c| {
                    let (r0, r1) = c.params();
                    c.carrier().mid_point(r0, r1)
                })
                .ok_or(bug("a joined edge's cover has no certified curve"))?
        };
        arcs.push((
            r,
            CarrierArc {
                p0,
                mid,
                p1,
                closed: v0 == v1,
            },
        ));
    }
    let cover = if line {
        Segment::of_edge(body, e)?.cover(
            arcs.into_iter().map(|(r, arc)| (r, arc.p0, arc.p1)),
            ON_MEMBER_EDGE,
            bnd,
        )?
    } else {
        Track::of_edge(body, e)?.cover(arcs, ON_MEMBER_EDGE, bnd)?
    };
    if let Some(&r) = cover.within.first() {
        return Ok(JoinedCover::One(r));
    }
    Ok(if cover.covered {
        JoinedCover::Set(cover.along)
    } else {
        JoinedCover::Faces
    })
}

/// The name of a joined edge that lies along the operand edges `set`:
/// `Merged` of their names, each wrapped by its side, and flat — an
/// operand edge that is itself a set stands for its constituents (N3).
/// Whether any of them is tied comes with it.
fn set_name<T: Decide>(
    node: RecipeNodeId,
    set: &[OpSide<EdgeKey>],
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
) -> Result<(StableName, bool), NamingError> {
    let mut names = Vec::with_capacity(set.len());
    let mut from_tie = false;
    for &r in set {
        let (op, k) = r.of(a, b);
        let up = upstream_name(op.table, op.node, ent(0, EntityKey::Edge(k)))?;
        from_tie |= up.tied;
        names.push(name1(EntityKind::Edge, node, r.wrap(up.name)));
    }
    Ok((merged::edge_set(node, names), from_tie))
}

/// The edges of a pair boolean's result that share one parent: its
/// name, whether that descends from a tie, and the edges. Their
/// vertices are named from `base` (a vertex cites an edge by its head),
/// and the edges then by their ends ([`name_edge_pieces`]).
struct EdgeGroup {
    base: StableName,
    from_tie: bool,
    edges: Vec<EdgeKey>,
    /// The operand edges a joined edge's set name lists, or empty.
    set: Vec<OpSide<EdgeKey>>,
    /// What a lone piece of the parent is named for.
    lone: Lone,
}

/// Boolean vertices: operand pass-downs (`FromA`/`FromB`), and seam
/// (crossing/fused) vertices named by the operand entities whose
/// crossing minted them — derived from the already-named incident edges
/// (combinatorial wiring facts) — and, where an edge crosses, by its
/// sense ([`senses_at`]): `Crossing`, `EdgeCrossing`, or `Seam` for
/// every other meeting. Returns the senses read at each seam vertex, for
/// a union's fold ([`CrossingSenses`]).
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
) -> Result<CrossingSenses, NamingError> {
    let bug = |what| NamingError::Emission { what };
    // Each edge's parent, the head a vertex cites it by, and whether
    // that descends from a tie.
    let edge_base: BTreeMap<EdgeKey, (&StableName, bool, &[OpSide<EdgeKey>])> = edge_groups
        .iter()
        .flat_map(|g| {
            g.edges
                .iter()
                .map(move |&e| (e, (&g.base, g.from_tie, g.set.as_slice())))
        })
        .collect();
    // The operand edges of a joined edge's set that hold vertex `v`:
    // A's where any does, else B's. A vertex cites an edge it lies on,
    // never the whole set (N2).
    let set_holding =
        |v: VertexKey, set: &[OpSide<EdgeKey>]| -> Result<Vec<OpSide<NameRef>>, NamingError> {
            let p = vertex_point(body, v)?;
            let mut held = Vec::new();
            for &r in set {
                let (op, k) = r.of(a, b);
                if Segment::of_edge(op.body, k)?.place(p, ON_MEMBER_EDGE, bnd)? != OnSegment::Off {
                    let up = upstream_name(op.table, op.node, ent(0, EntityKey::Edge(k)))?;
                    held.push(r.with(up.name));
                }
            }
            if held.iter().any(|r| matches!(r, OpSide::A(_))) {
                held.retain(|r| matches!(r, OpSide::A(_)));
            }
            Ok(held)
        };
    // A-side weld and zip fusions (`vertex_merges`; a B-side weld kills
    // a minted pierce vertex before the graft, which has no name to
    // owe): kept key → dead partners (a fused vertex may owe
    // its operand identity to a DEAD partner's key — e.g. a B corner
    // vertex fused into an A-side crossing key on a shared plane).
    let fused = Fused::of(naming);
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
    let mut senses = CrossingSenses::new();
    let mut groups: BTreeMap<RoleSeg, (bool, Vec<(VertexKey, Along)>)> = BTreeMap::new();
    for (v, _) in body.vertices() {
        // Operand pass-downs: the kept key itself, then its dead
        // fusion partners nearest first (`fused_partners`). The KEPT
        // key's identity wins when both operands fused here, and
        // along a chain of fusions the key nearest the survivor does:
        // `operand_identity` checks the graft destination first, and
        // the kept key is A's exactly because `zip_seam` keeps the
        // outer cycle's vertex.
        let mut identity = operand_identity(v)?;
        if identity.is_none() {
            for &dead in fused.partners.get(&v).into_iter().flatten() {
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
        // What the boolean classified here: the senses a crossing is
        // named by, and the evidence a union reads for a vertex its fold
        // names some other way (`CrossingSenses`).
        let at_v = senses_at(naming, inv_vertices, &fused, v, a, b)?;
        let crossed: Vec<(StableName, Sense)> = at_v
            .iter()
            .filter(|s| !s.unoriented)
            .filter_map(|s| s.sense.map(|sense| (s.name.clone(), sense)))
            .collect();
        if !crossed.is_empty() {
            senses.insert(v, crossed);
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
            let Some(&(ename, tied, set)) = edge_base.get(&e) else {
                return Err(bug("seam vertex incident to an unnamed edge"));
            };
            from_tie |= tied;
            match ename.path.first() {
                Some(RoleSeg::Merged(_)) => {
                    let held = set_holding(v, set)?;
                    if held.is_empty() {
                        return Err(bug("a vertex of a joined edge lies on none of its set"));
                    }
                    for r in held {
                        match r {
                            OpSide::A(x) => a_edges.push(x),
                            OpSide::B(x) => b_edges.push(x),
                        }
                    }
                }
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
            // meet once). A pinch is one too: several edges of one
            // operand pierce a face of the other at one vertex, so no
            // single edge is its parent; and so is a point edges of both
            // operands run through, one of them on one side, where no
            // edge pair is.
            (aes, bes, _, _)
                if seam_lines.len() >= 2
                    && match (aes.len(), bes.len()) {
                        (0, 0) => true,
                        (n, 0) | (0, n) => n >= 2,
                        (1, _) | (_, 1) => true,
                        _ => false,
                    } =>
            {
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
            // Written to a witness: an ordinary declared union reached
            // exactly this shape — one operand-descended edge on the A
            // side, none on the B side, and, everything above having
            // already run, no single B face and no contact-record partner
            // to supply the other parent. The vertex is half-decided, the
            // body is sound and the recipe is legal, so what is missing is
            // a rule. Maximal edges join that witness's vertex away, and
            // no document reaches the arm now
            // (`work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule.md`).
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
            // from the contact-record partner on either side, and on a
            // straight edge the output stage joins it away. Two kinds of
            // row hold the sides: `swapping_the_operands_swaps_the_sides_of_every_name`
            // (in `emit_boolean_vertex_keys`) holds the two sides
            // SYMMETRIC — the same geometry named alike with A and B
            // exchanged — which a consistent A/B relabel would pass; the
            // absolute rows beside it (the nested corners, the touched
            // reflex edge, the assembly's touched ridge) pin WHICH side
            // each name belongs to. A shape nobody has reached is not a
            // shape known to be legal, so the mirror stays in the residue
            // below.
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
        // The vertex's head: an edge crossing a face is a crossing with
        // the edge's sense, two edges crossing a crossing with both
        // senses, and anything else — a touch included, where an edge
        // neither enters nor leaves the other operand — a seam of its two
        // parents.
        let (pa, pb) = pair;
        let sense = |side, edge: &NameRef| sense_of(&at_v, side, edge);
        let senses = match (pa.kind, pb.kind) {
            (EntityKind::Edge, EntityKind::Face) => (sense(topo::Operand::A, &pa)?, None),
            (EntityKind::Face, EntityKind::Edge) => (None, sense(topo::Operand::B, &pb)?),
            (EntityKind::Edge, EntityKind::Edge) => {
                (sense(topo::Operand::A, &pa)?, sense(topo::Operand::B, &pb)?)
            }
            _ => (None, None),
        };
        let (seg, along) = match (pa.kind, pb.kind, senses) {
            (EntityKind::Edge, EntityKind::Face, (Some(sense), None)) => (
                RoleSeg::Crossing {
                    edge: edge_line(&pa),
                    face: pb,
                    sense,
                },
                Along::A(pa),
            ),
            (EntityKind::Face, EntityKind::Edge, (None, Some(sense))) => (
                RoleSeg::Crossing {
                    edge: edge_line(&pb),
                    face: pa,
                    sense,
                },
                Along::B(pb),
            ),
            (EntityKind::Edge, EntityKind::Edge, (Some(a_sense), Some(b_sense))) => (
                RoleSeg::EdgeCrossing {
                    a: edge_line(&pa),
                    a_sense,
                    b: edge_line(&pb),
                    b_sense,
                },
                Along::A(pa),
            ),
            _ => (
                RoleSeg::Seam {
                    a: edge_line(&pa),
                    b: edge_line(&pb),
                },
                Along::Either(pa, pb),
            ),
        };
        // Pieces of one line crossed alike share a name, and rank along
        // the line, each read on its own piece.
        let slot = groups.entry(seg).or_insert((false, Vec::new()));
        slot.0 |= from_tie;
        slot.1.push((v, along));
    }
    for (seg, (from_tie, verts)) in groups {
        let base = name1(EntityKind::Vertex, node, seg);
        rec.record_by_name(
            &base,
            verts
                .iter()
                .map(|&(v, _)| ent(0, EntityKey::Vertex(v)))
                .collect(),
            from_tie,
        );
        if let [(v, _)] = verts.as_slice() {
            put(t, tie, from_tie, base, ent(0, EntityKey::Vertex(*v)))?;
            continue;
        }
        // Several crossings with one name: ranked along the crossed
        // line (the A side's where both are edges), or tied where one
        // lies on no edge either table names.
        let mut crossings = Vec::with_capacity(verts.len());
        for (v, along) in &verts {
            let on = match along {
                Along::A(e) => crossed_edge(e, a.table).map(|k| (a, k, e)),
                Along::B(e) => crossed_edge(e, b.table).map(|k| (b, k, e)),
                Along::Either(pa, pb) => match crossed_edge(pa, a.table) {
                    Some(k) => Some((a, k, pa)),
                    None => crossed_edge(pb, b.table).map(|k| (b, k, pb)),
                },
            };
            let Some((op, k, parent)) = on else {
                crossings.clear();
                break;
            };
            crossings.push((
                ent(0, EntityKey::Vertex(*v)),
                vertex_point(body, *v)?,
                CrossedEdge {
                    body: op.body,
                    table: op.table,
                    edge: k,
                    name: parent,
                },
            ));
        }
        if crossings.is_empty() {
            let ents = verts
                .iter()
                .map(|&(v, _)| ent(0, EntityKey::Vertex(v)))
                .collect();
            mint_candidates(t, tie, from_tie, base, ents)?;
            continue;
        }
        rank_crossings(t, tie, from_tie, &base, &crossings, bnd)?;
    }
    Ok(senses)
}

/// The edge a seam vertex is ranked along: an A-side edge, a B-side
/// one, or the first of a seam's two parents that is an edge.
enum Along {
    A(NameRef),
    B(NameRef),
    Either(NameRef, NameRef),
}

/// What a boolean fused, read once per boolean: each result vertex's
/// fusion partners ([`fused_partners`]) and every pair of operand keys
/// a B-side weld (`BooleanNaming::weld_merges_b`) or a null edge
/// (`BooleanNaming::null_copies`) leaves at one point, both ways round.
struct Fused {
    partners: BTreeMap<VertexKey, Vec<VertexKey>>,
    one_point: Vec<((topo::Operand, VertexKey), (topo::Operand, VertexKey))>,
}

impl Fused {
    fn of(naming: &topo::BooleanNaming) -> Self {
        Self::from_rows(
            naming.vertex_merges.rows(),
            naming.weld_merges_b.rows(),
            &naming.null_copies,
        )
    }

    /// From the zips' and A-side welds' fusions `(dead, kept)`, the
    /// B-side welds' and the null copies, each in the order made.
    fn from_rows(
        merges: &[(VertexKey, VertexKey)],
        welds_b: &[(VertexKey, VertexKey)],
        null_copies: &[(topo::Operand, VertexKey, VertexKey)],
    ) -> Self {
        let one_point = welds_b
            .iter()
            .map(|&(dead, kept)| (topo::Operand::B, dead, kept))
            .chain(null_copies.iter().copied())
            .flat_map(|(side, x, y)| [((side, x), (side, y)), ((side, y), (side, x))])
            .collect();
        Self {
            partners: fused_partners(merges),
            one_point,
        }
    }
}

/// Each vertex the fusions `merges` (`(dead, kept)`, in the order
/// made) leave alive → every dead vertex fused into it through any
/// number of them, nearest first: the keys fused straight into it, then
/// those fused into one of them, each hop in the order they died. A
/// vertex's operand identity is the first of them its operand names, so
/// a key one fusion from the survivor wins over a key it absorbed
/// earlier, as when no chain formed.
fn fused_partners(merges: &[(VertexKey, VertexKey)]) -> BTreeMap<VertexKey, Vec<VertexKey>> {
    let mut into: BTreeMap<VertexKey, Vec<VertexKey>> = BTreeMap::new();
    for &(dead, kept) in merges {
        into.entry(kept).or_default().push(dead);
    }
    let dead: BTreeSet<VertexKey> = merges.iter().map(|&(d, _)| d).collect();
    into.keys()
        .filter(|k| !dead.contains(k))
        .map(|&survivor| {
            let mut order = Vec::new();
            let mut hop = vec![survivor];
            while !hop.is_empty() {
                hop = hop
                    .iter()
                    .flat_map(|k| into.get(k).into_iter().flatten().copied())
                    .collect();
                order.extend(&hop);
            }
            (survivor, order)
        })
        .collect()
}

/// **Every operand vertex result vertex `v` is**, `(operand, key)` in
/// that operand's clone keys: its own key read through the layout
/// ([`operand_key`]), each key fused into it, and, transitively, every
/// key a B-side weld (`BooleanNaming::weld_merges_b`) or a null edge
/// (`BooleanNaming::null_copies`) joins to any of those: either leaves
/// two keys of one point.
fn operand_vertex_keys(
    naming: &topo::BooleanNaming,
    inv_vertices: &BTreeMap<VertexKey, VertexKey>,
    fused: &Fused,
    v: VertexKey,
) -> Result<BTreeSet<(topo::Operand, VertexKey)>, NamingError> {
    let mut keys = BTreeSet::new();
    for &k in core::iter::once(&v).chain(fused.partners.get(&v).into_iter().flatten()) {
        keys.insert(operand_key(naming, inv_vertices, k)?.0.of_operand());
    }
    loop {
        let new: Vec<_> = fused
            .one_point
            .iter()
            .filter(|(from, to)| keys.contains(from) && !keys.contains(to))
            .map(|&(_, to)| to)
            .collect();
        if new.is_empty() {
            return Ok(keys);
        }
        keys.extend(new);
    }
}

/// One operand edge with a piece the boolean classified at a result
/// vertex ([`senses_at`]).
struct EdgeSense {
    side: topo::Operand,
    name: StableName,
    /// `None` where the edge does not cross there.
    sense: Option<Sense>,
    /// The edge is a seam with no first side to read it along, so a
    /// sense it would carry is no fact of the names ([`oriented`]).
    unoriented: bool,
}

/// **The senses at result vertex `v`** (N2): each operand edge with a
/// piece the kernel classified beside the vertex
/// ([`topo::EdgePieceClass`]), with its sense against the other
/// operand's closed body along the edge as [`crossed_edge_orientation`]
/// reads it, and `None` where it does not cross there — both sides of
/// the vertex lie in the closed body, or both outside it.
///
/// The vertex is read at its own key and at every key fused into it,
/// each in its operand's clone. A piece `In` or `On` lies in the closed
/// body. A side of the vertex with no piece counts as outside it where
/// the vertex is the operand edge's own end on that side, so no portion
/// of the edge lies there.
///
/// # Errors
///
/// [`NamingError::Emission`] where one side of the vertex holds pieces
/// of one edge on both sides of the other operand, or where a side with
/// edge beyond the vertex has no classified piece: a row the boolean
/// owed is missing, not a fact of the edge.
fn senses_at<T: Decide>(
    naming: &topo::BooleanNaming,
    inv_vertices: &BTreeMap<VertexKey, VertexKey>,
    fused: &Fused,
    v: VertexKey,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
) -> Result<Vec<EdgeSense>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let keys = operand_vertex_keys(naming, inv_vertices, fused, v)?;
    // (side, edge) → (the sides of the vertex in the closed body,
    // before it and after it).
    let mut sides: BTreeMap<(topo::Operand, EdgeKey), [BTreeSet<bool>; 2]> = BTreeMap::new();
    for row in &naming.edge_classes {
        if keys.contains(&(row.operand, row.vertex)) {
            sides.entry((row.operand, row.edge)).or_default()[usize::from(row.starts)]
                .insert(row.class != topo::SideCode::Out);
        }
    }
    let mut out = Vec::with_capacity(sides.len());
    for ((side, edge), [before, after]) in sides {
        let op = match side {
            topo::Operand::A => a,
            topo::Operand::B => b,
        };
        // A side with no piece lies past the edge's own end there.
        let (start, end) = edge_ends(op.body, edge)?;
        let one = |set: &BTreeSet<bool>, end_here: VertexKey| {
            side_in_body(set, keys.contains(&(side, end_here)))
        };
        let name = op
            .table
            .name_of(&ent(0, EntityKey::Edge(edge)))
            .ok_or(bug("a classified operand edge is not named"))?
            .clone();
        let sense = match (one(&before, start)?, one(&after, end)?) {
            (false, true) => Some(Sense::Enters),
            (true, false) => Some(Sense::Leaves),
            _ => None,
        };
        let forward = crossed_edge_orientation(op.body, op.table, edge, &name)?;
        let sense = match forward {
            Some(false) => sense.map(Sense::flipped),
            _ => sense,
        };
        out.push(EdgeSense {
            side,
            name,
            sense,
            unoriented: forward.is_none(),
        });
    }
    Ok(out)
}

/// **Whether one side of a vertex lies in the other operand's closed
/// body** along a crossed edge, from the classes of the edge's pieces
/// there (`true` for `In` or `On`): the one class read, or outside where
/// no piece lies there because the edge ends at the vertex
/// (`ends_here`).
///
/// # Errors
///
/// [`NamingError::Emission`] where the edge runs on past the vertex but
/// no piece of it there was classified — a row the boolean owed, not a
/// fact of the edge — or where its pieces there read both ways.
fn side_in_body(classes: &BTreeSet<bool>, ends_here: bool) -> Result<bool, NamingError> {
    let bug = |what| NamingError::Emission { what };
    match classes.len() {
        0 if ends_here => Ok(false),
        0 => Err(bug(UNCLASSIFIED_SIDE)),
        1 => Ok(classes.contains(&true)),
        _ => Err(bug(
            "a crossing's edge has pieces on both sides of the other operand on one side of it",
        )),
    }
}

/// A crossed edge runs on past the crossing with no piece classified.
const UNCLASSIFIED_SIDE: &str =
    "a crossing's edge runs on past the crossing on a side the boolean classified no piece of";

#[cfg(test)]
mod side_tests {
    #![allow(clippy::unwrap_used)]

    use super::{NamingError, UNCLASSIFIED_SIDE, side_in_body};

    /// **A side of a crossing with no classified piece is outside only
    /// where the edge ends there** (N2): past its own end no portion of
    /// the edge lies on that side; where it runs on, a missing class is a
    /// row the boolean owed, and refuses.
    #[test]
    fn a_side_with_no_classified_piece_is_outside_only_past_the_edges_end() {
        use std::collections::BTreeSet;

        let none = BTreeSet::new();
        assert!(!side_in_body(&none, true).unwrap());
        assert!(matches!(
            side_in_body(&none, false),
            Err(NamingError::Emission { what }) if what == UNCLASSIFIED_SIDE
        ));
        assert!(side_in_body(&BTreeSet::from([true]), false).unwrap());
        assert!(!side_in_body(&BTreeSet::from([false]), true).unwrap());
        assert!(side_in_body(&BTreeSet::from([true, false]), false).is_err());
    }
}

#[cfg(test)]
mod fused_tests {
    #![allow(clippy::unwrap_used)]

    use super::{BTreeMap, Fused, VertexKey, fused_partners, operand_vertex_keys};
    use topo::Operand::B;

    fn vk(i: u64) -> VertexKey {
        VertexKey::from(slotmap::KeyData::from_ffi((1 << 32) | i))
    }

    /// **A survivor's partners are every key fused into it, nearest
    /// first** (`fused_partners`): `b` died into `a1` before `a1` died
    /// into `a2`, so `b` reaches `a2` only through a chain, and comes
    /// after both keys `a2` absorbed directly, which keep the order
    /// they died in.
    #[test]
    fn a_chain_of_fusions_reaches_the_survivor_nearest_first() {
        let (b, c, a1, a2) = (vk(1), vk(2), vk(3), vk(4));
        let partners = fused_partners(&[(b, a1), (c, a2), (a1, a2)]);
        assert_eq!(
            partners,
            BTreeMap::from([(a2, vec![c, a1, b])]),
            "the survivor's partners, nearest first"
        );
    }

    /// The operand keys of result vertex `r`, the graft of B's `kb`,
    /// under `fused`.
    fn b_keys_of(fused: &Fused, r: VertexKey, kb: VertexKey) -> Vec<VertexKey> {
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Grafted,
            ..topo::BooleanNaming::default()
        };
        let inv = BTreeMap::from([(r, kb)]);
        operand_vertex_keys(&naming, &inv, fused, r)
            .unwrap()
            .into_iter()
            .map(|(side, k)| {
                assert_eq!(side, B, "a B vertex's keys are B's");
                k
            })
            .collect()
    }

    /// **A vertex is every B key a chain of B-side welds fused into its
    /// own**: `w0` welded into `w1`, then `w1` into the grafted `kb`.
    #[test]
    fn a_chain_of_b_welds_reaches_the_grafted_key() {
        let (r, w0, w1, kb) = (vk(10), vk(1), vk(2), vk(3));
        let fused = Fused::from_rows(&[], &[(w0, w1), (w1, kb)], &[]);
        let mut want = vec![w0, w1, kb];
        want.sort_unstable();
        assert_eq!(b_keys_of(&fused, r, kb), want, "the welded keys");
    }

    /// **A weld into a null copy of the vertex is the vertex too**: the
    /// null edge leaves `kb` a copy `kc`, and `w` was welded into `kc`.
    #[test]
    fn a_weld_into_a_null_copy_is_the_vertex() {
        let (r, kb, kc, w) = (vk(10), vk(1), vk(2), vk(3));
        let fused = Fused::from_rows(&[], &[(w, kc)], &[(B, kb, kc)]);
        let mut want = vec![kb, kc, w];
        want.sort_unstable();
        assert_eq!(b_keys_of(&fused, r, kb), want, "the copy and its weld");
    }
}

/// **The sense at a vertex of `side`'s edge `edge`**, among the
/// vertex's [`senses_at`]: `None` where it does not cross there.
///
/// # Errors
///
/// [`NamingError::Emission`] where no piece of the edge is classified at
/// the vertex, where edges sharing its name read different senses, or
/// where the edge crosses but has no orientation to read its sense
/// along ([`oriented`]).
fn sense_of(
    senses: &[EdgeSense],
    side: topo::Operand,
    edge: &StableName,
) -> Result<Option<Sense>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let mut read = senses.iter().filter(|s| s.side == side && s.name == *edge);
    let first = read.next().ok_or(bug(
        "a crossing's edge has no piece the boolean classified at the crossing",
    ))?;
    if read.any(|s| s.sense != first.sense) {
        return Err(bug(
            "edges of one name read different senses at one crossing",
        ));
    }
    if first.unoriented && first.sense.is_some() {
        return Err(bug(UNORIENTED));
    }
    Ok(first.sense)
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

    /// Whether segment `other` runs the same way as this one along
    /// this one's line (`Some(true)`), the other way (`Some(false)`), or
    /// does not lie on it (`None`): its two ends on the line, then the
    /// sign of the two directions' dot, each through `predicate`.
    pub(super) fn runs_with(
        &self,
        other: &Self,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Option<bool>, NamingError> {
        if !(self.on_line(other.q0, predicate, bnd)? && self.on_line(other.q1, predicate, bnd)?) {
            return Ok(None);
        }
        let way = decide(
            predicate,
            Margin::over_lever(self.d.dot(other.d), self.len),
            bnd,
        )
        .map_err(|source| NamingError::Escalated { predicate, source })?;
        match way {
            Sign::Positive => Ok(Some(true)),
            Sign::Negative => Ok(Some(false)),
            Sign::Zero => Err(NamingError::Emission {
                what: "two segments on one line run neither way along it",
            }),
        }
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

    /// The sign of `p − q` along the segment's direction, decided
    /// through `predicate` over the length between them.
    fn ahead(
        &self,
        p: Point3<T>,
        q: Point3<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Sign, NamingError> {
        decide(
            predicate,
            Margin::over_lever((p - q).dot(self.d), self.len),
            bnd,
        )
        .map_err(|source| NamingError::Escalated { predicate, source })
    }

    /// **How this segment lies along `candidates`**, each a key and its
    /// two end points: the candidates on its line that overlap it over
    /// a length, those of them it lies within, and whether together
    /// they cover it end to end ([`walk_cover`]). Every verdict goes
    /// through `predicate`.
    pub(super) fn cover<K>(
        &self,
        candidates: impl IntoIterator<Item = (K, Point3<T>, Point3<T>)>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Cover<K>, NamingError> {
        let mut read = Vec::new();
        for (k, p0, p1) in candidates {
            let stretch =
                if self.on_line(p0, predicate, bnd)? && self.on_line(p1, predicate, bnd)? {
                    match self.ahead(p0, p1, predicate, bnd)? {
                        Sign::Positive => vec![(p1, p0)],
                        _ => vec![(p0, p1)],
                    }
                } else {
                    Vec::new()
                };
            read.push((k, stretch, true));
        }
        walk_cover(read, (self.q0, self.q1), |p, q| {
            Ok(self.ahead(p, q, predicate, bnd)? == Sign::Positive)
        })
    }
}

/// **The flush rule's walk** (`names/README.md`, "Flush edges"), one
/// for a straight edge ([`Segment::cover`]) and a curved one
/// ([`Track::cover`]): over an edge's span from `start` to `end`, where
/// `ahead(p, q)` says `p` lies strictly past `q` along the edge. Each
/// candidate comes as the stretches of the span's parameter it holds
/// (a segment's one; an arc's, and that arc a period back) and whether
/// it may hold the edge whole. A candidate one of whose stretches
/// overlaps the span over a length runs along it; one that also holds
/// it end to end, the edge lies within; and the walk from the start,
/// each step to the farthest end of a stretch that starts at or before
/// the cursor and reaches past it, says whether they cover it.
fn walk_cover<K, S: Copy>(
    candidates: impl IntoIterator<Item = (K, Vec<(S, S)>, bool)>,
    (start, end): (S, S),
    ahead: impl Fn(S, S) -> Result<bool, NamingError>,
) -> Result<Cover<K>, NamingError> {
    let mut out = Cover {
        within: Vec::new(),
        along: Vec::new(),
        covered: false,
    };
    let mut spans: Vec<(S, S)> = Vec::new();
    for (k, stretches, may_hold) in candidates {
        let (mut overlaps, mut within) = (false, false);
        for (lo, hi) in stretches {
            if !ahead(hi, start)? || !ahead(end, lo)? {
                continue;
            }
            overlaps = true;
            within |= may_hold && !ahead(lo, start)? && !ahead(end, hi)?;
            spans.push((lo, hi));
        }
        if within {
            out.within.push(k);
        } else if overlaps {
            out.along.push(k);
        }
    }
    let mut at = start;
    loop {
        if !ahead(end, at)? {
            out.covered = true;
            break;
        }
        let mut next: Option<S> = None;
        for &(lo, hi) in &spans {
            if !ahead(lo, at)?
                && ahead(hi, at)?
                && match next {
                    None => true,
                    Some(n) => ahead(hi, n)?,
                }
            {
                next = Some(hi);
            }
        }
        match next {
            Some(n) => at = n,
            None => break,
        }
    }
    Ok(out)
}

/// What [`Segment::cover`] found: the candidates the segment lies
/// within, those it only overlaps, and whether all of them together
/// cover it.
pub(super) struct Cover<K> {
    pub(super) within: Vec<K>,
    pub(super) along: Vec<K>,
    pub(super) covered: bool,
}

/// A candidate for [`Track::cover`]: an edge's two end points, the
/// point halfway along it, and whether it is closed (one vertex at
/// both ends, structurally).
#[derive(Clone)]
pub(super) struct CarrierArc<T: Decide> {
    pub(super) p0: Point3<T>,
    pub(super) mid: Point3<T>,
    pub(super) p1: Point3<T>,
    pub(super) closed: bool,
}

/// **An edge's span along its certified carrier** — [`Segment`]'s twin
/// for a curved edge, the reading the flush rule takes on a curved
/// carrier (`names/README.md`, "Flush edges"). Positions are carrier
/// parameters, as offsets from the edge's start in its direction;
/// lengths between them are metered in metres through the carrier's
/// speed at the edge's middle. A closed edge is its carrier's whole
/// period: nothing [`Track::cover`] says of it depends on where its
/// vertex sits, which has no identity of its own (`docs/DESIGN.md`,
/// maximal edges).
pub(super) struct Track<T: Decide> {
    carrier: geom::Curve3<T>,
    t0: T,
    mid: T,
    len: T,
    period: Option<T>,
    closed: bool,
    speed: T,
}

impl<T: Decide> Track<T> {
    /// Edge `e` of `body` along its certified carrier, from its
    /// `he_plus` start to its end.
    pub(super) fn of_edge(body: &Body<T>, e: EdgeKey) -> Result<Self, NamingError> {
        let bug = |what| NamingError::Emission { what };
        let curve = body
            .get_edge(e)
            .and_then(|d| body.get_curve_geom(d.curve))
            .and_then(topo::CurveGeom::certified)
            .ok_or(bug("a joined edge's cover has no certified curve"))?;
        let carrier = curve.carrier().clone();
        let (t0, t1) = curve.params();
        let period = match carrier {
            geom::Curve3::Circle { .. }
            | geom::Curve3::Ellipse { .. }
            | geom::Curve3::Spiric { .. } => Some(T::from_f64(std::f64::consts::TAU)),
            _ => None,
        };
        let (v0, v1) = edge_ends(body, e)?;
        let closed = v0 == v1;
        let len = match (closed, period) {
            (true, Some(p)) => p,
            (true, None) => {
                return Err(NamingError::ClosedCarrierUnread {
                    edge: e,
                    carrier: carrier.kind(),
                });
            }
            (false, _) => t1 - t0,
        };
        let mid = geom::mid_param(t0, t1);
        let speed = carrier.deriv(mid).norm();
        Ok(Track {
            carrier,
            t0,
            mid,
            len,
            period,
            closed,
            speed,
        })
    }

    fn sign(
        &self,
        x: T,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Sign, NamingError> {
        decide(predicate, Margin::levered(x, self.speed), bnd)
            .map_err(|source| NamingError::Escalated { predicate, source })
    }

    /// `p`'s offset from the start along the carrier, in `[0, period)`
    /// on a periodic one, where `p` lies on the carrier (its distance
    /// from the carrier's point at its recovered parameter decided
    /// zero through `predicate`); `None` off it.
    fn offset(
        &self,
        p: Point3<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Option<T>, NamingError> {
        let Some(u) = self.carrier.param_near(p, self.mid) else {
            return Ok(None);
        };
        let gap = decide(
            predicate,
            Margin::of((p - self.carrier.eval(u)).norm()),
            bnd,
        )
        .map_err(|source| NamingError::Escalated { predicate, source })?;
        if gap != Sign::Zero {
            return Ok(None);
        }
        let off = u - self.t0;
        Ok(Some(match self.period {
            Some(per) => off - per * (off / per).floor(),
            None => off,
        }))
    }

    /// The stretches of offsets `arc` holds: one on an open carrier;
    /// on a periodic one the arc from the end it starts at through its
    /// middle, and that arc a period back, so a stretch through the
    /// start reads whole on one side or the other. `None` where the arc
    /// is off the carrier.
    fn stretches(
        &self,
        arc: &CarrierArc<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Option<Vec<(T, T)>>, NamingError> {
        let (Some(a0), Some(am), Some(a1)) = (
            self.offset(arc.p0, predicate, bnd)?,
            self.offset(arc.mid, predicate, bnd)?,
            self.offset(arc.p1, predicate, bnd)?,
        ) else {
            return Ok(None);
        };
        let Some(per) = self.period else {
            return Ok(Some(vec![match self.sign(a1 - a0, predicate, bnd)? {
                Sign::Negative => (a1, a0),
                _ => (a0, a1),
            }]));
        };
        if arc.closed {
            return Ok(Some(vec![(T::zero(), per), (T::zero() - per, T::zero())]));
        }
        let fwd = |x: T| x - per * (x / per).floor();
        let (d1, dm) = (fwd(a1 - a0), fwd(am - a0));
        // The middle is half the arc from either end, so this verdict
        // is never near its band.
        let (lo, n) = match self.sign(d1 - dm, predicate, bnd)? {
            Sign::Positive => (a0, d1),
            _ => (a1, per - d1),
        };
        Ok(Some(vec![(lo, lo + n), (lo - per, lo + n - per)]))
    }

    /// **How this edge lies along `candidates`** — [`Segment::cover`]
    /// over the carrier's parameter ([`walk_cover`]): the candidates on
    /// the carrier that overlap the edge over a length, those it lies
    /// within, and whether together they cover it end to end. A closed
    /// edge lies within a closed candidate only, and is covered over
    /// its whole period.
    pub(super) fn cover<K>(
        &self,
        candidates: impl IntoIterator<Item = (K, CarrierArc<T>)>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Cover<K>, NamingError> {
        let mut read = Vec::new();
        for (k, arc) in candidates {
            let stretches = self.stretches(&arc, predicate, bnd)?.unwrap_or_default();
            read.push((k, stretches, arc.closed || !self.closed));
        }
        walk_cover(read, (T::zero(), self.len), |p, q| {
            Ok(self.sign(p - q, predicate, bnd)? == Sign::Positive)
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

/// **Names the pieces of one parent edge** (N2) into `out`: a lone
/// piece is `base` when it is the whole parent ([`Lone::Whole`]), and
/// otherwise, as each of several is, the parent's line
/// ([`edge_line`]) + `Fragment(Ends)`, the sorted pair of its two end
/// vertices' names as `t` publishes them, read off body `ix`: a lone
/// piece of a divided edge is named by its ends, so no piece's name
/// says how many siblings it has, and a piece of an earlier piece is a
/// piece of its line. Every end vertex is named before this runs, so
/// `t` holds its name.
///
/// # Errors
///
/// [`NamingError::Emission`] for a piece ending at a vertex `t` does
/// not name.
pub(super) fn name_edge_pieces<T: geom_core::Real>(
    out: &mut EdgePieces,
    t: &NameTable,
    from_tie: bool,
    base: &StableName,
    (body, ix): (&Body<T>, u32),
    edges: &[EdgeKey],
    lone: Lone,
) -> Result<(), NamingError> {
    if let ([one], Lone::Whole) = (edges, lone) {
        out.push(base.clone(), from_tie, ent(ix, EntityKey::Edge(*one)));
        return Ok(());
    }
    let line = edge_line(&NameRef::new(base.clone()));
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
        let mut name = (*line).clone();
        name.path.push(RoleSeg::Fragment(Qualifier::Ends(ends)));
        out.push(name, from_tie, ent(ix, EntityKey::Edge(e)));
    }
    Ok(())
}

/// **The edge pieces of one emission**, gathered over every parent edge
/// ([`name_edge_pieces`]) and minted together: two parents on one line
/// whose pieces have one pair of ends are N4's tie, as two pieces of
/// one parent are.
#[derive(Default)]
pub(super) struct EdgePieces(BTreeMap<StableName, (bool, Vec<super::table::EntityRef>)>);

impl EdgePieces {
    fn push(&mut self, name: StableName, from_tie: bool, e: super::table::EntityRef) {
        let slot = self.0.entry(name).or_default();
        slot.0 |= from_tie;
        slot.1.push(e);
    }

    /// Mints every gathered name: strict for one piece, tied for
    /// several.
    ///
    /// # Errors
    ///
    /// The insert doors' own refusals.
    pub(super) fn mint(self, t: &mut NameTable, tie: &mut TieRows) -> Result<(), NamingError> {
        for (name, (from_tie, ents)) in self.0 {
            mint_candidates(t, tie, from_tie, name, ents)?;
        }
        Ok(())
    }
}

/// What a lone piece of a parent edge is named for: the parent whole,
/// or, where a set-named edge holds the rest of the parent, a piece of
/// it by its ends (the parent is not the piece's cell; N3).
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Lone {
    Whole,
    Piece,
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
/// own table `table`, are read along it — their senses and their ranks
/// (N2): `Some(true)` along the edge as `body` stores it, `Some(false)`
/// against it, and `None` where no orientation is defined, which no
/// sense can be read along and which ties the ranks.
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
    let sides = topo::readback::edge_sides(body, e)
        .map_err(|_| bug("a crossed seam edge is not live in its body"))?;
    let mut names = Vec::with_capacity(2);
    let (plus, minus) = sides.faces();
    for face in [plus, minus] {
        names.push(
            table
                .name_of(&ent(0, EntityKey::Face(face)))
                .ok_or_else(|| bug("a crossed seam edge's face is unnamed"))?,
        );
    }
    Ok(seam_pair::a_side_is_first(names[0], names[1], a, b))
}

/// Edge `e`'s certified carrier in `body`.
fn crossed_curve<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
) -> Result<&geom_brep::EdgeCurve<T>, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let edge = body
        .get_edge(e)
        .ok_or_else(|| bug("a crossed edge is not live in its body"))?;
    body.get_curve_geom(edge.curve)
        .ok_or_else(|| bug("a crossed edge's carrier is dangling"))?
        .certified()
        .ok_or_else(|| bug("a crossed edge carries no certified carrier"))
}

/// Where `p`, a point on edge `e` of `body`, lies along it: the edge's
/// carrier's own parameter, increasing as the edge runs as stored.
/// `None` for a carrier with no closed-form parameter here (a NURBS
/// curve). The parameter is read near the middle of the edge's
/// certified interval, so a closed edge's crossings, which lie inside
/// it, are read without the period's cut between them.
///
/// `p` lies within ε of the carrier, so two readings the band decides
/// apart, by at least Kε, order the points' feet only where Kε > 2ε:
/// this reading, like [`chord_along`]'s, relies on K > 2.
pub(super) fn param_along<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    p: Point3<T>,
) -> Result<Option<T>, NamingError> {
    use geom::Curve3;
    let curve = crossed_curve(body, e)?;
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

/// The edge a crossing is read along: edge `edge` of `body`, named
/// `name` in its own table `table`.
pub(super) struct CrossedEdge<'a, T: geom_core::Real> {
    pub(super) body: &'a Body<T>,
    pub(super) table: &'a NameTable,
    pub(super) edge: EdgeKey,
    pub(super) name: &'a StableName,
}

impl<T: geom_core::Real> Clone for CrossedEdge<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: geom_core::Real> Copy for CrossedEdge<'_, T> {}

/// One crossing of a group [`rank_crossings`] ranks: its entity, the
/// point it lies at, and the edge on the line it is read along.
pub(super) type Crossed<'a, T> = (super::table::EntityRef, Point3<T>, CrossedEdge<'a, T>);

/// **Ranks the crossings of one line by one face with one sense** (N2):
/// a lone crossing is `base`, and several take `base` +
/// `Fragment(OrderAlong)` by where each lies along the line.
///
/// Each crossing is read on the edge it lies on, a piece of the line,
/// by the carrier's own parameter within that piece's certified
/// interval ([`param_along`]): the pieces of one line are sub-intervals
/// of one parameterization, so every crossing reads in the line's
/// coordinates and no piece's name chooses where a closed carrier's
/// period is cut. Where the carrier has no closed-form parameter, a
/// crossing spans its piece's interval ([`param_span`]), so crossings
/// on different pieces order by their pieces, and two on one piece
/// order by where they lie along its chord when [`chord_along`]
/// certifies that this is the order of their parameters; otherwise
/// they tie. The chord is asked for only for such a pair. The
/// orientation is each piece's [`crossed_edge_orientation`]; with none,
/// or pieces that disagree, they tie.
pub(super) fn rank_crossings<T: Decide>(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    base: &StableName,
    crossings: &[Crossed<'_, T>],
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let keys: Vec<super::table::EntityRef> = crossings.iter().map(|&(e, _, _)| e).collect();
    if let [one] = keys.as_slice() {
        return Ok(put(t, tie, from_tie, base.clone(), *one)?);
    }
    let mut way: Option<bool> = None;
    let mut reads = Vec::with_capacity(crossings.len());
    for &(_, p, along) in crossings {
        let CrossedEdge {
            body,
            table,
            edge,
            name,
        } = along;
        let forward = match (crossed_edge_orientation(body, table, edge, name)?, way) {
            (Some(f), None) => f,
            (Some(f), Some(w)) if f == w => f,
            _ => return Ok(mint_candidates(t, tie, from_tie, base.clone(), keys)?),
        };
        way = Some(forward);
        let ((lo, hi), unread) = match param_along(body, edge, p)? {
            Some(at) => ((at, at), None),
            None => (param_span(body, edge)?, Some((body, edge))),
        };
        let extent = if forward {
            Extent { min: lo, max: hi }
        } else {
            Extent { min: -hi, max: -lo }
        };
        reads.push((extent, p, unread));
    }
    let forward = way.unwrap_or(true);
    type PieceKey<T> = (*const Body<T>, EdgeKey);
    let mut chords: Vec<(PieceKey<T>, Option<Chord<T>>)> = Vec::new();
    let ranks = rank_by(reads.len(), |i, j| match (&reads[i], &reads[j]) {
        ((_, pi, Some((bi, ei))), (_, pj, Some((bj, ej))))
            if std::ptr::eq(*bi, *bj) && ei == ej =>
        {
            let piece = (std::ptr::from_ref(*bi), *ei);
            let chord = match chords.iter().find(|(k, _)| *k == piece) {
                Some(&(_, chord)) => chord,
                None => {
                    let chord = chord_along(bi, *ei, bnd)?;
                    chords.push((piece, chord));
                    chord
                }
            };
            let Some(chord) = chord else {
                return Ok(None);
            };
            let at = |p: Point3<T>| {
                let c = chord.along(p);
                let c = if forward { c } else { -c };
                Extent { min: c, max: c }
            };
            extent_before(&at(*pi), &at(*pj), bnd)
        }
        ((xi, _, _), (xj, _, _)) => extent_before(xi, xj, bnd),
    })?;
    insert_ranked_or_tied(t, tie, from_tie, base, &keys, ranks, |e| *e)
}

/// A NURBS piece's chord, certified to order the piece's points as its
/// carrier's parameter does ([`chord_along`]).
#[derive(Clone, Copy)]
pub(super) struct Chord<T: geom_core::Real> {
    start: Point3<T>,
    d: Vec3<T>,
    len: T,
}

impl<T: geom_core::Real> Chord<T> {
    /// Where `p` lies along the chord, in meters from the piece's start.
    pub(super) fn along(&self, p: Point3<T>) -> T {
        (p - self.start).dot(self.d) / self.len
    }
}

/// The chord of edge `e` of `body`, when the edge's carrier is a NURBS
/// curve and readings along the chord order points as the carrier's
/// parameter does; `None` for any other carrier, or where that is not
/// certain. It never escalates: it is a sufficient certificate, so a
/// step it cannot decide positive, in-band included, is no certificate,
/// and the pair ties. Only the comparison of two readings decides.
///
/// The chord must have a length decided positive (a closed piece has
/// none), and the control points that shape the edge's certified
/// interval must advance strictly along it, each step decided positive
/// through `name_frag_order_along`. The weights are positive, so knot
/// insertion cuts that polygon down to the interval's own as convex
/// combinations of neighbours, which keeps it advancing, and variation
/// diminishing lets no plane across the chord meet the curve there
/// twice: the interval is a graph over its chord. Two points' readings
/// then differ with their parameters, and a difference the band
/// decides is the order of their feet, however each point sits off the
/// curve within ε, given K > 2 (as for [`param_along`]).
///
/// `geom`'s `NurbsCurve3::speed_lower_bound` makes a similar test in
/// its chord assembly: derivative coefficients projected on a chord.
/// It bounds the speed of the whole carrier, and certification refuses
/// a carrier where it collapses; this one asks only the piece's own
/// control points, along the piece's own chord, and a piece that fails
/// it ties. A carrier that certifies can still fail this test, on a
/// piece that folds back along its chord or sways across it.
pub(super) fn chord_along<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    bnd: geom_core::Band,
) -> Result<Option<Chord<T>>, NamingError> {
    let curve = crossed_curve(body, e)?;
    let geom::Curve3::Nurbs(nurbs) = curve.carrier() else {
        return Ok(None);
    };
    let (t0, t1) = curve.params();
    nurbs_chord(nurbs, t0, t1, bnd)
}

/// [`chord_along`]'s certificate for `nurbs` over `(t0, t1)`.
fn nurbs_chord<T: Decide>(
    nurbs: &geom::NurbsCurve3<T>,
    t0: T,
    t1: T,
    bnd: geom_core::Band,
) -> Result<Option<Chord<T>>, NamingError> {
    let positive = |m: Margin<T>| matches!(decide(ORDER_ALONG, m, bnd), Ok(Sign::Positive));
    let start = nurbs.eval(t0);
    let d = nurbs.eval(t1) - start;
    let len = d.norm();
    if !positive(Margin::of(len)) {
        return Ok(None);
    }
    let knots = nurbs.knots();
    let (s0, s1) = (t0.locate_spans(knots), t1.locate_spans(knots));
    let first = s0.first.first_control().min(s1.first.first_control());
    let last = s0.last.index().max(s1.last.index());
    let control = nurbs
        .control()
        .get(first..=last)
        .ok_or(NamingError::Emission {
            what: "a NURBS carrier's span window runs past its control net",
        })?;
    let advances = control
        .windows(2)
        .all(|step| positive(Margin::over_lever((step[1] - step[0]).dot(d), len)));
    Ok(advances.then_some(Chord { start, d, len }))
}

/// Edge `e`'s certified parameter interval on its carrier, increasing
/// as the edge runs as stored.
fn param_span<T: Decide>(body: &Body<T>, e: EdgeKey) -> Result<(T, T), NamingError> {
    Ok(crossed_curve(body, e)?.params())
}

/// Inserts a same-name group by its `ranks` (`rank_by`), or tied
/// where they are `None`.
fn insert_ranked_or_tied<K: Copy>(
    t: &mut NameTable,
    tie: &mut TieRows,
    from_tie: bool,
    base: &StableName,
    keys: &[K],
    ranks: Option<Vec<u32>>,
    to_ent: impl Fn(&K) -> super::table::EntityRef,
) -> Result<(), NamingError> {
    match ranks {
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
        // lines. A piece's edge is one of them when it descends from
        // one: whole, or as a piece the plane cut.
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
                    kept.insert((*edge_line(&up.name)).clone());
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
                RecipeNodeId::new(0, node),
                RoleSeg::CapVertex(
                    super::super::role::CapEnd::End,
                    super::super::role::ProfileVertexRef::Piece {
                        step: crate::node::StepId::new(0, 0),
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
            .mvfs(Point3::from_array(p), true)
            .expect("mvfs births a lone vertex");
        let edge = body
            .mev_line(
                topo::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                Point3::from_array(q),
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
            sweep::Extrusion::Distance {
                depth: 1.0_f64,
                side: crate::ExtrudeSide::Along,
            },
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
        assert_eq!(
            out.kef_minting(he, Tol::witness()).unwrap().killed_face,
            absorbed
        );
        out
    }

    #[test]
    fn merged_lane_names_kept_face_with_sorted_deduped_constituents() {
        // A unit-cube extrusion: the "result body" stand-in.
        let built = unit_cube();
        let ext_node = RecipeNodeId::new(0, 1);
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
        let bool_node = RecipeNodeId::new(0, 9);
        let a = OperandCtx {
            node: ext_node,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
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
        let ext_node = RecipeNodeId::new(0, 1);
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
            node: RecipeNodeId::new(0, 2),
            table: &empty,
            body: &built.body,
        };
        let err = name_boolean(
            RecipeNodeId::new(0, 9),
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
        let ext_node = RecipeNodeId::new(0, 1);
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
            node: RecipeNodeId::new(0, 2),
            table: &empty,
            body: &built.body,
        };
        let err = name_boolean(
            RecipeNodeId::new(0, 9),
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
        let ext_node = RecipeNodeId::new(0, 1);
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
            node: RecipeNodeId::new(0, 2),
            table: &b_table,
            body: &body,
        };
        let b = OperandCtx {
            node: ext_node,
            table: &b_table,
            body: &body,
        };
        let err = name_boolean(
            RecipeNodeId::new(0, 9),
            &body,
            &naming,
            &a,
            &b,
            Tol::witness(),
        )
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
        let ext_node = RecipeNodeId::new(0, 1);
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
            node: RecipeNodeId::new(0, 2),
            table: &empty,
            body: &built.body,
        };
        let err = name_boolean(
            RecipeNodeId::new(0, 9),
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
        let ext_node = RecipeNodeId::new(0, 1);
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
            node: RecipeNodeId::new(0, 2),
            table: &empty,
            body: &built.body,
        };
        let out = name_boolean(
            RecipeNodeId::new(0, 9),
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
        let ext_node = RecipeNodeId::new(0, 1);
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
        let bool_node = RecipeNodeId::new(0, 9);
        let a = OperandCtx {
            node: ext_node,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
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
        let ext_node = RecipeNodeId::new(0, 1);
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
            node: RecipeNodeId::new(0, 2),
            table: &empty,
            body: &built.body,
        };
        let err = name_boolean(
            RecipeNodeId::new(0, 9),
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

    fn ins(doc: ProfileDoc, node: crate::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
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
                side: crate::ExtrudeSide::Along,
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
                declare: Vec::new(),
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
            &topo::test_support::finished("the extrude", (**body).clone(), Tol::witness()),
            &SplitPlane {
                origin: *origin,
                normal: *normal,
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

#[cfg(test)]
mod crossings_rank_along_the_line {
    //! **Crossings on two pieces of one curved line rank in the line's
    //! own coordinates** (N2, *Vertices*). A 270° arc's top rim is one
    //! line; a notch near 200° leaves it two arcs, `A` holding 20° and
    //! `B` holding 260°. Read on `B`'s window of the carrier, the point
    //! at 20° lies past 260°; read on its own piece it lies where the
    //! line runs. So the group ranks the crossing on the piece that comes
    //! first along the line first, whichever order the pieces are handed
    //! in and so whichever of them is named least.
    //!
    //! Stand-in crossings, because no body the kernel builds today puts
    //! two same-sense crossings of one line by one face on two pieces of
    //! it: that needs a face meeting the line's carrier four times (a
    //! tilted cylinder through a rim circle), and the boolean refuses
    //! non-parallel cylinder pairs (`GermFrameUnsupported`).
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{CrossedEdge, param_span, rank_crossings};
    use crate::edit::DocEdit;
    use crate::eval::{BooleanValue, CancelToken, EvalOptions, ValuePayload, evaluate};
    use crate::ident::DocumentId;
    use crate::names::defer::TieRows;
    use crate::names::role::{EntityKind, Qualifier, RoleSeg, StableName};
    use crate::names::table::{EntityKey, EntityRef, Entry, NameTable};
    use crate::node::{BooleanOp, Node, RecipeNodeId};
    use crate::program::{LoopProgram, ProfileProgram, ProgramArcData, ProgramStep, ProgramTarget};
    use crate::test_support::{frame, len, scl};
    use crate::{ProfileDoc, RefusingReach};
    use geom_core::{Point3, Tol};

    fn ins(doc: ProfileDoc, node: crate::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &RefusingReach,
        )
        .expect("inserts");
        (a.doc, a.record.minted.expect("a node"))
    }

    fn extrude(
        doc: ProfileDoc,
        z0: f64,
        dz: f64,
        loop_: LoopProgram<crate::Formula>,
    ) -> (ProfileDoc, RecipeNodeId) {
        let (doc, plane) = ins(doc, frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        let (doc, profile) = ins(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![loop_],
                ids: Vec::new(),
            }),
        );
        ins(
            doc,
            Node::Extrude {
                profile,
                distance: len(dz),
                side: crate::ExtrudeSide::Along,
            },
        )
    }

    fn rim(deg: f64) -> Point3<f64> {
        let t = deg.to_radians();
        Point3::new(t.cos(), t.sin(), 1.0)
    }

    #[test]
    fn crossings_on_two_pieces_of_a_closed_line_rank_by_the_line() {
        let doc = ProfileDoc::empty(
            DocumentId::derive("crossings-along-the-line"),
            Tol::witness(),
        );
        // A 270° arc from 0° about the origin, closed through the
        // centre: its top rim arc is one edge, one line.
        let (doc, disc) = extrude(
            doc,
            0.0,
            1.0,
            LoopProgram::Chain(vec![
                ProgramStep::At([len(1.0), len(0.0)]),
                ProgramStep::ArcTo(ProgramArcData::Bulge {
                    target: ProgramTarget::Point([len(0.0), len(-1.0)]),
                    b: scl((3.0 * std::f64::consts::PI / 8.0).tan()),
                }),
                ProgramStep::LineTo(ProgramTarget::Point([len(0.0), len(0.0)])),
                ProgramStep::LineTo(ProgramTarget::Start),
            ]),
        );
        let (doc, notch) = extrude(
            doc,
            0.8,
            1.0,
            LoopProgram::polygon([(-1.5, -0.38), (-0.6, -0.38), (-0.6, -0.30), (-1.5, -0.30)])
                .expect("finite"),
        );
        let (doc, notched) = ins(
            doc,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a: disc,
                b: notch,
                declare: Vec::new(),
            },
        );
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let value = ev.value(notched).expect("the notched disc evaluates");
        let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
            panic!("a body");
        };
        let table = &value.name_table;
        // The two top-rim arcs, each with its name and its interval.
        let mut arcs = Vec::new();
        for (name, entry) in table.iter() {
            let (Entry::Unique(r), Some(RoleSeg::Fragment(Qualifier::Ends(_)))) =
                (entry, name.path.last())
            else {
                continue;
            };
            let EntityKey::Edge(e) = r.key else { continue };
            let (v0, v1) = super::edge_ends(body, e).unwrap();
            let z = |v| super::vertex_point(body, v).unwrap().z;
            if (z(v0) - 1.0).abs() < 1e-9 && (z(v1) - 1.0).abs() < 1e-9 {
                arcs.push((e, name.clone(), param_span(body, e).unwrap()));
            }
        }
        assert_eq!(arcs.len(), 2, "the notch leaves the rim two arcs: {arcs:?}");
        let on = |deg: f64| {
            let p = rim(deg);
            arcs.iter()
                .find(|(e, _, _)| {
                    super::param_along(body, *e, p).unwrap().is_some_and(|t| {
                        let (t0, t1) = param_span(body, *e).unwrap();
                        t0 <= t && t <= t1
                    })
                })
                .unwrap_or_else(|| panic!("an arc holds {deg}°"))
        };
        let (a, b) = (on(20.0), on(260.0));
        assert_ne!(a.0, b.0, "the two crossings lie on two arcs");
        let (on_a, on_b) = (
            CrossedEdge {
                body,
                table,
                edge: a.0,
                name: &a.1,
            },
            CrossedEdge {
                body,
                table,
                edge: b.0,
                name: &b.1,
            },
        );
        // Two stand-in vertices for the crossings: the ranks are the
        // question, not the entities.
        let mut vs = body.vertices().map(|(v, _)| v);
        let (x, y) = (vs.next().unwrap(), vs.next().unwrap());
        let ent = |v| EntityRef {
            body: 0,
            key: EntityKey::Vertex(v),
        };
        let base = StableName {
            kind: EntityKind::Vertex,
            node: notched,
            path: vec![RoleSeg::OutputBody],
        };
        let rank = |crossings: &[(EntityRef, Point3<f64>, CrossedEdge<'_, f64>)]| {
            let mut t = NameTable::new();
            let mut tie = TieRows::default();
            rank_crossings(
                &mut t,
                &mut tie,
                false,
                &base,
                crossings,
                geom_core::Band::linear(Tol::witness()).unwrap(),
            )
            .unwrap();
            tie.flush(&mut t).unwrap();
            let rank_of = |v| match t.name_of(&ent(v)).and_then(|n| n.path.last().cloned()) {
                Some(RoleSeg::Fragment(Qualifier::OrderAlong { rank, .. })) => rank,
                other => panic!("a ranked crossing, not {other:?}"),
            };
            (rank_of(x), rank_of(y))
        };
        // Along the line, the piece with the lesser interval comes first.
        let want = if a.2.0 < b.2.0 { (0, 1) } else { (1, 0) };
        let one = rank(&[(ent(x), rim(20.0), on_a), (ent(y), rim(260.0), on_b)]);
        let two = rank(&[(ent(y), rim(260.0), on_b), (ent(x), rim(20.0), on_a)]);
        assert_eq!(one, want, "ranks along the line, A {:?} B {:?}", a.2, b.2);
        assert_eq!(two, want, "the order handed in does not rank");
    }
}

#[cfg(test)]
mod nurbs_crossings_rank_by_parameter {
    //! **Two crossings of one NURBS piece rank by the carrier's
    //! parameter** (N2, *Vertices*), read along the piece's chord where
    //! that certifies the order ([`super::chord_along`]), and tie where
    //! the band cannot part them. A loft through four squares
    //! whose x offsets alternate has wavy corner edges, NURBS curves a
    //! plane across them meets three times; the first and third
    //! crossings have one sense.
    //!
    //! Stand-in crossings, because no body the kernel accepts puts a
    //! face across a NURBS edge: the split refuses a plane that may meet
    //! one, and the boolean refuses an operand that has one
    //! (`CurvedEdgeUnsupported`).
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{Crossed, CrossedEdge, param_span, rank_crossings};
    use crate::edit::DocEdit;
    use crate::eval::{CancelToken, EvalOptions, ValuePayload, evaluate};
    use crate::ident::DocumentId;
    use crate::names::defer::TieRows;
    use crate::names::role::{EntityKind, Qualifier, RoleSeg, StableName};
    use crate::names::table::{EntityKey, EntityRef, Entry, NameTable};
    use crate::node::{Node, RecipeNodeId};
    use crate::program::{LoopProgram, ProfileProgram};
    use crate::test_support::frame;
    use crate::{Formula, ProfileDoc, RefusingReach};
    use geom_core::{Point3, Tol};
    use topo::{Body, EdgeKey};

    fn ins(doc: ProfileDoc, node: crate::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &RefusingReach,
        )
        .expect("inserts");
        (a.doc, a.record.minted.expect("a node"))
    }

    /// The loft through 2 × 2 squares at z = 0, 1, 2, 3 whose x offsets
    /// alternate `−a, a, −a, a`, skinned at v-degree 2: its value.
    fn wavy(a: f64) -> (RecipeNodeId, crate::eval::NodeValue<f64>) {
        let mut doc = ProfileDoc::empty(DocumentId::derive("nurbs-crossings"), Tol::witness());
        let mut profiles = Vec::new();
        for (k, dx) in [-a, a, -a, a].into_iter().enumerate() {
            let z = [0.0, 1.0, 2.0, 3.0][k];
            let (d, plane) = ins(doc, frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
            let square = [
                (dx - 1.0, -1.0),
                (dx + 1.0, -1.0),
                (dx + 1.0, 1.0),
                (dx - 1.0, 1.0),
            ];
            let (d, profile) = ins(
                d,
                Node::Profile(ProfileProgram {
                    plane,
                    loops: vec![LoopProgram::polygon(square).expect("finite")],
                    ids: Vec::new(),
                }),
            );
            doc = d;
            profiles.push(profile);
        }
        let (doc, loft) = ins(
            doc,
            Node::Loft {
                profiles,
                v_degree: Formula::count(2),
            },
        );
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let value = ev.value(loft).expect("the loft evaluates").clone();
        (loft, value)
    }

    fn body_of(value: &crate::eval::NodeValue<f64>) -> &Body<f64> {
        let ValuePayload::Body(body) = &value.payload else {
            panic!("a body");
        };
        body
    }

    /// A corner edge — one whose carrier is a NURBS curve and whose ends
    /// lie on the first and last sections — with its name.
    fn corner(body: &Body<f64>, table: &NameTable) -> (EdgeKey, StableName) {
        table
            .iter()
            .find_map(|(name, entry)| {
                let Entry::Unique(r) = entry else { return None };
                let EntityKey::Edge(e) = r.key else {
                    return None;
                };
                let edge = body.get_edge(e)?;
                let carrier = body.get_curve_geom(edge.curve)?.certified()?.carrier();
                let (v0, v1) = super::edge_ends(body, e).ok()?;
                let z = |v| super::vertex_point(body, v).unwrap().z;
                let spans = (z(v0) - z(v1)).abs() > 2.5;
                (matches!(carrier, geom::Curve3::Nurbs(_)) && spans).then(|| (e, name.clone()))
            })
            .expect("a NURBS corner edge")
    }

    /// The parameters at which the corner's carrier crosses the plane
    /// `x = c` its two ends straddle evenly, with the sign of `x − c`
    /// after each: test-side root finding, the subject being the ranks.
    fn crossings_of(body: &Body<f64>, e: EdgeKey) -> Vec<(f64, bool)> {
        let carrier = carrier(body, e);
        let (t0, t1) = param_span(body, e).unwrap();
        let c = (carrier.eval(t0).x + carrier.eval(t1).x) / 2.0;
        let f = |t: f64| carrier.eval(t).x - c;
        let n = 4096;
        let at = |k: usize| t0 + (t1 - t0) * k as f64 / n as f64;
        let mut roots = Vec::new();
        for k in 0..n {
            let (mut lo, mut hi) = (at(k), at(k + 1));
            if (f(lo) < 0.0) == (f(hi) < 0.0) {
                continue;
            }
            let rising = f(hi) > 0.0;
            for _ in 0..80 {
                let mid = (lo + hi) / 2.0;
                if (f(mid) > 0.0) == rising {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            roots.push(((lo + hi) / 2.0, rising));
        }
        roots
    }

    /// The ranks of two stand-in crossings at `p` and `q` on `e`, or
    /// `None` where they tie; handed in both ways round, which must
    /// agree.
    fn ranks(
        node: RecipeNodeId,
        (body, table): (&Body<f64>, &NameTable),
        e: EdgeKey,
        name: &StableName,
        p: Point3<f64>,
        q: Point3<f64>,
    ) -> Option<(u32, u32)> {
        let on = CrossedEdge {
            body,
            table,
            edge: e,
            name,
        };
        let mut vs = body.vertices().map(|(v, _)| v);
        let (x, y) = (vs.next().unwrap(), vs.next().unwrap());
        let ent = |v| EntityRef {
            body: 0,
            key: EntityKey::Vertex(v),
        };
        let base = StableName {
            kind: EntityKind::Vertex,
            node,
            path: vec![RoleSeg::OutputBody],
        };
        let rank = |crossings: &[Crossed<'_, f64>]| {
            let mut t = NameTable::new();
            let mut tie = TieRows::default();
            rank_crossings(
                &mut t,
                &mut tie,
                false,
                &base,
                crossings,
                geom_core::Band::linear(Tol::witness()).unwrap(),
            )
            .unwrap();
            tie.flush(&mut t).unwrap();
            if t.is_tied(&base) {
                return None;
            }
            let rank_of = |v| match t.name_of(&ent(v)).and_then(|n| n.path.last().cloned()) {
                Some(RoleSeg::Fragment(Qualifier::OrderAlong { rank, .. })) => rank,
                other => panic!("a ranked or tied crossing, not {other:?}"),
            };
            Some((rank_of(x), rank_of(y)))
        };
        let one = rank(&[(ent(x), p, on), (ent(y), q, on)]);
        let two = rank(&[(ent(y), q, on), (ent(x), p, on)]);
        assert_eq!(one, two, "the order handed in does not rank");
        one
    }

    fn carrier(body: &Body<f64>, e: EdgeKey) -> &geom::Curve3<f64> {
        let edge = body.get_edge(e).unwrap();
        body.get_curve_geom(edge.curve)
            .unwrap()
            .certified()
            .unwrap()
            .carrier()
    }

    fn point_at(body: &Body<f64>, e: EdgeKey, t: f64) -> Point3<f64> {
        carrier(body, e).eval(t)
    }

    #[test]
    fn two_crossings_of_one_nurbs_piece_rank_by_its_parameter() {
        let (loft, value) = wavy(0.25);
        let body = body_of(&value);
        let (e, name) = corner(body, &value.name_table);
        let (t0, t1) = param_span(body, e).unwrap();
        assert!(t0 < t1, "the corner runs as its carrier: {t0} to {t1}");
        let roots = crossings_of(body, e);
        assert_eq!(
            roots.len(),
            3,
            "the plane meets the corner three times: {roots:?}"
        );
        let ((first, s0), (third, s2)) = (roots[0], roots[2]);
        assert_eq!(s0, s2, "the first and third crossings have one sense");
        let (p, q) = (point_at(body, e, first), point_at(body, e, third));
        assert_eq!(
            ranks(loft, (body, &value.name_table), e, &name, p, q),
            Some((0, 1)),
            "the crossing at the lesser parameter ranks first: t {first} and {third}"
        );
    }

    #[test]
    fn two_crossings_of_one_nurbs_piece_the_band_cannot_part_tie() {
        let (loft, value) = wavy(0.25);
        let body = body_of(&value);
        let (e, name) = corner(body, &value.name_table);
        let first = crossings_of(body, e)[0].0;
        let p = point_at(body, e, first);
        let along = carrier(body, e).deriv(first);
        let zero = geom_core::Band::linear(Tol::witness()).unwrap().zero();
        let apart = zero / 4.0;
        let q = p + along * (apart / along.norm());
        assert_eq!(
            ranks(loft, (body, &value.name_table), e, &name, p, q),
            None,
            "{apart} m apart"
        );
        assert_eq!(
            ranks(loft, (body, &value.name_table), e, &name, p, p),
            None,
            "one point"
        );
    }

    /// The unit cube with its bottom front edge (`mevs[0]`, between the
    /// bottom and front walls) carried by a degree-1 NURBS curve whose
    /// control points lie along that edge at fractions `along` of it, on
    /// `knots`: the edge, with the body.
    fn cube_piece(along: [f64; 4], knots: [f64; 6]) -> (Body<f64>, EdgeKey) {
        use topo::test_support::{describe_as_intersections, geometric_cube};
        let mut cube = geometric_cube::<f64>(Tol::witness());
        describe_as_intersections(&mut cube.body, Tol::witness());
        let e = cube.mevs[0].edge;
        let body = &mut cube.body;
        let (v0, v1) = super::edge_ends(body, e).unwrap();
        let (a, b) = (
            super::vertex_point(body, v0).unwrap(),
            super::vertex_point(body, v1).unwrap(),
        );
        let control = along.iter().map(|&s| a.lerp(b, s)).collect();
        let nurbs = geom::NurbsCurve3::new(
            geom_core::KnotVector::clamped(knots.to_vec(), 1).unwrap(),
            control,
            vec![1.0; 4],
        )
        .unwrap();
        let curve = body.get_edge(e).unwrap().curve;
        let mut spec = body
            .get_curve_geom(curve)
            .unwrap()
            .certified()
            .unwrap()
            .restated_spec();
        let geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, .. } = spec.description else {
            panic!("the cube's edges are described as intersections");
        };
        spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
            s1,
            s2,
            witness: nurbs.eval(0.5),
        };
        spec.carrier = geom::Curve3::Nurbs(std::sync::Arc::new(nurbs));
        spec.param_start = 0.0;
        spec.param_end = 1.0;
        body.set_edge_curve(e, spec, Tol::witness())
            .expect("the NURBS edge certifies");
        (cube.body, e)
    }

    fn cube_name() -> StableName {
        StableName {
            kind: EntityKind::Edge,
            node: RecipeNodeId::new(0, 0),
            path: vec![RoleSeg::OutputBody],
        }
    }

    /// A control step in the band is no certificate, not a refusal: the
    /// two crossings tie.
    #[test]
    fn a_nurbs_piece_with_a_control_step_in_the_band_ties_its_crossings() {
        let zero = geom_core::Band::linear(Tol::witness()).unwrap().zero();
        let (body, e) = cube_piece(
            [0.0, 0.4, 0.4 + 3.0 * zero, 1.0],
            [0.0, 0.0, 0.45, 0.45 + 1e-6, 1.0, 1.0],
        );
        let (p, q) = (point_at(&body, e, 0.2), point_at(&body, e, 0.9));
        let table = NameTable::new();
        let name = cube_name();
        assert_eq!(
            ranks(RecipeNodeId::new(0, 0), (&body, &table), e, &name, p, q),
            None
        );
    }

    /// A piece that folds back along its chord ties: at t = 0.25 it is
    /// 0.6 along, and at t = 0.6 only 0.4, so the chord would rank the
    /// two the wrong way round.
    #[test]
    fn a_nurbs_piece_folding_back_along_its_chord_ties_its_crossings() {
        let (body, e) = cube_piece(
            [0.0, 0.8, 0.3, 1.0],
            [0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0],
        );
        let (p, q) = (point_at(&body, e, 0.25), point_at(&body, e, 0.6));
        let (a, b) = (point_at(&body, e, 0.0), point_at(&body, e, 1.0));
        let at = |x: Point3<f64>| (x - a).norm() / (b - a).norm();
        assert!(
            (at(p) - 0.6).abs() < 1e-12 && (at(q) - 0.4).abs() < 1e-12,
            "the fold"
        );
        let table = NameTable::new();
        let name = cube_name();
        assert_eq!(
            ranks(RecipeNodeId::new(0, 0), (&body, &table), e, &name, p, q),
            None
        );
    }

    /// A closed piece has no chord: no certificate, and no refusal.
    #[test]
    fn a_closed_nurbs_piece_has_no_chord() {
        let control = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 0.0)]
            .map(|(x, y)| Point3::new(x, y, 0.0))
            .to_vec();
        let nurbs = geom::NurbsCurve3::new(
            geom_core::KnotVector::clamped(vec![0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0], 1)
                .unwrap(),
            control,
            vec![1.0; 4],
        )
        .unwrap();
        let bnd = geom_core::Band::linear(Tol::witness()).unwrap();
        let chord = super::nurbs_chord(&nurbs, 0.0, 1.0, bnd).expect("no refusal");
        assert!(chord.is_none(), "a closed piece certifies nothing");
    }
}

#[cfg(test)]
mod edge_pieces_of_one_line_tie {
    //! **Pieces of two parents on one line with one pair of ends are
    //! N4's tie** (N2, *Edge pieces*), as two pieces of one parent are:
    //! both are spelled on the line, so their names are one name. A rod
    //! cut flat leaves its top rim two half circles between the same two
    //! vertices; named as pieces of two parent edges that lie on one
    //! line, they mint one tied row rather than a duplicate.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::{EdgePieces, Lone, name_edge_pieces};
    use crate::edit::DocEdit;
    use crate::eval::{BooleanValue, CancelToken, EvalOptions, ValuePayload, evaluate};
    use crate::ident::DocumentId;
    use crate::names::defer::TieRows;
    use crate::names::role::{EntityKind, NameRef, Qualifier, RoleSeg, StableName};
    use crate::names::table::{Entry, NameTable};
    use crate::node::{BooleanOp, Node, RecipeNodeId};
    use crate::program::{LoopProgram, ProfileProgram};
    use crate::test_support::{frame, len};
    use crate::{ProfileDoc, RefusingReach};
    use geom_core::Tol;

    fn ins(doc: ProfileDoc, node: crate::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
            &RefusingReach,
        )
        .expect("inserts");
        (a.doc, a.record.minted.expect("a node"))
    }

    fn extrude(
        doc: ProfileDoc,
        z0: f64,
        dz: f64,
        loop_: LoopProgram<crate::Formula>,
    ) -> (ProfileDoc, RecipeNodeId) {
        let (doc, plane) = ins(doc, frame([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        let (doc, profile) = ins(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![loop_],
                ids: Vec::new(),
            }),
        );
        ins(
            doc,
            Node::Extrude {
                profile,
                distance: len(dz),
                side: crate::ExtrudeSide::Along,
            },
        )
    }

    #[test]
    fn pieces_of_two_parents_with_one_pair_of_ends_tie() {
        let doc = ProfileDoc::empty(DocumentId::derive("edge-pieces-one-line"), Tol::witness());
        let (doc, rod) = extrude(
            doc,
            0.0,
            1.5,
            LoopProgram::circle_split(0.0, 0.0, 1.0, 2, 0.0).unwrap(),
        );
        let (doc, top) = extrude(
            doc,
            1.0,
            1.0,
            LoopProgram::polygon([(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)])
                .expect("finite"),
        );
        let (doc, cut) = ins(
            doc,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a: rod,
                b: top,
                declare: Vec::new(),
            },
        );
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let value = ev.value(cut).expect("the cut evaluates");
        let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
            panic!("a body");
        };
        // The two half circles of the top rim, and a table holding only
        // the vertex rows, where their pieces are minted.
        let top_rim: Vec<topo::EdgeKey> = body
            .edges()
            .map(|(e, _)| e)
            .filter(|&e| {
                let (v0, v1) = super::edge_ends(body, e).unwrap();
                let z = |v| super::vertex_point(body, v).unwrap().z;
                v0 != v1 && (z(v0) - 1.0).abs() < 1e-9 && (z(v1) - 1.0).abs() < 1e-9
            })
            .collect();
        assert_eq!(top_rim.len(), 2, "the rim is two half circles: {top_rim:?}");
        let ends = |e| {
            let (v0, v1) = super::edge_ends(body, e).unwrap();
            let mut vs = [v0, v1];
            vs.sort();
            vs
        };
        assert_eq!(
            ends(top_rim[0]),
            ends(top_rim[1]),
            "between the same two vertices"
        );
        let mut t = NameTable::new();
        for (name, entry) in value.name_table.iter() {
            if let (EntityKind::Vertex, Entry::Unique(e)) = (name.kind, entry) {
                t.insert(name.clone(), *e).unwrap();
            }
        }
        // Two parent edges on one line `X`: `X` with two different
        // piece qualifiers, each wrapped as an operand's edge.
        let x = StableName {
            kind: EntityKind::Edge,
            node: rod,
            path: vec![RoleSeg::OutputBody],
        };
        let parent = |marker: StableName| {
            let mut p = x.clone();
            p.path
                .push(RoleSeg::Fragment(Qualifier::Ends(vec![marker])));
            StableName {
                kind: EntityKind::Edge,
                node: cut,
                path: vec![RoleSeg::FromA(NameRef::new(p))],
            }
        };
        let (p1, p2) = (
            parent(x.clone()),
            parent(StableName {
                kind: EntityKind::Vertex,
                node: rod,
                path: vec![RoleSeg::OutputBody],
            }),
        );
        let mut pieces = EdgePieces::default();
        let mut tie = TieRows::default();
        for (base, e) in [(&p1, top_rim[0]), (&p2, top_rim[1])] {
            name_edge_pieces(&mut pieces, &t, false, base, (body, 0), &[e], Lone::Piece).unwrap();
        }
        pieces.mint(&mut t, &mut tie).unwrap();
        tie.flush(&mut t).unwrap();
        let rows: Vec<&Entry> = t
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Edge)
            .map(|(_, e)| e)
            .collect();
        assert!(
            matches!(rows.as_slice(), [Entry::Tied(es)] if es.len() == 2),
            "one tied row of the two pieces: {rows:?}"
        );
    }
}

#[cfg(test)]
mod track_cover {
    //! **The flush rule along a curved carrier** ([`Track::cover`]).
    //! A closed edge's vertex is conventional and has no identity of
    //! its own (`docs/DESIGN.md`, maximal edges), so the same circle
    //! with its vertex anywhere, its candidates in either order, reads
    //! the same cover.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::f64::consts::TAU;

    use geom_core::{Band, Point3, Tol, Vec3};
    use topo::Body;

    use super::{CarrierArc, Cover, Track};
    use crate::names::discriminate::ON_MEMBER_EDGE;

    fn circle() -> geom::Curve3<f64> {
        geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        }
    }

    /// A body whose one edge is `circle()` from `t0` to `t1`: closed
    /// (`t1 = t0 + τ`, one vertex at `t0`) or an arc between two.
    fn track(t0: f64, t1: f64) -> Track<f64> {
        let tol = Tol::witness();
        let c = circle();
        let mut body = Body::<f64>::new();
        let born = body.mvfs(c.eval(t0), true).unwrap();
        let spec = geom_brep::EdgeCurveSpec::arc_of_circle(c.clone(), t0, t1).unwrap();
        let e = if (t1 - t0 - TAU).abs() < 1e-12 {
            let made = body
                .mef(
                    topo::MefSite::Lone {
                        r#loop: born.r#loop,
                    },
                    spec,
                    topo::FaceSurface::Inherit,
                    tol,
                )
                .unwrap();
            made.edge
        } else {
            body.mev(
                topo::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                c.eval(t1),
                spec,
                tol,
            )
            .unwrap()
            .edge
        };
        Track::of_edge(&body, e).unwrap()
    }

    fn arc(t0: f64, t1: f64) -> CarrierArc<f64> {
        let c = circle();
        CarrierArc {
            p0: c.eval(t0),
            mid: c.eval(0.5 * (t0 + t1)),
            p1: c.eval(t1),
            closed: (t1 - t0 - TAU).abs() < 1e-12,
        }
    }

    fn read(t: &Track<f64>, arcs: Vec<(u8, CarrierArc<f64>)>) -> (Vec<u8>, Vec<u8>, bool) {
        let band = Band::linear(Tol::witness()).unwrap();
        let Cover {
            mut within,
            mut along,
            covered,
        } = t.cover(arcs, ON_MEMBER_EDGE, band).unwrap();
        within.sort_unstable();
        along.sort_unstable();
        (within, along, covered)
    }

    /// Two arcs meeting at 1 and 4 that close the circle, one arc off
    /// it on a larger one, and the closed circle itself: read from
    /// three vertex positions (inside each arc, and at their joint),
    /// both orders.
    #[test]
    fn a_closed_edge_reads_alike_wherever_its_vertex_sits() {
        let off = CarrierArc {
            p0: Point3::new(2.0, 0.0, 0.0),
            mid: Point3::new(0.0, 2.0, 0.0),
            p1: Point3::new(-2.0, 0.0, 0.0),
            closed: false,
        };
        for vertex in [2.0, 5.5, 1.0, 4.0] {
            let t = track(vertex, vertex + TAU);
            let forward = vec![
                (0, arc(1.0, 4.0)),
                (1, arc(4.0, 1.0 + TAU)),
                (2, off.clone()),
            ];
            let backward = vec![
                (2, off.clone()),
                (1, arc(4.0, 1.0 + TAU)),
                (0, arc(1.0, 4.0)),
            ];
            for arcs in [forward, backward] {
                assert_eq!(
                    read(&t, arcs),
                    (vec![], vec![0, 1], true),
                    "vertex at {vertex}"
                );
            }
            assert_eq!(read(&t, vec![(0, arc(1.0, 4.0))]), (vec![], vec![0], false));
            assert_eq!(
                read(&t, vec![(0, arc(1.0, 4.0)), (3, arc(0.0, TAU))]),
                (vec![3], vec![0], true),
                "a closed edge lies within the closed rim alone"
            );
        }
    }

    /// An open arc lies within an arc that holds it, whichever way that
    /// arc runs, also across the carrier's parameter seam.
    #[test]
    fn an_open_arc_lies_within_the_arc_that_holds_it() {
        let t = track(5.0, 7.0);
        assert_eq!(read(&t, vec![(0, arc(4.0, 7.5))]), (vec![0], vec![], true));
        assert_eq!(read(&t, vec![(0, arc(7.5, 4.0))]), (vec![0], vec![], true));
        assert_eq!(
            read(&t, vec![(0, arc(4.0, 6.0)), (1, arc(6.0, 8.0))]),
            (vec![], vec![0, 1], true)
        );
        assert_eq!(read(&t, vec![(0, arc(2.0, 3.0))]), (vec![], vec![], false));
    }
}

/// **The boolean's rows at a vertex read twice name the result**: a
/// pyramid standing on its apex at a point of a plate's top that another
/// body's own contact holds, beside pyramids standing there, voids
/// hanging from it, islands in the voids and voids in the pyramids, or
/// two pyramids united at their apexes there, in every op and both
/// orders at every pose, name through [`name_boolean`] with no emission
/// refusal. Two shapes have no emitter rule, as on main (`no_rule`). `crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`
/// reads the same scenes' rows against their germs.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod touch_reread_rows {
    use super::{NamingError, OperandCtx, name_boolean};
    use crate::names::emit::{ent, name1};
    use crate::names::role::{CapEnd, EntityKind, RoleSeg};
    use crate::names::table::{EntityKey, NameTable};
    use crate::names::{PieceRole, PieceRun, ProfileEdgeRef, ProfileVertexRef};
    use crate::node::{RecipeNodeId, StepId};
    use geom_core::Tol;
    use topo::test_support::meeting::{
        PLATE, Pose, apex_pyramid, bearing, corners, mix, nest, nest_polygon, posed_box, poses,
    };
    use topo::{AtRestBody, BooleanResult, intersect, subtract, union};

    fn t() -> Tol {
        Tol::witness()
    }

    fn built(r: Result<BooleanResult<f64>, topo::BooleanError>) -> AtRestBody<f64> {
        match r {
            Ok(BooleanResult::Body(r)) => r.body,
            other => panic!("{:?}", other.map(|_| ())),
        }
    }

    /// A name for every entity of `body`, each its own: the rows read
    /// operand edges by name, not by role.
    fn table(body: &AtRestBody<f64>, node: RecipeNodeId) -> NameTable {
        let piece = |k: usize| ProfileEdgeRef::Piece {
            step: StepId::new(0, k as u64),
            role: PieceRole::Leg,
        };
        let at = |k: usize| ProfileVertexRef::Piece {
            step: StepId::new(0, k as u64),
            role: PieceRole::Leg,
        };
        let mut t = NameTable::new();
        t.insert(
            name1(EntityKind::Body, node, RoleSeg::OutputBody),
            ent(0, EntityKey::Body),
        )
        .unwrap();
        for (k, (f, _)) in body.faces().enumerate() {
            let name = name1(
                EntityKind::Face,
                node,
                RoleSeg::Lateral(PieceRun::one(piece(k))),
            );
            t.insert(name, ent(0, EntityKey::Face(f))).unwrap();
        }
        for (k, (e, _)) in body.edges().enumerate() {
            let name = name1(EntityKind::Edge, node, RoleSeg::LateralEdge(at(k)));
            t.insert(name, ent(0, EntityKey::Edge(e))).unwrap();
        }
        for (k, (v, _)) in body.vertices().enumerate() {
            let name = name1(
                EntityKind::Vertex,
                node,
                RoleSeg::CapVertex(CapEnd::End, at(k)),
            );
            t.insert(name, ent(0, EntityKey::Vertex(v))).unwrap();
        }
        t
    }

    /// The cells the emitter has no rule for, each named, as on main:
    /// - `y ∩ x` where the pyramid crosses into a void in an arch, a
    ///   seam vertex no rule parents in that order
    ///   (`work/emit/an-intersection-into-a-void-at-a-vertex-has-no-seam-vertex-rule-in-one-order.md`);
    /// - a union in either order over a quadrilateral void in a
    ///   quadrilateral arch, a seam vertex whose parentage its incident
    ///   edges leave underdetermined
    ///   (`work/wire/a-legal-declared-union-reaches-the-seam-vertex-parentage-residue-emission.md`).
    fn no_rule(label: &str, what: &str, e: &NamingError) -> bool {
        const SEAM: [&str; 5] = [
            "over a void in the bare arch",
            "over the void in the arch",
            "over the void, the island in it",
            "over a quad void in a quad arch",
            "over a quad void in a bare quad arch",
        ];
        const QUADS: [&str; 2] = [
            "over a quad void in a quad arch",
            "over a quad void in a bare quad arch",
        ];
        match e {
            NamingError::SeamVertexParentage { .. } => what == "y ∩ x" && SEAM.contains(&label),
            NamingError::Emission { what: why } => {
                (what == "x ∪ y" || what == "y ∪ x")
                    && QUADS.contains(&label)
                    && why.starts_with("seam vertex parentage underdetermined")
            }
            _ => false,
        }
    }

    /// Names every op on `(x, y)` in both orders; returns how many built.
    fn names(label: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>, pose: &Pose) -> usize {
        let (xn, yn) = (RecipeNodeId::new(0, 1), RecipeNodeId::new(0, 2));
        let (xt, yt) = (table(x, xn), table(y, yn));
        let mut named = 0;
        for (what, (a, an, at), (b, bn, bt), r) in [
            ("x − y", (x, xn, &xt), (y, yn, &yt), subtract(x, y, t())),
            ("y − x", (y, yn, &yt), (x, xn, &xt), subtract(y, x, t())),
            ("x ∪ y", (x, xn, &xt), (y, yn, &yt), union(x, y, t())),
            ("y ∪ x", (y, yn, &yt), (x, xn, &xt), union(y, x, t())),
            ("x ∩ y", (x, xn, &xt), (y, yn, &yt), intersect(x, y, t())),
            ("y ∩ x", (y, yn, &yt), (x, xn, &xt), intersect(y, x, t())),
        ] {
            let Ok(BooleanResult::Body(r)) = r else {
                continue;
            };
            let a = OperandCtx {
                node: an,
                table: at,
                body: a,
            };
            let b = OperandCtx {
                node: bn,
                table: bt,
                body: b,
            };
            match name_boolean(RecipeNodeId::new(0, 9), &r.body, &r.naming, &a, &b, t()) {
                Ok(_) => named += 1,
                Err(e) if no_rule(label, what, &e) => {}
                Err(e) => panic!("{label}, {}, {what}: names, got {e:?}", pose.label),
            }
        }
        named
    }

    /// At rest, every scene.
    #[test]
    fn a_vertex_read_twice_names_its_result_in_every_op() {
        assert!(every_scene(&poses()[..1]) > 0, "cells built");
    }

    /// The other poses, every scene.
    #[test]
    fn a_vertex_read_twice_names_its_result_in_every_op_at_every_pose() {
        assert!(every_scene(&poses()[1..]) > 0, "cells built");
    }

    fn every_scene(poses: &[Pose]) -> usize {
        let mut named = 0;
        for pose in poses {
            let p = |b: [[f64; 3]; 3]| apex_pyramid(&b, pose, t());
            let plate = posed_box("the plate", PLATE, pose, t());
            let arch_base = corners(60.0, 0.5, 0.4);
            let void = corners(120.0, -0.5, 0.4);
            let arch = p(arch_base);
            let one = built(union(&plate, &arch, t()));
            let both = built(subtract(&one, &p(void), t()));
            let cavity = built(subtract(&plate, &p(void), t()));
            let isle = nest(void, 0.7);
            let island = built(union(&cavity, &p(isle), t()));
            let hvoid = nest(arch_base, 0.7);
            let hollow = built(subtract(&one, &p(hvoid), t()));
            let bare_hollow = built(subtract(&arch, &p(hvoid), t()));
            let deep = built(union(&hollow, &p(nest(hvoid, 0.7)), t()));
            let arches = built(union(
                &plate,
                &[180.0, 300.0].iter().fold(arch.clone(), |u, &b| {
                    built(union(&u, &p(corners(b, 0.5, 0.4)), t()))
                }),
                t(),
            ));
            let block = posed_box("a block", [(1.0, 2.0), (0.5, 1.5), (0.3, 1.5)], pose, t());
            let pentagon: Vec<[f64; 3]> = (0..5)
                .map(|k| bearing(120.0 + 72.0 * f64::from(k), 0.3, -0.45))
                .collect();
            let pentagonal = built(subtract(&block, &apex_pyramid(&pentagon, pose, t()), t()));
            let quad = [
                bearing(40.0, 0.45, 0.5),
                bearing(80.0, 0.45, 0.5),
                bearing(80.0, 0.25, 0.5),
                bearing(40.0, 0.25, 0.5),
            ];
            let quad_void = nest_polygon(&quad, 0.7);
            let quad_arch = apex_pyramid(&quad, pose, t());
            let quad_hollow = built(subtract(
                &built(union(&plate, &quad_arch, t())),
                &apex_pyramid(&quad_void, pose, t()),
                t(),
            ));
            let bare_quad_hollow = built(subtract(
                &quad_arch,
                &apex_pyramid(&quad_void, pose, t()),
                t(),
            ));
            let cone = p(corners(240.0, 0.7, 0.5));
            let over = p(corners(50.0, 0.7, 0.5));
            let over_180 = p(corners(170.0, 0.7, 0.5));
            let over_300 = p(corners(290.0, 0.7, 0.5));
            let hang = p(corners(240.0, -0.6, 0.5));
            let hang_over = p(corners(130.0, -0.6, 0.5));
            let in_void = p(nest(void, 1.4));
            let in_island = p(nest(isle, 0.7));
            let in_hollow = p(nest(hvoid, 0.7));
            let pair = |x, y| built(union(&p(x), &p(y), t()));
            let two_up = pair(corners(40.0, 0.6, 0.5), corners(280.0, 0.6, 0.5));
            let two_down = pair(corners(200.0, -0.6, 0.5), corners(110.0, -0.6, 0.5));
            let third = 1.0 / 3.0;
            let cross3 = p(mix(
                arch_base,
                [[third, third, third], [0.75, 0.15, 0.1], [0.45, 0.22, 0.33]],
                0.6,
            ));
            let on2 = p(mix(
                arch_base,
                [[0.4, 0.4, 0.2], [0.45, 0.45, 0.1], [0.7, 0.25, 0.05]],
                0.6,
            ));
            let cross_in = p(mix(
                void,
                [[third, third, third], [0.7, 0.2, 0.1], [1.2, -0.3, 0.1]],
                0.6,
            ));
            for (label, x, y) in [
                ("the arches", &cone, &arches),
                ("one standing pyramid", &cone, &one),
                ("over the arch", &over, &one),
                ("over the arches", &over, &arches),
                ("over the second arch", &over_180, &arches),
                ("over the third arch", &over_300, &arches),
                ("over the arch and void", &over, &both),
                ("hanging below the cavity", &hang, &cavity),
                ("hanging across the cavity", &hang_over, &cavity),
                ("hanging into the void", &in_void, &cavity),
                ("hanging across the arch and void", &hang_over, &both),
                ("two up, one over the arch", &two_up, &one),
                ("two up over the arch and void", &two_up, &both),
                ("two down beside the void", &two_down, &both),
                ("two up over the bare arch", &two_up, &arch),
                ("in the island in the void", &in_island, &island),
                ("in the void in the arch", &in_hollow, &hollow),
                ("over a void in the bare arch", &over, &bare_hollow),
                ("two up over a void in the bare arch", &two_up, &bare_hollow),
                ("over the void in the arch", &over, &hollow),
                ("crossing the void in the arch", &cross3, &hollow),
                ("on the void's face in the arch", &on2, &hollow),
                ("crossing the island in the void", &cross_in, &island),
                ("beside the void in the arch", &cone, &hollow),
                ("crossing three levels", &cross3, &deep),
                ("over the void, the island in it", &over, &deep),
                ("hanging into a pentagonal void", &hang, &pentagonal),
                ("hanging across a pentagonal void", &hang_over, &pentagonal),
                ("over a quad void in a quad arch", &over, &quad_hollow),
                (
                    "over a quad void in a bare quad arch",
                    &over,
                    &bare_quad_hollow,
                ),
            ] {
                named += names(label, x, y, pose);
            }
        }
        named
    }
}

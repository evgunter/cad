//! Split/boolean name emission (spec D2/D3): descent-driven — every
//! result entity is chased to its operand parent through the kernels'
//! mint-time rows (`SplitNaming`, `BooleanNaming`, D5 `SplitEdge`
//! provenance), then named as pass-through, `From` (keyed by the seat's read),
//! fragment (with N2 qualifiers), seam, section, or merged. Nothing
//! is matched; unresolvable descent is a typed error.

use core::convert::Infallible;
use core::ops::ControlFlow;
use std::collections::{BTreeMap, BTreeSet};

use geom_core::k_stats::decide;
use geom_core::{Decide, Margin, Point3, Sign, Vec3};
use topo::splitting::{PlaneSide, SplitNaming};
use topo::{Body, EdgeKey, FaceKey, Provenance, VertexKey};

use super::borders::Obstacles;
use super::canonical;
use super::chase;
use super::defer::{TieRows, Upstream, mint_candidates, put, upstream_name};
use super::discriminate::{CHORD_ON_RIM, ON_MEMBER_EDGE, band};
use super::emit::{
    CrossingSensesShape, DisagreeShape, Incidence, JoinShape, NamingError, Rim, RimShare,
    SeamSidesShape, SeamVertexShape, WayShape, edge_ends, ent, face_half_edges, name1, rim_between,
    rims_between, vertex_point,
};
use super::groups::{Emitted, GroupRecord, Parent};
use super::merged::{self, NESTED_MERGED};
use super::plane_pieces;
use super::role::{
    EntityKind, NameRef, Qualifier, RoleSeg, RunLine, Sense, SplitHalf, StableName, edge_line,
};
use super::seam_pair;
use super::table::{EntityKey, EntityRef, Entry, NameTable};
use crate::node::RecipeNodeId;
use geom_core::Tol;

/// One split-side body under naming.
struct Side<'a, T: Decide> {
    body: &'a Body<T>,
    ix: u32,
    half: SplitHalf,
}

impl<T: Decide> Side<'_, T> {
    /// The half's place among [`HalfNames`]: its output's port.
    fn slot(&self) -> usize {
        usize::try_from(self.ix).unwrap_or(usize::MAX)
    }
}

/// **One half's names in the making** (NAMES N1, the scope): its own
/// table, the ties it defers and the groups it formed. A name is scoped
/// by the variable that holds its body, so each half is named apart and
/// no name says which half it is in; every entity is body 0 of its
/// half's table.
#[derive(Default)]
struct HalfNames {
    t: NameTable,
    tie: TieRows,
    rec: GroupRecord,
}

/// Chases an edge through `SplitEdge` birth records
/// (`Body::split_root`), STOPPING at the first key the operand's table
/// names: the table is the identity boundary — records deeper than the
/// operand's own entities belong to earlier ops (a union's rim fragment
/// must not chase past its own union-level name into its grand-parent).
///
/// Where the lineage stops at a key the table does not name and a kill
/// took ([`KilledSplits`]), the walk steps to that key's parent and goes
/// on: the boolean's kills drop birth records, and `killed` holds the
/// hops they dropped.
///
/// # Errors
///
/// [`NamingError::SplitLineage`], carrying the edge, where
/// `Body::split_root` refuses a cycling lineage.
///
/// # Panics
///
/// Where the kills' hops cycle ([`chase::to_root`]). `killed` copies
/// `SplitEdge` records, and a copy cycles only where the records do
/// (`topo::Provenance::SplitEdge`); the boolean that wrote them is the
/// only source of the rows this emitter reads.
fn chase_edge_to_table<T: Decide>(
    body: &Body<T>,
    table: &NameTable,
    killed: &BTreeMap<EdgeKey, EdgeKey>,
    e: EdgeKey,
) -> Result<EdgeKey, NamingError> {
    chase_edge_until(
        body,
        killed,
        |k| table.name_of(&ent(0, EntityKey::Edge(k))).is_some(),
        e,
    )
}

/// [`chase_edge_to_table`]'s walk, stopping at the first key `stop`
/// holds of instead of the first a table names.
///
/// # Panics
///
/// As [`chase_edge_to_table`].
fn chase_edge_until<T: geom_core::Real>(
    body: &Body<T>,
    killed: &BTreeMap<EdgeKey, EdgeKey>,
    stop: impl Fn(EdgeKey) -> bool,
    e: EdgeKey,
) -> Result<EdgeKey, NamingError> {
    chase::to_root(
        e,
        killed.len(),
        "see `emit_topo::chase_edge_to_table`'s Panics",
        |k| {
            let k = body.split_root(k, &stop)?;
            Ok(match killed.get(&k) {
                Some(&up) if !stop(k) => ControlFlow::Continue(up),
                _ => ControlFlow::Break(k),
            })
        },
    )
}

/// **The split hops a boolean's kills took out of the provenance**:
/// edge → the edge it was split from, for each edge that bounded a
/// face the boolean discarded and died with it, and each edge its
/// output stage merged or joined away. A kill drops the edge's birth
/// record, so a live piece split off a killed one has a lineage that
/// stops at the killed key; the discard rows keep each such edge's
/// chain ([`topo::DiscardRow::boundary_chains`]) and the output stage
/// its hop (`topo::BooleanNaming::killed_lineage`), and these are its
/// hops, with those of every edge an operand's carve killed
/// (`topo::BooleanNaming::carved_lineage`). `arena` holds the hops in
/// result keys (the output stage's, and an operand's whose clone is the
/// result arena); `graft_b` a grafted B's, in B's clone keys.
struct KilledSplits {
    arena: BTreeMap<EdgeKey, EdgeKey>,
    graft_b: BTreeMap<EdgeKey, EdgeKey>,
}

impl KilledSplits {
    fn of(naming: &topo::BooleanNaming) -> Self {
        use topo::Operand;
        let mut out = Self {
            arena: BTreeMap::new(),
            graft_b: BTreeMap::new(),
        };
        for row in &naming.discards {
            if let Some(hops) = out.space(naming, row.operand) {
                for chain in &row.boundary_chains {
                    for w in chain.windows(2) {
                        hops.insert(w[0], w[1]);
                    }
                }
            }
        }
        for (operand, carved) in [Operand::A, Operand::B]
            .into_iter()
            .zip(&naming.carved_lineage)
        {
            if let Some(hops) = out.space(naming, operand) {
                hops.extend(carved);
            }
        }
        out.arena.extend(&naming.killed_lineage);
        out
    }

    /// The hops of `operand`'s clone keys: the arena's where its clone
    /// is the result arena, `graft_b` for a grafted B; `None` for an
    /// absent operand.
    fn space(
        &mut self,
        naming: &topo::BooleanNaming,
        operand: topo::Operand,
    ) -> Option<&mut BTreeMap<EdgeKey, EdgeKey>> {
        use topo::{Operand, OperandKeys};
        match (operand, naming.a_keys, naming.b_keys) {
            (Operand::A, OperandKeys::Direct, _) | (Operand::B, _, OperandKeys::Direct) => {
                Some(&mut self.arena)
            }
            (Operand::B, _, OperandKeys::Grafted) => Some(&mut self.graft_b),
            _ => None,
        }
    }
}

/// [`chase_edge_to_table`] over a split's two halves. The halves are
/// carved from one scratch arena, so a key resolves in whichever half
/// kept the entity and only there (`SplitNaming`), and so does its
/// `SplitEdge` record: each hop reads the record from the half holding
/// that key. An operand edge the plane crosses twice has its middle
/// piece on one side and the outer two on the other, so an outer
/// piece's lineage can run through a key its own half does not hold.
///
/// # Panics
///
/// As [`chase_split_lineage`].
fn chase_split_edge_to_table<T: Decide>(
    sides: &[Side<'_, T>],
    table: &NameTable,
    e: EdgeKey,
) -> EdgeKey {
    chase_split_lineage(sides, table, &BTreeMap::new(), e)
}

/// [`chase_split_edge_to_table`] through `lineage` too: the split
/// records an edge its join killed with the edge it was split from
/// (`SplitNaming::joined_lineage`), since the kill took the birth
/// record the halves would otherwise hold.
///
/// # Panics
///
/// Where the lineage cycles ([`chase::to_root`]): the halves' records
/// are restrictions of one body's records, and `lineage` copies records
/// its joins killed, so the walk cycles only where the records do
/// (`topo::Provenance::SplitEdge`).
fn chase_split_lineage<T: Decide>(
    sides: &[Side<'_, T>],
    table: &NameTable,
    lineage: &BTreeMap<EdgeKey, EdgeKey>,
    e: EdgeKey,
) -> EdgeKey {
    let bound: usize = sides.iter().map(|s| s.body.edges().count()).sum::<usize>() + lineage.len();
    let Ok(root) = chase::to_root::<_, Infallible>(
        e,
        bound,
        "see `emit_topo::chase_split_lineage`'s Panics",
        |k| {
            if table.name_of(&ent(0, EntityKey::Edge(k))).is_some() {
                return Ok(ControlFlow::Break(k));
            }
            let mut held = sides.iter().filter_map(|s| s.body.edge_provenance_of(k));
            let record = held.next();
            debug_assert!(
                held.all(|other| Some(other) == record),
                "split halves disagree on edge {k:?}'s birth record"
            );
            Ok(match (record, lineage.get(&k)) {
                (Some(Provenance::SplitEdge { edge }), _) | (None, Some(edge)) => {
                    ControlFlow::Continue(*edge)
                }
                _ => ControlFlow::Break(k),
            })
        },
    );
    root
}

/// Names both sides of a split (spec D2's split vocabulary + N2), each
/// in its own table, in port order (`SplitHalf::output_body`): a half
/// with no material has an empty one.
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
) -> Result<[Emitted; 2], NamingError> {
    let b = band(tol)?;
    let mut halves: [HalfNames; 2] = Default::default();
    let frag_rows = &naming.face_fragments;
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
        halves[s.slot()].t.insert(
            name1(EntityKind::Body, node, RoleSeg::SplitBody),
            ent(0, EntityKey::Body),
        )?;
    }

    for &(f, side) in &naming.sections {
        let half = half_of(side, "section row", &f);
        let absorbed = naming
            .merge_groups
            .iter()
            .any(|(gs, g)| *gs == side && g.absorbed.contains(&f));
        if !sides
            .iter()
            .any(|s| s.half == half && (absorbed || s.body.get_face(f).is_some()))
        {
            unreachable!(
                "section face {f:?} is live in no {half:?} half and absorbed by no merge there: \
                 `SplitNaming::sections` lists the faces each side keeps, with the side its body \
                 was made on, and a merge that absorbs one keeps its row"
            );
        }
    }
    // Keys are unique across the two halves (restrictions of one
    // arena), and each side's joins keep their order, so one chase
    // over both sides' rows is each side's.
    let joins: Vec<topo::EdgeJoin> = naming.edge_joins.iter().map(|&(_, j)| j).collect();
    let covers = topo::join_covers(&joins);
    let keeps = KeepsReading {
        sides: &sides,
        target_node,
        target_table,
        lineage: &naming.joined_lineage,
        covers: &covers,
    };
    let mut merged = name_split_merges(node, &mut halves, &sides, naming, &keeps, target_body)?;
    name_split_planes(&mut halves, &sides, naming, &mut merged, &keeps)?;
    name_split_faces(
        node,
        &mut halves,
        &sides,
        frag_rows,
        &section_keys,
        &merged,
        &keeps,
        target_body,
    )?;
    for h in &mut halves {
        h.tie.flush(&mut h.t)?;
    }
    name_split_edges_vertices(
        node,
        &mut halves,
        &sides,
        naming,
        &merged,
        &joins,
        &covers,
        &keeps,
        target_body,
        b,
    )?;
    for h in &mut halves {
        h.tie.flush(&mut h.t)?;
    }

    for s in &sides {
        super::emit::check_total(&halves[s.slot()].t, s.body, 0)?;
    }
    let [above, below] = halves;
    Ok([above, below].map(|h| Emitted::new(h.t, h.rec)))
}

/// The half a split's record names by its side; `row` is the record's
/// entry, for the panic.
///
/// # Panics
///
/// Where `record` names the side `On`: `topo::splitting` writes each
/// section, join and merge row with the side whose body it was made
/// on, `Above` or `Below`.
#[track_caller]
fn half_of(side: PlaneSide, record: &'static str, row: &dyn core::fmt::Debug) -> SplitHalf {
    match side {
        PlaneSide::Above => SplitHalf::Above,
        PlaneSide::Below => SplitHalf::Below,
        PlaneSide::On => unreachable!(
            "{record} {row:?} names the side On: a split records each section, join and merge \
             with the side whose body it was made on, Above or Below"
        ),
    }
}

/// The parent of faces a split's merge linked ([`SplitNaming::merge_groups`]),
/// or of the plane's pieces: its `Merged` name, whether it descends from
/// a tie, and the operand faces among its constituents.
struct MergedFace {
    name: NameRef,
    tied: bool,
    operand_faces: BTreeSet<FaceKey>,
}

/// What [`name_split_merges`] named: each face a merge kept, by its
/// parent (`kept`), and every face of a merged parent, kept or a piece
/// that never merged (`held`); and in each half, the parent that holds
/// the plane (`plane`), whose faces [`name_split_planes`] names.
#[derive(Default)]
struct SplitMerges {
    kept: BTreeMap<FaceKey, MergedFace>,
    held: BTreeMap<FaceKey, MergedFace>,
    plane: [Option<PlaneParent>; 2],
}

/// A half's parent that holds the tool plane: its name, whether it
/// descends from a tie, and its faces in the half.
struct PlaneParent {
    name: StableName,
    tied: bool,
    members: Vec<FaceKey>,
}

impl SplitMerges {
    /// The parent `face` is cited by, as a seam cites the face beside it
    /// (N3): a merged parent's `Merged` set, else the operand face it
    /// descends from; and whether that descends from a tie.
    fn parent_of(
        &self,
        face: FaceKey,
        naming: &SplitNaming,
        target_node: RecipeNodeId,
        target_table: &NameTable,
    ) -> Result<(NameRef, bool), NamingError> {
        if let Some(held) = self.held.get(&face) {
            return Ok((held.name.clone(), held.tied));
        }
        let root = naming.face_fragments.root(face);
        let up = upstream_name(target_table, target_node, ent(0, EntityKey::Face(root)))?;
        Ok((up.name, up.tied))
    }

    /// The operand faces the parent [`Self::parent_of`] names is: the
    /// one `face` descends from, or a merged parent's set.
    fn parent_faces(&self, face: FaceKey, naming: &SplitNaming) -> BTreeSet<EntityKey> {
        match self.held.get(&face) {
            Some(held) => held
                .operand_faces
                .iter()
                .map(|&f| EntityKey::Face(f))
                .collect(),
            None => BTreeSet::from([EntityKey::Face(naming.face_fragments.root(face))]),
        }
    }
}

/// **The parents a group of pieces descends from, each as the operand
/// entities it is** (N2: a parent is a set of entities, never a name).
/// Every pass that groups pieces by the name they are spelled from keys
/// the group by these beside the name: tied parents spelled alike stay
/// apart, so each candidate's pieces are named among themselves and an
/// undivided one keeps the bare name (N4). A merged parent is the set of
/// its operand entities, which no candidate number spells.
pub(super) type ParentEntities<K> = BTreeSet<BTreeSet<K>>;

/// The tool plane, as a split `node` names it: the bare `SectionFace`.
fn plane_name(node: RecipeNodeId) -> StableName {
    name1(EntityKind::Face, node, RoleSeg::SectionFace)
}

/// The kernel's side for a half.
fn plane_side(half: SplitHalf) -> PlaneSide {
    match half {
        SplitHalf::Above => PlaneSide::Above,
        SplitHalf::Below => PlaneSide::Below,
    }
}

/// `Merged` of `names`' constituents, flat and in name order, and
/// whether any descends from a tie.
///
/// # Errors
///
/// The upstream table's refusals.
///
/// # Panics
///
/// Where a constituent would nest: an upstream table holds merged
/// names flat, since every emitter mints them flat and panics here on
/// any other, and the loop reads each operand face's set through its
/// wrappers.
fn merged_of(
    node: RecipeNodeId,
    mut constituents: Vec<StableName>,
    ops: &BTreeSet<FaceKey>,
    target_node: RecipeNodeId,
    target_table: &NameTable,
) -> Result<(StableName, bool, Vec<NameRef>), NamingError> {
    let mut tied = false;
    let mut op_names = Vec::with_capacity(ops.len());
    for &op in ops {
        let up = upstream_name(target_table, target_node, ent(0, EntityKey::Face(op)))?;
        tied |= up.tied;
        match merged::constituents_through_wrappers(&up.name) {
            Some(cs) => constituents.extend(cs),
            None => constituents.push((*up.name).clone()),
        }
        op_names.push(up.name);
    }
    if let Some(nested) = constituents
        .iter()
        .find(|c| merged::constituents_through_wrappers(c).is_some())
    {
        unreachable!(
            "{NESTED_MERGED} ({nested:?}, in split {node:?}'s merge of {ops:?}): an upstream \
             table holds merged names flat, as every emitter mints them, and this merge read \
             each operand face's set through its wrappers"
        );
    }
    let name = canonical::form(name1(EntityKind::Face, node, RoleSeg::Merged(constituents)));
    Ok((name, tied, op_names))
}

/// Names the faces of each split half's merged parents as a boolean
/// names its merged faces (N3). In each half, the faces one merge group
/// holds are linked, transitively through the operand faces they
/// descend from; a linked set is one parent, `Merged` of its operand
/// faces' names, flat and in name order. Every face of that half
/// descending from the parent, a merge's kept face or a piece of one of
/// its operand faces, is a face of it: one is the parent's bare name,
/// several are qualified by `Keeps` over the boundary edges of the
/// parent's operand faces (N2).
///
/// The plane links the groups that hold a section as an operand face
/// links its pieces (`SplitNaming::plane_pieces`): that parent is
/// `Merged` of the plane, by the bare `SectionFace` once, and the
/// operand faces it absorbed, or the bare plane where it absorbed none.
/// Its faces are every face of the half holding part of the plane, and
/// [`name_split_planes`] names them.
///
/// # Errors
///
/// The upstream table's and the name table's refusals.
///
/// # Panics
///
/// Where a merged name would nest ([`merged_of`]), or a merge's kept
/// face is live in no half its record names.
fn name_split_merges<T: Decide>(
    node: RecipeNodeId,
    halves: &mut [HalfNames; 2],
    sides: &[Side<'_, T>],
    naming: &SplitNaming,
    keeps: &KeepsReading<'_, '_, T>,
    target_body: &Body<T>,
) -> Result<SplitMerges, NamingError> {
    let (target_node, target_table) = (keeps.target_node, keeps.target_table);
    // A linked set: its operand faces, its kept faces.
    type Linked = (BTreeSet<FaceKey>, Vec<FaceKey>);
    // A named parent: its name, whether it descends from a tie, its
    // operand faces, its kept faces and its faces in the half.
    type Named = (
        StableName,
        bool,
        BTreeSet<FaceKey>,
        Vec<FaceKey>,
        Vec<FaceKey>,
    );
    let frag_rows = &naming.face_fragments;
    let mut out = SplitMerges::default();
    for s in sides {
        let plane = naming.plane_pieces(plane_side(s.half), s.body);
        let mut linked: Vec<Linked> = Vec::new();
        for (side, g) in &naming.merge_groups {
            if half_of(*side, "split merge row", g) != s.half {
                continue;
            }
            if s.body.get_face(g.kept).is_none() {
                unreachable!(
                    "a split merge's kept face {:?} is not live in the {:?} half its row names: \
                     `SplitNaming::merge_groups` records each group with the side whose body the \
                     merge ran on, and the kept face holds the group there",
                    g.kept, s.half
                );
            }
            if plane.kept.contains(&g.kept) {
                continue;
            }
            let mut set: Linked = (BTreeSet::new(), vec![g.kept]);
            for &c in core::iter::once(&g.kept).chain(&g.absorbed) {
                set.0.insert(frag_rows.root(c));
            }
            let (meets, rest): (Vec<Linked>, Vec<Linked>) =
                linked.into_iter().partition(|l| !l.0.is_disjoint(&set.0));
            for (ops, kept) in meets {
                set.0.extend(ops);
                set.1.extend(kept);
            }
            linked = rest;
            linked.push(set);
        }
        let mut parents: Vec<Named> = Vec::new();
        for (ops, kept) in linked {
            let (name, tied, op_names) =
                merged_of(node, Vec::new(), &ops, target_node, target_table)?;
            let held_by = |f: FaceKey| {
                kept.contains(&f)
                    || (!naming.holds_section(f)
                        && !naming.merge_groups.iter().any(|(_, g)| g.kept == f)
                        && ops.contains(&frag_rows.root(f)))
            };
            let members: Vec<FaceKey> = s
                .body
                .faces()
                .map(|(f, _)| f)
                .filter(|&f| held_by(f))
                .collect();
            let h = &mut halves[s.slot()];
            let ents: Vec<_> = members
                .iter()
                .map(|&f| ent(0, EntityKey::Face(f)))
                .collect();
            h.rec.record(&name, ents.clone(), Parent::Elsewhere);
            for op_name in &op_names {
                h.rec.record(
                    &split_base(node, op_name.clone()),
                    ents.clone(),
                    Parent::Elsewhere,
                );
            }
            if let [f] = members.as_slice() {
                put(
                    &mut h.t,
                    &mut h.tie,
                    tied,
                    name.clone(),
                    ent(0, EntityKey::Face(*f)),
                )?;
            } else {
                let boundary = operand_boundary(target_body, ops.iter().copied());
                let mut pieces = Vec::with_capacity(members.len());
                for &f in &members {
                    let edges = face_edges(s.body, f);
                    pieces.push((keeps.of(edges, &boundary)?, ent(0, EntityKey::Face(f))));
                }
                mint_qualified(&mut h.t, &mut h.tie, tied, &name, pieces)?;
            }
            parents.push((name, tied, ops, kept, members));
        }
        if !plane.faces.is_empty() {
            let (name, tied, op_names) = if plane.roots.is_empty() {
                (plane_name(node), false, Vec::new())
            } else {
                merged_of(
                    node,
                    vec![plane_name(node)],
                    &plane.roots,
                    target_node,
                    target_table,
                )?
            };
            let h = &mut halves[s.slot()];
            let ents: Vec<_> = plane
                .faces
                .iter()
                .map(|&f| ent(0, EntityKey::Face(f)))
                .collect();
            h.rec
                .record(&plane_name(node), ents.clone(), Parent::Elsewhere);
            if !plane.roots.is_empty() {
                h.rec.record(&name, ents.clone(), Parent::Elsewhere);
            }
            for op_name in &op_names {
                h.rec.record(
                    &split_base(node, op_name.clone()),
                    ents.clone(),
                    Parent::Elsewhere,
                );
            }
            if !plane.kept.is_empty() {
                parents.push((
                    name.clone(),
                    tied,
                    plane.roots.clone(),
                    plane.kept.iter().copied().collect(),
                    plane.faces.clone(),
                ));
            }
            out.plane[s.slot()] = Some(PlaneParent {
                name,
                tied,
                members: plane.faces,
            });
        }
        for (name, tied, ops, kept, members) in parents {
            let name = NameRef::new(name);
            for &f in &members {
                out.held.insert(
                    f,
                    MergedFace {
                        name: name.clone(),
                        tied,
                        operand_faces: ops.clone(),
                    },
                );
            }
            for f in kept {
                out.kept.insert(
                    f,
                    MergedFace {
                        name: name.clone(),
                        tied,
                        operand_faces: ops.clone(),
                    },
                );
            }
        }
    }
    Ok(out)
}

/// Names each split half's pieces of the tool plane (N2, a Split's
/// section faces), the faces of the parent [`name_split_merges`] left in
/// `merged.plane`. One piece is the parent's bare name; several are each
/// qualified by `Borders` over their walls
/// ([`plane_pieces::plane_walls`]), each wall cited by its parent as a
/// section chord cites the face it crosses, and a piece descends from a
/// tie where its parent or one of its walls does.
///
/// # Errors
///
/// The walls' parents' reading, and the table's refusals.
fn name_split_planes<T: Decide>(
    halves: &mut [HalfNames; 2],
    sides: &[Side<'_, T>],
    naming: &SplitNaming,
    merged: &mut SplitMerges,
    keeps: &KeepsReading<'_, '_, T>,
) -> Result<(), NamingError> {
    let (target_node, target_table) = (keeps.target_node, keeps.target_table);
    for s in sides {
        let slot = s.slot();
        let Some(parent) = merged.plane[slot].take() else {
            continue;
        };
        let h = &mut halves[slot];
        match parent.members.as_slice() {
            [] => {}
            [one] => put(
                &mut h.t,
                &mut h.tie,
                parent.tied,
                parent.name,
                ent(0, EntityKey::Face(*one)),
            )?,
            pieces => {
                let mut tied_walls: BTreeSet<StableName> = BTreeSet::new();
                let walls = plane_pieces::plane_walls(s.body, pieces, &naming.islands, |across| {
                    let (name, t) = merged.parent_of(across, naming, target_node, target_table)?;
                    let name = (*name).clone();
                    if t {
                        tied_walls.insert(name.clone());
                    }
                    Ok(name)
                })?;
                let mut by_walls: BTreeMap<Vec<StableName>, Vec<FaceKey>> = BTreeMap::new();
                for (set, &f) in walls.into_iter().zip(pieces) {
                    by_walls
                        .entry(set.into_iter().collect())
                        .or_default()
                        .push(f);
                }
                for (walls, faces) in by_walls {
                    let tied = parent.tied || walls.iter().any(|w| tied_walls.contains(w));
                    let mut name = parent.name.clone();
                    name.path.push(RoleSeg::Fragment(Qualifier::Borders(walls)));
                    let ents = faces.iter().map(|&f| ent(0, EntityKey::Face(f))).collect();
                    mint_candidates(&mut h.t, &mut h.tie, tied, canonical::form(name), ents)?;
                }
            }
        }
    }
    Ok(())
}

/// Chord edges: every boundary edge of `body`'s section faces, with the
/// operand face across it. Every face of `sections` is a live face of
/// `body`: its one caller, [`split_chords`], reads them off
/// `body.faces()` in the same call.
///
/// The walk is half-edge → mate → mate's loop → face, by hand, because
/// each hop panics in its own words; `Body::face_of_half_edge` answers
/// the last two hops as one `None`. `walk_tests` in [`super::emit`]
/// drives each hop's panic through a body with that hop's entity
/// removed.
///
/// # Panics
///
/// Where a hop does not resolve: the face is live (above), and each hop
/// reads a record of a body every
/// door keeps tier-1-valid, which mates every half-edge with a live one
/// in a live loop.
pub(super) fn chord_faces<T: geom_core::Real>(
    body: &Body<T>,
    sections: &[(FaceKey, PlaneSide)],
    section_keys: &BTreeSet<FaceKey>,
) -> BTreeMap<EdgeKey, FaceKey> {
    const TIER1: &str =
        "a tier-1-valid body mates every half-edge with a live half-edge in a live loop";
    let mut chord_faces: BTreeMap<EdgeKey, FaceKey> = BTreeMap::new();
    for &(sf, _) in sections {
        if !section_keys.contains(&sf) {
            continue;
        }
        for he in face_half_edges(body, sf) {
            let Some(mate) = body.mate(he) else {
                unreachable!("chord mate missing: half-edge {he:?} of section face {sf:?}; {TIER1}")
            };
            let Some(mate_he) = body.get_half_edge(mate) else {
                unreachable!(
                    "chord mate dangling: mate {mate:?} of half-edge {he:?} of section face \
                     {sf:?}; {TIER1}"
                )
            };
            let Some(other) = body.get_loop(mate_he.parent_loop) else {
                unreachable!(
                    "chord loop dangling: loop {:?} of mate {mate:?} of section face {sf:?}; \
                     {TIER1}",
                    mate_he.parent_loop
                )
            };
            chord_faces.insert(mate_he.edge, other.face);
        }
    }
    chord_faces
}

/// Why no edge lies between two faces the cut drew: each split half
/// ends with maximal faces (`topo::splitting`'s `end_maximal`, which
/// merges every coplanar pair across an edge, and two faces in the
/// plane are one), so none survives.
const BETWEEN_SECTIONS: &str = "each split half ends with maximal faces, and two faces in the tool \
                                plane across one edge are merged into one";

/// A split half's section chords: each edge of `body` bounding a face
/// the cut drew (`SplitNaming::holds_section`), with the operand face
/// across it. An edge along an operand edge lying in the plane is that
/// edge, or a piece of it, and not a chord, where both its faces
/// descend from that edge's two faces (N2); a face the merge kept
/// descends from the operand faces among its constituents.
///
/// # Panics
///
/// Where an edge lies between two faces the cut drew
/// ([`BETWEEN_SECTIONS`]), or a live edge's two faces do not read.
fn split_chords<T: Decide>(
    body: &Body<T>,
    naming: &SplitNaming,
    merged: &SplitMerges,
    root_of: &BTreeMap<EdgeKey, EdgeKey>,
    target_table: &NameTable,
    target_body: &Body<T>,
) -> BTreeMap<EdgeKey, FaceKey> {
    let holders: Vec<(FaceKey, PlaneSide)> = body
        .faces()
        .map(|(f, _)| f)
        .filter(|&f| naming.holds_section(f))
        .map(|f| (f, PlaneSide::On))
        .collect();
    let keys: BTreeSet<FaceKey> = holders.iter().map(|&(f, _)| f).collect();
    let descends = |g: FaceKey, from: FaceKey| match merged.kept.get(&g) {
        Some(m) => m.operand_faces.contains(&from),
        None => !keys.contains(&g) && naming.face_fragments.root(g) == from,
    };
    let mut out = BTreeMap::new();
    for (e, other) in chord_faces(body, &holders, &keys) {
        if keys.contains(&other) {
            unreachable!(
                "section chord {e:?} lies between two faces the cut drew: {BETWEEN_SECTIONS}"
            );
        }
        let root = root_of[&e];
        if target_table
            .name_of(&ent(0, EntityKey::Edge(root)))
            .is_some()
        {
            let (g0, g1) = live_edge_sides(body, e, "the chord walk read it off the half");
            let (fa, fb) = live_edge_sides(
                target_body,
                root,
                "the target table names it, and a table names its own body's live entities",
            );
            if (descends(g0, fa) && descends(g1, fb)) || (descends(g0, fb) && descends(g1, fa)) {
                continue;
            }
        }
        out.insert(e, other);
    }
    out
}

/// How a split names an edge its closing join made, read off the join
/// records (`SplitNaming::edge_joins`) by the input cells the edge's
/// cover lies along (`topo::join_covers`, each covered edge chased to
/// its operand root through `SplitNaming::joined_lineage`).
#[derive(Clone)]
enum Joined {
    /// The cover lies along exactly one operand edge, and no other live
    /// edge of either side descends from it: the edge IS that operand
    /// edge, cut and joined back whole, and takes its own name.
    Whole(Upstream),
    /// The cover lies along several whole operand edges: the `Merged`
    /// set of their names.
    Set { name: StableName, tied: bool },
    /// The cover lies along several operand edges, part of one or more
    /// of them: a piece of their `Merged` set, qualified by its ends
    /// where the half holds several.
    SetPiece {
        name: StableName,
        tied: bool,
        roots: Vec<EdgeKey>,
    },
    /// The cover lies along section chords and whole operand edges,
    /// across a merged face: named from its two faces, as a seam is.
    Seam {
        name: StableName,
        tied: bool,
        parents: ParentEntities<EntityKey>,
    },
}

/// The joined edges of both sides whose covers lie along whole operand
/// edges, with their names ([`Joined`]). An edge left out takes the
/// pass it would take unjoined:
/// - its cover lies along section chords alone, so it is a chord of
///   the operand face it crosses, as every chord is (`SectionEdge`);
/// - its cover lies along part of ONE operand edge whose other pieces
///   live on, so it is a piece of that edge (`SplitFragment`).
///
/// # Errors
///
/// [`NamingError::UnsupportedJoin`] where a cover has no input-cell reading: a
/// section chord joined to a piece of an operand edge.
#[allow(clippy::too_many_arguments)]
fn split_joined_readings<T: Decide>(
    node: RecipeNodeId,
    sides: &[Side<'_, T>],
    naming: &SplitNaming,
    merged: &SplitMerges,
    root_of: &BTreeMap<EdgeKey, EdgeKey>,
    (joins, covers): (&[topo::EdgeJoin], &BTreeMap<EdgeKey, Vec<EdgeKey>>),
    target_node: RecipeNodeId,
    target_table: &NameTable,
) -> Result<BTreeMap<EdgeKey, Joined>, NamingError> {
    let lineage = &naming.joined_lineage;
    // Each record states the half it was made on: the edge it leaves
    // lives there.
    for &(side, j) in &naming.edge_joins {
        let half = half_of(side, "split join row", &j);
        let left = topo::joined_edge(joins, j.kept);
        if !sides
            .iter()
            .any(|sb| sb.half == half && sb.body.get_edge(left).is_some())
        {
            unreachable!(
                "the edge {left:?} a split's join {j:?} left is not live in the {half:?} half its \
                 row names: `SplitNaming::edge_joins` records each join with the side whose body \
                 it was made on, and the joins after it on that side keep or rejoin its edge there"
            );
        }
    }
    let named = |k: EdgeKey| target_table.name_of(&ent(0, EntityKey::Edge(k))).is_some();
    let mut out = BTreeMap::new();
    for (&kept, cover) in covers {
        if !sides.iter().any(|sb| sb.body.get_edge(kept).is_some()) {
            unreachable!(
                "a split join's cover {kept:?} is live in neither half: `topo::join_covers` keys \
                 each cover by the edge its joins left, and each join's row was just found to \
                 leave that edge live in its half"
            );
        }
        let mut roots = BTreeSet::new();
        let mut chords = 0usize;
        for &c in cover {
            let root = chase_split_lineage(sides, target_table, lineage, c);
            if named(root) {
                roots.insert(root);
            } else {
                chords += 1;
            }
        }
        if roots.is_empty() {
            continue; // chords alone: the chord pass names it
        }
        // Whole: no other live edge of either side descends from the root.
        let whole = |r: EdgeKey| root_of.iter().all(|(&e, &er)| er != r || e == kept);
        if chords > 0 {
            let seam =
                split_join_seam(node, sides, naming, merged, kept, target_node, target_table)?;
            match seam {
                Some((name, tied, parents)) if roots.iter().all(|&r| whole(r)) => {
                    out.insert(
                        kept,
                        Joined::Seam {
                            name,
                            tied,
                            parents,
                        },
                    );
                    continue;
                }
                _ => {
                    return Err(NamingError::UnsupportedJoin {
                        edge: kept,
                        shape: JoinShape::ChordWithPiece,
                    });
                }
            }
        }
        match roots.into_iter().collect::<Vec<_>>().as_slice() {
            [r] if whole(*r) => {
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(*r)))?;
                out.insert(kept, Joined::Whole(up));
            }
            [_] => {} // a piece: the fragment pass names it
            several => {
                let mut names = Vec::with_capacity(several.len());
                for &r in several {
                    let up = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(r)))?;
                    names.push(((*up.name).clone(), up.tied));
                }
                let (name, tied) = super::join_names::joined_name(node, names);
                // Along part of an operand edge whose other parts live
                // on: a piece of the set, told from its siblings by its
                // ends, as a union's flush rim is.
                let joined = if several.iter().all(|&r| whole(r)) {
                    Joined::Set { name, tied }
                } else {
                    Joined::SetPiece {
                        name,
                        tied,
                        roots: several.to_vec(),
                    }
                };
                out.insert(kept, joined);
            }
        }
    }
    Ok(out)
}

/// The seam name of `kept`, a split's joined edge, where one of its two
/// faces is a face the merge kept: `Seam` of its two faces' parents in
/// name order (N3: a seam cites the merged face's `Merged` set and the
/// other face's operand face), whether either descends from a tie, and
/// the two parents. `None` where neither face is a merged one.
fn split_join_seam<T: Decide>(
    node: RecipeNodeId,
    sides: &[Side<'_, T>],
    naming: &SplitNaming,
    merged: &SplitMerges,
    kept: EdgeKey,
    target_node: RecipeNodeId,
    target_table: &NameTable,
) -> Result<Option<(StableName, bool, ParentEntities<EntityKey>)>, NamingError> {
    let Some(body) = sides
        .iter()
        .map(|s| s.body)
        .find(|b| b.get_edge(kept).is_some())
    else {
        unreachable!(
            "a split join's cover {kept:?} is live in neither half: `split_joined_readings` \
             found it live before reading its seam"
        );
    };
    let (f0, f1) = live_edge_sides(body, kept, "the half it was found in holds it");
    let (m, m_face, other) = match (merged.kept.get(&f0), merged.kept.get(&f1)) {
        (Some(m), None) => (m, f0, f1),
        (None, Some(m)) => (m, f1, f0),
        _ => return Ok(None),
    };
    if naming.holds_section(other) {
        if naming.holds_section(m_face) {
            unreachable!(
                "split joined edge {kept:?} lies between faces {f0:?} and {f1:?}, both drawn by \
                 the cut: {BETWEEN_SECTIONS}"
            );
        }
        return Err(NamingError::UnsupportedJoin {
            edge: kept,
            shape: JoinShape::ChordBesideMergedFace,
        });
    }
    let (parent, tied) = merged.parent_of(other, naming, target_node, target_table)?;
    let (a, b) = if *parent <= *m.name {
        (parent, m.name.clone())
    } else {
        (m.name.clone(), parent)
    };
    let name = canonical::form(name1(EntityKind::Edge, node, RoleSeg::Seam { a, b }));
    let parents = BTreeSet::from([
        merged.parent_faces(other, naming),
        m.operand_faces
            .iter()
            .map(|&f| EntityKey::Face(f))
            .collect(),
    ]);
    Ok(Some((name, tied || m.tied, parents)))
}

/// Split edges + vertices: pass-through, `SectionEdge` (chords),
/// `SplitFragment` (crossing-cut operand edges), `CrossingVertex`,
/// `OnToolVertex`. The edges are grouped by their parent first, the
/// vertices named from those parents, and then several pieces of one
/// parent qualified by their ends ([`name_edge_pieces`]).
#[allow(clippy::too_many_arguments)]
fn name_split_edges_vertices<T: Decide>(
    node: RecipeNodeId,
    halves: &mut [HalfNames; 2],
    sides: &[Side<'_, T>],
    naming: &SplitNaming,
    merged: &SplitMerges,
    joins: &[topo::EdgeJoin],
    covers: &BTreeMap<EdgeKey, Vec<EdgeKey>>,
    keeps: &KeepsReading<'_, '_, T>,
    target_body: &Body<T>,
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let (target_node, target_table) = (keeps.target_node, keeps.target_table);
    let copy_to_original: BTreeMap<VertexKey, VertexKey> =
        naming.vertex_pairs.iter().copied().collect();
    // (side, base, parents) → (from a tie, the side's edges under it).
    type GroupKey = (usize, StableName, ParentEntities<EntityKey>);
    let mut edge_groups: BTreeMap<GroupKey, (bool, Vec<EdgeKey>)> = BTreeMap::new();
    // Every live edge's operand root, once, on both sides.
    let mut root_of: BTreeMap<EdgeKey, EdgeKey> = BTreeMap::new();
    for sb in sides {
        for (e, _) in sb.body.edges() {
            root_of.insert(e, chase_split_edge_to_table(sides, target_table, e));
        }
    }
    // A divided parent is collected across BOTH sides first: a
    // kept-key first child looks like an intact operand edge in ITS
    // side alone. FRESH children only: an edge the target table already
    // names is the target's own entity, not a product of THIS split.
    let divided_edges: BTreeSet<EdgeKey> = sides
        .iter()
        .flat_map(|sb| sb.body.edges().map(|(e, _)| e))
        .filter(|&e| {
            target_table.name_of(&ent(0, EntityKey::Edge(e))).is_none()
                && sides.iter().any(|sb| {
                    matches!(
                        sb.body.edge_provenance_of(e),
                        Some(Provenance::SplitEdge { .. })
                    )
                })
        })
        .map(|e| root_of[&e])
        .collect();
    let joined = split_joined_readings(
        node,
        sides,
        naming,
        merged,
        &root_of,
        (joins, covers),
        target_node,
        target_table,
    )?;
    for s in sides {
        let slot = s.slot();
        let body = s.body;
        let HalfNames { t, tie, .. } = &mut halves[slot];
        let chord_faces = split_chords(body, naming, merged, &root_of, target_table, target_body);
        // Chord edges named by the operand face their section boundary
        // runs across. A section line that re-enters ONE operand face
        // (an inner loop, or any non-convex face) cuts several chords
        // `SectionEdge{side, face}` spells alike: pieces of one parent,
        // told apart by their ends like any other (N2).
        for (&e, &other) in &chord_faces {
            if joined.contains_key(&e) {
                // An operand edge lying in the cut plane bounds a section
                // face as a chord does; joined, it is named by the
                // operand edges its cover lies along, below. Reaching
                // this takes a vertex on the plane between two collinear
                // operand edges in it whose every off-plane edge fell to
                // the other half; no `ci` row builds one, and the records
                // decide the reading whatever `chord_faces` holds.
                continue;
            }
            let (parent, tied) = merged.parent_of(other, naming, target_node, target_table)?;
            let name = name1(
                EntityKind::Edge,
                node,
                RoleSeg::SectionEdge { face: parent },
            );
            let parents = BTreeSet::from([merged.parent_faces(other, naming)]);
            let group = edge_groups.entry((slot, name, parents)).or_default();
            group.0 |= tied;
            group.1.push(e);
        }
        // The side's pieces of each operand edge, for the crossings'
        // senses.
        // The side's pieces of each operand edge, each with whether it
        // runs the operand edge's way, for the crossings' senses.
        let mut pieces_of: BTreeMap<EdgeKey, Vec<(EdgeKey, bool)>> = BTreeMap::new();
        // Remaining edges: pass-through or crossing-cut fragments.
        for (e, _) in body.edges() {
            match joined.get(&e) {
                // A joined edge whose cover lies along whole operand
                // edges: the one's own name, or the set of them.
                Some(Joined::Whole(up)) => {
                    tie.carry(up.clone(), ent(0, EntityKey::Edge(e)))?;
                    continue;
                }
                Some(Joined::Set { name, tied }) => {
                    put(t, tie, *tied, name.clone(), ent(0, EntityKey::Edge(e)))?;
                    continue;
                }
                // Several seams of one pair of faces in the half are
                // told apart by their ends, as a boolean's seam pieces
                // are.
                Some(Joined::Seam {
                    name,
                    tied,
                    parents,
                }) => {
                    let group = edge_groups
                        .entry((slot, name.clone(), parents.clone()))
                        .or_default();
                    group.0 |= *tied;
                    group.1.push(e);
                    continue;
                }
                Some(Joined::SetPiece { name, tied, roots }) => {
                    // It holds the crossings at its ends of the operand
                    // edges it lies along, whichever way each runs.
                    let joined = EdgeCarrier::of_edge(body, e)?;
                    for &r in roots {
                        let same = EdgeCarrier::of_edge(target_body, r)?
                            .runs_with(&joined, ON_MEMBER_EDGE, bnd)?
                            .ok_or(NamingError::JoinedPieceOffRoot { edge: e })?;
                        pieces_of.entry(r).or_default().push((e, same));
                    }
                    let parents =
                        BTreeSet::from([roots.iter().map(|&r| EntityKey::Edge(r)).collect()]);
                    let group = edge_groups
                        .entry((slot, name.clone(), parents))
                        .or_default();
                    group.0 |= *tied;
                    group.1.push(e);
                    continue;
                }
                None => {}
            }
            if chord_faces.contains_key(&e) {
                continue;
            }
            let root = root_of[&e];
            pieces_of.entry(root).or_default().push((e, true));
            if target_table.name_of(&ent(0, EntityKey::Edge(e))).is_some()
                && !divided_edges.contains(&root)
            {
                // Intact operand edge: pass-through.
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(e)))?;
                tie.carry(up, ent(0, EntityKey::Edge(e)))?;
                continue;
            }
            if target_table
                .name_of(&ent(0, EntityKey::Edge(root)))
                .is_none()
            {
                unreachable!(
                    "split edge {e:?} is no chord, no join and no intact operand edge, yet its \
                     lineage stops at {root:?}, which the target table does not name: a split \
                     mints chords of the faces it draws and pieces of the operand edges it \
                     crosses, each piece with its `SplitEdge` record"
                );
            }
            let parent = upstream_name(target_table, target_node, ent(0, EntityKey::Edge(root)))?;
            let name = name1(
                EntityKind::Edge,
                node,
                RoleSeg::SplitFragment {
                    parent: parent.name,
                },
            );
            let parents = BTreeSet::from([BTreeSet::from([EntityKey::Edge(root)])]);
            let group = edge_groups.entry((slot, name, parents)).or_default();
            group.0 |= parent.tied;
            group.1.push(e);
        }
        // Vertices. Pair membership FIRST: a vertex the tool plane
        // passed through exists as a coincident copy in BOTH halves
        // (null-pair rows), so even an operand-named original takes
        // `OnToolVertex`, never the bare pass-through (the bare name
        // would alias across the two halves).
        let pair_originals: BTreeSet<VertexKey> =
            naming.vertex_pairs.iter().map(|&(_, o)| o).collect();
        // Each vertex a fusion kept → the vertices fused into it.
        let mut fused_into: BTreeMap<VertexKey, Vec<VertexKey>> = BTreeMap::new();
        for &(dead, _) in naming.vertex_fusions.rows() {
            fused_into
                .entry(naming.vertex_fusions.survivor(dead))
                .or_default()
                .push(dead);
        }
        // Crossing vertices, by the operand edge they cross: (from a
        // tie, the vertices).
        let mut crossings: BTreeMap<EdgeKey, (Upstream, Vec<VertexKey>)> = BTreeMap::new();
        // Copies of an operand vertex the plane passed through, by that
        // vertex.
        let mut on_tool: BTreeMap<VertexKey, (Upstream, Vec<VertexKey>)> = BTreeMap::new();
        for (v, _) in body.vertices() {
            let is_pair_member = copy_to_original.contains_key(&v) || pair_originals.contains(&v);
            if !is_pair_member
                && target_table
                    .name_of(&ent(0, EntityKey::Vertex(v)))
                    .is_some()
            {
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Vertex(v)))?;
                tie.carry(up, ent(0, EntityKey::Vertex(v)))?;
                continue;
            }
            // The vertex it stands for: itself, or the original its
            // null-pair copy rows end at. A vertex a fusion of a pinch's
            // cones kept stands for each vertex fused into it too
            // (`SplitNaming::vertex_fusions`), and takes the one of
            // those the target names least, whichever key survived.
            let src = fused_into
                .get(&v)
                .map_or(Ok(naming.original_of(v)), |dead| {
                    least_named(
                        core::iter::once(v)
                            .chain(dead.iter().copied())
                            .map(|k| naming.original_of(k)),
                        target_table,
                        target_node,
                    )
                })?;
            if target_table
                .name_of(&ent(0, EntityKey::Vertex(src)))
                .is_some()
            {
                // The plane passed THROUGH an operand vertex: its
                // copies in this half, named below. Its own birth
                // record, if any, is the operation that made the
                // operand, never a crossing of this plane.
                match on_tool.get_mut(&src) {
                    Some((_, vs)) => vs.push(v),
                    None => {
                        let of = upstream_name(
                            target_table,
                            target_node,
                            ent(0, EntityKey::Vertex(src)),
                        )?;
                        on_tool.insert(src, (of, vec![v]));
                    }
                }
                continue;
            }
            // A vertex the split minted: the original's record lives in
            // whichever side kept it.
            let parent_edge = sides
                .iter()
                .find_map(|sb| match sb.body.vertex_provenance_of(src) {
                    Some(Provenance::SplitEdge { edge }) => {
                        Some(chase_split_edge_to_table(sides, target_table, *edge))
                    }
                    _ => None,
                });
            let Some(parent_edge) = parent_edge else {
                unreachable!(
                    "split vertex {v:?} (standing for {src:?}) has neither a `SplitEdge` record \
                     nor an operand identity: a split mints a vertex only where the plane crosses \
                     an operand edge, recording the edge, or as a copy of an operand vertex in the \
                     plane, recorded in `SplitNaming::vertex_pairs`"
                );
            };
            // Crossing vertex: minted where the plane crossed an operand
            // edge's interior.
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
        }
        for (corner, (of, copies)) in on_tool {
            let base = name1(
                EntityKind::Vertex,
                node,
                RoleSeg::OnToolVertex { of: of.name },
            );
            if let [v] = copies.as_slice() {
                put(t, tie, of.tied, base, ent(0, EntityKey::Vertex(*v)))?;
                continue;
            }
            // Several copies in this half: each by the corner's own
            // edges it is an end of a piece of here (N2's `Keeps`).
            let own: BTreeSet<EdgeKey> = live_vertex_edges(
                target_body,
                corner,
                "the target table names it, and a table names its own body's live entities",
            )
            .into_iter()
            .collect();
            let mut pieces = Vec::with_capacity(copies.len());
            for v in copies {
                let at = live_vertex_edges(body, v, "the half's own vertex walk read it");
                pieces.push((keeps.of(at, &own)?, ent(0, EntityKey::Vertex(v))));
            }
            mint_qualified(t, tie, of.tied, &base, pieces)?;
        }
        // Each crossing carries its sense against this half; a plane
        // crosses a straight edge at most once, and an arc it crosses
        // twice leaves crossings of both senses on each side. Several
        // of one sense on one line tie (N4).
        let mut by_line: BTreeMap<(NameRef, Sense), (bool, Vec<super::table::EntityRef>)> =
            BTreeMap::new();
        for (crossed, (parent, verts)) in &crossings {
            let reading = edge_reading(target_body, target_table, *crossed, &parent.name);
            let pieces = pieces_of.get(crossed).map_or(&[][..], Vec::as_slice);
            for &v in verts {
                let sense = reading.read(split_sense(body, v, pieces)?, |side| {
                    NameRef::new(side.clone())
                })?;
                let on = by_line.entry((edge_line(&parent.name), sense)).or_default();
                on.0 |= parent.tied;
                on.1.push(ent(0, EntityKey::Vertex(v)));
            }
        }
        for ((line, sense), (tied, on)) in by_line {
            let base = name1(
                EntityKind::Vertex,
                node,
                RoleSeg::CrossingVertex { edge: line, sense },
            );
            mint_candidates(t, tie, tied, base, on)?;
        }
    }
    let mut pieces: [EdgePieces; 2] = Default::default();
    for h in halves.iter_mut() {
        h.tie.flush(&mut h.t)?;
    }
    // Each group is keyed by the slot of the side whose edges it holds.
    for s in sides {
        let slot = s.slot();
        for ((_, base, _), (from_tie, edges)) in
            edge_groups.iter().filter(|((at, ..), _)| *at == slot)
        {
            // A lone section chord is the whole of the section line
            // across its face; an operand edge's fragment is always a
            // piece of it.
            let lone = match base.path.first() {
                Some(RoleSeg::SectionEdge { .. } | RoleSeg::Merged(_) | RoleSeg::Seam { .. }) => {
                    Lone::Whole
                }
                _ => Lone::Piece,
            };
            name_edge_pieces(
                &mut pieces[slot],
                &halves[slot].t,
                *from_tie,
                base,
                (s.body, 0),
                edges,
                lone,
            );
        }
    }
    for (h, pieces) in halves.iter_mut().zip(pieces) {
        pieces.mint(&mut h.t, &mut h.tie)?;
    }
    Ok(())
}

/// The vertex of `candidates` whose name in `table` is least, or the
/// first where the table names none of them.
///
/// # Errors
///
/// [`upstream_name`]'s, where the table names a candidate it cannot
/// resolve.
fn least_named(
    mut candidates: impl Iterator<Item = VertexKey>,
    table: &NameTable,
    node: RecipeNodeId,
) -> Result<VertexKey, NamingError> {
    let Some(first) = candidates.next() else {
        unreachable!("a fused vertex stands for itself at least")
    };
    let mut least: Option<(NameRef, VertexKey)> = None;
    for k in core::iter::once(first).chain(candidates) {
        if table.name_of(&ent(0, EntityKey::Vertex(k))).is_none() {
            continue;
        }
        let name = upstream_name(table, node, ent(0, EntityKey::Vertex(k)))?.name;
        if least.as_ref().is_none_or(|(n, _)| name < *n) {
            least = Some((name, k));
        }
    }
    Ok(least.map_or(first, |(_, k)| k))
}

/// **The sense of a Split's crossing at `v`** (N2) against the half
/// whose body `body` holds it, along the crossed edge as it is stored:
/// `Enters` where the one piece of the edge the half holds at `v`
/// starts there along the edge, `Leaves` where it ends there. `pieces`
/// are the half's pieces of the crossed edge, each with whether it runs
/// the edge's way (a joined edge over several operand edges keeps one
/// of their directions).
///
/// # Errors
///
/// [`NamingError::Emission`] unless exactly one piece meets `v`: a
/// crossing the half holds no piece at, or two, is not a crossing of
/// the edge into or out of the half.
fn split_sense<T: geom_core::Real>(
    body: &Body<T>,
    v: VertexKey,
    pieces: &[(EdgeKey, bool)],
) -> Result<Sense, NamingError> {
    let mut senses = Vec::with_capacity(1);
    for &(e, same) in pieces {
        let (start, end) = edge_ends(body, e);
        let (start, end) = if same { (start, end) } else { (end, start) };
        if start == v {
            senses.push(Sense::Enters);
        }
        if end == v {
            senses.push(Sense::Leaves);
        }
    }
    match senses.as_slice() {
        [one] => Ok(one.clone()),
        _ => Err(NamingError::Emission {
            what: "a split's crossing vertex does not hold exactly one piece of the edge it crosses",
        }),
    }
}

/// One boolean operand under naming.
pub(crate) struct OperandCtx<'a, T: Decide> {
    /// The operand's node (error context).
    pub node: RecipeNodeId,
    /// The read its entities are carried in through ([`RoleSeg::From`]):
    /// a subtract's seat, or a union fold step's side.
    pub read: crate::VarId,
    /// Its name table (total over its body).
    pub table: &'a NameTable,
    /// Its body (the edges and faces its crossings are read on).
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
    /// Operand `side`'s key `k`.
    fn on(side: topo::Operand, k: K) -> Self {
        match side {
            topo::Operand::A => OpSide::A(k),
            topo::Operand::B => OpSide::B(k),
        }
    }

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

    /// The [`RoleSeg::From`] segment wrapping a name read on this side,
    /// keyed by that side's read.
    fn wrap<T: Decide>(
        self,
        a: &OperandCtx<'_, T>,
        b: &OperandCtx<'_, T>,
        inner: NameRef,
    ) -> RoleSeg {
        let (op, _) = self.of(a, b);
        RoleSeg::From {
            read: op.read,
            of: inner,
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

/// **A result key read back to its operand**
/// ([`topo::BooleanNaming::operand_of`]): which operand a result-arena
/// key belongs to, its key in that operand's space, and where that key
/// lives. Every pass that reads a result key back reads it through
/// here.
///
/// # Panics
///
/// As [`topo::BooleanNaming::operand_of`].
#[track_caller]
fn operand_key<K: Copy + Ord>(
    naming: &topo::BooleanNaming,
    inv: &BTreeMap<K, K>,
    k: K,
) -> (OpSide<K>, KeySpace) {
    let (operand, key, grafted) = naming.operand_of(inv, k);
    let space = if grafted {
        KeySpace::Graft
    } else {
        KeySpace::Arena
    };
    (OpSide::on(operand, key), space)
}

/// **A boolean's graft rows read backwards**, result key → B key: the
/// grafted edges, those and the dead-on-arrival keys standing for B
/// edges that died before the graft, and the grafted vertices.
pub(crate) struct GraftInverse {
    /// The grafted edges.
    pub(crate) edges: BTreeMap<EdgeKey, EdgeKey>,
    /// Every key a grafted record can name: `edges` and the dead ones.
    pub(crate) lineage: BTreeMap<EdgeKey, EdgeKey>,
    /// The grafted vertices.
    pub(crate) vertices: BTreeMap<VertexKey, VertexKey>,
}

impl GraftInverse {
    /// `naming`'s graft rows, read backwards.
    pub(crate) fn of(naming: &topo::BooleanNaming) -> Self {
        let back = |&(s, d): &(EdgeKey, EdgeKey)| (d, s);
        Self {
            edges: naming.graft_edges.iter().map(back).collect(),
            lineage: naming
                .graft_edges
                .iter()
                .chain(&naming.graft_dead_edges)
                .map(back)
                .collect(),
            vertices: naming.graft_vertices.iter().map(|&(s, d)| (d, s)).collect(),
        }
    }
}

/// **No name reads a cell a sphere re-chart turned**
/// (`topo::BooleanNaming::recharted`): a live edge whose descent reaches
/// one, chased as the edge pass chases it ([`chase_edge_to_table`],
/// through the split hops the kills dropped), or a live vertex that is
/// one, is a turned seam piece or pole, which descends from no cell of
/// its operand. A re-charted face is not asked after: a kept piece of a
/// turned sphere is a band or a cap about the turned axis, which a
/// turned seam crosses or a turned pole lies in, so its edges or
/// vertices refuse first. `reads` are the operands' reads, A's then
/// B's, and `b_body` B's body, which a grafted B's descent is chased in.
///
/// # Errors
///
/// [`NamingError::UnsupportedTurnedCell`] for the first such edge, in
/// arena order, else the first such vertex;
/// [`NamingError::SplitLineage`] for a cycling lineage.
pub(crate) fn refuse_turned_cells<T: geom_core::Real>(
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    inv: &GraftInverse,
    reads: [crate::VarId; 2],
    b_body: &Body<T>,
) -> Result<(), NamingError> {
    use topo::Operand;
    let turned = |operand: Operand, cell: topo::Cell| {
        naming.recharted[usize::from(operand == Operand::B)].contains(cell)
    };
    if naming
        .recharted
        .iter()
        .all(topo::boolean::Recharts::is_empty)
    {
        return Ok(());
    }
    let refuse = |operand: Operand, cell| NamingError::UnsupportedTurnedCell {
        read: reads[usize::from(operand == Operand::B)],
        cell,
    };
    let killed = KilledSplits::of(naming);
    for (e, _) in body.edges() {
        let (root, space) = operand_key(naming, &inv.edges, e);
        let (operand, root) = match (root, space) {
            (OpSide::A(k), _) | (OpSide::B(k), KeySpace::Arena) => {
                let operand = root.operand();
                let at = |k| turned(operand, topo::Cell::Edge(k));
                (operand, chase_edge_until(body, &killed.arena, at, k)?)
            }
            (OpSide::B(_), KeySpace::Graft) => {
                let at_b = |k| turned(Operand::B, topo::Cell::Edge(k));
                let root = body.split_root(e, |r| inv.lineage.get(&r).is_none_or(|&k| at_b(k)))?;
                // A walk that leaves the graft rows is the edge pass's to
                // refuse.
                let Some(&kb) = inv.lineage.get(&root) else {
                    continue;
                };
                (
                    Operand::B,
                    chase_edge_until(b_body, &killed.graft_b, at_b, kb)?,
                )
            }
        };
        let cell = topo::Cell::Edge(root);
        if turned(operand, cell) {
            return Err(refuse(operand, cell));
        }
    }
    for (v, _) in body.vertices() {
        let (operand, k, _) = naming.operand_of(&inv.vertices, v);
        let cell = topo::Cell::Vertex(k);
        if turned(operand, cell) {
            return Err(refuse(operand, cell));
        }
    }
    Ok(())
}

/// Names a boolean result (spec D2's boolean vocabulary; N2/N3).
pub(crate) fn name_boolean<T: Decide>(
    node: RecipeNodeId,
    op: topo::BooleanOp,
    body: &Body<T>,
    naming: &topo::BooleanNaming,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
    tol: Tol,
) -> Result<Emitted, NamingError> {
    let bnd = band(tol)?;
    let mut t = NameTable::new();
    let mut rec = GroupRecord::new();
    t.insert(
        name1(EntityKind::Body, node, RoleSeg::OutputBody),
        ent(0, EntityKey::Body),
    )?;

    let inv_faces: BTreeMap<FaceKey, FaceKey> =
        naming.graft_faces.iter().map(|&(s, d)| (d, s)).collect();
    let inv = GraftInverse::of(naming);
    let (inv_edges, lineage_inv, inv_vertices) = (&inv.edges, &inv.lineage, &inv.vertices);
    refuse_turned_cells(body, naming, &inv, [a.read, b.read], b.body)?;
    let (a_rows, b_rows) = (&naming.face_fragments_a, &naming.face_fragments_b);
    let seam_set: BTreeSet<EdgeKey> = naming.seam_edges.iter().copied().collect();
    let inc = Incidence::of(body);

    // Result face → operand-space root (fragment rows chased in the
    // right key space). The key the B arm chases — and therefore the
    // key a refusal here renders — is a B-CLONE key, not a key of the
    // result arena: the rows are `face_fragments_b`, minted before the
    // clone was grafted. A reader comparing the rendered key against
    // the result body will not find it.
    let descend_face = |f: FaceKey| -> Result<OpSide<FaceKey>, NamingError> {
        Ok(match operand_key(naming, &inv_faces, f).0 {
            OpSide::A(fa) => OpSide::A(a_rows.root(fa)),
            OpSide::B(fb) => OpSide::B(b_rows.root(fb)),
        })
    };
    let operand_face_name = |d: OpSide<FaceKey>| -> Result<Upstream, NamingError> {
        let (op, f) = d.of(a, b);
        upstream_name(op.table, op.node, ent(0, EntityKey::Face(f)))
    };

    // ---- Faces: merges first (N3), then descent groups. ----
    let mut tie = TieRows::default();
    let mut handled: BTreeSet<FaceKey> = BTreeSet::new();
    // Kept face → constituent descents: which operands a merged face
    // holds faces of, and the faces a chord between two merged faces
    // may lie along a rim of.
    let mut merged_descents: BTreeMap<FaceKey, Vec<OpSide<FaceKey>>> = BTreeMap::new();
    // Operand face → the merged faces it survives in: those faces
    // descend from it too, so they are members of its group
    // (`names::groups`).
    let mut merged_into: BTreeMap<OpSide<FaceKey>, Vec<FaceKey>> = BTreeMap::new();
    // Merged face → its parent's name, the one a `Borders` wall and a
    // seam cite (N3), and whether it descends from a tie.
    let mut merged_names: BTreeMap<FaceKey, (NameRef, bool)> = BTreeMap::new();
    // The merged parents in the order the merges first list them, each
    // keyed by the operand faces its merges list: a parent is a set of
    // entities, never a name, so tied parents spelled alike stay apart.
    let mut merged_parents: Vec<MergedParent> = Vec::new();
    let mut parent_at: BTreeMap<BTreeSet<OpSide<FaceKey>>, usize> = BTreeMap::new();
    for (kept, absorbed) in &naming.merge_groups {
        if body.get_face(*kept).is_none() {
            unreachable!(
                "the face {kept:?} a boolean's merge kept is not live in its result: the merge is \
                 the output stage's last face operation, and `BooleanNaming::merge_groups` lists \
                 each group by the face that holds it"
            );
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
                Some(cs) => {
                    constituents.extend(cs.into_iter().map(|inner| {
                        name1(EntityKind::Face, node, d.wrap(a, b, NameRef::new(inner)))
                    }))
                }
                None => constituents.push(name1(EntityKind::Face, node, d.wrap(a, b, up.name))),
            }
        }
        // The kernel's guarantee that the set is flat, held here for
        // both emitters: a constituent that is itself a merged face
        // (through any wrapping) is a name this loop cannot have
        // built from a flat operand, so it panics rather than being
        // published. `emit_union`'s `collapse` holds the same rule at
        // the union's own door, with its own ending.
        if let Some(nested) = constituents
            .iter()
            .find(|c| merged::constituents_through_wrappers(c).is_some())
        {
            unreachable!(
                "{NESTED_MERGED} ({nested:?}, in the merge kept as {kept:?}): an operand table \
                 holds merged names flat, as every emitter mints them, and this loop read each \
                 constituent's set through its wrappers"
            );
        }
        let parents: BTreeSet<OpSide<FaceKey>> = descents.iter().copied().collect();
        merged_descents.insert(*kept, descents);
        for &d in &parents {
            merged_into.entry(d).or_default().push(*kept);
        }
        let merged = canonical::form(name1(EntityKind::Face, node, RoleSeg::Merged(constituents)));
        merged_names.insert(*kept, (NameRef::new(merged.clone()), from_tie));
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
            topo::Operand::A => OpSide::A(a_rows.root(f)),
            topo::Operand::B => OpSide::B(b_rows.root(f)),
        }]))
    })?;
    // A wall is cited by its parent as this boolean's table names it
    // (N3), as a seam side is: a merged face's `Merged` name, or the
    // `From` the operand face it descends from came in through.
    let wall = |g: FaceKey| -> Result<StableName, NamingError> {
        if let Some((n, _)) = merged_names.get(&g) {
            return Ok((**n).clone());
        }
        let d = descend_face(g)?;
        let up = operand_face_name(d)?;
        Ok(name1(EntityKind::Face, node, d.wrap(a, b, up.name)))
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
        let base = name1(EntityKind::Face, node, d.wrap(a, b, root_name.name));
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
    // An operand corner the result holds as several vertices: each
    // copy's divider walls (N2), read through the faces' obstacles. The
    // corner's faces are its parent, and its pieces are the faces that
    // hold part of them (`held_by`).
    let corner_walls = |sides: &BTreeSet<OpSide<VertexKey>>, copies: &[VertexKey]| {
        let mut parent: BTreeSet<OpSide<FaceKey>> = BTreeSet::new();
        for &side in sides {
            let (op, k) = side.of(a, b);
            let faces = live_vertex_faces(
                op.body,
                k,
                "its operand's table names the corner, and a table names its own body's live \
                 entities",
            );
            parent.extend(faces.into_iter().map(|f| side.with(f)));
        }
        let pieces: BTreeSet<FaceKey> = parent.iter().flat_map(&held_by).collect();
        obstacles.split_corner(body, &parent, copies, &pieces, &wall)
    };

    let (edge_groups, side_holds) = name_boolean_edges(
        node,
        op,
        &mut rec,
        body,
        naming,
        a,
        b,
        inv_edges,
        inv_vertices,
        lineage_inv,
        &seam_set,
        &inc,
        &descend_face,
        &operand_face_name,
        &merged_descents,
        &merged_names,
        bnd,
    )?;
    name_boolean_vertices(
        node,
        &mut t,
        &mut tie,
        &mut rec,
        body,
        naming,
        inv_vertices,
        a,
        b,
        &inc,
        &edge_groups,
        &side_holds,
        &corner_walls,
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
        );
    }
    pieces.mint(&mut t, &mut tie)?;
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
                mint_candidates(t, tie, from_tie, canonical::form(name), ents)?;
            }
            Ok(())
        }
    }
}

/// The vertices one operand corner is held as: its name, whether that
/// name is tied, and each copy with its row's entity.
pub(super) struct CornerCopies {
    pub(super) name: StableName,
    pub(super) tied: bool,
    pub(super) copies: Vec<(VertexKey, EntityRef)>,
}

/// **Operand corners, each by the vertices it is held as** (N2, N4). A
/// corner is keyed by `K`, what tells it from every other corner: at
/// the pair boolean the operand keys of one candidate of its name, at
/// the union the member corners it sits at. One held as one
/// vertex is its name, a tied one joining its name's tie with the other
/// undivided candidates. One held as several is fragments of it: each
/// copy is `Fragment(Borders)` over the divider walls `split` finds for
/// it (`Obstacles::split_corner`), so only a divided candidate's copies
/// take `Borders`, and pieces with one name, of one candidate or
/// several, tie. The pair boolean's vertex pass and the union's end pass
/// both name their corners here.
pub(super) fn name_corner_copies<K>(
    t: &mut NameTable,
    tie: &mut TieRows,
    corners: impl IntoIterator<Item = (K, CornerCopies)>,
    mut split: impl FnMut(
        &K,
        &[VertexKey],
    ) -> Result<BTreeMap<Vec<StableName>, Vec<VertexKey>>, NamingError>,
) -> Result<(), NamingError> {
    let mut pieces: BTreeMap<StableName, (bool, Vec<EntityRef>)> = BTreeMap::new();
    for (at, held) in corners {
        if let [(_, one)] = held.copies[..] {
            put(t, tie, held.tied, held.name, one)?;
            continue;
        }
        let keys: Vec<VertexKey> = held.copies.iter().map(|&(v, _)| v).collect();
        for (walls, vs) in split(&at, &keys)? {
            let mut piece = held.name.clone();
            piece
                .path
                .push(RoleSeg::Fragment(Qualifier::Borders(walls)));
            let slot = pieces
                .entry(canonical::form(piece))
                .or_insert((false, Vec::new()));
            slot.0 |= held.tied;
            slot.1.extend(
                held.copies
                    .iter()
                    .filter(|(v, _)| vs.contains(v))
                    .map(|&(_, e)| e),
            );
        }
    }
    for (name, (from_tie, ents)) in pieces {
        mint_candidates(t, tie, from_tie, name, ents)?;
    }
    Ok(())
}

/// Boolean edges, grouped by parent: `Seam` for zip-minted edges,
/// `From` for operand-descended ones, and `Merged` for an edge
/// the output stage joined across several operand edges
/// ([`joined_cover`]). A group's pieces are qualified once the vertices
/// are named ([`EdgeGroup`]). Each seam side's held operand faces come
/// back with them ([`SideHolds`]), for the vertex pass.
#[allow(clippy::too_many_arguments)]
fn name_boolean_edges<T: Decide>(
    node: RecipeNodeId,
    op: topo::BooleanOp,
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
    merged_names: &BTreeMap<FaceKey, (NameRef, bool)>,
    bnd: geom_core::Band,
) -> Result<(Vec<EdgeGroup>, SideHolds), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let fused = Fused::of(naming);

    // ---- Seam edges (zip-listed AND derived — see below), grouped
    // by the two parents they lie between. A derived chord between two
    // SAME-operand faces (the collinear channel-cut lane re-mints a
    // sub-edge of an operand edge as a chord) descends instead to an
    // operand edge its two parent faces share — combinatorial adjacency
    // of emitted anchors, not matching, while the pair shares one edge.
    // When it shares SEVERAL — pieces of one rim an earlier step cut, or
    // separate curves such as a half-cylinder's two lines between its
    // flat and curved faces — the chord's geometry picks the one it lies
    // within (`rim_holding`); when nothing picks one,
    // `NamingError::SharedRim` is what the arm below says. ----
    /// One side of a seam: the parent of the face beside it as this
    /// node's table names it (N3) — a merged face's `Merged` set, an
    /// unmerged face's `From` its operand's read came in through —
    /// whether that descends from a tie, and the operand faces it holds.
    struct SeamSide {
        name: NameRef,
        tied: bool,
        holds: Vec<OpSide<FaceKey>>,
    }
    enum ChordKind {
        /// A seam: its two sides in name order, and the one operand both
        /// faces descend from where they do.
        Cross(SeamSide, SeamSide, Option<topo::Operand>),
        SameA(EdgeKey),
        SameB(EdgeKey),
    }
    // A seam's two sides are in name order (N3): the order spells a
    // set, and which operand a side came from is in its own name.
    let seam = |s0: SeamSide, s1: SeamSide, one: Option<topo::Operand>| {
        if s1.name < s0.name {
            ChordKind::Cross(s1, s0, one)
        } else {
            ChordKind::Cross(s0, s1, one)
        }
    };
    let plain = |d: OpSide<FaceKey>| -> Result<SeamSide, NamingError> {
        let up = operand_face_name(d)?;
        Ok(SeamSide {
            name: NameRef::new(name1(EntityKind::Face, node, d.wrap(a, b, up.name))),
            tied: up.tied,
            holds: vec![d],
        })
    };
    // A merged face's seam side is its parent, the `Merged` set, never a
    // constituent (N3).
    // Called only for a face `merged_descents` holds, and the merge loop
    // inserts its name and its descents (the kept face's among them, so
    // never none) together.
    let merged_side = |f: FaceKey| -> SeamSide {
        let (Some((name, tied)), Some(holds)) = (merged_names.get(&f), merged_descents.get(&f))
        else {
            unreachable!(
                "merged face {f:?} lacks its parent name or its descents: a chord's face is read \
                 as merged only where `merged_descents` holds it, and the merge loop records both \
                 for every kept face, its own descent among them"
            )
        };
        SeamSide {
            name: name.clone(),
            tied: *tied,
            holds: holds.clone(),
        }
    };
    // The faces of operand `side` a merged face holds, in key order.
    let constituents_on = |f: FaceKey, side: topo::Operand| -> Vec<FaceKey> {
        let mut on: Vec<FaceKey> = merged_descents
            .get(&f)
            .into_iter()
            .flatten()
            .filter(|d| d.operand() == side)
            .map(|d| d.of_operand().1)
            .collect();
        on.sort_unstable();
        on.dedup();
        on
    };
    // The one operand every face a merged face holds is of, if one is.
    let only_operand = |f: FaceKey| -> Option<topo::Operand> {
        let mut ops = merged_descents
            .get(&f)
            .into_iter()
            .flatten()
            .map(|d| d.operand());
        let first = ops.next()?;
        ops.all(|o| o == first).then_some(first)
    };
    let same = |side: topo::Operand, rim: EdgeKey| match side {
        topo::Operand::A => ChordKind::SameA(rim),
        topo::Operand::B => ChordKind::SameB(rim),
    };

    // **One rule for a chord between faces of ONE operand**, merged or
    // not: the chord's rim is the one rim of that operand, between a face
    // on one side of it and a face on the other, it lies within
    // ([`rim_among`]); a merged face offers every face of that operand
    // it holds. What a reading that holds no rim means is
    // [`one_operand_chord`]'s.
    let chord_kind = |e: EdgeKey| -> Result<ChordKind, NamingError> {
        let Some(faces) = inc.edge_faces.get(&e) else {
            unreachable!(
                "seam edge {e:?} bounds no face: the incidence read every half-edge of the \
                 result, and in a tier-1-valid body each live edge has two half-edges, each in \
                 a face's loop"
            )
        };
        if faces.len() != 2 {
            return Err(NamingError::UnsupportedSeamSides {
                edge: e,
                shape: SeamSidesShape::NotTwoFaces,
            });
        }
        let union = op == topo::BooleanOp::Union;
        // A merged face beside an unmerged one. Where the merged face
        // holds a face of the other operand, the chord is a seam between
        // the merged face's parent and the unmerged face's. Where it
        // holds faces of the unmerged face's operand only, the two are
        // faces of one operand, read as two unmerged faces are.
        let beside = |m: FaceKey, f: FaceKey| -> Result<ChordKind, NamingError> {
            let d = descend_face(f)?;
            let side = d.operand();
            if !constituents_on(m, side.other()).is_empty() {
                return Ok(seam(plain(d)?, merged_side(m), None));
            }
            let (ctx, fd) = d.of(a, b);
            let held = constituents_on(m, side);
            let pairs: Vec<(FaceKey, FaceKey)> = held.iter().map(|&c| (c, fd)).collect();
            let shared = held.contains(&fd).then_some(fd);
            let pick = rim_among(body, e, ctx, &pairs, bnd)?;
            Ok(match one_operand_chord(pick, e, ctx, shared, union)? {
                OneOperand::Rim(rim) => same(side, rim),
                OneOperand::Gap => seam(merged_side(m), plain(d)?, Some(side)),
            })
        };
        // Two merged faces. Each holding faces of the same one operand,
        // the chord is read against that operand's rims as two unmerged
        // faces' is. Otherwise it is read against every rim of BOTH
        // operands between the faces they hold: one holding is a piece of
        // that rim, and none is the seam of the two parents, as a
        // crossing between them is.
        let merged_chord = |m0: FaceKey, m1: FaceKey| -> Result<ChordKind, NamingError> {
            if let (Some(x), Some(y)) = (only_operand(m0), only_operand(m1))
                && x == y
            {
                let (c0, c1) = (constituents_on(m0, x), constituents_on(m1, x));
                let pairs: Vec<(FaceKey, FaceKey)> = c0
                    .iter()
                    .flat_map(|&f0| c1.iter().map(move |&f1| (f0, f1)))
                    .collect();
                let shared = c0.iter().find(|f| c1.contains(f)).copied();
                let ctx = OpSide::on(x, ()).of(a, b).0;
                let pick = rim_among(body, e, ctx, &pairs, bnd)?;
                return Ok(match one_operand_chord(pick, e, ctx, shared, union)? {
                    OneOperand::Rim(rim) => same(x, rim),
                    OneOperand::Gap => seam(merged_side(m0), merged_side(m1), Some(x)),
                });
            }
            let (ca0, ca1) = (
                constituents_on(m0, topo::Operand::A),
                constituents_on(m1, topo::Operand::A),
            );
            let (cb0, cb1) = (
                constituents_on(m0, topo::Operand::B),
                constituents_on(m1, topo::Operand::B),
            );
            let sides = [
                (topo::Operand::A, a, ca0.as_slice(), ca1.as_slice()),
                (topo::Operand::B, b, cb0.as_slice(), cb1.as_slice()),
            ];
            Ok(match merged_chord_rim(body, e, sides, bnd)? {
                Some((side, rim)) => same(side, rim),
                None => seam(merged_side(m0), merged_side(m1), None),
            })
        };
        let merged0 = merged_descents.contains_key(&faces[0]);
        let merged1 = merged_descents.contains_key(&faces[1]);
        let (d0, d1) = match (merged0, merged1) {
            (false, false) => (descend_face(faces[0])?, descend_face(faces[1])?),
            (true, false) => return beside(faces[0], faces[1]),
            (false, true) => return beside(faces[1], faces[0]),
            (true, true) => return merged_chord(faces[0], faces[1]),
        };
        let ((OpSide::A(f0), OpSide::A(f1)) | (OpSide::B(f0), OpSide::B(f1))) = (d0, d1) else {
            return Ok(seam(plain(d0)?, plain(d1)?, None));
        };
        let side = d0.operand();
        let ctx = d0.of(a, b).0;
        let pick = rim_among(body, e, ctx, &[(f0, f1)], bnd)?;
        Ok(
            match one_operand_chord(pick, e, ctx, (f0 == f1).then_some(f0), union)? {
                OneOperand::Rim(rim) => same(side, rim),
                OneOperand::Gap => seam(plain(d0)?, plain(d1)?, Some(side)),
            },
        )
    };
    // (the two sides' names, their parents) → (descends from a tie, the
    // one operand both faces descend from, edges).
    type SeamGroup = (bool, Option<topo::Operand>, Vec<EdgeKey>);
    type SeamKey = (NameRef, NameRef, ParentEntities<OpSide<FaceKey>>);
    let mut seam_groups: BTreeMap<SeamKey, SeamGroup> = BTreeMap::new();
    // Each side's name → the operand faces it holds, which the vertex
    // pass reads a side's operand off.
    let mut side_holds: SideHolds = BTreeMap::new();
    // One pair of parents is one seam, so its edges agree on whether
    // both faces are one operand's; a pair that disagrees is two seams
    // spelled alike, which the group cannot hold.
    let mut add_seam = |fa: SeamSide,
                        fb: SeamSide,
                        one: Option<topo::Operand>,
                        e: EdgeKey|
     -> Result<(), NamingError> {
        let from_tie = fa.tied || fb.tied;
        for s in [&fa, &fb] {
            let held = side_holds.entry(s.name.clone()).or_default();
            held.extend(s.holds.iter().copied());
            held.sort_unstable();
            held.dedup();
        }
        let parents = BTreeSet::from([
            fa.holds.iter().copied().collect(),
            fb.holds.iter().copied().collect(),
        ]);
        let slot =
            seam_groups
                .entry((fa.name, fb.name, parents))
                .or_insert((false, one, Vec::new()));
        if slot.1 != one {
            return Err(NamingError::UnsupportedSeamSides {
                edge: e,
                shape: SeamSidesShape::NameSpellsTwoSeams,
            });
        }
        slot.0 |= from_tie;
        slot.2.push(e);
        Ok(())
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

    // A zip-listed edge between two merged faces that lies within an
    // operand rim is a piece of it, as a derived chord is; one between
    // faces of one operand that are not both merged is the join's own
    // seam, so it has no rim to be.
    let mut zip_rims: Vec<(OpSide<EdgeKey>, EdgeKey)> = Vec::new();
    for &e in &naming.seam_edges {
        if body.get_edge(e).is_none() || joined_named(&e) {
            continue; // consumed by the merge stage, or named above
        }
        let both_merged = inc
            .edge_faces
            .get(&e)
            .is_some_and(|fs| fs.iter().all(|f| merged_descents.contains_key(f)));
        match chord_kind(e)? {
            ChordKind::Cross(fa, fb, one) => add_seam(fa, fb, one, e)?,
            ChordKind::SameA(k) if both_merged => zip_rims.push((OpSide::A(k), e)),
            ChordKind::SameB(k) if both_merged => zip_rims.push((OpSide::B(k), e)),
            ChordKind::SameA(_) | ChordKind::SameB(_) => {
                return Err(NamingError::UnsupportedSeamSides {
                    edge: e,
                    shape: SeamSidesShape::WithinOneOperand,
                });
            }
        }
    }

    // ---- Operand-descended edges, grouped by (side, root). ----
    // A GRAFTED B side chases in the result body — the graft forwards
    // every record into result keys — stopping at the first key whose B
    // preimage B's table names, and reads the root back to B. A B
    // ancestor that died before the graft is a dead result key with a
    // `graft_dead_edges` row, so the walk still reaches it; where the
    // chain breaks at a fragment killed PRE-graft (the middle of a
    // doubly-pierced edge), the walk goes on in B's keys through the
    // hops the kill dropped (`KilledSplits`). A walk that leaves the
    // graft rows is a record the graft did not forward, and refuses.
    let killed = KilledSplits::of(naming);
    let chase_b = |e: EdgeKey| -> Result<EdgeKey, NamingError> {
        let named_in_b = |k: &EdgeKey| b.table.name_of(&ent(0, EntityKey::Edge(*k))).is_some();
        let root = body.split_root(e, |r| lineage_inv.get(&r).is_none_or(named_in_b))?;
        let kb = lineage_inv
            .get(&root)
            .copied()
            .ok_or(bug("a grafted edge's split lineage left the graft rows"))?;
        chase_edge_to_table(b.body, b.table, &killed.graft_b, kb)
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
        let (side, space) = operand_key(naming, inv_edges, e);
        let (op, k) = side.of(a, b);
        let root_key = match space {
            KeySpace::Arena => chase_edge_to_table(body, op.table, &killed.arena, k)?,
            KeySpace::Graft => chase_b(e)?,
        };
        let root = side.with(root_key);
        // The chase reads every split the reduction made, a killed
        // piece's included, so a root no operand table names was not
        // split off an operand edge: a join-minted crossing chord that
        // survived OUTSIDE the zip's list (channel cuts: chords on the
        // operand's own faces) — a DERIVED seam edge, named by its
        // adjacent faces' descent like any seam. So is an edge a join
        // made that its operand edges do not cover (`joined_cover`).
        let (op, k) = root.of(a, b);
        let resolves =
            op.table.name_of(&ent(0, EntityKey::Edge(k))).is_some() && !joined_faces.contains(&e);
        if resolves {
            groups.entry(root).or_default().push(e);
        } else if naming.chart_cuts[usize::from(root.operand() == topo::Operand::B)].contains(&k) {
            // The output stage glues a chart cut away with the two
            // pieces of the face it divided; one left is a body the
            // emitter cannot read.
            return Err(bug("a chart cut survives in the result"));
        } else {
            match chord_kind(e)? {
                ChordKind::Cross(fa, fb, one) => {
                    add_seam(fa, fb, one, e)?;
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
    for (r, e) in zip_rims {
        groups.entry(r).or_default().push(e);
    }
    for (e, r) in joined_one {
        groups.entry(r).or_default().push(e);
    }
    let in_sets: BTreeSet<OpSide<EdgeKey>> = joined_sets.keys().flatten().copied().collect();
    let mut out = Vec::with_capacity(seam_groups.len() + groups.len() + joined_sets.len());
    for (set, edges) in joined_sets {
        let (base, from_tie) = set_name(node, &set, a, b)?;
        // Keyed by the operand edges themselves, so a tied set's group
        // is one candidate's.
        rec.record(
            &base,
            edges.iter().map(|&e| ent(0, EntityKey::Edge(e))).collect(),
            Parent::Elsewhere,
        );
        out.push(EdgeGroup {
            base,
            from_tie,
            edges,
            set,
            root: None,
            side: None,
            lone: Lone::Whole,
        });
    }
    for ((fa, fb, _), (from_tie, one, edges)) in seam_groups {
        let base = name1(EntityKind::Edge, node, RoleSeg::Seam { a: fa, b: fb });
        rec.record(
            &base,
            edges.iter().map(|&e| ent(0, EntityKey::Edge(e))).collect(),
            Parent::Elsewhere,
        );
        out.push(EdgeGroup {
            base,
            from_tie,
            edges,
            set: Vec::new(),
            root: None,
            side: one,
            lone: Lone::Whole,
        });
    }
    for (root, edges) in groups {
        let (op, root_key) = root.of(a, b);
        let inner = upstream_name(op.table, op.node, ent(0, EntityKey::Edge(root_key)))?;
        let base = name1(EntityKind::Edge, node, root.wrap(a, b, inner.name));
        // Undivided: the one edge runs between the operand edge's own two
        // ends, as the operand's keys read the result's vertices there.
        let whole = match edges.as_slice() {
            [one] => {
                let (r0, r1) = edge_ends(op.body, root_key);
                let (e0, e1) = edge_ends(body, *one);
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
            root: Some(root_key),
            side: Some(root.operand()),
            lone,
        });
    }
    Ok((out, side_holds))
}

/// A seam side's name → the operand faces the parent it names holds:
/// one for an unmerged face's parent, a merged face's every constituent.
type SideHolds = BTreeMap<NameRef, Vec<OpSide<FaceKey>>>;

/// What an edge the output stage's joins made lies along.
enum JoinedCover {
    /// Along no operand edges that cover it: named from its two faces,
    /// as a seam is (`ChordKind`).
    Faces,
    /// Within one operand edge, a piece of it.
    One(OpSide<EdgeKey>),
    /// Along several operand edges that together cover it and none of
    /// which holds it whole: named for the set.
    Set(Vec<OpSide<EdgeKey>>),
}

/// **The operand edges joined edge `e` lies along** (`names/README.md`,
/// "Flush edges"): the edges of either operand between two faces that
/// `e`'s two faces descend from ([`joined_candidates`]) that lie on
/// `e`'s carrier and overlap it over a length ([`joined_along`]). One
/// that holds `e` whole makes
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
    let Some(faces) = inc.edge_faces.get(&e) else {
        unreachable!(
            "joined edge {e:?} bounds no face: the incidence read every half-edge of the result, \
             and in a tier-1-valid body each live edge has two half-edges, each in a face's loop"
        )
    };
    let [f0, f1] = faces.as_slice() else {
        return Err(NamingError::UnsupportedSeamSides {
            edge: e,
            shape: SeamSidesShape::NotTwoFaces,
        });
    };
    let descents = |f: FaceKey| -> Result<Vec<OpSide<FaceKey>>, NamingError> {
        Ok(match merged_descents.get(&f) {
            Some(ds) => ds.clone(),
            None => vec![descend_face(f)?],
        })
    };
    let operand = |ds: Vec<OpSide<FaceKey>>| -> Vec<(topo::Operand, FaceKey)> {
        ds.into_iter().map(OpSide::of_operand).collect()
    };
    let (d0, d1) = (operand(descents(*f0)?), operand(descents(*f1)?));
    let candidates = joined_candidates(
        &d0,
        &d1,
        |side| Ok(OpSide::on(side, ()).of(a, b).0.body),
        is_straight(body, e),
    )?;
    let cover = joined_along(
        body,
        e,
        candidates
            .into_iter()
            .map(|((side, r), arc)| (OpSide::on(side, r), arc)),
        bnd,
    )?;
    if let Some(&r) = cover.within.first() {
        return Ok(JoinedCover::One(r));
    }
    Ok(if cover.covered {
        JoinedCover::Set(cover.along)
    } else {
        JoinedCover::Faces
    })
}

/// The name of a joined edge that lies along the operand edges `set`
/// (several, `joined_one` holding the single ones): `Merged` of their
/// names, each wrapped by its side, and flat — an operand edge that is
/// itself a set stands for its constituents (N3). Whether any of them
/// is tied comes with it (`join_names::joined_name`).
fn set_name<T: Decide>(
    node: RecipeNodeId,
    set: &[OpSide<EdgeKey>],
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
) -> Result<(StableName, bool), NamingError> {
    let mut names = Vec::with_capacity(set.len());
    for &r in set {
        let (op, k) = r.of(a, b);
        let up = upstream_name(op.table, op.node, ent(0, EntityKey::Edge(k)))?;
        names.push((
            name1(EntityKind::Edge, node, r.wrap(a, b, up.name)),
            up.tied,
        ));
    }
    Ok(super::join_names::joined_name(node, names))
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
    /// The operand edge a carried edge descends from, in its operand's
    /// keys; `None` for a seam or a joined set.
    root: Option<EdgeKey>,
    /// The operand a carried edge descends from, or a seam's both faces
    /// do; `None` for any other seam or a joined set. Its name's read is
    /// no side: both seats may read one family (`xs[1] − xs[0]`), and the
    /// read keys the name either way (REFERENCES DM4).
    side: Option<topo::Operand>,
    /// What a lone piece of the parent is named for.
    lone: Lone,
}

/// Boolean vertices: operand pass-downs (`From`, + `Fragment(Borders)`
/// at each of several vertices one corner is held as: null copies and
/// the zip's copies per cone, [`Fused`]), and seam
/// (crossing/fused) vertices named by the operand entities whose
/// crossing minted them — derived from the already-named incident edges
/// (combinatorial wiring facts) — and, where an edge crosses, by its
/// sense ([`senses_at`]): `Crossing`, `EdgeCrossing`, or `Seam` for
/// every other meeting.
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
    side_holds: &SideHolds,
    corner_walls: &impl Fn(
        &BTreeSet<OpSide<VertexKey>>,
        &[VertexKey],
    ) -> Result<BTreeMap<Vec<StableName>, Vec<VertexKey>>, NamingError>,
    bnd: geom_core::Band,
) -> Result<(), NamingError> {
    let bug = |what| NamingError::Emission { what };
    // Each edge's parent, the head a vertex cites it by, whether that
    // descends from a tie, a joined edge's set, a carried edge's operand
    // edge, and its operand.
    type EdgeBase<'g> = (
        &'g StableName,
        bool,
        &'g [OpSide<EdgeKey>],
        Option<EdgeKey>,
        Option<topo::Operand>,
    );
    let edge_base: BTreeMap<EdgeKey, EdgeBase<'_>> = edge_groups
        .iter()
        .flat_map(|g| {
            g.edges
                .iter()
                .map(move |&e| (e, (&g.base, g.from_tie, g.set.as_slice(), g.root, g.side)))
        })
        .collect();
    // The operand edges of a joined edge's set that hold vertex `v`, each
    // with its name in its operand's table: A's where any does, else
    // B's. A vertex cites an edge it lies on, never the whole set (N2).
    let set_holding = |v: VertexKey,
                       set: &[OpSide<EdgeKey>]|
     -> Result<Vec<(OpSide<EdgeKey>, NameRef)>, NamingError> {
        let held = edges_holding(
            vertex_point(body, v),
            set.iter().map(|&r| {
                let (op, k) = r.of(a, b);
                (r, op.body, k)
            }),
            bnd,
        )?;
        let a_holds = held.iter().any(|r| matches!(r, OpSide::A(_)));
        let mut out = Vec::with_capacity(held.len());
        for r in held {
            if a_holds && matches!(r, OpSide::B(_)) {
                continue;
            }
            let (op, k) = r.of(a, b);
            let up = upstream_name(op.table, op.node, ent(0, EntityKey::Edge(k)))?;
            out.push((r, up.name));
        }
        Ok(out)
    };
    // An operand edge named as this node's table carries it in: the
    // `From` its operand's read came in through, on the line it lies on.
    let carried = |side: topo::Operand, e: &NameRef| -> NameRef {
        edge_line(&NameRef::new(name1(
            EntityKind::Edge,
            node,
            OpSide::on(side, ()).wrap(a, b, e.clone()),
        )))
    };
    // The zips' and the partings' fusions (`vertex_merges`): kept key →
    // dead partners (a fused vertex may owe
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
    // The vertex named as operand vertex `side`, its upstream `u`.
    let identity_of = |side: OpSide<VertexKey>, u: Upstream| {
        let parent = side.map(EntityKey::Vertex).parent();
        (
            name1(EntityKind::Vertex, node, side.wrap(a, b, u.name)),
            u.tied,
            parent,
            side,
        )
    };
    // Candidate seam-vertex names, grouped for multiplicity: the key
    // is the (A, B) parent pair, the value (descends-from-a-tie,
    // vertices).
    let mut groups: BTreeMap<RoleSeg, (bool, Vec<VertexKey>)> = BTreeMap::new();
    // Operand corners, each a candidate of its name → where it descends
    // from, the operand keys read there, and the result vertices that
    // are it: several where the boolean leaves several at one corner
    // (null copies, and the zip's copies per cone,
    // `BooleanNaming::cone_splits`). Keyed by the operand vertex it is
    // as well as its name, so a tied name's candidates are corners
    // apart, wherever they lie.
    struct Corner {
        parent: Parent,
        held: CornerCopies,
    }
    let mut corners: BTreeMap<(StableName, ParentEntities<OpSide<VertexKey>>), Corner> =
        BTreeMap::new();
    for (v, _) in body.vertices() {
        // A vertex that is one of an operand's own vertices is that
        // vertex (N2), read at every operand key the boolean leaves at
        // the point: the kept key, then its fusion partners nearest
        // first (`fused_partners`; A's where both operands fused here,
        // as `zip_seam` keeps the outer cycle's vertex), each a copy per
        // cone read as the vertex it copies (`Fused::origin`), then the rest
        // A's first (`operand_vertex_keys`), then the vertex the
        // reduction's contact rows pair one of them with
        // (`contact_identity`).
        let keys = operand_vertex_keys(naming, inv_vertices, &fused, v)?;
        let mut identity = None;
        let kept = core::iter::once(&v).chain(fused.partners.get(&v).into_iter().flatten());
        let mut order = Vec::new();
        for &k in kept {
            order.push(operand_key(naming, inv_vertices, fused.origin(k)).0);
        }
        order.extend(keys.iter().map(|&(o, k)| OpSide::on(o, k)));
        for side in order {
            if let Some(u) = named_in(side)? {
                identity = Some(identity_of(side, u));
                break;
            }
        }
        if identity.is_none()
            && let Some((side, u)) =
                contact_identity(v, &keys, &naming.reduction_contacts, named_in)?
        {
            identity = Some(identity_of(side, u));
        }
        if let Some((name, tied, parent, side)) = identity {
            let at = corners
                .entry((name.clone(), BTreeSet::from([BTreeSet::from([side])])))
                .or_insert_with(|| Corner {
                    parent,
                    held: CornerCopies {
                        name,
                        tied,
                        copies: Vec::new(),
                    },
                });
            at.held.tied |= tied;
            at.held.copies.push((v, ent(0, EntityKey::Vertex(v))));
            continue;
        }
        // What the boolean classified here: the senses a crossing is
        // named by.
        let at_v = senses_at(node, naming, v, &keys, a, b)?;
        // Seam vertex: parents from incident edges' names.
        let Some(edges) = inc.vertex_edges.get(&v) else {
            unreachable!(
                "seam vertex {v:?} starts no half-edge: the incidence read every half-edge of the \
                 result, and a boolean's result is a closed body, whose every vertex bounds edges"
            )
        };
        // The parentage a seam vertex is named from is read off the
        // incident edges' HEADS (N2: never a piece's qualifier, which is
        // named from its ends): each operand's edges by their names in
        // its own table, which the senses are read against,
        // and the seam lines by their two sides.
        let mut a_edges: Vec<NameRef> = Vec::new();
        let mut b_edges: Vec<NameRef> = Vec::new();
        // The operand edges at the vertex, in their operands' keys: what
        // a side holding faces of both operands is read against.
        let mut edge_keys: BTreeSet<OpSide<EdgeKey>> = BTreeSet::new();
        // The distinct seam LINES through this vertex: a set, because
        // several incident seam edges lie on one line. The junction's
        // name is these lines, and `names::canonical` orders them.
        let mut seam_lines: BTreeSet<(NameRef, NameRef)> = BTreeSet::new();
        // B1: a seam vertex reads its parentage off the incident edges'
        // parents, so a parent descended from a tie makes the vertex
        // name tie-descended too.
        let mut from_tie = false;
        for &e in edges {
            let Some(&(ename, tied, set, root, side)) = edge_base.get(&e) else {
                unreachable!(
                    "seam vertex {v:?} meets edge {e:?}, which no edge group holds: the edge pass \
                     puts every live edge of the result in a group or refuses"
                )
            };
            from_tie |= tied;
            match ename.path.first() {
                Some(RoleSeg::Merged(_)) => {
                    let held = set_holding(v, set)?;
                    if held.is_empty() {
                        return Err(NamingError::BandVerdictsDisagree {
                            vertex: v,
                            shape: DisagreeShape::VertexOffJoinedSet,
                        });
                    }
                    for (r, x) in held {
                        edge_keys.insert(r);
                        match r {
                            OpSide::A(_) => a_edges.push(x),
                            OpSide::B(_) => b_edges.push(x),
                        }
                    }
                }
                Some(RoleSeg::From { of: x, .. }) => {
                    let (Some(side), Some(k)) = (side, root) else {
                        return Err(bug("a carried edge's group names no operand"));
                    };
                    edge_keys.insert(OpSide::on(side, k));
                    match side {
                        topo::Operand::A => a_edges.push(x.clone()),
                        topo::Operand::B => b_edges.push(x.clone()),
                    }
                }
                // Zip-listed AND derived seams both qualify (M4 PR 5:
                // declared merges reroute channel-cut chords into the
                // derived-seam lane, so a seam vertex may lean on a
                // Seam-named edge outside `naming.seam_edges`) — the
                // NAME is the evidence either way.
                // A seam between two faces of one operand lies along the
                // other operand's edge that filled the gap between them,
                // so it ends at operand vertices, and it has no A and B
                // side to cross.
                Some(RoleSeg::Seam { .. }) if side.is_some() => {
                    return Err(NamingError::UnsupportedSeamVertex {
                        vertex: v,
                        shape: SeamVertexShape::OneOperandSeam,
                    });
                }
                Some(RoleSeg::Seam { a: s0, b: s1 }) => {
                    seam_lines.insert((s0.clone(), s1.clone()));
                }
                _ => unreachable!(
                    "seam vertex {v:?} meets edge {e:?} named {ename:?}: the edge pass bases each \
                     group on a `Merged` set or a `From` of the joined operand edges, a `Seam`, or \
                     a `From` of the operand edge it descends from"
                ),
            }
        }
        for list in [&mut a_edges, &mut b_edges] {
            list.sort_unstable();
            list.dedup();
        }
        // The sides across operand `o`'s edges here: of each seam line,
        // the side not on those edges' side, read off the sides' names.
        // A side holding faces of `o` only is on it and one holding none
        // of `o`'s is not, so a line with such a side has the other
        // across. Between two sides each holding faces of both operands,
        // the one on it is the one holding a face those edges bound.
        // `None` where a line has no one side across.
        let across = |o: topo::Operand| -> Result<Option<Vec<NameRef>>, NamingError> {
            let mut bounded: BTreeSet<OpSide<FaceKey>> = BTreeSet::new();
            for &r in edge_keys.iter().filter(|r| r.operand() == o) {
                let (op, k) = r.of(a, b);
                let (f0, f1) = live_edge_sides(
                    op.body,
                    k,
                    "its operand's table names it, and a table names its own body's live entities",
                );
                bounded.extend([r.with(f0), r.with(f1)]);
            }
            // Whether a side holds faces of `o`, and of the other operand.
            let holds = |s: &NameRef| -> Result<(bool, bool, bool), NamingError> {
                let Some(held) = side_holds.get(s) else {
                    unreachable!(
                        "seam side {s:?} holds no operand face: the edge pass records both sides \
                         of each seam it groups as it groups it"
                    )
                };
                Ok((
                    held.iter().any(|d| d.operand() == o),
                    held.iter().any(|d| d.operand() != o),
                    held.iter().any(|d| bounded.contains(d)),
                ))
            };
            let mut out = BTreeSet::new();
            for (s0, s1) in &seam_lines {
                let far = match (holds(s0)?, holds(s1)?) {
                    ((true, false, _), (true, false, _)) | ((false, _, _), (false, _, _)) => {
                        return Ok(None);
                    }
                    ((true, false, _), _) | (_, (false, _, _)) => s1,
                    (_, (true, false, _)) | ((false, _, _), _) => s0,
                    ((true, true, on0), (true, true, on1)) => match (on0, on1) {
                        (true, false) => s1,
                        (false, true) => s0,
                        (true, true) | (false, false) => return Ok(None),
                    },
                };
                out.insert(far.clone());
            }
            Ok(Some(out.into_iter().collect()))
        };
        // The one side across operand `o`'s edges, where there is one.
        let one_across = |o: topo::Operand| -> Result<Option<NameRef>, NamingError> {
            Ok(match across(o)?.as_deref() {
                Some([one]) => Some(one.clone()),
                _ => None,
            })
        };
        let sense = |side, edge: &NameRef| sense_of(v, &at_v, side, edge);
        // The line of operand `o`'s edges `on`, flush on one line, with
        // its sense: one edge's line, or `Merged` of the several, read
        // along each of them.
        let line = |o: topo::Operand, on: &[NameRef]| -> Result<(NameRef, Sense), NamingError> {
            let mut read = Vec::with_capacity(on.len());
            for e in on {
                read.push((carried(o, e), sense(o, e)?));
            }
            let name = line_name(node, read.iter().map(|(l, _)| l.clone()));
            Ok((name, sense_along_line(v, read)?))
        };
        // A crossing of operand `o`'s line `on` by the face across it.
        let crossing = |o: topo::Operand, on: &[NameRef], face: NameRef| {
            let (edge, sense) = line(o, on)?;
            Ok::<_, NamingError>(RoleSeg::Crossing { edge, sense, face })
        };
        // Several operand lines meeting: their run, in name order.
        let run = |lines: &[(topo::Operand, &[NameRef])]| -> Result<RoleSeg, NamingError> {
            let mut run = Vec::with_capacity(lines.len());
            for &(o, on) in lines {
                let (line, sense) = line(o, on)?;
                run.push(RunLine { line, sense });
            }
            run.sort();
            Ok(RoleSeg::EdgeCrossing(run))
        };
        let a_across = if a_edges.is_empty() {
            None
        } else {
            one_across(topo::Operand::A)?
        };
        let b_across = if b_edges.is_empty() {
            None
        } else {
            one_across(topo::Operand::B)?
        };
        // The vertex's head: an edge meeting a face is a crossing with
        // the edge's sense, two edges meeting a crossing with both
        // senses, and a point on one seam line a seam of its two faces.
        let only_line = match seam_lines.len() {
            1 => seam_lines.first(),
            _ => None,
        };
        let seg = match (
            a_edges.as_slice(),
            b_edges.as_slice(),
            only_line,
            &a_across,
            &b_across,
        ) {
            ([ae], [be], ..) => run(&[
                (topo::Operand::A, core::slice::from_ref(ae)),
                (topo::Operand::B, core::slice::from_ref(be)),
            ])?,
            // A pure seam-junction vertex (M4 PR 5: declared merges
            // can consume every operand-descended edge at a crossing
            // vertex): the incident seam edges' sides determine it when
            // they lie on ONE line.
            ([], [], Some((s0, s1)), ..) => RoleSeg::Seam {
                a: s0.clone(),
                b: s1.clone(),
            },
            ([ae], [], _, Some(face), _) => {
                crossing(topo::Operand::A, core::slice::from_ref(ae), face.clone())?
            }
            ([], [be], _, _, Some(face)) => {
                crossing(topo::Operand::B, core::slice::from_ref(be), face.clone())?
            }
            // A seam JUNCTION (M4 PR 5: declared merges can consume
            // every operand-descended edge at a crossing): the vertex
            // where k ≥ 2 seam LINES meet and no operand edge does. N2
            // counts a seam as a line of a vertex's name only at a union,
            // so here no line is an edge and the name is the path of the
            // lines' Seam segments, in the canonical form's order.
            ([], [], ..) if seam_lines.len() >= 2 => {
                let name = canonical::form(StableName {
                    kind: EntityKind::Vertex,
                    node,
                    path: seam_lines
                        .iter()
                        .map(|(s0, s1)| RoleSeg::Seam {
                            a: s0.clone(),
                            b: s1.clone(),
                        })
                        .collect(),
                });
                put(t, tie, from_tie, name, ent(0, EntityKey::Vertex(v)))?;
                continue;
            }
            // Operand edges where seam lines meet, no one edge pair being
            // the parentage: a pinch, where several edges of one operand
            // pierce a face of the other, or a point edges of both
            // operands run through, one of them on one side. N2 names it
            // by its lines: each side's edges, those flush on one line as
            // the set of them ([`flush_lines`]). One line and one face
            // of the other operand is that line's `Crossing`; two or more
            // lines are their run.
            (aes, bes, ..)
                if seam_lines.len() >= 2
                    && match (aes.len(), bes.len()) {
                        (n, 0) | (0, n) => n >= 1,
                        (1, _) | (_, 1) => true,
                        _ => false,
                    } =>
            {
                let a_lines = flush_lines(aes, a, bnd)?;
                let b_lines = flush_lines(bes, b, bnd)?;
                match (a_lines.as_slice(), b_lines.as_slice(), &a_across, &b_across) {
                    ([ae], [], Some(face), _) => crossing(topo::Operand::A, ae, face.clone())?,
                    ([], [be], _, Some(face)) => crossing(topo::Operand::B, be, face.clone())?,
                    (al, bl, ..) if al.len() + bl.len() >= 2 => {
                        let lines: Vec<(topo::Operand, &[NameRef])> = al
                            .iter()
                            .map(|on| (topo::Operand::A, on.as_slice()))
                            .chain(bl.iter().map(|on| (topo::Operand::B, on.as_slice())))
                            .collect();
                        run(&lines)?
                    }
                    // One line meeting several faces of the other
                    // operand and no edge of it. Several faces of one
                    // operand meet at a point only along that operand's
                    // edges, and with planar faces the region a pair
                    // keeps about the line either keeps a piece of such
                    // an edge at the point, holds one of its faces
                    // there, or pinches into two vertices, each one
                    // line meeting one face. Curved faces are outside
                    // that argument, so no rule names this vertex.
                    ([_], [], ..) | ([], [_], ..) => {
                        return Err(NamingError::UnsupportedSeamVertex {
                            vertex: v,
                            shape: SeamVertexShape::LineAcrossSeveralFaces,
                        });
                    }
                    (al, bl, ..) => unreachable!(
                        "seam vertex {v:?}'s operand edges hold no line ({al:?}, {bl:?}): the arm \
                         reads a side's edges only where it has one or more, and one line per \
                         run of flush edges always remains"
                    ),
                }
            }
            // Every other vertex is one no rule names: one operand
            // edge with no single face of the other operand across it
            // and fewer than two seam lines (an ordinary declared union
            // reached this on the A side before maximal edges joined its
            // vertex away,
            // `work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule.md`),
            // its mirror, several edges of each operand, or a vertex on
            // a seam between two faces of one operand. The body is sound
            // and nothing proves these shapes unreachable, so what is
            // missing is a rule.
            _ => {
                return Err(NamingError::UnsupportedSeamVertex {
                    vertex: v,
                    shape: SeamVertexShape::NoEdgePair,
                });
            }
        };
        // Vertices that share a name tie (N4).
        let slot = groups.entry(seg).or_insert((false, Vec::new()));
        slot.0 |= from_tie;
        slot.1.push(v);
    }
    let mut held = Vec::with_capacity(corners.len());
    for ((_, sides), c) in corners {
        let members = c.held.copies.iter().map(|&(_, e)| e).collect();
        rec.record(&c.held.name, members, c.parent);
        held.push((sides.into_iter().flatten().collect(), c.held));
    }
    name_corner_copies(t, tie, held, corner_walls)?;
    // A seam vertex is never qualified, so it forms no fragment group.
    for (seg, (from_tie, verts)) in groups {
        let base = name1(EntityKind::Vertex, node, seg);
        let ents: Vec<_> = verts
            .iter()
            .map(|&v| ent(0, EntityKey::Vertex(v)))
            .collect();
        mint_candidates(t, tie, from_tie, base, ents)?;
    }
    Ok(())
}

/// **The lines of one operand's edges at a seam vertex** (N2), each the
/// set of its flush edges ([`sets_per_line`]). An edge whose table does
/// not name it once is a line of its own.
fn flush_lines<T: Decide>(
    edges: &[NameRef],
    op: &OperandCtx<'_, T>,
    bnd: geom_core::Band,
) -> Result<Vec<Vec<NameRef>>, NamingError> {
    sets_per_line(
        edges.iter().cloned(),
        |e| match crossed_edge(e, op.table) {
            Some(k) => EdgeCarrier::of_edge(op.body, k).map(Some),
            None => Ok(None),
        },
        bnd,
    )
}

/// **Edges grouped by the line they lie on** (N2's lines at a vertex),
/// each line the set of its edges in order, the lines in the order of
/// their least edges: an edge lies on a line where it runs along the
/// carrier of the line's least edge through [`ON_MEMBER_EDGE`], a
/// straight edge on its straight line, a curved one on its certified
/// carrier ([`EdgeCarrier::runs_with`]). `line` reads an edge's carrier;
/// an edge it answers `None` for (the pair's edge its operand's table
/// does not name once, `flush_lines`), and an [`EdgeCarrier::Unread`]
/// one, is a line of its own. No edge on a line is chosen over another:
/// the pair boolean (`flush_lines`) and the union
/// (`emit_union::Flush::meeting`) cite each line as the set of them.
pub(super) fn sets_per_line<E: Ord, T: Decide>(
    edges: impl IntoIterator<Item = E>,
    mut line: impl FnMut(&E) -> Result<Option<EdgeCarrier<T>>, NamingError>,
    bnd: geom_core::Band,
) -> Result<Vec<Vec<E>>, NamingError> {
    let mut sorted: Vec<E> = edges.into_iter().collect();
    sorted.sort();
    let mut lines: Vec<(Vec<E>, Option<EdgeCarrier<T>>)> = Vec::with_capacity(sorted.len());
    for e in sorted {
        let on_line = line(&e)?;
        let mut on = None;
        if let Some(carrier) = &on_line {
            for (i, (_, of)) in lines.iter().enumerate() {
                let Some(of) = of else { continue };
                if of.runs_with(carrier, ON_MEMBER_EDGE, bnd)?.is_some() {
                    on = Some(i);
                    break;
                }
            }
        }
        match on {
            Some(i) => lines[i].0.push(e),
            None => lines.push((vec![e], on_line)),
        }
    }
    Ok(lines.into_iter().map(|(line, _)| line).collect())
}

/// **The name of a line of things flush on it** (N2): the one thing's
/// line, or `Merged` of the several, flat and in name order
/// ([`merged::edge_set`]). The pair boolean's lines of operand edges
/// and the union's of member edges are both named here. A line holds
/// one thing or more ([`sets_per_line`]).
pub(super) fn line_name(node: RecipeNodeId, on: impl IntoIterator<Item = NameRef>) -> NameRef {
    let mut on = on.into_iter();
    match (on.next(), on.next()) {
        (Some(one), None) => one,
        (first, second) => NameRef::new(merged::edge_set(
            node,
            first
                .into_iter()
                .chain(second)
                .chain(on)
                .map(|l| (*l).clone()),
        )),
    }
}

/// **The sense of a line at vertex `v`** (N2), from each thing on it,
/// by its line name, with its sense as [`EdgeReading::read`] reads it:
/// a flush edge's as its body stores the edge, a seam edge's crossing
/// as `Through` the side it enters along. A line of one thing reads as
/// that thing. A transversal crossing of several is `Through` every
/// thing whose direction runs into the body, in name order, and a touch
/// of several is their one touch.
///
/// # Errors
///
/// [`NamingError::BandVerdictsDisagree`] where one thing crosses and
/// another touches, or two touch from opposite sides. Each thing's
/// sense is its own band judgement, and the things are flush with each
/// other only within [`ON_MEMBER_EDGE`]. Exactly flush, they cannot
/// disagree: an operand edge's sense is the classes of its pieces on
/// either side of `v` ([`senses_at`]), and the boolean classes a piece
/// by where its points lie against the other operand's closed body (an
/// edge lying on the crossing face is `On`, so in it, whichever faces
/// the edge bounds). Two exactly coincident pieces beside `v` hold the
/// same points, so they read the same touch, or cross the same way up
/// to their stored directions, which a `Through` reads apart. A
/// union's member edge is read from pairwise judgements of the same
/// kind (`emit_union`'s `Flush::sense_against`). Only the band, which
/// can decide two nearly coincident pieces differently, splits them.
///
/// # Panics
///
/// Where `read` is empty: a line holds an edge or more
/// ([`sets_per_line`]).
pub(super) fn sense_along_line(
    v: VertexKey,
    read: impl IntoIterator<Item = (NameRef, Sense)>,
) -> Result<Sense, NamingError> {
    let disagree = || NamingError::BandVerdictsDisagree {
        vertex: v,
        shape: DisagreeShape::FlushSensesDisagree,
    };
    let mut read = read.into_iter();
    let Some(first) = read.next() else {
        unreachable!(
            "a line at vertex {v:?} holds nothing: `sets_per_line` makes each line of one edge or \
             more"
        )
    };
    let Some(second) = read.next() else {
        return Ok(first.1);
    };
    let mut touch: Option<Sense> = None;
    let mut crosses = false;
    let mut entering = Vec::new();
    for (line, sense) in [first, second].into_iter().chain(read) {
        match sense {
            t @ (Sense::Outside | Sense::Inside) => {
                if touch.as_ref().is_some_and(|was| *was != t) {
                    return Err(disagree());
                }
                touch = Some(t);
            }
            Sense::Enters => {
                crosses = true;
                entering.push(line);
            }
            Sense::Leaves => crosses = true,
            Sense::Through { entering: sides } => {
                crosses = true;
                entering.extend(sides);
            }
        }
    }
    // Each thing either touches or crosses, so a line with no touch
    // crosses.
    match touch {
        Some(t) if !crosses => Ok(t),
        Some(_) => Err(disagree()),
        None => {
            entering.sort();
            entering.dedup();
            Ok(Sense::Through { entering })
        }
    }
}

/// What a boolean fused, read once per boolean: each result vertex's
/// fusion partners ([`fused_partners`]) and every pair of operand keys
/// a null edge (`BooleanNaming::null_copies`) leaves at one point, both
/// ways round, and the vertex each of the zip's splits per cone
/// (`BooleanNaming::cone_splits`) copied. Both kinds of copy leave
/// several result vertices at one operand point: a null copy is read
/// through `one_point`, a copy per cone through [`Fused::origin`].
struct Fused {
    partners: BTreeMap<VertexKey, Vec<VertexKey>>,
    one_point: Vec<((topo::Operand, VertexKey), (topo::Operand, VertexKey))>,
    cones: topo::ConeCopies,
}

impl Fused {
    fn of(naming: &topo::BooleanNaming) -> Self {
        let mut fused = Self::from_rows(naming.vertex_merges.rows(), &naming.null_copies);
        fused.cones = naming.cone_splits.clone();
        fused
    }

    /// The result vertex `k` is a copy of: the vertex a split per cone
    /// copied it from, through every split, or `k` itself. A split's
    /// copy is minted on the result, so it is the operand vertex it
    /// copies, not a key of either operand. Total: the splits were
    /// checked as each was made, so they cannot cycle.
    fn origin(&self, k: VertexKey) -> VertexKey {
        self.cones.root(k)
    }

    /// From the zips' and the partings' fusions `(dead, kept)`
    /// (`vertex_merges`) and the null copies, each
    /// in the order made, with no copy per cone ([`Fused::of`] adds
    /// those).
    fn from_rows(
        merges: &[(VertexKey, VertexKey)],
        null_copies: &[(topo::Operand, VertexKey, VertexKey)],
    ) -> Self {
        let one_point = null_copies
            .iter()
            .copied()
            .flat_map(|(side, x, y)| [((side, x), (side, y)), ((side, y), (side, x))])
            .collect();
        Self {
            partners: fused_partners(merges),
            one_point,
            cones: topo::ConeCopies::new(topo::LineageRows::ConeCopies),
        }
    }
}

/// Each vertex the fusions `merges` (`(dead, kept)`, in the order
/// made) leave alive → every dead vertex fused into it through any
/// number of them, nearest first: the keys fused straight into it, then
/// those fused into one of them, each hop in the order they died. A
/// vertex's operand identity is the first of them its operand names, so
/// a key one fusion from the survivor wins over a key it absorbed
/// earlier, as when no chain formed. A partner may be a copy the zip
/// made per cone, which the caller reads as the vertex it copies
/// ([`Fused::origin`]).
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
/// ([`operand_key`]), each key fused into it, each read through the
/// copy per cone it may be ([`Fused::origin`]), and, transitively, every
/// key a null edge (`BooleanNaming::null_copies`) joins to any of
/// those: it leaves two keys of one point.
fn operand_vertex_keys(
    naming: &topo::BooleanNaming,
    inv_vertices: &BTreeMap<VertexKey, VertexKey>,
    fused: &Fused,
    v: VertexKey,
) -> Result<BTreeSet<(topo::Operand, VertexKey)>, NamingError> {
    let mut keys = BTreeSet::new();
    for &k in core::iter::once(&v).chain(fused.partners.get(&v).into_iter().flatten()) {
        keys.insert(
            operand_key(naming, inv_vertices, fused.origin(k))
                .0
                .of_operand(),
        );
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
    /// The sense; `None` for a transversal crossing of a seam edge no
    /// side of which the names settle ([`EdgeReading::read`]).
    sense: Option<Sense>,
}

/// **The senses at the result vertex whose operand keys are `keys`**
/// (N2, [`operand_vertex_keys`]): each operand edge with a piece the
/// kernel classified beside the vertex ([`topo::EdgePieceClass`]), with
/// its sense against the other operand's closed body along the edge as
/// its operand stores it, read as [`edge_reading`] says: a seam edge's
/// as the side it enters along, cited as this node cites the operand's
/// faces. A piece `In` or `On` lies in the closed body.
///
/// # Errors
///
/// [`NamingError::PartnerUnread`] where a key is one the boolean read no
/// partner of ([`topo::UnreadPartner`]), whose edges hold no class to
/// read a sense from; [`NamingError::UnsupportedCrossingSenses`] where one side of
/// the vertex holds pieces of one edge on both sides of the other
/// operand; [`NamingError::Emission`] where it holds no classified piece
/// ([`UNCLASSIFIED_SIDE`]).
///
/// # Panics
///
/// Where a classified edge is one its operand's table does not name.
fn senses_at<T: Decide>(
    node: RecipeNodeId,
    naming: &topo::BooleanNaming,
    v: VertexKey,
    keys: &BTreeSet<(topo::Operand, VertexKey)>,
    a: &OperandCtx<'_, T>,
    b: &OperandCtx<'_, T>,
) -> Result<Vec<EdgeSense>, NamingError> {
    // A key the boolean read no partner of holds no class, by the
    // boolean's own account: its edges would drop out of the senses
    // unseen, so the crossing refuses naming why instead. A listed key
    // is an operand's own vertex, which the identity pass names first
    // (`name_boolean_vertices`), so this is reached only where its
    // operand's table names no vertex at any key the result vertex
    // holds.
    if let Some(u) = naming
        .unread_partners
        .iter()
        .find(|u| keys.contains(&(u.operand, u.vertex)))
    {
        return Err(NamingError::PartnerUnread {
            vertex: v,
            partner: u.partner,
            unread: *u,
        });
    }
    // (side, edge) → (the sides of the vertex in the closed body,
    // before it and after it).
    let mut sides: BTreeMap<(topo::Operand, EdgeKey), [BTreeSet<bool>; 2]> = BTreeMap::new();
    // A chart cut is no edge of its operand, so no line of a vertex.
    let cut = |row: &topo::EdgePieceClass| {
        naming.chart_cuts[usize::from(row.operand == topo::Operand::B)].contains(&row.edge)
    };
    for row in &naming.edge_classes {
        if keys.contains(&(row.operand, row.vertex)) && !cut(row) {
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
        let Some(name) = op.table.name_of(&ent(0, EntityKey::Edge(edge))) else {
            unreachable!(
                "operand edge {edge:?} of {side:?}, classified at result vertex {v:?}, is not \
                 named: `topo::EdgePieceClass` holds an operand's own edge key, chart cuts \
                 aside, and an operand's table is total over its body"
            )
        };
        let name = name.clone();
        let sense = match (side_in_body(v, &before)?, side_in_body(v, &after)?) {
            (false, true) => Sense::Enters,
            (true, false) => Sense::Leaves,
            (false, false) => Sense::Outside,
            (true, true) => Sense::Inside,
        };
        let sense = edge_reading(op.body, op.table, edge, &name)
            .read(sense, |s| {
                NameRef::new(name1(
                    EntityKind::Face,
                    node,
                    OpSide::on(side, ()).wrap(a, b, NameRef::new(s.clone())),
                ))
            })
            .ok();
        out.push(EdgeSense { side, name, sense });
    }
    Ok(out)
}

/// **Whether one side of a vertex lies in the other operand's closed
/// body** along a crossed edge, from the classes of the edge's pieces
/// there (`true` for `In` or `On`).
///
/// # Errors
///
/// [`NamingError::UnsupportedCrossingSenses`] at `vertex` where its pieces there
/// read both ways; [`NamingError::Emission`] where no piece there was
/// classified ([`UNCLASSIFIED_SIDE`]).
fn side_in_body(vertex: VertexKey, classes: &BTreeSet<bool>) -> Result<bool, NamingError> {
    match classes.len() {
        0 => Err(NamingError::Emission {
            what: UNCLASSIFIED_SIDE,
        }),
        1 => Ok(classes.contains(&true)),
        _ => Err(NamingError::UnsupportedCrossingSenses {
            vertex,
            shape: CrossingSensesShape::PiecesBothSides,
        }),
    }
}

/// A crossed edge has a side at the crossing with no piece classified.
/// [`senses_at`] reads only a vertex no operand names, which lies inside
/// each edge it is on and so holds a piece of it on both sides, and the
/// boolean classes every piece at a vertex whose partner it read
/// ([`NamingError::PartnerUnread`] is the one it did not), so this is a
/// row the boolean owed. It stays typed rather than a panic: a legal
/// document reached its union twin (`work/borders/` row
/// `a-whole-ball-cut-by-a-tilted-brick-refuses-an-unclassified-member-edge-side`),
/// and the crossing's "no piece the boolean classified" is reached too,
/// so no one has shown a correct kernel never reaches it.
const UNCLASSIFIED_SIDE: &str = "a crossing's edge has a side the boolean classified no piece of: \
                                 a vertex no operand names lies inside each edge it is on, and the \
                                 boolean classes the pieces on both sides of it";

#[cfg(test)]
mod side_tests {
    #![allow(clippy::unwrap_used)]

    use std::collections::BTreeSet;

    use super::{VertexKey, side_in_body};

    /// **A side of a crossing with no classified piece refuses typed**
    /// (N2): a vertex that is not an operand's own holds the edge on
    /// both sides, so a missing class is a row the boolean owed.
    #[test]
    fn a_side_with_no_classified_piece_refuses() {
        assert!(matches!(
            side_in_body(VertexKey::default(), &BTreeSet::new()),
            Err(super::NamingError::Emission { what }) if what == super::UNCLASSIFIED_SIDE
        ));
    }

    /// One class reads its side; two are a crossing no rule reads, at
    /// the vertex asked about.
    #[test]
    fn a_side_reads_its_one_class() {
        let v = VertexKey::default();
        assert!(side_in_body(v, &BTreeSet::from([true])).unwrap());
        assert!(!side_in_body(v, &BTreeSet::from([false])).unwrap());
        assert!(matches!(
            side_in_body(v, &BTreeSet::from([true, false])),
            Err(super::NamingError::UnsupportedCrossingSenses {
                vertex,
                shape: super::CrossingSensesShape::PiecesBothSides,
            }) if vertex == v
        ));
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

    /// **A copy of a null copy of the vertex is the vertex too**: one
    /// null edge leaves `kb` a copy `kc`, and another leaves `kc` a
    /// copy `w`, as a ring's struts hang from one another.
    #[test]
    fn a_chain_of_null_copies_is_the_vertex() {
        let (r, kb, kc, w) = (vk(10), vk(1), vk(2), vk(3));
        let fused = Fused::from_rows(&[], &[(B, kb, kc), (B, kc, w)]);
        let mut want = vec![kb, kc, w];
        want.sort_unstable();
        assert_eq!(b_keys_of(&fused, r, kb), want, "the copies");
    }
}

/// **The sense at a vertex of `side`'s edge `edge`**, among the
/// vertex's [`senses_at`].
///
/// # Errors
///
/// [`NamingError::UnsupportedCrossingSenses`] at `vertex` where edges sharing its
/// name read different senses; [`NamingError::Emission`] where no piece
/// of the edge is classified at the vertex, or where the edge is a seam
/// it crosses whose side it enters along the names do not settle
/// ([`UNSIDED`]).
fn sense_of(
    vertex: VertexKey,
    senses: &[EdgeSense],
    side: topo::Operand,
    edge: &StableName,
) -> Result<Sense, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let mut read = senses.iter().filter(|s| s.side == side && s.name == *edge);
    let first = read.next().ok_or(bug(
        "a crossing's edge has no piece the boolean classified at the crossing",
    ))?;
    if read.any(|s| s.sense != first.sense) {
        return Err(NamingError::UnsupportedCrossingSenses {
            vertex,
            shape: CrossingSensesShape::NameReadsTwoSenses,
        });
    }
    first.sense.clone().ok_or(bug(UNSIDED))
}

/// **The operand vertex the reduction's contact rows pair result
/// vertex `vertex` with**, where no operand names any of its own keys
/// `keys` ([`operand_vertex_keys`]): the other operand's vertex in
/// every vertex-pair row (`ReductionContacts::vv`) that holds one of
/// them. A vertex-on-face row (`a_on_b`, `b_on_a`) is not read: it
/// holds the resting vertex's own key and a face, no second vertex key
/// at the point to pair with, so it can name a result vertex only
/// through that key, which `keys` already reads.
fn contact_identity(
    vertex: VertexKey,
    keys: &BTreeSet<(topo::Operand, VertexKey)>,
    contacts: &topo::ReductionContacts,
    named_in: impl Fn(OpSide<VertexKey>) -> Result<Option<Upstream>, NamingError>,
) -> Result<Option<(OpSide<VertexKey>, Upstream)>, NamingError> {
    let mut named = Vec::new();
    for row in &contacts.vv {
        let partner = if keys.contains(&(topo::Operand::A, row.a)) {
            OpSide::B(row.b)
        } else if keys.contains(&(topo::Operand::B, row.b)) {
            OpSide::A(row.a)
        } else {
            continue;
        };
        if let Some(u) = named_in(partner)? {
            named.push((partner, u));
        }
    }
    one_partner(vertex, named)
}

/// The one contact-record partner a vertex is named by, from the
/// named partners of EVERY row that pairs it: none is no partner, one
/// name — however many rows carry it — is that partner (tie-descended
/// if any row's is), and several distinct names refuse
/// ([`NamingError::SeamVertexPartners`]) rather than choose.
fn one_partner<K: Copy>(
    vertex: VertexKey,
    named: Vec<(OpSide<K>, Upstream)>,
) -> Result<Option<(OpSide<K>, Upstream)>, NamingError> {
    let mut distinct: Vec<(OpSide<K>, Upstream)> = Vec::new();
    for (side, u) in named {
        match distinct
            .iter_mut()
            .find(|(s, d)| s.operand() == side.operand() && d.name == u.name)
        {
            Some((_, d)) => d.tied |= u.tied,
            None => distinct.push((side, u)),
        }
    }
    if distinct.len() > 1 {
        let mut candidates: Vec<StableName> =
            distinct.iter().map(|(_, u)| (*u.name).clone()).collect();
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
    edge: EdgeKey,
    q0: Point3<T>,
    q1: Point3<T>,
    d: Vec3<T>,
    len: T,
}

impl<T: Decide> Segment<T> {
    /// Edge `e` of `body`, from its `he_plus` start to its end.
    #[track_caller]
    pub(super) fn of_edge(body: &Body<T>, e: EdgeKey) -> Self {
        let (v0, v1) = edge_ends(body, e);
        let (q0, q1) = (vertex_point(body, v0), vertex_point(body, v1));
        let d = q1 - q0;
        Segment {
            edge: e,
            q0,
            q1,
            d,
            len: d.norm(),
        }
    }

    /// Whether segment `other` runs the same way as this one along
    /// this one's line (`Some(true)`), the other way (`Some(false)`), or
    /// does not lie on it (`None`): its two ends on the line, then the
    /// sign of the two directions' dot, each through `predicate`.
    ///
    /// # Errors
    ///
    /// [`NamingError::Escalated`] where a verdict falls in the band;
    /// [`NamingError::WayInZeroBand`] where the dot decides zero. A
    /// certified edge spans at least the escalate band, K zero bands, but
    /// its ends may each lie up to a zero band off this line, so for a
    /// small enough K (any finite K > 1 is accepted) its projection on the
    /// line can fall in the zero band.
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
            Sign::Zero => Err(NamingError::WayInZeroBand {
                edge: other.edge,
                shape: WayShape::Line,
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
#[derive(Clone, Debug)]
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
/// greatest speed over every parameter
/// ([`geom::carrier_speed::speed_range`]), so no verdict depends on
/// which edge of the carrier reads it, and a stretch decided zero is
/// within the zero band in metres. A closed edge is its carrier's whole
/// period: nothing [`Track::cover`] says of it depends on where its
/// vertex sits, which has no identity of its own (`docs/DESIGN.md`,
/// maximal edges).
pub(super) struct Track<T: Decide> {
    edge: EdgeKey,
    carrier: geom::Curve3<T>,
    t0: T,
    mid: T,
    len: T,
    period: Option<T>,
    closed: bool,
    rate: geom_core::SupSpeed<T>,
}

impl<T: Decide> Track<T> {
    /// Edge `e` of `body` along its certified carrier, from its
    /// `he_plus` start to its end.
    pub(super) fn of_edge(body: &Body<T>, e: EdgeKey) -> Result<Self, NamingError> {
        let curve = certified_curve(body, e);
        let (v0, v1) = edge_ends(body, e);
        Self::on(e, curve.carrier().clone(), curve.params(), v0 == v1)
    }

    /// Edge `e` on `carrier` over `(t0, t1)`, `closed` when one vertex
    /// ends it at both ends.
    fn on(
        e: EdgeKey,
        carrier: geom::Curve3<T>,
        (t0, t1): (T, T),
        closed: bool,
    ) -> Result<Self, NamingError> {
        let period = match carrier {
            geom::Curve3::Circle { .. }
            | geom::Curve3::Ellipse { .. }
            | geom::Curve3::Spiric { .. } => Some(T::from_f64(std::f64::consts::TAU)),
            _ => None,
        };
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
        let rate = geom::carrier_speed::speed_range(&carrier).1;
        Ok(Track {
            edge: e,
            carrier,
            t0,
            mid,
            len,
            period,
            closed,
            rate,
        })
    }

    /// The sign of the stretch `x` along the carrier, metered by its
    /// greatest speed.
    fn sign(
        &self,
        x: T,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Sign, NamingError> {
        decide(predicate, Margin::metered_sup(x, self.rate), bnd)
            .map_err(|source| NamingError::Escalated { predicate, source })
    }

    /// `p`'s offset from the start along the carrier, where `p` lies on
    /// the carrier (its distance from the carrier's point at its
    /// recovered parameter decided zero through `predicate`); `None` off
    /// it. On a periodic carrier the offset lies in `[0, period)`: zero
    /// at the start, decided first, and otherwise read from the point
    /// half a period on ([`geom::Curve3::param_near`] anchored there)
    /// and reduced into one period ([`Track::forward`]), so the one
    /// point whose reading the anchor cuts, and where the reduction
    /// jumps, is the start.
    /// A closed arc's whole turn is `arc.closed`, which
    /// [`Self::stretches`] answers from whatever its offsets read.
    ///
    /// # Errors
    ///
    /// [`NamingError::CarrierUnplaced`] on a carrier that recovers no
    /// parameter for a point (a spline), where "off it" would be a
    /// guess.
    fn offset(
        &self,
        p: Point3<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Option<T>, NamingError> {
        let gap = |q: Point3<T>| {
            decide(predicate, Margin::of((p - q).norm()), bnd)
                .map_err(|source| NamingError::Escalated { predicate, source })
        };
        let anchor = match self.period {
            Some(per) => {
                if gap(self.carrier.eval(self.t0))? == Sign::Zero {
                    return Ok(Some(T::zero()));
                }
                self.t0 + per * T::from_f64(0.5)
            }
            None => self.mid,
        };
        let u = self.place(p, anchor)?;
        if gap(self.carrier.eval(u))? != Sign::Zero {
            return Ok(None);
        }
        // `param_near` answers within half a period of its anchor, so
        // `u − t0` lies in `(0, period]`, and the reduction moves only
        // the start's own reading, which is decided above.
        Ok(Some(self.forward(u - self.t0)))
    }

    /// **Where `p` lies along the carrier**: its parameter within half
    /// a period of `anchor` ([`geom::Curve3::param_near`]), the one
    /// placement every reading along a curved edge takes.
    ///
    /// # Errors
    ///
    /// [`NamingError::CarrierUnplaced`] on a carrier that recovers no
    /// parameter for a point (a spline).
    fn place(&self, p: Point3<T>, anchor: T) -> Result<T, NamingError> {
        self.carrier
            .param_near(p, anchor)
            .ok_or(NamingError::CarrierUnplaced {
                edge: self.edge,
                carrier: self.carrier.kind(),
            })
    }

    /// `x` reduced into `[0, period)` on a periodic carrier, unchanged
    /// on another: a forward stretch along the carrier.
    fn forward(&self, x: T) -> T {
        match self.period {
            Some(per) => x - per * (x / per).floor(),
            None => x,
        }
    }

    /// **Where `arc` lies along the carrier**: the offsets of the end
    /// it starts at and of its other end, in the carrier's direction
    /// and the second past the first, and whether it starts at
    /// `arc.p1`. On a periodic carrier the arc runs from the end it
    /// starts at through its middle. `None` where the arc is off the
    /// carrier.
    fn span(
        &self,
        arc: &CarrierArc<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Option<(T, T, bool)>, NamingError> {
        let (Some(a0), Some(am), Some(a1)) = (
            self.offset(arc.p0, predicate, bnd)?,
            self.offset(arc.mid, predicate, bnd)?,
            self.offset(arc.p1, predicate, bnd)?,
        ) else {
            return Ok(None);
        };
        let Some(per) = self.period else {
            return Ok(Some(match self.sign(a1 - a0, predicate, bnd)? {
                Sign::Negative => (a1, a0, true),
                _ => (a0, a1, false),
            }));
        };
        let (d1, dm) = (self.forward(a1 - a0), self.forward(am - a0));
        // The middle is half the arc from either end, so this verdict
        // is never near its band.
        Ok(Some(match self.sign(d1 - dm, predicate, bnd)? {
            Sign::Positive => (a0, a0 + d1, false),
            _ => (a1, a1 + per - d1, true),
        }))
    }

    /// The stretches of offsets `arc` holds ([`Track::span`]): one on
    /// an open carrier; on a periodic one the arc and that arc a period
    /// back, so a stretch through the start reads whole on one side or
    /// the other, and a closed arc's whole period. `None` where the arc
    /// is off the carrier.
    fn stretches(
        &self,
        arc: &CarrierArc<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Option<Vec<(T, T)>>, NamingError> {
        let Some((lo, hi, _)) = self.span(arc, predicate, bnd)? else {
            return Ok(None);
        };
        Ok(Some(match self.period {
            None => vec![(lo, hi)],
            Some(per) if arc.closed => vec![(T::zero(), per), (T::zero() - per, T::zero())],
            Some(per) => vec![(lo, hi), (lo - per, hi - per)],
        }))
    }

    /// Whether `p` lies on this edge: on its carrier at an offset within
    /// its span, the ends included. A closed edge holds every point of
    /// its carrier, and a point just short of a periodic edge's start
    /// reads a period on, at the start.
    fn holds(
        &self,
        p: Point3<T>,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<bool, NamingError> {
        let Some(u) = self.offset(p, predicate, bnd)? else {
            return Ok(false);
        };
        if self.closed {
            return Ok(true);
        }
        if let Some(per) = self.period
            && self.sign(per - u, predicate, bnd)? == Sign::Zero
        {
            return Ok(true);
        }
        Ok(self.sign(u, predicate, bnd)? != Sign::Negative
            && self.sign(self.len - u, predicate, bnd)? != Sign::Negative)
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

/// **An edge's line for the flush readings**: its [`Segment`] where it
/// is straight ([`is_straight`]), its [`Track`] along its certified
/// carrier where it is curved, and [`EdgeCarrier::Unread`] where that
/// carrier places no point.
pub(super) enum EdgeCarrier<T: Decide> {
    Straight(Segment<T>),
    Curved(Track<T>),
    /// A curved edge whose carrier recovers no parameter for a point
    /// (a spline: [`geom::Curve3::param_near`] answers `None`), so no
    /// reading along it can say whether a point lies on it. It is a
    /// line of its own and runs with nothing, and a union's joined edge
    /// on it covers nothing (`emit_union::Flush::edge_cover`): the
    /// readings refuse nothing they would not refuse on a straight
    /// edge, at the cost of not counting such edges flush
    /// (`work/splice/a-unions-flush-reading-cannot-follow-a-nurbs-carrier.md`).
    Unread,
}

impl<T: Decide> EdgeCarrier<T> {
    /// Edge `e` of `body`, from its `he_plus` start to its end.
    pub(super) fn of_edge(body: &Body<T>, e: EdgeKey) -> Result<Self, NamingError> {
        if is_straight(body, e) {
            return Ok(EdgeCarrier::Straight(Segment::of_edge(body, e)));
        }
        if !places_points(certified_curve(body, e).carrier()) {
            return Ok(EdgeCarrier::Unread);
        }
        Ok(EdgeCarrier::Curved(Track::of_edge(body, e)?))
    }

    /// Whether `other` lies on this edge's line, and if so whether the
    /// two run the same way along it: [`Segment::runs_with`] for two
    /// straight edges, [`Track::runs_with`] for two curved ones. A
    /// straight edge lies on no curved one's carrier, nor a curved one
    /// on a straight one's line, and an [`EdgeCarrier::Unread`] edge
    /// lies on no other's line, nor another on its.
    pub(super) fn runs_with(
        &self,
        other: &Self,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Option<bool>, NamingError> {
        match (self, other) {
            (EdgeCarrier::Straight(a), EdgeCarrier::Straight(b)) => a.runs_with(b, predicate, bnd),
            (EdgeCarrier::Curved(a), EdgeCarrier::Curved(b)) => a.runs_with(b, predicate, bnd),
            (EdgeCarrier::Unread, _) | (_, EdgeCarrier::Unread) => Ok(None),
            (EdgeCarrier::Straight(_), EdgeCarrier::Curved(_))
            | (EdgeCarrier::Curved(_), EdgeCarrier::Straight(_)) => Ok(None),
        }
    }
}

/// Whether `carrier` recovers a parameter for a point
/// ([`geom::Curve3::param_near`]): every kind but a spline.
fn places_points<T: Decide>(carrier: &geom::Curve3<T>) -> bool {
    let origin = Point3::new(T::zero(), T::zero(), T::zero());
    carrier.param_near(origin, T::zero()).is_some()
}

/// Edge `e`'s certified curve in `body` ([`Body::stored_curve`]).
///
/// # Panics
///
/// Where `e` is stale or a null edge: every caller reads an edge it
/// holds live, off `body` or a table total over it, and tier 2 refuses
/// a null edge at rest (`topo::validate`'s `ScaffoldAtRest`).
#[track_caller]
fn certified_curve<T: Decide>(body: &Body<T>, e: EdgeKey) -> &geom_brep::EdgeCurve<T> {
    match body.stored_curve(e) {
        Ok(curve) => curve,
        Err(err) => unreachable!(
            "edge {e:?} has no certified curve ({err:?}): its caller holds it live in a body at \
             rest, where tier 2 refuses a null edge"
        ),
    }
}

impl<T: Decide> Track<T> {
    /// Whether edge `other` lies on this edge's carrier, and if so
    /// whether the two run the same way along it, read off `other`'s
    /// points a quarter, half and three quarters of the way along it:
    /// it lies on the carrier when all three do ([`Track::offset`]),
    /// and runs this edge's way when, going forward along the carrier
    /// from the first, the second comes before the third. The two
    /// stretches compared differ by a quarter of `other`'s length in
    /// metres. Certification refuses an edge whose span is under the
    /// escalate band (`interval_span_forward`), so that quarter is at
    /// least a quarter of the escalate band, K/4 zero bands: for an edge
    /// under four escalate bands long the verdict may escalate, and at an
    /// ambiguity factor K ≤ 4 (any finite K > 1 is accepted) it may
    /// decide zero.
    ///
    /// # Errors
    ///
    /// As [`Track::offset`]; [`NamingError::Escalated`] where the band
    /// cannot decide the two stretches; [`NamingError::WayInZeroBand`]
    /// where it decides them equal.
    pub(super) fn runs_with(
        &self,
        other: &Self,
        predicate: &'static str,
        bnd: geom_core::Band,
    ) -> Result<Option<bool>, NamingError> {
        if self.carrier.kind() != other.carrier.kind() {
            return Ok(None);
        }
        let mut at = Vec::with_capacity(3);
        for f in [T::from_f64(0.25), T::from_f64(0.5), T::from_f64(0.75)] {
            let t = other.t0 + other.len * f;
            match self.offset(other.carrier.eval(t), predicate, bnd)? {
                Some(o) => at.push(o),
                None => return Ok(None),
            }
        }
        let ahead = self.forward(at[2] - at[0]) - self.forward(at[1] - at[0]);
        match self.sign(ahead, predicate, bnd)? {
            Sign::Positive => Ok(Some(true)),
            Sign::Negative => Ok(Some(false)),
            Sign::Zero => Err(NamingError::WayInZeroBand {
                edge: other.edge,
                shape: WayShape::Carrier,
            }),
        }
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
    let seg = Segment::of_edge(op_body, rim);
    let (c0, c1) = edge_ends(body, chord);
    for v in [c0, c1] {
        if seg.place(vertex_point(body, v), CHORD_ON_RIM, bnd)? == OnSegment::Off {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Whether result edge `chord` lies within operand edge `rim` of
/// `op_body`, read along their common carrier: a straight chord within
/// a straight rim when both its ends lie on the rim's segment
/// ([`chord_on_rim`]); a curved one within a curved rim when it lies
/// within the rim along the chord's carrier ([`Track::cover`]), the
/// reading a pair boolean's joined edge takes on a curved carrier
/// (`names/README.md`, "Flush edges"). A straight chord lies within no
/// curved rim, nor a curved chord within a straight one.
fn chord_within<T: Decide>(
    body: &Body<T>,
    chord: EdgeKey,
    op_body: &Body<T>,
    rim: EdgeKey,
    bnd: geom_core::Band,
) -> Result<bool, NamingError> {
    match (is_straight(body, chord), is_straight(op_body, rim)) {
        (true, true) => chord_on_rim(body, chord, op_body, rim, bnd),
        (false, false) => Ok(!Track::of_edge(body, chord)?
            .cover([(rim, carrier_arc(op_body, rim))], CHORD_ON_RIM, bnd)?
            .within
            .is_empty()),
        _ => Ok(false),
    }
}

/// One operand's reading of a chord between two merged faces: the
/// operand, and the faces of it each merged face holds.
type MergedSides<'o, 'a, T> = (
    topo::Operand,
    &'o OperandCtx<'a, T>,
    &'o [FaceKey],
    &'o [FaceKey],
);

/// **The rim a chord between two merged faces lies within**, read
/// against every rim of each operand between a face one merged face
/// holds and a face the other does ([`rim_among`]), for `sides`: each
/// operand with the faces of it the two hold. One rim holding it is the
/// answer; `None` where none does. Neither depends on the order of the
/// faces or of their keys.
///
/// # Errors
///
/// As [`rim_among`] refuses, and a pair whose reading escalated where
/// no rim holds the chord, that pair's escalation.
///
/// # Panics
///
/// Where a rim of each operand holds the chord, a kernel bug.
fn merged_chord_rim<T: Decide>(
    body: &Body<T>,
    chord: EdgeKey,
    sides: [MergedSides<'_, '_, T>; 2],
    bnd: geom_core::Band,
) -> Result<Option<(topo::Operand, EdgeKey)>, NamingError> {
    let mut held = Vec::new();
    let mut escalated = None;
    for (side, op, c0, c1) in sides {
        let pairs: Vec<(FaceKey, FaceKey)> = c0
            .iter()
            .flat_map(|&f0| c1.iter().map(move |&f1| (f0, f1)))
            .collect();
        if pairs.is_empty() {
            continue;
        }
        match rim_among(body, chord, op, &pairs, bnd)? {
            RimPick::Holds(rim) => held.push((side, rim)),
            RimPick::Several(err) => return Err(err),
            RimPick::Escalated(err) => {
                escalated.get_or_insert(err);
            }
            // Each merged face holds faces of both operands, so the chord
            // may lie within neither's rims: it may cross between the
            // parents. A side whose pairs it lies off, or whose pairs
            // share no one rim, says only that it is not that side's rim.
            RimPick::Off | RimPick::NoRim(_) => {}
        }
    }
    match (held.as_slice(), escalated) {
        ([one], _) => Ok(Some(*one)),
        ([], Some(err)) => Err(err),
        ([], None) => Ok(None),
        _ => unreachable!(
            "chord {chord:?} between two merged faces lies within a rim of each operand \
             ({held:?}): two operand edges flush along one line are carried as a piece of one \
             of them, and an edge the join made over them is named by `joined_cover` before \
             any chord is read, so no zip-listed or derived chord lies along both"
        ),
    }
}

/// What a chord between faces of one operand is ([`one_operand_chord`]).
enum OneOperand {
    /// A piece of that operand's rim.
    Rim(EdgeKey),
    /// The seam of the two faces' parents: a union's gap.
    Gap,
}

/// **What a chord between faces of one operand reads as**, from its
/// [`RimPick`] over the pairs it may lie between, at the op `union` or
/// not, `shared` the operand face on both sides of it if one is.
///
/// - One rim holding it is a piece of that rim; an escalation refuses.
/// - A union's chord between DISTINCT faces lying within no rim is
///   where the other operand filled the gap between them and was
///   consumed: their seam ([`OneOperand::Gap`]).
/// - Pieces of one face (`shared`) lying off its rims are pieces the
///   output stage's merge left apart (a group it skipped,
///   `topo`'s `finish_output`), which a correct result holds and no rule
///   names: [`NamingError::ChordOffRim`].
/// - A subtract's or an intersect's chord between distinct faces lying
///   off their one rim is a kernel bug (row 4 below).
///
/// # Errors
///
/// The pick's own refusal, an escalation, or [`NamingError::ChordOffRim`].
fn one_operand_chord<T: Decide>(
    pick: RimPick,
    chord: EdgeKey,
    op: &OperandCtx<'_, T>,
    shared: Option<FaceKey>,
    union: bool,
) -> Result<OneOperand, NamingError> {
    match pick {
        RimPick::Holds(rim) => Ok(OneOperand::Rim(rim)),
        RimPick::Escalated(err) => Err(err),
        RimPick::Off | RimPick::NoRim(_) | RimPick::Several(_) if union && shared.is_none() => {
            Ok(OneOperand::Gap)
        }
        RimPick::Off => match shared {
            Some(face) => Err(NamingError::ChordOffRim {
                edge: chord,
                node: op.node,
                face,
            }),
            None => unreachable!(
                "chord {chord:?} between distinct faces of operand {:?} lies off the one rim each \
                 pair of them shares, at a subtract or an intersect: such a result keeps only \
                 pieces of the operands' faces, two pieces of distinct faces of one operand \
                 meet only along an edge those faces share, and a certified reading off that \
                 edge contradicts it",
                op.node
            ),
        },
        RimPick::NoRim(err) | RimPick::Several(err) => Err(err),
    }
}

/// What a chord reads as against the pairs of one operand's faces it
/// may lie between ([`rim_among`]).
#[derive(Debug)]
enum RimPick {
    /// The one rim the chord lies within.
    Holds(EdgeKey),
    /// A pair's one rim, which the chord lies off; no pair holds it.
    Off,
    /// No rim holds the chord and no pair offers one rim to lie off:
    /// [`NamingError::SharedRim`].
    NoRim(NamingError),
    /// Two distinct rims hold the chord: [`NamingError::SharedRim`]
    /// with `found: Several`.
    Several(NamingError),
    /// No rim holds the chord, and a pair's reading escalated, so it
    /// may hold it there: that pair's [`NamingError::Escalated`].
    Escalated(NamingError),
}

/// **Where chord `chord` of `body` lies among the rims of `op`'s body
/// between the faces of each of `pairs`**, decided after every pair is
/// read and by names, never by the order of the pairs or of their keys.
///
/// One shared edge is a pair's rim when the chord lies within it along
/// their carrier ([`chord_within`]); of several shared edges, the chord
/// picks the one it lies within ([`rim_holding`]). A pair with no rim,
/// or shared edges the chord picks none of, refutes the pairing, not
/// the body. A pair's rims are disjoint but for their ends, and so are
/// the rims of one body, so a chord lies within one rim at most:
///
/// 1. One rim holding it is the answer, whatever any other pair read,
///    an escalated one included: had that pair held it too, the body
///    would hold two rims overlapping along the chord. Two distinct
///    rims holding it are [`RimPick::Several`]: as [`rim_holding`]'s
///    several do, [`NamingError::SharedRim`] with `found: Several`, at
///    the least pair by name.
/// 2. Else a pair whose reading escalated may hold it: the least such
///    pair by name refuses with its escalation.
/// 3. Else, where a pair's one rim lies off the chord, [`RimPick::Off`].
/// 4. Else [`RimPick::NoRim`]: `SharedRim` at the least pair by what it
///    found (`Several` before `NotAdjacent`), then by name.
///
/// A pair's name is its two faces' names in `op`'s table, in name
/// order. Tied names compare equal, and the least
/// of equals is any of them: they spell one refusal.
///
/// # Errors
///
/// A pair's rim reading's refusal, other than
/// [`NamingError::Escalated`], which ranks among the picks.
///
/// # Panics
///
/// Where `pairs` is empty, which no caller passes.
fn rim_among<T: Decide>(
    body: &Body<T>,
    chord: EdgeKey,
    op: &OperandCtx<'_, T>,
    pairs: &[(FaceKey, FaceKey)],
    bnd: geom_core::Band,
) -> Result<RimPick, NamingError> {
    let name = |k: EntityKey| -> Option<NameRef> {
        upstream_name(op.table, op.node, ent(0, k))
            .ok()
            .map(|u| u.name)
    };
    let pair_name = |f0: FaceKey, f1: FaceKey| {
        let (n0, n1) = (name(EntityKey::Face(f0)), name(EntityKey::Face(f1)));
        if n1 < n0 { (n1, n0) } else { (n0, n1) }
    };
    let shared = |f0: FaceKey, f1: FaceKey, found: RimShare| NamingError::SharedRim {
        node: op.node,
        face: f0,
        other: f1,
        found,
    };
    let mut holds: BTreeMap<EdgeKey, (FaceKey, FaceKey)> = BTreeMap::new();
    let mut escalated = Vec::new();
    let mut off = false;
    let mut none = Vec::new();
    for &(f0, f1) in pairs {
        let read = match rim_between(op.body, f0, f1) {
            Rim::One(rim) => chord_within(body, chord, op.body, rim, bnd)
                .map(|within| if within { Ok(rim) } else { Err(Some(rim)) }),
            Rim::NotOne(RimShare::Several) => {
                rim_holding(body, chord, op.body, f0, f1, bnd).map(|held| held.ok_or(None))
            }
            Rim::NotOne(found) => {
                none.push((found != RimShare::Several, pair_name(f0, f1), f0, f1, found));
                continue;
            }
        };
        match read {
            Ok(Ok(rim)) => {
                holds.entry(rim).or_insert((f0, f1));
            }
            Ok(Err(Some(_))) => off = true,
            Ok(Err(None)) => none.push((false, pair_name(f0, f1), f0, f1, RimShare::Several)),
            Err(err @ NamingError::Escalated { .. }) => {
                escalated.push((pair_name(f0, f1), err));
            }
            Err(err) => return Err(err),
        }
    }
    if let [(rim, _)] = holds.iter().collect::<Vec<_>>()[..] {
        return Ok(RimPick::Holds(*rim));
    }
    if let Some(&(f0, f1)) = holds.values().min_by_key(|&&(f0, f1)| pair_name(f0, f1)) {
        return Ok(RimPick::Several(shared(f0, f1, RimShare::Several)));
    }
    if let Some((_, err)) = escalated.into_iter().min_by(|x, y| x.0.cmp(&y.0)) {
        return Ok(RimPick::Escalated(err));
    }
    if off {
        return Ok(RimPick::Off);
    }
    let Some((_, _, f0, f1, found)) = none
        .into_iter()
        .min_by(|x, y| (x.0, &x.1).cmp(&(y.0, &y.1)))
    else {
        unreachable!(
            "chord {chord:?}'s rim was sought among no pair of faces: each caller pairs the \
             faces of the operand on one side of it with those on the other, and each side holds \
             one or more"
        )
    };
    Ok(RimPick::NoRim(shared(f0, f1, found)))
}

/// Of the SEVERAL edges operand faces `f0` and `f1` share in
/// `op_body`, the one result edge `chord` lies within
/// ([`chord_within`]) — `None` unless exactly one does.
///
/// Two faces of one operand share several edges where what they meet
/// along is cut into pieces — a union step before this one met the
/// pair along one rim and left it in pieces, or a cut through the rim
/// did — or where they meet along several curves at all, as a
/// half-cylinder's flat and curved faces meet along two parallel
/// lines. "The rim they share" is then not one edge, and adjacency
/// cannot say which one a chord derived on them belongs to; its
/// geometry can. The shared edges are disjoint but for their shared
/// ends, so a chord strictly inside one lies within no other, and the
/// answer is one edge or none. None, or more than one, and the caller
/// refuses the pair: the chord does not pick an edge, and no other rule
/// does.
fn rim_holding<T: Decide>(
    body: &Body<T>,
    chord: EdgeKey,
    op_body: &Body<T>,
    f0: FaceKey,
    f1: FaceKey,
    bnd: geom_core::Band,
) -> Result<Option<EdgeKey>, NamingError> {
    let mut within = Vec::new();
    for rim in rims_between(op_body, f0, f1) {
        if chord_within(body, chord, op_body, rim, bnd)? {
            within.push(rim);
        }
    }
    Ok(match within.as_slice() {
        [one] => Some(*one),
        _ => None,
    })
}

/// Whether edge `e` of `body` is carried by a line: a tag read, no
/// number consulted, and the one straightness test the flush readings
/// share. The flush tests are straight-segment tests, and a curved
/// edge's chord is not its locus (a sphere's two meridians share both
/// ends and both faces).
pub(super) fn is_straight<T: Decide>(body: &Body<T>, e: EdgeKey) -> bool {
    topo::query::edge_carrier_kind(body, e) == Some(topo::query::CurveKind::Line)
}

/// Whether edge `e` of `body` holds point `p`, its ends included,
/// through `predicate`: on its closed [`Segment`] where it is straight,
/// and within its span along its certified carrier where it is curved
/// ([`Track`]), never by a curved edge's chord.
pub(super) fn edge_holds<T: Decide>(
    body: &Body<T>,
    e: EdgeKey,
    p: Point3<T>,
    predicate: &'static str,
    bnd: geom_core::Band,
) -> Result<bool, NamingError> {
    if is_straight(body, e) {
        Ok(Segment::of_edge(body, e).place(p, predicate, bnd)? != OnSegment::Off)
    } else {
        Track::of_edge(body, e)?.holds(p, predicate, bnd)
    }
}

/// **The edges a joined edge may lie along** (`names/README.md`,
/// "Flush edges"), for a pair boolean's operands and a union's members
/// alike, each a source `S` with its own body (`body_of`): the edges
/// between two faces of one source that the joined edge's two faces
/// descend from (`d0` and `d1`, a merged face listing each of its
/// constituents), straight where `line` is and curved where it is not,
/// each with its [`CarrierArc`].
pub(super) fn joined_candidates<'b, T: Decide + 'b, S: Copy + Ord>(
    d0: &[(S, FaceKey)],
    d1: &[(S, FaceKey)],
    body_of: impl Fn(S) -> Result<&'b Body<T>, NamingError>,
    line: bool,
) -> Result<BTreeMap<(S, EdgeKey), CarrierArc<T>>, NamingError> {
    let mut out = BTreeMap::new();
    for &(s, g0) in d0 {
        let body = body_of(s)?;
        for &(_, g1) in d1.iter().filter(|(s1, g1)| *s1 == s && *g1 != g0) {
            for r in rims_between(body, g0, g1) {
                if is_straight(body, r) == line
                    && let std::collections::btree_map::Entry::Vacant(slot) = out.entry((s, r))
                {
                    slot.insert(carrier_arc(body, r));
                }
            }
        }
    }
    Ok(out)
}

/// **How joined edge `e` lies along `candidates`**: along its line
/// ([`Segment::cover`]) where it is straight, along its certified
/// carrier ([`Track::cover`]) where it is curved, through
/// [`ON_MEMBER_EDGE`].
pub(super) fn joined_along<T: Decide, K>(
    body: &Body<T>,
    e: EdgeKey,
    candidates: impl IntoIterator<Item = (K, CarrierArc<T>)>,
    bnd: geom_core::Band,
) -> Result<Cover<K>, NamingError> {
    if is_straight(body, e) {
        Segment::of_edge(body, e).cover(
            candidates.into_iter().map(|(k, arc)| (k, arc.p0, arc.p1)),
            ON_MEMBER_EDGE,
            bnd,
        )
    } else {
        Track::of_edge(body, e)?.cover(candidates, ON_MEMBER_EDGE, bnd)
    }
}

/// The edges of `set`, each with its source's body, that hold point
/// `p` ([`edge_holds`] through [`ON_MEMBER_EDGE`]), in `set`'s order: a
/// vertex of a joined edge cites the edges of its set it lies on, never
/// the whole set (N2).
pub(super) fn edges_holding<'b, T: Decide + 'b, K>(
    p: Point3<T>,
    set: impl IntoIterator<Item = (K, &'b Body<T>, EdgeKey)>,
    bnd: geom_core::Band,
) -> Result<Vec<K>, NamingError> {
    let mut held = Vec::new();
    for (k, body, e) in set {
        if edge_holds(body, e, p, ON_MEMBER_EDGE, bnd)? {
            held.push(k);
        }
    }
    Ok(held)
}

/// Edge `e` of `body` as a cover candidate: its ends and, on a curved
/// edge, the point halfway along its certified carrier. A segment is
/// read by its ends alone, so its middle is its start.
#[track_caller]
pub(super) fn carrier_arc<T: Decide>(body: &Body<T>, e: EdgeKey) -> CarrierArc<T> {
    let (v0, v1) = edge_ends(body, e);
    let mid = if is_straight(body, e) {
        vertex_point(body, v0)
    } else {
        let c = certified_curve(body, e);
        let (r0, r1) = c.params();
        c.carrier().mid_point(r0, r1)
    };
    CarrierArc {
        p0: vertex_point(body, v0),
        mid,
        p1: vertex_point(body, v1),
        closed: v0 == v1,
    }
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
/// # Panics
///
/// Where a piece ends at a vertex `t` does not name. Every caller names
/// its body's vertices first: the split and boolean emitters run their
/// vertex passes before their edge pieces; `emit_shell` writes a row for
/// every vertex of its body (each minted, or carried `From` its input's
/// name, or the emission refuses) and flushes them before naming pieces;
/// and `emit_union` names pieces off the table `cite_member_edges`
/// returns, which writes back every vertex row of the collapsed fold
/// table, itself total over the body, as each fold step's boolean table
/// is (`check_total`).
#[track_caller]
pub(super) fn name_edge_pieces<T: geom_core::Real>(
    out: &mut EdgePieces,
    t: &NameTable,
    from_tie: bool,
    base: &StableName,
    (body, ix): (&Body<T>, u32),
    edges: &[EdgeKey],
    lone: Lone,
) {
    if let ([one], Lone::Whole) = (edges, lone) {
        out.push(base.clone(), from_tie, ent(ix, EntityKey::Edge(*one)));
        return;
    }
    let line = edge_line(&NameRef::new(base.clone()));
    for &e in edges {
        let (v0, v1) = edge_ends(body, e);
        let mut ends = Vec::with_capacity(2);
        for v in [v0, v1] {
            let Some(end) = t.name_of(&ent(ix, EntityKey::Vertex(v))) else {
                unreachable!(
                    "edge piece {e:?} under {base:?} ends at vertex {v:?}, which the table does \
                     not name: every caller (split, boolean, shell, union) names each vertex of \
                     its body before its edge pieces"
                )
            };
            ends.push(end.clone());
        }
        ends.sort();
        let mut name = (*line).clone();
        name.path.push(RoleSeg::Fragment(Qualifier::Ends(ends)));
        out.push(name, from_tie, ent(ix, EntityKey::Edge(e)));
    }
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

/// **How a crossing of edge `e` of `body`, named `name` in its own
/// table `table`, reads its sense** (N2): an orientation is read, never
/// chosen.
///
/// An edge on a seam line is a line of two things, the seam's sides,
/// whose loops run along it opposite ways; each side is found by name
/// among the edge's two faces (`seam_pair::a_side_is_first`), and the
/// one whose loop runs the edge as stored is `with`. A pair whose two
/// sides carry one name is read alike either way. Any other edge is
/// read as stored.
///
/// # Errors
///
/// None; a seam edge whose faces the names do not match to its sides
/// one way reads [`EdgeReading::Unsided`], which refuses a crossing
/// ([`EdgeReading::read`]).
///
/// # Panics
///
/// Where `e` is not live in `body`, or a face beside it is one `table`
/// does not name: every caller reads `e` live, and `table` is total over
/// `body`.
#[track_caller]
pub(super) fn edge_reading<T: geom_core::Real>(
    body: &Body<T>,
    table: &NameTable,
    e: EdgeKey,
    name: &StableName,
) -> EdgeReading {
    let (Some((a, b)), Some((carried_a, carried_b))) = (
        seam_pair::seam_edge_sides(name),
        seam_pair::seam_edge_sides_carried(name),
    ) else {
        return EdgeReading::Stored;
    };
    let sided = |with: &StableName, against: &StableName| EdgeReading::Sides {
        with: with.clone(),
        against: against.clone(),
    };
    if a == b {
        return sided(&carried_a, &carried_b);
    }
    let (plus, minus) = live_edge_sides(body, e, "its caller read it off `table`, which names it");
    let mut names = Vec::with_capacity(2);
    for face in [plus, minus] {
        let Some(n) = table.name_of(&ent(0, EntityKey::Face(face))) else {
            unreachable!(
                "face {face:?} beside crossed seam edge {e:?} is unnamed: `table` is total over \
                 the body it names"
            )
        };
        names.push(n);
    }
    match seam_pair::a_side_is_first(names[0], names[1], a, b) {
        Some(true) => sided(&carried_a, &carried_b),
        Some(false) => sided(&carried_b, &carried_a),
        None => EdgeReading::Unsided,
    }
}

/// How a crossing of one edge reads its sense ([`edge_reading`]).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum EdgeReading {
    /// As the body stores the edge.
    Stored,
    /// By a seam's sides: `with`'s loop runs the edge as stored, and
    /// `against`'s the other way. Each is the side's name in the edge
    /// name's own space (`seam_pair::seam_edge_sides_carried`).
    Sides {
        with: StableName,
        against: StableName,
    },
    /// A seam edge whose two faces the names match to neither, or to
    /// both, assignments of its sides.
    Unsided,
}

impl EdgeReading {
    /// The sense of a crossing whose sense along the edge as stored is
    /// `stored`: `stored` itself on an edge read as stored, and on a
    /// seam's a touch as it is and a transversal crossing as the side
    /// whose direction runs in, cited through `cite` in the vertex's
    /// own space.
    ///
    /// # Errors
    ///
    /// [`NamingError::Emission`] for a transversal crossing of an
    /// [`EdgeReading::Unsided`] seam edge: which side it enters along is
    /// no fact of the names.
    pub(super) fn read(
        &self,
        stored: Sense,
        cite: impl Fn(&StableName) -> NameRef,
    ) -> Result<Sense, NamingError> {
        Ok(match (self, stored) {
            (Self::Stored, stored) => stored,
            (_, touch @ (Sense::Outside | Sense::Inside)) => touch,
            (Self::Sides { with, .. }, Sense::Enters) => Sense::Through {
                entering: vec![cite(with)],
            },
            (Self::Sides { against, .. }, Sense::Leaves) => Sense::Through {
                entering: vec![cite(against)],
            },
            (Self::Sides { .. }, through @ Sense::Through { .. }) => through,
            (Self::Unsided, _) => return Err(NamingError::Emission { what: UNSIDED }),
        })
    }
}

/// A transversal crossing of a seam edge whose two faces the names do
/// not match to its sides one way: which side it enters along is no
/// fact of the names, so neither is its sense.
pub(super) const UNSIDED: &str = "a crossed seam edge's two faces match its sides neither way, or \
                                  both ways, so no side is the one the crossing enters along";

/// The edges at vertex `v` of `body` ([`Body::edges_of_vertex`]).
///
/// # Panics
///
/// Where `v` does not read (that door refuses only a stale key or a
/// broken orbit): `held` says why the caller holds it live, in a body
/// every door keeps tier-1-valid.
#[track_caller]
fn live_vertex_edges<T: geom_core::Real>(
    body: &Body<T>,
    v: VertexKey,
    held: &'static str,
) -> Vec<EdgeKey> {
    match body.edges_of_vertex(v) {
        Some(edges) => edges,
        None => unreachable!("vertex {v:?}'s edges do not read, yet {held}"),
    }
}

/// The faces at vertex `v` of `body` ([`Body::faces_of_vertex`]).
///
/// # Panics
///
/// Where `v` does not read (that door refuses only a stale key or a
/// broken orbit): `held` says why the caller holds it live, in a body
/// every door keeps tier-1-valid.
#[track_caller]
fn live_vertex_faces<T: geom_core::Real>(
    body: &Body<T>,
    v: VertexKey,
    held: &'static str,
) -> Vec<FaceKey> {
    match body.faces_of_vertex(v) {
        Some(faces) => faces,
        None => unreachable!("vertex {v:?}'s faces do not read, yet {held}"),
    }
}

/// The two faces edge `e` of `body` bounds, its `he_plus` side first.
///
/// # Panics
///
/// Where `e` is not live in `body` (`topo::readback::edge_sides`
/// refuses only a stale key): `held` says why the caller holds it live.
#[track_caller]
fn live_edge_sides<T: geom_core::Real>(
    body: &Body<T>,
    e: EdgeKey,
    held: &'static str,
) -> (FaceKey, FaceKey) {
    match topo::readback::edge_sides(body, e) {
        Ok(sides) => sides.faces(),
        Err(err) => unreachable!("edge {e:?} is not live in its body ({err:?}), yet {held}"),
    }
}

/// The boundary edges of operand faces `faces`: the parent edges a
/// split piece's `Keeps` cites from (N2).
fn operand_boundary<T: Decide>(
    target_body: &Body<T>,
    faces: impl IntoIterator<Item = FaceKey>,
) -> BTreeSet<EdgeKey> {
    let mut boundary = BTreeSet::new();
    for f in faces {
        boundary.extend(face_edges(target_body, f));
    }
    boundary
}

/// The edges of face `f`'s loops in `body`, one per half-edge
/// ([`face_half_edges`]).
///
/// # Panics
///
/// Where [`face_half_edges`] panics, or a half-edge that walk returned does not resolve: it read each
/// one off `f`'s loops a moment before, in a body every door keeps
/// tier-1-valid.
#[track_caller]
fn face_edges<T: geom_core::Real>(body: &Body<T>, f: FaceKey) -> Vec<EdgeKey> {
    let hes = face_half_edges(body, f);
    let mut out = Vec::with_capacity(hes.len());
    for he in hes {
        let Some(h) = body.get_half_edge(he) else {
            unreachable!(
                "half-edge {he:?} of face {f:?}'s loops does not resolve: the face walk read it \
                 off those loops, and a tier-1-valid body holds every half-edge they cycle through"
            )
        };
        out.push(h.edge);
    }
    out
}

/// N2's `Keeps`, one reading for a split's face pieces and corner
/// copies: the parent's edges an entity holds a stretch of, by descent
/// alone. An edge of the half holds a stretch of the operand edge it
/// descends from, through the split's `SplitEdge` records and, for an
/// edge a join made, through each edge its cover lies along
/// (`SplitNaming::joined_lineage`).
struct KeepsReading<'r, 's, T: Decide> {
    sides: &'r [Side<'s, T>],
    target_node: RecipeNodeId,
    target_table: &'r NameTable,
    lineage: &'r BTreeMap<EdgeKey, EdgeKey>,
    covers: &'r BTreeMap<EdgeKey, Vec<EdgeKey>>,
}

impl<T: Decide> KeepsReading<'_, '_, T> {
    /// `Keeps` over the lines of the `parents` the half's `edges`
    /// descend from, in name order.
    fn of(
        &self,
        edges: impl IntoIterator<Item = EdgeKey>,
        parents: &BTreeSet<EdgeKey>,
    ) -> Result<Qualifier, NamingError> {
        let mut lines = BTreeSet::new();
        for e in edges {
            for c in self.covers.get(&e).cloned().unwrap_or_else(|| vec![e]) {
                let root = chase_split_lineage(self.sides, self.target_table, self.lineage, c);
                if parents.contains(&root) {
                    let up = upstream_name(
                        self.target_table,
                        self.target_node,
                        ent(0, EntityKey::Edge(root)),
                    )?;
                    lines.insert((*edge_line(&up.name)).clone());
                }
            }
        }
        Ok(Qualifier::Keeps(lines.into_iter().collect()))
    }
}

/// Split faces: pass-through for uncut operand faces; `SplitFragment`
/// (side-discriminated, same-side multiplicity by `Keeps`) for cut
/// ones.
#[allow(clippy::too_many_arguments)]
fn name_split_faces<T: Decide>(
    node: RecipeNodeId,
    halves: &mut [HalfNames; 2],
    sides: &[Side<'_, T>],
    frag_rows: &topo::FaceDivisions,
    section_keys: &BTreeSet<FaceKey>,
    merged: &SplitMerges,
    keeps: &KeepsReading<'_, '_, T>,
    target_body: &Body<T>,
) -> Result<(), NamingError> {
    let (target_node, target_table) = (keeps.target_node, keeps.target_table);
    // Every root that was ever divided: fragments cover it.
    let divided: BTreeSet<FaceKey> = frag_rows
        .rows()
        .iter()
        .map(|&(_, p)| frag_rows.root(p))
        .collect();
    // (root, side) → the side's body and the members.
    type Group<'b, T> = (&'b Body<T>, Vec<FaceKey>);
    let mut groups: BTreeMap<(FaceKey, usize), Group<'_, T>> = BTreeMap::new();
    for s in sides {
        for (f, _) in s.body.faces() {
            if section_keys.contains(&f) || merged.held.contains_key(&f) {
                continue;
            }
            let root = frag_rows.root(f);
            if root == f && !divided.contains(&root) {
                // Uncut operand face: pass-through (N1: the split
                // contributes no segment to survivors). It is still
                // its (face, side) group's one member, spelled by its
                // upstream name rather than the group's base.
                let up = upstream_name(target_table, target_node, ent(0, EntityKey::Face(f)))?;
                let h = &mut halves[s.slot()];
                h.rec.record(
                    &split_base(node, up.name.clone()),
                    vec![ent(0, EntityKey::Face(f))],
                    Parent::Elsewhere,
                );
                h.tie.carry(up, ent(0, EntityKey::Face(f)))?;
            } else {
                groups
                    .entry((root, s.slot()))
                    .or_insert_with(|| (s.body, Vec::new()))
                    .1
                    .push(f);
            }
        }
    }
    for ((root, slot), (body, members)) in groups {
        let parent = upstream_name(target_table, target_node, ent(0, EntityKey::Face(root)))?;
        let from_tie = parent.tied;
        let base_name = split_base(node, parent.name);

        let h = &mut halves[slot];
        h.rec.record(
            &base_name,
            members
                .iter()
                .map(|&f| ent(0, EntityKey::Face(f)))
                .collect(),
            Parent::Elsewhere,
        );
        if let [f] = members.as_slice() {
            put(
                &mut h.t,
                &mut h.tie,
                from_tie,
                base_name,
                ent(0, EntityKey::Face(*f)),
            )?;
            continue;
        }
        // Same-side multiplicity: each piece by the parent's boundary
        // edges it holds a stretch of (N2's `Keeps`).
        let boundary = operand_boundary(target_body, [root]);
        let mut pieces = Vec::with_capacity(members.len());
        for &f in &members {
            let edges = face_edges(body, f);

            pieces.push((keeps.of(edges, &boundary)?, ent(0, EntityKey::Face(f))));
        }
        mint_qualified(&mut h.t, &mut h.tie, from_tie, &base_name, pieces)?;
    }
    Ok(())
}

/// The base of a split's (face, side) group: the face `parent` in the
/// half whose table holds it. One spelling for the group the split
/// divides and for the face it passes through whole.
fn split_base(node: RecipeNodeId, parent: NameRef) -> StableName {
    name1(EntityKind::Face, node, RoleSeg::SplitFragment { parent })
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
    use topo::construction::Construct;

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
        assert!(one_partner::<()>(v, Vec::new()).unwrap().is_none());
        let b = OpSide::B(());
        let (_, got) = one_partner(v, vec![(b, upstream(3, false)), (b, upstream(3, true))])
            .unwrap()
            .expect("two rows naming one partner decide it");
        assert_eq!(got.name, upstream(3, false).name);
        assert!(got.tied, "tie-descended if any row's partner is");
    }

    #[test]
    fn one_partner_refuses_two_distinct_partners() {
        let v = VertexKey::default();
        let Err(NamingError::SeamVertexPartners { vertex, candidates }) = one_partner(
            v,
            vec![
                (OpSide::B(()), upstream(3, false)),
                (OpSide::B(()), upstream(4, false)),
            ],
        ) else {
            panic!("two distinct partners must refuse as SeamVertexPartners")
        };
        assert_eq!(vertex, v);
        assert_eq!(candidates.len(), 2);
    }

    /// A body holding one straight edge from `p` to `q`, and that edge.
    fn segment(p: [f64; 3], q: [f64; 3]) -> (Body<f64>, EdgeKey) {
        let mut body = Body::<f64>::new();
        let born = body
            .mvfs(Point3::from_array(p), true)
            .expect("mvfs births a lone vertex");
        let edge = body
            .mev_line(
                topo::construction::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                Point3::from_array(q),
                Tol::witness(),
            )
            .expect("mev on an empty loop grows it by one edge")
            .edge;
        (body, edge)
    }

    /// **A crossing's sense reads the crossed edge's direction, not the
    /// piece's.** One edge from `p` to `q`: as a piece running the
    /// crossed edge's way it enters at `p` and leaves at `q`; as a
    /// joined edge running against it, the senses swap.
    #[test]
    fn a_split_sense_follows_the_crossed_edge_through_a_reversed_piece() {
        let (body, e) = segment([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let (p, q) = super::edge_ends(&body, e);
        let sense = |v, same| super::split_sense(&body, v, &[(e, same)]).unwrap();
        assert_eq!(
            [sense(p, true), sense(q, true)],
            [Sense::Enters, Sense::Leaves],
            "a piece the edge's way"
        );
        assert_eq!(
            [sense(p, false), sense(q, false)],
            [Sense::Leaves, Sense::Enters],
            "a piece against it"
        );
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
        let plane = profile::SketchPlane::from_frame(geom_core::RigidMotion::axes_xy(
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
                side: sweep::ExtrudeSide::Along,
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
            read: crate::names::FOLD_A,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
            read: crate::names::FOLD_B,
            table: &empty,
            body: &built.body,
        };
        let t = name_boolean(
            bool_node,
            topo::BooleanOp::Union,
            &result,
            &naming,
            &a,
            &b,
            Tol::witness(),
        )
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

    /// **A crossing at a vertex the boolean read no partner of refuses
    /// why** (`topo::UnreadPartner`): its edges hold no class, so their
    /// senses would drop out of the name unseen. A crossing whose keys
    /// the list does not hold reads its rows as before (here, none).
    /// Constructed, since a crossing is not reached at a listed vertex
    /// in a model (`a_mate_on_a_full_turn_bore_names` lists partners and
    /// names cleanly): a listed key is an operand's own vertex, and
    /// `name_boolean_vertices` names a result vertex by that identity
    /// before it reads senses, so `senses_at` meets a listed key only
    /// where the operand's table names no vertex at it.
    #[test]
    fn a_crossing_at_an_unread_partner_refuses_why() {
        let built = unit_cube();
        let mut corners = built.body.vertices().map(|(v, _)| v);
        let (listed, partner, other) = (
            corners.next().unwrap(),
            corners.next().unwrap(),
            corners.next().unwrap(),
        );
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Direct,
            unread_partners: vec![topo::UnreadPartner {
                operand: topo::Operand::A,
                vertex: listed,
                partner,
                neighbourhood: topo::UnreadNeighbourhood::OneFace,
            }],
            ..topo::BooleanNaming::default()
        };
        let table = NameTable::new();
        let ctx = |read| OperandCtx {
            node: RecipeNodeId::new(0, 1),
            read,
            table: &table,
            body: &built.body,
        };
        let (a, b) = (ctx(crate::names::FOLD_A), ctx(crate::names::FOLD_B));
        let keys = BTreeSet::from([(topo::Operand::B, partner), (topo::Operand::A, listed)]);
        assert!(matches!(
            senses_at(RecipeNodeId::new(0, 9), &naming, other, &keys, &a, &b),
            Err(NamingError::PartnerUnread { vertex: at, partner: met, unread })
                if at == other && met == partner && unread == naming.unread_partners[0]
        ));
        // The same vertex key read from the other operand is not listed.
        let keys = BTreeSet::from([(topo::Operand::B, listed)]);
        assert!(
            senses_at(RecipeNodeId::new(0, 9), &naming, other, &keys, &a, &b)
                .unwrap()
                .is_empty()
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
            read: crate::names::FOLD_A,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
            read: crate::names::FOLD_B,
            table: &empty,
            body: &built.body,
        };
        let err = name_boolean(
            RecipeNodeId::new(0, 9),
            topo::BooleanOp::Union,
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

    /// **A lineage a kill broke reads on through the discard rows.**
    /// `split_edge` twice off one lateral gives `e2 → e1 → e0`; killing
    /// the middle piece `e1` drops its birth record, so the walk from
    /// `e2` stops at the dead `e1`. A discard row whose chain is
    /// `[e1, e0]` — what the boolean records for an edge that died with
    /// a discarded face — carries the walk on to the named `e0`.
    #[test]
    fn a_lineage_through_a_killed_piece_reaches_the_table() {
        let built = unit_cube();
        let table = name_extrude(
            RecipeNodeId::new(0, 1),
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
        // Collapse e1 onto its start: e2 then runs from there to the
        // lateral's far end, along the same line.
        let point = |b: &Body<f64>, he| {
            let v = b.get_half_edge(he).unwrap().start;
            *b.get_point(b.get_vertex(v).unwrap().point).unwrap()
        };
        let d1 = body.get_edge(e1).unwrap();
        let (m, he) = (point(&body, d1.he_plus), d1.he_plus);
        let d2 = body.get_edge(e2).unwrap();
        let q = point(&body, d2.he_minus);
        body.kev_describing(
            he,
            &[(e2, topo::EdgeCurveSpec::line_between(m, q))],
            Tol::witness(),
        )
        .expect("the middle piece collapses");
        assert!(
            body.edge_provenance_of(e1).is_none() && body.get_edge(e2).is_some(),
            "the kill takes e1's record and leaves e2"
        );
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            discards: vec![topo::DiscardRow {
                operand: topo::Operand::A,
                face: body.faces().next().map(|(f, _)| f).unwrap(),
                bordered: Vec::new(),
                bordered_through: Vec::new(),
                boundary_chains: vec![vec![e1, e0]],
            }],
            ..topo::BooleanNaming::default()
        };
        let killed = KilledSplits::of(&naming);
        let none = std::collections::BTreeMap::new();
        assert_eq!(
            chase_edge_to_table(&body, &table, &none, e2).unwrap(),
            e1,
            "without the discard rows the walk stops at the killed piece"
        );
        assert_eq!(
            chase_edge_to_table(&body, &table, &killed.arena, e2).unwrap(),
            e0,
            "the discard rows carry the walk to the named lateral"
        );
        assert!(
            killed.graft_b.is_empty(),
            "an A row in an A-clone result is in arena keys"
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
            read: crate::names::FOLD_A,
            table: &b_table,
            body: &body,
        };
        let b = OperandCtx {
            node: ext_node,
            read: crate::names::FOLD_B,
            table: &b_table,
            body: &body,
        };
        let err = name_boolean(
            RecipeNodeId::new(0, 9),
            topo::BooleanOp::Union,
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
        // {From(bottom), From(laterals[0])}.
        let naming = topo::BooleanNaming {
            a_keys: topo::OperandKeys::Direct,
            b_keys: topo::OperandKeys::Absent,
            face_fragments_a: topo::FaceDivisions::from_rows([
                (built.top, built.bottom),
                (laterals[1], laterals[0]),
            ])
            .unwrap(),
            merge_groups: vec![
                (built.top, vec![laterals[0]]),
                (built.bottom, vec![laterals[1]]),
            ],
            ..topo::BooleanNaming::default()
        };
        let empty = NameTable::new();
        let a = OperandCtx {
            node: ext_node,
            read: crate::names::FOLD_A,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
            read: crate::names::FOLD_B,
            table: &empty,
            body: &built.body,
        };
        let err = name_boolean(
            RecipeNodeId::new(0, 9),
            topo::BooleanOp::Union,
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
    fn two_merges_of_tied_faces_publish_one_tied_row() {
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
            read: crate::names::FOLD_A,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
            read: crate::names::FOLD_B,
            table: &empty,
            body: &built.body,
        };
        let out = name_boolean(
            RecipeNodeId::new(0, 9),
            topo::BooleanOp::Union,
            &built.body,
            &naming,
            &a,
            &b,
            Tol::witness(),
        )
        .expect("two tied single-face merges name as one tied row");
        // A single-face merge is a `Merged` of one, which is that face
        // (N3), so the row is the tied face's own name.
        let merged: Vec<_> = out
            .table
            .iter()
            .filter(|(_, e)| matches!(e, Entry::Tied(_)))
            .collect();
        assert_eq!(merged.len(), 1, "one tied row: {merged:?}");
        let (name, entry) = merged[0];
        assert!(
            matches!(name.path.as_slice(), [RoleSeg::From { .. }]),
            "the row is the face carried in, with no qualifier: {name:?}"
        );
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
            read: crate::names::FOLD_A,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
            read: crate::names::FOLD_B,
            table: &empty,
            body: &built.body,
        };
        let t = name_boolean(
            bool_node,
            topo::BooleanOp::Union,
            &result,
            &naming,
            &a,
            &b,
            Tol::witness(),
        )
        .unwrap()
        .table;
        let wrap = |inner: &StableName| {
            name1(
                EntityKind::Face,
                bool_node,
                RoleSeg::From {
                    read: crate::names::FOLD_A,
                    of: inner.clone().into(),
                },
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
                    Some(RoleSeg::From {
                        read: crate::names::FOLD_A,
                        of: inner,
                    }) => {
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
    /// merged face — a table no emitter of this crate produces — panics
    /// at the merge-group loop, not published nested. The pair boolean
    /// has no `collapse` after it, so this is the one door that holds N3
    /// for it.
    #[test]
    #[should_panic(
        expected = "a boolean table carries a merged face whose constituent is itself a merged face"
    )]
    fn a_merge_over_a_nested_merged_face_panics_at_the_mint() {
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
            read: crate::names::FOLD_A,
            table: &a_table,
            body: &built.body,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
            read: crate::names::FOLD_B,
            table: &empty,
            body: &built.body,
        };
        let _ = name_boolean(
            RecipeNodeId::new(0, 9),
            topo::BooleanOp::Union,
            &built.body,
            &naming,
            &a,
            &b,
            Tol::witness(),
        );
    }

    /// The unit cube's table, as an operand reads it, with its top, its
    /// bottom and its laterals at y = 0 and y = 1.
    struct CubeOperand {
        built: sweep::Extruded<f64>,
        table: std::sync::Arc<NameTable>,
        front: FaceKey,
        back: FaceKey,
    }

    impl CubeOperand {
        fn new() -> Self {
            let built = unit_cube();
            let table = name_extrude(
                RecipeNodeId::new(0, 1),
                &built,
                &crate::eval::ProfilePieces::numbered(
                    &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
                ),
            )
            .unwrap();
            let at_y = |y: f64| {
                *built.side_faces()[0]
                    .iter()
                    .find(|&&l| {
                        let Rim::One(rim) = rim_between(&built.body, built.top, l) else {
                            panic!("the top meets each lateral along one rim")
                        };
                        let (v0, v1) = edge_ends(&built.body, rim);
                        [v0, v1]
                            .into_iter()
                            .all(|v| (vertex_point(&built.body, v).y - y).abs() < 1e-12)
                    })
                    .unwrap()
            };
            let (front, back) = (at_y(0.0), at_y(1.0));
            Self {
                built,
                table,
                front,
                back,
            }
        }

        fn ctx(&self) -> OperandCtx<'_, f64> {
            OperandCtx {
                node: RecipeNodeId::new(0, 1),
                read: crate::names::FOLD_A,
                table: &self.table,
                body: &self.built.body,
            }
        }

        fn rim(&self, f: FaceKey, g: FaceKey) -> EdgeKey {
            let Rim::One(rim) = rim_between(&self.built.body, f, g) else {
                panic!("{f:?} and {g:?} meet along one rim")
            };
            rim
        }
    }

    /// Every order of `xs`.
    fn orders(xs: &[FaceKey]) -> Vec<Vec<FaceKey>> {
        if xs.len() <= 1 {
            return vec![xs.to_vec()];
        }
        let mut out = Vec::new();
        for i in 0..xs.len() {
            let mut rest = xs.to_vec();
            let first = rest.remove(i);
            for mut o in orders(&rest) {
                o.insert(0, first);
                out.push(o);
            }
        }
        out
    }

    /// `merged_chord_rim` over the two merged faces' faces `c0s` and
    /// `c1s` of the cube read as operand A, and `d0s` and `d1s` of it
    /// read as operand B, in every order of each and with the two faces
    /// swapped.
    fn every_order(
        cube: &CubeOperand,
        chord: [[f64; 3]; 2],
        (c0s, c1s): (&[FaceKey], &[FaceKey]),
        (d0s, d1s): (&[FaceKey], &[FaceKey]),
    ) -> Vec<Result<Option<(topo::Operand, EdgeKey)>, NamingError>> {
        let (chord_body, e) = segment(chord[0], chord[1]);
        let bnd = band(Tol::witness()).unwrap();
        let ctx = cube.ctx();
        let mut out = Vec::new();
        for o0 in orders(c0s) {
            for o1 in orders(c1s) {
                for (x, y) in [(&o0, &o1), (&o1, &o0)] {
                    let (d0, d1) = if x == &o0 { (d0s, d1s) } else { (d1s, d0s) };
                    let (d0, d1) = (d0.to_vec(), d1.to_vec());
                    let sides = [
                        (topo::Operand::A, &ctx, x.as_slice(), y.as_slice()),
                        (topo::Operand::B, &ctx, d0.as_slice(), d1.as_slice()),
                    ];
                    out.push(merged_chord_rim(&chord_body, e, sides, bnd));
                }
            }
        }
        out
    }

    /// **A chord between two merged faces is the one rim between a face
    /// of each that it lies within**, whatever the order of their faces
    /// or of the two merged faces. One merged face holds the cube's top
    /// and bottom, the other its laterals at y = 0 and y = 1; the chord
    /// lies within the top's rim with the front, and the three other
    /// pairs each share a rim it lies off. Neither merged face holds a
    /// face of the other operand.
    #[test]
    fn a_chord_between_two_merged_faces_is_the_rim_it_lies_within() {
        let cube = CubeOperand::new();
        let (top, bottom) = (cube.built.top, cube.built.bottom);
        let want = cube.rim(top, cube.front);
        for got in every_order(
            &cube,
            [[0.2, 0.0, 1.0], [0.6, 0.0, 1.0]],
            (&[top, bottom], &[cube.front, cube.back]),
            (&[], &[]),
        ) {
            assert_eq!(
                got.unwrap(),
                Some((topo::Operand::A, want)),
                "the top's rim with the front holds the chord"
            );
        }
    }

    /// **A chord between two merged faces within none of their rims is
    /// no rim's**, whatever the order: its emitter names it the seam of
    /// the two parents. The chord crosses the top's interior, off the
    /// four rims the top and bottom share with the front and back
    /// laterals; and the top against the bottom share no rim at all.
    #[test]
    fn a_chord_within_no_rim_of_two_merged_faces_is_no_rims_in_every_order() {
        let cube = CubeOperand::new();
        let (top, bottom) = (cube.built.top, cube.built.bottom);
        let interior = [[0.2, 0.5, 1.0], [0.6, 0.5, 1.0]];
        for (faces, what) in [
            (
                (&[top, bottom][..], &[cube.front, cube.back][..]),
                "off every rim",
            ),
            ((&[top][..], &[bottom][..]), "no shared rim"),
        ] {
            for got in every_order(&cube, interior, faces, (&[], &[])) {
                assert_eq!(got.unwrap(), None, "{what}");
            }
        }
    }

    /// **A chord within a rim of each operand is a kernel bug**: two
    /// operand edges flush along it are carried or joined before any
    /// chord is read. The cube read as both operands, the merged faces
    /// holding its top and front on each side.
    #[test]
    #[should_panic(expected = "lies within a rim of each operand")]
    fn a_chord_within_a_rim_of_each_operand_is_a_kernel_bug() {
        let cube = CubeOperand::new();
        let top = cube.built.top;
        let _ = every_order(
            &cube,
            [[0.2, 0.0, 1.0], [0.6, 0.0, 1.0]],
            (&[top], &[cube.front]),
            (&[top], &[cube.front]),
        );
    }

    /// **The rim may be either operand's**: the same chord, the merged
    /// faces' faces read as operand B and holding none of A's.
    #[test]
    fn a_chord_between_two_merged_faces_may_lie_within_the_b_operands_rim() {
        let cube = CubeOperand::new();
        let top = cube.built.top;
        let want = cube.rim(top, cube.front);
        for got in every_order(
            &cube,
            [[0.2, 0.0, 1.0], [0.6, 0.0, 1.0]],
            (&[], &[]),
            (&[top], &[cube.front, cube.back]),
        ) {
            assert_eq!(got.unwrap(), Some((topo::Operand::B, want)));
        }
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

    use crate::ProfileDoc;
    use crate::edit::DocEdit;
    use crate::eval::{CancelToken, EvalOptions, Evaluation, evaluate};
    use crate::ident::DocumentId;
    use crate::names::table::{EntityRef, Entry, NameTable};
    use crate::node::{Node, RecipeNodeId};
    use crate::program::{LoopProgram, ProfileProgram};
    use crate::test_support::{against, axes_at, len, plane_on};
    use geom_core::Tol;

    fn ins(doc: ProfileDoc, node: crate::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("inserts");
        (a.doc, a.record.minted.expect("a node"))
    }

    fn prism(doc: ProfileDoc, dz: f64, pts: &[(f64, f64)]) -> (ProfileDoc, RecipeNodeId) {
        let (doc, profile) = ins(
            doc,
            Node::Profile(ProfileProgram {
                loops: vec![LoopProgram::polygon(pts.iter().copied()).expect("finite")],
                ids: Vec::new(),
                on: None,
            }),
        );
        ins(
            doc,
            Node::Extrude {
                profile: profile.into(),
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
        let (doc, a) = prism(doc, 4.0, &[(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)]);
        let (doc, b) = prism(
            doc,
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
        // The cutter stands one unit up the block's own z.
        let (doc, cutter) = against(
            doc,
            a,
            b,
            &axes_at([0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            Tol::witness(),
        );
        let (doc, sub) = ins(
            doc,
            Node::Subtract {
                from: a.into(),
                tool: crate::BodyRead::plain(crate::Operand::output(cutter, 0)),
            },
        );
        let tool = plane_on(&doc, a, [0.0, y, 0.0], [0.0, ny, 0.0], Tol::witness());
        let (doc, split) = ins(
            doc,
            Node::Split {
                target: sub.into(),
                tool,
            },
        );
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
        ev.table_at(id)
            .unwrap_or_else(|| panic!("node {id:?} evaluates: {:?}", ev.nodes.get(&id)))
    }

    /// Every tied row of the subtract each half's table carries, as
    /// (entry shape in that half, whether each candidate kept its
    /// number). Keys are kept through the split, so an entity of a half
    /// names the same key in the subtract's one body.
    fn carried_ties(y: f64, ny: f64) -> Vec<(bool, Vec<bool>)> {
        let (ev, sub, split) = split_u_cutter(y, ny);
        let sub = table(&ev, sub);
        let mut out = Vec::new();
        for port in 0..2 {
            let var = ev.output_var(split, port).expect("a read per half");
            let half = ev.table(var).expect("each half evaluates");
            for (name, entry) in sub.iter_refs() {
                if !matches!(entry, Entry::Tied(_)) {
                    continue;
                }
                let Some(row) = half.rows().find(|(n, _)| *n == name).map(|(_, r)| r) else {
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
                let tied = matches!(half.entry_of(name), Some(Entry::Tied(_)));
                out.push((tied, kept));
            }
        }
        out
    }

    #[test]
    fn a_split_carries_each_tie_candidates_number_tied_or_narrowed() {
        // Planes between the prongs separate the candidates, one per
        // half, and each half's table narrows the tie to the candidate
        // it holds, under its number; a plane clear of both keeps both
        // in one half, `Tied`.
        for (what, y, ny, together) in [
            ("between the prongs, +y", 2.0, 1.0, false),
            ("between the prongs, -y", 2.0, -1.0, false),
            ("clear of both prongs, +y", 3.5, 1.0, true),
            ("clear of both prongs, -y", 3.5, -1.0, true),
        ] {
            let rows = carried_ties(y, ny);
            if together {
                assert!(
                    rows.iter().any(|(tied, kept)| *tied && kept.len() == 2),
                    "{what}: the premise, a subtract tie one half keeps Tied: {rows:?}"
                );
            } else {
                assert!(
                    rows.iter().filter(|(_, kept)| kept.len() == 1).count() >= 2,
                    "{what}: the premise, a subtract tie each half narrows: {rows:?}"
                );
            }
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
    use crate::eval::{CancelToken, EvalOptions, PoseValue, ValuePayload, evaluate};
    use crate::names::role::SplitHalf;
    use crate::names::table::{EntityKey, EntityRef};
    use crate::test_support::clipped_cylinder;
    use geom_core::Tol;
    use topo::{Provenance, SplitPlane};

    #[test]
    fn a_twice_crossed_rim_arcs_pieces_chase_to_the_rim_across_halves() {
        let (doc, [ext, split]) = clipped_cylinder(Tol::witness());
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
        // The plane the split verb reads, as the split binds it.
        let Some(crate::Node::Split { tool, .. }) = doc.node(split) else {
            panic!("the split is a split");
        };
        let Ok(PoseValue::Plane { normal, offset }) =
            crate::eval::bound_pose(&doc, &ev, crate::OperandSlot::Tool, *tool, Tol::witness())
        else {
            panic!("the tool is a plane");
        };
        let table = ev.table_at(ext).expect("the extrude's table");
        let out = topo::split(
            &topo::test_support::finished("the extrude", body.body().clone(), Tol::witness()),
            &SplitPlane {
                origin: PoseValue::plane_foot(normal, offset),
                normal,
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
                let root = chase_split_edge_to_table(&sides, table, e);
                assert!(
                    named(root),
                    "{:?} edge {e:?} chases to {root:?}, which the extrude never named",
                    s.half
                );
                let within =
                    chase_edge_to_table(s.body, table, &std::collections::BTreeMap::new(), e)
                        .expect("acyclic");
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
mod wavy_loft {
    //! A loft through four squares whose x offsets alternate: its corner
    //! edges are NURBS curves, which a plane across them meets three
    //! times, the first and third crossings with one sense.
    //!
    //! No body the kernel builds puts a face across such an edge yet: the
    //! boolean's sweep splits the corners where a face of the other
    //! operand crosses them, the sector walk reads the spline walls
    //! around each crossing vertex, and the union refuses one step on,
    //! where a spline wall's trim cannot place a pierce point
    //! ([`a_union_across_a_nurbs_edge_refuses_at_a_spline_walls_held_pierce`],
    //! which reds the day that site reads one).
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use crate::edit::DocEdit;
    use crate::eval::{CancelToken, EvalOptions, ValuePayload, evaluate};
    use crate::ident::DocumentId;
    use crate::names::table::NameTable;
    use crate::node::{Node, RecipeNodeId};
    use crate::program::{LoopProgram, ProfileProgram};
    use crate::{Formula, ProfileDoc};
    use geom_core::Tol;
    use topo::{Body, EdgeKey};

    fn ins(doc: ProfileDoc, node: crate::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("inserts");
        (a.doc, a.record.minted.expect("a node"))
    }

    /// The loft through 2 × 2 squares at z = 0, 1, 2, 3 whose x offsets
    /// alternate `−a, a, −a, a`, skinned at v-degree 2: its value.
    pub(super) fn wavy(a: f64) -> (RecipeNodeId, crate::eval::NodeValue<f64>, NameTable) {
        let (doc, loft) = wavy_doc(a);
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let value = ev.value(loft).expect("the loft evaluates").clone();
        let table = ev.table_at(loft).expect("the loft's table").clone();
        (loft, value, table)
    }

    /// [`wavy`]'s document and its loft node.
    fn wavy_doc(a: f64) -> (ProfileDoc, RecipeNodeId) {
        let mut doc = ProfileDoc::empty(DocumentId::derive("nurbs-crossings"), Tol::witness());
        let mut profiles = Vec::new();
        for dx in [-a, a, -a, a] {
            let square = [
                (dx - 1.0, -1.0),
                (dx + 1.0, -1.0),
                (dx + 1.0, 1.0),
                (dx - 1.0, 1.0),
            ];
            let (d, profile) = ins(
                doc,
                Node::Profile(ProfileProgram {
                    loops: vec![LoopProgram::polygon(square).expect("finite")],
                    ids: Vec::new(),
                    on: None,
                }),
            );
            doc = d;
            profiles.push(profile);
        }
        // Each later section stands `k` up the first's own z.
        let mut sections = profiles.into_iter();
        let first = sections.next().expect("four sections");
        let then = (1u32..)
            .zip(sections)
            .map(|(k, profile)| crate::node::LoftSection {
                profile: profile.into(),
                station: vec![crate::node::StationMove::Slide {
                    along: crate::node::Axis3::Z,
                    by: crate::test_support::len(f64::from(k)),
                }],
            })
            .collect();
        ins(
            doc,
            Node::Loft {
                first: first.into(),
                then,
                v_degree: Formula::count(2),
            },
        )
    }

    fn body_of(value: &crate::eval::NodeValue<f64>) -> &Body<f64> {
        let ValuePayload::Body(body) = &value.payload else {
            panic!("a body");
        };
        body.body()
    }

    fn carrier(body: &Body<f64>, e: EdgeKey) -> &geom::Curve3<f64> {
        super::certified_curve(body, e).carrier()
    }

    /// The parameters at which `e`'s carrier crosses the plane `x = c`
    /// its two ends straddle evenly, with the sign of `x − c` after each:
    /// test-side root finding.
    fn crossings_of(body: &Body<f64>, e: EdgeKey) -> Vec<(f64, bool)> {
        let carrier = carrier(body, e);
        let (t0, t1) = super::certified_curve(body, e).params();
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

    /// **A tripwire, not the end-to-end row**: the wavy loft (`a = 0.3`)
    /// united through the recipe with a box over `x ∈ [1, 3]`,
    /// `z ∈ [−½, 3½]`, whose face `x = 1` crosses each of the loft's two
    /// corner edges at `x = dx + 1` three times, the first and third with
    /// one sense (asserted on the carriers). The boolean's sweep reads
    /// each corner against the face through the curve × face table's
    /// ring-composite arm and splits it at all three (`sweep`'s
    /// `a_face_across_a_wavy_corner` pins the splits), the sector walk
    /// reads the spline walls each crossing vertex lies between
    /// (`topo::sector_face`'s spline arm), and the union refuses one step
    /// on: `vtxfac::faces_holding_pierce` asks a spline wall of the loft
    /// whether it holds a pierce point of the face `x = 1`, and the
    /// wall's trim cannot say (`HeldPierceUnbuilt`, `placed: false`).
    ///
    /// It reds the day that site places a point in a spline face. The
    /// end-to-end row, the crossings named through the boolean (the
    /// first and third, of one sense, tie), is
    /// `work/splice/a-spline-wall-cannot-say-whether-it-holds-a-pierce-point.md`'s.
    #[test]
    fn a_union_across_a_nurbs_edge_refuses_at_a_spline_walls_held_pierce() {
        let (doc, loft) = wavy_doc(0.3);
        let (doc, profile) = ins(
            doc,
            Node::Profile(ProfileProgram {
                loops: vec![
                    LoopProgram::polygon([(1.0, -3.0), (3.0, -3.0), (3.0, 3.0), (1.0, 3.0)])
                        .expect("finite"),
                ],
                ids: Vec::new(),
                on: None,
            }),
        );
        let (doc, block) = ins(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: crate::test_support::len(4.0),
            },
        );
        let (doc, copy) = crate::test_support::against(
            doc,
            loft,
            block,
            &crate::test_support::axes_at([0.0, 0.0, -0.5], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            Tol::witness(),
        );
        let (doc, union) = ins(
            doc,
            Node::Union {
                members: crate::Bodies::Spelled(vec![
                    crate::BodyRead::plain(loft),
                    crate::BodyRead::plain(crate::Operand::output(copy, 0)),
                ]),
            },
        );
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let body = body_of(ev.value(loft).expect("the loft evaluates"));
        let near: Vec<EdgeKey> = body
            .edges()
            .map(|(e, _)| e)
            .filter(|&e| {
                let (t0, t1) = super::certified_curve(body, e).params();
                let c = carrier(body, e);
                matches!(c, geom::Curve3::Nurbs(_))
                    && ((c.eval(t0).x + c.eval(t1).x) / 2.0 - 1.0).abs() < 1e-9
            })
            .collect();
        assert_eq!(near.len(), 2, "two corner edges straddle x = 1: {near:?}");
        for &e in &near {
            let senses: Vec<bool> = crossings_of(body, e).iter().map(|&(_, s)| s).collect();
            assert!(
                senses.len() == 3 && senses[0] == senses[2] && senses[0] != senses[1],
                "x = 1 crosses corner {e:?} three times, the first and third with one sense: \
                 {senses:?}"
            );
        }
        let Some(crate::eval::NodeError {
            kind:
                crate::eval::NodeErrorKind::Boolean(topo::BooleanError::HeldPierceUnbuilt {
                    operand: topo::Operand::A,
                    face,
                    pierced,
                    kind: geom::SurfaceKind::Nurbs,
                    pierced_kind: geom::SurfaceKind::Plane,
                    placed: false,
                }),
            ..
        }) = ev.node_error(union)
        else {
            panic!(
                "the union refuses where a spline wall of the loft cannot place a pierce \
                 point of the box, got {:?}",
                ev.node_error(union).map(|e| &e.kind)
            );
        };
        // The recipe mints both bodies fresh, so their keys collide and
        // `operand` is what states the owner: `face` is read in the loft,
        // `pierced` in the box.
        assert!(
            matches!(
                body.get_face(*face)
                    .and_then(|f| body.get_surface(f.surface)),
                Some(geom::Surface::Nurbs(_))
            ),
            "the refusal names a spline wall of the loft"
        );
        let block = ev
            .value(copy)
            .expect("the box evaluates")
            .payload
            .output_body(0)
            .expect("the placed box");
        // Where the placed box stands: its body is its source's, in its
        // own coordinates, under its frame (D10 `[placement.frame]`).
        let plane = block
            .body()
            .get_face(*pierced)
            .and_then(|f| block.body().get_surface(f.surface));
        assert!(
            matches!(
                plane,
                Some(geom::Surface::Plane { origin, normal, .. })
                    if (block.point_in_space(*origin).x - 1.0).abs() < 1e-9
                        && block.vec_in_space(*normal).x.abs() > 1.0 - 1e-9
            ),
            "the pierced face is the box's face x = 1"
        );
        let walls: Vec<_> = near
            .iter()
            .filter_map(|&e| body.get_edge(e))
            .flat_map(|e| [e.he_plus, e.he_minus])
            .filter_map(|he| body.face_of_half_edge(he))
            .collect();
        assert!(
            walls.contains(face),
            "the refusal names a wall on a crossed corner: {face:?} of {walls:?}"
        );
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
    use crate::ProfileDoc;
    use crate::edit::DocEdit;
    use crate::eval::{BooleanValue, CancelToken, EvalOptions, ValuePayload, evaluate};
    use crate::ident::DocumentId;
    use crate::names::defer::TieRows;
    use crate::names::role::{EntityKind, NameRef, Qualifier, RoleSeg, StableName};
    use crate::names::table::{Entry, NameTable};
    use crate::node::{Node, RecipeNodeId};
    use crate::program::{LoopProgram, ProfileProgram};
    use crate::test_support::len;
    use geom_core::Tol;

    fn ins(doc: ProfileDoc, node: crate::AuthoredNode) -> (ProfileDoc, RecipeNodeId) {
        let a = crate::apply(
            &doc,
            &DocEdit::InsertNode {
                node: Box::new(node),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("inserts");
        (a.doc, a.record.minted.expect("a node"))
    }

    fn extrude(
        doc: ProfileDoc,
        dz: f64,
        loop_: LoopProgram<crate::Formula>,
    ) -> (ProfileDoc, RecipeNodeId) {
        let (doc, profile) = ins(
            doc,
            Node::Profile(ProfileProgram {
                loops: vec![loop_],
                ids: Vec::new(),
                on: None,
            }),
        );
        ins(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(dz),
            },
        )
    }

    /// `tool` cut from `from`, standing `z0` up `from`'s own z.
    fn cut(
        doc: ProfileDoc,
        from: RecipeNodeId,
        tool: RecipeNodeId,
        z0: f64,
    ) -> (ProfileDoc, RecipeNodeId) {
        let (doc, copy) = crate::test_support::against(
            doc,
            from,
            tool,
            &crate::test_support::axes_at([0.0, 0.0, z0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]),
            Tol::witness(),
        );
        ins(
            doc,
            Node::Subtract {
                from: from.into(),
                tool: crate::BodyRead::plain(crate::Operand::output(copy, 0)),
            },
        )
    }

    #[test]
    fn pieces_of_two_parents_with_one_pair_of_ends_tie() {
        let doc = ProfileDoc::empty(DocumentId::derive("edge-pieces-one-line"), Tol::witness());
        let (doc, rod) = extrude(
            doc,
            1.5,
            LoopProgram::circle_split(0.0, 0.0, 1.0, 2, 0.0).unwrap(),
        );
        let (doc, top) = extrude(
            doc,
            1.0,
            LoopProgram::polygon([(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)])
                .expect("finite"),
        );
        let (doc, cut) = cut(doc, rod, top, 1.0);
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
            .body()
            .edges()
            .map(|(e, _)| e)
            .filter(|&e| {
                let (v0, v1) = super::edge_ends(body.body(), e);
                let z = |v| super::vertex_point(body.body(), v).z;
                v0 != v1 && (z(v0) - 1.0).abs() < 1e-9 && (z(v1) - 1.0).abs() < 1e-9
            })
            .collect();
        assert_eq!(top_rim.len(), 2, "the rim is two half circles: {top_rim:?}");
        let ends = |e| {
            let (v0, v1) = super::edge_ends(body.body(), e);
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
        for (name, entry) in ev.table_at(cut).expect("the cut's table").iter() {
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
                path: vec![RoleSeg::From {
                    read: crate::names::FOLD_A,
                    of: NameRef::new(p),
                }],
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
            name_edge_pieces(
                &mut pieces,
                &t,
                false,
                base,
                (body.body(), 0),
                &[e],
                Lone::Piece,
            );
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
    use topo::construction::Construct;

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
                    topo::construction::MefSite::Lone {
                        r#loop: born.r#loop,
                    },
                    spec,
                    topo::construction::FaceSurface::Inherit,
                    tol,
                )
                .unwrap();
            made.edge
        } else {
            body.mev(
                topo::construction::MevSite::Lone {
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
/// orders at every pose, name through [`name_boolean`] with no
/// refusal. `crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`
/// reads the same scenes' rows against their germs.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod touch_reread_rows {
    use super::{OperandCtx, name_boolean};
    use crate::names::emit::{ent, name1};
    use crate::names::role::{CapEnd, EntityKind, RoleSeg};
    use crate::names::table::{EntityKey, EntityRef, Entry, NameTable};
    use crate::names::{PieceRole, PieceRun, ProfileEdgeRef, ProfileVertexRef};
    use crate::node::{RecipeNodeId, StepId};
    use geom_core::Tol;
    use topo::test_support::meeting::{
        PLATE, Pose, along_lying_ray, apex_pyramid, bearing, corners, lying, mix, near_flat, nest,
        nest_polygon, posed_box, poses,
    };
    use topo::{AtRestBody, BooleanResult, VertexKey, intersect, subtract, union};

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

    /// What naming the ops of one scene found.
    #[derive(Default)]
    struct Named {
        /// Ops that built.
        built: usize,
        /// Operand corners named as fragments (several result vertices).
        fragmented: usize,
        /// Corner names, bare or fragment, that tie.
        tied_corners: usize,
    }

    /// Names every op on `(x, y)` in both orders.
    fn names(label: &str, x: &AtRestBody<f64>, y: &AtRestBody<f64>, pose: &Pose) -> Named {
        use crate::names::role::Qualifier;
        use crate::names::table::Entry;
        use std::collections::BTreeSet;
        let (xn, yn) = (RecipeNodeId::new(0, 1), RecipeNodeId::new(0, 2));
        let (xt, yt) = (table(x, xn), table(y, yn));
        let mut named = Named::default();
        use topo::BooleanOp::{Intersect, Subtract, Union};
        for (what, (a, an, at), (b, bn, bt), r, op) in [
            (
                "x − y",
                (x, xn, &xt),
                (y, yn, &yt),
                subtract(x, y, t()),
                Subtract,
            ),
            (
                "y − x",
                (y, yn, &yt),
                (x, xn, &xt),
                subtract(y, x, t()),
                Subtract,
            ),
            ("x ∪ y", (x, xn, &xt), (y, yn, &yt), union(x, y, t()), Union),
            ("y ∪ x", (y, yn, &yt), (x, xn, &xt), union(y, x, t()), Union),
            (
                "x ∩ y",
                (x, xn, &xt),
                (y, yn, &yt),
                intersect(x, y, t()),
                Intersect,
            ),
            (
                "y ∩ x",
                (y, yn, &yt),
                (x, xn, &xt),
                intersect(y, x, t()),
                Intersect,
            ),
        ] {
            let Ok(BooleanResult::Body(r)) = r else {
                continue;
            };
            let a = OperandCtx {
                node: an,
                read: crate::names::FOLD_A,
                table: at,
                body: a,
            };
            let b = OperandCtx {
                node: bn,
                read: crate::names::FOLD_B,
                table: bt,
                body: b,
            };
            let table =
                match name_boolean(RecipeNodeId::new(0, 9), op, &r.body, &r.naming, &a, &b, t()) {
                    Ok(e) => e.table,
                    Err(e) => panic!("{label}, {}, {what}: names, got {e:?}", pose.label),
                };
            named.built += 1;
            let mut fragmented = BTreeSet::new();
            for (n, e) in table.iter().filter(|(n, _)| n.kind == EntityKind::Vertex) {
                let corner = match n.path.as_slice() {
                    [c @ RoleSeg::From { .. }] => c,
                    [
                        c @ RoleSeg::From { .. },
                        RoleSeg::Fragment(Qualifier::Borders(_)),
                    ] => {
                        fragmented.insert(c.clone());
                        c
                    }
                    _ => continue,
                };
                if matches!(e, Entry::Tied(_)) {
                    eprintln!("{label}, {}, {what}: {corner:?} ties", pose.label);
                    named.tied_corners += 1;
                }
            }
            named.fragmented += fragmented.len();
        }
        named
    }

    /// **A corner the result holds as several vertices is a fragment
    /// of it at each** (N2): "over a void in the bare arch", `x − y` at
    /// rest, leaves `x`'s apex at the plate's top as two result
    /// vertices, each a null copy of it, and each is `x`'s corner +
    /// `Fragment(Borders)` over the walls of `y` that divide the corner
    /// there, the two sets apart.
    #[test]
    fn a_corner_held_as_several_result_vertices_is_a_fragment_at_each() {
        use crate::names::role::Qualifier;
        use crate::names::table::Entry;
        let pose = &poses()[0];
        let p = |b: [[f64; 3]; 3]| apex_pyramid(&b, pose, t());
        let arch_base = corners(60.0, 0.5, 0.4);
        let bare_hollow = built(subtract(&p(arch_base), &p(nest(arch_base, 0.7)), t()));
        let over = p(corners(50.0, 0.7, 0.5));
        let (xn, yn) = (RecipeNodeId::new(0, 1), RecipeNodeId::new(0, 2));
        let (xt, yt) = (table(&over, xn), table(&bare_hollow, yn));
        let Ok(BooleanResult::Body(r)) = subtract(&over, &bare_hollow, t()) else {
            panic!("x − y builds");
        };
        let a = OperandCtx {
            node: xn,
            read: crate::names::FOLD_A,
            table: &xt,
            body: &over,
        };
        let b = OperandCtx {
            node: yn,
            read: crate::names::FOLD_B,
            table: &yt,
            body: &bare_hollow,
        };
        let named = name_boolean(
            RecipeNodeId::new(0, 9),
            topo::BooleanOp::Subtract,
            &r.body,
            &r.naming,
            &a,
            &b,
            t(),
        )
        .expect("x − y names");
        let ties: Vec<&crate::names::StableName> = named
            .table
            .iter()
            .filter(|(n, e)| n.kind == EntityKind::Vertex && matches!(e, Entry::Tied(_)))
            .map(|(n, _)| n)
            .collect();
        assert!(ties.is_empty(), "no vertex name ties: {ties:?}");
        let corner_of_x = |seg: &RoleSeg| {
            matches!(seg, RoleSeg::From { read, of }
                if *read == crate::names::FOLD_A
                    && matches!(of.path.as_slice(), [RoleSeg::CapVertex(..)]))
        };
        let fragments: Vec<(&RoleSeg, &Vec<crate::names::StableName>)> = named
            .table
            .iter()
            .filter(|(n, _)| n.kind == EntityKind::Vertex)
            .filter_map(|(n, _)| match n.path.as_slice() {
                [corner, RoleSeg::Fragment(Qualifier::Borders(walls))] if corner_of_x(corner) => {
                    Some((corner, walls))
                }
                _ => None,
            })
            .collect();
        let [(corner, one), (also, other)] = fragments.as_slice() else {
            panic!("two fragments of x's corner: {fragments:?}");
        };
        assert_eq!(corner, also, "both fragments are of one corner");
        assert_ne!(one, other, "the two copies' walls tell them apart");
        for walls in [one, other] {
            assert!(
                !walls.is_empty()
                    && walls.iter().all(|w| w.kind == EntityKind::Face
                        && matches!(w.path.as_slice(), [RoleSeg::From { read, of }]
                            if *read == crate::names::FOLD_B && of.node == yn)),
                "each copy's walls are y's faces as the boolean carries them in: {walls:?}"
            );
        }
        let bare = named
            .table
            .iter()
            .any(|(n, _)| n.kind == EntityKind::Vertex && n.path == [(*corner).clone()]);
        assert!(!bare, "a corner held as several vertices has no bare name");
    }

    /// [`table`], with the vertices `tied` one tied row, under the
    /// first one's name.
    fn table_tying(body: &AtRestBody<f64>, node: RecipeNodeId, tied: [VertexKey; 2]) -> NameTable {
        let plain = table(body, node);
        let at = tied.map(|v| ent(0, EntityKey::Vertex(v)));
        let name = plain
            .name_of(&at[0])
            .expect("a tied vertex is named")
            .clone();
        let mut t = NameTable::new();
        for (n, e) in plain.iter() {
            let Entry::Unique(r) = e else {
                unreachable!("`table` names every entity apart")
            };
            if !at.contains(r) {
                t.insert(n.clone(), *r).unwrap();
            }
        }
        t.insert_tied(name, at.to_vec()).unwrap();
        t
    }

    /// The scene of
    /// [`a_corner_held_as_several_result_vertices_is_a_fragment_at_each`]:
    /// `x`, `y`, and `x − y`.
    fn over_a_void() -> (AtRestBody<f64>, AtRestBody<f64>, topo::BooleanBody<f64>) {
        let pose = &poses()[0];
        let p = |b: [[f64; 3]; 3]| apex_pyramid(&b, pose, t());
        let arch_base = corners(60.0, 0.5, 0.4);
        let bare_hollow = built(subtract(&p(arch_base), &p(nest(arch_base, 0.7)), t()));
        let over = p(corners(50.0, 0.7, 0.5));
        let Ok(BooleanResult::Body(r)) = subtract(&over, &bare_hollow, t()) else {
            panic!("x − y builds");
        };
        (over, bare_hollow, r)
    }

    /// `x − y` of [`over_a_void`] named with `xt` as `x`'s table.
    fn name_over_a_void(
        (x, y, r): &(AtRestBody<f64>, AtRestBody<f64>, topo::BooleanBody<f64>),
        xt: &NameTable,
    ) -> Result<std::sync::Arc<NameTable>, crate::names::NamingError> {
        let yt = table(y, RecipeNodeId::new(0, 2));
        let a = OperandCtx {
            node: RecipeNodeId::new(0, 1),
            read: crate::names::FOLD_A,
            table: xt,
            body: x,
        };
        let b = OperandCtx {
            node: RecipeNodeId::new(0, 2),
            read: crate::names::FOLD_B,
            table: &yt,
            body: y,
        };
        let op = topo::BooleanOp::Subtract;
        Ok(name_boolean(RecipeNodeId::new(0, 9), op, &r.body, &r.naming, &a, &b, t())?.table)
    }

    /// **Each candidate of a tied corner is a corner of its own** (N4,
    /// "A tie's candidates keep their identity"): in `x − y` of
    /// [`over_a_void`], `x`'s apex is held as two vertices and other
    /// corners of `x` as one each. With two whole corners tied in `x`'s
    /// table, at two points, the result ties them under the bare name.
    /// With the apex tied to a whole corner, the apex's copies are the
    /// fragments they are untied, each `Fragment(Borders)`, and the whole
    /// corner stays under the bare name, narrowed to it.
    #[test]
    fn a_tied_corners_candidates_are_named_apart_wherever_they_lie() {
        use crate::names::role::Qualifier;
        let scene = over_a_void();
        let x = &scene.0;
        let xn = RecipeNodeId::new(0, 1);
        let plain = name_over_a_void(&scene, &table(x, xn)).expect("x − y names");
        let xt = table(x, xn);
        // `x`'s vertex under the name `of`, and the corners of `x` the
        // plain naming holds whole (one result vertex) or as fragments.
        let vertex_of = |of: &crate::names::StableName| match xt.lookup(of) {
            Some(Entry::Unique(r)) => match r.key {
                EntityKey::Vertex(v) => v,
                other => panic!("{of:?} names {other:?}"),
            },
            other => panic!("{of:?}: {other:?}"),
        };
        let mut whole = Vec::new();
        let mut apex_copies: Vec<(Vec<crate::names::StableName>, Entry)> = Vec::new();
        let mut apex = None;
        for (n, e) in plain.iter().filter(|(n, _)| n.kind == EntityKind::Vertex) {
            match n.path.as_slice() {
                [RoleSeg::From { read, of }] if *read == crate::names::FOLD_A => {
                    whole.push((vertex_of(of), e.clone()));
                }
                [
                    RoleSeg::From { read, of },
                    RoleSeg::Fragment(Qualifier::Borders(walls)),
                ] if *read == crate::names::FOLD_A => {
                    apex = Some(vertex_of(of));
                    apex_copies.push((walls.clone(), e.clone()));
                }
                _ => {}
            }
        }
        let apex = apex.expect("x's apex is held as several vertices");
        assert_eq!(apex_copies.len(), 2, "the apex's copies: {apex_copies:?}");
        let [(w0, e0), (w1, _), ..] = whole.as_slice() else {
            panic!("two of x's corners are held whole: {whole:?}");
        };
        let at = |v: VertexKey| {
            let p = crate::names::emit::vertex_point(x, v);
            [p.x, p.y, p.z]
        };
        assert_ne!(at(*w0), at(*w1), "the whole corners lie apart");
        // The tied name, as the result carries it in.
        let carried = |t: &NameTable| {
            let of = t.name_of(&ent(0, EntityKey::Vertex(*w0))).unwrap().clone();
            name1(
                EntityKind::Vertex,
                RecipeNodeId::new(0, 9),
                RoleSeg::From {
                    read: crate::names::FOLD_A,
                    of: of.into(),
                },
            )
        };
        let rows_under = |t: &NameTable, base: &crate::names::StableName| {
            t.iter()
                .filter(|(n, _)| {
                    n.kind == EntityKind::Vertex && n.path.first() == base.path.first()
                })
                .map(|(n, e)| (n.clone(), e.clone()))
                .collect::<Vec<_>>()
        };

        // Both candidates whole: one tied row, the two whole vertices.
        let both = table_tying(x, xn, [*w0, *w1]);
        let named = name_over_a_void(&scene, &both).expect("two whole candidates name");
        let base = carried(&both);
        let rows = rows_under(&named, &base);
        let [(n, Entry::Tied(es))] = rows.as_slice() else {
            panic!("the whole candidates tie under the bare name: {rows:?}");
        };
        assert_eq!(*n, base, "the bare name");
        let held: Vec<EntityRef> = [w0, w1]
            .iter()
            .map(
                |&&w| match &whole.iter().find(|(v, _)| *v == w).unwrap().1 {
                    Entry::Unique(r) => *r,
                    other => panic!("{other:?}"),
                },
            )
            .collect();
        let mut es = es.clone();
        es.sort();
        let mut held_sorted = held.clone();
        held_sorted.sort();
        assert_eq!(es, held_sorted, "the tie holds the two whole vertices");

        // One candidate divided: its copies take `Borders`, as untied,
        // and the whole one keeps the bare name.
        let divided = table_tying(x, xn, [*w0, apex]);
        let named = name_over_a_void(&scene, &divided).expect("a divided candidate names");
        let base = carried(&divided);
        let rows = rows_under(&named, &base);
        let mut bare = Vec::new();
        let mut fragments = Vec::new();
        for (n, e) in rows {
            match n.path.as_slice() {
                [_] => bare.push(e),
                [_, RoleSeg::Fragment(Qualifier::Borders(walls))] => {
                    fragments.push((walls.clone(), e));
                }
                other => panic!("a row of the tied corner: {other:?}"),
            }
        }
        assert_eq!(
            bare,
            vec![e0.clone()],
            "the whole candidate keeps the bare name"
        );
        fragments.sort_by(|p, q| p.0.cmp(&q.0));
        apex_copies.sort_by(|p, q| p.0.cmp(&q.0));
        assert_eq!(
            fragments, apex_copies,
            "the divided candidate's copies, as untied"
        );
    }

    /// At rest, every scene: seven operand corners are held as several
    /// result vertices, and each copy's walls tell it apart. They are
    /// `x`'s apex, where `y`'s void meets it, in `x − y` over a void in
    /// the arch, the bare arch, the quad arch and the bare quad arch and
    /// over the void with the island in it, and in `x ∩ y` crossing the
    /// island in the void and hanging across the near-flat quad void.
    #[test]
    fn a_vertex_read_twice_names_its_result_in_every_op() {
        let named = every_scene(&poses()[..1]);
        assert!(named.built > 0, "cells built");
        assert_eq!(named.fragmented, 7, "corners held as several vertices");
        assert_eq!(named.tied_corners, 0, "corner names that tie");
    }

    /// The other poses, every scene: the same seven corners at each, as
    /// a pose moves no scene's topology.
    #[test]
    fn a_vertex_read_twice_names_its_result_in_every_op_at_every_pose() {
        let rest = &poses()[1..];
        let named = every_scene(rest);
        assert!(named.built > 0, "cells built");
        assert_eq!(
            named.fragmented,
            7 * rest.len(),
            "corners held as several vertices"
        );
        assert_eq!(named.tied_corners, 0, "corner names that tie");
    }

    fn every_scene(poses: &[Pose]) -> Named {
        let mut named = Named::default();
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
            // A dart, whose apex is a reflex edge: on the plate, bare,
            // and apart from the arch in one body, a pyramid in its cone.
            let dart = |b: f64| {
                apex_pyramid(
                    &[
                        bearing(b - 30.0, 0.45, 0.5),
                        bearing(b, 0.6, 0.5),
                        bearing(b + 30.0, 0.45, 0.5),
                        bearing(b, 0.5, 0.5),
                    ],
                    pose,
                    t(),
                )
            };
            let dart_one = built(union(&plate, &dart(60.0), t()));
            let arch_dart = built(union(&arch, &dart(230.0), t()));
            let in_dart = p([
                bearing(222.0, 0.4, 0.4),
                bearing(238.0, 0.4, 0.4),
                bearing(230.0, 0.448, 0.4),
            ]);
            // A near-flat quadrilateral void buried in the block, its
            // apex a reflex edge, and an island in it.
            let q = near_flat(-1e-3);
            let flat_island = built(union(
                &built(subtract(&block, &apex_pyramid(&q, pose, t()), t())),
                &apex_pyramid(&nest_polygon(&q, 0.6), pose, t()),
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
                ("beside a dart", &cone, &dart_one),
                ("over a dart", &over, &dart_one),
                (
                    "inside a bare dart beside a bare arch",
                    &in_dart,
                    &arch_dart,
                ),
                (
                    "near-flat quad void -1e-3 buried, island, hang",
                    &hang,
                    &flat_island,
                ),
                (
                    "near-flat quad void -1e-3 buried, island, hang_over",
                    &hang_over,
                    &flat_island,
                ),
            ] {
                let n = names(label, x, y, pose);
                named.built += n.built;
                named.fragmented += n.fragmented;
                named.tied_corners += n.tied_corners;
            }
            // A pyramid lying on the top, its ray along it, and the
            // pyramids beside it and beside that ray.
            let lying = built(union(&plate, &p(lying(0.0)), t()));
            for (label, x) in [
                ("standing beside a lying pyramid", cone.clone()),
                ("over a lying pyramid", over.clone()),
                ("hanging below a lying pyramid", hang.clone()),
            ]
            .into_iter()
            .chain(along_lying_ray().map(|(label, base)| (label, p(base))))
            {
                let n = names(label, &x, &lying, pose);
                named.built += n.built;
                named.fragmented += n.fragmented;
                named.tied_corners += n.tied_corners;
            }
        }
        named
    }
}

/// **Which piece of a shared curved rim a chord lies within**
/// ([`rim_holding`]), on a real body: the 270° arc prism — the unit arc
/// from azimuth −90° counter-clockwise to 180°, closed through the
/// origin, extruded from z = 0 — with its bottom rim split at 45° and
/// 120° into three pieces the bottom cap and the arc wall share. The
/// chords are arcs of the same circle in a body of their own, carried
/// from their own start, so a chord that starts before 0° wraps its
/// carrier's period.
#[cfg(test)]
mod rim_holding_rows {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use geom_core::{Point2, Point3, Tol, Vec3};
    use topo::construction::Construct;
    use topo::{Body, EdgeKey, FaceKey};

    use super::{chord_within, rim_holding, rims_between};
    use crate::names::discriminate::band;

    const DEG: f64 = std::f64::consts::PI / 180.0;

    fn circle() -> geom::Curve3<f64> {
        geom::Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        }
    }

    fn at(deg: f64) -> Point3<f64> {
        circle().eval(deg * DEG)
    }

    /// The prism unsplit: its bottom cap and arc wall, and the one rim
    /// arc they share.
    fn prism() -> (Body<f64>, FaceKey, FaceKey, EdgeKey) {
        let tol = Tol::witness();
        let body = sweep::test_support::prism_at(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(0.0, -1.0), (270.0 * DEG / 4.0).tan()),
                (Point2::new(-1.0, 0.0), 0.0),
            ],
            0.0,
            1.0,
            tol,
        );
        let rim = body
            .edges()
            .map(|(k, _)| k)
            .find(|&k| {
                let (v0, v1) = super::edge_ends(&body, k);
                let z = |v| super::vertex_point(&body, v).z;
                topo::query::edge_carrier_kind(&body, k) == Some(topo::query::CurveKind::Circle)
                    && z(v0).abs() < 1e-12
                    && z(v1).abs() < 1e-12
            })
            .expect("the prism has a bottom rim arc");
        let faces: Vec<FaceKey> = {
            let e = body.get_edge(rim).unwrap();
            [e.he_plus, e.he_minus]
                .into_iter()
                .map(|he| body.face_of_half_edge(he).unwrap())
                .collect()
        };
        (body.clone(), faces[0], faces[1], rim)
    }

    /// The prism, its bottom cap and arc wall, and the three pieces of
    /// the rim they share, in azimuth order.
    fn split_prism() -> (Body<f64>, FaceKey, FaceKey, [EdgeKey; 3]) {
        let tol = Tol::witness();
        let (mut body, f0, f1, rim) = prism();
        let faces = [f0, f1];
        let param_at = |body: &Body<f64>, k: EdgeKey, deg: f64| {
            let c = body
                .get_edge(k)
                .and_then(|d| body.get_curve_geom(d.curve))
                .and_then(topo::CurveGeom::certified)
                .unwrap();
            let (t0, t1) = c.params();
            c.carrier()
                .param_near(at(deg), geom::mid_param(t0, t1))
                .unwrap()
        };
        let t = param_at(&body, rim, 45.0);
        let second = body.split_edge(rim, t, tol).unwrap().new_edge;
        let (first, rest) = if within_deg(&body, rim, 0.0) {
            (rim, second)
        } else {
            (second, rim)
        };
        let t = param_at(&body, rest, 120.0);
        let third_new = body.split_edge(rest, t, tol).unwrap().new_edge;
        let (middle, last) = if within_deg(&body, rest, 90.0) {
            (rest, third_new)
        } else {
            (third_new, rest)
        };
        let mut shared = rims_between(&body, faces[0], faces[1]);
        shared.sort_unstable();
        let mut pieces = vec![first, middle, last];
        pieces.sort_unstable();
        assert_eq!(
            shared, pieces,
            "the cap and the wall share the three pieces"
        );
        (body, faces[0], faces[1], [first, middle, last])
    }

    /// Whether `deg` lies strictly inside arc edge `k`'s span.
    fn within_deg(body: &Body<f64>, k: EdgeKey, deg: f64) -> bool {
        let (v0, v1) = super::edge_ends(body, k);
        let az = |v| {
            let p = super::vertex_point(body, v);
            p.y.atan2(p.x) / DEG
        };
        let (a, b) = (az(v0).min(az(v1)), az(v0).max(az(v1)));
        // Every piece's span lies within (−90°, 180°].
        a < deg && deg < b
    }

    /// A body whose one edge is the arc of circle `c` from its own
    /// parameter `t0` to `t1`, and that edge.
    fn arc_on(c: &geom::Curve3<f64>, t0: f64, t1: f64) -> (Body<f64>, EdgeKey) {
        let mut body = Body::<f64>::new();
        let born = body.mvfs(c.eval(t0), true).unwrap();
        let spec = geom_brep::EdgeCurveSpec::arc_of_circle(c.clone(), t0, t1).unwrap();
        let e = body
            .mev(
                topo::construction::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                c.eval(t1),
                spec,
                Tol::witness(),
            )
            .unwrap()
            .edge;
        (body, e)
    }

    /// A body whose one edge is the unit circle's arc from `d0` to `d1`
    /// degrees, counter-clockwise, at z = 0, and that edge.
    fn chord_arc(d0: f64, d1: f64) -> (Body<f64>, EdgeKey) {
        arc_on(&circle(), d0 * DEG, d1 * DEG)
    }

    /// A body whose one edge is the segment from `p` to `q`.
    fn chord_line(p: Point3<f64>, q: Point3<f64>) -> (Body<f64>, EdgeKey) {
        let mut body = Body::<f64>::new();
        let born = body.mvfs(p, true).unwrap();
        let e = body
            .mev_line(
                topo::construction::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                q,
                Tol::witness(),
            )
            .unwrap()
            .edge;
        (body, e)
    }

    fn holding(chord: (Body<f64>, EdgeKey)) -> Option<EdgeKey> {
        let (op, f0, f1, _) = split_prism();
        let bnd = band(Tol::witness()).unwrap();
        rim_holding(&chord.0, chord.1, &op, f0, f1, bnd).unwrap()
    }

    /// A chord inside the middle piece is held by it.
    #[test]
    fn a_curved_chord_is_held_by_the_piece_it_lies_within() {
        let pieces = split_prism().3;
        assert_eq!(holding(chord_arc(60.0, 100.0)), Some(pieces[1]));
    }

    /// A chord across the split at 45° lies within neither piece: no
    /// piece holds it, and the caller refuses `SharedRim`.
    #[test]
    fn a_chord_across_a_split_is_held_by_no_piece() {
        assert_eq!(holding(chord_arc(30.0, 60.0)), None);
    }

    /// **A point on a curved edge is placed along its carrier, never by
    /// its chord**: of the rim's three pieces (−90°–45°, 45°–120°,
    /// 120°–180°), a point on the arc inside the middle one is held by
    /// it alone, the midpoint of that piece's chord (inside the circle,
    /// on no piece) by none, a split point by the two pieces it ends,
    /// and the rim's own ends by the piece each ends.
    #[test]
    fn a_point_on_a_curved_piece_is_held_along_its_carrier() {
        let (body, _, _, pieces) = split_prism();
        let bnd = band(Tol::witness()).unwrap();
        let held = |p: Point3<f64>| {
            super::edges_holding(p, pieces.iter().map(|&k| (k, &body, k)), bnd).unwrap()
        };
        let chord_mid = at(45.0) + (at(120.0) - at(45.0)) * 0.5;
        assert_eq!(
            held(at(80.0)),
            vec![pieces[1]],
            "on the arc, inside the middle piece"
        );
        assert_eq!(
            held(chord_mid),
            Vec::<EdgeKey>::new(),
            "the middle piece's chord"
        );
        let mut split = held(at(45.0));
        split.sort_unstable();
        let mut ends = vec![pieces[0], pieces[1]];
        ends.sort_unstable();
        assert_eq!(split, ends, "the split at 45°");
        assert_eq!(held(at(-90.0)), vec![pieces[0]], "the rim's start");
        assert_eq!(held(at(180.0)), vec![pieces[2]], "the rim's end");
    }

    /// A chord ending exactly at the split at 45° is held by the piece
    /// it lies along, not by the piece that only meets it there.
    #[test]
    fn a_chord_ending_at_a_piece_end_is_held_by_that_piece() {
        let pieces = split_prism().3;
        assert_eq!(holding(chord_arc(45.0, 90.0)), Some(pieces[1]));
        assert_eq!(holding(chord_arc(10.0, 45.0)), Some(pieces[0]));
    }

    /// A chord through 0° lies within the first piece, which starts at
    /// −90°: read from the chord's start, the piece's stretch is the one
    /// a period back.
    #[test]
    fn a_chord_through_zero_is_held_through_the_stretch_a_period_back() {
        let pieces = split_prism().3;
        assert_eq!(holding(chord_arc(-20.0, 20.0)), Some(pieces[0]));
    }

    /// The one rim the cap and the wall share holds a chord within it,
    /// and not one that runs past its end at 180°: the lone curved rim
    /// is read along its carrier as each of several is.
    #[test]
    fn a_lone_curved_rim_holds_only_a_chord_within_it() {
        let (op, _, _, rim) = prism();
        let bnd = band(Tol::witness()).unwrap();
        let on = |(b, e): (Body<f64>, EdgeKey)| chord_within(&b, e, &op, rim, bnd).unwrap();
        assert!(on(chord_arc(60.0, 100.0)), "a chord inside the rim");
        assert!(on(chord_arc(-20.0, 20.0)), "a chord through 0°");
        assert!(!on(chord_arc(170.0, 200.0)), "a chord past the rim's end");
        assert!(
            !on(chord_line(at(-90.0), at(180.0))),
            "a straight chord between the rim's ends"
        );
    }

    /// A straight chord between the middle piece's own ends lies within
    /// no curved piece, though both its ends lie on that piece's chord.
    #[test]
    fn a_straight_chord_is_held_by_no_curved_piece() {
        assert_eq!(holding(chord_line(at(45.0), at(120.0))), None);
    }

    /// The circle of `radius` about `center` in the plane z = 0, about
    /// `axis` (±z), its parameter zero toward +x.
    fn circle_of(center: [f64; 2], radius: f64, axis: Vec3<f64>) -> geom::Curve3<f64> {
        geom::Curve3::Circle {
            center: Point3::new(center[0], center[1], 0.0),
            axis,
            radius,
            u_ref: Vec3::unit_x(),
        }
    }

    /// **Two arcs lie on one line when they lie on one circle, and run
    /// the way their tangents agree** (`EdgeCarrier::runs_with`), so the
    /// lines at a vertex count coincident arcs once (`sets_per_line`):
    /// an arc of the unit circle runs with a disjoint arc of it the
    /// same way, and with a clockwise one (carried by the circle about
    /// −z) the other way; an arc of another circle through both ends of
    /// the base arc, a straight edge, and an arc of a wider circle are
    /// not on its line.
    #[test]
    fn arcs_of_one_circle_are_one_line_either_way_along_it() {
        use super::{EdgeCarrier, sets_per_line};
        use crate::names::discriminate::ON_MEMBER_EDGE;
        let bnd = band(Tol::witness()).unwrap();
        let line = |(b, e): (Body<f64>, EdgeKey)| EdgeCarrier::of_edge(&b, e).unwrap();
        let base = line(chord_arc(0.0, 90.0));
        let runs = |other: (Body<f64>, EdgeKey)| {
            base.runs_with(&line(other), ON_MEMBER_EDGE, bnd).unwrap()
        };
        let clockwise = circle_of([0.0, 0.0], 1.0, -Vec3::unit_z());
        assert_eq!(
            runs(chord_arc(120.0, 200.0)),
            Some(true),
            "a disjoint arc, counter-clockwise"
        );
        assert_eq!(
            runs(arc_on(&clockwise, -60.0 * DEG, -10.0 * DEG)),
            Some(false),
            "an overlapping arc, clockwise"
        );
        // The unit circle about (1, 1) passes through (0, 1) at 180°
        // and (1, 0) at 270°, the base arc's two ends.
        let through_ends = circle_of([1.0, 1.0], 1.0, Vec3::unit_z());
        assert_eq!(
            runs(arc_on(&through_ends, 180.0 * DEG, 270.0 * DEG)),
            None,
            "an arc of another circle through the base arc's ends"
        );
        assert_eq!(
            runs(chord_line(at(0.0), at(90.0))),
            None,
            "the straight chord"
        );
        let wider = circle_of([0.0, 0.0], 2.0, Vec3::unit_z());
        assert_eq!(
            runs(arc_on(&wider, 0.0, 90.0 * DEG)),
            None,
            "an arc of a wider circle"
        );
        let edges = [
            chord_arc(0.0, 90.0),
            arc_on(&clockwise, -60.0 * DEG, -10.0 * DEG),
            chord_arc(200.0, 250.0),
        ];
        let lines = sets_per_line(
            0..edges.len(),
            |&i| EdgeCarrier::of_edge(&edges[i].0, edges[i].1).map(Some),
            bnd,
        )
        .unwrap();
        assert_eq!(
            lines,
            vec![vec![0, 1, 2]],
            "three arcs of one circle are one line, the set of them"
        );
    }

    /// **An arc a few tolerances long escalates rather than guess its
    /// way** (`Track::runs_with`): an arc two escalate bands long on the
    /// base arc's circle lies on it, and the two stretches it compares
    /// differ by a quarter of its length, half an escalate band, inside
    /// the band at the default ambiguity factor K = 10. At K ≤ 4 the same
    /// half escalate band can fall in the zero band instead, which
    /// [`an_arc_a_few_tolerances_long_at_a_small_k_refuses_typed`] holds.
    #[test]
    fn an_arc_a_few_tolerances_long_escalates_its_way() {
        use super::EdgeCarrier;
        use crate::names::discriminate::ON_MEMBER_EDGE;
        use crate::names::emit::NamingError;
        let bnd = band(Tol::witness()).unwrap();
        let line = |(b, e): (Body<f64>, EdgeKey)| EdgeCarrier::of_edge(&b, e).unwrap();
        let base = line(chord_arc(0.0, 90.0));
        let t = 30.0 * DEG;
        let short = line(arc_on(&circle(), t, t + 2.0 * bnd.escalate()));
        let way = base.runs_with(&short, ON_MEMBER_EDGE, bnd);
        assert!(
            matches!(way, Err(NamingError::Escalated { .. })),
            "an arc two escalate bands long escalates its way, got {way:?}"
        );
    }

    /// **At a small ambiguity factor the way an arc runs is a precision
    /// limit, refused typed, not a panic** (`Track::runs_with`): under
    /// K = 1.5, which `Tolerance` accepts (any finite K > 1), an arc two
    /// escalate bands long compares two stretches 0.75 zero bands apart,
    /// inside the zero band.
    #[test]
    fn an_arc_a_few_tolerances_long_at_a_small_k_refuses_typed() {
        use super::EdgeCarrier;
        use crate::names::discriminate::ON_MEMBER_EDGE;
        use crate::names::emit::NamingError;
        // A wide band, so the arc spans the witness tolerance's escalate
        // band too and the body that holds it builds.
        let zero = 1e-3;
        let bnd = geom_core::Band::new(zero, 1.5 * zero).unwrap();
        let line = |(b, e): (Body<f64>, EdgeKey)| EdgeCarrier::of_edge(&b, e).unwrap();
        let base = line(chord_arc(0.0, 90.0));
        let t = 30.0 * DEG;
        let short = line(arc_on(&circle(), t, t + 2.0 * bnd.escalate()));
        let way = base.runs_with(&short, ON_MEMBER_EDGE, bnd);
        assert!(
            matches!(
                way,
                Err(NamingError::WayInZeroBand {
                    shape: crate::names::WayShape::Carrier,
                    ..
                })
            ),
            "at K = 1.5 the arc's way decides zero and refuses typed, got {way:?}"
        );
        let shown = way.unwrap_err().to_string();
        assert!(
            shown.contains("ambiguity factor K"),
            "the refusal names its lever: {shown}"
        );
    }

    /// **At a small ambiguity factor a segment across a line can lie on
    /// it and run neither way, refused typed** (`Segment::runs_with`):
    /// under K = 1.5 a segment 1.8 zero bands long, square across the
    /// base line with each end 0.9 zero bands off it, spans the escalate
    /// band, lies on the line at both ends, and projects to nothing.
    #[test]
    fn a_segment_across_a_line_at_a_small_k_refuses_typed() {
        use super::EdgeCarrier;
        use crate::names::discriminate::ON_MEMBER_EDGE;
        use crate::names::emit::NamingError;
        let zero = 1e-3;
        let bnd = geom_core::Band::new(zero, 1.5 * zero).unwrap();
        let line = |(b, e): (Body<f64>, EdgeKey)| EdgeCarrier::of_edge(&b, e).unwrap();
        let base = line(chord_line(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ));
        let across = line(chord_line(
            Point3::new(0.5, 0.9 * zero, 0.0),
            Point3::new(0.5, -0.9 * zero, 0.0),
        ));
        let way = base.runs_with(&across, ON_MEMBER_EDGE, bnd);
        assert!(
            matches!(
                way,
                Err(NamingError::WayInZeroBand {
                    shape: crate::names::WayShape::Line,
                    ..
                })
            ),
            "the segment across the line refuses typed, got {way:?}"
        );
    }

    /// **A spline edge is a line of its own** (`EdgeCarrier::Unread`):
    /// the wavy loft's corner edges are NURBS curves, whose carrier
    /// places no point, so an edge runs with neither itself nor another,
    /// and two of them are two lines at a vertex, as before the curved
    /// reading. No union reaches such an edge today: the boolean refuses
    /// it where it meets the other operand (`EdgeCarrierUnsupported`).
    #[test]
    fn a_spline_edge_is_a_line_of_its_own() {
        use super::{EdgeCarrier, sets_per_line};
        use crate::names::discriminate::ON_MEMBER_EDGE;
        let bnd = band(Tol::witness()).unwrap();
        let (_, value, _) = super::wavy_loft::wavy(0.3);
        let crate::eval::ValuePayload::Body(body) = &value.payload else {
            panic!("the loft is a body");
        };
        let splines: Vec<EdgeKey> = body
            .body()
            .edges()
            .map(|(k, _)| k)
            .filter(|&k| {
                topo::query::edge_carrier_kind(body.body(), k)
                    == Some(topo::query::CurveKind::Nurbs)
            })
            .collect();
        assert!(splines.len() >= 2, "the loft has spline edges: {splines:?}");
        let lines: Vec<EdgeCarrier<f64>> = splines
            .iter()
            .map(|&k| EdgeCarrier::of_edge(body.body(), k).unwrap())
            .collect();
        assert!(
            lines.iter().all(|l| matches!(l, EdgeCarrier::Unread)),
            "every spline edge reads as Unread"
        );
        assert_eq!(
            lines[0].runs_with(&lines[0], ON_MEMBER_EDGE, bnd).unwrap(),
            None,
            "a spline edge runs with nothing, itself included"
        );
        let kept = sets_per_line(
            0..splines.len(),
            |&i| EdgeCarrier::of_edge(body.body(), splines[i]).map(Some),
            bnd,
        )
        .unwrap();
        assert_eq!(
            kept,
            (0..splines.len()).map(|i| vec![i]).collect::<Vec<_>>(),
            "each spline edge is a line of its own"
        );
    }
}

#[cfg(test)]
mod a_pairs_flush_line {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::collections::BTreeSet;

    use super::{OperandCtx, name_boolean};
    use crate::names::emit::{edge_ends, ent, name1, vertex_point};
    use crate::names::role::{CapEnd, EntityKind, RoleSeg, Sense, StableName};
    use crate::names::table::{EntityKey, Entry, NameTable};
    use crate::names::{PieceRole, PieceRun, ProfileEdgeRef, ProfileVertexRef};
    use crate::node::{RecipeNodeId, StepId};
    use geom_core::Tol;
    use topo::test_support::meeting::{Pose, posed_box};
    use topo::{AtRestBody, BooleanResult, EdgeKey, union};

    fn t() -> Tol {
        Tol::witness()
    }

    /// A name for every entity of `body`, the `k`th of its kind by key
    /// named for piece `k`, except that `swap`'s two edges trade names.
    fn table(body: &AtRestBody<f64>, node: RecipeNodeId, swap: [EdgeKey; 2]) -> NameTable {
        let at = |k: usize| ProfileVertexRef::Piece {
            step: StepId::new(0, k as u64),
            role: PieceRole::Leg,
        };
        let name = |k: usize| name1(EntityKind::Edge, node, RoleSeg::LateralEdge(at(k)));
        let keys: Vec<EdgeKey> = body.edges().map(|(e, _)| e).collect();
        let index = |e: EdgeKey| keys.iter().position(|&k| k == e).unwrap();
        let mut t = NameTable::new();
        t.insert(
            name1(EntityKind::Body, node, RoleSeg::OutputBody),
            ent(0, EntityKey::Body),
        )
        .unwrap();
        for &e in &keys {
            let k = match swap {
                [x, y] if e == x => index(y),
                [x, y] if e == y => index(x),
                _ => index(e),
            };
            t.insert(name(k), ent(0, EntityKey::Edge(e))).unwrap();
        }
        for (k, (f, _)) in body.faces().enumerate() {
            let lateral = RoleSeg::Lateral(PieceRun::one(ProfileEdgeRef::Piece {
                step: StepId::new(0, k as u64),
                role: PieceRole::Leg,
            }));
            t.insert(
                name1(EntityKind::Face, node, lateral),
                ent(0, EntityKey::Face(f)),
            )
            .unwrap();
        }
        for (k, (v, _)) in body.vertices().enumerate() {
            let corner = RoleSeg::CapVertex(CapEnd::End, at(k));
            t.insert(
                name1(EntityKind::Vertex, node, corner),
                ent(0, EntityKey::Vertex(v)),
            )
            .unwrap();
        }
        t
    }

    /// The operand edges a carried line name cites, by key: the one edge
    /// or each of a `Merged` set.
    fn cited(line: &StableName, twins: &NameTable) -> BTreeSet<EdgeKey> {
        let one = |n: &StableName| {
            let [RoleSeg::From { of, .. }] = n.path.as_slice() else {
                panic!("a carried operand edge: {n:?}");
            };
            match twins.lookup(of) {
                Some(Entry::Unique(r)) => match r.key {
                    EntityKey::Edge(e) => e,
                    other => panic!("{of:?} names {other:?}"),
                },
                other => panic!("{of:?} is no one edge of the twins: {other:?}"),
            }
        };
        match line.path.as_slice() {
            [RoleSeg::Merged(set)] => set.iter().map(one).collect(),
            _ => BTreeSet::from([one(line)]),
        }
    }

    /// **A pair boolean reads a line of several flush operand edges
    /// along each of them, whichever is least** (N2): two blocks kissing
    /// along the line x = 1.5, y = 1, one turned over so their edges on
    /// it run opposite ways, united with a lid whose floor crosses the
    /// line at z = 1.25. The crossing cites both edges and is `Through`
    /// the one running up into the lid, whichever of the two holds the
    /// lesser name, so trading their names changes nothing by key.
    #[test]
    fn a_crossing_of_two_flush_edges_running_opposite_ways_reads_each() {
        let rest = Pose::rest();
        let flip = Pose::turn("flipped", [1.0, 0.0, 0.0], core::f64::consts::PI, [0.0; 3]);
        let left = posed_box("left", [(1.0, 1.5), (0.5, 1.0), (0.5, 2.0)], &rest, t());
        let right = posed_box(
            "right",
            [(1.5, 2.0), (-1.5, -1.0), (-2.0, -0.5)],
            &flip,
            t(),
        );
        let Ok(BooleanResult::Body(twins)) = union(&left, &right, t()) else {
            panic!("the twins unite");
        };
        let twins = twins.body;
        let lid = posed_box("lid", [(0.5, 2.5), (0.0, 2.0), (1.25, 2.75)], &rest, t());
        let on_line =
            |p: geom_core::Point3<f64>| (p.x - 1.5).abs() < 1e-9 && (p.y - 1.0).abs() < 1e-9;
        let line: Vec<(EdgeKey, bool)> = twins
            .edges()
            .filter_map(|(e, _)| {
                let (s, f) = edge_ends(&twins, e);
                let (s, f) = (vertex_point(&twins, s), vertex_point(&twins, f));
                (on_line(s) && on_line(f)).then_some((e, f.z > s.z))
            })
            .collect();
        let [(x, x_up), (y, y_up)] = line.as_slice() else {
            panic!("the twins hold two edges on their shared line: {line:?}");
        };
        assert_ne!(x_up, y_up, "the twins' edges on the line run opposite ways");
        let up = if *x_up { *x } else { *y };
        let Ok(BooleanResult::Body(r)) = union(&lid, &twins, t()) else {
            panic!("the lid and the twins unite");
        };
        let (an, bn) = (RecipeNodeId::new(0, 1), RecipeNodeId::new(0, 2));
        let lid_table = table(&lid, an, [EdgeKey::default(); 2]);
        let mut read = Vec::new();
        for swap in [false, true] {
            let twins_table = table(&twins, bn, if swap { [*x, *y] } else { [*x; 2] });
            let a = OperandCtx {
                node: an,
                read: crate::names::FOLD_A,
                table: &lid_table,
                body: &lid,
            };
            let b = OperandCtx {
                node: bn,
                read: crate::names::FOLD_B,
                table: &twins_table,
                body: &twins,
            };
            let named = name_boolean(
                RecipeNodeId::new(0, 9),
                topo::BooleanOp::Union,
                &r.body,
                &r.naming,
                &a,
                &b,
                t(),
            )
            .expect("the union names");
            let mut at = Vec::new();
            for (n, e) in named.table.iter() {
                let Entry::Unique(row) = e else { continue };
                let EntityKey::Vertex(v) = row.key else {
                    continue;
                };
                let p = vertex_point(&r.body, v);
                if !on_line(p) || (p.z - 1.25).abs() > 1e-9 {
                    continue;
                }
                let [RoleSeg::Crossing { edge, sense, .. }] = n.path.as_slice() else {
                    panic!("swap {swap}: the crossing of the lid's floor: {n:?}");
                };
                let edges = cited(edge, &twins_table);
                let entering = match sense {
                    Sense::Enters => edges.clone(),
                    Sense::Leaves => BTreeSet::new(),
                    Sense::Through { entering } => entering
                        .iter()
                        .flat_map(|n| cited(n, &twins_table))
                        .collect(),
                    other => panic!("swap {swap}: a crossing reads {other:?}"),
                };
                at.push((edges, entering));
            }
            let [one] = at.as_slice() else {
                panic!("swap {swap}: one vertex at the crossing: {at:?}");
            };
            read.push(one.clone());
        }
        assert_eq!(
            read[0], read[1],
            "trading the edges' names moves the crossing's reading"
        );
        assert_eq!(
            read[0],
            (BTreeSet::from([*x, *y]), BTreeSet::from([up])),
            "the crossing cites both edges and enters along the one running up"
        );
    }
}

//! **The n-ary union's naming** ([`crate::Node::Union`]; DM4).
//!
//! A union's names record WHICH MEMBER an entity came from and not
//! which fold step reached it. The fold's own tables are the pair
//! emitter's, whose two sides are keyed by the fold's own reads
//! ([`crate::names::FOLD_A`], [`crate::names::FOLD_B`]), so a member's
//! entity accumulates a descent chain as long as its position in the
//! list; that chain is what this module takes out, in two halves.
//!
//! **Going in** ([`member_view`]): each member's table enters its fold
//! step already wrapped in one [`RoleSeg::From`] carrying that member's
//! read. Every operand of every step is therefore in the
//! union's own name space, and every name the pair emitter embeds — a
//! seam's two sides, a merge's constituents, a fragment's walls —
//! already says which member it came from. It has to be put there: a
//! pass-through op mints no name (N1), so N placements of one
//! prototype carry N IDENTICAL tables and no inner name can tell them
//! apart.
//!
//! **Coming out** ([`name_union`] for the published table,
//! [`collapse_table`] for the accumulation a fold step reads, and
//! [`collapse_name`] for one name, as the fragment groups read a fold
//! step's bases (`names::groups`) — three consumers of one collapse, which the published table follows with the two end
//! passes below): a fold-table name is rewritten by descending its
//! fold-side chain to the member-keyed [`RoleSeg::From`] at its foot —
//! one wrapper, whatever the depth. The names a segment embeds are
//! collapsed by the same rule. The result is then put in canonical
//! form by `names::canonical`, the one list of a path's name-ordered
//! positions, which the pair emitter's mint and every rewrite of a
//! published name also end in: a `Merged` set and a `Borders` set
//! are sorted, an `Ends` pair is sorted, a junction's run of lines and
//! an `EdgeCrossing`'s run are sorted, a `Through` sense's things are
//! sorted, and a `Seam`'s two sides are put in name order, because a
//! union has no A and B.
//!
//! Putting a `Seam`'s sides in name order rewrites no VALUE: a crossing
//! of a seam edge names the side it enters along, whichever order the
//! pair is written in.
//!
//! More rewrites happen at the END, on the published table only,
//! because they read the finished table or the finished body. Each
//! replaces something the fold wrote down about WHEN it met an entity
//! with something the finished union says about WHAT the entity is:
//! - a face is named for its PARENT, the member faces the pairwise
//!   judgements consumed together, transitively ([`Links`]), which a
//!   finished face takes from the member faces it descends from by
//!   entity through the fold ([`Fold`], [`Parents`]): the parent
//!   itself when the finished body holds it as one face, and
//!   otherwise the parent and one `Borders`
//!   over the divider walls each piece borders; a seam edge is named
//!   for the two parents it lies between; and any other name cites a
//!   face as its parent ([`name_by_parents`]) — whether a step cut a
//!   face before or after it merged, and in how many steps, is fold
//!   history;
//! - an entity of the finished body that belongs to several members at
//!   once (a flush stretch, a corner at another member's corner) is
//!   named for the set of the member entities that hold it, `Merged` of
//!   them, never for one chosen among them; an edge lying within one
//!   member edge is named as a piece of it, and an edge lying within
//!   none but along several that cover it (a flush rim the output stage
//!   joined) is named for that set ([`Flush`], [`group_member_edges`])
//!   — which member was operand A is fold history;
//! - a vertex is named for the member vertex it sits at, `Merged` of
//!   the several where more than one member has a vertex there
//!   ([`Flush::member_vertices`]), each copy of a member corner the
//!   finished body holds as several a `Fragment(Borders)` of it
//!   ([`name_corner_copies`]), or for the lines through it — the
//!   member edge it lies on (or the set of those flush on one line) and
//!   the one face crossing it there, or the run of such lines and seams
//!   that meet there, each member edge's sense read from the members'
//!   pairwise judgements and each seam's from which of its pieces the
//!   result keeps there ([`Flush::meeting`]) — and otherwise cites the member
//!   edge it lies on whole, not the stretch of it the fold had cut when it met the
//!   vertex, and a seam by its head ([`cite_member_edges`]);
//! - every piece of a member edge the finished body does not hold
//!   whole, a lone one included, and several pieces of one seam, are
//!   told apart by their ends, over the vertex names just published
//!   (N2's `Ends`) — which step cut the edge is fold history.
//!
//! These names are functions of the finished body, so they are
//! order-free as far as the boolean's output is, and every output stage
//! leaves maximal faces and maximal edges (`docs/DESIGN.md`), a complex
//! unique to the union's region and face partition.
//!
//! A refusal raised mid-fold carries no fold-table name: a step's
//! refusal is the kernel's own, under the member that step folds in
//! (`NodeErrorKind::UnionFoldStep`). A coincidence row a step decides
//! is read against the collapsed accumulation ([`collapse_table`]) and
//! spelled by the member cells that meet there ([`member_cells`]), as
//! the pairwise judgement's rows are.
//!
//! # How an intermediate row is told from a member's row
//!
//! In a FOLD table, by the HEAD segment alone. Every row of every fold
//! step is minted under the union's id, so the id separates nothing;
//! what separates them is that a member-keyed row's head is
//! `From` a member's read and an intermediate row's is `From` a fold side, which is
//! descended through.
//!
//! That is a statement about the fold's own tables and about nothing
//! else. Once a name is COLLAPSED the fold-side chain is gone,
//! so a published row is told apart by its whole path and not by its
//! head — a member's row is a one-segment `From` path, and
//! anything longer is a row the fold minted. That is the shape
//! `eval::wire`'s `sited_member` reads to say whether a refusal's row
//! is a member's entity, a merge of member entities, or a row no
//! declaration can name, and it is the reason the two questions are
//! asked in two places rather than shared.

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::{Margin, Sign};

use crate::names::borders::Obstacles;
use crate::names::defer::{TieRows, mint_candidates};
use crate::names::discriminate::{ON_MEMBER_EDGE, band};
use crate::names::emit::{
    DisagreeShape, Incidence, NamingError, SeamSidesShape, SeamVertexShape, UnionNameShape,
    check_total, edge_ends, ent, face_half_edges, vertex_point,
};
use crate::names::emit_topo::{
    CornerCopies, Cover, EdgeCarrier, EdgePieces, EdgeReading, Lone, ParentEntities, UNSIDED,
    edge_reading, edges_holding, is_straight, joined_along, joined_candidates,
    line_name as line_name_of, name_corner_copies, name_edge_pieces, name_parent_faces,
    sense_along_line, sets_per_line,
};
use crate::names::groups::{GroupRecord, Parent as GroupParent};
use crate::names::least_root::LeastRoot;
use crate::names::nest::{Descent, Kept, Stopped, descend};
use crate::names::role::{
    Carry, EntityKind, NameRef, Qualifier, RoleSeg, RunLine, SegRewrite, Sense, StableName,
    edge_line, is_fold_side, never_in_a_boolean_table,
};
use crate::names::seam_pair;
use crate::names::table::{EntityKey, Entry, NameTable};
use crate::node::RecipeNodeId;
use crate::var::VarId;

/// One member's table as the UNION sees it: every row's name wrapped
/// in one [`RoleSeg::From`] naming this member's read, minted under the
/// union's node.
///
/// The entities are untouched — same body index, same keys — so the
/// view is a name rewrite and the pair emitter's reverse lookups read
/// it exactly as they read the member's own table.
pub(crate) fn member_view(
    union: RecipeNodeId,
    member: VarId,
    table: &NameTable,
) -> Result<NameTable, NamingError> {
    // The member's table is an operand read whole, so it is sealed
    // here for the same reason `upstream_name` seals a table read one
    // entity at a time — and each row below EMBEDS the member's own
    // handle rather than a copy, so a member name keeps its order
    // cache through the wrapper.
    table.seal_order();
    let mut view = NameTable::new();
    for (name, entry) in table.iter_refs() {
        put_entry(
            &mut view,
            keyed(union, member, name.clone(), name.kind),
            entry,
        )?;
    }
    Ok(view)
}

/// **One piece's view in a `⊔` that reads `member` more than once**:
/// [`member_view`]'s names, each row a candidate of the tie the `⊔`
/// itself names over every piece of that read
/// ([`super::emit::name_assemble`]), numbered on from `next`, which
/// carries the count from one piece of the read to the next. The
/// gather carries the pieces' rows one source at a time, and these
/// candidates merge back into that tie there.
pub(crate) fn repeated_member_view(
    union: RecipeNodeId,
    member: VarId,
    table: &NameTable,
    next: &mut BTreeMap<StableName, u32>,
) -> Result<NameTable, NamingError> {
    table.seal_order();
    let mut view = NameTable::new();
    for (name, row) in table.rows() {
        let wrapped = keyed(union, member, name.clone(), name.kind);
        let k = next.entry(wrapped.clone()).or_default();
        let pairs = row
            .pairs()
            .map(|(_, e)| {
                let c = crate::names::table::Candidate::Of(*k);
                *k += 1;
                (c, e)
            })
            .collect();
        view.insert_candidates_ref(NameRef::new(wrapped), pairs)?;
    }
    Ok(view)
}

/// **The table of a union or an intersect of one member** (DM4): the
/// member's rows keyed by its read, as [`member_view`] keys them, and the
/// body named by the node itself. No boolean runs, so this is the whole
/// naming.
pub(crate) fn name_lone_member(
    union: RecipeNodeId,
    member: VarId,
    table: &NameTable,
) -> Result<NameTable, NamingError> {
    table.seal_order();
    let mut out = NameTable::new();
    out.insert(
        StableName {
            kind: EntityKind::Body,
            node: union,
            path: vec![RoleSeg::OutputBody],
        },
        ent(0, EntityKey::Body),
    )?;
    for (name, entry) in table.iter_refs() {
        if name.kind != EntityKind::Body {
            put_entry(
                &mut out,
                keyed(union, member, name.clone(), name.kind),
                entry,
            )?;
        }
    }
    Ok(out)
}

/// **One entity of one member, in the union's own name space** — the
/// row [`member_view`] puts into that member's operand table, for a
/// test that has the name rather than the table.
#[cfg(test)]
fn member_name(union: RecipeNodeId, member: VarId, name: &StableName) -> StableName {
    keyed(union, member, NameRef::new(name.clone()), name.kind)
}

/// The rule itself: one [`RoleSeg::From`] segment keyed by the member's
/// read, minted under the union's node, carrying the entity's own kind.
fn keyed(
    union: RecipeNodeId,
    read: VarId,
    of: NameRef,
    kind: crate::names::role::EntityKind,
) -> StableName {
    StableName {
        kind,
        node: union,
        path: vec![RoleSeg::From { read, of }],
    }
}

/// Rewrites the fold's final table into member-keyed names.
///
/// `folded` is the table the last fold step emitted (under `node`),
/// `body` the body it names. The rewrite is one-to-one on rows —
/// distinct entities keep distinct names, and a collision would be an
/// emission bug, refused typed by [`NameTable::insert`] rather than
/// aliased. [`check_total`] re-runs afterwards, so the rewritten table
/// is held to the same totality the fold's was.
pub(crate) fn name_union<T: geom_core::Decide>(
    node: RecipeNodeId,
    body: &topo::Body<T>,
    folded: &NameTable,
    members: &[Member<'_, T>],
    fold: &Fold<'_>,
    links: &Links<T>,
    tol: geom_core::Tol,
) -> Result<(Arc<NameTable>, GroupRecord), NamingError> {
    let bnd = band(tol)?;
    let t = collapse_table(node, folded)?;
    let parents = Parents::of(node, body, members, fold, links)?;
    let flush = Flush::of(node, body, members, &parents, bnd)?;
    let by_parents = name_by_parents(node, &t, body, &parents, fold, &flush)?;
    let (t, member_edges, member_lines) =
        group_member_edges(by_parents.table, by_parents.held, &flush)?;
    let lines = EdgeLines {
        member: member_lines,
        seam: by_parents
            .seams
            .iter()
            .flat_map(|g| g.edges.iter().map(move |&k| (k, &g.base)))
            .collect(),
    };
    let mut t = cite_member_edges(
        t,
        by_parents.vertices,
        body,
        (&flush, fold),
        (&lines, links),
    )?;
    let mut tie = TieRows::default();
    let mut pieces = EdgePieces::default();
    for g in by_parents.seams.iter().chain(&member_edges) {
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
    check_total(&t, body, 0)?;
    Ok((Arc::new(t), by_parents.groups))
}

/// The edges of a union's finished body that are pieces of one parent
/// — a seam between two parents, a member edge, or a set of member
/// edges — under that parent's name. They are qualified by their ends
/// once the vertices are named ([`name_edge_pieces`]); a lone piece
/// too, where a set-named edge holds the rest of its member edge.
struct PieceGroup {
    base: StableName,
    from_tie: bool,
    edges: Vec<topo::EdgeKey>,
    lone: Lone,
}

/// One member of a union as [`name_union`] reads it: its read, its own
/// body and its own table — the place a member's edge is defined.
pub(crate) struct Member<'a, T: geom_core::Decide> {
    /// The read the member came through.
    pub read: VarId,
    /// The member's body, in the space the union was built in.
    pub body: &'a topo::Body<T>,
    /// The member's own table (not the union's view of it).
    pub table: &'a NameTable,
}

/// **The pieces of each member edge, grouped under it.**
///
/// A row `From(m, e)` followed only by the fold's
/// `Fragment(Ends)`, `e` an edge, is a piece of `m`'s edge `e`; so is
/// any edge row that lies within a member edge whatever the fold named
/// it (a merged edge the fold met as a seam). Either is a piece of the
/// member edge it lies within, or of the SET of them where it lies
/// within several flush there ([`Flush`]). An edge row that lies
/// within none but runs along several member edges covering it, however
/// the fold named it, is a piece of that SET. Each group is named
/// `From(m, e)` or `Merged` of the set: a member edge alone where
/// the finished body holds it whole ([`Flush::holds_whole`]), and
/// otherwise each piece of it, a lone one included, by its ends over the
/// union's published vertex names, read off the finished body: the
/// fold's qualifiers record which step cut the edge and which member
/// kept a flush stretch, both of which depend on member order.
///
/// Returns the table without those rows, the groups, and each grouped
/// edge's member edge or set.
#[allow(clippy::type_complexity)] // the three outputs of one pass
fn group_member_edges<T: geom_core::Decide>(
    t: NameTable,
    held: Vec<(StableName, Entry)>,
    flush: &Flush<'_, T>,
) -> Result<(NameTable, Vec<PieceGroup>, BTreeMap<topo::EdgeKey, Along>), NamingError> {
    // Member edge, or set of them → (from a tie, its edges, whether
    // every row is an edge the finished body holds as the whole member
    // edge).
    let mut groups: BTreeMap<Along, (bool, Vec<topo::EdgeKey>, bool)> = BTreeMap::new();
    let mut lines = BTreeMap::new();
    let mut out = NameTable::new();
    let rows = t
        .iter()
        .map(|(name, entry)| (name.clone(), entry.clone()))
        .chain(held);
    for (name, entry) in rows {
        let keys = match &entry {
            Entry::Unique(e) => vec![e.key],
            Entry::Tied(es) => es.iter().map(|e| e.key).collect(),
        };
        let edges = keys
            .iter()
            .filter_map(|k| match k {
                EntityKey::Edge(k) => Some(*k),
                _ => None,
            })
            .collect::<Vec<_>>();
        let (key, whole) = match (member_edge_piece(&name), edges.as_slice()) {
            (Some(_), []) => unreachable!(
                "union row {name:?} is a member edge's piece and names no edge but {keys:?}: a \
                 table seats a row only on entities of its name's kind"
            ),
            (_, [k]) if flush.spans.contains_key(k) => (
                Some(Along::Set(flush.spans[k].iter().cloned().collect())),
                true,
            ),
            (Some((member, edge, _)), [k]) => {
                flush.piece_of(*k, flush.within_or(*k, (member, edge)))?
            }
            (Some((member, edge, _)), _) => (Some(Along::One((member, edge))), false),
            (None, [k]) => match flush.within(*k) {
                Some(within) => flush.piece_of(*k, within.clone())?,
                None => (None, false),
            },
            (None, _) => (None, false),
        };
        match key {
            Some(key) => {
                for &k in &edges {
                    lines.insert(k, key.clone());
                }
                let group = groups.entry(key).or_insert((false, Vec::new(), true));
                group.0 |= matches!(entry, Entry::Tied(_));
                group.1.extend(edges);
                group.2 &= whole;
            }
            None => put_entry(&mut out, name, &entry)?,
        }
    }
    let in_sets: BTreeSet<MemberEntity> = groups
        .keys()
        .filter_map(|k| match k {
            Along::Set(set) => Some(set.iter().cloned()),
            Along::One(_) => None,
        })
        .flatten()
        .collect();
    let groups = groups
        .into_iter()
        .map(|(key, (from_tie, edges, whole))| PieceGroup {
            lone: match &key {
                Along::One(e) if in_sets.contains(e) => Lone::Piece,
                Along::One(_) if !(whole && edges.len() == 1) => Lone::Piece,
                _ => Lone::Whole,
            },
            base: match key {
                Along::One(e) => entity_name(flush.union, &e),
                Along::Set(set) => crate::names::merged::edge_set(
                    flush.union,
                    set.iter().map(|e| entity_name(flush.union, e)),
                ),
            },
            from_tie,
            edges,
        })
        .collect();
    Ok((out, groups, lines))
}

/// What each finished edge at a vertex is a line of (N2): a member edge
/// or set ([`group_member_edges`]), or a seam, by its group's base
/// ([`name_by_parents`]).
struct EdgeLines<'g> {
    member: BTreeMap<topo::EdgeKey, Along>,
    seam: BTreeMap<topo::EdgeKey, &'g StableName>,
}

/// What a union's edge is a piece of: one member edge, or the set of
/// member edges it runs along where it lies within none of them.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Along {
    One(MemberEntity),
    Set(Vec<MemberEntity>),
}

/// One row into a table, through the door its entry's shape takes.
fn put_entry(t: &mut NameTable, name: StableName, entry: &Entry) -> Result<(), NamingError> {
    match entry {
        Entry::Unique(e) => t.insert(name, *e),
        Entry::Tied(es) => t.insert_tied(name, es.clone()),
    }?;
    Ok(())
}

/// Member `member`'s edge `edge` in its own body, as its own table
/// names it, a cited line read as the least row on it (N5, "A cited
/// line"). Where that row is tied, its candidates are several edges and
/// the reader needs one, which no rule picks:
/// [`NamingError::MemberEdgeTied`]. A member or name the union does not
/// have is an emission bug (every member-keyed row came from that
/// member's table).
///
/// A constituent of an edge set the member publishes (a nested union's
/// joined edge, which a flat set of the fold lists by its constituents,
/// N3) is read as AN edge on its line, not as the set the citing name
/// means: the stretch it keeps outside every set where it keeps one,
/// else the least set listing it. Every one of those lies on the
/// constituent's line, which is all its one reader asks of it
/// (`Flush::way_of` reads only whether it lies on another edge's line).
fn member_edge<'a, T: geom_core::Decide>(
    members: &'a [Member<'a, T>],
    member: VarId,
    edge: &StableName,
) -> Result<(&'a topo::Body<T>, topo::EdgeKey), NamingError> {
    let (i, k) = member_edge_at(members, member, edge)?;
    Ok((members[i].body, k))
}

/// [`member_edge`], the member given by its index in `members`.
fn member_edge_at<T: geom_core::Decide>(
    members: &[Member<'_, T>],
    member: VarId,
    edge: &StableName,
) -> Result<(usize, topo::EdgeKey), NamingError> {
    let (i, m) = member_at(members, member, edge);
    let by_line = |edge: &StableName| {
        m.table
            .lookup(edge)
            .or_else(|| m.table.lookup(m.table.on_line(edge).first()?))
    };
    let entry = by_line(edge).or_else(|| by_line(m.table.sets_listing(edge).first()?.name()));
    match entry {
        Some(Entry::Unique(e)) => match e.key {
            EntityKey::Edge(k) => Ok((i, k)),
            key => unreachable!(
                "member {member:?}'s edge {edge:?} names {key:?} in its member, on its line or \
                 in a set listing it: a table seats a row only on entities of its name's kind, \
                 and an edge's line and sets are edges"
            ),
        },
        Some(Entry::Tied(_)) => Err(NamingError::MemberEdgeTied {
            member,
            edge: Box::new(edge.clone()),
        }),
        None => unreachable!(
            "member {member:?}'s table holds {edge:?}, on its line or in a set listing it, and \
             answers none of them: `member_at` chose this member for holding one"
        ),
    }
}

/// The member of a union read through `read` whose table holds `name`,
/// on its line or in a set listing it: every member-keyed row came from
/// a member, so a read the union does not hold is an emission bug.
/// Several members share a read where a family is read whole, each
/// holding its own instances' names, and where a read is spelled twice,
/// with one table between them.
#[track_caller]
fn member_of<'a, T: geom_core::Decide>(
    members: &'a [Member<'a, T>],
    read: VarId,
    name: &StableName,
) -> &'a Member<'a, T> {
    member_at(members, read, name).1
}

/// [`member_of`], with the member's index in `members`.
#[track_caller]
fn member_at<'a, T: geom_core::Decide>(
    members: &'a [Member<'a, T>],
    read: VarId,
    name: &StableName,
) -> (usize, &'a Member<'a, T>) {
    let Some(found) = members
        .iter()
        .enumerate()
        .filter(|(_, m)| m.read == read)
        .find(|(_, m)| {
            m.table.lookup(name).is_some()
                || !m.table.on_line(name).is_empty()
                || !m.table.sets_listing(name).is_empty()
        })
    else {
        unreachable!(
            "no member read through {read:?} holds {name:?}, on its line or in a set listing \
             it: every member-keyed name the union reads came from its member's own table, \
             keyed by that member's read (`member_view`, `Flush`)"
        )
    };
    found
}

/// Row `row`, on the line of member `member`'s edge `line`, is not in
/// that member's table.
fn not_in_its_table(member: VarId, line: &StableName, row: &StableName) -> String {
    format!(
        "member {member:?}'s row {row:?} on the line of {line:?} is not in its table: \
         `NameTable::on_line` lists rows of the table it reads"
    )
}

/// Row `row` of member `member`, on an edge's line, names a cell that is
/// no edge.
fn names_no_edge(member: VarId, row: &StableName, key: EntityKey) -> String {
    format!(
        "member {member:?}'s row {row:?} on an edge's line names {key:?}: a table seats a row \
         only on entities of its name's kind, and an edge's line holds edges"
    )
}

/// A member's entity: the member's read, and the entity's name in that
/// member's own table. Ordered as the union's [`RoleSeg::From`] names
/// for them are, read first.
type MemberEntity = (VarId, NameRef);

/// The union's name for the line of member edges `line`, each edge
/// named for its member ([`entity_name`]) and the line as the pair
/// boolean names its lines ([`line_name_of`]).
fn line_name<'a>(union: RecipeNodeId, line: impl IntoIterator<Item = &'a MemberEntity>) -> NameRef {
    line_name_of(
        union,
        line.into_iter()
            .map(|e| edge_line(&NameRef::new(entity_name(union, e)))),
    )
}

/// The union's name for member entity `(member, of)`, sharing the
/// member's handle for `of` so its order cache comes along.
fn entity_name(union: RecipeNodeId, (member, of): &MemberEntity) -> StableName {
    keyed(union, *member, of.clone(), of.kind)
}

/// **An entity of the finished body that belongs to several members
/// at once is named for the set of them.**
///
/// Where two members run flush, one stretch of the finished body's
/// boundary is a stretch of both: a member edge lying along another
/// member's edge, a member's corner sitting on another's corner.
/// The fold keeps whichever operand was A at the step that met them, so
/// its name for the stretch says which member was folded first. The
/// union has no A and B, and no member is chosen over another: the
/// stretch is `Merged` of every member entity that holds it, flat and
/// in name order. A stretch the output stage joined across several
/// member edges lies within none of them, and is named for the set of
/// those it runs along.
///
/// "Holds" is read off the finished body and the members' own bodies,
/// never off the fold:
/// - a finished EDGE lies within member `n`'s edge `e` when its two
///   faces descend from `e`'s two faces in `n` (their names cite them,
///   through a `Merged` set or a `Fragment`) and it lies within `e`
///   along `e`'s carrier: a segment of its line where `e` is straight,
///   a span of its certified curve where `e` is curved
///   ([`joined_along`], [`ON_MEMBER_EDGE`]); it runs ALONG `e` when
///   instead the two overlap over a length of that carrier;
/// - a finished VERTEX sits at member `n`'s vertex `w` when it is at
///   `w`'s point ([`ON_MEMBER_EDGE`]) and a face around it descends
///   from a face of `n` that `w` lies on.
///
/// The face test is what keeps two shells apart that only touch: a
/// shell's entity lies on the other member's edge or corner, but none
/// of its faces descends from that member.
struct Flush<'a, T: geom_core::Decide> {
    union: RecipeNodeId,
    body: &'a topo::Body<T>,
    members: &'a [Member<'a, T>],
    inc: Incidence,
    /// Finished face → the member faces it descends from, each by its
    /// member's index: a tied name's candidates are separate parents
    /// spelled alike, so a member face is read by its key and never
    /// looked up by name.
    face_ents: BTreeMap<topo::FaceKey, BTreeSet<MemberFace>>,
    /// Finished edge → the member edges it lies within.
    edges: BTreeMap<topo::EdgeKey, BTreeSet<MemberEntity>>,
    /// Finished edge that lies within none → the several member edges
    /// it lies along, which together cover it: the set it is named for.
    spans: BTreeMap<topo::EdgeKey, BTreeSet<MemberEntity>>,
    /// Finished face → its name.
    face_names: BTreeMap<topo::FaceKey, StableName>,
    bnd: geom_core::Band,
}

impl<'a, T: geom_core::Decide> Flush<'a, T> {
    /// The finished body's faces as the collapsed table `t` names them,
    /// and every finished edge's member edges.
    fn of(
        union: RecipeNodeId,
        body: &'a topo::Body<T>,
        members: &'a [Member<'a, T>],
        parents: &Parents,
        bnd: geom_core::Band,
    ) -> Result<Self, NamingError> {
        let mut face_ents = BTreeMap::new();
        let mut face_names = BTreeMap::new();
        for &f in parents.of_face.keys() {
            let parent = parents.face(f);
            face_ents.insert(f, parent.entities.clone());
            face_names.insert(f, parent.name.clone());
        }
        let mut flush = Flush {
            union,
            body,
            members,
            inc: Incidence::of(body),
            face_ents,
            edges: BTreeMap::new(),
            spans: BTreeMap::new(),
            face_names,
            bnd,
        };
        let mut edges = BTreeMap::new();
        let mut spans = BTreeMap::new();
        for (&k, fs) in &flush.inc.edge_faces {
            let cover = flush.edge_cover(k, fs)?;
            if !cover.within.is_empty() {
                edges.insert(k, cover.within.into_iter().collect());
            } else if cover.covered {
                spans.insert(k, cover.along.into_iter().collect());
            }
        }
        flush.edges = edges;
        flush.spans = spans;
        Ok(flush)
    }

    #[track_caller]
    fn member(&self, read: VarId, name: &StableName) -> &'a Member<'a, T> {
        member_of(self.members, read, name)
    }

    /// How finished edge `k`, between faces `fs`, lies along the member
    /// edges between two faces of one member that its faces descend
    /// from ([`joined_candidates`], [`joined_along`]), each by its
    /// member's name for it, a tied one included.
    ///
    /// # Errors
    ///
    /// As [`EdgeCarrier::of_edge`], [`joined_candidates`] and
    /// [`joined_along`].
    fn edge_cover(
        &self,
        k: topo::EdgeKey,
        fs: &[topo::FaceKey],
    ) -> Result<Cover<MemberEntity>, NamingError> {
        let none = Cover {
            within: Vec::new(),
            along: Vec::new(),
            covered: false,
        };
        let [f0, f1] = fs else { return Ok(none) };
        let (Some(from0), Some(from1)) = (self.face_ents.get(f0), self.face_ents.get(f1)) else {
            return Ok(none);
        };
        // An edge on a carrier that places no point covers nothing, and
        // the fold's name for it stands
        // (`work/splice/a-unions-flush-reading-cannot-follow-a-nurbs-carrier.md`).
        if let EdgeCarrier::Unread = EdgeCarrier::of_edge(self.body, k)? {
            return Ok(none);
        }
        let (d0, d1): (Vec<MemberFace>, Vec<MemberFace>) = (
            from0.iter().copied().collect(),
            from1.iter().copied().collect(),
        );
        let candidates = joined_candidates(
            &d0,
            &d1,
            |i| Ok(self.member_by_index(i).body),
            is_straight(self.body, k),
        )?;
        let mut named = Vec::with_capacity(candidates.len());
        for ((i, r), arc) in candidates {
            let m = self.member_by_index(i);
            let name = m
                .table
                .name_ref_of(&ent(0, EntityKey::Edge(r)))
                .unwrap_or_else(|| {
                    unreachable!(
                        "member {:?}'s edge {r:?}, which finished edge {k:?} may lie along, is \
                         unnamed in its table: a member's table is total over its body \
                         (`check_total`)",
                        m.read
                    )
                });
            named.push(((m.read, name.clone()), arc));
        }
        joined_along(self.body, k, named, self.bnd)
    }

    /// Member `i` of `members`, by the index a member entity's key is
    /// read under.
    #[track_caller]
    fn member_by_index(&self, i: usize) -> &'a Member<'a, T> {
        let Some(member) = self.members.get(i) else {
            unreachable!(
                "a union reads a member entity under index {i} of {} members: every index is a \
                 member's place in the fold (`Fold`, `Links::judged`)",
                self.members.len()
            )
        };
        member
    }

    /// The member edges finished edge `k` lies on at its vertex `v`:
    /// those it lies within, or those of the set it is named for that
    /// hold `v` ([`edges_holding`]), in name order.
    ///
    /// # Errors
    ///
    /// [`NamingError::MemberEdgeTied`] where a tied member edge of the
    /// set has a candidate holding `v`: the vertex lies on it, and its
    /// name stands for several edges. A tied edge none of whose
    /// candidates holds `v` does not bear on the vertex.
    fn lines_at(
        &self,
        v: topo::VertexKey,
        k: topo::EdgeKey,
    ) -> Result<Vec<MemberEntity>, NamingError> {
        if let Some(w) = self.edges.get(&k) {
            return Ok(w.iter().cloned().collect());
        }
        let Some(set) = self.spans.get(&k) else {
            return Ok(Vec::new());
        };
        let p = vertex_point(self.body, v);
        let mut on = Vec::with_capacity(set.len());
        for line in set {
            let read = line.0;
            let m = self.member(read, line.1.name());
            let entry = m.table.lookup(line.1.name()).unwrap_or_else(|| {
                unreachable!(
                    "member edge {:?}, which finished edge {k:?} lies along, is not in member \
                     {read:?}'s table: `Flush::edge_cover` read it there by key",
                    line.1.name()
                )
            });
            let mut candidates = Vec::with_capacity(entry.entities().len());
            for e in entry.entities() {
                let EntityKey::Edge(r) = e.key else {
                    unreachable!(
                        "member edge {:?} names {:?}: a table seats a row only on entities of its \
                         name's kind",
                        line.1.name(),
                        e.key
                    );
                };
                candidates.push((r, m.body, r));
            }
            match entry {
                Entry::Unique(_) => {
                    on.extend(candidates.into_iter().map(|(_, b, r)| (line.clone(), b, r)))
                }
                Entry::Tied(_) => {
                    if !edges_holding(p, candidates, self.bnd)?.is_empty() {
                        return Err(NamingError::MemberEdgeTied {
                            member: line.0,
                            edge: Box::new(line.1.name().clone()),
                        });
                    }
                }
            }
        }
        edges_holding(p, on, self.bnd)
    }

    /// What a row the fold named a piece of `own`, finished edge `k`, is
    /// a piece of: every member edge `k` lies within, whatever the fold
    /// named it, and `own` where it lies within none.
    fn within_or(&self, k: topo::EdgeKey, own: MemberEntity) -> BTreeSet<MemberEntity> {
        self.within(k)
            .cloned()
            .unwrap_or_else(|| BTreeSet::from([own]))
    }

    /// What finished edge `k`, lying within member edges `within`, is a
    /// piece of, and whether it is the whole of it: one member edge, or
    /// the set of several, which a lone edge holds whole.
    fn piece_of(
        &self,
        k: topo::EdgeKey,
        within: BTreeSet<MemberEntity>,
    ) -> Result<(Option<Along>, bool), NamingError> {
        let mut within = within.into_iter();
        Ok(match (within.next(), within.next()) {
            (Some(one), None) => {
                let whole = self.holds_whole(k, &one)?;
                (Some(Along::One(one)), whole)
            }
            (Some(a), Some(b)) => {
                let set = [a, b].into_iter().chain(within).collect();
                (Some(Along::Set(set)), true)
            }
            (None, _) => (None, false),
        })
    }

    /// Whether finished edge `k`, a piece of member edge `edge`, is the
    /// whole of it: its two ends at the member edge's two ends, read off
    /// the points through [`ON_MEMBER_EDGE`], one end at each (a closed
    /// edge's one end twice).
    fn holds_whole(&self, k: topo::EdgeKey, edge: &MemberEntity) -> Result<bool, NamingError> {
        let (member_body, member_edge) = member_edge(self.members, edge.0, edge.1.name())?;
        let (m0, m1) = edge_ends(member_body, member_edge);
        let (c0, c1) = edge_ends(self.body, k);
        let ends = [vertex_point(member_body, m0), vertex_point(member_body, m1)];
        let at = |c| -> Result<Vec<usize>, NamingError> {
            let p = vertex_point(self.body, c);
            let mut at = Vec::with_capacity(2);
            for (i, q) in ends.iter().enumerate() {
                let off = decide(ON_MEMBER_EDGE, Margin::of((p - *q).norm()), self.bnd).map_err(
                    |source| NamingError::Escalated {
                        predicate: ON_MEMBER_EDGE,
                        source,
                    },
                )?;
                if off == Sign::Zero {
                    at.push(i);
                }
            }
            Ok(at)
        };
        let (a0, a1) = (at(c0)?, at(c1)?);
        Ok(if m0 == m1 {
            c0 == c1 && !a0.is_empty()
        } else {
            (a0.contains(&0) && a1.contains(&1)) || (a0.contains(&1) && a1.contains(&0))
        })
    }

    /// The member edges finished edge `k` lies within, if any.
    fn within(&self, k: topo::EdgeKey) -> Option<&BTreeSet<MemberEntity>> {
        self.edges.get(&k).filter(|w| !w.is_empty())
    }

    /// **Finished vertex `v`, read from the finished body by the lines
    /// through it** (N2), where it is no member vertex.
    ///
    /// Its lines are the member edges the finished edges at `v` lie on,
    /// those that run flush on one line once as the set of them
    /// ([`sets_per_line`], [`Flush::line_sense`]), and each seam
    /// at `v` whose two faces descend from no face of those member edges,
    /// cited by its group's base ([`EdgeLines`]). One member edge and no
    /// seam is the `Crossing` of that edge and the one face at `v` that
    /// descends from none of its faces, cited by its parent; two or more
    /// lines are their `EdgeCrossing` run. Each line carries its sense
    /// against the members of the name's other entries: a line of member
    /// edges from the pairwise judgements ([`Flush::line_sense`]), a seam
    /// from which of its pieces the result keeps at `v`
    /// ([`Flush::seam_sense`]).
    ///
    /// `None` for a point on one seam line alone, whose seam's own vertex
    /// name stands.
    ///
    /// # Errors
    ///
    /// [`NamingError::BandVerdictsDisagree`] where a finished edge at `v`
    /// lies on none of the member edges it is named for;
    /// [`NamingError::UnsupportedSeamVertex`] where one member edge and no
    /// seam meet several faces; [`NamingError::Emission`] where they meet
    /// none; or as [`Flush::sense_against`].
    fn meeting(
        &self,
        v: topo::VertexKey,
        lines: &EdgeLines<'_>,
        links: &Links<T>,
    ) -> Result<Option<StableName>, NamingError> {
        let bug = |what| NamingError::Emission { what };
        let at = self.inc.vertex_edges.get(&v).map_or(&[][..], Vec::as_slice);
        let mut along = BTreeSet::new();
        let mut seam_edges = Vec::new();
        for &k in at {
            if let Some(line) = lines.member.get(&k) {
                let on = self.line_at(v, k, line)?;
                // The cover and an end point are two band decisions, and a
                // span set of tied edges leaves `lines_at` empty.
                if on.is_empty() {
                    return Err(NamingError::BandVerdictsDisagree {
                        vertex: v,
                        shape: DisagreeShape::JoinedEdgeOffMembers,
                    });
                }
                along.extend(on);
            } else if let Some(&base) = lines.seam.get(&k) {
                seam_edges.push((k, base));
            } else {
                unreachable!(
                    "union vertex {v:?} lies on finished edge {k:?}, which is neither a member \
                     edge's piece nor a seam: every edge of a union's body is a piece of member \
                     edges or lies between two members' faces (`group_member_edges`, \
                     `name_by_parents`)"
                );
            }
        }
        // Member edges that run flush on one line are that line once,
        // as the set of them.
        let cited = sets_per_line(
            along.iter().cloned(),
            |e| {
                let (body, edge) = member_edge(self.members, e.0, e.1.name())?;
                EdgeCarrier::of_edge(body, edge).map(Some)
            },
            self.bnd,
        )?;
        let sides = self.sides(&along)?;
        let bounds = |f: &topo::FaceKey| {
            self.face_ents
                .get(f)
                .is_some_and(|from| !from.is_disjoint(&sides))
        };
        // Each seam's members, and its pieces at `v`.
        let mut seams: BTreeMap<&StableName, (BTreeSet<VarId>, Vec<topo::EdgeKey>)> =
            BTreeMap::new();
        for (k, base) in seam_edges {
            let fs = self.inc.edge_faces.get(&k).map_or(&[][..], Vec::as_slice);
            if fs.iter().any(bounds) {
                continue;
            }
            let (members, pieces) = seams.entry(base).or_default();
            members.extend(fs.iter().flat_map(|f| self.members_of(f)));
            pieces.push(k);
        }
        if cited.is_empty() && seams.len() < 2 {
            return Ok(None);
        }
        if let ([line], true) = (cited.as_slice(), seams.is_empty()) {
            let mut faces = BTreeMap::new();
            for k in at {
                for f in self.inc.edge_faces.get(k).into_iter().flatten() {
                    if bounds(f) {
                        continue;
                    }
                    let name = self.face_names.get(f).unwrap_or_else(|| {
                        unreachable!(
                            "face {f:?} at union vertex {v:?} has no parent name: `Flush::of` \
                             names every face of the finished body for its parent"
                        )
                    });
                    faces
                        .entry(name)
                        .or_insert_with(BTreeSet::new)
                        .extend(self.members_of(f));
                }
            }
            let mut faces = faces.into_iter();
            let (face, others) = match (faces.next(), faces.next()) {
                (Some(one), None) => one,
                // One line meeting several faces of other members and no
                // seam. Faces of other members meet at a point only along
                // their edges through it, and with planar faces the region
                // kept about the line either keeps a piece of such an edge
                // at the point, holds one of the faces there, or pinches
                // into two vertices, each one line meeting one face, so no
                // manifold union of planar faces reaches this shape. Curved
                // faces are outside that argument, so no rule names this
                // vertex.
                (Some(_), Some(_)) => {
                    return Err(NamingError::UnsupportedSeamVertex {
                        vertex: v,
                        shape: SeamVertexShape::LineAcrossSeveralFaces,
                    });
                }
                (None, _) => {
                    return Err(bug(
                        "one member edge meets no face at a union vertex no seam reaches",
                    ));
                }
            };
            let sense = self.line_sense(v, line, &others, links)?;
            return Ok(Some(StableName {
                kind: EntityKind::Vertex,
                node: self.union,
                path: vec![RoleSeg::Crossing {
                    edge: line_name(self.union, line),
                    face: NameRef::new(face.clone()),
                    sense,
                }],
            }));
        }
        // Every member an entry belongs to; each edge is read against
        // those other than its own.
        let entries: BTreeSet<VarId> = cited
            .iter()
            .flatten()
            .map(|l| l.0)
            .chain(seams.values().flat_map(|(members, _)| members).copied())
            .collect();
        let mut run = Vec::with_capacity(cited.len() + seams.len());
        for line in &cited {
            let mut others = entries.clone();
            for (member, _) in line {
                others.remove(member);
            }
            run.push(RunLine {
                line: line_name(self.union, line),
                sense: self.line_sense(v, line, &others, links)?,
            });
        }
        for (base, (_, pieces)) in seams {
            run.push(RunLine {
                line: edge_line(&NameRef::new(base.clone())),
                sense: self.seam_sense(v, base, &pieces)?,
            });
        }
        run.sort();
        Ok(Some(StableName {
            kind: EntityKind::Vertex,
            node: self.union,
            path: vec![RoleSeg::EdgeCrossing(run)],
        }))
    }

    /// **The sense at finished vertex `v` of union seam `base`, read by
    /// which of its `pieces` at `v` the result keeps** (N2): the union
    /// keeps a stretch of a seam only outside every other member, and a
    /// seam in a run has faces that bound none of the run's member
    /// edges, so where no piece goes on past `v` its side faces were
    /// trimmed there by another member in the run, the seam's continuation
    /// lying in that member's closed body. Read along side `a`'s loop: a
    /// piece arriving at `v` and none leaving enters along `a`, one
    /// leaving and none arriving enters along `b`, and pieces on both
    /// sides read `Outside`. A pair whose two sides carry one name is
    /// read alike either way.
    ///
    /// # Errors
    ///
    /// [`NamingError::Emission`] where the faces beside a piece match the
    /// seam's sides neither way, or both ([`UNSIDED`]).
    fn seam_sense(
        &self,
        v: topo::VertexKey,
        base: &StableName,
        pieces: &[topo::EdgeKey],
    ) -> Result<Sense, NamingError> {
        let Some((a, b)) = seam_pair::seam_edge_sides(base) else {
            unreachable!(
                "union seam group {base:?} at vertex {v:?} is not a seam edge: `EdgeLines` keys \
                 each seam piece by its group's `Seam` base (`name_by_parents`)"
            )
        };
        // Whether a piece lies on each side of `v` along `a`'s loop:
        // before it (arriving) and after it (leaving).
        let (mut before, mut after) = (false, false);
        for &k in pieces {
            let (v0, v1) = edge_ends(self.body, k);
            let with_a = if a == b {
                true
            } else {
                let (plus, minus) = topo::readback::edge_sides(self.body, k)
                    .unwrap_or_else(|e| {
                        unreachable!(
                            "finished seam edge {k:?} has no two sides ({e:?}): the finished body \
                             is one every door keeps tier-1-valid"
                        )
                    })
                    .faces();
                let parent = |f: topo::FaceKey| {
                    self.face_names.get(&f).unwrap_or_else(|| {
                        unreachable!(
                            "face {f:?} beside finished seam edge {k:?} has no parent name: \
                             `Flush::of` names every face of the finished body for its parent"
                        )
                    })
                };
                match seam_pair::a_side_is_first(parent(plus), parent(minus), a, b) {
                    Some(with_a) => with_a,
                    None => return Err(NamingError::Emission { what: UNSIDED }),
                }
            };
            let (start, end) = if with_a { (v0, v1) } else { (v1, v0) };
            after |= start == v;
            before |= end == v;
        }
        let side = |s: &StableName| Sense::Through {
            entering: vec![NameRef::new(s.clone())],
        };
        Ok(match (before, after) {
            (true, true) => Sense::Outside,
            (true, false) => side(a),
            (false, true) => side(b),
            // `Flush::meeting` reads a seam at `v` off the finished
            // edges at `v`, each with `v` for an end.
            (false, false) => unreachable!(
                "union seam {base:?} has no piece ending at vertex {v:?}: `Flush::meeting` \
                 lists only seam pieces at it"
            ),
        })
    }

    /// The member edges finished edge `k`, named for `line`, lies on at
    /// its vertex `v`: those it lies within, its own member
    /// edge among them, or those of the set it is named for whose closed
    /// segment holds `v` ([`Flush::lines_at`]).
    fn line_at(
        &self,
        v: topo::VertexKey,
        k: topo::EdgeKey,
        line: &Along,
    ) -> Result<Vec<MemberEntity>, NamingError> {
        Ok(match line {
            Along::One(e) => {
                let mut on: BTreeSet<MemberEntity> =
                    self.edges.get(&k).cloned().unwrap_or_default();
                on.insert(e.clone());
                on.into_iter().collect()
            }
            Along::Set(_) => self.lines_at(v, k)?,
        })
    }

    /// **The sense at finished vertex `v` of the line of member edges
    /// `line`** against the closed bodies of members `others`: each
    /// edge's own ([`Flush::sense_against`]), composed over the line
    /// ([`Flush::compose`]).
    ///
    /// # Errors
    ///
    /// As [`Flush::sense_against`] and [`Flush::compose`].
    fn line_sense<'e>(
        &self,
        v: topo::VertexKey,
        line: impl IntoIterator<Item = &'e MemberEntity>,
        others: &BTreeSet<VarId>,
        links: &Links<T>,
    ) -> Result<Sense, NamingError> {
        let mut read = Vec::new();
        for e in line {
            let (stored, reading) = self.sense_against(v, e, others, links)?;
            read.push((e, stored, reading));
        }
        self.compose(v, read)
    }

    /// The faces of member edges `edges`, each in its member.
    ///
    /// # Errors
    ///
    /// As [`member_edge`]: a tied member edge is
    /// [`NamingError::MemberEdgeTied`], since its candidates' faces are
    /// not one pair.
    fn sides(&self, edges: &BTreeSet<MemberEntity>) -> Result<BTreeSet<MemberFace>, NamingError> {
        let mut sides = BTreeSet::new();
        for (n, name) in edges {
            let (i, r) = member_edge_at(self.members, *n, name.name())?;
            let (plus, minus) = topo::readback::edge_sides(self.members[i].body, r)
                .unwrap_or_else(|e| {
                    unreachable!(
                        "member {n:?}'s edge {r:?} has no two sides ({e:?}): a member's body is \
                         one every door keeps tier-1-valid"
                    )
                })
                .faces();
            sides.extend([(i, plus), (i, minus)]);
        }
        Ok(sides)
    }

    /// The members finished face `f` descends from a face of, by read.
    fn members_of(&self, f: &topo::FaceKey) -> BTreeSet<VarId> {
        self.face_ents
            .get(f)
            .into_iter()
            .flatten()
            .map(|&(i, _)| self.members[i].read)
            .collect()
    }

    /// **The sense at finished vertex `v` of member edge `line`, against
    /// the closed bodies of members `others`** (N2), along the edge as
    /// its member stores it, with how a crossing of it reads
    /// ([`edge_reading`]): each side of `v` along the edge lies in them
    /// where any pairwise judgement of the edge's member with one of them
    /// classed a piece of the edge there in the other's closed body
    /// ([`Links`]).
    ///
    /// # Errors
    ///
    /// [`NamingError::PartnerUnread`] where no such judgement classed a
    /// side because one read no partner of the member's vertex at `v`
    /// ([`topo::UnreadPartner`]); [`NamingError::BandVerdictsDisagree`]
    /// where two pieces of the line are classified at `v`;
    /// [`NamingError::Emission`] where no judgement classed a side
    /// otherwise.
    fn sense_against(
        &self,
        v: topo::VertexKey,
        line: &MemberEntity,
        others: &BTreeSet<VarId>,
        links: &Links<T>,
    ) -> Result<(Sense, EdgeReading), NamingError> {
        let bug = |what| NamingError::Emission { what };
        let (n, name) = line;
        // The member by its place in the list: the members of a family
        // read whole share one read and one body's keys, and the name
        // tells them apart.
        let (at_member, m) = member_at(self.members, *n, name.name());
        // The member's rows on the line, each with its edge, a tied
        // row with each candidate's: the classified piece is the one at
        // `v`. The line itself need not be a row.
        let mut rows: BTreeMap<topo::EdgeKey, &StableName> = BTreeMap::new();
        let head = m.table.lookup(name.name()).map(|e| (name.name(), e));
        let mut on_line = Vec::new();
        for row in m.table.on_line(name.name()) {
            let entry = m.table.lookup(row).unwrap_or_else(|| {
                unreachable!("{}", not_in_its_table(*n, name.name(), row.name()))
            });
            on_line.push((row.name(), entry));
        }
        for (row, entry) in head.into_iter().chain(on_line) {
            for e in entry.entities() {
                let EntityKey::Edge(k) = e.key else {
                    unreachable!("{}", names_no_edge(*n, row, e.key));
                };
                rows.entry(k).or_insert(row);
            }
        }
        let p = vertex_point(self.body, v);
        let read = |i: usize| self.members.get(i).map(|m| m.read);
        // Whether a judgement of the edge's member against one the name
        // cites recorded something at the point `at` that is `v`'s.
        let here = |member: usize, other: usize, at: geom_core::Point3<T>| {
            // Another member of the line's own family is one the name
            // cites: its read is the line's, which the name cites once.
            let cited = |o: VarId| others.contains(&o) || (o == *n && other != at_member);
            if member != at_member || !read(other).is_some_and(cited) {
                return Ok(false);
            }
            let gap = Margin::of((at - p).norm());
            decide(ON_MEMBER_EDGE, gap, self.bnd)
                .map(|off| off == Sign::Zero)
                .map_err(|source| NamingError::Escalated {
                    predicate: ON_MEMBER_EDGE,
                    source,
                })
        };
        let mut sides: [Option<bool>; 2] = [None, None];
        let mut piece = BTreeSet::new();
        for c in &links.classes {
            if !rows.contains_key(&c.edge) || !here(c.member, c.other, c.at)? {
                continue;
            }
            piece.insert(c.edge);
            let side = &mut sides[usize::from(c.starts)];
            *side = Some(side.unwrap_or(false) | c.inside);
        }
        // A judgement that read no partner at `v` classed none of the
        // member's edges there, by its own account: the crossing refuses
        // naming why, so no listed vertex reaches the missing-row bug.
        let unread = || -> Result<Option<NamingError>, NamingError> {
            for u in &links.unread {
                if here(u.member, u.other, u.at)? {
                    return Ok(Some(NamingError::PartnerUnread {
                        vertex: v,
                        partner: u.record.partner,
                        unread: u.record,
                    }));
                }
            }
            Ok(None)
        };
        let unclassified = || match unread() {
            Ok(Some(e)) | Err(e) => e,
            Ok(None) => bug(UNCLASSIFIED_SIDE),
        };
        let mut piece = piece.into_iter();
        let k = match (piece.next(), piece.next()) {
            (Some(k), None) => k,
            (None, _) => return Err(unclassified()),
            // A piece shorter than the band puts both its neighbours'
            // classes at one point.
            (Some(_), Some(_)) => {
                return Err(NamingError::BandVerdictsDisagree {
                    vertex: v,
                    shape: DisagreeShape::PiecesAtOneVertex,
                });
            }
        };
        let [Some(before), Some(after)] = sides else {
            return Err(unclassified());
        };
        let sense = match (before, after) {
            (false, true) => Sense::Enters,
            (true, false) => Sense::Leaves,
            (false, false) => Sense::Outside,
            (true, true) => Sense::Inside,
        };
        Ok((sense, edge_reading(m.body, m.table, k, rows[&k])))
    }

    /// How a crossing of member edge `e` reads its sense
    /// ([`edge_reading`]), on the edge its member's table names it.
    ///
    /// # Errors
    ///
    /// As [`member_edge`].
    fn reading_of(&self, e: &MemberEntity) -> Result<EdgeReading, NamingError> {
        let (body, k) = member_edge(self.members, e.0, e.1.name())?;
        let m = self.member(e.0, e.1.name());
        Ok(edge_reading(body, m.table, k, e.1.name()))
    }

    /// The union's name for side `side` of a seam edge of member `n`, as
    /// the member's own seam name writes it: the side carried in through
    /// the member's read, as the union cites any member entity.
    fn member_side(&self, n: VarId, side: &StableName) -> NameRef {
        NameRef::new(keyed(
            self.union,
            n,
            NameRef::new(side.clone()),
            EntityKind::Face,
        ))
    }

    /// **The sense at finished vertex `v` of a line of member edges,
    /// each with its sense along it as its member stores it and how a
    /// crossing of it reads** (N2), as [`sense_along_line`] reads it.
    ///
    /// # Errors
    ///
    /// As [`EdgeReading::read`] and [`sense_along_line`]: there the
    /// senses are independent pairwise judgements of edges flush only
    /// within `ON_MEMBER_EDGE`.
    fn compose(
        &self,
        v: topo::VertexKey,
        read: Vec<(&MemberEntity, Sense, EdgeReading)>,
    ) -> Result<Sense, NamingError> {
        let mut senses = Vec::with_capacity(read.len());
        for (e, stored, reading) in read {
            senses.push((
                edge_line(&NameRef::new(entity_name(self.union, e))),
                reading.read(stored, |s| self.member_side(e.0, s))?,
            ));
        }
        sense_along_line(v, senses)
    }

    /// Whether member edge `of` runs the same way as member edge `like`:
    /// `Some(true)` for the same edge, and for an edge lying on `like`'s
    /// line, straight or curved, the way their directions agree
    /// ([`EdgeCarrier::runs_with`]); `None` where `of` does not lie on
    /// that line.
    fn same_way(
        &self,
        of: &MemberEntity,
        like: &MemberEntity,
    ) -> Result<Option<bool>, NamingError> {
        if of == like {
            return Ok(Some(true));
        }
        let (of_body, of_edge) = member_edge(self.members, of.0, of.1.name())?;
        let (like_body, like_edge) = member_edge(self.members, like.0, like.1.name())?;
        EdgeCarrier::of_edge(like_body, like_edge)?.runs_with(
            &EdgeCarrier::of_edge(of_body, of_edge)?,
            ON_MEMBER_EDGE,
            self.bnd,
        )
    }

    /// `now`, the rewrite of vertex name `was` at finished vertex `at`
    /// with every member edge it cites whole ([`WholeMemberEdges`]), with
    /// each sense it carries re-read along the line it now cites. The
    /// lines are matched by image, not by position: the rewrite puts an
    /// `EdgeCrossing`'s run in name order, each sense with its line. A
    /// sense read along a member edge is read back to that edge as its
    /// member stores it, carried to each member edge of the line the
    /// rewrite cites along the way it runs there ([`Flush::same_way`]),
    /// and composed over that line ([`Flush::compose`]). A line that
    /// cites no member edge before the rewrite is a joined edge
    /// (`Merged`) or a name the union does not own, which re-citing its
    /// constituents does not move, so its sense stands.
    ///
    /// # Errors
    ///
    /// As [`Flush::cited_whole`], [`Flush::same_way`] and
    /// [`Flush::compose`]; [`NamingError::Emission`] where the sense
    /// crosses a seam edge whose side it enters along the names do not
    /// settle ([`UNSIDED`]).
    fn resensed(
        &self,
        was: &StableName,
        at: Option<topo::VertexKey>,
        mut now: StableName,
    ) -> Result<StableName, NamingError> {
        let mut pairs: Vec<(&StableName, (&NameRef, &mut Sense))> = Vec::new();
        match (was.path.first(), now.path.first_mut()) {
            (
                Some(RoleSeg::Crossing { edge: old, .. }),
                Some(RoleSeg::Crossing { edge, sense, .. }),
            ) => pairs.push((old, (&*edge, sense))),
            (Some(RoleSeg::EdgeCrossing(old)), Some(RoleSeg::EdgeCrossing(run))) => {
                let mut images = Vec::with_capacity(old.len());
                for l in old {
                    images.push((&*l.line, self.cited_whole(was.node, at, &l.line)?));
                }
                let distinct: BTreeSet<&StableName> = images.iter().map(|(_, i)| i).collect();
                if distinct.len() != images.len() {
                    unreachable!(
                        "union vertex name {was:?} cites one member edge on two of its lines \
                         once re-cited whole ({images:?}): the lines of a run are distinct \
                         lines through one vertex, and pieces of distinct member edges are \
                         re-cited as distinct lines"
                    );
                }
                for l in run.iter_mut() {
                    let (line, sense) = (&l.line, &mut l.sense);
                    let old = images
                        .iter()
                        .find(|(_, image)| *image == **line)
                        .map(|(old, _)| *old)
                        .unwrap_or_else(|| {
                            unreachable!(
                                "the rewrite of union vertex name {was:?} cites line {line:?}, \
                                 the image of none of its lines ({images:?}): the rewrite maps \
                                 each line through `Flush::cited_whole`, as these images do"
                            )
                        });
                    pairs.push((old, (line, sense)));
                }
            }
            _ => {}
        }
        for (old, (edge, sense)) in pairs {
            let Some((m0, e0, _)) = member_edge_piece(old) else {
                if member_edge_piece(edge).is_some() {
                    unreachable!("{}", recited_off_line(was, old, edge));
                }
                continue;
            };
            let image = self.member_edges_of(edge).unwrap_or_else(|| {
                unreachable!("{}", recited_off_line(was, old, edge));
            });
            let own = (m0, e0);
            if image == [own.clone()] {
                continue;
            }
            // The sense along `own` as its member stores it.
            let stored = match (self.reading_of(&own)?, sense.clone()) {
                (EdgeReading::Stored, stored) => stored,
                (_, touch @ (Sense::Outside | Sense::Inside)) => touch,
                (EdgeReading::Sides { with, against }, Sense::Through { entering })
                    if with != against =>
                {
                    if entering == [self.member_side(own.0, &with)] {
                        Sense::Enters
                    } else if entering == [self.member_side(own.0, &against)] {
                        Sense::Leaves
                    } else {
                        return Err(NamingError::Emission {
                            what: RESENSED_OFF_SIDES,
                        });
                    }
                }
                _ => return Err(NamingError::Emission { what: UNSIDED }),
            };
            let mut read = Vec::with_capacity(image.len());
            for e in &image {
                let along = match self.same_way(e, &own)? {
                    Some(true) => stored.clone(),
                    Some(false) => stored.clone().flipped(),
                    None => unreachable!(
                        "union vertex name {was:?} re-cites member edge {own:?} as member edge \
                         {e:?}, off its line: `Flush::line_through` cites only edges on the line \
                         through the vertex"
                    ),
                };
                read.push((e, along, self.reading_of(e)?));
            }
            *sense = match (read.as_slice(), at) {
                ([(e, along, reading)], _) => {
                    reading.read(along.clone(), |s| self.member_side(e.0, s))?
                }
                (_, Some(at)) => self.compose(at, read)?,
                (_, None) => unreachable!(
                    "union vertex name {was:?} re-cites a line of several member edges with no \
                     vertex to read it at: `Flush::cited_whole` cites such a line only through \
                     `Flush::line_through`, which reads one"
                ),
            };
        }
        Ok(now)
    }

    /// The member edges a line name cites: the member edge it names,
    /// or each member edge of `Merged` of them; `None` for any other
    /// name.
    fn member_edges_of(&self, name: &StableName) -> Option<Vec<MemberEntity>> {
        if let Some((m, e, _)) = member_edge_piece(name) {
            return Some(vec![(m, e)]);
        }
        let [RoleSeg::Merged(set)] = name.path.as_slice() else {
            return None;
        };
        set.iter()
            .map(|n| member_edge_piece(n).map(|(m, e, _)| (m, e)))
            .collect()
    }

    /// The image [`WholeMemberEdges`] gives side `side` of a vertex name
    /// of `union` at finished vertex `at`: a member edge cited whole as
    /// the line of member edges through the vertex, a name the union does
    /// not own as it stands, and anything else with its embedded names
    /// rewritten.
    fn cited_whole(
        &self,
        union: RecipeNodeId,
        at: Option<topo::VertexKey>,
        side: &StableName,
    ) -> Result<StableName, NamingError> {
        if side.node != union {
            return Ok(side.clone());
        }
        if let Some((member, edge, _)) = member_edge_piece(side) {
            let line = match at {
                Some(v) => self.line_through(v, (member, edge))?,
                None => BTreeSet::from([(member, edge)]),
            };
            return Ok((*line_name(union, &line)).clone());
        }
        side.clone().rewrite_path(&mut WholeMemberEdges {
            union,
            at,
            flush: self,
        })
    }

    /// The line a name of finished vertex `v` cites, where its name
    /// cites `own`: `own` and every member edge that a finished edge at
    /// `v` lying on `own`'s line there lies on too ([`Flush::lines_at`]),
    /// each read as its line ([`edge_line`]).
    fn line_through(
        &self,
        v: topo::VertexKey,
        own: MemberEntity,
    ) -> Result<BTreeSet<MemberEntity>, NamingError> {
        let line = |(m, e): &MemberEntity| (*m, edge_line(e));
        let own = line(&own);
        let mut through = BTreeSet::from([own.clone()]);
        for k in self.inc.vertex_edges.get(&v).into_iter().flatten() {
            let on: Vec<MemberEntity> = self.lines_at(v, *k)?.iter().map(line).collect();
            if on.contains(&own) {
                through.extend(on);
            }
        }
        Ok(through)
    }

    /// **The name of the member vertices finished vertex `v` sits at**,
    /// if any: each vertex `w` of a member `n` at `v`'s point that lies on
    /// a face of `n` a face around `v` descends from, the one such vertex
    /// or `Merged` of the several; whether a member's table ties one of
    /// them, a tie its citing name descends from (N2); and the vertices,
    /// each by its member's index.
    ///
    /// # Errors
    ///
    /// [`NamingError::Escalated`] where a member vertex's point is
    /// undecided against `v`'s.
    fn member_vertices(&self, v: topo::VertexKey) -> Result<Option<MemberVertices>, NamingError> {
        let mut cited = BTreeSet::new();
        for k in self.inc.vertex_edges.get(&v).into_iter().flatten() {
            for f in self.inc.edge_faces.get(k).into_iter().flatten() {
                cited.extend(self.face_ents.get(f).into_iter().flatten().copied());
            }
        }
        let p = vertex_point(self.body, v);
        let mut seen = BTreeSet::new();
        let mut at_v: BTreeSet<MemberEntity> = BTreeSet::new();
        let mut corners = BTreeSet::new();
        let mut tied = false;
        for (i, f) in cited {
            let m = self.member_by_index(i);
            for he in face_half_edges(m.body, f) {
                let w = m
                    .body
                    .get_half_edge(he)
                    .unwrap_or_else(|| {
                        unreachable!(
                            "member {:?}'s face {f:?} lists half-edge {he:?}, which is dangling: \
                             a member's body is one every door keeps tier-1-valid",
                            m.read
                        )
                    })
                    .start;
                if !seen.insert((i, w)) {
                    continue;
                }
                let gap = Margin::of((vertex_point(m.body, w) - p).norm());
                let at = decide(ON_MEMBER_EDGE, gap, self.bnd).map_err(|source| {
                    NamingError::Escalated {
                        predicate: ON_MEMBER_EDGE,
                        source,
                    }
                })?;
                if at != Sign::Zero {
                    continue;
                }
                let name = m
                    .table
                    .name_ref_of(&ent(0, EntityKey::Vertex(w)))
                    .unwrap_or_else(|| {
                        unreachable!(
                            "member {:?}'s vertex {w:?}, which union vertex {v:?} sits at, is \
                             unnamed in its table: a member's table is total over its body \
                             (`check_total`)",
                            m.read
                        )
                    });
                tied |= m.table.is_tied(name.name());
                at_v.insert((m.read, name.clone()));
                corners.insert((i, w));
            }
        }
        let name = match at_v.iter().collect::<Vec<_>>().as_slice() {
            [] => return Ok(None),
            [w] => entity_name(self.union, w),
            several => crate::names::merged::entity_set(
                EntityKind::Vertex,
                self.union,
                several.iter().map(|w| entity_name(self.union, w)),
            ),
        };
        Ok(Some(MemberVertices {
            name,
            tied,
            corners,
        }))
    }
}

/// What [`Flush::member_vertices`] reads at a finished vertex.
struct MemberVertices {
    name: StableName,
    tied: bool,
    corners: BTreeSet<(usize, topo::VertexKey)>,
}

/// The member faces a face name of the union descends from: its own
/// member face, or every constituent's of a `Merged` set, through any
/// `Fragment` after either.
fn member_faces(name: &StableName, out: &mut BTreeSet<MemberEntity>) {
    let mut names = vec![name];
    while let Some(name) = names.pop() {
        match name.path.first() {
            Some(RoleSeg::From { read, of }) if of.kind == EntityKind::Face => {
                out.insert((*read, of.clone()));
            }
            Some(RoleSeg::Merged(set)) => names.extend(set),
            _ => {}
        }
    }
}

/// The member entity a name of this shape is a piece of — `From(m, x)`,
/// then nothing but `Fragment` qualifiers — and those qualifiers. The
/// one reader of that shape: [`member_edge_piece`] narrows it to an
/// edge's pieces, [`member_cells`] reads a fold row's cell through it.
fn member_piece(name: &StableName) -> Option<(VarId, &NameRef, &[RoleSeg])> {
    let (RoleSeg::From { read, of }, tail) = name.path.split_first()? else {
        return None;
    };
    tail.iter()
        .all(|s| matches!(s, RoleSeg::Fragment(_)))
        .then_some((*read, of, tail))
}

/// The (member, member edge) an EDGE name of this shape is about —
/// `From(m, e)`, `e` an edge, then nothing but `Fragment(Ends)`
/// qualifiers, the only qualifier an edge's piece carries — and whether
/// any qualifier follows.
fn member_edge_piece(name: &StableName) -> Option<(VarId, NameRef, bool)> {
    if name.kind != EntityKind::Edge {
        return None;
    }
    let (member, of, tail) = member_piece(name)?;
    if of.kind != EntityKind::Edge
        || !tail
            .iter()
            .all(|s| matches!(s, RoleSeg::Fragment(Qualifier::Ends(_))))
    {
        return None;
    }
    Some((member, of.clone(), !tail.is_empty()))
}

/// **A vertex is named for what it sits on in the finished body, and a
/// name embedded in another cites a member edge whole.**
///
/// A vertex that sits at a member vertex is that member vertex, and
/// `Merged` of them where several members have one there
/// ([`Flush::member_vertices`]).
/// Any other is named by the lines through it in the finished body,
/// `Crossing { edge, face, sense }` or an `EdgeCrossing` run, each sense
/// read from the members' pairwise judgements ([`Flush::meeting`]),
/// however the fold met it.
///
/// A seam vertex the finished body does not name embeds the member edge it lies on as far as the fold
/// had cut it when it met the vertex — a piece that no published name
/// denotes. It is cited as `From(m, e)`, which is how a vertex
/// already cites an edge the fold had not cut, and as the set of member
/// edges through the vertex along that line ([`Flush::line_through`]), each
/// sense it carries re-read along the edge it now cites
/// ([`Flush::resensed`]). The rewrite is [`StableName::rewrite_path`],
/// which ends in the canonical form (`names::canonical::form`).
///
/// Vertices that share a name tie (N4): nothing ranks them. A vertex
/// at a member corner is grouped apart, keyed by the member corners it
/// sits at across members (a tied name's candidates being separate
/// corners), and named with the other copies of that corner
/// ([`name_corner_copies`]).
#[allow(clippy::too_many_arguments)]
fn cite_member_edges<T: geom_core::Decide>(
    t: NameTable,
    vertex_rows: Vec<(StableName, Entry)>,
    body: &topo::Body<T>,
    (flush, fold): (&Flush<'_, T>, &Fold<'_>),
    (lines, links): (&EdgeLines<'_>, &Links<T>),
) -> Result<NameTable, NamingError> {
    let mut out = NameTable::new();
    let mut tie = TieRows::default();
    // The member corners a finished vertex sits at → its name, whether
    // a member ties it, and the finished vertices sitting there.
    let mut corners: BTreeMap<BTreeSet<(usize, topo::VertexKey)>, CornerCopies> = BTreeMap::new();
    // Each vertex name → the rows it names.
    let mut vertices: BTreeMap<StableName, Vec<Entry>> = BTreeMap::new();
    let rows = t
        .iter()
        .map(|(name, entry)| (name.clone(), entry.clone()))
        .chain(vertex_rows);
    for (name, entry) in rows {
        // The vertex a row names, when it names one vertex, and its
        // row's entity.
        let one = match &entry {
            Entry::Unique(e) => match e.key {
                EntityKey::Vertex(v) => Some((v, *e)),
                _ => None,
            },
            Entry::Tied(_) => None,
        };
        let at = one.map(|(v, _)| v);
        let member_vertex = match one {
            Some((v, e)) => flush.member_vertices(v)?.map(|m| (m, v, e)),
            None => None,
        };
        let crossing = match (&member_vertex, at) {
            (None, Some(v)) => flush.meeting(v, lines, links)?,
            _ => None,
        };
        let cited = match (member_vertex, crossing) {
            (Some((m, v, e)), _) => {
                let held = corners.entry(m.corners).or_insert_with(|| CornerCopies {
                    name: m.name,
                    tied: m.tied,
                    copies: Vec::new(),
                });
                held.copies.push((v, e));
                continue;
            }
            (None, Some(crossing)) => crossing,
            (None, None) => {
                let whole = name.clone().rewrite_path(&mut WholeMemberEdges {
                    union: name.node,
                    at,
                    flush,
                })?;
                flush.resensed(&name, at, whole)?
            }
        };
        if cited.kind != EntityKind::Vertex {
            put_entry(&mut out, cited, &entry)?;
            continue;
        }
        vertices.entry(cited).or_default().push(entry);
    }
    // Vertices that share a name tie (N4).
    for (name, entries) in vertices {
        if let [entry] = entries.as_slice() {
            put_entry(&mut out, name, entry)?;
            continue;
        }
        let ents = entries.iter().flat_map(Entry::entities).copied().collect();
        mint_candidates(&mut out, &mut tie, false, name, ents)?;
    }
    name_corner_copies(&mut out, &mut tie, corners, |at, keys| {
        member_corner_walls(at, keys, body, (flush, fold))
    })?;
    tie.flush(&mut out)?;
    Ok(out)
}

/// The divider walls each of `keys`, the finished vertices member
/// corners `at` are held as, meets: read over the fold's discards as a
/// face piece's are ([`Obstacles::split_corner`]), each wall cited by
/// its parent.
fn member_corner_walls<T: geom_core::Decide>(
    at: &BTreeSet<(usize, topo::VertexKey)>,
    keys: &[topo::VertexKey],
    body: &topo::Body<T>,
    (flush, fold): (&Flush<'_, T>, &Fold<'_>),
) -> Result<BTreeMap<Vec<StableName>, Vec<topo::VertexKey>>, NamingError> {
    let mut parent: BTreeSet<MemberFace> = BTreeSet::new();
    for &(i, w) in at {
        let faces = flush
            .member_by_index(i)
            .body
            .faces_of_vertex(w)
            .unwrap_or_else(|| {
                unreachable!(
                    "member {i}'s corner {w:?} has no faces: `Flush::member_vertices` read it \
                     off a live face's loop"
                )
            });
        parent.extend(faces.into_iter().map(|f| (i, f)));
    }
    let faces: BTreeSet<topo::FaceKey> = flush
        .face_ents
        .iter()
        .filter(|(_, ents)| !ents.is_disjoint(&parent))
        .map(|(&f, _)| f)
        .collect();
    let wall = |g: topo::FaceKey| {
        Ok(flush.face_names.get(&g).cloned().unwrap_or_else(|| {
            unreachable!("finished face {g:?} has no parent: `Parents::of` gives every face one")
        }))
    };
    fold.obstacles
        .split_corner(body, &parent, keys, &faces, wall)
}

/// A member edge at a union vertex has a side no pairwise judgement of
/// its member with the members its name reads it against classified.
const UNCLASSIFIED_SIDE: &str = "a member edge in a union vertex's name has a side no \
     pairwise judgement classified against the members the name cites";

/// A fold name's sense along a member's seam edge names neither of the
/// edge's sides as the union cites them, so it reads no way along the
/// edge to carry onto the line the vertex now cites. Typed rather than a
/// panic: the fold's name reaches here with its faces cited as their
/// parents ([`CiteParents`]), and no one has shown a parent never
/// re-spells a member's side.
const RESENSED_OFF_SIDES: &str = "a union vertex name's sense along a member's seam edge names \
                                  neither of the edge's sides, so no way along it carries onto \
                                  the line it now cites";

/// Why a rewrite that re-cites line `old` of vertex name `was` as `now`
/// is no rewrite [`Flush::cited_whole`] makes.
fn recited_off_line(was: &StableName, old: &StableName, now: &StableName) -> String {
    format!(
        "union vertex name {was:?} re-cites line {old:?} as {now:?}: `Flush::cited_whole` turns a \
         member edge into the line of member edges through the vertex, and leaves any other line \
         one of none"
    )
}

/// The [`SegRewrite`] of [`cite_member_edges`]: an embedded qualified piece
/// of a member edge becomes the member edge. Only names the UNION
/// minted are rewritten or entered: the name a member-keyed `From` carries is
/// the member's own, final in the member.
struct WholeMemberEdges<'f, 'a, T: geom_core::Decide> {
    union: RecipeNodeId,
    /// The vertex whose name is rewritten, when it is one vertex: a
    /// member edge its name cites is cited as the line of member edges
    /// through it ([`Flush::line_through`]).
    at: Option<topo::VertexKey>,
    flush: &'f Flush<'a, T>,
}

impl<T: geom_core::Decide> SegRewrite for WholeMemberEdges<'_, '_, T> {
    type Error = NamingError;

    fn name(&mut self, n: &StableName) -> Result<Carry, NamingError> {
        if n.node != self.union {
            return Ok(Carry::Keep);
        }
        if let Some((member, edge, _)) = member_edge_piece(n) {
            let line = match self.at {
                Some(v) => self.flush.line_through(v, (member, edge))?,
                None => BTreeSet::from([(member, edge)]),
            };
            let whole = (*line_name(self.union, &line)).clone();
            return Ok(if whole == *n {
                Carry::Keep
            } else {
                Carry::Replace(whole)
            });
        }
        Ok(Carry::Descend)
    }

    fn descended(
        &mut self,
        n: &StableName,
        walked: StableName,
    ) -> Result<Option<StableName>, NamingError> {
        Ok((walked != *n).then_some(walked))
    }

    /// An index is the document's own: re-citing a member edge keeps it.
    fn index(&mut self, index: VarId) -> Result<VarId, Self::Error> {
        Ok(index)
    }
}

/// A member face, by entity: the member's position in the list and the
/// face's key in the member's own body. The position is bookkeeping and
/// reaches no name.
type MemberFace = (usize, topo::FaceKey);

/// **What a union's pairwise judgements record for its end pass**: the
/// member faces each consumed together (N2), and each member edge piece
/// each classified beside a point against the other member's closed body
/// (N2's senses).
///
/// A judgement is the two members' own union, so a pair it merged
/// (`BooleanNaming::merge_groups`) or whose region it held through the
/// other face (`BooleanNaming::substituted`) is one its margins decided one
/// carrier (declared or not, D10), and that lies with one orientation:
/// the kernel merges and covers no other. A certified pair the judgement
/// never brought together is in neither record and links nothing.
pub(crate) struct Links<T: geom_core::Real> {
    pairs: Vec<(MemberFace, MemberFace)>,
    classes: Vec<PairClass<T>>,
    unread: Vec<UnreadAt<T>>,
}

impl<T: geom_core::Real> Default for Links<T> {
    fn default() -> Self {
        Self {
            pairs: Vec::new(),
            classes: Vec::new(),
            unread: Vec::new(),
        }
    }
}

/// One member edge piece a pairwise judgement classified beside a point
/// (`BooleanNaming::edge_classes`), in member terms: the edge `edge` of
/// member `member` (its own body's key), the point `at` the piece starts
/// or ends at, whether it starts there as the edge is stored, and
/// whether it lies in member `other`'s closed body.
struct PairClass<T: geom_core::Real> {
    member: usize,
    other: usize,
    edge: topo::EdgeKey,
    at: geom_core::Point3<T>,
    starts: bool,
    inside: bool,
}

/// One member vertex a pairwise judgement met the other member at and
/// read no partner of (`BooleanNaming::unread_partners`), in member
/// terms: member `member`'s edges at the point `at` have no piece the
/// judgement against member `other` classified. The judgement's own
/// `record` rides along: the vertex's operand in it, the partner it met
/// and why that read nothing.
struct UnreadAt<T: geom_core::Real> {
    member: usize,
    other: usize,
    at: geom_core::Point3<T>,
    record: topo::UnreadPartner,
}

impl<T: geom_core::Real> Links<T> {
    /// What one judgement recorded: members `a` (operand A) and `b`
    /// (operand B), the kernel's record `naming` of their union and its
    /// result `body`; `reads` are the members' reads and `b_body` is
    /// member `b`'s body.
    ///
    /// A classified piece, and a vertex read no partner of, is placed at
    /// the result vertex its operand key settles on
    /// ([`topo::Settler::settle`]). A key that lies at no
    /// vertex of the result holds no piece of it: one the result never
    /// received (a B key the graft left out, discarded material whose
    /// vertex died with no row), or one settled inside an edge or a face.
    ///
    /// # Errors
    ///
    /// [`NamingError::Settle`] where a key's rows lead to a cell the
    /// result does not hold: a corrupt record, not an absent key; and
    /// [`NamingError::UnsupportedTurnedCell`] where the judgement keeps
    /// a cell a sphere re-chart turned
    /// ([`super::emit_topo::refuse_turned_cells`]).
    pub(crate) fn judged(
        &mut self,
        a: usize,
        b: usize,
        naming: &topo::BooleanNaming,
        body: &topo::Body<T>,
        reads: [crate::VarId; 2],
        b_body: &topo::Body<T>,
    ) -> Result<(), NamingError> {
        // A judgement's body is named by no pair emitter, and its faces
        // are the node's links.
        let inv = super::emit_topo::GraftInverse::of(naming);
        super::emit_topo::refuse_turned_cells(body, naming, &inv, reads, b_body)?;
        let descent = topo::FaceDescent::of(naming);
        let member = |(operand, f): (topo::Operand, topo::FaceKey)| match operand {
            topo::Operand::A => (a, f),
            topo::Operand::B => (b, f),
        };
        for faces in descent.merged().values() {
            let mut held = BTreeSet::new();
            for &g in faces {
                held.insert(member(descent.result_face(g)));
            }
            for &x in held.iter().filter(|m| m.0 == a) {
                for &y in held.iter().filter(|m| m.0 == b) {
                    self.pairs.push((x, y));
                }
            }
        }
        for s in &naming.substituted {
            let (fa, fb) = match s.operand {
                topo::Operand::A => (s.face, s.by),
                topo::Operand::B => (s.by, s.face),
            };
            self.pairs.push((
                (a, descent.clone_face(topo::Operand::A, fa)),
                (b, descent.clone_face(topo::Operand::B, fb)),
            ));
        }
        let grafted: BTreeMap<topo::VertexKey, topo::VertexKey> =
            naming.graft_vertices.iter().copied().collect();
        let settler = naming.settler(body);
        // The members an operand's key reads, and the result point it
        // settles at, if any.
        let place = |operand: topo::Operand, vertex: topo::VertexKey| {
            let (keys, member, other) = match operand {
                topo::Operand::A => (naming.a_keys, a, b),
                topo::Operand::B => (naming.b_keys, b, a),
            };
            let key = match keys {
                topo::OperandKeys::Direct => Some(vertex),
                topo::OperandKeys::Grafted => grafted.get(&vertex).copied(),
                topo::OperandKeys::Absent => None,
            };
            let Some(key) = key else { return Ok(None) };
            let v = match settler.settle(key) {
                Ok(topo::Cell::Vertex(v)) => v,
                Ok(topo::Cell::Edge(_) | topo::Cell::Face(_)) => return Ok(None),
                // No fusion and no row moved the key: it is a vertex the
                // result never held.
                Err(topo::SettleError::Dead { vertex, reached })
                    if reached == topo::Cell::Vertex(vertex)
                        && naming.vertex_merges.survivor(vertex) == vertex =>
                {
                    return Ok(None);
                }
                Err(e) => return Err(NamingError::from(e)),
            };
            Ok(Some((member, other, vertex_point(body, v))))
        };
        for row in &naming.edge_classes {
            let Some((member, other, at)) = place(row.operand, row.vertex)? else {
                continue;
            };
            self.classes.push(PairClass {
                member,
                other,
                edge: row.edge,
                at,
                starts: row.starts,
                inside: row.class != topo::SideCode::Out,
            });
        }
        for u in &naming.unread_partners {
            let Some((member, other, at)) = place(u.operand, u.vertex)? else {
                continue;
            };
            self.unread.push(UnreadAt {
                member,
                other,
                at,
                record: *u,
            });
        }
        Ok(())
    }
}

/// **What a union's fold records for its end pass**: which member faces
/// each face of the accumulation descends from, read off the kernel's
/// n-ary door ([`topo::MemberLineage`], the last step's), and every
/// step's discards ([`Obstacles`]) keyed by the member faces the
/// discarded face descends from.
pub(crate) struct Fold<'l> {
    /// Accumulation face → the member faces it descends from.
    lineage: Cow<'l, topo::MemberLineage>,
    obstacles: Obstacles<MemberFace>,
}

impl<'l> Fold<'l> {
    /// The fold before any step: the first member's body.
    pub(crate) fn new<T: geom_core::Real>(first: usize, body: &topo::Body<T>) -> Self {
        Self {
            lineage: Cow::Owned(topo::MemberLineage::of_member(first, body)),
            obstacles: Obstacles::new(),
        }
    }

    /// One fold step: `member` folded into the accumulation, giving
    /// `result` with the kernel's record `naming`, whose faces descend
    /// from the member faces `lineage` holds. A discarded face is read
    /// through the kernel's own rule ([`topo::MemberLineage::operand_faces`]).
    pub(crate) fn step<T: geom_core::Real>(
        &mut self,
        member: usize,
        naming: &topo::BooleanNaming,
        result: &topo::Body<T>,
        lineage: &'l topo::MemberLineage,
    ) -> Result<(), NamingError> {
        let descent = topo::FaceDescent::of(naming);
        let held = &self.lineage;
        self.obstacles.record(naming, result, |operand, f| {
            Ok(held.operand_faces(member, operand, descent.clone_face(operand, f)))
        })?;
        self.lineage = Cow::Borrowed(lineage);
        Ok(())
    }
}

/// **A union's faces, grouped by parent** (N2, N3).
///
/// Member faces are linked in member space, by the pairs the pairwise
/// judgements consumed ([`Links`]), and linking is transitive. The
/// member faces so linked are one parent, named `Merged` of all of them;
/// a member face nothing links is its own parent, named as the member's
/// face. A finished face takes the parent of the member faces it
/// descends from, read by entity off the fold ([`Fold`]); which faces a
/// fold step merged, kept or discarded does not enter, and a finished
/// face descending from faces of two parents refuses. Tied member faces
/// are separate parents spelled alike.
struct Parents {
    /// Finished face → its parent.
    of_face: BTreeMap<topo::FaceKey, usize>,
    /// Member face, by name → the names of the parents it is linked
    /// into (several only for a tied name whose candidates went apart).
    /// A merged member face's constituents are keyed too, each to its
    /// face's parents.
    of_member: BTreeMap<MemberEntity, BTreeSet<StableName>>,
    all: Vec<Parent>,
}

/// One parent of [`Parents`].
struct Parent {
    name: StableName,
    /// The member faces linked into it.
    entities: BTreeSet<MemberFace>,
    /// The finished faces that are it or pieces of it.
    faces: Vec<topo::FaceKey>,
    /// Whether a member face in it is tied in its member's table (N2's
    /// tie, propagated).
    tied: bool,
}

impl Parents {
    /// The parents of the finished body's faces.
    fn of<T: geom_core::Decide>(
        union: RecipeNodeId,
        body: &topo::Body<T>,
        members: &[Member<'_, T>],
        fold: &Fold<'_>,
        links: &Links<T>,
    ) -> Result<Self, NamingError> {
        // Finished face → the member faces it descends from, and the
        // least of them.
        let mut rows: Vec<(topo::FaceKey, &BTreeSet<MemberFace>, MemberFace)> = Vec::new();
        for (f, _) in body.faces() {
            let (from, first) = fold
                .lineage
                .member_faces(f)
                .and_then(|s| Some((s, *s.first()?)))
                .unwrap_or_else(|| {
                    unreachable!(
                        "finished face {f:?} of union {union:?} descends from no member face: \
                         the body is the fold's last result, and each fold step gives each of \
                         its faces the member faces its operands' faces descend from, one or \
                         more (`Fold::step`)"
                    )
                });
            rows.push((f, from, first));
        }
        let mut link = LeastRoot::new();
        for &(x, y) in &links.pairs {
            link.insert(x);
            link.insert(y);
            link.join(x, y);
        }
        for (_, from, _) in &rows {
            for &m in *from {
                link.insert(m);
            }
        }
        // What the fold merged does not link: a finished face whose member
        // faces are of several parents has no one parent to take. Two
        // faces of DIFFERENT members a fold step merged meet in their own
        // pair's union too (a union only adds material, and the pair is
        // judged under the same declarations), and that judgement merges
        // or covers them. Two faces of ONE member a judgement merges link
        // nothing: `Links::judged` records member-a × member-b pairs
        // only. A legal recipe reaches that (a bar through a sunk boss's
        // two wall pieces,
        // `wire_legal_union_refusals::a_bar_through_a_sunk_boss_refuses_at_the_unions_parents`)
        // and refuses here under the bug's words; what such a pair is
        // is `work/refs/a-union-judgement-that-merges-two-faces-of-one-member-links-neither.md`'s.
        // A curved run the pair leaves unmerged (`merge_skipped`), the
        // fold, holding no fewer of its faces, leaves unmerged too.
        for (_, from, _) in &rows {
            let mut roots = from.iter().map(|&m| link.root(m));
            let first = roots.next();
            if roots.any(|r| Some(r) != first) {
                return Err(NamingError::Emission {
                    what: "a union's face descends from member faces no judged pair links",
                });
            }
        }
        let mut classes: BTreeMap<MemberFace, BTreeSet<MemberFace>> = BTreeMap::new();
        for m in link.keys() {
            classes.entry(link.root(m)).or_default().insert(m);
        }
        let mut parents = Parents {
            of_face: BTreeMap::new(),
            of_member: BTreeMap::new(),
            all: Vec::new(),
        };
        let mut index: BTreeMap<MemberFace, usize> = BTreeMap::new();
        for (r, entities) in classes {
            let mut from = BTreeSet::new();
            let mut tied = false;
            for &(m, f) in &entities {
                let member = members.get(m).unwrap_or_else(|| {
                    unreachable!(
                        "union {union:?} links member face {f:?} under index {m} of {} members: \
                         every index is a member's place in the fold (`Fold`, `Links::judged`)",
                        members.len()
                    )
                });
                let of = member
                    .table
                    .name_ref_of(&ent(0, EntityKey::Face(f)))
                    .unwrap_or_else(|| {
                        unreachable!(
                            "member {:?}'s face {f:?}, which union {union:?} holds, is unnamed in \
                             its table: a member's table is total over its body (`check_total`)",
                            member.read
                        )
                    })
                    .clone();
                tied |= member.table.is_tied(&of);
                from.insert((member.read, of));
            }
            let name = parent_name(union, &from);
            // A nested union's merged member face is cited by the fold
            // through its constituents (N3's flat mint), each keyed by
            // the member, so each cites the parent the face is in.
            let constituents = from.iter().flat_map(|(m, of)| {
                crate::names::merged::constituents_through_wrappers(of.name())
                    .unwrap_or_default()
                    .into_iter()
                    .map(|c| (*m, NameRef::new(c)))
            });
            for m in from.iter().cloned().chain(constituents) {
                parents.of_member.entry(m).or_default().insert(name.clone());
            }
            index.insert(r, parents.all.len());
            parents.all.push(Parent {
                name,
                entities,
                faces: Vec::new(),
                tied,
            });
        }
        for (f, _, first) in rows {
            let p = *index.get(&link.root(first)).unwrap_or_else(|| {
                unreachable!(
                    "finished face {f:?} of union {union:?} has no parent: every member face it \
                     descends from was linked, and each link class is a parent"
                )
            });
            parents.all[p].faces.push(f);
            parents.of_face.insert(f, p);
        }
        Ok(parents)
    }

    /// No parents: for a table that names no face.
    #[cfg(test)]
    fn empty() -> Self {
        Self {
            of_face: BTreeMap::new(),
            of_member: BTreeMap::new(),
            all: Vec::new(),
        }
    }

    /// Finished face `f`'s parent.
    #[track_caller]
    fn face(&self, f: topo::FaceKey) -> &Parent {
        let Some(&p) = self.of_face.get(&f) else {
            unreachable!(
                "finished face {f:?} of a union has no parent: `Parents::of` gives every face of \
                 the finished body one"
            )
        };
        &self.all[p]
    }

    /// The parent a face name some row embeds cites: the one parent all
    /// the member faces it lists are linked into. `None` for a name that
    /// is not a member face, a merge of member faces or a piece of
    /// either.
    ///
    /// A name listing member faces of several parents cites no one face
    /// of the finished body, and refuses.
    fn cited(
        &self,
        union: RecipeNodeId,
        n: &StableName,
    ) -> Result<Option<StableName>, NamingError> {
        if n.kind != EntityKind::Face {
            return Ok(None);
        }
        let mut from = BTreeSet::new();
        member_faces(n, &mut from);
        let mut cited = BTreeSet::new();
        for m in &from {
            match self.of_member.get(m) {
                Some(names) => cited.extend(names.iter().cloned()),
                None => {
                    cited.insert(entity_name(union, m));
                }
            }
        }
        let mut cited = cited.into_iter();
        let Some(first) = cited.next() else {
            return Ok(None);
        };
        if cited.next().is_some() {
            return Err(NamingError::UnsupportedUnionName {
                name: Box::new(n.clone()),
                shape: UnionNameShape::SeveralParents,
            });
        }
        Ok(Some(first))
    }
}

/// What [`name_by_parents`] returns: the table, the seam rows held
/// What [`name_by_parents`] hands on.
struct ByParents {
    /// Every face, and every row but a seam edge or a vertex.
    table: NameTable,
    /// Seam rows lying along a member edge, their faces cited as
    /// parents, for [`group_member_edges`].
    held: Vec<(StableName, Entry)>,
    /// Vertex rows, their faces cited as parents and the seams they cite
    /// by their heads.
    vertices: Vec<(StableName, Entry)>,
    /// The seam edges, grouped by their two parents.
    seams: Vec<PieceGroup>,
    /// The groups formed.
    groups: GroupRecord,
}

/// A parent's name: its one member face, or `Merged` of all of them,
/// flat (N3): a member face that is itself a merged face, a nested
/// union's, stands for its constituents, each keyed by its member.
fn parent_name(union: RecipeNodeId, from: &BTreeSet<MemberEntity>) -> StableName {
    match from.iter().collect::<Vec<_>>().as_slice() {
        [one] => entity_name(union, one),
        many => canonical::form(StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::Merged(
                many.iter()
                    .flat_map(|m| {
                        let n = entity_name(union, m);
                        crate::names::merged::constituents_through_wrappers(&n)
                            .unwrap_or_else(|| vec![n])
                    })
                    .collect(),
            )],
        }),
    }
}

/// **A union's faces and seam edges are named for parents, read off the
/// finished body** (N2, N3).
///
/// - **Faces.** A parent the finished body holds as one face is that
///   face's name. A parent it holds as several faces qualifies each with
///   one `Fragment(Borders)` over the divider walls it borders, each
///   cited by its parent's name: the one rule the pair boolean reads
///   ([`Obstacles::split`], over every fold step's discards). Pieces
///   with one set are N2's tie.
/// - **Seam edges.** A seam edge that lies along no member edge is
///   `Seam` of the parents on its two sides, in name order. Several such
///   edges between the same two parents are pieces of that seam, grouped
///   to be told apart by their ends once the vertices are named.
/// - **Groups.** The groups formed are returned for the diagnosis
///   ladder. Each is one parent's: a face group holds one parent's
///   faces, and a seam group is keyed by its two parents' member faces.
///   So two tied parents, which share a base, form two groups, and each
///   counts its own.
/// - **Every other row** cites a face as its parent, and a re-derived
///   seam edge by its head (N2: a vertex cites an edge by its head,
///   never by a piece's qualifier).
///
/// A seam edge lying along a member edge is returned apart, with only
/// the faces it cites rewritten: [`group_member_edges`] names it as a
/// piece of that member edge, so its seam spelling is never published
/// and must not collide with one that is.
fn name_by_parents<T: geom_core::Decide>(
    union: RecipeNodeId,
    t: &NameTable,
    body: &topo::Body<T>,
    parents: &Parents,
    fold: &Fold<'_>,
    flush: &Flush<'_, T>,
) -> Result<ByParents, NamingError> {
    let mut out = NameTable::new();
    let mut tie = TieRows::default();
    let mut rec = GroupRecord::new();
    let mut held: Vec<(StableName, Entry)> = Vec::new();

    // ---- Seam edges, by the parents on their two sides. ----
    let is_seam = |name: &StableName| {
        name.kind == EntityKind::Edge && matches!(name.path.first(), Some(RoleSeg::Seam { .. }))
    };
    let mut seams: BTreeSet<topo::EdgeKey> = BTreeSet::new();
    // A unique seam row's collapsed spelling → its edge.
    let mut respelled: BTreeMap<StableName, topo::EdgeKey> = BTreeMap::new();
    for (name, entry) in t.iter() {
        if !is_seam(name) {
            continue;
        }
        let keys = match entry {
            Entry::Unique(e) => vec![e.key],
            Entry::Tied(es) => es.iter().map(|e| e.key).collect(),
        };
        let mut edges = Vec::with_capacity(keys.len());
        for key in keys {
            let EntityKey::Edge(k) = key else {
                unreachable!(
                    "union seam row {name:?} names {key:?}: a table seats a row only on entities \
                     of its name's kind"
                );
            };
            edges.push(k);
        }
        if edges.iter().any(|k| flush.edges.contains_key(k)) {
            held.push((name.clone(), entry.clone()));
            continue;
        }
        if let [k] = edges.as_slice() {
            respelled.insert(name.clone(), *k);
        }
        seams.extend(edges);
    }
    // (base, the two parents) → (from a tie, its edges).
    type SeamKey = (StableName, ParentEntities<MemberFace>);
    let mut seam_groups: BTreeMap<SeamKey, (bool, Vec<topo::EdgeKey>)> = BTreeMap::new();
    for &k in &seams {
        // `Incidence` dedups a wrap edge's one face: a wrap edge pierced
        // at a point and joined back after the boolean is one way in.
        let Some([f0, f1]) = flush.inc.edge_faces.get(&k).map(Vec::as_slice) else {
            return Err(NamingError::UnsupportedSeamSides {
                edge: k,
                shape: SeamSidesShape::NotTwoFaces,
            });
        };
        // A pair that merges A's face with B's links them, but the fold
        // may skip the same curved group (`SkippedMerge` reads the
        // group's shape, and the fold's group holds more faces).
        if parents.of_face.get(f0) == parents.of_face.get(f1) {
            return Err(NamingError::UnsupportedSeamSides {
                edge: k,
                shape: SeamSidesShape::InsideOneParent,
            });
        }
        let (p0, p1) = (parents.face(*f0), parents.face(*f1));
        let (a, b) = if p0.name <= p1.name {
            (&p0.name, &p1.name)
        } else {
            (&p1.name, &p0.name)
        };
        let base = StableName {
            kind: EntityKind::Edge,
            node: union,
            path: vec![RoleSeg::Seam {
                a: NameRef::new(a.clone()),
                b: NameRef::new(b.clone()),
            }],
        };
        let sides = BTreeSet::from([p0.entities.clone(), p1.entities.clone()]);
        let slot = seam_groups.entry((base, sides)).or_default();
        slot.0 |= p0.tied || p1.tied;
        slot.1.push(k);
    }
    // Edge → its seam's head, for the rows that cite it.
    let mut seam_head: BTreeMap<topo::EdgeKey, StableName> = BTreeMap::new();
    let mut groups = Vec::with_capacity(seam_groups.len());
    for ((base, _), (tied, keys)) in seam_groups {
        rec.record(
            &base,
            keys.iter().map(|&k| ent(0, EntityKey::Edge(k))).collect(),
            GroupParent::Elsewhere,
        );
        for &k in &keys {
            seam_head.insert(k, base.clone());
        }
        groups.push(PieceGroup {
            base,
            from_tie: tied,
            edges: keys,
            lone: Lone::Whole,
        });
    }

    // ---- Faces, by parent. ----
    for parent in parents.all.iter().filter(|p| !p.faces.is_empty()) {
        let faces = &parent.faces;
        rec.record(
            &parent.name,
            faces.iter().map(|&f| ent(0, EntityKey::Face(f))).collect(),
            GroupParent::Elsewhere,
        );
        name_parent_faces(
            &mut out,
            &mut tie,
            parent.tied,
            parent.name.clone(),
            faces,
            &[],
            (&fold.obstacles, body, &parent.entities),
            |g| Ok(parents.face(g).name.clone()),
        )?;
    }

    // ---- Every other row: its faces cited as parents. ----
    let mut cite = CiteParents {
        union,
        parents,
        seams: respelled
            .into_iter()
            .filter_map(|(old, k)| Some((old, seam_head.get(&k)?.clone())))
            .collect(),
    };
    let mut vertices = Vec::new();
    for (name, entry) in t.iter() {
        if name.kind == EntityKind::Face || is_seam(name) {
            continue;
        }
        let spelled = name.clone().rewrite_path(&mut cite)?;
        if name.kind == EntityKind::Vertex {
            vertices.push((spelled, entry.clone()));
        } else {
            put_entry(&mut out, spelled, entry)?;
        }
    }
    for (name, _) in &mut held {
        *name = name.clone().rewrite_path(&mut cite)?;
    }
    tie.flush(&mut out)?;
    Ok(ByParents {
        table: out,
        held,
        vertices,
        seams: groups,
        groups: rec,
    })
}

/// The [`SegRewrite`] of [`name_by_parents`]: a face a name embeds is
/// cited as its parent, and a seam edge the union re-derived as it now
/// publishes. Only names the UNION minted are rewritten: a
/// member-keyed `From`'s own name is the member's, final in the member.
struct CiteParents<'p> {
    union: RecipeNodeId,
    parents: &'p Parents,
    /// A unique seam row's collapsed spelling → its published head.
    seams: BTreeMap<StableName, StableName>,
}

impl SegRewrite for CiteParents<'_> {
    type Error = NamingError;

    fn name(&mut self, n: &StableName) -> Result<Carry, NamingError> {
        if n.node != self.union {
            return Ok(Carry::Keep);
        }
        if let Some(parent) = self.parents.cited(self.union, n)? {
            return Ok(if parent == *n {
                Carry::Keep
            } else {
                Carry::Replace(parent)
            });
        }
        if let Some(now) = self.seams.get(n) {
            return Ok(if now == n {
                Carry::Keep
            } else {
                Carry::Replace(now.clone())
            });
        }
        Ok(Carry::Descend)
    }

    fn descended(
        &mut self,
        n: &StableName,
        walked: StableName,
    ) -> Result<Option<StableName>, NamingError> {
        Ok((walked != *n).then_some(walked))
    }

    /// An index is the document's own: citing a parent keeps it.
    fn index(&mut self, index: VarId) -> Result<VarId, Self::Error> {
        Ok(index)
    }
}

/// A whole fold table in the union's published space — the collapse
/// [`name_union`] publishes, without its end passes over member edges
/// and without the totality check.
///
/// Two callers, and the split is what tells them apart. [`name_union`]
/// rewrites the LAST step's table, which names a finished body and is
/// held to totality. The fold rewrites an INTERMEDIATE step's, and
/// for a different purpose: a step's coincidence rows are named in this
/// node's own space, so the fold reads the accumulation in that space.
/// The entities are untouched — same keys, same ties — so the keys a
/// lookup returns are the accumulation's own.
///
/// Rows of one seam edge the fold met at several steps collapse to one
/// spelling: two faces of one member set seam wherever a later member
/// fills a gap between them, a step each. They are pieces of one seam,
/// held as one tied row here until the end pass tells them apart by
/// their ends ([`name_union`]); any other two rows that collapse alike
/// refuse as a duplicate.
pub(crate) fn collapse_table(
    node: RecipeNodeId,
    folded: &NameTable,
) -> Result<NameTable, NamingError> {
    let mut rows: BTreeMap<StableName, Vec<&Entry>> = BTreeMap::new();
    for (name, entry) in folded.iter() {
        rows.entry(collapse(node, name)?).or_default().push(entry);
    }
    let mut t = NameTable::new();
    for (name, entries) in rows {
        let seam_edge =
            name.kind == EntityKind::Edge && matches!(name.path.as_slice(), [RoleSeg::Seam { .. }]);
        match entries.as_slice() {
            [entry] => put_entry(&mut t, name, entry)?,
            several if seam_edge => {
                let mut ents: Vec<_> = several
                    .iter()
                    .flat_map(|e| match e {
                        Entry::Unique(r) => vec![*r],
                        Entry::Tied(rs) => rs.clone(),
                    })
                    .collect();
                ents.sort_unstable();
                t.insert_tied(name, ents)?;
            }
            _ => {
                return Err(NamingError::Duplicate {
                    name: Box::new(name),
                });
            }
        }
    }
    Ok(t)
}

/// One fold-table name in the union's published space.
///
/// The collapse [`name_union`] applies to a whole table (before its end
/// passes over member edges, which need the finished body), exposed
/// for one name read out of a step that did NOT finish: a fold step's
/// table is still headed by a fold side, and a name in that shape is
/// in the fold's internal space, which no published table holds. The
/// fragment groups (`names::groups`) collapse a step's bases through
/// here to compare them with the published ones.
pub(crate) fn collapse_name(
    node: RecipeNodeId,
    name: &StableName,
) -> Result<StableName, NamingError> {
    collapse(node, name)
}

/// **The member cells a fold row's cell is made of** (DM4, "A row
/// names member cells"): `name` is a collapsed fold name
/// ([`collapse_table`]), each member cell by its member's read and its
/// name in that read's table, sorted, each once. Under a member's
/// `From` head the cell is a piece of the member entity the head names,
/// and is that entity. A cell the fold minted is the member cells it is
/// made of, read the same way down every name it holds:
///
/// - a `Crossing` vertex, its edge and its face;
/// - a `Seam` edge, its two faces;
/// - a junction's run of `Seam` lines, or an `EdgeCrossing` run, each
///   line;
/// - a `Merged` face or edge, each constituent.
///
/// # Panics
///
/// For a head followed by anything but `Fragment`s (a junction's run
/// aside), a head no fold step mints, or a fold side's read under a
/// `From`: the collapse mints none of those.
pub(crate) fn member_cells(name: &StableName) -> Vec<(VarId, StableName)> {
    fn walk(name: &StableName, out: &mut Vec<(VarId, StableName)>) {
        let foreign = |why: &str| -> ! {
            unreachable!(
                "a union fold row's cell {name:?} {why}: a fold row's cell is a collapsed name of \
                 the fold's own table, whose every row the collapse rewrote into the union's \
                 member-keyed shapes (`collapse`)"
            )
        };
        if let Some((read, of, _)) = member_piece(name) {
            if is_fold_side(read) {
                foreign("is keyed by a fold side");
            }
            out.push((read, of.name().clone()));
            return;
        }
        if is_junction(name) {
            for seg in &name.path {
                if let RoleSeg::Seam { a, b } = seg {
                    walk(a.name(), out);
                    walk(b.name(), out);
                }
            }
            return;
        }
        let Some((head, tail)) = name.path.split_first() else {
            foreign("has no role");
        };
        if !tail.iter().all(|s| matches!(s, RoleSeg::Fragment(_))) {
            foreign("carries a segment past its head that is not a fragment");
        }
        match head {
            RoleSeg::Crossing { edge, face, .. } => {
                walk(edge.name(), out);
                walk(face.name(), out);
            }
            RoleSeg::Seam { a, b } => {
                walk(a.name(), out);
                walk(b.name(), out);
            }
            RoleSeg::EdgeCrossing(run) => run.iter().for_each(|line| walk(line.line.name(), out)),
            RoleSeg::Merged(set) => set.iter().for_each(|n| walk(n, out)),
            _ => foreign("has a head no fold step mints"),
        }
    }
    let mut out = Vec::new();
    walk(name, &mut out);
    out.sort();
    out.dedup();
    out
}

use super::canonical::{self, is_junction};

/// One fold-table name, keyed by member.
///
/// Returns a name in the UNION's own space (`node` is the union's, as
/// every fold row's already is): either one [`RoleSeg::From`],
/// [`RoleSeg::Seam`], [`RoleSeg::Merged`] or [`RoleSeg::OutputBody`]
/// head followed by the `Fragment` discriminators the fold
/// accumulated, outermost step last — or, for a seam JUNCTION vertex,
/// a run of two or more `Seam` lines and nothing else.
///
/// Two halves. [`orient`] rewrites every name the path holds into this
/// node's space and keeps the fold's order, so every seam pair is still
/// in the fold step's name order. The canonical form
/// (`names::canonical::form`) then puts the name-ordered positions in
/// order. Each name the path embeds comes out of this function whole,
/// so it is canonical before the path holding it is ordered.
///
/// A fold name nests one level per fold step, so the collapse keeps
/// its own stack (`names::nest::descend`): every held name a collapse
/// reads is collapsed first ([`held_collapses`]), once, and kept by its
/// address.
fn collapse(node: RecipeNodeId, root: &StableName) -> Result<StableName, NamingError> {
    descend(root, &mut Collapse(node))
}

/// [`collapse`] as a [`Descent`]: a level is one fold name collapsed,
/// kept for the levels above it behind a handle.
struct Collapse(RecipeNodeId);

impl<'s> Descent<'s> for Collapse {
    type Level = StableName;
    type Kept = NameRef;
    type Error = NamingError;

    fn first(&mut self, name: &'s StableName, _: &Kept<NameRef>, out: &mut Vec<&'s StableName>) {
        out.extend(held_collapses(name));
    }

    fn level(
        &mut self,
        name: &'s StableName,
        kept: &Kept<NameRef>,
    ) -> Result<StableName, Stopped<'s, NamingError>> {
        collapse_one(self.0, name, kept)
    }

    fn keep(&mut self, _: &'s StableName, collapsed: StableName) -> Result<NameRef, NamingError> {
        Ok(NameRef::new(collapsed))
    }
}

/// The held names `name`'s collapse reads: the seam sides and merged
/// constituents at the foot of its fold-side descent, and every
/// `Borders` wall and `Ends` vertex along it, in the order the collapse
/// meets them.
fn held_collapses(name: &StableName) -> Vec<&StableName> {
    let mut out = Vec::new();
    let mut at = name;
    while let Some((head, tail)) = at.path.split_first() {
        match head {
            RoleSeg::From { read, of } if is_fold_side(*read) => at = of,
            RoleSeg::Seam { .. } => {
                for seg in &at.path {
                    if let RoleSeg::Seam { a, b } = seg {
                        out.extend([&**a, &**b]);
                    }
                }
                break;
            }
            RoleSeg::Merged(set) => {
                out.extend(set);
                break;
            }
            _ => break,
        }
        for seg in tail {
            if let RoleSeg::Fragment(Qualifier::Borders(names) | Qualifier::Ends(names)) = seg {
                out.extend(names);
            }
        }
    }
    out
}

/// One fold name collapsed, every held name it reads read from `done`.
fn collapse_one<'s>(
    node: RecipeNodeId,
    name: &'s StableName,
    done: &Kept<NameRef>,
) -> Result<StableName, Stopped<'s, NamingError>> {
    let fold = name;
    let name = canonical::form(orient(node, name, done)?);
    // The junction's run is NOT deduplicated, unlike a `Merged` set.
    // The pair emitter deduplicates the lines before it mints, so the
    // run holds k DISTINCT fold-space lines; two collapsing to one
    // would make the rewrite many-to-one on lines, and dropping one
    // would publish a name for a vertex with fewer lines than it has.
    // A `Merged` set is a set of FACES whose name is the set (N3), so
    // there a repeat is the same constituent reached twice, and two
    // merges that collapse to one set collide at insert instead.
    if is_junction(&name) && name.path.windows(2).any(|w| w[0] == w[1]) {
        return Err(NamingError::UnsupportedUnionName {
            name: Box::new(fold.clone()),
            shape: UnionNameShape::JunctionLinesCollide,
        }
        .into());
    }
    Ok(name)
}

/// A fold name's sense with every thing it names collapsed.
fn collapsed_sense<'s>(
    sense: &'s Sense,
    done: &Kept<NameRef>,
) -> Result<Sense, Stopped<'s, NamingError>> {
    Ok(match sense {
        Sense::Through { entering } => Sense::Through {
            entering: entering
                .iter()
                .map(|n| Ok(done.need(n)?.clone()))
                .collect::<Result<_, Stopped<'s, NamingError>>>()?,
        },
        plain @ (Sense::Enters | Sense::Leaves | Sense::Outside | Sense::Inside) => plain.clone(),
    })
}

/// [`collapse`]'s rewrite half: the fold-table name with every name it
/// holds collapsed into this node's space, in the fold's order.
///
/// A fold-side `From` head is descended through by this same half, so
/// the inner name's own positions stay in fold order until the one
/// canonicalization at the top.
///
/// The descent is a loop: the tails of the levels it passes through
/// are kept, and put after the foot's path innermost first, which is
/// the order a descent level by level would write them in.
fn orient<'s>(
    node: RecipeNodeId,
    name: &'s StableName,
    done: &Kept<NameRef>,
) -> Result<StableName, Stopped<'s, NamingError>> {
    // `at` is the name or the tail segment that differs from what the
    // pair emitter mints.
    let foreign = |at: &dyn core::fmt::Debug, why: &str| -> ! {
        unreachable!(
            "union {node:?}'s fold name {name:?} holds {at:?}, {why}: every row of a fold table \
             is the pair emitter's mint over operands `member_view` keyed under this union, and \
             the pair emitter mints none of that"
        )
    };
    let mut tails: Vec<&[RoleSeg]> = Vec::new();
    let mut at = name;
    let (mut path, foot_tail) = loop {
        if at.node != node {
            foreign(at, "minted by another node");
        }
        let Some((head, mut tail)) = at.path.split_first() else {
            foreign(at, "a name with no role");
        };
        let path = match head {
            // Already member-keyed: the foot of a descent chain, put there
            // by `member_view` before the step ran.
            RoleSeg::From { read, .. } if !is_fold_side(*read) => vec![head.clone()],
            // The accumulated body's own name at every step, and the
            // union's at the last one: one body out, one output-body row.
            RoleSeg::OutputBody => vec![RoleSeg::OutputBody],
            // The descent. Every operand of every step is in this node's
            // space, so a fold side's argument is always an earlier step's
            // row: descended THROUGH, carrying its own discriminators out
            // with it.
            RoleSeg::From { of, .. } => {
                tails.push(tail);
                at = of;
                continue;
            }
            // A seam: one line, each side collapsed, with any `Fragment`
            // tail after it.
            //
            // Or a seam JUNCTION: the pair emitter names the VERTEX where
            // k ≥ 2 seam lines meet by the run of those lines' `Seam`
            // segments and nothing after them, so a run is admitted in
            // exactly that shape — a vertex, the whole path — and any
            // other run (an edge's, or one followed by a discriminator) is
            // a shape the pair emitter does not mint. The pair emitter
            // ordered the run in the fold's space; a name stable under
            // member reordering is ordered by the member-space lines,
            // which the canonicalization does.
            RoleSeg::Seam { a, b }
                if tail
                    .first()
                    .is_some_and(|s| matches!(s, RoleSeg::Seam { .. })) =>
            {
                if at.kind != EntityKind::Vertex {
                    foreign(at, "a run of seams naming no vertex");
                }
                let mut lines = vec![seam_line(a, b, done)?];
                for seg in std::mem::take(&mut tail) {
                    let RoleSeg::Seam { a, b } = seg else {
                        foreign(at, "a run of seams followed by another segment");
                    };
                    lines.push(seam_line(a, b, done)?);
                }
                lines
            }
            RoleSeg::Seam { a, b } => {
                vec![seam_line(a, b, done)?]
            }
            // A crossing: its edge, its face and the things its sense
            // names collapsed.
            RoleSeg::Crossing { edge, face, sense } => vec![RoleSeg::Crossing {
                edge: done.need(edge)?.clone(),
                face: done.need(face)?.clone(),
                sense: collapsed_sense(sense, done)?,
            }],
            RoleSeg::EdgeCrossing(run) => vec![RoleSeg::EdgeCrossing(
                run.iter()
                    .map(|l| {
                        Ok(RunLine {
                            line: done.need(&l.line)?.clone(),
                            sense: collapsed_sense(&l.sense, done)?,
                        })
                    })
                    .collect::<Result<_, Stopped<'s, NamingError>>>()?,
            )],
            // An F7 merged face: its constituents are result-face names in
            // the minting node's space (N3), so they stay in this union's
            // space, each collapsed by this same rule.
            //
            // The pair emitter mints `Merged` for the kernel's merge
            // groups — faces whose margins decide them one carrier — so a
            // fold step's table carries these rows.
            //
            // The constituent set is FLAT (N3): a constituent is never
            // itself a bare merged face. The mint (`emit_topo`'s
            // merge-group loop) holds that at the first door; this is the
            // same rule read at the union's second door — a constituent
            // that collapses to a merged face, bare or through any
            // `From` wrapping, is refused as the
            // emission bug it is, never flattened. A fragment of a merged
            // face is a fragment, not a merge (`RoleSeg::Merged`'s doc).
            //
            // The canonical form makes the constituent SET the name, as
            // the pair emitter's mint does (it tells two faces of one
            // set apart by a `Fragment` tail): two rows collapsing to
            // one name collide LOUDLY at insert (`DuplicateName` →
            // typed `NamingError`), never silently aliasing two faces
            // onto one name.
            RoleSeg::Merged(constituents) => {
                let mut set = Vec::with_capacity(constituents.len());
                for c in constituents {
                    let c = done.need(c)?.name().clone();
                    if crate::names::merged::constituents_through_wrappers(&c).is_some() {
                        foreign(
                            at,
                            "a merged face whose constituent collapses to a merged face",
                        );
                    }
                    set.push(c);
                }
                vec![RoleSeg::Merged(set)]
            }
            // A `Fragment` is a TAIL segment — it discriminates a head,
            // it is never one — and everything after it is a segment the
            // boolean emitter does not mint at all, so a fold table
            // carrying either is an emission bug. The long half is
            // [`never_in_a_boolean_table`], which is where a new
            // `RoleSeg` is classified; this match still stops the
            // compiler if one is added and not classified there.
            RoleSeg::Fragment(_) | never_in_a_boolean_table!() => {
                foreign(at, "a head no boolean table carries")
            }
        };
        break (path, tail);
    };
    for seg in core::iter::once(foot_tail)
        .chain(tails.into_iter().rev())
        .flatten()
    {
        path.push(match seg {
            // The walls are the fold step's own names, each wall's
            // parent as the step's table names it (N3), as a seam's
            // sides are, so they collapse the same way.
            RoleSeg::Fragment(Qualifier::Borders(walls)) => RoleSeg::Fragment(Qualifier::Borders(
                walls
                    .iter()
                    .map(|n| Ok(done.need(n)?.name().clone()))
                    .collect::<Result<Vec<_>, Stopped<'s, NamingError>>>()?,
            )),
            // So are a piece's ends.
            RoleSeg::Fragment(Qualifier::Ends(ends)) => RoleSeg::Fragment(Qualifier::Ends(
                ends.iter()
                    .map(|n| Ok(done.need(n)?.name().clone()))
                    .collect::<Result<Vec<_>, Stopped<'s, NamingError>>>()?,
            )),
            // A split's qualifier, which no boolean table carries.
            RoleSeg::Fragment(Qualifier::Keeps(_)) => {
                foreign(seg, "a split's qualifier in its tail")
            }
            // Only a `Fragment` follows the head in a boolean table;
            // anything else in the tail is an emission bug — the head
            // segments above included, which are heads and not
            // discriminators. A junction's run of `Seam`s never reaches
            // this loop — the head arm takes the whole path or refuses
            // it — so a `Seam` here follows a `From`, `Merged` or
            // `OutputBody` head, or a `Fragment`, and the pair emitter
            // mints none of those shapes. The long half is
            // [`never_in_a_boolean_table`], for the reason the head
            // match names.
            RoleSeg::OutputBody
            | RoleSeg::From { .. }
            | RoleSeg::Seam { .. }
            | RoleSeg::Crossing { .. }
            | RoleSeg::EdgeCrossing { .. }
            | RoleSeg::Merged(_)
            | never_in_a_boolean_table!() => foreign(seg, "a head segment in its tail"),
        });
    }
    Ok(StableName {
        kind: name.kind,
        node,
        path,
    })
}

/// One seam line between two members, in the union's space and in the
/// fold's order.
///
/// The pair emitter's `a`/`b` are the crossing faces' parents as the
/// fold step's own table names them, in that table's name order, which
/// is this node's space, so each is collapsed the same way every other
/// row is. The pair keeps the fold's order here, and the
/// canonicalization puts it in the member-space names' order.
fn seam_line<'s>(
    a: &'s StableName,
    b: &'s StableName,
    done: &Kept<NameRef>,
) -> Result<RoleSeg, Stopped<'s, NamingError>> {
    Ok(RoleSeg::Seam {
        a: done.need(a)?.clone(),
        b: done.need(b)?.clone(),
    })
}

#[cfg(test)]
mod tests {
    //! The rewrite's own rows: a merged face collapses to its flat
    //! member-space set, and a NESTED merged face — a shape the pair
    //! emitter's flat mint never produces — panics as the kernel bug it
    //! is instead of being flattened here. A seam junction's run of lines
    //! collapses to the sorted set of its member-space lines, and a
    //! `Seam` outside that leading run panics as foreign.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::{CapEnd, EntityKind};
    use topo::construction::Construct;

    fn face(node: RecipeNodeId, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node,
            path,
        }
    }

    /// Member `m`'s start cap, as `member_view` keys it under `union`.
    fn member_cap(union: RecipeNodeId, m: u64) -> StableName {
        face(
            union,
            vec![RoleSeg::From {
                read: crate::VarId::new(1, m),
                of: face(RecipeNodeId::new(0, m), vec![RoleSeg::Cap(CapEnd::Start)]).into(),
            }],
        )
    }

    fn from_a(union: RecipeNodeId, inner: StableName) -> StableName {
        face(
            union,
            vec![RoleSeg::From {
                read: crate::names::FOLD_A,
                of: inner.into(),
            }],
        )
    }

    fn from_b(union: RecipeNodeId, inner: StableName) -> StableName {
        face(
            union,
            vec![RoleSeg::From {
                read: crate::names::FOLD_B,
                of: inner.into(),
            }],
        )
    }

    #[test]
    fn a_flat_merged_face_collapses_to_its_member_space_set() {
        let union = RecipeNodeId::new(0, 9);
        // Step 2's merge of step 1's merge with a third member, as the
        // flat mint spells it: every constituent descends to a member.
        let folded = face(
            union,
            vec![RoleSeg::Merged(vec![
                from_a(union, from_a(union, member_cap(union, 1))),
                from_a(union, from_b(union, member_cap(union, 2))),
                from_b(union, member_cap(union, 3)),
            ])],
        );
        let out = collapse(union, &folded).unwrap();
        let mut want = vec![
            member_cap(union, 1),
            member_cap(union, 2),
            member_cap(union, 3),
        ];
        want.sort();
        assert_eq!(out.path, vec![RoleSeg::Merged(want)]);
    }

    #[test]
    #[should_panic(expected = "a merged face whose constituent collapses to a merged face")]
    fn a_nested_merged_face_panics_as_an_emission_bug() {
        let union = RecipeNodeId::new(0, 9);
        let inner = face(
            union,
            vec![RoleSeg::Merged(vec![
                from_a(union, member_cap(union, 1)),
                from_b(union, member_cap(union, 2)),
            ])],
        );
        let nested = face(
            union,
            vec![RoleSeg::Merged(vec![
                from_a(union, inner),
                from_b(union, member_cap(union, 3)),
            ])],
        );
        let _ = collapse(union, &nested);
    }

    fn vertex(node: RecipeNodeId, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind: EntityKind::Vertex,
            node,
            path,
        }
    }

    fn seam(a: StableName, b: StableName) -> RoleSeg {
        RoleSeg::Seam {
            a: a.into(),
            b: b.into(),
        }
    }

    #[test]
    fn a_seam_junction_collapses_to_its_sorted_member_space_lines() {
        let union = RecipeNodeId::new(0, 9);
        // Two lines of one junction, as a fold step writes them at a
        // step whose B operand is member 1: each line's first side is an
        // accumulation row (members 3 and 2, reached through the
        // descent) and its second member 1's face. In the fold's
        // space the member-3 line sorts first; in the union's it sorts
        // second, and every line's sides swap.
        let folded = vertex(
            union,
            vec![
                seam(from_a(union, member_cap(union, 3)), member_cap(union, 1)),
                seam(from_b(union, member_cap(union, 2)), member_cap(union, 1)),
            ],
        );
        let out = collapse(union, &folded).unwrap();
        assert_eq!(
            out.path,
            vec![
                seam(member_cap(union, 1), member_cap(union, 2)),
                seam(member_cap(union, 1), member_cap(union, 3)),
            ]
        );
    }

    #[test]
    #[should_panic(expected = "a head segment in its tail")]
    fn a_seam_after_a_fragment_is_foreign() {
        let union = RecipeNodeId::new(0, 9);
        let line = || seam(from_a(union, member_cap(union, 2)), member_cap(union, 1));
        let name = vertex(
            union,
            vec![
                line(),
                RoleSeg::Fragment(Qualifier::Borders(Vec::new())),
                line(),
            ],
        );
        let _ = collapse(union, &name);
    }

    /// Two lines of one junction, as a fold step writes them.
    fn junction_lines(union: RecipeNodeId) -> Vec<RoleSeg> {
        vec![
            seam(from_a(union, member_cap(union, 3)), member_cap(union, 1)),
            seam(from_b(union, member_cap(union, 2)), member_cap(union, 1)),
        ]
    }

    /// The junction shape itself is admitted: a run of seams naming a
    /// vertex, and nothing after it.
    #[test]
    fn a_run_of_seams_is_admitted_as_a_whole_vertex_path() {
        let union = RecipeNodeId::new(0, 9);
        assert!(collapse(union, &vertex(union, junction_lines(union))).is_ok());
    }

    /// An EDGE named by a run: the pair emitter names a seam edge by one
    /// line.
    #[test]
    #[should_panic(expected = "a run of seams naming no vertex")]
    fn a_run_of_seams_naming_an_edge_panics() {
        let union = RecipeNodeId::new(0, 9);
        let _ = collapse(
            union,
            &StableName {
                kind: EntityKind::Edge,
                node: union,
                path: junction_lines(union),
            },
        );
    }

    /// A run followed by a discriminator: a junction is unique per line
    /// set, so the pair emitter never qualifies one.
    #[test]
    #[should_panic(expected = "a run of seams followed by another segment")]
    fn a_qualified_run_of_seams_panics() {
        let union = RecipeNodeId::new(0, 9);
        let mut path = junction_lines(union);
        path.push(RoleSeg::Fragment(Qualifier::Borders(Vec::new())));
        let _ = collapse(union, &vertex(union, path));
    }

    /// Member `m`'s first lateral edge, as `member_view` keys it.
    fn member_edge(union: RecipeNodeId, m: u64) -> StableName {
        StableName {
            kind: EntityKind::Edge,
            node: union,
            path: vec![RoleSeg::From {
                read: crate::VarId::new(1, m),
                of: StableName {
                    kind: EntityKind::Edge,
                    node: RecipeNodeId::new(0, m),
                    path: vec![RoleSeg::LateralEdge(
                        crate::names::role::ProfileVertexRef::Piece {
                            step: crate::node::StepId::new(0, 0),
                            role: crate::names::PieceRole::Leg,
                        },
                    )],
                }
                .into(),
            }],
        }
    }

    /// `inner` descended through one fold step, keeping its kind.
    fn through_a(inner: StableName) -> StableName {
        StableName {
            kind: inner.kind,
            node: inner.node,
            path: vec![RoleSeg::From {
                read: crate::names::FOLD_A,
                of: inner.into(),
            }],
        }
    }

    /// Member `m`'s first lateral edge's start vertex, as `member_view`
    /// keys it.
    fn member_vertex(union: RecipeNodeId, m: u64) -> StableName {
        let mut v = member_edge(union, m);
        v.kind = EntityKind::Vertex;
        v
    }

    /// **A seam edge piece whose pair canonicalization swaps keeps its
    /// ends.** The fold writes member 5's face first here and member
    /// 2's second; in name order member 2 comes first, and the
    /// piece's `Ends`, which orders nothing, comes through as it was
    /// written, each end collapsed.
    #[test]
    fn a_swapped_seam_edge_piece_keeps_its_ends() {
        let union = RecipeNodeId::new(0, 9);
        let ends = vec![member_vertex(union, 3), member_vertex(union, 4)];
        let swapped = seam(through_a(member_cap(union, 5)), member_cap(union, 2));
        let kept = seam(through_a(member_cap(union, 2)), member_cap(union, 5));
        for line in [swapped, kept] {
            let piece = StableName {
                kind: EntityKind::Edge,
                node: union,
                path: vec![line, RoleSeg::Fragment(Qualifier::Ends(ends.clone()))],
            };
            let out = collapse(union, &piece).unwrap();
            assert!(
                matches!(&out.path[0], RoleSeg::Seam { a, .. } if **a == member_cap(union, 2)),
                "the pair is not in name order: {out:?}"
            );
            assert_eq!(
                out.path[1],
                RoleSeg::Fragment(Qualifier::Ends(ends.clone())),
                "{out:?}"
            );
        }
    }

    /// A seam EDGE between member 5's and member 2's caps, as a fold
    /// step wrote it: `swap` puts member 5 first, which the union's name
    /// order reverses.
    fn seam_edge(union: RecipeNodeId, swap: bool) -> StableName {
        let line = if swap {
            seam(through_a(member_cap(union, 5)), member_cap(union, 2))
        } else {
            seam(through_a(member_cap(union, 2)), member_cap(union, 5))
        };
        StableName {
            kind: EntityKind::Edge,
            node: union,
            path: vec![line],
        }
    }

    /// **A later step's pieces of a seam edge keep their ends**: the
    /// seam's pair goes into name order and the piece's `Ends`
    /// stands, whichever side the emitter wrote first.
    #[test]
    fn a_descent_through_a_swapped_seam_edge_keeps_its_ends() {
        let union = RecipeNodeId::new(0, 9);
        let ends = vec![member_vertex(union, 3), member_vertex(union, 4)];
        let mut out = Vec::new();
        for swap in [true, false] {
            let piece = StableName {
                kind: EntityKind::Edge,
                node: union,
                path: vec![
                    RoleSeg::From {
                        read: crate::names::FOLD_A,
                        of: seam_edge(union, swap).into(),
                    },
                    RoleSeg::Fragment(Qualifier::Ends(ends.clone())),
                ],
            };
            out.push(collapse(union, &piece).unwrap());
        }
        assert_eq!(out[0], out[1], "one name from either side");
        assert_eq!(
            out[0].path.last(),
            Some(&RoleSeg::Fragment(Qualifier::Ends(ends))),
            "{:?}",
            out[0]
        );
    }

    /// **A crossing of a seam edge keeps the side it enters along
    /// through a swap of the seam's pair**: the sense names the side, so
    /// the pair goes into name order and the sense comes through with
    /// its side collapsed, whichever side the fold wrote first.
    #[test]
    fn a_crossing_of_a_swapped_seam_edge_keeps_the_side_it_enters_along() {
        use crate::names::role::Sense;
        let union = RecipeNodeId::new(0, 9);
        let mut out = Vec::new();
        for swap in [true, false] {
            let crossing = vertex(
                union,
                vec![RoleSeg::Crossing {
                    edge: through_a(seam_edge(union, swap)).into(),
                    face: member_cap(union, 7).into(),
                    sense: Sense::Through {
                        entering: vec![through_a(member_cap(union, 5)).into()],
                    },
                }],
            );
            out.push(collapse(union, &crossing).unwrap());
        }
        assert_eq!(out[0], out[1], "one name from either side");
        assert!(
            matches!(
                out[0].path.as_slice(),
                [RoleSeg::Crossing {
                    sense: Sense::Through { entering },
                    ..
                }] if entering.as_slice() == [NameRef::new(member_cap(union, 5))]
            ),
            "the sense names the side it entered along: {:?}",
            out[0]
        );
    }

    #[test]
    fn two_junction_lines_collapsing_to_one_refuse() {
        let union = RecipeNodeId::new(0, 9);
        // Distinct in the fold's space — member 3's face reached as the
        // A operand's row and as the B operand's — and one line in the
        // union's.
        let name = vertex(
            union,
            vec![
                seam(from_a(union, member_cap(union, 3)), member_cap(union, 1)),
                seam(from_b(union, member_cap(union, 3)), member_cap(union, 1)),
            ],
        );
        let err = collapse(union, &name).unwrap_err();
        assert!(
            matches!(
                &err,
                NamingError::UnsupportedUnionName {
                    name: at,
                    shape: UnionNameShape::JunctionLinesCollide,
                } if at.node == union
            ),
            "{err:?}"
        );
    }

    /// A body holding two edges, and their keys: the arena is the only
    /// source of an `EdgeKey`.
    fn two_edge_body() -> (topo::Body<f64>, topo::EdgeKey, topo::EdgeKey) {
        let mut body = topo::Body::<f64>::new();
        let mut mint = |x: f64| {
            let born = body
                .mvfs(geom_core::Point3::new(x, 0.0, 0.0), true)
                .expect("mvfs births a lone vertex");
            body.mev_line(
                topo::construction::MevSite::Lone {
                    r#loop: born.r#loop,
                },
                geom_core::Point3::new(x + 1.0, 0.0, 0.0),
                geom_core::Tol::witness(),
            )
            .expect("mev grows the loop by one edge")
            .edge
        };
        let (a, b) = (mint(0.0), mint(10.0));
        (body, a, b)
    }

    fn edge_ref(k: topo::EdgeKey) -> crate::names::table::EntityRef {
        crate::names::table::EntityRef {
            body: 0,
            key: EntityKey::Edge(k),
        }
    }

    /// **The fold's tie over two pieces of a member edge is grouped
    /// under the member edge**, as one group descended from a tie: the
    /// pieces are told apart by their ends afterwards, so no rule has to
    /// pick one edge, and the fold's qualifier is not carried.
    #[test]
    fn a_tied_member_edge_piece_is_grouped_under_its_member_edge() {
        let (union, m) = (RecipeNodeId::new(0, 9), RecipeNodeId::new(0, 4));
        let (body, k0, k1) = two_edge_body();
        let edge = StableName {
            kind: EntityKind::Edge,
            node: m,
            path: vec![RoleSeg::LateralEdge(
                crate::names::role::ProfileVertexRef::Piece {
                    step: crate::node::StepId::new(0, 0),
                    role: crate::names::PieceRole::Leg,
                },
            )],
        };
        let read = crate::VarId::new(1, 4);
        let mut piece = member_name(union, read, &edge);
        piece.path.push(RoleSeg::Fragment(Qualifier::Ends(vec![
            member_vertex(union, 3),
            member_vertex(union, 4),
        ])));
        let bnd = geom_core::Band::new(1e-9, 1e-6).unwrap();
        let mut member_table = NameTable::new();
        member_table.insert(edge.clone(), edge_ref(k0)).unwrap();
        let mut tied_rows = NameTable::new();
        tied_rows
            .insert_tied(piece, vec![edge_ref(k0), edge_ref(k1)])
            .unwrap();
        let members = [Member {
            read,
            body: &body,
            table: &member_table,
        }];
        let parents = Parents::empty();
        let flush = Flush::of(union, &body, &members, &parents, bnd).unwrap();
        let (rest, groups, _) = group_member_edges(tied_rows, Vec::new(), &flush).unwrap();
        assert_eq!(rest.iter().count(), 0, "the row was not taken into a group");
        let [group] = groups.as_slice() else {
            panic!("one group, got {}", groups.len());
        };
        assert_eq!(group.base, member_name(union, read, &edge));
        assert!(group.from_tie, "the group does not descend from the tie");
        assert_eq!(group.edges, vec![k0, k1]);
    }

    /// The pieces [`cite_member_edges`] is run on: member 5's body
    /// holding its one edge (x = 0 → 1), that member's table naming it,
    /// and a union body holding two lone vertices on the edge (x = 0.25
    /// and 0.75).
    struct CitedGroup {
        union: RecipeNodeId,
        member_body: topo::Body<f64>,
        member_table: NameTable,
        body: topo::Body<f64>,
        along: [topo::VertexKey; 2],
    }

    impl CitedGroup {
        fn new() -> Self {
            let union = RecipeNodeId::new(0, 9);
            let mut member_body = topo::Body::<f64>::new();
            let born = member_body
                .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0), true)
                .expect("mvfs births a lone vertex");
            let edge = member_body
                .mev_line(
                    topo::construction::MevSite::Lone {
                        r#loop: born.r#loop,
                    },
                    geom_core::Point3::new(1.0, 0.0, 0.0),
                    geom_core::Tol::witness(),
                )
                .expect("mev grows the loop by one edge")
                .edge;
            let mut member_table = NameTable::new();
            let keyed = member_edge(union, 5);
            let [RoleSeg::From { of, .. }] = keyed.path.as_slice() else {
                unreachable!("member_edge is one From segment")
            };
            member_table.insert((**of).clone(), edge_ref(edge)).unwrap();
            let mut body = topo::Body::<f64>::new();
            let mut lone = |x: f64| {
                body.mvfs(geom_core::Point3::new(x, 0.0, 0.0), true)
                    .expect("mvfs births a lone vertex")
                    .vertex
            };
            let along = [lone(0.25), lone(0.75)];
            CitedGroup {
                union,
                member_body,
                member_table,
                body,
                along,
            }
        }

        fn members(&self) -> [Member<'_, f64>; 1] {
            [Member {
                read: crate::VarId::new(1, 5),
                body: &self.member_body,
                table: &self.member_table,
            }]
        }

        /// A piece of member 5's edge as the fold spells it once a step
        /// has cut it in two.
        fn piece(&self, k: u64) -> StableName {
            let mut n = member_edge(self.union, 5);
            n.path.push(RoleSeg::Fragment(Qualifier::Ends(vec![
                member_vertex(self.union, 10 + k),
                member_vertex(self.union, 11 + k),
            ])));
            n
        }

        /// A vertex name of the group: `side` against member 2's cap, in
        /// name order as a union's seams are.
        fn named(&self, side: StableName) -> StableName {
            let cap = member_cap(self.union, 2);
            let line = if side < cap {
                seam(side, cap)
            } else {
                seam(cap, side)
            };
            vertex(self.union, vec![line])
        }

        /// The one row a cited table holds: both vertices, tied under
        /// `name`.
        fn tied(&self, out: &NameTable, name: Option<&StableName>) {
            let want = |v| crate::names::table::EntityRef {
                body: 0,
                key: EntityKey::Vertex(v),
            };
            let rows: Vec<_> = out.iter().collect();
            let [(row, entry)] = rows.as_slice() else {
                panic!("one row: {out:?}")
            };
            assert_eq!(
                *entry,
                &Entry::Tied(self.along.map(want).to_vec()),
                "both vertices, tied: {out:?}"
            );
            if let Some(name) = name {
                assert_eq!(*row, name, "{out:?}");
            }
        }

        fn cite(&self, rows: Vec<(StableName, topo::VertexKey)>) -> Result<NameTable, NamingError> {
            let rows = rows
                .into_iter()
                .map(|(name, v)| {
                    let at = crate::names::table::EntityRef {
                        body: 0,
                        key: EntityKey::Vertex(v),
                    };
                    (name, Entry::Unique(at))
                })
                .collect();
            let bnd = geom_core::Band::new(1e-9, 1e-6).unwrap();
            let members = self.members();
            let parents = Parents::empty();
            let flush = Flush::of(self.union, &self.body, &members, &parents, bnd)?;
            let lines = EdgeLines {
                member: BTreeMap::new(),
                seam: BTreeMap::new(),
            };
            cite_member_edges(
                NameTable::new(),
                rows,
                &self.body,
                (&flush, &Fold::new(0, &self.body)),
                (&lines, &Links::default()),
            )
        }
    }

    /// **A member edge with no classified side at a vertex the
    /// judgement read no partner of refuses why**
    /// (`topo::UnreadPartner`), and at any other vertex stays the
    /// missing-row bug. Constructed on [`CitedGroup`]'s member 5 judged
    /// against a member 2, since a model does not reach it
    /// (`a_mate_on_a_full_turn_bore_names` lists partners and names
    /// cleanly): a pairwise judgement's operands are members, so a
    /// listed vertex is a member's own, and a finished vertex at a member
    /// vertex is cited as one, never read by its lines in
    /// [`Flush::meeting`], the one caller of `sense_against`.
    #[test]
    fn a_side_unclassed_at_an_unread_partner_refuses_why() {
        let g = CitedGroup::new();
        let empty = NameTable::new();
        let [five] = g.members();
        let members = [
            five,
            Member {
                read: crate::VarId::new(1, 2),
                body: &g.member_body,
                table: &empty,
            },
        ];
        let bnd = geom_core::Band::new(1e-9, 1e-6).unwrap();
        let flush = Flush::of(g.union, &g.body, &members, &Parents::empty(), bnd).unwrap();
        let [listed, other] = g.along;
        let mut corners = g.member_body.vertices().map(|(k, _)| k);
        let record = topo::UnreadPartner {
            operand: topo::Operand::B,
            vertex: corners.next().unwrap(),
            partner: corners.next().unwrap(),
            neighbourhood: topo::UnreadNeighbourhood::ConeUnread,
        };
        let links = Links {
            pairs: Vec::new(),
            classes: Vec::new(),
            unread: vec![UnreadAt {
                member: 0,
                other: 1,
                at: vertex_point(&g.body, listed),
                record,
            }],
        };
        let RoleSeg::From { of, .. } = &member_edge(g.union, 5).path[0] else {
            unreachable!("member_edge is one From segment")
        };
        let line = (members[0].read, NameRef::new((**of).clone()));
        let others = BTreeSet::from([members[1].read]);
        assert!(matches!(
            flush.sense_against(listed, &line, &others, &links),
            Err(NamingError::PartnerUnread { vertex, partner, unread })
                if vertex == listed && partner == record.partner && unread == record
        ));
        // Judged against a member the name does not cite, the record
        // says nothing of this crossing.
        assert!(matches!(
            flush.sense_against(listed, &line, &BTreeSet::from([members[0].read]), &links),
            Err(NamingError::Emission { what }) if what == UNCLASSIFIED_SIDE
        ));
        assert!(matches!(
            flush.sense_against(other, &line, &others, &links),
            Err(NamingError::Emission { what }) if what == UNCLASSIFIED_SIDE
        ));
    }

    /// **A seam-vertex group on one member edge ties whether or not its
    /// vertices moved** (N4): nothing ranks it, so the one tied row is
    /// the same in every member order.
    ///
    /// Two vertices of one seam, member 5's edge against member 2's cap.
    /// In one member order a step cut the edge before the cap met it, so
    /// the fold's names cite pieces and the pass cites the edge whole. In
    /// the other the edge was whole when the cap met it, so no name
    /// moves. No document reaches that today (no union in the review
    /// corpus forms a group of two), which is why it is pinned on the
    /// pass.
    #[test]
    fn a_cited_group_ties_whether_or_not_it_moved() {
        let g = CitedGroup::new();
        let whole = member_edge(g.union, 5);
        let [first, second] = g.along;
        let cut = g
            .cite(vec![
                (g.named(g.piece(0)), first),
                (g.named(g.piece(1)), second),
            ])
            .unwrap();
        let whole_then = g
            .cite(vec![
                (g.named(whole.clone()), second),
                (g.named(whole.clone()), first),
            ])
            .unwrap();
        for out in [&cut, &whole_then] {
            g.tied(out, Some(&g.named(whole.clone())));
        }
    }

    /// **Crossings of one member edge by one face tie only among those
    /// of one sense** (N2, N4). Two crossings of member 5's edge by
    /// member 2's cap, the fold's names citing pieces of the edge: with
    /// one sense they are one tied row; with opposite senses each is its
    /// own.
    #[test]
    fn a_cited_crossing_group_ties_only_among_one_sense() {
        use crate::names::role::Sense;

        let g = CitedGroup::new();
        let crossing = |edge: StableName, sense| {
            vertex(
                g.union,
                vec![RoleSeg::Crossing {
                    edge: NameRef::new(edge),
                    face: NameRef::new(member_cap(g.union, 2)),
                    sense,
                }],
            )
        };
        let whole = member_edge(g.union, 5);
        let [first, second] = g.along;
        let at = |v| {
            Some(Entry::Unique(crate::names::table::EntityRef {
                body: 0,
                key: EntityKey::Vertex(v),
            }))
        };
        let one_sense = g
            .cite(vec![
                (crossing(g.piece(0), Sense::Enters), first),
                (crossing(g.piece(1), Sense::Enters), second),
            ])
            .unwrap();
        g.tied(&one_sense, Some(&crossing(whole.clone(), Sense::Enters)));
        let two_senses = g
            .cite(vec![
                (crossing(g.piece(0), Sense::Enters), first),
                (crossing(g.piece(1), Sense::Leaves), second),
            ])
            .unwrap();
        assert_eq!(two_senses.len(), 2, "{two_senses:?}");
        for (sense, want) in [(Sense::Enters, first), (Sense::Leaves, second)] {
            assert_eq!(
                two_senses
                    .lookup(&crossing(whole.clone(), sense.clone()))
                    .cloned(),
                at(want),
                "two senses: the {sense:?} crossing is its own"
            );
        }
    }

    /// A union vertex `Crossing` of `edge` and member 2's cap.
    fn crossing(
        union: RecipeNodeId,
        edge: StableName,
        sense: crate::names::role::Sense,
    ) -> StableName {
        vertex(
            union,
            vec![RoleSeg::Crossing {
                edge: NameRef::new(edge),
                face: NameRef::new(member_cap(union, 2)),
                sense,
            }],
        )
    }

    /// The `Merged` set of members `ms`' edges.
    fn edge_set(union: RecipeNodeId, ms: [u64; 2]) -> StableName {
        StableName {
            kind: EntityKind::Edge,
            node: union,
            path: vec![RoleSeg::Merged(
                ms.iter().map(|&m| member_edge(union, m)).collect(),
            )],
        }
    }

    /// Members 3, 5 and 6 each hold one straight edge on the x axis (3
    /// runs −x, 5 and 6 run +x), member 4 one on the y axis, and member
    /// 7 one on the x axis past 5's end, running −x: `check` reads the
    /// `Flush` over them.
    fn crossing_fixture(check: impl FnOnce(&Flush<'_, f64>, RecipeNodeId)) {
        let union = RecipeNodeId::new(0, 9);
        let line = |p: [f64; 3], q: [f64; 3]| {
            let mut body = topo::Body::<f64>::new();
            let born = body
                .mvfs(geom_core::Point3::new(p[0], p[1], p[2]), true)
                .expect("mvfs births a lone vertex");
            let edge = body
                .mev_line(
                    topo::construction::MevSite::Lone {
                        r#loop: born.r#loop,
                    },
                    geom_core::Point3::new(q[0], q[1], q[2]),
                    geom_core::Tol::witness(),
                )
                .expect("mev grows the loop by one edge")
                .edge;
            (body, edge)
        };
        let table = |m: u64, edge| {
            let keyed = member_edge(union, m);
            let [RoleSeg::From { of, .. }] = keyed.path.as_slice() else {
                unreachable!("member_edge is one From segment")
            };
            let mut t = NameTable::new();
            t.insert((**of).clone(), edge_ref(edge)).unwrap();
            t
        };
        let (b3, e3) = line([1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        let (b4, e4) = line([0.5, -0.5, 0.0], [0.5, 0.5, 0.0]);
        let (b5, e5) = line([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let (b6, e6) = line([0.25, 0.0, 0.0], [0.75, 0.0, 0.0]);
        let (b7, e7) = line([2.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
        let tables = [
            table(3, e3),
            table(4, e4),
            table(5, e5),
            table(6, e6),
            table(7, e7),
        ];
        let members = [(3, &b3), (4, &b4), (5, &b5), (6, &b6), (7, &b7)]
            .into_iter()
            .zip(&tables)
            .map(|((m, body), table)| Member {
                read: crate::VarId::new(1, m),
                body,
                table,
            })
            .collect::<Vec<_>>();
        let mut body = topo::Body::<f64>::new();
        body.mvfs(geom_core::Point3::new(0.5, 0.0, 0.0), true)
            .expect("mvfs births a lone vertex");
        let bnd = geom_core::Band::new(1e-9, 1e-6).unwrap();
        let parents = Parents::empty();
        let flush = Flush::of(union, &body, &members, &parents, bnd).unwrap();
        check(&flush, union);
    }

    /// **A rewrite's crossing senses are re-read along the edges it now
    /// cites, side by side by image** ([`Flush::resensed`]), over
    /// [`crossing_fixture`].
    /// - A `Crossing` re-cited from 5's edge to 3's flips its sense, and
    ///   to 6's keeps it.
    /// - An `EdgeCrossing` whose sides the rewrite put in name order is
    ///   matched by image, so each sense stays with its edge.
    /// - A side that cites a member edge before the rewrite and the line
    ///   of several flush member edges after it crosses `Through` the
    ///   edges whose direction enters, whichever is least.
    #[test]
    fn a_rewrites_crossing_senses_are_reread_side_by_side_along_the_edges_it_cites() {
        use crate::names::role::Sense;
        crossing_fixture(|flush, union| {
            let was = crossing(union, member_edge(union, 5), Sense::Enters);
            let flipped = flush
                .resensed(
                    &was,
                    None,
                    crossing(union, member_edge(union, 3), Sense::Enters),
                )
                .unwrap();
            assert_eq!(
                flipped,
                crossing(union, member_edge(union, 3), Sense::Leaves),
                "re-cited along an edge running the other way"
            );
            let kept = flush
                .resensed(
                    &was,
                    None,
                    crossing(union, member_edge(union, 6), Sense::Enters),
                )
                .unwrap();
            assert_eq!(
                kept,
                crossing(union, member_edge(union, 6), Sense::Enters),
                "re-cited along an edge running the same way"
            );

            let edge_crossing = |lines: &[(u64, Sense)]| {
                vertex(
                    union,
                    vec![RoleSeg::EdgeCrossing(
                        lines
                            .iter()
                            .map(|(m, sense)| RunLine {
                                line: NameRef::new(member_edge(union, *m)),
                                sense: sense.clone(),
                            })
                            .collect(),
                    )],
                )
            };
            let was = edge_crossing(&[(5, Sense::Enters), (4, Sense::Leaves)]);
            let ordered = edge_crossing(&[(4, Sense::Leaves), (5, Sense::Enters)]);
            assert_eq!(
                flush.resensed(&was, None, ordered.clone()).unwrap(),
                ordered,
                "each sense stays with its line where the run is reordered"
            );

            let at = flush.body.vertices().map(|(v, _)| v).next();
            let through = |m: u64| Sense::Through {
                entering: vec![NameRef::new(member_edge(union, m))],
            };
            assert_eq!(
                flush
                    .resensed(
                        &crossing(union, member_edge(union, 5), Sense::Enters),
                        at,
                        crossing(union, edge_set(union, [3, 5]), Sense::Enters),
                    )
                    .unwrap(),
                crossing(union, edge_set(union, [3, 5]), through(5)),
                "re-cited as a line of flush edges: 5's direction enters, 3's leaves"
            );
            assert_eq!(
                flush
                    .resensed(
                        &crossing(union, member_edge(union, 7), Sense::Enters),
                        at,
                        crossing(union, edge_set(union, [5, 7]), Sense::Enters),
                    )
                    .unwrap(),
                crossing(union, edge_set(union, [5, 7]), through(7)),
                "a span joined end to end lies on one line: 7's direction enters"
            );
        });
    }

    /// A side re-cited as a set of edges off one line panics:
    /// `Flush::cited_whole` never makes one.
    #[test]
    #[should_panic(expected = "off its line")]
    fn a_side_re_cited_as_a_set_off_one_line_panics() {
        use crate::names::role::Sense;
        crossing_fixture(|flush, union| {
            let at = flush.body.vertices().map(|(v, _)| v).next();
            let _ = flush.resensed(
                &crossing(union, member_edge(union, 5), Sense::Enters),
                at,
                crossing(union, edge_set(union, [3, 4]), Sense::Enters),
            );
        });
    }

    /// **A group of several vertices sharing a name that is not one seam
    /// ties** (N4): no rule asks what to rank them along. The name here is
    /// a run of two lines whose first cites a piece of member 5's edge, so
    /// the pass moves it.
    #[test]
    fn a_cited_group_that_is_not_one_seam_ties() {
        let g = CitedGroup::new();
        let other = seam(member_cap(g.union, 3), member_cap(g.union, 4));
        let run = |side: StableName| {
            let mut path = g.named(side).into_path();
            path.push(other.clone());
            vertex(g.union, path)
        };
        let out = g
            .cite(vec![
                (run(g.piece(0)), g.along[0]),
                (run(g.piece(1)), g.along[1]),
            ])
            .unwrap();
        g.tied(&out, None);
    }

    /// **A group on one seam neither of whose sides is a member edge
    /// ties** (N4). The seam's edge side is an edge that itself cites a
    /// piece of member 5's edge, so the pass moves it without making it
    /// a member edge.
    #[test]
    fn a_cited_group_with_no_member_edge_side_ties() {
        let g = CitedGroup::new();
        let carrying = |side: StableName| StableName {
            kind: EntityKind::Edge,
            node: g.union,
            path: vec![seam(side, member_cap(g.union, 3))],
        };
        let out = g
            .cite(vec![
                (g.named(carrying(g.piece(0))), g.along[0]),
                (g.named(carrying(g.piece(1))), g.along[1]),
            ])
            .unwrap();
        g.tied(&out, None);
    }

    /// **A cell the glue left unglued is a refusal, never a silent
    /// table** (REFERENCES DM5): a read spelled twice glues every cell to
    /// its twin, and the twins' rows collapse to the one name they share.
    /// Where a twin pair is left as two cells, the fold table holds both
    /// sides' rows of one member name, and the collapse refuses them as
    /// the emitter's `DuplicateName` rather than publishing one name for
    /// two entities.
    #[test]
    fn an_unglued_twin_refuses_as_a_duplicate_name() {
        let union = RecipeNodeId::new(0, 9);
        let cell = |i: u64| crate::EntityRef {
            body: 0,
            key: EntityKey::Face(topo::FaceKey::from(slotmap::KeyData::from_ffi(i))),
        };
        let mut folded = NameTable::new();
        folded
            .insert(from_a(union, member_cap(union, 1)), cell(1))
            .unwrap();
        folded
            .insert(from_b(union, member_cap(union, 1)), cell(2))
            .unwrap();
        match collapse_table(union, &folded) {
            Err(NamingError::Duplicate { name }) => assert_eq!(*name, member_cap(union, 1)),
            other => panic!("two cells under one member name refuse: {other:?}"),
        }
    }

    /// A unit cube, its extrude's own table, and the cube read as both
    /// a union's one member (read `(1, 7)`) and its finished body, each
    /// finished face descending from the member face it is: the fold
    /// before any step.
    struct SelfMember {
        union: RecipeNodeId,
        read: VarId,
        body: topo::Body<f64>,
        table: NameTable,
    }

    impl SelfMember {
        fn new() -> Self {
            use profile::RawLoop;
            let plane = profile::SketchPlane::from_frame(geom_core::RigidMotion::axes_xy(
                geom_core::Point3::new(0.0, 0.0, 0.0),
            ));
            let square = profile::ProfileLoop::polygon(
                [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]
                    .into_iter()
                    .map(|(x, y)| geom_core::Point2::new(x, y)),
            );
            let prof = profile::Profile::new(plane, vec![square])
                .validate(geom_core::Tol::witness())
                .unwrap();
            let built = sweep::extrude(
                &prof,
                sweep::Extrusion::Distance {
                    depth: 1.0_f64,
                    side: sweep::ExtrudeSide::Along,
                },
                geom_core::Tol::witness(),
            )
            .unwrap();
            let table = crate::names::emit_sweep::name_extrude(
                RecipeNodeId::new(0, 4),
                &built,
                &crate::eval::ProfilePieces::numbered(
                    &built.side_faces().iter().map(Vec::len).collect::<Vec<_>>(),
                ),
            )
            .unwrap();
            SelfMember {
                union: RecipeNodeId::new(0, 9),
                read: VarId::new(1, 7),
                body: built.body,
                table: (*table).clone(),
            }
        }

        /// The table with entities `a` and `b` tied under `a`'s name,
        /// `b`'s own name dropped: the member's own N2 tie.
        fn tie(&mut self, a: EntityKey, b: EntityKey) -> StableName {
            let name = self.table.name_of(&ent(0, a)).unwrap().clone();
            let mut t = NameTable::new();
            for (n, e) in self.table.iter() {
                let Entry::Unique(e) = e else {
                    unreachable!("an extrude ties nothing")
                };
                if e.key != a && e.key != b {
                    t.insert(n.clone(), *e).unwrap();
                }
            }
            t.insert_tied(name.clone(), vec![ent(0, a), ent(0, b)])
                .unwrap();
            self.table = t;
            name
        }

        fn members(&self) -> [Member<'_, f64>; 1] {
            [Member {
                read: self.read,
                body: &self.body,
                table: &self.table,
            }]
        }

        /// The member's flush reading, through [`Parents::of`] and
        /// [`Flush::of`] over a fold that is the member alone.
        fn flush<'a>(&'a self, members: &'a [Member<'a, f64>]) -> Flush<'a, f64> {
            let fold = Fold::new(0, &self.body);
            let parents =
                Parents::of(self.union, &self.body, members, &fold, &Links::default()).unwrap();
            Flush::of(
                self.union,
                &self.body,
                members,
                &parents,
                geom_core::Band::new(1e-9, 1e-6).unwrap(),
            )
            .unwrap()
        }

        /// A face of the cube, and an edge it bounds with its faces.
        fn face_and_edge(&self) -> (topo::FaceKey, topo::EdgeKey, [topo::FaceKey; 2]) {
            let inc = Incidence::of(&self.body);
            let (&k, fs) = inc.edge_faces.iter().next().unwrap();
            let [f0, f1] = fs.as_slice() else {
                panic!("a cube edge has two faces")
            };
            (*f0, k, [*f0, *f1])
        }
    }

    /// **A member face its member ties is read by its key** (N2: the
    /// candidates of a tied row are separate parents spelled alike): the
    /// joined-edge reading still finds the member edge between it and
    /// its neighbour, rather than reading the tied face as no face.
    #[test]
    fn a_member_face_its_member_ties_is_read_by_its_key() {
        let mut m = SelfMember::new();
        let (f, k, fs) = m.face_and_edge();
        let other = m
            .body
            .faces()
            .map(|(g, _)| g)
            .find(|g| !fs.contains(g))
            .unwrap();
        m.tie(EntityKey::Face(f), EntityKey::Face(other));
        let edge = m
            .table
            .name_ref_of(&ent(0, EntityKey::Edge(k)))
            .unwrap()
            .clone();
        let members = m.members();
        let flush = m.flush(&members);
        let cover = flush.edge_cover(k, &fs).unwrap();
        assert_eq!(
            cover.within,
            vec![(m.read, edge)],
            "the edge between a tied member face and its neighbour lies within the member edge"
        );
        assert_eq!(
            flush.within(k),
            Some(&BTreeSet::from([(m.read, cover.within[0].1.clone())])),
            "Flush::of records it from the parents' member faces"
        );
    }

    /// **A member vertex its member ties is cited, and the tie goes
    /// downstream**: the finished vertex at either candidate sits at the
    /// member vertex, spelled by the tied name, and the two finished
    /// vertices that cite it are one tied row, not a vertex read as no
    /// member vertex and named by the lines through it.
    #[test]
    fn a_member_vertex_its_member_ties_is_cited_with_its_tie() {
        let mut m = SelfMember::new();
        let vs: Vec<_> = m.body.vertices().map(|(v, _)| v).collect();
        let (v0, v1, v2) = (vs[0], vs[1], vs[2]);
        let tied = m.tie(EntityKey::Vertex(v0), EntityKey::Vertex(v1));
        let members = m.members();
        let flush = m.flush(&members);
        let cited = |v| flush.member_vertices(v).unwrap().map(|m| (m.name, m.tied));
        let tied_ref = m
            .table
            .name_ref_of(&ent(0, EntityKey::Vertex(v0)))
            .unwrap()
            .clone();
        assert_eq!(*tied_ref, tied);
        let cited_tie = Some((entity_name(m.union, &(m.read, tied_ref.clone())), true));
        assert_eq!(cited(v0), cited_tie, "candidate 0");
        assert_eq!(cited(v1), cited_tie, "candidate 1");
        let own = m
            .table
            .name_ref_of(&ent(0, EntityKey::Vertex(v2)))
            .unwrap()
            .clone();
        assert_eq!(
            cited(v2),
            Some((entity_name(m.union, &(m.read, own)), false)),
            "an untied vertex"
        );

        let fold_name = |i: u64| StableName {
            kind: EntityKind::Vertex,
            node: m.union,
            path: vec![RoleSeg::Member {
                at: vec![(crate::VarId::new(9, 9), u32::try_from(i).unwrap())],
                of: NameRef::new(tied.clone()),
            }],
        };
        let mut t = NameTable::new();
        t.insert(fold_name(0), ent(0, EntityKey::Vertex(v0)))
            .unwrap();
        t.insert(fold_name(1), ent(0, EntityKey::Vertex(v1)))
            .unwrap();
        let lines = EdgeLines {
            member: BTreeMap::new(),
            seam: BTreeMap::new(),
        };
        let out = cite_member_edges(
            t,
            Vec::new(),
            &m.body,
            (&flush, &Fold::new(0, &m.body)),
            (&lines, &Links::default()),
        )
        .unwrap();
        let name = entity_name(m.union, &(m.read, tied_ref));
        match out.lookup(&name) {
            Some(Entry::Tied(es)) => assert_eq!(
                es.iter().map(|e| e.key).collect::<BTreeSet<_>>(),
                BTreeSet::from([EntityKey::Vertex(v0), EntityKey::Vertex(v1)]),
            ),
            other => panic!("the two candidates are one tied row: {other:?}"),
        }
        assert_eq!(out.len(), 1, "no row keeps the fold's spelling");
    }

    /// **A vertex two members hold, one tying its vertex and one not, is
    /// `Merged` of both and discriminated by the untied one**: a member's
    /// tie spells two vertices alike, but the copy's own names tell them
    /// apart, so each finished vertex is its own row, not one tied row.
    #[test]
    fn a_vertex_held_by_a_tied_and_an_untied_member_is_its_own_row() {
        let mut tied = SelfMember::new();
        let plain = SelfMember::new();
        let vs: Vec<_> = tied.body.vertices().map(|(v, _)| v).collect();
        let (v0, v1) = (vs[0], vs[1]);
        let tie = tied.tie(EntityKey::Vertex(v0), EntityKey::Vertex(v1));
        let copy = VarId::new(1, 8);
        let members = [
            Member {
                read: tied.read,
                body: &tied.body,
                table: &tied.table,
            },
            Member {
                read: copy,
                body: &tied.body,
                table: &plain.table,
            },
        ];
        let inc = Incidence::of(&tied.body);
        let face_ents = tied
            .body
            .faces()
            .map(|(f, _)| (f, BTreeSet::from([(0, f), (1, f)])))
            .collect();
        let flush = Flush {
            union: tied.union,
            body: &tied.body,
            members: &members,
            inc,
            face_ents,
            edges: BTreeMap::new(),
            spans: BTreeMap::new(),
            face_names: BTreeMap::new(),
            bnd: geom_core::Band::new(1e-9, 1e-6).unwrap(),
        };
        let held = |v| {
            let own = plain
                .table
                .name_ref_of(&ent(0, EntityKey::Vertex(v)))
                .unwrap();
            crate::names::merged::entity_set(
                EntityKind::Vertex,
                tied.union,
                [
                    entity_name(tied.union, &(tied.read, NameRef::new(tie.clone()))),
                    entity_name(tied.union, &(copy, own.clone())),
                ],
            )
        };
        assert_eq!(
            flush.member_vertices(v0).unwrap().map(|m| (m.name, m.tied)),
            Some((held(v0), true)),
            "candidate 0: both members' vertices, one of them tied"
        );
        let mut t = NameTable::new();
        for (i, v) in [v0, v1].into_iter().enumerate() {
            let fold = vertex(
                tied.union,
                vec![RoleSeg::Member {
                    at: vec![(crate::VarId::new(9, 9), u32::try_from(i).unwrap())],
                    of: NameRef::new(tie.clone()),
                }],
            );
            t.insert(fold, ent(0, EntityKey::Vertex(v))).unwrap();
        }
        let lines = EdgeLines {
            member: BTreeMap::new(),
            seam: BTreeMap::new(),
        };
        let out = cite_member_edges(
            t,
            Vec::new(),
            &tied.body,
            (&flush, &Fold::new(0, &tied.body)),
            (&lines, &Links::default()),
        )
        .unwrap();
        for v in [v0, v1] {
            assert_eq!(
                out.lookup(&held(v)),
                Some(&Entry::Unique(ent(0, EntityKey::Vertex(v)))),
                "{v:?} is its own row under the copy's discriminating name"
            );
        }
        assert_eq!(out.len(), 2, "no tied row");
    }

    /// **A member edge its member ties refuses where one edge's faces
    /// are needed**: the faces a vertex's lines bound are one pair per
    /// member edge, and a tie offers several, so the reading refuses
    /// typed instead of reading the edge as bounding no face.
    #[test]
    fn a_member_edge_its_member_ties_refuses_where_its_faces_are_read() {
        let mut m = SelfMember::new();
        let es: Vec<_> = m.body.edges().map(|(e, _)| e).collect();
        let (e0, e1) = (es[0], es[1]);
        let tied = m.tie(EntityKey::Edge(e0), EntityKey::Edge(e1));
        let members = m.members();
        let flush = m.flush(&members);
        let edge = (m.read, NameRef::new(tied.clone()));
        match flush.sides(&BTreeSet::from([edge.clone()])) {
            Err(NamingError::MemberEdgeTied { member, edge }) => {
                assert_eq!((member, *edge), (m.read, tied));
            }
            other => panic!("a tied member edge's faces refuse: {other:?}"),
        }
    }
}

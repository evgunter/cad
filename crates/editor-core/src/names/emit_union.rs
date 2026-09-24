//! **The n-ary union's naming** ([`crate::Node::Union`]; DM4).
//!
//! A union's names record WHICH MEMBER an entity came from and not
//! which fold step reached it. The fold's own tables are the pair
//! emitter's, so a member's entity accumulates a `FromA`/`FromB`
//! descent chain as long as its position in the list; that chain is
//! what this module takes out, in two halves.
//!
//! **Going in** ([`member_view`]): each member's table enters its fold
//! step already wrapped in one [`RoleSeg::FromMember`] carrying that
//! member's node id. Every operand of every step is therefore in the
//! union's own name space, and every name the pair emitter embeds — a
//! seam's two sides, a merge's constituents, a fragment's partners —
//! already says which member it came from. It has to be put there: a
//! pass-through op mints no name (N1), so N placements of one
//! prototype carry N IDENTICAL tables and no inner name can tell them
//! apart.
//!
//! **Coming out** ([`name_union`] for the published table,
//! [`collapse_table`] for the accumulation the declaration door
//! reads, and [`collapse_name`] for the refusal paths — three
//! consumers of one collapse, which the published table follows with
//! the two end passes below): a fold-table name is rewritten by descending its
//! `FromA`/`FromB` chain to the [`RoleSeg::FromMember`] at its foot —
//! one wrapper, whatever the depth. The names a segment embeds are
//! collapsed by the same rule, and what happens to their ORDER differs
//! by segment: a `Seam`'s two sides are put in name order (a union has
//! no A and B), and a junction's run of lines is re-sorted; a
//! `Merged` set is re-sorted and deduplicated; a `Fragment`'s `SideOf`
//! partners keep the order the pair emitter wrote them in, which is
//! fold-space name order and not re-established here
//! (`work/emit/name-ordered-positions-in-a-path-have-no-single-home.md`).
//!
//! Putting a `Seam`'s sides in name order can also rewrite a VALUE in
//! the tail. The pair emitter ranks every chain along a seam line along
//! that seam pair's `n_a × n_b`, which is oriented by side. That covers
//! the pieces of a seam minted already cut, the pieces of a whole seam a
//! later step cut, and a seam-vertex group ranked along a seam edge.
//! Which ranks lie on a seam line, and which pair's line, is ONE answer,
//! `names::seam_pair`, read by the pair emitter to pick the direction
//! and by this collapse to decide the rule. For each `OrderAlong` the
//! collapse finds that pair from the fold-space name, through any depth
//! of `FromA`/`FromB` wrapping, and reads the rank from the other end
//! (`of − 1 − rank`) exactly where the pair comes out swapped in name
//! order. See [`RankRule`].
//!
//! Two more rewrites happen at the END, on the published table only,
//! because they read the finished body. The pieces of each member EDGE
//! are numbered by the cells the finished body's vertices cut that edge
//! into, counting every cell whoever holds it ([`rank_member_edges`]);
//! and a seam vertex cites the member edge it lies on whole, not the
//! stretch of it the fold had cut when it met the vertex
//! ([`cite_member_edges`]). A rank the fold wrote is fold history — the
//! same name would denote different pieces in different member orders.
//! A refusal raised mid-fold, and the declaration door's view of an
//! intermediate step, have no finished body and keep the fold's ranks;
//! neither carries a member edge today (both name faces).
//!
//! # How an intermediate row is told from a member's row
//!
//! In a FOLD table, by the HEAD segment alone. Every row of every fold
//! step is minted under the union's id, so the id separates nothing;
//! what separates them is that a member-keyed row's head is
//! `FromMember` and an intermediate row's is `FromA`/`FromB`, which is
//! descended through.
//!
//! That is a statement about the fold's own tables and about nothing
//! else. Once a name is COLLAPSED the `FromA`/`FromB` chain is gone,
//! so a published row is told apart by its whole path and not by its
//! head — a member's row is a one-segment `FromMember` path, and
//! anything longer is a row the fold minted. That is the shape
//! `eval::wire`'s `sited_member` reads to say whether a refusal's row
//! is a member's entity, a merge of member entities, or a row no
//! declaration can name, and it is the reason the two questions are
//! asked in two places rather than shared.

use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::{Margin, Sign};

use crate::names::defer::TieRows;
use crate::names::discriminate::{Extent, ON_MEMBER_EDGE, band, order_along};
use crate::names::emit::{NamingError, check_total, edge_ends};
use crate::names::emit_topo::{
    OnSegment, edge_dir, insert_ranked_or_tied, place_on_segment, vertex_point,
};
use crate::names::role::{
    EntityKind, NameRef, Qualifier, RoleSeg, SegRewrite, StableName, never_in_a_boolean_table,
};
use crate::names::table::{EntityKey, Entry, NameTable};
use crate::node::RecipeNodeId;

/// One member's table as the UNION sees it: every row's name wrapped
/// in one [`RoleSeg::FromMember`] naming this member, minted under the
/// union's node.
///
/// The entities are untouched — same body index, same keys — so the
/// view is a name rewrite and the pair emitter's reverse lookups read
/// it exactly as they read the member's own table.
pub(crate) fn member_view(
    union: RecipeNodeId,
    member: RecipeNodeId,
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
        let keyed = keyed(union, member, name.clone(), name.kind);
        match entry {
            Entry::Unique(e) => view.insert(keyed, *e),
            Entry::Tied(es) => view.insert_tied(keyed, es.clone()),
        }?;
    }
    Ok(view)
}

/// **One entity of one member, in the union's own name space** — the
/// row [`member_view`] puts into that member's operand table, for a
/// caller that has the name rather than the table.
///
/// The declaration channel is that caller: a declared pair names
/// SITED entities (`SitedRef { at, name }`), and the union rewrites
/// each into this space before the shared resolver runs. Minting the
/// row here rather than there is what keeps the member-keying rule to
/// ONE definition — the view and the door cannot disagree about what
/// a member's entity is called.
pub(crate) fn member_name(
    union: RecipeNodeId,
    member: RecipeNodeId,
    name: &StableName,
) -> StableName {
    keyed(union, member, NameRef::new(name.clone()), name.kind)
}

/// The rule itself: one [`RoleSeg::FromMember`] segment, minted under
/// the union's node, carrying the entity's own kind.
fn keyed(
    union: RecipeNodeId,
    member: RecipeNodeId,
    of: NameRef,
    kind: crate::names::role::EntityKind,
) -> StableName {
    StableName {
        kind,
        node: union,
        path: vec![RoleSeg::FromMember { member, of }],
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
    tol: geom_core::Tol,
) -> Result<Arc<NameTable>, NamingError> {
    let bnd = band(tol)?;
    let t = collapse_table(node, folded)?;
    let t = rank_member_edges(t, body, members, bnd)?;
    let t = cite_member_edges(t, body, members, bnd)?;
    check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// One member of a union as [`name_union`] reads it: its node, its own
/// body and its own table — the place a member's edge is defined.
pub(crate) struct Member<'a, T: geom_core::Decide> {
    /// The member's node.
    pub node: RecipeNodeId,
    /// The member's body, in the space the union was built in.
    pub body: &'a topo::Body<T>,
    /// The member's own table (not the union's view of it).
    pub table: &'a NameTable,
}

/// **The pieces of one member's edge are ranked by where they lie on
/// it in the finished body, and a rank counts every piece of it.**
///
/// The fold ranks a member edge's pieces at whichever step cuts it
/// (`emit_topo`'s descent groups), and a later step cutting a piece
/// ranks ITS pieces under the first rank. Which step cuts where depends
/// on member order, and so does which pieces the member keeps where it
/// runs flush against another member (the coincident run is named for
/// whichever member the fold reached first). A rank written either way
/// is fold history, and the same name lands on different pieces in
/// different orders.
///
/// So the published table re-ranks against the GEOMETRY. Every row
/// `FromMember(m, e)` followed only by `Fragment(OrderAlong)` ranks, for
/// an EDGE `e`, is a piece of `m`'s edge `e`. The finished body's
/// vertices lying on `e` — decided through [`ON_MEMBER_EDGE`] against
/// `e`'s two ends in `m`'s own body — cut the closed segment into
/// cells, numbered from `e`'s start in `m`'s body ([`Cells`]). A piece
/// is one cell — or several, where a vertex of another shell sits inside
/// it ([`Cells::rank_of`]) — and publishes `FromMember(m, e)` followed by
/// one `Fragment(OrderAlong { rank, of })`: `rank` is its first cell's
/// index and
/// `of` counts every cell, whether `m` holds it, another member holds
/// it, or it lies inside another member and holds no edge at all. A
/// segment with no vertex inside it is one cell, and its piece carries
/// no rank. The cut points and the cell count are a function of the
/// finished body alone; which member was folded first cannot move
/// either.
///
/// Refused rather than guessed:
/// - a group holding a TIED row, or a member edge the member's own
///   table ties: [`NamingError::MemberEdgeTied`], a missing rule — there
///   is no one edge to cut;
/// - a piece that ends off the segment or starts and ends at one place,
///   or a member edge that names nothing in the member:
///   [`NamingError::Emission`], an invariant of the body or of the fold
///   found broken.
///
/// The rank rewrites the published table only. A refusal raised
/// mid-fold, and the declaration door's view of an intermediate step,
/// still carry the fold's ranks (`collapse_name`, `collapse_table`),
/// because the pieces are not final there.
fn rank_member_edges<T: geom_core::Decide>(
    t: NameTable,
    body: &topo::Body<T>,
    members: &[Member<'_, T>],
    bnd: geom_core::Band,
) -> Result<NameTable, NamingError> {
    use std::collections::BTreeMap;
    let bug = |what| NamingError::Emission { what };
    // (member, member edge) → the group's rows.
    type Key = (RecipeNodeId, StableName);
    let mut groups: BTreeMap<Key, Vec<(StableName, Entry)>> = BTreeMap::new();
    let mut out = NameTable::new();
    for (name, entry) in t.iter() {
        match member_edge_piece(name) {
            Some(key) => groups
                .entry(key)
                .or_default()
                .push((name.clone(), entry.clone())),
            None => put_entry(&mut out, name.clone(), entry.clone())?,
        }
    }
    for ((member, edge), rows) in groups {
        let tied = || NamingError::MemberEdgeTied {
            member,
            edge: Box::new(edge.clone()),
        };
        let (member_body, member_edge) = member_edge(members, member, &edge)?;
        let pieces = rows
            .iter()
            .map(|(_, entry)| match entry {
                Entry::Unique(e) => match e.key {
                    EntityKey::Edge(k) => Ok((*e, k)),
                    _ => Err(bug("a union's member-edge row names no edge")),
                },
                Entry::Tied(_) => Err(tied()),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let cells = Cells::of(body, member_body, member_edge, bnd)?;
        let head = StableName {
            kind: EntityKind::Edge,
            node: rows[0].0.node,
            path: vec![rows[0].0.path[0].clone()],
        };
        for (e, k) in pieces {
            let name = match cells.count()? {
                1 => head.clone(),
                of => {
                    let mut name = head.clone();
                    name.path.push(RoleSeg::Fragment(Qualifier::OrderAlong {
                        rank: cells.rank_of(body, k)?,
                        of,
                    }));
                    name
                }
            };
            out.insert(name, e)?;
        }
    }
    Ok(out)
}

/// One row into a table, through the door its entry's shape takes.
fn put_entry(t: &mut NameTable, name: StableName, entry: Entry) -> Result<(), NamingError> {
    match entry {
        Entry::Unique(e) => t.insert(name, e),
        Entry::Tied(es) => t.insert_tied(name, es),
    }?;
    Ok(())
}

/// Member `member`'s edge `edge`, as its own body holds it: the body and
/// the key the member's own table names uniquely. A tie there is
/// [`NamingError::MemberEdgeTied`]; a member or a name the union does
/// not have is an emission bug, since every member-keyed row came out
/// of that member's own table ([`member_view`]).
fn member_edge<'a, T: geom_core::Decide>(
    members: &'a [Member<'a, T>],
    member: RecipeNodeId,
    edge: &StableName,
) -> Result<(&'a topo::Body<T>, topo::EdgeKey), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let m = members
        .iter()
        .find(|m| m.node == member)
        .ok_or_else(|| bug("a union's row is keyed by a node that is not one of its members"))?;
    match m.table.lookup(edge) {
        Some(Entry::Unique(e)) => match e.key {
            EntityKey::Edge(k) => Ok((m.body, k)),
            _ => Err(bug("a member's edge name names no edge in the member")),
        },
        Some(Entry::Tied(_)) => Err(NamingError::MemberEdgeTied {
            member,
            edge: Box::new(edge.clone()),
        }),
        None => Err(bug(
            "a union's member-keyed row names nothing in its member",
        )),
    }
}

/// **The cells a member edge is cut into in the finished body**: the
/// places on its closed segment where a vertex of the body sits, in
/// order from the edge's start, the two ends always among them.
///
/// A place is a set of vertices, not one: a union's body can hold
/// several shells, and members that only touch meet at vertices of
/// different shells sitting at one point. An end no vertex sits on is
/// still a place, with no vertices — the stretch between it and the
/// nearest vertex is a cell too.
struct Cells {
    places: Vec<Vec<topo::VertexKey>>,
}

impl Cells {
    fn of<T: geom_core::Decide>(
        body: &topo::Body<T>,
        member_body: &topo::Body<T>,
        member_edge: topo::EdgeKey,
        bnd: geom_core::Band,
    ) -> Result<Self, NamingError> {
        let (r0, r1) = edge_ends(member_body, member_edge)?;
        let (q0, q1) = (
            vertex_point(member_body, r0)?,
            vertex_point(member_body, r1)?,
        );
        let d = q1 - q0;
        let len = d.norm();
        let (mut start, mut end) = (Vec::new(), Vec::new());
        // The interior places, each with its vertices and its position
        // along the edge; a vertex joins the place it is `Zero` from.
        let mut inside: Vec<(Vec<topo::VertexKey>, T)> = Vec::new();
        for (v, _) in body.vertices() {
            let p = vertex_point(body, v)?;
            match place_on_segment(p, (q0, q1), ON_MEMBER_EDGE, bnd)? {
                OnSegment::Off => {}
                OnSegment::AtStart => start.push(v),
                OnSegment::AtEnd => end.push(v),
                OnSegment::Inside => {
                    let t = (p - q0).dot(d) / len;
                    let mut joined = false;
                    for (vs, at) in &mut inside {
                        let sign =
                            decide(ON_MEMBER_EDGE, Margin::of(t - *at), bnd).map_err(|source| {
                                NamingError::Escalated {
                                    predicate: ON_MEMBER_EDGE,
                                    source,
                                }
                            })?;
                        if sign == Sign::Zero {
                            vs.push(v);
                            joined = true;
                            break;
                        }
                    }
                    if !joined {
                        inside.push((vec![v], t));
                    }
                }
            }
        }
        let along: Vec<_> = inside
            .iter()
            .map(|&(_, t)| Extent { min: t, max: t })
            .collect();
        let ranks = order_along(&along, bnd)?.ok_or(NamingError::Emission {
            what: "two places on a member edge the order along it does not separate",
        })?;
        let mut places = vec![Vec::new(); inside.len() + 2];
        places[0] = start;
        for ((vs, _), rank) in inside.into_iter().zip(ranks) {
            places[rank as usize + 1] = vs;
        }
        *places.last_mut().ok_or(NamingError::Emission {
            what: "a member edge with no end",
        })? = end;
        Ok(Self { places })
    }

    /// How many cells: one more than the places inside the segment.
    fn count(&self) -> Result<u32, NamingError> {
        u32::try_from(self.places.len() - 1).map_err(|_| NamingError::Emission {
            what: "a member edge cut into more cells than a rank can count",
        })
    }

    /// The rank of piece `k` of the finished body: the index of the
    /// first cell it covers, from its nearer end's place.
    ///
    /// A piece usually covers one cell. One that has a vertex of ANOTHER
    /// shell of the body sitting inside it covers more, and is still
    /// named by its first: two pieces of one member edge are disjoint, so
    /// no two start at one place.
    fn rank_of<T: geom_core::Decide>(
        &self,
        body: &topo::Body<T>,
        k: topo::EdgeKey,
    ) -> Result<u32, NamingError> {
        let bug = |what| NamingError::Emission { what };
        let (v0, v1) = edge_ends(body, k)?;
        let at = |v| {
            self.places
                .iter()
                .position(|p| p.contains(&v))
                .ok_or_else(|| bug("a piece of a member edge ends off that edge"))
        };
        let (i, j) = (at(v0)?, at(v1)?);
        if i == j {
            return Err(bug("a piece of a member edge starts and ends at one place"));
        }
        u32::try_from(i.min(j))
            .map_err(|_| bug("a member edge cut into more cells than a rank can count"))
    }
}

/// The (member, member edge) a published row is a piece of: an EDGE
/// row `FromMember(m, e)` with `e` an edge, followed by nothing but
/// `Fragment(OrderAlong)` ranks.
fn member_edge_piece(name: &StableName) -> Option<(RecipeNodeId, StableName)> {
    if name.kind != EntityKind::Edge {
        return None;
    }
    let (RoleSeg::FromMember { member, of }, tail) = name.path.split_first()? else {
        return None;
    };
    if of.kind != EntityKind::Edge
        || !tail
            .iter()
            .all(|s| matches!(s, RoleSeg::Fragment(Qualifier::OrderAlong { .. })))
    {
        return None;
    }
    Some((*member, (**of).clone()))
}

/// **A name embedded in another cites a member edge whole, never one of
/// its pieces.**
///
/// A seam vertex names the entities that cross at it: at the fold step
/// where a face crossed a member edge, that edge was the operand's edge
/// as far as the fold had cut it, `FromMember(m, e)` followed by the
/// fold's ranks. That stretch is fold history — cut in one order, whole
/// in another, and in general more than one of the published cells
/// [`rank_member_edges`] numbers, so no published rank denotes it. What
/// the vertex lies on, in every order, is `m`'s edge `e`. So every
/// embedded name `FromMember(m, e)` followed by ranks is rewritten to
/// `FromMember(m, e)`, at any depth, through the one path walk
/// ([`StableName::rewrite_path`]). An embedded member edge the fold had
/// not cut already reads that way, so the rewrite makes the ranked
/// spelling agree with it.
///
/// Dropping the stretch can make two vertex names one, where a curved
/// face crosses one member edge twice and the fold met the two crossings
/// on different stretches. So a VERTEX whose name moved also drops the
/// fold's own rank on it (ranked within that stretch), and the vertices
/// that come out with one name are ranked along the member edge they
/// cite ([`insert_ranked_or_tied`], the fold's own ranker), over the
/// finished body. A vertex citing TWO member edges has no one carrier,
/// and several of them under one name refuse, as `collapse` refuses the
/// fold's own rank on that shape. A moved name that is not a vertex's,
/// or that collides with a name that did not move, refuses at insert.
///
/// Stripping a rank cannot reorder a seam's two sides: they differ at
/// their first segment — two pieces of one member edge do not cross —
/// and the rank is after it. That is checked, not assumed.
fn cite_member_edges<T: geom_core::Decide>(
    t: NameTable,
    body: &topo::Body<T>,
    members: &[Member<'_, T>],
    bnd: geom_core::Band,
) -> Result<NameTable, NamingError> {
    use std::collections::BTreeMap;
    let bug = |what| NamingError::Emission { what };
    let mut out = NameTable::new();
    let mut vertices: BTreeMap<StableName, Vec<(StableName, Entry)>> = BTreeMap::new();
    for (name, entry) in t.iter() {
        let cited = name
            .clone()
            .rewrite_path(&mut WholeMemberEdges { union: name.node })?;
        if cited == *name {
            put_entry(&mut out, cited, entry.clone())?;
            continue;
        }
        if !seams_in_name_order(&cited) {
            return Err(bug(SEAM_REORDERED));
        }
        if cited.kind != EntityKind::Vertex {
            put_entry(&mut out, cited, entry.clone())?;
            continue;
        }
        let mut base = cited;
        while matches!(
            base.path.last(),
            Some(RoleSeg::Fragment(Qualifier::OrderAlong { .. }))
        ) && base.path.len() > 1
        {
            base.path.pop();
        }
        vertices
            .entry(base)
            .or_default()
            .push((name.clone(), entry.clone()));
    }
    let mut tie = TieRows::default();
    for (base, rows) in vertices {
        if let [(_, entry)] = rows.as_slice() {
            put_entry(&mut out, base, entry.clone())?;
            continue;
        }
        let (member, edge) = match base.path.as_slice() {
            [RoleSeg::Seam { a, b }] => match (cited_edge(a), cited_edge(b)) {
                (Some(m), None) | (None, Some(m)) => m,
                _ => return Err(bug(SIDED_VERTEX_RANK)),
            },
            _ => return Err(bug(SIDED_VERTEX_RANK)),
        };
        let (member_body, member_edge) = member_edge(members, member, &edge)?;
        let dir = edge_dir(member_body, member_edge)?;
        let mut keys = Vec::with_capacity(rows.len());
        let mut extents = Vec::with_capacity(rows.len());
        for (_, entry) in rows {
            let Entry::Unique(e) = entry else {
                return Err(NamingError::MemberEdgeTied {
                    member,
                    edge: Box::new(edge),
                });
            };
            let EntityKey::Vertex(v) = e.key else {
                return Err(bug("a union's vertex row names no vertex"));
            };
            let p = vertex_point(body, v)?;
            let along = geom_core::Vec3::new(p.x, p.y, p.z).dot(dir);
            keys.push(e);
            extents.push(Extent {
                min: along,
                max: along,
            });
        }
        insert_ranked_or_tied(&mut out, &mut tie, false, base, &keys, &extents, bnd, |e| {
            *e
        })?;
    }
    tie.flush(&mut out)?;
    Ok(out)
}

/// A seam's sides came out of [`cite_member_edges`] out of name order.
const SEAM_REORDERED: &str = "citing a member edge whole reordered the two sides of a union's seam";

/// The (member, edge) an embedded name cites whole: `FromMember(m, e)`
/// alone, `e` an edge.
fn cited_edge(n: &StableName) -> Option<(RecipeNodeId, StableName)> {
    match n.path.as_slice() {
        [RoleSeg::FromMember { member, of }] if of.kind == EntityKind::Edge => {
            Some((*member, (**of).clone()))
        }
        _ => None,
    }
}

/// Whether every `Seam` segment of `name` has its sides in name order,
/// the order `collapse` put them in.
fn seams_in_name_order(name: &StableName) -> bool {
    name.path.iter().all(|s| match s {
        RoleSeg::Seam { a, b } => **a <= **b,
        _ => true,
    })
}

/// The [`SegRewrite`] of [`cite_member_edges`]: an embedded ranked piece
/// of a member edge becomes the member edge, and any other embedded
/// name in the union's space is walked for one.
///
/// Only names the UNION minted are this rewrite's: the name a
/// [`RoleSeg::FromMember`] carries is the member's own, published by
/// the member — a nested union's ranks are that union's final ones — so
/// the walk neither rewrites nor enters a name another node minted.
struct WholeMemberEdges {
    union: RecipeNodeId,
}

impl SegRewrite for WholeMemberEdges {
    type Error = NamingError;

    fn name(&mut self, n: &StableName) -> Result<Option<StableName>, NamingError> {
        if n.node != self.union {
            return Ok(None);
        }
        if member_edge_piece(n).is_some() {
            return Ok((n.path.len() > 1).then(|| StableName {
                kind: n.kind,
                node: n.node,
                path: vec![n.path[0].clone()],
            }));
        }
        let walked = n.clone().rewrite_path(self)?;
        Ok((walked != *n).then_some(walked))
    }
}

/// A whole fold table in the union's published space — the collapse
/// [`name_union`] publishes, without its end passes over member edges
/// and without the totality check.
///
/// Two callers, and the split is what tells them apart. [`name_union`]
/// rewrites the LAST step's table, which names a finished body and is
/// held to totality. The declaration door rewrites an INTERMEDIATE
/// step's, and for a different purpose: a declared pair is written
/// against what this node's refusals name (`collapse_name`), so the
/// door that resolves one has to read the accumulation in that same
/// space. The entities are untouched — same keys, same ties — so the
/// keys a lookup returns are the accumulation's own.
pub(crate) fn collapse_table(
    node: RecipeNodeId,
    folded: &NameTable,
) -> Result<NameTable, NamingError> {
    let mut t = NameTable::new();
    for (name, entry) in folded.iter() {
        let keyed = collapse(node, name)?;
        match entry {
            Entry::Unique(e) => t.insert(keyed, *e),
            Entry::Tied(es) => t.insert_tied(keyed, es.clone()),
        }?;
    }
    Ok(t)
}

/// One fold-table name in the union's published space.
///
/// The collapse [`name_union`] applies to a whole table (before its end
/// passes over member edges, which need the finished body), exposed
/// for the paths that carry a name out of a step that did NOT finish:
/// a refusal raised at step `k` reads the ACCUMULATED table, whose
/// rows are still `FromA`/`FromB`-headed, and a name in that shape is
/// in the fold's internal space — no published table holds it and
/// nothing can resolve it. Every name a union's refusal carries goes
/// through here first, so what a caller is handed denotes in the space
/// this node's own names live in.
pub(crate) fn collapse_name(
    node: RecipeNodeId,
    name: &StableName,
) -> Result<StableName, NamingError> {
    collapse(node, name)
}

/// The emission bug this module can raise: a fold table carrying a
/// segment the pair emitter does not mint.
const FOREIGN: &str = "a union fold's table carries a segment the boolean emitter does not mint";

/// Two lines of one seam junction collapsed to the same member-space
/// line: the member-keying rewrite was not one-to-one on the lines.
const JUNCTION_LINES_COLLIDE: &str =
    "two lines of a union's seam junction collapse to one member-space line";

use super::merged::NESTED_MERGED;
use super::seam_pair::seam_line_pair;

/// One fold-table name, keyed by member.
///
/// Returns a name in the UNION's own space (`node` is the union's, as
/// every fold row's already is): either one [`RoleSeg::FromMember`],
/// [`RoleSeg::Seam`], [`RoleSeg::Merged`] or [`RoleSeg::OutputBody`]
/// head followed by the `Fragment` discriminators the fold
/// accumulated, outermost step last — or, for a seam JUNCTION vertex,
/// a sorted run of two or more `Seam` lines and nothing else.
///
/// The tail keeps its segments, but not always their values: an
/// `OrderAlong` rank is rewritten by the name's [`RankRule`].
fn collapse(node: RecipeNodeId, name: &StableName) -> Result<StableName, NamingError> {
    let bug = |what| NamingError::Emission { what };
    if name.node != node {
        return Err(bug(
            "a union fold's table carries a row minted by another node",
        ));
    }
    let Some((head, mut tail)) = name.path.split_first() else {
        return Err(bug("a union fold's table carries a name with no role"));
    };
    let mut path = match head {
        // Already member-keyed: the foot of a descent chain, put there
        // by `member_view` before the step ran.
        RoleSeg::FromMember { .. } => vec![head.clone()],
        // The accumulated body's own name at every step, and the
        // union's at the last one: one body out, one output-body row.
        RoleSeg::OutputBody => vec![RoleSeg::OutputBody],
        // The descent. Every operand of every step is in this node's
        // space, so a `FromA`/`FromB` argument is always an earlier
        // step's row: descended THROUGH, carrying its own
        // discriminators out with it.
        RoleSeg::FromA(inner) | RoleSeg::FromB(inner) => collapse(node, inner)?.path,
        // A seam: one line, collapsed by [`seam_line`], with any
        // `Fragment` tail after it.
        //
        // Or a seam JUNCTION: the pair emitter names the VERTEX where
        // k ≥ 2 seam lines meet by the sorted run of those lines'
        // `Seam` segments and nothing after them, so a run is admitted
        // in exactly that shape — a vertex, the whole path — and any
        // other run (an edge's, or one followed by a discriminator) is
        // a shape the pair emitter does not mint. Each line is
        // collapsed, then the run re-sorted: the pair emitter sorted
        // it in the fold's space, and a name stable under member
        // reordering is ordered by the member-space lines.
        //
        // The run is NOT deduplicated, unlike a `Merged` set. The pair
        // emitter deduplicates the lines before it mints, so the run
        // holds k DISTINCT fold-space lines; two collapsing to one
        // would make the rewrite many-to-one on lines, and dropping one
        // would publish a name for a vertex with fewer lines than it
        // has. A `Merged` set is a set of FACES whose name is the set
        // (N3), so there a repeat is the same constituent reached
        // twice, and two merges that collapse to one set collide at
        // insert instead.
        RoleSeg::Seam { a, b }
            if tail
                .first()
                .is_some_and(|s| matches!(s, RoleSeg::Seam { .. })) =>
        {
            if name.kind != EntityKind::Vertex {
                return Err(bug(FOREIGN));
            }
            let mut lines = vec![seam_line(node, a, b)?];
            for seg in std::mem::take(&mut tail) {
                let RoleSeg::Seam { a, b } = seg else {
                    return Err(bug(FOREIGN));
                };
                lines.push(seam_line(node, a, b)?);
            }
            lines.sort_unstable();
            if lines.windows(2).any(|w| w[0] == w[1]) {
                return Err(bug(JUNCTION_LINES_COLLIDE));
            }
            lines
        }
        RoleSeg::Seam { a, b } => {
            vec![seam_line(node, a, b)?]
        }
        // An F7 merged face: its constituents are result-face names in
        // the minting node's space (N3), so they stay in this union's
        // space, each collapsed by this same rule.
        //
        // The pair emitter mints `Merged` for a DECLARED contact's
        // merge groups, and a union carries a declaration channel of
        // its own (`Node::Union`'s `declare` input), so a step whose
        // bucket holds a coincident pair produces these rows and a
        // union's published table carries them.
        //
        // The constituent set is FLAT (N3): a constituent is never
        // itself a bare merged face. The mint (`emit_topo`'s
        // merge-group loop) holds that at the first door; this is the
        // same rule read at the union's second door — a constituent
        // that collapses to a bare merged face is refused as the
        // emission bug it is, never flattened. A fragment of a merged
        // face is a fragment, not a merge (`RoleSeg::Merged`'s doc).
        //
        // The sort-and-dedup makes the constituent SET the name, the
        // same choice the pair emitter's twin makes (`emit_topo.rs`,
        // review R8): two merge groups collapsing to ONE constituent
        // set collide LOUDLY at insert (`DuplicateName` → typed
        // `NamingError`), never silently aliasing two faces onto one
        // name.
        RoleSeg::Merged(constituents) => {
            let mut set = Vec::with_capacity(constituents.len());
            for c in constituents {
                let c = collapse(node, c)?;
                if matches!(c.path.as_slice(), [RoleSeg::Merged(_)]) {
                    return Err(bug(NESTED_MERGED));
                }
                set.push(c);
            }
            set.sort();
            set.dedup();
            vec![RoleSeg::Merged(set)]
        }
        // A `Fragment` is a TAIL segment — it discriminates a head,
        // it is never one — and everything after it is a segment the
        // boolean emitter does not mint at all, so a fold table
        // carrying either is an emission bug. The long half is
        // [`never_in_a_boolean_table`], which is where a new
        // `RoleSeg` is classified; this match still stops the
        // compiler if one is added and not classified there.
        RoleSeg::Fragment(_) | never_in_a_boolean_table!() => return Err(bug(FOREIGN)),
    };
    for seg in tail {
        path.push(match seg {
            // The discriminator's partner names are OPERAND-space
            // names (N2) — this node's space, like every other embedded
            // name here — so they collapse the same way. The verdicts
            // are untouched: they are the recorded predicate evidence,
            // not a reference.
            RoleSeg::Fragment(Qualifier::SideOf(vec)) => {
                let partners = vec
                    .iter()
                    .map(|(n, v)| Ok((collapse(node, n)?, *v)))
                    .collect::<Result<Vec<_>, NamingError>>()?;
                RoleSeg::Fragment(Qualifier::SideOf(partners))
            }
            // The rank is a place along a direction; the head's rule says
            // whether canonicalizing the head moved that direction.
            RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }) => {
                RoleSeg::Fragment(RankRule::of(node, name)?.apply(*rank, *of)?)
            }
            // Only a `Fragment` follows the head in a boolean table;
            // anything else in the tail is an emission bug — the head
            // segments above included, which are heads and not
            // discriminators. A junction's run of `Seam`s never reaches
            // this loop — the head arm takes the whole path or refuses
            // it — so a `Seam` here follows a `FromMember`, `Merged` or
            // `OutputBody` head, or a `Fragment`, and the pair emitter
            // mints none of those shapes. The long half is
            // [`never_in_a_boolean_table`], for the reason the head
            // match names.
            RoleSeg::OutputBody
            | RoleSeg::FromA(_)
            | RoleSeg::FromB(_)
            | RoleSeg::FromMember { .. }
            | RoleSeg::Seam { .. }
            | RoleSeg::Merged(_)
            | never_in_a_boolean_table!() => return Err(bug(FOREIGN)),
        });
    }
    Ok(StableName {
        kind: name.kind,
        node,
        path,
    })
}

/// What the collapse does to a name's `Fragment(OrderAlong)` rank. A
/// rank is a place along a direction, so this depends on whether
/// putting the pair of the line it lies on in canonical order reversed
/// that direction ([`RankRule::of`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RankRule {
    /// The rank lies along a direction the collapse does not change:
    /// an edge on no seam line (it ranks along its own carrier), an edge
    /// on a seam line whose pair stays in minted order, a seam vertex
    /// group whose edge parent is such an edge, a group with no edge
    /// parent, and every face.
    Keep,
    /// An edge on a seam line whose pair canonicalization swapped, or a
    /// seam vertex group whose edge parent is one. The pair emitter
    /// ranks along `n_a × n_b`, with the minted `a` face's outward
    /// normal first. With the sides swapped that line is negated, and
    /// the rank reads from the other end: `of − 1 − rank`.
    Reverse,
    /// A seam VERTEX group whose two parents are both edges. The pair
    /// emitter ranks it along the A side's edge, so which carrier was
    /// used depends on operand order and there is no canonical one.
    /// Two straight edges cross at most once, so such a group needs
    /// curved edges, and none is known. It refuses rather than publish
    /// a rank a member reorder could rebind.
    Refuse,
}

/// A seam vertex group ranked along a carrier the pair emitter chose by
/// side, where either side's edge could have served.
const SIDED_VERTEX_RANK: &str =
    "a union's seam vertex group is ranked along one of two edge parents, chosen by operand side";

impl RankRule {
    /// The rule for fold-space name `name`'s `OrderAlong` rank.
    ///
    /// An EDGE's rank lies along the seam line [`seam_line_pair`] finds
    /// for it — the edge's own pair, or the pair of the seam it is a
    /// piece of, through any `FromA`/`FromB` wrapping — which is the
    /// line the pair emitter ranked it along (`emit_topo`'s
    /// `seam_line_dir`, the same answer). A seam VERTEX group's rank
    /// lies along its edge parent's line. What the collapse changes is
    /// only the pair's order, so the rule is `Reverse` exactly where
    /// that line's pair comes out swapped.
    fn of(node: RecipeNodeId, name: &StableName) -> Result<Self, NamingError> {
        match (name.kind, name.path.first()) {
            (EntityKind::Edge, _) => Self::of_line(node, name),
            (EntityKind::Vertex, Some(RoleSeg::Seam { a, b })) => {
                match (a.kind == EntityKind::Edge, b.kind == EntityKind::Edge) {
                    (true, true) => Ok(Self::Refuse),
                    (true, false) => Self::of_line(node, a),
                    (false, true) => Self::of_line(node, b),
                    (false, false) => Ok(Self::Keep),
                }
            }
            (EntityKind::Vertex, _) | (EntityKind::Face | EntityKind::Body, _) => Ok(Self::Keep),
        }
    }

    /// The rule for a rank along EDGE `edge`'s line.
    fn of_line(node: RecipeNodeId, edge: &StableName) -> Result<Self, NamingError> {
        Ok(match seam_line_pair(edge) {
            None => Self::Keep,
            Some((a, b)) if collapse(node, a)? > collapse(node, b)? => Self::Reverse,
            Some(_) => Self::Keep,
        })
    }

    /// The tail's `OrderAlong` rank, rewritten against the canonical head.
    fn apply(self, rank: u32, of: u32) -> Result<Qualifier, NamingError> {
        let bug = |what| NamingError::Emission { what };
        match self {
            Self::Keep => Ok(Qualifier::OrderAlong { rank, of }),
            Self::Reverse => of
                .checked_sub(1)
                .and_then(|last| last.checked_sub(rank))
                .map(|rank| Qualifier::OrderAlong { rank, of })
                .ok_or_else(|| bug(RANK_OUTSIDE_COUNT)),
            Self::Refuse => Err(bug(SIDED_VERTEX_RANK)),
        }
    }
}

/// A seam chain's rank at or beyond its count.
const RANK_OUTSIDE_COUNT: &str = "a seam chain's rank lies outside its count";

/// One seam line between two members, in the union's space.
///
/// The pair emitter's `a`/`b` are the crossing entities in the two
/// OPERANDS' tables, which are this node's space on both sides, so
/// each is collapsed the same way every other row is. The pair is
/// then CANONICALIZED by name order: a union is commutative, so
/// "which side" would record only which of the two members the fold
/// reached first, which is the position this node exists not to
/// record. A rank the pair emitter oriented by side follows the swap
/// through [`RankRule`].
fn seam_line(node: RecipeNodeId, a: &StableName, b: &StableName) -> Result<RoleSeg, NamingError> {
    let (x, y) = (collapse(node, a)?, collapse(node, b)?);
    let (a, b) = if x <= y { (x, y) } else { (y, x) };
    Ok(RoleSeg::Seam {
        a: NameRef::new(a),
        b: NameRef::new(b),
    })
}

#[cfg(test)]
mod tests {
    //! The rewrite's own rows: a merged face collapses to its flat
    //! member-space set, and a NESTED merged face — a shape the pair
    //! emitter's flat mint never produces — refuses as an emission bug
    //! instead of being flattened here. A seam junction's run of lines
    //! collapses to the sorted set of its member-space lines, and a
    //! `Seam` outside that leading run is still foreign.
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::{CapEnd, EntityKind};

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
            vec![RoleSeg::FromMember {
                member: RecipeNodeId(m),
                of: face(RecipeNodeId(m), vec![RoleSeg::Cap(CapEnd::Start)]).into(),
            }],
        )
    }

    fn from_a(union: RecipeNodeId, inner: StableName) -> StableName {
        face(union, vec![RoleSeg::FromA(inner.into())])
    }

    fn from_b(union: RecipeNodeId, inner: StableName) -> StableName {
        face(union, vec![RoleSeg::FromB(inner.into())])
    }

    #[test]
    fn a_flat_merged_face_collapses_to_its_member_space_set() {
        let union = RecipeNodeId(9);
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
    fn a_nested_merged_face_refuses_as_an_emission_bug() {
        let union = RecipeNodeId(9);
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
        let err = collapse(union, &nested).unwrap_err();
        assert!(
            matches!(err, NamingError::Emission { what } if what == NESTED_MERGED),
            "{err:?}"
        );
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
        let union = RecipeNodeId(9);
        // Two lines of one junction, as the pair emitter spells them at
        // a step whose B operand is member 1: each line's A side is an
        // accumulation row (members 3 and 2, reached through the
        // descent) and its B side is member 1's face. In the fold's
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
    fn a_seam_after_a_fragment_is_foreign() {
        let union = RecipeNodeId(9);
        let line = || seam(from_a(union, member_cap(union, 2)), member_cap(union, 1));
        let name = vertex(
            union,
            vec![
                line(),
                RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }),
                line(),
            ],
        );
        let err = collapse(union, &name).unwrap_err();
        assert!(
            matches!(err, NamingError::Emission { what } if what == FOREIGN),
            "{err:?}"
        );
    }

    fn foreign(name: &StableName) -> bool {
        let union = name.node;
        matches!(collapse(union, name), Err(NamingError::Emission { what }) if what == FOREIGN)
    }

    #[test]
    fn a_run_of_seams_is_admitted_only_as_a_whole_vertex_path() {
        let union = RecipeNodeId(9);
        let lines = || {
            vec![
                seam(from_a(union, member_cap(union, 3)), member_cap(union, 1)),
                seam(from_b(union, member_cap(union, 2)), member_cap(union, 1)),
            ]
        };
        // The junction shape itself is admitted.
        assert!(collapse(union, &vertex(union, lines())).is_ok());
        // An EDGE named by a run: the pair emitter names a seam edge by
        // one line.
        let edge = StableName {
            kind: EntityKind::Edge,
            node: union,
            path: lines(),
        };
        assert!(foreign(&edge), "{:?}", collapse(union, &edge));
        // A run followed by a discriminator: a junction is unique per
        // line set, so the pair emitter never ranks one.
        let mut path = lines();
        path.push(RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }));
        let tailed = vertex(union, path);
        assert!(foreign(&tailed), "{:?}", collapse(union, &tailed));
    }

    /// Member `m`'s first lateral edge, as `member_view` keys it.
    fn member_edge(union: RecipeNodeId, m: u64) -> StableName {
        StableName {
            kind: EntityKind::Edge,
            node: union,
            path: vec![RoleSeg::FromMember {
                member: RecipeNodeId(m),
                of: StableName {
                    kind: EntityKind::Edge,
                    node: RecipeNodeId(m),
                    path: vec![RoleSeg::LateralEdge(crate::names::role::ProfileVertexRef {
                        loop_index: 0,
                        vertex: 0,
                    })],
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
            path: vec![RoleSeg::FromA(inner.into())],
        }
    }

    fn ranked(
        kind: EntityKind,
        union: RecipeNodeId,
        line: RoleSeg,
        rank: u32,
        of: u32,
    ) -> StableName {
        StableName {
            kind,
            node: union,
            path: vec![line, RoleSeg::Fragment(Qualifier::OrderAlong { rank, of })],
        }
    }

    fn rank_of(name: &StableName) -> u32 {
        match name.path.as_slice() {
            [
                RoleSeg::Seam { .. },
                RoleSeg::Fragment(Qualifier::OrderAlong { rank, .. }),
            ] => *rank,
            other => panic!("not a ranked seam name: {other:?}"),
        }
    }

    /// **A seam edge whose pair canonicalization swaps reads its rank
    /// from the other end.** The emitter's A side here is member 5's
    /// face and its B side member 2's; in name order member 2 comes
    /// first, so the chain's line is reversed and rank 0 of 3 becomes 2.
    /// In minted order the same rank stands.
    #[test]
    fn a_swapped_seam_edge_reads_its_rank_from_the_other_end() {
        let union = RecipeNodeId(9);
        let swapped = seam(through_a(member_cap(union, 5)), member_cap(union, 2));
        let kept = seam(through_a(member_cap(union, 2)), member_cap(union, 5));
        for (line, want) in [(swapped, 2), (kept, 0)] {
            let out = collapse(union, &ranked(EntityKind::Edge, union, line, 0, 3)).unwrap();
            assert_eq!(rank_of(&out), want, "{out:?}");
            assert!(
                matches!(&out.path[0], RoleSeg::Seam { a, .. } if **a == member_cap(union, 2)),
                "the pair is not in name order: {out:?}"
            );
        }
    }

    /// **A seam vertex group ranked along an edge parent keeps its rank
    /// through the same swap.** Its carrier is member 5's edge, the same
    /// edge on either side, so the swap does not reorient it.
    #[test]
    fn a_swapped_edge_and_face_seam_vertex_keeps_its_rank() {
        let union = RecipeNodeId(9);
        let line = seam(through_a(member_edge(union, 5)), member_cap(union, 2));
        let out = collapse(union, &ranked(EntityKind::Vertex, union, line, 0, 2)).unwrap();
        assert!(
            matches!(&out.path[0], RoleSeg::Seam { a, .. } if **a == member_cap(union, 2)),
            "the pair did not swap, so this row proves nothing: {out:?}"
        );
        assert_eq!(rank_of(&out), 0, "{out:?}");
    }

    /// **A seam vertex group ranked with an edge on each side refuses**:
    /// its carrier was chosen by side, so no rank survives a reorder.
    #[test]
    fn a_ranked_seam_vertex_between_two_edges_refuses() {
        let union = RecipeNodeId(9);
        let line = seam(through_a(member_edge(union, 5)), member_edge(union, 2));
        let err = collapse(union, &ranked(EntityKind::Vertex, union, line, 1, 2)).unwrap_err();
        assert!(
            matches!(err, NamingError::Emission { what } if what == SIDED_VERTEX_RANK),
            "{err:?}"
        );
    }

    /// A seam EDGE between member 5's and member 2's caps, as the emitter
    /// minted it: `swap` puts member 5 on the A side, which name order
    /// reverses.
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

    /// **A later step's pieces of a seam edge take the seam's rule.** The
    /// pair emitter ranks the pieces of a cut seam along the seam's own
    /// pair line, so where name order reverses that pair, the
    /// descent's rank reads from the other end too.
    #[test]
    fn a_descent_through_a_swapped_seam_edge_reads_its_rank_from_the_other_end() {
        let union = RecipeNodeId(9);
        for (swap, want) in [(true, 1), (false, 0)] {
            let piece = StableName {
                kind: EntityKind::Edge,
                node: union,
                path: vec![
                    RoleSeg::FromA(seam_edge(union, swap).into()),
                    RoleSeg::Fragment(Qualifier::OrderAlong { rank: 0, of: 2 }),
                ],
            };
            let out = collapse(union, &piece).unwrap();
            assert_eq!(rank_of(&out), want, "swap={swap}: {out:?}");
        }
    }

    /// **A seam vertex group ranked along a seam edge takes that edge's
    /// rule**, not its own pair's: the group lies on the edge's line.
    #[test]
    fn a_seam_vertex_on_a_swapped_seam_edge_reads_its_rank_from_the_other_end() {
        let union = RecipeNodeId(9);
        for (swap, want) in [(true, 1), (false, 0)] {
            let line = seam(through_a(seam_edge(union, swap)), member_cap(union, 7));
            let out = collapse(union, &ranked(EntityKind::Vertex, union, line, 0, 2)).unwrap();
            assert_eq!(rank_of(&out), want, "swap={swap}: {out:?}");
        }
    }

    /// **A rank at or past its count refuses rather than wrapping** when
    /// it has to be read from the other end.
    #[test]
    fn a_reversed_rank_outside_its_count_refuses() {
        let union = RecipeNodeId(9);
        let line = seam(through_a(member_cap(union, 5)), member_cap(union, 2));
        for (rank, of) in [(2, 2), (0, 0)] {
            let err = collapse(
                union,
                &ranked(EntityKind::Edge, union, line.clone(), rank, of),
            )
            .unwrap_err();
            assert!(
                matches!(err, NamingError::Emission { what } if what == RANK_OUTSIDE_COUNT),
                "rank {rank} of {of}: {err:?}"
            );
        }
    }

    #[test]
    fn two_junction_lines_collapsing_to_one_refuse() {
        let union = RecipeNodeId(9);
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
            matches!(err, NamingError::Emission { what } if what == JUNCTION_LINES_COLLIDE),
            "{err:?}"
        );
    }

    /// A body holding two edges, and their keys: the arena is the only
    /// source of an `EdgeKey`.
    fn two_edge_body() -> (topo::Body<f64>, topo::EdgeKey, topo::EdgeKey) {
        let mut body = topo::Body::<f64>::new();
        let mut mint = |x: f64| {
            let born = body
                .mvfs(geom_core::Point3::new(x, 0.0, 0.0))
                .expect("mvfs births a lone vertex");
            body.mev_line(
                topo::MevSite::Lone {
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

    /// **A tie where the member-edge ranker needs one edge refuses as a
    /// missing rule**, in both places a tie can stand: the member's own
    /// table ties the edge's name, or the union's table ties two of its
    /// pieces under one name. Neither is guessed past with the fold's
    /// ranks, which are order-dependent. No probe document reaches either
    /// (0 tied rows over the review corpus and its fixtures), so the
    /// refusal is pinned here, on the ranker itself.
    #[test]
    fn a_tied_member_edge_or_piece_refuses_as_a_missing_rule() {
        let (union, m) = (RecipeNodeId(9), RecipeNodeId(4));
        let (body, k0, k1) = two_edge_body();
        let edge = StableName {
            kind: EntityKind::Edge,
            node: m,
            path: vec![RoleSeg::LateralEdge(crate::names::role::ProfileVertexRef {
                loop_index: 0,
                vertex: 0,
            })],
        };
        let piece = |rank| {
            let mut n = member_name(union, m, &edge);
            n.path
                .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 }));
            n
        };
        let bnd = geom_core::Band::new(1e-9, 1e-6).unwrap();
        // The member ties its edge's name; the union's row is unique.
        let mut tied_member = NameTable::new();
        tied_member
            .insert_tied(edge.clone(), vec![edge_ref(k0), edge_ref(k1)])
            .unwrap();
        let mut t = NameTable::new();
        t.insert(piece(0), edge_ref(k0)).unwrap();
        // The member names its edge once; the union ties two pieces.
        let mut unique_member = NameTable::new();
        unique_member.insert(edge.clone(), edge_ref(k0)).unwrap();
        let mut tied_rows = NameTable::new();
        tied_rows
            .insert_tied(piece(0), vec![edge_ref(k0), edge_ref(k1)])
            .unwrap();
        for (label, member_table, table) in [
            ("member ties the edge", &tied_member, t),
            ("union ties two pieces", &unique_member, tied_rows),
        ] {
            let members = [Member {
                node: m,
                body: &body,
                table: member_table,
            }];
            let err = rank_member_edges(table, &body, &members, bnd).unwrap_err();
            assert!(
                matches!(&err, NamingError::MemberEdgeTied { member, edge: e }
                    if *member == m && **e == edge),
                "{label}: {err:?}"
            );
        }
    }
}

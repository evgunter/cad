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
//! collapsed by the same rule. The result is then put in canonical
//! form by `names::canonical`, the one list of a path's name-ordered
//! positions, which the pair emitter's mint and every rewrite of a
//! published name also end in: a `Merged` set and a `SideOf` vector
//! are sorted, a junction's run of lines is sorted, and a `Seam`'s two
//! sides are put in name order, because a union has no A and B.
//!
//! Putting a `Seam`'s sides in name order can also rewrite a VALUE in
//! the tail. The pair emitter ranks every chain along a seam line along
//! that seam pair's `n_a × n_b`, which is oriented by side. That covers
//! the pieces of a seam minted already cut, the pieces of a whole seam a
//! later step cut, and a seam-vertex group ranked along a seam edge.
//! Which ranks lie on a seam line, and which pair's line, is ONE answer,
//! `names::seam_pair`, read by the pair emitter to pick the direction
//! and by the canonical form (`names::canonical`) to decide what the
//! ordering does. The line is found in the fold-space name, through any
//! depth of `FromA`/`FromB` wrapping, and in the collapsed one, and the
//! rank reads from the other end (`of − 1 − rank`) exactly where the
//! pair comes out swapped in name order.
//!
//! More rewrites happen at the END, on the published table only,
//! because they read the finished table or the finished body. Each
//! replaces something the fold wrote down about WHEN it met an entity
//! with something the finished union says about WHAT the entity is:
//! - a face is named for its PARENT, the member faces the union's merges
//!   link to it, transitively ([`Parents`]): the parent itself when the
//!   finished body holds it as one face, and otherwise the parent and one
//!   `SideOf` against the parents across its seams; a seam edge is named
//!   for the two parents it lies between, ranked along the seam when
//!   there are several; and any other name cites a face as its parent
//!   ([`name_by_parents`]) — whether a step cut a face before or after
//!   it merged, and in how many steps, is fold history;
//! - an entity of the finished body that belongs to several members at
//!   once (a flush stretch, a corner on another member's rim) is named
//!   for the least member entity that holds it, and every edge lying
//!   along a member edge is named as a piece of it ([`Flush`]) — which
//!   member was operand A is fold history;
//! - the pieces of each member EDGE are numbered by the cells the
//!   finished body's vertices cut that edge into, counting every cell
//!   whoever holds it ([`rank_member_edges`]) — which step cut the edge
//!   is fold history;
//! - a vertex is named for the member vertex it sits at, or for the
//!   member edge it lies on and the one face crossing it there, and
//!   otherwise cites the member edge it lies on whole, not the stretch
//!   of it the fold had cut when it met the vertex
//!   ([`cite_member_edges`]).
//!
//! These names are functions of the finished body, so they are
//! order-free as far as the boolean's output is: where different member
//! orders leave different vertices (a declared merge can,
//! `work/zip/a-declared-merge-leaves-a-collinear-valence-two-vertex-an-earlier-cut-made.md`),
//! the names differ with them.
//!
//! A refusal raised mid-fold, and the declaration door's view of an
//! intermediate step, have no finished body and keep the fold's ranks.
//! Neither may carry a fold-ranked member-edge piece: the declaration
//! door admits no edge (`DeclareUnsupportedPair`), and a refusal that
//! would carry one refuses as an emission bug instead
//! ([`is_fold_ranked_member_edge`]).
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

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom_core::k_stats::decide;
use geom_core::{Margin, Sign};

use crate::names::defer::{TieRows, mint_candidates, put};
use crate::names::discriminate::{Extent, ON_MEMBER_EDGE, band, order_along, side_of_face};
use crate::names::emit::{
    Incidence, NamingError, check_total, edge_ends, ent, face_half_edges, rims_between, to_u32,
    vertex_point,
};
use crate::names::emit_topo::{OnSegment, Segment, edge_extent, face_plane, insert_ranked_or_tied};
use crate::names::groups::GroupRecord;
use crate::names::role::{
    EntityKind, NameRef, Qualifier, RoleSeg, SegRewrite, SideVerdict, StableName,
    never_in_a_boolean_table,
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
        put_entry(
            &mut view,
            keyed(union, member, name.clone(), name.kind),
            entry,
        )?;
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
) -> Result<(Arc<NameTable>, GroupRecord), NamingError> {
    let bnd = band(tol)?;
    let t = collapse_table(node, folded)?;
    let parents = Parents::of(node, &t, members)?;
    let flush = Flush::of(node, body, members, &parents, bnd)?;
    let (t, held, groups) = name_by_parents(node, &t, body, &parents, &flush, bnd)?;
    let t = rank_member_edges(t, held, body, members, &flush, bnd)?;
    let t = cite_member_edges(t, body, members, &flush, bnd)?;
    check_total(&t, body, 0)?;
    Ok((Arc::new(t), groups))
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

/// **The pieces of each member edge, numbered by the cells the finished
/// body cuts it into.**
///
/// A row `FromMember(m, e)` followed only by `Fragment(OrderAlong)`, `e`
/// an edge, is a piece of `m`'s edge `e`; so is any edge row that lies
/// along a member edge whatever the fold named it (a merged edge the
/// fold met as a seam). Either is a piece of the LEAST member edge it
/// lies within ([`Flush`]), which is `e` unless `e` runs flush with a
/// lesser member's edge there. The places on `e`'s closed
/// segment (in `m`'s own body) where a vertex of the finished body sits
/// cut it into cells, numbered from `e`'s start ([`Cells`]). A piece
/// publishes `FromMember(m, e)` followed by `OrderAlong { rank, of }`:
/// `rank` is the first cell it covers and `of` counts every cell,
/// whether `m`, another member or no member holds it. One cell, no rank.
/// The fold's ranks are replaced, not refined: they record which step
/// cut the edge and which member kept a flush stretch, both of which
/// depend on member order. These depend only on the finished body, so they are
/// order-free as far as the boolean's output is.
///
/// Refuses [`NamingError::MemberEdgeTied`] for a tie where one edge is
/// needed, and [`NamingError::Emission`] for a piece that is not a
/// stretch of its edge between two places.
fn rank_member_edges<T: geom_core::Decide>(
    t: NameTable,
    held: Vec<(StableName, Entry)>,
    body: &topo::Body<T>,
    members: &[Member<'_, T>],
    flush: &Flush<'_, T>,
    bnd: geom_core::Band,
) -> Result<NameTable, NamingError> {
    use std::collections::BTreeMap;
    let bug = |what| NamingError::Emission { what };
    // (member, member edge) → the group's rows.
    let mut groups: BTreeMap<MemberEntity, Vec<(StableName, Entry)>> = BTreeMap::new();
    let mut out = NameTable::new();
    let rows = t
        .iter()
        .map(|(name, entry)| (name.clone(), entry.clone()))
        .chain(held);
    for (name, entry) in rows {
        let k = match &entry {
            Entry::Unique(e) => match e.key {
                EntityKey::Edge(k) => Some(k),
                _ => None,
            },
            Entry::Tied(_) => None,
        };
        let key = match (member_edge_piece(&name), k) {
            (Some((member, edge, _)), Some(k)) => Some(flush.least_edge(k, (member, edge))),
            (Some((member, edge, _)), None) => Some((member, edge)),
            (None, Some(k)) => flush.least_within(k),
            (None, None) => None,
        };
        match key {
            Some(key) => groups.entry(key).or_default().push((name, entry)),
            None => put_entry(&mut out, name, &entry)?,
        }
    }
    for ((member, edge), rows) in groups {
        let (member_body, member_edge) = member_edge(members, member, edge.name())?;
        let pieces = rows
            .iter()
            .map(|(_, entry)| match entry {
                Entry::Unique(e) => match e.key {
                    EntityKey::Edge(k) => Ok((*e, k)),
                    _ => Err(bug("a union's member-edge row names no edge")),
                },
                Entry::Tied(_) => Err(NamingError::MemberEdgeTied {
                    member,
                    edge: Box::new(edge.name().clone()),
                }),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let cells = Cells::of(body, &Segment::of_edge(member_body, member_edge)?, bnd)?;
        let head = entity_name(flush.union, &(member, edge));
        for (e, k) in pieces {
            let mut name = head.clone();
            let of = cells.count()?;
            if of > 1 {
                let rank = cells.rank_of(body, k)?;
                name.path
                    .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }));
            }
            out.insert(name, e)?;
        }
    }
    Ok(out)
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
/// names it: a tie there is [`NamingError::MemberEdgeTied`], and a
/// member or name the union does not have is an emission bug (every
/// member-keyed row came from that member's table).
fn member_edge<'a, T: geom_core::Decide>(
    members: &'a [Member<'a, T>],
    member: RecipeNodeId,
    edge: &StableName,
) -> Result<(&'a topo::Body<T>, topo::EdgeKey), NamingError> {
    let bug = |what| NamingError::Emission { what };
    let m = member_of(members, member)?;
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

/// The member of a union whose node is `node`; every member-keyed row
/// came from a member, so another node is an emission bug.
fn member_of<'a, T: geom_core::Decide>(
    members: &'a [Member<'a, T>],
    node: RecipeNodeId,
) -> Result<&'a Member<'a, T>, NamingError> {
    members
        .iter()
        .find(|m| m.node == node)
        .ok_or(NamingError::Emission {
            what: "a union's row is keyed by a node that is not one of its members",
        })
}

/// **The places on a member edge's segment where a vertex of the
/// finished body sits**, in order from its start, both ends always among
/// them (an end no vertex sits on is a place with none).
///
/// A place can hold several vertices: members that only touch leave one
/// in each shell at one point. Vertices are one place when a chain of
/// `Zero` gaps joins them, the ends' vertices included: the connected
/// components of that relation, which no visiting order changes (`Zero`
/// is not transitive, so a greedy first match would). That needs a band
/// with K ≥ 2, or a chain of two coincidences could span a definite
/// separation; a narrower band refuses ([`NamingError::NarrowBand`]).
///
/// The places are the finished body's, so the cells are order-free as
/// far as the boolean's output is.
struct Cells {
    places: Vec<Vec<topo::VertexKey>>,
}

impl Cells {
    fn of<T: geom_core::Decide>(
        body: &topo::Body<T>,
        seg: &Segment<T>,
        bnd: geom_core::Band,
    ) -> Result<Self, NamingError> {
        let bug = |what| NamingError::Emission { what };
        if bnd.escalate() < 2.0 * bnd.zero() {
            return Err(NamingError::NarrowBand {
                zero: bnd.zero(),
                escalate: bnd.escalate(),
            });
        }
        // Every vertex on the segment, with where it lies and how far
        // along.
        let mut on = Vec::new();
        for (v, _) in body.vertices() {
            let p = vertex_point(body, v)?;
            match seg.place(p, ON_MEMBER_EDGE, bnd)? {
                OnSegment::Off => {}
                at => on.push((v, at, seg.along(p))),
            }
        }
        // Components of the `Zero` relation, by union–find.
        fn find(root: &mut [usize], mut i: usize) -> usize {
            while root[i] != i {
                root[i] = root[root[i]];
                i = root[i];
            }
            i
        }
        let mut root: Vec<usize> = (0..on.len()).collect();
        for i in 0..on.len() {
            for j in (i + 1)..on.len() {
                let gap = decide(ON_MEMBER_EDGE, Margin::of(on[j].2 - on[i].2), bnd).map_err(
                    |source| NamingError::Escalated {
                        predicate: ON_MEMBER_EDGE,
                        source,
                    },
                )?;
                if gap == Sign::Zero {
                    let (a, b) = (find(&mut root, i), find(&mut root, j));
                    root[a.max(b)] = a.min(b);
                }
            }
        }
        // (vertices, extent along, holds the start, holds the end)
        type Place<T> = (Vec<topo::VertexKey>, Extent<T>, bool, bool);
        let mut components: Vec<Place<T>> = Vec::new();
        let mut slot = vec![usize::MAX; on.len()];
        for (i, &(v, at, t)) in on.iter().enumerate() {
            let r = find(&mut root, i);
            if slot[r] == usize::MAX {
                slot[r] = components.len();
                components.push((Vec::new(), Extent { min: t, max: t }, false, false));
            }
            let (vs, x, start, end) = &mut components[slot[r]];
            vs.push(v);
            x.min = x.min.min(t);
            x.max = x.max.max(t);
            *start |= at == OnSegment::AtStart;
            *end |= at == OnSegment::AtEnd;
        }
        let (mut first, mut last, mut inside) = (None, None, Vec::new());
        for (vs, x, start, end) in components {
            let end_place = match (start, end) {
                (true, true) => return Err(bug("one place on a member edge is at both its ends")),
                (true, false) => &mut first,
                (false, true) => &mut last,
                (false, false) => {
                    inside.push((vs, x));
                    continue;
                }
            };
            if end_place.replace(vs).is_some() {
                return Err(bug("two places at one end of a member edge"));
            }
        }
        let along: Vec<_> = inside
            .iter()
            .map(|(_, x)| Extent {
                min: x.min,
                max: x.max,
            })
            .collect();
        let ranks = order_along(&along, bnd)?.ok_or_else(|| {
            bug("two places on a member edge the order along it does not separate")
        })?;
        let mut places = vec![Vec::new(); inside.len() + 2];
        for ((vs, _), rank) in inside.into_iter().zip(ranks) {
            places[rank as usize + 1] = vs;
        }
        places[0] = first.unwrap_or_default();
        *places
            .last_mut()
            .ok_or_else(|| bug("a member edge with no end"))? = last.unwrap_or_default();
        Ok(Self { places })
    }

    /// How many cells: one fewer than the places.
    fn count(&self) -> Result<u32, NamingError> {
        u32::try_from(self.places.len() - 1).map_err(|_| NamingError::Emission {
            what: "a member edge cut into more cells than a rank can count",
        })
    }

    /// The first cell piece `k` covers: the index of its nearer end's
    /// place. A piece covers more than one cell only where another
    /// shell's vertex sits inside it; pieces of one edge are disjoint,
    /// so no two start at one place.
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

/// A member's entity: the member, and the entity's name in that
/// member's own table. Ordered as the union's [`RoleSeg::FromMember`]
/// names for them are, member first.
type MemberEntity = (RecipeNodeId, NameRef);

/// The union's name for member entity `(member, of)`, sharing the
/// member's handle for `of` so its order cache comes along.
fn entity_name(union: RecipeNodeId, (member, of): &MemberEntity) -> StableName {
    keyed(union, *member, of.clone(), of.kind)
}

/// **An entity of the finished body that belongs to several members
/// at once is named for the least of them.**
///
/// Where two members run flush, one stretch of the finished body's
/// boundary is a stretch of both: a member edge lying along another
/// member's edge, a member's corner sitting on another's rim or corner.
/// The fold keeps whichever operand was A at the step that met them, so
/// its name for the stretch says which member was folded first. The
/// union has no A and B; it names the stretch for the least member
/// entity that holds it, in the order of the [`RoleSeg::FromMember`]
/// names it publishes.
///
/// "Holds" is read off the finished body and the members' own bodies,
/// never off the fold:
/// - a finished EDGE lies within member `n`'s edge `e` when its two
///   faces descend from `e`'s two faces in `n` (their names cite them,
///   through a `Merged` set or a `Fragment`) and both its ends lie on
///   `e`'s closed segment ([`ON_MEMBER_EDGE`]);
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
    /// Finished face → the member faces its name descends from.
    faces: BTreeMap<topo::FaceKey, BTreeSet<MemberEntity>>,
    /// Finished edge → the member edges it lies within.
    edges: BTreeMap<topo::EdgeKey, BTreeSet<MemberEntity>>,
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
        let mut faces = BTreeMap::new();
        let mut face_names = BTreeMap::new();
        for (&f, name) in &parents.of_face {
            let parent = parents.all.get(name).ok_or(NamingError::Emission {
                what: "a finished face of a union has no parent",
            })?;
            faces.insert(f, parent.from.clone());
            face_names.insert(f, name.clone());
        }
        let mut flush = Flush {
            union,
            body,
            members,
            inc: Incidence::of(body)?,
            faces,
            edges: BTreeMap::new(),
            face_names,
            bnd,
        };
        let mut edges = BTreeMap::new();
        for (&k, fs) in &flush.inc.edge_faces {
            let within = flush.edge_within(k, fs)?;
            if !within.is_empty() {
                edges.insert(k, within);
            }
        }
        flush.edges = edges;
        Ok(flush)
    }

    fn member(&self, node: RecipeNodeId) -> Result<&'a Member<'a, T>, NamingError> {
        member_of(self.members, node)
    }

    /// The member edges finished edge `k`, between faces `fs`, lies
    /// within.
    fn edge_within(
        &self,
        k: topo::EdgeKey,
        fs: &[topo::FaceKey],
    ) -> Result<BTreeSet<MemberEntity>, NamingError> {
        let mut out = BTreeSet::new();
        let [f0, f1] = fs else { return Ok(out) };
        let (Some(from0), Some(from1)) = (self.faces.get(f0), self.faces.get(f1)) else {
            return Ok(out);
        };
        if !straight(self.body, k) {
            return Ok(out);
        }
        let (c0, c1) = edge_ends(self.body, k)?;
        let ends = [vertex_point(self.body, c0)?, vertex_point(self.body, c1)?];
        for (n, g0) in from0 {
            for (_, g1) in from1.iter().filter(|(n1, g1)| n1 == n && g1 != g0) {
                let m = self.member(*n)?;
                let (Some(a), Some(b)) =
                    (face_key(m.table, g0.name()), face_key(m.table, g1.name()))
                else {
                    continue;
                };
                for r in rims_between(m.body, a, b)? {
                    if !straight(m.body, r) {
                        continue;
                    }
                    let seg = Segment::of_edge(m.body, r)?;
                    let mut on = true;
                    for p in ends {
                        on &= seg.place(p, ON_MEMBER_EDGE, self.bnd)? != OnSegment::Off;
                    }
                    if let (true, Some(name)) = (on, unique_name(m.table, EntityKey::Edge(r))) {
                        out.insert((*n, name));
                    }
                }
            }
        }
        Ok(out)
    }

    /// The member edge a piece of `own` that is finished edge `k` is
    /// named for: the least member edge `k` lies within, `own` among them.
    fn least_edge(&self, k: topo::EdgeKey, own: MemberEntity) -> MemberEntity {
        match self.edges.get(&k).and_then(|w| w.first()) {
            Some(least) if *least < own => least.clone(),
            _ => own,
        }
    }

    /// The least member edge finished edge `k` lies within, if any.
    fn least_within(&self, k: topo::EdgeKey) -> Option<MemberEntity> {
        self.edges.get(&k).and_then(|w| w.first()).cloned()
    }

    /// **Finished vertex `v` where one member edge is crossed by one
    /// face**, named `Seam { edge, face }` in name order: the edge is the
    /// least member edge the finished edges at `v` lie within, and it is
    /// one line — every finished edge at `v` that lies within a member
    /// edge names the same least one; the face is the one face at `v`
    /// that descends from none of the faces of the member edges along
    /// that line, cited without its `Fragment`s. Anything else is `None`.
    fn crossing(&self, v: topo::VertexKey) -> Result<Option<StableName>, NamingError> {
        let at = self.inc.vertex_edges.get(&v).map_or(&[][..], Vec::as_slice);
        let mut lines = BTreeSet::new();
        let mut along = BTreeSet::new();
        for k in at {
            if let Some(w) = self.edges.get(k) {
                lines.extend(w.first().cloned());
                along.extend(w.iter().cloned());
            }
        }
        let mut lines = lines.into_iter();
        let (Some((member, edge)), None) = (lines.next(), lines.next()) else {
            return Ok(None);
        };
        let mut sides = BTreeSet::new();
        for (n, name) in &along {
            let m = self.member(*n)?;
            let Some(Entry::Unique(e)) = m.table.lookup(name.name()) else {
                continue;
            };
            let EntityKey::Edge(r) = e.key else { continue };
            for f in edge_faces(m.body, r)? {
                sides.extend(unique_name(m.table, EntityKey::Face(f)).map(|f| (*n, f)));
            }
        }
        let mut others = BTreeSet::new();
        for k in at {
            for f in self.inc.edge_faces.get(k).into_iter().flatten() {
                if self
                    .faces
                    .get(f)
                    .is_some_and(|from| !from.is_disjoint(&sides))
                {
                    continue;
                }
                let Some(name) = self.face_names.get(f) else {
                    return Ok(None);
                };
                others.insert(name.clone());
            }
        }
        let mut others = others.into_iter();
        let (Some(face), None) = (others.next(), others.next()) else {
            return Ok(None);
        };
        let edge = entity_name(self.union, &(member, edge));
        let (a, b) = if edge < face {
            (edge, face)
        } else {
            (face, edge)
        };
        Ok(Some(StableName {
            kind: EntityKind::Vertex,
            node: self.union,
            path: vec![RoleSeg::Seam {
                a: NameRef::new(a),
                b: NameRef::new(b),
            }],
        }))
    }

    /// The member edge a name of finished vertex `v` cites whole, where
    /// its name cites `own`: the least member edge that a finished edge
    /// at `v` lying within `own` lies within too.
    fn least_at(&self, v: topo::VertexKey, own: MemberEntity) -> MemberEntity {
        let mut least = own.clone();
        for k in self.inc.vertex_edges.get(&v).into_iter().flatten() {
            if let Some(w) = self.edges.get(k).filter(|w| w.contains(&own))
                && let Some(first) = w.first()
                && *first < least
            {
                least = first.clone();
            }
        }
        least
    }

    /// The least member vertex finished vertex `v` sits at, if any: a
    /// vertex `w` of member `n` at `v`'s point that lies on a face of `n`
    /// a face around `v` descends from.
    fn least_vertex(&self, v: topo::VertexKey) -> Result<Option<MemberEntity>, NamingError> {
        let mut cited = BTreeSet::new();
        for k in self.inc.vertex_edges.get(&v).into_iter().flatten() {
            for f in self.inc.edge_faces.get(k).into_iter().flatten() {
                cited.extend(self.faces.get(f).into_iter().flatten());
            }
        }
        let p = vertex_point(self.body, v)?;
        let mut seen = BTreeSet::new();
        let mut least: Option<MemberEntity> = None;
        for (n, face) in cited {
            let m = self.member(*n)?;
            let Some(f) = face_key(m.table, face.name()) else {
                continue;
            };
            for he in face_half_edges(m.body, f)? {
                let w = m
                    .body
                    .get_half_edge(he)
                    .ok_or(NamingError::Emission {
                        what: "a member face's half-edge is dangling",
                    })?
                    .start;
                if !seen.insert((*n, w)) {
                    continue;
                }
                let gap = Margin::of((vertex_point(m.body, w)? - p).norm());
                let at = decide(ON_MEMBER_EDGE, gap, self.bnd).map_err(|source| {
                    NamingError::Escalated {
                        predicate: ON_MEMBER_EDGE,
                        source,
                    }
                })?;
                if at != Sign::Zero {
                    continue;
                }
                if let Some(name) = unique_name(m.table, EntityKey::Vertex(w)) {
                    let c = (*n, name);
                    if least.as_ref().is_none_or(|l| c < *l) {
                        least = Some(c);
                    }
                }
            }
        }
        Ok(least)
    }
}

/// The member faces a face name of the union descends from: its own
/// member face, or every constituent's of a `Merged` set, through any
/// `Fragment` after either.
fn member_faces(name: &StableName, out: &mut BTreeSet<MemberEntity>) {
    match name.path.first() {
        Some(RoleSeg::FromMember { member, of }) if of.kind == EntityKind::Face => {
            out.insert((*member, of.clone()));
        }
        Some(RoleSeg::Merged(set)) => {
            for c in set {
                member_faces(c, out);
            }
        }
        _ => {}
    }
}

/// Whether edge `e` of `body` is carried by a line: a tag read, no
/// number consulted. The flush tests are straight-segment tests, and a
/// curved edge's chord is not its locus (a sphere's two meridians share
/// both ends and both faces).
fn straight<T: geom_core::Decide>(body: &topo::Body<T>, e: topo::EdgeKey) -> bool {
    topo::query::edge_carrier_kind(body, e) == Some(topo::query::CurveKind::Line)
}

/// The two faces edge `e` of `body` lies between.
fn edge_faces<T: geom_core::Decide>(
    body: &topo::Body<T>,
    e: topo::EdgeKey,
) -> Result<[topo::FaceKey; 2], NamingError> {
    let bug = || NamingError::Emission {
        what: "a member edge without two faces",
    };
    let edge = body.get_edge(e).ok_or_else(bug)?;
    let face = |he| {
        body.get_half_edge(he)
            .and_then(|h| body.get_loop(h.parent_loop))
            .map(|l| l.face)
            .ok_or_else(bug)
    };
    Ok([face(edge.he_plus)?, face(edge.he_minus)?])
}

/// The face a member's table names `name`, when it names one face.
fn face_key(table: &NameTable, name: &StableName) -> Option<topo::FaceKey> {
    match table.lookup(name)? {
        Entry::Unique(e) => match e.key {
            EntityKey::Face(f) => Some(f),
            _ => None,
        },
        Entry::Tied(_) => None,
    }
}

/// The name a member's table gives its entity `key`, unless it is tied.
fn unique_name(table: &NameTable, key: EntityKey) -> Option<NameRef> {
    let name = table.name_ref_of(&ent(0, key))?;
    (!table.is_tied(name.name())).then(|| name.clone())
}

/// The (member, member edge) an EDGE name of this shape is about —
/// `FromMember(m, e)`, `e` an edge, then nothing but
/// `Fragment(OrderAlong)` ranks — and whether any rank follows.
fn member_edge_piece(name: &StableName) -> Option<(RecipeNodeId, NameRef, bool)> {
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
    Some((*member, of.clone(), !tail.is_empty()))
}

/// **A vertex is named for what it sits on in the finished body, and a
/// name embedded in another cites a member edge whole.**
///
/// A vertex that sits at a member vertex is that member vertex, the
/// least if several members have one there ([`Flush::least_vertex`]).
/// One that lies along one member edge, crossed there by one face, is
/// `Seam { edge, face }` ([`Flush::crossing`]), however the fold met it
/// — as a seam of the edge's faces, or as a junction of seam lines.
///
/// Any other seam vertex embeds the member edge it lies on as far as the fold
/// had cut it when it met the vertex — a stretch that spans several of
/// [`rank_member_edges`]' cells, so no published rank denotes it. It is
/// cited as `FromMember(m, e)`, which is how a vertex already cites an
/// edge the fold had not cut, and as the least member edge through the
/// vertex along that line ([`Flush::least_at`]). The rewrite is
/// [`StableName::rewrite_path`], which ends in the canonical form
/// (`names::canonical::rewritten`).
///
/// A vertex's own trailing rank is fold history too once its name moved,
/// so vertices are grouped by their name without it. A group of several
/// whose name is one seam citing one member edge is ranked WHOLE along
/// that edge, oriented as it is in the member's own body
/// ([`insert_ranked_or_tied`]), whether or not any of its names moved:
/// the direction is then the member's, in every member order, and not
/// the carrier the fold happened to rank along at the step that met the
/// group. A seam citing two member edges refuses, as the collapse
/// already does for such a group's ranks. A group with a moved name and
/// no single seam, or a seam citing no member edge, has no carrier and
/// refuses. A group with no moved name and no member edge to rank along
/// keeps its names: its ranks, if any, lie along a seam line, in the
/// orientation the collapse put in canonical form.
fn cite_member_edges<T: geom_core::Decide>(
    t: NameTable,
    body: &topo::Body<T>,
    members: &[Member<'_, T>],
    flush: &Flush<'_, T>,
    bnd: geom_core::Band,
) -> Result<NameTable, NamingError> {
    use std::collections::BTreeMap;
    let bug = |what| NamingError::Emission { what };
    let mut out = NameTable::new();
    // base → (name as it stands, entry, whether the rewrite moved it)
    let mut vertices: BTreeMap<StableName, Vec<(StableName, Entry, bool)>> = BTreeMap::new();
    for (name, entry) in t.iter() {
        // The vertex a row names, when it names one vertex.
        let at = match entry {
            Entry::Unique(e) => match e.key {
                EntityKey::Vertex(v) => Some(v),
                _ => None,
            },
            Entry::Tied(_) => None,
        };
        let member_vertex = match at {
            Some(v) => flush.least_vertex(v)?,
            None => None,
        };
        let crossing = match (&member_vertex, at) {
            (None, Some(v)) => flush.crossing(v)?,
            _ => None,
        };
        let cited = match (member_vertex, crossing) {
            (Some(vertex), _) => entity_name(name.node, &vertex),
            (None, Some(crossing)) => crossing,
            (None, None) => name.clone().rewrite_path(&mut WholeMemberEdges {
                union: name.node,
                at,
                flush,
            })?,
        };
        if cited.kind != EntityKind::Vertex {
            put_entry(&mut out, cited, entry)?;
            continue;
        }
        let moved = cited != *name;
        let base = without_tail(&cited, |q| matches!(q, Qualifier::OrderAlong { .. }));
        vertices
            .entry(base)
            .or_default()
            .push((cited, entry.clone(), moved));
    }
    let mut tie = TieRows::default();
    for (base, rows) in vertices {
        let moved = rows.iter().any(|&(_, _, moved)| moved);
        if let [(name, entry, _)] = rows.as_slice() {
            put_entry(&mut out, if moved { base } else { name.clone() }, entry)?;
            continue;
        }
        let whole = |n: &StableName| member_edge_piece(n).filter(|(_, _, ranked)| !ranked);
        let carrier = match base.path.as_slice() {
            [RoleSeg::Seam { a, b }] => match (whole(a), whole(b)) {
                (Some(m), None) | (None, Some(m)) => Some(m),
                (Some(_), Some(_)) => return Err(bug(Unrankable::SidedVertexRank.what())),
                (None, None) if moved => return Err(bug(CITED_GROUP_NO_MEMBER_EDGE)),
                (None, None) => None,
            },
            _ if moved => return Err(bug(CITED_GROUP_NOT_ONE_SEAM)),
            _ => None,
        };
        let Some((member, edge, _)) = carrier else {
            for (name, entry, _) in rows {
                put_entry(&mut out, name, &entry)?;
            }
            continue;
        };
        let (member_body, member_edge) = member_edge(members, member, &edge)?;
        let seg = Segment::of_edge(member_body, member_edge)?;
        let mut keys = Vec::with_capacity(rows.len());
        let mut extents = Vec::with_capacity(rows.len());
        for (_, entry, _) in rows {
            let Entry::Unique(e) = entry else {
                return Err(NamingError::MemberEdgeTied {
                    member,
                    edge: Box::new(edge.name().clone()),
                });
            };
            let EntityKey::Vertex(v) = e.key else {
                return Err(bug("a union's vertex row names no vertex"));
            };
            let along = seg.along(vertex_point(body, v)?);
            keys.push(e);
            extents.push(Extent {
                min: along,
                max: along,
            });
        }
        insert_ranked_or_tied(
            &mut out,
            &mut tie,
            false,
            &base,
            &keys,
            &extents,
            bnd,
            |e| *e,
        )?;
    }
    tie.flush(&mut out)?;
    Ok(out)
}

/// Several vertices share one name once they cite member edges whole,
/// and that name is not a single seam line (a junction's run, say), so
/// nothing says what to rank them along.
const CITED_GROUP_NOT_ONE_SEAM: &str = "several vertices of a union share one name once they cite \
     member edges whole, and that name is not a single seam to rank them along";

/// The same, where the name is one seam but neither side is a member
/// edge cited whole.
const CITED_GROUP_NO_MEMBER_EDGE: &str = "several vertices of a union share one seam name once \
     they cite member edges whole, and neither side of it is a member edge to rank them along";

/// Whether `name` is a RANKED piece of a member edge. In a name collapsed
/// out of a fold that did not finish — a refusal's — that rank is the
/// fold's, which no published table holds, so such a name cannot be
/// handed out.
pub(crate) fn is_fold_ranked_member_edge(name: &StableName) -> bool {
    member_edge_piece(name).is_some_and(|(_, _, ranked)| ranked)
}

/// The [`SegRewrite`] of [`cite_member_edges`]: an embedded ranked piece
/// of a member edge becomes the member edge. Only names the UNION
/// minted are rewritten or entered: the name a `FromMember` carries is
/// the member's own, final in the member.
struct WholeMemberEdges<'f, 'a, T: geom_core::Decide> {
    union: RecipeNodeId,
    /// The vertex whose name is rewritten, when it is one vertex: a
    /// member edge its name cites is cited as the least member edge
    /// through it ([`Flush::least_at`]).
    at: Option<topo::VertexKey>,
    flush: &'f Flush<'a, T>,
}

impl<T: geom_core::Decide> SegRewrite for WholeMemberEdges<'_, '_, T> {
    type Error = NamingError;

    fn name(&mut self, n: &StableName) -> Result<Option<StableName>, NamingError> {
        if n.node != self.union {
            return Ok(None);
        }
        if let Some((member, edge, _)) = member_edge_piece(n) {
            let (member, edge) = match self.at {
                Some(v) => self.flush.least_at(v, (member, edge)),
                None => (member, edge),
            };
            let whole = entity_name(self.union, &(member, edge));
            return Ok((whole != *n).then_some(whole));
        }
        let walked = n.clone().rewrite_path(self)?;
        Ok((walked != *n).then_some(walked))
    }
}

/// **A union's faces, grouped by parent** (N2, N3).
///
/// A face's parent is its merge closure: every merged face links the
/// member faces its name lists, a face that is a piece of one member
/// face links nothing, and linking is transitive. The member faces so
/// linked are one parent, named `Merged` of all of them; a member face
/// nothing links to another is its own parent, named as the member's
/// face. Which step met a face first, or cut it before or after it
/// merged, changes the spelling the fold gives it but not the member
/// faces it lists, so the grouping reads only those.
///
/// A member face that a row embeds and no face of the finished body
/// descends from is its own parent.
struct Parents {
    /// Finished face → its parent's name.
    of_face: BTreeMap<topo::FaceKey, StableName>,
    /// Member face → the name of the parent it is linked into.
    of_member: BTreeMap<MemberEntity, StableName>,
    /// Parent name → the parent.
    all: BTreeMap<StableName, Parent>,
}

/// One parent of [`Parents`].
struct Parent {
    /// The member faces linked into it.
    from: BTreeSet<MemberEntity>,
    /// The finished faces that are it or pieces of it.
    faces: Vec<topo::FaceKey>,
    /// Whether a member face in it is tied in its member's table (N2's
    /// tie, propagated).
    tied: bool,
}

/// The root of `m`'s class in [`Parents::of`]'s union–find.
fn root(link: &BTreeMap<MemberEntity, MemberEntity>, m: &MemberEntity) -> MemberEntity {
    let mut at = m.clone();
    while let Some(up) = link.get(&at).filter(|up| **up != at) {
        at = up.clone();
    }
    at
}

impl Parents {
    /// The parents of the finished faces the collapsed table `t` names.
    fn of<T: geom_core::Decide>(
        union: RecipeNodeId,
        t: &NameTable,
        members: &[Member<'_, T>],
    ) -> Result<Self, NamingError> {
        let bug = |what| NamingError::Emission { what };
        let mut rows: Vec<(topo::FaceKey, BTreeSet<MemberEntity>)> = Vec::new();
        for (name, entry) in t.iter() {
            if name.kind != EntityKind::Face {
                continue;
            }
            let mut from = BTreeSet::new();
            member_faces(name, &mut from);
            if from.is_empty() {
                return Err(bug("a union's face descends from no member face"));
            }
            let keys: Vec<EntityKey> = match entry {
                Entry::Unique(e) => vec![e.key],
                Entry::Tied(es) => es.iter().map(|e| e.key).collect(),
            };
            for key in keys {
                let EntityKey::Face(f) = key else {
                    return Err(bug("a union's face row names no face"));
                };
                rows.push((f, from.clone()));
            }
        }
        // Union–find over member faces; a class's root is its least
        // member face, so no visiting order changes it.
        let mut link: BTreeMap<MemberEntity, MemberEntity> = BTreeMap::new();
        for (_, from) in &rows {
            let mut from = from.iter();
            let Some(first) = from.next() else { continue };
            link.entry(first.clone()).or_insert_with(|| first.clone());
            for m in from {
                link.entry(m.clone()).or_insert_with(|| m.clone());
                let (r0, r1) = (root(&link, first), root(&link, m));
                let (lo, hi) = if r0 < r1 { (r0, r1) } else { (r1, r0) };
                link.insert(hi, lo);
            }
        }
        let mut classes: BTreeMap<MemberEntity, BTreeSet<MemberEntity>> = BTreeMap::new();
        for m in link.keys() {
            classes.entry(root(&link, m)).or_default().insert(m.clone());
        }
        let mut parents = Parents {
            of_face: BTreeMap::new(),
            of_member: BTreeMap::new(),
            all: BTreeMap::new(),
        };
        let mut named: BTreeMap<MemberEntity, StableName> = BTreeMap::new();
        for (r, from) in classes {
            let name = parent_name(union, &from);
            let mut tied = false;
            for (m, of) in &from {
                tied |= member_of(members, *m)?.table.is_tied(of.name());
                parents.of_member.insert((*m, of.clone()), name.clone());
            }
            named.insert(r, name.clone());
            parents.all.insert(
                name,
                Parent {
                    from,
                    faces: Vec::new(),
                    tied,
                },
            );
        }
        for (f, from) in rows {
            let first = from
                .first()
                .ok_or_else(|| bug("a union's face descends from no member face"))?;
            let name = named
                .get(&root(&link, first))
                .ok_or_else(|| bug("a union's face has no parent"))?
                .clone();
            parents
                .all
                .get_mut(&name)
                .ok_or_else(|| bug("a union's face has no parent"))?
                .faces
                .push(f);
            parents.of_face.insert(f, name);
        }
        Ok(parents)
    }

    /// Finished face `f`'s parent's name.
    fn face(&self, f: topo::FaceKey) -> Result<&StableName, NamingError> {
        self.of_face.get(&f).ok_or(NamingError::Emission {
            what: "a finished face of a union has no parent",
        })
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
        let mut cited = from.iter().map(|m| {
            self.of_member
                .get(m)
                .cloned()
                .unwrap_or_else(|| entity_name(union, m))
        });
        let Some(first) = cited.next() else {
            return Ok(None);
        };
        if cited.any(|c| c != first) {
            return Err(NamingError::Emission {
                what: "a name a union publishes cites faces of several parents",
            });
        }
        Ok(Some(first))
    }
}

/// What [`name_by_parents`] returns: the table, the seam rows held
/// apart for [`rank_member_edges`], and the groups it formed.
type ByParents = (NameTable, Vec<(StableName, Entry)>, GroupRecord);

/// A parent's name: its one member face, or `Merged` of all of them.
fn parent_name(union: RecipeNodeId, from: &BTreeSet<MemberEntity>) -> StableName {
    match from.iter().collect::<Vec<_>>().as_slice() {
        [one] => entity_name(union, one),
        many => canonical::minted(StableName {
            kind: EntityKind::Face,
            node: union,
            path: vec![RoleSeg::Merged(
                many.iter().map(|m| entity_name(union, m)).collect(),
            )],
        }),
    }
}

/// **A union's faces and seam edges are named for parents, read off the
/// finished body** (N2, N3).
///
/// - **Faces.** A parent the finished body holds as one face is that
///   face's name. A parent it holds as several faces qualifies each with
///   one `Fragment(SideOf)`: the partners are the parents of the faces
///   across the group's seam edges, cited by their parent names, and
///   each verdict is `name_frag_side_of` against a partner face's
///   outward carrier plane. Equal vectors are N2's tie. Tied member
///   faces share a name and so a parent; where no seam meets any of its
///   faces, they are the tie's candidates under the parent's name. An
///   untied parent held as several faces with no seam partner has no
///   rule, and refuses.
/// - **Seam edges.** A seam edge that lies along no member edge is
///   `Seam` of the parents on its two sides, in name order. Several such
///   edges between the same two parents are pieces of that seam, ranked
///   along the pair's `n_a × n_b`, `a` the lesser side.
/// - **Every other row** cites a face as its parent, and a re-derived
///   seam edge as it now publishes.
///
/// A seam edge lying along a member edge is returned apart, with only
/// the faces it cites rewritten: [`rank_member_edges`] names it as a
/// piece of that member edge, so its seam spelling is never published
/// and must not collide with one that is.
///
/// Returns the table, the rows held apart, and the groups formed.
fn name_by_parents<T: geom_core::Decide>(
    union: RecipeNodeId,
    t: &NameTable,
    body: &topo::Body<T>,
    parents: &Parents,
    flush: &Flush<'_, T>,
    bnd: geom_core::Band,
) -> Result<ByParents, NamingError> {
    let bug = |what| NamingError::Emission { what };
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
                return Err(bug("a union's seam row names no edge"));
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
    // Base → (from a tie, [(edge, a-side face, b-side face)]).
    type SeamGroup = (bool, Vec<(topo::EdgeKey, topo::FaceKey, topo::FaceKey)>);
    let mut seam_groups: BTreeMap<StableName, SeamGroup> = BTreeMap::new();
    for &k in &seams {
        let Some([f0, f1]) = flush.inc.edge_faces.get(&k).map(Vec::as_slice) else {
            return Err(bug("a union's seam edge does not lie between two faces"));
        };
        let (p0, p1) = (parents.face(*f0)?, parents.face(*f1)?);
        if p0 == p1 {
            return Err(bug("a union's seam edge lies inside one parent"));
        }
        let ((a, fa), (b, fb)) = if p0 < p1 {
            ((p0, *f0), (p1, *f1))
        } else {
            ((p1, *f1), (p0, *f0))
        };
        let tied = parents.all.get(a).is_some_and(|p| p.tied)
            || parents.all.get(b).is_some_and(|p| p.tied);
        let base = StableName {
            kind: EntityKind::Edge,
            node: union,
            path: vec![RoleSeg::Seam {
                a: NameRef::new(a.clone()),
                b: NameRef::new(b.clone()),
            }],
        };
        let slot = seam_groups.entry(base).or_insert((false, Vec::new()));
        slot.0 |= tied;
        slot.1.push((k, fa, fb));
    }
    // Edge → its seam name, for the rows that cite it.
    let mut seam_name: BTreeMap<topo::EdgeKey, StableName> = BTreeMap::new();
    for (base, (tied, edges)) in &seam_groups {
        let keys: Vec<topo::EdgeKey> = edges.iter().map(|(k, _, _)| *k).collect();
        rec.record_by_name(
            base,
            keys.iter().map(|&k| ent(0, EntityKey::Edge(k))).collect(),
            *tied,
        );
        let &[(_, fa, fb), ..] = edges.as_slice() else {
            continue;
        };
        if let [k] = keys.as_slice() {
            put(
                &mut out,
                &mut tie,
                *tied,
                base.clone(),
                ent(0, EntityKey::Edge(*k)),
            )?;
            seam_name.insert(*k, base.clone());
            continue;
        }
        let (_, na) = face_plane(body, fa)?;
        let (_, nb) = face_plane(body, fb)?;
        let dir = na.cross(nb);
        let extents = keys
            .iter()
            .map(|&k| edge_extent(body, k, dir))
            .collect::<Result<Vec<_>, _>>()?;
        match order_along(&extents, bnd)? {
            Some(ranks) => {
                let of = to_u32(
                    keys.len(),
                    "a union's seam has more pieces than a rank holds",
                )?;
                for (&k, rank) in keys.iter().zip(ranks) {
                    let mut name = base.clone();
                    name.path
                        .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }));
                    put(
                        &mut out,
                        &mut tie,
                        *tied,
                        name.clone(),
                        ent(0, EntityKey::Edge(k)),
                    )?;
                    seam_name.insert(k, name);
                }
            }
            None => mint_candidates(
                &mut out,
                &mut tie,
                *tied,
                base.clone(),
                keys.iter().map(|&k| ent(0, EntityKey::Edge(k))).collect(),
            )?,
        }
    }

    // ---- Faces, by parent. ----
    for (pname, parent) in &parents.all {
        let faces = &parent.faces;
        rec.record_by_name(
            pname,
            faces.iter().map(|&f| ent(0, EntityKey::Face(f))).collect(),
            parent.tied,
        );
        match faces.as_slice() {
            [] => {}
            [one] => put(
                &mut out,
                &mut tie,
                parent.tied,
                pname.clone(),
                ent(0, EntityKey::Face(*one)),
            )?,
            _ => {
                let mut partners: BTreeMap<&StableName, topo::FaceKey> = BTreeMap::new();
                for &f in faces {
                    for he in face_half_edges(body, f)? {
                        let e = body
                            .get_half_edge(he)
                            .ok_or_else(|| bug("a union face's half-edge is dangling"))?
                            .edge;
                        if !seams.contains(&e) {
                            continue;
                        }
                        for &g in flush.inc.edge_faces.get(&e).into_iter().flatten() {
                            let q = parents.face(g)?;
                            if q != pname {
                                partners.entry(q).or_insert(g);
                            }
                        }
                    }
                }
                if partners.is_empty() {
                    // Tied member faces share one name, so one "parent"
                    // can be several whole faces nothing cuts: the tie's
                    // candidates, published as the tie (N2, B1).
                    if parent.tied {
                        mint_candidates(
                            &mut out,
                            &mut tie,
                            true,
                            pname.clone(),
                            faces.iter().map(|&f| ent(0, EntityKey::Face(f))).collect(),
                        )?;
                        continue;
                    }
                    return Err(bug(
                        "a union holds a parent as several faces with no seam partner",
                    ));
                }
                let mut by_vector: BTreeMap<Vec<(StableName, SideVerdict)>, Vec<topo::FaceKey>> =
                    BTreeMap::new();
                for &f in faces {
                    let mut vector = Vec::with_capacity(partners.len());
                    for (&q, &g) in &partners {
                        let (origin, normal) = face_plane(body, g)?;
                        vector.push((q.clone(), side_of_face(body, f, origin, normal, bnd)?));
                    }
                    by_vector.entry(vector).or_default().push(f);
                }
                for (vector, fs) in by_vector {
                    let mut name = pname.clone();
                    name.path.push(RoleSeg::Fragment(Qualifier::SideOf(vector)));
                    mint_candidates(
                        &mut out,
                        &mut tie,
                        parent.tied,
                        canonical::minted(name),
                        fs.iter().map(|&f| ent(0, EntityKey::Face(f))).collect(),
                    )?;
                }
            }
        }
    }

    // ---- Every other row: its faces cited as parents. ----
    let mut cite = CiteParents {
        union,
        parents,
        seams: respelled
            .into_iter()
            .filter_map(|(old, k)| Some((old, seam_name.get(&k)?.clone())))
            .collect(),
    };
    for (name, entry) in t.iter() {
        if name.kind == EntityKind::Face || is_seam(name) {
            continue;
        }
        let spelled = name.clone().rewrite_path(&mut cite)?;
        put_entry(&mut out, spelled, entry)?;
    }
    for (name, _) in &mut held {
        *name = name.clone().rewrite_path(&mut cite)?;
    }
    tie.flush(&mut out)?;
    Ok((out, held, rec))
}

/// The [`SegRewrite`] of [`name_by_parents`]: a face a name embeds is
/// cited as its parent, and a seam edge the union re-derived as it now
/// publishes. Only names the UNION minted are rewritten: a
/// `FromMember`'s own name is the member's, final in the member.
struct CiteParents<'p> {
    union: RecipeNodeId,
    parents: &'p Parents,
    /// A unique seam row's collapsed spelling → its published one.
    seams: BTreeMap<StableName, StableName>,
}

impl SegRewrite for CiteParents<'_> {
    type Error = NamingError;

    fn name(&mut self, n: &StableName) -> Result<Option<StableName>, NamingError> {
        if n.node != self.union {
            return Ok(None);
        }
        if let Some(parent) = self.parents.cited(self.union, n)? {
            return Ok((parent != *n).then_some(parent));
        }
        if let Some(now) = self.seams.get(n) {
            return Ok((now != n).then(|| now.clone()));
        }
        let walked = n.clone().rewrite_path(self)?;
        Ok((walked != *n).then_some(walked))
    }
}

/// `name` without the trailing `Fragment`s whose qualifier `drop`
/// accepts, its head always kept.
fn without_tail(name: &StableName, drop: impl Fn(&Qualifier) -> bool) -> StableName {
    let mut base = name.clone();
    while base.path.len() > 1 && matches!(base.path.last(), Some(RoleSeg::Fragment(q)) if drop(q)) {
        base.path.pop();
    }
    base
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
        put_entry(&mut t, collapse(node, name)?, entry)?;
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

use super::canonical::{self, Stop, Unrankable, is_junction};
use super::merged::NESTED_MERGED;

/// One fold-table name, keyed by member.
///
/// Returns a name in the UNION's own space (`node` is the union's, as
/// every fold row's already is): either one [`RoleSeg::FromMember`],
/// [`RoleSeg::Seam`], [`RoleSeg::Merged`] or [`RoleSeg::OutputBody`]
/// head followed by the `Fragment` discriminators the fold
/// accumulated, outermost step last — or, for a seam JUNCTION vertex,
/// a run of two or more `Seam` lines and nothing else.
///
/// Two halves. [`orient`] rewrites every name the path holds into this
/// node's space and keeps the fold's order, so every seam pair is still
/// A-first and every rank is still along the A-first line. The
/// canonical form (`names::canonical::collapsed`) then puts the
/// name-ordered positions in order, and re-reads each rank against the
/// line as the fold-space name and the collapsed one write it. Each
/// name the path embeds comes out of this function whole, so it is
/// canonical before the path holding it is ordered.
fn collapse(node: RecipeNodeId, name: &StableName) -> Result<StableName, NamingError> {
    let name = canonical::collapsed(name, orient(node, name)?, &mut |n| collapse(node, n))
        .map_err(|stop| match stop {
            Stop::Image(e) => e,
            Stop::Unrankable(u) => NamingError::Emission { what: u.what() },
        })?;
    // The junction's run is NOT deduplicated, unlike a `Merged` set.
    // The pair emitter deduplicates the lines before it mints, so the
    // run holds k DISTINCT fold-space lines; two collapsing to one
    // would make the rewrite many-to-one on lines, and dropping one
    // would publish a name for a vertex with fewer lines than it has.
    // A `Merged` set is a set of FACES whose name is the set (N3), so
    // there a repeat is the same constituent reached twice, and two
    // merges that collapse to one set collide at insert instead.
    if is_junction(&name) && name.path.windows(2).any(|w| w[0] == w[1]) {
        return Err(NamingError::Emission {
            what: JUNCTION_LINES_COLLIDE,
        });
    }
    Ok(name)
}

/// [`collapse`]'s rewrite half: the fold-table name with every name it
/// holds collapsed into this node's space, in the fold's order.
///
/// A `FromA`/`FromB` head is descended through by this same half, so
/// the inner name's own positions stay in fold order, and its ranks
/// along the fold's A-first line, until the one canonicalization at the
/// top. Every rank the flattened path carries lies along the one line
/// the canonical form finds through the same wrapping: an edge's
/// pieces all lie on its seam line, and a seam vertex's rank on its
/// edge parent's.
fn orient(node: RecipeNodeId, name: &StableName) -> Result<StableName, NamingError> {
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
        RoleSeg::FromA(inner) | RoleSeg::FromB(inner) => orient(node, inner)?.path,
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
        // The canonical form makes the constituent SET the name, the
        // form the pair emitter's mint gives it too (review R8): two
        // merge groups collapsing to ONE constituent set collide
        // LOUDLY at insert (`DuplicateName` → typed `NamingError`),
        // never silently aliasing two faces onto one name.
        RoleSeg::Merged(constituents) => {
            let mut set = Vec::with_capacity(constituents.len());
            for c in constituents {
                let c = collapse(node, c)?;
                if matches!(c.path.as_slice(), [RoleSeg::Merged(_)]) {
                    return Err(bug(NESTED_MERGED));
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
            // The rank is a place along the fold's direction; the
            // canonicalization re-reads it.
            RoleSeg::Fragment(Qualifier::OrderAlong { .. }) => seg.clone(),
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

/// One seam line between two members, in the union's space and in the
/// fold's order.
///
/// The pair emitter's `a`/`b` are the crossing entities in the two
/// OPERANDS' tables, which are this node's space on both sides, so
/// each is collapsed the same way every other row is. The pair keeps
/// the fold's A-first order here, and the canonicalization puts it in
/// name order: a union is commutative, so "which side" would record
/// only which of the two members the fold reached first, which is the
/// position this node exists not to record.
fn seam_line(node: RecipeNodeId, a: &StableName, b: &StableName) -> Result<RoleSeg, NamingError> {
    Ok(RoleSeg::Seam {
        a: NameRef::new(collapse(node, a)?),
        b: NameRef::new(collapse(node, b)?),
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
    use crate::names::canonical::Unrankable;
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
            matches!(err, NamingError::Emission { what } if what == Unrankable::SidedVertexRank.what()),
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
                matches!(err, NamingError::Emission { what } if what == Unrankable::RankOutsideCount.what()),
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
            let parents = Parents::of(union, &table, &members).unwrap();
            let flush = Flush::of(union, &body, &members, &parents, bnd).unwrap();
            let err =
                rank_member_edges(table, Vec::new(), &body, &members, &flush, bnd).unwrap_err();
            assert!(
                matches!(&err, NamingError::MemberEdgeTied { member, edge: e }
                    if *member == m && **e == edge),
                "{label}: {err:?}"
            );
        }
    }

    /// **A band with ambiguity K below 2 refuses to count cells**: two
    /// coincidences in a row could span a definite separation there, so
    /// a place would be a guess.
    #[test]
    fn a_band_narrower_than_twice_its_zero_refuses_to_count_cells() {
        let (body, k0, _) = two_edge_body();
        let seg = Segment::of_edge(&body, k0).unwrap();
        let narrow = geom_core::Band::new(1e-9, 1.5e-9).unwrap();
        let err = Cells::of(&body, &seg, narrow).map(|_| ()).unwrap_err();
        assert!(
            matches!(err, NamingError::NarrowBand { zero, escalate } if zero == 1e-9 && escalate == 1.5e-9),
            "{err:?}"
        );
        let wide = geom_core::Band::new(1e-9, 2e-9).unwrap();
        assert!(Cells::of(&body, &seg, wide).is_ok());
    }

    /// The pieces [`cite_member_edges`] is run on: member 5's body
    /// holding its one edge (x = 0 → 1), that member's table naming it,
    /// and a union body holding two lone vertices on the edge (x = 0.25
    /// and 0.75), returned in order ALONG the member edge — the order
    /// every ranking of a group on it has to use.
    struct CitedGroup {
        union: RecipeNodeId,
        member_body: topo::Body<f64>,
        member_table: NameTable,
        body: topo::Body<f64>,
        along: [topo::VertexKey; 2],
    }

    impl CitedGroup {
        fn new() -> Self {
            let union = RecipeNodeId(9);
            let mut member_body = topo::Body::<f64>::new();
            let born = member_body
                .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0))
                .expect("mvfs births a lone vertex");
            let edge = member_body
                .mev_line(
                    topo::MevSite::Lone {
                        r#loop: born.r#loop,
                    },
                    geom_core::Point3::new(1.0, 0.0, 0.0),
                    geom_core::Tol::witness(),
                )
                .expect("mev grows the loop by one edge")
                .edge;
            let mut member_table = NameTable::new();
            let keyed = member_edge(union, 5);
            let [RoleSeg::FromMember { of, .. }] = keyed.path.as_slice() else {
                unreachable!("member_edge is one FromMember segment")
            };
            member_table.insert((**of).clone(), edge_ref(edge)).unwrap();
            let mut body = topo::Body::<f64>::new();
            let mut lone = |x: f64| {
                body.mvfs(geom_core::Point3::new(x, 0.0, 0.0))
                    .expect("mvfs births a lone vertex")
                    .vertex
            };
            let (low, high) = (lone(0.25), lone(0.75));
            let seg = Segment::of_edge(&member_body, edge).unwrap();
            let at = |v| seg.along(vertex_point(&body, v).unwrap());
            let along = if at(low) < at(high) {
                [low, high]
            } else {
                [high, low]
            };
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
                node: RecipeNodeId(5),
                body: &self.member_body,
                table: &self.member_table,
            }]
        }

        /// A piece of member 5's edge as the fold spells it once a step
        /// has cut it in two.
        fn piece(&self, rank: u32) -> StableName {
            let mut n = member_edge(self.union, 5);
            n.path
                .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 }));
            n
        }

        /// A vertex name of the group: `side` against member 2's cap, in
        /// name order as a union's seams are, ranked `rank` of 2 when
        /// given.
        fn named(&self, side: StableName, rank: Option<u32>) -> StableName {
            let cap = member_cap(self.union, 2);
            let line = if side < cap {
                seam(side, cap)
            } else {
                seam(cap, side)
            };
            let mut path = vec![line];
            path.extend(rank.map(|rank| RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 })));
            vertex(self.union, path)
        }

        fn cite(&self, rows: Vec<(StableName, topo::VertexKey)>) -> Result<NameTable, NamingError> {
            let mut t = NameTable::new();
            for (name, v) in rows {
                t.insert(
                    name,
                    crate::names::table::EntityRef {
                        body: 0,
                        key: EntityKey::Vertex(v),
                    },
                )
                .unwrap();
            }
            let bnd = geom_core::Band::new(1e-9, 1e-6).unwrap();
            let members = self.members();
            let parents = Parents::of(self.union, &t, &members)?;
            let flush = Flush::of(self.union, &self.body, &members, &parents, bnd)?;
            cite_member_edges(t, &self.body, &members, &flush, bnd)
        }
    }

    /// **A seam-vertex group on one member edge is ranked along that
    /// edge whether or not its vertices moved**, so `#k of 2` is the
    /// same vertex in every member order.
    ///
    /// Two vertices of one seam, member 5's edge against member 2's cap.
    /// In one member order a step cut the edge before the cap met it, so
    /// the fold's names cite pieces and the group is re-ranked here. In
    /// the other the edge was whole when the cap met it, so no name
    /// moves, and the fold's ranks stand unless this pass ranks the group
    /// itself. Here the fold ranked it the other way along the line, as
    /// a carrier running against the member edge would: in the
    /// accumulated body rather than the member's, or along a seam line.
    /// No document reaches that today (no union in the review corpus
    /// forms a group of two), which is why it is pinned on the pass.
    #[test]
    fn a_cited_group_ranks_along_its_member_edge_whether_or_not_it_moved() {
        let g = CitedGroup::new();
        let whole = member_edge(g.union, 5);
        let [first, second] = g.along;
        let cut = g
            .cite(vec![
                (g.named(g.piece(0), None), first),
                (g.named(g.piece(1), None), second),
            ])
            .unwrap();
        let whole_then = g
            .cite(vec![
                (g.named(whole.clone(), Some(0)), second),
                (g.named(whole.clone(), Some(1)), first),
            ])
            .unwrap();
        for (label, out) in [("cut first", &cut), ("whole when met", &whole_then)] {
            for (rank, want) in [(0, first), (1, second)] {
                let name = g.named(whole.clone(), Some(rank));
                assert_eq!(
                    out.lookup(&name),
                    Some(&Entry::Unique(crate::names::table::EntityRef {
                        body: 0,
                        key: EntityKey::Vertex(want),
                    })),
                    "{label}: #{rank} of 2 is not the vertex {rank} along the member edge"
                );
            }
            assert_eq!(out.len(), 2, "{label}: {out:?}");
        }
    }

    /// **A group of several vertices sharing a name that is not one seam
    /// refuses**: nothing says which line to rank them along. The name
    /// here is a run of two lines whose first cites a piece of member 5's
    /// edge, so the pass moves it.
    #[test]
    fn a_cited_group_that_is_not_one_seam_refuses() {
        let g = CitedGroup::new();
        let other = seam(member_cap(g.union, 3), member_cap(g.union, 4));
        let run = |k| {
            let one = g.named(g.piece(k), None);
            let mut path = one.path;
            path.push(other.clone());
            vertex(g.union, path)
        };
        let err = g
            .cite(vec![(run(0), g.along[0]), (run(1), g.along[1])])
            .unwrap_err();
        assert!(
            matches!(err, NamingError::Emission { what } if what == CITED_GROUP_NOT_ONE_SEAM),
            "{err:?}"
        );
    }

    /// **A group on one seam neither of whose sides is a member edge
    /// refuses**: there is no member edge to rank along. The seam's edge
    /// side is an edge that itself cites a piece of member 5's edge, so
    /// the pass moves it without making it a member edge.
    #[test]
    fn a_cited_group_with_no_member_edge_side_refuses() {
        let g = CitedGroup::new();
        let carrying = |k: u32| StableName {
            kind: EntityKind::Edge,
            node: g.union,
            path: vec![seam(g.piece(k), member_cap(g.union, 3))],
        };
        let err = g
            .cite(vec![
                (g.named(carrying(0), None), g.along[0]),
                (g.named(carrying(1), None), g.along[1]),
            ])
            .unwrap_err();
        assert!(
            matches!(err, NamingError::Emission { what } if what == CITED_GROUP_NO_MEMBER_EDGE),
            "{err:?}"
        );
    }
}

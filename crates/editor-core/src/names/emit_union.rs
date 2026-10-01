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
//! seam's two sides, a merge's constituents, a fragment's walls —
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
//! published name also end in: a `Merged` set and a `Borders` set
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
//!   link to it, transitively, followed by entity through the fold
//!   ([`Fold`], [`Parents`]): the parent itself when the finished body
//!   holds it as one face, and otherwise the parent and one `Borders`
//!   over the divider walls each piece borders; a seam edge is named
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

use crate::names::borders::Obstacles;
use crate::names::defer::{TieRows, mint_candidates};
use crate::names::discriminate::{ON_MEMBER_EDGE, band};
use crate::names::emit::{
    Incidence, NamingError, check_total, edge_ends, ent, face_half_edges, rims_between,
    vertex_point,
};
use crate::names::emit_topo::{
    FaceDescent, OnSegment, Segment, name_edge_pieces, name_parent_faces, rank_crossings,
};
use crate::names::groups::Rederived;
use crate::names::least_root::LeastRoot;
use crate::names::nest::{Descent, Kept, Stopped, descend};
use crate::names::role::{
    Carry, EntityKind, NameRef, Qualifier, RoleSeg, SegRewrite, StableName,
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
    fold: &Fold,
    tol: geom_core::Tol,
) -> Result<(Arc<NameTable>, Rederived), NamingError> {
    let bnd = band(tol)?;
    let t = collapse_table(node, folded)?;
    let parents = Parents::of(node, body, members, fold)?;
    let flush = Flush::of(node, body, members, &parents, bnd)?;
    let by_parents = name_by_parents(node, &t, body, &parents, fold, &flush)?;
    let (t, member_edges) = group_member_edges(by_parents.table, by_parents.held, &flush)?;
    let mut t = cite_member_edges(t, by_parents.vertices, body, members, &flush, bnd)?;
    let mut tie = TieRows::default();
    for g in by_parents.seams.iter().chain(&member_edges) {
        name_edge_pieces(&mut t, &mut tie, g.from_tie, &g.base, body, 0, &g.edges)?;
    }
    tie.flush(&mut t)?;
    check_total(&t, body, 0)?;
    Ok((Arc::new(t), by_parents.groups))
}

/// The edges of a union's finished body that are pieces of one parent
/// — a seam between two parents, or a member edge — under that
/// parent's name. They are qualified by their ends once the vertices
/// are named ([`name_edge_pieces`]).
struct PieceGroup {
    base: StableName,
    from_tie: bool,
    edges: Vec<topo::EdgeKey>,
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

/// **The pieces of each member edge, grouped under it.**
///
/// A row `FromMember(m, e)` followed only by the fold's
/// `Fragment(Ends)`, `e` an edge, is a piece of `m`'s edge `e`; so is
/// any edge row that lies along a member edge whatever the fold named
/// it (a merged edge the fold met as a seam). Either is a piece of the
/// LEAST member edge it lies within ([`Flush`]), which is `e` unless `e`
/// runs flush with a lesser member's edge there. Each group is named
/// `FromMember(m, e)`, and several pieces of it by their ends over the
/// union's published vertex names, read off the finished body: the
/// fold's qualifiers record which step cut the edge and which member
/// kept a flush stretch, both of which depend on member order.
///
/// Returns the table without those rows, and the groups.
fn group_member_edges<T: geom_core::Decide>(
    t: NameTable,
    held: Vec<(StableName, Entry)>,
    flush: &Flush<'_, T>,
) -> Result<(NameTable, Vec<PieceGroup>), NamingError> {
    let bug = |what| NamingError::Emission { what };
    // (member, member edge) → (from a tie, its edges).
    let mut groups: BTreeMap<MemberEntity, (bool, Vec<topo::EdgeKey>)> = BTreeMap::new();
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
        let key = match (member_edge_piece(&name), edges.as_slice()) {
            (Some(_), []) => return Err(bug("a union's member-edge row names no edge")),
            (Some((member, edge, _)), [k]) => Some(flush.least_edge(*k, (member, edge))),
            (Some((member, edge, _)), _) => Some((member, edge)),
            (None, [k]) => flush.least_within(*k),
            (None, _) => None,
        };
        match key {
            Some(key) => {
                let group = groups.entry(key).or_default();
                group.0 |= matches!(entry, Entry::Tied(_));
                group.1.extend(edges);
            }
            None => put_entry(&mut out, name, &entry)?,
        }
    }
    let groups = groups
        .into_iter()
        .map(|(key, (from_tie, edges))| PieceGroup {
            base: entity_name(flush.union, &key),
            from_tie,
            edges,
        })
        .collect();
    Ok((out, groups))
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
        for &f in parents.of_face.keys() {
            let parent = parents.face(f)?;
            faces.insert(f, parent.from.clone());
            face_names.insert(f, parent.name.clone());
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
    let mut names = vec![name];
    while let Some(name) = names.pop() {
        match name.path.first() {
            Some(RoleSeg::FromMember { member, of }) if of.kind == EntityKind::Face => {
                out.insert((*member, of.clone()));
            }
            Some(RoleSeg::Merged(set)) => names.extend(set),
            _ => {}
        }
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
/// `FromMember(m, e)`, `e` an edge, then nothing but `Fragment(Ends)`
/// qualifiers — and whether any qualifier follows.
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
            .all(|s| matches!(s, RoleSeg::Fragment(Qualifier::Ends(_))))
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
/// had cut it when it met the vertex — a piece that no published name
/// denotes. It is cited as `FromMember(m, e)`, which is how a vertex
/// already cites an edge the fold had not cut, and as the least member
/// edge through the vertex along that line ([`Flush::least_at`]). The
/// rewrite is [`StableName::rewrite_path`], which ends in the canonical
/// form (`names::canonical::rewritten`).
///
/// A vertex's own trailing rank is fold history too once its name moved,
/// so vertices are grouped by their name without it. A group of several
/// whose name is one seam citing one member edge is ranked WHOLE along
/// that edge, by its carrier's parameter, oriented as it is in the
/// member's own body ([`rank_crossings`]), whether or not any of its
/// names moved: the order is then the member's, in every member order,
/// and not the one the fold happened to rank along at the step that met
/// the group. A seam citing two member edges refuses, as the collapse
/// already does for such a group's ranks. A group whose one seam
/// crosses a union seam [`name_by_parents`] re-spelled by its head has
/// no one carrier to rank along, since the seam may be several curves,
/// and ties. A
/// group with a moved name and no single seam, or a seam citing no
/// edge, refuses. A group with no moved name and no member edge to rank
/// along keeps its names: its ranks, if any, lie along a seam, in the
/// orientation the collapse put in canonical form.
fn cite_member_edges<T: geom_core::Decide>(
    t: NameTable,
    vertex_rows: Vec<(StableName, Entry, bool)>,
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
    // The bases of groups a seam cited by its head moved.
    let mut respelled_bases: BTreeSet<StableName> = BTreeSet::new();
    let rows = t
        .iter()
        .map(|(name, entry)| (name.clone(), entry.clone(), false))
        .chain(vertex_rows);
    for (name, entry, respelled) in rows {
        // The vertex a row names, when it names one vertex.
        let at = match &entry {
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
            put_entry(&mut out, cited, &entry)?;
            continue;
        }
        let moved = respelled || cited != name;
        let base = without_tail(&cited, |q| matches!(q, Qualifier::OrderAlong { .. }));
        if respelled {
            respelled_bases.insert(base.clone());
        }
        vertices
            .entry(base)
            .or_default()
            .push((cited, entry, moved));
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
                (None, None) if respelled_bases.contains(&base) => {
                    let mut ents = Vec::with_capacity(rows.len());
                    for (_, entry, _) in rows {
                        match entry {
                            Entry::Unique(e) => ents.push(e),
                            Entry::Tied(es) => ents.extend(es),
                        }
                    }
                    mint_candidates(&mut out, &mut tie, false, base, ents)?;
                    continue;
                }
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
        let mut keys = Vec::with_capacity(rows.len());
        let mut points = Vec::with_capacity(rows.len());
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
            keys.push(e);
            points.push(vertex_point(body, v)?);
        }
        let member_table = member_of(members, member)?.table;
        rank_crossings(
            &mut out,
            &mut tie,
            false,
            &base,
            (member_body, member_table, member_edge, edge.name()),
            &keys,
            &points,
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
     they cite member edges whole, and neither side of it is an edge to rank them along";

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

    fn name(&mut self, n: &StableName) -> Result<Carry, NamingError> {
        if n.node != self.union {
            return Ok(Carry::Keep);
        }
        if let Some((member, edge, _)) = member_edge_piece(n) {
            let (member, edge) = match self.at {
                Some(v) => self.flush.least_at(v, (member, edge)),
                None => (member, edge),
            };
            let whole = entity_name(self.union, &(member, edge));
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
}

/// A member face, by entity: the member's node and the face's key in
/// the member's own body.
type MemberFace = (RecipeNodeId, topo::FaceKey);

/// **What a union's fold records for its end pass**: which member faces
/// each face of the accumulation descends from, by entity, and every
/// step's discards ([`Obstacles`]) keyed by the member faces the
/// discarded face descends from.
///
/// A face's descent is read off each step's rows, never off a name:
/// a piece of a face descends from what the face does, a merged face
/// from what each face it absorbed does, and a face of the member the
/// step folds in from that member's face.
pub(crate) struct Fold {
    /// Accumulation face → the member faces it descends from.
    lineage: BTreeMap<topo::FaceKey, BTreeSet<MemberFace>>,
    obstacles: Obstacles<MemberFace>,
}

impl Fold {
    /// The fold before any step: the first member's body.
    pub(crate) fn new<T: geom_core::Real>(first: RecipeNodeId, body: &topo::Body<T>) -> Self {
        Self {
            lineage: body
                .faces()
                .map(|(f, _)| (f, BTreeSet::from([(first, f)])))
                .collect(),
            obstacles: Obstacles::new(),
        }
    }

    /// One fold step: `member` folded into the accumulation, giving
    /// `result` with the kernel's record `naming`.
    pub(crate) fn step<T: geom_core::Real>(
        &mut self,
        member: RecipeNodeId,
        naming: &topo::BooleanNaming,
        result: &topo::Body<T>,
    ) -> Result<(), NamingError> {
        let bug = |what| NamingError::Emission { what };
        let descent = FaceDescent::of(naming);
        let from = |lineage: &BTreeMap<topo::FaceKey, BTreeSet<MemberFace>>,
                    (operand, f): (topo::Operand, topo::FaceKey)|
         -> Result<BTreeSet<MemberFace>, NamingError> {
            match operand {
                topo::Operand::A => lineage
                    .get(&f)
                    .cloned()
                    .ok_or_else(|| bug("a union fold step's face descends from no face it held")),
                topo::Operand::B => Ok(BTreeSet::from([(member, f)])),
            }
        };
        self.obstacles.record(naming, result, |operand, f| {
            from(&self.lineage, (operand, descent.clone_face(operand, f)?))
        })?;
        let merged = descent.merged();
        let mut next = BTreeMap::new();
        for (f, _) in result.faces() {
            let mut set = BTreeSet::new();
            for &g in merged.get(&f).map_or(&[f][..], Vec::as_slice) {
                set.extend(from(&self.lineage, descent.result_face(g)?)?);
            }
            next.insert(f, set);
        }
        self.lineage = next;
        Ok(())
    }
}

/// **A union's faces, grouped by parent** (N2, N3).
///
/// A face's parent is its merge closure, read by entity off the fold
/// ([`Fold`]): a finished face links the member faces it descends
/// from, and linking is transitive. The member faces so linked are one
/// parent, named `Merged` of all of them; a member face nothing links
/// to another is its own parent, named as the member's face. Which step
/// met a face first, or cut it before or after it merged, changes the
/// spelling the fold gives it but not the member faces it descends
/// from. Tied member faces are separate parents spelled alike.
///
/// A member face that a row embeds and no face of the finished body
/// descends from is its own parent.
struct Parents {
    /// Finished face → its parent.
    of_face: BTreeMap<topo::FaceKey, usize>,
    /// Member face, by name → the names of the parents it is linked
    /// into (several only for a tied name whose candidates went apart).
    of_member: BTreeMap<MemberEntity, BTreeSet<StableName>>,
    all: Vec<Parent>,
}

/// One parent of [`Parents`].
struct Parent {
    name: StableName,
    /// The member faces linked into it.
    entities: BTreeSet<MemberFace>,
    /// Their names in their members' tables.
    from: BTreeSet<MemberEntity>,
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
        fold: &Fold,
    ) -> Result<Self, NamingError> {
        let bug = |what| NamingError::Emission { what };
        let mut rows: Vec<(topo::FaceKey, &BTreeSet<MemberFace>)> = Vec::new();
        for (f, _) in body.faces() {
            let from = fold
                .lineage
                .get(&f)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| bug("a union's face descends from no member face"))?;
            rows.push((f, from));
        }
        // The member faces one finished face descends from are one class.
        let mut link = LeastRoot::new();
        for (_, from) in &rows {
            let mut from = from.iter().copied();
            let Some(first) = from.next() else { continue };
            link.insert(first);
            for m in from {
                link.join(first, m);
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
                let member = member_of(members, m)?;
                let of = member
                    .table
                    .name_ref_of(&ent(0, EntityKey::Face(f)))
                    .ok_or_else(|| bug("a member face a union holds is unnamed in its member"))?
                    .clone();
                tied |= member.table.is_tied(&of);
                from.insert((m, of));
            }
            let name = parent_name(union, &from);
            for m in &from {
                parents
                    .of_member
                    .entry(m.clone())
                    .or_default()
                    .insert(name.clone());
            }
            index.insert(r, parents.all.len());
            parents.all.push(Parent {
                name,
                entities,
                from,
                faces: Vec::new(),
                tied,
            });
        }
        for (f, from) in rows {
            let first = *from
                .first()
                .ok_or_else(|| bug("a union's face descends from no member face"))?;
            let p = *index
                .get(&link.root(first))
                .ok_or_else(|| bug("a union's face has no parent"))?;
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
    fn face(&self, f: topo::FaceKey) -> Result<&Parent, NamingError> {
        self.of_face
            .get(&f)
            .map(|&p| &self.all[p])
            .ok_or(NamingError::Emission {
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
            return Err(NamingError::Emission {
                what: "a name a union publishes cites faces of several parents",
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
    /// by their heads, each with whether a seam it cites was re-spelled.
    vertices: Vec<(StableName, Entry, bool)>,
    /// The seam edges, grouped by their two parents.
    seams: Vec<PieceGroup>,
    /// The groups formed.
    groups: Rederived,
}

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
///   one `Fragment(Borders)` over the divider walls it borders, each
///   cited by its parent's name: the one rule the pair boolean reads
///   ([`Obstacles::split`], over every fold step's discards). Pieces
///   with one set are N2's tie.
/// - **Seam edges.** A seam edge that lies along no member edge is
///   `Seam` of the parents on its two sides, in name order. Several such
///   edges between the same two parents are pieces of that seam, grouped
///   to be told apart by their ends once the vertices are named.
/// - **Groups.** The groups formed are returned for the diagnosis
///   ladder, except those of a tied parent, which are formed by name and
///   so hold no one parent's count: those bases are left to the fold
///   ([`Rederived`]).
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
    fold: &Fold,
    flush: &Flush<'_, T>,
) -> Result<ByParents, NamingError> {
    let bug = |what| NamingError::Emission { what };
    let mut out = NameTable::new();
    let mut tie = TieRows::default();
    let mut rec = Rederived::default();
    // A group is formed by parent NAME, so tied parents share one and it
    // holds no one parent's count: the fold's entity-followed groups
    // answer for it instead.
    let mut record = |base: &StableName, members: Vec<_>, tied: bool| {
        if tied {
            rec.by_fold.insert(base.clone());
        } else {
            rec.groups.record_by_name(base, members, false);
        }
    };
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
    // Base → (from a tie, its edges).
    let mut seam_groups: BTreeMap<StableName, (bool, Vec<topo::EdgeKey>)> = BTreeMap::new();
    for &k in &seams {
        let Some([f0, f1]) = flush.inc.edge_faces.get(&k).map(Vec::as_slice) else {
            return Err(bug("a union's seam edge does not lie between two faces"));
        };
        if parents.of_face.get(f0) == parents.of_face.get(f1) {
            return Err(bug("a union's seam edge lies inside one parent"));
        }
        let (p0, p1) = (parents.face(*f0)?, parents.face(*f1)?);
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
        let slot = seam_groups.entry(base).or_default();
        slot.0 |= p0.tied || p1.tied;
        slot.1.push(k);
    }
    // Edge → its seam's head, for the rows that cite it.
    let mut seam_head: BTreeMap<topo::EdgeKey, StableName> = BTreeMap::new();
    let mut groups = Vec::with_capacity(seam_groups.len());
    for (base, (tied, keys)) in seam_groups {
        record(
            &base,
            keys.iter().map(|&k| ent(0, EntityKey::Edge(k))).collect(),
            tied,
        );
        for &k in &keys {
            seam_head.insert(k, base.clone());
        }
        groups.push(PieceGroup {
            base,
            from_tie: tied,
            edges: keys,
        });
    }

    // ---- Faces, by parent. ----
    for parent in &parents.all {
        let faces = &parent.faces;
        record(
            &parent.name,
            faces.iter().map(|&f| ent(0, EntityKey::Face(f))).collect(),
            parent.tied,
        );
        name_parent_faces(
            &mut out,
            &mut tie,
            parent.tied,
            parent.name.clone(),
            faces,
            &[],
            (&fold.obstacles, body, &parent.entities),
            |g| Ok(parents.face(g)?.name.clone()),
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
        respelled: false,
    };
    let mut vertices = Vec::new();
    for (name, entry) in t.iter() {
        if name.kind == EntityKind::Face || is_seam(name) {
            continue;
        }
        cite.respelled = false;
        let spelled = name.clone().rewrite_path(&mut cite)?;
        if name.kind == EntityKind::Vertex {
            vertices.push((spelled, entry.clone(), cite.respelled));
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
/// `FromMember`'s own name is the member's, final in the member.
struct CiteParents<'p> {
    union: RecipeNodeId,
    parents: &'p Parents,
    /// A unique seam row's collapsed spelling → its published head.
    seams: BTreeMap<StableName, StableName>,
    /// Whether the name being rewritten cites a seam whose spelling
    /// moved.
    respelled: bool,
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
                self.respelled = true;
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
/// constituents at the foot of its `FromA`/`FromB` descent, and every
/// `Borders` wall and `Ends` vertex along it, in the order the collapse
/// meets them.
fn held_collapses(name: &StableName) -> Vec<&StableName> {
    let mut out = Vec::new();
    let mut at = name;
    while let Some((head, tail)) = at.path.split_first() {
        match head {
            RoleSeg::FromA(inner) | RoleSeg::FromB(inner) => at = inner,
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
    let oriented = orient(node, name, done)?;
    let mut image = |n: &'s StableName| done.need(n).map(|r| r.name().clone());
    let name = canonical::collapsed(name, oriented, &mut image).map_err(|stop| match stop {
        Stop::Image(stopped) => stopped,
        Stop::Unrankable(u) => Stopped::Refused(NamingError::Emission { what: u.what() }),
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
        }
        .into());
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
///
/// The descent is a loop: the tails of the levels it passes through
/// are kept, and put after the foot's path innermost first, which is
/// the order a descent level by level would write them in.
fn orient<'s>(
    node: RecipeNodeId,
    name: &'s StableName,
    done: &Kept<NameRef>,
) -> Result<StableName, Stopped<'s, NamingError>> {
    let bug = |what| Stopped::Refused(NamingError::Emission { what });
    let mut tails: Vec<&[RoleSeg]> = Vec::new();
    let mut at = name;
    let (mut path, foot_tail) = loop {
        if at.node != node {
            return Err(bug(
                "a union fold's table carries a row minted by another node",
            ));
        }
        let Some((head, mut tail)) = at.path.split_first() else {
            return Err(bug("a union fold's table carries a name with no role"));
        };
        let path = match head {
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
            RoleSeg::FromA(inner) | RoleSeg::FromB(inner) => {
                tails.push(tail);
                at = inner;
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
                    return Err(bug(FOREIGN));
                }
                let mut lines = vec![seam_line(a, b, done)?];
                for seg in std::mem::take(&mut tail) {
                    let RoleSeg::Seam { a, b } = seg else {
                        return Err(bug(FOREIGN));
                    };
                    lines.push(seam_line(a, b, done)?);
                }
                lines
            }
            RoleSeg::Seam { a, b } => {
                vec![seam_line(a, b, done)?]
            }
            // An F7 merged face: its constituents are result-face names in
            // the minting node's space (N3), so they stay in this union's
            // space, each collapsed by this same rule.
            //
            // The pair emitter mints `Merged` for the kernel's merge
            // groups — faces that share a recipe source, or a declared
            // coincidence, which a union carries through its own
            // `declare` input — so a fold step's table carries these rows.
            //
            // The constituent set is FLAT (N3): a constituent is never
            // itself a bare merged face. The mint (`emit_topo`'s
            // merge-group loop) holds that at the first door; this is the
            // same rule read at the union's second door — a constituent
            // that collapses to a bare merged face is refused as the
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
        break (path, tail);
    };
    for seg in core::iter::once(foot_tail)
        .chain(tails.into_iter().rev())
        .flatten()
    {
        path.push(match seg {
            // The walls are OPERAND-space names (N2) — this node's
            // space, like every other embedded name here — so they
            // collapse the same way.
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
            // The rank is a place along the fold's direction; the
            // canonicalization re-reads it.
            RoleSeg::Fragment(Qualifier::OrderAlong { .. }) => seg.clone(),
            // A split's qualifier, which no boolean table carries.
            RoleSeg::Fragment(Qualifier::Keeps(_)) => return Err(bug(FOREIGN)),
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
                    path: vec![RoleSeg::LateralEdge(
                        crate::names::role::ProfileVertexRef::Piece {
                            step: crate::node::StepId(0),
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

    /// Member `m`'s first lateral edge's start vertex, as `member_view`
    /// keys it.
    fn member_vertex(union: RecipeNodeId, m: u64) -> StableName {
        let mut v = member_edge(union, m);
        v.kind = EntityKind::Vertex;
        v
    }

    /// **A seam edge piece whose pair canonicalization swaps keeps its
    /// ends.** The emitter's A side here is member 5's face and its B
    /// side member 2's; in name order member 2 comes first, and the
    /// piece's `Ends`, which orders nothing, comes through as it was
    /// written, each end collapsed.
    #[test]
    fn a_swapped_seam_edge_piece_keeps_its_ends() {
        let union = RecipeNodeId(9);
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

    /// **A later step's pieces of a seam edge carry no rank to re-read**:
    /// the seam's pair goes into name order and the piece's `Ends`
    /// stands, whichever side the emitter wrote first.
    #[test]
    fn a_descent_through_a_swapped_seam_edge_keeps_its_ends() {
        let union = RecipeNodeId(9);
        let ends = vec![member_vertex(union, 3), member_vertex(union, 4)];
        let mut out = Vec::new();
        for swap in [true, false] {
            let piece = StableName {
                kind: EntityKind::Edge,
                node: union,
                path: vec![
                    RoleSeg::FromA(seam_edge(union, swap).into()),
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
        let line = seam(through_a(seam_edge(union, true)), member_cap(union, 7));
        for (rank, of) in [(2, 2), (0, 0)] {
            let err = collapse(
                union,
                &ranked(EntityKind::Vertex, union, line.clone(), rank, of),
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
                .mvfs(geom_core::Point3::new(x, 0.0, 0.0), true)
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

    /// **The fold's tie over two pieces of a member edge is grouped
    /// under the member edge**, as one group descended from a tie: the
    /// pieces are told apart by their ends afterwards, so no rule has to
    /// pick one edge, and the fold's qualifier is not carried.
    #[test]
    fn a_tied_member_edge_piece_is_grouped_under_its_member_edge() {
        let (union, m) = (RecipeNodeId(9), RecipeNodeId(4));
        let (body, k0, k1) = two_edge_body();
        let edge = StableName {
            kind: EntityKind::Edge,
            node: m,
            path: vec![RoleSeg::LateralEdge(
                crate::names::role::ProfileVertexRef::Piece {
                    step: crate::node::StepId(0),
                    role: crate::names::PieceRole::Leg,
                },
            )],
        };
        let mut piece = member_name(union, m, &edge);
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
            node: m,
            body: &body,
            table: &member_table,
        }];
        let parents = Parents::empty();
        let flush = Flush::of(union, &body, &members, &parents, bnd).unwrap();
        let (rest, groups) = group_member_edges(tied_rows, Vec::new(), &flush).unwrap();
        assert_eq!(rest.iter().count(), 0, "the row was not taken into a group");
        let [group] = groups.as_slice() else {
            panic!("one group, got {}", groups.len());
        };
        assert_eq!(group.base, member_name(union, m, &edge));
        assert!(group.from_tie, "the group does not descend from the tie");
        assert_eq!(group.edges, vec![k0, k1]);
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
                .mvfs(geom_core::Point3::new(0.0, 0.0, 0.0), true)
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
                body.mvfs(geom_core::Point3::new(x, 0.0, 0.0), true)
                    .expect("mvfs births a lone vertex")
                    .vertex
            };
            let (low, high) = (lone(0.25), lone(0.75));
            let at = |v| {
                crate::names::emit_topo::param_along(
                    &member_body,
                    edge,
                    vertex_point(&body, v).unwrap(),
                )
                .unwrap()
                .unwrap()
            };
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
        fn piece(&self, k: u64) -> StableName {
            let mut n = member_edge(self.union, 5);
            n.path.push(RoleSeg::Fragment(Qualifier::Ends(vec![
                member_vertex(self.union, 10 + k),
                member_vertex(self.union, 11 + k),
            ])));
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
            let rows = rows
                .into_iter()
                .map(|(name, v)| {
                    let at = crate::names::table::EntityRef {
                        body: 0,
                        key: EntityKey::Vertex(v),
                    };
                    (name, Entry::Unique(at), false)
                })
                .collect();
            let bnd = geom_core::Band::new(1e-9, 1e-6).unwrap();
            let members = self.members();
            let parents = Parents::empty();
            let flush = Flush::of(self.union, &self.body, &members, &parents, bnd)?;
            cite_member_edges(NameTable::new(), rows, &self.body, &members, &flush, bnd)
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
        let run = |k: u64| {
            let one = g.named(g.piece(k), None);
            let mut path = one.into_path();
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
        let carrying = |k: u64| StableName {
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

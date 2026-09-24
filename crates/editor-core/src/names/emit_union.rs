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
//! consumers of one rewrite): a fold-table name is rewritten by descending its
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
//! the tail. The pair emitter ranks a seam edge along the line
//! `n_a × n_b`, which is oriented by side. Where the canonical order
//! swaps the sides, that line reverses, so the edge's
//! `Fragment(OrderAlong)` rank is read from the other end. See
//! [`RankRule`].
//!
//! One more rewrite happens at the END, on the published table only:
//! the pieces of each member EDGE are ranked once, over the finished
//! body, along that edge's own direction ([`rank_member_edges`]). The
//! fold ranks them at whichever step cut them, and a rank written that
//! way is fold history — the same name would denote different pieces in
//! different member orders. A refusal raised mid-fold names the fold's
//! own ranks, since the pieces are not final there.
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

use crate::names::discriminate::{band, order_along};
use crate::names::emit::{NamingError, check_total};
use crate::names::emit_topo::{edge_dir, edge_extent};
use crate::names::role::{
    EntityKind, NameRef, Qualifier, RoleSeg, StableName, never_in_a_boolean_table,
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
    let t = collapse_table(node, folded)?;
    let t = rank_member_edges(t, body, members, band(tol)?)?;
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

/// **The pieces of one member's edge are ranked once, over the
/// finished body, along that edge's own direction.**
///
/// The fold ranks a member edge's pieces at whichever step cuts it
/// (`emit_topo`'s descent groups), and a later step cutting a piece
/// ranks ITS pieces under the first rank. Which step cuts where depends
/// on member order, and so does which pieces the member keeps where it
/// runs flush against another member (the coincident run is named for
/// whichever member the fold reached first). A rank written that way
/// is fold history, and the same name lands on different pieces in
/// different orders.
///
/// So the published table re-ranks: every row
/// `FromMember(m, e)` followed only by `Fragment(OrderAlong)` ranks, for
/// an EDGE `e`, is one piece of `m`'s edge `e`, and the group is ranked
/// afresh along `e`'s own oriented carrier in `m`'s own body — one
/// `Fragment(OrderAlong { rank, of })` over the pieces the body has,
/// or none when one piece remains. The pieces are disjoint stretches
/// of one straight edge, so the order along it is total; a pair the
/// band cannot order refuses typed ([`order_along`]), and a genuine
/// overlap ties, as the fold's own ranker does.
///
/// A group holding a TIED row, or a member edge the member's table
/// does not name uniquely, is left as the fold named it: there is no
/// entity to rank.
fn rank_member_edges<T: geom_core::Decide>(
    t: NameTable,
    body: &topo::Body<T>,
    members: &[Member<'_, T>],
    bnd: geom_core::Band,
) -> Result<NameTable, NamingError> {
    use std::collections::BTreeMap;
    let bug = |what| NamingError::Emission { what };
    // (member, member edge) → the group's rows, each with its entity.
    type Key = (RecipeNodeId, StableName);
    let mut groups: BTreeMap<Key, Vec<(StableName, Entry)>> = BTreeMap::new();
    let mut out = NameTable::new();
    for (name, entry) in t.iter() {
        match member_edge_piece(name) {
            Some(key) => groups
                .entry(key)
                .or_default()
                .push((name.clone(), entry.clone())),
            None => match entry {
                Entry::Unique(e) => out.insert(name.clone(), *e)?,
                Entry::Tied(es) => out.insert_tied(name.clone(), es.clone())?,
            },
        }
    }
    for ((member, edge), rows) in groups {
        let keep = |out: &mut NameTable, rows: Vec<(StableName, Entry)>| {
            for (name, entry) in rows {
                match entry {
                    Entry::Unique(e) => out.insert(name, e)?,
                    Entry::Tied(es) => out.insert_tied(name, es)?,
                }
            }
            Ok::<(), NamingError>(())
        };
        let source =
            members
                .iter()
                .find(|m| m.node == member)
                .and_then(|m| match m.table.lookup(&edge) {
                    Some(Entry::Unique(e)) => match e.key {
                        EntityKey::Edge(k) => Some((m.body, k)),
                        _ => None,
                    },
                    _ => None,
                });
        let pieces: Option<Vec<_>> = rows
            .iter()
            .map(|(_, entry)| match entry {
                Entry::Unique(e) => match e.key {
                    EntityKey::Edge(k) => Some((*e, k)),
                    _ => None,
                },
                Entry::Tied(_) => None,
            })
            .collect();
        let (Some((member_body, member_edge)), Some(pieces)) = (source, pieces) else {
            keep(&mut out, rows)?;
            continue;
        };
        let head = StableName {
            kind: EntityKind::Edge,
            node: rows[0].0.node,
            path: vec![rows[0].0.path[0].clone()],
        };
        if pieces.len() == 1 {
            out.insert(head, pieces[0].0)?;
            continue;
        }
        let dir = edge_dir(member_body, member_edge)?;
        let extents = pieces
            .iter()
            .map(|&(_, k)| edge_extent(body, k, dir))
            .collect::<Result<Vec<_>, _>>()?;
        match order_along(&extents, bnd)? {
            Some(ranks) => {
                let of = u32::try_from(pieces.len())
                    .map_err(|_| bug("a member edge in more pieces than a rank can count"))?;
                for (&(e, _), rank) in pieces.iter().zip(ranks) {
                    let mut name = head.clone();
                    name.path
                        .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }));
                    out.insert(name, e)?;
                }
            }
            None => out.insert_tied(head, pieces.iter().map(|&(e, _)| e).collect())?,
        }
    }
    Ok(out)
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

/// A whole fold table in the union's published space — the rewrite
/// [`name_union`] publishes, without the totality check.
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
/// The same rewrite [`name_union`] applies to a whole table, exposed
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

/// One fold-table name, keyed by member.
///
/// Returns a name in the UNION's own space (`node` is the union's, as
/// every fold row's already is): either one [`RoleSeg::FromMember`],
/// [`RoleSeg::Seam`], [`RoleSeg::Merged`] or [`RoleSeg::OutputBody`]
/// head followed by the `Fragment` discriminators the fold
/// accumulated, outermost step last — or, for a seam JUNCTION vertex,
/// a sorted run of two or more `Seam` lines and nothing else.
///
/// The tail keeps its segments, but not always their values. A `Seam`
/// head's [`RankRule`] says what its own `OrderAlong` rank becomes once
/// the pair is in canonical order, and that rank can be rewritten.
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
    let (mut path, ranks) = match head {
        // Already member-keyed: the foot of a descent chain, put there
        // by `member_view` before the step ran.
        RoleSeg::FromMember { .. } => (vec![head.clone()], RankRule::Keep),
        // The accumulated body's own name at every step, and the
        // union's at the last one: one body out, one output-body row.
        RoleSeg::OutputBody => (vec![RoleSeg::OutputBody], RankRule::Keep),
        // The descent. Every operand of every step is in this node's
        // space, so a `FromA`/`FromB` argument is always an earlier
        // step's row: descended THROUGH, carrying its own
        // discriminators out with it.
        RoleSeg::FromA(inner) | RoleSeg::FromB(inner) => {
            (collapse(node, inner)?.path, RankRule::Keep)
        }
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
            let mut lines = vec![seam_line(node, a, b)?.seg];
            for seg in std::mem::take(&mut tail) {
                let RoleSeg::Seam { a, b } = seg else {
                    return Err(bug(FOREIGN));
                };
                lines.push(seam_line(node, a, b)?.seg);
            }
            lines.sort_unstable();
            if lines.windows(2).any(|w| w[0] == w[1]) {
                return Err(bug(JUNCTION_LINES_COLLIDE));
            }
            (lines, RankRule::Keep)
        }
        RoleSeg::Seam { a, b } => {
            let line = seam_line(node, a, b)?;
            let ranks = RankRule::of_seam(name.kind, &line);
            (vec![line.seg], ranks)
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
            (vec![RoleSeg::Merged(set)], RankRule::Keep)
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
                RoleSeg::Fragment(ranks.apply(*rank, *of)?)
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

/// Which way a collapsed seam pair's sides run, relative to the order
/// the pair emitter minted them in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PairOrder {
    /// The emitter's A side is the canonical first.
    AsMinted,
    /// Canonicalization put the emitter's B side first.
    Swapped,
}

/// One seam line between two members, in the union's space.
struct SeamLine {
    /// The canonical `Seam` segment.
    seg: RoleSeg,
    /// Whether canonicalizing it swapped the emitter's sides.
    order: PairOrder,
    /// Both sides name EDGES (a vertex where two edges cross).
    edge_pair: bool,
}

/// What a collapsed head does to the `Fragment(OrderAlong)` rank in its
/// own tail. A rank is a place along a direction, so this depends on
/// whether putting the head in canonical order moved that direction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RankRule {
    /// The rank was taken along a direction the collapse does not
    /// change: every non-`Seam` head, a seam edge whose pair stayed in
    /// minted order, and a seam VERTEX group whose parents are an edge
    /// and a face. Such a group is ranked along the parent edge's own
    /// carrier in that edge's operand body, which is the same edge
    /// whichever side it sits on.
    Keep,
    /// A seam EDGE whose pair canonicalization swapped. The pair emitter
    /// ranks a seam chain along `n_a × n_b`, with the A-side face's
    /// outward normal first. With the sides swapped that line is
    /// negated, and the rank reads from the other end: `of − 1 − rank`.
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
    /// The rule for a `Seam` head of `kind`.
    fn of_seam(kind: EntityKind, line: &SeamLine) -> Self {
        match (kind, line.order, line.edge_pair) {
            (EntityKind::Edge, PairOrder::Swapped, _) => Self::Reverse,
            (EntityKind::Vertex, _, true) => Self::Refuse,
            _ => Self::Keep,
        }
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
/// record. Anything the pair emitter oriented by side has to follow
/// the swap, and [`SeamLine::order`] is what reports it.
fn seam_line(node: RecipeNodeId, a: &StableName, b: &StableName) -> Result<SeamLine, NamingError> {
    let (x, y) = (collapse(node, a)?, collapse(node, b)?);
    let edge_pair = x.kind == EntityKind::Edge && y.kind == EntityKind::Edge;
    let (order, (a, b)) = if x > y {
        (PairOrder::Swapped, (y, x))
    } else {
        (PairOrder::AsMinted, (x, y))
    };
    Ok(SeamLine {
        seg: RoleSeg::Seam {
            a: NameRef::new(a),
            b: NameRef::new(b),
        },
        order,
        edge_pair,
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
}

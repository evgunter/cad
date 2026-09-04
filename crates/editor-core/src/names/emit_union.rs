//! **The n-ary union's naming** ([`crate::Node::Union`]; DM4).
//!
//! The node's VALUE is a fold of the pair verb, and the fold's own
//! tables are the pair emitter's: every step wraps the accumulated
//! body's names in `FromA` and the joining member's in `FromB`, so
//! after `n` steps a member's entity carries a descent chain whose
//! LENGTH is the member's position in the list. That chain is exactly
//! what a union must not record — it is why a pairwise chain renames
//! every earlier member when one link is removed.
//!
//! Two halves take it out, and they are two because the fold's steps
//! and the fold's result need different things.
//!
//! **Going in** ([`member_view`]): each member's table is presented to
//! its fold step ALREADY member-keyed — every row wrapped in one
//! [`RoleSeg::FromMember`] carrying that member's node id. So every
//! operand of every step is in the union's own name space, and every
//! name the pair emitter embeds — a seam's two sides, a merge's
//! constituents, a fragment's discriminator partners — is a name that
//! already says which member it came from. Nothing downstream has to
//! guess a member from an inner name, which is exactly what cannot be
//! done: a pass-through op mints no name (N1), so N placements of one
//! prototype carry N identical tables.
//!
//! **Coming out** ([`name_union`]): the fold's final table is rewritten
//! once, collapsing each entity's `FromA`/`FromB` descent chain down to
//! the `FromMember` at its foot — one wrapper, whatever the depth.
//! `Seam`, `Merged` and `Fragment` keep the shapes the pair emitter
//! minted them in, with the names they embed collapsed by the same
//! rule.
//!
//! # How an intermediate row is told from a member's row
//!
//! By the head segment, and by the minting node under it. Every row of
//! every fold step is emitted under the UNION's id, and a member-keyed
//! row's head is `FromMember`: a `FromA`/`FromB` is therefore always
//! another step's row and is descended through, and the descent stops
//! at the first `FromMember` it reaches.

use std::sync::Arc;

use crate::names::emit::{NamingError, check_total};
use crate::names::role::{Qualifier, RoleSeg, StableName};
use crate::names::table::{Entry, NameTable};
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
    let mut view = NameTable::new();
    for (name, entry) in table.iter() {
        let keyed = StableName {
            kind: name.kind,
            node: union,
            path: vec![RoleSeg::FromMember {
                member,
                of: Box::new(name.clone()),
            }],
        };
        match entry {
            Entry::Unique(e) => view.insert(keyed, *e),
            Entry::Tied(es) => view.insert_tied(keyed, es.clone()),
        }?;
    }
    Ok(view)
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
) -> Result<Arc<NameTable>, NamingError> {
    let mut t = NameTable::new();
    for (name, entry) in folded.iter() {
        let keyed = collapse(node, name)?;
        match entry {
            Entry::Unique(e) => t.insert(keyed, *e),
            Entry::Tied(es) => t.insert_tied(keyed, es.clone()),
        }?;
    }
    check_total(&t, body, 0)?;
    Ok(Arc::new(t))
}

/// The emission bug this module can raise: a fold table carrying a
/// segment the pair emitter does not mint.
const FOREIGN: &str = "a union fold's table carries a segment the boolean emitter does not mint";

/// One fold-table name, keyed by member.
///
/// Returns a name in the UNION's own space (`node` is the union's, as
/// every fold row's already is): the head segment is
/// [`RoleSeg::FromMember`], [`RoleSeg::Seam`], [`RoleSeg::Merged`] or
/// [`RoleSeg::OutputBody`], followed by the `Fragment` discriminators
/// the fold accumulated, outermost step last.
fn collapse(node: RecipeNodeId, name: &StableName) -> Result<StableName, NamingError> {
    let bug = |what| NamingError::Emission { what };
    if name.node != node {
        return Err(bug(
            "a union fold's table carries a row minted by another node",
        ));
    }
    let Some((head, tail)) = name.path.split_first() else {
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
        // A seam between two members. The pair emitter's `a`/`b` are
        // the crossing entities in the two OPERANDS' tables, which are
        // this node's space on both sides, so each is collapsed the
        // same way every other row is. The pair is then CANONICALIZED
        // by name order: a union is commutative, so "which side" would
        // record only which of the two members the fold reached first,
        // which is the position this node exists not to record.
        RoleSeg::Seam { a, b } => {
            let (x, y) = (collapse(node, a)?, collapse(node, b)?);
            let (a, b) = if x <= y { (x, y) } else { (y, x) };
            vec![RoleSeg::Seam {
                a: Box::new(a),
                b: Box::new(b),
            }]
        }
        // An F7 merged face: its constituents are result-face names in
        // the minting node's space (N3), so they stay in this union's
        // space, each collapsed by this same rule. Re-sorted and
        // deduplicated because collapsing may reorder them — the
        // constituent SET is the name.
        RoleSeg::Merged(constituents) => {
            let mut set = constituents
                .iter()
                .map(|c| collapse(node, c))
                .collect::<Result<Vec<_>, _>>()?;
            set.sort();
            set.dedup();
            vec![RoleSeg::Merged(set)]
        }
        _ => return Err(bug(FOREIGN)),
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
            RoleSeg::Fragment(q @ Qualifier::OrderAlong { .. }) => RoleSeg::Fragment(q.clone()),
            _ => return Err(bug(FOREIGN)),
        });
    }
    Ok(StableName {
        kind: name.kind,
        node,
        path,
    })
}

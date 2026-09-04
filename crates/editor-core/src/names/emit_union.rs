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
//! So the fold's final table is rewritten here, once, into names keyed
//! by MEMBER: each surviving entity's descent chain collapses to the
//! member it came from and mints one [`RoleSeg::FromMember`] wrapping
//! that member's name in the member's own table — one wrapper,
//! whatever the depth. Everything the pair emitter minted that is not
//! a descent is kept as it minted it: a seam's `Seam { a, b }`, a
//! merge's `Merged`, a fragment's `Fragment` qualifier, each with the
//! names it embeds un-nested by the same rule.
//!
//! # How an intermediate row is told from a member's row
//!
//! By the minting node, which is what a name's `node` field is for.
//! Every row of every fold step is emitted under the UNION's id, and
//! no member's own name can carry that id — the union node mints only
//! its own names. So a `FromA`/`FromB` argument whose `node` is the
//! union's is another step's row and is descended through; anything
//! else is a member's own name and is where the descent stops.

use std::sync::Arc;

use crate::names::emit::{NamingError, check_total};
use crate::names::role::{Qualifier, RoleSeg, StableName};
use crate::names::table::{Entry, NameTable};
use crate::node::RecipeNodeId;

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
        }
        ?;
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
        // The accumulated body's own name at every step, and the
        // union's at the last one: one body out, one output-body row.
        RoleSeg::OutputBody => vec![RoleSeg::OutputBody],
        // The descent. An argument minted by this node is an earlier
        // step's row and is descended THROUGH, carrying its own
        // discriminators out with it; anything else is the member's
        // own name and is what the one wrapper holds.
        RoleSeg::FromA(inner) | RoleSeg::FromB(inner) => {
            if inner.node == node {
                collapse(node, inner)?.path
            } else {
                vec![RoleSeg::FromMember(inner.clone())]
            }
        }
        // A seam between two members. The pair emitter's `a`/`b` are
        // the crossing entities in the two OPERANDS' tables, and at a
        // fold step past the first the A operand is the accumulation —
        // so each side is un-nested to the member entity it came from.
        // The pair is then CANONICALIZED by name order: a union is
        // commutative, so "which side" would record only which of the
        // two members the fold reached first, which is the position
        // this node exists not to record.
        RoleSeg::Seam { a, b } => {
            let (x, y) = (unnest(node, a)?, unnest(node, b)?);
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
            // names (N2), so a step past the first records partners in
            // the accumulation's space: un-nested like every other
            // embedded name. The verdicts are untouched — they are the
            // recorded predicate evidence, not a reference.
            RoleSeg::Fragment(Qualifier::SideOf(vec)) => {
                let partners = vec
                    .iter()
                    .map(|(n, v)| Ok((unnest(node, n)?, *v)))
                    .collect::<Result<Vec<_>, NamingError>>()?;
                RoleSeg::Fragment(Qualifier::SideOf(partners))
            }
            RoleSeg::Fragment(q @ Qualifier::OrderAlong { .. }) => {
                RoleSeg::Fragment(q.clone())
            }
            _ => return Err(bug(FOREIGN)),
        });
    }
    Ok(StableName {
        kind: name.kind,
        node,
        path,
    })
}

/// An embedded name, un-nested: the MEMBER's own name where the chain
/// bottoms out at a member entity, and this union's collapsed name
/// where it bottoms out at something the union itself minted (a seam
/// edge crossed by a later seam vertex, a fragment of a member face).
/// The second case is why this is not simply "descend to the bottom":
/// there is no member entity to name, and the union's own row for it
/// is the honest reference.
fn unnest(node: RecipeNodeId, name: &StableName) -> Result<StableName, NamingError> {
    if name.node != node {
        return Ok(name.clone());
    }
    let keyed = collapse(node, name)?;
    match keyed.path.as_slice() {
        [RoleSeg::FromMember(inner)] => Ok((**inner).clone()),
        _ => Ok(keyed),
    }
}

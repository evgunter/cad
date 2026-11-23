//! A refactoring's node map as the list Python reads.
//!
//! A `NodeId` has no order a Python caller can read, so the list's own
//! order is the only one it gets, and that is the document's: the pairs
//! in the order the mapped-to document holds their targets, which is
//! their id order.
//!
//! Python-independent, so the default build tests it.

use pncad::document::{NodeMap, ProfileDoc, RecipeNodeId};

/// `map`'s pairs in the order `doc` holds their targets: by target id.
///
/// # Panics
///
/// When `doc` does not hold a target. Every pair a split or an inline
/// maps lands a node in the document it builds, so a target missing
/// from it is a kernel bug, not a state to sort somewhere.
#[must_use]
pub fn in_target_id_order(map: &NodeMap, doc: &ProfileDoc) -> Vec<(RecipeNodeId, RecipeNodeId)> {
    let mut pairs: Vec<(RecipeNodeId, RecipeNodeId)> = map
        .iter()
        .map(|(&from, &to)| {
            if doc.node(to).is_none() {
                unreachable!("a node map lands {to:?} in a document that does not hold it")
            }
            (from, to)
        })
        .collect();
    pairs.sort_unstable_by_key(|&(_, to)| to);
    pairs
}

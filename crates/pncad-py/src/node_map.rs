//! A refactoring's node map as the list Python reads.
//!
//! A `NodeId` has no order a Python caller can read, so the list's own
//! order is the only one it gets, and that is the document's: the pairs
//! in the order the mapped-to document holds their targets. The map is
//! keyed by id, and an id is a digest, so the map's own order is none a
//! reader can follow.
//!
//! Python-independent, so the default build tests it.

use pncad::document::{NodeMap, ProfileDoc, RecipeNodeId};

/// `map`'s pairs in the order `doc` holds their targets.
///
/// # Panics
///
/// When `doc` does not hold a target. Every pair a split or an inline
/// maps lands a node in the document it builds, so a target missing
/// from it is a kernel bug, not a state to sort somewhere.
#[must_use]
pub fn in_document_order(map: &NodeMap, doc: &ProfileDoc) -> Vec<(RecipeNodeId, RecipeNodeId)> {
    let at = doc.positions();
    let mut pairs: Vec<(usize, RecipeNodeId, RecipeNodeId)> = map
        .iter()
        .map(|(&from, &to)| {
            let Some(&position) = at.get(&to) else {
                unreachable!("a node map lands {to:?} in a document that does not hold it")
            };
            (position, from, to)
        })
        .collect();
    pairs.sort_unstable_by_key(|&(position, _, _)| position);
    pairs.into_iter().map(|(_, from, to)| (from, to)).collect()
}

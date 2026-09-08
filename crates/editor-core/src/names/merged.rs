//! **Reading a merged face's flat constituent set** (N3): the two
//! questions every consumer of a `Merged` row asks, answered once.
//!
//! A merged face's name is a FLAT set of face names; a merge over a
//! merged face lists the faces, never the merge. Two things follow
//! for a reader. A name that is itself a merged face — read THROUGH
//! its `FromA`/`FromB` descent wrappers, since a face carried through
//! untouched booleans is still that face — stands for its
//! constituents re-wrapped by that same chain
//! ([`constituents_through_wrappers`]); and a merged row COVERS a name
//! when the name is one of its constituents, or when the name is a
//! merged face all of whose re-wrapped constituents are ([`covers`]).
//! The mint (`emit_topo`'s merge-group loop) uses the first to decide
//! what it publishes; the N3 offer sites and the union's look-through
//! use the second to read what was published. Neither flattens a
//! name: the set is flat because the mint made it so.

use super::role::{EntityKind, RoleSeg, StableName};

/// The emission bug a nested merged face is — a `Merged` constituent
/// that is itself a merged face, through any wrapping — refused at
/// the mint (`emit_topo`) and again at the union's collapse
/// (`emit_union`): one rule, two doors.
pub(crate) const NESTED_MERGED: &str =
    "a boolean table carries a merged face whose constituent is itself a merged face";

/// The constituents of a merged face read through its descent
/// wrappers, each re-wrapped by that same chain — or `None` when the
/// name, peeled to its foot, is not a merged face.
///
/// Only a bare `FromA`/`FromB` chain is peeled: a foot that carries a
/// tail (`[Merged(cs), Fragment(q)]`) is a FRAGMENT of a merged face,
/// a face in its own right, and is left whole.
pub(crate) fn constituents_through_wrappers(name: &StableName) -> Option<Vec<StableName>> {
    let rewrap = |inner: Vec<StableName>, side: fn(Box<StableName>) -> RoleSeg| {
        inner
            .into_iter()
            .map(|c| StableName {
                kind: EntityKind::Face,
                node: name.node,
                path: vec![side(Box::new(c))],
            })
            .collect()
    };
    match name.path.as_slice() {
        [RoleSeg::Merged(cs)] => Some(cs.clone()),
        [RoleSeg::FromA(inner)] => {
            constituents_through_wrappers(inner).map(|cs| rewrap(cs, RoleSeg::FromA))
        }
        [RoleSeg::FromB(inner)] => {
            constituents_through_wrappers(inner).map(|cs| rewrap(cs, RoleSeg::FromB))
        }
        _ => None,
    }
}

/// True iff a merged row whose constituent set is `set` covers
/// `name`: `name` is a constituent, or `name` is a merged face (read
/// through its wrappers) every one of whose re-wrapped constituents
/// is. `set` is sorted, as every minted set is (name order), so both
/// halves are binary searches.
pub(crate) fn covers(set: &[StableName], name: &StableName) -> bool {
    if set.binary_search(name).is_ok() {
        return true;
    }
    match constituents_through_wrappers(name) {
        Some(cs) => !cs.is_empty() && cs.iter().all(|c| set.binary_search(c).is_ok()),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::names::role::CapEnd;
    use crate::node::RecipeNodeId;

    fn face(node: u64, path: Vec<RoleSeg>) -> StableName {
        StableName {
            kind: EntityKind::Face,
            node: RecipeNodeId(node),
            path,
        }
    }

    fn cap(node: u64) -> StableName {
        face(node, vec![RoleSeg::Cap(CapEnd::End)])
    }

    fn from_a(node: u64, inner: StableName) -> StableName {
        face(node, vec![RoleSeg::FromA(Box::new(inner))])
    }

    fn from_b(node: u64, inner: StableName) -> StableName {
        face(node, vec![RoleSeg::FromB(Box::new(inner))])
    }

    fn merged(node: u64, mut set: Vec<StableName>) -> StableName {
        set.sort();
        face(node, vec![RoleSeg::Merged(set)])
    }

    /// The outer row of a boolean over a boolean: the inner merge's
    /// constituents re-wrapped, plus the partner. It covers each
    /// constituent, the consumed inner merged face wrapped once, and
    /// the same face carried through one more untouched step — and
    /// not a face it never listed.
    #[test]
    fn a_flat_row_covers_its_constituents_and_the_merged_faces_they_came_from() {
        let inner = merged(7, vec![from_a(7, cap(2)), from_b(7, cap(5))]);
        let outer_set = {
            let mut s = vec![
                from_a(12, from_a(7, cap(2))),
                from_a(12, from_b(7, cap(5))),
                from_b(12, cap(10)),
            ];
            s.sort();
            s
        };
        assert!(covers(&outer_set, &from_a(12, from_a(7, cap(2)))));
        assert!(covers(&outer_set, &from_b(12, cap(10))));
        assert!(covers(&outer_set, &from_a(12, inner.clone())));
        assert!(!covers(&outer_set, &from_b(12, inner.clone())));
        assert!(!covers(&outer_set, &from_a(12, from_a(7, cap(3)))));
        // Carried through one more untouched step, as operand B.
        let carried_set = {
            let mut s = vec![
                from_a(17, from_b(12, from_a(7, cap(2)))),
                from_a(17, from_b(12, from_b(7, cap(5)))),
                from_b(17, cap(15)),
            ];
            s.sort();
            s
        };
        assert!(covers(&carried_set, &from_a(17, from_b(12, inner))));
        // A fragment of a merged face is a face, not a merge: it is
        // covered only as a constituent.
        let fragment = face(
            12,
            vec![
                RoleSeg::Merged(vec![from_a(12, cap(2))]),
                RoleSeg::Fragment(crate::names::role::Qualifier::OrderAlong { rank: 0, of: 2 }),
            ],
        );
        assert!(constituents_through_wrappers(&fragment).is_none());
        assert!(!covers(&outer_set, &fragment));
    }
}

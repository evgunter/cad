//! **A union over a union publishes FLAT sets** (N3,
//! `crates/editor-core/src/names/README.md`): a merged face or a joined
//! edge of the outer union that takes in one of the inner union's lists
//! the inner set's constituents, each re-wrapped as the outer union's
//! `FromMember(inner, c)`, never `FromMember(inner, Merged{…})` as one
//! constituent — in every member order of both unions.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::docm7_union_declare::{declared_union, failure, flush_pairs, run};
use crate::emit_shared_rim_several::{Bx, document, permutations};
use crate::emit_union_rim_piece_ranks::signature;
use crate::fixture::{flush_segs, fname, member_entity, table};

use editor_core::{EntityKind, RecipeNodeId, RoleSeg, SitedRef, StableName};

/// `a` and `b` overlap and are declared flush in the inner union, so its
/// two y-walls and two caps are merged faces and its four x-running rims
/// joined edges; `c` overlaps `b` past `a`'s end and is declared flush
/// with those merged faces in the outer union.
const A: Bx = ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
const B: Bx = ((0.5, 1.5), (0.0, 1.0), (0.0, 1.0));
const C: Bx = ((1.0, 2.0), (0.0, 1.0), (0.0, 1.0));

/// Whether `n`, read through every `FromA`/`FromB`/`FromMember` wrapper,
/// is a bare merged name: the constituent N3 forbids.
fn bare_merge_through_wrappers(n: &StableName) -> bool {
    match n.path.as_slice() {
        [RoleSeg::Merged(_)] => true,
        [RoleSeg::FromA(inner) | RoleSeg::FromB(inner)]
        | [RoleSeg::FromMember { of: inner, .. }] => bare_merge_through_wrappers(inner),
        _ => false,
    }
}

/// Every order of both unions: the order's label and the outer union's
/// signature, with the member ids `[a, b, c]` and the inner union.
fn orders() -> Vec<(String, BTreeMap<StableName, String>, Vec<RecipeNodeId>, RecipeNodeId)> {
    let (doc, ids) = document(&[A, B, C], &[0, 1, 2]);
    let mut out = Vec::new();
    for inner in permutations(&[0, 1]) {
        let io: Vec<_> = inner.iter().map(|&i| ids[i]).collect();
        let (d1, u1) = declared_union(
            doc.clone(),
            &io,
            flush_pairs(&doc, (ids[0], ids[0]), (ids[1], ids[1])),
        );
        // The inner union's merged face over `b`'s face of each family,
        // as its own table publishes it.
        let ev1 = run(&d1);
        assert!(failure(&ev1, u1).is_none(), "{inner:?}: the inner union refused");
        let merged_over = |seg: RoleSeg| -> StableName {
            let b_face = member_entity(u1, ids[1], fname(ids[1], seg), EntityKind::Face);
            table(&ev1, u1)
                .iter()
                .map(|(n, _)| n)
                .find(|n| matches!(n.path.as_slice(), [RoleSeg::Merged(set)] if set.contains(&b_face)))
                .unwrap_or_else(|| panic!("{inner:?}: no merged face holds {b_face:?}"))
                .clone()
        };
        let outer_pairs: Vec<(SitedRef, SitedRef)> = flush_segs(&doc, ids[1])
            .into_iter()
            .zip(flush_segs(&doc, ids[2]))
            .map(|(s, t)| {
                (
                    SitedRef::new(u1, merged_over(s)),
                    SitedRef::new(ids[2], fname(ids[2], t)),
                )
            })
            .collect();
        for outer in permutations(&[0, 1]) {
            let members: Vec<_> = outer.iter().map(|&i| [u1, ids[2]][i]).collect();
            let (docx, top) = declared_union(d1.clone(), &members, outer_pairs.clone());
            let ev = run(&docx);
            let at = format!("{inner:?}{outer:?}");
            assert!(
                failure(&ev, top).is_none(),
                "{at}: the outer union refused: {:?}",
                failure(&ev, top)
            );
            out.push((at, signature(&ev, top), ids.clone(), u1));
        }
    }
    out
}

/// **The outer union's merged faces and joined edges list faces and
/// edges, never the inner union's sets, and its table is one table in
/// every order.** Each of the four flush families is one face of the
/// outer union, `Merged` of `a`'s, `b`'s and `c`'s faces, the first two
/// through the inner union; each of the four x-running rims is one edge,
/// `Merged` of the three blocks' rims, so.
///
/// Red while `FromMember` is not read through: those sets list
/// `FromMember(inner, Merged{a's, b's})` and `c`'s, two constituents.
#[test]
fn a_union_over_a_union_publishes_flat_sets_in_every_order() {
    let orders = orders();
    assert_eq!(orders.len(), 4, "two inner orders by two outer orders");
    for (at, sig, ids, u1) in &orders {
        let mut three = BTreeMap::<EntityKind, usize>::new();
        for name in sig.keys() {
            for seg in &name.path {
                let RoleSeg::Merged(set) = seg else { continue };
                for c in set {
                    assert!(
                        !bare_merge_through_wrappers(c),
                        "{at}: {name:?} lists a merged name as one constituent: {c:?}"
                    );
                }
                let via_inner = set
                    .iter()
                    .filter(|c| matches!(c.path.as_slice(), [RoleSeg::FromMember { member, .. }] if member == u1))
                    .count();
                let via_c = set
                    .iter()
                    .filter(|c| matches!(c.path.as_slice(), [RoleSeg::FromMember { member, .. }] if *member == ids[2]))
                    .count();
                if set.len() == 3 && via_inner == 2 && via_c == 1 {
                    *three.entry(name.kind).or_default() += 1;
                }
            }
        }
        assert_eq!(
            three.get(&EntityKind::Face).copied().unwrap_or(0),
            4,
            "{at}: the four flush families are each one face of all three blocks"
        );
        assert_eq!(
            three.get(&EntityKind::Edge).copied().unwrap_or(0),
            4,
            "{at}: the four x-running rims are each one edge of all three blocks"
        );
    }
    let (first_at, first, ..) = &orders[0];
    for (at, sig, ..) in &orders[1..] {
        assert_eq!(first, sig, "{first_at} against {at}");
    }
}

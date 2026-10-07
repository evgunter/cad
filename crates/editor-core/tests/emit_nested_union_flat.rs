//! **A union over a union publishes FLAT sets** (N3,
//! `crates/editor-core/src/names/README.md`): a merged face or a joined
//! edge of the outer union that takes in one of the inner union's lists
//! the inner set's constituents, each re-wrapped as the outer union's
//! `FromMember(inner, c)`, never `FromMember(inner, Merged{…})` as one
//! constituent — in every member order of both unions.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::{BTreeMap, BTreeSet};

use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::emit_shared_rim_several::{Bx, document, permutations};
use crate::emit_union_rim_piece_ranks::signature;
use crate::fixture::{flush_segs, fname, member_entity, table};

use editor_core::{EntityKind, NodeErrorKind, RecipeNodeId, RoleSeg, SitedRef, StableName};

/// `a` and `b` overlap and are declared flush in the inner union, so its
/// two y-walls and two caps are merged faces and its four x-running rims
/// joined edges; `c` overlaps `b` past `a`'s end and is declared flush
/// with those merged faces in the outer union. `g`, a pillar standing
/// across the top's y = 0 rim past `b`'s end, is in the outer union too:
/// it notches the top and that wall without dividing either, and crosses
/// the rim, so the fold meets a joined edge of all three blocks where
/// `g` crosses it and reads its sense along it.
const A: Bx = ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
const B: Bx = ((0.5, 1.5), (0.0, 1.0), (0.0, 1.0));
const C: Bx = ((1.0, 2.0), (0.0, 1.0), (0.0, 1.0));
const G: Bx = ((1.6, 1.7), (-0.5, 0.5), (0.5, 3.0));

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
/// signature, with the member ids `[a, b, c, g]` and the inner union.
/// `c` and `g` are made after the inner union, so its member-keyed names
/// order before theirs and a set's reader meets its constituents first.
fn orders() -> Vec<(
    String,
    BTreeMap<StableName, String>,
    Vec<RecipeNodeId>,
    RecipeNodeId,
)> {
    let (doc, ab) = document(&[A, B], &[0, 1]);
    let mut out = Vec::new();
    for inner in permutations(&[0, 1]) {
        let io: Vec<_> = inner.iter().map(|&i| ab[i]).collect();
        let (d1, u1) = declared_union(
            doc.clone(),
            &io,
            flush_pairs(&doc, (ab[0], ab[0]), (ab[1], ab[1])),
        );
        let ((cx, cy, cz), (gx, gy, gz)) = (C, G);
        let (d1, c) = block(d1, cx, cy, cz.0, cz.1);
        let (d1, g) = block(d1, gx, gy, gz.0, gz.1);
        let ids = vec![ab[0], ab[1], c, g];
        // The inner union's merged face over `b`'s face of each family,
        // as its own table publishes it.
        let ev1 = run(&d1);
        assert!(
            failure(&ev1, u1).is_none(),
            "{inner:?}: the inner union refused"
        );
        let merged_over = |seg: RoleSeg| -> StableName {
            let b_face = member_entity(u1, ab[1], fname(ab[1], seg), EntityKind::Face);
            table(&ev1, u1)
                .iter()
                .map(|(n, _)| n)
                .find(|n| matches!(n.path.as_slice(), [RoleSeg::Merged(set)] if set.contains(&b_face)))
                .unwrap_or_else(|| panic!("{inner:?}: no merged face holds {b_face:?}"))
                .clone()
        };
        let outer_pairs: Vec<(SitedRef, SitedRef)> = flush_segs(&d1, ab[1])
            .into_iter()
            .zip(flush_segs(&d1, c))
            .map(|(s, t)| {
                (
                    SitedRef::new(u1, merged_over(s)),
                    SitedRef::new(c, fname(c, t)),
                )
            })
            .collect();
        for outer in permutations(&[0, 1, 2]) {
            let members: Vec<_> = outer.iter().map(|&i| [u1, c, g][i]).collect();
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
/// every order.** Each of the four flush families is one parent face of
/// the outer union, `Merged` of `a`'s, `b`'s and `c`'s faces, the first
/// two through the inner union; each of the four x-running rims is one
/// set, `Merged` of the three blocks' rims, so (`g` cuts the y = 0 top
/// rim, and its piece along all three blocks is a piece of that set).
///
/// Red while `FromMember` is not read through: those sets list
/// `FromMember(inner, Merged{a's, b's})` and `c`'s, two constituents.
#[test]
fn a_union_over_a_union_publishes_flat_sets_in_every_order() {
    let orders = orders();
    assert_eq!(orders.len(), 12, "two inner orders by six outer orders");
    for (at, sig, ids, u1) in &orders {
        let mut sets = BTreeMap::<EntityKind, BTreeSet<&[StableName]>>::new();
        for name in sig.keys() {
            for seg in &name.path {
                let RoleSeg::Merged(set) = seg else { continue };
                for c in set {
                    assert!(
                        !bare_merge_through_wrappers(c),
                        "{at}: {name:?} lists a merged name as one constituent: {c:?}"
                    );
                }
                let of = |m: RecipeNodeId| {
                    set.iter()
                        .filter(|c| matches!(c.path.as_slice(), [RoleSeg::FromMember { member, .. }] if *member == m))
                        .count()
                };
                if set.len() == 3 && of(*u1) == 2 && of(ids[2]) == 1 {
                    sets.entry(name.kind).or_default().insert(set);
                }
            }
        }
        let count = |kind| sets.get(&kind).map_or(0, BTreeSet::len);
        assert_eq!(
            count(EntityKind::Face),
            4,
            "{at}: the four flush families are each one face of all three blocks"
        );
        assert_eq!(
            count(EntityKind::Edge),
            4,
            "{at}: the four x-running rims are each one edge set of all three blocks"
        );
    }
    let (first_at, first, ..) = &orders[0];
    for (at, sig, ..) in &orders[1..] {
        assert_eq!(first, sig, "{first_at} against {at}");
    }
}

/// `d` overlaps `a` from the other end and is declared flush with the
/// inner union's merged faces as `c` is; `s` is a slab across the top
/// over `a`'s stretch, dividing it.
const D: Bx = ((-0.5, 0.25), (0.0, 1.0), (0.0, 1.0));
const S: Bx = ((0.3, 0.4), (-1.0, 2.0), (0.5, 3.0));

/// **No order of a union over a union, whose merged top a fold step
/// merges and then divides before its other declared partner joins,
/// drops that declaration as consumed.** The outer union holds the
/// inner one, `c` and `d`, each of the two declared flush with the
/// inner union's merged faces, and `s`. Where `d` joins and `s` divides
/// the merged top before `c` joins, the top survives in pieces, so the
/// declaration naming it refuses `ConsumedByFold { FragmentedMerge }`,
/// as a member's own face would; it is never read as consumed whole and
/// dropped, which leaves the step to meet an undeclared contact and
/// refuse as an emission bug. Where `s` divides it before either partner
/// joins it refuses `ConsumedByFold { Split }`; every other order
/// publishes.
#[test]
fn a_nested_unions_merged_face_divided_in_the_fold_is_never_read_as_consumed() {
    let (doc, ab) = document(&[A, B], &[0, 1]);
    let (d1, u1) = declared_union(
        doc.clone(),
        &ab,
        flush_pairs(&doc, (ab[0], ab[0]), (ab[1], ab[1])),
    );
    let mut d1 = d1;
    let mut more = Vec::new();
    for (x, y, z) in [C, D, S] {
        let (d, id) = block(d1, x, y, z.0, z.1);
        d1 = d;
        more.push(id);
    }
    let ev1 = run(&d1);
    assert!(failure(&ev1, u1).is_none(), "the inner union refused");
    let merged_over = |seg: RoleSeg| -> StableName {
        let b_face = member_entity(u1, ab[1], fname(ab[1], seg), EntityKind::Face);
        table(&ev1, u1)
            .iter()
            .map(|(n, _)| n)
            .find(|n| matches!(n.path.as_slice(), [RoleSeg::Merged(set)] if set.contains(&b_face)))
            .unwrap_or_else(|| panic!("no merged face holds {b_face:?}"))
            .clone()
    };
    let pairs: Vec<(SitedRef, SitedRef)> = more[..2]
        .iter()
        .flat_map(|&k| {
            flush_segs(&d1, ab[1])
                .into_iter()
                .zip(flush_segs(&d1, k))
                .map(move |(s, t)| (s, k, t))
        })
        .map(|(s, k, t)| {
            (
                SitedRef::new(u1, merged_over(s)),
                SitedRef::new(k, fname(k, t)),
            )
        })
        .collect();
    let outer = [u1, more[0], more[1], more[2]];
    let (mut published, mut split, mut fragmented) = (0, 0, 0);
    for order in permutations(&[0, 1, 2, 3]) {
        let members: Vec<_> = order.iter().map(|&i| outer[i]).collect();
        let (docx, top) = declared_union(d1.clone(), &members, pairs.clone());
        let ev = run(&docx);
        let Some(e) = failure(&ev, top) else {
            published += 1;
            continue;
        };
        let shown = format!("{e:?}");
        match e {
            NodeErrorKind::DeclareResolve { .. }
                if shown.contains("ConsumedByFold { by: FragmentedMerge }") =>
            {
                fragmented += 1;
            }
            NodeErrorKind::DeclareResolve { .. }
                if shown.contains("ConsumedByFold { by: Split }") =>
            {
                split += 1;
            }
            _ => {
                panic!("{order:?}: the outer union refused other than as a consumed face: {shown}")
            }
        }
    }
    assert!(published > 0, "no order publishes");
    assert!(
        split > 0,
        "no order divides the merged top before it merges"
    );
    assert!(
        fragmented > 0,
        "no order merges the top with `d` and divides it before `c` joins"
    );
}

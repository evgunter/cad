//! **A pair boolean that cuts a face and merges part of it in one step
//! names each piece for what it holds** (N2, N3).
//!
//! `bg` = `b` ∪ `g`: `b` = x 0.5..1.5, and `g` a slab over
//! x 0.3..0.4 standing through the y = 0 plane and both cap planes.
//! `bg` ∪ `a`, `a` = x 0..1, is flush with `b` on all four families. `g`
//! cuts `a`'s y = 0 wall into x 0..0.3 and x 0.4..1, and the second
//! piece merges with `b`'s wall. The first piece holds `a`'s wall alone
//! and is a `Borders` piece of it; the second holds both walls and is a
//! `Borders` piece of their merge. No constituent is published bare.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::fixture::{insert, table};

use editor_core::{
    EntityKind, Node, ProfileDoc, Qualifier, Resolution, RoleSeg, RunCtx, StableName,
};
use geom_core::Tol;

#[test]
fn a_face_cut_and_merged_in_one_pair_step_names_each_piece_for_what_it_holds() {
    let doc = ProfileDoc::empty_derived("emit_pair_cut_and_merged", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block_on(doc, a, (0.3, 0.4), (-1.0, 0.5), -0.5, 2.5);
    let (doc, bg) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![b.into(), g.into()]),
        },
    );
    let (doc, pair) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![bg.into(), a.into()]),
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, pair).is_none(), "{:?}", failure(&ev, pair));
    let t = table(&ev, pair);
    let ctx = RunCtx {
        doc: &doc,
        eval: &ev,
    };
    let is_piece_of = |piece: &StableName, parent: &StableName| {
        piece.kind == EntityKind::Face
            && piece.path.len() == parent.path.len() + 1
            && piece.path.starts_with(&parent.path)
            && matches!(
                piece.path.last(),
                Some(RoleSeg::Fragment(Qualifier::Borders(_)))
            )
    };
    let mut cut_parents = std::collections::BTreeSet::new();
    for (row, _) in t.iter() {
        let Some(RoleSeg::Merged(set)) = row.path.first() else {
            continue;
        };
        let merged = StableName {
            kind: row.kind,
            node: row.node,
            path: vec![RoleSeg::Merged(set.clone())],
        };
        if is_piece_of(row, &merged) {
            cut_parents.insert(merged.clone());
        } else {
            assert_eq!(row, &merged, "a merged row is whole or a Borders piece");
        }
        for c in set {
            assert!(
                t.lookup(c).is_none(),
                "{c:?}, a constituent of {row:?}, is published"
            );
            match crate::fixture::resolve_at_mint(ctx, c) {
                Resolution::Failed(f) => assert!(
                    f.offers.contains(row),
                    "{c:?} does not offer {row:?}: {:?}",
                    f.offers
                ),
                other => panic!("{c:?}, a constituent of {row:?}, resolved: {other:?}"),
            }
        }
    }
    // `a`'s y = 0 wall is cut by `g` and partly merged with `b`'s: the
    // merged rest is a piece of the merge, and the piece `g` cuts off a
    // piece of `a`'s wall.
    assert_eq!(
        cut_parents.len(),
        1,
        "exactly the merge of the y = 0 walls is held as pieces: {cut_parents:?}"
    );
    let parent = cut_parents.first().expect("one cut parent");
    assert_eq!(
        t.iter().filter(|(n, _)| is_piece_of(n, parent)).count(),
        1,
        "the y = 0 merge is held as the merged rest"
    );
    let Some(RoleSeg::Merged(set)) = parent.path.first() else {
        unreachable!("a cut parent is a merge");
    };
    let alone: Vec<&StableName> = t
        .iter()
        .map(|(n, _)| n)
        .filter(|n| set.iter().any(|c| is_piece_of(n, c)))
        .collect();
    assert_eq!(
        alone.len(),
        1,
        "one piece holds a constituent alone, the one `g` cuts off: {alone:?}"
    );
}

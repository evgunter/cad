//! **A pair boolean that cuts a face and merges part of it in one step
//! publishes no face under the name its merge retires** (N3).
//!
//! `bg` = `b` ∪ `g`, undeclared: `b` = x 0.5..1.5, and `g` a slab over
//! x 0.3..0.4 standing through the y = 0 plane and both cap planes.
//! `bg` ∪ `a`, `a` = x 0..1, is declared flush with `b` on all four
//! families. `g` cuts `a`'s y = 0 wall into x 0..0.3 and x 0.4..1, and
//! the second piece merges with `b`'s wall, so the merge lists `a`'s
//! wall as a constituent. The merge is then the parent of both pieces:
//! each is published as a `Borders` piece of it, and neither under
//! `a`'s retired wall name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, failure, flush_pairs, run};
use crate::fixture::{insert, table};

use editor_core::{
    EntityKind, NameRef, Node, ProfileDoc, Qualifier, Resolution, RoleSeg, RunCtx, StableName,
    resolve,
};
use geom_core::Tol;

#[test]
fn a_face_cut_and_merged_in_one_pair_step_publishes_no_constituent() {
    let doc = ProfileDoc::empty_derived("emit_pair_cut_and_merged", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, g) = block(doc, (0.3, 0.4), (-1.0, 0.5), -0.5, 2.5);
    let (doc, bg) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![b.into(), g.into()]),
            declare: Vec::new(),
        },
    );
    // `b`'s faces, sited at the operand that holds them: `bg`'s A side.
    let (bg_read, b_read) = (crate::fixture::out(&doc, bg), crate::fixture::out(&doc, b));
    let pairs = flush_pairs(&doc, (b, b), (a, a))
        .into_iter()
        .map(|(mut l, r)| {
            l.at = bg_read;
            l.name = StableName {
                kind: EntityKind::Face,
                node: bg,
                path: vec![RoleSeg::From {
                    read: b_read,
                    of: NameRef::new(l.name),
                }],
            };
            (l, r)
        })
        .collect();
    let decl = editor_core::declare_continuation(pairs);
    let (doc, pair) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![bg.into(), a.into()]),
            declare: decl,
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
            assert!(
                !t.iter().any(|(n, _)| is_piece_of(n, c)),
                "a piece of {c:?}, a constituent of {row:?}, is published"
            );
            match resolve(ctx, c) {
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
    // merge is its parent, held as two faces, each a piece of it.
    assert_eq!(
        cut_parents.len(),
        1,
        "exactly the merge of the y = 0 walls is held as pieces: {cut_parents:?}"
    );
    let parent = cut_parents.first().expect("one cut parent");
    assert_eq!(
        t.iter().filter(|(n, _)| is_piece_of(n, parent)).count(),
        2,
        "the y = 0 merge is held as the piece `g` cuts off and the merged rest"
    );
}

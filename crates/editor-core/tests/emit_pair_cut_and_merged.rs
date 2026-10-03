//! **A pair boolean that cuts a face and merges part of it in one step
//! publishes no face under the name its merge retires** (N3).
//!
//! `bg` = `b` ∪ `g`, undeclared: `b` = x 0.5..1.5, and `g` a slab over
//! x 0.3..0.4 standing through the y = 0 plane and both cap planes.
//! `bg` ∪ `a`, `a` = x 0..1, is declared flush with `b` on all four
//! families. `g` cuts `a`'s y = 0 wall into x 0..0.3 and x 0.4..1, and
//! the second piece merges with `b`'s wall, so the merge lists `a`'s
//! wall as a constituent while the first piece stays `a`'s alone.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::docm7_union_declare::{block, failure, flush_pairs, run};
use crate::fixture::{insert, table};

use editor_core::{
    BooleanOp, EntityKind, NameRef, Node, ProfileDoc, Qualifier, Resolution, RoleSeg, RunCtx,
    StableName, resolve,
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
        Node::Boolean {
            op: BooleanOp::Union,
            a: b,
            b: g,
            declare: Vec::new(),
        },
    );
    // `b`'s faces, sited at the operand that holds them: `bg`'s A side.
    let pairs = flush_pairs(&doc, (b, b), (a, a))
        .into_iter()
        .map(|(mut l, r)| {
            l.at = bg;
            l.name = StableName {
                kind: EntityKind::Face,
                node: bg,
                path: vec![RoleSeg::FromA(NameRef::new(l.name))],
            };
            (l, r)
        })
        .collect();
    let decl = editor_core::declare_continuation(pairs);
    let (doc, pair) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: bg,
            b: a,
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
    let mut cut_and_merged = 0;
    for (merged, _) in t.iter() {
        let [RoleSeg::Merged(set)] = merged.path.as_slice() else {
            continue;
        };
        for c in set {
            assert!(
                t.lookup(c).is_none(),
                "{c:?}, a constituent of {merged:?}, is published"
            );
            match resolve(ctx, c) {
                Resolution::Failed(f) => assert!(
                    f.offers.contains(merged),
                    "{c:?} does not offer {merged:?}: {:?}",
                    f.offers
                ),
                other => panic!("{c:?}, a constituent of {merged:?}, resolved: {other:?}"),
            }
            if t.iter().any(|(n, _)| is_piece_of(n, c)) {
                cut_and_merged += 1;
            }
        }
    }
    assert_eq!(
        cut_and_merged, 1,
        "exactly `a`'s y = 0 wall is both merged and held as a piece of its own"
    );
}

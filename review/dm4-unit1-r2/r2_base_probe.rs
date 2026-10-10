//! r2's base probe (PR 4527 review): the covered-contact three-member union.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::fixture::{insert, len};
use editor_core::{Node, NodeError, NodeResult, ProfileDoc, RecipeNodeId};
use geom_core::Tol;

fn block(doc: ProfileDoc, (x0, x1): (f64, f64), (y0, y1): (f64, f64), h: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = crate::fixture::on_frame(
        doc, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(doc, Node::Extrude { profile: p.into(), distance: len(h), side: editor_core::ExtrudeSide::Along })
}

#[test]
fn r2_base_covered_three_member_union() {
    let doc = ProfileDoc::empty_derived("r2-covered", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, c) = block(doc, (1.0, 2.0), (0.0, 1.0), 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (-1.0, 2.0), 3.0);
    let ev = crate::corpus::eval::<f64>(&doc);
    let f = |p, q| editor_core::declared_pairs(&editor_core::find_flush_candidates(&ev, p, q, Tol::witness()).unwrap());
    let mut declare = f(a, c);
    declare.extend(f(a, b));
    declare.extend(f(b, c));
    let (doc, three) = insert(doc, Node::Union { members: vec![a.into(), b.into(), c.into()], declare });
    let ev = crate::corpus::eval::<f64>(&doc);
    match ev.nodes.get(&three) {
        Some(NodeResult::Failed(NodeError { kind, .. })) => println!("base three: {kind:?}"),
        Some(r) => println!("base three: builds={}", r.value().is_some()),
        None => println!("base three: none"),
    }
}

//! **A cap on two flush blocks names alike in every member order, up
//! to the position of the circle's vertex** (`names/README.md`, "A
//! conventional vertex").
//!
//! Blocks `a` (x 0..1) and `b` (x 1..2) stand flush on x = 1, both
//! y 0..1 and z 0..0.5, and a ball of radius 0.3 centred (1, 0.5, 0.35)
//! stands up through their merged top in a circle no edge reaches: the
//! ball's seam lies in z = 0.35, below the top. Which step meets the
//! circle, and where its one vertex ends up, is member order: one order
//! merges the tops before the ball cuts them in, another cuts `a`'s top
//! and `b`'s apart and lets the join close the circle across x = 1. The
//! vertex is named for the circle in every order, so every order
//! publishes one table, compared modulo that vertex's position
//! ([`crate::fixture::point_modulo_position`]).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::blocks::{block, block_on, failure, run};
use crate::emit_shared_rim_several::permutations;
use crate::emit_union_rim_piece_ranks::signature;
use editor_core::{EntityKey, EntityKind, Entry, ProfileDoc, RecipeNodeId, RoleSeg, StableName};
use geom_core::Tol;

/// The two blocks and the ball, each an operand placed in `a`'s space,
/// and the union over them in `order`.
fn document(order: &[usize]) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("emit_union_conventional_vertex_orders", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.5);
    let (doc, b) = block_on(doc, a, (1.0, 2.0), (0.0, 1.0), 0.0, 0.5);
    let (doc, ball) = crate::emit_sphere_cut_in_cap::ball(doc, 0.3, 0.5);
    let (doc, ball) = crate::fixture::slid(
        doc,
        a,
        editor_core::Operand::output(ball, 0),
        [1.0, 0.0, 0.35],
    );
    let members = [a, b, ball];
    let ordered: Vec<RecipeNodeId> = order.iter().map(|&i| members[i]).collect();
    crate::fixture::union_over(doc, &ordered)
}

#[test]
fn a_cap_on_two_flush_blocks_names_alike_in_every_member_order() {
    let mut tables: BTreeMap<String, BTreeMap<StableName, String>> = BTreeMap::new();
    let mut held_at = std::collections::BTreeSet::new();
    for order in permutations(&[0, 1, 2]) {
        let at = format!("{order:?}");
        let (doc, union) = document(&order);
        let ev = run(&doc);
        if let Some(e) = failure(&ev, union) {
            panic!("{at}: the union names: {e}");
        }
        let sig = signature(&ev, union);
        let circle: Vec<_> = sig
            .keys()
            .filter(|n| {
                n.kind == EntityKind::Vertex
                    && matches!(n.path.as_slice(), [RoleSeg::ConventionalVertex(_)])
            })
            .collect();
        assert_eq!(
            circle.len(),
            1,
            "{at}: the circle keeps one vertex: {circle:?}"
        );
        let Some(Entry::Unique(v)) = crate::fixture::table(&ev, union).lookup(circle[0]) else {
            panic!("{at}: the circle's vertex is one row");
        };
        let EntityKey::Vertex(v) = v.key else {
            panic!("{at}: the circle's vertex is a vertex");
        };
        let body = crate::corpus::body_of(&ev, union);
        let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        let r = |x: f64| (x * 1e6).round() as i64;
        held_at.insert((r(p.x), r(p.y), r(p.z)));
        tables.insert(at, sig);
    }
    assert!(
        held_at.len() > 1,
        "the orders hold the circle's vertex at different points, so the rows compare \
         modulo its position: {held_at:?}"
    );
    let (first_at, first) = tables.iter().next().expect("six orders");
    for (at, table) in &tables {
        assert_eq!(first, table, "{first_at} against {at}");
    }
}

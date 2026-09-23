//! **A chord between two merged faces that lies on one member is named
//! as that member's edge.**
//!
//! `a` and `b` meet flush along x with all four families declared, so
//! their caps and y-walls merge. A third member folded in before `a`
//! splits `a`'s rim edge where `a` stands alone (x < 0.5); when `a`
//! then joins, a piece of that rim edge sits between the merged top
//! cap and the merged y-wall. Its key is `a`'s own, but its split
//! lineage died before the graft, so it is named from its faces — and
//! both faces are merged, each with a constituent on both sides. The
//! key says which side: the piece reads through to `a`'s top cap and
//! `a`'s y-wall, and is the rim they share.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{edge_of, member_entity, table};

use editor_core::{
    CapEnd, EntityKind, NamingError, NodeErrorKind, ProfileDoc, ProfileEdgeRef, Qualifier,
    RecipeNodeId, RoleSeg, StableName,
};
use geom_core::Tol;

/// Every ordering of `items`.
fn permutations(items: &[RecipeNodeId]) -> Vec<Vec<RecipeNodeId>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for (i, &head) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(i);
        for mut tail in permutations(&rest) {
            tail.insert(0, head);
            out.push(tail);
        }
    }
    out
}

type Bx = ((f64, f64), (f64, f64), f64, f64);

const A: Bx = ((0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
const B: Bx = ((0.5, 1.5), (0.0, 1.0), 0.0, 1.0);

/// A slab through the far y-wall and out the top, over x = 0.3..0.4.
const CORNER: Bx = ((0.3, 0.4), (0.5, 2.0), 0.5, 3.0);

fn slab(x0: f64, x1: f64) -> Bx {
    ((x0, x1), (-1.0, 2.0), 0.5, 3.0)
}

/// `a`, `b` and the rest, in that creation order; `a` and `b` declared
/// flush on all four families.
fn document(rest: &[Bx]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty_derived("emit_seam_edge_merged", Tol::witness());
    let mut ids = Vec::new();
    for &(x, y, z0, dz) in [A, B].iter().chain(rest) {
        let (d, id) = block(doc, x, y, z0, dz);
        doc = d;
        ids.push(id);
    }
    (doc, ids)
}

/// **`[b, c, a]` fuses, and the chord is `a`'s top/far-wall rim edge,
/// ranked along it.** On main it refused `Emission("seam edge between
/// two merged faces (unsupported)")`.
#[test]
fn a_chord_between_two_merged_faces_is_named_as_its_members_rim_edge() {
    let (doc, ids) = document(&[CORNER]);
    let (a, b, c) = (ids[0], ids[1], ids[2]);
    let (docx, union, _) = declared_union(doc, &[b, c, a], flush_pairs((a, a), (b, b)));
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));

    // `a`'s rim between its top cap and its y = 1 wall, in two pieces:
    // x = 0.0..0.3 and x = 0.4..0.5 (the rest of it is inside `b`'s
    // merged faces or `c`).
    let rim = StableName {
        kind: EntityKind::Edge,
        node: a,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            ProfileEdgeRef {
                loop_index: 0,
                segment: 2,
            },
        )],
    };
    let piece = |rank| {
        let mut n = member_entity(union, a, rim.clone(), EntityKind::Edge);
        n.path
            .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of: 2 }));
        n
    };
    let t = table(&ev, union);
    let body = body_of(&ev, union);
    let span = |n: &StableName| {
        let e = edge_of(t, "the rim piece", n);
        let edge = body.get_edge(e).unwrap();
        let x = |he| {
            let v = body.get_half_edge(he).unwrap().start;
            let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            assert!(
                (p.y - 1.0).abs() < 1e-12 && (p.z - 1.0).abs() < 1e-12,
                "{p:?}"
            );
            p.x
        };
        let (x0, x1) = (x(edge.he_plus), x(edge.he_minus));
        (x0.min(x1), x0.max(x1))
    };
    let mut spans = [span(&piece(0)), span(&piece(1))];
    spans.sort_by(|p, q| p.0.total_cmp(&q.0));
    let near =
        |(p, q): (f64, f64), (r, s): (f64, f64)| (p - r).abs() < 1e-12 && (q - s).abs() < 1e-12;
    assert!(
        near(spans[0], (0.0, 0.3)) && near(spans[1], (0.4, 0.5)),
        "the two pieces of a's rim, got {spans:?}"
    );
}

/// **No order of the row's documents refuses the merged-chord
/// `Emission`.** Orders that refuse for reasons of their own —
/// `SharedRim` (a fragmented merged face,
/// `work/emit/shared-rim-several-is-a-missing-rule-legal-declared-unions-reach.md`),
/// a declaration that no longer resolves — are other rows' subjects;
/// none of them is an `Emission` of any kind. The two orders of the
/// corner document that fused only now name every entity identically.
#[test]
fn no_order_of_the_rows_documents_refuses_the_merged_chord() {
    for (label, rest) in [
        ("corner", vec![CORNER]),
        ("two slabs", vec![slab(0.3, 0.4), slab(1.2, 1.3)]),
        (
            "three slabs",
            vec![slab(0.3, 0.4), slab(1.2, 1.3), slab(0.7, 0.8)],
        ),
    ] {
        let (doc, ids) = document(&rest);
        let (a, b) = (ids[0], ids[1]);
        for order in permutations(&ids) {
            let (docx, union, _) = declared_union(doc.clone(), &order, flush_pairs((a, a), (b, b)));
            let ev = run(&docx);
            if let Some(e) = failure(&ev, union) {
                assert!(
                    !matches!(e, NodeErrorKind::Naming(NamingError::Emission { .. })),
                    "{label} {order:?}: {e}"
                );
            }
        }
    }

    let (doc, ids) = document(&[CORNER]);
    let (a, b, c) = (ids[0], ids[1], ids[2]);
    let names = |order: &[RecipeNodeId]| {
        let (docx, union, _) = declared_union(doc.clone(), order, flush_pairs((a, a), (b, b)));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: {:?}",
            failure(&ev, union)
        );
        table(&ev, union)
            .iter()
            .map(|(n, _)| n.clone())
            .collect::<std::collections::BTreeSet<_>>()
    };
    assert_eq!(names(&[b, c, a]), names(&[c, b, a]));
}

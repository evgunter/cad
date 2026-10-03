//! **A chord between two merged faces that lies on one member is named
//! as that member's edge.**
//!
//! `a` and `b` meet flush along x with all four families declared, so
//! their caps and y-walls merge. A third member folded in before `a`
//! splits `a`'s rim edge where `a` stands alone (x < 0.5); when `a`
//! then joins, a piece of that rim edge sits between the merged top
//! cap and the merged y-wall. Its key is `a`'s own, but its split
//! lineage died before the graft, so it is named from its faces — and
//! both faces are merged, each with a constituent on both sides. It is
//! read through to its key's side — `a`'s top cap and `a`'s y-wall —
//! and named as the rim they share, because it lies within that rim.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::emit_shared_rim_several::is_rim_piece;
use crate::fixture::{edge_of, table};

use editor_core::{
    CapEnd, EntityKind, NamingError, NodeErrorKind, ProfileDoc, Qualifier, RecipeNodeId, RoleSeg,
    StableName,
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

/// **The chord is `a`'s top/far-wall rim edge, ranked along it**, in
/// `[b, c, a]` and `[c, b, a]` — each order pinned by name and by the
/// span the name answers to, so two orders wrong the same way cannot
/// agree their way past it — and the two orders publish one name set,
/// which covers every other entity of the body. On main both refused
/// `Emission("seam edge between two merged faces (unsupported)")`.
#[test]
fn a_chord_between_two_merged_faces_is_named_as_its_members_rim_edge() {
    let (doc, ids) = document(&[CORNER]);
    let (a, b, c) = (ids[0], ids[1], ids[2]);
    let mut name_sets = Vec::new();
    for order in [[b, c, a], [c, b, a]] {
        let (docx, union) = declared_union(doc.clone(), &order, flush_pairs(&doc, (a, a), (b, b)));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: {:?}",
            failure(&ev, union)
        );
        assert_rim_pieces(&doc, &ev, union, a, a < b, &order);
        name_sets.push(
            table(&ev, union)
                .iter()
                .map(|(n, _)| n.clone())
                .collect::<std::collections::BTreeSet<_>>(),
        );
    }
    assert_eq!(
        name_sets[0], name_sets[1],
        "[b, c, a] and [c, b, a] name the body differently"
    );
}

/// `a`'s rim between its top cap and its y = 1 wall publishes its
/// pieces at y = z = 1, each named by its ends: the body's vertices cut
/// the rim at 0.5, 0.4 and 0.3, x = 0.3..0.4 is inside `c`, and x =
/// 0.5..1.0, where `a` runs flush with `b`, is named for the lesser
/// member (`emit_union::Flush`) — `a`'s piece when `lesser` says so.
fn assert_rim_pieces(
    doc: &editor_core::ProfileDoc,
    ev: &editor_core::Evaluation<f64>,
    union: RecipeNodeId,
    a: RecipeNodeId,
    lesser: bool,
    order: &[RecipeNodeId],
) {
    let rim = StableName {
        kind: EntityKind::Edge,
        node: a,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            crate::fixture::piece(doc, a, 0, 2),
        )],
    };
    let t = table(ev, union);
    let pieces: Vec<StableName> = t
        .iter()
        .filter(|(n, _)| is_rim_piece(n, a, &rim))
        .map(|(n, _)| n.clone())
        .collect();
    let body = body_of(ev, union);
    let span = |n: &StableName| {
        let e = edge_of(t, "the rim piece", n);
        let edge = body.get_edge(e).unwrap();
        let x = |he| {
            let v = body.get_half_edge(he).unwrap().start;
            let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            assert!(
                (p.y - 1.0).abs() < 1e-12 && (p.z - 1.0).abs() < 1e-12,
                "{order:?}: {p:?}"
            );
            p.x
        };
        let (x0, x1) = (x(edge.he_plus), x(edge.he_minus));
        (x0.min(x1), x0.max(x1))
    };
    let micro = |x: f64| (x * 1e6).round() as i64;
    let mut spans: Vec<(i64, i64)> = pieces
        .iter()
        .map(|n| {
            assert!(
                matches!(n.path.last(), Some(RoleSeg::Fragment(Qualifier::Ends(_)))),
                "{order:?}: a piece of a's rim is named by its ends: {n:?}"
            );
            let (x0, x1) = span(n);
            (micro(x0), micro(x1))
        })
        .collect();
    spans.sort_unstable();
    let mut want = vec![(0.0, 0.3), (0.4, 0.5)];
    if lesser {
        want.push((0.5, 1.0));
    }
    let want: Vec<(i64, i64)> = want
        .into_iter()
        .map(|(p, q)| (micro(p), micro(q)))
        .collect();
    assert_eq!(spans, want, "{order:?}: a's rim pieces");
}

/// **No order of the row's documents refuses with an `Emission`.**
/// Orders that refuse for reasons of their own — `SharedRim` (a
/// fragmented merged face,
/// `work/emit/shared-rim-several-is-a-missing-rule-legal-declared-unions-reach.md`),
/// a declaration that no longer resolves — are other rows' subjects,
/// and what is asserted of them is that none of them reads as a kernel
/// bug.
#[test]
fn no_order_of_the_rows_documents_refuses_with_an_emission() {
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
            let (docx, union) =
                declared_union(doc.clone(), &order, flush_pairs(&doc, (a, a), (b, b)));
            let ev = run(&docx);
            if let Some(e) = failure(&ev, union) {
                assert!(
                    !matches!(e, NodeErrorKind::Naming(NamingError::Emission { .. })),
                    "{label} {order:?}: {e}"
                );
            }
        }
    }
}

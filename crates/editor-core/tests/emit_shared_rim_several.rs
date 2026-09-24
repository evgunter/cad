//! **A chord on one of several collinear rim pieces is named as the
//! piece it lies on.**
//!
//! `a` and `b` meet flush along x with all four families declared, so
//! a union step that folds them together leaves `a`'s top cap and its
//! y = 1 wall (or their merged successors) meeting along one line in
//! PIECES, split where `b`'s end meets it. A later member cuts a chord
//! on that line out of one piece, and the chord's two faces, descended
//! into the accumulated operand, share every piece: "the one edge the
//! pair shares" no longer picks the chord's edge. The chord's geometry
//! does — it lies within exactly one piece — and that piece names it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{edge_of, member_entity, table};

use editor_core::{
    CapEnd, EntityKind, NamingError, NodeErrorKind, ProfileDoc, ProfileEdgeRef, Qualifier,
    RecipeNodeId, RoleSeg, StableName,
};
use geom_core::Tol;

pub(crate) type Bx = ((f64, f64), (f64, f64), (f64, f64));

const A: Bx = ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
const B: Bx = ((0.5, 1.5), (0.0, 1.0), (0.0, 1.0));
/// A slab through both y-walls and out the top, over x = 0.3..0.4.
const G: Bx = ((0.3, 0.4), (-1.0, 2.0), (0.5, 3.0));

pub(crate) fn permutations(items: &[usize]) -> Vec<Vec<usize>> {
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

/// The blocks in creation order `creation`; `ids[i]` is block `i`'s node.
pub(crate) fn document(blocks: &[Bx], creation: &[usize]) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let mut doc = ProfileDoc::empty_derived("emit_shared_rim_several", Tol::witness());
    let mut ids = vec![RecipeNodeId(0); blocks.len()];
    for &i in creation {
        let (x, y, z) = blocks[i];
        let (d, id) = block(doc, x, y, z.0, z.1);
        doc = d;
        ids[i] = id;
    }
    (doc, ids)
}

/// A published piece of member `m`'s edge `edge`: the member-keyed
/// name, ranked `(rank, of)` when the edge is in several pieces.
pub(crate) fn rim_piece(
    union: RecipeNodeId,
    m: RecipeNodeId,
    edge: &StableName,
    rank: Option<(u32, u32)>,
) -> StableName {
    let mut n = member_entity(union, m, edge.clone(), EntityKind::Edge);
    if let Some((rank, of)) = rank {
        n.path
            .push(RoleSeg::Fragment(Qualifier::OrderAlong { rank, of }));
    }
    n
}

/// The x-span of a published edge that runs along y = z = 1.
fn x_span(ev: &editor_core::Evaluation<f64>, union: RecipeNodeId, n: &StableName) -> (f64, f64) {
    let t = table(ev, union);
    let body = body_of(ev, union);
    let edge = body.get_edge(edge_of(t, "the rim piece", n)).unwrap();
    let x = |he| {
        let v = body.get_half_edge(he).unwrap().start;
        let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        assert!(
            (p.y - 1.0).abs() < 1e-12 && (p.z - 1.0).abs() < 1e-12,
            "{n:?} leaves the line: {p:?}"
        );
        p.x
    };
    let (x0, x1) = (x(edge.he_plus), x(edge.he_minus));
    (x0.min(x1), x0.max(x1))
}

/// Every published edge named `FromMember(m, <edge of m>)`, ranked or
/// not, lies within that edge of member `m`'s own body: both of its
/// ends on the segment. A chord named for the wrong rim piece fails
/// here even when the table is otherwise well formed.
fn every_member_edge_lies_on_its_source(
    ev: &editor_core::Evaluation<f64>,
    union: RecipeNodeId,
    at: &str,
) {
    let t = table(ev, union);
    let body = body_of(ev, union);
    let point = |b: &topo::Body<f64>, he| {
        let v = b.get_half_edge(he).unwrap().start;
        *b.get_point(b.get_vertex(v).unwrap().point).unwrap()
    };
    for (name, entry) in t.iter() {
        let (editor_core::Entry::Unique(e), Some(RoleSeg::FromMember { member, of })) =
            (entry, name.path.first())
        else {
            continue;
        };
        let editor_core::EntityKey::Edge(k) = e.key else {
            continue;
        };
        if of.kind != EntityKind::Edge {
            continue;
        }
        let src_body = body_of(ev, *member);
        let src = edge_of(table(ev, *member), "the member's edge", of);
        let se = src_body.get_edge(src).unwrap();
        let (q0, q1) = (point(src_body, se.he_plus), point(src_body, se.he_minus));
        let edge = body.get_edge(k).unwrap();
        for p in [point(body, edge.he_plus), point(body, edge.he_minus)] {
            let d = q1 - q0;
            let off = (p - q0).cross(d).norm() / d.norm();
            let s = (p - q0).dot(d) / d.dot(d);
            assert!(
                off < 1e-9 && s > -1e-9 && s < 1.0 + 1e-9,
                "{at}: {name:?} has an end at {p:?}, off its source edge {q0:?}..{q1:?}"
            );
        }
    }
}

/// **`[a, b, g]`: the chord x = 0.0..0.3 is named as a piece of `a`'s
/// top/y = 1 rim**, ranked with that rim's other pieces. The rim runs
/// from x = 1 to x = 0 (segment 2 of `a`'s profile), and the body's
/// vertices cut it at 0.5, 0.4 and 0.3 into four cells, numbered along
/// that direction (`emit_union::rank_member_edges`). `a` holds cells 0, 1
/// and 3; cell 2 lies inside `g`. On main this order refused
/// `SharedRim { found: Several }`.
#[test]
fn the_chord_is_named_as_the_rim_piece_it_lies_on() {
    let (doc, ids) = document(&[A, B, G], &[0, 1, 2]);
    let (a, b, g) = (ids[0], ids[1], ids[2]);
    let (docx, union, _) = declared_union(doc, &[a, b, g], flush_pairs((a, a), (b, b)));
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
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
    let near =
        |(p, q): (f64, f64), (r, s): (f64, f64)| (p - r).abs() < 1e-12 && (q - s).abs() < 1e-12;
    for (rank, want) in [(0, (0.5, 1.0)), (1, (0.4, 0.5)), (3, (0.0, 0.3))] {
        let got = x_span(&ev, union, &rim_piece(union, a, &rim, Some((rank, 4))));
        assert!(near(got, want), "#{rank} of 4: {got:?}, wanted {want:?}");
    }
}

/// **No order of the review probe's documents refuses
/// `SharedRim { found: Several }`, and none refuses with an
/// `Emission`.** The documents are PR 3112's review corpus: `a` and
/// `b` declared flush on all four families, plus one to three slabs,
/// every member order. On main, 62 of its cells refused
/// `SharedRim(Several)`; every chord there lay within exactly one
/// piece. What the other orders refuse with (a declaration that no
/// longer resolves, an undeclared contact) is other rows' subject.
/// PR 3112's review corpus: `a` and `b` declared flush on all four
/// families, plus one to three slabs; `(label, blocks, creation order)`.
pub(crate) fn probe_corpus() -> Vec<(String, Vec<Bx>, Vec<usize>)> {
    let g = ((0.3, 0.4), (-1.0, 2.0), (0.5, 3.0));
    let glow = ((0.3, 0.4), (-1.0, 2.0), (-0.5, 1.0));
    let h = ((1.0, 1.1), (-1.0, 2.0), (0.5, 3.0));
    let c = ((0.3, 0.4), (0.5, 2.0), (0.5, 3.5));
    let g2 = ((1.2, 1.3), (-1.0, 2.0), (0.5, 3.5));
    let cross = ((-1.0, 2.0), (0.9, 1.1), (0.5, 3.0));
    let mut docs: Vec<(String, Vec<Bx>, Vec<usize>)> = vec![
        ("row".into(), vec![A, B, g, h], vec![0, 1, 2, 3]),
        ("rowids".into(), vec![A, B, g, h], vec![3, 2, 1, 0]),
        ("abg".into(), vec![A, B, g], vec![0, 1, 2]),
        ("abgids".into(), vec![A, B, g], vec![2, 1, 0]),
        ("abglow".into(), vec![A, B, glow], vec![0, 1, 2]),
        ("abc".into(), vec![A, B, c], vec![0, 1, 2]),
        ("abgg2".into(), vec![A, B, g, g2], vec![0, 1, 2, 3]),
        ("cross".into(), vec![A, B, g, cross], vec![0, 1, 2, 3]),
    ];
    let xs = [(0.3, 0.4), (0.6, 0.7), (1.2, 1.3)];
    let ys = [(-1.0, 2.0), (-1.0, 0.5), (0.5, 2.0)];
    let zs = [(0.5, 3.0), (-0.5, 1.0), (-0.5, 2.5)];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            for (k, z) in zs.iter().enumerate() {
                docs.push((
                    format!("fam{i}{j}{k}"),
                    vec![A, B, (*x, *y, *z)],
                    vec![0, 1, 2],
                ));
            }
        }
    }
    docs
}

#[test]
fn no_order_of_the_probe_corpus_refuses_several_shared_rims() {
    let docs = probe_corpus();
    let mut fused = 0;
    for (label, blocks, creation) in docs {
        let (doc, ids) = document(&blocks, &creation);
        for order in permutations(&(0..blocks.len()).collect::<Vec<_>>()) {
            let members: Vec<_> = order.iter().map(|&i| ids[i]).collect();
            let (docx, union, _) = declared_union(
                doc.clone(),
                &members,
                flush_pairs((ids[0], ids[0]), (ids[1], ids[1])),
            );
            let ev = run(&docx);
            match failure(&ev, union) {
                None => {
                    every_member_edge_lies_on_its_source(&ev, union, &format!("{label} {order:?}"));
                    fused += 1;
                }
                Some(e @ NodeErrorKind::Naming(NamingError::SharedRim { .. }))
                | Some(e @ NodeErrorKind::Naming(NamingError::Emission { .. })) => {
                    panic!("{label} {order:?}: {e}")
                }
                Some(_) => {}
            }
        }
    }
    // 100 cells of this corpus fused on main and 60 more fuse now (two
    // more get past the rim and refuse an undeclarable contact at the
    // fourth member); the count pins both halves.
    assert_eq!(fused, 160, "cells fused");
}

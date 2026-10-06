//! **A chord on a flush rim line is named for the member edges it lies
//! along.**
//!
//! `a` and `b` meet flush along x with all four families declared, so
//! `a`'s top cap and its y = 1 wall (or their merged successors) meet
//! along one line that runs over both members' rims. A slab cuts a
//! chord out of that line. In orders that fold `a` and `b` first, the
//! accumulated operand holds the line as one joined edge across both
//! rims; in the others, as the two members' rims. Either way the
//! finished body names a chord within one rim as a piece of it, and one
//! spanning both rims for the set of them (`emit_union::Flush`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{edge_of, table};

use editor_core::{
    CapEnd, EntityKind, NamingError, NodeErrorKind, ProfileDoc, Qualifier, RecipeNodeId, RoleSeg,
    StableName,
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

/// Whether `n` is a published piece of member `m`'s edge `edge`: the
/// member-keyed name, bare or, when the edge is in several pieces,
/// qualified by its ends.
pub(crate) fn is_rim_piece(n: &StableName, m: RecipeNodeId, edge: &StableName) -> bool {
    n.kind == EntityKind::Edge
        && match n.path.as_slice() {
            [RoleSeg::FromMember { member, of }, tail @ ..] => {
                *member == m
                    && **of == *edge
                    && matches!(tail, [] | [RoleSeg::Fragment(Qualifier::Ends(_))])
            }
            _ => false,
        }
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
/// ends on the segment. Every edge named `Merged` of member edges lies
/// along each of them, and each of its ends lies on one of them. A
/// chord named for the wrong rim piece, or for a set it leaves, fails
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
    // The source segment of member edge `of` of `member`.
    let source = |member: RecipeNodeId, of: &StableName| {
        let src_body = body_of(ev, member);
        let src = edge_of(table(ev, member), "the member's edge", of);
        let se = src_body.get_edge(src).unwrap();
        (point(src_body, se.he_plus), point(src_body, se.he_minus))
    };
    // Where `p` lies against segment `q0..q1`: its distance off the
    // line, and its parameter along it.
    let place = |p: geom_core::Point3<f64>, (q0, q1): (geom_core::Point3<f64>, _)| {
        let d: geom_core::Vec3<f64> = q1 - q0;
        (
            (p - q0).cross(d).norm() / d.norm(),
            (p - q0).dot(d) / d.dot(d),
        )
    };
    let on = |(off, s): (f64, f64)| off < 1e-9 && s > -1e-9 && s < 1.0 + 1e-9;
    for (name, entry) in t.iter() {
        let editor_core::Entry::Unique(e) = entry else {
            continue;
        };
        let editor_core::EntityKey::Edge(k) = e.key else {
            continue;
        };
        let edge = body.get_edge(k).unwrap();
        let ends = [point(body, edge.he_plus), point(body, edge.he_minus)];
        match name.path.first() {
            Some(RoleSeg::FromMember { member, of }) if of.kind == EntityKind::Edge => {
                let seg = source(*member, of);
                for p in ends {
                    assert!(
                        on(place(p, seg)),
                        "{at}: {name:?} has an end at {p:?}, off its source edge {seg:?}"
                    );
                }
            }
            Some(RoleSeg::Merged(set)) if name.kind == EntityKind::Edge => {
                let segs: Vec<_> = set
                    .iter()
                    .map(|c| match c.path.as_slice() {
                        [RoleSeg::FromMember { member, of }] => source(*member, of),
                        _ => panic!("{at}: {name:?} lists a constituent that is no member edge"),
                    })
                    .collect();
                for &(q0, q1) in &segs {
                    let ((o0, s0), (o1, s1)) =
                        (place(q0, (ends[0], ends[1])), place(q1, (ends[0], ends[1])));
                    assert!(
                        o0 < 1e-9 && o1 < 1e-9 && s0.max(s1) > 1e-9 && s0.min(s1) < 1.0 - 1e-9,
                        "{at}: {name:?} lists a member edge {q0:?}..{q1:?} it does not run along"
                    );
                }
                for p in ends {
                    assert!(
                        segs.iter().any(|&seg| on(place(p, seg))),
                        "{at}: {name:?} has an end at {p:?} on none of its member edges"
                    );
                }
            }
            _ => {}
        }
    }
}

/// **`[a, b, g]`: the chord x = 0.0..0.3 is named as `a`'s top/y = 1
/// rim, and x = 0.4..1.5 for the set of that rim and `b`'s.** The rim
/// runs from x = 1 to x = 0 (segment 2 of `a`'s profile), and the
/// slab's walls cut the line at 0.3 and 0.4: 0.0..0.3 lies within `a`'s
/// rim alone, 0.3..0.4 lies inside `g`, and 0.4..1.5 runs along `a`'s
/// rim and on along `b`'s, which together cover it. This order once
/// refused `SharedRim { found: Several }`.
#[test]
fn the_chord_is_named_for_the_rims_it_lies_along() {
    let (doc, ids) = document(&[A, B, G], &[0, 1, 2]);
    let (a, b) = (ids[0], ids[1]);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (docx, union) = declared_union(doc, &ids, pairs);
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let rim = |m: RecipeNodeId, seg: usize| StableName {
        kind: EntityKind::Edge,
        node: m,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            crate::fixture::piece(&docx, m, 0, seg),
        )],
    };
    let micro = |x: f64| (x * 1e6).round() as i64;
    let span = |n: &StableName| {
        let (x0, x1) = x_span(&ev, union, n);
        (micro(x0), micro(x1))
    };
    let t = table(&ev, union);
    let pieces: Vec<(i64, i64)> = t
        .iter()
        .filter(|(n, _)| is_rim_piece(n, a, &rim(a, 2)))
        .map(|(n, _)| span(n))
        .collect();
    assert_eq!(pieces, vec![(0, micro(0.3))], "a's rim pieces");
    let mut set = vec![
        crate::fixture::member_entity(union, a, rim(a, 2), EntityKind::Edge),
        crate::fixture::member_entity(union, b, rim(b, 2), EntityKind::Edge),
    ];
    set.sort();
    let joined = StableName {
        kind: EntityKind::Edge,
        node: union,
        path: vec![RoleSeg::Merged(set)],
    };
    assert_eq!(
        span(&joined),
        (micro(0.4), micro(1.5)),
        "the set-named edge"
    );
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
            let (docx, union) = declared_union(
                doc.clone(),
                &members,
                flush_pairs(&doc, (ids[0], ids[0]), (ids[1], ids[1])),
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
    // 148 cells fuse. `row` and `rowids` refuse in all 24 orders here,
    // because their (a, h) contact is undeclared and contact is judged
    // pairwise (DM4); the 6 orders of each that fused when the fold
    // judged contact, with `b` covering it, are the 12 the count lost.
    assert_eq!(fused, 148, "cells fused");
}

/// **A reference to a retired rim piece is offered the joined edge.**
/// Where `a` runs flush with `b`, `a`'s rim is no longer held in
/// pieces: the output stage joins it with `b`'s into one edge, named
/// for the set of the two rims. A piece name spelled before that (here
/// built by hand, as a document saved then would carry it) resolves
/// `Vanished`, and the set-named edge that holds the whole rim, and so
/// every piece of it, is among its offers.
#[test]
fn a_retired_rim_piece_is_offered_its_joined_edge() {
    let (doc, ids) = document(&[A, B], &[0, 1]);
    let (a, b) = (ids[0], ids[1]);
    let pairs = flush_pairs(&doc, (a, a), (b, b));
    let (docx, union) = declared_union(doc, &ids, pairs);
    let ev = run(&docx);
    assert!(failure(&ev, union).is_none(), "{:?}", failure(&ev, union));
    let rim = |m: RecipeNodeId| {
        crate::fixture::member_entity(
            union,
            m,
            StableName {
                kind: EntityKind::Edge,
                node: m,
                path: vec![RoleSeg::RimEdge(
                    CapEnd::End,
                    crate::fixture::piece(&docx, m, 0, 2),
                )],
            },
            EntityKind::Edge,
        )
    };
    let mut set = vec![rim(a), rim(b)];
    set.sort();
    let joined = StableName {
        kind: EntityKind::Edge,
        node: union,
        path: vec![RoleSeg::Merged(set)],
    };
    let t = table(&ev, union);
    edge_of(t, "the joined rim", &joined);
    let corners: Vec<StableName> = t
        .iter()
        .filter(|(n, _)| n.kind == EntityKind::Vertex)
        .map(|(n, _)| n.clone())
        .take(2)
        .collect();
    let mut piece = rim(a);
    piece.path.push(RoleSeg::Fragment(Qualifier::Ends(corners)));
    let ctx = editor_core::RunCtx {
        doc: &docx,
        eval: &ev,
    };
    let editor_core::Resolution::Failed(f) = editor_core::resolve(ctx, &piece) else {
        panic!("the retired piece resolves");
    };
    assert!(
        matches!(f.error, editor_core::ResolveError::Vanished { .. }),
        "{:?}",
        f.error
    );
    assert!(f.offers.contains(&joined), "{:?}", f.offers);
}

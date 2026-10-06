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

use std::collections::BTreeMap;

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
/// along each of them, and each of its ends lies on one of them. No
/// edge named as a seam runs along an edge of any of `members`. A chord
/// named for the wrong rim piece, for a set it leaves, or as a seam
/// where it lies along member edges, fails here even when the table is
/// otherwise well formed.
pub(crate) fn every_member_edge_lies_on_its_source(
    ev: &editor_core::Evaluation<f64>,
    union: RecipeNodeId,
    members: &[RecipeNodeId],
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
            Some(RoleSeg::Seam { .. }) if name.kind == EntityKind::Edge => {
                // A seam runs along no member edge: an edge that does is
                // named for the member edges it lies along, never as a
                // seam, so a joined edge part seam and part member edge
                // would show up here.
                for &m in members {
                    let mb = body_of(ev, m);
                    for (_, me) in mb.edges() {
                        let (q0, q1) = (point(mb, me.he_plus), point(mb, me.he_minus));
                        let (o0, s0) = place(q0, (ends[0], ends[1]));
                        let (o1, s1) = place(q1, (ends[0], ends[1]));
                        let overlap = s0.max(s1).min(1.0) - s0.min(s1).max(0.0);
                        assert!(
                            !(o0 < 1e-9 && o1 < 1e-9 && overlap > 1e-9),
                            "{at}: the seam {name:?} runs along member {m:?}'s edge {q0:?}..{q1:?}"
                        );
                    }
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
                    every_member_edge_lies_on_its_source(
                        &ev,
                        union,
                        &ids,
                        &format!("{label} {order:?}"),
                    );
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
/// for the set of the two rims. A piece name spelled before that, its
/// ends the two corners that cut it (built by hand, as a document saved
/// then would carry it), resolves `Vanished`, and the set-named edge
/// that holds the whole rim, and so every piece of it, is among its
/// offers.
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
    // The piece x = 0..0.5 as it was spelled when `b`'s corner cut the
    // rim: its ends are `a`'s corner at x = 0 and `b`'s at x = 0.5, both
    // vertex 3 of their profiles (x0, y1), on the top cap.
    let corner = |m: RecipeNodeId| {
        crate::fixture::member_entity(
            union,
            m,
            StableName {
                kind: EntityKind::Vertex,
                node: m,
                path: vec![RoleSeg::CapVertex(
                    CapEnd::End,
                    crate::fixture::vpiece(&docx, m, 0, 3),
                )],
            },
            EntityKind::Vertex,
        )
    };
    let mut ends = vec![corner(a), corner(b)];
    ends.sort();
    let mut piece = rim(a);
    piece.path.push(RoleSeg::Fragment(Qualifier::Ends(ends)));
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

/// Every member order of the declared union of `blocks`, each pair in
/// `flush` declared flush on all four families: the orders that fuse,
/// each with its table read as geometry
/// (`emit_union_rim_piece_ranks::signature`), and every one of them held
/// to [`every_member_edge_lies_on_its_source`]. An order that refuses
/// with a naming error fails the row; the others may refuse for reasons
/// other rows own.
fn fused_tables(
    blocks: &[Bx],
    flush: &[(usize, usize)],
) -> Vec<(Vec<usize>, RecipeNodeId, BTreeMap<StableName, String>)> {
    let creation: Vec<usize> = (0..blocks.len()).collect();
    let (doc, ids) = document(blocks, &creation);
    let pairs: Vec<_> = flush
        .iter()
        .flat_map(|&(p, q)| flush_pairs(&doc, (ids[p], ids[p]), (ids[q], ids[q])))
        .collect();
    let mut out = Vec::new();
    for order in permutations(&creation) {
        let members: Vec<_> = order.iter().map(|&i| ids[i]).collect();
        let (docx, union) = declared_union(doc.clone(), &members, pairs.clone());
        let ev = run(&docx);
        match failure(&ev, union) {
            None => {
                every_member_edge_lies_on_its_source(&ev, union, &ids, &format!("{order:?}"));
                out.push((
                    order,
                    union,
                    crate::emit_union_rim_piece_ranks::signature(&ev, union),
                ));
            }
            Some(e @ NodeErrorKind::Naming(_)) => panic!("{order:?}: {e}"),
            Some(_) => {}
        }
    }
    out
}

/// The fused orders publish one table: every name, and the geometry it
/// denotes.
fn one_table(label: &str, fused: &[(Vec<usize>, RecipeNodeId, BTreeMap<StableName, String>)]) {
    let [(first_at, _, first), rest @ ..] = fused else {
        panic!("{label}: no order fuses");
    };
    assert!(!rest.is_empty(), "{label}: only {first_at:?} fuses");
    for (at, _, table) in rest {
        assert_eq!(first, table, "{label}: {first_at:?} against {at:?}");
    }
}

/// The set-named edges of a published table, each as the x-sorted
/// member indices its constituents name.
fn sets(
    table: &BTreeMap<StableName, String>,
    ids: impl Fn(RecipeNodeId) -> usize,
) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = table
        .keys()
        .filter(|n| n.kind == EntityKind::Edge)
        .filter_map(|n| match n.path.as_slice() {
            [RoleSeg::Merged(set)] => Some(
                set.iter()
                    .map(|c| match c.path.as_slice() {
                        [RoleSeg::FromMember { member, .. }] => ids(*member),
                        _ => panic!("{n:?} lists a constituent that is no member edge"),
                    })
                    .collect::<std::collections::BTreeSet<_>>()
                    .into_iter()
                    .collect(),
            ),
            _ => None,
        })
        .collect();
    out.sort();
    out
}

/// **Four boxes flush in a line publish one table in all 24 orders.**
/// Each overlaps its neighbours along x and no other box, every
/// neighbour pair declared flush. The four long rims are each one edge
/// along all four members' rims, named for the set of the four. Red when
/// any name, a joined edge's among them, depends on member order, or
/// when a joined edge is named for less than the rims it runs along.
#[test]
fn four_flush_boxes_in_a_line_publish_one_table_in_every_order() {
    let row = |x0: f64, x1: f64| ((x0, x1), (0.0, 1.0), (0.0, 1.0));
    let blocks = [row(0.0, 1.0), row(0.6, 1.6), row(1.2, 2.2), row(1.8, 2.8)];
    let fused = fused_tables(&blocks, &[(0, 1), (1, 2), (2, 3)]);
    assert_eq!(fused.len(), 24, "every order fuses");
    one_table("four in a line", &fused);
    let (_, ids) = document(&blocks, &[0, 1, 2, 3]);
    let index = |m: RecipeNodeId| ids.iter().position(|&i| i == m).unwrap();
    assert_eq!(sets(&fused[0].2, index), vec![vec![0, 1, 2, 3]; 4]);
}

/// **Partial overlaps: a rim in two sets.** `a` = x 0..2, `b` = 1.5..3,
/// `c` = 2.5..4 and `d` = 3.5..5, each neighbour pair flush. Uncut, each
/// long rim is one edge along all four. With a slab over x 2.6..2.7 the
/// top rims are cut there: x 0..2.6 runs along `a`'s, `b`'s and the
/// first tenth of `c`'s rim, and 2.7..5 along the last stretch of `b`'s,
/// `c`'s and `d`'s, so `b`'s and `c`'s rims are in both sets, as the
/// rims a cell runs along
/// (`names/README.md`). In every order, one table. A reference to a
/// piece of `c`'s rim is offered both sets that list it.
#[test]
fn partial_overlaps_name_each_joined_edge_for_the_rims_it_runs_along() {
    let row = |x0: f64, x1: f64| ((x0, x1), (0.0, 1.0), (0.0, 1.0));
    let (a, b, c, d) = (row(0.0, 2.0), row(1.5, 3.0), row(2.5, 4.0), row(3.5, 5.0));
    let flush = [(0, 1), (1, 2), (2, 3)];
    let fused = fused_tables(&[a, b, c, d], &flush);
    one_table("partial overlaps", &fused);
    let (_, ids) = document(&[a, b, c, d], &[0, 1, 2, 3]);
    let index = |m: RecipeNodeId| ids.iter().position(|&i| i == m).unwrap();
    assert_eq!(sets(&fused[0].2, index), vec![vec![0, 1, 2, 3]; 4]);

    let slab = ((2.6, 2.7), (-1.0, 2.0), (0.5, 3.0));
    let fused = fused_tables(&[a, b, c, d, slab], &flush);
    one_table("partial overlaps under a slab", &fused);
    let (doc, ids) = document(&[a, b, c, d, slab], &[0, 1, 2, 3, 4]);
    let index = |m: RecipeNodeId| ids.iter().position(|&i| i == m).unwrap();
    assert_eq!(
        sets(&fused[0].2, index),
        vec![
            vec![0, 1, 2],
            vec![0, 1, 2],
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3],
            vec![1, 2, 3],
            vec![1, 2, 3],
        ],
        "the bottom rims whole, each top rim in two"
    );

    // The offers for a piece of `c`'s top y = 0 rim (segment 0).
    let pairs: Vec<_> = flush
        .iter()
        .flat_map(|&(p, q)| flush_pairs(&doc, (ids[p], ids[p]), (ids[q], ids[q])))
        .collect();
    let (docx, union) = declared_union(doc, &ids, pairs);
    let ev = run(&docx);
    let rim = crate::fixture::member_entity(
        union,
        ids[2],
        StableName {
            kind: EntityKind::Edge,
            node: ids[2],
            path: vec![RoleSeg::RimEdge(
                CapEnd::End,
                crate::fixture::piece(&docx, ids[2], 0, 0),
            )],
        },
        EntityKind::Edge,
    );
    let listing: Vec<StableName> = table(&ev, union)
        .iter()
        .map(|(n, _)| n.clone())
        .filter(|n| matches!(n.path.as_slice(), [RoleSeg::Merged(set)] if set.contains(&rim)))
        .collect();
    assert_eq!(
        listing.len(),
        2,
        "c's top y = 0 rim is in two sets: {listing:?}"
    );
    let mut piece = rim.clone();
    piece
        .path
        .push(RoleSeg::Fragment(Qualifier::Ends(vec![rim.clone(), rim])));
    let ctx = editor_core::RunCtx {
        doc: &docx,
        eval: &ev,
    };
    let editor_core::Resolution::Failed(f) = editor_core::resolve(ctx, &piece) else {
        panic!("the retired piece resolves");
    };
    for set in &listing {
        assert!(
            f.offers.contains(set),
            "{set:?} is not offered: {:?}",
            f.offers
        );
    }
}

/// **One member inside two others.** `c` = x 1..2.5 lies inside the
/// union of `a` = 0..2 and `b` = 1.5..3 and runs flush with both, each
/// pair declared. Each long rim is one edge along all three members'
/// rims, named for the set of the three, in every order.
#[test]
fn a_member_inside_two_others_is_in_their_rims_set() {
    let row = |x0: f64, x1: f64| ((x0, x1), (0.0, 1.0), (0.0, 1.0));
    let blocks = [row(0.0, 2.0), row(1.5, 3.0), row(1.0, 2.5)];
    let fused = fused_tables(&blocks, &[(0, 1), (0, 2), (1, 2)]);
    one_table("one inside two", &fused);
    let (_, ids) = document(&blocks, &[0, 1, 2]);
    let index = |m: RecipeNodeId| ids.iter().position(|&i| i == m).unwrap();
    assert_eq!(sets(&fused[0].2, index), vec![vec![0, 1, 2]; 4]);
}

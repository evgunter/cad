//! **A member edge's pieces are ranked once, over the finished union,
//! so one name lands on one piece in every member order.**
//!
//! The fold ranks a member edge's pieces at whichever step cuts it, and
//! which step that is — and which member keeps the pieces where two run
//! flush — depends on member order. The published table re-ranks each
//! member edge's pieces over the finished body along the edge's own
//! direction (`emit_union::rank_member_edges`). These rows pin the
//! consequence over PR 3112's review corpus: a name two fused orders
//! both publish denotes the same geometry in both.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeMap;

use crate::corpus::body_of;
use crate::docm7_union_declare::{declared_union, failure, flush_pairs, run};
use crate::emit_shared_rim_several::{document, permutations, probe_corpus, rim_piece};
use crate::fixture::{face_vertices, table};

use editor_core::{CapEnd, EntityKey, EntityKind, Entry, ProfileEdgeRef, RoleSeg, StableName};

/// A rounded point, comparable across two evaluations.
type P = (i64, i64, i64);

fn round(p: &geom_core::Point3<f64>) -> P {
    let r = |x: f64| (x * 1e6).round() as i64;
    (r(p.x), r(p.y), r(p.z))
}

/// What each uniquely named entity of a published table IS, independent
/// of arena keys: a vertex's point, an edge's two end points, a face's
/// vertex points.
fn geometry(
    ev: &editor_core::Evaluation<f64>,
    union: editor_core::RecipeNodeId,
) -> BTreeMap<StableName, Vec<P>> {
    let body = body_of(ev, union);
    let point = |v| round(body.get_point(body.get_vertex(v).unwrap().point).unwrap());
    let mut out = BTreeMap::new();
    for (name, entry) in table(ev, union).iter() {
        let Entry::Unique(e) = entry else { continue };
        let mut sig = match e.key {
            EntityKey::Vertex(v) => vec![point(v)],
            EntityKey::Edge(k) => {
                let edge = body.get_edge(k).unwrap();
                [edge.he_plus, edge.he_minus]
                    .iter()
                    .map(|&he| point(body.get_half_edge(he).unwrap().start))
                    .collect()
            }
            EntityKey::Face(f) => face_vertices(body, f).into_iter().map(point).collect(),
            _ => continue,
        };
        sig.sort_unstable();
        out.insert(name.clone(), sig);
    }
    out
}

/// **No name rebinds across member orders.** Over every document of the
/// corpus and every pair of fused orders, a name both tables publish
/// denotes the same vertex point, the same edge ends and the same face
/// vertices. Before the union re-ranked member edges, 102 of the 350
/// pairs rebound a `FromMember` rim-edge piece (191 names).
#[test]
fn a_name_two_member_orders_both_publish_denotes_the_same_geometry() {
    let mut compared = 0;
    for (label, blocks, creation) in probe_corpus() {
        let (doc, ids) = document(&blocks, &creation);
        let mut seen: BTreeMap<StableName, (Vec<P>, Vec<usize>)> = BTreeMap::new();
        for order in permutations(&(0..blocks.len()).collect::<Vec<_>>()) {
            let members: Vec<_> = order.iter().map(|&i| ids[i]).collect();
            let (docx, union, _) = declared_union(
                doc.clone(),
                &members,
                flush_pairs((ids[0], ids[0]), (ids[1], ids[1])),
            );
            let ev = run(&docx);
            if failure(&ev, union).is_some() {
                continue;
            }
            for (name, sig) in geometry(&ev, union) {
                match seen.get(&name) {
                    Some((first, at)) => {
                        compared += 1;
                        assert_eq!(
                            first, &sig,
                            "{label}: {name:?} is {first:?} in {at:?} and {sig:?} in {order:?}"
                        );
                    }
                    None => {
                        seen.insert(name, (sig, order.clone()));
                    }
                }
            }
        }
    }
    assert!(
        compared > 1000,
        "only {compared} cross-order names compared"
    );
}

/// **`fam010`, the row's own case.** `a`'s bottom-y rim (segment 0,
/// x = 0 → 1 at y = 0, z = 1) is ranked along +x over the pieces `a`
/// keeps: three in `[a, b, g]`, where `a` keeps the stretch it runs
/// flush with `b`, and two in `[b, a, g]`, where `b` does. Before, the
/// name `#1 of 2` was x = 0.5..1.0 in the first order and x = 0.4..0.5
/// in the second.
#[test]
fn fam010_ranks_a_rim_the_same_way_in_both_orders() {
    let g = ((0.3, 0.4), (-1.0, 0.5), (0.5, 3.0));
    let (doc, ids) = document(
        &[
            ((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
            ((0.5, 1.5), (0.0, 1.0), (0.0, 1.0)),
            g,
        ],
        &[0, 1, 2],
    );
    let (a, b, c) = (ids[0], ids[1], ids[2]);
    let rim = StableName {
        kind: EntityKind::Edge,
        node: a,
        path: vec![RoleSeg::RimEdge(
            CapEnd::End,
            ProfileEdgeRef {
                loop_index: 0,
                segment: 0,
            },
        )],
    };
    let span = |order: [editor_core::RecipeNodeId; 3], rank| {
        let (docx, union, _) = declared_union(doc.clone(), &order, flush_pairs((a, a), (b, b)));
        let ev = run(&docx);
        assert!(
            failure(&ev, union).is_none(),
            "{order:?}: {:?}",
            failure(&ev, union)
        );
        let geo = geometry(&ev, union);
        let sig = geo
            .get(&rim_piece(union, a, &rim, Some(rank)))
            .unwrap_or_else(|| panic!("{order:?}: no piece {rank:?}"))
            .clone();
        sig.iter().map(|p| p.0).collect::<Vec<_>>()
    };
    let x = |a: f64, b: f64| vec![(a * 1e6).round() as i64, (b * 1e6).round() as i64];
    assert_eq!(span([a, b, c], (0, 3)), x(0.0, 0.3));
    assert_eq!(span([a, b, c], (1, 3)), x(0.4, 0.5));
    assert_eq!(span([a, b, c], (2, 3)), x(0.5, 1.0));
    assert_eq!(span([b, a, c], (0, 2)), x(0.0, 0.3));
    assert_eq!(span([b, a, c], (1, 2)), x(0.4, 0.5));
}

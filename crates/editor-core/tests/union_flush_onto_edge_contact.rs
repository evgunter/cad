//! **A flush partner folded onto an edge contact, through the public
//! door.** `a` and `b` are flush along x; `c` touches them along one
//! edge only. Every member order of `Node::Union`, and every order of
//! two chained two-member `Node::Union`s, builds at the three blocks'
//! volume — the kernel rows are `topo`'s suite of the same name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, failure, run};
use crate::corpus::body_of;
use crate::fixture::insert;
use editor_core::{Node, ProfileDoc, RecipeNodeId, RoleSeg};
use geom_core::Tol;

/// `c`'s x-range: over `b` alone, inside `a ∩ b`, and across `a`'s
/// far end.
const C_SPANS: [(f64, f64); 3] = [(0.5, 1.5), (0.25, 0.75), (-0.5, 0.5)];

const ORDERS: [[usize; 3]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

const NAMES: [&str; 3] = ["a", "b", "c"];

/// The three blocks, in `a, b, c` order.
fn blocks(span: (f64, f64)) -> (ProfileDoc, [RecipeNodeId; 3]) {
    let doc = ProfileDoc::empty_derived("union_flush_onto_edge_contact", Tol::witness());
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 1.0);
    let (doc, b) = block_on(doc, a, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block_on(doc, a, span, (-1.0, 0.0), -1.0, 1.0);
    (doc, [a, b, c])
}

fn volume(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::test_support::pinned_volume(
        "the union has mass",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&(body_of(ev, id))),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume
}

/// **`Node::Union` builds in every member order.** Red when the
/// kernel classifies a vertex-vertex pair after another pair sharing
/// its vertex moved that vertex's orbit: `b,c,a` (every span) and
/// `a,c,b` (the span over `b`) panic at the block folded last.
#[test]
fn the_union_node_folds_a_flush_partner_onto_the_edge_contact() {
    for span in C_SPANS {
        for order in ORDERS {
            let (doc, ids) = blocks(span);
            let members: Vec<_> = order.iter().map(|&i| ids[i]).collect();
            let (doc, union) = crate::fixture::union_over(doc, &members);
            let ev = run(&doc);
            let label = order.map(|i| NAMES[i]);
            assert!(
                failure(&ev, union).is_none(),
                "c over {span:?}, members {label:?}: {:?}",
                failure(&ev, union)
            );
            let (got, want) = (volume(&ev, union), 1.5 + (span.1 - span.0));
            assert!(
                (got - want).abs() < 1e-9,
                "c over {span:?}, members {label:?}: volume {got}, want {want}"
            );
        }
    }
}

/// **`Node::Union` publishes one table in every member order.** For
/// each span of `c`, every member order names every entity of the union
/// alike, each name denoting the same geometry
/// (`emit_union_rim_piece_ranks::signature`). The bodies are one complex
/// in every order (maximal edges), and the names are read off it: the
/// flush rims `a` and `b` share are each one edge named for the set of
/// their two rims, and a vertex where `c` touches one cites the rim it
/// lies on.
#[test]
fn the_union_node_publishes_one_table_in_every_member_order() {
    for span in C_SPANS {
        let mut tables = Vec::new();
        for order in ORDERS {
            let (doc, ids) = blocks(span);
            let members: Vec<_> = order.iter().map(|&i| ids[i]).collect();
            let (doc, union) = crate::fixture::union_over(doc, &members);
            let ev = run(&doc);
            let label = order.map(|i| NAMES[i]);
            assert!(failure(&ev, union).is_none(), "{label:?}");
            tables.push((
                label,
                crate::emit_union_rim_piece_ranks::signature(&ev, union),
            ));
        }
        let joined = tables[0]
            .1
            .keys()
            .filter(|n| {
                n.kind == editor_core::EntityKind::Edge
                    && matches!(n.path.as_slice(), [RoleSeg::Merged(_)])
            })
            .count();
        assert!(joined >= 3, "c over {span:?}: {joined} set-named edges");
        let (first_at, first) = &tables[0];
        for (at, table) in &tables[1..] {
            assert_eq!(first, table, "c over {span:?}: {first_at:?} against {at:?}");
        }
    }
}

/// **Two chained pair unions build in every order.** The flush pair
/// glues at whichever union joins `a` and `b`. Red as the row above, at
/// the outer union.
#[test]
fn chained_pair_unions_fold_a_flush_partner_onto_the_edge_contact() {
    for span in C_SPANS {
        for [p, q, r] in ORDERS {
            let (doc, ids) = blocks(span);
            let pair = |doc: ProfileDoc, x: RecipeNodeId, y: RecipeNodeId| {
                insert(
                    doc,
                    Node::Union {
                        members: editor_core::Bodies::Spelled(vec![x.into(), y.into()]),
                    },
                )
            };
            let (doc, inner) = pair(doc, ids[p], ids[q]);
            let (doc, outer) = pair(doc, inner, ids[r]);
            let ev = run(&doc);
            let label = [p, q, r].map(|i| NAMES[i]);
            for (node, which) in [(inner, "inner"), (outer, "outer")] {
                assert!(
                    failure(&ev, node).is_none(),
                    "c over {span:?}, ({} ∪ {}) ∪ {}: the {which} union refused: {:?}",
                    label[0],
                    label[1],
                    label[2],
                    failure(&ev, node)
                );
            }
            let (got, want) = (volume(&ev, outer), 1.5 + (span.1 - span.0));
            assert!(
                (got - want).abs() < 1e-9,
                "c over {span:?}, {label:?}: volume {got}, want {want}"
            );
        }
    }
}

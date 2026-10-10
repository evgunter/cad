//! **A flush partner folded onto an edge contact, through the public
//! door.** `a` and `b` are flush along x; `c` touches them along one
//! edge only. Every member order of `Node::Union`, and every order of
//! two chained two-member `Node::Union`s, builds at the three blocks'
//! volume — the kernel rows are `topo`'s suite of the same name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{flush_segs, fname, insert, out};
use editor_core::{Node, ProfileDoc, RecipeNodeId, RoleSeg, SitedRef, VarId};
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
    let (doc, a) = block(doc, (0.0, 1.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 1.0);
    let (doc, c) = block(doc, span, (-1.0, 0.0), -1.0, 1.0);
    (doc, [a, b, c])
}

fn volume(ev: &editor_core::Evaluation<f64>, id: RecipeNodeId) -> f64 {
    topo::mass_properties(body_of(ev, id), Tol::witness())
        .expect("the union has mass")
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
            let pairs = flush_pairs(&doc, (ids[0], ids[0]), (ids[1], ids[1]));
            let (doc, union) = declared_union(doc, &members, pairs);
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
            let pairs = flush_pairs(&doc, (ids[0], ids[0]), (ids[1], ids[1]));
            let (doc, union) = declared_union(doc, &members, pairs);
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
/// is declared at whichever union joins `a` and `b`: between the two
/// blocks when they are its operands, and between the inner union's
/// copy of the one and the other block when they are not. Red as the
/// row above, at the outer union.
#[test]
fn chained_pair_unions_fold_a_flush_partner_onto_the_edge_contact() {
    for span in C_SPANS {
        for [p, q, r] in ORDERS {
            let (doc, ids) = blocks(span);
            // `i`'s flush faces, read directly or (`at`) through the
            // inner union, which names a member's face `From` that
            // member's read; against `j`'s, read directly.
            let flush = |doc: &ProfileDoc, (at, i): (Option<RecipeNodeId>, usize), j: usize| {
                let wrap = |seg: RoleSeg| match at {
                    Some(inner) => fname(
                        inner,
                        RoleSeg::From {
                            read: out(doc, ids[i]),
                            of: fname(ids[i], seg).into(),
                        },
                    ),
                    None => fname(ids[i], seg),
                };
                flush_segs(doc, ids[i])
                    .into_iter()
                    .zip(flush_segs(doc, ids[j]))
                    .map(|(s, t)| {
                        (
                            SitedRef::new(out(doc, at.unwrap_or(ids[i])), wrap(s)),
                            SitedRef::new(out(doc, ids[j]), fname(ids[j], t)),
                        )
                    })
                    .collect::<Vec<_>>()
            };
            let pair =
                |doc: ProfileDoc,
                 x: RecipeNodeId,
                 y: RecipeNodeId,
                 pairs: Option<Vec<(SitedRef<VarId>, SitedRef<VarId>)>>| {
                    insert(
                        doc,
                        Node::Union {
                            members: editor_core::Bodies::Spelled(vec![x.into(), y.into()]),
                            declare: pairs
                                .map(editor_core::declare_continuation)
                                .unwrap_or_default(),
                        },
                    )
                };
            let is_ab = |i: usize, j: usize| i != j && i < 2 && j < 2;
            let inner_pairs = is_ab(p, q).then(|| flush(&doc, (None, p), q));
            let (doc, inner) = pair(doc, ids[p], ids[q], inner_pairs);
            // The inner union's copy of `r`'s flush partner, if it holds
            // one: its faces are named through the read it entered by.
            let partner = [p, q].into_iter().find(|&m| is_ab(m, r));
            let outer_pairs = partner.map(|m| flush(&doc, (Some(inner), m), r));
            let (doc, outer) = pair(doc, inner, ids[r], outer_pairs);
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

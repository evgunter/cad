//! **A flush partner folded onto an edge contact, through the public
//! door.** `a` and `b` are flush along x; `c` touches them along one
//! edge only. Every member order of `Node::Union`, and every order of
//! two chained `Node::Boolean` unions, builds at the three blocks'
//! volume — the kernel rows are `topo`'s suite of the same name.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, declared_union, failure, flush_pairs, run};
use crate::fixture::{flush_segs, fname, insert};
use editor_core::{BooleanOp, Node, ProfileDoc, RecipeNodeId, RoleSeg, SitedRef};
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

/// The role segment an operand's surviving face is named through in a
/// pair boolean's result: `FromA` or `FromB`.
type Side = fn(editor_core::NameRef) -> RoleSeg;

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
/// `a,c,b` (the span over `b`) refuse `Boolean(CorruptOperand {
/// operand: B, .. })` naming the block folded last.
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
            let flush = |doc: &ProfileDoc,
                         (at, i): (Option<(RecipeNodeId, Side)>, usize),
                         (other_at, j): (RecipeNodeId, usize)| {
                let wrap = |seg: RoleSeg| match at {
                    Some((inner, seg_of)) => fname(inner, seg_of(fname(ids[i], seg).into())),
                    None => fname(ids[i], seg),
                };
                flush_segs(doc, ids[i])
                    .into_iter()
                    .zip(flush_segs(doc, ids[j]))
                    .map(|(s, t)| {
                        (
                            SitedRef::new(at.map_or(ids[i], |(inner, _)| inner), wrap(s)),
                            SitedRef::new(other_at, fname(ids[j], t)),
                        )
                    })
                    .collect::<Vec<_>>()
            };
            let pair = |doc: ProfileDoc, x, y, pairs: Option<Vec<(SitedRef, SitedRef)>>| {
                insert(
                    doc,
                    Node::Boolean {
                        op: BooleanOp::Union,
                        a: x,
                        b: y,
                        declare: pairs
                            .map(editor_core::declare_continuation)
                            .unwrap_or_default(),
                    },
                )
            };
            let is_ab = |i: usize, j: usize| i != j && i < 2 && j < 2;
            let inner_pairs = is_ab(p, q).then(|| flush(&doc, (None, p), (ids[q], q)));
            let (doc, inner) = pair(doc, ids[p], ids[q], inner_pairs);
            // The inner union's copy of `r`'s flush partner, if it holds
            // one: its faces are named through the side it entered on.
            let partner = [(p, RoleSeg::FromA as Side), (q, RoleSeg::FromB as Side)]
                .into_iter()
                .find(|&(m, _)| is_ab(m, r));
            let outer_pairs =
                partner.map(|(m, seg_of)| flush(&doc, (Some((inner, seg_of)), m), (ids[r], r)));
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

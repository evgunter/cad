//! **Where two edges cross, the vertex carries each edge's sense against
//! the other operand's closed body** (N2). Two blocks overlap in an L,
//! their caps declared continuations so each cap pair merges: on each
//! merged cap, an edge of `a` crosses an edge of `b` where the walls
//! cross. Past the crossing one edge runs on into the other block's cap
//! — into its closed body — and the other runs out of the first's, so
//! each crossing is named by the two edges and their two senses, read
//! here against the blocks' own rectangles along the edges as each block
//! stores them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus::body_of;
use crate::docm7_union_declare::{block, failure, run};
use crate::fixture::{ends, fname, insert, point, table};
use editor_core::{
    BooleanOp, CapEnd, EntityKey, Entry, Node, ProfileDoc, RecipeNodeId, RoleSeg, Sense, SitedRef,
    StableName,
};
use geom_core::Tol;

/// A closed rectangle in plan, `(x0, x1, y0, y1)`.
type Plan = (f64, f64, f64, f64);

/// The sense at `v` of the edge `edge` names in `node`'s table, read
/// against the closed block whose plan is `other`: whether the edge, as
/// `node` stores it, runs from outside the plan into it at `v` or out of
/// it. A plan's edge is inside it.
fn sense_against(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    edge: &StableName,
    v: [f64; 2],
    other: Plan,
) -> Sense {
    let Some(Entry::Unique(r)) = table(ev, node).lookup(edge) else {
        panic!("{edge:?} names one edge");
    };
    let EntityKey::Edge(e) = r.key else {
        panic!("{edge:?} names an edge");
    };
    let body = body_of(ev, node);
    let [p, q] = ends(body, e).map(|w| point(body, w));
    let (dx, dy) = (q.x - p.x, q.y - p.y);
    let k = 0.01 / (dx * dx + dy * dy).sqrt();
    let inside = |s: f64| {
        let (x, y) = (v[0] + s * k * dx, v[1] + s * k * dy);
        let eps = 1e-12;
        x >= other.0 - eps && x <= other.1 + eps && y >= other.2 - eps && y <= other.3 + eps
    };
    match (inside(-1.0), inside(1.0)) {
        (false, true) => Sense::Enters,
        (true, false) => Sense::Leaves,
        either => panic!("{edge:?} crosses {other:?} at {v:?}: {either:?}"),
    }
}

#[test]
fn two_crossing_edges_carry_each_ones_sense_against_the_other_block() {
    let plan_a: Plan = (0.0, 1.0, 0.0, 1.0);
    let plan_b: Plan = (0.5, 1.5, 0.25, 1.25);
    let doc = ProfileDoc::empty_derived("crossing-sense-edges", Tol::witness());
    let (doc, a) = block(doc, (plan_a.0, plan_a.1), (plan_a.2, plan_a.3), 0.0, 1.0);
    let (doc, b) = block(doc, (plan_b.0, plan_b.1), (plan_b.2, plan_b.3), 0.0, 1.0);
    let decl = editor_core::declare_continuation(
        [CapEnd::End, CapEnd::Start]
            .into_iter()
            .map(|cap| {
                (
                    SitedRef::new(a, fname(a, RoleSeg::Cap(cap))),
                    SitedRef::new(b, fname(b, RoleSeg::Cap(cap))),
                )
            })
            .collect(),
    );
    let (doc, u) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a,
            b,
            declare: decl,
        },
    );
    let ev = run(&doc);
    assert!(failure(&ev, u).is_none(), "{:?}", failure(&ev, u));
    let body = body_of(&ev, u);
    let mut seen = 0;
    for (name, entry) in table(&ev, u).iter() {
        let (
            Entry::Unique(r),
            [
                RoleSeg::EdgeCrossing {
                    a: ea,
                    a_sense,
                    b: eb,
                    b_sense,
                },
            ],
        ) = (entry, name.path.as_slice())
        else {
            continue;
        };
        let EntityKey::Vertex(v) = r.key else {
            panic!("a crossing names a vertex: {name:?}");
        };
        let p = point(body, v);
        let at = [p.x, p.y];
        assert_eq!(
            *a_sense,
            sense_against(&ev, a, ea, at, plan_b),
            "a's edge against b at {p:?}: {name:?}"
        );
        assert_eq!(
            *b_sense,
            sense_against(&ev, b, eb, at, plan_a),
            "b's edge against a at {p:?}: {name:?}"
        );
        seen += 1;
    }
    assert_eq!(
        seen, 4,
        "the walls cross at two corners of the overlap, on each cap"
    );
}

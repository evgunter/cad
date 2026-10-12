//! **Where two edges cross, the vertex carries each edge's sense against
//! the other operand's closed body** (N2). Two blocks overlap in an L,
//! their caps flush so each cap pair merges: on each
//! merged cap, an edge of `a` crosses an edge of `b` where the walls
//! cross. Past the crossing one edge runs on into the other block's cap
//! — into its closed body — and the other runs out of the first's, so
//! each crossing is named by the two edges and their two senses, read
//! here against the blocks' own rectangles along the edges as each block
//! stores them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, failure, run};
use crate::corpus::body_of;
use crate::fixture::{ends, insert, point, table};
use editor_core::{
    EntityKey, Entry, Node, ProfileDoc, RecipeNodeId, RoleSeg, Sense, Speaker, StableName,
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
    let (doc, u) = insert(
        doc,
        Node::Union {
            members: editor_core::Bodies::Spelled(vec![a.into(), b.into()]),
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
        // Each side names a member's edge in the union's space,
        // `From` the member's read: read against the OTHER block.
        let (ar, br) = (crate::fixture::out(&doc, a), crate::fixture::out(&doc, b));
        let member = |edge: &StableName| match edge.path.as_slice() {
            [RoleSeg::From { read, of }] if *read == ar => (a, (**of).clone(), plan_b),
            [RoleSeg::From { read, of }] if *read == br => (b, (**of).clone(), plan_a),
            _ => panic!("a crossing's side is a member's edge: {edge:?}"),
        };
        for (edge, sense) in [(ea, a_sense), (eb, b_sense)] {
            let (node, of, other) = member(edge);
            assert_eq!(
                *sense,
                sense_against(&ev, node, &of, at, other),
                "{edge:?} against the other block at {p:?}: {name:?}"
            );
        }
        seen += 1;
    }
    assert_eq!(
        seen, 4,
        "the walls cross at two corners of the overlap, on each cap"
    );
}

/// The names `node`'s table binds to a vertex at `at`, to 1e-9.
fn names_at(
    ev: &editor_core::Evaluation<f64>,
    node: RecipeNodeId,
    at: [f64; 3],
) -> Vec<StableName> {
    let body = body_of(ev, node);
    table(ev, node)
        .iter()
        .filter(|(_, e)| {
            matches!(e, Entry::Unique(r) if matches!(r.key, EntityKey::Vertex(v) if {
                let p = point(body, v);
                (p.x - at[0]).abs() < 1e-9 && (p.y - at[1]).abs() < 1e-9 && (p.z - at[2]).abs() < 1e-9
            }))
        })
        .map(|(n, _)| n.clone())
        .collect()
}

/// **A rim lying on the other operand's face on both sides of where
/// it meets a wall touches that wall from inside** (N2: a side on the
/// face counts as inside). In the corpus's `crossing_slots`, the second
/// slot's floor is flush with the first cavity's, so a bottom rim of the
/// second slot lies on that floor across the first cavity: where it
/// meets a wall of the cavity it touches the wall from inside, and where
/// it meets the cavity's own bottom rims the two edges cross, the
/// cavity's rim entering or leaving the slot and the slot's rim inside
/// the cavity on both sides.
#[test]
fn a_rim_on_the_other_cavitys_floor_touches_its_wall_from_inside() {
    let d = crate::corpus::documents()
        .into_iter()
        .find(|d| d.name == "crossing_slots")
        .expect("the corpus holds crossing_slots");
    let ev = crate::corpus::eval::<f64>(&d.doc);
    let cut = ev
        .order
        .iter()
        .copied()
        .rfind(|&id| {
            table(&ev, id).iter().any(|(n, _)| {
                n.node == id && matches!(n.path.first(), Some(RoleSeg::Crossing { .. }))
            })
        })
        .expect("a subtract mints crossings");
    let body = body_of(&ev, cut);
    let mut touches = Vec::new();
    let mut crossings = Vec::new();
    for (name, entry) in table(&ev, cut).iter() {
        let Entry::Unique(r) = entry else { continue };
        let EntityKey::Vertex(v) = r.key else {
            continue;
        };
        match name.path.as_slice() {
            [RoleSeg::Crossing { sense, .. }] if !sense.crosses() => {
                touches.push((name.clone(), *sense, point(body, v)));
            }
            [
                RoleSeg::EdgeCrossing {
                    a_sense, b_sense, ..
                },
            ] => {
                crossings.push((*a_sense, *b_sense, point(body, v)));
            }
            _ => {}
        }
    }
    let [(name, sense, p)] = touches.as_slice() else {
        panic!("one touch: {touches:?}");
    };
    assert_eq!(*sense, Sense::Inside, "{name:?}");
    assert!(
        (p.z - 0.5).abs() < 1e-12,
        "the touch is on the floor: {p:?}"
    );
    let said = Speaker::TAG.within(&ev).name(name).to_string();
    assert!(
        said.starts_with("the touch where ") && said.contains(" meets from inside the body of "),
        "{said}"
    );
    assert_eq!(crossings.len(), 2, "{crossings:?}");
    for (a_sense, b_sense, p) in &crossings {
        assert!(
            a_sense.crosses() && *b_sense == Sense::Inside && (p.z - 0.5).abs() < 1e-12,
            "{a_sense:?} {b_sense:?} at {p:?}"
        );
    }
}

/// **A corner resting on the other member's face is that corner** (N2),
/// the name it has where that member lies apart: `b` stands on `a`'s
/// top, overhanging two of its sides, and its bottom corner at
/// `(0.5, 0.25, 1)` rests on `a`'s top face. The union names it as `b`'s
/// corner in either member order, as the union with `a` moved off to
/// `x = 5` does.
#[test]
fn a_corner_resting_on_a_face_is_the_corner_in_either_order() {
    let doc = ProfileDoc::empty_derived("corner-rest", Tol::witness());
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 2.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 2.5), (0.25, 2.25), 1.0, 1.0);
    let (doc, a_off) = block(doc, (5.0, 7.0), (0.0, 2.0), 0.0, 1.0);
    let mut first: Option<StableName> = None;
    for members in [[a, b], [b, a], [a_off, b]] {
        let (doc, u) = crate::fixture::union_over(doc.clone(), &members);
        let ev = run(&doc);
        assert!(
            failure(&ev, u).is_none(),
            "{members:?}: {:?}",
            failure(&ev, u)
        );
        let names = names_at(&ev, u, [0.5, 0.25, 1.0]);
        let [name] = names.as_slice() else {
            panic!("{members:?}: one name at the corner: {names:?}");
        };
        let [RoleSeg::From { of, .. }] = name.path.as_slice() else {
            panic!("{members:?}: the corner is a member's own vertex: {name:?}");
        };
        assert_eq!(of.node, b, "{members:?}: b's corner: {name:?}");
        match &first {
            None => first = Some((**of).clone()),
            Some(want) => assert_eq!(**of, *want, "{members:?}"),
        }
    }
}

/// **Where a member's edge crosses another's, the union names the vertex
/// by the two edges and their senses in either member order**, read
/// from the finished body (`emit_union::Flush::edge_crossing`): `b`
/// stands out of `a`'s top, flush with it on the `y = 0` and `y = 1`
/// walls and the bottom, and its lateral edge at `x = 1.5`, `y = 0`
/// crosses `a`'s top rim there.
#[test]
fn an_edge_crossing_of_two_members_is_named_alike_in_either_order() {
    let doc = ProfileDoc::empty_derived("edge-crossing-orders", Tol::witness());
    let (doc, a) = block(doc, (0.0, 2.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, b) = block(doc, (0.5, 1.5), (0.0, 1.0), 0.0, 2.0);
    let mut first: Option<(&str, Vec<StableName>)> = None;
    for (label, members) in [("a b", [a, b]), ("b a", [b, a])] {
        let (doc, u) = crate::fixture::union_over(doc.clone(), &members);
        let ev = run(&doc);
        assert!(failure(&ev, u).is_none(), "{label}: {:?}", failure(&ev, u));
        for at in [
            [1.5, 0.0, 1.0],
            [1.5, 1.0, 1.0],
            [0.5, 0.0, 1.0],
            [0.5, 1.0, 1.0],
        ] {
            let names = names_at(&ev, u, at);
            assert!(
                matches!(
                    names.as_slice(),
                    [StableName { path, .. }] if matches!(
                        path.as_slice(),
                        [RoleSeg::EdgeCrossing { a_sense, b_sense, .. }]
                            if a_sense.crosses() && b_sense.crosses()
                    )
                ),
                "{label}: the crossing at {at:?}: {names:?}"
            );
        }
        let names: Vec<StableName> = [
            [1.5, 0.0, 1.0],
            [1.5, 1.0, 1.0],
            [0.5, 0.0, 1.0],
            [0.5, 1.0, 1.0],
        ]
        .into_iter()
        .flat_map(|at| names_at(&ev, u, at))
        .collect();
        match &first {
            None => first = Some((label, names)),
            Some((was, want)) => assert_eq!(&names, want, "{label} against {was}"),
        }
    }
}

//! **A turn's mitre is named through the document.** Two adjacent top
//! edges of an extruded box, and its whole top rim, chamfered and
//! filleted by name: the node evaluates — the emitter naming every
//! output entity, a mint or a survivor — and each turn mints one
//! `Mitre { vertex }` edge, one `TurnFoot { vertex }` vertex and one
//! `FootVertex { vertex, support }` on the face its two bands share,
//! each answering to one entity.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::{
    CancelToken, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, Node, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, ValuePayload, evaluate,
};
use fixture::{insert, len, on_frame, table};
use geom_core::{Point3, Tol};
use topo::Body;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn body_at(ev: &Evaluation<f64>, id: RecipeNodeId) -> &Body<f64> {
    match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => b,
        other => panic!("expected a body, got {other:?}"),
    }
}

/// The name of the box's edge between two points.
fn edge_named(ev: &Evaluation<f64>, id: RecipeNodeId, a: [f64; 3], b: [f64; 3]) -> StableName {
    let body = body_at(ev, id);
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let at = |p: Point3<f64>, q: [f64; 3]| (p - Point3::new(q[0], q[1], q[2])).norm() < 1e-12;
    table(ev, id)
        .iter()
        .find_map(|(n, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(e) => {
                    let he = body.get_edge(e).unwrap().he_plus;
                    let s = point(body.get_half_edge(he).unwrap().start);
                    let t = point(body.half_edge_end(he).unwrap());
                    ((at(s, a) && at(t, b)) || (at(s, b) && at(t, a))).then(|| n.clone())
                }
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .unwrap_or_else(|| panic!("an edge named between {a:?} and {b:?}"))
}

#[test]
fn a_turn_is_named_by_its_mitre_and_feet() {
    let doc = ProfileDoc::empty_derived("band_planar_mitre_names", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.5), (0.0, 1.5)]],
    );
    let (doc, the_box) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let ev = run(&doc);
    let rim = [
        edge_named(&ev, the_box, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge_named(&ev, the_box, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]),
        edge_named(&ev, the_box, [2.0, 1.5, 1.0], [0.0, 1.5, 1.0]),
        edge_named(&ev, the_box, [0.0, 1.5, 1.0], [0.0, 0.0, 1.0]),
    ];
    // (request, turns, end arcs, feet): a turn mints one mitre, one turn
    // foot and one foot on the shared face; a cut-off one end arc and
    // two feet.
    let rows = [
        (vec![rim[0].clone(), rim[3].clone()], 1, 2, 1 + 2 * 2),
        (rim.to_vec(), 4, 0, 4),
    ];
    for (edges, turns, arcs, feet) in rows {
        for chamfer in [false, true] {
            let node = if chamfer {
                Node::chamfer(the_box, len(0.1), edges.clone())
            } else {
                Node::fillet(the_box, len(0.1), edges.clone())
            };
            let (doc, f) = insert(doc.clone(), node);
            let ev = run(&doc);
            let t = table(&ev, f);
            let count = |want: fn(&RoleSeg) -> bool, kind: EntityKind| {
                t.iter()
                    .filter(|(n, entry)| {
                        n.kind == kind
                            && n.node == f
                            && n.path.first().is_some_and(want)
                            && matches!(entry, Entry::Unique(_))
                    })
                    .count()
            };
            let what = format!("{} edges, chamfer {chamfer}", edges.len());
            assert_eq!(
                count(|s| matches!(s, RoleSeg::Mitre { .. }), EntityKind::Edge),
                turns,
                "{what}: the mitres"
            );
            assert_eq!(
                count(
                    |s| matches!(s, RoleSeg::TurnFoot { .. }),
                    EntityKind::Vertex
                ),
                turns,
                "{what}: the turn feet"
            );
            assert_eq!(
                count(|s| matches!(s, RoleSeg::EndArc { .. }), EntityKind::Edge),
                arcs,
                "{what}: the end arcs"
            );
            assert_eq!(
                count(
                    |s| matches!(s, RoleSeg::FootVertex { .. }),
                    EntityKind::Vertex
                ),
                feet,
                "{what}: the feet"
            );
        }
    }
}

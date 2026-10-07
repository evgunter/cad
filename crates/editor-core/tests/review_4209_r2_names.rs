#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::{
    CancelToken, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, Node, ProfileDoc,
    RecipeNodeId, RoleSeg, StableName, ValuePayload, evaluate,
};
use fixture::{insert, len, on_frame, table};
use geom_core::{Point3, Tol};
use std::collections::BTreeSet;
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

fn names_of<T: geom_core::Decide + topo::AtRestPolicy>(
    ev: &Evaluation<T>,
    id: RecipeNodeId,
) -> BTreeSet<String> {
    let v = ev
        .value(id)
        .unwrap_or_else(|| panic!("node {id:?}: {:?}", ev.nodes.get(&id)));
    v.name_table
        .iter()
        .filter(|(n, e)| n.node == id && matches!(e, Entry::Unique(_)))
        .map(|(n, _)| format!("{n:?}"))
        .collect()
}

/// Review probe (band-dual-4209-r2): the turn's names are the same set on a
/// re-evaluation and at `Interval`.
#[test]
fn probe_turn_names_stable_across_reevaluation_and_interval() {
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
    for edges in [vec![rim[0].clone(), rim[3].clone()], rim.to_vec()] {
        for chamfer in [false, true] {
            let node = if chamfer {
                Node::chamfer(the_box, len(0.1), edges.clone())
            } else {
                Node::fillet(the_box, len(0.1), edges.clone())
            };
            let (doc, f) = insert(doc.clone(), node);
            let a = names_of(&run(&doc), f);
            let b = names_of(&run(&doc), f);
            assert_eq!(a, b, "re-evaluation moves a name");
            let iv = evaluate::<geom_core::Interval>(
                &doc,
                None,
                &CancelToken::new(),
                &EvalOptions::default(),
                Tol::witness(),
            );
            let c = names_of(&iv, f);
            let only_f64: Vec<_> = a.difference(&c).collect();
            let only_iv: Vec<_> = c.difference(&a).collect();
            eprintln!(
                "{} edges chamfer {chamfer}: {} names; f64-only {only_f64:?}; interval-only {only_iv:?}",
                edges.len(),
                a.len()
            );
            assert!(
                only_f64.is_empty() && only_iv.is_empty(),
                "f64 and Interval name sets differ"
            );
        }
    }
}

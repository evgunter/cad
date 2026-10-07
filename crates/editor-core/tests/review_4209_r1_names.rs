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
use geom_core::{Interval, Point3, Tol};
use topo::Body;

#[allow(dead_code)]
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

fn names<T: geom_core::Decide>(ev: &Evaluation<T>, id: RecipeNodeId) -> Vec<String> {
    let mut v: Vec<String> = ev
        .value(id)
        .expect("the node evaluated")
        .name_table
        .iter()
        .filter(|(n, e)| n.node == id && matches!(e, Entry::Unique(_)))
        .map(|(n, _)| format!("{:?}{:?}", n.kind, n.path))
        .collect();
    v.sort();
    v
}

#[test]
fn mitre_names_are_stable_across_reevaluation_and_scalars() {
    let doc = ProfileDoc::empty_derived("review_4209_names", Tol::witness());
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
    let rim = vec![
        edge_named(&ev, the_box, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]),
        edge_named(&ev, the_box, [2.0, 0.0, 1.0], [2.0, 1.5, 1.0]),
        edge_named(&ev, the_box, [2.0, 1.5, 1.0], [0.0, 1.5, 1.0]),
        edge_named(&ev, the_box, [0.0, 1.5, 1.0], [0.0, 0.0, 1.0]),
    ];
    for edges in [vec![rim[0].clone(), rim[3].clone()], rim.clone()] {
        for chamfer in [false, true] {
            let node = if chamfer {
                Node::chamfer(the_box, len(0.1), edges.clone())
            } else {
                Node::fillet(the_box, len(0.1), edges.clone())
            };
            let (doc, f) = insert(doc.clone(), node);
            let a = names(&run(&doc), f);
            let b = names(&run(&doc), f);
            let iv = evaluate::<Interval>(
                &doc,
                None,
                &CancelToken::new(),
                &EvalOptions::default(),
                Tol::witness(),
            );
            let c = names(&iv, f);
            let mitres = a.iter().filter(|n| n.contains("Mitre")).count();
            eprintln!(
                "PROBE {} edges chamfer {chamfer}: {} names, {mitres} mitre-rooted; interval {}",
                edges.len(),
                a.len(),
                c.len()
            );
            assert_eq!(a, b, "re-evaluation moves no name");
            assert_eq!(a, c, "f64 and Interval mint the same names");
        }
    }
}

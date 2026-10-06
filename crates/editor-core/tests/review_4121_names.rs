//! Review probe (PR 4121): twice-split rims and re-evaluation stability.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::{
    CancelToken, EntityKey, EntityKind, Entry, EvalOptions, Evaluation, Node, ProfileDoc,
    RoleSeg, StableName, evaluate,
};
use fixture::{insert, len, on_frame, table};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness())
}

#[test]
fn review_4121_twice_split_rims_name_distinctly_and_stably() {
    let doc = ProfileDoc::empty_derived("review_4121_names", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.5), (0.0, 1.5)]],
    );
    let (doc, the_box) = insert(
        doc,
        Node::Extrude { profile: p, distance: len(1.0), side: ExtrudeSide::Along },
    );
    let ev = run(&doc);
    // every x-parallel edge of the box (both ends on x = 0 / x = 2)
    let names: Vec<(StableName, EdgeSel)> = Vec::new();
    let _ = names;
    let t0 = table(&ev, the_box);
    let mut x_edges: Vec<StableName> = Vec::new();
    {
        let body = match &ev.value(the_box).unwrap().payload {
            editor_core::ValuePayload::Body(b) => b,
            _ => panic!(),
        };
        let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
        for (n, entry) in t0.iter() {
            if let Entry::Unique(r) = entry {
                if let EntityKey::Edge(e) = r.key {
                    let he = body.get_edge(e).unwrap().he_plus;
                    let s: geom_core::Point3<f64> = point(body.get_half_edge(he).unwrap().start);
                    let t: geom_core::Point3<f64> = point(body.half_edge_end(he).unwrap());
                    if (s.x - t.x).abs() > 1.0 {
                        x_edges.push(n.clone());
                    }
                }
            }
        }
    }
    x_edges.sort_by(|a, b| format!("{a:?}").cmp(&format!("{b:?}")));
    assert_eq!(x_edges.len(), 4);
    for (k, edges) in [(2usize, x_edges[..2].to_vec()), (4, x_edges.clone())] {
        for chamfer in [false, true] {
            let node = if chamfer {
                Node::chamfer(the_box, len(0.1), edges.clone())
            } else {
                Node::fillet(the_box, len(0.1), edges.clone())
            };
            let (doc, f) = insert(doc.clone(), node);
            let snap = |ev: &Evaluation<f64>| -> Vec<String> {
                let mut v: Vec<String> = table(ev, f)
                    .iter()
                    .filter(|(n, _)| n.node == f)
                    .map(|(n, e)| format!("{n:?} => {}", match e {
                        Entry::Unique(r) => format!("U {:?}", r.key),
                        Entry::Tied(x) => format!("TIED {x:?}"),
                    }))
                    .collect();
                v.sort();
                v
            };
            let (ev1, ev2) = (run(&doc), run(&doc));
            let (a, b) = (snap(&ev1), snap(&ev2));
            let tied = a.iter().filter(|s| s.contains("TIED")).count();
            let t = table(&ev1, f);
            let count = |want: fn(&RoleSeg) -> bool, kind: EntityKind| {
                t.iter()
                    .filter(|(n, entry)| n.kind == kind && n.node == f && n.path.first().is_some_and(want) && matches!(entry, Entry::Unique(_)))
                    .count()
            };
            let arcs = count(|s| matches!(s, RoleSeg::EndArc { .. }), EntityKind::Edge);
            let feet = count(|s| matches!(s, RoleSeg::FootVertex { .. }), EntityKind::Vertex);
            println!("PROBE names k={k} chamfer={chamfer}: entries={} tied={tied} arcs={arcs} feet={feet} stable={}", a.len(), a == b);
            for s in a.iter().filter(|s| s.contains("TIED")) {
                println!("PROBE   {s}");
            }
            assert_eq!(a, b, "stable across re-evaluation");
            assert_eq!(tied, 0, "no tied name");
            assert_eq!((arcs, feet), (2 * k, 4 * k));
        }
    }
}

struct EdgeSel;

//! Review probe (band-dual-4121-r2): cut-off names are total, unique,
//! stable across re-evaluation and across f64/Interval, including the
//! twice-split rims of two parallel bands.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;
use editor_core::{
    CancelToken, EntityKey, Entry, EvalOptions, Node, ProfileDoc, RecipeNodeId, StableName,
    ValuePayload, evaluate,
};
use fixture::{insert, len, on_frame};
use geom_core::{Interval, Point3, Tol};

fn names<S: editor_core::EvalScalar>(doc: &ProfileDoc, f: RecipeNodeId) -> (Vec<String>, usize) {
    let ev = evaluate::<S>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let v = ev
        .value(f)
        .unwrap_or_else(|| panic!("no value: {:?}", ev.nodes.get(&f)));
    let mut out = Vec::new();
    let mut tied = 0;
    for (n, e) in v.name_table.iter() {
        if matches!(e, Entry::Tied(_)) {
            tied += 1;
        }
        out.push(format!("{n:?}"));
    }
    out.sort();
    (out, tied)
}

#[test]
fn r2_cut_off_names_are_stable_and_unique() {
    let doc = ProfileDoc::empty_derived("r2", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.0, 1.5), (0.0, 1.5)]],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let ev = evaluate::<f64>(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    );
    let body = match &ev.value(b).unwrap().payload {
        ValuePayload::Body(x) => x.clone(),
        _ => panic!(),
    };
    let pt = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let named = |a: [f64; 3], c: [f64; 3]| -> StableName {
        let at = |p: Point3<f64>, q: [f64; 3]| (p - Point3::new(q[0], q[1], q[2])).norm() < 1e-12;
        ev.value(b)
            .unwrap()
            .name_table
            .iter()
            .find_map(|(n, e)| match e {
                Entry::Unique(r) => match r.key {
                    EntityKey::Edge(k) => {
                        let he = body.get_edge(k).unwrap().he_plus;
                        let (s, t) = (
                            pt(body.get_half_edge(he).unwrap().start),
                            pt(body.half_edge_end(he).unwrap()),
                        );
                        ((at(s, a) && at(t, c)) || (at(s, c) && at(t, a))).then(|| n.clone())
                    }
                    _ => None,
                },
                _ => None,
            })
            .unwrap()
    };
    let tf = named([0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let tb = named([0.0, 1.5, 1.0], [2.0, 1.5, 1.0]);
    let bf = named([0.0, 0.0, 0.0], [2.0, 0.0, 0.0]);
    let bb = named([0.0, 1.5, 0.0], [2.0, 1.5, 0.0]);
    let up = named([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let ac = named([0.0, 0.0, 1.0], [0.0, 1.5, 1.0]);
    for (what, req) in [
        ("two parallel", vec![tf.clone(), tb.clone()]),
        ("four x", vec![tf.clone(), tb.clone(), bf, bb]),
        ("corner", vec![tf, up, ac]),
    ] {
        for chamfer in [false, true] {
            let node = if chamfer {
                Node::chamfer(b, len(0.1), req.clone())
            } else {
                Node::fillet(b, len(0.1), req.clone())
            };
            let (doc, f) = insert(doc.clone(), node);
            let (a1, t1) = names::<f64>(&doc, f);
            let (a2, _) = names::<f64>(&doc, f);
            let (ai, ti) = names::<Interval>(&doc, f);
            let ends = a1.iter().filter(|s| s.contains("EndArc")).count();
            let feet = a1.iter().filter(|s| s.contains("FootVertex")).count();
            let frags = a1.iter().filter(|s| s.contains("Fragment")).count();
            println!(
                "R2N {what} chamfer={chamfer}: names={} tied={t1}/{ti} endarc={ends} foot={feet} fragment-names={frags} f64-stable={} interval-equal={}",
                a1.len(),
                a1 == a2,
                a1 == ai
            );
            assert_eq!(a1, a2, "{what}: re-evaluation");
            assert_eq!(a1, ai, "{what}: f64 vs Interval");
            assert_eq!((t1, ti), (0, 0), "{what}: no tied names");
        }
    }
}

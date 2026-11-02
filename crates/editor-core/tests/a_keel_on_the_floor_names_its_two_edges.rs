//! **A keel resting on the floor names its two edges apart.** The slab
//! `[0, 4]² × [0, 1]` less a V-trench along x whose keel rests on the
//! floor `z = 0`. The material parts the keel into two coincident edges,
//! floor | wall a and wall b | floor, and the subtraction publishes one
//! unique name for each, the two different: closed at both ends (a ring
//! of the floor), open at one end (a slit of its outline, the open end
//! one vertex per wedge), and wall to wall (two floor faces, two solids).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{against, axes_at, desc, insert, len, table};
use editor_core::{
    BooleanValue, CancelToken, EntityKey, Entry, EvalOptions, Node, ProfileDoc, ValuePayload,
    evaluate,
};
use geom_core::Tol;

#[test]
fn a_keel_on_the_floor_names_its_two_edges_apart() {
    for (what, x0, length) in [
        ("closed at both ends", 1.5, 1.0),
        ("open at one end", 1.5, 3.0),
        ("wall to wall", -0.5, 5.0),
    ] {
        let doc = ProfileDoc::empty_derived("a_keel_on_the_floor", Tol::witness());
        let extrude = |doc, pts: Vec<(f64, f64)>, d: f64| {
            let (doc, p) = insert(doc, Node::Profile(desc(vec![pts])));
            insert(
                doc,
                Node::Extrude {
                    profile: p.into(),
                    distance: len(d),
                },
            )
        };
        let (doc, slab) = extrude(
            doc,
            vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
            1.0,
        );
        // The trench drawn on its own xy, its copy placed against the
        // slab with its x along the slab's y and its y along z.
        let (doc, prism) = extrude(doc, vec![(2.0, 0.0), (2.6, 2.0), (1.4, 2.0)], length);
        let (doc, trench) = against(
            doc,
            slab,
            prism,
            &axes_at([x0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
        );
        let (doc, cut) = insert(
            doc,
            Node::Subtract {
                from: slab.into(),
                tool: trench.into(),
            },
        );
        let ev = evaluate::<f64>(
            &doc,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        );
        let value = ev
            .value(cut)
            .unwrap_or_else(|| panic!("{what}: the subtraction: {:?}", ev.nodes.get(&cut)));
        let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
            panic!("{what}: the subtraction's value: {:?}", value.payload);
        };
        // The two edges along the keel `y = 2, z = 0`.
        let on_keel = |p: &geom_core::Point3<f64>| (p.y - 2.0).abs() < 1e-9 && p.z.abs() < 1e-9;
        let keel: Vec<topo::EdgeKey> = body
            .body()
            .edges()
            .map(|(e, _)| e)
            .filter(|&e| {
                let he = body.body().get_edge(e).unwrap().he_plus;
                [
                    body.body().get_half_edge(he).unwrap().start,
                    body.body().half_edge_end(he).unwrap(),
                ]
                .iter()
                .all(|&v| {
                    on_keel(
                        body.body()
                            .get_point(body.body().get_vertex(v).unwrap().point)
                            .unwrap(),
                    )
                })
            })
            .collect();
        let [a, b] = keel[..] else {
            panic!("{what}: the keel's edges {keel:?}, not two");
        };
        let names = table(&ev, cut);
        let named = |e: topo::EdgeKey| {
            let held: Vec<_> = names
                .iter()
                .filter(|(_, entry)| match entry {
                    Entry::Unique(r) => r.key == EntityKey::Edge(e),
                    Entry::Tied(rs) => rs.iter().any(|r| r.key == EntityKey::Edge(e)),
                })
                .collect();
            let [(name, Entry::Unique(_))] = held[..] else {
                panic!("{what}: the keel's edge {e:?} answers to {held:?}, not one unique name");
            };
            name.clone()
        };
        assert_ne!(
            named(a),
            named(b),
            "{what}: the keel's two edges take one name"
        );
    }
}

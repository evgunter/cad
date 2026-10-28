//! **A split whose plane holds a keel wrapped round at both ends names
//! the ring's two edges apart.** The box `[0, 4] × [0, 4] × [−1, 1]` less
//! a V-trench along x over `x ∈ [1.5, 2.5]`, its keel on `z = 0`, split
//! by that plane. The half above carries the keel as a zero-area ring of
//! two coincident edges, floor | wall a and floor | wall b; each is a
//! section chord of the wall it bounds, so the two take two names, each
//! unique, as every chord of the section does. The keel's own name names
//! nothing on either half: the half below loses the keel, and the half
//! above holds it as the two chords.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{against, axes_at, half, insert, len, plane_on};
use editor_core::{
    CancelToken, EntityKey, Entry, EvalOptions, Node, ProfileDoc, RoleSeg, SplitHalf, SplitSide,
    ValuePayload, evaluate,
};
use geom_core::Tol;

#[test]
fn a_keel_ring_names_its_two_edges_apart() {
    let doc = ProfileDoc::empty_derived("a_keel_ring", Tol::witness());
    // Each prism on its own xy: the block 2 thick, the trench a
    // triangle extruded 1, its copy standing across the block's mid
    // plane `z = 1`, which the split cuts along.
    let extrude = |doc, pts: Vec<(f64, f64)>, d: f64| {
        let (doc, p) = insert(doc, Node::Profile(crate::fixture::desc(vec![pts])));
        insert(
            doc,
            Node::Extrude {
                profile: p.into(),
                distance: len(d),
            },
        )
    };
    let (doc, block) = extrude(
        doc,
        vec![(0.0, 0.0), (4.0, 0.0), (4.0, 4.0), (0.0, 4.0)],
        2.0,
    );
    let (doc, trench) = extrude(doc, vec![(2.0, 0.0), (2.6, 2.0), (1.4, 2.0)], 1.0);
    let (doc, trench) = against(
        doc,
        block,
        trench,
        &axes_at([1.5, 0.0, 1.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]),
    );
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: block.into(),
            tool: trench.into(),
        },
    );
    let tool = plane_on(&doc, cut, [0.0, 0.0, 1.0], [0.0, 0.0, 1.0]);
    let (doc, split) = insert(
        doc,
        Node::Split {
            target: cut.into(),
            tool,
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
        .value(split)
        .unwrap_or_else(|| panic!("the split: {:?}", ev.nodes.get(&split)));
    let ValuePayload::Split {
        above: SplitSide::Body(above),
        below: SplitSide::Body(below),
    } = &value.payload
    else {
        panic!("the split's halves: {:?}", value.payload);
    };
    let tol = Tol::witness();
    for (what, b, v) in [("above", above, 16.0 - 0.3), ("below", below, 16.0)] {
        let got = topo::test_support::pinned_volume(
            "the measured body",
            &topo::test_support::finished("the measured body", topo::Body::clone(b.body()), tol),
            tol,
        )
        .volume;
        assert!((got - v).abs() < 1e-9 * v, "{what}: volume {got}, not {v}");
    }
    let ring: Vec<topo::EdgeKey> = above
        .body()
        .faces()
        .flat_map(|(_, f)| f.rings.clone())
        .filter_map(|l| match above.body().get_loop(l).unwrap().boundary {
            topo::LoopBoundary::Cycle { first } => Some(above.body().loop_cycle(first).unwrap()),
            topo::LoopBoundary::Empty { .. } => None,
        })
        .filter(|hes| hes.len() == 2)
        .flatten()
        .map(|he| above.body().get_half_edge(he).unwrap().edge)
        .collect();
    let [a, b] = ring[..] else {
        panic!("the half above's two-edge rings hold {ring:?}");
    };
    assert_ne!(a, b, "the ring's two edges are one");
    let table = half(&ev, split, SplitHalf::Above);
    let named = |e: topo::EdgeKey| {
        let names: Vec<_> = table
            .iter()
            .filter(|(_, entry)| match entry {
                Entry::Unique(r) => r.key == EntityKey::Edge(e),
                Entry::Tied(rs) => rs.iter().any(|r| r.key == EntityKey::Edge(e)),
            })
            .collect();
        let [(name, Entry::Unique(_))] = names[..] else {
            panic!("the ring's edge {e:?} answers to {names:?}, not one unique name");
        };
        let Some(RoleSeg::SectionEdge { .. }) = name.path.first() else {
            panic!("the ring's edge {e:?} is named {name:?}, not as a section chord");
        };
        name.clone()
    };
    let (na, nb) = (named(a), named(b));
    assert_ne!(na, nb, "the ring's two edges take one name");
}

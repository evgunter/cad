//! **The bracket blended all round through the document**: a plate and
//! a boss flush with its side, joined by `Node::Union`, every edge by
//! name, filleted and chamfered. Each of its four mixed corners names
//! its patch `CornerFace { vertex }`, its three arcs `EndArc`, its arc
//! on the shared face `TrimEdge` of the odd edge there, and its two
//! host feet `HostFoot { vertex, edge }`, one per pair edge: every name
//! is unique, a replay names the same entities alike, and an unrelated
//! edit keeps them. Chamfered with the odd edge and one other at each
//! mixed corner, each names its `Mitre { vertex }` and its
//! `TurnFoot { vertex }` once, as alike on replay and across the edit.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{insert, len, slid, table};
use editor_core::{
    Bodies, CancelToken, DocEdit, EntityKey, EntityKind, Entry, EvalOptions, Evaluation,
    LoopProgram, Node, Operand, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg, SlotId,
    StableName, ValuePayload, apply, evaluate,
};
use geom_core::{Point3, Tol};

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

fn prism(doc: ProfileDoc, outline: [(f64, f64); 4], d: f64) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::polygon(outline).unwrap()],
            ids: Vec::new(),
            on: None,
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(d),
        },
    )
}

/// The bore-less bracket: a 6 × 4 × 0.5 plate and a 0.5 × 3 boss flush
/// with its `x = 0` side, standing to `z = 2`. Returns the document,
/// the boss's extrude (whose height an unrelated edit bumps) and the
/// union.
fn bracket() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("band_mixed_corner", Tol::witness());
    let (doc, plate) = prism(doc, [(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)], 0.5);
    let (doc, boss) = prism(doc, [(0.0, 0.5), (0.5, 0.5), (0.5, 3.5), (0.0, 3.5)], 1.5);
    let (doc, placed) = slid(doc, plate, boss, [0.0, 0.0, 0.5]);
    let (doc, union) = insert(
        doc,
        Node::Union {
            members: Bodies::Spelled(vec![
                Operand::output(plate, 0).into(),
                Operand::output(placed, 0).into(),
            ]),
        },
    );
    (doc, boss, union)
}

/// The names of `node`'s table whose head role `keep` selects, sorted.
fn named(ev: &Evaluation<f64>, node: RecipeNodeId, keep: fn(&RoleSeg) -> bool) -> Vec<StableName> {
    let mut v: Vec<StableName> = table(ev, node)
        .iter()
        .filter(|(n, _)| n.node == node && n.path.first().is_some_and(keep))
        .map(|(n, _)| n.clone())
        .collect();
    v.sort();
    v
}

fn every_name(ev: &Evaluation<f64>, node: RecipeNodeId) -> Vec<StableName> {
    named(ev, node, |_| true)
}

#[test]
fn the_brackets_mixed_corners_name_each_entity_once_and_keep_their_names() {
    let (doc, boss, union) = bracket();
    let every = editor_core::all_edges(&run(&doc), union);
    for (verb, node) in [
        ("fillet", Node::fillet(union, len(0.05), every.clone())),
        ("chamfer", Node::chamfer(union, len(0.05), every.clone())),
    ] {
        let (doc, blend) = insert(doc.clone(), node);
        let ev = run(&doc);
        let host_feet = named(&ev, blend, |s| matches!(s, RoleSeg::HostFoot { .. }));
        let mut unique = host_feet.clone();
        unique.dedup();
        assert_eq!(
            (host_feet.len(), unique.len()),
            (8, 8),
            "{verb}: two host feet at each of the four mixed corners, each named once"
        );
        assert!(
            host_feet.iter().all(|n| n.kind == EntityKind::Vertex),
            "{verb}: a host foot is a vertex"
        );
        let all = every_name(&ev, blend);
        let mut distinct = all.clone();
        distinct.dedup();
        assert_eq!(
            all.len(),
            distinct.len(),
            "{verb}: every name of the table is unique"
        );
        assert_eq!(
            all,
            every_name(&run(&doc), blend),
            "{verb}: a replay names every entity alike"
        );
        let bumped = apply(
            &doc,
            &DocEdit::SetParam {
                node: boss,
                slot: SlotId::Distance,
                value: len(1.75).into(),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("the bump applies");
        assert_eq!(
            all,
            every_name(&run(&bumped.doc), blend),
            "{verb}: every name survives the boss's height edit"
        );
    }
}

/// The name of the edge of `id`'s body between two points.
fn edge_named(ev: &Evaluation<f64>, id: RecipeNodeId, a: [f64; 3], b: [f64; 3]) -> StableName {
    let body = match &ev.value(id).expect("the node evaluated").payload {
        ValuePayload::Body(b) => b.body(),
        other => panic!("expected a body, got {other:?}"),
    };
    let point = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    let at = |p: Point3<f64>, q: [f64; 3]| (p - Point3::new(q[0], q[1], q[2])).norm() < 1e-9;
    table(ev, id)
        .iter()
        .find_map(|(n, entry)| match entry {
            Entry::Unique(r) => match r.key {
                EntityKey::Edge(e) => {
                    let (v0, v1) = body.edge_vertices(e).unwrap();
                    let (s, t) = (point(v0), point(v1));
                    ((at(s, a) && at(t, b)) || (at(s, b) && at(t, a))).then(|| n.clone())
                }
                _ => None,
            },
            Entry::Tied(_) => None,
        })
        .unwrap_or_else(|| panic!("an edge named between {a:?} and {b:?}"))
}

/// **The bracket chamfered with the odd edge and one other at each of
/// its four mixed corners** (`band_mixed_corner_odd_plus_one`'s
/// request): each corner is a turn whose overrun cuts the pair strip off
/// in the odd strip's plane, so it names one `Mitre` and one `TurnFoot`,
/// and no mixed corner's host foot; every name is unique, alike on
/// replay, and kept across the boss's height edit.
#[test]
fn the_brackets_odd_plus_one_chamfer_names_each_turn_once_and_keeps_its_names() {
    let (doc, boss, union) = bracket();
    let ev = run(&doc);
    let request: Vec<StableName> = [
        ([0.0, 0.5, 0.5], [0.5, 0.5, 0.5]),
        ([0.0, 3.5, 0.5], [0.5, 3.5, 0.5]),
        ([0.0, 0.5, 0.5], [0.0, 0.5, 2.0]),
        ([0.0, 3.5, 0.5], [0.0, 4.0, 0.5]),
        ([0.5, 0.5, 0.5], [0.5, 0.5, 2.0]),
        ([0.5, 3.5, 0.5], [0.5, 3.5, 2.0]),
    ]
    .into_iter()
    .map(|(a, b)| edge_named(&ev, union, a, b))
    .collect();
    let (doc, blend) = insert(doc, Node::chamfer(union, len(0.05), request));
    let ev = run(&doc);
    let count = |keep: fn(&RoleSeg) -> bool, kind: EntityKind| {
        let v = named(&ev, blend, keep);
        assert!(v.iter().all(|n| n.kind == kind), "{kind:?}: {v:?}");
        v.len()
    };
    assert_eq!(
        (
            count(|s| matches!(s, RoleSeg::Mitre { .. }), EntityKind::Edge),
            count(
                |s| matches!(s, RoleSeg::TurnFoot { .. }),
                EntityKind::Vertex
            ),
            count(
                |s| matches!(s, RoleSeg::HostFoot { .. }),
                EntityKind::Vertex
            ),
        ),
        (4, 4, 0),
        "a mitre and a turn foot at each mixed corner, and no host foot"
    );
    let all = every_name(&ev, blend);
    let mut distinct = all.clone();
    distinct.dedup();
    assert_eq!(
        all.len(),
        distinct.len(),
        "every name of the table is unique"
    );
    assert_eq!(
        all,
        every_name(&run(&doc), blend),
        "a replay names every entity alike"
    );
    let bumped = apply(
        &doc,
        &DocEdit::SetParam {
            node: boss,
            slot: SlotId::Distance,
            value: len(1.75).into(),
            fresh: Vec::new(),
        },
        Tol::witness(),
    )
    .expect("the bump applies");
    assert_eq!(
        all,
        every_name(&run(&bumped.doc), blend),
        "every name survives the boss's height edit"
    );
}

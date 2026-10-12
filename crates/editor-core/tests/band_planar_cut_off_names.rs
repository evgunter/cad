//! **A plane–plane band's cut-off is named through the document.** One
//! edge of an extruded box, and the three edges of one of its corners,
//! chamfered and filleted by name: the node evaluates — which is the
//! emitter naming every output entity, a mint or a survivor — and the
//! cut-off's end curves are `EndArc { vertex, edge }`, its feet
//! `FootVertex { vertex, support }`, each answering to one entity.

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
fn a_cut_off_is_named_by_its_end_arcs_and_feet() {
    let doc = ProfileDoc::empty_derived("band_planar_cut_off_names", Tol::witness());
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
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let ev = run(&doc);
    let front = edge_named(&ev, the_box, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let up = edge_named(&ev, the_box, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
    let across = edge_named(&ev, the_box, [0.0, 0.0, 1.0], [0.0, 1.5, 1.0]);
    // (request, end arcs, feet): a cut-off mints one end arc and two
    // feet per end; a corner patch three arcs and three feet.
    let rows = [
        (vec![front.clone()], 2, 4),
        (vec![front, up, across], 3 + 3, 3 * 2 + 3),
    ];
    for (edges, arcs, feet) in rows {
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

/// **An oblique cut-off is named the same way**: a parallelogram
/// extruded through the document, its top front edge ending at two
/// slanted side walls. Filleted by name, each end curve is an arc of
/// the wall's elliptic section — `EndArc { vertex, edge }`, unique,
/// resolving to an edge whose certified carrier is that ellipse, of
/// minor semi-axis `r` and major `r / cos θ` for the walls' 26.6° lean
/// — and its four feet are `FootVertex`es; chamfered, the same names
/// resolve to chords.
#[test]
fn an_oblique_cut_off_is_named_by_its_elliptic_end_arcs_and_feet() {
    let doc = ProfileDoc::empty_derived("band_planar_oblique_names", Tol::witness());
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (2.0, 0.0), (2.5, 1.0), (0.5, 1.0)]],
    );
    let (doc, prism) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let ev = run(&doc);
    let front = edge_named(&ev, prism, [0.0, 0.0, 1.0], [2.0, 0.0, 1.0]);
    let cos = 1.0 / 1.25_f64.sqrt();
    for chamfer in [false, true] {
        let node = if chamfer {
            Node::chamfer(prism, len(0.1), vec![front.clone()])
        } else {
            Node::fillet(prism, len(0.1), vec![front.clone()])
        };
        let (doc, f) = insert(doc.clone(), node);
        let ev = run(&doc);
        let body = body_at(&ev, f);
        let t = table(&ev, f);
        let mine = |n: &StableName| n.node == f;
        let mut arcs = 0;
        let mut feet = 0;
        for (n, entry) in t.iter().filter(|(n, _)| mine(n)) {
            match (n.path.first(), entry) {
                (Some(RoleSeg::EndArc { .. }), Entry::Unique(r)) => {
                    arcs += 1;
                    let EntityKey::Edge(e) = r.key else {
                        panic!("an end arc names an edge, got {:?}", r.key);
                    };
                    let curve = body
                        .get_curve_geom(body.get_edge(e).unwrap().curve)
                        .and_then(|g| g.certified())
                        .expect("a certified end curve");
                    match (chamfer, curve.carrier()) {
                        (true, geom::Curve3::Line { .. }) => {}
                        (false, geom::Curve3::Ellipse { major, minor, .. }) => {
                            assert!((minor - 0.1).abs() < 1e-15, "minor = r, got {minor}");
                            assert!(
                                (major - 0.1 / cos).abs() < 1e-12,
                                "major = r / cos θ, got {major}"
                            );
                        }
                        (_, other) => panic!("chamfer {chamfer}: an end curve, got {other:?}"),
                    }
                }
                (Some(RoleSeg::FootVertex { .. }), Entry::Unique(_)) => feet += 1,
                (Some(RoleSeg::EndArc { .. } | RoleSeg::FootVertex { .. }), Entry::Tied(_)) => {
                    panic!("chamfer {chamfer}: a cut-off name is tied: {n:?}")
                }
                _ => {}
            }
        }
        assert_eq!(
            (arcs, feet),
            (2, 4),
            "chamfer {chamfer}: two end arcs, four feet"
        );
    }
}

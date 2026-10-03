//! **The `(vertex, edge)` contact record at the census, both
//! directions.** Two bodies placed in one arena with nothing refined:
//! a vertex of one rests on an edge's interior of the other, which no
//! v-v or v-on-f record can name. The record backs the event itself,
//! and the bound of an edge/edge or edge-on-face overlap the event
//! ends; a record with no such event is stale.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::Tol;
use topo::{
    Body, CensusContact, ContactRecords, EdgeKey, StaleDeclaration, ValidationError, VeContact,
    VertexKey, VfContact, validate_pseudomanifold,
};

/// `a` = the unit cube and `c` = x∈(0.5,1.5), y∈(−1,0), z∈(−1,0), in
/// one arena: `c`'s top-front edge lies along `a`'s bottom-front edge
/// over x∈(0.5,1). Each edge's interior holds the other's end there:
/// `c`'s corner (0.5,0,0) and `a`'s corner (1,0,0).
fn edge_touch() -> Body<f64> {
    let tol = Tol::witness();
    let mut body = common::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let c = common::brick((0.5, 1.5), (-1.0, 0.0), (-1.0, 0.0), tol);
    topo::graft_disjoint_all_keyed(&mut body, &c).unwrap();
    body
}

fn point(body: &Body<f64>, v: VertexKey) -> [f64; 3] {
    let p = body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    [p.x, p.y, p.z]
}

fn vertex_at(body: &Body<f64>, at: [f64; 3]) -> VertexKey {
    body.vertices()
        .map(|(k, _)| k)
        .find(|&k| point(body, k) == at)
        .unwrap_or_else(|| panic!("a vertex at {at:?}"))
}

fn edge_between(body: &Body<f64>, p: [f64; 3], q: [f64; 3]) -> EdgeKey {
    body.edges()
        .find(|(_, e)| {
            let ends =
                [e.he_plus, e.he_minus].map(|h| point(body, body.get_half_edge(h).unwrap().start));
            ends == [p, q] || ends == [q, p]
        })
        .unwrap_or_else(|| panic!("an edge {p:?}–{q:?}"))
        .0
}

fn errors(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    validate_pseudomanifold(body, records, Tol::witness())
        .err()
        .unwrap_or_default()
}

fn ve(vertex: VertexKey, edge: EdgeKey) -> VeContact {
    VeContact { vertex, edge }
}

/// The two records `edge_touch` needs, in the order its bounds come.
fn edge_touch_records(body: &Body<f64>) -> [VeContact; 2] {
    let a_edge = edge_between(body, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]);
    let c_edge = edge_between(body, [0.5, 0.0, 0.0], [1.5, 0.0, 0.0]);
    [
        ve(vertex_at(body, [0.5, 0.0, 0.0]), a_edge),
        ve(vertex_at(body, [1.0, 0.0, 0.0]), c_edge),
    ]
}

/// **Undeclared direction.** With no records, both resting vertices
/// and the overlap between them refuse; with both records the body
/// certifies; with one, its own event and the overlap it bounds refuse
/// and the other event does not. Red when pass 2, or the overlap's
/// one-vertex bound, reads only the face rung and never the record.
#[test]
fn a_vertex_on_edge_record_backs_its_event_and_the_overlap_it_bounds() {
    let body = edge_touch();
    let [at_a, at_c] = edge_touch_records(&body);
    let on_edge = |es: &[ValidationError]| -> Vec<(VertexKey, EdgeKey)> {
        es.iter()
            .filter_map(|e| match e {
                ValidationError::UndeclaredContact {
                    contact: CensusContact::VertexOnEdge { vertex, edge },
                    ..
                } => Some((*vertex, *edge)),
                _ => None,
            })
            .collect()
    };
    let overlaps = |es: &[ValidationError]| {
        es.iter()
            .filter(|e| {
                matches!(
                    e,
                    ValidationError::UndeclaredContact {
                        contact: CensusContact::EdgeEdgeOverlap { .. },
                        ..
                    }
                )
            })
            .count()
    };

    let none = errors(&body, &ContactRecords::default());
    let mut found = on_edge(&none);
    found.sort();
    let mut want = vec![(at_a.vertex, at_a.edge), (at_c.vertex, at_c.edge)];
    want.sort();
    assert_eq!(
        found, want,
        "no records: both resting vertices refuse: {none:?}"
    );
    assert_eq!(
        overlaps(&none),
        1,
        "no records: the overlap refuses: {none:?}"
    );

    let both = ContactRecords {
        ve: vec![at_a, at_c],
        ..ContactRecords::default()
    };
    assert_eq!(
        errors(&body, &both),
        vec![],
        "both records: the body certifies"
    );

    for (kept, dropped) in [(at_a, at_c), (at_c, at_a)] {
        let one = ContactRecords {
            ve: vec![kept],
            ..ContactRecords::default()
        };
        let es = errors(&body, &one);
        assert_eq!(
            on_edge(&es),
            vec![(dropped.vertex, dropped.edge)],
            "one record: only the other event refuses: {es:?}"
        );
        assert_eq!(
            overlaps(&es),
            1,
            "one record: the overlap it bounds refuses: {es:?}"
        );
    }
}

/// **Stale direction.** A record whose vertex does not rest on the
/// edge, whose vertex ends the edge, or whose edge is dead is stale,
/// typed as the vertex-on-edge kind. Red when the confirm pass leaves
/// the kind out, or witnesses a record from the line alone (the far
/// vertex is on `a`'s bottom-front line, outside its span).
#[test]
fn a_vertex_on_edge_record_without_its_event_is_stale() {
    let body = edge_touch();
    let records = edge_touch_records(&body);
    let a_edge = records[0].edge;
    for (label, body, bad) in [
        (
            "on the line, past the span",
            &body,
            ve(vertex_at(&body, [1.5, 0.0, 0.0]), a_edge),
        ),
        (
            "off the line",
            &body,
            ve(vertex_at(&body, [1.5, -1.0, -1.0]), a_edge),
        ),
        (
            "the edge's own end",
            &body,
            ve(vertex_at(&body, [0.0, 0.0, 0.0]), a_edge),
        ),
    ] {
        let set = ContactRecords {
            ve: [records.as_slice(), &[bad]].concat(),
            ..ContactRecords::default()
        };
        let es = errors(body, &set);
        assert_eq!(
            es,
            vec![ValidationError::StaleContactDeclaration {
                declaration: StaleDeclaration::VertexOnEdge {
                    vertex: bad.vertex,
                    edge: bad.edge,
                },
            }],
            "{label}"
        );
    }
}

/// **The edge-on-face bound a record holds.** A wedge's apex edge
/// lies in the cube's x = 1 face, from (1, 0.5, 0.5), which rests in
/// the face, to (1, 0.5, 1), which rests on the face's top edge. The
/// overlap's upper bound is the wedge's vertex there, backed by its
/// `(vertex, edge)` record onto an edge that bounds the face. Red when
/// the edge-on-face bound reads v-on-f and v-v records only: the
/// overlap refuses `UndeclaredContact` with both records present.
#[test]
fn an_edge_on_face_overlap_ending_on_the_faces_edge_is_bounded_by_the_record() {
    let tol = Tol::witness();
    let mut body = common::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let wedge = common::prism_z::<f64>(&[(1.0, 0.5), (2.0, 0.2), (2.0, 0.8)], 0.5, 1.0, tol);
    topo::graft_disjoint_all_keyed(&mut body, &wedge.body).unwrap();
    let low = vertex_at(&body, [1.0, 0.5, 0.5]);
    let high = vertex_at(&body, [1.0, 0.5, 1.0]);
    let side = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| {
            let outer = body.get_face(f).unwrap().outer;
            let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
                return false;
            };
            body.loop_cycle(first)
                .unwrap()
                .iter()
                .all(|&h| point(&body, body.get_half_edge(h).unwrap().start)[0] == 1.0)
        })
        .expect("the cube's x = 1 face");
    let rim = edge_between(&body, [1.0, 0.0, 1.0], [1.0, 1.0, 1.0]);
    let records = ContactRecords {
        a_on_b: vec![VfContact {
            vertex: low,
            face: side,
        }],
        ve: vec![ve(high, rim)],
        ..ContactRecords::default()
    };
    assert_eq!(
        errors(&body, &records),
        vec![],
        "both records: the seat certifies"
    );
    let without = ContactRecords {
        ve: vec![],
        ..records.clone()
    };
    let es = errors(&body, &without);
    assert!(
        es.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: CensusContact::VertexOnEdge { vertex, edge },
                ..
            } if (*vertex, *edge) == (high, rim)
        )),
        "without the record its event refuses: {es:?}"
    );
}

/// `a ∪ c` of [`edge_touch`]'s bricks, joined: the union splits each
/// rim where the other's corner rests, and the join makes each rim one
/// edge again, so its two v-v records become `(vertex, edge)` records.
fn joined_edge_touch(tol: Tol) -> topo::BooleanBody<f64> {
    let a = common::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    let c = common::brick((0.5, 1.5), (-1.0, 0.0), (-1.0, 0.0), tol);
    let topo::BooleanResult::Body(mut y) =
        topo::union_with(&a, &c, &topo::BooleanDeclarations::none(), tol).expect("a ∪ c")
    else {
        panic!("a ∪ c came back empty");
    };
    y.join_edges(tol).expect("the join");
    y
}

/// The two end points of `edge`.
fn ends(body: &Body<f64>, edge: EdgeKey) -> [[f64; 3]; 2] {
    let e = body.get_edge(edge).expect("a live edge");
    let mut ends =
        [e.he_plus, e.he_minus].map(|h| point(body, body.get_half_edge(h).unwrap().start));
    ends.sort_by(|p, q| p.partial_cmp(q).unwrap());
    ends
}

/// **Edge-split lineage**: a carried `(vertex, edge)` record lands on
/// the piece of its split edge the vertex rests on. `c`'s corner
/// (0.5, 0, 0) rests on `a`'s joined rim x∈(0, 1), and `a`'s corner
/// (1, 0, 0) on `c`'s, x∈(0.5, 1.5). A slab across one rim, clear of
/// the other brick, splits it at two points on one side of the corner,
/// and the record follows the corner onto the piece between the
/// nearer cut and the rim's far end: (0.3, 0, 0)–(1, 0, 0), and
/// (0.5, 0, 0)–(1.2, 0, 0). The result certifies 3′ with the carried
/// records. Red when the record stays on the edge's original key, or
/// moves to the wrong side of a split: the piece it names does not
/// hold the corner (`StaleContactDeclaration`), and the corner's event
/// is undeclared.
#[test]
fn a_vertex_on_edge_record_follows_its_vertex_onto_a_piece_of_a_split_edge() {
    let tol = Tol::witness();
    let y = joined_edge_touch(tol);
    let carried = topo::CarriedContacts {
        vv: y
            .contacts
            .vv
            .iter()
            .map(|&pair| topo::CarriedVv {
                pair,
                class: topo::ContactClass::Rest,
            })
            .collect(),
        ve: y
            .contacts
            .ve
            .iter()
            .map(|&rest| topo::CarriedVe {
                rest,
                class: topo::ContactClass::Rest,
            })
            .collect(),
        ..topo::CarriedContacts::default()
    };
    for (at, rim, slab, piece) in [
        (
            [0.5, 0.0, 0.0],
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            (0.2, 0.3),
            [[0.3, 0.0, 0.0], [1.0, 0.0, 0.0]],
        ),
        (
            [1.0, 0.0, 0.0],
            [[0.5, 0.0, 0.0], [1.5, 0.0, 0.0]],
            (1.2, 1.3),
            [[0.5, 0.0, 0.0], [1.2, 0.0, 0.0]],
        ),
    ] {
        let corner = vertex_at(&y.body, at);
        let joined = y
            .contacts
            .ve
            .iter()
            .find(|r| r.vertex == corner)
            .expect("the join leaves the corner's (vertex, edge) record");
        assert_eq!(ends(&y.body, joined.edge), rim, "the joined rim");
        let s = common::brick(slab, (-0.5, 0.5), (-0.5, 0.5), tol);
        let decls = topo::BooleanDeclarations {
            carried_a: carried.clone(),
            ..topo::BooleanDeclarations::none()
        };
        let topo::BooleanResult::Body(out) = topo::union_with(&y.body, &s, &decls, tol)
            .unwrap_or_else(|e| panic!("slab {slab:?}: {e:?}"))
        else {
            panic!("slab {slab:?}: empty");
        };
        let onto: Vec<_> = out
            .contacts
            .ve
            .iter()
            .filter(|r| r.vertex == corner)
            .map(|r| ends(&out.body, r.edge))
            .collect();
        assert_eq!(
            onto,
            vec![piece],
            "slab {slab:?}: the corner's record lands on its piece"
        );
        assert_eq!(
            errors(&out.body, &out.contacts),
            vec![],
            "slab {slab:?}: the result certifies"
        );
    }
}

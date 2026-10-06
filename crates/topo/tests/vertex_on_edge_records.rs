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

/// Two points that agree to within rounding: the fixtures' corners are
/// written as decimals, and a rotated one lands an ulp off.
fn same(p: [f64; 3], q: [f64; 3]) -> bool {
    p.iter().zip(q).all(|(x, y)| (x - y).abs() < 1e-12)
}

fn vertex_at(body: &Body<f64>, at: [f64; 3]) -> VertexKey {
    body.vertices()
        .map(|(k, _)| k)
        .find(|&k| same(point(body, k), at))
        .unwrap_or_else(|| panic!("a vertex at {at:?}"))
}

fn edge_between(body: &Body<f64>, p: [f64; 3], q: [f64; 3]) -> EdgeKey {
    body.edges()
        .find(|(_, e)| {
            let [s, t] =
                [e.he_plus, e.he_minus].map(|h| point(body, body.get_half_edge(h).unwrap().start));
            (same(s, p) && same(t, q)) || (same(s, q) && same(t, p))
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

/// **Unrecorded direction.** With no records, both resting vertices
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
/// the kind out, witnesses a record from the line alone (the first far
/// vertex is on `a`'s bottom-front line, outside its span), or from the
/// span alone (the third projects inside the span, off the line).
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
            "off the line, inside the span",
            &body,
            ve(vertex_at(&body, [0.5, -1.0, -1.0]), a_edge),
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
    let a = common::finished(
        "a",
        common::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let c = common::finished(
        "c",
        common::brick((0.5, 1.5), (-1.0, 0.0), (-1.0, 0.0), tol),
        tol,
    );
    let topo::BooleanResult::Body(y) =
        topo::union_with(&a, &c, &topo::BooleanDeclarations::none(), tol).expect("a ∪ c")
    else {
        panic!("a ∪ c came back empty");
    };
    y.join_edges(tol).expect("the join").0
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
/// records. Red when the record stays on the edge's original key: the
/// piece it names does not hold the corner (`StaleContactDeclaration`),
/// and the corner's event is unrecorded. Both slabs read "past the
/// split"; the next row reads both sides and the split vertex itself.
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
        ve: y.contacts.ve.clone(),
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
        let s = common::finished(
            "slab",
            common::brick(slab, (-0.5, 0.5), (-0.5, 0.5), tol),
            tol,
        );
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

/// **Edge-split lineage on both sides of a split, and at it.** `c`, a
/// corner prism hanging below `b` = x∈(0, 1), y∈(−0.5, 0.5),
/// z∈(0, 1), touches `b`'s edge along y at the origin; `b ∪ c`, joined,
/// holds the corner's `(vertex, edge)` record on that edge, whichever
/// way the edge runs. A slab across the edge clear of `c` splits it
/// twice on one side of the corner: at y∈(0.2, 0.3) the record stays on
/// the piece from (0, −0.5, 0), at y∈(−0.3, −0.2) it moves to the piece
/// to (0, 0.5, 0) — one of the two reads "before the split" and the
/// other "past it", in either edge direction. A slab whose face y = 0
/// cuts the edge at the corner itself turns the record into a v-v
/// record with the split's vertex. Every result certifies 3′. Red when
/// lineage never moves a record, always moves it, or reads the split
/// vertex as a side (the piece then does not hold the corner, or the
/// corner sits on the split vertex unrecorded).
///
/// Not pinned: the in-band arm (`VertexOnVertex`). A corner within ε
/// of a split vertex along the edge is also within ε of it in space,
/// and the reduction's own coincidence test meets that first.
#[test]
fn a_vertex_on_edge_record_stays_before_a_split_moves_past_it_and_meets_it() {
    let tol = Tol::witness();
    // A right-handed triple with every ray below z = 0: the prism
    // touches `b` at its corner alone.
    let rays = [[1.0, 0.3, -1.0], [1.0, -0.3, -1.0], [0.2, 0.0, -1.0]];
    let c = crate::union_flush_onto_edge_contact::corner_prism(rays, 0.5, tol);
    let b = common::finished(
        "b",
        common::brick((0.0, 1.0), (-0.5, 0.5), (0.0, 1.0), tol),
        tol,
    );
    let topo::BooleanResult::Body(bc) =
        topo::union_with(&b, &c, &topo::BooleanDeclarations::none(), tol).expect("b ∪ c")
    else {
        panic!("b ∪ c came back empty");
    };
    let (y, _) = bc.join_edges(tol).expect("the join");
    let corner = vertex_at(&y.body, [0.0, 0.0, 0.0]);
    assert_eq!(
        y.contacts
            .ve
            .iter()
            .map(|r| (r.vertex, ends(&y.body, r.edge)))
            .collect::<Vec<_>>(),
        vec![(corner, [[0.0, -0.5, 0.0], [0.0, 0.5, 0.0]])],
        "the join leaves the corner's record on b's edge"
    );
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
        ve: y.contacts.ve.clone(),
        ..topo::CarriedContacts::default()
    };
    let near = |p: [[f64; 3]; 2], q: [[f64; 3]; 2]| {
        p.iter()
            .flatten()
            .zip(q.iter().flatten())
            .all(|(x, y)| (x - y).abs() < 1e-12)
    };
    for (slab, piece) in [
        ((0.2, 0.3), Some([[0.0, -0.5, 0.0], [0.0, 0.2, 0.0]])),
        ((-0.3, -0.2), Some([[0.0, -0.2, 0.0], [0.0, 0.5, 0.0]])),
        ((0.0, 0.3), None),
    ] {
        // The cut at the corner keeps clear of `c` by staying above
        // z = 0; its bottom is then flush with `b`'s.
        let z = if piece.is_some() {
            (-0.5, 0.5)
        } else {
            (0.0, 0.5)
        };
        let s = common::finished("slab", common::brick((-0.5, 0.5), slab, z, tol), tol);
        let mut decls = common::flush_declarations(&y.body, &s, tol);
        decls.carried_a = carried.clone();
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
        match piece {
            Some(piece) => assert!(
                onto.len() == 1 && near(onto[0], piece),
                "slab {slab:?}: the corner's record lands on {piece:?}, got {onto:?}"
            ),
            None => {
                assert!(
                    onto.is_empty(),
                    "slab {slab:?}: no (vertex, edge) record: {onto:?}"
                );
                assert!(
                    out.contacts.vv.iter().any(|r| {
                        let other = if r.a == corner {
                            r.b
                        } else if r.b == corner {
                            r.a
                        } else {
                            return false;
                        };
                        other != corner && point(&out.body, other) == [0.0, 0.0, 0.0]
                    }),
                    "slab {slab:?}: the corner pairs with the split's vertex: {:?}",
                    out.contacts.vv
                );
            }
        }
        assert_eq!(
            errors(&out.body, &out.contacts),
            vec![],
            "slab {slab:?}: the result certifies"
        );
    }
}

/// The unit cube and a box touching it at one point, (0.5, 0, 1): the
/// box's edge runs along (0, 1, 1)/√2 through that point, which is its
/// own edge's midpoint, and its material opens away from the cube on
/// both sides of the cube's top-front edge, so the two edges cross skew
/// there and the bodies meet nowhere else. Built by the boolean, with no
/// declarations: the union crosses the two edges and records the two
/// vertices it cut them at.
fn skew_crossing(tol: Tol) -> topo::BooleanBody<f64> {
    let cube = common::finished(
        "cube",
        common::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let (u, v, w) = ([-r, -0.5, 0.5], [r, -0.5, 0.5], [0.0, r, r]);
    let box_ = common::finished(
        "box",
        common::mapped_cube::<f64>(
            |x, y, z| {
                let at = |k: usize| 0.5 * (x * u[k] + y * v[k] + (z - 0.5) * w[k]);
                geom_core::Point3::new(0.5 + at(0), at(1), 1.0 + at(2))
            },
            tol,
        ),
        tol,
    );
    let topo::BooleanResult::Body(out) =
        topo::union_with(&cube, &box_, &topo::BooleanDeclarations::none(), tol)
            .expect("cube ∪ box")
    else {
        panic!("cube ∪ box came back empty");
    };
    out
}

/// **A crossing the join makes is an edge-edge record.** The union of
/// [`skew_crossing`] holds one v-v record at (0.5, 0, 1), both of its
/// vertices joinable; the join makes each edge whole again and the
/// record an edge-edge record on the two edges, which backs the crossing
/// at rest. Red when a pair landing on two edges records nothing: 3′
/// refuses `EdgeEdgeCross` at the crossing (no vertex is left there for
/// any other record to name).
#[test]
fn a_crossing_the_join_makes_is_an_edge_edge_record() {
    let tol = Tol::witness();
    let out = skew_crossing(tol);
    assert_eq!(
        errors(&out.body, &out.contacts),
        vec![],
        "the union certifies"
    );
    let near = |p: [f64; 3]| {
        p.iter()
            .zip([0.5, 0.0, 1.0])
            .all(|(x, y)| (x - y).abs() < 1e-12)
    };
    assert!(
        out.contacts.vv.len() == 1
            && near(point(&out.body, out.contacts.vv[0].a))
            && near(point(&out.body, out.contacts.vv[0].b)),
        "the union records the crossing's two vertices: {:?}",
        out.contacts.vv
    );
    assert_eq!(
        topo::joinable_vertices(&out.body).len(),
        2,
        "both are joinable"
    );
    let (joined, joins) = out.join_edges(tol).expect("the join");
    assert_eq!(joins.len(), 2);
    let cube_edge = edge_between(&joined.body, [0.0, 0.0, 1.0], [1.0, 0.0, 1.0]);
    assert_eq!(
        joined.contacts.vv,
        vec![],
        "no vertex is left at the crossing"
    );
    assert!(
        joined
            .contacts
            .ee
            .iter()
            .any(|r| r.a == cube_edge || r.b == cube_edge),
        "the crossing is an edge-edge record on the cube's edge: {:?}",
        joined.contacts
    );
    assert_eq!(
        errors(&joined.body, &joined.contacts),
        vec![],
        "the joined union certifies"
    );
}

/// **Edge-edge records at the census, both directions.** Without its
/// record the joined crossing refuses `EdgeEdgeCross`; a record on two
/// edges whose interiors do not meet is stale, typed as the edge-edge
/// kind; and a record on [`edge_touch`]'s two overlapping edges is
/// witnessed by the overlap. Red when the crossing lane ignores the
/// record, the confirm pass leaves the kind out, or the confirm reads an
/// overlap as no meeting.
#[test]
fn an_edge_edge_record_backs_its_crossing_and_is_stale_without_one() {
    let tol = Tol::witness();
    let (joined, _) = skew_crossing(tol).join_edges(tol).expect("the join");
    let unrecorded = topo::ContactRecords {
        ee: vec![],
        ..joined.contacts.clone()
    };
    let es = errors(&joined.body, &unrecorded);
    assert!(
        es.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: CensusContact::EdgeEdgeCross { .. },
                ..
            }
        )),
        "without its record the crossing refuses: {es:?}"
    );
    let apart = topo::EeContact {
        a: edge_between(&joined.body, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        b: edge_between(&joined.body, [0.0, 1.0, 1.0], [1.0, 1.0, 1.0]),
    };
    let stale = topo::ContactRecords {
        ee: [joined.contacts.ee.as_slice(), &[apart]].concat(),
        ..joined.contacts.clone()
    };
    assert_eq!(
        errors(&joined.body, &stale),
        vec![ValidationError::StaleContactDeclaration {
            declaration: StaleDeclaration::EdgeEdge {
                a: apart.a,
                b: apart.b,
            },
        }],
        "a record whose edges do not meet is stale"
    );
    let body = edge_touch();
    let [at_a, at_c] = edge_touch_records(&body);
    let overlap = ContactRecords {
        ve: vec![at_a, at_c],
        ee: vec![topo::EeContact {
            a: at_a.edge,
            b: at_c.edge,
        }],
        ..ContactRecords::default()
    };
    assert_eq!(
        errors(&body, &overlap),
        vec![],
        "an overlap witnesses its record"
    );
}

/// **A carried `(vertex, edge)` row is validated at the door**, as its
/// v-v and v-on-f siblings are: a key that does not resolve in its
/// operand refuses `InvalidDeclaration` before anything runs. Red when
/// the door reads carried v-on-e rows unchecked (the row would reach the
/// substitution door and leave silently).
#[test]
fn a_carried_vertex_on_edge_row_whose_key_does_not_resolve_refuses_at_the_door() {
    let tol = Tol::witness();
    let y = joined_edge_touch(tol);
    let corner = vertex_at(&y.body, [0.5, 0.0, 0.0]);
    let s = common::finished(
        "slab",
        common::brick((0.2, 0.3), (-0.5, 0.5), (-0.5, 0.5), tol),
        tol,
    );
    let decls = topo::BooleanDeclarations {
        carried_a: topo::CarriedContacts {
            ve: vec![VeContact {
                vertex: corner,
                edge: EdgeKey::default(),
            }],
            ..topo::CarriedContacts::default()
        },
        ..topo::BooleanDeclarations::none()
    };
    let got = topo::union_with(&y.body, &s, &decls, tol).map(|_| ());
    assert!(
        matches!(
            got,
            Err(topo::BooleanError::InvalidDeclaration {
                operand: topo::Operand::A,
                what: "carried v-on-e edge key does not resolve",
            })
        ),
        "{got:?}"
    );
}

/// **The edge-on-face bound where the face's own vertex rests on the
/// edge.** A wedge's apex edge lies along the cube's top-face diagonal
/// from (−0.5, −0.5, 1) to (0.5, 0.5, 1): it enters the face at the
/// face's corner (0, 0, 1), which rests on the edge's interior, and ends
/// at a vertex inside the face. With the corner's `(vertex, edge)`
/// record and the end's vertex-on-face record, the overlap is bounded at
/// both ends and the body certifies. Red when the edge-on-face bound
/// where the edge holds no vertex reads only the face rung: the overlap
/// refuses `EdgeFaceOverlap` though its event is recorded.
#[test]
fn an_edge_on_face_bound_at_the_faces_vertex_is_bounded_by_its_record() {
    let tol = Tol::witness();
    let r = std::f64::consts::FRAC_1_SQRT_2;
    let mut wedge = Body::<f64>::new();
    common::prism_ops(
        &mut wedge,
        &[(0.0, 0.0), (0.2, 0.3), (-0.2, 0.3)],
        (-r, r),
        |a, b, t| geom_core::Point3::new(r * (t - a), r * (t + a), 1.0 + b),
        common::FaceGeometry::Certified,
        tol,
    );
    common::describe_as_intersections(&mut wedge, tol);
    let mut body = common::brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
    topo::graft_disjoint_all_keyed(&mut body, &wedge).unwrap();
    let corner = vertex_at(&body, [0.0, 0.0, 1.0]);
    let apex = edge_between(&body, [-0.5, -0.5, 1.0], [0.5, 0.5, 1.0]);
    let end = vertex_at(&body, [0.5, 0.5, 1.0]);
    let top = body
        .faces()
        .map(|(f, _)| f)
        .find(|&f| {
            let outer = body.get_face(f).unwrap().outer;
            let topo::LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
                return false;
            };
            let corners: Vec<_> = body
                .loop_cycle(first)
                .unwrap()
                .iter()
                .map(|&h| point(&body, body.get_half_edge(h).unwrap().start))
                .collect();
            corners.len() == 4 && corners.iter().all(|p| (p[2] - 1.0).abs() < 1e-12)
        })
        .expect("the cube's top");
    let records = ContactRecords {
        a_on_b: vec![VfContact {
            vertex: end,
            face: top,
        }],
        ve: vec![ve(corner, apex)],
        ..ContactRecords::default()
    };
    assert_eq!(
        errors(&body, &records),
        vec![],
        "both bounds recorded: the seat certifies"
    );
}

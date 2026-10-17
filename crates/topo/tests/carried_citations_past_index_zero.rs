//! **A carried citation at an index past 0 still names its record.** A
//! one-record list agrees with any indexing scheme, so here the
//! list carried into an op that touches none of it holds an edge–edge
//! record first and then four vertex pairs, decided across three earlier
//! unions; each citation of the result is followed back through every
//! list to the decision at its end, and checked against the geometry at
//! each hop.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common::{brick, finished};
use geom_core::Tol;
use topo::{
    AtRestBody, Backing, Body, BooleanBody, BooleanInputs, BooleanResult, Cell, Cites, ContactKind,
    ContactRecords, Operand, RowCell,
};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    finished(what, brick(x, y, z, tol()), tol())
}

fn union(a: &AtRestBody<f64>, b: &AtRestBody<f64>, inputs: &BooleanInputs) -> BooleanBody<f64> {
    match topo::boolean_op_with(
        topo::BooleanOp::Union,
        a,
        b,
        inputs,
        topo::SweepStrategy::Realized,
        tol(),
    ) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

fn point(body: &Body<f64>, cell: Cell) -> [f64; 3] {
    let Cell::Vertex(v) = cell else {
        panic!("a vertex cell: {cell:?}")
    };
    let (_, p) = body
        .vertex_points()
        .find(|(k, _)| *k == v)
        .expect("a live vertex");
    [p.x, p.y, p.z]
}

/// Follows one citation of a record standing at `at` back to a decision
/// whose two input cells both lie at `at`; returns the chain's length.
fn follow(cites: &Cites, at: [f64; 3], lists: &[(&BooleanBody<f64>, [&Body<f64>; 2])]) -> usize {
    let Some(((r, inputs), rest)) = lists.split_last() else {
        panic!("a citation past the first op: {cites:?}")
    };
    let mut hops = 0;
    for b in cites.iter() {
        match b {
            Backing::Decided(k) => {
                let row = &r.coincidences[k as usize];
                for c in row.cells {
                    let RowCell::Input { input, cell } = c else {
                        panic!("an input cell: {c:?}")
                    };
                    let body = inputs[usize::from(input == Operand::B)];
                    assert_eq!(
                        point(body, cell),
                        at,
                        "row {k}'s cell {c:?} is at the record"
                    );
                }
                hops = hops.max(1);
            }
            Backing::Carried { input, record } => {
                assert_eq!(input, 0, "only operand A carries records here");
                let (prev, _) = rest.last().expect("an earlier op");
                let cited = &prev.contacts.rows()[record as usize];
                let [x, y] = cited.cells();
                assert_eq!(point(&prev.body, x), at);
                assert_eq!(point(&prev.body, y), at);
                hops = hops.max(1 + follow(&cited.cites, at, rest));
            }
        }
    }
    hops
}

#[test]
fn a_carried_citation_past_index_zero_names_its_record_and_its_decision() {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    let c = box_of((-1.0, 0.0), (-1.0, 0.0), (-1.0, 0.0), "c");
    // On a's edge x = 0, z = 1, end to end.
    let e = box_of((-1.0, 0.0), (0.0, 1.0), (1.0, 2.0), "e");
    let far = box_of((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), "far");
    let carrying = |records: &ContactRecords| BooleanInputs {
        records_a: records.carried(),
        ..BooleanInputs::default()
    };
    let kiss = union(&a, &b, &BooleanInputs::default());
    assert_eq!(kiss.contacts.len(), 1, "{:?}", kiss.contacts);
    let r2 = union(&kiss.body, &c, &carrying(&kiss.contacts));
    assert_eq!(r2.contacts.len(), 2, "{:?}", r2.contacts);
    let r3 = union(&r2.body, &e, &carrying(&r2.contacts));
    // The edge–edge record first, then the vertex pairs: each vertex
    // pair's index in the list is past 0 and past its index among the
    // vertex pairs.
    let (edges, vertices): (Vec<_>, Vec<_>) = r3
        .contacts
        .rows()
        .iter()
        .cloned()
        .partition(|r| matches!(r.kind(), ContactKind::EdgeEdge(..)));
    assert_eq!((edges.len(), vertices.len()), (1, 4), "{:?}", r3.contacts);
    let list: ContactRecords = edges.into_iter().chain(vertices).collect();
    let r4 = union(&r3.body, &far, &carrying(&list));
    assert_eq!(r4.contacts.len(), 5, "{:?}", r4.contacts);
    let mut indices = Vec::new();
    let mut depth = Vec::new();
    for rec in r4.contacts.rows() {
        let [Backing::Carried { input: 0, record }] = rec.cites.iter().collect::<Vec<_>>()[..]
        else {
            panic!("one carried citation: {:?}", rec.cites)
        };
        indices.push(record);
        let cited = &list.rows()[record as usize];
        if let ContactKind::EdgeEdge(..) = rec.kind() {
            assert_eq!(record, 0, "the edge–edge record is the list's first");
            continue;
        }
        let [x, y] = rec.cells();
        let at = point(&r4.body, x);
        assert_eq!(point(&r4.body, y), at, "a vertex pair at one point");
        let [cx, cy] = cited.cells();
        assert_eq!(
            (point(&r3.body, cx), point(&r3.body, cy)),
            (at, at),
            "record {record} of the carried list is the pair at {at:?}"
        );
        depth.push(follow(
            &cited.cites,
            at,
            &[
                (&kiss, [&*a, &*b]),
                (&r2, [&*kiss.body, &*c]),
                (&r3, [&*r2.body, &*e]),
            ],
        ));
    }
    indices.sort_unstable();
    assert_eq!(indices, [0, 1, 2, 3, 4], "each record cited once");
    depth.sort_unstable();
    assert_eq!(
        depth,
        [1, 1, 2, 3],
        "the edge's two ends decided in r3, c's kiss carried from r2, b's from the kiss"
    );
}

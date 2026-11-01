//! **A carried vertex-vertex record ties two keys only where its two
//! points are one.** The output stage puts the two vertices of every
//! vertex-vertex record of the result on one point key
//! (`boolean::ops::record_classes`), deciding each pair's gap Zero
//! first: a carried record lands unread on vertices the op kept, so a
//! record whose two vertices are not one point refuses there as the
//! caller's stale record and never moves a vertex; a record that holds
//! rebinds.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, Body, BooleanError, BooleanInputs, BooleanOp, BooleanResult, Cell, Cited, Cites,
    Contact, ContactRecords, VertexKey,
};

fn tol() -> Tol {
    Tol::witness()
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    common::brick(x, y, z, tol())
}

/// Two unit cubes kissing at `(1, 1, 1)`, grafted with no boolean: the
/// two corners there sit on two point keys, and no record names them.
fn grafted_kiss() -> Body<f64> {
    let mut body = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    topo::graft_disjoint(&mut body, &brick((1.0, 2.0), (1.0, 2.0), (1.0, 2.0))).unwrap();
    body
}

/// The vertices of `body` at `p`.
fn at(body: &Body<f64>, p: [f64; 3]) -> Vec<VertexKey> {
    body.vertex_points()
        .filter(|(_, q)| (*q - Point3::new(p[0], p[1], p[2])).norm() < 1e-12)
        .map(|(k, _)| k)
        .collect()
}

/// `base`, carrying the one vertex-vertex record `(x, y)`, united with a
/// box far from it.
fn carried_union(
    base: &Body<f64>,
    x: VertexKey,
    y: VertexKey,
) -> Result<BooleanResult<f64>, BooleanError> {
    let record: ContactRecords = [Cited::new(
        Contact::new(base, Cell::Vertex(x), Cell::Vertex(y)).unwrap(),
        Cites::decided(0),
    )]
    .into_iter()
    .collect();
    let base = AtRestBody::validate(base.clone(), tol()).unwrap();
    let far = common::finished("far", brick((10.0, 11.0), (10.0, 11.0), (0.0, 1.0)), tol());
    let inputs = BooleanInputs {
        records_a: record.carried(),
        ..BooleanInputs::default()
    };
    topo::boolean_op_with(
        BooleanOp::Union,
        &base,
        &far,
        &inputs,
        topo::SweepStrategy::Realized,
        tol(),
    )
}

/// **A carried record whose vertices are apart refuses stale, and moves
/// nothing.** The grafted kiss carries a record pairing the first cube's
/// corner at `(0, 1, 1)`, or at the origin, with the second cube's at
/// `(1, 1, 1)`: 1 and √3 apart. Each refuses as the operand's stale
/// record, the caller's. Red if the output stage rebinds the pair (the
/// result's gate then refuses `ResultInvalid`, a kernel defect it is not).
#[test]
fn a_carried_vertex_pair_that_is_not_one_point_refuses_stale() {
    let base = grafted_kiss();
    let [_, b] = at(&base, [1.0, 1.0, 1.0])[..] else {
        panic!("two corners at the kiss");
    };
    for p in [[0.0, 1.0, 1.0], [0.0, 0.0, 0.0]] {
        let [a] = at(&base, p)[..] else {
            panic!("one corner at {p:?}");
        };
        let got = carried_union(&base, a, b).map(|_| ());
        assert!(
            matches!(
                got,
                Err(BooleanError::CarryRefused {
                    row: topo::RefusedRow::Carried(topo::Operand::A),
                    refusal: topo::CarryRefusal::Stale { record: 0 },
                })
            ),
            "{p:?}: {got:?}"
        );
    }
}

/// **A carried record whose vertices are one point rebinds them onto one
/// key.** The grafted kiss's two corners at `(1, 1, 1)`, on two keys,
/// carried as one record into a union with a far box: the result keeps
/// the record and puts both corners on one key. Red if a carried record
/// no longer ties keys.
#[test]
fn a_carried_vertex_pair_that_is_one_point_shares_a_key() {
    let base = grafted_kiss();
    let [a, b] = at(&base, [1.0, 1.0, 1.0])[..] else {
        panic!("two corners at the kiss");
    };
    let key = |body: &Body<f64>, v: VertexKey| body.get_vertex(v).unwrap().point;
    assert_ne!(key(&base, a), key(&base, b), "the graft keeps two keys");
    let Ok(BooleanResult::Body(r)) = carried_union(&base, a, b) else {
        panic!("the union builds");
    };
    let [(x, y)] = r.contacts.vertex_vertex().collect::<Vec<_>>()[..] else {
        panic!("the record survives: {:?}", r.contacts);
    };
    assert_eq!(
        key(&r.body, x),
        key(&r.body, y),
        "the corners share one key"
    );
    assert_eq!(
        at(&r.body, [1.0, 1.0, 1.0]).len(),
        2,
        "two corners at the kiss"
    );
}

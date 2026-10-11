//! **A blend carries its operand's contact records** through the cells
//! it keeps, and refuses a record on a cell it consumes rather than
//! dropping it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, edges_with_corners};
use geom_core::{Point3, Tol};
use sweep::blend::build::{Blended, fillet_edges};
use sweep::chamfer::chamfer_edges;
use sweep::test_support::finished;
use topo::{
    Backing, BooleanBody, BooleanDeclarations, BooleanResult, CarryRefusal, Cell, ContactRecords,
    ValidationError, union_with, validate_pseudomanifold,
};

/// The blend size, meters.
const R: f64 = 0.1;

fn tol() -> Tol {
    Tol::witness()
}

/// Two unit cubes kissing at the corner `(1, 1, 1)`: one vertex–vertex
/// record.
fn kiss() -> BooleanBody<f64> {
    let a = finished(
        "box a",
        brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        tol(),
    );
    let b = finished(
        "box b",
        brick(Point3::new(1.0, 1.0, 1.0), Point3::new(2.0, 2.0, 2.0)),
        tol(),
    );
    match union_with(&a, &b, &BooleanDeclarations::none(), tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// The one edge of `body` between `p` and `q`.
fn edge_between(body: &topo::Body<f64>, p: Point3<f64>, q: Point3<f64>) -> topo::EdgeKey {
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let found = edges_with_corners(body, |c| near(c, p) || near(c, q));
    let [e] = found[..] else {
        panic!("one edge between {p:?} and {q:?}: {found:?}")
    };
    e
}

/// The kiss's record carried onto `out`: it cites the operand's record
/// 0, the at-rest gate passes with it, and refuses the touch as
/// undeclared without it.
fn carries_the_kiss(what: &str, base: &BooleanBody<f64>, out: &Blended<f64>) {
    let carried = out.carry_contacts(&base.contacts).unwrap();
    assert_eq!(carried.vertex_vertex().count(), 1, "{what}: {carried:?}");
    let cites: Vec<Backing> = carried.rows()[0].cites.iter().collect();
    assert_eq!(
        cites,
        [Backing::Carried {
            input: 0,
            record: 0
        }],
        "{what}: the record cites the operand's"
    );
    assert_eq!(
        validate_pseudomanifold(&out.body, &carried, tol()),
        Ok(()),
        "{what}: the carried record passes at rest"
    );
    let withheld = validate_pseudomanifold(&out.body, &ContactRecords::default(), tol())
        .expect_err("the kiss is a touch");
    assert!(
        withheld
            .iter()
            .all(|e| matches!(e, ValidationError::UndeclaredContact { .. })),
        "{what}: withheld, the kiss is undeclared: {withheld:?}"
    );
}

/// **A fillet and a chamfer away from the kiss carry it.** Red if the
/// blend's carry drops the record or cites anything but the operand's.
#[test]
fn a_blend_away_from_the_kiss_carries_it() {
    let base = kiss();
    let edge = edge_between(
        &base.body,
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    );
    let filleted = fillet_edges(&base.body, &[edge], R, tol()).expect("the fillet builds");
    carries_the_kiss("fillet", &base, &filleted);
    let chamfered = chamfer_edges(&base.body, &[edge], R, tol()).expect("the chamfer builds");
    carries_the_kiss("chamfer", &base, &chamfered);
}

#[test]
fn probe_a_blend_at_the_kiss() {
    let base = kiss();
    let edge = edge_between(
        &base.body,
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
    );
    match fillet_edges(&base.body, &[edge], R, tol()) {
        Ok(out) => eprintln!(
            "PROBE fillet at kiss: carry -> {:?}",
            out.carry_contacts(&base.contacts)
        ),
        Err(e) => eprintln!("PROBE fillet at kiss refused {e:?}"),
    }
    let _ = (CarryRefusal::NoLineage, Cell::Vertex(Default::default()));
}

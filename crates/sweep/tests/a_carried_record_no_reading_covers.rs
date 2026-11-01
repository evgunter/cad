//! **The carry places a record by lineage, and reads geometry only where
//! an op divided one of its cells** (D1's carry clause). A record whose
//! carriers no census reading covers, a straight edge against a
//! cylinder's wall, rides `plate ∪ rod`'s records into later ops: an op
//! that keeps both cells lands it unread, and an op that divides one
//! puts the pieces to the census's record confirm, which refuses
//! `Unread`. The record is bookkeeping (the plate's far bottom edge
//! touches nothing); what it stands in for is a record minted where it
//! was read, whose carriers the census has no lane for.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::SurfaceKind;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, ProfileLoop};
use sweep::ExtrudeSide;
use sweep::test_support::{brick, finished, sketch_at};
use sweep::{Extrusion, extrude};
use topo::boolean::BooleanResult;
use topo::{AtRestBody, Body, Cell, ContactRecords};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> geom_core::Band {
    geom_core::Band::linear(tol()).unwrap()
}

fn rod() -> AtRestBody<f64> {
    let at = |t: f64| Point2::new(0.5 * t.cos(), 2.5 + 0.5 * t.sin());
    let circle: ProfileLoop<f64> =
        bulge_loop(vec![(at(0.0), 1.0), (at(core::f64::consts::PI), 1.0)]);
    let profile = Profile::new(sketch_at(0.25), vec![circle])
        .validate(tol())
        .unwrap();
    let body: Body<f64> = extrude(
        &profile,
        Extrusion::Distance {
            depth: 0.5,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .unwrap()
    .body;
    finished("rod", body, tol())
}

fn block(what: &str, x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    finished(what, brick::<f64>(x, y, z, tol()), tol())
}

/// `plate ∪ rod` and its one record of the plate's bottom edge at
/// y = −2 against the rod's wall.
fn rested() -> (topo::BooleanBody<f64>, ContactRecords) {
    let plate = block("plate", (-2.0, 2.0), (-2.0, 2.0), (0.0, 1.0));
    let u = match topo::union(&plate, &rod(), tol()) {
        Ok(BooleanResult::Body(u)) => u,
        other => panic!("plate ∪ rod: {:?}", other.map(|_| ())),
    };
    let p = |v: topo::VertexKey| {
        u.body
            .vertex_points()
            .find(|(k, _)| *k == v)
            .map(|(_, p)| p)
            .unwrap()
    };
    let edge = u
        .body
        .edges()
        .map(|(e, _)| e)
        .find(|&e| {
            u.body
                .half_edges()
                .filter(|(_, h)| h.edge == e)
                .all(|(_, h)| {
                    let q = p(h.start);
                    q.y == -2.0 && q.z == 0.0
                })
        })
        .expect("the plate's bottom edge at y = −2");
    let wall = u
        .body
        .faces()
        .find(|(_, f)| u.body.get_surface(f.surface).unwrap().kind() == SurfaceKind::Cylinder)
        .map(|(k, _)| k)
        .expect("the rod's wall");
    let one: ContactRecords = [topo::Cited::new(
        topo::Contact::new(&u.body, Cell::Edge(edge), Cell::Face(wall)).unwrap(),
        topo::Cites::decided(0),
    )]
    .into_iter()
    .collect();
    (u, one)
}

fn carried(
    op: topo::BooleanOp,
    u: &topo::BooleanBody<f64>,
    one: &ContactRecords,
    tool: &AtRestBody<f64>,
) -> Result<BooleanResult<f64>, topo::BooleanError> {
    topo::boolean_op_with(
        op,
        &u.body,
        tool,
        &topo::BooleanInputs {
            records_a: one.carried(),
            ..topo::BooleanInputs::default()
        },
        topo::SweepStrategy::Realized,
        tol(),
    )
}

/// An op that keeps both cells lands the record unread. Red on the
/// carry that confirms every pair, kept ones included: there each op
/// refused `CarryRefused { Carried(A), Unread }`.
#[test]
fn a_record_no_reading_covers_lands_by_lineage_through_an_op_that_keeps_its_cells() {
    let (u, one) = rested();
    let far = block("far", (10.0, 11.0), (10.0, 11.0), (0.0, 1.0));
    let hole = block("hole", (-1.0, -0.5), (-1.0, -0.5), (-1.0, 2.0));
    for (what, op, tool) in [
        ("∪ a far block", topo::BooleanOp::Union, &far),
        ("∖ a hole in the plate", topo::BooleanOp::Subtract, &hole),
    ] {
        match carried(op, &u, &one, tool) {
            Ok(BooleanResult::Body(r)) => assert_eq!(
                r.contacts.edge_face().count(),
                1,
                "{what}: the record lands: {:?}",
                r.contacts
            ),
            other => panic!("{what}: {:?}", other.map(|_| ())),
        }
    }
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, 0.9),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let split = topo::split(&u.body, &plane, tol()).expect("plate ∪ rod splits at z = 0.9");
    let sides = split
        .carry_contacts(&one, band(), Some(topo::RegionLane::certified()))
        .unwrap_or_else(|r| panic!("a split above both cells: the record lands: {r:?}"));
    assert_eq!(
        (sides.below.edge_face().count(), sides.above.len()),
        (1, 0),
        "a split above both cells: the record lands below: {sides:?}"
    );
}

/// An op that divides one of the record's cells puts each piece to the
/// census's record confirm, which has no reading for a line against a
/// cylinder. A notch across the plate's bottom edge divides the edge;
/// a split across the rod divides its wall and the edge.
#[test]
fn a_record_no_reading_covers_refuses_where_an_op_divides_one_of_its_cells() {
    let (u, one) = rested();
    let notch = block("notch", (-1.0, -0.5), (-2.5, -1.5), (-0.5, 0.5));
    let got = carried(topo::BooleanOp::Subtract, &u, &one, &notch).map(|_| ());
    assert!(
        matches!(
            got,
            Err(topo::BooleanError::CarryRefused {
                row: topo::RefusedRow::Carried(topo::Operand::A),
                refusal: topo::CarryRefusal::Unread {
                    record: 0,
                    cells: [Cell::Edge(_), Cell::Face(_)],
                },
            })
        ),
        "∖ a notch across the edge: {got:?}"
    );
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        tol(),
    );
    let split = topo::split(&u.body, &plane, tol()).expect("plate ∪ rod splits at x = 0");
    let got = split.carry_contacts(&one, band(), Some(topo::RegionLane::certified()));
    assert!(
        matches!(
            got,
            Err(topo::CarryRefusal::Unread {
                record: 0,
                cells: [Cell::Edge(_), Cell::Face(_)],
            })
        ),
        "a split across both cells: {got:?}"
    );
}

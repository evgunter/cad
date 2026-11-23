//! **The carry places a record by lineage, and reads geometry only where
//! an op divided one of its cells** (D1's carry clause). A planted
//! record of a straight edge against a cylinder's wall rides
//! `plate ∪ rod`'s records into later ops: an op that keeps both cells
//! lands it unread, and an op that divides one puts the pieces to the
//! census's record confirm, which reads the pair through the curve ×
//! face crossing reading (`census::point::edge_face`). The record is
//! bookkeeping (the plate's far bottom edge touches nothing), so the
//! confirm reads it stale, at the carry where an op divided a cell and
//! at the gate of every body handed it.
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
    .body
    .into_body();
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
        .find(|(fk, _)| u.body.face_surface(*fk).unwrap().kind() == SurfaceKind::Cylinder)
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

/// `plate ∪ rod` handed to the gate with `one` as its records.
fn handed(u: &topo::BooleanBody<f64>, one: &ContactRecords) -> Result<(), topo::NotFinished> {
    topo::finish(
        topo::Built::of(Body::clone(&u.body)).recording(one.clone()),
        tol(),
    )
    .map(|_| ())
}

/// `plate ∪ rod` finished, holding `one` alone, unread: what no door
/// stages (`topo::test_support::unconfirmed`), for the rows that read
/// what a later door makes of it.
fn holding(u: &topo::BooleanBody<f64>, one: &ContactRecords) -> AtRestBody<f64> {
    let bare = topo::finish(topo::Built::of(Body::clone(&u.body)), tol())
        .unwrap_or_else(|e| panic!("plate ∪ rod finishes with no record: {e:?}"));
    topo::test_support::unconfirmed::holding(&bare, one.clone())
}

/// The gate's refusal of the planted record: the census confirms no
/// touch where it lies.
fn stale(findings: &topo::NotFinished) -> bool {
    matches!(
        findings.as_slice(),
        [topo::ValidationError::StaleContactDeclaration { .. }]
    )
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

/// An op that keeps both cells lands the record by lineage, unread by
/// the carry, and the op's gate, which confirms every record it is
/// handed, refuses it: `ResultInvalid`, the census's stale finding, not
/// a carry refusal. The gate refuses the same record on `plate ∪ rod`
/// itself, and a split that keeps both cells on one side lands it there
/// for that side's gate to refuse. Red on the
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
        let got = carried(op, &u, &one, tool).map(|_| ());
        assert!(
            matches!(
                &got,
                Err(topo::BooleanError::ResultInvalid { findings }) if stale(findings)
            ),
            "{what}: the record lands and the gate reads it stale: {got:?}"
        );
    }
    let got = handed(&u, &one);
    assert!(
        matches!(&got, Err(findings) if stale(findings)),
        "the gate refuses the record on `plate ∪ rod`: {got:?}"
    );
    // A split above both cells keeps them on one side: the record lands
    // there by lineage, unread, and that side's gate refuses it.
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, 0.9),
        Vec3::new(0.0, 0.0, 1.0),
        tol(),
    );
    let got = topo::split(&holding(&u, &one), &plane, tol()).map(|_| ());
    assert!(
        matches!(
            &got,
            Err(topo::SplitError::Finish(
                topo::SplitFinishError::ResultNotAtRest { findings, .. }
            )) if stale(findings)
        ),
        "a split above both cells: the record lands and the side's gate reads it stale: {got:?}"
    );
}

/// An op that divides one of the record's cells puts each piece to the
/// census's record confirm, which reads a line against a cylinder: the
/// plate's bottom edge touches the rod's wall nowhere, so each piece
/// reads stale: a notch across the edge drops it from the result, and
/// a split across the rod refuses it.
#[test]
fn a_record_an_op_divides_is_read_and_found_stale() {
    let (u, one) = rested();
    let notch = block("notch", (-1.0, -0.5), (-2.5, -1.5), (-0.5, 0.5));
    let got = carried(topo::BooleanOp::Subtract, &u, &one, &notch);
    match got {
        Ok(BooleanResult::Body(r)) => assert_eq!(
            r.contacts().edge_face().count(),
            0,
            "∖ a notch across the edge: the stale record is dropped: {:?}",
            r.contacts()
        ),
        other => panic!("∖ a notch across the edge: {:?}", other.map(|_| ())),
    }
    // A split across the rod divides both cells: the split's carry puts
    // the pieces to the curve × face reading and refuses the record
    // stale, before any gate.
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        tol(),
    );
    let got = topo::split(&holding(&u, &one), &plane, tol()).map(|_| ());
    assert!(
        matches!(
            got,
            Err(topo::SplitError::Finish(topo::SplitFinishError::Carry {
                refusal: topo::CarryRefusal::Stale { record: 0 },
            }))
        ),
        "a split across both cells: {got:?}"
    );
}

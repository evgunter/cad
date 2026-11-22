//! **A finished body holds no record its gate did not confirm**
//! (`[topology.tiers]`). A deciding door's gate reads each record it is
//! handed where it lies, and a reading that is not the record's
//! confirmation refuses, whatever it is: in the band, on a boundary the
//! region door declines, contradicted. A `⊔`'s gate takes arrivals and
//! reads their records off them; one that does not confirm on the graft
//! refuses there. A move carries the records it holds.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use geom::SurfaceKind;
use geom_core::{Point3, RigidMotion, Tol, Vec3};
use topo::{
    Assembled, AtRestBody, Body, BooleanResult, Built, Cell, Cited, Cites, Contact, ContactRecords,
    FaceKey, FinishedBody, Isometry, LeafBody, ValidationError, finish, finish_assembled, union,
};

fn tol() -> Tol {
    Tol::witness()
}

fn cube(x: f64, y: f64, z: f64) -> AtRestBody<f64> {
    common::finished(
        "cube",
        common::brick::<f64>((x, x + 1.0), (y, y + 1.0), (z, z + 1.0), tol()),
        tol(),
    )
}

/// The planar face of `body` whose every vertex has `x` at `at` and `y`
/// in the unit span from `corner`.
fn face_at_x(body: &Body<f64>, at: f64, corner: f64) -> FaceKey {
    let points: std::collections::BTreeMap<_, _> = body.vertex_points().collect();
    body.faces()
        .map(|(f, _)| f)
        .find(|&f| {
            let mut starts = body
                .half_edges()
                .filter(|(he, _)| body.face_of_half_edge(*he) == Some(f))
                .map(|(_, h)| points[&h.start])
                .peekable();
            body.face_surface(f).map(geom::Surface::kind) == Some(SurfaceKind::Plane)
                && starts.peek().is_some()
                && starts.all(|p| p.x == at && p.y >= corner - 1e-6 && p.y <= corner + 1.0 + 1e-6)
        })
        .unwrap_or_else(|| panic!("a face at x = {at}"))
}

fn face_face(body: &Body<f64>, a: FaceKey, b: FaceKey) -> ContactRecords {
    [Cited::new(
        Contact::new(body, Cell::Face(a), Cell::Face(b)).unwrap(),
        Cites::decided(0),
    )]
    .into_iter()
    .collect()
}

/// Two cubes as one body of two solids, `gap` apart along x, the second
/// raised `lift` along y.
fn two_cubes(gap: f64, lift: f64) -> AtRestBody<f64> {
    let mut joined = Assembled::<f64>::new();
    joined
        .graft_finished(&cube(0.0, 0.0, 0.0), None, tol())
        .unwrap();
    joined
        .graft_finished(&cube(1.0 + gap, lift, 0.0), None, tol())
        .unwrap();
    finish_assembled(joined, tol()).unwrap().body
}

/// Two faces a nanometre apart, recorded as touching: the census reads
/// their carriers' offset in the band and escalates, which is not a
/// confirmation, and the gate refuses. Red on a gate that refuses only the stale and the
/// contradicted: it held the record.
#[test]
fn a_record_the_census_reads_in_the_band_refuses() {
    let two = two_cubes(1e-9, 0.0);
    let record = face_face(
        &two,
        face_at_x(&two, 1.0, 0.0),
        face_at_x(&two, 1.0 + 1e-9, 0.0),
    );
    let got = finish(Built::of(Body::clone(&two)).recording(record), tol());
    let findings = got.expect_err("a record read in the band is not confirmed");
    assert!(
        matches!(
            findings.as_slice(),
            [ValidationError::CensusEscalated { .. }]
        ),
        "{findings:?}"
    );
}

/// Two faces on one plane, facing each other, that meet at an edge
/// alone, recorded as touching: the region door declines the pair
/// (their boundaries touch), which is not a confirmation, and the gate
/// refuses. Red on a gate that refuses only the stale and the
/// contradicted.
#[test]
fn a_record_the_region_door_declines_refuses() {
    let two = two_cubes(0.0, 1.0);
    let record = face_face(&two, face_at_x(&two, 1.0, 0.0), face_at_x(&two, 1.0, 1.0));
    let got = finish(Built::of(Body::clone(&two)).recording(record), tol());
    let findings = got.expect_err("a pair the region door declines is not confirmed");
    assert!(
        matches!(
            findings.as_slice(),
            [ValidationError::CensusUnsupported { .. }]
        ),
        "{findings:?}"
    );
}

/// An arrival holding a record of two faces that do not meet: the
/// `⊔`'s gate reads what its arrivals carried in, and refuses it. Red on
/// a gate that holds every carried record and reports the contradiction
/// beside the finished body.
#[test]
fn a_joins_gate_refuses_a_record_an_arrival_carried_that_does_not_confirm() {
    let kiss = common::unconfirmed::kiss_holding_a_false_record(tol());
    let mut joined = Assembled::<f64>::new();
    joined.graft_finished(&kiss, None, tol()).unwrap();
    joined
        .graft_finished(&cube(10.0, 10.0, 10.0), None, tol())
        .unwrap();
    let findings = finish_assembled(joined, tol())
        .map(|censused| censused.body.records().len())
        .expect_err("the false record refuses at the join's gate");
    assert!(
        common::unconfirmed::the_false_record_alone(&findings),
        "{findings:?}"
    );
}

/// A corner kiss and its record.
fn kiss() -> AtRestBody<f64> {
    match union(&cube(0.0, 0.0, 0.0), &cube(1.0, 1.0, 1.0), tol()).unwrap() {
        BooleanResult::Body(kiss) => kiss.body,
        BooleanResult::Empty => panic!("the kiss is empty"),
    }
}

/// A finished body moved, or mirrored, holds the records it held, on
/// the cells it held them on: an isometry re-mints no key. As a leaf at
/// `f64` the image is finished again, and its gate confirms them. Red
/// on a move that drops the records.
#[test]
fn a_move_and_a_mirror_keep_the_records() {
    let kiss = kiss();
    let held: Vec<_> = kiss.records().kinds().collect();
    assert_eq!(held.len(), 1, "the kiss holds its one record: {held:?}");
    let slide = Isometry::Proper(RigidMotion::axes_xy(Point3::new(3.0, -2.0, 0.5)));
    let flip = Isometry::Improper(
        topo::Reflection::across(Point3::new(0.25, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), tol())
            .unwrap(),
    );
    for (what, iso) in [("moved", &slide), ("mirrored", &flip)] {
        let image = FinishedBody::moved_by(&kiss, iso, tol()).unwrap();
        assert_eq!(
            image.records().kinds().collect::<Vec<_>>(),
            held,
            "{what}: a finished body"
        );
        let image = LeafBody::moved_by(&kiss, iso, tol()).unwrap();
        assert_eq!(
            LeafBody::records(&image).kinds().collect::<Vec<_>>(),
            held,
            "{what}: a leaf"
        );
    }
}

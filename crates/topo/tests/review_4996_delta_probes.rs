//! Delta review of PR 4996's second fix pass: a hand-built (`Default`)
//! offset outcome carries an empty source, so every record is read.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, finished};
use geom_core::{Point3, Tol};
use topo::{AtRestBody, BooleanBody, BooleanResult, Cell, ContactRecords, Surface, union};

fn tol() -> Tol {
    Tol::witness()
}

fn box_of(x: (f64, f64), y: (f64, f64), z: (f64, f64), what: &str) -> AtRestBody<f64> {
    finished(what, brick(x, y, z, tol()), tol())
}

fn kiss() -> BooleanBody<f64> {
    let a = box_of((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), "a");
    let b = box_of((1.0, 2.0), (1.0, 2.0), (1.0, 2.0), "b");
    match union(&a, &b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// The kiss carries its real record and a bookkeeping one (a's origin
/// corner against b's far face x = 2, no touch) through a `Default`
/// outcome over the unchanged body. The real one is read and lands; the
/// bookkeeping one is read apart. Prints what becomes of it: consumed
/// (the empty source reads every image as moved) or `Stale`.
#[test]
fn a_default_offset_outcome_reads_every_record() {
    let base = kiss();
    let corner = base
        .body
        .vertex_points()
        .find(|(_, p)| (*p - Point3::new(0.0, 0.0, 0.0)).norm() < 1e-12)
        .unwrap()
        .0;
    let far = base
        .body
        .faces()
        .find(|(_, f)| {
            matches!(
                base.body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.x.abs() == 1.0 && origin.x == 2.0
            )
        })
        .unwrap()
        .0;
    let band = geom_core::Band::linear(tol()).unwrap();
    let region = Some(topo::RegionLane::certified());
    let outcome = topo::OffsetOutcome::default();
    let real = outcome.carry_contacts(&base.contacts, &base.body, band, region);
    assert_eq!(real.as_ref().map(ContactRecords::len), Ok(1), "{real:?}");
    let false_one: ContactRecords = [topo::Cited::new(
        topo::Contact::new(&base.body, Cell::Vertex(corner), Cell::Face(far)).unwrap(),
        topo::Cites::decided(0),
    )]
    .into_iter()
    .collect();
    let got = outcome.carry_contacts(&false_one, &base.body, band, region);
    println!("bookkeeping record through a Default outcome: {got:?}");
    assert!(
        !matches!(&got, Ok(r) if !r.is_empty()),
        "never lands unread: {got:?}"
    );
}

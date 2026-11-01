//! The carry by lineage through an op that keeps a cell's key while
//! moving its geometry: the cell's image no longer holds its geometry,
//! and a record on it is confirmed.
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use topo::{AtRestBody, Body, BooleanBody, BooleanResult, Cell, FaceKey, Surface, union};

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

fn faces_at_x(body: &Body<f64>, x: f64) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.x.abs() == 1.0 && origin.x == x
            )
        })
        .map(|(k, _)| k)
        .collect()
}

fn point(body: &Body<f64>, c: Cell) -> geom_core::Point3<f64> {
    let Cell::Vertex(v) = c else { panic!("{c:?}") };
    body.vertex_points().find(|(k, _)| *k == v).unwrap().1
}

/// **An offset that moves one kissing corner off the kiss consumes the
/// record.** The offset door keeps every key and hands its lineage a
/// snapshot of the input's geometry; the moved face, its edges and
/// corners, and the faces and edges around them no longer hold it. The kiss record on a
/// moved corner goes to the census's confirm, reads apart, and is
/// consumed, as a record on a divided cell with its remainder removed
/// is. Red on the carry that read the moved corner as kept: the record
/// then landed unread on two vertices 0.1 apart.
#[test]
fn an_offset_moving_a_kissing_corner_off_the_kiss_consumes_the_record() {
    let base = kiss();
    assert_eq!(base.contacts.len(), 1);
    let [x, y] = base.contacts.rows()[0].record.cells();
    let faces = faces_at_x(&base.body, 1.0);
    assert_eq!(faces.len(), 2, "each cube's face at x = 1");
    for face in faces {
        for d in [-0.1, 0.1] {
            let mut moved = Body::clone(&base.body);
            let outcome = topo::replace_face_offset(&mut moved, face, d, tol())
                .unwrap_or_else(|e| panic!("face {face:?} by {d}: the offset builds: {e:?}"));
            let gap = (point(&moved, x) - point(&moved, y)).norm();
            assert!(gap > 0.09, "face {face:?} by {d}: the corners part: {gap}");
            let carried = outcome.carry_contacts(
                &base.contacts,
                &moved,
                geom_core::Band::linear(tol()).unwrap(),
                Some(topo::RegionLane::certified()),
            );
            assert_eq!(
                carried,
                Ok(topo::ContactRecords::default()),
                "face {face:?} by {d}: the kiss is consumed"
            );
        }
    }
}

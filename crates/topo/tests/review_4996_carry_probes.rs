//! Review rows for PR 4996 (the carry by lineage): an op that keeps a
//! cell's key while moving its geometry.
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

/// **An offset that moves one kissing corner away keeps the kiss
/// record.** The offset door keeps every key, so its lineage calls the
/// moved face and its corner vertex kept; the carry then lands the
/// vertex–vertex record unread on two vertices that no longer meet.
#[test]
fn an_offset_moving_a_kissing_corner_away_lands_the_kiss_unread() {
    let base = kiss();
    assert_eq!(base.contacts.len(), 1);
    let [x, y] = base.contacts.rows()[0].record.cells();
    let faces = faces_at_x(&base.body, 1.0);
    assert!(!faces.is_empty());
    for face in faces {
        for d in [-0.1, 0.1] {
            let mut moved = Body::clone(&base.body);
            let Ok(outcome) = topo::replace_face_offset(&mut moved, face, d, tol()) else {
                eprintln!("face {face:?} d {d}: offset refused");
                continue;
            };
            let carried = outcome.carry_contacts(
                &base.contacts,
                &moved,
                geom_core::Band::linear(tol()).unwrap(),
                Some(topo::RegionLane::certified()),
            );
            let gap = (point(&moved, x) - point(&moved, y)).norm();
            let rest = carried.as_ref().ok().map(|c| {
                topo::validate_pseudomanifold(&moved, c, tol())
            });
            eprintln!(
                "face {face:?} d {d}: corners {:?} {:?} gap {gap}; carry {carried:?}; at rest {rest:?}",
                point(&moved, x),
                point(&moved, y)
            );
            if gap > 1e-3 {
                assert!(
                    !matches!(&carried, Ok(c) if c.len() == 1),
                    "a record on corners {gap} apart lands: {carried:?}"
                );
            }
        }
    }
}

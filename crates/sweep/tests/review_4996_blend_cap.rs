//! **A cap a fillet band carves is read, not kept.** A fillet of radius
//! 0.1 on a unit cube's edge along x at y = z = 0 ends on the faces
//! x = 0 and x = 1 and carves a corner off each. The blend keeps each
//! cap's key, but its boundary is other edges now, so a record on it
//! goes to the census's confirm rather than landing unread. A
//! bookkeeping record (the far corner (0, 1, 1) against the cap x = 1,
//! no touch) is read apart and consumed. Red on the lineage that called
//! a carved cap kept: the record then landed.
//!
//! A body touching a cap inside the carved corner, the record that
//! would land false, is refused before any carry: a tilted cube balanced
//! on its corner at (1, 0.01, 0.01) makes the battery refuse
//! `FaceClearance` on that cube's face.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::edges_with_corners;
use geom_core::{Point3, Tol};
use sweep::blend::build::fillet_edges;
use sweep::test_support::{brick, finished};
use topo::{Body, Cell, FaceKey, Surface};

fn tol() -> Tol {
    Tol::witness()
}

fn cap(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.x.abs() == 1.0 && origin.x == 1.0
            )
        })
        .map(|(k, _)| k)
        .unwrap()
}

#[test]
fn a_record_on_a_cap_a_band_carves_is_read() {
    let cube = finished(
        "cube",
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let found = edges_with_corners(&cube, |c| {
        near(c, Point3::new(0.0, 0.0, 0.0)) || near(c, Point3::new(1.0, 0.0, 0.0))
    });
    let [edge] = found[..] else {
        panic!("{found:?}")
    };
    let far = cube
        .vertex_points()
        .find(|(_, p)| near(*p, Point3::new(0.0, 1.0, 1.0)))
        .map(|(k, _)| k)
        .unwrap();
    let record: topo::ContactRecords = [topo::Cited::new(
        topo::Contact::new(&cube, Cell::Vertex(far), Cell::Face(cap(&cube))).unwrap(),
        topo::Cites::decided(0),
    )]
    .into_iter()
    .collect();
    let out = fillet_edges(&cube, &[edge], 0.1, tol()).expect("the fillet builds");
    assert_eq!(
        out.carry_contacts(
            &record,
            geom_core::Band::linear(tol()).unwrap(),
            Some(topo::RegionLane::certified())
        ),
        Ok(topo::ContactRecords::default()),
        "the record on the carved cap is read apart and consumed"
    );
}

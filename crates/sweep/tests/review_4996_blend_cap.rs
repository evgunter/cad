//! **A cap a fillet band carves is reshaped.** A fillet of radius 0.1
//! on a unit cube's edge along x at y = z = 0 ends on the faces x = 0
//! and x = 1 and carves a corner off each. The blend keeps each cap's
//! key; its lineage states it reshaped, so a record on it goes to the
//! census's confirm rather than landing unread. Red on the lineage that
//! called a carved cap kept.
//!
//! A body touching a cap inside the carved corner, the record that
//! would then land false, is refused before any carry: a tilted cube
//! balanced on its corner at (1, 0.01, 0.01) makes the battery refuse
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

fn caps(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { normal, .. }) if normal.x.abs() == 1.0
            )
        })
        .map(|(k, _)| k)
        .collect()
}

#[test]
fn the_caps_a_band_carves_are_reshaped() {
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
    let out = fillet_edges(&cube, &[edge], 0.1, tol()).expect("the fillet builds");
    let lineage = out.lineage();
    let caps = caps(&cube);
    assert_eq!(caps.len(), 2, "the faces x = 0 and x = 1");
    for f in caps {
        assert!(
            matches!(lineage.fate(Cell::Face(f)), topo::Fate::Divided(_)),
            "the carved cap {f:?} is reshaped: {:?}",
            lineage.fate(Cell::Face(f))
        );
    }
}

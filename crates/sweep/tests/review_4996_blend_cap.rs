//! Review probe for PR 4996: what the blend's lineage says of the cap
//! face at a cap-terminated band's end, whose corner the band carves.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, edges_with_corners};
use geom_core::{Point3, Tol};
use sweep::blend::build::fillet_edges;
use sweep::test_support::finished;
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

fn sides(body: &Body<f64>, f: FaceKey) -> Vec<topo::EdgeKey> {
    body.half_edges()
        .filter(|(_, h)| body.get_loop(h.parent_loop).unwrap().face == f)
        .map(|(_, h)| h.edge)
        .collect()
}

#[test]
fn the_cap_a_band_carves_is_kept() {
    let body = finished(
        "cube",
        brick(Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)),
        tol(),
    );
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let found = edges_with_corners(&body, |c| {
        near(c, Point3::new(0.0, 0.0, 0.0)) || near(c, Point3::new(1.0, 0.0, 0.0))
    });
    let [edge] = found[..] else { panic!("{found:?}") };
    let out = fillet_edges(&body, &[edge], 0.1, tol()).expect("the fillet builds");
    let f = cap(&body);
    let before = sides(&body, f);
    let after = sides(&out.body, f);
    let fate = out.lineage().fate(Cell::Face(f));
    eprintln!("cap {f:?}: fate {fate:?}; sides before {before:?}; after {after:?}");
    for (k, _) in body.faces() {
        eprintln!("face {k:?}: {:?}", out.lineage().fate(Cell::Face(k)));
    }
    assert!(
        !matches!(fate, topo::Fate::Kept { .. }) || after.iter().all(|e| before.contains(e)),
        "the cap's boundary gained an edge the band carved, yet its fate is Kept: {fate:?}"
    );
}

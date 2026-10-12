//! Review probe (R2, PR 4809): a support face's removed remainder
//! consumes a record on it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::cavity::{brick, edges_with_corners};
use geom_core::{Point3, Tol};
use sweep::blend::build::fillet_edges;
use sweep::test_support::finished;
use topo::{BooleanBody, BooleanResult, union};

fn tol() -> Tol {
    Tol::witness()
}

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
    match union(&a, &b, tol()) {
        Ok(BooleanResult::Body(r)) => r,
        other => panic!("the union builds: {other:?}"),
    }
}

/// A bookkeeping record (cube a's bottom face against the far cube's top
/// corner, no touch) through a fillet of a's bottom-front edge, whose
/// support the bottom face is.
#[test]
fn r2_a_record_on_a_support_face_is_consumed() {
    let base = kiss();
    let near = |a: Point3<f64>, b: Point3<f64>| (a - b).norm() < 1e-9;
    let (p, q) = (Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0));
    let found = edges_with_corners(&base.body, |c| near(c, p) || near(c, q));
    let [edge] = found[..] else {
        panic!("{found:?}")
    };
    let bottom = base.body.faces().find(|(_, f)| matches!(base.body.get_surface(f.surface),
        Some(topo::Surface::Plane { origin, normal, .. }) if normal.z.abs() == 1.0 && origin.z == 0.0))
        .map(|(k, _)| k).unwrap();
    let far = base
        .body
        .vertices()
        .find(|(_, v)| {
            let x = base.body.get_point(v.point).unwrap();
            near(*x, Point3::new(2.0, 2.0, 2.0))
        })
        .map(|(k, _)| k)
        .unwrap();
    let mut records = base.contacts.clone();
    records.extend([topo::Cited::new(
        topo::Contact::new(
            &base.body,
            topo::Cell::Vertex(far),
            topo::Cell::Face(bottom),
        )
        .unwrap(),
        topo::Cites::decided(7),
    )]);
    let out = fillet_edges(&base.body, &[edge], 0.1, tol()).expect("the fillet builds");
    eprintln!(
        "R2PROBE support fate: {:?}",
        out.lineage().fate(topo::Cell::Face(bottom))
    );
    let got = out.carry_contacts(
        &records,
        geom_core::Band::linear(tol()).unwrap(),
        Some(topo::RegionLane::certified()),
    );
    eprintln!("R2PROBE support carry: {got:?}");
    assert_eq!(
        got.map(|r| r.len()),
        Ok(1),
        "the bookkeeping record is consumed, the kiss carries"
    );
}

//! probe
#![allow(clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use topo::Body;
use topo::test_support::{brick, finished};

#[test]
fn probe() {
    let tol = Tol::witness();
    let empty = finished("an empty body", Body::<f64>::new(), tol);
    let block = finished(
        "a block",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let plane = topo::test_support::split_plane(Point3::new(0.0, 0.0, 0.5), Vec3::unit_z(), tol);
    eprintln!("split: {:?}", topo::split(&empty, &plane, tol).map(|_| ()));
    for (n, r) in [
        ("union e,b", topo::union(&empty, &block, tol)),
        ("union b,e", topo::union(&block, &empty, tol)),
        ("union e,e", topo::union(&empty, &empty, tol)),
        ("intersect e,b", topo::intersect(&empty, &block, tol)),
        ("intersect b,e", topo::intersect(&block, &empty, tol)),
        ("subtract e,b", topo::subtract(&empty, &block, tol)),
        ("subtract b,e", topo::subtract(&block, &empty, tol)),
    ] {
        eprintln!("{n}: {:?}", r.map(|b| b.body().is_some()));
    }
    panic!("probe");
}

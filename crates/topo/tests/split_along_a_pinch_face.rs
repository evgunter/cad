//! **A plane the pinch's two bricks meet on puts each brick on its own
//! side.** Two bricks united along the line `(1, 1, z)`, split by the
//! plane `x = 1`, which holds a face of each and cuts neither: the split
//! has no section, yet brick `a` lies below the plane and brick `b`
//! above it. Each side is that brick alone, every vertex on its side of
//! the plane. Red when an uncut operand lands whole on the side its
//! first vertex reads, `b` with it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{brick, finished, split_plane};
use geom_core::{Point3, Tol, Vec3};
use topo::{BooleanResult, SplitPart, split, union};

#[test]
fn a_plane_through_a_pinch_puts_each_brick_on_its_side() {
    let tol = Tol::witness();
    let a = finished(
        "a",
        brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
        tol,
    );
    let b = finished(
        "b",
        brick::<f64>((1.0, 2.0), (1.0, 2.0), (0.0, 1.0), tol),
        tol,
    );
    let BooleanResult::Body(pinch) = union(&a, &b, tol).unwrap() else {
        panic!("the pinch is empty");
    };
    let plane = split_plane(Point3::new(1.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0), tol);
    let r = split(&pinch.body, &plane, tol).unwrap();
    for (what, side, x) in [
        ("above", r.above, (1.0, 2.0)),
        ("below", r.below, (0.0, 1.0)),
    ] {
        let SplitPart::Body(body) = side else {
            panic!("{what}: a brick lies on this side, read empty");
        };
        let xs: Vec<f64> = body.vertex_points().map(|(_, p)| p.x).collect();
        assert!(
            xs.len() == 8 && xs.iter().all(|&px| px == x.0 || px == x.1),
            "{what}: the side is one brick over x in {x:?}, read vertices at x = {xs:?}"
        );
    }
}

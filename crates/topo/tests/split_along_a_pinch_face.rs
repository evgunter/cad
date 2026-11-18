//! **A plane the pinch's two bricks meet on puts each brick on its own
//! side.** Two bricks united along the line `(1, 1, z)`, split by the
//! plane `x = 1`, which holds a face of each and cuts neither: the split
//! has no section, yet brick `a` lies below the plane and brick `b`
//! above it. Each side is that brick alone, every vertex on its side of
//! the plane. Red when an uncut operand lands whole on the side its
//! first vertex reads, `b` with it. The pinch's own record is parted by
//! that plane, and the door refuses it.
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
    // The pinch's record is a touch between the two sides, and neither
    // side holds a face of the cut for it to lie on, so the split's carry
    // has no rule that retires it
    // (`work/wirehold/a-record-the-split-parts-has-no-consumption-rule.md`):
    // the door refuses it, never drops it.
    assert!(
        matches!(
            split(&pinch.body, &plane, tol),
            Err(topo::SplitError::Carry {
                refusal: topo::CarryRefusal::Parted { record: 0 }
            })
        ),
        "the pinch's record is parted off any face of the cut"
    );
    // The same bits with the record given up split as the plane parts
    // them.
    let bare = topo::finish(pinch.body.into_body(), tol).expect("the pinch finishes bare");
    let r = split(&bare, &plane, tol).unwrap();
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

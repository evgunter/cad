//! R1 attribution probe (runs on MAIN): is the coplanar-split tier-3
//! refusal pre-existing, or did PCURVE P-1b introduce it?
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

#[test]
fn coplanar_split_tier3_verdict_on_this_head() {
    let notched = ProfileLoop::polygon(
        [
            (0.0, 0.0),
            (8.0, 0.0),
            (8.0, 2.0),
            (7.0, 1.0),
            (6.0, 1.0),
            (5.0, 2.0),
            (4.0, 1.0),
            (3.0, 2.0),
            (0.0, 2.0),
        ]
        .map(|(x, y)| p2(x, y)),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![notched])
        .validate(Tol::witness())
        .unwrap();
    let body = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    assert_eq!(topo::validate_geometric(&body, Tol::witness()), Ok(()));
    let result = topo::split(
        &body,
        &topo::SplitPlane {
            origin: Point3::new(0.0, 1.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
        },
        Tol::witness(),
    )
    .expect("the face-coplanar split runs");
    for (side, part) in [("above", &result.above), ("below", &result.below)] {
        let b = part.body().expect("material");
        println!(
            "{side}: validate_geometric = {:?}",
            topo::validate_geometric(b, Tol::witness())
        );
    }
}

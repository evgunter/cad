//! **A plane grazing a curved edge never leaves a silent pinch**
//! (`work/contacthold/zero-to-distinct-cells-on-one-point-doors-audit.md`,
//! the split's door): a split leaves two vertices of one part on one
//! point key only at an ON vertex with two runs, which it records.
#![allow(clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use sweep::test_support::{brick, prism_on, sketch_from_axes};
use topo::test_support::finished;
use topo::{AtRestBody, BooleanResult, SectionGermFault, SplitError, SplitReduceError};

fn tol() -> Tol {
    Tol::witness()
}

/// Whether some part holds two vertices on one point key at `(1, 0, 0)`.
fn pinched_at_the_graze(parts: [Option<&topo::Body<f64>>; 2]) -> bool {
    parts.into_iter().flatten().any(|body| {
        let mut keys: Vec<_> = body
            .vertex_points()
            .filter(|(_, p)| (p.x, p.y, p.z) == (1.0, 0.0, 0.0))
            .filter_map(|(v, _)| body.get_vertex(v).map(|v| v.point))
            .collect();
        let n = keys.len();
        keys.sort();
        keys.dedup();
        keys.len() < n
    })
}

/// A unit rod on `z ∈ [0, 2]` and the block it bores, each split by the
/// plane through its rim point `(1, 0, 0)` at three normals: tilted
/// `±z` (grazing the rim circle alone) and `(1, 0, 0)` (tangent to the
/// side along its ruling). The convex rod lands at each with no row and
/// no pinch. The concave bore refuses `SectionGermsUnordered
/// { Coincident }` tilted `+z` and `KnifeEdge` tangent, and lands tilted
/// `−z`, its rim's ON point no pinch. Red if a graze lands with two
/// vertices of one part on one key and no row.
#[test]
fn a_grazed_rim_lands_whole_or_refuses_typed() {
    let plane = sketch_from_axes(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        tol(),
    );
    let disc = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    let rod = finished("the rod", prism_on(plane, disc, 2.0, tol()), tol());
    let block = finished(
        "the block",
        brick((-2.0, 2.0), (-2.0, 2.0), (0.0, 2.0), tol()),
        tol(),
    );
    let bored: AtRestBody<f64> = match topo::subtract(&block, &rod, tol()) {
        Ok(BooleanResult::Body(r)) => r.body,
        other => panic!("the bore builds: {other:?}"),
    };
    let split = |body: &AtRestBody<f64>, n: [f64; 3]| {
        let len = n.iter().map(|c| c * c).sum::<f64>().sqrt();
        topo::split(
            body,
            &topo::test_support::split_plane(
                Point3::new(1.0, 0.0, 0.0),
                Vec3::new(n[0] / len, n[1] / len, n[2] / len),
                tol(),
            ),
            tol(),
        )
    };
    let (up, down, side) = ([1.0, 0.0, 0.3], [1.0, 0.0, -0.3], [1.0, 0.0, 0.0]);
    for (what, n) in [("+z", up), ("-z", down), ("tangent", side)] {
        match split(&rod, n) {
            Ok(s) => {
                assert!(s.coincidences.is_empty(), "the rod {what}: no row");
                assert!(
                    !pinched_at_the_graze([s.above.body(), s.below.body()]),
                    "the rod {what}: no pinch"
                );
            }
            Err(e) => panic!("the rod {what} lands: {e:?}"),
        }
    }
    match split(&bored, up) {
        Err(SplitError::Reduce(SplitReduceError::SectionGermsUnordered {
            fault: SectionGermFault::Coincident,
            ..
        })) => {}
        other => panic!("the bore +z refuses with unordered germs: {other:?}"),
    }
    match split(&bored, side) {
        Err(SplitError::Reduce(SplitReduceError::KnifeEdge(_))) => {}
        other => panic!("the bore tangent refuses a knife edge: {other:?}"),
    }
    match split(&bored, down) {
        Ok(s) => assert!(
            !pinched_at_the_graze([s.above.body(), s.below.body()]),
            "the bore -z: no pinch at the rim: {:?}",
            s.coincidences
        ),
        Err(e) => panic!("the bore -z lands: {e:?}"),
    }
}

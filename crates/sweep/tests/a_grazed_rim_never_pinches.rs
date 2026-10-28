//! **A plane grazing a curved edge never leaves a silent pinch**
//! (`work/contacthold/zero-to-distinct-cells-on-one-point-doors-audit.md`,
//! the split's door): a split leaves two vertices of one part on one
//! point key only at an ON vertex with two runs, which it records.
#![allow(clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::SketchPlane;
use sweep::test_support::{brick, prism_on};
use topo::test_support::{finished, pinned_volume};
use topo::{AtRestBody, BooleanResult, SplitError, SplitReduceError};

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
/// no pinch. The concave bore refuses `KnifeEdge` tangent, and lands
/// tilted `±z`, its two sides' volumes summing to its own: tilted `−z`
/// its rim's ON point no pinch; tilted `+z` the trace touches the rim
/// with material on both sides, the piece between them two horns
/// meeting at the point, a pinch with its row. Red if a graze lands
/// with two vertices of one part on one key and no row.
#[test]
fn a_grazed_rim_lands_whole_or_refuses_typed() {
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, 0.0));
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
                    !pinched_at_the_graze([
                        s.above.body().map(|b| &**b),
                        s.below.body().map(|b| &**b)
                    ]),
                    "the rod {what}: no pinch"
                );
            }
            Err(e) => panic!("the rod {what} lands: {e:?}"),
        }
    }
    match split(&bored, side) {
        Err(SplitError::Reduce(SplitReduceError::KnifeEdge(_))) => {}
        other => panic!("the bore tangent refuses a knife edge: {other:?}"),
    }
    let whole = (16.0 - PI) * 2.0;
    for (what, n, horns) in [("+z", up, true), ("-z", down, false)] {
        match split(&bored, n) {
            Ok(s) => {
                let parts = [s.above.body(), s.below.body()];
                assert_eq!(
                    pinched_at_the_graze(parts.map(|b| b.map(|b| &**b))),
                    horns,
                    "the bore {what}: a pinch at the rim: {:?}",
                    s.coincidences
                );
                assert_eq!(
                    s.coincidences.is_empty(),
                    !horns,
                    "the bore {what}: a row with the pinch"
                );
                let (mut kept, mut pad) = (0.0, 0.0);
                for b in parts.into_iter().flatten() {
                    let m = pinned_volume(what, b, tol());
                    (kept, pad) = (kept + m.volume, pad + m.volume_pad);
                }
                assert!(
                    (kept - whole).abs() <= pad + 1e-9 * whole,
                    "the bore {what}: the sides keep {kept} ± {pad} of {whole}"
                );
            }
            Err(e) => panic!("the bore {what} lands: {e:?}"),
        }
    }
}

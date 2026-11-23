//! **A joinable vertex keeps a body from finishing** (tier 3's check 11;
//! Ev's PR 4251 ruling), so no door is handed one: the finish refuses
//! the hand-split cube at the base scalar, carrying check 11's finding
//! at the split vertex. A dual's finish is tier 2 alone and does not read
//! check 11, so this row lives at `f64`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use topo::ValidationError;
use topo::construction::Construct;

#[test]
fn a_split_cube_does_not_finish() {
    let tol = Tol::witness();
    let mut cube = sweep::test_support::cube::<f64>(1.0, tol);
    let (edge, curve) = cube.edges().next().map(|(k, d)| (k, d.curve)).unwrap();
    let (t0, t1) = cube
        .get_curve_geom(curve)
        .and_then(topo::CurveGeom::certified)
        .unwrap()
        .params();
    let split = cube.split_edge(edge, (t0 + t1) * 0.5, tol).unwrap();
    let refused = topo::finish(cube, tol).expect_err("a split vertex is construction state");
    assert_eq!(
        refused.into_findings(),
        vec![ValidationError::JoinableVertexAtRest {
            vertex: split.vertex
        }],
        "check 11's finding at the split vertex, and nothing else"
    );
}

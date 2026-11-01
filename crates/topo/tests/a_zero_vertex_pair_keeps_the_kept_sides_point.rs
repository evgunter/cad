//! **A vertex pair the boolean decides Zero keeps the first-written
//! operand's point** (`[coincidence.booleans]`: which description a
//! glued pair keeps is decided once, at the glue). The output stage
//! puts the two vertices of a vertex-vertex record on one point key
//! (`boolean::ops::share_recorded_points`); the two points may lie apart
//! inside the band, so the key it keeps is the point the pair keeps, and
//! it is operand A's whatever slot each point took in the result's arena.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Point3, Tol};
use topo::{Body, BooleanResult, PointKey};

fn tol() -> Tol {
    Tol::witness()
}

/// The vertices of `body` within `r` of `p`, with their point keys.
fn near(body: &Body<f64>, p: Point3<f64>, r: f64) -> Vec<(PointKey, Point3<f64>)> {
    body.vertex_points()
        .filter(|(_, q)| (*q - p).norm() < r)
        .map(|(v, q)| (body.get_vertex(v).unwrap().point, q))
        .collect()
}

/// **Two cubes kissing a quarter-ε apart, A's point kept.** A is built in
/// a fresh-keyed arena (`fresh_keyed_body`): its corners sit on high
/// point slots over free low ones, which B's grafted corners take. So
/// B's point at the kiss holds the lower key, and the pair's one key
/// keeps A's `(1, 1, 1)`, not B's `(1 + δ)³`. Red if the record class
/// keeps its smallest live key.
#[test]
fn a_zero_corner_kiss_keeps_operand_as_point_over_a_lower_key() {
    let delta = tol().eps() / 4.0;
    let a = common::finished(
        "a",
        common::brick_with_fresh_keys((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let lo = 1.0 + delta;
    let b = common::finished(
        "b",
        common::brick((lo, 2.0), (lo, 2.0), (lo, 2.0), tol()),
        tol(),
    );
    let Ok(BooleanResult::Body(r)) = topo::union(&a, &b, tol()) else {
        panic!("the kiss unites");
    };
    let one = Point3::new(1.0, 1.0, 1.0);
    let at_kiss = near(&r.body, one, 10.0 * delta);
    let [(ka, pa), (kb, pb)] = at_kiss[..] else {
        panic!("two corners at the kiss: {at_kiss:?}");
    };
    assert_eq!(ka, kb, "the kiss's two corners share one key");
    let xyz = |p: Point3<f64>| [p.x, p.y, p.z];
    assert_eq!(
        [xyz(pa), xyz(pb)],
        [[1.0; 3]; 2],
        "the kiss keeps operand A's point"
    );
    // The witness's premise: B's corners took the slots below A's.
    let b_far = near(&r.body, Point3::new(2.0, 2.0, 2.0), 1e-9);
    let [(k_far, _)] = b_far[..] else {
        panic!("one corner of B at (2, 2, 2): {b_far:?}");
    };
    assert!(
        k_far < ka,
        "B's grafted points sit below A's kiss key: {k_far:?} vs {ka:?}"
    );
}

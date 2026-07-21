//! PR 3 adversarial review — target 4: `point_in_loop` (F8's seed)
//! driven adversarially: concave loops, the graze-retry schedule, a
//! constructed RayExhausted witness, and boundary-pre-pass edges.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism;
use geom_core::{Point3, Vec3};
use topo::{LoopContainment, PointInLoopError, point_in_loop};

fn n_z() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

/// Concave (L-shaped) loop: points in the notch are Out even though
/// they sit inside the convex hull; points in both arms are In.
#[test]
fn concave_loop_in_out() {
    let profile = [
        (0.0, 0.0),
        (3.0, 0.0),
        (3.0, 3.0),
        (2.0, 3.0),
        (2.0, 1.0),
        (0.0, 1.0),
    ];
    let fx = prism::<f64>(&profile, 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear().unwrap();
    let q = |x: f64, y: f64| Point3::new(x, y, 1.0);
    let pil = |p| point_in_loop(&fx.body, top.outer, n_z(), p, band).unwrap();
    assert_eq!(pil(q(0.5, 0.5)), LoopContainment::In); // bottom arm
    assert_eq!(pil(q(2.5, 2.0)), LoopContainment::In); // right arm
    assert_eq!(pil(q(1.0, 2.0)), LoopContainment::Out); // the notch
    assert_eq!(pil(q(-1.0, 0.5)), LoopContainment::Out);
    assert_eq!(pil(q(2.0, 2.0)), LoopContainment::OnBoundary); // notch wall
}

/// Graze-retry: the first schedule ray (+x from q) passes EXACTLY
/// through the dart vertex (3, 1) — the side predicate reads Zero and
/// the next schedule member must take over and still answer
/// correctly on both sides of the dart.
#[test]
fn ray_graze_retries_deterministically() {
    let profile = [(0.0, 0.0), (4.0, 0.0), (3.0, 1.0), (4.0, 2.0), (0.0, 2.0)];
    let fx = prism::<f64>(&profile, 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear().unwrap();
    let pil = |x: f64, y: f64| {
        point_in_loop(&fx.body, top.outer, n_z(), Point3::new(x, y, 1.0), band).unwrap()
    };
    // q = (1,1): the +x ray hits (3,1) dead on; retry must say In.
    assert_eq!(pil(1.0, 1.0), LoopContainment::In);
    // q = (3.5,1): right of the dart tip, in the notch: Out (its +x
    // ray ALSO grazes the dart vertex... from behind; retry decides).
    assert_eq!(pil(3.5, 1.0), LoopContainment::Out);
    // Determinism: same answers on replay.
    assert_eq!(pil(1.0, 1.0), LoopContainment::In);
}

/// A constructed RayExhausted: a 15-gon with one vertex ON each
/// usable schedule ray line through the query point (for n = +z the
/// [0,0,1] member projects degenerately and is skipped), every ray
/// grazes and the typed exhaustion error comes back — reachable, not
/// just decorative.
#[test]
fn ray_exhausted_is_reachable() {
    // In-plane directions of the schedule for a +z normal.
    let dirs: [(f64, f64); 15] = [
        (1.0, 0.0),
        (0.0, 1.0),
        (0.5, 0.25),
        (1.0, 0.5),
        (0.25, 1.0),
        (-0.5, 1.0),
        (0.125, -0.5),
        (1.0, 0.125),
        (0.75, -1.0),
        (0.375, 0.75),
        (-1.0, 0.375),
        (0.625, 0.9375),
        (0.3125, -0.625),
        (0.9375, 0.3125),
        (-0.75, -0.25),
    ];
    // One polygon vertex per ray LINE, placed at the direction's angle
    // (radius varied to break symmetry), sorted by angle => a simple
    // star-shaped polygon around the origin.
    let mut corners: Vec<(f64, f64, f64)> = dirs
        .iter()
        .enumerate()
        .map(|(k, &(x, y))| {
            let ang = y.atan2(x);
            let r = 2.0 + 0.03 * k as f64;
            let l = (x * x + y * y).sqrt();
            (ang, r * x / l, r * y / l)
        })
        .collect();
    corners.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let profile: Vec<(f64, f64)> = corners.iter().map(|&(_, x, y)| (x, y)).collect();
    let fx = prism::<f64>(&profile, 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear().unwrap();
    let err =
        point_in_loop(&fx.body, top.outer, n_z(), Point3::new(0.0, 0.0, 1.0), band).unwrap_err();
    assert!(
        matches!(err, PointInLoopError::RayExhausted { .. }),
        "got {err:?}"
    );
    // One step off the degenerate center, the same loop answers.
    let ok = point_in_loop(
        &fx.body,
        top.outer,
        n_z(),
        Point3::new(0.011, 0.017, 1.0),
        band,
    )
    .unwrap();
    assert_eq!(ok, LoopContainment::In);
}

/// Boundary pre-pass: interior of an edge, a corner, and a point one
/// band-width off the boundary (clean In — no boundary absorption).
#[test]
fn boundary_pre_pass_edges() {
    let fx = prism::<f64>(&[(0.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)], 1.0);
    let top = fx.body.get_face(fx.top_face).unwrap();
    let band = geom_core::Band::linear().unwrap();
    let pil = |x: f64, y: f64| {
        point_in_loop(&fx.body, top.outer, n_z(), Point3::new(x, y, 1.0), band).unwrap()
    };
    assert_eq!(pil(1.0, 0.0), LoopContainment::OnBoundary);
    assert_eq!(pil(2.0, 2.0), LoopContainment::OnBoundary);
    // 100·K·ε off the wall: decisively In/Out at every ε row (the
    // pre-pass must not swallow a clean margin).
    let off = 1000.0 * geom_core::Tolerance::get().eps;
    assert_eq!(pil(1.0, off), LoopContainment::In);
    assert_eq!(pil(1.0, -off), LoopContainment::Out);
}

//! Review timing probe for PR 7497 (base vs head). Not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Tol, Vec3};
use std::time::Instant;
use topo::AtRestPolicy;

fn time<R>(what: &str, n: usize, mut f: impl FnMut() -> R) {
    let _ = f();
    let t = Instant::now();
    for _ in 0..n {
        std::hint::black_box(f());
    }
    println!(
        "[timing] {what}: {:.3} ms/run",
        t.elapsed().as_secs_f64() * 1e3 / n as f64
    );
}

#[test]
#[ignore = "review timing probe"]
fn review_7497_timing() {
    let t = Tol::witness();
    let torus_v = || vec![(Point2::new(2.0, -0.5), 1.0), (Point2::new(2.0, 0.5), 1.0)];
    let cyl_v = || {
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 2.0), 0.0),
            (Point2::new(0.0, 2.0), 0.0),
        ]
    };
    time("revolve door torus", 20, || {
        sweep::test_support::revolved_about_y(torus_v(), sweep::Revolution::Full, t)
    });
    time("revolve door cylinder", 20, || {
        sweep::test_support::revolved_about_y(cyl_v(), sweep::Revolution::Full, t)
    });
    let torus = sweep::test_support::revolved_about_y(torus_v(), sweep::Revolution::Full, t);
    time("f64::finish torus (rows complete)", 20, || {
        f64::finish(torus.clone(), t).is_ok()
    });
    let loft = crate::common::approx::twisted_loft(0.3);
    time("f64::finish twisted loft (rows complete)", 10, || {
        f64::finish(loft.clone(), t).is_ok()
    });
    let sphere = sweep::test_support::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), t);
    time("f64::finish sphere (rows complete)", 20, || {
        f64::finish(sphere.clone(), t).is_ok()
    });
    let cyl = topo::test_support::finished(
        "cyl",
        sweep::test_support::revolved_about_y(cyl_v(), sweep::Revolution::Full, t),
        t,
    );
    let block = topo::test_support::finished(
        "block",
        topo::transform_rigid(
            &sweep::test_support::cube::<f64>(1.0, t),
            &geom_core::RigidMotion::translation(Vec3::new(0.5, 0.5, 0.3)).unwrap(),
        )
        .unwrap(),
        t,
    );
    time("union cylinder+cube", 5, || {
        topo::union(&cyl, &block, t).is_ok()
    });
    time("subtract cylinder-cube", 5, || {
        topo::subtract(&cyl, &block, t).is_ok()
    });
    time("shell cylinder 0.1", 5, || {
        topo::shell(&cyl, 0.1, t).is_ok()
    });
    time("loft door (twisted)", 5, || {
        crate::common::approx::twisted_loft(0.3)
    });
}

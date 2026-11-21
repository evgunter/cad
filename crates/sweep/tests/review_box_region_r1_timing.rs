//! Reviewer r1's timing of the session box's ask alone (PR 7225), base
//! against head: `topo::session_box_at` under the identity, and the
//! finished-body gate, on the PR's four f64 bodies. Compiles on main.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Point2, RigidMotion, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;

#[test]
#[ignore = "a timing, run by hand"]
fn r1_time_the_box_ask() {
    let tol = Tol::witness();
    let bodies: Vec<(&str, topo::Body<f64>)> = vec![
        ("brick", sweep::test_support::brick::<f64>((0.0, 1.0), (0.0, 2.0), (0.0, 0.5), tol)),
        ("ball", sweep::test_support::ball_poled_y::<f64>(50.0, Vec3::new(0.0, 0.0, 0.0), tol)),
        (
            "torus",
            revolved_about_y(
                vec![(Point2::new(450.0, -400.0), 1.0), (Point2::new(450.0, 400.0), 1.0)],
                Revolution::Full,
                tol,
            ),
        ),
        ("loft", sweep::test_support::loft_prism(tol)),
    ];
    let id = RigidMotion::translation(Vec3::new(0.0, 0.0, 0.0)).unwrap();
    for (what, body) in &bodies {
        let n = 200;
        let t = std::time::Instant::now();
        for _ in 0..n {
            let _ = topo::session_box_at(body, &id, tol);
        }
        let ask = t.elapsed().as_secs_f64() / f64::from(n) * 1e6;
        let m = 20;
        let t = std::time::Instant::now();
        for _ in 0..m {
            let _ = topo::validate_geometric(body, tol);
        }
        let gate = t.elapsed().as_secs_f64() / f64::from(m) * 1e3;
        println!("TIMING {what}: ask {ask:.1} us, gate {gate:.2} ms");
    }
}

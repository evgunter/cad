//! Reviewer probe (PR 7146): corner cuts of a plate with one arc side,
//! through `topo::split`. Prints each outcome.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]
use geom_core::{Point2, Point3, Tol, Vec3};

fn main() {
    let tol = Tol::witness();
    let eps = tol.eps();
    for r in [1.0e3, 1.0e4] {
        for sign in [1.0, -1.0] {
            let theta = 2.0 * (5.0 / r as f64).asin();
            let bulge = sign * (theta / 4.0).tan();
            let verts = vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(10.0, 0.0), 0.0),
                (Point2::new(10.0, 1.0), bulge),
                (Point2::new(0.0, 1.0), 0.0),
            ];
            let body = sweep::test_support::prism::<f64>(verts, 1.0, tol);
            let fin = sweep::test_support::finished("plate", body, tol);
            for n in [(1.0, 1.0, 1.0), (0.05, 1.0, 1.0), (0.2, 1.0, 0.7), (1.0, 0.3, 1.0)] {
                let nv = Vec3::new(n.0, n.1, n.2);
                let nn = nv / nv.norm();
                for k in [2.0, 5.0, 8.0, 12.0, 20.0, 50.0] {
                    let d = k * eps;
                    let at = Point3::new(10.0, 1.0, 1.0) - nn * d;
                    let plane = topo::test_support::split_plane(at, nn, tol);
                    let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| topo::split(&fin, &plane, tol)));
                    let s = match got {
                        Ok(Ok(_)) => "Ok".to_string(),
                        Ok(Err(e)) => format!("{e:?}"),
                        Err(_) => "PANIC".to_string(),
                    };
                    let s: String = s.chars().take(260).collect();
                    println!("CUT eps={eps:e} R={r:e} sign={sign} n={n:?} k={k} -> {s}");
                }
            }
        }
    }
}

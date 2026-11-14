//! Reviewer evidence scan (asserts nothing; `#[ignore]`d): the tilted
//! hollow tube over a ladder of majors, on two frames and with an arc
//! window, printing what each door answers at the run's ε.
//!
//! `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --test all \
//!  -E 'test(edge_turns_review_probe)' --run-ignored all --no-capture`
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point3, Tol, Vec3};
use sweep::test_support::tube_frame;
use sweep::{TubeWindow, tube_along_arc_hollow};

#[test]
#[ignore = "reviewer evidence scan; asserts nothing"]
fn scan() {
    let tol = Tol::witness();
    let (s3, s2) = (3.0_f64.sqrt().recip(), 2.0_f64.sqrt().recip());
    let (s14, s5) = (14.0_f64.sqrt().recip(), 5.0_f64.sqrt().recip());
    let frames = [
        (
            "origin-111",
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(s3, s3, s3),
            Vec3::new(s2, -s2, 0.0),
        ),
        (
            "off-2-13",
            Point3::new(30.0, -110.0, 200.0),
            Vec3::new(2.0 * s14, -s14, 3.0 * s14),
            Vec3::new(s5, 2.0 * s5, 0.0),
        ),
    ];
    let (outer, wall) = (2.5, 0.001);
    let inner: f64 = outer - wall;
    for (name, o, axis, x) in frames {
        for major in [100.0, 300.0, 600.0, 750.0, 800.0, 990.0, 1100.0] {
            for window in [None, Some((0.3, 2.9))] {
                let w = match window {
                    None => TubeWindow::Full,
                    Some((t0, t1)) => TubeWindow::Arc { t0, t1 },
                };
                let got = tube_along_arc_hollow::<f64>(
                    tube_frame(o, axis, x, tol),
                    major,
                    w,
                    outer,
                    wall,
                    tol,
                );
                let line = match got {
                    Ok(t) => {
                        let v = topo::test_support::pinned_volume("the tube", &t.body, tol).volume;
                        let turn = window.map_or(2.0 * PI, |(a, b)| b - a);
                        let want = turn * PI * major * (outer * outer - inner * inner);
                        format!("builds, rel err {:e}", (v - want).abs() / want)
                    }
                    Err(e) => {
                        let s = format!("{e:?}");
                        format!("REFUSES {}", &s[..s.len().min(420)])
                    }
                };
                eprintln!("REVIEW-SCAN {name} major {major} window {window:?}: {line}");
            }
        }
    }
}

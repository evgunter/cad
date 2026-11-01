//! Delta review of PR 5956's fix pass: a needle cone's vertex near its
//! apex but off it reads as main reads it. Self-contained (public API
//! only) so the same file runs on main for a differential: it prints one
//! `NEEDLE` line per pose, which the reviewer diffs between the trees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common::needle;
use geom_core::{Point3, Tol, Vec3};

fn head(s: String) -> String {
    let cut = s.find([' ', '{', '(']).map_or(s.len(), |i| i);
    let inner = s[cut..].trim_start_matches(['(', ' ']);
    let cut2 = inner.find([' ', '{', '(']).map_or(inner.len(), |i| i);
    format!("{} {}", &s[..cut], &inner[..cut2])
}

fn vol(b: Option<&topo::AtRestBody<f64>>, tol: Tol) -> String {
    match b {
        None => "-".into(),
        Some(b) => match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            topo::test_support::pinned_volume("needle side", b, tol).volume
        })) {
            Ok(v) => format!("{v:.6e}"),
            Err(_) => "VOLUME-PANIC".into(),
        },
    }
}

#[test]
#[ignore = "delta-review differential: run on main and on the head, diff the NEEDLE lines"]
fn delta_needle_cone_near_apex_reads_as_main() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let apex = Point3::new(0.0, 1.0, 0.0);
    for deg in [1.0f64, 2.0, 3.0] {
        let a = deg.to_radians();
        for window in [None, Some(4.0)] {
            let body = needle(deg, window, tol);
            // The seam ruling (azimuth 0) runs from the apex to (tan α, 0, 0).
            let g = Vec3::new(a.sin(), -a.cos(), 0.0);
            for k in [0.3f64, 2.0, 10.0, 0.5 / a.sin(), 3.0 / a.sin(), 1e3] {
                let foot = apex + g * (k * eps);
                for (tilt, spin) in [(0.4f64, 0.7f64), (1.2, 2.0), (0.05, 4.0), (1.5, 0.0)] {
                    let n = Vec3::new(tilt.sin() * spin.cos(), tilt.cos(), tilt.sin() * spin.sin());
                    let plane = topo::test_support::split_plane(foot, n, tol);
                    let got = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        topo::splitting::split(&body, &plane, tol)
                    })) {
                        Err(_) => "PANIC".to_string(),
                        Ok(Ok(r)) => format!(
                            "Ok above {} below {}",
                            vol(r.above.body(), tol),
                            vol(r.below.body(), tol)
                        ),
                        Ok(Err(e)) => head(format!("{e:?}")),
                    };
                    eprintln!(
                        "NEEDLE split ε {eps:e} α {deg}° w {window:?} k {k:.3} tilt {tilt} spin {spin}: {got}"
                    );
                }
                // A box whose top face passes through the same foot,
                // square to the axis: the Boolean's sector walk reads the
                // crossing vertex near the axis.
                let y = foot.y;
                let bx = sweep::test_support::finished(
                    "box",
                    sweep::test_support::brick((-2.0, 2.0), (y - 3.0, y), (-2.0, 2.0), tol),
                    tol,
                );
                for (op, name) in [(0, "∩"), (1, "∖")] {
                    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        if op == 0 {
                            topo::intersect(&body, &bx, tol)
                        } else {
                            topo::subtract(&body, &bx, tol)
                        }
                    }));
                    let got = match res {
                        Err(_) => "PANIC".to_string(),
                        Ok(Ok(r)) => format!("Ok {}", vol(r.body().map(|b| &b.body), tol)),
                        Ok(Err(e)) => head(format!("{e:?}")),
                    };
                    eprintln!("NEEDLE bool{name} ε {eps:e} α {deg}° w {window:?} k {k:.3}: {got}");
                }
            }
        }
    }
}

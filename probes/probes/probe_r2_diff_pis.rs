//! R2 review probe (PR #4135): cross-tree differential of
//! `topo::point_in_solid` against cone bodies (the ray lane's two cone
//! arms now read the shared `line_cone_quadratic`). The same file runs on
//! the merge base and on the head; each writes `R2_DIFF_PIS`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stdout)]

use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};
use std::fmt::Write as _;

fn frustum(pts: &[(f64, f64)], rev: Revolution<f64>, pose: Affine3<f64>) -> topo::Body<f64> {
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    let b = revolve(&validated(vec![lp]), axis_y(), rev, Tol::witness()).unwrap().body;
    topo::transform_rigid(&b, &pose, Tol::witness()).unwrap()
}

#[test]
fn probe_r2_diff_pis() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut out = String::new();
    let mut s: u64 = 0x4242_0000_1111_2222;
    let mut f = move || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    for &sc in &[1e-3, 1.0, 1e3] {
        let shapes: Vec<(&str, Vec<(f64, f64)>, Revolution<f64>)> = vec![
            ("widening", vec![(0.0, 0.0), (0.5 * sc, 0.0), (sc, sc), (0.0, sc)], Revolution::Full),
            ("narrowing", vec![(0.0, 0.0), (sc, 0.0), (0.5 * sc, sc), (0.0, sc)], Revolution::Full),
            ("full", vec![(0.0, 0.0), (sc, 0.0), (0.0, sc)], Revolution::Full),
            ("half-widening", vec![(0.0, 0.0), (0.5 * sc, 0.0), (sc, sc), (0.0, sc)], Revolution::Partial(3.0)),
            ("off-axis-ring", vec![(0.2 * sc, 0.0), (0.6 * sc, 0.0), (0.9 * sc, sc), (0.3 * sc, sc)], Revolution::Full),
        ];
        for (name, pts, rev) in shapes {
            for j in 0..3 {
                let pose = if j == 0 {
                    Affine3::identity()
                } else {
                    Affine3::translation(Vec3::new(f() - 0.5, f() - 0.5, f() - 0.5) * sc)
                        * Affine3::rotation_about_axis(
                            Point3::origin(),
                            Vec3::new(f() - 0.5, f() - 0.5, f() - 0.5).normalize(),
                            f() * 3.0,
                        )
                };
                let Ok(body) = std::panic::catch_unwind(|| frustum(&pts, rev, pose)) else {
                    writeln!(out, "{name} s{sc} j{j}: no body").unwrap();
                    continue;
                };
                for k in 0..150 {
                    let local = if k % 3 == 0 {
                        // Near the lateral wall: on a generator, nudged.
                        let y = f() * sc;
                        let (x0, y0, x1, y1) = (pts[1].0, pts[1].1, pts[2].0, pts[2].1);
                        let t = y / sc;
                        let r = x0 + (x1 - x0) * t;
                        let phi = f() * 6.283;
                        let nudge = (f() - 0.5) * 20.0 * tol.eps();
                        let _ = (y0, y1);
                        Point3::new((r + nudge) * phi.cos(), y, (r + nudge) * phi.sin())
                    } else {
                        Point3::new((f() - 0.5) * 2.4 * sc, (f() - 0.2) * 1.4 * sc, (f() - 0.5) * 2.4 * sc)
                    };
                    let q = pose.transform_point(local);
                    let got = topo::point_in_solid(&body, q, band, tol);
                    writeln!(out, "{name} s{sc} j{j} k{k}: {got:?}").unwrap();
                }
            }
        }
    }
    std::fs::write(std::env::var("R2_DIFF_PIS").unwrap(), out).unwrap();
}

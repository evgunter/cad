//! Reviewer probe (reach-dual4135-r1): the ray lane against cones, run
//! unchanged on the merge base and on the frozen head; the printed digest
//! of every `point_in_solid` answer (escalations included) must agree.
//! Mounted temporarily into `crates/sweep/tests/all.rs` on both trees.
#![allow(clippy::unwrap_used, clippy::panic)]
use crate::revolve_common::{axis_y, validated};
use geom_core::{Affine3, Band, Point2, Point3, Tol, Vec3};
use profile::{ProfileLoop, RawLoop};
use sweep::{Revolution, revolve};

fn cone_body(y0: f64, y1: f64, r0: f64, k: f64) -> topo::Body<f64> {
    let r1 = r0 + k * (y1 - y0);
    let mut pts = vec![(0.0, y0), (r0, y0)];
    if r1 > 1e-15 {
        pts.push((r1, y1));
    }
    pts.push((0.0, y1));
    let lp = ProfileLoop::polygon(pts.iter().map(|&(x, y)| Point2::new(x, y)));
    revolve(&validated(vec![lp]), axis_y(), Revolution::Full, Tol::witness()).unwrap().body
}

#[test]
fn r1_ray_lane_digest() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let mut digest: u64 = 0xcbf2_9ce4_8422_2325;
    let mut counts = std::collections::BTreeMap::new();
    let mut n = 0usize;
    for s in [1e-2, 1.0, 1e2] {
        for (y0, y1, r0, k) in [(0.0, 1.0, 0.5, 0.5), (0.0, 1.0, 1.0, -0.5), (0.0, 1.0, 1.0, -1.0), (0.0, 1.0, 0.3, 0.02), (0.0, 0.04, 0.1, 20.0)] {
            let body = cone_body(y0 * s, y1 * s, r0 * s, k);
            for pose in [Affine3::identity(), Affine3::translation(Vec3::new(0.3 * s, -0.2 * s, 0.7 * s)) * Affine3::rotation_about_axis(Point3::origin(), Vec3::new(0.2, 0.9, 0.4).normalize(), 0.8)] {
                let b = topo::transform_rigid(&body, &pose, tol).unwrap();
                let span = (r0.abs() + (k * (y1 - y0)).abs() + (y1 - y0)) * s;
                let m = 14;
                for i in 0..m {
                    for j in 0..m {
                        for l in 0..m {
                            let c = |v: usize| (v as f64 + 0.37) / m as f64 * 2.4 - 1.2;
                            let p = Point3::new(c(i) * span, y0 * s + (c(j) + 1.2) / 2.4 * (y1 - y0) * s * 1.4 - 0.2 * (y1 - y0) * s, c(l) * span);
                            // and a point pulled onto the wall: radius r(y)
                            let rho = p.x.hypot(p.z);
                            let ry = r0 * s + k * (p.y - y0 * s);
                            let onwall = if rho > 0.0 && ry > 0.0 { Point3::new(p.x * ry / rho * (1.0 + 1e-7), p.y, p.z * ry / rho * (1.0 + 1e-7)) } else { p };
                            for q in [p, onwall] {
                                let got = topo::point_in_solid(&b, pose.transform_point(q), band, tol);
                                let txt = format!("{got:?}");
                                let key: String = txt.chars().take(28).collect();
                                *counts.entry(key).or_insert(0usize) += 1;
                                for byte in txt.bytes() {
                                    digest ^= u64::from(byte);
                                    digest = digest.wrapping_mul(0x100_0000_01b3);
                                }
                                n += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    eprintln!("R1RAY eps={:e} n={n} digest={digest:016x} counts={counts:?}", tol.eps());
}

//! **What one skew or unequal-radius cylinder boolean costs.** The unit
//! cylinder `z ∈ [−2, 2]` against the fenced row's two partners
//! (`verbs_germarms2::the_fenced_poses_keep_their_own_outcomes`): one
//! `topo::union`/`intersect`/`subtract` each, wall time and volume.
//!
//! `cargo run -p sweep --release --example cylinder_pair_cost_probe [pose] [op]`
//! (`pose` ∈ `unequal`, `skew`; `op` ∈ `union`, `intersect`, `subtract`;
//! either omitted runs every one).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use std::time::Instant;

use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::Body;

fn cyl(r: f64, h: f64) -> Body<f64> {
    let tol = Tol::witness();
    let lp = profile::circle(Point2::new(0.0, 0.0), r, tol).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, -h));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    extrude(
        &profile,
        Extrusion::Distance {
            depth: 2.0 * h,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
    .into_body()
}

fn moved(b: &Body<f64>, m: &Affine3<f64>) -> Body<f64> {
    let tol = Tol::witness();
    topo::transform_rigid(b, &geom_core::test_support::rigid(m, tol), tol).unwrap()
}

fn main() {
    let tol = Tol::witness();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let quarter = Affine3::rotation_about_axis(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        PI / 2.0,
    );
    let a = topo::finish(cyl(1.0, 2.0), tol).unwrap();
    let unequal = moved(&cyl(0.6, 2.0), &quarter);
    let skew = moved(
        &moved(&cyl(1.0, 2.0), &quarter),
        &Affine3::translation(Vec3::new(0.35, 0.0, 0.0)),
    );
    for (pose, b) in [("unequal", unequal), ("skew", skew)] {
        if args.first().is_some_and(|p| p != pose) {
            continue;
        }
        let b = topo::finish(b, tol).unwrap();
        for op in ["union", "intersect", "subtract"] {
            if args.get(1).is_some_and(|o| o != op) {
                continue;
            }
            let from = Instant::now();
            let r = match op {
                "union" => topo::union(&a, &b, tol),
                "intersect" => topo::intersect(&a, &b, tol),
                _ => topo::subtract(&a, &b, tol),
            };
            let t = from.elapsed();
            let what = match &r {
                Ok(topo::BooleanResult::Body(bb)) => {
                    let from = Instant::now();
                    let v = topo::test_support::pinned_volume("probe", &bb.body, tol).volume;
                    format!(
                        "body, volume {v:.12} (read in {:.3} s)",
                        from.elapsed().as_secs_f64()
                    )
                }
                other => format!("{other:?}").chars().take(300).collect(),
            };
            println!("{pose}\t{op}\t{:.3} s\t{what}", t.as_secs_f64());
        }
    }
}

//! Review probe for PR #3853: the germ-normal and cylinder-axis arms on
//! real poses (pipe bores plate), including far-origin plates and poses
//! that put a germ site on the plate's plane origin. Prints only.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::k_stats::{self, Probe};
use geom_core::{Affine3, Point2, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::{Extrusion, extrude};

fn run(label: &str, origin: (f64, f64), centre: (f64, f64), r: f64, scale: f64) {
    let tol = Tol::witness();
    let s = |v: f64| Probe(v * scale);
    let at = |x: f64, y: f64, z: f64| {
        SketchPlane::new(Affine3::translation(Vec3::new(s(x), s(y), s(z))))
    };
    // Pipe sketched in WORLD position (its own plane at the origin).
    let Ok(c) = profile::circle(Point2::new(s(centre.0), s(centre.1)), s(r), tol) else {
        println!("[{label}] profile refuses the circle");
        return;
    };
    let Ok(pipe) = Profile::new(at(0.0, 0.0, -2.0), vec![c.into()]).validate(tol) else {
        println!("[{label}] profile validation refuses");
        return;
    };
    let Ok(pipe) = extrude(&pipe, Extrusion::Distance(s(4.0)), tol) else {
        println!("[{label}] extrude refuses");
        return;
    };
    let pipe = pipe.body;
    // Plate sketched on a plane translated to `origin`, corners local so
    // the plate sits at world [-2,2]^2.
    let corners = [(-2.0, -2.0), (2.0, -2.0), (2.0, 2.0), (-2.0, 2.0)]
        .map(|(x, y)| (Point2::new(s(x - origin.0), s(y - origin.1)), Probe(0.0)));
    let plate = Profile::new(
        at(origin.0, origin.1, -0.5),
        vec![profile::test_support::bulge_loop(corners.to_vec())],
    )
    .validate(tol)
    .unwrap();
    let plate = extrude(&plate, Extrusion::Distance(s(1.0)), tol).unwrap().body;
    for (op, name) in [(0, "subtract"), (1, "union"), (2, "intersect")] {
        k_stats::start_recording();
        let res = match op {
            0 => topo::subtract(&plate, &pipe, tol),
            1 => topo::union(&plate, &pipe, tol),
            _ => topo::intersect(&plate, &pipe, tol),
        };
        let samples = k_stats::take_samples();
        let pick = |n: &str| -> Vec<f64> {
            let mut v: Vec<f64> = samples.iter().filter(|x| x.predicate == n).map(|x| x.margin).collect();
            v.sort_by(f64::total_cmp);
            v.dedup();
            v
        };
        let out = match &res {
            Ok(topo::BooleanResult::Body(b)) => format!("Ok faces={}", b.body.faces().count()),
            Ok(_) => "Ok empty".to_string(),
            Err(e) => format!("Err {e}"),
        };
        println!(
            "[{label}] {name} scale={scale:e} -> {out}\n    germ={:?}\n    cyl={:?}",
            pick("bool_germ_plane_normal"),
            pick("bool_box_cylinder_axis")
        );
    }
}

#[test]
fn arm_poses() {
    for scale in [1e-3, 1.0] {
        run("plain", (0.0, 0.0), (0.0, 0.0), 1.0, scale);
        run("seam-on-origin", (0.0, 0.0), (-1.0, 0.0), 1.0, scale);
        run("seam-on-origin-L", (0.0, 0.0), (-0.5, 0.5), 0.5, scale);
        run("far-origin-1e3", (1e3, 1e3), (0.0, 0.0), 1.0, scale);
        run("far-origin-1e6", (1e6, -1e6), (0.5, 0.5), 0.5, scale);
        run("tiny-pipe", (0.0, 0.0), (1.0, 1.0), 1e-3, scale);
    }
    for r in [5e-11, 2e-9, 5e-9, 6e-9, 7e-9, 8e-9, 9e-9, 1e-8, 1.1e-8, 1.3e-8, 1.6e-8, 2e-8, 1e-7] {
        run(&format!("sub-band-pipe r={r:e}"), (0.0, 0.0), (1.0, 1.0), r, 1.0);
    }
}

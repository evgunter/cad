//! Design probe (branch `flush/needle-design-y`, not for merge): the
//! interval enclosure of a band-scale torus's volume. Run with
//! `--ignored --nocapture` under `CAD_TOLERANCE_EPS=1e-12`.
#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Point2, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};

use super::shell7_common::tol;

#[test]
#[ignore = "design probe"]
fn needle_design_y_torus_enclosure() {
    let e = tol().eps();
    println!("eps={e:e} K={}", tol().k());
    for (major, minor) in [(1.0e-10, 2.0e-11), (2.0e-9, 5.0e-10), (2.0e-8, 5.0e-9)] {
        let a = Point2::new(major + minor, 0.0);
        let b = Point2::new(major - minor, 0.0);
        let lp = bulge_loop(vec![(a, 1.0), (b, 1.0)]);
        let Ok(profile) = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol()) else {
            println!("R={major:e} r={minor:e}: the profile refuses");
            continue;
        };
        let axis = RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        };
        let Ok(rev) = revolve(&profile, axis, Revolution::Full, tol()) else {
            println!("R={major:e} r={minor:e}: the revolve door refuses");
            continue;
        };
        let closed = 2.0 * core::f64::consts::PI.powi(2) * major * minor * minor;
        match topo::volume_enclosure_probe(&rev.body, tol()) {
            Ok((v, area, lo, hi, recentred)) => println!(
                "torus R={major:e} r={minor:e}: V={v:.4e} (closed form {closed:.4e}) A={area:.3e} V/A={:.4}eps enclosure=[{lo:.6e}, {hi:.6e}] width={:.3e} width/|V|={:.2e} recentred={recentred} straddles={}",
                v / area / e,
                hi - lo,
                (hi - lo) / v.abs(),
                lo <= 0.0 && hi >= 0.0
            ),
            Err(err) => println!("torus R={major:e} r={minor:e}: props refuse {err:?}"),
        }
        match topo::AtRestBody::validate(rev.body.clone(), tol()) {
            Ok(_) => println!("    at rest: ok"),
            Err(errs) => println!("    at rest: {}", format!("{errs:?}").chars().take(400).collect::<String>()),
        }
    }
}

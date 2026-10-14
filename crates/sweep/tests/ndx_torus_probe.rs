//! NEEDLE-DESIGN-X probe (not a pin): a revolve-door torus near the band,
//! both orientations, at rest.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Vec2};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Revolution, RevolveAxis, revolve};

#[test]
#[ignore = "design probe"]
fn ndx_small_torus() {
    let tol = geom_core::Tol::witness();
    let s = tol.eps() / 1e-12;
    for (major, minor) in [(1.0e-10 * s, 2.0e-11 * s), (2.0e-10 * s, 4.0e-11 * s)] {
        let a = Point2::new(major + minor, 0.0);
        let b = Point2::new(major - minor, 0.0);
        let lp = bulge_loop(vec![(a, 1.0), (b, 1.0)]);
        let Ok(profile) = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol) else {
            eprintln!("NDXT R={major:e}: profile refuses");
            continue;
        };
        let Ok(rev) = revolve(
            &profile,
            RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) },
            Revolution::Full,
            tol,
        ) else {
            eprintln!("NDXT R={major:e}: revolve refuses");
            continue;
        };
        let solo = topo::AtRestBody::validate(rev.body.clone(), tol).map(|_| ());
        let inv = topo::AtRestBody::validate(rev.body.revert(), tol).map(|_| ());
        let f = |r: &Result<(), Vec<topo::ValidationError>>| format!("{r:?}").chars().take(300).collect::<String>();
        eprintln!("NDXT eps={:e} R={major:e} r={minor:e}\n   solo={}\n   inv={}", tol.eps(), f(&solo), f(&inv));
    }
}

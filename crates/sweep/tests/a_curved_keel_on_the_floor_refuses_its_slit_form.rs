//! **A curved keel resting on the floor refuses its slit form, as a
//! construction gap.** A 270° partial revolve about `y` of the slab
//! `0.5..3 × 1..2`, less a 150° partial revolve of a V whose tip lies on
//! its floor `y = 1`, turned about `y`. The keel is a circular arc of
//! radius 2 lying in the floor, and the material about it is two wedges,
//! floor | wall a and wall b | floor (`crates/topo/README.md`, tier 3),
//! so the floor would carry it as two coincident arcs: turned 60°, the V
//! lies inside the slab's 270° and the arcs are a zero-area ring of the
//! floor; turned −20°, the keel leaves through the slab's end face and
//! they are a slit of its outline.
//!
//! Built, either form passes tiers 1, 2 and 3 at the closed-form volume,
//! and tier 3′ refuses the two arcs, on two curve entries, as an
//! `EdgeEdgeOverlap` no record backs, as it does the split's curved ring
//! (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
//! So the Boolean refuses `EdgeInFace` (`Curved`), as the split refuses
//! `CurvedSlitTrace`, rather than ship it or the one arc the floor
//! passes through.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{BooleanError, EdgeInFaceGap, subtract, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

/// **Each turn refuses `EdgeInFace` (`Curved`)**, naming an arc of
/// radius 2 on the floor `y = 1`.
#[test]
fn a_curved_keel_on_the_floor_refuses_its_slit_form() {
    let loop_of = |pts: &[(f64, f64)]| pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let slab = finished(
        "the slab",
        revolved_about_y(
            loop_of(&[(0.5, 1.0), (3.0, 1.0), (3.0, 2.0), (0.5, 2.0)]),
            Revolution::Partial(1.5 * PI),
            tol(),
        ),
        tol(),
    );
    for turn in [60.0, -20.0] {
        let what = format!("the V turned {turn}°");
        let v = revolved_about_y(
            loop_of(&[(1.7, 2.5), (2.0, 1.0), (2.3, 2.5)]),
            Revolution::Partial(150f64.to_radians()),
            tol(),
        );
        let turned =
            Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), f64::to_radians(turn));
        let v =
            transform_rigid(&v, &geom_core::test_support::rigid(&turned, tol()), tol()).unwrap();
        match subtract(&slab, &finished("the V", v, tol()), tol()) {
            Err(BooleanError::EdgeInFace {
                gap: EdgeInFaceGap::Curved,
                ..
            }) => {}
            other => panic!(
                "{what}: {:?}, not the curved keel's refusal",
                other.map(drop)
            ),
        }
    }
}

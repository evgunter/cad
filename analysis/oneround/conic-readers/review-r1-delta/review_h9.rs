//! Review probe (PR 7510 delta, reviewer r1), H9: how far off square to
//! its axis a stored seam can stand, by mint, and what size puts a
//! stored point a default ε (1e-9 m) off its extent.
#![allow(clippy::unwrap_used, clippy::print_stdout)]

use core::f64::consts::FRAC_PI_2;

use geom_core::{Band, Point3, Vec3};
use test_utils::fuzz::Rng;

use super::unit;
use crate::{Conic, Curve3, Ellipse, Frame};

#[test]
fn review_h9_seam_squareness_by_mint() {
    let mut rng = Rng::from_seed(0x5eed_c0de_0031);
    // 1. `Frame::new` (every `Curve3::circle`, `Ellipse::new` over it,
    // `Ellipse::anchored`) against the angle of the handed seam off the axis.
    for theta in [1.0_f64, 1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8] {
        let mut worst: f64 = 0.0;
        for _ in 0..4000 {
            let n = unit(&mut rng);
            let side = n.cross(unit(&mut rng)).normalize();
            let toward = n * theta.cos() + side * theta.sin();
            let f = Frame::new(Point3::origin(), n, toward);
            worst = worst.max(f.u_ref().dot(f.axis()).abs());
        }
        println!("H9 Frame::new, seam handed {theta:e} rad off the axis: worst |u_ref.axis| {worst:e}; a stored point stands 1e-9 m off at a = {:e} m", 1e-9 / worst);
    }
    // 1b. The STEP placement's mint (`AXIS2_PLACEMENT_3D`, ISO 10303-42
    // `build_axes`), as `step-import` calls it.
    for theta in [1e-2_f64, 1e-4, 1e-6, 1e-7, 1e-8] {
        let (mut worst, mut refused) = (0.0_f64, 0);
        for _ in 0..4000 {
            let n = unit(&mut rng);
            let side = n.cross(unit(&mut rng)).normalize();
            let reference = n * theta.cos() + side * theta.sin();
            match geom_core::RigidMotion::from_axis_and_reference(Point3::origin(), n, reference, "step_placement_axes", Band::least()) {
                Ok(m) => {
                    let f = Frame::of_motion(&m);
                    worst = worst.max(f.u_ref().dot(f.axis()).abs());
                }
                Err(_) => refused += 1,
            }
        }
        println!("H9 STEP build_axes, reference {theta:e} rad off the axis: refused {refused} of 4000, worst |u_ref.axis| {worst:e}; 1e-9 m off at a = {:e} m", 1e-9 / worst);
    }
    // 2. The deciding doors: do they refuse a seam near the axis?
    for theta in [1e-3_f64, 1e-5, 1e-7] {
        let n = Vec3::new(1.0, 2.0, 2.0) / 3.0;
        let side = Vec3::new(2.0, 1.0, -2.0) / 3.0;
        let toward = n * theta.cos() + side * theta.sin();
        let e = Curve3::ellipse_anchored(Point3::origin(), n, toward, (2.0, 1.0), Band::least());
        let skew = e.as_ref().ok().and_then(|c| c.conic().map(|k| k.u_ref().dot(k.axis()).abs()));
        println!("H9 Curve3::ellipse_anchored, seam {theta:e} rad off the axis: {} skew {skew:?}", if e.is_ok() { "ACCEPTED" } else { "refused" });
    }
    // 3. Re-seams of a square frame: one, and a thousand quarter turns.
    let mut worst = [0.0_f64; 3];
    for _ in 0..2000 {
        let n = unit(&mut rng);
        let u = n.cross(unit(&mut rng)).normalize();
        let e0 = Ellipse::anchored(Point3::new(1.0, 2.0, 3.0), n, u, 2.0, 1.0);
        worst[0] = worst[0].max(e0.u_ref().dot(e0.axis()).abs());
        let e1 = e0.seamed_near(FRAC_PI_2);
        worst[1] = worst[1].max(e1.u_ref().dot(e1.axis()).abs());
        let mut e = e0;
        for _ in 0..1000 {
            e = e.seamed_near(FRAC_PI_2);
        }
        worst[2] = worst[2].max(e.u_ref().dot(e.axis()).abs());
    }
    println!("H9 re-seam: as minted {:e}, one quarter turn {:e}, a thousand {:e}", worst[0], worst[1], worst[2]);
}

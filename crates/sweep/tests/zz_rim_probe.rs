#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_core::{Affine3, Arc2, Point2, Point3, Tol, Vec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::AtRestBody;

fn tol() -> Tol { Tol::witness() }
fn fin(w: &str, b: topo::Body<f64>) -> AtRestBody<f64> { topo::test_support::finished(w, b, tol()) }

/// ∫ over the disc |p - c| ≤ r of 2√(R² − |p|²).
fn chord_volume(c: (f64, f64), r: f64, big: f64) -> f64 {
    // Gauss-Legendre in ρ (20 nodes via Golub-Welsch-free table substitute: composite Simpson fine), trapezoid in θ.
    let (nr, nt) = (4000usize, 256usize);
    let mut s = 0.0;
    for i in 0..nr {
        let rho = (i as f64 + 0.5) * r / nr as f64;
        let mut ring = 0.0;
        for j in 0..nt {
            let t = TAU * j as f64 / nt as f64;
            let (x, y) = (c.0 + rho * t.cos(), c.1 + rho * t.sin());
            ring += 2.0 * (big * big - x * x - y * y).sqrt();
        }
        s += ring * TAU / nt as f64 * rho * r / nr as f64;
    }
    s
}

#[test]
fn probe_tilted_tube_through_ball() {
    let big = 1.5;
    let ball = fin("ball", sweep::test_support::ball_poled_z(big, Vec3::zero(), tol()));
    let (cx, cz, r) = (0.3, 0.2, 0.5);
    let azs: Vec<f64> = std::env::var("PROBE_AZ").map(|v| vec![v.parse().unwrap()]).unwrap_or(vec![0.0, 1.0, FRAC_PI_2, 3.0]);
    for az in azs {
        let lp = RawLoop::new([(
            Point2::new(cx + r * az.cos(), -cz + r * az.sin()),
            Segment::Arc(Arc2 { centre: Point2::new(cx, -cz), radius: r, sweep: TAU }),
        )]);
        let profile = Profile::new(SketchPlane::<f64>::xy(), vec![lp]).validate(tol()).unwrap();
        let body = extrude(&profile, Extrusion::Distance { depth: 6.0, side: ExtrudeSide::Along }, tol()).unwrap().body;
        let place = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), -FRAC_PI_2)
            * Affine3::translation(Vec3::new(0.0, 0.0, -3.0));
        let tube = fin("tube", topo::transform_rigid(&body, &geom_core::test_support::rigid(&place, tol())).unwrap());
        let vab = chord_volume((cx, cz), r, big);
        let vt = PI * r * r * 6.0;
        let vb = 4.0 / 3.0 * PI * big.powi(3);
        for (op, res, want) in crate::common::differential::every_op_both_orders(&tube, &ball, (vt, vb, vab), tol()) {
            let err = res.as_ref().err().map(|e| format!("{e:?}"));
            if let Ok(b) = &res && let Some(bb) = b.body() {
                let far = fin("far", sweep::test_support::brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol()));
                eprintln!("OPERAND {op}: {:?}", topo::union(&bb.body, &far, tol()).err());
            }
            let j = crate::common::differential::judged(res, Some(want), tol()).line;
            eprintln!("PROBE az={az} {op}: {j} err={err:?}");
        }
    }
}

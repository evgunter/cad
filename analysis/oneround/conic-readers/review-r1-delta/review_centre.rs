//! Review probe (PR 7510 delta, reviewer r1): the centre-held arc's
//! farthest bound against sampled points, for main and the head to be
//! compared (`Arc::far`'s bulge factor is the one centre-held formula
//! the fix pass moved). Uses only doors both trees have.
#![allow(clippy::unwrap_used, clippy::print_stdout)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::{Point3, Vec3};
use test_utils::fuzz::Rng;

use super::{ParamWindow, Site, point, unit};
use crate::{Curve3, Ellipse, Frame, Line};

#[test]
fn review_centre_held_far_holds_sampled_points() {
    let mut rng = Rng::from_seed(0x5eed_c0de_0011);
    let ratios = [1e-3, 0.1, 0.5, 0.8, 1.0, 1.25, 10.0, 1e3];
    let lens = [1e-9, 3e-8, 1e-7, 1e-6, 1e-4, 1e-2, 0.3, 1.0, FRAC_PI_2, 2.0, PI, 4.0, TAU, 7.0];
    let (mut worst_far, mut worst_near, mut worst_ball) = (0.0_f64, 0.0_f64, 0.0_f64);
    let (mut at_far, mut at_near) = (String::new(), String::new());
    let mut sum_far = 0.0_f64;
    for i in 0..8000 {
        let a = rng.log_uniform(1e-2, 3e7);
        let circle = rng.below(3) == 0;
        let b = if circle { a } else { a * ratios[rng.below(ratios.len())] };
        let reach = [0.0, 1.0, 50.0, 1e3][rng.below(4)];
        let c = point(&mut rng, reach);
        let (n, u) = (Vec3::new(1.0, 2.0, 2.0) / 3.0, Vec3::new(2.0, 1.0, -2.0) / 3.0);
        let (n, u) = if rng.below(2) == 0 { (n, u) } else { (Vec3::new(0.0, 0.0, 1.0), Vec3::new(0.6, 0.8, 0.0)) };
        let carrier = if circle {
            Curve3::circle(c, n, a, u)
        } else {
            Curve3::Ellipse(Ellipse::new(Frame::new(c, n, u), a, b))
        };
        let len = lens[rng.below(lens.len())];
        let lo = rng.range(-7.0, 7.0);
        let (t0, t1) = if rng.below(5) == 0 { (lo + len, lo) } else { (lo, lo + len) };
        let cover = carrier.cover_over(ParamWindow::new(t0, t1)).unwrap();
        let span = (t1 - t0).clamp(-TAU, TAU);
        let pts: Vec<Point3<f64>> = (0..=400).map(|k| carrier.eval(t0 + span * f64::from(k) / 400.0)).collect();
        let scale = (c - Point3::origin()).norm() + a.max(b);
        let sites = [
            Site::Point(pts[200] + unit(&mut rng) * rng.log_uniform(1e-9, 1.0) * a.max(b)),
            Site::Point(pts[0]),
            Site::Point(c),
            Site::Point(point(&mut rng, 3.0 * scale)),
            Site::Line(Line { origin: point(&mut rng, scale), dir: unit(&mut rng) }),
            Site::Line(Line { origin: pts[400], dir: unit(&mut rng) }),
        ];
        let (mid, r) = cover.ball();
        for s in &sites {
            let (near, far) = cover.distance_range(s);
            sum_far += far / scale;
            for &p in &pts {
                let d = match s {
                    Site::Point(q) => (p - *q).norm(),
                    Site::Line(l) => l.distance(p),
                };
                if (d - far) / scale > worst_far {
                    worst_far = (d - far) / scale;
                    at_far = format!("#{i} a {a:e} b {b:e} window [{t0}, {t1}] miss {:e}", d - far);
                }
                if (near - d) / scale > worst_near {
                    worst_near = (near - d) / scale;
                    at_near = format!("#{i} a {a:e} b {b:e} window [{t0}, {t1}] miss {:e}", near - d);
                }
            }
        }
        for &p in &pts {
            worst_ball = worst_ball.max(((p - mid).norm() - r) / scale);
        }
    }
    println!("CENTRE far: worst miss / scale {worst_far:e} at {at_far}");
    println!("CENTRE near: worst miss / scale {worst_near:e} at {at_near}");
    println!("CENTRE ball: worst miss / scale {worst_ball:e}");
    println!("CENTRE sum of far / scale over all readings {sum_far:.17e}");
}

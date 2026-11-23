//! Review probe (PR 7510 delta, reviewer r1), H1: the arc-held extent
//! at `Interval` along the stored normal and the directions beside it,
//! and along the directions that put the anchor at a crest or a trough:
//! is it an enclosure, and how wide.
#![allow(clippy::unwrap_used, clippy::print_stdout)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom_core::{Bounds, Point3, Real, Vec3};
use test_utils::fuzz::Rng;

use super::{ipt, iv, ivec, point, unit};
use crate::curves::boxes::conic_arc_extent_along;
use crate::{Conic, Curve3, Ellipse};

#[test]
fn review_h1_interval_extent_is_an_enclosure_of_the_arcs_size() {
    let mut rng = Rng::from_seed(0x5eed_c0de_0021);
    let ratios = [1e-3, 0.1, 0.8, 1.0, 1.25, 10.0, 1e3];
    let lens = [1e-7, 1e-3, 0.3, 1.0, FRAC_PI_2, PI, 4.0, TAU, 9.0];
    let seams = [None, Some(0.0), Some(FRAC_PI_2), Some(PI), Some(-FRAC_PI_2), Some(2.3)];
    // Per family: worst width over the conic's scale, worst miss over it.
    let names = ["normal", "minus normal", "3.7 x normal", "normal + 1e-17", "normal + 1e-15", "normal + 1e-12", "normal + 1e-8", "anchor at crest", "anchor at trough", "k near 0", "u_ref", "v_ref", "random"];
    let mut width = vec![0.0_f64; names.len()];
    let mut miss = vec![0.0_f64; names.len()];
    let mut at = vec![String::new(); names.len()];
    let mut unbounded = 0;
    for i in 0..4000 {
        let a = rng.log_uniform(1e-2, 3e7);
        let b = a * ratios[rng.below(ratios.len())];
        let reach = [0.0, 1.0, 50.0, 1e3][rng.below(4)];
        let anchor = point(&mut rng, reach);
        let (axis, toward) = (unit(&mut rng), unit(&mut rng));
        if axis.cross(toward).norm() < 0.3 {
            continue;
        }
        let seam = seams[rng.below(seams.len())];
        let rev = rng.below(4) == 0;
        let e = Ellipse::anchored(anchor, axis, toward, a, b);
        let e = seam.map_or(e, |t| e.seamed_near(t));
        let e = if rev { e.reversed() } else { e };
        let ie = Ellipse::anchored(ipt(anchor), ivec(axis), ivec(toward), iv(a), iv(b));
        let ie = seam.map_or(ie, |t| ie.seamed_near(iv(t)));
        let ie = if rev { ie.reversed() } else { ie };
        let len = lens[rng.below(lens.len())];
        let lo = if rng.below(2) == 0 { -0.5 * len } else { rng.range(-7.0, 7.0) };
        let (t0, t1) = if rng.below(5) == 0 { (lo + len, lo) } else { (lo, lo + len) };
        let n = e.axis();
        let (u, v) = (e.u_ref(), e.v_ref());
        // The stored point's own direction in the harmonic: the anchor is
        // the point the ellipse stores; its parameter is where the point is.
        let stored = e.point_at(0.0) - Conic::center(&e);
        let radial = u * (stored.dot(u) / (a_of(&e).0 * a_of(&e).0)) + v * (stored.dot(v) / (a_of(&e).1 * a_of(&e).1));
        let radial = radial.normalize();
        let tangent = n.cross(radial).normalize();
        let w = unit(&mut rng);
        let dirs = [
            n,
            -n,
            n * 3.7,
            n + u * 1e-17,
            n + v * 1e-15,
            n + u * 1e-12,
            n + v * 1e-8,
            radial,
            -radial,
            tangent,
            u,
            v,
            w,
        ];
        let o = point(&mut rng, reach);
        let span = (t1 - t0).clamp(-TAU, TAU);
        let pts: Vec<Point3<f64>> = (0..=300).map(|k| e.point_at(t0 + span * f64::from(k) / 300.0)).collect();
        let scale = (anchor - Point3::origin()).norm() + (o - Point3::origin()).norm() + a.max(b);
        let skew = u.dot(n).abs() + v.dot(n).abs();
        for (j, d) in dirs.into_iter().enumerate() {
            let (elo, ehi) = conic_arc_extent_along(&Curve3::Ellipse(ie), iv(t0), iv(t1), ipt(o), ivec(d)).unwrap();
            if !(elo.is_bounded() && ehi.is_bounded()) {
                unbounded += 1;
                continue;
            }
            let wd = (Bounds::hi(elo) - Bounds::lo(elo)).max(Bounds::hi(ehi) - Bounds::lo(ehi)) / (scale * d.norm());
            let (mut smin, mut smax) = (f64::INFINITY, f64::NEG_INFINITY);
            for &p in &pts {
                let h = (p - o).dot(d);
                smin = smin.min(h);
                smax = smax.max(h);
            }
            let m = (Bounds::lo(elo) - smin).max(smax - Bounds::hi(ehi)) / (scale * d.norm()) - 2.0 * skew;
            if wd > width[j] {
                width[j] = wd;
                at[j] = format!("#{i} a {a:e} b {b:e} seam {seam:?} rev {rev} window [{t0}, {t1}]");
            }
            miss[j] = miss[j].max(m);
        }
    }
    for (j, name) in names.iter().enumerate() {
        println!("H1 {name}: worst Interval width / scale {:e}, worst miss / scale {:e}   ({})", width[j], miss[j], at[j]);
    }
    println!("H1 unbounded: {unbounded}");
}

fn a_of(e: &Ellipse<f64>) -> (f64, f64) {
    e.semi_axes()
}

//! Review-2 probe (scratch, not for merge): the real topo face reach of
//! random cylinder patches against a dense sample of the patch.
#![allow(clippy::all, clippy::pedantic, missing_docs)]

use crate::test_support_fixtures::{CylFrame, CylKey, cyl_wall_sheet_keyed};
use geom_core::Tol;
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

#[test]
#[ignore = "review probe"]
fn r2_real_patch_reach_against_a_dense_sample() {
    let mut g = fuzz::pinned("r2_patch", 11);
    let unit = |g: &mut fuzz::Rng| loop {
        let v = Vec3::new(g.range(-1., 1.), g.range(-1., 1.), g.range(-1., 1.));
        if (0.2..1.0).contains(&v.norm()) {
            return v.normalize();
        }
    };
    let log = |g: &mut fuzz::Rng, lo: f64, hi: f64| (lo.ln() + (hi / lo).ln() * g.unit()).exp();
    let n: usize = std::env::var("R2_N").map(|s| s.parse().unwrap()).unwrap_or(1000);
    let (mut short, mut long, mut built, mut worst_short, mut ratio_sum) = (0, 0, 0, 0.0_f64, 0.0);
    let mut worst_ratio = 0.0_f64;
    for i in 0..n {
        let r = [1e-3_f64, 1.0, 1e3][g.below(3)];
        let axis = unit(&mut g);
        let seam = { let v = unit(&mut g); (v - axis * v.dot(axis)).normalize() };
        let origin = Point3::origin() + unit(&mut g) * 1e3;
        let frame = CylFrame { origin, axis, radius: r, u_ref: seam };
        let u0 = g.range(-3.0, 3.0);
        let du = log(&mut g, (1e4 * 1e-9 / r).max(1e-6), 6.0);
        let v0 = g.range(-1.0, 1.0) * r;
        let h = log(&mut g, (1e4 * 1e-9 / r).max(1e-6), 10.0) * r;
        let mut body = crate::Body::<f64>::new();
        let built_ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            cyl_wall_sheet_keyed(&mut body, frame, CylKey::Bare, None, (u0, u0 + du), (v0, v0 + h), Tol::witness())
        }));
        let Ok((face, _)) = built_ok else { continue };
        built += 1;
        let corners: Vec<_> = body.vertex_points().map(|(_, p)| p).collect();
        let ats = [
            corners[g.below(corners.len())],
            frame.at::<f64>(u0 + du * g.unit(), v0 + h * g.unit()),
            origin + axis * (v0 + h * g.unit()),
            origin + unit(&mut g) * r * log(&mut g, 1e-3, 1e3),
            frame.at::<f64>(u0 + du * 0.5 + core::f64::consts::PI, v0 + h * 0.5),
        ];
        for at in ats {
            let head = crate::splitting::rules::face_reach_from(&body, face, at).unwrap();
            let round = crate::splitting::rules::face_reach_round_from(&body, face, at).unwrap();
            // Truth: dense grid over the patch, with golden refinement along each rim.
            let d = |u: f64, v: f64| (frame.at::<f64>(u, v) - at).norm();
            let mut truth = 0.0_f64;
            let (nu, nv) = (300, 40);
            for a in 0..=nu {
                for b in 0..=nv {
                    let (u, v) = (u0 + du * a as f64 / nu as f64, v0 + h * b as f64 / nv as f64);
                    truth = truth.max(d(u, v));
                }
            }
            for v in [v0, v0 + h] {
                let (mut lo, mut hi) = (u0, u0 + du);
                // refine around the sampled maximum on this rim
                let best = (0..=4000).map(|k| u0 + du * k as f64 / 4000.0).fold((u0, 0.0_f64), |(bu, bd), u| if d(u, v) > bd { (u, d(u, v)) } else { (bu, bd) });
                lo = (best.0 - du / 4000.0).max(lo); hi = (best.0 + du / 4000.0).min(hi);
                for _ in 0..100 {
                    let (m1, m2) = (lo + (hi - lo) * 0.381_966, lo + (hi - lo) * 0.618_034);
                    if d(m1, v) < d(m2, v) { lo = m1 } else { hi = m2 }
                }
                truth = truth.max(best.1).max(d(0.5 * (lo + hi), v));
            }
            let tol = 8.0 * f64::EPSILON * ((at - Point3::origin()).norm() + 1e3 + r);
            if head < truth - tol {
                short += 1;
                worst_short = worst_short.max((truth - head) / tol * 8.0);
                println!("SHORT {i} r={r:e} du={du:e} h={h:e} head={head:e} truth={truth:e} by {:e}", truth - head);
            }
            if head > round {
                long += 1;
                println!("LONG {i} head={head:e} round={round:e}");
            }
            ratio_sum += head / truth;
            worst_ratio = worst_ratio.max(head / truth);
        }
    }
    println!("built {built}/{n}; reads {}; short {short} (worst {worst_short:.2} ulps); longer than round {long}; head/truth mean {:.6} max {:.6}", built * 5, ratio_sum / (built * 5) as f64, worst_ratio);
}

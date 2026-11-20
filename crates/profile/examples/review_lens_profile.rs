//! Reviewer probe (PR 7146): a two-vertex profile (an arc of radius `r`
//! and its chord), its width swept across the band; prints each verdict.
#![allow(clippy::unwrap_used, clippy::print_stdout)]
use geom_core::{Point2, Tol};
use profile::{Profile, SketchPlane, test_support::bulge_loop};

fn main() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let mut s: u64 = 0x1234_5678_9abc_def1;
    let mut rnd = || {
        s ^= s << 13; s ^= s >> 7; s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    for r in [1.0e2, 1.0e3, 1.0e4, 1.0e5, 1.0e6] {
        for i in 0..200 {
            let w = 12.0 * eps * rnd();
            let theta = (12.0 * w / r).sqrt();
            let chord = 2.0 * r * (theta / 2.0).sin();
            let psi = 6.283_185_307_179_586 * rnd();
            let p0 = Point2::new(4.0 * rnd() - 2.0, 4.0 * rnd() - 2.0);
            let p1 = Point2::new(p0.x + chord * psi.cos(), p0.y + chord * psi.sin());
            let bulge = if i % 2 == 0 { (theta / 4.0).tan() } else { -(theta / 4.0).tan() };
            let lp = bulge_loop(vec![(p0, bulge), (p1, 0.0)]);
            let got = Profile::new(SketchPlane::xy(), vec![lp]).validate(tol);
            let v = match got {
                Ok(_) => "Ok".to_string(),
                Err(e) => format!("{e:?}").chars().take(200).collect(),
            };
            println!("PL eps={eps:e} r={r:e} i={i} theta={theta:e} p0=({:e},{:e}) p1=({:e},{:e}) b={bulge:e} -> {v}", p0.x, p0.y, p1.x, p1.y);
        }
    }
}

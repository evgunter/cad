//! Reviewer r1 merge-read probe: how far the derived centre of an
//! arc-held ellipse moves across an f64 quarter and half turn, on frames
//! in general position; and the near-anchor resolution after a quarter turn.
use geom::{Conic, Ellipse};
use geom_core::{Point3, Vec3};

#[test]
#[ignore]
fn the_derived_centre_across_a_quarter_turn() {
    let u = geom_core::UNIT_ROUNDOFF;
    let mut rng = 0x2545_F491_4F6C_DD1D_u64;
    let mut next = || {
        rng ^= rng << 13;
        rng ^= rng >> 7;
        rng ^= rng << 17;
        (rng >> 11) as f64 / (1u64 << 53) as f64 - 0.5
    };
    let fixed = (Vec3::new(2.0, 2.0, 1.0), Vec3::new(1.0, -2.0, 2.0));
    for a in [1e4, 1e5, 1e6] {
        let mut worst_q = 0.0_f64;
        let mut worst_h = 0.0_f64;
        let mut all: Vec<(f64, f64)> = Vec::new();
        let mut cases = vec![fixed];
        for _ in 0..40 {
            cases.push((Vec3::new(next(), next(), next()), Vec3::new(next(), next(), next())));
        }
        for (k, (n, toward)) in cases.into_iter().enumerate() {
            let e = Ellipse::anchored(Point3::new(3.5, -1.0, 2.0), n, toward, a, 0.37 * a);
            let c0 = e.center();
            let q = (e.seamed_near(1.4).center() - c0).norm() / (u * a);
            let h = (e.seamed_near(3.0).center() - c0).norm() / (u * a);
            if k == 0 {
                println!("a {a:e} the rows' fixture: quarter {q:.2} u·a, half {h:.2} u·a");
            }
            let sq = n.normalize().cross(toward.normalize()).norm();
            if k > 0 { all.push((q, sq)); }
            worst_q = worst_q.max(q);
            worst_h = worst_h.max(h);
        }
        all.sort_by(|x, y| x.0.total_cmp(&y.0));
        let qs: Vec<String> = all.iter().map(|(q, s)| format!("{q:.1}@{s:.2}")).collect();
        println!("a {a:e} sorted quarter u·a @ |n̂×t̂|: {}", qs.join(" "));
        println!("a {a:e} 40 general frames: worst quarter {worst_q:.2} u·a, worst half {worst_h:.2} u·a");
    }
}

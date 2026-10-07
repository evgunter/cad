//! PR 4249 second review's probe (review branch only): four pairs at
//! one vertex. The notch's corner against four cube corners touching
//! only at `v` (three tilted round the grid direction, one along it),
//! read by review r2's `r2_notch_vs`: `outcome`, the vertices at `v`,
//! `cones_at`, faces through two vertices, the mesh and the point
//! finding. Printed, for a diff between two trees.

use super::*;

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn rf_four_pairs_at_one_vertex() {
    let tilts = [-0.36, -0.2, -0.5];
    let mut kept = 0;
    for i in 0..12 {
        for j in 0..7 {
            let n = unit(direction(i, j));
            let [p, q, _] = frame(n, 0.0);
            for (t, &tilt) in tilts.iter().enumerate() {
                for k in 0..4 {
                    let spin = f64::from(k) * 0.7 + 0.1;
                    let mut fs: Vec<_> = (0..3)
                        .map(|c| {
                            let a = std::f64::consts::TAU * f64::from(c) / 3.0 + 0.3 * f64::from(k);
                            let d = [0, 1, 2].map(|x| a.cos() * p[x] + a.sin() * q[x] + tilt * n[x]);
                            r2_diag_frame(d, spin + 1.3 * f64::from(c))
                        })
                        .collect();
                    fs.push(r2_diag_frame(n, spin + 0.5));
                    let tag = format!("four i={i} j={j} t={t} k={k}");
                    if (0..4).any(|x| (x + 1..4).any(|y| r2_cones_overlap(fs[x], fs[y]))) {
                        println!("{tag}: SKIP overlap");
                        continue;
                    }
                    kept += 1;
                    for l in r2_notch_vs(&tag, &fs, SIDE) {
                        println!("{l}");
                    }
                }
            }
        }
    }
    println!("four: {kept} poses kept");
}

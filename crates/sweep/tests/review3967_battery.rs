//! Review probe for PR 3967: the reflex-corner booleans at near-flush
//! turns of the reviewer's choosing (not `rc_wide_battery`'s), one
//! `outcome` line per pose, for a merge-base / head diff.
//! `R3967_SHARD="k/n"` runs one shard of the (profile, turn) pairs.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;

use crate::common::differential::{REFLEX_OPS, REFLEX_PROFILES, outcome, reflex_pose, reflex_run};

#[test]
#[ignore = "review differential; run with --ignored --nocapture"]
fn review3967_near_flush_battery() {
    let tol = Tol::witness();
    let shard = std::env::var("R3967_SHARD").unwrap_or_else(|_| "0/1".into());
    let w: Vec<usize> = shard.split('/').map(|x| x.parse().unwrap()).collect();
    let (k, n) = (w[0], w[1]);
    let shears = [-0.6, -0.25, -0.05, 0.0, 0.05, 0.25, 0.6];
    let rots = [
        1e-4, -3e-4, 1e-3, -2e-3, 6e-3, 0.02, 45.0005, 44.998, 90.001, -0.0005, 135.003,
    ];
    let mut idx = 0;
    for (name, _) in REFLEX_PROFILES {
        for rot in rots {
            idx += 1;
            if idx % n != k {
                continue;
            }
            for &sx in &shears {
                for &sy in &shears {
                    if sx == 0.0 && sy == 0.0 {
                        continue;
                    }
                    let p = reflex_pose(name, rot, sx, sy, tol);
                    for (op, want) in REFLEX_OPS.iter().zip(p.want) {
                        println!(
                            "R3967 {name} rot={rot} sx={sx} sy={sy} {op} => {}",
                            outcome(reflex_run(&p, op, tol), want, tol)
                        );
                    }
                }
            }
        }
    }
}

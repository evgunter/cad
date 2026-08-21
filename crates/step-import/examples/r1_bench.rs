//! R1 latency probe: min-of-N import wall time per corpus file.
//! Usage: `cargo run --release --example r1_bench -- <path> [N]`.
//!
//! Authored on the R1 review branch (`m7/tier-review-probes`) and
//! ADOPTED at the M7-7 fix pass: it is how the gate's before/after
//! delta is measured against a real merge-base build rather than by
//! subtracting a re-run from a total.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Instant;

use step_import::{ImportOptions, import_step};
use geom_core::Tol;

fn main() {
    let tol = Tol::witness();
    let mut args = std::env::args().skip(1);
    let path = args.next().expect("usage: r1_bench <path> [N]");
    let n: u32 = args.next().map_or(25, |s| s.parse().unwrap());
    let text = std::fs::read_to_string(&path).unwrap();
    let options = ImportOptions::default();
    // warmup
    for _ in 0..3 {
        let _ = import_step(&text, &options, tol);
    }
    let mut best = f64::INFINITY;
    for _ in 0..n {
        let t = Instant::now();
        let r = import_step(&text, &options, tol);
        let dt = t.elapsed().as_secs_f64() * 1e3;
        std::hint::black_box(&r);
        best = best.min(dt);
    }
    println!("{path}: min-of-{n} {best:.3} ms");
}

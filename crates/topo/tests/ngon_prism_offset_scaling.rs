//! **How one simultaneous offset's cost grows with its own scope.**
//! Every face of one n-gon prism is offset outward by
//! [`topo::offset_planes_together`], and the call is timed per `n`; the
//! result is checked closed. The door's cost is superlinear in the
//! faces it moves: its writes drive its walks and tier 1 is derived
//! once at its end, but `WedgeReach::reach` folds a cap's whole
//! n-point boundary for each rim edge, O(n²) over the call
//! (`work/offset/wedge-reach-folds-a-faces-whole-boundary-per-edge`).
//! The prism leaves nothing to join, so the per-kill sweep of
//! `work/offset/an-offset-doors-trailing-join-sweeps-tier-1-per-kill`
//! does not show here.
//!
//! Evidence, not a gate (timings on a shared box are shape only):
//! `cargo nextest run -p topo --release --test all ngon_prism_offset_scaling
//! --run-ignored only --no-capture`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::time::Instant;

use geom_core::{Band, Tol};
use topo::test_support::prism;
use topo::{ChartMove, offset_planes_together};

#[test]
#[ignore = "timing evidence; run with --run-ignored only --no-capture"]
fn ngon_prism_offset_scaling() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    for n in [64_usize, 256, 1024] {
        let profile: Vec<(f64, f64)> = (0..n)
            .map(|k| {
                let t = std::f64::consts::TAU * k as f64 / n as f64;
                (t.cos(), t.sin())
            })
            .collect();
        let built = prism::<f64>(&profile, 1.0, tol);
        let moves: Vec<ChartMove<f64>> = built
            .body
            .faces()
            .map(|(f, _)| ChartMove {
                faces: vec![f],
                distance: 0.01,
            })
            .collect();
        let mut best = f64::INFINITY;
        let mut out = built.body.clone();
        for _ in 0..3 {
            let mut body = built.body.clone();
            let started = Instant::now();
            offset_planes_together(&mut body, &moves, band, tol).unwrap();
            best = best.min(started.elapsed().as_secs_f64());
            out = body;
        }
        println!(
            "n = {n:>4}: {} faces offset together, best of 3 {:.2} ms",
            moves.len(),
            best * 1e3
        );
        assert_eq!(
            topo::validate_closed(&out),
            Ok(()),
            "n = {n}: the offset prism is closed"
        );
    }
}

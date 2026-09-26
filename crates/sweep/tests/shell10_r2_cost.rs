//! **SHELL-10 review, R2 cost rows** — reproduces the PR's cost table
//! shape (release, 20 warm-up calls, three runs of 200, median ms per
//! call) on rows a reviewer can run at any SHA, base or head. Uses
//! nothing SHELL-10 added, so the same file compiles on both trees.
//! Ignored by default: run with `--release -- --ignored r2_cost`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::time::Instant;

use geom_core::{Tol, Vec3};
use sweep::test_support::block;

use crate::common::approx::band;
use crate::common::charts::{charts_of, moves_by};
use crate::shell8_common::{beside, cap, tol};
use crate::verbs_shell::{hollow_box, outer_and_void, vessel};

fn median_ms(label: &str, mut f: impl FnMut()) {
    for _ in 0..20 {
        f();
    }
    let mut runs: Vec<f64> = Vec::new();
    for _ in 0..3 {
        let started = Instant::now();
        for _ in 0..200 {
            f();
        }
        runs.push(started.elapsed().as_secs_f64() * 1e3 / 200.0);
    }
    runs.sort_by(f64::total_cmp);
    println!(
        "[r2-10 cost] {label}: median {:.4} ms/call, runs {runs:.4?}",
        runs[1]
    );
}

#[test]
#[ignore = "a timing row; run explicitly in release"]
fn r2_cost_rows() {
    let pair = beside(
        &block(2.0, 3.0, 4.0, Tol::witness()),
        &vessel(1.0, 2.0),
        10.0,
    );
    median_ms("shell_open, box beside vessel, sealed", || {
        topo::shell_open(&pair, 0.05, &[], tol()).unwrap();
    });
    let hollow = hollow_box();
    let (outer, _) = outer_and_void(&hollow);
    let lid = cap(&hollow, outer, Vec3::new(0.0, 0.0, 1.0), 4.0);
    median_ms("shell_open, hollow box, sealed", || {
        topo::shell_open(&hollow, 0.05, &[], tol()).unwrap();
    });
    median_ms("shell_open, hollow box, outer lid open", || {
        topo::shell_open(&hollow, 0.05, &lid, tol()).unwrap();
    });
    for n in [1usize, 2, 4, 8] {
        let mut body = vessel(1.0, 2.0);
        for i in 1..n {
            body = beside(&body, &vessel(1.0, 2.0), 10.0 * i as f64);
        }
        topo::mint_pcurves(&mut body, tol()).unwrap();
        let first = body.solids().next().unwrap().0;
        let moves = moves_by(charts_of(&body, first), -0.05);
        median_ms(&format!("offset_charts_together, 1 of {n} vessels"), || {
            let mut work = body.clone();
            topo::offset_charts_together(&mut work, &moves, band(), tol()).unwrap();
        });
    }
    for n in [1usize, 2, 4, 8] {
        let mut body = block(2.0, 3.0, 4.0, Tol::witness());
        for i in 1..n {
            body = beside(
                &body,
                &block(2.0, 3.0, 4.0, Tol::witness()),
                10.0 * i as f64,
            );
        }
        let first = body.solids().next().unwrap().0;
        let moves = moves_by(charts_of(&body, first), -0.05);
        median_ms(&format!("offset_planes_together, 1 of {n} boxes"), || {
            let mut work = body.clone();
            topo::offset_planes_together(&mut work, &moves, band(), tol()).unwrap();
        });
    }
}

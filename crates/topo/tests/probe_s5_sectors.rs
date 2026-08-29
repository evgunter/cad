//! S5 sector-predicate probe: the FULL recorded margin stream of a
//! boolean run and a plane-split run at the recording scalar, printed
//! in RECORDED ORDER, for merge-base-vs-tip byte-identity diffing.
//!
//! Why it exists: the vertex-neighborhood sector-shape rungs (metering
//! arm, wideness, subdivision direction) used to be written twice, once
//! per lane, and were merged into [`topo`]'s shared `sector_shape`
//! module (smell scan S5). #647 merged the bodies with the K predicate
//! names still a per-lane parameter; **#652 then pooled the names**,
//! six into three. Each of those steps has a load-bearing claim about
//! this stream — "same names, same margins" for the first, "same
//! margins, same order, three names where there were six" for the
//! second — and this is how they are reproduced rather than asserted:
//!
//! ```text
//! cargo test -p topo --features probe --test all -- --nocapture \
//!     probe_s5_sectors::sector_margin_stream | grep '^K '
//! ```
//!
//! on the merge base and on the tip, then diff. Order is part of the
//! claim — a reordering of decisions shows up even when the multiset is
//! preserved — so the rows are NOT sorted.
//!
//! **CI COMPILES AND EXECUTES THIS SUITE.** CI's `k-lint` job has
//! a step named *"compile and list every probe-gated test target"*
//! (`scripts/gates/probe-suite-census.sh` derives the crate set; the step
//! builds each one `--features probe --all-targets` and feeds the listing
//! back), so this file cannot rot into a build error unnoticed. That step
//! name is grepped for by the census gate, so this sentence cannot go
//! quietly false. The suite is ALSO rostered in that gate's `RUN_FLOOR`
//! and run under the DEFAULT selection by `scripts/k_probe_sweep.sh`,
//! whose tally is floored by `--check-executed`: the recorded stream and
//! the six per-lane coverage assertions below are gates, and a drift in
//! either reds the merge. By hand:
//! `cargo test -p topo --features probe --test all -- probe_s5_sectors::`.
//! `tests/probe_census.rs` is in the same position. The ε sweep over the
//! same telemetry is CI's `k-lint`, which runs
//! `scripts/k_probe_sweep.sh` at three ε.
//!
//! The fixtures are chosen to drive BOTH lanes: two boolean subtracts
//! at two scales, and three plane splits of the notched block whose
//! plane lands ON vertices (the splitting lane's sector walk only runs
//! for ON vertices). Since #652 both lanes emit the SAME three names,
//! so the predicate column can no longer tell you which lane a row came
//! from — and the per-lane coverage claim is NOT left to the fixtures
//! and the recorded order to carry by implication. The two lanes are
//! recorded into two sinks, drained in order and printed as one stream
//! (so the dump is byte-for-byte what it was when they shared a sink),
//! and each of the three rungs is asserted to have fired in EACH lane.
//! Delete the splitting fixtures and six assertions go red ON A HAND RUN,
//! exactly as they did when the six lane-prefixed names carried that job —
//! no merge runs them, per the disposition above.
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod common;

use common::{prism, prism_z};
use geom_core::Tol;
use geom_core::k_stats::{self, Probe};
use geom_core::{Point3, Real, Vec3};
use topo::{BooleanResult, SplitPlane, split, subtract};

/// The Fig. 14.2 analogue: flat notch floor and V-notch tips that the
/// split planes below land exactly on.
const NOTCHED: &[(f64, f64)] = &[
    (0.0, 0.0),
    (8.0, 0.0),
    (8.0, 2.0),
    (7.0, 1.0),
    (6.0, 1.0),
    (5.0, 2.0),
    (4.0, 1.0),
    (3.0, 2.0),
    (2.0, 1.0),
    (0.0, 1.0),
];

/// The three rungs this probe exists for — one name set, shared by
/// both lanes since #652.
const SECTOR_PREDICATES: [&str; 3] = ["sector_arm", "sector_reflex", "sector_straight"];

fn bx(s: f64, x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> topo::Body<Probe> {
    let f = |v: f64| v * s;
    prism_z::<Probe>(
        &[
            (f(x.0), f(y.0)),
            (f(x.1), f(y.0)),
            (f(x.1), f(y.1)),
            (f(x.0), f(y.1)),
        ],
        f(z.0),
        f(z.1),
    )
    .body
}

fn plane_y(c: f64) -> SplitPlane<Probe> {
    SplitPlane {
        origin: Point3::new(
            Probe::from_f64(0.0),
            Probe::from_f64(c),
            Probe::from_f64(0.0),
        ),
        normal: Vec3::new(
            Probe::from_f64(0.0),
            Probe::from_f64(1.0),
            Probe::from_f64(0.0),
        ),
    }
}

#[test]
fn sector_margin_stream() {
    // Recorded in TWO segments, boolean lane then splitting lane, so
    // that per-lane participation can be asserted and not merely
    // implied: since #652 the predicate column is lane-blind, and a
    // pooled `seen[i] > 0` over three names is satisfied by the boolean
    // fixtures alone. Draining and re-installing the sink between the
    // two groups changes nothing about the rows or their order — the
    // fixtures already ran in this order — so the printed stream is
    // byte-identical to the single-sink version.
    k_stats::start_recording();
    for scale in [1e-3, 1.0] {
        let a1 = bx(scale, (0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b1 = bx(scale, (1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
        match subtract(&a1, &b1, Tol::witness()).expect("corner subtract") {
            BooleanResult::Body(_) => {}
            other => panic!("corner: {other:?}"),
        }
        let a2 = bx(scale, (0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
        let b2 = bx(scale, (1.0, 2.0), (1.0, 2.0), (-1.0, 2.0));
        subtract(&a2, &b2, Tol::witness()).expect("pocket subtract");
    }
    let bool_samples = k_stats::take_samples();

    k_stats::start_recording();
    for c in [1.0, 1.5, 2.0] {
        let body = prism::<Probe>(NOTCHED, 3.0).body;
        // The result is not the point; the recorded decisions are. A
        // typed refusal is a legitimate outcome of a vertex-grazing
        // plane and its margins are recorded either way.
        let _ = split(&body, &plane_y(c), Tol::witness());
    }
    let split_samples = k_stats::take_samples();

    let mut seen = [[0_usize; 3]; 2];
    for (lane, samples) in [&bool_samples, &split_samples].into_iter().enumerate() {
        for s in samples {
            // Recorded ORDER is part of the claim: no sort.
            println!(
                "K {}|{:?}|{:?}|{:?}|{:?}",
                s.predicate, s.outcome, s.margin, s.band_zero, s.band_escalate
            );
            for (i, name) in SECTOR_PREDICATES.iter().enumerate() {
                if s.predicate == *name {
                    seen[lane][i] += 1;
                }
            }
        }
    }
    // Without these the dump could "pass" while exercising neither
    // lane's sector walk, and a diff of two empty streams proves
    // nothing. Per-lane, not pooled: the whole point of the fixture set
    // is that BOTH walks reach all three rungs, and after the name pool
    // nothing else in this file can tell if one of them stopped.
    for (lane, lane_name) in ["boolean", "splitting"].into_iter().enumerate() {
        for (i, name) in SECTOR_PREDICATES.iter().enumerate() {
            assert!(
                seen[lane][i] > 0,
                "the {lane_name} fixtures recorded no `{name}` samples, so this \
                 probe is not covering the rung it exists for in that lane \
                 (recorded: boolean {}, splitting {})",
                bool_samples.len(),
                split_samples.len()
            );
        }
    }
}

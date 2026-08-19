//! S5 sector-predicate probe: the FULL recorded margin stream of a
//! boolean run and a plane-split run at the recording scalar, printed
//! in RECORDED ORDER, for merge-base-vs-tip byte-identity diffing.
//!
//! Why it exists: the vertex-neighborhood sector-shape rungs (metering
//! arm, wideness, subdivision direction) used to be written twice, once
//! per lane, and were merged into [`topo`]'s shared `sector_shape`
//! module with the K predicate names as a parameter (smell scan S5).
//! "Same names, same margins" is the load-bearing claim of that merge,
//! and this is how it is reproduced rather than asserted:
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
//! **NOT run by CI, and not a gate.** `#![cfg(feature = "probe")]`
//! means the default rows do not even type-check this file, and nothing
//! in `.github/workflows/` runs `cargo test -p topo --features probe`
//! (the K sweep runs `-p editor-core --features probe`). So this is a
//! reproducible HAND-RUN artifact: it can bit-rot green, and a claim
//! that leans on it must say so. `tests/probe_census.rs` and
//! `tests/probe_f34_review.rs` are in the same position — a class, not
//! this suite's peculiarity. The standing gate over the same telemetry
//! is CI's `k-lint`, which runs `scripts/k_probe_sweep.sh` at three ε.
//!
//! The fixtures are chosen to drive BOTH lanes: two boolean subtracts at
//! two scales (the `bool_sector_*` rungs) and three plane splits of the
//! notched block whose plane lands ON vertices (the `split_sector_*`
//! rungs, which only run for ON vertices).
#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod common;

use common::{prism, prism_z};
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

/// The six rungs this probe exists for.
const SECTOR_PREDICATES: [&str; 6] = [
    "bool_sector_arm",
    "bool_sector_reflex",
    "bool_sector_straight",
    "split_sector_arm",
    "split_sector_reflex",
    "split_sector_straight",
];

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
    k_stats::start_recording();
    for scale in [1e-3, 1.0] {
        let a1 = bx(scale, (0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b1 = bx(scale, (1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
        match subtract(&a1, &b1).expect("corner subtract") {
            BooleanResult::Body(_) => {}
            other => panic!("corner: {other:?}"),
        }
        let a2 = bx(scale, (0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
        let b2 = bx(scale, (1.0, 2.0), (1.0, 2.0), (-1.0, 2.0));
        subtract(&a2, &b2).expect("pocket subtract");
    }
    for c in [1.0, 1.5, 2.0] {
        let body = prism::<Probe>(NOTCHED, 3.0).body;
        // The result is not the point; the recorded decisions are. A
        // typed refusal is a legitimate outcome of a vertex-grazing
        // plane and its margins are recorded either way.
        let _ = split(&body, &plane_y(c));
    }
    let samples = k_stats::take_samples();
    let mut seen = [0_usize; 6];
    for s in &samples {
        // Recorded ORDER is part of the claim: no sort.
        println!(
            "K {}|{:?}|{:?}|{:?}|{:?}",
            s.predicate, s.outcome, s.margin, s.band_zero, s.band_escalate
        );
        for (i, name) in SECTOR_PREDICATES.iter().enumerate() {
            if s.predicate == *name {
                seen[i] += 1;
            }
        }
    }
    // Without this the dump could "pass" while exercising neither
    // lane's sector walk, and a diff of two empty streams proves
    // nothing.
    for (i, name) in SECTOR_PREDICATES.iter().enumerate() {
        assert!(
            seen[i] > 0,
            "the fixtures recorded no `{name}` samples, so this probe is not \
             covering the rung it exists for (recorded {} samples total)",
            samples.len()
        );
    }
}

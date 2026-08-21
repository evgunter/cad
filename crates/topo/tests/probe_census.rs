//! Review probe (review/f34): dump the FULL per-sample census of both
//! twin boolean configurations at both scales, for merge-base-vs-tip
//! byte-identity diffing (T3).
//!
//! **NO TEST IN THIS FILE IS EXECUTED BY CI.** The probe suites CI runs are
//! rostered in `scripts/gates/probe-suite-census.sh` (`RUN_FLOOR`) and run
//! by `scripts/k_probe_sweep.sh`; this one is on neither list, so nothing
//! here can go red on a merge and its assertions are evidence for a reader
//! rather than a gate. By hand:
//! `cargo test -p topo --features probe --test all -- probe_census::`.

#![cfg(feature = "probe")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod common;

use common::prism_z;
use geom_core::k_stats::{self, Probe};
use topo::{BooleanResult, subtract};

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

#[test]
fn dump_full_census() {
    for scale in [1e-3, 1.0] {
        k_stats::start_recording();
        let a1 = bx(scale, (0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b1 = bx(scale, (1.0, 3.0), (1.0, 3.0), (1.0, 3.0));
        let r = match subtract(&a1, &b1).expect("corner") {
            BooleanResult::Body(b) => b,
            other => panic!("corner: {other:?}"),
        };
        topo::validate_pseudomanifold(&r.body, &topo::ContactRecords::default()).expect("census");
        let a2 = bx(scale, (0.0, 4.0), (0.0, 4.0), (0.0, 1.0));
        let b2 = bx(scale, (1.0, 2.0), (1.0, 2.0), (-1.0, 2.0));
        subtract(&a2, &b2).expect("pocket");
        for s in k_stats::take_samples() {
            println!(
                "CEN {} {} {:?} {:?}",
                scale, s.predicate, s.outcome, s.margin
            );
        }
    }
}
